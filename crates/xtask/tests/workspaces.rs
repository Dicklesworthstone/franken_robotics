//! Unit and integration tests for the workspace table, core forbidden-package list,
//! `core.forbidden-package` check-deps rule, exempt `apps/` link packages, absent-root
//! facts, and toolchain-file rules (`N34p4-02`).
//!
//! Run as `cargo test -p xtask --test workspaces`.
//! Scratch trees are created under `${BIG_SCRATCH:-${TMPDIR:-/tmp}}/frob-BrownHeron/`
//! and never deleted (COMMON.md 9b).

use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use frob_core::json::{Limits, admit};
use frob_core::registry::toml::{self, Value};
use xtask::check_deps::{
    self, OWNED_RULES as DEPS_OWNED_RULES, RULE_CORE_FORBIDDEN_PACKAGE,
    RULE_L4_FOUNDATION_CONFINED, check_deps,
};
use xtask::check_layers::{self, RULE_UNLISTED_CRATE, check_layers};
use xtask::cli::{self, CheckFn, EXIT_OK};
use xtask::crate_map::{WorkspaceMember, validate as validate_crate_map};
use xtask::metadata::{WorkspaceKind, WorkspaceRoot};
use xtask::report::FactValue;
use xtask::workspaces::{
    APPS_LINK_PACKAGES, CERT_TOOLCHAIN_FILE, EXPECTED_CODE_LINKING_COUNT, EXPECTED_RETARGET_COUNT,
    EXPECTED_WORKSPACE_COUNT, FORBIDDEN_CORE_PACKAGES, ForbiddenPackage, ForbiddenReason,
    REPOSITORY_LAYOUT, ROOT_TOOLCHAIN_FILE, WORKER_TOOLCHAIN_FILE, WORKSPACES,
    check_toolchain_files, link_packages, match_forbidden_core_manifest_dir,
    match_forbidden_core_package, package_pattern_matches, validate_forbidden_list,
    validate_workspace_table_against_repo, workspace_by_name, workspace_roots,
};

static SCRATCH_SEQ: AtomicU64 = AtomicU64::new(0);

const SAMPLE_RECEIPT: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const SAMPLE_GIT_REV: &str = "0123456789abcdef0123456789abcdef01234567";

fn repo_root() -> PathBuf {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest_dir
        .parent()
        .and_then(Path::parent)
        .expect("crates/xtask sits two levels beneath repo root")
        .to_path_buf()
}

fn scratch_dir(label: &str) -> PathBuf {
    let base = env::var_os("BIG_SCRATCH")
        .or_else(|| env::var_os("TMPDIR"))
        .map(PathBuf::from)
        .unwrap_or_else(env::temp_dir);
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let seq = SCRATCH_SEQ.fetch_add(1, Ordering::Relaxed);
    let dir = base
        .join("frob-BrownHeron")
        .join(format!("workspaces-{label}-{nanos}-{seq}"));
    fs::create_dir_all(&dir).expect("create scratch directory");
    dir
}

fn write_file(path: &Path, contents: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("create parent dirs");
    }
    fs::write(path, contents).expect("write file");
}

struct SynthEnv {
    base: PathBuf,
    repo: PathBuf,
    reg: PathBuf,
}

impl SynthEnv {
    fn new(label: &str) -> Self {
        let base = scratch_dir(label);
        let repo = base.join("repo");
        let reg = base.join("local-reg");
        fs::create_dir_all(&repo).expect("create repo dir");
        fs::create_dir_all(&reg).expect("create reg dir");
        write_file(
            &base.join(".cargo/config.toml"),
            &format!(
                "[source.crates-io]\nreplace-with = \"local-reg\"\n\n[source.local-reg]\ndirectory = \"{}\"\n",
                reg.display()
            ),
        );
        Self { base, repo, reg }
    }

    fn write_workspace(&self, members: &[&str], exclude: &[&str]) {
        let mem: Vec<String> = members.iter().map(|m| format!("\"{m}\"")).collect();
        let exc: Vec<String> = exclude.iter().map(|e| format!("\"{e}\"")).collect();
        let mut toml = format!(
            "[workspace]\nresolver = \"3\"\nmembers = [{}]\n",
            mem.join(", ")
        );
        if !exc.is_empty() {
            toml.push_str(&format!("exclude = [{}]\n", exc.join(", ")));
        }
        write_file(&self.repo.join("Cargo.toml"), &toml);
    }

