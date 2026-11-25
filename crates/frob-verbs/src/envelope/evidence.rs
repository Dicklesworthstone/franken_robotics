//! The `evidence` block of `frob.envelope/1` (N32a-03, BD-005, N33a-06).
//!
//! The only way to put a result into an envelope is to attach its claim product: an
//! [`EvidenceBlock`] is built from a [`ClaimProduct`] and nothing else. Its fields are
//! private, there is no constructor taking a class, tier, determinism or reality as text,
//! and `ClaimProduct` (BD-005) has no constructor taking a class: `Estimated` is its only
//! open leaf, the stronger classes come through its deprecated doorways (N34-07), and
//! `combine` only weakens. The stack identity (`stack_receipt`, `stack_dev`)
//! is not part of this block: the envelope serializer stamps it from `BUILT_UNDER`.
//!
//! Evidence cannot be built from a string literal:
//!
//! ```compile_fail,E0308
//! use frob_verbs::envelope::EvidenceBlock;
//! let _ = EvidenceBlock::from_claim_product("Tested", "m:9f2c");
//! ```
//!
//! ```compile_fail,E0277
//! use frob_verbs::envelope::EvidenceBlock;
//! let _: EvidenceBlock = "Proved".into();
//! ```
//!
//! Nor from a struct literal, even one whose every field is correctly typed:
//!
//! ```compile_fail,E0451
//! use frob_core::evidence::{DetClass, EvidenceClass};
//! use frob_verbs::envelope::EvidenceBlock;
//! let _ = EvidenceBlock {
//!     class: EvidenceClass::Proved,
//!     tier: String::from("static"),
//!     model_id: String::from("m:x"),
//!     determinism: DetClass::D0Exact,
//!     claims: Vec::new(),
//!     reality: String::from("no-claim"),
//!     seed: None,
//!     weakest: None,
//!     conditions: Vec::new(),
//! };
//! ```

use frob_core::canon::{CanonError, Object};
use frob_core::evidence::{
    ClaimKind, ClaimProduct, ConditionFlag, DetClass, EvidenceClass, RealityScope,
};

use super::EnvelopeError;

/// The claim-product part of `evidence`: the weakest proof class, physics tier and
/// determinism class among everything the result was computed from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvidenceBlock {
    class: EvidenceClass,
    tier: String,
    model_id: String,
    determinism: DetClass,
    claims: Vec<ClaimKind>,
    reality: String,
    seed: Option<u64>,
    weakest: Option<(String, String)>,
    conditions: Vec<ConditionFlag>,
}

impl EvidenceBlock {
    /// Serialize a claim product for the envelope. `model_id` names the model the result
    /// was computed on (an identifier, not a strength); it must be non-empty and free of
    /// control characters.
    pub fn from_claim_product(
        product: &ClaimProduct,
        model_id: &str,
    ) -> Result<Self, EnvelopeError> {
        if model_id.is_empty() || model_id.chars().any(|c| (c as u32) < 0x20 || c == '\u{7f}') {
            return Err(EnvelopeError::InvalidField {
                field: "evidence.model_id",
                reason: "must be a non-empty identifier without control characters".into(),
            });
        }
        let reality = match product.reality() {
            RealityScope::NoClaim => "no-claim".to_owned(),
            RealityScope::CalibratedFamily(f)
                if !f.is_empty() && !f.chars().any(char::is_control) =>
            {
                format!("calibrated:{f}")
            }
            RealityScope::CalibratedFamily(_) => {
                return Err(EnvelopeError::InvalidField {
                    field: "evidence.reality",
                    reason: "calibrated family is empty or has control characters".into(),
                });
            }
        };
        // `weakest` applies when more than one input fed the product: it names the one that
        // set the class. A single-input product's weakest input is that input.
        let weakest = (product.inputs().len() > 1).then(|| {
            (
                product.weakest().digest.to_hex(),
                product.weakest().field_path.clone(),
            )
        });
        Ok(Self {
            class: product.class(),
            tier: product.tier().to_string(),
            model_id: model_id.to_owned(),
            determinism: product.determinism(),
            claims: Vec::new(),
            reality,
            seed: None,
            weakest,
            conditions: product.conditions().iter().copied().collect(),
        })
    }

    /// Name the claims (of the seven) the result may be cited for. Empty for most studio
    /// answers; whether a claim is admissible above the product is refused elsewhere
    /// (`FROB-REF-819`, BD-308).
    #[must_use]
    pub fn with_claims(mut self, claims: &[ClaimKind]) -> Self {
        let mut v: Vec<ClaimKind> = claims.to_vec();
        v.sort();
        v.dedup();
        self.claims = v;
        self
    }

    /// Record the seed the result was computed under (serialized as a decimal string).
    #[must_use]
    pub const fn with_seed(mut self, seed: u64) -> Self {
        self.seed = Some(seed);
        self
    }

    #[must_use]
    pub const fn class(&self) -> EvidenceClass {
        self.class
    }

    #[must_use]
    pub const fn determinism(&self) -> DetClass {
        self.determinism
    }

    /// Write the claim-product members into the envelope's `evidence` object.
    pub(crate) fn write(&self, o: &mut Object<'_>) -> Result<(), CanonError> {
        o.array("claims", |a| {
            for c in &self.claims {
                a.push_str(c.as_str());
            }
            Ok(())
        })?;
        o.str("class", self.class.as_str())?;
        if !self.conditions.is_empty() {
            o.array("conditions", |a| {
                for c in &self.conditions {
                    a.push_str(c.as_str());
                }
                Ok(())
            })?;
        }
        o.str("determinism", self.determinism.as_str())?;
        o.str("model_id", &self.model_id)?;
        o.str("reality", &self.reality)?;
        if let Some(seed) = self.seed {
            o.u64str("seed", seed)?;
        }
        o.str("tier", &self.tier)?;
        if let Some((digest, field)) = &self.weakest {
            o.object("weakest", |w| {
                w.str("digest", digest)?;
                w.str("field_path", field)
            })?;
        }
        Ok(())
    }
}
