//! Closed, versioned proof-kind registry for `frob-cert` (N12-02, ADR-LEMMA-04, INV-LEMMA-04).
//!
//! This module is the single source of truth for the registry version string,
//! all 27 registered proof kinds, the closed `claim.kind -> Scope` table,
//! class-cap enforcement, and `(kind, version, scope) -> Checker` dispatch.
//!
//! No runtime registration exists: every row type has private fields and no public
//! constructor.

use std::fmt;

use crate::json::JsonValue;
use crate::kinds::rules::{
    REFUSAL_CLASS_ABOVE_CAP, REFUSAL_UNKNOWN_PROOF_KIND, REPAIRS_CLASS_ABOVE_CAP,
    REPAIRS_DISABLED_KIND, REPAIRS_UNKNOWN_KIND, RULE_CATALOGUE, RuleId,
};

#[cfg(all(feature = "registry-fixture", not(debug_assertions)))]
compile_error!("frob-cert `registry-fixture` feature is test-only and forbidden in release builds");

/// The sole definition of the compiled proof-kind registry version in `frob-cert`.
pub const REGISTRY_VERSION: &str = "frob.cert.kinds/1";

/// Evidence class order (`Estimated < Tested < Certified < Proved`, §12.2 / D-5).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Class {
    Estimated,
    Tested,
    Certified,
    Proved,
}

impl Class {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Estimated => "Estimated",
            Self::Tested => "Tested",
            Self::Certified => "Certified",
            Self::Proved => "Proved",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "Estimated" => Some(Self::Estimated),
            "Tested" => Some(Self::Tested),
            "Certified" => Some(Self::Certified),
            "Proved" => Some(Self::Proved),
            _ => None,
        }
    }
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Claim scope governing which proof kinds may back a bundle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Scope {
    Kernel,
    Analysis,
    Mission,
    Study,
}

impl Scope {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Kernel => "kernel",
            Self::Analysis => "analysis",
            Self::Mission => "mission",
            Self::Study => "study",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "kernel" => Some(Self::Kernel),
            "analysis" => Some(Self::Analysis),
            "mission" => Some(Self::Mission),
            "study" => Some(Self::Study),
            _ => None,
        }
    }
}

impl fmt::Display for Scope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Program tier of a proof kind (`Solid`, `Frontier`, `Moonshot`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tier {
    Solid,
    Frontier,
    Moonshot,
}

impl Tier {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Solid => "S",
            Self::Frontier => "F",
            Self::Moonshot => "M",
        }
    }
}

impl fmt::Display for Tier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Lifecycle status of a registered proof kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KindStatus {
    Enabled,
    DisabledPendingCause,
    NotImplemented,
}

impl KindStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Enabled => "enabled",
            Self::DisabledPendingCause => "disabled-pending-cause",
            Self::NotImplemented => "not-implemented",
        }
    }
}

impl fmt::Display for KindStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// One row of the closed compiled proof-kind registry.
///
/// All fields are private and there is no public constructor so no code outside
/// `frob-cert` can register or fabricate a proof-kind row at run time:
///
/// ```compile_fail
/// use frob_cert::kinds::registry::{Class, KindRow, KindStatus, Scope, Tier};
/// let _row = KindRow {
///     name: "fabricated",
///     version: 1,
///     cap: Class::Certified,
///     capped_by_inputs: false,
///     tier: Tier::Solid,
///     checker_bead: "NONE",
///     scopes: &[Scope::Mission],
///     recomputes: "none",
///     requires_side_conditions: &[],
///     payload_schema: "none",
///     status: KindStatus::Enabled,
/// };
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KindRow {
    name: &'static str,
    version: u32,
    cap: Class,
    capped_by_inputs: bool,
    tier: Tier,
    checker_bead: &'static str,
    scopes: &'static [Scope],
    recomputes: &'static str,
    requires_side_conditions: &'static [&'static str],
    payload_schema: &'static str,
    status: KindStatus,
}

impl KindRow {
    pub const fn name(&self) -> &'static str {
        self.name
    }

    pub const fn version(&self) -> u32 {
        self.version
    }

    pub const fn cap(&self) -> Class {
        self.cap
    }

    pub const fn capped_by_inputs(&self) -> bool {
        self.capped_by_inputs
    }

    pub const fn cap_display(&self) -> &'static str {
        if self.capped_by_inputs {
            "Certified, capped by inputs"
        } else {
            self.cap.as_str()
        }
    }

    pub const fn tier(&self) -> Tier {
        self.tier
    }

    pub const fn checker_bead(&self) -> &'static str {
        self.checker_bead
    }

    pub const fn scopes(&self) -> &'static [Scope] {
        self.scopes
    }

    pub const fn recomputes(&self) -> &'static str {
        self.recomputes
    }

    pub const fn requires_side_conditions(&self) -> &'static [&'static str] {
        self.requires_side_conditions
    }

    pub const fn payload_schema(&self) -> &'static str {
        self.payload_schema
    }

    pub const fn status(&self) -> KindStatus {
        self.status
    }

    /// True iff `scope` is in this kind's declared scope list.
    pub fn allows_scope(&self, scope: Scope) -> bool {
        self.scopes.contains(&scope)
    }
}

