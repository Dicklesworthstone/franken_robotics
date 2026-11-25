#![forbid(unsafe_code)]

//! Dev-only `harness` group of the FrankenSim airlock (`N33a-11`, Review Note 2).
//!
//! Exposes no production traits. Confines dev/oracle-only sibling crates
//! (`fs-roofline`, `fs-detaudit`, `fs-propcheck`, `fs-render`, oracle-only
//! `fs-la`, and test-only `fs-ir`) behind `frob-fsim` so test crates such as
//! `frob-conformance` never add direct volatile FrankenSim edges.

/// Canonical list of dev-only / oracle-only FrankenSim crates behind `harness`.
pub const HARNESS_CRATES: [&str; 6] = [
    "fs-roofline",
    "fs-detaudit",
    "fs-propcheck",
    "fs-render",
    "fs-la",
    "fs-ir",
];
