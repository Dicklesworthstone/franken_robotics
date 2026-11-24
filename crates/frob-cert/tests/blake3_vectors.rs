//! BLAKE3 known-answer and official test-vector verification (BD-026, KA-223).

use std::fs;
use std::path::PathBuf;

use frob_cert::blake3::{
    Hasher, KEY_LEN, derive_key_xof, domain_hash_hex, hash_hex, hash_xof, keyed_hash_xof, to_hex,
};
use frob_cert::json::{JsonValue, parse_non_canonical};
use frob_cert::limits::COMPILED_LIMITS;

#[test]
fn blake3_empty() {
    let expected = "af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262";
    assert_eq!(hash_hex(b""), expected);
    assert_eq!(Hasher::new().finalize_hex(), expected);
}

#[test]
fn blake3_vectors() {
    let vector_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("vectors")
        .join("blake3_official_vectors.json");
    let raw = fs::read(&vector_path)
        .unwrap_or_else(|e| panic!("failed to read {}: {e}", vector_path.display()));
    let root = parse_non_canonical(&raw, &COMPILED_LIMITS).unwrap();

    let key_str = root.get("key").and_then(JsonValue::as_str).unwrap();
    let key_bytes: [u8; KEY_LEN] = key_str
        .as_bytes()
        .try_into()
        .expect("official BLAKE3 test vector key must be 32 bytes");
    let context_str = root
        .get("context_string")
        .and_then(JsonValue::as_str)
        .unwrap();
    let cases = root.get("cases").and_then(JsonValue::as_arr).unwrap();
    assert!(
        cases.len() >= 30,
        "expected full official BLAKE3 vector suite, got {} cases",
        cases.len()
    );

    let step_sizes = [1_usize, 7, 63, 64, 65, 1023, 1024, 1025];

    for (idx, case) in cases.iter().enumerate() {
        let input_len = case.get("input_len").and_then(JsonValue::as_i64).unwrap() as usize;
        let expected_hash = case.get("hash").and_then(JsonValue::as_str).unwrap();
        let expected_keyed = case.get("keyed_hash").and_then(JsonValue::as_str).unwrap();
        let expected_derive = case.get("derive_key").and_then(JsonValue::as_str).unwrap();
        assert_eq!(expected_hash.len(), 262, "expected 131-byte XOF hex");

        let input: Vec<u8> = (0..input_len).map(|i| (i % 251) as u8).collect();
        let out_len = expected_hash.len() / 2;

        // 1. Plain hash (single-shot XOF + 32-byte prefix + incremental streaming).
        let mut out = vec![0_u8; out_len];
        hash_xof(&input, &mut out);
        assert_eq!(
            to_hex(&out),
            expected_hash,
            "plain BLAKE3 mismatch for input_len={input_len}"
        );
        assert_eq!(
            hash_hex(&input),
            &expected_hash[..64],
            "plain 32-byte BLAKE3 prefix mismatch for input_len={input_len}"
        );

        let step = step_sizes[idx % step_sizes.len()];
        let mut inc = Hasher::new();
        for chunk in input.chunks(step) {
            inc.update(chunk);
        }
        let mut inc_out = vec![0_u8; out_len];
        inc.finalize_xof(&mut inc_out);
        assert_eq!(
            to_hex(&inc_out),
            expected_hash,
            "streaming BLAKE3 mismatch for input_len={input_len}, step={step}"
        );

        // 2. Keyed hash (131-byte XOF).
        let mut keyed_out = vec![0_u8; out_len];
        keyed_hash_xof(&key_bytes, &input, &mut keyed_out);
        assert_eq!(
            to_hex(&keyed_out),
            expected_keyed,
            "keyed BLAKE3 mismatch for input_len={input_len}"
        );

        // 3. Derive-key (131-byte XOF).
        let mut derive_out = vec![0_u8; out_len];
        derive_key_xof(context_str, &input, &mut derive_out);
        assert_eq!(
            to_hex(&derive_out),
            expected_derive,
            "derive-key BLAKE3 mismatch for input_len={input_len}"
        );
    }

    // Domain separation across schema names.
    let canonical = br#"{"a":2,"b":1}"#;
    let d1 = domain_hash_hex("frob.design/1", canonical);
    let d2 = domain_hash_hex("frob.task/1", canonical);
    assert_ne!(d1, d2, "different schema domains must yield distinct hashes");
    assert_eq!(d1, domain_hash_hex("frob.design/1", canonical));
}
