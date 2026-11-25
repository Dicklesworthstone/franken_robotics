#![deny(unsafe_code)]

//! Refusal vocabulary of the Gauntlet (`N38-REF`, §38.10–§38.14, Appendix O, `ADR-GAUNT-01`).
//!
//! By decision (`ADR-GAUNT-01`), `frob-conformance` exposes no agent verb and defines
//! **no new public `FROB-REF-nnn` refusal codes**. Instead, this module defines:
//!
//! 1. [`HarnessReason`]: the closed enumeration of Gauntlet harness reasons, bound 1-to-1
//!    to `crates/frob-conformance/registry/reasons.toml` (schema `frob.gauntlet.reasons/1`).
//! 2. [`GAUNTLET_PUBLIC_CODES`]: the existing public `FROB-REF-nnn` codes defined by other
//!    chapters that the Gauntlet's rules rely on, with their family exit rules.
//! 3. Validators and scanners for source trees (`reason=<id>`) and `STD-E2E` `refusal` log
//!    events.
//!
//! Determinism: D0. Prints nothing (rule L-7). Zero `unsafe`.

use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

use frob_core::registry::refusals::RefusalRegistry;
use frob_core::registry::toml::{self, Fields, Spanned, Table, TomlError, Value};
use frob_core::testkit::registry as testkit_registry;

use crate::e2e::json::{self, JsonValue};

/// Canonical workspace-relative path of the Gauntlet harness reason registry.
pub const REASONS_PATH: &str = "crates/frob-conformance/registry/reasons.toml";

/// Canonical workspace-relative path of the public refusal registry.
pub const PUBLIC_REFUSALS_PATH: &str = "registries/refusals.toml";

/// Expected schema header for `crates/frob-conformance/registry/reasons.toml`.
pub const REASONS_SCHEMA: &str = "frob.gauntlet.reasons/1";

/// Embedded copy of `crates/frob-conformance/registry/reasons.toml`.
pub const EMBEDDED_REASONS_TOML: &str = include_str!("../registry/reasons.toml");

/// Admitted process exit codes for Gauntlet harness reasons (N38-REF).
///
/// Exit code `1` is never emitted on purpose anywhere in the program.
pub const VALID_HARNESS_EXITS: &[u8] = &[0, 3, 101];

/// Locate the active workspace root, honoring scratch-registry overrides (`FROB_REGISTRY_ROOT`,
/// `FROB_CONFORMANCE_ROOT`) before falling back to `frob_core::testkit::registry::root()`.
#[must_use]
pub fn workspace_root() -> PathBuf {
    for var in ["FROB_REGISTRY_ROOT", "FROB_CONFORMANCE_ROOT"] {
        if let Some(ov) = env::var_os(var)
            && !ov.is_empty()
        {
            return PathBuf::from(ov);
        }
    }
    testkit_registry::root().expect("locate workspace root via franken-stack.lock")
}

/// Classification of a harness reason entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ReasonKind {
    Refusal,
    Warning,
    Flag,
    Finding,
    Outcome,
    Answer,
}

impl ReasonKind {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Refusal => "refusal",
            Self::Warning => "warning",
            Self::Flag => "flag",
            Self::Finding => "finding",
            Self::Outcome => "outcome",
            Self::Answer => "answer",
        }
    }

    #[must_use]
    pub fn parse(raw: &str) -> Option<Self> {
        match raw {
            "refusal" => Some(Self::Refusal),
            "warning" => Some(Self::Warning),
            "flag" => Some(Self::Flag),
            "finding" => Some(Self::Finding),
            "outcome" => Some(Self::Outcome),
            "answer" => Some(Self::Answer),
            _ => None,
        }
    }
}

/// Closed enumeration of every Gauntlet harness reason registered by `N38-REF`.
///
/// Any new harness reason requires a recorded addition here and in
/// `crates/frob-conformance/registry/reasons.toml`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HarnessReason {
    // 1. Registry check (4)
    NoControl,
    DanglingControl,
    OneWayLink,
    UncaughtControl,
    // 2. Control loader (10)
    NoGate,
    UnknownGate,
    KindNotClosed,
    DefectOrExpectEmpty,
    FeatureMissing,
    FixtureMissing,
    UnknownSeedForm,
    BoundaryNotControl,
    CannotFail,
    PermanentControlRemoved,
    // 3. Control runner, tier runner, and control review (6; includes Note 6 `not-yet-due`)
    Missed,
    CaughtUnexpected,
    Stale,
    NotYetDue,
    NotEligible,
    SuspectUndisposed,
    // 4. Gate recorder core (15)
    ReceiptHashMismatch,
    ReceiptDirtyTree,
    ReceiptCommitUnknown,
    GateLineNotEligible,
    PassWithoutControl,
    ControlMissingFromReceipt,
    ControlNotCaught,
    SelfCertified,
    EvidenceUnresolved,
    ReceiptStackDev,
    DevBuildIdentity,
    ReceiptSetInvalid,
    PartRowsDropped,
    ReceiptNotOfRecord,
    ReducedNotRecordable,
    // 5. Seeding join (3)
    UnmatchedSeed,
    OrphanControl,
    RuleWithoutGate,
    // 6. Registry diff (5)
    Removed,
    Reused,
    Renumbered,
    OutOfOrder,
    Gap,
    // 7. Close audit (8; includes Note 6 `export-format-unreadable`)
    Untyped,
    ForeignTest,
    NoOutput,
    SelfClosed,
    SharedReason,
    SharedOutput,
    Burst,
    ExportFormatUnreadable,
    // 8. Live-dependency helper & status views (3)
    StandInSuspected,
    UnboundDependency,
    ViewsDiffer,
    // 9. Emitting-bead reasons after `reduced-not-recordable` (23)
    ClaimEvidenceMismatch,
    NoPotencyRecord,
    NotPotent,
    OriginLabelMismatch,
    OriginNotAdmissible,
    KaSecondFile,
    BoundaryOnly,
    RelationWithoutControl,
    ControlWithoutRelation,
    MutationSurvived,
    MutationStale,
    DueUnbound,
    UnexpectedOwner,
    UnallocatedNumber,
    MissingNumber,
    ProvisionalTagPastDue,
    CxCountMismatch,
    SameAsInvalid,
    CaseSupplierInvalid,
    FactWithoutRerun,
    PlantMissing,
    RerunDisagreesWithPilot,
    PlantChanged,
    // 10. Gate-register write validator (7 new; 4 shared in §4)
    TargetAsMeasurement,
    ClaimNotOneOfSeven,
    NoClaimOnNonReality,
    BlockedWithoutQuestion,
    KilledWithoutDecision,
    DuplicateId,
    ReusedId,
    // 11. Pilot funding check & Moonshot cap (11)
    Missing,
    Unregistered,
    Void,
    Registered,
    Running,
    FalsifierHolds,
    Inconclusive,
    KillGateToRegister,
    MoonshotCap,
    PlanTimeDiffers,
    PlanTimeNotRun,
    // 12. Pilot register loader (10 new; `reused` & `removed` shared in §6)
    PilotIdForm,
    PilotPartMissing,
    PilotCannotFail,
    ResultProtocolMismatch,
    ResultBeforeRegistration,
    PilotReceiptUncommitted,
    FallbackTierNotLower,
    FallbackAlsoUnknown,
    TierSWithoutRequiredBy,
    PlanTimeRecordUnknown,
}