/// One row of the closed `claim.kind -> Scope` mapping table.
///
/// Fields are private so no external code can mint a claim-kind entry:
///
/// ```compile_fail
/// use frob_cert::kinds::registry::{ClaimKindScope, Scope};
/// let _row = ClaimKindScope {
///     claim_kind: "invented",
///     scope: Scope::Analysis,
///     description: "none",
/// };
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClaimKindScope {
    claim_kind: &'static str,
    scope: Scope,
    description: &'static str,
}

impl ClaimKindScope {
    pub const fn claim_kind(&self) -> &'static str {
        self.claim_kind
    }

    pub const fn scope(&self) -> Scope {
        self.scope
    }

    pub const fn description(&self) -> &'static str {
        self.description
    }
}

/// Closed table mapping every admitted `claim.kind` value to its dispatch `Scope`
/// (N12-02 item 3 + round-4 slice S-04 review note resolving the `analysis` claim kinds).
///
/// Any `claim.kind` outside this table is rejected with `RuleId::KindOutOfScope` (`FROB-REF-823`).
pub const CLAIM_KIND_SCOPES: &[ClaimKindScope] = &[
    ClaimKindScope {
        claim_kind: "kernel-identity",
        scope: Scope::Kernel,
        description: "Exact algebraic or numeric kernel identity certificate (BD-027, N12-20)",
    },
    ClaimKindScope {
        claim_kind: "family-inclusion",
        scope: Scope::Mission,
        description: "Mission design-family inclusion certificate",
    },
    ClaimKindScope {
        claim_kind: "family-exclusion",
        scope: Scope::Mission,
        description: "Mission design-family exclusion certificate",
    },
    ClaimKindScope {
        claim_kind: "partition",
        scope: Scope::Mission,
        description: "Mission parameter-box partition certificate",
    },
    ClaimKindScope {
        claim_kind: "qualification-receipt",
        scope: Scope::Study,
        description: "Empirical study / qualification receipt (Tested cap, sample-coverage)",
    },
    ClaimKindScope {
        claim_kind: "analysis",
        scope: Scope::Analysis,
        description: "General model or mechanism analysis certificate",
    },
    ClaimKindScope {
        claim_kind: "card-instance",
        scope: Scope::Analysis,
        description: "Theorem-card instance certificate (e.g., T-19 inertia, T-38)",
    },
    ClaimKindScope {
        claim_kind: "law-convexity",
        scope: Scope::Analysis,
        description: "Per-law contact/joint potential convexity certificate (LawConvexity, N12-51 / T-08)",
    },
    ClaimKindScope {
        claim_kind: "energy-budget",
        scope: Scope::Analysis,
        description: "Mechanism energy-budget analysis certificate (N12-17 / T-28)",
    },
    ClaimKindScope {
        claim_kind: "theorem-card",
        scope: Scope::Analysis,
        description: "Sealed theorem-card proof certificate (e.g., lean-capsule, BD-381)",
    },
];

/// Verdict produced by registry dispatch, cap checks, and individual proof checkers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CheckVerdict {
    Accept,
    Reject {
        rule: RuleId,
        refusal: Option<&'static str>,
        repairs: &'static [&'static str],
        message: String,
    },
    Unsupported {
        kind: String,
        version: u32,
        message: String,
    },
}

impl CheckVerdict {
    pub const fn label(&self) -> &'static str {
        match self {
            Self::Accept => "accept",
            Self::Reject { .. } => "reject",
            Self::Unsupported { .. } => "unsupported",
        }
    }
}

/// Outcome of a full registry probe (`kind`, `version`, `claim_kind`, `class`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProbeOutcome {
    pub scope: Option<Scope>,
    pub scope_check: &'static str,
    pub cap_check: &'static str,
    pub verdict: CheckVerdict,
}

/// Trait implemented by every compiled proof-kind checker inside `frob-cert`.
pub trait Checker: Send + Sync {
    fn kind(&self) -> &'static str;
    fn version(&self) -> u32;
    fn check(&self, payload: &JsonValue) -> CheckVerdict;
}

