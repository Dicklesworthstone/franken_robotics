//! Exact integer, exact decimal, and shortest round-trip binary64 encodings for
//! `frob_core::canon` (bead N34-18).
//!
//! Resolves N34-18 review note 1: `frob-core` (layer 0) owns a self-contained,
//! std-only arbitrary-precision signed integer [`CanonInt`] (aliased as [`Int`])
//! using little-endian 32-bit limbs compatible with `frob-lie`'s limb scheme,
//! with exact binary GCD for rational reduction checks and zero silent reduction.
//!
//! Independent from `frob-cert`; std-only; prints nothing (L-7); zero `unsafe`; determinism D0.

use std::cmp::Ordering;
use std::fmt::{self, Write as _};
use std::str::FromStr;

use super::writer::CanonError;

/// Maximum magnitude of exponent allowed when normalizing an [`ExactDecimal`] string.
const MAX_EXACT_DECIMAL_EXP: i32 = 4096;

/// Arbitrary-precision signed integer for canonical rational serialization.
///
/// Stored as a sign flag and little-endian base-`2^32` limbs with no trailing zero limbs
/// (and canonical zero represented by `negative = false`, `limbs = []`).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CanonInt {
    negative: bool,
    limbs: Vec<u32>,
}

/// Alias matching the N34-18 specification sketch (`rational_pair(p: &Int, q: &Int)`).
pub type Int = CanonInt;

