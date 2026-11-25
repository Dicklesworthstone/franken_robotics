#![deny(unsafe_code)]

//! Unit and integration tests for the negative-control runner and the five rules
//! of the negative-control registry (`N38b-04`).
//!
//! Uses real scratch Cargo workspaces, real `cargo` builds (via PATH/`rch` with
//! isolated `CARGO_TARGET_DIR`), real physics/math evaluators for all five kinds,
//! and real fixtures. Never deletes files or directories (COMMON.md 9b).

use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use frob_conformance::compendium::rat::r;
use frob_conformance::control_runner::{
    CONTROL_REVIEWS_PATH, ControlEval, ControlHistoryEntry, ControlLine, ControlReviews,
    ControlRunnerConfig, ControlStatus, DRIVERS, DesignVerdict, Disposition, MutationRoute,
    SuspectCause, admit_fixed_n_study, append_history_entry, build_suspect_report,
    check_mutation_guard, classify_cart_design, docking_three_gains_min_worst_error,
    docking_two_gains_admitted, enumerate_fixed_n_rule, evaluate_aa_campaign,
    evaluate_wheel_loop_host_row, inspect_cargo_args, paired_lower_bound_95, parse_history_ndjson,
    resolve_mutation_route, resolve_review_milestone, run_controls_for_gate,
    run_controls_for_gate_default, verify_release_build_fails,
    verify_transported_docking_certificate, wheel_loop_spectral_radius_active,
    wheel_loop_spectral_radius_euler_mutant, wheel_loop_spectral_radius_exact,
    wheel_loop_spectral_radius_r_iref,
};
use frob_conformance::controls::{CONTROLS_PATH, ControlReason, load_and_validate};
use frob_conformance::gates_v0::{GATES_PATH, GateRegistry};
use frob_conformance::reasons::workspace_root;
use frob_conformance::tiers::Milestone;
use frob_core::testkit::registry::{Control, Controls};
use frob_core::testkit::{GateOutcome, NotEligibleReason};

static SCRATCH_SEQ: AtomicU64 = AtomicU64::new(0);
static DRIVER_TEST_LOCK: Mutex<()> = Mutex::new(());

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
        .join("frob-WindyDog")
        .join(format!("control_runner-{label}-{nanos}-{seq}"));
    fs::create_dir_all(&dir).expect("create scratch dir");
    dir
}

fn isolated_target_dir(label: &str) -> PathBuf {
    let base = env::var_os("RCH_TARGET_BASE")
        .or_else(|| env::var_os("TMPDIR"))
        .map(PathBuf::from)
        .unwrap_or_else(env::temp_dir);
    let seq = SCRATCH_SEQ.fetch_add(1, Ordering::Relaxed);
    let dir = base.join(format!("frob-n38b-04-{label}-{seq}"));
    fs::create_dir_all(&dir).expect("create target dir");
    dir
}

/// Build a tiny self-contained std-only scratch Cargo workspace with `frob-conformance`
/// (having `gauntlet-mutations` and a `g1_known_answers` test target) and one owning
/// crate `<crate_name>` declaring `<feature_block>` and `<lib_rs>`.
fn create_scratch_mutation_workspace(
    label: &str,
    crate_name: &str,
    feature_block: &str,
    lib_rs: &str,
    tier_test_rs: &str,
    controls_toml: &str,
) -> PathBuf {
    let ws = scratch_dir(label);
    let crate_dir = ws.join("crates").join(crate_name);
    let conf_dir = ws.join("crates/frob-conformance");
    fs::create_dir_all(crate_dir.join("src")).expect("mkdir crate src");
    fs::create_dir_all(conf_dir.join("src")).expect("mkdir conf src");
    fs::create_dir_all(conf_dir.join("tests")).expect("mkdir conf tests");
    fs::create_dir_all(conf_dir.join("registry")).expect("mkdir conf registry");

    fs::write(
        ws.join("franken-stack.lock"),
        "schema = \"frob.stack.lock/1\"\n",
    )
    .expect("write lock");
    fs::write(
        ws.join("Cargo.toml"),
        format!(
            "[workspace]\nresolver = \"2\"\nmembers = [\"crates/{crate_name}\", \"crates/frob-conformance\"]\n"
        ),
    )
    .expect("write workspace Cargo.toml");

    fs::write(
        crate_dir.join("Cargo.toml"),
        format!(
            "[package]\nname = \"{crate_name}\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[features]\n{feature_block}\n"
        ),
    )
    .expect("write crate Cargo.toml");
    fs::write(crate_dir.join("src/lib.rs"), lib_rs).expect("write crate lib.rs");

    fs::write(
        conf_dir.join("Cargo.toml"),
        format!(
            r#"[package]
name = "frob-conformance"
version = "0.1.0"
edition = "2024"

[features]
default = []
gauntlet-mutations = ["{crate_name}/conformance"]

[dependencies]
{crate_name} = {{ path = "../{crate_name}" }}
"#
        ),
    )
    .expect("write conf Cargo.toml");
    fs::write(conf_dir.join("src/lib.rs"), "// scratch conformance\n").expect("write conf lib.rs");
    fs::write(conf_dir.join("tests/g1_known_answers.rs"), tier_test_rs)
        .expect("write g1_known_answers.rs");
    fs::write(conf_dir.join("registry/controls.toml"), controls_toml).expect("write controls.toml");

    ws
}

fn parse_controls(raw: &str) -> Controls {
    Controls::parse_str(raw, CONTROLS_PATH).expect("parse scratch controls.toml")
}

fn parse_gates(raw: &str) -> GateRegistry {
    GateRegistry::parse_str(raw, GATES_PATH).expect("parse scratch gates.toml")
}

// ============================================================================
// Host stand-in row for NC-0412 (passes normally; fails with 1.497 under cx-262)
// ============================================================================

#[test]
fn wheel_loop_host_standin_row() {
    let rho = wheel_loop_spectral_radius_active();
    let res = evaluate_wheel_loop_host_row(rho);
    assert!(res.is_ok(), "{}", res.unwrap_err());
}

// ============================================================================
// 1. Five control kinds: real physics/math evaluators
// ============================================================================

