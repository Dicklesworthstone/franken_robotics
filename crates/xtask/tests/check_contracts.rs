//! Integration and unit tests for `xtask check-contracts` (bead N34-08, GATE-WS-01 plant 6, KA-784).
//!
//! Synthetic scratch workspaces are created under
//! `${BIG_SCRATCH:-${TMPDIR:-/tmp}}/frob-BronzePelican/` and never deleted (COMMON.md 9b).
//! Every test runs through the real `cargo metadata --no-deps --locked --offline` reader,
//! `frob_core::registry::toml`, `crate_map::CRATE_MAP`, `check_unsafe::DOORWAY_SITES`,
//! and the `frob.xtask.report/1` canonical writer.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use frob_core::json::{Limits, admit};
use xtask::check_contracts::{
    self, OWNED_RULES, REQUIRED_CONTRACT_SECTIONS, RULE_CAP_OVER_115,
    RULE_CONTRACT_DETERMINISM_EVIDENCE, RULE_CONTRACT_FEATURES_MISMATCH, RULE_CONTRACT_FLOAT_OK,
    RULE_CONTRACT_LAYER_MISMATCH, RULE_CONTRACT_MISSING, RULE_CONTRACT_MISSING_FOR_TARGET,
    RULE_CONTRACT_NO_CONFORMANCE_TEST, RULE_CONTRACT_PLANE, RULE_CONTRACT_SAFE_MODE,
    RULE_CONTRACT_SCHEMA, RULE_CONTRACT_SECTION_MISSING, RULE_CONTRACT_UNKNOWN_REFERENCE,
    SCHEMA_API_KEYS, SCHEMA_BUDGETS_REQUIRED_KEYS, SCHEMA_CRATE_KEYS, SCHEMA_FEATURE_KEYS,
    SCHEMA_SAFE_MODE_KEYS, SCHEMA_TOP_KEYS, check_contracts, check_contracts_with_tables,
};
use xtask::check_unsafe::DOORWAY_SITES;
use xtask::cli::{self, CheckError, CheckFn, EXIT_FINDINGS, EXIT_OK};
use xtask::crate_map::{CRATE_MAP, CrateRow, Layer, Reason, Tier, total_cap_kloc};
use xtask::metadata::WorkspaceRoot;
use xtask::report::{CheckReport, validate_report_json};

static COUNTER: AtomicU64 = AtomicU64::new(0);

const TEST_COMMANDS: &[(&str, CheckFn)] = &[("check-contracts", check_contracts::run_command)];

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
        .join(format!("xtask-check-contracts-{label}-{ts}-{pid}-{n}"));
    fs::create_dir_all(&dir).expect("create scratch dir");
    dir
}

fn write_file(path: &Path, content: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("create parent dirs");
    }
    fs::write(path, content).expect("write synthetic fixture file");
}

fn full_contract_md(name: &str) -> String {
    let mut s = format!("# `{name}` Contract\n\n");
    for &sec in REQUIRED_CONTRACT_SECTIONS {
        s.push_str(&format!("## {sec}\n\nConcrete specification for {sec}.\n\n"));
    }
    s
}

fn seed_registries(root: &Path) {
    write_file(
        &root.join("registries/refusals.toml"),
        "schema = \"frob.refusals/1\"\n\
         \n\
         [[refusal]]\n\
         code = \"FROB-REF-702\"\n\
         meaning = \"Budget exhausted\"\n\
         defined_in = \"§35\"\n\
         exit = 4\n\
         retry = \"with-budget\"\n\
         repairs = [\"Increase budget\"]\n\
         raised_by = [\"N34-08\"]\n\
         \n\
         [[refusal]]\n\
         code = \"FROB-REF-802\"\n\
         meaning = \"Invalid arguments\"\n\
         defined_in = \"§32\"\n\
         exit = 3\n\
         retry = \"after-change\"\n\
         repairs = [\"Fix arguments\"]\n\
         raised_by = [\"BD-005\"]\n",
    );
    write_file(
        &root.join("registries/gates.toml"),
        "schema = \"frob.gates/1\"\n\
         \n\
         [[gate]]\n\
         id = \"GATE-TRAC-01\"\n\
         claim = \"known-answer\"\n\
         claim_id = \"GATE-TRAC-01\"\n\
         section = \"18.1\"\n\
         threshold = \"exact\"\n\
         milestone = \"M0\"\n\
         tier = [\"G0:host\"]\n\
         controls = [\"NC-0001\"]\n\
         \n\
         [[gate]]\n\
         id = \"GATE-KILL-FILM\"\n\
         claim = \"known-answer\"\n\
         claim_id = \"GATE-KILL-FILM\"\n\
         section = \"18.2\"\n\
         threshold = \"kill\"\n\
         milestone = \"M1\"\n\
         tier = [\"G0:host\"]\n\
         controls = [\"NC-0002\"]\n",
    );
}

/// Seed a clean synthetic workspace containing `frob-contact` (layer 3, rank 2 in `CRATE_MAP`).
fn create_clean_contact_workspace(label: &str) -> PathBuf {
    let root = make_scratch_dir(label);
    write_file(
        &root.join("Cargo.toml"),
        "[workspace]\nresolver = \"3\"\nmembers = [\"crates/*\"]\n",
    );
    seed_registries(&root);
    write_file(
        &root.join("crates/frob-contact/Cargo.toml"),
        "[package]\n\
         name = \"frob-contact\"\n\
         version = \"0.0.1\"\n\
         edition = \"2024\"\n\
         \n\
         [package.metadata.frob]\n\
         layer = 3\n\
         rank = 2\n",
    );
    write_file(
        &root.join("crates/frob-contact/src/lib.rs"),
        "pub fn contact_step() -> u32 { 1 }\n",
    );
    write_file(
        &root.join("crates/frob-contact/CONTRACT.md"),
        &full_contract_md("frob-contact"),
    );
    write_file(
        &root.join("crates/frob-contact/contract.toml"),
        "[crate]\n\
         name = \"frob-contact\"\n\
         layer = 3\n\
         rank = 2\n\
         plane = \"compute\"\n\
         determinism = \"D2\"\n\
         evidence = \"Tested\"\n\
         gates = [\"GATE-TRAC-01\"]\n\
         refusals = [\"FROB-REF-702\"]\n\
         \n\
         [budgets]\n\
         wall_ms = 50\n\
         memory_mib = 256\n\
         \n\
         [safe_mode]\n\
         on_budget = \"refuse:FROB-REF-702\"\n\
         on_calibration = \"reference-path\"\n",
    );
    write_file(
        &root.join("crates/frob-contact/tests/conformance.rs"),
        "#[test]\nfn ok() { assert_eq!(1, 1); }\n",
    );
    root
}

fn run_check(root: &Path) -> Vec<xtask::crate_map::Finding> {
    let outcome = check_contracts(root, &[WorkspaceRoot::core(root)])
        .expect("check_contracts should succeed without tool fault");
    outcome.findings
}

