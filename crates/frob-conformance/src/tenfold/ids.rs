//! Identifier types for the FrankenRobotics ten-fold harness (bead `N39a-01`, §39.1).
//!
//! Lives inside the scorer boundary (`BOUNDARY_FILES`): imports strictly `std`
//! and `frob_cert` (plus sibling boundary modules). Zero floats, zero `unsafe`,
//! determinism class D0.

use std::fmt;

use crate::tenfold::refusal::TenfoldRefusal;

/// Closed gate identifiers accepted in ten-fold harness record headers.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GateId {
    M0Baseline,
    M1Baseline,
    M3Partial,
    M7Tournament,
    Rehearsal(String),
}

impl GateId {
    /// Parse a strict gate identifier (`M0-baseline`, `M1-baseline`,
    /// `M3-partial`, `M7-tournament`, or `rehearsal-<name>`).
    pub fn parse(raw: &str, pointer: &str) -> Result<Self, TenfoldRefusal> {
        match raw {
            "M0-baseline" => Ok(Self::M0Baseline),
            "M1-baseline" => Ok(Self::M1Baseline),
            "M3-partial" => Ok(Self::M3Partial),
            "M7-tournament" => Ok(Self::M7Tournament),
            other => {
                if let Some(name) = other.strip_prefix("rehearsal-")
                    && !name.is_empty()
                    && name.bytes().all(|b| {
                        b.is_ascii_lowercase() || b.is_ascii_digit() || matches!(b, b'-' | b'_')
                    })
                    && name.as_bytes()[0].is_ascii_alphanumeric()
                {
                    return Ok(Self::Rehearsal(name.to_owned()));
                }
                Err(TenfoldRefusal::noncanonical_number(
                    pointer,
                    format!("{raw:?}"),
                    "one of M0-baseline, M1-baseline, M3-partial, M7-tournament, or rehearsal-<slug>",
                    vec![
                        "Use a valid gate identifier matching the ten-fold header grammar"
                            .to_owned(),
                    ],
                ))
            }
        }
    }

    #[must_use]
    pub fn as_string(&self) -> String {
        match self {
            Self::M0Baseline => "M0-baseline".to_owned(),
            Self::M1Baseline => "M1-baseline".to_owned(),
            Self::M3Partial => "M3-partial".to_owned(),
            Self::M7Tournament => "M7-tournament".to_owned(),
            Self::Rehearsal(name) => format!("rehearsal-{name}"),
        }
    }

    #[must_use]
    pub const fn is_rehearsal(&self) -> bool {
        matches!(self, Self::Rehearsal(_))
    }
}

impl fmt::Display for GateId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.as_string())
    }
}

/// Experimental arms of the ten-fold test (`A` FrankenRobotics, `B` strongest
/// composed rival, `C` ablation arm, `D` optional product-over-incumbent arm).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Arm {
    A,
    B,
    C,
    D,
}

impl Arm {
    pub const ALL: [Self; 4] = [Self::A, Self::B, Self::C, Self::D];

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::A => "A",
            Self::B => "B",
            Self::C => "C",
            Self::D => "D",
        }
    }

    pub fn parse(raw: &str, pointer: &str) -> Result<Self, TenfoldRefusal> {
        match raw {
            "A" => Ok(Self::A),
            "B" => Ok(Self::B),
            "C" => Ok(Self::C),
            "D" => Ok(Self::D),
            other => Err(TenfoldRefusal::noncanonical_number(
                pointer,
                format!("{other:?}"),
                "arm in {A, B, C, D}",
                vec!["Set arm to one of \"A\", \"B\", \"C\", or \"D\"".to_owned()],
            )),
        }
    }
}

impl fmt::Display for Arm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// The eleven closed strata of the ten-fold suite (`N39a-12`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StratumId {
    LeggedQuad,
    LeggedBiped,
    Wheeled,
    Tracked,
    Hybrid,
    Envelope,
    Sensing,
    Rebody,
    Failure,
    Throughput,
    Facility,
}

