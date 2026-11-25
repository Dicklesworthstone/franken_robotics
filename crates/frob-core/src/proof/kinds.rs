//! Closed list of the fifteen mission proof kinds (`frob.cert.kinds/1`, `N25b-01`).
//!
//! Names match the auditor's closed registry (`docs/registries/cert-kinds.md`) verbatim
//! so `frob-lemma` maps nothing when writing `.frobcert` bundles.

use std::fmt;

use crate::evidence::EvidenceClass;
use crate::proof::num::CanonicalEncoder;

/// Cell verdict polarity asserted by a proof object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CellPolarity {
    Inner,
    Excluded,
}

impl CellPolarity {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Inner => "inner",
            Self::Excluded => "excluded",
        }
    }
}

/// Expected cell verdict (`Inner` or `Excluded`) paired with the claimed evidence class
/// of the cell (`N25b-01` rule 4).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Expected {
    pub polarity: CellPolarity,
    pub class: EvidenceClass,
}

impl Expected {
    #[must_use]
    pub const fn new(polarity: CellPolarity, class: EvidenceClass) -> Self {
        Self { polarity, class }
    }

    #[must_use]
    pub const fn inner_certified() -> Self {
        Self::new(CellPolarity::Inner, EvidenceClass::Certified)
    }

    #[must_use]
    pub const fn excluded_certified() -> Self {
        Self::new(CellPolarity::Excluded, EvidenceClass::Certified)
    }

    #[must_use]
    pub const fn inner_tested() -> Self {
        Self::new(CellPolarity::Inner, EvidenceClass::Tested)
    }

    #[must_use]
    pub const fn excluded_tested() -> Self {
        Self::new(CellPolarity::Excluded, EvidenceClass::Tested)
    }

    #[must_use]
    pub const fn inner_estimated() -> Self {
        Self::new(CellPolarity::Inner, EvidenceClass::Estimated)
    }

    #[must_use]
    pub const fn excluded_estimated() -> Self {
        Self::new(CellPolarity::Excluded, EvidenceClass::Estimated)
    }

    pub fn encode_canonical(self, enc: &mut CanonicalEncoder) {
        enc.tag(b"expected");
        enc.str(self.polarity.as_str());
        enc.str(self.class.as_str());
    }
}

/// The fifteen mission proof kinds owned by `N25b-01`, named identically to `frob.cert.kinds/1`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ProofKind {
    LpPrimal,
    LpFarkas,
    ConicFarkas,
    IntervalBox,
    TaylorBox,
    SosGram,
    FiniteGame,
    AliasPair,
    MonotoneTransport,
    SymmetryTransport,
    Partition,
    Composition,
    ProductUnreachable,
    TraceWitness,
    SampleCoverage,
}

impl ProofKind {
    /// All fifteen proof kinds in canonical declaration order.
    pub const ALL: &'static [Self] = &[
        Self::LpPrimal,
        Self::LpFarkas,
        Self::ConicFarkas,
        Self::IntervalBox,
        Self::TaylorBox,
        Self::SosGram,
        Self::FiniteGame,
        Self::AliasPair,
        Self::MonotoneTransport,
        Self::SymmetryTransport,
        Self::Partition,
        Self::Composition,
        Self::ProductUnreachable,
        Self::TraceWitness,
        Self::SampleCoverage,
    ];

    /// Mission-scope kinds already registered in `frob.cert.kinds/1` whose proof-object
    /// variants and payloads land in later beads (`N25e-10` and `N22b-14`).
    pub const PENDING_MISSION_KINDS: &'static [&'static str] = &[
        "monomial-product",
        "energy-budget",
    ];

    /// Canonical registry string name matching `docs/registries/cert-kinds.md`.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LpPrimal => "lp-primal",
            Self::LpFarkas => "lp-farkas",
            Self::ConicFarkas => "conic-farkas",
            Self::IntervalBox => "interval-box",
            Self::TaylorBox => "taylor-box",
            Self::SosGram => "sos-gram",
            Self::FiniteGame => "finite-game",
            Self::AliasPair => "alias-pair",
            Self::MonotoneTransport => "monotone-transport",
            Self::SymmetryTransport => "symmetry-transport",
            Self::Partition => "partition",
            Self::Composition => "composition",
            Self::ProductUnreachable => "product-unreachable",
            Self::TraceWitness => "trace-witness",
            Self::SampleCoverage => "sample-coverage",
        }
    }

    /// Parse a canonical registry name into a [`ProofKind`].
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "lp-primal" => Some(Self::LpPrimal),
            "lp-farkas" => Some(Self::LpFarkas),
            "conic-farkas" => Some(Self::ConicFarkas),
            "interval-box" => Some(Self::IntervalBox),
            "taylor-box" => Some(Self::TaylorBox),
            "sos-gram" => Some(Self::SosGram),
            "finite-game" => Some(Self::FiniteGame),
            "alias-pair" => Some(Self::AliasPair),
            "monotone-transport" => Some(Self::MonotoneTransport),
            "symmetry-transport" => Some(Self::SymmetryTransport),
            "partition" => Some(Self::Partition),
            "composition" => Some(Self::Composition),
            "product-unreachable" => Some(Self::ProductUnreachable),
            "trace-witness" => Some(Self::TraceWitness),
            "sample-coverage" => Some(Self::SampleCoverage),
            _ => None,
        }
    }

    /// Maximum evidence class this proof kind may establish on a mission answer (`N25b-01` table,
    /// §25.5.2, Appendix I.3: `trace-witness` caps at `Tested`; `sample-coverage` caps at
    /// `Estimated` on mission answers — empirical study receipts under `study` scope cap at `Tested`
    /// in `frob-cert`; all other thirteen kinds cap at `Certified`).
    #[must_use]
    pub const fn class_cap(self) -> EvidenceClass {
        match self {
            Self::TraceWitness => EvidenceClass::Tested,
            Self::SampleCoverage => EvidenceClass::Estimated,
            Self::LpPrimal
            | Self::LpFarkas
            | Self::ConicFarkas
            | Self::IntervalBox
            | Self::TaylorBox
            | Self::SosGram
            | Self::FiniteGame
            | Self::AliasPair
            | Self::MonotoneTransport
            | Self::SymmetryTransport
            | Self::Partition
            | Self::Composition
            | Self::ProductUnreachable => EvidenceClass::Certified,
        }
    }

    /// Whether this proof kind can establish `polarity` (`N25b-01` rule 4).
    #[must_use]
    pub const fn allows_polarity(self, polarity: CellPolarity) -> bool {
        match self {
            Self::LpPrimal | Self::FiniteGame => matches!(polarity, CellPolarity::Inner),
            Self::LpFarkas
            | Self::ConicFarkas
            | Self::AliasPair
            | Self::ProductUnreachable => matches!(polarity, CellPolarity::Excluded),
            Self::IntervalBox
            | Self::TaylorBox
            | Self::SosGram
            | Self::MonotoneTransport
            | Self::SymmetryTransport
            | Self::Partition
            | Self::Composition
            | Self::TraceWitness
            | Self::SampleCoverage => true,
        }
    }
}

impl fmt::Display for ProofKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
