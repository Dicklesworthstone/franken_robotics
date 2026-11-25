#![deny(unsafe_code)]

//! Confinement, text-hint, and generator-row tests for `N33c-03` (`frob_core::qty`).
//!
//! Never deletes any file or directory (COMMON.md 9b).

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

use xtask::check_deps::{self, RULE_L2_FS_QTY_DIRECT, check_deps, find_fs_qty_path_hints};
use xtask::check_generated::run_check;
use xtask::cli::{self, CheckFn, EXIT_OK};
use xtask::generate::{BUILTIN_GENERATORS, RULE_DIFFERS};
use xtask::metadata::WorkspaceRoot;

static COUNTER: AtomicU64 = AtomicU64::new(0);
const SAMPLE_RECEIPT: &str = "d3b0885b1d5bfbe09039a84ae190c40fce4ad3b0b81b5c28bf946201987462d4";
const SAMPLE_GIT_REV: &str = "1111111111111111111111111111111111111111";

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("workspace root")
        .to_path_buf()
}

fn fresh_dir(tag: &str) -> PathBuf {
    let base = env::var_os("BIG_SCRATCH")
        .or_else(|| env::var_os("TMPDIR"))
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("/tmp"));
    let dir = base.join(format!(
        "frob_qty_confinement_{tag}_{}_{}",
        std::process::id(),
        COUNTER.fetch_add(1, Ordering::Relaxed)
    ));
    fs::create_dir_all(&dir).expect("mkdir scratch");
    dir
}

fn write_file(path: &Path, content: &str) {
    if let Some(p) = path.parent() {
        fs::create_dir_all(p).expect("mkdir parent");
    }
    fs::write(path, content).expect("write file");
}

fn copy_file(src: &Path, dst: &Path) {
    if let Some(p) = dst.parent() {
        fs::create_dir_all(p).expect("mkdir parent");
    }
    fs::copy(src, dst).expect("copy file");
}

fn copy_dir_recursive(src: &Path, dst: &Path) {
    if !src.is_dir() {
        return;
    }
    fs::create_dir_all(dst).expect("mkdir dst");
    for ent in fs::read_dir(src).expect("read_dir").flatten() {
        let p = ent.path();
        let target = dst.join(ent.file_name());
        if p.is_dir() {
            copy_dir_recursive(&p, &target);
        } else {
            fs::copy(&p, &target).expect("copy file");
        }
    }
}