    fn write_crate(&self, rel_dir: &str, name: &str, layer_toml: &str, rank: u8, extra_toml: &str) {
        let dir = self.repo.join(rel_dir);
        let mut manifest =
            format!("[package]\nname = \"{name}\"\nversion = \"0.0.1\"\nedition = \"2024\"\n");
        if !layer_toml.is_empty() {
            manifest.push_str(&format!(
                "\n[package.metadata.frob]\nlayer = {layer_toml}\nrank = {rank}\n"
            ));
        }
        if !extra_toml.is_empty() {
            manifest.push('\n');
            manifest.push_str(extra_toml);
            manifest.push('\n');
        }
        write_file(&dir.join("Cargo.toml"), &manifest);
        write_file(&dir.join("src/lib.rs"), "// synthetic crate\n");
    }

    fn write_bin_crate(&self, rel_dir: &str, name: &str, extra_toml: &str) {
        let dir = self.repo.join(rel_dir);
        let mut manifest =
            format!("[package]\nname = \"{name}\"\nversion = \"0.0.1\"\nedition = \"2024\"\n");
        if !extra_toml.is_empty() {
            manifest.push('\n');
            manifest.push_str(extra_toml);
            manifest.push('\n');
        }
        write_file(&dir.join("Cargo.toml"), &manifest);
        write_file(&dir.join("src/main.rs"), "fn main() {}\n");
    }

    fn add_reg_crate(&self, name: &str, version: &str, extra_toml: &str) {
        let dir = self.reg.join(format!("{name}-{version}"));
        let mut manifest = format!(
            "[workspace]\n\n[package]\nname = \"{name}\"\nversion = \"{version}\"\nedition = \"2021\"\n"
        );
        if !extra_toml.is_empty() {
            manifest.push('\n');
            manifest.push_str(extra_toml);
            manifest.push('\n');
        }
        write_file(&dir.join("Cargo.toml"), &manifest);
        write_file(&dir.join("src/lib.rs"), "// synthetic registry crate\n");
        write_file(&dir.join(".cargo-checksum.json"), "{\"files\":{}}\n");
    }

    fn write_sibling_crate(
        &self,
        rel_from_base: &str,
        name: &str,
        version: &str,
        extra_toml: &str,
    ) {
        let dir = self.base.join(rel_from_base);
        let mut manifest = format!(
            "[workspace]\n\n[package]\nname = \"{name}\"\nversion = \"{version}\"\nedition = \"2021\"\n"
        );
        if !extra_toml.is_empty() {
            manifest.push('\n');
            manifest.push_str(extra_toml);
            manifest.push('\n');
        }
        write_file(&dir.join("Cargo.toml"), &manifest);
        write_file(&dir.join("src/lib.rs"), "// synthetic sibling crate\n");
    }

    fn write_stack_lock(&self, foundations_toml: &str) {
        let text = format!("toolchain = \"nightly-2026-08-31\"\n\n{foundations_toml}");
        write_file(&self.repo.join("franken-stack.lock"), &text);
    }

    fn generate_lockfile(&self, ws_dir: &Path) {
        let out = Command::new("cargo")
            .current_dir(ws_dir)
            .args([
                "generate-lockfile",
                "--offline",
                "--manifest-path",
                &ws_dir.join("Cargo.toml").display().to_string(),
            ])
            .output()
            .expect("spawn cargo generate-lockfile");
        assert!(
            out.status.success(),
            "cargo generate-lockfile failed in {}: stdout={}, stderr={}",
            ws_dir.display(),
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        );
    }
}

fn cudarc_foundation_toml() -> String {
    format!(
        "[[foundation]]\n\
         id = \"FD-CUDA\"\n\
         name = \"cudarc\"\n\
         posture = \"consume\"\n\
         version = \"=0.19.9\"\n\
         confined_to = [\"spark/frob-spark-cuda\"]\n\
         admission_gate = \"INT-CUDA-01\"\n\
         receipt = \"{SAMPLE_RECEIPT}\"\n\
         fallback = \"CPU reference\"\n"
    )
}

fn helper_foundation_toml() -> String {
    format!(
        "[[foundation]]\n\
         id = \"FD-HELPER\"\n\
         name = \"ext-helper\"\n\
         posture = \"consume-path\"\n\
         path = \"../ext-helper\"\n\
         git_rev = \"{SAMPLE_GIT_REV}\"\n\
         confined_to = [\"frob-core\"]\n\
         admission_gate = \"INT-HELPER-01\"\n\
         receipt = \"{SAMPLE_RECEIPT}\"\n\
         fallback = \"synthetic fallback\"\n"
    )
}

// ============================================================================
// 1. seven_workspaces_listed
// ============================================================================