const SCOPE_MISSION: &[Scope] = &[Scope::Mission];
const SCOPE_MISSION_STUDY: &[Scope] = &[Scope::Mission, Scope::Study];
const SCOPE_KERNEL_ANALYSIS: &[Scope] = &[Scope::Kernel, Scope::Analysis];
const SCOPE_KERNEL_ANALYSIS_MISSION: &[Scope] = &[Scope::Kernel, Scope::Analysis, Scope::Mission];

#[cfg(feature = "registry-fixture")]
const ALPHA_ROOT_STATUS: KindStatus = KindStatus::DisabledPendingCause;
#[cfg(not(feature = "registry-fixture"))]
const ALPHA_ROOT_STATUS: KindStatus = KindStatus::NotImplemented;

/// Closed table of all 27 proof kinds in this registry baseline.
pub const REGISTRY: &[KindRow] = &[
    KindRow {
        name: "poly-identity",
        version: 1,
        cap: Class::Certified,
        capped_by_inputs: false,
        tier: Tier::Solid,
        checker_bead: "BD-027",
        scopes: SCOPE_KERNEL_ANALYSIS,
        recomputes: "Normal-form equality of two polynomial expressions over Q",
        requires_side_conditions: &[],
        payload_schema: "frob.cert.payload.poly-identity/1",
        status: KindStatus::NotImplemented,
    },
    KindRow {
        name: "rational-ldlt",
        version: 1,
        cap: Class::Certified,
        capped_by_inputs: false,
        tier: Tier::Solid,
        checker_bead: "BD-027",
        scopes: SCOPE_KERNEL_ANALYSIS,
        recomputes: "Exact LDL^T with positive pivots",
        requires_side_conditions: &[],
        payload_schema: "frob.cert.payload.rational-ldlt/1",
        status: KindStatus::NotImplemented,
    },
    KindRow {
        name: "interval-box",
        version: 1,
        cap: Class::Certified,
        capped_by_inputs: false,
        tier: Tier::Frontier,
        checker_bead: "BD-132",
        scopes: SCOPE_MISSION,
        recomputes: "Outward-rounded enclosure of a constraint over a box (split set at most six parameters)",
        requires_side_conditions: &[],
        payload_schema: "frob.cert.payload.interval-box/1",
        status: KindStatus::NotImplemented,
    },
    KindRow {
        name: "interval-root",
        version: 1,
        cap: Class::Certified,
        capped_by_inputs: false,
        tier: Tier::Frontier,
        checker_bead: "N12-16",
        scopes: SCOPE_KERNEL_ANALYSIS,
        recomputes: "Existence and uniqueness of a root in a box",
        requires_side_conditions: &[],
        payload_schema: "frob.cert.payload.interval-root/1",
        status: KindStatus::NotImplemented,
    },
    KindRow {
        name: "taylor-box",
        version: 1,
        cap: Class::Certified,
        capped_by_inputs: false,
        tier: Tier::Frontier,
        checker_bead: "N12-16",
        scopes: SCOPE_MISSION,
        recomputes: "The enclosure of a constraint over a box by a Taylor model: remainder bound and range",
        requires_side_conditions: &[],
        payload_schema: "frob.cert.payload.taylor-box/1",
        status: KindStatus::NotImplemented,
    },
    KindRow {
        name: "sos-gram",
        version: 1,
        cap: Class::Certified,
        capped_by_inputs: false,
        tier: Tier::Frontier,
        checker_bead: "N12-16",
        scopes: SCOPE_MISSION,
        recomputes: "Nonnegativity of a polynomial on a box: exact polynomial identity of a rational Gram matrix and multipliers, and PSD by LDL^T over Q",
        requires_side_conditions: &[],
        payload_schema: "frob.cert.payload.sos-gram/1",
        status: KindStatus::NotImplemented,
    },
    KindRow {
        name: "lp-farkas",
        version: 1,
        cap: Class::Certified,
        capped_by_inputs: false,
        tier: Tier::Solid,
        checker_bead: "BD-132",
        scopes: SCOPE_MISSION,
        recomputes: "M^T y in the dual cone and r^T y < 0 in exact rationals (box form: y^T w > sum_i max(0, (A^T y)_i) * c_i)",
        requires_side_conditions: &[],
        payload_schema: "frob.cert.payload.lp-farkas/1",
        status: KindStatus::NotImplemented,
    },
    KindRow {
        name: "conic-farkas",
        version: 1,
        cap: Class::Certified,
        capped_by_inputs: false,
        tier: Tier::Frontier,
        checker_bead: "N12-16",
        scopes: SCOPE_MISSION,
        recomputes: "M^T y in the dual second-order cone by exact squaring; strict separation",
        requires_side_conditions: &[],
        payload_schema: "frob.cert.payload.conic-farkas/1",
        status: KindStatus::NotImplemented,
    },
    KindRow {
        name: "lp-primal",
        version: 1,
        cap: Class::Certified,
        capped_by_inputs: false,
        tier: Tier::Solid,
        checker_bead: "BD-132",
        scopes: SCOPE_MISSION,
        recomputes: "An explicit static witness against the primitive equations at every vertex, with the convexity or monotonicity argument named",
        requires_side_conditions: &[],
        payload_schema: "frob.cert.payload.lp-primal/1",
        status: KindStatus::NotImplemented,
    },
    KindRow {
        name: "finite-game",
        version: 1,
        cap: Class::Certified,
        capped_by_inputs: false,
        tier: Tier::Solid,
        checker_bead: "BD-132",
        scopes: SCOPE_MISSION,
        recomputes: "A policy table against every world and error vertex; equal histories receive equal commands",
        requires_side_conditions: &[],
        payload_schema: "frob.cert.payload.finite-game/1",
        status: KindStatus::NotImplemented,
    },
    KindRow {
        name: "trace-witness",
        version: 1,
        cap: Class::Tested,
        capped_by_inputs: false,
        tier: Tier::Solid,
        checker_bead: "BD-132",
        scopes: SCOPE_MISSION,
        recomputes: "Format and digest of a replayed execution",
        requires_side_conditions: &[],
        payload_schema: "frob.cert.payload.trace-witness/1",
        status: KindStatus::NotImplemented,
    },
    KindRow {
        name: "partition",
        version: 1,
        cap: Class::Certified,
        capped_by_inputs: false,
        tier: Tier::Solid,
        checker_bead: "BD-132",
        scopes: SCOPE_MISSION,
        recomputes: "That the listed cells cover the claimed region, interiors disjoint, and that each carries its own certificate",
        requires_side_conditions: &["cell-certificates"],
        payload_schema: "frob.cert.payload.partition/1",
        status: KindStatus::NotImplemented,
    },
    KindRow {
        name: "monotone-transport",
        version: 1,
        cap: Class::Certified,
        capped_by_inputs: false,
        tier: Tier::Frontier,
        checker_bead: "N12-14",
        scopes: SCOPE_MISSION,
        recomputes: "A recorded monotonicity or polarity implication between two cells or two assumption sets",
        requires_side_conditions: &["source-certificate"],
        payload_schema: "frob.cert.payload.monotone-transport/1",
        status: KindStatus::NotImplemented,
    },
    KindRow {
        name: "symmetry-transport",
        version: 1,
        cap: Class::Certified,
        capped_by_inputs: false,
        tier: Tier::Frontier,
        checker_bead: "N12-14",
        scopes: SCOPE_MISSION,
        recomputes: "The exact images of a verdict under a signed permutation, with invariance of the mission, world set, and observation map",
        requires_side_conditions: &["source-certificate"],
        payload_schema: "frob.cert.payload.symmetry-transport/1",
        status: KindStatus::NotImplemented,
    },
    KindRow {
        name: "composition",
        version: 1,
        cap: Class::Certified,
        capped_by_inputs: false,
        tier: Tier::Frontier,
        checker_bead: "N12-14",
        scopes: SCOPE_MISSION,
        recomputes: "Each end-set subset entry-set inclusion of a chain of stage contracts; no clock or resource spent twice",
        requires_side_conditions: &["stage-certificates"],
        payload_schema: "frob.cert.payload.composition/1",
        status: KindStatus::NotImplemented,
    },
    KindRow {
        name: "monomial-product",
        version: 1,
        cap: Class::Certified,
        capped_by_inputs: false,
        tier: Tier::Frontier,
        checker_bead: "N12-14",
        scopes: SCOPE_MISSION,
        recomputes: "The monomial screen in exponent coordinates, with a cell's box bounds entered as constraints",
        requires_side_conditions: &[],
        payload_schema: "frob.cert.payload.monomial-product/1",
        status: KindStatus::NotImplemented,
    },
    KindRow {
        name: "complex-connectivity",
        version: 1,
        cap: Class::Certified,
        capped_by_inputs: false,
        tier: Tier::Solid,
        checker_bead: "N12-15",
        scopes: SCOPE_KERNEL_ANALYSIS,
        recomputes: "A path inside the inner complex, or component labels of the outer complex",
        requires_side_conditions: &[],
        payload_schema: "frob.cert.payload.complex-connectivity/1",
        status: KindStatus::NotImplemented,
    },
    KindRow {
        name: "tube-inclusion",
        version: 1,
        cap: Class::Certified,
        capped_by_inputs: false,
        tier: Tier::Moonshot,
        checker_bead: "N12-19",
        scopes: SCOPE_KERNEL_ANALYSIS,
        recomputes: "The tube inclusion for every cell and enabled guard of a table",
        requires_side_conditions: &[],
        payload_schema: "frob.cert.payload.tube-inclusion/1",
        status: KindStatus::NotImplemented,
    },
    KindRow {
        name: "rewrite-trace",
        version: 1,
        cap: Class::Certified,
        capped_by_inputs: false,
        tier: Tier::Frontier,
        checker_bead: "N12p2-02",
        scopes: SCOPE_KERNEL_ANALYSIS,
        recomputes: "Replay of a class-E rule trace from source to target",
        requires_side_conditions: &[],
        payload_schema: "frob.cert.payload.rewrite-trace/1",
        status: KindStatus::NotImplemented,
    },
    KindRow {
        name: "orbit-count",
        version: 1,
        cap: Class::Certified,
        capped_by_inputs: false,
        tier: Tier::Solid,
        checker_bead: "N12-18",
        scopes: SCOPE_KERNEL_ANALYSIS,
        recomputes: "Orbit representatives and counts",
        requires_side_conditions: &[],
        payload_schema: "frob.cert.payload.orbit-count/1",
        status: KindStatus::NotImplemented,
    },
    KindRow {
        name: "group-enumeration",
        version: 1,
        cap: Class::Certified,
        capped_by_inputs: false,
        tier: Tier::Frontier,
        checker_bead: "N12-18",
        scopes: SCOPE_KERNEL_ANALYSIS,
        recomputes: "The subgroup lattice of a small group",
        requires_side_conditions: &[],
        payload_schema: "frob.cert.payload.group-enumeration/1",
        status: KindStatus::NotImplemented,
    },
    KindRow {
        name: "energy-budget",
        version: 1,
        cap: Class::Certified,
        capped_by_inputs: true,
        tier: Tier::Solid,
        checker_bead: "N12-17",
        scopes: SCOPE_KERNEL_ANALYSIS_MISSION,
        recomputes: "The energy-budget inequality with its error bounds and its list of sources",
        requires_side_conditions: &[],
        payload_schema: "frob.cert.payload.energy-budget/1",
        status: KindStatus::NotImplemented,
    },
    KindRow {
        name: "alias-pair",
        version: 1,
        cap: Class::Certified,
        capped_by_inputs: false,
        tier: Tier::Solid,
        checker_bead: "BD-132",
        scopes: SCOPE_MISSION,
        recomputes: "Two exact replays with identical packet histories and disjoint success sets",
        requires_side_conditions: &[],
        payload_schema: "frob.cert.payload.alias-pair/1",
        status: KindStatus::NotImplemented,
    },
    KindRow {
        name: "product-unreachable",
        version: 1,
        cap: Class::Certified,
        capped_by_inputs: false,
        tier: Tier::Frontier,
        checker_bead: "N12-17",
        scopes: SCOPE_MISSION,
        recomputes: "Exact case analysis or enclosure of a product model's outcome pairs",
        requires_side_conditions: &[],
        payload_schema: "frob.cert.payload.product-unreachable/1",
        status: KindStatus::NotImplemented,
    },
    KindRow {
        name: "alpha-root",
        version: 1,
        cap: Class::Certified,
        capped_by_inputs: false,
        tier: Tier::Moonshot,
        checker_bead: "N12p1-01",
        scopes: SCOPE_KERNEL_ANALYSIS,
        recomputes: "The alpha bound of a rational point, with Smale's theorem as a named assumption",
        requires_side_conditions: &["smale-assumption"],
        payload_schema: "frob.cert.payload.alpha-root/1",
        status: ALPHA_ROOT_STATUS,
    },
    KindRow {
        name: "lean-capsule",
        version: 1,
        cap: Class::Proved,
        capped_by_inputs: false,
        tier: Tier::Frontier,
        checker_bead: "BD-381",
        scopes: SCOPE_KERNEL_ANALYSIS,
        recomputes: "The hash of a sealed FrankenLean capsule and the statement it proves; the capsule is checked by fln",
        requires_side_conditions: &["sealed-capsule"],
        payload_schema: "frob.cert.payload.lean-capsule/1",
        status: KindStatus::NotImplemented,
    },
    KindRow {
        name: "sample-coverage",
        version: 1,
        cap: Class::Tested,
        capped_by_inputs: false,
        tier: Tier::Solid,
        checker_bead: "N12-13",
        scopes: SCOPE_MISSION_STUDY,
        recomputes: "The arithmetic of a recorded study (counts, e-values, spent alpha). Never a proof",
        requires_side_conditions: &[],
        payload_schema: "frob.cert.payload.sample-coverage/1",
        status: KindStatus::NotImplemented,
    },
];

