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
fn product_binary_default_enabling_conformance_rejected() {
    let ws = scratch_dir("prod_bin_default_conformance");
    let cli_dir = ws.join("crates/frob-cli");
    let helper_dir = ws.join("crates/frob-helper");
    fs::create_dir_all(cli_dir.join("src")).expect("mkdir frob-cli src");
    fs::create_dir_all(helper_dir.join("src")).expect("mkdir frob-helper src");
    fs::create_dir_all(ws.join("crates/frob-conformance/registry")).expect("mkdir reg");
    fs::write(
        ws.join("crates/frob-conformance/registry/controls.toml"),
        "schema = \"frob.gauntlet.controls/1\"\n",
    )
    .expect("write controls");

    // 1. Default feature enabling local `conformance`, `<dep>/conformance` via alias, and active
    //    weak `<dep>?/conformance` on distinct deps -> all rejected; inactive weak dep -> ignored.
    fs::write(
        cli_dir.join("Cargo.toml"),
        r#"[package]
name = "frob-cli"
version.workspace = true
edition.workspace = true

[[bin]]
name = "frob"
path = "src/main.rs"

[dependencies]
frob-verbs = { path = "../frob-verbs", default-features = false, features = ["handlers"] }
frob-lie = { path = "../frob-lie", default-features = false }
frob-opt = { path = "../frob-opt", optional = true }

[features]
default = ["conformance", "probe-mode", "weak-active", "weak-inactive"]
conformance = []
probe-mode = ["frob-verbs/conformance"]
weak-active = ["frob-lie?/conformance"]
weak-inactive = ["frob-opt?/conformance"]
"#,
    )
    .expect("write frob-cli Cargo.toml");
    fs::write(cli_dir.join("src/main.rs"), "fn main() {}\n").expect("write main.rs");

    let rep = check_mutation_guard(&ws, true).expect("check default enabling conformance");
    assert!(
        !rep.is_ok(),
        "product binary default enabling conformance must fail"
    );
    for expected in [
        "conformance",
        "frob-verbs/conformance",
        "frob-lie/conformance",
    ] {
        assert!(
            rep.violations.iter().any(|v| {
                v.crate_name == "frob-cli"
                    && v.rule == "product-binary-conformance"
                    && v.feature == expected
            }),
            "expected `{expected}` violation, got: {:?}",
            rep.violations
        );
    }
    assert!(
        !rep.violations
            .iter()
            .any(|v| v.feature.starts_with("frob-opt/")),
        "inactive optional weak dep must not trigger violation, got: {:?}",
        rep.violations
    );

    // 2. Transitive default through an intermediate workspace crate (`default-features = true`) -> rejected.
    fs::write(
        helper_dir.join("Cargo.toml"),
        r#"[package]
name = "frob-helper"
version = "0.0.1"
edition = "2024"

[dependencies]
frob-verbs = { path = "../frob-verbs", default-features = false }

[features]
default = ["leak-conf"]
leak-conf = ["frob-verbs/conformance"]
"#,
    )
    .expect("write frob-helper Cargo.toml");
    fs::write(helper_dir.join("src/lib.rs"), "pub fn ok() {}\n").expect("write lib.rs");
    fs::write(
        cli_dir.join("Cargo.toml"),
        r#"[package]
name = "frob-cli"
version.workspace = true
edition.workspace = true

[[bin]]
name = "frob"
path = "src/main.rs"

[dependencies]
frob-helper = { path = "../frob-helper" }
frob-verbs = { path = "../frob-verbs", default-features = false, features = ["handlers"] }

[features]
default = ["referee"]
referee = ["frob-verbs/referee"]
"#,
    )
    .expect("write transitive frob-cli Cargo.toml");

    let rep_trans = check_mutation_guard(&ws, true).expect("check transitive default conformance");
    assert!(
        !rep_trans.is_ok(),
        "product binary pulling conformance transitively through a dependency's default must fail"
    );
    assert!(
        rep_trans.violations.iter().any(|v| {
            v.crate_name == "frob-cli"
                && v.rule == "product-binary-conformance"
                && v.feature == "frob-verbs/conformance"
        }),
        "expected transitive conformance violation, got: {:?}",
        rep_trans.violations
    );

    // 3. Disabling default-features on frob-helper (and keeping non-default alias unreachable) -> clean.
    fs::write(
        cli_dir.join("Cargo.toml"),
        r#"[package]
name = "frob-cli"
version.workspace = true
edition.workspace = true

[[bin]]
name = "frob"
path = "src/main.rs"

[dependencies]
frob-helper = { path = "../frob-helper", default-features = false }
frob-verbs = { path = "../frob-verbs", default-features = false, features = ["handlers"] }

[features]
default = ["referee"]
referee = ["frob-verbs/referee"]
extra-probe = ["frob-verbs/conformance"]
"#,
    )
    .expect("write clean non-default frob-cli Cargo.toml");

    let rep_clean = check_mutation_guard(&ws, true).expect("check clean non-default conformance");
    assert!(
        rep_clean.is_ok(),
        "unreachable non-default alias with default-features=false on helper must pass, got: {:?}",
        rep_clean.violations
    );
}

