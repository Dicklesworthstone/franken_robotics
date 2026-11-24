//! Outward-rounded dyadic interval arithmetic `frob-cert/dyadic-1` (BD-026, PO-AUGUR-03).
//!
//! Endpoints are dyadic rationals `m * 2^-p` with integer `m` and declared precision `p`.
//! Every operation computes the exact bounds of the result and rounds the lower endpoint
//! down (`floor`) and the upper endpoint up (`ceil`) onto the `2^-p` grid, widening by at
//! most 1 ULP per side and never narrowing.
//!
//! No floating-point type appears anywhere in this module.

use std::fmt;

use crate::bigint::{ArithError, BigInt};
use crate::limits::COMPILED_LIMITS;
use crate::rational::Rational;

/// Declared interval implementation identifier (BD-026 item 6).
pub const INTERVAL_IMPL_ID: &str = "frob-cert/dyadic-1";

/// Outward-rounded dyadic interval `[lo * 2^-prec, hi * 2^-prec]` with `lo <= hi`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Interval {
    lo: BigInt,
    hi: BigInt,
    prec: u32,
}

impl Interval {
    /// Construct directly from integer grid endpoints `[lo * 2^-prec, hi * 2^-prec]`.
    pub fn from_dyadic(lo: BigInt, hi: BigInt, prec: u32) -> Result<Self, ArithError> {
        check_prec(prec)?;
        lo.check_limit(COMPILED_LIMITS.max_bigint_bits)?;
        hi.check_limit(COMPILED_LIMITS.max_bigint_bits)?;
        if lo > hi {
            return Err(ArithError::DomainError(format!(
                "interval lower bound {lo} exceeds upper bound {hi}"
            )));
        }
        Ok(Self { lo, hi, prec })
    }

    /// Construct the tightest outward-rounded dyadic enclosure of an exact `Rational`.
    pub fn from_rational(r: &Rational, prec: u32) -> Result<Self, ArithError> {
        Self::from_rational_bounds(r, r, prec)
    }

    /// Construct the tightest outward-rounded dyadic enclosure of `[lo_r, hi_r]`.
    pub fn from_rational_bounds(
        lo_r: &Rational,
        hi_r: &Rational,
        prec: u32,
    ) -> Result<Self, ArithError> {
        check_prec(prec)?;
        if lo_r > hi_r {
            return Err(ArithError::DomainError(format!(
                "interval lower bound {lo_r} exceeds upper bound {hi_r}"
            )));
        }
        let lo = round_rational_down(lo_r, prec)?;
        let hi = round_rational_up(hi_r, prec)?;
        Self::from_dyadic(lo, hi, prec)
    }

    /// Exact point interval for an integer `n` on grid `2^-prec`.
    pub fn from_i64(n: i64, prec: u32) -> Result<Self, ArithError> {
        check_prec(prec)?;
        let scaled = BigInt::from_i64(n).shl_bits(prec as usize)?;
        Self::from_dyadic(scaled.clone(), scaled, prec)
    }

    pub fn lo_grid(&self) -> &BigInt {
        &self.lo
    }

    pub fn hi_grid(&self) -> &BigInt {
        &self.hi
    }

    pub fn prec(&self) -> u32 {
        self.prec
    }

    /// Exact `Rational` value of the lower endpoint `lo * 2^-prec`.
    pub fn lo_rational(&self) -> Result<Rational, ArithError> {
        let den = BigInt::one().shl_bits(self.prec as usize)?;
        Rational::new(self.lo.clone(), den)
    }

    /// Exact `Rational` value of the upper endpoint `hi * 2^-prec`.
    pub fn hi_rational(&self) -> Result<Rational, ArithError> {
        let den = BigInt::one().shl_bits(self.prec as usize)?;
        Rational::new(self.hi.clone(), den)
    }

