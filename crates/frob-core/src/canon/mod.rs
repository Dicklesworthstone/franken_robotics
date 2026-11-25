//! Canonical JSON writer from typed terms with explicit code-point sorting (bead N34-18).
//!
//! Produces deterministic D0 [`CanonicalBytes`] directly from typed terms without serializing
//! any library map or library number type. Independent from `frob-cert`; std-only; prints
//! nothing (L-7); zero `unsafe`.
//!
//! Review note 1 resolution: [`CanonInt`] (aliased as [`Int`]) is defined here in `frob-core`
//! as a self-contained arbitrary-precision signed integer on little-endian 32-bit limbs so
//! layer-0 canonical rational serialization needs no upward dependency on `frob-lie`.

#[cfg(all(
    any(
        feature = "writer-no-key-sort",
        feature = "identity-on-raw-bytes",
        feature = "utf16-sort",
        feature = "escape-all-non-ascii"
    ),
    not(feature = "gauntlet-mutations")
))]
compile_error!("frob-core canonical-writer plant features require `gauntlet-mutations`");

#[cfg(all(
    any(
        feature = "writer-no-key-sort",
        feature = "identity-on-raw-bytes",
        feature = "utf16-sort",
        feature = "escape-all-non-ascii"
    ),
    not(debug_assertions)
))]
compile_error!("frob-core canonical-writer plant features are forbidden in release builds");

#[cfg(all(feature = "writer-no-key-sort", feature = "utf16-sort"))]
compile_error!("`writer-no-key-sort` and `utf16-sort` are mutually exclusive");

mod number;
mod writer;

