#![forbid(unsafe_code)]

//! Pinned AC7 libFuzzer lane for `frob_core::json::admit` (bead N34-17).
//!
//! Builds the standalone `crates/frob-core/fuzz` target `admit` with the pinned
//! nightly toolchain (`nightly-2026-08-31`) and LLVM libFuzzer/ASan flags using
//! `--locked --offline` against the committed `crates/frob-core/fuzz/Cargo.lock`,
//! then runs it in a single process (no `-jobs=` / `-workers=` / `--job`) for
//! `10_000_000` executions and asserts:
//! 1. Exit code `0`;
//! 2. `Done 10000000 runs` in libFuzzer's output;
//! 3. Zero crash, leak, timeout, OOM, or slow-unit artifacts.
//!
//! Also provides `json_admit_fuzz_planted_negative_caught`, which runs the exact
//! same harness against `--features planted-panic` and asserts that the harness
//! fails with a non-zero exit code, the planted panic diagnostic, and a written
//! `crash-*` artifact.
//!
//! Run centrally on a worker with `nightly-2026-08-31` and a C++ compiler (e.g. `ovh-a`):
//!   `cargo test --locked --offline -p frob-core --test json_fuzz_lane -- --ignored --nocapture`

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

const FUZZ_TOOLCHAIN: &str = "nightly-2026-08-31";
const AC7_RUNS: u64 = 10_000_000;
const PLANTED_NEGATIVE_RUNS: u64 = 100_000;
const PLANTED_PANIC_FEATURE: &str = "planted-panic";
const PLANTED_PANIC_MARKER: &str = "planted negative (N34-17 AC7)";

const ARTIFACT_PREFIXES: &[&str] = &["crash-", "leak-", "timeout-", "oom-", "slow-unit-"];

/// Small valid and edge-case JSON seeds so libFuzzer exercises deep RFC 8259
/// grammar, escape decoding, duplicate-key rejection, and plain re-serialization
/// round-trips immediately alongside arbitrary byte mutations.
const AC7_JSON_SEEDS: &[(&str, &[u8])] = &[
    ("seed_null", b"null"),
    ("seed_obj", br#"{"a":1,"b":[true,false,null,-0,1.5e-2]}"#),
    ("seed_dup_escape", br#"{"a":1,"\u0061":2}"#),
    (
        "seed_surrogate",
        br#"{"emoji":"\ud83d\ude00","ctrl":"\u0000\b\f\n\r\t"}"#,
    ),
    ("seed_int53", b"[9007199254740991,9007199254740993,-0]"),
    ("seed_nested", b"[[[[[[[{\"k\":\"v\"}]]]]]]]"),
    ("seed_unicode", "{\"\\u00e9\":1,\"e\\u0301\":2}".as_bytes()),
    ("seed_empty", b"{}"),
];

static LANE_SEQ: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone, PartialEq, Eq)]
struct FuzzRunOutcome {
    exit_code: Option<i32>,
    combined_output: String,
    artifacts: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct FuzzLaneFailure {
    exit_code: Option<i32>,
    artifacts: Vec<String>,
    reasons: Vec<String>,
    combined_output: String,
}

fn fuzz_crate_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("fuzz")
        .canonicalize()
        .expect("resolve pinned crates/frob-core/fuzz directory")
}

fn make_work_dir(label: &str) -> PathBuf {
    let base = option_env!("CARGO_TARGET_TMPDIR")
        .map(PathBuf::from)
        .or_else(|| env::var_os("BIG_SCRATCH").map(PathBuf::from))
        .or_else(|| env::var_os("TMPDIR").map(PathBuf::from))
        .unwrap_or_else(env::temp_dir);
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_nanos());
    let seq = LANE_SEQ.fetch_add(1, Ordering::Relaxed);
    let dir = base.join(format!(
        "frob-n34-17-fuzz-{label}-{}-{nanos}-{seq}",
        std::process::id()
    ));
    fs::create_dir_all(&dir).expect("create private fuzz work directory");
    fs::canonicalize(&dir).unwrap_or(dir)
}