#[test]
fn seven_workspaces_listed() {
    assert_eq!(WORKSPACES.len(), EXPECTED_WORKSPACE_COUNT);
    assert_eq!(EXPECTED_WORKSPACE_COUNT, 7);

    let names: Vec<&str> = WORKSPACES.iter().map(|w| w.name).collect();
    assert_eq!(
        names,
        vec![
            "core",
            "spark-cuda",
            "spark-metal",
            "apps",
            "derive",
            "bindings",
            "worker",
        ]
    );

    let mut seen_names = BTreeSet::new();
    let mut seen_roots = BTreeSet::new();
    let mut seen_locks = BTreeSet::new();
    for row in WORKSPACES {
        assert!(
            seen_names.insert(row.name),
            "duplicate workspace name {}",
            row.name
        );
        assert!(
            seen_roots.insert(row.root_dir),
            "duplicate root_dir {}",
            row.root_dir
        );
        assert!(
            seen_locks.insert(row.lockfile),
            "duplicate lockfile {}",
            row.lockfile
        );
        assert_eq!(
            row.lockfile,
            row.expected_lockfile(),
            "lockfile path mismatch for workspace `{}`",
            row.name
        );
    }

    let core = workspace_by_name("core").expect("core row");
    assert_eq!(core.root_dir, "");
    assert_eq!(core.lockfile, "Cargo.lock");
    assert_eq!(core.kind, WorkspaceKind::Core);
    assert!(core.includes_surface_mcp);

    let apps = workspace_by_name("apps").expect("apps row");
    assert_eq!(apps.root_dir, "apps");
    assert_eq!(apps.kind, WorkspaceKind::Link);
    assert_eq!(apps.link_packages, APPS_LINK_PACKAGES);
    assert_eq!(link_packages(), vec!["frob", "frobd"]);

    assert!(DEPS_OWNED_RULES.contains(&RULE_CORE_FORBIDDEN_PACKAGE));
    assert!(!REPOSITORY_LAYOUT.is_empty());
}

// ============================================================================
// 2. six_link_code_one_retargets
// ============================================================================

#[test]
fn six_link_code_one_retargets() {
    let linking: Vec<&str> = WORKSPACES
        .iter()
        .filter(|w| w.links_code())
        .map(|w| w.name)
        .collect();
    let retarget: Vec<&str> = WORKSPACES
        .iter()
        .filter(|w| w.kind == WorkspaceKind::Retarget)
        .map(|w| w.name)
        .collect();

    assert_eq!(linking.len(), EXPECTED_CODE_LINKING_COUNT);
    assert_eq!(retarget.len(), EXPECTED_RETARGET_COUNT);
    assert_eq!(
        linking,
        vec![
            "core",
            "spark-cuda",
            "spark-metal",
            "apps",
            "derive",
            "bindings"
        ]
    );
    assert_eq!(retarget, vec!["worker"]);
}

// ============================================================================
// 3. forbidden_core_packages_cover_every_annex
// ============================================================================

