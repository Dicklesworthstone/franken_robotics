#![deny(unsafe_code)]

//! Unit and integration tests for the gate recorder (`N38b-19`, `GATE-GAUNT-10`, `GATE-GAUNT-01`).
//!
//! Every test builds a real scratch Git repository with real files under the OS temp dir
//! (never touches the checkout of record and never deletes files).

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Barrier, Mutex};
use std::thread;
use std::time::Duration;

use frob_conformance::control_runner::{CONTROL_HISTORY_PATH, load_history};
use frob_conformance::controls::CONTROLS_PATH;
use frob_conformance::reasons::HarnessReason;
use frob_conformance::register::kill::{apply_decisions, render_register_printout};
use frob_conformance::register::print::readme_table;
use frob_conformance::register::record::{
    RECEIPT_SET_SCHEMA, RecordHookContext, RecordHookRefusal, RecordOptions, RegistryLock,
    record_gate, register_check, unregister_check,
};
use frob_conformance::register::{Status, parse_register_bytes};
use frob_conformance::tiers::{
    ControlRunOutcome, GauntletReceipt, ReceiptControl, ReceiptCounts, ReceiptEntry, ReceiptGate,
    ReceiptMeasured, ReducedCount, Site, Tier, write_receipt,
};
use frob_core::evidence::DetClass;
use frob_core::id::hash_blob;
use frob_core::json::{self, Limits};
use frob_core::stack::BUILT_UNDER;

static SCRATCH_SEQ: AtomicU64 = AtomicU64::new(0);
static HOOK_TEST_LOCK: Mutex<()> = Mutex::new(());

const CLAIM_A: &str = "clm:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const CLAIM_B: &str = "clm:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";
const CLAIM_C: &str = "clm:cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc";

struct ScratchRepo {
    root: PathBuf,
    head: String,
}

impl ScratchRepo {
    fn new(name: &str, impl_sessions: &[&str]) -> Self {
        let seq = SCRATCH_SEQ.fetch_add(1, Ordering::Relaxed);
        let root =
            std::env::temp_dir().join(format!("frob-n38b19-{name}-{}-{seq}", std::process::id()));
        fs::create_dir_all(root.join("registries")).expect("mkdir registries");
        fs::create_dir_all(root.join("crates/frob-conformance/registry")).expect("mkdir reg");
        fs::create_dir_all(root.join("target/gauntlet/receipts")).expect("mkdir receipts");
        fs::create_dir_all(root.join("receipts/gates")).expect("mkdir committed receipts");
        fs::create_dir_all(root.join("docs/delivery/logs")).expect("mkdir logs");
        fs::create_dir_all(root.join("docs/adr")).expect("mkdir adr");

        git(&root, &["init", "-q"]);
        git(&root, &["config", "user.email", "test@franken.invalid"]);
        git(&root, &["config", "user.name", "Franken Test"]);
        git(&root, &["config", "commit.gpgsign", "false"]);

        for (idx, sess) in impl_sessions.iter().enumerate() {
            let f = root.join(format!("capability_{idx}.txt"));
            fs::write(&f, format!("impl {sess} {idx}\n")).expect("write impl file");
            git(&root, &["add", "."]);
            let msg = format!("implement capability {idx}\n\nFrob-Session: {sess}");
            git(&root, &["commit", "-q", "-m", &msg]);
        }
        let head = git_stdout(&root, &["rev-parse", "HEAD"]);
        Self { root, head }
    }

    fn write_gates(&self, toml: &str) -> PathBuf {
        let p = self.root.join("registries/gates.toml");
        fs::write(&p, toml).expect("write gates.toml");
        p
    }

    fn write_controls(&self, toml: &str) -> PathBuf {
        let p = self.root.join(CONTROLS_PATH);
        fs::write(&p, toml).expect("write controls.toml");
        p
    }

    fn write_receipt(&self, receipt: &GauntletReceipt) -> (PathBuf, String) {
        write_receipt(&self.root.join("target/gauntlet/receipts"), receipt)
            .expect("write canonical receipt")
    }

    fn commit_extra(&self, file: &str, content: &str, sess: &str) -> String {
        fs::write(self.root.join(file), content).expect("write extra");
        git(&self.root, &["add", file]);
        let msg = format!("update {file}\n\nFrob-Session: {sess}");
        git(&self.root, &["commit", "-q", "-m", &msg]);
        git_stdout(&self.root, &["rev-parse", "HEAD"])
    }
}

fn git(root: &Path, args: &[&str]) {
    let out = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .output()
        .expect("run git");
    assert!(
        out.status.success(),
        "git {args:?} failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
}

fn git_stdout(root: &Path, args: &[&str]) -> String {
    let out = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .output()
        .expect("run git");
    assert!(out.status.success(), "git {args:?} failed");
    String::from_utf8_lossy(&out.stdout).trim().to_owned()
}

fn std_controls_two() -> &'static str {
    r#"schema = "frob.gauntlet.controls/1"

[[control]]
id = "NC-0001"
seed = "GATE-TEST-01"
gate = ["GATE-TEST-01"]
kind = "planted-input"
target = "frob-conformance::test::a"
defect = "planted defect A"
expect = "fails with code 1"
fixture = "fixtures/a.toml"
due = "M0"
state = "runnable"

[[control]]
id = "NC-0002"
seed = "GATE-TEST-01"
gate = ["GATE-TEST-01"]
kind = "planted-input"
target = "frob-conformance::test::b"
defect = "planted defect B"
expect = "fails with code 2"
fixture = "fixtures/b.toml"
due = "M0"
state = "runnable"
"#
}

fn std_gate_toml(id: &str, claim: &str, claim_id: &str, controls: &str) -> String {
    format!(
        r#"schema = "frob.gates/1"

[[gate]]
id = "{id}"
kind = "convergence"
claim = "{claim}"
claim_id = "{claim_id}"
section = "38.15"
threshold = "spectral radius <= 0.95"
milestone = "M0"
tier = ["G1:host"]
controls = [{controls}]

[[gate.part]]
milestone = "M0"
status = "not-run"
measured = ""
evidence = []
recorded = {{ date = "", session = "", implemented_by = "" }}
"#
    )
}

fn make_receipt(
    commit: &str,
    tier: Tier,
    gate_id: &str,
    gate_line: &str,
    controls: Vec<ReceiptControl>,
    entries: Vec<ReceiptEntry>,
) -> GauntletReceipt {
    GauntletReceipt {
        schema: "frob.gauntlet.receipt/1".to_owned(),
        tier,
        filter: format!("gate:{gate_id}"),
        commit: commit.to_owned(),
        dirty: false,
        toolchain: "rustc-nightly-2026-07-06".to_owned(),
        fingerprint: "host-cpu-fp".to_owned(),
        seed: 42,
        profile: "dev".to_owned(),
        started: "2026-11-25T12:00:00Z".to_owned(),
        wall_ms: 12,
        entries,
        controls,
        gates: vec![ReceiptGate {
            id: gate_id.to_owned(),
            line: gate_line.to_owned(),
            reduced: None,
        }],
        counts: ReceiptCounts {
            pass: 1,
            fail: 0,
            refused: 0,
            skipped: 0,
            obligations: 0,
        },
        session: "session:runner".to_owned(),
        stack_receipt: BUILT_UNDER.receipt.to_string(),
        stack_dev: false,
        build_identity: "depgraph-receipt".to_owned(),
    }
}

fn entry_ok(id: &str, val: &str, unit: &str) -> ReceiptEntry {
    ReceiptEntry {
        id: id.to_owned(),
        site: Site::Host,
        outcome: "pass".to_owned(),
        measured: Some(ReceiptMeasured {
            name: "rho".to_owned(),
            value: val.to_owned(),
            unit: unit.to_owned(),
            threshold: "<= 0.95".to_owned(),
        }),
        first_diff: None,
        determinism: Some(DetClass::D0Exact),
        synthetic: false,
        reduced: None,
    }
}

fn ctrl(id: &str, gate: &str, outcome: ControlRunOutcome) -> ReceiptControl {
    ReceiptControl {
        id: id.to_owned(),
        gate: gate.to_owned(),
        outcome,
    }
}

fn base_opts(repo: &ScratchRepo, gate_id: &str, hash: String, session: &str) -> RecordOptions {
    let mut opts = RecordOptions::new(&repo.root, gate_id, vec![hash]);
    opts.session = Some(session.to_owned());
    opts.date = Some("2026-11-25".to_owned());
    opts.bead_id = Some("BD-TEST".to_owned());
    opts.skip_global_checks = true;
    opts
}

#[test]
fn record_pass_happy_path() {
    let repo = ScratchRepo::new("happy", &["session:s1"]);
    repo.write_gates(&std_gate_toml(
        "GATE-TEST-01",
        "known-answer",
        CLAIM_A,
        "\"NC-0001\", \"NC-0002\"",
    ));
    repo.write_controls(std_controls_two());
    let rec = make_receipt(
        &repo.head,
        Tier::G1KnownAnswers,
        "GATE-TEST-01",
        "pass",
        vec![
            ctrl("NC-0001", "GATE-TEST-01", ControlRunOutcome::Caught),
            ctrl("NC-0002", "GATE-TEST-01", ControlRunOutcome::Caught),
        ],
        vec![entry_ok("ka:wheel", "0.9207", "1")],
    );
    let (_, hash) = repo.write_receipt(&rec);
    let out = record_gate(&base_opts(
        &repo,
        "GATE-TEST-01",
        hash.clone(),
        "session:s2",
    ))
    .expect("happy path records pass");

    assert_eq!(out.part.status, Status::Pass);
    assert_eq!(out.part.measured, "0.9207 (1)");
    assert_eq!(out.part.recorded.session, "session:s2");
    assert_eq!(out.part.recorded.implemented_by, "session:s1");
    assert_eq!(out.part.receipt, hash);
    assert!(!out.idempotent_noop);

    let history = load_history(&repo.root).expect("load history");
    assert_eq!(
        history.len(),
        2,
        "two history lines written for NC-0001 and NC-0002"
    );
    let log_raw = fs::read_to_string(repo.root.join("docs/delivery/logs/BD-TEST.ndjson"))
        .expect("bead log written");
    assert!(log_raw.contains("\"recorded_by\":\"session:s2\""));
    assert!(log_raw.contains(&format!("\"receipt\":\"{hash}\"")));
}

