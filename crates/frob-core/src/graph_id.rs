//! Artifact-graph identifiers in the FrankenSuite text form (`N34-11`, §34.12, §29.13).
//!
//! Provides the four artifact-graph identifier types plus the text-form mirror of the
//! sibling's [`DecisionId`], keeping `frob-core` (layer 0) completely free of `serde`
//! and sibling runtime crates (`OPEN-CHRON-3` fail-closed default):
//!
//! - [`ClaimId`]: content-addressed claim identity (`clm:<64 lower-case hex>`) over
//!   canonical JSON under schema `frob.claim/1`.
//! - [`EvidenceId`]: content-addressed evidence identity (`ev:<64 lower-case hex>`) over
//!   canonical JSON under schema `frob.evidence/1`.
//! - [`DecisionPolicyId`]: versioned decision-rule identifier (`<dotted-name>@v<u32>`),
//!   serialized as its text form string in every JSON view.
//! - [`TraceId`]: 128-bit execution trace identifier (high 48 bits Unix ms timestamp
//!   saturated at `2^48 - 1`, low 80 bits random), printed as 32 lower-case hex digits.
//! - [`DecisionId`]: 128-bit execution decision-instance mirror printed as 32 lower-case
//!   hex digits (not an artifact-graph node; never enters any content hash).
//!
//! Determinism: D0 for [`ClaimId`], [`EvidenceId`], [`DecisionPolicyId`], [`DottedName`],
//! and [`EvidenceKind`]. [`TraceId`] and [`DecisionId`] identify executions, never implement
//! [`Canonical`], and cannot be passed to [`crate::id::identity`] or
//! [`crate::canon::canonical_identity`].
//!
//! Prints nothing (rule L-7); admits no `serde` (rule L-5); zero `unsafe`.

use std::fmt;
use std::str::FromStr;

use crate::canon::{CanonError, Canonical, Value, canonical_identity, to_canonical};
use crate::evidence::{ClaimKind, Quantifier, Scope, ScopeDomain};
use crate::id::{Blake3, CanonicalBytes, SchemaName};

/// Schema domain string for [`ClaimId`] canonical pre-images.
pub const CLAIM_SCHEMA: &str = "frob.claim/1";

/// Schema domain string for [`EvidenceId`] canonical pre-images.
pub const EVIDENCE_SCHEMA: &str = "frob.evidence/1";

/// Error returned while constructing or parsing an artifact-graph identifier.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GraphIdError {
    InvalidPrefix {
        expected: &'static str,
        input: String,
    },
    InvalidHex {
        kind: &'static str,
        expected_len: usize,
        input: String,
    },
    InvalidEvidenceKind {
        input: String,
    },
    InvalidClaimType {
        input: String,
    },
    InvalidDottedName {
        input: String,
        reason: &'static str,
    },
    InvalidDecisionPolicy {
        input: String,
        reason: &'static str,
    },
    InvalidClaimInput {
        reason: &'static str,
    },
    Canon(CanonError),
}

impl From<CanonError> for GraphIdError {
    fn from(err: CanonError) -> Self {
        Self::Canon(err)
    }
}

impl fmt::Display for GraphIdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidPrefix { expected, input } => {
                write!(
                    f,
                    "invalid identifier prefix in `{input}`: expected `{expected}`"
                )
            }
            Self::InvalidHex {
                kind,
                expected_len,
                input,
            } => write!(
                f,
                "invalid {kind} hex `{input}`: expected {expected_len} lower-case hex digits"
            ),
            Self::InvalidEvidenceKind { input } => write!(
                f,
                "unknown evidence kind `{input}` (expected one of: episode-set, chunk-range, snapshot, certificate, receipt, ledger-row, analysis-result)"
            ),
            Self::InvalidClaimType { input } => write!(
                f,
                "unknown claim type `{input}` (expected one of: proof, known-answer, parity, fidelity, design-comparison, reality, performance)"
            ),
            Self::InvalidDottedName { input, reason } => {
                write!(f, "invalid dotted name `{input}`: {reason}")
            }
            Self::InvalidDecisionPolicy { input, reason } => {
                write!(f, "invalid decision policy identifier `{input}`: {reason}")
            }
            Self::InvalidClaimInput { reason } => write!(f, "invalid claim pre-image: {reason}"),
            Self::Canon(err) => fmt::Display::fmt(err, f),
        }
    }
}

impl std::error::Error for GraphIdError {}

/// Parse one of the seven canonical lower-case hyphenated claim kinds (§8, N34-11).
pub fn parse_claim_kind(raw: &str) -> Result<ClaimKind, GraphIdError> {
    match raw {
        "proof" => Ok(ClaimKind::Proof),
        "known-answer" => Ok(ClaimKind::KnownAnswer),
        "parity" => Ok(ClaimKind::Parity),
        "fidelity" => Ok(ClaimKind::Fidelity),
        "design-comparison" => Ok(ClaimKind::DesignComparison),
        "reality" => Ok(ClaimKind::Reality),
        "performance" => Ok(ClaimKind::Performance),
        _ => Err(GraphIdError::InvalidClaimType {
            input: raw.to_owned(),
        }),
    }
}

fn decode_lower_hex_nibble(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        _ => None,
    }
}

fn parse_u128_lower_hex(kind: &'static str, s: &str) -> Result<u128, GraphIdError> {
    let raw = s.as_bytes();
    if raw.len() != 32 {
        return Err(GraphIdError::InvalidHex {
            kind,
            expected_len: 32,
            input: s.to_owned(),
        });
    }
    let mut acc: u128 = 0;
    for &b in raw {
        let nib = decode_lower_hex_nibble(b).ok_or_else(|| GraphIdError::InvalidHex {
            kind,
            expected_len: 32,
            input: s.to_owned(),
        })?;
        acc = (acc << 4) | u128::from(nib);
    }
    Ok(acc)
}

