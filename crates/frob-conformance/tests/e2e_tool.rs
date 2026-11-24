//! Unit tests for the `std`-only `frob-e2e` tool (`STD-E2E`).
//! Zero mocks; real files written under a scratch directory (no file deletions).

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use frob_conformance::e2e::json::{self, JsonValue, obj};
use frob_conformance::e2e::line::{
    CmpOutcome, Rational, eval_cmp, redact_private_path as redact_path, run_emit, run_exec,
    schema_json,
};
use frob_conformance::e2e::lint::{PRIVATE_CANARY, TOKEN_CANARY, lint_log_file};
use frob_conformance::e2e::report::{generate_gate_report, parse_script_header};
use frob_conformance::e2e::sha256::{sha256_hex, sha256_path};

static COUNTER: AtomicU64 = AtomicU64::new(0);

fn scratch_dir(label: &str) -> PathBuf {
    let base = env::var("BIG_SCRATCH")
        .or_else(|_| env::var("TMPDIR"))
        .unwrap_or_else(|_| "/tmp".to_string());
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = PathBuf::from(base)
        .join("frob-BlackOx")
        .join(format!("e2e-tool-{}-{n}-{label}", std::process::id()));
    fs::create_dir_all(&dir).expect("create scratch dir");
    dir
}

fn svec(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_string()).collect()
}

fn emit_basic_start(log: &Path, bead: &str, scale: &str) {
    let log_s = log.to_string_lossy().into_owned();
    run_emit(&svec(&[
        "--log",
        &log_s,
        "--event",
        "start",
        "--run-id",
        "20261124T120000Z-deadbeef",
        "--script",
        "st_fixture",
        "--bead",
        bead,
        "--step",
        "",
        "--seed",
        "1",
        "--ts",
        "2026-11-24T12:00:00.000Z",
        "--commit",
        "abc1234",
        "--dirty",
        "false",
        "--toolchain",
        "nightly-2026-08-31",
        "--frob-version",
        "0.0.1",
        "--host-os",
        "linux",
        "--host-arch",
        "x86_64",
        "--host-fingerprint",
        "fp0123456789",
        "--scale",
        scale,
        "--devices",
        "cpu",
        "--plant",
        "",
    ]))
    .expect("emit start");
}

fn emit_pass_assert(log: &Path, bead: &str, name: &str, gate: Option<&str>) {
    let log_s = log.to_string_lossy().into_owned();
    let mut args = vec![
        "--log",
        &log_s,
        "--event",
        "assert",
        "--run-id",
        "20261124T120000Z-deadbeef",
        "--script",
        "st_fixture",
        "--bead",
        bead,
        "--step",
        "s1",
        "--seed",
        "1",
        "--ts",
        "2026-11-24T12:00:01.000Z",
        "--name",
        name,
        "--kind",
        "eq",
        "--status",
        "pass",
        "--observed",
        "1",
        "--expected",
        "1",
    ];
    if let Some(g) = gate {
        args.push("--gate");
        args.push(g);
    }
    run_emit(&svec(&args)).expect("emit assert");
}

fn emit_summary(log: &Path, bead: &str, registries: &str) -> String {
    let log_s = log.to_string_lossy().into_owned();
    run_emit(&svec(&[
        "--log",
        &log_s,
        "--event",
        "summary",
        "--run-id",
        "20261124T120000Z-deadbeef",
        "--script",
        "st_fixture",
        "--bead",
        bead,
        "--step",
        "",
        "--seed",
        "1",
        "--ts",
        "2026-11-24T12:00:02.000Z",
        "--registries",
        registries,
    ]))
    .expect("emit summary")
}

// ---------------------------------------------------------------------------
// Required named unit tests from STD-E2E test plan
// ---------------------------------------------------------------------------

