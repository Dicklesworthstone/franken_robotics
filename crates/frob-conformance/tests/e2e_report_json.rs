//! Regression tests for `frob-e2e` JSON parser/pointer and gate report aggregation (`STD-E2E`).
//!
//! Scratch directories live under `$BIG_SCRATCH` / `$TMPDIR` and are intentionally
//! preserved (no file or directory deletion).

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use frob_conformance::e2e::json::{self, JsonValue};
use frob_conformance::e2e::line::run_emit;
use frob_conformance::e2e::report::{cmd_list, generate_gate_report};

static COUNTER: AtomicU64 = AtomicU64::new(0);

fn scratch_dir(tag: &str) -> PathBuf {
    let base = std::env::var_os("BIG_SCRATCH")
        .map(PathBuf::from)
        .unwrap_or_else(std::env::temp_dir)
        .join("frob-BrownHeron");
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_nanos());
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = base.join(format!("e2e_report_json-{tag}-{ts}-{}-{n}", std::process::id()));
    fs::create_dir_all(&dir).expect("create scratch dir");
    dir
}

fn svec(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_string()).collect()
}

fn emit_start(log: &Path, script: &str, bead: &str, scale: &str) {
    run_emit(&svec(&[
        "--log",
        &log.to_string_lossy(),
        "--event",
        "start",
        "--run-id",
        "20261124T120000Z-abcdef01",
        "--script",
        script,
        "--bead",
        bead,
        "--step",
        "",
        "--seed",
        "1",
        "--ts",
        "2026-11-24T12:00:00.000Z",
        "--commit",
        "0123456789abcdef",
        "--dirty",
        "false",
        "--toolchain",
        "1.99.0",
        "--frob-version",
        "0.0.1",
        "--host-os",
        "macos",
        "--host-arch",
        "aarch64",
        "--host-fingerprint",
        "0011223344556677",
        "--scale",
        scale,
        "--devices",
        "cpu",
        "--plant",
        "",
    ]))
    .expect("emit start");
}

fn emit_pass(log: &Path, script: &str, bead: &str, name: &str, gate: Option<&str>) {
    let lp = log.to_string_lossy().into_owned();
    let mut args = vec![
        "--log", &lp, "--event", "assert", "--run-id", "20261124T120000Z-abcdef01",
        "--script", script, "--bead", bead, "--step", "s1", "--seed", "1",
        "--ts", "2026-11-24T12:00:01.000Z", "--name", name, "--kind", "eq",
        "--status", "pass", "--observed", "ok", "--expected", "ok",
    ];
    if let Some(g) = gate {
        args.extend(["--gate", g]);
    }
    run_emit(&svec(&args)).expect("emit pass assert");
}

fn emit_caught_control(log: &Path, script: &str, bead: &str, name: &str, gate: &str) {
    let lp = log.to_string_lossy().into_owned();
    run_emit(&svec(&[
        "--log", &lp, "--event", "assert", "--run-id", "20261124T120000Z-abcdef01",
        "--script", script, "--bead", bead, "--step", "s1", "--seed", "1",
        "--ts", "2026-11-24T12:00:02.000Z", "--name", name, "--kind", "negative-control",
        "--status", "pass", "--observed", "caught", "--expected", "caught",
        "--caught", "true", "--gate", gate,
    ]))
    .expect("emit caught control");
}

fn emit_skip(log: &Path, script: &str, bead: &str) {
    let lp = log.to_string_lossy().into_owned();
    run_emit(&svec(&[
        "--log", &lp, "--event", "assert", "--run-id", "20261124T120000Z-abcdef01",
        "--script", script, "--bead", bead, "--step", "s1", "--seed", "1",
        "--ts", "2026-11-24T12:00:01.000Z", "--name", "skip", "--kind", "check",
        "--status", "skip", "--reason", "device absent",
    ]))
    .expect("emit skip assert");
}

fn emit_summary(log: &Path, script: &str, bead: &str, registries: &str) {
    let lp = log.to_string_lossy().into_owned();
    run_emit(&svec(&[
        "--log", &lp, "--event", "summary", "--run-id", "20261124T120000Z-abcdef01",
        "--script", script, "--bead", bead, "--step", "", "--seed", "1",
        "--ts", "2026-11-24T12:00:03.000Z", "--registries", registries,
    ]))
    .expect("emit summary");
}

