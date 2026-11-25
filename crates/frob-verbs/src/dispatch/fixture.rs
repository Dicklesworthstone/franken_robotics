//! `# synthetic` dispatch conformance fixture shared by unit tests,
//! `tests/dispatch_conformance.rs`, and the companion binaries
//! `examples/frob_rpc_serve.rs` / `examples/frob_rpc_call.rs` (bead N32a-04).
//!
//! Contains one `read` (`design.check`), one `mutate` (`design.edit`), one CPU-tier `run`
//! (`sim.run`), one device-tier `run` (`sim.gpu`), one `forbidden_keys` verb
//! (`export.packet`), one unimplemented verb (`unimpl.verb` -> 724), and one entry raising
//! a refusal for each exit code available at M0 (`3`, `4`, `5`, `6`, `9`).
//!
//! Works under BOTH default `handlers` and `--no-default-features --features registry-only`
//! (ADR-TETHER-02).

use std::collections::BTreeMap;
use std::sync::Arc;

use frob_core::evidence::{
    ClaimKind, ClaimProduct, DetClass, FlagSet, InputRef, RealityScope, Scope as EvScope, TierMeet,
};
use frob_core::id::hash_blob;
use frob_core::refusal::{Refusal, RefusalCode, RefusalFingerprint, Repair, RetryPolicy};
use frob_core::registry::refusals::{EMBEDDED_REFUSALS_TOML, RefusalRegistry};

use crate::envelope::{
    Bound, Cost, Envelope, EvidenceBlock, JsonBody, Progress, ProgressReport, RefusalFields, Tree,
};
use crate::registry::{Kind, Milestone, ParamKind, SchemaId, Scope, Tier, VerbEntry, WaitDefault};

use super::pipeline::{InputSchema, NameResolver, Pipeline};
use super::{
    Dispatch, DispatchCx, DispatchError, Frame, JobRef, JobStream, ProgressFrame, Request,
    ResolvedRequest, RunTier, WireEnvelope,
};

#[cfg(feature = "handlers")]
use super::in_process::{FnHandler, HandlerContext, HandlerTable, InProcess, ProgressEmit};

/// Synthetic extension of `registries/refusals.toml` adding `FROB-REF-601` (pending N21-REF)
/// so exit class 6 and `refusal_601_no_daemon` are exercised deterministically at M0.
const SYNTHETIC_REFUSAL_601_ROW: &str = r#"
[[refusal]]
code = "FROB-REF-601"
meaning = "synthetic: backend or device unavailable (no frobd answering; pointer daemon; due M0, pending N21-REF)"
defined_in = "§21.11"
exit = 6
retry = "backoff"
repairs = ["frobd --state-root ~/.frob", "Use a CPU-tier run"]
raised_by = ["N32a-04"]
"#;

/// Load `registries/refusals.toml` augmented with synthetic `FROB-REF-601` (if N21-REF has
/// not entered 601 yet).
#[must_use]
pub fn synthetic_refusals() -> RefusalRegistry {
    let base = RefusalRegistry::load().expect("embedded registries/refusals.toml must load");
    if base.lookup("FROB-REF-601", "fixture").is_ok() {
        return base;
    }
    let combined = format!("{EMBEDDED_REFUSALS_TOML}\n{SYNTHETIC_REFUSAL_601_ROW}");
    RefusalRegistry::from_bytes(combined.as_bytes()).expect("synthetic refusal registry must load")
}

/// Synthetic immutable identifier for mutable design name `"quad-a"`.
pub const QUAD_A_ID: &str = "d:41aa000000000000000000000000000000000000000000000000000000000001";

const DESIGN_CHECK_PARAMS: &[(&str, ParamKind)] = &[("scene", ParamKind::ArtifactInput("scene"))];

