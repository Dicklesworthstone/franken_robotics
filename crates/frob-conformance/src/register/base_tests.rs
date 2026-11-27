//! Unit tests for `frob-conformance::register` base records (`N38b-18`).
//!
//! Run with: `cargo test -p frob-conformance register::base`
//! All inline fixtures below are labelled `synthetic` per `STD-TEST`.

use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

use crate::register::{
    BaseDocument, BaseKind, Claim, GATE_COUNTS_PATH, GateCountsExpectation, KILL_FORBIDDEN_FIELDS,
    KindCounts, Status,
};

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("locate workspace root from CARGO_MANIFEST_DIR")
        .to_path_buf()
}

// synthetic fixture: exact GATE-ARENA-05 example from N38b-18.
const SYNTHETIC_ARENA_05: &str = r#"schema = "frob.gates/1"

[[gate]]
id = "GATE-ARENA-05"
claim = "known-answer"
claim_id = "clm:0000000000000000000000000000000000000000000000000000000000000005"
section = "24.17"
threshold = "eval.preanalyze reproduces the fixed-n tables exactly and refuses the unreachable verdict"
milestone = "M1"
tier = ["G1:host"]
controls = ["NC-0398", "NC-0399"]

[[gate.part]]
milestone = "M1"
status = "not-run"
measured = ""
evidence = []
recorded = { date = "", session = "", implemented_by = "" }
"#;

#[test]
fn base_record_roundtrip() {
    let doc = BaseDocument::parse_str(SYNTHETIC_ARENA_05, "synthetic:arena05.toml")
        .expect("GATE-ARENA-05 example must parse");
    assert_eq!(doc.gates.len(), 1);
    let g = &doc.gates[0];
    assert_eq!(g.id, "GATE-ARENA-05");
    assert_eq!(g.section, "24.17");
    let m = g.measured.as_ref().expect("measured fields");
    assert_eq!(m.claim, Claim::KnownAnswer);
    assert_eq!(m.milestone, "M1");
    assert_eq!(m.tier, vec!["G1:host"]);
    assert_eq!(m.controls, vec!["NC-0398", "NC-0399"]);
    assert_eq!(m.parts.len(), 1);
    assert_eq!(m.parts[0].status, Status::NotRun);
    assert_eq!(g.derived_status(), Status::NotRun);
    assert_eq!(g.printed_status(None), "not run");

    let printed = doc.print_toml();
    assert_eq!(
        printed, SYNTHETIC_ARENA_05,
        "printed GATE-ARENA-05 must equal input byte-for-byte (D0)"
    );
    let reparsed = BaseDocument::parse_str(&printed, "synthetic:roundtrip.toml")
        .expect("printed output must re-parse");
    assert_eq!(doc, reparsed);
}

#[test]
fn claim_is_one_of_seven() {
    // Each of the seven closed claim kinds loads and round-trips cleanly.
    for &claim in Claim::ALL {
        let toml = format!(
            "schema = \"frob.gates/1\"\n\n[[gate]]\nid = \"GATE-CLAIM-01\"\nclaim = \"{}\"\nclaim_id = \"clm:0000000000000000000000000000000000000000000000000000000000000001\"\nsection = \"38.13\"\nthreshold = \"t\"\nmilestone = \"M0\"\ntier = [\"G0:host\"]\ncontrols = [\"NC-0001\"]\n",
            claim.as_str()
        );
        let doc = BaseDocument::parse_str(&toml, "synthetic:claim.toml")
            .unwrap_or_else(|e| panic!("valid claim `{}` failed: {e}", claim.as_str()));
        assert_eq!(doc.gates[0].measured.as_ref().unwrap().claim, claim);
    }

    // "method", "performance of the method", "Performance of the method", and two-claim arrays
    // are each rejected naming the gate.
    for bad in [
        "\"method\"",
        "\"performance of the method\"",
        "\"Performance of the method\"",
        "[\"parity\", \"performance\"]",
        "\"\"",
    ] {
        let toml = format!(
            "schema = \"frob.gates/1\"\n\n[[gate]]\nid = \"GATE-BADCLAIM-07\"\nclaim = {bad}\nclaim_id = \"clm:0000000000000000000000000000000000000000000000000000000000000007\"\nsection = \"38.13\"\nthreshold = \"t\"\nmilestone = \"M0\"\ntier = [\"G0:host\"]\ncontrols = [\"NC-0001\"]\n"
        );
        let err = BaseDocument::parse_str(&toml, "synthetic:bad_claim.toml")
            .expect_err(&format!("bad claim {bad} must fail"));
        assert_eq!(err.gate_id.as_deref(), Some("GATE-BADCLAIM-07"));
        if bad.to_ascii_lowercase().contains("method") {
            assert!(
                err.message.contains("design-comparison"),
                "method rejection must direct explicit edit to design-comparison: {}",
                err.message
            );
        }
    }
}

