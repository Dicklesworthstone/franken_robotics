//! M0 catalog referee run (BD-024).

use std::sync::Arc;

use frob_core::refusal::{Repair, RetryPolicy};
use frob_fsim::Group;

use crate::dispatch::{HandlerTable, in_process::FnHandler};
use crate::envelope::{HandlerFault, RefusalFields};
use crate::operator::argument_fields;

pub(super) fn register(table: &mut HandlerTable) {
    table.register("sim.run", Arc::new(FnHandler(run)));
}

fn refused(code: &str, fields: RefusalFields) -> HandlerFault {
    HandlerFault::Refused {
        code: code.into(),
        fields,
    }
}

fn unavailable(parameter: &str, observed: &str, required: &str) -> HandlerFault {
    let mut fields = argument_fields(parameter, "not implemented in this build");
    fields.observed = observed.into();
    fields.required = required.into();
    fields.retry = RetryPolicy::Never;
    fields.repairs = vec![Repair::new("Use frob sim run g1 --tier referee --worlds 1")];
    refused("FROB-REF-724", fields)
}

fn run(
    cx: &crate::dispatch::DispatchCx,
    req: &crate::dispatch::ResolvedRequest,
    _: &crate::dispatch::HandlerContext<'_>,
    _: &mut dyn crate::dispatch::ProgressEmit,
) -> Result<crate::envelope::Envelope, HandlerFault> {
    let string = |name| req.args.get(name).and_then(crate::envelope::Tree::as_str);
    for (field, supported) in [("tier", "referee"), ("design", "g1")] {
        let supplied = string(field).unwrap_or_default();
        if supplied != supported {
            return Err(unavailable(field, supplied, supported));
        }
    }
    if let Some(worlds) = req
        .args
        .get("worlds")
        .and_then(crate::envelope::Tree::as_int53)
        && worlds != 1
    {
        return Err(unavailable("worlds", &worlds.to_string(), "1"));
    }
    if req.common.estimate_only {
        return Err(unavailable("estimate_only", "true", "false"));
    }
    frob_fsim::require(&[Group::Referee])
        .map_err(|r| refused("FROB-REF-790", RefusalFields::of(&r)))?;
    #[cfg(feature = "referee")]
    return referee::execute(cx, req);
    #[cfg(not(feature = "referee"))]
    {
        let _ = cx;
        Err(refused(
            "FROB-REF-790",
            RefusalFields::of(&frob_fsim::group_absent_refusal(Group::Referee)),
        ))
    }
}

#[cfg(feature = "referee")]
mod referee {
    #[cfg(test)]
    mod tests {
        include!("sim_run_tests.rs");
    }

    use std::path::PathBuf;

    use frob_chronicle::m0::{
        self, CHUNK_STEPS, DEFAULT_STEPS, Manifest, TICKS_PER_SECOND, TableId,
    };
    use frob_core::evidence::{
        ClaimKind, ClaimProduct, DetClass, FlagSet, InputRef, RealityScope, Scope, TierMeet,
    };
    use frob_core::id::{Blake3, hash_blob};
    use frob_core::json::INT53_MAX;
    use frob_core::registry::toml::{self, Table};
    use frob_fsim::referee::{
        CatalogRobot, CoordEntry, ModelFacts, Rational, RefereeDynamics, STANDARD_GRAVITY_M_S2,
        digest, new_referee_dynamics,
    };

    use super::{HandlerFault, argument_fields, refused};
    use crate::dispatch::{DispatchCx, ResolvedRequest};
    use crate::envelope::{Cost, Envelope, EvidenceBlock, JsonBody, Tree};

    const LOCK: &[u8] = include_bytes!("../../../../franken-stack.lock");
    const ORACLE: &[u8] =
        include_bytes!("../../../../tests/fixtures/oracle/fsim/g1_fd_digest.toml");

    fn unmapped(error: impl ToString) -> HandlerFault {
        HandlerFault::Unmapped(error.to_string())
    }

    fn bad_input(field: &str, value: &str, requirement: &str) -> HandlerFault {
        let mut fields = argument_fields(field, requirement);
        fields.observed = value.into();
        fields.required = requirement.into();
        refused("FROB-REF-802", fields)
    }

