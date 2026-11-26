//! Static verb registry table, lookup functions, and build-time lint engine (bead N32a-02).
//!
//! `VERBS`, `TASK_CLUSTERS`, `SCHEMAS`, and `REGISTRY_DIGEST` are generated at build time
//! from `registries/verbs.toml` and `schemas/` into `$OUT_DIR/verbs_table.rs`.

mod arguments;
pub mod codegen;
pub mod derive;
pub mod entry;
pub mod lint;
pub mod names;
pub mod schema_id;

pub use arguments::parse_arguments;
pub use derive::{
    Annotations, CommonProperty, DeriveError, KeyRule, ListedRefusal, MAX_SHORT_DOC_CHARS,
    PROCESS_OPTIONS, RefusalOrigin, RefusalSource, ShadowError, Source, entry_schema_json,
};
pub use entry::{
    Action, AdditionRecord, CountsRecord, Kind, Milestone, OwnedAction, OwnedParamKind,
    OwnedTaskCluster, OwnedVerbEntry, OwnedWaitDefault, ParamKind, ParseError, ParsedRegistry,
    ResolvedAction, RetiredName, Scope, TaskCluster, Tier, VERBS_ENTRY_SCHEMA,
    VERBS_REGISTRY_SCHEMA, VerbEntry, WaitDefault,
};
pub use lint::{
    LintContext, LintError, LintReport, MAX_DESCRIPTION_CHARS, MAX_MCP_WAIT_MS, MAX_PURPOSE_CHARS,
    MAX_TOOLS_PER_TASK_CLUSTER, REGISTERED_ID_PREFIXES, RESERVED_RL_INT, RESERVED_RL11,
    lint_registry, parse_and_lint,
};
pub use names::{BASE_TOOLS, BASE_VERB_NAMES, NameError, cli_path, is_valid_segment, tool_name};
pub use schema_id::{OwnedSchemaId, SchemaId, SchemaIdError, schema_file_name};

/// A retired name and its ordered CLI repair targets. It has no dispatch entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RetiredVerb {
    pub name: &'static str,
    pub use_instead: &'static [&'static str],
    pub reason: &'static str,
}

include!(concat!(env!("OUT_DIR"), "/verbs_table.rs"));

/// `true` iff this build compiled against an alternate registry via `FROB_VERBS_REGISTRY`.
///
/// Checked by `xtask check-generated` (N34p4-03 / BD-022) to reject release builds made
/// against a non-canonical registry override.
pub const BUILT_WITH_OVERRIDE: bool = match option_env!("FROB_VERBS_BUILT_WITH_OVERRIDE") {
    Some(val) => matches!(val.as_bytes(), b"1"),
    None => false,
};

/// Look up a registered verb by exact registry name (`O(log N)` binary search on [`VERBS`]).
#[must_use]
pub fn lookup(name: &str) -> Option<&'static VerbEntry> {
    lookup_in(&VERBS, name)
}

/// Look up the canonical JSON registry row for an exact verb name.
#[must_use]
pub fn lookup_entry_json(name: &str) -> Option<&'static str> {
    let index = VERBS.binary_search_by_key(&name, |entry| entry.name).ok()?;
    VERB_ENTRY_JSON.get(index).copied()
}

/// Look up repair metadata for a retired spelling.
#[must_use]
pub fn lookup_retired_verb(name: &str) -> Option<&'static RetiredVerb> {
    let index = RETIRED_VERBS
        .binary_search_by_key(&name, |row| row.name)
        .ok()?;
    RETIRED_VERBS.get(index)
}

/// Look up a registered verb by its MCP tool name (`"."` and `"-"` mapped to `"_"`).
#[must_use]
pub fn lookup_tool(tool: &str) -> Option<&'static VerbEntry> {
    lookup_tool_in(&VERBS, tool)
}

/// Look up an output schema or an unambiguous input schema by schema identifier.
/// For a specific verb input use [`lookup_input_schema`].
#[must_use]
pub fn lookup_schema(id: &str) -> Option<&'static str> {
    SCHEMAS
        .binary_search_by_key(&id, |(sid, _)| sid.as_str())
        .ok()
        .map(|idx| SCHEMAS[idx].1)
}

/// Look up the served input schema of a verb, with production stripping applied for that verb.
#[must_use]
pub fn lookup_input_schema(name: &str) -> Option<&'static str> {
    let index = VERBS.binary_search_by_key(&name, |entry| entry.name).ok()?;
    VERB_INPUT_SCHEMAS.get(index).copied()
}

/// Binary-search a sorted `VerbEntry` slice by exact verb name.
#[must_use]
pub fn lookup_in<'a>(table: &'a [VerbEntry], name: &str) -> Option<&'a VerbEntry> {
    table
        .binary_search_by_key(&name, |v| v.name)
        .ok()
        .map(|idx| &table[idx])
}

/// Look up a `VerbEntry` in `table` by its MCP tool name.
#[must_use]
pub fn lookup_tool_in<'a>(table: &'a [VerbEntry], tool: &str) -> Option<&'a VerbEntry> {
    table.iter().find(|entry| entry.tool_name() == tool)
}

impl VerbEntry {
    /// Compiled input schema with common properties and cross-action rules (NJL-01).
    pub fn input_schema_with_common_fields(
        &self,
    ) -> Result<crate::envelope::Schema, crate::envelope::EnvelopeError> {
        let id = self.input_schema.as_str();
        let raw = lookup_input_schema(self.name).ok_or_else(|| {
            crate::envelope::EnvelopeError::Schema(format!(
                "input schema `{id}` of `{}` is not embedded",
                self.name
            ))
        })?;
        self.input_schema_with_common_fields_for(raw)
    }

    /// Compiled input schema with common properties for an explicit raw input schema JSON string.
    pub fn input_schema_with_common_fields_for(
        &self,
        raw_input_schema: &str,
    ) -> Result<crate::envelope::Schema, crate::envelope::EnvelopeError> {
        let merged = self
            .input_schema_with_common_fields_json(raw_input_schema)
            .map_err(|e| crate::envelope::EnvelopeError::Schema(e.to_string()))?;
        crate::envelope::Schema::compile_json(&merged)
            .map_err(|e| crate::envelope::EnvelopeError::Schema(e.to_string()))
    }

    /// Per-verb `frob.envelope/1` schema, delegating directly to N32a-03's [`crate::envelope::envelope_schema`].
    pub fn envelope_schema(
        &self,
    ) -> Result<crate::envelope::Schema, crate::envelope::EnvelopeError> {
        crate::envelope::envelope_schema(self)
    }

    /// Per-verb `frob.envelope/1` schema for an explicit output schema JSON string (delegates to N32a-03).
    pub fn envelope_schema_for(
        &self,
        output_schema: &str,
    ) -> Result<crate::envelope::Schema, crate::envelope::EnvelopeError> {
        crate::envelope::envelope_schema_for(self, output_schema)
    }

    /// Full refusal list using the embedded input schema and embedded refusal registry.
    #[must_use]
    pub fn all_refusals(&self) -> Vec<ListedRefusal> {
        let raw = lookup_input_schema(self.name).unwrap_or("{}");
        match crate::envelope::embedded_registry() {
            Ok(reg) => self.all_refusals_with(raw, reg),
            Err(_) => Vec::new(),
        }
    }

