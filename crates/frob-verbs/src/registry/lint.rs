//! Build-time and unit-testable registry lints (`RL-01`..`RL-24`, reserved `RL-11` & `RL-INT`)
//! for `registries/verbs.toml` (bead N32a-02).
//!
//! Every lint returns a structured [`LintError`] naming the offending entry, the lint
//! identifier (`RL-XX`), the canonical error slug, and the offending value.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use frob_core::canon::{CanonWriter, Value as CanonValue, write_json};
use frob_core::json::{Json, Limits, admit};
use frob_core::registry::refusals::RefusalRegistry;

use crate::envelope::{Schema, Tree};

use super::arguments::parse_arguments;
use super::entry::{Kind, Milestone, OwnedVerbEntry, ParseError, ParsedRegistry, Tier};
use super::names::{base_tool_owner, is_base_verb, tool_name, validate_verb_name};
use super::schema_id::OwnedSchemaId;

/// Maximum allowed number of non-base verbs sharing any single task cluster (RL-04).
pub const MAX_TOOLS_PER_TASK_CLUSTER: usize = 7;

/// Maximum allowed `mcp_max_ms` ceiling (OPEN-TETHER-3 fail-closed bound, RL-09).
pub const MAX_MCP_WAIT_MS: i64 = 30_000;

/// Maximum character length of an entry's `purpose` field (RL-20).
pub const MAX_PURPOSE_CHARS: usize = 200;

/// Reserved maximum length of NJL-01's derived `short_doc` (RL-11, wired with NJL-01).
pub const MAX_DESCRIPTION_CHARS: usize = 800;

/// Reserved lint identifier & error slug for NJL-01's `short_doc` length check.
pub const RESERVED_RL11: (&str, &str) = ("RL-11", "description-too-long");

/// Reserved lint identifier & error slug for N32a-07's integer schema type check.
pub const RESERVED_RL_INT: (&str, &str) = ("RL-INT", "integer-type");

/// Process-level CLI flags accepted on every verb invocation (never an input property).
pub const PROCESS_FLAGS: &[(&str, bool)] = &[
    ("json", false),
    ("help", false),
    ("session", true),
    ("registries-root", true),
    ("session-token-file", true),
    ("no-daemon", false),
];

/// Registered FrankenRobotics identity prefixes admitted as abbreviated identities in RL-07.
pub const REGISTERED_ID_PREFIXES: &[&str] = &[
    "d", "m", "s", "k", "p", "e", "r", "j", "clm", "ev", "sr", "c", "b",
];

/// Structured failure returned by [`lint_registry`] or [`parse_and_lint`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LintError {
    pub entry: String,
    pub lint: &'static str,
    pub slug: &'static str,
    pub value: String,
    pub detail: String,
}

impl LintError {
    #[must_use]
    pub fn new(
        entry: impl Into<String>,
        lint: &'static str,
        slug: &'static str,
        value: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            entry: entry.into(),
            lint,
            slug,
            value: value.into(),
            detail: detail.into(),
        }
    }
}

impl fmt::Display for LintError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "entry `{}` failed {} ({}) on value `{}`: {}",
            self.entry, self.lint, self.slug, self.value, self.detail
        )
    }
}

impl std::error::Error for LintError {}

impl From<ParseError> for LintError {
    fn from(err: ParseError) -> Self {
        match err {
            ParseError::UnknownField {
                entry,
                table,
                key,
                line,
                column,
            } => Self::new(
                entry,
                "RL-13",
                "unknown-field",
                key.clone(),
                format!("{line}:{column}: unknown key `{key}` in closed table `{table}`"),
            ),
            ParseError::InvalidSchema { observed } => Self::new(
                "<root>",
                "RL-13",
                "invalid-schema",
                observed.clone(),
                format!("unexpected registry schema `{observed}`"),
            ),
            ParseError::InvalidParamKind {
                entry,
                param,
                value,
                reason,
            } => Self::new(
                entry,
                "RL-13",
                "invalid-param-kind",
                value,
                format!("params.{param}: {reason}"),
            ),
            ParseError::Toml(te) => Self::new(
                "<registry>",
                "RL-13",
                "toml-parse",
                format!("{}:{}", te.line, te.column),
                te.to_string(),
            ),
        }
    }
}

/// External context needed to validate a parsed verb registry.
#[derive(Debug, Clone)]
pub struct LintContext {
    /// Loaded refusal registry (`frob-core`) for RL-06 checks.
    pub refusals: RefusalRegistry,
    /// Map from schema file name (e.g. `frob.design.check.in.1.json`) to raw UTF-8 JSON bytes.
    pub schemas: BTreeMap<String, Vec<u8>>,
    /// Known capability scopes (`registries/scopes.toml` once M2 lands; `None` emits warnings only).
    pub scopes: Option<BTreeSet<String>>,
    /// Whether kind-level surface-wide refusals are appended when checking RL-06 non-empty full list.
    pub append_kind_refusals: bool,
    /// Whether the `conformance` feature is enabled (retains `conformance_only` schema properties).
    pub conformance: bool,
}