#[test]
fn forbidden_core_packages_cover_every_annex() {
    // Every annex row in WORKSPACES must be justified by at least one forbidden core package entry.
    let annex_rows: BTreeSet<&str> = WORKSPACES
        .iter()
        .filter(|w| w.kind == WorkspaceKind::Annex)
        .map(|w| w.name)
        .collect();
    let covered_annexes: BTreeSet<&str> = FORBIDDEN_CORE_PACKAGES
        .iter()
        .filter_map(|entry| match entry.why {
            Some(ForbiddenReason::Annex(ws)) => Some(ws),
            _ => None,
        })
        .collect();
    assert_eq!(
        covered_annexes, annex_rows,
        "every annex workspace must be covered by FORBIDDEN_CORE_PACKAGES"
    );

    // Every required package from N34p4-02 & N21c-24 matches the table.
    for required in [
        "cudarc",
        "metal",
        "objc2-metal",
        "ft-kernel-metal",
        "block",
        "wgpu",
        "pyo3",
        "pyo3-ffi",
        "cpython",
        "numpy",
        "rustpython-vm",
        "fsym-calculus",
        "ash",
        "vulkano",
        "opencl3",
        "cubecl-core",
        "burn-core",
        "spirv-std",
        "shader-slang",
        "slang-sys",
    ] {
        assert!(
            match_forbidden_core_package(required).is_some(),
            "required forbidden package `{required}` not matched by FORBIDDEN_CORE_PACKAGES"
        );
    }

    // Annex and apps manifest paths are also recognized as forbidden in the core.
    assert_eq!(
        match_forbidden_core_manifest_dir("spark/frob-spark-cuda"),
        Some(ForbiddenReason::Annex("spark-cuda"))
    );
    assert_eq!(
        match_forbidden_core_manifest_dir("spark/frob-spark-metal"),
        Some(ForbiddenReason::Annex("spark-metal"))
    );
    assert_eq!(
        match_forbidden_core_manifest_dir("derive/frob-derive"),
        Some(ForbiddenReason::Annex("derive"))
    );
    assert_eq!(
        match_forbidden_core_manifest_dir("bindings/frob-gymnasium"),
        Some(ForbiddenReason::Annex("bindings"))
    );
    assert_eq!(
        match_forbidden_core_manifest_dir("apps/frob"),
        Some(ForbiddenReason::Annex("apps"))
    );
    assert_eq!(match_forbidden_core_manifest_dir("crates/frob-core"), None);
    assert_eq!(match_forbidden_core_manifest_dir("surface/frob-mcp"), None);

    // Known answer: the real core Cargo.lock contains zero forbidden packages.
    let lock_bytes = fs::read(repo_root().join("Cargo.lock")).expect("read core Cargo.lock");
    let lock_table = toml::parse(&lock_bytes).expect("parse core Cargo.lock");
    let mut hits = Vec::new();
    if let Some(Value::Array(pkgs)) = lock_table.get("package") {
        for item in pkgs {
            if let Some(name) = item
                .value
                .as_table()
                .and_then(|t| t.get("name"))
                .and_then(Value::as_str)
                && let Some(matched) = match_forbidden_core_package(name)
            {
                hits.push((name.to_owned(), matched.package));
            }
        }
    }
    assert!(
        hits.is_empty(),
        "real core Cargo.lock must contain 0 forbidden packages, found: {hits:?}"
    );
}

// ============================================================================
// 4. every_forbidden_entry_has_a_reason
// ============================================================================

#[test]
fn every_forbidden_entry_has_a_reason() {
    // 1. Committed table passes against the real repository (reading real registries when present).
    validate_workspace_table_against_repo(&repo_root())
        .expect("committed FORBIDDEN_CORE_PACKAGES must be valid and sorted");

    // 2. Planted entry with none of annex, route, or nongoal must fail.
    let missing_reason = [ForbiddenPackage {
        package: "rogue-crate",
        why: None,
    }];
    let errs = validate_forbidden_list(&missing_reason, WORKSPACES, None, None)
        .expect_err("entry with no reason must fail");
    assert!(
        errs.iter().any(|e| e.contains("has no reason")),
        "expected missing-reason error, got: {errs:?}"
    );

    // 3. Planted entry naming an unknown annex workspace must fail.
    let bad_annex = [ForbiddenPackage::annex("mystery-gpu", "no-such-annex")];
    let errs = validate_forbidden_list(&bad_annex, WORKSPACES, None, None)
        .expect_err("unknown annex must fail");
    assert!(
        errs.iter().any(|e| e.contains("unknown annex workspace")),
        "expected unknown-annex error, got: {errs:?}"
    );

    // 4. Route naming a row absent from a synthetic `routes_not_taken.toml` must fail;
    //    when present in the synthetic file it passes.
    let synth_routes_missing = br#"
schema = "frob.routes-not-taken/1"

[[route]]
id = "ROUTE-SPARK-WGPU"
"#;
    let route_entries = [
        ForbiddenPackage::route("burn*", "ROUTE-SPARK-RUSTGPU-BURN"),
        ForbiddenPackage::route("wgpu", "ROUTE-SPARK-WGPU"),
    ];
    let errs =
        validate_forbidden_list(&route_entries, WORKSPACES, Some(synth_routes_missing), None)
            .expect_err("missing route id must fail");
    assert!(
        errs.iter()
            .any(|e| e.contains("ROUTE-SPARK-RUSTGPU-BURN") && e.contains("routes_not_taken.toml")),
        "expected missing-route error, got: {errs:?}"
    );

    let synth_routes_complete = br#"
schema = "frob.routes-not-taken/1"

[[route]]
id = "ROUTE-SPARK-WGPU"

[[route]]
id = "ROUTE-SPARK-RUSTGPU-BURN"
"#;
    validate_forbidden_list(
        &route_entries,
        WORKSPACES,
        Some(synth_routes_complete),
        None,
    )
    .expect("complete synthetic routes_not_taken.toml must pass");

    // 5. Nongoal naming no record of a synthetic `nongoals.toml` must fail;
    //    when present in the synthetic file it passes.
    let synth_nongoals = br#"
schema = "frob.nongoals/1"

[[nongoal]]
id = "NONGOAL-01"
"#;
    let bad_nongoal = [ForbiddenPackage::nongoal("bevy_render", "NONGOAL-99")];
    let errs = validate_forbidden_list(&bad_nongoal, WORKSPACES, None, Some(synth_nongoals))
        .expect_err("missing nongoal id must fail");
    assert!(
        errs.iter()
            .any(|e| e.contains("NONGOAL-99") && e.contains("nongoals.toml")),
        "expected missing-nongoal error, got: {errs:?}"
    );

    let ok_nongoal = [ForbiddenPackage::nongoal("bevy_render", "NONGOAL-01")];
    validate_forbidden_list(&ok_nongoal, WORKSPACES, None, Some(synth_nongoals))
        .expect("matching synthetic nongoals.toml must pass");
}