fn write_scope_domain(domain: &ScopeDomain, w: Value<'_>) -> Result<(), CanonError> {
    w.object(|o| match domain {
        ScopeDomain::Unrestricted => {
            o.str("kind", "unrestricted")?;
            Ok(())
        }
        ScopeDomain::Empty => {
            o.str("kind", "empty")?;
            Ok(())
        }
        ScopeDomain::Named(name) => {
            o.str("kind", "named")?;
            o.str("name", name)?;
            Ok(())
        }
    })
}

fn write_scope(scope: &Scope, w: Value<'_>) -> Result<(), CanonError> {
    w.object(|o| {
        o.member("design_family", |v| {
            write_scope_domain(&scope.design_family, v)
        })?;
        o.member("horizon_steps", |v| match scope.horizon_steps {
            None => {
                v.null();
                Ok(())
            }
            Some(steps) => {
                v.u64str(steps);
                Ok(())
            }
        })?;
        o.member("quantifiers", |v| {
            v.array(|a| {
                for binder in &scope.quantifiers.0 {
                    let q_tok = match binder.quantifier {
                        Quantifier::ForAll => "forall",
                        Quantifier::Exists => "exists",
                        Quantifier::Prob => "prob",
                    };
                    a.push(|bv| {
                        bv.object(|bo| {
                            bo.str("quantifier", q_tok)?;
                            bo.str("variable", &binder.variable)?;
                            Ok(())
                        })
                    })?;
                }
                Ok(())
            })
        })?;
        o.member("validity_region", |v| {
            write_scope_domain(&scope.validity_region, v)
        })?;
        o.member("world_family", |v| {
            write_scope_domain(&scope.world_family, v)
        })?;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// 1. ClaimId (`clm:<64 lower-case hex>`)
// ---------------------------------------------------------------------------

/// Canonical pre-image of a [`ClaimId`], hashed under schema `frob.claim/1`.
///
/// Canonical JSON layout:
/// `{"scope": <scope>, "statement": "<text>", "subjects": ["<identity>", ...], "type": "<claim-kind>"}`
/// where `subjects` is sorted in Unicode code-point order so subject permutation leaves
/// the canonical bytes and [`ClaimId`] unchanged.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClaimPreimage {
    claim_type: ClaimKind,
    subjects: Vec<String>,
    statement: String,
    scope: Scope,
}

impl ClaimPreimage {
    /// Construct a validated claim pre-image with `subjects` sorted in Unicode code-point order.
    pub fn new(
        claim_type: ClaimKind,
        subjects: impl IntoIterator<Item = impl Into<String>>,
        statement: impl Into<String>,
        scope: Scope,
    ) -> Result<Self, GraphIdError> {
        let statement = statement.into();
        if statement.is_empty() {
            return Err(GraphIdError::InvalidClaimInput {
                reason: "statement must not be empty",
            });
        }
        let mut sorted_subjects: Vec<String> = subjects.into_iter().map(Into::into).collect();
        if sorted_subjects.iter().any(|s| s.is_empty()) {
            return Err(GraphIdError::InvalidClaimInput {
                reason: "subject identities must not be empty",
            });
        }
        sorted_subjects.sort();
        Ok(Self {
            claim_type,
            subjects: sorted_subjects,
            statement,
            scope,
        })
    }

    #[must_use]
    pub const fn claim_type(&self) -> ClaimKind {
        self.claim_type
    }

    #[must_use]
    pub fn subjects(&self) -> &[String] {
        &self.subjects
    }

    #[must_use]
    pub fn statement(&self) -> &str {
        &self.statement
    }

    #[must_use]
    pub const fn scope(&self) -> &Scope {
        &self.scope
    }

    /// Encode this claim pre-image into deterministic canonical JSON bytes.
    pub fn to_canonical_bytes(&self) -> Result<CanonicalBytes, GraphIdError> {
        Ok(to_canonical(self)?)
    }

    /// Compute the [`ClaimId`] under schema `frob.claim/1`.
    pub fn claim_id(&self) -> Result<ClaimId, GraphIdError> {
        ClaimId::from_preimage(self)
    }
}

impl Canonical for ClaimPreimage {
    fn write(&self, w: Value<'_>) -> Result<(), CanonError> {
        w.object(|o| {
            o.member("scope", |v| write_scope(&self.scope, v))?;
            o.str("statement", &self.statement)?;
            o.member("subjects", |v| {
                v.array(|a| {
                    for subj in &self.subjects {
                        a.push_str(subj);
                    }
                    Ok(())
                })
            })?;
            o.str("type", self.claim_type.as_str())?;
            Ok(())
        })
    }
}

/// Content-addressed artifact-graph claim identifier (`clm:<64 lower-case hex>`).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ClaimId(Blake3);

impl ClaimId {
    /// Required wire prefix.
    pub const PREFIX: &'static str = "clm:";

    /// Compute a [`ClaimId`] directly from its four semantic parts.
    pub fn compute(
        claim_type: ClaimKind,
        subjects: impl IntoIterator<Item = impl Into<String>>,
        statement: impl Into<String>,
        scope: Scope,
    ) -> Result<Self, GraphIdError> {
        let pre = ClaimPreimage::new(claim_type, subjects, statement, scope)?;
        Self::from_preimage(&pre)
    }

    /// Compute a [`ClaimId`] from a validated [`ClaimPreimage`] under `frob.claim/1`.
    pub fn from_preimage(preimage: &ClaimPreimage) -> Result<Self, GraphIdError> {
        let schema = SchemaName::new("claim", 1).expect("static claim schema is valid");
        let digest = canonical_identity(&schema, preimage)?;
        Ok(Self(digest))
    }

    /// Wrap a precomputed BLAKE3 digest as a [`ClaimId`].
    #[must_use]
    pub const fn from_digest(digest: Blake3) -> Self {
        Self(digest)
    }

    /// Return the underlying 32-byte BLAKE3 digest.
    #[must_use]
    pub const fn digest(self) -> Blake3 {
        self.0
    }

    /// Parse the single canonical text form `clm:<64 lower-case hex>`.
    pub fn parse(s: &str) -> Result<Self, GraphIdError> {
        let hex = s
            .strip_prefix(Self::PREFIX)
            .ok_or_else(|| GraphIdError::InvalidPrefix {
                expected: Self::PREFIX,
                input: s.to_owned(),
            })?;
        let digest = Blake3::from_hex(hex).map_err(|_| GraphIdError::InvalidHex {
            kind: "ClaimId",
            expected_len: 64,
            input: s.to_owned(),
        })?;
        Ok(Self(digest))
    }
}

impl fmt::Display for ClaimId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", Self::PREFIX, self.0)
    }
}

