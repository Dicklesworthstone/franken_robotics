//! `xtask check-contracts`: `CONTRACT.md` sections, `contract.toml` schema and
//! registry cross-checks, doorway evidence rules, and crate line caps (bead N34-08).
//!
//! Enforces eleven rule groups over declared `cargo metadata --no-deps` edges,
//! `CONTRACT.md`, `contract.toml` (read strictly through `frob_core::registry::toml`),
//! `registries/refusals.toml`, `registries/gates.toml`, `crate_map::CRATE_MAP`,
//! `check_unsafe::DOORWAY_SITES`, and workspace `*.rs` source files:
//! 1. `contract.missing` / `contract.missing-for-target`
//! 2. `contract.section-missing` (ten required level-2 sections, non-empty bodies)
//! 3. `contract.no-conformance-test` (`tests/conformance.rs` presence)
//! 4. `contract.schema` (`contract.toml` against `schemas/contract.schema.json`)
//! 5. `contract.layer-mismatch` (`layer`/`rank` against `CRATE_MAP` only)
//! 6. `contract.features-mismatch` (manifest `[features]` parity + F/M prefix & kill gate)
//! 7. `contract.unknown-reference` (`refusals`, `gates`, and feature `kill` gates)
//! 8. `contract.safe-mode` (`reference-path` or `refuse:<code>` with registered code)
//! 9. `contract.plane` (`compute` or `decision`; L-11 layer contradictions stay in `check-layers`)
//! 10. `contract.float-ok` & `contract.determinism-evidence` (reading `DOORWAY_SITES`)
//! 11. `cap.over-115` (LF-exact line counting, >= 115% cap, `facts.watch`, non-Rust & workspace totals)
//!
//! Determinism class D0: findings and facts are strictly sorted and repeatable;
//! non-LF (`\r`) line endings in `*.rs` files are reported as a tree fault.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

use frob_core::registry::toml::{self, Fields, Spanned, Table, Value};

use crate::check_unsafe::{DOORWAY_SITES, DoorwaySite};
use crate::cli::{CheckContext, CheckError, CheckOutcome};
use crate::crate_map::{
    CAP_TRIGGER_PERCENT, CRATE_MAP, CrateRow, Finding, Layer, NON_RUST_CAPS, lookup_in,
    trigger_cap_kloc,
};
use crate::metadata::{
    PackageMeta, WorkspaceKind, WorkspaceRoot, canonical_or_clean, load_declared_metadata,
    normalize_rel_path,
};
use crate::report::FactValue;

/// Rule 1a: a workspace member lacks `CONTRACT.md` or `contract.toml`.
pub const RULE_CONTRACT_MISSING: &str = "contract.missing";
/// Rule 1b: a crate depended on by another repository crate lacks `CONTRACT.md` or `contract.toml`.
pub const RULE_CONTRACT_MISSING_FOR_TARGET: &str = "contract.missing-for-target";
/// Rule 2: `CONTRACT.md` lacks one of the ten required level-2 sections or a section is empty.
pub const RULE_CONTRACT_SECTION_MISSING: &str = "contract.section-missing";
/// Rule 3: `tests/conformance.rs` is missing.
pub const RULE_CONTRACT_NO_CONFORMANCE_TEST: &str = "contract.no-conformance-test";
/// Rule 4: `contract.toml` fails `schemas/contract.schema.json`.
pub const RULE_CONTRACT_SCHEMA: &str = "contract.schema";
/// Rule 5: `layer` or `rank` in `contract.toml` differs from the crate map.
pub const RULE_CONTRACT_LAYER_MISMATCH: &str = "contract.layer-mismatch";
/// Rule 6: `[features]` disagrees with manifest `[features]` or F/M tier lacks prefix/kill gate.
pub const RULE_CONTRACT_FEATURES_MISMATCH: &str = "contract.features-mismatch";
/// Rule 7: a cited refusal code, gate ID, or feature kill gate is absent from its registry.
pub const RULE_CONTRACT_UNKNOWN_REFERENCE: &str = "contract.unknown-reference";
/// Rule 8: `[safe_mode]` value is neither `reference-path` nor `refuse:<code>` with registered code.
pub const RULE_CONTRACT_SAFE_MODE: &str = "contract.safe-mode";
/// Rule 9: `[crate].plane` is neither `compute` nor `decision`.
pub const RULE_CONTRACT_PLANE: &str = "contract.plane";
/// Rule 10a: an `[api.float_ok]` entry has an empty reason string.
pub const RULE_CONTRACT_FLOAT_OK: &str = "contract.float-ok";
/// Rule 10b: `D4` with evidence above `Estimated`, or `Certified`/`Proved` outside doorway crates.
pub const RULE_CONTRACT_DETERMINISM_EVIDENCE: &str = "contract.determinism-evidence";
/// Rule 11: crate physical Rust line count is >= 115% of its scope cap.
pub const RULE_CAP_OVER_115: &str = "cap.over-115";

/// Complete list of finding rule identifiers owned exclusively by `check-contracts`.
pub const OWNED_RULES: &[&str] = &[
    RULE_CONTRACT_MISSING,
    RULE_CONTRACT_MISSING_FOR_TARGET,
    RULE_CONTRACT_SECTION_MISSING,
    RULE_CONTRACT_NO_CONFORMANCE_TEST,
    RULE_CONTRACT_SCHEMA,
    RULE_CONTRACT_LAYER_MISMATCH,
    RULE_CONTRACT_FEATURES_MISMATCH,
    RULE_CONTRACT_UNKNOWN_REFERENCE,
    RULE_CONTRACT_SAFE_MODE,
    RULE_CONTRACT_PLANE,
    RULE_CONTRACT_FLOAT_OK,
    RULE_CONTRACT_DETERMINISM_EVIDENCE,
    RULE_CAP_OVER_115,
];

/// Ten required level-2 (`##`) sections of every `CONTRACT.md` (rule 2).
pub const REQUIRED_CONTRACT_SECTIONS: &[&str] = &[
    "Purpose",
    "Inputs",
    "Outputs",
    "Invariants",
    "Refusals",
    "Determinism class",
    "Evidence class of outputs",
    "Validity domain",
    "Oracles",
    "Known gaps",
];

/// Schema keys read by `check-contracts` (cross-checked against `schemas/contract.schema.json`).
pub const SCHEMA_TOP_KEYS: &[&str] = &["crate", "budgets", "safe_mode", "features", "api"];
pub const SCHEMA_CRATE_KEYS: &[&str] = &[
    "name",
    "layer",
    "rank",
    "plane",
    "determinism",
    "evidence",
    "gates",
    "refusals",
];
pub const SCHEMA_BUDGETS_REQUIRED_KEYS: &[&str] = &["wall_ms", "memory_mib"];
pub const SCHEMA_SAFE_MODE_KEYS: &[&str] = &["on_budget", "on_calibration"];
pub const SCHEMA_FEATURE_KEYS: &[&str] = &[
    "tier",
    "kill",
    "pilot",
    "default",
    "dev_only",
    "traits",
    "crates",
    "consumers",
    "test_consumers",
    "first_milestone",
    "gate",
    "parent_group",
];
pub const SCHEMA_API_KEYS: &[&str] = &["float_ok"];

/// Non-group `frob-fsim` features omitted from `contract.toml` feature-group records (N33a-11).
const FROB_FSIM_NON_GROUP_FEATURES: &[&str] = &["mode-inproc", "mode-remote", "conformance"];

const VALID_DETERMINISM: &[&str] = &["D0", "D1", "D2", "D3", "D4"];
const VALID_EVIDENCE: &[&str] = &["Estimated", "Tested", "Certified", "Proved"];
const TEMPLATE_CONTRACT_MD: &str = "assets/templates/CONTRACT.md";
const TEMPLATE_CONTRACT_TOML: &str = "assets/templates/contract.toml";

/// One per-crate line-count and contract-metrics record emitted in report `facts`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrateLineFact {
    pub crate_name: String,
    pub rel_dir: String,
    pub lines: u64,
    pub generated_lines: u64,
    pub cap_kloc: Option<u16>,
    pub cap_lines: Option<u64>,
    pub trigger_lines: Option<u64>,
    pub ratio: Option<String>,
    pub status: String,
    pub float_ok_count: usize,
}

/// Non-Rust scope-cap observation emitted in report `facts` without findings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NonRustLineFact {
    pub name: String,
    pub dir: String,
    pub language: String,
    pub lines: u64,
    pub cap_kloc: Option<u16>,
    pub cap_lines: Option<u64>,
}

/// CLI handler plugged into the static `COMMANDS` array in `src/main.rs`.
pub fn run_command(ctx: &CheckContext) -> Result<CheckOutcome, CheckError> {
    if !ctx.extra_args.is_empty() {
        return Err(CheckError::Usage(format!(
            "check-contracts accepts no extra arguments: {}",
            ctx.extra_args.join(" ")
        )));
    }
    check_contracts(&ctx.repo_root, &ctx.roots)
}

