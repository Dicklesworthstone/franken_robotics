#![forbid(unsafe_code)]

//! Referee dynamics, canonical digest, oracle parity, perturbation controls,
//! capture-record authentication, refusal validation, repeatability, and bit-flip
//! injectivity tests (`BD-007`, `INT-FSIM-01`).

use std::fs;
use std::path::{Path, PathBuf};

use frob_core::registry::toml::{self, Table, Value};
use frob_fsim::referee::digest;

#[cfg(feature = "referee")]
use std::env;
#[cfg(feature = "referee")]
use std::process::Command;

#[cfg(feature = "referee")]
use frob_fsim::referee::{
    CatalogRobot, InProcRefereeDynamics, Rational, RefereeDynamics, RefereeRefusalKind,
    RefereeState, new_referee_dynamics,
};

/// Independently computed BLAKE3 of the exact 50-byte synthetic layout
/// (`links=1, nq=1, nv=1, q=[1.0], v=[-0.0], tag=0x00`; verified with `b3sum`).
const SYNTHETIC_NEG_ZERO_DIGEST: &str =
    "8b709fdf8d41050ac2efa7d32f266637318db7a15e9d95be128822de6ebe0893";

/// Independently computed BLAKE3 of the same 50-byte synthetic layout with
/// `v=[+0.0]` instead of `v=[-0.0]` (verified with `b3sum`).
const SYNTHETIC_POS_ZERO_DIGEST: &str =
    "9acf26e2bdc3aab4b074b73845a65de73a5640eef11b7e18a800af6d71786851";

#[cfg(feature = "referee")]
const ROLLOUT_STEPS_2S: usize = 960;

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crates/frob-fsim must sit two levels below workspace root")
        .to_path_buf()
}

fn fixture_path() -> PathBuf {
    workspace_root().join("tests/fixtures/oracle/fsim/g1_fd_digest.toml")
}

fn capture_log_path() -> PathBuf {
    workspace_root().join("tests/fixtures/oracle/fsim/g1_fd_digest.capture.log")
}

fn parse_toml_bytes(bytes: &[u8], _label: &str) -> Table {
    toml::parse(bytes).expect("failed parsing TOML fixture")
}

fn parse_toml_file(path: &Path) -> Table {
    let bytes = fs::read(path).expect("failed reading TOML fixture file");
    parse_toml_bytes(&bytes, &path.display().to_string())
}

fn read_lock_fsim_pin(root: &Path) -> String {
    let lock = parse_toml_file(&root.join("franken-stack.lock"));
    let foundations = lock
        .get("foundation")
        .and_then(Value::as_array)
        .expect("franken-stack.lock must contain `[[foundation]]`");
    foundations
        .iter()
        .filter_map(|item| item.value.as_table())
        .find(|tbl| tbl.get("id").and_then(Value::as_str) == Some("FD-FSIM"))
        .and_then(|tbl| tbl.get("git_rev").and_then(Value::as_str))
        .expect("franken-stack.lock missing [[foundation]] entry with id = \"FD-FSIM\" and git_rev")
        .to_owned()
}

fn require_fixture_digest_for_pin(
    fixture: &Table,
    lock_pin: &str,
    field: &str,
) -> Result<String, String> {
    let fixture_pin = fixture
        .get("pin")
        .and_then(Value::as_str)
        .ok_or_else(|| format!("capture missing for pin {lock_pin}: fixture lacks `pin`"))?;
    if fixture_pin != lock_pin {
        return Err(format!(
            "capture missing for pin {lock_pin}: fixture pin is {fixture_pin}"
        ));
    }
    let val = fixture
        .get(field)
        .and_then(Value::as_str)
        .filter(|s| s.len() == 64 && s.bytes().all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f')))
        .ok_or_else(|| {
            format!("capture missing for pin {lock_pin}: fixture lacks valid 64-hex `{field}`")
        })?;
    Ok(val.to_owned())
}

fn find_kv_line<'a>(log_text: &'a str, key: &str) -> Option<&'a str> {
    let prefix = format!("{key}=");
    for line in log_text.lines() {
        if let Some(val) = line.strip_prefix(&prefix) {
            return Some(val.trim());
        }
    }
    None
}