    /// Generated `short_doc` description (at most 800 characters) using embedded schema and registry.
    #[must_use]
    pub fn short_doc(&self) -> String {
        let raw = lookup_input_schema(self.name).unwrap_or("{}");
        match crate::envelope::embedded_registry() {
            Ok(reg) => self.short_doc_with(raw, reg),
            Err(_) => String::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, BTreeSet};

    use frob_core::json::{Limits, admit};
    use frob_core::refusal::RefusalCode;
    use frob_core::registry::refusals::RefusalRegistry;

    use super::*;
    use crate::registry::codegen::{compile_registry, compile_registry_bytes};

    const SYNTH_REFUSALS_TOML: &str = r#"schema = "frob.refusals/1"

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
"8xx" = 3
"9xx" = 9
conflict = []
integrity = []
capability = []

[[refusal]]
code = "FROB-REF-101"
meaning = "synthetic 101"
defined_in = "§0"
exit = 3
retry = "after-change"
repairs = ["fix"]
raised_by = ["N32a-02"]

[[refusal]]
code = "FROB-REF-104"
meaning = "synthetic 104"
defined_in = "§0"
exit = 3
retry = "after-change"
repairs = ["fix"]
raised_by = ["N32a-02"]

[[refusal]]
code = "FROB-REF-107"
meaning = "synthetic 107"
defined_in = "§0"
exit = 3
retry = "after-change"
repairs = ["fix"]
raised_by = ["N32a-02"]

[[refusal]]
code = "FROB-REF-110"
meaning = "synthetic 110"
defined_in = "§0"
exit = 3
retry = "after-change"
repairs = ["fix"]
raised_by = ["N32a-02"]

[[refusal]]
code = "FROB-REF-802"
meaning = "synthetic 802"
defined_in = "§32.18"
exit = 3
retry = "after-change"
repairs = ["fix"]
raised_by = ["BD-005"]

[[refusal]]
code = "FROB-REF-816"
meaning = "synthetic 816"
defined_in = "§37.12"
exit = 3
retry = "after-change"
repairs = ["fix"]
raised_by = ["BD-005"]

[[refusal]]
code = "FROB-REF-990"
meaning = "synthetic internal fault"
defined_in = "§32.4"
exit = 9
retry = "never"
repairs = ["run doctor"]
raised_by = ["N32a-03"]
"#;

    const DESIGN_CHECK_IN: &str = r#"{"type":"object","required":["design"],"x-frob-positionals":["design"],"properties":{"design":{"type":"string"},"strict":{"type":"boolean"}}}"#;
    const EMPTY_OBJ_SCHEMA: &str = r#"{"type":"object","properties":{}}"#;
    const REQUEST_DIGEST_OUT: &str = r#"{"type":"object","x-frob-identity":"request-digest","properties":{"id":{"type":"string"}}}"#;
    const SESSION_SHOW_IN: &str =
        r#"{"type":"object","properties":{"all":{"type":"boolean"},"id":{"type":"string"}}}"#;
    const RESULT_SHOW_IN: &str = r#"{"type":"object","required":["result"],"x-frob-positionals":["result"],"properties":{"result":{"type":"string","x-frob-identity-prefix":"r:"}}}"#;
    const SIM_RUN_IN: &str = r#"{"type":"object","required":["design"],"x-frob-positionals":["design"],"properties":{"design":{"type":"string"},"mutant":{"type":"string"},"steps":{"type":"integer"}}}"#;

    fn synth_refusals() -> RefusalRegistry {
        RefusalRegistry::from_bytes(SYNTH_REFUSALS_TOML.as_bytes())
            .expect("synthetic refusal registry loads")
    }

    fn base_schemas() -> BTreeMap<String, Vec<u8>> {
        let mut m = BTreeMap::new();
        for (id, body) in [
            ("frob.design.check.in/1", DESIGN_CHECK_IN),
            ("frob.design.check.out/1", EMPTY_OBJ_SCHEMA),
            ("frob.telemetry.inspect-pixel.in/1", EMPTY_OBJ_SCHEMA),
            ("frob.telemetry.inspect-pixel.out/1", EMPTY_OBJ_SCHEMA),
            ("frob.robot-docs.in/1", EMPTY_OBJ_SCHEMA),
            ("frob.robot-docs.out/1", EMPTY_OBJ_SCHEMA),
            ("frob.robot.docs.in/1", EMPTY_OBJ_SCHEMA),
            ("frob.robot.docs.out/1", EMPTY_OBJ_SCHEMA),
            ("frob.doctor.in/1", EMPTY_OBJ_SCHEMA),
            ("frob.doctor.out/1", EMPTY_OBJ_SCHEMA),
            ("frob.session.show.in/1", SESSION_SHOW_IN),
            ("frob.session.show.out/1", EMPTY_OBJ_SCHEMA),
            ("frob.session.focus.in/1", EMPTY_OBJ_SCHEMA),
            ("frob.session.focus.out/1", EMPTY_OBJ_SCHEMA),
            ("frob.study.plan.in/1", EMPTY_OBJ_SCHEMA),
            ("frob.study.plan.out/1", REQUEST_DIGEST_OUT),
            ("frob.design.mutate.in/1", EMPTY_OBJ_SCHEMA),
            ("frob.design.mutate.out/1", EMPTY_OBJ_SCHEMA),
            ("frob.sim.run.in/1", SIM_RUN_IN),
            ("frob.sim.run.out/1", EMPTY_OBJ_SCHEMA),
            ("frob.spark.cache.in/1", EMPTY_OBJ_SCHEMA),
            ("frob.spark.cache.out/1", EMPTY_OBJ_SCHEMA),
            ("frob.spark.check.in/1", EMPTY_OBJ_SCHEMA),
            ("frob.spark.check.out/1", EMPTY_OBJ_SCHEMA),
            ("frob.result.show.in/1", RESULT_SHOW_IN),
            ("frob.result.show.out/1", EMPTY_OBJ_SCHEMA),
            ("frob.mission.adversarial-pair.in/1", EMPTY_OBJ_SCHEMA),
            ("frob.mission.adversarial-pair.out/1", EMPTY_OBJ_SCHEMA),
        ] {
            m.insert(schema_file_name(id), body.as_bytes().to_vec());
        }
        m
    }

    fn synth_ctx(conformance: bool) -> LintContext {
        LintContext {
            refusals: synth_refusals(),
            schemas: base_schemas(),
            scopes: None,
            append_kind_refusals: true,
            conformance,
        }
    }

    fn specimen_design_check_toml() -> &'static str {
        r#"schema = "frob.verbs/1"

[[verb]]
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
"#
    }

    fn make_valid_read_row(
        name: &str,
        cluster: &str,
        task_cluster: &str,
        schema_prefix: &str,
    ) -> String {
        let cli = cli_path(name).join(" ");
        format!(
            r#"[[verb]]
name = "{name}"
cluster = "{cluster}"
kind = "read"
tier = "S"
capabilities = ["{cluster}:read"]
input_schema = "{schema_prefix}.in/1"
output_schema = "{schema_prefix}.out/1"
refusals = ["FROB-REF-802"]
idempotent = true
cost_model = "static: under 10 ms"
first_milestone = "M0"
task_clusters = ["{task_cluster}"]
examples = ["frob {cli} --json"]
purpose = "Synthetic read verb {name}."
use_when = "Synthetic test case."
owner = "frob-verbs"
"#
        )
    }

