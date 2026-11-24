//! Closed catalogue of auditor rejection rule identifiers (`RuleId`) for `frob-cert` (N12-02).
//!
//! Every rejection returned by the auditor names a `RuleId` from this closed enum,
//! formatted as a dotted ASCII identifier `<area>.<rule>`. No checker can return an
//! identifier outside `RULE_CATALOGUE`.

use std::fmt;

use crate::limits::{
    REFUSAL_ARITHMETIC_MALFORMED, RULE_INPUT_UNREADABLE, RULE_JSON_BOM, RULE_JSON_DUPLICATE_KEY,
    RULE_JSON_ESCAPE, RULE_JSON_KEY_ORDER, RULE_JSON_SYNTAX, RULE_JSON_WHITESPACE,
    RULE_KIND_UNKNOWN, RULE_LIMITS_EXCEEDED, RULE_NUM_FLOAT_IN_BUNDLE, RULE_NUM_INT_FORM,
    RULE_NUM_INT_RANGE, RULE_NUM_RATIONAL_FORM,
};

/// Refusal code for unknown proof kind, unknown version, out-of-scope use, or disabled kind
/// (N12-REF, ADR-LEMMA-04, INV-LEMMA-04).
pub const REFUSAL_UNKNOWN_PROOF_KIND: &str = "FROB-REF-823";

/// Refusal code for a claim presented above its kind's class cap or weakest input (N12-REF).
pub const REFUSAL_CLASS_ABOVE_CAP: &str = "FROB-REF-834";

/// Ranked repairs for `kind.unknown`, `kind.version-unknown`, and `kind.out-of-scope` (N12-02 / N12-REF).
pub const REPAIRS_UNKNOWN_KIND: &[&str] = &[
    "re-emit with a registered kind",
    "upgrade the auditor",
];

/// Ranked repairs for `kind.disabled` (N12-REF).
pub const REPAIRS_DISABLED_KIND: &[&str] = &[
    "re-derive with another registered kind",
    "report the cell Tested",
];

/// Ranked repairs for `class.above-kind-cap` (N12-REF).
pub const REPAIRS_CLASS_ABOVE_CAP: &[&str] = &[
    "restate the result at or below the kind's class cap",
    "re-derive using a proof kind whose cap covers the claimed class",
];

/// Closed enum of every rule identifier the auditor may emit.
///
/// Constructing a rule outside this enum is impossible at compile time:
///
/// ```compile_fail
/// use frob_cert::kinds::rules::RuleId;
/// let _r: RuleId = RuleId("custom.invented");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RuleId {
    // Area: input
    InputUnreadable,
    // Area: json
    JsonSyntax,
    JsonBom,
    JsonDuplicateKey,
    JsonKeyOrder,
    JsonWhitespace,
    JsonEscape,
    // Area: schema
    SchemaUnknown,
    SchemaMissingField,
    SchemaFieldType,
    // Area: hash
    HashMismatch,
    HashMerkleRoot,
    // Area: kind
    KindUnknown,
    KindVersionUnknown,
    KindOutOfScope,
    KindDisabled,
    // Area: query
    QueryMismatch,
    QueryDigestMismatch,
    // Area: side
    SideConditionMissing,
    SideConditionRejected,
    // Area: field
    FieldUnitsMismatch,
    FieldFrameMismatch,
    FieldPortOrder,
    // Area: num
    NumFloatInBundle,
    NumIntRange,
    NumIntForm,
    NumRationalForm,
    // Area: limits
    LimitsExceeded,
    // Area: class
    ClassAboveKindCap,
    ClassAboveInput,
    ClassNoReceipt,
    // Area: flags
    FlagsMissing,
    FlagsTrue,
    // One area per proof kind (27 kinds)
    PolyIdentityCheckFailed,
    RationalLdltCheckFailed,
    IntervalBoxCheckFailed,
    IntervalRootCheckFailed,
    TaylorBoxCheckFailed,
    SosGramCheckFailed,
    LpFarkasCheckFailed,
    ConicFarkasCheckFailed,
    LpPrimalCheckFailed,
    FiniteGameCheckFailed,
    TraceWitnessCheckFailed,
    PartitionCheckFailed,
    MonotoneTransportCheckFailed,
    SymmetryTransportCheckFailed,
    CompositionCheckFailed,
    MonomialProductCheckFailed,
    ComplexConnectivityCheckFailed,
    TubeInclusionCheckFailed,
    RewriteTraceCheckFailed,
    OrbitCountCheckFailed,
    GroupEnumerationCheckFailed,
    EnergyBudgetCheckFailed,
    AliasPairCheckFailed,
    ProductUnreachableCheckFailed,
    AlphaRootCheckFailed,
    LeanCapsuleCheckFailed,
    SampleCoverageCheckFailed,
}

