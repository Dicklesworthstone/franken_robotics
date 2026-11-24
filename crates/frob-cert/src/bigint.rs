//! Arbitrary-precision signed integers with strict bit-length limits (BD-026).
//!
//! Sign-and-magnitude representation using little-endian `u32` limbs.
//! Operations enforce `COMPILED_LIMITS.max_bigint_bits` (16,384 bits by default)
//! on every operand and result, returning `ArithError::LimitsExceeded` (`limits.exceeded`)
//! instead of allocating beyond the declared limit.

use std::cmp::Ordering;
use std::fmt;

use crate::limits::{
    COMPILED_LIMITS, REFUSAL_ARITHMETIC_MALFORMED, RULE_LIMITS_EXCEEDED, RULE_NUM_INT_FORM,
    RULE_NUM_RATIONAL_FORM,
};

/// Errors produced by exact integer, rational, or interval arithmetic.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArithError {
    /// An operand or result exceeded `max_bigint_bits` (rule `limits.exceeded`).
    LimitsExceeded {
        bits: usize,
        limit: usize,
    },
    /// Malformed integer string (leading zero, `-0`, empty, non-digit).
    InvalidIntForm(String),
    /// Malformed or non-reduced rational encoding (`num.rational-form`, `FROB-REF-849`).
    InvalidRationalForm(String),
    /// Division by zero or by an interval containing zero.
    DivisionByZero,
    /// Domain error (e.g., square root of a negative number or interval, mismatched precision, lo > hi).
    DomainError(String),
}

impl ArithError {
    /// Stable rule identifier associated with this error.
    pub fn rule(&self) -> &'static str {
        match self {
            Self::LimitsExceeded { .. } => RULE_LIMITS_EXCEEDED,
            Self::InvalidIntForm(_) => RULE_NUM_INT_FORM,
            Self::InvalidRationalForm(_) | Self::DivisionByZero | Self::DomainError(_) => {
                RULE_NUM_RATIONAL_FORM
            }
        }
    }

    /// Optional refusal code (BD-026 item 2: `FROB-REF-849` for malformed arithmetic).
    pub fn refusal(&self) -> Option<&'static str> {
        match self {
            Self::LimitsExceeded { .. } => None,
            Self::InvalidIntForm(_)
            | Self::InvalidRationalForm(_)
            | Self::DivisionByZero
            | Self::DomainError(_) => Some(REFUSAL_ARITHMETIC_MALFORMED),
        }
    }
}

impl fmt::Display for ArithError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LimitsExceeded { bits, limit } => {
                write!(f, "integer bit length {bits} exceeds limit {limit}")
            }
            Self::InvalidIntForm(msg) => write!(f, "invalid integer form: {msg}"),
            Self::InvalidRationalForm(msg) => write!(f, "invalid rational form: {msg}"),
            Self::DivisionByZero => write!(f, "division by zero"),
            Self::DomainError(msg) => write!(f, "arithmetic domain error: {msg}"),
        }
    }
}

impl std::error::Error for ArithError {}

/// Arbitrary-precision signed integer in canonical sign-and-magnitude form.
///
/// Invariants:
/// - `limbs` is little-endian base `2^32` with no trailing `0` limbs.
/// - Zero is represented by `limbs.is_empty()` and `neg == false`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BigInt {
    neg: bool,
    limbs: Vec<u32>,
}

impl Default for BigInt {
    fn default() -> Self {
        Self::zero()
    }
}

impl BigInt {
    /// Zero (`0`).
    pub fn zero() -> Self {
        Self {
            neg: false,
            limbs: Vec::new(),
        }
    }

    /// One (`1`).
    pub fn one() -> Self {
        Self {
            neg: false,
            limbs: vec![1],
        }
    }

    /// Construct from `u64`.
    pub fn from_u64(v: u64) -> Self {
        if v == 0 {
            return Self::zero();
        }
        let lo = v as u32;
        let hi = (v >> 32) as u32;
        if hi == 0 {
            Self {
                neg: false,
                limbs: vec![lo],
            }
        } else {
            Self {
                neg: false,
                limbs: vec![lo, hi],
            }
        }
    }

    /// Construct from `i64`.
    pub fn from_i64(v: i64) -> Self {
        if v == 0 {
            return Self::zero();
        }
        let neg = v < 0;
        let mag = v.unsigned_abs();
        let mut res = Self::from_u64(mag);
        res.neg = neg;
        res
    }

    /// Construct from `u128`.
    pub fn from_u128(mut v: u128) -> Self {
        if v == 0 {
            return Self::zero();
        }
        let mut limbs = Vec::with_capacity(4);
        while v != 0 {
            limbs.push(v as u32);
            v >>= 32;
        }
        Self { neg: false, limbs }
    }

