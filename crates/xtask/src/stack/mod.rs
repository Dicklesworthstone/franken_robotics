//! FrankenSuite stack-consumption vocabulary, provenance-lift validation (`N33a-01`),
//! and strict `franken-stack.lock` (`frob.stack-lock/1`) parser (`N33a-03`).
//!
//! Holds the closed posture enumeration, foundation-use vocabulary, and
//! posture-to-graph edge rules ([`posture`]), the provenance-lift registry
//! parser and validator ([`lifts`]), and the strict `franken-stack.lock` typed
//! model, validator, canonical serializer, stack-lock digest, and JSON Schema
//! generator ([`lock`]). All modules are pure functions of their inputs
//! (determinism class D0), read TOML strictly through
//! `frob_core::registry::toml` (no second TOML parser), print nothing (rule
//! L-7), use zero `unsafe` code, and never write or delete repository files.

pub mod lifts;
pub mod lock;
pub mod posture;
