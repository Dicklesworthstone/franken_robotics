#![deny(unsafe_code)]

//! Spark shared-dialect preludes v0, kernel manifest `frob.kernel/1` (`N21b-01`),
//! kernel skeletons, and CPU/device kernel library home (`frob-kernels`, `BD-013`,
//! `N21b-01`, `BD-014`).
//!
//! Determinism class `D0` for dialect assembly, lint validation, and manifest
//! identity/validation; zero `unsafe`; prints nothing to stdout/stderr (L-7).

pub mod backend;
pub mod cpu;
pub mod dialect;
pub mod harness;
pub mod manifest;
pub mod receipt;
pub mod selftest;
