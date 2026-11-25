//! One-file-per-kind proof payload definitions and closed [`ProofPayload`] union (`N25b-01`).

pub mod alias_pair;
pub mod composition;
pub mod conic_farkas;
pub mod finite_game;
pub mod interval_box;
pub mod lp_farkas;
pub mod lp_primal;
pub mod monotone_transport;
pub mod partition;
pub mod product_unreachable;
pub mod sample_coverage;
pub mod sos_gram;
pub mod symmetry_transport;
pub mod taylor_box;
pub mod trace_witness;

pub use alias_pair::{AliasPairPayload, AliasScope};
pub use composition::{CompositionPayload, LedgerEntry, StageLink};
pub use conic_farkas::ConicFarkasPayload;
pub use finite_game::{Comparator, FiniteGamePayload};
pub use interval_box::{BoxOutcome, IntervalBoxPayload};
pub use lp_farkas::{DualVector, LpFarkasPayload};
pub use lp_primal::{CellEndForces, LoadVertexWitness, LpLaw, LpPrimalPayload, vertex_force_values};
pub use monotone_transport::MonotoneTransportPayload;
pub use partition::{PartitionCell, PartitionPayload};
pub use product_unreachable::ProductUnreachablePayload;
pub use sample_coverage::SampleCoveragePayload;
pub use sos_gram::{SosGramPayload, SosMultiplier};
pub use symmetry_transport::SymmetryTransportPayload;
pub use taylor_box::TaylorBoxPayload;
pub use trace_witness::TraceWitnessPayload;

use crate::proof::kinds::{CellPolarity, ProofKind};
use crate::proof::num::{CanonicalEncoder, ProofError};

/// Closed payload enum with exactly one variant per [`ProofKind`] (15 mission kinds).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ProofPayload {
    LpPrimal(LpPrimalPayload),
    LpFarkas(LpFarkasPayload),
    ConicFarkas(ConicFarkasPayload),
    IntervalBox(IntervalBoxPayload),
    TaylorBox(TaylorBoxPayload),
    SosGram(SosGramPayload),
    FiniteGame(FiniteGamePayload),
    AliasPair(AliasPairPayload),
    MonotoneTransport(MonotoneTransportPayload),
    SymmetryTransport(SymmetryTransportPayload),
    Partition(PartitionPayload),
    Composition(CompositionPayload),
    ProductUnreachable(ProductUnreachablePayload),
    TraceWitness(TraceWitnessPayload),
    SampleCoverage(SampleCoveragePayload),
}

impl ProofPayload {
    #[must_use]
    pub const fn kind(&self) -> ProofKind {
        match self {
            Self::LpPrimal(_) => ProofKind::LpPrimal,
            Self::LpFarkas(_) => ProofKind::LpFarkas,
            Self::ConicFarkas(_) => ProofKind::ConicFarkas,
            Self::IntervalBox(_) => ProofKind::IntervalBox,
            Self::TaylorBox(_) => ProofKind::TaylorBox,
            Self::SosGram(_) => ProofKind::SosGram,
            Self::FiniteGame(_) => ProofKind::FiniteGame,
            Self::AliasPair(_) => ProofKind::AliasPair,
            Self::MonotoneTransport(_) => ProofKind::MonotoneTransport,
            Self::SymmetryTransport(_) => ProofKind::SymmetryTransport,
            Self::Partition(_) => ProofKind::Partition,
            Self::Composition(_) => ProofKind::Composition,
            Self::ProductUnreachable(_) => ProofKind::ProductUnreachable,
            Self::TraceWitness(_) => ProofKind::TraceWitness,
            Self::SampleCoverage(_) => ProofKind::SampleCoverage,
        }
    }

    pub fn validate_shape(&self) -> Result<(), ProofError> {
        match self {
            Self::LpPrimal(p) => p.validate_shape(),
            Self::LpFarkas(p) => p.validate_shape(),
            Self::ConicFarkas(p) => p.validate_shape(),
            Self::IntervalBox(p) => p.validate_shape(),
            Self::TaylorBox(p) => p.validate_shape(),
            Self::SosGram(p) => p.validate_shape(),
            Self::FiniteGame(p) => p.validate_shape(),
            Self::AliasPair(p) => p.validate_shape(),
            Self::MonotoneTransport(p) => p.validate_shape(),
            Self::SymmetryTransport(p) => p.validate_shape(),
            Self::Partition(p) => p.validate_shape(),
            Self::Composition(p) => p.validate_shape(),
            Self::ProductUnreachable(p) => p.validate_shape(),
            Self::TraceWitness(p) => p.validate_shape(),
            Self::SampleCoverage(p) => p.validate_shape(),
        }
    }

    /// Check payload-specific consistency with expected cell polarity (e.g. `holds` vs `fails`).
    pub fn check_polarity_agreement(&self, polarity: CellPolarity) -> Result<(), ProofError> {
        match self {
            Self::IntervalBox(p) => match (p.outcome, polarity) {
                (BoxOutcome::Holds, CellPolarity::Inner)
                | (BoxOutcome::Fails, CellPolarity::Excluded) => Ok(()),
                _ => Err(ProofError::VerdictKindMismatch {
                    kind: "interval-box",
                    expected_polarity: polarity.as_str(),
                }),
            },
            Self::TaylorBox(p) => match (p.outcome, polarity) {
                (BoxOutcome::Holds, CellPolarity::Inner)
                | (BoxOutcome::Fails, CellPolarity::Excluded) => Ok(()),
                _ => Err(ProofError::VerdictKindMismatch {
                    kind: "taylor-box",
                    expected_polarity: polarity.as_str(),
                }),
            },
            _ => Ok(()),
        }
    }

    pub fn encode_canonical(&self, enc: &mut CanonicalEncoder) {
        match self {
            Self::LpPrimal(p) => p.encode_canonical(enc),
            Self::LpFarkas(p) => p.encode_canonical(enc),
            Self::ConicFarkas(p) => p.encode_canonical(enc),
            Self::IntervalBox(p) => p.encode_canonical(enc),
            Self::TaylorBox(p) => p.encode_canonical(enc),
            Self::SosGram(p) => p.encode_canonical(enc),
            Self::FiniteGame(p) => p.encode_canonical(enc),
            Self::AliasPair(p) => p.encode_canonical(enc),
            Self::MonotoneTransport(p) => p.encode_canonical(enc),
            Self::SymmetryTransport(p) => p.encode_canonical(enc),
            Self::Partition(p) => p.encode_canonical(enc),
            Self::Composition(p) => p.encode_canonical(enc),
            Self::ProductUnreachable(p) => p.encode_canonical(enc),
            Self::TraceWitness(p) => p.encode_canonical(enc),
            Self::SampleCoverage(p) => p.encode_canonical(enc),
        }
    }
}
