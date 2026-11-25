//! Integration and property tests for `xtask` CLI surface, `frob.xtask.report/1`,
//! and `check-layers` (bead N34p4-01, INV-WS-01, CX-363, KA-784 layer half).
//!
//! Synthetic scratch workspaces are created under
//! `${BIG_SCRATCH:-${TMPDIR:-/tmp}}/frob-BrownHeron/` and never deleted (COMMON.md 9b).
//! Every workspace check runs through the real `cargo metadata --no-deps --locked --offline`
//! reader, `frob_core::json::admit`, and `frob_core::registry::toml::parse`.

use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use frob_core::json::{Limits, admit};
use xtask::check_layers::{
    self, GraphEdge, RULE_INVERSION_WITHOUT_ADR, RULE_L1_RANK, RULE_L1_UPWARD,
    RULE_L3_ANNEX_TO_ANNEX, RULE_L3_CORE_TO_ANNEX, RULE_L11_CONTROLLER_IN_COMPUTE_PLANE,
    RULE_METADATA_MISMATCH, RULE_UNLISTED_CRATE, RULE_UNLISTED_IMPLEMENTOR,
    RULE_UNREGISTERED_INVERSION, RULE_WRONG_HOME, check_edges, check_layers,
    check_layers_with_tables,
};
use xtask::cli::{self, CheckFn, EXIT_FINDINGS, EXIT_OK, EXIT_USAGE};
use xtask::crate_map::{CRATE_MAP, CrateRow, Layer, Reason, TOMBSTONES, Tier};
use xtask::inversions::{InversionAuthority, InversionRow};
use xtask::metadata::{
    DepKind, MetadataLoadError, OFFLINE_FETCH_REPAIR, WorkspaceRoot,
    looks_like_offline_or_network_error,
};
use xtask::report::{CheckReport, FactValue, Finding, REPORT_SCHEMA, validate_report_json};

static COUNTER: AtomicU64 = AtomicU64::new(0);

const TEST_COMMANDS: &[(&str, CheckFn)] = &[("check-layers", check_layers::run_command)];

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crates/xtask sits two levels below repo root")
        .to_path_buf()
}

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(name)
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
        .join(format!("xtask-check-layers-{label}-{ts}-{pid}-{n}"));
    fs::create_dir_all(&dir).expect("create scratch dir");
    dir
}

fn write_file(path: &Path, content: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("create parent dirs");
    }
    fs::write(path, content).expect("write synthetic fixture file");
}

fn write_workspace(root: &Path, members: &[&str], exclude: &[&str]) {
    let m: Vec<String> = members.iter().map(|s| format!("\"{s}\"")).collect();
    let mut toml = format!(
        "# synthetic workspace for N34p4-01\n[workspace]\nresolver = \"3\"\nmembers = [{}]\n",
        m.join(", ")
    );
    if !exclude.is_empty() {
        let e: Vec<String> = exclude.iter().map(|s| format!("\"{s}\"")).collect();
        toml.push_str(&format!("exclude = [{}]\n", e.join(", ")));
    }
    write_file(&root.join("Cargo.toml"), &toml);
}

fn write_crate(
    root: &Path,
    rel_dir: &str,
    name: &str,
    layer_toml: &str,
    rank: u8,
    extra_toml: &str,
    lib_rs: &str,
) {
    let dir = root.join(rel_dir);
    let meta = if layer_toml.is_empty() {
        String::new()
    } else {
        format!("[package.metadata.frob]\nlayer = {layer_toml}\nrank = {rank}\n")
    };
    let manifest = format!(
        "# synthetic crate for N34p4-01\n[package]\nname = \"{name}\"\nversion = \"0.0.1\"\nedition = \"2024\"\n\n{meta}\n{extra_toml}\n"
    );
    write_file(&dir.join("Cargo.toml"), &manifest);
    write_file(
        &dir.join("src").join("lib.rs"),
        if lib_rs.is_empty() {
            "// synthetic crate source\n"
        } else {
            lib_rs
        },
    );
}

fn rules_of(findings: &[Finding]) -> Vec<&'static str> {
    findings.iter().map(|f| f.rule).collect()
}

#[test]
fn upward_edge_fails() {
    let root = make_scratch_dir("upward-edge");
    write_workspace(&root, &["crates/*"], &[]);
    write_crate(
        &root,
        "crates/frob-lie",
        "frob-lie",
        "1",
        1,
        "[dependencies]\nfrob-design = { path = \"../frob-design\" }\n",
        "",
    );
    write_crate(&root, "crates/frob-design", "frob-design", "2", 2, "", "");

    let outcome = check_layers(&root, &[WorkspaceRoot::core(&root)]).expect("check_layers");
    assert_eq!(rules_of(&outcome.findings), vec![RULE_L1_UPWARD]);
    assert_eq!(outcome.findings[0].subject, "frob-lie -> frob-design");
    assert_eq!(outcome.findings[0].path, "crates/frob-lie/Cargo.toml");

    // Verify CLI exits 2 with valid JSON report on the same scratch copy.
    let mut out = Vec::new();
    let mut err = Vec::new();
    let args = vec![
        "check-layers".to_owned(),
        "--root".to_owned(),
        root.to_string_lossy().into_owned(),
        "--json".to_owned(),
    ];
    let code = cli::run(TEST_COMMANDS, &args, &mut out, &mut err);
    assert_eq!(code, ExitCode::from(EXIT_FINDINGS));
    assert!(err.is_empty(), "unexpected stderr: {}", String::from_utf8_lossy(&err));
    validate_report_json(&out).expect("valid frob.xtask.report/1");
}

