#![deny(unsafe_code)]

//! Mutation-feature guard test suite (`N38b-04`).
//!
//! Run as `cargo test -p frob-conformance --test mutation_guard`.
//! Verifies:
//! 1. Every `mutation` control's feature in `controls.toml` implies `conformance`,
//!    is never a default feature, and its declaring crate carries the release-profile
//!    `compile_error!` guard.
//! 2. Product binaries (`frob`, `frobd`, `frob-mcp`, `frob-verify`) carry neither
//!    `conformance` nor any mutation feature.
//! 3. Enabling `conformance` / `cx-262` under a release-equivalent build (`-O`,
//!    no `debug_assertions`) fails to compile with `compile_error!`.

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use frob_conformance::control_runner::{check_mutation_guard, verify_release_build_fails};
use frob_conformance::reasons::workspace_root;

static GUARD_SEQ: AtomicU64 = AtomicU64::new(0);

fn scratch_dir(label: &str) -> PathBuf {
    let base = env::var_os("BIG_SCRATCH")
        .or_else(|| env::var_os("TMPDIR"))
        .map(PathBuf::from)
        .unwrap_or_else(env::temp_dir);
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let seq = GUARD_SEQ.fetch_add(1, Ordering::Relaxed);
    let dir = base
        .join("frob-WindyDog")
        .join(format!("mutation_guard-{label}-{nanos}-{seq}"));
    fs::create_dir_all(&dir).expect("create scratch dir");
    dir
}

#[test]
fn workspace_mutation_guard_clean() {
    let root = workspace_root();
    let strict = env::var_os("FROB_CONFORMANCE_ROOT").is_some()
        || env::var_os("FROB_REGISTRY_ROOT").is_some();
    let report = check_mutation_guard(&root, strict).expect("run workspace mutation_guard");
    assert!(
        report.is_ok(),
        "mutation_guard failed on {}:\n{}",
        root.display(),
        report
            .violations
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join("\n")
    );
    assert!(report.crates_checked > 0);
}

#[test]
fn conformance_crate_release_compile_fail() {
    let root = workspace_root();
    let out_dir = scratch_dir("rustc_release_probe");
    let lib_rs = root.join("crates/frob-conformance/src/lib.rs");
    let rustc = env::var_os("RUSTC").unwrap_or_else(|| "rustc".into());

    let output = Command::new(rustc)
        .args([
            "--edition",
            "2024",
            "--crate-type",
            "lib",
            "--crate-name",
            "frob_conformance_release_guard_probe",
            "--cfg",
            "feature=\"conformance\"",
            "--cfg",
            "feature=\"cx-262\"",
            "-O",
            "--out-dir",
        ])
        .arg(&out_dir)
        .arg(&lib_rs)
        .output()
        .expect("invoke rustc for release compile-fail probe");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !output.status.success(),
        "compiling frob-conformance under -O with feature=conformance MUST fail"
    );
    assert!(
        stderr.contains("forbidden in release builds"),
        "expected release compile_error! diagnostic, stderr:\n{stderr}"
    );
}

#[test]
fn product_binary_with_conformance_rejected() {
    let ws = scratch_dir("prod_bin_guard");
    let bin_dir = ws.join("crates/frob");
    fs::create_dir_all(bin_dir.join("src")).expect("mkdir frob src");
    fs::create_dir_all(ws.join("crates/frob-conformance/registry")).expect("mkdir reg");
    fs::write(
        ws.join("crates/frob-conformance/registry/controls.toml"),
        "schema = \"frob.gauntlet.controls/1\"\n",
    )
    .expect("write controls");
    fs::write(
        bin_dir.join("Cargo.toml"),
        r#"[package]
name = "frob"
version = "0.1.0"
edition = "2024"

[[bin]]
name = "frob"
path = "src/main.rs"

[dependencies]
frob-lie = { path = "../frob-lie", features = ["conformance"] }
"#,
    )
    .expect("write frob Cargo.toml");
    fs::write(bin_dir.join("src/main.rs"), "fn main() {}\n").expect("write main.rs");

    let rep = check_mutation_guard(&ws, true).expect("check product binary");
    assert!(
        !rep.is_ok(),
        "product binary enabling conformance must fail"
    );
    assert!(
        rep.violations
            .iter()
            .any(|v| v.crate_name == "frob" && v.feature.contains("conformance")),
        "expected violation naming product binary and conformance feature: {:?}",
        rep.violations
    );
}

#[test]
fn scratch_crate_release_feature_fails_to_compile() {
    let ws = scratch_dir("scratch_release_fail");
    fs::create_dir_all(ws.join("src")).expect("mkdir src");
    fs::write(
        ws.join("Cargo.toml"),
        r#"[workspace]

[package]
name = "frob-release-probe"
version = "0.1.0"
edition = "2024"

[features]
default = []
conformance = []
cx-262 = ["conformance"]
"#,
    )
    .expect("write Cargo.toml");
    fs::write(
        ws.join("src/lib.rs"),
        r#"#[cfg(all(feature = "conformance", not(debug_assertions)))]
compile_error!("mutation feature forbidden in release builds");
pub fn ok() -> i32 { 0 }
"#,
    )
    .expect("write lib.rs");

    let target_dir = scratch_dir("scratch_release_target");
    let stderr =
        verify_release_build_fails(&ws.join("Cargo.toml"), None, "cx-262", &target_dir, true)
            .expect("--release --features cx-262 MUST fail to compile");
    assert!(
        stderr.contains("mutation feature forbidden in release builds"),
        "stderr:\n{stderr}"
    );
}
