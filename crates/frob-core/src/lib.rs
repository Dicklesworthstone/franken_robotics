//! Core foundational library for FrankenRobotics (layer 0, rank 1, determinism D0).
//!
//! Owns the workspace-wide vocabulary: content-addressed identities and BLAKE3 hashing
//! (`id`), artifact-graph identifiers in the suite's text form (`graph_id`), canonical JSON
//! serialization from typed terms (`canon`), evidence classes and the weakest-input-wins
//! claim product (`evidence`), constant-`false` no-authorization flags (`auth`), raw-byte
//! JSON admission (`json`), plain mission proof objects (`proof`), structured refusals and
//! the generated `RefusalCode` (`refusal`), strict registry readers (`registry`), closed SI
//! quantities (`qty`), reference workload records and row-capacity rule (`perf`),
//! build-embedded stack identities and the same-stack rule (`stack`), and the opt-in
//! test-support kit (`testkit` behind feature `test-support`).
//! Prints nothing (rule L-7), admits no `serde` or async runtime (rule L-5), and contains
//! zero `unsafe` code.

#![deny(unsafe_code)]

pub mod auth;
pub mod canon;
pub mod evidence;
pub mod graph_id;
pub mod id;
pub mod json;
pub mod perf;
pub mod proof;
pub mod qty;
pub mod refusal;
pub mod registry;
pub mod stack;

#[cfg(feature = "test-support")]
pub mod testkit;
