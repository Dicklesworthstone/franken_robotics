//! Integration and unit tests for `xtask check-deps` (bead BD-003, rules L-2,
//! L-4, L-5, L-9, proof-boundary edges, closure facts, and ownership partition
//! with `check-layers`).
//!
//! Synthetic scratch workspaces are created under
//! `${BIG_SCRATCH:-${TMPDIR:-/tmp}}/frob-BrownHeron/` and never deleted (COMMON.md 9b).
//! Every workspace check runs through the real `cargo metadata --no-deps --locked --offline`
//! and `cargo metadata --locked --offline` readers, `frob_core::json::admit`,
//! `stack::lock::load` (N33a-03), and `stack::posture::edge_allowed` (N33a-01).

use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use frob_core::json::{Limits, admit};
use xtask::check_deps::{
    self, PROOF_BOUNDARY_EDGES, RULE_EDGE_FORBIDDEN, RULE_FSIM_VOLATILE_EDGE,
    RULE_L2_AIRLOCK_REEXPORT, RULE_L2_FS_QTY_DIRECT, RULE_L4_EDGE_SPEC,
    RULE_L4_FOUNDATION_CONFINED, RULE_L5_FORBIDDEN, RULE_L5_PYTHON_OUTSIDE_BINDINGS,
    RULE_L5_SERDE_NOT_EXACT, RULE_L5_SERDE_OUTSIDE_BOUNDARY, RULE_L5_THIRD_PARTY,
    RULE_L9_CERT_DEPENDENT, RULE_L9_CERT_HAS_DEPS, check_deps, check_deps_with_config,
};
use xtask::check_layers::{self, RULE_L1_RANK, RULE_L1_UPWARD, check_layers, check_layers_with_tables};
use xtask::cli::{self, CheckFn, EXIT_FINDINGS, EXIT_OK, EXIT_USAGE};
use xtask::crate_map::{CrateRow, Layer, Reason, TOMBSTONES, Tier};
use xtask::inversions::INVERSIONS;
use xtask::metadata::{
    OFFLINE_FETCH_REPAIR, WorkspaceRoot, load_declared_metadata, normalize_rel_path,
};
use xtask::report::{CheckReport, Finding, REPORT_SCHEMA, validate_report_json};
use xtask::stack::lock;

static COUNTER: AtomicU64 = AtomicU64::new(0);

const TEST_COMMANDS: &[(&str, CheckFn)] = &[
    ("check-layers", check_layers::run_command),
    ("check-deps", check_deps::run_command),
];

const SAMPLE_RECEIPT: &str = "d3b0885b1d5bfbe09039a84ae190c40fce4ad3b0b81b5c28bf946201987462d4";
const SAMPLE_GIT_REV: &str = "1111111111111111111111111111111111111111";

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crates/xtask sits two levels below repo root")
        .to_path_buf()
}

fn make_scratch_dir(label: &str) -> PathBuf {
    let base = env::var_os("BIG_SCRATCH")
        .or_else(|| env::var_os("TMPDIR"))
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("/tmp"));
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_secs());
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    let pid = std::process::id();
    let dir = base
        .join("frob-BrownHeron")
        .join(format!("xtask-check-deps-{label}-{ts}-{pid}-{n}"));
    fs::create_dir_all(&dir).expect("create scratch dir");
    dir
}

fn write_file(path: &Path, content: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("create parent dirs");
    }
    fs::write(path, content).expect("write synthetic fixture file");
}

/// Synthetic workspace harness with sibling repositories and an offline local
/// Cargo directory source so every test runs real `cargo metadata --locked --offline`.
struct SynthEnv {
    base: PathBuf,
    repo: PathBuf,
    reg: PathBuf,
    git_sources: Vec<String>,
}

impl SynthEnv {
    fn new(label: &str) -> Self {
        let base = make_scratch_dir(label);
        let repo = base.join("repo");
        let reg = base.join("local-reg");
        fs::create_dir_all(&repo).expect("create repo dir");
        fs::create_dir_all(&reg).expect("create local-reg dir");
        Self {
            base,
            repo,
            reg,
            git_sources: Vec::new(),
        }
    }

    fn write_workspace(&self, members: &[&str], exclude: &[&str]) {
        let m: Vec<String> = members.iter().map(|s| format!("\"{s}\"")).collect();
        let mut toml = format!(
            "# synthetic workspace for BD-003\n[workspace]\nresolver = \"3\"\nmembers = [{}]\n",
            m.join(", ")
        );
        if !exclude.is_empty() {
            let e: Vec<String> = exclude.iter().map(|s| format!("\"{s}\"")).collect();
            toml.push_str(&format!("exclude = [{}]\n", e.join(", ")));
        }
        write_file(&self.repo.join("Cargo.toml"), &toml);
    }

    fn write_crate(
        &self,
        rel_dir: &str,
        name: &str,
        layer_toml: &str,
        rank: u8,
        extra_toml: &str,
        lib_rs: &str,
    ) {
        let dir = self.repo.join(rel_dir);
        let meta = if layer_toml.is_empty() {
            String::new()
        } else {
            format!("[package.metadata.frob]\nlayer = {layer_toml}\nrank = {rank}\n")
        };
        let manifest = format!(
            "# synthetic crate for BD-003\n[package]\nname = \"{name}\"\nversion = \"0.0.1\"\nedition = \"2024\"\n\n{meta}\n{extra_toml}\n"
        );
        write_file(&dir.join("Cargo.toml"), &manifest);
        write_file(
            &dir.join("src/lib.rs"),
            if lib_rs.is_empty() {
                "// synthetic crate source\n"
            } else {
                lib_rs
            },
        );
    }

    /// Create a sibling crate outside `repo` (e.g. `frankensim/crates/fs-ga`).
    fn write_sibling_crate(
        &self,
        sib_rel_dir: &str,
        name: &str,
        version: &str,
        extra_toml: &str,
        with_build_rs: bool,
    ) {
        let dir = self.base.join(sib_rel_dir);
        let manifest = format!(
            "# synthetic sibling crate for BD-003\n[package]\nname = \"{name}\"\nversion = \"{version}\"\nedition = \"2021\"\n\n{extra_toml}\n"
        );
        write_file(&dir.join("Cargo.toml"), &manifest);
        write_file(&dir.join("src/lib.rs"), "// synthetic sibling crate\n");
        if with_build_rs {
            write_file(&dir.join("build.rs"), "fn main() {}\n");
        }
    }

    /// Add a synthetic package to the offline local directory registry (`crates.io` replacement).
    fn add_reg_crate(&self, name: &str, version: &str, extra_toml: &str, with_build_rs: bool) {
        let dir = self.reg.join(format!("{name}-{version}"));
        let manifest = format!(
            "# synthetic registry crate for BD-003\n[package]\nname = \"{name}\"\nversion = \"{version}\"\nedition = \"2021\"\n\n{extra_toml}\n"
        );
        write_file(&dir.join("Cargo.toml"), &manifest);
        write_file(&dir.join("src/lib.rs"), "// synthetic registry crate\n");
        if with_build_rs {
            write_file(&dir.join("build.rs"), "fn main() {}\n");
        }
        write_file(&dir.join(".cargo-checksum.json"), "{\"files\":{}}\n");
    }

    fn add_git_source_replacement(&mut self, git_url: &str) {
        self.git_sources.push(git_url.to_owned());
    }

    /// Write `.cargo/config.toml` at `self.base` (so both `repo` and any annex workspace
    /// under `repo` inherit it when Cargo runs with `current_dir(&ws.root_dir)`).
    fn write_cargo_config(&self) {
        let reg_str = normalize_rel_path(&self.reg);
        let mut cfg = "\
# synthetic offline registry config for BD-003
[source.crates-io]
replace-with = \"local-reg\"

"
        .to_owned();
        for url in &self.git_sources {
            cfg.push_str(&format!(
                "[source.\"git+{url}\"]\n\
                 git = \"{url}\"\n\
                 replace-with = \"local-reg\"\n\n"
            ));
        }
        cfg.push_str(&format!(
            "[source.local-reg]\n\
             directory = \"{reg_str}\"\n"
        ));
        write_file(&self.base.join(".cargo/config.toml"), &cfg);
    }

