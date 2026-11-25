//! `xtask check-deps`: rules L-2, L-4, L-5, L-9, and proof-boundary edges (bead BD-003).
//!
//! Enforces seven ordered steps over declared (`--no-deps`) and resolved `cargo metadata`
//! reads plus `franken-stack.lock` (`stack::lock::load`, N33a-03) and posture-to-graph
//! rules (`stack::posture::edge_allowed`, N33a-01):
//! 1. L-2 (`fsim.volatile-edge`, `L-2.fs-qty-direct`, `L-2.airlock-reexport`)
//! 2. L-4 (`L-4.foundation-confined`, `L-4.edge-spec`) via `stack::posture::edge_allowed`
//! 3. L-5 (`L-5.third-party`, `L-5.serde-outside-boundary`, `L-5.serde-not-exact`)
//! 4. L-5 (`L-5.forbidden`, `L-5.python-outside-bindings`, core `links`/`cc`/`cmake`/`bindgen`)
//! 5. L-9 (`L-9.cert-has-deps`, `L-9.cert-dependent`)
//! 6. `edge.forbidden` proof-boundary table (only when layers permit the edge)
//! 7. Closure `facts` (total/registry package counts and transitive forbidden hits with one path)
//!
//! Determinism class D0: findings and facts are sorted deterministically. Compiles nothing
//! and touches no network.

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fs;
use std::path::{Path, PathBuf};

use crate::check_layers::{GraphEdge, check_edges};
use crate::cli::{CheckContext, CheckError, CheckOutcome};
use crate::crate_map::{CRATE_MAP, CrateRow, Finding, Layer, lookup_in};
use crate::metadata::{
    DeclaredDep, DepKind, PackageMeta, ResolvedPackage, ResolvedWorkspace, WorkspaceKind,
    WorkspaceRoot, load_declared_metadata, load_resolved_metadata, normalize_rel_path,
};
use crate::report::FactValue;
use crate::stack::lock::{self, Foundation, StackLock};
use crate::stack::posture::{
    BEDROCK_DIRECT_CRATES, DepSpec, EdgeDecision, EdgeDenial, EdgeKind, check_edge, edge_allowed,
    is_exact_semver,
};

/// Rule identifier for a FrankenSim volatile dependency outside `frob-fsim` (INV-SUITE-01).
pub const RULE_FSIM_VOLATILE_EDGE: &str = "fsim.volatile-edge";
/// Rule identifier for a direct `fs-qty` dependency outside `frob-core` and `frob-fsim`.
pub const RULE_L2_FS_QTY_DIRECT: &str = "L-2.fs-qty-direct";
/// Rule identifier for `frob-fsim` re-exporting the root of a volatile FrankenSim crate.
pub const RULE_L2_AIRLOCK_REEXPORT: &str = "L-2.airlock-reexport";
/// Rule identifier for a non-repo edge outside foundation confinement or missing a foundation row.
pub const RULE_L4_FOUNDATION_CONFINED: &str = "L-4.foundation-confined";
/// Rule identifier for an admitted foundation crate with an invalid manifest dependency specification.
pub const RULE_L4_EDGE_SPEC: &str = "L-4.edge-spec";
/// Rule identifier for an unapproved registry or git direct dependency of a core crate.
pub const RULE_L5_THIRD_PARTY: &str = "L-5.third-party";
/// Rule identifier for `serde` or `serde_json` outside the six boundary crates.
pub const RULE_L5_SERDE_OUTSIDE_BOUNDARY: &str = "L-5.serde-outside-boundary";
/// Rule identifier for `serde` or `serde_json` in a boundary crate without an exact `=x.y.z` pin.
pub const RULE_L5_SERDE_NOT_EXACT: &str = "L-5.serde-not-exact";
/// Rule identifier for a forbidden direct dependency (tokio, rayon, BLAS/LAPACK, core native C/C++).
pub const RULE_L5_FORBIDDEN: &str = "L-5.forbidden";
/// Rule identifier for a Python interpreter or numpy binding outside `bindings/frob-gymnasium`.
pub const RULE_L5_PYTHON_OUTSIDE_BINDINGS: &str = "L-5.python-outside-bindings";
/// Rule identifier for any dependency entry in `crates/frob-cert/Cargo.toml`.
pub const RULE_L9_CERT_HAS_DEPS: &str = "L-9.cert-has-deps";
/// Rule identifier for a crate outside the three allowed dependents depending on `frob-cert`.
pub const RULE_L9_CERT_DEPENDENT: &str = "L-9.cert-dependent";
/// Rule identifier for a proof-boundary forbidden edge that layers alone would admit.
pub const RULE_EDGE_FORBIDDEN: &str = "edge.forbidden";

/// Complete list of finding rule identifiers owned by `check-deps` (BD-003 core rules).
pub const OWNED_RULES: &[&str] = &[
    RULE_FSIM_VOLATILE_EDGE,
    RULE_L2_FS_QTY_DIRECT,
    RULE_L2_AIRLOCK_REEXPORT,
    RULE_L4_FOUNDATION_CONFINED,
    RULE_L4_EDGE_SPEC,
    RULE_L5_THIRD_PARTY,
    RULE_L5_SERDE_OUTSIDE_BOUNDARY,
    RULE_L5_SERDE_NOT_EXACT,
    RULE_L5_FORBIDDEN,
    RULE_L5_PYTHON_OUTSIDE_BINDINGS,
    RULE_L9_CERT_HAS_DEPS,
    RULE_L9_CERT_DEPENDENT,
    RULE_EDGE_FORBIDDEN,
];

/// The six boundary crates permitted exact-pinned `serde` / `serde_json` direct dependencies (L-5).
pub const SERDE_BOUNDARY_CRATES: &[&str] = &[
    "frob-io",
    "frob-verbs",
    "frob-chronicle",
    "frob-design",
    "frob-spec",
    "frob-mcp",
];

/// The three crates permitted to depend on the independent auditor `frob-cert` (L-9, INV-WS-02).
pub const CERT_ALLOWED_DEPENDENTS: &[&str] = &[
    "frob-verbs",
    "frob-cli",
    "frob-conformance",
];

/// Build-dependency crate names indicating C/C++ native compilation forbidden in core direct deps.
pub const FORBIDDEN_NATIVE_BUILD_DEPS: &[&str] = &["cc", "cmake", "bindgen"];

/// One entry in the direct-edge forbidden name table (extended by N00-23 with `nongoal = "NONGOAL-04"`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ForbiddenNameRow {
    /// Exact crate name or prefix pattern ending in `*` (e.g. `"tokio-*"`, `"blas*"`).
    pub pattern: &'static str,
    /// Stable rule or non-goal justification for this ban.
    pub reason: &'static str,
}