impl CanonInt {
    /// Construct canonical integer `0`.
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            negative: false,
            limbs: Vec::new(),
        }
    }

    /// Construct canonical integer `1`.
    #[must_use]
    pub fn one() -> Self {
        Self {
            negative: false,
            limbs: vec![1],
        }
    }

    /// Construct from a signed 64-bit integer.
    #[must_use]
    pub fn from_i64(value: i64) -> Self {
        let negative = value < 0;
        let mag = value.unsigned_abs();
        Self::from_sign_and_u64(negative, mag)
    }

    /// Construct from an unsigned 64-bit integer.
    #[must_use]
    pub fn from_u64(value: u64) -> Self {
        Self::from_sign_and_u64(false, value)
    }

    /// Construct from a signed 128-bit integer.
    #[must_use]
    pub fn from_i128(value: i128) -> Self {
        let negative = value < 0;
        let mag = value.unsigned_abs();
        Self::from_sign_and_u128(negative, mag)
    }

    /// Construct from an unsigned 128-bit integer.
    #[must_use]
    pub fn from_u128(value: u128) -> Self {
        Self::from_sign_and_u128(false, value)
    }

    /// Construct from a sign flag and little-endian 32-bit magnitude limbs.
    ///
    /// Trailing zero limbs are stripped and zero is always normalized to non-negative.
    #[must_use]
    pub fn from_le_limbs(negative: bool, mut limbs: Vec<u32>) -> Self {
        trim_limbs(&mut limbs);
        Self {
            negative: negative && !limbs.is_empty(),
            limbs,
        }
    }

    /// Parse a strict canonical signed decimal integer matching `^(0|-?[1-9][0-9]*)$`.
    pub fn parse_decimal(s: &str) -> Result<Self, CanonError> {
        if s.is_empty() {
            return Err(CanonError::InvalidInteger {
                input: s.to_owned(),
                reason: "empty integer string",
            });
        }
        if s.starts_with('+') {
            return Err(CanonError::InvalidInteger {
                input: s.to_owned(),
                reason: "leading '+' is forbidden",
            });
        }
        let (negative, digits) = s.strip_prefix('-').map_or((false, s), |rest| (true, rest));
        if digits.is_empty() {
            return Err(CanonError::InvalidInteger {
                input: s.to_owned(),
                reason: "missing digits after '-'",
            });
        }
        if !digits.bytes().all(|b| b.is_ascii_digit()) {
            return Err(CanonError::InvalidInteger {
                input: s.to_owned(),
                reason: "integer contains non-ASCII-digit characters",
            });
        }
        if digits == "0" {
            if negative {
                return Err(CanonError::InvalidInteger {
                    input: s.to_owned(),
                    reason: "'-0' is not a canonical integer",
                });
            }
            return Ok(Self::zero());
        }
        if digits.starts_with('0') {
            return Err(CanonError::InvalidInteger {
                input: s.to_owned(),
                reason: "leading zeros are forbidden",
            });
        }

        let mut limbs = Vec::with_capacity((digits.len() / 9) + 1);
        for chunk in digits.as_bytes().chunks(9) {
            let mut val = 0u32;
            let mut scale = 1u32;
            for &b in chunk {
                val = val * 10 + u32::from(b - b'0');
                scale *= 10;
            }
            mul_add_u32(&mut limbs, scale, val);
        }
        trim_limbs(&mut limbs);
        Ok(Self {
            negative: negative && !limbs.is_empty(),
            limbs,
        })
    }

    /// Return `true` if this integer equals `0`.
    #[must_use]
    pub fn is_zero(&self) -> bool {
        self.limbs.is_empty()
    }

    /// Return `true` if this integer is strictly greater than `0`.
    #[must_use]
    pub fn is_positive(&self) -> bool {
        !self.negative && !self.limbs.is_empty()
    }

    /// Return `true` if this integer is strictly less than `0`.
    #[must_use]
    pub fn is_negative(&self) -> bool {
        self.negative && !self.limbs.is_empty()
    }

    /// Return `true` if this integer equals `+1`.
    #[must_use]
    pub fn is_one(&self) -> bool {
        !self.negative && self.limbs.as_slice() == [1]
    }

    /// Borrow the normalized little-endian base-`2^32` magnitude limbs.
    #[must_use]
    pub fn magnitude_limbs(&self) -> &[u32] {
        &self.limbs
    }

    /// Return `true` iff `gcd(|self|, |other|) == 1` (computed exactly via Stein's binary GCD).
    #[must_use]
    pub fn is_coprime_with(&self, other: &Self) -> bool {
        if self.limbs.is_empty() {
            return other.limbs.as_slice() == [1];
        }
        if other.limbs.is_empty() {
            return self.limbs.as_slice() == [1];
        }
        if (self.limbs[0] & 1) == 0 && (other.limbs[0] & 1) == 0 {
            return false;
        }

        let mut a = self.limbs.clone();
        let mut b = other.limbs.clone();
        let tz_a = trailing_zeros_mag(&a);
        shr_mag(&mut a, tz_a);
        let tz_b = trailing_zeros_mag(&b);
        shr_mag(&mut b, tz_b);

        loop {
            match cmp_mag(&a, &b) {
                Ordering::Equal => return a.as_slice() == [1],
                Ordering::Greater => {
                    sub_mag_in_place(&mut a, &b);
                    let tz = trailing_zeros_mag(&a);
                    shr_mag(&mut a, tz);
                }
                Ordering::Less => {
                    sub_mag_in_place(&mut b, &a);
                    let tz = trailing_zeros_mag(&b);
                    shr_mag(&mut b, tz);
                }
            }
        }
    }

    /// Render this integer as a canonical base-10 string matching `^(0|-?[1-9][0-9]*)$`.
    #[must_use]
    pub fn to_decimal_string(&self) -> String {
        if self.limbs.is_empty() {
            return String::from("0");
        }
        let mut mag = self.limbs.clone();
        let mut chunks: Vec<u32> = Vec::with_capacity(mag.len() + 1);
        while !mag.is_empty() {
            let rem = div_rem_u32(&mut mag, 1_000_000_000);
            chunks.push(rem);
            trim_limbs(&mut mag);
        }

        let mut out = String::with_capacity(chunks.len() * 9 + usize::from(self.negative));
        if self.negative {
            out.push('-');
        }
        if let Some(top) = chunks.pop() {
            let _ = write!(out, "{top}");
            for chunk in chunks.into_iter().rev() {
                let _ = write!(out, "{chunk:09}");
            }
        }
        out
    }

    fn from_sign_and_u64(negative: bool, mag: u64) -> Self {
        if mag == 0 {
            return Self::zero();
        }
        let lo = mag as u32;
        let hi = (mag >> 32) as u32;
        let limbs = if hi == 0 { vec![lo] } else { vec![lo, hi] };
        Self { negative, limbs }
    }

    fn from_sign_and_u128(negative: bool, mut mag: u128) -> Self {
        if mag == 0 {
            return Self::zero();
        }
        let mut limbs = Vec::with_capacity(4);
        while mag != 0 {
            limbs.push(mag as u32);
            mag >>= 32;
        }
        Self { negative, limbs }
    }
}