    /// Write a valid `franken-stack.lock` at `repo/franken-stack.lock`.
    fn write_stack_lock(&self, extra_foundations: &str) {
        let lock_toml = format!(
            "# synthetic franken-stack.lock for BD-003\n\
             toolchain = \"nightly-2026-08-31\"\n\n\
             [[foundation]]\n\
             id = \"FD-FSIM\"\n\
             name = \"frankensim\"\n\
             posture = \"consume-path\"\n\
             path = \"../frankensim\"\n\
             git_rev = \"{SAMPLE_GIT_REV}\"\n\
             toolchain = \"nightly-2026-07-06\"\n\
             features = [\"referee\", \"geometry\", \"ledger\"]\n\
             confined_to = [\"frob-fsim\", \"bedrock-direct\"]\n\
             admission_gate = \"INT-FSIM-01\"\n\
             receipt = \"{SAMPLE_RECEIPT}\"\n\
             fallback = \"synthetic fallback\"\n\n\
             [[foundation]]\n\
             id = \"FD-ASUP\"\n\
             name = \"asupersync\"\n\
             posture = \"consume\"\n\
             version = \"=0.6.2\"\n\
             confined_to = [\"any\"]\n\
             admission_gate = \"INT-ASUP-01\"\n\
             receipt = \"{SAMPLE_RECEIPT}\"\n\
             fallback = \"synthetic fallback\"\n\n\
             [[foundation]]\n\
             id = \"FD-FSQL\"\n\
             name = \"frankensqlite\"\n\
             posture = \"consume\"\n\
             version = \"=0.5.1\"\n\
             confined_to = [\"frob-chronicle\", \"frob-conformance\"]\n\
             admission_gate = \"INT-FSQL-01\"\n\
             receipt = \"{SAMPLE_RECEIPT}\"\n\
             fallback = \"synthetic fallback\"\n\n\
             [[foundation]]\n\
             id = \"FD-FKERN\"\n\
             name = \"franken-kernel\"\n\
             posture = \"oracle\"\n\
             confined_to = [\"frob-conformance\"]\n\
             admission_gate = \"INT-FKERN-01\"\n\
             receipt = \"{SAMPLE_RECEIPT}\"\n\
             fallback = \"synthetic fallback\"\n\n\
             {extra_foundations}\n"
        );
        write_file(&self.repo.join("franken-stack.lock"), &lock_toml);
    }

    /// Generate `Cargo.lock` completely offline for a workspace directory.
    fn generate_lockfile(&self, ws_dir: &Path) {
        self.write_cargo_config();
        let out = Command::new("cargo")
            .current_dir(ws_dir)
            .args(["generate-lockfile", "--offline", "--manifest-path"])
            .arg(ws_dir.join("Cargo.toml"))
            .output()
            .expect("invoke cargo generate-lockfile --offline");
        assert!(
            out.status.success(),
            "cargo generate-lockfile --offline failed in {}:\n{}",
            ws_dir.display(),
            String::from_utf8_lossy(&out.stderr)
        );
    }
}

fn rules_of(findings: &[Finding]) -> Vec<&'static str> {
    findings.iter().map(|f| f.rule).collect()
}

// ============================================================================
// Step 1: L-2 (FrankenSim volatile confinement, fs-qty, airlock re-exports)
// ============================================================================

#[test]
fn volatile_outside_airlock_fails() {
    let env = SynthEnv::new("volatile-outside-airlock");
    env.write_workspace(&["crates/*"], &[]);
    env.write_sibling_crate("frankensim/crates/fs-mbd", "fs-mbd", "0.1.0", "", false);
    env.write_crate(
        "crates/frob-dyn",
        "frob-dyn",
        "3",
        3,
        "[dependencies]\nfs-mbd = { path = \"../../../frankensim/crates/fs-mbd\", default-features = false }\n",
        "",
    );
    env.write_stack_lock("");
    env.generate_lockfile(&env.repo);

    let outcome = check_deps(&env.repo, &[WorkspaceRoot::core(&env.repo)]).expect("check_deps");
    assert_eq!(rules_of(&outcome.findings), vec![RULE_FSIM_VOLATILE_EDGE]);
    assert_eq!(outcome.findings[0].subject, "frob-dyn -> fs-mbd");
    assert_eq!(outcome.findings[0].path, "crates/frob-dyn/Cargo.toml");
}

#[test]
fn bedrock_direct_passes() {
    let env = SynthEnv::new("bedrock-direct-passes");
    env.write_workspace(&["crates/*"], &[]);
    env.write_sibling_crate("frankensim/crates/fs-ga", "fs-ga", "0.1.0", "", false);
    env.write_crate(
        "crates/frob-lie",
        "frob-lie",
        "1",
        1,
        "[dependencies]\nfs-ga = { path = \"../../../frankensim/crates/fs-ga\", default-features = false }\n",
        "",
    );
    env.write_stack_lock("");
    env.generate_lockfile(&env.repo);

    let outcome = check_deps(&env.repo, &[WorkspaceRoot::core(&env.repo)]).expect("check_deps");
    assert!(
        outcome.findings.is_empty(),
        "bedrock-direct fs-ga from frob-lie must pass, got {:?}",
        outcome.findings
    );
}

#[test]
fn fs_qty_direct_fails() {
    let env = SynthEnv::new("fs-qty-direct");
    env.write_workspace(&["crates/*"], &[]);
    env.write_sibling_crate("frankensim/crates/fs-qty", "fs-qty", "0.1.0", "", false);
    // Allowed: frob-core and frob-fsim.
    env.write_crate(
        "crates/frob-core",
        "frob-core",
        "0",
        1,
        "[dependencies]\nfs-qty = { path = \"../../../frankensim/crates/fs-qty\", default-features = false }\n",
        "",
    );
    env.write_crate(
        "crates/frob-fsim",
        "frob-fsim",
        "0",
        2,
        "[dependencies]\nfs-qty = { path = \"../../../frankensim/crates/fs-qty\", default-features = false }\n",
        "",
    );
    // Forbidden: frob-lie directly depending on fs-qty.
    env.write_crate(
        "crates/frob-lie",
        "frob-lie",
        "1",
        1,
        "[dependencies]\nfs-qty = { path = \"../../../frankensim/crates/fs-qty\", default-features = false }\n",
        "",
    );
    env.write_stack_lock("");
    env.generate_lockfile(&env.repo);

    let outcome = check_deps(&env.repo, &[WorkspaceRoot::core(&env.repo)]).expect("check_deps");
    assert_eq!(rules_of(&outcome.findings), vec![RULE_L2_FS_QTY_DIRECT]);
    assert_eq!(outcome.findings[0].subject, "frob-lie -> fs-qty");
    assert_eq!(outcome.findings[0].path, "crates/frob-lie/Cargo.toml");
}

#[test]
fn airlock_reexport_fails() {
    let env = SynthEnv::new("airlock-reexport");
    env.write_workspace(&["crates/*"], &[]);
    env.write_sibling_crate("frankensim/crates/fs-mbd", "fs-mbd", "0.1.0", "", false);
    env.write_sibling_crate("frankensim/crates/fs-dyn", "fs-dyn", "0.1.0", "", false);
    env.write_crate(
        "crates/frob-fsim",
        "frob-fsim",
        "0",
        2,
        "[dependencies]\nfs-mbd = { path = \"../../../frankensim/crates/fs-mbd\", default-features = false }\nfs-dyn = { path = \"../../../frankensim/crates/fs-dyn\", default-features = false }\n",
        "\
// pub use fs_mbd::CommentOnly;
pub const DOC: &str = \"pub use fs_mbd::StringOnly;\";
pub(crate) use fs_mbd::InternalOnly;
pub use fs_mbd::World;
pub extern crate fs_dyn;
",
    );
    env.write_stack_lock("");
    env.generate_lockfile(&env.repo);

    let outcome = check_deps(&env.repo, &[WorkspaceRoot::core(&env.repo)]).expect("check_deps");
    assert_eq!(
        rules_of(&outcome.findings),
        vec![RULE_L2_AIRLOCK_REEXPORT, RULE_L2_AIRLOCK_REEXPORT]
    );
    let subjects: Vec<&str> = outcome.findings.iter().map(|f| f.subject.as_str()).collect();
    assert_eq!(subjects, vec!["frob-fsim::fs_dyn", "frob-fsim::fs_mbd"]);
    let lines: Vec<Option<u32>> = outcome.findings.iter().map(|f| f.line).collect();
    assert_eq!(lines, vec![Some(5), Some(4)]);
}

