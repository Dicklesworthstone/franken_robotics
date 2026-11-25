//! Unit and property tests for kind-based verb derivation, action rules, shadowing lint,
//! `RL-11` (`description-too-long`), and `frob.verbs.entry/1` schema (bead NJL-01).

use std::collections::{BTreeMap, BTreeSet};

use frob_core::canon::to_canonical;
use frob_core::json::{Limits, admit};
use frob_core::refusal::RefusalCode;
use frob_core::registry::refusals::RefusalRegistry;
use frob_core::testkit::prop;

use frob_verbs::envelope::{Schema, Tree, envelope_schema_for};
use frob_verbs::registry::codegen::compile_registry_bytes;
use frob_verbs::registry::{
    Annotations, KeyRule, Kind, LintContext, Milestone, OwnedVerbEntry, PROCESS_OPTIONS,
    ParsedRegistry, RefusalOrigin, SchemaId, Scope, Tier, VERBS, VerbEntry, entry_schema_json,
    lookup_entry_json, parse_and_lint, schema_file_name,
};

const STR_SCHEMA: &str = r#"{"type":"object","properties":{"design":{"type":"string"}}}"#;
const EMPTY_SCHEMA: &str = r#"{"type":"object","properties":{}}"#;
const REQ_DIGEST_OUT: &str =
    r#"{"type":"object","x-frob-identity":"request-digest","properties":{"id":{"type":"string"}}}"#;

fn synth_refusals(with_path_codes: bool) -> RefusalRegistry {
    let mut doc = String::from(
        r#"schema = "frob.refusals/1"

[exit_rule]
"1xx" = 3
"2xx" = 3
"3xx" = 3
"4xx" = 3
"5xx" = 3
"6xx" = 6
"70x" = 4
"72x" = 5
"73x" = 5
"76x" = 4
"8xx" = 3
"9xx" = 9
conflict = ["FROB-REF-803", "FROB-REF-804"]
integrity = []
capability = []
"#,
    );
    let mut codes = vec![
        (101, 3),
        (104, 3),
        (107, 3),
        (110, 3),
        (701, 4),
        (702, 4),
        (703, 4),
        (704, 4),
        (720, 5),
        (723, 5),
        (724, 5),
        (725, 5),
        (761, 4),
        (802, 3),
        (803, 7),
        (804, 7),
        (805, 3),
        (806, 3),
        (807, 3),
        (808, 3),
        (891, 3),
        (892, 3),
        (893, 3),
        (990, 9),
    ];
    if with_path_codes {
        codes.push((734, 5));
        codes.push((828, 3));
    }
    for (num, exit) in codes {
        doc.push_str(&format!(
            "\n[[refusal]]\ncode = \"FROB-REF-{num}\"\nmeaning = \"synthetic {num}\"\ndefined_in = \"§32\"\nexit = {exit}\nretry = \"after-change\"\nrepairs = [\"fix\"]\nraised_by = [\"NJL-01\"]\n"
        ));
    }
    RefusalRegistry::from_bytes(doc.as_bytes()).expect("synthetic refusal registry loads")
}

fn parse_one(entry_toml: &str) -> OwnedVerbEntry {
    let full = format!("schema = \"frob.verbs/1\"\n\n{entry_toml}");
    let reg = ParsedRegistry::parse_toml(full.as_bytes()).expect("synthetic verb TOML parses");
    assert_eq!(reg.verbs.len(), 1);
    reg.verbs.into_iter().next().unwrap()
}

fn verb_toml(name: &str, cluster: &str, kind: &str, extra: &str) -> String {
    let idempotent = matches!(kind, "read" | "admin");
    let wait_line = if kind == "run" && !extra.contains("wait =") {
        "wait = { cli = \"to-completion\", mcp_ms = 5000, mcp_max_ms = 30000 }\n"
    } else {
        ""
    };
    let cli = name.replace('.', " ");
    let cost = if kind == "run" {
        "sim_estimator"
    } else {
        "static: under 50 ms"
    };
    format!(
        r#"[[verb]]
name = "{name}"
cluster = "{cluster}"
kind = "{kind}"
tier = "S"
capabilities = ["{cluster}:read"]
input_schema = "frob.{name}.in/1"
output_schema = "frob.{name}.out/1"
refusals = []
idempotent = {idempotent}
cost_model = "{cost}"
first_milestone = "M0"
task_clusters = ["orient"]
examples = ["frob {cli} --json"]
purpose = "Synthetic {kind} verb {name}."
use_when = "Synthetic derivation test."
owner = "frob-verbs"
{wait_line}{extra}"#
    )
}

fn specimen_design_check() -> OwnedVerbEntry {
    parse_one(
        r#"[[verb]]
name = "design.check"
cluster = "design"
kind = "read"
tier = "S"
capabilities = ["design:read"]
input_schema = "frob.design.check.in/1"
output_schema = "frob.design.check.out/1"
refusals = ["FROB-REF-101", "FROB-REF-104", "FROB-REF-107", "FROB-REF-110"]
idempotent = true
cost_model = "static: under 2 s for template-sized designs"
first_milestone = "M1"
task_clusters = ["author", "places"]
examples = ["frob design check quad-a --json"]
purpose = "Run the preflight stages and the analyses marked for check; return findings with ranked repairs."
use_when = "After any edit and before any simulation."
owner = "frob-design"
schema_version = 1
paginated = false
exports = false
estimable = false
"#,
    )
}

