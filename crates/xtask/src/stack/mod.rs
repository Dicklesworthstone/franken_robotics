//! FrankenSuite stack-consumption vocabulary, provenance-lift validation (`N33a-01`),
//! strict `franken-stack.lock` (`frob.stack-lock/1`) parser (`N33a-03`),
//! resolution receipt (`frob.stack-receipt/1`) writer (`N33a-04`), and
//! `check-stack` verification plus dirty-tree PASS guard (`N33a-05`).
//!
//! Holds the closed posture enumeration, foundation-use vocabulary, and
//! posture-to-graph edge rules ([`posture`]), the provenance-lift registry
//! parser and validator ([`lifts`]), the strict `franken-stack.lock` typed
//! model, validator, canonical serializer, stack-lock digest, and JSON Schema
//! generator ([`lock`]), the resolution receipt writer, `frob.tree/1` tree
//! hash, drift classifier, and content-addressed store ([`receipt`]), the
//! dirty-tree PASS guard ([`guard`]), and the `check-stack` three-way
//! verification and `--write-receipt` engine ([`verify`]). All modules are pure
//! functions of their inputs in verify mode (determinism class D0), read TOML
//! strictly through `frob_core::registry::toml`, serialize canonical JSON
//! strictly through `frob_core::canon`, print nothing (rule L-7), use zero
//! `unsafe` code, and never write or delete sibling files.

pub mod guard;
pub mod lifts;
pub mod lock;
pub mod posture;
pub mod receipt;
pub mod verify;