impl HarnessReason {
    /// All 105 registered Gauntlet harness reasons in canonical registry order.
    pub const ALL: &'static [Self] = &[
        Self::NoControl,
        Self::DanglingControl,
        Self::OneWayLink,
        Self::UncaughtControl,
        Self::NoGate,
        Self::UnknownGate,
        Self::KindNotClosed,
        Self::DefectOrExpectEmpty,
        Self::FeatureMissing,
        Self::FixtureMissing,
        Self::UnknownSeedForm,
        Self::BoundaryNotControl,
        Self::CannotFail,
        Self::PermanentControlRemoved,
        Self::Missed,
        Self::CaughtUnexpected,
        Self::Stale,
        Self::NotYetDue,
        Self::NotEligible,
        Self::SuspectUndisposed,
        Self::ReceiptHashMismatch,
        Self::ReceiptDirtyTree,
        Self::ReceiptCommitUnknown,
        Self::GateLineNotEligible,
        Self::PassWithoutControl,
        Self::ControlMissingFromReceipt,
        Self::ControlNotCaught,
        Self::SelfCertified,
        Self::EvidenceUnresolved,
        Self::ReceiptStackDev,
        Self::DevBuildIdentity,
        Self::ReceiptSetInvalid,
        Self::PartRowsDropped,
        Self::ReceiptNotOfRecord,
        Self::ReducedNotRecordable,
        Self::UnmatchedSeed,
        Self::OrphanControl,
        Self::RuleWithoutGate,
        Self::Removed,
        Self::Reused,
        Self::Renumbered,
        Self::OutOfOrder,
        Self::Gap,
        Self::Untyped,
        Self::ForeignTest,
        Self::NoOutput,
        Self::SelfClosed,
        Self::SharedReason,
        Self::SharedOutput,
        Self::Burst,
        Self::ExportFormatUnreadable,
        Self::StandInSuspected,
        Self::UnboundDependency,
        Self::ViewsDiffer,
        Self::ClaimEvidenceMismatch,
        Self::NoPotencyRecord,
        Self::NotPotent,
        Self::OriginLabelMismatch,
        Self::OriginNotAdmissible,
        Self::KaSecondFile,
        Self::BoundaryOnly,
        Self::RelationWithoutControl,
        Self::ControlWithoutRelation,
        Self::MutationSurvived,
        Self::MutationStale,
        Self::DueUnbound,
        Self::UnexpectedOwner,
        Self::UnallocatedNumber,
        Self::MissingNumber,
        Self::ProvisionalTagPastDue,
        Self::CxCountMismatch,
        Self::SameAsInvalid,
        Self::CaseSupplierInvalid,
        Self::FactWithoutRerun,
        Self::PlantMissing,
        Self::RerunDisagreesWithPilot,
        Self::PlantChanged,
        Self::TargetAsMeasurement,
        Self::ClaimNotOneOfSeven,
        Self::NoClaimOnNonReality,
        Self::BlockedWithoutQuestion,
        Self::KilledWithoutDecision,
        Self::DuplicateId,
        Self::ReusedId,
        Self::Missing,
        Self::Unregistered,
        Self::Void,
        Self::Registered,
        Self::Running,
        Self::FalsifierHolds,
        Self::Inconclusive,
        Self::KillGateToRegister,
        Self::MoonshotCap,
        Self::PlanTimeDiffers,
        Self::PlanTimeNotRun,
        Self::PilotIdForm,
        Self::PilotPartMissing,
        Self::PilotCannotFail,
        Self::ResultProtocolMismatch,
        Self::ResultBeforeRegistration,
        Self::PilotReceiptUncommitted,
        Self::FallbackTierNotLower,
        Self::FallbackAlsoUnknown,
        Self::TierSWithoutRequiredBy,
        Self::PlanTimeRecordUnknown,
    ];

    /// Canonical kebab-case wire identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoControl => "no-control",
            Self::DanglingControl => "dangling-control",
            Self::OneWayLink => "one-way-link",
            Self::UncaughtControl => "uncaught-control",
            Self::NoGate => "no-gate",
            Self::UnknownGate => "unknown-gate",
            Self::KindNotClosed => "kind-not-closed",
            Self::DefectOrExpectEmpty => "defect-or-expect-empty",
            Self::FeatureMissing => "feature-missing",
            Self::FixtureMissing => "fixture-missing",
            Self::UnknownSeedForm => "unknown-seed-form",
            Self::BoundaryNotControl => "boundary-not-control",
            Self::CannotFail => "cannot-fail",
            Self::PermanentControlRemoved => "permanent-control-removed",
            Self::Missed => "missed",
            Self::CaughtUnexpected => "caught-unexpected",
            Self::Stale => "stale",
            Self::NotYetDue => "not-yet-due",
            Self::NotEligible => "not-eligible",
            Self::SuspectUndisposed => "suspect-undisposed",
            Self::ReceiptHashMismatch => "receipt-hash-mismatch",
            Self::ReceiptDirtyTree => "receipt-dirty-tree",
            Self::ReceiptCommitUnknown => "receipt-commit-unknown",
            Self::GateLineNotEligible => "gate-line-not-eligible",
            Self::PassWithoutControl => "pass-without-control",
            Self::ControlMissingFromReceipt => "control-missing-from-receipt",
            Self::ControlNotCaught => "control-not-caught",
            Self::SelfCertified => "self-certified",
            Self::EvidenceUnresolved => "evidence-unresolved",
            Self::ReceiptStackDev => "receipt-stack-dev",
            Self::DevBuildIdentity => "dev-build-identity",
            Self::ReceiptSetInvalid => "receipt-set-invalid",
            Self::PartRowsDropped => "part-rows-dropped",
            Self::ReceiptNotOfRecord => "receipt-not-of-record",
            Self::ReducedNotRecordable => "reduced-not-recordable",
            Self::UnmatchedSeed => "unmatched-seed",
            Self::OrphanControl => "orphan-control",
            Self::RuleWithoutGate => "rule-without-gate",
            Self::Removed => "removed",
            Self::Reused => "reused",
            Self::Renumbered => "renumbered",
            Self::OutOfOrder => "out-of-order",
            Self::Gap => "gap",
            Self::Untyped => "untyped",
            Self::ForeignTest => "foreign-test",
            Self::NoOutput => "no-output",
            Self::SelfClosed => "self-closed",
            Self::SharedReason => "shared-reason",
            Self::SharedOutput => "shared-output",
            Self::Burst => "burst",
            Self::ExportFormatUnreadable => "export-format-unreadable",
            Self::StandInSuspected => "stand-in-suspected",
            Self::UnboundDependency => "unbound-dependency",
            Self::ViewsDiffer => "views-differ",
            Self::ClaimEvidenceMismatch => "claim-evidence-mismatch",
            Self::NoPotencyRecord => "no-potency-record",
            Self::NotPotent => "not-potent",
            Self::OriginLabelMismatch => "origin-label-mismatch",
            Self::OriginNotAdmissible => "origin-not-admissible",
            Self::KaSecondFile => "ka-second-file",
            Self::BoundaryOnly => "boundary-only",
            Self::RelationWithoutControl => "relation-without-control",
            Self::ControlWithoutRelation => "control-without-relation",
            Self::MutationSurvived => "mutation-survived",
            Self::MutationStale => "mutation-stale",
            Self::DueUnbound => "due-unbound",
            Self::UnexpectedOwner => "unexpected-owner",
            Self::UnallocatedNumber => "unallocated-number",
            Self::MissingNumber => "missing-number",
            Self::ProvisionalTagPastDue => "provisional-tag-past-due",
            Self::CxCountMismatch => "cx-count-mismatch",
            Self::SameAsInvalid => "same-as-invalid",
            Self::CaseSupplierInvalid => "case-supplier-invalid",
            Self::FactWithoutRerun => "fact-without-rerun",
            Self::PlantMissing => "plant-missing",
            Self::RerunDisagreesWithPilot => "rerun-disagrees-with-pilot",
            Self::PlantChanged => "plant-changed",
            Self::TargetAsMeasurement => "target-as-measurement",
            Self::ClaimNotOneOfSeven => "claim-not-one-of-seven",
            Self::NoClaimOnNonReality => "no-claim-on-non-reality",
            Self::BlockedWithoutQuestion => "blocked-without-question",
            Self::KilledWithoutDecision => "killed-without-decision",
            Self::DuplicateId => "duplicate-id",
            Self::ReusedId => "reused-id",
            Self::Missing => "missing",
            Self::Unregistered => "unregistered",
            Self::Void => "void",
            Self::Registered => "registered",
            Self::Running => "running",
            Self::FalsifierHolds => "falsifier-holds",
            Self::Inconclusive => "inconclusive",
            Self::KillGateToRegister => "kill-gate-to-register",
            Self::MoonshotCap => "moonshot-cap",
            Self::PlanTimeDiffers => "plan-time-differs",
            Self::PlanTimeNotRun => "plan-time-not-run",
            Self::PilotIdForm => "pilot-id-form",
            Self::PilotPartMissing => "pilot-part-missing",
            Self::PilotCannotFail => "pilot-cannot-fail",
            Self::ResultProtocolMismatch => "result-protocol-mismatch",
            Self::ResultBeforeRegistration => "result-before-registration",
            Self::PilotReceiptUncommitted => "pilot-receipt-uncommitted",
            Self::FallbackTierNotLower => "fallback-tier-not-lower",
            Self::FallbackAlsoUnknown => "fallback-also-unknown",
            Self::TierSWithoutRequiredBy => "tier-s-without-required-by",
            Self::PlanTimeRecordUnknown => "plan-time-record-unknown",
        }
    }

    /// Parse a canonical kebab-case reason string into [`HarnessReason`].
    #[must_use]
    pub fn parse(id: &str) -> Option<Self> {
        Self::ALL.iter().copied().find(|r| r.as_str() == id)
    }

    /// Format the first line of a Gauntlet tool failure: `reason=<id> <detail>`.
    #[must_use]
    pub fn format_failure_line(self, detail: &str) -> String {
        if detail.is_empty() {
            format!("reason={}", self.as_str())
        } else {
            format!("reason={} {detail}", self.as_str())
        }
    }
}