#[test]
fn mutation_caught_wheel_loop() {
    // Exact one-tick spectral radii from the Plan §24.10 physical wheel-loop ODE.
    let rho_exact = wheel_loop_spectral_radius_exact();
    let rho_r_iref = wheel_loop_spectral_radius_r_iref();
    let rho_euler = wheel_loop_spectral_radius_euler_mutant();

    assert!(
        (rho_exact - 0.9207).abs() < 5e-4,
        "exact R*i spectral radius must match 0.9207, got {rho_exact:.6}"
    );
    assert!(
        (rho_r_iref - 1.4820).abs() < 5e-4,
        "exact R*i_ref spectral radius must match 1.4820, got {rho_r_iref:.6}"
    );
    assert!(
        (rho_euler - 1.497).abs() < 5e-4,
        "explicit Euler current-before-wheel mutant at 0.5 ms must match 1.497, got {rho_euler:.6}"
    );
    assert!(evaluate_wheel_loop_host_row(rho_exact).is_ok());
    let mutant_err = evaluate_wheel_loop_host_row(rho_euler)
        .expect_err("Euler-order mutant (1.497) must fail wheel-loop row");
    assert!(mutant_err.contains("1.497"), "got: {mutant_err}");

    // Run NC-0412 from the real workspace registry at M1 via M0 host stand-in.
    let root = workspace_root();
    let (controls, _) = Controls::load(&root).expect("load real controls.toml");
    let mut cfg = ControlRunnerConfig::new(root, Milestone::M1);
    cfg.allow_m0_host_standin = true;
    let verdict = run_controls_for_gate_default("GATE-ARENA-04", true, &controls, &cfg);
    assert_eq!(verdict.lines.len(), 1);
    assert_eq!(verdict.lines[0].id, "NC-0412");
    assert_eq!(
        verdict.lines[0].status,
        ControlStatus::Caught,
        "NC-0412 must be caught via host stand-in at M0: {:?}",
        verdict.lines[0]
    );
    assert!(verdict.lines[0].observed.contains("1.497"));
    assert_eq!(*verdict.gate_line.outcome(), GateOutcome::Pass);
}

#[test]
fn planted_input_tie_at_64() {
    // Verify exact §24.10 pre-analysis enumeration at n = 64 and n = 86.
    let p64 = enumerate_fixed_n_rule(64, 0.05);
    assert_eq!(p64.total_pairs, 2_145);
    assert_eq!(p64.prefer_b, 521);
    assert_eq!(p64.b_inferior, 521);
    assert_eq!(p64.practical_tie, 0);
    assert_eq!(p64.inconclusive, 1_103);
    assert!(
        (p64.perfect_concordance_bound - 0.0662).abs() < 1e-4,
        "perfect concordance at n=64 must give 0.0662, got {:.6}",
        p64.perfect_concordance_bound
    );

    let p85 = enumerate_fixed_n_rule(85, 0.05);
    assert_eq!(
        p85.practical_tie, 0,
        "tie must still be unreachable at n = 85"
    );

    let p86 = enumerate_fixed_n_rule(86, 0.05);
    assert_eq!(p86.total_pairs, 3_828);
    assert_eq!(p86.prefer_b, 1_031);
    assert_eq!(p86.b_inferior, 1_031);
    assert_eq!(
        p86.practical_tie, 1,
        "tie first becomes reachable at n = 86"
    );
    assert_eq!(p86.inconclusive, 1_765);

    // 1. With pre-analysis check enabled: fixed-n tie at n = 64 refused with FROB-REF-532.
    let refusal = admit_fixed_n_study(64, 0.05, &["practical-tie"], true)
        .expect_err("tie at n=64 must be refused with FROB-REF-532");
    assert!(refusal.contains("FROB-REF-532"), "got: {refusal}");

    // Run through the control runner on the real synthetic fixture.
    let root = workspace_root();
    let ctrls = parse_controls(
        r#"schema = "frob.gauntlet.controls/1"

[[control]]
id = "NC-0398"
seed = "KA-002"
gate = ["GATE-ARENA-05"]
kind = "planted-input"
target = "N24b-01:eval-preanalyze-unreachable-tie"
defect = "request for a tie verdict at n = 64 with unreachable-verdict guard disabled"
expect = "eval.preanalyze refuses unreachable tie verdict at n = 64 (FROB-REF-532)"
fixture = "fixtures/gauntlet/ex_planted_input_tie64.toml"
due = "M0"
removed_check = "unreachable tie verdict refusal at n = 64"
"#,
    );
    let cfg = ControlRunnerConfig::new(root, Milestone::M0);
    let verdict = run_controls_for_gate_default("GATE-ARENA-05", true, &ctrls, &cfg);
    assert_eq!(verdict.lines.len(), 1);
    assert_eq!(verdict.lines[0].status, ControlStatus::Caught);
    assert!(verdict.lines[0].observed.contains("FROB-REF-532"));
    assert_eq!(*verdict.gate_line.outcome(), GateOutcome::Pass);

    // 2. Mutant with pre-analysis check disabled: request is admitted, then caught by the
    //    row that enumerates reachable verdicts (practical_tie == 0 at n = 64).
    let mutant_admitted = admit_fixed_n_study(64, 0.05, &["practical-tie"], false)
        .expect("mutant with pre-analysis disabled admits the study");
    assert_eq!(
        mutant_admitted.practical_tie, 0,
        "reachable-verdict enumeration row catches the mutant: practical_tie count is 0 at n = 64"
    );
}

#[test]
fn planted_design_defective_caught() {
    let root = workspace_root();
    let sound = frob_conformance::control_runner::CartDesignSpec {
        total_mass_kg: 8.0,
        wheel_radius_m: 0.060,
        lip_height_m: 0.012,
        wheelbase_m: 0.32,
        motor_kt_nm_per_a: 0.2,
        current_limit_a: 6.0,
    };
    assert!(matches!(
        classify_cart_design(&sound),
        DesignVerdict::Sound { .. }
    ));

    let ctrls = parse_controls(
        r#"schema = "frob.gauntlet.controls/1"

[[control]]
id = "NC-0901"
seed = "GATE-ATEL-01"
gate = ["GATE-ATEL-01"]
kind = "planted-design"
target = "frob-conformance::control_runner::examples::classify_cart_design"
defect = "undersized front-drive current limit (1.0 A => 0.400 N·m < 1.270 N·m required)"
expect = "exact classifier: defective; naive mass screen wrongly accepted"
fixture = "fixtures/gauntlet/ex_planted_design_undersized.toml"
due = "M0"
"#,
    );
    let cfg = ControlRunnerConfig::new(root, Milestone::M0);
    let verdict = run_controls_for_gate_default("GATE-ATEL-01", true, &ctrls, &cfg);
    assert_eq!(verdict.lines.len(), 1);
    assert_eq!(verdict.lines[0].status, ControlStatus::Caught);
    assert_eq!(*verdict.gate_line.outcome(), GateOutcome::Pass);
}

