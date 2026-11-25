//! Suite deny-rule and posture-register unit/integration tests for
//! `xtask check-deps` (bead N33c-09).
//!
//! Synthetic scratch workspaces are created under
//! `${BIG_SCRATCH:-${TMPDIR:-/tmp}}/frob-BronzePelican/` and never deleted (COMMON.md 9b).
//! Every workspace check runs real `cargo metadata --locked --offline` and
//! `cargo metadata --all-features --locked --offline` against offline local directory
//! sources and sibling directories.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use xtask::check_deps::{
    self, CargoPolicy, NEVER_DEPENDENCY_SIBLINGS, RULE_ASUP_V0_5_0, RULE_FTUI_EXECUTOR_FEATURE,
    RULE_L4_FOUNDATION_CONFINED, RULE_SUITE_EGG, RULE_SUITE_FORBIDDEN_PACKAGE, SIBLINGS_REL_PATH,
    check_deps, discover_workspace_roots, load_siblings_register, parse_siblings_register_bytes,
    validate_siblings_against_stack_lock,
};
use xtask::cli::{self, CheckFn, EXIT_FINDINGS, EXIT_OK};
use xtask::metadata::{WorkspaceRoot, normalize_rel_path};
use xtask::report::{Finding, validate_report_json};
use xtask::stack::lock;
use xtask::stack::posture::Posture;

static COUNTER: AtomicU64 = AtomicU64::new(0);

const SAMPLE_RECEIPT: &str = "d3b0885b1d5bfbe09039a84ae190c40fce4ad3b0b81b5c28bf946201987462d4";
const SAMPLE_GIT_REV: &str = "1111111111111111111111111111111111111111";

const SUITE_COMMANDS: &[(&str, CheckFn)] = &[("check-deps", check_deps::run_command)];

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
        .join("frob-BronzePelican")
        .join(format!("xtask-check-deps-suite-{label}-{ts}-{pid}-{n}"));
    fs::create_dir_all(&dir).expect("create scratch dir");
    dir
}

fn write_file(path: &Path, content: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("create parent dirs");
    }
    fs::write(path, content).expect("write fixture file");
}

fn rules_of(findings: &[Finding]) -> Vec<&'static str> {
    findings.iter().map(|f| f.rule).collect()
}

/// Offline synthetic multi-workspace + sibling harness for `check-deps` suite rules.
struct SuiteEnv {
    base: PathBuf,
    repo: PathBuf,
    reg: PathBuf,
}

impl SuiteEnv {
    fn new(label: &str) -> Self {
        let base = make_scratch_dir(label);
        let repo = base.join("repo");
        let reg = base.join("local-reg");
        fs::create_dir_all(&repo).expect("create repo dir");
        fs::create_dir_all(&reg).expect("create local-reg dir");
        Self { base, repo, reg }
    }