#[test]
fn record_idempotent() {
    let repo = ScratchRepo::new("idem", &["session:s1"]);
    repo.write_gates(&std_gate_toml(
        "GATE-TEST-01",
        "known-answer",
        CLAIM_A,
        "\"NC-0001\", \"NC-0002\"",
    ));
    repo.write_controls(std_controls_two());
    let rec = make_receipt(
        &repo.head,
        Tier::G1KnownAnswers,
        "GATE-TEST-01",
        "pass",
        vec![
            ctrl("NC-0001", "GATE-TEST-01", ControlRunOutcome::Caught),
            ctrl("NC-0002", "GATE-TEST-01", ControlRunOutcome::Caught),
        ],
        vec![entry_ok("ka:wheel", "0.9207", "1")],
    );
    let (_, hash) = repo.write_receipt(&rec);
    let opts = base_opts(&repo, "GATE-TEST-01", hash, "session:s2");
    let first = record_gate(&opts).expect("first record");
    assert!(!first.idempotent_noop);

    let before_gates = fs::read(repo.root.join("registries/gates.toml")).unwrap();
    let before_hist = fs::read(repo.root.join(CONTROL_HISTORY_PATH)).unwrap();
    let before_log = fs::read(repo.root.join("docs/delivery/logs/BD-TEST.ndjson")).unwrap();

    let second = record_gate(&opts).expect("second record is idempotent no-op");
    assert!(second.idempotent_noop);
    assert_eq!(
        fs::read(repo.root.join("registries/gates.toml")).unwrap(),
        before_gates
    );
    assert_eq!(
        fs::read(repo.root.join(CONTROL_HISTORY_PATH)).unwrap(),
        before_hist
    );
    assert_eq!(
        fs::read(repo.root.join("docs/delivery/logs/BD-TEST.ndjson")).unwrap(),
        before_log
    );
}

#[test]
fn record_history_directory_refuses_then_retry_repairs_each_effect_once() {
    let repo = ScratchRepo::new("history_directory", &["session:s1"]);
    let gates = repo.write_gates(&std_gate_toml(
        "GATE-TEST-01",
        "known-answer",
        CLAIM_A,
        "\"NC-0001\", \"NC-0002\"",
    ));
    let controls = repo.write_controls(std_controls_two());
    let receipt = make_receipt(
        &repo.head,
        Tier::G1KnownAnswers,
        "GATE-TEST-01",
        "pass",
        vec![
            ctrl("NC-0001", "GATE-TEST-01", ControlRunOutcome::Caught),
            ctrl("NC-0002", "GATE-TEST-01", ControlRunOutcome::Caught),
        ],
        vec![entry_ok("ka:wheel", "0.9207", "1")],
    );
    let (source_receipt, hash) = repo.write_receipt(&receipt);
    let receipt_dir = repo.root.join("receipts/gates");
    let committed = receipt_dir.join(format!("{hash}.json"));
    let readme = repo.root.join("README.md");
    let initial_readme = b"# Gates\n\n<!-- status:begin -->\n<!-- status:end -->\n";
    fs::write(&readme, initial_readme).unwrap();
    let log = repo.root.join("docs/delivery/logs/BD-TEST.ndjson");
    let initial_log = b"{\"kind\":\"implemented\"}\n";
    fs::write(&log, initial_log).unwrap();
    let mut opts = base_opts(&repo, "GATE-TEST-01", hash, "session:s2");
    let history = repo.root.join("configured-history.ndjson");
    opts.control_history_path = Some(history.clone());
    opts.readme_path = Some(readme.clone());
    fs::create_dir(&history).unwrap();
    let sentinel = history.join("keep");
    fs::write(&sentinel, b"unchanged history directory").unwrap();
    let before_gates = fs::read(&gates).unwrap();
    let before_controls = fs::read(&controls).unwrap();

    let err = record_gate(&opts).expect_err("history directory must refuse the entire recording");
    assert_eq!(err.reason, "io-error");
    assert!(err.detail.contains("configured-history.ndjson"), "{err}");
    assert_eq!(fs::read(&gates).unwrap(), before_gates);
    assert_eq!(fs::read(&controls).unwrap(), before_controls);
    assert_eq!(fs::read_dir(&receipt_dir).unwrap().count(), 0);
    assert_eq!(fs::read(&sentinel).unwrap(), b"unchanged history directory");
    assert_eq!(fs::read_dir(&history).unwrap().count(), 1);
    assert_eq!(fs::read(&log).unwrap(), initial_log);
    assert_eq!(fs::read(&readme).unwrap(), initial_readme);

    fs::rename(&history, repo.root.join("planted-history-directory")).unwrap();
    let first = record_gate(&opts).expect("usable history destination allows recording");
    assert_eq!(first.part.status, Status::Pass);
    assert!(!first.idempotent_noop);
    assert_eq!(
        fs::read(committed).unwrap(),
        fs::read(source_receipt).unwrap()
    );
    assert_eq!(fs::read_dir(&receipt_dir).unwrap().count(), 1);
    let saved_gates = fs::read(&gates).unwrap();
    let register = parse_register_bytes(&saved_gates, "gates.toml").unwrap();
    let parts = &register.get("GATE-TEST-01").unwrap().parts;
    assert_eq!(parts.len(), 1);
    assert_eq!(parts[0].receipt, first.part.receipt);
    let saved_history = fs::read_to_string(&history).unwrap();
    let entries =
        frob_conformance::control_runner::parse_history_ndjson(&saved_history, "history").unwrap();
    assert_eq!(
        entries
            .iter()
            .map(|entry| entry.id.as_str())
            .collect::<Vec<_>>(),
        ["NC-0001", "NC-0002"]
    );
    let saved_log = fs::read_to_string(&log).unwrap();
    assert_eq!(saved_log.matches("\"kind\":\"gate-record\"").count(), 1);
    let saved_readme = fs::read(&readme).unwrap();
    assert_ne!(saved_readme, initial_readme);

    // Model a previously published pass with only one history row and stale derived files.
    fs::write(
        &history,
        format!("{}\n", saved_history.lines().next().unwrap()),
    )
    .unwrap();
    fs::write(&log, initial_log).unwrap();
    fs::write(&readme, initial_readme).unwrap();
    opts.date = Some("2026-11-26".to_owned());
    let repaired = record_gate(&opts).expect("retry fills missing effects");
    assert!(!repaired.idempotent_noop);
    assert_eq!(repaired.part.recorded.date, first.part.recorded.date);
    assert_eq!(fs::read(&gates).unwrap(), saved_gates);
    assert_eq!(fs::read_to_string(&history).unwrap(), saved_history);
    assert_eq!(fs::read_to_string(&log).unwrap(), saved_log);
    assert_eq!(fs::read(&readme).unwrap(), saved_readme);
    assert!(record_gate(&opts).unwrap().idempotent_noop);

    for (path, complete) in [
        (&history, saved_history.as_bytes()),
        (&log, saved_log.as_bytes()),
    ] {
        // A complete final JSON record needs no terminating newline. Removing its closing
        // brace as well models the interrupted append left by the old recorder.
        fs::write(path, &complete[..complete.len() - 1]).unwrap();
        assert!(record_gate(&opts).unwrap().idempotent_noop);
        let torn = &complete[..complete.len() - 2];
        let offset = torn.iter().rposition(|&byte| byte == b'\n').unwrap() + 1;
        assert!(json::admit(&torn[offset..], &Limits::default()).is_err());
        fs::write(path, torn).unwrap();
        let err = record_gate(&opts).expect_err("an incomplete NDJSON row must refuse");
        assert_eq!(err.reason, "ndjson-invalid");
        assert!(
            err.detail
                .contains(&format!("`{}` at byte {offset}", path.display())),
            "{err}"
        );
        assert_eq!(fs::read(path).unwrap(), torn);
        assert_eq!(fs::read(&gates).unwrap(), saved_gates);
        assert_eq!(fs::read(&readme).unwrap(), saved_readme);
        fs::write(path, complete).unwrap();
    }
    assert!(record_gate(&opts).unwrap().idempotent_noop);
}