#[test]
fn planted_world_third_gain() {
    let u = r(3, 4);
    let eps = r(1, 64);
    let g1 = r(1, 1);
    let g2 = r(2, 1);
    let g3 = r(33, 32);

    assert!(
        docking_two_gains_admitted(&u, &eps),
        "gains {{1, 2}} at U=3/4, eps=1/64 must admit exact docking"
    );
    let worst = docking_three_gains_min_worst_error(&u, &eps, &g3)
        .expect("compute exact minimax error with third gain 33/32");
    assert_eq!(
        worst,
        r(1, 65),
        "adding gain 33/32 at eps=1/64 must give exact minimax error 1/65 m"
    );

    let rej =
        verify_transported_docking_certificate(&[g1.clone(), g2.clone()], &[g1, g3, g2], &u, &eps)
            .expect_err("transported two-gain certificate must be rejected on three-gain world");
    assert!(rej.contains("reject") && rej.contains("1/65"), "got: {rej}");

    let root = workspace_root();
    let ctrls = parse_controls(
        r#"schema = "frob.gauntlet.controls/1"

[[control]]
id = "NC-0902"
seed = "CX-282"
gate = ["GATE-AUGUR-03"]
kind = "planted-world"
target = "frob-conformance::control_runner::examples::verify_transported_docking_certificate"
defect = "enlarge docking hypothesis support {1, 2} with gain 33/32 at U=3/4, eps=1/64"
expect = "reject: no exact docking; best worst-case error 1/65 m"
fixture = "fixtures/gauntlet/ex_planted_world_third_gain.toml"
due = "M0"
"#,
    );
    let cfg = ControlRunnerConfig::new(root, Milestone::M0);
    let verdict = run_controls_for_gate_default("GATE-AUGUR-03", true, &ctrls, &cfg);
    assert_eq!(verdict.lines.len(), 1);
    assert_eq!(verdict.lines[0].status, ControlStatus::Caught);
    assert_eq!(*verdict.gate_line.outcome(), GateOutcome::Pass);
}

#[test]
fn a_a_does_not_clear() {
    let arm_a = [10.0, 12.0, 11.5, 9.8, 10.4, 13.1, 11.0, 10.2, 12.5, 10.7];
    let arm_b = arm_a;
    let lb = paired_lower_bound_95(&arm_a, &arm_b).expect("compute paired lower bound");
    assert_eq!(
        lb, 1.0,
        "ten pairs of equal arms must give exact paired lower bound 1.0"
    );
    assert!(lb < 1.02, "A/A paired lower bound must not clear 1.02");

    // Constant ratio r = 1.25 returns 1.25 exact (PO-GAUNT-01).
    let scaled_b: Vec<f64> = arm_a.iter().map(|x| x * 1.25).collect();
    let lb_scaled = paired_lower_bound_95(&arm_a, &scaled_b).expect("constant ratio bound");
    assert!((lb_scaled - 1.25).abs() < 1e-12);

    assert!(evaluate_aa_campaign(&arm_a, &arm_b, 1.02).is_ok());

    let root = workspace_root();
    let ctrls = parse_controls(
        r#"schema = "frob.gauntlet.controls/1"

[[control]]
id = "NC-0903"
seed = "PO-GAUNT-01"
gate = ["GATE-GAUNT-09"]
kind = "a-a"
target = "frob-conformance::control_runner::examples::evaluate_aa_campaign"
defect = "ten pairs of equal arms submitted as an A/A campaign against threshold 1.02"
expect = "paired lower bound 1.0000 does not clear 1.02"
fixture = "fixtures/gauntlet/ex_aa_ten_equal_arms.toml"
due = "M0"
"#,
    );
    let cfg = ControlRunnerConfig::new(root, Milestone::M0);
    let verdict = run_controls_for_gate_default("GATE-GAUNT-09", true, &ctrls, &cfg);
    assert_eq!(verdict.lines.len(), 1);
    assert_eq!(verdict.lines[0].status, ControlStatus::Caught);
    assert_eq!(*verdict.gate_line.outcome(), GateOutcome::Pass);
}

// ============================================================================
// 2. Missed, caught-unexpected, and stale outcomes (real scratch builds)
// ============================================================================

#[test]
fn missed_makes_gate_fail() {
    let controls_toml = r#"schema = "frob.gauntlet.controls/1"

[[control]]
id = "NC-0501"
seed = "CX-262"
gate = ["GATE-DEMO-01"]
kind = "mutation"
target = "frob-noop-mut::compute"
defect = "no-op mutation feature leaves computation unchanged"
expect = "row reports 1.497 where 0.9207 is true"
feature = "cx-noop"
due = "M0"
"#;
    let ws = create_scratch_mutation_workspace(
        "missed_noop",
        "frob-noop-mut",
        "default = []\nconformance = []\ncx-noop = [\"conformance\"]\n",
        r#"#[cfg(all(feature = "conformance", not(debug_assertions)))]
compile_error!("conformance forbidden in release");

pub fn value() -> f64 {
    // Intentional no-op: cx-noop feature does not alter the value.
    0.9207
}
"#,
        r#"#[test]
fn demo_gate_row() {
    assert!((frob_noop_mut::value() - 0.9207).abs() < 1e-4);
}
"#,
        controls_toml,
    );
    let ctrls = parse_controls(controls_toml);
    let mut cfg = ControlRunnerConfig::new(ws, Milestone::M0);
    cfg.cargo_target_dir = Some(isolated_target_dir("missed"));
    cfg.allow_m0_host_standin = false;
    cfg.tier_target = "g1_known_answers".to_owned();

    // Even when every passing entry passed (entries_passed = true), one missed control -> FAIL.
    let verdict = run_controls_for_gate_default("GATE-DEMO-01", true, &ctrls, &cfg);
    assert_eq!(verdict.lines.len(), 1);
    assert_eq!(verdict.lines[0].status, ControlStatus::Missed);
    assert_eq!(*verdict.gate_line.outcome(), GateOutcome::Fail);
}

#[test]
fn caught_for_wrong_reason() {
    let controls_toml = r#"schema = "frob.gauntlet.controls/1"

[[control]]
id = "NC-0502"
seed = "CX-262"
gate = ["GATE-DEMO-02"]
kind = "mutation"
target = "frob-wrong-mut::compute"
defect = "mutant panics elsewhere instead of reporting 1.497"
expect = "wheel-loop row reports spectral radius 1.497 where 0.9207 is true"
feature = "cx-wrong-panic"
due = "M0"
"#;
    let ws = create_scratch_mutation_workspace(
        "wrong_reason",
        "frob-wrong-mut",
        "default = []\nconformance = []\ncx-wrong-panic = [\"conformance\"]\n",
        r#"#[cfg(all(feature = "conformance", not(debug_assertions)))]
compile_error!("conformance forbidden in release");

pub fn value() -> f64 {
    #[cfg(feature = "cx-wrong-panic")]
    panic!("unrelated index out of bounds in setup helper");
    #[cfg(not(feature = "cx-wrong-panic"))]
    {
        0.9207
    }
}
"#,
        r#"#[test]
fn demo_gate_row() {
    assert!((frob_wrong_mut::value() - 0.9207).abs() < 1e-4);
}
"#,
        controls_toml,
    );
    let ctrls = parse_controls(controls_toml);
    let mut cfg = ControlRunnerConfig::new(ws, Milestone::M0);
    cfg.cargo_target_dir = Some(isolated_target_dir("wrong-reason"));
    cfg.allow_m0_host_standin = false;

    let verdict = run_controls_for_gate_default("GATE-DEMO-02", true, &ctrls, &cfg);
    assert_eq!(verdict.lines.len(), 1);
    assert_eq!(
        verdict.lines[0].status,
        ControlStatus::CaughtUnexpected,
        "unrelated panic must be caught-unexpected, not caught: {:?}",
        verdict.lines[0]
    );
    assert!(!verdict.lines[0].status.counts_as_caught());
    assert!(verdict.lines[0].status.counts_as_missed());
    assert_eq!(*verdict.gate_line.outcome(), GateOutcome::Fail);
}

