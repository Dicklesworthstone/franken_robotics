//! Exact rational arithmetic over Q with strict parsing and bit limits (BD-026).
//!
//! Every `Rational` is stored in canonical reduced form `(num, den)` with `den > 0`
//! and `gcd(|num|, den) == 1`. Zero is canonically `0 / 1`.
//! No floating-point type appears anywhere in this module.

use std::cmp::Ordering;
use std::fmt;

use crate::bigint::{ArithError, BigInt};
use crate::limits::COMPILED_LIMITS;

/// Exact rational number `num / den` in lowest terms with `den > 0`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Rational {
    num: BigInt,
    den: BigInt,
}

impl Default for Rational {
    fn default() -> Self {
        Self::zero()
    }
}

impl Rational {
    /// `0 / 1`.
    pub fn zero() -> Self {
        Self {
            num: BigInt::zero(),
            den: BigInt::one(),
        }
    }

    /// `1 / 1`.
    pub fn one() -> Self {
        Self {
            num: BigInt::one(),
            den: BigInt::one(),
        }
    }

    /// Construct from an integer `n / 1`.
    pub fn from_bigint(n: BigInt) -> Result<Self, ArithError> {
        n.check_limit(COMPILED_LIMITS.max_bigint_bits)?;
        Ok(Self {
            num: n,
            den: BigInt::one(),
        })
    }

    /// Construct from `i64`.
    pub fn from_i64(n: i64) -> Self {
        Self {
            num: BigInt::from_i64(n),
            den: BigInt::one(),
        }
    }

    /// Construct from `num / den`, reducing by GCD and enforcing `den != 0` and bit limits.
    pub fn new(num: BigInt, den: BigInt) -> Result<Self, ArithError> {
        num.check_limit(COMPILED_LIMITS.max_bigint_bits)?;
        den.check_limit(COMPILED_LIMITS.max_bigint_bits)?;
        if den.is_zero() {
            return Err(ArithError::DivisionByZero);
        }
        if num.is_zero() {
            return Ok(Self::zero());
        }
        let (n, d) = if den.is_negative() {
            (num.negate(), den.negate())
        } else {
            (num, den)
        };
        let g = n.gcd(&d)?;
        if g.is_one() {
            Ok(Self { num: n, den: d })
        } else {
            let (rn, _) = n.div_rem_trunc(&g)?;
            let (rd, _) = d.div_rem_trunc(&g)?;
            rn.check_limit(COMPILED_LIMITS.max_bigint_bits)?;
            rd.check_limit(COMPILED_LIMITS.max_bigint_bits)?;
            Ok(Self { num: rn, den: rd })
        }
    }

    /// Construct from integer pair `p / q` with `i64` convenience.
    pub fn from_i64_pair(p: i64, q: i64) -> Result<Self, ArithError> {
        Self::new(BigInt::from_i64(p), BigInt::from_i64(q))
    }

    /// Parse strict pair encoding `["p", "q"]` (BD-026 item 2):
    /// both must be strict integer strings, `q > 0`, and `gcd(|p|, q) == 1`.
    pub fn parse_pair_strict(p_str: &str, q_str: &str) -> Result<Self, ArithError> {
        let p = BigInt::parse_strict(p_str).map_err(|e| match e {
            ArithError::LimitsExceeded { bits, limit } => {
                ArithError::LimitsExceeded { bits, limit }
            }
            other => ArithError::InvalidRationalForm(format!("numerator: {other}")),
        })?;
        let q = BigInt::parse_strict(q_str).map_err(|e| match e {
            ArithError::LimitsExceeded { bits, limit } => {
                ArithError::LimitsExceeded { bits, limit }
            }
            other => ArithError::InvalidRationalForm(format!("denominator: {other}")),
        })?;
        if !q.is_positive() {
            return Err(ArithError::InvalidRationalForm(format!(
                "denominator must be positive, got {q_str:?}"
            )));
        }
        let g = p.gcd(&q)?;
        if !g.is_one() {
            return Err(ArithError::InvalidRationalForm(format!(
                "rational {p_str}/{q_str} is not reduced (gcd = {g})"
            )));
        }
        Ok(Self { num: p, den: q })
    }

    /// Parse strict slash encoding `"p/q"` or whole integer `"p"` (BD-026 item 2):
    /// - Whole numbers must travel as `"p"` alone; `"p/1"` is refused.
    /// - Fractions `"p/q"` require `q > 1` and `gcd(|p|, q) == 1`.
    pub fn parse_slash_strict(s: &str) -> Result<Self, ArithError> {
        if let Some((p_str, q_str)) = s.split_once('/') {
            if q_str.contains('/') {
                return Err(ArithError::InvalidRationalForm(format!(
                    "multiple '/' in {s:?}"
                )));
            }
            let r = Self::parse_pair_strict(p_str, q_str)?;
            if r.den.is_one() {
                return Err(ArithError::InvalidRationalForm(format!(
                    "whole number must omit '/1' in slash encoding: {s:?}"
                )));
            }
            Ok(r)
        } else {
            let p = BigInt::parse_strict(s).map_err(|e| match e {
                ArithError::LimitsExceeded { bits, limit } => {
                    ArithError::LimitsExceeded { bits, limit }
                }
                other => ArithError::InvalidRationalForm(format!("whole rational: {other}")),
            })?;
            Ok(Self {
                num: p,
                den: BigInt::one(),
            })
        }
    }