impl fmt::Display for HarnessReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// One parsed `[[reason]]` row from `crates/frob-conformance/registry/reasons.toml`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReasonRecord {
    pub id: String,
    pub tool: Vec<String>,
    pub meaning: String,
    pub repairs: Vec<String>,
    pub exit: u8,
    pub alt_exits: Vec<u8>,
    pub kind: ReasonKind,
    pub dispositions: Vec<String>,
}

impl ReasonRecord {
    /// Returns `true` if `self.tool` contains `name` (case-sensitive exact match).
    #[must_use]
    pub fn has_tool(&self, name: &str) -> bool {
        self.tool.iter().any(|t| t == name)
    }

    /// Returns `true` if `exit` is the primary `exit` or listed in `alt_exits`.
    #[must_use]
    pub fn allows_exit(&self, exit: u8) -> bool {
        self.exit == exit || self.alt_exits.contains(&exit)
    }
}

/// Errors produced when loading or validating `registry/reasons.toml`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReasonRegistryError {
    Io {
        file: String,
        detail: String,
    },
    Toml {
        file: String,
        err: TomlError,
    },
    InvalidSchema {
        file: String,
        observed: String,
    },
    InvalidField {
        file: String,
        row: String,
        detail: String,
    },
    DuplicateId {
        file: String,
        id: String,
    },
    MissingFromRegistry {
        missing: Vec<String>,
    },
    UnknownInEnum {
        unknown: Vec<String>,
    },
}