    /// Exact containment check `self.lo * 2^-prec <= r <= self.hi * 2^-prec`.
    pub fn contains_rational(&self, r: &Rational) -> bool {
        let scaled_r = r.num().shl_bits_unchecked(self.prec as usize);
        let lo_prod = self.lo.mul_unchecked(r.den());
        let hi_prod = self.hi.mul_unchecked(r.den());
        lo_prod <= scaled_r && scaled_r <= hi_prod
    }

    /// True iff `0` lies in `[lo, hi]`.
    pub fn contains_zero(&self) -> bool {
        !self.lo.is_positive() && !self.hi.is_negative()
    }

    /// True iff the entire interval is strictly positive (`lo > 0`).
    pub fn is_strictly_positive(&self) -> bool {
        self.lo.is_positive()
    }

    /// True iff the entire interval is strictly negative (`hi < 0`).
    pub fn is_strictly_negative(&self) -> bool {
        self.hi.is_negative()
    }

    fn require_same_prec(&self, other: &Self) -> Result<(), ArithError> {
        if self.prec != other.prec {
            return Err(ArithError::DomainError(format!(
                "interval precision mismatch: {} vs {}",
                self.prec, other.prec
            )));
        }
        Ok(())
    }

    /// Exact negation `[-hi, -lo]`.
    pub fn negate(&self) -> Self {
        Self {
            lo: self.hi.negate(),
            hi: self.lo.negate(),
            prec: self.prec,
        }
    }

    /// Exact addition on equal dyadic grids (0 ULP widening).
    pub fn add(&self, rhs: &Self) -> Result<Self, ArithError> {
        self.require_same_prec(rhs)?;
        let lo = self.lo.add(&rhs.lo)?;
        let hi = self.hi.add(&rhs.hi)?;
        Self::from_dyadic(lo, hi, self.prec)
    }

    /// Exact subtraction on equal dyadic grids (0 ULP widening).
    pub fn sub(&self, rhs: &Self) -> Result<Self, ArithError> {
        self.require_same_prec(rhs)?;
        let lo = self.lo.sub(&rhs.hi)?;
        let hi = self.hi.sub(&rhs.lo)?;
        Self::from_dyadic(lo, hi, self.prec)
    }

    /// Outward-rounded multiplication (four corner products, rounded down/up by `2^-prec`).
    pub fn mul(&self, rhs: &Self) -> Result<Self, ArithError> {
        self.require_same_prec(rhs)?;
        #[cfg(feature = "nc-interval-mul-nearest")]
        {
            return self.mul_round_nearest_mutant(rhs);
        }
        #[cfg(not(feature = "nc-interval-mul-nearest"))]
        {
            self.mul_outward(rhs)
        }
    }

    #[cfg(any(test, not(feature = "nc-interval-mul-nearest")))]
    fn mul_outward(&self, rhs: &Self) -> Result<Self, ArithError> {
        let prods = [
            self.lo.mul_unchecked(&rhs.lo),
            self.lo.mul_unchecked(&rhs.hi),
            self.hi.mul_unchecked(&rhs.lo),
            self.hi.mul_unchecked(&rhs.hi),
        ];
        let mut min_p = &prods[0];
        let mut max_p = &prods[0];
        for p in &prods[1..] {
            if p < min_p {
                min_p = p;
            }
            if p > max_p {
                max_p = p;
            }
        }
        let shift = self.prec as usize;
        let lo = min_p.shr_floor_bits(shift);
        let hi = max_p.shr_ceil_bits(shift);
        Self::from_dyadic(lo, hi, self.prec)
    }

