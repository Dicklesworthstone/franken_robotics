//! Exact arbitrary-precision rational arithmetic and small-matrix linear algebra
//! for the Mathematics Compendium oracle library (`N12-41`).
//!
//! Pure `std`-only, zero `unsafe`, determinism class D0. Every [`Rat`] is reduced
//! to lowest terms with strictly positive denominator (`0` is `0/1`), so exact
//! rational pose composition, Cayley maps, adjoints, inverse differentials, and
//! 5th-power shell limits never overflow or lose precision.

use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

const BASE: u64 = 1 << 32;

/// Arbitrary-precision unsigned integer in little-endian base-`2^32` limbs.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct BigNat {
    limbs: Vec<u32>,
}

impl BigNat {
    #[must_use]
    pub const fn zero() -> Self {
        Self { limbs: Vec::new() }
    }

    #[must_use]
    pub fn one() -> Self {
        Self { limbs: vec![1] }
    }

    #[must_use]
    pub fn from_u64(v: u64) -> Self {
        if v == 0 {
            return Self::zero();
        }
        let lo = v as u32;
        let hi = (v >> 32) as u32;
        if hi == 0 {
            Self { limbs: vec![lo] }
        } else {
            Self { limbs: vec![lo, hi] }
        }
    }

    #[must_use]
    pub fn from_u128(mut v: u128) -> Self {
        let mut limbs = Vec::new();
        while v != 0 {
            limbs.push(v as u32);
            v >>= 32;
        }
        Self { limbs }
    }

    #[must_use]
    pub fn is_zero(&self) -> bool {
        self.limbs.is_empty()
    }

    #[must_use]
    pub fn is_one(&self) -> bool {
        self.limbs.len() == 1 && self.limbs[0] == 1
    }

    fn trim(&mut self) {
        while self.limbs.last() == Some(&0) {
            self.limbs.pop();
        }
    }

    fn bit_len(&self) -> usize {
        match self.limbs.last() {
            None => 0,
            Some(&top) => (self.limbs.len() - 1) * 32 + (32 - top.leading_zeros() as usize),
        }
    }

    fn get_bit(&self, idx: usize) -> bool {
        let limb = idx / 32;
        let bit = idx % 32;
        self.limbs.get(limb).is_some_and(|&w| ((w >> bit) & 1) != 0)
    }

    fn set_bit(&mut self, idx: usize) {
        let limb = idx / 32;
        let bit = idx % 32;
        if limb >= self.limbs.len() {
            self.limbs.resize(limb + 1, 0);
        }
        self.limbs[limb] |= 1u32 << bit;
    }

    fn shl1_add_bit(&mut self, bit: bool) {
        let mut carry = u64::from(bit);
        for w in &mut self.limbs {
            let v = (u64::from(*w) << 1) | carry;
            *w = v as u32;
            carry = v >> 32;
        }
        if carry != 0 {
            self.limbs.push(carry as u32);
        }
        self.trim();
    }

    #[must_use]
    pub fn add_nat(&self, rhs: &Self) -> Self {
        let n = self.limbs.len().max(rhs.limbs.len());
        let mut out = Vec::with_capacity(n + 1);
        let mut carry = 0u64;
        for i in 0..n {
            let a = u64::from(self.limbs.get(i).copied().unwrap_or(0));
            let b = u64::from(rhs.limbs.get(i).copied().unwrap_or(0));
            let s = a + b + carry;
            out.push(s as u32);
            carry = s >> 32;
        }
        if carry != 0 {
            out.push(carry as u32);
        }
        let mut r = Self { limbs: out };
        r.trim();
        r
    }

    /// Precondition: `self >= rhs`.
    #[must_use]
    pub fn sub_nat(&self, rhs: &Self) -> Self {
        let mut out = Vec::with_capacity(self.limbs.len());
        let mut borrow = 0i64;
        for i in 0..self.limbs.len() {
            let a = i64::from(self.limbs[i]);
            let b = i64::from(rhs.limbs.get(i).copied().unwrap_or(0));
            let mut d = a - b - borrow;
            if d < 0 {
                d += BASE as i64;
                borrow = 1;
            } else {
                borrow = 0;
            }
            out.push(d as u32);
        }
        let mut r = Self { limbs: out };
        r.trim();
        r
    }