#[test]
fn json_roundtrip_escapes() {
    let payload = "quote:\" slash:\\ nl:\n ctrl:\u{0001} emoji:\u{1F600}";
    let orig = obj(vec![
        ("text", JsonValue::String(payload.to_string())),
        ("nested", JsonValue::Array(vec![JsonValue::String("\t\r\u{0008}\u{000C}".into())])),
    ]);
    let encoded = orig.to_json_string();
    assert!(encoded.contains("\\u0001"), "U+0001 must be escaped: {encoded}");
    assert!(!encoded.contains('\n'), "newline must be escaped in single-line JSON");
    let decoded = json::parse_strict(&encoded).expect("roundtrip parse");
    assert_eq!(decoded, orig);
    assert_eq!(decoded.pointer("/text").and_then(JsonValue::as_str), Some(payload));

    // Surrogate pair decoding for U+1F600 (\uD83D\uDE00)
    let from_surrogate = json::parse_strict(r#"{"e":"\uD83D\uDE00"}"#).expect("surrogate pair");
    assert_eq!(
        from_surrogate.pointer("/e").and_then(JsonValue::as_str),
        Some("\u{1F600}")
    );
}

#[test]
fn json_duplicate_member_refused() {
    let err = json::parse_strict(r#"{"a":1,"a":2}"#).expect_err("duplicate key must fail");
    assert!(
        err.message.contains("duplicate"),
        "unexpected error message: {err}"
    );
    let nested_err = json::parse_strict(r#"{"outer":{"x":1,"y":2,"x":3}}"#)
        .expect_err("nested duplicate key must fail");
    assert!(nested_err.message.contains("duplicate"));
}

#[test]
fn cmp_decimal_exact() {
    // Exact decimal equality without binary float rounding
    match eval_cmp("num", &["0.30", "==", "0.3"]).expect("cmp num") {
        CmpOutcome::Result { ok, .. } => assert!(ok, "0.30 == 0.3 must be true"),
        _ => panic!("expected comparison result"),
    }
    match eval_cmp("num", &["1e-3", "<", "0.0011"]).expect("cmp num") {
        CmpOutcome::Result { ok, .. } => assert!(ok, "1e-3 < 0.0011 must be true"),
        _ => panic!("expected comparison result"),
    }
    // 0.1 + 0.2 exact check via Rational arithmetic: 0.30000000000000004 != 0.3
    match eval_cmp("num", &["0.30000000000000004", "==", "0.3"]).expect("cmp num") {
        CmpOutcome::Result { ok, .. } => assert!(!ok, "f64 artifact must NOT equal 0.3"),
        _ => panic!("expected comparison result"),
    }
    let a = Rational::parse_decimal("0.1").unwrap();
    let b = Rational::parse_decimal("0.2").unwrap();
    let c = Rational::parse_decimal("0.3").unwrap();
    assert_eq!(a.abs_diff(&c), b, "|0.1 - 0.3| == 0.2 exactly");
}

#[test]
fn cmp_rational() {
    match eval_cmp("rational", &["6/8", "3/4"]).expect("cmp rational") {
        CmpOutcome::Result { ok, observed, expected, .. } => {
            assert!(ok, "6/8 must equal 3/4");
            assert_eq!(observed, "3/4");
            assert_eq!(expected, "3/4");
        }
        _ => panic!("expected comparison result"),
    }
    match eval_cmp("rational", &["27/32", "27/33"]).expect("cmp rational") {
        CmpOutcome::Result { ok, first_diff, .. } => {
            assert!(!ok, "27/32 must differ from 27/33");
            assert!(first_diff.is_some());
        }
        _ => panic!("expected comparison result"),
    }
}

#[test]
fn cmp_close_rel() {
    match eval_cmp("close", &["1.0000004", "1", "rel", "5e-7"]).expect("cmp close pass") {
        CmpOutcome::Result { ok, first_diff, .. } => {
            assert!(ok, "1.0000004 within rel 5e-7 of 1");
            assert!(first_diff.is_none());
        }
        _ => panic!("expected comparison result"),
    }
    match eval_cmp("close", &["1.0000004", "1", "rel", "3e-7"]).expect("cmp close fail") {
        CmpOutcome::Result { ok, first_diff, .. } => {
            assert!(!ok, "1.0000004 exceeds rel 3e-7 of 1");
            let (_, obs, exp) = first_diff.expect("first_diff required on close failure");
            assert!(obs.contains("1.0000004"));
            assert!(exp.contains("3e-7"));
        }
        _ => panic!("expected comparison result"),
    }
}

#[test]
fn sha256_vectors() {
    assert_eq!(
        sha256_hex(b""),
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    );
    assert_eq!(
        sha256_hex(b"abc"),
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
    assert_eq!(
        sha256_hex(b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq"),
        "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1"
    );
}

#[test]
fn emit_refuses_private_member() {
    let dir = scratch_dir("private_member");
    let log = dir.join("test.ndjson");
    let log_s = log.to_string_lossy().into_owned();

    // Forbidden member `voxels` at nested depth is refused.
    let err_voxels = run_emit(&svec(&[
        "--log",
        &log_s,
        "--raw-json",
        r#"{"schema":"frob.e2elog/1","payload":{"voxels":[1,2,3]}}"#,
    ]))
    .expect_err("voxels member must be refused");
    assert!(err_voxels.contains("voxels"), "err: {err_voxels}");
    assert!(!log.exists(), "refused line must not be written");

    // A 5,000-byte string is refused.
    let big = "a".repeat(5000);
    let err_big = run_emit(&svec(&[
        "--log",
        &log_s,
        "--event",
        "assert",
        "--run-id",
        "20261124T120000Z-deadbeef",
        "--script",
        "st_big",
        "--bead",
        "STD-E2E",
        "--step",
        "s1",
        "--seed",
        "0",
        "--name",
        "big",
        "--kind",
        "eq",
        "--status",
        "pass",
        "--observed",
        &big,
        "--expected",
        "ok",
    ]))
    .expect_err("5000-byte string must be refused");
    assert!(err_big.contains("4096"), "err: {err_big}");
    assert!(!log.exists(), "refused oversized string must not be written");
}

#[test]
fn redact_private_path() {
    let root = "/Users/owner/private_scans";
    let p = "/Users/owner/private_scans/kitchen/scan_01.ply";
    let redacted = redact_path(p, Some(root));
    assert!(redacted.starts_with("<private:"), "got: {redacted}");
    assert!(redacted.ends_with('>'), "got: {redacted}");
    let hex = &redacted["<private:".len()..redacted.len() - 1];
    assert_eq!(hex.len(), 12, "digest prefix must be 12 hex chars: {hex}");
    assert!(hex.bytes().all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f')));
    assert!(!redacted.contains("kitchen"));

    // Lexical traversal staying beneath root is still redacted.
    let trav = "/Users/owner/private_scans/a/../b/floor.pcd";
    assert!(redact_path(trav, Some(root)).starts_with("<private:"));

    // Non-private path is untouched.
    let pub_path = "/tmp/public/fixture.json";
    assert_eq!(redact_path(pub_path, Some(root)), pub_path);
}

#[test]
fn lint_seq_gap() {
    let dir = scratch_dir("lint_seq_gap");
    let log = dir.join("gap.ndjson");
    emit_basic_start(&log, "STD-E2E", "full");
    let log_s = log.to_string_lossy().into_owned();
    // Skip seq 1, emit seq 2
    run_emit(&svec(&[
        "--log", &log_s, "--event", "assert", "--seq", "2",
        "--run-id", "20261124T120000Z-deadbeef", "--script", "st_fixture", "--bead", "STD-E2E",
        "--step", "s1", "--seed", "1", "--ts", "2026-11-24T12:00:01.000Z",
        "--name", "a1", "--kind", "eq", "--status", "pass",
    ]))
    .unwrap();
    emit_summary(&log, "STD-E2E", "of-record");
    let findings = lint_log_file(&log, &[], None).unwrap();
    assert!(
        findings.iter().any(|f| f.rule == "seq-gap"),
        "expected seq-gap, got {findings:?}"
    );
}

#[test]
fn lint_summary_recount() {
    let dir = scratch_dir("lint_summary_recount");
    let log = dir.join("recount.ndjson");
    emit_basic_start(&log, "STD-E2E", "full");
    emit_pass_assert(&log, "STD-E2E", "a1", None);
    emit_summary(&log, "STD-E2E", "of-record");

    // Tamper summary asserts_passed from 1 to 9 while keeping body_sha256 valid
    let text = fs::read_to_string(&log).unwrap();
    let tampered = text.replacen(r#""asserts_passed":1"#, r#""asserts_passed":9"#, 1);
    fs::write(&log, tampered).unwrap();

    let findings = lint_log_file(&log, &[], None).unwrap();
    assert!(
        findings.iter().any(|f| f.rule == "summary-recount"),
        "expected summary-recount, got {findings:?}"
    );
}

#[test]
fn lint_missing_summary() {
    let dir = scratch_dir("lint_missing_summary");
    let log = dir.join("nosum.ndjson");
    emit_basic_start(&log, "STD-E2E", "full");
    emit_pass_assert(&log, "STD-E2E", "a1", None);
    let findings = lint_log_file(&log, &[], None).unwrap();
    assert!(
        findings.iter().any(|f| f.rule == "missing-summary"),
        "expected missing-summary, got {findings:?}"
    );
}

#[test]
fn lint_small_claims_gate() {
    let dir = scratch_dir("lint_small_claims_gate");
    let log = dir.join("small.ndjson");
    emit_basic_start(&log, "STD-E2E", "small");
    emit_pass_assert(&log, "STD-E2E", "a1", Some("GATE-DEMO-01"));
    let log_s = log.to_string_lossy().into_owned();
    run_emit(&svec(&[
        "--log", &log_s, "--event", "assert",
        "--run-id", "20261124T120000Z-deadbeef", "--script", "st_fixture", "--bead", "STD-E2E",
        "--step", "s1", "--seed", "1", "--ts", "2026-11-24T12:00:01.500Z",
        "--name", "nc1", "--kind", "negative-control", "--status", "pass", "--caught", "true",
        "--gate", "GATE-DEMO-01",
    ]))
    .unwrap();
    emit_summary(&log, "STD-E2E", "of-record");
    // Flip gates_claimable to true on a small-scale run
    let text = fs::read_to_string(&log).unwrap();
    let tampered = text.replacen(r#""gates_claimable":false"#, r#""gates_claimable":true"#, 1);
    fs::write(&log, tampered).unwrap();
    let findings = lint_log_file(&log, &[], None).unwrap();
    assert!(
        findings.iter().any(|f| f.rule == "small-claims-gate"),
        "expected small-claims-gate, got {findings:?}"
    );
}

#[test]
fn lint_no_control() {
    let dir = scratch_dir("lint_no_control");
    let log = dir.join("nocontrol.ndjson");
    emit_basic_start(&log, "STD-E2E", "full");
    emit_pass_assert(&log, "STD-E2E", "a1", Some("GATE-DEMO-02"));
    emit_summary(&log, "STD-E2E", "of-record");
    let findings = lint_log_file(&log, &[], None).unwrap();
    assert!(
        findings.iter().any(|f| f.rule == "no-control"),
        "expected no-control, got {findings:?}"
    );
}

#[test]
fn lint_self_recording() {
    let dir = scratch_dir("lint_self_recording");
    let write_record_gate_log = |name: &str, bead: &str| -> PathBuf {
        let log = dir.join(name);
        emit_basic_start(&log, bead, "full");
        let log_s = log.to_string_lossy().into_owned();
        let cmd_json = obj(vec![
            ("schema", JsonValue::String("frob.e2elog/1".into())),
            ("ts", JsonValue::String("2026-11-24T12:00:01.000Z".into())),
            ("seq", JsonValue::Number("1".into())),
            ("run_id", JsonValue::String("20261124T120000Z-deadbeef".into())),
            ("script", JsonValue::String("st_fixture".into())),
            ("bead", JsonValue::String(bead.into())),
            ("step", JsonValue::String("s1".into())),
            ("event", JsonValue::String("cmd".into())),
            ("seed", JsonValue::String("1".into())),
            (
                "argv",
                JsonValue::Array(
                    ["cargo", "run", "-p", "frob-conformance", "--example", "record_gate"]
                        .into_iter()
                        .map(|s| JsonValue::String(s.into()))
                        .collect(),
                ),
            ),
            (
                "cmdline",
                JsonValue::String("cargo run -p frob-conformance --example record_gate".into()),
            ),
            ("cwd", JsonValue::String("work".into())),
            ("exit", JsonValue::Number("0".into())),
            ("expected_exit", JsonValue::Array(vec![JsonValue::Number("0".into())])),
            ("wall_ms", JsonValue::Number("12".into())),
            ("stdout_sha256", JsonValue::String(sha256_hex(b""))),
        ])
        .to_json_string();
        run_emit(&svec(&["--log", &log_s, "--raw-json", &cmd_json])).unwrap();
        emit_pass_assert(&log, bead, "a1", None);
        emit_summary(&log, bead, "of-record");
        log
    };

    // Normal bead fails as self-recording.
    let bad = write_record_gate_log("bad.ndjson", "BD-022");
    let bad_findings = lint_log_file(&bad, &[], None).unwrap();
    assert!(
        bad_findings.iter().any(|f| f.rule == "self-recording"),
        "expected self-recording, got {bad_findings:?}"
    );

    // Exempt recorder bead N38b-19 passes cleanly.
    let ok = write_record_gate_log("exempt.ndjson", "N38b-19");
    let ok_findings = lint_log_file(&ok, &[], None).unwrap();
    assert!(
        ok_findings.is_empty(),
        "expected N38b-19 exemption to pass, got {ok_findings:?}"
    );
}

#[test]
fn lint_scratch_registries() {
    let dir = scratch_dir("lint_scratch_registries");
    let log = dir.join("scratch.ndjson");
    emit_basic_start(&log, "STD-E2E", "full");
    emit_pass_assert(&log, "STD-E2E", "a1", Some("GATE-DEMO-03"));
    let log_s = log.to_string_lossy().into_owned();
    run_emit(&svec(&[
        "--log", &log_s, "--event", "assert",
        "--run-id", "20261124T120000Z-deadbeef", "--script", "st_fixture", "--bead", "STD-E2E",
        "--step", "s1", "--seed", "1", "--ts", "2026-11-24T12:00:01.500Z",
        "--name", "nc1", "--kind", "negative-control", "--status", "pass", "--caught", "true",
        "--gate", "GATE-DEMO-03",
    ]))
    .unwrap();
    emit_summary(&log, "STD-E2E", "override");
    let text = fs::read_to_string(&log).unwrap();
    let tampered = text.replacen(r#""gates_claimable":false"#, r#""gates_claimable":true"#, 1);
    fs::write(&log, tampered).unwrap();
    let findings = lint_log_file(&log, &[], None).unwrap();
    assert!(
        findings.iter().any(|f| f.rule == "scratch-registries"),
        "expected scratch-registries, got {findings:?}"
    );
}

#[test]
fn lint_tier_without_receipt() {
    let dir = scratch_dir("lint_tier_without_receipt");
    let log = dir.join("tier.ndjson");
    emit_basic_start(&log, "STD-E2E", "full");
    let log_s = log.to_string_lossy().into_owned();
    let cmd_json = obj(vec![
        ("schema", JsonValue::String("frob.e2elog/1".into())),
        ("ts", JsonValue::String("2026-11-24T12:00:01.000Z".into())),
        ("seq", JsonValue::Number("1".into())),
        ("run_id", JsonValue::String("20261124T120000Z-deadbeef".into())),
        ("script", JsonValue::String("st_fixture".into())),
        ("bead", JsonValue::String("STD-E2E".into())),
        ("step", JsonValue::String("s1".into())),
        ("event", JsonValue::String("cmd".into())),
        ("seed", JsonValue::String("1".into())),
        (
            "argv",
            JsonValue::Array(
                ["cargo", "test", "-p", "frob-conformance", "--test", "g1_known_answers", "--", "ka_001"]
                    .into_iter()
                    .map(|s| JsonValue::String(s.into()))
                    .collect(),
            ),
        ),
        (
            "cmdline",
            JsonValue::String("cargo test -p frob-conformance --test g1_known_answers -- ka_001".into()),
        ),
        ("cwd", JsonValue::String("work".into())),
        ("exit", JsonValue::Number("0".into())),
        ("expected_exit", JsonValue::Array(vec![JsonValue::Number("0".into())])),
        ("wall_ms", JsonValue::Number("45".into())),
        ("stdout_sha256", JsonValue::String(sha256_hex(b""))),
    ])
    .to_json_string();
    run_emit(&svec(&["--log", &log_s, "--raw-json", &cmd_json])).unwrap();
    emit_pass_assert(&log, "STD-E2E", "a1", None);
    emit_summary(&log, "STD-E2E", "of-record");
    let findings = lint_log_file(&log, &[], None).unwrap();
    assert!(
        findings.iter().any(|f| f.rule == "tier-without-receipt"),
        "expected tier-without-receipt, got {findings:?}"
    );
}

#[test]
fn header_parse() {
    let dir = scratch_dir("header_parse");
    // Multi-line style (one key=value per `# e2e:` line)
    let multi = dir.join("multi_sample.sh");
    fs::write(
        &multi,
        "#!/usr/bin/env bash\n\
         # e2e: bead=BD-001\n\
         # e2e: milestone=M0\n\
         # e2e: labels=m0,solid,workspace\n\
         # e2e: gates=GATE-WS-01,GATE-WS-02\n\
         # e2e: seed=42\n\
         # e2e: timeout_s=1200\n\
         # e2e: needs=cpu\n\
         # e2e: plants=dirty-sibling,edited-receipt\n",
    )
    .unwrap();
    let h = parse_script_header(&multi).unwrap().expect("header present");
    assert_eq!(h.bead, "BD-001");
    assert_eq!(h.milestone, "M0");
    assert_eq!(h.labels, svec(&["m0", "solid", "workspace"]));
    assert_eq!(h.gates, svec(&["GATE-WS-01", "GATE-WS-02"]));
    assert_eq!(h.seed, 42);
    assert_eq!(h.timeout_s, 1200);
    assert_eq!(h.needs, svec(&["cpu"]));
    assert_eq!(h.plants, svec(&["dirty-sibling", "edited-receipt"]));

    // Single-line multi-pair style
    let single = dir.join("selftest_style.sh");
    fs::write(
        &single,
        "#!/usr/bin/env bash\n# e2e: bead=STD-E2E milestone=M0 labels=gauntlet,harness seed=7 plants=boundary\n",
    )
    .unwrap();
    let hs = parse_script_header(&single).unwrap().expect("header present");
    assert_eq!(hs.bead, "STD-E2E");
    assert_eq!(hs.milestone, "M0");
    assert_eq!(hs.labels, svec(&["gauntlet", "harness"]));
    assert_eq!(hs.seed, 7);
    assert_eq!(hs.timeout_s, 600);
    assert_eq!(hs.plants, svec(&["boundary"]));
}

#[test]
fn header_seed_default_zero() {
    let dir = scratch_dir("header_seed_default");
    let script = dir.join("no_seed.sh");
    fs::write(
        &script,
        "#!/usr/bin/env bash\n# e2e: bead=BD-011 milestone=M0 labels=marrow\n",
    )
    .unwrap();
    let h = parse_script_header(&script).unwrap().expect("header present");
    assert_eq!(h.seed, 0);
    assert_eq!(h.timeout_s, 600);
}

// ---------------------------------------------------------------------------
// Additional coverage: schema, Convention 15 --observe, redaction, run, report
// ---------------------------------------------------------------------------

#[test]
fn schema_is_valid_strict_json() {
    let val = json::parse_strict(schema_json()).expect("schema_json must parse strictly");
    assert_eq!(
        val.pointer("/properties/schema/const").and_then(JsonValue::as_str),
        Some("frob.e2elog/1")
    );
    assert!(val.pointer("/properties/asserted").is_some());
    assert!(val.pointer("/properties/verdict").is_some());
}

#[test]
fn lint_observe_metric_admitted() {
    let dir = scratch_dir("observe_metric");
    let log = dir.join("pilot.ndjson");
    emit_basic_start(&log, "N38a-09", "full");
    emit_pass_assert(&log, "N38a-09", "sanity", None);
    let log_s = log.to_string_lossy().into_owned();
    run_emit(&svec(&[
        "--log", &log_s, "--event", "metric",
        "--run-id", "20261124T120000Z-deadbeef", "--script", "st_fixture", "--bead", "N38a-09",
        "--step", "s1", "--seed", "1", "--ts", "2026-11-24T12:00:01.200Z",
        "--name", "speedup", "--value", "0.85", "--unit", "ratio",
        "--op", ">=", "--threshold", "1.5", "--observe",
    ]))
    .unwrap();
    let out = emit_summary(&log, "N38a-09", "of-record");
    assert!(out.contains("E2E_SUMMARY_EXIT=0"), "observe not-met must keep exit 0: {out}");
    assert!(out.contains("E2E_ASSERTS_FAILED=0"));
    let text = fs::read_to_string(&log).unwrap();
    assert!(text.contains(r#""asserted":false"#));
    assert!(text.contains(r#""verdict":"not-met""#));
    let findings = lint_log_file(&log, &[], None).unwrap();
    assert!(findings.is_empty(), "observe metric should lint clean: {findings:?}");
}

#[test]
fn lint_canaries_in_log_and_artifacts() {
    let dir = scratch_dir("canaries");
    let log = dir.join("20261124T120000Z.ndjson");
    let art_dir = dir.join("20261124T120000Z.artifacts");
    fs::create_dir_all(&art_dir).unwrap();
    emit_basic_start(&log, "STD-E2E", "full");
    emit_pass_assert(&log, "STD-E2E", "ok", None);
    emit_summary(&log, "STD-E2E", "of-record");

    // Clean initially
    assert!(lint_log_file(&log, &[], None).unwrap().is_empty());

    // Plant token canary in sibling artifact
    fs::write(art_dir.join("s1.1.stdout"), format!("leaked {TOKEN_CANARY}\n")).unwrap();
    let findings = lint_log_file(&log, &[], None).unwrap();
    assert!(
        findings.iter().any(|f| f.rule == "redaction-violation" && f.detail.contains(TOKEN_CANARY)),
        "expected token canary violation, got {findings:?}"
    );

    // Private canary directly in a second log
    let log2 = dir.join("priv.ndjson");
    emit_basic_start(&log2, "STD-E2E", "full");
    let log2_s = log2.to_string_lossy().into_owned();
    run_emit(&svec(&[
        "--log", &log2_s, "--event", "assert",
        "--run-id", "20261124T120000Z-deadbeef", "--script", "st_fixture", "--bead", "STD-E2E",
        "--step", "s1", "--seed", "1", "--ts", "2026-11-24T12:00:01.000Z",
        "--name", "leak", "--kind", "eq", "--status", "pass",
        "--observed", PRIVATE_CANARY, "--expected", PRIVATE_CANARY,
    ]))
    .unwrap();
    emit_summary(&log2, "STD-E2E", "of-record");
    let f2 = lint_log_file(&log2, &[], None).unwrap();
    assert!(
        f2.iter().any(|f| f.rule == "redaction-violation" && f.detail.contains(PRIVATE_CANARY)),
        "expected private canary violation, got {f2:?}"
    );
}

#[test]
fn run_secret_redaction_and_mode_0600() {
    let dir = scratch_dir("secret_run");
    let log = dir.join("run.ndjson");
    let transcript = dir.join("run.transcript.txt");
    let arts = dir.join("run.artifacts");
    let token_file = dir.join("state/sessions/agent-a.token");
    emit_basic_start(&log, "STD-E2E", "full");

    let secret = format!("{TOKEN_CANARY}-real-root-token");
    let env_json = format!(
        r#"{{"schema":"frob.envelope/1","verb":"session.open","ok":true,"exit":0,"result":{{"token":"{secret}"}}}}"#
    );
    let out = run_exec(&svec(&[
        "--log", &log.to_string_lossy(),
        "--transcript", &transcript.to_string_lossy(),
        "--artifacts-dir", &arts.to_string_lossy(),
        "--scratch-root", &dir.to_string_lossy(),
        "--run-id", "20261124T120000Z-deadbeef",
        "--script", "st_fixture",
        "--bead", "STD-E2E",
        "--step", "s1",
        "--seed", "1",
        "--secret", "/result/token",
        "--secret-out", &token_file.to_string_lossy(),
        "--",
        "printf", "%s", &env_json,
    ]))
    .expect("run_exec");
    assert!(out.contains("E2E_LAST_SECRET_WRITTEN=1"), "out: {out}");
    assert_eq!(fs::read_to_string(&token_file).unwrap(), secret);

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mode = fs::metadata(&token_file).unwrap().permissions().mode() & 0o777;
        assert_eq!(mode, 0o600, "token file must be mode 0600");
    }

    // Raw secret must not appear in log, transcript, or stdout capture.
    let log_text = fs::read_to_string(&log).unwrap();
    let tr_text = fs::read_to_string(&transcript).unwrap();
    let cap_text = fs::read_to_string(arts.join("s1.1.stdout")).unwrap();
    assert!(!log_text.contains(&secret));
    assert!(!tr_text.contains(&secret));
    assert!(!cap_text.contains(&secret));
    assert!(cap_text.contains("<token:"), "redacted tag missing: {cap_text}");

    emit_pass_assert(&log, "STD-E2E", "token_ok", None);
    emit_summary(&log, "STD-E2E", "of-record");
    let findings = lint_log_file(&log, &[], None).unwrap();
    assert!(findings.is_empty(), "redacted run must lint clean: {findings:?}");
}

#[test]
fn run_refusal_805_hint_and_report_classifications() {
    let dir = scratch_dir("refusal_and_report");
    let log = dir.join("logs/s_ref/20261124T120000Z.ndjson");
    let tr = dir.join("logs/s_ref/20261124T120000Z.transcript.txt");
    let arts = dir.join("logs/s_ref/20261124T120000Z.artifacts");
    emit_basic_start(&log, "STD-E2E", "full");
    let ref_env = r#"{"schema":"frob.envelope/1","verb":"sim.run","ok":false,"exit":3,"refusal":{"code":"FROB-REF-805","exit":3,"message":"missing idempotency key","repairs":["add-key"]}}"#;
    run_exec(&svec(&[
        "--log", &log.to_string_lossy(),
        "--transcript", &tr.to_string_lossy(),
        "--artifacts-dir", &arts.to_string_lossy(),
        "--scratch-root", &dir.to_string_lossy(),
        "--run-id", "20261124T120000Z-deadbeef",
        "--script", "s_ref",
        "--bead", "STD-E2E",
        "--step", "s1",
        "--seed", "1",
        "--",
        "sh", "-c", &format!("printf '%s' '{ref_env}'; exit 3"),
    ]))
    .expect("run 805");
    let tr_text = fs::read_to_string(&tr).unwrap();
    assert!(
        tr_text.contains("hint: pass --key \"$(e2e_key <label>)\""),
        "805 hint missing from transcript: {tr_text}"
    );

    // Test gate report classifications: no-control, small-only, not-run
    let scripts_dir = dir.join("scripts");
    let logs_dir = dir.join("rep_logs");
    let out_dir = dir.join("rep_out");
    fs::create_dir_all(&scripts_dir).unwrap();
    fs::write(
        scripts_dir.join("g_nocontrol.sh"),
        "#!/usr/bin/env bash\n# e2e: bead=BD-1 milestone=M0 gates=GATE-NC-01\n",
    )
    .unwrap();
    fs::write(
        scripts_dir.join("g_small.sh"),
        "#!/usr/bin/env bash\n# e2e: bead=BD-2 milestone=M0 gates=GATE-SM-01\n",
    )
    .unwrap();
    fs::write(
        scripts_dir.join("g_notrun.sh"),
        "#!/usr/bin/env bash\n# e2e: bead=BD-3 milestone=M1 gates=GATE-NR-01\n",
    )
    .unwrap();

    let nc_log = logs_dir.join("g_nocontrol/20261124T120000Z.ndjson");
    emit_basic_start(&nc_log, "BD-1", "full");
    emit_pass_assert(&nc_log, "BD-1", "ok", Some("GATE-NC-01"));
    emit_summary(&nc_log, "BD-1", "of-record");

    let sm_log = logs_dir.join("g_small/20261124T120000Z.ndjson");
    emit_basic_start(&sm_log, "BD-2", "small");
    emit_pass_assert(&sm_log, "BD-2", "ok", Some("GATE-SM-01"));
    emit_summary(&sm_log, "BD-2", "of-record");

    let rep = generate_gate_report(&scripts_dir, &logs_dir, &out_dir).expect("generate report");
    let rows = rep.get("gates").and_then(JsonValue::as_array).unwrap();
    let find_result = |id: &str| {
        rows.iter()
            .find(|r| r.get("id").and_then(JsonValue::as_str) == Some(id))
            .and_then(|r| r.get("result"))
            .and_then(JsonValue::as_str)
            .unwrap()
    };
    assert_eq!(find_result("GATE-NC-01"), "no-control");
    assert_eq!(find_result("GATE-SM-01"), "small-only");
    assert_eq!(find_result("GATE-NR-01"), "not-run");
    assert!(out_dir.join("gates.json").is_file());
    assert!(out_dir.join("gates.md").is_file());
}

#[test]
fn sha256_dir_and_pointer_escapes() {
    let dir = scratch_dir("sha_dir");
    fs::write(dir.join("b.txt"), b"beta").unwrap();
    fs::write(dir.join("a.txt"), b"alpha").unwrap();
    let d1 = sha256_path(&dir).unwrap();
    let expected = sha256_hex(
        format!("a.txt {}\nb.txt {}\n", sha256_hex(b"alpha"), sha256_hex(b"beta")).as_bytes(),
    );
    assert_eq!(d1, expected);

    // RFC 6901 pointer escapes: ~1 -> /, ~0 -> ~, and ~01 -> ~1 (NOT /)
    let doc = json::parse_strict(r#"{"a/b":1,"m~n":2,"~1":3}"#).unwrap();
    assert_eq!(doc.pointer("/a~1b").and_then(JsonValue::as_i64), Some(1));
    assert_eq!(doc.pointer("/m~0n").and_then(JsonValue::as_i64), Some(2));
    assert_eq!(doc.pointer("/~01").and_then(JsonValue::as_i64), Some(3));
}
