//! Authoritative workspace table, core forbidden-package list, repository layout,
//! and lockfile/toolchain-file rules for FrankenRobotics (`N34p4-02`, §34.1, §34.3,
//! §21.2 ADR-SPARK-01, §33.13, §33.14).
//!
//! Single source of truth consumed by:
//! - `xtask::cli` (`CheckContext::roots` passed to `check-layers`, `check-deps`,
//!   `check-contracts`, `check-unsafe`),
//! - `xtask::check_deps` (`core.forbidden-package` over the core lockfile and resolved graph),
//! - `xtask::check_layers` + `xtask::crate_map::validate` (exempt `apps/` link packages),
//! - `scripts/e2e/workspace_layout.sh` (reads this file directly for table & lockfile checks),
//! - downstream Spark policy (`N21a-01`, `N21c-24`) and non-goal (`N00-23`) beads.
//!
//! Determinism class D0: compiled tables are sorted deterministically by name/pattern.

use std::collections::BTreeSet;
use std::fmt;
use std::fs;
use std::path::Path;

use frob_core::registry::toml::{self, Value};

use crate::metadata::{WorkspaceKind, WorkspaceRoot, normalize_rel_path};

/// Expected number of Rust workspaces in [`WORKSPACES`].
///
/// Raised to `8` in the same commit if the MCP kill gate (`KILL-MCP`, `N32a-06`)
/// splits `surface/frob-mcp` into its own `mcp` annex workspace.
pub const EXPECTED_WORKSPACE_COUNT: usize = 7;

/// Expected number of workspaces in [`WORKSPACES`] that link code (`Core`, `Annex`, `Link`).
///
/// Raised to `7` if `N32a-06` adds the `mcp` annex row.
pub const EXPECTED_CODE_LINKING_COUNT: usize = 6;

/// Expected number of workspaces in [`WORKSPACES`] that only re-target an existing crate (`Retarget`).
pub const EXPECTED_RETARGET_COUNT: usize = 1;

/// Exempt thin-binary link package names under `apps/` (`N34p4-02`, `N34-01`, `N34-02`).
///
/// These two-line binaries hold no logic and carry no `crate_map.rs` row; any other
/// package under `apps/` fails `ADR-WS-01.unlisted-crate`.
pub const APPS_LINK_PACKAGES: &[&str] = &["frob", "frobd"];

/// Governing root toolchain file naming the single workspace toolchain T.
pub const ROOT_TOOLCHAIN_FILE: &str = "rust-toolchain.toml";

/// Standalone stable auditor toolchain exception (`crates/frob-cert`).
pub const CERT_TOOLCHAIN_FILE: &str = "crates/frob-cert/rust-toolchain.toml";

/// Conditional worker fallback toolchain exception (`worker/`, active only while `KILL-FSIM`
/// fallback (a) is recorded in `franken-stack.lock`).
pub const WORKER_TOOLCHAIN_FILE: &str = "worker/rust-toolchain.toml";

/// Repository-relative paths of optional justification registries cross-checked by
/// [`validate_workspace_table_against_repo`].
pub const ROUTES_NOT_TAKEN_PATH: &str = "registries/routes_not_taken.toml";
pub const NONGOALS_PATH: &str = "registries/nongoals.toml";

/// One Rust workspace row in the repository's closed workspace table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WorkspaceRow {
    /// Stable workspace identifier (`core`, `spark-cuda`, `spark-metal`, `apps`, `derive`, `bindings`, `worker`).
    pub name: &'static str,
    /// Repository-relative root directory (`""` for the root `core` workspace).
    pub root_dir: &'static str,
    /// Repository-relative committed `Cargo.lock` path.
    pub lockfile: &'static str,
    /// Architectural boundary classification (`Core`, `Annex`, `Link`, `Retarget`).
    pub kind: WorkspaceKind,
    /// Exempt link-package names passed to `crate_map::validate` (non-empty only for `Link`).
    pub link_packages: &'static [&'static str],
    /// True when this workspace owns `surface/frob-mcp` (`core` by default; flipped by `N32a-06`).
    pub includes_surface_mcp: bool,
}

impl WorkspaceRow {
    /// True for workspaces that link code (`Core`, `Annex`, `Link`); false for `Retarget`.
    #[must_use]
    pub const fn links_code(self) -> bool {
        !matches!(self.kind, WorkspaceKind::Retarget)
    }