#[test]
fn rank_edge_fails() {
    let root = make_scratch_dir("rank-edge");
    write_workspace(&root, &["crates/*"], &[]);
    write_crate(
        &root,
        "crates/frob-arena",
        "frob-arena",
        "4",
        6,
        "[dependencies]\nfrob-opt = { path = \"../frob-opt\" }\n",
        "",
    );
    write_crate(&root, "crates/frob-opt", "frob-opt", "4", 7, "", "");

    let outcome = check_layers(&root, &[WorkspaceRoot::core(&root)]).expect("check_layers");
    assert_eq!(rules_of(&outcome.findings), vec![RULE_L1_RANK]);
    assert_eq!(outcome.findings[0].subject, "frob-arena -> frob-opt");
    assert_eq!(outcome.findings[0].path, "crates/frob-arena/Cargo.toml");

    let mut out = Vec::new();
    let mut err = Vec::new();
    let args = vec![
        "check-layers".to_owned(),
        "--root".to_owned(),
        root.to_string_lossy().into_owned(),
        "--json".to_owned(),
    ];
    let code = cli::run(TEST_COMMANDS, &args, &mut out, &mut err);
    assert_eq!(code, ExitCode::from(EXIT_FINDINGS));
    validate_report_json(&out).expect("valid report");
}

#[test]
fn same_layer_lower_rank_passes() {
    let root = make_scratch_dir("same-layer-lower-rank");
    write_workspace(&root, &["crates/*"], &[]);
    write_crate(&root, "crates/frob-lie", "frob-lie", "1", 1, "", "");
    write_crate(
        &root,
        "crates/frob-algebra",
        "frob-algebra",
        "1",
        2,
        "[dependencies]\nfrob-lie = { path = \"../frob-lie\" }\n",
        "",
    );

    let outcome = check_layers(&root, &[WorkspaceRoot::core(&root)]).expect("check_layers");
    assert!(
        outcome.findings.is_empty(),
        "expected no findings, got {:?}",
        outcome.findings
    );
}

#[test]
fn dev_dependency_upward_fails() {
    let root = make_scratch_dir("dev-dep-upward");
    write_workspace(&root, &["crates/*"], &[]);
    write_crate(
        &root,
        "crates/frob-lie",
        "frob-lie",
        "1",
        1,
        "[dev-dependencies]\nfrob-design = { path = \"../frob-design\" }\n",
        "",
    );
    write_crate(&root, "crates/frob-design", "frob-design", "2", 2, "", "");

    let outcome = check_layers(&root, &[WorkspaceRoot::core(&root)]).expect("check_layers");
    assert_eq!(rules_of(&outcome.findings), vec![RULE_L1_UPWARD]);
    assert!(outcome.findings[0].message.contains("dev"));

    // Build-dependencies upward must likewise fail L-1.upward.
    let root_build = make_scratch_dir("build-dep-upward");
    write_workspace(&root_build, &["crates/*"], &[]);
    write_crate(
        &root_build,
        "crates/frob-lie",
        "frob-lie",
        "1",
        1,
        "[build-dependencies]\nfrob-design = { path = \"../frob-design\" }\n",
        "",
    );
    write_crate(&root_build, "crates/frob-design", "frob-design", "2", 2, "", "");
    let build_outcome =
        check_layers(&root_build, &[WorkspaceRoot::core(&root_build)]).expect("check_layers");
    assert_eq!(rules_of(&build_outcome.findings), vec![RULE_L1_UPWARD]);
    assert!(build_outcome.findings[0].message.contains("build"));
}

#[test]
fn cx363_arena_opt_cycle() {
    // Allowed direction: frob-opt (4.7) -> frob-arena (4.6) passes cleanly.
    let ok_root = make_scratch_dir("cx363-opt-to-arena");
    write_workspace(&ok_root, &["crates/*"], &[]);
    write_crate(&ok_root, "crates/frob-arena", "frob-arena", "4", 6, "", "");
    write_crate(
        &ok_root,
        "crates/frob-opt",
        "frob-opt",
        "4",
        7,
        "[dependencies]\nfrob-arena = { path = \"../frob-arena\" }\n",
        "",
    );
    let ok_outcome = check_layers(&ok_root, &[WorkspaceRoot::core(&ok_root)]).expect("ok");
    assert!(
        ok_outcome.findings.is_empty(),
        "frob-opt -> frob-arena must pass: {:?}",
        ok_outcome.findings
    );

    // Forbidden reverse edge on its own (before any cycle exists): frob-arena -> frob-opt fails L-1.rank.
    let bad_root = make_scratch_dir("cx363-arena-to-opt");
    write_workspace(&bad_root, &["crates/*"], &[]);
    write_crate(
        &bad_root,
        "crates/frob-arena",
        "frob-arena",
        "4",
        6,
        "[dependencies]\nfrob-opt = { path = \"../frob-opt\" }\n",
        "",
    );
    write_crate(&bad_root, "crates/frob-opt", "frob-opt", "4", 7, "", "");
    let bad_outcome = check_layers(&bad_root, &[WorkspaceRoot::core(&bad_root)]).expect("bad");
    assert_eq!(rules_of(&bad_outcome.findings), vec![RULE_L1_RANK]);
    assert_eq!(bad_outcome.findings[0].subject, "frob-arena -> frob-opt");
}