fn run_cli_json(root: &Path) -> (ExitCode, String) {
    let args = vec![
        "check-contracts".to_owned(),
        "--root".to_owned(),
        root.to_string_lossy().into_owned(),
        "--json".to_owned(),
    ];
    let mut out = Vec::new();
    let mut err = Vec::new();
    let code = cli::run(TEST_COMMANDS, &args, &mut out, &mut err);
    assert!(
        err.is_empty(),
        "unexpected stderr: {}",
        String::from_utf8_lossy(&err)
    );
    let json_text = String::from_utf8(out).expect("stdout must be UTF-8");
    validate_report_json(json_text.as_bytes()).expect("valid frob.xtask.report/1");
    (code, json_text)
}

// ============================================================================
// Rule 2: CONTRACT.md ten sections, empty section, extra sections, Plant 6
// ============================================================================

#[test]
fn ten_sections_required() {
    for (idx, omitted) in REQUIRED_CONTRACT_SECTIONS.iter().enumerate() {
        let root = create_clean_contact_workspace(&format!("omit-sec-{idx}"));
        let mut md = String::from("# `frob-contact` Contract\n\n");
        for &sec in REQUIRED_CONTRACT_SECTIONS {
            if sec != *omitted {
                md.push_str(&format!("## {sec}\n\nDetails for {sec}.\n\n"));
            }
        }
        write_file(&root.join("crates/frob-contact/CONTRACT.md"), &md);

        // Plant 6 of GATE-WS-01: deleting `Validity domain` exits 2 with `contract.section-missing`.
        if *omitted == "Validity domain" {
            let (code, json_text) = run_cli_json(&root);
            assert_eq!(code, ExitCode::from(EXIT_FINDINGS));
            let adm = admit(json_text.as_bytes(), &Limits::default()).expect("admit");
            let findings = adm
                .value()
                .get("findings")
                .and_then(|v| v.as_array())
                .expect("findings");
            assert_eq!(findings.len(), 1);
            assert_eq!(
                findings[0].get("rule").and_then(|v| v.as_str()),
                Some(RULE_CONTRACT_SECTION_MISSING)
            );
            continue;
        }

        let findings = run_check(&root);
        assert_eq!(
            findings.len(),
            1,
            "omitting `{omitted}` should yield 1 finding, got: {findings:?}"
        );
        assert_eq!(findings[0].rule, RULE_CONTRACT_SECTION_MISSING);
        assert!(findings[0].message.contains(omitted));
    }
}

#[test]
fn empty_section_fails() {
    let root = create_clean_contact_workspace("empty-section");
    let mut md = String::from("# `frob-contact` Contract\n\n");
    for &sec in REQUIRED_CONTRACT_SECTIONS {
        if sec == "Validity domain" {
            md.push_str("## Validity domain\n\n<!-- only a comment -->\n\n");
        } else {
            md.push_str(&format!("## {sec}\n\nDetails for {sec}.\n\n"));
        }
    }
    write_file(&root.join("crates/frob-contact/CONTRACT.md"), &md);

    let findings = run_check(&root);
    assert_eq!(findings.len(), 1, "expected 1 finding, got: {findings:?}");
    assert_eq!(findings[0].rule, RULE_CONTRACT_SECTION_MISSING);
    assert!(findings[0].message.contains("Validity domain"));
}

#[test]
fn extra_sections_allowed() {
    let root = create_clean_contact_workspace("extra-sections");
    // Reversed section order + extra level-2 section + level-3 subheading must all pass.
    let mut md = String::from("# `frob-contact` Contract\n\n## Background Notes\n\nExtra context.\n\n");
    for &sec in REQUIRED_CONTRACT_SECTIONS.iter().rev() {
        md.push_str(&format!(
            "## {sec}\n\n### Subsection\n\nDetails for {sec}.\n\n"
        ));
    }
    write_file(&root.join("crates/frob-contact/CONTRACT.md"), &md);

    let findings = run_check(&root);
    assert!(
        findings.is_empty(),
        "reordered and extra sections must pass: {findings:?}"
    );
}

// ============================================================================
// Rule 3: tests/conformance.rs required
// ============================================================================

#[test]
fn conformance_test_required() {
    let root = make_scratch_dir("no-conformance-test");
    write_file(
        &root.join("Cargo.toml"),
        "[workspace]\nresolver = \"3\"\nmembers = [\"crates/*\"]\n",
    );
    seed_registries(&root);
    write_file(
        &root.join("crates/frob-contact/Cargo.toml"),
        "[package]\nname = \"frob-contact\"\nversion = \"0.0.1\"\nedition = \"2024\"\n",
    );
    write_file(
        &root.join("crates/frob-contact/src/lib.rs"),
        "pub fn f() {}\n",
    );
    write_file(
        &root.join("crates/frob-contact/CONTRACT.md"),
        &full_contract_md("frob-contact"),
    );
    write_file(
        &root.join("crates/frob-contact/contract.toml"),
        "[crate]\nname = \"frob-contact\"\nlayer = 3\nrank = 2\nplane = \"compute\"\n\
         determinism = \"D2\"\nevidence = \"Tested\"\ngates = []\nrefusals = []\n\
         \n[budgets]\nwall_ms = 50\nmemory_mib = 256\n\
         \n[safe_mode]\non_budget = \"reference-path\"\non_calibration = \"reference-path\"\n",
    );
    // Deliberately omit tests/conformance.rs.
    let findings = run_check(&root);
    assert_eq!(findings.len(), 1, "expected 1 finding, got: {findings:?}");
    assert_eq!(findings[0].rule, RULE_CONTRACT_NO_CONFORMANCE_TEST);
    assert!(findings[0].repair.contains("assets/templates/"));
}

// ============================================================================
// Rule 4: contract.toml schema validation + JSON Schema key agreement
// ============================================================================