#[test]
fn status_closed_set() {
    // All seven statuses (six base + blocked) load cleanly and format reconciled printed forms.
    for &status in Status::ALL {
        let toml = format!(
            "schema = \"frob.gates/1\"\n\n[[gate]]\nid = \"GATE-STAT-01\"\nclaim = \"reality\"\nclaim_id = \"clm:0000000000000000000000000000000000000000000000000000000000000001\"\nsection = \"38.13\"\nthreshold = \"t\"\nmilestone = \"M0\"\ntier = [\"G0:host\"]\ncontrols = [\"NC-0001\"]\n\n[[gate.part]]\nmilestone = \"M0\"\nstatus = \"{}\"\nmeasured = \"\"\nevidence = []\nrecorded = {{ date = \"\", session = \"\", implemented_by = \"\" }}\n",
            status.as_str()
        );
        let doc = BaseDocument::parse_str(&toml, "synthetic:status.toml")
            .unwrap_or_else(|e| panic!("valid status `{}` failed: {e}", status.as_str()));
        assert_eq!(doc.gates[0].derived_status(), status);
    }

    assert_eq!(Status::NotRun.printed_form(None), "not run");
    assert_eq!(
        Status::Killed.printed_form(Some("fallback-solver")),
        "killed: fallback adopted (fallback-solver)"
    );
    assert_eq!(
        Status::Blocked.printed_form(Some("OPEN-GAUNT-04")),
        "blocked: OPEN-GAUNT-04"
    );

    // Invalid statuses "passed", "ok", and "" are rejected naming the gate.
    for bad in ["passed", "ok", ""] {
        let toml = format!(
            "schema = \"frob.gates/1\"\n\n[[gate]]\nid = \"GATE-BADSTAT-09\"\nclaim = \"known-answer\"\nclaim_id = \"clm:0000000000000000000000000000000000000000000000000000000000000009\"\nsection = \"38.13\"\nthreshold = \"t\"\nmilestone = \"M0\"\ntier = [\"G0:host\"]\ncontrols = [\"NC-0001\"]\n\n[[gate.part]]\nmilestone = \"M0\"\nstatus = \"{bad}\"\nmeasured = \"\"\nevidence = []\nrecorded = {{ date = \"\", session = \"\", implemented_by = \"\" }}\n"
        );
        let err = BaseDocument::parse_str(&toml, "synthetic:bad_status.toml")
            .expect_err(&format!("bad status `{bad}` must fail"));
        assert!(
            err.gate_id
                .as_deref()
                .is_some_and(|g| g.starts_with("GATE-BADSTAT-09")),
            "error must name gate GATE-BADSTAT-09, got: {err}"
        );
    }
}