#[test]
fn core_to_annex_fails() {
    let root = make_scratch_dir("core-to-annex");
    write_workspace(&root, &["crates/*"], &["spark"]);
    write_crate(
        &root,
        "crates/frob-kernels",
        "frob-kernels",
        "3",
        1,
        "[dependencies]\nfrob-spark-cuda = { path = \"../../spark/frob-spark-cuda\" }\n",
        "",
    );
    write_crate(
        &root,
        "spark/frob-spark-cuda",
        "frob-spark-cuda",
        "\"annex\"",
        1,
        "",
        "",
    );

    let outcome = check_layers(&root, &[WorkspaceRoot::core(&root)]).expect("check_layers");
    assert_eq!(rules_of(&outcome.findings), vec![RULE_L3_CORE_TO_ANNEX]);
    assert_eq!(outcome.findings[0].subject, "frob-kernels -> frob-spark-cuda");
}

#[test]
fn annex_to_core_passes() {
    let root = make_scratch_dir("annex-to-core");
    write_workspace(&root, &["crates/*"], &["spark"]);
    write_crate(&root, "crates/frob-core", "frob-core", "0", 1, "", "");
    write_crate(
        &root,
        "spark/frob-spark-cuda",
        "frob-spark-cuda",
        "\"annex\"",
        1,
        "[workspace]\n\n[dependencies]\nfrob-core = { path = \"../../crates/frob-core\" }\n",
        "",
    );

    let roots = vec![
        WorkspaceRoot::core(&root),
        WorkspaceRoot::annex(
            "spark-cuda",
            "spark/frob-spark-cuda",
            root.join("spark/frob-spark-cuda"),
        ),
    ];
    let outcome = check_layers(&root, &roots).expect("check_layers");
    assert!(
        outcome.findings.is_empty(),
        "annex -> core must pass, got {:?}",
        outcome.findings
    );
}

#[test]
fn annex_to_annex_fails() {
    let root = make_scratch_dir("annex-to-annex");
    write_workspace(&root, &["crates/*"], &["spark", "apps"]);
    write_crate(&root, "crates/frob-core", "frob-core", "0", 1, "", "");
    write_crate(
        &root,
        "spark/frob-spark-metal",
        "frob-spark-metal",
        "\"annex\"",
        2,
        "[workspace]\n",
        "",
    );
    write_crate(
        &root,
        "spark/frob-spark-cuda",
        "frob-spark-cuda",
        "\"annex\"",
        1,
        "[workspace]\n\n[dependencies]\nfrob-spark-metal = { path = \"../frob-spark-metal\" }\n",
        "",
    );

    let roots = vec![
        WorkspaceRoot::core(&root),
        WorkspaceRoot::annex(
            "spark-cuda",
            "spark/frob-spark-cuda",
            root.join("spark/frob-spark-cuda"),
        ),
        WorkspaceRoot::annex(
            "spark-metal",
            "spark/frob-spark-metal",
            root.join("spark/frob-spark-metal"),
        ),
    ];
    let outcome = check_layers(&root, &roots).expect("check_layers");
    assert_eq!(rules_of(&outcome.findings), vec![RULE_L3_ANNEX_TO_ANNEX]);
    assert_eq!(
        outcome.findings[0].subject,
        "frob-spark-cuda -> frob-spark-metal"
    );

    // By contrast, an exempt link package under apps/ linking core + both annexes passes.
    let apps_root = make_scratch_dir("apps-link-annexes");
    write_workspace(&apps_root, &["crates/*"], &["spark", "apps"]);
    write_crate(&apps_root, "crates/frob-core", "frob-core", "0", 1, "", "");
    write_crate(
        &apps_root,
        "spark/frob-spark-cuda",
        "frob-spark-cuda",
        "\"annex\"",
        1,
        "[workspace]\n",
        "",
    );
    write_crate(
        &apps_root,
        "spark/frob-spark-metal",
        "frob-spark-metal",
        "\"annex\"",
        2,
        "[workspace]\n",
        "",
    );
    write_workspace(&apps_root.join("apps"), &["frob"], &[]);
    write_crate(
        &apps_root,
        "apps/frob",
        "frob",
        "",
        0,
        "[dependencies]\nfrob-core = { path = \"../../crates/frob-core\" }\nfrob-spark-cuda = { path = \"../../spark/frob-spark-cuda\" }\nfrob-spark-metal = { path = \"../../spark/frob-spark-metal\" }\n",
        "",
    );
    let link_roots = vec![
        WorkspaceRoot::core(&apps_root),
        WorkspaceRoot::link(
            "apps",
            "apps",
            apps_root.join("apps"),
            vec!["frob".to_owned(), "frobd".to_owned()],
        ),
    ];
    let apps_outcome = check_layers(&apps_root, &link_roots).expect("apps link check");
    assert!(
        apps_outcome.findings.is_empty(),
        "apps/ link package linking core+annexes must pass, got {:?}",
        apps_outcome.findings
    );
}