#[test]
fn refuse_self_certified() {
    let repo = ScratchRepo::new("selfcert", &["session:s1", "session:s3"]);
    let gates_path = repo.write_gates(&std_gate_toml(
        "GATE-TEST-01",
        "known-answer",
        CLAIM_A,
        "\"NC-0001\", \"NC-0002\"",
    ));
    let orig = fs::read(&gates_path).unwrap();
    let controls_path = repo.write_controls(std_controls_two());
    let original_controls = fs::read(&controls_path).unwrap();
    let rec = make_receipt(
        &repo.head,
        Tier::G1KnownAnswers,
        "GATE-TEST-01",
        "pass",
        vec![
            ctrl("NC-0001", "GATE-TEST-01", ControlRunOutcome::Caught),
            ctrl("NC-0002", "GATE-TEST-01", ControlRunOutcome::Caught),
        ],
        vec![entry_ok("ka:wheel", "0.9207", "1")],
    );
    let (_, hash) = repo.write_receipt(&rec);

    for bad_sess in ["session:s1", "session:s3"] {
        let err = record_gate(&base_opts(&repo, "GATE-TEST-01", hash.clone(), bad_sess))
            .expect_err("implementer must be refused");
        assert_eq!(err.harness_reason, Some(HarnessReason::SelfCertified));
        assert_eq!(fs::read(&gates_path).unwrap(), orig);
    }
    let mut unavailable_scope = base_opts(&repo, "GATE-TEST-01", hash.clone(), "session:s1");
    unavailable_scope
        .delivering_paths
        .push(repo.root.with_extension("outside"));
    let mut unreadable_log = base_opts(&repo, "GATE-TEST-01", hash.clone(), "session:s1");
    unreadable_log
        .bead_logs
        .push(repo.root.join("docs/delivery/logs"));
    for (opts, source) in [
        (unavailable_scope, "git log failed"),
        (unreadable_log, "bead log"),
    ] {
        let err = record_gate(&opts).expect_err("failed provenance must refuse recording");
        assert_eq!(err.reason, "provenance-unavailable");
        assert!(err.detail.contains(source), "{err}");
        assert_eq!(fs::read(&gates_path).unwrap(), orig);
        assert_eq!(fs::read(&controls_path).unwrap(), original_controls);
        assert_eq!(
            fs::read_dir(repo.root.join("receipts/gates"))
                .unwrap()
                .count(),
            0
        );
        assert_eq!(
            fs::read_dir(repo.root.join("docs/delivery/logs"))
                .unwrap()
                .count(),
            0
        );
        assert!(!repo.root.join(CONTROL_HISTORY_PATH).exists());
    }
    let ok = record_gate(&base_opts(&repo, "GATE-TEST-01", hash, "session:s2"))
        .expect("non-implementer s2 succeeds");
    assert_eq!(ok.part.status, Status::Pass);
}

#[test]
fn refuse_control_missed() {
    let repo = ScratchRepo::new("ctrl_missed", &["session:s1"]);
    let gates_path = repo.write_gates(&std_gate_toml(
        "GATE-TEST-01",
        "known-answer",
        CLAIM_A,
        "\"NC-0001\", \"NC-0002\"",
    ));
    let orig = fs::read(&gates_path).unwrap();
    repo.write_controls(std_controls_two());
    let rec = make_receipt(
        &repo.head,
        Tier::G1KnownAnswers,
        "GATE-TEST-01",
        "pass",
        vec![
            ctrl("NC-0001", "GATE-TEST-01", ControlRunOutcome::Caught),
            ctrl("NC-0002", "GATE-TEST-01", ControlRunOutcome::Missed),
        ],
        vec![entry_ok("ka:wheel", "0.9207", "1")],
    );
    let (_, hash) = repo.write_receipt(&rec);
    let err = record_gate(&base_opts(&repo, "GATE-TEST-01", hash, "session:s2")).unwrap_err();
    assert_eq!(err.harness_reason, Some(HarnessReason::ControlNotCaught));
    assert_eq!(fs::read(&gates_path).unwrap(), orig);
}

#[test]
fn refuse_control_absent_from_receipt() {
    let repo = ScratchRepo::new("ctrl_absent", &["session:s1"]);
    let gates_path = repo.write_gates(&std_gate_toml(
        "GATE-TEST-01",
        "known-answer",
        CLAIM_A,
        "\"NC-0001\", \"NC-0002\"",
    ));
    let orig = fs::read(&gates_path).unwrap();
    repo.write_controls(std_controls_two());
    let rec = make_receipt(
        &repo.head,
        Tier::G1KnownAnswers,
        "GATE-TEST-01",
        "pass",
        vec![ctrl("NC-0001", "GATE-TEST-01", ControlRunOutcome::Caught)],
        vec![entry_ok("ka:wheel", "0.9207", "1")],
    );
    let (_, hash) = repo.write_receipt(&rec);
    let err = record_gate(&base_opts(&repo, "GATE-TEST-01", hash, "session:s2")).unwrap_err();
    assert_eq!(
        err.harness_reason,
        Some(HarnessReason::ControlMissingFromReceipt)
    );
    assert!(err.detail.contains("NC-0002"));
    assert_eq!(fs::read(&gates_path).unwrap(), orig);
}

#[test]
fn refuse_empty_controls() {
    let repo = ScratchRepo::new("empty_ctrl", &["session:s1"]);
    let gates_path = repo.write_gates(&std_gate_toml("GATE-TEST-01", "known-answer", CLAIM_A, ""));
    let orig = fs::read(&gates_path).unwrap();
    repo.write_controls("schema = \"frob.gauntlet.controls/1\"\n");
    let rec = make_receipt(
        &repo.head,
        Tier::G1KnownAnswers,
        "GATE-TEST-01",
        "pass",
        vec![],
        vec![entry_ok("ka:wheel", "0.9207", "1")],
    );
    let (_, hash) = repo.write_receipt(&rec);
    let err = record_gate(&base_opts(&repo, "GATE-TEST-01", hash, "session:s2")).unwrap_err();
    assert_eq!(err.harness_reason, Some(HarnessReason::PassWithoutControl));
    assert_eq!(fs::read(&gates_path).unwrap(), orig);
}

#[test]
fn refuse_tampered_receipt() {
    let repo = ScratchRepo::new("tamper", &["session:s1"]);
    let gates_path = repo.write_gates(&std_gate_toml(
        "GATE-TEST-01",
        "known-answer",
        CLAIM_A,
        "\"NC-0001\", \"NC-0002\"",
    ));
    let orig = fs::read(&gates_path).unwrap();
    repo.write_controls(std_controls_two());
    let rec = make_receipt(
        &repo.head,
        Tier::G1KnownAnswers,
        "GATE-TEST-01",
        "pass",
        vec![
            ctrl("NC-0001", "GATE-TEST-01", ControlRunOutcome::Caught),
            ctrl("NC-0002", "GATE-TEST-01", ControlRunOutcome::Caught),
        ],
        vec![entry_ok("ka:wheel", "0.9207", "1")],
    );
    let (path, hash) = repo.write_receipt(&rec);
    let mut bytes = fs::read(&path).unwrap();
    bytes.push(b' ');
    fs::write(&path, &bytes).unwrap();

    let err = record_gate(&base_opts(&repo, "GATE-TEST-01", hash, "session:s2")).unwrap_err();
    assert_eq!(err.harness_reason, Some(HarnessReason::ReceiptHashMismatch));
    assert_eq!(fs::read(&gates_path).unwrap(), orig);
}

#[test]
fn refuse_dirty_tree() {
    let repo = ScratchRepo::new("dirty", &["session:s1"]);
    repo.write_gates(&std_gate_toml(
        "GATE-TEST-01",
        "known-answer",
        CLAIM_A,
        "\"NC-0001\", \"NC-0002\"",
    ));
    repo.write_controls(std_controls_two());
    let mut rec = make_receipt(
        &repo.head,
        Tier::G1KnownAnswers,
        "GATE-TEST-01",
        "pass",
        vec![
            ctrl("NC-0001", "GATE-TEST-01", ControlRunOutcome::Caught),
            ctrl("NC-0002", "GATE-TEST-01", ControlRunOutcome::Caught),
        ],
        vec![entry_ok("ka:wheel", "0.9207", "1")],
    );
    rec.dirty = true;
    let (_, hash) = repo.write_receipt(&rec);
    let err = record_gate(&base_opts(&repo, "GATE-TEST-01", hash, "session:s2")).unwrap_err();
    assert_eq!(err.harness_reason, Some(HarnessReason::ReceiptDirtyTree));
}

#[test]
fn refuse_unknown_commit() {
    let repo = ScratchRepo::new("unknown_commit", &["session:s1"]);
    repo.write_gates(&std_gate_toml(
        "GATE-TEST-01",
        "known-answer",
        CLAIM_A,
        "\"NC-0001\", \"NC-0002\"",
    ));
    repo.write_controls(std_controls_two());
    let rec = make_receipt(
        "deadbeefdeadbeefdeadbeefdeadbeefdeadbeef",
        Tier::G1KnownAnswers,
        "GATE-TEST-01",
        "pass",
        vec![
            ctrl("NC-0001", "GATE-TEST-01", ControlRunOutcome::Caught),
            ctrl("NC-0002", "GATE-TEST-01", ControlRunOutcome::Caught),
        ],
        vec![entry_ok("ka:wheel", "0.9207", "1")],
    );
    let (_, hash) = repo.write_receipt(&rec);
    let err = record_gate(&base_opts(&repo, "GATE-TEST-01", hash, "session:s2")).unwrap_err();
    assert_eq!(
        err.harness_reason,
        Some(HarnessReason::ReceiptCommitUnknown)
    );
}

