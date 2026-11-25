//! Conformance and unit tests for the hypothesis verdict register (`N00-10`).

use std::path::PathBuf;

use frob_conformance::hypotheses::{
    CERT_DISAGREEMENT_ID, CertEffectTotals, EvidenceKind, HYPOTHESES_SCHEMA, HistoryEntry,
    HypothesisRegister, NO_CERT_LEDGER_LINE, RULE_ACTIVE_UNSETTLED, RULE_BANNED_WORDING,
    RULE_DEFERRED_AS_LEVER, RULE_HISTORY_EDITED, RULE_NEW_ENTRY_MISSING_RECORD,
    RULE_PROPOSAL_ONLY_CANNOT_MOVE, RULE_REFUTED_BOUND, RULE_REFUTED_MOVE_WITHOUT_REVERSAL,
    RULE_REVERSAL_EDITED, RULE_REVERSAL_WITHOUT_OWNER_DECISION, RULE_ROW_REMOVED,
    RULE_STATE_CHANGE_WITHOUT_ENTRY, RULE_UNLABELLED_NUMBER, RegisterError, ScopeFinding,
    VerdictState, ZERO_CHANGED_DEFAULT_TEXT, banned, check_change_rule, embedded,
    format_cert_effect_line, h22b_paired_gain, lint_text, lint_tree, load_base_from_git,
    render_exit_review_report, scope, scope_with_register,
};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .expect("crates/frob-conformance sits two levels below repo root")
        .to_path_buf()
}

#[test]
fn counts_by_state_at_conversion() {
    let reg = HypothesisRegister::load_embedded().expect("embedded hypotheses.toml loads");
    assert_eq!(reg.schema, HYPOTHESES_SCHEMA);
    let counts = reg.counts_by_state();
    assert_eq!(counts.total, 32);
    assert_eq!(counts.refuted, 4);
    assert_eq!(counts.deferred, 1);
    assert_eq!(counts.active, 11);
    assert_eq!(counts.narrowed, 16);
    assert_eq!(reg.disagreements.len(), 8);
    assert_eq!(reg.open_disagreements().len(), 8);

    // Exact membership by state per N00-10 AC 1.
    for id in ["H-04", "H-09", "H-15", "H-16"] {
        assert_eq!(reg.get(id).expect(id).state, VerdictState::Refuted);
    }
    assert_eq!(reg.get("H-12").expect("H-12").state, VerdictState::Deferred);
    for id in [
        "H-04b", "H-12b", "H-18", "H-19", "H-22a", "H-22b", "H-25", "H-26", "H-27", "H-28", "H-29",
    ] {
        assert_eq!(reg.get(id).expect(id).state, VerdictState::Active);
    }

    // H-04 has recorded reversal; H-09, H-15, H-16 start with reversal_proposed only.
    assert!(reg.get("H-04").unwrap().reversal.is_some());
    for id in ["H-09", "H-15", "H-16"] {
        let row = reg.get(id).unwrap();
        assert!(row.reversal.is_none(), "{id} must start without recorded reversal");
        assert!(
            row.reversal_proposed.is_some(),
            "{id} must carry reversal_proposed"
        );
    }
}

#[test]
fn history_is_append_only() {
    let base = HypothesisRegister::load_embedded().unwrap();

    // Editing an existing history entry's evidence fails.
    let mut edited = base.clone();
    edited.get_mut("H-01").unwrap().history[0].evidence = "rewritten history".to_owned();
    let v_edit = check_change_rule(&base, &edited);
    assert!(v_edit.iter().any(|v| v.rule == RULE_HISTORY_EDITED && v.row_id == "H-01"));

    // Deleting a history entry fails.
    let mut shrunk = base.clone();
    shrunk.get_mut("H-01").unwrap().history.clear();
    let v_shrink = check_change_rule(&base, &shrunk);
    assert!(v_shrink.iter().any(|v| v.rule == RULE_HISTORY_EDITED && v.row_id == "H-01"));

    // Deleting an entire hypothesis row fails.
    let mut removed = base.clone();
    removed.hypotheses.retain(|h| h.id != "H-05");
    let v_rem = check_change_rule(&base, &removed);
    assert!(v_rem.iter().any(|v| v.rule == RULE_ROW_REMOVED && v.row_id == "H-05"));
}