    /// Construct from `i128`.
    pub fn from_i128(v: i128) -> Self {
        if v == 0 {
            return Self::zero();
        }
        let neg = v < 0;
        let mut res = Self::from_u128(v.unsigned_abs());
        res.neg = neg;
        res
    }

    /// Construct from sign and little-endian `u32` limbs, checking the compiled bit limit.
    pub fn from_parts(neg: bool, mut limbs: Vec<u32>) -> Result<Self, ArithError> {
        trim_limbs(&mut limbs);
        let is_neg = neg && !limbs.is_empty();
        let val = Self { neg: is_neg, limbs };
        val.check_limit(COMPILED_LIMITS.max_bigint_bits)?;
        Ok(val)
    }

    /// Construct from sign and limbs without enforcing the bit limit (crate-internal
    /// for intermediate cross-products in `Ord::cmp`).
    pub(crate) fn from_parts_unchecked(neg: bool, mut limbs: Vec<u32>) -> Self {
        trim_limbs(&mut limbs);
        Self {
            neg: neg && !limbs.is_empty(),
            limbs,
        }
    }

    /// Little-endian magnitude limbs (read-only).
    pub fn limbs(&self) -> &[u32] {
        &self.limbs
    }

    /// Exact bit length of the magnitude (`0` for zero).
    pub fn bit_len(&self) -> usize {
        mag_bit_len(&self.limbs)
    }

    /// Check that `self.bit_len() <= max_bits`.
    pub fn check_limit(&self, max_bits: usize) -> Result<(), ArithError> {
        let bits = self.bit_len();
        if bits > max_bits {
            return Err(ArithError::LimitsExceeded {
                bits,
                limit: max_bits,
            });
        }
        Ok(())
    }

    pub fn is_zero(&self) -> bool {
        self.limbs.is_empty()
    }

    pub fn is_negative(&self) -> bool {
        self.neg && !self.limbs.is_empty()
    }

    pub fn is_positive(&self) -> bool {
        !self.neg && !self.limbs.is_empty()
    }

    pub fn is_one(&self) -> bool {
        !self.neg && self.limbs.len() == 1 && self.limbs[0] == 1
    }

    pub fn signum(&self) -> i8 {
        if self.limbs.is_empty() {
            0
        } else if self.neg {
            -1
        } else {
            1
        }
    }

    pub fn abs(&self) -> Self {
        Self {
            neg: false,
            limbs: self.limbs.clone(),
        }
    }

    pub fn negate(&self) -> Self {
        if self.limbs.is_empty() {
            Self::zero()
        } else {
            Self {
                neg: !self.neg,
                limbs: self.limbs.clone(),
            }
        }
    }

    /// Parse a strict canonical decimal integer string (BD-026 item 2):
    /// optional leading `-`, ASCII digits only, no leading zeros (except `"0"`), `"-0"` refused.
    pub fn parse_strict(s: &str) -> Result<Self, ArithError> {
        Self::parse_strict_with_limit(s, COMPILED_LIMITS.max_bigint_bits)
    }

    /// Strict decimal parse with an explicit bit limit.
    pub fn parse_strict_with_limit(s: &str, max_bits: usize) -> Result<Self, ArithError> {
        if s.is_empty() {
            return Err(ArithError::InvalidIntForm("empty string".to_string()));
        }
        let bytes = s.as_bytes();
        let (neg, digits) = if bytes[0] == b'-' {
            (true, &bytes[1..])
        } else {
            (false, bytes)
        };
        if digits.is_empty() {
            return Err(ArithError::InvalidIntForm("bare minus sign".to_string()));
        }
        if !digits.iter().all(|b| b.is_ascii_digit()) {
            return Err(ArithError::InvalidIntForm(format!(
                "non-digit byte in {s:?}"
            )));
        }
        if digits.len() > 1 && digits[0] == b'0' {
            return Err(ArithError::InvalidIntForm(format!(
                "leading zero in {s:?}"
            )));
        }
        if neg && digits.len() == 1 && digits[0] == b'0' {
            return Err(ArithError::InvalidIntForm("negative zero '-0'".to_string()));
        }
        if digits.len() == 1 && digits[0] == b'0' {
            return Ok(Self::zero());
        }

        // Fast upper-bound rejection before allocation:
        // A d-digit positive decimal number is >= 10^(d-1) > 2^(3.3219*(d-1)).
        // Conservative lower bound on bits: (d - 1) * 33219 / 10000 + 1.
        let min_possible_bits = ((digits.len() - 1).saturating_mul(33_219)) / 10_000 + 1;
        if min_possible_bits > max_bits {
            return Err(ArithError::LimitsExceeded {
                bits: min_possible_bits,
                limit: max_bits,
            });
        }

        // Accumulate in base 10^9 chunks.
        let mut limbs: Vec<u32> = Vec::new();
        let mut i = 0;
        while i < digits.len() {
            let chunk_end = (i + 9).min(digits.len());
            let mut chunk_val = 0_u32;
            let mut chunk_mul = 1_u32;
            for &b in &digits[i..chunk_end] {
                chunk_val = chunk_val * 10 + u32::from(b - b'0');
                chunk_mul *= 10;
            }
            mul_add_small(&mut limbs, chunk_mul, chunk_val);
            let bits = mag_bit_len(&limbs);
            if bits > max_bits {
                return Err(ArithError::LimitsExceeded {
                    bits,
                    limit: max_bits,
                });
            }
            i = chunk_end;
        }
        trim_limbs(&mut limbs);
        Ok(Self {
            neg: neg && !limbs.is_empty(),
            limbs,
        })
    }

