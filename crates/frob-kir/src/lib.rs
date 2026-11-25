#![deny(unsafe_code)]

//! Spark kernel IR (`frob.kir/1`, `BD-018`) and shared-dialect static lint (`BD-013`).
//!
//! Determinism class `D0`; zero `unsafe`; prints nothing to stdout/stderr (L-7).

pub mod cay_step;
pub mod dialect_lint;
pub mod emit;
pub mod ir;

/// Version of the shared-dialect lint engine recorded in lint reports and caches.
pub const DIALECT_LINT_VERSION: u32 = 1;

/// Shared-dialect prelude version fixed by BD-013 (`FROB_PRELUDE_VERSION`).
pub const FROB_PRELUDE_VERSION: u32 = 0;

pub use ir::KIR_SCHEMA;
