//! `lp-farkas` payload (`N25b-01`, §25.5.2, KA-624).

use crate::proof::expr::ExprDag;
use crate::proof::num::{CanonicalEncoder, ProofError, Ratio};
use crate::proof::qty_types::{FrameSpec, Unit};

/// Dual vector `y`: either constant exact rationals or rational expressions in cell parameters.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DualVector {
    Constant(Vec<Ratio>),
    RationalInChart(Vec<ExprDag>),
}

impl DualVector {
    #[must_use]
    pub fn len(&self) -> usize {
        match self {
            Self::Constant(v) => v.len(),
            Self::RationalInChart(v) => v.len(),
        }
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn encode_canonical(&self, enc: &mut CanonicalEncoder) {
        match self {
            Self::Constant(v) => {
                enc.tag(b"dual_const");
                enc.usize(v.len());
                for &r in v { r.encode_canonical(enc); }
            }
            Self::RationalInChart(v) => {
                enc.tag(b"dual_chart");
                enc.usize(v.len());
                for d in v { d.encode_canonical(enc); }
            }
        }
    }
}

/// Exact payload for `lp-farkas`: relaxation `A * f = w` with `0 <= f <= c`, dual `y`, sign proof.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LpFarkasPayload {
    pub matrix_a: Vec<Vec<Ratio>>,
    pub rhs_w: Vec<Ratio>,
    pub capacities: Vec<Ratio>,
    pub dual_y: DualVector,
    pub sign_proof: String,
    pub sign_proof_dag: Option<ExprDag>,
    pub unit: Unit,
    pub frame: FrameSpec,
}

impl LpFarkasPayload {
    pub fn validate_shape(&self) -> Result<(), ProofError> {
        let kind = "lp-farkas";
        if self.matrix_a.is_empty() {
            return Err(ProofError::InvalidPayload {
                kind,
                detail: "matrix_a must have at least one row".to_owned(),
            });
        }
        let cols = self.capacities.len();
        if cols == 0 {
            return Err(ProofError::InvalidPayload {
                kind,
                detail: "capacities must be non-empty".to_owned(),
            });
        }
        for (i, row) in self.matrix_a.iter().enumerate() {
            if row.len() != cols {
                return Err(ProofError::InvalidPayload {
                    kind,
                    detail: format!("matrix_a row {i} has len {}, expected {cols}", row.len()),
                });
            }
        }
        if self.rhs_w.len() != self.matrix_a.len() {
            return Err(ProofError::InvalidPayload {
                kind,
                detail: format!(
                    "rhs_w len ({}) must equal matrix_a rows ({})",
                    self.rhs_w.len(),
                    self.matrix_a.len()
                ),
            });
        }
        if self.dual_y.len() != self.matrix_a.len() {
            return Err(ProofError::InvalidPayload {
                kind,
                detail: format!(
                    "dual_y len ({}) must equal matrix_a rows ({})",
                    self.dual_y.len(),
                    self.matrix_a.len()
                ),
            });
        }
        if self.sign_proof.trim().is_empty() {
            return Err(ProofError::EmptyIdentifier { field: "lp_farkas.sign_proof" });
        }
        Ok(())
    }

    pub fn encode_canonical(&self, enc: &mut CanonicalEncoder) {
        enc.tag(b"payload:lp-farkas");
        enc.usize(self.matrix_a.len());
        for row in &self.matrix_a {
            enc.usize(row.len());
            for &r in row { r.encode_canonical(enc); }
        }
        enc.usize(self.rhs_w.len());
        for &r in &self.rhs_w { r.encode_canonical(enc); }
        enc.usize(self.capacities.len());
        for &r in &self.capacities { r.encode_canonical(enc); }
        self.dual_y.encode_canonical(enc);
        enc.str(&self.sign_proof);
        match &self.sign_proof_dag {
            Some(d) => { enc.bool(true); d.encode_canonical(enc); }
            None => enc.bool(false),
        }
        self.unit.encode_canonical(enc);
        self.frame.encode_canonical(enc);
    }
}