    /// Format as canonical decimal string (no leading zeros, no `-0`).
    pub fn to_decimal_string(&self) -> String {
        if self.limbs.is_empty() {
            return "0".to_string();
        }
        let mut cur = self.limbs.clone();
        let mut chunks: Vec<u32> = Vec::new();
        const BASE_10_9: u32 = 1_000_000_000;
        while !cur.is_empty() {
            let rem = div_rem_small_in_place(&mut cur, BASE_10_9);
            chunks.push(rem);
        }
        let mut out = String::new();
        if self.neg {
            out.push('-');
        }
        if let Some(&top) = chunks.last() {
            out.push_str(&top.to_string());
        }
        for &chunk in chunks.iter().rev().skip(1) {
            out.push_str(&format!("{chunk:09}"));
        }
        out
    }

    /// Exact addition with default compiled bit limit.
    pub fn add(&self, rhs: &Self) -> Result<Self, ArithError> {
        self.add_with_limit(rhs, COMPILED_LIMITS.max_bigint_bits)
    }

    pub fn add_with_limit(&self, rhs: &Self, max_bits: usize) -> Result<Self, ArithError> {
        self.check_limit(max_bits)?;
        rhs.check_limit(max_bits)?;
        let res = if self.neg == rhs.neg {
            let limbs = add_mag(&self.limbs, &rhs.limbs);
            Self::from_parts_unchecked(self.neg, limbs)
        } else {
            match cmp_mag(&self.limbs, &rhs.limbs) {
                Ordering::Equal => Self::zero(),
                Ordering::Greater => {
                    let limbs = sub_mag(&self.limbs, &rhs.limbs);
                    Self::from_parts_unchecked(self.neg, limbs)
                }
                Ordering::Less => {
                    let limbs = sub_mag(&rhs.limbs, &self.limbs);
                    Self::from_parts_unchecked(rhs.neg, limbs)
                }
            }
        };
        res.check_limit(max_bits)?;
        Ok(res)
    }

    /// Exact subtraction with default compiled bit limit.
    pub fn sub(&self, rhs: &Self) -> Result<Self, ArithError> {
        self.sub_with_limit(rhs, COMPILED_LIMITS.max_bigint_bits)
    }

    pub fn sub_with_limit(&self, rhs: &Self, max_bits: usize) -> Result<Self, ArithError> {
        self.add_with_limit(&rhs.negate(), max_bits)
    }

    /// Exact schoolbook multiplication enforcing `COMPILED_LIMITS.max_bigint_bits`.
    pub fn mul(&self, rhs: &Self) -> Result<Self, ArithError> {
        self.mul_with_limit(rhs, COMPILED_LIMITS.max_bigint_bits)
    }

    pub fn mul_with_limit(&self, rhs: &Self, max_bits: usize) -> Result<Self, ArithError> {
        self.check_limit(max_bits)?;
        rhs.check_limit(max_bits)?;
        if self.is_zero() || rhs.is_zero() {
            return Ok(Self::zero());
        }
        // Lower bound on product bit length: bit_len(a) + bit_len(b) - 1.
        // Reject before allocation if product is guaranteed to exceed max_bits.
        let min_bits = self.bit_len() + rhs.bit_len() - 1;
        if min_bits > max_bits {
            return Err(ArithError::LimitsExceeded {
                bits: min_bits,
                limit: max_bits,
            });
        }
        let limbs = mul_mag(&self.limbs, &rhs.limbs);
        let res = Self::from_parts_unchecked(self.neg ^ rhs.neg, limbs);
        res.check_limit(max_bits)?;
        Ok(res)
    }

