//! Workspace verification library for FrankenRobotics (`xtask`).
//!
//! Holds the compiled workspace tables and rule validators consumed by the
//! six `xtask` subcommands (`check-layers`, `check-deps`, `check-unsafe`,
//! `check-generated`, `check-contracts`, `check-stack`).

pub mod assembly;
pub mod check_contracts;
pub mod check_deps;
pub mod check_deps_airlock;
pub mod check_generated;
pub mod check_layers;
pub mod check_unsafe;
pub mod cli;
pub mod crate_map;
pub mod e2e_runner;
pub mod generate;
pub mod inversions;
pub mod metadata;
pub mod report;
pub mod stack;
pub mod status_table;
pub mod workspaces;

// Share pure source with frob-conformance without linking the crate.
#[path = "../../frob-conformance/src/register/print.rs"]
mod conformance_register_print;
#[path = "../../frob-conformance/src/e2e/json.rs"]
pub mod e2e_json;