#[test]
fn contract_toml_schema() {
    // 1. Unknown key fails contract.schema.
    {
        let root = create_clean_contact_workspace("schema-unknown-key");
        write_file(
            &root.join("crates/frob-contact/contract.toml"),
            "[crate]\nname = \"frob-contact\"\nlayer = 3\nrank = 2\nplane = \"compute\"\n\
             determinism = \"D2\"\nevidence = \"Tested\"\ngates = []\nrefusals = []\n\
             unregistered_field = true\n\
             \n[budgets]\nwall_ms = 50\nmemory_mib = 256\n\
             \n[safe_mode]\non_budget = \"reference-path\"\non_calibration = \"reference-path\"\n",
        );
        let findings = run_check(&root);
        assert_eq!(findings.len(), 1, "unknown key should emit 1 finding: {findings:?}");
        assert_eq!(findings[0].rule, RULE_CONTRACT_SCHEMA);
    }

    // 2. Wrong type fails contract.schema.
    {
        let root = create_clean_contact_workspace("schema-wrong-type");
        write_file(
            &root.join("crates/frob-contact/contract.toml"),
            "[crate]\nname = \"frob-contact\"\nlayer = 3\nrank = 2\nplane = \"compute\"\n\
             determinism = \"D2\"\nevidence = \"Tested\"\ngates = []\nrefusals = []\n\
             \n[budgets]\nwall_ms = \"fast\"\nmemory_mib = 256\n\
             \n[safe_mode]\non_budget = \"reference-path\"\non_calibration = \"reference-path\"\n",
        );
        let findings = run_check(&root);
        assert_eq!(findings.len(), 1, "wrong type should emit 1 finding: {findings:?}");
        assert_eq!(findings[0].rule, RULE_CONTRACT_SCHEMA);
    }

    // 3. Missing `[safe_mode]` fails contract.schema.
    {
        let root = create_clean_contact_workspace("schema-missing-safe-mode");
        write_file(
            &root.join("crates/frob-contact/contract.toml"),
            "[crate]\nname = \"frob-contact\"\nlayer = 3\nrank = 2\nplane = \"compute\"\n\
             determinism = \"D2\"\nevidence = \"Tested\"\ngates = []\nrefusals = []\n\
             \n[budgets]\nwall_ms = 50\nmemory_mib = 256\n",
        );
        let findings = run_check(&root);
        assert_eq!(findings.len(), 1, "missing [safe_mode] should emit 1 finding: {findings:?}");
        assert_eq!(findings[0].rule, RULE_CONTRACT_SCHEMA);
    }

    // 4. Cross-check that `schemas/contract.schema.json` declares every key read by check-contracts.
    let schema_bytes =
        fs::read(repo_root().join("schemas/contract.schema.json")).expect("read contract.schema.json");
    let adm = admit(&schema_bytes, &Limits::default()).expect("admit contract.schema.json");
    let props = adm.value().get("properties").expect("top properties");
    for &k in SCHEMA_TOP_KEYS {
        assert!(props.get(k).is_some(), "contract.schema.json missing top key `{k}`");
    }
    let crate_props = props.get("crate").and_then(|v| v.get("properties")).expect("crate props");
    for &k in SCHEMA_CRATE_KEYS {
        assert!(crate_props.get(k).is_some(), "contract.schema.json missing crate.{k}");
    }
    let budget_props = props
        .get("budgets")
        .and_then(|v| v.get("properties"))
        .expect("budgets props");
    for &k in SCHEMA_BUDGETS_REQUIRED_KEYS {
        assert!(budget_props.get(k).is_some(), "contract.schema.json missing budgets.{k}");
    }
    let sm_props = props
        .get("safe_mode")
        .and_then(|v| v.get("properties"))
        .expect("safe_mode props");
    for &k in SCHEMA_SAFE_MODE_KEYS {
        assert!(sm_props.get(k).is_some(), "contract.schema.json missing safe_mode.{k}");
    }
    let feat_props = props
        .get("features")
        .and_then(|v| v.get("additionalProperties"))
        .and_then(|v| v.get("properties"))
        .expect("features props");
    for &k in SCHEMA_FEATURE_KEYS {
        assert!(feat_props.get(k).is_some(), "contract.schema.json missing features.*.{k}");
    }
    let api_props = props.get("api").and_then(|v| v.get("properties")).expect("api props");
    for &k in SCHEMA_API_KEYS {
        assert!(api_props.get(k).is_some(), "contract.schema.json missing api.{k}");
    }
}

// ============================================================================
// Rule 5: layer/rank match CRATE_MAP; manifest mismatch is NOT reported here
// ============================================================================

#[test]
fn layer_rank_must_match_crate_map() {
    // `frob-contact` is layer 3, rank 2 in CRATE_MAP; declaring layer = 4 fails.
    let root_layer = create_clean_contact_workspace("layer-mismatch");
    write_file(
        &root_layer.join("crates/frob-contact/contract.toml"),
        "[crate]\nname = \"frob-contact\"\nlayer = 4\nrank = 2\nplane = \"compute\"\n\
         determinism = \"D2\"\nevidence = \"Tested\"\ngates = []\nrefusals = []\n\
         \n[budgets]\nwall_ms = 50\nmemory_mib = 256\n\
         \n[safe_mode]\non_budget = \"reference-path\"\non_calibration = \"reference-path\"\n",
    );
    let f_layer = run_check(&root_layer);
    assert_eq!(f_layer.len(), 1, "expected 1 finding, got: {f_layer:?}");
    assert_eq!(f_layer[0].rule, RULE_CONTRACT_LAYER_MISMATCH);

    // Declaring wrong rank = 9 also fails.
    let root_rank = create_clean_contact_workspace("rank-mismatch");
    write_file(
        &root_rank.join("crates/frob-contact/contract.toml"),
        "[crate]\nname = \"frob-contact\"\nlayer = 3\nrank = 9\nplane = \"compute\"\n\
         determinism = \"D2\"\nevidence = \"Tested\"\ngates = []\nrefusals = []\n\
         \n[budgets]\nwall_ms = 50\nmemory_mib = 256\n\
         \n[safe_mode]\non_budget = \"reference-path\"\non_calibration = \"reference-path\"\n",
    );
    let f_rank = run_check(&root_rank);
    assert_eq!(f_rank.len(), 1, "expected 1 finding, got: {f_rank:?}");
    assert_eq!(f_rank[0].rule, RULE_CONTRACT_LAYER_MISMATCH);
}

#[test]
fn wrong_manifest_alone_is_not_reported_here() {
    // Manifest `[package.metadata.frob]` is wrong (layer 7, rank 99), while contract.toml
    // matches CRATE_MAP (layer 3, rank 2). check-contracts reports ZERO findings because
    // check-layers owns manifest-vs-map comparison.
    let root = create_clean_contact_workspace("wrong-manifest-only");
    write_file(
        &root.join("crates/frob-contact/Cargo.toml"),
        "[package]\nname = \"frob-contact\"\nversion = \"0.0.1\"\nedition = \"2024\"\n\
         \n[package.metadata.frob]\nlayer = 7\nrank = 99\n",
    );
    let findings = run_check(&root);
    assert!(
        findings.is_empty(),
        "manifest metadata mismatch must not be reported by check-contracts: {findings:?}"
    );
}

// ============================================================================
// Rule 6: manifest features parity + F/M prefix and kill gate
// ============================================================================

#[test]
fn features_must_match_manifest() {
    // Feature present in Cargo.toml (excluding `default`) and absent from contract.toml fails.
    let root_missing = create_clean_contact_workspace("feat-missing-in-contract");
    write_file(
        &root_missing.join("crates/frob-contact/Cargo.toml"),
        "[package]\nname = \"frob-contact\"\nversion = \"0.0.1\"\nedition = \"2024\"\n\
         \n[features]\ndefault = []\nextra-check = []\n",
    );
    let f1 = run_check(&root_missing);
    assert_eq!(f1.len(), 1, "expected 1 finding, got: {f1:?}");
    assert_eq!(f1[0].rule, RULE_CONTRACT_FEATURES_MISMATCH);
    assert!(f1[0].message.contains("extra-check"));

    // Feature present in contract.toml and absent from Cargo.toml also fails.
    let root_extra = create_clean_contact_workspace("feat-extra-in-contract");
    write_file(
        &root_extra.join("crates/frob-contact/contract.toml"),
        "[crate]\nname = \"frob-contact\"\nlayer = 3\nrank = 2\nplane = \"compute\"\n\
         determinism = \"D2\"\nevidence = \"Tested\"\ngates = []\nrefusals = []\n\
         \n[budgets]\nwall_ms = 50\nmemory_mib = 256\n\
         \n[safe_mode]\non_budget = \"reference-path\"\non_calibration = \"reference-path\"\n\
         \n[features]\nghost = { tier = \"S\" }\n",
    );
    let f2 = run_check(&root_extra);
    assert_eq!(f2.len(), 1, "expected 1 finding, got: {f2:?}");
    assert_eq!(f2[0].rule, RULE_CONTRACT_FEATURES_MISMATCH);
    assert!(f2[0].message.contains("ghost"));
}

