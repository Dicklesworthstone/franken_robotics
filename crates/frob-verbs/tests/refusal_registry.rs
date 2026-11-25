//! Tether refusal-code registry tests (bead N32-REF, §32.18, §32.4).
//!
//! Operates directly against the real embedded `registries/refusals.toml` through
//! `frob_core::registry::refusals::RefusalRegistry` and `frob_verbs::registry::lint`
//! (zero mocks, zero deletions).

use std::collections::{BTreeMap, BTreeSet};

use frob_core::refusal::{RefusalCode, RetryPolicy};
use frob_core::registry::refusals::{
    EMBEDDED_REFUSALS_TOML, RefusalRegistry, RefusalRegistryError, render_refusal_code_enum,
};
use frob_verbs::registry::{LintContext, MAX_MCP_WAIT_MS, VERBS, parse_and_lint, schema_file_name};

/// The eighteen Tether chapter codes plus the two first-round DESIGN rows (`726`, `990`).
const TETHER_CODES: &[(&str, u8, RetryPolicy, &str)] = &[
    ("FROB-REF-701", 4, RetryPolicy::AfterChange, "M2"),
    ("FROB-REF-702", 4, RetryPolicy::AfterChange, "M1"),
    ("FROB-REF-703", 4, RetryPolicy::Backoff, "M3"),
    ("FROB-REF-704", 4, RetryPolicy::AfterChange, "M3"),
    ("FROB-REF-720", 5, RetryPolicy::AfterChange, "M2"),
    ("FROB-REF-723", 5, RetryPolicy::AfterChange, "M2"),
    ("FROB-REF-724", 5, RetryPolicy::Never, "M0"),
    ("FROB-REF-725", 5, RetryPolicy::AfterChange, "M3"),
    ("FROB-REF-726", 5, RetryPolicy::AfterChange, "M3"),
    ("FROB-REF-801", 3, RetryPolicy::AfterChange, "M0"),
    ("FROB-REF-802", 3, RetryPolicy::AfterChange, "M0"),
    ("FROB-REF-803", 7, RetryPolicy::AfterChange, "M1"),
    ("FROB-REF-804", 7, RetryPolicy::AfterChange, "M1"),
    ("FROB-REF-805", 3, RetryPolicy::AfterChange, "M1"),
    ("FROB-REF-806", 3, RetryPolicy::AfterChange, "M1"),
    ("FROB-REF-807", 3, RetryPolicy::AfterChange, "M1"),
    ("FROB-REF-808", 3, RetryPolicy::AfterChange, "M1"),
    ("FROB-REF-809", 7, RetryPolicy::AfterChange, "M1"),
    ("FROB-REF-810", 9, RetryPolicy::Never, "M1"),
    ("FROB-REF-990", 9, RetryPolicy::Never, "M0"),
];

const DESIGN_CHECK_IN: &str = r#"{"type":"object","required":["design"],"x-frob-positionals":["design"],"properties":{"design":{"type":"string"}}}"#;
const EMPTY_OBJ_SCHEMA: &str = r#"{"type":"object","properties":{}}"#;

fn embedded_lint_ctx(append_kind_refusals: bool) -> LintContext {
    let mut schemas = BTreeMap::new();
    schemas.insert(
        schema_file_name("frob.design.check.in/1"),
        DESIGN_CHECK_IN.as_bytes().to_vec(),
    );
    schemas.insert(
        schema_file_name("frob.design.check.out/1"),
        EMPTY_OBJ_SCHEMA.as_bytes().to_vec(),
    );
    let mut ctx = LintContext::with_embedded_refusals(schemas, None, false)
        .expect("embedded refusal registry must load");
    ctx.append_kind_refusals = append_kind_refusals;
    ctx
}

fn specimen_verb_toml(refusals_toml_list: &str) -> String {
    format!(
        r#"schema = "frob.verbs/1"

[[verb]]
name = "design.check"
cluster = "design"
kind = "read"
tier = "S"
capabilities = ["design:read"]
input_schema = "frob.design.check.in/1"
output_schema = "frob.design.check.out/1"
refusals = [{refusals_toml_list}]
idempotent = true
cost_model = "static: under 2 s for template-sized designs"
first_milestone = "M1"
task_clusters = ["author"]
examples = ["frob design check quad-a --json"]
purpose = "Run preflight checks and return ranked repairs."
use_when = "After any edit and before any simulation."
owner = "frob-design"
schema_version = 1
paginated = false
exports = false
estimable = false
"#
    )
}

