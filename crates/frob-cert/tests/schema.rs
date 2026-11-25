//! Bundle schema `frob.cert/1`, format rules R1–R5, generic rejection rules 1–6,
//! Merkle root, golden round-trip, and mutant manifest tests (N12-03).

use std::fs;
use std::path::{Path, PathBuf};

use frob_cert::blake3;
use frob_cert::json::{JsonValue, parse_canonical, serialize_canonical};
use frob_cert::kinds::registry::Class;
use frob_cert::kinds::rules::{
    REFUSAL_CLASS_ABOVE_CAP, REFUSAL_FLAG_VIOLATION, REFUSAL_QUERY_MISMATCH,
    REFUSAL_SIDE_OR_FIELD, REFUSAL_UNKNOWN_PROOF_KIND, RuleId,
};
use frob_cert::limits::{COMPILED_LIMITS, REFUSAL_ARITHMETIC_MALFORMED};
use frob_cert::schema::{
    BUNDLE_SCHEMA_VERSION, bundle_bytes_leaf_digest, bundle_leaf_digest, compute_merkle_root,
    set_weakest_class, validate_bundle_bytes, validate_bundle_value, verify_bundle_set_bytes,
};
use frob_cert::{Verdict, audit_bytes, audit_bytes_with_query};

fn golden_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("golden")
}

fn load_golden(rel: &str) -> Vec<u8> {
    let p = golden_dir().join(rel);
    fs::read(&p).unwrap_or_else(|e| panic!("failed to read golden {}: {e}", p.display()))
}

fn load_stance_json() -> JsonValue {
    let bytes = load_golden("stance_inclusion.frobcert");
    parse_canonical(&bytes, &COMPILED_LIMITS).expect("stance golden must parse as canonical JSON")
}

fn load_docking_json() -> JsonValue {
    let bytes = load_golden("docking_open_threshold.frobcert");
    parse_canonical(&bytes, &COMPILED_LIMITS).expect("docking golden must parse as canonical JSON")
}

fn obj_set(obj: &mut JsonValue, key: &str, val: JsonValue) {
    let JsonValue::Obj(entries) = obj else {
        panic!("expected JsonValue::Obj");
    };
    if let Some((_, slot)) = entries.iter_mut().find(|(k, _)| k == key) {
        *slot = val;
    } else {
        entries.push((key.to_string(), val));
        entries.sort_by(|(a, _), (b, _)| a.cmp(b));
    }
}

fn obj_remove(obj: &mut JsonValue, key: &str) -> bool {
    let JsonValue::Obj(entries) = obj else {
        panic!("expected JsonValue::Obj");
    };
    let before = entries.len();
    entries.retain(|(k, _)| k != key);
    entries.len() != before
}

fn obj_get_mut<'a>(obj: &'a mut JsonValue, key: &str) -> &'a mut JsonValue {
    let JsonValue::Obj(entries) = obj else {
        panic!("expected JsonValue::Obj");
    };
    &mut entries
        .iter_mut()
        .find(|(k, _)| k == key)
        .unwrap_or_else(|| panic!("missing key {key:?}"))
        .1
}

/// Recompute `lineage.merkle_root` as a single-leaf root and serialize canonically.
fn recompute_single_merkle_and_serialize(doc: &mut JsonValue) -> Vec<u8> {
    if let JsonValue::Obj(entries) = doc {
        entries.sort_by(|(a, _), (b, _)| a.cmp(b));
    }
    if doc.get("lineage").and_then(|l| l.get("merkle_root")).is_some() {
        let leaf = bundle_leaf_digest(doc).expect("compute leaf digest");
        let hex = blake3::to_hex(&leaf);
        obj_set(obj_get_mut(doc, "lineage"), "merkle_root", JsonValue::Str(hex));
    }
    serialize_canonical(doc).into_bytes()
}

fn make_minimal_kernel_bundle(with_review: bool) -> Vec<u8> {
    let policy_class = JsonValue::Obj(vec![
        ("kind".into(), JsonValue::Str("kernel-identity".into())),
        ("randomization".into(), JsonValue::Str("none".into())),
    ]);
    let pc_hex = blake3::hash_hex(serialize_canonical(&policy_class).as_bytes());
    let domain = JsonValue::Obj(vec![
        (
            "cell".into(),
            JsonValue::Obj(vec![
                (
                    "hi".into(),
                    JsonValue::Arr(vec![JsonValue::Str("1".into()), JsonValue::Str("1".into())]),
                ),
                (
                    "lo".into(),
                    JsonValue::Arr(vec![JsonValue::Str("0".into()), JsonValue::Str("1".into())]),
                ),
            ]),
        ),
        ("chart".into(), JsonValue::Str("cayley-slp-rational".into())),
    ]);
    let assumptions = JsonValue::Arr(vec![JsonValue::Obj(vec![
        ("id".into(), JsonValue::Str("field".into())),
        ("survives".into(), JsonValue::Str("grow".into())),
        ("value".into(), JsonValue::Str("Q".into())),
    ])]);
    let quantifiers = JsonValue::Arr(vec![JsonValue::Str("forall_rationals".into())]);
    let query_obj = JsonValue::Obj(vec![
        ("assumptions".into(), assumptions.clone()),
        ("domain".into(), domain.clone()),
        ("policy_class".into(), policy_class),
        ("quantifiers".into(), quantifiers.clone()),
        ("schema".into(), JsonValue::Str("frob.kernel.query/1".into())),
    ]);
    let query_bytes = serialize_canonical(&query_obj);
    let query_hash = blake3::hash_hex(query_bytes.as_bytes());

    let mut entries: Vec<(String, JsonValue)> = vec![
        ("assumptions".into(), assumptions),
        (
            "authorization".into(),
            JsonValue::Obj(vec![
                ("execution_authorized".into(), JsonValue::Bool(false)),
                ("hardware_safety_signoff".into(), JsonValue::Bool(false)),
                ("sim_to_real_certified".into(), JsonValue::Bool(false)),
            ]),
        ),
        (
            "checker".into(),
            JsonValue::Obj(vec![
                ("build".into(), JsonValue::Str("0.0.1".into())),
                ("name".into(), JsonValue::Str("frob-verify".into())),
            ]),
        ),
        (
            "claim".into(),
            JsonValue::Obj(vec![
                ("class".into(), JsonValue::Str("Certified".into())),
                ("horizon".into(), JsonValue::Str("algebraic".into())),
                ("kind".into(), JsonValue::Str("kernel-identity".into())),
                ("quantifiers".into(), quantifiers),
            ]),
        ),
        (
            "definitions".into(),
            JsonValue::Obj(vec![
                ("clocks".into(), JsonValue::Arr(vec![])),
                ("frame".into(), JsonValue::Str("SO(3) right-hand".into())),
                ("ports_ordered".into(), JsonValue::Arr(vec![JsonValue::Str("omega".into())])),
                ("units".into(), JsonValue::Str("dimensionless".into())),
            ]),
        ),
        ("domain".into(), domain),
        ("expected_verdict".into(), JsonValue::Str("accept".into())),
        (
            "lineage".into(),
            JsonValue::Obj(vec![
                ("alpha_spend".into(), JsonValue::Arr(vec![])),
                ("merkle_root".into(), JsonValue::Str(String::new())),
            ]),
        ),
        (
            "plant".into(),
            JsonValue::Obj(vec![
                ("model".into(), JsonValue::Str("K-cay-step".into())),
                ("policy_class".into(), JsonValue::Str(pc_hex)),
            ]),
        ),
        (
            "proof".into(),
            JsonValue::Obj(vec![
                ("kind".into(), JsonValue::Str("poly-identity".into())),
                (
                    "payload".into(),
                    JsonValue::Obj(vec![("card".into(), JsonValue::Str("T-01".into()))]),
                ),
                (
                    "primitive_equations".into(),
                    JsonValue::Arr(vec![JsonValue::Str("cay(W)^T * cay(W) = I".into())]),
                ),
                ("version".into(), JsonValue::Int(1)),
            ]),
        ),
        (
            "query".into(),
            JsonValue::Obj(vec![
                ("blake3".into(), JsonValue::Str(query_hash)),
                ("bytes".into(), JsonValue::Str(query_bytes)),
            ]),
        ),
        ("schema".into(), JsonValue::Str(BUNDLE_SCHEMA_VERSION.into())),
    ];
    if with_review {
        entries.push((
            "review".into(),
            JsonValue::Obj(vec![("equations_reviewed".into(), JsonValue::Bool(false))]),
        ));
    }
    entries.sort_by(|(a, _), (b, _)| a.cmp(b));
    let mut doc = JsonValue::Obj(entries);
    recompute_single_merkle_and_serialize(&mut doc)
}