/// Run `check-contracts` over `roots` using the compiled `CRATE_MAP` and `DOORWAY_SITES`.
pub fn check_contracts(
    repo_root: &Path,
    roots: &[WorkspaceRoot],
) -> Result<CheckOutcome, CheckError> {
    check_contracts_with_tables(repo_root, roots, CRATE_MAP, DOORWAY_SITES)
}

/// Run `check-contracts` with explicit crate map and doorway table (used by unit tests).
pub fn check_contracts_with_tables(
    repo_root: &Path,
    roots: &[WorkspaceRoot],
    crate_map: &[CrateRow],
    doorways: &[DoorwaySite],
) -> Result<CheckOutcome, CheckError> {
    let mut findings: Vec<Finding> = Vec::new();
    let mut checked_workspaces: Vec<String> = Vec::new();
    let mut absent_workspaces: Vec<String> = Vec::new();
    let mut all_members: Vec<PackageMeta> = Vec::new();

    for ws in roots {
        if !ws.has_manifest() {
            if ws.kind == WorkspaceKind::Core {
                return Err(CheckError::Usage(format!(
                    "core workspace manifest not found at {}",
                    ws.manifest_path().display()
                )));
            }
            absent_workspaces.push(ws.name.clone());
            continue;
        }

        let declared = load_declared_metadata(repo_root, ws).map_err(|err| {
            if err.is_usage_error() {
                CheckError::Usage(err.to_string())
            } else {
                CheckError::Fault(err.to_string())
            }
        })?;
        checked_workspaces.push(ws.name.clone());
        all_members.extend(declared.members);
    }

    all_members.sort_by(|a, b| a.name.cmp(&b.name).then_with(|| a.rel_dir.cmp(&b.rel_dir)));
    all_members.dedup_by(|a, b| a.name == b.name && a.rel_dir == b.rel_dir);

    // Build reverse-dependency map across all discovered workspace members for Rule 1b.
    let member_names: BTreeSet<String> = all_members.iter().map(|m| m.name.clone()).collect();
    let mut dependents_of: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for pkg in &all_members {
        for dep in &pkg.dependencies {
            if dep.name != pkg.name && member_names.contains(&dep.name) {
                dependents_of
                    .entry(dep.name.clone())
                    .or_default()
                    .insert(pkg.name.clone());
            }
        }
    }

    // Load registered refusal codes and gate IDs strictly through `frob_core::registry::toml`.
    let registered_refusals = load_registered_refusals(repo_root)?;
    let registered_gates = load_registered_gates(repo_root)?;

    let canon_repo = canonical_or_clean(repo_root);
    let mut crate_facts: Vec<CrateLineFact> = Vec::with_capacity(all_members.len());
    let mut watch_crates: Vec<String> = Vec::new();
    let mut over_115_crates: Vec<String> = Vec::new();
    let mut total_rust_lines: u64 = 0;
    let mut total_generated_rust_lines: u64 = 0;

    for pkg in &all_members {
        let crate_dir = pkg.crate_dir();
        let contract_md_path = crate_dir.join("CONTRACT.md");
        let contract_toml_path = crate_dir.join("contract.toml");
        let conformance_rs_path = crate_dir.join("tests").join("conformance.rs");

        let contract_md_rel = join_rel(&pkg.rel_dir, "CONTRACT.md");
        let contract_toml_rel = join_rel(&pkg.rel_dir, "contract.toml");
        let conformance_rs_rel = join_rel(&pkg.rel_dir, "tests/conformance.rs");

        let has_md = contract_md_path.is_file();
        let has_toml = contract_toml_path.is_file();

        // Rule 1: `contract.missing` vs `contract.missing-for-target`.
        if !has_md || !has_toml {
            let missing_desc = match (has_md, has_toml) {
                (false, false) => "CONTRACT.md and contract.toml",
                (false, true) => "CONTRACT.md",
                (true, false) => "contract.toml",
                (true, true) => unreachable!(),
            };
            let target_rel = if !has_md {
                contract_md_rel.clone()
            } else {
                contract_toml_rel.clone()
            };
            if let Some(deps) = dependents_of.get(&pkg.name)
                && !deps.is_empty()
            {
                let dep_list: Vec<&str> = deps.iter().map(String::as_str).collect();
                findings.push(Finding {
                    rule: RULE_CONTRACT_MISSING_FOR_TARGET,
                    subject: pkg.name.clone(),
                    path: target_rel,
                    line: None,
                    message: format!(
                        "dependency target `{}` is missing {missing_desc} (depended on by: {})",
                        pkg.name,
                        dep_list.join(", ")
                    ),
                    repair: format!(
                        "create `{contract_md_rel}` from `{TEMPLATE_CONTRACT_MD}` and `{contract_toml_rel}` from `{TEMPLATE_CONTRACT_TOML}` before other crates depend on `{}`",
                        pkg.name
                    ),
                });
            } else {
                findings.push(Finding {
                    rule: RULE_CONTRACT_MISSING,
                    subject: pkg.name.clone(),
                    path: target_rel,
                    line: None,
                    message: format!("crate `{}` is missing {missing_desc}", pkg.name),
                    repair: format!(
                        "copy `{TEMPLATE_CONTRACT_MD}` to `{contract_md_rel}` and `{TEMPLATE_CONTRACT_TOML}` to `{contract_toml_rel}` and fill all required fields"
                    ),
                });
            }
        }

        // Rule 2: `CONTRACT.md` ten required level-2 sections (non-empty).
        if has_md {
            let md_text = fs::read_to_string(&contract_md_path).map_err(|e| {
                CheckError::Fault(format!("failed reading {}: {e}", contract_md_path.display()))
            })?;
            check_contract_md_sections(&pkg.name, &contract_md_rel, &md_text, &mut findings);
        }

        // Rule 3: `tests/conformance.rs` presence.
        if !conformance_rs_path.is_file() {
            findings.push(Finding {
                rule: RULE_CONTRACT_NO_CONFORMANCE_TEST,
                subject: pkg.name.clone(),
                path: conformance_rs_rel.clone(),
                line: None,
                message: format!("crate `{}` is missing `tests/conformance.rs`", pkg.name),
                repair: format!(
                    "add `{conformance_rs_rel}` verifying `{contract_md_rel}` and `{contract_toml_rel}` (see `{TEMPLATE_CONTRACT_MD}` and `{TEMPLATE_CONTRACT_TOML}`)"
                ),
            });
        }

        // Rules 4..=10: `contract.toml` schema, layer/rank, features, registries, safe_mode, plane, float_ok, determinism/evidence.
        let mut float_ok_count = 0usize;
        if has_toml {
            let toml_bytes = fs::read(&contract_toml_path).map_err(|e| {
                CheckError::Fault(format!(
                    "failed reading {}: {e}",
                    contract_toml_path.display()
                ))
            })?;
            let manifest_features = read_manifest_features(&pkg.manifest_path)?;
            float_ok_count = check_contract_toml(
                pkg,
                &contract_toml_rel,
                &toml_bytes,
                &manifest_features,
                crate_map,
                &registered_refusals,
                &registered_gates,
                doorways,
                &mut findings,
            );
        }

        // Rule 11: exact LF line count across all `*.rs` files under crate_dir (excluding `target/`).
        let (crate_lines, generated_lines) = count_crate_rs_lines(&canon_repo, crate_dir)?;
        total_rust_lines = total_rust_lines.saturating_add(crate_lines);
        total_generated_rust_lines = total_generated_rust_lines.saturating_add(generated_lines);

        let map_row = lookup_in(crate_map, &pkg.name);
        let (cap_kloc, cap_lines, trigger_lines, ratio, status) = match map_row {
            Some(row) => {
                let cap_l = u64::from(row.cap_kloc) * 1000;
                let trig_l = (cap_l * u64::from(CAP_TRIGGER_PERCENT)) / 100;
                let ratio_str = format_ratio(crate_lines, cap_l);
                let st = if crate_lines.saturating_mul(100)
                    >= cap_l.saturating_mul(u64::from(CAP_TRIGGER_PERCENT))
                {
                    over_115_crates.push(pkg.name.clone());
                    findings.push(Finding {
                        rule: RULE_CAP_OVER_115,
                        subject: pkg.name.clone(),
                        path: pkg.rel_dir.clone(),
                        line: None,
                        message: format!(
                            "crate `{}` has {crate_lines} lines of Rust (ratio {ratio_str} against cap {} lines, >= 115% trigger {trig_l})",
                            pkg.name, cap_l
                        ),
                        repair: format!(
                            "hold a scope conversation for `{}`: record an ADR raising its cap in `crate_map.rs` or move a mechanism out",
                            pkg.name
                        ),
                    });
                    "over-115".to_owned()
                } else if crate_lines > cap_l {
                    watch_crates.push(pkg.name.clone());
                    "watch".to_owned()
                } else if crate_lines == cap_l {
                    "at-cap".to_owned()
                } else {
                    "under-cap".to_owned()
                };
                (
                    Some(row.cap_kloc),
                    Some(cap_l),
                    Some(trig_l),
                    Some(ratio_str),
                    st,
                )
            }
            None => (None, None, None, None, "unlisted".to_owned()),
        };

        crate_facts.push(CrateLineFact {
            crate_name: pkg.name.clone(),
            rel_dir: pkg.rel_dir.clone(),
            lines: crate_lines,
            generated_lines,
            cap_kloc,
            cap_lines,
            trigger_lines,
            ratio,
            status,
            float_ok_count,
        });
    }

    // Non-Rust scope caps (reported in facts, no findings).
    let non_rust_facts = count_non_rust_caps(&canon_repo, repo_root)?;

    // Workspace cap totals derived directly from `crate_map` (reflects ADR-WS-04: 576 / 662).
    let ws_cap_kloc: u32 = crate_map
        .iter()
        .filter(|r| r.is_counted())
        .map(|r| u32::from(r.cap_kloc))
        .sum();
    let ws_trigger_kloc = trigger_cap_kloc(ws_cap_kloc);

    findings.sort();
    findings.dedup();
    checked_workspaces.sort();
    absent_workspaces.sort();
    watch_crates.sort();
    over_115_crates.sort();
    crate_facts.sort_by(|a, b| a.crate_name.cmp(&b.crate_name));

    let facts = build_facts(
        &crate_facts,
        &watch_crates,
        &over_115_crates,
        total_rust_lines,
        total_generated_rust_lines,
        ws_cap_kloc,
        ws_trigger_kloc,
        &non_rust_facts,
        &checked_workspaces,
        &absent_workspaces,
    );

    Ok(CheckOutcome { findings, facts })
}