#[test]
fn state_change_needs_evidence_kind() {
    // 1. Invalid evidence kind is rejected at TOML load time.
    let bad_kind_toml = r#"
schema = "frob.hypothesis/1"
[[hypothesis]]
id = "H-99"
statement = "Synthetic row"
state = "ACTIVE"
survives = "Claim"
binds = ["BET-99"]
settled_by = ["GATE-99"]
history = [
  { date = "2026-11-24", state = "ACTIVE", evidence = "opinion", kind = "hand-waving" }
]
"#;
    assert!(matches!(
        HypothesisRegister::from_bytes(bad_kind_toml.as_bytes()),
        Err(RegisterError::InvalidField { ref field, .. }) if field == "history[0].kind"
    ));

    // 2. Changing state without appending a new history entry fails both loader and change rule.
    let base = HypothesisRegister::load_embedded().unwrap();
    let mut no_entry = base.clone();
    no_entry.get_mut("H-18").unwrap().state = VerdictState::Narrowed;
    let v_no_entry = check_change_rule(&base, &no_entry);
    assert!(
        v_no_entry
            .iter()
            .any(|v| v.rule == RULE_STATE_CHANGE_WITHOUT_ENTRY && v.row_id == "H-18")
    );

    // 3. New history entry without a cited ledger/gate/source record fails.
    let mut no_record = base.clone();
    let row = no_record.get_mut("H-18").unwrap();
    row.state = VerdictState::Narrowed;
    row.history.push(HistoryEntry {
        date: "2026-12-01".to_owned(),
        state: VerdictState::Narrowed,
        evidence: "measured split".to_owned(),
        kind: EvidenceKind::GateMeasurement,
        record: None,
        reversal_met: false,
        owner_decision: None,
    });
    let v_no_rec = check_change_rule(&base, &no_record);
    assert!(
        v_no_rec
            .iter()
            .any(|v| v.rule == RULE_NEW_ENTRY_MISSING_RECORD && v.row_id == "H-18")
    );

    // 4. Appending a new entry with an allowed kind AND cited record passes cleanly.
    let mut valid_move = base.clone();
    let row_ok = valid_move.get_mut("H-18").unwrap();
    row_ok.state = VerdictState::Narrowed;
    row_ok.history.push(HistoryEntry {
        date: "2026-12-01".to_owned(),
        state: VerdictState::Narrowed,
        evidence: "M3 phase ledger split measured".to_owned(),
        kind: EvidenceKind::GateMeasurement,
        record: Some("GATE-TENFOLD-01:rec-001".to_owned()),
        reversal_met: false,
        owner_decision: None,
    });
    assert!(check_change_rule(&base, &valid_move).is_empty());
}

#[test]
fn refuted_row_moves_only_by_reversal() {
    let base = HypothesisRegister::load_embedded().unwrap();

    // Moving H-04 (which has a recorded reversal) without reversal_met = true fails.
    let mut without_rev = base.clone();
    let h04 = without_rev.get_mut("H-04").unwrap();
    h04.state = VerdictState::Active;
    h04.history.push(HistoryEntry {
        date: "2026-12-10".to_owned(),
        state: VerdictState::Active,
        evidence: "re-checked trace".to_owned(),
        kind: EvidenceKind::GateMeasurement,
        record: Some("GATE-SPARK-14:rec-01".to_owned()),
        reversal_met: false,
        owner_decision: None,
    });
    let v_bad = check_change_rule(&base, &without_rev);
    assert!(
        v_bad
            .iter()
            .any(|v| v.rule == RULE_REFUTED_MOVE_WITHOUT_REVERSAL && v.row_id == "H-04")
    );

    // Moving H-04 WITH reversal_met = true and cited gate record passes.
    let mut with_rev = base.clone();
    let h04_ok = with_rev.get_mut("H-04").unwrap();
    h04_ok.state = VerdictState::Active;
    h04_ok.history.push(HistoryEntry {
        date: "2026-12-10".to_owned(),
        state: VerdictState::Active,
        evidence: "M2 profile measured specializable share 0.39 >= 1/3 reversal condition".to_owned(),
        kind: EvidenceKind::GateMeasurement,
        record: Some("GATE-SPARK-14:rec-02".to_owned()),
        reversal_met: true,
        owner_decision: None,
    });
    assert!(check_change_rule(&base, &with_rev).is_empty());
}