#[test]
fn unlisted_crate_fails() {
    let root = make_scratch_dir("unlisted-crate");
    write_workspace(&root, &["crates/*"], &[]);
    write_crate(&root, "crates/frob-core", "frob-core", "0", 1, "", "");
    write_crate(&root, "crates/frob-extra", "frob-extra", "2", 6, "", "");

    let outcome = check_layers(&root, &[WorkspaceRoot::core(&root)]).expect("check_layers");
    assert_eq!(rules_of(&outcome.findings), vec![RULE_UNLISTED_CRATE]);
    assert_eq!(outcome.findings[0].subject, "frob-extra");
    assert_eq!(outcome.findings[0].path, "crates/frob-extra/Cargo.toml");
}

#[test]
fn metadata_mismatch_fails() {
    let root = make_scratch_dir("metadata-mismatch");
    write_workspace(&root, &["crates/*"], &[]);
    // frob-dyn is layer 3, rank 3 in CRATE_MAP; declaring layer = 2 must fail.
    write_crate(&root, "crates/frob-dyn", "frob-dyn", "2", 3, "", "");

    let outcome = check_layers(&root, &[WorkspaceRoot::core(&root)]).expect("check_layers");
    assert_eq!(rules_of(&outcome.findings), vec![RULE_METADATA_MISMATCH]);
    assert_eq!(outcome.findings[0].subject, "frob-dyn");
    assert_eq!(outcome.findings[0].path, "crates/frob-dyn/Cargo.toml");
}

#[test]
fn l10_rules() {
    // Two-row synthetic inversion register:
    // 1. evidence-sink defined in frob-core, allowed in frob-chronicle.
    // 2. device-backend defined in frob-kernels, allowed in frob-spark-cuda (trait not yet in fixture!).
    const SYNTH_ROWS: &[InversionRow] = &[
        InversionRow {
            id: "evidence-sink",
            trait_path: "frob_core::ports::EvidenceSink",
            defining_crate: "frob-core",
            allowed_implementors: &["frob-chronicle"],
            authority: InversionAuthority::Standing,
        },
        InversionRow {
            id: "device-backend",
            trait_path: "frob_kernels::KernelBackend",
            defining_crate: "frob-kernels",
            allowed_implementors: &["frob-spark-cuda"],
            authority: InversionAuthority::Standing,
        },
    ];

    // 1. Conforming fixture:
    //    - marker + trait + null impl in defining crate frob-core -> passes
    //    - allowed impl in frob-chronicle -> passes
    //    - row `device-backend` whose trait no crate defines -> passes
    //    - doc comments and string literals mentioning markers/impls -> ignored (no false positive)
    let ok_root = make_scratch_dir("l10-conforming");
    write_workspace(&ok_root, &["crates/*"], &[]);
    write_crate(
        &ok_root,
        "crates/frob-core",
        "frob-core",
        "0",
        1,
        "",
        "\
// frob:inversion evidence-sink
pub trait EvidenceSink {
    fn record(&self);
}
pub struct Null;
impl EvidenceSink for Null {
    fn record(&self) {}
}
/// Doc mention of // frob:inversion nope must not trigger.
pub const DOC_STR: &str = \"// frob:inversion nope\\nimpl EvidenceSink for Fake {}\";
",
    );
    write_crate(
        &ok_root,
        "crates/frob-chronicle",
        "frob-chronicle",
        "6",
        2,
        "[dependencies]\nfrob-core = { path = \"../frob-core\" }\n",
        "\
use frob_core::EvidenceSink;
pub struct LedgerSink;
impl<'a> EvidenceSink for &'a LedgerSink {
    fn record(&self) {}
}
",
    );
    write_crate(
        &ok_root,
        "crates/frob-control",
        "frob-control",
        "4",
        2,
        "[dependencies]\nfrob-core = { path = \"../frob-core\" }\n",
        "\
use frob_core::{EvidenceSink, Null};
pub fn helper() -> impl EvidenceSink { Null }
",
    );
    let ok_out = check_layers_with_tables(
        &ok_root,
        &[WorkspaceRoot::core(&ok_root)],
        CRATE_MAP,
        TOMBSTONES,
        SYNTH_ROWS,
    )
    .expect("conforming L-10");
    assert!(
        ok_out.findings.is_empty(),
        "conforming L-10 fixture must pass, got {:?}",
        ok_out.findings
    );

    // 2. Unregistered marker: `// frob:inversion nope` -> L-10.unregistered-inversion.
    let bad_marker = make_scratch_dir("l10-unregistered-marker");
    write_workspace(&bad_marker, &["crates/*"], &[]);
    write_crate(
        &bad_marker,
        "crates/frob-core",
        "frob-core",
        "0",
        1,
        "",
        "// frob:inversion nope\npub trait Nope {}\n",
    );
    let out_marker = check_layers_with_tables(
        &bad_marker,
        &[WorkspaceRoot::core(&bad_marker)],
        CRATE_MAP,
        TOMBSTONES,
        SYNTH_ROWS,
    )
    .expect("unregistered marker");
    assert_eq!(rules_of(&out_marker.findings), vec![RULE_UNREGISTERED_INVERSION]);
    assert_eq!(out_marker.findings[0].subject, "nope");
    assert_eq!(out_marker.findings[0].line, Some(1));

    // 3. Unlisted implementor: `impl EvidenceSink for X` in frob-control -> L-10.unlisted-implementor.
    let bad_impl = make_scratch_dir("l10-unlisted-implementor");
    write_workspace(&bad_impl, &["crates/*"], &[]);
    write_crate(
        &bad_impl,
        "crates/frob-core",
        "frob-core",
        "0",
        1,
        "",
        "// frob:inversion evidence-sink\npub trait EvidenceSink {}\n",
    );
    write_crate(
        &bad_impl,
        "crates/frob-control",
        "frob-control",
        "4",
        2,
        "[dependencies]\nfrob-core = { path = \"../frob-core\" }\n",
        "pub struct X;\nimpl frob_core::EvidenceSink for X {}\npub struct Y;\nimpl for<'a> frob_core::EvidenceSink for &'a Y {}\n",
    );
    let out_impl = check_layers_with_tables(
        &bad_impl,
        &[WorkspaceRoot::core(&bad_impl)],
        CRATE_MAP,
        TOMBSTONES,
        SYNTH_ROWS,
    )
    .expect("unlisted implementor");
    assert_eq!(
        rules_of(&out_impl.findings),
        vec![RULE_UNLISTED_IMPLEMENTOR, RULE_UNLISTED_IMPLEMENTOR]
    );
    assert_eq!(out_impl.findings[0].path, "crates/frob-control/src/lib.rs");
    assert_eq!(out_impl.findings[0].line, Some(2));
    assert_eq!(out_impl.findings[1].line, Some(4));

    // 4. Trait defined in wrong crate -> L-10.wrong-home.
    let bad_home = make_scratch_dir("l10-wrong-home");
    write_workspace(&bad_home, &["crates/*"], &[]);
    write_crate(
        &bad_home,
        "crates/frob-control",
        "frob-control",
        "4",
        2,
        "",
        "pub trait EvidenceSink {}\n",
    );
    let out_home = check_layers_with_tables(
        &bad_home,
        &[WorkspaceRoot::core(&bad_home)],
        CRATE_MAP,
        TOMBSTONES,
        SYNTH_ROWS,
    )
    .expect("wrong home");
    assert_eq!(rules_of(&out_home.findings), vec![RULE_WRONG_HOME]);
    assert_eq!(out_home.findings[0].path, "crates/frob-control/src/lib.rs");
    assert_eq!(out_home.findings[0].line, Some(1));

    // 5. Third row with neither `standing` nor an existing `adr` file -> L-10.inversion-without-adr.
    const ROWS_MISSING_ADR: &[InversionRow] = &[
        SYNTH_ROWS[0],
        SYNTH_ROWS[1],
        InversionRow {
            id: "rogue-none",
            trait_path: "frob_core::RogueA",
            defining_crate: "frob-core",
            allowed_implementors: &["frob-verbs"],
            authority: InversionAuthority::None,
        },
        InversionRow {
            id: "rogue-missing-adr",
            trait_path: "frob_core::RogueB",
            defining_crate: "frob-core",
            allowed_implementors: &["frob-verbs"],
            authority: InversionAuthority::Adr("docs/adr/ADR-NONEXISTENT.md"),
        },
        InversionRow {
            id: "valid-adr",
            trait_path: "frob_core::ValidAdr",
            defining_crate: "frob-core",
            allowed_implementors: &["frob-verbs"],
            authority: InversionAuthority::Adr("docs/adr/ADR-SYNTH-01.md"),
        },
    ];
    let adr_root = make_scratch_dir("l10-adr");
    write_workspace(&adr_root, &["crates/*"], &[]);
    write_crate(&adr_root, "crates/frob-core", "frob-core", "0", 1, "", "");
    write_file(
        &adr_root.join("docs/adr/ADR-SYNTH-01.md"),
        "# synthetic ADR fixture\n",
    );
    let out_adr = check_layers_with_tables(
        &adr_root,
        &[WorkspaceRoot::core(&adr_root)],
        CRATE_MAP,
        TOMBSTONES,
        ROWS_MISSING_ADR,
    )
    .expect("adr check");
    assert_eq!(
        rules_of(&out_adr.findings),
        vec![RULE_INVERSION_WITHOUT_ADR, RULE_INVERSION_WITHOUT_ADR]
    );
    let subjects: Vec<&str> = out_adr.findings.iter().map(|f| f.subject.as_str()).collect();
    assert_eq!(subjects, vec!["rogue-missing-adr", "rogue-none"]);
}

