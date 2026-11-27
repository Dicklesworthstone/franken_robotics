//! N38b-19 recorder lanes for a commit-pinned RCH Cargo test.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use xtask::e2e_json::{JsonValue, parse_strict};
use xtask::e2e_runner::run_script;

const PLANTS: [&str; 4] = [
    "self-certify",
    "missed-control",
    "tampered-receipt",
    "unknown-mode",
];
const REQUIRED_CHECKS: [&str; 10] = [
    "plant_self_certify_reason",
    "plant_missed_control_reason",
    "plant_tampered_receipt_reason",
    "committed_receipt_matches",
    "recorded_readme_measured_and_recorded",
    "implicit_record_mode_noop",
    "implicit_record_mode_unchanged",
    "unknown_mode_word_reason",
    "unknown_mode_word_unchanged",
    "checkout_gates_unchanged",
];

#[test]
#[ignore = "pinned e2e: the orchestrator runs it with --ignored"]
fn record_gate_plain() {
    run_record_gate(false);
}

#[test]
#[ignore = "pinned e2e: the orchestrator runs it with --ignored"]
fn record_gate_verify_plants() {
    run_record_gate(true);
}

fn run_record_gate(verify_plants: bool) {
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
        .join(format!("record-gate-{}-{stamp}", std::process::id()));
    fs::create_dir_all(&work).expect("create private recorder lane directory");
    let work = work
        .canonicalize()
        .expect("resolve recorder lane directory");
    let target = work.join("tool-target");
    let build = work.join("build");
    let logs = work.join("logs");
    let scratch = work.join("scratch");
    let mut env = vec![
        ("CARGO_BUILD_JOBS", "1"),
        ("CARGO_NET_OFFLINE", "true"),
        ("CARGO_BUILD_BUILD_DIR", build.to_str().unwrap()),
        ("E2E_DRY_RUN", "0"),
        ("E2E_LOGS_ROOT", logs.to_str().unwrap()),
        ("E2E_SCRATCH_ROOT", scratch.to_str().unwrap()),
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
        .status()
        .expect("launch pinned frob-e2e build");
    assert!(status.success(), "pinned frob-e2e build: {status}");
    let tool = target.join("debug/frob-e2e");
    assert!(tool.is_file(), "pinned frob-e2e missing: {tool:?}");
    env.push(("FROB_E2E_TOOL", tool.to_str().unwrap()));

    let script = "record_gate";
    let mut args = vec!["--script", script];
    if verify_plants {
        args.push("--verify-plants");
    }
    let run = run_script("scripts/e2e/run-all.sh", &args, &env).expect("launch record_gate");
    assert_eq!(run.status.code(), Some(0), "record_gate {args:?}");
    let mut runs = BTreeMap::new();
    for entry in fs::read_dir(logs.join(script)).expect("find record_gate logs") {
        let path = entry.expect("read record_gate log entry").path();
        if path.extension().is_some_and(|ext| ext == "ndjson") {
            let plant = assert_recorder_log(&path, &manifest);
            assert!(
                runs.insert(plant.clone(), path).is_none(),
                "duplicate run for {plant:?}"
            );
        }
    }

    let stdout = String::from_utf8_lossy(&run.stdout);
    let passes: Vec<_> = stdout
        .lines()
        .filter(|line| line.starts_with("PASS "))
        .collect();
    let expected = if verify_plants {
        assert_eq!(runs.len(), PLANTS.len(), "one fresh log per plant");
        PLANTS
            .iter()
            .map(|plant| {
                assert!(runs.contains_key(*plant), "missing log for {plant}");
                format!("PASS  {script:<32} plant={plant} exit=1 (plant caught)")
            })
            .collect()
    } else {
        assert_eq!(runs.len(), 1, "one fresh log for the plain run");
        let path = runs.get("").expect("plain lane selected a plant");
        vec![format!(
            "PASS  {script:<32} exit=0 lint=ok log={}",
            path.display()
        )]
    };
    assert_eq!(passes, expected, "{stdout}");
}

fn assert_recorder_log(path: &Path, manifest: &Path) -> String {
    let log = fs::read_to_string(path).expect("read record_gate log");
    let mut plant = None;
    let mut cargo_exits = Vec::new();
    let mut checks = BTreeMap::new();
    for line in log.lines() {
        let row = parse_strict(line).expect("parse record_gate log row");
        let field = |name| row.get(name).and_then(JsonValue::as_str);
        match field("event") {
            Some("start") if field("step") == Some("") => {
                plant = Some(field("plant").expect("plant selection").to_owned());
            }
            Some("cmd") => {
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
                    assert!(
                        !field("step").unwrap_or_default().is_empty(),
                        "missing Cargo step: {line}"
                    );
                    cargo_exits.push(
                        row.get("exit")
                            .and_then(JsonValue::as_i64)
                            .expect("Cargo exit"),
                    );
                }
            }
            Some("assert") => {
                let name = field("name").expect("assertion name");
                if PLANTS.contains(&name) || REQUIRED_CHECKS.contains(&name) {
                    let status = field("status").expect("assertion status");
                    assert!(
                        checks.insert(name.to_owned(), status.to_owned()).is_none(),
                        "duplicate recorder assertion: {line}"
                    );
                }
            }
            _ => {}
        }
    }
    assert_eq!(cargo_exits, [0, 3, 3, 3, 0, 0, 0, 3], "{path:?}");
    let plant = plant.expect("record_gate start event");
    for name in PLANTS.into_iter().chain(REQUIRED_CHECKS) {
        let status = if name == plant { "fail" } else { "pass" };
        assert_eq!(
            checks.get(name).map(String::as_str),
            Some(status),
            "{name} in {path:?}"
        );
    }
    plant
}
