//! `trace-witness` payload (`N25b-01`, §25.5.2, class cap `Tested`).

use crate::id::Blake3;
use crate::proof::num::{CanonicalEncoder, ProofError};

/// Exact payload for `trace-witness`: one replayed execution refuting a fixed design and
/// controller, or separating two designs.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TraceWitnessPayload {
    pub replay_address: String,
    pub trace_digest: Blake3,
    pub monitor_verdict: String,
    pub separates_design: Option<String>,
}

impl TraceWitnessPayload {
    pub fn validate_shape(&self) -> Result<(), ProofError> {
        if self.replay_address.trim().is_empty() {
            return Err(ProofError::EmptyIdentifier { field: "trace_witness.replay_address" });
        }
        if self.monitor_verdict.trim().is_empty() {
            return Err(ProofError::EmptyIdentifier { field: "trace_witness.monitor_verdict" });
        }
        if let Some(ref sep) = self.separates_design
            && sep.trim().is_empty()
        {
            return Err(ProofError::EmptyIdentifier { field: "trace_witness.separates_design" });
        }
        Ok(())
    }

    pub fn encode_canonical(&self, enc: &mut CanonicalEncoder) {
        enc.tag(b"payload:trace-witness");
        enc.str(&self.replay_address);
        enc.blake3(self.trace_digest);
        enc.str(&self.monitor_verdict);
        match &self.separates_design {
            Some(s) => { enc.bool(true); enc.str(s); }
            None => enc.bool(false),
        }
    }
}