// ============================================================================
// 5. prefix_pattern_matches
// ============================================================================

#[test]
fn prefix_pattern_matches() {
    assert!(package_pattern_matches("burn*", "burn-core"));
    assert!(package_pattern_matches("burn*", "burn"));
    assert!(package_pattern_matches("burn*", "burn-tensor"));
    assert!(!package_pattern_matches("burn*", "sunburn"));
    assert!(!package_pattern_matches("burn*", "my-burn-core"));

    assert!(package_pattern_matches("fsym-*", "fsym-calculus"));
    assert!(!package_pattern_matches("fsym-*", "fsym"));
    assert!(!package_pattern_matches("fsym-*", "not-fsym-calculus"));

    assert!(package_pattern_matches("cudarc", "cudarc"));
    assert!(!package_pattern_matches("cudarc", "cudarc-sys"));
    assert!(!package_pattern_matches("*", "anything"));
}

// ============================================================================
// 6. link_packages_exempt
// ============================================================================

#[test]
fn link_packages_exempt() {
    let env = SynthEnv::new("link-packages-exempt");
    env.write_workspace(&["crates/*"], &["apps"]);
    env.write_crate("crates/frob-core", "frob-core", "0", 1, "");

    // Create a synthetic apps/ workspace with the two exempt link packages (`frob`, `frobd`)
    // plus a third unlisted package `apps/frob-extra`.
    write_file(
        &env.repo.join("apps/Cargo.toml"),
        "[workspace]\nresolver = \"3\"\nmembers = [\"frob\", \"frobd\", \"frob-extra\"]\n",
    );
    env.write_bin_crate(
        "apps/frob",
        "frob",
        "[dependencies]\nfrob-core = { path = \"../../crates/frob-core\" }",
    );
    env.write_bin_crate(
        "apps/frobd",
        "frobd",
        "[dependencies]\nfrob-core = { path = \"../../crates/frob-core\" }",
    );
    env.write_bin_crate(
        "apps/frob-extra",
        "frob-extra",
        "[dependencies]\nfrob-core = { path = \"../../crates/frob-core\" }",
    );
    env.generate_lockfile(&env.repo);
    env.generate_lockfile(&env.repo.join("apps"));

    // Direct crate_map::validate check using workspaces::link_packages().
    let apps_members = vec![
        WorkspaceMember {
            name: "frob".to_owned(),
            rel_dir: "apps/frob".to_owned(),
            manifest_path: env.repo.join("apps/frob/Cargo.toml"),
        },
        WorkspaceMember {
            name: "frobd".to_owned(),
            rel_dir: "apps/frobd".to_owned(),
            manifest_path: env.repo.join("apps/frobd/Cargo.toml"),
        },
        WorkspaceMember {
            name: "frob-extra".to_owned(),
            rel_dir: "apps/frob-extra".to_owned(),
            manifest_path: env.repo.join("apps/frob-extra/Cargo.toml"),
        },
    ];
    let direct_findings = validate_crate_map(&apps_members, &link_packages());
    assert_eq!(
        direct_findings.len(),
        1,
        "only apps/frob-extra should fail validation, got: {direct_findings:?}"
    );
    assert_eq!(direct_findings[0].rule, RULE_UNLISTED_CRATE);
    assert_eq!(direct_findings[0].subject, "frob-extra");

    // End-to-end check_layers wired with workspaces::workspace_roots(&env.repo).
    let roots = workspace_roots(&env.repo);
    let outcome = check_layers(&env.repo, &roots).expect("check_layers on synthetic apps/");
    assert_eq!(
        outcome.findings.len(),
        1,
        "frob and frobd must pass; only frob-extra must fail, got: {:?}",
        outcome.findings
    );
    assert_eq!(outcome.findings[0].rule, RULE_UNLISTED_CRATE);
    assert_eq!(outcome.findings[0].subject, "frob-extra");
    assert_eq!(outcome.findings[0].path, "apps/frob-extra/Cargo.toml");
}

