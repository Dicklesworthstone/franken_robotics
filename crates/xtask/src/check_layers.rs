//! `xtask check-layers`: crate-map validation, L-1, L-3, L-10, and L-11 (bead N34p4-01).
//!
//! Enforces five ordered rule groups over declared `cargo metadata --no-deps`
//! edges, manifests, Rust source files, and `contract.toml` files (read through
//! `frob_core::registry::toml`):
//! 1. Crate-map validation (`ADR-WS-01.unlisted-crate`, `map.metadata-mismatch`)
//! 2. L-1 layer and intra-layer rank order (`L-1.upward`, `L-1.rank`)
//! 3. L-3 annex isolation (`L-3.core-to-annex`, `L-3.annex-to-annex`)
//! 4. L-10 inversion register (`L-10.unregistered-inversion`, `L-10.unlisted-implementor`,
//!    `L-10.wrong-home`, `L-10.inversion-without-adr`)
//! 5. L-11 two-planes boundary (`L-11.controller-in-compute-plane`)
//!
//! Determinism class D0: findings are sorted by `(rule, subject, path)` and
//! deduplicated. Compiles nothing and touches no network.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use frob_core::registry::toml::{self, Value};

use crate::cli::{CheckContext, CheckError, CheckOutcome};
use crate::crate_map::{
    CRATE_MAP, CrateRow, Finding, Layer, TOMBSTONES, Tombstone, WorkspaceMember, lookup_in,
    validate_with_table,
};
use crate::inversions::{INVERSIONS, InversionRow, check_inversions};
use crate::metadata::{
    DepKind, PackageMeta, WorkspaceKind, WorkspaceRoot, load_declared_metadata,
};
use crate::report::FactValue;

pub use crate::crate_map::{RULE_METADATA_MISMATCH, RULE_UNLISTED_CRATE};
pub use crate::inversions::{
    RULE_INVERSION_WITHOUT_ADR, RULE_UNLISTED_IMPLEMENTOR, RULE_UNREGISTERED_INVERSION,
    RULE_WRONG_HOME,
};

/// Rule identifier for a dependency from a lower core layer to a higher core layer.
pub const RULE_L1_UPWARD: &str = "L-1.upward";
/// Rule identifier for an intra-layer dependency whose target rank is >= the source rank.
pub const RULE_L1_RANK: &str = "L-1.rank";
/// Rule identifier for an edge from a core crate to an annex crate or annex/apps path.
pub const RULE_L3_CORE_TO_ANNEX: &str = "L-3.core-to-annex";
/// Rule identifier for an edge between two annex crates outside `apps/`.
pub const RULE_L3_ANNEX_TO_ANNEX: &str = "L-3.annex-to-annex";
/// Rule identifier for a decision-plane declaration or controller record in layers 1..=3.
pub const RULE_L11_CONTROLLER_IN_COMPUTE_PLANE: &str = "L-11.controller-in-compute-plane";

/// Complete list of finding rule identifiers owned exclusively by `check-layers`.
pub const OWNED_RULES: &[&str] = &[
    RULE_UNLISTED_CRATE,
    RULE_METADATA_MISMATCH,
    RULE_L1_UPWARD,
    RULE_L1_RANK,
    RULE_L3_CORE_TO_ANNEX,
    RULE_L3_ANNEX_TO_ANNEX,
    RULE_UNREGISTERED_INVERSION,
    RULE_UNLISTED_IMPLEMENTOR,
    RULE_WRONG_HOME,
    RULE_INVERSION_WITHOUT_ADR,
    RULE_L11_CONTROLLER_IN_COMPUTE_PLANE,
];

/// One declared dependency edge evaluated by [`check_edges`] (also exercised by INV-WS-01 property tests).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphEdge {
    pub from: String,
    pub from_rel_dir: String,
    pub from_rel_manifest: String,
    pub to: String,
    /// Repository-relative target path when `to` is a path dependency inside this repository.
    pub to_rel_path: Option<String>,
    pub kind: DepKind,
}