/// Render a deterministic human-readable per-crate line table and workspace totals.
#[must_use]
pub fn format_line_table(
    crates: &[CrateLineFact],
    workspace_lines: u64,
    workspace_cap_kloc: u32,
    workspace_trigger_kloc: u32,
) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "crates ({}):", crates.len());
    for c in crates {
        let cap_str = c
            .cap_lines
            .map_or_else(|| "none".to_owned(), |n| n.to_string());
        let ratio_str = c.ratio.as_deref().unwrap_or("n/a");
        let _ = writeln!(
            out,
            "  {}  lines={}  cap={}  ratio={}  status={}  float_ok={}",
            c.crate_name, c.lines, cap_str, ratio_str, c.status, c.float_ok_count
        );
    }
    let _ = writeln!(
        out,
        "workspace_total: lines={workspace_lines} cap_kloc={workspace_cap_kloc} trigger_kloc={workspace_trigger_kloc}"
    );
    out
}

#[allow(clippy::too_many_arguments)]
fn build_facts(
    crate_facts: &[CrateLineFact],
    watch_crates: &[String],
    over_115_crates: &[String],
    total_rust_lines: u64,
    total_generated_rust_lines: u64,
    ws_cap_kloc: u32,
    ws_trigger_kloc: u32,
    non_rust_facts: &[NonRustLineFact],
    checked_workspaces: &[String],
    absent_workspaces: &[String],
) -> BTreeMap<String, FactValue> {
    let mut facts = BTreeMap::new();

    let crate_vals: Vec<FactValue> = crate_facts
        .iter()
        .map(|c| {
            let mut obj = BTreeMap::new();
            obj.insert("crate".to_owned(), FactValue::str(c.crate_name.clone()));
            obj.insert("dir".to_owned(), FactValue::str(c.rel_dir.clone()));
            obj.insert("lines".to_owned(), fact_u64(c.lines));
            obj.insert("generated_lines".to_owned(), fact_u64(c.generated_lines));
            obj.insert(
                "cap_kloc".to_owned(),
                c.cap_kloc.map_or(FactValue::Null, |k| FactValue::Int(i64::from(k))),
            );
            obj.insert(
                "cap_lines".to_owned(),
                c.cap_lines.map_or(FactValue::Null, fact_u64),
            );
            obj.insert(
                "trigger_lines".to_owned(),
                c.trigger_lines.map_or(FactValue::Null, fact_u64),
            );
            obj.insert(
                "ratio".to_owned(),
                c.ratio
                    .as_ref()
                    .map_or(FactValue::Null, |r| FactValue::str(r.clone())),
            );
            obj.insert("status".to_owned(), FactValue::str(c.status.clone()));
            obj.insert(
                "float_ok_count".to_owned(),
                FactValue::int_usize(c.float_ok_count),
            );
            FactValue::Object(obj)
        })
        .collect();

    let non_rust_vals: Vec<FactValue> = non_rust_facts
        .iter()
        .map(|n| {
            let mut obj = BTreeMap::new();
            obj.insert("name".to_owned(), FactValue::str(n.name.clone()));
            obj.insert("dir".to_owned(), FactValue::str(n.dir.clone()));
            obj.insert("language".to_owned(), FactValue::str(n.language.clone()));
            obj.insert("lines".to_owned(), fact_u64(n.lines));
            obj.insert(
                "cap_kloc".to_owned(),
                n.cap_kloc.map_or(FactValue::Null, |k| FactValue::Int(i64::from(k))),
            );
            obj.insert(
                "cap_lines".to_owned(),
                n.cap_lines.map_or(FactValue::Null, fact_u64),
            );
            FactValue::Object(obj)
        })
        .collect();

    let ws_cap_lines = u64::from(ws_cap_kloc) * 1000;
    let ws_trigger_lines = (ws_cap_lines * u64::from(CAP_TRIGGER_PERCENT)) / 100;
    let mut ws_total = BTreeMap::new();
    ws_total.insert("lines".to_owned(), fact_u64(total_rust_lines));
    ws_total.insert(
        "generated_rust_lines".to_owned(),
        fact_u64(total_generated_rust_lines),
    );
    ws_total.insert("cap_kloc".to_owned(), FactValue::Int(i64::from(ws_cap_kloc)));
    ws_total.insert("cap_lines".to_owned(), fact_u64(ws_cap_lines));
    ws_total.insert(
        "trigger_kloc".to_owned(),
        FactValue::Int(i64::from(ws_trigger_kloc)),
    );
    ws_total.insert("trigger_lines".to_owned(), fact_u64(ws_trigger_lines));
    ws_total.insert(
        "ratio".to_owned(),
        if ws_cap_lines > 0 {
            FactValue::str(format_ratio(total_rust_lines, ws_cap_lines))
        } else {
            FactValue::Null
        },
    );

    facts.insert("crates".to_owned(), FactValue::Array(crate_vals));
    facts.insert(
        "generated_rust_lines".to_owned(),
        fact_u64(total_generated_rust_lines),
    );
    facts.insert("non_rust_caps".to_owned(), FactValue::Array(non_rust_vals));
    facts.insert(
        "over_115".to_owned(),
        FactValue::str_array(over_115_crates.iter().cloned()),
    );
    facts.insert(
        "watch".to_owned(),
        FactValue::str_array(watch_crates.iter().cloned()),
    );
    facts.insert("workspace_total".to_owned(), FactValue::Object(ws_total));
    facts.insert(
        "workspaces_absent".to_owned(),
        FactValue::str_array(absent_workspaces.iter().cloned()),
    );
    facts.insert(
        "workspaces_checked".to_owned(),
        FactValue::str_array(checked_workspaces.iter().cloned()),
    );
    facts
}

fn fact_u64(n: u64) -> FactValue {
    FactValue::Int(i64::try_from(n).unwrap_or(i64::MAX))
}

fn format_ratio(lines: u64, cap_lines: u64) -> String {
    if cap_lines == 0 {
        return "0.000".to_owned();
    }
    let scaled = lines.saturating_mul(1000) / cap_lines;
    let whole = scaled / 1000;
    let frac = scaled % 1000;
    format!("{whole}.{frac:03}")
}

fn join_rel(rel_dir: &str, child: &str) -> String {
    if rel_dir.is_empty() {
        child.to_owned()
    } else {
        format!("{rel_dir}/{child}")
    }
}

// ============================================================================
// Registry loaders (strictly via frob_core::registry::toml)
// ============================================================================

fn load_registered_refusals(repo_root: &Path) -> Result<BTreeSet<String>, CheckError> {
    let path = repo_root.join("registries").join("refusals.toml");
    if !path.is_file() {
        return Ok(BTreeSet::new());
    }
    let bytes = fs::read(&path)
        .map_err(|e| CheckError::Fault(format!("failed reading {}: {e}", path.display())))?;
    let root = toml::parse(&bytes)
        .map_err(|e| CheckError::Fault(format!("failed parsing registries/refusals.toml: {e}")))?;
    let mut codes = BTreeSet::new();
    if let Some(items) = root.get("refusal").and_then(Value::as_array) {
        for item in items {
            if let Some(tbl) = item.value.as_table()
                && let Some(code) = tbl.get("code").and_then(Value::as_str)
            {
                codes.insert(code.to_owned());
            }
        }
    }
    Ok(codes)
}

