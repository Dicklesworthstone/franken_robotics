//! Workspace verification library for FrankenRobotics (`xtask`).
//!
//! Holds the compiled workspace tables and rule validators consumed by the
//! six `xtask` subcommands (`check-layers`, `check-deps`, `check-unsafe`,
//! `check-generated`, `check-contracts`, `check-stack`).

pub mod check_deps;
pub mod check_layers;
pub mod cli;
pub mod crate_map;
pub mod inversions;
pub mod metadata;
pub mod report;
pub mod stack;
pub mod status_table;