#[test]
fn tier_feature_needs_prefix_and_kill_gate() {
    // 1. Valid `frontier-film` with registered kill gate passes.
    let root_ok = create_clean_contact_workspace("tier-f-ok");
    write_file(
        &root_ok.join("crates/frob-contact/Cargo.toml"),
        "[package]\nname = \"frob-contact\"\nversion = \"0.0.1\"\nedition = \"2024\"\n\
         \n[features]\ndefault = []\nfrontier-film = []\n",
    );
    write_file(
        &root_ok.join("crates/frob-contact/contract.toml"),
        "[crate]\nname = \"frob-contact\"\nlayer = 3\nrank = 2\nplane = \"compute\"\n\
         determinism = \"D2\"\nevidence = \"Tested\"\ngates = []\nrefusals = []\n\
         \n[budgets]\nwall_ms = 50\nmemory_mib = 256\n\
         \n[safe_mode]\non_budget = \"reference-path\"\non_calibration = \"reference-path\"\n\
         \n[features]\n\"frontier-film\" = { tier = \"F\", kill = \"GATE-KILL-FILM\" }\n",
    );
    assert!(run_check(&root_ok).is_empty());

    // 2. `frontier-film` renamed `film` (missing `frontier-` prefix) fails.
    let root_noprefix = create_clean_contact_workspace("tier-f-no-prefix");
    write_file(
        &root_noprefix.join("crates/frob-contact/Cargo.toml"),
        "[package]\nname = \"frob-contact\"\nversion = \"0.0.1\"\nedition = \"2024\"\n\
         \n[features]\nfilm = []\n",
    );
    write_file(
        &root_noprefix.join("crates/frob-contact/contract.toml"),
        "[crate]\nname = \"frob-contact\"\nlayer = 3\nrank = 2\nplane = \"compute\"\n\
         determinism = \"D2\"\nevidence = \"Tested\"\ngates = []\nrefusals = []\n\
         \n[budgets]\nwall_ms = 50\nmemory_mib = 256\n\
         \n[safe_mode]\non_budget = \"reference-path\"\non_calibration = \"reference-path\"\n\
         \n[features]\nfilm = { tier = \"F\", kill = \"GATE-KILL-FILM\" }\n",
    );
    let f_np = run_check(&root_noprefix);
    assert_eq!(f_np.len(), 1, "expected 1 finding, got: {f_np:?}");
    assert_eq!(f_np[0].rule, RULE_CONTRACT_FEATURES_MISMATCH);

    // 3. Tier M without `moonshot-` prefix AND without `kill` gate fails both checks.
    let root_m = create_clean_contact_workspace("tier-m-missing");
    write_file(
        &root_m.join("crates/frob-contact/Cargo.toml"),
        "[package]\nname = \"frob-contact\"\nversion = \"0.0.1\"\nedition = \"2024\"\n\
         \n[features]\nwarp = []\n",
    );
    write_file(
        &root_m.join("crates/frob-contact/contract.toml"),
        "[crate]\nname = \"frob-contact\"\nlayer = 3\nrank = 2\nplane = \"compute\"\n\
         determinism = \"D2\"\nevidence = \"Tested\"\ngates = []\nrefusals = []\n\
         \n[budgets]\nwall_ms = 50\nmemory_mib = 256\n\
         \n[safe_mode]\non_budget = \"reference-path\"\non_calibration = \"reference-path\"\n\
         \n[features]\nwarp = { tier = \"M\" }\n",
    );
    let f_m = run_check(&root_m);
    assert_eq!(f_m.len(), 2, "expected 2 findings (prefix + kill), got: {f_m:?}");
    assert!(f_m.iter().all(|f| f.rule == RULE_CONTRACT_FEATURES_MISMATCH));

    // 4. Tier F with unregistered kill gate fails contract.unknown-reference.
    let root_badkill = create_clean_contact_workspace("tier-f-bad-kill");
    write_file(
        &root_badkill.join("crates/frob-contact/Cargo.toml"),
        "[package]\nname = \"frob-contact\"\nversion = \"0.0.1\"\nedition = \"2024\"\n\
         \n[features]\nfrontier-film = []\n",
    );
    write_file(
        &root_badkill.join("crates/frob-contact/contract.toml"),
        "[crate]\nname = \"frob-contact\"\nlayer = 3\nrank = 2\nplane = \"compute\"\n\
         determinism = \"D2\"\nevidence = \"Tested\"\ngates = []\nrefusals = []\n\
         \n[budgets]\nwall_ms = 50\nmemory_mib = 256\n\
         \n[safe_mode]\non_budget = \"reference-path\"\non_calibration = \"reference-path\"\n\
         \n[features]\n\"frontier-film\" = { tier = \"F\", kill = \"GATE-NONEXISTENT-99\" }\n",
    );
    let f_bk = run_check(&root_badkill);
    assert_eq!(f_bk.len(), 1, "expected 1 finding, got: {f_bk:?}");
    assert_eq!(f_bk[0].rule, RULE_CONTRACT_UNKNOWN_REFERENCE);
}

// ============================================================================
// Rule 7 & Rule 8: unknown refusal/gate and safe_mode values
// ============================================================================

#[test]
fn unknown_refusal_fails() {
    let root = create_clean_contact_workspace("unknown-refusal");
    write_file(
        &root.join("crates/frob-contact/contract.toml"),
        "[crate]\nname = \"frob-contact\"\nlayer = 3\nrank = 2\nplane = \"compute\"\n\
         determinism = \"D2\"\nevidence = \"Tested\"\ngates = []\nrefusals = [\"FROB-REF-999\"]\n\
         \n[budgets]\nwall_ms = 50\nmemory_mib = 256\n\
         \n[safe_mode]\non_budget = \"reference-path\"\non_calibration = \"reference-path\"\n",
    );
    let findings = run_check(&root);
    assert_eq!(findings.len(), 1, "expected 1 finding, got: {findings:?}");
    assert_eq!(findings[0].rule, RULE_CONTRACT_UNKNOWN_REFERENCE);
    assert!(findings[0].message.contains("FROB-REF-999"));
}

