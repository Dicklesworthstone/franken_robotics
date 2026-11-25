//! Integration and unit tests for `xtask check-deps` N34-03 rules:
//! 1. One `asupersync` package per binary (`asup.duplicate`, INV-SUITE-02) with
//!    per-binary `cargo tree` precision confirmation and per-binary report facts.
//! 2. Airlock feature groups (`fsim.group-not-allowed`, `fsim.default-set`,
//!    `fsim.optional-default-on`, INV-SUITE-01), reading allow-lists dynamically
//!    from `crates/frob-fsim/contract.toml` and consuming N33a-11's planted
//!    fixture in place.
//! 3. Tier features off by default (`tier.default-on`, `tier.unnamed`, L-12, INV-WS-04).
//! 4. All-features resolution (`resolve.all-features`).
//! 5. Cross-check rule ownership disjointness across all six `xtask` checks.
//!
//! Synthetic scratch workspaces live under `${BIG_SCRATCH:-${TMPDIR:-/tmp}}/frob-BlackOx/`
//! and are never deleted (COMMON.md 9b). Every workspace test resolves through real
//! `cargo metadata --locked --offline` and (on candidate duplicates) real
//! `cargo tree -p <pkg> -e normal,build --prefix depth --locked --offline`.

use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use xtask::check_contracts::{self, REQUIRED_CONTRACT_SECTIONS};
use xtask::check_deps::{
    self, RULE_ASUP_DUPLICATE, RULE_FSIM_DEFAULT_SET, RULE_FSIM_GROUP_NOT_ALLOWED,
    RULE_FSIM_OPTIONAL_DEFAULT_ON, RULE_RESOLVE_ALL_FEATURES, RULE_TIER_DEFAULT_ON,
    RULE_TIER_UNNAMED, check_airlock_manifest_file, check_deps, load_airlock_groups,
};
use xtask::check_deps_airlock::collect_binary_asupersync_closure;
use xtask::check_generated;
use xtask::check_layers;
use xtask::check_unsafe;
use xtask::crate_map::Finding;
use xtask::metadata::{WorkspaceRoot, load_resolved_metadata, normalize_rel_path};
use xtask::report::{CheckReport, FactValue};
use xtask::stack::verify::{self as stack_verify, VerifyOptions};

static COUNTER: AtomicU64 = AtomicU64::new(0);

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
        .join("frob-BlackOx")
        .join(format!("xtask-deps-suite-{label}-{ts}-{pid}-{n}"));
    fs::create_dir_all(&dir).expect("create scratch dir");
    dir
}

fn write_file(path: &Path, content: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("create parent dirs");
    }
    fs::write(path, content).expect("write synthetic fixture file");
}

fn rules_of(findings: &[Finding]) -> Vec<&'static str> {
    findings.iter().map(|f| f.rule).collect()
}

fn binary_fact_str<'a>(facts: &'a BTreeMap<String, FactValue>, bin: &str, field: &str) -> &'a str {
    let Some(FactValue::Object(bins)) = facts.get("binaries") else {
        panic!("missing `binaries` object in facts: {facts:?}");
    };
    let Some(FactValue::Object(entry)) = bins.get(bin) else {
        panic!("missing binary `{bin}` in facts.binaries: {bins:?}");
    };
    match entry.get(field) {
        Some(FactValue::Str(s)) => s.as_str(),
        other => panic!("expected string field `{field}` for binary `{bin}`, got {other:?}"),
    }
}

/// Offline synthetic workspace builder for N34-03 tests.
struct SynthEnv {
    base: PathBuf,
    repo: PathBuf,
    reg: PathBuf,
}

impl SynthEnv {
    fn new(label: &str) -> Self {
        let base = make_scratch_dir(label);
        let repo = base.join("repo");
        let reg = base.join("local-reg");
        fs::create_dir_all(&repo).expect("create repo dir");
        fs::create_dir_all(&reg).expect("create local-reg dir");
        Self { base, repo, reg }
    }

