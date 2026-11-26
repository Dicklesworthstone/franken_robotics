//! Exact rational numbers, outward-rounded intervals, and deterministic canonical
//! byte encoding for plain proof objects (`N25b-01`).
//!
//! Zero floating-point fields (`f32` / `f64`), zero `unsafe`, determinism class D0.

use std::cmp::Ordering;
use std::fmt;

use crate::id::{Blake3, hash_blob};

/// Error returned when a proof-object component violates a structural shape invariant.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProofError {
    ZeroDenominator,
    RatioOverflow,
    InvalidInterval { detail: String },
    EmptyIdentifier { field: &'static str },
    DuplicatePort { port: String },
    PortLengthMismatch { expected: usize, observed: usize },
    InvalidCell { detail: String },
    EmptyPrefix,
    KindPayloadMismatch { declared: &'static str, payload: &'static str },
    VerdictKindMismatch { kind: &'static str, expected_polarity: &'static str },
    ClassExceedsKindCap { kind: &'static str, requested: &'static str, max_allowed: &'static str },
    EmptySupport { kind: &'static str },
    InvalidPayload { kind: &'static str, detail: String },
    InvalidExprDag { detail: String },
}

impl fmt::Display for ProofError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroDenominator => f.write_str("rational denominator cannot be zero"),
            Self::RatioOverflow => f.write_str("exact rational numerator is out of range"),
            Self::InvalidInterval { detail } => write!(f, "invalid interval: {detail}"),
            Self::EmptyIdentifier { field } => write!(f, "identifier field `{field}` cannot be empty"),
            Self::DuplicatePort { port } => write!(f, "duplicate port `{port}` in ordered port list"),
            Self::PortLengthMismatch { expected, observed } => {
                write!(f, "port vector length mismatch: expected {expected}, observed {observed}")
            }
            Self::InvalidCell { detail } => write!(f, "invalid cell: {detail}"),
            Self::EmptyPrefix => f.write_str("establishes.prefix must contain at least one binder"),
            Self::KindPayloadMismatch { declared, payload } => {
                write!(f, "proof kind `{declared}` does not match payload kind `{payload}`")
            }
            Self::VerdictKindMismatch { kind, expected_polarity } => {
                write!(f, "proof kind `{kind}` cannot establish `{expected_polarity}`")
            }
            Self::ClassExceedsKindCap { kind, requested, max_allowed } => {
                write!(f, "proof kind `{kind}` supports class at most `{max_allowed}`, got `{requested}`")
            }
            Self::EmptySupport { kind } => {
                write!(f, "proof kind `{kind}` requires non-empty dependency support")
            }
            Self::InvalidPayload { kind, detail } => write!(f, "invalid `{kind}` payload: {detail}"),
            Self::InvalidExprDag { detail } => write!(f, "invalid expression DAG: {detail}"),
        }
    }
}

impl std::error::Error for ProofError {}

/// Exact reduced rational `num / den` stored in lowest terms with strictly positive
/// denominator (`den > 0`; zero is uniquely `0 / 1`).
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ratio {
    num: i128,
    den: u128,
}

impl Ratio {
    /// Construct a reduced rational `num / den` with a representable signed numerator.
    pub fn new(num: i128, den: i128) -> Result<Self, ProofError> {
        if den == 0 {
            return Err(ProofError::ZeroDenominator);
        }
        if num == 0 {
            return Ok(Self { num: 0, den: 1 });
        }
        let neg = (num < 0) ^ (den < 0);
        let a = num.unsigned_abs();
        let b = den.unsigned_abs();
        let g = gcd_u128(a, b);
        let rn = a / g;
        let rd = b / g;
        let signed_num = if neg {
            if rn == (i128::MAX as u128) + 1 {
                i128::MIN
            } else {
                -(rn as i128)
            }
        } else {
            i128::try_from(rn).map_err(|_| ProofError::RatioOverflow)?
        };
        Ok(Self { num: signed_num, den: rd })
    }

    #[must_use]
    pub const fn from_int(n: i128) -> Self {
        Self { num: n, den: 1 }
    }

    #[must_use]
    pub const fn zero() -> Self {
        Self { num: 0, den: 1 }
    }

    #[must_use]
    pub const fn one() -> Self {
        Self { num: 1, den: 1 }
    }

    #[must_use]
    pub const fn numer(self) -> i128 {
        self.num
    }

    #[must_use]
    pub const fn denom(self) -> u128 {
        self.den
    }

    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.num == 0
    }

    #[must_use]
    pub const fn is_neg(self) -> bool {
        self.num < 0
    }

    pub const fn neg_exact(self) -> Result<Self, ProofError> {
        match self.num.checked_neg() {
            Some(num) => Ok(Self { num, den: self.den }),
            None => Err(ProofError::RatioOverflow),
        }
    }

    pub fn encode_canonical(self, enc: &mut CanonicalEncoder) {
        enc.tag(b"rat");
        enc.i128(self.num);
        enc.u128(self.den);
    }
}