    /// Expected `Cargo.lock` path derived strictly from `root_dir`.
    #[must_use]
    pub fn expected_lockfile(self) -> String {
        if self.root_dir.is_empty() {
            "Cargo.lock".to_owned()
        } else {
            format!("{}/Cargo.lock", self.root_dir)
        }
    }

    /// Materialize this compiled row into a [`WorkspaceRoot`] rooted at `repo_root`.
    #[must_use]
    pub fn to_workspace_root(self, repo_root: &Path) -> WorkspaceRoot {
        let abs_dir = if self.root_dir.is_empty() {
            repo_root.to_path_buf()
        } else {
            repo_root.join(self.root_dir)
        };
        match self.kind {
            WorkspaceKind::Core => WorkspaceRoot::core(abs_dir),
            WorkspaceKind::Annex => WorkspaceRoot::annex(self.name, self.root_dir, abs_dir),
            WorkspaceKind::Link => WorkspaceRoot::link(
                self.name,
                self.root_dir,
                abs_dir,
                self.link_packages.iter().map(|s| (*s).to_owned()).collect(),
            ),
            WorkspaceKind::Retarget => WorkspaceRoot::retarget(self.name, self.root_dir, abs_dir),
        }
    }
}

/// Authoritative table of the seven Rust workspaces of FrankenRobotics (`N34p4-02`).
///
/// Six link code (`core`, `spark-cuda`, `spark-metal`, `apps`, `derive`, `bindings`)
/// and one (`worker`) only re-targets an existing crate (`frob-fsim`).
pub const WORKSPACES: &[WorkspaceRow] = &[
    WorkspaceRow {
        name: "core",
        root_dir: "",
        lockfile: "Cargo.lock",
        kind: WorkspaceKind::Core,
        link_packages: &[],
        includes_surface_mcp: true,
    },
    WorkspaceRow {
        name: "spark-cuda",
        root_dir: "spark/frob-spark-cuda",
        lockfile: "spark/frob-spark-cuda/Cargo.lock",
        kind: WorkspaceKind::Annex,
        link_packages: &[],
        includes_surface_mcp: false,
    },
    WorkspaceRow {
        name: "spark-metal",
        root_dir: "spark/frob-spark-metal",
        lockfile: "spark/frob-spark-metal/Cargo.lock",
        kind: WorkspaceKind::Annex,
        link_packages: &[],
        includes_surface_mcp: false,
    },
    WorkspaceRow {
        name: "apps",
        root_dir: "apps",
        lockfile: "apps/Cargo.lock",
        kind: WorkspaceKind::Link,
        link_packages: APPS_LINK_PACKAGES,
        includes_surface_mcp: false,
    },
    WorkspaceRow {
        name: "derive",
        root_dir: "derive/frob-derive",
        lockfile: "derive/frob-derive/Cargo.lock",
        kind: WorkspaceKind::Annex,
        link_packages: &[],
        includes_surface_mcp: false,
    },
    WorkspaceRow {
        name: "bindings",
        root_dir: "bindings/frob-gymnasium",
        lockfile: "bindings/frob-gymnasium/Cargo.lock",
        kind: WorkspaceKind::Annex,
        link_packages: &[],
        includes_surface_mcp: false,
    },
    WorkspaceRow {
        name: "worker",
        root_dir: "worker",
        lockfile: "worker/Cargo.lock",
        kind: WorkspaceKind::Retarget,
        link_packages: &[],
        includes_surface_mcp: false,
    },
];

/// Look up a compiled workspace row by its stable name.
#[must_use]
pub fn workspace_by_name(name: &str) -> Option<&'static WorkspaceRow> {
    WORKSPACES.iter().find(|w| w.name == name)
}

/// Build [`WorkspaceRoot`] entries for all rows of [`WORKSPACES`] rooted at `repo_root`.
///
/// Passed to `check-layers`, `check-deps`, `check-contracts`, and `check-unsafe` via
/// `xtask::cli`. Rows whose root has no `Cargo.toml` yet are skipped by the checks
/// and recorded under `facts.workspaces_absent` (never reported as clean).
#[must_use]
pub fn workspace_roots(repo_root: &Path) -> Vec<WorkspaceRoot> {
    WORKSPACES
        .iter()
        .map(|row| row.to_workspace_root(repo_root))
        .collect()
}

