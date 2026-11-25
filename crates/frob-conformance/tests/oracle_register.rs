#![deny(unsafe_code)]

//! Unit and integration tests for the oracle register, role grammar, pin-move
//! audit, pinned fixtures, and disagreement findings (`N38a-04`, `GATE-GAUNT-05`).
//!
//! Run as `cargo test -p frob-conformance --test oracle_register`.
//! Supports `FROB_ORACLE_ROOT=<dir>` and `FROB_ORACLE_BASELINE_HASH=<rel>=<sha256>`
//! for scratch plants in `scripts/e2e/oracle_register.sh`.
//! Never deletes files or directories (COMMON.md 9b).

use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use frob_conformance::oracles::{
    AdmissionStatus, COMPARATORS_PATH, DEVELOPMENT_ORACLE_ID, DisagreementVerdict, FixtureStatus,
    HELD_OUT_ORACLE_IDS, ORACLE_FINDINGS_PATH, ORACLES_PATH, OracleErrorKind, OracleMeasurement,
    OracleRegister, SEED_ORACLE_IDS, check_capture_script_inputs, compute_fixture_bytes_sha256,
    fixture_supports_gate, load_findings, parse_and_verify_fixture, parse_role,
    record_disagreement_if_any, replace_bytes_sha256_line, scan_and_verify_fixtures,
    verify_fixture_against_workspace, verify_fixture_change_against_baseline,
};
use frob_core::testkit::registry;

static SCRATCH_SEQ: AtomicU64 = AtomicU64::new(0);

fn workspace_root() -> PathBuf {
    if let Some(ov) = env::var_os("FROB_ORACLE_ROOT") {
        return PathBuf::from(ov);
    }
    registry::root().expect("locate workspace root via franken-stack.lock")
}

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
        .join(format!("oracle_register-{label}-{nanos}-{seq}"));
    fs::create_dir_all(&dir).expect("create scratch dir");
    dir
}

// ============================================================================
// Clean-tree validator (also driven by scripts/e2e/oracle_register.sh)
// ============================================================================

#[test]
fn oracle_register_clean_tree() {
    let root = workspace_root();
    let reg = OracleRegister::load(&root).expect("load crates/frob-conformance/registry/oracles.toml");
    let findings = load_findings(&root.join(ORACLE_FINDINGS_PATH))
        .expect("load crates/frob-conformance/registry/oracle_findings.toml");
    let fixtures = scan_and_verify_fixtures(&root, &reg).unwrap_or_else(|errs| {
        let rendered: Vec<String> = errs.iter().map(ToString::to_string).collect();
        panic!(
            "oracle fixture verification failed on {}:\n{}",
            root.display(),
            rendered.join("\n")
        );
    });

    // Optional baseline hash check used by scripts/e2e/oracle_register.sh for regen-from-candidate.
    if let Ok(spec) = env::var("FROB_ORACLE_BASELINE_HASH")
        && let Some((rel, base_hash)) = spec.split_once('=')
    {
        let fx = fixtures
            .iter()
            .find(|f| f.rel_path == rel)
            .unwrap_or_else(|| panic!("baseline fixture `{rel}` not found in scanned fixtures"));
        verify_fixture_change_against_baseline(&reg, fx, base_hash)
            .unwrap_or_else(|e| panic!("baseline fixture change check failed: {e}"));
    }

    let mut per_oracle: BTreeMap<&str, usize> = BTreeMap::new();
    let mut active_count = 0usize;
    let mut disputed_count = 0usize;
    for fx in &fixtures {
        *per_oracle.entry(fx.oracle.as_str()).or_insert(0) += 1;
        match fx.status {
            FixtureStatus::Active => active_count += 1,
            FixtureStatus::Disputed => disputed_count += 1,
        }
        println!(
            "oracle_fixture: path={} oracle={} status={} bytes_sha256={}",
            fx.rel_path,
            fx.oracle,
            fx.status.as_str(),
            fx.bytes_sha256
        );
    }
    println!(
        "oracle_register: oracles={} fixtures={} active={} disputed={} findings={}",
        reg.oracles.len(),
        fixtures.len(),
        active_count,
        disputed_count,
        findings.len()
    );
    for (oid, cnt) in &per_oracle {
        println!("oracle_fixture_count: oracle={oid} count={cnt}");
    }
}

