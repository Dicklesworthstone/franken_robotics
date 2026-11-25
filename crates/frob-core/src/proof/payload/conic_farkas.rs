//! `conic-farkas` payload (`N25b-01`, §25.5.2).

use crate::proof::num::{CanonicalEncoder, ProofError, Ratio};
use crate::proof::qty_types::Unit;

/// Exact payload for `conic-farkas`: rational dual vector in the dual second-order cone.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConicFarkasPayload {
    pub dual_y: Vec<Ratio>,
    pub cone_dims: Vec<usize>,
    pub separation_margin: Ratio,
    pub unit: Unit,
}

impl ConicFarkasPayload {
    pub fn validate_shape(&self) -> Result<(), ProofError> {
        let kind = "conic-farkas";
        if self.dual_y.is_empty() || self.cone_dims.is_empty() {
            return Err(ProofError::InvalidPayload {
                kind,
                detail: "dual_y and cone_dims must be non-empty".to_owned(),
            });
        }
        if self.cone_dims.contains(&0) {
            return Err(ProofError::InvalidPayload {
                kind,
                detail: "cone block dimensions must be positive".to_owned(),
            });
        }
        Ok(())
    }

    pub fn encode_canonical(&self, enc: &mut CanonicalEncoder) {
        enc.tag(b"payload:conic-farkas");
        enc.usize(self.dual_y.len());
        for &r in &self.dual_y { r.encode_canonical(enc); }
        enc.usize(self.cone_dims.len());
        for &d in &self.cone_dims { enc.usize(d); }
        self.separation_margin.encode_canonical(enc);
        self.unit.encode_canonical(enc);
    }
}