#[test]
fn volatile_edge_reported_once() {
    // frob-dyn -> fs-mbd must emit ONLY `fsim.volatile-edge`, never an L-4 finding on the same edge.
    let env = SynthEnv::new("volatile-edge-once");
    env.write_workspace(&["crates/*"], &[]);
    env.write_sibling_crate("frankensim/crates/fs-mbd", "fs-mbd", "0.1.0", "", false);
    env.write_crate(
        "crates/frob-dyn",
        "frob-dyn",
        "3",
        3,
        "[dependencies]\nfs-mbd = { path = \"../../../frankensim/crates/fs-mbd\" }\n",
        "",
    );
    env.write_stack_lock("");
    env.generate_lockfile(&env.repo);

    let outcome = check_deps(&env.repo, &[WorkspaceRoot::core(&env.repo)]).expect("check_deps");
    assert_eq!(outcome.findings.len(), 1, "expected exactly one finding: {:?}", outcome.findings);
    assert_eq!(outcome.findings[0].rule, RULE_FSIM_VOLATILE_EDGE);
    assert!(
        outcome
            .findings
            .iter()
            .all(|f| !f.rule.starts_with("L-4.")),
        "must not emit any L-4 finding on volatile edge: {:?}",
        outcome.findings
    );
}

// ============================================================================
// Step 2: L-4 (stack-lock confinement & spec via N33a-01 `edge_allowed`)
// ============================================================================

#[test]
fn foundation_confined() {
    // 1. frankensqlite from frob-arena fails `L-4.foundation-confined`.
    let bad = SynthEnv::new("foundation-confined-arena");
    bad.write_workspace(&["crates/*"], &[]);
    bad.add_reg_crate("frankensqlite", "0.5.1", "", false);
    bad.write_crate(
        "crates/frob-arena",
        "frob-arena",
        "4",
        6,
        "[dependencies]\nfrankensqlite = { version = \"=0.5.1\", default-features = false }\n",
        "",
    );
    bad.write_stack_lock("");
    bad.generate_lockfile(&bad.repo);
    let out_bad = check_deps(&bad.repo, &[WorkspaceRoot::core(&bad.repo)]).expect("check_deps");
    assert_eq!(rules_of(&out_bad.findings), vec![RULE_L4_FOUNDATION_CONFINED]);
    assert_eq!(out_bad.findings[0].subject, "frob-arena -> frankensqlite");

    // 2. frankensqlite from frob-chronicle passes.
    let ok = SynthEnv::new("foundation-confined-chronicle");
    ok.write_workspace(&["crates/*"], &[]);
    ok.add_reg_crate("frankensqlite", "0.5.1", "", false);
    ok.write_crate(
        "crates/frob-chronicle",
        "frob-chronicle",
        "6",
        2,
        "[dependencies]\nfrankensqlite = { version = \"=0.5.1\", default-features = false }\n",
        "",
    );
    ok.write_stack_lock("");
    ok.generate_lockfile(&ok.repo);
    let out_ok = check_deps(&ok.repo, &[WorkspaceRoot::core(&ok.repo)]).expect("check_deps");
    assert!(
        out_ok.findings.is_empty(),
        "frankensqlite from frob-chronicle must pass, got {:?}",
        out_ok.findings
    );

    // 3. A sibling path dependency with no foundation row fails `L-4.foundation-confined`.
    let orphan = SynthEnv::new("foundation-confined-orphan-path");
    orphan.write_workspace(&["crates/*"], &[]);
    orphan.write_sibling_crate("unregistered-sib", "unregistered-sib", "0.1.0", "", false);
    orphan.write_crate(
        "crates/frob-dyn",
        "frob-dyn",
        "3",
        3,
        "[dependencies]\nunregistered-sib = { path = \"../../../unregistered-sib\", default-features = false }\n",
        "",
    );
    orphan.write_stack_lock("");
    orphan.generate_lockfile(&orphan.repo);
    let out_orphan =
        check_deps(&orphan.repo, &[WorkspaceRoot::core(&orphan.repo)]).expect("check_deps");
    assert_eq!(
        rules_of(&out_orphan.findings),
        vec![RULE_L4_FOUNDATION_CONFINED]
    );
    assert_eq!(out_orphan.findings[0].subject, "frob-dyn -> unregistered-sib");
}