impl fmt::Debug for ClaimId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ClaimId({}{})", Self::PREFIX, self.0)
    }
}

impl FromStr for ClaimId {
    type Err = GraphIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl Canonical for ClaimId {
    fn write(&self, w: Value<'_>) -> Result<(), CanonError> {
        w.str(&self.to_string());
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// 2. EvidenceKind & EvidenceId (`ev:<64 lower-case hex>`)
// ---------------------------------------------------------------------------

/// Closed enumeration of artifact-graph evidence kinds (`N34-11`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EvidenceKind {
    EpisodeSet,
    ChunkRange,
    Snapshot,
    Certificate,
    Receipt,
    LedgerRow,
    AnalysisResult,
}

impl EvidenceKind {
    /// All seven admitted evidence kinds.
    pub const ALL: &'static [Self] = &[
        Self::EpisodeSet,
        Self::ChunkRange,
        Self::Snapshot,
        Self::Certificate,
        Self::Receipt,
        Self::LedgerRow,
        Self::AnalysisResult,
    ];

    /// Canonical lower-case hyphenated token.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EpisodeSet => "episode-set",
            Self::ChunkRange => "chunk-range",
            Self::Snapshot => "snapshot",
            Self::Certificate => "certificate",
            Self::Receipt => "receipt",
            Self::LedgerRow => "ledger-row",
            Self::AnalysisResult => "analysis-result",
        }
    }

    /// Parse one of the seven closed evidence kinds.
    pub fn parse(s: &str) -> Result<Self, GraphIdError> {
        match s {
            "episode-set" => Ok(Self::EpisodeSet),
            "chunk-range" => Ok(Self::ChunkRange),
            "snapshot" => Ok(Self::Snapshot),
            "certificate" => Ok(Self::Certificate),
            "receipt" => Ok(Self::Receipt),
            "ledger-row" => Ok(Self::LedgerRow),
            "analysis-result" => Ok(Self::AnalysisResult),
            _ => Err(GraphIdError::InvalidEvidenceKind {
                input: s.to_owned(),
            }),
        }
    }
}

impl fmt::Display for EvidenceKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for EvidenceKind {
    type Err = GraphIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl Canonical for EvidenceKind {
    fn write(&self, w: Value<'_>) -> Result<(), CanonError> {
        w.str(self.as_str());
        Ok(())
    }
}

/// Canonical pre-image of an [`EvidenceId`], hashed under schema `frob.evidence/1`.
///
/// Canonical JSON layout: `{"kind":"<kind>","payload":"<64-hex-hash>"}`.
/// Contains no trace identifier and no wall-clock timestamp so two executions of the
/// same work produce one identical [`EvidenceId`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EvidencePreimage {
    kind: EvidenceKind,
    payload: Blake3,
}

impl EvidencePreimage {
    #[must_use]
    pub const fn new(kind: EvidenceKind, payload: Blake3) -> Self {
        Self { kind, payload }
    }

    #[must_use]
    pub const fn kind(self) -> EvidenceKind {
        self.kind
    }

    #[must_use]
    pub const fn payload(self) -> Blake3 {
        self.payload
    }

    /// Encode this evidence pre-image into deterministic canonical JSON bytes.
    pub fn to_canonical_bytes(self) -> Result<CanonicalBytes, GraphIdError> {
        Ok(to_canonical(&self)?)
    }

    /// Compute the [`EvidenceId`] under schema `frob.evidence/1`.
    #[must_use]
    pub fn evidence_id(self) -> EvidenceId {
        EvidenceId::from_preimage(self)
    }
}

impl Canonical for EvidencePreimage {
    fn write(&self, w: Value<'_>) -> Result<(), CanonError> {
        let hex = self.payload.to_hex();
        w.object(|o| {
            o.str("kind", self.kind.as_str())?;
            o.str("payload", &hex)?;
            Ok(())
        })
    }
}

/// Content-addressed artifact-graph evidence identifier (`ev:<64 lower-case hex>`).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EvidenceId(Blake3);

impl EvidenceId {
    /// Required wire prefix.
    pub const PREFIX: &'static str = "ev:";

    /// Compute an [`EvidenceId`] from a closed [`EvidenceKind`] and payload digest.
    #[must_use]
    pub fn compute(kind: EvidenceKind, payload: Blake3) -> Self {
        Self::from_preimage(EvidencePreimage::new(kind, payload))
    }

    /// Parse `kind` against the closed seven-element set and compute the [`EvidenceId`].
    pub fn from_kind_str(kind: &str, payload: Blake3) -> Result<Self, GraphIdError> {
        let parsed = EvidenceKind::parse(kind)?;
        Ok(Self::compute(parsed, payload))
    }

    /// Compute an [`EvidenceId`] from its canonical pre-image under `frob.evidence/1`.
    #[must_use]
    pub fn from_preimage(preimage: EvidencePreimage) -> Self {
        let schema = SchemaName::new("evidence", 1).expect("static evidence schema is valid");
        let digest = canonical_identity(&schema, &preimage)
            .expect("canonicalizing EvidencePreimage is infallible");
        Self(digest)
    }

    /// Wrap a precomputed BLAKE3 digest as an [`EvidenceId`].
    #[must_use]
    pub const fn from_digest(digest: Blake3) -> Self {
        Self(digest)
    }