    fn write_workspace(&self, members: &[&str], exclude: &[&str]) {
        let m: Vec<String> = members.iter().map(|s| format!("\"{s}\"")).collect();
        let mut toml = format!(
            "# synthetic workspace for N33c-09\n[workspace]\nresolver = \"3\"\nmembers = [{}]\n",
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
    ) {
        let dir = self.repo.join(rel_dir);
        let meta = if layer_toml.is_empty() {
            String::new()
        } else {
            format!("[package.metadata.frob]\nlayer = {layer_toml}\nrank = {rank}\n")
        };
        let manifest = format!(
            "# synthetic crate for N33c-09\n[package]\nname = \"{name}\"\nversion = \"0.0.1\"\nedition = \"2024\"\n\n{meta}\n{extra_toml}\n"
        );
        write_file(&dir.join("Cargo.toml"), &manifest);
        write_file(&dir.join("src/lib.rs"), "// synthetic crate\n");
    }

    fn write_sibling_crate(&self, sib_rel_dir: &str, name: &str, version: &str, extra_toml: &str) {
        let dir = self.base.join(sib_rel_dir);
        let manifest = format!(
            "# synthetic sibling crate for N33c-09\n[package]\nname = \"{name}\"\nversion = \"{version}\"\nedition = \"2021\"\n\n[workspace]\n\n{extra_toml}\n"
        );
        write_file(&dir.join("Cargo.toml"), &manifest);
        write_file(&dir.join("src/lib.rs"), "// synthetic sibling crate\n");
    }

    fn add_reg_crate(&self, name: &str, version: &str, extra_toml: &str) {
        let dir = self.reg.join(format!("{name}-{version}"));
        let manifest = format!(
            "# synthetic registry crate for N33c-09\n[package]\nname = \"{name}\"\nversion = \"{version}\"\nedition = \"2021\"\n\n{extra_toml}\n"
        );
        write_file(&dir.join("Cargo.toml"), &manifest);
        write_file(&dir.join("src/lib.rs"), "// synthetic registry crate\n");
        write_file(&dir.join(".cargo-checksum.json"), "{\"files\":{}}\n");
    }

    fn write_cargo_config(&self) {
        let reg_str = normalize_rel_path(&self.reg);
        let cfg = format!(
            "# offline local directory source for N33c-09\n\
             [source.crates-io]\n\
             replace-with = \"local-reg\"\n\n\
             [source.local-reg]\n\
             directory = \"{reg_str}\"\n"
        );
        write_file(&self.base.join(".cargo/config.toml"), &cfg);
    }

    fn copy_real_stack_lock(&self) {
        let src = fs::read_to_string(repo_root().join("franken-stack.lock"))
            .expect("read real franken-stack.lock");
        write_file(&self.repo.join("franken-stack.lock"), &src);
    }

    fn copy_real_siblings_register(&self) {
        let src = fs::read_to_string(repo_root().join(SIBLINGS_REL_PATH))
            .expect("read real registries/siblings.toml");
        write_file(&self.repo.join(SIBLINGS_REL_PATH), &src);
    }

    fn write_stack_lock_raw(&self, toml_src: &str) {
        write_file(&self.repo.join("franken-stack.lock"), toml_src);
    }

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

// ============================================================================
// 1. asup.v0_5_0 (resolved path, unique 0.5.0, unlinked Cargo.lock)
// ============================================================================

#[test]
fn denies_asupersync_0_5_0() {
    // Plant 1: a path dependency (`fs-ga`, bedrock-direct) that transitively resolves `asupersync 0.5.0`.
    let env = SuiteEnv::new("asup-0-5-0-path");
    env.write_workspace(&["crates/*"], &[]);
    env.add_reg_crate("asupersync", "0.5.0", "");
    env.write_sibling_crate(
        "frankensim/crates/fs-ga",
        "fs-ga",
        "0.1.0",
        "[dependencies]\nasupersync = \"=0.5.0\"\n",
    );
    env.write_crate(
        "crates/frob-lie",
        "frob-lie",
        "1",
        1,
        "[dependencies]\nfs-ga = { path = \"../../../frankensim/crates/fs-ga\", default-features = false }\n",
    );
    env.copy_real_stack_lock();
    env.copy_real_siblings_register();
    env.generate_lockfile(&env.repo);

    let out = check_deps(&env.repo, &[WorkspaceRoot::core(&env.repo)]).expect("check_deps");
    assert_eq!(rules_of(&out.findings), vec![RULE_ASUP_V0_5_0]);
    assert!(
        out.findings[0].message.contains("frob-lie -> fs-ga -> asupersync"),
        "expected full dependency path in message: {}",
        out.findings[0].message
    );
    assert_eq!(out.findings[0].path, "crates/frob-lie/Cargo.toml");

    // DESIGN: whole 0.5.* line (e.g. 0.5.3) is also caught by `asup.v0_5_0`.
    let patch_env = SuiteEnv::new("asup-0-5-3-line");
    patch_env.write_workspace(&["crates/*"], &[]);
    patch_env.add_reg_crate("asupersync", "0.5.3", "");
    patch_env.write_sibling_crate(
        "frankensim/crates/fs-ga",
        "fs-ga",
        "0.1.0",
        "[dependencies]\nasupersync = \"=0.5.3\"\n",
    );
    patch_env.write_crate(
        "crates/frob-lie",
        "frob-lie",
        "1",
        1,
        "[dependencies]\nfs-ga = { path = \"../../../frankensim/crates/fs-ga\", default-features = false }\n",
    );
    patch_env.copy_real_stack_lock();
    patch_env.generate_lockfile(&patch_env.repo);

    let patch_out =
        check_deps(&patch_env.repo, &[WorkspaceRoot::core(&patch_env.repo)]).expect("check_deps");
    assert_eq!(rules_of(&patch_out.findings), vec![RULE_ASUP_V0_5_0]);
    assert!(patch_out.findings[0].message.contains("0.5.3"));
}

#[test]
fn denies_asupersync_0_5_0_when_unique() {
    // Workspace whose ONLY asupersync package is 0.5.0 (no duplicate, even if scratch lock pins =0.5.0).
    let env = SuiteEnv::new("asup-0-5-0-unique");
    env.write_workspace(&["crates/*"], &[]);
    env.add_reg_crate("asupersync", "0.5.0", "");
    env.write_crate(
        "crates/frob-dyn",
        "frob-dyn",
        "3",
        3,
        "[dependencies]\nasupersync = { version = \"=0.5.0\", default-features = false }\n",
    );
    env.write_stack_lock_raw(&format!(
        "toolchain = \"nightly-2026-08-31\"\n\n\
         [[foundation]]\n\
         id = \"FD-ASUP\"\n\
         name = \"asupersync\"\n\
         posture = \"consume\"\n\
         version = \"=0.5.0\"\n\
         confined_to = [\"any\"]\n\
         admission_gate = \"INT-ASUP-01\"\n\
         receipt = \"{SAMPLE_RECEIPT}\"\n\
         fallback = \"synthetic\"\n"
    ));
    env.generate_lockfile(&env.repo);

    let out = check_deps(&env.repo, &[WorkspaceRoot::core(&env.repo)]).expect("check_deps");
    assert_eq!(rules_of(&out.findings), vec![RULE_ASUP_V0_5_0]);
    assert!(
        out.findings[0].message.contains("frob-dyn -> asupersync"),
        "unexpected message: {}",
        out.findings[0].message
    );
}

#[test]
fn denies_asupersync_0_5_0_in_lock_unlinked() {
    // Case A: `Cargo.lock` carries an unlinked `[[patch.unused]]` entry for `asupersync 0.5.0`
    // while no crate or binary links it.
    let unused_patch = SuiteEnv::new("asup-0-5-0-lock-patch-unused");
    unused_patch.write_workspace(&["crates/*"], &[]);
    unused_patch.write_crate("crates/frob-core", "frob-core", "0", 1, "");
    unused_patch.copy_real_stack_lock();
    unused_patch.generate_lockfile(&unused_patch.repo);
    let mut lock_txt =
        fs::read_to_string(unused_patch.repo.join("Cargo.lock")).expect("read generated Cargo.lock");
    lock_txt.push_str("\n[[patch.unused]]\nname = \"asupersync\"\nversion = \"0.5.0\"\n");
    write_file(&unused_patch.repo.join("Cargo.lock"), &lock_txt);

    let out_unused = check_deps(&unused_patch.repo, &[WorkspaceRoot::core(&unused_patch.repo)])
        .expect("check_deps with [[patch.unused]]");
    assert_eq!(rules_of(&out_unused.findings), vec![RULE_ASUP_V0_5_0]);
    assert_eq!(out_unused.findings[0].path, "Cargo.lock");

    // Case B: an orphaned `[[package]]` entry for `asupersync 0.5.0` in `Cargo.lock` that no
    // crate or binary links at all still fails with `asup.v0_5_0` (exit 2, not exit 3).
    let orphan = SuiteEnv::new("asup-0-5-0-lock-orphan-pkg");
    orphan.write_workspace(&["crates/*"], &[]);
    orphan.write_crate("crates/frob-core", "frob-core", "0", 1, "");
    orphan.copy_real_stack_lock();
    orphan.generate_lockfile(&orphan.repo);
    let mut orphan_lock =
        fs::read_to_string(orphan.repo.join("Cargo.lock")).expect("read generated Cargo.lock");
    orphan_lock.push_str(
        "\n[[package]]\nname = \"asupersync\"\nversion = \"0.5.0\"\nsource = \"registry+https://github.com/rust-lang/crates.io-index\"\n",
    );
    write_file(&orphan.repo.join("Cargo.lock"), &orphan_lock);

    let out_orphan = check_deps(&orphan.repo, &[WorkspaceRoot::core(&orphan.repo)])
        .expect("check_deps with orphan lock entry");
    assert_eq!(rules_of(&out_orphan.findings), vec![RULE_ASUP_V0_5_0]);
    assert_eq!(out_orphan.findings[0].path, "Cargo.lock");
    assert!(
        out_orphan.findings[0].message.contains("unlinked"),
        "expected unlinked note in message: {}",
        out_orphan.findings[0].message
    );

    // Case C: another committed lockfile under the repo (e.g. `kit/scorer/Cargo.lock`) containing
    // `asupersync 0.5.0` that no workspace binary links also fails `asup.v0_5_0`.
    let sub_lock = SuiteEnv::new("asup-0-5-0-sub-lock");
    sub_lock.write_workspace(&["crates/*"], &[]);
    sub_lock.write_crate("crates/frob-core", "frob-core", "0", 1, "");
    sub_lock.copy_real_stack_lock();
    sub_lock.generate_lockfile(&sub_lock.repo);
    write_file(
        &sub_lock.repo.join("kit/scorer/Cargo.lock"),
        "version = 4\n\n[[package]]\nname = \"asupersync\"\nversion = \"0.5.0\"\n",
    );

    let out_sub = check_deps(&sub_lock.repo, &[WorkspaceRoot::core(&sub_lock.repo)])
        .expect("check_deps with sub Cargo.lock");
    assert_eq!(rules_of(&out_sub.findings), vec![RULE_ASUP_V0_5_0]);
    assert_eq!(out_sub.findings[0].path, "kit/scorer/Cargo.lock");
}

// ============================================================================
// 2. ftui.executor-feature (resolved feature + frob-tui confinement)
// ============================================================================

#[test]
fn denies_ftui_executor_feature() {
    let ftui_manifest = "\
[features]
default = []
runtime = []
native-backend = []
asupersync-executor = []
";

    // Positive control: `frob-tui` with default-features = false, runtime + native-backend passes.
    let ok = SuiteEnv::new("ftui-clean-ok");
    ok.write_workspace(&["crates/*"], &[]);
    ok.add_reg_crate("ftui-runtime", "0.11.0", ftui_manifest);
    ok.write_crate(
        "crates/frob-tui",
        "frob-tui",
        "7",
        2,
        "[dependencies]\nftui-runtime = { version = \"=0.11.0\", default-features = false, features = [\"runtime\", \"native-backend\"] }\n",
    );
    ok.copy_real_stack_lock();
    ok.copy_real_siblings_register();
    ok.generate_lockfile(&ok.repo);

    let out_ok = check_deps(&ok.repo, &[WorkspaceRoot::core(&ok.repo)]).expect("clean ftui");
    assert!(
        out_ok.findings.is_empty(),
        "frob-tui without asupersync-executor must pass, got {:?}",
        out_ok.findings
    );

    // Plant 2: `frob-tui` enabling `ftui-runtime/asupersync-executor` fails `ftui.executor-feature`.
    let bad = SuiteEnv::new("ftui-executor-enabled");
    bad.write_workspace(&["crates/*"], &[]);
    bad.add_reg_crate("ftui-runtime", "0.11.0", ftui_manifest);
    bad.write_crate(
        "crates/frob-tui",
        "frob-tui",
        "7",
        2,
        "[dependencies]\nftui-runtime = { version = \"=0.11.0\", default-features = false, features = [\"runtime\", \"native-backend\", \"asupersync-executor\"] }\n",
    );
    bad.copy_real_stack_lock();
    bad.copy_real_siblings_register();
    bad.generate_lockfile(&bad.repo);

    let out_bad = check_deps(&bad.repo, &[WorkspaceRoot::core(&bad.repo)]).expect("bad ftui");
    assert_eq!(rules_of(&out_bad.findings), vec![RULE_FTUI_EXECUTOR_FEATURE]);
    assert!(out_bad.findings[0].message.contains("asupersync-executor"));
    assert!(out_bad.findings[0].message.contains("frob-tui -> ftui-runtime"));

    // Plant 2b: `ftui-runtime/asupersync-executor` hidden behind an optional feature is caught by `--all-features`.
    let opt_bad = SuiteEnv::new("ftui-executor-optional-feat");
    opt_bad.write_workspace(&["crates/*"], &[]);
    opt_bad.add_reg_crate("ftui-runtime", "0.11.0", ftui_manifest);
    opt_bad.write_crate(
        "crates/frob-tui",
        "frob-tui",
        "7",
        2,
        "[features]\ndefault = []\nasync-exec = [\"ftui-runtime/asupersync-executor\"]\n\n\
         [dependencies]\nftui-runtime = { version = \"=0.11.0\", default-features = false, features = [\"runtime\", \"native-backend\"] }\n",
    );
    opt_bad.copy_real_stack_lock();
    opt_bad.copy_real_siblings_register();
    opt_bad.generate_lockfile(&opt_bad.repo);

    let out_opt =
        check_deps(&opt_bad.repo, &[WorkspaceRoot::core(&opt_bad.repo)]).expect("opt ftui");
    assert_eq!(rules_of(&out_opt.findings), vec![RULE_FTUI_EXECUTOR_FEATURE]);

    // Plant 2c: direct frankentui dependency from a crate other than `frob-tui` fails `ftui.executor-feature`.
    let wrong_crate = SuiteEnv::new("ftui-outside-frob-tui");
    wrong_crate.write_workspace(&["crates/*"], &[]);
    wrong_crate.add_reg_crate("ftui-runtime", "0.11.0", ftui_manifest);
    wrong_crate.write_crate(
        "crates/frob-cli",
        "frob-cli",
        "7",
        1,
        "[dependencies]\nftui-runtime = { version = \"=0.11.0\", default-features = false, features = [\"runtime\"] }\n",
    );
    wrong_crate.copy_real_stack_lock();
    wrong_crate.copy_real_siblings_register();
    wrong_crate.generate_lockfile(&wrong_crate.repo);

    let out_wrong =
        check_deps(&wrong_crate.repo, &[WorkspaceRoot::core(&wrong_crate.repo)]).expect("wrong");
    assert_eq!(rules_of(&out_wrong.findings), vec![RULE_FTUI_EXECUTOR_FEATURE]);
    assert_eq!(out_wrong.findings[0].subject, "frob-cli -> ftui-runtime");
}

// ============================================================================
// 3. suite.forbidden-package (all 8 never-dependency siblings + derive/ + registry)
// ============================================================================

#[test]
fn denies_never_dependency_sibling() {
    // Parameterized over all eight never-dependency sibling directories from N33c-09.
    let cases: &[(&str, &str, &str, &str)] = &[
        ("frankenjax", "frankenjax/crates/fj-egraph", "fj-egraph", "crates/frob-design"),
        ("franken_threed", "franken_threed/crates/f3d-math", "f3d-math", "crates/frob-scene"),
        ("franken_textiles", "franken_textiles/crates/ftx-drape", "ftx-drape", "crates/frob-dyn"),
        ("franken_firmware", "franken_firmware/crates/franken-hal", "franken-hal", "crates/frob-dyn"),
        ("franken_networkx", "franken_networkx/crates/fnx-algorithms", "fnx-algorithms", "crates/frob-design"),
        ("franken_markdown", "franken_markdown", "franken_markdown", "crates/frob-chronicle"),
        ("frankenmermaid", "frankenmermaid/crates/fm-core", "fm-core", "crates/frob-chronicle"),
        ("franken_surveillance_system", "franken_surveillance_system/crates/fss-twin", "fss-twin", "crates/frob-scene"),
    ];
    assert_eq!(cases.len(), NEVER_DEPENDENCY_SIBLINGS.len());

    for &(sib_name, sib_dir, pkg_name, consumer_dir) in cases {
        let consumer_name = consumer_dir.rsplit('/').next().unwrap();
        let env = SuiteEnv::new(&format!("never-{sib_name}"));
        env.write_workspace(&["crates/*"], &[]);
        env.write_sibling_crate(sib_dir, pkg_name, "0.1.0", "");
        env.write_crate(
            consumer_dir,
            consumer_name,
            "6",
            1,
            &format!("[dependencies]\n{pkg_name} = {{ path = \"../../../{sib_dir}\" }}\n"),
        );
        env.copy_real_stack_lock();
        env.copy_real_siblings_register();
        env.generate_lockfile(&env.repo);

        let out = check_deps(&env.repo, &[WorkspaceRoot::core(&env.repo)]).expect(sib_name);
        assert_eq!(
            rules_of(&out.findings),
            vec![RULE_SUITE_FORBIDDEN_PACKAGE],
            "case {sib_name} failed: {:?}",
            out.findings
        );
        assert!(
            out.findings[0]
                .message
                .contains(&format!("{consumer_name} -> {pkg_name}")),
            "expected dependency path for {sib_name}: {}",
            out.findings[0].message
        );
    }

    // Plant 5 (acceptance criterion 2): a `derive/` workspace dependency on `fj-egraph`
    // fails `suite.forbidden-package` through automatic workspace discovery.
    let derive_env = SuiteEnv::new("never-derive-fj-egraph");
    derive_env.write_workspace(&["crates/*"], &["derive"]);
    derive_env.write_crate("crates/frob-core", "frob-core", "0", 1, "");
    derive_env.write_sibling_crate("frankenjax/crates/fj-egraph", "fj-egraph", "0.1.0", "");
    derive_env.write_crate(
        "derive/frob-derive",
        "frob-derive",
        "\"annex\"",
        1,
        "[workspace]\n\n[dependencies]\nfj-egraph = { path = \"../../../frankenjax/crates/fj-egraph\" }\n",
    );
    derive_env.copy_real_stack_lock();
    derive_env.copy_real_siblings_register();
    derive_env.generate_lockfile(&derive_env.repo);
    derive_env.generate_lockfile(&derive_env.repo.join("derive/frob-derive"));

    let roots = discover_workspace_roots(&derive_env.repo);
    assert!(
        roots.iter().any(|r| r.rel_dir == "derive/frob-derive"),
        "expected derive/frob-derive to be discovered: {roots:?}"
    );
    let derive_out = check_deps(&derive_env.repo, &roots).expect("derive fj-egraph");
    assert_eq!(rules_of(&derive_out.findings), vec![RULE_SUITE_FORBIDDEN_PACKAGE]);
    assert!(derive_out.findings[0].message.contains("frob-derive -> fj-egraph"));

    // Registry-consumed fallback (when sibling is not by path) is also caught by package pattern.
    let reg_env = SuiteEnv::new("never-registry-fnx");
    reg_env.write_workspace(&["crates/*"], &[]);
    reg_env.add_reg_crate("fnx-algorithms", "0.1.0", "");
    reg_env.write_crate(
        "crates/frob-design",
        "frob-design",
        "6",
        3,
        "[dependencies]\nfnx-algorithms = \"=0.1.0\"\n",
    );
    reg_env.copy_real_stack_lock();
    reg_env.copy_real_siblings_register();
    reg_env.generate_lockfile(&reg_env.repo);

    let reg_out =
        check_deps(&reg_env.repo, &[WorkspaceRoot::core(&reg_env.repo)]).expect("registry fnx");
    assert_eq!(rules_of(&reg_out.findings), vec![RULE_SUITE_FORBIDDEN_PACKAGE]);
}

#[test]
fn catches_plant_behind_optional_feature() {
    // Acceptance criterion 4: a feature `cloth-oracle` adding `ftx-drape` that no default
    // build enables is still caught via `--all-features` resolution.
    let env = SuiteEnv::new("optional-feature-cloth-oracle");
    env.write_workspace(&["crates/*"], &[]);
    env.write_sibling_crate("franken_textiles/crates/ftx-drape", "ftx-drape", "0.1.0", "");
    env.write_crate(
        "crates/frob-dyn",
        "frob-dyn",
        "3",
        3,
        "[features]\ndefault = []\ncloth-oracle = [\"dep:ftx-drape\"]\n\n\
         [dependencies]\nftx-drape = { path = \"../../../franken_textiles/crates/ftx-drape\", optional = true }\n",
    );
    env.copy_real_stack_lock();
    env.copy_real_siblings_register();
    env.generate_lockfile(&env.repo);

    let out = check_deps(&env.repo, &[WorkspaceRoot::core(&env.repo)]).expect("check_deps");
    assert_eq!(rules_of(&out.findings), vec![RULE_SUITE_FORBIDDEN_PACKAGE]);
    assert!(
        out.findings[0].message.contains("frob-dyn -> ftx-drape"),
        "expected dependency path in message: {}",
        out.findings[0].message
    );
}

#[test]
fn forbidden_package_admitted_only_by_stack_lock_row() {
    // Committed franken-stack.lock must admit NONE of the 8 siblings in any workspace.
    let real_lock =
        lock::load(&repo_root().join("franken-stack.lock")).expect("load real franken-stack.lock");
    for spec in NEVER_DEPENDENCY_SIBLINGS {
        if let Some(row) = real_lock.foundations.iter().find(|f| f.name == spec.name) {
            let allows_cargo = row.uses.iter().any(|u| {
                matches!(
                    u.posture,
                    Posture::Consume | Posture::ConsumePath | Posture::BuildTime | Posture::Oracle
                )
            });
            assert!(
                !allows_cargo,
                "committed franken-stack.lock must not admit Cargo edges for never-dependency sibling `{}`",
                spec.name
            );
        }
    }

    // With a scratch stack lock whose FD-FJAX row admits `derive/frob-derive` with a named
    // admission gate, `fj-egraph` PASSES in `derive/frob-derive` and STILL FAILS in `core`.
    let admitted_lock = format!(
        "toolchain = \"nightly-2026-08-31\"\n\n\
         [[foundation]]\n\
         id = \"FD-FJAX\"\n\
         name = \"frankenjax\"\n\
         path = \"../frankenjax\"\n\
         git_rev = \"{SAMPLE_GIT_REV}\"\n\
         toolchain = \"nightly-2026-08-31\"\n\
         admission_gate = \"INT-FJAX-01\"\n\
         receipt = \"{SAMPLE_RECEIPT}\"\n\
         fallback = \"Common-subexpression elimination only.\"\n\n\
         [[foundation.uses]]\n\
         posture = \"build-time\"\n\
         purpose = \"Admitted e-graph derivation pass in derive/frob-derive\"\n\
         confined_to = [\"derive/frob-derive\"]\n\
         tier = \"frontier\"\n\
         first_needed = \"M2\"\n"
    );

    let env = SuiteEnv::new("admitted-in-derive-only");
    env.write_workspace(&["crates/*"], &["derive"]);
    env.write_crate("crates/frob-core", "frob-core", "0", 1, "");
    env.write_sibling_crate("frankenjax/crates/fj-egraph", "fj-egraph", "0.1.0", "");
    env.write_crate(
        "derive/frob-derive",
        "frob-derive",
        "\"annex\"",
        1,
        "[workspace]\n\n[dependencies]\nfj-egraph = { path = \"../../../frankenjax/crates/fj-egraph\" }\n",
    );
    env.write_stack_lock_raw(&admitted_lock);
    env.generate_lockfile(&env.repo);
    env.generate_lockfile(&env.repo.join("derive/frob-derive"));

    let derive_roots = vec![
        WorkspaceRoot::core(&env.repo),
        WorkspaceRoot::annex(
            "derive-frob-derive",
            "derive/frob-derive",
            env.repo.join("derive/frob-derive"),
        ),
    ];
    let ok_out = check_deps(&env.repo, &derive_roots).expect("admitted derive workspace");
    assert!(
        ok_out.findings.is_empty(),
        "fj-egraph must pass in admitted workspace derive/frob-derive, got {:?}",
        ok_out.findings
    );

    // Same package in `core` (not listed in FD-FJAX) still fails `suite.forbidden-package`.
    let bad_core = SuiteEnv::new("admitted-derive-still-fails-core");
    bad_core.write_workspace(&["crates/*"], &[]);
    bad_core.write_sibling_crate("frankenjax/crates/fj-egraph", "fj-egraph", "0.1.0", "");
    bad_core.write_crate(
        "crates/frob-dyn",
        "frob-dyn",
        "3",
        3,
        "[dependencies]\nfj-egraph = { path = \"../../../frankenjax/crates/fj-egraph\" }\n",
    );
    bad_core.write_stack_lock_raw(&admitted_lock);
    bad_core.generate_lockfile(&bad_core.repo);

    let bad_out =
        check_deps(&bad_core.repo, &[WorkspaceRoot::core(&bad_core.repo)]).expect("core fails");
    assert_eq!(rules_of(&bad_out.findings), vec![RULE_SUITE_FORBIDDEN_PACKAGE]);

    // Fail-closed: if `admission_gate = "none"`, even the listed workspace still fails!
    let no_gate = SuiteEnv::new("admitted-without-gate-fails");
    no_gate.write_workspace(&["crates/*"], &["derive"]);
    no_gate.write_crate("crates/frob-core", "frob-core", "0", 1, "");
    no_gate.write_sibling_crate("frankenjax/crates/fj-egraph", "fj-egraph", "0.1.0", "");
    no_gate.write_crate(
        "derive/frob-derive",
        "frob-derive",
        "\"annex\"",
        1,
        "[workspace]\n\n[dependencies]\nfj-egraph = { path = \"../../../frankenjax/crates/fj-egraph\" }\n",
    );
    no_gate.write_stack_lock_raw(&admitted_lock.replace("INT-FJAX-01", "none"));
    no_gate.generate_lockfile(&no_gate.repo);
    no_gate.generate_lockfile(&no_gate.repo.join("derive/frob-derive"));

    let ng_roots = vec![
        WorkspaceRoot::core(&no_gate.repo),
        WorkspaceRoot::annex(
            "derive-frob-derive",
            "derive/frob-derive",
            no_gate.repo.join("derive/frob-derive"),
        ),
    ];
    let ng_out = check_deps(&no_gate.repo, &ng_roots).expect("no-gate fails");
    assert_eq!(rules_of(&ng_out.findings), vec![RULE_SUITE_FORBIDDEN_PACKAGE]);

    // Admission-probe workspace under `tests/admission/` is outside checked workspaces.
    let probe = SuiteEnv::new("tests-admission-outside-checked");
    probe.write_workspace(&["crates/*"], &["tests/admission"]);
    probe.write_crate("crates/frob-core", "frob-core", "0", 1, "");
    probe.write_sibling_crate("franken_threed/crates/f3d-math", "f3d-math", "0.1.0", "");
    probe.write_crate(
        "tests/admission/f3d-probe",
        "f3d-probe",
        "",
        0,
        "[workspace]\n\n[dependencies]\nf3d-math = { path = \"../../../../franken_threed/crates/f3d-math\" }\n",
    );
    probe.copy_real_stack_lock();
    probe.copy_real_siblings_register();
    probe.generate_lockfile(&probe.repo);
    probe.generate_lockfile(&probe.repo.join("tests/admission/f3d-probe"));

    let probe_roots = discover_workspace_roots(&probe.repo);
    assert!(
        probe_roots
            .iter()
            .all(|r| !r.rel_dir.starts_with("tests/admission")),
        "tests/admission must never be in discovered workspaces: {probe_roots:?}"
    );
    let probe_out = check_deps(&probe.repo, &probe_roots).expect("probe ignored");
    assert!(probe_out.findings.is_empty(), "{:?}", probe_out.findings);
}

// ============================================================================
// 4. Dev-only franken-kernel (FD-FKER, decided ONLY by L-4.foundation-confined)
// ============================================================================

#[test]
fn denies_franken_kernel_normal_edge() {
    // Plant 4: normal dependency of `frob-core` on `franken-kernel` must fail as
    // `L-4.foundation-confined` on the FD-FKER row, and as NOTHING ELSE.
    let env = SuiteEnv::new("fker-normal-frob-core");
    env.write_workspace(&["crates/*"], &[]);
    env.write_sibling_crate("asupersync/franken_kernel", "franken-kernel", "0.6.0", "");
    env.write_crate(
        "crates/frob-core",
        "frob-core",
        "0",
        1,
        "[dependencies]\nfranken-kernel = { path = \"../../../asupersync/franken_kernel\" }\n",
    );
    env.copy_real_stack_lock();
    env.copy_real_siblings_register();
    env.generate_lockfile(&env.repo);

    let out = check_deps(&env.repo, &[WorkspaceRoot::core(&env.repo)]).expect("check_deps");
    assert_eq!(
        rules_of(&out.findings),
        vec![RULE_L4_FOUNDATION_CONFINED],
        "expected ONLY L-4.foundation-confined on FD-FKER, got {:?}",
        out.findings
    );
    assert_eq!(out.findings[0].subject, "frob-core -> franken-kernel");
    assert!(
        out.findings[0].message.contains("FD-FKER"),
        "expected FD-FKER in message: {}",
        out.findings[0].message
    );
}

#[test]
fn allows_franken_kernel_dev_edge_in_conformance() {
    // Dev-dependency of `frob-conformance` on `franken-kernel` passes cleanly under FD-FKER.
    let env = SuiteEnv::new("fker-dev-conformance");
    env.write_workspace(&["crates/*"], &[]);
    env.write_sibling_crate("asupersync/franken_kernel", "franken-kernel", "0.6.0", "");
    env.write_crate(
        "crates/frob-conformance",
        "frob-conformance",
        "7",
        4,
        "[dev-dependencies]\nfranken-kernel = { path = \"../../../asupersync/franken_kernel\" }\n",
    );
    env.copy_real_stack_lock();
    env.copy_real_siblings_register();
    env.generate_lockfile(&env.repo);

    let out = check_deps(&env.repo, &[WorkspaceRoot::core(&env.repo)]).expect("check_deps");
    assert!(
        out.findings.is_empty(),
        "franken-kernel dev-dep in frob-conformance must pass, got {:?}",
        out.findings
    );
}

// ============================================================================
// 5. suite.egg (direct and transitive, hinting frankenjax)
// ============================================================================

#[test]
fn denies_egg() {
    // Direct `egg` dependency -> `suite.egg` with frankenjax hint.
    let direct = SuiteEnv::new("egg-direct");
    direct.write_workspace(&["crates/*"], &[]);
    direct.add_reg_crate("egg", "0.9.5", "");
    direct.write_crate(
        "crates/frob-kir",
        "frob-kir",
        "2",
        1,
        "[dependencies]\negg = \"=0.9.5\"\n",
    );
    direct.copy_real_stack_lock();
    direct.copy_real_siblings_register();
    direct.generate_lockfile(&direct.repo);

    let out_direct =
        check_deps(&direct.repo, &[WorkspaceRoot::core(&direct.repo)]).expect("direct egg");
    assert_eq!(rules_of(&out_direct.findings), vec![RULE_SUITE_EGG]);
    assert!(out_direct.findings[0].message.contains("frankenjax"));
    assert!(out_direct.findings[0].message.contains("frob-kir -> egg"));

    // Transitive `egg` via a sibling helper -> `suite.egg` naming the full dependency path.
    let trans = SuiteEnv::new("egg-transitive");
    trans.write_workspace(&["crates/*"], &[]);
    trans.add_reg_crate("egg", "0.9.5", "");
    trans.write_sibling_crate(
        "frankensim/crates/fs-ga",
        "fs-ga",
        "0.1.0",
        "[dependencies]\negg = \"=0.9.5\"\n",
    );
    trans.write_crate(
        "crates/frob-lie",
        "frob-lie",
        "1",
        1,
        "[dependencies]\nfs-ga = { path = \"../../../frankensim/crates/fs-ga\", default-features = false }\n",
    );
    trans.copy_real_stack_lock();
    trans.generate_lockfile(&trans.repo);

    let out_trans =
        check_deps(&trans.repo, &[WorkspaceRoot::core(&trans.repo)]).expect("transitive egg");
    assert_eq!(rules_of(&out_trans.findings), vec![RULE_SUITE_EGG]);
    assert!(out_trans.findings[0].message.contains("frob-lie -> fs-ga -> egg"));
    assert!(out_trans.findings[0].message.contains("frankenjax"));
}

// ============================================================================
// 6. registries/siblings.toml schema & franken-stack.lock agreement
// ============================================================================

#[test]
fn siblings_registry_schema() {
    // 1. Unknown posture string is a load error.
    let bad_posture = b"\
schema = \"frob.siblings/1\"

[[sibling]]
name = \"frankentui\"
path = \"../frankentui\"
survey = true
posture = [\"not-a-real-posture\"]
cargo = \"allowed\"
reason = \"some reason\"
fallback = \"none\"
";
    let err_p = parse_siblings_register_bytes(bad_posture, "siblings.toml")
        .expect_err("unknown posture must fail");
    assert!(
        err_p.to_string().contains("not-a-real-posture"),
        "unexpected error: {err_p}"
    );

    // 2. Missing / empty `reason` on a policy row is a load error.
    let missing_reason = b"\
schema = \"frob.siblings/1\"

[[sibling]]
name = \"frankenjax\"
path = \"../frankenjax\"
survey = true
posture = [\"pattern\"]
cargo = \"never\"
fallback = \"CSE only\"
";
    let err_r = parse_siblings_register_bytes(missing_reason, "siblings.toml")
        .expect_err("missing reason must fail");
    assert!(
        err_r.to_string().contains("reason"),
        "unexpected error: {err_r}"
    );

    // 3. `franken_networkx` with `posture = []` and `cargo = "never"` (not admitted) loads.
    let nx_ok = b"\
schema = \"frob.siblings/1\"

[[sibling]]
name = \"franken_networkx\"
path = \"../franken_networkx\"
survey = false
posture = []
cargo = \"never\"
allowed_in = []
forbidden_features = []
reason = \"Not admitted; kinematic tree/cycle algorithms live in frob-design.\"
fallback = \"In-house kinematic graph algorithms.\"
";
    let parsed_nx =
        parse_siblings_register_bytes(nx_ok, "siblings.toml").expect("franken_networkx loads");
    assert_eq!(parsed_nx.siblings.len(), 1);
    assert!(parsed_nx.siblings[0].posture.is_empty());
    assert_eq!(parsed_nx.siblings[0].cargo, CargoPolicy::Never);

    // 4. Real committed `registries/siblings.toml` validates and populates all ten N33c-09 table rows.
    let real_reg = load_siblings_register(&repo_root().join(SIBLINGS_REL_PATH))
        .expect("real registries/siblings.toml must validate");
    let expected_ten = [
        ("frankentui", CargoPolicy::Allowed),
        ("franken_threed", CargoPolicy::Never),
        ("frankenjax", CargoPolicy::Never),
        ("franken_textiles", CargoPolicy::Never),
        ("franken-kernel", CargoPolicy::DevOnly),
        ("franken_firmware", CargoPolicy::Never),
        ("franken_networkx", CargoPolicy::Never),
        ("franken_markdown", CargoPolicy::Never),
        ("frankenmermaid", CargoPolicy::Never),
        ("franken_surveillance_system", CargoPolicy::Never),
    ];
    for (name, expected_cargo) in expected_ten {
        let row = real_reg
            .find(name)
            .unwrap_or_else(|| panic!("missing required sibling row `{name}`"));
        assert_eq!(row.cargo, expected_cargo, "cargo policy mismatch on {name}");
        assert!(!row.reason.trim().is_empty(), "empty reason on {name}");
        assert!(!row.fallback.trim().is_empty(), "empty fallback on {name}");
    }
    assert!(
        real_reg
            .find("franken_networkx")
            .unwrap()
            .posture
            .is_empty(),
        "franken_networkx must have empty posture (not admitted)"
    );
    assert_eq!(
        real_reg.find("frankentui").unwrap().forbidden_features,
        vec!["asupersync-executor".to_owned()]
    );
}

#[test]
fn siblings_policy_agrees_with_stack_lock() {
    let real_lock =
        lock::load(&repo_root().join("franken-stack.lock")).expect("load real franken-stack.lock");
    let real_reg = load_siblings_register(&repo_root().join(SIBLINGS_REL_PATH))
        .expect("load real registries/siblings.toml");
    validate_siblings_against_stack_lock(&real_reg, &real_lock)
        .expect("committed registries/siblings.toml must agree with franken-stack.lock");

    // Disagreeing posture (`frankentui` set to `pattern` here, `consume` in franken-stack.lock)
    // is a load error explicitly naming BOTH `registries/siblings.toml` and `franken-stack.lock`.
    let bad_sibs = b"\
schema = \"frob.siblings/1\"

[[sibling]]
name = \"frankentui\"
path = \"../frankentui\"
survey = true
posture = [\"pattern\"]
cargo = \"never\"
reason = \"planted disagreement\"
fallback = \"none\"
";
    let parsed_bad =
        parse_siblings_register_bytes(bad_sibs, SIBLINGS_REL_PATH).expect("schema itself valid");
    let err = validate_siblings_against_stack_lock(&parsed_bad, &real_lock)
        .expect_err("posture disagreement must fail");
    let msg = err.to_string();
    assert!(
        msg.contains("registries/siblings.toml") && msg.contains("franken-stack.lock"),
        "error must name both registries/siblings.toml and franken-stack.lock, got: {msg}"
    );

    // End-to-end check_deps also refuses a disagreeing `registries/siblings.toml` as a load error.
    let env = SuiteEnv::new("siblings-lock-disagree");
    env.write_workspace(&["crates/*"], &[]);
    env.write_crate("crates/frob-core", "frob-core", "0", 1, "");
    env.copy_real_stack_lock();
    write_file(
        &env.repo.join(SIBLINGS_REL_PATH),
        std::str::from_utf8(bad_sibs).unwrap(),
    );
    env.generate_lockfile(&env.repo);

    let check_err = check_deps(&env.repo, &[WorkspaceRoot::core(&env.repo)])
        .expect_err("check_deps must fail on siblings/stack-lock disagreement");
    let check_msg = check_err.to_string();
    assert!(
        check_msg.contains("registries/siblings.toml") && check_msg.contains("franken-stack.lock"),
        "check_deps error must name both files, got: {check_msg}"
    );
}

// ============================================================================
// 7. Clean real tree with all optional features enabled & CLI exit codes
// ============================================================================

#[test]
fn clean_tree_passes_with_all_features_and_suite_rules() {
    let root = repo_root();
    let mut out = Vec::new();
    let mut err = Vec::new();
    let args = vec![
        "check-deps".to_owned(),
        "--root".to_owned(),
        root.to_string_lossy().into_owned(),
        "--json".to_owned(),
    ];
    let code = cli::run(SUITE_COMMANDS, &args, &mut out, &mut err);
    assert_eq!(
        code,
        ExitCode::from(EXIT_OK),
        "clean tree must pass check-deps with all features enabled; stdout={}, stderr={}",
        String::from_utf8_lossy(&out),
        String::from_utf8_lossy(&err)
    );
    assert!(err.is_empty(), "unexpected stderr: {}", String::from_utf8_lossy(&err));
    validate_report_json(&out).expect("clean tree report must validate");

    // Verify CLI exits 2 (`EXIT_FINDINGS`) when a suite deny rule fires.
    let bad = SuiteEnv::new("cli-exit-2-suite");
    bad.write_workspace(&["crates/*"], &[]);
    bad.add_reg_crate("egg", "0.9.5", "");
    bad.write_crate("crates/frob-kir", "frob-kir", "2", 1, "[dependencies]\negg = \"=0.9.5\"\n");
    bad.copy_real_stack_lock();
    bad.copy_real_siblings_register();
    bad.generate_lockfile(&bad.repo);

    let mut out2 = Vec::new();
    let mut err2 = Vec::new();
    let args2 = vec![
        "check-deps".to_owned(),
        "--root".to_owned(),
        bad.repo.to_string_lossy().into_owned(),
        "--json".to_owned(),
    ];
    let code2 = cli::run(SUITE_COMMANDS, &args2, &mut out2, &mut err2);
    assert_eq!(code2, ExitCode::from(EXIT_FINDINGS));
    validate_report_json(&out2).expect("finding report must validate");
}