#[test]
fn l11_rules() {
    let root = make_scratch_dir("l11-two-planes");
    write_workspace(&root, &["crates/*"], &[]);
    write_crate(&root, "crates/frob-dyn", "frob-dyn", "3", 3, "", "");
    write_crate(&root, "crates/frob-design", "frob-design", "2", 2, "", "");
    write_crate(&root, "crates/frob-opt", "frob-opt", "4", 7, "", "");
    write_crate(&root, "crates/frob-lie", "frob-lie", "1", 1, "", "");

    // Layer-3 crate declaring plane = "decision" -> L-11.controller-in-compute-plane.
    write_file(
        &root.join("crates/frob-dyn/contract.toml"),
        "\
[crate]
name = \"frob-dyn\"
layer = 3
rank = 3
plane = \"decision\"
determinism = \"D0\"
evidence = \"Tested\"
gates = []
refusals = []
",
    );

    // Layer-2 crate carrying a [[controller]] record -> L-11.controller-in-compute-plane.
    write_file(
        &root.join("crates/frob-design/contract.toml"),
        "\
[crate]
name = \"frob-design\"
layer = 2
rank = 2
plane = \"compute\"
determinism = \"D0\"
evidence = \"Tested\"
gates = []
refusals = []

[[controller]]
id = \"bad.scheduler\"
kind = \"scheduler\"
",
    );

    // Layer-4 crate with plane = "decision" and [[controller]] -> allowed (no finding).
    write_file(
        &root.join("crates/frob-opt/contract.toml"),
        "\
[crate]
name = \"frob-opt\"
layer = 4
rank = 7
plane = \"decision\"
determinism = \"D0\"
evidence = \"Tested\"
gates = []
refusals = []

[[controller]]
id = \"opt.race.halving\"
kind = \"racing-rule\"
",
    );

    // Layer-1 crate with plane = "compute" and no [[controller]] -> allowed (no finding).
    write_file(
        &root.join("crates/frob-lie/contract.toml"),
        "\
[crate]
name = \"frob-lie\"
layer = 1
rank = 1
plane = \"compute\"
determinism = \"D0\"
evidence = \"Tested\"
gates = []
refusals = []
",
    );

    let outcome = check_layers(&root, &[WorkspaceRoot::core(&root)]).expect("l11 check");
    assert_eq!(
        rules_of(&outcome.findings),
        vec![
            RULE_L11_CONTROLLER_IN_COMPUTE_PLANE,
            RULE_L11_CONTROLLER_IN_COMPUTE_PLANE
        ]
    );
    let paths: Vec<&str> = outcome.findings.iter().map(|f| f.path.as_str()).collect();
    assert_eq!(
        paths,
        vec![
            "crates/frob-design/contract.toml",
            "crates/frob-dyn/contract.toml"
        ]
    );
    assert!(outcome.findings.iter().all(|f| f.line.is_some()));
}