#[test]
fn product_binary_default_enabling_control_mutation_rejected() {
    let ws = scratch_dir("prod_bin_default_control");
    let cli_dir = ws.join("crates/frob-cli");
    let lie_dir = ws.join("crates/frob-lie");
    let cert_dir = ws.join("crates/frob-cert");
    fs::create_dir_all(cli_dir.join("src")).expect("mkdir frob-cli src");
    fs::create_dir_all(lie_dir.join("src")).expect("mkdir frob-lie src");
    fs::create_dir_all(cert_dir.join("src/bin")).expect("mkdir frob-cert src/bin");
    fs::create_dir_all(ws.join("crates/frob-conformance/registry")).expect("mkdir reg");
    fs::write(
        ws.join("crates/frob-conformance/registry/controls.toml"),
        r#"schema = "frob.gauntlet.controls/1"

[[control]]
id = "NC-0412"
seed = "CX-262"
gate = ["GATE-ARENA-04"]
kind = "mutation"
target = "frob-actuate::stage::integrate"
defect = "explicit Euler"
expect = "spectral radius 1.497"
feature = "cx-262"
due = "M1"
"#,
    )
    .expect("write controls");

    // frob-lie sorts alphabetically AFTER frob-cli (tests pass-2/pass-3 ordering).
    fs::write(
        lie_dir.join("Cargo.toml"),
        r#"[package]
name = "frob-lie"
version = "0.0.1"
edition = "2024"

[features]
default = []
conformance = []
twist-swap = ["conformance"]
"#,
    )
    .expect("write frob-lie Cargo.toml");
    fs::write(
        lie_dir.join("src/lib.rs"),
        r#"#[cfg(all(feature = "conformance", not(debug_assertions)))]
compile_error!("frob-lie: conformance forbidden in release builds");
"#,
    )
    .expect("write frob-lie lib.rs");

    // Non-default declared nc-* / gauntlet-mutations on frob-cert (not in default, not in controls.toml)
    // must NOT be rejected by prefix or declared name.
    fs::write(
        cert_dir.join("Cargo.toml"),
        r#"[package]
name = "frob-cert"
version = "0.0.1"
edition = "2024"

[[bin]]
name = "frob-verify"
path = "src/bin/frob-verify.rs"

[features]
gauntlet-mutations = []
nc-skip-key-order = []
"#,
    )
    .expect("write frob-cert Cargo.toml");
    fs::write(cert_dir.join("src/bin/frob-verify.rs"), "fn main() {}\n")
        .expect("write frob-verify.rs");

    // frob-cli's default enables BOTH a control-named feature (`cx-262`) AND a workspace
    // conformance-implying mutation (`frob-lie/twist-swap`).
    fs::write(
        cli_dir.join("Cargo.toml"),
        r#"[package]
name = "frob-cli"
version.workspace = true
edition.workspace = true

[[bin]]
name = "frob"
path = "src/main.rs"

[dependencies]
frob-lie = { path = "../frob-lie" }

[features]
default = ["twist-bridge", "cx-262"]
twist-bridge = ["frob-lie/twist-swap"]
cx-262 = []
"#,
    )
    .expect("write frob-cli Cargo.toml");
    fs::write(cli_dir.join("src/main.rs"), "fn main() {}\n").expect("write frob-cli main.rs");

    let rep = check_mutation_guard(&ws, true).expect("check default control mutation");
    assert!(
        !rep.is_ok(),
        "product binary default enabling control or workspace mutation feature must fail"
    );
    assert!(
        rep.violations.iter().any(|v| {
            v.crate_name == "frob-cli"
                && v.rule == "product-binary-mutation-feature"
                && v.feature == "cx-262"
        }),
        "expected control feature `cx-262` violation, got: {:?}",
        rep.violations
    );
    assert!(
        rep.violations.iter().any(|v| {
            v.crate_name == "frob-cli"
                && v.rule == "product-binary-mutation-feature"
                && v.feature == "frob-lie/twist-swap"
        }),
        "expected forwarded workspace mutation `frob-lie/twist-swap` violation, got: {:?}",
        rep.violations
    );
    assert!(
        rep.violations.iter().any(|v| {
            v.crate_name == "frob-cli"
                && v.rule == "product-binary-conformance"
                && v.feature == "frob-lie/conformance"
        }),
        "expected implied `frob-lie/conformance` violation, got: {:?}",
        rep.violations
    );
    assert!(
        !rep.violations.iter().any(|v| v.crate_name == "frob-cert"),
        "non-default declared features on frob-cert must not be rejected by prefix: {:?}",
        rep.violations
    );
}