#[test]
fn specimen_parses() {
    let bytes = load_golden("stance_inclusion.frobcert");
    let validated = validate_bundle_bytes(&bytes, None, &COMPILED_LIMITS)
        .expect("normative stance specimen must pass schema and all generic rules");
    assert_eq!(validated.bundle.schema, BUNDLE_SCHEMA_VERSION);
    assert_eq!(validated.bundle.claim.kind, "family-inclusion");
    assert_eq!(validated.bundle.claim.class, Class::Certified);
    assert_eq!(validated.bundle.proof.kind, "lp-primal");
    assert_eq!(validated.bundle.proof.version, 1);
    assert!(!validated.bundle.flags.execution_authorized);
    assert!(!validated.bundle.flags.hardware_safety_signoff);
    assert!(!validated.bundle.flags.sim_to_real_certified);
    assert_eq!(
        validated.bundle.review.as_ref().map(|r| r.equations_reviewed),
        Some(false)
    );
}

#[test]
fn unknown_field_rejects() {
    let mut doc = load_stance_json();
    obj_set(&mut doc, "surprise_field", JsonValue::Str("nope".into()));
    let bytes = serialize_canonical(&doc).into_bytes();
    let err = validate_bundle_bytes(&bytes, None, &COMPILED_LIMITS).unwrap_err();
    assert_eq!(err.rule, RuleId::SchemaUnknownField);
    assert_eq!(err.path, "surprise_field");
}

#[test]
fn duplicate_key_rejects() {
    let bytes = load_golden("stance_inclusion.frobcert");
    let raw = String::from_utf8(bytes).unwrap();
    let dup = raw.replacen(
        r#""expected_verdict":"accept""#,
        r#""expected_verdict":"accept","expected_verdict":"reject""#,
        1,
    );
    let err = validate_bundle_bytes(dup.as_bytes(), None, &COMPILED_LIMITS).unwrap_err();
    assert_eq!(err.rule, RuleId::JsonDuplicateKey);
}

#[test]
fn flag_true_rejects() {
    for flag in [
        "execution_authorized",
        "hardware_safety_signoff",
        "sim_to_real_certified",
    ] {
        let mut doc = load_stance_json();
        obj_set(obj_get_mut(&mut doc, "authorization"), flag, JsonValue::Bool(true));
        let bytes = recompute_single_merkle_and_serialize(&mut doc);
        let err = validate_bundle_bytes(&bytes, None, &COMPILED_LIMITS).unwrap_err();
        assert_eq!(err.rule, RuleId::FlagsTrue);
        assert_eq!(err.refusal, Some(REFUSAL_FLAG_VIOLATION));
        assert_eq!(err.path, format!("authorization.{flag}"));
    }
}

#[test]
fn flag_missing_rejects() {
    // Deleting one flag field rejects with flags.missing (FROB-REF-833).
    for flag in [
        "execution_authorized",
        "hardware_safety_signoff",
        "sim_to_real_certified",
    ] {
        let mut doc = load_stance_json();
        assert!(obj_remove(obj_get_mut(&mut doc, "authorization"), flag));
        let bytes = recompute_single_merkle_and_serialize(&mut doc);
        let err = validate_bundle_bytes(&bytes, None, &COMPILED_LIMITS).unwrap_err();
        assert_eq!(err.rule, RuleId::FlagsMissing);
        assert_eq!(err.refusal, Some(REFUSAL_FLAG_VIOLATION));
        assert_eq!(err.path, format!("authorization.{flag}"));
    }

    // Deleting the entire authorization object also rejects with flags.missing (FROB-REF-833).
    let mut doc = load_stance_json();
    assert!(obj_remove(&mut doc, "authorization"));
    let bytes = recompute_single_merkle_and_serialize(&mut doc);
    let err = validate_bundle_bytes(&bytes, None, &COMPILED_LIMITS).unwrap_err();
    assert_eq!(err.rule, RuleId::FlagsMissing);
    assert_eq!(err.refusal, Some(REFUSAL_FLAG_VIOLATION));
    assert_eq!(err.path, "authorization");
}

#[test]
fn quantifier_reorder_rejects() {
    let mut doc = load_stance_json();
    let reordered = JsonValue::Arr(vec![
        JsonValue::Str("forall_admitted_worlds".into()),
        JsonValue::Str("exists_causal_policy_per_design".into()),
        JsonValue::Str("forall_admitted_executions".into()),
    ]);
    obj_set(obj_get_mut(&mut doc, "claim"), "quantifiers", reordered);
    let bytes = recompute_single_merkle_and_serialize(&mut doc);
    let err = validate_bundle_bytes(&bytes, None, &COMPILED_LIMITS).unwrap_err();
    assert_eq!(err.rule, RuleId::QueryPrefix);
    assert_eq!(err.refusal, Some(REFUSAL_QUERY_MISMATCH));
    assert_eq!(err.path, "claim.quantifiers");
}

