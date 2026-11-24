//! Exact integer, rational, and outward-rounded dyadic interval tests (BD-026, PO-AUGUR-02, PO-AUGUR-03).

use std::cmp::Ordering;

use frob_cert::bigint::{ArithError, BigInt};
use frob_cert::interval::Interval;
use frob_cert::limits::{COMPILED_LIMITS, RULE_LIMITS_EXCEEDED};
use frob_cert::rational::Rational;

// ---------------------------------------------------------------------------
// Deterministic PRNG (SplitMix64)
// ---------------------------------------------------------------------------

struct Rng(u64);

impl Rng {
    fn new(seed: u64) -> Self {
        Self(seed)
    }

    fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    fn next_bigint_limbs(&mut self, max_limbs: usize) -> BigInt {
        let len = ((self.next_u64() as usize) % max_limbs) + 1;
        let mut limbs = Vec::with_capacity(len);
        for _ in 0..len {
            limbs.push(self.next_u64() as u32);
        }
        let neg = (self.next_u64() & 1) == 1;
        BigInt::from_parts(neg, limbs).unwrap()
    }

    fn next_small_rational(&mut self, max_num: i64, max_den: i64) -> Rational {
        let sign = if (self.next_u64() & 1) == 1 { -1 } else { 1 };
        let num = (self.next_u64() % (max_num as u64 + 1)) as i64 * sign;
        let den = ((self.next_u64() % max_den as u64) + 1) as i64;
        Rational::from_i64_pair(num, den).unwrap()
    }
}

// ---------------------------------------------------------------------------
// Independent schoolbook reference implementation (PO-AUGUR-02)
//
// Uses base-10,000 little-endian `u16` digits in a fixed stack buffer (no heap
// allocations, no base-2^32 limbs, no Knuth Algorithm D — uses binary-search
// per decimal digit for division) so it shares zero algorithmic structure with
// `frob_cert::bigint` and `frob_cert::rational`.
// ---------------------------------------------------------------------------

const REF_BASE: u32 = 10_000;
const REF_CAP: usize = 48; // up to 192 decimal digits (~637 bits)

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct RefInt {
    neg: bool,
    len: usize,
    d: [u16; REF_CAP],
}

impl RefInt {
    fn zero() -> Self {
        Self {
            neg: false,
            len: 0,
            d: [0; REF_CAP],
        }
    }

    fn one() -> Self {
        let mut d = [0; REF_CAP];
        d[0] = 1;
        Self {
            neg: false,
            len: 1,
            d,
        }
    }

    fn from_i128(v: i128) -> Self {
        if v == 0 {
            return Self::zero();
        }
        let neg = v < 0;
        let mut mag = v.unsigned_abs();
        let mut d = [0_u16; REF_CAP];
        let mut len = 0;
        while mag > 0 {
            d[len] = (mag % u128::from(REF_BASE)) as u16;
            mag /= u128::from(REF_BASE);
            len += 1;
        }
        Self { neg, len, d }
    }

    fn from_decimal(s: &str) -> Self {
        let bytes = s.as_bytes();
        let (neg, digits) = if bytes[0] == b'-' {
            (true, &bytes[1..])
        } else {
            (false, bytes)
        };
        if digits == b"0" {
            return Self::zero();
        }
        let mut d = [0_u16; REF_CAP];
        let mut len = 0;
        let mut end = digits.len();
        while end > 0 {
            let start = end.saturating_sub(4);
            let mut val = 0_u16;
            for &b in &digits[start..end] {
                val = val * 10 + u16::from(b - b'0');
            }
            d[len] = val;
            len += 1;
            end = start;
        }
        let mut out = Self { neg, len, d };
        out.trim();
        out
    }

    fn to_decimal(&self) -> String {
        if self.len == 0 {
            return "0".to_string();
        }
        let mut out = String::new();
        if self.neg {
            out.push('-');
        }
        out.push_str(&self.d[self.len - 1].to_string());
        for i in (0..self.len - 1).rev() {
            out.push_str(&format!("{:04}", self.d[i]));
        }
        out
    }

    fn trim(&mut self) {
        while self.len > 0 && self.d[self.len - 1] == 0 {
            self.len -= 1;
        }
        if self.len == 0 {
            self.neg = false;
        }
    }

    fn is_zero(&self) -> bool {
        self.len == 0
    }

    fn negate(&self) -> Self {
        let mut c = *self;
        if c.len > 0 {
            c.neg = !c.neg;
        }
        c
    }

    fn abs(&self) -> Self {
        let mut c = *self;
        c.neg = false;
        c
    }

    fn cmp_mag(&self, rhs: &Self) -> Ordering {
        match self.len.cmp(&rhs.len) {
            Ordering::Equal => {
                for i in (0..self.len).rev() {
                    match self.d[i].cmp(&rhs.d[i]) {
                        Ordering::Equal => continue,
                        other => return other,
                    }
                }
                Ordering::Equal
            }
            other => other,
        }
    }