impl Ord for CanonInt {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.negative, other.negative) {
            (true, false) => Ordering::Less,
            (false, true) => Ordering::Greater,
            (false, false) => cmp_mag(&self.limbs, &other.limbs),
            (true, true) => cmp_mag(&other.limbs, &self.limbs),
        }
    }
}

impl PartialOrd for CanonInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for CanonInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_decimal_string())
    }
}

impl FromStr for CanonInt {
    type Err = CanonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse_decimal(s)
    }
}

impl From<i64> for CanonInt {
    fn from(value: i64) -> Self {
        Self::from_i64(value)
    }
}

impl From<u64> for CanonInt {
    fn from(value: u64) -> Self {
        Self::from_u64(value)
    }
}

impl From<i128> for CanonInt {
    fn from(value: i128) -> Self {
        Self::from_i128(value)
    }
}

impl From<u128> for CanonInt {
    fn from(value: u128) -> Self {
        Self::from_u128(value)
    }
}

fn trim_limbs(limbs: &mut Vec<u32>) {
    while limbs.last() == Some(&0) {
        limbs.pop();
    }
}

fn cmp_mag(a: &[u32], b: &[u32]) -> Ordering {
    a.len()
        .cmp(&b.len())
        .then_with(|| a.iter().rev().cmp(b.iter().rev()))
}

fn trailing_zeros_mag(limbs: &[u32]) -> usize {
    for (idx, &limb) in limbs.iter().enumerate() {
        if limb != 0 {
            return idx * 32 + (limb.trailing_zeros() as usize);
        }
    }
    0
}

fn shr_mag(limbs: &mut Vec<u32>, bits: usize) {
    if bits == 0 || limbs.is_empty() {
        return;
    }
    let word_shift = bits / 32;
    let bit_shift = (bits % 32) as u32;
    if word_shift >= limbs.len() {
        limbs.clear();
        return;
    }
    if word_shift > 0 {
        limbs.drain(..word_shift);
    }
    if bit_shift > 0 {
        let inv = 32 - bit_shift;
        for i in 0..limbs.len() {
            let lo = limbs[i] >> bit_shift;
            let hi = if i + 1 < limbs.len() {
                limbs[i + 1] << inv
            } else {
                0
            };
            limbs[i] = lo | hi;
        }
    }
    trim_limbs(limbs);
}

fn sub_mag_in_place(a: &mut Vec<u32>, b: &[u32]) {
    let mut borrow = 0u64;
    for i in 0..a.len() {
        let av = u64::from(a[i]);
        let bv = if i < b.len() { u64::from(b[i]) } else { 0 };
        let sub = bv + borrow;
        if av >= sub {
            a[i] = (av - sub) as u32;
            borrow = 0;
        } else {
            a[i] = ((1u64 << 32) + av - sub) as u32;
            borrow = 1;
        }
    }
    debug_assert_eq!(borrow, 0);
    trim_limbs(a);
}

fn mul_add_u32(limbs: &mut Vec<u32>, mul: u32, add: u32) {
    let mut carry = u64::from(add);
    let m = u64::from(mul);
    for limb in limbs.iter_mut() {
        let prod = u64::from(*limb) * m + carry;
        *limb = prod as u32;
        carry = prod >> 32;
    }
    if carry != 0 {
        limbs.push(carry as u32);
    }
}

fn div_rem_u32(limbs: &mut [u32], divisor: u32) -> u32 {
    let d = u64::from(divisor);
    let mut rem = 0u64;
    for limb in limbs.iter_mut().rev() {
        let cur = (rem << 32) | u64::from(*limb);
        *limb = (cur / d) as u32;
        rem = cur % d;
    }
    rem as u32
}