/// Authoritative direct-edge forbidden package name table (rule L-5).
pub const FORBIDDEN_DIRECT_NAMES: &[ForbiddenNameRow] = &[
    // Tokio family and async stacks built on it.
    ForbiddenNameRow { pattern: "tokio", reason: "L-5:tokio-family" },
    ForbiddenNameRow { pattern: "tokio-*", reason: "L-5:tokio-family" },
    ForbiddenNameRow { pattern: "hyper", reason: "L-5:tokio-family" },
    ForbiddenNameRow { pattern: "hyper-*", reason: "L-5:tokio-family" },
    ForbiddenNameRow { pattern: "tonic", reason: "L-5:tokio-family" },
    ForbiddenNameRow { pattern: "axum", reason: "L-5:tokio-family" },
    ForbiddenNameRow { pattern: "tower", reason: "L-5:tokio-family" },
    ForbiddenNameRow { pattern: "tower-*", reason: "L-5:tokio-family" },
    ForbiddenNameRow { pattern: "reqwest", reason: "L-5:tokio-family" },
    // Direct Rayon pools.
    ForbiddenNameRow { pattern: "rayon", reason: "L-5:direct-rayon" },
    ForbiddenNameRow { pattern: "rayon-core", reason: "L-5:direct-rayon" },
    // BLAS and LAPACK bindings.
    ForbiddenNameRow { pattern: "blas*", reason: "L-5:blas-lapack" },
    ForbiddenNameRow { pattern: "lapack*", reason: "L-5:blas-lapack" },
    ForbiddenNameRow { pattern: "openblas-src", reason: "L-5:blas-lapack" },
    ForbiddenNameRow { pattern: "intel-mkl-src", reason: "L-5:blas-lapack" },
    ForbiddenNameRow { pattern: "netlib-src", reason: "L-5:blas-lapack" },
    ForbiddenNameRow { pattern: "accelerate-src", reason: "L-5:blas-lapack" },
    ForbiddenNameRow { pattern: "ndarray-linalg", reason: "L-5:blas-lapack" },
    ForbiddenNameRow { pattern: "nalgebra-lapack", reason: "L-5:blas-lapack" },
];

/// Python interpreter and numpy binding patterns permitted only in `bindings/frob-gymnasium`.
pub const PYTHON_FORBIDDEN_PATTERNS: &[&str] = &[
    "pyo3",
    "pyo3-*",
    "cpython",
    "rustpython*",
    "numpy",
];

/// One row of the proof-boundary forbidden-edge table (step 6).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ForbiddenEdgeRow {
    pub from: &'static str,
    pub to: &'static str,
    pub reason: &'static str,
}

/// Authoritative proof-boundary forbidden-edge table (step 6).
pub const PROOF_BOUNDARY_EDGES: &[ForbiddenEdgeRow] = &[ForbiddenEdgeRow {
    from: "frob-arena",
    to: "frob-nemesis",
    reason: "qualification code (`frob-arena`) must not link the searcher (`frob-nemesis`)",
}];

/// CLI handler plugged into the static `COMMANDS` array in `src/main.rs`.
pub fn run_command(ctx: &CheckContext) -> Result<CheckOutcome, CheckError> {
    if !ctx.extra_args.is_empty() {
        return Err(CheckError::Usage(format!(
            "check-deps accepts no extra arguments: {}",
            ctx.extra_args.join(" ")
        )));
    }
    check_deps(&ctx.repo_root, &ctx.roots)
}

/// Run `check-deps` over `roots` reading `<repo_root>/franken-stack.lock` via `stack::lock::load`.
pub fn check_deps(repo_root: &Path, roots: &[WorkspaceRoot]) -> Result<CheckOutcome, CheckError> {
    let lock_path = repo_root.join("franken-stack.lock");
    if !lock_path.is_file() {
        return Err(CheckError::Usage(format!(
            "stack lock not found at {}",
            lock_path.display()
        )));
    }
    let stack_lock = lock::load(&lock_path)
        .map_err(|err| CheckError::Fault(format!("failed to load {}: {err}", lock_path.display())))?;
    check_deps_with_config(
        repo_root,
        roots,
        &stack_lock,
        CRATE_MAP,
        PROOF_BOUNDARY_EDGES,
    )
}