const fn mk_entry(
    name: &'static str,
    cluster: &'static str,
    kind: Kind,
    caps: &'static [Scope],
    in_s: &'static str,
    out_s: &'static str,
    refusals: &'static [RefusalCode],
    cost_model: &'static str,
    wait: Option<WaitDefault>,
    params: &'static [(&'static str, ParamKind)],
) -> VerbEntry {
    VerbEntry {
        name,
        cluster,
        kind,
        tier: Tier::S,
        capabilities: caps,
        input_schema: SchemaId(in_s),
        output_schema: SchemaId(out_s),
        refusals,
        idempotent: true,
        cost_model,
        first_milestone: Milestone::M0,
        task_clusters: &["author"],
        examples: &["frob doctor --json"],
        purpose: "Synthetic fixture verb for N32a-04 dispatch conformance.",
        use_when: "Synthetic test fixture.",
        owner: "frob-verbs",
        wait,
        actions: &[],
        schema_version: 1,
        paginated: false,
        exports: false,
        estimable: false,
        destructive: false,
        public: false,
        key_optional: false,
        option_capabilities: &[],
        spends_physics: None,
        released: false,
        params,
        conformance_only: &[],
    }
}

const RUN_WAIT: Option<WaitDefault> = Some(WaitDefault {
    cli: "to-completion",
    mcp_ms: 5_000,
    mcp_max_ms: 30_000,
});

/// Static synthetic verb entries covering every kind, tier, and M0 refusal exit class.
pub const SYNTHETIC_VERBS: &[VerbEntry] = &[
    mk_entry(
        "design.check",
        "design",
        Kind::Read,
        &[Scope("design:read")],
        "frob.design.check.in/1",
        "frob.design.check.out/1",
        &[RefusalCode::FrobRef802],
        "static",
        None,
        DESIGN_CHECK_PARAMS,
    ),
    mk_entry(
        "design.edit",
        "design",
        Kind::Mutate,
        &[Scope("design:write")],
        "frob.design.edit.in/1",
        "frob.design.edit.out/1",
        &[RefusalCode::FrobRef802, RefusalCode::FrobRef804],
        "static",
        None,
        &[],
    ),
    mk_entry(
        "export.packet",
        "export",
        Kind::Read,
        &[Scope("design:read")],
        "frob.export.packet.in/1",
        "frob.export.packet.out/1",
        &[RefusalCode::FrobRef801, RefusalCode::FrobRef802],
        "static",
        None,
        &[],
    ),
    mk_entry(
        "refuse.exit3",
        "refuse",
        Kind::Read,
        &[Scope("design:read")],
        "frob.refuse.in/1",
        "frob.refuse.out/1",
        &[RefusalCode::FrobRef802],
        "static",
        None,
        &[],
    ),
    mk_entry(
        "refuse.exit4",
        "refuse",
        Kind::Read,
        &[Scope("design:read")],
        "frob.refuse.in/1",
        "frob.refuse.out/1",
        &[RefusalCode::FrobRef701],
        "static",
        None,
        &[],
    ),
    mk_entry(
        "refuse.exit5",
        "refuse",
        Kind::Read,
        &[Scope("design:read")],
        "frob.refuse.in/1",
        "frob.refuse.out/1",
        &[RefusalCode::FrobRef724],
        "static",
        None,
        &[],
    ),
    mk_entry(
        "refuse.exit6",
        "refuse",
        Kind::Read,
        &[Scope("design:read")],
        "frob.refuse.in/1",
        "frob.refuse.out/1",
        &[RefusalCode::FrobRef802],
        "static",
        None,
        &[],
    ),
    mk_entry(
        "refuse.exit9",
        "refuse",
        Kind::Read,
        &[Scope("design:read")],
        "frob.refuse.in/1",
        "frob.refuse.out/1",
        &[RefusalCode::FrobRef810],
        "static",
        None,
        &[],
    ),
    mk_entry(
        "sim.gpu",
        "sim",
        Kind::Run,
        &[Scope("sim:run")],
        "frob.sim.gpu.in/1",
        "frob.sim.gpu.out/1",
        &[RefusalCode::FrobRef802],
        "device: bounded",
        RUN_WAIT,
        &[],
    ),
    mk_entry(
        "sim.run",
        "sim",
        Kind::Run,
        &[Scope("sim:run")],
        "frob.sim.run.in/1",
        "frob.sim.run.out/1",
        &[RefusalCode::FrobRef802],
        "cpu: bounded",
        RUN_WAIT,
        &[],
    ),
    mk_entry(
        "unimpl.verb",
        "unimpl",
        Kind::Read,
        &[Scope("design:read")],
        "frob.unimpl.in/1",
        "frob.unimpl.out/1",
        &[RefusalCode::FrobRef724],
        "static",
        None,
        &[],
    ),
];