const fn gcd_u128(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

impl Ord for Ratio {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.num == other.num && self.den == other.den {
            return Ordering::Equal;
        }
        match (self.num < 0, other.num < 0) {
            (true, false) => Ordering::Less,
            (false, true) => Ordering::Greater,
            (false, false) => cmp_mul_u128(self.num.unsigned_abs(), other.den, other.num.unsigned_abs(), self.den),
            (true, true) => cmp_mul_u128(other.num.unsigned_abs(), self.den, self.num.unsigned_abs(), other.den),
        }
    }
}

impl PartialOrd for Ratio {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Exact overflow-free comparison of `a * b` against `c * d` in 256-bit arithmetic.
fn cmp_mul_u128(a: u128, b: u128, c: u128, d: u128) -> Ordering {
    mul_wide(a, b).cmp(&mul_wide(c, d))
}

fn mul_wide(a: u128, b: u128) -> (u128, u128) {
    let a_lo = a & 0xFFFF_FFFF_FFFF_FFFF;
    let a_hi = a >> 64;
    let b_lo = b & 0xFFFF_FFFF_FFFF_FFFF;
    let b_hi = b >> 64;

    let ll = a_lo * b_lo;
    let lh = a_lo * b_hi;
    let hl = a_hi * b_lo;
    let hh = a_hi * b_hi;

    let mid = (ll >> 64) + (lh & 0xFFFF_FFFF_FFFF_FFFF) + (hl & 0xFFFF_FFFF_FFFF_FFFF);
    let lo = (ll & 0xFFFF_FFFF_FFFF_FFFF) | (mid << 64);
    let hi = hh + (lh >> 64) + (hl >> 64) + (mid >> 64);
    (hi, lo)
}

impl fmt::Display for Ratio {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.den == 1 {
            write!(f, "{}", self.num)
        } else {
            write!(f, "{}/{}", self.num, self.den)
        }
    }
}

impl fmt::Debug for Ratio {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

/// Outward-rounded interval `[lo, hi]` with exact rational endpoints and a declared
/// interval implementation identifier and precision in bits (no floating-point fields).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Interval {
    lo: Ratio,
    hi: Ratio,
    impl_id: String,
    precision_bits: u32,
}

impl Interval {
    pub fn new(
        lo: Ratio,
        hi: Ratio,
        impl_id: impl Into<String>,
        precision_bits: u32,
    ) -> Result<Self, ProofError> {
        let impl_id = impl_id.into();
        if impl_id.trim().is_empty() {
            return Err(ProofError::EmptyIdentifier { field: "interval.impl_id" });
        }
        if precision_bits == 0 {
            return Err(ProofError::InvalidInterval {
                detail: "precision_bits must be >= 1".to_owned(),
            });
        }
        if lo > hi {
            return Err(ProofError::InvalidInterval {
                detail: format!("lower bound {lo} exceeds upper bound {hi}"),
            });
        }
        Ok(Self { lo, hi, impl_id, precision_bits })
    }

    #[must_use]
    pub const fn lo(&self) -> Ratio {
        self.lo
    }

    #[must_use]
    pub const fn hi(&self) -> Ratio {
        self.hi
    }

    #[must_use]
    pub fn impl_id(&self) -> &str {
        &self.impl_id
    }

    #[must_use]
    pub const fn precision_bits(&self) -> u32 {
        self.precision_bits
    }

    pub fn encode_canonical(&self, enc: &mut CanonicalEncoder) {
        enc.tag(b"iv");
        self.lo.encode_canonical(enc);
        self.hi.encode_canonical(enc);
        enc.str(&self.impl_id);
        enc.u32(self.precision_bits);
    }
}

/// Deterministic D0 canonical encoder for proof objects and expression DAGs.
///
/// Length-prefixes every string and sequence so distinct structured values never collide.
#[derive(Debug, Clone, Default)]
pub struct CanonicalEncoder {
    buf: Vec<u8>,
}

impl CanonicalEncoder {
    #[must_use]
    pub fn new() -> Self {
        Self { buf: Vec::with_capacity(256) }
    }

    pub fn tag(&mut self, tag: &[u8]) {
        self.usize(tag.len());
        self.buf.extend_from_slice(tag);
    }

    pub fn bool(&mut self, v: bool) {
        self.buf.push(u8::from(v));
    }

    pub fn i8(&mut self, v: i8) {
        self.buf.extend_from_slice(&v.to_le_bytes());
    }

    pub fn u32(&mut self, v: u32) {
        self.buf.extend_from_slice(&v.to_le_bytes());
    }

    pub fn u64(&mut self, v: u64) {
        self.buf.extend_from_slice(&v.to_le_bytes());
    }

    pub fn usize(&mut self, v: usize) {
        self.u64(u64::try_from(v).unwrap_or(u64::MAX));
    }

    pub fn i128(&mut self, v: i128) {
        self.buf.extend_from_slice(&v.to_le_bytes());
    }

    pub fn u128(&mut self, v: u128) {
        self.buf.extend_from_slice(&v.to_le_bytes());
    }

    pub fn str(&mut self, s: &str) {
        self.usize(s.len());
        self.buf.extend_from_slice(s.as_bytes());
    }

    pub fn blake3(&mut self, d: Blake3) {
        self.buf.extend_from_slice(d.as_bytes());
    }

    #[must_use]
    pub fn finish(self) -> Vec<u8> {
        self.buf
    }

    #[must_use]
    pub fn finish_digest(self) -> Blake3 {
        hash_blob(&self.buf)
    }
}