    #[test]
    fn table_sorted_and_unique() {
        for win in VERBS.windows(2) {
            assert!(
                win[0].name.as_bytes() < win[1].name.as_bytes(),
                "VERBS must be strictly increasing by name: {} vs {}",
                win[0].name,
                win[1].name
            );
        }
        assert_eq!(BASE_TOOLS, &["session_focus", "job_wait", "job_cancel"]);
    }

    #[test]
    fn name_map_examples() {
        assert_eq!(tool_name("design.check"), "design_check");
        assert_eq!(
            tool_name("telemetry.inspect-pixel"),
            "telemetry_inspect_pixel"
        );
        assert_eq!(tool_name("robot-docs"), "robot_docs");
        assert_eq!(tool_name("eval.rank-agreement"), "eval_rank_agreement");
        assert_eq!(cli_path("design.check"), vec!["design", "check"]);
        assert_eq!(
            cli_path("telemetry.inspect-pixel"),
            vec!["telemetry", "inspect-pixel"]
        );
        assert_eq!(cli_path("robot-docs"), vec!["robot-docs"]);
    }

    #[test]
    fn name_map_injective_on_registry() {
        let mut seen = BTreeSet::new();
        for entry in VERBS.iter() {
            let t = entry.tool_name();
            assert!(seen.insert(t.clone()), "duplicate tool name `{t}`");
        }
        assert_eq!(seen.len(), VERBS.len());
        assert_eq!(tool_name("eval.rank-agreement"), "eval_rank_agreement");
    }

    #[test]
    fn lookup_and_lookup_tool_specimen() {
        const SPECIMEN_TABLE: [VerbEntry; 2] = [
            VerbEntry {
                name: "design.check",
                cluster: "design",
                kind: Kind::Read,
                tier: Tier::S,
                capabilities: &[Scope("design:read")],
                input_schema: SchemaId("frob.design.check.in/1"),
                output_schema: SchemaId("frob.design.check.out/1"),
                refusals: &[RefusalCode::FrobRef802],
                idempotent: true,
                cost_model: "static: under 2 s",
                first_milestone: Milestone::M1,
                task_clusters: &["author", "places"],
                examples: &["frob design check quad-a --json"],
                purpose: "Run preflight checks.",
                use_when: "After any edit.",
                owner: "frob-design",
                wait: None,
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
                params: &[],
                conformance_only: &[],
            },
            VerbEntry {
                name: "telemetry.inspect-pixel",
                cluster: "telemetry",
                kind: Kind::Read,
                tier: Tier::S,
                capabilities: &[Scope("telemetry:read")],
                input_schema: SchemaId("frob.telemetry.inspect-pixel.in/1"),
                output_schema: SchemaId("frob.telemetry.inspect-pixel.out/1"),
                refusals: &[RefusalCode::FrobRef802],
                idempotent: true,
                cost_model: "static: under 100 ms",
                first_milestone: Milestone::M1,
                task_clusters: &["inspect"],
                examples: &["frob telemetry inspect-pixel --json"],
                purpose: "Inspect a rendered pixel.",
                use_when: "Diagnosing optical observations.",
                owner: "frob-sensor",
                wait: None,
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
                params: &[],
                conformance_only: &[],
            },
        ];

        assert_eq!(
            lookup_in(&SPECIMEN_TABLE, "design.check"),
            lookup_tool_in(&SPECIMEN_TABLE, "design_check")
        );
        assert_eq!(
            lookup_tool_in(&SPECIMEN_TABLE, "telemetry_inspect_pixel").map(|v| v.name),
            Some("telemetry.inspect-pixel")
        );
        assert_eq!(lookup_in(&SPECIMEN_TABLE, "unknown.verb"), None);
    }

    #[test]
    fn entry_specimen_parses() {
        let (reg, _rep) =
            parse_and_lint(specimen_design_check_toml().as_bytes(), &synth_ctx(false))
                .expect("specimen design.check must parse and lint");
        assert_eq!(reg.verbs.len(), 1);
        let e = &reg.verbs[0];
        assert_eq!(e.name, "design.check");
        assert_eq!(e.cluster, "design");
        assert_eq!(Kind::parse(&e.kind_raw), Some(Kind::Read));
        assert_eq!(Tier::parse(&e.tier_raw), Some(Tier::S));
        assert_eq!(e.refusals.len(), 4);
        assert_eq!(e.task_clusters, vec!["author", "places"]);
        assert_eq!(e.tool_name(), "design_check");
        assert_eq!(e.cli_path(), vec!["design", "check"]);
    }

    #[test]
    fn rl01_underscore_caught() {
        let doc = format!(
            "schema = \"frob.verbs/1\"\n{}",
            make_valid_read_row("design.check_all", "design", "author", "frob.design.check")
        );
        let err = parse_and_lint(doc.as_bytes(), &synth_ctx(false)).unwrap_err();
        assert_eq!(err.lint, "RL-01");
        assert_eq!(err.slug, "name-grammar");
        assert_eq!(err.entry, "design.check_all");
    }

    #[test]
    fn lint_cluster_mismatch() {
        let doc = format!(
            "schema = \"frob.verbs/1\"\n{}",
            make_valid_read_row("design.check", "sim", "author", "frob.design.check")
        );
        let err = parse_and_lint(doc.as_bytes(), &synth_ctx(false)).unwrap_err();
        assert_eq!(err.lint, "RL-01");
        assert_eq!(err.slug, "cluster-mismatch");
    }

    #[test]
    fn rl02_collision_caught() {
        let doc = format!(
            "schema = \"frob.verbs/1\"\n{}\n{}",
            make_valid_read_row("robot-docs", "operator", "orient", "frob.robot-docs"),
            make_valid_read_row("robot.docs", "robot", "orient", "frob.robot.docs")
        );
        let err = parse_and_lint(doc.as_bytes(), &synth_ctx(false)).unwrap_err();
        assert_eq!(err.lint, "RL-02");
        assert_eq!(err.slug, "tool-name-collision");
        assert_eq!(err.value, "robot_docs");
    }

    #[test]
    fn rl03_retired_name_caught() {
        let doc = format!(
            r#"schema = "frob.verbs/1"
[[retired_name]]
name = "mission.adversarial-pair"
use = ["mission.separate"]
reason = "one name per mechanism"

{}"#,
            make_valid_read_row(
                "mission.adversarial-pair",
                "mission",
                "orient",
                "frob.mission.adversarial-pair"
            )
        );
        let err = parse_and_lint(doc.as_bytes(), &synth_ctx(false)).unwrap_err();
        assert_eq!(err.lint, "RL-03");
        assert_eq!(err.slug, "retired-name");
        assert!(err.detail.contains("mission.separate"));

        // A retired_name row without `reason` must fail the strict parse.
        let missing_reason = r#"schema = "frob.verbs/1"
[[retired_name]]
name = "mission.adversarial-pair"
use = ["mission.separate"]
"#;
        assert!(ParsedRegistry::parse_toml(missing_reason.as_bytes()).is_err());
    }