/// Return the raw JSON input schema text for a synthetic fixture verb.
#[must_use]
pub fn synthetic_schema_json(verb: &str) -> &'static str {
    match verb {
        "design.check" => {
            r#"{"type":"object","required":["design"],"additionalProperties":false,"properties":{"design":{"type":"string"},"scene":{"type":"string"}}}"#
        }
        "design.edit" => {
            r#"{"type":"object","required":["design","param","value"],"additionalProperties":false,"properties":{"design":{"type":"string"},"param":{"type":"string"},"value":{"type":"number"}}}"#
        }
        "export.packet" => {
            r#"{"type":"object","required":["artifact"],"additionalProperties":false,"forbidden_keys":{"FROB-REF-801":["released"]},"properties":{"artifact":{"type":"string"},"meta":{"type":"object"}}}"#
        }
        "sim.run" | "sim.gpu" => {
            r#"{"type":"object","required":["design","duration_s"],"additionalProperties":false,"properties":{"design":{"type":"string"},"duration_s":{"type":"number","minimum":0},"worlds":{"type":"integer","minimum":1,"default":1},"tier":{"enum":["cpu","referee","device","cuda"]}}}"#
        }
        _ => r#"{"type":"object","additionalProperties":false,"properties":{}}"#,
    }
}

/// Synthetic mutable-name resolver mapping `"quad-a"` -> [`QUAD_A_ID`].
#[derive(Debug, Default, Clone, Copy)]
pub struct SyntheticNameResolver;

impl NameResolver for SyntheticNameResolver {
    fn resolve_name(
        &self,
        _verb: &str,
        param: &str,
        value: &str,
    ) -> Result<Option<String>, Refusal> {
        if param == "design" && value == "quad-a" {
            Ok(Some(QUAD_A_ID.to_owned()))
        } else {
            Ok(None)
        }
    }
}

/// Build the synthetic [`Pipeline`] with all fixture verbs, input schemas, and name resolver.
#[must_use]
pub fn synthetic_pipeline() -> Pipeline {
    let refusals = synthetic_refusals();
    let mut schemas = BTreeMap::new();
    for entry in SYNTHETIC_VERBS {
        let compiled =
            InputSchema::compile_for(entry, synthetic_schema_json(entry.name), &refusals)
                .expect("static synthetic fixture schema must compile");
        schemas.insert(entry.name.to_owned(), compiled);
    }
    Pipeline::new(SYNTHETIC_VERBS.to_vec(), schemas, refusals)
        .with_name_resolver(Arc::new(SyntheticNameResolver))
        .with_run_tier("sim.run", RunTier::Cpu)
        .with_run_tier("sim.gpu", RunTier::Device)
}

fn synthetic_evidence(tier: TierMeet) -> EvidenceBlock {
    let product = ClaimProduct::estimated(
        ClaimKind::KnownAnswer,
        InputRef::new(hash_blob(b"synthetic-dispatch-fixture"), "/fixture"),
        EvScope::default(),
        tier,
        DetClass::D0Exact,
        RealityScope::NoClaim,
        FlagSet::new(),
    );
    EvidenceBlock::from_claim_product(&product, "m:synthetic-fixture").expect("valid evidence")
}