impl StratumId {
    pub const ALL: [Self; 11] = [
        Self::LeggedQuad,
        Self::LeggedBiped,
        Self::Wheeled,
        Self::Tracked,
        Self::Hybrid,
        Self::Envelope,
        Self::Sensing,
        Self::Rebody,
        Self::Failure,
        Self::Throughput,
        Self::Facility,
    ];

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LeggedQuad => "legged-quad",
            Self::LeggedBiped => "legged-biped",
            Self::Wheeled => "wheeled",
            Self::Tracked => "tracked",
            Self::Hybrid => "hybrid",
            Self::Envelope => "envelope",
            Self::Sensing => "sensing",
            Self::Rebody => "rebody",
            Self::Failure => "failure",
            Self::Throughput => "throughput",
            Self::Facility => "facility",
        }
    }

    pub fn parse(raw: &str, pointer: &str) -> Result<Self, TenfoldRefusal> {
        match raw {
            "legged-quad" => Ok(Self::LeggedQuad),
            "legged-biped" => Ok(Self::LeggedBiped),
            "wheeled" => Ok(Self::Wheeled),
            "tracked" => Ok(Self::Tracked),
            "hybrid" => Ok(Self::Hybrid),
            "envelope" => Ok(Self::Envelope),
            "sensing" => Ok(Self::Sensing),
            "rebody" => Ok(Self::Rebody),
            "failure" => Ok(Self::Failure),
            "throughput" => Ok(Self::Throughput),
            "facility" => Ok(Self::Facility),
            other => Err(TenfoldRefusal::noncanonical_number(
                pointer,
                format!("{other:?}"),
                "one of the eleven registered strata (legged-quad..facility)",
                vec!["Use one of the eleven closed StratumId names from N39a-12".to_owned()],
            )),
        }
    }
}

impl fmt::Display for StratumId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Evaluation regime (`cold` or `warm`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Regime {
    Cold,
    Warm,
}

impl Regime {
    pub const ALL: [Self; 2] = [Self::Cold, Self::Warm];

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Cold => "cold",
            Self::Warm => "warm",
        }
    }

    pub fn parse(raw: &str, pointer: &str) -> Result<Self, TenfoldRefusal> {
        match raw {
            "cold" => Ok(Self::Cold),
            "warm" => Ok(Self::Warm),
            other => Err(TenfoldRefusal::noncanonical_number(
                pointer,
                format!("{other:?}"),
                "regime in {cold, warm}",
                vec!["Set regime to \"cold\" or \"warm\"".to_owned()],
            )),
        }
    }
}

impl fmt::Display for Regime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// The thirteen exclusive phases plus `unattributed` (`N39a-23`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Phase {
    Author,
    Propose,
    Check,
    Synthesize,
    Compile,
    Screen,
    Train,
    Evaluate,
    Qualify,
    Certify,
    Diagnose,
    Wait,
    Adjudicate,
    Unattributed,
}

impl Phase {
    /// The thirteen exclusive phases in canonical specification order (`N39a-23`),
    /// excluding the fourteenth non-phase bucket `unattributed`.
    pub const PHASES: [Self; 13] = [
        Self::Author,
        Self::Propose,
        Self::Check,
        Self::Synthesize,
        Self::Compile,
        Self::Screen,
        Self::Train,
        Self::Evaluate,
        Self::Qualify,
        Self::Certify,
        Self::Diagnose,
        Self::Wait,
        Self::Adjudicate,
    ];

    /// All fourteen phase ledger buckets in canonical specification order.
    pub const ALL: [Self; 14] = [
        Self::Author,
        Self::Propose,
        Self::Check,
        Self::Synthesize,
        Self::Compile,
        Self::Screen,
        Self::Train,
        Self::Evaluate,
        Self::Qualify,
        Self::Certify,
        Self::Diagnose,
        Self::Wait,
        Self::Adjudicate,
        Self::Unattributed,
    ];