impl fmt::Display for ReasonRegistryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io { file, detail } => write!(f, "{file}: {detail}"),
            Self::Toml { file, err } => write!(f, "{file}: {err}"),
            Self::InvalidSchema { file, observed } => {
                write!(
                    f,
                    "{file}: invalid schema `{observed}` (expected `{REASONS_SCHEMA}`)"
                )
            }
            Self::InvalidField { file, row, detail } => {
                write!(f, "{file}: row `{row}`: {detail}")
            }
            Self::DuplicateId { file, id } => {
                write!(
                    f,
                    "reason=reused {file}: duplicate harness reason id `{id}`"
                )
            }
            Self::MissingFromRegistry { missing } => {
                write!(
                    f,
                    "reason=removed missing registered harness reason(s) in reasons.toml: {}",
                    missing.join(", ")
                )
            }
            Self::UnknownInEnum { unknown } => {
                write!(
                    f,
                    "reason=out-of-order reasons.toml declares id(s) absent from HarnessReason enum: {}",
                    unknown.join(", ")
                )
            }
        }
    }
}

impl std::error::Error for ReasonRegistryError {}

/// Loaded Gauntlet harness reason registry (`frob.gauntlet.reasons/1`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReasonRegistry {
    schema: String,
    records: Vec<ReasonRecord>,
    by_id: BTreeMap<String, usize>,
}

impl ReasonRegistry {
    /// Load from the compile-time embedded `crates/frob-conformance/registry/reasons.toml`.
    pub fn load_embedded() -> Result<Self, ReasonRegistryError> {
        Self::from_bytes(EMBEDDED_REASONS_TOML.as_bytes(), REASONS_PATH)
    }

    /// Load from `<root>/crates/frob-conformance/registry/reasons.toml`.
    pub fn load_from_root(root: &Path) -> Result<Self, ReasonRegistryError> {
        let path = root.join(REASONS_PATH);
        let bytes = fs::read(&path).map_err(|e| ReasonRegistryError::Io {
            file: path.display().to_string(),
            detail: e.to_string(),
        })?;
        Self::from_bytes(&bytes, REASONS_PATH)
    }

    /// Parse `reasons.toml` strictly via `frob_core::registry::toml`.
    pub fn from_bytes(bytes: &[u8], file: &str) -> Result<Self, ReasonRegistryError> {
        let root = toml::parse(bytes).map_err(|err| ReasonRegistryError::Toml {
            file: file.to_owned(),
            err,
        })?;
        let mut top = Fields::new(root, "reasons.toml");
        let schema = top
            .require_str("schema")
            .map_err(|err| ReasonRegistryError::Toml {
                file: file.to_owned(),
                err,
            })?;
        if schema != REASONS_SCHEMA {
            return Err(top.fail(ReasonRegistryError::InvalidSchema {
                file: file.to_owned(),
                observed: schema,
            }));
        }
        let items = top
            .opt_array("reason")
            .map_err(|err| ReasonRegistryError::Toml {
                file: file.to_owned(),
                err,
            })?
            .unwrap_or_default();
        top.finish().map_err(|err| ReasonRegistryError::Toml {
            file: file.to_owned(),
            err,
        })?;

        let mut records = Vec::with_capacity(items.len());
        let mut by_id = BTreeMap::new();
        for (idx, sp) in items.into_iter().enumerate() {
            let Value::Table(tbl) = sp.value else {
                return Err(ReasonRegistryError::InvalidField {
                    file: file.to_owned(),
                    row: format!("reason[{idx}]"),
                    detail: format!("expected table, found {}", sp.value.type_name()),
                });
            };
            let rec = parse_reason_row(tbl, file, idx)?;
            if by_id.contains_key(&rec.id) {
                return Err(ReasonRegistryError::DuplicateId {
                    file: file.to_owned(),
                    id: rec.id,
                });
            }
            by_id.insert(rec.id.clone(), records.len());
            records.push(rec);
        }

        Ok(Self {
            schema,
            records,
            by_id,
        })
    }

    /// Verify exact 1-to-1 bijection between [`HarnessReason::ALL`] and `self.records`.
    pub fn validate_against_enum(&self) -> Result<(), ReasonRegistryError> {
        let mut missing = Vec::new();
        for &variant in HarnessReason::ALL {
            if !self.by_id.contains_key(variant.as_str()) {
                missing.push(variant.as_str().to_owned());
            }
        }
        if !missing.is_empty() {
            return Err(ReasonRegistryError::MissingFromRegistry { missing });
        }

        let mut unknown = Vec::new();
        for rec in &self.records {
            if HarnessReason::parse(&rec.id).is_none() {
                unknown.push(rec.id.clone());
            }
        }
        if !unknown.is_empty() {
            return Err(ReasonRegistryError::UnknownInEnum { unknown });
        }

        Ok(())
    }

    #[must_use]
    pub fn schema(&self) -> &str {
        &self.schema
    }

    #[must_use]
    pub fn records(&self) -> &[ReasonRecord] {
        &self.records
    }

    #[must_use]
    pub fn get(&self, id: &str) -> Option<&ReasonRecord> {
        self.by_id.get(id).map(|&idx| &self.records[idx])
    }

    #[must_use]
    pub fn contains(&self, id: &str) -> bool {
        self.by_id.contains_key(id)
    }

    #[must_use]
    pub fn count_by_exit(&self) -> BTreeMap<u8, usize> {
        let mut counts = BTreeMap::new();
        for rec in &self.records {
            *counts.entry(rec.exit).or_insert(0) += 1;
        }
        counts
    }

    #[must_use]
    pub fn count_by_tool(&self) -> BTreeMap<String, usize> {
        let mut counts = BTreeMap::new();
        for rec in &self.records {
            for t in &rec.tool {
                *counts.entry(t.clone()).or_insert(0) += 1;
            }
        }
        counts
    }
}

