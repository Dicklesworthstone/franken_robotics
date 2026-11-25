//! `xtask check-deps`: rules L-2, L-4, L-5, L-9, proof-boundary edges (bead BD-003),
//! and suite deny rules + posture register validation (bead N33c-09).
//!
//! Enforces ordered steps over declared (`--no-deps`), resolved, and `--all-features`
//! resolved `cargo metadata` reads, committed `Cargo.lock` files, `franken-stack.lock`
//! (`stack::lock::load`, N33a-03), `registries/siblings.toml` (`frob.siblings/1`, N33c-09),
//! and posture-to-graph rules (`stack::posture::edge_allowed`, N33a-01):
//! 1. L-2 (`fsim.volatile-edge`, `L-2.fs-qty-direct`, `L-2.airlock-reexport`)
//! 2. L-4 (`L-4.foundation-confined`, `L-4.edge-spec`) via `stack::posture::edge_allowed`
//! 3. L-5 (`L-5.third-party`, `L-5.serde-outside-boundary`, `L-5.serde-not-exact`)
//! 4. L-5 (`L-5.forbidden`, `L-5.python-outside-bindings`, core `links`/`cc`/`cmake`/`bindgen`)
//! 5. L-9 (`L-9.cert-has-deps`, `L-9.cert-dependent`)
//! 6. `edge.forbidden` proof-boundary table (only when layers permit the edge)
//! 7. Suite deny rules (`asup.v0_5_0`, `ftui.executor-feature`, `suite.forbidden-package`,
//!    `suite.egg`) across default + `--all-features` resolved graphs and `Cargo.lock` files
//! 8. Closure `facts` (total/registry package counts and transitive forbidden hits with one path)
//!
//! Determinism class D0: findings and facts are sorted deterministically. Compiles nothing
//! and touches no network.

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

use frob_core::registry::toml::{self, Fields, Table, Value};

use crate::check_deps_airlock::{
    self, BinaryAsupFact, RULE_ASUP_DUPLICATE as AIRLOCK_RULE_ASUP_DUPLICATE,
    RULE_FSIM_DEFAULT_SET as AIRLOCK_RULE_FSIM_DEFAULT_SET,
    RULE_FSIM_GROUP_NOT_ALLOWED as AIRLOCK_RULE_FSIM_GROUP_NOT_ALLOWED,
    RULE_FSIM_OPTIONAL_DEFAULT_ON as AIRLOCK_RULE_FSIM_OPTIONAL_DEFAULT_ON,
    RULE_RESOLVE_ALL_FEATURES as AIRLOCK_RULE_RESOLVE_ALL_FEATURES,
    RULE_TIER_DEFAULT_ON as AIRLOCK_RULE_TIER_DEFAULT_ON,
    RULE_TIER_UNNAMED as AIRLOCK_RULE_TIER_UNNAMED,
};
use crate::check_layers::{GraphEdge, check_edges};
use crate::cli::{CheckContext, CheckError, CheckOutcome};
use crate::crate_map::{CRATE_MAP, CrateRow, Finding, Layer, lookup_in};
use crate::metadata::{
    DeclaredDep, DepKind, MetadataLoadError, PackageMeta, ResolvedPackage, ResolvedWorkspace,
    WorkspaceKind, WorkspaceRoot, load_declared_metadata, load_resolved_metadata,
    load_resolved_metadata_all_features, normalize_rel_path,
};

pub use crate::check_deps_airlock::{
    AirlockGroupRecord, AsupInstance, BinaryAsupFact as BinaryAsupersyncFact,
    RULE_ASUP_DUPLICATE, RULE_FSIM_DEFAULT_SET, RULE_FSIM_GROUP_NOT_ALLOWED,
    RULE_FSIM_OPTIONAL_DEFAULT_ON, RULE_RESOLVE_ALL_FEATURES, RULE_TIER_DEFAULT_ON,
    RULE_TIER_UNNAMED, check_airlock_manifest_file, load_airlock_groups,
    parse_airlock_groups_bytes,
};
use crate::report::FactValue;
use crate::stack::lock::{self, Foundation, RelPath, StackLock};
use crate::stack::posture::{
    BEDROCK_DIRECT_CRATES, DepSpec, EdgeDecision, EdgeDenial, EdgeKind, Posture, check_edge,
    edge_allowed, is_exact_semver,
};
use crate::workspaces::{self, ForbiddenReason};

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
/// Rule identifier for any `asupersync` package on the `0.5.*` line in a resolved graph or lockfile (N33c-09).
pub const RULE_ASUP_V0_5_0: &str = "asup.v0_5_0";
/// Rule identifier for `frankentui` resolving `asupersync-executor` or used outside `frob-tui` (N33c-09).
pub const RULE_FTUI_EXECUTOR_FEATURE: &str = "ftui.executor-feature";
/// Rule identifier for any package from the eight never-dependency siblings in a resolved graph (N33c-09).
pub const RULE_SUITE_FORBIDDEN_PACKAGE: &str = "suite.forbidden-package";
/// Rule identifier for the third-party `egg` crate in any resolved graph (arrives via frankenjax, N33c-09).
pub const RULE_SUITE_EGG: &str = "suite.egg";
/// Rule identifier for any package on the workspace table's core forbidden-package list
/// appearing in the core `Cargo.lock` or resolved dependency graph (`N34p4-02`).
pub const RULE_CORE_FORBIDDEN_PACKAGE: &str = "core.forbidden-package";

/// Complete list of finding rule identifiers owned by `check-deps` (BD-003 + N33c-09 + N34p4-02 + N34-03).
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
    RULE_ASUP_V0_5_0,
    RULE_FTUI_EXECUTOR_FEATURE,
    RULE_SUITE_FORBIDDEN_PACKAGE,
    RULE_SUITE_EGG,
    RULE_CORE_FORBIDDEN_PACKAGE,
    AIRLOCK_RULE_ASUP_DUPLICATE,
    AIRLOCK_RULE_FSIM_GROUP_NOT_ALLOWED,
    AIRLOCK_RULE_FSIM_DEFAULT_SET,
    AIRLOCK_RULE_FSIM_OPTIONAL_DEFAULT_ON,
    AIRLOCK_RULE_TIER_DEFAULT_ON,
    AIRLOCK_RULE_TIER_UNNAMED,
    AIRLOCK_RULE_RESOLVE_ALL_FEATURES,
];

/// Schema identifier for the shared sibling register (`N33c-07` / `N33c-09`).
pub const SIBLINGS_SCHEMA: &str = "frob.siblings/1";
/// Repository-relative path of the sibling register.
pub const SIBLINGS_REL_PATH: &str = "registries/siblings.toml";
/// Forbidden optional feature on `ftui-runtime` / `frankentui` crates (pulls `asupersync =0.5.0`).
pub const FTUI_FORBIDDEN_FEATURE: &str = "asupersync-executor";

/// Specification of one never-dependency sibling enforced by `suite.forbidden-package` (N33c-09).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ForbiddenSiblingSpec {
    pub name: &'static str,
    pub rel_path: &'static str,
    /// Package-name patterns matching registry-consumed crates of this sibling.
    pub pkg_patterns: &'static [&'static str],
}