impl LintContext {
    /// Construct a default context backed by `frob-core`'s embedded refusal registry.
    pub fn with_embedded_refusals(
        schemas: BTreeMap<String, Vec<u8>>,
        scopes: Option<BTreeSet<String>>,
        conformance: bool,
    ) -> Result<Self, LintError> {
        let refusals = RefusalRegistry::load().map_err(|e| {
            LintError::new(
                "<refusals>",
                "RL-06",
                "unknown-refusal",
                "registries/refusals.toml",
                e.to_string(),
            )
        })?;
        Ok(Self {
            refusals,
            schemas,
            scopes,
            append_kind_refusals: true,
            conformance,
        })
    }
}

/// Result returned when all active registry lints succeed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LintReport {
    /// Non-fatal warnings (for example RL-15 scope proposals before `registries/scopes.toml` lands at M2).
    pub warnings: Vec<String>,
    /// Output schemas and inputs with an unambiguous schema ID.
    pub embedded_schemas: BTreeMap<String, String>,
    /// Served input schema JSON by verb name, with per-verb stripping in production.
    pub embedded_inputs: BTreeMap<String, String>,
    /// Largest non-base task-cluster tool count observed across entries.
    pub largest_task_cluster: usize,
    /// Distinct registry cluster count.
    pub cluster_count: usize,
    /// Longest generated description in Unicode code points.
    pub longest_description: usize,
}

/// Parse raw `verbs.toml` bytes and run all active registry lints.
pub fn parse_and_lint(
    bytes: &[u8],
    ctx: &LintContext,
) -> Result<(ParsedRegistry, LintReport), LintError> {
    let reg = ParsedRegistry::parse_toml(bytes).map_err(LintError::from)?;
    let report = lint_registry(&reg, ctx)?;
    Ok((reg, report))
}