/// CLI handler plugged into the static `COMMANDS` array in `src/main.rs`.
pub fn run_command(ctx: &CheckContext) -> Result<CheckOutcome, CheckError> {
    if !ctx.extra_args.is_empty() {
        return Err(CheckError::Usage(format!(
            "check-layers accepts no extra arguments: {}",
            ctx.extra_args.join(" ")
        )));
    }
    check_layers(&ctx.repo_root, &ctx.roots)
}

/// Run `check-layers` over `roots` using the compiled `CRATE_MAP`, `TOMBSTONES`, and `INVERSIONS`.
pub fn check_layers(repo_root: &Path, roots: &[WorkspaceRoot]) -> Result<CheckOutcome, CheckError> {
    check_layers_with_tables(repo_root, roots, CRATE_MAP, TOMBSTONES, INVERSIONS)
}

/// Run `check-layers` over `roots` with explicit tables (used by unit tests and `synthetic` fixtures).
pub fn check_layers_with_tables(
    repo_root: &Path,
    roots: &[WorkspaceRoot],
    crate_map: &[CrateRow],
    tombstones: &[Tombstone],
    inversions: &[InversionRow],
) -> Result<CheckOutcome, CheckError> {
    let mut findings = Vec::new();
    let mut checked_workspaces = Vec::new();
    let mut absent_workspaces = Vec::new();
    let mut all_members: Vec<PackageMeta> = Vec::new();
    let mut all_edges: Vec<GraphEdge> = Vec::new();

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

        // Step 1: Crate-map validation (ADR-WS-01.unlisted-crate, map.metadata-mismatch).
        let ws_members: Vec<WorkspaceMember> = declared
            .members
            .iter()
            .map(PackageMeta::to_workspace_member)
            .collect();
        let link_refs: Vec<&str> = ws.link_packages.iter().map(String::as_str).collect();
        findings.extend(validate_with_table(
            crate_map,
            tombstones,
            &ws_members,
            &link_refs,
        ));

        for pkg in declared.members {
            for dep in &pkg.dependencies {
                all_edges.push(GraphEdge {
                    from: pkg.name.clone(),
                    from_rel_dir: pkg.rel_dir.clone(),
                    from_rel_manifest: pkg.rel_manifest.clone(),
                    to: dep.name.clone(),
                    to_rel_path: dep.rel_path.clone(),
                    kind: dep.kind,
                });
            }
            all_members.push(pkg);
        }
    }

    // Steps 2 & 3: L-1 (layer/rank order) and L-3 (annex isolation).
    findings.extend(check_edges(&all_edges, crate_map));

    // Step 4: L-10 (inversion register & source markers/impls/trait homes).
    findings.extend(check_inversions(repo_root, &all_members, inversions));

    // Step 5: L-11 (two planes: no decision plane or [[controller]] in layers 1..=3).
    findings.extend(check_compute_plane_contracts(&all_members, crate_map));

    findings.sort();
    findings.dedup();
    checked_workspaces.sort();
    absent_workspaces.sort();

    let mut facts = BTreeMap::new();
    facts.insert("crates_checked".to_owned(), FactValue::int_usize(all_members.len()));
    facts.insert("edges_checked".to_owned(), FactValue::int_usize(all_edges.len()));
    facts.insert(
        "inversions_registered".to_owned(),
        FactValue::int_usize(inversions.len()),
    );
    facts.insert(
        "workspaces_absent".to_owned(),
        FactValue::str_array(absent_workspaces),
    );
    facts.insert(
        "workspaces_checked".to_owned(),
        FactValue::str_array(checked_workspaces),
    );

    Ok(CheckOutcome { findings, facts })
}