fn code_set(entry: &OwnedVerbEntry, schema: &str, reg: &RefusalRegistry) -> Vec<u16> {
    entry
        .all_refusals_with(schema, reg)
        .into_iter()
        .map(|r| r.code)
        .collect()
}

fn compile_merged(entry: &OwnedVerbEntry, raw_schema: &str) -> Schema {
    let merged = entry
        .input_schema_with_common_fields_json(raw_schema)
        .expect("merged input schema JSON");
    Schema::compile_json(&merged).expect("merged input schema compiles")
}

fn tree(json: &str) -> Tree {
    Tree::parse(json.as_bytes()).expect("valid test JSON")
}

#[test]
fn derive_imports_are_minimal() {
    let src = include_str!("../src/registry/derive.rs");
    for line in src.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("use ") {
            assert!(
                rest.starts_with("std::")
                    || rest.starts_with("frob_core::")
                    || rest.starts_with("super::entry::"),
                "derive.rs must import only std, frob_core, or super::entry; found: `{trimmed}`"
            );
        }
    }
    assert!(
        PROCESS_OPTIONS.contains(&"no-daemon"),
        "PROCESS_OPTIONS must include no-daemon (review note 5)"
    );
}

#[test]
fn appended_refusals_by_kind() {
    let reg = synth_refusals(true);

    let read = parse_one(&verb_toml("design.show", "design", "read", ""));
    assert_eq!(code_set(&read, STR_SCHEMA, &reg), vec![720, 723, 724, 802]);

    let run = parse_one(&verb_toml("sim.run", "sim", "run", ""));
    assert_eq!(
        code_set(&run, STR_SCHEMA, &reg),
        vec![701, 702, 703, 704, 720, 723, 724, 761, 802, 803, 805, 808]
    );

    let mutate = parse_one(&verb_toml("design.edit", "design", "mutate", ""));
    assert_eq!(
        code_set(&mutate, STR_SCHEMA, &reg),
        vec![720, 723, 724, 725, 802, 803, 804, 805]
    );

    let admin = parse_one(&verb_toml("job.cancel", "job", "admin", ""));
    assert_eq!(code_set(&admin, STR_SCHEMA, &reg), vec![720, 723, 724, 802]);
    let int_schema =
        r#"{"type":"object","properties":{"n":{"type":"integer","x-frob-int-type":"int53"}}}"#;
    assert_eq!(
        code_set(&admin, int_schema, &reg),
        vec![720, 723, 724, 802, 808]
    );

    for name in ["study.plan", "session.focus"] {
        let cluster = name.split('.').next().unwrap();
        let ko = parse_one(&verb_toml(name, cluster, "mutate", "key_optional = true\n"));
        assert_eq!(
            code_set(&ko, STR_SCHEMA, &reg),
            vec![720, 723, 724, 725, 802, 803, 804]
        );
    }
}

#[test]
fn common_properties_by_kind() {
    let reg = synth_refusals(true);

    let run = parse_one(&verb_toml("sim.run", "sim", "run", ""));
    let run_props = run.common_properties();
    let names: Vec<&str> = run_props.iter().map(|p| p.name).collect();
    let flags: Vec<&str> = run_props.iter().map(|p| p.cli_flag).collect();
    assert_eq!(
        names,
        vec![
            "sync_wait_ms",
            "idempotency_key",
            "budget",
            "estimate_only",
            "on_cancel",
            "envelope_scope",
        ]
    );
    assert_eq!(
        flags,
        vec![
            "--sync-wait-ms",
            "--key",
            "--budget",
            "--estimate",
            "--on-cancel",
            "--envelope-scope",
        ]
    );

    let mutate = parse_one(&verb_toml("design.edit", "design", "mutate", ""));
    let m_props = mutate.common_properties();
    assert_eq!(
        m_props.iter().map(|p| p.name).collect::<Vec<_>>(),
        vec!["idempotency_key", "expected_parent"]
    );
    assert_eq!(
        m_props.iter().map(|p| p.cli_flag).collect::<Vec<_>>(),
        vec!["--key", "--expected-parent"]
    );

    let plain_read = parse_one(&verb_toml("design.show", "design", "read", ""));
    assert!(plain_read.common_properties().is_empty());

    let est_read = parse_one(&verb_toml(
        "design.cost",
        "design",
        "read",
        "estimable = true\n",
    ));
    let est_props = est_read.common_properties();
    assert_eq!(
        est_props.iter().map(|p| p.name).collect::<Vec<_>>(),
        vec!["estimate_only"]
    );
    assert!(!code_set(&est_read, STR_SCHEMA, &reg).contains(&701));

    let page_read = parse_one(&verb_toml(
        "design.list",
        "design",
        "read",
        "paginated = true\n",
    ));
    assert_eq!(
        page_read
            .common_properties()
            .iter()
            .map(|p| p.name)
            .collect::<Vec<_>>(),
        vec!["cursor", "limit", "follow"]
    );

    // A run entry with paginated = true gets no cursor/limit/follow and no 806/807.
    let page_run = parse_one(&verb_toml("sim.run", "sim", "run", "paginated = true\n"));
    let pr_names: Vec<&str> = page_run
        .common_properties()
        .iter()
        .map(|p| p.name)
        .collect();
    assert!(!pr_names.contains(&"cursor"));
    assert!(!pr_names.contains(&"limit"));
    assert!(!pr_names.contains(&"follow"));
    let pr_codes = code_set(&page_run, STR_SCHEMA, &reg);
    assert!(!pr_codes.contains(&806));
    assert!(!pr_codes.contains(&807));
}

