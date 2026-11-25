#![deny(unsafe_code)]

//! Negative-control registry validation and `GATE-GAUNT-01` v0 check (`BD-025`).
//!
//! Run as `cargo test -p frob-conformance --test registry_check`.
//! Supports `FROB_CONFORMANCE_ROOT=<dir>` for scratch workspaces in
//! `scripts/e2e/conformance_v0.sh`. Never deletes files or directories (COMMON.md 9b).

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use frob_conformance::controls::{
    CONTROLS_PATH, ControlFinding, ControlReason, check_registry, load_and_validate,
    serialize_control, serialize_controls_file, validate,
};
use frob_conformance::gates_v0::{GATES_PATH, GateRegistry};
use frob_core::testkit::registry::{self, Controls};

static SCRATCH_SEQ: AtomicU64 = AtomicU64::new(0);

fn scratch_dir(label: &str) -> PathBuf {
    let base = env::var_os("BIG_SCRATCH")
        .or_else(|| env::var_os("TMPDIR"))
        .map(PathBuf::from)
        .unwrap_or_else(env::temp_dir);
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let seq = SCRATCH_SEQ.fetch_add(1, Ordering::Relaxed);
    let dir = base
        .join("frob-BronzePelican")
        .join(format!("registry_check-{label}-{nanos}-{seq}"));
    fs::create_dir_all(&dir).expect("create scratch directory");
    dir
}

fn workspace_root() -> PathBuf {
    if let Some(ov) = env::var_os("FROB_CONFORMANCE_ROOT") {
        return PathBuf::from(ov);
    }
    registry::root().expect("locate workspace root via franken-stack.lock")
}

fn has_reason(findings: &[ControlFinding], reason: ControlReason) -> bool {
    findings.iter().any(|f| f.reason == reason)
}

fn sample_gate(id: &str, milestone: &str, controls: &str, part_status: &str, session: &str) -> String {
    let measured = if part_status == "pass" { "ok" } else { "" };
    format!(
        r#"[[gate]]
id = "{id}"
claim = "known-answer"
claim_id = "INV-GAUNT-02"
section = "§38.1"
threshold = "every pass gate has a caught negative control"
milestone = "{milestone}"
tier = ["G0:host"]
controls = [{controls}]

[[gate.part]]
milestone = "{milestone}"
status = "{part_status}"
measured = "{measured}"
evidence = []
recorded = {{ date = "2026-11-24", session = "{session}", implemented_by = "" }}
"#
    )
}

fn parse_gates(body: &str) -> GateRegistry {
    let doc = format!("schema = \"frob.gates/1\"\n\n{body}");
    GateRegistry::parse_str(&doc, GATES_PATH).expect("parse scratch gates.toml")
}

// ============================================================================
// 1. Clean-tree check (used directly and by scripts/e2e/conformance_v0.sh)
// ============================================================================

#[test]
fn registry_check_clean_tree() {
    let root = workspace_root();
    let gates = GateRegistry::load(&root).expect("load registries/gates.toml");
    let controls_file = root.join(CONTROLS_PATH);
    let controls_text = fs::read_to_string(&controls_file)
        .unwrap_or_else(|e| panic!("failed to read {}: {e}", controls_file.display()));

    let controls = load_and_validate(&controls_text, CONTROLS_PATH, None, &gates, &root)
        .unwrap_or_else(|findings| {
            let rendered: Vec<String> = findings.iter().map(ToString::to_string).collect();
            panic!(
                "controls validation failed on {}:\n{}",
                root.display(),
                rendered.join("\n")
            );
        });

    let report = check_registry(&gates, &controls);
    print!("{}", report.format_summary());
    assert!(
        report.is_ok(),
        "registry_check failed on {}:\n{}",
        root.display(),
        report.format_summary()
    );
}

// ============================================================================
// 2. BD-025 Test Plan unit tests
// ============================================================================

#[test]
fn control_roundtrip() {
    let raw = r#"schema = "frob.gauntlet.controls/1"

[[control]]
id = "NC-0412"
seed = "CX-262"
gate = ["GATE-ARENA-04"]
kind = "mutation"
target = "frob-actuate::stage::integrate"
defect = "explicit Euler, current before wheel speed, at 0.5 ms"
expect = "wheel-loop row reports spectral radius 1.497 where 0.9207 is true; the row fails"
last_seen = { caught = true, session = "sess-2026-11-24", date = "2026-11-24" }
feature = "cx-262"
due = "M1"
"#;

    let parsed = Controls::parse_str(raw, CONTROLS_PATH).expect("parse NC-0412");
    assert_eq!(parsed.all().len(), 1);
    let nc = parsed.get("NC-0412").expect("NC-0412 present");

    let serialized_once = serialize_controls_file(parsed.all());
    let reparsed = Controls::parse_str(&serialized_once, CONTROLS_PATH).expect("reparse NC-0412");
    assert_eq!(reparsed.all(), parsed.all());

    let serialized_twice = serialize_controls_file(reparsed.all());
    assert_eq!(serialized_once, serialized_twice);
    assert_eq!(serialize_control(nc), serialize_control(&reparsed.all()[0]));
}

