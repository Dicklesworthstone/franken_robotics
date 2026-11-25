//! The output guard of INV-TETHER-02 (N32a-03): with `--json`, every invocation writes
//! zero or more `frob.progress/1` lines and then exactly one envelope, always the last line.
//!
//! [`serve`] runs one handler call under `catch_unwind`: a panic, an error that maps to no
//! refusal code, or an envelope answering another verb becomes the `FROB-REF-990` envelope
//! (exit 9); an unknown verb becomes `FROB-REF-801` (exit 3 once its registry row exists).
//! The writer is the caller's; nothing here touches the process's standard output (L-7).
//! [`check_stream`] checks captured output against the rule.

use std::fmt;
use std::io::{self, Write};
use std::panic::{self, AssertUnwindSafe};
use std::time::Instant;

use frob_core::canon::{self, Object, Value};
use frob_core::refusal::{Repair, RetryPolicy};
use frob_core::registry::refusals::RefusalRegistry;

use crate::registry::{VerbEntry, lookup_in};

use super::blocks::{Cost, Progress, ProgressReport, RefusalFields, job_state};
use super::exit::ExitCode;
use super::model::{Envelope, FaultContext, embedded_registry};
use super::read::{ReadEnvelope, ReadError, read_envelope_with};
use super::schema::{base_schema, progress_schema};
use super::tree::Tree;
use super::{EnvelopeError, PROGRESS_SCHEMA_ID};

/// The code an unknown verb is refused with (§32.18, entered by N32-REF).
pub const UNKNOWN_VERB_CODE: &str = "FROB-REF-801";

/// One handler call as the guard sees it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Call<'a> {
    /// The verb named by the request (registry name, `design.check`).
    pub verb: &'a str,
    /// The call's idempotency key, when it carried one.
    pub idempotency_key: Option<&'a str>,
    /// Where a fault's diagnostic is written (the 990 envelope's `observed`).
    pub diagnostic_path: &'a str,
}

/// A handler's failure other than a panic.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HandlerFault {
    /// Refused with a registered code; the guard reads the exit from the registry.
    Refused { code: String, fields: RefusalFields },
    /// An error that maps to no refusal code (becomes `FROB-REF-990`).
    Unmapped(String),
}

/// Where a handler writes its progress events, ahead of the envelope.
pub struct ProgressSink<'w> {
    out: &'w mut dyn Write,
    job: Option<String>,
    lines: usize,
    io_error: Option<io::Error>,
}

impl<'w> ProgressSink<'w> {
    fn new(out: &'w mut dyn Write) -> Self {
        Self {
            out,
            job: None,
            lines: 0,
            io_error: None,
        }
    }

    /// Write one `frob.progress/1` line. A report without a denominator, a `done` above its
    /// `total`, or a second job id in one stream is refused and nothing is written.
    pub fn emit(
        &mut self,
        job: &str,
        state: &str,
        report: &ProgressReport,
    ) -> Result<(), EnvelopeError> {
        let progress = Progress::from_report(report)?;
        if job.is_empty() || job.chars().any(char::is_control) {
            return Err(EnvelopeError::InvalidField {
                field: "job",
                reason: "empty or has control characters".into(),
            });
        }
        job_state("state", state)?;
        if let Some(j) = &self.job
            && j != job
        {
            return Err(EnvelopeError::InvalidField {
                field: "job",
                reason: format!("stream already reports `{j}`"),
            });
        }
        let line = canon::to_canonical(&ProgressLine {
            job,
            state,
            progress: &progress,
        })
        .map_err(EnvelopeError::Canon)?;
        if self.io_error.is_none() {
            let written = self
                .out
                .write_all(line.as_bytes())
                .and_then(|()| self.out.write_all(b"\n"));
            if let Err(e) = written {
                self.io_error = Some(e);
            }
        }
        self.job = Some(job.to_owned());
        self.lines += 1;
        Ok(())
    }

    /// Progress lines written so far.
    #[must_use]
    pub const fn lines(&self) -> usize {
        self.lines
    }
}

struct ProgressLine<'a> {
    job: &'a str,
    state: &'a str,
    progress: &'a Progress,
}

impl canon::Canonical for ProgressLine<'_> {
    fn write(&self, w: Value<'_>) -> Result<(), canon::CanonError> {
        w.object(|o: &mut Object<'_>| {
            self.progress.write(o)?;
            o.str("job", self.job)?;
            o.str("schema", PROGRESS_SCHEMA_ID)?;
            o.str("state", self.state)
        })
    }
}