fn verify_capture_record(fixture: &Table, log_bytes: &[u8]) -> Result<(), String> {
    let declared_log_b3 = fixture
        .get("capture_log_blake3")
        .and_then(Value::as_str)
        .ok_or_else(|| "fixture missing `capture_log_blake3`".to_owned())?;
    let actual_log_b3 = digest::blake3_hex(log_bytes);
    if actual_log_b3 != declared_log_b3 {
        return Err(format!(
            "capture_log_blake3 mismatch: fixture declares {declared_log_b3}, actual log is {actual_log_b3}"
        ));
    }

    let log_text = std::str::from_utf8(log_bytes)
        .map_err(|e| format!("capture log is not valid UTF-8: {e}"))?;

    for key in ["pin", "toolchain", "fd_digest", "rollout_2s_digest"] {
        let fix_val = fixture
            .get(key)
            .and_then(Value::as_str)
            .ok_or_else(|| format!("fixture missing `{key}`"))?;
        let log_val = find_kv_line(log_text, key)
            .ok_or_else(|| format!("capture log missing `{key}=...` line"))?;
        if log_val != fix_val {
            return Err(format!(
                "capture record mismatch for `{key}`: fixture `{fix_val}` != log `{log_val}`"
            ));
        }
    }

    let rustc_ver = fixture
        .get("rustc_version")
        .and_then(Value::as_str)
        .ok_or_else(|| "fixture missing `rustc_version`".to_owned())?;
    if !log_text.lines().any(|line| line.trim() == rustc_ver) {
        return Err(format!(
            "capture log missing rustc -vV header line matching `{rustc_ver}`"
        ));
    }

    Ok(())
}

// =========================================================================
// Synthetic & pure digest tests (run with or without `referee` feature)
// =========================================================================

#[test]
fn digest_layout_known_answer() {
    let q = [1.0_f64];
    let v = [-0.0_f64];
    let bytes = digest::encode_state_bytes(1, &q, &v, None);
    assert_eq!(
        bytes.len(),
        50,
        "21-byte tag + 12-byte dims + 16-byte floats + 1-byte payload tag must be 50 bytes"
    );

    let mut expected_bytes = Vec::with_capacity(50);
    expected_bytes.extend_from_slice(b"frob.referee-state/1\0");
    expected_bytes.extend_from_slice(&1_u32.to_le_bytes());
    expected_bytes.extend_from_slice(&1_u32.to_le_bytes());
    expected_bytes.extend_from_slice(&1_u32.to_le_bytes());
    expected_bytes.extend_from_slice(&1.0_f64.to_bits().to_le_bytes());
    expected_bytes.extend_from_slice(&(-0.0_f64).to_bits().to_le_bytes());
    expected_bytes.push(0x00);
    assert_eq!(bytes, expected_bytes);

    let hex = digest::state_digest_hex(1, &q, &v);
    assert_eq!(
        hex, SYNTHETIC_NEG_ZERO_DIGEST,
        "canonical 50-byte synthetic layout digest must match independent b3sum calculation"
    );
}

#[test]
fn digest_distinguishes_signed_zero() {
    let pos = digest::state_digest_hex(1, &[1.0_f64], &[0.0_f64]);
    let neg = digest::state_digest_hex(1, &[1.0_f64], &[-0.0_f64]);
    assert_eq!(pos, SYNTHETIC_POS_ZERO_DIGEST);
    assert_eq!(neg, SYNTHETIC_NEG_ZERO_DIGEST);
    assert_ne!(pos, neg, "+0.0 and -0.0 must produce distinct digests");

    let fd_pos = digest::forward_dynamics_digest_hex(1, &[0.0_f64], &[0.0_f64], &[0.0_f64]);
    let fd_neg = digest::forward_dynamics_digest_hex(1, &[0.0_f64], &[0.0_f64], &[-0.0_f64]);
    assert_ne!(
        fd_pos, fd_neg,
        "+0.0 and -0.0 in acceleration must produce distinct forward-dynamics digests"
    );
}