#[test]
fn control_ids_append_only() {
    let ws = scratch_dir("append_only");
    let gates = parse_gates(&sample_gate("GATE-GAUNT-01", "M0", "\"NC-0010\", \"NC-0011\", \"NC-0012\"", "not-run", ""));

    // NC-0010 twice -> reused
    let dup = r#"schema = "frob.gauntlet.controls/1"

[[control]]
id = "NC-0010"
seed = "GATE-GAUNT-01"
gate = ["GATE-GAUNT-01"]
kind = "planted-input"
target = "frob-conformance::controls::check_registry"
defect = "first defect"
expect = "fails with no-control"
fixture = "fixtures/a.toml"

[[control]]
id = "NC-0010"
seed = "GATE-GAUNT-01"
gate = ["GATE-GAUNT-01"]
kind = "planted-input"
target = "frob-conformance::controls::check_registry"
defect = "duplicate id"
expect = "fails with no-control"
fixture = "fixtures/b.toml"
"#;
    let err_dup = load_and_validate(dup, CONTROLS_PATH, None, &gates, &ws)
        .expect_err("duplicate NC-0010 must fail");
    assert!(has_reason(&err_dup, ControlReason::Reused), "got: {err_dup:?}");

    // NC-0012 before NC-0011 -> out-of-order
    let ooo = r#"schema = "frob.gauntlet.controls/1"

[[control]]
id = "NC-0012"
seed = "GATE-GAUNT-01"
gate = ["GATE-GAUNT-01"]
kind = "planted-input"
target = "frob-conformance::controls::check_registry"
defect = "first"
expect = "fails with no-control"
fixture = "fixtures/a.toml"

[[control]]
id = "NC-0011"
seed = "GATE-GAUNT-01"
gate = ["GATE-GAUNT-01"]
kind = "planted-input"
target = "frob-conformance::controls::check_registry"
defect = "out of order"
expect = "fails with no-control"
fixture = "fixtures/b.toml"
"#;
    let err_ooo = load_and_validate(ooo, CONTROLS_PATH, None, &gates, &ws)
        .expect_err("NC-0012 before NC-0011 must fail");
    assert!(has_reason(&err_ooo, ControlReason::OutOfOrder), "got: {err_ooo:?}");

    // Previous commit had NC-0010 and NC-0011.
    let prev_toml = r#"schema = "frob.gauntlet.controls/1"

[[control]]
id = "NC-0010"
seed = "GATE-GAUNT-01"
gate = ["GATE-GAUNT-01"]
kind = "planted-input"
target = "frob-conformance::controls::check_registry"
defect = "empty controls"
expect = "fails with no-control"
fixture = "fixtures/a.toml"

[[control]]
id = "NC-0011"
seed = "GATE-GAUNT-01"
gate = ["GATE-GAUNT-01"]
kind = "planted-input"
target = "frob-conformance::controls::validate"
defect = "dangling control"
expect = "fails with dangling-control"
fixture = "fixtures/b.toml"
"#;
    let prev = Controls::parse_str(prev_toml, CONTROLS_PATH).expect("parse prev");

    // Previous identifier absent -> removed
    let removed_toml = r#"schema = "frob.gauntlet.controls/1"

[[control]]
id = "NC-0010"
seed = "GATE-GAUNT-01"
gate = ["GATE-GAUNT-01"]
kind = "planted-input"
target = "frob-conformance::controls::check_registry"
defect = "empty controls"
expect = "fails with no-control"
fixture = "fixtures/a.toml"
"#;
    let err_rem = load_and_validate(removed_toml, CONTROLS_PATH, Some(&prev), &gates, &ws)
        .expect_err("removed NC-0011 must fail");
    assert!(has_reason(&err_rem, ControlReason::Removed), "got: {err_rem:?}");

    // NC-0011's target changed against previous commit -> renumbered
    let changed_toml = prev_toml.replace(
        "frob-conformance::controls::validate",
        "frob-conformance::controls::other_target",
    );
    let err_ren = load_and_validate(&changed_toml, CONTROLS_PATH, Some(&prev), &gates, &ws)
        .expect_err("target change on non-placeholder must fail");
    assert!(has_reason(&err_ren, ControlReason::Renumbered), "got: {err_ren:?}");
}