fn elapsed_cost(started: Instant) -> Cost {
    let wall_ms = u64::try_from(started.elapsed().as_millis())
        .unwrap_or(u64::MAX)
        .min((1 << 53) - 1);
    Cost {
        wall_ms,
        ..Cost::ZERO
    }
}

fn sanitize(verb: &str) -> String {
    verb.chars()
        .map(|c| if c.is_control() { '\u{fffd}' } else { c })
        .collect()
}

fn fault(call: &Call<'_>, cost: Cost) -> Envelope {
    Envelope::internal_fault(
        &FaultContext {
            verb: call.verb.to_owned(),
            idempotency_key: call.idempotency_key.map(str::to_owned),
            diagnostic_path: call.diagnostic_path.to_owned(),
        },
        cost,
    )
}

fn unknown_verb(registry: &RefusalRegistry, call: &Call<'_>, cost: Cost) -> Envelope {
    let shown = sanitize(call.verb);
    let fields = RefusalFields {
        message: format!("unknown verb `{shown}`"),
        pointer: "/verb".to_owned(),
        observed: shown.clone(),
        required: "a verb of the registry".to_owned(),
        unit: String::new(),
        detail: None,
        affected_claims: Vec::new(),
        retry: RetryPolicy::AfterChange,
        repairs: vec![Repair::with_detail(
            "frob robot-docs",
            "Find the registered verbs in the command reference.",
        )],
    };
    // Until N32-REF enters 801's row the registry cannot give it an exit: that is a fault of
    // the binary's registry, reported as one envelope with exit 9, never as bare text.
    Envelope::from_code_in(registry, &shown, UNKNOWN_VERB_CODE, fields, cost)
        .unwrap_or_else(|_| fault(call, cost))
}

