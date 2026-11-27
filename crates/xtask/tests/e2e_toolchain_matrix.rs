//! N33c-01's full shell test suite in a pinned RCH Cargo test with its siblings bound.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use xtask::e2e_runner::run_script;

const NIGHTLIES: [&str; 4] = [
    "nightly-2026-07-06",
    "nightly-2026-08-20",
    "nightly-2026-08-25",
    "nightly-2026-08-31",
];
const CASES: [&str; 16] = [
    "cell_pass",
    "cell_fail_names_crate",
    "cell_rust_version",
    "cell_timeout",
    "cell_resolve_missing_path",
    "not_run_is_retried",
    "resume_skips_done",
    "record_is_canonical",
    "probe_is_own_workspace",
    "select_sets_and_channels",
    "siblings_untouched",
    "cell_build_script_salt_unset",
    "harness_exports_no_salt",
    "test_channel_runs_tests_on_sets_3_to_6",
    "exec_cell_timeout_and_limits",
    "classifier_unit_all_7_kinds",
];

#[test]
#[ignore = "pinned e2e: the orchestrator runs it with --ignored"]
fn toolchain_matrix_all_tests() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("resolve pinned workspace");
    assert_required_toolchains(&root);

    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let lane = format!("matrix-{}-{stamp}", std::process::id());
    let work = Path::new(env!("CARGO_TARGET_TMPDIR")).join(&lane);
    fs::create_dir_all(&work).expect("create private matrix lane");
    let work = work.canonicalize().expect("resolve private matrix lane");
    let target = work.join("target");

    // The salt-unset case needs scratch outside the checkout's Cargo configuration.
    let external_base = [work.clone(), env::temp_dir(), PathBuf::from("/tmp")]
        .into_iter()
        .filter_map(|dir| dir.canonicalize().ok())
        .find(|dir| !dir.starts_with(&root))
        .expect("toolchain matrix requires scratch outside the pinned checkout");
    let scratch = external_base.join(&lane);

    let target_dir = target.to_str().unwrap();
    let mut vars = vec![
        ("CARGO_TARGET_DIR", target_dir),
        ("CARGO_NET_OFFLINE", "true"),
        ("BIG_SCRATCH", scratch.to_str().unwrap()),
        ("E2E_DRY_RUN", "0"),
    ];
    let status = Command::new("cargo")
        .args(["build", "--locked", "-j", "1", "-p", "frob-conformance"])
        .args(["--bin", "frob-e2e", "--manifest-path"])
        .arg(root.join("Cargo.toml"))
        .current_dir(&root)
        .envs(vars.iter().copied())
        .env("CARGO_BUILD_BUILD_DIR", &target)
        .status()
        .expect("launch pinned frob-e2e build");
    assert!(status.success(), "pinned frob-e2e build: {status}");
    let tool = target.join("debug/frob-e2e");
    assert!(tool.is_file(), "pinned frob-e2e missing: {tool:?}");
    vars.push(("FROB_E2E_TOOL", tool.to_str().unwrap()));

    let run = run_script(
        "scripts/admission/tests/toolchain_matrix_test.sh",
        &[],
        &vars,
    )
    .expect("launch toolchain matrix test suite");
    assert_eq!(run.status.code(), Some(0), "toolchain matrix shell suite");
    let stdout = String::from_utf8_lossy(&run.stdout);
    let passed: Vec<_> = stdout
        .lines()
        .filter_map(|line| line.strip_prefix("PASS: "))
        .collect();
    assert_eq!(passed, CASES, "{stdout}");
    let summary = format!("Summary: passed={} failed=0 skipped=0", CASES.len());
    assert_eq!(stdout.lines().last(), Some(summary.as_str()));
}

fn assert_required_toolchains(root: &Path) {
    // Listing installed toolchains cannot trigger rustup's implicit installation.
    let out = Command::new("rustup")
        .args(["toolchain", "list"])
        .current_dir(root)
        .output()
        .expect("rustup is required on the matrix worker");
    assert!(
        out.status.success(),
        "rustup toolchain list: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let installed = String::from_utf8_lossy(&out.stdout);
    let missing: Vec<_> = NIGHTLIES
        .into_iter()
        .filter(|channel| {
            !installed
                .lines()
                .filter_map(|line| line.split_whitespace().next())
                .any(|name| {
                    name == *channel
                        || name
                            .strip_prefix(*channel)
                            .is_some_and(|suffix| suffix.starts_with('-'))
                })
        })
        .collect();
    assert!(
        missing.is_empty(),
        "matrix worker is missing: {}",
        missing.join(", ")
    );
}