#[test]
fn placeholder_target_replaced_once() {
    let ws = scratch_dir("placeholder");
    let gates = parse_gates(&sample_gate("GATE-ARENA-04", "M1", "\"NC-0412\", \"NC-0413\"", "not-run", ""));

    // Previous commit: two placeholder targets, neither with last_seen.
    let prev_no_ls = r#"schema = "frob.gauntlet.controls/1"

[[control]]
id = "NC-0412"
seed = "CX-262"
gate = ["GATE-ARENA-04"]
kind = "mutation"
target = "BD-123:cx-262"
defect = "explicit Euler, current before wheel speed, at 0.5 ms"
expect = "wheel-loop row reports spectral radius 1.497 where 0.9207 is true; the row fails"
feature = "cx-262"
due = "M1"

[[control]]
id = "NC-0413"
seed = "CX-301"
gate = ["GATE-ARENA-04"]
kind = "mutation"
target = "CX-301"
defect = "unseeded plant placeholder"
expect = "row fails with error > 0.1"
feature = "cx-301"
due = "M1"
"#;
    let prev = Controls::parse_str(prev_no_ls, CONTROLS_PATH).expect("parse prev");

    // First replacement of both placeholders with real targets succeeds.
    let upgraded_toml = prev_no_ls
        .replace("target = \"BD-123:cx-262\"", "target = \"frob-actuate::stage::integrate\"")
        .replace("target = \"CX-301\"", "target = \"frob-actuate::stage::limit\"");
    let upgraded = load_and_validate(&upgraded_toml, CONTROLS_PATH, Some(&prev), &gates, &ws)
        .expect("one-time placeholder target upgrade must load");

    // Second change of either target -> renumbered.
    let second_change_a = upgraded_toml.replace(
        "frob-actuate::stage::integrate",
        "frob-actuate::stage::integrate_v2",
    );
    let err_a = load_and_validate(&second_change_a, CONTROLS_PATH, Some(&upgraded), &gates, &ws)
        .expect_err("second change of NC-0412 target must fail");
    assert!(has_reason(&err_a, ControlReason::Renumbered), "got: {err_a:?}");

    let second_change_b = upgraded_toml.replace(
        "frob-actuate::stage::limit",
        "frob-actuate::stage::limit_v2",
    );
    let err_b = load_and_validate(&second_change_b, CONTROLS_PATH, Some(&upgraded), &gates, &ws)
        .expect_err("second change of NC-0413 target must fail");
    assert!(has_reason(&err_b, ControlReason::Renumbered), "got: {err_b:?}");

    // Same first change when previous commit already had last_seen -> renumbered.
    let prev_with_ls = r#"schema = "frob.gauntlet.controls/1"

[[control]]
id = "NC-0412"
seed = "CX-262"
gate = ["GATE-ARENA-04"]
kind = "mutation"
target = "BD-123:cx-262"
defect = "explicit Euler, current before wheel speed, at 0.5 ms"
expect = "wheel-loop row reports spectral radius 1.497 where 0.9207 is true; the row fails"
last_seen = { caught = true, session = "sess-1", date = "2026-11-24" }
feature = "cx-262"
due = "M1"
"#;
    let prev_ls = Controls::parse_str(prev_with_ls, CONTROLS_PATH).expect("parse prev_with_ls");
    let changed_after_ls = prev_with_ls.replace(
        "target = \"BD-123:cx-262\"",
        "target = \"frob-actuate::stage::integrate\"",
    );
    let err_ls = load_and_validate(&changed_after_ls, CONTROLS_PATH, Some(&prev_ls), &gates, &ws)
        .expect_err("placeholder change after last_seen must fail");
    assert!(has_reason(&err_ls, ControlReason::Renumbered), "got: {err_ls:?}");
}

#[test]
fn control_needs_observable() {
    let ws = scratch_dir("observable");
    let gates = parse_gates(&sample_gate("GATE-GAUNT-01", "M0", "\"NC-0001\"", "not-run", ""));

    let empty_defect = r#"schema = "frob.gauntlet.controls/1"

[[control]]
id = "NC-0001"
seed = "GATE-GAUNT-01"
gate = ["GATE-GAUNT-01"]
kind = "planted-input"
target = "frob-conformance::controls::check_registry"
defect = ""
expect = "registry check fails with no-control"
fixture = "fixtures/a.toml"
"#;
    let err1 = load_and_validate(empty_defect, CONTROLS_PATH, None, &gates, &ws)
        .expect_err("empty defect must fail");
    assert!(has_reason(&err1, ControlReason::DefectOrExpectEmpty), "got: {err1:?}");

    let empty_expect = r#"schema = "frob.gauntlet.controls/1"

[[control]]
id = "NC-0001"
seed = "GATE-GAUNT-01"
gate = ["GATE-GAUNT-01"]
kind = "planted-input"
target = "frob-conformance::controls::check_registry"
defect = "planted empty control list"
expect = ""
fixture = "fixtures/a.toml"
"#;
    let err2 = load_and_validate(empty_expect, CONTROLS_PATH, None, &gates, &ws)
        .expect_err("empty expect must fail");
    assert!(has_reason(&err2, ControlReason::DefectOrExpectEmpty), "got: {err2:?}");

    let vague_expect = empty_expect.replace("expect = \"\"", "expect = \"looks somewhat off\"");
    let err3 = load_and_validate(&vague_expect, CONTROLS_PATH, None, &gates, &ws)
        .expect_err("expect without observable must fail");
    assert!(has_reason(&err3, ControlReason::DefectOrExpectEmpty), "got: {err3:?}");
}

#[test]
fn control_needs_gate() {
    let ws = scratch_dir("needs_gate");
    let gates = parse_gates(&sample_gate("GATE-GAUNT-01", "M0", "\"NC-0001\"", "not-run", ""));

    let no_gate = r#"schema = "frob.gauntlet.controls/1"

[[control]]
id = "NC-0001"
seed = "GATE-GAUNT-01"
gate = []
kind = "planted-input"
target = "frob-conformance::controls::check_registry"
defect = "planted empty controls list"
expect = "registry check fails with no-control"
fixture = "fixtures/a.toml"
"#;
    let err_no = load_and_validate(no_gate, CONTROLS_PATH, None, &gates, &ws)
        .expect_err("empty gate list must fail");
    assert!(has_reason(&err_no, ControlReason::NoGate), "got: {err_no:?}");

    let unknown_gate = no_gate.replace("gate = []", "gate = [\"GATE-NO-SUCH-99\"]");
    let err_unk = load_and_validate(&unknown_gate, CONTROLS_PATH, None, &gates, &ws)
        .expect_err("unknown gate must fail");
    assert!(has_reason(&err_unk, ControlReason::UnknownGate), "got: {err_unk:?}");
}