#[test]
fn digest_injective_on_single_bit_flips() {
    // Deterministic SplitMix64 generator (explicit fixed seed; zero external deps).
    let mut state: u64 = 0xBD00_7000_0000_0001;
    let mut next_u64 = || -> u64 {
        state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    };

    const NUM_STATES: usize = 1_000;
    const NQ: usize = 8;
    const NV: usize = 7;

    for trial in 0..NUM_STATES {
        let mut q = [0.0_f64; NQ];
        let mut v = [0.0_f64; NV];
        for slot in &mut q {
            *slot = f64::from_bits(next_u64());
        }
        for slot in &mut v {
            *slot = f64::from_bits(next_u64());
        }

        let base = digest::state_digest_bytes(30, &q, &v);

        let q_idx = (next_u64() as usize) % NQ;
        let q_bit = (next_u64() as u32) % 64;
        let mut q_mut = q;
        q_mut[q_idx] = f64::from_bits(q[q_idx].to_bits() ^ (1_u64 << q_bit));
        let after_q = digest::state_digest_bytes(30, &q_mut, &v);
        assert_ne!(
            base, after_q,
            "trial {trial}: flipping bit {q_bit} of q[{q_idx}] must change digest"
        );

        let v_idx = (next_u64() as usize) % NV;
        let v_bit = (next_u64() as u32) % 64;
        let mut v_mut = v;
        v_mut[v_idx] = f64::from_bits(v[v_idx].to_bits() ^ (1_u64 << v_bit));
        let after_v = digest::state_digest_bytes(30, &q, &v_mut);
        assert_ne!(
            base, after_v,
            "trial {trial}: flipping bit {v_bit} of v[{v_idx}] must change digest"
        );
    }
}

#[test]
fn fixture_capture_record_consistent() {
    let root = workspace_root();
    let lock_pin = read_lock_fsim_pin(&root);
    let fix_path = fixture_path();
    let log_path = capture_log_path();

    let fix_text = fs::read_to_string(&fix_path).expect("failed reading oracle fixture");
    let log_bytes = fs::read(&log_path).expect("failed reading oracle capture log");
    let fixture = parse_toml_bytes(fix_text.as_bytes(), "g1_fd_digest.toml");

    // 1. Checked-in fixture and capture log must verify cleanly for the current pin.
    require_fixture_digest_for_pin(&fixture, &lock_pin, "fd_digest")
        .expect("checked-in fixture must carry valid fd_digest for current pin");
    require_fixture_digest_for_pin(&fixture, &lock_pin, "rollout_2s_digest")
        .expect("checked-in fixture must carry valid rollout_2s_digest for current pin");
    verify_capture_record(&fixture, &log_bytes)
        .expect("checked-in fixture and capture log must agree");

    // Also verify digest.rs source hash recorded in the fixture matches our in-tree digest.rs.
    let digest_src = fs::read(root.join("crates/frob-fsim/src/referee/digest.rs"))
        .expect("read crates/frob-fsim/src/referee/digest.rs");
    let expected_src_b3 = fixture
        .get("digest_source_blake3")
        .and_then(Value::as_str)
        .expect("fixture must record digest_source_blake3");
    assert_eq!(
        digest::blake3_hex(&digest_src),
        expected_src_b3,
        "crates/frob-fsim/src/referee/digest.rs must match the exact source hashed in the oracle probe"
    );

    let real_fd = fixture
        .get("fd_digest")
        .and_then(Value::as_str)
        .expect("fd_digest");
    let real_rollout = fixture
        .get("rollout_2s_digest")
        .and_then(Value::as_str)
        .expect("rollout_2s_digest");
    let real_toolchain = fixture
        .get("toolchain")
        .and_then(Value::as_str)
        .expect("toolchain");

    let fake_digest = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

    // Planted negative control 1: candidate fd_digest copied into fixture, log unchanged.
    let planted_fd_text = fix_text.replacen(real_fd, fake_digest, 1);
    let planted_fd = parse_toml_bytes(planted_fd_text.as_bytes(), "planted_fd");
    let err1 = verify_capture_record(&planted_fd, &log_bytes)
        .expect_err("planted fd_digest without matching capture log must be refused");
    assert!(
        err1.contains("fd_digest"),
        "expected fd_digest mismatch error, got: {err1}"
    );

    // Planted negative control 2: candidate rollout_2s_digest copied into fixture, log unchanged.
    let planted_roll_text = fix_text.replacen(real_rollout, fake_digest, 1);
    let planted_roll = parse_toml_bytes(planted_roll_text.as_bytes(), "planted_rollout");
    let err2 = verify_capture_record(&planted_roll, &log_bytes)
        .expect_err("planted rollout_2s_digest without matching capture log must be refused");
    assert!(
        err2.contains("rollout_2s_digest"),
        "expected rollout_2s_digest mismatch error, got: {err2}"
    );

    // Planted negative control 3: capture log edited by one byte.
    let mut edited_log = log_bytes.clone();
    assert!(!edited_log.is_empty(), "capture log must be non-empty");
    let last_idx = edited_log.len() - 1;
    edited_log[last_idx] ^= 0x01;
    let err3 = verify_capture_record(&fixture, &edited_log)
        .expect_err("one-byte edit to capture log must be refused");
    assert!(
        err3.contains("capture_log_blake3"),
        "expected capture_log_blake3 mismatch error, got: {err3}"
    );

    // Planted negative control 4: toolchain line changed in fixture.
    let planted_tc_text = fix_text.replacen(
        &format!("toolchain = \"{real_toolchain}\""),
        "toolchain = \"nightly-2099-01-01\"",
        1,
    );
    let planted_tc = parse_toml_bytes(planted_tc_text.as_bytes(), "planted_toolchain");
    let err4 = verify_capture_record(&planted_tc, &log_bytes)
        .expect_err("changed toolchain line must be refused");
    assert!(
        err4.contains("toolchain"),
        "expected toolchain mismatch error, got: {err4}"
    );

    // Verify pin-mismatch and missing-field helpers refuse with "capture missing for pin".
    let err_pin = require_fixture_digest_for_pin(&fixture, "deadbeef", "fd_digest")
        .expect_err("wrong pin must be refused");
    assert!(
        err_pin.contains("capture missing for pin"),
        "expected 'capture missing for pin', got: {err_pin}"
    );
    let no_rollout_text = fix_text
        .lines()
        .filter(|l| !l.starts_with("rollout_2s_digest"))
        .collect::<Vec<_>>()
        .join("\n");
    let no_rollout = parse_toml_bytes(no_rollout_text.as_bytes(), "no_rollout");
    let err_missing = require_fixture_digest_for_pin(&no_rollout, &lock_pin, "rollout_2s_digest")
        .expect_err("fixture without rollout_2s_digest must be refused");
    assert!(
        err_missing.contains("capture missing for pin"),
        "expected 'capture missing for pin', got: {err_missing}"
    );
}

