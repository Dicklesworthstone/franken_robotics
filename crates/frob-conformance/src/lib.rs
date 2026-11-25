//! Gauntlet verification crate for FrankenRobotics (`BD-025`, `STD-E2E`).
//!
//! - `e2e`: `std`-only end-to-end log tooling (`STD-E2E`).
//! - `gates_v0`: v0 reader for `registries/gates.toml` (`frob.gates/1`, `BD-025`).
//! - `controls`: negative-control registry validator, serializer, and `GATE-GAUNT-01`
//!   registry check (`BD-025`).

pub mod controls;
pub mod e2e;
pub mod gates_v0;