#[test]
fn control_kind_closed() {
    let ws = scratch_dir("kind_closed");
    // Create a dummy target crate manifest without feature `cx-999` and one valid synthetic fixture.
    fs::create_dir_all(ws.join("crates/frob-actuate/src")).expect("create dummy crate");
    fs::write(
        ws.join("crates/frob-actuate/Cargo.toml"),
        "[package]\nname = \"frob-actuate\"\nversion = \"0.0.1\"\nedition = \"2024\"\n\n[features]\ndefault = []\ncx-262 = []\n",
    )
    .expect("write dummy Cargo.toml");
    fs::create_dir_all(ws.join("fixtures")).expect("create fixtures dir");
    fs::write(ws.join("fixtures/valid_world.toml"), "# synthetic\nvalue = 1\n")
        .expect("write valid fixture");

    let gates_not_run = parse_gates(&sample_gate(
        "GATE-ARENA-04",
        "M1",
        "\"NC-0001\", \"NC-0002\", \"NC-0003\", \"NC-0004\", \"NC-0005\"",
        "not-run",
        "",
    ));
    let gates_pass = parse_gates(&sample_gate(
        "GATE-ARENA-04",
        "M1",
        "\"NC-0001\", \"NC-0002\"",
        "pass",
        "sess-1",
    ));

    // Invalid kind `boundary` -> kind-not-closed
    let bad_kind = r#"schema = "frob.gauntlet.controls/1"

[[control]]
id = "NC-0001"
seed = "CX-262"
gate = ["GATE-ARENA-04"]
kind = "boundary"
target = "frob-actuate::stage::integrate"
defect = "bad kind"
expect = "fails with kind-not-closed"
fixture = "fixtures/valid_world.toml"
"#;
    let err_kind = load_and_validate(bad_kind, CONTROLS_PATH, None, &gates_not_run, &ws)
        .expect_err("unknown kind must fail");
    assert!(has_reason(&err_kind, ControlReason::KindNotClosed), "got: {err_kind:?}");

    // All five valid kinds load cleanly.
    let all_five = r#"schema = "frob.gauntlet.controls/1"

[[control]]
id = "NC-0001"
seed = "CX-262"
gate = ["GATE-ARENA-04"]
kind = "mutation"
target = "frob-actuate::stage::integrate"
defect = "explicit Euler"
expect = "spectral radius 1.497 fails row"
feature = "cx-262"

[[control]]
id = "NC-0002"
seed = "GATE-ARENA-04"
gate = ["GATE-ARENA-04"]
kind = "planted-input"
target = "frob-actuate::input"
defect = "corrupted torque table"
expect = "row fails with error > 1.0"
fixture = "fixtures/valid_world.toml"

[[control]]
id = "NC-0003"
seed = "GATE-ARENA-04"
gate = ["GATE-ARENA-04"]
kind = "planted-design"
target = "frob-actuate::design"
defect = "undersized gearbox"
expect = "design check fails margin < 1.0"
fixture = "fixtures/valid_world.toml"

[[control]]
id = "NC-0004"
seed = "GATE-ARENA-04"
gate = ["GATE-ARENA-04"]
kind = "planted-world"
target = "frob-actuate::world"
defect = "unmodeled friction drop"
expect = "world check fails slip > 0.2"
fixture = "fixtures/valid_world.toml"

[[control]]
id = "NC-0005"
seed = "GATE-ARENA-04"
gate = ["GATE-ARENA-04"]
kind = "a-a"
target = "frob-actuate::ab"
defect = "identical baseline vs baseline"
expect = "a-a procedure reports delta 0.0 within tolerance"
fixture = "fixtures/valid_world.toml"
"#;
    load_and_validate(all_five, CONTROLS_PATH, None, &gates_not_run, &ws)
        .expect("all five valid control kinds must load");

    // Mutation with no feature field -> feature-missing
    let no_feat = r#"schema = "frob.gauntlet.controls/1"

[[control]]
id = "NC-0001"
seed = "CX-262"
gate = ["GATE-ARENA-04"]
kind = "mutation"
target = "frob-actuate::stage::integrate"
defect = "explicit Euler"
expect = "spectral radius 1.497 fails row"
"#;
    let err_nf = load_and_validate(no_feat, CONTROLS_PATH, None, &gates_not_run, &ws)
        .expect_err("mutation without feature must fail");
    assert!(has_reason(&err_nf, ControlReason::FeatureMissing), "got: {err_nf:?}");

    // Mutation naming undeclared feature `cx-999` + planted-world naming absent fixture:
    // MUST load when listed only by `not-run` gate, and MUST fail when listed by `pass` gate!
    let unbuilt_plants = r#"schema = "frob.gauntlet.controls/1"

[[control]]
id = "NC-0001"
seed = "CX-262"
gate = ["GATE-ARENA-04"]
kind = "mutation"
target = "frob-actuate::stage::integrate"
defect = "undeclared feature plant"
expect = "spectral radius 1.497 fails row"
feature = "cx-999"

[[control]]
id = "NC-0002"
seed = "GATE-ARENA-04"
gate = ["GATE-ARENA-04"]
kind = "planted-world"
target = "frob-actuate::world"
defect = "absent world fixture"
expect = "world check fails slip > 0.2"
fixture = "fixtures/does_not_exist.toml"
"#;
    load_and_validate(unbuilt_plants, CONTROLS_PATH, None, &gates_not_run, &ws)
        .expect("unbuilt plants listed only by not-run gates must load");

    let err_pass = load_and_validate(unbuilt_plants, CONTROLS_PATH, None, &gates_pass, &ws)
        .expect_err("unbuilt plants listed by pass gate must fail");
    assert!(
        has_reason(&err_pass, ControlReason::FeatureMissing),
        "expected feature-missing, got: {err_pass:?}"
    );
    assert!(
        has_reason(&err_pass, ControlReason::FixtureMissing),
        "expected fixture-missing, got: {err_pass:?}"
    );
}