fn query_nightly_host_triple() -> String {
    let output = Command::new("rustc")
        .arg("-vV")
        .env("RUSTUP_TOOLCHAIN", FUZZ_TOOLCHAIN)
        .output()
        .expect("invoke PATH rustc -vV with pinned nightly toolchain");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        output.status.success(),
        "rustc -vV failed under RUSTUP_TOOLCHAIN={FUZZ_TOOLCHAIN}:\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
    assert!(
        stdout.contains("nightly"),
        "expected nightly rustc under RUSTUP_TOOLCHAIN={FUZZ_TOOLCHAIN}, got:\n{stdout}"
    );
    stdout
        .lines()
        .find_map(|line| line.strip_prefix("host:"))
        .map(str::trim)
        .filter(|host| !host.is_empty())
        .map(str::to_owned)
        .expect("rustc -vV output must contain a non-empty `host:` line")
}

fn libfuzzer_rustflags(host_triple: &str) -> String {
    let mut flags = vec![
        "-Cpasses=sancov-module",
        "-Cllvm-args=-sanitizer-coverage-level=4",
        "-Cllvm-args=-sanitizer-coverage-inline-8bit-counters",
        "-Cllvm-args=-sanitizer-coverage-pc-table",
        "-Cllvm-args=-sanitizer-coverage-trace-compares",
        "-Cllvm-args=-simplifycfg-branch-fold-threshold=0",
        "--cfg",
        "fuzzing",
        "-Cdebug-assertions",
        "-Ccodegen-units=1",
        "-Zsanitizer=address",
    ];
    if host_triple.contains("linux") {
        flags.push("-Cllvm-args=-sanitizer-coverage-stack-depth");
    }
    flags.join(" ")
}

fn build_admit_fuzz_binary(
    fuzz_dir: &Path,
    target_dir: &Path,
    host_triple: &str,
    extra_feature: Option<&str>,
) -> PathBuf {
    let manifest = fuzz_dir.join("Cargo.toml");
    let lockfile = fuzz_dir.join("Cargo.lock");
    assert!(
        manifest.is_file(),
        "fuzz manifest missing: {}",
        manifest.display()
    );
    assert!(
        lockfile.is_file(),
        "committed fuzz Cargo.lock missing: {}",
        lockfile.display()
    );

    let rustflags = libfuzzer_rustflags(host_triple);
    let target_rustflags_env = format!(
        "CARGO_TARGET_{}_RUSTFLAGS",
        host_triple.to_ascii_uppercase().replace('-', "_")
    );
    let mut cmd = Command::new("cargo");
    cmd.arg("build")
        .arg("--manifest-path")
        .arg(&manifest)
        .args([
            "--bin",
            "admit",
            "--locked",
            "--offline",
            "--release",
            "--config",
            "profile.release.debug=\"line-tables-only\"",
            "--target",
            host_triple,
            "-j",
            "1",
        ]);
    if let Some(feature) = extra_feature {
        cmd.args(["--features", feature]);
    }
    cmd.current_dir(fuzz_dir)
        .env("RUSTUP_TOOLCHAIN", FUZZ_TOOLCHAIN)
        .env_remove("RUSTC")
        .env("CARGO_TARGET_DIR", target_dir)
        .env_remove("CARGO_BUILD_BUILD_DIR")
        .env_remove("CARGO_ENCODED_RUSTFLAGS")
        .env_remove(&target_rustflags_env)
        .env("RUSTFLAGS", &rustflags)
        .env("CARGO_NET_OFFLINE", "true");

    let output = cmd.output().expect("launch cargo build for admit fuzz bin");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        output.status.success(),
        "building pinned admit fuzz binary failed (feature={extra_feature:?}, target={host_triple}):\n\
         RUSTFLAGS={rustflags}\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );

    let bin = target_dir.join(host_triple).join("release").join("admit");
    assert!(
        bin.is_file(),
        "compiled admit fuzz binary not found at {}",
        bin.display()
    );
    bin
}