/// Run `check-deps` with explicit `StackLock`, `CrateRow` map, and proof-boundary table
/// (used directly by unit tests and `synthetic` fixtures).
pub fn check_deps_with_config(
    repo_root: &Path,
    roots: &[WorkspaceRoot],
    stack_lock: &StackLock,
    crate_map: &[CrateRow],
    proof_boundary_edges: &[ForbiddenEdgeRow],
) -> Result<CheckOutcome, CheckError> {
    let mut findings = Vec::new();
    let mut checked_workspaces = Vec::new();
    let mut absent_workspaces = Vec::new();
    let mut ws_facts_map: BTreeMap<String, FactValue> = BTreeMap::new();

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

        let declared = load_declared_metadata(repo_root, ws).map_err(metadata_err_to_check_err)?;
        let resolved = load_resolved_metadata(repo_root, ws).map_err(metadata_err_to_check_err)?;
        checked_workspaces.push(ws.name.clone());

        evaluate_workspace(
            &declared.members,
            &resolved,
            ws.kind,
            stack_lock,
            crate_map,
            proof_boundary_edges,
            &mut findings,
        );

        ws_facts_map.insert(ws.name.clone(), build_workspace_closure_facts(&resolved));
    }

    findings.sort();
    findings.dedup();
    checked_workspaces.sort();
    absent_workspaces.sort();

    let mut facts = BTreeMap::new();
    facts.insert(
        "direct_only_policy".to_owned(),
        FactValue::str(
            "Franken-only applies to direct edges; resolved closure counts and transitive forbidden hits are reported under `workspaces` as facts.",
        ),
    );
    facts.insert(
        "workspaces".to_owned(),
        FactValue::Object(ws_facts_map),
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

fn metadata_err_to_check_err(err: crate::metadata::MetadataLoadError) -> CheckError {
    if err.is_usage_error() {
        CheckError::Usage(err.to_string())
    } else {
        CheckError::Fault(err.to_string())
    }
}

fn evaluate_workspace(
    members: &[PackageMeta],
    resolved: &ResolvedWorkspace,
    ws_kind: WorkspaceKind,
    stack_lock: &StackLock,
    crate_map: &[CrateRow],
    proof_boundary_edges: &[ForbiddenEdgeRow],
    findings: &mut Vec<Finding>,
) {
    let fsim_row = find_fsim_foundation(stack_lock);

    for pkg in members {
        // Step 1c: L-2.airlock-reexport source scan on `frob-fsim`.
        if pkg.name == "frob-fsim" {
            scan_airlock_reexports(pkg, findings);
        }

        // Step 5a: L-9.cert-has-deps on `frob-cert` (any normal/dev/build entry).
        if pkg.name == "frob-cert" {
            for dep in &pkg.dependencies {
                findings.push(Finding {
                    rule: RULE_L9_CERT_HAS_DEPS,
                    subject: format!("frob-cert -> {}", dep.name),
                    path: pkg.rel_manifest.clone(),
                    line: None,
                    message: format!(
                        "`frob-cert` has a {} dependency on `{}` (rule L-9, INV-WS-02: auditor dependency tables must be empty)",
                        dep.kind, dep.name
                    ),
                    repair: format!(
                        "remove `{}` from crates/frob-cert/Cargo.toml so frob-cert stays std-only",
                        dep.name
                    ),
                });
            }
            continue;
        }

        let from_is_core = is_core_package(pkg, ws_kind, crate_map);
        let from_id = confinement_subject_for(pkg);

        for dep in &pkg.dependencies {
            // Step 5b: L-9.cert-dependent (only frob-verbs, frob-cli, frob-conformance may depend on frob-cert).
            if dep.name == "frob-cert" {
                if !CERT_ALLOWED_DEPENDENTS.contains(&pkg.name.as_str()) {
                    findings.push(Finding {
                        rule: RULE_L9_CERT_DEPENDENT,
                        subject: format!("{} -> frob-cert", pkg.name),
                        path: pkg.rel_manifest.clone(),
                        line: None,
                        message: format!(
                            "crate `{}` depends ({}) on `frob-cert`, allowed only for {:?} (rule L-9, INV-WS-02)",
                            pkg.name, dep.kind, CERT_ALLOWED_DEPENDENTS
                        ),
                        repair: format!(
                            "remove the dependency on `frob-cert` from `{}`; producers never link the auditor",
                            pkg.name
                        ),
                    });
                }
                continue;
            }

            // Repo-internal edge: skip L-2 / L-4 / L-5, evaluate only Step 6 (`edge.forbidden`).
            if is_repo_internal_dep(dep, crate_map) {
                check_proof_boundary_edge(pkg, dep, crate_map, proof_boundary_edges, findings);
                continue;
            }

            // Step 4a: L-5.python-outside-bindings.
            if matches_any_pattern(&dep.name, PYTHON_FORBIDDEN_PATTERNS) {
                if pkg.rel_dir != "bindings/frob-gymnasium" && pkg.name != "frob-gymnasium" {
                    findings.push(Finding {
                        rule: RULE_L5_PYTHON_OUTSIDE_BINDINGS,
                        subject: format!("{} -> {}", pkg.name, dep.name),
                        path: pkg.rel_manifest.clone(),
                        line: None,
                        message: format!(
                            "crate `{}` has a {} dependency on Python binding `{}` outside bindings/frob-gymnasium (rule L-5)",
                            pkg.name, dep.kind, dep.name
                        ),
                        repair: format!(
                            "remove `{}` from `{}`; Python bindings live exclusively in bindings/frob-gymnasium",
                            dep.name, pkg.name
                        ),
                    });
                    continue;
                }
                // Inside bindings/frob-gymnasium: permitted by Python rule; continue to L-4 if matched.
            }

            // Step 4b: L-5.forbidden by package name (tokio, rayon, BLAS/LAPACK, N00-23 rows).
            if let Some(forb) = match_forbidden_direct_name(&dep.name) {
                findings.push(Finding {
                    rule: RULE_L5_FORBIDDEN,
                    subject: format!("{} -> {}", pkg.name, dep.name),
                    path: pkg.rel_manifest.clone(),
                    line: None,
                    message: format!(
                        "crate `{}` has a forbidden {} dependency on `{}` ({}; rule L-5)",
                        pkg.name, dep.kind, dep.name, forb.reason
                    ),
                    repair: format!(
                        "remove forbidden dependency `{}` from `{}`",
                        dep.name, pkg.name
                    ),
                });
                continue;
            }

            // Step 4c: In the core, any direct dependency with a `links` key or build-dep on cc/cmake/bindgen.
            if from_is_core
                && let Some(resolved_target) = resolve_direct_dep_target(resolved, &pkg.id, dep)
                && let Some(native_reason) = core_native_links_or_build_dep_reason(resolved_target)
            {
                findings.push(Finding {
                    rule: RULE_L5_FORBIDDEN,
                    subject: format!("{} -> {}", pkg.name, dep.name),
                    path: pkg.rel_manifest.clone(),
                    line: None,
                    message: format!(
                        "core crate `{}` directly depends ({}) on native-build package `{}` ({native_reason}; rule L-5)",
                        pkg.name, dep.kind, dep.name
                    ),
                    repair: format!(
                        "remove `{}` from core crate `{}` or move native/FFI dependencies into an annex workspace",
                        dep.name, pkg.name
                    ),
                });
                continue;
            }

            // Step 3b: serde / serde_json boundary rules in the core.
            if dep.name == "serde" || dep.name == "serde_json" {
                if from_is_core {
                    check_serde_boundary(pkg, dep, findings);
                }
                continue;
            }

            // Step 1: Rule L-2 (FrankenSim volatile confinement and `fs-qty` confinement).
            if is_frankensim_dep(dep, fsim_row) {
                if dep.name == "fs-qty" && pkg.name != "frob-core" && pkg.name != "frob-fsim" {
                    findings.push(Finding {
                        rule: RULE_L2_FS_QTY_DIRECT,
                        subject: format!("{} -> fs-qty", pkg.name),
                        path: pkg.rel_manifest.clone(),
                        line: None,
                        message: format!(
                            "crate `{}` has a direct {} dependency on `fs-qty`; only `frob-core` and `frob-fsim` may depend on `fs-qty` directly (rule L-2)",
                            pkg.name, dep.kind
                        ),
                        repair: format!(
                            "remove `fs-qty` from `{}` and import quantities through `frob_core::qty`",
                            pkg.name
                        ),
                    });
                    continue;
                }

                if !BEDROCK_DIRECT_CRATES.contains(&dep.name.as_str()) && pkg.name != "frob-fsim" {
                    let src_desc = dep
                        .sibling_rel_path
                        .as_deref()
                        .or(dep.source.as_deref())
                        .unwrap_or("<path>");
                    findings.push(Finding {
                        rule: RULE_FSIM_VOLATILE_EDGE,
                        subject: format!("{} -> {}", pkg.name, dep.name),
                        path: pkg.rel_manifest.clone(),
                        line: None,
                        message: format!(
                            "crate `{}` has a direct {} dependency on volatile FrankenSim crate `{}` ({src_desc}); only `frob-fsim` may depend on non-bedrock FrankenSim crates (rule L-2, INV-SUITE-01)",
                            pkg.name, dep.kind, dep.name
                        ),
                        repair: format!(
                            "remove `{}` from `{}` and route FrankenSim physics/volatile calls through the airlock crate `frob-fsim`",
                            dep.name, pkg.name
                        ),
                    });
                    // DESIGN (BD-003): one finding per defect — no L-4 finding on this edge.
                    continue;
                }
            }

            // Step 2 & Step 3a: Rule L-4 (stack-lock confinement & spec via `edge_allowed`) and L-5.third-party.
            check_foundation_and_third_party(
                pkg,
                dep,
                from_id,
                from_is_core,
                stack_lock,
                findings,
            );
        }
    }
}

fn check_serde_boundary(pkg: &PackageMeta, dep: &DeclaredDep, findings: &mut Vec<Finding>) {
    let is_boundary = SERDE_BOUNDARY_CRATES.contains(&pkg.name.as_str())
        || pkg.rel_dir == "surface/frob-mcp";
    if !is_boundary {
        findings.push(Finding {
            rule: RULE_L5_SERDE_OUTSIDE_BOUNDARY,
            subject: format!("{} -> {}", pkg.name, dep.name),
            path: pkg.rel_manifest.clone(),
            line: None,
            message: format!(
                "core crate `{}` depends ({}) on `{}` outside the six allowed boundary crates {:?} (rule L-5)",
                pkg.name, dep.kind, dep.name, SERDE_BOUNDARY_CRATES
            ),
            repair: format!(
                "remove `{}` from `{}`; use `frob_core::json` and `frob_core::canon` in non-boundary crates",
                dep.name, pkg.name
            ),
        });
        return;
    }

    let is_git = dep
        .source
        .as_deref()
        .is_some_and(|s| s.starts_with("git+"));
    if is_git || dep.path.is_some() || !is_exact_semver(&dep.req) {
        findings.push(Finding {
            rule: RULE_L5_SERDE_NOT_EXACT,
            subject: format!("{} -> {}", pkg.name, dep.name),
            path: pkg.rel_manifest.clone(),
            line: None,
            message: format!(
                "boundary crate `{}` declares `{}` with requirement `{}` instead of an exact `=x.y.z` registry pin (rule L-5)",
                pkg.name, dep.name, dep.req
            ),
            repair: format!(
                "pin `{}` in `{}` to an exact version `=x.y.z` matching franken-stack.lock",
                dep.name, pkg.rel_manifest
            ),
        });
    }
}

fn check_foundation_and_third_party(
    pkg: &PackageMeta,
    dep: &DeclaredDep,
    from_id: &str,
    from_is_core: bool,
    stack_lock: &StackLock,
    findings: &mut Vec<Finding>,
) {
    let Some(row) = match_foundation_row(dep, stack_lock) else {
        if dep.path.is_some() {
            findings.push(Finding {
                rule: RULE_L4_FOUNDATION_CONFINED,
                subject: format!("{} -> {}", pkg.name, dep.name),
                path: pkg.rel_manifest.clone(),
                line: None,
                message: format!(
                    "crate `{}` has a {} path dependency on `{}` ({}) that matches no [[foundation]] row in franken-stack.lock (rule L-4)",
                    pkg.name,
                    dep.kind,
                    dep.name,
                    dep.sibling_rel_path.as_deref().unwrap_or("<external-path>")
                ),
                repair: format!(
                    "remove `{}` from `{}` or admit the foundation in franken-stack.lock",
                    dep.name, pkg.name
                ),
            });
        } else if from_is_core {
            findings.push(Finding {
                rule: RULE_L5_THIRD_PARTY,
                subject: format!("{} -> {}", pkg.name, dep.name),
                path: pkg.rel_manifest.clone(),
                line: None,
                message: format!(
                    "core crate `{}` has an unadmitted {} third-party dependency `{}` (`{}`) with no [[foundation]] row (rule L-5)",
                    pkg.name, dep.kind, dep.name, dep.req
                ),
                repair: format!(
                    "remove third-party dependency `{}` from `{}` or admit it through DEPENDENCY_ADMISSION.md and franken-stack.lock",
                    dep.name, pkg.name
                ),
            });
        } else {
            findings.push(Finding {
                rule: RULE_L4_FOUNDATION_CONFINED,
                subject: format!("{} -> {}", pkg.name, dep.name),
                path: pkg.rel_manifest.clone(),
                line: None,
                message: format!(
                    "annex crate `{}` has a {} dependency on `{}` with no [[foundation]] row in franken-stack.lock (rule L-4)",
                    pkg.name, dep.kind, dep.name
                ),
                repair: format!(
                    "remove `{}` from `{}` or add its [[foundation]] row to franken-stack.lock",
                    dep.name, pkg.name
                ),
            });
        }
        return;
    };

    let edge_kind = to_edge_kind(dep.kind);
    let spec = build_dep_spec(dep, row);
    let is_bedrock_target = BEDROCK_DIRECT_CRATES.contains(&dep.name.as_str());

    let mut any_spec_denial: Option<EdgeDenial> = None;
    let mut first_confinement_denial: Option<EdgeDenial> = None;

    for use_ in &row.uses {
        let effective_from = if is_bedrock_target
            && use_.confined_to.iter().any(|c| c == "bedrock-direct")
        {
            "bedrock-direct"
        } else {
            from_id
        };

        // Authoritative posture-to-graph gate (N33a-01 / BD-003).
        if edge_allowed(use_, effective_from, edge_kind, &spec) {
            return;
        }

        if let EdgeDecision::Deny(denial) = check_edge(use_, effective_from, edge_kind, &spec) {
            if is_spec_denial(denial) {
                if any_spec_denial.is_none() {
                    any_spec_denial = Some(denial);
                }
            } else if first_confinement_denial.is_none() {
                first_confinement_denial = Some(denial);
            }
        }
    }

    if let Some(spec_denial) = any_spec_denial {
        findings.push(Finding {
            rule: RULE_L4_EDGE_SPEC,
            subject: format!("{} -> {}", pkg.name, dep.name),
            path: pkg.rel_manifest.clone(),
            line: None,
            message: format!(
                "crate `{}` is admitted for foundation `{}` ({}), but {} dependency `{}` violates posture spec: {} (rule L-4)",
                pkg.name,
                row.id,
                row.name,
                dep.kind,
                dep.name,
                describe_denial(spec_denial)
            ),
            repair: repair_for_spec_denial(row, dep, spec_denial),
        });
    } else {
        let reason = first_confinement_denial
            .map(describe_denial)
            .unwrap_or("crate or edge kind is not admitted by any declared use");
        findings.push(Finding {
            rule: RULE_L4_FOUNDATION_CONFINED,
            subject: format!("{} -> {}", pkg.name, dep.name),
            path: pkg.rel_manifest.clone(),
            line: None,
            message: format!(
                "crate `{}` is not permitted a {} dependency on foundation `{}` ({}) under franken-stack.lock: {reason} (rule L-4)",
                pkg.name, dep.kind, row.id, row.name
            ),
            repair: format!(
                "remove `{}` from `{}` or restrict its use to a crate and edge kind admitted by `{}` in franken-stack.lock",
                dep.name, pkg.name, row.id
            ),
        });
    }
}

fn check_proof_boundary_edge(
    pkg: &PackageMeta,
    dep: &DeclaredDep,
    crate_map: &[CrateRow],
    proof_boundary_edges: &[ForbiddenEdgeRow],
    findings: &mut Vec<Finding>,
) {
    let Some(row) = proof_boundary_edges
        .iter()
        .find(|r| r.from == pkg.name && r.to == dep.name)
    else {
        return;
    };

    // Single-owner rule: if `check-layers` already forbids this edge, stay silent here.
    let edge = GraphEdge {
        from: pkg.name.clone(),
        from_rel_dir: pkg.rel_dir.clone(),
        from_rel_manifest: pkg.rel_manifest.clone(),
        to: dep.name.clone(),
        to_rel_path: dep.rel_path.clone(),
        kind: dep.kind,
    };
    if !check_edges(&[edge], crate_map).is_empty() {
        return;
    }

    findings.push(Finding {
        rule: RULE_EDGE_FORBIDDEN,
        subject: format!("{} -> {}", pkg.name, dep.name),
        path: pkg.rel_manifest.clone(),
        line: None,
        message: format!(
            "forbidden proof-boundary {} edge `{} -> {}`: {} (BD-003)",
            dep.kind, pkg.name, dep.name, row.reason
        ),
        repair: format!(
            "remove the proof-boundary dependency `{} -> {}` from {}",
            pkg.name, dep.name, pkg.rel_manifest
        ),
    });
}

fn build_workspace_closure_facts(resolved: &ResolvedWorkspace) -> FactValue {
    let total = resolved.packages.len();
    let registry_count = resolved
        .packages
        .values()
        .filter(|p| p.is_registry())
        .count();

    let hits = collect_transitive_forbidden_hits(resolved);
    let hit_values: Vec<FactValue> = hits
        .into_iter()
        .map(|(pkg_name, reason, path)| {
            let mut obj = BTreeMap::new();
            obj.insert("package".to_owned(), FactValue::str(pkg_name));
            obj.insert("path".to_owned(), FactValue::str_array(path));
            obj.insert("reason".to_owned(), FactValue::str(reason));
            FactValue::Object(obj)
        })
        .collect();

    let mut ws_obj = BTreeMap::new();
    ws_obj.insert("packages_total".to_owned(), FactValue::int_usize(total));
    ws_obj.insert(
        "registry_packages".to_owned(),
        FactValue::int_usize(registry_count),
    );
    ws_obj.insert(
        "transitive_forbidden_hits".to_owned(),
        FactValue::Array(hit_values),
    );
    FactValue::Object(ws_obj)
}

/// Deterministic BFS from workspace members over `resolved.nodes` collecting every
/// transitive forbidden package hit with one shortest dependency path.
fn collect_transitive_forbidden_hits(
    resolved: &ResolvedWorkspace,
) -> Vec<(String, &'static str, Vec<String>)> {
    let mut member_ids: Vec<&str> = resolved
        .workspace_members
        .iter()
        .map(String::as_str)
        .collect();
    member_ids.sort_by(|a, b| {
        let na = resolved.packages.get(*a).map_or(*a, |p| p.name.as_str());
        let nb = resolved.packages.get(*b).map_or(*b, |p| p.name.as_str());
        na.cmp(nb).then_with(|| a.cmp(b))
    });

    // BFS tracking parent pointer for deterministic shortest path from any workspace member.
    let mut parent: BTreeMap<&str, Option<&str>> = BTreeMap::new();
    let mut depth: BTreeMap<&str, usize> = BTreeMap::new();
    let mut queue: VecDeque<&str> = VecDeque::new();

    for &mid in &member_ids {
        parent.insert(mid, None);
        depth.insert(mid, 0);
        queue.push_back(mid);
    }

    while let Some(cur) = queue.pop_front() {
        let cur_depth = depth.get(cur).copied().unwrap_or(0);
        let Some(node) = resolved.nodes.get(cur) else {
            continue;
        };
        let mut children: Vec<&str> = node.dependencies.iter().map(String::as_str).collect();
        children.sort_by(|a, b| {
            let na = resolved.packages.get(*a).map_or(*a, |p| p.name.as_str());
            let nb = resolved.packages.get(*b).map_or(*b, |p| p.name.as_str());
            na.cmp(nb).then_with(|| a.cmp(b))
        });
        for child in children {
            if !parent.contains_key(child) {
                parent.insert(child, Some(cur));
                depth.insert(child, cur_depth + 1);
                queue.push_back(child);
            }
        }
    }

    let mut seen_pkg_names: BTreeSet<String> = BTreeSet::new();
    let mut results = Vec::new();
    let mut reachable: Vec<(&str, &ResolvedPackage)> = parent
        .keys()
        .filter_map(|id| resolved.packages.get(*id).map(|p| (*id, p)))
        .collect();
    reachable.sort_by(|a, b| a.1.name.cmp(&b.1.name).then_with(|| a.0.cmp(b.0)));

    for (pkg_id, pkg) in reachable {
        if resolved.workspace_members.contains(pkg_id) {
            continue;
        }
        let reason = match_forbidden_direct_name(&pkg.name)
            .map(|r| r.reason)
            .or_else(|| {
                matches_any_pattern(&pkg.name, PYTHON_FORBIDDEN_PATTERNS)
                    .then_some("L-5:python-binding")
            });
        let Some(reason) = reason else {
            continue;
        };
        if !seen_pkg_names.insert(pkg.name.clone()) {
            continue;
        }

        let mut chain = Vec::new();
        let mut cursor = Some(pkg_id);
        while let Some(id) = cursor {
            let label = resolved
                .packages
                .get(id)
                .map_or_else(|| id.to_owned(), |p| p.name.clone());
            chain.push(label);
            cursor = parent.get(id).copied().flatten();
        }
        chain.reverse();
        results.push((pkg.name.clone(), reason, chain));
    }

    results.sort();
    results
}

fn resolve_direct_dep_target<'a>(
    resolved: &'a ResolvedWorkspace,
    pkg_id: &str,
    dep: &DeclaredDep,
) -> Option<&'a ResolvedPackage> {
    if let Some(node) = resolved.nodes.get(pkg_id) {
        for rdep in &node.deps {
            if let Some(target_pkg) = resolved.packages.get(&rdep.pkg)
                && target_pkg.name == dep.name
            {
                return Some(target_pkg);
            }
        }
    }
    resolved.packages.values().find(|p| p.name == dep.name)
}

