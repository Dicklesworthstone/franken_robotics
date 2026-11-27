//! Run shell tests in the source tree Cargo compiled. The outer RCH invocation pins the tree.

use std::collections::{BTreeMap, BTreeSet};
use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::{self, Read, Seek, SeekFrom};
use std::path::{Component, Path, PathBuf};
use std::process::Command;

use crate::e2e_json::{JsonValue, parse_strict};

pub use std::process::Output as ScriptRun;

/// Run a workspace-relative `.sh` file with the given arguments and environment overrides.
/// A nonzero script exit is an ordinary result. Path and launch errors are returned separately.
pub fn run_script(
    rel_path: &str,
    args: &[&str],
    env: &[(&str, &str)],
) -> Result<ScriptRun, String> {
    let path = Path::new(rel_path);
    if path.is_absolute() || path.components().any(|part| part == Component::ParentDir) {
        return Err(format!("script outside workspace: {rel_path}"));
    }
    if path.extension() != Some(OsStr::new("sh")) {
        return Err(format!("script path must end in .sh: {rel_path}"));
    }
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .map_err(|err| format!("resolve Cargo workspace: {err}"))?;
    let script = root
        .join(path)
        .canonicalize()
        .map_err(|err| format!("resolve workspace script {rel_path}: {err}"))?;
    if !script.starts_with(&root) {
        return Err(format!("script outside workspace: {rel_path}"));
    }
    if !script.is_file() || script.extension() != Some(OsStr::new("sh")) {
        return Err(format!("script must resolve to a .sh file: {rel_path}"));
    }
    let logs = e2e_logs(&root, path, args, env);
    // STD-E2E allocates a new log name for every invocation, including each plant.
    let prior_logs = logs.as_deref().map(log_paths).unwrap_or_default();
    let run = Command::new("bash")
        .arg(script)
        .args(args)
        .envs(env.iter().copied())
        .current_dir(root)
        .output()
        .map_err(|err| format!("run workspace script {rel_path}: {err}"))?;
    print!("{}", String::from_utf8_lossy(&run.stdout));
    eprint!("{}", String::from_utf8_lossy(&run.stderr));
    if !run.status.success()
        && let Some(logs) = logs
    {
        let fresh_logs = log_paths(&logs);
        let mut fresh = fresh_logs.difference(&prior_logs).peekable();
        if fresh.peek().is_none() {
            eprintln!("No fresh E2E log under {}", logs.display());
        }
        for log in fresh {
            print_failure_evidence(log);
        }
    }
    Ok(run)
}

fn e2e_logs(root: &Path, script: &Path, args: &[&str], env: &[(&str, &str)]) -> Option<PathBuf> {
    let env_root = env
        .iter()
        .rev()
        .find_map(|&(key, value)| (key == "E2E_LOGS_ROOT").then_some(value));
    let logs_dir = (script == Path::new("scripts/e2e/run-all.sh"))
        .then(|| {
            args.windows(2)
                .rfind(|pair| pair[0] == "--logs-dir")
                .map(|pair| pair[1])
        })
        .flatten();
    if let Some(dir) = logs_dir.or(env_root).filter(|dir| !dir.is_empty()) {
        return Some(root.join(dir));
    }
    if env_root.is_none()
        && script.starts_with("scripts/e2e")
        && let Some(dir) = std::env::var_os("E2E_LOGS_ROOT").filter(|dir| !dir.is_empty())
    {
        return Some(root.join(dir));
    }
    script
        .starts_with("scripts/e2e")
        .then(|| root.join("target/e2e-logs"))
}

fn log_paths(root: &Path) -> BTreeSet<PathBuf> {
    fs::read_dir(root)
        .into_iter()
        .flatten()
        .flatten()
        .filter(|entry| {
            entry.file_name() != OsStr::new("_runs")
                && entry.file_type().is_ok_and(|kind| kind.is_dir())
        })
        .filter_map(|entry| fs::read_dir(entry.path()).ok())
        .flatten()
        .flatten()
        .filter(|entry| entry.file_type().is_ok_and(|kind| kind.is_file()))
        .map(|entry| entry.path())
        .filter(|path| path.extension() == Some(OsStr::new("ndjson")))
        .collect()
}

type LogCommand<'a> = (&'a str, JsonValue);

fn print_failure_evidence(log: &Path) {
    eprintln!("E2E failure evidence from {}", log.display());
    let text = match fs::read_to_string(log) {
        Ok(text) => text,
        Err(err) => {
            eprintln!("cannot read E2E log: {err}");
            return;
        }
    };
    let (failures, commands) = failure_rows(&text);
    for row in failures {
        eprintln!("{row}");
    }
    for (row, command) in commands {
        eprintln!("{row}");
        for (field, stream) in [("stdout_path", "stdout"), ("stderr_path", "stderr")] {
            if let Some(path) = command.get(field).and_then(JsonValue::as_str) {
                print_capture_tail(log, Path::new(path), stream);
            }
        }
    }
}

