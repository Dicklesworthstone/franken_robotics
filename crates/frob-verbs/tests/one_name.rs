//! NJL-04: registry aliases and their staged replacement targets.

use std::collections::BTreeSet;

use frob_verbs::registry::{
    LintContext, LintError, LintReport, ParsedRegistry, RETIRED_VERBS, VERBS, lint_registry,
    lookup, lookup_retired_verb, lookup_tool, schema_file_name, tool_name,
};

const RETIRED: &str = r#"schema = "frob.verbs/1"
[[retired_name]]
name = "autopsy.fork"
use = ["run.fork"]
reason = "Replay owns forks."
"#;

fn row(name: &str, schemas: &str) -> String {
    let (cluster, verb) = name.split_once('.').unwrap();
    format!(
        r#"
[[verb]]
name = "{name}"
cluster = "{cluster}"
kind = "read"
tier = "S"
capabilities = []
public = true
input_schema = "frob.{schemas}.in/1"
output_schema = "frob.{schemas}.out/1"
refusals = []
idempotent = true
cost_model = "static: read"
first_milestone = "M0"
task_clusters = ["orient"]
examples = ["frob {cluster} {verb} --json"]
purpose = "Synthetic verb."
use_when = "Checking the alias lint."
owner = "frob-verbs"
"#
    )
}

fn lint(doc: &str) -> Result<LintReport, LintError> {
    let registry = ParsedRegistry::parse_toml(doc.as_bytes()).unwrap();
    let schemas = registry
        .verbs
        .iter()
        .flat_map(|entry| [&entry.input_schema, &entry.output_schema])
        .map(|id| (schema_file_name(id), br#"{"type":"object"}"#.to_vec()))
        .collect();
    lint_registry(
        &registry,
        &LintContext::with_embedded_refusals(schemas, None, false)?,
    )
}

#[test]
fn lint_retired_name() {
    let err = lint(&format!("{RETIRED}{}", row("autopsy.fork", "autopsy.fork"))).unwrap_err();
    assert_eq!(err.slug, "retired-name");
    assert_eq!(err.entry, "autopsy.fork");
    assert!(err.detail.contains("run.fork"));
}

#[test]
fn lint_same_schemas() {
    let err = lint(&format!(
        "{RETIRED}{}{}",
        row("design.check", "design.check"),
        row("design.verify", "design.check"),
    ))
    .unwrap_err();
    assert_eq!(err.slug, "same-schemas");
    assert!(err.detail.contains("design.check"));
}

#[test]
fn retired_targets_exist() {
    let report = lint(RETIRED).unwrap();
    assert!(
        report
            .warnings
            .iter()
            .any(|s| s.contains("not-due") && s.contains("`run`"))
    );
    let err = lint(&format!("{RETIRED}{}", row("run.show", "run.show"))).unwrap_err();
    assert_eq!(err.slug, "retired-target");
    assert!(err.to_string().contains("run.fork"));
    assert!(lint(&format!("{RETIRED}{}", row("run.fork", "run.fork"))).is_ok());
}

#[test]
fn mechanisms_have_one_name() {
    let mechanisms = [
        "mission.separate",
        "run.fork",
        "telemetry.divergence",
        "autopsy.intervene",
        "autopsy.attribute",
        "nemesis.audit",
        "nemesis.bound",
        "nemesis.hardset",
        "fidelity.modelform",
        "eval.rank-agreement",
        "eval.preanalyze",
        "prove.bundle",
        "prove.verify",
        "prove.list",
        "ground.ingest",
        "ground.identify",
        "ground.score",
        "foundry.size",
        "foundry.optimize",
        "foundry.check",
        "foundry.export",
        "opt.cascade",
        "solve.run",
        "mission.transport",
        "skill.transport",
        "telemetry.export-html",
    ];
    let clusters: BTreeSet<_> = VERBS.iter().map(|entry| entry.cluster).collect();
    let mut not_due = BTreeSet::new();
    for name in mechanisms {
        let (cluster, _) = name.split_once('.').unwrap();
        if clusters.contains(cluster) {
            assert!(
                lookup(name).is_some(),
                "missing `{name}` in active cluster `{cluster}`"
            );
        } else {
            not_due.insert(cluster);
        }
    }
    for cluster in not_due {
        println!("not-due: cluster {cluster}");
    }
    for row in RETIRED_VERBS {
        assert!(lookup(row.name).is_none(), "retired CLI name {}", row.name);
        assert!(
            lookup_tool(&tool_name(row.name)).is_none(),
            "retired MCP name {}",
            row.name
        );
    }
}

#[test]
fn two_verbs_one_engine_allowed() {
    let doc = format!(
        "{RETIRED}{}{}{}{}",
        row("result.revalidate", "result.revalidate"),
        row("mission.revalidate", "mission.revalidate"),
        row("opt.cascade", "opt.cascade"),
        row("solve.run", "solve.run"),
    );
    assert!(lint(&doc).is_ok());
}

#[test]
#[cfg(feature = "handlers")]
fn lint_same_handler() {
    let mut table = frob_verbs::handlers::handler_table();
    table.check_one_name_per_handler().unwrap();
    let handler = table.get("doctor").unwrap().clone();
    table.register("doctor.alias", handler);
    let err = table.check_one_name_per_handler().unwrap_err();
    assert_eq!(err.slug, "same-handler");
    assert!(err.detail.contains("doctor"));
}

#[test]
fn fidelity_rank_has_one_status() {
    assert_ne!(
        lookup("fidelity.rank").is_some(),
        lookup_retired_verb("fidelity.rank").is_some(),
    );
    assert_eq!(
        RETIRED_VERBS.len(),
        11 + usize::from(lookup_retired_verb("fidelity.rank").is_some())
    );
}

#[test]
fn retired_and_unknown_cli_refusals() {
    use frob_verbs::{
        dispatch::{Pipeline, Request},
        envelope::{Tree, embedded_registry},
    };

    let pipeline = Pipeline::new(
        VERBS.to_vec(),
        Default::default(),
        embedded_registry().unwrap().clone(),
    );
    for (name, prefix) in [
        ("autopsy.fork", Some("frob run fork <run>")),
        (
            "nemesis.show",
            Some("frob nemesis hardset --class <class> --json"),
        ),
        ("nosuch.verb", None),
    ] {
        let request = Request::new(name, Tree::Obj(vec![]));
        let answer = pipeline.validate_request(&request).unwrap().unwrap_err();
        assert_eq!(answer.refusal_code(), Some("FROB-REF-801"));
        assert_eq!(answer.exit().code(), 3);
        let repairs = answer
            .tree()
            .get("refusal")
            .and_then(|r| r.get("repairs"))
            .and_then(Tree::as_array)
            .unwrap();
        if let Some(prefix) = prefix {
            let action = repairs[0].get("action").and_then(Tree::as_str).unwrap();
            assert!(action.starts_with(prefix), "{action}");
            assert!(!action.contains("--action show"));
        } else {
            assert!(repairs.is_empty());
        }
    }
}