#[test]
fn product_binary_clean_with_normal_features_accepted() {
    let ws = scratch_dir("prod_bin_clean_normal");
    let cli_dir = ws.join("crates/frob-cli");
    let verbs_dir = ws.join("crates/frob-verbs");
    fs::create_dir_all(cli_dir.join("src")).expect("mkdir frob-cli src");
    fs::create_dir_all(verbs_dir.join("src")).expect("mkdir frob-verbs src");
    fs::create_dir_all(ws.join("crates/frob-conformance/registry")).expect("mkdir reg");
    fs::write(
        ws.join("crates/frob-conformance/registry/controls.toml"),
        "schema = \"frob.gauntlet.controls/1\"\n",
    )
    .expect("write controls");
    fs::write(
        verbs_dir.join("Cargo.toml"),
        r#"[package]
name = "frob-verbs"
version = "0.0.1"
edition = "2024"

[features]
default = ["referee"]
handlers = []
referee = ["handlers"]
conformance = []
"#,
    )
    .expect("write frob-verbs Cargo.toml");
    fs::write(
        verbs_dir.join("src/lib.rs"),
        r#"#[cfg(all(feature = "conformance", not(debug_assertions)))]
compile_error!("frob-verbs: conformance forbidden in release builds");
"#,
    )
    .expect("write frob-verbs lib.rs");
    fs::write(
        cli_dir.join("Cargo.toml"),
        r#"[package]
name = "frob-cli"
version.workspace = true
edition.workspace = true

[package.metadata.frob]
layer = 7
rank = 2

[[bin]]
name = "frob"
path = "src/main.rs"

[dependencies]
frob-verbs = { path = "../frob-verbs", default-features = false, features = ["handlers"] }

[features]
default = ["referee"]
referee = ["frob-verbs/referee"]
conformance = ["frob-verbs/conformance"]
"#,
    )
    .expect("write clean frob-cli Cargo.toml");
    fs::write(cli_dir.join("src/main.rs"), "fn main() {}\n").expect("write clean frob-cli main.rs");

    let rep = check_mutation_guard(&ws, false).expect("check clean product binary");
    assert!(
        rep.is_ok(),
        "clean product binary whose default build carries no conformance/mutation feature must pass, got: {:?}",
        rep.violations
    );
    assert_eq!(rep.product_binaries_checked, 1);
}

