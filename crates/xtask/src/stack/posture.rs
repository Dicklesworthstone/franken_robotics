//! Suite-consumption postures, foundation-use vocabulary, and posture-to-graph
//! rules for FrankenRobotics (bead `N33a-01`, ADR-SUITE-01, rule L-4).
//!
//! Shared with the `franken-stack.lock` strict parser (`N33a-03`) and
//! `xtask check-deps` (`BD-003`). Uses `frob_core::registry::toml` for TOML and
//! Cargo's `semver` grammar for exact pins. Pure and deterministic (class D0).

use std::fmt;
use std::str::FromStr;

use frob_core::registry::toml::{self, Fields, Spanned, Table, Value};

/// Bedrock-direct FrankenSim crates permitted outside `frob-fsim` under rule L-2
/// (shared constant for `check-deps`, `BD-003`).
pub const BEDROCK_DIRECT_CRATES: &[&str] = &[
    "fs-ga",
    "fs-math",
    "fs-qty",
    "fs-rand",
    "fs-blake3",
    "fs-ivl",
    "fs-simd",
    "fs-exec",
    "fs-evidence",
    "fs-ad",
];

/// Annex workspace directory prefixes permitted to carry `build-time` edges.
pub const ANNEX_WORKSPACE_PREFIXES: &[&str] = &["derive/", "spark/", "bindings/"];

/// Error returned when parsing a posture, tier, milestone, or foundation `Use`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PostureError {
    pub row: String,
    pub field: String,
    pub message: String,
}

impl PostureError {
    #[must_use]
    pub fn new(
        row: impl Into<String>,
        field: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            row: row.into(),
            field: field.into(),
            message: message.into(),
        }
    }
}

impl fmt::Display for PostureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "row `{}` field `{}`: {}", self.row, self.field, self.message)
    }
}

impl std::error::Error for PostureError {}

/// The seven closed suite-consumption postures (`§33.1`, ADR-SUITE-01).
///
/// The five conceptual postures (`consume`, `subprocess`, `lift`, `oracle`,
/// `pattern`) are refined by `consume-path` (sibling path dependency with
/// explicit feature groups) and `build-time` (annex-only build consumption that
/// never enters a `frob` binary's runtime graph).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Posture {
    Consume,
    ConsumePath,
    Subprocess,
    Lift,
    Oracle,
    Pattern,
    BuildTime,
}

impl Posture {
    /// All seven closed posture variants in canonical order.
    pub const ALL: [Self; 7] = [
        Self::Consume,
        Self::ConsumePath,
        Self::Subprocess,
        Self::Lift,
        Self::Oracle,
        Self::Pattern,
        Self::BuildTime,
    ];

    /// Canonical TOML string representation.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Consume => "consume",
            Self::ConsumePath => "consume-path",
            Self::Subprocess => "subprocess",
            Self::Lift => "lift",
            Self::Oracle => "oracle",
            Self::Pattern => "pattern",
            Self::BuildTime => "build-time",
        }
    }

    /// Parse a closed posture string; unknown values fail naming `row`.
    pub fn parse(raw: &str, row: &str) -> Result<Self, PostureError> {
        match raw {
            "consume" => Ok(Self::Consume),
            "consume-path" => Ok(Self::ConsumePath),
            "subprocess" => Ok(Self::Subprocess),
            "lift" => Ok(Self::Lift),
            "oracle" => Ok(Self::Oracle),
            "pattern" => Ok(Self::Pattern),
            "build-time" => Ok(Self::BuildTime),
            other => Err(PostureError::new(
                row,
                "posture",
                format!(
                    "unknown posture `{other}` (expected one of: consume, consume-path, subprocess, lift, oracle, pattern, build-time)"
                ),
            )),
        }
    }
}

impl fmt::Display for Posture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for Posture {
    type Err = PostureError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s, "<unknown>")
    }
}

/// Capability tier for a foundation use (`solid`, `frontier`, `moonshot`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum Tier {
    #[default]
    Solid,
    Frontier,
    Moonshot,
}

impl Tier {
    pub const ALL: [Self; 3] = [Self::Solid, Self::Frontier, Self::Moonshot];

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Solid => "solid",
            Self::Frontier => "frontier",
            Self::Moonshot => "moonshot",
        }
    }

    pub fn parse(raw: &str, row: &str) -> Result<Self, PostureError> {
        match raw {
            "solid" => Ok(Self::Solid),
            "frontier" => Ok(Self::Frontier),
            "moonshot" => Ok(Self::Moonshot),
            other => Err(PostureError::new(
                row,
                "tier",
                format!("unknown tier `{other}` (expected one of: solid, frontier, moonshot)"),
            )),
        }
    }
}

impl fmt::Display for Tier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for Tier {
    type Err = PostureError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s, "<unknown>")
    }
}

