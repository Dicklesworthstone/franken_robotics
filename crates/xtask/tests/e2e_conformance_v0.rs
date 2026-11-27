//! BD-025 lanes. Use a commit-pinned RCH Cargo test with bound sibling repositories.
//! Forward `CARGO_TARGET_DIR` with RCH target reuse enabled for the external-scratch lane.

use std::fs;
use std::path::Path;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use xtask::e2e_json as json;
use xtask::e2e_runner::run_script;

#[test]
#[ignore = "pinned e2e: the orchestrator runs it with --ignored"]
fn conformance_v0_plain() {
    conformance_run(&[], false);
}

#[test]
#[ignore = "pinned e2e: the orchestrator runs it with --ignored"]
fn conformance_v0_verify_plants() {
    conformance_run(&["--verify-plants"], false);
}

#[test]
#[ignore = "pinned e2e: the orchestrator runs it with --ignored"]
fn conformance_v0_external_scratch() {
    conformance_run(&[], true);
}

fn conformance_run(flags: &[&str], outside: bool) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("resolve pinned workspace");
    let manifest = root.join("Cargo.toml");
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let work = Path::new(env!("CARGO_TARGET_TMPDIR"))
        .join(format!("bd025-{}-{stamp}", std::process::id()));
    fs::create_dir_all(&work).expect("create private lane directory");
    let work = work.canonicalize().expect("resolve lane directory");
    assert!(
        !outside || !work.starts_with(&root),
        "need external CARGO_TARGET_TMPDIR: {work:?}"
    );
    let scratch = work.join("scratch");
    let scratch_env = if outside {
        scratch.to_str().unwrap()
    } else {
        ""
    };
    let logs = work.join("logs");
    let target = work.join("target");
    let target_dir = target.to_str().unwrap();
    let mut env = vec![
        ("CARGO_TARGET_DIR", target_dir),
        ("CARGO_BUILD_BUILD_DIR", target_dir),
        ("CARGO_BUILD_JOBS", "1"),
        ("CARGO_NET_OFFLINE", "true"),
        ("E2E_LOGS_ROOT", logs.to_str().unwrap()),
        ("E2E_SCRATCH_ROOT", scratch_env),
        ("E2E_DRY_RUN", "0"),
    ];
    // Build the log tool from the pinned manifest so the shell dispatcher cannot pick a cache.
    let status = Command::new("cargo")
        .args(["build", "--locked", "-p", "frob-conformance"])
        .args(["--bin", "frob-e2e", "--manifest-path"])
        .arg(&manifest)
        .current_dir(&root)
        .envs(env.iter().copied())
        .status()
        .expect("launch pinned frob-e2e build");
    assert!(status.success(), "pinned frob-e2e build: {status}");
    let tool = target.join("debug/frob-e2e");
    assert!(tool.is_file(), "pinned frob-e2e missing: {tool:?}");
    env.push(("FROB_E2E_TOOL", tool.to_str().unwrap()));

    // run-all owns --verify-plants and emits one PASS for each successful selected run.
    let mut args = vec!["--script", "conformance_v0"];
    args.extend_from_slice(flags);
    let run = run_script("scripts/e2e/run-all.sh", &args, &env).expect("launch conformance_v0");
    assert_eq!(run.status.code(), Some(0), "conformance_v0 {flags:?}");
    assert!(
        !outside || scratch.join("conformance_v0").is_dir(),
        "unused scratch: {scratch:?}"
    );
    let stdout = String::from_utf8_lossy(&run.stdout);
    let expected = if flags.is_empty() { 1 } else { 3 };
    let passed = stdout
        .lines()
        .filter(|line| line.starts_with("PASS  conformance_v0 "));
    assert_eq!(passed.count(), expected, "{stdout}");

    let mut completed = 0;
    for entry in fs::read_dir(logs.join("conformance_v0")).expect("find conformance_v0 logs") {
        let path = entry.unwrap().path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("ndjson") {
            continue;
        }
        let mut cargo_steps = 0;
        let log = fs::read_to_string(&path).expect("read conformance_v0 log");
        for line in log.lines() {
            let row = json::parse_strict(line).expect("parse conformance_v0 log row");
            let string = |field| row.get(field).and_then(json::JsonValue::as_str);
            if string("event") != Some("cmd") {
                continue;
            }
            let argv = row
                .get("argv")
                .and_then(json::JsonValue::as_array)
                .expect("command argv");
            if argv.iter().any(|arg| arg.as_str() == Some("cargo")) {
                let pinned = argv.windows(2).any(|pair| {
                    pair[0].as_str() == Some("--manifest-path")
                        && pair[1].as_str() == manifest.to_str()
                });
                assert!(pinned, "cargo ran outside the pinned manifest: {line}");
                let step = string("step").unwrap_or_default();
                assert!(!step.is_empty(), "cargo ran outside a step: {line}");
                cargo_steps += 1;
            }
        }
        assert!(cargo_steps > 0, "no executed Cargo steps in {path:?}");
        completed += 1;
    }
    assert_eq!(completed, expected, "one fresh script log per invocation");
}