fn gate_result<'a>(report: &'a JsonValue, gate_id: &str) -> &'a str {
    report
        .get("gates")
        .and_then(JsonValue::as_array)
        .expect("gates array")
        .iter()
        .find(|row| row.get("id").and_then(JsonValue::as_str) == Some(gate_id))
        .and_then(|row| row.get("result"))
        .and_then(JsonValue::as_str)
        .unwrap_or_else(|| panic!("missing gate {gate_id} in report"))
}

#[test]
fn json_pointer_rejects_plus_signed_array_index() {
    let doc = json::parse_strict(r#"{"arr":["a","b"]}"#).expect("valid json");
    assert_eq!(doc.pointer("/arr/0").and_then(JsonValue::as_str), Some("a"));
    assert_eq!(doc.pointer("/arr/1").and_then(JsonValue::as_str), Some("b"));
    assert!(doc.pointer("/arr/+0").is_none(), "RFC 6901 forbids '+0'");
    assert!(doc.pointer("/arr/+1").is_none(), "RFC 6901 forbids '+1'");
    assert!(doc.pointer("/arr/01").is_none(), "RFC 6901 forbids leading zero");
    assert!(doc.pointer("/arr/-").is_none(), "RFC 6901 '-' does not reference existing element");
}

#[test]
fn json_parse_prefix_rejects_leading_zero() {
    let err_pos = json::parse_prefix_with_offset("01").expect_err("01 must fail");
    assert!(
        err_pos.message.contains("leading zero"),
        "expected leading-zero diagnostic, got: {err_pos}"
    );
    let err_neg = json::parse_prefix_with_offset("-01").expect_err("-01 must fail");
    assert!(
        err_neg.message.contains("leading zero"),
        "expected leading-zero diagnostic, got: {err_neg}"
    );
    assert!(json::parse_strict("01").is_err());
    assert!(json::parse_strict("-00").is_err());

    // Valid forms around zero remain accepted.
    assert_eq!(json::parse_prefix_with_offset("0").expect("0").0, JsonValue::Number("0".into()));
    assert_eq!(json::parse_prefix_with_offset("-0").expect("-0").0, JsonValue::Number("-0".into()));
    assert_eq!(json::parse_prefix_with_offset("0.5").expect("0.5").0, JsonValue::Number("0.5".into()));
    assert_eq!(json::parse_prefix_with_offset("0e2").expect("0e2").0, JsonValue::Number("0e2".into()));
}

#[test]
fn report_multi_script_gate_mixed_full_and_small_or_override_is_small_only() {
    let dir = scratch_dir("multi_small_override");
    let scripts = dir.join("scripts");
    let logs = dir.join("logs");
    let out = dir.join("out");
    fs::create_dir_all(&scripts).unwrap();

    // GATE-MIX-01: script A full + of-record + control; script B small + of-record + control.
    fs::write(
        scripts.join("mix_full.sh"),
        "#!/usr/bin/env bash\n# e2e: bead=BD-1 milestone=M0 gates=GATE-MIX-01\n",
    )
    .unwrap();
    fs::write(
        scripts.join("mix_small.sh"),
        "#!/usr/bin/env bash\n# e2e: bead=BD-2 milestone=M0 gates=GATE-MIX-01\n",
    )
    .unwrap();
    // GATE-OVR-01: script A full + of-record + control; script B full + override + control.
    fs::write(
        scripts.join("ovr_full.sh"),
        "#!/usr/bin/env bash\n# e2e: bead=BD-3 milestone=M0 gates=GATE-OVR-01\n",
    )
    .unwrap();
    fs::write(
        scripts.join("ovr_scratch.sh"),
        "#!/usr/bin/env bash\n# e2e: bead=BD-4 milestone=M0 gates=GATE-OVR-01\n",
    )
    .unwrap();

    let l_mf = logs.join("mix_full/20261124T120000Z.ndjson");
    emit_start(&l_mf, "mix_full", "BD-1", "full");
    emit_pass(&l_mf, "mix_full", "BD-1", "ok", Some("GATE-MIX-01"));
    emit_caught_control(&l_mf, "mix_full", "BD-1", "nc1", "GATE-MIX-01");
    emit_summary(&l_mf, "mix_full", "BD-1", "of-record");

    let l_ms = logs.join("mix_small/20261124T120000Z.ndjson");
    emit_start(&l_ms, "mix_small", "BD-2", "small");
    emit_pass(&l_ms, "mix_small", "BD-2", "ok", Some("GATE-MIX-01"));
    emit_caught_control(&l_ms, "mix_small", "BD-2", "nc2", "GATE-MIX-01");
    emit_summary(&l_ms, "mix_small", "BD-2", "of-record");

    let l_of = logs.join("ovr_full/20261124T120000Z.ndjson");
    emit_start(&l_of, "ovr_full", "BD-3", "full");
    emit_pass(&l_of, "ovr_full", "BD-3", "ok", Some("GATE-OVR-01"));
    emit_caught_control(&l_of, "ovr_full", "BD-3", "nc3", "GATE-OVR-01");
    emit_summary(&l_of, "ovr_full", "BD-3", "of-record");

    let l_os = logs.join("ovr_scratch/20261124T120000Z.ndjson");
    emit_start(&l_os, "ovr_scratch", "BD-4", "full");
    emit_pass(&l_os, "ovr_scratch", "BD-4", "ok", Some("GATE-OVR-01"));
    emit_caught_control(&l_os, "ovr_scratch", "BD-4", "nc4", "GATE-OVR-01");
    emit_summary(&l_os, "ovr_scratch", "BD-4", "override");

    let rep = generate_gate_report(&scripts, &logs, &out).expect("generate report");
    assert_eq!(gate_result(&rep, "GATE-MIX-01"), "small-only");
    assert_eq!(gate_result(&rep, "GATE-OVR-01"), "small-only");
}

#[test]
fn report_multi_script_gate_partial_skip_or_unmeasured_is_not_run() {
    let dir = scratch_dir("multi_partial");
    let scripts = dir.join("scripts");
    let logs = dir.join("logs");
    let out = dir.join("out");
    fs::create_dir_all(&scripts).unwrap();

    // GATE-PART-01: one script passed, second script never ran -> not-run.
    fs::write(
        scripts.join("part_a.sh"),
        "#!/usr/bin/env bash\n# e2e: bead=BD-1 milestone=M0 gates=GATE-PART-01\n",
    )
    .unwrap();
    fs::write(
        scripts.join("part_b.sh"),
        "#!/usr/bin/env bash\n# e2e: bead=BD-2 milestone=M0 gates=GATE-PART-01\n",
    )
    .unwrap();

    // GATE-SKIP-01: one script passed, second script exited 77 -> not-run.
    fs::write(
        scripts.join("skip_a.sh"),
        "#!/usr/bin/env bash\n# e2e: bead=BD-3 milestone=M0 gates=GATE-SKIP-01\n",
    )
    .unwrap();
    fs::write(
        scripts.join("skip_b.sh"),
        "#!/usr/bin/env bash\n# e2e: bead=BD-4 milestone=M0 gates=GATE-SKIP-01\n",
    )
    .unwrap();

    // GATE-UNMEAS-01: one script passed, second declared gate in header but never logged it -> not-run.
    fs::write(
        scripts.join("unmeas_a.sh"),
        "#!/usr/bin/env bash\n# e2e: bead=BD-5 milestone=M0 gates=GATE-UNMEAS-01\n",
    )
    .unwrap();
    fs::write(
        scripts.join("unmeas_b.sh"),
        "#!/usr/bin/env bash\n# e2e: bead=BD-6 milestone=M0 gates=GATE-UNMEAS-01\n",
    )
    .unwrap();

    // GATE-PASS-01: both scripts passed full + of-record with caught control -> pass.
    fs::write(
        scripts.join("pass_a.sh"),
        "#!/usr/bin/env bash\n# e2e: bead=BD-7 milestone=M0 gates=GATE-PASS-01\n",
    )
    .unwrap();
    fs::write(
        scripts.join("pass_b.sh"),
        "#!/usr/bin/env bash\n# e2e: bead=BD-8 milestone=M0 gates=GATE-PASS-01\n",
    )
    .unwrap();

    for (sname, bead, gid) in [
        ("part_a", "BD-1", "GATE-PART-01"),
        ("skip_a", "BD-3", "GATE-SKIP-01"),
        ("unmeas_a", "BD-5", "GATE-UNMEAS-01"),
        ("pass_a", "BD-7", "GATE-PASS-01"),
        ("pass_b", "BD-8", "GATE-PASS-01"),
    ] {
        let lp = logs.join(format!("{sname}/20261124T120000Z.ndjson"));
        emit_start(&lp, sname, bead, "full");
        emit_pass(&lp, sname, bead, "ok", Some(gid));
        emit_caught_control(&lp, sname, bead, "nc", gid);
        emit_summary(&lp, sname, bead, "of-record");
    }

    let l_skip_b = logs.join("skip_b/20261124T120000Z.ndjson");
    emit_start(&l_skip_b, "skip_b", "BD-4", "full");
    emit_skip(&l_skip_b, "skip_b", "BD-4");
    emit_summary(&l_skip_b, "skip_b", "BD-4", "of-record");

    let l_unmeas_b = logs.join("unmeas_b/20261124T120000Z.ndjson");
    emit_start(&l_unmeas_b, "unmeas_b", "BD-6", "full");
    emit_pass(&l_unmeas_b, "unmeas_b", "BD-6", "unrelated", None);
    emit_summary(&l_unmeas_b, "unmeas_b", "BD-6", "of-record");

    let rep = generate_gate_report(&scripts, &logs, &out).expect("generate report");
    assert_eq!(gate_result(&rep, "GATE-PART-01"), "not-run");
    assert_eq!(gate_result(&rep, "GATE-SKIP-01"), "not-run");
    assert_eq!(gate_result(&rep, "GATE-UNMEAS-01"), "not-run");
    assert_eq!(gate_result(&rep, "GATE-PASS-01"), "pass");
}

#[test]
fn report_corrupt_or_duplicate_summary_log_is_fail() {
    let dir = scratch_dir("corrupt_and_dup_summary");
    let scripts = dir.join("scripts");
    let logs = dir.join("logs");
    let out = dir.join("out");
    fs::create_dir_all(&scripts).unwrap();

    fs::write(
        scripts.join("corrupt.sh"),
        "#!/usr/bin/env bash\n# e2e: bead=BD-1 milestone=M0 gates=GATE-CORRUPT-01\n",
    )
    .unwrap();
    fs::write(
        scripts.join("dupsum.sh"),
        "#!/usr/bin/env bash\n# e2e: bead=BD-2 milestone=M0 gates=GATE-DUPSUM-01\n",
    )
    .unwrap();

    // Truncated JSON line (e.g., SIGKILL mid-write): must report fail, never not-run.
    let l_corrupt = logs.join("corrupt/20261124T120000Z.ndjson");
    emit_start(&l_corrupt, "corrupt", "BD-1", "full");
    let mut raw = fs::read_to_string(&l_corrupt).unwrap();
    raw.push_str("{\"schema\":\"frob.e2elog/1\",\"truncated\":");
    fs::write(&l_corrupt, raw).unwrap();

    // Duplicate summary line: second summary must not mask invalid log; must report fail.
    let l_dup = logs.join("dupsum/20261124T120000Z.ndjson");
    emit_start(&l_dup, "dupsum", "BD-2", "full");
    emit_pass(&l_dup, "dupsum", "BD-2", "ok", Some("GATE-DUPSUM-01"));
    emit_caught_control(&l_dup, "dupsum", "BD-2", "nc", "GATE-DUPSUM-01");
    emit_summary(&l_dup, "dupsum", "BD-2", "of-record");
    emit_summary(&l_dup, "dupsum", "BD-2", "of-record");

    let rep = generate_gate_report(&scripts, &logs, &out).expect("generate report");
    assert_eq!(gate_result(&rep, "GATE-CORRUPT-01"), "fail");
    assert_eq!(gate_result(&rep, "GATE-DUPSUM-01"), "fail");
}

#[test]
fn list_through_rejects_signed_milestone() {
    let dir = scratch_dir("signed_milestone");
    fs::write(
        dir.join("s1.sh"),
        "#!/usr/bin/env bash\n# e2e: bead=BD-1 milestone=M0\n",
    )
    .unwrap();
    let dstr = dir.to_string_lossy().into_owned();
    assert_eq!(cmd_list(&svec(&["--scripts-dir", &dstr, "--through", "M+1"])), 2);
    assert_eq!(cmd_list(&svec(&["--scripts-dir", &dstr, "--through", "M0"])), 0);
}