#[test]
fn foundation_edge_spec() {
    // 1. `consume` edge (`asupersync`) with caret `^0.6` -> `L-4.edge-spec`.
    let caret = SynthEnv::new("edge-spec-caret");
    caret.write_workspace(&["crates/*"], &[]);
    caret.add_reg_crate("asupersync", "0.6.2", "", false);
    caret.write_crate(
        "crates/frob-dyn",
        "frob-dyn",
        "3",
        3,
        "[dependencies]\nasupersync = { version = \"^0.6\", default-features = false }\n",
        "",
    );
    caret.write_stack_lock("");
    caret.generate_lockfile(&caret.repo);
    let out_caret = check_deps(&caret.repo, &[WorkspaceRoot::core(&caret.repo)]).expect("caret");
    assert_eq!(rules_of(&out_caret.findings), vec![RULE_L4_EDGE_SPEC]);

    // 2. `consume` edge (`asupersync`) with default-features on -> `L-4.edge-spec`.
    let def_on = SynthEnv::new("edge-spec-default-features");
    def_on.write_workspace(&["crates/*"], &[]);
    def_on.add_reg_crate("asupersync", "0.6.2", "", false);
    def_on.write_crate(
        "crates/frob-dyn",
        "frob-dyn",
        "3",
        3,
        "[dependencies]\nasupersync = \"=0.6.2\"\n",
        "",
    );
    def_on.write_stack_lock("");
    def_on.generate_lockfile(&def_on.repo);
    let out_def = check_deps(&def_on.repo, &[WorkspaceRoot::core(&def_on.repo)]).expect("def_on");
    assert_eq!(rules_of(&out_def.findings), vec![RULE_L4_EDGE_SPEC]);

    // 3. `consume` edge (`asupersync`) with `=0.6.2` and default-features off -> passes.
    let exact = SynthEnv::new("edge-spec-exact-ok");
    exact.write_workspace(&["crates/*"], &[]);
    exact.add_reg_crate("asupersync", "0.6.2", "", false);
    exact.write_crate(
        "crates/frob-dyn",
        "frob-dyn",
        "3",
        3,
        "[dependencies]\nasupersync = { version = \"=0.6.2\", default-features = false }\n",
        "",
    );
    exact.write_stack_lock("");
    exact.generate_lockfile(&exact.repo);
    let out_exact = check_deps(&exact.repo, &[WorkspaceRoot::core(&exact.repo)]).expect("exact");
    assert!(
        out_exact.findings.is_empty(),
        "exact consume pin with default-features=false must pass, got {:?}",
        out_exact.findings
    );

    // 4. `consume-path` edge through a git source -> `L-4.edge-spec` (100% offline via Cargo source replacement + lockfile).
    let mut git_env = SynthEnv::new("edge-spec-consume-path-git");
    let git_url = "https://example.invalid/frankensim.git";
    git_env.add_git_source_replacement(git_url);
    git_env.write_workspace(&["crates/*"], &[]);
    git_env.add_reg_crate("fs-ga", "0.1.0", "", false);
    git_env.write_crate(
        "crates/frob-lie",
        "frob-lie",
        "1",
        1,
        &format!("[dependencies]\nfs-ga = {{ git = \"{git_url}\", default-features = false }}\n"),
        "",
    );
    git_env.write_stack_lock("");
    git_env.write_cargo_config();
    write_file(
        &git_env.repo.join("Cargo.lock"),
        &format!(
            "# This file is automatically @generated by Cargo.\n\
             # It is not intended for manual editing.\n\
             version = 4\n\n\
             [[package]]\n\
             name = \"frob-lie\"\n\
             version = \"0.0.1\"\n\
             dependencies = [\n\
              \"fs-ga\",\n\
             ]\n\n\
             [[package]]\n\
             name = \"fs-ga\"\n\
             version = \"0.1.0\"\n\
             source = \"git+{git_url}#{SAMPLE_GIT_REV}\"\n"
        ),
    );
    let out_git =
        check_deps(&git_env.repo, &[WorkspaceRoot::core(&git_env.repo)]).expect("consume-path git");
    assert_eq!(rules_of(&out_git.findings), vec![RULE_L4_EDGE_SPEC]);
    assert_eq!(out_git.findings[0].subject, "frob-lie -> fs-ga");

    // 5. `oracle` row (`franken-kernel`): normal dep of `frob-conformance` -> `L-4.foundation-confined`,
    //    dev-dependency of `frob-conformance` -> passes.
    let oracle_norm = SynthEnv::new("edge-spec-oracle-normal");
    oracle_norm.write_workspace(&["crates/*"], &[]);
    oracle_norm.add_reg_crate("franken-kernel", "0.2.0", "", false);
    oracle_norm.write_crate(
        "crates/frob-conformance",
        "frob-conformance",
        "7",
        4,
        "[dependencies]\nfranken-kernel = { version = \"=0.2.0\", default-features = false }\n",
        "",
    );
    oracle_norm.write_stack_lock("");
    oracle_norm.generate_lockfile(&oracle_norm.repo);
    let out_oracle_norm =
        check_deps(&oracle_norm.repo, &[WorkspaceRoot::core(&oracle_norm.repo)])
            .expect("oracle normal");
    assert_eq!(
        rules_of(&out_oracle_norm.findings),
        vec![RULE_L4_FOUNDATION_CONFINED]
    );
    assert_eq!(
        out_oracle_norm.findings[0].subject,
        "frob-conformance -> franken-kernel"
    );

    let oracle_dev = SynthEnv::new("edge-spec-oracle-dev");
    oracle_dev.write_workspace(&["crates/*"], &[]);
    oracle_dev.add_reg_crate("franken-kernel", "0.2.0", "", false);
    oracle_dev.write_crate(
        "crates/frob-conformance",
        "frob-conformance",
        "7",
        4,
        "[dev-dependencies]\nfranken-kernel = { version = \"=0.2.0\", default-features = false }\n",
        "",
    );
    oracle_dev.write_stack_lock("");
    oracle_dev.generate_lockfile(&oracle_dev.repo);
    let out_oracle_dev =
        check_deps(&oracle_dev.repo, &[WorkspaceRoot::core(&oracle_dev.repo)]).expect("oracle dev");
    assert!(
        out_oracle_dev.findings.is_empty(),
        "oracle dev-dep in frob-conformance must pass, got {:?}",
        out_oracle_dev.findings
    );
}

// ============================================================================
// Step 3 & Step 4: L-5 (third-party, serde boundary, forbidden, Python, native)
// ============================================================================

#[test]
fn serde_boundary_rules() {
    // Case 1: `serde_json = "1"` (non-exact) in boundary crate `frob-verbs` -> `L-5.serde-not-exact`.
    let non_exact = SynthEnv::new("serde-not-exact");
    non_exact.write_workspace(&["crates/*"], &[]);
    non_exact.add_reg_crate("serde_json", "1.0.154", "", false);
    non_exact.write_crate(
        "crates/frob-verbs",
        "frob-verbs",
        "6",
        4,
        "[dependencies]\nserde_json = \"1\"\n",
        "",
    );
    non_exact.write_stack_lock("");
    non_exact.generate_lockfile(&non_exact.repo);
    let out_ne = check_deps(&non_exact.repo, &[WorkspaceRoot::core(&non_exact.repo)]).expect("ne");
    assert_eq!(rules_of(&out_ne.findings), vec![RULE_L5_SERDE_NOT_EXACT]);
    assert_eq!(out_ne.findings[0].subject, "frob-verbs -> serde_json");

    // Case 2: `serde = "=1.0.210"` in non-boundary crate `frob-dyn` -> `L-5.serde-outside-boundary`.
    let outside = SynthEnv::new("serde-outside-boundary");
    outside.write_workspace(&["crates/*"], &[]);
    outside.add_reg_crate("serde", "1.0.210", "", false);
    outside.write_crate(
        "crates/frob-dyn",
        "frob-dyn",
        "3",
        3,
        "[dependencies]\nserde = \"=1.0.210\"\n",
        "",
    );
    outside.write_stack_lock("");
    outside.generate_lockfile(&outside.repo);
    let out_out = check_deps(&outside.repo, &[WorkspaceRoot::core(&outside.repo)]).expect("out");
    assert_eq!(
        rules_of(&out_out.findings),
        vec![RULE_L5_SERDE_OUTSIDE_BOUNDARY]
    );
    assert_eq!(out_out.findings[0].subject, "frob-dyn -> serde");

    // Case 3: exact pins in boundary crate `frob-verbs` -> passes (and unadmitted third-party fails `L-5.third-party`).
    let ok = SynthEnv::new("serde-boundary-ok");
    ok.write_workspace(&["crates/*"], &[]);
    ok.add_reg_crate("serde", "1.0.210", "", false);
    ok.add_reg_crate("serde_json", "1.0.154", "", false);
    ok.write_crate(
        "crates/frob-verbs",
        "frob-verbs",
        "6",
        4,
        "[dependencies]\nserde = \"=1.0.210\"\nserde_json = \"=1.0.154\"\n",
        "",
    );
    ok.write_stack_lock("");
    ok.generate_lockfile(&ok.repo);
    let out_ok = check_deps(&ok.repo, &[WorkspaceRoot::core(&ok.repo)]).expect("ok");
    assert!(
        out_ok.findings.is_empty(),
        "exact serde/serde_json in boundary crate must pass, got {:?}",
        out_ok.findings
    );

    // Bonus check: unadmitted registry crate in core fails `L-5.third-party`.
    let third = SynthEnv::new("third-party-unadmitted");
    third.write_workspace(&["crates/*"], &[]);
    third.add_reg_crate("some-random-crate", "0.3.0", "", false);
    third.write_crate(
        "crates/frob-dyn",
        "frob-dyn",
        "3",
        3,
        "[dependencies]\nsome-random-crate = \"=0.3.0\"\n",
        "",
    );
    third.write_stack_lock("");
    third.generate_lockfile(&third.repo);
    let out_tp = check_deps(&third.repo, &[WorkspaceRoot::core(&third.repo)]).expect("third-party");
    assert_eq!(rules_of(&out_tp.findings), vec![RULE_L5_THIRD_PARTY]);
    assert_eq!(out_tp.findings[0].subject, "frob-dyn -> some-random-crate");
}

