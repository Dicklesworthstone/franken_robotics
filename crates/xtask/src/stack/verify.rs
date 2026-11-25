//! `xtask check-stack`: three-way comparison of `franken-stack.lock`, the
//! newest receipt under `receipts/HEAD`, and the current repository + sibling
//! trees, plus `--write-receipt` mode (bead `N33a-05`, INV-SUITE-03,
//! INV-SUITE-04, GATE-WS-01 plant 7).
//!
//! Strictly read-only on sibling checkouts: never opens any sibling path for
//! writing and uses only read-only `git --no-optional-locks` queries. Writes
//! only inside this repository under `receipts/` when `--write-receipt` is
//! explicitly passed. Determinism class D0, zero `unsafe`, prints nothing
//! (rule L-7).

use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Component, Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

use frob_core::json::{Limits, admit};
use frob_core::registry::toml::{self, Value as TomlValue};

use crate::cli::{CheckContext, CheckError, CheckOutcome};
use crate::crate_map::Finding;
use crate::report::FactValue;
use crate::stack::guard;
use crate::stack::lifts;
use crate::stack::lock::{self, GateRef, GitRev, StackLock};
use crate::stack::receipt::{
    self, DriftClass, DriftReason, OwnLockEntry, ReceiptError, SiblingEntry, StackReceipt,
    WORKSPACES,
};

/// Schema identifier for the `check-stack` facts block (`N33a-05`).
pub const CHECK_STACK_FACTS_SCHEMA: &str = "frob.check-stack/1";

/// Rule identifiers owned exclusively by `check-stack` (every identifier begins with `stack.`).
pub const RULE_LOCK_SCHEMA: &str = "stack.lock-schema";
pub const RULE_LIFT: &str = "stack.lift";
pub const RULE_RECEIPT_MISSING: &str = "stack.receipt-missing";
pub const RULE_RECEIPT_HASH: &str = "stack.receipt-hash";
pub const RULE_TOOLCHAIN: &str = "stack.toolchain";
pub const RULE_TOOLCHAIN_FILE: &str = "stack.toolchain-file";
pub const RULE_PIN: &str = "stack.pin";
pub const RULE_SIBLING: &str = "stack.sibling";
pub const RULE_OWN_LOCK: &str = "stack.own-lock";
pub const RULE_LOCKFILE: &str = "stack.lockfile";
pub const RULE_PATCH: &str = "stack.patch";
pub const RULE_PROFILE: &str = "stack.profile";
pub const RULE_ENV: &str = "stack.env";

/// Complete list of finding rule identifiers owned exclusively by `check-stack`.
pub const OWNED_RULES: &[&str] = &[
    RULE_LOCK_SCHEMA,
    RULE_LIFT,
    RULE_RECEIPT_MISSING,
    RULE_RECEIPT_HASH,
    RULE_TOOLCHAIN,
    RULE_TOOLCHAIN_FILE,
    RULE_PIN,
    RULE_SIBLING,
    RULE_OWN_LOCK,
    RULE_LOCKFILE,
    RULE_PATCH,
    RULE_PROFILE,
    RULE_ENV,
];

/// Authoritative kind-to-rule mapping: every kind belongs to exactly one `stack.*` identifier.
pub const KIND_TO_RULE: &[(&str, &str)] = &[
    ("io-error", RULE_LOCK_SCHEMA),
    ("toml-syntax", RULE_LOCK_SCHEMA),
    ("unknown-field", RULE_LOCK_SCHEMA),
    ("missing-field", RULE_LOCK_SCHEMA),
    ("empty-field", RULE_LOCK_SCHEMA),
    ("invalid-field-type", RULE_LOCK_SCHEMA),
    ("empty-string-entry", RULE_LOCK_SCHEMA),
    ("invalid-row", RULE_LOCK_SCHEMA),
    ("missing-id", RULE_LOCK_SCHEMA),
    ("invalid-id", RULE_LOCK_SCHEMA),
    ("duplicate-id", RULE_LOCK_SCHEMA),
    ("duplicate-name", RULE_LOCK_SCHEMA),
    ("invalid-posture", RULE_LOCK_SCHEMA),
    ("missing-posture", RULE_LOCK_SCHEMA),
    ("empty-uses", RULE_LOCK_SCHEMA),
    ("invalid-use-entry", RULE_LOCK_SCHEMA),
    ("invalid-tier", RULE_LOCK_SCHEMA),
    ("invalid-milestone", RULE_LOCK_SCHEMA),
    ("conflicting-firewall-milestone", RULE_LOCK_SCHEMA),
    ("invalid-admission-gate", RULE_LOCK_SCHEMA),
    ("invalid-git-rev", RULE_LOCK_SCHEMA),
    ("invalid-path", RULE_LOCK_SCHEMA),
    ("missing-path", RULE_LOCK_SCHEMA),
    ("unadmitted-rev-after-admission", RULE_LOCK_SCHEMA),
    ("missing-git-rev", RULE_LOCK_SCHEMA),
    ("invalid-version", RULE_LOCK_SCHEMA),
    ("missing-version", RULE_LOCK_SCHEMA),
    ("invalid-fsim-feature", RULE_LOCK_SCHEMA),
    ("invalid-build-env", RULE_LOCK_SCHEMA),
    ("invalid-toolchain", RULE_LOCK_SCHEMA),
    ("missing-toolchain", RULE_LOCK_SCHEMA),
    ("invalid-receipt", RULE_LOCK_SCHEMA),
    ("missing-receipt", RULE_LOCK_SCHEMA),
    ("missing-receipt-file", RULE_LOCK_SCHEMA),
    ("empty-fallback", RULE_LOCK_SCHEMA),
    ("lock-parse-error", RULE_LOCK_SCHEMA),
    ("lifts.parse", RULE_LIFT),
    ("lifts.record.missing-file", RULE_LIFT),
    ("lifts.record.header-mismatch", RULE_LIFT),
    ("lifts.record.missing-golden-test", RULE_LIFT),
    ("lifts.record.golden-test-missing-ref", RULE_LIFT),
    ("lifts.unregistered", RULE_LIFT),
    ("lifts.forbidden-edge", RULE_LIFT),
    ("no-receipt", RULE_RECEIPT_MISSING),
    ("receipt-tampered", RULE_RECEIPT_HASH),
    ("toolchain-changed", RULE_TOOLCHAIN),
    ("off-T", RULE_TOOLCHAIN),
    ("row-toolchain-stale", RULE_TOOLCHAIN),
    ("extra-toolchain-file", RULE_TOOLCHAIN_FILE),
    ("lock-edited", RULE_PIN),
    ("off-pin", RULE_PIN),
    ("sibling-moved", RULE_SIBLING),
    ("dirty-changed", RULE_SIBLING),
    ("dirty-unchanged", RULE_SIBLING),
    ("own-lock-disagrees", RULE_OWN_LOCK),
    ("lockfile-changed", RULE_LOCKFILE),
    ("lockfile-missing", RULE_LOCKFILE),
    ("patch-missing", RULE_PATCH),
    ("profile-override-missing", RULE_PROFILE),
    ("build-env-missing", RULE_ENV),
    ("foreign-build-class", RULE_ENV),
];

/// Reason accepted by `--write-receipt --reason <reason>`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WriteReason {
    Admission,
    PinMove,
    ToolchainTrain,
}

impl WriteReason {
    pub fn parse(raw: &str) -> Result<Self, String> {
        match raw {
            "admission" => Ok(Self::Admission),
            "pin-move" => Ok(Self::PinMove),
            "toolchain-train" => Ok(Self::ToolchainTrain),
            other => Err(format!(
                "invalid `--reason` value `{other}` (expected admission, pin-move, or toolchain-train)"
            )),
        }
    }
}

/// Parsed CLI options for `cargo xtask check-stack`.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CheckStackArgs {
    pub write_receipt: bool,
    pub reason: Option<WriteReason>,
    pub gate: Option<GateRef>,
}

/// Optional inputs supplied by callers/tests (e.g. admitted gates from `N33a-07`).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct VerifyOptions {
    pub admitted: BTreeSet<GateRef>,
    /// Optional override for duplicate package list in synthetic tests.
    pub duplicates_override: Option<Vec<String>>,
}

/// Kind-level detail row emitted inside `facts.kinds` in `frob.check-stack/1`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KindRecord {
    pub rule: &'static str,
    pub kind: String,
    pub fd: Option<String>,
    pub class: Option<String>,
    pub fails: bool,
}

impl KindRecord {
    fn to_fact(&self) -> FactValue {
        let mut map = BTreeMap::new();
        map.insert("rule".to_owned(), FactValue::str(self.rule));
        map.insert("kind".to_owned(), FactValue::str(&self.kind));
        map.insert(
            "fd".to_owned(),
            self.fd.as_deref().map_or(FactValue::Null, FactValue::str),
        );
        map.insert(
            "class".to_owned(),
            self.class.as_deref().map_or(FactValue::Null, FactValue::str),
        );
        map.insert("fails".to_owned(), FactValue::Bool(self.fails));
        FactValue::Object(map)
    }
}

/// CLI handler registered in `src/main.rs`'s static `COMMANDS` array.
pub fn run_command(ctx: &CheckContext) -> Result<CheckOutcome, CheckError> {
    let parsed = parse_check_stack_args(&ctx.extra_args).map_err(CheckError::Usage)?;
    if parsed.write_receipt {
        run_write_receipt(&ctx.repo_root, &VerifyOptions::default())
    } else {
        verify_repo(&ctx.repo_root, &VerifyOptions::default())
    }
}

/// Parse `check-stack`-specific flags (`--write-receipt`, `--reason`, `--gate`).
pub fn parse_check_stack_args(extra: &[String]) -> Result<CheckStackArgs, String> {
    let mut out = CheckStackArgs::default();
    let mut i = 0usize;
    while i < extra.len() {
        let arg = &extra[i];
        if arg == "--write-receipt" {
            out.write_receipt = true;
            i += 1;
        } else if arg == "--reason" {
            let Some(val) = extra.get(i + 1) else {
                return Err("`--reason` requires a value (admission, pin-move, or toolchain-train)".to_owned());
            };
            out.reason = Some(WriteReason::parse(val)?);
            i += 2;
        } else if let Some(val) = arg.strip_prefix("--reason=") {
            out.reason = Some(WriteReason::parse(val)?);
            i += 1;
        } else if arg == "--gate" {
            let Some(val) = extra.get(i + 1) else {
                return Err("`--gate` requires an admission gate identifier (e.g. INT-FSIM-01)".to_owned());
            };
            let gate = GateRef::parse(val, "--gate").map_err(|e| e.message)?;
            out.gate = Some(gate);
            i += 2;
        } else if let Some(val) = arg.strip_prefix("--gate=") {
            let gate = GateRef::parse(val, "--gate").map_err(|e| e.message)?;
            out.gate = Some(gate);
            i += 1;
        } else {
            return Err(format!("unknown check-stack flag `{arg}`"));
        }
    }
    if (out.reason.is_some() || out.gate.is_some()) && !out.write_receipt {
        return Err("`--reason` and `--gate` require `--write-receipt`".to_owned());
    }
    Ok(out)
}

