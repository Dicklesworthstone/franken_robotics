//! In-process dispatch back end (`ADR-TETHER-03` back end 1, default under `feature = "handlers"`)
//! (bead N32a-04, §32.3).
//!
//! Executes pipeline steps 1–5 via [`Pipeline`], checks step 6 (`FROB-REF-724` when no handler
//! is registered), and routes step 7:
//! - `read`, `mutate`, and `admin` run in the calling process through [`HandlerTable`].
//! - `run` verbs forward to `frobd` via [`DaemonClient`] whenever a daemon answers.
//! - With no daemon answering: CPU-tier `run` runs in process with `executor: in-process`
//!   (plus `no-job-record` at M0 before a local ledger is attached); Device-tier `run` is
//!   refused with `FROB-REF-601`, `pointer: "daemon"`, exit 6, `retry: backoff`, first
//!   repair naming `frobd`.
//!
//! Determinism D0 apart from `cost.wall_ms` and the `executor` warning; prints nothing
//! (L-7); zero `unsafe`.

use std::collections::BTreeMap;
use std::fmt;
use std::panic::{self, AssertUnwindSafe};
use std::sync::Arc;

use frob_core::refusal::{Repair, RetryPolicy};
use frob_core::registry::refusals::RefusalRegistry;

use crate::envelope::{Bound, Cost, Envelope, FaultContext, HandlerFault, RefusalFields, Warning};
use crate::registry::{Kind, LintError};

use super::pipeline::Pipeline;
use super::socket::{DaemonClient, SocketError};
use super::{
    Dispatch, DispatchCx, DispatchError, Frame, JobRef, JobStream, ProgressFrame, Request,
    ResolvedRequest, RunTier, WireEnvelope,
};

/// Sink through which a [`VerbHandler`] emits progress frames ahead of its terminal envelope.
pub trait ProgressEmit {
    fn emit(&mut self, frame: ProgressFrame);
}

struct VecProgressSink(Vec<Frame>);

impl ProgressEmit for VecProgressSink {
    fn emit(&mut self, frame: ProgressFrame) {
        self.0.push(Frame::Progress(frame));
    }
}

/// Read-only context handed to a [`VerbHandler`] invocation.
#[derive(Debug, Clone, Copy)]
pub struct HandlerContext<'a> {
    pub refusals: &'a RefusalRegistry,
    pub diagnostic_path: &'a str,
}

impl<'a> HandlerContext<'a> {
    /// Build a refusal envelope resolved against this dispatch instance's refusal registry.
    pub fn refuse(
        &self,
        verb: &str,
        code: &str,
        fields: RefusalFields,
        cost: Cost,
    ) -> Result<Envelope, crate::envelope::EnvelopeError> {
        Envelope::from_code_in(self.refusals, verb, code, fields, cost)
    }
}

/// Thin per-verb handler binding living in `frob-verbs::handlers` (or a synthetic test table).
pub trait VerbHandler: Send + Sync {
    /// Execute one resolved request, emitting zero or more progress frames to `sink` and
    /// returning the terminal [`Envelope`] (or a [`HandlerFault`]).
    fn run(
        &self,
        cx: &DispatchCx,
        req: &ResolvedRequest,
        ctx: &HandlerContext<'_>,
        sink: &mut dyn ProgressEmit,
    ) -> Result<Envelope, HandlerFault>;
}

/// Closure adapter implementing [`VerbHandler`].
pub struct FnHandler<F>(pub F);

impl<F> VerbHandler for FnHandler<F>
where
    F: Fn(
            &DispatchCx,
            &ResolvedRequest,
            &HandlerContext<'_>,
            &mut dyn ProgressEmit,
        ) -> Result<Envelope, HandlerFault>
        + Send
        + Sync,
{
    fn run(
        &self,
        cx: &DispatchCx,
        req: &ResolvedRequest,
        ctx: &HandlerContext<'_>,
        sink: &mut dyn ProgressEmit,
    ) -> Result<Envelope, HandlerFault> {
        (self.0)(cx, req, ctx, sink)
    }
}