fn collect_fuzz_artifacts(artifacts_dir: &Path, work_dir: &Path) -> Vec<String> {
    let mut found = Vec::new();
    if let Ok(entries) = fs::read_dir(artifacts_dir) {
        for entry in entries.flatten() {
            if entry.path().is_file()
                && let Some(name) = entry.file_name().to_str()
            {
                found.push(name.to_owned());
            }
        }
    }
    if let Ok(entries) = fs::read_dir(work_dir) {
        for entry in entries.flatten() {
            if entry.path().is_file()
                && let Some(name) = entry.file_name().to_str()
                && ARTIFACT_PREFIXES.iter().any(|p| name.starts_with(p))
            {
                found.push(name.to_owned());
            }
        }
    }
    found.sort();
    found.dedup();
    found
}

fn validate_fuzz_run(outcome: &FuzzRunOutcome, expected_runs: u64) -> Result<(), FuzzLaneFailure> {
    let mut reasons = Vec::new();
    if outcome.exit_code != Some(0) {
        reasons.push(format!("expected exit code 0, got {:?}", outcome.exit_code));
    }
    let expected_done = format!("Done {expected_runs} runs");
    if !outcome
        .combined_output
        .lines()
        .any(|line| line.starts_with(&expected_done))
    {
        reasons.push(format!(
            "missing required libFuzzer completion line `{expected_done}`"
        ));
    }
    if !outcome.artifacts.is_empty() {
        reasons.push(format!(
            "unexpected fuzz artifacts present: {:?}",
            outcome.artifacts
        ));
    }
    if reasons.is_empty() {
        Ok(())
    } else {
        Err(FuzzLaneFailure {
            exit_code: outcome.exit_code,
            artifacts: outcome.artifacts.clone(),
            reasons,
            combined_output: outcome.combined_output.clone(),
        })
    }
}

fn run_admit_fuzz_lane(
    label: &str,
    runs: u64,
    extra_feature: Option<&str>,
    seed_corpus: bool,
) -> Result<FuzzRunOutcome, FuzzLaneFailure> {
    let fuzz_dir = fuzz_crate_dir();
    let host_triple = query_nightly_host_triple();
    let work_dir = make_work_dir(label);
    let target_dir = work_dir.join("target");
    let corpus_dir = work_dir.join("corpus");
    let artifacts_dir = work_dir.join("artifacts");
    fs::create_dir_all(&corpus_dir).expect("create corpus dir");
    fs::create_dir_all(&artifacts_dir).expect("create artifacts dir");

    if seed_corpus {
        for (name, bytes) in AC7_JSON_SEEDS {
            fs::write(corpus_dir.join(name), bytes).expect("write JSON fuzz seed");
        }
    }

    // Build failures panic immediately so a broken build can never masquerade as a
    // caught planted negative.
    let bin = build_admit_fuzz_binary(&fuzz_dir, &target_dir, &host_triple, extra_feature);

    let artifact_prefix = format!("-artifact_prefix={}/", artifacts_dir.display());
    let runs_arg = format!("-runs={runs}");

    // Single-process libFuzzer run (no `-jobs=` / `-workers=` / `--job`).
    let output = Command::new(&bin)
        .arg(&artifact_prefix)
        .arg(&runs_arg)
        .arg(&corpus_dir)
        .current_dir(&work_dir)
        .env("ASAN_OPTIONS", "detect_odr_violation=0")
        .env("RUST_BACKTRACE", "1")
        .output()
        .expect("execute compiled admit fuzz binary");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let combined_output = format!("{stdout}\n{stderr}");
    let artifacts = collect_fuzz_artifacts(&artifacts_dir, &work_dir);

    let outcome = FuzzRunOutcome {
        exit_code: output.status.code(),
        combined_output,
        artifacts,
    };

    validate_fuzz_run(&outcome, runs)?;
    Ok(outcome)
}

