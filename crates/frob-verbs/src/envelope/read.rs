//! The envelope reader (N32a-03): strict JSON admission, schema validation, then typed
//! checks the schema cannot express.
//!
//! Typed checks, in order: the authorization flags through BD-005's parsers (a flag that is
//! missing or not `false` is `FROB-REF-816`); the `exit` of a refusal against the `exit` of
//! its registry row (whole-token code comparison; `FROB-REF-990` checked against the
//! `[exit_rule]` while N32-REF has not entered its row); `job.progress.done <= total`;
//! `job.eta_s.lo <= point <= hi`; and `evidence.stack_receipt` parsed as a `StackReceiptHash`.

use std::fmt;

use frob_core::auth::{NoAuthorization, PacketAuthorization};
use frob_core::refusal::{Refusal, RefusalFingerprint};
use frob_core::registry::refusals::RefusalRegistry;
use frob_core::stack::StackReceiptHash;

use super::exit::{ExitCode, ExitLookupError, exit_for_code, rule_exit_for_code};
use super::model::embedded_registry;
use super::schema::base_schema;
use super::tree::Tree;
use super::validate::{Schema, Violation};
use super::{EnvelopeError, INTERNAL_FAULT_CODE};

/// A validated envelope.
#[derive(Debug, Clone, PartialEq)]
pub struct ReadEnvelope {
    tree: Tree,
    verb: String,
    exit: ExitCode,
    refusal_code: Option<String>,
    refusal_fingerprint: Option<RefusalFingerprint>,
    stack_receipt: StackReceiptHash,
    stack_dev: bool,
}

impl ReadEnvelope {
    #[must_use]
    pub const fn tree(&self) -> &Tree {
        &self.tree
    }

    #[must_use]
    pub fn verb(&self) -> &str {
        &self.verb
    }

    #[must_use]
    pub const fn exit(&self) -> ExitCode {
        self.exit
    }

    #[must_use]
    pub fn refusal_code(&self) -> Option<&str> {
        self.refusal_code.as_deref()
    }

    #[must_use]
    pub const fn refusal_fingerprint(&self) -> Option<&RefusalFingerprint> {
        self.refusal_fingerprint.as_ref()
    }

    #[must_use]
    pub const fn stack_receipt(&self) -> StackReceiptHash {
        self.stack_receipt
    }

    #[must_use]
    pub const fn stack_dev(&self) -> bool {
        self.stack_dev
    }
}

/// Why bytes are not a valid envelope.
#[derive(Debug, Clone, PartialEq)]
pub enum ReadError {
    /// Not strict JSON (duplicate members, bad UTF-8, trailing bytes, ...).
    NotJson { pointer: String, message: String },
    /// Fails the schema; the first violation is the first differing field.
    Schema(Vec<Violation>),
    /// The authorization flags are missing or not `false` (BD-005's parser refused).
    Authorization(Box<Refusal>),
    /// The refusal's `exit` differs from the exit its registry row carries.
    ExitMismatch {
        code: String,
        observed: u8,
        expected: u8,
    },
    /// The refusal code has no exit (unregistered or malformed).
    Code(ExitLookupError),
    /// A job field failed typed decoding or an interval check.
    InvalidJobField {
        field: &'static str,
        reason: &'static str,
    },
    /// `job.progress.done` exceeds `total`.
    ProgressOverrun { done: i64, total: i64 },
    /// `evidence.stack_receipt` does not parse.
    StackReceipt(String),
    /// The schema or the registry could not be loaded.
    Setup(EnvelopeError),
}

impl ReadError {
    /// JSON pointer of the first differing field, for logs.
    #[must_use]
    pub fn first_field(&self) -> String {
        match self {
            Self::NotJson { pointer, .. } => pointer.clone(),
            Self::Schema(v) => v.first().map(|x| x.instance.clone()).unwrap_or_default(),
            Self::Authorization(r) => format!("/authorization{}", r.pointer),
            Self::ExitMismatch { .. } => "/exit".to_owned(),
            Self::Code(_) => "/refusal/code".to_owned(),
            Self::InvalidJobField { field, .. } => (*field).to_owned(),
            Self::ProgressOverrun { .. } => "/job/progress/done".to_owned(),
            Self::StackReceipt(_) => "/evidence/stack_receipt".to_owned(),
            Self::Setup(_) => String::new(),
        }
    }
}

