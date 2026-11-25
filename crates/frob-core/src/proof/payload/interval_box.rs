//! `interval-box` payload (`N25b-01`, §25.5.2).

use crate::id::Blake3;
use crate::proof::establishes::{Bound, Var};
use crate::proof::num::{CanonicalEncoder, Interval, ProofError};

/// Claimed outcome of an `interval-box` or `taylor-box` constraint enclosure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BoxOutcome {
    Holds,
    Fails,
}

impl BoxOutcome {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Holds => "holds",
            Self::Fails => "fails",
        }
    }
}

/// Exact payload for `interval-box`: expression DAG digest, box, split set (`<= 6`),
/// outward-rounded enclosure, and `holds` / `fails` verdict.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IntervalBoxPayload {
    pub expr_digest: Blake3,
    pub box_bounds: Vec<Bound>,
    pub split_set: Vec<Var>,
    pub enclosure: Interval,
    pub outcome: BoxOutcome,
}

impl IntervalBoxPayload {
    pub const MAX_SPLIT_PARAMS: usize = 6;

    pub fn validate_shape(&self) -> Result<(), ProofError> {
        let kind = "interval-box";
        if self.box_bounds.is_empty() {
            return Err(ProofError::InvalidPayload {
                kind,
                detail: "box_bounds must be non-empty".to_owned(),
            });
        }
        if self.split_set.len() > Self::MAX_SPLIT_PARAMS {
            return Err(ProofError::InvalidPayload {
                kind,
                detail: format!(
                    "split_set has {} parameters, maximum is {}",
                    self.split_set.len(),
                    Self::MAX_SPLIT_PARAMS
                ),
            });
        }
        Ok(())
    }

    pub fn encode_canonical(&self, enc: &mut CanonicalEncoder) {
        enc.tag(b"payload:interval-box");
        enc.blake3(self.expr_digest);
        enc.usize(self.box_bounds.len());
        for b in &self.box_bounds { b.encode_canonical(enc); }
        enc.usize(self.split_set.len());
        for v in &self.split_set { v.encode_canonical(enc); }
        self.enclosure.encode_canonical(enc);
        enc.str(self.outcome.as_str());
    }
}