// ============================================================================
// 7. absent_root_is_fact_not_clean
// ============================================================================

#[test]
fn absent_root_is_fact_not_clean() {
    let env = SynthEnv::new("absent-root-fact");
    env.write_workspace(
        &["crates/*"],
        &["spark", "apps", "derive", "bindings", "worker"],
    );
    env.write_crate("crates/frob-core", "frob-core", "0", 1, "");
    env.write_stack_lock(&cudarc_foundation_toml());
    env.generate_lockfile(&env.repo);

    let roots = workspace_roots(&env.repo);
    assert_eq!(roots.len(), EXPECTED_WORKSPACE_COUNT);

    let expected_absent = vec![
        "apps".to_owned(),
        "bindings".to_owned(),
        "derive".to_owned(),
        "spark-cuda".to_owned(),
        "spark-metal".to_owned(),
        "worker".to_owned(),
    ];

    // Both `check-layers` and `check-deps` report every workspace without a manifest under
    // `facts.workspaces_absent` and NEVER under `facts.workspaces_checked`.
    let layers_out = check_layers(&env.repo, &roots).expect("check_layers");
    assert!(layers_out.findings.is_empty(), "{:?}", layers_out.findings);
    assert_eq!(
        layers_out.facts.get("workspaces_checked"),
        Some(&FactValue::str_array(vec!["core".to_owned()]))
    );
    assert_eq!(
        layers_out.facts.get("workspaces_absent"),
        Some(&FactValue::str_array(expected_absent.clone()))
    );

    let deps_out = check_deps(&env.repo, &roots).expect("check_deps");
    assert!(deps_out.findings.is_empty(), "{:?}", deps_out.findings);
    assert_eq!(
        deps_out.facts.get("workspaces_checked"),
        Some(&FactValue::str_array(vec!["core".to_owned()]))
    );
    assert_eq!(
        deps_out.facts.get("workspaces_absent"),
        Some(&FactValue::str_array(expected_absent))
    );
}

// ============================================================================
// 8. forbidden_package_in_core_closure
// ============================================================================