#[test]
fn tether_rows_present() {
    let reg = RefusalRegistry::load().expect("embedded registries/refusals.toml must load");
    assert_eq!(TETHER_CODES.len(), 20);
    for &(code, _, _, _) in TETHER_CODES {
        let row = reg
            .lookup(code, "tether_rows_present")
            .unwrap_or_else(|e| panic!("missing Tether row {code}: {e}"));
        assert!(
            !row.meaning.trim().is_empty(),
            "{code}: meaning must be non-empty"
        );
        assert!(
            !row.defined_in.trim().is_empty(),
            "{code}: defined_in must be non-empty"
        );
        assert!(
            !row.repairs.is_empty(),
            "{code}: at least one repair is required"
        );
        assert!(
            row.repairs.iter().all(|r| !r.trim().is_empty()),
            "{code}: repairs must not contain blank items"
        );
        assert!(
            !row.raised_by.is_empty(),
            "{code}: raised_by must be non-empty"
        );
    }
}

#[test]
fn one_exit_per_code() {
    let reg = RefusalRegistry::load().expect("embedded registries/refusals.toml must load");

    // Whole-registry check: code -> exit is a function and every row matches [exit_rule].
    let mut seen = BTreeSet::new();
    for row in reg.rows() {
        assert!(seen.insert(row.number), "duplicate row for {}", row.code);
        let (expected_exit, rule_entry) = reg
            .exit_rule()
            .expected_exit(row.number)
            .unwrap_or_else(|e| panic!("{}: {e}", row.code));
        assert_eq!(
            row.exit, expected_exit,
            "{} exit disagrees with {rule_entry}",
            row.code
        );
    }

    // Exact N32-REF exit mapping for all 20 Tether rows.
    for &(code, want_exit, _, _) in TETHER_CODES {
        let parsed = RefusalCode::parse(code).expect("generated enum covers every Tether row");
        assert_eq!(
            reg.exit_for(parsed).expect("exit_for"),
            want_exit,
            "{code} must map to exit {want_exit}"
        );
    }

    // Negative control: one code entered twice (two exits) must fail the loader.
    let dup = format!(
        "{EMBEDDED_REFUSALS_TOML}\n[[refusal]]\ncode = \"FROB-REF-802\"\nmeaning = \"dup\"\ndefined_in = \"§32.18\"\nexit = 4\nretry = \"after-change\"\nrepairs = [\"r\"]\nraised_by = [\"N32-REF\"]\n"
    );
    assert!(
        RefusalRegistry::from_bytes(dup.as_bytes()).is_err(),
        "duplicate code with conflicting exit must fail to load"
    );

    // Negative control: a conflict code entered with exit 3 must fail with ExitMismatch.
    let bad_conflict = EMBEDDED_REFUSALS_TOML.replacen(
        "code = \"FROB-REF-803\"\nmeaning = \"Idempotency key reused with different resolved canonical request content for (principal, verb, key); both resolutions are shown (pointer /idempotency_key; due M1)\"\ndefined_in = \"§32.18\"\nexit = 7",
        "code = \"FROB-REF-803\"\nmeaning = \"Idempotency key reused with different resolved canonical request content for (principal, verb, key); both resolutions are shown (pointer /idempotency_key; due M1)\"\ndefined_in = \"§32.18\"\nexit = 3",
        1,
    );
    assert_ne!(bad_conflict, EMBEDDED_REFUSALS_TOML);
    assert!(matches!(
        RefusalRegistry::from_bytes(bad_conflict.as_bytes()),
        Err(RefusalRegistryError::ExitMismatch {
            observed: 3,
            expected: Some(7),
            ..
        })
    ));
}

