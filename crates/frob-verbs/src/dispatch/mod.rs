//! Surface-shared verb dispatch interface: one service, two back ends (`InProcess` and
//! `Socket`), the 7-step request pipeline, `frob.rpc/1` wire framing, and the
//! control-plane granularity lint (bead N32a-04, ADR-TETHER-03, ADR-TETHER-11).
//!
//! Feature boundary (ADR-TETHER-02):
//! - Under `--no-default-features --features registry-only`, this module exposes
//!   [`Dispatch`], [`Socket`], [`DaemonClient`], [`rpc`], [`pipeline`], and [`granularity`]
//!   with zero physics, subsystem, or async-runtime dependencies, and does **not** expose
//!   [`in_process::InProcess`].
//! - Under the default `handlers` feature, this module additionally exposes
//!   [`in_process::InProcess`], [`in_process::HandlerTable`], and [`in_process::VerbHandler`].
//!
//! Determinism class D0 apart from `cost.wall_ms` and the `executor` warning; prints
//! nothing in library code (L-7); zero `unsafe`.

pub mod fixture;
pub mod granularity;
#[cfg(feature = "handlers")]
pub mod in_process;
pub mod pipeline;
pub mod rpc;
pub mod socket;

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fmt;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;

use frob_core::canon::{CanonError, Canonical, CanonicalBytes, Object, Value};
use frob_core::id::Blake3;
use frob_core::registry::refusals::RefusalRegistry;

use crate::envelope::{
    Bound, Envelope, EnvelopeError, ExitCode, FixtureBinding, JobHandle, Progress, ReadEnvelope,
    ReadError, Tree, base_schema, escape_ptr, read_envelope_with,
};
use crate::registry::Kind;

pub use granularity::{GranularityError, check_control_plane_granularity};
#[cfg(feature = "handlers")]
pub use in_process::{HandlerContext, HandlerTable, InProcess, ProgressEmit, VerbHandler};
pub use pipeline::{
    FixtureResolver, InputSchema, NameResolver, NoopFixtureResolver, NoopNameResolver, Pipeline,
    PipelineError,
};
pub use socket::{DaemonClient, Socket, SocketConfig, SocketError};

/// Cancellation signal observed by dispatch and handlers at checkpoints.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cancelled;

impl fmt::Display for Cancelled {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("dispatch call cancelled or deadline exceeded")
    }
}

impl std::error::Error for Cancelled {}

/// Caller cancellation and deadline context passed through [`Dispatch`] and handlers.
///
/// Zero external async dependencies so `frob-verbs --no-default-features --features
/// registry-only` preserves ADR-TETHER-02; adapts cleanly to `asupersync::Cx` once
/// `BD-008` (`INT-ASUP-01`) / `N33c-35` wires `asupersync` into the core workspace.
#[derive(Debug, Clone, Default)]
pub struct DispatchCx {
    cancelled: Arc<AtomicBool>,
    deadline: Option<Instant>,
}

impl DispatchCx {
    /// Construct an active, uncancelled context with no deadline.
    #[must_use]
    pub fn new() -> Self {
        Self {
            cancelled: Arc::new(AtomicBool::new(false)),
            deadline: None,
        }
    }

    /// Construct a context with an absolute deadline.
    #[must_use]
    pub fn with_deadline(deadline: Instant) -> Self {
        Self {
            cancelled: Arc::new(AtomicBool::new(false)),
            deadline: Some(deadline),
        }
    }

    /// Signal cancellation to every clone sharing this context.
    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::SeqCst);
    }

    /// Return `true` if cancellation was signalled or the deadline has passed.
    #[must_use]
    pub fn is_cancelled(&self) -> bool {
        if self.cancelled.load(Ordering::SeqCst) {
            return true;
        }
        self.deadline.is_some_and(|d| Instant::now() >= d)
    }

    /// Optional deadline carried by this context.
    #[must_use]
    pub const fn deadline(&self) -> Option<Instant> {
        self.deadline
    }

    /// Return `Err(Cancelled)` if cancelled or past deadline.
    pub fn checkpoint(&self) -> Result<(), Cancelled> {
        if self.is_cancelled() {
            Err(Cancelled)
        } else {
            Ok(())
        }
    }
}

