//! `taylor-box` payload (`N25b-01`, §25.5.2).

use crate::id::Blake3;
use crate::proof::establishes::{Bound, Var};
use crate::proof::num::{CanonicalEncoder, Interval, ProofError, Ratio};
use crate::proof::payload::interval_box::{BoxOutcome, IntervalBoxPayload};

/// Exact payload for `taylor-box`: polynomial coefficients, outward-rounded remainder bound,
/// claimed enclosure, and outcome over a box (`<= 6` split parameters).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TaylorBoxPayload {
    pub expr_digest: Blake3,
    pub box_bounds: Vec<Bound>,
    pub split_set: Vec<Var>,
    pub coefficients: Vec<(Vec<u32>, Ratio)>,
    pub remainder: Interval,
    pub enclosure: Interval,
    pub outcome: BoxOutcome,
}

impl TaylorBoxPayload {
    pub fn validate_shape(&self) -> Result<(), ProofError> {
        let kind = "taylor-box";
        if self.box_bounds.is_empty() || self.coefficients.is_empty() {
            return Err(ProofError::InvalidPayload {
                kind,
                detail: "box_bounds and coefficients must be non-empty".to_owned(),
            });
        }
        if self.split_set.len() > IntervalBoxPayload::MAX_SPLIT_PARAMS {
            return Err(ProofError::InvalidPayload {
                kind,
                detail: format!(
                    "split_set has {} parameters, maximum is {}",
                    self.split_set.len(),
                    IntervalBoxPayload::MAX_SPLIT_PARAMS
                ),
            });
        }
        Ok(())
    }

    pub fn encode_canonical(&self, enc: &mut CanonicalEncoder) {
        enc.tag(b"payload:taylor-box");
        enc.blake3(self.expr_digest);
        enc.usize(self.box_bounds.len());
        for b in &self.box_bounds { b.encode_canonical(enc); }
        enc.usize(self.split_set.len());
        for v in &self.split_set { v.encode_canonical(enc); }
        enc.usize(self.coefficients.len());
        for (exps, coeff) in &self.coefficients {
            enc.usize(exps.len());
            for &e in exps { enc.u32(e); }
            coeff.encode_canonical(enc);
        }
        self.remainder.encode_canonical(enc);
        self.enclosure.encode_canonical(enc);
        enc.str(self.outcome.as_str());
    }
}