#[test]
fn assumption_missing_rejects() {
    let mut doc = load_stance_json();
    let first = doc.get("assumptions").unwrap().as_arr().unwrap()[0].clone();
    obj_set(&mut doc, "assumptions", JsonValue::Arr(vec![first]));
    let bytes = recompute_single_merkle_and_serialize(&mut doc);
    let err = validate_bundle_bytes(&bytes, None, &COMPILED_LIMITS).unwrap_err();
    assert_eq!(err.rule, RuleId::QueryAssumptions);
    assert_eq!(err.refusal, Some(REFUSAL_QUERY_MISMATCH));
    assert_eq!(err.path, "assumptions");
}

#[test]
fn moment_as_torque_rejects() {
    // Passing planar first moment (60, 30) in proof.payload.torque where 3D (30, -60, 0) N*m is expected.
    let mut doc = load_stance_json();
    let bad_torque = JsonValue::Obj(vec![
        ("frame".into(), JsonValue::Str("footprint centre, +z up".into())),
        ("origin".into(), JsonValue::Str("footprint centre".into())),
        ("quantity".into(), JsonValue::Str("first-moment".into())),
        ("sign".into(), JsonValue::Str("+z up right-hand".into())),
        ("unit".into(), JsonValue::Str("N*m".into())),
        (
            "value".into(),
            JsonValue::Arr(vec![
                JsonValue::Arr(vec![JsonValue::Str("60".into()), JsonValue::Str("1".into())]),
                JsonValue::Arr(vec![JsonValue::Str("30".into()), JsonValue::Str("1".into())]),
            ]),
        ),
    ]);
    obj_set(
        obj_get_mut(obj_get_mut(&mut doc, "proof"), "payload"),
        "torque",
        bad_torque,
    );
    let bytes = recompute_single_merkle_and_serialize(&mut doc);
    let err = validate_bundle_bytes(&bytes, None, &COMPILED_LIMITS).unwrap_err();
    assert_eq!(err.rule, RuleId::FieldType);
    assert_eq!(err.refusal, Some(REFUSAL_SIDE_OR_FIELD));
    assert_eq!(err.path, "proof.payload.torque");

    // Even when relabeled quantity="torque" with 3D (60, 30, 0), +z up sign convention catches it.
    let mut doc2 = load_stance_json();
    let wrong_sign = JsonValue::Obj(vec![
        ("frame".into(), JsonValue::Str("footprint centre, +z up".into())),
        ("origin".into(), JsonValue::Str("footprint centre".into())),
        ("quantity".into(), JsonValue::Str("torque".into())),
        ("sign".into(), JsonValue::Str("+z up right-hand".into())),
        ("unit".into(), JsonValue::Str("N*m".into())),
        (
            "value".into(),
            JsonValue::Arr(vec![
                JsonValue::Arr(vec![JsonValue::Str("60".into()), JsonValue::Str("1".into())]),
                JsonValue::Arr(vec![JsonValue::Str("30".into()), JsonValue::Str("1".into())]),
                JsonValue::Arr(vec![JsonValue::Str("0".into()), JsonValue::Str("1".into())]),
            ]),
        ),
    ]);
    obj_set(
        obj_get_mut(obj_get_mut(&mut doc2, "proof"), "payload"),
        "torque",
        wrong_sign,
    );
    let bytes2 = recompute_single_merkle_and_serialize(&mut doc2);
    let err2 = validate_bundle_bytes(&bytes2, None, &COMPILED_LIMITS).unwrap_err();
    assert_eq!(err2.rule, RuleId::FieldType);
    assert_eq!(err2.refusal, Some(REFUSAL_SIDE_OR_FIELD));
    assert_eq!(err2.path, "proof.payload.torque");
}

#[test]
fn class_above_assumption_rejects() {
    let mut doc = load_stance_json();
    let JsonValue::Arr(items) = obj_get_mut(&mut doc, "assumptions") else {
        panic!("assumptions array");
    };
    obj_set(&mut items[1], "class", JsonValue::Str("Estimated".into()));
    let bytes = recompute_single_merkle_and_serialize(&mut doc);
    let err = validate_bundle_bytes(&bytes, None, &COMPILED_LIMITS).unwrap_err();
    assert_eq!(err.rule, RuleId::ClassAboveInput);
    assert_eq!(err.refusal, Some(REFUSAL_CLASS_ABOVE_CAP));
    assert_eq!(err.path, "assumptions[1].class");
}

#[test]
fn rational_nonreduced_rejects() {
    let mut doc = load_stance_json();
    obj_set(
        obj_get_mut(obj_get_mut(&mut doc, "domain"), "cell"),
        "lo",
        JsonValue::Arr(vec![JsonValue::Str("2".into()), JsonValue::Str("4".into())]),
    );
    let bytes = recompute_single_merkle_and_serialize(&mut doc);
    let err = validate_bundle_bytes(&bytes, None, &COMPILED_LIMITS).unwrap_err();
    assert_eq!(err.rule, RuleId::NumRationalForm);
    assert_eq!(err.refusal, Some(REFUSAL_ARITHMETIC_MALFORMED));
}

#[test]
fn rational_negative_denominator_rejects() {
    let mut doc = load_stance_json();
    obj_set(
        obj_get_mut(obj_get_mut(&mut doc, "domain"), "cell"),
        "lo",
        JsonValue::Arr(vec![JsonValue::Str("3".into()), JsonValue::Str("-14".into())]),
    );
    let bytes = recompute_single_merkle_and_serialize(&mut doc);
    let err = validate_bundle_bytes(&bytes, None, &COMPILED_LIMITS).unwrap_err();
    assert_eq!(err.rule, RuleId::NumRationalForm);
    assert_eq!(err.refusal, Some(REFUSAL_ARITHMETIC_MALFORMED));
}

fn load_six_entry_bytes() -> Vec<Vec<u8>> {
    [
        "six_entry/e1_inclusion_c03.frobcert",
        "six_entry/e2_inclusion_c06a.frobcert",
        "six_entry/e3_exclusion_c01_c04.frobcert",
        "six_entry/e4_exclusion_c07_c12.frobcert",
        "six_entry/e5_partition_box.frobcert",
        "six_entry/e6_qualification_receipt.frobcert",
    ]
    .iter()
    .map(|rel| load_golden(rel))
    .collect()
}

#[test]
fn merkle_root_recomputed() {
    let files = load_six_entry_bytes();
    let slices: Vec<&[u8]> = files.iter().map(Vec::as_slice).collect();
    let summary = verify_bundle_set_bytes(&slices, false).expect("six-entry set must verify");
    assert!(summary.merkle_recomputed);
    assert!(summary.rejection.is_none(), "unexpected set rejection: {:?}", summary.rejection);
    assert_eq!(summary.level_widths, vec![6, 3, 2, 1]);

    // Single bundle alone reports recomputed = false (does not fail on set root).
    let single = verify_bundle_set_bytes(&[slices[0]], false).expect("single bundle");
    assert!(!single.merkle_recomputed);
    assert!(single.rejection.is_none());
}

