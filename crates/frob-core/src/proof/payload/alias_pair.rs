//! `alias-pair` payload (`N25b-01`, §25.5.2).

use crate::proof::num::{CanonicalEncoder, ProofError};

/// Scope over which an alias-pair impossibility holds.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AliasScope {
    AllAdmittedProbes,
    NamedExperiment(String),
}

impl AliasScope {
    pub fn encode_canonical(&self, enc: &mut CanonicalEncoder) {
        match self {
            Self::AllAdmittedProbes => enc.tag(b"all-admitted-probes"),
            Self::NamedExperiment(s) => {
                enc.tag(b"named-experiment");
                enc.str(s);
            }
        }
    }
}

/// Exact payload for `alias-pair`: two worlds, observation sets up to commitment,
/// disjoint success sets, and probe scope.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AliasPairPayload {
    pub world_a: String,
    pub world_b: String,
    pub observation_set_a: Vec<String>,
    pub observation_set_b: Vec<String>,
    pub success_set_a: Vec<String>,
    pub success_set_b: Vec<String>,
    pub scope: AliasScope,
}

impl AliasPairPayload {
    pub fn validate_shape(&self) -> Result<(), ProofError> {
        let kind = "alias-pair";
        if self.world_a.trim().is_empty() || self.world_b.trim().is_empty() {
            return Err(ProofError::EmptyIdentifier { field: "alias_pair.world" });
        }
        if let AliasScope::NamedExperiment(ref s) = self.scope
            && s.trim().is_empty()
        {
            return Err(ProofError::EmptyIdentifier { field: "alias_pair.scope" });
        }
        if self.observation_set_a.is_empty() || self.observation_set_b.is_empty() {
            return Err(ProofError::InvalidPayload {
                kind,
                detail: "observation sets must be non-empty".to_owned(),
            });
        }
        Ok(())
    }

    pub fn encode_canonical(&self, enc: &mut CanonicalEncoder) {
        enc.tag(b"payload:alias-pair");
        enc.str(&self.world_a);
        enc.str(&self.world_b);
        for list in [
            &self.observation_set_a,
            &self.observation_set_b,
            &self.success_set_a,
            &self.success_set_b,
        ] {
            enc.usize(list.len());
            for item in list { enc.str(item); }
        }
        self.scope.encode_canonical(enc);
    }
}