/// Caller authorization state checked in pipeline step 3 (*Authorize*).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SessionAuth {
    /// Local caller before M2 capability tokens exist: holds the full local scope set.
    LocalFull { principal: String },
    /// Restricted session holding an explicit set of scopes (tested at M0 for step 3).
    Scoped {
        principal: String,
        scopes: BTreeSet<String>,
    },
}

impl Default for SessionAuth {
    fn default() -> Self {
        Self::local_full()
    }
}

impl SessionAuth {
    /// Default M0/M1 local principal holding full local scopes.
    #[must_use]
    pub fn local_full() -> Self {
        Self::LocalFull {
            principal: "local".to_owned(),
        }
    }

    /// Construct a restricted synthetic/token caller holding only `scopes`.
    #[must_use]
    pub fn scoped<I, S>(principal: impl Into<String>, scopes: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self::Scoped {
            principal: principal.into(),
            scopes: scopes.into_iter().map(Into::into).collect(),
        }
    }

    /// Principal identifier of the caller.
    #[must_use]
    pub fn principal(&self) -> &str {
        match self {
            Self::LocalFull { principal } | Self::Scoped { principal, .. } => principal,
        }
    }

    /// Return `true` iff this caller holds `scope`.
    #[must_use]
    pub fn has_scope(&self, scope: &str) -> bool {
        match self {
            Self::LocalFull { .. } => true,
            Self::Scoped { scopes, .. } => scopes.contains(scope),
        }
    }
}

/// Exact integer session budget tracker (verifies steps 1 and 2 charge zero budget).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SessionBudget {
    remaining: BTreeMap<String, u64>,
}

impl SessionBudget {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            remaining: BTreeMap::new(),
        }
    }

    #[must_use]
    pub fn with_unit(mut self, unit: impl Into<String>, amount: u64) -> Self {
        self.remaining.insert(unit.into(), amount);
        self
    }

    #[must_use]
    pub fn remaining(&self, unit: &str) -> u64 {
        self.remaining.get(unit).copied().unwrap_or(0)
    }

    /// Debit `amount` from `unit`; returns `false` without mutating if insufficient.
    pub fn charge(&mut self, unit: &str, amount: u64) -> bool {
        let Some(slot) = self.remaining.get_mut(unit) else {
            return false;
        };
        if *slot < amount {
            return false;
        }
        *slot -= amount;
        true
    }

    #[must_use]
    pub const fn snapshot(&self) -> &BTreeMap<String, u64> {
        &self.remaining
    }
}

/// Behaviour when a caller cancels its wait on a running job (`on_cancel` common field).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OnCancel {
    /// Leave the job running in the daemon (default; INV-TETHER-10).
    #[default]
    Leave,
    /// Request job cancellation when the caller disconnects/cancels.
    Cancel,
}

impl OnCancel {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Leave => "leave",
            Self::Cancel => "cancel",
        }
    }

    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "leave" => Some(Self::Leave),
            "cancel" => Some(Self::Cancel),
            _ => None,
        }
    }
}

/// Common request modifiers shared across CLI and MCP (`frob.rpc/1` `common` object).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct CommonArgs {
    pub idempotency_key: Option<String>,
    pub budget: Option<Tree>,
    pub sync_wait_ms: Option<i64>,
    pub estimate_only: bool,
    pub on_cancel: OnCancel,
    pub envelope_scope: Option<String>,
}