fn is_valid_reason_id(id: &str) -> bool {
    let bytes = id.as_bytes();
    !bytes.is_empty()
        && bytes[0].is_ascii_lowercase()
        && bytes
            .iter()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || *b == b'-')
        && !id.ends_with('-')
        && !id.contains("--")
}

fn parse_string_array(
    file: &str,
    row: &str,
    key: &str,
    items: Vec<Spanned<Value>>,
) -> Result<Vec<String>, ReasonRegistryError> {
    let mut out = Vec::with_capacity(items.len());
    for sp in items {
        match sp.value {
            Value::Str(s) => {
                if s.trim().is_empty() {
                    return Err(ReasonRegistryError::InvalidField {
                        file: file.to_owned(),
                        row: row.to_owned(),
                        detail: format!("`{key}` contains an empty string"),
                    });
                }
                out.push(s);
            }
            other => {
                return Err(ReasonRegistryError::InvalidField {
                    file: file.to_owned(),
                    row: row.to_owned(),
                    detail: format!(
                        "`{key}` expected string element, found {}",
                        other.type_name()
                    ),
                });
            }
        }
    }
    Ok(out)
}

fn parse_exit_u8(file: &str, row: &str, key: &str, raw: i64) -> Result<u8, ReasonRegistryError> {
    if raw == 1 {
        return Err(ReasonRegistryError::InvalidField {
            file: file.to_owned(),
            row: row.to_owned(),
            detail: format!("`{key}` must never be 1 (exit 1 is reserved for unhandled failures)"),
        });
    }
    let val = u8::try_from(raw)
        .ok()
        .filter(|v| VALID_HARNESS_EXITS.contains(v));
    val.ok_or_else(|| ReasonRegistryError::InvalidField {
        file: file.to_owned(),
        row: row.to_owned(),
        detail: format!("`{key}` value `{raw}` outside admitted harness exits {{0, 3, 101}}"),
    })
}

fn parse_reason_row(
    tbl: Table,
    file: &str,
    idx: usize,
) -> Result<ReasonRecord, ReasonRegistryError> {
    let row_hint = tbl
        .get("id")
        .and_then(Value::as_str)
        .map_or_else(|| format!("reason[{idx}]"), str::to_owned);
    let mut f = Fields::new(tbl, format!("reason[{row_hint}]"));
    let toml_err = |err: TomlError| ReasonRegistryError::Toml {
        file: file.to_owned(),
        err,
    };

    let id = f.require_str("id").map_err(toml_err)?;
    if !is_valid_reason_id(&id) {
        return Err(f.fail(ReasonRegistryError::InvalidField {
            file: file.to_owned(),
            row: id.clone(),
            detail: format!("invalid kebab-case harness reason id `{id}`"),
        }));
    }

    let tool_arr = f.require_array("tool").map_err(toml_err)?;
    let tool = parse_string_array(file, &id, "tool", tool_arr).map_err(|e| f.fail(e))?;
    if tool.is_empty() {
        return Err(f.fail(ReasonRegistryError::InvalidField {
            file: file.to_owned(),
            row: id,
            detail: "`tool` must list at least one emitting tool".to_owned(),
        }));
    }

    let meaning = f.require_str("meaning").map_err(toml_err)?;
    if meaning.trim().is_empty() {
        return Err(f.fail(ReasonRegistryError::InvalidField {
            file: file.to_owned(),
            row: id,
            detail: "`meaning` must be non-empty".to_owned(),
        }));
    }

    let repairs_arr = f.require_array("repairs").map_err(toml_err)?;
    let repairs = parse_string_array(file, &id, "repairs", repairs_arr).map_err(|e| f.fail(e))?;
    if repairs.is_empty() {
        return Err(f.fail(ReasonRegistryError::InvalidField {
            file: file.to_owned(),
            row: id,
            detail: "`repairs` must list at least one ranked repair".to_owned(),
        }));
    }

    let exit_raw = f.require_int("exit").map_err(toml_err)?;
    let exit = parse_exit_u8(file, &id, "exit", exit_raw).map_err(|e| f.fail(e))?;

    let alt_exits = match f.opt_array("alt_exits").map_err(toml_err)? {
        None => Vec::new(),
        Some(arr) => {
            let mut parsed = Vec::with_capacity(arr.len());
            for sp in arr {
                match sp.value {
                    Value::Int(n) => {
                        parsed
                            .push(parse_exit_u8(file, &id, "alt_exits", n).map_err(|e| f.fail(e))?);
                    }
                    other => {
                        return Err(f.fail(ReasonRegistryError::InvalidField {
                            file: file.to_owned(),
                            row: id,
                            detail: format!(
                                "`alt_exits` expected integer, found {}",
                                other.type_name()
                            ),
                        }));
                    }
                }
            }
            parsed
        }
    };

    let kind = match f.opt_str("kind").map_err(toml_err)? {
        None => ReasonKind::Refusal,
        Some(raw) => ReasonKind::parse(&raw).ok_or_else(|| {
            f.fail(ReasonRegistryError::InvalidField {
                file: file.to_owned(),
                row: id.clone(),
                detail: format!("unknown `kind` `{raw}`"),
            })
        })?,
    };

    let dispositions = match f.opt_array("dispositions").map_err(toml_err)? {
        None => Vec::new(),
        Some(arr) => parse_string_array(file, &id, "dispositions", arr).map_err(|e| f.fail(e))?,
    };

    f.finish().map_err(toml_err)?;

    Ok(ReasonRecord {
        id,
        tool,
        meaning,
        repairs,
        exit,
        alt_exits,
        kind,
        dispositions,
    })
}

// ============================================================================
// Existing public `FROB-REF-nnn` codes relied on by Gauntlet rules (N38-REF)
// ============================================================================

/// One public `FROB-REF-nnn` refusal code defined by an owning chapter that the
/// Gauntlet's rules rely on (N38-REF table; `frob-conformance` owns none of them).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PublicCodeRef {
    pub code: &'static str,
    pub number: u16,
    pub meaning: &'static str,
    pub invoked_in: &'static str,
    pub expected_exit: u8,
    pub capability_exception: bool,
}