fn core_native_links_or_build_dep_reason(pkg: &ResolvedPackage) -> Option<String> {
    if let Some(links) = pkg.links.as_deref()
        && !links.trim().is_empty()
    {
        return Some(format!("declares `links = \"{links}\"`"));
    }
    for bdep in &pkg.dependencies {
        if bdep.kind == DepKind::Build
            && FORBIDDEN_NATIVE_BUILD_DEPS.contains(&bdep.name.as_str())
        {
            return Some(format!("build-depends on `{}`", bdep.name));
        }
    }
    None
}

fn find_fsim_foundation(stack_lock: &StackLock) -> Option<&Foundation> {
    stack_lock
        .foundations
        .iter()
        .find(|f| f.id.as_str() == "FD-FSIM" || f.name == "frankensim")
}

fn is_frankensim_dep(dep: &DeclaredDep, fsim_row: Option<&Foundation>) -> bool {
    if let Some(sib_path) = dep.sibling_rel_path.as_deref() {
        let fsim_prefix = fsim_row
            .and_then(|r| r.path.as_ref())
            .map_or("../frankensim", |p| p.as_str());
        if sib_path == fsim_prefix || sib_path.starts_with(&format!("{fsim_prefix}/")) {
            return true;
        }
    }
    if let Some(source) = dep.source.as_deref()
        && let Some(url) = source.strip_prefix("git+")
    {
        let slug = normalize_pkg_ident(git_url_repo_name(url));
        let expected = fsim_row.map_or("frankensim", |r| r.name.as_str());
        if slug == normalize_pkg_ident(expected) {
            return true;
        }
    }
    if dep.name == "fs-qty" && (dep.path.is_some() || dep.source.is_some()) {
        return true;
    }
    false
}