#[test]
fn retry_classes() {
    let reg = RefusalRegistry::load().expect("embedded registries/refusals.toml must load");
    for &(code, _, want_retry, _) in TETHER_CODES {
        let row = reg.lookup(code, "retry_classes").expect("row exists");
        assert_eq!(
            row.retry, want_retry,
            "{code}: expected retry {want_retry}, got {}",
            row.retry
        );
    }

    // Negative control: 990 with retry != never must fail to load.
    let bad_990 = EMBEDDED_REFUSALS_TOML.replacen(
        "code = \"FROB-REF-990\"\nmeaning = \"Internal fault (internal-fault): handler panicked, returned an error mapping to no refusal code, or serializer refused an out-of-range int53; one envelope is still written with pointer naming the verb and observed holding the diagnostic file path; never listed in any verb's refusals (RL-06; due M0)\"\ndefined_in = \"§32.4\"\nexit = 9\nretry = \"never\"",
        "code = \"FROB-REF-990\"\nmeaning = \"Internal fault (internal-fault): handler panicked, returned an error mapping to no refusal code, or serializer refused an out-of-range int53; one envelope is still written with pointer naming the verb and observed holding the diagnostic file path; never listed in any verb's refusals (RL-06; due M0)\"\ndefined_in = \"§32.4\"\nexit = 9\nretry = \"after-change\"",
        1,
    );
    assert_ne!(bad_990, EMBEDDED_REFUSALS_TOML);
    assert!(
        RefusalRegistry::from_bytes(bad_990.as_bytes()).is_err(),
        "9xx row with retry = after-change must fail to load"
    );
}

#[test]
fn internal_fault_row() {
    let reg = RefusalRegistry::load().expect("embedded registries/refusals.toml must load");
    let row = reg
        .lookup("FROB-REF-990", "internal_fault_row")
        .expect("FROB-REF-990 row exists");
    assert_eq!(row.defined_in, "§32.4");
    assert_eq!(row.exit, 9);
    assert_eq!(row.retry, RetryPolicy::Never);
    assert_eq!(
        row.repairs,
        vec![
            "frob job find --verb <verb> --key <key>".to_owned(),
            "frob doctor".to_owned(),
        ]
    );
    assert!(
        row.meaning.contains("diagnostic file path") && row.meaning.contains("observed"),
        "990 must type observed as a diagnostic file path: {}",
        row.meaning
    );
    assert!(
        row.meaning.contains("pointer naming the verb"),
        "990 must document pointer naming the verb: {}",
        row.meaning
    );

    // No registered verb entry lists FROB-REF-990.
    for verb in VERBS {
        assert!(
            !verb.refusals.contains(&RefusalCode::FrobRef990),
            "verb `{}` must not list FROB-REF-990 (RL-06)",
            verb.name
        );
    }

    // Negative control: a planted verb entry listing FROB-REF-990 fails RL-06.
    let planted = specimen_verb_toml("\"FROB-REF-802\", \"FROB-REF-990\"");
    let err = parse_and_lint(planted.as_bytes(), &embedded_lint_ctx(true))
        .expect_err("verb listing FROB-REF-990 must fail RL-06");
    assert_eq!(err.lint, "RL-06");
    assert_eq!(err.slug, "unknown-refusal");
    assert_eq!(err.value, "FROB-REF-990");
    assert!(
        err.detail.contains("internal-fault-not-listable"),
        "expected internal-fault-not-listable detail, got: {}",
        err.detail
    );
}

#[test]
fn ref804_covers_field_form() {
    let reg = RefusalRegistry::load().expect("embedded registries/refusals.toml must load");
    let row = reg
        .lookup("FROB-REF-804", "ref804_covers_field_form")
        .expect("FROB-REF-804 exists");
    assert_eq!(row.exit, 7);
    assert_eq!(row.retry, RetryPolicy::AfterChange);
    for needle in [
        "expected_parent",
        "design.edit",
        "set-param",
        "head",
        "field",
        "expected",
        "current",
    ] {
        assert!(
            row.meaning.contains(needle),
            "FROB-REF-804 meaning missing `{needle}`: {}",
            row.meaning
        );
    }
    assert!(
        row.raised_by.iter().any(|s| s == "design.edit")
            && row.raised_by.iter().any(|s| s == "BD-058"),
        "FROB-REF-804 raised_by must include design.edit and BD-058: {:?}",
        row.raised_by
    );
    assert!(row.related.iter().any(|s| s == "FROB-REF-112"));
}

