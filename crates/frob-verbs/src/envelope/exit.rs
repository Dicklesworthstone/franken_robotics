//! The exit-code table of the surface and the refusal-code → exit mapping (N32a-03, §32.4).
//!
//! [`ExitCode`] has exactly nine values and no conversion from an arbitrary integer: exit 1
//! is not representable, so no code path can construct it (an unhandled abort is its only
//! source). The exit of a refusal is read from its row in the refusal registry
//! (`registries/refusals.toml`, BD-005's loader, N35-33's `[exit_rule]`); this module
//! chooses no value. Codes are parsed and compared as whole tokens
//! (`FROB-REF-[1-9][0-9]{2,3}`, ADR-NUM-refusal-8xx): `FROB-REF-8100` is never read as
//! `FROB-REF-810`.

use std::fmt;

use frob_core::registry::refusals::{RefusalRegistry, RefusalRegistryError};

/// Failure class of an exit code (§32.4).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FailureClass {
    /// Not a failure (0, 2, 8).
    None,
    /// Domain failure: the question or the session was refused (3 to 7).
    Domain,
    /// Transport failure or integrity fault (9).
    Transport,
}

impl FailureClass {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Domain => "domain",
            Self::Transport => "transport",
        }
    }
}

/// The nine exit codes of `frob --json` (and the `exit` field of every envelope).
///
/// ```compile_fail,E0277
/// use frob_verbs::envelope::ExitCode;
/// let _: ExitCode = 1_i32.into();
/// ```
///
/// ```compile_fail,E0599
/// use frob_verbs::envelope::ExitCode;
/// let _ = ExitCode::Failure;
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum ExitCode {
    /// 0: completed, nothing to flag.
    Completed = 0,
    /// 2: completed with findings or flags.
    Flagged = 2,
    /// 3: refused input or question.
    Refused = 3,
    /// 4: budget refused at admission or stopped; nothing partial is published.
    Budget = 4,
    /// 5: capability not implemented, not admitted, or outside the token's scopes.
    Capability = 5,
    /// 6: backend or device unavailable.
    Unavailable = 6,
    /// 7: conflict (key reuse with different content, stale parent, lineage).
    Conflict = 7,
    /// 8: accepted and still running; the envelope carries a job handle.
    Running = 8,
    /// 9: indeterminate; never retry blindly, recover by key. Also integrity faults.
    Indeterminate = 9,
}

impl ExitCode {
    /// All nine values, ascending.
    pub const ALL: [Self; 9] = [
        Self::Completed,
        Self::Flagged,
        Self::Refused,
        Self::Budget,
        Self::Capability,
        Self::Unavailable,
        Self::Conflict,
        Self::Running,
        Self::Indeterminate,
    ];

    /// The process exit status.
    #[must_use]
    pub const fn code(self) -> u8 {
        self as u8
    }

    /// One-line meaning, as the exit table prints it.
    #[must_use]
    pub const fn meaning(self) -> &'static str {
        match self {
            Self::Completed => "Completed, nothing to flag",
            Self::Flagged => "Completed with findings or flags",
            Self::Refused => "Refused input or question",
            Self::Budget => {
                "Budget: refused at admission or stopped; nothing partial is published as a result"
            }
            Self::Capability => {
                "Capability not implemented, not admitted, or outside the session token's scopes"
            }
            Self::Unavailable => "Backend or device unavailable",
            Self::Conflict => {
                "Conflict: idempotency key reuse with different content, stale parent, or lineage conflict"
            }
            Self::Running => "Accepted and still running: the envelope carries a job handle",
            Self::Indeterminate => "Indeterminate: never retry blindly; recover by key",
        }
    }

    #[must_use]
    pub const fn failure_class(self) -> FailureClass {
        match self {
            Self::Completed | Self::Flagged | Self::Running => FailureClass::None,
            Self::Refused
            | Self::Budget
            | Self::Capability
            | Self::Unavailable
            | Self::Conflict => FailureClass::Domain,
            Self::Indeterminate => FailureClass::Transport,
        }
    }

    /// `ok` of an envelope with this exit (`true` for 0, 2 and 8).
    #[must_use]
    pub const fn is_ok(self) -> bool {
        matches!(self, Self::Completed | Self::Flagged | Self::Running)
    }

    /// The exit a refusal-registry row may carry (3, 4, 5, 6, 7 or 9); `None` otherwise.
    #[must_use]
    pub const fn from_refusal_exit(exit: u8) -> Option<Self> {
        match exit {
            3 => Some(Self::Refused),
            4 => Some(Self::Budget),
            5 => Some(Self::Capability),
            6 => Some(Self::Unavailable),
            7 => Some(Self::Conflict),
            9 => Some(Self::Indeterminate),
            _ => None,
        }
    }

    /// Read the `exit` member of an envelope. `1` and every value outside the table is `None`.
    #[must_use]
    pub fn from_wire(exit: i64) -> Option<Self> {
        Self::ALL.into_iter().find(|e| i64::from(e.code()) == exit)
    }
}

