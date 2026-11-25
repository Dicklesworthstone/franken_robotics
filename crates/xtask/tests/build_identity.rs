//! Unit and integration tests for FrankenSim build identity (`N33a-08`, §33.2, §33.5, §33.8, §34.1).
//!
//! Run with `cargo test -p xtask build_identity`.
//!
//! Every integration test operates on a fresh, self-contained scratch directory
//! under `BIG_SCRATCH` / `TMPDIR` / `std::env::temp_dir()`, invokes `cargo`
//! strictly through `PATH`, treats sibling repositories as strictly read-only,
//! and deletes no files or directories.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use frob_core::stack::build_identity::{ENV_RECEIPT, ENV_SALT};
use xtask::report::FactValue;
use xtask::stack::build_identity::{
    EXPECTED_DEV_SALT, NoDefaultKind, lint_no_default_in_repo, lint_text, read_sibling_salt,
    salt_matches_lock as check_salt_matches_lock,
};
use xtask::stack::receipt::parse_build_env_with_receipt;
use xtask::stack::verify::{RULE_ENV, VerifyOptions, run_write_receipt, verify_repo};

const HASH_A: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const LOCK_A: &str = "1111111111111111111111111111111111111111111111111111111111111111";

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crates/xtask sits two levels below workspace root")
        .to_path_buf()
}

fn make_scratch_dir(label: &str) -> PathBuf {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let base = env::var_os("BIG_SCRATCH")
        .or_else(|| env::var_os("TMPDIR"))
        .map(PathBuf::from)
        .unwrap_or_else(env::temp_dir);
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_nanos());
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    let pid = std::process::id();
    let dir = base.join(format!("frob-bi-{label}-{pid}-{nanos}-{n}"));
    fs::create_dir_all(&dir).expect("create scratch dir");
    fs::canonicalize(&dir).unwrap_or(dir)
}

fn write_file(path: &Path, content: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("create parent dirs");
    }
    fs::write(path, content).expect("write fixture file");
}

fn copy_dir_recursive(src: &Path, dst: &Path) {
    fs::create_dir_all(dst).expect("create destination dir");
    for entry in fs::read_dir(src).expect("read source dir") {
        let entry = entry.expect("dir entry");
        let ty = entry.file_type().expect("file type");
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if ty.is_dir() {
            copy_dir_recursive(&src_path, &dst_path);
        } else if ty.is_file() {
            fs::copy(&src_path, &dst_path).expect("copy fixture file");
        }
    }
}