/// Program milestone (`M0..=M8`) at which a foundation row or use is first needed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Milestone {
    M0,
    M1,
    M2,
    M3,
    M4,
    M5,
    M6,
    M7,
    M8,
}

impl Milestone {
    pub const ALL: [Self; 9] = [
        Self::M0,
        Self::M1,
        Self::M2,
        Self::M3,
        Self::M4,
        Self::M5,
        Self::M6,
        Self::M7,
        Self::M8,
    ];

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::M0 => "M0",
            Self::M1 => "M1",
            Self::M2 => "M2",
            Self::M3 => "M3",
            Self::M4 => "M4",
            Self::M5 => "M5",
            Self::M6 => "M6",
            Self::M7 => "M7",
            Self::M8 => "M8",
        }
    }

    pub fn parse(raw: &str, row: &str) -> Result<Self, PostureError> {
        match raw {
            "M0" => Ok(Self::M0),
            "M1" => Ok(Self::M1),
            "M2" => Ok(Self::M2),
            "M3" => Ok(Self::M3),
            "M4" => Ok(Self::M4),
            "M5" => Ok(Self::M5),
            "M6" => Ok(Self::M6),
            "M7" => Ok(Self::M7),
            "M8" => Ok(Self::M8),
            other => Err(PostureError::new(
                row,
                "first_needed",
                format!("unknown milestone `{other}` (expected M0..=M8)"),
            )),
        }
    }
}

impl fmt::Display for Milestone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for Milestone {
    type Err = PostureError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s, "<unknown>")
    }
}

/// One declared use of a sibling foundation (`§33.1`, shared with `N33a-03`).
///
/// A posture belongs to a use, not to a repository: one `[[foundation]]` row
/// may carry multiple `Use` entries (e.g. FrankenSQLite as `consume` for the
/// ledger and `pattern` + `oracle` for pager policy).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Use {
    pub posture: Posture,
    pub purpose: String,
    pub confined_to: Vec<String>,
    pub tier: Tier,
    /// `M0..=M8` when admitted later than the parent row; `None` inherits the
    /// parent row's `first_needed`.
    pub first_needed: Option<Milestone>,
}

/// Cargo dependency edge kind evaluated by [`edge_allowed`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EdgeKind {
    Normal,
    Build,
    Dev,
}

impl EdgeKind {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Normal => "normal",
            Self::Build => "build",
            Self::Dev => "dev",
        }
    }
}

impl fmt::Display for EdgeKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Dependency specification as written in a Cargo manifest, paired with the
/// optional row-level `version` or `path` expectation from `franken-stack.lock`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DepSpec {
    /// Version requirement written in the manifest (e.g. `"=0.6.2"`, `"^0.6"`, `"0.6.2"`).
    pub version: Option<String>,
    /// Path written in the manifest (e.g. `"../frankensim/crates/fs-ga"`).
    pub path: Option<String>,
    /// Git source URL if present (forbidden for `consume`, `consume-path`, `oracle`, `build-time`).
    pub git: Option<String>,
    /// Whether `default-features` is enabled (Cargo defaults to `true`).
    pub default_features: bool,
    /// Expected exact version from the foundation row (`"=x.y.z"`), when checked against a row.
    pub row_version: Option<String>,
    /// Expected sibling root path from the foundation row (e.g. `"../frankensim"`), when checked against a row.
    pub row_path: Option<String>,
}

impl Default for DepSpec {
    fn default() -> Self {
        Self {
            version: None,
            path: None,
            git: None,
            default_features: true,
            row_version: None,
            row_path: None,
        }
    }
}

impl DepSpec {
    /// Construct a registry dependency specification.
    #[must_use]
    pub fn registry(version: impl Into<String>, default_features: bool) -> Self {
        Self {
            version: Some(version.into()),
            path: None,
            git: None,
            default_features,
            row_version: None,
            row_path: None,
        }
    }

    /// Construct a sibling path dependency specification.
    #[must_use]
    pub fn path(path: impl Into<String>, default_features: bool) -> Self {
        Self {
            version: None,
            path: Some(path.into()),
            git: None,
            default_features,
            row_version: None,
            row_path: None,
        }
    }

    /// Construct a git dependency specification.
    #[must_use]
    pub fn git(url: impl Into<String>, default_features: bool) -> Self {
        Self {
            version: None,
            path: None,
            git: Some(url.into()),
            default_features,
            row_version: None,
            row_path: None,
        }
    }

    /// Attach the foundation row's exact `version` pin for cross-checking.
    #[must_use]
    pub fn with_row_version(mut self, row_version: impl Into<String>) -> Self {
        self.row_version = Some(row_version.into());
        self
    }