    /// Return the underlying 32-byte BLAKE3 digest.
    #[must_use]
    pub const fn digest(self) -> Blake3 {
        self.0
    }

    /// Parse the single canonical text form `ev:<64 lower-case hex>`.
    pub fn parse(s: &str) -> Result<Self, GraphIdError> {
        let hex = s
            .strip_prefix(Self::PREFIX)
            .ok_or_else(|| GraphIdError::InvalidPrefix {
                expected: Self::PREFIX,
                input: s.to_owned(),
            })?;
        let digest = Blake3::from_hex(hex).map_err(|_| GraphIdError::InvalidHex {
            kind: "EvidenceId",
            expected_len: 64,
            input: s.to_owned(),
        })?;
        Ok(Self(digest))
    }
}

impl fmt::Display for EvidenceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", Self::PREFIX, self.0)
    }
}

impl fmt::Debug for EvidenceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "EvidenceId({}{})", Self::PREFIX, self.0)
    }
}

impl FromStr for EvidenceId {
    type Err = GraphIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl Canonical for EvidenceId {
    fn write(&self, w: Value<'_>) -> Result<(), CanonError> {
        w.str(&self.to_string());
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// 3. DottedName & DecisionPolicyId (`name@vN`)
// ---------------------------------------------------------------------------

/// Validated dotted lower-case ASCII name (one or more non-empty segments separated by `.`).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DottedName(String);

impl DottedName {
    /// Validate and construct a [`DottedName`].
    ///
    /// Rules (`N34-11` step 5): one or more lower-case ASCII segments separated by `.`;
    /// empty segments, upper-case ASCII letters, white space, and invalid punctuation
    /// are rejected.
    pub fn parse(raw: &str) -> Result<Self, GraphIdError> {
        if raw.is_empty() {
            return Err(GraphIdError::InvalidDottedName {
                input: raw.to_owned(),
                reason: "dotted name must not be empty",
            });
        }
        for segment in raw.split('.') {
            if segment.is_empty() {
                return Err(GraphIdError::InvalidDottedName {
                    input: raw.to_owned(),
                    reason: "empty segment in dotted name",
                });
            }
            for &b in segment.as_bytes() {
                if b.is_ascii_uppercase() {
                    return Err(GraphIdError::InvalidDottedName {
                        input: raw.to_owned(),
                        reason: "upper-case letters are forbidden in dotted names",
                    });
                }
                if b.is_ascii_whitespace() {
                    return Err(GraphIdError::InvalidDottedName {
                        input: raw.to_owned(),
                        reason: "white space is forbidden in dotted names",
                    });
                }
                if !(b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-' || b == b'_') {
                    return Err(GraphIdError::InvalidDottedName {
                        input: raw.to_owned(),
                        reason: "only lower-case ASCII letters, digits, `-`, and `_` are permitted in segments",
                    });
                }
            }
            if !segment
                .as_bytes()
                .iter()
                .any(|b| b.is_ascii_lowercase() || b.is_ascii_digit())
            {
                return Err(GraphIdError::InvalidDottedName {
                    input: raw.to_owned(),
                    reason: "segment must contain at least one lower-case letter or digit",
                });
            }
        }
        Ok(Self(raw.to_owned()))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for DottedName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl FromStr for DottedName {
    type Err = GraphIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl AsRef<str> for DottedName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// Versioned decision-rule identifier (`name@vN`, e.g. `atelier.dc01.promote@v3`).
///
/// Mirrors the text form of `franken_kernel::PolicyId` under the name `DecisionPolicyId`
/// (distinct from FrankenRobotics's content-addressed control-policy `id::PolicyId`).
/// In every JSON view, [`DecisionPolicyId`] is serialized as its single text string
/// `"name@vN"`, never as an object.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DecisionPolicyId {
    name: DottedName,
    version: u32,
}

impl DecisionPolicyId {
    /// Construct a [`DecisionPolicyId`] from a dotted name string and version number.
    pub fn new(name: impl AsRef<str>, version: u32) -> Result<Self, GraphIdError> {
        let dotted = DottedName::parse(name.as_ref())?;
        Ok(Self {
            name: dotted,
            version,
        })
    }

    /// Construct a [`DecisionPolicyId`] from an already validated [`DottedName`].
    #[must_use]
    pub const fn from_dotted(name: DottedName, version: u32) -> Self {
        Self { name, version }
    }

    /// Return the dotted rule name as `&str`.
    #[must_use]
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Return the validated [`DottedName`].
    #[must_use]
    pub const fn dotted_name(&self) -> &DottedName {
        &self.name
    }

    /// Return the policy version number.
    #[must_use]
    pub const fn version(&self) -> u32 {
        self.version
    }

    /// Parse the single canonical text form `<dotted-name>@v<u32>`.
    pub fn parse(s: &str) -> Result<Self, GraphIdError> {
        let Some((name_part, ver_part)) = s.split_once('@') else {
            return Err(GraphIdError::InvalidDecisionPolicy {
                input: s.to_owned(),
                reason: "missing `@v<version>` suffix",
            });
        };
        let Some(digits) = ver_part.strip_prefix('v') else {
            return Err(GraphIdError::InvalidDecisionPolicy {
                input: s.to_owned(),
                reason: "version suffix must begin with `@v`",
            });
        };
        if digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()) {
            return Err(GraphIdError::InvalidDecisionPolicy {
                input: s.to_owned(),
                reason: "version after `@v` must be a non-empty ASCII decimal integer",
            });
        }
        if digits.len() > 1 && digits.starts_with('0') {
            return Err(GraphIdError::InvalidDecisionPolicy {
                input: s.to_owned(),
                reason: "leading zeros are forbidden in policy versions",
            });
        }
        let version: u32 = digits
            .parse()
            .map_err(|_| GraphIdError::InvalidDecisionPolicy {
                input: s.to_owned(),
                reason: "policy version exceeds u32::MAX",
            })?;
        let name = DottedName::parse(name_part)?;
        Ok(Self { name, version })
    }
}

impl fmt::Display for DecisionPolicyId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}@v{}", self.name, self.version)
    }
}

impl FromStr for DecisionPolicyId {
    type Err = GraphIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl Canonical for DecisionPolicyId {
    fn write(&self, w: Value<'_>) -> Result<(), CanonError> {
        w.str(&self.to_string());
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// 4. TraceId (128-bit execution identifier; NEVER enters any content hash)
// ---------------------------------------------------------------------------

/// 128-bit time-ordered execution trace identifier (`32 lower-case hex digits`).
///
/// Layout (mirroring `franken_kernel::TraceId`):
/// - High 48 bits: millisecond Unix timestamp, saturated at `2^48 - 1`.
/// - Low 80 bits: random bits from the job context.
///
/// A `TraceId` identifies an execution, not content. It intentionally does **not**
/// implement [`Canonical`] and cannot be passed to [`crate::id::identity`] or
/// [`crate::canon::canonical_identity`]:
///
/// ```compile_fail,E0277
/// use frob_core::canon::to_canonical;
/// use frob_core::graph_id::TraceId;
///
/// let trace = TraceId::from_parts(1_700_000_000_000, 0xABCD);
/// let _ = to_canonical(&trace);
/// ```
///
/// ```compile_fail,E0277
/// use frob_core::canon::canonical_identity;
/// use frob_core::graph_id::TraceId;
/// use frob_core::id::SchemaName;
///
/// let schema = SchemaName::new("evidence", 1).unwrap();
/// let trace = TraceId::from_parts(1_700_000_000_000, 0xABCD);
/// let _ = canonical_identity(&schema, &trace);
/// ```
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TraceId(u128);

impl TraceId {
    /// Maximum representable 48-bit millisecond timestamp (`2^48 - 1`).
    pub const MAX_TIMESTAMP_MS: u64 = (1u64 << 48) - 1;

    /// Bit mask for the low 80-bit random field.
    pub const RANDOM_MASK: u128 = 0xFFFF_FFFF_FFFF_FFFF_FFFF;

    /// Construct from a raw 128-bit integer.
    #[must_use]
    pub const fn from_raw(raw: u128) -> Self {
        Self(raw)
    }

    /// Construct from a millisecond timestamp (high 48 bits, saturated at `2^48 - 1`)
    /// and 80 random bits (masked to the low 80 bits).
    #[must_use]
    pub const fn from_parts(ts_ms: u64, random: u128) -> Self {
        let saturated_ts = if ts_ms > Self::MAX_TIMESTAMP_MS {
            Self::MAX_TIMESTAMP_MS
        } else {
            ts_ms
        };
        let ts_bits = (saturated_ts as u128) << 80;
        let rand_bits = random & Self::RANDOM_MASK;
        Self(ts_bits | rand_bits)
    }

    /// Strict constructor returning `Err(ts_ms)` if `ts_ms >= 2^48`.
    pub const fn try_from_parts(ts_ms: u64, random: u128) -> Result<Self, u64> {
        if ts_ms > Self::MAX_TIMESTAMP_MS {
            return Err(ts_ms);
        }
        let ts_bits = (ts_ms as u128) << 80;
        let rand_bits = random & Self::RANDOM_MASK;
        Ok(Self(ts_bits | rand_bits))
    }

    /// Extract the 48-bit millisecond timestamp from the high bits.
    #[must_use]
    pub const fn timestamp_ms(self) -> u64 {
        (self.0 >> 80) as u64
    }

    /// Extract the low 80 random bits.
    #[must_use]
    pub const fn random_bits(self) -> u128 {
        self.0 & Self::RANDOM_MASK
    }

    /// Return the raw 128-bit value.
    #[must_use]
    pub const fn as_u128(self) -> u128 {
        self.0
    }

    /// Return big-endian bytes.
    #[must_use]
    pub const fn to_bytes(self) -> [u8; 16] {
        self.0.to_be_bytes()
    }

    /// Construct from big-endian bytes.
    #[must_use]
    pub const fn from_bytes(bytes: [u8; 16]) -> Self {
        Self(u128::from_be_bytes(bytes))
    }

    /// Parse the single canonical text form: 32 lower-case hexadecimal digits.
    pub fn parse(s: &str) -> Result<Self, GraphIdError> {
        Ok(Self(parse_u128_lower_hex("TraceId", s)?))
    }
}

impl fmt::Debug for TraceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TraceId({:032x})", self.0)
    }
}

impl fmt::Display for TraceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:032x}", self.0)
    }
}

