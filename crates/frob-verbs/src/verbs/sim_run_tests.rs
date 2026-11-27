use super::*;
use crate::dispatch::{CommonArgs, InputSchema, Pipeline, Request};
use crate::envelope::{Outcome, Schema, embedded_registry};
use crate::registry::{Kind, lookup, lookup_schema};

fn request(key: &str, seconds: &str) -> ResolvedRequest {
    let entry = lookup("sim.run").unwrap();
    let refusals = embedded_registry().unwrap();
    let raw = lookup_schema(entry.input_schema.as_str()).unwrap();
    let schema = entry.input_schema_with_common_fields_json(raw).unwrap();
    let schema = InputSchema::compile_for(entry, &schema, refusals).unwrap();
    let pipeline = Pipeline::new(
        vec![*entry],
        [("sim.run".to_owned(), schema)].into(),
        refusals.clone(),
    );
    let req = Request::new(
        "sim.run",
        object([
            ("design", text("g1")),
            ("tier", text("referee")),
            ("seconds", text(seconds)),
        ]),
    )
    .with_common(CommonArgs {
        idempotency_key: Some(key.into()),
        ..Default::default()
    });
    pipeline.prepare(&req).unwrap().unwrap()
}

fn store() -> PathBuf {
    let trace = m0::new_trace(m0::unix_ms()).unwrap();
    std::env::temp_dir()
        .join(format!("frob-bd024-{trace}"))
        .join("ledger.db")
}

fn result(envelope: &Envelope) -> &Tree {
    let result = match envelope.outcome() {
        Outcome::Completed { result, .. } => Some(result),
        _ => None,
    };
    result.expect("completed envelope").tree()
}

fn field<'a>(result: &'a Tree, name: &str) -> &'a str {
    result.get(name).and_then(Tree::as_str).unwrap()
}

#[test]
fn chunk_boundaries_m0() {
    assert_eq!(
        chunk_ranges(DEFAULT_STEPS).collect::<Vec<_>>(),
        [(0, 256), (256, 512), (512, 768), (768, 960)]
    );
}

#[test]
fn seconds_whole_steps() {
    for (s, expected) in [
        ("2", Some(960)),
        ("0.025", Some(12)),
        ("1/48", Some(10)),
        ("2.00000", Some(960)),
        ("1/480", Some(1)),
        ("600/480", Some(600)),
        ("0.01", None),
        ("0", None),
        ("1/0", None),
        ("9007199254740993", None),
    ] {
        assert_eq!(whole_steps(s), expected, "{s}");
    }
    let path = store();
    let refused = execute_at(&DispatchCx::new(), &request("k", "0.01"), &path, || {}).unwrap_err();
    let HandlerFault::Refused { code, fields } = refused else {
        panic!("{refused:?}")
    };
    assert_eq!(
        (code.as_str(), fields.pointer.as_str()),
        ("FROB-REF-802", "/seconds")
    );
    assert!(!path.exists());
}

#[test]
fn rollout_match_fail_closed() {
    let pin = lock_pin(&toml::parse(LOCK).unwrap()).unwrap().to_owned();
    let fixture = toml::parse(ORACLE).unwrap();
    let oracle_field = "rollout_2s_digest";
    let digest = fixture
        .get(oracle_field)
        .and_then(toml::Value::as_str)
        .unwrap();
    assert_eq!(
        compare_oracle(&fixture, &pin, oracle_field, digest),
        ("equal", Some(digest))
    );
    let missing = format!("pin = {pin:?}");
    let stale = format!("pin = \"different-pin\"\n{oracle_field} = {digest:?}");
    for raw in [missing, stale] {
        let fixture = toml::parse(raw.as_bytes()).unwrap();
        let envelope = execute_with_oracle(
            &DispatchCx::new(),
            &request("fixture", "2"),
            &store(),
            &fixture,
            || {},
        )
        .unwrap();
        assert_eq!(envelope.exit().code(), 2);
        assert_eq!(field(result(&envelope), "rollout_match"), "no-fixture");
    }
}

#[test]
fn registry_row_from_verbs_sim() {
    let row = lookup("sim.run").unwrap();
    assert_eq!(
        (row.kind, row.idempotent, row.owner),
        (Kind::Run, false, "frob-dyn")
    );
    assert!(
        row.examples
            .contains(&"frob sim run g1 --tier referee --seconds 2 --key run-m0 --json")
    );
    assert!(crate::handlers::handler_table().contains(row.name));
    assert_eq!(row.refusals, &[frob_core::refusal::RefusalCode::FrobRef790]);
}

