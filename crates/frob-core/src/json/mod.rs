//! Raw-byte JSON admission reader for `frob-core` (bead N34-17).
//!
//! Validates raw bytes before any map or digest is built:
//! 1. Size (`max_bytes`), valid UTF-8, and no UTF-8 BOM.
//! 2. Strict RFC 8259 grammar, no trailing data, no raw control bytes in strings, and
//!    pre-allocation enforcement of `max_depth`, `max_string`, and `max_number_len`.
//! 3. Strict `\uXXXX` escape and surrogate-pair decoding (lone surrogates refused).
//! 4. Duplicate object member rejection after escape decoding by exact code points
//!    (no Unicode normalization), O(n log n) per object.
//! 5. Raw number lexemes preserved verbatim (`Json::Num(&'a str)`) with typed accessors
//!    (`as_int53`, `as_u64str`, `as_i64str`, `as_f64_lexeme`).
//! 6. Envelope identifier validation (`validate_identifier`: non-empty, no `< 32` or `127`).
//!
//! Independent from `frob-cert`; std-only; prints nothing (L-7); zero `unsafe`; determinism D0.

#[cfg(all(
    any(
        feature = "dup-first-wins",
        feature = "dup-last-wins",
        feature = "skip-escape-decode"
    ),
    not(feature = "gauntlet-mutations")
))]
compile_error!("frob-core JSON plant features require `gauntlet-mutations`");

#[cfg(all(
    any(
        feature = "gauntlet-mutations",
        feature = "dup-first-wins",
        feature = "dup-last-wins",
        feature = "skip-escape-decode"
    ),
    not(debug_assertions)
))]
compile_error!("frob-core gauntlet mutation features are forbidden in release builds");

#[cfg(all(feature = "dup-first-wins", feature = "dup-last-wins"))]
compile_error!("`dup-first-wins` and `dup-last-wins` are mutually exclusive");

mod parser;
mod types;

pub use parser::admit;
pub use types::{
    AdmissionFault, Admitted, DEFAULT_MAX_BYTES, DEFAULT_MAX_DEPTH, DEFAULT_MAX_NUMBER_LEN,
    DuplicateMemberInfo, FaultKind, INT53_MAX, INT53_MIN, Json, Limits, Member, Span, Spanned,
    validate_identifier,
};

#[cfg(test)]
mod tests {
    use std::fmt::Write as _;

    use super::{
        AdmissionFault, Admitted, FaultKind, INT53_MAX, INT53_MIN, Json, Limits, Spanned, admit,
        validate_identifier,
    };

