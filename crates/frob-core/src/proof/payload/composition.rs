//! `composition` payload (`N25b-01`, §25.5.2).

use crate::proof::establishes::ProofRef;
use crate::proof::num::{CanonicalEncoder, ProofError, Ratio};
use crate::proof::qty_types::Unit;

/// One stage-to-stage inclusion link (`end_set subset entry_set`) in a composition chain.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StageLink {
    pub from_stage: String,
    pub to_stage: String,
    pub inclusion_proof: ProofRef,
}

impl StageLink {
    pub fn validate_shape(&self) -> Result<(), ProofError> {
        if self.from_stage.trim().is_empty() || self.to_stage.trim().is_empty() {
            return Err(ProofError::EmptyIdentifier { field: "composition.stage" });
        }
        Ok(())
    }

    pub fn encode_canonical(&self, enc: &mut CanonicalEncoder) {
        enc.tag(b"stage_link");
        enc.str(&self.from_stage);
        enc.str(&self.to_stage);
        self.inclusion_proof.encode_canonical(enc);
    }
}

/// Exact clock or resource budget ledger entry for a composed stage.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LedgerEntry {
    pub name: String,
    pub spent: Ratio,
    pub limit: Ratio,
    pub unit: Unit,
}

impl LedgerEntry {
    pub fn validate_shape(&self) -> Result<(), ProofError> {
        if self.name.trim().is_empty() {
            return Err(ProofError::EmptyIdentifier { field: "composition.ledger.name" });
        }
        if self.spent.is_neg() || self.limit.is_neg() {
            return Err(ProofError::InvalidPayload {
                kind: "composition",
                detail: format!("ledger `{}` spent/limit must be non-negative", self.name),
            });
        }
        Ok(())
    }

    pub fn encode_canonical(&self, enc: &mut CanonicalEncoder) {
        enc.tag(b"ledger");
        enc.str(&self.name);
        self.spent.encode_canonical(enc);
        self.limit.encode_canonical(enc);
        self.unit.encode_canonical(enc);
    }
}

/// Exact payload for `composition`: end-set ⊆ entry-set sub-certificates, clock and resource ledgers.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CompositionPayload {
    pub stage_links: Vec<StageLink>,
    pub clock_ledger: Vec<LedgerEntry>,
    pub resource_ledger: Vec<LedgerEntry>,
}

impl CompositionPayload {
    pub fn validate_shape(&self) -> Result<(), ProofError> {
        if self.stage_links.is_empty() {
            return Err(ProofError::InvalidPayload {
                kind: "composition",
                detail: "stage_links must be non-empty".to_owned(),
            });
        }
        for s in &self.stage_links { s.validate_shape()?; }
        for c in &self.clock_ledger { c.validate_shape()?; }
        for r in &self.resource_ledger { r.validate_shape()?; }
        Ok(())
    }

    pub fn encode_canonical(&self, enc: &mut CanonicalEncoder) {
        enc.tag(b"payload:composition");
        enc.usize(self.stage_links.len());
        for s in &self.stage_links { s.encode_canonical(enc); }
        enc.usize(self.clock_ledger.len());
        for c in &self.clock_ledger { c.encode_canonical(enc); }
        enc.usize(self.resource_ledger.len());
        for r in &self.resource_ledger { r.encode_canonical(enc); }
    }
}