#[test]
fn forbidden_names() {
    // One test case per forbidden family: tokio family, rayon family, BLAS/LAPACK family.
    let cases = [
        ("tokio-family", "tokio", "1.40.0", "L-5:tokio-family"),
        ("hyper-family", "hyper", "1.4.0", "L-5:tokio-family"),
        ("rayon-family", "rayon", "1.10.0", "L-5:direct-rayon"),
        ("blas-family", "blas-src", "0.10.0", "L-5:blas-lapack"),
        (
            "lapack-family",
            "nalgebra-lapack",
            "0.25.0",
            "L-5:blas-lapack",
        ),
    ];

    for (label, crate_name, ver, expected_reason) in cases {
        let env = SynthEnv::new(&format!("forbidden-{label}"));
        env.write_workspace(&["crates/*"], &[]);
        env.add_reg_crate(crate_name, ver, "", false);
        env.write_crate(
            "crates/frob-dyn",
            "frob-dyn",
            "3",
            3,
            &format!("[dependencies]\n{crate_name} = \"={ver}\"\n"),
            "",
        );
        env.write_stack_lock("");
        env.generate_lockfile(&env.repo);

        let outcome = check_deps(&env.repo, &[WorkspaceRoot::core(&env.repo)]).expect(label);
        assert_eq!(
            rules_of(&outcome.findings),
            vec![RULE_L5_FORBIDDEN],
            "case {label} failed: {:?}",
            outcome.findings
        );
        assert_eq!(outcome.findings[0].subject, format!("frob-dyn -> {crate_name}"));
        assert!(
            outcome.findings[0].message.contains(expected_reason),
            "expected reason {expected_reason} in {:?}",
            outcome.findings[0].message
        );
    }
}

#[test]
fn python_only_in_bindings() {
    let pyo3_foundation = format!(
        "[[foundation]]\n\
         id = \"FD-PYO3\"\n\
         name = \"pyo3\"\n\
         posture = \"consume\"\n\
         version = \"=0.22.0\"\n\
         confined_to = [\"bindings/frob-gymnasium\"]\n\
         admission_gate = \"INT-PYO3-01\"\n\
         receipt = \"{SAMPLE_RECEIPT}\"\n\
         fallback = \"synthetic fallback\"\n"
    );

    // 1. `pyo3` in `frob-learn` (outside bindings/frob-gymnasium) -> `L-5.python-outside-bindings`.
    let bad = SynthEnv::new("python-outside-bindings");
    bad.write_workspace(&["crates/*"], &[]);
    bad.add_reg_crate("pyo3", "0.22.0", "", false);
    bad.write_crate(
        "crates/frob-learn",
        "frob-learn",
        "4",
        5,
        "[dependencies]\npyo3 = { version = \"=0.22.0\", default-features = false }\n",
        "",
    );
    bad.write_stack_lock(&pyo3_foundation);
    bad.generate_lockfile(&bad.repo);
    let out_bad = check_deps(&bad.repo, &[WorkspaceRoot::core(&bad.repo)]).expect("bad py");
    assert_eq!(
        rules_of(&out_bad.findings),
        vec![
            RULE_L5_PYTHON_OUTSIDE_BINDINGS,
            xtask::check_deps::RULE_CORE_FORBIDDEN_PACKAGE,
        ]
    );
    assert_eq!(out_bad.findings[0].subject, "frob-learn -> pyo3");
    assert!(
        out_bad.findings[1].message.contains("annex = \"bindings\""),
        "expected annex = \"bindings\" in core.forbidden-package finding: {:?}",
        out_bad.findings[1]
    );

    // 2. `pyo3` inside `bindings/frob-gymnasium` (annex workspace) -> passes.
    let ok = SynthEnv::new("python-inside-bindings");
    ok.write_workspace(&["crates/*"], &["bindings"]);
    ok.write_crate("crates/frob-core", "frob-core", "0", 1, "", "");
    ok.add_reg_crate("pyo3", "0.22.0", "", false);
    ok.write_crate(
        "bindings/frob-gymnasium",
        "frob-gymnasium",
        "\"annex\"",
        4,
        "[workspace]\n\n[dependencies]\npyo3 = { version = \"=0.22.0\", default-features = false }\n",
        "",
    );
    ok.write_stack_lock(&pyo3_foundation);
    ok.generate_lockfile(&ok.repo);
    ok.generate_lockfile(&ok.repo.join("bindings/frob-gymnasium"));

    let roots = vec![
        WorkspaceRoot::core(&ok.repo),
        WorkspaceRoot::annex(
            "bindings-gym",
            "bindings/frob-gymnasium",
            ok.repo.join("bindings/frob-gymnasium"),
        ),
    ];
    let out_ok = check_deps(&ok.repo, &roots).expect("ok py");
    assert!(
        out_ok.findings.is_empty(),
        "pyo3 inside bindings/frob-gymnasium must pass, got {:?}",
        out_ok.findings
    );
}

#[test]
fn links_key_in_core_fails() {
    let foo_foundation = format!(
        "[[foundation]]\n\
         id = \"FD-FOO\"\n\
         name = \"foo-native\"\n\
         posture = \"consume-path\"\n\
         path = \"../foo-native\"\n\
         git_rev = \"{SAMPLE_GIT_REV}\"\n\
         confined_to = [\"frob-core\", \"spark/frob-spark-cuda\"]\n\
         admission_gate = \"INT-FOO-01\"\n\
         receipt = \"{SAMPLE_RECEIPT}\"\n\
         fallback = \"synthetic fallback\"\n"
    );

    // 1. Core crate depending on a synthetic package with `links = "foo"` -> `L-5.forbidden`.
    let core_links = SynthEnv::new("links-key-core");
    core_links.write_workspace(&["crates/*"], &[]);
    core_links.write_sibling_crate("foo-native", "foo-native", "0.1.0", "links = \"foo\"\n", true);
    core_links.write_crate(
        "crates/frob-core",
        "frob-core",
        "0",
        1,
        "[dependencies]\nfoo-native = { path = \"../../../foo-native\", default-features = false }\n",
        "",
    );
    core_links.write_stack_lock(&foo_foundation);
    core_links.generate_lockfile(&core_links.repo);
    let out_core =
        check_deps(&core_links.repo, &[WorkspaceRoot::core(&core_links.repo)]).expect("core links");
    assert_eq!(rules_of(&out_core.findings), vec![RULE_L5_FORBIDDEN]);
    assert_eq!(out_core.findings[0].subject, "frob-core -> foo-native");
    assert!(out_core.findings[0].message.contains("links = \"foo\""));

    // 2. Core crate depending on a synthetic package with build-dependency `cc` -> `L-5.forbidden`.
    let core_cc = SynthEnv::new("cc-build-dep-core");
    core_cc.write_workspace(&["crates/*"], &[]);
    core_cc.add_reg_crate("cc", "1.0.0", "", false);
    core_cc.write_sibling_crate(
        "foo-native",
        "foo-native",
        "0.1.0",
        "[build-dependencies]\ncc = \"=1.0.0\"\n",
        true,
    );
    core_cc.write_crate(
        "crates/frob-core",
        "frob-core",
        "0",
        1,
        "[dependencies]\nfoo-native = { path = \"../../../foo-native\", default-features = false }\n",
        "",
    );
    core_cc.write_stack_lock(&foo_foundation);
    core_cc.generate_lockfile(&core_cc.repo);
    let out_cc =
        check_deps(&core_cc.repo, &[WorkspaceRoot::core(&core_cc.repo)]).expect("core cc");
    assert_eq!(rules_of(&out_cc.findings), vec![RULE_L5_FORBIDDEN]);
    assert!(out_cc.findings[0].message.contains("build-depends on `cc`"));

    // 3. Same `links = "foo"` dependency in an annex workspace -> no finding.
    let annex_links = SynthEnv::new("links-key-annex");
    annex_links.write_workspace(&["crates/*"], &["spark"]);
    annex_links.write_crate("crates/frob-core", "frob-core", "0", 1, "", "");
    annex_links.write_sibling_crate("foo-native", "foo-native", "0.1.0", "links = \"foo\"\n", true);
    annex_links.write_crate(
        "spark/frob-spark-cuda",
        "frob-spark-cuda",
        "\"annex\"",
        1,
        "[workspace]\n\n[dependencies]\nfoo-native = { path = \"../../../foo-native\", default-features = false }\n",
        "",
    );
    annex_links.write_stack_lock(&foo_foundation);
    annex_links.generate_lockfile(&annex_links.repo);
    annex_links.generate_lockfile(&annex_links.repo.join("spark/frob-spark-cuda"));

    let roots = vec![
        WorkspaceRoot::core(&annex_links.repo),
        WorkspaceRoot::annex(
            "spark-cuda",
            "spark/frob-spark-cuda",
            annex_links.repo.join("spark/frob-spark-cuda"),
        ),
    ];
    let out_annex = check_deps(&annex_links.repo, &roots).expect("annex links");
    assert!(
        out_annex.findings.is_empty(),
        "links = \"foo\" inside an annex workspace must produce 0 findings, got {:?}",
        out_annex.findings
    );
}