#[test]
fn refuse_unresolved_evidence() {
    let repo = ScratchRepo::new("unresolved_ev", &["session:s1"]);
    repo.write_gates(&std_gate_toml(
        "GATE-TEST-01",
        "known-answer",
        CLAIM_A,
        "\"NC-0001\", \"NC-0002\"",
    ));
    repo.write_controls(std_controls_two());
    let rec = make_receipt(
        &repo.head,
        Tier::G1KnownAnswers,
        "GATE-TEST-01",
        "pass",
        vec![
            ctrl("NC-0001", "GATE-TEST-01", ControlRunOutcome::Caught),
            ctrl("NC-0002", "GATE-TEST-01", ControlRunOutcome::Caught),
        ],
        vec![entry_ok("ka:wheel", "0.9207", "1")],
    );
    let (_, hash) = repo.write_receipt(&rec);

    // Dangling node.
    let mut opts_dangling = base_opts(&repo, "GATE-TEST-01", hash.clone(), "session:s2");
    opts_dangling.evidence = vec!["ev:dangling-999".to_owned()];
    opts_dangling.live_evidence = Some(BTreeSet::from(["ev:live-001".to_owned()]));
    let err_d = record_gate(&opts_dangling).unwrap_err();
    assert_eq!(
        err_d.harness_reason,
        Some(HarnessReason::EvidenceUnresolved)
    );
    assert!(err_d.detail.contains("ev:dangling-999"));

    // Tombstoned node.
    let mut opts_tomb = base_opts(&repo, "GATE-TEST-01", hash, "session:s2");
    opts_tomb.evidence = vec!["ev:tomb-42".to_owned()];
    opts_tomb.tombstoned_nodes.insert("ev:tomb-42".to_owned());
    let err_t = record_gate(&opts_tomb).unwrap_err();
    assert_eq!(
        err_t.harness_reason,
        Some(HarnessReason::EvidenceUnresolved)
    );
    assert!(err_t.detail.contains("ev:tomb-42"));
}

#[test]
fn refuse_reduced_count() {
    let repo = ScratchRepo::new("reduced", &["session:s1"]);
    repo.write_gates(&std_gate_toml(
        "GATE-TEST-01",
        "known-answer",
        CLAIM_A,
        "\"NC-0001\", \"NC-0002\"",
    ));
    repo.write_controls(std_controls_two());
    let mut ent = entry_ok("ka:wheel", "0.9207", "1");
    ent.reduced = Some(ReducedCount {
        count: 1_000,
        target: 10_000,
    });
    let mut rec = make_receipt(
        &repo.head,
        Tier::G1KnownAnswers,
        "GATE-TEST-01",
        "pass",
        vec![
            ctrl("NC-0001", "GATE-TEST-01", ControlRunOutcome::Caught),
            ctrl("NC-0002", "GATE-TEST-01", ControlRunOutcome::Caught),
        ],
        vec![ent],
    );
    rec.gates[0].reduced = Some(true);
    let (_, hash) = repo.write_receipt(&rec);
    let err = record_gate(&base_opts(&repo, "GATE-TEST-01", hash, "session:s2")).unwrap_err();
    assert_eq!(
        err.harness_reason,
        Some(HarnessReason::ReducedNotRecordable)
    );
}

#[test]
fn lock_serializes_recorders() {
    let repo = ScratchRepo::new("lock_serial", &["session:s1"]);
    let gates_path = repo.write_gates(&std_gate_toml(
        "GATE-TEST-01",
        "known-answer",
        CLAIM_A,
        "\"NC-0001\", \"NC-0002\"",
    ));
    repo.write_controls(std_controls_two());
    let rec = make_receipt(
        &repo.head,
        Tier::G1KnownAnswers,
        "GATE-TEST-01",
        "pass",
        vec![
            ctrl("NC-0001", "GATE-TEST-01", ControlRunOutcome::Caught),
            ctrl("NC-0002", "GATE-TEST-01", ControlRunOutcome::Caught),
        ],
        vec![entry_ok("ka:wheel", "0.9207", "1")],
    );
    let (_, hash) = repo.write_receipt(&rec);

    // 1. Live contention: hold the lock on a worker thread, launch recorder concurrently,
    // release after 120ms, verify the recorder waits and writes a valid register.
    let barrier = Arc::new(Barrier::new(2));
    let b2 = Arc::clone(&barrier);
    let g2 = gates_path.clone();
    let holder = thread::spawn(move || {
        let _lk = RegistryLock::acquire(&g2, "session:holder", Duration::from_secs(2))
            .expect("holder acquires lock");
        b2.wait();
        thread::sleep(Duration::from_millis(120));
    });
    barrier.wait();
    let out = record_gate(&base_opts(
        &repo,
        "GATE-TEST-01",
        hash.clone(),
        "session:s2",
    ))
    .expect("second recorder waited on lock and succeeded");
    holder.join().expect("holder joined");
    assert_eq!(out.part.status, Status::Pass);

    // 2. Planted stale lock (dead PID) is reported with its holder and NEVER silently broken.
    let lock_path = RegistryLock::lock_path_for(&gates_path);
    fs::write(
        &lock_path,
        "pid=99999999\nsession=session:ghost\nstarted_unix=1700000000\n",
    )
    .expect("plant stale lock");
    let err = record_gate(&base_opts(&repo, "GATE-TEST-01", hash, "session:s2")).unwrap_err();
    assert!(
        err.detail.contains("stale") && err.detail.contains("session:ghost"),
        "expected stale lock naming holder, got: {err}"
    );
    assert!(
        lock_path.is_file(),
        "stale lock must never be silently broken"
    );
}

#[test]
fn fail_is_recorded() {
    let repo = ScratchRepo::new("fail_rec", &["session:s1"]);
    repo.write_gates(&std_gate_toml(
        "GATE-TEST-01",
        "known-answer",
        CLAIM_A,
        "\"NC-0001\"",
    ));
    let mut ent = entry_ok("ka:wheel", "1.497", "1");
    ent.outcome = "fail".to_owned();
    ent.first_diff = Some("spectral_radius=1.497 > 0.95".to_owned());
    let mut rec = make_receipt(
        &repo.head,
        Tier::G1KnownAnswers,
        "GATE-TEST-01",
        "fail",
        vec![],
        vec![ent],
    );
    rec.counts.pass = 0;
    rec.counts.fail = 1;
    let (_, hash) = repo.write_receipt(&rec);
    let out = record_gate(&base_opts(&repo, "GATE-TEST-01", hash, "session:s2"))
        .expect("fail is recorded");
    assert_eq!(out.part.status, Status::Fail);
    assert_eq!(out.part.measured, "spectral_radius=1.497 > 0.95");
}

#[test]
fn killed_is_derived_not_written() {
    let repo = ScratchRepo::new("killed_derived", &["session:s1"]);
    let adr_rel = "docs/adr/ADR-KILL-X-M2.md";
    fs::write(repo.root.join(adr_rel), "# ADR KILL-X M2\n").unwrap();
    let reg_toml = format!(
        r#"schema = "frob.gates/1"

[[gate]]
id = "GATE-Y"
kind = "convergence"
claim = "known-answer"
claim_id = "{CLAIM_A}"
section = "38.15"
threshold = "error <= 0.01"
milestone = "M2"
tier = ["G1:host"]
controls = ["NC-0001"]

[[gate.part]]
milestone = "M2"
status = "not-run"
measured = ""
evidence = []
recorded = {{ date = "", session = "", implemented_by = "" }}

[[gate]]
id = "KILL-X"
kind = "kill"
section = "38.15"
trigger = "GATE-Y fails"
fallback = "cpu-fallback"
decided = "M2"
mode = "exit"
reads = ["GATE-Y"]
decision = {{ milestone = "M2", outcome = "fired", adr = "{adr_rel}", session = "session:s2", date = "2026-11-25", recorded_by = "session:s2" }}
"#
    );
    let gates_path = repo.write_gates(&reg_toml);
    let mut ent = entry_ok("ka:y", "0.25", "m");
    ent.outcome = "fail".to_owned();
    ent.first_diff = Some("error=0.25 m".to_owned());
    let mut rec = make_receipt(
        &repo.head,
        Tier::G1KnownAnswers,
        "GATE-Y",
        "fail",
        vec![],
        vec![ent],
    );
    rec.counts.pass = 0;
    rec.counts.fail = 1;
    let (_, hash) = repo.write_receipt(&rec);
    let mut opts = base_opts(&repo, "GATE-Y", hash, "session:s2");
    opts.part_milestone = Some("M2".to_owned());
    let out = record_gate(&opts).expect("records fail on GATE-Y");
    assert_eq!(out.part.status, Status::Fail);

    // Stored file never writes status = "killed".
    let stored = fs::read_to_string(&gates_path).unwrap();
    assert!(
        !stored.contains("status = \"killed\""),
        "recorder must never write killed: {stored}"
    );

    let parsed = parse_register_bytes(stored.as_bytes(), "gates.toml").unwrap();
    let derived = apply_decisions(&parsed);
    let gy = derived.get("GATE-Y").unwrap();
    assert_eq!(gy.derived_status(), Status::Killed);
    assert_eq!(gy.parts[0].detail.as_deref(), Some("KILL-X: cpu-fallback"));
    let printed = render_register_printout(&parsed);
    assert!(printed.contains("killed: fallback adopted (cpu-fallback)"));

    // With outcome = "not-fired", apply_decisions keeps GATE-Y as `fail`.
    let mut not_fired = parsed;
    not_fired.get_mut("KILL-X").unwrap().kill.decisions[0].outcome =
        frob_conformance::register::KillOutcome::NotFired;
    let derived_nf = apply_decisions(&not_fired);
    assert_eq!(
        derived_nf.get("GATE-Y").unwrap().derived_status(),
        Status::Fail
    );
}

