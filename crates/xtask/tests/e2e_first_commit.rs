//! N34-16 first_commit.sh lanes for a commit-pinned RCH Cargo test.
#![deny(unsafe_code)]

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use xtask::e2e_json::{JsonValue, parse_strict};
use xtask::e2e_runner::run_script;

const SELFTEST_CASES: [&str; 14] = [
    "miniature_zero_crates_pass",
    "determinism_two_runs",
    "plant_step2_missing_dependency_admission",
    "plant_step3_duplicate_format_extension",
    "plant_step4_annex_toolchain_file",
    "plant_step5_edited_receipt_bytes",
    "live_dependency_proof_sibling_rev",
    "plant_step7_hand_edited_status_block",
    "plant_step8_non_compiling_member",
    "plant_step9_missing_p0_upstream_bead",
    "plant_step10_measured_admission_record",
    "plant_step10_measured_gate_isolated",
    "standing_mode_populated_and_10th_with_schema",
    "standing_mode_10th_without_schema_fails_step3",
];

const EXPECTED_STEPS: [&str; 10] = [
    "step_1_clone_and_list",
    "step_2_presence",
    "step_3_registries",
    "step_4_toolchain",
    "step_5_lock_and_receipt",
    "step_6_build_identity_and_patches",
    "step_7_status_table",
    "step_8_no_uncompiled_crate",
    "step_9_initial_beads",
    "step_10_honesty_scan",
];

#[test]
#[ignore = "pinned e2e: the orchestrator runs it with --ignored"]
fn first_commit_selftest_all() {
    let (root, work, tool) = build_pinned_e2e_tool("n34-16-selftest");
    let scratch = work.join("big-scratch");
    let target_base = work.join("rch-target");
    fs::create_dir_all(&scratch).expect("create external scratch dir");
    fs::create_dir_all(&target_base).expect("create external target base");
    assert!(
        !work.starts_with(&root),
        "lane scratch must live outside checkout: {work:?}"
    );

    let vars = [
        ("CARGO_BUILD_JOBS", "1"),
        ("CARGO_NET_OFFLINE", "true"),
        ("BIG_SCRATCH", scratch.to_str().unwrap()),
        ("RCH_TARGET_BASE", target_base.to_str().unwrap()),
        ("FROB_E2E_TOOL", tool.to_str().unwrap()),
        ("E2E_DRY_RUN", "0"),
    ];

    let run = run_script("scripts/e2e/tests/first_commit_selftest.sh", &[], &vars)
        .expect("launch first_commit_selftest.sh");
    assert_eq!(run.status.code(), Some(0), "first_commit_selftest.sh");

    let stdout = String::from_utf8_lossy(&run.stdout);
    let passed: Vec<&str> = stdout
        .lines()
        .filter_map(|line| line.strip_prefix("PASS: "))
        .collect();
    let mut expected: Vec<&str> = SELFTEST_CASES.to_vec();
    expected.push("run_all_verify_plants_all_8");
    assert_eq!(passed, expected, "{stdout}");
    let summary = format!("Summary: passed={} failed=0 skipped=0", expected.len());
    assert_eq!(stdout.lines().last(), Some(summary.as_str()), "{stdout}");
}

#[test]
#[ignore = "pinned e2e: the orchestrator runs it with --ignored"]
fn first_commit_real_47e0ffa9() {
    run_real_commit("n34-16-real-47e0ffa9", "47e0ffa9", "first");
}

#[test]
#[ignore = "pinned e2e: the orchestrator runs it with --ignored"]
fn first_commit_standing_head() {
    run_real_commit("n34-16-standing-head", "HEAD", "standing");
}

