//! Integration tests for `BD-004`: the six `cargo xtask` checks assembled behind
//! one closed command table, finding-identifier ownership disjointness, the 7 x 6
//! GATE-WS-01 plant matrix on a miniature synthetic workspace (plus harness negative
//! controls and D0 determinism), Plant 5 on a scratch copy of the real tree, and
//! GATE-WS-04 in-process warm-tree timing instrumentation.
//!
//! Scratch copies live under `${BIG_SCRATCH:-${TMPDIR:-/tmp}}/frob-BlackOx/` and are
//! never deleted (COMMON.md 9b). Zero `unsafe`, zero network.

use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use frob_core::json::{Limits, admit};
use xtask::assembly::{
    CHECK_OWNERS, COMMANDS, CliJsonRun, CommandTableError, EXPECTED_SUBCOMMANDS, GATE_WS_01_PLANTS,
    GATE_WS_04_LAYERS_AND_DEPS_BUDGET_MS, GATE_WS_04_LAYERS_AND_DEPS_SUBSHARES, GATE_WS_04_SHARES,
    GATE_WS_04_TOTAL_BUDGET_MS, OwnershipError, evaluate_gate_ws_04_timing,
    normalize_report_json_for_determinism, run_check_cli_json, verify_command_table,
    verify_finding_id_ownership,
};
use xtask::check_contracts::{REQUIRED_CONTRACT_SECTIONS, RULE_CONTRACT_SECTION_MISSING};
use xtask::check_deps::RULE_ASUP_DUPLICATE;
use xtask::check_layers::{RULE_L1_RANK, RULE_L1_UPWARD};
use xtask::check_unsafe::RULE_CAPSULE_UNREGISTERED;
use xtask::cli::{
    self, CheckContext, CheckError, CheckFn, CheckOutcome, EXIT_FINDINGS, EXIT_OK, EXIT_USAGE,
};
use xtask::generate::RULE_DIFFERS;
use xtask::generate::admission_doc;
use xtask::metadata::normalize_rel_path;
use xtask::report::validate_report_json;
use xtask::stack::admission::{
    ADMISSION_SCHEMA, AdmissionCovers, AdmissionRecord, AdmissionStatus, ControlResult, HostInfo,
    RecordedBy,
};
use xtask::stack::lock::{self as stack_lock, Blake3Hex, GitRev};
use xtask::stack::verify::{self as stack_verify, RULE_LOCKFILE, RULE_PIN, VerifyOptions};

static COUNTER: AtomicU64 = AtomicU64::new(0);

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crates/xtask sits two levels below repo root")
        .to_path_buf()
}

fn make_scratch_dir(label: &str) -> PathBuf {
    let base = env::var_os("BIG_SCRATCH")
        .or_else(|| env::var_os("TMPDIR"))
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("/tmp"));
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_secs());
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    let pid = std::process::id();
    let dir = base
        .join("frob-BlackOx")
        .join(format!("xtask-bd004-{label}-{ts}-{pid}-{n}"));
    fs::create_dir_all(&dir).expect("create scratch dir");
    dir
}

fn write_file(path: &Path, content: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("create parent dirs");
    }
    fs::write(path, content).expect("write synthetic fixture file");
}

fn copy_file(src: &Path, dst: &Path) {
    if let Some(parent) = dst.parent() {
        fs::create_dir_all(parent).expect("create parent dirs");
    }
    fs::copy(src, dst).expect("copy fixture file");
}

fn copy_dir_recursive_skipping(src: &Path, dst: &Path, skip_names: &[&str]) {
    if !src.is_dir() {
        return;
    }
    fs::create_dir_all(dst).expect("mkdir dst");
    let mut entries: Vec<PathBuf> = fs::read_dir(src)
        .expect("read_dir")
        .flatten()
        .map(|e| e.path())
        .collect();
    entries.sort();
    for p in entries {
        let Some(name_os) = p.file_name() else {
            continue;
        };
        let name = name_os.to_string_lossy();
        if skip_names.iter().any(|s| *s == name.as_ref()) {
            continue;
        }
        let target = dst.join(name_os);
        if p.is_symlink() {
            continue;
        }
        if p.is_dir() {
            copy_dir_recursive_skipping(&p, &target, skip_names);
        } else if p.is_file() {
            fs::copy(&p, &target).expect("copy file");
        }
    }
}

