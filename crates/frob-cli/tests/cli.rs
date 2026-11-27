#[path = "../src/grammar.rs"]
mod grammar;

use std::{
    process::{Command, Output},
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use frob_verbs::{
    dispatch::{
        Dispatch, DispatchCx, InProcess, InputSchema, Pipeline, Request, SessionAuth,
        fixture::SYNTHETIC_VERBS, rpc::RpcServerHandle,
    },
    envelope::{HandlerFault, Schema, Tree, check_stream, embedded_registry},
    operator::doctor::{DoctorSection, SectionReport},
    registry::{Action, Kind, Scope, VERBS, lookup},
};

fn call(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_frob"))
        .env_remove("FROB_DISPATCH")
        .env_remove("FROB_SESSION")
        .args(args)
        .output()
        .expect("run the built CLI")
}

fn checked(out: &Output, exit: u8) -> Tree {
    assert_eq!(out.status.code(), Some(i32::from(exit)), "{out:?}");
    assert!(out.stderr.is_empty(), "{out:?}");
    let answer = check_stream(&out.stdout).expect("exactly one validated terminal envelope");
    assert_eq!(answer.envelope.exit().code(), exit);
    answer.envelope.tree().clone()
}

fn sim_refusal(args: &[&str], exit: u8, code: &str, pointer: &str) {
    let store = std::env::temp_dir().join(format!(
        "frob-cli-bd024-{}-{}.db",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let output = Command::new(env!("CARGO_BIN_EXE_frob"))
        .env_clear()
        .env("FROB_STORE", &store)
        .args(["sim", "run", "g1", "--key", "refusal", "--json"])
        .args(args)
        .output()
        .unwrap();
    let envelope = checked(&output, exit);
    let refusal = envelope.get("refusal").unwrap();
    assert_eq!(refusal.get("code").and_then(Tree::as_str), Some(code));
    assert_eq!(refusal.get("pointer").and_then(Tree::as_str), Some(pointer));
    assert!(!store.exists());
}

#[test]
fn tier_not_implemented_724() {
    for tier in ["batch-cpu", "gpu-metal", "gpu-cuda"] {
        sim_refusal(&["--tier", tier], 5, "FROB-REF-724", "/tier");
    }
    sim_refusal(
        &["--tier", "referee", "--worlds", "2"],
        5,
        "FROB-REF-724",
        "/worlds",
    );
}

#[test]
fn tier_misspelt_802() {
    sim_refusal(&["--tier", "refere"], 3, "FROB-REF-802", "/tier");
}

#[test]
fn unknown_flag_802() {
    sim_refusal(
        &["--tier", "referee", "--fixture", "g1"],
        3,
        "FROB-REF-802",
        "/fixture",
    );
}

#[cfg(feature = "referee")]
#[test]
fn invalid_seconds_and_seed_802() {
    for seconds in ["0", "0.01"] {
        sim_refusal(
            &["--tier", "referee", "--seconds", seconds],
            3,
            "FROB-REF-802",
            "/seconds",
        );
    }
    sim_refusal(
        &["--tier", "referee", "--seed", "18446744073709551616"],
        3,
        "FROB-REF-802",
        "/seed",
    );
}

#[cfg(not(feature = "referee"))]
#[test]
fn missing_referee_group_790() {
    sim_refusal(
        &["--tier", "referee"],
        5,
        "FROB-REF-790",
        "foundation/referee/compiled",
    );
}

#[test]
fn doctor_reports_compiled_stack_receipt_and_lock() {
    let out = call(&["doctor", "--section", "stack_receipt", "--json"]);
    let envelope = checked(&out, 0);
    let result = envelope.get("result").unwrap();
    let stack = result.get("stack_receipt").unwrap();
    for (field, prefix, contents) in [
        ("receipt", "sr", include_str!("../../../receipts/HEAD")),
        ("lock", "sl", include_str!("../../../receipts/LOCK_DIGEST")),
    ] {
        let expected = format!("{prefix}:{}", contents.trim());
        assert_eq!(
            stack.get(field).and_then(Tree::as_str),
            Some(expected.as_str())
        );
    }
    assert_eq!(result.as_object().unwrap().len(), 1);
}

#[test]
fn refusal_802_unknown_flag_and_section() {
    for (args, pointer) in [
        (&["doctor", "--sektion", "stack"][..], "/sektion"),
        (&["doctor", "--section", "kernelz"][..], "/section"),
        (&["schema", "--refusals", "--verbs"][..], "/args"),
    ] {
        let out = call(&[args, &["--json"]].concat());
        let envelope = checked(&out, 3);
        let refusal = envelope.get("refusal").unwrap();
        assert_eq!(
            refusal.get("code").and_then(Tree::as_str),
            Some("FROB-REF-802")
        );
        assert_eq!(refusal.get("pointer").and_then(Tree::as_str), Some(pointer));
        assert_eq!(call(args).status.code(), Some(3));
    }
}

#[test]
fn refusal_801_unknown_and_empty_command() {
    for args in [&["dezign", "check"][..], &[][..]] {
        let out = call(&[args, &["--json"]].concat());
        let envelope = checked(&out, 3);
        let refusal = envelope.get("refusal").unwrap();
        assert_eq!(
            refusal.get("code").and_then(Tree::as_str),
            Some("FROB-REF-801")
        );
        if args.is_empty() {
            let actions = envelope
                .get("next_actions")
                .and_then(Tree::as_array)
                .unwrap();
            assert_eq!(
                actions[0].get("command").and_then(Tree::as_str),
                Some("frob robot-docs")
            );
        }
        assert_eq!(call(args).status.code(), Some(3));
    }
}

#[test]
fn schema_positional_verb_equals_flag() {
    let named = call(&["schema", "--verb", "doctor", "--out", "--json"]);
    let positional = call(&["schema", "doctor", "--out", "--json"]);
    assert_eq!(named.stdout, positional.stdout);
    let envelope = checked(&named, 0);
    let result = envelope.get("result").unwrap();
    assert_eq!(
        result
            .get("schema")
            .unwrap()
            .get("$id")
            .and_then(Tree::as_str),
        Some("frob.doctor.out/1")
    );
    for selector in ["frob.envelope/1", "doctor"] {
        checked(&call(&["schema", selector, "--json"]), 0);
    }
}

#[test]
fn schema_bare_equals_refusals() {
    let named = call(&["schema", "--refusals", "--json"]);
    let bare = call(&["--json", "schema"]);
    assert_eq!(named.stdout, bare.stdout);
    let result = checked(&bare, 0);
    let rows = result
        .get("result")
        .unwrap()
        .get("refusals")
        .and_then(Tree::as_array)
        .unwrap();
    assert_eq!(
        rows.len(),
        frob_verbs::envelope::embedded_registry()
            .unwrap()
            .rows()
            .len()
    );
}

#[test]
fn help_json_matches_derived_input_and_refusals() {
    let out = checked(&call(&["doctor", "--help", "--json"]), 0);
    let result = out.get("result").unwrap();
    let entry = lookup("doctor").unwrap();
    let raw = frob_verbs::registry::lookup_schema(entry.input_schema.as_str()).unwrap();
    let derived = entry.input_schema_with_common_fields_json(raw).unwrap();
    assert!(
        result
            .get("input_schema")
            .unwrap()
            .same_value(&Tree::parse(derived.as_bytes()).unwrap())
    );
    let reg = frob_verbs::envelope::embedded_registry().unwrap();
    let refusals = result.get("refusals").and_then(Tree::as_array).unwrap();
    let expected = entry.all_refusals_with(raw, reg);
    assert!(!expected.is_empty());
    assert_eq!(refusals.len(), expected.len());
    for (actual, expected) in refusals.iter().zip(expected) {
        assert_eq!(
            actual.get("code").and_then(Tree::as_str),
            Some(expected.code_str().as_str())
        );
        assert_eq!(
            actual.get("source").and_then(Tree::as_str),
            Some(expected.source.to_string().as_str())
        );
    }
}

#[test]
fn robot_docs_and_capabilities_cover_the_registry() {
    let docs = call(&["robot-docs", "--json"]);
    assert_eq!(docs.stdout, call(&["--json", "robot-docs"]).stdout);
    let docs = checked(&docs, 0);
    let capabilities = checked(&call(&["capabilities", "--json"]), 0);
    let docs = docs
        .get("result")
        .unwrap()
        .get("verbs")
        .and_then(Tree::as_array)
        .unwrap();
    let capabilities = capabilities
        .get("result")
        .unwrap()
        .get("capabilities")
        .and_then(Tree::as_array)
        .unwrap();
    assert_eq!(docs.len(), VERBS.len());
    assert_eq!(capabilities.len(), VERBS.len());
    for ((doc, cap), entry) in docs.iter().zip(capabilities).zip(&VERBS) {
        assert_eq!(
            doc.get("entry").unwrap().get("name").and_then(Tree::as_str),
            Some(entry.name)
        );
        assert_eq!(cap.get("verb").and_then(Tree::as_str), Some(entry.name));
        let linked = frob_verbs::handlers::handler_table().contains(entry.name);
        assert_eq!(cap.get("linked").and_then(Tree::as_bool), Some(linked));
        assert_eq!(
            cap.get("status").and_then(Tree::as_str),
            Some(if linked { "partial" } else { "not-in-build" })
        );
    }
}

#[test]
fn refusal_724_declared_session_and_available_help() {
    let refused = checked(&call(&["session", "show", "--json"]), 5);
    assert_eq!(
        refused
            .get("refusal")
            .unwrap()
            .get("code")
            .and_then(Tree::as_str),
        Some("FROB-REF-724")
    );
    let help = checked(&call(&["session", "open", "--help", "--json"]), 0);
    assert!(
        help.get("result")
            .unwrap()
            .get("input_schema")
            .unwrap()
            .get("properties")
            .unwrap()
            .get("idempotency_key")
            .is_some()
    );
}

#[test]
fn schema_declared_examples_validate() {
    for (id, _) in &frob_verbs::registry::SCHEMAS {
        let envelope = checked(&call(&["schema", id.as_str(), "--json"]), 0);
        let result = envelope.get("result").unwrap();
        let schema = Schema::compile(result.get("schema").unwrap().clone()).unwrap();
        schema.validate(result.get("example").unwrap()).unwrap();
    }
}

#[test]
fn refusal_802_schema_unknown_name_lists_nearest() {
    let unknown = checked(&call(&["schema", "doctr", "--json"]), 3);
    let refusal = unknown.get("refusal").unwrap();
    assert_eq!(refusal.get("pointer").and_then(Tree::as_str), Some("/id"));
    assert!(
        refusal
            .get("observed")
            .and_then(Tree::as_str)
            .unwrap()
            .split(", ")
            .any(|name| name == "doctor")
    );
}

#[test]
fn refusal_720_named_action_checks_its_own_scope() {
    // Synthetic registry data. Dispatch validation and authorization are the real pipeline.
    let mut entry = *SYNTHETIC_VERBS
        .iter()
        .find(|entry| entry.name == "design.check")
        .unwrap();
    entry.actions = &[Action {
        name: "maintain",
        kind: Kind::Admin,
        capabilities: &[Scope("design:admin")],
        paginated: false,
        destructive: true,
    }];
    let refusals = embedded_registry().unwrap();
    let raw = r#"{"$id":"frob.design.check.in/1","type":"object","properties":{},"additionalProperties":false}"#;
    let derived = entry.input_schema_with_common_fields_json(raw).unwrap();
    let schema = InputSchema::compile_for(&entry, &derived, refusals).unwrap();
    let pipeline = Pipeline::new(
        vec![entry],
        [(entry.name.into(), schema)].into(),
        refusals.clone(),
    );
    let request = Request::new(
        entry.name,
        Tree::parse(br#"{"action":"maintain"}"#).unwrap(),
    )
    .with_auth(SessionAuth::scoped("reader", ["design:read"]));
    let denied = pipeline.prepare(&request).unwrap().unwrap_err();
    assert_eq!(denied.refusal_code(), Some("FROB-REF-720"));
    let allowed = request.with_auth(SessionAuth::scoped("maintainer", ["design:admin"]));
    let resolved = pipeline.prepare(&allowed).unwrap().unwrap();
    assert_eq!(resolved.kind, Kind::Admin);
}

#[test]
fn grammar_from_schema() {
    // Synthetic registry entry, with an independently declared positional.
    let argv = ["design", "check", "quad-a"].map(str::to_owned);
    let (entry, count) = grammar::locate(SYNTHETIC_VERBS, &argv).unwrap();
    let shape = Tree::parse(br#"{"type":"object","x-frob-cli":{"positional":["design"]},"properties":{"design":{"type":"string"}}}"#).unwrap();
    assert_eq!(
        grammar::parse(entry, &shape, &argv[count..]).unwrap(),
        Tree::parse(br#"{"design":"quad-a"}"#).unwrap()
    );

    // Exercise the real derivation of two common flags on a synthetic run entry.
    let run = SYNTHETIC_VERBS
        .iter()
        .find(|entry| entry.kind == Kind::Run)
        .unwrap();
    let flags = [
        "--estimate",
        "--key",
        "run-1",
        "--envelope-scope",
        "gate:example",
    ]
    .map(str::to_owned);
    let args = grammar::parse(run, &Tree::Obj(vec![]), &flags).unwrap();
    assert_eq!(
        args.get("estimate_only").and_then(Tree::as_bool),
        Some(true)
    );
    assert_eq!(
        args.get("idempotency_key").and_then(Tree::as_str),
        Some("run-1")
    );
    assert_eq!(
        args.get("envelope_scope").and_then(Tree::as_str),
        Some("gate:example")
    );
    assert!(grammar::parse(run, &Tree::Obj(vec![]), &["--evict".into()]).is_err());
}

struct SyntheticSection(&'static str);

impl DoctorSection for SyntheticSection {
    fn name(&self) -> &'static str {
        "zz-test"
    }

    fn run(&self, cx: &DispatchCx) -> Result<SectionReport, HandlerFault> {
        cx.checkpoint()
            .map_err(|error| HandlerFault::Unmapped(error.to_string()))?;
        let finding = Tree::Obj(vec![
            ("code".into(), Tree::Str("synthetic-section".into())),
            ("severity".into(), Tree::Str(self.0.into())),
        ]);
        Ok(SectionReport {
            findings: vec![finding],
            ..SectionReport::default()
        })
    }
}

#[test]
fn doctor_section_registration_and_severity() {
    let entry = lookup("doctor").unwrap();
    let raw = frob_verbs::registry::lookup_schema(entry.input_schema.as_str()).unwrap();
    let registry = embedded_registry().unwrap();
    let schema = InputSchema::compile_for(entry, raw, registry).unwrap();
    let cases: [(&'static [&'static dyn DoctorSection], u8); 2] = [
        (&[&SyntheticSection("high")], 2),
        (&[&SyntheticSection("info")], 0),
    ];
    for (extra, expected_exit) in cases {
        let mut table = frob_verbs::handlers::handler_table();
        frob_verbs::operator::doctor::register(&mut table, extra);
        let pipeline = Pipeline::new(
            vec![*entry],
            [(entry.name.into(), schema.clone())].into(),
            registry.clone(),
        );
        let dispatch = InProcess::new(pipeline, table);
        let req = Request::new(entry.name, Tree::Obj(vec![]));
        let (_, answer) = dispatch
            .submit(&DispatchCx::new(), &req)
            .unwrap()
            .into_parts()
            .unwrap();
        assert_eq!(answer.exit().code(), expected_exit);
        let report = answer.tree().get("result").unwrap().as_object().unwrap();
        assert_eq!(
            report
                .iter()
                .map(|(name, _)| name.as_str())
                .collect::<Vec<_>>(),
            ["stack_receipt", "zz-test"]
        );
    }
}

#[test]
fn operator_socket_parity() {
    let verbs = ["doctor", "capabilities"].map(|name| *lookup(name).unwrap());
    let refusals = embedded_registry().unwrap();
    let schemas = verbs
        .iter()
        .map(|entry| {
            let raw = frob_verbs::registry::lookup_schema(entry.input_schema.as_str()).unwrap();
            let merged = entry.input_schema_with_common_fields_json(raw).unwrap();
            (
                entry.name.to_owned(),
                InputSchema::compile_for(entry, &merged, refusals).unwrap(),
            )
        })
        .collect();
    let backend = InProcess::new(
        Pipeline::new(verbs.to_vec(), schemas, refusals.clone()),
        frob_verbs::handlers::handler_table(),
    );
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let path = format!("/tmp/frob-bd022-{}-{nanos}.sock", std::process::id());
    let server =
        RpcServerHandle::spawn_current_uid(&path, Arc::new(backend), refusals.clone()).unwrap();
    for entry in verbs {
        let remote = Command::new(env!("CARGO_BIN_EXE_frob"))
            .env("FROB_DISPATCH", "socket")
            .env("FROB_SOCKET", &path)
            .env_remove("FROB_SESSION")
            .args([entry.name, "--json"])
            .output()
            .unwrap();
        checked(&remote, 0);
        assert_eq!(remote.stdout, call(&[entry.name, "--json"]).stdout);
    }
    assert_eq!(server.shutdown().unwrap().completed, 2);
}
