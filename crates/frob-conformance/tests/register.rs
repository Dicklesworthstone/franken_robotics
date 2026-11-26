#![deny(unsafe_code)]

//! Comprehensive unit, property, and negative-control test suite for `registries/gates.toml`
//! (`N40-01`, `ADR-GAUNT-04`, `N38-REF`).
//!
//! Covers every acceptance criterion, all seven statuses, all seven closed claim kinds,
//! all nine register rules, all ten write-time refusals, receipt & receipt-set BLAKE3
//! resolution, due-control filtering (Round-4 Review Note #3), `pilot:` scope grammar
//! (Review Note #4), optional `sources` citations (Review Note #5), atomic write-path
//! digest preservation (`write_path`), and append-only ID property testing over 1,000
//! deterministic edit scripts.
//!
//! Zero mocks; zero file deletions (`COMMON.md` 9b).

use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use frob_conformance::reasons::HarnessReason;
use frob_conformance::register::{
    COMPILED_RECURRING_GATES, Claim, LATE_RECURRING_GATES, Part, Recorded, Status,
    ValidationContext, Violation, check_append_only_ids, load, load_with_context,
    parse_and_validate_bytes, parse_register_bytes, record, validate,
};
use frob_core::id::hash_blob;
use frob_core::testkit::registry::{self as testkit_registry, Controls};

const VALID_CLAIM_ID: &str =
    "clm:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
const OTHER_CLAIM_ID: &str =
    "clm:fedcba9876543210fedcba9876543210fedcba9876543210fedcba9876543210";
const VALID_EVIDENCE_ID: &str =
    "ev:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const COMMIT_SHA: &str = "0123456789abcdef0123456789abcdef01234567";

static SCRATCH_SEQ: AtomicU64 = AtomicU64::new(0);

fn scratch_dir(label: &str) -> PathBuf {
    let base = env::var_os("BIG_SCRATCH")
        .or_else(|| env::var_os("TMPDIR"))
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("/tmp"));
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_secs());
    let n = SCRATCH_SEQ.fetch_add(1, Ordering::Relaxed);
    let dir = base
        .join("frob-CopperDove")
        .join(format!("n40-01-{label}-{ts}-{}-{n}", std::process::id()));
    fs::create_dir_all(dir.join("receipts/gates")).expect("create scratch receipts/gates");
    fs::create_dir_all(dir.join("registries")).expect("create scratch registries");
    dir
}

/// Write a committed receipt file at `<root>/receipts/gates/<hash>.json` and return `<hash>`.
fn commit_receipt(root: &Path, body: &str) -> String {
    let bytes = body.as_bytes();
    let hex = hash_blob(bytes).to_hex();
    let path = root.join("receipts/gates").join(format!("{hex}.json"));
    fs::write(&path, bytes).expect("write receipt");
    hex
}

fn caught_controls(due: Option<&str>, caught: bool) -> Controls {
    let due_line = due.map_or_else(String::new, |d| format!("due = \"{d}\"\n"));
    let toml = format!(
        "schema = \"frob.gauntlet.controls/1\"\n\
         [[control]]\n\
         id = \"NC-0001\"\n\
         seed = \"CX-001\"\n\
         gate = [\"GATE-KIN-01\"]\n\
         kind = \"mutation\"\n\
         target = \"crates/frob-core/src/lib.rs\"\n\
         defect = \"synthetic defect\"\n\
         expect = \"fails check\"\n\
         {due_line}\
         last_seen = {{ caught = {caught}, session = \"s2\", date = \"2026-11-24\" }}\n"
    );
    Controls::parse_str(&toml, "synthetic-controls.toml").expect("parse controls")
}