/// Complete compiled catalogue of every `RuleId` the auditor can return.
pub const RULE_CATALOGUE: &[RuleId] = &[
    RuleId::InputUnreadable,
    RuleId::JsonSyntax,
    RuleId::JsonBom,
    RuleId::JsonDuplicateKey,
    RuleId::JsonKeyOrder,
    RuleId::JsonWhitespace,
    RuleId::JsonEscape,
    RuleId::SchemaUnknown,
    RuleId::SchemaMissingField,
    RuleId::SchemaFieldType,
    RuleId::HashMismatch,
    RuleId::HashMerkleRoot,
    RuleId::KindUnknown,
    RuleId::KindVersionUnknown,
    RuleId::KindOutOfScope,
    RuleId::KindDisabled,
    RuleId::QueryMismatch,
    RuleId::QueryDigestMismatch,
    RuleId::SideConditionMissing,
    RuleId::SideConditionRejected,
    RuleId::FieldUnitsMismatch,
    RuleId::FieldFrameMismatch,
    RuleId::FieldPortOrder,
    RuleId::NumFloatInBundle,
    RuleId::NumIntRange,
    RuleId::NumIntForm,
    RuleId::NumRationalForm,
    RuleId::LimitsExceeded,
    RuleId::ClassAboveKindCap,
    RuleId::ClassAboveInput,
    RuleId::ClassNoReceipt,
    RuleId::FlagsMissing,
    RuleId::FlagsTrue,
    RuleId::PolyIdentityCheckFailed,
    RuleId::RationalLdltCheckFailed,
    RuleId::IntervalBoxCheckFailed,
    RuleId::IntervalRootCheckFailed,
    RuleId::TaylorBoxCheckFailed,
    RuleId::SosGramCheckFailed,
    RuleId::LpFarkasCheckFailed,
    RuleId::ConicFarkasCheckFailed,
    RuleId::LpPrimalCheckFailed,
    RuleId::FiniteGameCheckFailed,
    RuleId::TraceWitnessCheckFailed,
    RuleId::PartitionCheckFailed,
    RuleId::MonotoneTransportCheckFailed,
    RuleId::SymmetryTransportCheckFailed,
    RuleId::CompositionCheckFailed,
    RuleId::MonomialProductCheckFailed,
    RuleId::ComplexConnectivityCheckFailed,
    RuleId::TubeInclusionCheckFailed,
    RuleId::RewriteTraceCheckFailed,
    RuleId::OrbitCountCheckFailed,
    RuleId::GroupEnumerationCheckFailed,
    RuleId::EnergyBudgetCheckFailed,
    RuleId::AliasPairCheckFailed,
    RuleId::ProductUnreachableCheckFailed,
    RuleId::AlphaRootCheckFailed,
    RuleId::LeanCapsuleCheckFailed,
    RuleId::SampleCoverageCheckFailed,
];

