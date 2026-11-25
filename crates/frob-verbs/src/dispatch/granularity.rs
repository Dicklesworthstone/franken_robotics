//! Control-plane granularity lint for `run` verbs (bead N32a-04, §32.2).
//!
//! No registry entry's unit of work may be a single simulation step, tick, or kernel
//! launch: control-plane calls act on whole runs, designs, jobs, chunks, or queries.
//! Every `Kind::Run` verb's input schema must declare a required duration, count, or
//! horizon field (or explicitly declare `x-frob-work-unit` for non-step whole-work plans).
//!
//! Determinism D0; prints nothing (L-7); zero `unsafe`.

use std::fmt;

use crate::envelope::Tree;
use crate::registry::{Kind, VerbEntry};

/// Step/tick-granular verb name suffixes refused on `Kind::Run` verbs.
const FORBIDDEN_RUN_SUFFIXES: &[&str] = &[
    ".step",
    ".tick",
    ".substep",
    ".launch-kernel",
    ".kernel-launch",
    ".single-step",
];

/// Input property names recognized as a whole-work duration, count, horizon, or plan unit.
const WORK_UNIT_FIELDS: &[&str] = &[
    "duration",
    "duration_s",
    "horizon",
    "horizon_s",
    "steps",
    "step_count",
    "episodes",
    "worlds",
    "samples",
    "count",
    "iterations",
    "rollouts",
    "epochs",
    "trials",
    "seeds",
    "batch_size",
    "eval_count",
    "plan",
    "plan_id",
    "budget",
];

/// Why a verb failed the control-plane granularity lint.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GranularityError {
    pub verb: String,
    pub reason: String,
}

impl fmt::Display for GranularityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "control-plane granularity lint failed for `{}`: {}",
            self.verb, self.reason
        )
    }
}

impl std::error::Error for GranularityError {}

/// Inspect `entry` and its input JSON Schema text under the control-plane granularity rule.
///
/// Non-`run` verbs pass unconditionally. A `Kind::Run` verb fails if:
/// 1. Its name ends in a single-step / tick / kernel-launch suffix (e.g. `sim.step`), or
/// 2. Its input schema does not require at least one duration, count, horizon, or plan
///    field (listed in `required` and declared under `properties`).
pub fn check_control_plane_granularity(
    entry: &VerbEntry,
    input_schema_json: &str,
) -> Result<(), GranularityError> {
    if entry.kind != Kind::Run {
        return Ok(());
    }

    for suffix in FORBIDDEN_RUN_SUFFIXES {
        if entry.name.ends_with(suffix) {
            return Err(GranularityError {
                verb: entry.name.to_owned(),
                reason: format!(
                    "verb name ends in single-step suffix `{suffix}`; control-plane verbs act on whole runs, not single steps or kernel launches"
                ),
            });
        }
    }

    let doc = Tree::parse(input_schema_json.as_bytes()).map_err(|e| GranularityError {
        verb: entry.name.to_owned(),
        reason: format!(
            "input schema is not valid JSON: {} at `{}`",
            e.message, e.pointer
        ),
    })?;

    let props = doc.get("properties").and_then(Tree::as_object);
    let required: Vec<&str> = doc
        .get("required")
        .and_then(Tree::as_array)
        .map(|items| items.iter().filter_map(Tree::as_str).collect())
        .unwrap_or_default();

    let has_required_work_unit = required.iter().any(|req_name| {
        let is_known_unit = WORK_UNIT_FIELDS.contains(req_name)
            || props.is_some_and(|ps| {
                ps.iter()
                    .find(|(k, _)| k == req_name)
                    .and_then(|(_, schema)| schema.get("x-frob-work-unit"))
                    .and_then(Tree::as_bool)
                    == Some(true)
            });
        let declared_in_props = props.is_some_and(|ps| ps.iter().any(|(k, _)| k == req_name));
        is_known_unit && declared_in_props
    });

    if !has_required_work_unit {
        return Err(GranularityError {
            verb: entry.name.to_owned(),
            reason: "a `run` verb input schema must require at least one duration, count, horizon, or plan work-unit field".to_owned(),
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use frob_core::refusal::RefusalCode;

    use crate::registry::{Milestone, SchemaId, Scope, Tier, WaitDefault};

    use super::*;

    fn synthetic_run_entry(name: &'static str) -> VerbEntry {
        VerbEntry {
            name,
            cluster: "sim",
            kind: Kind::Run,
            tier: Tier::S,
            capabilities: &[Scope("sim:run")],
            input_schema: SchemaId("frob.sim.run.in/1"),
            output_schema: SchemaId("frob.sim.run.out/1"),
            refusals: &[RefusalCode::FrobRef802],
            idempotent: true,
            cost_model: "bounded",
            first_milestone: Milestone::M0,
            task_clusters: &["sim"],
            examples: &["frob sim run quad-a --json"],
            purpose: "Synthetic run entry for granularity lint test.",
            use_when: "Unit test.",
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
            estimable: true,
            destructive: false,
            public: false,
            key_optional: false,
            option_capabilities: &[],
            spends_physics: Some(true),
            released: false,
            params: &[],
            conformance_only: &[],
        }
    }

    #[test]
    fn granularity_lint_catches_step_verb() {
        let step_entry = synthetic_run_entry("sim.step");
        let step_schema = r#"{"type":"object","required":["design"],"additionalProperties":false,"properties":{"design":{"type":"string"}}}"#;
        let err = check_control_plane_granularity(&step_entry, step_schema)
            .expect_err("synthetic sim.step must fail the granularity lint");
        assert_eq!(err.verb, "sim.step");

        // Even with a non-step name, a `run` verb lacking a required duration/count/horizon fails.
        let bare_run = synthetic_run_entry("sim.run");
        let err2 = check_control_plane_granularity(&bare_run, step_schema)
            .expect_err("run verb with no duration/count/horizon must fail");
        assert_eq!(err2.verb, "sim.run");

        // Positive control: `sim.run` requiring `duration_s` passes.
        let valid_schema = r#"{"type":"object","required":["design","duration_s"],"additionalProperties":false,"properties":{"design":{"type":"string"},"duration_s":{"type":"number","minimum":0}}}"#;
        check_control_plane_granularity(&bare_run, valid_schema)
            .expect("sim.run with required duration_s must pass");
    }
}