/// Execute `check-stack --write-receipt`: compute the current receipt, write
/// `receipts/<64-hex>.json` atomically (leaving an intact existing receipt
/// untouched), and atomically update `receipts/HEAD` and `receipts/LOCK_DIGEST`.
pub fn run_write_receipt(
    repo_root: &Path,
    opts: &VerifyOptions,
) -> Result<CheckOutcome, CheckError> {
    let lock_path = repo_root.join("franken-stack.lock");
    let receipts_dir = repo_root.join("receipts");

    let stack_lock = match lock::load(&lock_path) {
        Ok(l) => l,
        Err(err) => {
            let finding = Finding {
                rule: RULE_LOCK_SCHEMA,
                subject: err.row.clone(),
                path: "franken-stack.lock".to_owned(),
                line: None,
                message: err.to_string(),
                repair: "fix franken-stack.lock schema or field validation error".to_owned(),
            };
            let kind = KindRecord {
                rule: RULE_LOCK_SCHEMA,
                kind: err.kind.to_owned(),
                fd: non_root_fd(&err.row),
                class: None,
                fails: true,
            };
            let facts = build_facts(
                None,
                None,
                &[],
                None,
                None,
                &[],
                &[],
                &[kind],
                false,
                None,
            );
            return Ok(CheckOutcome {
                findings: vec![finding],
                facts,
            });
        }
    };

    let lock_digest_hex = bytes_to_lower_hex(&stack_lock.digest());
    let (receipt, canon, hash) = receipt::build_receipt(repo_root, &lock_path)
        .map_err(|e| CheckError::Fault(format!("failed building receipt: {e}")))?;
    let (receipt_path, written_hash) = receipt::write_receipt(&receipts_dir, &canon)
        .map_err(|e| CheckError::Fault(format!("failed writing receipt: {e}")))?;

    atomic_write_line(&receipts_dir.join("HEAD"), &written_hash)
        .map_err(|e| CheckError::Fault(format!("failed updating receipts/HEAD: {e}")))?;
    atomic_write_line(&receipts_dir.join("LOCK_DIGEST"), &lock_digest_hex)
        .map_err(|e| CheckError::Fault(format!("failed updating receipts/LOCK_DIGEST: {e}")))?;

    let dup_list = opts
        .duplicates_override
        .as_deref()
        .unwrap_or(&receipt.duplicates);
    let may_pass = guard::may_record_pass(&receipt_path).is_ok()
        && !dup_list.iter().any(|d| guard::is_asupersync_duplicate(d));
    let sib_facts = build_sibling_facts(&stack_lock, &receipt.siblings);
    let facts = build_facts(
        Some(stack_lock.toolchain.as_str()),
        Some(&lock_digest_hex),
        &stack_lock.foundations,
        Some(&written_hash),
        Some(&hash),
        &sib_facts,
        dup_list,
        &[],
        may_pass,
        Some(&written_hash),
    );
    Ok(CheckOutcome {
        findings: Vec::new(),
        facts,
    })
}

