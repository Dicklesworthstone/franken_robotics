//! Verb registry, build-time static table, lints, and surface-shared definitions for
//! FrankenRobotics (layer 6, rank 4, tier Solid, cap 22,000 lines; bead N32a-02).
//!
//! Compiled from `registries/verbs.toml` (schema `frob.verbs/1`) by `build.rs` into
//! `$OUT_DIR/verbs_table.rs` (ADR-TETHER-01). Under `--no-default-features --features
//! registry-only` (ADR-TETHER-02), compiles only the registry, schemas, name map, and
//! lint/codegen types with zero physics or subsystem handler dependencies.
//!
//! Determinism class D0; prints nothing (L-7); zero `unsafe`.

#![deny(unsafe_code)]

#[cfg(all(feature = "conformance", not(debug_assertions)))]
compile_error!("frob-verbs feature `conformance` is forbidden in release builds");

pub mod registry;

#[cfg(feature = "handlers")]
pub mod handlers;
