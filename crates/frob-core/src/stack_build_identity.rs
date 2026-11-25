//! FrankenSim build-identity mode (`dev-salt` vs `depgraph-receipt`),
//! compile-time [`BUILD_IDENTITY_MODE`], mode-resolution precedence rule, and
//! gate-citation guard [`may_cite`] (`N33a-08`, §33.2, §33.5, §33.8, §34.1).
//!
//! Determinism class D0; prints nothing (rule L-7); zero `unsafe`.

use std::fmt;

use crate::evidence::ClaimKind;

/// Environment variable naming the Mode 1 development equivalence class.
pub const ENV_SALT: &str = "FRANKENSIM_DEPGRAPH_SALT";

/// Environment variable naming the Mode 2 evidence-bearing dependency-graph receipt.
pub const ENV_RECEIPT: &str = "FRANKENSIM_DEPGRAPH_RECEIPT";

/// Compile-time mode string for Mode 1 (development salt).
pub const MODE_DEV_SALT: &str = "dev-salt";

/// Compile-time mode string for Mode 2 (evidence-bearing receipt).
pub const MODE_DEPGRAPH_RECEIPT: &str = "depgraph-receipt";

/// Refusal reason returned by [`may_cite`] when a Performance or Parity gate
/// attempts to cite a result produced under `dev-salt`.
pub const REASON_DEV_BUILD_IDENTITY: &str = "dev-build-identity";

/// Refusal reason returned when an unrecognized build-identity mode string is
/// supplied for a Performance or Parity gate.
pub const REASON_UNKNOWN_BUILD_IDENTITY: &str = "unknown-build-identity";

/// Precedence fact for `frankensim/crates/fs-la/build.rs` when both
/// `FRANKENSIM_DEPGRAPH_SALT` and `FRANKENSIM_DEPGRAPH_RECEIPT` are present
/// in the build environment (`N33a-08` Mode 2 step 2).
///
/// MUST remain `None` until both the receipt variable form and the two-variable
/// precedence are recorded as re-read sibling facts in the `INT-FSIM-01`
/// admission record (N33a-09). Change only in a commit citing that record.
pub const RECEIPT_WINS_WHEN_BOTH: Option<bool> = None;

/// Compile-time FrankenSim build-identity mode embedded by `crates/frob-core/build.rs`.
///
/// Values are `"dev-salt"` (Mode 1, default) and `"depgraph-receipt"` (Mode 2,
/// evidence-bearing). Rebuilt automatically when either `FRANKENSIM_DEPGRAPH_SALT`
/// or `FRANKENSIM_DEPGRAPH_RECEIPT` changes via `cargo:rerun-if-env-changed`.
pub const BUILD_IDENTITY_MODE: &str = env!("FROB_BUILD_IDENTITY_MODE");

/// Typed representation of [`BUILD_IDENTITY_MODE`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BuildIdentityMode {
    /// Mode 1: project-owned development equivalence-class salt from `.cargo/config.toml`.
    DevSalt,
    /// Mode 2: explicit evidence-bearing dependency-graph receipt exported at build time.
    DepgraphReceipt,
}

/// Compile-time invariant ensuring `crates/frob-core/build.rs` emitted a
/// canonical `FROB_BUILD_IDENTITY_MODE` (`"dev-salt"` or `"depgraph-receipt"`).
const _VALIDATE_BUILD_IDENTITY_MODE: [(); 1] =
    [(); (const_str_eq(BUILD_IDENTITY_MODE, MODE_DEV_SALT)
        || const_str_eq(BUILD_IDENTITY_MODE, MODE_DEPGRAPH_RECEIPT)) as usize];

impl BuildIdentityMode {
    /// Canonical kebab-case label matching [`BUILD_IDENTITY_MODE`].
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DevSalt => MODE_DEV_SALT,
            Self::DepgraphReceipt => MODE_DEPGRAPH_RECEIPT,
        }
    }

    /// Parse a canonical mode string in a `const` context.
    #[must_use]
    pub const fn parse_const(raw: &str) -> Option<Self> {
        if const_str_eq(raw, MODE_DEV_SALT) {
            Some(Self::DevSalt)
        } else if const_str_eq(raw, MODE_DEPGRAPH_RECEIPT) {
            Some(Self::DepgraphReceipt)
        } else {
            None
        }
    }

    /// Parse a canonical mode string (`"dev-salt"` or `"depgraph-receipt"`).
    #[must_use]
    pub fn parse(raw: &str) -> Option<Self> {
        Self::parse_const(raw)
    }

    /// Return the compile-time mode embedded in this binary.
    #[must_use]
    pub const fn current() -> Self {
        let _ = _VALIDATE_BUILD_IDENTITY_MODE;
        if const_str_eq(BUILD_IDENTITY_MODE, MODE_DEPGRAPH_RECEIPT) {
            Self::DepgraphReceipt
        } else {
            Self::DevSalt
        }
    }
}