/// Validate a parsed registry against all active `N32a-02` lints (`RL-01`..`RL-24`, except
/// `RL-11` wired by NJL-01 and `RL-INT` wired by N32a-07).
pub fn lint_registry(reg: &ParsedRegistry, ctx: &LintContext) -> Result<LintReport, LintError> {
    let mut warnings = Vec::new();
    let mut embedded_schemas: BTreeMap<String, String> = BTreeMap::new();
    let mut embedded_inputs = BTreeMap::new();
    let mut input_by_id: BTreeMap<String, Option<String>> = BTreeMap::new();
    let mut longest_description = 0;

    let mut seen_names: BTreeSet<&str> = BTreeSet::new();
    let mut seen_tools: BTreeMap<String, &str> = BTreeMap::new();
    let mut seen_schema_pairs: BTreeMap<(&str, &str), &str> = BTreeMap::new();
    let mut retired_tools = BTreeMap::new();
    let mut task_cluster_counts: BTreeMap<&str, usize> = BTreeMap::new();
    let mut clusters: BTreeSet<&str> = BTreeSet::new();

    for ret in &reg.retired_names {
        if ret.reason.trim().is_empty() {
            return Err(LintError::new(
                &ret.name,
                "RL-03",
                "retired-name",
                &ret.name,
                "retired_name entry requires a non-empty reason",
            ));
        }
        if let Some(prior) = retired_tools.insert(tool_name(&ret.name), ret) {
            return Err(LintError::new(
                &ret.name,
                "RL-03",
                "retired-name",
                &ret.name,
                format!("duplicate retired tool name also used by `{}`", prior.name),
            ));
        }
    }

    // Track which entries strip which `conformance_only` properties from a given input schema.

    for entry in &reg.verbs {
        // RL-14: duplicate verb name.
        if !seen_names.insert(entry.name.as_str()) {
            return Err(LintError::new(
                &entry.name,
                "RL-14",
                "duplicate-name",
                &entry.name,
                "duplicate verb name in registry",
            ));
        }

        // RL-01: name grammar and cluster match.
        if let Err(err) = validate_verb_name(&entry.name, &entry.cluster) {
            match err {
                super::names::NameError::Grammar { name, reason } => {
                    return Err(LintError::new(
                        &entry.name,
                        "RL-01",
                        "name-grammar",
                        name,
                        reason,
                    ));
                }
                super::names::NameError::ClusterMismatch {
                    expected_cluster,
                    observed_cluster,
                    ..
                } => {
                    return Err(LintError::new(
                        &entry.name,
                        "RL-01",
                        "cluster-mismatch",
                        observed_cluster,
                        format!(
                            "expected cluster `{expected_cluster}` for verb `{}`",
                            entry.name
                        ),
                    ));
                }
            }
        }
        clusters.insert(entry.cluster.as_str());

        // RL-02: one-to-one MCP tool name map over all entries and the three base tools.
        let tname = tool_name(&entry.name);
        if let Some(prev) = seen_tools.insert(tname.clone(), entry.name.as_str()) {
            return Err(LintError::new(
                &entry.name,
                "RL-02",
                "tool-name-collision",
                tname,
                format!("collides with registered verb `{prev}`"),
            ));
        }
        if let Some(owner_verb) = base_tool_owner(&tname)
            && entry.name != owner_verb
        {
            return Err(LintError::new(
                &entry.name,
                "RL-02",
                "tool-name-collision",
                tname,
                format!("collides with base tool owned by `{owner_verb}`"),
            ));
        }

        // RL-03: retired names and shared (input_schema, output_schema) pairs.
        if let Some(retired) = retired_tools.get(&tname) {
            return Err(LintError::new(
                &entry.name,
                "RL-03",
                "retired-name",
                &retired.name,
                format!(
                    "retired verb or tool name (use {:?}; reason: {})",
                    retired.use_instead, retired.reason
                ),
            ));
        }
        if let Some(prev) = seen_schema_pairs.insert(
            (entry.input_schema.as_str(), entry.output_schema.as_str()),
            entry.name.as_str(),
        ) {
            return Err(LintError::new(
                &entry.name,
                "RL-03",
                "same-schemas",
                format!("({}, {})", entry.input_schema, entry.output_schema),
                format!("shares both input and output schemas with `{prev}`"),
            ));
        }

        // RL-12: tier and first_milestone validity.
        if Tier::parse(&entry.tier_raw).is_none() {
            return Err(LintError::new(
                &entry.name,
                "RL-12",
                "tier",
                &entry.tier_raw,
                "tier must be one of `S`, `F`, or `M`",
            ));
        }
        if Milestone::parse(&entry.first_milestone_raw).is_none() {
            return Err(LintError::new(
                &entry.name,
                "RL-12",
                "milestone",
                &entry.first_milestone_raw,
                "first_milestone must be one of `M0`..`M8`",
            ));
        }

        // RL-08: kind rules (default action and named actions).
        let kind = Kind::parse(&entry.kind_raw).ok_or_else(|| {
            LintError::new(
                &entry.name,
                "RL-08",
                "kind",
                &entry.kind_raw,
                "kind must be one of `read`, `run`, `mutate`, or `admin`",
            )
        })?;
        match kind {
            Kind::Read => {
                if !entry.idempotent {
                    return Err(LintError::new(
                        &entry.name,
                        "RL-08",
                        "kind",
                        "idempotent = false",
                        "read verbs must set `idempotent = true`",
                    ));
                }
            }
            Kind::Run | Kind::Mutate => {
                if entry.idempotent {
                    return Err(LintError::new(
                        &entry.name,
                        "RL-08",
                        "kind",
                        "idempotent = true",
                        "run and mutate verbs must set `idempotent = false` and carry idempotency_key",
                    ));
                }
            }
            Kind::Admin => {
                if entry.capabilities.is_empty() {
                    return Err(LintError::new(
                        &entry.name,
                        "RL-08",
                        "kind",
                        "capabilities = []",
                        "admin verbs must declare at least one capability scope",
                    ));
                }
            }
        }
        if entry.destructive_explicit.is_some() && kind != Kind::Admin {
            return Err(LintError::new(
                &entry.name,
                "RL-08",
                "kind",
                "destructive",
                "`destructive` may only be declared on an `admin` entry",
            ));
        }

        // RL-23 (entry level): paginated = true only on an entry whose default action is `read`.
        if entry.paginated && kind != Kind::Read {
            return Err(LintError::new(
                &entry.name,
                "RL-23",
                "action-paginated-not-read",
                format!("paginated = true on kind `{kind}`"),
                "entry-level `paginated = true` is allowed only when default kind is `read`",
            ));
        }

        // Validate named actions (RL-08, RL-14, RL-23).
        let mut seen_action_names: BTreeSet<&str> = BTreeSet::new();
        let mut has_run_action = kind == Kind::Run;
        for act in &entry.actions {
            if !seen_action_names.insert(act.name.as_str()) {
                return Err(LintError::new(
                    &entry.name,
                    "RL-14",
                    "duplicate-name",
                    &act.name,
                    "duplicate action name within verb entry",
                ));
            }
            let act_kind = Kind::parse(&act.kind_raw).ok_or_else(|| {
                LintError::new(
                    &entry.name,
                    "RL-08",
                    "kind",
                    &act.kind_raw,
                    format!("action `{}` has invalid kind", act.name),
                )
            })?;
            if act_kind == Kind::Run {
                has_run_action = true;
            }
            if act_kind == Kind::Admin && act.capabilities.is_empty() {
                return Err(LintError::new(
                    &entry.name,
                    "RL-08",
                    "kind",
                    format!("actions.{}.capabilities = []", act.name),
                    "admin action must declare at least one capability scope",
                ));
            }
            if act.destructive_explicit.is_some() && act_kind != Kind::Admin {
                return Err(LintError::new(
                    &entry.name,
                    "RL-08",
                    "kind",
                    format!("actions.{}.destructive", act.name),
                    "`destructive` may only be declared on an `admin` action",
                ));
            }
            if act.paginated && act_kind != Kind::Read {
                return Err(LintError::new(
                    &entry.name,
                    "RL-23",
                    "action-paginated-not-read",
                    format!("actions.{}.paginated = true (kind `{act_kind}`)", act.name),
                    "action `paginated = true` is allowed only on a `read` action",
                ));
            }
        }

        // RL-09 & RL-18: `wait` rules.
        match (has_run_action, &entry.wait) {
            (true, None) => {
                return Err(LintError::new(
                    &entry.name,
                    "RL-09",
                    "run-without-wait",
                    "<missing wait>",
                    "every `run` verb (or verb with a `run` action) must declare a `wait` table",
                ));
            }
            (false, Some(_)) => {
                return Err(LintError::new(
                    &entry.name,
                    "RL-18",
                    "wait-on-non-run",
                    "wait",
                    "`wait` may only appear on a `run` verb or a verb with a `run` action",
                ));
            }
            (true, Some(w)) => {
                if w.cli != "to-completion" {
                    return Err(LintError::new(
                        &entry.name,
                        "RL-09",
                        "mcp-wait-out-of-range",
                        &w.cli,
                        "`wait.cli` must equal `\"to-completion\"`",
                    ));
                }
                if w.mcp_ms < 0 || w.mcp_max_ms < w.mcp_ms || w.mcp_max_ms > MAX_MCP_WAIT_MS {
                    return Err(LintError::new(
                        &entry.name,
                        "RL-09",
                        "mcp-wait-out-of-range",
                        format!("mcp_ms={}, mcp_max_ms={}", w.mcp_ms, w.mcp_max_ms),
                        format!(
                            "wait bounds must satisfy 0 <= mcp_ms <= mcp_max_ms <= {MAX_MCP_WAIT_MS}"
                        ),
                    ));
                }
            }
            (false, None) => {}
        }

        // RL-10: capabilities present; empty list requires `public = true`.
        if entry.capabilities.is_empty() && !entry.public {
            return Err(LintError::new(
                &entry.name,
                "RL-10",
                "empty-capabilities-not-public",
                "capabilities = [], public = false",
                "an empty `capabilities` list requires `public = true`",
            ));
        }

        // RL-15: scopes check (error when scopes catalogue is present; warning until M2).
        check_scopes(
            &entry.name,
            "capabilities",
            &entry.capabilities,
            ctx,
            &mut warnings,
        )?;
        for act in &entry.actions {
            check_scopes(
                &entry.name,
                &format!("actions.{}.capabilities", act.name),
                &act.capabilities,
                ctx,
                &mut warnings,
            )?;
        }
        for (opt, scopes) in &entry.option_capabilities {
            check_scopes(
                &entry.name,
                &format!("option_capabilities.{opt}"),
                scopes,
                ctx,
                &mut warnings,
            )?;
        }

        // RL-17: non-empty cost_model.
        if entry.cost_model.trim().is_empty() {
            return Err(LintError::new(
                &entry.name,
                "RL-17",
                "empty-cost-model",
                &entry.cost_model,
                "`cost_model` must not be empty",
            ));
        }

        // RL-19: owner is a subsystem crate, never a surface crate.
        if entry.owner.trim().is_empty()
            || matches!(
                entry.owner.as_str(),
                "frob-cli" | "frob-mcp" | "surface/frob-mcp"
            )
        {
            return Err(LintError::new(
                &entry.name,
                "RL-19",
                "owner-is-surface",
                &entry.owner,
                "`owner` must name a subsystem crate, never `frob-cli` or `surface/frob-mcp`",
            ));
        }

        // RL-20: purpose <= 200 characters (and non-empty).
        let purpose_len = entry.purpose.chars().count();
        if purpose_len == 0 || purpose_len > MAX_PURPOSE_CHARS {
            return Err(LintError::new(
                &entry.name,
                "RL-20",
                "purpose-too-long",
                format!("{purpose_len} chars"),
                format!("`purpose` must be between 1 and {MAX_PURPOSE_CHARS} characters"),
            ));
        }

        // RL-04: non-empty task_clusters and <= 7 non-base verbs per task cluster.
        if entry.task_clusters.is_empty() {
            return Err(LintError::new(
                &entry.name,
                "RL-04",
                "empty-task-clusters",
                "[]",
                "every verb must list at least one task cluster",
            ));
        }
        let base_verb = is_base_verb(&entry.name);
        for tc in &entry.task_clusters {
            if tc.trim().is_empty() {
                return Err(LintError::new(
                    &entry.name,
                    "RL-04",
                    "empty-task-clusters",
                    tc,
                    "task cluster name must not be empty",
                ));
            }
            if !base_verb && tc != "base" {
                let count = task_cluster_counts.entry(tc.as_str()).or_insert(0);
                *count += 1;
                if *count > MAX_TOOLS_PER_TASK_CLUSTER {
                    return Err(LintError::new(
                        &entry.name,
                        "RL-04",
                        "empty-task-clusters",
                        format!("{tc} ({count} tools)"),
                        format!(
                            "task cluster `{tc}` exceeds cap of {MAX_TOOLS_PER_TASK_CLUSTER} non-base tools"
                        ),
                    ));
                }
            }
        }

        // RL-05: schema identifiers well-formed, major matches schema_version, files exist and parse.
        let in_id = OwnedSchemaId::parse(&entry.input_schema).map_err(|e| {
            LintError::new(
                &entry.name,
                "RL-05",
                "schema-id-form",
                &entry.input_schema,
                e.to_string(),
            )
        })?;
        let out_id = OwnedSchemaId::parse(&entry.output_schema).map_err(|e| {
            LintError::new(
                &entry.name,
                "RL-05",
                "schema-id-form",
                &entry.output_schema,
                e.to_string(),
            )
        })?;
        let expected_ver = u32::try_from(entry.schema_version).map_err(|_| {
            LintError::new(
                &entry.name,
                "RL-05",
                "schema-id-form",
                entry.schema_version.to_string(),
                "`schema_version` must be a positive u32",
            )
        })?;
        if expected_ver == 0 || in_id.major() != expected_ver || out_id.major() != expected_ver {
            return Err(LintError::new(
                &entry.name,
                "RL-05",
                "schema-id-form",
                format!(
                    "schema_version={}, in={}, out={}",
                    entry.schema_version, entry.input_schema, entry.output_schema
                ),
                "schema major versions must match `schema_version`",
            ));
        }

        let in_bytes = ctx.schemas.get(&in_id.file_name()).ok_or_else(|| {
            LintError::new(
                &entry.name,
                "RL-05",
                "schema-id-form",
                in_id.relative_path(),
                format!("input schema file `{}` is missing", in_id.relative_path()),
            )
        })?;
        let out_bytes = ctx.schemas.get(&out_id.file_name()).ok_or_else(|| {
            LintError::new(
                &entry.name,
                "RL-05",
                "schema-id-form",
                out_id.relative_path(),
                format!("output schema file `{}` is missing", out_id.relative_path()),
            )
        })?;

        let in_admitted = admit(in_bytes, &Limits::default()).map_err(|e| {
            LintError::new(
                &entry.name,
                "RL-05",
                "schema-id-form",
                in_id.relative_path(),
                format!("input schema is not valid JSON: {e}"),
            )
        })?;
        let out_admitted = admit(out_bytes, &Limits::default()).map_err(|e| {
            LintError::new(
                &entry.name,
                "RL-05",
                "schema-id-form",
                out_id.relative_path(),
                format!("output schema is not valid JSON: {e}"),
            )
        })?;

        let in_info = extract_input_schema_info(in_admitted.value());

        // RL-06: refusal codes valid in frob-core, FROB-REF-990 forbidden, full list non-empty.
        for code in &entry.refusals {
            if code == "FROB-REF-990" {
                return Err(LintError::new(
                    &entry.name,
                    "RL-06",
                    "unknown-refusal",
                    code,
                    "internal-fault-not-listable: FROB-REF-990 is outside every verb refusal list",
                ));
            }
            ctx.refusals
                .lookup(code, &format!("verb `{}`", entry.name))
                .map_err(|e| {
                    LintError::new(&entry.name, "RL-06", "unknown-refusal", code, e.to_string())
                })?;
        }
        let appended_count = if ctx.append_kind_refusals {
            surface_wide_refusal_count(kind, entry)
        } else {
            0
        };
        if entry.refusals.is_empty() && appended_count == 0 {
            return Err(LintError::new(
                &entry.name,
                "RL-06",
                "unknown-refusal",
                "[]",
                "full refusal list (chapter list plus surface-wide codes) must be non-empty",
            ));
        }

        // RL-21: key_optional allowed only on `session.focus` or a `mutate` entry whose output
        // schema declares `x-frob-identity: "request-digest"`.
        if entry.key_optional {
            let output_req_digest = out_admitted
                .value()
                .get("x-frob-identity")
                .and_then(|v| v.as_str())
                == Some("request-digest");
            let allowed =
                entry.name == "session.focus" || (kind == Kind::Mutate && output_req_digest);
            if !allowed {
                return Err(LintError::new(
                    &entry.name,
                    "RL-21",
                    "key-optional-not-content-addressed",
                    "key_optional = true",
                    "`key_optional = true` is allowed only on `session.focus` or a `mutate` verb whose output schema declares `x-frob-identity = \"request-digest\"`",
                ));
            }
        }

        // RL-22: every key of option_capabilities must exist in the entry's input schema.
        for opt_key in entry.option_capabilities.keys() {
            if !in_info.properties.contains_key(opt_key) {
                return Err(LintError::new(
                    &entry.name,
                    "RL-22",
                    "unknown-option",
                    opt_key,
                    format!(
                        "`option_capabilities` key `{opt_key}` is not a property of `{}`",
                        entry.input_schema
                    ),
                ));
            }
        }

        // RL-24: every name in conformance_only must be an optional, non-positional input property.
        for prop_name in &entry.conformance_only {
            let is_prop = in_info.properties.contains_key(prop_name);
            let is_req = in_info.required.contains(prop_name);
            let is_pos = in_info.positionals.contains(prop_name);
            if !is_prop || is_req || is_pos {
                let reason = if !is_prop {
                    "is not a property of the input schema"
                } else if is_req {
                    "is listed in `required`"
                } else {
                    "is marked positional"
                };
                return Err(LintError::new(
                    &entry.name,
                    "RL-24",
                    "conformance-only-not-an-input",
                    prop_name,
                    format!("`conformance_only` property `{prop_name}` {reason}"),
                ));
            }
        }

        let strips = if ctx.conformance {
            BTreeSet::new()
        } else {
            entry.conformance_only.iter().cloned().collect()
        };
        let served_in =
            canonical_json_strip_properties(in_admitted.value(), &strips).map_err(|e| {
                LintError::new(
                    &entry.name,
                    "RL-05",
                    "schema-id-form",
                    &entry.input_schema,
                    format!("input schema canonicalization failed: {e}"),
                )
            })?;

        if let Err(err) = entry.check_shadowing(&served_in) {
            return Err(LintError::new(
                &entry.name,
                "NJL-01",
                "common-property-shadowed",
                err.property.clone(),
                err.to_string(),
            ));
        }

        // RL-07 and RL-16 use the same served schema and argument conversion as the CLI.
        if entry.examples.is_empty() {
            return Err(LintError::new(
                &entry.name,
                "RL-07",
                "empty-examples",
                "[]",
                "every verb must provide at least one runnable example",
            ));
        }
        let example_error = |reason| {
            LintError::new(
                &entry.name,
                "RL-07",
                "empty-examples",
                &entry.input_schema,
                reason,
            )
        };
        let merged = entry
            .input_schema_for_examples(&served_in)
            .map_err(|e| example_error(e.to_string()))?;
        let raw = Tree::parse(served_in.as_bytes()).map_err(|e| example_error(e.to_string()))?;
        let schema = Schema::compile_json(&merged).map_err(|e| example_error(e.to_string()))?;
        for example in &entry.examples {
            validate_example(entry, example, &in_info, &raw, &schema)?;
        }
        let short_doc = entry.short_doc_checked_with(&served_in, &ctx.refusals).map_err(|doc_len| {
            LintError::new(
                &entry.name,
                RESERVED_RL11.0,
                RESERVED_RL11.1,
                format!("{doc_len} chars"),
                format!("derived `short_doc` is {doc_len} characters, exceeding {MAX_DESCRIPTION_CHARS}"),
            )
        })?;
        longest_description = longest_description.max(short_doc.chars().count());

        let out_canon = write_json(out_admitted.value()).map_err(|e| {
            LintError::new(
                &entry.name,
                "RL-05",
                "schema-id-form",
                &entry.output_schema,
                format!("output schema canonicalization failed: {e}"),
            )
        })?;
        embedded_schemas.insert(entry.output_schema.clone(), out_canon.as_str().to_owned());

        // Bare ID lookup remains available when all input uses agree. Output uses keep
        // the original schema; input consumers always select the exact verb's schema.
        input_by_id
            .entry(entry.input_schema.clone())
            .and_modify(|candidate| {
                if candidate.as_ref() != Some(&served_in) {
                    *candidate = None;
                }
            })
            .or_insert_with(|| Some(served_in.clone()));
        embedded_inputs.insert(entry.name.clone(), served_in);
    }

    for (id, candidate) in input_by_id {
        if let Some(schema) = candidate {
            embedded_schemas.entry(id).or_insert(schema);
        }
    }

    for retired in &reg.retired_names {
        for target in &retired.use_instead {
            let cluster = target
                .split_once('.')
                .map_or("operator", |(cluster, _)| cluster);
            if validate_verb_name(target, cluster).is_err()
                || retired_tools.contains_key(&tool_name(target))
                || (clusters.contains(cluster) && !seen_names.contains(target.as_str()))
            {
                return Err(LintError::new(
                    &retired.name,
                    "RL-03",
                    "retired-target",
                    target,
                    format!("expected a registered replacement in cluster `{cluster}`"),
                ));
            }
            if !seen_names.contains(target.as_str()) {
                warnings.push(format!(
                    "RL-03 not-due: `{}` uses `{target}`; cluster `{cluster}` is absent",
                    retired.name
                ));
            }
        }
    }

    let largest_task_cluster = task_cluster_counts.values().copied().max().unwrap_or(0);
    Ok(LintReport {
        warnings,
        embedded_schemas,
        embedded_inputs,
        largest_task_cluster,
        cluster_count: clusters.len(),
        longest_description,
    })
}