/// Execute `check-stack` in read-only verify mode on `repo_root`.
///
/// Writes nothing anywhere inside or outside `repo_root`.
pub fn verify_repo(repo_root: &Path, opts: &VerifyOptions) -> Result<CheckOutcome, CheckError> {
    let lock_path = repo_root.join("franken-stack.lock");
    let receipts_dir = repo_root.join("receipts");

    let mut failing: Vec<(Finding, KindRecord)> = Vec::new();
    let mut informational: Vec<KindRecord> = Vec::new();

    // 1. Load stack lock + run state-dependent admission rules.
    let stack_lock = match lock::load(&lock_path) {
        Ok(l) => l,
        Err(err) => {
            failing.push((
                Finding {
                    rule: RULE_LOCK_SCHEMA,
                    subject: err.row.clone(),
                    path: "franken-stack.lock".to_owned(),
                    line: None,
                    message: err.to_string(),
                    repair: "fix franken-stack.lock to satisfy frob.stack-lock/1 rules 1-11"
                        .to_owned(),
                },
                KindRecord {
                    rule: RULE_LOCK_SCHEMA,
                    kind: err.kind.to_owned(),
                    fd: non_root_fd(&err.row),
                    class: None,
                    fails: true,
                },
            ));
            return Ok(finalize_outcome(
                failing,
                informational,
                None,
                None,
                &[],
                None,
                None,
                &[],
                &[],
                false,
                None,
            ));
        }
    };

    let lock_digest_hex = bytes_to_lower_hex(&stack_lock.digest());
    for err in stack_lock.check_admitted(&opts.admitted, &receipts_dir) {
        let (rule, kind_name) = if err.rule == 10 && err.kind == "missing-receipt-file" {
            (RULE_RECEIPT_MISSING, "no-receipt".to_owned())
        } else {
            (RULE_LOCK_SCHEMA, err.kind.to_owned())
        };
        failing.push((
            Finding {
                rule,
                subject: err.row.clone(),
                path: "franken-stack.lock".to_owned(),
                line: None,
                message: err.to_string(),
                repair: "record the admitted pin or missing receipt under receipts/".to_owned(),
            },
            KindRecord {
                rule,
                kind: kind_name,
                fd: non_root_fd(&err.row),
                class: None,
                fails: true,
            },
        ));
    }

    // 2. Provenance-lift validator (`N33a-01`).
    if repo_root.join("registries/lifts.toml").is_file() || repo_root.join("registries").is_dir() {
        match lifts::validate(repo_root) {
            Ok(lift_findings) => {
                for lf in lift_findings {
                    failing.push((
                        Finding {
                            rule: RULE_LIFT,
                            subject: lf.file.clone(),
                            path: lf.file.clone(),
                            line: None,
                            message: lf.message.clone(),
                            repair: "register the lifted file in registries/lifts.toml with matching header and golden_test".to_owned(),
                        },
                        KindRecord {
                            rule: RULE_LIFT,
                            kind: lf.rule,
                            fd: None,
                            class: None,
                            fails: true,
                        },
                    ));
                }
            }
            Err(err) => {
                failing.push((
                    Finding {
                        rule: RULE_LIFT,
                        subject: err.context.clone(),
                        path: "registries/lifts.toml".to_owned(),
                        line: None,
                        message: err.to_string(),
                        repair: "repair registries/lifts.toml so it conforms to frob.lifts/1"
                            .to_owned(),
                    },
                    KindRecord {
                        rule: RULE_LIFT,
                        kind: "lifts.parse".to_owned(),
                        fd: None,
                        class: None,
                        fails: true,
                    },
                ));
            }
        }
    }

    // 3. Read newest receipt named by `receipts/HEAD`.
    let (newest_hash, newest_receipt, newest_path) =
        load_newest_receipt(&receipts_dir, &mut failing);

    // 4. Inspect root files directly (toolchain, lockfiles, patches, profile_overrides, build_env).
    let cur_toolchain = read_root_toolchain_channel(repo_root);
    let tc_mismatch_lock = cur_toolchain
        .as_deref()
        .is_none_or(|ch| ch != stack_lock.toolchain.as_str());
    let tc_mismatch_receipt = newest_receipt
        .as_ref()
        .is_some_and(|nr| cur_toolchain.as_deref() != Some(nr.toolchain.as_str()));
    if tc_mismatch_lock || tc_mismatch_receipt {
        let obs = cur_toolchain.as_deref().unwrap_or("<missing>");
        failing.push((
            Finding {
                rule: RULE_TOOLCHAIN,
                subject: "rust-toolchain.toml".to_owned(),
                path: "rust-toolchain.toml".to_owned(),
                line: None,
                message: format!(
                    "toolchain-changed: rust-toolchain.toml is `{obs}`, lock has `{}`",
                    stack_lock.toolchain
                ),
                repair: "align rust-toolchain.toml with franken-stack.lock or run a toolchain train and `--write-receipt`".to_owned(),
            },
            KindRecord {
                rule: RULE_TOOLCHAIN,
                kind: "toolchain-changed".to_owned(),
                fd: None,
                class: None,
                fails: true,
            },
        ));
    }

    // Row off-T check (informational until an adopted toolchain train exists in receipts/train-log.ndjson).
    let train_adopted = has_adopted_toolchain_train(&receipts_dir);
    for row in &stack_lock.foundations {
        if let Some(row_tc) = &row.toolchain
            && row_tc != &stack_lock.toolchain
        {
            let kind = KindRecord {
                rule: RULE_TOOLCHAIN,
                kind: "off-T".to_owned(),
                fd: Some(row.id.as_str().to_owned()),
                class: None,
                fails: train_adopted,
            };
            if train_adopted {
                failing.push((
                    Finding {
                        rule: RULE_TOOLCHAIN,
                        subject: row.id.as_str().to_owned(),
                        path: "franken-stack.lock".to_owned(),
                        line: None,
                        message: format!(
                            "off-T: row `{}` toolchain `{row_tc}` differs from T `{}` after adopted toolchain train",
                            row.id, stack_lock.toolchain
                        ),
                        repair: "advance the sibling to T through a compatibility train".to_owned(),
                    },
                    kind,
                ));
            } else {
                informational.push(kind);
            }
        }
    }

    // Lockfile existence and hash comparison per workspace.
    let mut lockfiles_present = true;
    for &(ws_name, ws_rel) in WORKSPACES {
        let ws_dir = if ws_rel == "." {
            repo_root.to_path_buf()
        } else {
            repo_root.join(ws_rel)
        };
        let manifest_exists = ws_name == "core" || ws_dir.join("Cargo.toml").is_file();
        if !manifest_exists {
            continue;
        }
        let ws_lock = ws_dir.join("Cargo.lock");
        let lock_rel = if ws_rel == "." {
            "Cargo.lock".to_owned()
        } else {
            format!("{ws_rel}/Cargo.lock")
        };
        if !ws_lock.is_file() {
            lockfiles_present = false;
            failing.push((
                Finding {
                    rule: RULE_LOCKFILE,
                    subject: ws_name.to_owned(),
                    path: lock_rel,
                    line: None,
                    message: format!("lockfile-missing: workspace `{ws_name}` has no Cargo.lock"),
                    repair: "restore or regenerate the workspace Cargo.lock and run `cargo xtask check-stack --write-receipt`".to_owned(),
                },
                KindRecord {
                    rule: RULE_LOCKFILE,
                    kind: "lockfile-missing".to_owned(),
                    fd: None,
                    class: None,
                    fails: true,
                },
            ));
            continue;
        }

        if let Some(nr) = &newest_receipt
            && let Some(rec_lf) = nr.lockfiles.iter().find(|l| l.workspace == ws_name)
            && let Ok(cur_hash) = blake3_file(&ws_lock)
            && cur_hash != rec_lf.hash
        {
            failing.push((
                Finding {
                    rule: RULE_LOCKFILE,
                    subject: ws_name.to_owned(),
                    path: lock_rel,
                    line: None,
                    message: format!(
                        "lockfile-changed: workspace `{ws_name}` Cargo.lock hash `{cur_hash}` differs from newest receipt `{}`",
                        rec_lf.hash
                    ),
                    repair: "run `cargo xtask check-stack --write-receipt` after reviewing Cargo.lock changes".to_owned(),
                },
                KindRecord {
                    rule: RULE_LOCKFILE,
                    kind: "lockfile-changed".to_owned(),
                    fd: None,
                    class: None,
                    fails: true,
                },
            ));
        }
    }

    // Root repeated settings: patches, profile overrides, build_env.
    let (cur_patches, patch_paths_by_crate, cur_profiles) = match fs::read(repo_root.join("Cargo.toml"))
        .ok()
        .and_then(|b| toml::parse(&b).ok())
    {
        Some(tbl) => {
            let (p, map) = receipt::parse_patches(&tbl).unwrap_or_default();
            let prof = receipt::parse_profile_overrides(&tbl);
            (p, map, prof)
        }
        None => (Vec::new(), BTreeMap::new(), Vec::new()),
    };
    let cur_build_env = receipt::parse_build_env(repo_root).unwrap_or_default();

    if let Some(nr) = &newest_receipt {
        for expected_patch in &nr.patches {
            if !cur_patches.contains(expected_patch) {
                failing.push((
                    Finding {
                        rule: RULE_PATCH,
                        subject: expected_patch.clone(),
                        path: "Cargo.toml".to_owned(),
                        line: None,
                        message: format!(
                            "patch-missing: root Cargo.toml is missing `[patch]` entry `{expected_patch}` recorded in newest receipt"
                        ),
                        repair: "restore the repeated `[patch.crates-io]` entry in root Cargo.toml or write a new receipt".to_owned(),
                    },
                    KindRecord {
                        rule: RULE_PATCH,
                        kind: "patch-missing".to_owned(),
                        fd: None,
                        class: None,
                        fails: true,
                    },
                ));
            }
        }
        for expected_prof in &nr.profile_overrides {
            if !cur_profiles.contains(expected_prof) {
                failing.push((
                    Finding {
                        rule: RULE_PROFILE,
                        subject: expected_prof.clone(),
                        path: "Cargo.toml".to_owned(),
                        line: None,
                        message: format!(
                            "profile-override-missing: root Cargo.toml is missing profile override `{expected_prof}` recorded in newest receipt"
                        ),
                        repair: "restore the repeated `[profile.dev.package.*]` override in root Cargo.toml or write a new receipt".to_owned(),
                    },
                    KindRecord {
                        rule: RULE_PROFILE,
                        kind: "profile-override-missing".to_owned(),
                        fd: None,
                        class: None,
                        fails: true,
                    },
                ));
            }
        }
        let mut reported_env_keys: BTreeSet<String> = BTreeSet::new();
        for (k, v) in &nr.build_env {
            if cur_build_env.get(k) != Some(v) {
                reported_env_keys.insert(k.clone());
                let owner_fd = stack_lock
                    .foundations
                    .iter()
                    .find(|row| row.build_env.contains_key(k))
                    .map(|row| row.id.as_str().to_owned());
                failing.push((
                    Finding {
                        rule: RULE_ENV,
                        subject: k.clone(),
                        path: ".cargo/config.toml".to_owned(),
                        line: None,
                        message: format!(
                            "build-env-missing: `.cargo/config.toml` `[env]` key `{k}` is absent or differs from newest receipt (`{v}`)"
                        ),
                        repair: format!(
                            "set `{k} = \"{v}\"` in `.cargo/config.toml` [env] or write a new receipt"
                        ),
                    },
                    KindRecord {
                        rule: RULE_ENV,
                        kind: "build-env-missing".to_owned(),
                        fd: owner_fd,
                        class: None,
                        fails: true,
                    },
                ));
            }
        }
        for row in &stack_lock.foundations {
            for (k, v) in &row.build_env {
                if !reported_env_keys.contains(k) && cur_build_env.get(k) != Some(v) {
                    reported_env_keys.insert(k.clone());
                    failing.push((
                        Finding {
                            rule: RULE_ENV,
                            subject: k.clone(),
                            path: ".cargo/config.toml".to_owned(),
                            line: None,
                            message: format!(
                                "build-env-missing: `.cargo/config.toml` `[env]` key `{k}` differs from `{}` lock entry (`{v}`)",
                                row.id
                            ),
                            repair: format!("set `{k} = \"{v}\"` in `.cargo/config.toml` [env]"),
                        },
                        KindRecord {
                            rule: RULE_ENV,
                            kind: "build-env-missing".to_owned(),
                            fd: Some(row.id.as_str().to_owned()),
                            class: None,
                            fails: true,
                        },
                    ));
                }
            }
        }
    } else {
        for row in &stack_lock.foundations {
            for (k, v) in &row.build_env {
                if cur_build_env.get(k) != Some(v) {
                    failing.push((
                        Finding {
                            rule: RULE_ENV,
                            subject: k.clone(),
                            path: ".cargo/config.toml".to_owned(),
                            line: None,
                            message: format!(
                                "build-env-missing: `.cargo/config.toml` `[env]` key `{k}` differs from `{}` lock entry (`{v}`)",
                                row.id
                            ),
                            repair: format!("set `{k} = \"{v}\"` in `.cargo/config.toml` [env]"),
                        },
                        KindRecord {
                            rule: RULE_ENV,
                            kind: "build-env-missing".to_owned(),
                            fd: Some(row.id.as_str().to_owned()),
                            class: None,
                            fails: true,
                        },
                    ));
                }
            }
        }
    }

    // 5. Compute current resolution via the receipt writer (same code path, no writes).
    let (current_hash, current_siblings, computed_duplicates) = if !tc_mismatch_lock && lockfiles_present {
        match receipt::build_receipt(repo_root, &lock_path) {
            Ok((cur_rec, _, cur_hash)) => (Some(cur_hash), cur_rec.siblings, cur_rec.duplicates),
            Err(_) => {
                let sibs = inspect_siblings_fallback(repo_root, &stack_lock, &patch_paths_by_crate);
                (None, sibs, Vec::new())
            }
        }
    } else {
        let sibs = inspect_siblings_fallback(repo_root, &stack_lock, &patch_paths_by_crate);
        (None, sibs, Vec::new())
    };

    let duplicates = opts
        .duplicates_override
        .clone()
        .unwrap_or(computed_duplicates);

    // 6. Compare lock rows, newest receipt, and current siblings.
    for row in &stack_lock.foundations {
        let fd = row.id.as_str();
        let rec_sib = newest_receipt
            .as_ref()
            .and_then(|nr| nr.siblings.iter().find(|s| s.fd == fd));
        let cur_sib = current_siblings.iter().find(|s| s.fd == fd);

        // Lock vs newest receipt (`stack.pin` / `lock-edited`, plant 7 of GATE-WS-01).
        if let Some(nr) = &newest_receipt {
            let mut edited_reasons = Vec::new();
            if let Some(rs) = rec_sib {
                if let Some(GitRev::Commit(pin)) = &row.git_rev
                    && pin != &rs.rev
                {
                    edited_reasons.push(format!("git_rev `{pin}` != receipt `{}`", rs.rev));
                }
                if row.features != rs.features {
                    edited_reasons.push(format!(
                        "features {:?} != receipt {:?}",
                        row.features, rs.features
                    ));
                }
            }
            for (k, v) in &row.build_env {
                if nr.build_env.get(k) != Some(v) {
                    edited_reasons.push(format!("build_env `{k}` != receipt"));
                }
            }
            if !edited_reasons.is_empty() {
                failing.push((
                    Finding {
                        rule: RULE_PIN,
                        subject: fd.to_owned(),
                        path: "franken-stack.lock".to_owned(),
                        line: None,
                        message: format!(
                            "lock-edited: `{fd}` changed without a new receipt ({})",
                            edited_reasons.join("; ")
                        ),
                        repair: "run `cargo xtask check-stack --write-receipt` to record a new resolution receipt".to_owned(),
                    },
                    KindRecord {
                        rule: RULE_PIN,
                        kind: "lock-edited".to_owned(),
                        fd: Some(fd.to_owned()),
                        class: None,
                        fails: true,
                    },
                ));
            }
        }

        // Current sibling vs lock pin (`stack.pin` / `off-pin`).
        if let Some(cs) = cur_sib {
            if let Some(GitRev::Commit(pin)) = &row.git_rev
                && &cs.rev != pin
            {
                let sib_dir = resolve_sibling_dir(repo_root, row, &patch_paths_by_crate);
                let (cls, reason) = sib_dir
                    .as_deref()
                    .and_then(|d| receipt::classify_checkout(d, Some(pin.as_str())).ok())
                    .unwrap_or((DriftClass::Drift, Some(DriftReason::UnknownPin)));
                let class_desc = format_drift_label(cls, reason);
                failing.push((
                    Finding {
                        rule: RULE_PIN,
                        subject: fd.to_owned(),
                        path: "franken-stack.lock".to_owned(),
                        line: None,
                        message: format!(
                            "off-pin: `{fd}` checkout is at `{}` but lock pins `{pin}` ({class_desc})",
                            cs.rev
                        ),
                        repair: "advance the pin through a compatibility train (`--write-receipt`), never roll back a sibling checkout".to_owned(),
                    },
                    KindRecord {
                        rule: RULE_PIN,
                        kind: "off-pin".to_owned(),
                        fd: Some(fd.to_owned()),
                        class: Some(class_desc),
                        fails: true,
                    },
                ));
            }

            // Current sibling vs newest receipt (`stack.sibling`).
            if let Some(rs) = rec_sib {
                if cs.rev != rs.rev {
                    let sib_dir = resolve_sibling_dir(repo_root, row, &patch_paths_by_crate);
                    let move_class = sib_dir
                        .as_deref()
                        .map_or_else(|| "unknown-pin".to_owned(), |d| classify_commit_move(d, &rs.rev, &cs.rev));
                    failing.push((
                        Finding {
                            rule: RULE_SIBLING,
                            subject: fd.to_owned(),
                            path: row
                                .path
                                .as_ref()
                                .map_or_else(|| fd.to_owned(), |p| p.as_str().to_owned()),
                            line: None,
                            message: format!(
                                "sibling-moved: `{fd}` moved from receipt `{}` to `{}` ({move_class})",
                                rs.rev, cs.rev
                            ),
                            repair: "run the compatibility train and record a new receipt with `--write-receipt`".to_owned(),
                        },
                        KindRecord {
                            rule: RULE_SIBLING,
                            kind: "sibling-moved".to_owned(),
                            fd: Some(fd.to_owned()),
                            class: Some(move_class),
                            fails: true,
                        },
                    ));
                }

                if cs.dirty {
                    if rs.dirty && cs.tree_hash == rs.tree_hash && cs.tree_hash.is_some() {
                        informational.push(KindRecord {
                            rule: RULE_SIBLING,
                            kind: "dirty-unchanged".to_owned(),
                            fd: Some(fd.to_owned()),
                            class: Some("drift".to_owned()),
                            fails: false,
                        });
                    } else {
                        failing.push((
                            Finding {
                                rule: RULE_SIBLING,
                                subject: fd.to_owned(),
                                path: row
                                    .path
                                    .as_ref()
                                    .map_or_else(|| fd.to_owned(), |p| p.as_str().to_owned()),
                                line: None,
                                message: format!(
                                    "dirty-changed: `{fd}` has uncommitted tracked edits differing from newest receipt"
                                ),
                                repair: "commit or coordinate sibling changes, or record a dev receipt with `--write-receipt` (no gate PASS allowed)".to_owned(),
                            },
                            KindRecord {
                                rule: RULE_SIBLING,
                                kind: "dirty-changed".to_owned(),
                                fd: Some(fd.to_owned()),
                                class: Some("dirty".to_owned()),
                                fails: true,
                            },
                        ));
                    }
                }
            } else if cs.dirty {
                failing.push((
                    Finding {
                        rule: RULE_SIBLING,
                        subject: fd.to_owned(),
                        path: row
                            .path
                            .as_ref()
                            .map_or_else(|| fd.to_owned(), |p| p.as_str().to_owned()),
                        line: None,
                        message: format!("dirty-changed: `{fd}` is dirty with no matching receipt entry"),
                        repair: "record a resolution receipt with `--write-receipt`".to_owned(),
                    },
                    KindRecord {
                        rule: RULE_SIBLING,
                        kind: "dirty-changed".to_owned(),
                        fd: Some(fd.to_owned()),
                        class: Some("dirty".to_owned()),
                        fails: true,
                    },
                ));
            }

            // Sibling own_lock disagreement (`stack.own-lock`, only when the sibling carries an own lock file).
            if cs.own_lock.file.is_some() && !cs.own_lock.agrees {
                let class_changed = rec_sib.is_none_or(|rs| rs.own_lock.class != cs.own_lock.class);
                let kind = KindRecord {
                    rule: RULE_OWN_LOCK,
                    kind: "own-lock-disagrees".to_owned(),
                    fd: Some(fd.to_owned()),
                    class: Some(cs.own_lock.class.as_str().to_owned()),
                    fails: class_changed,
                };
                if class_changed {
                    failing.push((
                        Finding {
                            rule: RULE_OWN_LOCK,
                            subject: fd.to_owned(),
                            path: cs
                                .own_lock
                                .file
                                .clone()
                                .unwrap_or_else(|| fd.to_owned()),
                            line: None,
                            message: format!(
                                "own-lock-disagrees: `{fd}` own lock class changed to `{}`",
                                cs.own_lock.class
                            ),
                            repair: "file or advance the upstream sibling lock alignment item and refresh the receipt".to_owned(),
                        },
                        kind,
                    ));
                } else {
                    informational.push(kind);
                }
            }
        }
    }

    let guard_ok = newest_path
        .as_deref()
        .is_some_and(|p| guard::may_record_pass(p).is_ok())
        && !duplicates.iter().any(|d| guard::is_asupersync_duplicate(d))
        && !current_siblings.iter().any(|s| s.dirty);
    let may_record_pass = failing.is_empty() && guard_ok;

    let sib_facts = build_sibling_facts(&stack_lock, &current_siblings);
    Ok(finalize_outcome(
        failing,
        informational,
        Some(stack_lock.toolchain.as_str()),
        Some(&lock_digest_hex),
        &stack_lock.foundations,
        newest_hash.as_deref(),
        current_hash.as_deref(),
        &sib_facts,
        &duplicates,
        may_record_pass,
        None,
    ))
}