fn eval_fixture_verb(
    req: &ResolvedRequest,
    refusals: &RefusalRegistry,
) -> Result<(Vec<ProgressFrame>, Envelope), DispatchError> {
    let mut progress = Vec::new();
    let env = match req.verb.as_str() {
        "design.check" => {
            let design = req
                .args
                .get("design")
                .and_then(Tree::as_str)
                .unwrap_or("unknown");
            let body = JsonBody::from_json(
                format!(
                    r#"{{"closure":"closed","design":"{design}","digest":"{}","findings":[]}}"#,
                    req.digest.to_hex()
                )
                .as_bytes(),
            )
            .map_err(DispatchError::Envelope)?;
            Envelope::completed(
                "design.check",
                body,
                synthetic_evidence(TierMeet::Static),
                Cost::ZERO,
            )
            .map_err(DispatchError::Envelope)?
        }
        "design.edit" => {
            let design = req
                .args
                .get("design")
                .and_then(Tree::as_str)
                .unwrap_or("unknown");
            let param = req
                .args
                .get("param")
                .and_then(Tree::as_str)
                .unwrap_or("unknown");
            let body = JsonBody::from_json(
                format!(r#"{{"design":"{design}","edited":"{param}","status":"applied"}}"#)
                    .as_bytes(),
            )
            .map_err(DispatchError::Envelope)?;
            Envelope::completed(
                "design.edit",
                body,
                synthetic_evidence(TierMeet::Static),
                Cost::ZERO,
            )
            .map_err(DispatchError::Envelope)?
        }
        "export.packet" => {
            let artifact = req
                .args
                .get("artifact")
                .and_then(Tree::as_str)
                .unwrap_or("a:0");
            let body = JsonBody::from_json(
                format!(r#"{{"artifact":"{artifact}","status":"ready"}}"#).as_bytes(),
            )
            .map_err(DispatchError::Envelope)?;
            Envelope::completed(
                "export.packet",
                body,
                synthetic_evidence(TierMeet::Static),
                Cost::ZERO,
            )
            .map_err(DispatchError::Envelope)?
        }
        "sim.run" | "sim.gpu" => {
            let worlds = req.args.get("worlds").and_then(Tree::as_int53).unwrap_or(1);
            let job_id = if req.verb == "sim.gpu" {
                "j:syn-gpu-1"
            } else {
                "j:syn-cpu-1"
            };
            let total = u64::try_from(worlds.max(1)).unwrap_or(1);
            let p1 = ProgressFrame::new(
                job_id,
                "running",
                Progress::from_report(&ProgressReport {
                    done: 0,
                    total: Some(total),
                    unit: "worlds".to_owned(),
                })
                .map_err(DispatchError::Envelope)?,
            )
            .map_err(DispatchError::Envelope)?;
            let p2 = ProgressFrame::new(
                job_id,
                "running",
                Progress::from_report(&ProgressReport {
                    done: total,
                    total: Some(total),
                    unit: "worlds".to_owned(),
                })
                .map_err(DispatchError::Envelope)?,
            )
            .map_err(DispatchError::Envelope)?;
            progress.push(p1);
            progress.push(p2);
            let tier_label = match req.run_tier {
                RunTier::Cpu => "cpu",
                RunTier::Device => "device",
            };
            let body = JsonBody::from_json(
                format!(r#"{{"status":"done","tier":"{tier_label}","worlds":{worlds}}}"#)
                    .as_bytes(),
            )
            .map_err(DispatchError::Envelope)?;
            Envelope::completed(
                &req.verb,
                body,
                synthetic_evidence(TierMeet::Referee),
                Cost::ZERO,
            )
            .map_err(DispatchError::Envelope)?
        }
        "refuse.exit3" => Envelope::from_code_in(
            refusals,
            "refuse.exit3",
            "FROB-REF-802",
            RefusalFields {
                message: "synthetic exit 3 refusal".to_owned(),
                pointer: "/args".to_owned(),
                observed: "bad".to_owned(),
                required: "valid".to_owned(),
                unit: String::new(),
                detail: None,
                affected_claims: Vec::new(),
                retry: RetryPolicy::AfterChange,
                repairs: vec![Repair::new("frob schema frob.refuse.in/1")],
            },
            Cost::ZERO,
        )
        .map_err(DispatchError::Envelope)?,
        "refuse.exit4" => Envelope::from_code_in(
            refusals,
            "refuse.exit4",
            "FROB-REF-701",
            RefusalFields {
                message: "synthetic exit 4 budget refusal".to_owned(),
                pointer: "/budget/gpu_s".to_owned(),
                observed: "100".to_owned(),
                required: "50".to_owned(),
                unit: "s".to_owned(),
                detail: None,
                affected_claims: Vec::new(),
                retry: RetryPolicy::AfterChange,
                repairs: vec![Repair::new("Reduce job size")],
            },
            Cost::ZERO,
        )
        .map_err(DispatchError::Envelope)?,
        "refuse.exit5" => Envelope::from_code_in(
            refusals,
            "refuse.exit5",
            "FROB-REF-724",
            RefusalFields {
                message: "synthetic exit 5 capability refusal".to_owned(),
                pointer: "/verb".to_owned(),
                observed: "refuse.exit5".to_owned(),
                required: "implemented capability".to_owned(),
                unit: String::new(),
                detail: None,
                affected_claims: Vec::new(),
                retry: RetryPolicy::Never,
                repairs: vec![Repair::new("frob capabilities --json")],
            },
            Cost::ZERO,
        )
        .map_err(DispatchError::Envelope)?,
        "refuse.exit6" => Envelope::from_code_in_with_fingerprint(
            refusals,
            "refuse.exit6",
            "FROB-REF-601",
            RefusalFields {
                message: "synthetic exit 6 unavailable refusal".to_owned(),
                pointer: "daemon".to_owned(),
                observed: "offline".to_owned(),
                required: "online".to_owned(),
                unit: String::new(),
                detail: None,
                affected_claims: Vec::new(),
                retry: RetryPolicy::Backoff,
                repairs: vec![Repair::new("frobd --state-root ~/.frob")],
            },
            Some(RefusalFingerprint::new(
                "daemon",
                "synthetic-daemon-offline",
            )),
            Cost::ZERO,
        )
        .map_err(DispatchError::Envelope)?,
        "refuse.exit9" => Envelope::from_code_in(
            refusals,
            "refuse.exit9",
            "FROB-REF-810",
            RefusalFields {
                message: "synthetic exit 9 integrity refusal".to_owned(),
                pointer: "/chunk".to_owned(),
                observed: "hash-a".to_owned(),
                required: "hash-b".to_owned(),
                unit: "digest".to_owned(),
                detail: None,
                affected_claims: Vec::new(),
                retry: RetryPolicy::Never,
                repairs: vec![Repair::new("frob job find --verb refuse.exit9 --key k")],
            },
            Cost::ZERO,
        )
        .map_err(DispatchError::Envelope)?,
        other => {
            return Err(DispatchError::Protocol(format!(
                "no synthetic evaluator for `{other}`"
            )));
        }
    };
    Ok((progress, env))
}

/// Build the synthetic [`HandlerTable`] (all fixture verbs except `unimpl.verb`).
#[cfg(feature = "handlers")]
#[must_use]
pub fn synthetic_handler_table() -> HandlerTable {
    let mut table = HandlerTable::new();
    for entry in SYNTHETIC_VERBS {
        if entry.name == "unimpl.verb" {
            continue;
        }
        table.register(
            entry.name,
            Arc::new(FnHandler(
                |_cx: &DispatchCx,
                 req: &ResolvedRequest,
                 ctx: &HandlerContext<'_>,
                 sink: &mut dyn ProgressEmit| {
                    let (progress, env) = eval_fixture_verb(req, ctx.refusals)
                        .map_err(|e| crate::envelope::HandlerFault::Unmapped(e.to_string()))?;
                    for p in progress {
                        sink.emit(p);
                    }
                    Ok(env)
                },
            )),
        );
    }
    table
}

/// Build the synthetic [`InProcess`] back end (no daemon attached).
#[cfg(feature = "handlers")]
#[must_use]
pub fn synthetic_in_process() -> InProcess {
    InProcess::new(synthetic_pipeline(), synthetic_handler_table())
}

/// Pure `Dispatch` server implementation of the synthetic fixture that compiles under BOTH
/// `handlers` AND `--no-default-features --features registry-only` (acts as the simulated
/// `frobd` daemon inside `rpc::serve` so `sim.gpu` executes cleanly on the daemon!).
pub struct SyntheticDaemonService {
    pipeline: Pipeline,
}

impl SyntheticDaemonService {
    #[must_use]
    pub fn new() -> Self {
        Self {
            pipeline: synthetic_pipeline(),
        }
    }
}

impl Default for SyntheticDaemonService {
    fn default() -> Self {
        Self::new()
    }
}

impl Dispatch for SyntheticDaemonService {
    fn submit(&self, cx: &DispatchCx, req: &Request) -> Result<JobStream, DispatchError> {
        cx.checkpoint()?;
        let resolved = match self.pipeline.prepare(req)? {
            Ok(r) => r,
            Err(refusal_env) => return Ok(JobStream::single_envelope(refusal_env)),
        };
        if resolved.verb == "unimpl.verb" {
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
        }
        let refusals = self.pipeline.refusals();
        let (progress, mut env) = eval_fixture_verb(&resolved, refusals)?;
        if resolved.bound != Bound::new() {
            env = env.with_bound(resolved.bound.clone());
        }
        let wire = WireEnvelope::from_envelope(&env, refusals)?;
        let mut frames: Vec<Frame> = progress.into_iter().map(Frame::Progress).collect();
        frames.push(Frame::Envelope(wire));
        JobStream::from_frames(frames, None)
    }

    fn attach(&self, cx: &DispatchCx, job: &JobRef) -> Result<JobStream, DispatchError> {
        cx.checkpoint()?;
        let label = match job {
            JobRef::Id(id) => id.clone(),
            JobRef::Key {
                verb,
                idempotency_key,
            } => format!("{verb}:{idempotency_key}"),
        };
        let body = JsonBody::from_json(
            format!(r#"{{"attached":"{label}","state":"completed"}}"#).as_bytes(),
        )
        .map_err(DispatchError::Envelope)?;
        let env = Envelope::completed(
            "job.wait",
            body,
            synthetic_evidence(TierMeet::Static),
            Cost::ZERO,
        )
        .map_err(DispatchError::Envelope)?;
        Ok(JobStream::single_envelope(WireEnvelope::from_envelope(
            &env,
            self.pipeline.refusals(),
        )?))
    }
}

/// Construct the shared synthetic daemon service (`Arc<dyn Dispatch>`) for `rpc::serve`.
#[must_use]
pub fn synthetic_dispatch_service() -> Arc<dyn Dispatch> {
    Arc::new(SyntheticDaemonService::new())
}

/// Every entry of the N32a-04 conformance fixture: `(case_name, request, expected_exit)`.
///
/// Covers `read`, `mutate`, CPU-tier `run`, device-tier `run` (over daemon), `forbidden_keys`,
/// unimplemented `724`, and refusals for exits `3`, `4`, `5`, `6`, `9`.
#[must_use]
pub fn parity_fixture_calls() -> Vec<(&'static str, Request, u8)> {
    vec![
        (
            "read_design_check",
            Request::new(
                "design.check",
                Tree::parse(br#"{"design":"quad-a"}"#).unwrap(),
            ),
            0,
        ),
        (
            "mutate_design_edit",
            Request::new(
                "design.edit",
                Tree::parse(br#"{"design":"quad-a","param":"knee.ratio","value":1.9}"#).unwrap(),
            ),
            0,
        ),
        (
            "run_cpu_sim_run",
            Request::new(
                "sim.run",
                Tree::parse(br#"{"design":"quad-a","duration_s":2.0,"worlds":4,"tier":"cpu"}"#)
                    .unwrap(),
            ),
            0,
        ),
        (
            "run_device_sim_gpu",
            Request::new(
                "sim.gpu",
                Tree::parse(br#"{"design":"quad-a","duration_s":1.5,"worlds":2,"tier":"device"}"#)
                    .unwrap(),
            ),
            0,
        ),
        (
            "forbidden_key_export",
            Request::new(
                "export.packet",
                Tree::parse(br#"{"artifact":"a:1","meta":{"released":true}}"#).unwrap(),
            ),
            3,
        ),
        (
            "unimplemented_724",
            Request::new("unimpl.verb", Tree::parse(b"{}").unwrap()),
            5,
        ),
        (
            "refuse_exit_3",
            Request::new("refuse.exit3", Tree::parse(b"{}").unwrap()),
            3,
        ),
        (
            "refuse_exit_4",
            Request::new("refuse.exit4", Tree::parse(b"{}").unwrap()),
            4,
        ),
        (
            "refuse_exit_5",
            Request::new("refuse.exit5", Tree::parse(b"{}").unwrap()),
            5,
        ),
        (
            "refuse_exit_6",
            Request::new("refuse.exit6", Tree::parse(b"{}").unwrap()),
            6,
        ),
        (
            "refuse_exit_9",
            Request::new("refuse.exit9", Tree::parse(b"{}").unwrap()),
            9,
        ),
    ]
}