fn load_registered_gates(repo_root: &Path) -> Result<BTreeSet<String>, CheckError> {
    let path = repo_root.join("registries").join("gates.toml");
    if !path.is_file() {
        return Ok(BTreeSet::new());
    }
    let bytes = fs::read(&path)
        .map_err(|e| CheckError::Fault(format!("failed reading {}: {e}", path.display())))?;
    let root = toml::parse(&bytes)
        .map_err(|e| CheckError::Fault(format!("failed parsing registries/gates.toml: {e}")))?;
    let mut gates = BTreeSet::new();
    if let Some(items) = root.get("gate").and_then(Value::as_array) {
        for item in items {
            if let Some(tbl) = item.value.as_table()
                && let Some(id) = tbl.get("id").and_then(Value::as_str)
            {
                gates.insert(id.to_owned());
            }
        }
    }
    Ok(gates)
}

// ============================================================================
// Rule 2: CONTRACT.md ten level-2 sections
// ============================================================================

#[derive(Debug, Clone)]
struct MdSection {
    heading_line: u32,
    body: String,
}

fn check_contract_md_sections(
    crate_name: &str,
    rel_path: &str,
    text: &str,
    findings: &mut Vec<Finding>,
) {
    let sections = parse_level2_sections(text);

    for &required in REQUIRED_CONTRACT_SECTIONS {
        let key = normalize_contract_heading(required);
        match sections.get(&key) {
            None => {
                findings.push(Finding {
                    rule: RULE_CONTRACT_SECTION_MISSING,
                    subject: crate_name.to_owned(),
                    path: rel_path.to_owned(),
                    line: None,
                    message: format!(
                        "`{rel_path}` is missing required level-2 section `## {required}`"
                    ),
                    repair: format!(
                        "add a non-empty `## {required}` section to `{rel_path}` (see `{TEMPLATE_CONTRACT_MD}`)"
                    ),
                });
            }
            Some(sec) if strip_html_comments(&sec.body).trim().is_empty() => {
                findings.push(Finding {
                    rule: RULE_CONTRACT_SECTION_MISSING,
                    subject: crate_name.to_owned(),
                    path: rel_path.to_owned(),
                    line: Some(sec.heading_line),
                    message: format!(
                        "`{rel_path}` has an empty `## {required}` section"
                    ),
                    repair: format!(
                        "fill the `## {required}` section in `{rel_path}` (see `{TEMPLATE_CONTRACT_MD}`)"
                    ),
                });
            }
            Some(_) => {}
        }
    }
}

fn parse_level2_sections(text: &str) -> BTreeMap<String, MdSection> {
    let mut sections: BTreeMap<String, MdSection> = BTreeMap::new();
    let mut active: Option<(String, u32, String)> = None;
    let mut in_fence: Option<char> = None;

    for (idx, raw_line) in text.lines().enumerate() {
        let line_no = u32::try_from(idx + 1).unwrap_or(u32::MAX);
        let trimmed = raw_line.trim_start();

        if let Some(fence_ch) = in_fence {
            if is_fence_close(trimmed, fence_ch) {
                in_fence = None;
            }
            if let Some((_, _, ref mut body)) = active {
                body.push_str(raw_line);
                body.push('\n');
            }
            continue;
        }
        if let Some(fence_ch) = detect_fence_open(trimmed) {
            in_fence = Some(fence_ch);
            if let Some((_, _, ref mut body)) = active {
                body.push_str(raw_line);
                body.push('\n');
            }
            continue;
        }

        if let Some((level, title)) = parse_atx_heading(trimmed) {
            if level <= 2 {
                if let Some((key, heading_line, body)) = active.take() {
                    sections
                        .entry(key)
                        .or_insert(MdSection { heading_line, body });
                }
                if level == 2 {
                    let norm = normalize_contract_heading(title);
                    active = Some((norm, line_no, String::new()));
                }
                continue;
            }
            // Level 3+ subheading stays part of the active level-2 section's body.
            if let Some((_, _, ref mut body)) = active {
                body.push_str(raw_line);
                body.push('\n');
            }
            continue;
        }

        if let Some((_, _, ref mut body)) = active {
            body.push_str(raw_line);
            body.push('\n');
        }
    }

    if let Some((key, heading_line, body)) = active {
        sections
            .entry(key)
            .or_insert(MdSection { heading_line, body });
    }

    sections
}

fn detect_fence_open(trimmed: &str) -> Option<char> {
    if trimmed.starts_with("```") {
        Some('`')
    } else if trimmed.starts_with("~~~") {
        Some('~')
    } else {
        None
    }
}

fn is_fence_close(trimmed: &str, ch: char) -> bool {
    let count = trimmed.chars().take_while(|&c| c == ch).count();
    count >= 3 && trimmed[count..].trim().is_empty()
}

fn parse_atx_heading(trimmed: &str) -> Option<(usize, &str)> {
    let level = trimmed.bytes().take_while(|&b| b == b'#').count();
    if !(1..=6).contains(&level) {
        return None;
    }
    let rest = &trimmed[level..];
    if !rest.is_empty() && !rest.starts_with(' ') && !rest.starts_with('\t') {
        return None;
    }
    let title = rest.trim().trim_end_matches('#').trim();
    if title.is_empty() {
        return None;
    }
    Some((level, title))
}

fn normalize_contract_heading(raw: &str) -> String {
    let trimmed = raw.trim();
    // Accept optional leading section numbering (e.g. `1. Purpose` in frob-cert / frob-conformance)
    // alongside unnumbered headings (`Purpose`).
    let unnumbered = match trimmed.find(". ") {
        Some(idx) if idx > 0 && trimmed[..idx].bytes().all(|b| b.is_ascii_digit()) => {
            trimmed[idx + 2..].trim()
        }
        _ => trimmed,
    };
    unnumbered
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_ascii_lowercase()
}

fn strip_html_comments(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut rest = input;
    while let Some(start) = rest.find("<!--") {
        out.push_str(&rest[..start]);
        let after = &rest[start + 4..];
        if let Some(end) = after.find("-->") {
            rest = &after[end + 3..];
        } else {
            return out;
        }
    }
    out.push_str(rest);
    out
}

// ============================================================================
// Rules 4..=10: contract.toml schema, layer/rank, features, refs, safe_mode, plane, float_ok, det/ev
// ============================================================================

#[allow(clippy::too_many_arguments)]
fn check_contract_toml(
    pkg: &PackageMeta,
    rel_path: &str,
    bytes: &[u8],
    manifest_features: &BTreeSet<String>,
    crate_map: &[CrateRow],
    registered_refusals: &BTreeSet<String>,
    registered_gates: &BTreeSet<String>,
    doorways: &[DoorwaySite],
    findings: &mut Vec<Finding>,
) -> usize {
    let root = match toml::parse(bytes) {
        Ok(tbl) => tbl,
        Err(err) => {
            findings.push(Finding {
                rule: RULE_CONTRACT_SCHEMA,
                subject: pkg.name.clone(),
                path: rel_path.to_owned(),
                line: Some(err.line),
                message: format!("invalid TOML in `{rel_path}`: {err}"),
                repair: format!("fix TOML syntax/subset errors in `{rel_path}` (see `{TEMPLATE_CONTRACT_TOML}`)"),
            });
            return 0;
        }
    };

    let mut top = Fields::new(root, rel_path);
    let crate_tbl = take_required_table(&mut top, "crate", &pkg.name, rel_path, findings);
    let budgets_tbl = take_required_table(&mut top, "budgets", &pkg.name, rel_path, findings);
    let safe_mode_tbl = take_required_table(&mut top, "safe_mode", &pkg.name, rel_path, findings);
    let features_tbl = take_optional_table(&mut top, "features", &pkg.name, rel_path, findings);
    let api_tbl = take_optional_table(&mut top, "api", &pkg.name, rel_path, findings);
    if let Err(err) = top.finish() {
        push_schema_err(&pkg.name, rel_path, Some(err.line), err.to_string(), findings);
    }

    // Validate `[crate]` section (Rules 4, 5, 7, 9, 10b).
    if let Some(tbl) = crate_tbl {
        check_crate_section(
            pkg,
            rel_path,
            tbl,
            crate_map,
            registered_refusals,
            registered_gates,
            doorways,
            findings,
        );
    }

    // Validate `[budgets]` section (Rule 4).
    if let Some(tbl) = budgets_tbl {
        check_budgets_section(&pkg.name, rel_path, &tbl, findings);
    }

    // Validate `[safe_mode]` section (Rules 4 and 8).
    if let Some(tbl) = safe_mode_tbl {
        check_safe_mode_section(&pkg.name, rel_path, tbl, registered_refusals, findings);
    }

    // Validate `[features]` section (Rules 4, 6, 7).
    check_features_section(
        &pkg.name,
        rel_path,
        features_tbl,
        manifest_features,
        registered_gates,
        findings,
    );

    // Validate `[api]` / `[api.float_ok]` section (Rules 4 and 10a).
    let mut float_ok_count = 0usize;
    if let Some(tbl) = api_tbl {
        float_ok_count = check_api_section(&pkg.name, rel_path, tbl, findings);
    }
    float_ok_count
}