fn make_synth_repo(tag: &str, frob_lie_deps: &str, frob_lie_src: &str) -> PathBuf {
    let base = fresh_dir(tag);
    let repo = base.join("repo");
    let fs_qty = base.join("frankensim/crates/fs-qty");
    write_file(
        &fs_qty.join("Cargo.toml"),
        "[package]\nname = \"fs-qty\"\nversion = \"0.0.1\"\nedition = \"2021\"\n",
    );
    write_file(&fs_qty.join("src/lib.rs"), "pub struct Length;\n");

    write_file(
        &repo.join("Cargo.toml"),
        "[workspace]\nresolver = \"3\"\nmembers = [\"crates/frob-core\", \"crates/frob-fsim\", \"crates/frob-lie\"]\n",
    );
    for (dir, name, layer, rank, deps, src) in [
        (
            "crates/frob-core",
            "frob-core",
            "0",
            1,
            "[dependencies]\nfs-qty = { path = \"../../../frankensim/crates/fs-qty\", default-features = false }\n",
            "// frob-core allowed\n",
        ),
        (
            "crates/frob-fsim",
            "frob-fsim",
            "0",
            2,
            "[dependencies]\nfs-qty = { path = \"../../../frankensim/crates/fs-qty\", default-features = false }\n",
            "// frob-fsim allowed\n",
        ),
        (
            "crates/frob-lie",
            "frob-lie",
            "1",
            1,
            frob_lie_deps,
            frob_lie_src,
        ),
    ] {
        write_file(
            &repo.join(dir).join("Cargo.toml"),
            &format!(
                "[package]\nname = \"{name}\"\nversion = \"0.0.1\"\nedition = \"2024\"\n\n[package.metadata.frob]\nlayer = {layer}\nrank = {rank}\n\n{deps}\n"
            ),
        );
        write_file(&repo.join(dir).join("src/lib.rs"), src);
    }

    let lock_toml = format!(
        "toolchain = \"nightly-2026-07-06\"\n\n[[foundation]]\nid = \"FD-FSIM\"\nname = \"frankensim\"\nposture = \"consume-path\"\npath = \"../frankensim\"\ngit_rev = \"{SAMPLE_GIT_REV}\"\ntoolchain = \"nightly-2026-07-06\"\nfeatures = [\"referee\"]\nconfined_to = [\"frob-fsim\", \"bedrock-direct\"]\nadmission_gate = \"INT-FSIM-01\"\nreceipt = \"{SAMPLE_RECEIPT}\"\nfallback = \"synthetic\"\n"
    );
    write_file(&repo.join("franken-stack.lock"), &lock_toml);

    let out = Command::new("cargo")
        .current_dir(&repo)
        .args(["generate-lockfile", "--offline", "--manifest-path"])
        .arg(repo.join("Cargo.toml"))
        .output()
        .expect("generate lockfile");
    assert!(
        out.status.success(),
        "generate-lockfile failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    repo
}

#[test]
fn check_deps_rejects_fs_qty_outside_allowed() {
    let repo = make_synth_repo(
        "reject",
        "[dependencies]\nfs-qty = { path = \"../../../frankensim/crates/fs-qty\", default-features = false }\n",
        "// planted forbidden direct fs-qty edge\n",
    );
    let outcome = check_deps(&repo, &[WorkspaceRoot::core(&repo)]).expect("check_deps runs");
    assert!(
        outcome.findings.iter().any(|f| {
            f.rule == RULE_L2_FS_QTY_DIRECT
                && f.subject == "frob-lie -> fs-qty"
                && f.path == "crates/frob-lie/Cargo.toml"
        }),
        "expected L-2.fs-qty-direct for frob-lie -> fs-qty, got {:?}",
        outcome.findings
    );
}

#[test]
fn fs_qty_path_hint_reported() {
    let repo = make_synth_repo(
        "hint",
        "",
        "#[allow(unused)]\nfn hint_sample() { let _ = fs_qty::Length; }\n",
    );
    let hints = find_fs_qty_path_hints(&repo);
    assert!(
        hints
            .iter()
            .any(|h| h.contains("crates/frob-lie/src/lib.rs:2: fs_qty::")),
        "expected fs_qty:: hint in frob-lie, got {hints:?}"
    );
    // Reported as a `facts` entry, NOT a finding when manifest has no forbidden dep.
    const CMDS: &[(&str, CheckFn)] = &[("check-deps", check_deps::run_command)];
    let args = vec![
        "check-deps".to_owned(),
        "--root".to_owned(),
        repo.to_string_lossy().into_owned(),
        "--json".to_owned(),
    ];
    let (mut out, mut err) = (Vec::new(), Vec::new());
    let exit = cli::run(CMDS, &args, &mut out, &mut err);
    assert_eq!(
        exit,
        std::process::ExitCode::from(EXIT_OK),
        "hint must not become a finding: {}",
        String::from_utf8_lossy(&out)
    );
    let json = String::from_utf8(out).expect("utf8");
    assert!(json.contains("\"fs_qty_path_hints\""));
    assert!(json.contains("crates/frob-lie/src/lib.rs:2: fs_qty::"));
}

#[test]
fn qty_generator_row() {
    let real = repo_root();
    let dir = fresh_dir("gen");
    for rel in [
        "README.md",
        "DEPENDENCY_ADMISSION.md",
        "crates/frob-core/src/refusal/code_gen.rs",
        "crates/frob-core/src/qty_gen.rs",
        "crates/xtask/src/stack/lock.rs",
    ] {
        copy_file(&real.join(rel), &dir.join(rel));
    }
    copy_dir_recursive(&real.join("registries"), &dir.join("registries"));
    copy_dir_recursive(&real.join("schemas"), &dir.join("schemas"));
    copy_dir_recursive(
        &real.join("receipts/admissions"),
        &dir.join("receipts/admissions"),
    );
    copy_dir_recursive(&real.join("kernels"), &dir.join("kernels"));

    // Clean tree passes check-generated.
    let clean = run_check(&dir, BUILTIN_GENERATORS, false).expect("clean check");
    assert!(
        clean.findings.is_empty(),
        "clean findings: {:?}",
        clean.findings
    );

    // Plant 1: flip one byte in qty_gen.rs -> gen.differs.
    let gen_path = dir.join("crates/frob-core/src/qty_gen.rs");
    let orig = fs::read_to_string(&gen_path).expect("read qty_gen.rs");
    let mutated = orig.replacen("ALIAS_COUNT: usize = 36;", "ALIAS_COUNT: usize = 37;", 1);
    assert_ne!(mutated, orig);
    fs::write(&gen_path, &mutated).expect("write mutated qty_gen.rs");
    let bad_byte = run_check(&dir, BUILTIN_GENERATORS, false).expect("check mutated byte");
    assert!(
        bad_byte
            .findings
            .iter()
            .any(|f| f.rule == RULE_DIFFERS && f.path == "crates/frob-core/src/qty_gen.rs"),
        "expected gen.differs on mutated qty_gen.rs, got {:?}",
        bad_byte.findings
    );

    // Restore qty_gen.rs, then Plant 2: append an alias to qty_aliases.toml without regeneration -> gen.differs.
    fs::write(&gen_path, &orig).expect("restore qty_gen.rs");
    let aliases_path = dir.join("registries/qty_aliases.toml");
    let mut aliases = fs::read_to_string(&aliases_path).expect("read aliases");
    aliases.push_str(
        "\n[[alias]]\nname = \"Jerk\"\ndimension = [1, 0, -3, 0, 0, 0]\nkind = \"jerk\"\nunit = \"m·s^-3\"\n",
    );
    fs::write(&aliases_path, &aliases).expect("append alias");
    let bad_reg = run_check(&dir, BUILTIN_GENERATORS, false).expect("check unregenerated registry");
    assert!(
        bad_reg
            .findings
            .iter()
            .any(|f| f.rule == RULE_DIFFERS && f.path == "crates/frob-core/src/qty_gen.rs"),
        "expected gen.differs on unregenerated alias, got {:?}",
        bad_reg.findings
    );
}