// =========================================================================
// RefereeDynamics adapter tests (compiled strictly under `feature = "referee"`)
// =========================================================================

#[cfg(feature = "referee")]
fn run_g1_rollout_2s(dyn_api: &InProcRefereeDynamics) -> (RefereeState, String) {
    let model = dyn_api
        .catalog_model(CatalogRobot::G1)
        .expect("catalog_model(G1) must succeed");
    let facts = dyn_api.describe(&model);
    let mut state = dyn_api.reference_state(&model);
    let tau = vec![0.0_f64; facts.actuators as usize];
    for step_idx in 0..ROLLOUT_STEPS_2S {
        let next = dyn_api.step_free(&model, &state, &tau, Rational::ONE_OVER_480);
        assert!(
            next.is_ok(),
            "step_free failed at step {step_idx}: {:?}",
            next.as_ref().err()
        );
        state = next.expect("checked Ok above");
    }
    let hex = digest::state_digest_hex(facts.links, &state.q, &state.v);
    (state, hex)
}

#[cfg(feature = "referee")]
#[test]
fn g1_fd_digest_matches_oracle() {
    let root = workspace_root();
    let lock_pin = read_lock_fsim_pin(&root);
    let fixture = parse_toml_file(&fixture_path());
    let oracle_res = require_fixture_digest_for_pin(&fixture, &lock_pin, "fd_digest");
    assert!(
        oracle_res.is_ok(),
        "{}",
        oracle_res.as_ref().err().map_or("", String::as_str)
    );
    let oracle_digest = oracle_res.expect("checked Ok above");

    let dyn_api = new_referee_dynamics().expect("referee feature is enabled");
    let model = dyn_api
        .catalog_model(CatalogRobot::G1)
        .expect("catalog_model(G1)");
    let facts = dyn_api.describe(&model);
    let state = dyn_api.reference_state(&model);
    let tau = vec![0.0_f64; facts.actuators as usize];

    let qdd = dyn_api
        .forward_dynamics(&model, &state, &tau)
        .expect("forward_dynamics at reference state must succeed");
    assert_eq!(qdd.len(), facts.nv as usize);

    let candidate_digest =
        digest::forward_dynamics_digest_hex(facts.links, &state.q, &state.v, &qdd);

    if candidate_digest != oracle_digest || env::var_os("FROB_REFEREE_DIAG").is_some() {
        eprintln!("candidate_fd_digest={candidate_digest}");
        eprintln!("oracle_fd_digest={oracle_digest}");
        for (i, &x) in qdd.iter().enumerate() {
            eprintln!("candidate.qdd[{i}]={:016x} ({x:.17e})", x.to_bits());
        }
    }

    assert_eq!(
        candidate_digest, oracle_digest,
        "candidate G1 forward-dynamics digest must equal FrankenSim in-tree oracle digest at pin {lock_pin}"
    );
}