#[test]
fn no_claim_only_reality() {
    let repo = ScratchRepo::new("no_claim", &["session:s1"]);
    let toml = format!(
        r#"schema = "frob.gates/1"

[[gate]]
id = "GATE-REAL-01"
kind = "convergence"
claim = "reality"
claim_id = "{CLAIM_A}"
section = "38.15"
threshold = "cell band <= 5%"
milestone = "M0"
tier = ["G1:host"]
controls = []

[[gate.part]]
milestone = "M0"
status = "not-run"
measured = ""
evidence = []
recorded = {{ date = "", session = "", implemented_by = "" }}

[[gate]]
id = "GATE-PERF-01"
kind = "convergence"
claim = "performance"
claim_id = "{CLAIM_B}"
section = "38.15"
threshold = ">= 1000 steps/s"
milestone = "M0"
tier = ["G1:host"]
controls = ["NC-0001"]

[[gate.part]]
milestone = "M0"
status = "not-run"
measured = ""
evidence = []
recorded = {{ date = "", session = "", implemented_by = "" }}
"#
    );
    let gates_path = repo.write_gates(&toml);

    // Reality gate accepts `no-claim: no-dataset-admitted` with no control in receipt.
    let rec_real = make_receipt(
        &repo.head,
        Tier::G1KnownAnswers,
        "GATE-REAL-01",
        "no-claim: no-dataset-admitted",
        vec![],
        vec![],
    );
    let (_, h_real) = repo.write_receipt(&rec_real);
    let out = record_gate(&base_opts(&repo, "GATE-REAL-01", h_real, "session:s2"))
        .expect("reality gate records no-claim");
    assert_eq!(out.part.status, Status::NoClaim);
    assert_eq!(out.part.detail.as_deref(), Some("no-dataset-admitted"));

    // Performance gate refuses `no-claim` with `no-claim-on-non-reality` and writes nothing.
    let before = fs::read(&gates_path).unwrap();
    let rec_perf = make_receipt(
        &repo.head,
        Tier::G1KnownAnswers,
        "GATE-PERF-01",
        "no-claim: no-dataset-admitted",
        vec![],
        vec![],
    );
    let (_, h_perf) = repo.write_receipt(&rec_perf);
    let err = record_gate(&base_opts(&repo, "GATE-PERF-01", h_perf, "session:s2")).unwrap_err();
    assert_eq!(err.harness_reason, Some(HarnessReason::NoClaimOnNonReality));
    assert_eq!(fs::read(&gates_path).unwrap(), before);
}

#[test]
fn refuse_stack_dev() {
    let repo = ScratchRepo::new("stack_dev", &["session:s1"]);
    repo.write_gates(&std_gate_toml(
        "GATE-TEST-01",
        "known-answer",
        CLAIM_A,
        "\"NC-0001\", \"NC-0002\"",
    ));
    repo.write_controls(std_controls_two());

    for line in ["pass", "fail"] {
        let mut rec = make_receipt(
            &repo.head,
            Tier::G1KnownAnswers,
            "GATE-TEST-01",
            line,
            vec![
                ctrl("NC-0001", "GATE-TEST-01", ControlRunOutcome::Caught),
                ctrl("NC-0002", "GATE-TEST-01", ControlRunOutcome::Caught),
            ],
            vec![entry_ok("ka:wheel", "0.9207", "1")],
        );
        rec.stack_dev = true;
        let (_, hash) = repo.write_receipt(&rec);
        let err = record_gate(&base_opts(&repo, "GATE-TEST-01", hash, "session:s2")).unwrap_err();
        assert_eq!(
            err.harness_reason,
            Some(HarnessReason::ReceiptStackDev),
            "stack_dev=true must be refused for `{line}`"
        );
    }
}

#[test]
fn refuse_dev_build_identity() {
    let repo = ScratchRepo::new("dev_bi", &["session:s1"]);
    let toml = format!(
        r#"schema = "frob.gates/1"

[[gate]]
id = "GATE-PERF"
kind = "convergence"
claim = "performance"
claim_id = "{CLAIM_A}"
section = "38.15"
threshold = ">= 1000/s"
milestone = "M0"
tier = ["G1:host"]
controls = ["NC-0001"]

[[gate.part]]
milestone = "M0"
status = "not-run"
measured = ""
evidence = []
recorded = {{ date = "", session = "", implemented_by = "" }}

[[gate]]
id = "GATE-PAR"
kind = "convergence"
claim = "parity"
claim_id = "{CLAIM_B}"
section = "38.15"
threshold = "max diff <= 1e-6"
milestone = "M0"
tier = ["G2:host"]
controls = ["NC-0001"]

[[gate.part]]
milestone = "M0"
status = "not-run"
measured = ""
evidence = []
recorded = {{ date = "", session = "", implemented_by = "" }}

[[gate]]
id = "GATE-KA"
kind = "convergence"
claim = "known-answer"
claim_id = "{CLAIM_C}"
section = "38.15"
threshold = "rho <= 0.95"
milestone = "M0"
tier = ["G1:host"]
controls = ["NC-0001"]

[[gate.part]]
milestone = "M0"
status = "not-run"
measured = ""
evidence = []
recorded = {{ date = "", session = "", implemented_by = "" }}
"#
    );
    repo.write_gates(&toml);
    repo.write_controls(
        r#"schema = "frob.gauntlet.controls/1"

[[control]]
id = "NC-0001"
seed = "GATE-KA"
gate = ["GATE-PERF", "GATE-PAR", "GATE-KA"]
kind = "planted-input"
target = "frob-conformance::test::a"
defect = "planted defect"
expect = "fails with code 1"
fixture = "fixtures/a.toml"
due = "M0"
state = "runnable"
"#,
    );

    // Under `dev-salt`: performance and parity refused; known-answer accepted.
    for (gid, tier, expect_ok) in [
        ("GATE-PERF", Tier::G1KnownAnswers, false),
        ("GATE-PAR", Tier::G2Parity, false),
        ("GATE-KA", Tier::G1KnownAnswers, true),
    ] {
        let mut rec = make_receipt(
            &repo.head,
            tier,
            gid,
            "pass",
            vec![ctrl("NC-0001", gid, ControlRunOutcome::Caught)],
            vec![entry_ok("e:1", "0.5", "1")],
        );
        rec.build_identity = "dev-salt".to_owned();
        let (_, hash) = repo.write_receipt(&rec);
        let res = record_gate(&base_opts(&repo, gid, hash, "session:s2"));
        if expect_ok {
            assert_eq!(res.expect("KA under dev-salt").part.status, Status::Pass);
        } else {
            assert_eq!(
                res.unwrap_err().harness_reason,
                Some(HarnessReason::DevBuildIdentity)
            );
        }
    }

    // Under `depgraph-receipt`: all three record.
    for (gid, tier) in [
        ("GATE-PERF", Tier::G1KnownAnswers),
        ("GATE-PAR", Tier::G2Parity),
        ("GATE-KA", Tier::G1KnownAnswers),
    ] {
        let rec = make_receipt(
            &repo.head,
            tier,
            gid,
            "pass",
            vec![ctrl("NC-0001", gid, ControlRunOutcome::Caught)],
            vec![entry_ok("e:2", "0.4", "1")],
        );
        let (_, hash) = repo.write_receipt(&rec);
        let out = record_gate(&base_opts(&repo, gid, hash, "session:s2"))
            .expect("depgraph-receipt allows all three");
        assert_eq!(out.part.status, Status::Pass);
    }
}

#[test]
fn receipt_committed_and_logged() {
    let repo = ScratchRepo::new("committed_log", &["session:s1"]);
    repo.write_gates(&std_gate_toml(
        "GATE-TEST-01",
        "known-answer",
        CLAIM_A,
        "\"NC-0001\", \"NC-0002\"",
    ));
    repo.write_controls(std_controls_two());
    let rec = make_receipt(
        &repo.head,
        Tier::G1KnownAnswers,
        "GATE-TEST-01",
        "pass",
        vec![
            ctrl("NC-0001", "GATE-TEST-01", ControlRunOutcome::Caught),
            ctrl("NC-0002", "GATE-TEST-01", ControlRunOutcome::Caught),
        ],
        vec![entry_ok("ka:wheel", "0.9207", "1")],
    );
    let (src_path, hash) = repo.write_receipt(&rec);
    let out = record_gate(&base_opts(
        &repo,
        "GATE-TEST-01",
        hash.clone(),
        "session:s2",
    ))
    .expect("records pass");

    let committed = repo.root.join(format!("receipts/gates/{hash}.json"));
    assert_eq!(fs::read(&committed).unwrap(), fs::read(&src_path).unwrap());

    let log_line = fs::read_to_string(repo.root.join("docs/delivery/logs/BD-TEST.ndjson")).unwrap();
    let doc = json::admit(log_line.trim().as_bytes(), &Limits::default()).expect("valid JSON");
    assert_eq!(
        doc.value().get("kind").and_then(|v| v.as_str()),
        Some("gate-record")
    );
    assert_eq!(
        doc.value().get("receipt").and_then(|v| v.as_str()),
        Some(out.part.receipt.as_str())
    );
}