impl FromStr for TraceId {
    type Err = GraphIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

// ---------------------------------------------------------------------------
// 5. DecisionId (128-bit text-form mirror; NEVER enters any content hash)
// ---------------------------------------------------------------------------

/// 128-bit decision-instance identifier mirroring the text form of
/// `franken_kernel::DecisionId` (`32 lower-case hex digits`).
///
/// Like [`TraceId`], `DecisionId` identifies one execution decision instance rather than
/// content, is not a node of the artifact graph, intentionally does **not** implement
/// [`Canonical`], and cannot be passed to [`crate::id::identity`] or
/// [`crate::canon::canonical_identity`]:
///
/// ```compile_fail,E0277
/// use frob_core::canon::to_canonical;
/// use frob_core::graph_id::DecisionId;
///
/// let decision = DecisionId::from_raw(0x0123_4567_89ab_cdef_0123_4567_89ab_cdef);
/// let _ = to_canonical(&decision);
/// ```
///
/// ```compile_fail,E0277
/// use frob_core::canon::canonical_identity;
/// use frob_core::graph_id::DecisionId;
/// use frob_core::id::SchemaName;
///
/// let schema = SchemaName::new("claim", 1).unwrap();
/// let decision = DecisionId::from_raw(0x0123_4567_89ab_cdef_0123_4567_89ab_cdef);
/// let _ = canonical_identity(&schema, &decision);
/// ```
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DecisionId(u128);

impl DecisionId {
    /// Construct from a raw 128-bit integer.
    #[must_use]
    pub const fn from_raw(raw: u128) -> Self {
        Self(raw)
    }

