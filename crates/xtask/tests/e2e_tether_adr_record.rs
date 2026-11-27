//! N32a-01 Tether ADR conformance lane for a commit-pinned RCH Cargo test.

use std::fs;
use std::path::Path;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use frob_conformance::e2e::json::{JsonValue, parse_strict};
use xtask::e2e_runner::run_script;

#[test]
#[ignore = "pinned e2e: the orchestrator runs it with --ignored"]
fn tether_adr_record_pinned() {
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
        .join(format!("tether-adr-{}-{stamp}", std::process::id()));
    fs::create_dir_all(&work).expect("create private Tether ADR lane directory");
    let work = work.canonicalize().expect("resolve lane directory");
    let target = work.join("tool-target");
    let build = work.join("suite-build");
    let logs = work.join("logs");
    let scratch = work.join("scratch");
    let mut env = vec![
        ("CARGO_BUILD_JOBS", "1"),
        ("CARGO_NET_OFFLINE", "true"),
        ("CARGO_BUILD_BUILD_DIR", build.to_str().unwrap()),
        ("E2E_DRY_RUN", "0"),
        ("E2E_LOGS_ROOT", logs.to_str().unwrap()),
        ("E2E_SCRATCH_ROOT", scratch.to_str().unwrap()),
        ("FROB_CONFORMANCE_ROOT", root.to_str().unwrap()),
        ("RCH_TARGET_BASE", work.to_str().unwrap()),
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
        .env("CARGO_TARGET_DIR", &target)
        .env("CARGO_BUILD_BUILD_DIR", &target)
        .status()
        .expect("launch pinned frob-e2e build");
    assert!(status.success(), "pinned frob-e2e build: {status}");
    let tool = target.join("debug/frob-e2e");
    assert!(tool.is_file(), "pinned frob-e2e missing: {tool:?}");
    env.push(("FROB_E2E_TOOL", tool.to_str().unwrap()));

    let script = "tether_adr_record";
    let run = run_script("scripts/e2e/run-all.sh", &["--script", script], &env)
        .expect("launch tether_adr_record");
    assert_eq!(run.status.code(), Some(0), "{script}");
    let script_logs: Vec<_> = fs::read_dir(logs.join(script))
        .expect("find tether_adr_record logs")
        .map(|entry| entry.expect("read tether_adr_record log entry").path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "ndjson"))
        .collect();
    let [log_path] = script_logs.as_slice() else {
        panic!("expected one fresh {script} log, got {script_logs:?}");
    };
    let stdout = String::from_utf8_lossy(&run.stdout);
    let passes: Vec<_> = stdout
        .lines()
        .filter(|line| line.starts_with("PASS "))
        .collect();
    let pass = format!(
        "PASS  {script:<32} exit=0 lint=ok log={}",
        log_path.display()
    );
    assert_eq!(passes, [pass.as_str()], "{stdout}");

    assert_tether_controls(log_path, &manifest);
}

fn assert_tether_controls(path: &Path, manifest: &Path) {
    let log = fs::read_to_string(path).expect("read Tether ADR log");
    let mut cargo_runs = Vec::new();
    let mut controls = Vec::new();
    for line in log.lines() {
        let row = parse_strict(line).expect("parse Tether ADR log row");
        let field = |name| row.get(name).and_then(JsonValue::as_str);
        if field("event") == Some("cmd") {
            let argv = row
                .get("argv")
                .and_then(JsonValue::as_array)
                .expect("command argv");
            if let Some(cargo) = argv.iter().position(|arg| arg.as_str() == Some("cargo")) {
                assert!(
                    argv[cargo + 1..].windows(2).any(|args| {
                        args[0].as_str() == Some("--manifest-path")
                            && args[1].as_str() == manifest.to_str()
                    }),
                    "Cargo ran without the pinned manifest: {line}"
                );
                let step = field("step").expect("Cargo command step");
                let exit = row
                    .get("exit")
                    .and_then(JsonValue::as_i64)
                    .expect("Cargo exit");
                cargo_runs.push((step.to_owned(), exit));
            }
        }
        if field("event") == Some("assert") && field("step") == Some("negative_controls") {
            let name = field("name").expect("negative-control name");
            assert_eq!(field("status"), Some("pass"), "{line}");
            assert_eq!(field("observed"), Some("caught"), "{line}");
            assert_eq!(field("expected"), Some("caught"), "{line}");
            controls.push(name.to_owned());
        }
    }
    let cargo_runs: Vec<_> = cargo_runs
        .iter()
        .map(|(step, exit)| (step.as_str(), *exit))
        .collect();
    assert_eq!(
        cargo_runs,
        [
            ("conformance_clean", 0),
            ("negative_controls", 101),
            ("negative_controls", 101)
        ]
    );
    assert_eq!(
        controls,
        [
            "nc_empty_enforced_by_caught",
            "nc_pending_beyond_milestone_caught"
        ]
    );
}