#[test]
fn exit_codes() {
    // 1. Clean tree fixture -> exit 0.
    let clean = fixture_path("clean_ws");
    let mut out = Vec::new();
    let mut err = Vec::new();
    let code = cli::run(
        TEST_COMMANDS,
        &[
            "check-layers".to_owned(),
            "--root".to_owned(),
            clean.to_string_lossy().into_owned(),
        ],
        &mut out,
        &mut err,
    );
    assert_eq!(code, ExitCode::from(EXIT_OK));
    assert!(err.is_empty());

    // 2. Fixture with a finding -> exit 2.
    let bad = make_scratch_dir("exit-findings");
    write_workspace(&bad, &["crates/*"], &[]);
    write_crate(&bad, "crates/frob-extra", "frob-extra", "0", 1, "", "");
    let mut out2 = Vec::new();
    let mut err2 = Vec::new();
    let code2 = cli::run(
        TEST_COMMANDS,
        &[
            "check-layers".to_owned(),
            "--root".to_owned(),
            bad.to_string_lossy().into_owned(),
        ],
        &mut out2,
        &mut err2,
    );
    assert_eq!(code2, ExitCode::from(EXIT_FINDINGS));

    // 3. No argument -> exit 3, subcommands printed.
    let mut out3 = Vec::new();
    let mut err3 = Vec::new();
    let code3 = cli::run(TEST_COMMANDS, &[], &mut out3, &mut err3);
    assert_eq!(code3, ExitCode::from(EXIT_USAGE));
    assert!(String::from_utf8(err3).expect("utf8").contains("check-layers"));

    // 4. Unknown subcommand -> exit 3, subcommands printed.
    let mut out4 = Vec::new();
    let mut err4 = Vec::new();
    let code4 = cli::run(TEST_COMMANDS, &["check-bogus".to_owned()], &mut out4, &mut err4);
    assert_eq!(code4, ExitCode::from(EXIT_USAGE));
    assert!(String::from_utf8(err4).expect("utf8").contains("check-layers"));

    // 5. Missing --root directory -> exit 3.
    let mut out5 = Vec::new();
    let mut err5 = Vec::new();
    let missing = make_scratch_dir("missing-parent").join("does-not-exist");
    let code5 = cli::run(
        TEST_COMMANDS,
        &[
            "check-layers".to_owned(),
            "--root".to_owned(),
            missing.to_string_lossy().into_owned(),
        ],
        &mut out5,
        &mut err5,
    );
    assert_eq!(code5, ExitCode::from(EXIT_USAGE));

    // 6. Unknown extra flag to check-layers -> exit 3.
    let mut out6 = Vec::new();
    let mut err6 = Vec::new();
    let code6 = cli::run(
        TEST_COMMANDS,
        &[
            "check-layers".to_owned(),
            "--root".to_owned(),
            clean.to_string_lossy().into_owned(),
            "--unknown-flag".to_owned(),
        ],
        &mut out6,
        &mut err6,
    );
    assert_eq!(code6, ExitCode::from(EXIT_USAGE));

    // 7. Offline/network cargo error maps to exit 3 with exact repair hint.
    assert!(looks_like_offline_or_network_error(
        "error: failed to get `foo` as a dependency: can't update in --offline mode"
    ));
    let net_err = MetadataLoadError::OfflineNeedsFetch {
        detail: "offline mode".to_owned(),
    };
    assert!(net_err.is_usage_error());
    assert_eq!(net_err.repair(), Some(OFFLINE_FETCH_REPAIR));
    assert!(net_err.to_string().contains(OFFLINE_FETCH_REPAIR));
}