    /// Crate-internal unchecked multiplication (used by exact cross-multiplication
    /// in `Rational::cmp` and exact interval scaling so comparisons never fail).
    pub(crate) fn mul_unchecked(&self, rhs: &Self) -> Self {
        if self.is_zero() || rhs.is_zero() {
            return Self::zero();
        }
        let limbs = mul_mag(&self.limbs, &rhs.limbs);
        Self::from_parts_unchecked(self.neg ^ rhs.neg, limbs)
    }

    /// Euclidean division: returns `(q, r)` such that `self = q * rhs + r`
    /// and `0 <= r < |rhs|` (BD-026 `bigint_divrem_identity`).
    pub fn div_rem_euclid(&self, rhs: &Self) -> Result<(Self, Self), ArithError> {
        self.div_rem_euclid_with_limit(rhs, COMPILED_LIMITS.max_bigint_bits)
    }

    pub fn div_rem_euclid_with_limit(
        &self,
        rhs: &Self,
        max_bits: usize,
    ) -> Result<(Self, Self), ArithError> {
        self.check_limit(max_bits)?;
        rhs.check_limit(max_bits)?;
        if rhs.is_zero() {
            return Err(ArithError::DivisionByZero);
        }
        if self.is_zero() {
            return Ok((Self::zero(), Self::zero()));
        }
        let (q_mag, r_mag) = div_rem_mag(&self.limbs, &rhs.limbs);
        let q_trunc = Self::from_parts_unchecked(self.neg ^ rhs.neg, q_mag);
        let r_trunc = Self::from_parts_unchecked(self.neg, r_mag);

        let (q, r) = if r_trunc.is_negative() {
            // Adjust so 0 <= r < |rhs|:
            // If rhs > 0: q = q_trunc - 1, r = r_trunc + rhs
            // If rhs < 0: q = q_trunc + 1, r = r_trunc - rhs (= r_trunc + |rhs|)
            if rhs.is_positive() {
                (
                    q_trunc.sub_with_limit(&Self::one(), max_bits)?,
                    r_trunc.add_with_limit(rhs, max_bits)?,
                )
            } else {
                (
                    q_trunc.add_with_limit(&Self::one(), max_bits)?,
                    r_trunc.sub_with_limit(rhs, max_bits)?,
                )
            }
        } else {
            (q_trunc, r_trunc)
        };
        q.check_limit(max_bits)?;
        r.check_limit(max_bits)?;
        Ok((q, r))
    }

    /// Truncating division towards zero: `self = q * rhs + r`, `r` has sign of `self` (or 0).
    pub fn div_rem_trunc(&self, rhs: &Self) -> Result<(Self, Self), ArithError> {
        self.check_limit(COMPILED_LIMITS.max_bigint_bits)?;
        rhs.check_limit(COMPILED_LIMITS.max_bigint_bits)?;
        if rhs.is_zero() {
            return Err(ArithError::DivisionByZero);
        }
        if self.is_zero() {
            return Ok((Self::zero(), Self::zero()));
        }
        let (q_mag, r_mag) = div_rem_mag(&self.limbs, &rhs.limbs);
        let q = Self::from_parts(self.neg ^ rhs.neg, q_mag)?;
        let r = Self::from_parts(self.neg, r_mag)?;
        Ok((q, r))
    }

    /// Floor division `floor(self / rhs)` for `rhs > 0` (also works for `rhs < 0`).
    pub fn div_floor(&self, rhs: &Self) -> Result<Self, ArithError> {
        if rhs.is_zero() {
            return Err(ArithError::DivisionByZero);
        }
        let (q_mag, r_mag) = div_rem_mag(&self.limbs, &rhs.limbs);
        let q_trunc = Self::from_parts_unchecked(self.neg ^ rhs.neg, q_mag);
        let has_rem = !r_mag.is_empty();
        let res = if has_rem && (self.neg ^ rhs.neg) {
            q_trunc.sub(&Self::one())?
        } else {
            q_trunc
        };
        res.check_limit(COMPILED_LIMITS.max_bigint_bits)?;
        Ok(res)
    }

    /// Ceiling division `ceil(self / rhs)` for `rhs != 0`.
    pub fn div_ceil(&self, rhs: &Self) -> Result<Self, ArithError> {
        if rhs.is_zero() {
            return Err(ArithError::DivisionByZero);
        }
        let (q_mag, r_mag) = div_rem_mag(&self.limbs, &rhs.limbs);
        let q_trunc = Self::from_parts_unchecked(self.neg ^ rhs.neg, q_mag);
        let has_rem = !r_mag.is_empty();
        let res = if has_rem && !(self.neg ^ rhs.neg) {
            q_trunc.add(&Self::one())?
        } else {
            q_trunc
        };
        res.check_limit(COMPILED_LIMITS.max_bigint_bits)?;
        Ok(res)
    }