    /// Deliberately defective round-to-nearest interval multiply (negative control of BD-026).
    #[cfg(any(test, feature = "nc-interval-mul-nearest"))]
    pub fn mul_round_nearest_mutant(&self, rhs: &Self) -> Result<Self, ArithError> {
        self.require_same_prec(rhs)?;
        let prods = [
            self.lo.mul_unchecked(&rhs.lo),
            self.lo.mul_unchecked(&rhs.hi),
            self.hi.mul_unchecked(&rhs.lo),
            self.hi.mul_unchecked(&rhs.hi),
        ];
        let min_p = prods.iter().min().cloned().unwrap_or_else(BigInt::zero);
        let max_p = prods.iter().max().cloned().unwrap_or_else(BigInt::zero);
        let shift = self.prec as usize;
        let lo = round_shift_nearest(&min_p, shift);
        let hi = round_shift_nearest(&max_p, shift);
        Self::from_dyadic(lo, hi, self.prec)
    }

    /// Outward-rounded division by an interval that strictly excludes zero.
    pub fn div(&self, rhs: &Self) -> Result<Self, ArithError> {
        self.require_same_prec(rhs)?;
        if rhs.contains_zero() {
            return Err(ArithError::DivisionByZero);
        }
        // Notice: (a * 2^-p) / (b * 2^-p) == a / b, so scaled to grid 2^-p the exact
        // corner values are (a_end << p) / b_end!
        let shift = self.prec as usize;
        let corners = [
            (&self.lo, &rhs.lo),
            (&self.lo, &rhs.hi),
            (&self.hi, &rhs.lo),
            (&self.hi, &rhs.hi),
        ];
        // Find exact min and max among the 4 rational quotients a_end / b_end.
        let mut min_pair = corners[0];
        let mut max_pair = corners[0];
        for &cand in &corners[1..] {
            if cmp_ratio(cand.0, cand.1, min_pair.0, min_pair.1).is_lt() {
                min_pair = cand;
            }
            if cmp_ratio(cand.0, cand.1, max_pair.0, max_pair.1).is_gt() {
                max_pair = cand;
            }
        }
        let lo_num = min_pair.0.shl_bits_unchecked(shift);
        let hi_num = max_pair.0.shl_bits_unchecked(shift);
        let lo = lo_num.div_floor_unchecked(min_pair.1);
        let hi = hi_num.div_ceil_unchecked(max_pair.1);
        Self::from_dyadic(lo, hi, self.prec)
    }

    /// Outward-rounded non-negative integer power `self^exp`.
    /// Even powers of an interval containing zero start at exact `0`.
    pub fn pow_u32(&self, exp: u32) -> Result<Self, ArithError> {
        if exp == 0 {
            return Self::from_i64(1, self.prec);
        }
        if exp == 1 {
            return Ok(self.clone());
        }
        // For exact 1-ULP enclosure on moderate powers, compute exact integer powers
        // when within a generous scratch budget, else fall back to repeated outward squaring.
        let max_endpoint_bits = self.lo.bit_len().max(self.hi.bit_len());
        let exact_bits_est = max_endpoint_bits.saturating_mul(exp as usize);
        if exact_bits_est <= COMPILED_LIMITS.max_bigint_bits * 2 {
            let total_shift = (self.prec as usize).saturating_mul((exp - 1) as usize);
            if exp & 1 == 0 {
                if self.contains_zero() {
                    let max_abs = self.lo.abs().max(self.hi.abs());
                    let hi_raw = pow_mag_unchecked(&max_abs, exp);
                    let hi = hi_raw.shr_ceil_bits(total_shift);
                    Self::from_dyadic(BigInt::zero(), hi, self.prec)
                } else {
                    let a = self.lo.abs();
                    let b = self.hi.abs();
                    let (min_abs, max_abs) = if a <= b { (a, b) } else { (b, a) };
                    let lo = pow_mag_unchecked(&min_abs, exp).shr_floor_bits(total_shift);
                    let hi = pow_mag_unchecked(&max_abs, exp).shr_ceil_bits(total_shift);
                    Self::from_dyadic(lo, hi, self.prec)
                }
            } else {
                let lo = pow_signed_unchecked(&self.lo, exp).shr_floor_bits(total_shift);
                let hi = pow_signed_unchecked(&self.hi, exp).shr_ceil_bits(total_shift);
                Self::from_dyadic(lo, hi, self.prec)
            }
        } else {
            // Repeated outward-rounded binary exponentiation for huge exponents.
            if exp & 1 == 0 && self.contains_zero() {
                let max_abs = self.lo.abs().max(self.hi.abs());
                let upper_iv = Self::from_dyadic(BigInt::zero(), max_abs, self.prec)?;
                let mut base = upper_iv;
                let mut acc = Self::from_i64(1, self.prec)?;
                let mut e = exp;
                while e > 0 {
                    if e & 1 == 1 {
                        acc = acc.mul(&base)?;
                    }
                    e >>= 1;
                    if e > 0 {
                        base = base.mul(&base)?;
                    }
                }
                Self::from_dyadic(BigInt::zero(), acc.hi, self.prec)
            } else {
                let mut base = self.clone();
                let mut acc = Self::from_i64(1, self.prec)?;
                let mut e = exp;
                while e > 0 {
                    if e & 1 == 1 {
                        acc = acc.mul(&base)?;
                    }
                    e >>= 1;
                    if e > 0 {
                        base = base.mul(&base)?;
                    }
                }
                Ok(acc)
            }
        }
    }