/// Return only the [`WorkspaceRoot`] entries from [`WORKSPACES`] whose root currently holds
/// a `Cargo.toml` manifest.
#[must_use]
pub fn present_workspace_roots(repo_root: &Path) -> Vec<WorkspaceRoot> {
    workspace_roots(repo_root)
        .into_iter()
        .filter(WorkspaceRoot::has_manifest)
        .collect()
}

/// Return the exempt thin-binary link package names from the `Link` row(s) of [`WORKSPACES`].
#[must_use]
pub fn link_packages() -> Vec<&'static str> {
    let mut out = Vec::new();
    for row in WORKSPACES {
        if row.kind == WorkspaceKind::Link {
            out.extend_from_slice(row.link_packages);
        }
    }
    out
}

/// Single allowed justification kind for an entry in [`FORBIDDEN_CORE_PACKAGES`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ForbiddenReason {
    /// Package belongs in the named annex workspace's lockfile (`annex = "<workspace>"`).
    Annex(&'static str),
    /// Package belongs to a route researched and not taken (`route = "<route id>"`, `registries/routes_not_taken.toml`).
    Route(&'static str),
    /// Package belongs to a recorded non-goal (`nongoal = "<NONGOAL-id>"`, `registries/nongoals.toml`).
    Nongoal(&'static str),
}

impl ForbiddenReason {
    /// Render in canonical `kind = "id"` notation cited by `core.forbidden-package` findings.
    #[must_use]
    pub fn format_tag(self) -> String {
        match self {
            Self::Annex(ws) => format!("annex = \"{ws}\""),
            Self::Route(id) => format!("route = \"{id}\""),
            Self::Nongoal(id) => format!("nongoal = \"{id}\""),
        }
    }

    #[must_use]
    pub const fn identifier(self) -> &'static str {
        match self {
            Self::Annex(s) | Self::Route(s) | Self::Nongoal(s) => s,
        }
    }
}

impl fmt::Display for ForbiddenReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.format_tag())
    }
}

/// One entry in the workspace table's core forbidden-package list (`N34p4-02`).
///
/// Every entry carries `{ package, why }`, where `why` is `Some(ForbiddenReason)` naming
/// exactly one of `annex`, `route`, or `nongoal`. A `None` reason is rejected by
/// [`validate_forbidden_list`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ForbiddenPackage {
    /// Exact crate name or prefix pattern ending in `*` (e.g. `"burn*"`, `"fsym-*"`).
    pub package: &'static str,
    /// Justification pointing to an annex workspace, a routes-not-taken row, or a non-goal record.
    pub why: Option<ForbiddenReason>,
}

impl ForbiddenPackage {
    #[must_use]
    pub const fn annex(package: &'static str, workspace: &'static str) -> Self {
        Self {
            package,
            why: Some(ForbiddenReason::Annex(workspace)),
        }
    }

    #[must_use]
    pub const fn route(package: &'static str, route_id: &'static str) -> Self {
        Self {
            package,
            why: Some(ForbiddenReason::Route(route_id)),
        }
    }

    #[must_use]
    pub const fn nongoal(package: &'static str, nongoal_id: &'static str) -> Self {
        Self {
            package,
            why: Some(ForbiddenReason::Nongoal(nongoal_id)),
        }
    }
}

/// Single authoritative list of packages the core workspace `Cargo.lock` and resolved graph
/// must never contain (`N34p4-02`, `N21a-01`, `N21c-24`, `N00-23`).
///
/// Strictly sorted by `package` for D0 determinism. Every entry names one `annex`, `route`,
/// or `nongoal` justification. Downstream beads (`N21c-24`, `N00-23`) append rows here and
/// keep no second deny list anywhere.
pub const FORBIDDEN_CORE_PACKAGES: &[ForbiddenPackage] = &[
    ForbiddenPackage::route("ash", "ROUTE-SPARK-WGPU"),
    ForbiddenPackage::annex("block", "spark-metal"),
    ForbiddenPackage::route("burn*", "ROUTE-SPARK-RUSTGPU-BURN"),
    ForbiddenPackage::annex("cpython", "bindings"),
    ForbiddenPackage::route("cubecl*", "ROUTE-SPARK-RUSTGPU-BURN"),
    ForbiddenPackage::annex("cudarc", "spark-cuda"),
    ForbiddenPackage::annex("fsym-*", "derive"),
    ForbiddenPackage::annex("ft-kernel-metal", "spark-metal"),
    ForbiddenPackage::annex("metal", "spark-metal"),
    ForbiddenPackage::annex("numpy", "bindings"),
    ForbiddenPackage::annex("objc2-metal", "spark-metal"),
    ForbiddenPackage::route("opencl3", "ROUTE-SPARK-WGPU"),
    ForbiddenPackage::annex("pyo3", "bindings"),
    ForbiddenPackage::annex("pyo3-*", "bindings"),
    ForbiddenPackage::annex("rustpython*", "bindings"),
    ForbiddenPackage::route("shader-slang*", "ROUTE-SPARK-SLANG"),
    ForbiddenPackage::route("slang-*", "ROUTE-SPARK-SLANG"),
    ForbiddenPackage::route("spirv-std", "ROUTE-SPARK-RUSTGPU-BURN"),
    ForbiddenPackage::route("vulkano", "ROUTE-SPARK-WGPU"),
    ForbiddenPackage::route("wgpu", "ROUTE-SPARK-WGPU"),
    ForbiddenPackage::route("wgpu-*", "ROUTE-SPARK-WGPU"),
];

