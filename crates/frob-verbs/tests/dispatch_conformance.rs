//! Dual-backend dispatch conformance suite (bead N32a-04, ADR-TETHER-03).
//!
//! Runs the synthetic dispatch fixture through both `InProcess` (when `feature = "handlers"`
//! is enabled) and `Socket` (against `frob_verbs::rpc::serve` on a real mode-`0600` Unix
//! domain socket in a scratch directory), verifying byte-identical masked envelopes,
//! resolution echoing, and negative controls.
//!
//! Also compiles and passes under `--no-default-features --features registry-only` where
//! `Socket` is exposed and `InProcess` is absent (ADR-TETHER-02).

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use frob_verbs::dispatch::fixture::{
    QUAD_A_ID, parity_fixture_calls, synthetic_dispatch_service, synthetic_pipeline,
    synthetic_refusals,
};
use frob_verbs::dispatch::rpc::RpcServerHandle;
use frob_verbs::dispatch::{
    DaemonClient, Dispatch, DispatchCx, JobRef, Request, Socket, check_control_plane_granularity,
    first_diff_pointer, mask_envelope_for_parity,
};
use frob_verbs::envelope::{ExitCode, Tree};
use frob_verbs::registry::{Kind, Milestone, SchemaId, Scope, Tier, VerbEntry, WaitDefault};

#[cfg(feature = "handlers")]
use frob_core::evidence::{
    ClaimKind, ClaimProduct, DetClass, FlagSet, InputRef, RealityScope, Scope as EvScope, TierMeet,
};
#[cfg(feature = "handlers")]
use frob_core::id::hash_blob;
#[cfg(feature = "handlers")]
use frob_verbs::dispatch::ResolvedRequest;
#[cfg(feature = "handlers")]
use frob_verbs::dispatch::in_process::{
    FnHandler, HandlerContext, HandlerTable, InProcess, ProgressEmit,
};
#[cfg(feature = "handlers")]
use frob_verbs::envelope::{Cost, Envelope, EvidenceBlock, JsonBody};
#[cfg(feature = "handlers")]
use std::sync::Arc;

fn unique_socket_path(tag: &str) -> PathBuf {
    static SEQ: AtomicU64 = AtomicU64::new(0);
    let n = SEQ.fetch_add(1, Ordering::Relaxed);
    let pid = std::process::id();
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.subsec_nanos());
    let base = match std::env::var_os("BIG_SCRATCH").or_else(|| std::env::var_os("TMPDIR")) {
        Some(dir) if dir.len() <= 48 => PathBuf::from(dir),
        _ => PathBuf::from("/tmp"),
    };
    let dir = base.join(format!("fbh-c-{pid}-{nanos:09}-{n}"));
    fs::create_dir_all(&dir).expect("create scratch dir");
    let _ = fs::set_permissions(&dir, fs::Permissions::from_mode(0o700));
    dir.join(format!("{tag}.sock"))
}

/// Runs every entry of the synthetic dispatch fixture through `Socket` against the real
/// `frob_verbs::rpc::serve` server loop. Compiles and passes under BOTH `handlers` AND
/// `--no-default-features --features registry-only`.
#[test]
fn socket_conformance_all_fixture_entries() {
    let refusals = synthetic_refusals();
    let sock = unique_socket_path("sock-conf");
    let server =
        RpcServerHandle::spawn_current_uid(&sock, synthetic_dispatch_service(), refusals.clone())
            .expect("spawn rpc server");

    let client = DaemonClient::new(&sock, refusals).expect("create daemon client");
    let backend = Socket::new(client).with_local_pipeline(synthetic_pipeline());
    let cx = DispatchCx::new();

    for (name, req, want_exit) in parity_fixture_calls() {
        let (progress, env) = backend
            .submit(&cx, &req)
            .unwrap_or_else(|e| panic!("socket call `{name}` failed: {e}"))
            .into_parts()
            .expect("terminal envelope");
        assert_eq!(
            env.exit().code(),
            want_exit,
            "{name}: unexpected exit code (body={})",
            env.as_str()
        );
        if name.starts_with("run_") {
            assert!(!progress.is_empty(), "{name}: expected progress frames");
        }
    }

    // Verify `attach` round-trips cleanly over `frob.rpc/1`.
    let (_prog, attached) = backend
        .attach(&cx, &JobRef::Id("j:syn-cpu-1".to_owned()))
        .expect("attach ok")
        .into_parts()
        .expect("envelope");
    assert_eq!(attached.exit(), ExitCode::Completed);
    assert_eq!(
        attached
            .tree()
            .get("result")
            .and_then(|r| r.get("attached"))
            .and_then(Tree::as_str),
        Some("j:syn-cpu-1")
    );

    let _stats = server.shutdown().expect("shutdown server");
}