    /// Non-negative greatest common divisor `gcd(|self|, |rhs|)`.
    pub fn gcd(&self, rhs: &Self) -> Result<Self, ArithError> {
        self.check_limit(COMPILED_LIMITS.max_bigint_bits)?;
        rhs.check_limit(COMPILED_LIMITS.max_bigint_bits)?;
        let mut a = self.limbs.clone();
        let mut b = rhs.limbs.clone();
        while !b.is_empty() {
            let (_, r) = div_rem_mag(&a, &b);
            a = b;
            b = r;
        }
        Self::from_parts(false, a)
    }

    /// Shift left by `bits` (multiply by `2^bits`), checking bit limit before allocating.
    pub fn shl_bits(&self, bits: usize) -> Result<Self, ArithError> {
        self.shl_bits_with_limit(bits, COMPILED_LIMITS.max_bigint_bits)
    }

    pub fn shl_bits_with_limit(&self, bits: usize, max_bits: usize) -> Result<Self, ArithError> {
        self.check_limit(max_bits)?;
        if self.is_zero() || bits == 0 {
            return Ok(self.clone());
        }
        let new_bits = self.bit_len().saturating_add(bits);
        if new_bits > max_bits {
            return Err(ArithError::LimitsExceeded {
                bits: new_bits,
                limit: max_bits,
            });
        }
        let shifted = shl_mag(&self.limbs, bits);
        Ok(Self::from_parts_unchecked(self.neg, shifted))
    }

    /// Crate-internal shift left without bit-limit failure (used inside exact interval steps
    /// whose final rounded result is checked against `COMPILED_LIMITS.max_bigint_bits`).
    pub(crate) fn shl_bits_unchecked(&self, bits: usize) -> Self {
        if self.is_zero() || bits == 0 {
            return self.clone();
        }
        Self::from_parts_unchecked(self.neg, shl_mag(&self.limbs, bits))
    }

    /// Exact `floor(self / 2^bits)`.
    pub fn shr_floor_bits(&self, bits: usize) -> Self {
        if self.is_zero() || bits == 0 {
            return self.clone();
        }
        let (q_mag, had_nonzero_low) = shr_mag_with_sticky(&self.limbs, bits);
        if !self.neg {
            Self::from_parts_unchecked(false, q_mag)
        } else if had_nonzero_low {
            let one = [1_u32];
            Self::from_parts_unchecked(true, add_mag(&q_mag, &one))
        } else {
            Self::from_parts_unchecked(true, q_mag)
        }
    }

    /// Exact `ceil(self / 2^bits)`.
    pub fn shr_ceil_bits(&self, bits: usize) -> Self {
        if self.is_zero() || bits == 0 {
            return self.clone();
        }
        let (q_mag, had_nonzero_low) = shr_mag_with_sticky(&self.limbs, bits);
        if self.neg {
            Self::from_parts_unchecked(true, q_mag)
        } else if had_nonzero_low {
            let one = [1_u32];
            Self::from_parts_unchecked(false, add_mag(&q_mag, &one))
        } else {
            Self::from_parts_unchecked(false, q_mag)
        }
    }

    /// Exact integer exponentiation `self^exp` enforcing `COMPILED_LIMITS.max_bigint_bits`.
    pub fn pow_u32(&self, mut exp: u32) -> Result<Self, ArithError> {
        self.check_limit(COMPILED_LIMITS.max_bigint_bits)?;
        if exp == 0 {
            return Ok(Self::one());
        }
        if self.is_zero() {
            return Ok(Self::zero());
        }
        if self.is_one() {
            return Ok(Self::one());
        }
        if self.neg && self.limbs.len() == 1 && self.limbs[0] == 1 {
            return Ok(if exp & 1 == 0 {
                Self::one()
            } else {
                self.clone()
            });
        }
        // Quick lower bound: |self| >= 2 => bit_len >= 2 => |self|^exp has >= (bit_len - 1)*exp + 1 bits.
        let min_bits = (self.bit_len() - 1)
            .saturating_mul(exp as usize)
            .saturating_add(1);
        if min_bits > COMPILED_LIMITS.max_bigint_bits {
            return Err(ArithError::LimitsExceeded {
                bits: min_bits,
                limit: COMPILED_LIMITS.max_bigint_bits,
            });
        }
        let mut base = self.clone();
        let mut acc = Self::one();
        while exp > 0 {
            if exp & 1 == 1 {
                acc = acc.mul(&base)?;
            }
            exp >>= 1;
            if exp > 0 {
                base = base.mul(&base)?;
            }
        }
        Ok(acc)
    }

