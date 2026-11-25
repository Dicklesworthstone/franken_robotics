//! `product-unreachable` payload (`N25b-01`, §25.5.2).

use crate::proof::num::{CanonicalEncoder, Interval, ProofError};

/// Exact payload for `product-unreachable`: enclosure of joint outcomes showing
/// `(A succeeds, B fails)` is unreachable, with its coverage argument.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProductUnreachablePayload {
    pub outcome_a: String,
    pub outcome_b: String,
    pub joint_enclosure: Vec<Interval>,
    pub coverage_argument: String,
}

impl ProductUnreachablePayload {
    pub fn validate_shape(&self) -> Result<(), ProofError> {
        let kind = "product-unreachable";
        if self.outcome_a.trim().is_empty() || self.outcome_b.trim().is_empty() {
            return Err(ProofError::EmptyIdentifier { field: "product_unreachable.outcome" });
        }
        if self.joint_enclosure.is_empty() {
            return Err(ProofError::InvalidPayload {
                kind,
                detail: "joint_enclosure must be non-empty".to_owned(),
            });
        }
        if self.coverage_argument.trim().is_empty() {
            return Err(ProofError::EmptyIdentifier { field: "product_unreachable.coverage_argument" });
        }
        Ok(())
    }

    pub fn encode_canonical(&self, enc: &mut CanonicalEncoder) {
        enc.tag(b"payload:product-unreachable");
        enc.str(&self.outcome_a);
        enc.str(&self.outcome_b);
        enc.usize(self.joint_enclosure.len());
        for iv in &self.joint_enclosure { iv.encode_canonical(enc); }
        enc.str(&self.coverage_argument);
    }
}