fn run_git(dir: &Path, args: &[&str]) -> String {
    let out = Command::new("git")
        .current_dir(dir)
        .args(args)
        .output()
        .expect("invoke git");
    assert!(
        out.status.success(),
        "git {:?} failed in {}:\nstdout={}\nstderr={}",
        args,
        dir.display(),
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8_lossy(&out.stdout).trim().to_owned()
}

fn git_init_repo(dir: &Path) -> String {
    run_git(dir, &["init", "-q"]);
    run_git(dir, &["config", "user.email", "blackox@example.invalid"]);
    run_git(dir, &["config", "user.name", "BlackOx"]);
    run_git(dir, &["add", "."]);
    run_git(dir, &["commit", "-q", "-m", "initial commit"]);
    run_git(dir, &["rev-parse", "HEAD"])
}

fn git_commit_all(dir: &Path, msg: &str) -> String {
    run_git(dir, &["add", "."]);
    run_git(dir, &["commit", "-q", "-m", msg]);
    run_git(dir, &["rev-parse", "HEAD"])
}

fn generate_lockfile_offline(ws_dir: &Path) {
    let out = Command::new("cargo")
        .current_dir(ws_dir)
        .args(["generate-lockfile", "--offline", "--manifest-path"])
        .arg(ws_dir.join("Cargo.toml"))
        .output()
        .expect("invoke cargo generate-lockfile --offline");
    assert!(
        out.status.success(),
        "cargo generate-lockfile --offline failed in {}:\n{}",
        ws_dir.display(),
        String::from_utf8_lossy(&out.stderr)
    );
}

fn refresh_miniature_stack_receipt(repo: &Path) {
    let result = stack_verify::run_write_receipt(repo, &VerifyOptions::default())
        .expect("write miniature stack receipt");
    assert!(
        result.findings.is_empty(),
        "miniature stack receipt findings: {:?}",
        result.findings
    );
}

/// Give new fixture foundations admission records tied to a clean, committed dependency graph.
fn seed_miniature_admissions(repo: &Path, repo_rev: &str) {
    refresh_miniature_stack_receipt(repo);
    let head = fs::read_to_string(repo.join("receipts/HEAD")).expect("read miniature receipt hash");
    let receipt_hash = head.trim();
    let lock_path = repo.join("franken-stack.lock");
    let mut lock = stack_lock::load(&lock_path).expect("load miniature stack lock");
    let output = "Synthetic BD-004 miniature admission fixture\n";
    let output_hash = fs_blake3::hash_bytes(output.as_bytes()).to_hex();
    write_file(
        &repo.join(format!("receipts/outputs/{output_hash}.txt")),
        output,
    );

    for row in lock
        .foundations
        .iter_mut()
        .filter(|row| row.receipt.is_none())
    {
        let record = AdmissionRecord {
            schema: ADMISSION_SCHEMA.to_owned(),
            gate: row.admission_gate.as_str().to_owned(),
            foundation: row.id.as_str().to_owned(),
            covers: AdmissionCovers {
                crates: vec![row.name.clone()],
                groups: Vec::new(),
                tool: None,
            },
            repo_rev: repo_rev.to_owned(),
            workspace: "core".to_owned(),
            stack_receipt: format!("sr:{receipt_hash}"),
            sibling_rev: match &row.git_rev {
                Some(GitRev::Commit(rev)) => Some(rev.clone()),
                _ => None,
            },
            toolchain: lock.toolchain.as_str().to_owned(),
            harness_receipt: None,
            command: vec!["bd004-miniature-fixture".to_owned()],
            exit: 0,
            output_hash: output_hash.clone(),
            output_excerpt: output.to_owned(),
            measured: Vec::new(),
            controls: vec![ControlResult {
                id: "NC-BD004-FIXTURE".to_owned(),
                caught: true,
            }],
            facts_reread: Vec::new(),
            status: AdmissionStatus::Pass,
            host: HostInfo {
                class: "trj".to_owned(),
                fingerprint: None,
            },
            recorded: RecordedBy {
                date: "2026-11-24T00:00:00Z".to_owned(),
                session: "miniature-fixture-reviewer".to_owned(),
                implemented_by: "miniature-fixture-author".to_owned(),
            },
        };
        let bytes = record
            .to_canonical_bytes()
            .expect("serialize fixture admission");
        let hash = fs_blake3::hash_bytes(bytes.as_bytes()).to_hex();
        write_file(
            &repo.join(format!("receipts/admissions/{}/{hash}.json", record.gate)),
            bytes.as_str(),
        );
        row.receipt = Some(
            Blake3Hex::parse(receipt_hash, row.id.as_str(), "receipt")
                .expect("parse miniature receipt hash"),
        );
    }

    fs::write(lock_path, lock.canonical_bytes()).expect("write admitted miniature stack lock");
    admission_doc::check_or_bless(repo, true).expect("regenerate miniature admission document");
    git_commit_all(repo, "record miniature fixture admissions");
    refresh_miniature_stack_receipt(repo);
}

fn write_valid_contract(repo: &Path, rel_dir: &str, name: &str, layer_toml: &str, rank: u8) {
    let dir = repo.join(rel_dir);
    let mut md = format!("# Contract: {name}\n\n");
    for sec in REQUIRED_CONTRACT_SECTIONS {
        md.push_str(&format!(
            "## {sec}\n\nSynthetic contract section `{sec}` for `{name}`.\n\n"
        ));
    }
    write_file(&dir.join("CONTRACT.md"), &md);
    let toml = format!(
        "[crate]\n\
         name = \"{name}\"\n\
         layer = {layer_toml}\n\
         rank = {rank}\n\
         plane = \"compute\"\n\
         determinism = \"D0\"\n\
         evidence = \"Tested\"\n\
         gates = []\n\
         refusals = []\n\n\
         [budgets]\n\
         wall_ms = 50\n\
         memory_mib = 64\n\n\
         [safe_mode]\n\
         on_budget = \"reference-path\"\n\
         on_calibration = \"reference-path\"\n"
    );
    write_file(&dir.join("contract.toml"), &toml);
    write_file(
        &dir.join("tests/conformance.rs"),
        "#[test]\nfn synthetic_conformance() {}\n",
    );
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ParsedFinding {
    rule: String,
    subject: String,
    path: String,
    line: Option<i64>,
}

fn parse_report_findings_and_elapsed(run: &CliJsonRun) -> (bool, u64, Vec<ParsedFinding>) {
    validate_report_json(run.stdout.as_bytes()).unwrap_or_else(|err| {
        panic!(
            "check `{}` did not emit valid frob.xtask.report/1 JSON (exit={:?}, stderr={:?}, stdout={:?}): {err}",
            run.check, run.exit_code, run.stderr, run.stdout
        )
    });
    let admitted = admit(run.stdout.as_bytes(), &Limits::default()).unwrap_or_else(|err| {
        panic!(
            "check `{}` report JSON failed admission (exit={:?}, stderr={:?}): {err}",
            run.check, run.exit_code, run.stderr
        )
    });
    let root = admitted.value();
    let ok = root.get("ok").and_then(|v| v.as_bool()).expect("ok bool");
    let elapsed_i64 = root
        .get("elapsed_ms")
        .and_then(|v| v.as_int53().ok())
        .expect("elapsed_ms int53");
    let elapsed_ms = u64::try_from(elapsed_i64).expect("non-negative elapsed_ms");
    let arr = root
        .get("findings")
        .and_then(|v| v.as_array())
        .expect("findings array");
    let mut findings = Vec::with_capacity(arr.len());
    for item in arr {
        let rule = item
            .get("rule")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_owned();
        let subject = item
            .get("subject")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_owned();
        let path = item
            .get("path")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_owned();
        let line = item.get("line").and_then(|v| v.as_int53().ok());
        findings.push(ParsedFinding {
            rule,
            subject,
            path,
            line,
        });
    }
    (ok, elapsed_ms, findings)
}

/// Miniature synthetic workspace containing three core crates (`frob-core`, `frob-lie`,
/// `frob-algebra`), one annex (`spark/frob-spark-cuda`), one registry of each kind,
/// a two-commit sibling repository (`../sib-demo`), and a clean stack receipt.
struct MiniatureEnv {
    repo: PathBuf,
    sib_rev1: String,
    sib_rev2: String,
    dummy_sib_rev: String,
}

impl MiniatureEnv {
    fn build(label: &str) -> Self {
        let base = make_scratch_dir(label);
        let repo = base.join("repo");
        let reg = base.join("local-reg");
        fs::create_dir_all(&repo).expect("mkdir repo");
        fs::create_dir_all(&reg).expect("mkdir local-reg");

        // Offline directory registry holding `asupersync 0.6.2`.
        let asup_dir = reg.join("asupersync-0.6.2");
        write_file(
            &asup_dir.join("Cargo.toml"),
            "[package]\nname = \"asupersync\"\nversion = \"0.6.2\"\nedition = \"2021\"\n",
        );
        write_file(
            &asup_dir.join("src/lib.rs"),
            "// synthetic asupersync 0.6.2\n",
        );
        write_file(&asup_dir.join(".cargo-checksum.json"), "{\"files\":{}}\n");

        // Offline `.cargo/config.toml` at `base` inherited by `repo` and annexes.
        let reg_posix = normalize_rel_path(&reg);
        write_file(
            &base.join(".cargo/config.toml"),
            &format!(
                "[source.crates-io]\nreplace-with = \"local-reg\"\n\n\
                 [source.local-reg]\ndirectory = \"{reg_posix}\"\n"
            ),
        );

        // Two-commit sibling repository `../sib-demo` (for Plant 7 stale lock check).
        let sib_demo = base.join("sib-demo");
        write_file(
            &sib_demo.join("Cargo.toml"),
            "[package]\nname = \"sib-demo\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
        );
        write_file(&sib_demo.join("src/lib.rs"), "// sib-demo rev 1\n");
        let sib_rev1 = git_init_repo(&sib_demo);
        write_file(&sib_demo.join("src/lib.rs"), "// sib-demo rev 2\n");
        let sib_rev2 = git_commit_all(&sib_demo, "advance sib-demo to rev2");

        // Plant 3 enables the sibling's optional second asupersync after admitting the sibling.
        let asup_alt = base.join("asup-alt");
        write_file(
            &asup_alt.join("Cargo.toml"),
            "[package]\nname = \"asupersync\"\nversion = \"0.7.0\"\nedition = \"2021\"\n",
        );
        write_file(
            &asup_alt.join("src/lib.rs"),
            "// synthetic asupersync 0.7.0\n",
        );
        let dummy_sib = base.join("dummy-sib");
        write_file(
            &dummy_sib.join("Cargo.toml"),
            "[package]\nname = \"dummy-sib\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n\
             [features]\nalternate-asup = [\"dep:asupersync\"]\n\n\
             [dependencies]\nasupersync = { path = \"../asup-alt\", optional = true }\n",
        );
        write_file(
            &dummy_sib.join("src/lib.rs"),
            "// dummy sibling pulling second asupersync\n",
        );
        let dummy_sib_rev = git_init_repo(&dummy_sib);

        // Root core workspace manifest with the inherited Rust lints.
        write_file(
            &repo.join("Cargo.toml"),
            "[workspace]\n\
             resolver = \"3\"\n\
             members = [\"crates/frob-core\", \"crates/frob-lie\", \"crates/frob-algebra\"]\n\
             exclude = [\"spark/frob-spark-cuda\"]\n\n\
             [workspace.lints.rust]\n\
             unsafe_code = \"deny\"\n\
             deprecated = \"deny\"\n",
        );
        write_file(
            &repo.join("rust-toolchain.toml"),
            "[toolchain]\nchannel = \"nightly-2026-08-31\"\n",
        );

        // Core crate 1: frob-core (L0, R1), links asupersync 0.6.2.
        write_file(
            &repo.join("crates/frob-core/Cargo.toml"),
            "[package]\nname = \"frob-core\"\nversion = \"0.0.1\"\nedition = \"2024\"\n\n\
             [package.metadata.frob]\nlayer = 0\nrank = 1\n\n\
             [dependencies]\nasupersync = { version = \"=0.6.2\", default-features = false }\n\n\
             [lints]\nworkspace = true\n",
        );
        write_file(
            &repo.join("crates/frob-core/src/lib.rs"),
            "pub fn core_ready() -> bool { true }\n",
        );
        write_valid_contract(&repo, "crates/frob-core", "frob-core", "0", 1);

        // Core crate 2: frob-lie (L1, R1).
        write_file(
            &repo.join("crates/frob-lie/Cargo.toml"),
            "[package]\nname = \"frob-lie\"\nversion = \"0.0.1\"\nedition = \"2024\"\n\n\
             [package.metadata.frob]\nlayer = 1\nrank = 1\n\n\
             [lints]\nworkspace = true\n",
        );
        write_file(
            &repo.join("crates/frob-lie/src/lib.rs"),
            "pub fn lie_ready() -> bool { true }\n",
        );
        write_valid_contract(&repo, "crates/frob-lie", "frob-lie", "1", 1);

        // Core crate 3: frob-algebra (L1, R2) with binary `frob`, depends on frob-core + sib-demo.
        write_file(
            &repo.join("crates/frob-algebra/Cargo.toml"),
            "[package]\nname = \"frob-algebra\"\nversion = \"0.0.1\"\nedition = \"2024\"\n\n\
             [package.metadata.frob]\nlayer = 1\nrank = 2\n\n\
             [[bin]]\nname = \"frob\"\npath = \"src/main.rs\"\n\n\
             [dependencies]\n\
             frob-core = { path = \"../frob-core\" }\n\
             sib-demo = { path = \"../../../sib-demo\", default-features = false }\n\n\
             [lints]\nworkspace = true\n",
        );
        write_file(
            &repo.join("crates/frob-algebra/src/lib.rs"),
            "pub fn alg_ready() -> bool { true }\n",
        );
        write_file(
            &repo.join("crates/frob-algebra/src/main.rs"),
            "fn main() {}\n",
        );
        write_valid_contract(&repo, "crates/frob-algebra", "frob-algebra", "1", 2);

        // Annex crate: spark/frob-spark-cuda (own workspace + Cargo.lock, zero unsafe initially).
        write_file(
            &repo.join("spark/frob-spark-cuda/Cargo.toml"),
            "[workspace]\nresolver = \"3\"\nmembers = [\".\"]\n\n\
             [package]\nname = \"frob-spark-cuda\"\nversion = \"0.0.1\"\nedition = \"2024\"\n\n\
             [package.metadata.frob]\nlayer = \"annex\"\nrank = 1\n",
        );
        write_file(
            &repo.join("spark/frob-spark-cuda/src/lib.rs"),
            "pub fn cuda_annex_stub() -> u32 { 0 }\n",
        );
        write_valid_contract(
            &repo,
            "spark/frob-spark-cuda",
            "frob-spark-cuda",
            "\"annex\"",
            1,
        );

        // One registry of each kind + schema needed by check-contracts and check-generated.
        let real = repo_root();
        copy_file(
            &real.join("schemas/contract.schema.json"),
            &repo.join("schemas/contract.schema.json"),
        );
        write_file(
            &repo.join("registries/formats.toml"),
            "schema = \"frob.formats/1\"\n\n\
             [[format]]\n\
             id = \"FMT-FBOT\"\n\
             extension = \".fbot\"\n\
             schema = \"frob.design/1\"\n\
             owner = \"frob-design\"\n\
             producers = [\"frob-design\"]\n\
             consumers = [\"frob-design\"]\n\
             encoding = \"canonical\"\n",
        );
        write_file(
            &repo.join("registries/verbs.toml"),
            "schema = \"frob.verbs/1\"\n",
        );
        write_file(
            &repo.join("registries/refusals.toml"),
            "schema = \"frob.refusals/1\"\n\n\
             [exit_rule]\n\
             \"1xx\" = 3\n\
             \"2xx\" = 3\n\
             \"3xx\" = 3\n\
             \"4xx\" = 3\n\
             \"5xx\" = 3\n\
             \"6xx\" = 6\n\
             \"70x\" = 4\n\
             \"72x\" = 5\n\
             \"73x\" = 5\n\
             \"79x\" = 5\n\
             \"8xx\" = 3\n\
             \"9xx\" = 9\n\
             conflict = []\n\
             integrity = []\n\
             capability = []\n\n\
             [[refusal]]\n\
             code = \"FROB-REF-802\"\n\
             meaning = \"Synthetic refusal row for BD-004 miniature workspace\"\n\
             defined_in = \"§32.18\"\n\
             exit = 3\n\
             retry = \"after-change\"\n\
             repairs = [\"Supply arguments matching the schema\"]\n\
             raised_by = [\"BD-004\"]\n",
        );
        write_file(
            &repo.join("registries/gates.toml"),
            "schema = \"frob.gates/1\"\n\n\
             [[gate]]\n\
             id = \"GATE-SYN-01\"\n\
             claim = \"known-answer\"\n\
             claim_id = \"KA-784\"\n\
             section = \"34.13\"\n\
             threshold = \"7 of 7 plants caught\"\n\
             milestone = \"M0\"\n\
             tier = [\"G0:host\"]\n\
             controls = [\"NC-0001\"]\n\n\
             [[gate.part]]\n\
             milestone = \"M0\"\n\
             status = \"not-run\"\n\
             measured = \"\"\n\
             evidence = []\n\
             recorded = { date = \"\", session = \"\", implemented_by = \"\" }\n",
        );
        write_file(
            &repo.join("registries/kernels.toml"),
            "schema = \"frob.kernels/1\"\n",
        );
        write_file(
            &repo.join("registries/capsules.toml"),
            "schema = \"frob.capsules/1\"\n",
        );
        write_file(
            &repo.join("registries/lifts.toml"),
            "schema = \"frob.lifts/1\"\n",
        );
        write_file(
            &repo.join("registries/qty_aliases.toml"),
            "schema = \"frob.qty_aliases/1\"\n\n\
             [[alias]]\n\
             name = \"Length\"\n\
             dimension = [1, 0, 0, 0, 0, 0]\n\
             kind = \"length\"\n\
             unit = \"m\"\n\n\
             [[alias]]\n\
             name = \"Area\"\n\
             dimension = [2, 0, 0, 0, 0, 0]\n\
             kind = \"area\"\n\
             unit = \"m^2\"\n",
        );
        write_file(
            &repo.join("registries/qty_products.toml"),
            "schema = \"frob.qty_products/1\"\n\n\
             [[product]]\n\
             lhs = \"Length\"\n\
             op = \"mul\"\n\
             rhs = \"Length\"\n\
             out = \"Area\"\n",
        );
        write_file(
            &repo.join("README.md"),
            "# Synthetic Miniature Workspace\n\n<!-- status:begin -->\n<!-- status:end -->\n",
        );
        write_file(
            &repo.join(".gitignore"),
            "/receipts/HEAD\n/receipts/LOCK_DIGEST\n",
        );

        // Stack lock pinning `FD-ASUP` (=0.6.2) and `FD-SIBDEMO` (sib_rev2).
        let lock_toml = format!(
            "# synthetic franken-stack.lock for BD-004 miniature workspace\n\
             toolchain = \"nightly-2026-08-31\"\n\n\
             [[foundation]]\n\
             id = \"FD-ASUP\"\n\
             name = \"asupersync\"\n\
             posture = \"consume\"\n\
             version = \"=0.6.2\"\n\
             confined_to = [\"any\"]\n\
             admission_gate = \"INT-ASUP-01\"\n\
             fallback = \"synthetic fallback\"\n\n\
             [[foundation]]\n\
             id = \"FD-SIBDEMO\"\n\
             name = \"sib-demo\"\n\
             posture = \"consume-path\"\n\
             path = \"../sib-demo\"\n\
             git_rev = \"{sib_rev2}\"\n\
             toolchain = \"nightly-2026-08-31\"\n\
             confined_to = [\"frob-algebra\"]\n\
             admission_gate = \"INT-SIBDEMO-01\"\n\
             fallback = \"synthetic fallback\"\n"
        );
        write_file(&repo.join("franken-stack.lock"), &lock_toml);

        // Materialize all six built-in generated outputs cleanly via `check-generated --write`.
        let gen_write = run_check_cli_json(&repo, "check-generated", &["--write"]);
        assert_eq!(
            gen_write.exit_code,
            ExitCode::from(EXIT_OK),
            "seed check-generated --write failed:\nstdout={}\nstderr={}",
            gen_write.stdout,
            gen_write.stderr
        );

        // Generate lockfiles offline for both core and annex workspaces.
        generate_lockfile_offline(&repo);
        generate_lockfile_offline(&repo.join("spark/frob-spark-cuda"));

        let repo_rev = git_init_repo(&repo);
        seed_miniature_admissions(&repo, &repo_rev);

        Self {
            repo,
            sib_rev1,
            sib_rev2,
            dummy_sib_rev,
        }
    }

    /// Normalize a manifest-changing plant as a legitimate change would: regenerate
    /// `Cargo.lock` offline and refresh `receipts/HEAD` so only the planted defect remains.
    fn normalize_manifest_change(&self) {
        generate_lockfile_offline(&self.repo);
        refresh_miniature_stack_receipt(&self.repo);
    }

    /// Run all six `xtask` checks in process via `cli::run(COMMANDS, ...)` and return
    /// `(exit_code, parsed_findings, raw_stdout)` per check.
    fn run_all_six(&self) -> BTreeMap<String, (ExitCode, Vec<ParsedFinding>, String)> {
        let mut results = BTreeMap::new();
        for &check in &EXPECTED_SUBCOMMANDS {
            let run = run_check_cli_json(&self.repo, check, &[]);
            assert!(
                run.exit_code == ExitCode::from(EXIT_OK)
                    || run.exit_code == ExitCode::from(EXIT_FINDINGS),
                "check `{check}` returned unexpected exit {:?} (stderr={}):\nstdout={}",
                run.exit_code,
                run.stderr,
                run.stdout
            );
            assert!(
                run.stderr.is_empty(),
                "check `{check}` wrote unexpected stderr: {}",
                run.stderr
            );
            let (ok, _elapsed_ms, findings) = parse_report_findings_and_elapsed(&run);
            assert_eq!(
                ok,
                run.exit_code == ExitCode::from(EXIT_OK),
                "report ok field must agree with exit code for `{check}`"
            );
            results.insert(check.to_owned(), (run.exit_code, findings, run.stdout));
        }
        results
    }
}

fn dummy_seventh_handler(_ctx: &CheckContext) -> Result<CheckOutcome, CheckError> {
    Ok(CheckOutcome {
        findings: Vec::new(),
        facts: BTreeMap::new(),
    })
}

// ============================================================================
// 1. Closed six-subcommand guard (INV-WS-05) & usage errors
// ============================================================================

#[test]
fn exactly_six_subcommands() {
    // Live command table passes.
    assert_eq!(COMMANDS.len(), 6);
    verify_command_table(COMMANDS).expect("COMMANDS must contain strictly the six checks");

    let actual_names: BTreeSet<&str> = COMMANDS.iter().map(|&(n, _)| n).collect();
    let expected_names: BTreeSet<&str> = EXPECTED_SUBCOMMANDS.into_iter().collect();
    assert_eq!(actual_names, expected_names);
    assert!(
        !actual_names.contains("all"),
        "no `all` subcommand may exist (BD-004)"
    );

    // Negative control 1: adding a seventh subcommand MUST fail the guard.
    let mut with_seventh: Vec<(&str, CheckFn)> = COMMANDS.to_vec();
    with_seventh.push(("check-seventh", dummy_seventh_handler));
    let err_seventh = verify_command_table(&with_seventh)
        .expect_err("adding a seventh subcommand must fail INV-WS-05 guard");
    assert_eq!(
        err_seventh,
        CommandTableError::NameSetMismatch {
            missing: Vec::new(),
            unexpected: vec!["check-seventh".to_owned()],
        }
    );

    // Negative control 2: renaming one entry (still count 6!) MUST also fail because
    // the guard compares the set of names, not only the count.
    let mut renamed: Vec<(&str, CheckFn)> = COMMANDS.to_vec();
    renamed[5] = ("check-bogus", dummy_seventh_handler);
    let err_renamed = verify_command_table(&renamed)
        .expect_err("renaming a subcommand must fail set comparison even when len == 6");
    assert_eq!(
        err_renamed,
        CommandTableError::NameSetMismatch {
            missing: vec!["check-stack".to_owned()],
            unexpected: vec!["check-bogus".to_owned()],
        }
    );

    // No-argument invocation exits 3 and prints all six names.
    let mut out = Vec::new();
    let mut err = Vec::new();
    let code = cli::run(COMMANDS, &[], &mut out, &mut err);
    assert_eq!(code, ExitCode::from(EXIT_USAGE));
    assert!(out.is_empty());
    let stderr = String::from_utf8(err).expect("utf8 stderr");
    for &name in &EXPECTED_SUBCOMMANDS {
        assert!(
            stderr.contains(name),
            "no-arg usage output must list `{name}`, got:\n{stderr}"
        );
    }
}

#[test]
fn unknown_subcommand_is_usage_error() {
    for bad in ["unknown-check", "all", "check-seventh"] {
        let mut out = Vec::new();
        let mut err = Vec::new();
        let args = vec![bad.to_owned()];
        let code = cli::run(COMMANDS, &args, &mut out, &mut err);
        assert_eq!(
            code,
            ExitCode::from(EXIT_USAGE),
            "`{bad}` must exit 3 (usage error)"
        );
        assert!(out.is_empty(), "`{bad}` must emit no stdout");
        let stderr = String::from_utf8(err).expect("utf8 stderr");
        assert!(
            stderr.contains("unknown subcommand"),
            "expected unknown subcommand diagnostic for `{bad}`, got:\n{stderr}"
        );
        for &name in &EXPECTED_SUBCOMMANDS {
            assert!(
                stderr.contains(name),
                "usage stderr for `{bad}` must list `{name}`, got:\n{stderr}"
            );
        }
    }
}

// ============================================================================
// 2. Unique finding-identifier ownership rule
// ============================================================================

#[test]
fn every_finding_id_has_one_owner() {
    let map = verify_finding_id_ownership(CHECK_OWNERS)
        .expect("union of finding IDs across all six checks must have zero duplicates");
    assert!(
        map.len() >= 50,
        "expected full union of finding identifiers across all six checks, got {}",
        map.len()
    );

    // Every one of the six checks owns at least one rule identifier.
    let represented_owners: BTreeSet<&str> = map.values().map(String::as_str).collect();
    let expected_owners: BTreeSet<&str> = EXPECTED_SUBCOMMANDS.into_iter().collect();
    assert_eq!(represented_owners, expected_owners);

    // Negative control: duplicating one rule ID across two checks MUST fail naming both owners.
    let colliding: &[(&str, &[&str])] = &[
        ("check-layers", &["L-1.upward", "shared.rule"]),
        ("check-deps", &["asup.duplicate", "shared.rule"]),
        ("check-unsafe", &["unsafe.core-lint"]),
        ("check-generated", &["gen.differs"]),
        ("check-contracts", &["contract.missing"]),
        ("check-stack", &["stack.pin"]),
    ];
    let err = verify_finding_id_ownership(colliding)
        .expect_err("duplicate finding ID across two checks must fail ownership verification");
    assert_eq!(
        err,
        OwnershipError::DuplicateAcrossOwners {
            rule: "shared.rule".to_owned(),
            first_owner: "check-layers".to_owned(),
            second_owner: "check-deps".to_owned(),
        }
    );
}

// ============================================================================
// 3. GATE-WS-01 7 x 6 Plant Matrix + Harness Negative Controls + D0 Determinism
// ============================================================================

#[test]
fn plant_matrix_diagonal() {
    assert_eq!(GATE_WS_01_PLANTS.len(), 7);

    // 0. Clean copy (and no-op plant negative control): ALL 6 checks exit 0, and
    //    two runs produce byte-for-byte identical `--json` reports excluding `elapsed_ms`.
    let clean = MiniatureEnv::build("clean");
    let clean_run_1 = clean.run_all_six();
    let clean_run_2 = clean.run_all_six();
    for &check in &EXPECTED_SUBCOMMANDS {
        let (code1, findings1, raw1) = &clean_run_1[check];
        assert_eq!(
            *code1,
            ExitCode::from(EXIT_OK),
            "clean miniature workspace must pass `{check}`, got findings={findings1:?}"
        );
        assert!(findings1.is_empty());

        let (_, _, raw2) = &clean_run_2[check];
        let norm1 = normalize_report_json_for_determinism(raw1).expect("normalize run 1");
        let norm2 = normalize_report_json_for_determinism(raw2).expect("normalize run 2");
        assert_eq!(
            norm1, norm2,
            "`{check}` --json output must be D0 byte-identical across two runs excluding elapsed_ms"
        );
    }

    // Track exact counts across the 7 x 6 matrix: must be 7 failures and 35 passes.
    let mut total_failures = 0usize;
    let mut total_passes = 0usize;

    // Helper verifying one row of the 7 x 6 diagonal matrix.
    let mut assert_plant_row = |plant_name: &str,
                                expected_owner: &str,
                                expected_rule: &str,
                                env: &MiniatureEnv| {
        let res = env.run_all_six();
        for &check in &EXPECTED_SUBCOMMANDS {
            let (code, findings, _raw) = &res[check];
            if check == expected_owner {
                assert_eq!(
                    *code,
                    ExitCode::from(EXIT_FINDINGS),
                    "{plant_name}: owning check `{check}` must exit 2, got findings={findings:?}"
                );
                assert!(
                    findings.iter().any(|f| f.rule == expected_rule),
                    "{plant_name}: owning check `{check}` must emit `{expected_rule}`, got {findings:?}"
                );
                total_failures += 1;
            } else {
                assert_eq!(
                    *code,
                    ExitCode::from(EXIT_OK),
                    "{plant_name}: non-owning check `{check}` must exit 0 (caught by no other), got findings={findings:?}"
                );
                assert!(findings.is_empty());
                total_passes += 1;
            }
        }
    };

    // Plant 1: deliberate upward edge (`frob-core` L0 -> `frob-lie` L1), normalized.
    {
        let env = MiniatureEnv::build("plant-1-upward");
        write_file(
            &env.repo.join("crates/frob-core/Cargo.toml"),
            "[package]\nname = \"frob-core\"\nversion = \"0.0.1\"\nedition = \"2024\"\n\n\
             [package.metadata.frob]\nlayer = 0\nrank = 1\n\n\
             [dependencies]\n\
             asupersync = { version = \"=0.6.2\", default-features = false }\n\
             frob-lie = { path = \"../frob-lie\" }\n\n\
             [lints]\nworkspace = true\n",
        );
        env.normalize_manifest_change();
        assert_plant_row("plant-1-upward-edge", "check-layers", RULE_L1_UPWARD, &env);
    }

    // Plant 2: deliberate intra-layer edge against rank (`frob-lie` L1/R1 -> `frob-algebra` L1/R2), normalized.
    {
        let env = MiniatureEnv::build("plant-2-rank");
        write_file(
            &env.repo.join("crates/frob-lie/Cargo.toml"),
            "[package]\nname = \"frob-lie\"\nversion = \"0.0.1\"\nedition = \"2024\"\n\n\
             [package.metadata.frob]\nlayer = 1\nrank = 1\n\n\
             [dependencies]\n\
             frob-algebra = { path = \"../frob-algebra\" }\n\n\
             [lints]\nworkspace = true\n",
        );
        env.normalize_manifest_change();
        assert_plant_row("plant-2-rank-edge", "check-layers", RULE_L1_RANK, &env);
    }

    // Plant 3: second `asupersync` package pulled into binary `frob` via admitted sibling path dep, normalized.
    {
        let env = MiniatureEnv::build("plant-3-asup-dup");
        let write_manifest = |features: &str| {
            write_file(
                &env.repo.join("crates/frob-algebra/Cargo.toml"),
                &format!(
                    "[package]\nname = \"frob-algebra\"\nversion = \"0.0.1\"\nedition = \"2024\"\n\n\
                     [package.metadata.frob]\nlayer = 1\nrank = 2\n\n\
                     [[bin]]\nname = \"frob\"\npath = \"src/main.rs\"\n\n\
                     [dependencies]\n\
                     frob-core = {{ path = \"../frob-core\" }}\n\
                     sib-demo = {{ path = \"../../../sib-demo\", default-features = false }}\n\
                     dummy-sib = {{ path = \"../../../dummy-sib\", default-features = false{features} }}\n\n\
                     [lints]\nworkspace = true\n"
                ),
            );
        };
        write_manifest("");
        let lock_path = env.repo.join("franken-stack.lock");
        let original_lock = fs::read_to_string(&lock_path).expect("read miniature stack lock");
        let lock_with_dummy = format!(
            "{original_lock}\n\
             [[foundation]]\n\
             id = \"FD-DUMMY\"\n\
             name = \"dummy-sib\"\n\
             posture = \"consume-path\"\n\
             path = \"../dummy-sib\"\n\
             git_rev = \"{}\"\n\
             toolchain = \"nightly-2026-08-31\"\n\
             confined_to = [\"frob-algebra\"]\n\
             admission_gate = \"INT-DUMMY-01\"\n\
             fallback = \"synthetic fallback\"\n",
            env.dummy_sib_rev
        );
        write_file(&lock_path, &lock_with_dummy);
        generate_lockfile_offline(&env.repo);
        let repo_rev = git_commit_all(&env.repo, "add dummy sibling without alternate asupersync");
        seed_miniature_admissions(&env.repo, &repo_rev);

        write_manifest(", features = [\"alternate-asup\"]");
        env.normalize_manifest_change();
        assert_plant_row(
            "plant-3-second-asupersync",
            "check-deps",
            RULE_ASUP_DUPLICATE,
            &env,
        );
    }

    // Plant 4: unregistered unsafe capsule inside annex `spark/frob-spark-cuda`.
    {
        let env = MiniatureEnv::build("plant-4-capsule");
        write_file(
            &env.repo
                .join("spark/frob-spark-cuda/src/unregistered_capsule.rs"),
            "pub fn raw_ptr_len(ptr: *const u8) -> usize {\n    unsafe { ptr as usize }\n}\n",
        );
        assert_plant_row(
            "plant-4-unregistered-capsule",
            "check-unsafe",
            RULE_CAPSULE_UNREGISTERED,
            &env,
        );
    }

    // Plant 5: hand-edited generated schema file `schemas/frob.design.1.json`.
    {
        let env = MiniatureEnv::build("plant-5-generated");
        let schema_path = env.repo.join("schemas/frob.design.1.json");
        let orig = fs::read_to_string(&schema_path).expect("read generated schema");
        let mut lines: Vec<String> = orig.split('\n').map(ToOwned::to_owned).collect();
        assert!(lines.len() >= 6);
        lines[5].push(' ');
        write_file(&schema_path, &lines.join("\n"));
        assert_plant_row(
            "plant-5-edited-generated-file",
            "check-generated",
            RULE_DIFFERS,
            &env,
        );
    }

    // Plant 6: `CONTRACT.md` missing a required section (`## Known gaps`).
    {
        let env = MiniatureEnv::build("plant-6-contract");
        let mut md = String::from("# Contract: frob-lie\n\n");
        for sec in REQUIRED_CONTRACT_SECTIONS {
            if *sec == "Known gaps" {
                continue;
            }
            md.push_str(&format!("## {sec}\n\nSynthetic body for `{sec}`.\n\n"));
        }
        write_file(&env.repo.join("crates/frob-lie/CONTRACT.md"), &md);
        assert_plant_row(
            "plant-6-contract-missing-section",
            "check-contracts",
            RULE_CONTRACT_SECTION_MISSING,
            &env,
        );
    }

    // Plant 7: stale `franken-stack.lock` (`FD-SIBDEMO` pinned to `sib_rev1` while sibling HEAD is `sib_rev2`).
    {
        let env = MiniatureEnv::build("plant-7-stale-lock");
        let lock_path = env.repo.join("franken-stack.lock");
        let orig_lock = fs::read_to_string(&lock_path).expect("read stack lock");
        let stale_lock = orig_lock.replace(&env.sib_rev2, &env.sib_rev1);
        assert_ne!(orig_lock, stale_lock, "sib_rev1 and sib_rev2 must differ");
        write_file(&lock_path, &stale_lock);
        assert_plant_row("plant-7-stale-stack-lock", "check-stack", RULE_PIN, &env);
    }

    assert_eq!(
        total_failures, 7,
        "7 x 6 diagonal must have exactly 7 failures"
    );
    assert_eq!(
        total_passes, 35,
        "7 x 6 diagonal must have exactly 35 passes"
    );

    // Harness negative control: Plant 1 applied WITHOUT receipt normalization MUST be
    // reported by TWO checks (`check-layers` L-1.upward AND `check-stack` stack.lockfile),
    // proving the harness can see double reporting.
    {
        let env = MiniatureEnv::build("neg-control-unnormalized");
        write_file(
            &env.repo.join("crates/frob-core/Cargo.toml"),
            "[package]\nname = \"frob-core\"\nversion = \"0.0.1\"\nedition = \"2024\"\n\n\
             [package.metadata.frob]\nlayer = 0\nrank = 1\n\n\
             [dependencies]\n\
             asupersync = { version = \"=0.6.2\", default-features = false }\n\
             frob-lie = { path = \"../frob-lie\" }\n\n\
             [lints]\nworkspace = true\n",
        );
        // Regenerate Cargo.lock so Cargo metadata succeeds, but intentionally DO NOT
        // refresh `receipts/HEAD` (omitting normalization).
        generate_lockfile_offline(&env.repo);
        let res = env.run_all_six();
        let failing_checks: Vec<&str> = EXPECTED_SUBCOMMANDS
            .iter()
            .copied()
            .filter(|&c| res[c].0 == ExitCode::from(EXIT_FINDINGS))
            .collect();
        assert_eq!(
            failing_checks,
            vec!["check-layers", "check-stack"],
            "unnormalized manifest plant must be reported by both check-layers and check-stack, got {failing_checks:?}"
        );
        assert!(
            res["check-layers"]
                .1
                .iter()
                .any(|f| f.rule == RULE_L1_UPWARD)
        );
        assert!(res["check-stack"].1.iter().any(|f| f.rule == RULE_LOCKFILE));
    }
}

// ============================================================================
// 4. Plant 5 on a scratch copy of the real tree
// ============================================================================

#[cfg(unix)]
fn link_real_siblings(real_root: &Path, scratch_parent: &Path) {
    let Some(real_parent) = real_root.parent() else {
        return;
    };
    let Ok(entries) = fs::read_dir(real_parent) else {
        return;
    };
    let real_name = real_root.file_name();
    for ent in entries.flatten() {
        let p = ent.path();
        if !p.is_dir() {
            continue;
        }
        let name = ent.file_name();
        if Some(name.as_os_str()) == real_name {
            continue;
        }
        let dst = scratch_parent.join(&name);
        if !dst.exists() {
            let _ = std::os::unix::fs::symlink(&p, &dst);
        }
    }
}

#[cfg(not(unix))]
fn link_real_siblings(_real_root: &Path, _scratch_parent: &Path) {}

fn build_real_tree_scratch_copy(label: &str) -> PathBuf {
    let real = repo_root();
    let scratch_parent = make_scratch_dir(label);
    let dst = scratch_parent.join("repo");
    fs::create_dir_all(&dst).expect("mkdir real scratch copy");

    for file in [
        "Cargo.toml",
        "Cargo.lock",
        "LICENSE",
        "README.md",
        "DEPENDENCY_ADMISSION.md",
        "rust-toolchain.toml",
        "franken-stack.lock",
        "docs/generated/refusals.md",
    ] {
        let src = real.join(file);
        if src.is_file() {
            copy_file(&src, &dst.join(file));
        }
    }
    for dir in [
        ".cargo",
        "assets",
        "crates",
        "demos",
        "docs/adr",
        "registries",
        "schemas",
        "receipts",
        "kernels",
    ] {
        let src = real.join(dir);
        if src.is_dir() {
            copy_dir_recursive_skipping(&src, &dst.join(dir), &["target", ".git"]);
        }
    }

    // Initialize local git history on the scratch copy so `check-stack` runs cleanly.
    let _rev = git_init_repo(&dst);
    link_real_siblings(&real, &scratch_parent);
    dst
}

#[test]
fn plant_5_on_real_tree_scratch_copy() {
    let scratch_repo = build_real_tree_scratch_copy("real-plant5");

    // 1. Baseline all six checks on the unedited scratch copy: every check must exit 0 with zero findings.
    for &check in &EXPECTED_SUBCOMMANDS {
        let run = run_check_cli_json(&scratch_repo, check, &[]);
        let (_, _, findings) = parse_report_findings_and_elapsed(&run);
        assert_eq!(
            run.exit_code,
            ExitCode::from(EXIT_OK),
            "baseline `{check}` on real-tree scratch copy must exit 0; findings={findings:?}\nstderr={}",
            run.stderr
        );
        assert!(
            findings.is_empty(),
            "baseline `{check}` on real-tree scratch copy must report zero findings, got {findings:?}"
        );
    }

    // 2. Apply Plant 5: flip one byte on line 6 of the real generated `schemas/frob.design.1.json`.
    let target_rel = "schemas/frob.design.1.json";
    let target_path = scratch_repo.join(target_rel);
    let orig = fs::read_to_string(&target_path).expect("read real generated schema");
    let mut lines: Vec<String> = orig.split('\n').map(ToOwned::to_owned).collect();
    assert!(
        lines.len() >= 8,
        "real frob.design.1.json must have >= 8 lines"
    );
    lines[5].push(' ');
    write_file(&target_path, &lines.join("\n"));

    // 3. Re-run all six checks: ONLY `check-generated` reports Plant 5 (`gen.differs` on line 6);
    // all five non-owning checks must exit 0 with zero findings (BD-004 AC2).
    for &check in &EXPECTED_SUBCOMMANDS {
        let run = run_check_cli_json(&scratch_repo, check, &[]);
        let (_, _, after_findings) = parse_report_findings_and_elapsed(&run);

        if check == "check-generated" {
            assert_eq!(
                run.exit_code,
                ExitCode::from(EXIT_FINDINGS),
                "Plant 5 on real-tree scratch copy must cause check-generated to exit 2; stderr={}",
                run.stderr
            );
            assert!(
                after_findings
                    .iter()
                    .any(|f| f.rule == RULE_DIFFERS && f.path == target_rel && f.line == Some(6)),
                "check-generated must report gen.differs on {target_rel} line 6, got {after_findings:?}"
            );
        } else {
            assert_eq!(
                run.exit_code,
                ExitCode::from(EXIT_OK),
                "Plant 5 must leave non-owning check `{check}` clean (exit 0), got findings={after_findings:?}\nstderr={}",
                run.stderr
            );
            assert!(
                after_findings.is_empty(),
                "Plant 5 must produce zero findings in non-owning check `{check}`, got {after_findings:?}"
            );
        }
    }
}

// ============================================================================
// 5. GATE-WS-04 Timing Table & In-Process Warm-Tree Timing
// ============================================================================

#[test]
fn in_process_timing() {
    // Verify the compiled GATE-WS-04 budget table arithmetic first.
    let shares_sum: u64 = GATE_WS_04_SHARES.iter().map(|s| s.budget_ms).sum();
    assert_eq!(shares_sum, GATE_WS_04_TOTAL_BUDGET_MS);
    let subshares_sum: u64 = GATE_WS_04_LAYERS_AND_DEPS_SUBSHARES
        .iter()
        .map(|&(_, ms)| ms)
        .sum();
    assert_eq!(subshares_sum, GATE_WS_04_LAYERS_AND_DEPS_BUDGET_MS);

    // Negative control: an overrun on a single check is reported by name even when total <= 60 s.
    let mut synthetic_overrun = BTreeMap::new();
    synthetic_overrun.insert("check-layers".to_owned(), 1_000);
    synthetic_overrun.insert("check-deps".to_owned(), 2_000);
    synthetic_overrun.insert("check-unsafe".to_owned(), 6_000); // > 5_000 ms share!
    synthetic_overrun.insert("check-generated".to_owned(), 1_000);
    synthetic_overrun.insert("check-contracts".to_owned(), 1_000);
    synthetic_overrun.insert("check-stack".to_owned(), 1_000);
    let eval_over =
        evaluate_gate_ws_04_timing(&synthetic_overrun).expect("evaluate synthetic timing");
    assert!(eval_over.within_total_budget, "total 12 s is <= 60 s");
    assert!(
        !eval_over.passed(),
        "single-share overrun must fail evaluation"
    );
    assert_eq!(eval_over.overruns.len(), 1);
    assert_eq!(eval_over.overruns[0].name, "check-unsafe");
    assert!(eval_over.format_summary().contains("check-unsafe"));

    // Negative control 2: combined `check-layers + check-deps` > 25 s reported by name.
    let mut combined_overrun = synthetic_overrun;
    combined_overrun.insert("check-unsafe".to_owned(), 1_000);
    combined_overrun.insert("check-layers".to_owned(), 9_000);
    combined_overrun.insert("check-deps".to_owned(), 17_000); // combined 26_000 > 25_000
    let eval_comb =
        evaluate_gate_ws_04_timing(&combined_overrun).expect("evaluate combined timing");
    assert!(eval_comb.within_total_budget);
    assert!(!eval_comb.passed());
    assert!(
        eval_comb
            .overruns
            .iter()
            .any(|o| o.name == "check-layers+check-deps"),
        "expected check-layers+check-deps overrun, got {:?}",
        eval_comb.overruns
    );

    // Warm all six checks on the real tree first, then measure six clean warmed passes and
    // retain all six `elapsed_ms` values from canonical `frob.xtask.report/1` JSON reports.
    let real = repo_root();
    for &check in &EXPECTED_SUBCOMMANDS {
        let warm = run_check_cli_json(&real, check, &[]);
        let (_, _, warm_findings) = parse_report_findings_and_elapsed(&warm);
        assert_eq!(
            warm.exit_code,
            ExitCode::from(EXIT_OK),
            "warmup `{check}` on real tree must exit 0; findings={warm_findings:?}\nstderr={}",
            warm.stderr
        );
        assert!(
            warm_findings.is_empty(),
            "warmup `{check}` on real tree must report zero findings, got {warm_findings:?}"
        );
    }

    let mut measured: BTreeMap<String, u64> = BTreeMap::new();
    for &check in &EXPECTED_SUBCOMMANDS {
        let run = run_check_cli_json(&real, check, &[]);
        let (_, elapsed_ms, findings) = parse_report_findings_and_elapsed(&run);
        assert_eq!(
            run.exit_code,
            ExitCode::from(EXIT_OK),
            "warmed `{check}` on real tree must exit 0; findings={findings:?}\nstderr={}",
            run.stderr
        );
        assert!(
            findings.is_empty(),
            "warmed `{check}` on real tree must report zero findings, got {findings:?}"
        );
        measured.insert(check.to_owned(), elapsed_ms);
    }
    assert_eq!(measured.len(), EXPECTED_SUBCOMMANDS.len());

    let eval = evaluate_gate_ws_04_timing(&measured).expect("evaluate GATE-WS-04 timing");
    assert!(
        eval.passed(),
        "GATE-WS-04 warm-tree timing budget exceeded (measured={measured:?}):\n{}",
        eval.format_summary()
    );
}