fn match_foundation_row<'a>(dep: &DeclaredDep, stack_lock: &'a StackLock) -> Option<&'a Foundation> {
    // 1. Sibling path prefix match (consume-path / build-time / oracle / path-form consume).
    if let Some(sib_path) = dep.sibling_rel_path.as_deref() {
        for row in &stack_lock.foundations {
            if let Some(row_path) = row.path.as_ref() {
                let rp = row_path.as_str().trim_end_matches('/');
                if sib_path == rp || sib_path.starts_with(&format!("{rp}/")) {
                    return Some(row);
                }
            }
        }
    }

    // 2. Package name match (exact or hyphen/underscore normalized).
    let dep_norm = normalize_pkg_ident(&dep.name);
    for row in &stack_lock.foundations {
        if row.name == dep.name || normalize_pkg_ident(&row.name) == dep_norm {
            return Some(row);
        }
    }

    // 3. Git URL repo-name match (so a consume-path foundation specified via git= maps to L-4.edge-spec).
    if let Some(source) = dep.source.as_deref()
        && let Some(url) = source.strip_prefix("git+")
    {
        let repo_slug = git_url_repo_name(url);
        let slug_norm = normalize_pkg_ident(repo_slug);
        for row in &stack_lock.foundations {
            if normalize_pkg_ident(&row.name) == slug_norm {
                return Some(row);
            }
            if let Some(row_path) = row.path.as_ref()
                && let Some(dir_name) = row_path.as_str().rsplit('/').next()
                && normalize_pkg_ident(dir_name) == slug_norm
            {
                return Some(row);
            }
        }
    }

    // 4. FrankenSim bedrock/fs-* crate referenced via git or registry maps to FD-FSIM.
    if BEDROCK_DIRECT_CRATES.contains(&dep.name.as_str()) {
        return find_fsim_foundation(stack_lock);
    }

    None
}

