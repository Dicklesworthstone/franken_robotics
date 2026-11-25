//! Integration, build-script, and compile-fail tests for `frob-core::stack` (`N33a-06`).
//!
//! Each scratch workspace is self-contained under the system temp directory (no external
//! network access, no mutations to the real repository, and no file deletions).

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const REMEDY: &str = "FROB_STACK_DEV=1 cargo xtask check-stack --write-receipt";
const ZERO_HEX: &str = "0000000000000000000000000000000000000000000000000000000000000000";
const HASH_A: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const HASH_B: &str = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";
const LOCK_A: &str = "1111111111111111111111111111111111111111111111111111111111111111";
const LOCK_B: &str = "2222222222222222222222222222222222222222222222222222222222222222";

const PROBE_PRINT_BUILT_UNDER: &str = r#"fn main() {
    let id = frob_core::stack::BUILT_UNDER;
    println!("receipt={}", id.receipt);
    println!("lock={}", id.lock);
    println!("dev={}", id.dev);
}
"#;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crates/frob-core sits two levels below workspace root")
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
    let dir = base.join(format!("frob-stack-it-{label}-{pid}-{nanos}-{n}"));
    fs::create_dir_all(&dir).expect("create scratch dir");
    fs::canonicalize(&dir).unwrap_or(dir)
}

fn copy_dir_recursive(src: &Path, dst: &Path) {
    fs::create_dir_all(dst).expect("create scratch destination dir");
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

fn write_file(path: &Path, content: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("create parent dirs");
    }
    fs::write(path, content).expect("write fixture file");
}

/// Self-contained offline Cargo workspace containing a fresh copy of `frob-core`, its
/// two zero-external-dep sibling crates (`fs-blake3`, `fs-qty`), `registries/refusals.toml`,
/// `franken-stack.lock`, and a tiny `probe` binary crate.
struct ScratchWorkspace {
    root: PathBuf,
    target_dir: PathBuf,
}

impl ScratchWorkspace {
    fn new(label: &str, probe_main_rs: &str) -> Self {
        let real_root = repo_root();
        let real_core = real_root.join("crates/frob-core");
        let real_frankensim = real_root
            .parent()
            .expect("workspace parent containing frankensim")
            .join("frankensim");

        let root = make_scratch_dir(label);
        let target_dir = root.join("target");

        // Workspace manifest and license.
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

        // Copy registries/refusals.toml (embedded by frob-core) and franken-stack.lock.
        let refusals = fs::read_to_string(real_root.join("registries/refusals.toml"))
            .expect("read registries/refusals.toml");
        write_file(&root.join("registries/refusals.toml"), &refusals);

        let stack_lock = fs::read_to_string(real_root.join("franken-stack.lock"))
            .expect("read franken-stack.lock");
        write_file(&root.join("franken-stack.lock"), &stack_lock);

        // Vendor pure-std fs-blake3 and fs-qty sources with standalone manifests (no dev-deps).
        copy_dir_recursive(
            &real_frankensim.join("crates/fs-blake3/src"),
            &root.join("vendor/fs-blake3/src"),
        );
        write_file(
            &root.join("vendor/fs-blake3/Cargo.toml"),
            "[package]\n\
             name = \"fs-blake3\"\n\
             version = \"0.0.1\"\n\
             edition = \"2024\"\n\
             publish = false\n\n\
             [dependencies]\n",
        );

        copy_dir_recursive(
            &real_frankensim.join("crates/fs-qty/src"),
            &root.join("vendor/fs-qty/src"),
        );
        write_file(
            &root.join("vendor/fs-qty/Cargo.toml"),
            "[package]\n\
             name = \"fs-qty\"\n\
             version = \"0.0.1\"\n\
             edition = \"2024\"\n\
             publish = false\n\n\
             [dependencies]\n\
             fs-blake3 = { path = \"../fs-blake3\" }\n",
        );

        // Copy frob-core src, build.rs, and manifest rewritten to vendored sibling paths.
        copy_dir_recursive(&real_core.join("src"), &root.join("crates/frob-core/src"));
        let build_rs = fs::read_to_string(real_core.join("build.rs")).expect("read build.rs");
        write_file(&root.join("crates/frob-core/build.rs"), &build_rs);

        let core_toml = fs::read_to_string(real_core.join("Cargo.toml"))
            .expect("read frob-core/Cargo.toml")
            .replace(
                "../../../frankensim/crates/fs-blake3",
                "../../vendor/fs-blake3",
            )
            .replace("../../../frankensim/crates/fs-qty", "../../vendor/fs-qty");
        write_file(&root.join("crates/frob-core/Cargo.toml"), &core_toml);
        // Satisfy the declared [[test]] path entry without copying the full test suite.
        write_file(&root.join("crates/frob-core/tests/testkit.rs"), "");

        // Probe binary crate depending on frob-core.
        write_file(
            &root.join("crates/probe/Cargo.toml"),
            "[package]\n\
             name = \"probe\"\n\
             version.workspace = true\n\
             edition.workspace = true\n\
             publish = false\n\n\
             [dependencies]\n\
             frob-core = { path = \"../frob-core\" }\n",
        );
        write_file(&root.join("crates/probe/src/main.rs"), probe_main_rs);

        Self { root, target_dir }
    }