    /// Exact integer square root `floor(sqrt(self))` for `self >= 0`.
    pub fn isqrt_floor(&self) -> Result<Self, ArithError> {
        if self.is_negative() {
            return Err(ArithError::DomainError(
                "square root of negative integer".to_string(),
            ));
        }
        if self.is_zero() {
            return Ok(Self::zero());
        }
        let s_limbs = isqrt_mag(&self.limbs);
        Self::from_parts(false, s_limbs)
    }

    /// Exact ceiling integer square root `ceil(sqrt(self))` for `self >= 0`.
    pub fn isqrt_ceil(&self) -> Result<Self, ArithError> {
        let floor = self.isqrt_floor()?;
        let sq = floor.mul_unchecked(&floor);
        if &sq == self {
            Ok(floor)
        } else {
            floor.add(&Self::one())
        }
    }

    /// Crate-internal unbounded magnitude division (for exact interval endpoints
    /// before outward rounding back to `COMPILED_LIMITS.max_bigint_bits`).
    pub(crate) fn div_floor_unchecked(&self, rhs: &Self) -> Self {
        let (q_mag, r_mag) = div_rem_mag(&self.limbs, &rhs.limbs);
        let q_trunc = Self::from_parts_unchecked(self.neg ^ rhs.neg, q_mag);
        if !r_mag.is_empty() && (self.neg ^ rhs.neg) {
            let one = Self::one();
            q_trunc.sub_unchecked(&one)
        } else {
            q_trunc
        }
    }

    pub(crate) fn div_ceil_unchecked(&self, rhs: &Self) -> Self {
        let (q_mag, r_mag) = div_rem_mag(&self.limbs, &rhs.limbs);
        let q_trunc = Self::from_parts_unchecked(self.neg ^ rhs.neg, q_mag);
        if !r_mag.is_empty() && !(self.neg ^ rhs.neg) {
            let one = Self::one();
            q_trunc.add_unchecked(&one)
        } else {
            q_trunc
        }
    }

    pub(crate) fn add_unchecked(&self, rhs: &Self) -> Self {
        if self.neg == rhs.neg {
            Self::from_parts_unchecked(self.neg, add_mag(&self.limbs, &rhs.limbs))
        } else {
            match cmp_mag(&self.limbs, &rhs.limbs) {
                Ordering::Equal => Self::zero(),
                Ordering::Greater => {
                    Self::from_parts_unchecked(self.neg, sub_mag(&self.limbs, &rhs.limbs))
                }
                Ordering::Less => {
                    Self::from_parts_unchecked(rhs.neg, sub_mag(&rhs.limbs, &self.limbs))
                }
            }
        }
    }

    pub(crate) fn sub_unchecked(&self, rhs: &Self) -> Self {
        self.add_unchecked(&rhs.negate())
    }

    pub(crate) fn isqrt_floor_unchecked(&self) -> Self {
        debug_assert!(!self.is_negative());
        if self.is_zero() {
            Self::zero()
        } else {
            Self::from_parts_unchecked(false, isqrt_mag(&self.limbs))
        }
    }

    pub(crate) fn isqrt_ceil_unchecked(&self) -> Self {
        let floor = self.isqrt_floor_unchecked();
        let sq = floor.mul_unchecked(&floor);
        if &sq == self {
            floor
        } else {
            floor.add_unchecked(&Self::one())
        }
    }
}

impl Ord for BigInt {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.is_negative(), other.is_negative()) {
            (true, false) => Ordering::Less,
            (false, true) => Ordering::Greater,
            (false, false) => cmp_mag(&self.limbs, &other.limbs),
            (true, true) => cmp_mag(&other.limbs, &self.limbs),
        }
    }
}

impl PartialOrd for BigInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for BigInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_decimal_string())
    }
}

// ---------------------------------------------------------------------------
// Low-level little-endian unsigned u32 limb kernels
// ---------------------------------------------------------------------------

fn trim_limbs(limbs: &mut Vec<u32>) {
    while limbs.last() == Some(&0) {
        limbs.pop();
    }
}

fn mag_bit_len(limbs: &[u32]) -> usize {
    match limbs.last() {
        None => 0,
        Some(&top) => (limbs.len() - 1) * 32 + (32 - top.leading_zeros() as usize),
    }
}

fn cmp_mag(a: &[u32], b: &[u32]) -> Ordering {
    match a.len().cmp(&b.len()) {
        Ordering::Equal => {
            for (&x, &y) in a.iter().rev().zip(b.iter().rev()) {
                match x.cmp(&y) {
                    Ordering::Equal => continue,
                    non_eq => return non_eq,
                }
            }
            Ordering::Equal
        }
        non_eq => non_eq,
    }
}