#[allow(clippy::too_many_arguments)]
fn check_crate_section(
    pkg: &PackageMeta,
    rel_path: &str,
    tbl: Table,
    crate_map: &[CrateRow],
    registered_refusals: &BTreeSet<String>,
    registered_gates: &BTreeSet<String>,
    doorways: &[DoorwaySite],
    findings: &mut Vec<Finding>,
) {
    let mut cf = Fields::new(tbl, "[crate]");

    let name_opt = take_required_str(&mut cf, "name", &pkg.name, rel_path, findings);
    let layer_sp = cf.take("layer");
    let rank_opt = take_required_int(&mut cf, "rank", &pkg.name, rel_path, findings);
    let plane_opt = take_required_str(&mut cf, "plane", &pkg.name, rel_path, findings);
    let det_opt = take_required_str(&mut cf, "determinism", &pkg.name, rel_path, findings);
    let ev_opt = take_required_str(&mut cf, "evidence", &pkg.name, rel_path, findings);
    let gates_opt = take_required_array(&mut cf, "gates", &pkg.name, rel_path, findings);
    let refs_opt = take_required_array(&mut cf, "refusals", &pkg.name, rel_path, findings);

    if let Err(err) = cf.finish() {
        push_schema_err(&pkg.name, rel_path, Some(err.line), err.to_string(), findings);
    }

    if let Some((name, line)) = name_opt {
        if name.trim().is_empty() {
            push_schema_err(
                &pkg.name,
                rel_path,
                Some(line),
                "`[crate].name` must not be empty".to_owned(),
                findings,
            );
        } else if name != pkg.name {
            push_schema_err(
                &pkg.name,
                rel_path,
                Some(line),
                format!(
                    "`[crate].name` is `{name}`, expected package name `{}`",
                    pkg.name
                ),
                findings,
            );
        }
    }

    // Parse `layer` (integer 0..=7 or `"annex"`).
    let parsed_layer: Option<(Layer, u32)> = match layer_sp {
        None => {
            push_schema_err(
                &pkg.name,
                rel_path,
                None,
                "missing required key `layer` in `[crate]`".to_owned(),
                findings,
            );
            None
        }
        Some(sp) => match &sp.value {
            Value::Int(n) if (0..=7).contains(n) => {
                Some((Layer::Core(u8::try_from(*n).unwrap_or(0)), sp.line))
            }
            Value::Str(s) if s == "annex" => Some((Layer::Annex, sp.line)),
            other => {
                push_schema_err(
                    &pkg.name,
                    rel_path,
                    Some(sp.line),
                    format!(
                        "`[crate].layer` must be integer 0..=7 or `\"annex\"`, found {}",
                        other.type_name()
                    ),
                    findings,
                );
                None
            }
        },
    };

    let parsed_rank: Option<(u8, u32)> = match rank_opt {
        Some((r, line)) if r >= 0 && r <= i64::from(u8::MAX) => {
            Some((u8::try_from(r).unwrap_or(0), line))
        }
        Some((r, line)) => {
            push_schema_err(
                &pkg.name,
                rel_path,
                Some(line),
                format!("`[crate].rank` ({r}) must be a non-negative integer in 0..=255"),
                findings,
            );
            None
        }
        None => None,
    };

    // Rule 5: compare `layer` and `rank` against `CRATE_MAP` ONLY (manifest is `check-layers`).
    if let Some(row) = lookup_in(crate_map, &pkg.name) {
        if let Some((layer, line)) = parsed_layer
            && layer != row.layer
        {
            findings.push(Finding {
                rule: RULE_CONTRACT_LAYER_MISMATCH,
                subject: pkg.name.clone(),
                path: rel_path.to_owned(),
                line: Some(line),
                message: format!(
                    "contract declares layer `{layer}`, but crate map assigns `{}` to `{}`",
                    row.layer, pkg.name
                ),
                repair: format!("set `[crate].layer` in `{rel_path}` to `{}`", row.layer),
            });
        }
        if let Some((rank, line)) = parsed_rank
            && rank != row.rank
        {
            findings.push(Finding {
                rule: RULE_CONTRACT_LAYER_MISMATCH,
                subject: pkg.name.clone(),
                path: rel_path.to_owned(),
                line: Some(line),
                message: format!(
                    "contract declares rank `{rank}`, but crate map assigns `{}` to `{}`",
                    row.rank, pkg.name
                ),
                repair: format!("set `[crate].rank` in `{rel_path}` to `{}`", row.rank),
            });
        }
    }

    // Rule 9: `plane` must be `"compute"` or `"decision"`.
    // (A layer 1..=3 crate declaring `"decision"` is owned exclusively by `check-layers` rule L-11.)
    if let Some((plane, line)) = plane_opt
        && plane != "compute"
        && plane != "decision"
    {
        findings.push(Finding {
            rule: RULE_CONTRACT_PLANE,
            subject: pkg.name.clone(),
            path: rel_path.to_owned(),
            line: Some(line),
            message: format!(
                "`[crate].plane` is `{plane}`; must be `\"compute\"` or `\"decision\"`"
            ),
            repair: format!("set `[crate].plane` in `{rel_path}` to `\"compute\"` or `\"decision\"`"),
        });
    }

    // Validate `determinism` & `evidence` enums (Rule 4) and Rule 10b (`contract.determinism-evidence`).
    let valid_det = match &det_opt {
        Some((det, line)) if !VALID_DETERMINISM.contains(&det.as_str()) => {
            push_schema_err(
                &pkg.name,
                rel_path,
                Some(*line),
                format!(
                    "`[crate].determinism` `{det}` must be one of {}",
                    VALID_DETERMINISM.join(", ")
                ),
                findings,
            );
            None
        }
        Some((det, line)) => Some((det.as_str(), *line)),
        None => None,
    };

    let valid_ev = match &ev_opt {
        Some((ev, line)) if !VALID_EVIDENCE.contains(&ev.as_str()) => {
            push_schema_err(
                &pkg.name,
                rel_path,
                Some(*line),
                format!(
                    "`[crate].evidence` `{ev}` must be one of {}",
                    VALID_EVIDENCE.join(", ")
                ),
                findings,
            );
            None
        }
        Some((ev, line)) => Some((ev.as_str(), *line)),
        None => None,
    };

    if let (Some((det, _)), Some((ev, ev_line))) = (valid_det, valid_ev) {
        if det == "D4" && ev != "Estimated" {
            findings.push(Finding {
                rule: RULE_CONTRACT_DETERMINISM_EVIDENCE,
                subject: pkg.name.clone(),
                path: rel_path.to_owned(),
                line: Some(ev_line),
                message: format!(
                    "crate `{}` declares `determinism = \"D4\"` with `evidence = \"{ev}\"` (D4 permits only `Estimated`)",
                    pkg.name
                ),
                repair: format!(
                    "lower `[crate].evidence` to `\"Estimated\"` or strengthen determinism class in `{rel_path}`"
                ),
            });
        }
        if (ev == "Certified" || ev == "Proved") && !crate_allowed_proof_doorway(&pkg.name, doorways)
        {
            findings.push(Finding {
                rule: RULE_CONTRACT_DETERMINISM_EVIDENCE,
                subject: pkg.name.clone(),
                path: rel_path.to_owned(),
                line: Some(ev_line),
                message: format!(
                    "crate `{}` declares `evidence = \"{ev}\"`, which is permitted only for Proof-claim doorway crates in `xtask::check_unsafe::DOORWAY_SITES`",
                    pkg.name
                ),
                repair: format!(
                    "set `[crate].evidence` in `{rel_path}` to `\"Estimated\"` or `\"Tested\"`"
                ),
            });
        }
    }

    // Rule 7: `gates` must resolve in `registries/gates.toml`.
    if let Some((items, _)) = gates_opt {
        for (idx, sp) in items.into_iter().enumerate() {
            match sp.value {
                Value::Str(gate) => {
                    if gate.trim().is_empty() {
                        push_schema_err(
                            &pkg.name,
                            rel_path,
                            Some(sp.line),
                            format!("`[crate].gates[{idx}]` must not be empty"),
                            findings,
                        );
                    } else if !registered_gates.contains(&gate) {
                        findings.push(Finding {
                            rule: RULE_CONTRACT_UNKNOWN_REFERENCE,
                            subject: pkg.name.clone(),
                            path: rel_path.to_owned(),
                            line: Some(sp.line),
                            message: format!(
                                "gate `{gate}` cited by `{}` is absent from `registries/gates.toml`",
                                pkg.name
                            ),
                            repair: format!(
                                "register `{gate}` in `registries/gates.toml` or remove it from `[crate].gates` in `{rel_path}`"
                            ),
                        });
                    }
                }
                other => {
                    push_schema_err(
                        &pkg.name,
                        rel_path,
                        Some(sp.line),
                        format!(
                            "`[crate].gates[{idx}]` expected string, found {}",
                            other.type_name()
                        ),
                        findings,
                    );
                }
            }
        }
    }

    // Rule 7: `refusals` must resolve in `registries/refusals.toml`.
    if let Some((items, _)) = refs_opt {
        for (idx, sp) in items.into_iter().enumerate() {
            match sp.value {
                Value::Str(code) => {
                    if code.trim().is_empty() {
                        push_schema_err(
                            &pkg.name,
                            rel_path,
                            Some(sp.line),
                            format!("`[crate].refusals[{idx}]` must not be empty"),
                            findings,
                        );
                    } else if !registered_refusals.contains(&code) {
                        findings.push(Finding {
                            rule: RULE_CONTRACT_UNKNOWN_REFERENCE,
                            subject: pkg.name.clone(),
                            path: rel_path.to_owned(),
                            line: Some(sp.line),
                            message: format!(
                                "refusal code `{code}` cited by `{}` is absent from `registries/refusals.toml`",
                                pkg.name
                            ),
                            repair: format!(
                                "register `{code}` in `registries/refusals.toml` or remove it from `[crate].refusals` in `{rel_path}`"
                            ),
                        });
                    }
                }
                other => {
                    push_schema_err(
                        &pkg.name,
                        rel_path,
                        Some(sp.line),
                        format!(
                            "`[crate].refusals[{idx}]` expected string, found {}",
                            other.type_name()
                        ),
                        findings,
                    );
                }
            }
        }
    }
}