// ============================================================================
// Step 5: L-9 (frob-cert std-only auditor and dependent allow-list)
// ============================================================================

#[test]
fn cert_dependency_fails() {
    for (label, section) in [
        ("normal", "[dependencies]"),
        ("dev", "[dev-dependencies]"),
        ("build", "[build-dependencies]"),
    ] {
        let env = SynthEnv::new(&format!("cert-dep-{label}"));
        env.write_workspace(&["crates/*"], &[]);
        env.write_crate("crates/frob-core", "frob-core", "0", 1, "", "");
        env.write_crate(
            "crates/frob-cert",
            "frob-cert",
            "6",
            1,
            &format!("{section}\nfrob-core = {{ path = \"../frob-core\" }}\n"),
            "",
        );
        env.write_stack_lock("");
        env.generate_lockfile(&env.repo);

        let outcome = check_deps(&env.repo, &[WorkspaceRoot::core(&env.repo)]).expect(label);
        assert_eq!(
            rules_of(&outcome.findings),
            vec![RULE_L9_CERT_HAS_DEPS],
            "frob-cert {label} dep must fail L-9.cert-has-deps"
        );
        assert_eq!(outcome.findings[0].subject, "frob-cert -> frob-core");
        assert_eq!(outcome.findings[0].path, "crates/frob-cert/Cargo.toml");
    }
}

#[test]
fn cert_dependent_allow_list() {
    // 1. All three allowed dependents (`frob-verbs`, `frob-cli`, `frob-conformance`) pass.
    let ok = SynthEnv::new("cert-dependents-ok");
    ok.write_workspace(&["crates/*"], &[]);
    ok.write_crate("crates/frob-cert", "frob-cert", "6", 1, "", "");
    ok.write_crate(
        "crates/frob-verbs",
        "frob-verbs",
        "6",
        4,
        "[dependencies]\nfrob-cert = { path = \"../frob-cert\" }\n",
        "",
    );
    ok.write_crate(
        "crates/frob-cli",
        "frob-cli",
        "7",
        2,
        "[dependencies]\nfrob-cert = { path = \"../frob-cert\" }\n",
        "",
    );
    ok.write_crate(
        "crates/frob-conformance",
        "frob-conformance",
        "7",
        4,
        "[dependencies]\nfrob-cert = { path = \"../frob-cert\" }\n",
        "",
    );
    ok.write_stack_lock("");
    ok.generate_lockfile(&ok.repo);

    let out_ok = check_deps(&ok.repo, &[WorkspaceRoot::core(&ok.repo)]).expect("ok");
    assert!(
        out_ok.findings.is_empty(),
        "allowed frob-cert dependents must pass, got {:?}",
        out_ok.findings
    );

    // 2. `frob-lemma` (layer 6.3) -> `frob-cert` (layer 6.1) passes layers/ranks, fails `L-9.cert-dependent`.
    let bad = SynthEnv::new("cert-dependent-lemma");
    bad.write_workspace(&["crates/*"], &[]);
    bad.write_crate("crates/frob-cert", "frob-cert", "6", 1, "", "");
    bad.write_crate(
        "crates/frob-lemma",
        "frob-lemma",
        "6",
        3,
        "[dependencies]\nfrob-cert = { path = \"../frob-cert\" }\n",
        "",
    );
    bad.write_stack_lock("");
    bad.generate_lockfile(&bad.repo);

    let roots = vec![WorkspaceRoot::core(&bad.repo)];
    let layer_out = check_layers(&bad.repo, &roots).expect("check_layers");
    assert!(
        layer_out.findings.is_empty(),
        "ranks alone permit frob-lemma -> frob-cert: {:?}",
        layer_out.findings
    );
    let deps_out = check_deps(&bad.repo, &roots).expect("check_deps");
    assert_eq!(rules_of(&deps_out.findings), vec![RULE_L9_CERT_DEPENDENT]);
    assert_eq!(deps_out.findings[0].subject, "frob-lemma -> frob-cert");
    assert_eq!(deps_out.findings[0].path, "crates/frob-lemma/Cargo.toml");
}

// ============================================================================
// Step 6 & Ownership Partition: `check-layers` vs `check-deps` + `edge.forbidden`
// ============================================================================

#[test]
fn layer_and_dep_plants_owned_once() {
    // 1. Upward edge (`frob-lie` -> `frob-design`): check-layers reports `L-1.upward`, check-deps is clean.
    let upward = SynthEnv::new("owned-once-upward");
    upward.write_workspace(&["crates/*"], &[]);
    upward.write_crate(
        "crates/frob-lie",
        "frob-lie",
        "1",
        1,
        "[dependencies]\nfrob-design = { path = \"../frob-design\" }\n",
        "",
    );
    upward.write_crate("crates/frob-design", "frob-design", "2", 2, "", "");
    upward.write_stack_lock("");
    upward.generate_lockfile(&upward.repo);
    let up_roots = vec![WorkspaceRoot::core(&upward.repo)];
    assert_eq!(
        rules_of(&check_layers(&upward.repo, &up_roots).expect("layers").findings),
        vec![RULE_L1_UPWARD]
    );
    assert!(
        check_deps(&upward.repo, &up_roots)
            .expect("deps")
            .findings
            .is_empty()
    );

    // 2. Intra-layer edge against rank (`frob-arena` -> `frob-opt`): check-layers reports `L-1.rank`, check-deps is clean.
    let rank = SynthEnv::new("owned-once-rank");
    rank.write_workspace(&["crates/*"], &[]);
    rank.write_crate(
        "crates/frob-arena",
        "frob-arena",
        "4",
        6,
        "[dependencies]\nfrob-opt = { path = \"../frob-opt\" }\n",
        "",
    );
    rank.write_crate("crates/frob-opt", "frob-opt", "4", 7, "", "");
    rank.write_stack_lock("");
    rank.generate_lockfile(&rank.repo);
    let rk_roots = vec![WorkspaceRoot::core(&rank.repo)];
    assert_eq!(
        rules_of(&check_layers(&rank.repo, &rk_roots).expect("layers").findings),
        vec![RULE_L1_RANK]
    );
    assert!(
        check_deps(&rank.repo, &rk_roots)
            .expect("deps")
            .findings
            .is_empty()
    );

    // 3. Forbidden dependency (`tokio` in `frob-dyn`): check-deps reports `L-5.forbidden` (exit 2), check-layers is clean.
    let tokio_env = SynthEnv::new("owned-once-tokio");
    tokio_env.write_workspace(&["crates/*"], &[]);
    tokio_env.add_reg_crate("tokio", "1.40.0", "", false);
    tokio_env.write_crate(
        "crates/frob-dyn",
        "frob-dyn",
        "3",
        3,
        "[dependencies]\ntokio = \"=1.40.0\"\n",
        "",
    );
    tokio_env.write_stack_lock("");
    tokio_env.generate_lockfile(&tokio_env.repo);
    let tk_roots = vec![WorkspaceRoot::core(&tokio_env.repo)];
    assert!(
        check_layers(&tokio_env.repo, &tk_roots)
            .expect("layers")
            .findings
            .is_empty()
    );
    assert_eq!(
        rules_of(&check_deps(&tokio_env.repo, &tk_roots).expect("deps").findings),
        vec![RULE_L5_FORBIDDEN]
    );

    // Verify CLI exit code 2 and valid report on the tokio seed done-condition plant.
    let mut out = Vec::new();
    let mut err = Vec::new();
    let args = vec![
        "check-deps".to_owned(),
        "--root".to_owned(),
        tokio_env.repo.to_string_lossy().into_owned(),
        "--json".to_owned(),
    ];
    let code = cli::run(TEST_COMMANDS, &args, &mut out, &mut err);
    assert_eq!(code, ExitCode::from(EXIT_FINDINGS));
    assert!(err.is_empty(), "unexpected stderr: {}", String::from_utf8_lossy(&err));
    validate_report_json(&out).expect("valid report JSON");
}