#[cfg(feature = "referee")]
#[test]
fn g1_fd_digest_changes_on_ulp_perturbation() {
    let root = workspace_root();
    let lock_pin = read_lock_fsim_pin(&root);
    let fixture = parse_toml_file(&fixture_path());
    let oracle_res = require_fixture_digest_for_pin(&fixture, &lock_pin, "fd_digest");
    assert!(
        oracle_res.is_ok(),
        "{}",
        oracle_res.as_ref().err().map_or("", String::as_str)
    );
    let oracle_digest = oracle_res.expect("checked Ok above");

    let dyn_api = InProcRefereeDynamics::new();
    let model = dyn_api
        .catalog_model(CatalogRobot::G1)
        .expect("catalog_model(G1)");
    let facts = dyn_api.describe(&model);
    let base_state = dyn_api.reference_state(&model);
    let tau = vec![0.0_f64; facts.actuators as usize];

    // +1 ULP on q[0] (base_x) as named in the Test Plan.
    let mut s_q0 = base_state.clone();
    s_q0.q[0] = f64::from_bits(s_q0.q[0].to_bits() + 1);
    let qdd_q0 = dyn_api
        .forward_dynamics(&model, &s_q0, &tau)
        .expect("forward_dynamics on q[0] + 1 ULP");
    let d_q0 = digest::forward_dynamics_digest_hex(facts.links, &s_q0.q, &s_q0.v, &qdd_q0);
    assert_ne!(
        d_q0, oracle_digest,
        "perturbing q[0] by +1 ULP must change the forward-dynamics digest"
    );

    // +1 ULP on q[7] (first actuated joint coordinate) per Technical Approach step 4.
    let mut s_j0 = base_state;
    s_j0.q[7] = f64::from_bits(s_j0.q[7].to_bits() + 1);
    let qdd_j0 = dyn_api
        .forward_dynamics(&model, &s_j0, &tau)
        .expect("forward_dynamics on first joint q[7] + 1 ULP");
    let d_j0 = digest::forward_dynamics_digest_hex(facts.links, &s_j0.q, &s_j0.v, &qdd_j0);
    assert_ne!(
        d_j0, oracle_digest,
        "perturbing first joint q[7] by +1 ULP must change the forward-dynamics digest"
    );
    assert_ne!(d_q0, d_j0);
}

#[cfg(feature = "referee")]
#[test]
fn g1_fd_digest_changes_on_torque_perturbation() {
    let root = workspace_root();
    let lock_pin = read_lock_fsim_pin(&root);
    let fixture = parse_toml_file(&fixture_path());
    let oracle_res = require_fixture_digest_for_pin(&fixture, &lock_pin, "fd_digest");
    assert!(
        oracle_res.is_ok(),
        "{}",
        oracle_res.as_ref().err().map_or("", String::as_str)
    );
    let oracle_digest = oracle_res.expect("checked Ok above");

    let dyn_api = InProcRefereeDynamics::new();
    let model = dyn_api
        .catalog_model(CatalogRobot::G1)
        .expect("catalog_model(G1)");
    let facts = dyn_api.describe(&model);
    let state = dyn_api.reference_state(&model);

    let mut tau_perturbed = vec![0.0_f64; facts.actuators as usize];
    tau_perturbed[0] = 1.0;
    let qdd_perturbed = dyn_api
        .forward_dynamics(&model, &state, &tau_perturbed)
        .expect("forward_dynamics with tau[0]=1.0 N*m");
    let d_perturbed =
        digest::forward_dynamics_digest_hex(facts.links, &state.q, &state.v, &qdd_perturbed);
    assert_ne!(
        d_perturbed, oracle_digest,
        "changing tau[0] from 0.0 to 1.0 N*m with q and v unchanged must change the digest"
    );

    let tau_zero = vec![0.0_f64; facts.actuators as usize];
    let qdd_zero = dyn_api
        .forward_dynamics(&model, &state, &tau_zero)
        .expect("forward_dynamics with tau[0]=0.0 N*m");
    let d_zero = digest::forward_dynamics_digest_hex(facts.links, &state.q, &state.v, &qdd_zero);
    assert_ne!(
        qdd_perturbed
            .iter()
            .map(|x| x.to_bits())
            .collect::<Vec<_>>(),
        qdd_zero.iter().map(|x| x.to_bits()).collect::<Vec<_>>(),
        "torque perturbation must alter acceleration coordinates themselves"
    );
    assert_eq!(
        d_zero, oracle_digest,
        "restoring tau[0]=0.0 must reproduce the oracle forward-dynamics digest"
    );
}

