//! `sos-gram` payload (`N25b-01`, §25.5.2).

use crate::proof::establishes::{Bound, Var};
use crate::proof::num::{CanonicalEncoder, ProofError, Ratio};

/// Box-constraint multiplier `sigma_i(x)` represented by its rational Gram matrix.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SosMultiplier {
    pub var: Var,
    pub gram: Vec<Vec<Ratio>>,
}

impl SosMultiplier {
    pub fn validate_shape(&self) -> Result<(), ProofError> {
        let n = self.gram.len();
        if n == 0 || self.gram.iter().any(|row| row.len() != n) {
            return Err(ProofError::InvalidPayload {
                kind: "sos-gram",
                detail: "multiplier gram matrix must be non-empty and square".to_owned(),
            });
        }
        Ok(())
    }

    pub fn encode_canonical(&self, enc: &mut CanonicalEncoder) {
        enc.tag(b"sos_mult");
        self.var.encode_canonical(enc);
        enc.usize(self.gram.len());
        for row in &self.gram {
            enc.usize(row.len());
            for &r in row { r.encode_canonical(enc); }
        }
    }
}

/// Exact payload for `sos-gram`: rational Gram matrix and box multipliers certifying nonnegativity.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SosGramPayload {
    pub box_bounds: Vec<Bound>,
    pub monomial_basis: Vec<Vec<u32>>,
    pub gram_matrix: Vec<Vec<Ratio>>,
    pub multipliers: Vec<SosMultiplier>,
}

impl SosGramPayload {
    pub fn validate_shape(&self) -> Result<(), ProofError> {
        let kind = "sos-gram";
        let n = self.gram_matrix.len();
        if n == 0 || self.monomial_basis.len() != n {
            return Err(ProofError::InvalidPayload {
                kind,
                detail: "gram_matrix must be non-empty and match monomial_basis length".to_owned(),
            });
        }
        if self.gram_matrix.iter().any(|row| row.len() != n) {
            return Err(ProofError::InvalidPayload {
                kind,
                detail: "gram_matrix must be square".to_owned(),
            });
        }
        for m in &self.multipliers {
            m.validate_shape()?;
        }
        Ok(())
    }

    pub fn encode_canonical(&self, enc: &mut CanonicalEncoder) {
        enc.tag(b"payload:sos-gram");
        enc.usize(self.box_bounds.len());
        for b in &self.box_bounds { b.encode_canonical(enc); }
        enc.usize(self.monomial_basis.len());
        for m in &self.monomial_basis {
            enc.usize(m.len());
            for &e in m { enc.u32(e); }
        }
        enc.usize(self.gram_matrix.len());
        for row in &self.gram_matrix {
            enc.usize(row.len());
            for &r in row { r.encode_canonical(enc); }
        }
        enc.usize(self.multipliers.len());
        for mult in &self.multipliers { mult.encode_canonical(enc); }
    }
}