#[test]
fn boundary_is_not_control() {
    let ws = scratch_dir("boundary");
    let gates = parse_gates(&sample_gate("GATE-GAUNT-01", "M0", "\"NC-0001\"", "not-run", ""));

    let boundary_only = r#"schema = "frob.gauntlet.controls/1"

[[control]]
id = "NC-0001"
seed = "GATE-GAUNT-01"
gate = ["GATE-GAUNT-01"]
kind = "planted-input"
target = "frob-contact::film::check"
defect = "a 4 mm film of water is refused"
expect = "request is refused with FROB-REF-201"
fixture = "fixtures/water_film_4mm.toml"
"#;
    let err = load_and_validate(boundary_only, CONTROLS_PATH, None, &gates, &ws)
        .expect_err("boundary restatement without removed_check must fail");
    assert!(has_reason(&err, ControlReason::BoundaryNotControl), "got: {err:?}");

    let with_removed = format!("{boundary_only}removed_check = \"film thickness limit\"\n");
    load_and_validate(&with_removed, CONTROLS_PATH, None, &gates, &ws)
        .expect("record with removed_check must be accepted");
}

#[test]
fn gate_without_control_fails() {
    let gates = parse_gates(&sample_gate("GATE-TEST-01", "M0", "", "pass", "sess-1"));
    let controls = Controls::default();
    let report = check_registry(&gates, &controls);
    assert!(!report.is_ok());
    assert!(
        report
            .findings
            .iter()
            .any(|f| f.record == "GATE-TEST-01" && f.reason == ControlReason::NoControl),
        "expected no-control for GATE-TEST-01, got: {:?}",
        report.findings
    );
}

#[test]
fn one_way_link_fails() {
    let gates = parse_gates(&format!(
        "{}\n{}",
        sample_gate("GATE-GAUNT-01", "M0", "\"NC-0001\"", "not-run", ""),
        sample_gate("GATE-GAUNT-05", "M0", "", "not-run", "")
    ));
    // GATE-GAUNT-01 lists NC-0001, but NC-0001 only lists GATE-GAUNT-05.
    let controls = Controls::parse_str(
        r#"schema = "frob.gauntlet.controls/1"

[[control]]
id = "NC-0001"
seed = "GATE-GAUNT-05"
gate = ["GATE-GAUNT-05"]
kind = "planted-input"
target = "frob-conformance::controls::check_registry"
defect = "planted empty controls list"
expect = "fails with no-control"
fixture = "fixtures/a.toml"
"#,
        CONTROLS_PATH,
    )
    .expect("parse controls");

    let report = check_registry(&gates, &controls);
    assert!(!report.is_ok());
    assert!(
        report
            .findings
            .iter()
            .any(|f| f.record == "GATE-GAUNT-01" && f.reason == ControlReason::OneWayLink),
        "expected one-way-link on GATE-GAUNT-01, got: {:?}",
        report.findings
    );
    assert!(
        report
            .findings
            .iter()
            .any(|f| f.record == "GATE-GAUNT-05" && f.reason == ControlReason::OneWayLink),
        "expected one-way-link on the not-run gate with an empty list, got: {:?}",
        report.findings
    );
}

#[test]
fn uncaught_control_fails_pass() {
    let gates = parse_gates(&sample_gate("GATE-GAUNT-01", "M0", "\"NC-0001\"", "pass", "sess-1"));
    let controls = Controls::parse_str(
        r#"schema = "frob.gauntlet.controls/1"

[[control]]
id = "NC-0001"
seed = "GATE-GAUNT-01"
gate = ["GATE-GAUNT-01"]
kind = "planted-input"
target = "frob-conformance::controls::check_registry"
defect = "planted empty controls list"
expect = "fails with no-control"
last_seen = { caught = false, session = "sess-1", date = "2026-11-24" }
fixture = "fixtures/a.toml"
due = "M0"
"#,
        CONTROLS_PATH,
    )
    .expect("parse controls");

    let report = check_registry(&gates, &controls);
    assert!(!report.is_ok());
    assert!(
        report
            .findings
            .iter()
            .any(|f| f.record == "GATE-GAUNT-01" && f.reason == ControlReason::UncaughtControl),
        "expected uncaught-control, got: {:?}",
        report.findings
    );
}

#[test]
fn not_run_without_control_is_not_eligible() {
    let gates = parse_gates(&sample_gate("GATE-TEST-02", "M0", "", "not-run", ""));
    let controls = Controls::default();
    let report = check_registry(&gates, &controls);
    assert!(report.is_ok(), "not-run gate without control must not fail: {:?}", report.findings);
    assert!(
        report.not_eligible.iter().any(|ne| ne.gate_id == "GATE-TEST-02"),
        "expected GATE-TEST-02 in not_eligible: {:?}",
        report.not_eligible
    );
    assert!(report.format_summary().contains("not-eligible"));
}