/// Return all currently enabled `(kind, version)` pairs in compiled table order.
pub fn enabled_kinds() -> Vec<(&'static str, u32)> {
    REGISTRY
        .iter()
        .filter(|row| row.status == KindStatus::Enabled)
        .map(|row| (row.name, row.version))
        .collect()
}

/// Look up the first row matching `kind` regardless of version.
pub fn lookup(kind: &str) -> Option<&'static KindRow> {
    REGISTRY.iter().find(|row| row.name == kind)
}

/// Look up the row matching exact `(kind, version)`.
pub fn lookup_version(kind: &str, version: u32) -> Option<&'static KindRow> {
    REGISTRY
        .iter()
        .find(|row| row.name == kind && row.version == version)
}

/// Resolve a bundle's `claim.kind` string into its dispatch `Scope` through the
/// closed `CLAIM_KIND_SCOPES` table. Any unlisted claim kind is rejected with
/// `RuleId::KindOutOfScope` (`FROB-REF-823`).
pub fn scope_for_claim_kind(claim_kind: &str) -> Result<Scope, CheckVerdict> {
    if let Some(entry) = CLAIM_KIND_SCOPES
        .iter()
        .find(|entry| entry.claim_kind == claim_kind)
    {
        return Ok(entry.scope);
    }
    Err(CheckVerdict::Reject {
        rule: RuleId::KindOutOfScope,
        refusal: Some(REFUSAL_UNKNOWN_PROOF_KIND),
        repairs: REPAIRS_UNKNOWN_KIND,
        message: format!("claim.kind {claim_kind:?} is not in the closed claim-kind scope table"),
    })
}

