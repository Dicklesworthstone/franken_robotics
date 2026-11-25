//! `symmetry-transport` payload (`N25b-01`, §25.5.2).

use std::collections::BTreeSet;

use crate::proof::establishes::ProofRef;
use crate::proof::num::{CanonicalEncoder, ProofError};

/// Exact payload for `symmetry-transport`: signed permutation and invariance witnesses
/// for mission, world set, and policy class.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SymmetryTransportPayload {
    pub signed_permutation: Vec<(usize, i8)>,
    pub mission_invariant: String,
    pub world_set_invariant: String,
    pub policy_class_invariant: String,
    pub source_certificate: ProofRef,
}

impl SymmetryTransportPayload {
    pub fn validate_shape(&self) -> Result<(), ProofError> {
        let kind = "symmetry-transport";
        let n = self.signed_permutation.len();
        if n == 0 {
            return Err(ProofError::InvalidPayload {
                kind,
                detail: "signed_permutation must be non-empty".to_owned(),
            });
        }
        let mut seen = BTreeSet::new();
        for &(idx, sign) in &self.signed_permutation {
            if idx >= n || !seen.insert(idx) {
                return Err(ProofError::InvalidPayload {
                    kind,
                    detail: format!("invalid permutation index {idx} for length {n}"),
                });
            }
            if sign != -1 && sign != 1 {
                return Err(ProofError::InvalidPayload {
                    kind,
                    detail: format!("permutation sign must be -1 or +1, got {sign}"),
                });
            }
        }
        if self.mission_invariant.trim().is_empty()
            || self.world_set_invariant.trim().is_empty()
            || self.policy_class_invariant.trim().is_empty()
        {
            return Err(ProofError::EmptyIdentifier { field: "symmetry_transport.invariant" });
        }
        Ok(())
    }

    pub fn encode_canonical(&self, enc: &mut CanonicalEncoder) {
        enc.tag(b"payload:symmetry-transport");
        enc.usize(self.signed_permutation.len());
        for &(idx, sign) in &self.signed_permutation {
            enc.usize(idx);
            enc.i8(sign);
        }
        enc.str(&self.mission_invariant);
        enc.str(&self.world_set_invariant);
        enc.str(&self.policy_class_invariant);
        self.source_certificate.encode_canonical(enc);
    }
}