    /// Outward-rounded square root for a non-negative interval (`lo >= 0`).
    /// Exact: `new_lo = floor(sqrt(lo * 2^prec))`, `new_hi = ceil(sqrt(hi * 2^prec))`.
    pub fn sqrt(&self) -> Result<Self, ArithError> {
        if self.lo.is_negative() {
            return Err(ArithError::DomainError(
                "square root of interval with negative lower bound".to_string(),
            ));
        }
        let shift = self.prec as usize;
        let scaled_lo = self.lo.shl_bits_unchecked(shift);
        let scaled_hi = self.hi.shl_bits_unchecked(shift);
        let new_lo = scaled_lo.isqrt_floor_unchecked();
        let new_hi = scaled_hi.isqrt_ceil_unchecked();
        Self::from_dyadic(new_lo, new_hi, self.prec)
    }

    /// Exact absolute value of an interval.
    pub fn abs(&self) -> Self {
        if self.contains_zero() {
            Self {
                lo: BigInt::zero(),
                hi: self.lo.abs().max(self.hi.abs()),
                prec: self.prec,
            }
        } else if self.hi.is_negative() {
            self.negate()
        } else {
            self.clone()
        }
    }

    /// Exact interval minimum `min(self, rhs)`.
    pub fn min(&self, rhs: &Self) -> Result<Self, ArithError> {
        self.require_same_prec(rhs)?;
        Ok(Self {
            lo: self.lo.clone().min(rhs.lo.clone()),
            hi: self.hi.clone().min(rhs.hi.clone()),
            prec: self.prec,
        })
    }

    /// Exact interval maximum `max(self, rhs)`.
    pub fn max(&self, rhs: &Self) -> Result<Self, ArithError> {
        self.require_same_prec(rhs)?;
        Ok(Self {
            lo: self.lo.clone().max(rhs.lo.clone()),
            hi: self.hi.clone().max(rhs.hi.clone()),
            prec: self.prec,
        })
    }
}

impl fmt::Display for Interval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}] * 2^-{}", self.lo, self.hi, self.prec)
    }
}

fn check_prec(prec: u32) -> Result<(), ArithError> {
    if (prec as usize) >= COMPILED_LIMITS.max_bigint_bits {
        return Err(ArithError::LimitsExceeded {
            bits: prec as usize,
            limit: COMPILED_LIMITS.max_bigint_bits,
        });
    }
    Ok(())
}

fn round_rational_down(r: &Rational, prec: u32) -> Result<BigInt, ArithError> {
    let scaled_num = r.num().shl_bits_unchecked(prec as usize);
    let q = scaled_num.div_floor_unchecked(r.den());
    q.check_limit(COMPILED_LIMITS.max_bigint_bits)?;
    Ok(q)
}