    /// Attach the foundation row's `path` root for cross-checking.
    #[must_use]
    pub fn with_row_path(mut self, row_path: impl Into<String>) -> Self {
        self.row_path = Some(row_path.into());
        self
    }
}

/// High-level build-graph permission class for a [`Posture`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GraphPermission {
    /// Confined crate edge via exact registry pin (`=x.y.z`) with `default-features = false`.
    ConfinedExactRegistry,
    /// Confined crate edge via sibling path with `default-features = false` (no version range, no git).
    ConfinedSiblingPath,
    /// Dev-dependency only, confined to the listed crates (normally `frob-conformance`).
    DevOnlyConfined,
    /// Allowed only inside the named annex workspace (never a core crate).
    AnnexOnly,
    /// No Cargo dependency edge is allowed in any workspace.
    Forbidden,
}

/// Declarative posture-to-graph rule row consumed by `check-deps` (`BD-003`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PostureRule {
    pub posture: Posture,
    pub permission: GraphPermission,
    pub allows_normal: bool,
    pub allows_build: bool,
    pub allows_dev: bool,
    pub requires_default_features_off: bool,
    pub requires_exact_version: bool,
    pub requires_sibling_path: bool,
    pub allows_git: bool,
    pub annex_only: bool,
    pub requires_lifts_registry: bool,
}

/// Authoritative posture-to-graph rules table (`§33.1`, `N33a-01`, `BD-003`).
pub const POSTURE_RULES: [PostureRule; 7] = [
    PostureRule {
        posture: Posture::Consume,
        permission: GraphPermission::ConfinedExactRegistry,
        allows_normal: true,
        allows_build: true,
        allows_dev: true,
        requires_default_features_off: true,
        requires_exact_version: true,
        requires_sibling_path: false,
        allows_git: false,
        annex_only: false,
        requires_lifts_registry: false,
    },
    PostureRule {
        posture: Posture::ConsumePath,
        permission: GraphPermission::ConfinedSiblingPath,
        allows_normal: true,
        allows_build: true,
        allows_dev: true,
        requires_default_features_off: true,
        requires_exact_version: false,
        requires_sibling_path: true,
        allows_git: false,
        annex_only: false,
        requires_lifts_registry: false,
    },
    PostureRule {
        posture: Posture::Subprocess,
        permission: GraphPermission::Forbidden,
        allows_normal: false,
        allows_build: false,
        allows_dev: false,
        requires_default_features_off: false,
        requires_exact_version: false,
        requires_sibling_path: false,
        allows_git: false,
        annex_only: false,
        requires_lifts_registry: false,
    },
    PostureRule {
        posture: Posture::Lift,
        permission: GraphPermission::Forbidden,
        allows_normal: false,
        allows_build: false,
        allows_dev: false,
        requires_default_features_off: false,
        requires_exact_version: false,
        requires_sibling_path: false,
        allows_git: false,
        annex_only: false,
        requires_lifts_registry: true,
    },
    PostureRule {
        posture: Posture::Oracle,
        permission: GraphPermission::DevOnlyConfined,
        allows_normal: false,
        allows_build: false,
        allows_dev: true,
        requires_default_features_off: false,
        requires_exact_version: false,
        requires_sibling_path: false,
        allows_git: false,
        annex_only: false,
        requires_lifts_registry: false,
    },
    PostureRule {
        posture: Posture::Pattern,
        permission: GraphPermission::Forbidden,
        allows_normal: false,
        allows_build: false,
        allows_dev: false,
        requires_default_features_off: false,
        requires_exact_version: false,
        requires_sibling_path: false,
        allows_git: false,
        annex_only: false,
        requires_lifts_registry: false,
    },
    PostureRule {
        posture: Posture::BuildTime,
        permission: GraphPermission::AnnexOnly,
        allows_normal: true,
        allows_build: true,
        allows_dev: true,
        requires_default_features_off: false,
        requires_exact_version: false,
        requires_sibling_path: false,
        allows_git: false,
        annex_only: true,
        requires_lifts_registry: false,
    },
];

/// Look up the [`PostureRule`] row for `posture`.
#[must_use]
pub const fn rule_for(posture: Posture) -> &'static PostureRule {
    match posture {
        Posture::Consume => &POSTURE_RULES[0],
        Posture::ConsumePath => &POSTURE_RULES[1],
        Posture::Subprocess => &POSTURE_RULES[2],
        Posture::Lift => &POSTURE_RULES[3],
        Posture::Oracle => &POSTURE_RULES[4],
        Posture::Pattern => &POSTURE_RULES[5],
        Posture::BuildTime => &POSTURE_RULES[6],
    }
}