    #[must_use]
    pub fn mul_nat(&self, rhs: &Self) -> Self {
        if self.is_zero() || rhs.is_zero() {
            return Self::zero();
        }
        let mut out = vec![0u32; self.limbs.len() + rhs.limbs.len()];
        for (i, &a) in self.limbs.iter().enumerate() {
            let mut carry = 0u64;
            for (j, &b) in rhs.limbs.iter().enumerate() {
                let prod = u64::from(a) * u64::from(b) + u64::from(out[i + j]) + carry;
                out[i + j] = prod as u32;
                carry = prod >> 32;
            }
            out[i + rhs.limbs.len()] = carry as u32;
        }
        let mut r = Self { limbs: out };
        r.trim();
        r
    }

    /// Exact integer division with remainder (`rhs != 0`).
    #[must_use]
    pub fn div_rem(&self, rhs: &Self) -> (Self, Self) {
        assert!(!rhs.is_zero(), "BigNat division by zero");
        match self.cmp(rhs) {
            Ordering::Less => return (Self::zero(), self.clone()),
            Ordering::Equal => return (Self::one(), Self::zero()),
            Ordering::Greater => {}
        }
        if rhs.limbs.len() == 1 {
            let d = u64::from(rhs.limbs[0]);
            let mut q = vec![0u32; self.limbs.len()];
            let mut rem = 0u64;
            for i in (0..self.limbs.len()).rev() {
                let cur = (rem << 32) | u64::from(self.limbs[i]);
                q[i] = (cur / d) as u32;
                rem = cur % d;
            }
            let mut qn = Self { limbs: q };
            qn.trim();
            return (qn, Self::from_u64(rem));
        }
        let mut q = Self::zero();
        let mut r = Self::zero();
        for bit in (0..self.bit_len()).rev() {
            r.shl1_add_bit(self.get_bit(bit));
            if r >= *rhs {
                r = r.sub_nat(rhs);
                q.set_bit(bit);
            }
        }
        q.trim();
        r.trim();
        (q, r)
    }

    #[must_use]
    pub fn gcd(mut a: Self, mut b: Self) -> Self {
        while !b.is_zero() {
            let (_, r) = a.div_rem(&b);
            a = b;
            b = r;
        }
        a
    }

    /// Exact integer square root: returns `Some(r)` iff `r * r == *self`.
    #[must_use]
    pub fn exact_isqrt(&self) -> Option<Self> {
        if self.is_zero() {
            return Some(Self::zero());
        }
        let top_bit = (self.bit_len() - 1) / 2;
        let mut root = Self::zero();
        for bit in (0..=top_bit).rev() {
            let mut cand = root.clone();
            cand.set_bit(bit);
            if cand.mul_nat(&cand) <= *self {
                root = cand;
            }
        }
        if root.mul_nat(&root) == *self {
            Some(root)
        } else {
            None
        }
    }

    #[must_use]
    pub fn to_decimal(&self) -> String {
        if self.is_zero() {
            return "0".to_owned();
        }
        let mut cur = self.clone();
        let ten_e9 = Self::from_u64(1_000_000_000);
        let mut chunks = Vec::new();
        while !cur.is_zero() {
            let (q, r) = cur.div_rem(&ten_e9);
            let val = r.limbs.first().copied().unwrap_or(0);
            chunks.push(val);
            cur = q;
        }
        let mut out = String::new();
        if let Some(top) = chunks.pop() {
            out.push_str(&top.to_string());
        }
        for c in chunks.into_iter().rev() {
            out.push_str(&format!("{c:09}"));
        }
        out
    }

    #[must_use]
    pub fn to_f64_approx(&self) -> f64 {
        let mut acc = 0.0_f64;
        for &w in self.limbs.iter().rev() {
            acc = acc * (BASE as f64) + f64::from(w);
        }
        acc
    }
}

impl Ord for BigNat {
    fn cmp(&self, other: &Self) -> Ordering {
        self.limbs
            .len()
            .cmp(&other.limbs.len())
            .then_with(|| self.limbs.iter().rev().cmp(other.limbs.iter().rev()))
    }
}

impl PartialOrd for BigNat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Debug for BigNat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_decimal())
    }
}

/// Exact reduced rational `num / den` (`den > 0`, `gcd(|num|, den) == 1`, zero is `0/1`).
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Rat {
    neg: bool,
    num: BigNat,
    den: BigNat,
}

