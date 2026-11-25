#![deny(unsafe_code)]

//! Conformance and negative-control tests for the Gauntlet refusal vocabulary (`N38-REF`).
//!
//! Run as: `FROB_TEST_SEED=0 cargo test -p frob-conformance --test reasons`
//! Honors `FROB_REGISTRY_ROOT` / `FROB_CONFORMANCE_ROOT` for scratch registry negative controls
//! (used by `scripts/e2e/gauntlet_reasons.sh`). Never deletes files or directories (COMMON.md 9b).

use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use frob_conformance::controls::{CONTROLS_PATH, ControlReason, load_and_validate};
use frob_conformance::e2e::line::{run_emit, run_exec};
use frob_conformance::e2e::lint::lint_log_file;
use frob_conformance::gates_v0::{GATES_PATH, GateRegistry};
use frob_conformance::reasons::{
    EMBEDDED_REASONS_TOML, GAUNTLET_PUBLIC_CODES, HarnessReason, REASONS_PATH, ReasonKind,
    ReasonRegistry, VALID_HARNESS_EXITS, check_public_code_family_exits,
    find_conformance_owned_public_codes, find_unknown_public_codes, load_public_refusals,
    scan_e2e_refusal_logs, scan_gauntlet_tier_tests_for_public_codes,
    scan_source_tree_for_unregistered_reasons, scan_text_for_reasons, workspace_root,
};
use frob_core::registry::refusals::RefusalRegistry;
use frob_core::testkit::registry::Controls;

static SCRATCH_SEQ: AtomicU64 = AtomicU64::new(0);

fn scratch_dir(label: &str) -> PathBuf {
    let base = env::var_os("BIG_SCRATCH")
        .or_else(|| env::var_os("TMPDIR"))
        .map(PathBuf::from)
        .unwrap_or_else(env::temp_dir);
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let seq = SCRATCH_SEQ.fetch_add(1, Ordering::Relaxed);
    let dir = base
        .join("frob-WindyDog")
        .join(format!("reasons-{label}-{nanos}-{seq}"));
    fs::create_dir_all(&dir).expect("create scratch directory");
    dir
}

/// Remove the `[[reason]]` block for `target_id` from a `reasons.toml` string.
fn strip_reason_block(toml_text: &str, target_id: &str) -> String {
    let needle = format!("id = \"{target_id}\"");
    let parts: Vec<&str> = toml_text.split("[[reason]]").collect();
    assert!(
        parts.len() > 1,
        "reasons.toml must contain [[reason]] blocks"
    );
    let mut kept = vec![parts[0]];
    let mut removed = 0usize;
    for blk in &parts[1..] {
        if blk.contains(&needle) {
            removed += 1;
        } else {
            kept.push(blk);
        }
    }
    assert_eq!(
        removed, 1,
        "expected exactly one [[reason]] block for `{target_id}`, found {removed}"
    );
    kept.join("[[reason]]")
}

fn sample_gate(id: &str, milestone: &str, controls: &str, part_status: &str) -> GateRegistry {
    let measured = if part_status == "pass" { "ok" } else { "" };
    let session = if part_status == "pass" { "sess-1" } else { "" };
    let doc = format!(
        r#"schema = "frob.gates/1"

[[gate]]
id = "{id}"
claim = "known-answer"
claim_id = "INV-GAUNT-02"
section = "§38.1"
threshold = "every pass gate has a caught negative control"
milestone = "{milestone}"
tier = ["G0:host"]
controls = [{controls}]

[[gate.part]]
milestone = "{milestone}"
status = "{part_status}"
measured = "{measured}"
evidence = []
recorded = {{ date = "2026-11-24", session = "{session}", implemented_by = "" }}
"#
    );
    GateRegistry::parse_str(&doc, GATES_PATH).expect("parse scratch gates.toml")
}