impl CommonArgs {
    pub(crate) fn write_canonical(&self, o: &mut Object<'_>) -> Result<(), CanonError> {
        match &self.budget {
            Some(b) => o.member_term("budget", b)?,
            None => o.null("budget")?,
        }
        match &self.envelope_scope {
            Some(s) => o.str("envelope_scope", s)?,
            None => o.null("envelope_scope")?,
        }
        o.bool("estimate_only", self.estimate_only)?;
        match &self.idempotency_key {
            Some(k) => o.str("idempotency_key", k)?,
            None => o.null("idempotency_key")?,
        }
        o.str("on_cancel", self.on_cancel.as_str())?;
        match self.sync_wait_ms {
            Some(ms) => o.int53("sync_wait_ms", ms)?,
            None => o.null("sync_wait_ms")?,
        }
        Ok(())
    }
}

/// Raw unvalidated surface request entering pipeline step 1.
#[derive(Debug, Clone, PartialEq)]
pub struct Request {
    pub verb: String,
    pub args: Tree,
    pub common: CommonArgs,
    /// Optional wire session token (`None` for local unauthenticated M0/M1 calls).
    pub session: Option<String>,
    /// Caller authorization state checked in step 3.
    pub auth: SessionAuth,
}

impl Request {
    /// Construct a local request with default common arguments and full local auth.
    #[must_use]
    pub fn new(verb: impl Into<String>, args: Tree) -> Self {
        Self {
            verb: verb.into(),
            args,
            common: CommonArgs::default(),
            session: None,
            auth: SessionAuth::local_full(),
        }
    }

    #[must_use]
    pub fn with_common(mut self, common: CommonArgs) -> Self {
        self.common = common;
        self
    }

    #[must_use]
    pub fn with_session(mut self, session: Option<String>) -> Self {
        self.session = session;
        self
    }

    #[must_use]
    pub fn with_auth(mut self, auth: SessionAuth) -> Self {
        self.auth = auth;
        self
    }
}

/// Execution tier of a `Kind::Run` verb request (governs step 7 no-daemon routing).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunTier {
    /// CPU-tier run: executes in process with `executor: in-process` when no daemon answers.
    Cpu,
    /// Device-tier run: requires `frobd` to own the device; refused with `FROB-REF-601` when
    /// no daemon answers.
    Device,
}

/// Request that has passed steps 1–5 (located, validated, authorized, resolved, digested).
#[derive(Debug, Clone, PartialEq)]
pub struct ResolvedRequest {
    pub verb: String,
    pub kind: Kind,
    pub run_tier: RunTier,
    pub args: Tree,
    pub bound: Bound,
    pub fixtures: Vec<FixtureBinding>,
    pub common: CommonArgs,
    pub principal: String,
    pub session: Option<String>,
    pub auth: SessionAuth,
    /// Domain-separated canonical BLAKE3 digest over resolved args, bound IDs, fixture
    /// file digests, and semantic common options (step 5, ADR-TETHER-11).
    pub digest: Blake3,
}

/// Identifier used to re-attach to an existing job (`Dispatch::attach`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JobRef {
    /// Attach by daemon job identifier (`job.wait`).
    Id(String),
    /// Attach by `(verb, idempotency_key)` (`job.find`).
    Key {
        verb: String,
        idempotency_key: String,
    },
}

/// One progress frame emitted ahead of the terminal envelope.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProgressFrame {
    pub job: String,
    pub state: String,
    pub progress: Progress,
}

impl ProgressFrame {
    pub fn new(
        job: impl Into<String>,
        state: impl Into<String>,
        progress: Progress,
    ) -> Result<Self, EnvelopeError> {
        let job = job.into();
        let state = state.into();
        if job.is_empty() || job.chars().any(char::is_control) {
            return Err(EnvelopeError::InvalidField {
                field: "progress.job",
                reason: "empty or contains control characters".into(),
            });
        }
        let mut chars = state.chars();
        if !(chars.next().is_some_and(|c| c.is_ascii_lowercase())
            && chars.all(|c| c.is_ascii_lowercase() || c == '-'))
        {
            return Err(EnvelopeError::InvalidField {
                field: "progress.state",
                reason: format!("`{state}` is not a job state (`^[a-z][a-z-]*$`)"),
            });
        }
        Ok(Self {
            job,
            state,
            progress,
        })
    }