#[test]
fn obligation_control_does_not_support_pass() {
    // 1) Status `pass` with only an `obligation` control -> no-control.
    let gates_only_ob = parse_gates(&sample_gate("GATE-TEST-03", "M0", "\"NC-0001\"", "pass", "sess-1"));
    let ctrls_only_ob = Controls::parse_str(
        r#"schema = "frob.gauntlet.controls/1"

[[control]]
id = "NC-0001"
seed = "CX-262"
gate = ["GATE-TEST-03"]
kind = "mutation"
target = "BD-123:cx-262"
defect = "obligation pending numeric case"
expect = "row fails when implemented"
feature = "cx-262"
due = "M0"
state = "obligation"
"#,
        CONTROLS_PATH,
    )
    .expect("parse obligation control");

    let rep_fail = check_registry(&gates_only_ob, &ctrls_only_ob);
    assert!(!rep_fail.is_ok());
    assert!(
        rep_fail
            .findings
            .iter()
            .any(|f| f.record == "GATE-TEST-03" && f.reason == ControlReason::NoControl),
        "expected no-control for obligation-only pass, got: {:?}",
        rep_fail.findings
    );

    // 2) Adding a caught `runnable` control -> passes, and obligation is reported beside the gate.
    let gates_both = parse_gates(&sample_gate(
        "GATE-TEST-03",
        "M0",
        "\"NC-0001\", \"NC-0002\"",
        "pass",
        "sess-1",
    ));
    let ctrls_both = Controls::parse_str(
        r#"schema = "frob.gauntlet.controls/1"

[[control]]
id = "NC-0001"
seed = "CX-262"
gate = ["GATE-TEST-03"]
kind = "mutation"
target = "BD-123:cx-262"
defect = "obligation pending numeric case"
expect = "row fails when implemented"
feature = "cx-262"
due = "M0"
state = "obligation"

[[control]]
id = "NC-0002"
seed = "GATE-TEST-03"
gate = ["GATE-TEST-03"]
kind = "planted-input"
target = "frob-conformance::controls::check_registry"
defect = "empty controls list"
expect = "fails with no-control"
last_seen = { caught = true, session = "sess-1", date = "2026-11-24" }
fixture = "fixtures/a.toml"
due = "M0"
state = "runnable"
"#,
        CONTROLS_PATH,
    )
    .expect("parse obligation + runnable controls");

    let rep_ok = check_registry(&gates_both, &ctrls_both);
    assert!(rep_ok.is_ok(), "unexpected findings: {:?}", rep_ok.findings);
    assert!(
        rep_ok
            .obligations
            .iter()
            .any(|o| o.gate_id == "GATE-TEST-03" && o.control_id == "NC-0001"),
        "obligation must be reported beside gate: {:?}",
        rep_ok.obligations
    );
}

#[test]
fn mutation_feature_guard() {
    let root = workspace_root();
    // 1) Default feature list must NOT enable gauntlet-mutations.
    let manifest = fs::read_to_string(root.join("crates/frob-conformance/Cargo.toml"))
        .expect("read frob-conformance/Cargo.toml");
    for line in manifest.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("default") && trimmed.contains('=') {
            assert!(
                !trimmed.contains("gauntlet-mutations"),
                "default features must not enable gauntlet-mutations: {trimmed}"
            );
        }
    }

    // 2) Compile-fail check: compiling src/controls.rs under release-equivalent cfg
    // (`feature="gauntlet-mutations"`, no `debug_assertions`) MUST fail with compile_error!.
    let out_dir = scratch_dir("mutation_guard");
    let controls_rs = root.join("crates/frob-conformance/src/controls.rs");
    let rustc = env::var_os("RUSTC").unwrap_or_else(|| "rustc".into());
    let output = Command::new(rustc)
        .args([
            "--edition",
            "2024",
            "--crate-type",
            "lib",
            "--crate-name",
            "frob_conformance_guard_probe",
            "--cfg",
            "feature=\"gauntlet-mutations\"",
            "-O",
            "--out-dir",
        ])
        .arg(&out_dir)
        .arg(&controls_rs)
        .output()
        .expect("invoke rustc for compile-fail mutation feature guard");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !output.status.success(),
        "gauntlet-mutations without debug_assertions MUST fail to compile"
    );
    assert!(
        stderr.contains("gauntlet-mutations") && stderr.contains("forbidden in release builds"),
        "expected compile_error! naming gauntlet-mutations release guard, stderr:\n{stderr}"
    );
}