fn svec(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

// ============================================================================
// 1. enumeration_matches_registry
// ============================================================================

#[test]
fn enumeration_matches_registry() {
    let root = workspace_root();
    let reg = ReasonRegistry::load_from_root(&root).unwrap_or_else(|e| panic!("{e}"));
    reg.validate_against_enum()
        .unwrap_or_else(|e| panic!("{e}"));

    // Every enum variant maps uniquely to a kebab-case string and round-trips through parse().
    let mut seen = BTreeSet::new();
    for &variant in HarnessReason::ALL {
        let id = variant.as_str();
        assert!(
            seen.insert(id),
            "duplicate HarnessReason string `{id}` in HarnessReason::ALL"
        );
        assert_eq!(
            HarnessReason::parse(id),
            Some(variant),
            "HarnessReason::parse(`{id}`) must round-trip"
        );
        assert!(
            reg.contains(id),
            "reason=removed registry missing HarnessReason `{id}`"
        );
    }
    assert_eq!(reg.records().len(), HarnessReason::ALL.len());
    assert_eq!(HarnessReason::ALL.len(), 105);
}

// ============================================================================
// 2. every_reason_has_repair
// ============================================================================

#[test]
fn every_reason_has_repair() {
    let reg = ReasonRegistry::load_embedded().expect("embedded reasons.toml must load");
    for rec in reg.records() {
        assert!(
            !rec.meaning.trim().is_empty(),
            "reason `{}` has empty meaning",
            rec.id
        );
        assert!(
            !rec.tool.is_empty(),
            "reason `{}` has empty tool list",
            rec.id
        );
        assert!(
            !rec.repairs.is_empty(),
            "reason `{}` has empty repairs list",
            rec.id
        );
        for rep in &rec.repairs {
            assert!(
                !rep.trim().is_empty(),
                "reason `{}` has blank repair entry",
                rec.id
            );
        }
        assert_ne!(
            rec.exit, 1,
            "reason `{}` must never use exit code 1",
            rec.id
        );
        assert!(
            VALID_HARNESS_EXITS.contains(&rec.exit),
            "reason `{}` exit {} outside {:?}",
            rec.id,
            rec.exit,
            VALID_HARNESS_EXITS
        );
        for &alt in &rec.alt_exits {
            assert_ne!(alt, 1, "reason `{}` alt_exit must never be 1", rec.id);
            assert!(
                VALID_HARNESS_EXITS.contains(&alt),
                "reason `{}` alt_exit {} outside {:?}",
                rec.id,
                alt,
                VALID_HARNESS_EXITS
            );
        }
    }

    // Negative control: a planted row with exit = 1 or empty repairs must fail load.
    let bad_exit_1 = "schema = \"frob.gauntlet.reasons/1\"\n\n[[reason]]\nid = \"no-control\"\ntool = [\"registry-check\"]\nmeaning = \"m\"\nrepairs = [\"r\"]\nexit = 1\n";
    let err_1 = ReasonRegistry::from_bytes(bad_exit_1.as_bytes(), "scratch.toml")
        .expect_err("exit = 1 must be refused");
    assert!(
        err_1.to_string().contains("exit 1"),
        "expected exit 1 rejection detail, got: {err_1}"
    );

    let bad_empty_repairs = "schema = \"frob.gauntlet.reasons/1\"\n\n[[reason]]\nid = \"no-control\"\ntool = [\"registry-check\"]\nmeaning = \"m\"\nrepairs = []\nexit = 101\n";
    let err_rep = ReasonRegistry::from_bytes(bad_empty_repairs.as_bytes(), "scratch.toml")
        .expect_err("empty repairs must be refused");
    assert!(
        err_rep.to_string().contains("repairs"),
        "expected repairs rejection detail, got: {err_rep}"
    );
}

// ============================================================================
// 3. no_unregistered_reason_in_source
// ============================================================================

#[test]
fn no_unregistered_reason_in_source() {
    let root = workspace_root();
    let reg = ReasonRegistry::load_from_root(&root).unwrap_or_else(|e| panic!("{e}"));
    let unregistered = scan_source_tree_for_unregistered_reasons(&root, &reg)
        .expect("scan frob-conformance and scripts/e2e for reason= literals");
    assert!(
        unregistered.is_empty(),
        "found unregistered `reason=` string(s) in source or scripts: {unregistered:?}"
    );

    // Negative control: an unregistered reason string planted in synthetic source is caught.
    let planted_src = format!(
        "eprintln!(\"{}=not-a-registered-reason: planted defect\");\n",
        "reason"
    );
    let occs = scan_text_for_reasons("synthetic_test.rs", &planted_src);
    assert_eq!(occs.len(), 1, "scanner must extract planted reason=");
    assert_eq!(occs[0].reason, "not-a-registered-reason");
    assert!(
        !reg.contains(&occs[0].reason),
        "planted bogus reason must not be registered"
    );

    // Positive control: a registered reason string is extracted and passes registration check.
    let valid_src = format!(
        "panic!(\"{}=defect-or-expect-empty missing observable\");\n",
        "reason"
    );
    let valid_occs = scan_text_for_reasons("synthetic_ok.rs", &valid_src);
    assert_eq!(valid_occs.len(), 1);
    assert!(reg.contains(&valid_occs[0].reason));
}

// ============================================================================
// 4. cited_public_codes_exist (N38-REF + Review Note 2)
// ============================================================================

#[test]
fn cited_public_codes_exist() {
    let root = workspace_root();
    let public_reg = load_public_refusals(&root).expect("load registries/refusals.toml");

    // Per Review Note 2 (polishing round 4): ten of the chapter *-REF beads that register
    // rows in GAUNTLET_PUBLIC_CODES depend on N38-REF. Therefore we check:
    //   (a) every `FROB-REF-nnn` actually cited in the Gauntlet tier test sources
    //       (`tests/g0_laws.rs`..`tests/g5_determinism.rs`), which land after their REF bead;
    //   (b) every public code in `GAUNTLET_PUBLIC_CODES` whose owning bead has already landed
    //       (starting with `FROB-REF-816` from BD-005).
    let tier_cited = scan_gauntlet_tier_tests_for_public_codes(&root)
        .expect("scan Gauntlet tier tests for FROB-REF codes");
    let tier_codes: Vec<&str> = tier_cited.iter().map(|(_, _, c)| c.as_str()).collect();
    let unknown_tier = find_unknown_public_codes(tier_codes, &public_reg);
    assert!(
        unknown_tier.is_empty(),
        "Gauntlet tier tests cite unregistered public code(s): {unknown_tier:?}"
    );

    // Verify BD-005 seeded Gauntlet code `FROB-REF-816` exists in registries/refusals.toml.
    let unknown_seeded = find_unknown_public_codes(["FROB-REF-816"], &public_reg);
    assert!(
        unknown_seeded.is_empty(),
        "expected FROB-REF-816 in registries/refusals.toml: {unknown_seeded:?}"
    );

    // Verify GAUNTLET_PUBLIC_CODES contains all 24 unique codes from the N38-REF table.
    assert_eq!(GAUNTLET_PUBLIC_CODES.len(), 24);
    let mut seen_codes = BTreeSet::new();
    for entry in GAUNTLET_PUBLIC_CODES {
        assert!(
            seen_codes.insert(entry.code),
            "duplicate code {} in GAUNTLET_PUBLIC_CODES",
            entry.code
        );
        assert_eq!(entry.code, format!("FROB-REF-{:03}", entry.number));
    }

    // Negative control (N38-REF): a cited public code that does not exist (`FROB-REF-999`) fails.
    let unknown_999 = find_unknown_public_codes(["FROB-REF-816", "FROB-REF-999"], &public_reg);
    assert_eq!(
        unknown_999,
        vec!["FROB-REF-999".to_owned()],
        "unknown cited code FROB-REF-999 must fail lookup"
    );
}

// ============================================================================
// 5. no_public_code_owned_by_conformance
// ============================================================================

#[test]
fn no_public_code_owned_by_conformance() {
    let root = workspace_root();
    let public_reg = load_public_refusals(&root).expect("load registries/refusals.toml");
    let violations = find_conformance_owned_public_codes(&public_reg);
    assert!(
        violations.is_empty(),
        "registries/refusals.toml must not name frob-conformance as owner: {violations:?}"
    );

    // Negative control: a planted row owned by `frob-conformance` is rejected.
    let planted_refusals = r#"schema = "frob.refusals/1"

[exit_rule]
"1xx" = 3
"2xx" = 3
"3xx" = 3
"4xx" = 3
"5xx" = 3
"6xx" = 6
"70x" = 4
"72x" = 5
"73x" = 5
"8xx" = 3
"9xx" = 9
conflict = []
integrity = []
capability = []

[[refusal]]
code = "FROB-REF-899"
meaning = "planted conformance-owned refusal"
defined_in = "§38.10"
exit = 3
retry = "never"
repairs = ["Move ownership out of frob-conformance"]
raised_by = ["frob-conformance"]
"#;
    let planted_reg =
        RefusalRegistry::from_bytes(planted_refusals.as_bytes()).expect("parse planted refusals");
    let planted_violations = find_conformance_owned_public_codes(&planted_reg);
    assert_eq!(
        planted_violations.len(),
        1,
        "planted frob-conformance owner must be flagged: {planted_violations:?}"
    );
    assert!(planted_violations[0].contains("FROB-REF-899"));
}

// ============================================================================
// 6. exit_codes_by_family
// ============================================================================

#[test]
fn exit_codes_by_family() {
    let root = workspace_root();
    let public_reg = load_public_refusals(&root).expect("load registries/refusals.toml");
    let errs = check_public_code_family_exits(&public_reg);
    assert!(
        errs.is_empty(),
        "GAUNTLET_PUBLIC_CODES family exit check failed: {errs:?}"
    );

    // Explicitly verify the 5xx and 8xx codes map to 3, except FROB-REF-840 which maps to 5.
    for entry in GAUNTLET_PUBLIC_CODES {
        if entry.code == "FROB-REF-840" {
            assert_eq!(entry.expected_exit, 5, "FROB-REF-840 must map to exit 5");
            assert!(entry.capability_exception);
        } else {
            assert_eq!(entry.expected_exit, 3, "{} must map to exit 3", entry.code);
            assert!(!entry.capability_exception);
        }
    }
}

// ============================================================================
// 7. emitting_beads_reasons_registered
// ============================================================================

#[test]
fn emitting_beads_reasons_registered() {
    let reg = ReasonRegistry::load_embedded().expect("load embedded reasons.toml");

    // All 23 reasons after `reduced-not-recordable`, with emitting bead & expected exit.
    let emitting_23: &[(&str, &str, u8)] = &[
        ("claim-evidence-mismatch", "N38a-05", 3),
        ("no-potency-record", "N38a-36", 3),
        ("not-potent", "N38a-36", 3),
        ("origin-label-mismatch", "N38b-07", 3),
        ("origin-not-admissible", "N38b-07", 3),
        ("ka-second-file", "N38a-12", 101),
        ("boundary-only", "N38a-12", 0),
        ("relation-without-control", "N38a-14", 101),
        ("control-without-relation", "N38a-14", 101),
        ("mutation-survived", "N38a-14", 101),
        ("mutation-stale", "N38a-14", 101),
        ("due-unbound", "N38a-03", 101),
        ("unexpected-owner", "N38a-10", 101),
        ("unallocated-number", "N38a-40", 101),
        ("missing-number", "N38a-40", 101),
        ("provisional-tag-past-due", "N38a-40", 101),
        ("cx-count-mismatch", "N38b-05", 101),
        ("same-as-invalid", "N38b-05", 101),
        ("case-supplier-invalid", "N38b-05", 101),
        ("fact-without-rerun", "N38b-05", 101),
        ("plant-missing", "N38b-05", 101),
        ("rerun-disagrees-with-pilot", "N38b-05", 101),
        ("plant-changed", "N38b-05", 0),
    ];
    assert_eq!(emitting_23.len(), 23);

    for &(id, bead, exp_exit) in emitting_23 {
        let rec = reg
            .get(id)
            .unwrap_or_else(|| panic!("missing emitting-bead reason `{id}`"));
        assert_eq!(
            rec.exit, exp_exit,
            "reason `{id}` expected exit {exp_exit}, got {}",
            rec.exit
        );
        assert!(
            rec.has_tool(bead),
            "reason `{id}` tool {:?} must name emitting bead `{bead}`",
            rec.tool
        );
    }

    // `due-unbound` names both G3 binding (N38a-15..N38a-19) and tier-matrix (N38a-03).
    let due_unbound = reg.get("due-unbound").expect("due-unbound present");
    for bead in [
        "N38a-03", "N38a-15", "N38a-16", "N38a-17", "N38a-18", "N38a-19",
    ] {
        assert!(
            due_unbound.has_tool(bead),
            "due-unbound missing bead `{bead}` in {:?}",
            due_unbound.tool
        );
    }

    // `plant-changed` carries exit 0, kind finding, and its two dispositions.
    let plant_changed = reg.get("plant-changed").expect("plant-changed present");
    assert_eq!(plant_changed.exit, 0);
    assert_eq!(plant_changed.kind, ReasonKind::Finding);
    assert_eq!(
        plant_changed.dispositions,
        vec!["restored".to_owned(), "new-plant".to_owned()]
    );

    // Review Note 6 (F-U38b): `export-format-unreadable` (N38b-03, exit 101) and
    // `not-yet-due` (N38b-04, exit 0) are registered with their stated tools and exits.
    let export_unreadable = reg
        .get("export-format-unreadable")
        .expect("Review Note 6: export-format-unreadable must be registered");
    assert_eq!(export_unreadable.exit, 101);
    assert!(export_unreadable.has_tool("close-audit") && export_unreadable.has_tool("N38b-03"));

    let not_yet_due = reg
        .get("not-yet-due")
        .expect("Review Note 6: not-yet-due must be registered");
    assert_eq!(not_yet_due.exit, 0);
    assert!(not_yet_due.has_tool("control-runner") && not_yet_due.has_tool("N38b-04"));

    // Planted negative controls: removing `due-unbound`, `origin-not-admissible`, or
    // `rerun-disagrees-with-pilot` fails validation and names the missing record.
    for missing_id in [
        "due-unbound",
        "origin-not-admissible",
        "rerun-disagrees-with-pilot",
    ] {
        let stripped = strip_reason_block(EMBEDDED_REASONS_TOML, missing_id);
        let scratch_reg =
            ReasonRegistry::from_bytes(stripped.as_bytes(), REASONS_PATH).expect("parse scratch");
        let err = scratch_reg
            .validate_against_enum()
            .expect_err("missing reason must fail validate_against_enum");
        let msg = err.to_string();
        assert!(
            msg.starts_with("reason=removed") && msg.contains(missing_id),
            "expected failure naming `{missing_id}` with reason=removed, got: {msg}"
        );
    }
}

// ============================================================================
// 8. loader_reasons_cover_bd025_plants
// ============================================================================

#[test]
fn loader_reasons_cover_bd025_plants() {
    let reg = ReasonRegistry::load_embedded().expect("load embedded reasons.toml");
    let ws = scratch_dir("bd025_plants");
    fs::create_dir_all(ws.join("crates/frob-actuate/src")).expect("mkdir dummy crate");
    fs::write(
        ws.join("crates/frob-actuate/Cargo.toml"),
        "[package]\nname = \"frob-actuate\"\nversion = \"0.0.1\"\nedition = \"2024\"\n\n[features]\ndefault = []\n",
    )
    .expect("write dummy manifest");

    let gates_not_run = sample_gate("GATE-GAUNT-01", "M0", "\"NC-0001\", \"NC-0002\"", "not-run");
    let gates_pass = sample_gate("GATE-GAUNT-01", "M0", "\"NC-0001\", \"NC-0002\"", "pass");

    let base_prev = Controls::parse_str(
        r#"schema = "frob.gauntlet.controls/1"

[[control]]
id = "NC-0001"
seed = "GATE-GAUNT-01"
gate = ["GATE-GAUNT-01"]
kind = "planted-input"
target = "frob-conformance::controls::check_registry"
defect = "planted defect"
expect = "fails with no-control"
fixture = "fixtures/a.toml"

[[control]]
id = "NC-0002"
seed = "GATE-GAUNT-01"
gate = ["GATE-GAUNT-01"]
kind = "planted-input"
target = "frob-conformance::controls::validate"
defect = "second defect"
expect = "fails with dangling-control"
fixture = "fixtures/b.toml"
"#,
        CONTROLS_PATH,
    )
    .expect("parse base_prev");

    // Every one of BD-025's twelve loader plants triggers a ControlReason whose string
    // resolves in HarnessReason and in reasons.toml with exit 101.
    let plants: Vec<(&str, &str, Option<&Controls>, &GateRegistry, ControlReason)> = vec![
        (
            "out-of-order",
            "schema = \"frob.gauntlet.controls/1\"\n\n[[control]]\nid = \"NC-0002\"\nseed = \"GATE-GAUNT-01\"\ngate = [\"GATE-GAUNT-01\"]\nkind = \"planted-input\"\ntarget = \"frob-conformance::a\"\ndefect = \"d\"\nexpect = \"fails with error\"\nfixture = \"fixtures/a.toml\"\n\n[[control]]\nid = \"NC-0001\"\nseed = \"GATE-GAUNT-01\"\ngate = [\"GATE-GAUNT-01\"]\nkind = \"planted-input\"\ntarget = \"frob-conformance::b\"\ndefect = \"d\"\nexpect = \"fails with error\"\nfixture = \"fixtures/b.toml\"\n",
            None,
            &gates_not_run,
            ControlReason::OutOfOrder,
        ),
        (
            "reused",
            "schema = \"frob.gauntlet.controls/1\"\n\n[[control]]\nid = \"NC-0001\"\nseed = \"GATE-GAUNT-01\"\ngate = [\"GATE-GAUNT-01\"]\nkind = \"planted-input\"\ntarget = \"frob-conformance::a\"\ndefect = \"d\"\nexpect = \"fails with error\"\nfixture = \"fixtures/a.toml\"\n\n[[control]]\nid = \"NC-0001\"\nseed = \"GATE-GAUNT-01\"\ngate = [\"GATE-GAUNT-01\"]\nkind = \"planted-input\"\ntarget = \"frob-conformance::b\"\ndefect = \"d\"\nexpect = \"fails with error\"\nfixture = \"fixtures/b.toml\"\n",
            None,
            &gates_not_run,
            ControlReason::Reused,
        ),
        (
            "removed",
            "schema = \"frob.gauntlet.controls/1\"\n\n[[control]]\nid = \"NC-0001\"\nseed = \"GATE-GAUNT-01\"\ngate = [\"GATE-GAUNT-01\"]\nkind = \"planted-input\"\ntarget = \"frob-conformance::controls::check_registry\"\ndefect = \"planted defect\"\nexpect = \"fails with no-control\"\nfixture = \"fixtures/a.toml\"\n",
            Some(&base_prev),
            &gates_not_run,
            ControlReason::Removed,
        ),
        (
            "renumbered",
            "schema = \"frob.gauntlet.controls/1\"\n\n[[control]]\nid = \"NC-0001\"\nseed = \"GATE-GAUNT-01\"\ngate = [\"GATE-GAUNT-01\"]\nkind = \"planted-input\"\ntarget = \"frob-conformance::controls::check_registry\"\ndefect = \"planted defect\"\nexpect = \"fails with no-control\"\nfixture = \"fixtures/a.toml\"\n\n[[control]]\nid = \"NC-0002\"\nseed = \"GATE-GAUNT-01\"\ngate = [\"GATE-GAUNT-01\"]\nkind = \"planted-input\"\ntarget = \"frob-conformance::controls::other\"\ndefect = \"second defect\"\nexpect = \"fails with dangling-control\"\nfixture = \"fixtures/b.toml\"\n",
            Some(&base_prev),
            &gates_not_run,
            ControlReason::Renumbered,
        ),
        (
            "no-gate",
            "schema = \"frob.gauntlet.controls/1\"\n\n[[control]]\nid = \"NC-0001\"\nseed = \"GATE-GAUNT-01\"\ngate = []\nkind = \"planted-input\"\ntarget = \"frob-conformance::a\"\ndefect = \"d\"\nexpect = \"fails with error\"\nfixture = \"fixtures/a.toml\"\n",
            None,
            &gates_not_run,
            ControlReason::NoGate,
        ),
        (
            "unknown-gate",
            "schema = \"frob.gauntlet.controls/1\"\n\n[[control]]\nid = \"NC-0001\"\nseed = \"GATE-GAUNT-01\"\ngate = [\"GATE-UNKNOWN-99\"]\nkind = \"planted-input\"\ntarget = \"frob-conformance::a\"\ndefect = \"d\"\nexpect = \"fails with error\"\nfixture = \"fixtures/a.toml\"\n",
            None,
            &gates_not_run,
            ControlReason::UnknownGate,
        ),
        (
            "kind-not-closed",
            "schema = \"frob.gauntlet.controls/1\"\n\n[[control]]\nid = \"NC-0001\"\nseed = \"GATE-GAUNT-01\"\ngate = [\"GATE-GAUNT-01\"]\nkind = \"bogus-kind\"\ntarget = \"frob-conformance::a\"\ndefect = \"d\"\nexpect = \"fails with error\"\nfixture = \"fixtures/a.toml\"\n",
            None,
            &gates_not_run,
            ControlReason::KindNotClosed,
        ),
        (
            "feature-missing",
            "schema = \"frob.gauntlet.controls/1\"\n\n[[control]]\nid = \"NC-0001\"\nseed = \"CX-262\"\ngate = [\"GATE-GAUNT-01\"]\nkind = \"mutation\"\ntarget = \"frob-actuate::stage\"\ndefect = \"d\"\nexpect = \"fails with error\"\nfeature = \"undeclared-feat\"\n",
            None,
            &gates_pass,
            ControlReason::FeatureMissing,
        ),
        (
            "fixture-missing",
            "schema = \"frob.gauntlet.controls/1\"\n\n[[control]]\nid = \"NC-0001\"\nseed = \"GATE-GAUNT-01\"\ngate = [\"GATE-GAUNT-01\"]\nkind = \"planted-world\"\ntarget = \"frob-conformance::a\"\ndefect = \"d\"\nexpect = \"fails with error\"\nfixture = \"fixtures/absent.toml\"\n",
            None,
            &gates_pass,
            ControlReason::FixtureMissing,
        ),
        (
            "defect-or-expect-empty",
            "schema = \"frob.gauntlet.controls/1\"\n\n[[control]]\nid = \"NC-0001\"\nseed = \"GATE-GAUNT-01\"\ngate = [\"GATE-GAUNT-01\"]\nkind = \"planted-input\"\ntarget = \"frob-conformance::a\"\ndefect = \"\"\nexpect = \"fails with error\"\nfixture = \"fixtures/a.toml\"\n",
            None,
            &gates_not_run,
            ControlReason::DefectOrExpectEmpty,
        ),
        (
            "boundary-not-control",
            "schema = \"frob.gauntlet.controls/1\"\n\n[[control]]\nid = \"NC-0001\"\nseed = \"GATE-GAUNT-01\"\ngate = [\"GATE-GAUNT-01\"]\nkind = \"planted-input\"\ntarget = \"frob-contact::film\"\ndefect = \"a 4 mm film of water is refused\"\nexpect = \"request is refused at the limit\"\nfixture = \"fixtures/a.toml\"\n",
            None,
            &gates_not_run,
            ControlReason::BoundaryNotControl,
        ),
        (
            "unknown-seed-form",
            "schema = \"frob.gauntlet.controls/1\"\n\n[[control]]\nid = \"NC-0001\"\nseed = \"BUG-99\"\ngate = [\"GATE-GAUNT-01\"]\nkind = \"planted-input\"\ntarget = \"frob-conformance::a\"\ndefect = \"d\"\nexpect = \"fails with error\"\nfixture = \"fixtures/a.toml\"\n",
            None,
            &gates_not_run,
            ControlReason::UnknownSeedForm,
        ),
    ];
    assert_eq!(plants.len(), 12);

    for (expected_id, toml_src, prev, gates, expected_variant) in plants {
        let findings = load_and_validate(toml_src, CONTROLS_PATH, prev, gates, &ws).expect_err(
            &format!("BD-025 plant `{expected_id}` must fail validation"),
        );
        assert!(
            findings.iter().any(|f| f.reason == expected_variant),
            "plant `{expected_id}` expected {expected_variant:?}, got {findings:?}"
        );
        let parsed = HarnessReason::parse(expected_variant.as_str()).unwrap_or_else(|| {
            panic!(
                "BD-025 reason `{}` missing from HarnessReason enum",
                expected_variant.as_str()
            )
        });
        assert_eq!(parsed.as_str(), expected_id);
        let rec = reg
            .get(expected_id)
            .unwrap_or_else(|| panic!("BD-025 reason `{expected_id}` missing from reasons.toml"));
        assert_eq!(rec.exit, 101);
        assert!(
            rec.has_tool("control-loader") && rec.has_tool("BD-025"),
            "BD-025 reason `{expected_id}` tool list {:?} must name control-loader and BD-025",
            rec.tool
        );
        let line = parsed.format_failure_line("planted control rejected");
        assert!(line.starts_with(&format!("reason={expected_id} ")));
    }
}

// ============================================================================
// 9. recorder_reasons_exit_3
// ============================================================================

#[test]
fn recorder_reasons_exit_3() {
    let reg = ReasonRegistry::load_embedded().expect("load embedded reasons.toml");

    // Five emitting-bead recorder reasons + N38b-19's five named reasons + ten core recorder reasons.
    let recorder_exit_3 = [
        // 5 emitting-bead recorder reasons
        "claim-evidence-mismatch",
        "no-potency-record",
        "not-potent",
        "origin-label-mismatch",
        "origin-not-admissible",
        // 5 N38b-19 / N32ax-03 named reasons
        "receipt-stack-dev",
        "dev-build-identity",
        "receipt-set-invalid",
        "part-rows-dropped",
        "receipt-not-of-record",
        // 10 core recorder reasons
        "receipt-hash-mismatch",
        "receipt-dirty-tree",
        "receipt-commit-unknown",
        "gate-line-not-eligible",
        "pass-without-control",
        "control-missing-from-receipt",
        "control-not-caught",
        "self-certified",
        "evidence-unresolved",
        "reduced-not-recordable",
    ];
    for id in recorder_exit_3 {
        let rec = reg
            .get(id)
            .unwrap_or_else(|| panic!("recorder reason `{id}` missing"));
        assert_eq!(rec.exit, 3, "recorder reason `{id}` must carry exit 3");
        assert!(
            rec.has_tool("recorder"),
            "recorder reason `{id}` tool {:?} must name `recorder`",
            rec.tool
        );
    }

    // Allocation validator (3) and N38b-05 counterexample validation (6) exit 101.
    let alloc_and_cx_exit_101 = [
        ("unallocated-number", "N38a-40"),
        ("missing-number", "N38a-40"),
        ("provisional-tag-past-due", "N38a-40"),
        ("cx-count-mismatch", "N38b-05"),
        ("same-as-invalid", "N38b-05"),
        ("case-supplier-invalid", "N38b-05"),
        ("fact-without-rerun", "N38b-05"),
        ("plant-missing", "N38b-05"),
        ("rerun-disagrees-with-pilot", "N38b-05"),
    ];
    for (id, bead) in alloc_and_cx_exit_101 {
        let rec = reg
            .get(id)
            .unwrap_or_else(|| panic!("reason `{id}` missing"));
        assert_eq!(rec.exit, 101, "reason `{id}` must carry exit 101");
        assert!(
            rec.has_tool(bead),
            "reason `{id}` tool {:?} must name `{bead}`",
            rec.tool
        );
    }
}

// ============================================================================
// 10. gate_validator_and_pilot_reasons_registered
// ============================================================================

#[test]
fn gate_validator_and_pilot_reasons_registered() {
    let reg = ReasonRegistry::load_embedded().expect("load embedded reasons.toml");

    // 7 new gate-register validator IDs (N40-01)
    let validator_7 = [
        "target-as-measurement",
        "claim-not-one-of-seven",
        "no-claim-on-non-reality",
        "blocked-without-question",
        "killed-without-decision",
        "duplicate-id",
        "reused-id",
    ];
    for id in validator_7 {
        let rec = reg
            .get(id)
            .unwrap_or_else(|| panic!("validator reason `{id}` missing"));
        assert!(
            rec.has_tool("gate-register-validator") && rec.has_tool("N40-01"),
            "validator reason `{id}` tool {:?} must name gate-register-validator and N40-01",
            rec.tool
        );
        assert_eq!(
            rec.exit, 3,
            "validator reason `{id}` primary exit must be 3"
        );
        assert!(
            rec.allows_exit(101),
            "validator reason `{id}` must allow Cargo test exit 101"
        );
    }

    // 4 validator reasons shared with the recorder are one row each, naming both tools.
    for id in [
        "pass-without-control",
        "control-not-caught",
        "self-certified",
        "evidence-unresolved",
    ] {
        let rec = reg.get(id).expect("shared recorder/validator reason");
        assert!(
            rec.has_tool("recorder") && rec.has_tool("gate-register-validator"),
            "shared reason `{id}` must name both recorder and gate-register-validator: {:?}",
            rec.tool
        );
    }

    // `no-claim-on-non-reality` names both gate-register-validator and N38a-02's tier-runner.
    let no_claim = reg
        .get("no-claim-on-non-reality")
        .expect("no-claim-on-non-reality present");
    assert!(
        no_claim.has_tool("gate-register-validator")
            && no_claim.has_tool("tier-runner")
            && no_claim.has_tool("N38a-02"),
        "no-claim-on-non-reality must name validator and tier-runner (N38a-02): {:?}",
        no_claim.tool
    );

    // 11 pilot funding check IDs (N38a-09)
    let funding_11 = [
        ("missing", 0u8),
        ("unregistered", 0),
        ("void", 0),
        ("registered", 0),
        ("running", 0),
        ("falsifier-holds", 0),
        ("inconclusive", 0),
        ("kill-gate-to-register", 0),
        ("moonshot-cap", 3),
        ("plan-time-differs", 0),
        ("plan-time-not-run", 0),
    ];
    for (id, exp_exit) in funding_11 {
        let rec = reg
            .get(id)
            .unwrap_or_else(|| panic!("pilot funding reason `{id}` missing"));
        assert!(
            rec.has_tool("pilot-funding-check") && rec.has_tool("N38a-09"),
            "funding reason `{id}` must name pilot-funding-check and N38a-09: {:?}",
            rec.tool
        );
        assert_eq!(rec.exit, exp_exit, "funding reason `{id}` exit mismatch");
    }

    // Review Note 7 (F-U41): `moonshot-cap` also names N41-09 (`--example moonshot -- enter`).
    let mcap = reg.get("moonshot-cap").expect("moonshot-cap present");
    assert!(
        mcap.has_tool("pilot-status-change")
            && mcap.has_tool("pilot-funding-check")
            && mcap.has_tool("moonshot-transition")
            && mcap.has_tool("N41-09"),
        "moonshot-cap must include N41-09 transition command per Review Note 7: {:?}",
        mcap.tool
    );
    assert!(mcap.allows_exit(0) && mcap.allows_exit(3));

    // 10 new pilot load IDs (exit 101) + 2 shared registry-diff IDs (`reused`, `removed`).
    let pilot_load_10 = [
        "pilot-id-form",
        "pilot-part-missing",
        "pilot-cannot-fail",
        "result-protocol-mismatch",
        "result-before-registration",
        "pilot-receipt-uncommitted",
        "fallback-tier-not-lower",
        "fallback-also-unknown",
        "tier-s-without-required-by",
        "plan-time-record-unknown",
    ];
    for id in pilot_load_10 {
        let rec = reg
            .get(id)
            .unwrap_or_else(|| panic!("pilot load reason `{id}` missing"));
        assert_eq!(
            rec.exit, 101,
            "pilot load reason `{id}` must carry exit 101"
        );
        assert!(
            rec.has_tool("pilot-loader") && rec.has_tool("N38a-09"),
            "pilot load reason `{id}` must name pilot-loader and N38a-09: {:?}",
            rec.tool
        );
    }
    for id in ["reused", "removed"] {
        let rec = reg.get(id).expect("shared pilot-loader diff reason");
        assert_eq!(rec.exit, 101);
        assert!(
            rec.has_tool("pilot-loader") && rec.has_tool("N38a-09"),
            "shared diff reason `{id}` must also name pilot-loader and N38a-09: {:?}",
            rec.tool
        );
    }

    // Negative controls: scratch reasons.toml without `moonshot-cap` or `pilot-receipt-uncommitted`.
    for missing_id in ["moonshot-cap", "pilot-receipt-uncommitted"] {
        let stripped = strip_reason_block(EMBEDDED_REASONS_TOML, missing_id);
        let scratch_reg =
            ReasonRegistry::from_bytes(stripped.as_bytes(), REASONS_PATH).expect("parse scratch");
        let err = scratch_reg
            .validate_against_enum()
            .expect_err("missing reason must fail validate_against_enum");
        let msg = err.to_string();
        assert!(
            msg.starts_with("reason=removed") && msg.contains(missing_id),
            "expected failure naming `{missing_id}`, got: {msg}"
        );
    }
}

// ============================================================================
// 11. STD-E2E `refusal` event linter & log scanner (N38-REF E2E contract)
// ============================================================================

#[test]
fn e2e_refusal_event_reason_and_code_lint_and_scan() {
    let dir = scratch_dir("e2e_refusal_lint");
    let root = workspace_root();
    let reason_reg = ReasonRegistry::load_embedded().expect("load reasons");
    let public_reg = load_public_refusals(&root).expect("load public refusals");

    // 1) Valid log with one `reason`-only refusal event and one `code`-only refusal event.
    let good_log = dir.join("logs/gauntlet_ok/20261124T120000Z.ndjson");
    let good_tr = dir.join("logs/gauntlet_ok/20261124T120000Z.transcript.txt");
    let good_arts = dir.join("logs/gauntlet_ok/20261124T120000Z.artifacts");
    run_emit(&svec(&[
        "--log",
        &good_log.to_string_lossy(),
        "--transcript",
        &good_tr.to_string_lossy(),
        "--event",
        "start",
        "--run-id",
        "20261124T120000Z-aaaa1111",
        "--script",
        "gauntlet_ok",
        "--bead",
        "N38-REF",
        "--seed",
        "0",
        "--scale",
        "full",
    ]))
    .expect("emit start");
    run_emit(&svec(&[
        "--log",
        &good_log.to_string_lossy(),
        "--transcript",
        &good_tr.to_string_lossy(),
        "--event",
        "refusal",
        "--run-id",
        "20261124T120000Z-aaaa1111",
        "--script",
        "gauntlet_ok",
        "--bead",
        "N38-REF",
        "--step",
        "s1",
        "--seed",
        "0",
        "--reason",
        "defect-or-expect-empty",
        "--exit",
        "101",
        "--expected",
        "true",
        "--message",
        "missing observable",
    ]))
    .expect("emit reason-only refusal");
    run_emit(&svec(&[
        "--log",
        &good_log.to_string_lossy(),
        "--transcript",
        &good_tr.to_string_lossy(),
        "--event",
        "refusal",
        "--run-id",
        "20261124T120000Z-aaaa1111",
        "--script",
        "gauntlet_ok",
        "--bead",
        "N38-REF",
        "--step",
        "s1",
        "--seed",
        "0",
        "--code",
        "FROB-REF-816",
        "--exit",
        "3",
        "--expected",
        "true",
        "--message",
        "authorization flag check",
    ]))
    .expect("emit code-only refusal");
    run_exec(&svec(&[
        "--log",
        &good_log.to_string_lossy(),
        "--transcript",
        &good_tr.to_string_lossy(),
        "--artifacts-dir",
        &good_arts.to_string_lossy(),
        "--scratch-root",
        &dir.to_string_lossy(),
        "--run-id",
        "20261124T120000Z-aaaa1111",
        "--script",
        "gauntlet_ok",
        "--bead",
        "N38-REF",
        "--step",
        "s1",
        "--seed",
        "0",
        "--",
        "true",
    ]))
    .expect("emit cmd");
    run_emit(&svec(&[
        "--log",
        &good_log.to_string_lossy(),
        "--transcript",
        &good_tr.to_string_lossy(),
        "--event",
        "assert",
        "--run-id",
        "20261124T120000Z-aaaa1111",
        "--script",
        "gauntlet_ok",
        "--bead",
        "N38-REF",
        "--step",
        "s1",
        "--seed",
        "0",
        "--name",
        "ok",
        "--kind",
        "eq",
        "--status",
        "pass",
        "--observed",
        "0",
        "--expected",
        "0",
    ]))
    .expect("emit assert");
    run_emit(&svec(&[
        "--log",
        &good_log.to_string_lossy(),
        "--transcript",
        &good_tr.to_string_lossy(),
        "--event",
        "summary",
        "--run-id",
        "20261124T120000Z-aaaa1111",
        "--script",
        "gauntlet_ok",
        "--bead",
        "N38-REF",
        "--seed",
        "0",
        "--registries",
        "of-record",
    ]))
    .expect("emit summary");

    let findings = lint_log_file(&good_log, &[], None).expect("lint good log");
    assert!(
        findings.is_empty(),
        "valid reason-only and code-only refusal events must lint clean: {findings:?}"
    );
    let tr = fs::read_to_string(&good_tr).expect("read transcript");
    assert!(
        tr.contains("REFUSAL defect-or-expect-empty"),
        "transcript must display harness reason: {tr}"
    );

    let scan_ok =
        scan_e2e_refusal_logs(&dir.join("logs"), &reason_reg, &public_reg).expect("scan good logs");
    assert_eq!(scan_ok.events_scanned, 2);
    assert!(
        scan_ok.unknowns.is_empty(),
        "registered reason and code must scan clean: {:?}",
        scan_ok.unknowns
    );

    // 2) Negative control: `refusal` event with NEITHER `code` nor `reason` is rejected by
    //    both `frob-e2e emit` and `frob-e2e lint`.
    let bad_emit = run_emit(&svec(&[
        "--log",
        &dir.join("bad_emit.ndjson").to_string_lossy(),
        "--event",
        "refusal",
        "--run-id",
        "20261124T120000Z-bbbb2222",
        "--script",
        "bad",
        "--bead",
        "N38-REF",
        "--step",
        "s1",
        "--seed",
        "0",
        "--exit",
        "3",
        "--expected",
        "true",
    ]));
    assert!(
        bad_emit.is_err(),
        "emit refusal with neither --code nor --reason must fail"
    );

    let neither_log = dir.join("logs_bad/bad_neither/20261124T120000Z.ndjson");
    run_emit(&svec(&[
        "--log",
        &neither_log.to_string_lossy(),
        "--event",
        "start",
        "--run-id",
        "20261124T120000Z-cccc3333",
        "--script",
        "bad_neither",
        "--bead",
        "N38-REF",
        "--seed",
        "0",
        "--scale",
        "full",
    ]))
    .expect("emit bad start");
    run_emit(&svec(&[
        "--log", &neither_log.to_string_lossy(),
        "--raw-json",
        r#"{"schema":"frob.e2elog/1","ts":"2026-11-24T12:00:01.000Z","seq":1,"run_id":"20261124T120000Z-cccc3333","script":"bad_neither","bead":"N38-REF","step":"s1","event":"refusal","seed":"0","exit":3,"expected":true}"#,
    ]))
    .expect("emit raw refusal without code or reason");
    run_emit(&svec(&[
        "--log",
        &neither_log.to_string_lossy(),
        "--event",
        "summary",
        "--run-id",
        "20261124T120000Z-cccc3333",
        "--script",
        "bad_neither",
        "--bead",
        "N38-REF",
        "--seed",
        "0",
        "--registries",
        "of-record",
    ]))
    .expect("emit bad summary");

    let bad_findings = lint_log_file(&neither_log, &[], None).expect("lint bad neither log");
    assert!(
        bad_findings
            .iter()
            .any(|f| f.rule == "schema" && f.detail.contains("code or reason")),
        "linter must reject refusal event with neither code nor reason: {bad_findings:?}"
    );

    // 3) Negative control: log scanner catches unknown harness reason and unknown FROB-REF-999.
    let unk_log = dir.join("logs_unk/unk_script/20261124T120000Z.ndjson");
    run_emit(&svec(&[
        "--log",
        &unk_log.to_string_lossy(),
        "--event",
        "start",
        "--run-id",
        "20261124T120000Z-dddd4444",
        "--script",
        "unk_script",
        "--bead",
        "N38-REF",
        "--seed",
        "0",
        "--scale",
        "full",
    ]))
    .expect("emit unk start");
    run_emit(&svec(&[
        "--log",
        &unk_log.to_string_lossy(),
        "--event",
        "refusal",
        "--run-id",
        "20261124T120000Z-dddd4444",
        "--script",
        "unk_script",
        "--bead",
        "N38-REF",
        "--step",
        "s1",
        "--seed",
        "0",
        "--reason",
        "unregistered-bogus-reason",
        "--exit",
        "101",
        "--expected",
        "true",
    ]))
    .expect("emit unk reason");
    run_emit(&svec(&[
        "--log",
        &unk_log.to_string_lossy(),
        "--event",
        "refusal",
        "--run-id",
        "20261124T120000Z-dddd4444",
        "--script",
        "unk_script",
        "--bead",
        "N38-REF",
        "--step",
        "s1",
        "--seed",
        "0",
        "--code",
        "FROB-REF-999",
        "--exit",
        "3",
        "--expected",
        "true",
    ]))
    .expect("emit unk code");
    run_emit(&svec(&[
        "--log",
        &unk_log.to_string_lossy(),
        "--event",
        "summary",
        "--run-id",
        "20261124T120000Z-dddd4444",
        "--script",
        "unk_script",
        "--bead",
        "N38-REF",
        "--seed",
        "0",
        "--registries",
        "of-record",
    ]))
    .expect("emit unk summary");

    let scan_unk = scan_e2e_refusal_logs(&dir.join("logs_unk"), &reason_reg, &public_reg)
        .expect("scan unknown logs");
    assert_eq!(scan_unk.events_scanned, 2);
    assert_eq!(
        scan_unk.unknowns.len(),
        2,
        "expected 2 unknowns: {:?}",
        scan_unk.unknowns
    );
    assert!(
        scan_unk
            .unknowns
            .iter()
            .any(|u| u.contains("unregistered-bogus-reason"))
    );
    assert!(scan_unk.unknowns.iter().any(|u| u.contains("FROB-REF-999")));
}