#[test]
fn two_part_record_roundtrip() {
    // synthetic fixture: two-part gate due at M1 (pass) and M3 (fail).
    let src = r#"schema = "frob.gates/1"

[[gate]]
id = "GATE-MULTI-02"
claim = "known-answer"
claim_id = "clm:0000000000000000000000000000000000000000000000000000000000000002"
section = "13.1"
threshold = "laws hold for SO(3)/SE(3) at M1 and complete table at M3"
milestone = "M1"
tier = ["G0:host"]
controls = ["NC-0101"]

[[gate.part]]
milestone = "M1"
status = "pass"
measured = "12 of 12 SO(3)/SE(3) laws"
evidence = ["ev:0001"]
recorded = { date = "2026-11-24", session = "FreshReviewer", implemented_by = "BuilderA" }

[[gate.part]]
milestone = "M3"
status = "fail"
measured = "40 of 42 full laws"
evidence = ["ev:0002"]
recorded = { date = "2026-11-24", session = "FreshReviewer", implemented_by = "BuilderA" }
"#;
    let doc =
        BaseDocument::parse_str(src, "synthetic:two_part.toml").expect("two-part gate must parse");
    let g = &doc.gates[0];
    assert_eq!(g.parts().len(), 2);
    assert_eq!(g.derived_status_at("M0"), Status::NotRun);
    assert_eq!(g.printed_status(Some("M0")), "not run");
    assert_eq!(g.derived_status_at("M1"), Status::Pass);
    assert_eq!(g.derived_status_at("M2"), Status::Pass);
    assert_eq!(g.derived_status_at("M3"), Status::Fail);
    assert_eq!(g.derived_status(), Status::Fail);
    assert_eq!(
        doc.print_toml(),
        src,
        "two-part roundtrip must be byte-identical"
    );

    // A gate with no parts recorded derives NotRun and prints "not run".
    let zero_part = r#"schema = "frob.gates/1"

[[gate]]
id = "GATE-NOPART-01"
claim = "proof"
claim_id = "clm:0000000000000000000000000000000000000000000000000000000000000003"
section = "12.1"
threshold = "checker accepts specimen"
milestone = "M1"
tier = ["G0:host"]
controls = ["NC-0201"]
"#;
    let zp = BaseDocument::parse_str(zero_part, "synthetic:zero_part.toml")
        .expect("zero-part gate must parse");
    assert_eq!(zp.gates[0].derived_status(), Status::NotRun);
    assert_eq!(zp.gates[0].derived_status_at("M8"), Status::NotRun);
    assert_eq!(zp.gates[0].printed_status(None), "not run");
}

#[test]
fn missing_base_field_names_gate() {
    let base_lines: [(&str, &str); 7] = [
        ("claim", "claim = \"known-answer\""),
        (
            "claim_id",
            "claim_id = \"clm:0000000000000000000000000000000000000000000000000000000000000001\"",
        ),
        ("section", "section = \"38.13\""),
        ("threshold", "threshold = \"exact\""),
        ("milestone", "milestone = \"M0\""),
        ("tier", "tier = [\"G0:host\"]"),
        ("controls", "controls = [\"NC-0001\"]"),
    ];

    for (omitted, _) in base_lines {
        let mut toml =
            String::from("schema = \"frob.gates/1\"\n\n[[gate]]\nid = \"GATE-MISS-42\"\n");
        for (name, line) in base_lines {
            if name != omitted {
                toml.push_str(line);
                toml.push('\n');
            }
        }
        let err = BaseDocument::parse_str(&toml, "synthetic:missing.toml")
            .expect_err(&format!("omitting `{omitted}` must fail"));
        assert_eq!(
            err.gate_id.as_deref(),
            Some("GATE-MISS-42"),
            "missing `{omitted}` error must name GATE-MISS-42: {err}"
        );
        assert!(
            err.message.contains(omitted),
            "missing `{omitted}` error must name the missing field: {err}"
        );
    }

    // Missing `id` also fails cleanly.
    let no_id = "schema = \"frob.gates/1\"\n\n[[gate]]\nclaim = \"known-answer\"\nclaim_id = \"clm:1\"\nsection = \"38.13\"\nthreshold = \"t\"\nmilestone = \"M0\"\ntier = [\"G0:host\"]\ncontrols = [\"NC-0001\"]\n";
    let err =
        BaseDocument::parse_str(no_id, "synthetic:no_id.toml").expect_err("missing id must fail");
    assert!(err.message.contains("id"));
}