#[test]
fn living_world_codes_distinct() {
    let reg = RefusalRegistry::load().expect("embedded registries/refusals.toml must load");
    let r724 = reg
        .lookup("FROB-REF-724", "living_world_codes_distinct")
        .expect("724 exists");
    let r726 = reg
        .lookup("FROB-REF-726", "living_world_codes_distinct")
        .expect("726 exists");

    assert_eq!(r724.exit, 5);
    assert_eq!(r726.exit, 5);
    assert_eq!(r724.retry, RetryPolicy::Never);
    assert_eq!(r726.retry, RetryPolicy::AfterChange);

    let reasons_724 = ["not-linked", "feature-not-built", "killed: <gate>"];
    let reasons_726 = [
        "island-in-training",
        "tier-not-eligible",
        "purpose-not-eligible",
        "card-provenance",
    ];
    let set_724: BTreeSet<&str> = reasons_724.into_iter().collect();
    let set_726: BTreeSet<&str> = reasons_726.into_iter().collect();
    assert!(
        set_724.is_disjoint(&set_726),
        "724 and 726 reason vocabularies must be disjoint"
    );
    for reason in reasons_724 {
        assert!(
            r724.meaning.contains(reason),
            "724 missing reason `{reason}`: {}",
            r724.meaning
        );
        assert!(
            !r726.meaning.contains(reason),
            "726 must not carry 724 reason `{reason}`"
        );
    }
    for reason in reasons_726 {
        assert!(
            r726.meaning.contains(reason),
            "726 missing reason `{reason}`: {}",
            r726.meaning
        );
        assert!(
            !r724.meaning.contains(reason),
            "724 must not carry 726 reason `{reason}`"
        );
    }

    assert!(
        r726.meaning.contains("purpose check precedes build check"),
        "726 must document check order: {}",
        r726.meaning
    );
    assert!(
        r726.meaning.contains("decision ADR path"),
        "726 must carry decision ADR path for purpose-not-eligible: {}",
        r726.meaning
    );
    assert!(
        r726.repairs.len() >= 3
            && r726.repairs[1].contains("purpose-not-eligible")
            && r726.repairs[1].contains("--purpose evaluation")
            && r726.repairs[2].contains("card-provenance")
            && r726.repairs[2].contains("measured")
            && r726.repairs[2].contains("datasheet")
            && r726.repairs[0] != r726.repairs[1]
            && r726.repairs[0] != r726.repairs[2],
        "726 must provide distinct repairs for purpose-not-eligible and card-provenance: {:?}",
        r726.repairs
    );
    // Review Note 5: sim.run and sim.estimate are raising sites of 726.
    assert!(
        r726.raised_by.iter().any(|s| s == "sim.run")
            && r726.raised_by.iter().any(|s| s == "sim.estimate"),
        "726 raised_by must include sim.run and sim.estimate: {:?}",
        r726.raised_by
    );
    assert!(r724.related.iter().any(|s| s == "FROB-REF-726"));
    assert!(r726.related.iter().any(|s| s == "FROB-REF-724"));
}

#[test]
fn due_milestones() {
    let reg = RefusalRegistry::load().expect("embedded registries/refusals.toml must load");
    for &(code, _, _, want_due) in TETHER_CODES {
        let row = reg.lookup(code, "due_milestones").expect("row exists");
        let marker = format!("due {want_due}");
        assert!(
            row.meaning.contains(&marker),
            "{code}: expected `{marker}` in meaning: {}",
            row.meaning
        );
    }
    // Review Note 2: FROB-REF-702 is due M1 (budget stop in N32a-08/N29a-05), while 701 stays M2.
    assert!(
        reg.lookup("FROB-REF-702", "due_milestones")
            .unwrap()
            .meaning
            .contains("due M1")
    );
    assert!(
        reg.lookup("FROB-REF-701", "due_milestones")
            .unwrap()
            .meaning
            .contains("due M2")
    );
    assert!(
        reg.lookup("FROB-REF-726", "due_milestones")
            .unwrap()
            .meaning
            .contains("card-provenance M6")
    );
}

