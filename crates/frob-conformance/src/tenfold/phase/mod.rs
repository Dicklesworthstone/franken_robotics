//! Thirteen phases, product-vocabulary phase map, raw interval records, and
//! exclusive critical-path attribution for the ten-fold harness (`N39a-23`,
//! §39.6, §2.2, §36.15), plus skeleton submodules for `N39a-24`..`N39a-26`.
//!
//! Pure, determinism class D0, zero `unsafe`, zero floating-point arithmetic,
//! prints nothing (rule L-7).

pub mod closure;
pub mod exclusive;
pub mod interval;
pub mod levers;
pub mod resource;
pub mod tagmap;

use crate::tenfold::ids::Phase;
use crate::tenfold::refusal::TenfoldRefusal;

pub use exclusive::{
    AttributionWindow, ExclusiveTable, FLAG_CENSORED_REMAINDER, SegmentDiagnosis,
    attribute_exclusive, diagnose_instant,
};
pub use interval::{
    Interval, IntervalKind, ResourceInfo, parse_interval_line, parse_intervals_bytes,
    parse_intervals_ndjson, turn_phase, write_intervals_ndjson,
};

/// Which product phase vocabulary a source phase name belongs to (`N39a-23` §1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ProductVocabulary {
    /// Studio loop phases (§2.2): `authoring`, `review`, `check`, `synthesis`,
    /// `simulation`, `training`, `promotion`.
    StudioLoop,
    /// Performance program phases (§36.15): `compilation`, `admission`,
    /// `screening`, `training`, `evaluation`, `promotion`, `qualification`, `waiting`.
    PerformanceProgram,
}

impl ProductVocabulary {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StudioLoop => "studio-loop",
            Self::PerformanceProgram => "performance-program",
        }
    }
}

/// One row of the closed mapping from product phase names to ten-fold ledger phases (`N39a-23` §1).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProductPhaseMapping {
    pub vocab: ProductVocabulary,
    pub product_name: &'static str,
    pub phases: &'static [Phase],
}

/// Authoritative closed mapping from FrankenRobotics's two product phase
/// vocabularies (`studio loop` and `performance program`) into the thirteen
/// ten-fold ledger phases (`N39a-23` §1).
///
/// Note that `studio loop: review` maps many-to-one to `[Propose, Diagnose]`
/// (split by tool tags via [`turn_phase`]); `certify` and `adjudicate` have no
/// product phase name.
pub const PRODUCT_PHASE_MAP: &[ProductPhaseMapping] = &[
    ProductPhaseMapping {
        vocab: ProductVocabulary::StudioLoop,
        product_name: "authoring",
        phases: &[Phase::Author],
    },
    ProductPhaseMapping {
        vocab: ProductVocabulary::StudioLoop,
        product_name: "review",
        phases: &[Phase::Propose, Phase::Diagnose],
    },
    ProductPhaseMapping {
        vocab: ProductVocabulary::StudioLoop,
        product_name: "check",
        phases: &[Phase::Check],
    },
    ProductPhaseMapping {
        vocab: ProductVocabulary::StudioLoop,
        product_name: "synthesis",
        phases: &[Phase::Synthesize],
    },
    ProductPhaseMapping {
        vocab: ProductVocabulary::StudioLoop,
        product_name: "simulation",
        phases: &[Phase::Screen],
    },
    ProductPhaseMapping {
        vocab: ProductVocabulary::StudioLoop,
        product_name: "training",
        phases: &[Phase::Train],
    },
    ProductPhaseMapping {
        vocab: ProductVocabulary::StudioLoop,
        product_name: "promotion",
        phases: &[Phase::Evaluate],
    },
    ProductPhaseMapping {
        vocab: ProductVocabulary::PerformanceProgram,
        product_name: "compilation",
        phases: &[Phase::Compile],
    },
    ProductPhaseMapping {
        vocab: ProductVocabulary::PerformanceProgram,
        product_name: "admission",
        phases: &[Phase::Compile],
    },
    ProductPhaseMapping {
        vocab: ProductVocabulary::PerformanceProgram,
        product_name: "screening",
        phases: &[Phase::Screen],
    },
    ProductPhaseMapping {
        vocab: ProductVocabulary::PerformanceProgram,
        product_name: "training",
        phases: &[Phase::Train],
    },
    ProductPhaseMapping {
        vocab: ProductVocabulary::PerformanceProgram,
        product_name: "evaluation",
        phases: &[Phase::Evaluate],
    },
    ProductPhaseMapping {
        vocab: ProductVocabulary::PerformanceProgram,
        product_name: "promotion",
        phases: &[Phase::Evaluate],
    },
    ProductPhaseMapping {
        vocab: ProductVocabulary::PerformanceProgram,
        product_name: "qualification",
        phases: &[Phase::Qualify],
    },
    ProductPhaseMapping {
        vocab: ProductVocabulary::PerformanceProgram,
        product_name: "waiting",
        phases: &[Phase::Wait],
    },
];

/// Map a product phase name in `vocab` to its target ten-fold ledger phase(s).
/// Refuses unknown names with `record-unknown-field` (exit 3).
pub fn map_product_phase(
    vocab: ProductVocabulary,
    product_name: &str,
    pointer: &str,
) -> Result<&'static [Phase], TenfoldRefusal> {
    for row in PRODUCT_PHASE_MAP {
        if row.vocab == vocab && row.product_name == product_name {
            return Ok(row.phases);
        }
    }
    Err(TenfoldRefusal::unknown_field(
        pointer,
        format!("{}:{product_name:?}", vocab.as_str()),
        "registered product phase name in N39a-23 §1 vocabulary table",
    ))
}

/// Return all `(vocabulary, product_name)` pairs that map onto `phase`.
/// Returns an empty vector for `Certify`, `Adjudicate`, and `Unattributed`.
#[must_use]
pub fn product_names_for_phase(phase: Phase) -> Vec<(ProductVocabulary, &'static str)> {
    let mut out = Vec::new();
    for row in PRODUCT_PHASE_MAP {
        if row.phases.contains(&phase) {
            out.push((row.vocab, row.product_name));
        }
    }
    out
}