/// Static/runtime map from registered verb name -> [`VerbHandler`].
#[derive(Default)]
pub struct HandlerTable {
    handlers: BTreeMap<String, Arc<dyn VerbHandler>>,
}

impl fmt::Debug for HandlerTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HandlerTable")
            .field("count", &self.handlers.len())
            .finish()
    }
}

impl HandlerTable {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            handlers: BTreeMap::new(),
        }
    }

    /// Register `handler` for `verb` (replaces any prior handler for the same name).
    pub fn register(
        &mut self,
        verb: impl Into<String>,
        handler: Arc<dyn VerbHandler>,
    ) -> &mut Self {
        self.handlers.insert(verb.into(), handler);
        self
    }

    #[must_use]
    pub fn get(&self, verb: &str) -> Option<&Arc<dyn VerbHandler>> {
        self.handlers.get(verb)
    }

    #[must_use]
    pub fn contains(&self, verb: &str) -> bool {
        self.handlers.contains_key(verb)
    }

    /// Reject two registry names bound to the same handler object.
    pub fn check_one_name_per_handler(&self) -> Result<(), LintError> {
        let mut names = BTreeMap::new();
        for (name, handler) in &self.handlers {
            if let Some(prior) = names.insert(Arc::as_ptr(handler).cast::<()>(), name) {
                return Err(LintError::new(
                    name,
                    "NJL-04",
                    "same-handler",
                    name,
                    format!("shares the handler bound to `{prior}`"),
                ));
            }
        }
        Ok(())
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.handlers.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.handlers.is_empty()
    }
}

/// In-process dispatch back end (`ADR-TETHER-03` back end 1).
#[derive(Debug)]
pub struct InProcess {
    pipeline: Pipeline,
    table: HandlerTable,
    daemon: Option<DaemonClient>,
    has_local_ledger: bool,
    diagnostic_path: String,
}

impl InProcess {
    /// Construct an in-process back end over `pipeline` and `table` with no daemon attached.
    #[must_use]
    pub fn new(pipeline: Pipeline, table: HandlerTable) -> Self {
        Self {
            pipeline,
            table,
            daemon: None,
            has_local_ledger: false,
            diagnostic_path: ".frob/diag/dispatch-fault.txt".to_owned(),
        }
    }

    /// Attach a [`DaemonClient`] for step-7 `run`-verb routing and `attach` forwarding.
    #[must_use]
    pub fn with_daemon(mut self, daemon: DaemonClient) -> Self {
        self.daemon = Some(daemon);
        self
    }

    /// Mark whether a local single-writer ledger is attached (false at M0 -> `no-job-record`
    /// warning on in-process CPU-tier `run` calls).
    #[must_use]
    pub const fn with_local_ledger(mut self, has_local_ledger: bool) -> Self {
        self.has_local_ledger = has_local_ledger;
        self
    }

    #[must_use]
    pub const fn pipeline(&self) -> &Pipeline {
        &self.pipeline
    }

    #[must_use]
    pub const fn table(&self) -> &HandlerTable {
        &self.table
    }