impl fmt::Display for BuildIdentityMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Gate classification evaluated by [`may_cite`] (`N33a-08`).
///
/// Performance and Parity gates require an evidence-bearing build identity
/// (`depgraph-receipt`) and are refused with `"dev-build-identity"` under
/// `dev-salt`. Admission and other non-Performance/Parity gates are allowed
/// under `dev-salt`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GateClass {
    Admission,
    Proof,
    KnownAnswer,
    Parity,
    Fidelity,
    DesignComparison,
    Reality,
    Performance,
}

impl GateClass {
    /// Convert a [`ClaimKind`] into the corresponding [`GateClass`].
    #[must_use]
    pub const fn from_claim_kind(kind: ClaimKind) -> Self {
        match kind {
            ClaimKind::Proof => Self::Proof,
            ClaimKind::KnownAnswer => Self::KnownAnswer,
            ClaimKind::Parity => Self::Parity,
            ClaimKind::Fidelity => Self::Fidelity,
            ClaimKind::DesignComparison => Self::DesignComparison,
            ClaimKind::Reality => Self::Reality,
            ClaimKind::Performance => Self::Performance,
        }
    }

    /// Parse a canonical kebab-case gate or claim class name.
    #[must_use]
    pub fn parse(raw: &str) -> Option<Self> {
        match raw {
            "admission" => Some(Self::Admission),
            "proof" => Some(Self::Proof),
            "known-answer" => Some(Self::KnownAnswer),
            "parity" => Some(Self::Parity),
            "fidelity" => Some(Self::Fidelity),
            "design-comparison" => Some(Self::DesignComparison),
            "reality" => Some(Self::Reality),
            "performance" => Some(Self::Performance),
            _ => None,
        }
    }

    /// Canonical kebab-case name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Admission => "admission",
            Self::Proof => "proof",
            Self::KnownAnswer => "known-answer",
            Self::Parity => "parity",
            Self::Fidelity => "fidelity",
            Self::DesignComparison => "design-comparison",
            Self::Reality => "reality",
            Self::Performance => "performance",
        }
    }

    /// Returns `true` iff this gate class refuses `dev-salt` build identities.
    #[must_use]
    pub const fn requires_evidence_build_identity(self) -> bool {
        matches!(self, Self::Performance | Self::Parity)
    }
}

impl From<ClaimKind> for GateClass {
    fn from(kind: ClaimKind) -> Self {
        Self::from_claim_kind(kind)
    }
}

impl fmt::Display for GateClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Fail-closed refusal produced by [`resolve_mode`] when a build environment
/// does not uniquely select a valid build-identity mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolveModeError {
    /// Neither `FRANKENSIM_DEPGRAPH_SALT` nor `FRANKENSIM_DEPGRAPH_RECEIPT` is set.
    NeitherSet,
    /// One of the build-identity variables was present with an empty string value.
    EmptyValue { var: &'static str },
    /// Both variables are set while `receipt_wins_when_both` is `None` or `Some(false)`.
    AmbiguousBothSet {
        receipt_wins_when_both: Option<bool>,
    },
}

impl fmt::Display for ResolveModeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NeitherSet => write!(
                f,
                "neither {ENV_SALT} nor {ENV_RECEIPT} is set; failing closed"
            ),
            Self::EmptyValue { var } => {
                write!(f, "{var} is set to an empty string; failing closed")
            }
            Self::AmbiguousBothSet {
                receipt_wins_when_both,
            } => write!(
                f,
                "both {ENV_SALT} and {ENV_RECEIPT} are set with RECEIPT_WINS_WHEN_BOTH={receipt_wins_when_both:?}; refusing ambiguous build identity"
            ),
        }
    }
}

impl std::error::Error for ResolveModeError {}