// ============================================================================
// N38a-04 Test Plan unit tests
// ============================================================================

#[test]
fn register_has_seed_rows() {
    let root = workspace_root();
    let reg = OracleRegister::load(&root).expect("load real oracles.toml");

    // 1. All twelve seed rows are present.
    for seed_id in SEED_ORACLE_IDS {
        assert!(
            reg.get(seed_id).is_some(),
            "missing required seed oracle row `{seed_id}`"
        );
    }

    // 2. Both standing held-out rows are present with non-empty reasons and no roles.
    for ho_id in HELD_OUT_ORACLE_IDS {
        let row = reg
            .get(ho_id)
            .unwrap_or_else(|| panic!("missing held-out row `{ho_id}`"));
        assert!(!row.held_out.is_empty(), "held_out reason empty on `{ho_id}`");
        assert!(row.roles.is_empty(), "held_out row `{ho_id}` must have no roles");
        assert_eq!(row.admission.status, AdmissionStatus::NotAdmitted);
    }

    // 3. `incumbent-batched-simulators` names `comparators.toml` as its pin, and N38a-04
    // does not create `comparators.toml` (BD-039 owns that file).
    let inc = reg
        .get("incumbent-batched-simulators")
        .expect("incumbent-batched-simulators row");
    assert_eq!(inc.pin.value, COMPARATORS_PATH);
    assert!(
        !root.join(COMPARATORS_PATH).exists(),
        "N38a-04 must not create comparators.toml (owned by BD-039)"
    );

    // 4. Default register has no development-oracle row (N38b-35 default).
    assert!(reg.get(DEVELOPMENT_ORACLE_ID).is_none());

    // 5. Scratch copy with `development-oracle` in N38b-35's shape loads cleanly.
    let base_toml = fs::read_to_string(root.join(ORACLES_PATH)).expect("read oracles.toml");
    let with_dev = format!(
        r#"{base_toml}
[[oracle]]
id = "development-oracle"
posture = "out-of-process"
pin = {{ kind = "release", value = "3.2.7", commit = "0123456789abcdef0123456789abcdef01234567", sha256 = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef" }}
admission = {{ test = "g2:dev-oracle-pendulum", control = "NC-0002", status = "not-admitted", date = "" }}
roles = ["pilot"]
held_out = ""
tool = "mujoco-c-f64"
label = "development-oracle"
decided = "2026-11-24"
decision = "ADR-OPEN-GAUNT-4"
"#
    );
    let parsed_dev = OracleRegister::parse_str(&with_dev, ORACLES_PATH)
        .expect("register with valid development-oracle row must load");
    let dev_row = parsed_dev
        .get(DEVELOPMENT_ORACLE_ID)
        .expect("development-oracle present");
    assert_eq!(
        dev_row.dev_meta.as_ref().map(|m| m.label.as_str()),
        Some("development-oracle")
    );

    // 6. Those four fields on any other row are a load error.
    for extra_field in [
        "tool = \"other\"",
        "label = \"development-oracle\"",
        "decided = \"2026-11-24\"",
        "decision = \"ADR-1\"",
    ] {
        let bad = format!(
            r#"{base_toml}
[[oracle]]
id = "extra-solver"
posture = "out-of-process"
pin = {{ kind = "release", value = "1.0.0", sha256 = "" }}
admission = {{ test = "g2:extra", control = "NC-0002", status = "not-admitted", date = "" }}
roles = ["comparator:extra"]
family = "conventional-solvers"
held_out = ""
{extra_field}
"#
        );
        let err = OracleRegister::parse_str(&bad, ORACLES_PATH)
            .expect_err(&format!("field `{extra_field}` on non-dev row must fail"));
        assert_eq!(
            err.kind,
            OracleErrorKind::DevOracleFieldNotAllowed,
            "unexpected error for `{extra_field}`: {err}"
        );
    }
}

#[test]
fn posture_values_closed() {
    let bad = r#"schema = "frob.gauntlet.oracles/1"

[[oracle]]
id = "bad-posture-oracle"
posture = "embedded-magic"
pin = { kind = "release", value = "1.0.0", sha256 = "" }
admission = { test = "g2:x", control = "NC-0002", status = "not-admitted", date = "" }
roles = ["parity:x"]
held_out = ""
"#;
    let err = OracleRegister::parse_str(bad, ORACLES_PATH).expect_err("unknown posture must fail");
    assert_eq!(err.kind, OracleErrorKind::InvalidPosture);
    assert!(err.subject.contains("bad-posture-oracle"), "{err}");

    // Also verify floating tag (`latest` / `main`) is rejected.
    let floating = bad
        .replace("embedded-magic", "out-of-process")
        .replace("1.0.0", "latest");
    let err_fl = OracleRegister::parse_str(&floating, ORACLES_PATH)
        .expect_err("floating release tag must fail");
    assert_eq!(err_fl.kind, OracleErrorKind::FloatingPin);
}

#[test]
fn role_grammar() {
    for valid in [
        "parity:contact-free-dynamics",
        "import-fixtures",
        "pilot",
        "comparator:hybrid-reachability",
        "comparator:label-query",
        "second-checker:exact-interval",
        "second-checker:bounded-linf-residual",
        "comparator:ppo-update",
        "reference:planar-drive",
    ] {
        parse_role(valid, "row-ok").unwrap_or_else(|e| panic!("valid role `{valid}` failed: {e}"));
    }

    for invalid in ["benchmark:speed", "Parity:x", "comparator:", "parity:Upper-Case"] {
        let err = parse_role(invalid, "row-bad")
            .expect_err(&format!("invalid role `{invalid}` must fail"));
        assert_eq!(err.kind, OracleErrorKind::InvalidRole, "got: {err}");
        assert_eq!(err.subject, "row-bad");
    }

    // Confirm load-time error names the offending row in oracles.toml.
    let bad_toml = r#"schema = "frob.gauntlet.oracles/1"

[[oracle]]
id = "bad-role-row"
posture = "out-of-process"
pin = { kind = "release", value = "1.0.0", sha256 = "" }
admission = { test = "g2:x", control = "NC-0002", status = "not-admitted", date = "" }
roles = ["benchmark:speed"]
held_out = ""
"#;
    let err = OracleRegister::parse_str(bad_toml, ORACLES_PATH).expect_err("bad role must fail");
    assert_eq!(err.kind, OracleErrorKind::InvalidRole);
    assert_eq!(err.subject, "bad-role-row");
}

#[test]
fn appended_row_admitted() {
    let root = workspace_root();
    let base = fs::read_to_string(root.join(ORACLES_PATH)).expect("read oracles.toml");

    let appended = format!(
        r#"{base}
[[oracle]]
id = "z3-label-query"
posture = "out-of-process"
pin = {{ kind = "release", value = "4.13.3", sha256 = "" }}
admission = {{ test = "g2:z3-label-query", control = "NC-0002", status = "not-admitted", date = "" }}
roles = ["comparator:label-query"]
family = "conventional-solvers"
held_out = ""
"#
    );
    let reg = OracleRegister::parse_str(&appended, ORACLES_PATH)
        .expect("appended row specializing conventional-solvers must load");
    let row = reg.get("z3-label-query").expect("appended row present");
    assert_eq!(row.family.as_deref(), Some("conventional-solvers"));
    assert_eq!(row.admission.status, AdmissionStatus::NotAdmitted);

    // Must be refused until admitted.
    let err = reg
        .request_role("z3-label-query", "comparator:label-query", true)
        .expect_err("not-admitted appended row must be refused");
    assert_eq!(err.kind, OracleErrorKind::NotAdmitted);

    // Unknown family fails load.
    let bad_fam = appended.replace("conventional-solvers", "no-such-row");
    let err_fam = OracleRegister::parse_str(&bad_fam, ORACLES_PATH)
        .expect_err("unknown family must fail");
    assert_eq!(err_fam.kind, OracleErrorKind::UnknownFamily);
    assert_eq!(err_fam.subject, "z3-label-query");
}

#[test]
fn fixture_header_required() {
    let no_header = "[payload]\nperiod_s = \"2.0077\"\n";
    let err = parse_and_verify_fixture(no_header, "crates/frob-conformance/fixtures/no_header.toml")
        .expect_err("fixture without header must fail");
    assert_eq!(err.kind, OracleErrorKind::FixtureHeaderMissing);

    let wrong_kind = "kind = \"synthetic\"\noracle = \"analytic-derivations\"\n";
    let err2 = parse_and_verify_fixture(wrong_kind, "crates/frob-conformance/fixtures/wrong_kind.toml")
        .expect_err("non-oracle kind must fail");
    assert_eq!(err2.kind, OracleErrorKind::FixtureHeaderMissing);
}

#[test]
fn fixture_bytes_match_hash() {
    let root = workspace_root();
    let fx_rel = "crates/frob-conformance/fixtures/pendulum_period.toml";
    let original = fs::read_to_string(root.join(fx_rel)).expect("read pendulum_period.toml");

    // Clean fixture verifies cleanly.
    let fx = parse_and_verify_fixture(&original, fx_rel).expect("clean fixture must verify");
    assert_eq!(fx.oracle, "analytic-derivations");

    // Flip one byte in the payload -> detected!
    let flipped = original.replacen("theta0_rad = \"0.1\"", "theta0_rad = \"0.2\"", 1);
    assert_ne!(flipped, original);
    let err = parse_and_verify_fixture(&flipped, fx_rel)
        .expect_err("one flipped byte in payload must fail");
    assert_eq!(err.kind, OracleErrorKind::FixtureHashMismatch);

    // Flip one byte in the header -> also detected!
    let flipped_hdr = original.replacen("captured = \"2026-11-24\"", "captured = \"2026-11-25\"", 1);
    let err_hdr = parse_and_verify_fixture(&flipped_hdr, fx_rel)
        .expect_err("one flipped byte in header must fail");
    assert_eq!(err_hdr.kind, OracleErrorKind::FixtureHashMismatch);
}

#[test]
fn changed_fixture_needs_pin_move() {
    let root = workspace_root();
    let reg = OracleRegister::load(&root).expect("load oracles.toml");
    let fx_rel = "crates/frob-conformance/fixtures/pendulum_period.toml";
    let orig_txt = fs::read_to_string(root.join(fx_rel)).expect("read fixture");
    let orig_fx = parse_and_verify_fixture(&orig_txt, fx_rel).expect("parse original");

    // Create a modified & re-signed fixture (simulating candidate regeneration or recapture).
    let mutated_body = orig_txt
        .replacen("theta0_rad = \"0.1\"", "theta0_rad = \"0.2\"", 1)
        .replacen("pin = \"docs/derivations/\"", "pin = \"docs/derivations/v2/\"", 1);
    let new_hash = compute_fixture_bytes_sha256(&mutated_body);
    let mutated_signed = replace_bytes_sha256_line(&mutated_body, &new_hash);
    let mutated_fx = parse_and_verify_fixture(&mutated_signed, fx_rel).expect("re-signed fixture parses");
    assert_ne!(mutated_fx.bytes_sha256, orig_fx.bytes_sha256);

    // Without a [[pin_move]] record -> fails!
    let err = verify_fixture_change_against_baseline(&reg, &mutated_fx, &orig_fx.bytes_sha256)
        .expect_err("changed fixture without [[pin_move]] must fail");
    assert_eq!(err.kind, OracleErrorKind::FixtureChangedWithoutPinMove);

    // With an appended [[pin_move]] record listing the fixture -> passes!
    let mut moved_toml = fs::read_to_string(root.join(ORACLES_PATH)).expect("read oracles.toml");
    moved_toml = moved_toml.replacen(
        "value = \"docs/derivations/\"",
        "value = \"docs/derivations/v2/\"",
        1,
    );
    moved_toml.push_str(
        r#"
[[pin_move]]
oracle = "analytic-derivations"
from = "docs/derivations/"
to = "docs/derivations/v2/"
date = "2026-11-24"
reason = "recapture pendulum fixture with v2 elliptic series"
by = "session:reviewer"
fixtures_recaptured = ["crates/frob-conformance/fixtures/pendulum_period.toml"]
"#,
    );
    let moved_reg = OracleRegister::parse_str(&moved_toml, ORACLES_PATH).expect("parse moved register");
    verify_fixture_change_against_baseline(&moved_reg, &mutated_fx, &orig_fx.bytes_sha256)
        .expect("changed fixture with matching [[pin_move]] must pass");
}

#[test]
fn capture_inputs_exclude_workspace() {
    // 1. Capture script declaring `target/release/frob` as input is rejected.
    let bad_script = "#!/usr/bin/env bash\n# inputs: target/release/frob\necho ok\n";
    let err = check_capture_script_inputs(bad_script, "tools/oracle/bad/script.sh")
        .expect_err("script declaring target/release/frob must be rejected");
    assert_eq!(err.kind, OracleErrorKind::WorkspaceInputForbidden);

    // 2. Fixture declaring `target/release/frob` in `inputs` is also rejected against workspace.
    let root = workspace_root();
    let reg = OracleRegister::load(&root).expect("load oracles.toml");
    let fx_rel = "crates/frob-conformance/fixtures/pendulum_period.toml";
    let orig = fs::read_to_string(root.join(fx_rel)).expect("read fixture");
    let poisoned_body = orig.replacen(
        "inputs = []",
        "inputs = [{ path = \"target/release/frob\", sha256 = \"0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef\" }]",
        1,
    );
    let h = compute_fixture_bytes_sha256(&poisoned_body);
    let poisoned = replace_bytes_sha256_line(&poisoned_body, &h);
    let fx = parse_and_verify_fixture(&poisoned, fx_rel).expect("poisoned fixture parses");
    let err2 = verify_fixture_against_workspace(&root, &reg, &fx)
        .expect_err("fixture naming target/release/frob as input must fail");
    assert_eq!(err2.kind, OracleErrorKind::WorkspaceInputForbidden);
}

#[test]
fn held_out_refused() {
    let root = workspace_root();
    let reg = OracleRegister::load(&root).expect("load oracles.toml");

    // Requesting `planar_drive` (alias or canonical id) as a parity oracle returns the held-out reason.
    for name in ["planar_drive", "fs-mbd-planar-drive"] {
        let err = reg
            .request_role(name, "parity:wheeled-kinematics", true)
            .expect_err(&format!("held-out `{name}` must be refused"));
        assert_eq!(err.kind, OracleErrorKind::HeldOut);
        assert!(
            err.message.contains("INT-FSIM-05") && err.message.contains("planar_drive"),
            "held-out reason should cite INT-FSIM-05 and planar_drive: {err}"
        );
    }

    // Requesting `fs-g1-train-ppo-update` / `fs-g1-train` is likewise refused with INT-FSIM-06.
    for name in ["fs-g1-train-ppo-update", "fs-g1-train"] {
        let err = reg
            .request_role(name, "comparator:ppo-update", true)
            .expect_err(&format!("held-out `{name}` must be refused"));
        assert_eq!(err.kind, OracleErrorKind::HeldOut);
        assert!(err.message.contains("INT-FSIM-06"), "{err}");
    }
}

#[test]
fn oracle_pin_move_needs_record() {
    let root = workspace_root();
    let prev_txt = fs::read_to_string(root.join(ORACLES_PATH)).expect("read oracles.toml");
    let prev = OracleRegister::parse_str(&prev_txt, ORACLES_PATH).expect("parse prev");

    // Edit mujoco-c-f64 pin.value without appending [[pin_move]] -> fails!
    let edited_no_move = prev_txt.replacen("value = \"3.2.7\"", "value = \"3.3.0\"", 1);
    let curr_no_move = OracleRegister::parse_str(&edited_no_move, ORACLES_PATH).expect("parse curr");
    let err = curr_no_move
        .validate_against_previous(&prev)
        .expect_err("pin change without [[pin_move]] must fail");
    assert_eq!(err.kind, OracleErrorKind::PinMoveMissing);
    assert_eq!(err.subject, "mujoco-c-f64");

    // Append [[pin_move]] with `by` and `fixtures_recaptured` -> passes!
    let edited_with_move = format!(
        r#"{edited_no_move}
[[pin_move]]
oracle = "mujoco-c-f64"
from = "3.2.7"
to = "3.3.0"
date = "2026-11-24"
reason = "upgrade MuJoCo release after parity check"
by = "session:reviewer"
fixtures_recaptured = ["tests/fixtures/oracle/mujoco_pendulum.toml"]
"#
    );
    let curr_with_move =
        OracleRegister::parse_str(&edited_with_move, ORACLES_PATH).expect("parse with pin_move");
    curr_with_move
        .validate_against_previous(&prev)
        .expect("pin move with complete [[pin_move]] record must pass");

    // Editing a previously recorded [[pin_move]] is refused (append-only).
    let tampered_move = edited_with_move.replace("session:reviewer", "session:rewriter");
    let curr_tampered =
        OracleRegister::parse_str(&tampered_move, ORACLES_PATH).expect("parse tampered");
    let err_edit = curr_tampered
        .validate_against_previous(&curr_with_move)
        .expect_err("editing a previous [[pin_move]] must fail");
    assert_eq!(err_edit.kind, OracleErrorKind::PinMoveEdited);
}

#[test]
fn disagreement_is_finding() {
    let root = workspace_root();
    let ws = scratch_dir("disagreement");
    let fx_rel = "crates/frob-conformance/fixtures/pendulum_period.toml";
    let fx_dst = ws.join(fx_rel);
    fs::create_dir_all(fx_dst.parent().unwrap()).expect("mkdir fixtures");
    fs::copy(root.join(fx_rel), &fx_dst).expect("copy fixture");
    let findings_dst = ws.join(ORACLE_FINDINGS_PATH);

    let obs_a = OracleMeasurement {
        oracle_id: "mujoco-c-f64".to_owned(),
        pin: "3.2.7".to_owned(),
        value: 2.0061,
        tolerance: 1e-4,
        units: "s".to_owned(),
    };
    let obs_b = OracleMeasurement {
        oracle_id: "frankenscipy-solve-ivp".to_owned(),
        pin: "0.9.1".to_owned(),
        value: 2.0101,
        tolerance: 1e-4,
        units: "s".to_owned(),
    };

    let verdict = record_disagreement_if_any(
        fx_rel,
        "pendulum_period_s",
        &obs_a,
        &obs_b,
        &findings_dst,
        Some(&fx_dst),
    )
    .expect("record disagreement");

    match verdict {
        DisagreementVerdict::Disputed(ref f) => {
            assert_eq!(f.pin_a, "3.2.7");
            assert_eq!(f.pin_b, "0.9.1");
            assert_eq!(f.status, "disputed");
        }
        DisagreementVerdict::Agreed => panic!("2.0061 vs 2.0101 at tol 1e-4 must be disputed"),
    }

    // Verify finding persisted in oracle_findings.toml with both pins.
    let loaded_findings = load_findings(&findings_dst).expect("reload findings");
    assert_eq!(loaded_findings.len(), 1);
    assert_eq!(loaded_findings[0].pin_a, "3.2.7");
    assert_eq!(loaded_findings[0].pin_b, "0.9.1");
    assert_eq!(loaded_findings[0].status, "disputed");

    // Verify fixture on disk is marked `disputed`, stays hash-valid, and supports no gate.
    let updated_txt = fs::read_to_string(&fx_dst).expect("read disputed fixture");
    let updated_fx =
        parse_and_verify_fixture(&updated_txt, fx_rel).expect("disputed fixture remains hash-valid");
    assert_eq!(updated_fx.status, FixtureStatus::Disputed);
    assert!(!fixture_supports_gate(&updated_fx, &loaded_findings));
}

#[test]
fn determinism_d0_register_and_fixtures() {
    let root = workspace_root();
    let r1 = OracleRegister::load(&root).expect("load 1");
    let r2 = OracleRegister::load(&root).expect("load 2");
    assert_eq!(r1, r2);
    let f1 = scan_and_verify_fixtures(&root, &r1).expect("scan 1");
    let f2 = scan_and_verify_fixtures(&root, &r2).expect("scan 2");
    assert_eq!(f1, f2);
}