fn check_scopes(
    entry_name: &str,
    field: &str,
    scopes: &[String],
    ctx: &LintContext,
    warnings: &mut Vec<String>,
) -> Result<(), LintError> {
    for scope in scopes {
        if scope.trim().is_empty() {
            return Err(LintError::new(
                entry_name,
                "RL-15",
                "unknown-scope",
                scope,
                format!("{field} contains an empty capability scope"),
            ));
        }
        if let Some(known) = &ctx.scopes {
            if !known.contains(scope) {
                return Err(LintError::new(
                    entry_name,
                    "RL-15",
                    "unknown-scope",
                    scope,
                    format!("scope `{scope}` in `{field}` is absent from registries/scopes.toml"),
                ));
            }
        } else {
            warnings.push(format!(
                "RL-15 warning: entry `{entry_name}` `{field}` proposes scope `{scope}` (registries/scopes.toml not yet present)"
            ));
        }
    }
    Ok(())
}

/// Count of surface-wide refusal codes NJL-01 appends for `kind` (used by RL-06 non-empty check).
fn surface_wide_refusal_count(kind: Kind, entry: &OwnedVerbEntry) -> usize {
    match kind {
        Kind::Read | Kind::Admin => 4, // 720, 723, 724, 802
        Kind::Run => 11,               // 701..704, 720, 723, 724, 761, 802, 803, 805
        Kind::Mutate => {
            if entry.key_optional {
                7 // 720, 723, 724, 725, 802, 803, 804 (805 omitted)
            } else {
                8 // + 805
            }
        }
    }
}