    fn cmp_signed(&self, rhs: &Self) -> Ordering {
        match (self.neg, rhs.neg) {
            (true, false) => Ordering::Less,
            (false, true) => Ordering::Greater,
            (false, false) => self.cmp_mag(rhs),
            (true, true) => rhs.cmp_mag(self),
        }
    }

    fn add_mag(&self, rhs: &Self) -> Self {
        let n = self.len.max(rhs.len);
        let mut out = Self::zero();
        let mut carry = 0_u32;
        for i in 0..n {
            let s = u32::from(self.d[i]) + u32::from(rhs.d[i]) + carry;
            out.d[i] = (s % REF_BASE) as u16;
            carry = s / REF_BASE;
        }
        out.len = n;
        if carry > 0 {
            out.d[n] = carry as u16;
            out.len += 1;
        }
        out.trim();
        out
    }

    fn sub_mag(&self, rhs: &Self) -> Self {
        // Requires |self| >= |rhs|.
        let mut out = Self::zero();
        let mut borrow = 0_i32;
        for i in 0..self.len {
            let mut diff = i32::from(self.d[i]) - i32::from(rhs.d[i]) - borrow;
            if diff < 0 {
                diff += REF_BASE as i32;
                borrow = 1;
            } else {
                borrow = 0;
            }
            out.d[i] = diff as u16;
        }
        out.len = self.len;
        out.trim();
        out
    }

    fn add(&self, rhs: &Self) -> Self {
        if self.neg == rhs.neg {
            let mut r = self.add_mag(rhs);
            r.neg = self.neg && r.len > 0;
            r
        } else {
            match self.cmp_mag(rhs) {
                Ordering::Equal => Self::zero(),
                Ordering::Greater => {
                    let mut r = self.sub_mag(rhs);
                    r.neg = self.neg && r.len > 0;
                    r
                }
                Ordering::Less => {
                    let mut r = rhs.sub_mag(self);
                    r.neg = rhs.neg && r.len > 0;
                    r
                }
            }
        }
    }

    fn sub(&self, rhs: &Self) -> Self {
        self.add(&rhs.negate())
    }

    fn mul_small_mag(&self, m: u32) -> Self {
        if self.len == 0 || m == 0 {
            return Self::zero();
        }
        let mut out = Self::zero();
        let mut carry = 0_u32;
        for i in 0..self.len {
            let prod = u32::from(self.d[i]) * m + carry;
            out.d[i] = (prod % REF_BASE) as u16;
            carry = prod / REF_BASE;
        }
        out.len = self.len;
        while carry > 0 {
            out.d[out.len] = (carry % REF_BASE) as u16;
            carry /= REF_BASE;
            out.len += 1;
        }
        out.trim();
        out
    }

    fn mul(&self, rhs: &Self) -> Self {
        if self.is_zero() || rhs.is_zero() {
            return Self::zero();
        }
        let mut acc = [0_u32; REF_CAP];
        for i in 0..self.len {
            let ai = u32::from(self.d[i]);
            let mut carry = 0_u32;
            for j in 0..rhs.len {
                let cur = acc[i + j] + ai * u32::from(rhs.d[j]) + carry;
                acc[i + j] = cur % REF_BASE;
                carry = cur / REF_BASE;
            }
            let mut k = i + rhs.len;
            while carry > 0 {
                let cur = acc[k] + carry;
                acc[k] = cur % REF_BASE;
                carry = cur / REF_BASE;
                k += 1;
            }
        }
        let mut out = Self::zero();
        let max_len = (self.len + rhs.len).min(REF_CAP);
        for i in 0..max_len {
            out.d[i] = acc[i] as u16;
        }
        out.len = max_len;
        out.neg = self.neg ^ rhs.neg;
        out.trim();
        out
    }