fn git_url_repo_name(url: &str) -> &str {
    let base = url.split(['?', '#']).next().unwrap_or(url);
    let trimmed = base.trim_end_matches('/');
    let seg = trimmed.rsplit('/').next().unwrap_or(trimmed);
    seg.strip_suffix(".git").unwrap_or(seg)
}

fn normalize_pkg_ident(name: &str) -> String {
    name.replace('-', "_")
}

fn build_dep_spec(dep: &DeclaredDep, row: &Foundation) -> DepSpec {
    let mut spec = if let Some(source) = dep.source.as_deref()
        && let Some(url) = source.strip_prefix("git+")
    {
        DepSpec::git(url, dep.uses_default_features)
    } else if let Some(sib_path) = dep.sibling_rel_path.as_deref() {
        let mut s = DepSpec::path(sib_path, dep.uses_default_features);
        // Pure path dependencies report `req == "*"` in cargo metadata; only attach version
        // when the manifest explicitly declared a version requirement alongside `path`.
        if dep.req != "*" {
            s.version = Some(dep.req.clone());
        }
        s
    } else if dep.path.is_some() {
        let raw_path = dep
            .path
            .as_deref()
            .map(normalize_rel_path)
            .unwrap_or_default();
        DepSpec::path(raw_path, dep.uses_default_features)
    } else {
        DepSpec::registry(&dep.req, dep.uses_default_features)
    };

    if let Some(v) = row.version.as_ref() {
        spec = spec.with_row_version(v.as_str());
    }
    if let Some(p) = row.path.as_ref() {
        spec = spec.with_row_path(p.as_str());
    }
    spec
}

const fn to_edge_kind(kind: DepKind) -> EdgeKind {
    match kind {
        DepKind::Normal => EdgeKind::Normal,
        DepKind::Build => EdgeKind::Build,
        DepKind::Dev => EdgeKind::Dev,
    }
}

const fn is_spec_denial(denial: EdgeDenial) -> bool {
    matches!(
        denial,
        EdgeDenial::DefaultFeaturesEnabled
            | EdgeDenial::GitSourceForbidden
            | EdgeDenial::VersionNotExact
            | EdgeDenial::VersionMismatch
            | EdgeDenial::UnexpectedPathOnRegistryConsume
            | EdgeDenial::UnexpectedVersionOnConsumePath
            | EdgeDenial::MissingOrMismatchedPath
    )
}

const fn describe_denial(denial: EdgeDenial) -> &'static str {
    match denial {
        EdgeDenial::PostureForbidsCargoEdge => "posture forbids any Cargo dependency edge",
        EdgeDenial::EdgeKindNotAllowed => "dependency kind (normal/build/dev) is not permitted by this posture",
        EdgeDenial::CrateNotConfined => "crate is not listed in `confined_to`",
        EdgeDenial::CoreCrateInBuildTime => "core crate may not use a `build-time` annex foundation",
        EdgeDenial::DefaultFeaturesEnabled => "`default-features = false` is required",
        EdgeDenial::GitSourceForbidden => "git dependencies are forbidden",
        EdgeDenial::VersionNotExact => "exact version pin `=x.y.z` is required",
        EdgeDenial::VersionMismatch => "version pin does not match the foundation row in franken-stack.lock",
        EdgeDenial::UnexpectedPathOnRegistryConsume => "`consume` posture requires an exact registry pin, not a path dependency",
        EdgeDenial::UnexpectedVersionOnConsumePath => "`consume-path` posture must not carry a registry version requirement",
        EdgeDenial::MissingOrMismatchedPath => "dependency path does not match the foundation row's sibling path",
    }
}