#[derive(Debug, Default)]
struct PropMeta {
    identity_prefix: Option<String>,
}

#[derive(Debug, Default)]
struct InputSchemaInfo {
    properties: BTreeMap<String, PropMeta>,
    required: BTreeSet<String>,
    positionals: Vec<String>,
}

fn extract_input_schema_info(root: &Json<'_>) -> InputSchemaInfo {
    let mut info = InputSchemaInfo::default();
    if let Some(req_arr) = root.get("required").and_then(|v| v.as_array()) {
        for item in req_arr {
            if let Some(s) = item.as_str() {
                info.required.insert(s.to_owned());
            }
        }
    }
    let declared_positionals = root
        .get("x-frob-cli")
        .and_then(|cli| cli.get("positional"))
        .or_else(|| root.get("x-frob-positionals"));
    if let Some(pos_arr) = declared_positionals.and_then(|value| value.as_array()) {
        for item in pos_arr {
            if let Some(s) = item.as_str() {
                info.positionals.push(s.to_owned());
            }
        }
    }
    if let Some(props) = root.get("properties").and_then(|v| v.as_object()) {
        for member in props {
            let mut meta = PropMeta::default();
            if let Some(pref) = member
                .value
                .get("x-frob-identity-prefix")
                .and_then(|v| v.as_str())
            {
                meta.identity_prefix = Some(pref.trim_end_matches(':').to_owned());
            } else if let Some(pat) = member.value.get("pattern").and_then(|v| v.as_str())
                && let Some(rest) = pat.strip_prefix('^')
                && let Some((prefix, _)) = rest.split_once(':')
                && !prefix.is_empty()
                && prefix.bytes().all(|b| b.is_ascii_lowercase())
            {
                meta.identity_prefix = Some(prefix.to_owned());
            }
            if member
                .value
                .get("x-frob-positional")
                .and_then(|v| v.as_bool())
                == Some(true)
                && !info.positionals.contains(&member.name)
            {
                info.positionals.push(member.name.clone());
            }
            info.properties.insert(member.name.clone(), meta);
        }
    }
    info
}

