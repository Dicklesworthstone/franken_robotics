//! N25b-01 lanes for a commit-pinned RCH Cargo test with all siblings bound.

use std::fs;
use std::path::Path;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use xtask::e2e_json::{JsonValue, parse_strict};
use xtask::e2e_runner::run_script;

#[test]
#[ignore = "pinned e2e: the orchestrator runs it with --ignored"]
fn proof_objects_plain() {
    proof_objects_run(false);
}

#[test]
#[ignore = "pinned e2e: the orchestrator runs it with --ignored"]
fn proof_objects_verify_plants() {
    proof_objects_run(true);
}

fn proof_objects_run(verify_plants: bool) {
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
        .join(format!("proof-{}-{stamp}", std::process::id()));
    fs::create_dir_all(&work).expect("create private proof lane directory");
    let logs = work.join("logs");
    let target = work.join("target");
    let target_dir = target.to_str().unwrap();
    let mut env = vec![
        ("CARGO_TARGET_DIR", target_dir),
        ("CARGO_BUILD_BUILD_DIR", target_dir),
        ("CARGO_BUILD_JOBS", "1"),
        ("CARGO_NET_OFFLINE", "true"),
        ("E2E_LOGS_ROOT", logs.to_str().unwrap()),
        // The script runs Cargo from scratch and needs the workspace's .cargo/config.toml.
        ("E2E_SCRATCH_ROOT", ""),
        ("E2E_DRY_RUN", "0"),
    ];
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

    let mut args = vec!["--script", "proof_objects"];
    if verify_plants {
        args.push("--verify-plants");
    }
    let run = run_script("scripts/e2e/run-all.sh", &args, &env).expect("launch proof_objects");
    assert_eq!(run.status.code(), Some(0), "proof_objects {args:?}");
    let stdout = String::from_utf8_lossy(&run.stdout);
    let passes: Vec<_> = stdout
        .lines()
        .filter_map(|line| line.strip_prefix("PASS  proof_objects "))
        .map(str::trim_start)
        .collect();
    if verify_plants {
        assert_eq!(
            passes,
            [
                "plant=torque-alias exit=1 (plant caught)",
                "plant=empty-support exit=1 (plant caught)",
            ]
        );
    } else {
        assert_eq!(passes.len(), 1, "{stdout}");
        assert!(passes[0].starts_with("exit=0 lint=ok log="), "{stdout}");
    }

    let mut count = 0;
    for entry in fs::read_dir(logs.join("proof_objects")).expect("find proof_objects logs") {
        let path = entry.unwrap().path();
        if path.extension().and_then(|ext| ext.to_str()) == Some("ndjson") {
            assert_pinned_cargo(&path, &manifest);
            count += 1;
        }
    }
    assert_eq!(count, passes.len(), "one fresh log per PASS");
}

fn assert_pinned_cargo(log: &Path, manifest: &Path) {
    let text = fs::read_to_string(log).expect("read proof_objects log");
    let mut commands = 0;
    for line in text.lines() {
        let row = parse_strict(line).expect("parse proof_objects log row");
        let field = |name| row.get(name).and_then(JsonValue::as_str);
        if field("event") != Some("cmd") {
            continue;
        }
        let argv = row
            .get("argv")
            .and_then(JsonValue::as_array)
            .expect("command argv");
        if argv.iter().any(|arg| arg.as_str() == Some("cargo")) {
            let pinned = argv.windows(2).any(|pair| {
                pair[0].as_str() == Some("--manifest-path") && pair[1].as_str() == manifest.to_str()
            });
            assert!(pinned, "Cargo used a different manifest: {line}");
            assert!(
                !field("step").unwrap_or_default().is_empty(),
                "Cargo ran outside a step: {line}"
            );
            commands += 1;
        }
    }
    assert!(commands > 0, "no executed Cargo steps in {log:?}");
}