    #[test]
    fn rl04_eight_tools_caught() {
        let mut ctx = synth_ctx(false);
        let mut doc = String::from("schema = \"frob.verbs/1\"\n");
        for i in 1..=8 {
            let name = format!("design.op{i}");
            let prefix = format!("frob.design.op{i}");
            ctx.schemas.insert(
                schema_file_name(&format!("{prefix}.in/1")),
                EMPTY_OBJ_SCHEMA.as_bytes().to_vec(),
            );
            ctx.schemas.insert(
                schema_file_name(&format!("{prefix}.out/1")),
                EMPTY_OBJ_SCHEMA.as_bytes().to_vec(),
            );
            doc.push_str(&make_valid_read_row(&name, "design", "author", &prefix));
        }
        let err = parse_and_lint(doc.as_bytes(), &ctx).unwrap_err();
        assert_eq!(err.lint, "RL-04");
        assert_eq!(err.slug, "empty-task-clusters");

        // Empty task_clusters fails RL-04.
        let empty_tc = specimen_design_check_toml().replace(
            r#"task_clusters = ["author", "places"]"#,
            "task_clusters = []",
        );
        let err_empty = parse_and_lint(empty_tc.as_bytes(), &synth_ctx(false)).unwrap_err();
        assert_eq!(err_empty.lint, "RL-04");
        assert_eq!(err_empty.slug, "empty-task-clusters");

        // M0 state: no [[task_cluster]] table, rows naming `orient` load and TASK_CLUSTERS is empty.
        let m0_doc = format!(
            "schema = \"frob.verbs/1\"\n{}",
            make_valid_read_row("doctor", "operator", "orient", "frob.doctor")
        );
        let generated = compile_registry_bytes(m0_doc.as_bytes(), &synth_ctx(false))
            .expect("M0 registry without [[task_cluster]] loads");
        assert!(
            generated
                .source
                .contains("pub static TASK_CLUSTERS: [TaskCluster; 0] = [\n];")
        );
    }

    #[test]
    fn lint_schema_id_form() {
        let bad = specimen_design_check_toml().replace(
            r#"input_schema = "frob.design.check.in/1""#,
            r#"input_schema = "design.check.in/1""#,
        );
        let err = parse_and_lint(bad.as_bytes(), &synth_ctx(false)).unwrap_err();
        assert_eq!(err.lint, "RL-05");
        assert_eq!(err.slug, "schema-id-form");
    }

    #[test]
    fn rl06_unknown_refusal_caught() {
        let bad = specimen_design_check_toml().replace("FROB-REF-101", "FROB-REF-999999");
        let err = parse_and_lint(bad.as_bytes(), &synth_ctx(false)).unwrap_err();
        assert_eq!(err.lint, "RL-06");
        assert_eq!(err.slug, "unknown-refusal");
    }

    #[test]
    fn internal_fault_outside_lists() {
        // Committed VERBS table never lists FROB-REF-990.
        for entry in VERBS.iter() {
            for code in entry.refusals {
                assert_ne!(code.as_str(), "FROB-REF-990");
            }
        }
        let bad = specimen_design_check_toml().replace("FROB-REF-101", "FROB-REF-990");
        let err = parse_and_lint(bad.as_bytes(), &synth_ctx(false)).unwrap_err();
        assert_eq!(err.lint, "RL-06");
        assert!(err.detail.contains("internal-fault-not-listable"));
    }

    #[test]
    fn doctor_empty_chapter_refusals_vs_empty_full_list() {
        let doctor_doc = r#"schema = "frob.verbs/1"

[[verb]]
name = "doctor"
cluster = "operator"
kind = "read"
tier = "S"
capabilities = []
public = true
input_schema = "frob.doctor.in/1"
output_schema = "frob.doctor.out/1"
refusals = []
idempotent = true
cost_model = "static: under 500 ms"
first_milestone = "M0"
task_clusters = ["orient"]
examples = ["frob doctor --json"]
purpose = "Check toolchain, pins, devices, and caches."
use_when = "At session start or when diagnosing setup issues."
owner = "frob-verbs"
"#;
        // Loads normally when surface-wide refusals are appended by kind.
        assert!(parse_and_lint(doctor_doc.as_bytes(), &synth_ctx(false)).is_ok());

        // A synthetic entry whose full list would be empty fails RL-06.
        let mut ctx_no_append = synth_ctx(false);
        ctx_no_append.append_kind_refusals = false;
        let err = parse_and_lint(doctor_doc.as_bytes(), &ctx_no_append).unwrap_err();
        assert_eq!(err.lint, "RL-06");
        assert_eq!(err.slug, "unknown-refusal");
    }

    #[test]
    fn rl07_example_must_validate() {
        let bad_flag = specimen_design_check_toml().replace(
            "frob design check quad-a --json",
            "frob design check quad-a --strct --json",
        );
        let err = parse_and_lint(bad_flag.as_bytes(), &synth_ctx(false)).unwrap_err();
        assert_eq!(err.lint, "RL-07");
        assert_eq!(err.slug, "empty-examples");

        let no_examples = specimen_design_check_toml().replace(
            r#"examples = ["frob design check quad-a --json"]"#,
            "examples = []",
        );
        let err_empty = parse_and_lint(no_examples.as_bytes(), &synth_ctx(false)).unwrap_err();
        assert_eq!(err_empty.lint, "RL-07");
        assert_eq!(err_empty.slug, "empty-examples");
    }

    #[test]
    fn rl07_abbreviated_identity() {
        let make_doc = |example: &str| {
            format!(
                r#"schema = "frob.verbs/1"

[[verb]]
name = "result.show"
cluster = "result"
kind = "read"
tier = "S"
capabilities = ["result:read"]
input_schema = "frob.result.show.in/1"
output_schema = "frob.result.show.out/1"
refusals = ["FROB-REF-802"]
idempotent = true
cost_model = "static: under 50 ms"
first_milestone = "M4"
task_clusters = ["inspect"]
examples = ["{example}"]
purpose = "Show a stored run result."
use_when = "Inspecting a completed run."
owner = "frob-chronicle"
"#
            )
        };

        assert!(
            parse_and_lint(
                make_doc("frob result show r:19e2 --json").as_bytes(),
                &synth_ctx(false)
            )
            .is_ok()
        );
        let full_id = format!("frob result show r:{} --json", "a".repeat(64));
        assert!(parse_and_lint(make_doc(&full_id).as_bytes(), &synth_ctx(false)).is_ok());

        let err = parse_and_lint(
            make_doc("frob result show q:19e2 --json").as_bytes(),
            &synth_ctx(false),
        )
        .unwrap_err();
        assert_eq!(err.lint, "RL-07");
    }

    #[test]
    fn rl08_mutate_without_key_caught() {
        let bad = r#"schema = "frob.verbs/1"

[[verb]]
name = "design.mutate"
cluster = "design"
kind = "mutate"
tier = "S"
capabilities = ["design:write"]
input_schema = "frob.design.mutate.in/1"
output_schema = "frob.design.mutate.out/1"
refusals = ["FROB-REF-802"]
idempotent = true
cost_model = "static: under 50 ms"
first_milestone = "M1"
task_clusters = ["author"]
examples = ["frob design mutate --json"]
purpose = "Mutate a design."
use_when = "Editing a design."
owner = "frob-design"
"#;
        let err = parse_and_lint(bad.as_bytes(), &synth_ctx(false)).unwrap_err();
        assert_eq!(err.lint, "RL-08");
        assert_eq!(err.slug, "kind");
    }