fn round_rational_up(r: &Rational, prec: u32) -> Result<BigInt, ArithError> {
    let scaled_num = r.num().shl_bits_unchecked(prec as usize);
    let q = scaled_num.div_ceil_unchecked(r.den());
    q.check_limit(COMPILED_LIMITS.max_bigint_bits)?;
    Ok(q)
}

fn cmp_ratio(a_num: &BigInt, a_den: &BigInt, b_num: &BigInt, b_den: &BigInt) -> std::cmp::Ordering {
    // Normalize denominators to positive for comparison.
    let (an, ad) = if a_den.is_negative() {
        (a_num.negate(), a_den.negate())
    } else {
        (a_num.clone(), a_den.clone())
    };
    let (bn, bd) = if b_den.is_negative() {
        (b_num.negate(), b_den.negate())
    } else {
        (b_num.clone(), b_den.clone())
    };
    an.mul_unchecked(&bd).cmp(&bn.mul_unchecked(&ad))
}

fn pow_mag_unchecked(base: &BigInt, mut exp: u32) -> BigInt {
    let mut b = base.abs();
    let mut acc = BigInt::one();
    while exp > 0 {
        if exp & 1 == 1 {
            acc = acc.mul_unchecked(&b);
        }
        exp >>= 1;
        if exp > 0 {
            b = b.mul_unchecked(&b);
        }
    }
    acc
}

fn pow_signed_unchecked(base: &BigInt, exp: u32) -> BigInt {
    let mag = pow_mag_unchecked(base, exp);
    if base.is_negative() && (exp & 1 == 1) {
        mag.negate()
    } else {
        mag
    }
}

#[cfg(any(test, feature = "nc-interval-mul-nearest"))]
fn round_shift_nearest(x: &BigInt, shift: usize) -> BigInt {
    if shift == 0 {
        return x.clone();
    }
    let half = BigInt::one().shl_bits_unchecked(shift - 1);
    x.add_unchecked(&half).shr_floor_bits(shift)
}

#[cfg(test)]
mod nc_tests {
    use super::*;

    #[test]
    fn nc_interval_mul_nearest_mutant_breaks_enclosure() {
        // Exact witness on grid 2^-2 = 1/4:
        // a = 1/4 = [1, 1] * 2^-2, b = 3/4 = [3, 3] * 2^-2 => exact product = 3/16.
        // Outward rounding yields [0, 1/4] (encloses 3/16).
        // Nearest rounding yields [1/4, 1/4] (drops 3/16 below lo!).
        let prec = 2;
        let a = Interval::from_dyadic(BigInt::from_i64(1), BigInt::from_i64(1), prec).unwrap();
        let b = Interval::from_dyadic(BigInt::from_i64(3), BigInt::from_i64(3), prec).unwrap();
        let exact = Rational::from_i64_pair(3, 16).unwrap();

        let outward = a.mul_outward(&b).unwrap();
        assert!(
            outward.contains_rational(&exact),
            "outward multiply must enclose 3/16"
        );

        let mutant = a.mul_round_nearest_mutant(&b).unwrap();
        assert!(
            !mutant.contains_rational(&exact),
            "round-to-nearest mutant must fail enclosure on 1/4 * 3/4 = 3/16"
        );

        // Symmetric upper-bound witness: 1/4 * 1/4 = 1/16 -> nearest yields [0, 0].
        let exact_sq = Rational::from_i64_pair(1, 16).unwrap();
        let outward_sq = a.mul_outward(&a).unwrap();
        assert!(outward_sq.contains_rational(&exact_sq));
        let mutant_sq = a.mul_round_nearest_mutant(&a).unwrap();
        assert!(
            !mutant_sq.contains_rational(&exact_sq),
            "round-to-nearest mutant must fail upper enclosure on 1/4 * 1/4 = 1/16"
        );
    }
}