#[test]
fn receipt_set_two_tiers() {
    let repo = ScratchRepo::new("two_tiers", &["session:s1"]);
    repo.write_gates(&std_gate_toml(
        "GATE-TEST-01",
        "known-answer",
        CLAIM_A,
        "\"NC-0001\", \"NC-0002\"",
    ));
    repo.write_controls(std_controls_two());

    let r_g1 = make_receipt(
        &repo.head,
        Tier::G1KnownAnswers,
        "GATE-TEST-01",
        "pass",
        vec![ctrl("NC-0001", "GATE-TEST-01", ControlRunOutcome::Caught)],
        vec![entry_ok("ka:law", "0.9207", "1")],
    );
    let r_g4 = make_receipt(
        &repo.head,
        Tier::G4Chaos,
        "GATE-TEST-01",
        "pass",
        vec![ctrl("NC-0002", "GATE-TEST-01", ControlRunOutcome::Caught)],
        vec![entry_ok("chaos:rut", "0.002", "m")],
    );
    let (_, h_g1) = repo.write_receipt(&r_g1);
    let (_, h_g4) = repo.write_receipt(&r_g4);

    // Pass G4 before G1; recorder sorts members into tier order (G1, G4).
    let mut opts = base_opts(&repo, "GATE-TEST-01", h_g4.clone(), "session:s2");
    opts.receipt_hashes = vec![h_g4.clone(), h_g1.clone()];
    let committed_dir = repo.root.join("receipts/gates");
    let blocked = committed_dir.join(format!("{h_g4}.json"));
    fs::create_dir(&blocked).unwrap();
    let err = record_gate(&opts).expect_err("second member destination is a directory");
    assert_eq!(err.reason, "io-error");
    assert!(err.detail.contains(&h_g4), "{err}");
    assert!(blocked.is_dir());
    assert!(!committed_dir.join(format!("{h_g1}.json")).exists());
    assert_eq!(fs::read_dir(&committed_dir).unwrap().count(), 1);
    fs::rename(&blocked, repo.root.join("planted-receipt-directory")).unwrap();

    let out = record_gate(&opts).expect("two-tier receipt set records");
    assert_eq!(out.part.status, Status::Pass);
    assert_eq!(out.part.measured, "0.9207 (1); 0.002 (m)");

    let set_path = repo
        .root
        .join(format!("receipts/gates/{}.json", out.part.receipt));
    let set_bytes = fs::read(&set_path).expect("receipt set committed");
    assert_eq!(hash_blob(&set_bytes).to_hex(), out.part.receipt);
    let doc = json::admit(&set_bytes, &Limits::default()).expect("set json");
    assert_eq!(
        doc.value().get("schema").and_then(|v| v.as_str()),
        Some(RECEIPT_SET_SCHEMA)
    );
    let members: Vec<&str> = doc
        .value()
        .get("members")
        .and_then(|v| v.as_array())
        .unwrap()
        .iter()
        .filter_map(|v| v.as_str())
        .collect();
    assert_eq!(members, vec![h_g1.as_str(), h_g4.as_str()]);

    // One member failing -> part records fail.
    let mut r_g4_fail = r_g4.clone();
    r_g4_fail.gates[0].line = "fail".to_owned();
    r_g4_fail.entries[0].outcome = "fail".to_owned();
    r_g4_fail.entries[0].first_diff = Some("rut_depth=0.5 m".to_owned());
    let (_, h_g4_fail) = repo.write_receipt(&r_g4_fail);
    let mut fail_opts = base_opts(&repo, "GATE-TEST-01", h_g1.clone(), "session:s2");
    fail_opts.receipt_hashes = vec![h_g1.clone(), h_g4_fail];
    assert_eq!(record_gate(&fail_opts).unwrap().part.status, Status::Fail);

    // Two receipts of one tier -> receipt-set-invalid.
    let mut dup_opts = base_opts(&repo, "GATE-TEST-01", h_g1.clone(), "session:s2");
    dup_opts.receipt_hashes = vec![h_g1.clone(), h_g1.clone()];
    assert_eq!(
        record_gate(&dup_opts).unwrap_err().harness_reason,
        Some(HarnessReason::ReceiptSetInvalid)
    );

    // Two commits -> receipt-set-invalid.
    let head2 = repo.commit_extra("c2.txt", "v2\n", "session:s1");
    let r_g4_c2 = make_receipt(
        &head2,
        Tier::G4Chaos,
        "GATE-TEST-01",
        "pass",
        vec![ctrl("NC-0002", "GATE-TEST-01", ControlRunOutcome::Caught)],
        vec![entry_ok("chaos:rut", "0.002", "m")],
    );
    let (_, h_g4_c2) = repo.write_receipt(&r_g4_c2);
    let mut diff_c = base_opts(&repo, "GATE-TEST-01", h_g1, "session:s2");
    diff_c.receipt_hashes.push(h_g4_c2);
    assert_eq!(
        record_gate(&diff_c).unwrap_err().harness_reason,
        Some(HarnessReason::ReceiptSetInvalid)
    );
}

#[test]
fn rerecord_keeps_rows() {
    let repo = ScratchRepo::new("rerecord", &["session:s1"]);
    repo.write_gates(&std_gate_toml(
        "GATE-TEST-01",
        "known-answer",
        CLAIM_A,
        "\"NC-0001\", \"NC-0002\"",
    ));
    repo.write_controls(std_controls_two());

    let r_g1 = make_receipt(
        &repo.head,
        Tier::G1KnownAnswers,
        "GATE-TEST-01",
        "pass",
        vec![ctrl("NC-0001", "GATE-TEST-01", ControlRunOutcome::Caught)],
        vec![entry_ok("ka:law", "0.9207", "1")],
    );
    let r_g4 = make_receipt(
        &repo.head,
        Tier::G4Chaos,
        "GATE-TEST-01",
        "pass",
        vec![ctrl("NC-0002", "GATE-TEST-01", ControlRunOutcome::Caught)],
        vec![entry_ok("chaos:rut", "0.002", "m")],
    );
    let (_, h_g1) = repo.write_receipt(&r_g1);
    let (_, h_g4) = repo.write_receipt(&r_g4);
    let mut opts1 = base_opts(&repo, "GATE-TEST-01", h_g1.clone(), "session:s2");
    opts1.receipt_hashes = vec![h_g1.clone(), h_g4.clone()];
    let out1 = record_gate(&opts1).expect("initial two-tier recording");

    // Re-record from newer G4 receipt alone -> refused with part-rows-dropped naming ka:law.
    let head2 = repo.commit_extra("lift.txt", "lift\n", "session:s1");
    let r_g4_new = make_receipt(
        &head2,
        Tier::G4Chaos,
        "GATE-TEST-01",
        "pass",
        vec![
            ctrl("NC-0001", "GATE-TEST-01", ControlRunOutcome::Caught),
            ctrl("NC-0002", "GATE-TEST-01", ControlRunOutcome::Caught),
        ],
        vec![entry_ok("chaos:rut", "0.001", "m")],
    );
    let (_, h_g4_new) = repo.write_receipt(&r_g4_new);
    let err = record_gate(&base_opts(
        &repo,
        "GATE-TEST-01",
        h_g4_new.clone(),
        "session:s2",
    ))
    .unwrap_err();
    assert_eq!(err.harness_reason, Some(HarnessReason::PartRowsDropped));
    assert!(err.detail.contains("ka:law"));

    // Re-record from newer G1 + G4 -> part replaced, earlier receipts remain under receipts/gates/.
    let r_g1_new = make_receipt(
        &head2,
        Tier::G1KnownAnswers,
        "GATE-TEST-01",
        "pass",
        vec![ctrl("NC-0001", "GATE-TEST-01", ControlRunOutcome::Caught)],
        vec![entry_ok("ka:law", "0.9100", "1")],
    );
    let (_, h_g1_new) = repo.write_receipt(&r_g1_new);
    let mut opts2 = base_opts(&repo, "GATE-TEST-01", h_g1_new, "session:s2");
    opts2.receipt_hashes.push(h_g4_new);
    let out2 = record_gate(&opts2).expect("re-recording keeping all rows succeeds");
    assert_eq!(out2.part.measured, "0.9100 (1); 0.001 (m)");
    assert_ne!(out2.part.receipt, out1.part.receipt);
    assert!(
        repo.root
            .join(format!("receipts/gates/{h_g1}.json"))
            .is_file()
    );
    assert!(
        repo.root
            .join(format!("receipts/gates/{h_g4}.json"))
            .is_file()
    );
    assert!(
        repo.root
            .join(format!("receipts/gates/{}.json", out1.part.receipt))
            .is_file()
    );
}

#[test]
fn mode_word_dispatch() {
    let repo = ScratchRepo::new("mode_word", &["session:s1"]);
    let gates_path = repo.write_gates(&std_gate_toml(
        "GATE-TEST-01",
        "known-answer",
        CLAIM_A,
        "\"NC-0001\"",
    ));
    let orig = fs::read(&gates_path).unwrap();

    // Locate compiled example binary if built beside integration tests; otherwise verify via lib dispatch.
    if let Ok(exe) = std::env::current_exe()
        && let Some(examples_dir) = exe
            .parent()
            .and_then(Path::parent)
            .map(|p| p.join("examples"))
    {
        let bin = examples_dir.join("record_gate");
        if bin.is_file() {
            let out = Command::new(&bin)
                .args(["frobnicate", "--gate", "GATE-TEST-01", "--registry"])
                .arg(&gates_path)
                .output()
                .expect("run record_gate");
            assert!(
                !out.status.success(),
                "unknown mode word must exit non-zero"
            );
            let stderr = String::from_utf8_lossy(&out.stderr);
            assert!(stderr.contains("reaffirm") && stderr.contains("frobnicate"));
            assert_eq!(fs::read(&gates_path).unwrap(), orig);
            return;
        }
    }
    // Always verify reaffirm shared-lock path runs cleanly.
    let lock_path = frob_conformance::register::record::run_reaffirm_lock_dispatch(
        &gates_path,
        "session:reaffirm",
        Duration::from_secs(2),
    )
    .expect("reaffirm acquires and releases shared lock");
    assert!(!lock_path.exists(), "lock removed on exit");
    assert_eq!(fs::read(&gates_path).unwrap(), orig);
}

