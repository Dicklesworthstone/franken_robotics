//! `frob.envelope/1`: the one answer shape of every verb, its exit-code table, and the
//! progress stream that precedes a long job's envelope (bead N32a-03, §32.4).
//!
//! - [`Envelope`] is built only through constructors that fix the outcome and the exit:
//!   [`Envelope::completed`] (0), [`Envelope::completed_with_findings`] (2),
//!   [`Envelope::from_refusal`] (the exit of the code's registry row),
//!   [`Envelope::deferred`] (8) and [`Envelope::internal_fault`] (`FROB-REF-990`, 9).
//!   Its `evidence` comes from a [`frob_core::evidence::ClaimProduct`] through
//!   [`EvidenceBlock`]; every envelope carries `evidence.stack_receipt` (and
//!   `stack_dev: true` on a dev build) from [`frob_core::stack::BUILT_UNDER`].
//! - [`ExitCode`] has nine values; exit 1 is not representable.
//! - The schema is hand-written (`crates/frob-verbs/schemas/frob.envelope.1.json`) and
//!   specialized per verb by [`envelope_schema_for`]; [`Schema`] is a fail-closed draft
//!   2020-12 subset validator.
//! - [`read_envelope`] is the reader: strict JSON admission, schema validation, then typed
//!   checks (exit against the registry, authorization flags through BD-005's parser,
//!   `done <= total`, the stack receipt).
//! - [`serve`] is the output guard: zero or more `frob.progress/1` lines, then exactly one
//!   envelope, whatever the handler does (returns, refuses, panics); [`check_stream`]
//!   checks a captured standard output against that rule.
//!
//! Determinism class D0: equal envelopes are equal canonical bytes; `cost.wall_ms` is the
//! only wall-clock field. Nothing here prints (L-7): output goes to a caller's writer.

mod blocks;
mod evidence;
mod exit;
mod model;
mod output;
mod read;
mod regex;
mod schema;
mod tree;
mod validate;

#[cfg(test)]
mod tests;

use std::fmt;

use frob_core::canon::CanonError;

pub use blocks::{
    Authorization, Bound, Budget, Completion, Cost, Eta, FixtureBinding, JobHandle, JobReport,
    JsonBody, NextAction, Page, Progress, ProgressReport, RefusalBlock, RefusalFields, Warning,
};
pub use evidence::EvidenceBlock;
pub use exit::{
    ExitCode, ExitLookupError, FailureClass, exit_for_code, parse_refusal_code, rule_exit_for_code,
};
pub use model::{Envelope, FaultContext, Outcome, embedded_registry};
pub use output::{
    Call, HandlerFault, ProgressSink, StreamError, StreamSummary, UNKNOWN_VERB_CODE, check_stream,
    check_stream_with, serve,
};
pub use read::{ReadEnvelope, ReadError, read_envelope, read_envelope_with};
pub use schema::{
    base_schema, envelope_schema, envelope_schema_for, progress_schema, specialized_document,
};
pub use tree::Tree;
pub use validate::{Schema, SchemaError, Violation, escape_ptr};

/// Schema identifier carried in every envelope's `schema` member.
pub const ENVELOPE_SCHEMA_ID: &str = "frob.envelope/1";
/// Schema identifier carried in every progress line's `schema` member.
pub const PROGRESS_SCHEMA_ID: &str = "frob.progress/1";
/// `internal-fault`: a handler panicked or returned an error that maps to no code.
pub const INTERNAL_FAULT_CODE: &str = "FROB-REF-990";
/// `frob.envelope/1` JSON Schema text (hand-written).
pub const ENVELOPE_SCHEMA_JSON: &str = include_str!("../../schemas/frob.envelope.1.json");
/// `frob.progress/1` JSON Schema text (hand-written).
pub const PROGRESS_SCHEMA_JSON: &str = include_str!("../../schemas/frob.progress.1.json");

/// Why an envelope or one of its blocks could not be built.
#[derive(Debug, Clone, PartialEq)]
pub enum EnvelopeError {
    /// A field breaks its block's invariant.
    InvalidField { field: &'static str, reason: String },
    /// A progress block without a denominator (absent or zero `total`).
    ProgressWithoutDenominator,
    /// A builder that does not apply to this outcome.
    NotApplicable {
        what: &'static str,
        reason: &'static str,
    },
    /// The refusal code has no exit (malformed, unregistered, or no rule).
    ExitLookup(ExitLookupError),
    /// The embedded refusal registry did not load.
    Registry(String),
    /// The schema document did not compile or the verb's output schema is missing.
    Schema(String),
    /// The canonical writer refused a value.
    Canon(CanonError),
}

impl fmt::Display for EnvelopeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidField { field, reason } => write!(f, "envelope field `{field}`: {reason}"),
            Self::ProgressWithoutDenominator => {
                f.write_str("progress without a denominator (`total` absent or zero)")
            }
            Self::NotApplicable { what, reason } => write!(f, "`{what}` does not apply: {reason}"),
            Self::ExitLookup(e) => write!(f, "{e}"),
            Self::Registry(e) => write!(f, "refusal registry: {e}"),
            Self::Schema(e) => write!(f, "envelope schema: {e}"),
            Self::Canon(e) => write!(f, "canonical writer: {e}"),
        }
    }
}

impl std::error::Error for EnvelopeError {}
