//! `partition` payload (`N25b-01`, §25.5.2).

use crate::proof::establishes::{Cell, ProofRef};
use crate::proof::num::{CanonicalEncoder, ProofError};

/// One sub-cell of a partition together with the proof reference it names.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PartitionCell {
    pub cell: Cell,
    pub proof: ProofRef,
}

impl PartitionCell {
    pub fn encode_canonical(&self, enc: &mut CanonicalEncoder) {
        enc.tag(b"part_cell");
        self.cell.encode_canonical(enc);
        self.proof.encode_canonical(enc);
    }
}

/// Exact payload for `partition`: region covered exactly by listed cells with their proofs.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PartitionPayload {
    pub region: Cell,
    pub sub_cells: Vec<PartitionCell>,
}

impl PartitionPayload {
    pub fn validate_shape(&self) -> Result<(), ProofError> {
        if self.sub_cells.is_empty() {
            return Err(ProofError::InvalidPayload {
                kind: "partition",
                detail: "partition must list at least one sub-cell".to_owned(),
            });
        }
        Ok(())
    }

    pub fn encode_canonical(&self, enc: &mut CanonicalEncoder) {
        enc.tag(b"payload:partition");
        self.region.encode_canonical(enc);
        enc.usize(self.sub_cells.len());
        for sc in &self.sub_cells { sc.encode_canonical(enc); }
    }
}