    /// Independent grade-school base-10,000 long division using binary search per digit.
    fn div_rem_mag(&self, rhs: &Self) -> (Self, Self) {
        assert!(!rhs.is_zero());
        match self.cmp_mag(rhs) {
            Ordering::Less => return (Self::zero(), self.abs()),
            Ordering::Equal => return (Self::one(), Self::zero()),
            Ordering::Greater => {}
        }
        let mut q = Self::zero();
        let mut rem = Self::zero();
        q.len = self.len;
        for i in (0..self.len).rev() {
            // Shift rem left by 1 base-10,000 digit and insert self.d[i].
            if rem.len > 0 {
                for j in (0..rem.len).rev() {
                    rem.d[j + 1] = rem.d[j];
                }
                rem.d[0] = self.d[i];
                rem.len += 1;
            } else if self.d[i] != 0 {
                rem.d[0] = self.d[i];
                rem.len = 1;
            }
            // Binary search q_dig in 0..=9999.
            let mut lo = 0_u32;
            let mut hi = REF_BASE - 1;
            let mut best = 0_u32;
            while lo <= hi {
                let mid = (lo + hi) / 2;
                let prod = rhs.mul_small_mag(mid);
                if prod.cmp_mag(&rem) != Ordering::Greater {
                    best = mid;
                    lo = mid + 1;
                } else {
                    if mid == 0 {
                        break;
                    }
                    hi = mid - 1;
                }
            }
            q.d[i] = best as u16;
            if best > 0 {
                let sub = rhs.mul_small_mag(best);
                rem = rem.sub_mag(&sub);
            }
        }
        q.trim();
        rem.trim();
        (q, rem)
    }

    fn div_rem_euclid(&self, rhs: &Self) -> (Self, Self) {
        let (mut q, mut r) = self.div_rem_mag(rhs);
        q.neg = (self.neg ^ rhs.neg) && q.len > 0;
        r.neg = self.neg && r.len > 0;
        if r.neg {
            if !rhs.neg {
                q = q.sub(&Self::one());
                r = r.add(rhs);
            } else {
                q = q.add(&Self::one());
                r = r.sub(rhs);
            }
        }
        (q, r)
    }

