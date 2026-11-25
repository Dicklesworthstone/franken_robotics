//! Runs the whole end-to-end suite, `scripts/e2e/run-all.sh`, against the `frob-e2e` binary Cargo
//! built for this package (STD-E2E).
//!
//! The suite's scripts are the end-to-end evidence that many Beads name in their acceptance
//! criteria. Run by hand, `run-all.sh` uses whatever `frob-e2e` it finds or builds, on the host it
//! runs on. Run through Cargo, it is bound to the exact sources Cargo compiled, on any machine or
//! remote worker, so the verifier can cite one revision for every script's result. Each run takes
//! many minutes and writes logs, scratch and a private target directory (for the scripts' own
//! nested `cargo` calls, which must not wait on this test's build lock) under this test's
//! directory in `CARGO_TARGET_TMPDIR`, so both tests are opt-in:
//!
//! ```text
//! cargo test -p frob-conformance --test e2e_suite -- --ignored
//! ```
//!
//! Green requires: the runner exits 0, it prints one `PASS` line per selected script (the count
//! `run-all.sh --list` reports, which must be non-zero), and no `FAIL` line. Exit 0 alone could
//! hide a run that selected nothing or skipped a script.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn tail(text: &str, lines: usize) -> String {
    let all: Vec<&str> = text.lines().collect();
    all[all.len().saturating_sub(lines)..].join("\n")
}

/// `(pass_lines, fail_lines)` in the runner's stdout. Each selected script, or each plant under
/// `--verify-plants`, prints exactly one line that starts with `PASS ` or `FAIL `.
fn verdict_counts(stdout: &str) -> (usize, usize) {
    let pass = stdout.lines().filter(|l| l.starts_with("PASS ")).count();
    let fail = stdout.lines().filter(|l| l.starts_with("FAIL ")).count();
    (pass, fail)
}

/// Number of non-empty lines `run-all.sh --list` prints: one per selected script.
fn listed_scripts(listing: &str) -> usize {
    listing.lines().filter(|l| !l.trim().is_empty()).count()
}

#[test]
fn verdict_and_listing_counts() {
    let out = "PASS  envelope                         exit=0 lint=ok log=a\n\
               FAIL  toml_reader                      exit=1 lint=ok log=b\n\
               PASS  status_table                     exit=0 lint=ok log=c\n\
               run-all.sh: 2 passed, 1 failed\n";
    assert_eq!(verdict_counts(out), (2, 1));
    assert_eq!(verdict_counts("PASSED nothing\n FAIL indented\n"), (0, 0));
    assert_eq!(listed_scripts("a.sh\tN1\nb.sh\tN2\n\n"), 2);
    assert_eq!(listed_scripts(""), 0);
}

/// Runs `run-all.sh` with `args` in a fresh work directory; returns the output and a context
/// string (exit code and output tails) for assertion messages.
fn run_all(label: &str, args: &[&str]) -> (Output, String) {
    let root = repo_root();
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock after 1970")
        .as_nanos();
    let work = Path::new(env!("CARGO_TARGET_TMPDIR"))
        .join(format!("e2e_suite-{label}-{}-{stamp}", std::process::id()));
    let logs = work.join("logs");
    let scratch = work.join("scratch");
    let target = work.join("target");
    for dir in [&logs, &scratch, &target] {
        fs::create_dir_all(dir).expect("create work dir");
    }
    let out = Command::new("bash")
        .arg(root.join("scripts/e2e/run-all.sh"))
        .args(args)
        .arg("--logs-dir")
        .arg(&logs)
        .current_dir(&root)
        .env("FROB_E2E_TOOL", env!("CARGO_BIN_EXE_frob-e2e"))
        .env("E2E_LOGS_ROOT", &logs)
        .env("E2E_SCRATCH_ROOT", &scratch)
        .env("CARGO_TARGET_DIR", &target)
        .output()
        .expect("run scripts/e2e/run-all.sh");
    let context = format!(
        "exit {:?}\n--- stdout (tail)\n{}\n--- stderr (tail)\n{}",
        out.status.code(),
        tail(&String::from_utf8_lossy(&out.stdout), 60),
        tail(&String::from_utf8_lossy(&out.stderr), 40)
    );
    (out, context)
}

#[test]
#[ignore = "full e2e suite, many minutes; run with `-- --ignored`"]
fn suite_passes_with_the_cargo_built_tool() {
    let (listing, list_context) = run_all("list", &["--list"]);
    assert!(
        listing.status.success(),
        "run-all.sh --list failed: {list_context}"
    );
    let listed = listed_scripts(&String::from_utf8_lossy(&listing.stdout));
    assert!(
        listed > 0,
        "run-all.sh --list selected no script: {list_context}"
    );

    let (out, context) = run_all("run", &[]);
    let (passed, failed) = verdict_counts(&String::from_utf8_lossy(&out.stdout));
    assert_eq!(failed, 0, "{failed} e2e scripts failed: {context}");
    assert!(out.status.success(), "run-all.sh failed: {context}");
    assert_eq!(
        passed, listed,
        "{listed} scripts listed but {passed} passed: {context}"
    );
}

#[test]
#[ignore = "every declared plant of the e2e suite, many minutes; run with `-- --ignored`"]
fn suite_catches_every_declared_plant() {
    let (out, context) = run_all("plants", &["--verify-plants"]);
    let (caught, missed) = verdict_counts(&String::from_utf8_lossy(&out.stdout));
    assert_eq!(missed, 0, "{missed} plants were not caught: {context}");
    assert!(
        out.status.success(),
        "run-all.sh --verify-plants failed: {context}"
    );
    assert!(
        caught > 0,
        "run-all.sh --verify-plants checked no plant: {context}"
    );
}