fn validate_example(
    entry: &OwnedVerbEntry,
    example: &str,
    info: &InputSchemaInfo,
    raw: &Tree,
    schema: &Schema,
) -> Result<(), LintError> {
    let tokens = shell_split(example).map_err(|reason| {
        LintError::new(&entry.name, "RL-07", "empty-examples", example, reason)
    })?;
    if tokens.is_empty() || tokens[0] != "frob" {
        return Err(LintError::new(
            &entry.name,
            "RL-16",
            "example-not-this-verb",
            example,
            "example must begin with `frob <cli path>`",
        ));
    }

    let expected_path = entry.cli_path();
    if tokens.len() < 1 + expected_path.len() {
        return Err(LintError::new(
            &entry.name,
            "RL-16",
            "example-not-this-verb",
            example,
            format!(
                "example is shorter than verb CLI path `{}`",
                expected_path.join(" ")
            ),
        ));
    }
    for (idx, seg) in expected_path.iter().enumerate() {
        if tokens[1 + idx] != *seg {
            return Err(LintError::new(
                &entry.name,
                "RL-16",
                "example-not-this-verb",
                example,
                format!(
                    "example command path `{}` does not match verb CLI path `{}`",
                    tokens[1..=expected_path.len()].join(" "),
                    expected_path.join(" ")
                ),
            ));
        }
    }

    let error =
        |value, reason| LintError::new(&entry.name, "RL-07", "empty-examples", value, reason);
    let mut remaining = tokens.into_iter().skip(1 + expected_path.len());
    let mut args = Vec::new();
    while let Some(token) = remaining.next() {
        if token == "--" {
            args.push(token);
            args.extend(remaining);
            break;
        }
        let flag = token.strip_prefix("--").map(|f| {
            f.split_once('=')
                .map_or((f, None), |(name, val)| (name, Some(val)))
        });
        if let Some((name, inline)) = flag
            && let Some(&(_, takes_value)) = PROCESS_FLAGS.iter().find(|(f, _)| *f == name)
        {
            if takes_value && inline.is_none() {
                remaining
                    .next()
                    .filter(|val| !val.starts_with("--"))
                    .ok_or_else(|| {
                        error(
                            token.clone(),
                            format!("process option --{name} requires a value"),
                        )
                    })?;
            } else if !takes_value && inline.is_some() {
                return Err(error(
                    token.clone(),
                    format!("process option --{name} does not take a value"),
                ));
            }
        } else {
            args.push(token);
        }
    }

    let common = entry.common_properties();
    let mut instance = parse_arguments(
        raw,
        &common,
        info.positionals.iter().map(String::as_str),
        &args,
    )
    .map_err(|(name, reason)| error(name, reason.to_owned()))?;
    if let Tree::Obj(fields) = &mut instance {
        for (name, value) in fields {
            if let Tree::Str(raw) = value {
                let prefix = info
                    .properties
                    .get(name)
                    .and_then(|meta| meta.identity_prefix.as_deref());
                *raw = check_abbreviated_identity(&entry.name, raw, prefix)?;
            }
        }
    }
    schema.validate(&instance).map_err(|violations| {
        error(
            example.to_owned(),
            violations
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join("; "),
        )
    })
}