/// Repository-relative directory prefixes whose crates must never appear in the core
/// workspace's resolved dependency graph (`N34p4-02`).
pub const FORBIDDEN_CORE_DIR_PREFIXES: &[&str] = &["apps", "bindings", "derive", "spark"];

/// Match a package name against a forbidden pattern (exact name, or prefix match when
/// `pattern` ends with `*`).
///
/// Example: `"burn*"` matches `"burn"` and `"burn-core"`, and does not match `"sunburn"`.
#[must_use]
pub fn package_pattern_matches(pattern: &str, pkg_name: &str) -> bool {
    if let Some(prefix) = pattern.strip_suffix('*') {
        !prefix.is_empty() && pkg_name.starts_with(prefix)
    } else {
        pkg_name == pattern
    }
}

/// Look up the first matching row in [`FORBIDDEN_CORE_PACKAGES`] for `pkg_name`.
#[must_use]
pub fn match_forbidden_core_package(pkg_name: &str) -> Option<&'static ForbiddenPackage> {
    match_forbidden_in(FORBIDDEN_CORE_PACKAGES, pkg_name)
}

/// Look up the first matching row in an explicit forbidden-package slice.
#[must_use]
pub fn match_forbidden_in<'a>(
    entries: &'a [ForbiddenPackage],
    pkg_name: &str,
) -> Option<&'a ForbiddenPackage> {
    entries
        .iter()
        .find(|entry| package_pattern_matches(entry.package, pkg_name))
}

