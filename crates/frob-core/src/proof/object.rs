//! Plain [`ProofObject`] with shape-enforcing constructor and deterministic D0 encoding (`N25b-01`).

use crate::id::Blake3;
use crate::proof::establishes::{CellRef, Establishes, ProofRef};
use crate::proof::expr::Equation;
use crate::proof::kinds::{Expected, ProofKind};
use crate::proof::num::{CanonicalEncoder, ProofError};
use crate::proof::payload::ProofPayload;
use crate::proof::support::{Assumption, Support};

/// Plain proof object emitted by `frob-mission` (layer 5) and read by `frob-lemma` (layer 6)
/// without either crate linking `frob-cert` (`OPEN-AUGUR-6`, `ADR-WS-02`, `N25b-01`).
///
/// All fields are private so invariants (kind/payload match, verdict/class cap, non-empty
/// support for non-`partition` kinds, port-bound force vectors, exact numbers) hold by construction.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProofObject {
    id: ProofRef,
    kind: ProofKind,
    cell: CellRef,
    establishes: Establishes,
    payload: ProofPayload,
    side_conditions: Vec<ProofRef>,
    primitive_equations: Vec<Equation>,
    assumptions: Vec<Assumption>,
    support: Support,
    expects: Expected,
}

impl ProofObject {
    /// Construct and validate the structural shape of a proof object (never checks truth).
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: ProofRef,
        kind: ProofKind,
        cell: CellRef,
        establishes: Establishes,
        payload: ProofPayload,
        side_conditions: Vec<ProofRef>,
        primitive_equations: Vec<Equation>,
        assumptions: Vec<Assumption>,
        support: Support,
        expects: Expected,
    ) -> Result<Self, ProofError> {
        if payload.kind() != kind {
            return Err(ProofError::KindPayloadMismatch {
                declared: kind.as_str(),
                payload: payload.kind().as_str(),
            });
        }
        payload.validate_shape()?;

        if !kind.allows_polarity(expects.polarity) {
            return Err(ProofError::VerdictKindMismatch {
                kind: kind.as_str(),
                expected_polarity: expects.polarity.as_str(),
            });
        }
        payload.check_polarity_agreement(expects.polarity)?;

        let cap = kind.class_cap();
        if expects.class > cap {
            return Err(ProofError::ClassExceedsKindCap {
                kind: kind.as_str(),
                requested: expects.class.as_str(),
                max_allowed: cap.as_str(),
            });
        }

        #[cfg(not(feature = "nc-allow-empty-support"))]
        if kind != ProofKind::Partition && support.is_empty() {
            return Err(ProofError::EmptySupport { kind: kind.as_str() });
        }

        Ok(Self {
            id,
            kind,
            cell,
            establishes,
            payload,
            side_conditions,
            primitive_equations,
            assumptions,
            support,
            expects,
        })
    }

    #[must_use]
    pub const fn id(&self) -> &ProofRef {
        &self.id
    }

    #[must_use]
    pub const fn kind(&self) -> ProofKind {
        self.kind
    }

    #[must_use]
    pub const fn cell(&self) -> &CellRef {
        &self.cell
    }

    #[must_use]
    pub const fn establishes(&self) -> &Establishes {
        &self.establishes
    }

    #[must_use]
    pub const fn payload(&self) -> &ProofPayload {
        &self.payload
    }

    #[must_use]
    pub fn side_conditions(&self) -> &[ProofRef] {
        &self.side_conditions
    }

    #[must_use]
    pub fn primitive_equations(&self) -> &[Equation] {
        &self.primitive_equations
    }

    #[must_use]
    pub fn assumptions(&self) -> &[Assumption] {
        &self.assumptions
    }

    #[must_use]
    pub const fn support(&self) -> &Support {
        &self.support
    }

    #[must_use]
    pub const fn expects(&self) -> Expected {
        self.expects
    }

    /// Deterministic D0 canonical byte encoding of the entire proof object.
    ///
    /// Preserves port order, bound endpoints, prefix order, equation DAGs, assumptions,
    /// support, and payload; independent of build profile (debug vs release) and host architecture.
    #[must_use]
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut enc = CanonicalEncoder::new();
        enc.tag(b"frob.proof.object/1");
        self.id.encode_canonical(&mut enc);
        enc.str(self.kind.as_str());
        self.cell.encode_canonical(&mut enc);
        self.establishes.encode_canonical(&mut enc);
        self.payload.encode_canonical(&mut enc);
        enc.usize(self.side_conditions.len());
        for sc in &self.side_conditions {
            sc.encode_canonical(&mut enc);
        }
        enc.usize(self.primitive_equations.len());
        for eq in &self.primitive_equations {
            eq.encode_canonical(&mut enc);
        }
        enc.usize(self.assumptions.len());
        for a in &self.assumptions {
            a.encode_canonical(&mut enc);
        }
        self.support.encode_canonical(&mut enc);
        self.expects.encode_canonical(&mut enc);
        enc.finish()
    }

    /// Domain-separated BLAKE3 digest of [`Self::canonical_bytes`] (`frob.proof.object/1`).
    #[must_use]
    pub fn canonical_digest(&self) -> Blake3 {
        let bytes = self.canonical_bytes();
        Blake3::from_bytes(*fs_blake3::hash_domain("frob.proof.object/1", &bytes).as_bytes())
    }
}