#[test]
fn unknown_code_is_load_error() {
    let reg = RefusalRegistry::load().expect("embedded registries/refusals.toml must load");
    let verbs_with_799 = b"schema = \"frob.verbs/1\"\n[[verb]]\nname = \"design.check\"\nrefusals = [\"FROB-REF-802\", \"FROB-REF-799\"]\n";
    let err = reg
        .validate_verb_refusals(verbs_with_799)
        .expect_err("planted FROB-REF-799 must fail RefusalRegistry::validate_verb_refusals");
    assert!(matches!(
        err,
        RefusalRegistryError::UnknownRefusalCode { ref code, .. } if code == "FROB-REF-799"
    ));

    // Also verified through frob-verbs registry lint RL-06.
    let planted_799 = specimen_verb_toml("\"FROB-REF-802\", \"FROB-REF-799\"");
    let lint_err = parse_and_lint(planted_799.as_bytes(), &embedded_lint_ctx(true))
        .expect_err("planted FROB-REF-799 must fail RL-06");
    assert_eq!(lint_err.lint, "RL-06");

    // Negative control: a verb entry listing no refusals must fail RL-06 when no kind defaults append.
    let empty_refusals = specimen_verb_toml("");
    let empty_err = parse_and_lint(empty_refusals.as_bytes(), &embedded_lint_ctx(false))
        .expect_err("verb with empty refusals must fail RL-06");
    assert_eq!(empty_err.lint, "RL-06");
}

#[test]
fn held_8xx_requests_are_not_registered_rows() {
    // Per Review Note 1, ledger hold assertions on registries/refusal_allocations.toml move to
    // N35x-02/N35x-01; here we confirm the three held proposals (836, 837, 838) are not rows
    // of registries/refusals.toml and are not compiled into RefusalCode.
    let reg = RefusalRegistry::load().expect("embedded registries/refusals.toml must load");
    for held in ["FROB-REF-836", "FROB-REF-837", "FROB-REF-838"] {
        assert!(
            RefusalCode::parse(held).is_none(),
            "held proposal {held} must not be compiled into RefusalCode"
        );
        assert!(
            matches!(
                reg.lookup(held, "held_check"),
                Err(RefusalRegistryError::UnknownRefusalCode { .. })
            ),
            "held proposal {held} must not be a row of registries/refusals.toml"
        );
    }
}

#[test]
fn messages_carry_no_import_text() {
    let allowed_placeholders: BTreeSet<&str> = [
        "unit",
        "scope",
        "id",
        "gate",
        "name",
        "content-address",
        "field",
        "expected",
        "new",
        "head",
        "string",
        "verb",
        "key",
    ]
    .into_iter()
    .collect();

    let reg = RefusalRegistry::load().expect("embedded registries/refusals.toml must load");
    for &(code, _, _, _) in TETHER_CODES {
        let row = reg
            .lookup(code, "messages_carry_no_import_text")
            .expect("row exists");
        for text in std::iter::once(&row.meaning).chain(row.repairs.iter()) {
            assert!(
                !text.chars().any(char::is_control),
                "{code}: control character in `{text}`"
            );
            let mut rest = text.as_str();
            while let Some(start) = rest.find('<') {
                let after = &rest[start + 1..];
                let Some(end) = after.find('>') else {
                    break;
                };
                let token = &after[..end];
                if !token.is_empty() && !token.contains(' ') {
                    assert!(
                        allowed_placeholders.contains(token),
                        "{code}: unexpected untyped placeholder `<{token}>` in `{text}`"
                    );
                }
                rest = &after[end + 1..];
            }
        }
    }
}

fn check_sync_wait_ms(value: i64) -> Result<(), (RefusalCode, &'static str)> {
    if (0..=MAX_MCP_WAIT_MS).contains(&value) {
        Ok(())
    } else {
        Err((RefusalCode::FrobRef802, "/sync_wait_ms"))
    }
}