#[test]
fn proof_boundary_edge_owned_once() {
    // 1. Under real CRATE_MAP (`frob-arena` layer 4.6 -> `frob-nemesis` layer 5.3):
    //    `check-layers` reports `L-1.upward`, `check-deps` stays silent (0 findings).
    let real_env = SynthEnv::new("proof-boundary-real-map");
    real_env.write_workspace(&["crates/*"], &[]);
    real_env.write_crate(
        "crates/frob-arena",
        "frob-arena",
        "4",
        6,
        "[dependencies]\nfrob-nemesis = { path = \"../frob-nemesis\" }\n",
        "",
    );
    real_env.write_crate("crates/frob-nemesis", "frob-nemesis", "5", 3, "", "");
    real_env.write_stack_lock("");
    real_env.generate_lockfile(&real_env.repo);

    let roots = vec![WorkspaceRoot::core(&real_env.repo)];
    assert_eq!(
        rules_of(&check_layers(&real_env.repo, &roots).expect("layers").findings),
        vec![RULE_L1_UPWARD]
    );
    assert!(
        check_deps(&real_env.repo, &roots)
            .expect("deps")
            .findings
            .is_empty(),
        "check-deps must stay silent while check-layers forbids frob-arena -> frob-nemesis"
    );

    // 2. Under a synthetic map that ranks `frob-nemesis` below `frob-arena` (layer 4.1 < 4.6):
    //    `check-layers` reports nothing, `check-deps` reports `edge.forbidden`.
    const SYNTH_MAP: &[CrateRow] = &[
        CrateRow {
            name: "frob-nemesis",
            dir: "crates/frob-nemesis",
            layer: Layer::Core(4),
            rank: 1,
            tiers: &[Tier::Solid],
            chapters: &["§26"],
            cap_kloc: 10,
            reason: Reason::Original,
            adr: None,
            conditional: None,
        },
        CrateRow {
            name: "frob-arena",
            dir: "crates/frob-arena",
            layer: Layer::Core(4),
            rank: 6,
            tiers: &[Tier::Solid],
            chapters: &["§24"],
            cap_kloc: 14,
            reason: Reason::Original,
            adr: None,
            conditional: None,
        },
    ];
    let synth_env = SynthEnv::new("proof-boundary-synth-map");
    synth_env.write_workspace(&["crates/*"], &[]);
    synth_env.write_crate(
        "crates/frob-arena",
        "frob-arena",
        "4",
        6,
        "[dependencies]\nfrob-nemesis = { path = \"../frob-nemesis\" }\n",
        "",
    );
    synth_env.write_crate("crates/frob-nemesis", "frob-nemesis", "4", 1, "", "");
    synth_env.write_stack_lock("");
    synth_env.generate_lockfile(&synth_env.repo);

    let s_roots = vec![WorkspaceRoot::core(&synth_env.repo)];
    let s_layers = check_layers_with_tables(
        &synth_env.repo,
        &s_roots,
        SYNTH_MAP,
        TOMBSTONES,
        INVERSIONS,
    )
    .expect("synth layers");
    assert!(
        s_layers.findings.is_empty(),
        "synthetic map permits frob-arena -> frob-nemesis by rank: {:?}",
        s_layers.findings
    );

    let stack_lock =
        lock::load(&synth_env.repo.join("franken-stack.lock")).expect("load synth stack lock");
    let s_deps = check_deps_with_config(
        &synth_env.repo,
        &s_roots,
        &stack_lock,
        SYNTH_MAP,
        PROOF_BOUNDARY_EDGES,
    )
    .expect("synth deps");
    assert_eq!(rules_of(&s_deps.findings), vec![RULE_EDGE_FORBIDDEN]);
    assert_eq!(s_deps.findings[0].subject, "frob-arena -> frob-nemesis");
}

// ============================================================================
// Step 7: Closure facts, exit codes, canonical stability, rule disjointness, clean tree
// ============================================================================

#[test]
fn closure_facts_reported() {
    let env = SynthEnv::new("closure-facts-rayon");
    env.write_workspace(&["crates/*"], &[]);
    env.add_reg_crate("rayon", "1.10.0", "", false);
    env.write_sibling_crate(
        "frankensim/crates/fs-mbd",
        "fs-mbd",
        "0.1.0",
        "[dependencies]\nrayon = \"=1.10.0\"\n",
        false,
    );
    env.write_crate(
        "crates/frob-fsim",
        "frob-fsim",
        "0",
        2,
        "[dependencies]\nfs-mbd = { path = \"../../../frankensim/crates/fs-mbd\", default-features = false }\n",
        "",
    );
    env.write_stack_lock("");
    env.generate_lockfile(&env.repo);

    let mut out = Vec::new();
    let mut err = Vec::new();
    let args = vec![
        "check-deps".to_owned(),
        "--root".to_owned(),
        env.repo.to_string_lossy().into_owned(),
        "--json".to_owned(),
    ];
    let code = cli::run(TEST_COMMANDS, &args, &mut out, &mut err);
    assert_eq!(
        code,
        ExitCode::from(EXIT_OK),
        "transitive rayon through valid path dep must raise 0 findings; out={}, err={}",
        String::from_utf8_lossy(&out),
        String::from_utf8_lossy(&err)
    );
    validate_report_json(&out).expect("valid frob.xtask.report/1");

    let admitted = admit(&out, &Limits::default()).expect("admit report JSON");
    let core_facts = admitted
        .value()
        .get("facts")
        .and_then(|v| v.get("workspaces"))
        .and_then(|v| v.get("core"))
        .expect("facts.workspaces.core");
    assert_eq!(
        core_facts
            .get("packages_total")
            .and_then(|v| v.as_int53().ok()),
        Some(3)
    );
    assert_eq!(
        core_facts
            .get("registry_packages")
            .and_then(|v| v.as_int53().ok()),
        Some(1)
    );
    let hits = core_facts
        .get("transitive_forbidden_hits")
        .and_then(|v| v.as_array())
        .expect("transitive_forbidden_hits array");
    assert_eq!(hits.len(), 1, "expected 1 transitive forbidden hit");
    assert_eq!(hits[0].get("package").and_then(|v| v.as_str()), Some("rayon"));
    assert_eq!(
        hits[0].get("reason").and_then(|v| v.as_str()),
        Some("L-5:direct-rayon")
    );
    let path_elems: Vec<&str> = hits[0]
        .get("path")
        .and_then(|v| v.as_array())
        .expect("path array")
        .iter()
        .filter_map(|v| v.as_str())
        .collect();
    assert_eq!(path_elems, vec!["frob-fsim", "fs-mbd", "rayon"]);
}