fn failure_rows(log: &str) -> (Vec<&str>, Vec<LogCommand<'_>>) {
    let mut failures = Vec::new();
    let mut commands = BTreeMap::new();
    let mut implicated = BTreeSet::new();
    let mut last_command = None;
    let mut summary_exit = None;
    for line in log.lines() {
        let Ok(row) = parse_strict(line) else {
            failures.push(line);
            continue;
        };
        match row.get("event").and_then(JsonValue::as_str) {
            Some("cmd") => {
                if let Some(seq) = row.get("seq").and_then(JsonValue::as_u64) {
                    last_command = Some(seq);
                    commands.insert(seq, (line, row));
                }
            }
            Some("assert") if row.get("status").and_then(JsonValue::as_str) == Some("fail") => {
                failures.push(line);
                implicated.extend(
                    row.pointer("/repro/cmd_seq")
                        .and_then(JsonValue::as_u64)
                        .filter(|seq| commands.contains_key(seq))
                        .or(last_command),
                );
            }
            Some("summary") => summary_exit = row.get("exit").and_then(JsonValue::as_i64),
            _ => {}
        }
    }
    if implicated.is_empty() && summary_exit != Some(0) {
        implicated.extend(last_command);
    }
    let commands = implicated
        .into_iter()
        .filter_map(|seq| commands.remove(&seq))
        .collect();
    (failures, commands)
}

fn print_capture_tail(log: &Path, capture: &Path, stream: &str) {
    const MAX_TAIL: u64 = 4096;
    let read_tail = || -> io::Result<Vec<u8>> {
        let artifacts = log.with_extension("artifacts").canonicalize()?;
        let capture = capture.canonicalize()?;
        if capture.parent() != Some(artifacts.as_path()) || !capture.is_file() {
            return Err(io::Error::other("capture is outside this log's artifacts"));
        }
        let mut file = File::open(capture)?;
        file.seek(SeekFrom::Start(
            file.metadata()?.len().saturating_sub(MAX_TAIL),
        ))?;
        let mut tail = Vec::new();
        file.take(MAX_TAIL).read_to_end(&mut tail)?;
        Ok(tail)
    };
    match read_tail() {
        Ok(tail) if !tail.is_empty() => {
            let text = String::from_utf8_lossy(&tail);
            let mut display = String::new();
            for ch in text.chars() {
                if ch == '\n' || ch == '\t' || !ch.is_control() {
                    display.push(ch);
                } else {
                    display.extend(ch.escape_debug());
                }
            }
            eprintln!("{stream} tail:\n{display}");
        }
        Ok(_) => {}
        Err(err) => eprintln!("cannot read {stream} capture: {err}"),
    }
}

#[cfg(test)]
mod tests {
    use super::{failure_rows, run_script};

    #[test]
    fn failed_assertions_report_their_command_once() {
        let log = concat!(
            "{\"event\":\"cmd\",\"seq\":0,\"exit\":101,\"step\":\"plant\"}\n",
            "{\"event\":\"assert\",\"seq\":1,\"status\":\"pass\",\"repro\":{\"cmd_seq\":0}}\n",
            "{\"event\":\"cmd\",\"seq\":2,\"exit\":0,\"step\":\"check\"}\n",
            "{\"event\":\"assert\",\"seq\":3,\"status\":\"fail\",\"repro\":{\"cmd_seq\":2}}\n",
            "{\"event\":\"assert\",\"seq\":4,\"status\":\"fail\",\"repro\":{\"cmd_seq\":2}}\n",
            "{\"event\":\"cmd\",\"seq\":5,\"exit\":0,\"step\":\"cleanup\"}\n",
        );
        let (failures, commands) = failure_rows(log);
        assert_eq!(failures, log.lines().skip(3).take(2).collect::<Vec<_>>());
        assert_eq!(commands.len(), 1);
        assert_eq!(commands[0].0, log.lines().nth(2).unwrap());
    }

    #[test]
    fn refuses_escape_and_preserves_script_exit() {
        let err = run_script("../outside.sh", &[], &[]).unwrap_err();
        assert!(err.contains("outside workspace"), "{err}");
        let run = run_script(
            "crates/xtask/tests/fixtures/e2e_runner_exit_3.sh",
            &["fixture argument"],
            &[("FROB_RUNNER_FIXTURE", "fixture environment")],
        )
        .expect("launch runner fixture");
        assert_eq!(run.status.code(), Some(3));
        assert_eq!(run.stdout, b"fixture argument\n");
        assert_eq!(run.stderr, b"fixture environment\n");
    }
}