#[test]
fn refuted_without_reversal_or_proposal_rejected() {
    let bad_toml = r#"
schema = "frob.hypothesis/1"
[[hypothesis]]
id = "H-04"
statement = "Supercompiled kernels"
state = "REFUTED"
survives = "CPU derived kernels survive"
binds = ["BET-01"]
settled_by = ["GATE-SPARK-14"]
history = [
  { date = "2026-11-21", state = "REFUTED", evidence = "trace share 18%", kind = "recomputation" }
]
"#;
    assert_eq!(
        HypothesisRegister::from_bytes(bad_toml.as_bytes()),
        Err(RegisterError::RefutedMissingReversal {
            row_id: "H-04".to_owned()
        })
    );
}

#[test]
fn proposal_only_row_cannot_move() {
    let base = HypothesisRegister::load_embedded().unwrap();

    // H-15 holds only reversal_proposed; any appended entry attempting to move it fails closed.
    let mut moved = base.clone();
    let h15 = moved.get_mut("H-15").unwrap();
    h15.state = VerdictState::Active;
    h15.history.push(HistoryEntry {
        date: "2026-12-15".to_owned(),
        state: VerdictState::Active,
        evidence: "claimed proposal met".to_owned(),
        kind: EvidenceKind::Pilot,
        record: Some("PILOT-15:rec-01".to_owned()),
        reversal_met: true,
        owner_decision: None,
    });
    let violations = check_change_rule(&base, &moved);
    assert!(
        violations
            .iter()
            .any(|v| v.rule == RULE_PROPOSAL_ONLY_CANNOT_MOVE && v.row_id == "H-15")
    );

    // Even staying REFUTED with a synthetic reversal_met entry on proposal-only H-09 is rejected.
    let mut h09_try = base.clone();
    h09_try.get_mut("H-09").unwrap().history.push(HistoryEntry {
        date: "2026-12-15".to_owned(),
        state: VerdictState::Refuted,
        evidence: "attempted reopening without owner-recorded reversal".to_owned(),
        kind: EvidenceKind::Pilot,
        record: Some("PILOT-09:rec-01".to_owned()),
        reversal_met: true,
        owner_decision: None,
    });
    let v_h09 = check_change_rule(&base, &h09_try);
    assert!(
        v_h09
            .iter()
            .any(|v| v.rule == RULE_PROPOSAL_ONLY_CANNOT_MOVE && v.row_id == "H-09")
    );
}

#[test]
fn reversal_set_once_by_owner_record() {
    let base = HypothesisRegister::load_embedded().unwrap();

    // 1. Setting reversal from absent WITH NO owner decision record fails.
    let mut no_dec = base.clone();
    no_dec.get_mut("H-15").unwrap().reversal =
        Some("kappa * u < 1 in >= 99% of contact battery states after scaling".to_owned());
    let v_no_dec = check_change_rule(&base, &no_dec);
    assert!(
        v_no_dec
            .iter()
            .any(|v| v.rule == RULE_REVERSAL_WITHOUT_OWNER_DECISION && v.row_id == "H-15")
    );

    // 2. Setting reversal from absent WITH an owner decision record passes.
    let mut adopted = base.clone();
    let h15 = adopted.get_mut("H-15").unwrap();
    h15.reversal =
        Some("kappa * u < 1 in >= 99% of contact battery states after scaling".to_owned());
    h15.reversal_decision = Some("ADR-HYP-15".to_owned());
    assert!(check_change_rule(&base, &adopted).is_empty());

    // 3. Editing reversal afterwards is forbidden.
    let mut edited = adopted.clone();
    edited.get_mut("H-15").unwrap().reversal = Some("lowered bar: 50% of states".to_owned());
    let v_edit = check_change_rule(&adopted, &edited);
    assert!(
        v_edit
            .iter()
            .any(|v| v.rule == RULE_REVERSAL_EDITED && v.row_id == "H-15")
    );
}