    /// Dense index `0..14` matching `Phase::ALL`.
    #[must_use]
    pub const fn index(self) -> usize {
        match self {
            Self::Author => 0,
            Self::Propose => 1,
            Self::Check => 2,
            Self::Synthesize => 3,
            Self::Compile => 4,
            Self::Screen => 5,
            Self::Train => 6,
            Self::Evaluate => 7,
            Self::Qualify => 8,
            Self::Certify => 9,
            Self::Diagnose => 10,
            Self::Wait => 11,
            Self::Adjudicate => 12,
            Self::Unattributed => 13,
        }
    }

    /// True for the thirteen phases; false for the fourteenth bucket `unattributed`.
    #[must_use]
    pub const fn is_phase(self) -> bool {
        !matches!(self, Self::Unattributed)
    }

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Author => "author",
            Self::Propose => "propose",
            Self::Check => "check",
            Self::Synthesize => "synthesize",
            Self::Compile => "compile",
            Self::Screen => "screen",
            Self::Train => "train",
            Self::Evaluate => "evaluate",
            Self::Qualify => "qualify",
            Self::Certify => "certify",
            Self::Diagnose => "diagnose",
            Self::Wait => "wait",
            Self::Adjudicate => "adjudicate",
            Self::Unattributed => "unattributed",
        }
    }

    /// Strictly parse one of the thirteen phases for raw interval records (`N39a-23`).
    /// Refuses `"unattributed"` and any unknown/fourteenth phase name at read time.
    pub fn parse_phase(raw: &str, pointer: &str) -> Result<Self, TenfoldRefusal> {
        match raw {
            "author" => Ok(Self::Author),
            "propose" => Ok(Self::Propose),
            "check" => Ok(Self::Check),
            "synthesize" => Ok(Self::Synthesize),
            "compile" => Ok(Self::Compile),
            "screen" => Ok(Self::Screen),
            "train" => Ok(Self::Train),
            "evaluate" => Ok(Self::Evaluate),
            "qualify" => Ok(Self::Qualify),
            "certify" => Ok(Self::Certify),
            "diagnose" => Ok(Self::Diagnose),
            "wait" => Ok(Self::Wait),
            "adjudicate" => Ok(Self::Adjudicate),
            other => Err(TenfoldRefusal::unknown_field(
                pointer,
                format!("{other:?}"),
                "one of the thirteen phases (author, propose, check, synthesize, compile, screen, train, evaluate, qualify, certify, diagnose, wait, adjudicate)",
            )),
        }
    }

    pub fn parse(raw: &str, pointer: &str) -> Result<Self, TenfoldRefusal> {
        if raw == "unattributed" {
            return Ok(Self::Unattributed);
        }
        Self::parse_phase(raw, pointer).map_err(|_| {
            TenfoldRefusal::unknown_field(
                pointer,
                format!("{raw:?}"),
                "one of the thirteen phases plus unattributed",
            )
        })
    }
}

impl fmt::Display for Phase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// 32-byte BLAKE3 content address, rendered canonically as `b3:<64 lower hex>`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Hash32(pub [u8; 32]);

impl Hash32 {
    #[must_use]
    pub const fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    #[must_use]
    pub fn hex(&self) -> String {
        frob_cert::blake3::to_hex(&self.0)
    }

    #[must_use]
    pub fn to_b3_string(&self) -> String {
        format!("b3:{}", self.hex())
    }

    /// Parse a strict `b3:<64 lower-case hex>` content address.
    pub fn parse_b3(raw: &str, pointer: &str) -> Result<Self, TenfoldRefusal> {
        let Some(hex) = raw.strip_prefix("b3:") else {
            return Err(TenfoldRefusal::hash_mismatch(
                pointer,
                format!("{raw:?}"),
                "b3:<64 lowercase hex digits>",
                vec![
                    "Prefix content addresses with `b3:` followed by 64 lowercase hex digits"
                        .to_owned(),
                ],
            ));
        };
        parse_hex32(hex, pointer).map(Self)
    }
}