/// Specific reason an edge was refused by [`check_edge`], so `BD-003` can
/// distinguish `L-4.foundation-confined` from `L-4.edge-spec`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeDenial {
    PostureForbidsCargoEdge,
    EdgeKindNotAllowed,
    CrateNotConfined,
    CoreCrateInBuildTime,
    DefaultFeaturesEnabled,
    GitSourceForbidden,
    VersionNotExact,
    VersionMismatch,
    UnexpectedPathOnRegistryConsume,
    UnexpectedVersionOnConsumePath,
    MissingOrMismatchedPath,
}

/// Detailed edge evaluation outcome (`Allow` or `Deny(EdgeDenial)`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeDecision {
    Allow,
    Deny(EdgeDenial),
}

impl EdgeDecision {
    #[must_use]
    pub const fn is_allowed(self) -> bool {
        matches!(self, Self::Allow)
    }
}

/// Return `true` when `raw` is a strict exact Cargo version requirement of the
/// form `=MAJOR.MINOR.PATCH` (with optional `-prerelease` / `+build` suffix),
/// refusing bare caret requirements (`0.6.2`), `^`, `~`, `*`, ranges, or
/// partial versions (`=0.6`).
#[must_use]
pub fn is_exact_semver(raw: &str) -> bool {
    raw.strip_prefix('=')
        .is_some_and(|version| semver::Version::parse(version).is_ok())
}

/// Return `true` when `from` names a crate inside one of the annex workspaces
/// (`derive/...`, `spark/...`, `bindings/...`), never a core workspace crate.
#[must_use]
pub fn is_annex_crate_path(from: &str) -> bool {
    let trimmed = from.trim();
    if trimmed.is_empty() || trimmed.contains('\\') || trimmed.contains("..") {
        return false;
    }
    ANNEX_WORKSPACE_PREFIXES
        .iter()
        .any(|prefix| trimmed.starts_with(prefix) && trimmed.len() > prefix.len())
}

fn crate_in_confinement(confined_to: &[String], from: &str, allow_any_wildcard: bool) -> bool {
    let trimmed = from.trim();
    if trimmed.is_empty() {
        return false;
    }
    confined_to.iter().any(|entry| {
        if entry == "any" {
            allow_any_wildcard
        } else if entry == trimmed {
            true
        } else if entry.ends_with('/') {
            trimmed.starts_with(entry.as_str())
                && trimmed.len() > entry.len()
                && !trimmed.contains("..")
        } else {
            false
        }
    })
}

fn strip_leading_parent_segments(path: &str) -> (usize, &str) {
    let mut count = 0usize;
    let mut rest = path;
    while let Some(after) = rest.strip_prefix("../") {
        count += 1;
        rest = after;
    }
    (count, rest)
}

fn path_matches_sibling_row(dep_path: &str, row_path: Option<&str>) -> bool {
    let dep = dep_path.trim().trim_end_matches('/');
    if dep.is_empty()
        || dep.starts_with('/')
        || dep.starts_with('~')
        || dep.contains('\\')
        || dep.contains("://")
    {
        return false;
    }
    let (dep_ups, dep_tail) = strip_leading_parent_segments(dep);
    if dep_ups == 0 || dep_tail.is_empty() {
        return false;
    }
    if dep_tail
        .split('/')
        .any(|seg| seg.is_empty() || seg == "." || seg == "..")
    {
        return false;
    }

    let Some(expected) = row_path else {
        return true;
    };
    let row = expected.trim().trim_end_matches('/');
    let (row_ups, row_tail) = strip_leading_parent_segments(row);
    if row_ups == 0 || row_tail.is_empty() {
        return false;
    }
    if row_tail
        .split('/')
        .any(|seg| seg.is_empty() || seg == "." || seg == "..")
    {
        return false;
    }

    // Either exact path match (at workspace root or nested crate depth `../../<sibling>/...`)
    // or a clean sub-path inside the same sibling repository.
    if dep_tail == row_tail {
        return true;
    }
    dep_tail
        .strip_prefix(row_tail)
        .is_some_and(|rem| rem.starts_with('/') && rem.len() > 1)
}