fn crate_allowed_proof_doorway(crate_name: &str, doorways: &[DoorwaySite]) -> bool {
    doorways.iter().any(|site| {
        site.crate_name == crate_name
            && site
                .constructors
                .iter()
                .any(|c| c.contains("certified") || c.contains("proved"))
    })
}

fn check_budgets_section(
    crate_name: &str,
    rel_path: &str,
    tbl: &Table,
    findings: &mut Vec<Finding>,
) {
    for &req in SCHEMA_BUDGETS_REQUIRED_KEYS {
        if !tbl.contains_key(req) {
            push_schema_err(
                crate_name,
                rel_path,
                Some(tbl.span().0),
                format!("missing required key `{req}` in `[budgets]`"),
                findings,
            );
        }
    }
    for (k, sp) in tbl.iter() {
        match sp.value.as_int() {
            Some(v) if v >= 0 => {}
            Some(v) => {
                push_schema_err(
                    crate_name,
                    rel_path,
                    Some(sp.line),
                    format!("`[budgets].{k}` ({v}) must be a non-negative integer"),
                    findings,
                );
            }
            None => {
                push_schema_err(
                    crate_name,
                    rel_path,
                    Some(sp.line),
                    format!(
                        "`[budgets].{k}` expected integer, found {}",
                        sp.value.type_name()
                    ),
                    findings,
                );
            }
        }
    }
}

fn check_safe_mode_section(
    crate_name: &str,
    rel_path: &str,
    tbl: Table,
    registered_refusals: &BTreeSet<String>,
    findings: &mut Vec<Finding>,
) {
    let mut smf = Fields::new(tbl, "[safe_mode]");
    let on_budget = take_required_str(&mut smf, "on_budget", crate_name, rel_path, findings);
    let on_calibration =
        take_required_str(&mut smf, "on_calibration", crate_name, rel_path, findings);
    if let Err(err) = smf.finish() {
        push_schema_err(crate_name, rel_path, Some(err.line), err.to_string(), findings);
    }

    for (key, val_opt) in [("on_budget", on_budget), ("on_calibration", on_calibration)] {
        let Some((val, line)) = val_opt else {
            continue;
        };
        let valid = if val == "reference-path" {
            true
        } else if let Some(code) = val.strip_prefix("refuse:") {
            is_refusal_code(code) && registered_refusals.contains(code)
        } else {
            false
        };
        if !valid {
            findings.push(Finding {
                rule: RULE_CONTRACT_SAFE_MODE,
                subject: crate_name.to_owned(),
                path: rel_path.to_owned(),
                line: Some(line),
                message: format!(
                    "`[safe_mode].{key}` is `{val}`; must be `\"reference-path\"` or `\"refuse:FROB-REF-nnn\"` naming a registered code"
                ),
                repair: format!(
                    "set `[safe_mode].{key}` in `{rel_path}` to `\"reference-path\"` or `\"refuse:<code>\"` from `registries/refusals.toml`"
                ),
            });
        }
    }
}

fn check_features_section(
    crate_name: &str,
    rel_path: &str,
    features_tbl: Option<Table>,
    manifest_features: &BTreeSet<String>,
    registered_gates: &BTreeSet<String>,
    findings: &mut Vec<Finding>,
) {
    let mut contract_features: BTreeSet<String> = BTreeSet::new();

    if let Some(tbl) = features_tbl {
        for (fname, sp) in tbl.into_inner() {
            contract_features.insert(fname.clone());
            let feat_line = sp.line;
            let Value::Table(ft) = sp.value else {
                push_schema_err(
                    crate_name,
                    rel_path,
                    Some(feat_line),
                    format!("`[features].{fname}` must be a table"),
                    findings,
                );
                continue;
            };
            let ctx = format!("[features].{fname}");
            let mut ff = Fields::new(ft, &ctx);
            let tier_opt = take_required_str(&mut ff, "tier", crate_name, rel_path, findings);
            let kill_opt = take_optional_str(&mut ff, "kill", crate_name, rel_path, findings);
            let pilot_opt = take_optional_str(&mut ff, "pilot", crate_name, rel_path, findings);
            for bool_key in ["default", "dev_only"] {
                if let Some(sp) = ff.take(bool_key)
                    && sp.value.as_bool().is_none()
                {
                    push_schema_err(
                        crate_name,
                        rel_path,
                        Some(sp.line),
                        format!(
                            "`[features].{fname}.{bool_key}` expected bool, found {}",
                            sp.value.type_name()
                        ),
                        findings,
                    );
                }
            }
            for list_key in ["traits", "crates", "consumers", "test_consumers"] {
                if let Some(sp) = ff.take(list_key) {
                    match sp.value {
                        Value::Array(items) => {
                            for (idx, item) in items.into_iter().enumerate() {
                                match item.value {
                                    Value::Str(s) if !s.trim().is_empty() => {}
                                    Value::Str(_) => {
                                        push_schema_err(
                                            crate_name,
                                            rel_path,
                                            Some(item.line),
                                            format!(
                                                "`[features].{fname}.{list_key}[{idx}]` must not be empty"
                                            ),
                                            findings,
                                        );
                                    }
                                    other => {
                                        push_schema_err(
                                            crate_name,
                                            rel_path,
                                            Some(item.line),
                                            format!(
                                                "`[features].{fname}.{list_key}[{idx}]` expected string, found {}",
                                                other.type_name()
                                            ),
                                            findings,
                                        );
                                    }
                                }
                            }
                        }
                        other => {
                            push_schema_err(
                                crate_name,
                                rel_path,
                                Some(sp.line),
                                format!(
                                    "`[features].{fname}.{list_key}` expected array, found {}",
                                    other.type_name()
                                ),
                                findings,
                            );
                        }
                    }
                }
            }
            for str_key in ["first_milestone", "gate", "parent_group"] {
                if let Some((val, sline)) =
                    take_optional_str(&mut ff, str_key, crate_name, rel_path, findings)
                    && val.trim().is_empty()
                {
                    push_schema_err(
                        crate_name,
                        rel_path,
                        Some(sline),
                        format!("`[features].{fname}.{str_key}` must not be empty"),
                        findings,
                    );
                }
            }
            if let Err(err) = ff.finish() {
                push_schema_err(crate_name, rel_path, Some(err.line), err.to_string(), findings);
            }

            if let Some((pilot, pline)) = pilot_opt
                && pilot.trim().is_empty()
            {
                push_schema_err(
                    crate_name,
                    rel_path,
                    Some(pline),
                    format!("`[features].{fname}.pilot` must not be empty"),
                    findings,
                );
            }

            let tier = match tier_opt {
                Some((t, tline)) if matches!(t.as_str(), "S" | "F" | "M") => Some((t, tline)),
                Some((t, tline)) => {
                    push_schema_err(
                        crate_name,
                        rel_path,
                        Some(tline),
                        format!("`[features].{fname}.tier` `{t}` must be `S`, `F`, or `M`"),
                        findings,
                    );
                    None
                }
                None => None,
            };

            if let Some((ref t, _)) = tier {
                if t == "F" && !fname.starts_with("frontier-") {
                    findings.push(Finding {
                        rule: RULE_CONTRACT_FEATURES_MISMATCH,
                        subject: crate_name.to_owned(),
                        path: rel_path.to_owned(),
                        line: Some(feat_line),
                        message: format!(
                            "tier-F feature `{fname}` in `{crate_name}` must carry the `frontier-` prefix"
                        ),
                        repair: format!(
                            "rename `{fname}` to `frontier-{fname}` in both `Cargo.toml` and `{rel_path}`"
                        ),
                    });
                }
                if t == "M" && !fname.starts_with("moonshot-") {
                    findings.push(Finding {
                        rule: RULE_CONTRACT_FEATURES_MISMATCH,
                        subject: crate_name.to_owned(),
                        path: rel_path.to_owned(),
                        line: Some(feat_line),
                        message: format!(
                            "tier-M feature `{fname}` in `{crate_name}` must carry the `moonshot-` prefix"
                        ),
                        repair: format!(
                            "rename `{fname}` to `moonshot-{fname}` in both `Cargo.toml` and `{rel_path}`"
                        ),
                    });
                }
                if (t == "F" || t == "M")
                    && kill_opt
                        .as_ref()
                        .is_none_or(|(k, _)| k.trim().is_empty())
                {
                    findings.push(Finding {
                        rule: RULE_CONTRACT_FEATURES_MISMATCH,
                        subject: crate_name.to_owned(),
                        path: rel_path.to_owned(),
                        line: Some(feat_line),
                        message: format!(
                            "tier-{t} feature `{fname}` in `{crate_name}` must declare a non-empty `kill` gate"
                        ),
                        repair: format!(
                            "add `kill = \"GATE-...\"` to `[features].{fname}` in `{rel_path}`"
                        ),
                    });
                }
            }

            if let Some((kill_gate, kline)) = kill_opt {
                if kill_gate.trim().is_empty() {
                    push_schema_err(
                        crate_name,
                        rel_path,
                        Some(kline),
                        format!("`[features].{fname}.kill` must not be empty"),
                        findings,
                    );
                } else if !registered_gates.contains(&kill_gate) {
                    findings.push(Finding {
                        rule: RULE_CONTRACT_UNKNOWN_REFERENCE,
                        subject: crate_name.to_owned(),
                        path: rel_path.to_owned(),
                        line: Some(kline),
                        message: format!(
                            "kill gate `{kill_gate}` for feature `{fname}` in `{crate_name}` is absent from `registries/gates.toml`"
                        ),
                        repair: format!(
                            "register `{kill_gate}` in `registries/gates.toml` or update `[features].{fname}.kill` in `{rel_path}`"
                        ),
                    });
                }
            }
        }
    }

    // Rule 6: manifest `[features]` (excluding `default`, plus `frob-fsim`'s non-group
    // mode/conformance features per N33a-11) must equal contract `[features]`.
    let expected_manifest_features: BTreeSet<String> = manifest_features
        .iter()
        .filter(|feat| {
            !(crate_name == "frob-fsim"
                && FROB_FSIM_NON_GROUP_FEATURES.contains(&feat.as_str()))
        })
        .cloned()
        .collect();
    for feat in expected_manifest_features.difference(&contract_features) {
        findings.push(Finding {
            rule: RULE_CONTRACT_FEATURES_MISMATCH,
            subject: crate_name.to_owned(),
            path: rel_path.to_owned(),
            line: None,
            message: format!(
                "manifest feature `{feat}` in `{crate_name}` is missing from `[features]` in `{rel_path}`"
            ),
            repair: format!(
                "add `{feat} = {{ tier = \"S\" }}` (or `F`/`M` with `kill`) under `[features]` in `{rel_path}`"
            ),
        });
    }
    for feat in contract_features.difference(&expected_manifest_features) {
        findings.push(Finding {
            rule: RULE_CONTRACT_FEATURES_MISMATCH,
            subject: crate_name.to_owned(),
            path: rel_path.to_owned(),
            line: None,
            message: format!(
                "contract feature `{feat}` in `{rel_path}` does not exist in `Cargo.toml` `[features]`"
            ),
            repair: format!(
                "remove `{feat}` from `[features]` in `{rel_path}` or declare it in `Cargo.toml`"
            ),
        });
    }
}