#[cfg(feature = "referee")]
#[test]
fn g1_rollout_2s_digest_matches_oracle() {
    let root = workspace_root();
    let lock_pin = read_lock_fsim_pin(&root);
    let fixture = parse_toml_file(&fixture_path());
    let oracle_res = require_fixture_digest_for_pin(&fixture, &lock_pin, "rollout_2s_digest");
    assert!(
        oracle_res.is_ok(),
        "{}",
        oracle_res.as_ref().err().map_or("", String::as_str)
    );
    let oracle_rollout = oracle_res.expect("checked Ok above");

    let dyn_api = InProcRefereeDynamics::new();
    let (end_state, candidate_rollout) = run_g1_rollout_2s(&dyn_api);

    if candidate_rollout != oracle_rollout || env::var_os("FROB_REFEREE_DIAG").is_some() {
        eprintln!("candidate_rollout_2s_digest={candidate_rollout}");
        eprintln!("oracle_rollout_2s_digest={oracle_rollout}");
        for (i, &x) in end_state.q.iter().enumerate() {
            eprintln!("candidate.q_end[{i}]={:016x} ({x:.17e})", x.to_bits());
        }
        for (i, &x) in end_state.v.iter().enumerate() {
            eprintln!("candidate.v_end[{i}]={:016x} ({x:.17e})", x.to_bits());
        }
    }

    assert_eq!(
        candidate_rollout, oracle_rollout,
        "candidate 960-step (2 s at 1/480 s) rollout digest must equal FrankenSim in-tree oracle digest at pin {lock_pin}"
    );
}

#[cfg(feature = "referee")]
#[test]
fn state_with_nan_refused() {
    let dyn_api = InProcRefereeDynamics::new();
    let model = dyn_api
        .catalog_model(CatalogRobot::G1)
        .expect("catalog_model(G1)");
    let facts = dyn_api.describe(&model);
    let base = dyn_api.reference_state(&model);
    let tau_zero = vec![0.0_f64; facts.actuators as usize];

    for bad in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        let mut s_q = base.clone();
        s_q.q[7] = bad;
        let err_fd = dyn_api
            .forward_dynamics(&model, &s_q, &tau_zero)
            .expect_err("non-finite q must be refused by forward_dynamics");
        assert_eq!(
            err_fd.kind,
            RefereeRefusalKind::NonFinite {
                field: "q",
                index: 7
            }
        );
        let err_step = dyn_api
            .step_free(&model, &s_q, &tau_zero, Rational::ONE_OVER_480)
            .expect_err("non-finite q must be refused by step_free");
        assert_eq!(
            err_step.kind,
            RefereeRefusalKind::NonFinite {
                field: "q",
                index: 7
            }
        );

        let mut s_v = base.clone();
        s_v.v[2] = bad;
        let err_v = dyn_api
            .forward_dynamics(&model, &s_v, &tau_zero)
            .expect_err("non-finite v must be refused");
        assert_eq!(
            err_v.kind,
            RefereeRefusalKind::NonFinite {
                field: "v",
                index: 2
            }
        );

        let mut bad_tau = tau_zero.clone();
        bad_tau[5] = bad;
        let err_tau = dyn_api
            .forward_dynamics(&model, &base, &bad_tau)
            .expect_err("non-finite tau must be refused");
        assert_eq!(
            err_tau.kind,
            RefereeRefusalKind::NonFinite {
                field: "tau",
                index: 5
            }
        );
    }

    let err_zero_h = dyn_api
        .step_free(&model, &base, &tau_zero, Rational::from_raw(0, 480))
        .expect_err("zero step duration must be refused");
    assert_eq!(err_zero_h.kind, RefereeRefusalKind::InvalidStep);
    let err_zero_den = dyn_api
        .step_free(&model, &base, &tau_zero, Rational::from_raw(1, 0))
        .expect_err("zero denominator step duration must be refused");
    assert_eq!(err_zero_den.kind, RefereeRefusalKind::InvalidStep);
}