    fn execute_local(
        &self,
        cx: &DispatchCx,
        resolved: &ResolvedRequest,
        handler: &dyn VerbHandler,
        in_process_run_warnings: bool,
    ) -> Result<JobStream, DispatchError> {
        cx.checkpoint()?;
        let refusals = self.pipeline.refusals();
        let ctx = HandlerContext {
            refusals,
            diagnostic_path: &self.diagnostic_path,
        };
        let mut sink = VecProgressSink(Vec::new());

        let outcome = panic::catch_unwind(AssertUnwindSafe(|| {
            handler.run(cx, resolved, &ctx, &mut sink)
        }));

        let fault = || {
            Envelope::internal_fault(
                &FaultContext {
                    verb: resolved.verb.clone(),
                    idempotency_key: resolved.common.idempotency_key.clone(),
                    diagnostic_path: self.diagnostic_path.clone(),
                },
                Cost::ZERO,
            )
        };

        let mut env = match outcome {
            Ok(Ok(e)) if e.verb() == resolved.verb => e,
            Ok(Ok(_)) | Ok(Err(HandlerFault::Unmapped(_))) | Err(_) => fault(),
            Ok(Err(HandlerFault::Refused { code, fields })) => {
                Envelope::from_code_in(refusals, &resolved.verb, &code, fields, Cost::ZERO)
                    .unwrap_or_else(|_| fault())
            }
        };

        if resolved.bound != Bound::new() {
            env = env.with_bound(resolved.bound.clone());
        }
        if in_process_run_warnings {
            env = env.with_warning(Warning::executor_in_process());
            if !self.has_local_ledger {
                if let Ok(w) = Warning::new(
                    "no-job-record",
                    "no local job ledger attached at M0; run has no persisted job record",
                ) {
                    env = env.with_warning(w);
                }
            }
        }

        let wire = WireEnvelope::from_envelope(&env, refusals)?;
        sink.0.push(Frame::Envelope(wire));
        JobStream::from_frames(sink.0, None)
    }

    fn refuse_no_daemon_device(&self, verb: &str) -> Result<JobStream, DispatchError> {
        let refusals = self.pipeline.refusals();
        let fields = RefusalFields {
            message: "device-tier run requires frobd, but no daemon is answering".to_owned(),
            pointer: "daemon".to_owned(),
            observed: "no frobd answering".to_owned(),
            required: "running frobd owning the device".to_owned(),
            unit: String::new(),
            detail: None,
            affected_claims: Vec::new(),
            retry: RetryPolicy::Backoff,
            repairs: vec![
                Repair::with_detail(
                    "frobd --state-root ~/.frob",
                    "Start the local daemon frobd so it owns the device and job ledger.",
                ),
                Repair::with_detail(
                    "frob sim run --tier referee",
                    "Use a CPU-tier run that can execute in process without a daemon.",
                ),
            ],
        };
        let env = Envelope::from_code_in(refusals, verb, "FROB-REF-601", fields, Cost::ZERO)
            .unwrap_or_else(|_| {
                Envelope::internal_fault(
                    &FaultContext {
                        verb: verb.to_owned(),
                        idempotency_key: None,
                        diagnostic_path: self.diagnostic_path.clone(),
                    },
                    Cost::ZERO,
                )
            });
        Ok(JobStream::single_envelope(WireEnvelope::from_envelope(
            &env, refusals,
        )?))
    }
}

fn is_daemon_unreachable_err(err: &DispatchError) -> bool {
    match err {
        DispatchError::Socket(SocketError::Io(io_err)) | DispatchError::Io(io_err) => matches!(
            io_err.kind(),
            std::io::ErrorKind::NotFound
                | std::io::ErrorKind::ConnectionRefused
                | std::io::ErrorKind::ConnectionReset
        ),
        _ => false,
    }
}

impl Dispatch for InProcess {
    fn submit(&self, cx: &DispatchCx, req: &Request) -> Result<JobStream, DispatchError> {
        cx.checkpoint()?;

        // Steps 1–5: Locate, Validate, Authorize, Resolve, Digest.
        let resolved = match self.pipeline.prepare(req)? {
            Ok(r) => r,
            Err(refusal_env) => return Ok(JobStream::single_envelope(refusal_env)),
        };

        // Step 6: Implemented? Missing handler in this build -> FROB-REF-724 (exit 5).
        let Some(handler) = self.table.get(&resolved.verb) else {
            let env = self.pipeline.refusal_envelope(
                &resolved.verb,
                "FROB-REF-724",
                RefusalFields {
                    message: format!(
                        "verb `{}` has no handler linked in this build",
                        resolved.verb
                    ),
                    pointer: "/verb".to_owned(),
                    observed: resolved.verb.clone(),
                    required: "a linked handler in this build".to_owned(),
                    unit: String::new(),
                    detail: None,
                    affected_claims: Vec::new(),
                    retry: RetryPolicy::Never,
                    repairs: vec![Repair::with_detail(
                        "frob capabilities --json",
                        "List the verbs and capabilities linked in this build.",
                    )],
                },
            )?;
            return Ok(JobStream::single_envelope(env));
        };

        // Step 7: Route by (kind, run_tier, daemon reachability).
        match resolved.kind {
            Kind::Read | Kind::Mutate | Kind::Admin => {
                self.execute_local(cx, &resolved, handler.as_ref(), false)
            }
            Kind::Run => {
                if let Some(daemon) = &self.daemon {
                    match daemon.call_request(cx, req) {
                        Ok(stream) => return Ok(stream),
                        Err(err) if is_daemon_unreachable_err(&err) => {
                            // Fall through to the no-daemon branch below.
                        }
                        Err(err) => return Err(err),
                    }
                }
                match resolved.run_tier {
                    RunTier::Cpu => self.execute_local(cx, &resolved, handler.as_ref(), true),
                    RunTier::Device => self.refuse_no_daemon_device(&resolved.verb),
                }
            }
        }
    }