    /// Return the raw 128-bit value.
    #[must_use]
    pub const fn as_u128(self) -> u128 {
        self.0
    }

    /// Return big-endian bytes.
    #[must_use]
    pub const fn to_bytes(self) -> [u8; 16] {
        self.0.to_be_bytes()
    }

    /// Construct from big-endian bytes.
    #[must_use]
    pub const fn from_bytes(bytes: [u8; 16]) -> Self {
        Self(u128::from_be_bytes(bytes))
    }

    /// Parse the single canonical text form: 32 lower-case hexadecimal digits.
    ///
    /// Shares the sibling's 32-hex text form with [`TraceId`]; callers distinguish the two
    /// by the field holding them rather than by their text spelling (`N34-11` step 5).
    pub fn parse(s: &str) -> Result<Self, GraphIdError> {
        Ok(Self(parse_u128_lower_hex("DecisionId", s)?))
    }
}

impl fmt::Debug for DecisionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DecisionId({:032x})", self.0)
    }
}

impl fmt::Display for DecisionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:032x}", self.0)
    }
}

impl FromStr for DecisionId {
    type Err = GraphIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::evidence::{QuantifierBinder, QuantifierPrefix};
    use crate::id::{DesignId, ModelId, hash_blob};

    fn sample_scope(quantifiers: Vec<QuantifierBinder>, horizon_steps: Option<u64>) -> Scope {
        Scope {
            design_family: ScopeDomain::Named("quadruped-v1".to_owned()),
            world_family: ScopeDomain::Named("flat-concrete".to_owned()),
            quantifiers: QuantifierPrefix(quantifiers),
            horizon_steps,
            validity_region: ScopeDomain::Named("dry-nominal".to_owned()),
        }
    }

    #[test]
    fn claim_id_ignores_subject_order() {
        let d = DesignId::from_digest(hash_blob(b"design-a")).to_string();
        let m = ModelId::from_digest(hash_blob(b"model-a")).to_string();
        let s = "recovers KA-101 within 1e-6";
        let scope = sample_scope(vec![QuantifierBinder::for_all("w")], Some(1000));

        let c1 =
            ClaimId::compute(ClaimKind::KnownAnswer, [&d, &m], s, scope.clone()).expect("claim 1");
        let c2 = ClaimId::compute(ClaimKind::KnownAnswer, [&m, &d], s, scope).expect("claim 2");
        assert_eq!(c1, c2);
        assert_eq!(c1.to_string(), c2.to_string());
    }

    #[test]
    fn claim_id_changes_with_scope() {
        let subj = DesignId::from_digest(hash_blob(b"design-scope")).to_string();
        let stmt = "meets torque bound <= 40 N*m";

        let base = sample_scope(vec![QuantifierBinder::for_all("w")], Some(1000));
        let diff_design = Scope {
            design_family: ScopeDomain::Named("quadruped-v2".to_owned()),
            ..base.clone()
        };
        let diff_world = Scope {
            world_family: ScopeDomain::Named("wet-tile".to_owned()),
            ..base.clone()
        };
        let diff_horizon = Scope {
            horizon_steps: Some(2000),
            ..base.clone()
        };
        let diff_validity = Scope {
            validity_region: ScopeDomain::Unrestricted,
            ..base.clone()
        };

        let id_base = ClaimId::compute(ClaimKind::Fidelity, [&subj], stmt, base).unwrap();
        for (label, sc) in [
            ("design_family", diff_design),
            ("world_family", diff_world),
            ("horizon_steps", diff_horizon),
            ("validity_region", diff_validity),
        ] {
            let other = ClaimId::compute(ClaimKind::Fidelity, [&subj], stmt, sc).unwrap();
            assert_ne!(id_base, other, "ClaimId must change when `{label}` changes");
        }
    }

    #[test]
    fn claim_id_changes_with_quantifier_order() {
        let subj = DesignId::from_digest(hash_blob(b"design-quant")).to_string();
        let stmt = "stabilizes under bounded disturbance";

        let robust = sample_scope(
            vec![
                QuantifierBinder::for_all("w"),
                QuantifierBinder::exists("u"),
            ],
            Some(500),
        );
        let clairvoyant = sample_scope(
            vec![
                QuantifierBinder::exists("u"),
                QuantifierBinder::for_all("w"),
            ],
            Some(500),
        );

        let id_robust = ClaimId::compute(ClaimKind::Proof, [&subj], stmt, robust).unwrap();
        let id_clair = ClaimId::compute(ClaimKind::Proof, [&subj], stmt, clairvoyant).unwrap();
        assert_ne!(
            id_robust, id_clair,
            "claims differing only in quantifier order must produce distinct ClaimIds"
        );
    }

    #[test]
    fn claim_id_changes_with_claim_type() {
        let subj = DesignId::from_digest(hash_blob(b"design-type")).to_string();
        let stmt = "tracks reference trajectory within 0.01 m";
        let scope = sample_scope(vec![QuantifierBinder::for_all("w")], Some(1000));

        let parity =
            ClaimId::compute(ClaimKind::Parity, [&subj], stmt, scope.clone()).expect("parity");
        let reality =
            ClaimId::compute(ClaimKind::Reality, [&subj], stmt, scope.clone()).expect("reality");
        assert_ne!(
            parity, reality,
            "Parity and Reality claims about the same sentence must differ"
        );

        // Verify all seven claim kinds yield seven distinct ClaimIds.
        let all_kinds = [
            ClaimKind::Proof,
            ClaimKind::KnownAnswer,
            ClaimKind::Parity,
            ClaimKind::Fidelity,
            ClaimKind::DesignComparison,
            ClaimKind::Reality,
            ClaimKind::Performance,
        ];
        let mut seen = std::collections::BTreeSet::new();
        for kind in all_kinds {
            assert_eq!(parse_claim_kind(kind.as_str()).unwrap(), kind);
            let cid = ClaimId::compute(kind, [&subj], stmt, scope.clone()).unwrap();
            assert!(seen.insert(cid), "duplicate ClaimId for kind `{kind}`");
        }
        assert!(parse_claim_kind("method").is_err());
    }