    pub(super) fn whole_steps(seconds: &str) -> Option<i64> {
        let (numerator, denominator): (u128, u128) = if let Some((n, d)) = seconds.split_once('/') {
            (n.parse().ok()?, d.parse().ok()?)
        } else if let Some((n, decimal)) = seconds.split_once('.') {
            let decimal = decimal.trim_end_matches('0');
            let denominator = 10_u128.checked_pow(decimal.len().try_into().ok()?)?;
            (
                n.parse::<u128>()
                    .ok()?
                    .checked_mul(denominator)?
                    .checked_add(if decimal.is_empty() {
                        0
                    } else {
                        decimal.parse().ok()?
                    })?,
                denominator,
            )
        } else {
            (seconds.parse().ok()?, 1)
        };
        let factor = gcd(denominator, TICKS_PER_SECOND as u128);
        let ticks_denominator = denominator / factor;
        if numerator == 0 || ticks_denominator == 0 || !numerator.is_multiple_of(ticks_denominator)
        {
            return None;
        }
        let ticks =
            (numerator / ticks_denominator).checked_mul(TICKS_PER_SECOND as u128 / factor)?;
        (ticks <= INT53_MAX as u128).then_some(ticks as i64)
    }

    fn gcd(mut a: u128, mut b: u128) -> u128 {
        while b != 0 {
            (a, b) = (b, a % b);
        }
        a
    }

    fn object<const N: usize>(members: [(&str, Tree); N]) -> Tree {
        Tree::Obj(members.into_iter().map(|(k, v)| (k.into(), v)).collect())
    }

    fn text(s: impl ToString) -> Tree {
        Tree::Str(s.to_string())
    }
    fn num(n: impl ToString) -> Tree {
        Tree::Num(n.to_string())
    }

    fn model_identity(facts: &ModelFacts) -> Result<String, HandlerFault> {
        fn coords(entries: &[CoordEntry]) -> Tree {
            Tree::Arr(
                entries
                    .iter()
                    .map(|e| {
                        object([
                            ("index", num(e.index)),
                            ("name", text(&e.name)),
                            ("kind", text(e.kind.as_str())),
                            ("unit", text(e.unit)),
                        ])
                    })
                    .collect(),
            )
        }
        let description = object([
            ("catalog", text("g1")),
            ("links", num(facts.links)),
            ("nq", num(facts.nq)),
            ("nv", num(facts.nv)),
            ("mass_kg", num(facts.mass_kg)),
            ("positions", coords(&facts.layout.position)),
            ("velocities", coords(&facts.layout.velocity)),
            (
                "orientation",
                text(facts.layout.orientation_parameterization),
            ),
            (
                "actuators",
                Tree::Arr(facts.layout.actuators.iter().map(text).collect()),
            ),
            (
                "link_names",
                Tree::Arr(facts.layout.link_names.iter().map(text).collect()),
            ),
        ]);
        Ok(format!(
            "m:{}",
            hash_blob(description.to_canonical().map_err(unmapped)?.as_bytes())
        ))
    }

    fn lock_pin(lock: &Table) -> Option<&str> {
        lock.get("foundation")?
            .as_array()?
            .iter()
            .filter_map(|entry| entry.value.as_table())
            .find(|entry| entry.get("id").and_then(toml::Value::as_str) == Some("FD-FSIM"))?
            .get("git_rev")?
            .as_str()
    }