fn repair_for_spec_denial(row: &Foundation, dep: &DeclaredDep, denial: EdgeDenial) -> String {
    match denial {
        EdgeDenial::DefaultFeaturesEnabled => format!(
            "set `default-features = false` on `{}` in Cargo.toml",
            dep.name
        ),
        EdgeDenial::VersionNotExact | EdgeDenial::VersionMismatch => match row.version.as_ref() {
            Some(ver) => format!(
                "pin `{}` to `version = \"{ver}\", default-features = false` matching {}",
                dep.name, row.id
            ),
            None => format!(
                "pin `{}` to an exact `=x.y.z` version with `default-features = false`",
                dep.name
            ),
        },
        EdgeDenial::GitSourceForbidden | EdgeDenial::MissingOrMismatchedPath => {
            match row.path.as_ref() {
                Some(p) => format!(
                    "replace `{}` with a path dependency under `{}` with `default-features = false`",
                    dep.name, p
                ),
                None => format!(
                    "replace git dependency `{}` with the exact pin recorded in {}",
                    dep.name, row.id
                ),
            }
        }
        EdgeDenial::UnexpectedPathOnRegistryConsume => match row.version.as_ref() {
            Some(ver) => format!(
                "replace path dependency `{}` with `version = \"{ver}\", default-features = false`",
                dep.name
            ),
            None => format!("replace path dependency `{}` with an exact registry version", dep.name),
        },
        EdgeDenial::UnexpectedVersionOnConsumePath => format!(
            "remove the version requirement from path dependency `{}`",
            dep.name
        ),
        _ => format!("update `{}` dependency specification to match {}", dep.name, row.id),
    }
}

fn is_core_package(pkg: &PackageMeta, ws_kind: WorkspaceKind, crate_map: &[CrateRow]) -> bool {
    if ws_kind == WorkspaceKind::Annex || ws_kind == WorkspaceKind::Link {
        return false;
    }
    if pkg.rel_dir.starts_with("spark/")
        || pkg.rel_dir.starts_with("derive/")
        || pkg.rel_dir.starts_with("bindings/")
        || pkg.rel_dir.starts_with("apps/")
    {
        return false;
    }
    !lookup_in(crate_map, &pkg.name).is_some_and(|r| r.layer == Layer::Annex)
}

fn confinement_subject_for(pkg: &PackageMeta) -> &str {
    if pkg.rel_dir.starts_with("spark/")
        || pkg.rel_dir.starts_with("derive/")
        || pkg.rel_dir.starts_with("bindings/")
        || pkg.rel_dir.starts_with("surface/")
        || pkg.rel_dir.starts_with("apps/")
    {
        pkg.rel_dir.as_str()
    } else {
        pkg.name.as_str()
    }
}

fn is_repo_internal_dep(dep: &DeclaredDep, crate_map: &[CrateRow]) -> bool {
    dep.rel_path.is_some()
        || (dep.source.is_none()
            && dep.path.is_none()
            && lookup_in(crate_map, &dep.name).is_some())
}

/// Match a package name against an exact name or trailing-`*` prefix pattern.
#[must_use]
pub fn pattern_matches(pattern: &str, name: &str) -> bool {
    if let Some(prefix) = pattern.strip_suffix('*') {
        name.starts_with(prefix)
    } else {
        name == pattern
    }
}

#[must_use]
pub fn match_forbidden_direct_name(name: &str) -> Option<&'static ForbiddenNameRow> {
    FORBIDDEN_DIRECT_NAMES
        .iter()
        .find(|row| pattern_matches(row.pattern, name))
}

#[must_use]
pub fn matches_any_pattern(name: &str, patterns: &[&str]) -> bool {
    patterns.iter().any(|p| pattern_matches(p, name))
}

// ============================================================================
// Step 1c: L-2.airlock-reexport source scan on `frob-fsim`
// ============================================================================

fn scan_airlock_reexports(pkg: &PackageMeta, findings: &mut Vec<Finding>) {
    let src_dir = pkg.crate_dir().join("src");
    let mut rs_files = Vec::new();
    collect_rs_files(&src_dir, &mut rs_files);
    rs_files.sort();

    for file_path in rs_files {
        let Ok(src) = fs::read_to_string(&file_path) else {
            continue;
        };
        let rel_file = file_path
            .strip_prefix(pkg.crate_dir())
            .ok()
            .map(|r| {
                let sub = normalize_rel_path(r);
                if pkg.rel_dir.is_empty() {
                    sub
                } else {
                    format!("{}/{sub}", pkg.rel_dir)
                }
            })
            .unwrap_or_else(|| normalize_rel_path(&file_path));

        for (line, crate_ident) in find_volatile_reexports(&src) {
            findings.push(Finding {
                rule: RULE_L2_AIRLOCK_REEXPORT,
                subject: format!("frob-fsim::{crate_ident}"),
                path: rel_file.clone(),
                line: Some(line),
                message: format!(
                    "`frob-fsim` re-exports FrankenSim crate `{crate_ident}` publicly (rule L-2, INV-SUITE-01)"
                ),
                repair: format!(
                    "remove `pub use {crate_ident}` / `pub extern crate {crate_ident}` and expose only FrankenRobotics airlock wrapper types"
                ),
            });
        }
    }
}

