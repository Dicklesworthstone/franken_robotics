//! Subsystem verb handler bindings (enabled by default feature `handlers`, omitted under
//! `--no-default-features --features registry-only` per ADR-TETHER-02).
//!
//! M0 operator verb handlers (`doctor`, `capabilities`, `schema`, `robot-docs`) and the
//! dispatch service interface land in N32a-04 and BD-022.

use crate::{
    dispatch::HandlerTable,
    operator,
    registry::{VERBS, VerbEntry},
};

#[path = "verbs/sim_run.rs"]
mod sim_run;

/// Return the slice of verb entries present in the compiled static registry table.
///
/// Whether an individual verb has an active handler in this build is determined by the
/// dispatch table (N32a-04 / BD-022), never by `first_milestone`.
#[must_use]
pub fn registered_verbs() -> &'static [VerbEntry] {
    &VERBS
}

/// Bind the handlers compiled into this build for both dispatch surfaces.
#[must_use]
pub fn handler_table() -> HandlerTable {
    let mut table = HandlerTable::new();
    operator::register(&mut table);
    sim_run::register(&mut table);
    table
}