fn check_api_section(
    crate_name: &str,
    rel_path: &str,
    tbl: Table,
    findings: &mut Vec<Finding>,
) -> usize {
    let mut af = Fields::new(tbl, "[api]");
    let float_ok_tbl = take_optional_table(&mut af, "float_ok", crate_name, rel_path, findings);
    if let Err(err) = af.finish() {
        push_schema_err(crate_name, rel_path, Some(err.line), err.to_string(), findings);
    }

    let mut count = 0usize;
    if let Some(fok) = float_ok_tbl {
        for (sig, sp) in fok.iter() {
            count += 1;
            match sp.value.as_str() {
                Some(reason) if reason.trim().is_empty() => {
                    findings.push(Finding {
                        rule: RULE_CONTRACT_FLOAT_OK,
                        subject: crate_name.to_owned(),
                        path: rel_path.to_owned(),
                        line: Some(sp.line),
                        message: format!(
                            "`[api.float_ok].\"{sig}\"` in `{crate_name}` has an empty reason string"
                        ),
                        repair: format!(
                            "supply a non-empty L-6 justification for `{sig}` in `{rel_path}`"
                        ),
                    });
                }
                Some(_) => {}
                None => {
                    push_schema_err(
                        crate_name,
                        rel_path,
                        Some(sp.line),
                        format!(
                            "`[api.float_ok].\"{sig}\"` expected string, found {}",
                            sp.value.type_name()
                        ),
                        findings,
                    );
                }
            }
        }
    }
    count
}

fn push_schema_err(
    crate_name: &str,
    rel_path: &str,
    line: Option<u32>,
    message: String,
    findings: &mut Vec<Finding>,
) {
    findings.push(Finding {
        rule: RULE_CONTRACT_SCHEMA,
        subject: crate_name.to_owned(),
        path: rel_path.to_owned(),
        line,
        message,
        repair: format!(
            "update `{rel_path}` to match `schemas/contract.schema.json` (see `{TEMPLATE_CONTRACT_TOML}`)"
        ),
    });
}

fn take_required_table(
    fields: &mut Fields,
    key: &str,
    crate_name: &str,
    rel_path: &str,
    findings: &mut Vec<Finding>,
) -> Option<Table> {
    match fields.take(key) {
        None => {
            push_schema_err(
                crate_name,
                rel_path,
                None,
                format!("missing required table `[{key}]`"),
                findings,
            );
            None
        }
        Some(sp) => match sp.value {
            Value::Table(t) => Some(t),
            other => {
                push_schema_err(
                    crate_name,
                    rel_path,
                    Some(sp.line),
                    format!("`{key}` expected table, found {}", other.type_name()),
                    findings,
                );
                None
            }
        },
    }
}

fn take_optional_table(
    fields: &mut Fields,
    key: &str,
    crate_name: &str,
    rel_path: &str,
    findings: &mut Vec<Finding>,
) -> Option<Table> {
    let sp = fields.take(key)?;
    match sp.value {
        Value::Table(t) => Some(t),
        other => {
            push_schema_err(
                crate_name,
                rel_path,
                Some(sp.line),
                format!("`{key}` expected table, found {}", other.type_name()),
                findings,
            );
            None
        }
    }
}

fn take_required_str(
    fields: &mut Fields,
    key: &str,
    crate_name: &str,
    rel_path: &str,
    findings: &mut Vec<Finding>,
) -> Option<(String, u32)> {
    match fields.take(key) {
        None => {
            push_schema_err(
                crate_name,
                rel_path,
                None,
                format!("missing required string key `{key}`"),
                findings,
            );
            None
        }
        Some(sp) => match sp.value {
            Value::Str(s) => Some((s, sp.line)),
            other => {
                push_schema_err(
                    crate_name,
                    rel_path,
                    Some(sp.line),
                    format!("`{key}` expected string, found {}", other.type_name()),
                    findings,
                );
                None
            }
        },
    }
}

fn take_optional_str(
    fields: &mut Fields,
    key: &str,
    crate_name: &str,
    rel_path: &str,
    findings: &mut Vec<Finding>,
) -> Option<(String, u32)> {
    let sp = fields.take(key)?;
    match sp.value {
        Value::Str(s) => Some((s, sp.line)),
        other => {
            push_schema_err(
                crate_name,
                rel_path,
                Some(sp.line),
                format!("`{key}` expected string, found {}", other.type_name()),
                findings,
            );
            None
        }
    }
}

fn take_required_int(
    fields: &mut Fields,
    key: &str,
    crate_name: &str,
    rel_path: &str,
    findings: &mut Vec<Finding>,
) -> Option<(i64, u32)> {
    match fields.take(key) {
        None => {
            push_schema_err(
                crate_name,
                rel_path,
                None,
                format!("missing required integer key `{key}`"),
                findings,
            );
            None
        }
        Some(sp) => match sp.value {
            Value::Int(n) => Some((n, sp.line)),
            other => {
                push_schema_err(
                    crate_name,
                    rel_path,
                    Some(sp.line),
                    format!("`{key}` expected integer, found {}", other.type_name()),
                    findings,
                );
                None
            }
        },
    }
}

