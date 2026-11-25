#![deny(unsafe_code)]

//! Tier execution engine, same-run control runner, `GateReport` gate-line
//! computation, Reality-only `NoClaim` guard, and `harness = false` CLI entry
//! point (`N38a-02`).

use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use frob_core::id::hash_blob;
use frob_core::registry::toml::{self, Value as TomlValue};
use frob_core::stack::{BUILD_IDENTITY_MODE, BUILT_UNDER};
use frob_core::testkit::{GateOutcome, GateReport, Threshold};

use crate::gates_v0::GateRegistry;
use crate::reasons::{HarnessReason, workspace_root as default_workspace_root};

use super::receipt::{
    GAUNTLET_RECEIPT_SCHEMA, GauntletReceipt, ReceiptControl, ReceiptCounts, ReceiptEntry,
    ReceiptError, ReceiptGate, ReceiptMeasured, format_text_summary, write_receipt,
};
use super::select::{SelectError, select_entries};
use super::{ControlRunOutcome, Ctx, DeviceAvailability, Milestone, Outcome, TierTable};

/// Options controlling one tier run.
#[derive(Debug, Clone)]
pub struct RunOptions {
    pub filter: Option<String>,
    pub extra_args: Vec<String>,
    pub seed: u64,
    pub milestone: Milestone,
    pub workspace_root: PathBuf,
    pub receipts_dir: PathBuf,
    pub count_override: Option<u64>,
    pub devices: DeviceAvailability,
    pub session: String,
}

impl RunOptions {
    /// Construct defaults from the process environment (`FROB_TEST_SEED`,
    /// `FROB_MILESTONE`, `FROB_REGISTRY_ROOT`/`FROB_CONFORMANCE_ROOT`,
    /// `FROB_GAUNTLET_RECEIPTS_DIR`, `E2E_DEVICES`, `FROB_SESSION_ID`/`AGENT_NAME`).
    #[must_use]
    pub fn from_env() -> Self {
        let root = default_workspace_root();
        let seed = env::var("FROB_TEST_SEED")
            .ok()
            .and_then(|s| s.trim().parse::<u64>().ok())
            .unwrap_or(0);
        let milestone = env::var("FROB_MILESTONE")
            .ok()
            .and_then(|s| Milestone::parse(&s))
            .unwrap_or_else(|| default_milestone_from_gates(&root));
        let receipts_dir = env::var_os("FROB_GAUNTLET_RECEIPTS_DIR")
            .filter(|v| !v.is_empty())
            .map(PathBuf::from)
            .unwrap_or_else(|| root.join("target/gauntlet/receipts"));
        let session = env::var("FROB_SESSION_ID")
            .ok()
            .filter(|s| !s.trim().is_empty())
            .or_else(|| env::var("AGENT_NAME").ok().filter(|s| !s.trim().is_empty()))
            .or_else(|| env::var("USER").ok().filter(|s| !s.trim().is_empty()))
            .unwrap_or_else(|| "unknown-session".to_owned());
        Self {
            filter: None,
            extra_args: Vec::new(),
            seed,
            milestone,
            workspace_root: root,
            receipts_dir,
            count_override: None,
            devices: DeviceAvailability::from_env(),
            session,
        }
    }

    /// Construct isolated options rooted at `workspace_root` and writing receipts
    /// into `receipts_dir` (handy for unit tests).
    #[must_use]
    pub fn for_scratch(workspace_root: PathBuf, receipts_dir: PathBuf) -> Self {
        Self {
            filter: None,
            extra_args: Vec::new(),
            seed: 0,
            milestone: Milestone::M0,
            workspace_root,
            receipts_dir,
            count_override: None,
            devices: DeviceAvailability::CPU_ONLY,
            session: "test-session".to_owned(),
        }
    }
}

/// Fatal run error returned by [`run_tier`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RunError {
    Selection(SelectError),
    NoClaimOnNonReality {
        entry_id: String,
        gate_id: String,
        claim: String,
    },
    Receipt(ReceiptError),
}