#[test]
fn merkle_single_leaf() {
    let bytes = load_golden("stance_inclusion.frobcert");
    let leaf = bundle_bytes_leaf_digest(&bytes).expect("leaf digest");
    let tree = compute_merkle_root(&[leaf]).expect("single leaf tree");
    assert_eq!(tree.root, leaf);
    assert_eq!(tree.level_widths, vec![1]);

    let summary = verify_bundle_set_bytes(&[bytes.as_slice()], true).expect("single-claim set");
    assert!(summary.merkle_recomputed);
    assert!(summary.rejection.is_none());
    assert_eq!(summary.merkle_root.as_deref(), Some(tree.root_hex.as_str()));
}

#[test]
fn open_threshold_roundtrip() {
    let bytes = load_golden("docking_open_threshold.frobcert");
    let validated = validate_bundle_bytes(&bytes, None, &COMPILED_LIMITS)
        .expect("docking open threshold (eps < 5/16 at U = 3/4) must validate");
    let cb = validated.bundle.claim.cost_bounds.as_ref().expect("cost_bounds");
    assert!(!cb.attained);
    assert!(cb.strict);
    assert_eq!(cb.bound.as_ref().unwrap().to_slash_string(), "5/16");
    assert_eq!(cb.candidate.as_ref().unwrap().to_slash_string(), "1/4");
    assert_eq!(cb.gap.as_ref().unwrap().to_slash_string(), "1/16");

    // Exact canonical roundtrip
    let parsed = parse_canonical(&bytes, &COMPILED_LIMITS).unwrap();
    assert_eq!(serialize_canonical(&parsed).into_bytes(), bytes);
}

#[test]
fn attained_minimum_requires_lower_bound() {
    // Attained stance minimum b = 3/14 m with matching lower_bound = 3/14 m passes R3.
    let bytes = load_golden("stance_inclusion.frobcert");
    let validated = validate_bundle_bytes(&bytes, None, &COMPILED_LIMITS)
        .expect("attained minimum with matching lower bound must pass");
    let cb = validated.bundle.claim.cost_bounds.as_ref().expect("cost_bounds");
    assert!(cb.attained);
    assert_eq!(cb.incumbent.as_ref().unwrap().to_slash_string(), "3/14");
    assert_eq!(cb.lower_bound.as_ref().unwrap().to_slash_string(), "3/14");

    // Claiming attained=true with a non-matching lower bound rejects with claim.minimum-unbounded.
    let mut doc = load_stance_json();
    obj_set(
        obj_get_mut(obj_get_mut(&mut doc, "claim"), "cost_bounds"),
        "lower_bound",
        JsonValue::Arr(vec![JsonValue::Str("1".into()), JsonValue::Str("5".into())]),
    );
    let bad_bytes = recompute_single_merkle_and_serialize(&mut doc);
    let err = validate_bundle_bytes(&bad_bytes, None, &COMPILED_LIMITS).unwrap_err();
    assert_eq!(err.rule, RuleId::ClaimMinimumUnbounded);
}

#[test]
fn domain_change_rejects() {
    let mut doc = load_stance_json();
    obj_set(
        obj_get_mut(obj_get_mut(&mut doc, "domain"), "cell"),
        "lo",
        JsonValue::Arr(vec![JsonValue::Str("1".into()), JsonValue::Str("5".into())]),
    );
    let bytes = recompute_single_merkle_and_serialize(&mut doc);
    let err = validate_bundle_bytes(&bytes, None, &COMPILED_LIMITS).unwrap_err();
    assert_eq!(err.rule, RuleId::QueryDomain);
    assert_eq!(err.refusal, Some(REFUSAL_QUERY_MISMATCH));
    assert_eq!(err.path, "domain");
}

#[test]
fn merkle_change_one_entry_changes_root() {
    let mut files = load_six_entry_bytes();
    let orig_slices: Vec<&[u8]> = files.iter().map(Vec::as_slice).collect();
    let orig = verify_bundle_set_bytes(&orig_slices, false).unwrap();
    let orig_root = orig.merkle_root.clone().unwrap();

    // Mutate one entry (e1 checker.build = "0.0.2") without changing its declared merkle_root.
    let mut e1_doc = parse_canonical(&files[0], &COMPILED_LIMITS).unwrap();
    obj_set(
        obj_get_mut(&mut e1_doc, "checker"),
        "build",
        JsonValue::Str("0.0.2".into()),
    );
    files[0] = serialize_canonical(&e1_doc).into_bytes();

    let mod_slices: Vec<&[u8]> = files.iter().map(Vec::as_slice).collect();
    let modified = verify_bundle_set_bytes(&mod_slices, false).unwrap();
    assert!(modified.merkle_recomputed);
    assert_ne!(modified.merkle_root.as_deref(), Some(orig_root.as_str()));
    let rej = modified.rejection.expect("mutated set must fail hash.merkle-root");
    assert_eq!(rej.rule, RuleId::HashMerkleRoot);
}

#[test]
fn merkle_odd_level_carries_last_node() {
    let files = load_six_entry_bytes();
    let leaves: Vec<[u8; 32]> = files
        .iter()
        .map(|b| bundle_bytes_leaf_digest(b).unwrap())
        .collect();
    let tree = compute_merkle_root(&leaves).unwrap();
    assert_eq!(tree.level_widths, vec![6, 3, 2, 1]);

    // Manually verify the odd-level carry (level of 3 nodes carries node[2] up unchanged).
    let s = &tree.leaf_digests_sorted;
    let hash_inner = |l: &[u8; 32], r: &[u8; 32]| {
        let mut buf = [0u8; 65];
        buf[0] = 0x01;
        buf[1..33].copy_from_slice(l);
        buf[33..65].copy_from_slice(r);
        blake3::hash(&buf)
    };
    let n0 = hash_inner(&s[0], &s[1]);
    let n1 = hash_inner(&s[2], &s[3]);
    let n2 = hash_inner(&s[4], &s[5]);
    // Level 2 has 2 nodes: H(0x01 || n0 || n1) and n2 carried up unchanged.
    let m0 = hash_inner(&n0, &n1);
    let expected_root = hash_inner(&m0, &n2);
    assert_eq!(tree.root, expected_root);
}

#[test]
fn merkle_duplicate_leaf_rejects() {
    let b = load_golden("stance_inclusion.frobcert");
    let leaf = bundle_bytes_leaf_digest(&b).unwrap();
    let err = compute_merkle_root(&[leaf, leaf]).unwrap_err();
    assert_eq!(err.rule, RuleId::HashMerkleDuplicate);
}

#[test]
fn set_class_is_weakest() {
    let files = load_six_entry_bytes();
    let slices: Vec<&[u8]> = files.iter().map(Vec::as_slice).collect();
    let summary = verify_bundle_set_bytes(&slices, false).unwrap();
    assert_eq!(summary.class, Some(Class::Tested));
    assert_eq!(summary.weakest_entry, Some(5));

    let cls = set_weakest_class(&[
        Class::Certified,
        Class::Certified,
        Class::Certified,
        Class::Certified,
        Class::Certified,
        Class::Tested,
    ]);
    assert_eq!(cls, Some((Class::Tested, 5)));
}