/// Exact decimal value stored in canonical shortest plain decimal form.
///
/// Invariants guaranteed for every constructed value:
/// - Zero is always `"0"` (never `"-0"` or `"0.0"`).
/// - Integral values have no decimal point and no leading zeros (`"42"`, `"-7"`).
/// - Non-integral values have no leading zeros before the integer part (except `"0"`),
///   a decimal point, and no trailing fractional zeros (`"0.1"`, `"-12.375"`).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExactDecimal(String);

impl ExactDecimal {
    /// Parse an exact decimal lexeme (plain or bounded scientific `e`/`E`) and normalize it
    /// to shortest exact plain decimal form.
    ///
    /// Rejects empty strings, leading `+`, leading zeros in the integer part (e.g. `"01"`),
    /// missing digits around `.`, non-ASCII digits, or exponents outside `±4096`.
    pub fn parse(s: &str) -> Result<Self, CanonError> {
        let err = |reason: &'static str| CanonError::InvalidDecimal {
            input: s.to_owned(),
            reason,
        };
        if s.is_empty() {
            return Err(err("empty decimal string"));
        }
        if s.starts_with('+') {
            return Err(err("leading '+' is forbidden"));
        }
        let (negative, body) = s.strip_prefix('-').map_or((false, s), |rest| (true, rest));
        if body.is_empty() {
            return Err(err("missing digits after '-'"));
        }

        let (mantissa, exp) = match body.find(['e', 'E']) {
            Some(pos) => {
                let exp_str = &body[pos + 1..];
                if exp_str.is_empty() {
                    return Err(err("missing exponent digits"));
                }
                let (exp_neg, exp_digits) = if let Some(rest) = exp_str.strip_prefix('-') {
                    (true, rest)
                } else if let Some(rest) = exp_str.strip_prefix('+') {
                    (false, rest)
                } else {
                    (false, exp_str)
                };
                if exp_digits.is_empty() || !exp_digits.bytes().all(|b| b.is_ascii_digit()) {
                    return Err(err("invalid exponent digits"));
                }
                let abs_exp: i32 = exp_digits
                    .parse()
                    .map_err(|_| err("exponent out of range"))?;
                let signed_exp = if exp_neg { -abs_exp } else { abs_exp };
                if !(-MAX_EXACT_DECIMAL_EXP..=MAX_EXACT_DECIMAL_EXP).contains(&signed_exp) {
                    return Err(err("exponent exceeds ±4096 expansion cap"));
                }
                (&body[..pos], signed_exp)
            }
            None => (body, 0i32),
        };

        let (int_part, frac_part) = match mantissa.split_once('.') {
            Some((int_p, frac_p)) => {
                if int_p.is_empty() || frac_p.is_empty() {
                    return Err(err("decimal point requires digits on both sides"));
                }
                if frac_p.contains('.') {
                    return Err(err("multiple decimal points"));
                }
                (int_p, frac_p)
            }
            None => (mantissa, ""),
        };

        if int_part.is_empty() || !int_part.bytes().all(|b| b.is_ascii_digit()) {
            return Err(err("invalid integer digits"));
        }
        if int_part.len() > 1 && int_part.starts_with('0') {
            return Err(err("leading zeros in integer part are forbidden"));
        }
        if !frac_part.bytes().all(|b| b.is_ascii_digit()) {
            return Err(err("invalid fractional digits"));
        }

        let mut all_digits = Vec::with_capacity(int_part.len() + frac_part.len());
        all_digits.extend_from_slice(int_part.as_bytes());
        all_digits.extend_from_slice(frac_part.as_bytes());

        let mut dp =
            i64::try_from(int_part.len()).map_err(|_| err("mantissa too long"))? + i64::from(exp);

        let leading_zeros = all_digits.iter().take_while(|&&b| b == b'0').count();
        if leading_zeros == all_digits.len() {
            return Ok(Self(String::from("0")));
        }
        let trailing_zeros = all_digits.iter().rev().take_while(|&&b| b == b'0').count();
        let sig = &all_digits[leading_zeros..all_digits.len() - trailing_zeros];
        dp -= i64::try_from(leading_zeros).map_err(|_| err("mantissa too long"))?;

