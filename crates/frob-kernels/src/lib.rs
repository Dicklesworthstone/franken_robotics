#![deny(unsafe_code)]

//! Spark shared-dialect preludes v0, kernel skeletons, and kernel library home
//! (`frob-kernels`, BD-013).
//!
//! Determinism class `D0` for dialect assembly and lint validation; zero
//! `unsafe`; prints nothing to stdout/stderr (L-7). Downstream beads `N21b-01`
//! and `BD-014` add the `frob.kernel/1` manifest, `KernelBackend` trait, and
//! `batch-cpu` reference kernels.

pub mod dialect;