#[test]
fn banned_wording_flagged() {
    let reg = embedded();
    let doc = "Goal: the simulator uses half precision for contact solves to double speed.\n";
    let findings = lint_text(reg, "docs/proposal.md", doc);
    assert!(
        findings
            .iter()
            .any(|f| f.rule == RULE_BANNED_WORDING && f.row_id == "H-15" && f.pattern_index == Some(0)),
        "expected hyp.banned-wording for H-15, got {findings:?}"
    );

    // Shared pattern list accessor used by NAB-32 exposes the exact same list.
    let h15_pats = banned("H-15").expect("H-15 banned patterns");
    assert!(!h15_pats.is_empty());

    // Exempt paths (killed-idea register) produce zero findings.
    assert!(lint_text(reg, "registries/killed_ideas.toml", doc).is_empty());
    assert!(lint_text(reg, "docs/decisions/killed-ideas.md", doc).is_empty());

    // ACTIVE row with banned pattern (H-27) is also flagged as hyp.banned-wording.
    let h27_doc = "We highlight gradients through contact as a differentiator for Spark.\n";
    let h27_findings = lint_text(reg, "docs/marketing.md", h27_doc);
    assert!(
        h27_findings
            .iter()
            .any(|f| f.rule == RULE_BANNED_WORDING && f.row_id == "H-27"),
        "expected H-27 banned wording finding, got {h27_findings:?}"
    );
}

#[test]
fn deferred_mechanism_in_lever_list_flagged() {
    let reg = embedded();
    let doc = "Our ten-fold levers: family batches, generative structure, and sparse solves.\n";
    let findings = lint_text(reg, "docs/roadmap.md", doc);
    assert!(
        findings
            .iter()
            .any(|f| f.rule == RULE_DEFERRED_AS_LEVER && f.row_id == "H-12"),
        "expected hyp.deferred-as-lever for H-12, got {findings:?}"
    );
    // Must NOT be double-reported as hyp.banned-wording.
    assert!(
        !findings
            .iter()
            .any(|f| f.rule == RULE_BANNED_WORDING && f.row_id == "H-12"),
        "DEFERRED row H-12 must not double-report as hyp.banned-wording"
    );
}

#[test]
fn active_claim_number_needs_hypothesis_label() {
    let reg = embedded();
    let bad = "H-04b cuts factor operation counts by 2.6x and covers 62% of the step.\n";
    let findings = lint_text(reg, "docs/notes.md", bad);
    assert!(
        findings
            .iter()
            .any(|f| f.rule == RULE_UNLABELLED_NUMBER && f.row_id == "H-04b"),
        "expected hyp.unlabelled-number on H-04b, got {findings:?}"
    );

    let ok_hyp = "H-04b cuts factor operation counts by 2.6x (HYPOTHESIS).\n";
    assert!(lint_text(reg, "docs/notes.md", ok_hyp).is_empty());

    let ok_target = "H-07 achieves 10-100x at 1e-4 (TARGET).\n";
    assert!(lint_text(reg, "docs/notes.md", ok_target).is_empty());
}

#[test]
fn disagreement_settled_needs_gate_record() {
    let bad_toml = r#"
schema = "frob.hypothesis/1"
[[disagreement]]
id = "OD-01"
question = "Whether a thin layer over an incumbent recovers the gain"
settled_by = ["BET-14"]
status = "settled"
"#;
    assert_eq!(
        HypothesisRegister::from_bytes(bad_toml.as_bytes()),
        Err(RegisterError::SettledDisagreementMissingRecord {
            id: "OD-01".to_owned()
        })
    );

    let ok_toml = r#"
schema = "frob.hypothesis/1"
[[disagreement]]
id = "OD-01"
question = "Whether a thin layer over an incumbent recovers the gain"
settled_by = ["BET-14"]
status = "settled"
record = "GATE-TENFOLD-01:rec-m7"
"#;
    let reg = HypothesisRegister::from_bytes(ok_toml.as_bytes()).expect("settled with record loads");
    assert!(reg.open_disagreements().is_empty());
}