impl Rat {
    #[must_use]
    pub fn zero() -> Self {
        Self {
            neg: false,
            num: BigNat::zero(),
            den: BigNat::one(),
        }
    }

    #[must_use]
    pub fn one() -> Self {
        Self {
            neg: false,
            num: BigNat::one(),
            den: BigNat::one(),
        }
    }

    #[must_use]
    pub fn from_int(n: i64) -> Self {
        Self {
            neg: n < 0,
            num: BigNat::from_u64(n.unsigned_abs()),
            den: BigNat::one(),
        }
    }

    /// Construct reduced `num / den` (`den != 0`).
    #[must_use]
    pub fn new(num: i64, den: i64) -> Self {
        assert!(den != 0, "Rat denominator cannot be zero");
        let neg = (num < 0) ^ (den < 0);
        Self::from_parts(neg, BigNat::from_u64(num.unsigned_abs()), BigNat::from_u64(den.unsigned_abs()))
    }

    #[must_use]
    pub fn from_i128(num: i128, den: i128) -> Self {
        assert!(den != 0, "Rat denominator cannot be zero");
        let neg = (num < 0) ^ (den < 0);
        Self::from_parts(neg, BigNat::from_u128(num.unsigned_abs()), BigNat::from_u128(den.unsigned_abs()))
    }

    fn from_parts(neg: bool, num: BigNat, den: BigNat) -> Self {
        assert!(!den.is_zero(), "Rat denominator cannot be zero");
        if num.is_zero() {
            return Self::zero();
        }
        let g = BigNat::gcd(num.clone(), den.clone());
        if g.is_one() {
            Self { neg, num, den }
        } else {
            let (rn, _) = num.div_rem(&g);
            let (rd, _) = den.div_rem(&g);
            Self { neg, num: rn, den: rd }
        }
    }

    #[must_use]
    pub fn is_zero(&self) -> bool {
        self.num.is_zero()
    }

    #[must_use]
    pub fn is_pos(&self) -> bool {
        !self.neg && !self.num.is_zero()
    }

    #[must_use]
    pub fn is_neg(&self) -> bool {
        self.neg && !self.num.is_zero()
    }

    #[must_use]
    pub fn abs(&self) -> Self {
        Self {
            neg: false,
            num: self.num.clone(),
            den: self.den.clone(),
        }
    }

    #[must_use]
    pub fn powu(&self, mut exp: u32) -> Self {
        let mut base = self.clone();
        let mut acc = Self::one();
        while exp > 0 {
            if (exp & 1) == 1 {
                acc = &acc * &base;
            }
            exp >>= 1;
            if exp > 0 {
                base = &base * &base;
            }
        }
        acc
    }

    /// Exact rational square root: returns `Some(r)` with `r >= 0` iff `r * r == *self`.
    #[must_use]
    pub fn exact_sqrt(&self) -> Option<Self> {
        if self.is_neg() {
            return None;
        }
        if self.is_zero() {
            return Some(Self::zero());
        }
        let sn = self.num.exact_isqrt()?;
        let sd = self.den.exact_isqrt()?;
        Some(Self::from_parts(false, sn, sd))
    }

    /// Floating-point approximation for display/tolerance rows only.
    #[must_use]
    pub fn to_f64(&self) -> f64 {
        if self.is_zero() {
            return 0.0;
        }
        let mag = self.num.to_f64_approx() / self.den.to_f64_approx();
        if self.neg { -mag } else { mag }
    }
}

impl Default for Rat {
    fn default() -> Self {
        Self::zero()
    }
}

impl Ord for Rat {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.is_neg(), other.is_neg()) {
            (true, false) => Ordering::Less,
            (false, true) => Ordering::Greater,
            (false, false) => self.num.mul_nat(&other.den).cmp(&other.num.mul_nat(&self.den)),
            (true, true) => other.num.mul_nat(&self.den).cmp(&self.num.mul_nat(&other.den)),
        }
    }
}

impl PartialOrd for Rat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Neg for &Rat {
    type Output = Rat;
    fn neg(self) -> Rat {
        if self.is_zero() {
            Rat::zero()
        } else {
            Rat {
                neg: !self.neg,
                num: self.num.clone(),
                den: self.den.clone(),
            }
        }
    }
}

impl Neg for Rat {
    type Output = Rat;
    fn neg(self) -> Rat {
        -&self
    }
}