/// Authoritative table of the eight never-dependency siblings (`N33c-09`).
pub const NEVER_DEPENDENCY_SIBLINGS: &[ForbiddenSiblingSpec] = &[
    ForbiddenSiblingSpec {
        name: "frankenjax",
        rel_path: "../frankenjax",
        pkg_patterns: &["frankenjax", "frankenjax-*", "fj-*"],
    },
    ForbiddenSiblingSpec {
        name: "franken_threed",
        rel_path: "../franken_threed",
        pkg_patterns: &["franken_threed", "franken-threed", "f3d-*"],
    },
    ForbiddenSiblingSpec {
        name: "franken_textiles",
        rel_path: "../franken_textiles",
        pkg_patterns: &["franken_textiles", "franken-textiles", "ftx-*"],
    },
    ForbiddenSiblingSpec {
        name: "franken_firmware",
        rel_path: "../franken_firmware",
        pkg_patterns: &[
            "franken_firmware",
            "franken-firmware",
            "franken-alloc-probe",
            "franken-atomic",
            "franken-baseline",
            "franken-board-profile",
            "franken-clock",
            "franken-codec",
            "franken-control",
            "franken-dsp",
            "franken-hal",
            "franken-hal-linux",
            "franken-journal",
            "franken-platform",
            "franken-platform-linux",
            "franken-session",
            "franken-speaker",
            "franken-stream",
            "franken-test-support",
            "franken-update",
        ],
    },
    ForbiddenSiblingSpec {
        name: "franken_networkx",
        rel_path: "../franken_networkx",
        pkg_patterns: &["franken_networkx", "franken-networkx", "fnx-*"],
    },
    ForbiddenSiblingSpec {
        name: "franken_markdown",
        rel_path: "../franken_markdown",
        pkg_patterns: &["franken_markdown", "franken-markdown", "fmd", "fmd-*"],
    },
    ForbiddenSiblingSpec {
        name: "frankenmermaid",
        rel_path: "../frankenmermaid",
        pkg_patterns: &[
            "frankenmermaid",
            "frankenmermaid-*",
            "fm-cli",
            "fm-core",
            "fm-layout",
            "fm-parser",
            "fm-regression-harness",
            "fm-render-canvas",
            "fm-render-svg",
            "fm-render-term",
            "fm-wasm",
        ],
    },
    ForbiddenSiblingSpec {
        name: "franken_surveillance_system",
        rel_path: "../franken_surveillance_system",
        pkg_patterns: &[
            "franken_surveillance_system",
            "franken-surveillance-system",
            "fss-*",
        ],
    },
];

/// Cargo dependency policy recorded for a sibling row in `registries/siblings.toml` (`N33c-09`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CargoPolicy {
    /// Policy columns intentionally empty (foundation posture lives in `franken-stack.lock`).
    Empty,
    Allowed,
    DevOnly,
    Never,
}

impl CargoPolicy {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Empty => "",
            Self::Allowed => "allowed",
            Self::DevOnly => "dev-only",
            Self::Never => "never",
        }
    }

    pub fn parse(raw: &str, row: &str) -> Result<Self, SiblingsLoadError> {
        match raw {
            "" => Ok(Self::Empty),
            "allowed" => Ok(Self::Allowed),
            "dev-only" => Ok(Self::DevOnly),
            "never" => Ok(Self::Never),
            other => Err(SiblingsLoadError::new(format!(
                "registries/siblings.toml row `{row}`: invalid `cargo` policy `{other}` (expected \"\", \"allowed\", \"dev-only\", or \"never\")"
            ))),
        }
    }
}

/// One validated `[[sibling]]` row from `registries/siblings.toml` (`frob.siblings/1`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SiblingPolicyRow {
    pub name: String,
    pub path: String,
    pub survey: bool,
    pub posture: Vec<Posture>,
    pub cargo: CargoPolicy,
    pub allowed_in: Vec<String>,
    pub forbidden_features: Vec<String>,
    pub reason: String,
    pub fallback: String,
}

impl SiblingPolicyRow {
    #[must_use]
    pub fn has_policy(&self) -> bool {
        !self.posture.is_empty()
            || self.cargo != CargoPolicy::Empty
            || !self.allowed_in.is_empty()
            || !self.forbidden_features.is_empty()
            || !self.reason.is_empty()
            || !self.fallback.is_empty()
    }
}

/// Parsed and schema-validated `registries/siblings.toml` register (`frob.siblings/1`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SiblingsRegister {
    pub schema: String,
    pub siblings: Vec<SiblingPolicyRow>,
}

impl SiblingsRegister {
    #[must_use]
    pub fn find(&self, name: &str) -> Option<&SiblingPolicyRow> {
        self.siblings.iter().find(|r| r.name == name)
    }
}

/// Load or cross-validation error for `registries/siblings.toml`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SiblingsLoadError {
    pub message: String,
}