/// Evaluate L-1 (`L-1.upward`, `L-1.rank`) and L-3 (`L-3.core-to-annex`, `L-3.annex-to-annex`)
/// over declared edges against `crate_map`.
#[must_use]
pub fn check_edges(edges: &[GraphEdge], crate_map: &[CrateRow]) -> Vec<Finding> {
    let mut findings = Vec::new();

    for edge in edges {
        let from_row = lookup_in(crate_map, &edge.from);
        let to_row = lookup_in(crate_map, &edge.to);

        let from_is_apps = is_under_dir(&edge.from_rel_dir, "apps");
        let from_is_annex = !from_is_apps
            && (is_under_annex_dir(&edge.from_rel_dir)
                || from_row.is_some_and(|r| r.layer == Layer::Annex));
        let from_is_core = !from_is_apps && !from_is_annex;

        let to_under_annex_dir = edge.to_rel_path.as_deref().is_some_and(is_under_annex_dir);
        let to_under_apps_dir = edge
            .to_rel_path
            .as_deref()
            .is_some_and(|p| is_under_dir(p, "apps"));
        let to_is_annex =
            to_under_annex_dir || to_row.is_some_and(|r| r.layer == Layer::Annex);

        // Rule L-3: core never depends on annex or apps/; annexes never depend on each other outside apps/.
        if from_is_core && (to_is_annex || to_under_apps_dir) {
            let target_desc = edge
                .to_rel_path
                .as_deref()
                .map_or_else(|| edge.to.clone(), |p| format!("{} ({p})", edge.to));
            findings.push(Finding {
                rule: RULE_L3_CORE_TO_ANNEX,
                subject: format!("{} -> {}", edge.from, edge.to),
                path: edge.from_rel_manifest.clone(),
                line: None,
                message: format!(
                    "core crate `{}` has a {} dependency on annex/app target `{target_desc}` (rule L-3)",
                    edge.from, edge.kind
                ),
                repair: format!(
                    "remove the dependency on `{}` from `{}` and invert through a core trait",
                    edge.to, edge.from
                ),
            });
            continue;
        }

        if from_is_annex && to_is_annex && edge.from != edge.to {
            findings.push(Finding {
                rule: RULE_L3_ANNEX_TO_ANNEX,
                subject: format!("{} -> {}", edge.from, edge.to),
                path: edge.from_rel_manifest.clone(),
                line: None,
                message: format!(
                    "annex crate `{}` has a {} dependency on annex crate `{}` outside apps/ (rule L-3)",
                    edge.from, edge.kind, edge.to
                ),
                repair: format!(
                    "remove the annex-to-annex edge `{} -> {}` and link both only in apps/",
                    edge.from, edge.to
                ),
            });
            continue;
        }

        // Annex -> core and apps/ -> core|annex are permitted by L-3 and carry no L-1 violation.
        if from_is_annex || from_is_apps {
            continue;
        }

        // Rule L-1: every core-to-core edge (normal, build, dev) must point strictly downward.
        let (Some(a_row), Some(b_row)) = (from_row, to_row) else {
            continue;
        };
        let (Layer::Core(la), Layer::Core(lb)) = (a_row.layer, b_row.layer) else {
            continue;
        };

        if lb > la {
            findings.push(Finding {
                rule: RULE_L1_UPWARD,
                subject: format!("{} -> {}", edge.from, edge.to),
                path: edge.from_rel_manifest.clone(),
                line: None,
                message: format!(
                    "`{}` (layer {la}, rank {}) has a {} dependency on higher-layer crate `{}` (layer {lb}, rank {}) (rule L-1)",
                    edge.from, a_row.rank, edge.kind, edge.to, b_row.rank
                ),
                repair: format!(
                    "remove the upward {} dependency on `{}` from `{}` or invert through a registered L-10 trait",
                    edge.kind, edge.to, edge.from
                ),
            });
        } else if lb == la && b_row.rank >= a_row.rank {
            findings.push(Finding {
                rule: RULE_L1_RANK,
                subject: format!("{} -> {}", edge.from, edge.to),
                path: edge.from_rel_manifest.clone(),
                line: None,
                message: format!(
                    "`{}` (layer {la}, rank {}) has a {} dependency on same-layer crate `{}` with rank {} >= {} (rule L-1, CX-363)",
                    edge.from, a_row.rank, edge.kind, edge.to, b_row.rank, a_row.rank
                ),
                repair: format!(
                    "remove the against-rank {} dependency `{} -> {}` or reorder ranks in crate_map.rs by ADR",
                    edge.kind, edge.from, edge.to
                ),
            });
        }
    }

    findings.sort();
    findings.dedup();
    findings
}