/// Check whether a repository-relative manifest or crate directory lies under one of the
/// forbidden core boundary directories (`spark/`, `apps/`, `derive/`, `bindings/`).
///
/// Returns the matching workspace reason (`annex = "<workspace>"`) when forbidden.
#[must_use]
pub fn match_forbidden_core_manifest_dir(rel_path: &str) -> Option<ForbiddenReason> {
    let trimmed = rel_path.trim_matches('/');
    if trimmed.is_empty() {
        return None;
    }

    // Prefer the most specific non-core row in WORKSPACES (e.g. spark/frob-spark-cuda -> spark-cuda).
    let mut best: Option<(usize, &'static str)> = None;
    for row in WORKSPACES {
        if row.kind == WorkspaceKind::Core || row.root_dir.is_empty() {
            continue;
        }
        if matches_dir_prefix(trimmed, row.root_dir)
            && best.is_none_or(|(best_len, _)| row.root_dir.len() > best_len)
        {
            best = Some((row.root_dir.len(), row.name));
        }
    }
    if let Some((_, ws_name)) = best {
        // `worker` is a retarget workspace rather than a forbidden boundary directory unless
        // under one of spark/, apps/, derive/, bindings/.
        if FORBIDDEN_CORE_DIR_PREFIXES
            .iter()
            .any(|dir| matches_dir_prefix(trimmed, dir))
        {
            return Some(ForbiddenReason::Annex(ws_name));
        }
    }

    for &dir in FORBIDDEN_CORE_DIR_PREFIXES {
        if matches_dir_prefix(trimmed, dir) {
            return Some(ForbiddenReason::Annex(dir));
        }
    }
    None
}

fn matches_dir_prefix(rel_path: &str, prefix: &str) -> bool {
    rel_path == prefix || rel_path.starts_with(&format!("{prefix}/"))
}

/// Validate a forbidden-package table for structural hygiene, sort order, reason presence,
/// annex workspace existence, and cross-checks against `routes_not_taken.toml` and
/// `nongoals.toml` when provided.
pub fn validate_forbidden_list(
    entries: &[ForbiddenPackage],
    workspaces: &[WorkspaceRow],
    routes_toml: Option<&[u8]>,
    nongoals_toml: Option<&[u8]>,
) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();

    let annex_names: BTreeSet<&str> = workspaces
        .iter()
        .filter(|w| w.kind == WorkspaceKind::Annex)
        .map(|w| w.name)
        .collect();

    let route_ids = match routes_toml {
        Some(bytes) => match parse_registry_ids(bytes, "route", ROUTES_NOT_TAKEN_PATH) {
            Ok(ids) => Some(ids),
            Err(err) => {
                errors.push(err);
                None
            }
        },
        None => None,
    };

    let nongoal_ids = match nongoals_toml {
        Some(bytes) => match parse_registry_ids(bytes, "nongoal", NONGOALS_PATH) {
            Ok(ids) => Some(ids),
            Err(err) => {
                errors.push(err);
                None
            }
        },
        None => None,
    };

    let mut prev_pkg: Option<&str> = None;
    let mut seen_pkgs: BTreeSet<&str> = BTreeSet::new();

    for entry in entries {
        let pkg = entry.package;
        if pkg.trim().is_empty() {
            errors.push("forbidden-package entry has empty `package` pattern".to_owned());
            continue;
        }
        if let Some(prefix) = pkg.strip_suffix('*') {
            if prefix.is_empty() || prefix.contains('*') {
                errors.push(format!(
                    "forbidden-package pattern `{pkg}` must be a non-empty prefix with at most one trailing `*`"
                ));
            }
        } else if pkg.contains('*') {
            errors.push(format!(
                "forbidden-package pattern `{pkg}` contains `*` outside the trailing position"
            ));
        }

        if !seen_pkgs.insert(pkg) {
            errors.push(format!("duplicate forbidden-package entry `{pkg}`"));
        }
        if let Some(prev) = prev_pkg
            && pkg < prev
        {
            errors.push(format!(
                "forbidden-package list is not sorted by package name: `{pkg}` follows `{prev}`"
            ));
        }
        prev_pkg = Some(pkg);

        match entry.why {
            None => errors.push(format!(
                "forbidden-package entry `{pkg}` has no reason (must specify `annex`, `route`, or `nongoal`)"
            )),
            Some(ForbiddenReason::Annex(ws)) => {
                if ws.trim().is_empty() {
                    errors.push(format!(
                        "forbidden-package entry `{pkg}` has empty `annex` workspace name"
                    ));
                } else if !annex_names.contains(ws) {
                    errors.push(format!(
                        "forbidden-package entry `{pkg}` names unknown annex workspace `{ws}`"
                    ));
                }
            }
            Some(ForbiddenReason::Route(id)) => {
                if id.trim().is_empty() {
                    errors.push(format!(
                        "forbidden-package entry `{pkg}` has empty `route` identifier"
                    ));
                } else if let Some(ref known) = route_ids
                    && !known.contains(id)
                {
                    errors.push(format!(
                        "forbidden-package entry `{pkg}` cites route `{id}` absent from {ROUTES_NOT_TAKEN_PATH}"
                    ));
                }
            }
            Some(ForbiddenReason::Nongoal(id)) => {
                if id.trim().is_empty() {
                    errors.push(format!(
                        "forbidden-package entry `{pkg}` has empty `nongoal` identifier"
                    ));
                } else if let Some(ref known) = nongoal_ids
                    && !known.contains(id)
                {
                    errors.push(format!(
                        "forbidden-package entry `{pkg}` cites nongoal `{id}` absent from {NONGOALS_PATH}"
                    ));
                }
            }
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Validate [`FORBIDDEN_CORE_PACKAGES`] against [`WORKSPACES`] and, when present on disk
/// under `repo_root`, against `registries/routes_not_taken.toml` and `registries/nongoals.toml`.
pub fn validate_workspace_table_against_repo(repo_root: &Path) -> Result<(), Vec<String>> {
    let routes_path = repo_root.join(ROUTES_NOT_TAKEN_PATH);
    let routes_bytes = if routes_path.is_file() {
        Some(fs::read(&routes_path).map_err(|e| {
            vec![format!("failed reading {}: {e}", routes_path.display())]
        })?)
    } else {
        None
    };

    let nongoals_path = repo_root.join(NONGOALS_PATH);
    let nongoals_bytes = if nongoals_path.is_file() {
        Some(fs::read(&nongoals_path).map_err(|e| {
            vec![format!("failed reading {}: {e}", nongoals_path.display())]
        })?)
    } else {
        None
    };

    validate_forbidden_list(
        FORBIDDEN_CORE_PACKAGES,
        WORKSPACES,
        routes_bytes.as_deref(),
        nongoals_bytes.as_deref(),
    )
}

fn parse_registry_ids(
    bytes: &[u8],
    array_key: &str,
    label: &str,
) -> Result<BTreeSet<String>, String> {
    let root = toml::parse(bytes)
        .map_err(|e| format!("{label}: failed to parse strict TOML: {e}"))?;
    let mut ids = BTreeSet::new();
    if let Some(Value::Array(items)) = root.get(array_key) {
        for (idx, sp) in items.iter().enumerate() {
            let Some(tbl) = sp.value.as_table() else {
                return Err(format!("{label}: `{array_key}[{idx}]` must be a table"));
            };
            let Some(id) = tbl.get("id").and_then(Value::as_str) else {
                return Err(format!(
                    "{label}: `{array_key}[{idx}]` missing string `id` field"
                ));
            };
            if id.trim().is_empty() {
                return Err(format!("{label}: `{array_key}[{idx}]` has empty `id`"));
            }
            ids.insert(id.to_owned());
        }
    }
    Ok(ids)
}

/// One documented top-level directory or file group in the FrankenRobotics repository layout
/// (`N34p4-02`, §34.1).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LayoutRule {
    pub path: &'static str,
    pub is_rust: bool,
    pub enforced_by: &'static str,
    pub summary: &'static str,
}

/// Canonical repository layout and directory-rule ownership table (`N34p4-02`).
pub const REPOSITORY_LAYOUT: &[LayoutRule] = &[
    LayoutRule {
        path: "Cargo.toml",
        is_rust: true,
        enforced_by: "check-layers, check-deps, check-stack",
        summary: "core workspace root; owns the one governing rust-toolchain.toml (T) and franken-stack.lock",
    },
    LayoutRule {
        path: "crates/",
        is_rust: true,
        enforced_by: "check-layers, check-deps, check-unsafe, check-contracts",
        summary: "core workspace crates (unsafe denied, Franken-only direct runtime deps); crates/frob-cert also builds standalone on stable",
    },
    LayoutRule {
        path: "surface/frob-mcp/",
        is_rust: true,
        enforced_by: "check-layers, check-deps",
        summary: "MCP surface server; ordinary member of the core workspace unless KILL-MCP fires (N32a-06)",
    },
    LayoutRule {
        path: "spark/frob-spark-cuda/",
        is_rust: true,
        enforced_by: "check-deps, check-unsafe, check-stack",
        summary: "CUDA annex workspace with its own Cargo.lock, cudarc, and registered unsafe capsules",
    },
    LayoutRule {
        path: "spark/frob-spark-metal/",
        is_rust: true,
        enforced_by: "check-deps, check-unsafe, check-stack",
        summary: "Metal annex workspace with its own Cargo.lock and ft-kernel-metal gateway",
    },
    LayoutRule {
        path: "apps/",
        is_rust: true,
        enforced_by: "check-layers, check-deps, check-stack",
        summary: "link workspace linking core and annexes into thin frob and frobd binaries",
    },
    LayoutRule {
        path: "worker/",
        is_rust: true,
        enforced_by: "check-stack",
        summary: "retarget workspace manifest for the frob-fsim worker binary (optional fallback toolchain only if recorded in franken-stack.lock)",
    },
    LayoutRule {
        path: "derive/frob-derive/",
        is_rust: true,
        enforced_by: "check-deps, check-stack",
        summary: "build-time derivation annex workspace with FrankenSymPy (fsym-*)",
    },
    LayoutRule {
        path: "bindings/frob-gymnasium/",
        is_rust: true,
        enforced_by: "check-deps, check-stack",
        summary: "Python/Gymnasium annex workspace holding pyo3-class bindings",
    },
    LayoutRule {
        path: "lean/",
        is_rust: false,
        enforced_by: "lake",
        summary: "Lean 4 Lake project with its own toolchain; build outputs are never committed",
    },
    LayoutRule {
        path: "viewer/",
        is_rust: false,
        enforced_by: "viewer packaging",
        summary: "Three.js viewer and HTML export (non-Rust)",
    },
    LayoutRule {
        path: "kernels/",
        is_rust: false,
        enforced_by: "check-generated",
        summary: "shared-dialect kernel sources and generated arithmetic with digests",
    },
    LayoutRule {
        path: "registries/",
        is_rust: false,
        enforced_by: "check-generated, check-contracts, frob-conformance",
        summary: "strict-TOML workspace registries (verbs, refusals, kernels, gates, formats, upstream, lifts, capsules, workloads, cards/)",
    },
    LayoutRule {
        path: "schemas/",
        is_rust: false,
        enforced_by: "check-generated",
        summary: "generated from formats.toml and verbs.toml; never hand-edited",
    },
    LayoutRule {
        path: "receipts/",
        is_rust: false,
        enforced_by: "check-stack",
        summary: "resolution receipts named by the BLAKE3 hash of their canonical bytes",
    },
    LayoutRule {
        path: "tests/fixtures/oracle/",
        is_rust: false,
        enforced_by: "frob-conformance",
        summary: "pinned oracle captures with capture records; never regenerated from the candidate",
    },
    LayoutRule {
        path: "docs/delivery/",
        is_rust: false,
        enforced_by: "delivery review",
        summary: "delivery log and per-bead logs; planning text never records delivery state",
    },
];

/// Deterministically find every `rust-toolchain.toml` under `repo_root`, skipping VCS,
/// build artifacts, scratch trees, and non-Rust subprojects (`lean/`, `viewer/`).
#[must_use]
pub fn find_toolchain_files(repo_root: &Path) -> Vec<String> {
    let mut found = Vec::new();
    walk_toolchain_files(repo_root, repo_root, 0, &mut found);
    found.sort();
    found
}

fn walk_toolchain_files(repo_root: &Path, dir: &Path, depth: usize, out: &mut Vec<String>) {
    if depth > 6 {
        return;
    }
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    let mut paths: Vec<_> = entries.filter_map(|e| e.ok().map(|x| x.path())).collect();
    paths.sort();

    for path in paths {
        let Some(name) = path.file_name().and_then(|s| s.to_str()) else {
            continue;
        };
        if path.is_dir() {
            if matches!(
                name,
                ".git"
                    | "target"
                    | ".beads"
                    | ".ruff_cache"
                    | ".ee"
                    | ".local-implementation"
                    | ".ntm"
                    | "repro"
                    | "planning"
                    | "node_modules"
                    | "lean"
                    | "viewer"
            ) {
                continue;
            }
            walk_toolchain_files(repo_root, &path, depth + 1, out);
        } else if name == ROOT_TOOLCHAIN_FILE
            && let Ok(rel) = path.strip_prefix(repo_root)
        {
            out.push(normalize_rel_path(rel));
        }
    }
}

/// Evaluate the one-toolchain-file rule (`N34p4-02`, item 4):
/// `rust-toolchain.toml` may exist ONLY at the root, in `crates/frob-cert/`, and in
/// `worker/` only when `worker_fallback_active` is true.
///
/// Returns `Ok(found)` when compliant (and root `rust-toolchain.toml` is present), or
/// `Err(violations)` listing any missing root file or stray toolchain paths.
pub fn check_toolchain_files(
    repo_root: &Path,
    worker_fallback_active: bool,
) -> Result<Vec<String>, Vec<String>> {
    let found = find_toolchain_files(repo_root);
    let mut violations = Vec::new();

    if !found.iter().any(|p| p == ROOT_TOOLCHAIN_FILE) {
        violations.push(format!(
            "missing governing root toolchain file `{ROOT_TOOLCHAIN_FILE}`"
        ));
    }

    for path in &found {
        let allowed = path == ROOT_TOOLCHAIN_FILE
            || path == CERT_TOOLCHAIN_FILE
            || (worker_fallback_active && path == WORKER_TOOLCHAIN_FILE);
        if !allowed {
            violations.push(format!(
                "stray toolchain file `{path}` (allowed only at `{ROOT_TOOLCHAIN_FILE}`, `{CERT_TOOLCHAIN_FILE}`, and `{WORKER_TOOLCHAIN_FILE}` when worker fallback is active)"
            ));
        }
    }

    if violations.is_empty() {
        Ok(found)
    } else {
        Err(violations)
    }
}
