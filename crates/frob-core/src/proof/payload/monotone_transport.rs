//! `monotone-transport` payload (`N25b-01`, §25.5.2).

use crate::proof::establishes::{ProofRef, Var};
use crate::proof::num::{CanonicalEncoder, Interval, ProofError};

/// Exact payload for `monotone-transport`: order direction (`+1` or `-1` per variable),
/// sign enclosures, side-condition source certificates, and model stratum identifier.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MonotoneTransportPayload {
    pub direction: Vec<(Var, i8)>,
    pub sign_enclosures: Vec<(Var, Interval)>,
    pub source_certificates: Vec<ProofRef>,
    pub model_stratum: String,
}

impl MonotoneTransportPayload {
    pub fn validate_shape(&self) -> Result<(), ProofError> {
        let kind = "monotone-transport";
        if self.direction.is_empty() || self.source_certificates.is_empty() {
            return Err(ProofError::InvalidPayload {
                kind,
                detail: "direction and source_certificates must be non-empty".to_owned(),
            });
        }
        for (v, sign) in &self.direction {
            if *sign != -1 && *sign != 1 {
                return Err(ProofError::InvalidPayload {
                    kind,
                    detail: format!("direction sign for `{}` must be -1 or +1, got {sign}", v.as_str()),
                });
            }
        }
        if self.model_stratum.trim().is_empty() {
            return Err(ProofError::EmptyIdentifier { field: "monotone_transport.model_stratum" });
        }
        Ok(())
    }

    pub fn encode_canonical(&self, enc: &mut CanonicalEncoder) {
        enc.tag(b"payload:monotone-transport");
        enc.usize(self.direction.len());
        for (v, s) in &self.direction {
            v.encode_canonical(enc);
            enc.i8(*s);
        }
        enc.usize(self.sign_enclosures.len());
        for (v, iv) in &self.sign_enclosures {
            v.encode_canonical(enc);
            iv.encode_canonical(enc);
        }
        enc.usize(self.source_certificates.len());
        for p in &self.source_certificates { p.encode_canonical(enc); }
        enc.str(&self.model_stratum);
    }
}