    fn ok(bytes: &[u8]) -> Admitted<'_> {
        admit(bytes, &Limits::default()).expect("expected Ok")
    }

    fn err(bytes: &[u8]) -> AdmissionFault {
        admit(bytes, &Limits::default()).expect_err("expected Err")
    }

    fn fault_kind(bytes: &[u8]) -> FaultKind {
        err(bytes).kind
    }

    fn plain_reserialize(node: &Spanned<'_>, out: &mut String) {
        match &node.value {
            Json::Null => out.push_str("null"),
            Json::Bool(true) => out.push_str("true"),
            Json::Bool(false) => out.push_str("false"),
            Json::Num(lexeme) => out.push_str(lexeme),
            Json::Str(s) => write_json_string(s, out),
            Json::Arr(items) => {
                out.push('[');
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        out.push(',');
                    }
                    plain_reserialize(item, out);
                }
                out.push(']');
            }
            Json::Obj(members) => {
                out.push('{');
                for (i, member) in members.iter().enumerate() {
                    if i > 0 {
                        out.push(',');
                    }
                    write_json_string(&member.name, out);
                    out.push(':');
                    plain_reserialize(&member.value, out);
                }
                out.push('}');
            }
        }
    }

    fn write_json_string(s: &str, out: &mut String) {
        out.push('"');
        for ch in s.chars() {
            match ch {
                '"' => out.push_str("\\\""),
                '\\' => out.push_str("\\\\"),
                '\u{0008}' => out.push_str("\\b"),
                '\u{000C}' => out.push_str("\\f"),
                '\n' => out.push_str("\\n"),
                '\r' => out.push_str("\\r"),
                '\t' => out.push_str("\\t"),
                c if (c as u32) < 0x20 => {
                    let _ = write!(out, "\\u{:04x}", c as u32);
                }
                c => out.push(c),
            }
        }
        out.push('"');
    }

    #[test]
    fn dup_plain() {
        let fault = err(br#"{"wheel_radius_mm": 60, "wheel_radius_mm": 75}"#);
        let dup = fault.duplicate.expect("duplicate detail");
        assert_eq!(fault.kind, FaultKind::DuplicateMember);
        assert_eq!(fault.kind.slug(), "duplicate-member");
        assert_eq!(fault.pointer, "");
        assert_eq!(fault.offset, 24);
        assert_eq!(dup.name, "wheel_radius_mm");
        assert_eq!(dup.first_offset, 1);
        assert_eq!(dup.second_offset, 24);
    }

    #[test]
    fn dup_after_escape() {
        let fault = err(br#"{"a":1,"\u0061":2}"#);
        let dup = fault.duplicate.expect("duplicate detail");
        assert_eq!(fault.kind, FaultKind::DuplicateMember);
        assert_eq!(dup.name, "a");
        assert_eq!(dup.first_offset, 1);
        assert_eq!(dup.second_offset, 7);
    }

    #[test]
    fn dup_nested_depth_8() {
        for depth in 1..=8 {
            let mut raw = String::new();
            let mut expected_pointer = String::new();
            for d in 1..depth {
                let _ = write!(raw, "{{\"k{d}\":");
                let _ = write!(expected_pointer, "/k{d}");
            }
            raw.push_str(r#"{"x":1,"\u0078":2}"#);
            raw.push_str(&"}".repeat(depth - 1));

            let fault = err(raw.as_bytes());
            let dup = fault.duplicate.expect("duplicate detail");
            assert_eq!(fault.kind, FaultKind::DuplicateMember);
            assert_eq!(fault.pointer, expected_pointer);
            assert_eq!(dup.name, "x");
            assert!(dup.first_offset < dup.second_offset);
        }
    }

    #[test]
    fn dup_in_array_element() {
        let fault = err(br#"[{"ok":1}, {"dup":1, "dup":2}]"#);
        let dup = fault.duplicate.expect("duplicate detail");
        assert_eq!(fault.kind, FaultKind::DuplicateMember);
        assert_eq!(fault.pointer, "/1");
        assert_eq!(dup.name, "dup");
        assert_eq!(dup.first_offset, 12);
        assert_eq!(dup.second_offset, 21);
    }

    #[test]
    fn dup_reports_earliest_across_nested_objects() {
        // Outer duplicate `"a"` (offset 7) occurs before nested duplicate `"b"` (offset 18),
        // even though the inner object closes first.
        let outer_first = err(br#"{"a":1,"a":{"b":1,"b":2}}"#);
        let dup = outer_first.duplicate.expect("outer duplicate detail");
        assert_eq!(outer_first.kind, FaultKind::DuplicateMember);
        assert_eq!(outer_first.pointer, "");
        assert_eq!(outer_first.offset, 7);
        assert_eq!(dup.name, "a");
        assert_eq!(dup.first_offset, 1);
        assert_eq!(dup.second_offset, 7);

        // Inner duplicate `"b"` (offset 12) occurs before outer duplicate `"a"` (offset 19).
        let inner_first = err(br#"{"a":{"b":1,"b":2},"a":3}"#);
        let dup = inner_first.duplicate.expect("inner duplicate detail");
        assert_eq!(inner_first.kind, FaultKind::DuplicateMember);
        assert_eq!(inner_first.pointer, "/a");
        assert_eq!(inner_first.offset, 12);
        assert_eq!(dup.name, "b");
        assert_eq!(dup.first_offset, 6);
        assert_eq!(dup.second_offset, 12);

        // Earlier sibling object duplicate `"x"` (offset 8) wins over later sibling `"y"` (offset 22).
        let sibling_first = err(br#"[{"x":1,"x":2},{"y":1,"y":2}]"#);
        let dup = sibling_first.duplicate.expect("sibling duplicate detail");
        assert_eq!(sibling_first.kind, FaultKind::DuplicateMember);
        assert_eq!(sibling_first.pointer, "/0");
        assert_eq!(sibling_first.offset, 8);
        assert_eq!(dup.name, "x");
        assert_eq!(dup.first_offset, 2);
        assert_eq!(dup.second_offset, 8);
    }

    #[test]
    fn no_false_duplicate() {
        let case_diff = ok(br#"{"a":1,"A":2}"#);
        assert_eq!(case_diff.value().as_object().map(<[_]>::len), Some(2));

        let nfc_vs_nfd = ok(br#"{"\u00e9":1,"e\u0301":2}"#);
        let members = nfc_vs_nfd.value().as_object().expect("object");
        assert_eq!(members.len(), 2);
        assert_eq!(members[0].name, "\u{00e9}");
        assert_eq!(members[1].name, "e\u{0301}");
    }

    #[test]
    fn int53_bounds() {
        assert_eq!(
            ok(b"9007199254740991").root().as_int53().expect("max"),
            INT53_MAX
        );
        assert_eq!(
            ok(b"-9007199254740991").root().as_int53().expect("min"),
            INT53_MIN
        );
        assert_eq!(ok(b"0").root().as_int53().expect("zero"), 0);

        for lexeme in [
            "9007199254740992",
            "9007199254740993",
            "-9007199254740992",
            "-9007199254740993",
        ] {
            let admitted = ok(lexeme.as_bytes());
            assert_eq!(admitted.value().as_num_lexeme(), Some(lexeme));
            assert_eq!(
                admitted.root().as_int53().expect_err("out of range").kind,
                FaultKind::Int53Range
            );
        }

        assert_eq!(
            ok(br#""9007199254740991""#)
                .root()
                .as_u64str()
                .expect("u64str"),
            9_007_199_254_740_991
        );
        assert_eq!(
            ok(br#""9007199254740992""#)
                .root()
                .as_u64str()
                .expect("u64str"),
            9_007_199_254_740_992
        );
        assert_eq!(
            ok(br#""9007199254740993""#)
                .root()
                .as_u64str()
                .expect("u64str"),
            9_007_199_254_740_993
        );
        assert_eq!(
            ok(br#""1""#)
                .root()
                .as_int53()
                .expect_err("wrong type")
                .kind,
            FaultKind::WrongType
        );
    }

    #[test]
    fn minus_zero_is_not_int53() {
        let admitted = ok(b"-0");
        assert_eq!(admitted.value().as_num_lexeme(), Some("-0"));
        assert_eq!(
            admitted.root().as_int53().expect_err("-0 refused").kind,
            FaultKind::Int53Range
        );
    }

    #[test]
    fn u64str_pattern() {
        assert_eq!(ok(br#""0""#).root().as_u64str().expect("zero"), 0);
        assert_eq!(
            ok(br#""18446744073709551615""#)
                .root()
                .as_u64str()
                .expect("max"),
            u64::MAX
        );
        for bad in [
            "\"018\"",
            "\"18446744073709551616\"",
            "\"-1\"",
            "\"+1\"",
            "\"\"",
        ] {
            let fault = ok(bad.as_bytes())
                .root()
                .as_u64str()
                .expect_err("bad u64str");
            assert_eq!(fault.kind, FaultKind::IntegerStringForm);
        }

        assert_eq!(
            ok(br#""-9223372036854775808""#)
                .root()
                .as_i64str()
                .expect("i64 min"),
            i64::MIN
        );
        assert_eq!(
            ok(br#""9223372036854775807""#)
                .root()
                .as_i64str()
                .expect("i64 max"),
            i64::MAX
        );
        for bad in [
            "\"-0\"",
            "\"-01\"",
            "\"9223372036854775808\"",
            "\"-9223372036854775809\"",
        ] {
            let fault = ok(bad.as_bytes())
                .root()
                .as_i64str()
                .expect_err("bad i64str");
            assert_eq!(fault.kind, FaultKind::IntegerStringForm);
        }
        assert_eq!(
            ok(b"1").root().as_u64str().expect_err("wrong type").kind,
            FaultKind::WrongType
        );
    }

    #[test]
    fn number_grammar() {
        for bad in [
            "01", "-01", "+1", ".5", "1.", "1e", "1e+", "1E-", "-", "1.2.3",
        ] {
            assert_eq!(fault_kind(bad.as_bytes()), FaultKind::NumberForm);
        }

        let exp = ok(b"1e5");
        assert_eq!(exp.value().as_num_lexeme(), Some("1e5"));
        assert_eq!(
            exp.root().as_int53().expect_err("exponent refused").kind,
            FaultKind::Int53Range
        );
        assert_eq!(exp.root().as_f64_lexeme().expect("f64"), ("1e5", 100_000.0));
        assert_eq!(
            ok(b"1e500")
                .root()
                .as_f64_lexeme()
                .expect_err("overflow")
                .kind,
            FaultKind::NumberNotFinite
        );

        // Hard-to-round decimal near f64::MAX: lexeme stays verbatim through admit.
        let hard = "1.79769313486231570814527423731704356798070567525844996598917476803157260780028538760589558632766878171540458953514382464234321326889464182768467546703537516986049910576551282076245490090389328944075868508455133942304583236903222948165808559332123348274797826204144723168738177180919299881250404026184124858368e308";
        let admitted = ok(hard.as_bytes());
        assert_eq!(admitted.value().as_num_lexeme(), Some(hard));
        assert_eq!(
            admitted.root().as_f64_lexeme().expect("f64 max"),
            (hard, f64::MAX)
        );

        // Binary64 spacing at 2^53 is two. The exact midpoint rounds to the
        // even significand at 2^53 (exponent 53 + 1023, fraction zero).
        let midpoint = "9007199254740993.0";
        let admitted = ok(midpoint.as_bytes());
        let (lexeme, value) = admitted.root().as_f64_lexeme().expect("rounded midpoint");
        assert_eq!(lexeme, midpoint);
        assert_eq!(value.to_bits(), 0x4340_0000_0000_0000);
    }

    #[test]
    fn lone_surrogate_refused() {
        for bad in [
            r#""\ud800""#,
            r#""\udfff""#,
            r#""\ud800a""#,
            r#""\ud800\u0041""#,
        ] {
            assert_eq!(fault_kind(bad.as_bytes()), FaultKind::LoneSurrogate);
        }
    }

    #[test]
    fn surrogate_pair_decodes() {
        assert_eq!(ok(br#""\ud83d\ude00""#).value().as_str(), Some("\u{1F600}"));
    }

    #[test]
    fn string_and_number_length_limits() {
        let lim = Limits {
            max_bytes: 1024,
            max_depth: 16,
            max_string: 8,
            max_number_len: 8,
        };
        assert!(admit(br#""12345678""#, &lim).is_ok());
        assert!(admit(b"12345678", &lim).is_ok());
        assert_eq!(
            admit(br#""123456789""#, &lim)
                .expect_err("string over")
                .kind,
            FaultKind::StringTooLong
        );
        assert_eq!(
            admit(br#""1234567\n9""#, &lim)
                .expect_err("escaped over")
                .kind,
            FaultKind::StringTooLong
        );
        assert_eq!(
            admit(b"123456789", &lim).expect_err("number over").kind,
            FaultKind::NumberTooLong
        );

        let ok1024 = format!("1{}", "0".repeat(1023));
        assert!(admit(ok1024.as_bytes(), &Limits::default()).is_ok());
        assert_eq!(
            fault_kind(format!("{ok1024}0").as_bytes()),
            FaultKind::NumberTooLong
        );
    }

    #[test]
    fn bom_refused() {
        let fault = err(&[0xEF, 0xBB, 0xBF, b'{', b'}']);
        assert_eq!(fault.kind, FaultKind::Bom);
        assert_eq!(fault.offset, 0);
    }

    #[test]
    fn invalid_utf8_refused() {
        let fault = err(&[b'"', b'a', 0xFF, b'"']);
        assert_eq!(fault.kind, FaultKind::NotUtf8);
        assert_eq!(fault.offset, 2);
    }

    #[test]
    fn raw_control_in_string_refused() {
        let fault = err(b"\"hello\nworld\"");
        assert_eq!(fault.kind, FaultKind::ControlInString);
        assert_eq!(fault.offset, 6);
    }

    #[test]
    fn trailing_data_refused() {
        let fault = err(b"{} true");
        assert_eq!(fault.kind, FaultKind::TrailingData);
        assert_eq!(fault.offset, 3);

        let fault = err(br#"{"a":1,"a":2} true"#);
        assert_eq!(fault.kind, FaultKind::TrailingData);
        assert_eq!(fault.offset, 14);
        let fault = err(br#"[{"a":1,"a":2},]"#);
        assert_eq!(fault.kind, FaultKind::Syntax);
        assert_eq!(fault.offset, 15);
    }

    #[test]
    fn depth_limit() {
        let depth64 = format!("{}{}", "[".repeat(64), "]".repeat(64));
        let depth65 = format!("{}{}", "[".repeat(65), "]".repeat(65));
        assert!(admit(depth64.as_bytes(), &Limits::default()).is_ok());
        let fault = err(depth65.as_bytes());
        assert_eq!(fault.kind, FaultKind::TooDeep);
        assert_eq!(fault.offset, 64);

        let raised = Limits {
            max_depth: u16::MAX,
            ..Limits::default()
        };
        let fault = admit(depth65.as_bytes(), &raised).expect_err("depth exceeds hard cap");
        assert_eq!(fault.kind, FaultKind::TooDeep);
        assert_eq!(fault.offset, 64);
        std::thread::Builder::new()
            .stack_size(256 * 1024)
            .spawn(move || {
                let left = admit(depth64.as_bytes(), &raised).expect("admit at depth cap");
                let right = admit(depth64.as_bytes(), &raised).expect("admit at depth cap");
                assert_eq!(left, right);
                assert!(left.value().eq_ordered(right.value()));
                drop((left, right));
            })
            .expect("start bounded-stack test thread")
            .join()
            .expect("compare and drop trees at depth cap");
    }

    #[test]
    fn size_limit() {
        let lim = Limits {
            max_bytes: 10,
            ..Limits::default()
        };
        let fault = admit(b"[1,2,3,4,5]", &lim).expect_err("size limit");
        assert_eq!(fault.kind, FaultKind::TooLarge);
        assert_eq!(fault.offset, 10);
    }

    #[test]
    fn identifier_rule() {
        assert!(validate_identifier("a b").is_ok());
        assert!(validate_identifier("req-01_α").is_ok());
        assert_eq!(
            validate_identifier("").expect_err("empty").kind,
            FaultKind::IdentifierEmpty
        );

        let e7 = validate_identifier("a\u{0007}").expect_err("bell");
        assert_eq!(e7.kind, FaultKind::IdentifierNotPrintable);
        assert_eq!(e7.offset, 1);
        assert_eq!(e7.code_point, Some(7));

        let e127 = validate_identifier("a\u{007f}").expect_err("del");
        assert_eq!(e127.kind, FaultKind::IdentifierNotPrintable);
        assert_eq!(e127.offset, 1);
        assert_eq!(e127.code_point, Some(127));

        let mut buf = String::with_capacity(4);
        for cp in 0u32..32 {
            buf.clear();
            buf.push('x');
            if let Some(ch) = char::from_u32(cp) {
                buf.push(ch);
            }
            let fault = validate_identifier(&buf).expect_err("control char refused");
            assert_eq!(fault.kind, FaultKind::IdentifierNotPrintable);
            assert_eq!(fault.code_point, Some(cp));
        }
    }

    #[test]
    fn respelling_admits_to_the_same_tree() {
        let a = ok(br#"{"b":1,"a":2}"#);
        let b = ok(br#"{"a":2,"b":1}"#);
        let c = ok(br#"{ "a" : 2 , "b" : 1 }"#);
        assert!(a.value().eq_unordered(b.value()));
        assert!(b.value().eq_unordered(c.value()));
        let oa = a.value().as_object().expect("object a");
        let ob = b.value().as_object().expect("object b");
        assert_eq!(oa[0].name, "b");
        assert_eq!(ob[0].name, "a");
    }

    #[test]
    fn fault_offsets_are_stable() {
        let cases: &[&[u8]] = &[
            b"",
            b"\xEF\xBB\xBF{}",
            b"\"a\xFF\"",
            br#"{"a":1,"\u0061":2}"#,
            b"[1,01]",
            b"\"unterm",
            b"\"a\tb\"",
            br#"{"k":"\ud800"}"#,
            b"{} x",
        ];
        for case in cases {
            assert_eq!(err(case), err(case));
        }
    }

    #[test]
    fn fuzz_roundtrip_smoke() {
        let seeds: &[&[u8]] = &[
            b"null",
            b"true",
            b"false",
            b"0",
            b"-0",
            b"9007199254740993",
            b"1.25e-10",
            br#""hello\n\t\"\\\u0001\ud83d\ude00""#,
            br#"{"b":[1,2,{"z":true,"a":null}],"a":"ok"}"#,
        ];
        let mut rng = 0x9E37_79B9_7F4A_7C15u64;
        let lim = Limits::default();
        let mut next = || {
            rng ^= rng << 13;
            rng ^= rng >> 7;
            rng ^= rng << 17;
            rng
        };
        let mut buf = Vec::with_capacity(128);
        let mut out = String::with_capacity(128);
        for i in 0..4096usize {
            buf.clear();
            buf.extend_from_slice(seeds[i % seeds.len()]);
            if i >= seeds.len() {
                for _ in 0..((next() % 4) + 1) {
                    let op = next() % 3;
                    if op == 0 && !buf.is_empty() {
                        let idx = (next() as usize) % buf.len();
                        buf[idx] = u8::try_from(next() & 0xFF).unwrap_or(0);
                    } else if op == 1 && buf.len() < 96 {
                        let idx = (next() as usize) % (buf.len() + 1);
                        buf.insert(idx, u8::try_from(next() & 0xFF).unwrap_or(0));
                    } else if !buf.is_empty() {
                        let idx = (next() as usize) % buf.len();
                        buf.remove(idx);
                    }
                }
            }
            if let Ok(adm) = admit(&buf, &lim) {
                out.clear();
                plain_reserialize(adm.root(), &mut out);
                let adm2 = admit(out.as_bytes(), &lim).expect("mutated Ok must re-admit");
                assert!(adm.value().eq_ordered(adm2.value()));
            }
        }
    }
}