/// Evaluate a candidate Cargo dependency edge against `use_` and return a
/// structured [`EdgeDecision`].
#[must_use]
pub fn check_edge(use_: &Use, from: &str, kind: EdgeKind, spec: &DepSpec) -> EdgeDecision {
    let rule = rule_for(use_.posture);
    if rule.permission == GraphPermission::Forbidden {
        return EdgeDecision::Deny(EdgeDenial::PostureForbidsCargoEdge);
    }

    let kind_ok = match kind {
        EdgeKind::Normal => rule.allows_normal,
        EdgeKind::Build => rule.allows_build,
        EdgeKind::Dev => rule.allows_dev,
    };
    if !kind_ok {
        return EdgeDecision::Deny(EdgeDenial::EdgeKindNotAllowed);
    }

    if rule.annex_only {
        if !is_annex_crate_path(from) {
            return EdgeDecision::Deny(EdgeDenial::CoreCrateInBuildTime);
        }
        if !crate_in_confinement(&use_.confined_to, from, false) {
            return EdgeDecision::Deny(EdgeDenial::CrateNotConfined);
        }
    } else if !crate_in_confinement(&use_.confined_to, from, true) {
        return EdgeDecision::Deny(EdgeDenial::CrateNotConfined);
    }

    if !rule.allows_git && spec.git.is_some() {
        return EdgeDecision::Deny(EdgeDenial::GitSourceForbidden);
    }

    if rule.requires_default_features_off && spec.default_features {
        return EdgeDecision::Deny(EdgeDenial::DefaultFeaturesEnabled);
    }

    if rule.requires_exact_version {
        if spec.path.is_some() {
            return EdgeDecision::Deny(EdgeDenial::UnexpectedPathOnRegistryConsume);
        }
        let Some(ver) = spec.version.as_deref() else {
            return EdgeDecision::Deny(EdgeDenial::VersionNotExact);
        };
        if !is_exact_semver(ver) {
            return EdgeDecision::Deny(EdgeDenial::VersionNotExact);
        }
        if let Some(expected_ver) = spec.row_version.as_deref()
            && (!is_exact_semver(expected_ver) || ver != expected_ver)
        {
            return EdgeDecision::Deny(EdgeDenial::VersionMismatch);
        }
    }

    if rule.requires_sibling_path {
        if spec.version.is_some() {
            return EdgeDecision::Deny(EdgeDenial::UnexpectedVersionOnConsumePath);
        }
        let Some(dep_path) = spec.path.as_deref() else {
            return EdgeDecision::Deny(EdgeDenial::MissingOrMismatchedPath);
        };
        if !path_matches_sibling_row(dep_path, spec.row_path.as_deref()) {
            return EdgeDecision::Deny(EdgeDenial::MissingOrMismatchedPath);
        }
    } else if rule.annex_only
        && let Some(dep_path) = spec.path.as_deref()
        && !path_matches_sibling_row(dep_path, spec.row_path.as_deref())
    {
        return EdgeDecision::Deny(EdgeDenial::MissingOrMismatchedPath);
    }

    EdgeDecision::Allow
}

/// Authoritative posture-to-graph predicate consumed by `BD-003`'s `check-deps`.
///
/// Evaluates whether a dependency edge from workspace crate `from` of kind
/// `kind` (normal, build, or dev) with manifest specification `spec` is
/// admitted by foundation use `use_`.
#[must_use]
pub fn edge_allowed(use_: &Use, from: &str, kind: EdgeKind, spec: &DepSpec) -> bool {
    check_edge(use_, from, kind, spec).is_allowed()
}

/// Parse one `[[foundation.uses]]` (or `[[uses]]`) TOML table strictly via
/// `frob_core::registry::toml::Fields`, refusing unknown keys and naming `row`
/// on every error.
pub fn parse_use_table(table: Table, row: &str) -> Result<Use, PostureError> {
    let mut fields = Fields::new(table, row);
    let raw_posture = fields
        .require_str("posture")
        .map_err(|e| PostureError::new(row, "posture", e.to_string()))?;
    let raw_purpose = fields
        .opt_str("purpose")
        .map_err(|e| PostureError::new(row, "purpose", e.to_string()))?
        .unwrap_or_default();
    let raw_confined = fields
        .opt_array("confined_to")
        .map_err(|e| PostureError::new(row, "confined_to", e.to_string()))?;
    let raw_tier = fields
        .opt_str("tier")
        .map_err(|e| PostureError::new(row, "tier", e.to_string()))?;
    let raw_first_needed = fields
        .opt_str("first_needed")
        .map_err(|e| PostureError::new(row, "first_needed", e.to_string()))?;

    // Finish Fields BEFORE domain validation so Fields::drop never fires in debug builds.
    fields
        .finish()
        .map_err(|e| PostureError::new(row, "unknown", e.to_string()))?;

    let posture = Posture::parse(&raw_posture, row)?;
    let tier = match raw_tier.as_deref() {
        Some(t) => Tier::parse(t, row)?,
        None => Tier::Solid,
    };
    let first_needed = match raw_first_needed.as_deref() {
        Some(m) => Some(Milestone::parse(m, row)?),
        None => None,
    };

    let mut confined_to = Vec::new();
    if let Some(items) = raw_confined {
        confined_to.reserve(items.len());
        for (idx, sp) in items.into_iter().enumerate() {
            let Some(s) = sp.value.as_str() else {
                return Err(PostureError::new(
                    row,
                    "confined_to",
                    format!(
                        "confined_to[{idx}] at {}:{}: expected string, found {}",
                        sp.line,
                        sp.column,
                        sp.value.type_name()
                    ),
                ));
            };
            if s.trim().is_empty() {
                return Err(PostureError::new(
                    row,
                    "confined_to",
                    format!("confined_to[{idx}] must be a non-empty string"),
                ));
            }
            confined_to.push(s.to_owned());
        }
    }

    Ok(Use {
        posture,
        purpose: raw_purpose,
        confined_to,
        tier,
        first_needed,
    })
}