impl Add for &Rat {
    type Output = Rat;
    fn add(self, rhs: &Rat) -> Rat {
        if self.is_zero() {
            return rhs.clone();
        }
        if rhs.is_zero() {
            return self.clone();
        }
        let a = self.num.mul_nat(&rhs.den);
        let b = rhs.num.mul_nat(&self.den);
        let den = self.den.mul_nat(&rhs.den);
        if self.neg == rhs.neg {
            Rat::from_parts(self.neg, a.add_nat(&b), den)
        } else {
            match a.cmp(&b) {
                Ordering::Equal => Rat::zero(),
                Ordering::Greater => Rat::from_parts(self.neg, a.sub_nat(&b), den),
                Ordering::Less => Rat::from_parts(rhs.neg, b.sub_nat(&a), den),
            }
        }
    }
}

impl Add for Rat {
    type Output = Rat;
    fn add(self, rhs: Rat) -> Rat {
        &self + &rhs
    }
}

impl Sub for &Rat {
    type Output = Rat;
    fn sub(self, rhs: &Rat) -> Rat {
        self + &(-rhs)
    }
}

impl Sub for Rat {
    type Output = Rat;
    fn sub(self, rhs: Rat) -> Rat {
        &self - &rhs
    }
}

impl Mul for &Rat {
    type Output = Rat;
    fn mul(self, rhs: &Rat) -> Rat {
        if self.is_zero() || rhs.is_zero() {
            return Rat::zero();
        }
        Rat::from_parts(
            self.neg ^ rhs.neg,
            self.num.mul_nat(&rhs.num),
            self.den.mul_nat(&rhs.den),
        )
    }
}

impl Mul for Rat {
    type Output = Rat;
    fn mul(self, rhs: Rat) -> Rat {
        &self * &rhs
    }
}

impl Div for &Rat {
    type Output = Rat;
    fn div(self, rhs: &Rat) -> Rat {
        assert!(!rhs.is_zero(), "Rat division by zero");
        if self.is_zero() {
            return Rat::zero();
        }
        Rat::from_parts(
            self.neg ^ rhs.neg,
            self.num.mul_nat(&rhs.den),
            self.den.mul_nat(&rhs.num),
        )
    }
}

impl Div for Rat {
    type Output = Rat;
    fn div(self, rhs: Rat) -> Rat {
        &self / &rhs
    }
}

impl fmt::Display for Rat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_zero() {
            return f.write_str("0");
        }
        if self.neg {
            f.write_str("-")?;
        }
        f.write_str(&self.num.to_decimal())?;
        if !self.den.is_one() {
            f.write_str("/")?;
            f.write_str(&self.den.to_decimal())?;
        }
        Ok(())
    }
}

impl fmt::Debug for Rat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

// ---- Exact small-vector and small-matrix algebra over Q ----

pub type Vec3 = [Rat; 3];
pub type Vec6 = [Rat; 6];
pub type Mat3 = [[Rat; 3]; 3];
pub type Mat4 = [[Rat; 4]; 4];
pub type Mat6 = [[Rat; 6]; 6];

#[must_use]
pub fn r(num: i64, den: i64) -> Rat {
    Rat::new(num, den)
}

#[must_use]
pub fn ri(n: i64) -> Rat {
    Rat::from_int(n)
}

#[must_use]
pub fn vec3(x: Rat, y: Rat, z: Rat) -> Vec3 {
    [x, y, z]
}

#[must_use]
pub fn vec3_zero() -> Vec3 {
    [Rat::zero(), Rat::zero(), Rat::zero()]
}

#[must_use]
pub fn vec3_add(a: &Vec3, b: &Vec3) -> Vec3 {
    [&a[0] + &b[0], &a[1] + &b[1], &a[2] + &b[2]]
}

#[must_use]
pub fn vec3_sub(a: &Vec3, b: &Vec3) -> Vec3 {
    [&a[0] - &b[0], &a[1] - &b[1], &a[2] - &b[2]]
}

#[must_use]
pub fn vec3_neg(a: &Vec3) -> Vec3 {
    [-&a[0], -&a[1], -&a[2]]
}

#[must_use]
pub fn vec3_scale(s: &Rat, a: &Vec3) -> Vec3 {
    [s * &a[0], s * &a[1], s * &a[2]]
}

