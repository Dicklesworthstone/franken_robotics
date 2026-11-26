//! Shared registry parser, linter, and code-generator modules.
//!
//! Included both from `crates/frob-verbs/src/registry/mod.rs` and from
//! `crates/frob-verbs/build.rs` via `#[path = "src/registry/engine.rs"]` so the
//! build script and library execute identical parsing, linting, and codegen logic.

mod arguments;
pub mod codegen;
pub mod derive;
pub mod entry;
pub mod lint;
pub mod names;
pub mod schema_id;