#[test]
fn deps_exit_codes() {
    // 1. Clean synthetic tree -> exit 0.
    let clean = SynthEnv::new("exit-code-clean");
    clean.write_workspace(&["crates/*"], &[]);
    clean.write_crate("crates/frob-core", "frob-core", "0", 1, "", "");
    clean.write_stack_lock("");
    clean.generate_lockfile(&clean.repo);

    let mut out = Vec::new();
    let mut err = Vec::new();
    let args = vec![
        "check-deps".to_owned(),
        "--root".to_owned(),
        clean.repo.to_string_lossy().into_owned(),
        "--json".to_owned(),
    ];
    let code = cli::run(TEST_COMMANDS, &args, &mut out, &mut err);
    assert_eq!(code, ExitCode::from(EXIT_OK));
    assert!(err.is_empty());
    validate_report_json(&out).expect("clean report valid");

    // 2. Planted finding -> exit 2.
    let bad = SynthEnv::new("exit-code-finding");
    bad.write_workspace(&["crates/*"], &[]);
    bad.add_reg_crate("rayon", "1.10.0", "", false);
    bad.write_crate(
        "crates/frob-dyn",
        "frob-dyn",
        "3",
        3,
        "[dependencies]\nrayon = \"=1.10.0\"\n",
        "",
    );
    bad.write_stack_lock("");
    bad.generate_lockfile(&bad.repo);

    let mut out2 = Vec::new();
    let mut err2 = Vec::new();
    let args2 = vec![
        "check-deps".to_owned(),
        "--root".to_owned(),
        bad.repo.to_string_lossy().into_owned(),
        "--json".to_owned(),
    ];
    let code2 = cli::run(TEST_COMMANDS, &args2, &mut out2, &mut err2);
    assert_eq!(code2, ExitCode::from(EXIT_FINDINGS));
    assert!(err2.is_empty());
    validate_report_json(&out2).expect("finding report valid");

    // 3. Offline resolution needing the network -> exit 3 with exact repair hint.
    let offline = SynthEnv::new("exit-code-offline-network");
    offline.write_workspace(&["crates/*"], &[]);
    offline.write_crate(
        "crates/frob-dyn",
        "frob-dyn",
        "3",
        3,
        "[dependencies]\nfrob-nonexistent-offline-pkg-bd003 = \"=0.1.0\"\n",
        "",
    );
    offline.write_stack_lock("");
    // Deliberately write NO local-reg replacement; lockfile points at crates.io so
    // `cargo metadata --no-deps --locked --offline` passes and resolved `--locked --offline` fails needing network.
    write_file(
        &offline.repo.join("Cargo.lock"),
        "# This file is automatically @generated by Cargo.\n\
         # It is not intended for manual editing.\n\
         version = 4\n\n\
         [[package]]\n\
         name = \"frob-dyn\"\n\
         version = \"0.0.1\"\n\
         dependencies = [\n\
          \"frob-nonexistent-offline-pkg-bd003\",\n\
         ]\n\n\
         [[package]]\n\
         name = \"frob-nonexistent-offline-pkg-bd003\"\n\
         version = \"0.1.0\"\n\
         source = \"registry+https://github.com/rust-lang/crates.io-index\"\n",
    );

    let mut out3 = Vec::new();
    let mut err3 = Vec::new();
    let args3 = vec![
        "check-deps".to_owned(),
        "--root".to_owned(),
        offline.repo.to_string_lossy().into_owned(),
        "--json".to_owned(),
    ];
    let code3 = cli::run(TEST_COMMANDS, &args3, &mut out3, &mut err3);
    assert_eq!(
        code3,
        ExitCode::from(EXIT_USAGE),
        "offline cache miss must exit 3; stderr={}",
        String::from_utf8_lossy(&err3)
    );
    assert!(out3.is_empty());
    let err3_str = String::from_utf8(err3).expect("utf-8 stderr");
    assert!(
        err3_str.contains(OFFLINE_FETCH_REPAIR),
        "expected repair hint `{OFFLINE_FETCH_REPAIR}` in stderr: {err3_str}"
    );
}

#[test]
fn deps_report_is_sorted_and_stable() {
    let env = SynthEnv::new("report-sorted-stable");
    env.write_workspace(&["crates/*"], &[]);
    env.add_reg_crate("tokio", "1.40.0", "", false);
    env.add_reg_crate("rayon", "1.10.0", "", false);
    env.add_reg_crate("serde", "1.0.210", "", false);
    env.write_sibling_crate("frankensim/crates/fs-mbd", "fs-mbd", "0.1.0", "", false);
    env.write_crate(
        "crates/frob-dyn",
        "frob-dyn",
        "3",
        3,
        "[dependencies]\n\
         tokio = \"=1.40.0\"\n\
         rayon = \"=1.10.0\"\n\
         serde = \"=1.0.210\"\n\
         fs-mbd = { path = \"../../../frankensim/crates/fs-mbd\", default-features = false }\n",
        "",
    );
    env.write_stack_lock("");
    env.generate_lockfile(&env.repo);

    let roots = vec![WorkspaceRoot::core(&env.repo)];
    let run1 = check_deps(&env.repo, &roots).expect("run 1");
    let run2 = check_deps(&env.repo, &roots).expect("run 2");
    assert_eq!(run1, run2);

    let mut sorted = run1.findings.clone();
    sorted.sort();
    assert_eq!(run1.findings, sorted, "findings must already be sorted");

    let r1 = CheckReport::new("check-deps", "<root>", run1.findings, run1.facts, 0)
        .to_canonical_bytes()
        .expect("canonical bytes 1");
    let r2 = CheckReport::new("check-deps", "<root>", run2.findings, run2.facts, 0)
        .to_canonical_bytes()
        .expect("canonical bytes 2");
    assert_eq!(r1.as_bytes(), r2.as_bytes());
    validate_report_json(r1.as_bytes()).expect("valid canonical report");
}

#[test]
fn owned_rules_are_disjoint_from_check_layers() {
    let layer_rules: BTreeSet<&str> = check_layers::OWNED_RULES.iter().copied().collect();
    let dep_rules: BTreeSet<&str> = check_deps::OWNED_RULES.iter().copied().collect();
    let overlap: Vec<&str> = layer_rules.intersection(&dep_rules).copied().collect();
    assert!(
        overlap.is_empty(),
        "check-layers and check-deps must own disjoint rule sets; overlap={overlap:?}"
    );
}

#[test]
fn clean_tree_passes_check_deps() {
    let root = repo_root();

    // Acceptance: xtask's only dependencies are `frob-core` and `fs-blake3`.
    let declared = load_declared_metadata(&root, &WorkspaceRoot::core(&root))
        .expect("load real workspace declared metadata");
    let xtask_pkg = declared
        .members
        .iter()
        .find(|m| m.name == "xtask")
        .expect("xtask member in core workspace");
    let mut xtask_deps: Vec<&str> = xtask_pkg
        .dependencies
        .iter()
        .map(|d| d.name.as_str())
        .collect();
    xtask_deps.sort();
    assert_eq!(
        xtask_deps,
        vec!["frob-core", "fs-blake3"],
        "xtask's only dependencies must be frob-core and fs-blake3"
    );

    // Acceptance: the clean tree passes `check-deps` with exit 0, zero findings,
    // and valid `frob.xtask.report/1` JSON.
    let mut out = Vec::new();
    let mut err = Vec::new();
    let args = vec![
        "check-deps".to_owned(),
        "--root".to_owned(),
        root.to_string_lossy().into_owned(),
        "--json".to_owned(),
    ];
    let code = cli::run(TEST_COMMANDS, &args, &mut out, &mut err);
    assert_eq!(
        code,
        ExitCode::from(EXIT_OK),
        "clean tree must pass check-deps with exit 0; stdout={}, stderr={}",
        String::from_utf8_lossy(&out),
        String::from_utf8_lossy(&err)
    );
    assert!(
        err.is_empty(),
        "clean tree stderr must be empty, got: {}",
        String::from_utf8_lossy(&err)
    );
    validate_report_json(&out).expect("clean tree report validates against frob.xtask.report/1");

    let admitted = admit(&out, &Limits::default()).expect("admit clean report");
    assert_eq!(
        admitted.value().get("schema").and_then(|v| v.as_str()),
        Some(REPORT_SCHEMA)
    );
    assert_eq!(
        admitted.value().get("check").and_then(|v| v.as_str()),
        Some("check-deps")
    );
    assert_eq!(
        admitted.value().get("ok").and_then(|v| v.as_bool()),
        Some(true)
    );
}