fn multi_controls() -> Controls {
    let toml = "\
schema = \"frob.gauntlet.controls/1\"
[[control]]
id = \"NC-0001\"
seed = \"CX-001\"
gate = [\"GATE-KIN-01\"]
kind = \"mutation\"
target = \"crates/frob-core/src/lib.rs\"
defect = \"due M0 defect\"
expect = \"fails check\"
due = \"M0\"
last_seen = { caught = true, session = \"s2\", date = \"2026-11-24\" }

[[control]]
id = \"NC-0002\"
seed = \"CX-002\"
gate = [\"GATE-KIN-01\"]
kind = \"mutation\"
target = \"crates/frob-core/src/lib.rs\"
defect = \"future M2 defect\"
expect = \"fails later\"
due = \"M2\"
last_seen = { caught = false, session = \"s2\", date = \"2026-11-24\" }
";
    Controls::parse_str(toml, "synthetic-multi-controls.toml").expect("parse multi controls")
}

fn valid_pass_fixture(root: &Path, controls: Controls) -> (String, String, ValidationContext) {
    let receipt_hex = commit_receipt(
        root,
        "{\"schema\":\"frob.gauntlet.receipt/1\",\"gate\":\"GATE-KIN-01\",\"ok\":true}\n",
    );
    let toml = format!(
        "schema = \"frob.gates/1\"\n\n\
         [[gate]]\n\
         id = \"GATE-KIN-01\"\n\
         kind = \"convergence\"\n\
         claim = \"parity\"\n\
         claim_id = \"{VALID_CLAIM_ID}\"\n\
         section = \"12.4\"\n\
         consumers = [\"frob-kin\"]\n\
         threshold = \"rel_err <= 1e-12\"\n\
         milestone = \"M0\"\n\
         tier = [\"G0:host\"]\n\
         controls = [\"NC-0001\"]\n\n\
         [[gate.part]]\n\
         milestone = \"M0\"\n\
         status = \"pass\"\n\
         measured = \"rel_err = 2.1e-15\"\n\
         commit = \"{COMMIT_SHA}\"\n\
         command = \"cargo test -p frob-conformance --test g0_laws -- GATE-KIN-01\"\n\
         receipt = \"{receipt_hex}\"\n\
         fingerprint = \"fp:host-x86_64-0001\"\n\
         seed = \"0\"\n\
         evidence = [\"{VALID_EVIDENCE_ID}\"]\n\
         recorded = {{ date = \"2026-11-24\", session = \"s2\", implemented_by = \"s1\" }}\n"
    );
    let mut live_claims = BTreeSet::new();
    live_claims.insert(VALID_CLAIM_ID.to_owned());
    let mut live_evidence = BTreeSet::new();
    live_evidence.insert(VALID_EVIDENCE_ID.to_owned());
    let ctx = ValidationContext {
        repo_root: Some(root.to_path_buf()),
        controls: Some(controls),
        live_claims: Some(live_claims),
        live_evidence: Some(live_evidence),
        retired_ids: BTreeSet::new(),
        previous_ids: None,
        enforce_recurring_flags: false,
        ..ValidationContext::default()
    };
    (toml, receipt_hex, ctx)
}

fn has_harness_reason(errs: &[Violation], expected: HarnessReason) -> bool {
    errs.iter()
        .any(|v| v.reason == expected.as_str() && v.harness_reason == Some(expected))
}

// ============================================================================
// 1. Default status and printed form
// ============================================================================

#[test]
fn status_default_is_not_run() {
    let toml = "\
schema = \"frob.gates/1\"

[[gate]]
id = \"GATE-KIN-01\"
kind = \"convergence\"
claim = \"known-answer\"
claim_id = \"INV-KIN-01\"
section = \"12.1\"
threshold = \"err <= 1e-12\"
milestone = \"M0\"
tier = [\"G0:host\"]
controls = [\"NC-0001\"]
";
    let reg = parse_and_validate_bytes(toml.as_bytes(), "t.toml", &ValidationContext::default())
        .expect("row with no parts must load");
    let g = reg.get("GATE-KIN-01").expect("gate present");
    assert_eq!(g.derived_status(), Status::NotRun);
    assert_eq!(g.derived_status_at("M0"), Status::NotRun);
    assert_eq!(g.printed_status(None), "not run");
    assert_eq!(g.printed_status(Some("M0")), "not run");
    assert_ne!(g.printed_status(None), Status::Pass.as_str());
}

// ============================================================================
// 2. Pass requires all provenance fields
// ============================================================================

#[test]
fn pass_requires_all_fields() {
    let root = scratch_dir("pass-fields");
    let (valid_toml, _, ctx) = valid_pass_fixture(&root, caught_controls(Some("M0"), true));
    parse_and_validate_bytes(valid_toml.as_bytes(), "valid.toml", &ctx)
        .expect("complete pass row must validate");

    let cases: &[(&str, &str)] = &[
        ("measured", "measured = \"rel_err = 2.1e-15\""),
        ("commit", &format!("commit = \"{COMMIT_SHA}\"")),
        (
            "command",
            "command = \"cargo test -p frob-conformance --test g0_laws -- GATE-KIN-01\"",
        ),
        ("receipt", "receipt = "),
        ("fingerprint", "fingerprint = \"fp:host-x86_64-0001\""),
        ("seed", "seed = \"0\""),
    ];

    for &(field, needle) in cases {
        let mutated = if field == "measured" {
            valid_toml.replace(needle, "measured = \"\"")
        } else {
            valid_toml
                .lines()
                .filter(|line| !line.starts_with(needle))
                .collect::<Vec<_>>()
                .join("\n")
        };
        let errs = parse_and_validate_bytes(mutated.as_bytes(), "mut.toml", &ctx)
            .expect_err(&format!("removing `{field}` must be refused"));
        assert!(
            errs.iter().any(|v| v.field.as_deref() == Some(field)),
            "refusal for missing `{field}` must name `{field}`; got {errs:?}"
        );
    }
}

// ============================================================================
// 3. Committed receipt and receipt-set resolution under receipts/gates/
// ============================================================================

#[test]
fn receipt_resolves_under_receipts_gates() {
    let root = scratch_dir("receipt-res");
    let (valid_toml, receipt_hex, ctx) =
        valid_pass_fixture(&root, caught_controls(Some("M0"), true));
    parse_and_validate_bytes(valid_toml.as_bytes(), "ok.toml", &ctx)
        .expect("committed receipt matching hash must succeed");

    // (a) Receipt only under target/gauntlet/receipts/ (uncommitted) -> evidence-unresolved.
    let target_dir = root.join("target/gauntlet/receipts");
    fs::create_dir_all(&target_dir).expect("mkdir target receipts");
    let uncommitted_body = b"{\"schema\":\"frob.gauntlet.receipt/1\",\"uncommitted\":true}\n";
    let uncommitted_hex = hash_blob(uncommitted_body).to_hex();
    fs::write(
        target_dir.join(format!("{uncommitted_hex}.json")),
        uncommitted_body,
    )
    .expect("write uncommitted receipt");
    let only_target = valid_toml.replace(&receipt_hex, &uncommitted_hex);
    let errs = parse_and_validate_bytes(only_target.as_bytes(), "target-only.toml", &ctx)
        .expect_err("uncommitted target/ receipt must fail");
    assert!(
        has_harness_reason(&errs, HarnessReason::EvidenceUnresolved),
        "expected evidence-unresolved, got {errs:?}"
    );

    // (b) 1-byte tamper in committed receipt -> evidence-unresolved.
    let tamper_root = scratch_dir("receipt-tamper");
    let (tamper_toml, tamper_hex, tamper_ctx) =
        valid_pass_fixture(&tamper_root, caught_controls(Some("M0"), true));
    let receipt_file = tamper_root
        .join("receipts/gates")
        .join(format!("{tamper_hex}.json"));
    let mut raw = fs::read(&receipt_file).expect("read receipt");
    raw[0] = b' ';
    fs::write(&receipt_file, &raw).expect("tamper byte");
    let errs = parse_and_validate_bytes(tamper_toml.as_bytes(), "tamper.toml", &tamper_ctx)
        .expect_err("1-byte tampered receipt must fail");
    assert!(
        has_harness_reason(&errs, HarnessReason::EvidenceUnresolved),
        "expected evidence-unresolved on tamper, got {errs:?}"
    );

    // (c) Receipt set with 2 committed members -> accepted.
    let set_root = scratch_dir("receipt-set");
    let m1 = commit_receipt(&set_root, "{\"schema\":\"frob.gauntlet.receipt/1\",\"m\":1}\n");
    let m2 = commit_receipt(&set_root, "{\"schema\":\"frob.gauntlet.receipt/1\",\"m\":2}\n");
    let set_body = format!(
        "{{\"schema\":\"frob.gauntlet.receipt-set/1\",\"members\":[\"{m1}\",\"{m2}\"]}}\n"
    );
    let set_hex = commit_receipt(&set_root, &set_body);
    let (base_toml, base_hex, set_ctx) =
        valid_pass_fixture(&set_root, caught_controls(Some("M0"), true));
    let set_toml = base_toml.replace(&base_hex, &set_hex);
    parse_and_validate_bytes(set_toml.as_bytes(), "set-ok.toml", &set_ctx)
        .expect("receipt set with two committed members must pass");

    // (d) Receipt set with 1 missing member -> evidence-unresolved naming the missing member.
    let missing_hex = hash_blob(b"missing-member").to_hex();
    let bad_set_body = format!(
        "{{\"schema\":\"frob.gauntlet.receipt-set/1\",\"members\":[\"{m1}\",\"{missing_hex}\"]}}\n"
    );
    let bad_set_hex = commit_receipt(&set_root, &bad_set_body);
    let bad_set_toml = base_toml.replace(&base_hex, &bad_set_hex);
    let errs = parse_and_validate_bytes(bad_set_toml.as_bytes(), "set-bad.toml", &set_ctx)
        .expect_err("receipt set with missing member must fail");
    assert!(
        has_harness_reason(&errs, HarnessReason::EvidenceUnresolved),
        "expected evidence-unresolved, got {errs:?}"
    );
    assert!(
        errs.iter().any(|v| v.message.contains(&missing_hex)),
        "violation must name missing member `{missing_hex}`; got {errs:?}"
    );
}

// ============================================================================
// 4. Fail is a record (RC-67)
// ============================================================================

#[test]
fn fail_is_a_record() {
    let root = scratch_dir("fail-record");
    let receipt_hex = commit_receipt(
        &root,
        "{\"schema\":\"frob.gauntlet.receipt/1\",\"gate\":\"GATE-KIN-01\",\"ok\":false}\n",
    );
    let ctx = ValidationContext {
        repo_root: Some(root.clone()),
        ..ValidationContext::default()
    };
    let good_fail = format!(
        "schema = \"frob.gates/1\"\n\n\
         [[gate]]\n\
         id = \"GATE-KIN-01\"\n\
         kind = \"convergence\"\n\
         claim = \"parity\"\n\
         claim_id = \"INV-KIN-01\"\n\
         section = \"12.4\"\n\
         threshold = \"rel_err <= 1e-12\"\n\
         milestone = \"M0\"\n\
         tier = [\"G0:host\"]\n\
         controls = [\"NC-0001\"]\n\n\
         [[gate.part]]\n\
         milestone = \"M0\"\n\
         status = \"fail\"\n\
         measured = \"rel_err = 5.4e-9\"\n\
         commit = \"{COMMIT_SHA}\"\n\
         command = \"cargo test -p frob-conformance --test g0_laws -- GATE-KIN-01\"\n\
         receipt = \"{receipt_hex}\"\n\
         evidence = []\n\
         recorded = {{ date = \"2026-11-24\", session = \"s2\", implemented_by = \"s1\" }}\n"
    );
    parse_and_validate_bytes(good_fail.as_bytes(), "fail-ok.toml", &ctx)
        .expect("complete fail record must validate");

    let empty_receipt = good_fail.replace(&receipt_hex, "");
    let errs = parse_and_validate_bytes(empty_receipt.as_bytes(), "fail-no-receipt.toml", &ctx)
        .expect_err("fail record with empty receipt must be refused");
    assert!(
        errs.iter().any(|v| v.field.as_deref() == Some("receipt")),
        "refusal must name `receipt`; got {errs:?}"
    );
}

// ============================================================================
// 5. Kill row has no measurement fields (RC-66)
// ============================================================================

#[test]
fn kill_row_has_no_measurement_fields() {
    let valid_kill = "\
schema = \"frob.gates/1\"

[[gate]]
id = \"KILL-SIM-01\"
kind = \"kill\"
section = \"15.9\"
trigger = \"contact throughput < 0.25x baseline\"
fallback = \"adopt dense PGS fallback\"
decided = \"M2\"
";
    let reg = parse_and_validate_bytes(
        valid_kill.as_bytes(),
        "kill-ok.toml",
        &ValidationContext::default(),
    )
    .expect("clean kill row must load without claim");
    assert!(reg.get("KILL-SIM-01").expect("kill row").is_kill());

    for (fld, extra) in [
        ("claim", "claim = \"performance\"\n"),
        ("claim_id", "claim_id = \"INV-01\"\n"),
        ("threshold", "threshold = \"x > 0\"\n"),
        ("tier", "tier = [\"G0:host\"]\n"),
        ("controls", "controls = [\"NC-0001\"]\n"),
        (
            "part",
            "\n[[gate.part]]\nmilestone = \"M2\"\nstatus = \"not-run\"\nmeasured = \"\"\nevidence = []\nrecorded = { date = \"\", session = \"\", implemented_by = \"\" }\n",
        ),
    ] {
        let bad = format!("{valid_kill}{extra}");
        let errs = parse_and_validate_bytes(
            bad.as_bytes(),
            "kill-bad.toml",
            &ValidationContext::default(),
        )
        .expect_err(&format!("kill row carrying `{fld}` must fail load"));
        assert!(
            errs.iter().any(|v| v.field.as_deref() == Some(fld)),
            "kill row error must name `{fld}`; got {errs:?}"
        );
    }
}

// ============================================================================
// 6–9. Pass refusals (a), (b) + due-control Note #3(1), (c), (d)
// ============================================================================

#[test]
fn pass_refused_without_control() {
    let root = scratch_dir("no-ctrl");
    let (valid_toml, _, ctx) = valid_pass_fixture(&root, caught_controls(Some("M0"), true));
    let no_ctrl = valid_toml.replace("controls = [\"NC-0001\"]", "controls = []");
    let errs = parse_and_validate_bytes(no_ctrl.as_bytes(), "no-ctrl.toml", &ctx)
        .expect_err("empty controls on pass must fail");
    assert!(
        has_harness_reason(&errs, HarnessReason::PassWithoutControl),
        "expected pass-without-control, got {errs:?}"
    );
}

#[test]
fn pass_refused_when_control_missed() {
    let root = scratch_dir("missed-ctrl");
    let (valid_toml, _, ctx) = valid_pass_fixture(&root, caught_controls(Some("M0"), false));
    let errs = parse_and_validate_bytes(valid_toml.as_bytes(), "missed.toml", &ctx)
        .expect_err("uncaught due control must be refused");
    assert!(
        has_harness_reason(&errs, HarnessReason::ControlNotCaught),
        "expected control-not-caught, got {errs:?}"
    );

    // Round-4 Review Note #3(1): a control due at a LATER milestone (M2) on an M0 part
    // is `not-yet-due` and must NOT block the M0 part's pass, while at M2 it DOES block.
    let (mut multi_toml, _, multi_ctx) = valid_pass_fixture(&root, multi_controls());
    multi_toml = multi_toml.replace(
        "controls = [\"NC-0001\"]",
        "controls = [\"NC-0001\", \"NC-0002\"]",
    );
    parse_and_validate_bytes(multi_toml.as_bytes(), "m0-future-ctrl.toml", &multi_ctx)
        .expect("future M2 control must not block M0 part");

    let m2_part = multi_toml.replace(
        "[[gate.part]]\nmilestone = \"M0\"",
        "[[gate.part]]\nmilestone = \"M2\"",
    );
    let m2_errs = parse_and_validate_bytes(m2_part.as_bytes(), "m2-due-ctrl.toml", &multi_ctx)
        .expect_err("M2 control is due at M2 and uncaught, so M2 pass must fail");
    assert!(
        has_harness_reason(&m2_errs, HarnessReason::ControlNotCaught),
        "expected control-not-caught at M2, got {m2_errs:?}"
    );
}

#[test]
fn pass_refused_self_certified() {
    let root = scratch_dir("self-cert");
    let (valid_toml, _, ctx) = valid_pass_fixture(&root, caught_controls(Some("M0"), true));

    // Single implementer matching recording session.
    let direct = valid_toml.replace(
        "session = \"s2\", implemented_by = \"s1\"",
        "session = \"s1\", implemented_by = \"s1\"",
    );
    let errs = parse_and_validate_bytes(direct.as_bytes(), "direct.toml", &ctx)
        .expect_err("recording session == implemented_by must be refused");
    assert!(
        has_harness_reason(&errs, HarnessReason::SelfCertified),
        "expected self-certified, got {errs:?}"
    );

    // Co-implementer in comma-joined implemented_by (`s1,s3` with recording session `s3`).
    let co_impl = valid_toml.replace(
        "session = \"s2\", implemented_by = \"s1\"",
        "session = \"s3\", implemented_by = \"s1,s3\"",
    );
    let co_errs = parse_and_validate_bytes(co_impl.as_bytes(), "co-impl.toml", &ctx)
        .expect_err("co-implementer recording session must be refused");
    assert!(
        has_harness_reason(&co_errs, HarnessReason::SelfCertified),
        "expected self-certified for co-implementer, got {co_errs:?}"
    );

    // Fresh session `s2` with `implemented_by = "s1,s3"` succeeds.
    let fresh = valid_toml.replace("implemented_by = \"s1\"", "implemented_by = \"s1,s3\"");
    parse_and_validate_bytes(fresh.as_bytes(), "fresh.toml", &ctx)
        .expect("fresh session s2 must succeed");
}

// ============================================================================
// 10. Closed claim set (7)
// ============================================================================

#[test]
fn claim_closed_set() {
    for &c in Claim::ALL {
        let toml = format!(
            "schema = \"frob.gates/1\"\n\n\
             [[gate]]\n\
             id = \"GATE-CLAIM-01\"\n\
             kind = \"convergence\"\n\
             claim = \"{}\"\n\
             claim_id = \"INV-01\"\n\
             section = \"8.1\"\n\
             threshold = \"t\"\n\
             milestone = \"M0\"\n\
             tier = [\"G0:host\"]\n\
             controls = [\"NC-0001\"]\n",
            c.as_str()
        );
        parse_and_validate_bytes(toml.as_bytes(), "ok.toml", &ValidationContext::default())
            .expect("all 7 closed claim kinds must load");
    }

    for bad_claim in [
        "\"method\"",
        "\"Performance of the method\"",
        "\"performance of the method\"",
        "[\"parity\", \"performance\"]",
    ] {
        let toml = format!(
            "schema = \"frob.gates/1\"\n\n\
             [[gate]]\n\
             id = \"GATE-CLAIM-02\"\n\
             kind = \"convergence\"\n\
             claim = {bad_claim}\n\
             claim_id = \"INV-01\"\n\
             section = \"8.1\"\n\
             threshold = \"t\"\n\
             milestone = \"M0\"\n\
             tier = [\"G0:host\"]\n\
             controls = [\"NC-0001\"]\n"
        );
        let errs = parse_and_validate_bytes(
            toml.as_bytes(),
            "bad-claim.toml",
            &ValidationContext::default(),
        )
        .expect_err(&format!("invalid claim {bad_claim} must fail"));
        assert!(
            has_harness_reason(&errs, HarnessReason::ClaimNotOneOfSeven),
            "expected claim-not-one-of-seven for {bad_claim}, got {errs:?}"
        );
    }
}

// ============================================================================
// 11–16. no-claim, blocked, killed/superseded, target-as-measurement,
//        evidence-unresolved, GATE-GROUND-10 import lane, blocks_exit
// ============================================================================

#[test]
fn no_claim_only_for_reality() {
    let reality_ok = "\
schema = \"frob.gates/1\"

[[gate]]
id = \"GATE-GROUND-10\"
kind = \"convergence\"
claim = \"reality\"
claim_id = \"INV-GROUND-10\"
section = \"18.10\"
threshold = \"physical dataset agreement\"
milestone = \"M3\"
needs = \"import-lane\"
tier = [\"G2:host\"]
controls = [\"NC-0001\"]

[[gate.part]]
milestone = \"M3\"
status = \"no-claim\"
measured = \"\"
evidence = []
recorded = { date = \"2026-11-24\", session = \"s2\", implemented_by = \"\" }
detail = \"no-dataset-admitted\"
";
    parse_and_validate_bytes(
        reality_ok.as_bytes(),
        "reality.toml",
        &ValidationContext::default(),
    )
    .expect("reality gate with no-claim + detail must succeed");

    let non_reality = reality_ok
        .replace("id = \"GATE-GROUND-10\"", "id = \"GATE-KIN-01\"")
        .replace("claim = \"reality\"", "claim = \"parity\"")
        .replace("needs = \"import-lane\"\n", "");
    let errs = parse_and_validate_bytes(
        non_reality.as_bytes(),
        "non-reality.toml",
        &ValidationContext::default(),
    )
    .expect_err("no-claim on non-reality gate must be refused");
    assert!(
        has_harness_reason(&errs, HarnessReason::NoClaimOnNonReality),
        "expected no-claim-on-non-reality, got {errs:?}"
    );
}

#[test]
fn blocked_needs_open_id() {
    let template = |detail: &str| {
        format!(
            "schema = \"frob.gates/1\"\n\n\
             [[gate]]\n\
             id = \"GATE-SPARK-01\"\n\
             kind = \"convergence\"\n\
             claim = \"known-answer\"\n\
             claim_id = \"INV-SPARK-01\"\n\
             section = \"16.1\"\n\
             threshold = \"t\"\n\
             milestone = \"M1\"\n\
             tier = [\"G1:host\"]\n\
             controls = [\"NC-0001\"]\n\n\
             [[gate.part]]\n\
             milestone = \"M1\"\n\
             status = \"blocked\"\n\
             measured = \"\"\n\
             evidence = []\n\
             recorded = {{ date = \"2026-11-24\", session = \"s2\", implemented_by = \"\" }}\n\
             detail = \"{detail}\"\n"
        )
    };

    for ok_detail in ["OPEN-SPARK-1", "claim-reading: N16c-05"] {
        let reg = parse_and_validate_bytes(
            template(ok_detail).as_bytes(),
            "ok.toml",
            &ValidationContext::default(),
        )
        .expect(&format!("blocked detail `{ok_detail}` must succeed"));
        let printed = reg
            .get("GATE-SPARK-01")
            .expect("gate")
            .printed_status(Some("M1"));
        assert_eq!(printed, format!("blocked: {ok_detail}"));
    }

    for bad_detail in ["", "waiting", "TODO"] {
        let errs = parse_and_validate_bytes(
            template(bad_detail).as_bytes(),
            "bad.toml",
            &ValidationContext::default(),
        )
        .expect_err(&format!("blocked detail `{bad_detail}` must fail"));
        assert!(
            has_harness_reason(&errs, HarnessReason::BlockedWithoutQuestion),
            "expected blocked-without-question for `{bad_detail}`, got {errs:?}"
        );
    }
}

#[test]
fn killed_and_superseded_need_detail() {
    let killed_ok = "\
schema = \"frob.gates/1\"

[[gate]]
id = \"GATE-SIM-02\"
kind = \"convergence\"
claim = \"performance\"
claim_id = \"INV-SIM-02\"
section = \"15.2\"
threshold = \"t\"
milestone = \"M2\"
tier = [\"G1:host\"]
controls = [\"NC-0001\"]

[[gate.part]]
milestone = \"M2\"
status = \"killed\"
measured = \"\"
evidence = []
recorded = { date = \"2026-11-24\", session = \"s2\", implemented_by = \"\" }
kill_gate = \"KILL-SIM-01\"
fallback = \"dense PGS\"
detail = \"KILL-SIM-01; dense PGS; ADR-SIM-07\"
";
    let reg = parse_and_validate_bytes(
        killed_ok.as_bytes(),
        "killed-ok.toml",
        &ValidationContext::default(),
    )
    .expect("complete killed row must pass");
    assert_eq!(
        reg.get("GATE-SIM-02").expect("gate").printed_status(None),
        "killed: fallback adopted (dense PGS)"
    );

    // Program row citing BET- instead of KILL- also passes.
    let prog_bet = killed_ok
        .replace("id = \"GATE-SIM-02\"", "id = \"PROG-01\"")
        .replace("kind = \"convergence\"", "kind = \"program\"")
        .replace("kill_gate = \"KILL-SIM-01\"\n", "")
        .replace("KILL-SIM-01", "BET-04");
    parse_and_validate_bytes(
        prog_bet.as_bytes(),
        "prog-bet.toml",
        &ValidationContext::default(),
    )
    .expect("program row killed by BET- + fallback + ADR must pass");

    // Killed without decision/ADR -> killed-without-decision.
    let killed_bad = killed_ok
        .replace("kill_gate = \"KILL-SIM-01\"\n", "")
        .replace("fallback = \"dense PGS\"\n", "")
        .replace(
            "detail = \"KILL-SIM-01; dense PGS; ADR-SIM-07\"",
            "detail = \"stopped\"",
        );
    let errs = parse_and_validate_bytes(
        killed_bad.as_bytes(),
        "killed-bad.toml",
        &ValidationContext::default(),
    )
    .expect_err("killed without kill gate/fallback/ADR must fail");
    assert!(
        has_harness_reason(&errs, HarnessReason::KilledWithoutDecision),
        "expected killed-without-decision, got {errs:?}"
    );

    // Superseded naming an existing successor gate passes; naming a missing gate fails.
    let sup_ok = "\
schema = \"frob.gates/1\"

[[gate]]
id = \"GATE-OLD-01\"
kind = \"convergence\"
claim = \"parity\"
claim_id = \"INV-OLD-01\"
section = \"12.1\"
threshold = \"t\"
milestone = \"M0\"
tier = [\"G0:host\"]
controls = [\"NC-0001\"]

[[gate.part]]
milestone = \"M0\"
status = \"superseded\"
measured = \"\"
evidence = []
recorded = { date = \"2026-11-24\", session = \"s2\", implemented_by = \"\" }
detail = \"GATE-NEW-01\"

[[gate]]
id = \"GATE-NEW-01\"
kind = \"convergence\"
claim = \"parity\"
claim_id = \"INV-NEW-01\"
section = \"12.2\"
threshold = \"t\"
milestone = \"M0\"
tier = [\"G0:host\"]
controls = [\"NC-0001\"]
";
    parse_and_validate_bytes(sup_ok.as_bytes(), "sup-ok.toml", &ValidationContext::default())
        .expect("superseded pointing at existing gate must pass");

    let sup_bad = sup_ok.replace("detail = \"GATE-NEW-01\"", "detail = \"GATE-MISSING-99\"");
    let sup_errs = parse_and_validate_bytes(
        sup_bad.as_bytes(),
        "sup-bad.toml",
        &ValidationContext::default(),
    )
    .expect_err("superseded naming non-existent gate must fail");
    assert!(
        sup_errs
            .iter()
            .any(|v| v.reason == "superseded-target-missing"),
        "expected superseded-target-missing, got {sup_errs:?}"
    );
}

#[test]
fn target_as_measurement() {
    let root = scratch_dir("target-as-meas");
    let (valid_toml, _, ctx) = valid_pass_fixture(&root, caught_controls(Some("M0"), true));

    let same_as_threshold = valid_toml.replace(
        "measured = \"rel_err = 2.1e-15\"",
        "measured = \"rel_err <= 1e-12\"",
    );
    let errs = parse_and_validate_bytes(same_as_threshold.as_bytes(), "same.toml", &ctx)
        .expect_err("measured == threshold must be refused");
    assert!(
        has_harness_reason(&errs, HarnessReason::TargetAsMeasurement),
        "expected target-as-measurement, got {errs:?}"
    );

    let empty_meas = valid_toml.replace("measured = \"rel_err = 2.1e-15\"", "measured = \"\"");
    let empty_errs = parse_and_validate_bytes(empty_meas.as_bytes(), "empty.toml", &ctx)
        .expect_err("empty measured on pass must be refused");
    assert!(
        has_harness_reason(&empty_errs, HarnessReason::TargetAsMeasurement),
        "expected target-as-measurement on empty measured, got {empty_errs:?}"
    );
}

#[test]
fn evidence_unresolved() {
    let root = scratch_dir("ev-unresolved");
    let (valid_toml, receipt_hex, ctx) =
        valid_pass_fixture(&root, caught_controls(Some("M0"), true));

    // 1. Dangling evidence ID.
    let bad_ev = valid_toml.replace(
        VALID_EVIDENCE_ID,
        "ev:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
    );
    let errs1 = parse_and_validate_bytes(bad_ev.as_bytes(), "bad-ev.toml", &ctx)
        .expect_err("dangling evidence must fail");
    assert!(has_harness_reason(&errs1, HarnessReason::EvidenceUnresolved));

    // 2. Dangling ClaimId (well-formed N34-11 syntax, absent from live_claims).
    let bad_clm = valid_toml.replace(VALID_CLAIM_ID, OTHER_CLAIM_ID);
    let errs2 = parse_and_validate_bytes(bad_clm.as_bytes(), "bad-clm.toml", &ctx)
        .expect_err("dangling ClaimId must fail");
    assert!(has_harness_reason(&errs2, HarnessReason::EvidenceUnresolved));

    // 3. Malformed ClaimId (fails N34-11 ClaimId::parse — Round-4 Review Notes #1 & #3(2)).
    let malformed_clm = valid_toml.replace(VALID_CLAIM_ID, "INV-KIN-01");
    let errs3 = parse_and_validate_bytes(malformed_clm.as_bytes(), "malformed-clm.toml", &ctx)
        .expect_err("non-ClaimId on pass must fail");
    assert!(has_harness_reason(&errs3, HarnessReason::EvidenceUnresolved));

    // 4. Missing receipt file.
    let missing_hex = hash_blob(b"nonexistent-receipt").to_hex();
    let bad_rcpt = valid_toml.replace(&receipt_hex, &missing_hex);
    let errs4 = parse_and_validate_bytes(bad_rcpt.as_bytes(), "bad-rcpt.toml", &ctx)
        .expect_err("missing receipt must fail");
    assert!(has_harness_reason(&errs4, HarnessReason::EvidenceUnresolved));
}

#[test]
fn import_lane_rests_at_no_claim() {
    let make_g10 = |status: &str, detail: &str| {
        format!(
            "schema = \"frob.gates/1\"\n\n\
             [[gate]]\n\
             id = \"GATE-GROUND-10\"\n\
             kind = \"convergence\"\n\
             claim = \"reality\"\n\
             claim_id = \"INV-GROUND-10\"\n\
             section = \"18.10\"\n\
             threshold = \"physical dataset validation\"\n\
             milestone = \"M3\"\n\
             needs = \"import-lane\"\n\
             tier = [\"G2:host\"]\n\
             controls = [\"NC-0001\"]\n\n\
             [[gate.part]]\n\
             milestone = \"M3\"\n\
             status = \"{status}\"\n\
             measured = \"\"\n\
             evidence = []\n\
             recorded = {{ date = \"2026-11-24\", session = \"s2\", implemented_by = \"\" }}\n\
             detail = \"{detail}\"\n"
        )
    };

    for (st, det) in [
        ("blocked", "OPEN-GROUND-1"),
        ("no-claim", "no-dataset-admitted"),
    ] {
        let reg = parse_and_validate_bytes(
            make_g10(st, det).as_bytes(),
            "g10.toml",
            &ValidationContext::default(),
        )
        .expect(&format!("GATE-GROUND-10 ({st}/{det}) must validate"));
        for m in ["M0", "M1", "M2", "M3", "M4", "M5", "M6", "M7", "M8"] {
            assert!(
                reg.due_gates(m).is_empty(),
                "GATE-GROUND-10 ({st}) must never appear in {m} due set"
            );
        }
    }

    // Empty detail on `no-claim` -> refused.
    let errs_empty = parse_and_validate_bytes(
        make_g10("no-claim", "").as_bytes(),
        "g10-empty.toml",
        &ValidationContext::default(),
    )
    .expect_err("no-claim with empty detail must be refused");
    assert!(has_harness_reason(
        &errs_empty,
        HarnessReason::NoClaimOnNonReality
    ));

    // `needs = "import-lane"` on a non-reality row -> refused.
    let bad_needs = make_g10("blocked", "OPEN-GROUND-1").replace("reality", "fidelity");
    let errs_needs = parse_and_validate_bytes(
        bad_needs.as_bytes(),
        "bad-needs.toml",
        &ValidationContext::default(),
    )
    .expect_err("import-lane on non-reality must be refused");
    assert!(has_harness_reason(
        &errs_needs,
        HarnessReason::NoClaimOnNonReality
    ));
}

#[test]
fn blocks_exit_defaults_true() {
    let toml = "\
schema = \"frob.gates/1\"

[[gate]]
id = \"INT-FSIM-01\"
kind = \"admission\"
claim = \"parity\"
claim_id = \"INV-FSIM-01\"
section = \"33.1\"
threshold = \"admission ok\"
milestone = \"M0\"
tier = [\"G0:host\"]
controls = [\"NC-0001\"]

[[gate]]
id = \"INT-OPT-02\"
kind = \"admission\"
claim = \"parity\"
claim_id = \"INV-OPT-02\"
section = \"33.2\"
threshold = \"optional sibling pilot\"
milestone = \"M0\"
blocks_exit = false
tier = [\"G0:host\"]
controls = [\"NC-0002\"]
";
    let reg = parse_and_validate_bytes(toml.as_bytes(), "adm.toml", &ValidationContext::default())
        .expect("admission rows must load");
    assert!(reg.get("INT-FSIM-01").expect("g1").blocks_exit);
    assert!(!reg.get("INT-OPT-02").expect("g2").blocks_exit);
    let due_m0: Vec<&str> = reg.due_gates("M0").iter().map(|g| g.id.as_str()).collect();
    assert_eq!(due_m0, vec!["INT-FSIM-01"]);

    // `blocks_exit = false` on a convergence row fails validation.
    let bad = "\
schema = \"frob.gates/1\"

[[gate]]
id = \"GATE-KIN-01\"
kind = \"convergence\"
claim = \"parity\"
claim_id = \"INV-KIN-01\"
section = \"12.1\"
threshold = \"t\"
milestone = \"M0\"
blocks_exit = false
tier = [\"G0:host\"]
controls = [\"NC-0001\"]
";
    let errs = parse_and_validate_bytes(bad.as_bytes(), "bad-be.toml", &ValidationContext::default())
        .expect_err("blocks_exit = false on convergence row must fail");
    assert!(errs.iter().any(|v| v.reason == "blocks-exit-non-admission"));
}

// ============================================================================
// 17. Rule 8: recurring set (4 compiled + 5 late)
// ============================================================================

#[test]
fn recurring_set_is_four() {
    assert_eq!(COMPILED_RECURRING_GATES.len(), 4);
    assert_eq!(LATE_RECURRING_GATES.len(), 5);

    let mut toml = String::from("schema = \"frob.gates/1\"\n");
    for &id in COMPILED_RECURRING_GATES
        .iter()
        .chain(LATE_RECURRING_GATES.iter())
    {
        toml.push_str(&format!(
            "\n[[gate]]\n\
             id = \"{id}\"\n\
             kind = \"convergence\"\n\
             claim = \"known-answer\"\n\
             claim_id = \"INV-REC\"\n\
             section = \"38.15\"\n\
             threshold = \"t\"\n\
             milestone = \"M0\"\n\
             recurring = true\n\
             tier = [\"G0:host\"]\n\
             controls = [\"NC-0001\"]\n"
        ));
    }
    toml.push_str(
        "\n[[gate]]\n\
         id = \"GATE-KIN-04\"\n\
         kind = \"convergence\"\n\
         claim = \"known-answer\"\n\
         claim_id = \"INV-KIN-04\"\n\
         section = \"12.4\"\n\
         threshold = \"t\"\n\
         milestone = \"M1\"\n\
         tier = [\"G1:host\"]\n\
         controls = [\"NC-0001\"]\n",
    );

    let ctx = ValidationContext {
        enforce_recurring_flags: true,
        ..ValidationContext::default()
    };
    let reg = parse_and_validate_bytes(toml.as_bytes(), "rec-ok.toml", &ctx)
        .expect("exact recurring set must validate");
    // Recurring gates due at M0 are also due at M4.
    let due_m4: Vec<&str> = reg.due_gates("M4").iter().map(|g| g.id.as_str()).collect();
    assert!(due_m4.contains(&"GATE-SAFE-01"));
    assert!(!due_m4.contains(&"GATE-KIN-04"));

    // Planted extra `recurring = true` on `GATE-KIN-04` -> fails naming GATE-KIN-04.
    let extra = toml.replace(
        "id = \"GATE-KIN-04\"\nkind = \"convergence\"",
        "id = \"GATE-KIN-04\"\nkind = \"convergence\"\nrecurring = true",
    );
    let extra_errs = parse_and_validate_bytes(extra.as_bytes(), "rec-extra.toml", &ctx)
        .expect_err("extra recurring flag must fail");
    assert!(
        extra_errs
            .iter()
            .any(|v| v.gate_id.as_deref() == Some("GATE-KIN-04")
                && v.reason == "recurring-flag-mismatch"),
        "expected recurring-flag-mismatch on GATE-KIN-04, got {extra_errs:?}"
    );

    // Planted missing `recurring = true` on `GATE-SAFE-01` -> fails naming GATE-SAFE-01.
    let missing = toml.replace(
        "id = \"GATE-SAFE-01\"\nkind = \"convergence\"\nclaim = \"known-answer\"\nclaim_id = \"INV-REC\"\nsection = \"38.15\"\nthreshold = \"t\"\nmilestone = \"M0\"\nrecurring = true\n",
        "id = \"GATE-SAFE-01\"\nkind = \"convergence\"\nclaim = \"known-answer\"\nclaim_id = \"INV-REC\"\nsection = \"38.15\"\nthreshold = \"t\"\nmilestone = \"M0\"\n",
    );
    let miss_errs = parse_and_validate_bytes(missing.as_bytes(), "rec-miss.toml", &ctx)
        .expect_err("missing recurring flag on GATE-SAFE-01 must fail");
    assert!(
        miss_errs
            .iter()
            .any(|v| v.gate_id.as_deref() == Some("GATE-SAFE-01")
                && v.reason == "recurring-flag-mismatch"),
        "expected recurring-flag-mismatch on GATE-SAFE-01, got {miss_errs:?}"
    );
}

// ============================================================================
// 18. Append-only ID property test over 1,000 deterministic edit scripts
// ============================================================================

#[test]
fn ids_append_only() {
    let mut state: u64 = 0x5EED_4001_CAFE_BABEu64;
    let mut next_u64 = || -> u64 {
        state = state
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        state
    };

    for script_idx in 0..1000usize {
        let base_len = (next_u64() % 6) as usize + 2;
        let previous: Vec<String> = (1..=base_len)
            .map(|i| format!("GATE-PROP-{i:02}"))
            .collect();
        let mut retired = BTreeSet::new();
        retired.insert("GATE-RET-01".to_owned());

        let op = script_idx % 5;
        match op {
            0 => {
                // Pure append of 1..=3 fresh IDs -> MUST pass.
                let mut cur = previous.clone();
                let add_n = (next_u64() % 3) as usize + 1;
                for k in 1..=add_n {
                    cur.push(format!("GATE-PROP-{:02}", base_len + k));
                }
                let v = check_append_only_ids(&previous, &retired, &cur);
                assert!(v.is_empty(), "pure append #{script_idx} must pass: {v:?}");
            }
            1 => {
                // Remove one existing ID -> MUST fail with reused-id.
                let mut cur = previous.clone();
                let rm_idx = (next_u64() as usize) % cur.len();
                cur.remove(rm_idx);
                let v = check_append_only_ids(&previous, &retired, &cur);
                assert!(
                    has_harness_reason(&v, HarnessReason::ReusedId),
                    "removal #{script_idx} must fail: {v:?}"
                );
            }
            2 => {
                // Renumber / swap two existing IDs -> MUST fail with reused-id.
                let mut cur = previous.clone();
                cur.swap(0, 1);
                let v = check_append_only_ids(&previous, &retired, &cur);
                assert!(
                    has_harness_reason(&v, HarnessReason::ReusedId),
                    "renumber #{script_idx} must fail: {v:?}"
                );
            }
            3 => {
                // Duplicate an existing ID -> MUST fail with duplicate-id.
                let mut cur = previous.clone();
                let dup = cur[(next_u64() as usize) % cur.len()].clone();
                cur.push(dup);
                let v = check_append_only_ids(&previous, &retired, &cur);
                assert!(
                    has_harness_reason(&v, HarnessReason::DuplicateId),
                    "duplicate #{script_idx} must fail: {v:?}"
                );
            }
            _ => {
                // Reuse a retired ID -> MUST fail with reused-id.
                let mut cur = previous.clone();
                cur.push("GATE-RET-01".to_owned());
                let v = check_append_only_ids(&previous, &retired, &cur);
                assert!(
                    has_harness_reason(&v, HarnessReason::ReusedId),
                    "reuse retired #{script_idx} must fail: {v:?}"
                );
            }
        }
    }
}

// ============================================================================
// 19. Pure D0 TOML printer round-trip
// ============================================================================

#[test]
fn print_is_pure() {
    let root = scratch_dir("print-pure");
    let (valid_toml, _, ctx) = valid_pass_fixture(&root, caught_controls(Some("M0"), true));
    let reg = parse_and_validate_bytes(valid_toml.as_bytes(), "p.toml", &ctx).expect("parse");
    let p1 = reg.print_toml();
    let p2 = reg.print_toml();
    assert_eq!(p1, p2, "printer must be bit-identical across calls");
    let reparsed = parse_and_validate_bytes(p1.as_bytes(), "reparse.toml", &ctx)
        .expect("printed TOML must re-validate");
    assert_eq!(reg, reparsed, "print -> parse must round-trip cleanly");
}

// ============================================================================
// 20. Round-4 Review Notes #4 (`pilot:` scope) and #5 (`sources`)
// ============================================================================

#[test]
fn pilot_scope_and_sources_review_notes() {
    let ok = "\
schema = \"frob.gates/1\"

[[gate]]
id = \"GATE-ATEL-01\"
kind = \"convergence\"
claim = \"design-comparison\"
claim_id = \"INV-ATEL-01\"
section = \"26.3\"
sources = [\"SRC-001\", \"SRC-014#table-2\"]
threshold = \"pilot falsifier threshold\"
milestone = \"M2\"
tier = [\"G1:host\"]
controls = [\"NC-0001\"]

[[gate.part]]
milestone = \"M2\"
scope = \"pilot: PILOT-ATEL-01\"
status = \"not-run\"
measured = \"\"
evidence = []
recorded = { date = \"\", session = \"\", implemented_by = \"\" }
";
    parse_and_validate_bytes(ok.as_bytes(), "pilot-ok.toml", &ValidationContext::default())
        .expect("valid pilot: scope + valid sources must pass");

    let bad_pilot = ok.replace("PILOT-ATEL-01", "PILOT-bad");
    let errs_p = parse_and_validate_bytes(
        bad_pilot.as_bytes(),
        "bad-pilot.toml",
        &ValidationContext::default(),
    )
    .expect_err("malformed pilot ID in scope must fail");
    assert!(errs_p.iter().any(|v| v.reason == "invalid-pilot-scope"));

    let bad_pilot_status = ok.replace("status = \"not-run\"", "status = \"blocked\"\ndetail = \"OPEN-ATEL-1\"");
    let errs_st = parse_and_validate_bytes(
        bad_pilot_status.as_bytes(),
        "bad-pilot-st.toml",
        &ValidationContext::default(),
    )
    .expect_err("pilot-scoped part with non-not-run status must fail");
    assert!(
        errs_st
            .iter()
            .any(|v| v.reason == "pilot-scope-must-be-not-run")
    );

    let bad_source = ok.replace("SRC-001", "PAPER-001");
    let errs_src = parse_and_validate_bytes(
        bad_source.as_bytes(),
        "bad-src.toml",
        &ValidationContext::default(),
    )
    .expect_err("non-SRC- source citation must fail");
    assert!(
        errs_src
            .iter()
            .any(|v| v.reason == "invalid-source-citation")
    );
}

// ============================================================================
// 21. Write-path tests (name contains `write_path` for E2E targeting)
// ============================================================================

#[test]
fn write_path_refuses_self_certified_and_uncaught_leaving_file_unchanged() {
    let root = scratch_dir("write-path");
    let receipt_hex = commit_receipt(
        &root,
        "{\"schema\":\"frob.gauntlet.receipt/1\",\"gate\":\"GATE-KIN-01\",\"ok\":true}\n",
    );
    let gates_path = root.join("registries/gates.toml");
    let initial = format!(
        "schema = \"frob.gates/1\"\n\n\
         [[gate]]\n\
         id = \"GATE-KIN-01\"\n\
         kind = \"convergence\"\n\
         claim = \"parity\"\n\
         claim_id = \"{VALID_CLAIM_ID}\"\n\
         section = \"12.4\"\n\
         threshold = \"rel_err <= 1e-12\"\n\
         milestone = \"M0\"\n\
         tier = [\"G0:host\"]\n\
         controls = [\"NC-0001\"]\n\n\
         [[gate.part]]\n\
         milestone = \"M0\"\n\
         status = \"not-run\"\n\
         measured = \"\"\n\
         evidence = []\n\
         recorded = {{ date = \"\", session = \"\", implemented_by = \"\" }}\n"
    );
    fs::write(&gates_path, initial.as_bytes()).expect("write initial gates.toml");
    let before_hash = hash_blob(&fs::read(&gates_path).expect("read initial"));

    let mut live_claims = BTreeSet::new();
    live_claims.insert(VALID_CLAIM_ID.to_owned());
    let mut live_evidence = BTreeSet::new();
    live_evidence.insert(VALID_EVIDENCE_ID.to_owned());

    let make_part = |session: &str, impl_by: &str| Part {
        milestone: "M0".to_owned(),
        scope: None,
        status: Status::Pass,
        measured: "rel_err = 1.2e-15".to_owned(),
        commit: COMMIT_SHA.to_owned(),
        command: "cargo test -p frob-conformance --test g0_laws -- GATE-KIN-01".to_owned(),
        receipt: receipt_hex.clone(),
        fingerprint: "fp:host-0001".to_owned(),
        seed: "0".to_owned(),
        evidence: vec![VALID_EVIDENCE_ID.to_owned()],
        recorded: Recorded {
            date: "2026-11-24".to_owned(),
            session: session.to_owned(),
            implemented_by: impl_by.to_owned(),
        },
        detail: None,
        kill_gate: None,
        fallback: None,
        open_question: None,
    };

    // 1. Self-certified write refused; gates.toml bytes and BLAKE3 digest untouched.
    let ctx_caught = ValidationContext {
        repo_root: Some(root.clone()),
        controls: Some(caught_controls(Some("M0"), true)),
        live_claims: Some(live_claims.clone()),
        live_evidence: Some(live_evidence.clone()),
        retired_ids: BTreeSet::new(),
        previous_ids: None,
        enforce_recurring_flags: false,
        ..ValidationContext::default()
    };
    for (session, implemented_by) in [
        ("s1", "s1,s3"),
        ("s1", "session:s1"),
        ("session:s1", "s1"),
        ("s1", "session:s3, session:s1"),
        (" s1 ", " , session:s1, "),
    ] {
        let errs_self = record(
            &gates_path,
            "GATE-KIN-01",
            make_part(session, implemented_by),
            &ctx_caught,
        )
        .unwrap_err();
        assert!(
            errs_self.len() == 1 && has_harness_reason(&errs_self, HarnessReason::SelfCertified),
            "expected only self-certified for {session:?} in {implemented_by:?}, got {errs_self:?}"
        );
        assert_eq!(
            hash_blob(&fs::read(&gates_path).expect("read after self-cert")),
            before_hash,
            "gates.toml digest must be unchanged after self-certified refusal"
        );
    }

    // 2. Uncaught control write refused; gates.toml bytes and BLAKE3 digest untouched.
    let ctx_uncaught = ValidationContext {
        repo_root: Some(root.clone()),
        controls: Some(caught_controls(Some("M0"), false)),
        live_claims: Some(live_claims),
        live_evidence: Some(live_evidence),
        retired_ids: BTreeSet::new(),
        previous_ids: None,
        enforce_recurring_flags: false,
        ..ValidationContext::default()
    };
    let errs_uncaught = record(
        &gates_path,
        "GATE-KIN-01",
        make_part("s2", "s1"),
        &ctx_uncaught,
    )
    .expect_err("uncaught control write must be refused");
    assert!(has_harness_reason(
        &errs_uncaught,
        HarnessReason::ControlNotCaught
    ));
    assert_eq!(
        hash_blob(&fs::read(&gates_path).expect("read after uncaught")),
        before_hash,
        "gates.toml digest must be unchanged after control-not-caught refusal"
    );

    // 3. Valid fresh-session caught-control write succeeds atomically and re-loads cleanly.
    let updated = record(
        &gates_path,
        "GATE-KIN-01",
        make_part("s2", "session:s1"),
        &ctx_caught,
    )
    .expect("valid write must succeed");
    assert_eq!(
        updated.get("GATE-KIN-01").expect("gate").derived_status(),
        Status::Pass
    );
    let reloaded = load_with_context(&gates_path, &ctx_caught).expect("reloaded file validates");
    assert_eq!(updated, reloaded);
}

// ============================================================================
// 22. Unknown field, unknown status, duplicate/retired ID, committed file & ADR
// ============================================================================

#[test]
fn unknown_field_and_status_fail_naming_row() {
    let bad_field = "\
schema = \"frob.gates/1\"

[[gate]]
id = \"GATE-UNK-01\"
kind = \"convergence\"
claim = \"parity\"
claim_id = \"INV-UNK-01\"
section = \"12.1\"
threshold = \"t\"
milestone = \"M0\"
tier = [\"G0:host\"]
controls = [\"NC-0001\"]
surprise_key = true
";
    let errs = parse_register_bytes(bad_field.as_bytes(), "bad-field.toml")
        .expect_err("unknown field must fail");
    assert!(
        errs.iter()
            .any(|v| v.gate_id.as_deref() == Some("GATE-UNK-01") && v.reason == "unknown-field"),
        "must name GATE-UNK-01 and unknown-field; got {errs:?}"
    );

    let bad_status = "\
schema = \"frob.gates/1\"

[[gate]]
id = \"GATE-UNK-02\"
kind = \"convergence\"
claim = \"parity\"
claim_id = \"INV-UNK-02\"
section = \"12.1\"
threshold = \"t\"
milestone = \"M0\"
tier = [\"G0:host\"]
controls = [\"NC-0001\"]

[[gate.part]]
milestone = \"M0\"
status = \"green\"
measured = \"\"
evidence = []
recorded = { date = \"\", session = \"\", implemented_by = \"\" }
";
    let errs_st = parse_register_bytes(bad_status.as_bytes(), "bad-st.toml")
        .expect_err("unknown status must fail");
    assert!(
        errs_st
            .iter()
            .any(|v| v.gate_id.as_deref() == Some("GATE-UNK-02") && v.reason == "invalid-status"),
        "must name GATE-UNK-02 and invalid-status; got {errs_st:?}"
    );
}

#[test]
fn committed_gates_toml_and_schema_adr_check() {
    let ws = testkit_registry::root().expect("workspace root");
    let gates_path = ws.join("registries/gates.toml");
    let reg = load(&gates_path).expect("committed registries/gates.toml must validate");
    assert_eq!(reg.schema, "frob.gates/1");
    assert!(!reg.gates.is_empty());
    let v = validate(&reg, &ValidationContext::default());
    assert!(v.is_empty(), "committed register has violations: {v:?}");

    let schema_text =
        fs::read_to_string(ws.join("schemas/registries/gates.schema.json")).expect("read schema");
    assert!(
        schema_text.contains("docs/adr/ADR-GAUNT-04.md"),
        "gates.schema.json must cite docs/adr/ADR-GAUNT-04.md"
    );
    assert!(
        ws.join("docs/adr/ADR-GAUNT-04.md").is_file(),
        "docs/adr/ADR-GAUNT-04.md must exist"
    );
}
