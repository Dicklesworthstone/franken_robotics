//! Ten-fold comparative benchmark harness skeleton for FrankenRobotics
//! (bead `N39a-01`, §39 preamble, §39.1, BET-13, BET-14).
//!
//! Measures FrankenRobotics through public `frob` verbs and drives rival tools
//! out of process; exposes no product verb and no MCP surface (`ADR-GAUNT-01`,
//! `ADR-GAUNT-04`). Modules under the scorer boundary (`ids`, `refusal`,
//! `canon`, `records/*`, `score/*`) depend strictly on `std` and `frob_cert`
//! and build standalone in `kit/scorer/`.

pub mod accept;
pub mod arms;
pub mod canon;
pub mod clock;
pub mod cost;
pub mod ids;
pub mod phase;
pub mod preopen;
pub mod records;
pub mod refusal;
pub mod roles;
pub mod score;
pub mod seal;
pub mod store;
pub mod suite;