/// The 24 existing public refusal codes relied on by Chapter 38 / Gauntlet rules.
pub const GAUNTLET_PUBLIC_CODES: &[PublicCodeRef] = &[
    PublicCodeRef {
        code: "FROB-REF-104",
        number: 104,
        meaning: "Inertia not physically consistent (smallest eigenvalue reported)",
        invoked_in: "Certifier self-test of physical consistency",
        expected_exit: 3,
        capability_exception: false,
    },
    PublicCodeRef {
        code: "FROB-REF-532",
        number: 532,
        meaning: "A fixed-n study has no pre-analysis, or names a verdict it cannot reach",
        invoked_in: "Certifier self-test of the pre-analysis; the planted tie at n = 64",
        expected_exit: 3,
        capability_exception: false,
    },
    PublicCodeRef {
        code: "FROB-REF-537",
        number: 537,
        meaning: "A rung used without a rank-agreement record",
        invoked_in: "Calibration sets and the rank-agreement self-test",
        expected_exit: 3,
        capability_exception: false,
    },
    PublicCodeRef {
        code: "FROB-REF-544",
        number: 544,
        meaning: "A training gate record without its negative controls",
        invoked_in: "Behaviour, not plumbing",
        expected_exit: 3,
        capability_exception: false,
    },
    PublicCodeRef {
        code: "FROB-REF-546",
        number: 546,
        meaning: "Calibration from survivors",
        invoked_in: "Calibration sets and the rank-agreement self-test",
        expected_exit: 3,
        capability_exception: false,
    },
    PublicCodeRef {
        code: "FROB-REF-548",
        number: 548,
        meaning: "Pooling across two model identities",
        invoked_in: "Calibration sets and the rank-agreement self-test",
        expected_exit: 3,
        capability_exception: false,
    },
    PublicCodeRef {
        code: "FROB-REF-560",
        number: 560,
        meaning: "A bound offered episodes not drawn nominally",
        invoked_in: "Reading the hard-world set",
        expected_exit: 3,
        capability_exception: false,
    },
    PublicCodeRef {
        code: "FROB-REF-567",
        number: 567,
        meaning: "A hard or search-selected world offered as a confirmation sample",
        invoked_in: "Reading the hard-world set",
        expected_exit: 3,
        capability_exception: false,
    },
    PublicCodeRef {
        code: "FROB-REF-569",
        number: 569,
        meaning: "Splitting requested where no randomness enters after the fork",
        invoked_in: "Rare-event self-test control",
        expected_exit: 3,
        capability_exception: false,
    },
    PublicCodeRef {
        code: "FROB-REF-814",
        number: 814,
        meaning: "Pooling across cells, across model identities, or of synthetic with real data",
        invoked_in: "The `synthetic` label; the Reality lane",
        expected_exit: 3,
        capability_exception: false,
    },
    PublicCodeRef {
        code: "FROB-REF-815",
        number: 815,
        meaning: "Closing a bead without a resolving log line from another session",
        invoked_in: "Typed close reasons; no self-certification",
        expected_exit: 3,
        capability_exception: false,
    },
    PublicCodeRef {
        code: "FROB-REF-816",
        number: 816,
        meaning: "An artifact whose authorization flags are missing or not `false`",
        invoked_in: "`sim_to_real_certified` has no setter",
        expected_exit: 3,
        capability_exception: false,
    },
    PublicCodeRef {
        code: "FROB-REF-819",
        number: 819,
        meaning: "A result cited or presented above its claim product",
        invoked_in: "Label propagation; the claim-product self-test",
        expected_exit: 3,
        capability_exception: false,
    },
    PublicCodeRef {
        code: "FROB-REF-823",
        number: 823,
        meaning: "Unknown proof kind",
        invoked_in: "The certificate mutation corpus",
        expected_exit: 3,
        capability_exception: false,
    },
    PublicCodeRef {
        code: "FROB-REF-825",
        number: 825,
        meaning: "A result of class D4 offered as evidence",
        invoked_in: "The determinism-classifier self-test",
        expected_exit: 3,
        capability_exception: false,
    },
    PublicCodeRef {
        code: "FROB-REF-826",
        number: 826,
        meaning: "Quantifier prefix mismatch",
        invoked_in: "The certificate mutation corpus",
        expected_exit: 3,
        capability_exception: false,
    },
    PublicCodeRef {
        code: "FROB-REF-829",
        number: 829,
        meaning: "Missing side condition",
        invoked_in: "The certificate mutation corpus",
        expected_exit: 3,
        capability_exception: false,
    },
    PublicCodeRef {
        code: "FROB-REF-833",
        number: 833,
        meaning: "An artifact whose authorization flags are missing or not `false` (downstream form)",
        invoked_in: "`sim_to_real_certified` has no setter",
        expected_exit: 3,
        capability_exception: false,
    },
    PublicCodeRef {
        code: "FROB-REF-834",
        number: 834,
        meaning: "A result cited or presented above its claim product (downstream form)",
        invoked_in: "Label propagation; the claim-product self-test",
        expected_exit: 3,
        capability_exception: false,
    },
    PublicCodeRef {
        code: "FROB-REF-840",
        number: 840,
        meaning: "Sealed confirmation data asked for before close without the administrative scope",
        invoked_in: "The confirmation firewall in the qualification self-test",
        expected_exit: 5,
        capability_exception: true,
    },
    PublicCodeRef {
        code: "FROB-REF-850",
        number: 850,
        meaning: "Wrong frame, unit, sign, or port order",
        invoked_in: "The certificate mutation corpus",
        expected_exit: 3,
        capability_exception: false,
    },
    PublicCodeRef {
        code: "FROB-REF-869",
        number: 869,
        meaning: "Citing a claim whose evidence closure is dangling or tombstoned",
        invoked_in: "A `pass` whose evidence does not resolve",
        expected_exit: 3,
        capability_exception: false,
    },
    PublicCodeRef {
        code: "FROB-REF-873",
        number: 873,
        meaning: "A dataset lacking licence, provenance, rights, or measurement uncertainty",
        invoked_in: "Admission of the first real dataset",
        expected_exit: 3,
        capability_exception: false,
    },
    PublicCodeRef {
        code: "FROB-REF-880",
        number: 880,
        meaning: "An attribution cited as an established cause",
        invoked_in: "Attribution self-test",
        expected_exit: 3,
        capability_exception: false,
    },
];