/// Validate identity-shaped tokens `<prefix>:<hex>` in examples (RL-07).
///
/// - Unknown prefix (e.g. `q:19e2`) fails with RL-07.
/// - Registered prefix with `< 64` hex digits is expanded to `<prefix>:<64 zeros>` per N32a-02.
/// - Registered prefix with 64 hex digits passes unchanged.
/// - If `expected_prefix` is declared by the schema, the token's prefix must match it.
fn check_abbreviated_identity(
    entry_name: &str,
    token: &str,
    expected_prefix: Option<&str>,
) -> Result<String, LintError> {
    if let Some((prefix, hex)) = token.split_once(':')
        && !prefix.is_empty()
        && prefix.len() <= 4
        && prefix.bytes().all(|b| b.is_ascii_lowercase())
        && !hex.is_empty()
        && hex.len() <= 64
        && hex.bytes().all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f'))
    {
        if !REGISTERED_ID_PREFIXES.contains(&prefix) {
            return Err(LintError::new(
                entry_name,
                "RL-07",
                "empty-examples",
                token,
                format!("unknown identity prefix `{prefix}:` in example"),
            ));
        }
        if let Some(exp) = expected_prefix
            && prefix != exp
        {
            return Err(LintError::new(
                entry_name,
                "RL-07",
                "empty-examples",
                token,
                format!("identity prefix `{prefix}:` does not match schema prefix `{exp}:`"),
            ));
        }
        if hex.len() < 64 {
            return Ok(format!("{prefix}:{}", "0".repeat(64)));
        }
    } else if let Some(exp) = expected_prefix {
        return Err(LintError::new(
            entry_name,
            "RL-07",
            "empty-examples",
            token,
            format!("expected identity with prefix `{exp}:`"),
        ));
    }
    Ok(token.to_owned())
}