    fn attach(&self, cx: &DispatchCx, job: &JobRef) -> Result<JobStream, DispatchError> {
        cx.checkpoint()?;
        if let Some(daemon) = &self.daemon {
            match daemon.call_attach(cx, job, None) {
                Ok(stream) => return Ok(stream),
                Err(err) if is_daemon_unreachable_err(&err) => {}
                Err(err) => return Err(err),
            }
        }
        self.refuse_no_daemon_device("job.wait")
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicU64, Ordering};

    use crate::dispatch::fixture;
    use crate::dispatch::rpc::RpcServerHandle;
    use crate::dispatch::socket::test_socket_path;
    use crate::envelope::{ExitCode, Tree};

    use super::*;

    #[test]
    fn not_implemented_724() {
        let inproc = fixture::synthetic_in_process();
        let req = Request::new("unimpl.verb", Tree::parse(b"{}").unwrap());
        let (_progress, env) = inproc
            .submit(&DispatchCx::new(), &req)
            .expect("submit ok")
            .into_parts()
            .expect("envelope");
        assert_eq!(env.refusal_code(), Some("FROB-REF-724"));
        assert_eq!(env.exit(), ExitCode::Capability);
        assert_eq!(
            env.tree()
                .get("refusal")
                .and_then(|r| r.get("retry"))
                .and_then(Tree::as_str),
            Some("never")
        );
    }