impl SiblingsLoadError {
    #[must_use]
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for SiblingsLoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for SiblingsLoadError {}

/// Load and strictly validate `registries/siblings.toml` from `path`.
pub fn load_siblings_register(path: &Path) -> Result<SiblingsRegister, SiblingsLoadError> {
    let bytes = fs::read(path).map_err(|e| {
        SiblingsLoadError::new(format!("failed to read `{}`: {e}", path.display()))
    })?;
    parse_siblings_register_bytes(&bytes, &path.display().to_string())
}

/// Strictly parse and validate `registries/siblings.toml` (`frob.siblings/1`) bytes using
/// `frob_core::registry::toml` (D0; zero I/O).
pub fn parse_siblings_register_bytes(
    bytes: &[u8],
    source_label: &str,
) -> Result<SiblingsRegister, SiblingsLoadError> {
    let root = toml::parse(bytes).map_err(|e| {
        SiblingsLoadError::new(format!("{source_label}: TOML parse error: {e}"))
    })?;
    let mut top = Fields::new(root, source_label);
    let raw_schema = top
        .opt_str("schema")
        .map_err(|e| SiblingsLoadError::new(format!("{source_label}: {e}")))?;
    let raw_siblings = top
        .opt_array("sibling")
        .map_err(|e| SiblingsLoadError::new(format!("{source_label}: {e}")))?;
    top.finish()
        .map_err(|e| SiblingsLoadError::new(format!("{source_label}: {e}")))?;

    let schema = raw_schema.ok_or_else(|| {
        SiblingsLoadError::new(format!(
            "{source_label}: missing required top-level `schema = \"{SIBLINGS_SCHEMA}\"`"
        ))
    })?;
    if schema != SIBLINGS_SCHEMA {
        return Err(SiblingsLoadError::new(format!(
            "{source_label}: expected schema `{SIBLINGS_SCHEMA}`, got `{schema}`"
        )));
    }

    let items = raw_siblings.ok_or_else(|| {
        SiblingsLoadError::new(format!(
            "{source_label}: missing required `[[sibling]]` array of tables"
        ))
    })?;

    let mut siblings = Vec::with_capacity(items.len());
    let mut seen_names = BTreeSet::new();
    for (idx, sp) in items.into_iter().enumerate() {
        let fallback_row = format!("sibling[{idx}]");
        let Value::Table(tbl) = sp.value else {
            return Err(SiblingsLoadError::new(format!(
                "{source_label}: {fallback_row} must be a table, found {}",
                sp.value.type_name()
            )));
        };
        let row = parse_sibling_row(tbl, source_label, &fallback_row)?;
        if !seen_names.insert(row.name.clone()) {
            return Err(SiblingsLoadError::new(format!(
                "{source_label}: duplicate sibling name `{}`",
                row.name
            )));
        }
        siblings.push(row);
    }

    Ok(SiblingsRegister { schema, siblings })
}

fn parse_sibling_row(
    table: Table,
    source_label: &str,
    fallback_row: &str,
) -> Result<SiblingPolicyRow, SiblingsLoadError> {
    let mut f = Fields::new(table, fallback_row);
    let raw_name = f
        .opt_str("name")
        .map_err(|e| SiblingsLoadError::new(format!("{source_label}: {e}")))?;
    let row_label = raw_name.as_deref().unwrap_or(fallback_row).to_owned();

    let raw_path = f
        .opt_str("path")
        .map_err(|e| SiblingsLoadError::new(format!("{source_label} ({row_label}): {e}")))?;
    let raw_survey = f
        .opt_bool("survey")
        .map_err(|e| SiblingsLoadError::new(format!("{source_label} ({row_label}): {e}")))?;
    let _raw_tc = f
        .opt_str("toolchain_file")
        .map_err(|e| SiblingsLoadError::new(format!("{source_label} ({row_label}): {e}")))?;
    let _raw_asup_manifest = f
        .opt_str("asupersync_manifest")
        .map_err(|e| SiblingsLoadError::new(format!("{source_label} ({row_label}): {e}")))?;
    let _raw_asup_key = f
        .opt_str("asupersync_key")
        .map_err(|e| SiblingsLoadError::new(format!("{source_label} ({row_label}): {e}")))?;
    let raw_watched = f
        .opt_table("watched")
        .map_err(|e| SiblingsLoadError::new(format!("{source_label} ({row_label}): {e}")))?;
    let raw_posture = f
        .opt_array("posture")
        .map_err(|e| SiblingsLoadError::new(format!("{source_label} ({row_label}): {e}")))?;
    let raw_cargo = f
        .opt_str("cargo")
        .map_err(|e| SiblingsLoadError::new(format!("{source_label} ({row_label}): {e}")))?;
    let raw_allowed_in = f
        .opt_array("allowed_in")
        .map_err(|e| SiblingsLoadError::new(format!("{source_label} ({row_label}): {e}")))?;
    let raw_forbidden_features = f
        .opt_array("forbidden_features")
        .map_err(|e| SiblingsLoadError::new(format!("{source_label} ({row_label}): {e}")))?;
    let raw_reason = f
        .opt_str("reason")
        .map_err(|e| SiblingsLoadError::new(format!("{source_label} ({row_label}): {e}")))?;
    let raw_fallback = f
        .opt_str("fallback")
        .map_err(|e| SiblingsLoadError::new(format!("{source_label} ({row_label}): {e}")))?;

    // Finish Fields BEFORE domain validation so unknown keys fail cleanly and drop never trips.
    f.finish()
        .map_err(|e| SiblingsLoadError::new(format!("{source_label} ({row_label}): {e}")))?;

    let name = raw_name
        .filter(|s| !s.trim().is_empty())
        .ok_or_else(|| {
            SiblingsLoadError::new(format!(
                "{source_label}: {fallback_row} missing non-empty `name`"
            ))
        })?;
    let path_str = raw_path
        .filter(|s| !s.trim().is_empty())
        .ok_or_else(|| {
            SiblingsLoadError::new(format!(
                "{source_label}: row `{name}` missing non-empty `path`"
            ))
        })?;
    RelPath::parse(&path_str, &name).map_err(|e| {
        SiblingsLoadError::new(format!("{source_label}: row `{name}` invalid `path`: {e}"))
    })?;
    let survey = raw_survey.ok_or_else(|| {
        SiblingsLoadError::new(format!(
            "{source_label}: row `{name}` missing required boolean `survey`"
        ))
    })?;

    if let Some(watched_tbl) = raw_watched {
        for (wk, wsp) in watched_tbl.into_inner() {
            if !matches!(wsp.value, Value::Str(_)) {
                return Err(SiblingsLoadError::new(format!(
                    "{source_label}: row `{name}` `watched.{wk}` must be a string, found {}",
                    wsp.value.type_name()
                )));
            }
        }
    }

    let mut posture = Vec::new();
    if let Some(items) = raw_posture {
        for sp in items {
            let Value::Str(s) = sp.value else {
                return Err(SiblingsLoadError::new(format!(
                    "{source_label}: row `{name}` `posture` elements must be strings, found {}",
                    sp.value.type_name()
                )));
            };
            let parsed = Posture::parse(&s, &name).map_err(|e| {
                SiblingsLoadError::new(format!("{source_label}: {e}"))
            })?;
            posture.push(parsed);
        }
    }

    let cargo = CargoPolicy::parse(raw_cargo.as_deref().unwrap_or(""), &name)?;
    let allowed_in = parse_sibling_str_vec(raw_allowed_in, source_label, &name, "allowed_in")?;
    let forbidden_features = parse_sibling_str_vec(
        raw_forbidden_features,
        source_label,
        &name,
        "forbidden_features",
    )?;
    let reason = raw_reason.unwrap_or_default();
    let fallback = raw_fallback.unwrap_or_default();

    let policy_active = !posture.is_empty()
        || cargo != CargoPolicy::Empty
        || !allowed_in.is_empty()
        || !forbidden_features.is_empty()
        || !fallback.is_empty()
        || !survey;

    if policy_active {
        if cargo == CargoPolicy::Empty {
            return Err(SiblingsLoadError::new(format!(
                "{source_label}: row `{name}` sets policy fields or `survey = false` but leaves `cargo` empty"
            )));
        }
        if reason.trim().is_empty() {
            return Err(SiblingsLoadError::new(format!(
                "{source_label}: row `{name}` is missing a non-empty `reason`"
            )));
        }
    }

    Ok(SiblingPolicyRow {
        name,
        path: path_str,
        survey,
        posture,
        cargo,
        allowed_in,
        forbidden_features,
        reason,
        fallback,
    })
}

fn parse_sibling_str_vec(
    items: Option<Vec<frob_core::registry::toml::Spanned<Value>>>,
    source_label: &str,
    row_name: &str,
    field: &str,
) -> Result<Vec<String>, SiblingsLoadError> {
    let Some(items) = items else {
        return Ok(Vec::new());
    };
    let mut out = Vec::with_capacity(items.len());
    for sp in items {
        let Value::Str(s) = sp.value else {
            return Err(SiblingsLoadError::new(format!(
                "{source_label}: row `{row_name}` `{field}` elements must be strings, found {}",
                sp.value.type_name()
            )));
        };
        if s.trim().is_empty() {
            return Err(SiblingsLoadError::new(format!(
                "{source_label}: row `{row_name}` `{field}` must not contain empty strings"
            )));
        }
        out.push(s);
    }
    Ok(out)
}

/// Cross-check populated policy rows in `registries/siblings.toml` against `franken-stack.lock`.
///
/// Any posture or Cargo-policy disagreement with the matching `[[foundation]]` row is a
/// load error explicitly naming both `registries/siblings.toml` and `franken-stack.lock` (`N33c-09`).
pub fn validate_siblings_against_stack_lock(
    register: &SiblingsRegister,
    stack_lock: &StackLock,
) -> Result<(), SiblingsLoadError> {
    for row in &register.siblings {
        if !row.has_policy() {
            continue;
        }
        let row_norm = normalize_pkg_ident(&row.name);
        let Some(foundation) = stack_lock
            .foundations
            .iter()
            .find(|f| f.name == row.name || normalize_pkg_ident(&f.name) == row_norm)
        else {
            continue;
        };

        let sib_postures: BTreeSet<Posture> = row.posture.iter().copied().collect();
        let lock_postures: BTreeSet<Posture> = foundation.uses.iter().map(|u| u.posture).collect();
        if sib_postures != lock_postures {
            let sib_list: Vec<&str> = sib_postures.iter().map(|p| p.as_str()).collect();
            let lock_list: Vec<&str> = lock_postures.iter().map(|p| p.as_str()).collect();
            return Err(SiblingsLoadError::new(format!(
                "posture disagreement for sibling `{}` (`{}`): `registries/siblings.toml` declares {:?} but `franken-stack.lock` declares {:?}",
                row.name, foundation.id, sib_list, lock_list
            )));
        }

        let lock_allows_normal_or_build = lock_postures.iter().any(|p| {
            matches!(
                p,
                Posture::Consume | Posture::ConsumePath | Posture::BuildTime
            )
        });
        let lock_allows_oracle = lock_postures.contains(&Posture::Oracle);
        let cargo_ok = match row.cargo {
            CargoPolicy::Empty => true,
            CargoPolicy::Allowed => lock_allows_normal_or_build,
            CargoPolicy::DevOnly => lock_allows_oracle && !lock_allows_normal_or_build,
            CargoPolicy::Never => !lock_allows_normal_or_build && !lock_allows_oracle,
        };
        if !cargo_ok {
            let lock_list: Vec<&str> = lock_postures.iter().map(|p| p.as_str()).collect();
            return Err(SiblingsLoadError::new(format!(
                "cargo policy disagreement for sibling `{}` (`{}`): `registries/siblings.toml` declares cargo = `{}` but `franken-stack.lock` declares postures {:?}",
                row.name,
                foundation.id,
                row.cargo.as_str(),
                lock_list
            )));
        }
    }
    Ok(())
}

/// Discover all present FrankenRobotics workspaces under `repo_root` using the
/// authoritative workspace table (`workspaces::WORKSPACES`, `N34p4-02`), deliberately
/// excluding `tests/admission/`.
#[must_use]
pub fn discover_workspace_roots(repo_root: &Path) -> Vec<WorkspaceRoot> {
    let mut roots = vec![WorkspaceRoot::core(repo_root)];
    let mut seen_rel = BTreeSet::new();
    seen_rel.insert(String::new());

    for ws in workspaces::workspace_roots(repo_root) {
        if ws.kind != WorkspaceKind::Core && ws.has_manifest() && seen_rel.insert(ws.rel_dir.clone())
        {
            roots.push(ws);
        }
    }

    // Fallback for synthetic fixtures placing a workspace Cargo.toml directly at
    // `spark/`, `derive/`, or `bindings/` when the per-annex subdirectory has none.
    for (name, dir) in [("spark", "spark"), ("derive", "derive"), ("bindings", "bindings")] {
        let prefix = format!("{dir}/");
        let has_nested = seen_rel.iter().any(|r| r.starts_with(&prefix));
        if !has_nested {
            let ws = WorkspaceRoot::annex(name, dir, repo_root.join(dir));
            if ws.has_manifest() && seen_rel.insert(ws.rel_dir.clone()) {
                roots.push(ws);
            }
        }
    }

    roots
}

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
    if ctx.roots.len() == 1 && ctx.roots[0].kind == WorkspaceKind::Core {
        let roots = discover_workspace_roots(&ctx.repo_root);
        check_deps(&ctx.repo_root, &roots)
    } else {
        check_deps(&ctx.repo_root, &ctx.roots)
    }
}

