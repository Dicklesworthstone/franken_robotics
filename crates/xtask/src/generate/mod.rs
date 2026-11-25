//! Deterministic in-process document and schema generators for `xtask` (`N33a-07`, `N34p4-03`).
//!
//! Pure functions from registry and receipt files to repository artifacts
//! (`check-generated`, rule L-8). Determinism class D0, zero `unsafe`, prints
//! nothing (rule L-7).

pub mod admission_doc;