#[test]
fn fuzz_manifest_and_lockfile_present_and_validator_enforces_ac7() {
    let fuzz_dir = fuzz_crate_dir();
    let manifest = fs::read_to_string(fuzz_dir.join("Cargo.toml")).expect("read fuzz Cargo.toml");
    let lockfile = fs::read_to_string(fuzz_dir.join("Cargo.lock")).expect("read fuzz Cargo.lock");
    let target_src =
        fs::read_to_string(fuzz_dir.join("fuzz_targets/admit.rs")).expect("read admit.rs");

    assert!(manifest.contains("name = \"frob-core-fuzz\""));
    assert!(manifest.contains("name = \"admit\""));
    assert!(manifest.contains("planted-panic = []"));
    assert!(lockfile.contains("name = \"libfuzzer-sys\""));
    assert!(lockfile.contains("name = \"frob-core\""));
    assert!(target_src.contains("plain_reserialize"));
    assert!(target_src.contains("eq_ordered"));
    assert!(target_src.contains(PLANTED_PANIC_MARKER));

    let clean = FuzzRunOutcome {
        exit_code: Some(0),
        combined_output: "Done 10000000 runs in 19 second(s)\n".to_owned(),
        artifacts: Vec::new(),
    };
    assert!(validate_fuzz_run(&clean, AC7_RUNS).is_ok());

    let short_run = FuzzRunOutcome {
        exit_code: Some(0),
        combined_output: "Done 999 runs in 1 second(s)\n".to_owned(),
        artifacts: Vec::new(),
    };
    assert!(validate_fuzz_run(&short_run, AC7_RUNS).is_err());

    let nonzero = FuzzRunOutcome {
        exit_code: Some(77),
        combined_output: "Done 10000000 runs in 19 second(s)\n".to_owned(),
        artifacts: Vec::new(),
    };
    assert!(validate_fuzz_run(&nonzero, AC7_RUNS).is_err());

    for artifact in [
        "crash-da39a3ee5e6b4b0d3255bfef95601890afd80709",
        "leak-0123456789abcdef",
        "timeout-0123456789abcdef",
    ] {
        let with_artifact = FuzzRunOutcome {
            exit_code: Some(0),
            combined_output: "Done 10000000 runs in 19 second(s)\n".to_owned(),
            artifacts: vec![artifact.to_owned()],
        };
        assert!(
            validate_fuzz_run(&with_artifact, AC7_RUNS).is_err(),
            "validator must reject artifact `{artifact}` even on exit 0"
        );
    }
}

#[test]
#[ignore = "pinned N34-17 AC7 fuzz lane (10^7 runs): run on ovh-a with --ignored --nocapture"]
fn json_admit_fuzz_10m() {
    let outcome = run_admit_fuzz_lane("ac7-10m", AC7_RUNS, None, true).unwrap_or_else(|err| {
        let tail: Vec<&str> = err
            .combined_output
            .lines()
            .rev()
            .take(40)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect();
        panic!(
            "N34-17 AC7 10^7 fuzz lane failed: {:?}\nexit={:?}, artifacts={:?}\noutput tail:\n{}",
            err.reasons,
            err.exit_code,
            err.artifacts,
            tail.join("\n")
        );
    });
    for line in outcome.combined_output.lines() {
        if line.starts_with("Done ")
            || line.starts_with("stat::number_of_executed_units:")
            || line.starts_with("stat::average_exec_per_sec:")
            || line.starts_with("stat::peak_rss_mb:")
        {
            eprintln!("{line}");
        }
    }
}

#[test]
#[ignore = "pinned N34-17 AC7 fuzz planted negative: run on ovh-a with --ignored --nocapture"]
fn json_admit_fuzz_planted_negative_caught() {
    let failure = run_admit_fuzz_lane(
        "ac7-planted-panic",
        PLANTED_NEGATIVE_RUNS,
        Some(PLANTED_PANIC_FEATURE),
        false,
    )
    .expect_err("planted-panic fuzz target MUST be caught by the fuzz lane validator");

    assert_ne!(
        failure.exit_code,
        Some(0),
        "planted panic must exit non-zero, got: {:?}",
        failure
    );
    assert!(
        failure.combined_output.contains(PLANTED_PANIC_MARKER),
        "expected planted panic diagnostic `{PLANTED_PANIC_MARKER}` in output:\n{}",
        failure.combined_output
    );
    assert!(
        failure
            .artifacts
            .iter()
            .any(|name| name.starts_with("crash-")),
        "expected libFuzzer to write a `crash-*` artifact for planted panic, got: {:?}",
        failure.artifacts
    );
    eprintln!(
        "planted negative caught: exit={:?} artifacts={:?}",
        failure.exit_code, failure.artifacts
    );
}
