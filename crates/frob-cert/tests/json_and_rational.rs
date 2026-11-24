//! Canonical JSON, exact integer bounds, limits, BOM, and strict rational parsing tests (BD-026).

use frob_cert::audit_bytes;
use frob_cert::bigint::BigInt;
use frob_cert::blake3::domain_hash_hex;
use frob_cert::json::{
    JsonValue, canonicalize_bytes, parse_canonical, parse_non_canonical, serialize_canonical,
};
use frob_cert::limits::{
    COMPILED_LIMITS, Limits, MAX_JSON_INT, MIN_JSON_INT, REFUSAL_ARITHMETIC_MALFORMED,
    RULE_JSON_BOM, RULE_JSON_DUPLICATE_KEY, RULE_JSON_ESCAPE, RULE_JSON_KEY_ORDER,
    RULE_JSON_SYNTAX, RULE_JSON_WHITESPACE, RULE_LIMITS_EXCEEDED, RULE_NUM_FLOAT_IN_BUNDLE,
    RULE_NUM_INT_FORM, RULE_NUM_INT_RANGE, RULE_NUM_RATIONAL_FORM,
};
use frob_cert::rational::Rational;

#[test]
fn json_duplicate_key_refused() {
    // Wheel-radius counterexample from BD-026 item 1b.
    let wheel = br#"{"wheel_radius_mm":60,"wheel_radius_mm":75}"#;
    let err = parse_canonical(wheel, &COMPILED_LIMITS).unwrap_err();
    assert_eq!(err.rule, RULE_JSON_DUPLICATE_KEY);

    // Equal after escape decoding: 1b must fire before 1d escape error.
    let escaped_dup = br#"{"a":1,"\u0061":2}"#;
    let err2 = parse_canonical(escaped_dup, &COMPILED_LIMITS).unwrap_err();
    assert_eq!(err2.rule, RULE_JSON_DUPLICATE_KEY);

    // Nested duplicate at depth > 1.
    let nested = br#"{"outer":{"k":1,"k":2}}"#;
    let err3 = parse_canonical(nested, &COMPILED_LIMITS).unwrap_err();
    assert_eq!(err3.rule, RULE_JSON_DUPLICATE_KEY);
    assert_eq!(err3.path, "$.outer");

    // Non-canonical mode also refuses decoded duplicate keys (including surrogate pair).
    let surrogate_dup = "{\"😀\":1,\"\\ud83d\\ude00\":2}".as_bytes();
    let err4 = parse_non_canonical(surrogate_dup, &COMPILED_LIMITS).unwrap_err();
    assert_eq!(err4.rule, RULE_JSON_DUPLICATE_KEY);

    // End-to-end audit_bytes verdict reports the same rule.
    let report = audit_bytes(wheel);
    match report.verdict {
        frob_cert::Verdict::Reject { rule, .. } => assert_eq!(rule, RULE_JSON_DUPLICATE_KEY),
        other => panic!("expected reject, got {other:?}"),
    }
}