impl RuleId {
    /// Dotted ASCII `<area>.<rule>` string representation.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InputUnreadable => RULE_INPUT_UNREADABLE,
            Self::JsonSyntax => RULE_JSON_SYNTAX,
            Self::JsonBom => RULE_JSON_BOM,
            Self::JsonDuplicateKey => RULE_JSON_DUPLICATE_KEY,
            Self::JsonKeyOrder => RULE_JSON_KEY_ORDER,
            Self::JsonWhitespace => RULE_JSON_WHITESPACE,
            Self::JsonEscape => RULE_JSON_ESCAPE,
            Self::SchemaUnknown => "schema.unknown",
            Self::SchemaMissingField => "schema.missing-field",
            Self::SchemaFieldType => "schema.field-type",
            Self::HashMismatch => "hash.mismatch",
            Self::HashMerkleRoot => "hash.merkle-root",
            Self::KindUnknown => RULE_KIND_UNKNOWN,
            Self::KindVersionUnknown => "kind.version-unknown",
            Self::KindOutOfScope => "kind.out-of-scope",
            Self::KindDisabled => "kind.disabled",
            Self::QueryMismatch => "query.mismatch",
            Self::QueryDigestMismatch => "query.digest-mismatch",
            Self::SideConditionMissing => "side.condition-missing",
            Self::SideConditionRejected => "side.condition-rejected",
            Self::FieldUnitsMismatch => "field.units-mismatch",
            Self::FieldFrameMismatch => "field.frame-mismatch",
            Self::FieldPortOrder => "field.port-order",
            Self::NumFloatInBundle => RULE_NUM_FLOAT_IN_BUNDLE,
            Self::NumIntRange => RULE_NUM_INT_RANGE,
            Self::NumIntForm => RULE_NUM_INT_FORM,
            Self::NumRationalForm => RULE_NUM_RATIONAL_FORM,
            Self::LimitsExceeded => RULE_LIMITS_EXCEEDED,
            Self::ClassAboveKindCap => "class.above-kind-cap",
            Self::ClassAboveInput => "class.above-input",
            Self::ClassNoReceipt => "class.no-receipt",
            Self::FlagsMissing => "flags.missing",
            Self::FlagsTrue => "flags.true",
            Self::PolyIdentityCheckFailed => "poly-identity.check-failed",
            Self::RationalLdltCheckFailed => "rational-ldlt.check-failed",
            Self::IntervalBoxCheckFailed => "interval-box.check-failed",
            Self::IntervalRootCheckFailed => "interval-root.check-failed",
            Self::TaylorBoxCheckFailed => "taylor-box.check-failed",
            Self::SosGramCheckFailed => "sos-gram.check-failed",
            Self::LpFarkasCheckFailed => "lp-farkas.check-failed",
            Self::ConicFarkasCheckFailed => "conic-farkas.check-failed",
            Self::LpPrimalCheckFailed => "lp-primal.check-failed",
            Self::FiniteGameCheckFailed => "finite-game.check-failed",
            Self::TraceWitnessCheckFailed => "trace-witness.check-failed",
            Self::PartitionCheckFailed => "partition.check-failed",
            Self::MonotoneTransportCheckFailed => "monotone-transport.check-failed",
            Self::SymmetryTransportCheckFailed => "symmetry-transport.check-failed",
            Self::CompositionCheckFailed => "composition.check-failed",
            Self::MonomialProductCheckFailed => "monomial-product.check-failed",
            Self::ComplexConnectivityCheckFailed => "complex-connectivity.check-failed",
            Self::TubeInclusionCheckFailed => "tube-inclusion.check-failed",
            Self::RewriteTraceCheckFailed => "rewrite-trace.check-failed",
            Self::OrbitCountCheckFailed => "orbit-count.check-failed",
            Self::GroupEnumerationCheckFailed => "group-enumeration.check-failed",
            Self::EnergyBudgetCheckFailed => "energy-budget.check-failed",
            Self::AliasPairCheckFailed => "alias-pair.check-failed",
            Self::ProductUnreachableCheckFailed => "product-unreachable.check-failed",
            Self::AlphaRootCheckFailed => "alpha-root.check-failed",
            Self::LeanCapsuleCheckFailed => "lean-capsule.check-failed",
            Self::SampleCoverageCheckFailed => "sample-coverage.check-failed",
        }
    }

    /// The `<area>` prefix before the `.` separator.
    pub fn area(self) -> &'static str {
        let s = self.as_str();
        match s.split_once('.') {
            Some((area, _)) => area,
            None => s,
        }
    }

    /// Default refusal code associated with this rule identifier, if any.
    pub const fn refusal_code(self) -> Option<&'static str> {
        match self {
            Self::KindUnknown
            | Self::KindVersionUnknown
            | Self::KindOutOfScope
            | Self::KindDisabled => Some(REFUSAL_UNKNOWN_PROOF_KIND),
            Self::ClassAboveKindCap | Self::ClassAboveInput | Self::ClassNoReceipt => {
                Some(REFUSAL_CLASS_ABOVE_CAP)
            }
            Self::NumFloatInBundle
            | Self::NumIntRange
            | Self::NumIntForm
            | Self::NumRationalForm => Some(REFUSAL_ARITHMETIC_MALFORMED),
            _ => None,
        }
    }

    /// Look up a `RuleId` by its exact dotted string.
    pub fn parse(s: &str) -> Option<Self> {
        RULE_CATALOGUE.iter().copied().find(|r| r.as_str() == s)
    }
}

impl fmt::Display for RuleId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
