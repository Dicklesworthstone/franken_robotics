#![deny(unsafe_code)]

//! Gate-register (`frob.gates/1`) base records (`N38b-18`), full register types, nine-rule
//! validator, ten write-time refusals, and validated write path (`N40-01`, `ADR-GAUNT-04`).
//!
//! `row.rs` and `counts.rs` depend strictly on `std` and `frob-core`. `validate.rs` and
//! `write.rs` extend them additively with full `GateRow`, `Part`, `Kind`, `Needs`,
//! cross-file receipt/control/artifact-graph validation, and atomic write-time refusal checks.

pub mod counts;
pub mod kill;
pub mod print;
pub mod record;
pub mod row;
pub mod validate;
pub mod write;

pub use counts::{
    CountMismatch, CountReport, CountRevision, GATE_COUNTS_PATH, GATE_COUNTS_SCHEMA,
    GateCountsExpectation, PlanInventory,
};
pub use kill::{
    Amendment, AmendmentField, DECISION_ADR_DIR, DecisionReading, KillDecision, KillDueContext,
    KillDueItem, KillDueStatus, KillMode, KillOutcome, KillValidationContext, LiveIdExpectation,
    STANDING_OR_ROLLING_GATES, ScopeKind, SeededKillRow, adr_exists, apply_decisions,
    canonical_decision_adr_path, decisions_in_force, decisions_in_force_at, due_kill_gates,
    effective_decided_milestone, effective_fallback_text, format_introduce_inline_table,
    kill_row_derived_state, kill_row_seed_hash, parse_introduce_inline_table,
    parse_iso_timestamp_key, parse_live_ids_file, render_readme_status_table,
    render_register_printout, verify_live_kill_ids,
};
pub use row::{
    BaseDocument, BaseKind, BaseRowError, Claim, GATES_SCHEMA, GateBase, KILL_FORBIDDEN_FIELDS,
    KindCounts, MeasuredFields, PartBase, Recorded, Status, parse_milestone_ord,
};
pub use validate::{
    ADDITIVE_TOP_TABLES, AdmissionFields, COMPILED_RECURRING_GATES, GateRow, KillFields, Kind,
    LATE_RECURRING_GATES, Needs, Part, Register, ValidationContext, Violation,
    check_append_only_ids, has_valid_killed_decision, is_known_recurring_gate,
    is_session_implementer, is_valid_blocked_detail, is_valid_pilot_id, is_valid_source_citation,
    load, load_with_context, parse_and_validate_bytes, parse_register_bytes, validate,
};
pub use write::{record, record_in_memory};

#[cfg(test)]
#[path = "base_tests.rs"]
mod base;