impl fmt::Display for Hash32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_b3_string())
    }
}

fn hex_nibble(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        _ => None,
    }
}

fn parse_hex32(hex: &str, pointer: &str) -> Result<[u8; 32], TenfoldRefusal> {
    let bytes = hex.as_bytes();
    if bytes.len() != 64 {
        return Err(TenfoldRefusal::hash_mismatch(
            pointer,
            format!("{hex:?}"),
            "64 lowercase hex digits",
            vec!["Provide a full 256-bit (64 lowercase hex digit) BLAKE3 digest".to_owned()],
        ));
    }
    let mut out = [0_u8; 32];
    for (idx, chunk) in bytes.chunks_exact(2).enumerate() {
        let Some(hi) = hex_nibble(chunk[0]) else {
            return Err(TenfoldRefusal::hash_mismatch(
                pointer,
                format!("{hex:?}"),
                "64 lowercase hex digits",
                vec!["Use strictly lowercase hexadecimal digits [0-9a-f]".to_owned()],
            ));
        };
        let Some(lo) = hex_nibble(chunk[1]) else {
            return Err(TenfoldRefusal::hash_mismatch(
                pointer,
                format!("{hex:?}"),
                "64 lowercase hex digits",
                vec!["Use strictly lowercase hexadecimal digits [0-9a-f]".to_owned()],
            ));
        };
        out[idx] = (hi << 4) | lo;
    }
    Ok(out)
}

/// Mission family identifier (`f:` + 64 lowercase hex digits).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FamilyId(pub Hash32);

impl FamilyId {
    pub fn parse(raw: &str, pointer: &str) -> Result<Self, TenfoldRefusal> {
        let Some(hex) = raw.strip_prefix("f:") else {
            return Err(TenfoldRefusal::noncanonical_number(
                pointer,
                format!("{raw:?}"),
                "f:<64 lowercase hex digits>",
                vec!["Format FamilyId as `f:` followed by 64 lowercase hex digits".to_owned()],
            ));
        };
        let bytes = parse_hex32(hex, pointer).map_err(|_| {
            TenfoldRefusal::noncanonical_number(
                pointer,
                format!("{raw:?}"),
                "f:<64 lowercase hex digits>",
                vec!["Use strictly 64 lowercase hex digits after `f:`".to_owned()],
            )
        })?;
        Ok(Self(Hash32(bytes)))
    }

    #[must_use]
    pub fn as_string(&self) -> String {
        format!("f:{}", self.0.hex())
    }

    #[must_use]
    pub fn hex(&self) -> String {
        self.0.hex()
    }
}

impl fmt::Display for FamilyId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.as_string())
    }
}

/// Content address of a mission card (`b3:<64 hex>`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MissionHash(pub Hash32);

impl MissionHash {
    pub fn parse(raw: &str, pointer: &str) -> Result<Self, TenfoldRefusal> {
        Hash32::parse_b3(raw, pointer).map(Self)
    }

    #[must_use]
    pub fn hex(&self) -> String {
        self.0.hex()
    }

    #[must_use]
    pub fn to_b3_string(&self) -> String {
        self.0.to_b3_string()
    }
}

impl fmt::Display for MissionHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_b3_string())
    }
}

/// Campaign index (`u32`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CampaignIndex(pub u32);

impl CampaignIndex {
    pub fn from_i64(n: i64, pointer: &str) -> Result<Self, TenfoldRefusal> {
        if n < 0 || n > i64::from(u32::MAX) {
            return Err(TenfoldRefusal::noncanonical_number(
                pointer,
                n.to_string(),
                "integer in 0..=4294967295",
                vec!["Supply a non-negative 32-bit integer campaign index".to_owned()],
            ));
        }
        Ok(Self(n as u32))
    }
}

