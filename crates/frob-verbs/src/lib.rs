//! Verb registry, build-time static table, lints, and surface-shared definitions for
//! FrankenRobotics (layer 6, rank 4, tier Solid, cap 22,000 lines; beads N32a-02,
//! N32a-03, and N32a-04). The surface-shared definitions include [`envelope`]
//! (`frob.envelope/1`, the exit-code table, and the progress-then-one-envelope output
//! rule) and [`dispatch`] (the single `Dispatch` service, seven-step pipeline, `InProcess`
//! and `Socket` back ends, control-plane granularity lint, and [`rpc`] `frob.rpc/1`
//! canonical NDJSON transport over a mode-`0600` Unix domain socket).
//!
//! Compiled from `registries/verbs.toml` (schema `frob.verbs/1`) by `build.rs` into
//! `$OUT_DIR/verbs_table.rs` (ADR-TETHER-01). Under `--no-default-features --features
//! registry-only` (ADR-TETHER-02), compiles only the registry, schemas, name map,
//! envelope, `Socket` client, and lint/codegen types with zero physics or subsystem
//! handler dependencies.
//!
//! Determinism class D0; prints nothing (L-7); zero `unsafe`.

#![deny(unsafe_code)]

#[cfg(all(feature = "conformance", not(debug_assertions)))]
compile_error!("frob-verbs feature `conformance` is forbidden in release builds");

#[cfg(all(
    any(feature = "nc-bare-unknown-verb", feature = "nc-second-envelope"),
    not(debug_assertions)
))]
compile_error!("frob-verbs planted-defect features `nc-*` are forbidden in release builds");

pub mod dispatch;
pub mod envelope;
pub mod registry;

pub use dispatch::rpc;

#[cfg(feature = "handlers")]
pub mod handlers;