#[test]
fn json_canonical_roundtrip() {
    // KA-222: {"a":2,"b":1} re-serializes to identical bytes.
    let canonical = br#"{"a":2,"b":1}"#;
    let tree = parse_canonical(canonical, &COMPILED_LIMITS).unwrap();
    let reser = serialize_canonical(&tree);
    assert_eq!(reser.as_bytes(), canonical);

    // Key order violation.
    let unsorted = br#"{"b":1,"a":2}"#;
    assert_eq!(
        parse_canonical(unsorted, &COMPILED_LIMITS).unwrap_err().rule,
        RULE_JSON_KEY_ORDER
    );

    // Whitespace violations (interior and trailing newline).
    assert_eq!(
        parse_canonical(br#"{ "a":2 }"#, &COMPILED_LIMITS)
            .unwrap_err()
            .rule,
        RULE_JSON_WHITESPACE
    );
    assert_eq!(
        parse_canonical(b"{\"a\":2}\n", &COMPILED_LIMITS)
            .unwrap_err()
            .rule,
        RULE_JSON_WHITESPACE
    );

    // Floating-point and exponential JSON numbers refused.
    let float_err = parse_canonical(br#"{"a":2.0}"#, &COMPILED_LIMITS).unwrap_err();
    assert_eq!(float_err.rule, RULE_NUM_FLOAT_IN_BUNDLE);
    assert_eq!(float_err.refusal, Some(REFUSAL_ARITHMETIC_MALFORMED));
    let exp_err = parse_canonical(br#"{"a":1e2}"#, &COMPILED_LIMITS).unwrap_err();
    assert_eq!(exp_err.rule, RULE_NUM_FLOAT_IN_BUNDLE);

    // Minimal string escape rules (BD-026 1d / N34-18).
    assert_eq!(
        parse_canonical(br#"{"a":"\n"}"#, &COMPILED_LIMITS)
            .unwrap_err()
            .rule,
        RULE_JSON_ESCAPE
    );
    assert_eq!(
        parse_canonical(br#"{"a":"\/"}"#, &COMPILED_LIMITS)
            .unwrap_err()
            .rule,
        RULE_JSON_ESCAPE
    );
    assert_eq!(
        parse_canonical(br#"{"a":"\u0061"}"#, &COMPILED_LIMITS)
            .unwrap_err()
            .rule,
        RULE_JSON_ESCAPE
    );
    assert_eq!(
        parse_canonical(br#"{"a":"\u000A"}"#, &COMPILED_LIMITS)
            .unwrap_err()
            .rule,
        RULE_JSON_ESCAPE
    );
    let min_esc = "{\"a\":\"\\u000a\",\"b\":\"\\\"\\\\\",\"c\":\"é\u{007f}😀\"}".as_bytes();
    let parsed_min = parse_canonical(min_esc, &COMPILED_LIMITS).unwrap();
    assert_eq!(serialize_canonical(&parsed_min).as_bytes(), min_esc);

    // Unicode code-point / UTF-8 byte order beyond BMP: U+FFFF < U+10000.
    let bmp_order = "{\"\\u{FFFF}\":1,\"\\u{10000}\":2}"
        .replace("\\u{FFFF}", "\u{FFFF}")
        .replace("\\u{10000}", "\u{10000}");
    let bmp_rev = "{\"\\u{10000}\":2,\"\\u{FFFF}\":1}"
        .replace("\\u{FFFF}", "\u{FFFF}")
        .replace("\\u{10000}", "\u{10000}");
    let parsed_bmp = parse_canonical(bmp_order.as_bytes(), &COMPILED_LIMITS).unwrap();
    assert_eq!(serialize_canonical(&parsed_bmp), bmp_order);
    assert_eq!(
        parse_canonical(bmp_rev.as_bytes(), &COMPILED_LIMITS)
            .unwrap_err()
            .rule,
        RULE_JSON_KEY_ORDER
    );

    // Format-registry known answer: {"b":1,"a":2} and {"a":2,"b":1} have one identity,
    // and only the second is canonical.
    let canon_from_unsorted = canonicalize_bytes(unsorted, &COMPILED_LIMITS).unwrap();
    let canon_from_sorted = canonicalize_bytes(canonical, &COMPILED_LIMITS).unwrap();
    assert_eq!(canon_from_unsorted, canonical);
    assert_eq!(canon_from_sorted, canonical);
    assert_eq!(
        domain_hash_hex("frob.test/1", &canon_from_unsorted),
        domain_hash_hex("frob.test/1", &canon_from_sorted)
    );

    // Metamorphic relation: parse -> serialize -> parse yields equal tree and equal bytes.
    let mut rng = 0x517C_C1B7_2722_0A95_u64;
    for _ in 0..300 {
        let val = random_json_value(&mut rng, 0);
        let bytes = serialize_canonical(&val).into_bytes();
        let tree1 = parse_canonical(&bytes, &COMPILED_LIMITS).unwrap();
        let bytes2 = serialize_canonical(&tree1).into_bytes();
        let tree2 = parse_canonical(&bytes2, &COMPILED_LIMITS).unwrap();
        assert_eq!(bytes, bytes2);
        assert_eq!(tree1, tree2);
    }
}

#[test]
fn json_integer_bounds() {
    // 2^53 - 1 and -(2^53 - 1) accepted.
    let max_ok = format!("{{\"n\":{MAX_JSON_INT}}}");
    let min_ok = format!("{{\"n\":{MIN_JSON_INT}}}");
    assert_eq!(
        parse_canonical(max_ok.as_bytes(), &COMPILED_LIMITS)
            .unwrap()
            .get("n")
            .and_then(JsonValue::as_i64),
        Some(9_007_199_254_740_991)
    );
    assert_eq!(
        parse_canonical(min_ok.as_bytes(), &COMPILED_LIMITS)
            .unwrap()
            .get("n")
            .and_then(JsonValue::as_i64),
        Some(-9_007_199_254_740_991)
    );

    // 2^53 and -2^53 as JSON numbers refused with num.int-range.
    let over = br#"{"n":9007199254740992}"#;
    let err = parse_canonical(over, &COMPILED_LIMITS).unwrap_err();
    assert_eq!(err.rule, RULE_NUM_INT_RANGE);
    assert_eq!(err.refusal, Some(REFUSAL_ARITHMETIC_MALFORMED));

    let under = br#"{"n":-9007199254740992}"#;
    assert_eq!(
        parse_canonical(under, &COMPILED_LIMITS).unwrap_err().rule,
        RULE_NUM_INT_RANGE
    );

    // -0 and leading zero refused.
    assert_eq!(
        parse_canonical(br#"{"n":-0}"#, &COMPILED_LIMITS)
            .unwrap_err()
            .rule,
        RULE_NUM_INT_FORM
    );
    assert_eq!(
        parse_canonical(br#"{"n":01}"#, &COMPILED_LIMITS)
            .unwrap_err()
            .rule,
        RULE_JSON_SYNTAX
    );

    // Same value 9007199254740992 as a string parses to exact BigInt 2^53.
    let as_str = br#"{"n":"9007199254740992"}"#;
    let parsed = parse_canonical(as_str, &COMPILED_LIMITS).unwrap();
    let big = parsed.get("n").unwrap().parse_bigint().unwrap();
    assert_eq!(big, BigInt::from_u64(1_u64 << 53));
    assert_eq!(big.to_decimal_string(), "9007199254740992");
}

#[test]
fn json_limits_and_bom() {
    // UTF-8 BOM refused.
    let mut with_bom = vec![0xEF, 0xBB, 0xBF];
    with_bom.extend_from_slice(br#"{"a":1}"#);
    assert_eq!(
        parse_canonical(&with_bom, &COMPILED_LIMITS).unwrap_err().rule,
        RULE_JSON_BOM
    );

    // Nesting depth 64 accepted, depth 65 refused.
    let depth64 = format!("{}{}", "[".repeat(64), "]".repeat(64));
    assert!(parse_canonical(depth64.as_bytes(), &COMPILED_LIMITS).is_ok());

    let depth65 = format!("{}{}", "[".repeat(65), "]".repeat(65));
    assert_eq!(
        parse_canonical(depth65.as_bytes(), &COMPILED_LIMITS)
            .unwrap_err()
            .rule,
        RULE_LIMITS_EXCEEDED
    );

    // File size limit and numeric entry limit enforced.
    let tight = Limits {
        max_bigint_bits: 16_384,
        max_numeric_entries: 3,
        max_json_depth: 64,
        max_file_bytes: 16,
    };
    assert_eq!(
        parse_canonical(br#"[1,2,3,4]"#, &tight).unwrap_err().rule,
        RULE_LIMITS_EXCEEDED
    );
    assert_eq!(
        parse_canonical(br#"{"long_key":"0123456789"}"#, &tight)
            .unwrap_err()
            .rule,
        RULE_LIMITS_EXCEEDED
    );
}

#[test]
fn rational_forms() {
    // Accepted pair forms.
    let r_pair = parse_canonical(br#"["3","14"]"#, &COMPILED_LIMITS)
        .unwrap()
        .parse_rational_pair()
        .unwrap();
    assert_eq!(r_pair, Rational::from_i64_pair(3, 14).unwrap());
    assert_eq!(r_pair.to_pair_strings(), ("3".to_string(), "14".to_string()));
    assert_eq!(r_pair.to_slash_string(), "3/14");

    assert_eq!(
        Rational::parse_pair_strict("-3", "14").unwrap(),
        Rational::from_i64_pair(-3, 14).unwrap()
    );
    assert_eq!(Rational::parse_pair_strict("0", "1").unwrap(), Rational::zero());

    // Accepted slash forms.
    assert_eq!(
        Rational::parse_slash_strict("14/3").unwrap(),
        Rational::from_i64_pair(14, 3).unwrap()
    );
    assert_eq!(
        Rational::parse_slash_strict("3").unwrap(),
        Rational::from_i64(3)
    );
    assert_eq!(
        Rational::parse_slash_strict("-3").unwrap(),
        Rational::from_i64(-3)
    );
    assert_eq!(
        Rational::parse_slash_strict("0").unwrap(),
        Rational::zero()
    );

    // Refused pair forms: unreduced, negative denominator, zero denominator, -0, leading zero.
    for (p, q) in [
        ("6", "28"),
        ("3", "-14"),
        ("0", "2"),
        ("0", "0"),
        ("3", "0"),
        ("-0", "1"),
        ("03", "14"),
        ("3", "014"),
    ] {
        let err = Rational::parse_pair_strict(p, q).unwrap_err();
        assert_eq!(err.rule(), RULE_NUM_RATIONAL_FORM, "pair ({p}, {q})");
        assert_eq!(err.refusal(), Some(REFUSAL_ARITHMETIC_MALFORMED));
    }

    // Refused slash forms: unreduced, whole with "/1", "-0", negative/zero den, extra slash.
    for bad in ["6/28", "3/1", "0/1", "-0", "3/0", "3/-14", "03/14", "1/2/3", ""] {
        let err = Rational::parse_slash_strict(bad).unwrap_err();
        assert_eq!(err.rule(), RULE_NUM_RATIONAL_FORM, "slash {bad:?}");
        assert_eq!(err.refusal(), Some(REFUSAL_ARITHMETIC_MALFORMED));
    }
}

fn next_u64(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

fn random_string(state: &mut u64) -> String {
    let pool = [
        'a', 'b', 'c', 'Z', '0', '"', '\\', '/', '\n', '\r', '\t', '\u{0001}', '\u{001f}', 'é',
        '\u{FFFF}', '\u{10000}',
    ];
    let len = (next_u64(state) % 6) as usize;
    let mut s = String::new();
    for _ in 0..len {
        s.push(pool[(next_u64(state) as usize) % pool.len()]);
    }
    s
}

fn random_json_value(state: &mut u64, depth: usize) -> JsonValue {
    let kind = if depth >= 4 {
        next_u64(state) % 4
    } else {
        next_u64(state) % 6
    };
    match kind {
        0 => JsonValue::Null,
        1 => JsonValue::Bool((next_u64(state) & 1) == 1),
        2 => {
            let mag = (next_u64(state) & ((1_u64 << 52) - 1)) as i64;
            let val = if (next_u64(state) & 1) == 1 { -mag } else { mag };
            JsonValue::Int(val)
        }
        3 => JsonValue::Str(random_string(state)),
        4 => {
            let len = (next_u64(state) % 4) as usize;
            let mut items = Vec::with_capacity(len);
            for _ in 0..len {
                items.push(random_json_value(state, depth + 1));
            }
            JsonValue::Arr(items)
        }
        _ => {
            let len = (next_u64(state) % 5) as usize;
            let mut keys: Vec<String> = Vec::new();
            for _ in 0..len {
                let k = random_string(state);
                if !keys.contains(&k) {
                    keys.push(k);
                }
            }
            keys.sort_by(|a, b| a.as_bytes().cmp(b.as_bytes()));
            let entries = keys
                .into_iter()
                .map(|k| (k, random_json_value(state, depth + 1)))
                .collect();
            JsonValue::Obj(entries)
        }
    }
}