        let mut out = String::new();
        if negative {
            out.push('-');
        }
        if dp <= 0 {
            out.push_str("0.");
            let pad = usize::try_from(-dp).map_err(|_| err("exponent underflow"))?;
            for _ in 0..pad {
                out.push('0');
            }
            for &b in sig {
                out.push(char::from(b));
            }
        } else {
            let dp_usize = usize::try_from(dp).map_err(|_| err("exponent overflow"))?;
            if dp_usize >= sig.len() {
                for &b in sig {
                    out.push(char::from(b));
                }
                for _ in 0..(dp_usize - sig.len()) {
                    out.push('0');
                }
            } else {
                for &b in &sig[..dp_usize] {
                    out.push(char::from(b));
                }
                out.push('.');
                for &b in &sig[dp_usize..] {
                    out.push(char::from(b));
                }
            }
        }

        Ok(Self(out))
    }

    /// Validate that `s` is already in canonical shortest exact plain decimal form without
    /// normalizing it.
    pub fn parse_canonical(s: &str) -> Result<Self, CanonError> {
        let parsed = Self::parse(s)?;
        if parsed.0 != s {
            return Err(CanonError::InvalidDecimal {
                input: s.to_owned(),
                reason: "decimal is not in shortest canonical plain form",
            });
        }
        Ok(parsed)
    }

    /// Construct an exact decimal from a [`CanonInt`].
    #[must_use]
    pub fn from_int(n: &CanonInt) -> Self {
        Self(n.to_decimal_string())
    }

    /// Construct an exact decimal from a signed 64-bit integer.
    #[must_use]
    pub fn from_i64(n: i64) -> Self {
        Self(n.to_string())
    }

    /// Borrow the canonical shortest exact plain decimal text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ExactDecimal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl FromStr for ExactDecimal {
    type Err = CanonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

/// Fixed-capacity ASCII stack buffer implementing [`fmt::Write`] with zero heap allocation.
struct StackAscii<const N: usize> {
    buf: [u8; N],
    len: usize,
}

impl<const N: usize> StackAscii<N> {
    const fn new() -> Self {
        Self {
            buf: [0u8; N],
            len: 0,
        }
    }

    fn push_byte(&mut self, b: u8) -> Result<(), fmt::Error> {
        if self.len >= N {
            return Err(fmt::Error);
        }
        self.buf[self.len] = b;
        self.len += 1;
        Ok(())
    }

    fn push_bytes(&mut self, bytes: &[u8]) -> Result<(), fmt::Error> {
        if self.len + bytes.len() > N {
            return Err(fmt::Error);
        }
        self.buf[self.len..self.len + bytes.len()].copy_from_slice(bytes);
        self.len += bytes.len();
        Ok(())
    }

    fn as_bytes(&self) -> &[u8] {
        &self.buf[..self.len]
    }

    fn as_str(&self) -> Result<&str, CanonError> {
        std::str::from_utf8(self.as_bytes()).map_err(|_| CanonError::FloatRoundTripFailed)
    }
}

impl<const N: usize> fmt::Write for StackAscii<N> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.push_bytes(s.as_bytes())
    }
}