/// Run one handler call and write its output: the handler's progress lines, then exactly
/// one envelope. Returns the envelope's exit (the process exit code on the CLI). An `Err`
/// is a failure of the writer itself.
pub fn serve<W, F>(
    out: &mut W,
    registry: &RefusalRegistry,
    verbs: &[VerbEntry],
    call: &Call<'_>,
    handler: F,
) -> io::Result<ExitCode>
where
    W: Write,
    F: FnOnce(&mut ProgressSink<'_>) -> Result<Envelope, HandlerFault>,
{
    let started = Instant::now();
    if lookup_in(verbs, call.verb).is_none() {
        #[cfg(feature = "nc-bare-unknown-verb")]
        {
            // Planted defect (negative control for GATE-TETHER-03): bare text, no envelope.
            out.write_all(format!("error: unknown verb `{}`\n", sanitize(call.verb)).as_bytes())?;
            return Ok(ExitCode::Refused);
        }
        #[cfg(not(feature = "nc-bare-unknown-verb"))]
        {
            let env = unknown_verb(registry, call, elapsed_cost(started));
            return emit(out, &env, call);
        }
    }
    let mut sink = ProgressSink::new(&mut *out);
    let outcome = panic::catch_unwind(AssertUnwindSafe(|| handler(&mut sink)));
    if let Some(e) = sink.io_error.take() {
        return Err(e);
    }
    let cost = elapsed_cost(started);
    let env = match outcome {
        Ok(Ok(env)) if env.verb() == call.verb => env,
        Ok(Ok(_)) | Ok(Err(HandlerFault::Unmapped(_))) | Err(_) => fault(call, cost),
        Ok(Err(HandlerFault::Refused { code, fields })) => {
            Envelope::from_code_in(registry, call.verb, &code, fields, cost)
                .unwrap_or_else(|_| fault(call, cost))
        }
    };
    emit(out, &env, call)
}

fn emit<W: Write>(out: &mut W, env: &Envelope, call: &Call<'_>) -> io::Result<ExitCode> {
    let (bytes, exit) = match env.to_canonical() {
        Ok(b) => (b, env.exit()),
        Err(_) => {
            let f = fault(call, Cost::ZERO);
            let b = f
                .to_canonical()
                .map_err(|e| io::Error::other(e.to_string()))?;
            (b, f.exit())
        }
    };
    out.write_all(bytes.as_bytes())?;
    out.write_all(b"\n")?;
    #[cfg(feature = "nc-second-envelope")]
    {
        // Planted defect (negative control for GATE-TETHER-03): a second envelope.
        out.write_all(bytes.as_bytes())?;
        out.write_all(b"\n")?;
    }
    out.flush()?;
    Ok(exit)
}

/// What a well-formed standard output contained.
#[derive(Debug, Clone, PartialEq)]
pub struct StreamSummary {
    pub progress_lines: usize,
    pub envelope: ReadEnvelope,
}

/// How a standard output breaks the one-envelope rule.
#[derive(Debug, Clone, PartialEq)]
pub enum StreamError {
    /// Nothing at all was written.
    Empty,
    /// The last line is not terminated by `\n`.
    Unterminated,
    /// A line is neither a valid progress event nor a valid envelope.
    BadLine { line: usize, reason: String },
    /// A progress event after the envelope.
    ProgressAfterEnvelope { line: usize },
    /// A second envelope.
    SecondEnvelope { line: usize },
    /// Progress lines and no envelope.
    NoEnvelope,
    /// The schema or registry could not be loaded.
    Setup(EnvelopeError),
}

impl fmt::Display for StreamError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => f.write_str("no output"),
            Self::Unterminated => f.write_str("last line not newline-terminated"),
            Self::BadLine { line, reason } => write!(f, "line {line}: {reason}"),
            Self::ProgressAfterEnvelope { line } => {
                write!(f, "line {line}: progress after the envelope")
            }
            Self::SecondEnvelope { line } => write!(f, "line {line}: second envelope"),
            Self::NoEnvelope => f.write_str("no envelope"),
            Self::Setup(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for StreamError {}

/// Check captured standard output against the embedded registry.
pub fn check_stream(bytes: &[u8]) -> Result<StreamSummary, StreamError> {
    let registry = embedded_registry().map_err(StreamError::Setup)?;
    check_stream_with(bytes, registry)
}

/// Check captured standard output: zero or more `frob.progress/1` lines of one job, then
/// exactly one valid envelope as the last line.
pub fn check_stream_with(
    bytes: &[u8],
    registry: &RefusalRegistry,
) -> Result<StreamSummary, StreamError> {
    if bytes.is_empty() {
        return Err(StreamError::Empty);
    }
    let Some(body) = bytes.strip_suffix(b"\n") else {
        return Err(StreamError::Unterminated);
    };
    let envelope_schema = base_schema().map_err(StreamError::Setup)?;
    let progress = progress_schema().map_err(StreamError::Setup)?;
    let mut progress_lines = 0usize;
    let mut job: Option<String> = None;
    let mut envelope: Option<ReadEnvelope> = None;
    for (i, raw) in body.split(|b| *b == b'\n').enumerate() {
        let line = i + 1;
        let tree = Tree::parse(raw).map_err(|e| StreamError::BadLine {
            line,
            reason: format!("not JSON: {}", e.message),
        })?;
        match tree.get("schema").and_then(Tree::as_str) {
            Some(PROGRESS_SCHEMA_ID) => {
                if envelope.is_some() {
                    return Err(StreamError::ProgressAfterEnvelope { line });
                }
                progress.validate(&tree).map_err(|v| StreamError::BadLine {
                    line,
                    reason: v.first().map_or_else(String::new, ToString::to_string),
                })?;
                let count = |field| {
                    tree.get(field)
                        .and_then(Tree::as_int53)
                        .ok_or_else(|| StreamError::BadLine {
                            line,
                            reason: format!("`/{field}` requires an int53"),
                        })
                };
                let done = count("done")?;
                let total = count("total")?;
                if done > total {
                    return Err(StreamError::BadLine {
                        line,
                        reason: format!("done {done} exceeds total {total}"),
                    });
                }
                let id = tree.get("job").and_then(Tree::as_str).unwrap_or_default();
                if job.as_deref().is_some_and(|j| j != id) {
                    return Err(StreamError::BadLine {
                        line,
                        reason: "a second job id".into(),
                    });
                }
                job = Some(id.to_owned());
                progress_lines += 1;
            }
            Some(super::ENVELOPE_SCHEMA_ID) => {
                if envelope.is_some() {
                    return Err(StreamError::SecondEnvelope { line });
                }
                let read = read_envelope_with(raw, envelope_schema, registry).map_err(
                    |e: ReadError| StreamError::BadLine {
                        line,
                        reason: e.to_string(),
                    },
                )?;
                envelope = Some(read);
            }
            _ => {
                return Err(StreamError::BadLine {
                    line,
                    reason: "neither a progress event nor an envelope".into(),
                });
            }
        }
    }
    envelope
        .map(|envelope| StreamSummary {
            progress_lines,
            envelope,
        })
        .ok_or(StreamError::NoEnvelope)
}