// ============================================================================
// Internal helpers (strictly read-only outside `--write-receipt`)
// ============================================================================

#[allow(clippy::too_many_arguments)]
fn finalize_outcome(
    mut failing: Vec<(Finding, KindRecord)>,
    informational: Vec<KindRecord>,
    toolchain: Option<&str>,
    lock_digest: Option<&str>,
    foundations: &[lock::Foundation],
    newest_receipt: Option<&str>,
    current_hash: Option<&str>,
    siblings: &[FactValue],
    duplicates: &[String],
    may_record_pass: bool,
    wrote_receipt: Option<&str>,
) -> CheckOutcome {
    // Sort and dedup paired (Finding, KindRecord) by Finding so `facts.kinds`
    // stays in the exact 1-to-1 order of `report.findings` after CheckReport::new.
    failing.sort_by(|a, b| a.0.cmp(&b.0));
    failing.dedup_by(|a, b| a.0 == b.0);

    let mut findings = Vec::with_capacity(failing.len());
    let mut kinds = Vec::with_capacity(failing.len() + informational.len());
    for (f, k) in failing {
        findings.push(f);
        kinds.push(k);
    }
    kinds.extend(informational);

    let facts = build_facts(
        toolchain,
        lock_digest,
        foundations,
        newest_receipt,
        current_hash,
        siblings,
        duplicates,
        &kinds,
        may_record_pass,
        wrote_receipt,
    );
    CheckOutcome { findings, facts }
}

#[allow(clippy::too_many_arguments)]
fn build_facts(
    toolchain: Option<&str>,
    lock_digest: Option<&str>,
    foundations: &[lock::Foundation],
    newest_receipt: Option<&str>,
    current_hash: Option<&str>,
    siblings: &[FactValue],
    duplicates: &[String],
    kinds: &[KindRecord],
    may_record_pass: bool,
    wrote_receipt: Option<&str>,
) -> BTreeMap<String, FactValue> {
    let mut facts = BTreeMap::new();
    facts.insert("schema".to_owned(), FactValue::str(CHECK_STACK_FACTS_SCHEMA));
    facts.insert(
        "toolchain".to_owned(),
        toolchain.map_or(FactValue::Null, FactValue::str),
    );
    facts.insert(
        "lock_digest".to_owned(),
        lock_digest.map_or(FactValue::Null, FactValue::str),
    );

    let foundation_facts: Vec<FactValue> = foundations
        .iter()
        .map(|row| {
            let mut m = BTreeMap::new();
            m.insert("fd".to_owned(), FactValue::str(row.id.as_str()));
            let posture = row
                .uses
                .first()
                .map_or("unknown", |u| u.posture.as_str());
            m.insert("posture".to_owned(), FactValue::str(posture));
            FactValue::Object(m)
        })
        .collect();
    facts.insert("foundations".to_owned(), FactValue::Array(foundation_facts));
    facts.insert(
        "newest_receipt".to_owned(),
        newest_receipt.map_or(FactValue::Null, FactValue::str),
    );
    facts.insert(
        "current_hash".to_owned(),
        current_hash.map_or(FactValue::Null, FactValue::str),
    );
    facts.insert("siblings".to_owned(), FactValue::Array(siblings.to_vec()));
    facts.insert(
        "duplicates".to_owned(),
        FactValue::str_array(duplicates.iter().cloned()),
    );
    facts.insert(
        "kinds".to_owned(),
        FactValue::Array(kinds.iter().map(KindRecord::to_fact).collect()),
    );
    facts.insert("may_record_pass".to_owned(), FactValue::Bool(may_record_pass));
    facts.insert(
        "wrote_receipt".to_owned(),
        wrote_receipt.map_or(FactValue::Null, FactValue::str),
    );
    facts
}

fn build_sibling_facts(lock: &StackLock, siblings: &[SiblingEntry]) -> Vec<FactValue> {
    siblings
        .iter()
        .map(|s| {
            let mut m = BTreeMap::new();
            m.insert("fd".to_owned(), FactValue::str(&s.fd));
            m.insert("rev".to_owned(), FactValue::str(&s.rev));
            let class_str = s.own_lock.class.as_str();
            let reason_val = s
                .own_lock
                .reason
                .map_or(FactValue::Null, |r| FactValue::str(r.as_str()));
            // Also reflect pin-level class if row exists.
            let _ = lock.foundation(&s.fd);
            m.insert("class".to_owned(), FactValue::str(class_str));
            m.insert("reason".to_owned(), reason_val);
            m.insert("dirty".to_owned(), FactValue::Bool(s.dirty));
            FactValue::Object(m)
        })
        .collect()
}

fn load_newest_receipt(
    receipts_dir: &Path,
    failing: &mut Vec<(Finding, KindRecord)>,
) -> (Option<String>, Option<StackReceipt>, Option<PathBuf>) {
    let head_path = receipts_dir.join("HEAD");
    let raw_head = match fs::read_to_string(&head_path) {
        Ok(s) => s.trim().to_owned(),
        Err(_) => {
            failing.push((
                Finding {
                    rule: RULE_RECEIPT_MISSING,
                    subject: "receipts/HEAD".to_owned(),
                    path: "receipts/HEAD".to_owned(),
                    line: None,
                    message: "no-receipt: `receipts/HEAD` is missing".to_owned(),
                    repair: "run `cargo xtask check-stack --write-receipt` to record receipts/HEAD"
                        .to_owned(),
                },
                KindRecord {
                    rule: RULE_RECEIPT_MISSING,
                    kind: "no-receipt".to_owned(),
                    fd: None,
                    class: None,
                    fails: true,
                },
            ));
            return (None, None, None);
        }
    };

    if raw_head.len() != 64 || !raw_head.bytes().all(|b| b.is_ascii_digit() || matches!(b, b'a'..=b'f')) {
        failing.push((
            Finding {
                rule: RULE_RECEIPT_MISSING,
                subject: "receipts/HEAD".to_owned(),
                path: "receipts/HEAD".to_owned(),
                line: None,
                message: format!("no-receipt: `receipts/HEAD` does not contain a 64-hex hash (`{raw_head}`)"),
                repair: "run `cargo xtask check-stack --write-receipt` to rewrite receipts/HEAD"
                    .to_owned(),
            },
            KindRecord {
                rule: RULE_RECEIPT_MISSING,
                kind: "no-receipt".to_owned(),
                fd: None,
                class: None,
                fails: true,
            },
        ));
        return (None, None, None);
    }

    let receipt_path = receipts_dir.join(format!("{raw_head}.json"));
    if !receipt_path.is_file() {
        failing.push((
            Finding {
                rule: RULE_RECEIPT_MISSING,
                subject: raw_head.clone(),
                path: format!("receipts/{raw_head}.json"),
                line: None,
                message: format!("no-receipt: `receipts/HEAD` points to missing file `receipts/{raw_head}.json`"),
                repair: "restore the cited receipt or run `cargo xtask check-stack --write-receipt`"
                    .to_owned(),
            },
            KindRecord {
                rule: RULE_RECEIPT_MISSING,
                kind: "no-receipt".to_owned(),
                fd: None,
                class: None,
                fails: true,
            },
        ));
        return (Some(raw_head), None, None);
    }

    match receipt::load_and_verify_receipt(&receipt_path) {
        Ok((rec, _, hex)) => (Some(hex), Some(rec), Some(receipt_path)),
        Err(err) => {
            let (rule, kind_name) = match err {
                ReceiptError::HashMismatch { .. }
                | ReceiptError::Canon(_)
                | ReceiptError::Json(_)
                | ReceiptError::InvalidReceipt(_) => (RULE_RECEIPT_HASH, "receipt-tampered"),
                _ => (RULE_RECEIPT_MISSING, "no-receipt"),
            };
            failing.push((
                Finding {
                    rule,
                    subject: raw_head.clone(),
                    path: format!("receipts/{raw_head}.json"),
                    line: None,
                    message: format!("{kind_name}: {err}"),
                    repair: "restore the untampered receipt file or record a new receipt with `--write-receipt`".to_owned(),
                },
                KindRecord {
                    rule,
                    kind: kind_name.to_owned(),
                    fd: None,
                    class: None,
                    fails: true,
                },
            ));
            (Some(raw_head), None, None)
        }
    }
}