impl fmt::Display for ReadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotJson { pointer, message } => {
                write!(f, "not strict JSON at `{pointer}`: {message}")
            }
            Self::Schema(v) => match v.first() {
                Some(first) => write!(f, "schema: {first} ({} violation(s))", v.len()),
                None => f.write_str("schema: invalid"),
            },
            Self::Authorization(r) => write!(f, "{}: {} at `{}`", r.code, r.message, r.pointer),
            Self::ExitMismatch {
                code,
                observed,
                expected,
            } => {
                write!(
                    f,
                    "`{code}` carries exit {observed}; its registry row says {expected}"
                )
            }
            Self::Code(e) => write!(f, "{e}"),
            Self::InvalidJobField { field, reason } => write!(f, "`{field}`: {reason}"),
            Self::ProgressOverrun { done, total } => {
                write!(f, "progress done {done} exceeds total {total}")
            }
            Self::StackReceipt(e) => write!(f, "evidence.stack_receipt: {e}"),
            Self::Setup(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for ReadError {}

/// Read an envelope against the base schema and the embedded refusal registry.
pub fn read_envelope(bytes: &[u8]) -> Result<ReadEnvelope, ReadError> {
    let schema = base_schema().map_err(ReadError::Setup)?;
    let registry = embedded_registry().map_err(ReadError::Setup)?;
    read_envelope_with(bytes, schema, registry)
}

/// Read an envelope against a given (for example verb-specialized) schema and registry.
pub fn read_envelope_with(
    bytes: &[u8],
    schema: &Schema,
    registry: &RefusalRegistry,
) -> Result<ReadEnvelope, ReadError> {
    let tree = Tree::parse(bytes).map_err(|e| ReadError::NotJson {
        pointer: e.pointer.clone(),
        message: e.message.clone(),
    })?;

    if let Some(auth) = tree.get("authorization") {
        let text = auth.to_canonical().map_err(|e| ReadError::NotJson {
            pointer: "/authorization".into(),
            message: e.to_string(),
        })?;
        let packet = auth.get("fabrication_released").is_some();
        let parsed = if packet {
            PacketAuthorization::parse_json(text.as_str()).map(|_| ())
        } else {
            NoAuthorization::parse_json(text.as_str()).map(|_| ())
        };
        parsed.map_err(|r| ReadError::Authorization(Box::new(r)))?;
    }

    schema.validate(&tree).map_err(ReadError::Schema)?;

    let exit = tree
        .get("exit")
        .and_then(Tree::as_int53)
        .and_then(ExitCode::from_wire)
        .ok_or_else(|| {
            ReadError::Schema(vec![Violation {
                instance: "/exit".into(),
                keyword: "/properties/exit".into(),
                message: "not an exit of the table".into(),
            }])
        })?;
    let verb = tree
        .get("verb")
        .and_then(Tree::as_str)
        .unwrap_or_default()
        .to_owned();

    let refusal_code = match tree
        .get("refusal")
        .and_then(|r| r.get("code"))
        .and_then(Tree::as_str)
    {
        Some(code) => {
            let expected = match exit_for_code(registry, code) {
                Ok(e) => e,
                Err(ExitLookupError::Unregistered(_)) if code == INTERNAL_FAULT_CODE => {
                    rule_exit_for_code(registry, code).map_err(ReadError::Code)?
                }
                Err(e) => return Err(ReadError::Code(e)),
            };
            if expected != exit {
                return Err(ReadError::ExitMismatch {
                    code: code.to_owned(),
                    observed: exit.code(),
                    expected: expected.code(),
                });
            }
            Some(code.to_owned())
        }
        None => None,
    };

    if let Some(job) = tree.get("job") {
        if let Some(progress) = job.get("progress") {
            let count = |key, field| {
                progress
                    .get(key)
                    .and_then(Tree::as_int53)
                    .ok_or(ReadError::InvalidJobField {
                        field,
                        reason: "requires an int53",
                    })
            };
            let done = count("done", "/job/progress/done")?;
            let total = count("total", "/job/progress/total")?;
            if done > total {
                return Err(ReadError::ProgressOverrun { done, total });
            }
        }
        if let Some(eta) = job.get("eta_s") {
            let seconds = |key, field| {
                eta.get(key)
                    .and_then(Tree::as_f64)
                    .ok_or(ReadError::InvalidJobField {
                        field,
                        reason: "requires finite seconds",
                    })
            };
            let lo = seconds("lo", "/job/eta_s/lo")?;
            let point = seconds("point", "/job/eta_s/point")?;
            let hi = seconds("hi", "/job/eta_s/hi")?;
            if !(lo <= point && point <= hi) {
                return Err(ReadError::InvalidJobField {
                    field: "/job/eta_s",
                    reason: "requires lo <= point <= hi",
                });
            }
        }
    }

    let evidence = tree.get("evidence");
    let receipt_text = evidence
        .and_then(|e| e.get("stack_receipt"))
        .and_then(Tree::as_str)
        .unwrap_or_default();
    let stack_receipt = StackReceiptHash::parse(receipt_text)
        .map_err(|e| ReadError::StackReceipt(format!("{e:?}")))?;
    let stack_dev = evidence
        .and_then(|e| e.get("stack_dev"))
        .and_then(Tree::as_bool)
        .unwrap_or(false);

    let refusal_fingerprint = tree
        .get("refusal")
        .and_then(|r| r.get("fingerprint"))
        .and_then(|fp| {
            let device = fp.get("device").and_then(Tree::as_str)?.to_owned();
            let digest = match fp.get("digest")? {
                Tree::Str(s) => Some(s.clone()),
                Tree::Null => None,
                _ => return None,
            };
            Some(RefusalFingerprint { device, digest })
        });

    Ok(ReadEnvelope {
        tree,
        verb,
        exit,
        refusal_code,
        refusal_fingerprint,
        stack_receipt,
        stack_dev,
    })
}