    #[test]
    fn rl09_wait_out_of_range_caught() {
        let base_run = r#"schema = "frob.verbs/1"

[[verb]]
name = "sim.run"
cluster = "sim"
kind = "run"
tier = "S"
capabilities = ["sim:run"]
input_schema = "frob.sim.run.in/1"
output_schema = "frob.sim.run.out/1"
refusals = ["FROB-REF-802"]
idempotent = false
cost_model = "sim_estimator"
first_milestone = "M0"
task_clusters = ["simulate"]
examples = ["frob sim run quad-a --key k1 --json"]
purpose = "Run a simulation."
use_when = "Evaluating motion."
owner = "frob-arena"
wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 60000 }
"#;
        let err = parse_and_lint(base_run.as_bytes(), &synth_ctx(false)).unwrap_err();
        assert_eq!(err.lint, "RL-09");
        assert_eq!(err.slug, "mcp-wait-out-of-range");

        let no_wait = base_run.replace(
            "wait = { cli = \"to-completion\", mcp_ms = 5000, mcp_max_ms = 60000 }\n",
            "",
        );
        let err_missing = parse_and_lint(no_wait.as_bytes(), &synth_ctx(false)).unwrap_err();
        assert_eq!(err_missing.lint, "RL-09");
        assert_eq!(err_missing.slug, "run-without-wait");
    }

    #[test]
    fn lint_rl10_empty_capabilities_requires_public() {
        let bad = specimen_design_check_toml()
            .replace(r#"capabilities = ["design:read"]"#, "capabilities = []");
        let err = parse_and_lint(bad.as_bytes(), &synth_ctx(false)).unwrap_err();
        assert_eq!(err.lint, "RL-10");
        assert_eq!(err.slug, "empty-capabilities-not-public");
    }

    #[test]
    fn lint_rl12_milestone_and_tier() {
        let bad_ms = specimen_design_check_toml()
            .replace(r#"first_milestone = "M1""#, r#"first_milestone = "M9""#);
        let err_ms = parse_and_lint(bad_ms.as_bytes(), &synth_ctx(false)).unwrap_err();
        assert_eq!(err_ms.lint, "RL-12");
        assert_eq!(err_ms.slug, "milestone");

        let bad_tier = specimen_design_check_toml().replace(r#"tier = "S""#, r#"tier = "X""#);
        let err_tier = parse_and_lint(bad_tier.as_bytes(), &synth_ctx(false)).unwrap_err();
        assert_eq!(err_tier.lint, "RL-12");
        assert_eq!(err_tier.slug, "tier");
    }

    #[test]
    fn rl13_unknown_field() {
        let bad = format!("{}bogus_field = true\n", specimen_design_check_toml());
        let err = parse_and_lint(bad.as_bytes(), &synth_ctx(false)).unwrap_err();
        assert_eq!(err.lint, "RL-13");
        assert_eq!(err.slug, "unknown-field");
    }

    #[test]
    fn unknown_toml_key_is_error() {
        let bad = specimen_design_check_toml().replace("refusals =", "refusal =");
        let err = ParsedRegistry::parse_toml(bad.as_bytes()).unwrap_err();
        assert!(matches!(
            err,
            ParseError::UnknownField { .. } | ParseError::Toml(_)
        ));
    }

    #[test]
    fn later_field_refused_before_its_bead() {
        for field_line in [
            r#"threats = ["T1.b"]"#,
            "memo = true",
            "reports = true",
            "max_wall_ms = 5000",
            r#"same_engine_as = "design.check""#,
        ] {
            let doc = format!("{}{field_line}\n", specimen_design_check_toml());
            let err = parse_and_lint(doc.as_bytes(), &synth_ctx(false)).unwrap_err();
            assert_eq!(err.lint, "RL-13", "field `{field_line}` must fail RL-13");
            assert_eq!(err.slug, "unknown-field");
        }
    }

    #[test]
    fn rl14_duplicate_name() {
        let doc = format!(
            "schema = \"frob.verbs/1\"\n{}\n{}",
            make_valid_read_row("doctor", "operator", "orient", "frob.doctor"),
            make_valid_read_row("doctor", "operator", "orient", "frob.doctor")
        );
        let err = parse_and_lint(doc.as_bytes(), &synth_ctx(false)).unwrap_err();
        assert_eq!(err.lint, "RL-14");
        assert_eq!(err.slug, "duplicate-name");
    }

    #[test]
    fn lint_unknown_scope() {
        let mut ctx = synth_ctx(false);
        ctx.scopes = Some(["sim:run".to_owned()].into_iter().collect());
        let err = parse_and_lint(specimen_design_check_toml().as_bytes(), &ctx).unwrap_err();
        assert_eq!(err.lint, "RL-15");
        assert_eq!(err.slug, "unknown-scope");
    }

    #[test]
    fn rl16_example_of_another_verb() {
        let bad = specimen_design_check_toml().replace(
            "frob design check quad-a --json",
            "frob sim run quad-a --json",
        );
        let err = parse_and_lint(bad.as_bytes(), &synth_ctx(false)).unwrap_err();
        assert_eq!(err.lint, "RL-16");
        assert_eq!(err.slug, "example-not-this-verb");
    }

    #[test]
    fn rl17_empty_cost_model() {
        let bad = specimen_design_check_toml().replace(
            r#"cost_model = "static: under 2 s for template-sized designs""#,
            r#"cost_model = "   ""#,
        );
        let err = parse_and_lint(bad.as_bytes(), &synth_ctx(false)).unwrap_err();
        assert_eq!(err.lint, "RL-17");
        assert_eq!(err.slug, "empty-cost-model");
    }

    #[test]
    fn rl18_wait_on_read() {
        let bad = format!(
            "{}wait = {{ cli = \"to-completion\", mcp_ms = 5000, mcp_max_ms = 30000 }}\n",
            specimen_design_check_toml()
        );
        let err = parse_and_lint(bad.as_bytes(), &synth_ctx(false)).unwrap_err();
        assert_eq!(err.lint, "RL-18");
        assert_eq!(err.slug, "wait-on-non-run");
    }

    #[test]
    fn rl19_owner_is_surface() {
        for bad_owner in ["frob-cli", "surface/frob-mcp"] {
            let bad = specimen_design_check_toml().replace(
                r#"owner = "frob-design""#,
                &format!(r#"owner = "{bad_owner}""#),
            );
            let err = parse_and_lint(bad.as_bytes(), &synth_ctx(false)).unwrap_err();
            assert_eq!(err.lint, "RL-19");
            assert_eq!(err.slug, "owner-is-surface");
        }
    }

    #[test]
    fn rl20_purpose_201_chars() {
        let long_purpose = "a".repeat(201);
        let bad = specimen_design_check_toml().replace(
            r#"purpose = "Run the preflight stages and the analyses marked for check; return findings with ranked repairs.""#,
            &format!(r#"purpose = "{long_purpose}""#),
        );
        let err = parse_and_lint(bad.as_bytes(), &synth_ctx(false)).unwrap_err();
        assert_eq!(err.lint, "RL-20");
        assert_eq!(err.slug, "purpose-too-long");
    }

    #[test]
    fn rl21_key_optional_on_run() {
        let run_ko = r#"schema = "frob.verbs/1"

[[verb]]
name = "sim.run"
cluster = "sim"
kind = "run"
tier = "S"
capabilities = ["sim:run"]
input_schema = "frob.sim.run.in/1"
output_schema = "frob.sim.run.out/1"
refusals = ["FROB-REF-802"]
idempotent = false
cost_model = "sim_estimator"
first_milestone = "M0"
task_clusters = ["simulate"]
examples = ["frob sim run quad-a --json"]
purpose = "Run a simulation."
use_when = "Testing key_optional refusal."
owner = "frob-arena"
wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }
key_optional = true
"#;
        let err = parse_and_lint(run_ko.as_bytes(), &synth_ctx(false)).unwrap_err();
        assert_eq!(err.lint, "RL-21");
        assert_eq!(err.slug, "key-optional-not-content-addressed");

        // Plain mutate without x-frob-identity = "request-digest" fails RL-21.
        let bad_mutate = r#"schema = "frob.verbs/1"

[[verb]]
name = "design.mutate"
cluster = "design"
kind = "mutate"
tier = "S"
capabilities = ["design:write"]
input_schema = "frob.design.mutate.in/1"
output_schema = "frob.design.mutate.out/1"
refusals = ["FROB-REF-802"]
idempotent = false
cost_model = "static: under 50 ms"
first_milestone = "M1"
task_clusters = ["author"]
examples = ["frob design mutate --json"]
purpose = "Mutate design."
use_when = "Editing."
owner = "frob-design"
key_optional = true
"#;
        let err_mut = parse_and_lint(bad_mutate.as_bytes(), &synth_ctx(false)).unwrap_err();
        assert_eq!(err_mut.lint, "RL-21");
        assert_eq!(err_mut.slug, "key-optional-not-content-addressed");

        // `session.focus` and content-addressed `study.plan` both pass RL-21.
        let allowed = r#"schema = "frob.verbs/1"

[[verb]]
name = "session.focus"
cluster = "session"
kind = "mutate"
tier = "S"
capabilities = []
public = true
input_schema = "frob.session.focus.in/1"
output_schema = "frob.session.focus.out/1"
refusals = ["FROB-REF-802"]
idempotent = false
cost_model = "static: under 5 ms"
first_milestone = "M1"
task_clusters = ["base"]
examples = ["frob session focus --json"]
purpose = "Set active task cluster."
use_when = "Switching activities."
owner = "frob-verbs"
key_optional = true

[[verb]]
name = "study.plan"
cluster = "study"
kind = "mutate"
tier = "S"
capabilities = ["study:write"]
input_schema = "frob.study.plan.in/1"
output_schema = "frob.study.plan.out/1"
refusals = ["FROB-REF-802"]
idempotent = false
cost_model = "static: under 50 ms"
first_milestone = "M2"
task_clusters = ["studies"]
examples = ["frob study plan --json"]
purpose = "Create a content-addressed study plan."
use_when = "Planning sweeps."
owner = "frob-arena"
key_optional = true
"#;
        let (reg, _) = parse_and_lint(allowed.as_bytes(), &synth_ctx(false))
            .expect("session.focus and study.plan pass RL-21");
        assert_eq!(reg.verbs.len(), 2);
        assert!(reg.verbs.iter().all(|v| v.key_optional));
    }

    #[test]
    fn rl22_option_not_an_input_caught() {
        let make_doc = |opt: &str| {
            format!(
                r#"schema = "frob.verbs/1"

[[verb]]
name = "session.show"
cluster = "session"
kind = "read"
tier = "S"
capabilities = []
public = true
input_schema = "frob.session.show.in/1"
output_schema = "frob.session.show.out/1"
refusals = ["FROB-REF-802"]
idempotent = true
cost_model = "static: under 20 ms"
first_milestone = "M1"
task_clusters = ["orient"]
examples = ["frob session show --json"]
purpose = "Show session state."
use_when = "Checking budgets."
owner = "frob-verbs"
option_capabilities = {{ {opt} = ["session:read"] }}
"#
            )
        };
        assert!(parse_and_lint(make_doc("all").as_bytes(), &synth_ctx(false)).is_ok());
        let err = parse_and_lint(make_doc("everyone").as_bytes(), &synth_ctx(false)).unwrap_err();
        assert_eq!(err.lint, "RL-22");
        assert_eq!(err.slug, "unknown-option");
    }

    #[test]
    fn rl23_action_paginated_on_run() {
        let bad_action = r#"schema = "frob.verbs/1"

[[verb]]
name = "spark.check"
cluster = "spark"
kind = "read"
tier = "S"
capabilities = ["spark:read"]
input_schema = "frob.spark.check.in/1"
output_schema = "frob.spark.check.out/1"
refusals = ["FROB-REF-802"]
idempotent = true
cost_model = "static: under 50 ms"
first_milestone = "M0"
task_clusters = ["kernels"]
examples = ["frob spark check --json"]
purpose = "Check kernels."
use_when = "Verifying kernels."
owner = "frob-kernels"
wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }
actions = [{ name = "bench", kind = "run", capabilities = ["spark:run"], paginated = true }]
"#;
        let err = parse_and_lint(bad_action.as_bytes(), &synth_ctx(false)).unwrap_err();
        assert_eq!(err.lint, "RL-23");
        assert_eq!(err.slug, "action-paginated-not-read");

        // Entry-level paginated = true on a `run` verb also fails RL-23.
        let bad_entry = r#"schema = "frob.verbs/1"

[[verb]]
name = "sim.run"
cluster = "sim"
kind = "run"
tier = "S"
capabilities = ["sim:run"]
input_schema = "frob.sim.run.in/1"
output_schema = "frob.sim.run.out/1"
refusals = ["FROB-REF-802"]
idempotent = false
cost_model = "sim_estimator"
first_milestone = "M0"
task_clusters = ["simulate"]
examples = ["frob sim run quad-a --key k1 --json"]
purpose = "Run simulation."
use_when = "Testing entry paginated rule."
owner = "frob-arena"
wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }
paginated = true
"#;
        let err_entry = parse_and_lint(bad_entry.as_bytes(), &synth_ctx(false)).unwrap_err();
        assert_eq!(err_entry.lint, "RL-23");
        assert_eq!(err_entry.slug, "action-paginated-not-read");

        // Read action `list` with paginated = true passes.
        let ok_doc = r#"schema = "frob.verbs/1"

[[verb]]
name = "spark.check"
cluster = "spark"
kind = "read"
tier = "S"
capabilities = ["spark:read"]
input_schema = "frob.spark.check.in/1"
output_schema = "frob.spark.check.out/1"
refusals = ["FROB-REF-802"]
idempotent = true
cost_model = "static: under 50 ms"
first_milestone = "M0"
task_clusters = ["kernels"]
examples = ["frob spark check --action list --json"]
purpose = "Check or list kernels."
use_when = "Inspecting kernels."
owner = "frob-kernels"
actions = [{ name = "list", kind = "read", capabilities = ["spark:read"], paginated = true }]
"#;
        assert!(parse_and_lint(ok_doc.as_bytes(), &synth_ctx(false)).is_ok());
    }

    #[test]
    fn rl24_conformance_only_unknown_property() {
        let make_doc = |prop: &str| {
            format!(
                r#"schema = "frob.verbs/1"

[[verb]]
name = "sim.run"
cluster = "sim"
kind = "run"
tier = "S"
capabilities = ["sim:run"]
input_schema = "frob.sim.run.in/1"
output_schema = "frob.sim.run.out/1"
refusals = ["FROB-REF-802"]
idempotent = false
cost_model = "sim_estimator"
first_milestone = "M0"
task_clusters = ["simulate"]
examples = ["frob sim run quad-a --key k1 --json"]
purpose = "Run simulation."
use_when = "Running trajectories."
owner = "frob-arena"
wait = {{ cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }}
conformance_only = ["{prop}"]
"#
            )
        };

        // Unknown property fails RL-24.
        let err_unk =
            parse_and_lint(make_doc("nonexistent").as_bytes(), &synth_ctx(false)).unwrap_err();
        assert_eq!(err_unk.lint, "RL-24");
        assert_eq!(err_unk.slug, "conformance-only-not-an-input");

        // Required/positional property (`design`) fails RL-24.
        let err_req = parse_and_lint(make_doc("design").as_bytes(), &synth_ctx(false)).unwrap_err();
        assert_eq!(err_req.lint, "RL-24");
        assert_eq!(err_req.slug, "conformance-only-not-an-input");

        // Optional non-positional property (`mutant`) passes RL-24.
        assert!(parse_and_lint(make_doc("mutant").as_bytes(), &synth_ctx(false)).is_ok());
    }

    fn sim_run_specimen() -> &'static str {
        r#"schema = "frob.verbs/1"

[[verb]]
name = "sim.run"
cluster = "sim"
kind = "run"
tier = "S"
capabilities = ["sim:run"]
input_schema = "frob.sim.run.in/1"
output_schema = "frob.sim.run.out/1"
refusals = ["FROB-REF-802"]
idempotent = false
cost_model = "sim_estimator"
first_milestone = "M0"
task_clusters = ["simulate"]
examples = ["frob sim run quad-a --key k1 --json"]
purpose = "Run simulation."
use_when = "Running trajectories."
owner = "frob-arena"
wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }
conformance_only = ["mutant"]
"#
    }

    #[test]
    fn conformance_only_stripped() {
        let doc = sim_run_specimen();
        let (reg_prod, rep_prod) =
            parse_and_lint(doc.as_bytes(), &synth_ctx(false)).expect("prod build");
        let (reg_conf, rep_conf) =
            parse_and_lint(doc.as_bytes(), &synth_ctx(true)).expect("conformance build");

        let prod_schema = admit(
            rep_prod.embedded_schemas["frob.sim.run.in/1"].as_bytes(),
            &Limits::default(),
        )
        .expect("prod schema JSON");
        let conf_schema = admit(
            rep_conf.embedded_schemas["frob.sim.run.in/1"].as_bytes(),
            &Limits::default(),
        )
        .expect("conf schema JSON");

        assert!(
            prod_schema
                .value()
                .get("properties")
                .and_then(|p| p.get("mutant"))
                .is_none(),
            "non-conformance build must strip `mutant`"
        );
        assert!(
            conf_schema
                .value()
                .get("properties")
                .and_then(|p| p.get("mutant"))
                .is_some(),
            "conformance build must preserve `mutant`"
        );
        assert_eq!(
            reg_prod.registry_digest().unwrap(),
            reg_conf.registry_digest().unwrap(),
            "REGISTRY_DIGEST must be identical with and without conformance feature"
        );
    }

    #[test]
    fn shared_schema_strips_are_per_use() {
        let mut reg = ParsedRegistry::parse_toml(sim_run_specimen().as_bytes()).unwrap();
        let mut other = reg.verbs[0].clone();
        reg.verbs[0].output_schema = other.input_schema.clone();
        other.name = "sim.other".into();
        other.examples[0] = other.examples[0].replace("sim run", "sim other");
        other.conformance_only = vec!["steps".into()];
        reg.verbs.push(other);

        let report = lint_registry(&reg, &synth_ctx(false)).unwrap();
        let run = &report.embedded_inputs["sim.run"];
        let other = &report.embedded_inputs["sim.other"];
        let output = &report.embedded_schemas["frob.sim.run.in/1"];
        assert!(!run.contains("\"mutant\"") && run.contains("\"steps\""));
        assert!(other.contains("\"mutant\"") && !other.contains("\"steps\""));
        assert!(output.contains("\"mutant\"") && output.contains("\"steps\""));
    }

    #[test]
    fn rl07_validates_served_example_schemas() {
        let mut ctx = synth_ctx(false);
        ctx.schemas.insert(
            schema_file_name("frob.sim.run.in/1"),
            include_bytes!("../../../../schemas/verbs/frob.sim.run.in.1.json").to_vec(),
        );
        let mut reg = ParsedRegistry::parse_toml(sim_run_specimen().as_bytes()).unwrap();
        reg.verbs[0].conformance_only = vec!["perturb_ulp".into()];
        reg.verbs[0].actions.push(OwnedAction {
            name: "inspect".into(),
            kind_raw: "read".into(),
            capabilities: vec!["sim:read".into()],
            paginated: true,
            destructive_explicit: None,
        });
        for (args, detail) in [
            ("--seconds not-a-time --key k1", "/pattern"),
            ("--limit 2 --key k1", "/then/properties/limit/false"),
            ("--perturb-ulp q0 --key k1", "unknown flag"),
        ] {
            reg.verbs[0].examples[0] = format!("frob sim run g1 --tier referee {args} --json");
            let err = lint_registry(&reg, &ctx).unwrap_err();
            assert_eq!((err.entry.as_str(), err.lint), ("sim.run", "RL-07"));
            assert!(err.detail.contains(detail), "{err}");
        }
        ctx.conformance = true;
        lint_registry(&reg, &ctx).unwrap();
        reg.verbs[0].examples[0] =
            "frob sim run g1 --tier referee --action inspect --limit 2".into();
        ctx.conformance = false;
        lint_registry(&reg, &ctx).unwrap();
    }

    #[test]
    fn top_level_tables_closed() {
        let valid = r#"schema = "frob.verbs/1"

[counts]
total = 0
clusters = 0
by_kind = { read = 0, run = 0, mutate = 0, admin = 0 }

[[addition]]
name = "artifact.import"
added_by = "VERBS-artifact"
defined_in = "§34.9"
"#;
        let reg = ParsedRegistry::parse_toml(valid.as_bytes()).expect("valid top-level tables");
        assert!(reg.counts.is_some());
        assert_eq!(reg.additions.len(), 1);

        let bad_top = r#"schema = "frob.verbs/1"

[[retired]]
name = "old.verb"
"#;
        let err = parse_and_lint(bad_top.as_bytes(), &synth_ctx(false)).unwrap_err();
        assert_eq!(err.lint, "RL-13");
        assert_eq!(err.slug, "unknown-field");
    }

    #[test]
    fn two_kind_verb_actions() {
        let make_cache = |extra: &str| {
            format!(
                r#"schema = "frob.verbs/1"

[[verb]]
name = "spark.cache"
cluster = "spark"
kind = "read"
tier = "S"
capabilities = ["spark:read"]
input_schema = "frob.spark.cache.in/1"
output_schema = "frob.spark.cache.out/1"
refusals = ["FROB-REF-802"]
idempotent = true
cost_model = "static: under 50 ms"
first_milestone = "M1"
task_clusters = ["kernels"]
examples = ["frob spark cache --json"]
purpose = "Inspect or evict kernel cache entries."
use_when = "Managing compiled kernels."
owner = "frob-kernels"
actions = [{{ name = "evict", kind = "admin", capabilities = ["spark:admin"]{extra} }}]
"#
            )
        };

        // Default `evict` without `destructive` has destructive == true.
        let (reg_default, _) = parse_and_lint(make_cache("").as_bytes(), &synth_ctx(false))
            .expect("default evict passes");
        let entry = &reg_default.verbs[0];
        let (k_def, cap_def, _, dest_def) = entry.resolve_owned_action(None).expect("default");
        assert_eq!(k_def, Kind::Read);
        assert_eq!(cap_def, vec!["spark:read"]);
        assert!(!dest_def);

        let (k_ev, cap_ev, _, dest_ev) = entry.resolve_owned_action(Some("evict")).expect("evict");
        assert_eq!(k_ev, Kind::Admin);
        assert_eq!(cap_ev, vec!["spark:admin"]);
        assert!(dest_ev);

        // Unnamed default action cannot be addressed as action = "show" (802).
        assert_eq!(
            entry.resolve_owned_action(Some("show")),
            Err(RefusalCode::FrobRef802)
        );

        // Explicit `destructive = false` overrides the default.
        let (reg_false, _) = parse_and_lint(
            make_cache(", destructive = false").as_bytes(),
            &synth_ctx(false),
        )
        .expect("destructive = false passes");
        let (_, _, _, dest_false) = reg_false.verbs[0]
            .resolve_owned_action(Some("evict"))
            .expect("evict false");
        assert!(!dest_false);

        // `destructive` on a `read` action fails RL-08.
        let bad_read_dest = r#"schema = "frob.verbs/1"

[[verb]]
name = "spark.cache"
cluster = "spark"
kind = "read"
tier = "S"
capabilities = ["spark:read"]
input_schema = "frob.spark.cache.in/1"
output_schema = "frob.spark.cache.out/1"
refusals = ["FROB-REF-802"]
idempotent = true
cost_model = "static: under 50 ms"
first_milestone = "M1"
task_clusters = ["kernels"]
examples = ["frob spark cache --json"]
purpose = "Inspect cache."
use_when = "Testing RL-08."
owner = "frob-kernels"
actions = [{ name = "stats", kind = "read", capabilities = ["spark:read"], destructive = false }]
"#;
        let err = parse_and_lint(bad_read_dest.as_bytes(), &synth_ctx(false)).unwrap_err();
        assert_eq!(err.lint, "RL-08");
        assert_eq!(err.slug, "kind");
    }

    #[test]
    fn prop_entry_roundtrip() {
        let mut rng = 0x32A0_0200_2026_1124u64;
        let mut next = || {
            rng ^= rng << 13;
            rng ^= rng >> 7;
            rng ^= rng << 17;
            rng
        };

        let clusters = ["design", "sim", "spark", "eval", "scene", "operator"];
        let tiers = ["S", "F", "M"];
        let milestones = ["M0", "M1", "M2", "M3", "M4", "M5", "M6", "M7", "M8"];

        for i in 0..128usize {
            let cluster = clusters[(next() as usize) % clusters.len()];
            let name = if cluster == "operator" {
                format!("op-{i}")
            } else {
                format!("{cluster}.verb-{i}")
            };
            let kind_idx = (next() as usize) % 4;
            let (kind_raw, idempotent, wait, destructive_explicit) = match kind_idx {
                0 => ("read", true, None, None),
                1 => (
                    "run",
                    false,
                    Some(OwnedWaitDefault {
                        cli: String::from("to-completion"),
                        mcp_ms: 5000,
                        mcp_max_ms: 30000,
                    }),
                    None,
                ),
                2 => ("mutate", false, None, None),
                _ => ("admin", true, None, Some((next() & 1) == 1)),
            };
            let tier_raw = tiers[(next() as usize) % tiers.len()].to_owned();
            let first_milestone_raw = milestones[(next() as usize) % milestones.len()].to_owned();
            let cli = cli_path(&name).join(" ");
            let mut params = BTreeMap::new();
            if (next() & 1) == 1 {
                params.insert(
                    String::from("design"),
                    OwnedParamKind::ArtifactInput(String::from("FMT-FBOT")),
                );
            }
            if (next() & 1) == 1 {
                params.insert(String::from("raw"), OwnedParamKind::ForeignFile);
            }
            let entry = OwnedVerbEntry {
                name: name.clone(),
                cluster: cluster.to_owned(),
                kind_raw: kind_raw.to_owned(),
                tier_raw,
                capabilities: vec![format!("{cluster}:read")],
                input_schema: format!("frob.{name}.in/1"),
                output_schema: format!("frob.{name}.out/1"),
                refusals: vec![String::from("FROB-REF-802")],
                idempotent,
                cost_model: String::from("static: under 10 ms"),
                first_milestone_raw,
                task_clusters: vec![String::from("orient")],
                examples: vec![format!("frob {cli} --json")],
                purpose: format!("Generated entry {i}."),
                use_when: String::from("Property round-trip test."),
                owner: String::from("frob-verbs"),
                wait,
                actions: Vec::new(),
                schema_version: 1,
                paginated: kind_idx == 0 && (next() & 1) == 1,
                exports: (next() & 1) == 1,
                estimable: (next() & 1) == 1,
                destructive_explicit,
                public: false,
                key_optional: false,
                option_capabilities: BTreeMap::new(),
                spends_physics: if kind_idx == 1 { Some(true) } else { None },
                released: (next() & 1) == 1,
                params,
                conformance_only: Vec::new(),
            };

            let doc = format!("schema = \"frob.verbs/1\"\n\n{}", entry.to_toml_string());
            let parsed = ParsedRegistry::parse_toml(doc.as_bytes())
                .expect("serialized entry must parse back");
            assert_eq!(parsed.verbs.len(), 1);
            assert_eq!(
                parsed.verbs[0], entry,
                "round-trip drifted at iteration {i}"
            );
        }
    }

    #[test]
    fn permutation_invariance_and_deterministic_codegen() {
        let row_a = make_valid_read_row("doctor", "operator", "orient", "frob.doctor");
        let row_b = make_valid_read_row("robot-docs", "operator", "orient", "frob.robot-docs");
        let doc_ab = format!("schema = \"frob.verbs/1\"\n\n{row_a}\n{row_b}");
        let doc_ba = format!("schema = \"frob.verbs/1\"\n\n{row_b}\n{row_a}");

        let ctx = synth_ctx(false);
        let gen1 = compile_registry_bytes(doc_ab.as_bytes(), &ctx).expect("compile ab");
        let gen2 = compile_registry_bytes(doc_ba.as_bytes(), &ctx).expect("compile ba");
        let reg_ab = ParsedRegistry::parse_toml(doc_ab.as_bytes()).unwrap();
        let gen3 = compile_registry(&reg_ab, &ctx).expect("compile ab again");

        assert_eq!(gen1.digest, gen2.digest);
        assert_eq!(gen1.source, gen2.source);
        assert_eq!(gen1.source, gen3.source);
    }
}