#[test]
fn seed_forms_closed() {
    let ws = scratch_dir("seed_forms");
    let gates = parse_gates(&sample_gate("GATE-GAUNT-10", "M0", "\"NC-0001\"", "not-run", ""));

    let good_seeds = [
        ("CX-262", false),
        ("KA-501", false),
        ("MR-14", false),
        ("MR-C07", false),
        ("CERT-07", false),
        ("INV-CHRON-14", false),
        ("PO-GAUNT-03", false),
        ("RISK-37", false),
        ("SAFE-X01", false),
        ("GATE-GAUNT-10", false),
        ("", true),
    ];
    for (seed, permanent) in good_seeds {
        let perm_line = if permanent { "permanent = true\n" } else { "" };
        let doc = format!(
            "schema = \"frob.gauntlet.controls/1\"\n\n[[control]]\nid = \"NC-0001\"\nseed = \"{seed}\"\ngate = [\"GATE-GAUNT-10\"]\nkind = \"planted-input\"\ntarget = \"frob-conformance::controls::validate\"\ndefect = \"planted defect\"\nexpect = \"fails with error\"\nfixture = \"fixtures/a.toml\"\n{perm_line}"
        );
        let res = load_and_validate(&doc, CONTROLS_PATH, None, &gates, &ws);
        assert!(res.is_ok(), "seed `{seed}` (permanent={permanent}) should load: {res:?}");
    }

    let bad_seeds = [("", false), ("BUG-7", false), ("cx-262", false), ("MR-X07", false)];
    for (seed, permanent) in bad_seeds {
        let perm_line = if permanent { "permanent = true\n" } else { "" };
        let doc = format!(
            "schema = \"frob.gauntlet.controls/1\"\n\n[[control]]\nid = \"NC-0001\"\nseed = \"{seed}\"\ngate = [\"GATE-GAUNT-10\"]\nkind = \"planted-input\"\ntarget = \"frob-conformance::controls::validate\"\ndefect = \"planted defect\"\nexpect = \"fails with error\"\nfixture = \"fixtures/a.toml\"\n{perm_line}"
        );
        let err = load_and_validate(&doc, CONTROLS_PATH, None, &gates, &ws)
            .expect_err(&format!("seed `{seed}` (permanent={permanent}) must fail"));
        assert!(
            has_reason(&err, ControlReason::UnknownSeedForm),
            "seed `{seed}` expected unknown-seed-form, got: {err:?}"
        );
    }
}

#[test]
fn due_filter_skips_future_milestone_controls() {
    // Review Note 8 (N38b-04 due filter):
    // 1) M0 pass gate carrying an M0 caught control AND an M1 uncaught control passes at M0!
    let gates = parse_gates(&sample_gate(
        "GATE-GAUNT-01",
        "M0",
        "\"NC-0001\", \"NC-0002\"",
        "pass",
        "sess-m0",
    ));
    let workspace = scratch_dir("due_filter");
    fs::create_dir_all(workspace.join("fixtures")).expect("create fixtures directory");
    fs::write(
        workspace.join("fixtures/a.toml"),
        "# synthetic\nvalue = 1\n",
    )
    .expect("write the due M0 fixture");
    let raw_controls = r#"schema = "frob.gauntlet.controls/1"

[[control]]
id = "NC-0001"
seed = "GATE-GAUNT-01"
gate = ["GATE-GAUNT-01"]
kind = "planted-input"
target = "frob-conformance::controls::check_registry"
defect = "m0 caught defect"
expect = "fails with no-control"
last_seen = { caught = true, session = "sess-m0", date = "2026-11-24" }
fixture = "fixtures/a.toml"
due = "M0"

[[control]]
id = "NC-0002"
seed = "CX-262"
gate = ["GATE-GAUNT-01"]
kind = "mutation"
target = "BD-123:cx-262"
defect = "m1 seeded control not yet run at M0"
expect = "fails row with spectral radius 1.497"
feature = "cx-262"
due = "M1"
"#;
    let controls = load_and_validate(raw_controls, CONTROLS_PATH, None, &gates, &workspace)
        .expect("the unbuilt M1 plant must load at M0");

    let rep = check_registry(&gates, &controls);
    assert!(
        rep.is_ok(),
        "M1 control must not fail M0 pass gate: {:?}",
        rep.findings
    );

    let gates_at_m1 = parse_gates(&sample_gate(
        "GATE-GAUNT-01",
        "M1",
        "\"NC-0001\", \"NC-0002\"",
        "pass",
        "sess-m1",
    ));
    let findings = load_and_validate(raw_controls, CONTROLS_PATH, None, &gates_at_m1, &workspace)
        .expect_err("the missing plant must be refused once due");
    assert!(findings.iter().any(|finding| {
        finding.record == "NC-0002" && finding.reason == ControlReason::FeatureMissing
    }));

    // 2) M0 not-run part whose controls are ALL due at M1 -> reported not-eligible.
    let gates_future_only = parse_gates(&sample_gate("GATE-WS-07", "M0", "\"NC-0002\"", "not-run", ""));
    let controls_future_only = Controls::parse_str(
        r#"schema = "frob.gauntlet.controls/1"

[[control]]
id = "NC-0002"
seed = "CX-262"
gate = ["GATE-WS-07"]
kind = "mutation"
target = "BD-123:cx-262"
defect = "m1 seeded control"
expect = "fails row with 1.497"
feature = "cx-262"
due = "M1"
"#,
        CONTROLS_PATH,
    )
    .expect("parse future-only control");
    let rep_ne = check_registry(&gates_future_only, &controls_future_only);
    assert!(rep_ne.is_ok(), "not-run future-only must not fail: {:?}", rep_ne.findings);
    assert!(
        rep_ne.not_eligible.iter().any(|ne| ne.gate_id == "GATE-WS-07"),
        "M0 part with only M1 controls must be not-eligible: {:?}",
        rep_ne.not_eligible
    );

    // 3) Same M0 part marked `pass` when all its controls are due at M1 -> fails with no-control.
    let gates_future_pass = parse_gates(&sample_gate("GATE-WS-07", "M0", "\"NC-0002\"", "pass", "sess-m0"));
    let rep_fail = check_registry(&gates_future_pass, &controls_future_only);
    assert!(
        has_reason(&rep_fail.findings, ControlReason::NoControl),
        "M0 pass with only M1 controls must fail with no-control: {:?}",
        rep_fail.findings
    );
}