/// Check that a bundle's claimed `Class` does not exceed the proof kind's compiled cap.
pub fn check_class_cap(row: &KindRow, claimed: Class) -> Result<(), CheckVerdict> {
    if claimed > row.cap {
        return Err(CheckVerdict::Reject {
            rule: RuleId::ClassAboveKindCap,
            refusal: Some(REFUSAL_CLASS_ABOVE_CAP),
            repairs: REPAIRS_CLASS_ABOVE_CAP,
            message: format!(
                "claim class {} exceeds cap {} of proof kind {} v{}",
                claimed.as_str(),
                row.cap.as_str(),
                row.name,
                row.version
            ),
        });
    }
    Ok(())
}

#[cfg(feature = "shortcut-accept-on-hashes")]
struct ShortcutAcceptOnHashesMutant;

#[cfg(feature = "shortcut-accept-on-hashes")]
impl Checker for ShortcutAcceptOnHashesMutant {
    fn kind(&self) -> &'static str {
        "<shortcut-accept-on-hashes-mutant>"
    }

    fn version(&self) -> u32 {
        0
    }

    fn check(&self, _payload: &JsonValue) -> CheckVerdict {
        CheckVerdict::Accept
    }
}

#[cfg(feature = "shortcut-accept-on-hashes")]
static SHORTCUT_ACCEPT_ON_HASHES_MUTANT: ShortcutAcceptOnHashesMutant =
    ShortcutAcceptOnHashesMutant;