fn git_fixture(dir: &Path, args: &[&str]) -> String {
    let out = Command::new("git")
        .arg("-C")
        .arg(dir)
        .args(["-c", "commit.gpgsign=false", "-c", "core.autocrlf=false"])
        .args(args)
        .env("GIT_AUTHOR_NAME", "synthetic")
        .env("GIT_AUTHOR_EMAIL", "synthetic@example.com")
        .env("GIT_AUTHOR_DATE", "2026-11-24T00:00:00Z")
        .env("GIT_COMMITTER_NAME", "synthetic")
        .env("GIT_COMMITTER_EMAIL", "synthetic@example.com")
        .env("GIT_COMMITTER_DATE", "2026-11-24T00:00:00Z")
        .output()
        .expect("spawn git");
    assert!(
        out.status.success(),
        "git {:?} failed in {}:\n{}",
        args,
        dir.display(),
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8_lossy(&out.stdout).trim().to_owned()
}

fn commit_all(dir: &Path, msg: &str) -> String {
    git_fixture(dir, &["add", "."]);
    git_fixture(dir, &["commit", "-q", "-m", msg]);
    git_fixture(dir, &["rev-parse", "HEAD"])
}

fn has_kind(
    facts: &std::collections::BTreeMap<String, FactValue>,
    expected_rule: &str,
    expected_kind: &str,
    expected_fd: Option<&str>,
) -> bool {
    let Some(FactValue::Array(items)) = facts.get("kinds") else {
        return false;
    };
    items.iter().any(|item| {
        let FactValue::Object(map) = item else {
            return false;
        };
        let rule_ok = matches!(map.get("rule"), Some(FactValue::Str(s)) if s == expected_rule);
        let kind_ok = matches!(map.get("kind"), Some(FactValue::Str(s)) if s == expected_kind);
        let fd_ok = match expected_fd {
            Some(want) => matches!(map.get("fd"), Some(FactValue::Str(s)) if s == want),
            None => true,
        };
        let fails_ok = matches!(map.get("fails"), Some(FactValue::Bool(true)));
        rule_ok && kind_ok && fd_ok && fails_ok
    })
}

fn write_synthetic_stack_lock(repo: &Path, sib_rev: &str, build_env_toml: &str) {
    write_file(
        &repo.join("franken-stack.lock"),
        &format!(
            "toolchain = \"nightly-2026-08-31\"\n\n\
[[foundation]]\n\
id = \"FD-FSIM\"\n\
name = \"frankensim\"\n\
posture = \"consume-path\"\n\
path = \"../frankensim\"\n\
git_rev = \"{sib_rev}\"\n\
features = [\"referee\", \"geometry\", \"ledger\"]\n\
build_env = {build_env_toml}\n\
confined_to = [\"frob-conformance\"]\n\
admission_gate = \"INT-FSIM-01\"\n\
fallback = \"KILL-FSIM\"\n"
        ),
    );
}

fn build_synthetic_pair(label: &str, sibling_salt: &str, initial_salt: &str) -> (PathBuf, String) {
    let base = make_scratch_dir(label);
    let sib = base.join("frankensim");
    fs::create_dir_all(&sib).expect("mkdir sib");
    git_fixture(&sib, &["init", "-q", "-b", "main"]);
    write_file(
        &sib.join("Cargo.toml"),
        "[package]\nname = \"sib-fsim\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(&sib.join("src/lib.rs"), "pub const V: u32 = 1;\n");
    write_file(
        &sib.join(".cargo/config.toml"),
        &format!("[env]\n{ENV_SALT} = \"{sibling_salt}\"\n"),
    );
    let sib_rev = commit_all(&sib, "init sib");

    let repo = base.join("repo");
    fs::create_dir_all(&repo).expect("mkdir repo");
    git_fixture(&repo, &["init", "-q", "-b", "main"]);
    write_file(
        &repo.join("rust-toolchain.toml"),
        "[toolchain]\nchannel = \"nightly-2026-08-31\"\n",
    );
    write_file(
        &repo.join(".cargo/config.toml"),
        &format!("[env]\n{ENV_SALT} = \"{initial_salt}\"\n"),
    );
    write_file(
        &repo.join("Cargo.toml"),
        "[workspace]\nresolver = \"3\"\nmembers = [\"crates/frob-conformance\"]\n\n\
[patch.crates-io]\nsib-fsim = { path = \"../frankensim\" }\n",
    );
    write_file(
        &repo.join("crates/frob-conformance/Cargo.toml"),
        "[package]\nname = \"frob-conformance\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n\
[dependencies]\nsib-fsim = { path = \"../../../frankensim\" }\n",
    );
    write_file(
        &repo.join("crates/frob-conformance/src/lib.rs"),
        "pub fn v() -> u32 { sib_fsim::V }\n",
    );
    // The lock Cargo itself writes for this graph: the patch points at the same path the crate
    // depends on, so it is not unused and Cargo emits no `[[patch.unused]]` (one here made
    // `cargo metadata --locked` refuse "cannot update the lock file").
    write_file(
        &repo.join("Cargo.lock"),
        "# This file is automatically @generated by Cargo.\nversion = 4\n\n\
[[package]]\nname = \"frob-conformance\"\nversion = \"0.1.0\"\ndependencies = [\n \"sib-fsim\",\n]\n\n\
[[package]]\nname = \"sib-fsim\"\nversion = \"0.1.0\"\n",
    );
    write_file(
        &repo.join("registries/lifts.toml"),
        "schema = \"frob.lifts/1\"\n",
    );
    write_synthetic_stack_lock(
        &repo,
        &sib_rev,
        &format!("{{ {ENV_SALT} = \"{initial_salt}\" }}"),
    );
    let _ = commit_all(&repo, "init repo");

    let _ = run_write_receipt(&repo, &VerifyOptions::default()).expect("write initial receipt");
    (repo, sib_rev)
}

fn build_frob_core_scratch(
    label: &str,
    receipt_wins_when_both: Option<bool>,
) -> (PathBuf, PathBuf) {
    let real_root = repo_root();
    let real_core = real_root.join("crates/frob-core");
    let real_frankensim = real_root
        .parent()
        .expect("workspace parent containing frankensim")
        .join("frankensim");

    let root = make_scratch_dir(label);
    let target_dir = root.join("target");

    write_file(
        &root.join("Cargo.toml"),
        "[workspace]\n\
resolver = \"3\"\n\
members = [\"crates/frob-core\", \"crates/probe\"]\n\n\
[workspace.package]\n\
version = \"0.0.1\"\n\
edition = \"2024\"\n\
license-file = \"LICENSE\"\n\
repository = \"https://example.invalid/franken_robotics\"\n\
authors = [\"FrankenRobotics tests\"]\n\n\
[workspace.lints.rust]\n\
unsafe_code = \"deny\"\n",
    );
    write_file(&root.join("LICENSE"), "Test fixture license\n");
    write_file(
        &root.join(".cargo/config.toml"),
        &format!("[env]\n{ENV_SALT} = \"{EXPECTED_DEV_SALT}\"\n"),
    );

    let refusals = fs::read_to_string(real_root.join("registries/refusals.toml"))
        .expect("read registries/refusals.toml");
    write_file(&root.join("registries/refusals.toml"), &refusals);

    let stack_lock =
        fs::read_to_string(real_root.join("franken-stack.lock")).expect("read franken-stack.lock");
    write_file(&root.join("franken-stack.lock"), &stack_lock);
    write_file(&root.join("receipts/HEAD"), &format!("{HASH_A}\n"));
    write_file(&root.join("receipts/LOCK_DIGEST"), &format!("{LOCK_A}\n"));

    copy_dir_recursive(
        &real_frankensim.join("crates/fs-blake3/src"),
        &root.join("vendor/fs-blake3/src"),
    );
    write_file(
        &root.join("vendor/fs-blake3/Cargo.toml"),
        "[package]\nname = \"fs-blake3\"\nversion = \"0.0.1\"\nedition = \"2024\"\npublish = false\n\n[dependencies]\n",
    );

    copy_dir_recursive(
        &real_frankensim.join("crates/fs-qty/src"),
        &root.join("vendor/fs-qty/src"),
    );
    write_file(
        &root.join("vendor/fs-qty/Cargo.toml"),
        "[package]\nname = \"fs-qty\"\nversion = \"0.0.1\"\nedition = \"2024\"\npublish = false\n\n\
[dependencies]\nfs-blake3 = { path = \"../fs-blake3\" }\n",
    );

    copy_dir_recursive(&real_core.join("src"), &root.join("crates/frob-core/src"));
    let mut build_rs = fs::read_to_string(real_core.join("build.rs")).expect("read build.rs");
    if let Some(wins) = receipt_wins_when_both {
        let replacement = format!("const RECEIPT_WINS_WHEN_BOTH: Option<bool> = Some({wins});");
        assert!(
            build_rs.contains("const RECEIPT_WINS_WHEN_BOTH: Option<bool> = None;"),
            "frob-core/build.rs must declare `const RECEIPT_WINS_WHEN_BOTH: Option<bool> = None;`"
        );
        build_rs = build_rs.replace(
            "const RECEIPT_WINS_WHEN_BOTH: Option<bool> = None;",
            &replacement,
        );
        let mod_path = root.join("crates/frob-core/src/stack_build_identity.rs");
        let mod_src = fs::read_to_string(&mod_path).expect("read stack_build_identity.rs");
        write_file(
            &mod_path,
            &mod_src.replace(
                "pub const RECEIPT_WINS_WHEN_BOTH: Option<bool> = None;",
                &format!("pub const RECEIPT_WINS_WHEN_BOTH: Option<bool> = Some({wins});"),
            ),
        );
    }
    write_file(&root.join("crates/frob-core/build.rs"), &build_rs);

    let core_toml = fs::read_to_string(real_core.join("Cargo.toml"))
        .expect("read frob-core/Cargo.toml")
        .replace(
            "../../../frankensim/crates/fs-blake3",
            "../../vendor/fs-blake3",
        )
        .replace("../../../frankensim/crates/fs-qty", "../../vendor/fs-qty");
    write_file(&root.join("crates/frob-core/Cargo.toml"), &core_toml);
    write_file(&root.join("crates/frob-core/tests/testkit.rs"), "");

    write_file(
        &root.join("crates/probe/Cargo.toml"),
        "[package]\nname = \"probe\"\nversion.workspace = true\nedition.workspace = true\npublish = false\n\n\
[dependencies]\nfrob-core = { path = \"../frob-core\" }\n",
    );
    write_file(
        &root.join("crates/probe/src/main.rs"),
        "fn main() {\n    println!(\"{}\", frob_core::stack::BUILD_IDENTITY_MODE);\n}\n",
    );

    (root, target_dir)
}

fn run_cargo_clean_env(
    cwd: &Path,
    target_dir: &Path,
    args: &[&str],
    receipt_env: Option<&str>,
) -> Output {
    let mut cmd = Command::new("cargo");
    cmd.current_dir(cwd)
        .args(args)
        .env("CARGO_TARGET_DIR", target_dir)
        .env_remove(ENV_SALT)
        .env_remove("FROB_STACK_DEV");
    match receipt_env {
        Some(v) => {
            cmd.env(ENV_RECEIPT, v);
        }
        None => {
            cmd.env_remove(ENV_RECEIPT);
        }
    }
    cmd.output().expect("spawn cargo via PATH")
}

// ============================================================================
// Unit tests
// ============================================================================

#[test]
fn salt_matches_lock() {
    let root = repo_root();
    let salt = check_salt_matches_lock(&root).expect("config salt must equal FD-FSIM build_env");
    assert_eq!(salt, EXPECTED_DEV_SALT);

    // If the sibling frankensim checkout is present beside this repo, read its salt
    // strictly read-only and verify our equivalence class is distinct.
    if let Some(parent) = root.parent() {
        let sib = parent.join("frankensim");
        if sib.is_dir()
            && let Some(sib_salt) =
                read_sibling_salt(&sib).expect("read sibling .cargo/config.toml")
        {
            assert_ne!(
                salt, sib_salt,
                "FrankenRobotics must not reuse FrankenSim's own in-tree salt"
            );
        }
    }
}

#[test]
fn foreign_salt_rejected() {
    let foreign_salt = "synthetic-frankensim-in-tree-v1";
    let (repo, sib_rev) = build_synthetic_pair("foreign-salt", foreign_salt, foreign_salt);

    // 1. Config and lock both carry sibling's own salt -> fails `stack.env`, `foreign-build-class`.
    let out_foreign = verify_repo(&repo, &VerifyOptions::default()).expect("verify foreign salt");
    assert!(
        !out_foreign.findings.is_empty(),
        "matching sibling's own salt must fail check-stack"
    );
    assert!(
        has_kind(
            &out_foreign.facts,
            RULE_ENV,
            "foreign-build-class",
            Some("FD-FSIM"),
        ),
        "expected (`stack.env`, `foreign-build-class`, FD-FSIM), got facts: {:?}",
        out_foreign.facts.get("kinds")
    );

    // 2. Switch config and lock to project-owned `franken-robotics-dev-v1` + new receipt -> clean.
    write_file(
        &repo.join(".cargo/config.toml"),
        &format!("[env]\n{ENV_SALT} = \"{EXPECTED_DEV_SALT}\"\n"),
    );
    write_synthetic_stack_lock(
        &repo,
        &sib_rev,
        &format!("{{ {ENV_SALT} = \"{EXPECTED_DEV_SALT}\" }}"),
    );
    let _ = commit_all(&repo, "switch to project-owned salt");
    let _ = run_write_receipt(&repo, &VerifyOptions::default()).expect("write clean receipt");
    let out_clean = verify_repo(&repo, &VerifyOptions::default()).expect("verify clean salt");
    assert!(
        out_clean.findings.is_empty(),
        "project-owned salt must pass cleanly, got findings: {:?}",
        out_clean.findings
    );

    // 3. Config differs from lock -> fails `stack.env`, `build-env-missing`.
    write_file(
        &repo.join(".cargo/config.toml"),
        &format!("[env]\n{ENV_SALT} = \"drifted-salt-v9\"\n"),
    );
    let out_diff = verify_repo(&repo, &VerifyOptions::default()).expect("verify drifted salt");
    assert!(
        has_kind(
            &out_diff.facts,
            RULE_ENV,
            "build-env-missing",
            Some("FD-FSIM"),
        ),
        "config/lock mismatch must report (`stack.env`, `build-env-missing`), got: {:?}",
        out_diff.facts.get("kinds")
    );

    // Restore config so the only defect in case 4 is empty `build_env = {}` on FD-FSIM.
    write_file(
        &repo.join(".cargo/config.toml"),
        &format!("[env]\n{ENV_SALT} = \"{EXPECTED_DEV_SALT}\"\n"),
    );

    // 4. FD-FSIM `build_env = {}` -> fails `stack.env`, `build-env-missing`.
    write_synthetic_stack_lock(&repo, &sib_rev, "{}");
    let out_empty = verify_repo(&repo, &VerifyOptions::default()).expect("verify empty build_env");
    assert!(
        has_kind(
            &out_empty.facts,
            RULE_ENV,
            "build-env-missing",
            Some("FD-FSIM"),
        ),
        "empty FD-FSIM build_env must report (`stack.env`, `build-env-missing`, FD-FSIM), got: {:?}",
        out_empty.facts.get("kinds")
    );
}

#[test]
fn no_default_lint_clean() {
    let findings = lint_no_default_in_repo(&repo_root()).expect("run no-default lint on repo");
    assert!(
        findings.is_empty(),
        "no-default lint reported {} finding(s) on repository:\n{}",
        findings.len(),
        findings
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join("\n")
    );
}

#[test]
fn no_default_lint_catches_shell_default() {
    let scratch = make_scratch_dir("lint-shell");
    write_file(
        &scratch.join("scripts/planted.sh"),
        "#!/usr/bin/env bash\n\
set -euo pipefail\n\
# Safe comment mentioning ${FRANKENSIM_DEPGRAPH_SALT:-ignored}\n\
export FRANKENSIM_DEPGRAPH_SALT=\"${FRANKENSIM_DEPGRAPH_SALT:-franken-robotics-dev-v1}\"\n",
    );
    let findings = lint_no_default_in_repo(&scratch).expect("lint planted shell script");
    assert_eq!(findings.len(), 1, "expected 1 finding, got: {findings:?}");
    assert_eq!(findings[0].path, "scripts/planted.sh");
    assert_eq!(findings[0].line, 4);
    assert_eq!(findings[0].kind, NoDefaultKind::ShellDefault);
    assert_eq!(findings[0].variable, ENV_SALT);
}

#[test]
fn no_default_lint_catches_unwrap_or() {
    let scratch = make_scratch_dir("lint-unwrap-or");
    write_file(
        &scratch.join("crates/planted/build.rs"),
        "fn main() {\n\
    let _salt = std::env::var(\"FRANKENSIM_DEPGRAPH_SALT\").unwrap_or(\"fallback\".to_owned());\n\
}\n",
    );
    let findings = lint_no_default_in_repo(&scratch).expect("lint planted build.rs");
    assert_eq!(findings.len(), 1, "expected 1 finding, got: {findings:?}");
    assert_eq!(findings[0].path, "crates/planted/build.rs");
    assert_eq!(findings[0].line, 2);
    assert_eq!(findings[0].kind, NoDefaultKind::RustUnwrapOr);
    assert_eq!(findings[0].variable, ENV_SALT);
}

#[test]
fn no_default_lint_catches_option_env_fallback() {
    let scratch = make_scratch_dir("lint-option-env");
    write_file(
        &scratch.join("crates/planted/src/lib.rs"),
        "pub const MODE: &str = option_env!(\"FRANKENSIM_DEPGRAPH_RECEIPT\").unwrap_or(\"dev\");\n",
    );
    let findings = lint_no_default_in_repo(&scratch).expect("lint planted src/lib.rs");
    assert_eq!(findings.len(), 1, "expected 1 finding, got: {findings:?}");
    assert_eq!(findings[0].path, "crates/planted/src/lib.rs");
    assert_eq!(findings[0].line, 1);
    assert_eq!(findings[0].kind, NoDefaultKind::OptionEnvFallback);
    assert_eq!(findings[0].variable, ENV_RECEIPT);

    // Bare `if let Some(...) = option_env!(...)` without a fallback must remain clean.
    let clean = lint_text(
        "crates/clean/src/lib.rs",
        "pub fn probe() -> Option<&'static str> {\n\
            if let Some(v) = option_env!(\"FRANKENSIM_DEPGRAPH_SALT\") {\n\
                let _other = Some(1usize).unwrap_or(0);\n\
                return Some(v);\n\
            }\n\
            None\n\
         }\n",
    );
    assert!(
        clean.is_empty(),
        "bare option_env! must not false-positive: {clean:?}"
    );
}

// ============================================================================
// Integration tests (scratch workspaces, isolated target dirs, no deletions)
// ============================================================================

#[test]
fn mode_change_rebuilds() {
    // Part A: On unmodified source (`RECEIPT_WINS_WHEN_BOTH = None`), setting
    // `FRANKENSIM_DEPGRAPH_RECEIPT` while `.cargo/config.toml` sets salt MUST fail closed.
    let (unmod_root, unmod_target) = build_frob_core_scratch("mode-ambiguous", None);
    let ambiguous = run_cargo_clean_env(
        &unmod_root,
        &unmod_target,
        &["check", "--offline", "-j", "1", "-p", "probe"],
        Some("synthetic-receipt-v1"),
    );
    assert!(
        !ambiguous.status.success(),
        "both variables with RECEIPT_WINS_WHEN_BOTH = None must fail closed"
    );
    let ambig_stderr = String::from_utf8_lossy(&ambiguous.stderr);
    assert!(
        ambig_stderr.contains("RECEIPT_WINS_WHEN_BOTH") || ambig_stderr.contains("ambiguous"),
        "expected ambiguous build-identity message, got:\n{ambig_stderr}"
    );

    // Verify receipt build_env helper matches: Mode 2 fails closed under None, records
    // FRANKENSIM_DEPGRAPH_RECEIPT under Some(true) so receipt hashes differ.
    let env_mode1 =
        parse_build_env_with_receipt(&unmod_root, None, None).expect("mode 1 build_env");
    assert_eq!(
        env_mode1.get(ENV_SALT).map(String::as_str),
        Some(EXPECTED_DEV_SALT)
    );
    assert!(!env_mode1.contains_key(ENV_RECEIPT));
    assert!(
        parse_build_env_with_receipt(&unmod_root, Some("synthetic-receipt-v1"), None).is_err(),
        "mode 2 receipt build_env must refuse when RECEIPT_WINS_WHEN_BOTH is None"
    );
    let env_mode2 =
        parse_build_env_with_receipt(&unmod_root, Some("synthetic-receipt-v1"), Some(true))
            .expect("mode 2 build_env when receipt wins");
    assert_eq!(
        env_mode2.get(ENV_RECEIPT).map(String::as_str),
        Some("synthetic-receipt-v1")
    );
    assert_ne!(env_mode1, env_mode2);

    // Part B: With `RECEIPT_WINS_WHEN_BOTH = Some(true)` in the scratch copy and
    // ONE shared target directory (no files edited between builds), toggling
    // `FRANKENSIM_DEPGRAPH_RECEIPT` rebuilds `BUILD_IDENTITY_MODE`:
    // `dev-salt` -> `depgraph-receipt` -> `dev-salt`.
    let (root, target_dir) = build_frob_core_scratch("mode-rebuild", Some(true));

    let run1 = run_cargo_clean_env(
        &root,
        &target_dir,
        &["run", "--quiet", "--offline", "-j", "1", "-p", "probe"],
        None,
    );
    assert!(
        run1.status.success(),
        "run 1 (dev-salt) failed:\n{}",
        String::from_utf8_lossy(&run1.stderr)
    );
    assert_eq!(String::from_utf8_lossy(&run1.stdout).trim(), "dev-salt");

    let run2 = run_cargo_clean_env(
        &root,
        &target_dir,
        &["run", "--quiet", "--offline", "-j", "1", "-p", "probe"],
        Some("synthetic-receipt-v1"),
    );
    assert!(
        run2.status.success(),
        "run 2 (depgraph-receipt) failed:\n{}",
        String::from_utf8_lossy(&run2.stderr)
    );
    assert_eq!(
        String::from_utf8_lossy(&run2.stdout).trim(),
        "depgraph-receipt"
    );

    let run3 = run_cargo_clean_env(
        &root,
        &target_dir,
        &["run", "--quiet", "--offline", "-j", "1", "-p", "probe"],
        None,
    );
    assert!(
        run3.status.success(),
        "run 3 (back to dev-salt) failed:\n{}",
        String::from_utf8_lossy(&run3.stderr)
    );
    assert_eq!(String::from_utf8_lossy(&run3.stdout).trim(), "dev-salt");
}

#[test]
fn fail_closed_without_salt() {
    let real_root = repo_root();
    let parent = real_root
        .parent()
        .expect("workspace parent containing frankensim and asupersync");
    let fs_la_dir = fs::canonicalize(parent.join("frankensim/crates/fs-la"))
        .expect("pinned sibling ../frankensim/crates/fs-la must exist");
    let asup_dir =
        fs::canonicalize(parent.join("asupersync")).expect("sibling ../asupersync must exist");

    let scratch = make_scratch_dir("fail-closed-fs-la");
    let target_dir = scratch.join("target");

    write_file(
        &scratch.join("Cargo.toml"),
        &format!(
            "[workspace]\n\n\
[package]\n\
name = \"probe-fs-la\"\n\
version = \"0.0.1\"\n\
edition = \"2024\"\n\
publish = false\n\n\
[dependencies]\n\
fs-la = {{ path = \"{}\" }}\n\n\
[patch.crates-io]\n\
asupersync = {{ path = \"{}\" }}\n",
            fs_la_dir.display(),
            asup_dir.display(),
        ),
    );
    write_file(
        &scratch.join("src/lib.rs"),
        "extern crate fs_la;\npub fn linked() {}\n",
    );
    // Copy workspace Cargo.lock so `--offline` resolves transitive crates from Cargo's local cache.
    fs::copy(real_root.join("Cargo.lock"), scratch.join("Cargo.lock"))
        .expect("copy Cargo.lock into scratch probe");

    // 1. Without `.cargo/config.toml` and with neither variable exported -> fail closed
    //    with FrankenSim's `fs-la/build.rs` panic text.
    let fail_out = run_cargo_clean_env(
        &scratch,
        &target_dir,
        &["check", "--offline", "-j", "1"],
        None,
    );
    let combined = format!(
        "{}\n{}",
        String::from_utf8_lossy(&fail_out.stdout),
        String::from_utf8_lossy(&fail_out.stderr)
    );
    assert!(
        !fail_out.status.success(),
        "probe depending on fs-la must fail closed without salt or receipt:\n{combined}"
    );
    assert!(
        combined.contains("GEMM build identity requires dependency-graph evidence")
            && combined.contains(ENV_SALT)
            && combined.contains(ENV_RECEIPT),
        "expected fs-la/build.rs panic text in output, got:\n{combined}"
    );

    // 2. Restore `.cargo/config.toml` `[env]` (still no process env export) -> builds cleanly.
    write_file(
        &scratch.join(".cargo/config.toml"),
        &format!("[env]\n{ENV_SALT} = \"{EXPECTED_DEV_SALT}\"\n"),
    );
    let pass_out = run_cargo_clean_env(
        &scratch,
        &target_dir,
        &["check", "--offline", "-j", "1"],
        None,
    );
    assert!(
        pass_out.status.success(),
        "probe depending on fs-la must build once .cargo/config.toml supplies salt:\n{}",
        String::from_utf8_lossy(&pass_out.stderr)
    );
}

#[test]
fn nested_workspaces_see_salt() {
    let scratch = make_scratch_dir("nested-workspaces");
    let target_dir = scratch.join("target");

    write_file(
        &scratch.join(".cargo/config.toml"),
        &format!("[env]\n{ENV_SALT} = \"{EXPECTED_DEV_SALT}\"\n"),
    );

    let probe_build_rs = format!(
        "fn main() {{\n\
    println!(\"cargo:rerun-if-env-changed={ENV_SALT}\");\n\
    match std::env::var(\"{ENV_SALT}\") {{\n\
        Ok(v) if v == \"{EXPECTED_DEV_SALT}\" => {{}}\n\
        Ok(other) => {{\n\
            eprintln!(\"unexpected {ENV_SALT}={{other}}\");\n\
            std::process::exit(1);\n\
        }}\n\
        Err(_) => {{\n\
            eprintln!(\"{ENV_SALT} is not visible to nested workspace\");\n\
            std::process::exit(1);\n\
        }}\n\
    }}\n\
}}\n"
    );

    for (ws_rel, pkg_name) in [("apps", "probe-apps"), ("worker", "probe-worker")] {
        let ws_dir = scratch.join(ws_rel);
        write_file(
            &ws_dir.join("Cargo.toml"),
            &format!("[workspace]\nresolver = \"3\"\nmembers = [\"{pkg_name}\"]\n"),
        );
        write_file(
            &ws_dir.join(format!("{pkg_name}/Cargo.toml")),
            &format!(
                "[package]\nname = \"{pkg_name}\"\nversion = \"0.0.1\"\nedition = \"2024\"\npublish = false\n"
            ),
        );
        write_file(
            &ws_dir.join(format!("{pkg_name}/build.rs")),
            &probe_build_rs,
        );
        write_file(
            &ws_dir.join(format!("{pkg_name}/src/lib.rs")),
            "pub const OK: bool = true;\n",
        );
    }

    // 1. With root `.cargo/config.toml` `[env]` present and no shell export -> both succeed.
    for ws_rel in ["apps", "worker"] {
        let out = run_cargo_clean_env(
            &scratch.join(ws_rel),
            &target_dir,
            &["check", "--offline", "-j", "1"],
            None,
        );
        assert!(
            out.status.success(),
            "nested workspace `{ws_rel}` must see root `.cargo/config.toml` salt:\n{}",
            String::from_utf8_lossy(&out.stderr)
        );
    }

    // 2. Remove the `[env]` entry (by overwriting with an empty config, zero deletions)
    //    -> both nested workspaces fail closed.
    write_file(&scratch.join(".cargo/config.toml"), "# [env] removed\n");
    for ws_rel in ["apps", "worker"] {
        let out = run_cargo_clean_env(
            &scratch.join(ws_rel),
            &target_dir,
            &["check", "--offline", "-j", "1"],
            None,
        );
        assert!(
            !out.status.success(),
            "nested workspace `{ws_rel}` must fail once root `[env]` entry is removed"
        );
    }
}

#[test]
fn env_table_survives_env_u() {
    let scratch = make_scratch_dir("env-u-survives");
    let target_dir = scratch.join("target");

    write_file(
        &scratch.join(".cargo/config.toml"),
        &format!("[env]\n{ENV_SALT} = \"{EXPECTED_DEV_SALT}\"\n"),
    );
    write_file(
        &scratch.join("Cargo.toml"),
        "[workspace]\n\n\
[package]\n\
name = \"probe-env-u\"\n\
version = \"0.0.1\"\n\
edition = \"2024\"\n\
publish = false\n",
    );
    write_file(
        &scratch.join("build.rs"),
        &format!(
            "fn main() {{\n\
    println!(\"cargo:rerun-if-env-changed={ENV_SALT}\");\n\
    let seen = match std::env::var(\"{ENV_SALT}\") {{\n\
        Ok(v) => v,\n\
        Err(_) => \"<unset>\".to_owned(),\n\
    }};\n\
    println!(\"cargo:rustc-env=PROBED_SALT={{seen}}\");\n\
}}\n"
        ),
    );
    write_file(
        &scratch.join("src/main.rs"),
        "fn main() {\n    println!(\"{}\", env!(\"PROBED_SALT\"));\n}\n",
    );

    // Invoke `env -u FRANKENSIM_DEPGRAPH_SALT cargo run ...` via PATH: Cargo's `[env]`
    // table still supplies the variable when it is absent from the process environment.
    let out = Command::new("env")
        .current_dir(&scratch)
        .args([
            "-u",
            ENV_SALT,
            "-u",
            ENV_RECEIPT,
            "cargo",
            "run",
            "--quiet",
            "--offline",
            "-j",
            "1",
        ])
        .env("CARGO_TARGET_DIR", &target_dir)
        .output()
        .expect("spawn env -u ... cargo via PATH");
    assert!(
        out.status.success(),
        "env -u probe failed:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert_eq!(
        String::from_utf8_lossy(&out.stdout).trim(),
        EXPECTED_DEV_SALT,
        "Cargo [env] entry must remain visible under `env -u {ENV_SALT}`"
    );
}