    #[test]
    fn evidence_id_kinds_closed() {
        let payload = hash_blob(b"evidence-payload");
        let expected_names = [
            "episode-set",
            "chunk-range",
            "snapshot",
            "certificate",
            "receipt",
            "ledger-row",
            "analysis-result",
        ];
        assert_eq!(EvidenceKind::ALL.len(), expected_names.len());

        let mut ids = std::collections::BTreeSet::new();
        for (&kind, &name) in EvidenceKind::ALL.iter().zip(expected_names.iter()) {
            assert_eq!(kind.as_str(), name);
            assert_eq!(EvidenceKind::parse(name).unwrap(), kind);
            let eid1 = EvidenceId::compute(kind, payload);
            let eid2 = EvidenceId::from_kind_str(name, payload).unwrap();
            assert_eq!(eid1, eid2);
            assert!(
                ids.insert(eid1),
                "distinct kinds must yield distinct EvidenceIds"
            );
        }

        let err = EvidenceId::from_kind_str("log", payload)
            .expect_err("unknown evidence kind `log` must fail");
        assert_eq!(
            err,
            GraphIdError::InvalidEvidenceKind {
                input: "log".to_owned()
            }
        );
        for bad in ["", "LOG", "Receipt", "episode_set", "blob"] {
            assert!(
                EvidenceKind::parse(bad).is_err(),
                "expected invalid evidence kind for `{bad}`"
            );
        }
    }

    #[test]
    fn trace_id_layout() {
        let ts = 1_700_000_000_000u64;
        let rand = 0xABCD_EF01_2345_6789_ABu128;
        let id = TraceId::from_parts(ts, rand);
        assert_eq!(id.timestamp_ms(), ts);
        assert_eq!(id.random_bits(), rand);
        assert_eq!(id.as_u128(), (u128::from(ts) << 80) | rand);

        // Timestamp of 2^48 saturates to 2^48 - 1 and does not spill into random bits.
        let max_ts = (1u64 << 48) - 1;
        let over_exact = TraceId::from_parts(1u64 << 48, rand);
        assert_eq!(over_exact.timestamp_ms(), max_ts);
        assert_eq!(over_exact.random_bits(), rand);

        // Timestamp of 2^48 + 5 stores 2^48 - 1 and leaves random field intact.
        let over_five = TraceId::from_parts((1u64 << 48) + 5, rand);
        assert_eq!(over_five.timestamp_ms(), max_ts);
        assert_eq!(over_five.random_bits(), rand);
        assert_eq!(over_exact, over_five);

        // Upper bits above 80 in `random` are masked out and never corrupt timestamp.
        let dirty_rand = (0xDEAD_BEEFu128 << 80) | rand;
        let masked = TraceId::from_parts(ts, dirty_rand);
        assert_eq!(masked, id);

        // Strict variant refuses >= 2^48 and accepts <= 2^48 - 1.
        assert_eq!(TraceId::try_from_parts(1u64 << 48, rand), Err(1u64 << 48));
        assert_eq!(
            TraceId::try_from_parts(max_ts, rand),
            Ok(TraceId::from_parts(max_ts, rand))
        );
    }

    #[test]
    fn trace_id_text() {
        let samples = [
            TraceId::from_raw(0),
            TraceId::from_raw(u128::MAX),
            TraceId::from_parts(1_700_000_000_000, 0xABCD_EF01_2345_6789_AB),
            TraceId::from_parts(TraceId::MAX_TIMESTAMP_MS, 0),
        ];
        for id in samples {
            let text = id.to_string();
            assert_eq!(text.len(), 32);
            assert!(
                text.bytes()
                    .all(|b| b.is_ascii_digit() || matches!(b, b'a'..=b'f'))
            );
            assert_eq!(TraceId::parse(&text).unwrap(), id);
            assert_eq!(text.parse::<TraceId>().unwrap(), id);
            assert_eq!(TraceId::from_bytes(id.to_bytes()), id);
        }

        let valid = samples[2].to_string();
        assert!(TraceId::parse(&valid.to_ascii_uppercase()).is_err());
        assert!(TraceId::parse(&valid[..31]).is_err());
        assert!(TraceId::parse(&format!("{valid}0")).is_err());
        assert!(TraceId::parse(&format!("0x{valid}")).is_err());
        assert!(TraceId::parse(&format!("tr:{valid}")).is_err());
    }

    #[test]
    fn decision_id_text() {
        let id = DecisionId::from_raw(0x018b_c396_f400_00ab_cdef_0123_4567_89ab);
        let text = id.to_string();
        assert_eq!(text.len(), 32);
        assert!(
            text.bytes()
                .all(|b| b.is_ascii_digit() || matches!(b, b'a'..=b'f'))
        );
        assert_eq!(DecisionId::parse(&text).unwrap(), id);
        assert_eq!(text.parse::<DecisionId>().unwrap(), id);
        assert_eq!(DecisionId::from_bytes(id.to_bytes()), id);

        // Upper case and 31 digits refused.
        assert!(DecisionId::parse(&text.to_ascii_uppercase()).is_err());
        assert!(DecisionId::parse(&text[..31]).is_err());
        assert!(DecisionId::parse(&format!("{text}a")).is_err());

        // Stated exception (N34-11 step 5): DecisionId and TraceId share the sibling's
        // 32-hex text form, so the same text parses as a TraceId too.
        let as_trace = TraceId::parse(&text).expect("DecisionId text must parse as TraceId");
        assert_eq!(as_trace.as_u128(), id.as_u128());
        assert_eq!(as_trace.to_string(), text);
    }

