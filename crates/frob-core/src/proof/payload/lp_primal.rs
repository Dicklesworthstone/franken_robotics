//! `lp-primal` payload (`N25b-01`, §25.5.2, KA-624, CX-108).

use crate::proof::establishes::{Bound, Var};
use crate::proof::num::{CanonicalEncoder, ProofError, Ratio};
use crate::proof::qty_types::{
    ExactQty, FirstMoment, FrameSpec, OrderedPorts, PortForceVector, TorqueVec,
};

/// Interpolation / allocation law for an `lp-primal` witness.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LpLaw {
    MultilinearVertex { vars: Vec<Var> },
    Barycentric { vars: Vec<Var> },
    Affine { vars: Vec<Var> },
    Constant,
}

impl LpLaw {
    pub fn encode_canonical(&self, enc: &mut CanonicalEncoder) {
        match self {
            Self::MultilinearVertex { vars } => {
                enc.tag(b"multilinear-vertex");
                enc.usize(vars.len());
                for v in vars { v.encode_canonical(enc); }
            }
            Self::Barycentric { vars } => {
                enc.tag(b"barycentric");
                enc.usize(vars.len());
                for v in vars { v.encode_canonical(enc); }
            }
            Self::Affine { vars } => {
                enc.tag(b"affine");
                enc.usize(vars.len());
                for v in vars { v.encode_canonical(enc); }
            }
            Self::Constant => enc.tag(b"constant"),
        }
    }
}

/// One load vertex and its port-bound force witness.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LoadVertexWitness {
    pub load_weight: ExactQty,
    pub first_moment: FirstMoment,
    pub support_torque: TorqueVec,
    pub forces: PortForceVector,
}

impl LoadVertexWitness {
    pub fn encode_canonical(&self, enc: &mut CanonicalEncoder) {
        enc.tag(b"lp_vertex");
        self.load_weight.encode_canonical(enc);
        self.first_moment.encode_canonical(b"first_moment", enc);
        self.support_torque.encode_canonical(b"torque", enc);
        self.forces.encode_canonical(enc);
    }
}

/// Force vectors at the lower and upper ends of a chart variable for a cell-wide family.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CellEndForces {
    pub chart_var: Var,
    pub lower_end_forces: PortForceVector,
    pub upper_end_forces: PortForceVector,
}

impl CellEndForces {
    pub fn encode_canonical(&self, enc: &mut CanonicalEncoder) {
        enc.tag(b"cell_end_forces");
        self.chart_var.encode_canonical(enc);
        self.lower_end_forces.encode_canonical(enc);
        self.upper_end_forces.encode_canonical(enc);
    }
}

/// Exact payload for `lp-primal`: feasible point per vertex or affine-in-chart family over a cell.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LpPrimalPayload {
    pub ports: OrderedPorts,
    pub contact_coords: Vec<[ExactQty; 2]>,
    pub capacities: PortForceVector,
    pub authority: String,
    pub observed_before_decision: Vec<String>,
    pub vertices: Vec<LoadVertexWitness>,
    pub cell_end_forces: Vec<CellEndForces>,
    pub law_class: String,
    pub law: LpLaw,
    pub bounds_used: Vec<Bound>,
    pub frame: FrameSpec,
}

impl LpPrimalPayload {
    pub fn validate_shape(&self) -> Result<(), ProofError> {
        let kind = "lp-primal";
        if self.contact_coords.len() != self.ports.len() {
            return Err(ProofError::PortLengthMismatch {
                expected: self.ports.len(),
                observed: self.contact_coords.len(),
            });
        }
        if self.capacities.ports() != &self.ports {
            return Err(ProofError::InvalidPayload {
                kind,
                detail: "capacities port order must match payload ports".to_owned(),
            });
        }
        if self.authority.trim().is_empty() {
            return Err(ProofError::EmptyIdentifier { field: "lp_primal.authority" });
        }
        if self.law_class.trim().is_empty() {
            return Err(ProofError::EmptyIdentifier { field: "lp_primal.law_class" });
        }
        if self.vertices.is_empty() && self.cell_end_forces.is_empty() {
            return Err(ProofError::InvalidPayload {
                kind,
                detail: "must provide at least one vertex or cell-end force witness".to_owned(),
            });
        }
        for v in &self.vertices {
            if v.forces.ports() != &self.ports {
                return Err(ProofError::InvalidPayload {
                    kind,
                    detail: "vertex forces port order must match payload ports".to_owned(),
                });
            }
        }
        for c in &self.cell_end_forces {
            if c.lower_end_forces.ports() != &self.ports || c.upper_end_forces.ports() != &self.ports {
                return Err(ProofError::InvalidPayload {
                    kind,
                    detail: "cell-end forces port order must match payload ports".to_owned(),
                });
            }
        }
        let force_in_frame =
            |f: &PortForceVector| f.unit().as_str() == "N" && f.frame() == &self.frame;
        if self.frame.origin().trim().is_empty()
            || self.contact_coords.iter().flatten().any(|q| {
                q.unit().as_str() != "m" || q.frame() != Some(&self.frame)
            })
            || !force_in_frame(&self.capacities)
            || self.vertices.iter().any(|v| {
                v.load_weight.unit().as_str() != "N"
                    || v.load_weight.frame() != Some(&self.frame)
                    || !v.first_moment.unit().is_newton_meters()
                    || v.first_moment.frame() != &self.frame
                    || !v.support_torque.unit().is_newton_meters()
                    || v.support_torque.frame() != &self.frame
                    || !force_in_frame(&v.forces)
            })
            || self.cell_end_forces.iter().any(|c| {
                !force_in_frame(&c.lower_end_forces) || !force_in_frame(&c.upper_end_forces)
            })
        {
            return Err(ProofError::InvalidPayload {
                kind,
                detail: "contact lengths, forces, and moments require their physical units and the payload frame"
                    .to_owned(),
            });
        }
        Ok(())
    }

    pub fn encode_canonical(&self, enc: &mut CanonicalEncoder) {
        enc.tag(b"payload:lp-primal");
        self.ports.encode_canonical(enc);
        enc.usize(self.contact_coords.len());
        for [cx, cy] in &self.contact_coords {
            cx.encode_canonical(enc);
            cy.encode_canonical(enc);
        }
        self.capacities.encode_canonical(enc);
        enc.str(&self.authority);
        enc.usize(self.observed_before_decision.len());
        for obs in &self.observed_before_decision {
            enc.str(obs);
        }
        enc.usize(self.vertices.len());
        for v in &self.vertices {
            v.encode_canonical(enc);
        }
        enc.usize(self.cell_end_forces.len());
        for c in &self.cell_end_forces {
            c.encode_canonical(enc);
        }
        enc.str(&self.law_class);
        self.law.encode_canonical(enc);
        enc.usize(self.bounds_used.len());
        for b in &self.bounds_used {
            b.encode_canonical(enc);
        }
        self.frame.encode_canonical(enc);
    }
}

/// Helper for inspecting raw force vectors in tests without exposing unvalidated construction.
#[must_use]
pub fn vertex_force_values(v: &LoadVertexWitness) -> &[Ratio] {
    v.forces.values()
}