impl fmt::Display for RunError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Selection(e) => write!(f, "{e}"),
            Self::NoClaimOnNonReality {
                entry_id,
                gate_id,
                claim,
            } => write!(
                f,
                "reason={}: entry `{entry_id}` returned NoClaim for gate `{gate_id}` whose claim is `{claim}` (only `reality` gates admit NoClaim)",
                HarnessReason::NoClaimOnNonReality.as_str()
            ),
            Self::Receipt(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for RunError {}

/// Completed tier execution output, including the written hash-named receipt.
#[derive(Debug, Clone)]
pub struct TierRunOutput {
    pub receipt: GauntletReceipt,
    pub receipt_path: PathBuf,
    pub receipt_hash: String,
    pub text_summary: String,
    pub has_failure: bool,
}

impl TierRunOutput {
    /// Formatted final stdout line mandated by `N38a-02`: `receipt: <path>`.
    #[must_use]
    pub fn receipt_stdout_line(&self) -> String {
        format!("receipt: {}", self.receipt_path.display())
    }
}

/// Execute `table` under `opts`, write the canonical hash-named receipt, and
/// return the structured result.
pub fn run_tier(table: &TierTable, opts: &RunOptions) -> Result<TierRunOutput, RunError> {
    let selected = select_entries(table, opts).map_err(RunError::Selection)?;
    let started_ms = now_utc_ms();
    let started = format_utc_rfc3339_ms(started_ms);
    let timer = Instant::now();

    // Load registries/gates.toml (when present) to read each gate's claim kind and control list.
    let gate_reg = GateRegistry::load(&opts.workspace_root).ok();

    let mut receipt_entries: Vec<ReceiptEntry> = Vec::with_capacity(selected.entries.len());
    let mut counts = ReceiptCounts::default();
    let mut has_failure = false;

    // Per-gate collected entry outcomes for GateReport construction.
    #[derive(Default)]
    struct GateEntryAcc {
        passed: bool,
        failed: bool,
        skipped_or_obligation: bool,
        no_claim_reason: Option<String>,
        any_non_no_claim: bool,
        reduced: bool,
        measurements: Vec<(String, String, String, String, bool)>,
        declared_controls: BTreeSet<String>,
    }
    let mut per_gate: BTreeMap<String, GateEntryAcc> = BTreeMap::new();

    for entry in &selected.entries {
        let mut ctx = Ctx {
            seed: opts.seed,
            tier: table.tier(),
            site: entry.site,
            milestone: opts.milestone,
            workspace_root: opts.workspace_root.clone(),
            requested_count: selected.count_override,
            target_count: entry.target_count,
        };
        let reduced = ctx.reduced();

        let outcome = match entry.site.check_available(&opts.devices) {
            Ok(()) => (entry.run)(&mut ctx),
            Err(why) => Outcome::Skipped { why },
        };

        // Enforce Reality-only NoClaim rule before updating any gate accumulator.
        if let Outcome::NoClaim { ref reason } = outcome {
            if reason.trim().is_empty() {
                return Err(RunError::NoClaimOnNonReality {
                    entry_id: entry.id.to_owned(),
                    gate_id: entry.gates.first().copied().unwrap_or("<none>").to_owned(),
                    claim: "<empty-reason>".to_owned(),
                });
            }
            if entry.gates.is_empty() {
                return Err(RunError::NoClaimOnNonReality {
                    entry_id: entry.id.to_owned(),
                    gate_id: "<none>".to_owned(),
                    claim: "<unregistered>".to_owned(),
                });
            }
            for &gate_id in entry.gates {
                let claim = gate_reg
                    .as_ref()
                    .and_then(|r| r.get(gate_id))
                    .map_or("<unregistered>", |g| g.claim.as_str());
                if claim != "reality" {
                    return Err(RunError::NoClaimOnNonReality {
                        entry_id: entry.id.to_owned(),
                        gate_id: gate_id.to_owned(),
                        claim: claim.to_owned(),
                    });
                }
            }
        }

        let (measured_rec, first_diff, det, synthetic) = match &outcome {
            Outcome::Pass {
                measured,
                determinism,
                synthetic,
            } => {
                counts.pass += 1;
                (
                    measured.as_ref().map(|m| ReceiptMeasured {
                        name: m.name.clone(),
                        value: m.value.clone(),
                        unit: m.unit.clone(),
                        threshold: m.threshold.clone(),
                    }),
                    None,
                    *determinism,
                    *synthetic,
                )
            }
            Outcome::Fail {
                first_diff,
                measured,
                determinism,
                synthetic,
            } => {
                counts.fail += 1;
                has_failure = true;
                (
                    measured.as_ref().map(|m| ReceiptMeasured {
                        name: m.name.clone(),
                        value: m.value.clone(),
                        unit: m.unit.clone(),
                        threshold: m.threshold.clone(),
                    }),
                    Some(first_diff.clone()),
                    *determinism,
                    *synthetic,
                )
            }
            Outcome::Refused { code, detail } => {
                counts.refused += 1;
                has_failure = true;
                (None, Some(format!("{code}: {detail}")), None, false)
            }
            Outcome::Skipped { why } => {
                counts.skipped += 1;
                (None, Some(why.clone()), None, false)
            }
            Outcome::NoClaim { reason } => (None, Some(reason.clone()), None, false),
            Outcome::Obligation { reason } => {
                counts.obligations += 1;
                (None, Some(reason.clone()), None, false)
            }
        };

        for &gate_id in entry.gates {
            let acc = per_gate.entry(gate_id.to_owned()).or_default();
            if reduced.is_some() {
                acc.reduced = true;
            }
            for &cid in entry.controls {
                acc.declared_controls.insert(cid.to_owned());
            }
            match &outcome {
                Outcome::Pass { measured, .. } => {
                    acc.passed = true;
                    acc.any_non_no_claim = true;
                    if let Some(m) = measured {
                        acc.measurements.push((
                            m.name.clone(),
                            m.value.clone(),
                            m.unit.clone(),
                            m.threshold.clone(),
                            m.met,
                        ));
                        if !m.met {
                            acc.failed = true;
                            has_failure = true;
                        }
                    }
                }
                Outcome::Fail { .. } | Outcome::Refused { .. } => {
                    acc.failed = true;
                    acc.any_non_no_claim = true;
                }
                Outcome::Skipped { .. } | Outcome::Obligation { .. } => {
                    acc.skipped_or_obligation = true;
                    acc.any_non_no_claim = true;
                }
                Outcome::NoClaim { reason } => {
                    acc.no_claim_reason = Some(reason.clone());
                }
            }
        }

        receipt_entries.push(ReceiptEntry {
            id: entry.id.to_owned(),
            site: entry.site,
            outcome: outcome.as_str().to_owned(),
            measured: measured_rec,
            first_diff,
            determinism: det,
            synthetic,
            reduced,
        });
    }

    // Run registered negative controls after entries for every touched non-NoClaim gate,
    // and compute every gate line strictly through `frob_core::testkit::GateReport`.
    let mut receipt_controls: Vec<ReceiptControl> = Vec::new();
    let mut receipt_gates: Vec<ReceiptGate> = Vec::with_capacity(per_gate.len());

    for (gate_id, acc) in &per_gate {
        // Pure NoClaim on a Reality gate: no control executed, no measured value.
        if let Some(ref reason) = acc.no_claim_reason
            && !acc.any_non_no_claim
        {
            let gate_line = GateReport::new(gate_id).no_claim(reason).finish();
            let line = canonical_gate_line_str(gate_line.outcome());
            receipt_gates.push(ReceiptGate {
                id: gate_id.clone(),
                line,
                reduced: acc.reduced.then_some(true),
            });
            continue;
        }

        // Determine eligible controls for this gate:
        // If `gate_id` is in `registries/gates.toml`, only controls listed in
        // `gate_row.controls` are eligible (so emptying `controls = []` in a
        // scratch registry makes the gate `not-eligible`).
        let registry_allowed_controls: Option<BTreeSet<&str>> = gate_reg
            .as_ref()
            .and_then(|reg| reg.get(gate_id))
            .map(|row| row.controls.iter().map(String::as_str).collect());

        let mut gate_ctrl_results: Vec<(String, ControlRunOutcome)> = Vec::new();
        for ctrl in table.controls() {
            if ctrl.gate != gate_id {
                continue;
            }
            if let Some(ref allowed) = registry_allowed_controls
                && !allowed.contains(ctrl.id)
            {
                continue;
            }
            let mut ctrl_ctx = Ctx {
                seed: opts.seed,
                tier: table.tier(),
                site: super::Site::Host,
                milestone: opts.milestone,
                workspace_root: opts.workspace_root.clone(),
                requested_count: selected.count_override,
                target_count: None,
            };
            let ctrl_outcome = (ctrl.run)(&mut ctrl_ctx);
            if ctrl_outcome == ControlRunOutcome::Missed {
                has_failure = true;
            }
            receipt_controls.push(ReceiptControl {
                id: ctrl.id.to_owned(),
                gate: gate_id.clone(),
                outcome: ctrl_outcome,
            });
            gate_ctrl_results.push((ctrl.id.to_owned(), ctrl_outcome));
        }

        // Build the sealed `GateLine` strictly through `frob_core::testkit::GateReport`.
        let mut report = GateReport::new(gate_id);
        if acc.failed {
            report = report.measured("entry_status", "0", "pass", Threshold::eq("1"));
        } else if acc.passed {
            report = report.measured("entry_status", "1", "pass", Threshold::eq("1"));
        }

        if acc.skipped_or_obligation && !acc.failed {
            // A skipped site or obligation is never eligible for `pass`. Feed only
            // missed/stale controls (so a missed control still yields `fail`), and
            // withhold caught controls so `GateReport` emits `not-eligible`.
            for (cid, co) in &gate_ctrl_results {
                if matches!(co, ControlRunOutcome::Missed | ControlRunOutcome::Stale)
                    && let Some(go) = co.to_gate_report_outcome()
                {
                    report = report.control(cid, go);
                }
            }
        } else {
            for (cid, co) in &gate_ctrl_results {
                if let Some(go) = co.to_gate_report_outcome() {
                    report = report.control(cid, go);
                }
            }
        }

        let gate_line = report.finish();
        let line = canonical_gate_line_str(gate_line.outcome());
        receipt_gates.push(ReceiptGate {
            id: gate_id.clone(),
            line,
            reduced: acc.reduced.then_some(true),
        });
    }

    let wall_ms = u64::try_from(timer.elapsed().as_millis()).unwrap_or(u64::MAX);
    let (commit, dirty) = git_commit_and_dirty_ro(&opts.workspace_root);
    let toolchain = read_toolchain_channel(&opts.workspace_root);
    let fingerprint = host_fingerprint_hex();
    let profile = if cfg!(debug_assertions) {
        "debug".to_owned()
    } else {
        "release".to_owned()
    };

    let receipt = GauntletReceipt {
        schema: GAUNTLET_RECEIPT_SCHEMA.to_owned(),
        tier: table.tier(),
        filter: selected.filter_label,
        commit,
        dirty,
        toolchain,
        fingerprint,
        seed: opts.seed,
        profile,
        started,
        wall_ms,
        entries: receipt_entries,
        controls: receipt_controls,
        gates: receipt_gates,
        counts,
        session: opts.session.clone(),
        stack_receipt: BUILT_UNDER.receipt.to_string(),
        stack_dev: BUILT_UNDER.dev,
        build_identity: BUILD_IDENTITY_MODE.to_owned(),
    };

    let (receipt_path, receipt_hash) =
        write_receipt(&opts.receipts_dir, &receipt).map_err(RunError::Receipt)?;
    let text_summary = format_text_summary(&receipt);

    Ok(TierRunOutput {
        receipt,
        receipt_path,
        receipt_hash,
        text_summary,
        has_failure,
    })
}

/// Entry point for `harness = false` tier test binaries (`tests/g0_laws.rs`..
/// `tests/g5_determinism.rs`).
///
/// Supports:
/// - `FROB_TEST_SEED=<seed> cargo test -p frob-conformance --test <tier> -- <filter>`
/// - `--count <N>` (reduced-count execution)
/// - `--receipts-dir <dir>` (override receipt output directory)
/// - `--list` (list registered entry IDs and exit 0)
/// - Ignores benign Cargo/libtest flags (`--nocapture`, `--show-output`, `--exact`, `--quiet`)
///
/// Always prints `receipt: <path>` as the exact last line of stdout on a completed run,
/// and exits `0` on clean run or `101` on failure/error (never exit `1`, per `N38-REF`).
pub fn tier_main(table: &TierTable) -> ExitCode {
    let raw_args: Vec<String> = env::args().skip(1).collect();
    let mut opts = RunOptions::from_env();
    let mut list_only = false;

    let mut i = 0usize;
    while i < raw_args.len() {
        let arg = &raw_args[i];
        match arg.as_str() {
            "--list" => {
                list_only = true;
                i += 1;
            }
            "--nocapture" | "--show-output" | "--exact" | "--quiet" | "--ignored"
            | "--include-ignored" => {
                i += 1;
            }
            "--format" | "--color" | "--test-threads" | "--logfile" => {
                i += 2;
            }
            a if a.starts_with("--format=")
                || a.starts_with("--color=")
                || a.starts_with("--test-threads=") =>
            {
                i += 1;
            }
            "--count" => {
                let Some(val) = raw_args.get(i + 1) else {
                    eprintln!("tier_main: `--count` requires a positive integer");
                    return ExitCode::from(101);
                };
                match val.parse::<u64>() {
                    Ok(n) if n > 0 => opts.count_override = Some(n),
                    _ => {
                        eprintln!("tier_main: invalid `--count` value `{val}`");
                        return ExitCode::from(101);
                    }
                }
                i += 2;
            }
            a if a.starts_with("--count=") => {
                let val = &a["--count=".len()..];
                match val.parse::<u64>() {
                    Ok(n) if n > 0 => opts.count_override = Some(n),
                    _ => {
                        eprintln!("tier_main: invalid `--count` value `{val}`");
                        return ExitCode::from(101);
                    }
                }
                i += 1;
            }
            "--receipts-dir" => {
                let Some(val) = raw_args.get(i + 1) else {
                    eprintln!("tier_main: `--receipts-dir` requires a path");
                    return ExitCode::from(101);
                };
                opts.receipts_dir = PathBuf::from(val);
                i += 2;
            }
            a if a.starts_with("--receipts-dir=") => {
                opts.receipts_dir = PathBuf::from(&a["--receipts-dir=".len()..]);
                i += 1;
            }
            _ => {
                if opts.filter.is_none() && !arg.starts_with('-') {
                    opts.filter = Some(arg.clone());
                } else {
                    opts.extra_args.push(arg.clone());
                }
                i += 1;
            }
        }
    }

    if list_only {
        for e in table.entries() {
            println!("{}: test", e.id);
        }
        return ExitCode::SUCCESS;
    }

    match run_tier(table, &opts) {
        Ok(out) => {
            print!("{}", out.text_summary);
            // Display relative path when under workspace_root so scripts see
            // `receipt: target/gauntlet/receipts/<hash>.json`, or full path when outside.
            let display_path = out
                .receipt_path
                .strip_prefix(&opts.workspace_root)
                .unwrap_or(&out.receipt_path);
            println!("receipt: {}", display_path.display());
            if out.has_failure {
                ExitCode::from(101)
            } else {
                ExitCode::SUCCESS
            }
        }
        Err(err) => {
            eprintln!("tier {}: {err}", table.tier().target_name());
            ExitCode::from(101)
        }
    }
}

fn canonical_gate_line_str(outcome: &GateOutcome) -> String {
    match outcome {
        GateOutcome::Pass => "pass".to_owned(),
        GateOutcome::Fail => "fail".to_owned(),
        GateOutcome::NotEligible(_) => HarnessReason::NotEligible.as_str().to_owned(),
        GateOutcome::NoClaim(reason) => format!("no-claim: {reason}"),
    }
}

fn default_milestone_from_gates(root: &Path) -> Milestone {
    let Ok(reg) = GateRegistry::load(root) else {
        return Milestone::M0;
    };
    reg.all()
        .iter()
        .filter_map(|g| Milestone::parse(&g.milestone))
        .min()
        .unwrap_or(Milestone::M0)
}

fn git_commit_and_dirty_ro(root: &Path) -> (String, bool) {
    let rev = Command::new("git")
        .arg("--no-optional-locks")
        .arg("-C")
        .arg(root)
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_owned())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "0000000000000000000000000000000000000000".to_owned());

    let dirty = Command::new("git")
        .arg("--no-optional-locks")
        .arg("-C")
        .arg(root)
        .args(["status", "--porcelain", "--untracked-files=no"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .is_some_and(|o| !o.stdout.iter().all(u8::is_ascii_whitespace));

    (rev, dirty)
}

fn read_toolchain_channel(root: &Path) -> String {
    let path = root.join("rust-toolchain.toml");
    let Ok(bytes) = fs::read(&path) else {
        return "unknown".to_owned();
    };
    let Ok(tbl) = toml::parse(&bytes) else {
        return "unknown".to_owned();
    };
    tbl.get("toolchain")
        .and_then(TomlValue::as_table)
        .and_then(|t| t.get("channel"))
        .and_then(TomlValue::as_str)
        .filter(|s| !s.is_empty())
        .unwrap_or("unknown")
        .to_owned()
}

fn host_fingerprint_hex() -> String {
    let cpus = std::thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get);
    let payload = format!(
        "frob.host.fp/1|os={}|arch={}|cpus={}",
        env::consts::OS,
        env::consts::ARCH,
        cpus
    );
    hash_blob(payload.as_bytes()).to_hex()
}

fn now_utc_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| u64::try_from(d.as_millis()).unwrap_or(u64::MAX))
}

fn format_utc_rfc3339_ms(ms: u64) -> String {
    let secs = ms / 1000;
    let millis = ms % 1000;
    let days = secs / 86_400;
    let sod = secs % 86_400;
    let h = sod / 3600;
    let m = (sod % 3600) / 60;
    let s = sod % 60;
    let (y, mo, d) = civil_from_days(days as i64);
    format!("{y:04}-{mo:02}-{d:02}T{h:02}:{m:02}:{s:02}.{millis:03}Z")
}

/// Howard Hinnant's civil-from-days algorithm (UTC Gregorian calendar).
const fn civil_from_days(days_since_epoch: i64) -> (i64, u32, u32) {
    let z = days_since_epoch + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = (z - era * 146_097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = (yoe as i64) + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = if m <= 2 { y + 1 } else { y };
    (year, m as u32, d as u32)
}