    fn write_receipts(&self, head_hex: &str, lock_hex: &str) {
        write_file(&self.root.join("receipts/HEAD"), &format!("{head_hex}\n"));
        write_file(
            &self.root.join("receipts/LOCK_DIGEST"),
            &format!("{lock_hex}\n"),
        );
    }

    fn write_head_only(&self, head_hex: &str) {
        write_file(&self.root.join("receipts/HEAD"), &format!("{head_hex}\n"));
    }

    fn write_probe_main(&self, src: &str) {
        write_file(&self.root.join("crates/probe/src/main.rs"), src);
    }

    fn cargo(
        &self,
        args: &[&str],
        dev_env: Option<&str>,
        marker_path: Option<&Path>,
    ) -> Output {
        // Invoke `cargo` via PATH (never by absolute path).
        let mut cmd = Command::new("cargo");
        cmd.current_dir(&self.root)
            .args(args)
            .env("CARGO_TARGET_DIR", &self.target_dir);
        match dev_env {
            Some(v) => {
                cmd.env("FROB_STACK_DEV", v);
            }
            None => {
                cmd.env_remove("FROB_STACK_DEV");
            }
        }
        match marker_path {
            Some(p) => {
                cmd.env("FROB_STACK_BUILD_MARKER", p);
            }
            None => {
                cmd.env_remove("FROB_STACK_BUILD_MARKER");
            }
        }
        cmd.output().expect("spawn cargo")
    }

    fn run_probe(&self, dev_env: Option<&str>, marker_path: Option<&Path>) -> (String, String) {
        let out = self.cargo(
            &["run", "--quiet", "--offline", "-p", "probe"],
            dev_env,
            marker_path,
        );
        let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
        let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
        assert!(
            out.status.success(),
            "probe build/run failed (dev={dev_env:?}):\nstdout:\n{stdout}\nstderr:\n{stderr}"
        );
        (stdout, stderr)
    }
}

fn marker_run_count(path: &Path) -> usize {
    match fs::read_to_string(path) {
        Ok(s) => s.lines().count(),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => 0,
        Err(e) => {
            assert_eq!(e.kind(), std::io::ErrorKind::NotFound, "read marker: {e}");
            0
        }
    }
}

fn assert_probe_identity(stdout: &str, receipt_hex: &str, lock_hex: &str, dev: bool) {
    let expected = format!(
        "receipt=sr:{receipt_hex}\nlock=sl:{lock_hex}\ndev={dev}\n"
    );
    assert_eq!(stdout, expected, "unexpected BUILT_UNDER probe output");
}