    pub(crate) fn write_canonical(&self, o: &mut Object<'_>) -> Result<(), CanonError> {
        o.int53(
            "done",
            i64::try_from(self.progress.done())
                .map_err(|_| CanonError::Int53OutOfRange { value: i64::MAX })?,
        )?;
        o.str("job", &self.job)?;
        o.str("state", &self.state)?;
        o.int53(
            "total",
            i64::try_from(self.progress.total())
                .map_err(|_| CanonError::Int53OutOfRange { value: i64::MAX })?,
        )?;
        o.str("unit", self.progress.unit())
    }
}

/// A schema-validated, canonical `frob.envelope/1` carried across both back ends.
#[derive(Debug, Clone, PartialEq)]
pub struct WireEnvelope {
    bytes: CanonicalBytes,
    read: ReadEnvelope,
}

impl WireEnvelope {
    /// Serialize `envelope` canonically and validate it through `read_envelope_with`.
    pub fn from_envelope(
        envelope: &Envelope,
        registry: &RefusalRegistry,
    ) -> Result<Self, DispatchError> {
        let bytes = envelope.to_canonical().map_err(DispatchError::Canon)?;
        let schema = base_schema().map_err(DispatchError::Envelope)?;
        let read =
            read_envelope_with(bytes.as_bytes(), schema, registry).map_err(DispatchError::Read)?;
        Ok(Self { bytes, read })
    }

    /// Validate raw envelope JSON bytes against `frob.envelope/1` and `registry`,
    /// re-serializing canonically so equal envelopes are always equal bytes.
    pub fn from_bytes(raw: &[u8], registry: &RefusalRegistry) -> Result<Self, DispatchError> {
        let schema = base_schema().map_err(DispatchError::Envelope)?;
        let read = read_envelope_with(raw, schema, registry).map_err(DispatchError::Read)?;
        let bytes = read.tree().to_canonical().map_err(DispatchError::Canon)?;
        Ok(Self { bytes, read })
    }

    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        self.bytes.as_bytes()
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        self.bytes.as_str()
    }

    #[must_use]
    pub const fn read(&self) -> &ReadEnvelope {
        &self.read
    }

    #[must_use]
    pub const fn tree(&self) -> &Tree {
        self.read.tree()
    }

    #[must_use]
    pub fn verb(&self) -> &str {
        self.read.verb()
    }

    #[must_use]
    pub const fn exit(&self) -> ExitCode {
        self.read.exit()
    }

    #[must_use]
    pub fn refusal_code(&self) -> Option<&str> {
        self.read.refusal_code()
    }
}

/// One frame in a verb call's response stream: zero or more progress events, then one envelope.
#[derive(Debug, Clone, PartialEq)]
pub enum Frame {
    Progress(ProgressFrame),
    Envelope(WireEnvelope),
}

/// Response stream returned by [`Dispatch::submit`] and [`Dispatch::attach`].
#[derive(Debug, Clone, PartialEq)]
pub struct JobStream {
    frames: VecDeque<Frame>,
    handle: Option<JobHandle>,
}

impl JobStream {
    /// Build a validated stream: zero or more `Progress` frames terminated by exactly one
    /// `Envelope` frame.
    pub fn from_frames(
        frames: Vec<Frame>,
        handle: Option<JobHandle>,
    ) -> Result<Self, DispatchError> {
        let Some(Frame::Envelope(_)) = frames.last() else {
            return Err(DispatchError::Protocol(
                "job stream must terminate with an envelope frame".to_owned(),
            ));
        };
        if frames[..frames.len() - 1]
            .iter()
            .any(|f| matches!(f, Frame::Envelope(_)))
        {
            return Err(DispatchError::Protocol(
                "job stream carried more than one envelope frame".to_owned(),
            ));
        }
        Ok(Self {
            frames: frames.into(),
            handle,
        })
    }