#[test]
fn measured_copied_from_receipt() {
    let repo = ScratchRepo::new("measured_copy", &["session:s1"]);
    repo.write_gates(&std_gate_toml(
        "GATE-TEST-01",
        "known-answer",
        CLAIM_A,
        "\"NC-0001\", \"NC-0002\"",
    ));
    repo.write_controls(std_controls_two());
    let rec = make_receipt(
        &repo.head,
        Tier::G1KnownAnswers,
        "GATE-TEST-01",
        "pass",
        vec![
            ctrl("NC-0001", "GATE-TEST-01", ControlRunOutcome::Caught),
            ctrl("NC-0002", "GATE-TEST-01", ControlRunOutcome::Caught),
        ],
        vec![entry_ok("ka:wheel", "0.9207", "1")],
    );
    let (_, hash) = repo.write_receipt(&rec);
    let out = record_gate(&base_opts(&repo, "GATE-TEST-01", hash, "session:s2")).unwrap();
    assert_eq!(out.part.measured, "0.9207 (1)");
    assert_ne!(out.part.measured, "spectral radius <= 0.95");
}

#[test]
fn hook_refusal_stops_write() {
    let _serial = HOOK_TEST_LOCK
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    fn failing_hook(_ctx: &RecordHookContext<'_>) -> Result<(), RecordHookRefusal> {
        Err(RecordHookRefusal::new(
            HarnessReason::ClaimEvidenceMismatch,
            "synthetic hook refused claim kind",
            "Cite evidence matching the gate claim kind",
        ))
    }

    let repo = ScratchRepo::new("hook_refusal", &["session:s1"]);
    let gates_path = repo.write_gates(&std_gate_toml(
        "GATE-TEST-01",
        "known-answer",
        CLAIM_A,
        "\"NC-0001\", \"NC-0002\"",
    ));
    let orig = fs::read(&gates_path).unwrap();
    repo.write_controls(std_controls_two());
    let rec = make_receipt(
        &repo.head,
        Tier::G1KnownAnswers,
        "GATE-TEST-01",
        "pass",
        vec![
            ctrl("NC-0001", "GATE-TEST-01", ControlRunOutcome::Caught),
            ctrl("NC-0002", "GATE-TEST-01", ControlRunOutcome::Caught),
        ],
        vec![entry_ok("ka:wheel", "0.9207", "1")],
    );
    let (_, hash) = repo.write_receipt(&rec);

    register_check("test-claim-mismatch", failing_hook);
    let mut opts = base_opts(&repo, "GATE-TEST-01", hash.clone(), "session:s2");
    opts.skip_global_checks = false;
    let err = record_gate(&opts).unwrap_err();
    unregister_check("test-claim-mismatch");

    assert_eq!(
        err.harness_reason,
        Some(HarnessReason::ClaimEvidenceMismatch)
    );
    assert!(
        err.first_line()
            .starts_with("reason=claim-evidence-mismatch")
    );
    assert_eq!(fs::read(&gates_path).unwrap(), orig);
    assert!(!repo.root.join(CONTROL_HISTORY_PATH).exists());
    assert!(!repo.root.join("docs/delivery/logs/BD-TEST.ndjson").exists());

    // With hook list empty, the exact same receipt records pass.
    let ok = record_gate(&opts).expect("empty hooks allow pass");
    assert_eq!(ok.part.status, Status::Pass);
}

#[test]
fn due_filter_not_yet_due_and_all_deferred_refused() {
    // Round-4 Review Note 1: controls due after part.milestone are reported not-yet-due;
    // a part whose listed controls are all deferred fails with pass-without-control.
    let repo = ScratchRepo::new("due_filter", &["session:s1"]);
    repo.write_gates(&std_gate_toml(
        "GATE-TEST-01",
        "known-answer",
        CLAIM_A,
        "\"NC-0001\", \"NC-0002\"",
    ));
    repo.write_controls(
        r#"schema = "frob.gauntlet.controls/1"

[[control]]
id = "NC-0001"
seed = "GATE-TEST-01"
gate = ["GATE-TEST-01"]
kind = "planted-input"
target = "frob-conformance::test::a"
defect = "planted defect A"
expect = "fails with code 1"
fixture = "fixtures/a.toml"
due = "M0"
state = "runnable"

[[control]]
id = "NC-0002"
seed = "GATE-TEST-01"
gate = ["GATE-TEST-01"]
kind = "planted-input"
target = "frob-conformance::test::b"
defect = "planted defect B"
expect = "fails with code 2"
fixture = "fixtures/b.toml"
due = "M2"
state = "runnable"
"#,
    );
    let rec = make_receipt(
        &repo.head,
        Tier::G1KnownAnswers,
        "GATE-TEST-01",
        "pass",
        vec![ctrl("NC-0001", "GATE-TEST-01", ControlRunOutcome::Caught)],
        vec![entry_ok("ka:wheel", "0.9207", "1")],
    );
    let (_, hash) = repo.write_receipt(&rec);
    let out = record_gate(&base_opts(
        &repo,
        "GATE-TEST-01",
        hash.clone(),
        "session:s2",
    ))
    .expect("M2 control not required at M0");
    assert_eq!(out.not_yet_due_controls, vec!["NC-0002".to_owned()]);

    // Now make BOTH controls due M2 -> M0 has zero due controls -> pass-without-control.
    let repo2 = ScratchRepo::new("all_deferred", &["session:s1"]);
    repo2.write_gates(&std_gate_toml(
        "GATE-TEST-01",
        "known-answer",
        CLAIM_A,
        "\"NC-0001\", \"NC-0002\"",
    ));
    repo2.write_controls(
        r#"schema = "frob.gauntlet.controls/1"

[[control]]
id = "NC-0001"
seed = "GATE-TEST-01"
gate = ["GATE-TEST-01"]
kind = "planted-input"
target = "frob-conformance::test::a"
defect = "planted defect A"
expect = "fails with code 1"
fixture = "fixtures/a.toml"
due = "M2"
state = "runnable"

[[control]]
id = "NC-0002"
seed = "GATE-TEST-01"
gate = ["GATE-TEST-01"]
kind = "planted-input"
target = "frob-conformance::test::b"
defect = "planted defect B"
expect = "fails with code 2"
fixture = "fixtures/b.toml"
due = "M3"
state = "runnable"
"#,
    );
    let rec2 = make_receipt(
        &repo2.head,
        Tier::G1KnownAnswers,
        "GATE-TEST-01",
        "pass",
        vec![],
        vec![entry_ok("ka:wheel", "0.9207", "1")],
    );
    let (_, hash2) = repo2.write_receipt(&rec2);
    let err = record_gate(&base_opts(&repo2, "GATE-TEST-01", hash2, "session:s2")).unwrap_err();
    assert_eq!(err.harness_reason, Some(HarnessReason::PassWithoutControl));
}

#[test]
fn refuse_pilot_scope_part() {
    // Round-4 Review Note 2(1): a part whose scope starts with `pilot:` is never recorded.
    let repo = ScratchRepo::new("pilot_scope", &["session:s1"]);
    let toml = format!(
        r#"schema = "frob.gates/1"

[[gate]]
id = "GATE-PILOT-01"
kind = "convergence"
claim = "known-answer"
claim_id = "{CLAIM_A}"
section = "38.15"
threshold = "error <= 0.01"
milestone = "M3"
tier = ["G1:host"]
controls = ["NC-0001"]

[[gate.part]]
milestone = "M3"
scope = "pilot: PILOT-PT-01"
status = "not-run"
measured = ""
evidence = []
recorded = {{ date = "", session = "", implemented_by = "" }}
"#
    );
    let gates_path = repo.write_gates(&toml);
    let orig = fs::read(&gates_path).unwrap();
    let rec = make_receipt(
        &repo.head,
        Tier::G1KnownAnswers,
        "GATE-PILOT-01",
        "pass",
        vec![ctrl("NC-0001", "GATE-PILOT-01", ControlRunOutcome::Caught)],
        vec![entry_ok("ka:p", "0.001", "1")],
    );
    let (_, hash) = repo.write_receipt(&rec);
    let mut opts = base_opts(&repo, "GATE-PILOT-01", hash, "session:s2");
    opts.part_milestone = Some("M3".to_owned());
    let err = record_gate(&opts).unwrap_err();
    assert_eq!(err.reason, "pilot-part-not-recordable");
    assert_eq!(fs::read(&gates_path).unwrap(), orig);
}

