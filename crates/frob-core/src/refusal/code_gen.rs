// @generated from registries/refusals.toml by frob_core::registry::refusals::render_refusal_code_enum (N34p4-03 check-generated); do not edit by hand.

use std::fmt;

/// Closed refusal code enum generated from `registries/refusals.toml`.
///
/// Every variant corresponds to one registered `[[refusal]]` row; an unregistered
/// code cannot compile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum RefusalCode {
    /// FROB-REF-251 (§35.11): A banned operation appears in kernel source (build time): a fast-math relaxation, a fused multiply-add (contracted or explicit), a built-in transcendental or reciprocal square root, a floating-point atomic
    FrobRef251 = 251,
    /// FROB-REF-790 (§33.16): This binary was built without the airlock feature group the verb needs
    FrobRef790 = 790,
    /// FROB-REF-802 (§32.18): Arguments fail the schema, including an unknown parameter or an out-of-range sync_wait_ms
    FrobRef802 = 802,
    /// FROB-REF-816 (§37.12): An artifact whose authorization flags are missing or not false
    FrobRef816 = 816,
    /// FROB-REF-881 (§25.24): Evidence or a certificate establishes a different quantifier prefix than the query's, including clairvoyant evidence offered for a robust claim
    FrobRef881 = 881,
    /// FROB-REF-890 (§33.16): Evidence recorded under another stack receipt is offered as same-stack evidence without revalidation
    FrobRef890 = 890,
}

impl RefusalCode {
    /// All registered refusal codes in ascending numeric order.
    pub const ALL: &'static [Self] = &[
        Self::FrobRef251,
        Self::FrobRef790,
        Self::FrobRef802,
        Self::FrobRef816,
        Self::FrobRef881,
        Self::FrobRef890,
    ];

    /// Numeric code in `100..=999`.
    #[must_use]
    pub const fn number(self) -> u16 {
        self as u16
    }

    /// Family digit (`1..=9`).
    #[must_use]
    pub const fn family(self) -> u8 {
        (self.number() / 100) as u8
    }

    /// Canonical wire spelling `FROB-REF-nnn`.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FrobRef251 => "FROB-REF-251",
            Self::FrobRef790 => "FROB-REF-790",
            Self::FrobRef802 => "FROB-REF-802",
            Self::FrobRef816 => "FROB-REF-816",
            Self::FrobRef881 => "FROB-REF-881",
            Self::FrobRef890 => "FROB-REF-890",
        }
    }

    /// Parse a canonical wire code `FROB-REF-nnn`.
    #[must_use]
    pub fn parse(code: &str) -> Option<Self> {
        match code {
            "FROB-REF-251" => Some(Self::FrobRef251),
            "FROB-REF-790" => Some(Self::FrobRef790),
            "FROB-REF-802" => Some(Self::FrobRef802),
            "FROB-REF-816" => Some(Self::FrobRef816),
            "FROB-REF-881" => Some(Self::FrobRef881),
            "FROB-REF-890" => Some(Self::FrobRef890),
            _ => None,
        }
    }
}

impl fmt::Display for RefusalCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