#[test]
fn kill_row_carries_no_measurement_fields() {
    // synthetic fixture: clean kill row without any measurement fields.
    let kill_toml = r#"schema = "frob.gates/1"

[[gate]]
id = "KILL-SPARK-01"
kind = "kill"
section = "21.17"
decided = "M2"
"#;
    let doc = BaseDocument::parse_str(kill_toml, "synthetic:kill.toml")
        .expect("kill row without measurement fields must parse");
    assert_eq!(doc.gates.len(), 1);
    assert!(doc.gates[0].is_kill());
    assert_eq!(doc.gates[0].effective_kind(), BaseKind::Kill);
    assert!(doc.gates[0].measured.is_none());
    assert_eq!(doc.gates[0].decided.as_deref(), Some("M2"));
    assert_eq!(
        doc.print_toml(),
        kill_toml,
        "kill row must roundtrip identically"
    );

    // Adding any of the six measurement fields to a kill row fails naming gate and field.
    let forbidden_snippets: [(&str, &str); 6] = [
        ("claim", "claim = \"known-answer\""),
        ("claim_id", "claim_id = \"clm:0000\""),
        ("threshold", "threshold = \"x\""),
        ("tier", "tier = [\"G0:host\"]"),
        ("controls", "controls = [\"NC-0001\"]"),
        (
            "part",
            "[[gate.part]]\nmilestone = \"M2\"\nstatus = \"not-run\"\nmeasured = \"\"\nevidence = []\nrecorded = { date = \"\", session = \"\", implemented_by = \"\" }",
        ),
    ];
    assert_eq!(forbidden_snippets.len(), KILL_FORBIDDEN_FIELDS.len());
    for (field, snippet) in forbidden_snippets {
        let bad = format!("{kill_toml}{snippet}\n");
        let err = BaseDocument::parse_str(&bad, "synthetic:kill_bad.toml")
            .expect_err(&format!("kill row carrying `{field}` must fail"));
        assert_eq!(err.gate_id.as_deref(), Some("KILL-SPARK-01"));
        assert!(
            err.message.contains(field),
            "error must name forbidden field `{field}`: {err}"
        );
    }
}

#[test]
fn count_assertion() {
    let root = workspace_root();
    let exp = GateCountsExpectation::load(&root.join(GATE_COUNTS_PATH))
        .expect("committed registry/gate_counts.toml must load");

    // Known answer: plan-time inventory 421 = 278 + 111 + 32.
    assert_eq!(exp.plan_inventory.convergence, 278);
    assert_eq!(exp.plan_inventory.kill, 111);
    assert_eq!(exp.plan_inventory.admission, 32);
    assert_eq!(exp.plan_inventory.total, 421);
    assert_eq!(exp.plan_inventory.kill_intro_count, 112);
    assert!(
        exp.plan_inventory
            .kill_count_delta_report()
            .contains("delta=1")
    );
    // Round-4 review note #1198 (N40-02 step 5): 40 multi-part convergence gates + 3 admission.
    assert_eq!(exp.plan_inventory.multi_part_convergence, 40);
    assert_eq!(exp.plan_inventory.multi_part_admission, 3);

    // Synthetic 421-row first-inventory document (278 convergence + 111 kill + 32 admission).
    let mut synth = String::from("schema = \"frob.gates/1\"\n");
    for i in 1..=278 {
        let _ = write!(
            synth,
            "\n[[gate]]\nid = \"GATE-INV-{i:03}\"\nkind = \"convergence\"\nclaim = \"known-answer\"\nclaim_id = \"clm:{i:064x}\"\nsection = \"40.1\"\nthreshold = \"t\"\nmilestone = \"M1\"\ntier = [\"G1:host\"]\ncontrols = [\"NC-0001\"]\n"
        );
    }
    for i in 1..=111 {
        let _ = write!(
            synth,
            "\n[[gate]]\nid = \"KILL-INV-{i:03}\"\nkind = \"kill\"\nsection = \"40.1\"\ndecided = \"M2\"\n"
        );
    }
    for i in 1..=32 {
        let _ = write!(
            synth,
            "\n[[gate]]\nid = \"INT-INV-{i:03}\"\nkind = \"admission\"\nclaim = \"parity\"\nclaim_id = \"clm:{i:064x}\"\nsection = \"40.1\"\nthreshold = \"t\"\nmilestone = \"M0\"\ntier = [\"G0:host\"]\ncontrols = [\"NC-0001\"]\n"
        );
    }
    let inv_doc = BaseDocument::parse_str(&synth, "synthetic:inventory_421.toml")
        .expect("421-row inventory must parse");
    let inv_rep = exp
        .check_plan_inventory(&inv_doc)
        .expect("421-row inventory must satisfy plan-time count assertion");
    assert_eq!(
        inv_rep.observed,
        KindCounts {
            convergence: 278,
            kill: 111,
            admission: 32,
            program: 0,
            total: 421,
        }
    );

    // Committed seeded registries/gates.toml matches the latest dated revision.
    let seeded = BaseDocument::load(&root.join("registries/gates.toml"))
        .expect("committed registries/gates.toml must parse");
    let rev_rep = exp
        .check_latest(&seeded)
        .expect("seeded registries/gates.toml must match latest dated revision");
    assert_eq!(rev_rep.observed.total, seeded.gates.len());

    // Planting an extra record fails the count assertion and names the planted ID.
    let mut with_extra = seeded.print_toml();
    with_extra.push_str(
        "\n[[gate]]\nid = \"GATE-PLANTED-999\"\nclaim = \"known-answer\"\nclaim_id = \"clm:999\"\nsection = \"38.13\"\nthreshold = \"planted\"\nmilestone = \"M0\"\ntier = [\"G0:host\"]\ncontrols = [\"NC-0001\"]\n",
    );
    let extra_doc = BaseDocument::parse_str(&with_extra, "synthetic:extra.toml")
        .expect("planted extra doc parses structurally");
    let mismatch = exp
        .check_latest(&extra_doc)
        .expect_err("planted extra gate must fail count assertion");
    assert_eq!(mismatch.extra_ids, vec!["GATE-PLANTED-999"]);
    assert!(mismatch.missing_ids.is_empty());
}

