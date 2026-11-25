//! FrankenSuite stack-consumption vocabulary and provenance-lift validation (`N33a-01`).
//!
//! Holds the closed posture enumeration, foundation-use vocabulary, and
//! posture-to-graph edge rules ([`posture`]) together with the provenance-lift
//! registry parser and validator ([`lifts`]). Both modules are pure functions
//! of their inputs (determinism class D0), read TOML strictly through
//! `frob_core::registry::toml` (no second TOML parser), print nothing (rule
//! L-7), use zero `unsafe` code, and never write or delete repository files.

pub mod lifts;
pub mod posture;