    fn gcd(&self, rhs: &Self) -> Self {
        let mut a = self.abs();
        let mut b = rhs.abs();
        while !b.is_zero() {
            let (_, r) = a.div_rem_mag(&b);
            a = b;
            b = r;
        }
        a
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct RefRat {
    n: RefInt,
    d: RefInt,
}

impl RefRat {
    fn new(mut n: RefInt, mut d: RefInt) -> Self {
        assert!(!d.is_zero());
        if n.is_zero() {
            return Self {
                n: RefInt::zero(),
                d: RefInt::one(),
            };
        }
        if d.neg {
            n = n.negate();
            d = d.negate();
        }
        let g = n.gcd(&d);
        let (rn, _) = n.div_rem_mag(&g);
        let (rd, _) = d.div_rem_mag(&g);
        let mut rn_signed = rn;
        rn_signed.neg = n.neg && rn_signed.len > 0;
        Self { n: rn_signed, d: rd }
    }

    fn add(&self, rhs: &Self) -> Self {
        let num = self.n.mul(&rhs.d).add(&rhs.n.mul(&self.d));
        let den = self.d.mul(&rhs.d);
        Self::new(num, den)
    }

    fn sub(&self, rhs: &Self) -> Self {
        let num = self.n.mul(&rhs.d).sub(&rhs.n.mul(&self.d));
        let den = self.d.mul(&rhs.d);
        Self::new(num, den)
    }

    fn mul(&self, rhs: &Self) -> Self {
        Self::new(self.n.mul(&rhs.n), self.d.mul(&rhs.d))
    }

    fn div(&self, rhs: &Self) -> Self {
        Self::new(self.n.mul(&rhs.d), self.d.mul(&rhs.n))
    }

    fn cmp(&self, rhs: &Self) -> Ordering {
        self.n.mul(&rhs.d).cmp_signed(&rhs.n.mul(&self.d))
    }
}

fn ref_from_bigint(b: &BigInt) -> RefInt {
    RefInt::from_decimal(&b.to_decimal_string())
}

// ---------------------------------------------------------------------------
// Required BD-026 tests
// ---------------------------------------------------------------------------

#[test]
fn bigint_divrem_identity() {
    let mut rng = Rng::new(0xBD02_6001_CAFE_BABE);

    // Targeted Knuth Algorithm D edge cases (add-back, qhat == BASE, powers of 2 ± 1).
    let base32 = BigInt::one().shl_bits(32).unwrap();
    let edge_pairs: Vec<(BigInt, BigInt)> = vec![
        // Classic Knuth D6 add-back trigger (base 2^32):
        // v = [0xFFFF_FFFF, 0x8000_0000], u = [0, 0xFFFF_FFFE, 0x7FFF_FFFF]
        (
            BigInt::from_parts(false, vec![0, 0xFFFF_FFFE, 0x7FFF_FFFF]).unwrap(),
            BigInt::from_parts(false, vec![0xFFFF_FFFF, 0x8000_0000]).unwrap(),
        ),
        // qhat == BASE trigger:
        (
            BigInt::from_parts(false, vec![0xFFFF_FFFF, 0xFFFF_FFFF, 0x8000_0000]).unwrap(),
            BigInt::from_parts(false, vec![0xFFFF_FFFF, 0x8000_0000]).unwrap(),
        ),
        (
            BigInt::one().shl_bits(4095).unwrap().sub(&BigInt::one()).unwrap(),
            BigInt::one().shl_bits(2047).unwrap().add(&BigInt::one()).unwrap(),
        ),
        (base32.negate(), BigInt::from_i64(7)),
        (base32.negate(), BigInt::from_i64(-7)),
        (BigInt::zero(), BigInt::from_i64(-12345)),
    ];

    for (a, b) in edge_pairs {
        let (q, r) = a.div_rem_euclid(&b).unwrap();
        let recon = q.mul(&b).unwrap().add(&r).unwrap();
        assert_eq!(recon, a, "identity failed for a={a}, b={b}");
        assert!(!r.is_negative(), "remainder must be >= 0 for a={a}, b={b}");
        assert!(r < b.abs(), "remainder must be < |b| for a={a}, b={b}");
    }

    // Random property test up to 4,096 bits (128 limbs of 32 bits).
    for _ in 0..1_500 {
        let a = rng.next_bigint_limbs(128);
        let mut b = rng.next_bigint_limbs(64);
        if b.is_zero() {
            b = BigInt::one();
        }
        let (q, r) = a.div_rem_euclid(&b).unwrap();
        let recon = q.mul(&b).unwrap().add(&r).unwrap();
        assert_eq!(recon, a);
        assert!(!r.is_negative());
        assert!(r < b.abs());
    }
}

#[test]
fn bigint_limit() {
    // Two 9,000-bit integers: each <= 16,384 bits, product is ~18,000 bits -> limits.exceeded.
    let a = BigInt::one().shl_bits(9_000).unwrap();
    let b = BigInt::one().shl_bits(9_000).unwrap();
    let err = a.mul(&b).unwrap_err();
    assert!(matches!(err, ArithError::LimitsExceeded { .. }));
    assert_eq!(err.rule(), RULE_LIMITS_EXCEEDED);

    // Shift beyond 16,384 bits.
    let err_shl = BigInt::one()
        .shl_bits(COMPILED_LIMITS.max_bigint_bits)
        .unwrap_err();
    assert_eq!(err_shl.rule(), RULE_LIMITS_EXCEEDED);

    // Decimal string representing > 16,384 bits (~4,933+ digits).
    let huge_dec = format!("1{}", "0".repeat(5_000));
    let err_parse = BigInt::parse_strict(&huge_dec).unwrap_err();
    assert_eq!(err_parse.rule(), RULE_LIMITS_EXCEEDED);

    // Power beyond bit limit.
    let err_pow = BigInt::from_u64(3).pow_u32(11_000).unwrap_err();
    assert_eq!(err_pow.rule(), RULE_LIMITS_EXCEEDED);
}

#[test]
fn po_augur_02_differential_1m() {
    // PO-AUGUR-02: 1,000,000 seeded random integer & rational operations cross-checked
    // against the independent base-10,000 schoolbook reference implementation, PLUS
    // every number from the fixture certificates.
    let mut rng = Rng::new(0xA060_2026_0000_0002);
    let mut ops_checked: usize = 0;

    // 1. Fixture certificate numbers (stance specimen, BD-026, N12-03).
    let fixture_rats = [
        "0",
        "1",
        "-1",
        "1/2",
        "1/3",
        "3/14",
        "14/3",
        "47/10",
        "6",
        "15",
        "90",
        "110",
        "130",
        "160",
        "165",
        "841/800",
        "9007199254740991",
        "9007199254740992",
        "-9007199254740992",
    ];
    let parsed_fixtures: Vec<(Rational, RefRat)> = fixture_rats
        .iter()
        .map(|s| {
            let r = Rational::parse_slash_strict(s).unwrap();
            let rr = RefRat::new(ref_from_bigint(r.num()), ref_from_bigint(r.den()));
            (r, rr)
        })
        .collect();
    for (a, ra) in &parsed_fixtures {
        for (b, rb) in &parsed_fixtures {
            let sum = a.add(b).unwrap();
            let rsum = ra.add(rb);
            assert_eq!(sum.num().to_decimal_string(), rsum.n.to_decimal());
            assert_eq!(sum.den().to_decimal_string(), rsum.d.to_decimal());

            let diff = a.sub(b).unwrap();
            let rdiff = ra.sub(rb);
            assert_eq!(diff.num().to_decimal_string(), rdiff.n.to_decimal());
            assert_eq!(diff.den().to_decimal_string(), rdiff.d.to_decimal());

            let prod = a.mul(b).unwrap();
            let rprod = ra.mul(rb);
            assert_eq!(prod.num().to_decimal_string(), rprod.n.to_decimal());
            assert_eq!(prod.den().to_decimal_string(), rprod.d.to_decimal());

            assert_eq!(a.cmp(b), ra.cmp(rb));
            ops_checked += 4;

            if !b.is_zero() {
                let quot = a.div(b).unwrap();
                let rquot = ra.div(rb);
                assert_eq!(quot.num().to_decimal_string(), rquot.n.to_decimal());
                assert_eq!(quot.den().to_decimal_string(), rquot.d.to_decimal());
                ops_checked += 1;
            }
        }
    }

    // 2. High-volume seeded random differential corpus reaching >= 1,000,000 checked ops.
    // Tier A: 980,000 fast integer & rational ops across 1..=2-limb signed values
    // checked against RefInt/RefRat (base-10,000 independent implementation).
    for i in 0..98_000_usize {
        let a_i64 = (rng.next_u64() as i64) >> 14; // signed ~50-bit
        let b_i64 = (rng.next_u64() as i64) >> 14;
        let b_nz = if b_i64 == 0 { 1 } else { b_i64 };

        let a = BigInt::from_i64(a_i64);
        let b = BigInt::from_i64(b_nz);
        let ra = RefInt::from_i128(i128::from(a_i64));
        let rb = RefInt::from_i128(i128::from(b_nz));

        // 5 BigInt operations per iteration.
        let sum = a.add(&b).unwrap();
        let diff = a.sub(&b).unwrap();
        let prod = a.mul(&b).unwrap();
        let (q, r) = a.div_rem_euclid(&b).unwrap();
        let g = a.gcd(&b).unwrap();

        let rsum = ra.add(&rb);
        let rdiff = ra.sub(&rb);
        let rprod = ra.mul(&rb);
        let (rq, rr) = ra.div_rem_euclid(&rb);
        let rg = ra.gcd(&rb);

        // Compare via exact i128 round-trip for speed on Tier A, and via exact decimal
        // strings periodically + on every Tier B/C op.
        if i % 64 == 0 {
            assert_eq!(sum.to_decimal_string(), rsum.to_decimal());
            assert_eq!(diff.to_decimal_string(), rdiff.to_decimal());
            assert_eq!(prod.to_decimal_string(), rprod.to_decimal());
            assert_eq!(q.to_decimal_string(), rq.to_decimal());
            assert_eq!(r.to_decimal_string(), rr.to_decimal());
            assert_eq!(g.to_decimal_string(), rg.to_decimal());
        } else {
            assert_eq!(sum, bigint_from_ref(&rsum));
            assert_eq!(diff, bigint_from_ref(&rdiff));
            assert_eq!(prod, bigint_from_ref(&rprod));
            assert_eq!(q, bigint_from_ref(&rq));
            assert_eq!(r, bigint_from_ref(&rr));
            assert_eq!(g, bigint_from_ref(&rg));
        }

        // 5 Rational operations per iteration (total 10 ops per iteration = 980,000 ops).
        let d1 = ((rng.next_u64() % 100_000) + 1) as i64;
        let d2 = ((rng.next_u64() % 100_000) + 1) as i64;
        let n1 = ((rng.next_u64() as i64) >> 32) % 1_000_000;
        let mut n2 = ((rng.next_u64() as i64) >> 32) % 1_000_000;
        if n2 == 0 {
            n2 = 1;
        }
        let x = Rational::from_i64_pair(n1, d1).unwrap();
        let y = Rational::from_i64_pair(n2, d2).unwrap();
        let rx = RefRat::new(RefInt::from_i128(i128::from(n1)), RefInt::from_i128(i128::from(d1)));
        let ry = RefRat::new(RefInt::from_i128(i128::from(n2)), RefInt::from_i128(i128::from(d2)));

        let x_add_y = x.add(&y).unwrap();
        let rx_add_y = rx.add(&ry);
        assert_eq!(x_add_y.num(), &bigint_from_ref(&rx_add_y.n));
        assert_eq!(x_add_y.den(), &bigint_from_ref(&rx_add_y.d));

        let x_sub_y = x.sub(&y).unwrap();
        let rx_sub_y = rx.sub(&ry);
        assert_eq!(x_sub_y.num(), &bigint_from_ref(&rx_sub_y.n));
        assert_eq!(x_sub_y.den(), &bigint_from_ref(&rx_sub_y.d));

        let x_mul_y = x.mul(&y).unwrap();
        let rx_mul_y = rx.mul(&ry);
        assert_eq!(x_mul_y.num(), &bigint_from_ref(&rx_mul_y.n));
        assert_eq!(x_mul_y.den(), &bigint_from_ref(&rx_mul_y.d));

        let x_div_y = x.div(&y).unwrap();
        let rx_div_y = rx.div(&ry);
        assert_eq!(x_div_y.num(), &bigint_from_ref(&rx_div_y.n));
        assert_eq!(x_div_y.den(), &bigint_from_ref(&rx_div_y.d));

        assert_eq!(x.cmp(&y), rx.cmp(&ry));
        ops_checked += 10;
    }

    // Tier B: 20,000 multi-limb (up to 256-bit) ops verified via decimal strings against RefInt/RefRat.
    for _ in 0..4_000_usize {
        let a = rng.next_bigint_limbs(8);
        let mut b = rng.next_bigint_limbs(6);
        if b.is_zero() {
            b = BigInt::one();
        }
        let ra = ref_from_bigint(&a);
        let rb = ref_from_bigint(&b);

        assert_eq!(a.add(&b).unwrap().to_decimal_string(), ra.add(&rb).to_decimal());
        assert_eq!(a.sub(&b).unwrap().to_decimal_string(), ra.sub(&rb).to_decimal());
        assert_eq!(a.mul(&b).unwrap().to_decimal_string(), ra.mul(&rb).to_decimal());
        let (q, r) = a.div_rem_euclid(&b).unwrap();
        let (rq, rr) = ra.div_rem_euclid(&rb);
        assert_eq!(q.to_decimal_string(), rq.to_decimal());
        assert_eq!(r.to_decimal_string(), rr.to_decimal());
        assert_eq!(a.gcd(&b).unwrap().to_decimal_string(), ra.gcd(&rb).to_decimal());
        ops_checked += 5;
    }

    assert!(
        ops_checked >= 1_000_000,
        "PO-AUGUR-02 requires >= 1,000,000 checked ops, got {ops_checked}"
    );
}

fn bigint_from_ref(r: &RefInt) -> BigInt {
    if r.len == 0 {
        return BigInt::zero();
    }
    // Convert base-10,000 digits to BigInt via u128 when <= 32 digits (128 decimal digits -> fallback decimal).
    if r.len <= 9 {
        let mut v = 0_u128;
        for i in (0..r.len).rev() {
            v = v * u128::from(REF_BASE) + u128::from(r.d[i]);
        }
        let mag = BigInt::from_u128(v);
        if r.neg { mag.negate() } else { mag }
    } else {
        BigInt::parse_strict(&r.to_decimal()).unwrap()
    }
}

#[test]
fn rational_field_laws() {
    let mut rng = Rng::new(0xF1E1_D1A0_0001_0002);
    for _ in 0..1_000 {
        let a = rng.next_small_rational(10_000, 500);
        let b = rng.next_small_rational(10_000, 500);
        let c = rng.next_small_rational(10_000, 500);

        // Addition associativity and commutativity.
        assert_eq!(
            a.add(&b).unwrap().add(&c).unwrap(),
            a.add(&b.add(&c).unwrap()).unwrap()
        );
        assert_eq!(a.add(&b).unwrap(), b.add(&a).unwrap());
        assert_eq!(a.add(&a.negate()).unwrap(), Rational::zero());

        // Multiplication associativity, commutativity, distributivity.
        assert_eq!(
            a.mul(&b).unwrap().mul(&c).unwrap(),
            a.mul(&b.mul(&c).unwrap()).unwrap()
        );
        assert_eq!(a.mul(&b).unwrap(), b.mul(&a).unwrap());
        assert_eq!(
            a.mul(&b.add(&c).unwrap()).unwrap(),
            a.mul(&b).unwrap().add(&a.mul(&c).unwrap()).unwrap()
        );

        // Multiplicative inverse a * (1 / a) == 1 for a != 0.
        if !a.is_zero() {
            let inv = a.recip().unwrap();
            assert_eq!(a.mul(&inv).unwrap(), Rational::one());
            assert_eq!(a.div(&a).unwrap(), Rational::one());
        }
    }
}

#[test]
fn rational_order() {
    assert!(Rational::from_i64_pair(14, 3).unwrap() < Rational::from_i64(5));
    assert!(Rational::from_i64_pair(841, 800).unwrap() > Rational::one());
    assert!(Rational::from_i64(165) > Rational::from_i64(160));
    assert!(Rational::from_i64_pair(-14, 3).unwrap() > Rational::from_i64(-5));
    assert_eq!(
        Rational::from_i64_pair(2, 4).unwrap(),
        Rational::from_i64_pair(1, 2).unwrap()
    );
}

#[test]
fn interval_encloses_exact() {
    // Deterministic witness first: catches nc-interval-mul-nearest immediately.
    let w_a = Interval::from_dyadic(BigInt::from_i64(1), BigInt::from_i64(1), 2).unwrap();
    let w_b = Interval::from_dyadic(BigInt::from_i64(3), BigInt::from_i64(3), 2).unwrap();
    let w_prod = w_a.mul(&w_b).unwrap();
    assert!(
        w_prod.contains_rational(&Rational::from_i64_pair(3, 16).unwrap()),
        "interval mul must enclose exact 1/4 * 3/4 = 3/16"
    );

    let mut rng = Rng::new(0x107E_0A10_0003_2026);
    for prec in [4_u32, 8, 16, 40] {
        for _ in 0..250 {
            let x1 = rng.next_small_rational(200, 40);
            let x2 = rng.next_small_rational(200, 40);
            let (x_lo, x_hi) = if x1 <= x2 { (x1, x2) } else { (x2, x1) };
            let ix = Interval::from_rational_bounds(&x_lo, &x_hi, prec).unwrap();

            let y1 = rng.next_small_rational(200, 40);
            let y2 = rng.next_small_rational(200, 40);
            let (y_lo, y_hi) = if y1 <= y2 { (y1, y2) } else { (y2, y1) };
            let iy = Interval::from_rational_bounds(&y_lo, &y_hi, prec).unwrap();

            // Pick interior rational samples x in [x_lo, x_hi], y in [y_lo, y_hi].
            let t1 = Rational::from_i64_pair((rng.next_u64() % 11) as i64, 10).unwrap();
            let t2 = Rational::from_i64_pair((rng.next_u64() % 11) as i64, 10).unwrap();
            let x = x_lo.add(&x_hi.sub(&x_lo).unwrap().mul(&t1).unwrap()).unwrap();
            let y = y_lo.add(&y_hi.sub(&y_lo).unwrap().mul(&t2).unwrap()).unwrap();
            assert!(ix.contains_rational(&x));
            assert!(iy.contains_rational(&y));

            // add, sub, negate, mul, abs, min, max.
            assert!(ix.add(&iy).unwrap().contains_rational(&x.add(&y).unwrap()));
            assert!(ix.sub(&iy).unwrap().contains_rational(&x.sub(&y).unwrap()));
            assert!(ix.negate().contains_rational(&x.negate()));
            assert!(ix.mul(&iy).unwrap().contains_rational(&x.mul(&y).unwrap()));
            assert!(ix.abs().contains_rational(&x.abs()));
            assert!(ix.min(&iy).unwrap().contains_rational(if x <= y { &x } else { &y }));
            assert!(ix.max(&iy).unwrap().contains_rational(if x >= y { &x } else { &y }));

            // div (when divisor excludes zero).
            if !iy.contains_zero() {
                assert!(ix.div(&iy).unwrap().contains_rational(&x.div(&y).unwrap()));
            } else {
                assert!(matches!(ix.div(&iy), Err(ArithError::DivisionByZero)));
            }

            // pow_u32 for even & odd powers (including intervals containing zero).
            for exp in 0..=4_u32 {
                let p_iv = ix.pow_u32(exp).unwrap();
                let p_exact = x.pow_u32(exp).unwrap();
                assert!(p_iv.contains_rational(&p_exact), "pow {exp} failed enclosure");
                if exp > 0 && exp % 2 == 0 && ix.contains_zero() {
                    assert!(p_iv.lo_grid().is_zero());
                }
            }

            // sqrt on non-negative intervals: lo^2 <= x <= hi^2.
            let nonneg = ix.abs();
            let sx = x.abs();
            let sq = nonneg.sqrt().unwrap();
            let lo_r = sq.lo_rational().unwrap();
            let hi_r = sq.hi_rational().unwrap();
            assert!(lo_r.mul(&lo_r).unwrap() <= sx);
            assert!(sx <= hi_r.mul(&hi_r).unwrap());
        }
    }
}

#[test]
fn interval_width_one_ulp() {
    let mut rng = Rng::new(0x0101_0101_ABCD_EF01);
    let prec = 24_u32;
    let ulp = Rational::new(BigInt::one(), BigInt::one().shl_bits(prec as usize).unwrap()).unwrap();

    for _ in 0..300 {
        let a = rng.next_small_rational(500, 137);
        let b = rng.next_small_rational(500, 137);

        // Point rational conversion: within < 1 ULP on each side.
        let ia = Interval::from_rational(&a, prec).unwrap();
        let ia_lo = ia.lo_rational().unwrap();
        let ia_hi = ia.hi_rational().unwrap();
        assert!(ia_lo <= a && a.sub(&ia_lo).unwrap() < ulp);
        assert!(ia_hi >= a && ia_hi.sub(&a).unwrap() < ulp);

        // Exact dyadic interval multiply: exact corners within < 1 ULP per side.
        let ib = Interval::from_rational(&b, prec).unwrap();
        let prod = ia.mul(&ib).unwrap();
        let corners = [
            ia.lo_rational().unwrap().mul(&ib.lo_rational().unwrap()).unwrap(),
            ia.lo_rational().unwrap().mul(&ib.hi_rational().unwrap()).unwrap(),
            ia.hi_rational().unwrap().mul(&ib.lo_rational().unwrap()).unwrap(),
            ia.hi_rational().unwrap().mul(&ib.hi_rational().unwrap()).unwrap(),
        ];
        let exact_lo = corners.iter().min().unwrap().clone();
        let exact_hi = corners.iter().max().unwrap().clone();
        let prod_lo = prod.lo_rational().unwrap();
        let prod_hi = prod.hi_rational().unwrap();
        assert!(prod_lo <= exact_lo && exact_lo.sub(&prod_lo).unwrap() < ulp);
        assert!(prod_hi >= exact_hi && prod_hi.sub(&exact_hi).unwrap() < ulp);

        // Division when ib excludes zero: within < 1 ULP per side.
        if !ib.contains_zero() {
            let quot = ia.div(&ib).unwrap();
            let qcorners = [
                ia.lo_rational().unwrap().div(&ib.lo_rational().unwrap()).unwrap(),
                ia.lo_rational().unwrap().div(&ib.hi_rational().unwrap()).unwrap(),
                ia.hi_rational().unwrap().div(&ib.lo_rational().unwrap()).unwrap(),
                ia.hi_rational().unwrap().div(&ib.hi_rational().unwrap()).unwrap(),
            ];
            let q_lo = qcorners.iter().min().unwrap().clone();
            let q_hi = qcorners.iter().max().unwrap().clone();
            let got_lo = quot.lo_rational().unwrap();
            let got_hi = quot.hi_rational().unwrap();
            assert!(got_lo <= q_lo && q_lo.sub(&got_lo).unwrap() < ulp);
            assert!(got_hi >= q_hi && got_hi.sub(&q_hi).unwrap() < ulp);
        }

        // Square root on non-negative interval: (lo + ulp)^2 > exact_lo and (hi - ulp)^2 < exact_hi.
        let pos = ia.abs();
        let sq = pos.sqrt().unwrap();
        let s_lo = sq.lo_rational().unwrap();
        let s_hi = sq.hi_rational().unwrap();
        let pos_lo = pos.lo_rational().unwrap();
        let pos_hi = pos.hi_rational().unwrap();
        let s_lo_plus = s_lo.add(&ulp).unwrap();
        assert!(s_lo.mul(&s_lo).unwrap() <= pos_lo);
        assert!(s_lo_plus.mul(&s_lo_plus).unwrap() > pos_lo);
        assert!(s_hi.mul(&s_hi).unwrap() >= pos_hi);
        if s_hi >= ulp {
            let s_hi_minus = s_hi.sub(&ulp).unwrap();
            assert!(s_hi_minus.mul(&s_hi_minus).unwrap() < pos_hi);
        }
    }
}

#[test]
fn interval_stance_dual() {
    // BD-026 known answer: f(v) = 90 + 15*v - 160 at precision 2^-40.
    let prec = 40_u32;
    let c90 = Interval::from_i64(90, prec).unwrap();
    let c15 = Interval::from_i64(15, prec).unwrap();
    let c160 = Interval::from_i64(160, prec).unwrap();

    // Case 1: v in [14/3, 6] -> exact min is 90 + 15*(14/3) - 160 = 0.
    // Enclosure lower end must be <= 0, so strict positivity is NOT provable.
    let v1 = Interval::from_rational_bounds(
        &Rational::from_i64_pair(14, 3).unwrap(),
        &Rational::from_i64(6),
        prec,
    )
    .unwrap();
    let f1 = c90
        .add(&c15.mul(&v1).unwrap())
        .unwrap()
        .sub(&c160)
        .unwrap();
    assert!(
        f1.lo_rational().unwrap() <= Rational::zero(),
        "lower bound on [14/3, 6] must be <= 0, got {}",
        f1.lo_rational().unwrap()
    );
    assert!(!f1.is_strictly_positive());

    // Case 2: v in [47/10, 6] -> exact min is 90 + 15*(47/10) - 160 = 1/2.
    // Computed lower end is strictly positive and > 1/2 - 2^-36.
    let v2 = Interval::from_rational_bounds(
        &Rational::from_i64_pair(47, 10).unwrap(),
        &Rational::from_i64(6),
        prec,
    )
    .unwrap();
    let f2 = c90
        .add(&c15.mul(&v2).unwrap())
        .unwrap()
        .sub(&c160)
        .unwrap();
    let half = Rational::from_i64_pair(1, 2).unwrap();
    let margin = Rational::new(BigInt::one(), BigInt::one().shl_bits(36).unwrap()).unwrap();
    let lo2 = f2.lo_rational().unwrap();
    assert!(f2.is_strictly_positive(), "f2 must be strictly positive on [47/10, 6]");
    assert!(
        lo2 <= half,
        "outward rounding must keep lower end <= exact 1/2, got {lo2}"
    );
    assert!(
        lo2 > half.sub(&margin).unwrap(),
        "lower end must be > 1/2 - 2^-36, got {lo2}"
    );
}