#[test]
fn forbidden_package_in_core_closure() {
    // Case A: Transitive `cudarc` in core (`frob-core -> ext-helper -> cudarc`)
    // -> `core.forbidden-package` naming `cudarc`, `annex = "spark-cuda"`, and 3-step path.
    let trans = SynthEnv::new("core-transitive-cudarc");
    trans.write_workspace(&["crates/*"], &[]);
    trans.add_reg_crate("cudarc", "0.19.9", "");
    trans.write_sibling_crate(
        "ext-helper",
        "ext-helper",
        "0.1.0",
        "[dependencies]\ncudarc = \"=0.19.9\"",
    );
    trans.write_crate(
        "crates/frob-core",
        "frob-core",
        "0",
        1,
        "[dependencies]\next-helper = { path = \"../../../ext-helper\", default-features = false }",
    );
    trans.write_stack_lock(&format!(
        "{}\n{}",
        cudarc_foundation_toml(),
        helper_foundation_toml()
    ));
    trans.generate_lockfile(&trans.repo);

    let trans_out = check_deps(&trans.repo, &[WorkspaceRoot::core(&trans.repo)])
        .expect("check_deps transitive cudarc");
    let forb: Vec<_> = trans_out
        .findings
        .iter()
        .filter(|f| f.rule == RULE_CORE_FORBIDDEN_PACKAGE)
        .collect();
    assert_eq!(
        forb.len(),
        1,
        "expected exactly 1 core.forbidden-package finding, got: {:?}",
        trans_out.findings
    );
    assert_eq!(forb[0].subject, "frob-core -> ext-helper -> cudarc");
    assert!(
        forb[0].message.contains("cudarc")
            && forb[0].message.contains("annex = \"spark-cuda\"")
            && forb[0]
                .message
                .contains("frob-core -> ext-helper -> cudarc"),
        "unexpected transitive cudarc message: {}",
        forb[0].message
    );

    // Case B: Prefix pattern `burn*` matches `burn-core` in the core closure and names its route.
    let burn = SynthEnv::new("core-burn-core");
    burn.write_workspace(&["crates/*"], &[]);
    burn.add_reg_crate("burn-core", "0.16.0", "");
    burn.write_sibling_crate(
        "ext-helper",
        "ext-helper",
        "0.1.0",
        "[dependencies]\nburn-core = \"=0.16.0\"",
    );
    burn.write_crate(
        "crates/frob-core",
        "frob-core",
        "0",
        1,
        "[dependencies]\next-helper = { path = \"../../../ext-helper\", default-features = false }",
    );
    burn.write_stack_lock(&helper_foundation_toml());
    burn.generate_lockfile(&burn.repo);

    let burn_out =
        check_deps(&burn.repo, &[WorkspaceRoot::core(&burn.repo)]).expect("check_deps burn-core");
    let burn_forb: Vec<_> = burn_out
        .findings
        .iter()
        .filter(|f| f.rule == RULE_CORE_FORBIDDEN_PACKAGE)
        .collect();
    assert_eq!(
        burn_forb.len(),
        1,
        "expected 1 core.forbidden-package finding for burn-core, got: {:?}",
        burn_out.findings
    );
    assert_eq!(burn_forb[0].subject, "frob-core -> ext-helper -> burn-core");
    assert!(
        burn_forb[0].message.contains("burn-core")
            && burn_forb[0]
                .message
                .contains("route = \"ROUTE-SPARK-RUSTGPU-BURN\""),
        "expected route tag for burn-core, got: {}",
        burn_forb[0].message
    );

    // Case C: Package present in Cargo.lock and reached by no edge -> `core.forbidden-package`
    // with `lockfile-only`.
    let lock_only = SynthEnv::new("core-lockfile-only-cudarc");
    lock_only.write_workspace(&["crates/*"], &[]);
    lock_only.write_crate("crates/frob-core", "frob-core", "0", 1, "");
    lock_only.write_stack_lock(&cudarc_foundation_toml());
    lock_only.generate_lockfile(&lock_only.repo);
    let mut lock_text =
        fs::read_to_string(lock_only.repo.join("Cargo.lock")).expect("read Cargo.lock");
    lock_text.push_str("\n[[package]]\nname = \"cudarc\"\nversion = \"0.19.9\"\n");
    write_file(&lock_only.repo.join("Cargo.lock"), &lock_text);

    let lock_out = check_deps(&lock_only.repo, &[WorkspaceRoot::core(&lock_only.repo)])
        .expect("check_deps lockfile-only cudarc");
    let lock_forb: Vec<_> = lock_out
        .findings
        .iter()
        .filter(|f| f.rule == RULE_CORE_FORBIDDEN_PACKAGE)
        .collect();
    assert_eq!(
        lock_forb.len(),
        1,
        "expected 1 lockfile-only finding, got: {:?}",
        lock_out.findings
    );
    assert!(
        lock_forb[0].subject.contains("cudarc") && lock_forb[0].subject.contains("lockfile-only"),
        "expected lockfile-only subject, got: {}",
        lock_forb[0].subject
    );
    assert!(
        lock_forb[0].message.contains("annex = \"spark-cuda\"")
            && lock_forb[0].message.contains("lockfile-only"),
        "expected annex tag and lockfile-only in message: {}",
        lock_forb[0].message
    );

    // Case D: Direct `cudarc` in a core crate triggers BOTH L-4.foundation-confined (edge subject)
    // AND `core.forbidden-package` naming `cudarc` with `annex = "spark-cuda"`.
    let direct = SynthEnv::new("core-direct-cudarc");
    direct.write_workspace(&["crates/*"], &[]);
    direct.add_reg_crate("cudarc", "0.19.9", "");
    direct.write_crate(
        "crates/frob-core",
        "frob-core",
        "0",
        1,
        "[dependencies]\ncudarc = { version = \"=0.19.9\", default-features = false }",
    );
    direct.write_stack_lock(&cudarc_foundation_toml());
    direct.generate_lockfile(&direct.repo);

    let direct_out = check_deps(&direct.repo, &[WorkspaceRoot::core(&direct.repo)])
        .expect("check_deps direct cudarc");
    assert!(
        direct_out
            .findings
            .iter()
            .any(|f| f.rule == RULE_L4_FOUNDATION_CONFINED && f.subject == "frob-core -> cudarc"),
        "expected L-4.foundation-confined on frob-core -> cudarc, got: {:?}",
        direct_out.findings
    );
    assert!(
        direct_out.findings.iter().any(|f| {
            f.rule == RULE_CORE_FORBIDDEN_PACKAGE
                && f.message.contains("cudarc")
                && f.message.contains("annex = \"spark-cuda\"")
        }),
        "expected core.forbidden-package naming cudarc and annex = \"spark-cuda\", got: {:?}",
        direct_out.findings
    );

    // Case E: `cudarc` in the `spark-cuda` annex workspace raises NO `core.forbidden-package`
    // (and passes check-deps cleanly when admitted by FD-CUDA).
    let annex = SynthEnv::new("annex-spark-cuda-ok");
    annex.write_workspace(&["crates/*"], &["spark"]);
    annex.write_crate("crates/frob-core", "frob-core", "0", 1, "");
    annex.add_reg_crate("cudarc", "0.19.9", "");
    annex.write_crate(
        "spark/frob-spark-cuda",
        "frob-spark-cuda",
        "\"annex\"",
        2,
        "[workspace]\n\n[dependencies]\ncudarc = { version = \"=0.19.9\", default-features = false }",
    );
    annex.write_stack_lock(&cudarc_foundation_toml());
    annex.generate_lockfile(&annex.repo);
    annex.generate_lockfile(&annex.repo.join("spark/frob-spark-cuda"));

    let annex_roots = workspace_roots(&annex.repo);
    let annex_out = check_deps(&annex.repo, &annex_roots).expect("check_deps annex spark-cuda");
    assert!(
        annex_out.findings.is_empty(),
        "cudarc inside spark-cuda annex must raise zero findings, got: {:?}",
        annex_out.findings
    );

    // Case F: Clean real repository tree passes `check-deps` and `check-layers` via `cli::run`
    // with zero findings and absent workspaces recorded under `facts.workspaces_absent`.
    let cmds: &[(&str, CheckFn)] = &[
        ("check-layers", check_layers::run_command),
        ("check-deps", check_deps::run_command),
    ];
    for subcmd in ["check-layers", "check-deps"] {
        let mut out = Vec::new();
        let mut err = Vec::new();
        let args = vec![
            subcmd.to_owned(),
            "--root".to_owned(),
            repo_root().to_string_lossy().into_owned(),
            "--json".to_owned(),
        ];
        let code = cli::run(cmds, &args, &mut out, &mut err);
        assert_eq!(
            code,
            ExitCode::from(EXIT_OK),
            "{subcmd} on clean tree failed: stdout={}, stderr={}",
            String::from_utf8_lossy(&out),
            String::from_utf8_lossy(&err)
        );
        let admitted = admit(&out, &Limits::default()).expect("admit JSON report");
        assert_eq!(
            admitted.value().get("ok").and_then(|v| v.as_bool()),
            Some(true)
        );
    }
}