#[test]
fn permuting_rows_leaves_counts_and_sorted_ids_invariant() {
    let root = workspace_root();
    let seeded = BaseDocument::load(&root.join("registries/gates.toml"))
        .expect("committed registries/gates.toml must parse");
    let mut reversed = seeded.clone();
    reversed.gates.reverse();

    let rev_doc = BaseDocument::parse_str(&reversed.print_toml(), "synthetic:permuted.toml")
        .expect("permuted doc must parse");
    assert_eq!(seeded.counts_by_kind(), rev_doc.counts_by_kind());
    assert_eq!(seeded.counts_by_status(), rev_doc.counts_by_status());
    assert_eq!(seeded.counts_by_claim(), rev_doc.counts_by_claim());
}

#[test]
fn schema_contract_and_isolation_hold() {
    let root = workspace_root();
    let schema = fs::read_to_string(root.join("schemas/registries/gates.schema.json"))
        .expect("read schemas/registries/gates.schema.json");
    for needle in [
        "frob.gates/1",
        "design-comparison",
        "no-claim",
        "superseded",
        "blocked",
        "controls",
        "implemented_by",
        "FROB-REF-869",
        "receipts/gates/",
        "\"kill\"",
        "40 convergence gates",
    ] {
        assert!(
            schema.contains(needle),
            "gates.schema.json missing required contract clause `{needle}`"
        );
    }

    let contract = fs::read_to_string(root.join("crates/frob-conformance/CONTRACT.md"))
        .expect("read frob-conformance/CONTRACT.md");
    for needle in [
        "N38b-18",
        "recorded.implemented_by",
        "FROB-REF-869",
        "killed: fallback adopted",
        "40 convergence gates",
    ] {
        assert!(
            contract.contains(needle),
            "frob-conformance/CONTRACT.md missing `{needle}`"
        );
    }

    // Dependency isolation: src/register/{mod,row,counts}.rs import only std and frob_core.
    for rel in [
        "crates/frob-conformance/src/register/mod.rs",
        "crates/frob-conformance/src/register/row.rs",
        "crates/frob-conformance/src/register/counts.rs",
    ] {
        let src = fs::read_to_string(root.join(rel)).expect("read register source");
        for line in src.lines() {
            let t = line.trim();
            if t.starts_with("use ") {
                assert!(
                    t.starts_with("use std::")
                        || t.starts_with("use frob_core::")
                        || t.starts_with("use crate::register::"),
                    "{rel} must not depend outside std and frob_core: `{t}`"
                );
            }
        }
    }
}