#[test]
fn rule_order_flags_before_kind() {
    let mut doc = load_stance_json();
    obj_set(
        obj_get_mut(&mut doc, "authorization"),
        "execution_authorized",
        JsonValue::Bool(true),
    );
    obj_set(
        obj_get_mut(&mut doc, "proof"),
        "kind",
        JsonValue::Str("lp-primal-fast".into()),
    );
    let bytes = recompute_single_merkle_and_serialize(&mut doc);
    let err = validate_bundle_bytes(&bytes, None, &COMPILED_LIMITS).unwrap_err();
    assert_eq!(err.rule, RuleId::FlagsTrue);
    assert_eq!(err.refusal, Some(REFUSAL_FLAG_VIOLATION));
}

#[test]
fn equations_missing_rejects() {
    // Empty primitive_equations array -> side.equations-missing (FROB-REF-829).
    let mut doc = load_stance_json();
    obj_set(
        obj_get_mut(&mut doc, "proof"),
        "primitive_equations",
        JsonValue::Arr(vec![]),
    );
    let bytes = recompute_single_merkle_and_serialize(&mut doc);
    let err = validate_bundle_bytes(&bytes, None, &COMPILED_LIMITS).unwrap_err();
    assert_eq!(err.rule, RuleId::SideEquationsMissing);
    assert_eq!(err.refusal, Some(REFUSAL_SIDE_OR_FIELD));

    // Absent primitive_equations key -> side.equations-missing (FROB-REF-829).
    let mut doc2 = load_stance_json();
    assert!(obj_remove(obj_get_mut(&mut doc2, "proof"), "primitive_equations"));
    let bytes2 = recompute_single_merkle_and_serialize(&mut doc2);
    let err2 = validate_bundle_bytes(&bytes2, None, &COMPILED_LIMITS).unwrap_err();
    assert_eq!(err2.rule, RuleId::SideEquationsMissing);
    assert_eq!(err2.refusal, Some(REFUSAL_SIDE_OR_FIELD));
}

#[test]
fn minimum_without_lower_bound_rejects() {
    // Stance minimum without lower_bound -> claim.minimum-unbounded.
    let mut doc = load_stance_json();
    assert!(obj_remove(
        obj_get_mut(obj_get_mut(&mut doc, "claim"), "cost_bounds"),
        "lower_bound"
    ));
    let bytes = recompute_single_merkle_and_serialize(&mut doc);
    let err = validate_bundle_bytes(&bytes, None, &COMPILED_LIMITS).unwrap_err();
    assert_eq!(err.rule, RuleId::ClaimMinimumUnbounded);

    // Docking open threshold 5/16 claimed as attained -> claim.minimum-unbounded (CX-114).
    let mut dock = load_docking_json();
    obj_set(
        obj_get_mut(&mut dock, "claim"),
        "cost_bounds",
        JsonValue::Obj(vec![
            ("attained".into(), JsonValue::Bool(true)),
            (
                "incumbent".into(),
                JsonValue::Arr(vec![JsonValue::Str("5".into()), JsonValue::Str("16".into())]),
            ),
            (
                "lower_bound".into(),
                JsonValue::Arr(vec![JsonValue::Str("5".into()), JsonValue::Str("16".into())]),
            ),
            ("strict".into(), JsonValue::Bool(true)),
            ("unit".into(), JsonValue::Str("m".into())),
        ]),
    );
    let dock_bytes = recompute_single_merkle_and_serialize(&mut dock);
    let dock_err = validate_bundle_bytes(&dock_bytes, None, &COMPILED_LIMITS).unwrap_err();
    assert_eq!(dock_err.rule, RuleId::ClaimMinimumUnbounded);
}

#[test]
fn mass_without_weighting_rejects() {
    let mut doc = load_stance_json();
    obj_set(
        obj_get_mut(&mut doc, "claim"),
        "unresolved_mass",
        JsonValue::Obj(vec![(
            "value".into(),
            JsonValue::Arr(vec![JsonValue::Str("1".into()), JsonValue::Str("10".into())]),
        )]),
    );
    let bytes = recompute_single_merkle_and_serialize(&mut doc);
    let err = validate_bundle_bytes(&bytes, None, &COMPILED_LIMITS).unwrap_err();
    assert_eq!(err.rule, RuleId::ClaimMassWithoutWeighting);
}

#[test]
fn interval_without_declaration_rejects() {
    let mut doc = load_stance_json();
    obj_set(
        obj_get_mut(obj_get_mut(&mut doc, "proof"), "payload"),
        "margin_interval",
        JsonValue::Obj(vec![
            (
                "hi".into(),
                JsonValue::Arr(vec![JsonValue::Str("1".into()), JsonValue::Str("2".into())]),
            ),
            (
                "lo".into(),
                JsonValue::Arr(vec![JsonValue::Str("3".into()), JsonValue::Str("14".into())]),
            ),
        ]),
    );
    let bytes = recompute_single_merkle_and_serialize(&mut doc);
    let err = validate_bundle_bytes(&bytes, None, &COMPILED_LIMITS).unwrap_err();
    assert_eq!(err.rule, RuleId::NumIntervalDecl);
    assert_eq!(err.refusal, Some(REFUSAL_ARITHMETIC_MALFORMED));
}