    pub fn num(&self) -> &BigInt {
        &self.num
    }

    pub fn den(&self) -> &BigInt {
        &self.den
    }

    pub fn is_zero(&self) -> bool {
        self.num.is_zero()
    }

    pub fn is_negative(&self) -> bool {
        self.num.is_negative()
    }

    pub fn is_positive(&self) -> bool {
        self.num.is_positive()
    }

    pub fn signum(&self) -> i8 {
        self.num.signum()
    }

    pub fn negate(&self) -> Self {
        Self {
            num: self.num.negate(),
            den: self.den.clone(),
        }
    }

    pub fn abs(&self) -> Self {
        Self {
            num: self.num.abs(),
            den: self.den.clone(),
        }
    }

    /// Exact reciprocal `1 / self`.
    pub fn recip(&self) -> Result<Self, ArithError> {
        if self.is_zero() {
            return Err(ArithError::DivisionByZero);
        }
        if self.num.is_negative() {
            Ok(Self {
                num: self.den.negate(),
                den: self.num.abs(),
            })
        } else {
            Ok(Self {
                num: self.den.clone(),
                den: self.num.clone(),
            })
        }
    }

    /// Exact addition: `(a*d + b*c) / (b*d)` reduced by `gcd(b, d)`.
    pub fn add(&self, rhs: &Self) -> Result<Self, ArithError> {
        if self.is_zero() {
            return Ok(rhs.clone());
        }
        if rhs.is_zero() {
            return Ok(self.clone());
        }
        let g = self.den.gcd(&rhs.den)?;
        let (d1, _) = self.den.div_rem_trunc(&g)?;
        let (d2, _) = rhs.den.div_rem_trunc(&g)?;
        let lhs_term = self.num.mul(&d2)?;
        let rhs_term = rhs.num.mul(&d1)?;
        let num = lhs_term.add(&rhs_term)?;
        let den = d1.mul(&rhs.den)?;
        Self::new(num, den)
    }

    /// Exact subtraction.
    pub fn sub(&self, rhs: &Self) -> Result<Self, ArithError> {
        self.add(&rhs.negate())
    }

    /// Exact multiplication with cross-cancellation to stay within bit limits.
    pub fn mul(&self, rhs: &Self) -> Result<Self, ArithError> {
        if self.is_zero() || rhs.is_zero() {
            return Ok(Self::zero());
        }
        let g1 = self.num.gcd(&rhs.den)?;
        let g2 = rhs.num.gcd(&self.den)?;
        let (n1, _) = self.num.div_rem_trunc(&g1)?;
        let (d2, _) = rhs.den.div_rem_trunc(&g1)?;
        let (n2, _) = rhs.num.div_rem_trunc(&g2)?;
        let (d1, _) = self.den.div_rem_trunc(&g2)?;
        let num = n1.mul(&n2)?;
        let den = d1.mul(&d2)?;
        Self::new(num, den)
    }

    /// Exact division `self / rhs`.
    pub fn div(&self, rhs: &Self) -> Result<Self, ArithError> {
        let inv = rhs.recip()?;
        self.mul(&inv)
    }

    /// Exact non-negative integer power `self^exp`.
    pub fn pow_u32(&self, exp: u32) -> Result<Self, ArithError> {
        if exp == 0 {
            return Ok(Self::one());
        }
        if self.is_zero() {
            return Ok(Self::zero());
        }
        // Since gcd(|num|, den) == 1, gcd(|num|^exp, den^exp) == 1 automatically.
        let num = self.num.pow_u32(exp)?;
        let den = self.den.pow_u32(exp)?;
        Ok(Self { num, den })
    }

    /// Exact `floor(self)`.
    pub fn floor(&self) -> Result<BigInt, ArithError> {
        self.num.div_floor(&self.den)
    }

    /// Exact `ceil(self)`.
    pub fn ceil(&self) -> Result<BigInt, ArithError> {
        self.num.div_ceil(&self.den)
    }

    /// Format as pair of canonical integer strings `("p", "q")`.
    pub fn to_pair_strings(&self) -> (String, String) {
        (self.num.to_decimal_string(), self.den.to_decimal_string())
    }

    /// Format as canonical slash string (`"p"` if whole, else `"p/q"`).
    pub fn to_slash_string(&self) -> String {
        if self.den.is_one() {
            self.num.to_decimal_string()
        } else {
            format!(
                "{}/{}",
                self.num.to_decimal_string(),
                self.den.to_decimal_string()
            )
        }
    }
}

impl Ord for Rational {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.signum() != other.signum() {
            return self.signum().cmp(&other.signum());
        }
        if self.den == other.den {
            return self.num.cmp(&other.num);
        }
        // Exact cross-multiplication; unchecked so comparison near bit limit never panics.
        let lhs = self.num.mul_unchecked(&other.den);
        let rhs = other.num.mul_unchecked(&self.den);
        lhs.cmp(&rhs)
    }
}

impl PartialOrd for Rational {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for Rational {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_slash_string())
    }
}