#[test]
fn stale_control() {
    let controls_toml = r#"schema = "frob.gauntlet.controls/1"

[[control]]
id = "NC-0503"
seed = "CX-262"
gate = ["GATE-DEMO-03"]
kind = "mutation"
target = "frob-stale-mut::compute"
defect = "feature removed from manifest"
expect = "row reports 1.497"
feature = "cx-removed"
due = "M0"
"#;
    let ws = create_scratch_mutation_workspace(
        "stale_feat",
        "frob-stale-mut",
        "default = []\nconformance = []\n",
        r#"#[cfg(all(feature = "conformance", not(debug_assertions)))]
compile_error!("conformance forbidden in release");
pub fn value() -> f64 { 0.9207 }
"#,
        "#[test]\nfn ok() { assert!((frob_stale_mut::value() - 0.9207).abs() < 1e-4); }\n",
        controls_toml,
    );
    let ctrls = parse_controls(controls_toml);
    let mut cfg = ControlRunnerConfig::new(ws, Milestone::M0);
    cfg.allow_m0_host_standin = false;

    let verdict = run_controls_for_gate_default("GATE-DEMO-03", true, &ctrls, &cfg);
    assert_eq!(verdict.lines.len(), 1);
    assert_eq!(verdict.lines[0].status, ControlStatus::Stale);
    assert!(!verdict.lines[0].status.counts_as_caught());
    assert_eq!(
        *verdict.gate_line.outcome(),
        GateOutcome::NotEligible(NotEligibleReason::StaleControl("NC-0503".to_owned()))
    );
}

// ============================================================================
// 3. Exact feature spelling, owning-crate law suite, annex DRIVERS, not-yet-due
// ============================================================================

#[test]
fn feature_as_record_names_it() {
    let controls_toml = r#"schema = "frob.gauntlet.controls/1"

[[control]]
id = "NC-0412"
seed = "CX-262"
gate = ["GATE-ARENA-04"]
kind = "mutation"
target = "frob-actuate-scratch::integrate"
defect = "explicit Euler current before wheel speed"
expect = "wheel-loop row reports spectral radius 1.497 where 0.9207 is true"
feature = "cx-262"
due = "M0"
"#;
    let lib_with_cx262 = r#"#[cfg(all(feature = "conformance", not(debug_assertions)))]
compile_error!("conformance forbidden in release");

pub fn spectral_radius() -> f64 {
    #[cfg(feature = "cx-262")]
    { 1.497 }
    #[cfg(not(feature = "cx-262"))]
    { 0.9207 }
}
"#;
    let tier_test = r#"#[test]
fn wheel_loop_row() {
    let rho = frob_actuate_scratch::spectral_radius();
    assert!(
        (rho - 0.9207).abs() < 5e-4,
        "wheel-loop row reports spectral radius {rho:.3} where 0.9207 is true"
    );
}
"#;

    // 1. Crate declares exact feature `cx-262`: built with `--features gauntlet-mutations,frob-actuate-scratch/cx-262` and caught.
    let ws_ok = create_scratch_mutation_workspace(
        "feat_exact_ok",
        "frob-actuate-scratch",
        "default = []\nconformance = []\ncx-262 = [\"conformance\"]\n",
        lib_with_cx262,
        tier_test,
        controls_toml,
    );
    let ctrls = parse_controls(controls_toml);
    let route = resolve_mutation_route(
        &ws_ok,
        &ctrls.all()[0],
        "GATE-ARENA-04",
        "g1_known_answers",
        false,
    )
    .expect("resolve route");
    let args = inspect_cargo_args(&ws_ok, &route, true);
    assert!(
        args.iter()
            .any(|a| a == "gauntlet-mutations,frob-actuate-scratch/cx-262"),
        "must build exact feature from record, got args: {args:?}"
    );

    let mut cfg = ControlRunnerConfig::new(ws_ok, Milestone::M0);
    cfg.cargo_target_dir = Some(isolated_target_dir("feat-exact"));
    cfg.allow_m0_host_standin = false;
    // Empty gate filter on scratch tier target runs the single `wheel_loop_row` test.
    let verdict = run_controls_for_gate("GATE-ARENA-04", true, &ctrls, &cfg, |ctrl, c| {
        frob_conformance::control_runner::run_mutation_control(
            &c.workspace_root,
            ctrl,
            "",
            "g1_known_answers",
            c.cargo_target_dir.as_deref(),
            true,
            false,
        )
    });
    assert_eq!(verdict.lines[0].status, ControlStatus::Caught);
    assert_eq!(*verdict.gate_line.outcome(), GateOutcome::Pass);

    // 2. Same plant declared ONLY as `nc-0412` while the record names `cx-262` -> Stale!
    let ws_mismatch = create_scratch_mutation_workspace(
        "feat_exact_mismatch",
        "frob-actuate-scratch",
        "default = []\nconformance = []\nnc-0412 = [\"conformance\"]\n",
        lib_with_cx262,
        tier_test,
        controls_toml,
    );
    let mut cfg_mm = ControlRunnerConfig::new(ws_mismatch, Milestone::M0);
    cfg_mm.allow_m0_host_standin = false;
    let verdict_mm = run_controls_for_gate_default("GATE-ARENA-04", true, &ctrls, &cfg_mm);
    assert_eq!(
        verdict_mm.lines[0].status,
        ControlStatus::Stale,
        "declaring only nc-0412 when record names cx-262 must be Stale"
    );
    assert_eq!(
        *verdict_mm.gate_line.outcome(),
        GateOutcome::NotEligible(NotEligibleReason::StaleControl("NC-0412".to_owned()))
    );
}