#[test]
fn envelope_scope_schema() {
    let reg = synth_refusals(true);
    let run = parse_one(&verb_toml("sim.run", "sim", "run", ""));
    let schema = compile_merged(&run, EMPTY_SCHEMA);

    for ok in [
        r#"{"envelope_scope":"gate:GATE-SPARK-11"}"#,
        r#"{"envelope_scope":"campaign:rank-agreement"}"#,
        r#"{"envelope_scope":"pretrain:r-1"}"#,
        r#"{}"#,
    ] {
        assert!(schema.validate(&tree(ok)).is_ok(), "expected valid: {ok}");
    }
    for bad in [
        r#"{"envelope_scope":"adhoc"}"#,
        r#"{"envelope_scope":"rank-agreement"}"#,
        r#"{"envelope_scope":"gate:"}"#,
    ] {
        let errs = schema
            .validate(&tree(bad))
            .expect_err(&format!("expected failure: {bad}"));
        assert_eq!(errs[0].instance, "/envelope_scope");
    }
    // Optional and adds no code beyond the run set.
    let raw = run
        .input_schema_with_common_fields_json(EMPTY_SCHEMA)
        .unwrap();
    let doc = admit(raw.as_bytes(), &Limits::default()).unwrap();
    let req_has_scope = doc
        .value()
        .get("required")
        .and_then(|v| v.as_array())
        .is_some_and(|a| a.iter().any(|x| x.as_str() == Some("envelope_scope")));
    assert!(!req_has_scope);
    assert_eq!(
        code_set(&run, EMPTY_SCHEMA, &reg),
        vec![701, 702, 703, 704, 720, 723, 724, 761, 802, 803, 805, 808]
    );
}