/// Run `check-deps` over `roots` reading `<repo_root>/franken-stack.lock` via `stack::lock::load`
/// and `<repo_root>/registries/siblings.toml` (when present) via `load_siblings_register`.
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
    let siblings_path = repo_root.join(SIBLINGS_REL_PATH);
    let siblings = if siblings_path.is_file() {
        let reg = load_siblings_register(&siblings_path)
            .map_err(|err| CheckError::Fault(err.to_string()))?;
        validate_siblings_against_stack_lock(&reg, stack_lock)
            .map_err(|err| CheckError::Fault(err.to_string()))?;
        Some(reg)
    } else {
        None
    };

    let mut findings = Vec::new();
    let mut checked_workspaces = Vec::new();
    let mut absent_workspaces = Vec::new();
    let mut ws_facts_map: BTreeMap<String, FactValue> = BTreeMap::new();
    let mut all_binary_facts: Vec<BinaryAsupFact> = Vec::new();
    let mut all_airlock_facts: BTreeMap<String, FactValue> = BTreeMap::new();
    let mut reported_lock_asup: BTreeSet<(String, String)> = BTreeSet::new();

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
        let resolved = match load_resolved_metadata(repo_root, ws) {
            Ok(r) => r,
            Err(MetadataLoadError::ResolutionConflict { detail }) => {
                // Conflicting exact pins (including behind optional features, which Cargo
                // resolves eagerly) surface as `resolve.all-features` rather than a tool fault.
                findings.push(check_deps_airlock::build_resolve_all_features_finding(
                    ws, &detail,
                ));
                checked_workspaces.push(ws.name.clone());
                continue;
            }
            Err(err) => {
                // If an orphaned `asupersync 0.5.*` or core forbidden-package entry in this
                // workspace's Cargo.lock caused `--locked` to reject the lockfile, still report
                // the ban directly rather than hiding it behind a lockfile usage error.
                let lock_file = ws.root_dir.join("Cargo.lock");
                let mut lock_findings = Vec::new();
                if lock_file.is_file() {
                    scan_single_lockfile_for_asup_0_5(
                        repo_root,
                        &lock_file,
                        &reported_lock_asup,
                        &mut lock_findings,
                    );
                    if ws.kind == WorkspaceKind::Core {
                        let empty_resolved = BTreeSet::new();
                        scan_core_lockfile_for_forbidden_packages(
                            repo_root,
                            &lock_file,
                            &empty_resolved,
                            &mut lock_findings,
                        );
                    }
                }
                if !lock_findings.is_empty() {
                    findings.extend(lock_findings);
                    checked_workspaces.push(ws.name.clone());
                    continue;
                }
                return Err(metadata_err_to_check_err(err));
            }
        };
        let resolved_all = match load_resolved_metadata_all_features(repo_root, ws) {
            Ok(ra) => Some(ra),
            Err(err) => {
                // Default resolution succeeded, so an `--all-features` resolution/lockfile
                // failure is a `resolve.all-features` finding (N34-03 rule 5).
                findings.push(check_deps_airlock::build_resolve_all_features_finding(
                    ws,
                    &err.to_string(),
                ));
                None
            }
        };
        checked_workspaces.push(ws.name.clone());

        evaluate_workspace(
            ws,
            &declared.members,
            &resolved,
            stack_lock,
            siblings.as_ref(),
            crate_map,
            proof_boundary_edges,
            &mut findings,
        );

        evaluate_suite_resolved_rules(
            ws,
            &declared.members,
            &resolved,
            stack_lock,
            siblings.as_ref(),
            &mut reported_lock_asup,
            &mut findings,
        );
        if let Some(ref ra) = resolved_all {
            evaluate_suite_resolved_rules(
                ws,
                &declared.members,
                ra,
                stack_lock,
                siblings.as_ref(),
                &mut reported_lock_asup,
                &mut findings,
            );
        }

        if ws.kind == WorkspaceKind::Core {
            evaluate_core_forbidden_packages(
                repo_root,
                ws,
                &declared.members,
                &resolved,
                resolved_all.as_ref().unwrap_or(&resolved),
                &mut findings,
            );
        }

        // N34-03 Step 9: One `asupersync` per binary (INV-SUITE-02).
        let bin_facts = check_deps_airlock::evaluate_binary_asupersync(
            ws,
            &declared.members,
            &resolved,
            resolved_all.as_ref(),
            &mut findings,
        );
        all_binary_facts.extend(bin_facts);

        // N34-03 Step 10: Airlock feature groups read dynamically from `crates/frob-fsim/contract.toml` (INV-SUITE-01).
        let airlock_map = check_deps_airlock::evaluate_airlock_groups(
            repo_root,
            &declared.members,
            &resolved,
            &mut findings,
        )
        .map_err(CheckError::Fault)?;
        let airlock_fact_obj: BTreeMap<String, FactValue> = airlock_map
            .into_iter()
            .map(|(crate_name, grps)| (crate_name, FactValue::str_array(grps)))
            .collect();
        all_airlock_facts.insert(ws.name.clone(), FactValue::Object(airlock_fact_obj.clone()));

        // N34-03 Step 11: Tier features off by default (L-12, INV-WS-04).
        check_deps_airlock::evaluate_tier_features(
            ws,
            &declared.members,
            &resolved,
            crate_map,
            &mut findings,
        );

        let mut ws_fact = build_workspace_closure_facts(&resolved);
        if let FactValue::Object(ref mut obj) = ws_fact {
            obj.insert(
                "airlock_groups".to_owned(),
                FactValue::Object(airlock_fact_obj),
            );
        }
        ws_facts_map.insert(ws.name.clone(), ws_fact);
    }

    scan_lockfiles_for_asup_0_5(repo_root, roots, &reported_lock_asup, &mut findings);

    findings.sort();
    findings.dedup();
    checked_workspaces.sort();
    absent_workspaces.sort();
    all_binary_facts.sort_by(|a, b| {
        a.workspace
            .cmp(&b.workspace)
            .then_with(|| a.binary.cmp(&b.binary))
            .then_with(|| a.package.cmp(&b.package))
    });

    let mut binaries_map: BTreeMap<String, FactValue> = BTreeMap::new();
    for bf in &all_binary_facts {
        let key = if bf.workspace == "core" {
            bf.binary.clone()
        } else {
            format!("{}/{}", bf.workspace, bf.binary)
        };
        binaries_map.insert(key, bf.to_fact_value());
    }

    let mut facts = BTreeMap::new();
    facts.insert(
        "airlock_groups".to_owned(),
        FactValue::Object(all_airlock_facts),
    );
    facts.insert("binaries".to_owned(), FactValue::Object(binaries_map));
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