fn add_mag(a: &[u32], b: &[u32]) -> Vec<u32> {
    let n = a.len().max(b.len());
    let mut out = Vec::with_capacity(n + 1);
    let mut carry = 0_u64;
    for i in 0..n {
        let av = u64::from(*a.get(i).unwrap_or(&0));
        let bv = u64::from(*b.get(i).unwrap_or(&0));
        let sum = av + bv + carry;
        out.push(sum as u32);
        carry = sum >> 32;
    }
    if carry != 0 {
        out.push(carry as u32);
    }
    trim_limbs(&mut out);
    out
}

/// Requires `a >= b`.
fn sub_mag(a: &[u32], b: &[u32]) -> Vec<u32> {
    let mut out = Vec::with_capacity(a.len());
    let mut borrow = 0_i64;
    for i in 0..a.len() {
        let av = i64::from(a[i]);
        let bv = i64::from(*b.get(i).unwrap_or(&0));
        let mut diff = av - bv - borrow;
        if diff < 0 {
            diff += 1_i64 << 32;
            borrow = 1;
        } else {
            borrow = 0;
        }
        out.push(diff as u32);
    }
    debug_assert_eq!(borrow, 0);
    trim_limbs(&mut out);
    out
}

fn mul_add_small(limbs: &mut Vec<u32>, mul: u32, add: u32) {
    let m = u64::from(mul);
    let mut carry = u64::from(add);
    for limb in limbs.iter_mut() {
        let prod = u64::from(*limb) * m + carry;
        *limb = prod as u32;
        carry = prod >> 32;
    }
    if carry != 0 {
        limbs.push(carry as u32);
    }
    trim_limbs(limbs);
}

fn div_rem_small_in_place(limbs: &mut Vec<u32>, divisor: u32) -> u32 {
    let d = u64::from(divisor);
    let mut rem = 0_u64;
    for limb in limbs.iter_mut().rev() {
        let cur = (rem << 32) | u64::from(*limb);
        *limb = (cur / d) as u32;
        rem = cur % d;
    }
    trim_limbs(limbs);
    rem as u32
}

fn mul_mag(a: &[u32], b: &[u32]) -> Vec<u32> {
    if a.is_empty() || b.is_empty() {
        return Vec::new();
    }
    let mut out = vec![0_u32; a.len() + b.len()];
    for (i, &ai) in a.iter().enumerate() {
        let ai64 = u64::from(ai);
        if ai64 == 0 {
            continue;
        }
        let mut carry = 0_u64;
        for (j, &bj) in b.iter().enumerate() {
            let cur = u64::from(out[i + j]) + ai64 * u64::from(bj) + carry;
            out[i + j] = cur as u32;
            carry = cur >> 32;
        }
        out[i + b.len()] = carry as u32;
    }
    trim_limbs(&mut out);
    out
}

fn shl_mag(a: &[u32], bits: usize) -> Vec<u32> {
    if a.is_empty() {
        return Vec::new();
    }
    let word_shift = bits / 32;
    let bit_shift = (bits % 32) as u32;
    let mut out = vec![0_u32; word_shift + a.len() + usize::from(bit_shift > 0)];
    if bit_shift == 0 {
        out[word_shift..word_shift + a.len()].copy_from_slice(a);
    } else {
        let mut carry = 0_u32;
        for (i, &limb) in a.iter().enumerate() {
            out[word_shift + i] = (limb << bit_shift) | carry;
            carry = limb >> (32 - bit_shift);
        }
        if carry != 0 {
            out[word_shift + a.len()] = carry;
        }
    }
    trim_limbs(&mut out);
    out
}

fn shr_mag_with_sticky(a: &[u32], bits: usize) -> (Vec<u32>, bool) {
    if a.is_empty() {
        return (Vec::new(), false);
    }
    let word_shift = bits / 32;
    let bit_shift = (bits % 32) as u32;
    if word_shift >= a.len() {
        return (Vec::new(), true);
    }
    let mut sticky = a[..word_shift].iter().any(|&w| w != 0);
    if bit_shift > 0 {
        let low_mask = (1_u32 << bit_shift) - 1;
        if (a[word_shift] & low_mask) != 0 {
            sticky = true;
        }
    }
    let rem_len = a.len() - word_shift;
    let mut out = vec![0_u32; rem_len];
    if bit_shift == 0 {
        out.copy_from_slice(&a[word_shift..]);
    } else {
        for i in 0..rem_len {
            let lo = a[word_shift + i] >> bit_shift;
            let hi = if word_shift + i + 1 < a.len() {
                a[word_shift + i + 1] << (32 - bit_shift)
            } else {
                0
            };
            out[i] = lo | hi;
        }
    }
    trim_limbs(&mut out);
    (out, sticky)
}