#[test]
fn sync_wait_bounds_in_802() {
    let reg = RefusalRegistry::load().expect("embedded registries/refusals.toml must load");
    let row = reg
        .lookup("FROB-REF-802", "sync_wait_bounds_in_802")
        .expect("802 exists");
    assert_eq!(row.exit, 3);
    assert!(
        row.meaning.contains("sync_wait_ms")
            && row.meaning.contains("0..=30000")
            && row.meaning.contains("/sync_wait_ms"),
        "FROB-REF-802 must document sync_wait_ms bounds and /sync_wait_ms pointer: {}",
        row.meaning
    );
    assert_eq!(MAX_MCP_WAIT_MS, 30_000);
    assert_eq!(check_sync_wait_ms(0), Ok(()));
    assert_eq!(check_sync_wait_ms(MAX_MCP_WAIT_MS), Ok(()));
    assert_eq!(
        check_sync_wait_ms(-1),
        Err((RefusalCode::FrobRef802, "/sync_wait_ms"))
    );
    assert_eq!(
        check_sync_wait_ms(MAX_MCP_WAIT_MS + 1),
        Err((RefusalCode::FrobRef802, "/sync_wait_ms"))
    );
}

#[test]
fn ref801_retired_name_repair_first() {
    // Review Note 6: retired-name repair precedes edit-distance suggestions and frob robot-docs.
    let reg = RefusalRegistry::load().expect("embedded registries/refusals.toml must load");
    let row = reg
        .lookup("FROB-REF-801", "ref801_retired_name_repair_first")
        .expect("801 exists");
    assert_eq!(row.exit, 3);
    assert_eq!(row.retry, RetryPolicy::AfterChange);
    assert!(
        row.repairs.len() >= 3,
        "FROB-REF-801 needs retired-name, nearest-edit-distance, and robot-docs repairs: {:?}",
        row.repairs
    );
    assert!(
        row.repairs[0].contains("[[retired_name]]")
            && row.repairs[0].contains("frob autopsy fork -> frob run fork"),
        "FROB-REF-801 first repair must handle [[retired_name]] command skeleton: {}",
        row.repairs[0]
    );
    assert!(row.repairs[1].contains("edit distance"));
    assert_eq!(row.repairs[2], "frob robot-docs");
}

#[test]
fn ref808_input_validation_only() {
    // Review Note 3: 808 covers int53 input validation only; serializer out-of-range int53 is 990.
    let reg = RefusalRegistry::load().expect("embedded registries/refusals.toml must load");
    let r808 = reg
        .lookup("FROB-REF-808", "ref808_input_validation_only")
        .expect("808 exists");
    let r990 = reg
        .lookup("FROB-REF-990", "ref808_input_validation_only")
        .expect("990 exists");
    assert_eq!(r808.exit, 3);
    assert!(r808.meaning.contains("input validation only"));
    assert!(
        r808.raised_by
            .iter()
            .all(|s| !s.to_ascii_lowercase().contains("serializer")),
        "808 raised_by must not name the serializer (Review Note 3): {:?}",
        r808.raised_by
    );
    assert!(r808.related.iter().any(|s| s == "FROB-REF-990"));
    assert!(
        r990.meaning
            .contains("serializer refused an out-of-range int53")
    );
}

#[test]
fn layer_pairs_cross_referenced() {
    let reg = RefusalRegistry::load().expect("embedded registries/refusals.toml must load");
    for (code, partner) in [
        ("FROB-REF-704", "FROB-REF-761"),
        ("FROB-REF-804", "FROB-REF-112"),
        ("FROB-REF-806", "FROB-REF-896"),
        ("FROB-REF-810", "FROB-REF-878"),
        ("FROB-REF-724", "FROB-REF-726"),
        ("FROB-REF-726", "FROB-REF-724"),
    ] {
        let row = reg
            .lookup(code, "layer_pairs_cross_referenced")
            .expect("row exists");
        assert!(
            row.related.iter().any(|r| r == partner),
            "{code} must cross-reference {partner} in `related`: {:?}",
            row.related
        );
    }
}

#[test]
fn generated_enum_matches_embedded_registry() {
    let reg = RefusalRegistry::load().expect("embedded registries/refusals.toml must load");
    assert_eq!(RefusalCode::ALL.len(), reg.rows().len());
    for row in reg.rows() {
        let code = RefusalCode::parse(&row.code)
            .unwrap_or_else(|| panic!("generated enum missing {}", row.code));
        assert_eq!(code.as_str(), row.code);
        assert_eq!(code.number(), row.number);
    }
    let expected = render_refusal_code_enum(&reg);
    let actual = include_str!("../../frob-core/src/refusal/code_gen.rs");
    assert_eq!(actual, expected);
}