fn shell_split(input: &str) -> Result<Vec<String>, &'static str> {
    let mut tokens = Vec::new();
    let mut cur = String::new();
    let mut in_token = false;
    let mut quote: Option<char> = None;
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        match quote {
            Some('\'') => {
                if ch == '\'' {
                    quote = None;
                } else {
                    cur.push(ch);
                }
            }
            Some('"') => {
                if ch == '"' {
                    quote = None;
                } else if ch == '\\' {
                    let next = chars.next().ok_or("unterminated escape in example")?;
                    cur.push(next);
                } else {
                    cur.push(ch);
                }
            }
            Some(_) => unreachable!(),
            None => match ch {
                '\'' | '"' => {
                    quote = Some(ch);
                    in_token = true;
                }
                c if c.is_ascii_whitespace() => {
                    if in_token {
                        tokens.push(std::mem::take(&mut cur));
                        in_token = false;
                    }
                }
                '\\' => {
                    let next = chars.next().ok_or("unterminated escape in example")?;
                    cur.push(next);
                    in_token = true;
                }
                other => {
                    cur.push(other);
                    in_token = true;
                }
            },
        }
    }
    if quote.is_some() {
        return Err("unterminated quote in example command line");
    }
    if in_token {
        tokens.push(cur);
    }
    Ok(tokens)
}

/// Canonicalize a JSON schema object while removing every named property from top-level
/// `properties` (used when building without `feature = "conformance"`).
fn canonical_json_strip_properties(
    root: &Json<'_>,
    strip: &BTreeSet<String>,
) -> Result<String, frob_core::canon::CanonError> {
    let mut writer = CanonWriter::new();
    write_stripped_root(root, strip, writer.value()?)?;
    Ok(writer.finish()?.as_str().to_owned())
}

fn write_stripped_root(
    node: &Json<'_>,
    strip: &BTreeSet<String>,
    w: CanonValue<'_>,
) -> Result<(), frob_core::canon::CanonError> {
    let Some(members) = node.as_object() else {
        return w.term(node);
    };
    w.object(|o| {
        for member in members {
            if member.name == "properties"
                && let Some(prop_members) = member.value.as_object()
            {
                o.object("properties", |po| {
                    for prop in prop_members {
                        if !strip.contains(&prop.name) {
                            po.member_term(&prop.name, &prop.value)?;
                        }
                    }
                    Ok(())
                })?;
            } else {
                o.member_term(&member.name, &member.value)?;
            }
        }
        Ok(())
    })
}