// ============================================================================
// 9. Toolchain-file rules (N34p4-02 item 4)
// ============================================================================

#[test]
fn toolchain_file_rules_on_clean_tree_and_plants() {
    let found = check_toolchain_files(&repo_root(), false)
        .expect("clean tree must satisfy the one-toolchain-file rule");
    assert_eq!(
        found,
        vec![
            CERT_TOOLCHAIN_FILE.to_owned(),
            ROOT_TOOLCHAIN_FILE.to_owned()
        ]
    );

    // Planted stray toolchain file under `spark/frob-spark-cuda/` fails.
    let stray = scratch_dir("stray-toolchain");
    write_file(
        &stray.join(ROOT_TOOLCHAIN_FILE),
        "[toolchain]\nchannel = \"nightly\"\n",
    );
    write_file(
        &stray.join(CERT_TOOLCHAIN_FILE),
        "[toolchain]\nchannel = \"stable\"\n",
    );
    write_file(
        &stray.join("spark/frob-spark-cuda/rust-toolchain.toml"),
        "[toolchain]\nchannel = \"nightly\"\n",
    );
    let errs = check_toolchain_files(&stray, false)
        .expect_err("stray annex rust-toolchain.toml must fail");
    assert!(
        errs.iter()
            .any(|e| e.contains("spark/frob-spark-cuda/rust-toolchain.toml")),
        "expected stray spark toolchain error, got: {errs:?}"
    );

    // `worker/rust-toolchain.toml` is rejected when worker fallback is inactive and accepted when active.
    let worker = scratch_dir("worker-toolchain");
    write_file(
        &worker.join(ROOT_TOOLCHAIN_FILE),
        "[toolchain]\nchannel = \"nightly\"\n",
    );
    write_file(
        &worker.join(WORKER_TOOLCHAIN_FILE),
        "[toolchain]\nchannel = \"nightly\"\n",
    );
    assert!(check_toolchain_files(&worker, false).is_err());
    assert!(check_toolchain_files(&worker, true).is_ok());
}