#[test]
fn readme_splice_preserves_prose_and_updates_status() {
    let repo = ScratchRepo::new("readme_splice", &["session:s1"]);
    repo.write_gates(&std_gate_toml(
        "GATE-TEST-01",
        "known-answer",
        CLAIM_A,
        "\"NC-0001\", \"NC-0002\"",
    ));
    let controls = repo.write_controls(std_controls_two());
    let controls_before = fs::read(&controls).unwrap();

    // Existing README lacking markers or with inline markers is refused atomically without clobbering.
    let rec1 = make_receipt(
        &repo.head,
        Tier::G1KnownAnswers,
        "GATE-TEST-01",
        "pass",
        vec![
            ctrl("NC-0001", "GATE-TEST-01", ControlRunOutcome::Caught),
            ctrl("NC-0002", "GATE-TEST-01", ControlRunOutcome::Caught),
        ],
        vec![entry_ok("ka:wheel", "0.9207", "1")],
    );
    let (_, hash1) = repo.write_receipt(&rec1);
    let gates_before = fs::read(repo.root.join("registries/gates.toml")).unwrap();
    for (name, bad_prose) in [
        (
            "README_NO_MARKERS.md",
            "# Hand-written prose\n\nNever overwrite me.\n",
        ),
        (
            "README_INLINE_MARKERS.md",
            "# Prose\n<!-- status:begin -->inline<!-- status:end -->\n",
        ),
        (
            "README_INLINE_END.md",
            "# Prose\n<!-- status:begin -->\nKeep this text. <!-- status:end -->\n",
        ),
        (
            "README_INLINE_BEGIN.md",
            "# Prose <!-- status:begin -->\n<!-- status:end -->\n",
        ),
    ] {
        let bad_readme = repo.root.join(name);
        fs::write(&bad_readme, bad_prose).unwrap();
        let mut bad_opts = base_opts(&repo, "GATE-TEST-01", hash1.clone(), "session:s2");
        bad_opts.readme_path = Some(bad_readme.clone());
        let err = record_gate(&bad_opts).unwrap_err();
        assert_eq!(err.reason, "readme-marker-missing");
        assert_eq!(fs::read_to_string(&bad_readme).unwrap(), bad_prose);
        assert_eq!(
            fs::read(repo.root.join("registries/gates.toml")).unwrap(),
            gates_before
        );
        assert_eq!(fs::read(&controls).unwrap(), controls_before);
        assert!(!repo.root.join(CONTROL_HISTORY_PATH).exists());
        assert_eq!(
            fs::read_dir(repo.root.join("docs/delivery/logs"))
                .unwrap()
                .count(),
            0
        );
        assert!(
            !repo
                .root
                .join(format!("receipts/gates/{hash1}.json"))
                .exists()
        );
    }

    // Valid README with status markers preserves header and footer prose verbatim.
    let readme_path = repo.root.join("README.md");
    fs::write(
        &readme_path,
        "# FrankenRobotics\n\nIntro prose.\n\n<!-- status:begin -->\nold\n<!-- status:end -->\n\nFooter prose.\n",
    )
    .unwrap();
    let assert_status_view = |text: &str, status: &str, measured: &str| {
        let gates = fs::read(repo.root.join("registries/gates.toml")).unwrap();
        let (_, after_begin) = text.split_once("<!-- status:begin -->\n").unwrap();
        let (actual, _) = after_begin.split_once("<!-- status:end -->").unwrap();
        assert_eq!(actual, readme_table(&gates).unwrap());
        let row = actual
            .lines()
            .find(|line| line.starts_with("| GATE-TEST-01 "))
            .expect("status row for the recorded gate");
        let cells: Vec<_> = row.split('|').skip(1).take(6).map(str::trim).collect();
        assert_eq!(
            cells,
            [
                "GATE-TEST-01",
                "known-answer",
                "M0",
                status,
                measured,
                "2026-11-25 (session:s2)",
            ]
        );
    };
    let mut opts1 = base_opts(&repo, "GATE-TEST-01", hash1, "session:s2");
    opts1.readme_path = Some(readme_path.clone());
    let out1 = record_gate(&opts1).expect("records pass and splices README");
    assert_eq!(out1.regenerated_files, vec![readme_path.clone()]);
    let after1 = fs::read_to_string(&readme_path).unwrap();
    assert!(after1.starts_with("# FrankenRobotics\n\nIntro prose.\n\n<!-- status:begin -->\n"));
    assert!(after1.ends_with("<!-- status:end -->\n\nFooter prose.\n"));
    assert_status_view(&after1, "pass", "0.9207 (1)");
    assert!(!after1.contains("\nold\n"));

    // Second recording with a failing receipt updates the status row without duplicating markers or touching prose.
    let mut fail_entry = entry_ok("ka:wheel", "0.9800", "1");
    fail_entry.outcome = "fail".to_owned();
    fail_entry.first_diff = Some("0.9800 > 0.95".to_owned());
    let mut rec2 = make_receipt(
        &repo.head,
        Tier::G1KnownAnswers,
        "GATE-TEST-01",
        "fail",
        vec![
            ctrl("NC-0001", "GATE-TEST-01", ControlRunOutcome::Caught),
            ctrl("NC-0002", "GATE-TEST-01", ControlRunOutcome::Caught),
        ],
        vec![fail_entry],
    );
    rec2.counts.pass = 0;
    rec2.counts.fail = 1;
    let (_, hash2) = repo.write_receipt(&rec2);
    let mut opts2 = base_opts(&repo, "GATE-TEST-01", hash2, "session:s2");
    opts2.readme_path = Some(readme_path.clone());
    let out2 = record_gate(&opts2).expect("updates README status block on re-record");
    assert_eq!(out2.part.status, Status::Fail);
    let after2 = fs::read_to_string(&readme_path).unwrap();
    assert!(after2.starts_with("# FrankenRobotics\n\nIntro prose.\n\n<!-- status:begin -->\n"));
    assert!(after2.ends_with("<!-- status:end -->\n\nFooter prose.\n"));
    assert_eq!(after2.matches("<!-- status:begin -->").count(), 1);
    assert_eq!(after2.matches("<!-- status:end -->").count(), 1);
    assert_status_view(&after2, "fail", "0.9800 > 0.95");
    assert!(!after2.contains("| pass |"));
}

#[test]
fn refusal_on_disk_validation_leaves_controls_and_receipts_untouched() {
    let repo = ScratchRepo::new("atomic_refusal", &["session:s1"]);
    let missing_hex = "9999999999999999999999999999999999999999999999999999999999999999";
    let two_gates = format!(
        r#"schema = "frob.gates/1"

[[gate]]
id = "GATE-TEST-01"
kind = "convergence"
claim = "known-answer"
claim_id = "{CLAIM_A}"
section = "38.15"
threshold = "spectral radius <= 0.95"
milestone = "M0"
tier = ["G1:host"]
controls = ["NC-0001", "NC-0002"]

[[gate.part]]
milestone = "M0"
status = "not-run"
measured = ""
evidence = []
recorded = {{ date = "", session = "", implemented_by = "" }}

[[gate]]
id = "GATE-BROKEN-02"
kind = "convergence"
claim = "known-answer"
claim_id = "{CLAIM_B}"
section = "38.15"
threshold = "err <= 0.01"
milestone = "M0"
tier = ["G1:host"]
controls = ["NC-0001"]

[[gate.part]]
milestone = "M0"
status = "pass"
measured = "0.005"
commit = "{}"
command = "g1 -- gate:GATE-BROKEN-02"
receipt = "{missing_hex}"
fingerprint = "host"
seed = "1"
evidence = ["{missing_hex}"]
recorded = {{ date = "2026-11-25", session = "session:s2", implemented_by = "session:s1" }}
"#,
        repo.head
    );
    let gates_path = repo.write_gates(&two_gates);
    let controls_path = repo.write_controls(std_controls_two());
    let orig_gates = fs::read(&gates_path).unwrap();
    let orig_controls = fs::read(&controls_path).unwrap();

    let rec = make_receipt(
        &repo.head,
        Tier::G1KnownAnswers,
        "GATE-TEST-01",
        "pass",
        vec![
            ctrl("NC-0001", "GATE-TEST-01", ControlRunOutcome::Caught),
            ctrl("NC-0002", "GATE-TEST-01", ControlRunOutcome::Caught),
        ],
        vec![entry_ok("ka:wheel", "0.9207", "1")],
    );
    let (_, hash) = repo.write_receipt(&rec);
    let err = record_gate(&base_opts(
        &repo,
        "GATE-TEST-01",
        hash.clone(),
        "session:s2",
    ))
    .expect_err("broken existing receipt on disk must refuse before any write");
    assert_eq!(err.harness_reason, Some(HarnessReason::EvidenceUnresolved));
    assert_eq!(fs::read(&gates_path).unwrap(), orig_gates);
    assert_eq!(fs::read(&controls_path).unwrap(), orig_controls);
    assert!(!repo.root.join(CONTROL_HISTORY_PATH).exists());
    assert!(
        !repo
            .root
            .join(format!("receipts/gates/{hash}.json"))
            .exists()
    );
}

#[test]
fn bare_and_comma_trailer_sessions_caught_by_self_certified() {
    let repo = ScratchRepo::new("comma_trailer", &["s1, session:s3"]);
    repo.write_gates(&std_gate_toml(
        "GATE-TEST-01",
        "known-answer",
        CLAIM_A,
        "\"NC-0001\", \"NC-0002\"",
    ));
    repo.write_controls(std_controls_two());
    let rec = make_receipt(
        &repo.head,
        Tier::G1KnownAnswers,
        "GATE-TEST-01",
        "pass",
        vec![
            ctrl("NC-0001", "GATE-TEST-01", ControlRunOutcome::Caught),
            ctrl("NC-0002", "GATE-TEST-01", ControlRunOutcome::Caught),
        ],
        vec![entry_ok("ka:wheel", "0.9207", "1")],
    );
    let (_, hash) = repo.write_receipt(&rec);
    for bad in ["s1", "session:s1", "s3", "session:s3"] {
        let err = record_gate(&base_opts(&repo, "GATE-TEST-01", hash.clone(), bad))
            .expect_err("bare or comma-trailer implementer must be refused");
        assert_eq!(err.harness_reason, Some(HarnessReason::SelfCertified));
    }
    let ok = record_gate(&base_opts(&repo, "GATE-TEST-01", hash, "session:s2"))
        .expect("distinct reviewer succeeds");
    assert_eq!(ok.part.recorded.implemented_by, "session:s1, session:s3");
}