#[allow(clippy::too_many_arguments)]
fn evaluate_workspace(
    ws: &WorkspaceRoot,
    members: &[PackageMeta],
    resolved: &ResolvedWorkspace,
    stack_lock: &StackLock,
    siblings: Option<&SiblingsRegister>,
    crate_map: &[CrateRow],
    proof_boundary_edges: &[ForbiddenEdgeRow],
    findings: &mut Vec<Finding>,
) {
    let ws_kind = ws.kind;
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

            // Step 7 direct-edge ownership (N33c-09):
            // - `egg` is reported exclusively by `suite.egg` (no duplicate L-5.third-party).
            if dep.name == "egg" {
                continue;
            }
            // - Never-dependency siblings not explicitly admitted for this workspace in
            //   `franken-stack.lock` are reported exclusively by `suite.forbidden-package`.
            if let Some(sib) = match_never_dependency_dep(dep, siblings)
                && !is_forbidden_sibling_admitted_in_workspace(&sib, ws, Some(pkg), stack_lock)
            {
                continue;
            }
            // - Direct frankentui dependencies outside `frob-tui` (or `allowed_in` of siblings.toml)
            //   are refused by `ftui.executor-feature`.
            if is_frankentui_dep(dep, siblings) {
                let allowed_crates = ftui_allowed_crates(siblings);
                if !allowed_crates.iter().any(|c| c == &pkg.name) {
                    findings.push(Finding {
                        rule: RULE_FTUI_EXECUTOR_FEATURE,
                        subject: format!("{} -> {}", pkg.name, dep.name),
                        path: pkg.rel_manifest.clone(),
                        line: None,
                        message: format!(
                            "crate `{}` has a direct {} dependency on frankentui crate `{}`; frankentui crates may be direct dependencies of {:?} only (N33c-09)",
                            pkg.name, dep.kind, dep.name, allowed_crates
                        ),
                        repair: format!(
                            "remove `{}` from `{}`; restrict direct frankentui dependencies to `frob-tui` with default features off",
                            dep.name, pkg.name
                        ),
                    });
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
    // 1. Sibling path prefix match (pick the longest / most-specific `row.path` so
    //    `../asupersync/franken_kernel` matches `FD-FKER` rather than `FD-ASUP` `../asupersync`).
    if let Some(sib_path) = dep.sibling_rel_path.as_deref() {
        let mut best: Option<(usize, &'a Foundation)> = None;
        for row in &stack_lock.foundations {
            if let Some(row_path) = row.path.as_ref() {
                let rp = row_path.as_str().trim_end_matches('/');
                if (sib_path == rp || sib_path.starts_with(&format!("{rp}/")))
                    && best.is_none_or(|(best_len, _)| rp.len() > best_len)
                {
                    best = Some((rp.len(), row));
                }
            }
        }
        if let Some((_, row)) = best {
            return Some(row);
        }
    }

    // 2. Package name match (exact or hyphen/underscore normalized).
    let dep_norm = normalize_pkg_ident(&dep.name);
    for row in &stack_lock.foundations {
        if row.name == dep.name || normalize_pkg_ident(&row.name) == dep_norm {
            return Some(row);
        }
    }

    // 2b. Known multi-crate sibling family prefixes (for registry-consumed or subcrate names).
    if let Some(family_name) = foundation_family_for_package(&dep.name) {
        let fam_norm = normalize_pkg_ident(family_name);
        if let Some(row) = stack_lock
            .foundations
            .iter()
            .find(|f| f.name == family_name || normalize_pkg_ident(&f.name) == fam_norm)
        {
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

fn foundation_family_for_package(pkg_name: &str) -> Option<&'static str> {
    if pkg_name == "franken-kernel" || pkg_name == "franken-evidence" {
        return Some("franken-kernel");
    }
    if pkg_name == "frankentui"
        || pkg_name == "ftui"
        || pkg_name == "doctor_frankentui"
        || pkg_name.starts_with("ftui-")
    {
        return Some("frankentui");
    }
    if pkg_name.starts_with("fj-") {
        return Some("frankenjax");
    }
    if pkg_name.starts_with("f3d-") {
        return Some("franken_threed");
    }
    if pkg_name.starts_with("ftx-") {
        return Some("franken_textiles");
    }
    if pkg_name.starts_with("fss-") {
        return Some("franken_surveillance_system");
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

// ============================================================================
// Step 7: Suite deny rules (N33c-09) — resolved graphs, --all-features, Cargo.lock
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
struct MatchedForbiddenSibling {
    name: String,
    rel_path: String,
}

#[must_use]
fn is_asupersync_0_5(name: &str, version: &str) -> bool {
    name == "asupersync" && (version == "0.5" || version.starts_with("0.5."))
}

fn is_frankentui_name(name: &str) -> bool {
    name == "frankentui"
        || name == "ftui"
        || name == "doctor_frankentui"
        || name.starts_with("ftui-")
}

fn sib_path_matches(sib_path: &str, root_rel: &str) -> bool {
    let rp = root_rel.trim_end_matches('/');
    sib_path == rp || sib_path.starts_with(&format!("{rp}/"))
}

fn is_frankentui_path(sib_path: &str, siblings: Option<&SiblingsRegister>) -> bool {
    if sib_path_matches(sib_path, "../frankentui") {
        return true;
    }
    siblings
        .and_then(|reg| reg.find("frankentui"))
        .is_some_and(|row| sib_path_matches(sib_path, &row.path))
}

fn is_frankentui_dep(dep: &DeclaredDep, siblings: Option<&SiblingsRegister>) -> bool {
    if let Some(sp) = dep.sibling_rel_path.as_deref()
        && is_frankentui_path(sp, siblings)
    {
        return true;
    }
    is_frankentui_name(&dep.name)
}

fn is_frankentui_package(pkg: &ResolvedPackage, siblings: Option<&SiblingsRegister>) -> bool {
    if let Some(sp) = pkg.sibling_rel_path.as_deref()
        && is_frankentui_path(sp, siblings)
    {
        return true;
    }
    is_frankentui_name(&pkg.name)
}

fn ftui_allowed_crates(siblings: Option<&SiblingsRegister>) -> Vec<String> {
    if let Some(row) = siblings.and_then(|reg| reg.find("frankentui"))
        && !row.allowed_in.is_empty()
    {
        return row.allowed_in.clone();
    }
    vec!["frob-tui".to_owned()]
}

fn ftui_forbidden_features(siblings: Option<&SiblingsRegister>) -> Vec<String> {
    let mut feats = BTreeSet::new();
    feats.insert(FTUI_FORBIDDEN_FEATURE.to_owned());
    if let Some(row) = siblings.and_then(|reg| reg.find("frankentui")) {
        for f in &row.forbidden_features {
            feats.insert(f.clone());
        }
    }
    feats.into_iter().collect()
}

fn match_never_dependency_by_path_or_name(
    sib_path: Option<&str>,
    pkg_name: &str,
    siblings: Option<&SiblingsRegister>,
) -> Option<MatchedForbiddenSibling> {
    if let Some(sp) = sib_path {
        for spec in NEVER_DEPENDENCY_SIBLINGS {
            if sib_path_matches(sp, spec.rel_path) {
                return Some(MatchedForbiddenSibling {
                    name: spec.name.to_owned(),
                    rel_path: spec.rel_path.to_owned(),
                });
            }
        }
        if let Some(reg) = siblings {
            for row in &reg.siblings {
                if row.cargo == CargoPolicy::Never && sib_path_matches(sp, &row.path) {
                    return Some(MatchedForbiddenSibling {
                        name: row.name.clone(),
                        rel_path: row.path.clone(),
                    });
                }
            }
        }
    }

    for spec in NEVER_DEPENDENCY_SIBLINGS {
        if matches_any_pattern(pkg_name, spec.pkg_patterns) {
            return Some(MatchedForbiddenSibling {
                name: spec.name.to_owned(),
                rel_path: spec.rel_path.to_owned(),
            });
        }
    }

    if let Some(reg) = siblings {
        let pkg_norm = normalize_pkg_ident(pkg_name);
        for row in &reg.siblings {
            if row.cargo == CargoPolicy::Never
                && (row.name == pkg_name || normalize_pkg_ident(&row.name) == pkg_norm)
            {
                return Some(MatchedForbiddenSibling {
                    name: row.name.clone(),
                    rel_path: row.path.clone(),
                });
            }
        }
    }

    None
}

fn match_never_dependency_dep(
    dep: &DeclaredDep,
    siblings: Option<&SiblingsRegister>,
) -> Option<MatchedForbiddenSibling> {
    match_never_dependency_by_path_or_name(dep.sibling_rel_path.as_deref(), &dep.name, siblings)
}

fn match_never_dependency_package(
    pkg: &ResolvedPackage,
    siblings: Option<&SiblingsRegister>,
) -> Option<MatchedForbiddenSibling> {
    match_never_dependency_by_path_or_name(pkg.sibling_rel_path.as_deref(), &pkg.name, siblings)
}

/// Fail-closed stack-lock exception for `suite.forbidden-package` (`N33c-09`):
/// a workspace may carry a package of a never-dependency sibling ONLY if a
/// `[[foundation]]` row in `franken-stack.lock` matches that sibling, names a
/// non-`"none"` `admission_gate`, and declares a Cargo-allowing posture whose
/// `confined_to` explicitly lists this workspace or the importing crate.
fn is_forbidden_sibling_admitted_in_workspace(
    sib: &MatchedForbiddenSibling,
    ws: &WorkspaceRoot,
    root_member: Option<&PackageMeta>,
    stack_lock: &StackLock,
) -> bool {
    let sib_norm = normalize_pkg_ident(&sib.name);
    let sib_path_trim = sib.rel_path.trim_end_matches('/');
    let Some(foundation) = stack_lock.foundations.iter().find(|f| {
        f.name == sib.name
            || normalize_pkg_ident(&f.name) == sib_norm
            || f.path
                .as_ref()
                .is_some_and(|p| p.as_str().trim_end_matches('/') == sib_path_trim)
    }) else {
        return false;
    };

    if foundation.admission_gate.is_none() {
        return false;
    }

    foundation.uses.iter().any(|u| {
        let cargo_allowing = matches!(
            u.posture,
            Posture::Consume | Posture::ConsumePath | Posture::BuildTime | Posture::Oracle
        );
        cargo_allowing && confinement_admits_workspace(&u.confined_to, ws, root_member)
    })
}

fn confinement_admits_workspace(
    confined_to: &[String],
    ws: &WorkspaceRoot,
    root_member: Option<&PackageMeta>,
) -> bool {
    let ws_rel = ws.rel_dir.trim_end_matches('/');
    for entry in confined_to {
        // Explicit workspace or crate admission is required; `"any"` never grants an exception.
        if entry == "any" {
            continue;
        }
        let trimmed = entry.trim_end_matches('/');
        if trimmed.is_empty() {
            continue;
        }
        if !ws_rel.is_empty()
            && (trimmed == ws_rel || ws_rel.starts_with(&format!("{trimmed}/")))
        {
            return true;
        }
        if trimmed == ws.name {
            return true;
        }
        if let Some(pkg) = root_member {
            if trimmed == pkg.name || trimmed == pkg.rel_dir {
                return true;
            }
            if entry.ends_with('/')
                && !pkg.rel_dir.is_empty()
                && (pkg.rel_dir.starts_with(entry.as_str()) || pkg.rel_dir == trimmed)
            {
                return true;
            }
        }
    }
    false
}

/// Evaluate the four suite deny rules (`asup.v0_5_0`, `ftui.executor-feature`,
/// `suite.forbidden-package`, `suite.egg`) over one resolved workspace graph.
fn evaluate_suite_resolved_rules(
    ws: &WorkspaceRoot,
    declared_members: &[PackageMeta],
    resolved: &ResolvedWorkspace,
    stack_lock: &StackLock,
    siblings: Option<&SiblingsRegister>,
    reported_lock_asup: &mut BTreeSet<(String, String)>,
    findings: &mut Vec<Finding>,
) {
    let members_by_id: BTreeMap<&str, &PackageMeta> = declared_members
        .iter()
        .map(|m| (m.id.as_str(), m))
        .collect();
    let parent = build_resolved_bfs_parents(resolved);
    let default_manifest = if ws.rel_dir.is_empty() {
        "Cargo.toml".to_owned()
    } else {
        format!("{}/Cargo.toml", ws.rel_dir)
    };
    let ws_lock_rel = if ws.rel_dir.is_empty() {
        "Cargo.lock".to_owned()
    } else {
        format!("{}/Cargo.lock", ws.rel_dir)
    };

    let forbidden_ftui_feats = ftui_forbidden_features(siblings);

    let mut all_pkgs: Vec<(&str, &ResolvedPackage)> = resolved
        .packages
        .iter()
        .map(|(id, pkg)| (id.as_str(), pkg))
        .collect();
    all_pkgs.sort_by(|a, b| {
        a.1.name
            .cmp(&b.1.name)
            .then_with(|| a.1.version.cmp(&b.1.version))
            .then_with(|| a.0.cmp(b.0))
    });

    for (pkg_id, pkg) in all_pkgs {
        let (chain, root_member) =
            reconstruct_chain(pkg_id, &parent, resolved, &members_by_id);
        let chain_display = if chain.is_empty() {
            format!("{} -> {}", ws.name, pkg.name)
        } else {
            chain.join(" -> ")
        };
        let manifest_rel = root_member
            .map(|m| m.rel_manifest.clone())
            .unwrap_or_else(|| default_manifest.clone());

        // Rule 1: `asup.v0_5_0` — any `asupersync` package on the `0.5.*` line.
        if is_asupersync_0_5(&pkg.name, &pkg.version) {
            reported_lock_asup.insert((ws_lock_rel.clone(), pkg.version.clone()));
            findings.push(Finding {
                rule: RULE_ASUP_V0_5_0,
                subject: format!("{chain_display}@{}", pkg.version),
                path: manifest_rel.clone(),
                line: None,
                message: format!(
                    "forbidden `asupersync` `{}` (0.5.* line) in workspace `{}` via dependency path `{chain_display}` (N33c-09, INV-SUITE-02)",
                    pkg.version, ws.name
                ),
                repair: "remove the dependency path pulling `asupersync 0.5.*` and align on the single version pinned in franken-stack.lock".to_owned(),
            });
        }

        // Rule 2: `ftui.executor-feature` — resolved feature set of any frankentui package
        // must never enable `asupersync-executor` (or any forbidden feature in siblings.toml).
        if is_frankentui_package(pkg, siblings)
            && let Some(node) = resolved.nodes.get(pkg_id)
        {
            for feat in &forbidden_ftui_feats {
                if node.features.iter().any(|f| f == feat) {
                    findings.push(Finding {
                        rule: RULE_FTUI_EXECUTOR_FEATURE,
                        subject: format!("{chain_display} [{feat}]"),
                        path: manifest_rel.clone(),
                        line: None,
                        message: format!(
                            "frankentui package `{}` enables forbidden feature `{feat}` in workspace `{}` via dependency path `{chain_display}` (N33c-09)",
                            pkg.name, ws.name
                        ),
                        repair: format!(
                            "disable `{feat}` and set `default-features = false` on `{}` until FD-FTUI-01 lands",
                            pkg.name
                        ),
                    });
                }
            }
        }

        // Rule 3: `suite.forbidden-package` — eight never-dependency siblings,
        // fail-closed unless explicitly admitted for this workspace in `franken-stack.lock`.
        if let Some(sib) = match_never_dependency_package(pkg, siblings)
            && !is_forbidden_sibling_admitted_in_workspace(&sib, ws, root_member, stack_lock)
        {
            let loc = pkg
                .sibling_rel_path
                .as_deref()
                .or(pkg.source.as_deref())
                .unwrap_or(&sib.rel_path);
            findings.push(Finding {
                rule: RULE_SUITE_FORBIDDEN_PACKAGE,
                subject: chain_display.clone(),
                path: manifest_rel.clone(),
                line: None,
                message: format!(
                    "never-dependency sibling `{}` package `{}` ({loc}) appears in workspace `{}` via dependency path `{chain_display}` without a workspace-scoped admission row in franken-stack.lock (N33c-09)",
                    sib.name, pkg.name, ws.name
                ),
                repair: format!(
                    "remove `{}` (`{}`) from workspace `{}`; never-dependency siblings remain outside Cargo graphs unless admitted with a gate in franken-stack.lock",
                    pkg.name, sib.name, ws.name
                ),
            });
        }

        // Rule 5: `suite.egg` — third-party `egg` crate in any resolved graph.
        if pkg.name == "egg" {
            findings.push(Finding {
                rule: RULE_SUITE_EGG,
                subject: chain_display.clone(),
                path: manifest_rel,
                line: None,
                message: format!(
                    "forbidden third-party crate `egg` `{}` in workspace `{}` via dependency path `{chain_display}` (hint: `egg` typically arrives through `frankenjax` / `fj-egraph`; N33c-09)",
                    pkg.version, ws.name
                ),
                repair: "remove `egg` and any `frankenjax` / `fj-egraph` dependency pulling it into the build graph".to_owned(),
            });
        }
    }
}

fn build_resolved_bfs_parents(resolved: &ResolvedWorkspace) -> BTreeMap<&str, Option<&str>> {
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

    let mut parent: BTreeMap<&str, Option<&str>> = BTreeMap::new();
    let mut queue: VecDeque<&str> = VecDeque::new();
    for &mid in &member_ids {
        parent.insert(mid, None);
        queue.push_back(mid);
    }

    while let Some(cur) = queue.pop_front() {
        let Some(node) = resolved.nodes.get(cur) else {
            continue;
        };
        let mut child_set: BTreeSet<&str> = node.dependencies.iter().map(String::as_str).collect();
        for d in &node.deps {
            child_set.insert(d.pkg.as_str());
        }
        let mut children: Vec<&str> = child_set.into_iter().collect();
        children.sort_by(|a, b| {
            let na = resolved.packages.get(*a).map_or(*a, |p| p.name.as_str());
            let nb = resolved.packages.get(*b).map_or(*b, |p| p.name.as_str());
            na.cmp(nb).then_with(|| a.cmp(b))
        });
        for child in children {
            if !parent.contains_key(child) {
                parent.insert(child, Some(cur));
                queue.push_back(child);
            }
        }
    }

    parent
}

fn reconstruct_chain<'a>(
    pkg_id: &str,
    parent: &BTreeMap<&str, Option<&str>>,
    resolved: &ResolvedWorkspace,
    members_by_id: &BTreeMap<&str, &'a PackageMeta>,
) -> (Vec<String>, Option<&'a PackageMeta>) {
    if !parent.contains_key(pkg_id) {
        let label = resolved
            .packages
            .get(pkg_id)
            .map_or_else(|| pkg_id.to_owned(), |p| p.name.clone());
        return (vec![label], None);
    }
    let mut chain = Vec::new();
    let mut root_id = pkg_id;
    let mut cursor = Some(pkg_id);
    while let Some(id) = cursor {
        root_id = id;
        let label = resolved
            .packages
            .get(id)
            .map_or_else(|| id.to_owned(), |p| p.name.clone());
        chain.push(label);
        cursor = parent.get(id).copied().flatten();
    }
    chain.reverse();
    let root_member = members_by_id.get(root_id).copied();
    (chain, root_member)
}

/// Scan every checked workspace's `Cargo.lock` and every committed `Cargo.lock` under
/// `repo_root` (excluding `tests/admission/`, `target/`, `.git/`, etc.) for any
/// `asupersync` entry on the `0.5.*` line, catching unlinked lockfile entries (`N33c-09`).
fn scan_lockfiles_for_asup_0_5(
    repo_root: &Path,
    roots: &[WorkspaceRoot],
    reported_lock_asup: &BTreeSet<(String, String)>,
    findings: &mut Vec<Finding>,
) {
    let mut lockfiles: BTreeSet<PathBuf> = BTreeSet::new();
    for ws in roots {
        let candidate = ws.root_dir.join("Cargo.lock");
        if candidate.is_file() {
            lockfiles.insert(candidate);
        }
    }
    collect_committed_lockfiles(repo_root, repo_root, 0, &mut lockfiles);

    for lock_path in lockfiles {
        scan_single_lockfile_for_asup_0_5(repo_root, &lock_path, reported_lock_asup, findings);
    }
}

fn collect_committed_lockfiles(
    repo_root: &Path,
    dir: &Path,
    depth: usize,
    out: &mut BTreeSet<PathBuf>,
) {
    if depth > 4 {
        return;
    }
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    let mut paths: Vec<PathBuf> = entries.filter_map(|e| e.ok().map(|x| x.path())).collect();
    paths.sort();

    for path in paths {
        let Some(fname) = path.file_name().and_then(|s| s.to_str()) else {
            continue;
        };
        if path.is_dir() {
            if matches!(
                fname,
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
            ) {
                continue;
            }
            if let Ok(rel) = path.strip_prefix(repo_root) {
                let rel_s = normalize_rel_path(rel);
                if rel_s == "tests/admission" || rel_s.starts_with("tests/admission/") {
                    continue;
                }
            }
            collect_committed_lockfiles(repo_root, &path, depth + 1, out);
        } else if fname == "Cargo.lock" {
            out.insert(path);
        }
    }
}

fn scan_single_lockfile_for_asup_0_5(
    repo_root: &Path,
    lock_path: &Path,
    reported_lock_asup: &BTreeSet<(String, String)>,
    findings: &mut Vec<Finding>,
) {
    let Ok(bytes) = fs::read(lock_path) else {
        return;
    };
    let Ok(root) = toml::parse(&bytes) else {
        return;
    };
    let rel_lock = lock_path
        .strip_prefix(repo_root)
        .ok()
        .map(normalize_rel_path)
        .unwrap_or_else(|| normalize_rel_path(lock_path));

    // Collect `[[package]]` entries and reverse dependency pointers for diagnostics.
    let mut pkg_entries: Vec<(String, String, Vec<String>)> = Vec::new();
    if let Some(Value::Array(items)) = root.get("package") {
        for item in items {
            if let Value::Table(tbl) = &item.value {
                let name = tbl
                    .get("name")
                    .and_then(Value::as_str)
                    .unwrap_or("")
                    .to_owned();
                let ver = tbl
                    .get("version")
                    .and_then(Value::as_str)
                    .unwrap_or("")
                    .to_owned();
                let mut deps = Vec::new();
                if let Some(Value::Array(darr)) = tbl.get("dependencies") {
                    for d in darr {
                        if let Some(ds) = d.value.as_str() {
                            deps.push(ds.to_owned());
                        }
                    }
                }
                pkg_entries.push((name, ver, deps));
            }
        }
    }

    let mut hits: Vec<(String, String)> = Vec::new();
    for (name, ver, _) in &pkg_entries {
        if is_asupersync_0_5(name, ver) {
            if reported_lock_asup.contains(&(rel_lock.clone(), ver.clone())) {
                continue;
            }
            let dependents: Vec<String> = pkg_entries
                .iter()
                .filter(|(_, _, deps)| {
                    deps.iter().any(|d| {
                        let mut parts = d.split_whitespace();
                        let dname = parts.next().unwrap_or("");
                        let dver = parts.next();
                        dname == "asupersync" && dver.is_none_or(|dv| dv == ver)
                    })
                })
                .map(|(n, _, _)| n.clone())
                .collect();
            let detail = if dependents.is_empty() {
                "unlinked in lockfile".to_owned()
            } else {
                format!("referenced in lockfile by {}", dependents.join(", "))
            };
            hits.push((ver.clone(), detail));
        }
    }

    // Also inspect `[[patch.unused]]` entries (Cargo records unused patches here).
    if let Some(Value::Table(ptbl)) = root.get("patch")
        && let Some(Value::Array(uarr)) = ptbl.get("unused")
    {
        for item in uarr {
            if let Value::Table(tbl) = &item.value {
                let name = tbl.get("name").and_then(Value::as_str).unwrap_or("");
                let ver = tbl.get("version").and_then(Value::as_str).unwrap_or("");
                if is_asupersync_0_5(name, ver)
                    && !reported_lock_asup.contains(&(rel_lock.clone(), ver.to_owned()))
                {
                    hits.push((
                        ver.to_owned(),
                        "unlinked `[[patch.unused]]` entry in lockfile".to_owned(),
                    ));
                }
            }
        }
    }

    for (ver, detail) in hits {
        findings.push(Finding {
            rule: RULE_ASUP_V0_5_0,
            subject: format!("{rel_lock} -> asupersync@{ver}"),
            path: rel_lock.clone(),
            line: None,
            message: format!(
                "forbidden `asupersync` `{ver}` (0.5.* line) present in `{rel_lock}` ({detail}; N33c-09, INV-SUITE-02)"
            ),
            repair: format!(
                "remove `asupersync {ver}` from `{rel_lock}` and update any dependency or patch pulling the 0.5.* line"
            ),
        });
    }
}

// ============================================================================
// Step 8: Core forbidden-package rule (`core.forbidden-package`, N34p4-02)
// ============================================================================

/// Evaluate `core.forbidden-package` (`N34p4-02`) over the core workspace's resolved
/// dependency graphs (`resolved` and `resolved_all`) and its committed `Cargo.lock`
/// (parsed strictly through `frob_core::registry::toml`).
///
/// Matches both package-name / prefix-pattern entries in `workspaces::FORBIDDEN_CORE_PACKAGES`
/// and manifest paths lying under `spark/`, `apps/`, `derive/`, or `bindings/`. Each finding
/// reports the offending package, its justification (`annex = "..."`, `route = "..."`, or
/// `nongoal = "..."`), and either a shortest dependency path from a core crate or
/// `lockfile-only` when no edge of the resolved graph reaches the package.
fn evaluate_core_forbidden_packages(
    repo_root: &Path,
    ws: &WorkspaceRoot,
    declared_members: &[PackageMeta],
    resolved: &ResolvedWorkspace,
    resolved_all: &ResolvedWorkspace,
    findings: &mut Vec<Finding>,
) {
    let members_by_id: BTreeMap<&str, &PackageMeta> = declared_members
        .iter()
        .map(|m| (m.id.as_str(), m))
        .collect();
    let default_manifest = if ws.rel_dir.is_empty() {
        "Cargo.toml".to_owned()
    } else {
        format!("{}/Cargo.toml", ws.rel_dir)
    };

    let mut reported_pkg_names: BTreeSet<String> = BTreeSet::new();

    for res in [resolved, resolved_all] {
        let parent = build_resolved_bfs_parents(res);
        let mut all_pkgs: Vec<(&str, &ResolvedPackage)> = res
            .packages
            .iter()
            .map(|(id, pkg)| (id.as_str(), pkg))
            .collect();
        all_pkgs.sort_by(|a, b| {
            a.1.name
                .cmp(&b.1.name)
                .then_with(|| a.1.version.cmp(&b.1.version))
                .then_with(|| a.0.cmp(b.0))
        });

        for (pkg_id, pkg) in all_pkgs {
            if reported_pkg_names.contains(&pkg.name) {
                continue;
            }
            let reason = match workspaces::match_forbidden_core_package(&pkg.name) {
                Some(entry) => entry.why,
                None => pkg
                    .rel_dir
                    .as_deref()
                    .and_then(workspaces::match_forbidden_core_manifest_dir),
            };
            let Some(reason) = reason else {
                continue;
            };
            if !parent.contains_key(pkg_id) {
                continue;
            }

            let (chain, root_member) =
                reconstruct_chain(pkg_id, &parent, res, &members_by_id);
            let chain_display = if chain.is_empty() {
                pkg.name.clone()
            } else {
                chain.join(" -> ")
            };
            let manifest_rel = root_member
                .map(|m| m.rel_manifest.clone())
                .unwrap_or_else(|| default_manifest.clone());
            let reason_tag = reason.format_tag();

            reported_pkg_names.insert(pkg.name.clone());
            findings.push(Finding {
                rule: RULE_CORE_FORBIDDEN_PACKAGE,
                subject: chain_display.clone(),
                path: manifest_rel,
                line: None,
                message: format!(
                    "core workspace contains forbidden package `{}` ({reason_tag}) via dependency path `{chain_display}` (N34p4-02)",
                    pkg.name
                ),
                repair: format!(
                    "remove `{}` from the core dependency graph ({reason_tag}); keep annex/route-excluded crates out of the core lockfile",
                    pkg.name
                ),
            });
        }
    }

    let lock_file = ws.root_dir.join("Cargo.lock");
    if lock_file.is_file() {
        scan_core_lockfile_for_forbidden_packages(
            repo_root,
            &lock_file,
            &reported_pkg_names,
            findings,
        );
    }
}

/// Scan the core `Cargo.lock` (read through `frob_core::registry::toml::parse`) for any
/// forbidden package entry not already reported with a resolved dependency path, emitting
/// `core.forbidden-package` with dependency path `lockfile-only`.
fn scan_core_lockfile_for_forbidden_packages(
    repo_root: &Path,
    lock_path: &Path,
    already_reported_pkgs: &BTreeSet<String>,
    findings: &mut Vec<Finding>,
) {
    let Ok(bytes) = fs::read(lock_path) else {
        return;
    };
    let Ok(root) = toml::parse(&bytes) else {
        return;
    };
    let rel_lock = lock_path
        .strip_prefix(repo_root)
        .ok()
        .map(normalize_rel_path)
        .unwrap_or_else(|| normalize_rel_path(lock_path));

    let mut lock_pkg_names: BTreeSet<String> = BTreeSet::new();
    if let Some(Value::Array(items)) = root.get("package") {
        for item in items {
            if let Value::Table(tbl) = &item.value
                && let Some(name) = tbl.get("name").and_then(Value::as_str)
            {
                lock_pkg_names.insert(name.to_owned());
            }
        }
    }
    if let Some(Value::Table(ptbl)) = root.get("patch")
        && let Some(Value::Array(uarr)) = ptbl.get("unused")
    {
        for item in uarr {
            if let Value::Table(tbl) = &item.value
                && let Some(name) = tbl.get("name").and_then(Value::as_str)
            {
                lock_pkg_names.insert(name.to_owned());
            }
        }
    }

    for pkg_name in lock_pkg_names {
        if already_reported_pkgs.contains(&pkg_name) {
            continue;
        }
        let Some(entry) = workspaces::match_forbidden_core_package(&pkg_name) else {
            continue;
        };
        let reason_tag = entry
            .why
            .map_or_else(|| "reason = \"missing\"".to_owned(), ForbiddenReason::format_tag);
        findings.push(Finding {
            rule: RULE_CORE_FORBIDDEN_PACKAGE,
            subject: format!("{pkg_name} (lockfile-only)"),
            path: rel_lock.clone(),
            line: None,
            message: format!(
                "core lockfile `{rel_lock}` contains forbidden package `{pkg_name}` ({reason_tag}) with dependency path `lockfile-only` (N34p4-02)"
            ),
            repair: format!(
                "remove `{pkg_name}` from `{rel_lock}` ({reason_tag}) and refresh the core lockfile"
            ),
        });
    }
}