    #[test]
    fn policy_text() {
        let parsed = DecisionPolicyId::parse("scheduler.preempt@v3").expect("parse policy");
        assert_eq!(parsed.name(), "scheduler.preempt");
        assert_eq!(parsed.version(), 3);
        assert_eq!(parsed.to_string(), "scheduler.preempt@v3");

        let atelier = DecisionPolicyId::new("atelier.dc01.promote", 3).expect("atelier policy");
        assert_eq!(atelier.to_string(), "atelier.dc01.promote@v3");
        assert_eq!(
            DecisionPolicyId::parse(&atelier.to_string()).unwrap(),
            atelier
        );

        let v0 = DecisionPolicyId::new("a.b", 0).expect("v0 allowed");
        assert_eq!(DecisionPolicyId::parse("a.b@v0").unwrap(), v0);

        for bad in [
            "a@3",
            "a@v",
            "@v1",
            "a@V3",
            "a@v03",
            "a@v-1",
            "a@v+1",
            "a@v1@v2",
            "a.b",
            "A.b@v1",
            "a..b@v1",
            "a b@v1",
            "a@v99999999999999999999",
        ] {
            assert!(
                DecisionPolicyId::parse(bad).is_err(),
                "expected policy parse error for `{bad}`"
            );
        }
    }

    #[test]
    fn policy_json_is_text_form() {
        let policy = DecisionPolicyId::new("atelier.dc01.promote", 3).expect("valid policy");
        let canon = to_canonical(&policy).expect("canonicalize policy");
        assert_eq!(canon.as_str(), "\"atelier.dc01.promote@v3\"");
    }

    #[test]
    fn dotted_name_grammar() {
        for good in [
            "a",
            "a.b",
            "atelier.dc01.promote",
            "scheduler.preempt",
            "a-1.b_2",
        ] {
            let d = DottedName::parse(good).expect("valid dotted name");
            assert_eq!(d.as_str(), good);
            assert_eq!(d.to_string(), good);
        }

        for bad in [
            "", ".", ".a", "a.", "a..b", "A.b", "a.B", "a b", " a.b", "a.b ", "a\tb", "a\nb",
            "a/b", "a:b", "a@b", "-.-",
        ] {
            assert!(
                DottedName::parse(bad).is_err(),
                "expected DottedName rejection for `{bad}`"
            );
        }
    }

    #[test]
    fn claim_and_evidence_text_forms() {
        let digest = hash_blob(b"text-form-oracle");
        let clm = ClaimId::from_digest(digest);
        let ev = EvidenceId::from_digest(digest);

        let clm_text = clm.to_string();
        let ev_text = ev.to_string();
        assert!(clm_text.starts_with("clm:"));
        assert_eq!(clm_text.len(), 4 + 64);
        assert!(ev_text.starts_with("ev:"));
        assert_eq!(ev_text.len(), 3 + 64);

        // Print-then-parse round-trip.
        assert_eq!(ClaimId::parse(&clm_text).unwrap(), clm);
        assert_eq!(EvidenceId::parse(&ev_text).unwrap(), ev);

        // Cross-kind rejection.
        assert!(EvidenceId::parse(&clm_text).is_err());
        assert!(ClaimId::parse(&ev_text).is_err());

        // Single-spelling rejections: CLM:..., c:..., EV:..., e:..., 63 digits, upper-case hex.
        let hex = digest.to_hex();
        let upper_hex = hex.to_ascii_uppercase();
        let short_hex = &hex[..63];
        let long_hex = format!("{hex}0");
        for bad_clm in [
            format!("CLM:{hex}"),
            format!("c:{hex}"),
            format!("clm:{upper_hex}"),
            format!("clm:{short_hex}"),
            format!("clm:{long_hex}"),
            hex.clone(),
        ] {
            assert!(
                ClaimId::parse(&bad_clm).is_err(),
                "ClaimId::parse must reject `{bad_clm}`"
            );
        }
        for bad_ev in [
            format!("EV:{hex}"),
            format!("e:{hex}"),
            format!("ev:{upper_hex}"),
            format!("ev:{short_hex}"),
            format!("ev:{long_hex}"),
            hex,
        ] {
            assert!(
                EvidenceId::parse(&bad_ev).is_err(),
                "EvidenceId::parse must reject `{bad_ev}`"
            );
        }

        // Canonical JSON emits each as its text-form string.
        assert_eq!(
            to_canonical(&clm).unwrap().as_str(),
            format!("\"{clm_text}\"")
        );
        assert_eq!(
            to_canonical(&ev).unwrap().as_str(),
            format!("\"{ev_text}\"")
        );
    }

    #[test]
    fn two_executions_same_evidence_id() {
        let payload_hash = hash_blob(b"deterministic-episode-set-bytes");
        let trace_run_1 = TraceId::from_parts(1_700_000_000_000, 0x1111_2222_3333_4444_5555);
        let trace_run_2 = TraceId::from_parts(1_700_000_060_000, 0xAAAA_BBBB_CCCC_DDDD_EEEE);
        assert_ne!(trace_run_1, trace_run_2);

        // Two executions produce different traces and one identical EvidenceId.
        let ev_run_1 = EvidenceId::compute(EvidenceKind::EpisodeSet, payload_hash);
        let ev_run_2 = EvidenceId::compute(EvidenceKind::EpisodeSet, payload_hash);
        assert_eq!(ev_run_1, ev_run_2);
        assert_eq!(ev_run_1.to_string(), ev_run_2.to_string());

        // Pre-image canonical bytes contain only `kind` and `payload` (no trace or timestamp).
        let canon = EvidencePreimage::new(EvidenceKind::EpisodeSet, payload_hash)
            .to_canonical_bytes()
            .unwrap();
        assert_eq!(
            canon.as_str(),
            format!(
                "{{\"kind\":\"episode-set\",\"payload\":\"{}\"}}",
                payload_hash.to_hex()
            )
        );
    }
}