#[test]
fn scope_refuted_reason_until_reversal() {
    let reason = "Enable per-design kernel specialization as the default to accelerate batches";
    // Before any reversal entry on H-04 -> hyp.refuted-bound.
    let before = scope(reason, &["GATE-SPARK-14"]);
    assert!(
        before.iter().any(|f| matches!(
            f,
            ScopeFinding::RefutedBound {
                rule,
                row_id,
                pattern_index: 0,
                reversal: Some(_),
                proposal_only: false,
                ..
            } if *rule == RULE_REFUTED_BOUND && row_id == "H-04"
        )),
        "expected hyp.refuted-bound for H-04 before reversal, got {before:?}"
    );

    // After a synthetic reversal entry records H-04's reversal condition met -> cleared.
    let mut after_reg = HypothesisRegister::load_embedded().unwrap();
    after_reg.get_mut("H-04").unwrap().history.push(HistoryEntry {
        date: "2026-12-20".to_owned(),
        state: VerdictState::Refuted,
        evidence: "synthetic: M2 profile showed 0.40 specializable share".to_owned(),
        kind: EvidenceKind::GateMeasurement,
        record: Some("GATE-SPARK-14:synthetic".to_owned()),
        reversal_met: true,
        owner_decision: None,
    });
    let after = scope_with_register(&after_reg, reason, &["GATE-SPARK-14"]);
    assert!(
        !after
            .iter()
            .any(|f| f.rule() == RULE_REFUTED_BOUND && f.row_id() == "H-04"),
        "H-04 hyp.refuted-bound must clear after reversal entry, got {after:?}"
    );
}

#[test]
fn scope_proposal_only_row_never_clears() {
    let reason = "Run contact solves in half precision on tensor cores";
    let mut reg = HypothesisRegister::load_embedded().unwrap();
    // Even if a synthetic history entry sets reversal_met = true on proposal-only H-15,
    // no entry can satisfy a proposal until the owner records `reversal`.
    reg.get_mut("H-15").unwrap().history.push(HistoryEntry {
        date: "2026-12-20".to_owned(),
        state: VerdictState::Refuted,
        evidence: "synthetic entry on proposal-only row".to_owned(),
        kind: EvidenceKind::Pilot,
        record: Some("PILOT-15:synthetic".to_owned()),
        reversal_met: true,
        owner_decision: None,
    });
    let findings = scope_with_register(&reg, reason, &["BET-08"]);
    assert!(
        findings.iter().any(|f| matches!(
            f,
            ScopeFinding::RefutedBound {
                rule,
                row_id,
                reversal: None,
                reversal_proposed: Some(_),
                proposal_only: true,
                ..
            } if *rule == RULE_REFUTED_BOUND && row_id == "H-15"
        )),
        "proposal-only H-15 must stay hyp.refuted-bound, got {findings:?}"
    );
}

#[test]
fn scope_active_bound_names_settled_by() {
    let findings = scope(
        "Build sparse tree-ordered constraint solver",
        &["GATE-SPARK-15"],
    );
    assert!(
        findings.iter().any(|f| matches!(
            f,
            ScopeFinding::ActiveUnsettled {
                rule,
                row_id,
                matched_bind,
                settled_by,
            } if *rule == RULE_ACTIVE_UNSETTLED
                && row_id == "H-04b"
                && matched_bind == "GATE-SPARK-15"
                && settled_by.iter().any(|s| s == "GATE-SPARK-15")
        )),
        "expected hyp.active-unsettled naming H-04b and settled_by, got {findings:?}"
    );
}