fn read_root_toolchain_channel(repo_root: &Path) -> Option<String> {
    let bytes = fs::read(repo_root.join("rust-toolchain.toml")).ok()?;
    let tbl = toml::parse(&bytes).ok()?;
    tbl.get("toolchain")
        .and_then(TomlValue::as_table)
        .and_then(|t| t.get("channel"))
        .and_then(TomlValue::as_str)
        .map(ToOwned::to_owned)
}

fn has_adopted_toolchain_train(receipts_dir: &Path) -> bool {
    let log_path = receipts_dir.join("train-log.ndjson");
    let Ok(text) = fs::read_to_string(log_path) else {
        return false;
    };
    text.lines().any(|line| {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            return false;
        }
        admit(trimmed.as_bytes(), &Limits::default())
            .ok()
            .is_some_and(|adm| {
                adm.value().get("kind").and_then(|v| v.as_str()) == Some("toolchain")
                    && adm.value().get("outcome").and_then(|v| v.as_str()) == Some("adopted")
            })
    })
}

fn inspect_siblings_fallback(
    repo_root: &Path,
    stack_lock: &StackLock,
    patch_paths_by_crate: &BTreeMap<String, String>,
) -> Vec<SiblingEntry> {
    let mut out = Vec::new();
    for row in &stack_lock.foundations {
        let Some(sib_dir) = resolve_sibling_dir(repo_root, row, patch_paths_by_crate) else {
            continue;
        };
        if !sib_dir.is_dir() {
            continue;
        }
        let Ok(rev) = git_head_ro(&sib_dir) else {
            continue;
        };
        let dirty = git_dirty_ro(&sib_dir).unwrap_or(false);
        let sib_tree_hash = if dirty {
            receipt::tree_hash(&sib_dir).ok()
        } else {
            None
        };
        let pin = row.git_rev.as_ref().map(GitRev::as_str);
        let own_lock = receipt::classify_sibling_own_lock(&sib_dir, pin).unwrap_or(OwnLockEntry {
            file: None,
            hash: None,
            agrees: false,
            class: DriftClass::Drift,
            reason: Some(DriftReason::UnknownPin),
        });
        out.push(SiblingEntry {
            fd: row.id.as_str().to_owned(),
            rev,
            dirty,
            tree_hash: sib_tree_hash,
            features: row.features.clone(),
            own_lock,
        });
    }
    out.sort_by(|a, b| a.fd.cmp(&b.fd));
    out
}

fn resolve_sibling_dir(
    repo_root: &Path,
    row: &lock::Foundation,
    patch_paths_by_crate: &BTreeMap<String, String>,
) -> Option<PathBuf> {
    let rel = row
        .path
        .as_ref()
        .map(|p| p.as_str().to_owned())
        .or_else(|| patch_paths_by_crate.get(&row.name).cloned())?;
    let mut out = repo_root.to_path_buf();
    for comp in Path::new(&rel).components() {
        match comp {
            Component::CurDir => {}
            Component::ParentDir => {
                let _ = out.pop();
            }
            Component::Normal(seg) => out.push(seg),
            Component::RootDir | Component::Prefix(_) => return None,
        }
    }
    Some(out)
}

fn classify_commit_move(sib_dir: &Path, old_rev: &str, new_rev: &str) -> String {
    let old_spec = format!("{old_rev}^{{commit}}");
    if !git_ro_ok(sib_dir, &["cat-file", "-e", &old_spec]) {
        return "unknown-pin".to_owned();
    }
    if git_ro_ok(sib_dir, &["merge-base", "--is-ancestor", old_rev, new_rev]) {
        return "fast-forward".to_owned();
    }
    if git_ro_ok(sib_dir, &["merge-base", "--is-ancestor", new_rev, old_rev]) {
        return "retreated".to_owned();
    }
    "diverged".to_owned()
}

fn format_drift_label(cls: DriftClass, reason: Option<DriftReason>) -> String {
    match (cls, reason) {
        (DriftClass::Drift, Some(r)) => format!("drift:{}", r.as_str()),
        (c, _) => c.as_str().to_owned(),
    }
}

fn non_root_fd(row: &str) -> Option<String> {
    if row.starts_with("FD-") {
        Some(row.to_owned())
    } else {
        None
    }
}

fn atomic_write_line(dest: &Path, value: &str) -> Result<(), String> {
    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("create_dir_all: {e}"))?;
    }
    static SEQ: AtomicU64 = AtomicU64::new(0);
    let seq = SEQ.fetch_add(1, Ordering::Relaxed);
    let name = dest
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("file");
    let tmp = dest.with_file_name(format!(".{name}-{}-{seq}.tmp", std::process::id()));
    let mut f = File::create(&tmp).map_err(|e| format!("create temp: {e}"))?;
    let payload = format!("{value}\n");
    f.write_all(payload.as_bytes())
        .and_then(|()| f.sync_all())
        .map_err(|e| format!("write/sync temp: {e}"))?;
    drop(f);
    fs::rename(&tmp, dest).map_err(|e| format!("rename: {e}"))?;
    Ok(())
}

fn blake3_file(path: &Path) -> Result<String, String> {
    let mut f = File::open(path).map_err(|e| e.to_string())?;
    let mut hasher = fs_blake3::Blake3::new();
    let mut buf = [0u8; 65_536];
    loop {
        let n = f.read(&mut buf).map_err(|e| e.to_string())?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }
    Ok(hasher.finalize().to_hex())
}

fn bytes_to_lower_hex(bytes: &[u8; 32]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut s = String::with_capacity(64);
    for &b in bytes {
        s.push(HEX[(b >> 4) as usize] as char);
        s.push(HEX[(b & 0x0F) as usize] as char);
    }
    s
}

fn git_head_ro(dir: &Path) -> Result<String, String> {
    let out = Command::new("git")
        .arg("--no-optional-locks")
        .arg("-C")
        .arg(dir)
        .args(["rev-parse", "HEAD"])
        .output()
        .map_err(|e| e.to_string())?;
    if !out.status.success() {
        return Err(String::from_utf8_lossy(&out.stderr).trim().to_owned());
    }
    Ok(String::from_utf8_lossy(&out.stdout).trim().to_owned())
}

fn git_dirty_ro(dir: &Path) -> Result<bool, String> {
    let out = Command::new("git")
        .arg("--no-optional-locks")
        .arg("-C")
        .arg(dir)
        .args(["status", "--porcelain", "--untracked-files=no"])
        .output()
        .map_err(|e| e.to_string())?;
    if !out.status.success() {
        return Err(String::from_utf8_lossy(&out.stderr).trim().to_owned());
    }
    Ok(!out.stdout.iter().all(u8::is_ascii_whitespace))
}

fn git_ro_ok(dir: &Path, args: &[&str]) -> bool {
    Command::new("git")
        .arg("--no-optional-locks")
        .arg("-C")
        .arg(dir)
        .args(args)
        .output()
        .is_ok_and(|o| o.status.success())
}

// ============================================================================
// Unit tests (`cargo test -p xtask -- stack::verify stack::guard`)
// ============================================================================

#[cfg(test)]
mod tests {
    use std::process::ExitCode;

    use super::*;
    use crate::cli::{self, CheckFn, EXIT_FINDINGS, EXIT_OK, EXIT_USAGE};
    use crate::report::validate_report_json;

    const TEST_COMMANDS: &[(&str, CheckFn)] = &[("check-stack", run_command)];