#[cfg(feature = "referee")]
#[test]
fn wrong_length_refused() {
    let dyn_api = InProcRefereeDynamics::new();
    let model = dyn_api
        .catalog_model(CatalogRobot::G1)
        .expect("catalog_model(G1)");
    let facts = dyn_api.describe(&model);
    let base = dyn_api.reference_state(&model);
    let tau_zero = vec![0.0_f64; facts.actuators as usize];

    // Wrong q length.
    let mut bad_q = base.clone();
    bad_q.q.pop();
    let err_q = dyn_api
        .forward_dynamics(&model, &bad_q, &tau_zero)
        .expect_err("wrong q length must be refused");
    assert_eq!(
        err_q.kind,
        RefereeRefusalKind::LengthMismatch {
            field: "q",
            expected: facts.nq as usize,
            actual: facts.nq as usize - 1,
        }
    );
    assert!(err_q.reason.contains(&facts.nq.to_string()));
    assert!(err_q.reason.contains(&(facts.nq - 1).to_string()));

    // Wrong v length.
    let mut bad_v = base.clone();
    bad_v.v.push(0.0);
    let err_v = dyn_api
        .step_free(&model, &bad_v, &tau_zero, Rational::ONE_OVER_480)
        .expect_err("wrong v length must be refused");
    assert_eq!(
        err_v.kind,
        RefereeRefusalKind::LengthMismatch {
            field: "v",
            expected: facts.nv as usize,
            actual: facts.nv as usize + 1,
        }
    );
    assert!(err_v.reason.contains(&facts.nv.to_string()));
    assert!(err_v.reason.contains(&(facts.nv + 1).to_string()));

    // Wrong tau length (short and long).
    let short_tau = vec![0.0_f64; facts.actuators as usize - 3];
    let err_tau_fd = dyn_api
        .forward_dynamics(&model, &base, &short_tau)
        .expect_err("wrong tau length must be refused by forward_dynamics");
    assert_eq!(
        err_tau_fd.kind,
        RefereeRefusalKind::LengthMismatch {
            field: "tau",
            expected: facts.actuators as usize,
            actual: facts.actuators as usize - 3,
        }
    );
    assert!(err_tau_fd.reason.contains(&facts.actuators.to_string()));
    assert!(
        err_tau_fd
            .reason
            .contains(&(facts.actuators - 3).to_string())
    );

    let long_tau = vec![0.0_f64; facts.actuators as usize + 2];
    let err_tau_step = dyn_api
        .step_free(&model, &base, &long_tau, Rational::ONE_OVER_480)
        .expect_err("wrong tau length must be refused by step_free");
    assert_eq!(
        err_tau_step.kind,
        RefereeRefusalKind::LengthMismatch {
            field: "tau",
            expected: facts.actuators as usize,
            actual: facts.actuators as usize + 2,
        }
    );
    assert!(err_tau_step.reason.contains(&facts.actuators.to_string()));
    assert!(
        err_tau_step
            .reason
            .contains(&(facts.actuators + 2).to_string())
    );
}

#[cfg(feature = "referee")]
#[test]
fn forward_dynamics_is_repeatable() {
    let dyn_api = InProcRefereeDynamics::new();
    let model = dyn_api
        .catalog_model(CatalogRobot::G1)
        .expect("catalog_model(G1)");
    let facts = dyn_api.describe(&model);
    let state = dyn_api.reference_state(&model);
    let tau = vec![0.0_f64; facts.actuators as usize];

    let first = dyn_api
        .forward_dynamics(&model, &state, &tau)
        .expect("call 0");
    let first_bits: Vec<u64> = first.iter().map(|x| x.to_bits()).collect();
    for iter in 1..100 {
        let cur_res = dyn_api.forward_dynamics(&model, &state, &tau);
        assert!(
            cur_res.is_ok(),
            "call {iter} failed: {:?}",
            cur_res.as_ref().err()
        );
        let cur = cur_res.expect("checked Ok above");
        let cur_bits: Vec<u64> = cur.iter().map(|x| x.to_bits()).collect();
        assert_eq!(
            cur_bits, first_bits,
            "forward_dynamics call {iter} differed from call 0"
        );
    }
}