    fn write_workspace(&self, members: &[&str], with_deny_unsafe: bool) {
        let m: Vec<String> = members.iter().map(|s| format!("\"{s}\"")).collect();
        let mut toml = format!(
            "# synthetic workspace for N34-03\n[workspace]\nresolver = \"3\"\nmembers = [{}]\n",
            m.join(", ")
        );
        if with_deny_unsafe {
            toml.push_str("\n[workspace.lints.rust]\nunsafe_code = \"deny\"\n");
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
        bin_name: Option<&str>,
    ) {
        let dir = self.repo.join(rel_dir);
        let meta = if layer_toml.is_empty() {
            String::new()
        } else {
            format!("[package.metadata.frob]\nlayer = {layer_toml}\nrank = {rank}\n")
        };
        let bin_section = match bin_name {
            Some(bn) => format!("\n[[bin]]\nname = \"{bn}\"\npath = \"src/main.rs\"\n"),
            None => String::new(),
        };
        let manifest = format!(
            "# synthetic crate for N34-03\n[package]\nname = \"{name}\"\nversion = \"0.0.1\"\nedition = \"2024\"\n\n{meta}{bin_section}\n{extra_toml}\n"
        );
        write_file(&dir.join("Cargo.toml"), &manifest);
        write_file(&dir.join("src/lib.rs"), "// synthetic library\n");
        if bin_name.is_some() {
            write_file(&dir.join("src/main.rs"), "fn main() {}\n");
        }
    }

    fn write_sibling_crate(&self, sib_rel_dir: &str, name: &str, version: &str, extra_toml: &str) {
        let dir = self.base.join(sib_rel_dir);
        let manifest = format!(
            "# synthetic sibling crate for N34-03\n[package]\nname = \"{name}\"\nversion = \"{version}\"\nedition = \"2021\"\n\n{extra_toml}\n"
        );
        write_file(&dir.join("Cargo.toml"), &manifest);
        write_file(&dir.join("src/lib.rs"), "// synthetic sibling\n");
    }

    fn add_reg_crate(&self, name: &str, version: &str, extra_toml: &str) {
        let dir = self.reg.join(format!("{name}-{version}"));
        let manifest = format!(
            "# synthetic registry crate for N34-03\n[package]\nname = \"{name}\"\nversion = \"{version}\"\nedition = \"2021\"\n\n{extra_toml}\n"
        );
        write_file(&dir.join("Cargo.toml"), &manifest);
        write_file(&dir.join("src/lib.rs"), "// synthetic registry crate\n");
        write_file(&dir.join(".cargo-checksum.json"), "{\"files\":{}}\n");
    }

    fn write_cargo_config(&self) {
        let reg_str = normalize_rel_path(&self.reg);
        let cfg = format!(
            "# synthetic offline registry config for N34-03\n\
             [source.crates-io]\n\
             replace-with = \"local-reg\"\n\n\
             [source.local-reg]\n\
             directory = \"{reg_str}\"\n"
        );
        write_file(&self.base.join(".cargo/config.toml"), &cfg);
    }

    fn write_stack_lock(&self, asup_version: &str, extra_foundations: &str) {
        let lock_toml = format!(
            "# synthetic franken-stack.lock for N34-03\n\
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
             version = \"{asup_version}\"\n\
             confined_to = [\"any\"]\n\
             admission_gate = \"INT-ASUP-01\"\n\
             receipt = \"{SAMPLE_RECEIPT}\"\n\
             fallback = \"synthetic fallback\"\n\n\
             {extra_foundations}\n"
        );
        write_file(&self.repo.join("franken-stack.lock"), &lock_toml);
    }

    fn generate_lockfile(&self) {
        self.write_cargo_config();
        let out = Command::new("cargo")
            .current_dir(&self.repo)
            .args(["generate-lockfile", "--offline", "--manifest-path"])
            .arg(self.repo.join("Cargo.toml"))
            .output()
            .expect("invoke cargo generate-lockfile --offline");
        assert!(
            out.status.success(),
            "cargo generate-lockfile --offline failed in {}:\n{}",
            self.repo.display(),
            String::from_utf8_lossy(&out.stderr)
        );
    }

    /// Install a minimal `crates/frob-fsim` mirroring the real `contract.toml` and feature names.
    fn install_real_contract_frob_fsim(&self, default_line: &str) {
        let real_contract = fs::read_to_string(repo_root().join("crates/frob-fsim/contract.toml"))
            .expect("read real crates/frob-fsim/contract.toml");
        write_file(&self.repo.join("crates/frob-fsim/contract.toml"), &real_contract);
        let extra = format!(
            "[features]\n\
             default = {default_line}\n\
             mode-inproc = []\n\
             mode-remote = []\n\
             conformance = [\"mode-inproc\"]\n\
             referee = [\"mode-inproc\"]\n\
             geometry = [\"mode-inproc\"]\n\
             ledger = [\"mode-inproc\"]\n\
             optimize = [\"optimize-race\", \"mode-inproc\"]\n\
             optimize-race = [\"mode-inproc\"]\n\
             stats = [\"mode-inproc\"]\n\
             spectral = [\"mode-inproc\"]\n\
             actuation = [\"mode-inproc\"]\n\
             structure = [\"mode-inproc\"]\n\
             multiphysics = [\"mode-inproc\"]\n\
             harness = [\"mode-inproc\"]\n"
        );
        self.write_crate("crates/frob-fsim", "frob-fsim", "0", 2, &extra, None);
    }
}

fn git_init_and_commit(dir: &Path) -> String {
    for args in [
        &["init", "-q"][..],
        &["config", "user.email", "blackox@example.invalid"][..],
        &["config", "user.name", "BlackOx"][..],
        &["add", "."][..],
        &["commit", "-q", "-m", "synthetic seed"][..],
    ] {
        let out = Command::new("git")
            .current_dir(dir)
            .args(args)
            .output()
            .expect("run git");
        assert!(
            out.status.success(),
            "git {:?} failed in {}:\n{}",
            args,
            dir.display(),
            String::from_utf8_lossy(&out.stderr)
        );
    }
    let rev = Command::new("git")
        .current_dir(dir)
        .args(["rev-parse", "HEAD"])
        .output()
        .expect("git rev-parse HEAD");
    assert!(rev.status.success());
    String::from_utf8_lossy(&rev.stdout).trim().to_owned()
}

fn write_valid_contract_bundle(repo: &Path, rel_dir: &str, name: &str, layer: u8, rank: u8) {
    let dir = repo.join(rel_dir);
    let mut md = format!("# Contract: {name}\n\n");
    for sec in REQUIRED_CONTRACT_SECTIONS {
        md.push_str(&format!("## {sec}\n\nSynthetic body for `{name}` section `{sec}`.\n\n"));
    }
    write_file(&dir.join("CONTRACT.md"), &md);
    let toml = format!(
        "[crate]\n\
         name = \"{name}\"\n\
         layer = {layer}\n\
         rank = {rank}\n\
         plane = \"compute\"\n\
         determinism = \"D0\"\n\
         evidence = \"Tested\"\n\
         gates = []\n\
         refusals = []\n\n\
         [budgets]\n\
         wall_ms = 50\n\
         memory_mib = 64\n\n\
         [safe_mode]\n\
         on_budget = \"reference-path\"\n\
         on_calibration = \"reference-path\"\n"
    );
    write_file(&dir.join("contract.toml"), &toml);
    write_file(
        &dir.join("tests/conformance.rs"),
        "#[test]\nfn synthetic_conformance() {}\n",
    );
}

// ============================================================================
// 1. One `asupersync` per binary (INV-SUITE-02, GATE-WS-01 plant 3)
// ============================================================================

#[test]
fn one_asupersync_per_binary() {
    let env = SynthEnv::new("one-asup-per-bin");
    env.add_reg_crate("asupersync", "0.6.2", "");
    env.write_workspace(&["crates/*"], false);
    env.write_crate(
        "crates/frob-core",
        "frob-core",
        "0",
        1,
        "[dependencies]\nasupersync = \"=0.6.2\"\n",
        None,
    );
    env.write_crate(
        "crates/frob-verbs",
        "frob-verbs",
        "6",
        4,
        "[dependencies]\nfrob-core = { path = \"../frob-core\" }\nasupersync = \"=0.6.2\"\n",
        Some("frob"),
    );
    env.write_stack_lock("=0.6.2", "");
    env.generate_lockfile();

    let outcome = check_deps(&env.repo, &[WorkspaceRoot::core(&env.repo)]).expect("check_deps");
    assert!(
        outcome.findings.is_empty(),
        "single asupersync per binary must pass, got {:?}",
        outcome.findings
    );
    assert_eq!(binary_fact_str(&outcome.facts, "frob", "asupersync"), "0.6.2");
}

#[test]
fn second_asupersync_by_path_fails() {
    // Plant 3 of GATE-WS-01: `frob-verbs` gains a dummy path dependency pulling a
    // second `asupersync` package. `check-deps` fails with `asup.duplicate` naming
    // both versions and one path each; `check-layers`, `check-unsafe`, `check-contracts`,
    // `check-generated`, and `check-stack` stay completely clean on the exact same copy.
    let env = SynthEnv::new("second-asup-by-path");
    env.add_reg_crate("asupersync", "0.6.2", "");

    // Second `asupersync` by path, reached through an admitted sibling path dep.
    env.write_sibling_crate("asup-alt", "asupersync", "0.7.0", "");
    env.write_sibling_crate(
        "dummy-sib",
        "dummy-sib",
        "0.1.0",
        "[dependencies]\nasupersync = { path = \"../asup-alt\" }\n",
    );
    let dummy_rev = git_init_and_commit(&env.base.join("dummy-sib"));

    env.write_workspace(&["crates/*"], true);
    env.write_crate(
        "crates/frob-core",
        "frob-core",
        "0",
        1,
        "[dependencies]\nasupersync = \"=0.6.2\"\n\n[lints]\nworkspace = true\n",
        None,
    );
    env.write_crate(
        "crates/frob-verbs",
        "frob-verbs",
        "6",
        4,
        "[dependencies]\nfrob-core = { path = \"../frob-core\" }\ndummy-sib = { path = \"../../../dummy-sib\" }\n\n[lints]\nworkspace = true\n",
        Some("frob"),
    );

    // Populate contract/schema/registry/toolchain files so ALL six checks run cleanly.
    let real = repo_root();
    let schema = fs::read_to_string(real.join("schemas/contract.schema.json"))
        .expect("read contract.schema.json");
    write_file(&env.repo.join("schemas/contract.schema.json"), &schema);
    write_file(
        &env.repo.join("registries/refusals.toml"),
        "schema = \"frob.refusals/1\"\n",
    );
    write_file(
        &env.repo.join("registries/gates.toml"),
        "schema = \"frob.gates/1\"\n",
    );
    write_file(
        &env.repo.join("rust-toolchain.toml"),
        "[toolchain]\nchannel = \"nightly-2026-08-31\"\n",
    );
    write_valid_contract_bundle(&env.repo, "crates/frob-core", "frob-core", 0, 1);
    write_valid_contract_bundle(&env.repo, "crates/frob-verbs", "frob-verbs", 6, 4);

    // Lock with `FD-ASUP` (0.6.2) and `FD-DUMMY` (admitted consume-path for frob-verbs).
    let lock_toml = format!(
        "# synthetic franken-stack.lock for GATE-WS-01 plant 3\n\
         toolchain = \"nightly-2026-08-31\"\n\n\
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
         id = \"FD-DUMMY\"\n\
         name = \"dummy-sib\"\n\
         posture = \"consume-path\"\n\
         path = \"../dummy-sib\"\n\
         git_rev = \"{dummy_rev}\"\n\
         toolchain = \"nightly-2026-08-31\"\n\
         confined_to = [\"frob-verbs\"]\n\
         admission_gate = \"INT-DUMMY-01\"\n\
         receipt = \"{SAMPLE_RECEIPT}\"\n\
         fallback = \"synthetic fallback\"\n"
    );
    write_file(&env.repo.join("franken-stack.lock"), &lock_toml);
    env.generate_lockfile();
    let _repo_rev = git_init_and_commit(&env.repo);

    // Seed `receipts/HEAD` so `check-stack` compares against a valid receipt.
    let write_res = stack_verify::run_write_receipt(&env.repo, &VerifyOptions::default())
        .expect("write synthetic stack receipt");
    assert!(
        write_res.findings.is_empty(),
        "seed receipt must succeed cleanly, got {:?}",
        write_res.findings
    );

    let roots = [WorkspaceRoot::core(&env.repo)];

    // 1. `check-deps` MUST catch Plant 3 with exclusively `asup.duplicate`.
    let deps_out = check_deps(&env.repo, &roots).expect("check_deps");
    assert_eq!(
        rules_of(&deps_out.findings),
        vec![RULE_ASUP_DUPLICATE],
        "expected only asup.duplicate, got {:?}",
        deps_out.findings
    );
    let msg = &deps_out.findings[0].message;
    assert!(msg.contains("0.6.2"), "must name first version 0.6.2: {msg}");
    assert!(msg.contains("0.7.0"), "must name second version 0.7.0: {msg}");
    assert!(
        msg.contains("frob-verbs -> frob-core -> asupersync"),
        "must name first path: {msg}"
    );
    assert!(
        msg.contains("frob-verbs -> dummy-sib -> asupersync"),
        "must name second path: {msg}"
    );
    assert!(msg.contains("INV-SUITE-02"), "must name invariant: {msg}");
    assert!(msg.contains("confirmed: true"), "must be tree-confirmed: {msg}");

    // 2..6. All other five checks MUST remain clean on the exact same scratch copy.
    let layers_out = check_layers::check_layers(&env.repo, &roots).expect("check_layers");
    assert!(
        layers_out.findings.is_empty(),
        "check-layers must stay clean on plant 3, got {:?}",
        layers_out.findings
    );

    let unsafe_out = check_unsafe::check_unsafe(&env.repo, &roots).expect("check_unsafe");
    assert!(
        unsafe_out.findings.is_empty(),
        "check-unsafe must stay clean on plant 3, got {:?}",
        unsafe_out.findings
    );

    let contracts_out =
        check_contracts::check_contracts(&env.repo, &roots).expect("check_contracts");
    assert!(
        contracts_out.findings.is_empty(),
        "check-contracts must stay clean on plant 3, got {:?}",
        contracts_out.findings
    );

    let generated_out = check_generated::run_check(&env.repo, &[], false).expect("check_generated");
    assert!(
        generated_out.findings.is_empty(),
        "check-generated must stay clean on plant 3, got {:?}",
        generated_out.findings
    );

    let stack_out = stack_verify::verify_repo(&env.repo, &VerifyOptions::default())
        .expect("check-stack verify_repo");
    assert!(
        stack_out.findings.is_empty(),
        "check-stack must record duplicates as data and stay clean on plant 3, got {:?}",
        stack_out.findings
    );
}

#[test]
fn second_asupersync_by_registry_fails() {
    let env = SynthEnv::new("second-asup-by-registry");
    env.add_reg_crate("asupersync", "0.6.2", "");
    env.add_reg_crate("asupersync", "0.7.0", "");
    env.write_sibling_crate(
        "sib-reg",
        "sib-reg",
        "0.1.0",
        "[dependencies]\nasupersync = \"=0.7.0\"\n",
    );
    env.write_workspace(&["crates/*"], false);
    env.write_crate(
        "crates/frob-core",
        "frob-core",
        "0",
        1,
        "[dependencies]\nasupersync = \"=0.6.2\"\n",
        None,
    );
    env.write_crate(
        "crates/frob-verbs",
        "frob-verbs",
        "6",
        4,
        "[dependencies]\nfrob-core = { path = \"../frob-core\" }\nsib-reg = { path = \"../../../sib-reg\" }\n",
        Some("frob"),
    );
    let extra_fd = format!(
        "[[foundation]]\n\
         id = \"FD-SIBREG\"\n\
         name = \"sib-reg\"\n\
         posture = \"consume-path\"\n\
         path = \"../sib-reg\"\n\
         git_rev = \"{SAMPLE_GIT_REV}\"\n\
         toolchain = \"nightly-2026-08-31\"\n\
         confined_to = [\"frob-verbs\"]\n\
         admission_gate = \"INT-SIBREG-01\"\n\
         receipt = \"{SAMPLE_RECEIPT}\"\n\
         fallback = \"synthetic fallback\"\n"
    );
    env.write_stack_lock("=0.6.2", &extra_fd);
    env.generate_lockfile();

    let outcome = check_deps(&env.repo, &[WorkspaceRoot::core(&env.repo)]).expect("check_deps");
    assert_eq!(rules_of(&outcome.findings), vec![RULE_ASUP_DUPLICATE]);
    let msg = &outcome.findings[0].message;
    assert!(msg.contains("0.6.2") && msg.contains("0.7.0"), "got: {msg}");
    assert!(msg.contains("confirmed: true"), "got: {msg}");
}

#[test]
fn same_version_two_sources_fails() {
    // Registry `0.6.2` AND path `0.6.2` without `[patch]` are two distinct package identities.
    let env = SynthEnv::new("same-ver-two-sources");
    env.add_reg_crate("asupersync", "0.6.2", "");
    env.write_sibling_crate("asup-path", "asupersync", "0.6.2", "");
    env.write_sibling_crate(
        "sib-path",
        "sib-path",
        "0.1.0",
        "[dependencies]\nasupersync = { path = \"../asup-path\" }\n",
    );
    env.write_workspace(&["crates/*"], false);
    env.write_crate(
        "crates/frob-core",
        "frob-core",
        "0",
        1,
        "[dependencies]\nasupersync = \"=0.6.2\"\n",
        None,
    );
    env.write_crate(
        "crates/frob-verbs",
        "frob-verbs",
        "6",
        4,
        "[dependencies]\nfrob-core = { path = \"../frob-core\" }\nsib-path = { path = \"../../../sib-path\" }\n",
        Some("frob"),
    );
    let extra_fd = format!(
        "[[foundation]]\n\
         id = \"FD-SIBPATH\"\n\
         name = \"sib-path\"\n\
         posture = \"consume-path\"\n\
         path = \"../sib-path\"\n\
         git_rev = \"{SAMPLE_GIT_REV}\"\n\
         toolchain = \"nightly-2026-08-31\"\n\
         confined_to = [\"frob-verbs\"]\n\
         admission_gate = \"INT-SIBPATH-01\"\n\
         receipt = \"{SAMPLE_RECEIPT}\"\n\
         fallback = \"synthetic fallback\"\n"
    );
    env.write_stack_lock("=0.6.2", &extra_fd);
    env.generate_lockfile();

    let outcome = check_deps(&env.repo, &[WorkspaceRoot::core(&env.repo)]).expect("check_deps");
    assert_eq!(rules_of(&outcome.findings), vec![RULE_ASUP_DUPLICATE]);
    let msg = &outcome.findings[0].message;
    assert!(msg.contains("0.6.2"), "must name version 0.6.2: {msg}");
    assert!(msg.contains("path:"), "must distinguish path source: {msg}");
    assert!(msg.contains("registry"), "must distinguish registry source: {msg}");
    assert!(msg.contains("confirmed: true"), "must be confirmed: {msg}");
}

#[test]
fn binary_scoped_duplicate_caught() {
    // Default resolution has one `asupersync` everywhere, so workspace default checks pass;
    // enabling optional features pulls a second `asupersync` ONLY into `frob` (`frob-verbs`),
    // not into `frob-cli`.
    let env = SynthEnv::new("binary-scoped-dup");
    env.add_reg_crate("asupersync", "0.6.2", "");
    env.write_sibling_crate("asup-alt", "asupersync", "0.7.0", "");
    env.write_sibling_crate(
        "sib-opt",
        "sib-opt",
        "0.1.0",
        "[dependencies]\nasupersync = { path = \"../asup-alt\" }\n",
    );
    env.write_workspace(&["crates/*"], false);
    env.write_crate(
        "crates/frob-core",
        "frob-core",
        "0",
        1,
        "[dependencies]\nasupersync = \"=0.6.2\"\n",
        None,
    );
    env.write_crate(
        "crates/frob-verbs",
        "frob-verbs",
        "6",
        4,
        "[dependencies]\nfrob-core = { path = \"../frob-core\" }\nsib-opt = { path = \"../../../sib-opt\", optional = true }\n\n[features]\ndefault = []\nwith-alt = [\"dep:sib-opt\"]\n",
        Some("frob"),
    );
    env.write_crate(
        "crates/frob-cli",
        "frob-cli",
        "7",
        2,
        "[dependencies]\nfrob-core = { path = \"../frob-core\" }\n",
        Some("frob-cli"),
    );
    let extra_fd = format!(
        "[[foundation]]\n\
         id = \"FD-SIBOPT\"\n\
         name = \"sib-opt\"\n\
         posture = \"consume-path\"\n\
         path = \"../sib-opt\"\n\
         git_rev = \"{SAMPLE_GIT_REV}\"\n\
         toolchain = \"nightly-2026-08-31\"\n\
         confined_to = [\"frob-verbs\"]\n\
         admission_gate = \"INT-SIBOPT-01\"\n\
         receipt = \"{SAMPLE_RECEIPT}\"\n\
         fallback = \"synthetic fallback\"\n"
    );
    env.write_stack_lock("=0.6.2", &extra_fd);
    env.generate_lockfile();

    let ws = WorkspaceRoot::core(&env.repo);
    let default_resolved = load_resolved_metadata(&env.repo, &ws).expect("default resolved");
    let verbs_id = default_resolved
        .packages
        .iter()
        .find(|(_, p)| p.name == "frob-verbs")
        .map(|(id, _)| id.clone())
        .expect("frob-verbs id");
    assert_eq!(
        collect_binary_asupersync_closure(&default_resolved, &verbs_id).len(),
        1,
        "default closure must see only 1 asupersync"
    );

    let outcome = check_deps(&env.repo, &[ws]).expect("check_deps");
    assert_eq!(rules_of(&outcome.findings), vec![RULE_ASUP_DUPLICATE]);
    assert!(
        outcome.findings[0].subject.contains("frob-verbs"),
        "duplicate must be reported for frob-verbs only, got {:?}",
        outcome.findings
    );
    assert!(
        outcome.findings[0].message.contains("all-features")
            && outcome.findings[0].message.contains("confirmed: true"),
        "must report confirmed all-features duplicate: {}",
        outcome.findings[0].message
    );
    assert_eq!(
        binary_fact_str(&outcome.facts, "frob-cli", "asupersync"),
        "0.6.2"
    );
}

#[test]
fn frob_verify_links_none() {
    let env = SynthEnv::new("frob-verify-none");
    env.write_workspace(&["crates/*"], false);
    env.write_crate(
        "crates/frob-cert",
        "frob-cert",
        "6",
        1,
        "",
        Some("frob-verify"),
    );
    env.write_stack_lock("=0.6.2", "");
    env.generate_lockfile();

    let outcome = check_deps(&env.repo, &[WorkspaceRoot::core(&env.repo)]).expect("check_deps");
    assert!(
        outcome.findings.is_empty(),
        "frob-verify with zero deps must pass, got {:?}",
        outcome.findings
    );
    assert_eq!(
        binary_fact_str(&outcome.facts, "frob-verify", "asupersync"),
        "none"
    );
    assert_eq!(
        binary_fact_str(&outcome.facts, "frob-verify", "asupersync_source"),
        "none"
    );
}

#[test]
fn binary_without_runtime_reports_none() {
    let env = SynthEnv::new("bin-no-runtime");
    env.write_workspace(&["crates/*"], false);
    env.write_crate("crates/frob-core", "frob-core", "0", 1, "", None);
    env.write_crate(
        "crates/xtask",
        "xtask",
        "7",
        5,
        "[dependencies]\nfrob-core = { path = \"../frob-core\" }\n",
        Some("xtask"),
    );
    env.write_stack_lock("=0.6.2", "");
    env.generate_lockfile();

    let outcome = check_deps(&env.repo, &[WorkspaceRoot::core(&env.repo)]).expect("check_deps");
    assert!(
        outcome.findings.is_empty(),
        "binary without asupersync must not be a finding, got {:?}",
        outcome.findings
    );
    assert_eq!(binary_fact_str(&outcome.facts, "xtask", "asupersync"), "none");
}

#[test]
fn version_mismatch_is_not_reported_here() {
    // Stack lock pins `=0.7.0`, while binary `frob` transitively links a single
    // `asupersync 0.6.2` via an admitted sibling. Version equality against the
    // stack lock is `check-stack`'s rule (BD-008), never `check-deps`'s.
    let env = SynthEnv::new("version-mismatch-not-here");
    env.add_reg_crate("asupersync", "0.6.2", "");
    env.write_sibling_crate(
        "frankensim/crates/fs-ga",
        "fs-ga",
        "0.1.0",
        "[dependencies]\nasupersync = \"=0.6.2\"\n",
    );
    env.write_workspace(&["crates/*"], false);
    env.write_crate(
        "crates/frob-verbs",
        "frob-verbs",
        "6",
        4,
        "[dependencies]\nfs-ga = { path = \"../../../frankensim/crates/fs-ga\", default-features = false }\n",
        Some("frob"),
    );
    env.write_stack_lock("=0.7.0", "");
    env.generate_lockfile();

    let outcome = check_deps(&env.repo, &[WorkspaceRoot::core(&env.repo)]).expect("check_deps");
    assert!(
        outcome.findings.is_empty(),
        "single asupersync differing from stack lock version must NOT be a check-deps finding, got {:?}",
        outcome.findings
    );
    assert_eq!(binary_fact_str(&outcome.facts, "frob", "asupersync"), "0.6.2");
}

#[test]
fn unification_only_candidate_not_reported() {
    // Workspace feature unification causes `frob-core/alt-asup` (enabled ONLY by binary B
    // `frob-verbs`) to appear in `cargo metadata`'s resolve closure of binary A (`frob-cli`).
    // Per-package `cargo tree` confirmation MUST clear binary A (`frob-cli`) and report
    // ONLY binary B (`frob-verbs`) with `confirmed: true`.
    let env = SynthEnv::new("unification-only-cleared");
    env.add_reg_crate("asupersync", "0.6.2", "");
    env.write_sibling_crate("asup-alt", "asupersync", "0.7.0", "");
    env.write_sibling_crate(
        "sib-alt",
        "sib-alt",
        "0.1.0",
        "[dependencies]\nasupersync = { path = \"../asup-alt\" }\n",
    );
    env.write_workspace(&["crates/*"], false);
    env.write_crate(
        "crates/frob-core",
        "frob-core",
        "0",
        1,
        "[dependencies]\nasupersync = \"=0.6.2\"\nsib-alt = { path = \"../../../sib-alt\", optional = true }\n\n[features]\ndefault = []\nalt-asup = [\"dep:sib-alt\"]\n",
        None,
    );
    // Binary A: depends on frob-core with default features only (links ONLY 0.6.2).
    env.write_crate(
        "crates/frob-cli",
        "frob-cli",
        "7",
        2,
        "[dependencies]\nfrob-core = { path = \"../frob-core\" }\n",
        Some("frob-cli"),
    );
    // Binary B: enables `frob-core/alt-asup` (links BOTH 0.6.2 and 0.7.0).
    env.write_crate(
        "crates/frob-verbs",
        "frob-verbs",
        "6",
        4,
        "[dependencies]\nfrob-core = { path = \"../frob-core\", features = [\"alt-asup\"] }\n",
        Some("frob"),
    );
    let extra_fd = format!(
        "[[foundation]]\n\
         id = \"FD-SIBALT\"\n\
         name = \"sib-alt\"\n\
         posture = \"consume-path\"\n\
         path = \"../sib-alt\"\n\
         git_rev = \"{SAMPLE_GIT_REV}\"\n\
         toolchain = \"nightly-2026-08-31\"\n\
         confined_to = [\"frob-core\"]\n\
         admission_gate = \"INT-SIBALT-01\"\n\
         receipt = \"{SAMPLE_RECEIPT}\"\n\
         fallback = \"synthetic fallback\"\n"
    );
    env.write_stack_lock("=0.6.2", &extra_fd);
    env.generate_lockfile();

    let ws = WorkspaceRoot::core(&env.repo);
    // Prove workspace unification genuinely created a candidate on Binary A (`frob-cli`).
    let resolved = load_resolved_metadata(&env.repo, &ws).expect("resolved metadata");
    let cli_id = resolved
        .packages
        .iter()
        .find(|(_, p)| p.name == "frob-cli")
        .map(|(id, _)| id.clone())
        .expect("frob-cli id");
    assert_eq!(
        collect_binary_asupersync_closure(&resolved, &cli_id).len(),
        2,
        "workspace resolve must over-approximate frob-cli with 2 asupersync packages"
    );

    let outcome = check_deps(&env.repo, &[ws]).expect("check_deps");
    assert_eq!(
        rules_of(&outcome.findings),
        vec![RULE_ASUP_DUPLICATE],
        "expected exactly one confirmed duplicate (binary B only), got {:?}",
        outcome.findings
    );
    assert!(
        outcome.findings[0].subject.contains("frob-verbs"),
        "only binary B (frob-verbs) should be reported: {:?}",
        outcome.findings[0]
    );
    assert!(
        !outcome.findings[0].subject.contains("frob-cli"),
        "binary A (frob-cli) must be cleared by per-package confirmation"
    );
    assert!(
        outcome.findings[0].message.contains("confirmed: true"),
        "binary B finding must carry confirmed: true"
    );
    assert_eq!(
        binary_fact_str(&outcome.facts, "frob-cli", "asupersync"),
        "0.6.2"
    );
}

// ============================================================================
// 2. Airlock feature groups (INV-SUITE-01, dynamic `frob-fsim/contract.toml`)
// ============================================================================

#[test]
fn group_allow_list() {
    let real_contract = repo_root().join("crates/frob-fsim/contract.toml");
    let groups = load_airlock_groups(&real_contract).expect("load real airlock groups");
    let off_groups: Vec<_> = groups.values().filter(|g| !g.default).collect();
    assert!(
        off_groups.len() >= 8,
        "expected all off-by-default groups (optimize, optimize-race, stats, spectral, actuation, structure, multiphysics, harness), got {}",
        off_groups.len()
    );

    // Every off-by-default group: one listed crate passes and one unlisted crate fails.
    for rec in off_groups {
        // Passing case.
        let allowed_crate = if rec.dev_only {
            rec.test_consumers
                .iter()
                .find(|c| c.as_str() != "frob-fsim")
                .expect("dev_only group has a non-fsim test_consumer")
                .clone()
        } else {
            rec.consumers
                .first()
                .expect("optional group has at least one consumer")
                .clone()
        };
        let env_pass = SynthEnv::new(&format!("grp-pass-{}", rec.name));
        env_pass.write_workspace(&["crates/*"], false);
        env_pass.install_real_contract_frob_fsim("[\"referee\", \"geometry\", \"ledger\", \"mode-inproc\"]");
        let pass_toml = if rec.dev_only {
            format!(
                "[dev-dependencies]\nfrob-fsim = {{ path = \"../frob-fsim\", default-features = false, features = [\"{}\"] }}\n",
                rec.name
            )
        } else {
            format!(
                "[dependencies]\nfrob-fsim = {{ path = \"../frob-fsim\", default-features = false }}\n\n[features]\ndefault = []\nairlock-{} = [\"frob-fsim/{}\"]\n",
                rec.name, rec.name
            )
        };
        env_pass.write_crate(
            &format!("crates/{allowed_crate}"),
            &allowed_crate,
            "4",
            1,
            &pass_toml,
            None,
        );
        env_pass.write_stack_lock("=0.6.2", "");
        env_pass.generate_lockfile();

        let pass_out =
            check_deps(&env_pass.repo, &[WorkspaceRoot::core(&env_pass.repo)]).expect("check_deps pass");
        assert!(
            pass_out.findings.is_empty(),
            "listed crate `{allowed_crate}` enabling `{}` must pass, got {:?}",
            rec.name,
            pass_out.findings
        );

        // Failing case: pick a crate NOT listed for this group (`frob-gait` specifically for `optimize`).
        let unlisted_crate = if rec.name == "optimize" {
            "frob-gait"
        } else if !rec.consumers.iter().any(|c| c == "frob-spec")
            && !rec.test_consumers.iter().any(|c| c == "frob-spec")
        {
            "frob-spec"
        } else {
            "frob-lie"
        };
        let env_fail = SynthEnv::new(&format!("grp-fail-{}", rec.name));
        env_fail.write_workspace(&["crates/*"], false);
        env_fail.install_real_contract_frob_fsim("[\"referee\", \"geometry\", \"ledger\", \"mode-inproc\"]");
        let fail_toml = format!(
            "[dependencies]\nfrob-fsim = {{ path = \"../frob-fsim\", default-features = false }}\n\n[features]\ndefault = []\nairlock-{} = [\"frob-fsim/{}\"]\n",
            rec.name, rec.name
        );
        env_fail.write_crate(
            &format!("crates/{unlisted_crate}"),
            unlisted_crate,
            "4",
            1,
            &fail_toml,
            None,
        );
        env_fail.write_stack_lock("=0.6.2", "");
        env_fail.generate_lockfile();

        let fail_out =
            check_deps(&env_fail.repo, &[WorkspaceRoot::core(&env_fail.repo)]).expect("check_deps fail");
        assert_eq!(
            rules_of(&fail_out.findings),
            vec![RULE_FSIM_GROUP_NOT_ALLOWED],
            "unlisted crate `{unlisted_crate}` enabling `{}` must fail with fsim.group-not-allowed, got {:?}",
            rec.name,
            fail_out.findings
        );
        assert!(
            fail_out.findings[0].message.contains(unlisted_crate)
                && fail_out.findings[0].message.contains(&rec.name)
                && fail_out.findings[0].message.contains("INV-SUITE-01"),
            "finding must name crate, group, and INV-SUITE-01: {}",
            fail_out.findings[0].message
        );
    }

    // Also verify `harness` (dev_only) fails even for a listed test_consumer if placed in [dependencies].
    let env_dev_only = SynthEnv::new("grp-harness-normal-fails");
    env_dev_only.write_workspace(&["crates/*"], false);
    env_dev_only.install_real_contract_frob_fsim("[\"referee\", \"geometry\", \"ledger\", \"mode-inproc\"]");
    env_dev_only.write_crate(
        "crates/frob-conformance",
        "frob-conformance",
        "7",
        4,
        "[dependencies]\nfrob-fsim = { path = \"../frob-fsim\", default-features = false, features = [\"harness\"] }\n",
        None,
    );
    env_dev_only.write_stack_lock("=0.6.2", "");
    env_dev_only.generate_lockfile();
    let dev_out = check_deps(&env_dev_only.repo, &[WorkspaceRoot::core(&env_dev_only.repo)])
        .expect("check_deps dev_only");
    assert_eq!(rules_of(&dev_out.findings), vec![RULE_FSIM_GROUP_NOT_ALLOWED]);
}

#[test]
fn default_groups_are_three() {
    // 1. Canonical 3 default groups + non-group `mode-inproc` passes.
    let env_ok = SynthEnv::new("default-set-ok");
    env_ok.write_workspace(&["crates/*"], false);
    env_ok.install_real_contract_frob_fsim("[\"referee\", \"geometry\", \"ledger\", \"mode-inproc\"]");
    env_ok.write_stack_lock("=0.6.2", "");
    env_ok.generate_lockfile();
    let ok_out = check_deps(&env_ok.repo, &[WorkspaceRoot::core(&env_ok.repo)]).expect("check_deps");
    assert!(ok_out.findings.is_empty(), "got {:?}", ok_out.findings);

    // 2. Adding a fourth group (`optimize`) to `default` fails `fsim.default-set` only.
    let env_extra = SynthEnv::new("default-set-extra");
    env_extra.write_workspace(&["crates/*"], false);
    env_extra.install_real_contract_frob_fsim(
        "[\"referee\", \"geometry\", \"ledger\", \"optimize\", \"mode-inproc\"]",
    );
    env_extra.write_stack_lock("=0.6.2", "");
    env_extra.generate_lockfile();
    let extra_out =
        check_deps(&env_extra.repo, &[WorkspaceRoot::core(&env_extra.repo)]).expect("check_deps");
    assert_eq!(rules_of(&extra_out.findings), vec![RULE_FSIM_DEFAULT_SET]);

    // 3. Dropping `ledger` from `default` fails `fsim.default-set`.
    let env_missing = SynthEnv::new("default-set-missing");
    env_missing.write_workspace(&["crates/*"], false);
    env_missing.install_real_contract_frob_fsim("[\"referee\", \"geometry\", \"mode-inproc\"]");
    env_missing.write_stack_lock("=0.6.2", "");
    env_missing.generate_lockfile();
    let miss_out =
        check_deps(&env_missing.repo, &[WorkspaceRoot::core(&env_missing.repo)]).expect("check_deps");
    assert_eq!(rules_of(&miss_out.findings), vec![RULE_FSIM_DEFAULT_SET]);
}

#[test]
fn on_by_default_lists_are_explicit() {
    // `frob-spec` (listed for neither `referee` nor `geometry`) fails on either;
    // `frob-scene` enabling `geometry` passes.
    for grp in ["referee", "geometry"] {
        let env = SynthEnv::new(&format!("default-list-spec-{grp}"));
        env.write_workspace(&["crates/*"], false);
        env.install_real_contract_frob_fsim("[\"referee\", \"geometry\", \"ledger\", \"mode-inproc\"]");
        env.write_crate(
            "crates/frob-spec",
            "frob-spec",
            "4",
            1,
            &format!(
                "[dependencies]\nfrob-fsim = {{ path = \"../frob-fsim\", default-features = false, features = [\"{grp}\"] }}\n"
            ),
            None,
        );
        env.write_stack_lock("=0.6.2", "");
        env.generate_lockfile();
        let out = check_deps(&env.repo, &[WorkspaceRoot::core(&env.repo)]).expect("check_deps");
        assert_eq!(
            rules_of(&out.findings),
            vec![RULE_FSIM_GROUP_NOT_ALLOWED],
            "frob-spec enabling `{grp}` must fail with fsim.group-not-allowed, got {:?}",
            out.findings
        );
    }

    let env_scene = SynthEnv::new("default-list-scene-geometry");
    env_scene.write_workspace(&["crates/*"], false);
    env_scene.install_real_contract_frob_fsim("[\"referee\", \"geometry\", \"ledger\", \"mode-inproc\"]");
    env_scene.write_crate(
        "crates/frob-scene",
        "frob-scene",
        "3",
        4,
        "[dependencies]\nfrob-fsim = { path = \"../frob-fsim\", default-features = false, features = [\"geometry\"] }\n",
        None,
    );
    env_scene.write_stack_lock("=0.6.2", "");
    env_scene.generate_lockfile();
    let out = check_deps(&env_scene.repo, &[WorkspaceRoot::core(&env_scene.repo)]).expect("check_deps");
    assert!(
        out.findings.is_empty(),
        "frob-scene enabling geometry must pass, got {:?}",
        out.findings
    );
}

#[test]
fn optional_group_default_on() {
    // 1. `frob-opt` plain dep `features = ["optimize"]` -> `fsim.optional-default-on` naming
    //    `frob-opt`, `optimize`, and "plain declaration".
    let env_plain = SynthEnv::new("opt-default-plain");
    env_plain.write_workspace(&["crates/*"], false);
    env_plain.install_real_contract_frob_fsim("[\"referee\", \"geometry\", \"ledger\", \"mode-inproc\"]");
    env_plain.write_crate(
        "crates/frob-opt",
        "frob-opt",
        "4",
        7,
        "[dependencies]\nfrob-fsim = { path = \"../frob-fsim\", default-features = false, features = [\"optimize\"] }\n",
        None,
    );
    env_plain.write_stack_lock("=0.6.2", "");
    env_plain.generate_lockfile();
    let out_plain =
        check_deps(&env_plain.repo, &[WorkspaceRoot::core(&env_plain.repo)]).expect("check_deps");
    assert_eq!(
        rules_of(&out_plain.findings),
        vec![RULE_FSIM_OPTIONAL_DEFAULT_ON]
    );
    let m1 = &out_plain.findings[0].message;
    assert!(
        m1.contains("frob-opt") && m1.contains("optimize") && m1.contains("plain declaration"),
        "expected frob-opt, optimize, plain declaration: {m1}"
    );

    // 2. `frob-opt` `default = ["airlock-optimize"]`, `airlock-optimize = ["frob-fsim/optimize"]`.
    let env_direct = SynthEnv::new("opt-default-direct-feat");
    env_direct.write_workspace(&["crates/*"], false);
    env_direct.install_real_contract_frob_fsim("[\"referee\", \"geometry\", \"ledger\", \"mode-inproc\"]");
    env_direct.write_crate(
        "crates/frob-opt",
        "frob-opt",
        "4",
        7,
        "[dependencies]\nfrob-fsim = { path = \"../frob-fsim\", default-features = false }\n\n[features]\ndefault = [\"airlock-optimize\"]\nairlock-optimize = [\"frob-fsim/optimize\"]\n",
        None,
    );
    env_direct.write_stack_lock("=0.6.2", "");
    env_direct.generate_lockfile();
    let out_direct =
        check_deps(&env_direct.repo, &[WorkspaceRoot::core(&env_direct.repo)]).expect("check_deps");
    assert_eq!(
        rules_of(&out_direct.findings),
        vec![RULE_FSIM_OPTIONAL_DEFAULT_ON]
    );
    let m2 = &out_direct.findings[0].message;
    assert!(
        m2.contains("frob-opt") && m2.contains("optimize") && m2.contains("airlock-optimize"),
        "expected frob-opt, optimize, airlock-optimize: {m2}"
    );

    // 3. Transitive `default = ["full"]`, `full = ["airlock-optimize"]`.
    let env_trans = SynthEnv::new("opt-default-transitive");
    env_trans.write_workspace(&["crates/*"], false);
    env_trans.install_real_contract_frob_fsim("[\"referee\", \"geometry\", \"ledger\", \"mode-inproc\"]");
    env_trans.write_crate(
        "crates/frob-opt",
        "frob-opt",
        "4",
        7,
        "[dependencies]\nfrob-fsim = { path = \"../frob-fsim\", default-features = false }\n\n[features]\ndefault = [\"full\"]\nfull = [\"airlock-optimize\"]\nairlock-optimize = [\"frob-fsim/optimize\"]\n",
        None,
    );
    env_trans.write_stack_lock("=0.6.2", "");
    env_trans.generate_lockfile();
    let out_trans =
        check_deps(&env_trans.repo, &[WorkspaceRoot::core(&env_trans.repo)]).expect("check_deps");
    assert_eq!(
        rules_of(&out_trans.findings),
        vec![RULE_FSIM_OPTIONAL_DEFAULT_ON]
    );
    let m3 = &out_trans.findings[0].message;
    assert!(
        m3.contains("frob-opt")
            && m3.contains("optimize")
            && m3.contains("airlock-optimize")
            && m3.contains("full"),
        "expected transitive chain details: {m3}"
    );

    // 4. `airlock-optimize` outside `default` -> 0 findings.
    let env_off = SynthEnv::new("opt-default-off");
    env_off.write_workspace(&["crates/*"], false);
    env_off.install_real_contract_frob_fsim("[\"referee\", \"geometry\", \"ledger\", \"mode-inproc\"]");
    env_off.write_crate(
        "crates/frob-opt",
        "frob-opt",
        "4",
        7,
        "[dependencies]\nfrob-fsim = { path = \"../frob-fsim\", default-features = false }\n\n[features]\ndefault = []\nairlock-optimize = [\"frob-fsim/optimize\"]\n",
        None,
    );
    env_off.write_stack_lock("=0.6.2", "");
    env_off.generate_lockfile();
    let out_off =
        check_deps(&env_off.repo, &[WorkspaceRoot::core(&env_off.repo)]).expect("check_deps");
    assert!(
        out_off.findings.is_empty(),
        "off-by-default opt-in feature must pass, got {:?}",
        out_off.findings
    );

    // 5. Unlisted `frob-gait` doing any of the three forms -> `fsim.group-not-allowed` ALONE.
    for (tag, toml) in [
        (
            "plain",
            "[dependencies]\nfrob-fsim = { path = \"../frob-fsim\", default-features = false, features = [\"optimize\"] }\n",
        ),
        (
            "direct",
            "[dependencies]\nfrob-fsim = { path = \"../frob-fsim\", default-features = false }\n\n[features]\ndefault = [\"airlock-optimize\"]\nairlock-optimize = [\"frob-fsim/optimize\"]\n",
        ),
        (
            "transitive",
            "[dependencies]\nfrob-fsim = { path = \"../frob-fsim\", default-features = false }\n\n[features]\ndefault = [\"full\"]\nfull = [\"airlock-optimize\"]\nairlock-optimize = [\"frob-fsim/optimize\"]\n",
        ),
    ] {
        let env_unlisted = SynthEnv::new(&format!("opt-unlisted-gait-{tag}"));
        env_unlisted.write_workspace(&["crates/*"], false);
        env_unlisted
            .install_real_contract_frob_fsim("[\"referee\", \"geometry\", \"ledger\", \"mode-inproc\"]");
        env_unlisted.write_crate("crates/frob-gait", "frob-gait", "4", 3, toml, None);
        env_unlisted.write_stack_lock("=0.6.2", "");
        env_unlisted.generate_lockfile();
        let out_u = check_deps(&env_unlisted.repo, &[WorkspaceRoot::core(&env_unlisted.repo)])
            .expect("check_deps unlisted");
        assert_eq!(
            rules_of(&out_u.findings),
            vec![RULE_FSIM_GROUP_NOT_ALLOWED],
            "unlisted frob-gait ({tag}) must produce ONLY fsim.group-not-allowed, got {:?}",
            out_u.findings
        );
    }
}

#[test]
fn airlock_fixture_frob_dyn_structure_refused() {
    // Consume N33a-11's planted fixture IN PLACE (never copy it).
    let root = repo_root();
    let fixture_rel = "crates/frob-fsim/tests/fixtures/frob_dyn_enables_structure/Cargo.toml";
    let fixture_path = root.join(fixture_rel);
    assert!(
        fixture_path.is_file(),
        "N33a-11 planted fixture must exist at {}",
        fixture_path.display()
    );

    let real_contract_path = root.join("crates/frob-fsim/contract.toml");
    let real_groups = load_airlock_groups(&real_contract_path).expect("load real contract.toml");

    let findings = check_airlock_manifest_file(&fixture_path, fixture_rel, &real_groups)
        .expect("check planted fixture in place");
    assert_eq!(
        rules_of(&findings),
        vec![RULE_FSIM_GROUP_NOT_ALLOWED],
        "N33a-11 planted fixture must fail with fsim.group-not-allowed, got {findings:?}"
    );
    assert!(
        findings[0].message.contains("frob-dyn") && findings[0].message.contains("structure"),
        "finding must name frob-dyn and structure: {}",
        findings[0].message
    );

    // Scratch copy of `contract.toml` adding `frob-dyn` to `structure.consumers` proves the
    // allow-list is read dynamically from `contract.toml` (not compiled into `xtask`).
    let scratch = make_scratch_dir("fixture-dynamic-contract");
    let original_toml = fs::read_to_string(&real_contract_path).expect("read contract.toml");
    let needle = "consumers = [\"frob-foundry\", \"frob-ground\"]";
    assert!(
        original_toml.contains(needle),
        "real contract.toml structure consumers line changed unexpectedly"
    );
    let amended_toml = original_toml.replacen(
        needle,
        "consumers = [\"frob-foundry\", \"frob-ground\", \"frob-dyn\"]",
        1,
    );
    let scratch_contract = scratch.join("contract.toml");
    write_file(&scratch_contract, &amended_toml);
    let amended_groups = load_airlock_groups(&scratch_contract).expect("load amended contract.toml");
    let amended_findings = check_airlock_manifest_file(&fixture_path, fixture_rel, &amended_groups)
        .expect("check fixture against amended contract.toml");
    assert!(
        !amended_findings
            .iter()
            .any(|f| f.rule == RULE_FSIM_GROUP_NOT_ALLOWED),
        "adding frob-dyn to scratch structure.consumers must clear fsim.group-not-allowed, got {amended_findings:?}"
    );

    // If `structure` were also a default group in a scratch copy, even the plain-declaration
    // fixture produces zero findings total.
    let mut default_groups = amended_groups;
    default_groups
        .get_mut("structure")
        .expect("structure record")
        .default = true;
    let zero_findings = check_airlock_manifest_file(&fixture_path, fixture_rel, &default_groups)
        .expect("check fixture with default=true scratch record");
    assert!(
        zero_findings.is_empty(),
        "expected zero findings when listed and default=true, got {zero_findings:?}"
    );
}

// ============================================================================
// 3. Tier features off by default (`tier.default-on`, `tier.unnamed`, L-12, INV-WS-04)
// ============================================================================

#[test]
fn tier_feature_in_default_fails() {
    // 1. Direct `frontier-gait` in `default` fails `tier.default-on` (INV-WS-04).
    let env = SynthEnv::new("tier-in-default");
    env.write_workspace(&["crates/*"], false);
    env.write_crate(
        "crates/frob-opt",
        "frob-opt",
        "4",
        7,
        "[features]\ndefault = [\"frontier-gait\"]\nfrontier-gait = []\n",
        None,
    );
    env.write_stack_lock("=0.6.2", "");
    env.generate_lockfile();

    let outcome = check_deps(&env.repo, &[WorkspaceRoot::core(&env.repo)]).expect("check_deps");
    assert_eq!(rules_of(&outcome.findings), vec![RULE_TIER_DEFAULT_ON]);
    assert!(
        outcome.findings[0].message.contains("frontier-gait")
            && outcome.findings[0].message.contains("INV-WS-04"),
        "got: {}",
        outcome.findings[0].message
    );

    // 2. Transitive `default -> all -> moonshot-orbit` fails `tier.default-on`, and
    //    `contract.toml` F/M features without prefixes fail `tier.unnamed`.
    let env_unnamed = SynthEnv::new("tier-unnamed");
    env_unnamed.write_workspace(&["crates/*"], false);
    env_unnamed.write_crate(
        "crates/frob-synth",
        "frob-synth",
        "5",
        5,
        "[features]\ndefault = [\"all\"]\nall = [\"moonshot-orbit\"]\nmoonshot-orbit = []\nraw-f = []\nraw-m = []\n",
        None,
    );
    write_file(
        &env_unnamed.repo.join("crates/frob-synth/contract.toml"),
        "[crate]\nname = \"frob-synth\"\nlayer = 5\nrank = 5\n\n\
         [features.raw-f]\ntier = \"F\"\n\n\
         [features.raw-m]\ntier = \"M\"\n\n\
         [features.moonshot-orbit]\ntier = \"M\"\n",
    );
    env_unnamed.write_stack_lock("=0.6.2", "");
    env_unnamed.generate_lockfile();

    let out2 =
        check_deps(&env_unnamed.repo, &[WorkspaceRoot::core(&env_unnamed.repo)]).expect("check_deps");
    let rules = rules_of(&out2.findings);
    assert!(
        rules.contains(&RULE_TIER_DEFAULT_ON),
        "expected tier.default-on, got {:?}",
        out2.findings
    );
    let unnamed_count = rules.iter().filter(|&&r| r == RULE_TIER_UNNAMED).count();
    assert_eq!(
        unnamed_count, 2,
        "expected 2 tier.unnamed findings (raw-f and raw-m), got {:?}",
        out2.findings
    );
}

#[test]
fn tier_feature_leaks_through_dependency_fails() {
    // 1. Plain core dependency enabling `frontier-gait` fails `tier.default-on`.
    let env = SynthEnv::new("tier-leak-plain");
    env.write_workspace(&["crates/*"], false);
    env.write_crate(
        "crates/frob-opt",
        "frob-opt",
        "4",
        7,
        "[features]\ndefault = []\nfrontier-gait = []\n",
        None,
    );
    env.write_crate(
        "crates/frob-verbs",
        "frob-verbs",
        "6",
        4,
        "[dependencies]\nfrob-opt = { path = \"../frob-opt\", features = [\"frontier-gait\"] }\n",
        None,
    );
    env.write_stack_lock("=0.6.2", "");
    env.generate_lockfile();

    let out = check_deps(&env.repo, &[WorkspaceRoot::core(&env.repo)]).expect("check_deps");
    assert_eq!(rules_of(&out.findings), vec![RULE_TIER_DEFAULT_ON]);
    assert!(
        out.findings[0].message.contains("frontier-gait")
            && out.findings[0].message.contains("INV-WS-04"),
        "got: {}",
        out.findings[0].message
    );

    // 2. Non-binary core crate forwarding `frontier-gait` under a non-tier feature name fails.
    let env_feat = SynthEnv::new("tier-leak-non-tier-feat");
    env_feat.write_workspace(&["crates/*"], false);
    env_feat.write_crate(
        "crates/frob-opt",
        "frob-opt",
        "4",
        7,
        "[features]\ndefault = []\nfrontier-gait = []\n",
        None,
    );
    env_feat.write_crate(
        "crates/frob-verbs",
        "frob-verbs",
        "6",
        4,
        "[dependencies]\nfrob-opt = { path = \"../frob-opt\" }\n\n[features]\ndefault = []\nfast = [\"frob-opt/frontier-gait\"]\n",
        None,
    );
    env_feat.write_stack_lock("=0.6.2", "");
    env_feat.generate_lockfile();

    let out2 =
        check_deps(&env_feat.repo, &[WorkspaceRoot::core(&env_feat.repo)]).expect("check_deps");
    assert_eq!(rules_of(&out2.findings), vec![RULE_TIER_DEFAULT_ON]);
}

#[test]
fn tier_feature_behind_explicit_feature_passes() {
    let env = SynthEnv::new("tier-explicit-passes");
    env.write_workspace(&["crates/*"], false);
    env.write_crate(
        "crates/frob-opt",
        "frob-opt",
        "4",
        7,
        "[features]\ndefault = []\nfrontier-gait = []\n",
        None,
    );
    // Same-tier non-default forwarding on a core library crate passes.
    env.write_crate(
        "crates/frob-verbs",
        "frob-verbs",
        "6",
        4,
        "[dependencies]\nfrob-opt = { path = \"../frob-opt\" }\n\n[features]\ndefault = []\nfrontier-verbs = [\"frob-opt/frontier-gait\"]\n",
        None,
    );
    // Non-default opt-in feature on a binary crate passes.
    env.write_crate(
        "crates/frob-cli",
        "frob-cli",
        "7",
        2,
        "[dependencies]\nfrob-opt = { path = \"../frob-opt\" }\n\n[features]\ndefault = []\nenable-gait = [\"frob-opt/frontier-gait\"]\n",
        Some("frob"),
    );
    env.write_stack_lock("=0.6.2", "");
    env.generate_lockfile();

    let out = check_deps(&env.repo, &[WorkspaceRoot::core(&env.repo)]).expect("check_deps");
    assert!(
        out.findings.is_empty(),
        "tier feature gated behind explicit non-default features must pass, got {:?}",
        out.findings
    );
}

// ============================================================================
// 4. All-features resolution (`resolve.all-features`)
// ============================================================================

#[test]
fn all_features_conflict_fails() {
    // Baseline: without the conflicting optional pin, default + all-features resolution passes.
    let env_ok = SynthEnv::new("all-features-baseline-ok");
    env_ok.add_reg_crate("dummy-pin", "1.0.151", "");
    env_ok.add_reg_crate("dummy-pin", "1.0.154", "");
    env_ok.write_workspace(&["crates/*"], false);
    env_ok.write_crate(
        "crates/frob-core",
        "frob-core",
        "0",
        1,
        "[dependencies]\ndummy-pin = \"=1.0.151\"\n",
        None,
    );
    let extra_fd = format!(
        "[[foundation]]\n\
         id = \"FD-DPIN\"\n\
         name = \"dummy-pin\"\n\
         posture = \"consume\"\n\
         version = \"=1.0.151\"\n\
         confined_to = [\"any\"]\n\
         admission_gate = \"INT-DPIN-01\"\n\
         receipt = \"{SAMPLE_RECEIPT}\"\n\
         fallback = \"synthetic fallback\"\n"
    );
    env_ok.write_stack_lock("=0.6.2", &extra_fd);
    env_ok.generate_lockfile();
    let ok_out = check_deps(&env_ok.repo, &[WorkspaceRoot::core(&env_ok.repo)]).expect("baseline ok");
    assert!(ok_out.findings.is_empty(), "baseline must pass: {:?}", ok_out.findings);

    // Planted second exact pin `=1.0.154` behind an optional feature conflicts with `=1.0.151`.
    let env_bad = SynthEnv::new("all-features-conflict");
    env_bad.add_reg_crate("dummy-pin", "1.0.151", "");
    env_bad.add_reg_crate("dummy-pin", "1.0.154", "");
    env_bad.write_workspace(&["crates/*"], false);
    env_bad.write_crate(
        "crates/frob-core",
        "frob-core",
        "0",
        1,
        "[dependencies]\ndummy-pin = \"=1.0.151\"\n",
        None,
    );
    env_bad.write_crate(
        "crates/frob-opt",
        "frob-opt",
        "4",
        7,
        "[dependencies]\ndummy-pin = { version = \"=1.0.154\", optional = true }\n\n[features]\ndefault = []\nopt-pin = [\"dep:dummy-pin\"]\n",
        None,
    );
    env_bad.write_stack_lock("=0.6.2", &extra_fd);
    env_bad.write_cargo_config();

    let bad_out = check_deps(&env_bad.repo, &[WorkspaceRoot::core(&env_bad.repo)])
        .expect("resolution conflict must return CheckOutcome finding, not CheckError");
    assert_eq!(
        rules_of(&bad_out.findings),
        vec![RULE_RESOLVE_ALL_FEATURES],
        "expected resolve.all-features finding, got {:?}",
        bad_out.findings
    );
    let msg = &bad_out.findings[0].message;
    assert!(msg.contains("dummy-pin"), "must name package: {msg}");
    assert!(
        msg.contains("=1.0.151") && msg.contains("=1.0.154"),
        "must name both conflicting requirements: {msg}"
    );
}

// ============================================================================
// 5. Cross-check rule ownership and real repository sanity
// ============================================================================

#[test]
fn owned_rules_pairwise_disjoint_across_all_six_checks() {
    let tables: &[(&str, &[&str])] = &[
        ("check-layers", check_layers::OWNED_RULES),
        ("check-deps", check_deps::OWNED_RULES),
        ("check-unsafe", check_unsafe::OWNED_RULES),
        ("check-generated", check_generated::OWNED_RULES),
        ("check-contracts", check_contracts::OWNED_RULES),
        ("check-stack", stack_verify::OWNED_RULES),
    ];

    let mut owner_by_rule: BTreeMap<&str, &str> = BTreeMap::new();
    for &(check_name, rules) in tables {
        let mut local_seen = BTreeSet::new();
        for &rule in rules {
            assert!(
                local_seen.insert(rule),
                "duplicate rule `{rule}` inside `{check_name}::OWNED_RULES`"
            );
            if let Some(prev) = owner_by_rule.insert(rule, check_name) {
                panic!(
                    "rule `{rule}` is owned by both `{prev}` and `{check_name}` (ownership rule requires unique check owner)"
                );
            }
        }
    }

    for required in [
        RULE_ASUP_DUPLICATE,
        RULE_FSIM_GROUP_NOT_ALLOWED,
        RULE_FSIM_DEFAULT_SET,
        RULE_FSIM_OPTIONAL_DEFAULT_ON,
        RULE_TIER_DEFAULT_ON,
        RULE_TIER_UNNAMED,
        RULE_RESOLVE_ALL_FEATURES,
    ] {
        assert_eq!(
            owner_by_rule.get(required).copied(),
            Some("check-deps"),
            "N34-03 rule `{required}` must be owned exclusively by check-deps"
        );
    }
}

#[test]
fn real_tree_passes_n34_03_rules_and_reports_binary_facts() {
    let root = repo_root();
    let roots = [WorkspaceRoot::core(&root)];

    let out1 = check_deps(&root, &roots).expect("check_deps on real repo (run 1)");
    assert!(
        out1.findings.is_empty(),
        "real repository must pass check-deps cleanly, got {:?}",
        out1.findings
    );
    assert_eq!(
        binary_fact_str(&out1.facts, "frob-verify", "asupersync"),
        "none"
    );
    assert_eq!(binary_fact_str(&out1.facts, "xtask", "asupersync"), "none");

    // Determinism D0: canonical JSON reports are byte-for-byte identical across two runs.
    let out2 = check_deps(&root, &roots).expect("check_deps on real repo (run 2)");
    let r1 = CheckReport::new("check-deps", ".", out1.findings, out1.facts, 0)
        .to_canonical_json()
        .expect("canonical json 1");
    let r2 = CheckReport::new("check-deps", ".", out2.findings, out2.facts, 0)
        .to_canonical_json()
        .expect("canonical json 2");
    assert_eq!(r1, r2, "check-deps report must be deterministic (D0)");
}
