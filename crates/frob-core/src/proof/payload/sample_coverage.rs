//! `sample-coverage` payload (`N25b-01`, §25.5.2, class cap `Estimated`).

use crate::proof::establishes::PopulationId;
use crate::proof::num::{CanonicalEncoder, ProofError, Ratio};

/// Exact payload for `sample-coverage`: empirical counts under a named population and method.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SampleCoveragePayload {
    pub successes: u64,
    pub trials: u64,
    pub population: PopulationId,
    pub method: String,
    pub bound_ratio: Ratio,
    pub confidence: Ratio,
}

impl SampleCoveragePayload {
    pub fn validate_shape(&self) -> Result<(), ProofError> {
        let kind = "sample-coverage";
        if self.trials == 0 || self.successes > self.trials {
            return Err(ProofError::InvalidPayload {
                kind,
                detail: format!(
                    "invalid counts: successes={} must be <= trials={} and trials > 0",
                    self.successes, self.trials
                ),
            });
        }
        if self.method.trim().is_empty() {
            return Err(ProofError::EmptyIdentifier { field: "sample_coverage.method" });
        }
        let z = Ratio::zero();
        let one = Ratio::one();
        if self.bound_ratio < z || self.bound_ratio > one || self.confidence < z || self.confidence > one {
            return Err(ProofError::InvalidPayload {
                kind,
                detail: "bound_ratio and confidence must lie in [0, 1]".to_owned(),
            });
        }
        Ok(())
    }

    pub fn encode_canonical(&self, enc: &mut CanonicalEncoder) {
        enc.tag(b"payload:sample-coverage");
        enc.u64(self.successes);
        enc.u64(self.trials);
        self.population.encode_canonical(enc);
        enc.str(&self.method);
        self.bound_ratio.encode_canonical(enc);
        self.confidence.encode_canonical(enc);
    }
}