#[test]
fn product_binary_target_scoped_deps_rejected() {
    let ws = scratch_dir("prod_bin_target_deps");
    let cli_dir = ws.join("crates/frob-cli");
    fs::create_dir_all(cli_dir.join("src")).expect("mkdir frob-cli src");
    fs::create_dir_all(ws.join("crates/frob-conformance/registry")).expect("mkdir reg");
    fs::write(
        ws.join("crates/frob-conformance/registry/controls.toml"),
        r#"schema = "frob.gauntlet.controls/1"

[[control]]
id = "NC-0412"
seed = "CX-262"
gate = ["GATE-ARENA-04"]
kind = "mutation"
target = "frob-actuate::stage::integrate"
defect = "explicit Euler"
expect = "spectral radius 1.497"
feature = "cx-262"
due = "M1"
"#,
    )
    .expect("write controls");
    fs::write(
        cli_dir.join("Cargo.toml"),
        r#"[package]
name = "frob-cli"
version.workspace = true
edition.workspace = true

[[bin]]
name = "frob"
path = "src/main.rs"

[target.'cfg(unix)'.dependencies]
verbs_renamed = { package = "frob-verbs", path = "../frob-verbs", default-features = false, features = ["conformance"] }

[target.'cfg(target_os = "linux")'.dependencies.frob-actuate]
path = "../frob-actuate"
features = ["cx-262"]

[target.'cfg(unix)'.dev-dependencies]
frob-core = { path = "../frob-core", features = ["conformance", "cx-262"] }
"#,
    )
    .expect("write frob-cli Cargo.toml");
    fs::write(cli_dir.join("src/main.rs"), "fn main() {}\n").expect("write main.rs");

    let rep = check_mutation_guard(&ws, true).expect("check target-scoped deps");
    assert!(
        !rep.is_ok(),
        "product binary target-scoped deps enabling conformance or control feature must fail"
    );
    assert!(
        rep.violations.iter().any(|v| {
            v.crate_name == "frob-cli"
                && v.rule == "product-binary-conformance"
                && v.feature == "frob-verbs/conformance"
        }),
        "expected target inline renamed dep conformance violation, got: {:?}",
        rep.violations
    );
    assert!(
        rep.violations.iter().any(|v| {
            v.crate_name == "frob-cli"
                && v.rule == "product-binary-mutation-feature"
                && v.feature == "frob-actuate/cx-262"
        }),
        "expected target subtable dep control-feature violation, got: {:?}",
        rep.violations
    );
    assert!(
        !rep.violations
            .iter()
            .any(|v| v.feature.starts_with("frob-core/")),
        "target dev-dependencies must not trigger product binary violation: {:?}",
        rep.violations
    );
}

#[test]
fn product_binary_autobin_without_bin_table_rejected() {
    let ws = scratch_dir("prod_bin_autobin");
    let pkg_dir = ws.join("crates/auditor-pkg");
    fs::create_dir_all(pkg_dir.join("src/bin")).expect("mkdir src/bin");
    fs::create_dir_all(ws.join("crates/frob-conformance/registry")).expect("mkdir reg");
    fs::write(
        ws.join("crates/frob-conformance/registry/controls.toml"),
        r#"schema = "frob.gauntlet.controls/1"

[[control]]
id = "NC-0412"
seed = "CX-262"
gate = ["GATE-ARENA-04"]
kind = "mutation"
target = "frob-actuate::stage::integrate"
defect = "explicit Euler"
expect = "spectral radius 1.497"
feature = "cx-262"
due = "M1"
"#,
    )
    .expect("write controls");
    fs::write(pkg_dir.join("src/bin/frob-verify.rs"), "fn main() {}\n")
        .expect("write autobin frob-verify.rs");
    fs::write(
        pkg_dir.join("Cargo.toml"),
        r#"[package]
name = "auditor-pkg"
version = "0.0.1"
edition = "2024"

[dependencies]
frob-actuate = { path = "../frob-actuate", features = ["cx-262"] }
"#,
    )
    .expect("write auditor-pkg Cargo.toml");

    let rep = check_mutation_guard(&ws, true).expect("check autobin product binary");
    assert!(
        !rep.is_ok(),
        "auto-discovered src/bin/frob-verify.rs enabling control feature must fail"
    );
    assert!(
        rep.violations.iter().any(|v| {
            v.crate_name == "auditor-pkg"
                && v.rule == "product-binary-mutation-feature"
                && v.feature == "frob-actuate/cx-262"
        }),
        "expected autobin product-binary violation, got: {:?}",
        rep.violations
    );

    // Explicitly disabling autobins excludes src/bin/frob-verify.rs.
    fs::write(
        pkg_dir.join("Cargo.toml"),
        r#"[package]
name = "auditor-pkg"
version = "0.0.1"
edition = "2024"
autobins = false

[dependencies]
frob-actuate = { path = "../frob-actuate", features = ["cx-262"] }
"#,
    )
    .expect("write autobins=false Cargo.toml");
    let rep_disabled = check_mutation_guard(&ws, true).expect("check autobins=false");
    assert!(
        rep_disabled.is_ok(),
        "crate with autobins=false and no [[bin]] must not be treated as product binary: {:?}",
        rep_disabled.violations
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