#[test]
fn unknown_gate_fails() {
    let root = create_clean_contact_workspace("unknown-gate");
    write_file(
        &root.join("crates/frob-contact/contract.toml"),
        "[crate]\nname = \"frob-contact\"\nlayer = 3\nrank = 2\nplane = \"compute\"\n\
         determinism = \"D2\"\nevidence = \"Tested\"\ngates = [\"GATE-BOGUS-42\"]\nrefusals = []\n\
         \n[budgets]\nwall_ms = 50\nmemory_mib = 256\n\
         \n[safe_mode]\non_budget = \"reference-path\"\non_calibration = \"reference-path\"\n",
    );
    let findings = run_check(&root);
    assert_eq!(findings.len(), 1, "expected 1 finding, got: {findings:?}");
    assert_eq!(findings[0].rule, RULE_CONTRACT_UNKNOWN_REFERENCE);
    assert!(findings[0].message.contains("GATE-BOGUS-42"));
}

#[test]
fn safe_mode_values() {
    // 1. `reference-path` and `refuse:FROB-REF-702` (registered) pass.
    let root_ok = create_clean_contact_workspace("safe-mode-ok");
    assert!(run_check(&root_ok).is_empty());

    // 2. `on_budget = "degrade"` fails contract.safe-mode.
    let root_degrade = create_clean_contact_workspace("safe-mode-degrade");
    write_file(
        &root_degrade.join("crates/frob-contact/contract.toml"),
        "[crate]\nname = \"frob-contact\"\nlayer = 3\nrank = 2\nplane = \"compute\"\n\
         determinism = \"D2\"\nevidence = \"Tested\"\ngates = []\nrefusals = []\n\
         \n[budgets]\nwall_ms = 50\nmemory_mib = 256\n\
         \n[safe_mode]\non_budget = \"degrade\"\non_calibration = \"reference-path\"\n",
    );
    let f_deg = run_check(&root_degrade);
    assert_eq!(f_deg.len(), 1, "expected 1 finding, got: {f_deg:?}");
    assert_eq!(f_deg[0].rule, RULE_CONTRACT_SAFE_MODE);

    // 3. `on_calibration = "refuse:FROB-REF-000"` (unregistered) fails contract.safe-mode.
    let root_unreg = create_clean_contact_workspace("safe-mode-unreg");
    write_file(
        &root_unreg.join("crates/frob-contact/contract.toml"),
        "[crate]\nname = \"frob-contact\"\nlayer = 3\nrank = 2\nplane = \"compute\"\n\
         determinism = \"D2\"\nevidence = \"Tested\"\ngates = []\nrefusals = []\n\
         \n[budgets]\nwall_ms = 50\nmemory_mib = 256\n\
         \n[safe_mode]\non_budget = \"reference-path\"\non_calibration = \"refuse:FROB-REF-000\"\n",
    );
    let f_unreg = run_check(&root_unreg);
    assert_eq!(f_unreg.len(), 1, "expected 1 finding, got: {f_unreg:?}");
    assert_eq!(f_unreg[0].rule, RULE_CONTRACT_SAFE_MODE);
}

// ============================================================================
// Rules 9 & 10: plane, float_ok, D4/Tested, Certified/Proved doorway check
// ============================================================================

#[test]
fn plane_and_float_ok_validation() {
    // Invalid plane word fails contract.plane.
    let root_plane = create_clean_contact_workspace("plane-invalid");
    write_file(
        &root_plane.join("crates/frob-contact/contract.toml"),
        "[crate]\nname = \"frob-contact\"\nlayer = 3\nrank = 2\nplane = \"hybrid\"\n\
         determinism = \"D2\"\nevidence = \"Tested\"\ngates = []\nrefusals = []\n\
         \n[budgets]\nwall_ms = 50\nmemory_mib = 256\n\
         \n[safe_mode]\non_budget = \"reference-path\"\non_calibration = \"reference-path\"\n",
    );
    let f_plane = run_check(&root_plane);
    assert_eq!(f_plane.len(), 1, "expected 1 finding, got: {f_plane:?}");
    assert_eq!(f_plane[0].rule, RULE_CONTRACT_PLANE);

    // Layer-3 crate declaring `plane = "decision"` is L-11 (check-layers), NOT reported here.
    let root_l11 = create_clean_contact_workspace("plane-l11-ownership");
    write_file(
        &root_l11.join("crates/frob-contact/contract.toml"),
        "[crate]\nname = \"frob-contact\"\nlayer = 3\nrank = 2\nplane = \"decision\"\n\
         determinism = \"D2\"\nevidence = \"Tested\"\ngates = []\nrefusals = []\n\
         \n[budgets]\nwall_ms = 50\nmemory_mib = 256\n\
         \n[safe_mode]\non_budget = \"reference-path\"\non_calibration = \"reference-path\"\n",
    );
    assert!(run_check(&root_l11).is_empty());

    // Empty `[api.float_ok]` reason fails contract.float-ok.
    let root_fok = create_clean_contact_workspace("float-ok-empty");
    write_file(
        &root_fok.join("crates/frob-contact/contract.toml"),
        "[crate]\nname = \"frob-contact\"\nlayer = 3\nrank = 2\nplane = \"compute\"\n\
         determinism = \"D2\"\nevidence = \"Tested\"\ngates = []\nrefusals = []\n\
         \n[budgets]\nwall_ms = 50\nmemory_mib = 256\n\
         \n[safe_mode]\non_budget = \"reference-path\"\non_calibration = \"reference-path\"\n\
         \n[api.float_ok]\n\"buffers::DeviceView::as_f32\" = \"   \"\n",
    );
    let f_fok = run_check(&root_fok);
    assert_eq!(f_fok.len(), 1, "expected 1 finding, got: {f_fok:?}");
    assert_eq!(f_fok[0].rule, RULE_CONTRACT_FLOAT_OK);
}

#[test]
fn d4_with_tested_fails() {
    let root = create_clean_contact_workspace("d4-with-tested");
    write_file(
        &root.join("crates/frob-contact/contract.toml"),
        "[crate]\nname = \"frob-contact\"\nlayer = 3\nrank = 2\nplane = \"compute\"\n\
         determinism = \"D4\"\nevidence = \"Tested\"\ngates = []\nrefusals = []\n\
         \n[budgets]\nwall_ms = 50\nmemory_mib = 256\n\
         \n[safe_mode]\non_budget = \"reference-path\"\non_calibration = \"reference-path\"\n",
    );
    let findings = run_check(&root);
    assert_eq!(findings.len(), 1, "expected 1 finding, got: {findings:?}");
    assert_eq!(findings[0].rule, RULE_CONTRACT_DETERMINISM_EVIDENCE);

    // D4 with Estimated passes.
    let root_est = create_clean_contact_workspace("d4-with-estimated");
    write_file(
        &root_est.join("crates/frob-contact/contract.toml"),
        "[crate]\nname = \"frob-contact\"\nlayer = 3\nrank = 2\nplane = \"compute\"\n\
         determinism = \"D4\"\nevidence = \"Estimated\"\ngates = []\nrefusals = []\n\
         \n[budgets]\nwall_ms = 50\nmemory_mib = 256\n\
         \n[safe_mode]\non_budget = \"reference-path\"\non_calibration = \"reference-path\"\n",
    );
    assert!(run_check(&root_est).is_empty());
}