pub use crate::id::CanonicalBytes;
pub use number::{CanonInt, ExactDecimal, Int};
pub use writer::{
    Array, CanonError, CanonWriter, Canonical, Object, Value, canonical_identity, to_canonical,
    to_canonical_with_identity, write_admitted, write_json,
};

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;
    use std::fmt::Write as _;

    use super::{
        CanonError, CanonInt, CanonWriter, Canonical, ExactDecimal, Int, Value, canonical_identity,
        number::format_f64_shortest_into, to_canonical, write_admitted,
    };
    use crate::id::{SchemaName, identity};
    use crate::json::{INT53_MAX, INT53_MIN, Limits, admit};

    struct XorShift64(u64);

    impl XorShift64 {
        fn new(seed: u64) -> Self {
            Self(seed)
        }

        fn next_u64(&mut self) -> u64 {
            let mut x = self.0;
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            self.0 = x;
            x
        }

        fn next_usize(&mut self, bound: usize) -> usize {
            (self.next_u64() as usize) % bound
        }
    }

    #[derive(Debug, Clone)]
    enum RandTerm {
        Null,
        Bool(bool),
        Int53(i64),
        U64Str(u64),
        I64Str(i64),
        F64(f64),
        Str(String),
        Arr(Vec<RandTerm>),
        Obj(Vec<(String, RandTerm)>),
    }

    impl Canonical for RandTerm {
        fn write(&self, w: Value<'_>) -> Result<(), CanonError> {
            match self {
                Self::Null => {
                    w.null();
                    Ok(())
                }
                Self::Bool(b) => {
                    w.bool(*b);
                    Ok(())
                }
                Self::Int53(n) => w.int53(*n),
                Self::U64Str(n) => {
                    w.u64str(*n);
                    Ok(())
                }
                Self::I64Str(n) => {
                    w.i64str(*n);
                    Ok(())
                }
                Self::F64(x) => w.f64_shortest(*x),
                Self::Str(s) => {
                    w.str(s);
                    Ok(())
                }
                Self::Arr(items) => w.array(|a| {
                    for item in items {
                        a.push_term(item)?;
                    }
                    Ok(())
                }),
                Self::Obj(members) => w.object(|o| {
                    for (name, val) in members {
                        o.member_term(name, val)?;
                    }
                    Ok(())
                }),
            }
        }
    }

    fn rand_string(rng: &mut XorShift64, max_len: usize) -> String {
        const ALPHABET: &[char] = &[
            'a',
            'b',
            'B',
            '0',
            ' ',
            '"',
            '\\',
            '/',
            '\n',
            '\r',
            '\t',
            '\u{0000}',
            '\u{0001}',
            '\u{001f}',
            '\u{007f}',
            '\u{00e9}',
            '\u{4e2d}',
            '\u{ff21}',
            '\u{ffff}',
            '\u{10000}',
            '\u{1f600}',
        ];
        let len = rng.next_usize(max_len + 1);
        let mut s = String::with_capacity(len * 2);
        for _ in 0..len {
            s.push(ALPHABET[rng.next_usize(ALPHABET.len())]);
        }
        s
    }

    fn rand_finite_f64(rng: &mut XorShift64) -> f64 {
        loop {
            let bits = match rng.next_usize(8) {
                // Subnormals (exponent == 0, non-zero mantissa)
                0 => {
                    let sign = (rng.next_u64() & 1) << 63;
                    let frac = (rng.next_u64() & ((1u64 << 52) - 1)).max(1);
                    sign | frac
                }
                // Zeroes (+0.0 and -0.0)
                1 => (rng.next_u64() & 1) << 63,
                // Near boundary magnitudes (1e-7..1e-5, 1e20..1e22)
                2 => {
                    const SPECIALS: &[f64] = &[
                        0.0,
                        -0.0,
                        1.0,
                        -1.0,
                        0.1,
                        1e-6,
                        1e-7,
                        1.5e-7,
                        9.999999999999999e-7,
                        1e20,
                        1e21,
                        1.2345e22,
                        5e-324,
                        f64::MIN_POSITIVE,
                        f64::MAX,
                        f64::MIN,
                    ];
                    return SPECIALS[rng.next_usize(SPECIALS.len())];
                }
                // General random IEEE-754 bit pattern
                _ => rng.next_u64(),
            };
            let x = f64::from_bits(bits);
            if x.is_finite() {
                return x;
            }
        }
    }

    fn rand_term(rng: &mut XorShift64, depth: usize) -> RandTerm {
        let tag = if depth >= 3 {
            rng.next_usize(7)
        } else {
            rng.next_usize(9)
        };
        match tag {
            0 => RandTerm::Null,
            1 => RandTerm::Bool((rng.next_u64() & 1) == 1),
            2 => {
                let span = (INT53_MAX as u64) * 2;
                let n = ((rng.next_u64() % span) as i64) - INT53_MAX;
                RandTerm::Int53(n)
            }
            3 => RandTerm::U64Str(rng.next_u64()),
            4 => RandTerm::I64Str(rng.next_u64() as i64),
            5 => RandTerm::F64(rand_finite_f64(rng)),
            6 => RandTerm::Str(rand_string(rng, 6)),
            7 => {
                let len = rng.next_usize(5);
                let mut items = Vec::with_capacity(len);
                for _ in 0..len {
                    items.push(rand_term(rng, depth + 1));
                }
                RandTerm::Arr(items)
            }
            _ => {
                let len = rng.next_usize(5);
                let mut seen = BTreeSet::new();
                let mut members = Vec::with_capacity(len);
                for _ in 0..len {
                    let key = rand_string(rng, 4);
                    if seen.insert(key.clone()) {
                        members.push((key, rand_term(rng, depth + 1)));
                    }
                }
                RandTerm::Obj(members)
            }
        }
    }

    fn write_noncanonical_string(s: &str, out: &mut String) {
        out.push('"');
        for ch in s.chars() {
            match ch {
                '"' => out.push_str("\\\""),
                '\\' => out.push_str("\\\\"),
                '\n' => out.push_str("\\n"),
                '\r' => out.push_str("\\r"),
                '\t' => out.push_str("\\t"),
                c if (c as u32) < 0x20 => {
                    let _ = write!(out, "\\u{:04X}", c as u32);
                }
                c if c.is_ascii_alphabetic() => {
                    let _ = write!(out, "\\u{:04x}", c as u32);
                }
                c => out.push(c),
            }
        }
        out.push('"');
    }

    fn write_noncanonical_term(term: &RandTerm, out: &mut String) {
        match term {
            RandTerm::Null => out.push_str("null"),
            RandTerm::Bool(true) => out.push_str("true"),
            RandTerm::Bool(false) => out.push_str("false"),
            RandTerm::Int53(n) => {
                let _ = write!(out, "{n}");
            }
            RandTerm::U64Str(n) => {
                let _ = write!(out, "\"{n}\"");
            }
            RandTerm::I64Str(n) => {
                let _ = write!(out, "\"{n}\"");
            }
            RandTerm::F64(x) => {
                let mut b = Vec::new();
                format_f64_shortest_into(*x, &mut b).expect("finite f64");
                out.push_str(std::str::from_utf8(&b).expect("ascii"));
            }
            RandTerm::Str(s) => write_noncanonical_string(s, out),
            RandTerm::Arr(items) => {
                out.push_str("[ ");
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        out.push_str(" , ");
                    }
                    write_noncanonical_term(item, out);
                }
                out.push_str(" ]");
            }
            RandTerm::Obj(members) => {
                out.push_str("{ ");
                for (i, (k, v)) in members.iter().rev().enumerate() {
                    if i > 0 {
                        out.push_str(" , ");
                    }
                    write_noncanonical_string(k, out);
                    out.push_str(" : ");
                    write_noncanonical_term(v, out);
                }
                out.push_str(" }");
            }
        }
    }

    #[test]
    fn sorts_members_by_code_point() {
        let mut w = CanonWriter::new();
        w.value()
            .expect("root")
            .object(|o| {
                o.int53("b", 1)?;
                o.int53("a", 2)?;
                o.int53("aa", 3)?;
                o.int53("B", 4)?;
                Ok(())
            })
            .expect("object");
        let canon = w.finish().expect("finish");
        assert_eq!(canon.as_str(), r#"{"B":4,"a":2,"aa":3,"b":1}"#);

        // Acceptance line 1: both insertion orders of {a: 2, b: 1} produce {"a":2,"b":1}.
        let mut w1 = CanonWriter::new();
        w1.value()
            .expect("root")
            .object(|o| {
                o.int53("b", 1)?;
                o.int53("a", 2)?;
                Ok(())
            })
            .expect("obj1");
        let c1 = w1.finish().expect("finish1");

        let mut w2 = CanonWriter::new();
        w2.value()
            .expect("root")
            .object(|o| {
                o.int53("a", 2)?;
                o.int53("b", 1)?;
                Ok(())
            })
            .expect("obj2");
        let c2 = w2.finish().expect("finish2");

        assert_eq!(c1.as_bytes(), br#"{"a":2,"b":1}"#);
        assert_eq!(c2.as_bytes(), br#"{"a":2,"b":1}"#);
    }

    #[test]
    fn supplementary_plane_order() {
        // U+FFFF (UTF-8: EF BF BF) MUST precede U+10000 (UTF-8: F0 90 80 80).
        // Under UTF-16 sort, U+10000 (0xD800 0xDC00) wrongly precedes U+FFFF (0xFFFF).
        let bmp_max = "\u{FFFF}";
        let supp_min = "\u{10000}";
        assert!(bmp_max.as_bytes() < supp_min.as_bytes());

        let mut w = CanonWriter::new();
        w.value()
            .expect("root")
            .object(|o| {
                o.int53(supp_min, 2)?;
                o.int53(bmp_max, 1)?;
                Ok(())
            })
            .expect("object");
        let canon = w.finish().expect("finish");
        let expected = format!("{{\"{bmp_max}\":1,\"{supp_min}\":2}}");
        assert_eq!(canon.as_bytes(), expected.as_bytes());

        // Also verify U+FF21 (fullwidth A) vs U+1F600 (emoji) used in N34-21 corpus.
        let mut w2 = CanonWriter::new();
        w2.value()
            .expect("root")
            .object(|o| {
                o.int53("\u{1F600}", 2)?;
                o.int53("\u{FF21}", 1)?;
                Ok(())
            })
            .expect("object2");
        let canon2 = w2.finish().expect("finish2");
        assert_eq!(canon2.as_str(), "{\"Ａ\":1,\"😀\":2}");
    }

    #[test]
    fn minimal_escapes() {
        let cases: &[(&str, &str)] = &[
            ("\u{0001}", r#""\u0001""#),
            ("\u{0000}", r#""\u0000""#),
            ("\u{0008}", r#""\u0008""#),
            ("\t", r#""\u0009""#),
            ("\n", r#""\u000a""#),
            ("\u{000C}", r#""\u000c""#),
            ("\r", r#""\u000d""#),
            ("\u{001F}", r#""\u001f""#),
            ("\"", r#""\"""#),
            ("\\", r#""\\""#),
            ("/", r#""/""#),
            ("é", "\"é\""),
            ("\u{007F}", "\"\u{007F}\""),
            ("😀", "\"😀\""),
            ("a\nb\"c\\dé\u{007F}", "\"a\\u000ab\\\"c\\\\dé\u{007F}\""),
        ];

        for &(input, expected) in cases {
            let canon = to_canonical(input).expect("canonical string");
            assert_eq!(
                canon.as_str(),
                expected,
                "unexpected canonical escape for {input:?}"
            );
        }
    }

    #[test]
    fn int53_edges() {
        let mut w_max = CanonWriter::new();
        w_max.value().unwrap().int53(INT53_MAX).expect("2^53 - 1");
        assert_eq!(w_max.finish().unwrap().as_str(), "9007199254740991");

        let mut w_min = CanonWriter::new();
        w_min
            .value()
            .unwrap()
            .int53(INT53_MIN)
            .expect("-(2^53 - 1)");
        assert_eq!(w_min.finish().unwrap().as_str(), "-9007199254740991");

        let mut w_zero = CanonWriter::new();
        w_zero.value().unwrap().int53(0).expect("zero");
        assert_eq!(w_zero.finish().unwrap().as_str(), "0");

        for out_of_range in [INT53_MAX + 1, INT53_MIN - 1, i64::MAX, i64::MIN] {
            let mut w = CanonWriter::new();
            assert_eq!(
                w.value().unwrap().int53(out_of_range),
                Err(CanonError::Int53OutOfRange {
                    value: out_of_range
                })
            );
            assert_eq!(w.finish(), Err(CanonError::IncompleteValue));
        }

        let mut w_u64 = CanonWriter::new();
        w_u64.value().unwrap().u64str(9_007_199_254_740_992);
        assert_eq!(w_u64.finish().unwrap().as_str(), "\"9007199254740992\"");

        let mut w_u64_max = CanonWriter::new();
        w_u64_max.value().unwrap().u64str(u64::MAX);
        assert_eq!(
            w_u64_max.finish().unwrap().as_str(),
            "\"18446744073709551615\""
        );

        let mut w_i64_min = CanonWriter::new();
        w_i64_min.value().unwrap().i64str(i64::MIN);
        assert_eq!(
            w_i64_min.finish().unwrap().as_str(),
            "\"-9223372036854775808\""
        );
    }

    #[test]
    fn rational_forms() {
        let p3 = Int::from(3i64);
        let q14 = Int::from(14i64);
        let q1 = Int::from(1i64);

        let mut w_pair = CanonWriter::new();
        w_pair
            .value()
            .unwrap()
            .rational_pair(&p3, &q14)
            .expect("3/14 pair");
        assert_eq!(w_pair.finish().unwrap().as_str(), r#"["3","14"]"#);

        let mut w_slash = CanonWriter::new();
        w_slash
            .value()
            .unwrap()
            .rational_slash(&p3, &q14)
            .expect("3/14 slash");
        assert_eq!(w_slash.finish().unwrap().as_str(), r#""3/14""#);

        let mut w_int_slash = CanonWriter::new();
        w_int_slash
            .value()
            .unwrap()
            .rational_slash(&p3, &q1)
            .expect("3/1 slash");
        assert_eq!(w_int_slash.finish().unwrap().as_str(), r#""3""#);

        let mut w_int_pair = CanonWriter::new();
        w_int_pair
            .value()
            .unwrap()
            .rational_pair(&p3, &q1)
            .expect("3/1 pair");
        assert_eq!(w_int_pair.finish().unwrap().as_str(), r#"["3","1"]"#);

        // Zero numerator is reduced only with denominator 1.
        let zero = CanonInt::zero();
        let mut w_zero = CanonWriter::new();
        w_zero
            .value()
            .unwrap()
            .rational_slash(&zero, &q1)
            .expect("0/1");
        assert_eq!(w_zero.finish().unwrap().as_str(), r#""0""#);

        // Unreduced (6, 28) and (0, 2) must be refused, never silently reduced.
        let p6 = Int::from(6i64);
        let q28 = Int::from(28i64);
        let q2 = Int::from(2i64);
        let mut w_unred = CanonWriter::new();
        assert_eq!(
            w_unred.value().unwrap().rational_pair(&p6, &q28),
            Err(CanonError::UnreducedRational)
        );
        let mut w_unred_slash = CanonWriter::new();
        assert_eq!(
            w_unred_slash.value().unwrap().rational_slash(&p6, &q28),
            Err(CanonError::UnreducedRational)
        );
        let mut w_unred_zero = CanonWriter::new();
        assert_eq!(
            w_unred_zero.value().unwrap().rational_pair(&zero, &q2),
            Err(CanonError::UnreducedRational)
        );

        // Negative and zero denominators must be refused.
        let neg14 = Int::from(-14i64);
        let mut w_neg_q = CanonWriter::new();
        assert_eq!(
            w_neg_q.value().unwrap().rational_pair(&p3, &neg14),
            Err(CanonError::NonPositiveDenominator)
        );
        let mut w_zero_q = CanonWriter::new();
        assert_eq!(
            w_zero_q.value().unwrap().rational_slash(&p3, &zero),
            Err(CanonError::NonPositiveDenominator)
        );

        // Negative numerator with coprime positive denominator succeeds.
        let neg3 = Int::from(-3i64);
        let mut w_neg_p = CanonWriter::new();
        w_neg_p
            .value()
            .unwrap()
            .rational_slash(&neg3, &q14)
            .expect("-3/14");
        assert_eq!(w_neg_p.finish().unwrap().as_str(), r#""-3/14""#);
    }

    #[test]
    fn f64_layout() {
        let cases: &[(f64, &str)] = &[
            (0.1, "0.1"),
            (1e-6, "0.000001"),
            (1e-7, "1e-7"),
            (1e20, "100000000000000000000"),
            (1e21, "1e+21"),
            (5e-324, "5e-324"),
            (1.5e-7, "1.5e-7"),
            (1.2345e22, "1.2345e+22"),
            (-0.0, "-0"),
            (0.0, "0"),
            (1.0, "1"),
            (-1.0, "-1"),
            (-1e-6, "-0.000001"),
            (-1e-7, "-1e-7"),
            (-1e21, "-1e+21"),
        ];

        for &(x, expected) in cases {
            let mut w = CanonWriter::new();
            w.value().unwrap().f64_shortest(x).expect("finite f64");
            let canon = w.finish().expect("finish");
            assert_eq!(canon.as_str(), expected, "unexpected layout for {x:?}");
            let reparsed: f64 = canon.as_str().parse().expect("valid f64");
            assert_eq!(reparsed.to_bits(), x.to_bits());
        }

        for non_finite in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            let mut w = CanonWriter::new();
            assert_eq!(
                w.value().unwrap().f64_shortest(non_finite),
                Err(CanonError::NonFiniteFloat)
            );
        }
    }

    #[test]
    fn untyped_numbers_preserve_exact_decimal_value() {
        let limits = Limits::default();
        let schema = SchemaName::parse("frob.design/1").unwrap();
        let exact = admit(b"9007199254740992", &limits).unwrap();
        let bytes = write_admitted(&exact).unwrap();
        assert_eq!(bytes.as_str(), "9007199254740992");
        assert_eq!(
            canonical_identity(&schema, &exact),
            Ok(identity(&schema, &bytes))
        );

        for lexeme in ["9007199254740993", "1e-400", "1.0000000000000001"] {
            let admitted = admit(lexeme.as_bytes(), &limits).unwrap();
            assert_eq!(
                write_admitted(&admitted),
                Err(CanonError::InexactUntypedNumber)
            );
            assert_eq!(
                canonical_identity(&schema, &admitted),
                Err(CanonError::InexactUntypedNumber)
            );
        }

        let respelled = admit(b"1.00e-1", &limits).unwrap();
        assert_eq!(write_admitted(&respelled).unwrap().as_str(), "0.1");

        let mut writer = CanonWriter::new();
        writer.value().unwrap().f64_shortest(1e20).unwrap();
        let bytes = writer.finish().unwrap();
        assert_eq!(bytes.as_str(), "100000000000000000000");
        let admitted = admit(bytes.as_bytes(), &limits).unwrap();
        assert_eq!(admitted.into_canonical(), Ok(bytes));
    }

    #[test]
    fn roundtrip_random_terms() {
        let mut rng = XorShift64::new(0x0034_1800_2026_1124);
        let limits = Limits::default();
        let mut respelled = String::new();

        for idx in 0..10_000usize {
            let term = rand_term(&mut rng, 0);
            let bytes1 = to_canonical(&term).expect("canonical write");
            let admitted = admit(bytes1.as_bytes(), &limits).expect("admit canonical bytes");
            let bytes2 = write_admitted(&admitted).expect("re-write admitted");
            assert_eq!(
                bytes1.as_bytes(),
                bytes2.as_bytes(),
                "write->admit->write drifted at term {idx}"
            );

            let checked = admitted
                .into_canonical()
                .expect("canonical bytes must pass into_canonical");
            assert_eq!(checked.as_bytes(), bytes1.as_bytes());

            // Build a non-canonical respelling (extra whitespace, reversed keys, optional escapes)
            // and verify admit -> write recovers the exact same canonical bytes.
            respelled.clear();
            respelled.push(' ');
            write_noncanonical_term(&term, &mut respelled);
            respelled.push('\n');
            let adm_resp = admit(respelled.as_bytes(), &limits).expect("admit respelling");
            let canon_from_resp = write_admitted(&adm_resp).expect("canonicalize respelling");
            assert_eq!(
                canon_from_resp.as_bytes(),
                bytes1.as_bytes(),
                "admit->write of respelling drifted at term {idx}"
            );
            assert!(
                adm_resp.into_canonical().is_err(),
                "whitespace-padded respelling must fail into_canonical at term {idx}"
            );
        }
    }

    #[test]
    fn roundtrip_random_doubles() {
        let mut rng = XorShift64::new(0xF640_3418_2026_1124);
        let mut buf = Vec::with_capacity(48);

        // Explicitly include critical boundary values first.
        let boundary_values = [
            0.0_f64,
            -0.0_f64,
            5e-324_f64,
            -5e-324_f64,
            f64::from_bits(0x000f_ffff_ffff_ffff), // largest positive subnormal
            f64::from_bits(0x800f_ffff_ffff_ffff), // largest negative subnormal
            f64::MIN_POSITIVE,
            -f64::MIN_POSITIVE,
            1e-7,
            1e-6,
            1e20,
            1e21,
            f64::MAX,
            f64::MIN,
        ];
        for x in boundary_values {
            buf.clear();
            format_f64_shortest_into(x, &mut buf).expect("boundary f64");
            let s = std::str::from_utf8(&buf).expect("ascii");
            let back: f64 = s.parse().expect("parse");
            assert_eq!(back.to_bits(), x.to_bits());
        }

        // 10^6 seeded random finite doubles including subnormals and -0.
        let mut count = 0usize;
        while count < 1_000_000 {
            let bits = if count % 10 == 0 {
                // Guarantee heavy subnormal coverage (100,000 subnormals).
                let sign = (rng.next_u64() & 1) << 63;
                let frac = (rng.next_u64() & ((1u64 << 52) - 1)).max(1);
                sign | frac
            } else if count % 10_000 == 1 {
                // Periodic -0.0 and +0.0.
                (rng.next_u64() & 1) << 63
            } else {
                rng.next_u64()
            };
            let x = f64::from_bits(bits);
            if !x.is_finite() {
                continue;
            }
            buf.clear();
            format_f64_shortest_into(x, &mut buf).expect("finite f64");
            let s = std::str::from_utf8(&buf).expect("ascii");
            let back: f64 = s.parse().expect("parse");
            assert_eq!(
                back.to_bits(),
                x.to_bits(),
                "bit mismatch for {x:?} formatted as {s:?}"
            );
            count += 1;
        }
    }

    #[test]
    fn identity_changes_with_schema() {
        let mut w = CanonWriter::new();
        w.value()
            .unwrap()
            .object(|o| {
                o.int53("a", 2)?;
                o.int53("b", 1)?;
                Ok(())
            })
            .unwrap();
        let bytes = w.finish().unwrap();

        let design = SchemaName::parse("frob.design/1").unwrap();
        let task = SchemaName::parse("frob.task/1").unwrap();
        let id_design = identity(&design, &bytes);
        let id_task = identity(&task, &bytes);
        assert_ne!(id_design, id_task);
    }

    #[test]
    fn key_order_identity() {
        // Known answer KA-781: two insertion orders of {a: 2, b: 1} have one identity.
        struct Pair {
            b_first: bool,
        }
        impl Canonical for Pair {
            fn write(&self, w: Value<'_>) -> Result<(), CanonError> {
                w.object(|o| {
                    if self.b_first {
                        o.int53("b", 1)?;
                        o.int53("a", 2)?;
                    } else {
                        o.int53("a", 2)?;
                        o.int53("b", 1)?;
                    }
                    Ok(())
                })
            }
        }

        let schema = SchemaName::parse("frob.design/1").unwrap();
        let c1 = to_canonical(&Pair { b_first: true }).unwrap();
        let c2 = to_canonical(&Pair { b_first: false }).unwrap();
        assert_eq!(c1.as_bytes(), br#"{"a":2,"b":1}"#);
        assert_eq!(c1, c2);
        assert_eq!(
            canonical_identity(&schema, &Pair { b_first: true }).unwrap(),
            canonical_identity(&schema, &Pair { b_first: false }).unwrap()
        );
    }

    #[test]
    fn view_and_canonical_agree() {
        // Known answer KA-781 and negative control target for `identity-on-raw-bytes`.
        let schema = SchemaName::parse("frob.design/1").unwrap();
        let limits = Limits::default();

        let adm_swapped = admit(br#"{"b":1,"a":2}"#, &limits).expect("admit swapped");
        let adm_canon = admit(br#"{"a":2,"b":1}"#, &limits).expect("admit canon");
        let adm_spaced = admit(br#"{"a":2, "b":1}"#, &limits).expect("admit spaced");

        // Writing each admitted tree produces canonical {"a":2,"b":1} with one KA-781 identity.
        let written_swapped = write_admitted(&adm_swapped).expect("write swapped");
        let written_canon = write_admitted(&adm_canon).expect("write canon");
        let written_spaced = write_admitted(&adm_spaced).expect("write spaced");
        assert_eq!(written_swapped.as_bytes(), br#"{"a":2,"b":1}"#);
        assert_eq!(written_canon.as_bytes(), br#"{"a":2,"b":1}"#);
        assert_eq!(written_spaced.as_bytes(), br#"{"a":2,"b":1}"#);

        let expected_id = identity(&schema, &written_canon);
        assert_eq!(identity(&schema, &written_swapped), expected_id);
        assert_eq!(identity(&schema, &written_spaced), expected_id);

        // Only the canonical spelling passes `Admitted::into_canonical` unchanged.
        let passed = adm_canon
            .into_canonical()
            .expect("canonical spelling must pass into_canonical");
        assert_eq!(passed.as_bytes(), br#"{"a":2,"b":1}"#);
        assert_eq!(identity(&schema, &passed), expected_id);

        assert!(
            matches!(
                adm_swapped.into_canonical(),
                Err(CanonError::NotCanonical {
                    first_diff_offset: 2,
                    ..
                })
            ),
            "swapped key order must be refused by into_canonical"
        );
        assert!(
            matches!(
                adm_spaced.into_canonical(),
                Err(CanonError::NotCanonical {
                    first_diff_offset: 7,
                    ..
                })
            ),
            "extra whitespace must be refused by into_canonical"
        );
    }

    #[test]
    fn duplicate_member_refused() {
        let mut w = CanonWriter::new();
        let res = w.value().unwrap().object(|o| {
            o.int53("wheel_radius_mm", 60)?;
            o.int53("wheel_radius_mm", 75)?;
            Ok(())
        });
        assert_eq!(
            res,
            Err(CanonError::DuplicateMember {
                name: String::from("wheel_radius_mm")
            })
        );
        assert_eq!(w.finish(), Err(CanonError::IncompleteValue));
    }

    #[test]
    fn writer_state_guards() {
        // Finishing without writing fails closed.
        assert_eq!(
            CanonWriter::new().finish(),
            Err(CanonError::IncompleteValue)
        );

        // Dropping root Value handle without writing fails closed on reuse and finish.
        let mut w = CanonWriter::new();
        let _unused = w.value().expect("first value handle");
        assert_eq!(w.value().err(), Some(CanonError::WriterAlreadyUsed));
        assert_eq!(w.finish(), Err(CanonError::IncompleteValue));

        // Empty member closure fails with IncompleteValue rather than emitting `{"x":}`.
        let mut w_obj = CanonWriter::new();
        assert_eq!(
            w_obj
                .value()
                .unwrap()
                .object(|o| o.member("x", |_v| Ok(()))),
            Err(CanonError::IncompleteValue)
        );
        assert_eq!(w_obj.finish(), Err(CanonError::IncompleteValue));

        // Empty array element closure fails cleanly.
        let mut w_arr = CanonWriter::new();
        assert_eq!(
            w_arr.value().unwrap().array(|a| a.push(|_v| Ok(()))),
            Err(CanonError::IncompleteValue)
        );
        assert_eq!(w_arr.finish(), Err(CanonError::IncompleteValue));
    }

    #[test]
    fn failed_root_write_cannot_finish() {
        struct WritesThenFails;

        impl Canonical for WritesThenFails {
            fn write(&self, w: Value<'_>) -> Result<(), CanonError> {
                w.null();
                Err(CanonError::NonFiniteFloat)
            }
        }

        let mut writer = CanonWriter::new();
        assert_eq!(
            writer.write_term(&WritesThenFails),
            Err(CanonError::NonFiniteFloat)
        );
        assert_eq!(writer.finish(), Err(CanonError::IncompleteValue));

        let mut writer = CanonWriter::new();
        assert_eq!(
            writer.value().unwrap().term(&WritesThenFails),
            Err(CanonError::NonFiniteFloat)
        );
        assert_eq!(writer.finish(), Err(CanonError::IncompleteValue));
    }

    #[test]
    fn writer_matches_admission_depth_limit() {
        fn nested(w: Value<'_>, levels: u16, objects: bool, pair: bool) -> Result<(), CanonError> {
            match (levels, objects, pair) {
                (0, _, true) => w.rational_pair(&Int::from(1u64), &Int::from(2u64)),
                (0, _, false) => {
                    w.null();
                    Ok(())
                }
                (_, true, _) => {
                    w.object(|o| o.member("n", |v| nested(v, levels - 1, objects, pair)))
                }
                (_, false, _) => w.array(|a| a.push(|v| nested(v, levels - 1, objects, pair))),
            }
        }

        for (objects, pair) in [(false, false), (true, false), (true, true)] {
            let levels = 64 - u16::from(pair);
            let mut writer = CanonWriter::new();
            nested(writer.value().unwrap(), levels, objects, pair).unwrap();
            let bytes = writer.finish().unwrap();
            let admitted = admit(bytes.as_bytes(), &Limits::default()).unwrap();
            assert_eq!(admitted.into_canonical(), Ok(bytes));

            let mut writer = CanonWriter::new();
            assert_eq!(
                nested(writer.value().unwrap(), levels + 1, objects, pair),
                Err(CanonError::DepthLimit)
            );
            assert_eq!(writer.finish(), Err(CanonError::IncompleteValue));
        }
    }

    #[test]
    fn exact_decimal_and_multi_limb_int() {
        assert_eq!(ExactDecimal::parse("0").unwrap().as_str(), "0");
        assert_eq!(ExactDecimal::parse("-0.00").unwrap().as_str(), "0");
        assert_eq!(ExactDecimal::parse("1.0").unwrap().as_str(), "1");
        assert_eq!(ExactDecimal::parse("0.4200").unwrap().as_str(), "0.42");
        assert_eq!(ExactDecimal::parse("-1.25e-2").unwrap().as_str(), "-0.0125");
        assert_eq!(ExactDecimal::parse("1.25e4").unwrap().as_str(), "12500");

        assert!(ExactDecimal::parse_canonical("0.42").is_ok());
        assert!(ExactDecimal::parse_canonical("0.420").is_err());
        assert!(ExactDecimal::parse_canonical("1.0").is_err());
        assert!(ExactDecimal::parse_canonical("-0").is_err());
        assert!(ExactDecimal::parse("01").is_err());
        assert!(ExactDecimal::parse("+1").is_err());
        assert!(ExactDecimal::parse(".5").is_err());
        assert!(ExactDecimal::parse("1.").is_err());

        let d = ExactDecimal::parse("0.42").unwrap();
        assert_eq!(to_canonical(&d).unwrap().as_str(), "\"0.42\"");

        // > 128-bit exact integers (multi-limb decimal round-trip + exact binary GCD).
        let big_p = CanonInt::parse_decimal("123456789012345678901234567890123456789012345678903")
            .expect("big p");
        let big_q = CanonInt::parse_decimal("123456789012345678901234567890123456789012345678904")
            .expect("big q");
        assert_eq!(
            big_p.to_decimal_string(),
            "123456789012345678901234567890123456789012345678903"
        );
        assert!(big_p.is_coprime_with(&big_q));
        assert!(big_p < big_q);

        // Multiplying both by 3 breaks coprimality.
        let p3 = CanonInt::parse_decimal("370370367037037036703703703670370370367037037036709")
            .expect("3p");
        let q3 = CanonInt::parse_decimal("370370367037037036703703703670370370367037037036712")
            .expect("3q");
        assert!(!p3.is_coprime_with(&q3));

        let mut w = CanonWriter::new();
        w.value()
            .unwrap()
            .rational_slash(&big_p, &big_q)
            .expect("big coprime rational");
        assert_eq!(
            w.finish().unwrap().as_str(),
            "\"123456789012345678901234567890123456789012345678903/123456789012345678901234567890123456789012345678904\""
        );
    }
}