    fn fresh_dir(tag: &str) -> PathBuf {
        static CTR: AtomicU64 = AtomicU64::new(0);
        let dir = std::env::temp_dir().join(format!(
            "frob_xtask_verify_{tag}_{}_{}",
            std::process::id(),
            CTR.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(&dir).expect("mkdir verify fixture");
        dir
    }

    fn git_fixture(dir: &Path, args: &[&str]) -> String {
        let out = Command::new("git")
            .arg("-C")
            .arg(dir)
            .args(["-c", "commit.gpgsign=false", "-c", "core.autocrlf=false"])
            .args(args)
            .env("GIT_AUTHOR_NAME", "synthetic")
            .env("GIT_AUTHOR_EMAIL", "synthetic@example.com")
            .env("GIT_AUTHOR_DATE", "2026-11-24T00:00:00Z")
            .env("GIT_COMMITTER_NAME", "synthetic")
            .env("GIT_COMMITTER_EMAIL", "synthetic@example.com")
            .env("GIT_COMMITTER_DATE", "2026-11-24T00:00:00Z")
            .output()
            .expect("spawn synthetic git");
        assert!(
            out.status.success(),
            "synthetic git {:?} failed: {}",
            args,
            String::from_utf8_lossy(&out.stderr)
        );
        String::from_utf8_lossy(&out.stdout).trim().to_owned()
    }

    fn commit_file(dir: &Path, rel: &str, content: &str, msg: &str) -> String {
        let p = dir.join(rel);
        if let Some(parent) = p.parent() {
            fs::create_dir_all(parent).expect("mkdir parent");
        }
        fs::write(&p, content).expect("write fixture file");
        git_fixture(dir, &["add", "--", rel]);
        git_fixture(dir, &["commit", "-q", "-m", msg]);
        git_fixture(dir, &["rev-parse", "HEAD"])
    }

    struct SyntheticFixture {
        base: PathBuf,
        repo: PathBuf,
        sib: PathBuf,
        sib_c1: String,
        sib_c2: String,
        sib_c3: String,
        receipt_hash: String,
    }

    fn build_clean_fixture(tag: &str) -> SyntheticFixture {
        let base = fresh_dir(tag);
        let sib = base.join("frankensim");
        fs::create_dir_all(&sib).expect("mkdir sib");
        git_fixture(&sib, &["init", "-q", "-b", "main"]);
        fs::write(
            sib.join("Cargo.toml"),
            "[package]\nname = \"sib-fsim\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
        )
        .expect("sib manifest");
        fs::create_dir_all(sib.join("src")).expect("sib src");
        fs::write(sib.join("src/lib.rs"), "pub const V: u32 = 1;\n").expect("sib lib");
        git_fixture(&sib, &["add", "."]);
        git_fixture(&sib, &["commit", "-q", "-m", "c1"]);
        let sib_c1 = git_fixture(&sib, &["rev-parse", "HEAD"]);
        let sib_c2 = commit_file(&sib, "src/lib.rs", "pub const V: u32 = 2;\n", "c2");
        let sib_c3 = commit_file(&sib, "src/lib.rs", "pub const V: u32 = 3;\n", "c3");
        // Start sibling on c2 so both fast-forward (c3) and retreated (c1) exist.
        git_fixture(&sib, &["switch", "--detach", "-q", &sib_c2]);

        let repo = base.join("repo");
        fs::create_dir_all(&repo).expect("mkdir repo");
        git_fixture(&repo, &["init", "-q", "-b", "main"]);
        fs::write(
            repo.join("rust-toolchain.toml"),
            "[toolchain]\nchannel = \"nightly-2026-08-31\"\n",
        )
        .expect("toolchain");
        fs::create_dir_all(repo.join(".cargo")).expect(".cargo");
        fs::write(
            repo.join(".cargo/config.toml"),
            "[env]\nFRANKENSIM_DEPGRAPH_SALT = \"synthetic-dev-v1\"\n",
        )
        .expect("config");
        fs::write(
            repo.join("Cargo.toml"),
            "[workspace]\nresolver = \"3\"\nmembers = [\"crates/app\"]\n\n\
[patch.crates-io]\nsib-fsim = { path = \"../frankensim\" }\n\n\
[profile.dev.package.fsqlite]\nopt-level = 1\n",
        )
        .expect("root Cargo.toml");
        fs::create_dir_all(repo.join("crates/app/src")).expect("app src");
        fs::write(
            repo.join("crates/app/Cargo.toml"),
            "[package]\nname = \"app\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\nsib-fsim = { path = \"../../../frankensim\" }\n",
        )
        .expect("app manifest");
        fs::write(repo.join("crates/app/src/lib.rs"), "pub fn v() -> u32 { sib_fsim::V }\n")
            .expect("app lib");
        fs::write(
            repo.join("Cargo.lock"),
            "# This file is automatically @generated by Cargo.\nversion = 4\n\n\
[[package]]\nname = \"app\"\nversion = \"0.1.0\"\ndependencies = [\n \"sib-fsim\",\n]\n\n\
[[package]]\nname = \"sib-fsim\"\nversion = \"0.1.0\"\n\n\
[[patch.unused]]\nname = \"sib-fsim\"\nversion = \"0.1.0\"\n",
        )
        .expect("Cargo.lock");
        fs::create_dir_all(repo.join("registries")).expect("registries");
        fs::write(
            repo.join("registries/lifts.toml"),
            "schema = \"frob.lifts/1\"\n",
        )
        .expect("lifts.toml");
        fs::write(
            repo.join("franken-stack.lock"),
            format!(
                "toolchain = \"nightly-2026-08-31\"\n\n\
[[foundation]]\nid = \"FD-FSIM\"\nname = \"frankensim\"\nposture = \"consume-path\"\n\
path = \"../frankensim\"\ngit_rev = \"{sib_c2}\"\nfeatures = [\"referee\", \"geometry\", \"ledger\"]\n\
build_env = {{ FRANKENSIM_DEPGRAPH_SALT = \"synthetic-dev-v1\" }}\n\
confined_to = [\"app\"]\nadmission_gate = \"INT-FSIM-01\"\nfallback = \"KILL-FSIM\"\n"
            ),
        )
        .expect("stack lock");

        git_fixture(&repo, &["add", "."]);
        git_fixture(&repo, &["commit", "-q", "-m", "init"]);

        // Write initial receipt, HEAD, and LOCK_DIGEST via `--write-receipt`.
        let out = run_write_receipt(&repo, &VerifyOptions::default()).expect("write initial receipt");
        let receipt_hash = match out.facts.get("wrote_receipt") {
            Some(FactValue::Str(s)) => s.clone(),
            other => panic!("expected wrote_receipt hash, got {other:?}"),
        };
        SyntheticFixture {
            base,
            repo,
            sib,
            sib_c1,
            sib_c2,
            sib_c3,
            receipt_hash,
        }
    }

    fn run_cli_json(repo: &Path, extra: &[&str]) -> (ExitCode, String, String) {
        let mut args = vec![
            "check-stack".to_owned(),
            "--root".to_owned(),
            repo.to_string_lossy().into_owned(),
            "--json".to_owned(),
        ];
        args.extend(extra.iter().map(|s| (*s).to_owned()));
        let mut out = Vec::new();
        let mut err = Vec::new();
        let code = cli::run(TEST_COMMANDS, &args, &mut out, &mut err);
        (
            code,
            String::from_utf8(out).expect("stdout utf8"),
            String::from_utf8(err).expect("stderr utf8"),
        )
    }

    fn kinds_from(outcome: &CheckOutcome) -> Vec<(String, String, Option<String>, bool)> {
        let Some(FactValue::Array(arr)) = outcome.facts.get("kinds") else {
            return Vec::new();
        };
        arr.iter()
            .filter_map(|v| match v {
                FactValue::Object(m) => {
                    let rule = match m.get("rule") { Some(FactValue::Str(s)) => s.clone(), _ => return None };
                    let kind = match m.get("kind") { Some(FactValue::Str(s)) => s.clone(), _ => return None };
                    let class = match m.get("class") { Some(FactValue::Str(s)) => Some(s.clone()), _ => None };
                    let fails = match m.get("fails") { Some(FactValue::Bool(b)) => *b, _ => false };
                    Some((rule, kind, class, fails))
                }
                _ => None,
            })
            .collect()
    }

    fn digest_tree(dir: &Path) -> String {
        let mut files = Vec::new();
        let mut stack = vec![dir.to_path_buf()];
        while let Some(cur) = stack.pop() {
            if let Ok(entries) = fs::read_dir(&cur) {
                for ent in entries.flatten() {
                    let p = ent.path();
                    if p.is_dir() {
                        stack.push(p);
                    } else {
                        files.push(p);
                    }
                }
            }
        }
        files.sort();
        let mut h = fs_blake3::Blake3::new();
        for p in files {
            h.update(p.strip_prefix(dir).expect("strip").to_string_lossy().as_bytes());
            h.update(&[0]);
            if let Ok(b) = fs::read(&p) {
                h.update(&b);
            }
            h.update(&[0]);
        }
        h.finalize().to_hex()
    }

    #[cfg(unix)]
    fn set_tree_writable(dir: &Path, writable: bool) {
        use std::os::unix::fs::PermissionsExt;
        let mut dirs = Vec::new();
        let mut stack = vec![dir.to_path_buf()];
        while let Some(cur) = stack.pop() {
            if writable {
                let _ = fs::set_permissions(&cur, fs::Permissions::from_mode(0o755));
            }
            if let Ok(entries) = fs::read_dir(&cur) {
                for ent in entries.flatten() {
                    let p = ent.path();
                    if p.is_dir() {
                        stack.push(p);
                    } else if writable {
                        let _ = fs::set_permissions(&p, fs::Permissions::from_mode(0o644));
                    } else {
                        let _ = fs::set_permissions(&p, fs::Permissions::from_mode(0o444));
                    }
                }
            }
            if !writable {
                dirs.push(cur);
            }
        }
        for d in dirs.into_iter().rev() {
            let _ = fs::set_permissions(&d, fs::Permissions::from_mode(0o555));
        }
    }

    #[test]
    fn verify_clean_passes() {
        let fx = build_clean_fixture("clean");
        // Also verify read-only sibling checkout succeeds and remains byte-identical (.git included).
        let before = digest_tree(&fx.sib);
        #[cfg(unix)]
        set_tree_writable(&fx.sib, false);
        let outcome = verify_repo(&fx.repo, &VerifyOptions::default()).expect("verify");
        #[cfg(unix)]
        set_tree_writable(&fx.sib, true);
        assert_eq!(before, digest_tree(&fx.sib), "sibling must be byte-identical");

        assert!(outcome.findings.is_empty(), "expected clean, got {:?}", outcome.findings);
        assert_eq!(outcome.facts.get("may_record_pass"), Some(&FactValue::Bool(true)));
    }

    #[test]
    fn verify_lockfile_change_fails() {
        let fx = build_clean_fixture("lf_change");
        let mut lock_txt = fs::read_to_string(fx.repo.join("Cargo.lock")).expect("read");
        lock_txt.push('\n');
        fs::write(fx.repo.join("Cargo.lock"), lock_txt).expect("append byte");

        let outcome = verify_repo(&fx.repo, &VerifyOptions::default()).expect("verify");
        assert!(outcome.findings.iter().any(|f| f.rule == RULE_LOCKFILE));
        assert!(kinds_from(&outcome).iter().any(|(r, k, _, fl)| r == RULE_LOCKFILE && k == "lockfile-changed" && *fl));

        // After --write-receipt, verify passes again.
        let _ = run_write_receipt(&fx.repo, &VerifyOptions::default()).expect("rewrite receipt");
        let after = verify_repo(&fx.repo, &VerifyOptions::default()).expect("re-verify");
        assert!(after.findings.is_empty(), "after --write-receipt expected clean: {:?}", after.findings);
    }

    #[test]
    fn verify_toolchain_change_fails() {
        let fx = build_clean_fixture("tc_change");
        fs::write(
            fx.repo.join("rust-toolchain.toml"),
            "[toolchain]\nchannel = \"nightly-2026-08-25\"\n",
        )
        .expect("edit toolchain");
        let outcome = verify_repo(&fx.repo, &VerifyOptions::default()).expect("verify");
        assert!(outcome.findings.iter().any(|f| f.rule == RULE_TOOLCHAIN));
        assert!(kinds_from(&outcome).iter().any(|(r, k, _, fl)| r == RULE_TOOLCHAIN && k == "toolchain-changed" && *fl));
    }

    #[test]
    fn verify_sibling_fast_forward() {
        let fx = build_clean_fixture("sib_ff");
        git_fixture(&fx.sib, &["switch", "--detach", "-q", &fx.sib_c3]);
        let outcome = verify_repo(&fx.repo, &VerifyOptions::default()).expect("verify");
        assert!(
            kinds_from(&outcome)
                .iter()
                .any(|(r, k, cls, fl)| r == RULE_SIBLING && k == "sibling-moved" && cls.as_deref() == Some("fast-forward") && *fl),
            "expected sibling-moved fast-forward, got {:?}",
            kinds_from(&outcome)
        );
    }

    #[test]
    fn verify_sibling_retreated() {
        let fx = build_clean_fixture("sib_ret");
        git_fixture(&fx.sib, &["switch", "--detach", "-q", &fx.sib_c1]);
        let outcome = verify_repo(&fx.repo, &VerifyOptions::default()).expect("verify");
        assert!(
            kinds_from(&outcome)
                .iter()
                .any(|(r, k, cls, fl)| r == RULE_SIBLING && k == "sibling-moved" && cls.as_deref() == Some("retreated") && *fl),
            "expected sibling-moved retreated, got {:?}",
            kinds_from(&outcome)
        );
    }

    #[test]
    fn verify_sibling_diverged() {
        let fx = build_clean_fixture("sib_div");
        git_fixture(&fx.sib, &["switch", "-q", "-c", "side", &fx.sib_c1]);
        let _ = commit_file(&fx.sib, "src/lib.rs", "pub const V: u32 = 99;\n", "side");
        let outcome = verify_repo(&fx.repo, &VerifyOptions::default()).expect("verify");
        assert!(
            kinds_from(&outcome)
                .iter()
                .any(|(r, k, cls, fl)| r == RULE_SIBLING && k == "sibling-moved" && cls.as_deref() == Some("diverged") && *fl),
            "expected sibling-moved diverged, got {:?}",
            kinds_from(&outcome)
        );
    }

    #[test]
    fn verify_dirty_unchanged_is_dev_ok() {
        let fx = build_clean_fixture("dirty_unchanged");
        fs::write(fx.sib.join("src/lib.rs"), "pub const V: u32 = 42;\n").expect("dirty edit");
        let _ = run_write_receipt(&fx.repo, &VerifyOptions::default()).expect("record dirty receipt");

        let (code, stdout, _) = run_cli_json(&fx.repo, &[]);
        assert_eq!(code, ExitCode::from(EXIT_OK), "dirty-unchanged must exit 0: {stdout}");
        assert!(validate_report_json(stdout.as_bytes()).is_ok());

        let outcome = verify_repo(&fx.repo, &VerifyOptions::default()).expect("verify");
        assert!(outcome.findings.is_empty());
        assert_eq!(outcome.facts.get("may_record_pass"), Some(&FactValue::Bool(false)));
        assert!(
            kinds_from(&outcome)
                .iter()
                .any(|(r, k, _, fl)| r == RULE_SIBLING && k == "dirty-unchanged" && !*fl)
        );
    }

    #[test]
    fn verify_dirty_changed_fails() {
        let fx = build_clean_fixture("dirty_changed");
        fs::write(fx.sib.join("src/lib.rs"), "pub const V: u32 = 7;\n").expect("dirty edit 1");
        let outcome = verify_repo(&fx.repo, &VerifyOptions::default()).expect("verify");
        assert!(
            kinds_from(&outcome)
                .iter()
                .any(|(r, k, _, fl)| r == RULE_SIBLING && k == "dirty-changed" && *fl)
        );

        // Record dirty receipt, then change tracked file again -> must still fail dirty-changed.
        let _ = run_write_receipt(&fx.repo, &VerifyOptions::default()).expect("record dirty 1");
        fs::write(fx.sib.join("src/lib.rs"), "pub const V: u32 = 8;\n").expect("dirty edit 2");
        let outcome2 = verify_repo(&fx.repo, &VerifyOptions::default()).expect("verify 2");
        assert!(
            kinds_from(&outcome2)
                .iter()
                .any(|(r, k, _, fl)| r == RULE_SIBLING && k == "dirty-changed" && *fl)
        );
    }

    #[test]
    fn duplicates_are_recorded_not_failed() {
        let fx = build_clean_fixture("dups_ok");
        let opts = VerifyOptions {
            admitted: BTreeSet::new(),
            duplicates_override: Some(vec![
                "asupersync 0.5.0".to_owned(),
                "asupersync 0.6.2".to_owned(),
            ]),
        };
        let outcome = verify_repo(&fx.repo, &opts).expect("verify");
        assert!(outcome.findings.is_empty(), "duplicates must not be check-stack findings");
        assert_eq!(outcome.facts.get("may_record_pass"), Some(&FactValue::Bool(false)));
        assert_eq!(
            outcome.facts.get("duplicates"),
            Some(&FactValue::str_array(["asupersync 0.5.0", "asupersync 0.6.2"]))
        );
    }

    #[test]
    fn lock_edit_without_receipt_fails() {
        let fx = build_clean_fixture("lock_edit");
        let lock_path = fx.repo.join("franken-stack.lock");
        let orig = fs::read_to_string(&lock_path).expect("read lock");

        // Subcase 1: edit git_rev -> stack.pin with BOTH lock-edited and off-pin.
        fs::write(&lock_path, orig.replace(&fx.sib_c2, &fx.sib_c3)).expect("edit git_rev");
        let o1 = verify_repo(&fx.repo, &VerifyOptions::default()).expect("verify 1");
        let k1 = kinds_from(&o1);
        assert!(k1.iter().any(|(r, k, _, fl)| r == RULE_PIN && k == "lock-edited" && *fl));
        assert!(k1.iter().any(|(r, k, _, fl)| r == RULE_PIN && k == "off-pin" && *fl));

        // Subcase 2: edit features -> stack.pin lock-edited.
        fs::write(&lock_path, orig.replace("\"ledger\"", "\"stats\"")).expect("edit features");
        let o2 = verify_repo(&fx.repo, &VerifyOptions::default()).expect("verify 2");
        assert!(kinds_from(&o2).iter().any(|(r, k, _, fl)| r == RULE_PIN && k == "lock-edited" && *fl));
    }

    #[test]
    fn lockfile_missing_fails() {
        let fx = build_clean_fixture("lf_missing");
        let aside = fx.base.join("aside");
        fs::create_dir_all(&aside).expect("mkdir aside");
        fs::rename(fx.repo.join("Cargo.lock"), aside.join("Cargo.lock")).expect("move Cargo.lock aside");

        let outcome = verify_repo(&fx.repo, &VerifyOptions::default()).expect("verify");
        assert!(
            kinds_from(&outcome)
                .iter()
                .any(|(r, k, _, fl)| r == RULE_LOCKFILE && k == "lockfile-missing" && *fl)
        );
    }

    #[test]
    fn receipt_name_mismatch_fails() {
        let fx = build_clean_fixture("tamper");
        let rp = fx.repo.join("receipts").join(format!("{}.json", fx.receipt_hash));
        let mut bytes = fs::read(&rp).expect("read receipt");
        bytes.push(b' ');
        fs::write(&rp, bytes).expect("tamper");

        let outcome = verify_repo(&fx.repo, &VerifyOptions::default()).expect("verify");
        assert!(
            kinds_from(&outcome)
                .iter()
                .any(|(r, k, _, fl)| r == RULE_RECEIPT_HASH && k == "receipt-tampered" && *fl)
        );
    }

    #[test]
    fn finding_ids_are_namespaced() {
        for &rule in OWNED_RULES {
            assert!(
                rule.starts_with("stack."),
                "rule `{rule}` must start with `stack.`"
            );
        }
        let mut seen: BTreeMap<&str, &str> = BTreeMap::new();
        for &(kind, rule) in KIND_TO_RULE {
            assert!(
                rule.starts_with("stack."),
                "rule `{rule}` for kind `{kind}` must start with `stack.`"
            );
            assert!(
                OWNED_RULES.contains(&rule),
                "rule `{rule}` not listed in OWNED_RULES"
            );
            if let Some(prev) = seen.insert(kind, rule) {
                assert_eq!(
                    prev, rule,
                    "kind `{kind}` mapped to multiple rules: `{prev}` vs `{rule}`"
                );
            }
        }
        for &rule in OWNED_RULES {
            assert!(
                KIND_TO_RULE.iter().any(|&(_, r)| r == rule),
                "owned rule `{rule}` has no entry in KIND_TO_RULE"
            );
        }
    }

    #[test]
    fn exit_codes() {
        let fx = build_clean_fixture("exit_codes");
        let (code_ok, out_ok, _) = run_cli_json(&fx.repo, &[]);
        assert_eq!(code_ok, ExitCode::from(EXIT_OK));
        assert!(validate_report_json(out_ok.as_bytes()).is_ok());

        let (code_usage, _, _) = run_cli_json(&fx.repo, &["--unknown-flag"]);
        assert_eq!(code_usage, ExitCode::from(EXIT_USAGE));

        fs::write(
            fx.repo.join("rust-toolchain.toml"),
            "[toolchain]\nchannel = \"nightly-2026-08-25\"\n",
        )
        .expect("break toolchain");
        let (code_fail, out_fail, _) = run_cli_json(&fx.repo, &[]);
        assert_eq!(code_fail, ExitCode::from(EXIT_FINDINGS));
        assert!(validate_report_json(out_fail.as_bytes()).is_ok());
    }

    #[test]
    fn report_is_common_form() {
        let fx = build_clean_fixture("common_form");
        let lock_path = fx.repo.join("franken-stack.lock");
        let orig = fs::read_to_string(&lock_path).expect("read");
        // Plant lock-edited and add second template row (FD-ASUP).
        let edited = format!(
            "{}\n[[foundation]]\nid = \"FD-ASUP\"\nname = \"asupersync\"\nposture = \"consume\"\nversion = \"=0.7.0\"\nadmission_gate = \"INT-ASUP-01\"\nfallback = \"KILL-MCP\"\n",
            orig.replace(&fx.sib_c2, &fx.sib_c3)
        );
        fs::write(&lock_path, edited).expect("write edited lock");

        let (code, stdout, _) = run_cli_json(&fx.repo, &[]);
        assert_eq!(code, ExitCode::from(EXIT_FINDINGS));
        validate_report_json(stdout.as_bytes()).expect("valid frob.xtask.report/1");

        let adm = admit(stdout.as_bytes(), &Limits::default()).expect("admit report");
        let root = adm.value();
        assert_eq!(root.get("check").and_then(|v| v.as_str()), Some("check-stack"));
        let findings = root.get("findings").and_then(|v| v.as_array()).expect("findings");
        assert_eq!(findings[0].get("rule").and_then(|v| v.as_str()), Some(RULE_PIN));

        let facts = root.get("facts").expect("facts");
        let kinds = facts.get("kinds").and_then(|v| v.as_array()).expect("kinds");
        assert_eq!(kinds[0].get("kind").and_then(|v| v.as_str()), Some("lock-edited"));

        let fds = facts.get("foundations").and_then(|v| v.as_array()).expect("foundations");
        assert_eq!(fds.len(), 2);
        let fd_ids: BTreeSet<&str> = fds
            .iter()
            .filter_map(|f| f.get("fd").and_then(|v| v.as_str()))
            .collect();
        assert_eq!(fd_ids, BTreeSet::from(["FD-ASUP", "FD-FSIM"]));
    }

    #[test]
    fn verify_root_settings_missing() {
        // Subcase 1: missing [patch.crates-io] entry.
        let fx1 = build_clean_fixture("missing_patch");
        let c1 = fs::read_to_string(fx1.repo.join("Cargo.toml")).expect("read");
        fs::write(
            fx1.repo.join("Cargo.toml"),
            c1.replace("[patch.crates-io]\nsib-fsim = { path = \"../frankensim\" }\n", ""),
        )
        .expect("drop patch");
        let o1 = verify_repo(&fx1.repo, &VerifyOptions::default()).expect("verify 1");
        assert!(kinds_from(&o1).iter().any(|(r, k, _, fl)| r == RULE_PATCH && k == "patch-missing" && *fl));

        // Subcase 2: missing [profile.dev.package.fsqlite] override.
        let fx2 = build_clean_fixture("missing_prof");
        let c2 = fs::read_to_string(fx2.repo.join("Cargo.toml")).expect("read");
        fs::write(
            fx2.repo.join("Cargo.toml"),
            c2.replace("[profile.dev.package.fsqlite]\nopt-level = 1\n", ""),
        )
        .expect("drop profile");
        let o2 = verify_repo(&fx2.repo, &VerifyOptions::default()).expect("verify 2");
        assert!(kinds_from(&o2).iter().any(|(r, k, _, fl)| r == RULE_PROFILE && k == "profile-override-missing" && *fl));

        // Subcase 3: missing .cargo/config.toml [env] variable.
        let fx3 = build_clean_fixture("missing_env");
        fs::write(fx3.repo.join(".cargo/config.toml"), "[env]\n").expect("drop env");
        let o3 = verify_repo(&fx3.repo, &VerifyOptions::default()).expect("verify 3");
        assert!(kinds_from(&o3).iter().any(|(r, k, _, fl)| r == RULE_ENV && k == "build-env-missing" && *fl));
    }

    #[test]
    fn verify_reports_lift_findings() {
        let fx = build_clean_fixture("lifts");
        // Empty shipped registries/lifts.toml -> zero stack.lift findings.
        let clean = verify_repo(&fx.repo, &VerifyOptions::default()).expect("clean");
        assert!(!clean.findings.iter().any(|f| f.rule == RULE_LIFT));

        // Plant unregistered `// LIFTED from` file -> stack.lift finding.
        fs::write(
            fx.repo.join("crates/app/src/lifted.rs"),
            "// LIFTED from frankensim/src/x.rs at d02ad9c170d5932667d15359cd362d3198ac4806\npub fn x() {}\n",
        )
        .expect("plant lifted");
        let planted = verify_repo(&fx.repo, &VerifyOptions::default()).expect("planted");
        assert!(planted.findings.iter().any(|f| f.rule == RULE_LIFT));
        assert!(
            kinds_from(&planted)
                .iter()
                .any(|(r, k, _, fl)| r == RULE_LIFT && k == "lifts.unregistered" && *fl),
            "expected lifts.unregistered kind, got {:?}",
            kinds_from(&planted)
        );
        let (code, _, _) = run_cli_json(&fx.repo, &[]);
        assert_eq!(code, ExitCode::from(EXIT_FINDINGS));
    }

    #[test]
    fn verify_reports_admitted_rules() {
        let fx = build_clean_fixture("admitted_rules");
        let lock_path = fx.repo.join("franken-stack.lock");
        let orig = fs::read_to_string(&lock_path).expect("read");
        fs::write(&lock_path, orig.replace(&fx.sib_c2, "unadmitted")).expect("set unadmitted");
        let _ = run_write_receipt(&fx.repo, &VerifyOptions::default()).expect("write receipt");

        // Empty admitted set -> no stack.lock-schema finding.
        let empty = verify_repo(&fx.repo, &VerifyOptions::default()).expect("empty admitted");
        assert!(!empty.findings.iter().any(|f| f.rule == RULE_LOCK_SCHEMA));

        // With INT-FSIM-01 in admitted set -> stack.lock-schema rule 5.
        let mut admitted = BTreeSet::new();
        admitted.insert(GateRef::parse("INT-FSIM-01", "FD-FSIM").expect("gate"));
        let opts = VerifyOptions {
            admitted,
            duplicates_override: None,
        };
        let with_gate = verify_repo(&fx.repo, &opts).expect("with gate");
        assert!(
            with_gate
                .findings
                .iter()
                .any(|f| f.rule == RULE_LOCK_SCHEMA && f.message.contains("rule 5")),
            "expected stack.lock-schema rule 5, got {:?}",
            with_gate.findings
        );
    }

    #[test]
    fn write_receipt_writes_head_and_lock_digest() {
        let fx = build_clean_fixture("write_head");
        let lock = lock::load(&fx.repo.join("franken-stack.lock")).expect("load lock");
        let expected_lock_digest = bytes_to_lower_hex(&lock.digest());
        let receipts_dir = fx.repo.join("receipts");

        assert_eq!(
            fs::read_to_string(receipts_dir.join("HEAD")).expect("HEAD").trim(),
            fx.receipt_hash
        );
        assert_eq!(
            fs::read_to_string(receipts_dir.join("LOCK_DIGEST")).expect("LOCK_DIGEST").trim(),
            expected_lock_digest
        );

        // Bootstrap simulation: receipt present, HEAD and LOCK_DIGEST moved aside.
        let aside = fx.base.join("aside_boot");
        fs::create_dir_all(&aside).expect("aside");
        fs::rename(receipts_dir.join("HEAD"), aside.join("HEAD")).expect("move HEAD");
        fs::rename(receipts_dir.join("LOCK_DIGEST"), aside.join("LOCK_DIGEST")).expect("move LD");

        // Also test read-only sibling during --write-receipt.
        let sib_before = digest_tree(&fx.sib);
        #[cfg(unix)]
        set_tree_writable(&fx.sib, false);
        let out = run_write_receipt(&fx.repo, &VerifyOptions::default()).expect("bootstrap write");
        #[cfg(unix)]
        set_tree_writable(&fx.sib, true);
        assert_eq!(sib_before, digest_tree(&fx.sib));
        assert_eq!(out.facts.get("wrote_receipt"), Some(&FactValue::str(&fx.receipt_hash)));

        let json_count = fs::read_dir(&receipts_dir)
            .expect("read_dir")
            .flatten()
            .filter(|e| e.path().extension().and_then(|x| x.to_str()) == Some("json"))
            .count();
        assert_eq!(json_count, 1, "must not write a second receipt on unchanged bootstrap tree");
        assert_eq!(
            fs::read_to_string(receipts_dir.join("HEAD")).expect("HEAD2").trim(),
            fx.receipt_hash
        );
        assert_eq!(
            fs::read_to_string(receipts_dir.join("LOCK_DIGEST")).expect("LD2").trim(),
            expected_lock_digest
        );

        // Human-mode `--write-receipt` prints the receipt hash and exits 0.
        let mut h_out = Vec::new();
        let mut h_err = Vec::new();
        let h_args = vec![
            "check-stack".to_owned(),
            "--root".to_owned(),
            fx.repo.to_string_lossy().into_owned(),
            "--write-receipt".to_owned(),
        ];
        let h_code = cli::run(TEST_COMMANDS, &h_args, &mut h_out, &mut h_err);
        assert_eq!(h_code, ExitCode::from(EXIT_OK));
        let h_stdout = String::from_utf8(h_out).expect("human stdout utf8");
        assert!(
            h_stdout.contains(&fx.receipt_hash),
            "human --write-receipt output must print receipt hash `{}`; got:\n{h_stdout}",
            fx.receipt_hash
        );

        // Simulated crash leaving a stray temp file leaves HEAD & LOCK_DIGEST intact.
        fs::write(receipts_dir.join(".HEAD-crash.tmp"), "corrupt\n").expect("stray tmp");
        assert_eq!(
            fs::read_to_string(receipts_dir.join("HEAD")).expect("HEAD3").trim(),
            fx.receipt_hash
        );
        assert_eq!(
            fs::read_to_string(receipts_dir.join("LOCK_DIGEST")).expect("LD3").trim(),
            expected_lock_digest
        );
    }

    #[test]
    fn no_write_verbs_in_module() {
        let stack_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/stack");
        let forbidden = [
            "\"checkout\"",
            "\"reset\"",
            "\"stash\"",
            "\"pull\"",
            "\"fetch\"",
            "\"commit\"",
            "\"rebase\"",
            "\"clean\"",
            "\"switch\"",
            "git checkout",
            "git reset",
            "git stash",
            "git pull",
            "git fetch",
        ];
        for file in ["verify.rs", "guard.rs", "receipt.rs"] {
            let src = fs::read_to_string(stack_dir.join(file)).expect("read stack module");
            let prod = src.split("#[cfg(test)]").next().unwrap_or(&src);
            let code_lines: String = prod
                .lines()
                .filter(|line| !line.trim_start().starts_with("//"))
                .collect::<Vec<_>>()
                .join("\n");
            for pat in forbidden {
                assert!(
                    !code_lines.contains(pat),
                    "forbidden git write verb `{pat}` found in production code of `src/stack/{file}`"
                );
            }
        }
    }

    #[test]
    fn verify_is_pure() {
        // Build a diverse pool of real git/Cargo fixtures and verify 200 seeded
        // runs are strictly deterministic and mutate zero bytes of any fixture.
        let fx_clean = build_clean_fixture("pure_clean");
        let fx_tc = build_clean_fixture("pure_tc");
        fs::write(
            fx_tc.repo.join("rust-toolchain.toml"),
            "[toolchain]\nchannel = \"nightly-2026-08-25\"\n",
        )
        .expect("tc");
        let fx_dirty = build_clean_fixture("pure_dirty");
        fs::write(fx_dirty.sib.join("src/lib.rs"), "pub const V: u32 = 55;\n").expect("dirty");
        let fx_ff = build_clean_fixture("pure_ff");
        git_fixture(&fx_ff.sib, &["switch", "--detach", "-q", &fx_ff.sib_c3]);
        let fx_lock = build_clean_fixture("pure_lock");
        let ltxt = fs::read_to_string(fx_lock.repo.join("franken-stack.lock")).expect("l");
        fs::write(
            fx_lock.repo.join("franken-stack.lock"),
            ltxt.replace("\"ledger\"", "\"stats\""),
        )
        .expect("edit lock");
        let fx_missing_lf = build_clean_fixture("pure_missing_lf");
        let aside_lf = fx_missing_lf.base.join("aside");
        fs::create_dir_all(&aside_lf).expect("aside");
        fs::rename(
            fx_missing_lf.repo.join("Cargo.lock"),
            aside_lf.join("Cargo.lock"),
        )
        .expect("move Cargo.lock");
        let fx_tampered = build_clean_fixture("pure_tampered");
        let rp = fx_tampered
            .repo
            .join("receipts")
            .join(format!("{}.json", fx_tampered.receipt_hash));
        let mut raw = fs::read(&rp).expect("read receipt");
        raw.push(b' ');
        fs::write(&rp, raw).expect("tamper receipt");

        let pool = [
            (&fx_clean.base, &fx_clean.repo),
            (&fx_tc.base, &fx_tc.repo),
            (&fx_dirty.base, &fx_dirty.repo),
            (&fx_ff.base, &fx_ff.repo),
            (&fx_lock.base, &fx_lock.repo),
            (&fx_missing_lf.base, &fx_missing_lf.repo),
            (&fx_tampered.base, &fx_tampered.repo),
        ];

        let mut seed: u64 = 0x33A5_2026_1124_0001;
        for step in 0..200u32 {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            let (base, repo) = pool[(seed as usize) % pool.len()];
            let opts = if (seed & 1) == 0 {
                VerifyOptions::default()
            } else {
                VerifyOptions {
                    admitted: BTreeSet::new(),
                    duplicates_override: Some(vec!["asupersync 0.5.0".to_owned()]),
                }
            };

            let before = digest_tree(base);
            let r1 = verify_repo(repo, &opts).expect("run 1");
            let mid = digest_tree(base);
            let r2 = verify_repo(repo, &opts).expect("run 2");
            let after = digest_tree(base);

            assert_eq!(r1, r2, "verify not deterministic on step {step}");
            assert_eq!(before, mid, "verify run 1 mutated fixture on step {step}");
            assert_eq!(mid, after, "verify run 2 mutated fixture on step {step}");
        }
    }
}