#[test]
fn certified_outside_doorway_crates_fails() {
    // `frob-contact` is NOT a Proof-claim doorway crate in DOORWAY_SITES -> Certified fails.
    let root_bad = create_clean_contact_workspace("certified-outside-doorway");
    write_file(
        &root_bad.join("crates/frob-contact/contract.toml"),
        "[crate]\nname = \"frob-contact\"\nlayer = 3\nrank = 2\nplane = \"compute\"\n\
         determinism = \"D0\"\nevidence = \"Certified\"\ngates = []\nrefusals = []\n\
         \n[budgets]\nwall_ms = 50\nmemory_mib = 256\n\
         \n[safe_mode]\non_budget = \"reference-path\"\non_calibration = \"reference-path\"\n",
    );
    let findings = run_check(&root_bad);
    assert_eq!(findings.len(), 1, "expected 1 finding, got: {findings:?}");
    assert_eq!(findings[0].rule, RULE_CONTRACT_DETERMINISM_EVIDENCE);

    // `frob-lemma` IS in DOORWAY_SITES for Certified/Proved -> passes!
    let root_lemma = make_scratch_dir("certified-in-frob-lemma");
    write_file(
        &root_lemma.join("Cargo.toml"),
        "[workspace]\nresolver = \"3\"\nmembers = [\"crates/*\"]\n",
    );
    seed_registries(&root_lemma);
    write_file(
        &root_lemma.join("crates/frob-lemma/Cargo.toml"),
        "[package]\nname = \"frob-lemma\"\nversion = \"0.0.1\"\nedition = \"2024\"\n",
    );
    write_file(
        &root_lemma.join("crates/frob-lemma/src/lib.rs"),
        "pub fn issue() {}\n",
    );
    write_file(
        &root_lemma.join("crates/frob-lemma/CONTRACT.md"),
        &full_contract_md("frob-lemma"),
    );
    write_file(
        &root_lemma.join("crates/frob-lemma/contract.toml"),
        "[crate]\nname = \"frob-lemma\"\nlayer = 6\nrank = 3\nplane = \"decision\"\n\
         determinism = \"D0\"\nevidence = \"Certified\"\ngates = []\nrefusals = []\n\
         \n[budgets]\nwall_ms = 500\nmemory_mib = 128\n\
         \n[safe_mode]\non_budget = \"reference-path\"\non_calibration = \"reference-path\"\n",
    );
    write_file(
        &root_lemma.join("crates/frob-lemma/tests/conformance.rs"),
        "#[test]\nfn ok() {}\n",
    );
    assert!(
        run_check(&root_lemma).is_empty(),
        "frob-lemma with Certified evidence must pass via DOORWAY_SITES"
    );
}

// ============================================================================
// Rule 11: exact line counting (10+20+30=60, excludes target/, CR fault) & cap thresholds
// ============================================================================

#[test]
fn line_count_exact() {
    let root = create_clean_contact_workspace("line-count-exact");
    // Write 10 lines in src/lib.rs, 20 lines in src/mod_a.rs (@generated), 30 lines in tests/conformance.rs -> total 60.
    // Plus 500 lines inside crates/frob-contact/target/debug/gen.rs -> must NOT be counted!
    let ten = (0..10).map(|i| format!("// line {i}\n")).collect::<String>();
    let mut twenty = String::from("// @generated fixture\n");
    for i in 1..20 {
        twenty.push_str(&format!("// gen {i}\n"));
    }
    let thirty = (0..30).map(|i| format!("// test {i}\n")).collect::<String>();
    let target_junk = (0..500).map(|i| format!("// target {i}\n")).collect::<String>();

    write_file(&root.join("crates/frob-contact/src/lib.rs"), &ten);
    write_file(&root.join("crates/frob-contact/src/mod_a.rs"), &twenty);
    write_file(&root.join("crates/frob-contact/tests/conformance.rs"), &thirty);
    write_file(
        &root.join("crates/frob-contact/target/debug/gen.rs"),
        &target_junk,
    );

    let (code, json_text) = run_cli_json(&root);
    assert_eq!(code, ExitCode::from(EXIT_OK));
    let adm = admit(json_text.as_bytes(), &Limits::default()).expect("admit");
    let facts = adm.value().get("facts").expect("facts");
    let crates = facts
        .get("crates")
        .and_then(|v| v.as_array())
        .expect("crates");
    assert_eq!(crates.len(), 1);
    assert_eq!(crates[0].get("lines").and_then(|v| v.as_int53().ok()), Some(60));
    assert_eq!(
        crates[0]
            .get("generated_lines")
            .and_then(|v| v.as_int53().ok()),
        Some(20)
    );
    assert_eq!(
        facts
            .get("workspace_total")
            .and_then(|v| v.get("lines"))
            .and_then(|v| v.as_int53().ok()),
        Some(60)
    );

    // CRLF line endings in any `*.rs` file must return a tree fault (CheckError::Fault).
    let root_crlf = create_clean_contact_workspace("line-count-crlf-fault");
    write_file(
        &root_crlf.join("crates/frob-contact/src/lib.rs"),
        "pub fn bad() {}\r\n",
    );
    let err = check_contracts(&root_crlf, &[WorkspaceRoot::core(&root_crlf)])
        .expect_err("CRLF file must return tree fault");
    assert!(matches!(err, CheckError::Fault(ref msg) if msg.contains("CR/CRLF")));
}

#[test]
fn cap_thresholds() {
    const CAP1_TABLE: &[CrateRow] = &[CrateRow {
        name: "frob-contact",
        dir: "crates/frob-contact",
        layer: Layer::Core(3),
        rank: 2,
        tiers: &[Tier::Solid],
        chapters: &["§18"],
        cap_kloc: 1,
        reason: Reason::Original,
        adr: None,
        conditional: None,
    }];

    // Helper to build a workspace with an exact total line count across src/lib.rs + tests/conformance.rs.
    let run_with_lines = |label: &str, total_lines: usize| {
        let root = create_clean_contact_workspace(label);
        let conf = "#[test]\nfn ok() {}\n"; // 2 lines
        assert!(total_lines >= 2);
        let lib_lines = total_lines - 2;
        let lib = (0..lib_lines)
            .map(|i| format!("// l{i}\n"))
            .collect::<String>();
        write_file(&root.join("crates/frob-contact/src/lib.rs"), &lib);
        write_file(&root.join("crates/frob-contact/tests/conformance.rs"), conf);
        check_contracts_with_tables(
            &root,
            &[WorkspaceRoot::core(&root)],
            CAP1_TABLE,
            DOORWAY_SITES,
        )
        .expect("check_contracts_with_tables")
    };

    // 1. 1,000 lines against cap 1 (1,000): listed as `at-cap`, passes, NOT in watch.
    let o1000 = run_with_lines("cap-1000", 1000);
    assert!(o1000.findings.is_empty(), "1000 lines must pass: {:?}", o1000.findings);
    let r1000 = CheckReport::new("check-contracts", ".", o1000.findings, o1000.facts, 0)
        .to_canonical_json()
        .expect("json");
    let a1000 = admit(r1000.as_bytes(), &Limits::default()).expect("admit");
    let c1000 = &a1000.value().get("facts").unwrap().get("crates").unwrap().as_array().unwrap()[0];
    assert_eq!(c1000.get("status").and_then(|v| v.as_str()), Some("at-cap"));
    assert!(
        a1000.value().get("facts").unwrap().get("watch").unwrap().as_array().unwrap().is_empty()
    );

    // 2. 1,149 lines against cap 1: listed under `facts.watch`, passes with 0 findings.
    let o1149 = run_with_lines("cap-1149", 1149);
    assert!(o1149.findings.is_empty(), "1149 lines must pass: {:?}", o1149.findings);
    let r1149 = CheckReport::new("check-contracts", ".", o1149.findings, o1149.facts, 0)
        .to_canonical_json()
        .expect("json");
    let a1149 = admit(r1149.as_bytes(), &Limits::default()).expect("admit");
    let watch = a1149
        .value()
        .get("facts")
        .unwrap()
        .get("watch")
        .unwrap()
        .as_array()
        .unwrap();
    assert_eq!(watch.len(), 1);
    assert_eq!(watch[0].as_str(), Some("frob-contact"));

    // 3. 1,150 lines against cap 1: fails with `cap.over-115` (`>=`, not `>`!).
    let o1150 = run_with_lines("cap-1150", 1150);
    assert_eq!(o1150.findings.len(), 1, "1150 lines must fail: {:?}", o1150.findings);
    assert_eq!(o1150.findings[0].rule, RULE_CAP_OVER_115);
    assert_eq!(o1150.findings[0].subject, "frob-contact");
}