impl fmt::Display for ExitCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.code())
    }
}

/// Parse a refusal code as a whole token: `FROB-REF-` then a non-zero digit and two or three
/// more digits (ADR-NUM-refusal-8xx). Returns the number.
#[must_use]
pub fn parse_refusal_code(code: &str) -> Option<u16> {
    let digits = code.strip_prefix("FROB-REF-")?;
    let b = digits.as_bytes();
    if !(3..=4).contains(&b.len()) || !b.iter().all(u8::is_ascii_digit) || b[0] == b'0' {
        return None;
    }
    digits.parse().ok()
}

/// Why a refusal code has no exit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExitLookupError {
    /// Not of the form `FROB-REF-[1-9][0-9]{2,3}`.
    MalformedCode(String),
    /// Well-formed but not a row of the registry.
    Unregistered(String),
    /// The registry row carries an exit outside {3, 4, 5, 6, 7, 9} (the loader forbids it).
    BadRowExit { code: String, exit: u8 },
    /// No `[exit_rule]` entry covers the code.
    NoRule { code: String, reason: String },
}

impl fmt::Display for ExitLookupError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MalformedCode(c) => write!(
                f,
                "malformed refusal code `{c}` (expected FROB-REF-[1-9][0-9]{{2,3}})"
            ),
            Self::Unregistered(c) => {
                write!(f, "refusal code `{c}` is not a row of the refusal registry")
            }
            Self::BadRowExit { code, exit } => write!(
                f,
                "registry row `{code}` carries exit {exit}, outside the refusal exits"
            ),
            Self::NoRule { code, reason } => write!(f, "no exit rule covers `{code}`: {reason}"),
        }
    }
}

impl std::error::Error for ExitLookupError {}

/// The exit of a registered refusal code: the `exit` field of its registry row.
pub fn exit_for_code(registry: &RefusalRegistry, code: &str) -> Result<ExitCode, ExitLookupError> {
    parse_refusal_code(code).ok_or_else(|| ExitLookupError::MalformedCode(code.to_owned()))?;
    let row = registry
        .lookup(code, "frob.envelope/1 refusal block")
        .map_err(|_| ExitLookupError::Unregistered(code.to_owned()))?;
    // Whole-token check on top of the loader's own parse: the row found must be this code.
    if row.code != code {
        return Err(ExitLookupError::Unregistered(code.to_owned()));
    }
    ExitCode::from_refusal_exit(row.exit).ok_or_else(|| ExitLookupError::BadRowExit {
        code: code.to_owned(),
        exit: row.exit,
    })
}

/// The exit the registry's `[exit_rule]` assigns to a code whether or not it has a row:
/// the value a row of that code must carry to load.
pub fn rule_exit_for_code(
    registry: &RefusalRegistry,
    code: &str,
) -> Result<ExitCode, ExitLookupError> {
    let number =
        parse_refusal_code(code).ok_or_else(|| ExitLookupError::MalformedCode(code.to_owned()))?;
    let (exit, _entry) = registry
        .exit_rule()
        .expected_exit(number)
        .map_err(|e| match e {
            RefusalRegistryError::ExitMismatch { reason, .. } => ExitLookupError::NoRule {
                code: code.to_owned(),
                reason,
            },
            other => ExitLookupError::NoRule {
                code: code.to_owned(),
                reason: other.to_string(),
            },
        })?;
    ExitCode::from_refusal_exit(exit).ok_or(ExitLookupError::BadRowExit {
        code: code.to_owned(),
        exit,
    })
}