fn run_real_commit(tag: &str, rev: &str, mode: &str) {
    let (root, work, tool) = build_pinned_e2e_tool(tag);
    let logs = work.join("logs");
    let scratch = work.join("scratch");
    let target_base = work.join("rch-target");
    fs::create_dir_all(&logs).expect("create lane logs dir");
    fs::create_dir_all(&scratch).expect("create lane scratch dir");
    fs::create_dir_all(&target_base).expect("create lane target base");
    assert!(
        !work.starts_with(&root),
        "lane scratch must live outside checkout: {work:?}"
    );

    let vars = [
        ("CARGO_BUILD_JOBS", "1"),
        ("CARGO_NET_OFFLINE", "true"),
        ("FIRST_COMMIT_REV", rev),
        ("FIRST_COMMIT_MODE", mode),
        ("E2E_LOGS_ROOT", logs.to_str().unwrap()),
        ("E2E_SCRATCH_ROOT", scratch.to_str().unwrap()),
        ("RCH_TARGET_BASE", target_base.to_str().unwrap()),
        ("FROB_E2E_TOOL", tool.to_str().unwrap()),
        ("E2E_DRY_RUN", "0"),
    ];

    let run = run_script(
        "scripts/e2e/run-all.sh",
        &["--script", "first_commit"],
        &vars,
    )
    .expect("launch first_commit via run-all.sh");

    let script_logs: Vec<PathBuf> = fs::read_dir(logs.join("first_commit"))
        .expect("find first_commit logs")
        .map(|entry| entry.expect("read log entry").path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "ndjson"))
        .collect();
    let [log_path] = script_logs.as_slice() else {
        panic!("expected one fresh first_commit log, got {script_logs:?}");
    };

    let stdout = String::from_utf8_lossy(&run.stdout);
    let summary_line = stdout
        .lines()
        .find(|line| {
            line.starts_with("PASS  first_commit ") || line.starts_with("FAIL  first_commit ")
        })
        .unwrap_or_else(|| panic!("missing run-all summary line for first_commit: {stdout}"));
    assert!(
        summary_line.contains("lint=ok") || summary_line.contains("lint=0"),
        "frob-e2e lint failed on {log_path:?}: {summary_line}"
    );

    let log_text = fs::read_to_string(log_path).expect("read first_commit log");
    let mut seen_steps = Vec::new();
    let mut failed_asserts = Vec::new();
    for line in log_text.lines() {
        let row = parse_strict(line).expect("parse first_commit NDJSON row");
        let field = |name| row.get(name).and_then(JsonValue::as_str);
        if field("event") == Some("start")
            && let Some(step) = field("step")
            && !step.is_empty()
            && !seen_steps.iter().any(|s| s == step)
        {
            seen_steps.push(step.to_owned());
        }
        if field("event") == Some("assert") && field("status") == Some("fail") {
            failed_asserts.push(format!(
                "{}:{} (observed={})",
                field("step").unwrap_or(""),
                field("name").unwrap_or(""),
                field("observed").unwrap_or("")
            ));
        }
    }

    assert_eq!(
        seen_steps, EXPECTED_STEPS,
        "all ten steps must execute on {rev} ({mode})"
    );
    assert_eq!(
        run.status.code(),
        Some(0),
        "first_commit.sh --commit {rev} --mode {mode} failed assertions: {failed_asserts:?}"
    );
}

fn build_pinned_e2e_tool(tag: &str) -> (PathBuf, PathBuf, PathBuf) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("resolve pinned workspace");
    let manifest = root.join("Cargo.toml");
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock after epoch")
        .as_nanos();
    let work = Path::new(env!("CARGO_TARGET_TMPDIR"))
        .join(format!("{tag}-{}-{stamp}", std::process::id()));
    fs::create_dir_all(&work).expect("create private lane directory");
    let work = work.canonicalize().expect("resolve lane directory");
    let target = work.join("tool-target");

    let status = Command::new("cargo")
        .args([
            "build",
            "--offline",
            "-j",
            "1",
            "-p",
            "frob-conformance",
            "--bin",
            "frob-e2e",
        ])
        .arg("--manifest-path")
        .arg(&manifest)
        .current_dir(&root)
        .env("CARGO_TARGET_DIR", &target)
        .env("CARGO_BUILD_BUILD_DIR", &target)
        .env("CARGO_NET_OFFLINE", "true")
        .status()
        .expect("launch pinned frob-e2e build");
    assert!(status.success(), "pinned frob-e2e build: {status}");

    let tool = target.join("debug/frob-e2e");
    assert!(tool.is_file(), "pinned frob-e2e missing: {tool:?}");
    (root, work, tool)
}