/// Pure, fail-closed mode-resolution rule mirrored by `crates/frob-core/build.rs`
/// (`N33a-08` Mode 2 step 2).
///
/// - Salt only (non-empty) → [`BuildIdentityMode::DevSalt`]
/// - Receipt only (non-empty) → [`BuildIdentityMode::DepgraphReceipt`]
/// - Neither → `Err(ResolveModeError::NeitherSet)`
/// - Empty string in either variable → `Err(ResolveModeError::EmptyValue { .. })`
/// - Both set with `receipt_wins_when_both == Some(true)` → [`BuildIdentityMode::DepgraphReceipt`]
/// - Both set with `None` or `Some(false)` → `Err(ResolveModeError::AmbiguousBothSet { .. })`
pub const fn resolve_mode(
    salt: Option<&str>,
    receipt: Option<&str>,
    receipt_wins_when_both: Option<bool>,
) -> Result<BuildIdentityMode, ResolveModeError> {
    let has_salt = match salt {
        Some(v) => {
            if v.is_empty() {
                return Err(ResolveModeError::EmptyValue { var: ENV_SALT });
            }
            true
        }
        None => false,
    };
    let has_receipt = match receipt {
        Some(v) => {
            if v.is_empty() {
                return Err(ResolveModeError::EmptyValue { var: ENV_RECEIPT });
            }
            true
        }
        None => false,
    };

    match (has_salt, has_receipt) {
        (true, false) => Ok(BuildIdentityMode::DevSalt),
        (false, true) => Ok(BuildIdentityMode::DepgraphReceipt),
        (false, false) => Err(ResolveModeError::NeitherSet),
        (true, true) => match receipt_wins_when_both {
            Some(true) => Ok(BuildIdentityMode::DepgraphReceipt),
            Some(false) | None => Err(ResolveModeError::AmbiguousBothSet {
                receipt_wins_when_both,
            }),
        },
    }
}

/// Evaluate whether a result produced under `mode` may be cited in a gate row
/// of class `gate_class` (`N33a-08`).
///
/// Returns `Err("dev-build-identity")` for a Performance or Parity gate under
/// [`BuildIdentityMode::DevSalt`], and `Ok(())` otherwise.
pub const fn may_cite_under(
    mode: BuildIdentityMode,
    gate_class: GateClass,
) -> Result<(), &'static str> {
    match mode {
        BuildIdentityMode::DevSalt if gate_class.requires_evidence_build_identity() => {
            Err(REASON_DEV_BUILD_IDENTITY)
        }
        BuildIdentityMode::DevSalt | BuildIdentityMode::DepgraphReceipt => Ok(()),
    }
}

/// Evaluate whether a result produced by the current binary
/// ([`BUILD_IDENTITY_MODE`]) may be cited in a gate row of class `gate_class`
/// (`N33a-08`).
///
/// Returns `Err("dev-build-identity")` for a Performance or Parity gate under
/// `dev-salt`, and `Ok(())` for an Admission (or other non-Performance/Parity)
/// gate, or under `depgraph-receipt`.
pub const fn may_cite(gate_class: GateClass) -> Result<(), &'static str> {
    may_cite_under(BuildIdentityMode::current(), gate_class)
}

/// Evaluate [`may_cite_under`] from a recorded mode string (for example the
/// `build_identity` field of `frob.gauntlet.receipt/1`, consumed by `record_gate`
/// in `N38b-19`). Fails closed if `mode` is unrecognized for a gate that
/// requires an evidence build identity.
pub fn may_cite_mode_str(mode: &str, gate_class: GateClass) -> Result<(), &'static str> {
    match BuildIdentityMode::parse(mode) {
        Some(parsed) => may_cite_under(parsed, gate_class),
        None if gate_class.requires_evidence_build_identity() => Err(REASON_UNKNOWN_BUILD_IDENTITY),
        None => Ok(()),
    }
}