#[test]
fn report_is_sorted_and_stable() {
    let root = make_scratch_dir("report-stable");
    write_workspace(&root, &["crates/*"], &[]);
    write_crate(
        &root,
        "crates/frob-arena",
        "frob-arena",
        "4",
        6,
        "[dependencies]\nfrob-opt = { path = \"../frob-opt\" }\n",
        "",
    );
    write_crate(&root, "crates/frob-opt", "frob-opt", "4", 7, "", "");
    write_crate(
        &root,
        "crates/frob-lie",
        "frob-lie",
        "1",
        1,
        "[dependencies]\nfrob-design = { path = \"../frob-design\" }\n",
        "",
    );
    write_crate(&root, "crates/frob-design", "frob-design", "2", 2, "", "");
    write_crate(&root, "crates/frob-extra", "frob-extra", "2", 6, "", "");

    let run1 = check_layers(&root, &[WorkspaceRoot::core(&root)]).expect("run 1");
    let run2 = check_layers(&root, &[WorkspaceRoot::core(&root)]).expect("run 2");
    let rep1 = CheckReport::new("check-layers", ".", run1.findings, run1.facts, 0);
    let rep2 = CheckReport::new("check-layers", ".", run2.findings, run2.facts, 0);

    let bytes1 = rep1.to_canonical_json().expect("canon 1");
    let bytes2 = rep2.to_canonical_json().expect("canon 2");
    assert_eq!(bytes1, bytes2, "two runs must yield byte-identical canonical JSON");
    assert!(!bytes1.ends_with('\n'), "canonical JSON has no trailing newline");

    // Findings must be sorted by (rule, subject, path) regardless of construction order.
    let mut rev_findings = rep1.findings.clone();
    rev_findings.reverse();
    let rep_rev = CheckReport::new("check-layers", ".", rev_findings, rep1.facts.clone(), 0);
    assert_eq!(
        rep_rev.to_canonical_json().expect("canon rev"),
        bytes1
    );
    assert_eq!(
        rules_of(&rep1.findings),
        vec![RULE_UNLISTED_CRATE, RULE_L1_RANK, RULE_L1_UPWARD]
    );
}

#[test]
fn report_matches_schema() {
    // 1. Verify hand-written schemas/xtask.report.schema.json admits cleanly and declares expected keys.
    let schema_path = repo_root().join("schemas/xtask.report.schema.json");
    let schema_bytes = fs::read(&schema_path).expect("read schemas/xtask.report.schema.json");
    let schema_admitted = admit(&schema_bytes, &Limits::default()).expect("admit schema JSON");
    let schema_obj = schema_admitted.value();
    let required: BTreeSet<&str> = schema_obj
        .get("required")
        .and_then(|v| v.as_array())
        .expect("schema required")
        .iter()
        .map(|s| s.as_str().expect("string"))
        .collect();
    assert_eq!(
        required,
        BTreeSet::from(["schema", "check", "ok", "root", "findings", "facts", "elapsed_ms"])
    );
    assert_eq!(
        schema_obj
            .get("properties")
            .and_then(|p| p.get("schema"))
            .and_then(|s| s.get("const"))
            .and_then(|c| c.as_str()),
        Some(REPORT_SCHEMA)
    );
    assert_eq!(
        schema_obj
            .get("additionalProperties")
            .and_then(|v| v.as_bool()),
        Some(false)
    );

    // 2. Clean report via CLI --json validates against frob.xtask.report/1.
    let clean = fixture_path("clean_ws");
    let mut out_clean = Vec::new();
    let mut err_clean = Vec::new();
    let code = cli::run(
        TEST_COMMANDS,
        &[
            "check-layers".to_owned(),
            "--root".to_owned(),
            clean.to_string_lossy().into_owned(),
            "--json".to_owned(),
        ],
        &mut out_clean,
        &mut err_clean,
    );
    assert_eq!(code, ExitCode::from(EXIT_OK));
    validate_report_json(&out_clean).expect("clean JSON report must match schema");

    // 3. Report with findings (both line=Some and line=None) validates against frob.xtask.report/1.
    let mut facts = BTreeMap::new();
    facts.insert("crates_checked".to_owned(), FactValue::int_usize(2));
    let findings = vec![
        Finding {
            rule: RULE_L1_UPWARD,
            subject: "frob-lie -> frob-design".to_owned(),
            path: "crates/frob-lie/Cargo.toml".to_owned(),
            line: None,
            message: "upward edge".to_owned(),
            repair: "remove upward edge".to_owned(),
        },
        Finding {
            rule: RULE_L11_CONTROLLER_IN_COMPUTE_PLANE,
            subject: "frob-dyn".to_owned(),
            path: "crates/frob-dyn/contract.toml".to_owned(),
            line: Some(5),
            message: "decision plane in layer 3".to_owned(),
            repair: "set plane = \"compute\"".to_owned(),
        },
    ];
    let rep = CheckReport::new("check-layers", ".", findings, facts, 12);
    let json = rep.to_canonical_json().expect("encode report");
    validate_report_json(json.as_bytes()).expect("failing JSON report must match schema");
}

