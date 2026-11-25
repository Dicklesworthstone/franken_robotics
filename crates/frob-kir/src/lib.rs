#![deny(unsafe_code)]

//! Spark kernel IR home and shared-dialect static lint (`frob-kir`, BD-013).
//!
//! Determinism class `D0`; zero `unsafe`; prints nothing to stdout/stderr (L-7).
//! Downstream bead BD-018 adds the `frob.kir/1` straight-line program IR,
//! dimension checker (`FROB-REF-250`), bounded CSE, and Rust/dialect emitters.

pub mod dialect_lint;

/// Version of the shared-dialect lint engine recorded in lint reports and caches.
pub const DIALECT_LINT_VERSION: u32 = 1;

/// Shared-dialect prelude version fixed by BD-013 (`FROB_PRELUDE_VERSION`).
pub const FROB_PRELUDE_VERSION: u32 = 0;