#[test]
fn law_suite_built_in_owning_crate() {
    let ws = scratch_dir("law_suite_owning");
    let crate_dir = ws.join("crates/frob-laws-scratch");
    fs::create_dir_all(crate_dir.join("src")).expect("mkdir src");
    fs::create_dir_all(crate_dir.join("tests")).expect("mkdir tests");
    fs::write(
        ws.join("franken-stack.lock"),
        "schema = \"frob.stack.lock/1\"\n",
    )
    .expect("write lock");
    fs::write(
        ws.join("Cargo.toml"),
        "[workspace]\nresolver = \"2\"\nmembers = [\"crates/frob-laws-scratch\"]\n",
    )
    .expect("write root Cargo.toml");
    // Owning crate has NO `gauntlet-mutations` feature at all — only `conformance` + `cx-999`!
    fs::write(
        crate_dir.join("Cargo.toml"),
        r#"[package]
name = "frob-laws-scratch"
version = "0.1.0"
edition = "2024"

[features]
default = []
conformance = []
cx-999 = ["conformance"]
"#,
    )
    .expect("write crate Cargo.toml");
    fs::write(
        crate_dir.join("src/lib.rs"),
        r#"#[cfg(all(feature = "conformance", not(debug_assertions)))]
compile_error!("conformance forbidden in release");

pub fn bracket_sign() -> i32 {
    #[cfg(feature = "cx-999")]
    { -1 }
    #[cfg(not(feature = "cx-999"))]
    { 1 }
}
"#,
    )
    .expect("write src/lib.rs");
    fs::write(
        crate_dir.join("tests/laws_c99.rs"),
        r#"#[test]
fn jacobi_sign_law() {
    let s = frob_laws_scratch::bracket_sign();
    assert_eq!(s, 1, "law_c99 jacobi sign mismatch: got {s}");
}
"#,
    )
    .expect("write tests/laws_c99.rs");

    let controls_toml = r#"schema = "frob.gauntlet.controls/1"

[[control]]
id = "NC-0999"
seed = "CX-999"
gate = ["GATE-LAW-99"]
kind = "mutation"
target = "crates/frob-laws-scratch/tests/laws_c99.rs::jacobi_sign_law"
defect = "flips Lie bracket sign in owning crate law suite"
expect = "law_c99 jacobi sign mismatch"
feature = "cx-999"
due = "M0"
"#;
    let ctrls = parse_controls(controls_toml);
    let route = resolve_mutation_route(&ws, &ctrls.all()[0], "GATE-LAW-99", "g0_laws", false)
        .expect("resolve law suite route");
    assert!(
        matches!(
            &route,
            MutationRoute::LawSuite {
                crate_name,
                feature,
                test_target,
                test_name
            } if crate_name == "frob-laws-scratch"
                && feature == "cx-999"
                && test_target == "laws_c99"
                && test_name.as_deref() == Some("jacobi_sign_law")
        ),
        "unexpected route: {route:?}"
    );
    let args = inspect_cargo_args(&ws, &route, true);
    assert!(
        args.windows(2)
            .any(|w| w[0] == "-p" && w[1] == "frob-laws-scratch")
            && args
                .windows(2)
                .any(|w| w[0] == "--features" && w[1] == "cx-999")
            && args
                .windows(2)
                .any(|w| w[0] == "--test" && w[1] == "laws_c99")
            && !args.iter().any(|a| a.contains("gauntlet-mutations")),
        "law suite must build in owning crate without gauntlet-mutations, got: {args:?}"
    );

    let mut cfg = ControlRunnerConfig::new(ws, Milestone::M0);
    cfg.cargo_target_dir = Some(isolated_target_dir("law-suite"));
    cfg.allow_m0_host_standin = false;
    let verdict = run_controls_for_gate_default("GATE-LAW-99", true, &ctrls, &cfg);
    assert_eq!(verdict.lines.len(), 1);
    assert_eq!(
        verdict.lines[0].status,
        ControlStatus::Caught,
        "law suite mutant must compile without gauntlet-mutations and fail -> Caught: {:?}",
        verdict.lines[0]
    );
    assert_eq!(*verdict.gate_line.outcome(), GateOutcome::Pass);
}

fn synthetic_annex_driver(ctrl: &Control) -> ControlLine {
    ControlLine::new(
        ctrl,
        ctrl.gate.first().cloned().unwrap_or_default(),
        ControlStatus::Caught,
        format!("annex-driver caught {}: {}", ctrl.id, ctrl.expect),
        "synthetic annex driver caught control",
    )
}

#[test]
fn annex_target_uses_driver() {
    let _guard = DRIVER_TEST_LOCK
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let ws_name = "spark-test-annex";
    DRIVERS.unregister(ws_name);

    let ctrls = parse_controls(
        r#"schema = "frob.gauntlet.controls/1"

[[control]]
id = "NC-0888"
seed = "CX-216"
gate = ["GATE-SPARK-01"]
kind = "mutation"
target = "annex:spark-test-annex::kernel::step"
defect = "annex mutant feature inside Spark workspace"
expect = "annex test fails with mutant-216"
feature = "mutant-216"
due = "M0"
"#,
    );
    let cfg = ControlRunnerConfig::new(workspace_root(), Milestone::M0);

    // 1. With no driver registered -> Stale, gate not-eligible.
    let v_none = run_controls_for_gate_default("GATE-SPARK-01", true, &ctrls, &cfg);
    assert_eq!(v_none.lines.len(), 1);
    assert_eq!(v_none.lines[0].status, ControlStatus::Stale);
    assert_eq!(
        *v_none.gate_line.outcome(),
        GateOutcome::NotEligible(NotEligibleReason::StaleControl("NC-0888".to_owned()))
    );

    // 2. Register synthetic driver in control_runner::DRIVERS -> Caught and enters receipt.
    DRIVERS.register(ws_name, synthetic_annex_driver);
    let v_drv = run_controls_for_gate_default("GATE-SPARK-01", true, &ctrls, &cfg);
    DRIVERS.unregister(ws_name);

    assert_eq!(v_drv.lines.len(), 1);
    assert_eq!(v_drv.lines[0].status, ControlStatus::Caught);
    assert!(
        v_drv.lines[0]
            .observed
            .contains("annex-driver caught NC-0888")
    );
    let rc = v_drv.receipt_controls();
    assert_eq!(rc.len(), 1);
    assert_eq!(rc[0].id, "NC-0888");
    assert_eq!(rc[0].outcome.as_str(), "caught");
    assert_eq!(*v_drv.gate_line.outcome(), GateOutcome::Pass);
}