    #[test]
    fn route_by_kind_and_daemon() {
        let refusals = fixture::synthetic_refusals();
        let sock = test_socket_path("route");
        let server_backend = fixture::synthetic_dispatch_service();
        let server = RpcServerHandle::spawn_current_uid(&sock, server_backend, refusals.clone())
            .expect("spawn server");

        // Build an InProcess client with an instrumented local `sim.run` counter so we can
        // prove the client handler table is NOT called while the server is up, and IS called
        // when the server is down.
        let local_calls = Arc::new(AtomicU64::new(0));
        let local_calls_c = Arc::clone(&local_calls);
        let mut table = fixture::synthetic_handler_table();
        let inner = table.get("sim.run").expect("sim.run handler").clone();
        table.register(
            "sim.run",
            Arc::new(FnHandler(
                move |cx: &DispatchCx,
                      req: &ResolvedRequest,
                      ctx: &HandlerContext<'_>,
                      sink: &mut dyn ProgressEmit| {
                    local_calls_c.fetch_add(1, Ordering::SeqCst);
                    inner.run(cx, req, ctx, sink)
                },
            )),
        );

        let daemon_client = DaemonClient::new(&sock, refusals.clone()).expect("client");
        let inproc =
            InProcess::new(fixture::synthetic_pipeline(), table).with_daemon(daemon_client);

        let cpu_run = Request::new(
            "sim.run",
            Tree::parse(br#"{"design":"quad-a","duration_s":1.0,"tier":"cpu"}"#).unwrap(),
        );

        // 1. With server loop up: CPU-tier `run` routes to daemon; server counter rises by 1;
        //    client handler table is NOT called.
        assert_eq!(server.request_count(), 0);
        let (prog_daemon, env_daemon) = inproc
            .submit(&DispatchCx::new(), &cpu_run)
            .expect("submit via daemon")
            .into_parts()
            .expect("envelope");
        assert_eq!(env_daemon.exit(), ExitCode::Completed);
        assert!(!prog_daemon.is_empty(), "run verb emits progress frames");
        assert_eq!(
            server.request_count(),
            1,
            "daemon must have served the run call"
        );
        assert_eq!(
            local_calls.load(Ordering::SeqCst),
            0,
            "client handler table must not run while daemon answers"
        );

        // 2. Shut the server down: the same CPU-tier `run` now runs in process with the
        //    `executor-in-process` and `no-job-record` warnings.
        let _stats = server.shutdown().expect("shutdown server");
        let (_prog_local, env_local) = inproc
            .submit(&DispatchCx::new(), &cpu_run)
            .expect("submit no-daemon")
            .into_parts()
            .expect("envelope");
        assert_eq!(env_local.exit(), ExitCode::Completed);
        assert_eq!(
            local_calls.load(Ordering::SeqCst),
            1,
            "client handler table must run when daemon is down"
        );
        let warnings: Vec<&str> = env_local
            .tree()
            .get("warnings")
            .and_then(Tree::as_array)
            .expect("warnings")
            .iter()
            .filter_map(|w| w.get("code").and_then(Tree::as_str))
            .collect();
        assert!(
            warnings.contains(&"executor-in-process"),
            "missing executor-in-process warning: {warnings:?}"
        );
        assert!(
            warnings.contains(&"no-job-record"),
            "missing M0 no-job-record warning: {warnings:?}"
        );
    }

    #[test]
    fn refusal_601_no_daemon() {
        let inproc = fixture::synthetic_in_process();

        // Device-tier `run` with no daemon -> exit 6, FROB-REF-601, pointer `daemon`,
        // retry `backoff`, first repair naming `frobd`.
        let dev_run = Request::new(
            "sim.gpu",
            Tree::parse(br#"{"design":"quad-a","duration_s":1.0,"tier":"device"}"#).unwrap(),
        );
        let (_prog, env) = inproc
            .submit(&DispatchCx::new(), &dev_run)
            .expect("submit ok")
            .into_parts()
            .expect("envelope");
        assert_eq!(env.exit(), ExitCode::Unavailable);
        assert_eq!(env.exit().code(), 6);
        assert_eq!(env.refusal_code(), Some("FROB-REF-601"));
        let r = env.tree().get("refusal").expect("refusal block");
        assert_eq!(r.get("pointer").and_then(Tree::as_str), Some("daemon"));
        assert_eq!(r.get("retry").and_then(Tree::as_str), Some("backoff"));
        let first_repair = r
            .get("repairs")
            .and_then(Tree::as_array)
            .and_then(|a| a.first())
            .and_then(|x| x.get("action"))
            .and_then(Tree::as_str)
            .unwrap_or("");
        assert!(
            first_repair.contains("frobd"),
            "first repair must name frobd, got `{first_repair}`"
        );

        // CPU-tier `run` in the exact same no-daemon state runs in process with `executor-in-process`.
        let cpu_run = Request::new(
            "sim.run",
            Tree::parse(br#"{"design":"quad-a","duration_s":1.0,"tier":"cpu"}"#).unwrap(),
        );
        let (_prog2, env2) = inproc
            .submit(&DispatchCx::new(), &cpu_run)
            .expect("submit ok")
            .into_parts()
            .expect("envelope");
        assert_eq!(env2.exit(), ExitCode::Completed);
        let warnings: Vec<&str> = env2
            .tree()
            .get("warnings")
            .and_then(Tree::as_array)
            .expect("warnings")
            .iter()
            .filter_map(|w| w.get("code").and_then(Tree::as_str))
            .collect();
        assert!(warnings.contains(&"executor-in-process"));
    }
}