#[test]
fn paginated_read_action() {
    let reg = synth_refusals(true);
    let entry = parse_one(&verb_toml(
        "spark.check",
        "spark",
        "run",
        "actions = [{ name = \"list\", kind = \"read\", capabilities = [\"spark:read\"], paginated = true }]\n",
    ));
    let props = entry.common_properties();
    for pname in ["cursor", "limit", "follow"] {
        let cp = props
            .iter()
            .find(|p| p.name == pname)
            .expect("paginated read action property present");
        assert!(
            !cp.on_default,
            "{pname} must not belong to default run action"
        );
        assert_eq!(cp.actions, vec!["list"]);
        assert!(
            cp.schema_json.contains("list"),
            "{pname} description must name action `list`"
        );
    }
    let refs = entry.all_refusals_with(EMPTY_SCHEMA, &reg);
    for code in [806u16, 807u16] {
        let row = refs
            .iter()
            .find(|r| r.code == code)
            .expect("paginated read action code present");
        assert_eq!(row.source.action.as_deref(), Some("list"));
    }
    // Using `cursor` with the default action fails with pointer `/cursor`.
    let schema = compile_merged(&entry, EMPTY_SCHEMA);
    let errs = schema
        .validate(&tree(r#"{"cursor":"c1"}"#))
        .expect_err("cursor on default run action must fail");
    assert_eq!(errs[0].instance, "/cursor");
    assert!(
        schema
            .validate(&tree(r#"{"action":"list","cursor":"c1","limit":10}"#))
            .is_ok()
    );
}

#[test]
fn key_never_schema_required() {
    let cases = [
        (
            parse_one(&verb_toml("sim.run", "sim", "run", "")),
            KeyRule::Required,
        ),
        (
            parse_one(&verb_toml("design.edit", "design", "mutate", "")),
            KeyRule::Required,
        ),
        (
            parse_one(&verb_toml(
                "study.plan",
                "study",
                "mutate",
                "key_optional = true\n",
            )),
            KeyRule::Optional,
        ),
    ];
    for (entry, expected_rule) in cases {
        let cp = entry
            .common_properties()
            .into_iter()
            .find(|p| p.name == "idempotency_key")
            .expect("idempotency_key present");
        assert_eq!(cp.key_rule, Some(expected_rule));

        let merged = entry
            .input_schema_with_common_fields_json(STR_SCHEMA)
            .unwrap();
        let adm = admit(merged.as_bytes(), &Limits::default()).unwrap();
        let in_required = adm
            .value()
            .get("required")
            .and_then(|v| v.as_array())
            .is_some_and(|a| a.iter().any(|x| x.as_str() == Some("idempotency_key")));
        assert!(!in_required, "idempotency_key must never be in `required`");
        let x_key = adm
            .value()
            .get("properties")
            .and_then(|p| p.get("idempotency_key"))
            .and_then(|k| k.get("x-frob-key"))
            .and_then(|v| v.as_str());
        assert_eq!(x_key, Some(expected_rule.as_str()));
    }
}

#[test]
fn on_cancel_values() {
    let run = parse_one(&verb_toml("sim.run", "sim", "run", ""));
    let cp = run
        .common_properties()
        .into_iter()
        .find(|p| p.name == "on_cancel")
        .unwrap();
    let adm = admit(cp.schema_json.as_bytes(), &Limits::default()).unwrap();
    assert_eq!(
        adm.value().get("default").and_then(|v| v.as_str()),
        Some("leave")
    );
    let vals: Vec<&str> = adm
        .value()
        .get("enum")
        .and_then(|v| v.as_array())
        .unwrap()
        .iter()
        .filter_map(|v| v.as_str())
        .collect();
    assert_eq!(vals, vec!["leave", "cancel_job"]);

    let schema = compile_merged(&run, EMPTY_SCHEMA);
    assert!(schema.validate(&tree(r#"{"on_cancel":"leave"}"#)).is_ok());
    assert!(
        schema
            .validate(&tree(r#"{"on_cancel":"cancel_job"}"#))
            .is_ok()
    );
    let errs = schema
        .validate(&tree(r#"{"on_cancel":"leave_running"}"#))
        .expect_err("retired leave_running must fail");
    assert_eq!(errs[0].instance, "/on_cancel");
}

#[test]
fn sync_wait_bounds() {
    let run = parse_one(&verb_toml("sim.run", "sim", "run", ""));
    let schema = compile_merged(&run, EMPTY_SCHEMA);
    for ok in [0, 5_000, 30_000] {
        assert!(
            schema
                .validate(&tree(&format!(r#"{{"sync_wait_ms":{ok}}}"#)))
                .is_ok()
        );
    }
    for bad in [-1, 30_001] {
        let errs = schema
            .validate(&tree(&format!(r#"{{"sync_wait_ms":{bad}}}"#)))
            .expect_err("out-of-range sync_wait_ms must fail");
        assert_eq!(errs[0].instance, "/sync_wait_ms");
    }

    let capped = parse_one(&verb_toml(
        "sim.fast",
        "sim",
        "run",
        "wait = { cli = \"to-completion\", mcp_ms = 2500, mcp_max_ms = 10000 }\n",
    ));
    let capped_schema = compile_merged(&capped, EMPTY_SCHEMA);
    assert!(
        capped_schema
            .validate(&tree(r#"{"sync_wait_ms":10000}"#))
            .is_ok()
    );
    let errs = capped_schema
        .validate(&tree(r#"{"sync_wait_ms":10001}"#))
        .expect_err("10001 exceeds wait.mcp_max_ms = 10000");
    assert_eq!(errs[0].instance, "/sync_wait_ms");
    let cp = capped
        .common_properties()
        .into_iter()
        .find(|p| p.name == "sync_wait_ms")
        .unwrap();
    let adm = admit(cp.schema_json.as_bytes(), &Limits::default()).unwrap();
    assert_eq!(
        adm.value().get("default").and_then(|v| v.as_int53().ok()),
        Some(2500)
    );
}

#[test]
fn int53_drives_808() {
    let reg = synth_refusals(true);
    let read = parse_one(&verb_toml("design.count", "design", "read", ""));

    let top_int =
        r#"{"type":"object","properties":{"count":{"type":"integer","x-frob-int-type":"int53"}}}"#;
    let nested_int = r#"{"type":"object","properties":{"cfg":{"type":"object","properties":{"k":{"type":"integer"}}}}}"#;
    let u64str_only = r#"{"type":"object","properties":{"seed":{"type":"string","x-frob-int-type":"u64str"},"name":{"type":"string"}}}"#;
    let unknown_type = r#"{"type":"object","properties":{"x":{"type":"sint48"}}}"#;

    assert!(code_set(&read, top_int, &reg).contains(&808));
    assert!(code_set(&read, nested_int, &reg).contains(&808));
    assert!(!code_set(&read, u64str_only, &reg).contains(&808));
    assert!(code_set(&read, unknown_type, &reg).contains(&808));
}

#[test]
fn field_appended_codes() {
    let reg = synth_refusals(true);

    let check = |param_spec: &str, expected: &[u16], absent: &[u16]| {
        let entry = parse_one(&verb_toml(
            "design.load",
            "design",
            "read",
            &format!("params = {{ src = \"{param_spec}\" }}\n"),
        ));
        let refs = entry.all_refusals_with(STR_SCHEMA, &reg);
        let codes: Vec<u16> = refs.iter().map(|r| r.code).collect();
        for &c in expected {
            let found = refs
                .iter()
                .find(|r| r.code == c)
                .expect("expected param code");
            assert_eq!(found.source.origin, RefusalOrigin::Field("params"));
        }
        for &c in absent {
            assert!(!codes.contains(&c), "unexpected code {c} for {param_spec}");
        }
    };

    check("artifact-identity<design>", &[891, 892, 734, 828], &[893]);
    check("foreign-file", &[893, 734, 828], &[891, 892]);
    check("artifact-input<task>", &[891, 734, 828], &[892, 893]);
    check("artifact-ref<run>", &[891], &[892, 893, 734, 828]);

    // Chapter list already naming 828 yields one 828.
    let with_chapter_828 = parse_one(
        &verb_toml(
            "design.import",
            "design",
            "read",
            "params = { src = \"foreign-file\" }\n",
        )
        .replace("refusals = []", "refusals = [\"FROB-REF-828\"]"),
    );
    let count_828 = with_chapter_828
        .all_refusals_with(STR_SCHEMA, &reg)
        .iter()
        .filter(|r| r.code == 828)
        .count();
    assert_eq!(count_828, 1);
}

#[test]
fn appended_only_when_entered() {
    let reg_without = synth_refusals(false);
    for (spec, expected) in [
        ("artifact-identity<design>", &[891u16, 892][..]),
        ("foreign-file", &[893][..]),
        ("artifact-input<task>", &[891][..]),
        ("artifact-ref<run>", &[891][..]),
    ] {
        let entry = parse_one(&verb_toml(
            "design.load",
            "design",
            "read",
            &format!("params = {{ src = \"{spec}\" }}\n"),
        ));
        let codes = code_set(&entry, STR_SCHEMA, &reg_without);
        for &c in expected {
            assert!(codes.contains(&c));
        }
        assert!(
            !codes.contains(&734),
            "734 must not be appended when absent from registry"
        );
        assert!(
            !codes.contains(&828),
            "828 must not be appended when absent from registry"
        );
    }
}

#[test]
fn internal_fault_never_appended() {
    let reg = synth_refusals(true);
    for kind in ["read", "run", "mutate", "admin"] {
        for extra in [
            "",
            "paginated = true\n",
            "estimable = true\n",
            "key_optional = true\n",
            "params = { a = \"artifact-identity<d>\", b = \"foreign-file\" }\n",
            "actions = [{ name = \"act\", kind = \"admin\", capabilities = [\"x:admin\"] }]\n",
        ] {
            let entry = parse_one(&verb_toml("test.op", "test", kind, extra));
            let codes = code_set(&entry, STR_SCHEMA, &reg);
            assert!(
                !codes.contains(&990),
                "FROB-REF-990 must never appear in derived refusal list (kind={kind}, extra={extra:?})"
            );
        }
    }
}

#[test]
fn two_kind_actions() {
    let reg = synth_refusals(true);
    let cache = parse_one(&verb_toml(
        "spark.cache",
        "spark",
        "read",
        "actions = [{ name = \"evict\", kind = \"admin\", capabilities = [\"spark:admin\"] }]\n",
    ));
    let action_prop = cache
        .common_properties()
        .into_iter()
        .find(|p| p.name == "action")
        .expect("action property");
    assert_eq!(action_prop.actions, vec!["evict"]);
    let (_, def_caps, _, _) = cache.resolve_owned_action(None).unwrap();
    let (_, ev_caps, _, _) = cache.resolve_owned_action(Some("evict")).unwrap();
    assert!(!def_caps.contains(&"spark:admin".to_owned()));
    assert!(ev_caps.contains(&"spark:admin".to_owned()));
    let ann = cache.annotations();
    assert!(!ann.read_only);
    assert!(ann.destructive);
    let tags = cache.tags();
    assert!(tags.contains(&"kind:admin".to_owned()));
    assert!(tags.contains(&"kind:read".to_owned()));

    // Synthetic nemesis.hardset: default `read`, action `run` of kind `run`.
    let hardset = parse_one(&verb_toml(
        "nemesis.hardset",
        "nemesis",
        "read",
        "wait = { cli = \"to-completion\", mcp_ms = 5000, mcp_max_ms = 30000 }\nactions = [{ name = \"run\", kind = \"run\", capabilities = [\"nemesis:run\"] }]\n",
    ));
    let props = hardset.common_properties();
    for pname in [
        "sync_wait_ms",
        "idempotency_key",
        "budget",
        "estimate_only",
        "on_cancel",
        "envelope_scope",
    ] {
        let cp = props.iter().find(|p| p.name == pname).expect("run prop");
        assert!(!cp.on_default);
        assert_eq!(cp.actions, vec!["run"]);
        assert!(cp.schema_json.contains("run"));
    }
    let schema = compile_merged(&hardset, EMPTY_SCHEMA);
    let errs = schema
        .validate(&tree(r#"{"sync_wait_ms":1000}"#))
        .expect_err("sync_wait_ms with default read action must fail");
    assert_eq!(errs[0].instance, "/sync_wait_ms");
    assert!(
        schema
            .validate(&tree(r#"{"action":"run","sync_wait_ms":1000}"#))
            .is_ok()
    );
    let ref805 = hardset
        .all_refusals_with(EMPTY_SCHEMA, &reg)
        .into_iter()
        .find(|r| r.code == 805)
        .expect("805 in full list");
    assert_eq!(ref805.source.origin, RefusalOrigin::Kind(Kind::Run));
    assert_eq!(ref805.source.action.as_deref(), Some("run"));
    let ref808 = hardset
        .all_refusals_with(EMPTY_SCHEMA, &reg)
        .into_iter()
        .find(|r| r.code == 808)
        .expect("808 in full list");
    assert_eq!(ref808.source.action.as_deref(), Some("run"));
}

#[test]
fn annotations_rule() {
    let read = parse_one(&verb_toml("design.show", "design", "read", ""));
    assert_eq!(
        read.annotations(),
        Annotations {
            read_only: true,
            idempotent: true,
            destructive: false,
            open_world_hint: false,
        }
    );
    let admin = parse_one(&verb_toml(
        "job.purge",
        "job",
        "admin",
        "destructive = true\n",
    ));
    assert!(admin.annotations().destructive);
    assert!(!admin.annotations().open_world_hint);

    let mutate = parse_one(&verb_toml("design.edit", "design", "mutate", ""));
    assert!(!mutate.annotations().read_only);
    assert!(!mutate.annotations().idempotent);
    assert!(!mutate.annotations().open_world_hint);
}

#[test]
fn shadow_lint_catches() {
    let run_lint = |toml_body: &str, in_schema: &str, conformance: bool| {
        let doc = format!("schema = \"frob.verbs/1\"\n\n{toml_body}");
        let parsed = ParsedRegistry::parse_toml(doc.as_bytes()).unwrap();
        let e = &parsed.verbs[0];
        let mut schemas = BTreeMap::new();
        schemas.insert(
            schema_file_name(&e.input_schema),
            in_schema.as_bytes().to_vec(),
        );
        schemas.insert(
            schema_file_name(&e.output_schema),
            EMPTY_SCHEMA.as_bytes().to_vec(),
        );
        let ctx = LintContext {
            refusals: synth_refusals(true),
            schemas,
            scopes: None,
            append_kind_refusals: true,
            conformance,
        };
        parse_and_lint(doc.as_bytes(), &ctx)
    };

    let cases: &[(&str, &str, &str, &str)] = &[
        (
            &verb_toml("shield.fit", "shield", "run", ""),
            r#"{"type":"object","properties":{"budget":{"type":"number"}}}"#,
            "budget",
            "kind:run",
        ),
        (
            &verb_toml("design.edit", "design", "mutate", ""),
            r#"{"type":"object","properties":{"key":{"type":"string"}}}"#,
            "--key",
            "idempotency_key",
        ),
        (
            &verb_toml("design.show", "design", "read", ""),
            r#"{"type":"object","properties":{"json":{"type":"boolean"}}}"#,
            "--json",
            "process option",
        ),
        (
            &verb_toml("session.show", "session", "read", ""),
            r#"{"type":"object","properties":{"session":{"type":"string"}}}"#,
            "--session",
            "process option",
        ),
        (
            &verb_toml("design.show", "design", "read", ""),
            r#"{"type":"object","properties":{"no_daemon":{"type":"boolean"}}}"#,
            "--no-daemon",
            "process option",
        ),
        (
            &verb_toml("design.show", "design", "read", ""),
            r#"{"type":"object","properties":{"help":{"type":"boolean"}}}"#,
            "--help",
            "--help",
        ),
        (
            &verb_toml("design.list", "design", "read", "paginated = true\n"),
            r#"{"type":"object","properties":{"limit":{"type":"integer"}}}"#,
            "limit",
            "paginated",
        ),
        (
            &verb_toml(
                "spark.cache",
                "spark",
                "read",
                "actions = [{ name = \"evict\", kind = \"admin\", capabilities = [\"spark:admin\"] }]\n",
            ),
            r#"{"type":"object","properties":{"action":{"type":"string"}}}"#,
            "action",
            "actions",
        ),
        (
            &verb_toml("sim.run", "sim", "run", ""),
            r#"{"type":"object","properties":{"envelope_scope":{"type":"string"}}}"#,
            "envelope_scope",
            "kind:run",
        ),
    ];

    for &(body, schema, needle1, needle2) in cases {
        let err = run_lint(body, schema, false).expect_err("shadowing row must fail");
        assert_eq!(err.slug, "common-property-shadowed");
        assert!(
            err.detail.contains(needle1) && err.detail.contains(needle2),
            "expected `{needle1}` and `{needle2}` in `{}`",
            err.detail
        );
    }

    // `conformance_only` property is stripped in non-conformance builds and judged in conformance builds only.
    let conf_row = verb_toml(
        "shield.fit",
        "shield",
        "run",
        "conformance_only = [\"budget\"]\n",
    );
    let conf_schema = r#"{"type":"object","properties":{"budget":{"type":"number"}}}"#;
    assert!(run_lint(&conf_row, conf_schema, false).is_ok());
    let conf_err =
        run_lint(&conf_row, conf_schema, true).expect_err("conformance build catches shadow");
    assert_eq!(conf_err.slug, "common-property-shadowed");

    // Renamed property passes.
    let renamed = r#"{"type":"object","properties":{"energy_budget":{"type":"number"}}}"#;
    assert!(
        run_lint(
            &verb_toml("shield.fit", "shield", "run", ""),
            renamed,
            false
        )
        .is_ok()
    );
}

#[test]
fn short_doc_format() {
    let reg = synth_refusals(true);
    let spec = specimen_design_check();
    let doc = spec.short_doc_with(STR_SCHEMA, &reg);
    assert!(doc.starts_with(spec.purpose.as_str()));
    assert!(doc.contains("Refusals (FROB-REF-): 101, 104, 107, 110, 720, 723, 724, 802"));
    assert!(doc.ends_with("Example: frob design check quad-a --json"));
    assert!(doc.chars().count() <= 800);

    // Build an entry whose derived short_doc is exactly 801 characters.
    let mut long_entry = spec.clone();
    let base_len = long_entry.short_doc_with(STR_SCHEMA, &reg).chars().count();
    let pad = 801 - base_len;
    long_entry.use_when.push_str(&"x".repeat(pad));
    assert_eq!(
        long_entry.short_doc_checked_with(STR_SCHEMA, &reg),
        Err(801)
    );
}

#[test]
fn rl11_description_801_chars_caught() {
    let reg = synth_refusals(true);
    let mut entry = specimen_design_check();
    let base_len = entry.short_doc_with(STR_SCHEMA, &reg).chars().count();
    entry.use_when.push_str(&"x".repeat(801 - base_len));

    let make_ctx = || {
        let mut schemas = BTreeMap::new();
        schemas.insert(
            schema_file_name(&entry.input_schema),
            br#"{"type":"object","x-frob-cli":{"positional":["design"]},"properties":{"design":{"type":"string"}}}"#.to_vec(),
        );
        schemas.insert(
            schema_file_name(&entry.output_schema),
            EMPTY_SCHEMA.as_bytes().to_vec(),
        );
        LintContext {
            refusals: synth_refusals(true),
            schemas,
            scopes: None,
            append_kind_refusals: true,
            conformance: false,
        }
    };

    let doc_801 = format!("schema = \"frob.verbs/1\"\n\n{}", entry.to_toml_string());
    let err = compile_registry_bytes(doc_801.as_bytes(), &make_ctx())
        .expect_err("801-char short_doc must fail RL-11");
    assert_eq!(err.lint, "RL-11");
    assert_eq!(err.slug, "description-too-long");
    assert_eq!(err.entry, "design.check");

    // Boundary check: 800 chars passes.
    entry.use_when.pop();
    let doc_800 = format!("schema = \"frob.verbs/1\"\n\n{}", entry.to_toml_string());
    assert!(compile_registry_bytes(doc_800.as_bytes(), &make_ctx()).is_ok());
}

#[test]
fn derived_members_complete() {
    let reg = synth_refusals(true);
    let spec = specimen_design_check();
    let refs = spec.all_refusals_with(STR_SCHEMA, &reg);
    let codes: Vec<u16> = refs.iter().map(|r| r.code).collect();
    assert_eq!(codes, vec![101, 104, 107, 110, 720, 723, 724, 802]);
    assert!(codes.windows(2).all(|w| w[0] < w[1]));
    assert_eq!(refs[0].source.origin, RefusalOrigin::Chapter);
    assert_eq!(refs[4].source.origin, RefusalOrigin::Kind(Kind::Read));
    assert_eq!(
        spec.tags(),
        vec![
            "cluster:design",
            "kind:read",
            "task:author",
            "task:places",
            "tier:S",
        ]
    );
}

#[test]
fn envelope_schema_delegates() {
    const ENTRY: VerbEntry = VerbEntry {
        name: "study.plan",
        cluster: "study",
        kind: Kind::Mutate,
        tier: Tier::S,
        capabilities: &[Scope("study:write")],
        input_schema: SchemaId("frob.study.plan.in/1"),
        output_schema: SchemaId("frob.study.plan.out/1"),
        refusals: &[RefusalCode::FrobRef802],
        idempotent: false,
        cost_model: "static: under 50 ms",
        first_milestone: Milestone::M2,
        task_clusters: &["studies"],
        examples: &["frob study plan --json"],
        purpose: "Plan a study.",
        use_when: "Before submitting sweeps.",
        owner: "frob-arena",
        wait: None,
        actions: &[],
        schema_version: 1,
        paginated: false,
        exports: false,
        estimable: false,
        destructive: false,
        public: false,
        key_optional: true,
        option_capabilities: &[],
        spends_physics: None,
        released: false,
        params: &[],
        conformance_only: &[],
    };
    let via_member = ENTRY.envelope_schema_for(REQ_DIGEST_OUT).unwrap();
    let via_envelope = envelope_schema_for(&ENTRY, REQ_DIGEST_OUT).unwrap();
    assert_eq!(
        via_member.document().to_canonical().unwrap().as_bytes(),
        via_envelope.document().to_canonical().unwrap().as_bytes()
    );
}

#[test]
fn derivation_ignores_first_milestone() {
    let reg = synth_refusals(true);
    let m0 = parse_one(&verb_toml("sim.run", "sim", "run", ""));
    let mut m8 = m0.clone();
    m8.first_milestone_raw = "M8".to_owned();

    assert_eq!(m0.common_properties(), m8.common_properties());
    assert_eq!(
        m0.input_schema_with_common_fields_json(STR_SCHEMA).unwrap(),
        m8.input_schema_with_common_fields_json(STR_SCHEMA).unwrap()
    );
    assert_eq!(
        m0.all_refusals_with(STR_SCHEMA, &reg),
        m8.all_refusals_with(STR_SCHEMA, &reg)
    );
    assert_eq!(
        m0.short_doc_with(STR_SCHEMA, &reg),
        m8.short_doc_with(STR_SCHEMA, &reg)
    );
    assert_eq!(m0.tags(), m8.tags());
    assert_eq!(m0.annotations(), m8.annotations());
}

#[test]
fn entry_schema_file() {
    let on_disk = include_str!("../../../schemas/frob.verbs.entry.1.json");
    let generated = entry_schema_json();
    assert_eq!(
        on_disk, generated,
        "schemas/frob.verbs.entry.1.json must regenerate byte-identically"
    );

    let compiled = Schema::compile_json(on_disk).expect("entry schema compiles");
    // Validate its own embedded minimal example.
    let ex = compiled
        .document()
        .get("examples")
        .and_then(|v| v.as_array())
        .and_then(|a| a.first())
        .expect("embedded example");
    assert!(
        compiled.validate(ex).is_ok(),
        "embedded example must validate"
    );

    // Validate canonical serialization of specimen `design.check`.
    let spec_bytes = to_canonical(&specimen_design_check()).unwrap();
    let spec_tree = Tree::parse(spec_bytes.as_bytes()).unwrap();
    assert!(
        compiled.validate(&spec_tree).is_ok(),
        "specimen design.check must validate"
    );
    for entry in &VERBS {
        let row = lookup_entry_json(entry.name).expect("canonical registry row");
        let row = tree(row);
        assert_eq!(row.get("name").and_then(Tree::as_str), Some(entry.name));
        assert!(compiled.validate(&row).is_ok(), "{}", entry.name);
    }

    // Closed schema: unknown field must fail validation.
    let mut bad = spec_bytes.as_str().to_owned();
    bad.pop();
    bad.push_str(",\"unknown_field\":true}");
    assert!(compiled.validate(&tree(&bad)).is_err());
}

#[test]
fn prop_derivation_closed() {
    let reg = synth_refusals(true);
    let kinds = ["read", "run", "mutate", "admin"];
    let param_kinds = [
        "artifact-input<FMT-FBOT>",
        "artifact-ref<FMT-RUN>",
        "artifact-identity<FMT-DESIGN>",
        "foreign-file",
    ];
    let action_names = ["alpha", "beta", "gamma"];

    prop::check("prop_derivation_closed", 256, |g| {
        let kind = kinds[(g.next_u64(0) as usize) % kinds.len()];
        let paginated = g.bool(1);
        let estimable = g.bool(2);
        let key_optional = g.bool(3);
        let has_int53 = g.bool(4);
        let n_actions = (g.next_u64(5) as usize) % 4;
        let n_params = (g.next_u64(6) as usize) % 3;

        let mut extra = String::new();
        if paginated {
            extra.push_str("paginated = true\n");
        }
        if estimable {
            extra.push_str("estimable = true\n");
        }
        if key_optional {
            extra.push_str("key_optional = true\n");
        }
        let mut acts = Vec::new();
        for idx in 0..n_actions {
            let ak = kinds[(g.next_u64(10 + idx as u64) as usize) % kinds.len()];
            let ap = g.bool(20 + idx as u64);
            acts.push(format!(
                "{{ name = \"{}\", kind = \"{ak}\", capabilities = [\"test:act\"]{} }}",
                action_names[idx],
                if ap { ", paginated = true" } else { "" }
            ));
        }
        if !acts.is_empty() {
            extra.push_str(&format!("actions = [{}]\n", acts.join(", ")));
        }
        if n_params > 0 {
            let mut ps = Vec::new();
            for idx in 0..n_params {
                let pk = param_kinds[(g.next_u64(30 + idx as u64) as usize) % param_kinds.len()];
                ps.push(format!("p{idx} = \"{pk}\""));
            }
            extra.push_str(&format!("params = {{ {} }}\n", ps.join(", ")));
        }
        let raw_in = if has_int53 {
            r#"{"type":"object","properties":{"n":{"type":"integer","x-frob-int-type":"int53"}}}"#
        } else {
            STR_SCHEMA
        };

        let entry = parse_one(
            &verb_toml("test.op", "test", kind, &extra)
                .replace("refusals = []", "refusals = [\"FROB-REF-101\"]"),
        );

        let props = entry.common_properties();
        let mut seen_names = BTreeSet::new();
        let mut seen_flags = BTreeSet::new();
        for p in &props {
            if !seen_names.insert(p.name) {
                return Err(format!("duplicate common property `{}`", p.name));
            }
            if !seen_flags.insert(p.cli_flag) {
                return Err(format!("duplicate CLI flag `{}`", p.cli_flag));
            }
        }

        let schema1 = entry
            .input_schema_with_common_fields_json(raw_in)
            .map_err(|e| e.to_string())?;
        Schema::compile_json(&schema1).map_err(|e| e.to_string())?;
        let schema2 = entry
            .input_schema_with_common_fields_json(raw_in)
            .map_err(|e| e.to_string())?;
        if schema1 != schema2 {
            return Err("derivation is not pure".into());
        }

        let refs = entry.all_refusals_with(raw_in, &reg);
        let codes: Vec<u16> = refs.iter().map(|r| r.code).collect();
        if !codes.windows(2).all(|w| w[0] < w[1]) {
            return Err(format!("refusals not strictly ascending: {codes:?}"));
        }
        if !codes.contains(&101) {
            return Err("chapter code 101 dropped".into());
        }
        if codes.contains(&990) {
            return Err("990 leaked into derived list".into());
        }

        // Permuting actions must leave every derived output byte-identical.
        let mut permuted = entry.clone();
        permuted.actions.reverse();
        if permuted.common_properties() != props
            || permuted
                .input_schema_with_common_fields_json(raw_in)
                .map_err(|e| e.to_string())?
                != schema1
            || permuted.all_refusals_with(raw_in, &reg) != refs
            || permuted.short_doc_with(raw_in, &reg) != entry.short_doc_with(raw_in, &reg)
            || permuted.tags() != entry.tags()
            || permuted.annotations() != entry.annotations()
        {
            return Err("action permutation altered derived output".into());
        }
        Ok(())
    });
}