#[test]
fn certificate_disagreement_reads_cert_effect_totals() {
    let reg = embedded();

    // At M0 with no decision-effect ledger present: exact required line.
    assert_eq!(format_cert_effect_line(None), NO_CERT_LEDGER_LINE);
    let m0_report = render_exit_review_report(reg, None);
    assert!(m0_report.contains(CERT_DISAGREEMENT_ID));
    assert!(m0_report.contains(NO_CERT_LEDGER_LINE));

    // Synthetic read-out matching frob.prove.effects.out/1 (N12-12 three fixtures: 4, 2, 1, 1).
    let raw = br#"{
        "schema": "frob.prove.effects.out/1",
        "totals": {
            "consumed": 4,
            "changed": 2,
            "unchanged": 1,
            "unknown": 1
        }
    }"#;
    let totals = CertEffectTotals::from_effects_json(raw).expect("parse cert_effect totals");
    assert_eq!(totals.consumed, 4);
    assert_eq!(totals.changed, 2);
    assert_eq!(totals.unchanged, 1);
    assert_eq!(totals.unknown, 1);

    let line = format_cert_effect_line(Some(&totals));
    assert!(line.contains("consumed 4, changed 2, unchanged 1, unknown 1"));
    assert!(!line.contains("changed 3"), "unknown must never be added to changed");

    let m1_report = render_exit_review_report(reg, Some(&totals));
    assert!(m1_report.contains("consumed 4, changed 2, unchanged 1, unknown 1"));

    // Zero-changed read-out emits the fail-closed default text from N12-12.
    let zero_raw = br#"{
        "schema": "frob.prove.effects.out/1",
        "totals": { "consumed": 2, "changed": 0, "unchanged": 1, "unknown": 1 }
    }"#;
    let zero_totals = CertEffectTotals::from_effects_json(zero_raw).unwrap();
    let zero_line = format_cert_effect_line(Some(&zero_totals));
    assert!(zero_line.contains(ZERO_CHANGED_DEFAULT_TEXT));
}

#[test]
fn h22b_gain_known_answer() {
    // Known answers from N00-10: 1/(1 - rho^2) = 1.96, 2.78, 5.26 at rho = 0.7, 0.8, 0.9.
    assert!((h22b_paired_gain(0.7) - (1.0 / 0.51)).abs() < 1e-12);
    assert!((h22b_paired_gain(0.8) - (1.0 / 0.36)).abs() < 1e-12);
    assert!((h22b_paired_gain(0.9) - (1.0 / 0.19)).abs() < 1e-12);
    assert_eq!(format!("{:.2}", h22b_paired_gain(0.7)), "1.96");
    assert_eq!(format!("{:.2}", h22b_paired_gain(0.8)), "2.78");
    assert_eq!(format!("{:.2}", h22b_paired_gain(0.9)), "5.26");
}

#[test]
fn exit_review_report_change_rule_and_repo_tree_lint() {
    let root = repo_root();
    let reg = HypothesisRegister::from_path(
        &root.join("crates/frob-conformance/registry/hypotheses.toml"),
    )
    .expect("load workspace hypotheses.toml");

    // Exit-review report lists all 8 open disagreements with their settling tests.
    let report = render_exit_review_report(&reg, None);
    for id in [
        "OD-01", "OD-02", "OD-03", "OD-04", "OD-05", "OD-06", "OD-07", "OD-08",
    ] {
        assert!(report.contains(id), "report must list open disagreement {id}");
    }
    for test in ["BET-14", "BET-17", "BET-15", "BET-16", "BET-11", "BET-20"] {
        assert!(report.contains(test), "report must name settling test {test}");
    }

    // Git append-only change rule holds against base commit (or initial introduction).
    if let Some(base) = load_base_from_git(&root, "crates/frob-conformance/registry/hypotheses.toml")
        .expect("load git base")
    {
        let violations = check_change_rule(&base, &reg);
        assert!(
            violations.is_empty(),
            "committed hypotheses.toml violated append-only change rule: {violations:?}"
        );
    }

    // Wording lint passes cleanly over workspace docs and crate CONTRACT.md files.
    let tree_findings = lint_tree(&reg, &root).expect("lint workspace tree");
    assert!(
        tree_findings.is_empty(),
        "workspace tree wording lint failed: {tree_findings:?}"
    );
}