/// Acceptance criterion 1: every entry of the synthetic dispatch fixture returns byte-identical
/// masked envelopes across `InProcess` and `Socket` (against a real `rpc::serve` server loop
/// on a real mode-`0600` Unix domain socket).
#[cfg(feature = "handlers")]
#[test]
fn two_backends_equal() {
    let refusals = synthetic_refusals();
    let sock = unique_socket_path("parity");
    let server =
        RpcServerHandle::spawn_current_uid(&sock, synthetic_dispatch_service(), refusals.clone())
            .expect("spawn rpc server");

    // InProcess with daemon attached (handles device-tier `sim.gpu` via daemon, read/mutate in
    // process) AND a standalone no-daemon InProcess (runs CPU-tier `sim.run` in process with
    // `executor: in-process`, verifying the mask strips only `wall_ms` + executor warnings!).
    let inproc_with_daemon = frob_verbs::dispatch::fixture::synthetic_in_process()
        .with_daemon(DaemonClient::new(&sock, refusals.clone()).expect("daemon client"));
    let inproc_standalone = frob_verbs::dispatch::fixture::synthetic_in_process();
    let socket_backend = Socket::new(DaemonClient::new(&sock, refusals).expect("socket client"))
        .with_local_pipeline(synthetic_pipeline());

    let cx = DispatchCx::new();
    for (name, req, want_exit) in parity_fixture_calls() {
        let inproc_ref: &dyn Dispatch = if name == "run_device_sim_gpu" {
            &inproc_with_daemon
        } else {
            &inproc_standalone
        };

        let (p_in, env_in) = inproc_ref
            .submit(&cx, &req)
            .unwrap_or_else(|e| panic!("in-process `{name}` failed: {e}"))
            .into_parts()
            .expect("in-process envelope");
        let (p_sock, env_sock) = socket_backend
            .submit(&cx, &req)
            .unwrap_or_else(|e| panic!("socket `{name}` failed: {e}"))
            .into_parts()
            .expect("socket envelope");

        assert_eq!(env_in.exit().code(), want_exit, "{name} in-process exit");
        assert_eq!(env_sock.exit().code(), want_exit, "{name} socket exit");
        assert_eq!(
            p_in, p_sock,
            "{name}: progress frames must match across back ends"
        );

        let masked_in = mask_envelope_for_parity(&env_in).expect("mask in-process");
        let masked_sock = mask_envelope_for_parity(&env_sock).expect("mask socket");
        if masked_in.as_str() != masked_sock.as_str() {
            let diff = first_diff_pointer(
                &Tree::parse(masked_in.as_bytes()).unwrap(),
                &Tree::parse(masked_sock.as_bytes()).unwrap(),
            );
            panic!(
                "{name}: masked envelopes differ across back ends at {diff:?}\n  in-process: {}\n  socket    : {}",
                masked_in.as_str(),
                masked_sock.as_str()
            );
        }
    }

    let _stats = server.shutdown().expect("shutdown server");
}