/// Closed `(kind, version, scope) -> &'static dyn Checker` dispatch against `REGISTRY`.
///
/// Never reads a digest: an unknown kind is always rejected with `RuleId::KindUnknown`
/// (`FROB-REF-823`), an unknown version with `RuleId::KindVersionUnknown` (`FROB-REF-823`),
/// out-of-scope use with `RuleId::KindOutOfScope` (`FROB-REF-823`), a disabled kind with
/// `RuleId::KindDisabled` (`FROB-REF-823`), and a `not-implemented` kind with
/// `CheckVerdict::Unsupported` (never `Accept`).
pub fn dispatch(kind: &str, version: u32, scope: Scope) -> Result<&'static dyn Checker, CheckVerdict> {
    dispatch_in_table(REGISTRY, kind, version, scope)
}

fn dispatch_in_table(
    rows: &'static [KindRow],
    kind: &str,
    version: u32,
    scope: Scope,
) -> Result<&'static dyn Checker, CheckVerdict> {
    let any_name = rows.iter().any(|row| row.name == kind);
    if !any_name {
        #[cfg(feature = "shortcut-accept-on-hashes")]
        {
            return Ok(&SHORTCUT_ACCEPT_ON_HASHES_MUTANT);
        }
        #[cfg(not(feature = "shortcut-accept-on-hashes"))]
        {
            return Err(CheckVerdict::Reject {
                rule: RuleId::KindUnknown,
                refusal: Some(REFUSAL_UNKNOWN_PROOF_KIND),
                repairs: REPAIRS_UNKNOWN_KIND,
                message: format!("unknown proof kind {kind:?} not in {REGISTRY_VERSION}"),
            });
        }
    }

    let Some(row) = rows
        .iter()
        .find(|row| row.name == kind && row.version == version)
    else {
        return Err(CheckVerdict::Reject {
            rule: RuleId::KindVersionUnknown,
            refusal: Some(REFUSAL_UNKNOWN_PROOF_KIND),
            repairs: REPAIRS_UNKNOWN_KIND,
            message: format!(
                "proof kind {kind:?} has no version {version} in {REGISTRY_VERSION}"
            ),
        });
    };

    if !row.allows_scope(scope) {
        return Err(CheckVerdict::Reject {
            rule: RuleId::KindOutOfScope,
            refusal: Some(REFUSAL_UNKNOWN_PROOF_KIND),
            repairs: REPAIRS_UNKNOWN_KIND,
            message: format!(
                "proof kind {} v{} is not permitted under scope {}",
                row.name,
                row.version,
                scope.as_str()
            ),
        });
    }

    match row.status {
        KindStatus::DisabledPendingCause => Err(CheckVerdict::Reject {
            rule: RuleId::KindDisabled,
            refusal: Some(REFUSAL_UNKNOWN_PROOF_KIND),
            repairs: REPAIRS_DISABLED_KIND,
            message: format!(
                "proof kind {} v{} is disabled pending cause investigation",
                row.name, row.version
            ),
        }),
        KindStatus::NotImplemented => Err(CheckVerdict::Unsupported {
            kind: row.name.to_string(),
            version: row.version,
            message: format!(
                "proof kind {} v{} is registered (checker {}) but not implemented in this build",
                row.name, row.version, row.checker_bead
            ),
        }),
        KindStatus::Enabled => Err(CheckVerdict::Unsupported {
            kind: row.name.to_string(),
            version: row.version,
            message: format!(
                "proof kind {} v{} enabled without compiled checker",
                row.name, row.version
            ),
        }),
    }
}

