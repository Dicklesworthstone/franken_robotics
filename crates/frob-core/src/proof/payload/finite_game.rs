//! `finite-game` payload (`N25b-01`, §25.5.2).

use std::collections::BTreeMap;

use crate::proof::establishes::Bound;
use crate::proof::expr::Equation;
use crate::proof::num::{CanonicalEncoder, ProofError, Ratio};

/// Exact comparator attached to a terminal predicate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Comparator {
    Lt,
    Le,
    Gt,
    Ge,
    Eq,
}

impl Comparator {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Lt => "<",
            Self::Le => "<=",
            Self::Gt => ">",
            Self::Ge => ">=",
            Self::Eq => "==",
        }
    }
}

/// Exact payload for `finite-game`: causal policy table keyed by observation history,
/// worlds, error vertices, command bounds, and terminal predicate with comparator.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FiniteGamePayload {
    pub policy_table: BTreeMap<Vec<String>, Vec<Ratio>>,
    pub worlds: Vec<String>,
    pub error_vertices: Vec<Vec<Ratio>>,
    pub command_bounds: Vec<Bound>,
    pub terminal_predicate: Equation,
    pub comparator: Comparator,
}

impl FiniteGamePayload {
    pub fn validate_shape(&self) -> Result<(), ProofError> {
        let kind = "finite-game";
        if self.policy_table.is_empty() || self.worlds.is_empty() {
            return Err(ProofError::InvalidPayload {
                kind,
                detail: "policy_table and worlds must be non-empty".to_owned(),
            });
        }
        Ok(())
    }

    pub fn encode_canonical(&self, enc: &mut CanonicalEncoder) {
        enc.tag(b"payload:finite-game");
        enc.usize(self.policy_table.len());
        for (hist, cmd) in &self.policy_table {
            enc.usize(hist.len());
            for h in hist { enc.str(h); }
            enc.usize(cmd.len());
            for &r in cmd { r.encode_canonical(enc); }
        }
        enc.usize(self.worlds.len());
        for w in &self.worlds { enc.str(w); }
        enc.usize(self.error_vertices.len());
        for ev in &self.error_vertices {
            enc.usize(ev.len());
            for &r in ev { r.encode_canonical(enc); }
        }
        enc.usize(self.command_bounds.len());
        for b in &self.command_bounds { b.encode_canonical(enc); }
        self.terminal_predicate.encode_canonical(enc);
        enc.str(self.comparator.as_str());
    }
}