/// Format a finite binary64 using the shortest round-trip decimal digits laid out by
/// N34-18 Rule 4, verifying bit-for-bit round-trip by re-parsing before appending to `out`.
///
/// Layout rule:
/// - `NaN`, `+∞`, `-∞` → `Err(CanonError::NonFiniteFloat)`
/// - `+0.0` → `"0"`, `-0.0` → `"-0"`
/// - `10^-6 <= |x| < 10^21` (decimal exponent `-6 <= exp < 21`): plain notation, no `.0`
///   on integral values
/// - Otherwise: first digit, optional `.` + remaining digits, then `e+N` or `e-N`
pub(crate) fn format_f64_shortest_into(x: f64, out: &mut Vec<u8>) -> Result<(), CanonError> {
    if !x.is_finite() {
        return Err(CanonError::NonFiniteFloat);
    }
    if x == 0.0 {
        let token = if x.is_sign_negative() { "-0" } else { "0" };
        let parsed = token
            .parse::<f64>()
            .map_err(|_| CanonError::FloatRoundTripFailed)?;
        if parsed.to_bits() != x.to_bits() {
            return Err(CanonError::FloatRoundTripFailed);
        }
        out.extend_from_slice(token.as_bytes());
        return Ok(());
    }

    let negative = x.is_sign_negative();
    let abs = x.abs();

    // Rust's `LowerExp` (`{:e}`) formats finite positive f64 values using the shortest
    // round-trip decimal significand (Burger-Dybvig / Dragonbox, ties to even).
    let mut raw = StackAscii::<48>::new();
    write!(raw, "{abs:e}").map_err(|_| CanonError::FloatRoundTripFailed)?;
    let raw_str = raw.as_str()?;

    let (mantissa, exp_str) = raw_str
        .split_once('e')
        .ok_or(CanonError::FloatRoundTripFailed)?;
    let exp: i32 = exp_str
        .parse()
        .map_err(|_| CanonError::FloatRoundTripFailed)?;

    let mut digits = [0u8; 24];
    let mut n_digits = 0usize;
    for &b in mantissa.as_bytes() {
        if b == b'.' {
            continue;
        }
        if !b.is_ascii_digit() || n_digits >= digits.len() {
            return Err(CanonError::FloatRoundTripFailed);
        }
        digits[n_digits] = b;
        n_digits += 1;
    }
    while n_digits > 1 && digits[n_digits - 1] == b'0' {
        n_digits -= 1;
    }
    if n_digits == 0 {
        return Err(CanonError::FloatRoundTripFailed);
    }

    let mut formatted = StackAscii::<48>::new();
    if negative {
        formatted
            .push_byte(b'-')
            .map_err(|_| CanonError::FloatRoundTripFailed)?;
    }

    if (-6..21).contains(&exp) {
        let dp = exp + 1;
        if dp <= 0 {
            formatted
                .push_bytes(b"0.")
                .map_err(|_| CanonError::FloatRoundTripFailed)?;
            for _ in 0..(-dp) {
                formatted
                    .push_byte(b'0')
                    .map_err(|_| CanonError::FloatRoundTripFailed)?;
            }
            formatted
                .push_bytes(&digits[..n_digits])
                .map_err(|_| CanonError::FloatRoundTripFailed)?;
        } else {
            let dp_usize = dp as usize;
            if dp_usize >= n_digits {
                formatted
                    .push_bytes(&digits[..n_digits])
                    .map_err(|_| CanonError::FloatRoundTripFailed)?;
                for _ in 0..(dp_usize - n_digits) {
                    formatted
                        .push_byte(b'0')
                        .map_err(|_| CanonError::FloatRoundTripFailed)?;
                }
            } else {
                formatted
                    .push_bytes(&digits[..dp_usize])
                    .map_err(|_| CanonError::FloatRoundTripFailed)?;
                formatted
                    .push_byte(b'.')
                    .map_err(|_| CanonError::FloatRoundTripFailed)?;
                formatted
                    .push_bytes(&digits[dp_usize..n_digits])
                    .map_err(|_| CanonError::FloatRoundTripFailed)?;
            }
        }
    } else {
        formatted
            .push_byte(digits[0])
            .map_err(|_| CanonError::FloatRoundTripFailed)?;
        if n_digits > 1 {
            formatted
                .push_byte(b'.')
                .map_err(|_| CanonError::FloatRoundTripFailed)?;
            formatted
                .push_bytes(&digits[1..n_digits])
                .map_err(|_| CanonError::FloatRoundTripFailed)?;
        }
        if exp >= 0 {
            write!(formatted, "e+{exp}").map_err(|_| CanonError::FloatRoundTripFailed)?;
        } else {
            write!(formatted, "e-{abs_exp}", abs_exp = exp.unsigned_abs())
                .map_err(|_| CanonError::FloatRoundTripFailed)?;
        }
    }

    // Rule 4 mandatory re-parse verification before emitting.
    let formatted_str = formatted.as_str()?;
    let reparsed = formatted_str
        .parse::<f64>()
        .map_err(|_| CanonError::FloatRoundTripFailed)?;
    if reparsed.to_bits() != x.to_bits() {
        return Err(CanonError::FloatRoundTripFailed);
    }

    out.extend_from_slice(formatted.as_bytes());
    Ok(())
}