#[must_use]
pub fn dot3(a: &Vec3, b: &Vec3) -> Rat {
    &(&a[0] * &b[0]) + &(&(&a[1] * &b[1]) + &(&a[2] * &b[2]))
}

#[must_use]
pub fn cross3(a: &Vec3, b: &Vec3) -> Vec3 {
    [
        &(&a[1] * &b[2]) - &(&a[2] * &b[1]),
        &(&a[2] * &b[0]) - &(&a[0] * &b[2]),
        &(&a[0] * &b[1]) - &(&a[1] * &b[0]),
    ]
}

#[must_use]
pub fn hat3(v: &Vec3) -> Mat3 {
    [
        [Rat::zero(), -&v[2], v[1].clone()],
        [v[2].clone(), Rat::zero(), -&v[0]],
        [-&v[1], v[0].clone(), Rat::zero()],
    ]
}

#[must_use]
pub fn vee3(m: &Mat3) -> Vec3 {
    [m[2][1].clone(), m[0][2].clone(), m[1][0].clone()]
}

#[must_use]
pub fn outer3(a: &Vec3, b: &Vec3) -> Mat3 {
    std::array::from_fn(|i| std::array::from_fn(|j| &a[i] * &b[j]))
}

#[must_use]
pub fn mat3_zero() -> Mat3 {
    std::array::from_fn(|_| std::array::from_fn(|_| Rat::zero()))
}

#[must_use]
pub fn mat3_eye() -> Mat3 {
    std::array::from_fn(|i| std::array::from_fn(|j| if i == j { Rat::one() } else { Rat::zero() }))
}

#[must_use]
pub fn mat3_diag(d: &Vec3) -> Mat3 {
    std::array::from_fn(|i| std::array::from_fn(|j| if i == j { d[i].clone() } else { Rat::zero() }))
}

#[must_use]
pub fn mat3_add(a: &Mat3, b: &Mat3) -> Mat3 {
    std::array::from_fn(|i| std::array::from_fn(|j| &a[i][j] + &b[i][j]))
}

#[must_use]
pub fn mat3_sub(a: &Mat3, b: &Mat3) -> Mat3 {
    std::array::from_fn(|i| std::array::from_fn(|j| &a[i][j] - &b[i][j]))
}

#[must_use]
pub fn mat3_scale(s: &Rat, a: &Mat3) -> Mat3 {
    std::array::from_fn(|i| std::array::from_fn(|j| s * &a[i][j]))
}

#[must_use]
pub fn mat3_transpose(a: &Mat3) -> Mat3 {
    std::array::from_fn(|i| std::array::from_fn(|j| a[j][i].clone()))
}

#[must_use]
pub fn mat3_trace(a: &Mat3) -> Rat {
    &a[0][0] + &(&a[1][1] + &a[2][2])
}

#[must_use]
pub fn mat3_mul(a: &Mat3, b: &Mat3) -> Mat3 {
    std::array::from_fn(|i| {
        std::array::from_fn(|j| {
            &(&a[i][0] * &b[0][j]) + &(&(&a[i][1] * &b[1][j]) + &(&a[i][2] * &b[2][j]))
        })
    })
}

#[must_use]
pub fn mat3_vec(a: &Mat3, v: &Vec3) -> Vec3 {
    std::array::from_fn(|i| {
        &(&a[i][0] * &v[0]) + &(&(&a[i][1] * &v[1]) + &(&a[i][2] * &v[2]))
    })
}

#[must_use]
pub fn mat3_det(m: &Mat3) -> Rat {
    let c0 = &(&m[1][1] * &m[2][2]) - &(&m[1][2] * &m[2][1]);
    let c1 = &(&m[1][0] * &m[2][2]) - &(&m[1][2] * &m[2][0]);
    let c2 = &(&m[1][0] * &m[2][1]) - &(&m[1][1] * &m[2][0]);
    &(&m[0][0] * &c0) - &(&(&m[0][1] * &c1) - &(&m[0][2] * &c2))
}