/// Knuth's Algorithm D (TAOCP Vol. 2, §4.3.1) for unsigned base-`2^32` division.
/// Requires `!v.is_empty()`. Returns `(q, r)` with `u = q * v + r` and `0 <= r < v`.
fn div_rem_mag(u: &[u32], v: &[u32]) -> (Vec<u32>, Vec<u32>) {
    debug_assert!(!v.is_empty());
    match cmp_mag(u, v) {
        Ordering::Less => return (Vec::new(), u.to_vec()),
        Ordering::Equal => return (vec![1], Vec::new()),
        Ordering::Greater => {}
    }

    let n = v.len();
    if n == 1 {
        let mut q = u.to_vec();
        let rem = div_rem_small_in_place(&mut q, v[0]);
        let r = if rem == 0 { Vec::new() } else { vec![rem] };
        return (q, r);
    }

    const BASE: u64 = 1_u64 << 32;
    let m = u.len() - n;

    // D1. Normalize: shift left by `s` so MSB of vn[n-1] is set.
    let s = v[n - 1].leading_zeros();
    let vn = if s == 0 {
        v.to_vec()
    } else {
        shl_mag(v, s as usize)
    };
    debug_assert_eq!(vn.len(), n);
    debug_assert!(vn[n - 1] >= (1_u32 << 31));

    let mut un = if s == 0 {
        let mut buf = Vec::with_capacity(u.len() + 1);
        buf.extend_from_slice(u);
        buf
    } else {
        shl_mag(u, s as usize)
    };
    // Ensure un has length m + n + 1 (leading 0 limb if normalization did not carry).
    un.resize(m + n + 1, 0);

    let mut q = vec![0_u32; m + 1];
    let vn1 = u64::from(vn[n - 1]);
    let vn2 = u64::from(vn[n - 2]);

    // D2..D7. Main loop: j from m down to 0.
    for j in (0..=m).rev() {
        // D3. Calculate trial quotient qhat and remainder rhat.
        let num = (u64::from(un[j + n]) << 32) | u64::from(un[j + n - 1]);
        let mut qhat = num / vn1;
        let mut rhat = num % vn1;
        if qhat >= BASE {
            qhat = BASE - 1;
            rhat = num - qhat * vn1;
        }
        while rhat < BASE && qhat * vn2 > (rhat << 32) | u64::from(un[j + n - 2]) {
            qhat -= 1;
            rhat += vn1;
        }

        // D4. Multiply and subtract: un[j..=j+n] -= qhat * vn[0..n].
        let mut borrow = 0_i128;
        for i in 0..n {
            let p = u128::from(qhat) * u128::from(vn[i]);
            let t = i128::from(un[j + i]) - borrow - (p as u32 as i128);
            un[j + i] = t as u32;
            borrow = ((p >> 32) as i128) - (t >> 32);
        }
        let t = i128::from(un[j + n]) - borrow;
        un[j + n] = t as u32;

        // D5/D6. If subtraction went negative, decrement qhat and add back vn.
        if t < 0 {
            qhat -= 1;
            let mut carry = 0_u64;
            for i in 0..n {
                let sum = u64::from(un[j + i]) + u64::from(vn[i]) + carry;
                un[j + i] = sum as u32;
                carry = sum >> 32;
            }
            un[j + n] = un[j + n].wrapping_add(carry as u32);
        }
        q[j] = qhat as u32;
    }

    // D8. Unnormalize remainder.
    un.truncate(n);
    trim_limbs(&mut un);
    let r = if s == 0 {
        un
    } else {
        shr_mag_with_sticky(&un, s as usize).0
    };
    trim_limbs(&mut q);
    (q, r)
}

/// Exact integer square root `floor(sqrt(n))` via Newton's method on unsigned limbs.
fn isqrt_mag(n: &[u32]) -> Vec<u32> {
    if n.is_empty() {
        return Vec::new();
    }
    let bits = mag_bit_len(n);
    // Initial overestimate strictly >= floor(sqrt(n)): 2^ceil(bits / 2).
    let init_shift = bits.div_ceil(2);
    let mut x = shl_mag(&[1], init_shift);
    loop {
        let (q, _) = div_rem_mag(n, &x);
        let sum = add_mag(&x, &q);
        let (next, _) = shr_mag_with_sticky(&sum, 1);
        if cmp_mag(&next, &x) != Ordering::Less {
            return x;
        }
        x = next;
    }
}
