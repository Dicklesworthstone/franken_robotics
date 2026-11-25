//! FrankenSuite stack-consumption vocabulary, provenance-lift validation (`N33a-01`),
//! strict `franken-stack.lock` (`frob.stack-lock/1`) parser (`N33a-03`), and
//! resolution receipt (`frob.stack-receipt/1`) writer (`N33a-04`).
//!
//! Holds the closed posture enumeration, foundation-use vocabulary, and
//! posture-to-graph edge rules ([`posture`]), the provenance-lift registry
//! parser and validator ([`lifts`]), the strict `franken-stack.lock` typed
//! model, validator, canonical serializer, stack-lock digest, and JSON Schema
//! generator ([`lock`]), and the resolution receipt writer, `frob.tree/1` tree
//! hash, drift classifier, and content-addressed store ([`receipt`]). All
//! modules are pure functions of their inputs (determinism class D0), read TOML
//! strictly through `frob_core::registry::toml` (no second TOML parser),
//! serialize canonical JSON strictly through `frob_core::canon` (no second JSON
//! serializer), print nothing (rule L-7), use zero `unsafe` code, and never
//! write or delete sibling files.

pub mod lifts;
pub mod lock;
pub mod posture;
pub mod receipt;