// ============================================================================
// Rule 1b: dependency target missing contract reports `contract.missing-for-target`
// ============================================================================

#[test]
fn dependency_target_needs_contract() {
    let root = make_scratch_dir("dep-target-missing");
    write_file(
        &root.join("Cargo.toml"),
        "[workspace]\nresolver = \"3\"\nmembers = [\"crates/*\"]\n",
    );
    seed_registries(&root);

    // Target crate `frob-core` lacks CONTRACT.md (and contract.toml).
    write_file(
        &root.join("crates/frob-core/Cargo.toml"),
        "[package]\nname = \"frob-core\"\nversion = \"0.0.1\"\nedition = \"2024\"\n",
    );
    write_file(&root.join("crates/frob-core/src/lib.rs"), "pub fn id() -> u32 { 0 }\n");
    write_file(&root.join("crates/frob-core/tests/conformance.rs"), "#[test]\nfn ok() {}\n");

    // Dependent crate `frob-contact` has full contract and depends on `frob-core`.
    write_file(
        &root.join("crates/frob-contact/Cargo.toml"),
        "[package]\nname = \"frob-contact\"\nversion = \"0.0.1\"\nedition = \"2024\"\n\
         \n[dependencies]\nfrob-core = { path = \"../frob-core\" }\n",
    );
    write_file(
        &root.join("crates/frob-contact/src/lib.rs"),
        "pub fn use_core() -> u32 { frob_core::id() }\n",
    );
    write_file(
        &root.join("crates/frob-contact/CONTRACT.md"),
        &full_contract_md("frob-contact"),
    );
    write_file(
        &root.join("crates/frob-contact/contract.toml"),
        "[crate]\nname = \"frob-contact\"\nlayer = 3\nrank = 2\nplane = \"compute\"\n\
         determinism = \"D0\"\nevidence = \"Estimated\"\ngates = []\nrefusals = []\n\
         \n[budgets]\nwall_ms = 50\nmemory_mib = 64\n\
         \n[safe_mode]\non_budget = \"reference-path\"\non_calibration = \"reference-path\"\n",
    );
    write_file(
        &root.join("crates/frob-contact/tests/conformance.rs"),
        "#[test]\nfn ok() {}\n",
    );

    let findings = run_check(&root);
    assert_eq!(findings.len(), 1, "expected 1 finding, got: {findings:?}");
    assert_eq!(findings[0].rule, RULE_CONTRACT_MISSING_FOR_TARGET);
    assert_eq!(findings[0].subject, "frob-core");
    assert!(
        findings[0].message.contains("frob-contact"),
        "finding must name dependent crate: {}",
        findings[0].message
    );

    // By contrast, an undepended crate missing CONTRACT.md emits `contract.missing`.
    let root_solo = make_scratch_dir("solo-missing");
    write_file(
        &root_solo.join("Cargo.toml"),
        "[workspace]\nresolver = \"3\"\nmembers = [\"crates/*\"]\n",
    );
    seed_registries(&root_solo);
    write_file(
        &root_solo.join("crates/frob-core/Cargo.toml"),
        "[package]\nname = \"frob-core\"\nversion = \"0.0.1\"\nedition = \"2024\"\n",
    );
    write_file(&root_solo.join("crates/frob-core/src/lib.rs"), "pub fn id() {}\n");
    write_file(
        &root_solo.join("crates/frob-core/tests/conformance.rs"),
        "#[test]\nfn ok() {}\n",
    );
    let f_solo = run_check(&root_solo);
    assert_eq!(f_solo.len(), 1, "expected 1 finding, got: {f_solo:?}");
    assert_eq!(f_solo[0].rule, RULE_CONTRACT_MISSING);
}

// ============================================================================
// Templates pass & clean repository tree check
// ============================================================================

#[test]
fn templates_pass_the_check() {
    let md_tpl = fs::read_to_string(repo_root().join("assets/templates/CONTRACT.md"))
        .expect("assets/templates/CONTRACT.md must exist");
    let toml_tpl = fs::read_to_string(repo_root().join("assets/templates/contract.toml"))
        .expect("assets/templates/contract.toml must exist");

    // 1. Unfilled CONTRACT.md template has only HTML comments -> fails Rule 2 (10 empty sections).
    let root_unfilled = create_clean_contact_workspace("templates-unfilled");
    write_file(&root_unfilled.join("crates/frob-contact/CONTRACT.md"), &md_tpl);
    let f_unfilled = run_check(&root_unfilled);
    assert_eq!(
        f_unfilled.len(),
        10,
        "unfilled CONTRACT.md template must report 10 empty sections: {f_unfilled:?}"
    );
    assert!(
        f_unfilled
            .iter()
            .all(|f| f.rule == RULE_CONTRACT_SECTION_MISSING)
    );

    // 2. Synthetic crate created from both templates with placeholders filled passes Rules 1 to 4+.
    let root_ok = make_scratch_dir("templates-filled-pass");
    write_file(
        &root_ok.join("Cargo.toml"),
        "[workspace]\nresolver = \"3\"\nmembers = [\"crates/*\"]\n",
    );
    seed_registries(&root_ok);
    write_file(
        &root_ok.join("crates/frob-core/Cargo.toml"),
        "[package]\nname = \"frob-core\"\nversion = \"0.0.1\"\nedition = \"2024\"\n",
    );
    write_file(&root_ok.join("crates/frob-core/src/lib.rs"), "pub fn ok() {}\n");
    write_file(
        &root_ok.join("crates/frob-core/tests/conformance.rs"),
        "#[test]\nfn ok() {}\n",
    );

    let mut filled_md = md_tpl.replace("<crate-name>", "frob-core");
    for &sec in REQUIRED_CONTRACT_SECTIONS {
        let heading = format!("## {sec}");
        filled_md = filled_md.replace(&heading, &format!("{heading}\n\nFilled content for {sec}."));
    }
    let filled_toml = toml_tpl.replace("name = \"frob-example\"", "name = \"frob-core\"");
    write_file(&root_ok.join("crates/frob-core/CONTRACT.md"), &filled_md);
    write_file(&root_ok.join("crates/frob-core/contract.toml"), &filled_toml);

    let findings = run_check(&root_ok);
    assert!(
        findings.is_empty(),
        "filled templates must pass check-contracts cleanly: {findings:?}"
    );
}

