//! Runs the end-to-end harness self-test, `scripts/e2e/selftest.sh`, against the `frob-e2e`
//! binary Cargo built for this package (STD-E2E).
//!
//! Run through Cargo, the self-test is bound to the exact sources Cargo compiled, on any machine
//! or remote worker, instead of to whatever `frob-e2e` the scripts find on disk or build on their
//! own. It takes minutes and writes its logs and scratch under this test's own directory in
//! `CARGO_TARGET_TMPDIR`, so it is opt-in:
//!
//! ```text
//! cargo test -p frob-conformance --test e2e_selftest -- --ignored
//! ```
//!
//! Green requires all three: the script exits 0, its `end` event reports at least one passed
//! assertion, and it reports zero failed assertions. Exit 0 alone could hide a run that asserted
//! nothing.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn tail(text: &str, lines: usize) -> String {
    let all: Vec<&str> = text.lines().collect();
    all[all.len().saturating_sub(lines)..].join("\n")
}

/// The unsigned integer that follows `"key":` in one NDJSON line, if present.
fn field_u64(line: &str, key: &str) -> Option<u64> {
    let needle = format!("\"{key}\":");
    let start = line.find(&needle)? + needle.len();
    let digits: String = line[start..]
        .trim_start()
        .chars()
        .take_while(char::is_ascii_digit)
        .collect();
    digits.parse().ok()
}

/// `(asserts_passed, asserts_failed)` from the last line of `log` that carries both.
fn assertion_counts(log: &str) -> Option<(u64, u64)> {
    log.lines().rev().find_map(|line| {
        Some((
            field_u64(line, "asserts_passed")?,
            field_u64(line, "asserts_failed")?,
        ))
    })
}

#[test]
fn field_u64_reads_counts_and_rejects_absent_keys() {
    let line = r#"{"event":"end","asserts_passed": 412,"asserts_failed":0,"wall_ms":9}"#;
    assert_eq!(field_u64(line, "asserts_passed"), Some(412));
    assert_eq!(field_u64(line, "asserts_failed"), Some(0));
    assert_eq!(field_u64(line, "asserts_skipped"), None);
    assert_eq!(
        assertion_counts(&format!("{{\"event\":\"step\"}}\n{line}\n")),
        Some((412, 0))
    );
    assert_eq!(assertion_counts("{\"event\":\"step\"}\n"), None);
}

#[test]
#[ignore = "full e2e harness self-test, minutes long; run with `-- --ignored`"]
fn selftest_passes_with_the_cargo_built_tool() {
    let root = repo_root();
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock after 1970")
        .as_nanos();
    let work = Path::new(env!("CARGO_TARGET_TMPDIR"))
        .join(format!("e2e_selftest-{}-{stamp}", std::process::id()));
    let logs = work.join("logs");
    let scratch = work.join("scratch");
    fs::create_dir_all(&logs).expect("create logs dir");
    fs::create_dir_all(&scratch).expect("create scratch dir");

    let out = Command::new("bash")
        .arg(root.join("scripts/e2e/selftest.sh"))
        .current_dir(&root)
        .env("FROB_E2E_TOOL", env!("CARGO_BIN_EXE_frob-e2e"))
        .env("E2E_LOGS_ROOT", &logs)
        .env("E2E_SCRATCH_ROOT", &scratch)
        .output()
        .expect("run scripts/e2e/selftest.sh");
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    let context = format!(
        "exit {:?}\n--- stdout (tail)\n{}\n--- stderr (tail)\n{}",
        out.status.code(),
        tail(&stdout, 40),
        tail(&stderr, 40)
    );
    assert!(out.status.success(), "selftest.sh failed: {context}");

    let dir = logs.join("selftest");
    let latest = fs::read_to_string(dir.join("LATEST")).unwrap_or_else(|e| {
        panic!(
            "selftest wrote no LATEST under {}: {e}\n{context}",
            dir.display()
        )
    });
    let log_path = dir.join(format!("{}.ndjson", latest.trim()));
    let log = fs::read_to_string(&log_path)
        .unwrap_or_else(|e| panic!("read {}: {e}\n{context}", log_path.display()));
    let (passed, failed) = assertion_counts(&log).unwrap_or_else(|| {
        panic!(
            "{} has no line with asserts_passed and asserts_failed\n{context}",
            log_path.display()
        )
    });
    assert_eq!(
        failed, 0,
        "selftest reported {failed} failed assertions: {context}"
    );
    assert!(
        passed > 0,
        "selftest exited 0 but asserted nothing: {context}"
    );
}