#[test]
fn integer_above_2_53_as_number_rejects() {
    let bytes = load_golden("stance_inclusion.frobcert");
    let raw = String::from_utf8(bytes).unwrap();
    // 2^53 = 9007199254740992 exceeds ±(2^53 - 1) and must travel as a string, never a JSON number.
    let bad = raw.replacen(r#""vertices":4"#, r#""vertices":9007199254740992"#, 1);
    let err = validate_bundle_bytes(bad.as_bytes(), None, &COMPILED_LIMITS).unwrap_err();
    assert_eq!(err.rule, RuleId::NumIntRange);
    assert_eq!(err.refusal, Some(REFUSAL_ARITHMETIC_MALFORMED));
}

#[test]
fn certified_without_receipt_rejects() {
    let mut doc = load_stance_json();
    obj_set(
        obj_get_mut(&mut doc, "checker"),
        "build",
        JsonValue::Str(String::new()),
    );
    let bytes = recompute_single_merkle_and_serialize(&mut doc);
    let err = validate_bundle_bytes(&bytes, None, &COMPILED_LIMITS).unwrap_err();
    assert_eq!(err.rule, RuleId::ClassNoReceipt);
    assert_eq!(err.refusal, Some(REFUSAL_CLASS_ABOVE_CAP));

    let mut doc2 = load_stance_json();
    obj_set(
        obj_get_mut(&mut doc2, "lineage"),
        "merkle_root",
        JsonValue::Str(String::new()),
    );
    let bytes2 = serialize_canonical(&doc2).into_bytes();
    let err2 = validate_bundle_bytes(&bytes2, None, &COMPILED_LIMITS).unwrap_err();
    assert_eq!(err2.rule, RuleId::ClassNoReceipt);
    assert_eq!(err2.refusal, Some(REFUSAL_CLASS_ABOVE_CAP));
}

fn collect_frobcert_files(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(dir).unwrap() {
        let p = entry.unwrap().path();
        if p.is_dir() {
            if p.file_name().and_then(|s| s.to_str()) == Some("mutants") {
                continue;
            }
            collect_frobcert_files(&p, out);
        } else if p.extension().and_then(|s| s.to_str()) == Some("frobcert") {
            out.push(p);
        }
    }
}

#[test]
fn golden_files_roundtrip() {
    let mut files = Vec::new();
    collect_frobcert_files(&golden_dir(), &mut files);
    files.sort();
    assert_eq!(files.len(), 8, "expected 8 golden bundles (stance, docking, 6-entry)");

    for p in &files {
        let bytes = fs::read(p).unwrap();
        let parsed = parse_canonical(&bytes, &COMPILED_LIMITS)
            .unwrap_or_else(|e| panic!("golden {} failed canonical parse: {e}", p.display()));
        let reser = serialize_canonical(&parsed).into_bytes();
        assert_eq!(
            reser,
            bytes,
            "golden {} failed byte-identical round trip (PO-LEMMA-01)",
            p.display()
        );
        validate_bundle_value(&parsed, None, &COMPILED_LIMITS)
            .unwrap_or_else(|e| panic!("golden {} failed generic validation: {e}", p.display()));
    }
}

#[test]
fn review_missing_on_mission_claim_rejects() {
    let mut doc = load_stance_json();
    assert!(obj_remove(&mut doc, "review"));
    let bytes = recompute_single_merkle_and_serialize(&mut doc);
    let err = validate_bundle_bytes(&bytes, None, &COMPILED_LIMITS).unwrap_err();
    assert_eq!(err.rule, RuleId::SchemaReviewMissing);
    assert_eq!(err.path, "review");
}

#[test]
fn review_absent_on_kernel_identity_accepts() {
    let bytes = make_minimal_kernel_bundle(false);
    let validated = validate_bundle_bytes(&bytes, None, &COMPILED_LIMITS)
        .expect("kernel-identity bundle without review marker must pass generic rules");
    assert!(validated.bundle.review.is_none());
}

#[test]
fn review_true_with_wrong_digest_rejects() {
    let mut doc = load_stance_json();
    obj_set(
        &mut doc,
        "review",
        JsonValue::Obj(vec![
            ("equations_digest".into(), JsonValue::Str("0".repeat(64))),
            ("equations_reviewed".into(), JsonValue::Bool(true)),
            ("review_record".into(), JsonValue::Str(format!("rec:{}", "1".repeat(64)))),
            ("scope".into(), JsonValue::Str("stance-family".into())),
        ]),
    );
    let bytes = recompute_single_merkle_and_serialize(&mut doc);
    let err = validate_bundle_bytes(&bytes, None, &COMPILED_LIMITS).unwrap_err();
    assert_eq!(err.rule, RuleId::HashEquationsDigest);
    assert_eq!(err.path, "review.equations_digest");
}

#[test]
fn review_true_with_matching_digest_accepts() {
    let mut doc = load_stance_json();
    let eq_val = doc.get("proof").unwrap().get("primitive_equations").unwrap();
    let eq_digest = blake3::hash_hex(serialize_canonical(eq_val).as_bytes());
    obj_set(
        &mut doc,
        "review",
        JsonValue::Obj(vec![
            ("equations_digest".into(), JsonValue::Str(eq_digest.clone())),
            ("equations_reviewed".into(), JsonValue::Bool(true)),
            ("review_record".into(), JsonValue::Str(format!("rec:{}", "a".repeat(64)))),
            ("scope".into(), JsonValue::Str("stance-family".into())),
        ]),
    );
    let bytes = recompute_single_merkle_and_serialize(&mut doc);
    let validated = validate_bundle_bytes(&bytes, None, &COMPILED_LIMITS)
        .expect("matching equations_digest with equations_reviewed=true must pass");
    let rev = validated.bundle.review.as_ref().unwrap();
    assert!(rev.equations_reviewed);
    assert_eq!(rev.equations_digest.as_deref(), Some(eq_digest.as_str()));
}

#[test]
fn review_true_without_digest_rejects() {
    let mut doc = load_stance_json();
    obj_set(
        &mut doc,
        "review",
        JsonValue::Obj(vec![
            ("equations_reviewed".into(), JsonValue::Bool(true)),
            ("review_record".into(), JsonValue::Str(format!("rec:{}", "a".repeat(64)))),
            ("scope".into(), JsonValue::Str("stance-family".into())),
        ]),
    );
    let bytes = recompute_single_merkle_and_serialize(&mut doc);
    let err = validate_bundle_bytes(&bytes, None, &COMPILED_LIMITS).unwrap_err();
    assert_eq!(err.rule, RuleId::SchemaMissingField);
    assert_eq!(err.path, "review.equations_digest");
}

#[test]
fn unknown_kind_consistent_hashes_rejects() {
    // Bundle form of KA-215 / CX-106: fabricated kind lp-primal-fast with all hashes recomputed.
    // Under --features gauntlet-mutations,shortcut-accept-on-hashes this test MUST FAIL.
    let mut doc = load_stance_json();
    obj_set(
        obj_get_mut(&mut doc, "proof"),
        "kind",
        JsonValue::Str("lp-primal-fast".into()),
    );
    let bytes = recompute_single_merkle_and_serialize(&mut doc);
    let report = audit_bytes(&bytes);
    match report.verdict {
        Verdict::Reject { rule, refusal, path, .. } => {
            assert_eq!(rule, RuleId::KindUnknown.as_str());
            assert_eq!(refusal, Some(REFUSAL_UNKNOWN_PROOF_KIND));
            assert_eq!(path, "proof.kind");
        }
        other => panic!("expected Reject(kind.unknown, FROB-REF-823), got {other:?}"),
    }
}

#[test]
fn policy_class_mismatch_rejects() {
    // Synthetic: stance golden offered for a copy of its query whose policy-class section
    // differs ONLY in the arrival instant of one packet (prefix, domain, assumptions equal).
    // Under --features gauntlet-mutations,nc-omit-policy-class this test MUST FAIL.
    let bytes = load_golden("stance_inclusion.frobcert");
    let doc = load_stance_json();
    let embedded_q_str = doc.get("query").unwrap().get("bytes").unwrap().as_str().unwrap();

    // Against its own embedded query, the bundle passes generic rules (returns Unsupported at M0).
    let own_rep = audit_bytes_with_query(&bytes, Some(embedded_q_str.as_bytes()), COMPILED_LIMITS, false);
    assert!(
        own_rep.verdict.is_unsupported(),
        "against its own query, stance golden must pass Rule 4 and reach Unsupported, got {:?}",
        own_rep.verdict
    );

    // Mutate only the packet arrival instant inside policy_class.timing.arrival.
    let mut q_val = parse_canonical(embedded_q_str.as_bytes(), &COMPILED_LIMITS).unwrap();
    obj_set(
        obj_get_mut(obj_get_mut(&mut q_val, "policy_class"), "timing"),
        "arrival",
        JsonValue::Str("after-decision-late-packet".into()),
    );
    let late_q_bytes = serialize_canonical(&q_val).into_bytes();

    let rep = audit_bytes_with_query(&bytes, Some(&late_q_bytes), COMPILED_LIMITS, false);
    match rep.verdict {
        Verdict::Reject { rule, refusal, path, .. } => {
            assert_eq!(rule, RuleId::QueryPolicyClass.as_str());
            assert_eq!(refusal, Some(REFUSAL_QUERY_MISMATCH));
            assert_eq!(path, "plant.policy_class");
        }
        other => panic!("expected Reject(query.policy-class, FROB-REF-826), got {other:?}"),
    }
}

#[test]
fn policy_class_edited_rejects() {
    // Golden's plant.policy_class replaced with another digest, all other digests recomputed,
    // checked against its own embedded query -> query.policy-class (FROB-REF-826).
    let mut doc = load_stance_json();
    obj_set(
        obj_get_mut(&mut doc, "plant"),
        "policy_class",
        JsonValue::Str("f".repeat(64)),
    );
    let bytes = recompute_single_merkle_and_serialize(&mut doc);
    let err = validate_bundle_bytes(&bytes, None, &COMPILED_LIMITS).unwrap_err();
    assert_eq!(err.rule, RuleId::QueryPolicyClass);
    assert_eq!(err.refusal, Some(REFUSAL_QUERY_MISMATCH));
    assert_eq!(err.path, "plant.policy_class");
}

#[test]
fn not_implemented_kind_runs_generic_rules() {
    // At M0 lp-primal is not-implemented: golden gives Unsupported,
    // while its quantifier-reordered mutant gives Reject(query.prefix), NOT Unsupported.
    let golden_bytes = load_golden("stance_inclusion.frobcert");
    let golden_rep = audit_bytes(&golden_bytes);
    assert!(
        matches!(golden_rep.verdict, Verdict::Unsupported { ref kind, .. } if kind == "lp-primal"),
        "expected Unsupported(lp-primal), got {:?}",
        golden_rep.verdict
    );

    let mut_bytes = load_golden("mutants/reject-query.prefix-quantifiers-reordered.frobcert");
    let mut_rep = audit_bytes(&mut_bytes);
    match mut_rep.verdict {
        Verdict::Reject { rule, refusal, .. } => {
            assert_eq!(rule, RuleId::QueryPrefix.as_str());
            assert_eq!(refusal, Some(REFUSAL_QUERY_MISMATCH));
        }
        other => panic!("not-implemented kind with generic defect must Reject(query.prefix), got {other:?}"),
    }
}

#[test]
fn kind_version_required() {
    let mut doc = load_stance_json();
    assert!(obj_remove(obj_get_mut(&mut doc, "proof"), "version"));
    let bytes = recompute_single_merkle_and_serialize(&mut doc);
    let err = validate_bundle_bytes(&bytes, None, &COMPILED_LIMITS).unwrap_err();
    assert_eq!(err.rule, RuleId::SchemaMissingField);
    assert_eq!(err.path, "proof.version");
}

#[test]
fn property_single_field_deletion_rejects() {
    // Property: deleting ANY single top-level field or any required nested field from the
    // normative stance specimen is rejected by the validator.
    let base = load_stance_json();
    let JsonValue::Obj(top_entries) = &base else {
        panic!("root obj");
    };
    for (top_key, _) in top_entries {
        let mut mutated = base.clone();
        assert!(obj_remove(&mut mutated, top_key));
        let bytes = serialize_canonical(&mutated).into_bytes();
        assert!(
            validate_bundle_bytes(&bytes, None, &COMPILED_LIMITS).is_err(),
            "deleting top-level field {top_key:?} must be rejected"
        );
    }

    let required_nested: &[(&str, &str)] = &[
        ("claim", "kind"),
        ("claim", "class"),
        ("claim", "quantifiers"),
        ("claim", "horizon"),
        ("query", "blake3"),
        ("query", "bytes"),
        ("definitions", "units"),
        ("definitions", "frame"),
        ("definitions", "clocks"),
        ("definitions", "ports_ordered"),
        ("plant", "model"),
        ("plant", "policy_class"),
        ("domain", "chart"),
        ("domain", "cell"),
        ("proof", "kind"),
        ("proof", "version"),
        ("proof", "primitive_equations"),
        ("proof", "payload"),
        ("checker", "name"),
        ("checker", "build"),
        ("lineage", "merkle_root"),
        ("lineage", "alpha_spend"),
        ("review", "equations_reviewed"),
        ("authorization", "execution_authorized"),
        ("authorization", "hardware_safety_signoff"),
        ("authorization", "sim_to_real_certified"),
    ];

    for &(parent, child) in required_nested {
        let mut mutated = base.clone();
        assert!(obj_remove(obj_get_mut(&mut mutated, parent), child));
        let bytes = serialize_canonical(&mutated).into_bytes();
        assert!(
            validate_bundle_bytes(&bytes, None, &COMPILED_LIMITS).is_err(),
            "deleting required nested field {parent}.{child} must be rejected"
        );
    }
}

#[test]
fn golden_mutants_manifest_all_match() {
    let mut_dir = golden_dir().join("mutants");
    let manifest = fs::read_to_string(mut_dir.join("expected.toml"))
        .expect("expected.toml must exist in tests/golden/mutants/");

    // Parse simple [[mutant]] blocks with key = "value" rows (std-only).
    let mut rows: Vec<(String, String, String, String)> = Vec::new();
    let (mut file, mut rule, mut refusal, mut path) =
        (String::new(), String::new(), String::new(), String::new());
    let flush = |rows: &mut Vec<(String, String, String, String)>,
                 file: &mut String,
                 rule: &mut String,
                 refusal: &mut String,
                 path: &mut String| {
        if !file.is_empty() {
            rows.push((
                std::mem::take(file),
                std::mem::take(rule),
                std::mem::take(refusal),
                std::mem::take(path),
            ));
        }
    };
    for line in manifest.lines() {
        let t = line.trim();
        if t.is_empty() || t.starts_with('#') {
            continue;
        }
        if t == "[[mutant]]" {
            flush(&mut rows, &mut file, &mut rule, &mut refusal, &mut path);
            continue;
        }
        if let Some((k, v)) = t.split_once('=') {
            let k = k.trim();
            let v = v.trim().trim_matches('"').to_string();
            match k {
                "file" => file = v,
                "rule" => rule = v,
                "refusal" => refusal = v,
                "path" => path = v,
                _ => {}
            }
        }
    }
    flush(&mut rows, &mut file, &mut rule, &mut refusal, &mut path);

    assert!(rows.len() >= 20, "expected >= 20 generic mutants, found {}", rows.len());
    for (fname, exp_rule, exp_refusal, exp_path) in rows {
        let bytes = fs::read(mut_dir.join(&fname))
            .unwrap_or_else(|e| panic!("missing mutant {fname}: {e}"));
        let rep = audit_bytes(&bytes);
        match rep.verdict {
            Verdict::Reject {
                rule,
                refusal,
                path,
                message,
            } => {
                assert_eq!(
                    rule, exp_rule,
                    "mutant {fname}: expected rule {exp_rule:?}, got {rule:?} ({message})"
                );
                let exp_ref_opt = if exp_refusal.is_empty() {
                    None
                } else {
                    Some(exp_refusal.as_str())
                };
                assert_eq!(
                    refusal, exp_ref_opt,
                    "mutant {fname}: expected refusal {exp_ref_opt:?}, got {refusal:?}"
                );
                assert_eq!(
                    path, exp_path,
                    "mutant {fname}: expected path {exp_path:?}, got {path:?}"
                );
            }
            other => panic!("mutant {fname} must Reject({exp_rule}), got {other:?}"),
        }
    }
}

#[test]
fn unresolved_mass_rejects_arbitrary_object_and_out_of_range_interval() {
    // Arbitrary non-interval object in unresolved_mass.value must be rejected.
    let mut doc = load_stance_json();
    obj_set(
        obj_get_mut(&mut doc, "claim"),
        "unresolved_mass",
        JsonValue::Obj(vec![
            ("value".into(), JsonValue::Obj(vec![("foo".into(), JsonValue::Str("bar".into()))])),
            ("weighting".into(), JsonValue::Str("uniform".into())),
        ]),
    );
    let bytes = recompute_single_merkle_and_serialize(&mut doc);
    let err = validate_bundle_bytes(&bytes, None, &COMPILED_LIMITS).unwrap_err();
    assert_eq!(err.rule, RuleId::SchemaFieldType);
    assert_eq!(err.path, "claim.unresolved_mass.value");

    // Declared R5 interval outside [0, 1] must be rejected.
    let mut doc2 = load_stance_json();
    obj_set(
        obj_get_mut(&mut doc2, "claim"),
        "unresolved_mass",
        JsonValue::Obj(vec![
            (
                "value".into(),
                JsonValue::Obj(vec![
                    ("hi".into(), JsonValue::Arr(vec![JsonValue::Str("3".into()), JsonValue::Str("2".into())])),
                    ("impl".into(), JsonValue::Str("frob-cert/dyadic-1".into())),
                    ("kind".into(), JsonValue::Str("interval".into())),
                    ("lo".into(), JsonValue::Arr(vec![JsonValue::Str("0".into()), JsonValue::Str("1".into())])),
                    ("precision".into(), JsonValue::Int(64)),
                ]),
            ),
            ("weighting".into(), JsonValue::Str("uniform".into())),
        ]),
    );
    let bytes2 = recompute_single_merkle_and_serialize(&mut doc2);
    let err2 = validate_bundle_bytes(&bytes2, None, &COMPILED_LIMITS).unwrap_err();
    assert_eq!(err2.rule, RuleId::SchemaFieldType);
    assert_eq!(err2.path, "claim.unresolved_mass.value");

    // Valid declared R5 interval inside [0, 1] passes R4 & R5.
    let mut doc3 = load_stance_json();
    obj_set(
        obj_get_mut(&mut doc3, "claim"),
        "unresolved_mass",
        JsonValue::Obj(vec![
            (
                "value".into(),
                JsonValue::Obj(vec![
                    ("hi".into(), JsonValue::Arr(vec![JsonValue::Str("1".into()), JsonValue::Str("4".into())])),
                    ("impl".into(), JsonValue::Str("frob-cert/dyadic-1".into())),
                    ("kind".into(), JsonValue::Str("interval".into())),
                    ("lo".into(), JsonValue::Arr(vec![JsonValue::Str("0".into()), JsonValue::Str("1".into())])),
                    ("precision".into(), JsonValue::Int(64)),
                ]),
            ),
            ("weighting".into(), JsonValue::Str("uniform".into())),
        ]),
    );
    let bytes3 = recompute_single_merkle_and_serialize(&mut doc3);
    assert!(validate_bundle_bytes(&bytes3, None, &COMPILED_LIMITS).is_ok());
}

#[test]
fn plant_inputs_rejects_unknown_field_and_non_object() {
    let mut doc = load_stance_json();
    obj_set(
        obj_get_mut(&mut doc, "plant"),
        "inputs",
        JsonValue::Arr(vec![JsonValue::Obj(vec![
            ("class".into(), JsonValue::Str("Certified".into())),
            ("extra".into(), JsonValue::Bool(true)),
            ("id".into(), JsonValue::Str("mat-1".into())),
        ])]),
    );
    let bytes = recompute_single_merkle_and_serialize(&mut doc);
    let err = validate_bundle_bytes(&bytes, None, &COMPILED_LIMITS).unwrap_err();
    assert_eq!(err.rule, RuleId::SchemaUnknownField);
    assert_eq!(err.path, "plant.inputs[0].extra");

    let mut doc2 = load_stance_json();
    obj_set(
        obj_get_mut(&mut doc2, "plant"),
        "inputs",
        JsonValue::Arr(vec![JsonValue::Int(42)]),
    );
    let bytes2 = recompute_single_merkle_and_serialize(&mut doc2);
    let err2 = validate_bundle_bytes(&bytes2, None, &COMPILED_LIMITS).unwrap_err();
    assert_eq!(err2.rule, RuleId::SchemaFieldType);
    assert_eq!(err2.path, "plant.inputs[0]");
}

#[test]
fn nested_rationals_supports_slash_form_and_rejects_unreduced_slash() {
    let mut doc = load_stance_json();
    obj_set(
        obj_get_mut(obj_get_mut(&mut doc, "proof"), "payload"),
        "multipliers",
        JsonValue::Arr(vec![JsonValue::Str("1/2".into()), JsonValue::Str("3/4".into())]),
    );
    let bytes = recompute_single_merkle_and_serialize(&mut doc);
    assert!(validate_bundle_bytes(&bytes, None, &COMPILED_LIMITS).is_ok());

    let mut bad = load_stance_json();
    obj_set(
        obj_get_mut(obj_get_mut(&mut bad, "proof"), "payload"),
        "multipliers",
        JsonValue::Arr(vec![JsonValue::Str("1/2".into()), JsonValue::Str("2/4".into())]),
    );
    let bad_bytes = recompute_single_merkle_and_serialize(&mut bad);
    let err = validate_bundle_bytes(&bad_bytes, None, &COMPILED_LIMITS).unwrap_err();
    assert_eq!(err.rule, RuleId::NumRationalForm);
}