#[test]
fn clean_tree_has_no_finding() {
    let root = repo_root();
    assert_eq!(OWNED_RULES.len(), 13);

    // Verify xtask ships all three contract files.
    assert!(root.join("crates/xtask/CONTRACT.md").is_file());
    assert!(root.join("crates/xtask/contract.toml").is_file());
    assert!(root.join("crates/xtask/tests/conformance.rs").is_file());

    let started = Instant::now();
    let outcome = check_contracts(&root, &[WorkspaceRoot::core(&root)])
        .expect("check_contracts must succeed on real repo");
    let elapsed = started.elapsed();

    assert!(
        elapsed.as_secs() < 10,
        "check-contracts must finish in under 10 s on warm tree, took {elapsed:?}"
    );

    // Verify every finding on the real tree (ADR-WS-05 check: zero contract/schema/section/conformance defects).
    assert!(
        outcome.findings.is_empty(),
        "clean tree must have zero check-contracts findings, got: {:?}",
        outcome.findings
    );

    // Verify workspace_total.lines equals sum of per-crate counts and cap_kloc matches crate_map::total_cap_kloc().
    let report = CheckReport::new("check-contracts", ".", outcome.findings, outcome.facts, 0);
    let json1 = report.to_canonical_json().expect("canon 1");
    validate_report_json(json1.as_bytes()).expect("valid frob.xtask.report/1");

    let adm = admit(json1.as_bytes(), &Limits::default()).expect("admit");
    let facts = adm.value().get("facts").expect("facts");
    let crates = facts.get("crates").and_then(|v| v.as_array()).expect("crates");
    let sum_lines: i64 = crates
        .iter()
        .map(|c| c.get("lines").and_then(|v| v.as_int53().ok()).unwrap_or(0))
        .sum();
    let ws_total = facts.get("workspace_total").expect("workspace_total");
    assert_eq!(
        ws_total.get("lines").and_then(|v| v.as_int53().ok()),
        Some(sum_lines),
        "workspace_total.lines must equal sum of per-crate counts"
    );
    assert_eq!(
        ws_total.get("cap_kloc").and_then(|v| v.as_int53().ok()),
        Some(i64::from(total_cap_kloc())),
        "workspace_total.cap_kloc must match crate_map::total_cap_kloc() (ADR-WS-04: 576)"
    );
    assert_eq!(
        CRATE_MAP.iter().filter(|r| r.is_counted()).count(),
        43
    );

    // Verify human output prints the per-crate line table.
    let human = report.format_human();
    assert!(human.contains("crates ("));
    assert!(human.contains("workspace_total:"));

    // Verify D0 byte-identical determinism across repeated runs.
    let outcome2 = check_contracts(&root, &[WorkspaceRoot::core(&root)]).expect("run 2");
    let json2 = CheckReport::new("check-contracts", ".", outcome2.findings, outcome2.facts, 0)
        .to_canonical_json()
        .expect("canon 2");
    assert_eq!(json1, json2, "check-contracts output must be D0 deterministic");
}

#[test]
fn airlock_feature_group_records_accepted_and_non_group_features_omitted() {
    let root = make_scratch_dir("airlock-feature-records");
    write_file(
        &root.join("Cargo.toml"),
        "[workspace]\nresolver = \"3\"\nmembers = [\"crates/*\"]\n",
    );
    write_file(
        &root.join("registries/refusals.toml"),
        "schema = \"frob.refusals/1\"\n[[refusal]]\ncode = \"FROB-REF-790\"\n",
    );
    write_file(
        &root.join("registries/gates.toml"),
        "schema = \"frob.gates/1\"\n",
    );
    write_file(
        &root.join("crates/frob-fsim/Cargo.toml"),
        "[package]\nname = \"frob-fsim\"\nversion = \"0.0.1\"\nedition = \"2024\"\n\n\
         [features]\ndefault = [\"referee\", \"mode-inproc\"]\n\
         mode-inproc = []\nmode-remote = []\nconformance = [\"mode-inproc\"]\n\
         referee = [\"mode-inproc\"]\n",
    );
    write_file(&root.join("crates/frob-fsim/src/lib.rs"), "pub fn ok() {}\n");
    write_file(&root.join("crates/frob-fsim/tests/conformance.rs"), "#[test] fn ok() {}\n");
    write_file(&root.join("crates/frob-fsim/CONTRACT.md"), &full_contract_md("frob-fsim"));
    write_file(
        &root.join("crates/frob-fsim/contract.toml"),
        "[crate]\nname = \"frob-fsim\"\nlayer = 0\nrank = 2\nplane = \"compute\"\n\
         determinism = \"D0\"\nevidence = \"Tested\"\ngates = []\nrefusals = [\"FROB-REF-790\"]\n\n\
         [budgets]\nwall_ms = 50\nmemory_mib = 64\n\n\
         [safe_mode]\non_budget = \"reference-path\"\non_calibration = \"reference-path\"\n\n\
         [features.referee]\ntier = \"S\"\ndefault = true\ndev_only = false\n\
         traits = [\"RefereeDynamics\"]\ncrates = [\"fs-mbd\"]\n\
         consumers = [\"frob-dyn\"]\ntest_consumers = [\"frob-conformance\"]\n\
         first_milestone = \"M0\"\ngate = \"INT-FSIM-01\"\n",
    );

    let findings = run_check(&root);
    assert!(
        findings.is_empty(),
        "valid frob-fsim airlock feature record must pass check-contracts, got: {findings:?}"
    );

    // Listing non-group `conformance` in contract.toml must fail RULE_CONTRACT_FEATURES_MISMATCH.
    write_file(
        &root.join("crates/frob-fsim/contract.toml"),
        "[crate]\nname = \"frob-fsim\"\nlayer = 0\nrank = 2\nplane = \"compute\"\n\
         determinism = \"D0\"\nevidence = \"Tested\"\ngates = []\nrefusals = [\"FROB-REF-790\"]\n\n\
         [budgets]\nwall_ms = 50\nmemory_mib = 64\n\n\
         [safe_mode]\non_budget = \"reference-path\"\non_calibration = \"reference-path\"\n\n\
         [features.referee]\ntier = \"S\"\n\
         [features.conformance]\ntier = \"S\"\n",
    );
    let bad = run_check(&root);
    assert!(
        bad.iter().any(|f| f.rule == RULE_CONTRACT_FEATURES_MISMATCH),
        "non-group feature `conformance` in frob-fsim contract.toml must fail features-mismatch: {bad:?}"
    );
}