fn take_required_array(
    fields: &mut Fields,
    key: &str,
    crate_name: &str,
    rel_path: &str,
    findings: &mut Vec<Finding>,
) -> Option<(Vec<Spanned<Value>>, u32)> {
    match fields.take(key) {
        None => {
            push_schema_err(
                crate_name,
                rel_path,
                None,
                format!("missing required array key `{key}`"),
                findings,
            );
            None
        }
        Some(sp) => match sp.value {
            Value::Array(items) => Some((items, sp.line)),
            other => {
                push_schema_err(
                    crate_name,
                    rel_path,
                    Some(sp.line),
                    format!("`{key}` expected array, found {}", other.type_name()),
                    findings,
                );
                None
            }
        },
    }
}

fn is_refusal_code(raw: &str) -> bool {
    let Some(digits) = raw.strip_prefix("FROB-REF-") else {
        return false;
    };
    digits.len() == 3 && digits.bytes().all(|b| b.is_ascii_digit())
}

// ============================================================================
// Lightweight Cargo.toml `[features]` reader (excluding `default`)
// ============================================================================

fn read_manifest_features(manifest_path: &Path) -> Result<BTreeSet<String>, CheckError> {
    let text = fs::read_to_string(manifest_path).map_err(|e| {
        CheckError::Fault(format!("failed reading {}: {e}", manifest_path.display()))
    })?;
    Ok(parse_manifest_features(&text))
}

fn parse_manifest_features(text: &str) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    let mut in_features = false;
    let mut bracket_depth: i32 = 0;

    for raw_line in text.lines() {
        let stripped = strip_toml_line_comment(raw_line);
        let line = stripped.trim();
        if line.is_empty() {
            continue;
        }
        if bracket_depth <= 0 && line.starts_with('[') {
            in_features = line == "[features]";
            continue;
        }
        if !in_features {
            continue;
        }
        if bracket_depth <= 0
            && let Some((lhs, rhs)) = split_unquoted_eq(line)
        {
            let name = unquote_key(lhs.trim());
            if !name.is_empty() && name != "default" {
                out.insert(name);
            }
            bracket_depth += bracket_delta(rhs);
        } else {
            bracket_depth += bracket_delta(line);
        }
    }

    out
}

fn strip_toml_line_comment(line: &str) -> &str {
    let (mut in_double, mut in_single, mut escaped) = (false, false, false);
    for (idx, ch) in line.char_indices() {
        if in_double {
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == '"' {
                in_double = false;
            }
        } else if in_single {
            if ch == '\'' {
                in_single = false;
            }
        } else if ch == '"' {
            in_double = true;
        } else if ch == '\'' {
            in_single = true;
        } else if ch == '#' {
            return &line[..idx];
        }
    }
    line
}

fn split_unquoted_eq(line: &str) -> Option<(&str, &str)> {
    let (mut in_double, mut in_single, mut escaped) = (false, false, false);
    for (idx, ch) in line.char_indices() {
        if in_double {
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == '"' {
                in_double = false;
            }
        } else if in_single {
            if ch == '\'' {
                in_single = false;
            }
        } else if ch == '"' {
            in_double = true;
        } else if ch == '\'' {
            in_single = true;
        } else if ch == '=' {
            return Some((&line[..idx], &line[idx + 1..]));
        }
    }
    None
}

fn bracket_delta(s: &str) -> i32 {
    let (mut in_double, mut in_single, mut escaped) = (false, false, false);
    let mut delta = 0i32;
    for ch in s.chars() {
        if in_double {
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == '"' {
                in_double = false;
            }
        } else if in_single {
            if ch == '\'' {
                in_single = false;
            }
        } else {
            match ch {
                '"' => in_double = true,
                '\'' => in_single = true,
                '[' => delta += 1,
                ']' => delta -= 1,
                _ => {}
            }
        }
    }
    delta
}

fn unquote_key(raw: &str) -> String {
    if let Some(s) = raw.strip_prefix('"').and_then(|t| t.strip_suffix('"')) {
        s.to_owned()
    } else if let Some(s) = raw.strip_prefix('\'').and_then(|t| t.strip_suffix('\'')) {
        s.to_owned()
    } else {
        raw.to_owned()
    }
}

// ============================================================================
// Rule 11: Exact LF line counter (rejects CR/CRLF as tree fault, skips target/)
// ============================================================================

fn count_crate_rs_lines(canon_repo: &Path, crate_dir: &Path) -> Result<(u64, u64), CheckError> {
    let mut rs_files = Vec::new();
    collect_files_by_ext(crate_dir, &["rs"], &mut rs_files)?;
    rs_files.sort();

    let mut total_lines: u64 = 0;
    let mut generated_lines: u64 = 0;

    for path in rs_files {
        let bytes = fs::read(&path)
            .map_err(|e| CheckError::Fault(format!("failed reading {}: {e}", path.display())))?;
        if bytes.contains(&b'\r') {
            let rel = rel_display(canon_repo, &path);
            return Err(CheckError::Fault(format!(
                "tree fault: non-LF line ending (CR/CRLF) detected in `{rel}`; line counts require LF (`\\n`) endings"
            )));
        }
        let n = count_lf_lines(&bytes);
        total_lines = total_lines.saturating_add(n);
        if first_line_is_generated(&bytes) {
            generated_lines = generated_lines.saturating_add(n);
        }
    }

    Ok((total_lines, generated_lines))
}

fn count_non_rust_caps(
    canon_repo: &Path,
    repo_root: &Path,
) -> Result<Vec<NonRustLineFact>, CheckError> {
    let mut out = Vec::with_capacity(NON_RUST_CAPS.len());
    for cap in NON_RUST_CAPS {
        let dir = repo_root.join(cap.dir);
        let exts: &[&str] = match cap.language {
            "dialect" => &["frobk", "kir", "kern", "k", "wgsl", "cu", "metal"],
            "javascript" => &["js", "mjs", "cjs", "jsx", "ts", "tsx"],
            "python" => &["py"],
            "lean" => &["lean"],
            _ => &[],
        };
        let mut lines: u64 = 0;
        if dir.is_dir() && !exts.is_empty() {
            let mut files = Vec::new();
            collect_files_by_ext(&dir, exts, &mut files)?;
            files.sort();
            for path in files {
                let bytes = fs::read(&path).map_err(|e| {
                    CheckError::Fault(format!("failed reading {}: {e}", path.display()))
                })?;
                if bytes.contains(&b'\r') {
                    let rel = rel_display(canon_repo, &path);
                    return Err(CheckError::Fault(format!(
                        "tree fault: non-LF line ending (CR/CRLF) detected in `{rel}`"
                    )));
                }
                lines = lines.saturating_add(count_lf_lines(&bytes));
            }
        }
        out.push(NonRustLineFact {
            name: cap.name.to_owned(),
            dir: cap.dir.to_owned(),
            language: cap.language.to_owned(),
            lines,
            cap_kloc: cap.cap_kloc,
            cap_lines: cap.cap_kloc.map(|k| u64::from(k) * 1000),
        });
    }
    Ok(out)
}

fn collect_files_by_ext(
    dir: &Path,
    exts: &[&str],
    out: &mut Vec<PathBuf>,
) -> Result<(), CheckError> {
    let entries = fs::read_dir(dir)
        .map_err(|e| CheckError::Fault(format!("failed reading directory {}: {e}", dir.display())))?;
    let mut paths = Vec::new();
    for entry_res in entries {
        let entry = entry_res.map_err(|e| {
            CheckError::Fault(format!("failed reading dir entry in {}: {e}", dir.display()))
        })?;
        paths.push(entry.path());
    }
    paths.sort();

    for path in paths {
        let Some(name) = path.file_name().and_then(|s| s.to_str()) else {
            continue;
        };
        if name.starts_with('.') || name == "target" || name == "node_modules" || name == "__pycache__"
        {
            continue;
        }
        if path.is_dir() {
            collect_files_by_ext(&path, exts, out)?;
        } else if path.is_file()
            && let Some(ext) = path.extension().and_then(|s| s.to_str())
            && exts.contains(&ext)
        {
            out.push(path);
        }
    }
    Ok(())
}

fn count_lf_lines(bytes: &[u8]) -> u64 {
    if bytes.is_empty() {
        return 0;
    }
    let newlines = bytes.iter().filter(|&&b| b == b'\n').count() as u64;
    if bytes.ends_with(b"\n") {
        newlines
    } else {
        newlines + 1
    }
}

fn first_line_is_generated(bytes: &[u8]) -> bool {
    let end = bytes.iter().position(|&b| b == b'\n').unwrap_or(bytes.len());
    std::str::from_utf8(&bytes[..end]).is_ok_and(|line| line.contains("@generated"))
}

fn rel_display(canon_repo: &Path, path: &Path) -> String {
    let canon = canonical_or_clean(path);
    if let Ok(rel) = canon.strip_prefix(canon_repo) {
        normalize_rel_path(rel)
    } else {
        normalize_rel_path(path)
    }
}