    pub(super) fn compare_oracle<'a>(
        oracle: &'a Table,
        pin: &str,
        field: &str,
        actual: &str,
    ) -> (&'static str, Option<&'a str>) {
        let expected = oracle
            .get(field)
            .and_then(toml::Value::as_str)
            .filter(|hex| Blake3::from_hex(hex).is_ok())
            .filter(|_| oracle.get("pin").and_then(toml::Value::as_str) == Some(pin));
        match expected {
            Some(hex) if hex == actual => ("equal", expected),
            Some(_) => ("different", expected),
            None => ("no-fixture", None),
        }
    }

    fn answer(
        result: &Tree,
        seed: u64,
        model_id: &str,
        run_digest: Blake3,
    ) -> Result<Envelope, HandlerFault> {
        // N34-07 reserves Tested for the sanctioned evidence issuers.
        let claims = ClaimProduct::estimated(
            ClaimKind::KnownAnswer,
            InputRef::new(run_digest, "/result"),
            Scope::default(),
            TierMeet::Referee,
            DetClass::D2PerBackend,
            RealityScope::NoClaim,
            FlagSet::new(),
        );
        let evidence = EvidenceBlock::from_claim_product(&claims, model_id)
            .map_err(unmapped)?
            .with_seed(seed);
        let build = if result
            .get("findings")
            .and_then(Tree::as_array)
            .is_some_and(|r| !r.is_empty())
        {
            Envelope::completed_with_findings
        } else {
            Envelope::completed
        };
        build(
            "sim.run",
            JsonBody::from_term(result).map_err(unmapped)?,
            evidence,
            Cost::ZERO,
        )
        .map_err(unmapped)
    }

    fn replay(
        stored: &[u8],
        req: &ResolvedRequest,
        seed: u64,
        model_id: &str,
        run_digest: Blake3,
    ) -> Result<Envelope, HandlerFault> {
        let receipt = Tree::parse(stored).map_err(|e| unmapped(e.message))?;
        let expected = receipt
            .get("request_digest")
            .and_then(Tree::as_str)
            .ok_or_else(|| unmapped("stored request digest is missing"))?;
        if expected != req.digest.to_hex() {
            let mut fields =
                argument_fields("idempotency_key", "key already binds a different request");
            fields.observed = req.digest.to_hex();
            fields.required = expected.into();
            return Err(refused("FROB-REF-803", fields));
        }
        let result = receipt
            .get("result")
            .ok_or_else(|| unmapped("stored result is missing"))?;
        let original_run = result
            .get("run")
            .and_then(Tree::as_str)
            .ok_or_else(|| unmapped("stored run ID is missing"))?;
        if original_run.strip_prefix("r:") != Some(run_digest.to_hex().as_str()) {
            return Err(unmapped(
                "the stored request belongs to a different stack or catalog identity",
            ));
        }
        Ok(answer(result, seed, model_id, run_digest)?.replayed())
    }

    pub(super) fn chunk_ranges(steps: i64) -> impl Iterator<Item = (i64, i64)> {
        (0..steps)
            .step_by(CHUNK_STEPS as usize)
            .map(move |start| (start, (start + CHUNK_STEPS).min(steps)))
    }

    pub(super) fn first_mismatch(stored: &[Tree], actual: &[Tree]) -> Option<Tree> {
        (0..stored.len().max(actual.len())).find_map(|index| {
            let digest = |chunks: &[Tree]| {
                chunks
                    .get(index)
                    .and_then(|c| c.get("state_digest"))
                    .cloned()
                    .unwrap_or(Tree::Null)
            };
            let expected = digest(stored);
            let observed = digest(actual);
            (expected != observed).then(|| {
                object([
                    ("code", text("digest-mismatch")),
                    ("chunk", num(index)),
                    ("expected_digest", expected),
                    ("observed_digest", observed),
                ])
            })
        })
    }

    pub(super) fn execute(
        cx: &DispatchCx,
        req: &ResolvedRequest,
    ) -> Result<Envelope, HandlerFault> {
        let path = std::env::var_os("FROB_STORE")
            .map(PathBuf::from)
            .unwrap_or_else(|| "chronicle/ledger.db".into());
        execute_at(cx, req, &path, || {})
    }

    pub(super) fn execute_at(
        cx: &DispatchCx,
        req: &ResolvedRequest,
        path: &std::path::Path,
        after_chunk: impl FnMut(),
    ) -> Result<Envelope, HandlerFault> {
        let oracle = toml::parse(ORACLE).unwrap_or_default();
        execute_with_oracle(cx, req, path, &oracle, after_chunk)
    }

    fn execute_with_oracle(
        cx: &DispatchCx,
        req: &ResolvedRequest,
        path: &std::path::Path,
        oracle: &Table,
        mut after_chunk: impl FnMut(),
    ) -> Result<Envelope, HandlerFault> {
        let string = |name| req.args.get(name).and_then(Tree::as_str);
        let seconds = string("seconds").unwrap_or("2");
        let steps = whole_steps(seconds).ok_or_else(|| {
            bad_input(
                "seconds",
                seconds,
                "positive seconds with a whole int53 step count at 480 Hz",
            )
        })?;
        let seed = string("seed").unwrap_or("0");
        let seed = seed
            .parse()
            .map_err(|_| bad_input("seed", seed, "a decimal u64 string"))?;
        let dyns = new_referee_dynamics().map_err(unmapped)?;
        let model = dyns.catalog_model(CatalogRobot::G1).map_err(unmapped)?;
        let facts = dyns.describe(&model);
        let model_id = model_identity(&facts)?;
        let lock = toml::parse(LOCK).map_err(unmapped)?;
        let pin =
            lock_pin(&lock).ok_or_else(|| unmapped("FrankenSim pin missing from stack lock"))?;
        let manifest = Manifest {
            model_id: &model_id,
            foundation_pin: pin,
            steps,
            seed,
            perturbed: cfg!(feature = "conformance") && string("perturb_ulp") == Some("q0"),
        };
        let run_digest = manifest.digest().map_err(unmapped)?;
        let run_id = format!("r:{run_digest}");
        let request_key = req
            .common
            .idempotency_key
            .as_deref()
            .map(|key| m0::request_key(&req.principal, key))
            .transpose()
            .map_err(unmapped)?;
        let mut ledger = m0::open(path).map_err(unmapped)?;
        if let Some(key) = &request_key
            && let Some(stored) = ledger
                .get_committed(TableId::Meta, key.as_bytes())
                .map_err(unmapped)?
        {
            return replay(&stored, req, seed, &model_id, run_digest);
        }

        let start = m0::unix_ms();
        let wall_start = std::time::Instant::now();
        let trace = m0::new_trace(start).map_err(unmapped)?;
        let mut state = dyns.reference_state(&model);
        #[cfg(feature = "conformance")]
        if manifest.perturbed {
            let q0 = facts
                .layout
                .position
                .iter()
                .find(|e| e.kind == frob_fsim::referee::CoordKind::RevoluteJoint)
                .ok_or_else(|| unmapped("G1 has no revolute joint"))?
                .index as usize;
            state.q[q0] = state.q[q0].next_up();
        }
        let tau = vec![0.0; facts.actuators as usize];
        let accel = dyns
            .forward_dynamics(&model, &state, &tau)
            .map_err(unmapped)?;
        let fd = digest::forward_dynamics_digest_hex(facts.links, &state.q, &state.v, &accel);
        let (initial_z, _) = dyns
            .vertical_observables(&model, &state)
            .map_err(unmapped)?;
        let mut chunks = Vec::new();
        let mut completed_steps = 0;
        let mut final_digest = String::new();
        for (index, (begin, end)) in chunk_ranges(steps).enumerate() {
            for _ in begin..end {
                state = dyns
                    .step_free(&model, &state, &tau, Rational::ONE_OVER_480)
                    .map_err(unmapped)?;
            }
            completed_steps = end;
            final_digest = digest::state_digest_hex(facts.links, &state.q, &state.v);
            chunks.push(object([
                ("index", num(index)),
                ("tick_range", Tree::Arr(vec![num(begin), num(end)])),
                ("state_digest", text(&final_digest)),
            ]));
            after_chunk();
            if cx.is_cancelled() {
                break;
            }
        }
        let cancelled = cx.is_cancelled();
        let end = m0::unix_ms();
        let (final_z, momentum) = dyns
            .vertical_observables(&model, &state)
            .map_err(unmapped)?;
        let (oracle_match, oracle_fd) = compare_oracle(oracle, pin, "fd_digest", &fd);
        let (rollout_match, oracle_final) = if !cancelled && completed_steps == DEFAULT_STEPS {
            compare_oracle(oracle, pin, "rollout_2s_digest", &final_digest)
        } else {
            ("no-fixture", None)
        };
        let optional_text = |s: Option<&str>| s.map(text).unwrap_or(Tree::Null);
        let mut findings = Vec::new();
        if oracle_match != "equal" {
            findings.push(object([
                ("code", text("oracle-match")),
                ("match", text(oracle_match)),
            ]));
        }
        if cancelled {
            findings.push(object([
                ("code", text("cancelled")),
                ("last_completed_chunk", num(chunks.len() - 1)),
            ]));
        } else if rollout_match != "equal" {
            findings.push(object([
                ("code", text("rollout-match")),
                ("match", text(rollout_match)),
            ]));
        }
        let ticks = gcd(completed_steps as u128, TICKS_PER_SECOND as u128) as i64;
        let sim_seconds = Tree::Arr(vec![
            num(completed_steps / ticks),
            num(TICKS_PER_SECOND / ticks),
        ]);
        let elapsed = wall_start.elapsed().as_secs_f64();
        let t0 = object([
            ("links", num(facts.links)),
            ("nq", num(facts.nq)),
            ("nv", num(facts.nv)),
            ("mass_kg", num(facts.mass_kg)),
            ("gravity_m_s2", num(STANDARD_GRAVITY_M_S2)),
            ("com_drop_m", num(final_z - initial_z)),
            ("vertical_momentum_kg_m_s", num(momentum)),
            ("forward_dynamics_digest", text(&fd)),
        ]);
        let mut argv = [
            "frob",
            "sim",
            "run",
            "g1",
            "--tier",
            "referee",
            "--seconds",
            seconds,
            "--seed",
        ]
        .into_iter()
        .map(text)
        .collect::<Vec<_>>();
        argv.extend([text(seed), text("--worlds"), text("1"), text("--json")]);
        if manifest.perturbed {
            argv.extend([text("--perturb-ulp"), text("q0")]);
        }

        let mut txn = ledger.begin().map_err(unmapped)?;
        if let Some(key) = &request_key
            && let Some(stored) = txn.get(TableId::Meta, key.as_bytes()).map_err(unmapped)?
        {
            return replay(&stored, req, seed, &model_id, run_digest);
        }
        let episode_key = m0::episode_key(&run_id);
        let previous = txn
            .get(TableId::Episode, episode_key.as_bytes())
            .map_err(unmapped)?;
        let mut reproduced = false;
        let mut state = if cancelled { "cancelled" } else { "completed" };
        if !cancelled && let Some(bytes) = &previous {
            let episode = Tree::parse(bytes).map_err(|e| unmapped(e.message))?;
            let stored_chunks = episode
                .get("chunks")
                .and_then(Tree::as_array)
                .ok_or_else(|| unmapped("stored episode has no chunks"))?;
            if let Some(finding) = first_mismatch(stored_chunks, &chunks) {
                state = "determinism-violated";
                findings.push(finding);
            } else {
                reproduced = true;
            }
        }
        let write_run = previous.is_none() || state == "determinism-violated";
        if write_run {
            let row = manifest.run_row(&run_id, state).map_err(unmapped)?;
            txn.put(TableId::Run, run_id.as_bytes(), row.as_bytes())
                .map_err(unmapped)?;
        }
        let chunks = Tree::Arr(chunks);
        if !cancelled && previous.is_none() {
            let episode = object([
                ("run_id", text(&run_id)),
                ("world", num(0)),
                ("episode", num(0)),
                ("steps", num(completed_steps)),
                ("seed", text(seed)),
                ("sim_seconds", sim_seconds.clone()),
                ("chunks", chunks.clone()),
                ("final_state_digest", text(&final_digest)),
                ("outcome", text("completed")),
                ("t0", t0.clone()),
            ])
            .to_canonical()
            .map_err(unmapped)?;
            txn.put(TableId::Episode, episode_key.as_bytes(), episode.as_bytes())
                .map_err(unmapped)?;
        }
        let result = object([
            ("run", text(&run_id)),
            ("trace", text(trace)),
            ("final_state_digest", text(final_digest)),
            ("chunks", chunks),
            ("steps", num(completed_steps)),
            ("sim_seconds", sim_seconds),
            ("oracle_match", text(oracle_match)),
            ("rollout_match", text(rollout_match)),
            ("oracle_fd_digest", optional_text(oracle_fd)),
            ("oracle_rollout_digest", optional_text(oracle_final)),
            ("oracle_pin", text(pin)),
            (
                "outcome",
                text(if cancelled { "cancelled" } else { "completed" }),
            ),
            ("reproduced", Tree::Bool(reproduced)),
            ("recorded_command", Tree::Arr(argv)),
            ("execution_authorized", Tree::Bool(false)),
            ("hardware_safety_signoff", Tree::Bool(false)),
            ("sim_to_real_certified", Tree::Bool(false)),
            ("t0", t0),
            (
                "wall_s_per_sim_s",
                num(elapsed * TICKS_PER_SECOND as f64 / completed_steps as f64),
            ),
            ("findings", Tree::Arr(findings)),
        ]);
        let envelope = answer(&result, seed, &model_id, run_digest)?;
        if let Some(key) = request_key {
            let receipt = object([("request_digest", text(req.digest)), ("result", result)])
                .to_canonical()
                .map_err(unmapped)?;
            txn.put(TableId::Meta, key.as_bytes(), receipt.as_bytes())
                .map_err(unmapped)?;
        }
        let execution = m0::execution_row(&run_id, trace, end).map_err(unmapped)?;
        txn.put(
            TableId::Execution,
            trace.to_string().as_bytes(),
            execution.as_bytes(),
        )
        .map_err(unmapped)?;
        txn.commit().map_err(unmapped)?;
        Ok(envelope)
    }
}