/// Enforce rule L-11: no crate of core layers 1..=3 may declare `plane = "decision"`
/// or carry a `[[controller]]` record in its `contract.toml`.
#[must_use]
pub fn check_compute_plane_contracts(
    members: &[PackageMeta],
    crate_map: &[CrateRow],
) -> Vec<Finding> {
    let mut findings = Vec::new();

    for pkg in members {
        let layer = lookup_in(crate_map, &pkg.name)
            .map(|r| r.layer)
            .or_else(|| {
                pkg.frob_metadata
                    .as_ref()
                    .and_then(|res| res.as_ref().ok())
                    .map(|m| m.layer)
            });

        let Some(Layer::Core(layer_num @ 1..=3)) = layer else {
            continue;
        };

        let contract_path = pkg.crate_dir().join("contract.toml");
        if !contract_path.is_file() {
            continue;
        }
        let Ok(bytes) = fs::read(&contract_path) else {
            continue;
        };
        // Read exclusively via frob-core's strict TOML-subset reader (N34p2-01).
        // Malformed contract.toml belongs to `check-contracts` (`contract.schema`), not here.
        let Ok(table) = toml::parse(&bytes) else {
            continue;
        };

        let rel_contract = if pkg.rel_dir.is_empty() {
            "contract.toml".to_owned()
        } else {
            format!("{}/contract.toml", pkg.rel_dir)
        };

        let plane_sp = table
            .get("crate")
            .and_then(Value::as_table)
            .and_then(|c| c.get_spanned("plane"))
            .or_else(|| table.get_spanned("plane"));

        if let Some(sp) = plane_sp
            && sp.value.as_str() == Some("decision")
        {
            findings.push(Finding {
                rule: RULE_L11_CONTROLLER_IN_COMPUTE_PLANE,
                subject: pkg.name.clone(),
                path: rel_contract.clone(),
                line: Some(sp.line),
                message: format!(
                    "compute-plane crate `{}` (layer {layer_num}) declares `plane = \"decision\"` in contract.toml (rule L-11)",
                    pkg.name
                ),
                repair: "set `plane = \"compute\"` in contract.toml and move decision logic to layer 4 or above"
                    .to_owned(),
            });
        }

        if let Some(ctrl_sp) = table.get_spanned("controller")
            && let Some(items) = ctrl_sp.value.as_array()
        {
            for item in items {
                let ctrl_id = item
                    .value
                    .as_table()
                    .and_then(|t| t.get("id"))
                    .and_then(Value::as_str);
                let subject = match ctrl_id {
                    Some(id) if !id.is_empty() => format!("{}:{id}", pkg.name),
                    _ => pkg.name.clone(),
                };
                findings.push(Finding {
                    rule: RULE_L11_CONTROLLER_IN_COMPUTE_PLANE,
                    subject,
                    path: rel_contract.clone(),
                    line: Some(item.line),
                    message: format!(
                        "compute-plane crate `{}` (layer {layer_num}) declares a `[[controller]]` record in contract.toml (rule L-11)",
                        pkg.name
                    ),
                    repair: "move adaptive controllers, schedulers, and policies to layer 4 or above"
                        .to_owned(),
                });
            }
        }
    }

    findings.sort();
    findings.dedup();
    findings
}

fn is_under_dir(rel: &str, prefix: &str) -> bool {
    rel == prefix || rel.starts_with(&format!("{prefix}/"))
}

fn is_under_annex_dir(rel: &str) -> bool {
    is_under_dir(rel, "spark") || is_under_dir(rel, "derive") || is_under_dir(rel, "bindings")
}