#[test]
fn reexecution_same_run_id_new_trace() {
    let path = store();
    let cx = DispatchCx::new();
    let first = execute_at(&cx, &request("k-a", "2"), &path, || {}).unwrap();
    let second = execute_at(&cx, &request("k-b", "2"), &path, || {}).unwrap();
    let cached = execute_at(&cx, &request("k-a", "2"), &path, || {
        panic!("replay stepped physics")
    })
    .unwrap();
    let (first, second, old) = (result(&first), result(&second), result(&cached));
    for name in ["run", "chunks", "final_state_digest"] {
        assert_eq!(first.get(name), second.get(name), "{name}");
    }
    assert_eq!(old, first);
    assert_ne!(field(first, "trace"), field(second, "trace"));
    assert_eq!(second.get("reproduced").and_then(Tree::as_bool), Some(true));
    let argv = first
        .get("recorded_command")
        .and_then(Tree::as_array)
        .unwrap();
    assert!(argv.iter().all(|arg| arg.as_str() != Some("--key")));
    let ledger = m0::open(&path).unwrap();
    let run = field(first, "run");
    for (table, key) in [
        (TableId::Run, run.to_owned()),
        (TableId::Episode, m0::episode_key(run)),
        (TableId::Execution, field(first, "trace").to_owned()),
        (TableId::Execution, field(second, "trace").to_owned()),
    ] {
        assert!(
            ledger
                .get_committed(table, key.as_bytes())
                .unwrap()
                .is_some()
        );
    }
    assert_eq!(ledger.latest_commit_seq().unwrap().as_u64(), 2);
    let schema = lookup_schema("frob.sim.run.out/1").unwrap();
    Schema::compile(Tree::parse(schema.as_bytes()).unwrap())
        .unwrap()
        .validate(second)
        .unwrap();
    let conflict = execute_at(&cx, &request("k-a", "1/480"), &path, || {
        panic!("conflict stepped physics")
    });
    assert!(matches!(conflict, Err(HandlerFault::Refused {code, ..}) if code == "FROB-REF-803"));
}

#[test]
fn digest_mismatch_is_finding_not_overwrite() {
    let path = store();
    let cx = DispatchCx::new();
    let first = execute_at(&cx, &request("k-a", "1/480"), &path, || {}).unwrap();
    let run = field(result(&first), "run");
    let mut ledger = m0::open(&path).unwrap();
    let key = m0::episode_key(run);
    let original = ledger
        .get_committed(TableId::Episode, key.as_bytes())
        .unwrap()
        .unwrap();
    let original_digest = field(result(&first), "final_state_digest");
    let altered = String::from_utf8(original)
        .unwrap()
        .replace(original_digest, &"0".repeat(64));
    let mut txn = ledger.begin().unwrap();
    txn.put(TableId::Episode, key.as_bytes(), altered.as_bytes())
        .unwrap();
    txn.commit().unwrap();
    let second = execute_at(&cx, &request("k-b", "1/480"), &path, || {}).unwrap();
    assert_eq!(second.exit().code(), 2);
    let findings = result(&second)
        .get("findings")
        .and_then(Tree::as_array)
        .unwrap();
    let mismatch = findings
        .iter()
        .find(|f| field(f, "code") == "digest-mismatch")
        .unwrap();
    assert_eq!(mismatch.get("chunk").and_then(Tree::as_int53), Some(0));
    assert_eq!(field(mismatch, "expected_digest"), "0".repeat(64));
    assert_eq!(field(mismatch, "observed_digest"), original_digest);
    assert_eq!(
        ledger
            .get_committed(TableId::Episode, key.as_bytes())
            .unwrap()
            .unwrap(),
        altered.as_bytes()
    );
    let row = ledger
        .get_committed(TableId::Run, run.as_bytes())
        .unwrap()
        .unwrap();
    assert_eq!(
        field(&Tree::parse(&row).unwrap(), "state"),
        "determinism-violated"
    );
}

#[test]
fn cancel_between_chunks() {
    let path = store();
    let cx = DispatchCx::new();
    let mut chunks = 0;
    let envelope = execute_at(&cx, &request("cancel", "2"), &path, || {
        chunks += 1;
        if chunks == 2 {
            cx.cancel();
        }
    })
    .unwrap();
    assert_eq!(envelope.exit().code(), 2);
    let result = result(&envelope);
    assert_eq!(field(result, "outcome"), "cancelled");
    assert_eq!(result.get("steps").and_then(Tree::as_int53), Some(512));
    let findings = result.get("findings").and_then(Tree::as_array).unwrap();
    let cancellation = findings
        .iter()
        .find(|f| field(f, "code") == "cancelled")
        .unwrap();
    assert_eq!(
        cancellation
            .get("last_completed_chunk")
            .and_then(Tree::as_int53),
        Some(1)
    );
    let key = m0::episode_key(field(result, "run"));
    assert!(
        m0::open(&path)
            .unwrap()
            .get_committed(TableId::Episode, key.as_bytes())
            .unwrap()
            .is_none()
    );
}

#[test]
fn sanity_metrics_recorded() {
    let env = execute_at(
        &DispatchCx::new(),
        &request("metrics", "2"),
        &store(),
        || {},
    )
    .unwrap();
    let t0 = result(&env).get("t0").unwrap();
    let metric = |k| t0.get(k).and_then(Tree::as_f64).unwrap();
    let g = metric("gravity_m_s2");
    assert!((metric("com_drop_m") + 2.0 * g).abs() <= 2.0 * g / 480.0);
    assert!((metric("vertical_momentum_kg_m_s") + 2.0 * g * metric("mass_kg")).abs() < 0.01);
}