#[test]
fn not_yet_due_neither_blocks_nor_supports() {
    let ws = scratch_dir("not_yet_due");
    let fx_dir = ws.join("fixtures/gauntlet");
    fs::create_dir_all(&fx_dir).expect("mkdir fixtures");
    fs::write(fx_dir.join("m0_ok.toml"), "# synthetic\nval = 1\n").expect("write m0 fixture");

    // Gate GATE-WS-07 has M0 control NC-0701 (exists, caught) and M2 control NC-0702 (missing plant).
    // Gate GATE-M2-ONLY has ONLY M2 control NC-0703 (missing plant).
    let ctrls = parse_controls(
        r#"schema = "frob.gauntlet.controls/1"

[[control]]
id = "NC-0701"
seed = "GATE-GAUNT-01"
gate = ["GATE-WS-07"]
kind = "planted-input"
target = "frob-conformance::controls::check_registry"
defect = "m0 planted defect"
expect = "fails with no-control"
fixture = "fixtures/gauntlet/m0_ok.toml"
due = "M0"

[[control]]
id = "NC-0702"
seed = "GATE-GAUNT-01"
gate = ["GATE-WS-07"]
kind = "mutation"
target = "frob-future::stage"
defect = "m2 defect whose plant does not exist yet"
expect = "fails with future error"
feature = "cx-future-702"
due = "M2"

[[control]]
id = "NC-0703"
seed = "GATE-GAUNT-01"
gate = ["GATE-M2-ONLY"]
kind = "mutation"
target = "frob-future::stage"
defect = "only control is due M2"
expect = "fails with future error"
feature = "cx-future-703"
due = "M2"
"#,
    );

    // 1. At M0: NC-0701 caught, NC-0702 is not-yet-due -> GATE-WS-07 is Pass!
    let cfg_m0 = ControlRunnerConfig::new(ws.clone(), Milestone::M0);
    let v_m0 = run_controls_for_gate("GATE-WS-07", true, &ctrls, &cfg_m0, |ctrl, _| {
        assert_eq!(ctrl.id, "NC-0701");
        ControlEval::Failed {
            observed: "fails with no-control".to_owned(),
        }
    });
    assert_eq!(v_m0.lines.len(), 2);
    assert_eq!(v_m0.lines[0].status, ControlStatus::Caught);
    assert_eq!(v_m0.lines[1].status, ControlStatus::NotYetDue);
    assert_eq!(v_m0.lines[1].due.as_deref(), Some("M2"));
    assert_eq!(*v_m0.gate_line.outcome(), GateOutcome::Pass);

    // 2. At M2: NC-0702 is now due and its plant is absent -> Stale, GATE-WS-07 is NotEligible!
    let cfg_m2 = ControlRunnerConfig::new(ws.clone(), Milestone::M2);
    let v_m2 = run_controls_for_gate("GATE-WS-07", true, &ctrls, &cfg_m2, |_, _| {
        ControlEval::Failed {
            observed: "fails with no-control".to_owned(),
        }
    });
    assert_eq!(v_m2.lines[0].status, ControlStatus::Caught);
    assert_eq!(v_m2.lines[1].status, ControlStatus::Stale);
    assert_eq!(
        *v_m2.gate_line.outcome(),
        GateOutcome::NotEligible(NotEligibleReason::StaleControl("NC-0702".to_owned()))
    );

    // 3. GATE-M2-ONLY whose only control is due M2 is NotEligible at M0 (not-yet-due never supports a pass).
    let v_only = run_controls_for_gate("GATE-M2-ONLY", true, &ctrls, &cfg_m0, |_, _| {
        unreachable!("M2 control must not run at M0")
    });
    assert_eq!(v_only.lines.len(), 1);
    assert_eq!(v_only.lines[0].status, ControlStatus::NotYetDue);
    assert_eq!(
        *v_only.gate_line.outcome(),
        GateOutcome::NotEligible(NotEligibleReason::NoControl)
    );
}

// ============================================================================
// 4. Mutation guard (manifest rules + real release compile-fail)
// ============================================================================

#[test]
fn mutation_guard() {
    let controls_toml = r#"schema = "frob.gauntlet.controls/1"

[[control]]
id = "NC-0412"
seed = "CX-262"
gate = ["GATE-ARENA-04"]
kind = "mutation"
target = "frob-guard-probe::stage"
defect = "explicit Euler"
expect = "spectral radius 1.497"
feature = "cx-262"
due = "M0"
"#;
    let guarded_lib = r#"#[cfg(all(feature = "conformance", not(debug_assertions)))]
compile_error!("frob-guard-probe: conformance forbidden in release builds");

pub fn probe() -> u32 { 1 }
"#;

    // 1. Control feature NOT implying `conformance` -> fails naming crate and feature.
    let ws1 = create_scratch_mutation_workspace(
        "mg_no_impl",
        "frob-guard-probe",
        "default = []\nconformance = []\ncx-262 = []\n",
        guarded_lib,
        "",
        controls_toml,
    );
    let r1 = check_mutation_guard(&ws1, true).expect("run guard ws1");
    assert!(
        !r1.is_ok(),
        "feature without conformance implication must fail"
    );
    assert!(
        r1.violations
            .iter()
            .any(|v| v.crate_name == "frob-guard-probe" && v.feature == "cx-262"),
        "expected violation naming crate and feature, got: {:?}",
        r1.violations
    );

    // 2. Mutation feature listed in `default` -> fails naming crate and feature.
    let ws2 = create_scratch_mutation_workspace(
        "mg_in_default",
        "frob-guard-probe",
        "default = [\"cx-262\"]\nconformance = []\ncx-262 = [\"conformance\"]\n",
        guarded_lib,
        "",
        controls_toml,
    );
    let r2 = check_mutation_guard(&ws2, true).expect("run guard ws2");
    assert!(!r2.is_ok(), "mutation feature in default must fail");
    assert!(
        r2.violations
            .iter()
            .any(|v| v.crate_name == "frob-guard-probe"
                && v.feature == "cx-262"
                && v.rule == "mutation-in-default"),
        "got: {:?}",
        r2.violations
    );

    // 3. Crate declaring `conformance` without compile_error! guard line -> fails naming crate & feature.
    let ws3 = create_scratch_mutation_workspace(
        "mg_no_guard_line",
        "frob-guard-probe",
        "default = []\nconformance = []\ncx-262 = [\"conformance\"]\n",
        "pub fn probe() -> u32 { 1 }\n",
        "",
        controls_toml,
    );
    let r3 = check_mutation_guard(&ws3, true).expect("run guard ws3");
    assert!(
        !r3.is_ok(),
        "crate lacking release compile_error! guard must fail"
    );
    assert!(
        r3.violations
            .iter()
            .any(|v| v.crate_name == "frob-guard-probe"
                && (v.feature == "cx-262" || v.feature == "conformance")
                && v.rule == "missing-release-guard"),
        "got: {:?}",
        r3.violations
    );

    // 4. Clean scratch crate passes guard AND compile-fail: `--release --features cx-262` does NOT compile!
    let ws_ok = create_scratch_mutation_workspace(
        "mg_clean_compile_fail",
        "frob-guard-probe",
        "default = []\nconformance = []\ncx-262 = [\"conformance\"]\n",
        guarded_lib,
        "",
        controls_toml,
    );
    // Also add release guard to scratch frob-conformance so ws_ok is 100% clean.
    fs::write(
        ws_ok.join("crates/frob-conformance/src/lib.rs"),
        "#[cfg(all(feature = \"gauntlet-mutations\", not(debug_assertions)))]\ncompile_error!(\"forbidden\");\n",
    )
    .expect("write conf guard");
    let r_ok = check_mutation_guard(&ws_ok, true).expect("run guard ws_ok");
    assert!(
        r_ok.is_ok(),
        "clean scratch workspace must pass: {:?}",
        r_ok.violations
    );

    let err_out = verify_release_build_fails(
        &ws_ok.join("crates/frob-guard-probe/Cargo.toml"),
        None,
        "cx-262",
        &isolated_target_dir("mg-release-fail"),
        true,
    )
    .expect("release build with --features cx-262 MUST fail to compile");
    assert!(
        err_out.contains("conformance forbidden in release builds"),
        "expected compile_error! message, got:\n{err_out}"
    );
}