/// Load the workspace public refusal registry (`registries/refusals.toml`).
pub fn load_public_refusals(root: &Path) -> Result<RefusalRegistry, String> {
    let path = root.join(PUBLIC_REFUSALS_PATH);
    let bytes = fs::read(&path).map_err(|e| format!("read {}: {e}", path.display()))?;
    RefusalRegistry::from_bytes(&bytes).map_err(|e| format!("parse {}: {e}", path.display()))
}

/// Return any codes in `codes` that do not exist in `public_reg`.
#[must_use]
pub fn find_unknown_public_codes<'a>(
    codes: impl IntoIterator<Item = &'a str>,
    public_reg: &RefusalRegistry,
) -> Vec<String> {
    let mut unknown = BTreeSet::new();
    for code in codes {
        if public_reg.lookup(code, "gauntlet").is_err() {
            unknown.insert(code.to_owned());
        }
    }
    unknown.into_iter().collect()
}

/// Verify that no row of `registries/refusals.toml` names `frob-conformance` as its owner.
#[must_use]
pub fn find_conformance_owned_public_codes(public_reg: &RefusalRegistry) -> Vec<String> {
    let mut violations = Vec::new();
    for row in public_reg.rows() {
        for owner in &row.raised_by {
            let low = owner.to_ascii_lowercase();
            if low.contains("frob-conformance")
                || low.contains("frob_conformance")
                || low == "conformance"
                || low == "gauntlet"
            {
                violations.push(format!("{}: raised_by `{owner}`", row.code));
            }
        }
    }
    violations
}

/// Validate that every entry of [`GAUNTLET_PUBLIC_CODES`] obeys the family exit rules of
/// `registries/refusals.toml` (`1xx`/`5xx`/`8xx` -> `3`; `FROB-REF-840` -> `5` capability
/// exception) and agrees with any row already registered in `public_reg`.
#[must_use]
pub fn check_public_code_family_exits(public_reg: &RefusalRegistry) -> Vec<String> {
    let mut errors = Vec::new();
    let exit_rule = public_reg.exit_rule();

    for entry in GAUNTLET_PUBLIC_CODES {
        let family_digit = entry.number / 100;
        let family_key = format!("{family_digit}xx");
        let family_exit = exit_rule.ranges.get(&family_key).copied();

        if entry.capability_exception {
            if entry.expected_exit != 5 {
                errors.push(format!(
                    "{}: capability exception must expect exit 5, found {}",
                    entry.code, entry.expected_exit
                ));
            }
        } else {
            match family_exit {
                Some(rule_exit) if rule_exit == entry.expected_exit => {}
                Some(rule_exit) => errors.push(format!(
                    "{}: expected exit {} disagrees with [exit_rule].{family_key} = {rule_exit}",
                    entry.code, entry.expected_exit
                )),
                None => errors.push(format!(
                    "{}: [exit_rule] has no entry for family `{family_key}`",
                    entry.code
                )),
            }
        }

        if let Ok(row) = public_reg.lookup(entry.code, "exit_codes_by_family")
            && row.exit != entry.expected_exit
        {
            errors.push(format!(
                "{}: registered exit {} disagrees with N38-REF expected exit {}",
                entry.code, row.exit, entry.expected_exit
            ));
        }
    }

    errors
}

// ============================================================================
// Source scanners (`reason=<id>` and Gauntlet tier `FROB-REF-nnn` citations)
// ============================================================================

/// One literal `reason=<id>` occurrence found in source or scripts.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SourceReasonOccurrence {
    pub file: String,
    pub line: usize,
    pub reason: String,
}

/// Extract literal `reason=<kebab-id>` occurrences from source text.
///
/// Matches `reason=` preceded by a non-identifier boundary and immediately followed by a
/// lowercase ASCII kebab-case token terminated by a delimiter (whitespace, quotes, punctuation,
/// or end-of-line). Variable assignments such as `local reason="$1"` or f-string templates
/// `reason={...}` are ignored because no lowercase literal token immediately follows `=`.
#[must_use]
pub fn scan_text_for_reasons(file: &str, text: &str) -> Vec<SourceReasonOccurrence> {
    let mut found = Vec::new();
    for (idx, line) in text.lines().enumerate() {
        let bytes = line.as_bytes();
        let mut pos = 0usize;
        while let Some(rel) = line[pos..].find("reason=") {
            let start = pos + rel;
            let boundary_ok = start == 0
                || !(bytes[start - 1].is_ascii_alphanumeric() || bytes[start - 1] == b'_');
            let val_start = start + "reason=".len();
            if boundary_ok && val_start < bytes.len() && bytes[val_start].is_ascii_lowercase() {
                let mut end = val_start;
                while end < bytes.len()
                    && (bytes[end].is_ascii_lowercase()
                        || bytes[end].is_ascii_digit()
                        || bytes[end] == b'-')
                {
                    end += 1;
                }
                let next_ok = end == bytes.len()
                    || matches!(
                        bytes[end],
                        b' ' | b'\t'
                            | b'"'
                            | b'\''
                            | b'`'
                            | b','
                            | b';'
                            | b':'
                            | b')'
                            | b']'
                            | b'}'
                            | b'\\'
                    );
                if next_ok {
                    let token = &line[val_start..end];
                    if is_valid_reason_id(token) {
                        found.push(SourceReasonOccurrence {
                            file: file.to_owned(),
                            line: idx + 1,
                            reason: token.to_owned(),
                        });
                    }
                }
            }
            pos = val_start;
        }
    }
    found
}

/// Extract `FROB-REF-nnn` codes from source text.
#[must_use]
pub fn scan_text_for_public_codes(file: &str, text: &str) -> Vec<(String, usize, String)> {
    let mut found = Vec::new();
    for (idx, line) in text.lines().enumerate() {
        let bytes = line.as_bytes();
        let mut pos = 0usize;
        while let Some(rel) = line[pos..].find("FROB-REF-") {
            let start = pos + rel;
            let digits_start = start + "FROB-REF-".len();
            if digits_start + 3 <= bytes.len()
                && bytes[digits_start..digits_start + 3]
                    .iter()
                    .all(|b| b.is_ascii_digit())
            {
                let after = digits_start + 3;
                let end_ok = after == bytes.len() || !bytes[after].is_ascii_digit();
                if end_ok {
                    found.push((file.to_owned(), idx + 1, line[start..after].to_owned()));
                }
            }
            pos = digits_start;
        }
    }
    found
}

