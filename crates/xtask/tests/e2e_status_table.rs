//! BD-002 status-table lane. Run under a commit-pinned RCH Cargo test.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use frob_conformance::e2e::json;
use xtask::e2e_runner::run_script;

#[test]
#[ignore = "pinned e2e: the orchestrator runs it with --ignored"]
fn status_table_pinned() {
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
        .join(format!("bd002-{}-{stamp}", std::process::id()));
    fs::create_dir_all(work.parent().expect("Cargo target tmpdir"))
        .expect("create Cargo target tmpdir");
    fs::create_dir(&work).expect("create fresh private lane directory");
    let work = work.canonicalize().expect("resolve lane directory");
    let target = work.join("target");
    let logs = work.join("logs");
    let scratch = work.join("scratch");
    let mut env = vec![
        ("CARGO_TARGET_DIR", target.to_str().unwrap()),
        ("CARGO_BUILD_BUILD_DIR", target.to_str().unwrap()),
        ("CARGO_BUILD_JOBS", "1"),
        ("CARGO_NET_OFFLINE", "true"),
        ("RCH_TARGET_BASE", work.to_str().unwrap()),
        ("E2E_LOGS_ROOT", logs.to_str().unwrap()),
        ("E2E_SCRATCH_ROOT", scratch.to_str().unwrap()),
        ("E2E_DRY_RUN", "0"),
    ];

    let status = Command::new("cargo")
        .args([
            "build",
            "--locked",
            "-p",
            "frob-conformance",
            "--bin",
            "frob-e2e",
        ])
        .arg("--manifest-path")
        .arg(&manifest)
        .current_dir(&root)
        .envs(env.iter().copied())
        .status()
        .expect("launch pinned frob-e2e build");
    assert!(status.success(), "pinned frob-e2e build: {status}");
    let tool = target.join("debug/frob-e2e");
    assert!(tool.is_file(), "pinned frob-e2e missing: {tool:?}");
    env.push(("FROB_E2E_TOOL", tool.to_str().unwrap()));

    let run = run_script(
        "scripts/e2e/run-all.sh",
        &["--script", "status_table", "--devices", "cpu"],
        &env,
    )
    .expect("launch status_table");
    assert_eq!(run.status.code(), Some(0), "status_table");

    let script_logs: Vec<PathBuf> = fs::read_dir(logs.join("status_table"))
        .expect("find status_table logs")
        .map(|entry| entry.expect("read status_table log entry").path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "ndjson"))
        .collect();
    let [log_path] = script_logs.as_slice() else {
        panic!("expected one fresh status_table log, got {script_logs:?}");
    };
    let stdout = String::from_utf8_lossy(&run.stdout);
    let passes: Vec<_> = stdout
        .lines()
        .filter(|line| line.starts_with("PASS "))
        .collect();
    let expected = format!(
        "PASS  {:<32} exit=0 lint=ok log={}",
        "status_table",
        log_path.display()
    );
    assert_eq!(passes, [expected.as_str()], "{stdout}");

    let log = fs::read_to_string(log_path).expect("read status_table log");
    let mut cargo_steps = 0;
    for line in log.lines() {
        let row = json::parse_strict(line).expect("parse status_table log row");
        let string = |field| row.get(field).and_then(json::JsonValue::as_str);
        if string("event") != Some("cmd") {
            continue;
        }
        let argv = row
            .get("argv")
            .and_then(json::JsonValue::as_array)
            .expect("command argv");
        let Some(cargo) = argv.iter().position(|arg| arg.as_str() == Some("cargo")) else {
            continue;
        };
        let pinned = argv[cargo + 1..].windows(2).any(|pair| {
            pair[0].as_str() == Some("--manifest-path") && pair[1].as_str() == manifest.to_str()
        });
        // Negative control: run bless_4row from the workspace root with its manifest flag removed.
        assert!(pinned, "cargo ran without the pinned manifest: {line}");
        assert!(
            string("step").is_some_and(|step| !step.is_empty()),
            "cargo ran outside a step: {line}"
        );
        cargo_steps += 1;
    }
    assert!(cargo_steps > 0, "no executed Cargo steps in {log_path:?}");
}