#[test]
fn type_confusion_compile_fail() {
    let ws = ScratchWorkspace::new(
        "type-confusion",
        "fn takes_lock(_: frob_core::stack::StackLockDigest) {}\n\
         fn main() { takes_lock(frob_core::stack::BUILT_UNDER.receipt); }\n",
    );
    ws.write_receipts(HASH_A, LOCK_A);

    // Passing StackReceiptHash where StackLockDigest is expected must fail with E0308.
    let out_a = ws.cargo(&["check", "--offline", "-p", "probe"], None, None);
    let stderr_a = String::from_utf8_lossy(&out_a.stderr);
    assert!(
        !out_a.status.success(),
        "passing StackReceiptHash as StackLockDigest must fail to compile"
    );
    assert!(
        stderr_a.contains("E0308"),
        "expected E0308 type mismatch when passing StackReceiptHash as StackLockDigest:\n{stderr_a}"
    );
    assert!(
        stderr_a.contains("StackLockDigest") && stderr_a.contains("StackReceiptHash"),
        "diagnostic must name both distinct types:\n{stderr_a}"
    );

    // Passing StackLockDigest where StackReceiptHash is expected must also fail with E0308.
    ws.write_probe_main(
        "fn takes_receipt(_: frob_core::stack::StackReceiptHash) {}\n\
         fn main() { takes_receipt(frob_core::stack::BUILT_UNDER.lock); }\n",
    );
    let out_b = ws.cargo(&["check", "--offline", "-p", "probe"], None, None);
    let stderr_b = String::from_utf8_lossy(&out_b.stderr);
    assert!(
        !out_b.status.success(),
        "passing StackLockDigest as StackReceiptHash must fail to compile"
    );
    assert!(
        stderr_b.contains("E0308"),
        "expected E0308 type mismatch when passing StackLockDigest as StackReceiptHash:\n{stderr_b}"
    );
    assert!(
        stderr_b.contains("StackReceiptHash") && stderr_b.contains("StackLockDigest"),
        "diagnostic must name both distinct types:\n{stderr_b}"
    );

    // Also verify pinned compile_fail,E0308 doctests remain present in src/stack.rs.
    let stack_src = include_str!("../src/stack.rs");
    assert!(stack_src.contains("compile_fail,E0308"));
    assert!(stack_src.contains("let _wrong: StackLockDigest = BUILT_UNDER.receipt;"));
    assert!(stack_src.contains("let _wrong: StackReceiptHash = BUILT_UNDER.lock;"));
}

#[test]
fn build_script_fails_closed() {
    let ws = ScratchWorkspace::new("fail-closed", PROBE_PRINT_BUILT_UNDER);

    // 1. Both receipts/HEAD and receipts/LOCK_DIGEST absent, no FROB_STACK_DEV -> fail closed.
    let out_missing = ws.cargo(&["check", "--offline", "-p", "probe"], None, None);
    let stderr_missing = String::from_utf8_lossy(&out_missing.stderr);
    assert!(
        !out_missing.status.success(),
        "frob-core build must fail closed when receipts/HEAD is absent"
    );
    assert!(
        stderr_missing.contains(REMEDY),
        "fail-closed error must name `{REMEDY}`:\n{stderr_missing}"
    );
    assert!(
        stderr_missing.contains("receipts/HEAD"),
        "fail-closed error must name missing receipts/HEAD:\n{stderr_missing}"
    );

    // 2. With FROB_STACK_DEV=1 and receipts absent -> succeeds with dev: true and zero hashes.
    let (stdout_dev, _) = ws.run_probe(Some("1"), None);
    assert_probe_identity(&stdout_dev, ZERO_HEX, ZERO_HEX, true);

    // 3. With only receipts/HEAD present and FROB_STACK_DEV unset -> still fails closed on LOCK_DIGEST.
    ws.write_head_only(HASH_A);
    let out_no_lock = ws.cargo(&["check", "--offline", "-p", "probe"], None, None);
    let stderr_no_lock = String::from_utf8_lossy(&out_no_lock.stderr);
    assert!(
        !out_no_lock.status.success(),
        "frob-core build must fail closed when receipts/LOCK_DIGEST is absent"
    );
    assert!(
        stderr_no_lock.contains(REMEDY) && stderr_no_lock.contains("receipts/LOCK_DIGEST"),
        "fail-closed error must name `{REMEDY}` and receipts/LOCK_DIGEST:\n{stderr_no_lock}"
    );

    // 4. With both receipts/HEAD and receipts/LOCK_DIGEST written -> succeeds with dev: false.
    thread::sleep(Duration::from_millis(1100));
    ws.write_receipts(HASH_A, LOCK_A);
    let (stdout_rel, _) = ws.run_probe(None, None);
    assert_probe_identity(&stdout_rel, HASH_A, LOCK_A, false);
}