fn collect_files(dir: &Path, exts: &[&str], out: &mut Vec<PathBuf>) {
    let Ok(rd) = fs::read_dir(dir) else {
        return;
    };
    let mut entries: Vec<PathBuf> = rd.flatten().map(|e| e.path()).collect();
    entries.sort();
    for path in entries {
        if path.is_dir() {
            collect_files(&path, exts, out);
        } else if path.is_file()
            && path
                .extension()
                .and_then(|s| s.to_str())
                .is_some_and(|e| exts.contains(&e))
        {
            out.push(path);
        }
    }
}

/// Scan `crates/frob-conformance/{src,tests,examples}` and `scripts/e2e/` for any literal
/// `reason=<id>` that is not registered in `reg`.
pub fn scan_source_tree_for_unregistered_reasons(
    root: &Path,
    reg: &ReasonRegistry,
) -> Result<Vec<SourceReasonOccurrence>, String> {
    let mut files = Vec::new();
    for rel in [
        "crates/frob-conformance/src",
        "crates/frob-conformance/tests",
        "crates/frob-conformance/examples",
    ] {
        collect_files(&root.join(rel), &["rs"], &mut files);
    }
    collect_files(&root.join("scripts/e2e"), &["sh"], &mut files);
    files.sort();

    let mut unregistered = Vec::new();
    for path in files {
        let rel = path
            .strip_prefix(root)
            .unwrap_or(&path)
            .to_string_lossy()
            .replace('\\', "/");
        let text = fs::read_to_string(&path).map_err(|e| format!("read {rel}: {e}"))?;
        for occ in scan_text_for_reasons(&rel, &text) {
            if !reg.contains(&occ.reason) {
                unregistered.push(occ);
            }
        }
    }
    Ok(unregistered)
}

/// Scan the six Gauntlet tier test targets (`tests/g0_laws.rs`..`tests/g5_determinism.rs`)
/// for cited `FROB-REF-nnn` public codes.
pub fn scan_gauntlet_tier_tests_for_public_codes(
    root: &Path,
) -> Result<Vec<(String, usize, String)>, String> {
    const TIER_FILES: &[&str] = &[
        "crates/frob-conformance/tests/g0_laws.rs",
        "crates/frob-conformance/tests/g1_known_answers.rs",
        "crates/frob-conformance/tests/g2_parity.rs",
        "crates/frob-conformance/tests/g3_metamorphic.rs",
        "crates/frob-conformance/tests/g4_chaos.rs",
        "crates/frob-conformance/tests/g5_determinism.rs",
    ];
    let mut cited = Vec::new();
    for rel in TIER_FILES {
        let path = root.join(rel);
        if !path.is_file() {
            continue;
        }
        let text = fs::read_to_string(&path).map_err(|e| format!("read {rel}: {e}"))?;
        cited.extend(scan_text_for_public_codes(rel, &text));
    }
    Ok(cited)
}

// ============================================================================
// End-to-end NDJSON log `refusal` event scanner (N38-REF)
// ============================================================================

/// Result of scanning the newest log of each script under `target/e2e-logs/`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogRefusalScanReport {
    pub scripts_scanned: BTreeMap<String, usize>,
    pub events_scanned: usize,
    pub unknowns: Vec<String>,
}

/// Scan every `refusal` event in the newest `.ndjson` log of each script directory under
/// `e2e_logs_dir`, checking that every `reason` is registered in `reason_reg`, every `code`
/// is registered in `public_reg`, and no `refusal` event omits both.
pub fn scan_e2e_refusal_logs(
    e2e_logs_dir: &Path,
    reason_reg: &ReasonRegistry,
    public_reg: &RefusalRegistry,
) -> Result<LogRefusalScanReport, String> {
    let mut scripts_scanned = BTreeMap::new();
    let mut events_scanned = 0usize;
    let mut unknowns = Vec::new();

    if !e2e_logs_dir.is_dir() {
        return Ok(LogRefusalScanReport {
            scripts_scanned,
            events_scanned,
            unknowns,
        });
    }

    let rd =
        fs::read_dir(e2e_logs_dir).map_err(|e| format!("read {}: {e}", e2e_logs_dir.display()))?;
    let mut script_dirs: Vec<PathBuf> = rd
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.is_dir())
        .collect();
    script_dirs.sort();

    for dir in script_dirs {
        let script_name = dir
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_owned();
        let mut logs: Vec<PathBuf> = fs::read_dir(&dir)
            .map_err(|e| format!("read {}: {e}", dir.display()))?
            .flatten()
            .map(|e| e.path())
            .filter(|p| p.is_file() && p.extension().and_then(|s| s.to_str()) == Some("ndjson"))
            .collect();
        logs.sort();
        let Some(newest) = logs.pop() else {
            continue;
        };

        let raw =
            fs::read_to_string(&newest).map_err(|e| format!("read {}: {e}", newest.display()))?;
        let mut script_events = 0usize;

        for (idx, line) in raw.lines().enumerate() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            let Ok(val) = json::parse_strict(trimmed) else {
                continue;
            };
            if val.get("event").and_then(JsonValue::as_str) != Some("refusal") {
                continue;
            }
            script_events += 1;
            events_scanned += 1;

            let reason = val
                .get("reason")
                .and_then(JsonValue::as_str)
                .filter(|s| !s.is_empty());
            let code = val
                .get("code")
                .and_then(JsonValue::as_str)
                .filter(|s| !s.is_empty());

            if reason.is_none() && code.is_none() {
                unknowns.push(format!(
                    "{}:{} refusal event carries neither `reason` nor `code`",
                    newest.display(),
                    idx + 1
                ));
                continue;
            }
            if let Some(r) = reason
                && !reason_reg.contains(r)
            {
                unknowns.push(format!(
                    "{}:{} unregistered harness reason `{r}`",
                    newest.display(),
                    idx + 1
                ));
            }
            if let Some(c) = code
                && public_reg.lookup(c, "e2e-log").is_err()
            {
                unknowns.push(format!(
                    "{}:{} unknown public refusal code `{c}`",
                    newest.display(),
                    idx + 1
                ));
            }
        }

        scripts_scanned.insert(script_name, script_events);
    }

    Ok(LogRefusalScanReport {
        scripts_scanned,
        events_scanned,
        unknowns,
    })
}