impl fmt::Display for CampaignIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Exact duration in integer milliseconds (`u64`), parsed strictly from and
/// formatted strictly to canonical seconds strings matching
/// `^(0|[1-9][0-9]*)(\.[0-9]{0,2}[1-9])?$`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Millis(pub u64);

impl Millis {
    #[must_use]
    pub const fn as_u64(self) -> u64 {
        self.0
    }

    /// Checked sum of milliseconds.
    pub fn checked_add(self, rhs: Self) -> Option<Self> {
        self.0.checked_add(rhs.0).map(Self)
    }

    /// Strictly parse a canonical seconds string (`^(0|[1-9][0-9]*)(\.[0-9]{0,2}[1-9])?$`)
    /// into exact integer milliseconds without floating-point arithmetic.
    pub fn parse_canonical_seconds(raw: &str, pointer: &str) -> Result<Self, TenfoldRefusal> {
        let reject = |detail: &str| {
            TenfoldRefusal::noncanonical_number(
                pointer,
                format!("{raw:?}"),
                "canonical seconds string matching ^(0|[1-9][0-9]*)(\\.[0-9]{0,2}[1-9])?$",
                vec![format!(
                    "Rewrite duration as canonical decimal seconds ({detail})"
                )],
            )
        };

        if raw.is_empty() {
            return Err(reject("empty string"));
        }

        let (whole_str, frac_str) = match raw.split_once('.') {
            Some((w, f)) => {
                if f.contains('.') {
                    return Err(reject("multiple decimal points"));
                }
                (w, Some(f))
            }
            None => (raw, None),
        };

        if whole_str.is_empty() || !whole_str.bytes().all(|b| b.is_ascii_digit()) {
            return Err(reject("integer seconds must be unsigned base-10 digits"));
        }
        if whole_str.len() > 1 && whole_str.starts_with('0') {
            return Err(reject("leading zeros are forbidden"));
        }

        let secs: u64 = whole_str
            .parse()
            .map_err(|_| reject("seconds value overflows u64"))?;
        let base_ms = secs
            .checked_mul(1_000)
            .ok_or_else(|| reject("millisecond conversion overflows u64"))?;

        let frac_ms: u64 = match frac_str {
            None => 0,
            Some(f) => {
                if f.is_empty() {
                    return Err(reject("trailing decimal point with no fractional digits"));
                }
                if f.len() > 3 {
                    return Err(reject(
                        "at most three fractional digits (millisecond resolution)",
                    ));
                }
                if !f.bytes().all(|b| b.is_ascii_digit()) {
                    return Err(reject("fractional part must contain only ASCII digits"));
                }
                if f.ends_with('0') {
                    return Err(reject(
                        "trailing zero in fractional seconds is non-canonical",
                    ));
                }
                let val: u64 = f.parse().map_err(|_| reject("invalid fractional digits"))?;
                match f.len() {
                    1 => val * 100,
                    2 => val * 10,
                    3 => val,
                    _ => return Err(reject("invalid fractional length")),
                }
            }
        };

        let total = base_ms
            .checked_add(frac_ms)
            .ok_or_else(|| reject("total milliseconds overflow u64"))?;
        Ok(Self(total))
    }

    /// Emit the unique canonical seconds string matching
    /// `^(0|[1-9][0-9]*)(\.[0-9]{0,2}[1-9])?$`.
    #[must_use]
    pub fn to_canonical_seconds(self) -> String {
        let secs = self.0 / 1_000;
        let rem = self.0 % 1_000;
        if rem == 0 {
            secs.to_string()
        } else if rem % 100 == 0 {
            format!("{secs}.{}", rem / 100)
        } else if rem % 10 == 0 {
            format!("{secs}.{:02}", rem / 10)
        } else {
            format!("{secs}.{rem:03}")
        }
    }
}

impl fmt::Display for Millis {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_canonical_seconds())
    }
}