#[must_use]
pub fn mat_inv_n<const N: usize>(m: &[[Rat; N]; N]) -> Option<[[Rat; N]; N]> {
    let mut aug = vec![vec![Rat::zero(); 2 * N]; N];
    for i in 0..N {
        for j in 0..N {
            aug[i][j] = m[i][j].clone();
        }
        aug[i][N + i] = Rat::one();
    }
    for col in 0..N {
        let pivot_row = (col..N).find(|&row| !aug[row][col].is_zero())?;
        if pivot_row != col {
            aug.swap(pivot_row, col);
        }
        let piv = aug[col][col].clone();
        for c in col..(2 * N) {
            aug[col][c] = &aug[col][c] / &piv;
        }
        for row in 0..N {
            if row == col || aug[row][col].is_zero() {
                continue;
            }
            let factor = aug[row][col].clone();
            for c in col..(2 * N) {
                let delta = &factor * &aug[col][c];
                aug[row][c] = &aug[row][c] - &delta;
            }
        }
    }
    Some(std::array::from_fn(|i| {
        std::array::from_fn(|j| aug[i][N + j].clone())
    }))
}

#[must_use]
pub fn mat3_inv(m: &Mat3) -> Option<Mat3> {
    mat_inv_n::<3>(m)
}

#[must_use]
pub fn mat6_eye() -> Mat6 {
    std::array::from_fn(|i| std::array::from_fn(|j| if i == j { Rat::one() } else { Rat::zero() }))
}

#[must_use]
pub fn mat6_from_blocks(tl: &Mat3, tr: &Mat3, bl: &Mat3, br: &Mat3) -> Mat6 {
    std::array::from_fn(|i| {
        std::array::from_fn(|j| match (i < 3, j < 3) {
            (true, true) => tl[i][j].clone(),
            (true, false) => tr[i][j - 3].clone(),
            (false, true) => bl[i - 3][j].clone(),
            (false, false) => br[i - 3][j - 3].clone(),
        })
    })
}

#[must_use]
pub fn mat6_transpose(a: &Mat6) -> Mat6 {
    std::array::from_fn(|i| std::array::from_fn(|j| a[j][i].clone()))
}

#[must_use]
pub fn mat6_mul(a: &Mat6, b: &Mat6) -> Mat6 {
    std::array::from_fn(|i| {
        std::array::from_fn(|j| {
            let mut acc = Rat::zero();
            for k in 0..6 {
                acc = &acc + &(&a[i][k] * &b[k][j]);
            }
            acc
        })
    })
}

#[must_use]
pub fn mat6_vec(a: &Mat6, v: &Vec6) -> Vec6 {
    std::array::from_fn(|i| {
        let mut acc = Rat::zero();
        for k in 0..6 {
            acc = &acc + &(&a[i][k] * &v[k]);
        }
        acc
    })
}

#[must_use]
pub fn mat6_inv(m: &Mat6) -> Option<Mat6> {
    mat_inv_n::<6>(m)
}

#[must_use]
pub fn dot6(a: &Vec6, b: &Vec6) -> Rat {
    let mut acc = Rat::zero();
    for i in 0..6 {
        acc = &acc + &(&a[i] * &b[i]);
    }
    acc
}

/// Zero 4×4 rational matrix.
#[must_use]
pub fn mat4_zero() -> Mat4 {
    std::array::from_fn(|_| std::array::from_fn(|_| Rat::zero()))
}

/// Exact LDLᵀ positive-definiteness test for a symmetric 4×4 rational matrix (`P ≻ 0`).
#[must_use]
pub fn mat4_is_spd(m: &Mat4) -> bool {
    for i in 0..4 {
        for j in (i + 1)..4 {
            if m[i][j] != m[j][i] {
                return false;
            }
        }
    }
    let mut l = mat4_zero();
    let mut d: [Rat; 4] = std::array::from_fn(|_| Rat::zero());
    for j in 0..4 {
        let mut sum = m[j][j].clone();
        for k in 0..j {
            sum = &sum - &(&(&l[j][k] * &l[j][k]) * &d[k]);
        }
        if !sum.is_pos() {
            return false;
        }
        d[j] = sum;
        l[j][j] = Rat::one();
        for i in (j + 1)..4 {
            let mut off = m[i][j].clone();
            for k in 0..j {
                off = &off - &(&(&l[i][k] * &l[j][k]) * &d[k]);
            }
            l[i][j] = &off / &d[j];
        }
    }
    true
}

#[must_use]
pub fn mat4_sub(a: &Mat4, b: &Mat4) -> Mat4 {
    std::array::from_fn(|i| std::array::from_fn(|j| &a[i][j] - &b[i][j]))
}