fn collect_rs_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    let mut paths: Vec<PathBuf> = entries.filter_map(|e| e.ok().map(|x| x.path())).collect();
    paths.sort();
    for path in paths {
        if path.is_dir() {
            collect_rs_files(&path, out);
        } else if path.extension().and_then(|e| e.to_str()) == Some("rs") {
            out.push(path);
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Tok {
    Ident(String),
    Punct(char),
}

/// Find bare `pub use [::]fs_<name>` and `pub extern crate fs_<name>` re-exports
/// of volatile FrankenSim crates (or `fs_qty`) in Rust source, ignoring comments/strings
/// and restricted `pub(...)` visibility.
fn find_volatile_reexports(src: &str) -> Vec<(u32, String)> {
    let tokens = lex_rust_tokens(src);
    let mut hits = Vec::new();
    let mut i = 0usize;

    while i < tokens.len() {
        if let (line, Tok::Ident(word)) = &tokens[i]
            && word == "pub"
        {
            // Skip restricted visibility `pub(crate)` / `pub(super)` / `pub(in ...)`.
            if matches!(tokens.get(i + 1), Some((_, Tok::Punct('(')))) {
                i += 1;
                continue;
            }
            // Pattern A: `pub use [::] fs_<crate>...`
            if matches!(tokens.get(i + 1), Some((_, Tok::Ident(w))) if w == "use") {
                let mut target_idx = i + 2;
                if matches!(tokens.get(target_idx), Some((_, Tok::Punct(':'))))
                    && matches!(tokens.get(target_idx + 1), Some((_, Tok::Punct(':'))))
                {
                    target_idx += 2;
                }
                if let Some((_, Tok::Ident(crate_ident))) = tokens.get(target_idx)
                    && is_forbidden_reexport_crate(crate_ident)
                {
                    hits.push((*line, crate_ident.clone()));
                }
            }
            // Pattern B: `pub extern crate fs_<crate>...`
            if matches!(tokens.get(i + 1), Some((_, Tok::Ident(w))) if w == "extern")
                && matches!(tokens.get(i + 2), Some((_, Tok::Ident(w))) if w == "crate")
                && let Some((_, Tok::Ident(crate_ident))) = tokens.get(i + 3)
                && is_forbidden_reexport_crate(crate_ident)
            {
                hits.push((*line, crate_ident.clone()));
            }
        }
        i += 1;
    }

    hits
}

fn is_forbidden_reexport_crate(ident: &str) -> bool {
    if !ident.starts_with("fs_") {
        return false;
    }
    let kebab = ident.replace('_', "-");
    // Any volatile FrankenSim crate (not in bedrock-direct) plus `fs-qty` (owned by frob-core::qty).
    kebab == "fs-qty" || !BEDROCK_DIRECT_CRATES.contains(&kebab.as_str())
}

fn lex_rust_tokens(src: &str) -> Vec<(u32, Tok)> {
    let bytes = src.as_bytes();
    let len = bytes.len();
    let mut i = 0usize;
    let mut line = 1u32;
    let mut out = Vec::new();

    while i < len {
        let b = bytes[i];
        if b == b'\n' {
            line = line.saturating_add(1);
            i += 1;
            continue;
        }
        if matches!(b, b' ' | b'\t' | b'\r') {
            i += 1;
            continue;
        }
        // Line comment
        if b == b'/' && bytes.get(i + 1) == Some(&b'/') {
            i += 2;
            while i < len && bytes[i] != b'\n' {
                i += 1;
            }
            continue;
        }
        // Nested block comment
        if b == b'/' && bytes.get(i + 1) == Some(&b'*') {
            let mut depth = 1usize;
            i += 2;
            while i < len && depth > 0 {
                if bytes[i] == b'\n' {
                    line = line.saturating_add(1);
                    i += 1;
                } else if bytes[i] == b'/' && bytes.get(i + 1) == Some(&b'*') {
                    depth += 1;
                    i += 2;
                } else if bytes[i] == b'*' && bytes.get(i + 1) == Some(&b'/') {
                    depth -= 1;
                    i += 2;
                } else {
                    i += 1;
                }
            }
            continue;
        }
        // Raw strings r#"..."# / br#"..."# / cr#"..."#
        if let Some(next_i) = skip_raw_string(bytes, i, &mut line) {
            i = next_i;
            continue;
        }
        // Byte/C strings b"..." / c"..."
        if matches!(b, b'b' | b'c') && bytes.get(i + 1) == Some(&b'"') {
            i = skip_quoted(bytes, i + 1, &mut line);
            continue;
        }
        // Normal string "..."
        if b == b'"' {
            i = skip_quoted(bytes, i, &mut line);
            continue;
        }
        // Char / byte literal vs lifetime
        if b == b'b' && bytes.get(i + 1) == Some(&b'\'')
            && let Some(next_i) = skip_char_lit(bytes, i + 1, &mut line)
        {
            i = next_i;
            continue;
        }
        if b == b'\'' {
            if let Some(next_i) = skip_char_lit(bytes, i, &mut line) {
                i = next_i;
                continue;
            }
            i += 1;
            while i < len && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') {
                i += 1;
            }
            continue;
        }
        if b.is_ascii_alphabetic() || b == b'_' {
            let start = i;
            let tok_line = line;
            i += 1;
            while i < len && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') {
                i += 1;
            }
            out.push((tok_line, Tok::Ident(src[start..i].to_owned())));
            continue;
        }
        if b.is_ascii_punctuation() {
            out.push((line, Tok::Punct(b as char)));
        }
        i += 1;
    }

    out
}

fn skip_quoted(bytes: &[u8], open: usize, line: &mut u32) -> usize {
    let mut i = open + 1;
    while i < bytes.len() {
        match bytes[i] {
            b'\\' => {
                if bytes.get(i + 1) == Some(&b'\n') {
                    *line = line.saturating_add(1);
                }
                i = (i + 2).min(bytes.len());
            }
            b'"' => return i + 1,
            b'\n' => {
                *line = line.saturating_add(1);
                i += 1;
            }
            _ => i += 1,
        }
    }
    bytes.len()
}

fn skip_raw_string(bytes: &[u8], start: usize, line: &mut u32) -> Option<usize> {
    let mut i = start;
    if matches!(bytes.get(i), Some(b'b' | b'c')) {
        i += 1;
    }
    if bytes.get(i) != Some(&b'r') {
        return None;
    }
    i += 1;
    let mut hashes = 0usize;
    while bytes.get(i) == Some(&b'#') {
        hashes += 1;
        i += 1;
    }
    if bytes.get(i) != Some(&b'"') {
        return None;
    }
    i += 1;
    while i < bytes.len() {
        if bytes[i] == b'\n' {
            *line = line.saturating_add(1);
            i += 1;
        } else if bytes[i] == b'"' {
            let mut m = 0usize;
            while m < hashes && bytes.get(i + 1 + m) == Some(&b'#') {
                m += 1;
            }
            if m == hashes {
                return Some(i + 1 + hashes);
            }
            i += 1;
        } else {
            i += 1;
        }
    }
    Some(bytes.len())
}

fn skip_char_lit(bytes: &[u8], open: usize, line: &mut u32) -> Option<usize> {
    let next = *bytes.get(open + 1)?;
    if next == b'\\' {
        let mut i = open + 2;
        if i >= bytes.len() {
            return None;
        }
        i += 1;
        while i < bytes.len() && bytes[i] != b'\'' && bytes[i] != b'\n' {
            i += 1;
        }
        return (bytes.get(i) == Some(&b'\'')).then_some(i + 1);
    }
    let rest = std::str::from_utf8(&bytes[open + 1..]).ok()?;
    let ch = rest.chars().next()?;
    let after = open + 1 + ch.len_utf8();
    if bytes.get(after) == Some(&b'\'') {
        if ch == '\n' {
            *line = line.saturating_add(1);
        }
        return Some(after + 1);
    }
    None
}
