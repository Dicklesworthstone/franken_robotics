#![deny(unsafe_code)]

//! Base gate-register records and count expectations (`N38b-18`, schema `frob.gates/1`).
//!
//! Depends strictly on `std` and `frob-core`. Extended additively by `N40-01` (`GateRow`,
//! `Part`, `Kind`, `validate.rs`, `write.rs`).

pub mod counts;
pub mod row;

pub use counts::{
    CountMismatch, CountReport, CountRevision, GATE_COUNTS_PATH, GATE_COUNTS_SCHEMA,
    GateCountsExpectation, PlanInventory,
};
pub use row::{
    BaseDocument, BaseKind, BaseRowError, Claim, GATES_SCHEMA, GateBase, KILL_FORBIDDEN_FIELDS,
    KindCounts, MeasuredFields, PartBase, Recorded, Status, parse_milestone_ord,
};

#[cfg(test)]
#[path = "base_tests.rs"]
mod base;