/// Evaluate the full closed-registry decision chain for `(kind, version, claim_kind, class)`.
///
/// Order:
/// 1. Kind existence (`kind.unknown`, FROB-REF-823)
/// 2. Version existence (`kind.version-unknown`, FROB-REF-823)
/// 3. Claim-kind -> scope resolution and row scope check (`kind.out-of-scope`, FROB-REF-823)
/// 4. Class cap check (`class.above-kind-cap`, FROB-REF-834)
/// 5. Lifecycle status & checker dispatch (`kind.disabled`, `unsupported`, or enabled checker)
pub fn probe(kind: &str, version: u32, claim_kind: &str, claimed_class: Class) -> ProbeOutcome {
    if lookup(kind).is_none() {
        #[cfg(feature = "shortcut-accept-on-hashes")]
        {
            return ProbeOutcome {
                scope: None,
                scope_check: "skipped",
                cap_check: "skipped",
                verdict: CheckVerdict::Accept,
            };
        }
        #[cfg(not(feature = "shortcut-accept-on-hashes"))]
        {
            return ProbeOutcome {
                scope: None,
                scope_check: "not-reached",
                cap_check: "not-reached",
                verdict: CheckVerdict::Reject {
                    rule: RuleId::KindUnknown,
                    refusal: Some(REFUSAL_UNKNOWN_PROOF_KIND),
                    repairs: REPAIRS_UNKNOWN_KIND,
                    message: format!("unknown proof kind {kind:?} not in {REGISTRY_VERSION}"),
                },
            };
        }
    }

    let Some(row) = lookup_version(kind, version) else {
        return ProbeOutcome {
            scope: None,
            scope_check: "not-reached",
            cap_check: "not-reached",
            verdict: CheckVerdict::Reject {
                rule: RuleId::KindVersionUnknown,
                refusal: Some(REFUSAL_UNKNOWN_PROOF_KIND),
                repairs: REPAIRS_UNKNOWN_KIND,
                message: format!(
                    "proof kind {kind:?} has no version {version} in {REGISTRY_VERSION}"
                ),
            },
        };
    };

    let scope = match scope_for_claim_kind(claim_kind) {
        Ok(s) => s,
        Err(verdict) => {
            return ProbeOutcome {
                scope: None,
                scope_check: "fail",
                cap_check: "not-reached",
                verdict,
            };
        }
    };

    if !row.allows_scope(scope) {
        return ProbeOutcome {
            scope: Some(scope),
            scope_check: "fail",
            cap_check: "not-reached",
            verdict: CheckVerdict::Reject {
                rule: RuleId::KindOutOfScope,
                refusal: Some(REFUSAL_UNKNOWN_PROOF_KIND),
                repairs: REPAIRS_UNKNOWN_KIND,
                message: format!(
                    "proof kind {} v{} is not permitted under claim.kind {} (scope {})",
                    row.name,
                    row.version,
                    claim_kind,
                    scope.as_str()
                ),
            },
        };
    }

    if let Err(verdict) = check_class_cap(row, claimed_class) {
        return ProbeOutcome {
            scope: Some(scope),
            scope_check: "pass",
            cap_check: "fail",
            verdict,
        };
    }

    let verdict = match dispatch(kind, version, scope) {
        Ok(_checker) => CheckVerdict::Accept,
        Err(v) => v,
    };

    ProbeOutcome {
        scope: Some(scope),
        scope_check: "pass",
        cap_check: "pass",
        verdict,
    }
}