#[test]
fn random_graph_property() {
    // INV-WS-01: over 10^3 seeded random crate graphs, check-layers reports a finding
    // iff some edge violates the strict (layer, rank) order, with exact rule classification.
    let core_rows: Vec<&CrateRow> = CRATE_MAP
        .iter()
        .filter(|r| r.is_counted() && matches!(r.layer, Layer::Core(_)))
        .collect();
    let n_crates = core_rows.len();
    assert_eq!(n_crates, 39);

    let mut state: u64 = 0x34F4_0100_2026_1124;
    let mut next_u64 = || -> u64 {
        state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    };

    let kinds = [DepKind::Normal, DepKind::Dev, DepKind::Build];

    // Boundary cases + 1,000 seeded random crate graphs (1,004 cases total).
    for case_idx in 0..1004usize {
        let mut edges = Vec::new();
        let mut seen_pairs: BTreeSet<(usize, usize)> = BTreeSet::new();

        match case_idx {
            0 => {
                // Boundary: empty graph -> 0 violations.
            }
            1 => {
                // Boundary: every valid downward edge from a random subset -> 0 violations.
                for i in 0..n_crates {
                    for j in 0..i {
                        if core_rows[j].is_below(core_rows[i]) && (i + j) % 7 == 0 {
                            seen_pairs.insert((i, j));
                        }
                    }
                }
            }
            2 => {
                // Boundary: single intra-layer against-rank edge (frob-arena -> frob-opt).
                let a = core_rows.iter().position(|r| r.name == "frob-arena").unwrap();
                let b = core_rows.iter().position(|r| r.name == "frob-opt").unwrap();
                seen_pairs.insert((a, b));
            }
            3 => {
                // Boundary: single self-edge (same layer, equal rank -> L-1.rank).
                seen_pairs.insert((0, 0));
            }
            _ => {
                // Seeded random graph: 0..=24 edges. Half biased to valid downward DAGs,
                // half unconstrained (so both clean and violating graphs are well-sampled).
                let valid_only = case_idx % 2 == 0;
                let edge_count = (next_u64() as usize) % 25;
                for _ in 0..edge_count {
                    let a = (next_u64() as usize) % n_crates;
                    let b = (next_u64() as usize) % n_crates;
                    if valid_only && !core_rows[b].is_below(core_rows[a]) {
                        continue;
                    }
                    seen_pairs.insert((a, b));
                }
            }
        }

        let mut expected_upward = 0usize;
        let mut expected_rank = 0usize;
        for (a_idx, b_idx) in seen_pairs {
            let a = core_rows[a_idx];
            let b = core_rows[b_idx];
            let kind = kinds[(next_u64() as usize) % kinds.len()];
            edges.push(GraphEdge {
                from: a.name.to_owned(),
                from_rel_dir: a.dir.to_owned(),
                from_rel_manifest: format!("{}/Cargo.toml", a.dir),
                to: b.name.to_owned(),
                to_rel_path: Some(b.dir.to_owned()),
                kind,
            });
            if let (Layer::Core(la), Layer::Core(lb)) = (a.layer, b.layer) {
                if lb > la {
                    expected_upward += 1;
                } else if lb == la && b.rank >= a.rank {
                    expected_rank += 1;
                }
            }
        }

        let findings = check_edges(&edges, CRATE_MAP);
        let obs_upward = findings.iter().filter(|f| f.rule == RULE_L1_UPWARD).count();
        let obs_rank = findings.iter().filter(|f| f.rule == RULE_L1_RANK).count();
        assert_eq!(
            (obs_upward, obs_rank, findings.len()),
            (expected_upward, expected_rank, expected_upward + expected_rank),
            "INV-WS-01 mismatch on seeded case {case_idx}"
        );
    }

    // Also verify a synthetic custom CrateRow table works identically.
    let synth_table = [
        CrateRow {
            name: "s-a",
            dir: "crates/s-a",
            layer: Layer::Core(0),
            rank: 1,
            tiers: &[Tier::Solid],
            chapters: &["§34"],
            cap_kloc: 1,
            reason: Reason::Original,
            adr: None,
            conditional: None,
        },
        CrateRow {
            name: "s-b",
            dir: "crates/s-b",
            layer: Layer::Core(0),
            rank: 2,
            tiers: &[Tier::Solid],
            chapters: &["§34"],
            cap_kloc: 1,
            reason: Reason::Original,
            adr: None,
            conditional: None,
        },
    ];
    let bad_rank = vec![GraphEdge {
        from: "s-a".to_owned(),
        from_rel_dir: "crates/s-a".to_owned(),
        from_rel_manifest: "crates/s-a/Cargo.toml".to_owned(),
        to: "s-b".to_owned(),
        to_rel_path: Some("crates/s-b".to_owned()),
        kind: DepKind::Normal,
    }];
    assert_eq!(rules_of(&check_edges(&bad_rank, &synth_table)), vec![RULE_L1_RANK]);
}

#[test]
fn clean_tree_passes_check_layers() {
    let root = repo_root();
    let mut out = Vec::new();
    let mut err = Vec::new();
    let args = vec![
        "check-layers".to_owned(),
        "--root".to_owned(),
        root.to_string_lossy().into_owned(),
        "--json".to_owned(),
    ];
    let code = cli::run(TEST_COMMANDS, &args, &mut out, &mut err);
    assert!(
        err.is_empty(),
        "unexpected stderr on clean tree: {}",
        String::from_utf8_lossy(&err)
    );
    let rendered = String::from_utf8(out.clone()).expect("UTF-8 JSON");
    assert_eq!(
        code,
        ExitCode::from(EXIT_OK),
        "clean tree must pass check-layers with exit 0; report: {rendered}"
    );
    validate_report_json(&out).expect("clean tree report must validate against frob.xtask.report/1");
}