#[test]
fn gate_order_invariance_metamorphic() {
    let g_a = sample_gate("GATE-GAUNT-01", "M0", "\"NC-0001\"", "pass", "sess-1");
    let g_b = sample_gate("GATE-GAUNT-05", "M0", "", "not-run", "");
    let g_c = sample_gate("GATE-GAUNT-10", "M0", "\"NC-9999\"", "not-run", "");

    let gates_abc = parse_gates(&format!("{g_a}\n{g_b}\n{g_c}"));
    let gates_cba = parse_gates(&format!("{g_c}\n{g_b}\n{g_a}"));

    let controls = Controls::parse_str(
        r#"schema = "frob.gauntlet.controls/1"

[[control]]
id = "NC-0001"
seed = "GATE-GAUNT-01"
gate = ["GATE-GAUNT-01"]
kind = "planted-input"
target = "frob-conformance::controls::check_registry"
defect = "planted empty control list"
expect = "fails with no-control"
last_seen = { caught = true, session = "sess-1", date = "2026-11-24" }
fixture = "fixtures/a.toml"
due = "M0"
"#,
        CONTROLS_PATH,
    )
    .expect("parse controls");

    let r1 = check_registry(&gates_abc, &controls);
    let r2 = check_registry(&gates_cba, &controls);
    assert_eq!(r1, r2, "reordering [[gate]] rows must not change check verdict");
    assert_eq!(r1.format_summary(), r2.format_summary(), "D0 output must be byte-identical");

    // Also confirm validate() is invariant to gate row ordering.
    let ws = scratch_dir("metamorphic_validate");
    let f1 = validate(&controls, None, &gates_abc, &ws);
    let f2 = validate(&controls, None, &gates_cba, &ws);
    assert_eq!(f1, f2);
}

// ============================================================================
// 3. Registry-bound M0 control tests (STD-TEST Rule 4: NC-0001, NC-0002, NC-0004)
// ============================================================================

#[test]
fn nc_0001_empty_controls_fails_pass() {
    let gates = parse_gates(&sample_gate("GATE-GAUNT-01", "M0", "", "pass", "sess-m0"));
    let report = check_registry(&gates, &Controls::default());
    assert!(
        report
            .findings
            .iter()
            .any(|f| f.record == "GATE-GAUNT-01" && f.reason == ControlReason::NoControl),
        "NC-0001: expected no-control on empty-controls pass gate, got: {:?}",
        report.findings
    );
}

#[test]
fn nc_0002_missing_fixture_header_refused() {
    let ws = scratch_dir("nc_0002_header");
    fs::create_dir_all(ws.join("fixtures/gauntlet")).expect("mkdir fixtures/gauntlet");
    fs::write(
        ws.join("fixtures/gauntlet/v0_missing_fixture_header.toml"),
        "value = 42\n",
    )
    .expect("write unheaded fixture");
    let gates = parse_gates(&sample_gate(
        "GATE-GAUNT-05",
        "M0",
        "\"NC-0002\"",
        "pass",
        "sess-m0",
    ));
    let ctrls_toml = r#"schema = "frob.gauntlet.controls/1"

[[control]]
id = "NC-0002"
seed = "GATE-GAUNT-05"
gate = ["GATE-GAUNT-05"]
kind = "planted-input"
target = "frob-conformance::controls::validate"
defect = "planted fixture file lacking kind header on an active gate"
expect = "controls validator fails with fixture-missing"
fixture = "fixtures/gauntlet/v0_missing_fixture_header.toml"
due = "M0"
"#;
    let err = load_and_validate(ctrls_toml, CONTROLS_PATH, None, &gates, &ws)
        .expect_err("NC-0002: fixture without kind header on pass gate must fail");
    assert!(
        has_reason(&err, ControlReason::FixtureMissing),
        "NC-0002: expected fixture-missing, got: {err:?}"
    );
}

#[test]
fn nc_0004_uncaught_control_on_pass_refused() {
    let gates = parse_gates(&sample_gate(
        "GATE-GAUNT-10",
        "M0",
        "\"NC-0004\"",
        "pass",
        "sess-m0",
    ));
    let ctrls = Controls::parse_str(
        r#"schema = "frob.gauntlet.controls/1"

[[control]]
id = "NC-0004"
seed = "GATE-GAUNT-10"
gate = ["GATE-GAUNT-10"]
kind = "planted-input"
target = "frob-conformance::controls::check_registry"
defect = "planted pass gate whose runnable control has last_seen.caught = false"
expect = "registry_check fails with uncaught-control"
last_seen = { caught = false, session = "sess-m0", date = "2026-11-24" }
fixture = "fixtures/gauntlet/v0_uncaught_control_pass.toml"
due = "M0"
"#,
        CONTROLS_PATH,
    )
    .expect("parse NC-0004");
    let report = check_registry(&gates, &ctrls);
    assert!(
        report
            .findings
            .iter()
            .any(|f| f.record == "GATE-GAUNT-10" && f.reason == ControlReason::UncaughtControl),
        "NC-0004: expected uncaught-control on pass gate, got: {:?}",
        report.findings
    );
}