#[cfg(feature = "referee")]
#[test]
fn step_free_two_seconds_repeatable() {
    let dyn_api = InProcRefereeDynamics::new();

    if env::var_os("FROB_REFEREE_SUBPROCESS_ROLLOUT").is_some() {
        let (_, child_digest) = run_g1_rollout_2s(&dyn_api);
        println!("FROB_CHILD_ROLLOUT_DIGEST={child_digest}");
        return;
    }

    let (state_a, digest_a) = run_g1_rollout_2s(&dyn_api);
    let (state_b, digest_b) = run_g1_rollout_2s(&dyn_api);
    assert_eq!(digest_a, digest_b, "two in-process rollouts must match");
    assert_eq!(
        state_a.q.iter().map(|x| x.to_bits()).collect::<Vec<_>>(),
        state_b.q.iter().map(|x| x.to_bits()).collect::<Vec<_>>(),
    );
    assert_eq!(
        state_a.v.iter().map(|x| x.to_bits()).collect::<Vec<_>>(),
        state_b.v.iter().map(|x| x.to_bits()).collect::<Vec<_>>(),
    );

    // Run the exact same binary in a second OS process and confirm identical digest.
    let exe = env::current_exe().expect("locate current test executable");
    let out = Command::new("/usr/bin/env")
        .arg("FROB_REFEREE_SUBPROCESS_ROLLOUT=1")
        .arg(&exe)
        .args(["--exact", "step_free_two_seconds_repeatable", "--nocapture"])
        .output()
        .expect("spawn child process for cross-process repeatability check");
    assert!(
        out.status.success(),
        "child process failed with {}: {}",
        out.status,
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    let child_digest = stdout
        .lines()
        .find_map(|line| line.strip_prefix("FROB_CHILD_ROLLOUT_DIGEST="))
        .expect("child process stdout missing FROB_CHILD_ROLLOUT_DIGEST= line");
    assert_eq!(
        digest_a, child_digest,
        "960-step rollout digest must be identical across two processes"
    );
}

#[cfg(feature = "referee")]
#[test]
fn describe_matches_fixture_layout() {
    let fixture = parse_toml_file(&fixture_path());
    let layout_tbl = fixture
        .get("layout")
        .and_then(Value::as_table)
        .expect("fixture must include [layout]");

    let dyn_api = InProcRefereeDynamics::new();
    let model = dyn_api
        .catalog_model(CatalogRobot::G1)
        .expect("catalog_model(G1)");
    let facts = dyn_api.describe(&model);

    assert_eq!(facts.links, 30);
    assert_eq!(facts.nq, 36);
    assert_eq!(facts.nv, 35);
    assert_eq!(facts.actuators, 29);
    assert_eq!(format!("{:.8}", facts.mass_kg), "31.96314202");

    assert_eq!(
        i64::from(facts.links),
        fixture.get("links").and_then(Value::as_int).expect("links")
    );
    assert_eq!(
        i64::from(facts.nq),
        fixture.get("nq").and_then(Value::as_int).expect("nq")
    );
    assert_eq!(
        i64::from(facts.nv),
        fixture.get("nv").and_then(Value::as_int).expect("nv")
    );
    assert_eq!(
        i64::from(facts.actuators),
        fixture
            .get("actuators")
            .and_then(Value::as_int)
            .expect("actuators")
    );
    assert_eq!(
        format!("{:.8}", facts.mass_kg),
        fixture
            .get("mass_kg")
            .and_then(Value::as_str)
            .expect("mass_kg")
    );
    assert_eq!(
        facts.layout.orientation_parameterization,
        fixture
            .get("orientation_parameterization")
            .and_then(Value::as_str)
            .expect("orientation_parameterization")
    );

    let str_vec = |key: &str| -> Vec<String> {
        layout_tbl
            .get(key)
            .and_then(Value::as_array)
            .expect("fixture [layout] missing expected string array")
            .iter()
            .map(|sp| {
                sp.value
                    .as_str()
                    .expect("non-string entry in fixture [layout]")
                    .to_owned()
            })
            .collect()
    };

    assert_eq!(facts.layout.link_names, str_vec("link_names"));
    assert_eq!(facts.layout.actuators, str_vec("joint_names"));

    let pos_rendered: Vec<String> = facts
        .layout
        .position
        .iter()
        .map(|c| format!("{}:{}:{}:{}", c.index, c.name, c.kind.as_str(), c.unit))
        .collect();
    assert_eq!(pos_rendered, str_vec("position_coords"));

    let vel_rendered: Vec<String> = facts
        .layout
        .velocity
        .iter()
        .map(|c| format!("{}:{}:{}:{}", c.index, c.name, c.kind.as_str(), c.unit))
        .collect();
    assert_eq!(vel_rendered, str_vec("velocity_coords"));
}
