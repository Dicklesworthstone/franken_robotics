//! Gauntlet verification crate for FrankenRobotics (`BD-025`, `STD-E2E`).
//!
//! - `compendium`: exact mathematical oracle library, registry validator, and doc renderer (`N12-41`).
//! - `e2e`: `std`-only end-to-end log tooling (`STD-E2E`).
//! - `gates_v0`: v0 reader for `registries/gates.toml` (`frob.gates/1`, `BD-025`).
//! - `controls`: negative-control registry validator, serializer, and `GATE-GAUNT-01`
//!   registry check (`BD-025`).

pub mod compendium;
pub mod controls;
pub mod e2e;
pub mod gates_v0;
