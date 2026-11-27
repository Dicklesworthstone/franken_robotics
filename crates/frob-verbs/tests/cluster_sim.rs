use frob_verbs::envelope::{Schema, Tree, embedded_registry};
use frob_verbs::registry::{
    Kind, Milestone, Tier, VERBS, VerbEntry, lookup, lookup_schema, lookup_tool,
};

fn sim_rows() -> impl Iterator<Item = &'static VerbEntry> {
    VERBS.iter().filter(|entry| entry.cluster == "sim")
}

#[test]
fn rows_present() {
    let expected = [
        (
            "sim.estimate",
            Kind::Read,
            Milestone::M2,
            "sim:read",
            &["simulate", "learn", "fleet"][..],
        ),
        (
            "sim.replay",
            Kind::Run,
            Milestone::M2,
            "sim:run",
            &["simulate", "runs"][..],
        ),
        (
            "sim.run",
            Kind::Run,
            Milestone::M0,
            "sim:run",
            &["simulate"][..],
        ),
    ];
    assert_eq!(sim_rows().count(), expected.len());
    for (row, (name, kind, milestone, scope, clusters)) in sim_rows().zip(expected) {
        assert_eq!(
            (row.name, row.kind, row.first_milestone, row.tier),
            (name, kind, milestone, Tier::S)
        );
        assert_eq!(
            row.capabilities
                .iter()
                .map(|scope| scope.as_str())
                .collect::<Vec<_>>(),
            [scope]
        );
        assert_eq!(row.task_clusters, clusters);
        assert_eq!(
            lookup_tool(&row.tool_name()).map(|entry| entry.name),
            Some(name)
        );
        assert_eq!(row.idempotent, kind == Kind::Read);
        assert!(
            row.examples
                .iter()
                .all(|example| example.starts_with(&format!("frob {} ", row.cli_path().join(" "))))
        );
        if kind == Kind::Run {
            assert!(
                row.examples
                    .iter()
                    .all(|example| example.split_whitespace().any(|arg| arg == "--key"))
            );
            assert_eq!(row.wait.map(|wait| wait.cli), Some("to-completion"));
        }
    }
}

#[test]
fn schemas_and_refusal_lists() {
    let registry = embedded_registry().unwrap();
    for row in sim_rows() {
        for (kind, id) in [("in", row.input_schema), ("out", row.output_schema)] {
            assert_eq!(id.as_str(), format!("frob.{}.{kind}/1", row.name));
            let document = Tree::parse(lookup_schema(id.as_str()).unwrap().as_bytes()).unwrap();
            let example = &document.get("examples").and_then(Tree::as_array).unwrap()[0];
            Schema::compile(document.clone())
                .unwrap()
                .validate(example)
                .unwrap();
        }
        let raw = lookup_schema(row.input_schema.as_str()).unwrap();
        let expected = if row.name == "sim.run" {
            &[
                701, 702, 703, 704, 720, 723, 724, 761, 790, 802, 803, 805, 808,
            ][..]
        } else if row.kind == Kind::Run {
            &[701, 702, 703, 704, 720, 723, 724, 761, 802, 803, 805, 808][..]
        } else {
            &[720, 723, 724, 802, 808][..]
        };
        let codes = row.all_refusals_with(raw, registry);
        assert_eq!(
            codes.iter().map(|item| item.code).collect::<Vec<_>>(),
            expected
        );
        assert!(
            codes
                .iter()
                .all(|item| item.typed().and_then(|code| registry.row(code)).is_some())
        );
    }
}

#[test]
fn m0_run_input_contract() {
    let row = lookup("sim.run").unwrap();
    let raw = lookup_schema(row.input_schema.as_str()).unwrap();
    let derived = row.input_schema_with_common_fields_json(raw).unwrap();
    let doc = Tree::parse(derived.as_bytes()).unwrap();
    assert_eq!(
        doc.get("x-frob-cli")
            .unwrap()
            .get("positional")
            .unwrap()
            .as_array()
            .unwrap(),
        [Tree::Str("design".into())]
    );
    let properties = doc.get("properties").unwrap();
    for flag in [
        "sync_wait_ms",
        "idempotency_key",
        "budget",
        "estimate_only",
        "on_cancel",
        "envelope_scope",
    ] {
        assert!(properties.get(flag).is_some(), "missing {flag}");
    }
    assert_eq!(
        properties
            .get("idempotency_key")
            .unwrap()
            .get("x-frob-key")
            .and_then(Tree::as_str),
        Some("required")
    );
    assert_eq!(
        properties.as_object().unwrap().len(),
        if cfg!(feature = "conformance") {
            12
        } else {
            11
        }
    );
    let schema = Schema::compile(doc).unwrap();
    let validate = |json: &str| {
        schema
            .validate(&Tree::parse(json.as_bytes()).unwrap())
            .is_ok()
    };
    for json in [
        r#"{"design":"g1","tier":"referee","seconds":"1/48","seed":"18446744073709551615"}"#,
        r#"{"design":"g1","tier":"batch-cpu"}"#,
    ] {
        assert!(validate(json), "{json}");
    }
    for json in [
        r#"{"design":"g1","tier":"refere"}"#,
        r#"{"design":"g1","tier":"referee","seconds":"0"}"#,
        r#"{"design":"g1","tier":"referee","seed":"-1"}"#,
        r#"{"design":"g1","tier":"referee","fixture":"g1"}"#,
        r#"{"design":"g1","tier":"referee","sync_wait_ms":30001}"#,
    ] {
        assert!(!validate(json), "{json}");
    }
    assert_eq!(
        validate(r#"{"design":"g1","tier":"referee","perturb_ulp":"q0"}"#),
        cfg!(feature = "conformance")
    );
}