/// Resolution echoing (`design.check quad-a` echoes `bound.design == QUAD_A_ID` identically
/// across both back ends).
#[cfg(feature = "handlers")]
#[test]
fn resolution_echoed() {
    let refusals = synthetic_refusals();
    let sock = unique_socket_path("resolve");
    let server =
        RpcServerHandle::spawn_current_uid(&sock, synthetic_dispatch_service(), refusals.clone())
            .expect("spawn rpc server");

    let inproc = frob_verbs::dispatch::fixture::synthetic_in_process();
    let socket = Socket::new(DaemonClient::new(&sock, refusals).expect("client"))
        .with_local_pipeline(synthetic_pipeline());
    let req = Request::new(
        "design.check",
        Tree::parse(br#"{"design":"quad-a"}"#).unwrap(),
    );
    let cx = DispatchCx::new();

    for (label, backend) in [
        ("in-process", &inproc as &dyn Dispatch),
        ("socket", &socket as &dyn Dispatch),
    ] {
        let (_p, env) = backend.submit(&cx, &req).unwrap().into_parts().unwrap();
        assert_eq!(env.exit(), ExitCode::Completed, "{label}");
        assert_eq!(
            env.tree()
                .get("bound")
                .and_then(|b| b.get("design"))
                .and_then(Tree::as_str),
            Some(QUAD_A_ID),
            "{label}: bound.design must echo resolved immutable id"
        );
    }

    let _stats = server.shutdown().expect("shutdown server");
}

/// Negative control (N32a-04): a handler that reads a process-global instead of its request
/// diverges between the two back ends and is caught by masked-envelope parity comparison.
#[cfg(feature = "handlers")]
#[test]
fn negative_control_process_global_handler_diverges() {
    let refusals = synthetic_refusals();

    let make_stateful_backend = |tag: &'static str| -> Arc<dyn Dispatch> {
        let mut table = HandlerTable::new();
        table.register(
            "design.check",
            Arc::new(FnHandler(
                move |_cx: &DispatchCx,
                      _req: &ResolvedRequest,
                      _ctx: &HandlerContext<'_>,
                      _sink: &mut dyn ProgressEmit| {
                    let prod = ClaimProduct::estimated(
                        ClaimKind::KnownAnswer,
                        InputRef::new(hash_blob(b"nc"), "/nc"),
                        EvScope::default(),
                        TierMeet::Static,
                        DetClass::D0Exact,
                        RealityScope::NoClaim,
                        FlagSet::new(),
                    );
                    let ev = EvidenceBlock::from_claim_product(&prod, "m:nc").unwrap();
                    let body = JsonBody::from_json(
                        format!(r#"{{"process_global_origin":"{tag}"}}"#).as_bytes(),
                    )
                    .unwrap();
                    Ok(Envelope::completed("design.check", body, ev, Cost::ZERO).unwrap())
                },
            )),
        );
        Arc::new(InProcess::new(synthetic_pipeline(), table))
    };

    let sock = unique_socket_path("nc-global");
    let server = RpcServerHandle::spawn_current_uid(
        &sock,
        make_stateful_backend("daemon-global"),
        refusals.clone(),
    )
    .expect("spawn server");
    let client_inproc = make_stateful_backend("client-global");
    let socket = Socket::new(DaemonClient::new(&sock, refusals).expect("client"))
        .with_local_pipeline(synthetic_pipeline());

    let req = Request::new(
        "design.check",
        Tree::parse(br#"{"design":"quad-a"}"#).unwrap(),
    );
    let cx = DispatchCx::new();
    let (_, env_in) = client_inproc
        .submit(&cx, &req)
        .unwrap()
        .into_parts()
        .unwrap();
    let (_, env_sock) = socket.submit(&cx, &req).unwrap().into_parts().unwrap();
    let m_in = mask_envelope_for_parity(&env_in).unwrap();
    let m_sock = mask_envelope_for_parity(&env_sock).unwrap();

    assert_ne!(
        m_in.as_str(),
        m_sock.as_str(),
        "negative control: handler reading process-global state must diverge across back ends"
    );
    let diff = first_diff_pointer(
        &Tree::parse(m_in.as_bytes()).unwrap(),
        &Tree::parse(m_sock.as_bytes()).unwrap(),
    );
    assert_eq!(
        diff.as_ref().map(|(p, _, _)| p.as_str()),
        Some("/result/process_global_origin")
    );

    let _stats = server.shutdown().expect("shutdown server");
}

/// Negative control: synthetic step-granular `run` verb (`sim.step`) fails granularity lint.
#[test]
fn negative_control_step_granular_verb_fails_lint() {
    let step_entry = VerbEntry {
        name: "sim.step",
        cluster: "sim",
        kind: Kind::Run,
        tier: Tier::S,
        capabilities: &[Scope("sim:run")],
        input_schema: SchemaId("frob.sim.step.in/1"),
        output_schema: SchemaId("frob.sim.step.out/1"),
        refusals: &[frob_core::refusal::RefusalCode::FrobRef802],
        idempotent: true,
        cost_model: "single step",
        first_milestone: Milestone::M0,
        task_clusters: &["sim"],
        examples: &["frob sim step quad-a --json"],
        purpose: "Planted single-step verb.",
        use_when: "Negative control.",
        owner: "frob-sim",
        wait: Some(WaitDefault {
            cli: "to-completion",
            mcp_ms: 5_000,
            mcp_max_ms: 30_000,
        }),
        actions: &[],
        schema_version: 1,
        paginated: false,
        exports: false,
        estimable: false,
        destructive: false,
        public: false,
        key_optional: false,
        option_capabilities: &[],
        spends_physics: Some(true),
        released: false,
        params: &[],
        conformance_only: &[],
    };
    let schema =
        r#"{"type":"object","required":["design"],"properties":{"design":{"type":"string"}}}"#;
    assert!(
        check_control_plane_granularity(&step_entry, schema).is_err(),
        "step-granular verb must fail granularity lint"
    );
}

/// Under `--no-default-features --features registry-only`, `Socket` is exposed (and
/// `InProcess` is absent) and runs cleanly against a real `rpc::serve` server.
#[cfg(not(feature = "handlers"))]
#[test]
fn registry_only_exposes_socket_without_in_process() {
    let refusals = synthetic_refusals();
    let sock = unique_socket_path("regonly");
    let server =
        RpcServerHandle::spawn_current_uid(&sock, synthetic_dispatch_service(), refusals.clone())
            .expect("spawn server");
    let socket = Socket::new(DaemonClient::new(&sock, refusals).expect("client"))
        .with_local_pipeline(synthetic_pipeline());
    let req = Request::new(
        "design.check",
        Tree::parse(br#"{"design":"quad-a"}"#).unwrap(),
    );
    let (_p, env) = socket
        .submit(&DispatchCx::new(), &req)
        .expect("socket call under registry-only")
        .into_parts()
        .expect("envelope");
    assert_eq!(env.exit(), ExitCode::Completed);
    let _stats = server.shutdown().expect("shutdown server");
}