const fn const_str_eq(a: &str, b: &str) -> bool {
    let ab = a.as_bytes();
    let bb = b.as_bytes();
    if ab.len() != bb.len() {
        return false;
    }
    let mut i = 0;
    while i < ab.len() {
        if ab[i] != bb[i] {
            return false;
        }
        i += 1;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mode_constant_is_dev_salt_by_default() {
        assert_eq!(BUILD_IDENTITY_MODE, MODE_DEV_SALT);
        assert_eq!(crate::stack::BUILD_IDENTITY_MODE, MODE_DEV_SALT);
        assert_eq!(BuildIdentityMode::current(), BuildIdentityMode::DevSalt);
        assert_eq!(BuildIdentityMode::current().as_str(), "dev-salt");
        assert_eq!(
            RECEIPT_WINS_WHEN_BOTH, None,
            "RECEIPT_WINS_WHEN_BOTH must remain None until INT-FSIM-01 records re-read facts"
        );
    }

    #[test]
    fn performance_pass_refused_on_dev_identity() {
        assert_eq!(BUILD_IDENTITY_MODE, MODE_DEV_SALT);

        // Under default `dev-salt`, Performance and Parity gates are refused
        // with `dev-build-identity`, while Admission is accepted.
        assert_eq!(
            may_cite(GateClass::Performance),
            Err(REASON_DEV_BUILD_IDENTITY)
        );
        assert_eq!(may_cite(GateClass::Parity), Err(REASON_DEV_BUILD_IDENTITY));
        assert_eq!(may_cite(GateClass::Admission), Ok(()));
        assert_eq!(may_cite(GateClass::KnownAnswer), Ok(()));
        assert_eq!(may_cite(GateClass::Proof), Ok(()));

        // Under `depgraph-receipt`, Performance, Parity, and Admission are all accepted.
        assert_eq!(
            may_cite_under(BuildIdentityMode::DepgraphReceipt, GateClass::Performance),
            Ok(())
        );
        assert_eq!(
            may_cite_under(BuildIdentityMode::DepgraphReceipt, GateClass::Parity),
            Ok(())
        );
        assert_eq!(
            may_cite_under(BuildIdentityMode::DepgraphReceipt, GateClass::Admission),
            Ok(())
        );

        // String-based helper mirrors typed behavior and fails closed on unknown mode strings.
        assert_eq!(
            may_cite_mode_str("dev-salt", GateClass::Performance),
            Err(REASON_DEV_BUILD_IDENTITY)
        );
        assert_eq!(
            may_cite_mode_str("dev-salt", GateClass::Parity),
            Err(REASON_DEV_BUILD_IDENTITY)
        );
        assert_eq!(may_cite_mode_str("dev-salt", GateClass::Admission), Ok(()));
        assert_eq!(
            may_cite_mode_str("depgraph-receipt", GateClass::Performance),
            Ok(())
        );
        assert_eq!(
            may_cite_mode_str("bogus-mode", GateClass::Performance),
            Err(REASON_UNKNOWN_BUILD_IDENTITY)
        );
    }

    #[test]
    fn modes_are_exclusive() {
        // Salt only -> dev-salt (regardless of precedence setting).
        for prec in [None, Some(false), Some(true)] {
            assert_eq!(
                resolve_mode(Some("franken-robotics-dev-v1"), None, prec),
                Ok(BuildIdentityMode::DevSalt)
            );
        }

        // Receipt only -> depgraph-receipt (regardless of precedence setting).
        for prec in [None, Some(false), Some(true)] {
            assert_eq!(
                resolve_mode(None, Some("receipt-v1"), prec),
                Ok(BuildIdentityMode::DepgraphReceipt)
            );
        }

        // Neither -> refused (fail closed).
        for prec in [None, Some(false), Some(true)] {
            assert_eq!(
                resolve_mode(None, None, prec),
                Err(ResolveModeError::NeitherSet)
            );
        }

        // Empty values -> refused (fail closed).
        assert_eq!(
            resolve_mode(Some(""), None, None),
            Err(ResolveModeError::EmptyValue { var: ENV_SALT })
        );
        assert_eq!(
            resolve_mode(None, Some(""), None),
            Err(ResolveModeError::EmptyValue { var: ENV_RECEIPT })
        );

        // Both set with None or Some(false) -> refused as ambiguous.
        assert_eq!(
            resolve_mode(Some("franken-robotics-dev-v1"), Some("receipt-v1"), None),
            Err(ResolveModeError::AmbiguousBothSet {
                receipt_wins_when_both: None,
            })
        );
        assert_eq!(
            resolve_mode(
                Some("franken-robotics-dev-v1"),
                Some("receipt-v1"),
                Some(false),
            ),
            Err(ResolveModeError::AmbiguousBothSet {
                receipt_wins_when_both: Some(false),
            })
        );

        // Both set with Some(true) -> depgraph-receipt.
        assert_eq!(
            resolve_mode(
                Some("franken-robotics-dev-v1"),
                Some("receipt-v1"),
                Some(true),
            ),
            Ok(BuildIdentityMode::DepgraphReceipt)
        );
    }
}