/// Parse an array of `Use` tables from a strict TOML AST array.
pub fn parse_uses_array(items: Vec<Spanned<Value>>, row: &str) -> Result<Vec<Use>, PostureError> {
    let mut uses = Vec::with_capacity(items.len());
    for (idx, sp) in items.into_iter().enumerate() {
        match sp.value {
            Value::Table(tbl) => {
                let entry_row = format!("{row}.uses[{idx}]");
                uses.push(parse_use_table(tbl, &entry_row)?);
            }
            other => {
                return Err(PostureError::new(
                    row,
                    "uses",
                    format!(
                        "uses[{idx}] at {}:{}: expected table, found {}",
                        sp.line,
                        sp.column,
                        other.type_name()
                    ),
                ));
            }
        }
    }
    Ok(uses)
}

/// Parse a standalone TOML document holding `[[uses]]` tables using
/// `frob_core::registry::toml::parse`.
pub fn parse_uses_toml(bytes: &[u8], row: &str) -> Result<Vec<Use>, PostureError> {
    let root = toml::parse(bytes).map_err(|e| PostureError::new(row, "toml", e.to_string()))?;
    let mut fields = Fields::new(root, row);
    let raw_uses = fields
        .opt_array("uses")
        .map_err(|e| PostureError::new(row, "uses", e.to_string()))?;
    fields
        .finish()
        .map_err(|e| PostureError::new(row, "unknown", e.to_string()))?;

    match raw_uses {
        Some(items) => parse_uses_array(items, row),
        None => Ok(Vec::new()),
    }
}