/// Deterministically render `docs/registries/cert-kinds.md` from the compiled `const` tables.
pub fn render_registry_markdown() -> String {
    let mut out = String::new();
    out.push_str("# FrankenRobotics Certificate Proof-Kind Registry\n\n");
    out.push_str(&format!("- **Registry version:** `{REGISTRY_VERSION}`\n"));
    out.push_str(&format!("- **Registered proof kinds:** `{}`\n", REGISTRY.len()));
    let enabled = enabled_kinds();
    out.push_str(&format!("- **Enabled checkers in this build:** `{}`\n", enabled.len()));
    out.push_str("- **Determinism class:** `D0`\n");
    out.push_str("- **Authority:** Generated directly from `crates/frob-cert/src/kinds/registry.rs` (`cargo run -p frob-cert --example kind_registry -- --doc`). Do not edit by hand.\n\n");

    out.push_str("## 1. Proof Kinds\n\n");
    out.push_str("| Kind | Version | Cap | Tier | Scope | Status | Checker | Side Conditions | What the auditor recomputes |\n");
    out.push_str("|---|---:|---|---|---|---|---|---|---|\n");
    for row in REGISTRY {
        let scopes: Vec<&str> = row.scopes.iter().map(|s| s.as_str()).collect();
        let side = if row.requires_side_conditions.is_empty() {
            "-".to_string()
        } else {
            row.requires_side_conditions.join(", ")
        };
        out.push_str(&format!(
            "| `{}` | {} | {} | {} | {} | `{}` | {} | {} | {} |\n",
            row.name,
            row.version,
            row.cap_display(),
            row.tier.as_str(),
            scopes.join(", "),
            row.status.as_str(),
            row.checker_bead,
            side,
            row.recomputes,
        ));
    }

    out.push_str("\n## 2. Closed Claim-Kind Scope Table\n\n");
    out.push_str("Every bundle's `claim.kind` must appear in this closed table; any unlisted `claim.kind` is rejected with `kind.out-of-scope` (`FROB-REF-823`).\n\n");
    out.push_str("| `claim.kind` | Scope | Meaning |\n");
    out.push_str("|---|---|---|\n");
    for entry in CLAIM_KIND_SCOPES {
        out.push_str(&format!(
            "| `{}` | `{}` | {} |\n",
            entry.claim_kind,
            entry.scope.as_str(),
            entry.description,
        ));
    }

    out.push_str("\n## 3. Closed Rule Identifier Catalogue\n\n");
    out.push_str("Every rejection names a `RuleId` from this closed catalogue (`<area>.<rule>`).\n\n");
    out.push_str("| `RuleId` | Area | Refusal Code |\n");
    out.push_str("|---|---|---|\n");
    for &rule in RULE_CATALOGUE {
        let refusal = rule.refusal_code().unwrap_or("-");
        out.push_str(&format!(
            "| `{}` | `{}` | {} |\n",
            rule.as_str(),
            rule.area(),
            refusal,
        ));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    static DISABLED_FIXTURE_TABLE: &[KindRow] = &[KindRow {
        name: "fixture-disabled-kind",
        version: 1,
        cap: Class::Certified,
        capped_by_inputs: false,
        tier: Tier::Solid,
        checker_bead: "FIXTURE",
        scopes: SCOPE_KERNEL_ANALYSIS,
        recomputes: "synthetic disabled row fixture",
        requires_side_conditions: &[],
        payload_schema: "frob.cert.payload.fixture/1",
        status: KindStatus::DisabledPendingCause,
    }];

    #[test]
    fn disabled_kind_rejects_in_table() {
        let err = dispatch_in_table(
            DISABLED_FIXTURE_TABLE,
            "fixture-disabled-kind",
            1,
            Scope::Kernel,
        )
        .err()
        .expect("disabled row must not dispatch to an executable checker");
        match err {
            CheckVerdict::Reject {
                rule,
                refusal,
                repairs,
                ..
            } => {
                assert_eq!(rule, RuleId::KindDisabled);
                assert_eq!(refusal, Some(REFUSAL_UNKNOWN_PROOF_KIND));
                assert_eq!(repairs, REPAIRS_DISABLED_KIND);
            }
            other => panic!("expected Reject(KindDisabled), got {other:?}"),
        }
    }
}
