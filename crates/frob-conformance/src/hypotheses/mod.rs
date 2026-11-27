//! Hypothesis verdict register, append-only change rule, wording lint, scope binding,
//! open disagreements, and certificate decision-effect read-out (`N00-10`, §0.8, Appendix K.4).
//!
//! Determinism class D0; zero `unsafe`; strict TOML/JSON readers via `frob-core`.

pub mod lint;
pub mod report;
pub mod rules;
pub mod schema;

use std::sync::OnceLock;

pub use lint::{
    EXEMPT_PATHS, Pattern, RULE_BANNED_WORDING, RULE_DEFERRED_AS_LEVER, RULE_UNLABELLED_NUMBER,
    WordingFinding, is_exempt_path, lint_file, lint_text, lint_tree,
};
pub use report::{
    CERT_DISAGREEMENT_ID, CERT_EFFECTS_SCHEMA, CertEffectTotals, NO_CERT_LEDGER_LINE,
    ZERO_CHANGED_DEFAULT_TEXT, format_cert_effect_line, h22b_paired_gain,
    render_exit_review_report,
};
pub use rules::{
    ChangeRuleViolation, RULE_DISAGREEMENT_REMOVED, RULE_HISTORY_EDITED,
    RULE_NEW_ENTRY_MISSING_RECORD, RULE_PROPOSAL_ONLY_CANNOT_MOVE,
    RULE_REFUTED_MOVE_WITHOUT_REVERSAL, RULE_REVERSAL_EDITED, RULE_REVERSAL_WITHOUT_OWNER_DECISION,
    RULE_ROW_REMOVED, RULE_STATE_CHANGE_WITHOUT_ENTRY, RULE_STATE_HISTORY_MISMATCH,
    check_change_rule, load_base_from_git,
};
pub use schema::{
    DisagreementRow, DisagreementStatus, EMBEDDED_HYPOTHESES_TOML, EvidenceKind, HYPOTHESES_SCHEMA,
    HistoryEntry, HypothesisRegister, HypothesisRow, RULE_ACTIVE_UNSETTLED, RULE_REFUTED_BOUND,
    RegisterError, ScopeFinding, StateCounts, VerdictState, scope_with_register,
};

static EMBEDDED_REGISTER: OnceLock<HypothesisRegister> = OnceLock::new();

/// Access the embedded committed hypothesis verdict register (`frob.hypothesis/1`).
#[must_use]
pub fn embedded() -> &'static HypothesisRegister {
    EMBEDDED_REGISTER.get_or_init(|| {
        HypothesisRegister::load_embedded()
            .expect("embedded crates/frob-conformance/registry/hypotheses.toml must validate")
    })
}

/// Return the single authoritative `banned` pattern list for hypothesis row `id` (`N00-10`
/// step 3, shared with `NAB-32`'s killed-idea revive lint).
#[must_use]
pub fn banned(id: &str) -> Option<&'static [String]> {
    embedded().banned_patterns(id)
}

/// Evaluate scope bindings for a work package's stated `reason` and listed `gates` against
/// the committed hypothesis verdict register (`N00-10` step 4, called by `N00-09`).
#[must_use]
pub fn scope(reason: &str, gates: &[&str]) -> Vec<ScopeFinding> {
    scope_with_register(embedded(), reason, gates)
}