    /// Build a stream containing a single terminal envelope and no progress frames.
    #[must_use]
    pub fn single_envelope(envelope: WireEnvelope) -> Self {
        let mut frames = VecDeque::with_capacity(1);
        frames.push_back(Frame::Envelope(envelope));
        Self {
            frames,
            handle: None,
        }
    }

    /// Yield the next frame, observing cancellation on `cx`.
    pub fn next_frame(&mut self, cx: &DispatchCx) -> Result<Option<Frame>, Cancelled> {
        cx.checkpoint()?;
        Ok(self.frames.pop_front())
    }

    /// Optional job handle associated with this stream.
    #[must_use]
    pub const fn handle(&self) -> Option<&JobHandle> {
        self.handle.as_ref()
    }

    /// Drain all frames into `(progress_frames, terminal_envelope)`.
    pub fn into_parts(mut self) -> Result<(Vec<ProgressFrame>, WireEnvelope), DispatchError> {
        let mut progress = Vec::new();
        while let Some(frame) = self.frames.pop_front() {
            match frame {
                Frame::Progress(p) => progress.push(p),
                Frame::Envelope(e) => return Ok((progress, e)),
            }
        }
        Err(DispatchError::Protocol(
            "job stream ended without a terminal envelope".to_owned(),
        ))
    }
}

/// Errors returned by the dispatch transport or setup layer (domain refusals travel inside
/// `JobStream`'s terminal `WireEnvelope`).
#[derive(Debug)]
pub enum DispatchError {
    Cancelled(Cancelled),
    Envelope(EnvelopeError),
    Read(ReadError),
    Canon(CanonError),
    Socket(SocketError),
    Protocol(String),
    Io(std::io::Error),
}

impl fmt::Display for DispatchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Cancelled(e) => fmt::Display::fmt(e, f),
            Self::Envelope(e) => fmt::Display::fmt(e, f),
            Self::Read(e) => fmt::Display::fmt(e, f),
            Self::Canon(e) => fmt::Display::fmt(e, f),
            Self::Socket(e) => fmt::Display::fmt(e, f),
            Self::Protocol(m) => write!(f, "dispatch protocol error: {m}"),
            Self::Io(e) => write!(f, "dispatch I/O error: {e}"),
        }
    }
}

impl std::error::Error for DispatchError {}

impl From<Cancelled> for DispatchError {
    fn from(e: Cancelled) -> Self {
        Self::Cancelled(e)
    }
}

impl From<SocketError> for DispatchError {
    fn from(e: SocketError) -> Self {
        Self::Socket(e)
    }
}

impl From<std::io::Error> for DispatchError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

/// The single application-service dispatch trait shared by `InProcess` and `Socket`
/// (ADR-TETHER-03).
///
/// Every verb call returns a [`JobStream`] containing zero or more progress frames and
/// ending in exactly one validated `frob.envelope/1` (whether completed, flagged,
/// deferred, or refused). Transport failures return `Err(DispatchError)`.
pub trait Dispatch: Send + Sync {
    /// Submit a request. Replay-safe when carrying an idempotency key.
    fn submit(&self, cx: &DispatchCx, req: &Request) -> Result<JobStream, DispatchError>;

    /// Re-attach to an existing job by identifier (`job.wait`) or idempotency key (`job.find`).
    fn attach(&self, cx: &DispatchCx, job: &JobRef) -> Result<JobStream, DispatchError>;
}