#[test]
fn dev_flag_toggle_rebuilds() {
    let ws = ScratchWorkspace::new("dev-toggle", PROBE_PRINT_BUILT_UNDER);
    ws.write_receipts(HASH_A, LOCK_A);
    let marker = ws.root.join("build_runs.log");

    // 1. Clean build without FROB_STACK_DEV -> dev: false, 1 build.rs execution.
    let (out1, _) = ws.run_probe(None, Some(&marker));
    assert_probe_identity(&out1, HASH_A, LOCK_A, false);
    assert_eq!(
        marker_run_count(&marker),
        1,
        "initial build must invoke build.rs once"
    );

    // 2. Toggle FROB_STACK_DEV=1 in the same target dir -> rebuilds, dev: true, 2 executions.
    let (out2, _) = ws.run_probe(Some("1"), Some(&marker));
    assert_probe_identity(&out2, HASH_A, LOCK_A, true);
    assert_eq!(
        marker_run_count(&marker),
        2,
        "setting FROB_STACK_DEV=1 must rerun build.rs"
    );

    // 3. Unset FROB_STACK_DEV in the same target dir -> rebuilds, dev: false, 3 executions.
    let (out3, _) = ws.run_probe(None, Some(&marker));
    assert_probe_identity(&out3, HASH_A, LOCK_A, false);
    assert_eq!(
        marker_run_count(&marker),
        3,
        "unsetting FROB_STACK_DEV must rerun build.rs"
    );

    // 4. Repeat without changes -> no rebuild, marker count remains 3.
    let (out4, _) = ws.run_probe(None, Some(&marker));
    assert_probe_identity(&out4, HASH_A, LOCK_A, false);
    assert_eq!(
        marker_run_count(&marker),
        3,
        "unchanged fourth build must not rerun build.rs"
    );
}

#[test]
fn identity_follows_write_receipt() {
    let ws = ScratchWorkspace::new("follows-receipt", PROBE_PRINT_BUILT_UNDER);
    ws.write_receipts(HASH_A, LOCK_A);

    let (out1, _) = ws.run_probe(None, None);
    assert_probe_identity(&out1, HASH_A, LOCK_A, false);

    // Advance filesystem mtime beyond 1-second granularity, then update receipts/HEAD
    // and receipts/LOCK_DIGEST with no `cargo clean`.
    thread::sleep(Duration::from_millis(1100));
    ws.write_receipts(HASH_B, LOCK_B);

    let (out2, _) = ws.run_probe(None, None);
    assert_probe_identity(&out2, HASH_B, LOCK_B, false);
}

#[test]
fn contract_md_documents_stack_identities_and_same_stack_rule() {
    let contract = include_str!("../CONTRACT.md");
    for needle in [
        "StackReceiptHash",
        "sr:",
        "StackLockDigest",
        "sl:",
        "BUILT_UNDER",
        "FROB_STACK_DEV=1",
        "same_stack",
        "require_same_stack",
        "FROB-REF-890",
    ] {
        assert!(
            contract.contains(needle),
            "crates/frob-core/CONTRACT.md must document `{needle}`"
        );
    }
}