// ============================================================================
// 5. Rule 4 (suspect report & dispositions) and Rule 5 (permanent controls)
// ============================================================================

fn sample_gates_and_controls() -> (GateRegistry, Controls) {
    let gates = parse_gates(
        r#"schema = "frob.gates/1"

[[gate]]
id = "GATE-GAUNT-01"
claim = "known-answer"
claim_id = "INV-GAUNT-02"
section = "38.15"
threshold = "every pass gate catches its controls"
milestone = "M0"
tier = ["G0:host"]
controls = ["NC-0001", "NC-0002"]

[[gate.part]]
milestone = "M0"
status = "pass"
measured = "ok"
evidence = []
recorded = { date = "2026-11-24", session = "rec-1", implemented_by = "" }

[[gate]]
id = "GATE-ARENA-04"
claim = "known-answer"
claim_id = "GATE-ARENA-04"
section = "24.17"
threshold = "wheel-loop stability"
milestone = "M1"
tier = ["G1:host"]
controls = ["NC-0412"]

[[gate.part]]
milestone = "M1"
status = "not-run"
measured = ""
evidence = []
recorded = { date = "", session = "", implemented_by = "" }
"#,
    );
    let controls = parse_controls(
        r#"schema = "frob.gauntlet.controls/1"

[[control]]
id = "NC-0001"
seed = "GATE-GAUNT-01"
gate = ["GATE-GAUNT-01"]
kind = "planted-input"
target = "frob-conformance::controls::check_registry"
defect = "empty controls pass"
expect = "fails with no-control"
fixture = "fixtures/gauntlet/v0_empty_controls.toml"
due = "M0"

[[control]]
id = "NC-0002"
seed = "GATE-GAUNT-01"
gate = ["GATE-GAUNT-01"]
kind = "planted-input"
target = "frob-conformance::controls::validate"
defect = "never-caught planted control"
expect = "fails with fixture-missing"
fixture = "fixtures/gauntlet/v0_missing_fixture_header.toml"
due = "M0"

[[control]]
id = "NC-0412"
seed = "CX-262"
gate = ["GATE-ARENA-04"]
kind = "mutation"
target = "frob-actuate::stage::integrate"
defect = "explicit Euler"
expect = "spectral radius 1.497"
feature = "cx-262"
due = "M1"
"#,
    );
    (gates, controls)
}

#[test]
fn suspect_listed() {
    let (gates, controls) = sample_gates_and_controls();
    let hist_dir = scratch_dir("suspect_hist");
    let hist_file = hist_dir.join("control_history.ndjson");
    // Only NC-0001 has a recorded `caught` entry; NC-0002 has zero caught runs.
    append_history_entry(
        &hist_file,
        &ControlHistoryEntry {
            id: "NC-0001".to_owned(),
            gate: "GATE-GAUNT-01".to_owned(),
            receipt: "a".repeat(64),
            outcome: "caught".to_owned(),
            commit: "1234567".to_owned(),
            session: "rec-1".to_owned(),
            date: "2026-11-24".to_owned(),
        },
    )
    .expect("append history");
    let history = parse_history_ndjson(
        &fs::read_to_string(&hist_file).unwrap(),
        "control_history.ndjson",
    )
    .expect("parse history");

    let report = build_suspect_report(
        Milestone::M0,
        &gates,
        &controls,
        &history,
        &ControlReviews::default(),
        &BTreeSet::new(),
    );
    assert_eq!(report.suspects.len(), 1);
    assert_eq!(report.suspects[0].id, "NC-0002");
    assert_eq!(report.suspects[0].cause, SuspectCause::NeverCaught);
    assert!(report.blocks_exit());

    // A stale control is also listed even if it had an older caught line.
    let mut stale_set = BTreeSet::new();
    stale_set.insert("NC-0001".to_owned());
    let report_stale = build_suspect_report(
        Milestone::M0,
        &gates,
        &controls,
        &history,
        &ControlReviews::default(),
        &stale_set,
    );
    assert!(
        report_stale
            .suspects
            .iter()
            .any(|s| s.id == "NC-0001" && s.cause == SuspectCause::Stale)
    );
}

#[test]
fn suspect_needs_disposition() {
    let (gates, controls) = sample_gates_and_controls();
    let history = vec![ControlHistoryEntry {
        id: "NC-0001".to_owned(),
        gate: "GATE-GAUNT-01".to_owned(),
        receipt: "b".repeat(64),
        outcome: "caught".to_owned(),
        commit: "1234567".to_owned(),
        session: "rec-1".to_owned(),
        date: "2026-11-24".to_owned(),
    }];

    // Without disposition -> blocks exit with suspect-undisposed.
    let rep_blocked = build_suspect_report(
        Milestone::M0,
        &gates,
        &controls,
        &history,
        &ControlReviews::default(),
        &BTreeSet::new(),
    );
    assert!(rep_blocked.blocks_exit());
    assert!(rep_blocked.format_report().contains("suspect-undisposed"));

    // Withdrawn without a reason is refused by the review parser.
    let bad_withdrawn = r#"schema = "frob.gauntlet.control_reviews/1"

[[review]]
id = "NC-0002"
disposition = "withdrawn"
reason = ""
milestone = "M0"
"#;
    assert!(ControlReviews::parse_str(bad_withdrawn, CONTROL_REVIEWS_PATH).is_err());

    // Valid disposition (`replanted`, `gate-strengthened`, or `withdrawn` with reason) unblocks exit.
    for (disp, reason) in [
        ("replanted", "defect restated on current API"),
        (
            "gate-strengthened",
            "added assertion on missing fixture header",
        ),
        ("withdrawn", "superseded by NC-0009 after schema migration"),
    ] {
        let rev_toml = format!(
            r#"schema = "frob.gauntlet.control_reviews/1"

[[review]]
id = "NC-0002"
disposition = "{disp}"
reason = "{reason}"
milestone = "M0"
session = "rev-1"
date = "2026-11-24"
"#
        );
        let reviews = ControlReviews::parse_str(&rev_toml, CONTROL_REVIEWS_PATH)
            .expect("parse valid disposition");
        let rep = build_suspect_report(
            Milestone::M0,
            &gates,
            &controls,
            &history,
            &reviews,
            &BTreeSet::new(),
        );
        assert!(
            !rep.blocks_exit(),
            "disposition `{disp}` must unblock exit: {}",
            rep.format_report()
        );
        assert_eq!(rep.suspects[0].disposition, Disposition::parse(disp));
    }
}