fn escape_toml_basic_str(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for ch in s.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// Deterministically serialize a list of [`Use`] entries in the strict
/// `frob_core::registry::toml` subset (`[[uses]]` tables, LF endings, D0).
#[must_use]
pub fn format_uses_toml(uses: &[Use]) -> String {
    let mut out = String::new();
    for (idx, u) in uses.iter().enumerate() {
        if idx > 0 {
            out.push('\n');
        }
        out.push_str("[[uses]]\n");
        out.push_str("posture = ");
        out.push_str(&escape_toml_basic_str(u.posture.as_str()));
        out.push('\n');
        out.push_str("purpose = ");
        out.push_str(&escape_toml_basic_str(&u.purpose));
        out.push('\n');
        out.push_str("confined_to = [");
        for (cidx, c) in u.confined_to.iter().enumerate() {
            if cidx > 0 {
                out.push_str(", ");
            }
            out.push_str(&escape_toml_basic_str(c));
        }
        out.push_str("]\n");
        out.push_str("tier = ");
        out.push_str(&escape_toml_basic_str(u.tier.as_str()));
        out.push('\n');
        if let Some(m) = u.first_needed {
            out.push_str("first_needed = ");
            out.push_str(&escape_toml_basic_str(m.as_str()));
            out.push('\n');
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_use(posture: Posture, confined_to: &[&str]) -> Use {
        Use {
            posture,
            purpose: "test".to_owned(),
            confined_to: confined_to.iter().map(|s| (*s).to_owned()).collect(),
            tier: Tier::Solid,
            first_needed: None,
        }
    }

    #[test]
    fn posture_strings_roundtrip() {
        let expected = [
            (Posture::Consume, "consume"),
            (Posture::ConsumePath, "consume-path"),
            (Posture::Subprocess, "subprocess"),
            (Posture::Lift, "lift"),
            (Posture::Oracle, "oracle"),
            (Posture::Pattern, "pattern"),
            (Posture::BuildTime, "build-time"),
        ];
        assert_eq!(Posture::ALL.len(), expected.len());
        for (variant, text) in expected {
            let parsed = Posture::parse(text, "FD-TEST").expect("known posture must parse");
            assert_eq!(parsed, variant);
            assert_eq!(parsed.as_str(), text);
            assert_eq!(parsed.to_string(), text);
            let via_from_str: Posture = text.parse().expect("FromStr must parse known posture");
            assert_eq!(via_from_str, variant);
        }
    }

    #[test]
    fn use_first_needed_optional() {
        let with_m2 = br#"
[[uses]]
posture = "consume"
purpose = "CPU trainer crates"
confined_to = ["frob-learn"]
tier = "frontier"
first_needed = "M2"
"#;
        let parsed = parse_uses_toml(with_m2, "FD-FT").expect("use with M2 must parse");
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0].first_needed, Some(Milestone::M2));
        assert_eq!(parsed[0].tier, Tier::Frontier);
        let serialized = format_uses_toml(&parsed);
        let reparsed = parse_uses_toml(serialized.as_bytes(), "FD-FT")
            .expect("round-tripped use with M2 must parse");
        assert_eq!(reparsed, parsed);

        let without_first_needed = br#"
[[uses]]
posture = "consume"
purpose = "Metal gateway"
confined_to = ["spark/frob-spark-metal"]
"#;
        let parsed_none =
            parse_uses_toml(without_first_needed, "FD-FT").expect("use without first_needed must parse");
        assert_eq!(parsed_none.len(), 1);
        assert_eq!(parsed_none[0].first_needed, None);
        assert_eq!(parsed_none[0].tier, Tier::Solid);

        let bad_m9 = br#"
[[uses]]
posture = "consume"
purpose = "future"
confined_to = ["frob-learn"]
first_needed = "M9"
"#;
        let err = parse_uses_toml(bad_m9, "FD-FT").expect_err("first_needed = M9 must fail");
        let msg = err.to_string();
        assert!(msg.contains("FD-FT"), "error must name row identifier: {msg}");
        assert!(msg.contains("M9"), "error must mention M9: {msg}");
    }

    #[test]
    fn posture_unknown_is_error() {
        let err = Posture::parse("vendored", "FD-BOGUS")
            .expect_err("unknown posture `vendored` must fail");
        let msg = err.to_string();
        assert!(msg.contains("FD-BOGUS"), "error must name row identifier: {msg}");
        assert!(msg.contains("vendored"), "error must name unknown posture: {msg}");

        let toml_err = parse_uses_toml(
            br#"
[[uses]]
posture = "vendored"
purpose = "nope"
confined_to = ["frob-core"]
"#,
            "FD-BOGUS",
        )
        .expect_err("TOML with posture = vendored must fail");
        assert!(
            toml_err.to_string().contains("FD-BOGUS"),
            "TOML parse error must name row identifier: {toml_err}"
        );
    }

    #[test]
    fn edge_rules() {
        let consume_spec = DepSpec::registry("=0.6.2", false).with_row_version("=0.6.2");
        let u_consume = make_use(Posture::Consume, &["frob-chronicle"]);
        assert!(edge_allowed(&u_consume, "frob-chronicle", EdgeKind::Normal, &consume_spec));
        assert!(!edge_allowed(&u_consume, "frob-arena", EdgeKind::Normal, &consume_spec));

        let oracle_spec = DepSpec::registry("=0.5.1", false);
        let u_oracle = make_use(Posture::Oracle, &["frob-conformance"]);
        assert!(edge_allowed(&u_oracle, "frob-conformance", EdgeKind::Dev, &oracle_spec));
        assert!(!edge_allowed(&u_oracle, "frob-conformance", EdgeKind::Normal, &oracle_spec));
        assert!(!edge_allowed(&u_oracle, "frob-conformance", EdgeKind::Build, &oracle_spec));
        assert!(!edge_allowed(&u_oracle, "frob-chronicle", EdgeKind::Dev, &oracle_spec));

        let u_pattern = make_use(Posture::Pattern, &["any"]);
        for kind in [EdgeKind::Normal, EdgeKind::Build, EdgeKind::Dev] {
            assert!(!edge_allowed(&u_pattern, "frob-conformance", kind, &consume_spec));
        }

        let u_subprocess = make_use(Posture::Subprocess, &["any"]);
        for kind in [EdgeKind::Normal, EdgeKind::Build, EdgeKind::Dev] {
            assert!(!edge_allowed(&u_subprocess, "frob-fsim", kind, &consume_spec));
        }

        let u_lift = make_use(Posture::Lift, &["frob-fsim"]);
        for kind in [EdgeKind::Normal, EdgeKind::Build, EdgeKind::Dev] {
            assert!(!edge_allowed(&u_lift, "frob-fsim", kind, &consume_spec));
        }

        let build_spec = DepSpec::path("../frankensympy", false).with_row_path("../frankensympy");
        let u_build = make_use(Posture::BuildTime, &["derive/"]);
        assert!(edge_allowed(&u_build, "derive/frob-derive", EdgeKind::Normal, &build_spec));
        assert!(edge_allowed(&u_build, "derive/frob-derive", EdgeKind::Build, &build_spec));
        assert!(!edge_allowed(&u_build, "frob-dyn", EdgeKind::Normal, &build_spec));
        assert!(!edge_allowed(&u_build, "frob-core", EdgeKind::Build, &build_spec));
        assert!(!edge_allowed(&u_build, "crates/frob-dyn", EdgeKind::Normal, &build_spec));
    }

    #[test]
    fn edge_spec_rules() {
        let u_consume = make_use(Posture::Consume, &["frob-chronicle"]);

        let exact_off = DepSpec::registry("=0.6.2", false).with_row_version("=0.6.2");
        assert!(edge_allowed(&u_consume, "frob-chronicle", EdgeKind::Normal, &exact_off));

        let caret_explicit = DepSpec::registry("^0.6", false).with_row_version("=0.6.2");
        assert!(!edge_allowed(&u_consume, "frob-chronicle", EdgeKind::Normal, &caret_explicit));

        // Bare "0.6.2" is a caret requirement by Cargo's reading and must be refused.
        let caret_implicit = DepSpec::registry("0.6.2", false).with_row_version("=0.6.2");
        assert!(!edge_allowed(&u_consume, "frob-chronicle", EdgeKind::Normal, &caret_implicit));

        let default_feats_on = DepSpec::registry("=0.6.2", true).with_row_version("=0.6.2");
        assert!(!edge_allowed(&u_consume, "frob-chronicle", EdgeKind::Normal, &default_feats_on));

        let wrong_exact = DepSpec::registry("=0.6.3", false).with_row_version("=0.6.2");
        assert!(!edge_allowed(&u_consume, "frob-chronicle", EdgeKind::Normal, &wrong_exact));

        let u_path = make_use(Posture::ConsumePath, &["frob-fsim"]);
        let good_path = DepSpec::path("../frankensim/crates/fs-ga", false)
            .with_row_path("../frankensim");
        assert!(edge_allowed(&u_path, "frob-fsim", EdgeKind::Normal, &good_path));

        let good_root_path = DepSpec::path("../frankensim", false).with_row_path("../frankensim");
        assert!(edge_allowed(&u_path, "frob-fsim", EdgeKind::Normal, &good_root_path));

        let path_default_on = DepSpec::path("../frankensim/crates/fs-ga", true)
            .with_row_path("../frankensim");
        assert!(!edge_allowed(&u_path, "frob-fsim", EdgeKind::Normal, &path_default_on));

        let git_source = DepSpec::git("https://github.com/example/frankensim.git", false)
            .with_row_path("../frankensim");
        assert!(!edge_allowed(&u_path, "frob-fsim", EdgeKind::Normal, &git_source));

        let other_path = DepSpec::path("../frankensqlite/crates/fsqlite", false)
            .with_row_path("../frankensim");
        assert!(!edge_allowed(&u_path, "frob-fsim", EdgeKind::Normal, &other_path));

        let mut path_with_version = good_path;
        path_with_version.version = Some("=0.1.0".to_owned());
        assert!(!edge_allowed(&u_path, "frob-fsim", EdgeKind::Normal, &path_with_version));
    }

    #[test]
    fn two_postures_for_one_sibling_roundtrip() {
        let fsqlite_uses = vec![
            Use {
                posture: Posture::Consume,
                purpose: "chronicle design ledger storage".to_owned(),
                confined_to: vec!["frob-chronicle".to_owned()],
                tier: Tier::Solid,
                first_needed: Some(Milestone::M0),
            },
            Use {
                posture: Posture::Pattern,
                purpose: "pager eviction policy design".to_owned(),
                confined_to: vec![],
                tier: Tier::Solid,
                first_needed: None,
            },
            Use {
                posture: Posture::Oracle,
                purpose: "pager policy differential conformance oracle".to_owned(),
                confined_to: vec!["frob-conformance".to_owned()],
                tier: Tier::Solid,
                first_needed: Some(Milestone::M1),
            },
        ];

        let rendered = format_uses_toml(&fsqlite_uses);
        let reparsed = parse_uses_toml(rendered.as_bytes(), "FD-FSQL")
            .expect("multi-posture FrankenSQLite use list must round-trip");
        assert_eq!(reparsed, fsqlite_uses);

        // Verify the distinct postures enforce distinct rules on the same sibling.
        let spec = DepSpec::registry("=0.5.1", false).with_row_version("=0.5.1");
        assert!(edge_allowed(&reparsed[0], "frob-chronicle", EdgeKind::Normal, &spec));
        assert!(!edge_allowed(&reparsed[1], "frob-chronicle", EdgeKind::Normal, &spec));
        assert!(edge_allowed(&reparsed[2], "frob-conformance", EdgeKind::Dev, &spec));
        assert!(!edge_allowed(&reparsed[2], "frob-conformance", EdgeKind::Normal, &spec));
    }

    #[test]
    fn posture_rules_table_covers_all_seven_postures() {
        assert_eq!(POSTURE_RULES.len(), Posture::ALL.len());
        for p in Posture::ALL {
            assert_eq!(rule_for(p).posture, p);
        }
    }
}