/// Mask `cost.wall_ms` (zeroed) and the in-process executor warnings (`executor-in-process`,
/// `no-job-record`) so envelopes from `InProcess` and `Socket` compare byte-identically
/// (N32a-04 determinism & back-end parity contract).
pub fn mask_envelope_for_parity(env: &WireEnvelope) -> Result<CanonicalBytes, DispatchError> {
    let mut tree = env.tree().clone();
    if let Tree::Obj(members) = &mut tree {
        for (k, v) in members.iter_mut() {
            if k == "cost" {
                if let Tree::Obj(cost_members) = v {
                    for (ck, cv) in cost_members.iter_mut() {
                        if ck == "wall_ms" {
                            *cv = Tree::Num("0".to_owned());
                        }
                    }
                }
            } else if k == "warnings" {
                if let Tree::Arr(items) = v {
                    items.retain(|w| {
                        let code = w.get("code").and_then(Tree::as_str);
                        !matches!(code, Some("executor-in-process" | "no-job-record"))
                    });
                }
            }
        }
    }
    tree.to_canonical().map_err(DispatchError::Canon)
}

/// Find the first differing JSON pointer between two JSON trees (diagnostic helper for
/// parity tests and `scripts/e2e/tether_dispatch.sh`).
#[must_use]
pub fn first_diff_pointer(a: &Tree, b: &Tree) -> Option<(String, String, String)> {
    diff_walk("", a, b)
}

fn diff_walk(ptr: &str, a: &Tree, b: &Tree) -> Option<(String, String, String)> {
    match (a, b) {
        (Tree::Obj(ma), Tree::Obj(mb)) => {
            let mut keys: BTreeSet<&str> = BTreeSet::new();
            for (k, _) in ma {
                keys.insert(k.as_str());
            }
            for (k, _) in mb {
                keys.insert(k.as_str());
            }
            for key in keys {
                let child_ptr = format!("{ptr}/{}", escape_ptr(key));
                match (a.get(key), b.get(key)) {
                    (Some(va), Some(vb)) => {
                        if let Some(found) = diff_walk(&child_ptr, va, vb) {
                            return Some(found);
                        }
                    }
                    (Some(va), None) => {
                        return Some((child_ptr, render_tree(va), "<absent>".to_owned()));
                    }
                    (None, Some(vb)) => {
                        return Some((child_ptr, "<absent>".to_owned(), render_tree(vb)));
                    }
                    (None, None) => {}
                }
            }
            None
        }
        (Tree::Arr(aa), Tree::Arr(ab)) => {
            let max_len = aa.len().max(ab.len());
            for i in 0..max_len {
                let child_ptr = format!("{ptr}/{i}");
                match (aa.get(i), ab.get(i)) {
                    (Some(va), Some(vb)) => {
                        if let Some(found) = diff_walk(&child_ptr, va, vb) {
                            return Some(found);
                        }
                    }
                    (Some(va), None) => {
                        return Some((child_ptr, render_tree(va), "<absent>".to_owned()));
                    }
                    (None, Some(vb)) => {
                        return Some((child_ptr, "<absent>".to_owned(), render_tree(vb)));
                    }
                    (None, None) => {}
                }
            }
            None
        }
        _ => {
            if a.same_value(b) {
                None
            } else {
                let p = if ptr.is_empty() {
                    "/".to_owned()
                } else {
                    ptr.to_owned()
                };
                Some((p, render_tree(a), render_tree(b)))
            }
        }
    }
}

fn render_tree(t: &Tree) -> String {
    t.to_canonical()
        .map_or_else(|_| format!("{t:?}"), |c| c.as_str().to_owned())
}

/// Canonical wrapper for arbitrary `Canonical` helper serialization.
pub(crate) struct FnCanonical<F>(pub(crate) F);

impl<F> Canonical for FnCanonical<F>
where
    F: Fn(Value<'_>) -> Result<(), CanonError>,
{
    fn write(&self, w: Value<'_>) -> Result<(), CanonError> {
        (self.0)(w)
    }
}