#[test]
fn suspect_milestone_from_env() {
    let (gates, controls) = sample_gates_and_controls();
    let empty_hist = Vec::new();
    let empty_rev = ControlReviews::default();
    let empty_stale = BTreeSet::new();

    // Explicit `FROB_MILESTONE=M1` selects controls of gates due by M1 (M0 + M1).
    let m1 = resolve_review_milestone(Some("M1"), &gates).expect("resolve M1");
    assert_eq!(m1, Milestone::M1);
    let rep_m1 = build_suspect_report(m1, &gates, &controls, &empty_hist, &empty_rev, &empty_stale);
    assert_eq!(
        rep_m1.due_controls,
        vec!["NC-0001", "NC-0002", "NC-0412"],
        "M1 review must select all controls due by M1"
    );

    // Unset `FROB_MILESTONE` takes the current milestone from gates.toml (M0), NEVER "no controls".
    let m_cur =
        resolve_review_milestone(Some(""), &gates).expect("resolve default current milestone");
    assert_eq!(m_cur, Milestone::M0);
    let rep_m0 = build_suspect_report(
        m_cur,
        &gates,
        &controls,
        &empty_hist,
        &empty_rev,
        &empty_stale,
    );
    assert!(
        !rep_m0.due_controls.is_empty(),
        "unset FROB_MILESTONE must select current milestone's controls, never empty"
    );
    assert_eq!(rep_m0.due_controls, vec!["NC-0001", "NC-0002"]);
}

#[test]
fn permanent_cannot_be_removed() {
    let ws = scratch_dir("perm_ctrl");
    let gates = parse_gates(
        r#"schema = "frob.gates/1"

[[gate]]
id = "GATE-GAUNT-01"
claim = "known-answer"
claim_id = "INV-GAUNT-02"
section = "38.15"
threshold = "permanent control rule"
milestone = "M0"
tier = ["G0:host"]
controls = ["NC-0001", "NC-0002"]

[[gate.part]]
milestone = "M0"
status = "not-run"
measured = ""
evidence = []
recorded = { date = "", session = "", implemented_by = "" }

[[gate]]
id = "GATE-GAUNT-10"
claim = "known-answer"
claim_id = "INV-GAUNT-06"
section = "38.15"
threshold = "status accounting"
milestone = "M0"
tier = ["G0:host"]
controls = ["NC-0002"]

[[gate.part]]
milestone = "M0"
status = "not-run"
measured = ""
evidence = []
recorded = { date = "", session = "", implemented_by = "" }
"#,
    );

    let prev_src = r#"schema = "frob.gauntlet.controls/1"

[[control]]
id = "NC-0001"
seed = "GATE-GAUNT-01"
gate = ["GATE-GAUNT-01"]
kind = "planted-input"
target = "frob-conformance::controls::check_registry"
defect = "empty controls pass"
expect = "fails with no-control"
fixture = "fixtures/gauntlet/v0_empty_controls.toml"

[[control]]
id = "NC-0002"
seed = "CX-900"
gate = ["GATE-GAUNT-01", "GATE-GAUNT-10"]
kind = "planted-input"
target = "frob-conformance::controls::check_registry"
defect = "false certificate found in use becomes permanent control"
expect = "rejects false certificate"
fixture = "fixtures/gauntlet/v0_empty_controls.toml"
permanent = true
"#;
    let prev = parse_controls(prev_src);

    // 1. Deleting permanent control NC-0002 -> PermanentControlRemoved.
    let deleted_src = r#"schema = "frob.gauntlet.controls/1"

[[control]]
id = "NC-0001"
seed = "GATE-GAUNT-01"
gate = ["GATE-GAUNT-01"]
kind = "planted-input"
target = "frob-conformance::controls::check_registry"
defect = "empty controls pass"
expect = "fails with no-control"
fixture = "fixtures/gauntlet/v0_empty_controls.toml"
"#;
    let err_del = load_and_validate(deleted_src, CONTROLS_PATH, Some(&prev), &gates, &ws)
        .expect_err("deleting a permanent control must fail");
    assert!(
        err_del
            .iter()
            .any(|f| f.record == "NC-0002" && f.reason == ControlReason::PermanentControlRemoved),
        "expected permanent-control-removed on deletion, got: {err_del:?}"
    );

    // 2. Detaching GATE-GAUNT-10 from permanent control NC-0002 -> PermanentControlRemoved.
    let detached_src = prev_src.replace(
        r#"gate = ["GATE-GAUNT-01", "GATE-GAUNT-10"]"#,
        r#"gate = ["GATE-GAUNT-01"]"#,
    );
    let err_det = load_and_validate(&detached_src, CONTROLS_PATH, Some(&prev), &gates, &ws)
        .expect_err("detaching a gate from a permanent control must fail");
    assert!(
        err_det
            .iter()
            .any(|f| f.record == "NC-0002" && f.reason == ControlReason::PermanentControlRemoved),
        "expected permanent-control-removed on detach, got: {err_det:?}"
    );

    // 3. Clearing permanent = true -> PermanentControlRemoved.
    let unperm_src = prev_src.replace("permanent = true", "permanent = false");
    let err_unperm = load_and_validate(&unperm_src, CONTROLS_PATH, Some(&prev), &gates, &ws)
        .expect_err("clearing permanent flag must fail");
    assert!(
        err_unperm
            .iter()
            .any(|f| f.record == "NC-0002" && f.reason == ControlReason::PermanentControlRemoved),
        "expected permanent-control-removed on clearing permanent flag, got: {err_unperm:?}"
    );
}

#[test]
fn runner_verdict_deterministic_twice() {
    let root = workspace_root();
    let (controls, _) = Controls::load(&root).expect("load real controls.toml");
    let cfg = ControlRunnerConfig::new(root, Milestone::M0);
    let v1 = run_controls_for_gate_default("GATE-GAUNT-01", true, &controls, &cfg);
    let v2 = run_controls_for_gate_default("GATE-GAUNT-01", true, &controls, &cfg);
    assert_eq!(v1.lines, v2.lines);
    assert_eq!(v1.gate_line, v2.gate_line);
    assert_eq!(*v1.gate_line.outcome(), GateOutcome::Pass);
    assert_eq!(v1.caught_count(), 1);
}
