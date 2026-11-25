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

use super::entry::{Kind, Milestone, OwnedVerbEntry, ParseError, ParsedRegistry, Tier};
use super::names::{base_tool_owner, is_base_verb, tool_name, validate_verb_name};
use super::schema_id::{OwnedSchemaId, schema_file_name};

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
    /// Map from schema identifier (`frob.<...>/<major>`) to canonical JSON schema text to embed
    /// in `SCHEMAS` (with `conformance_only` properties stripped when `!ctx.conformance`).
    pub embedded_schemas: BTreeMap<String, String>,
    /// Largest non-base task-cluster tool count observed across entries.
    pub largest_task_cluster: usize,
    /// Distinct registry cluster count.
    pub cluster_count: usize,
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

    let mut seen_names: BTreeSet<&str> = BTreeSet::new();
    let mut seen_tools: BTreeMap<String, &str> = BTreeMap::new();
    let mut seen_schema_pairs: BTreeMap<(&str, &str), &str> = BTreeMap::new();
    let mut retired_map: BTreeMap<&str, (&[String], &str)> = BTreeMap::new();
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
        retired_map.insert(ret.name.as_str(), (&ret.use_instead, ret.reason.as_str()));
    }

    // Track which entries strip which `conformance_only` properties from a given input schema.
    let mut input_schema_strips: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();

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
        if let Some((use_instead, reason)) = retired_map.get(entry.name.as_str()) {
            return Err(LintError::new(
                &entry.name,
                "RL-03",
                "retired-name",
                entry.name.clone(),
                format!(
                    "retired verb name (use {:?}; reason: {reason})",
                    use_instead
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
                "retired-name",
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
            if !ctx.conformance {
                input_schema_strips
                    .entry(entry.input_schema.clone())
                    .or_default()
                    .insert(prop_name.clone());
            }
        }

        // RL-07 & RL-16: non-empty examples, matching CLI path, valid flags/args/identities.
        if entry.examples.is_empty() {
            return Err(LintError::new(
                &entry.name,
                "RL-07",
                "empty-examples",
                "[]",
                "every verb must provide at least one runnable example",
            ));
        }
        for example in &entry.examples {
            validate_example(entry, kind, example, &in_info)?;
        }

        // Record canonical embedded schemas (with conformance_only stripping applied below).
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

        // Ensure input schema is registered for canonical embedding after collecting strip sets.
        embedded_schemas
            .entry(entry.input_schema.clone())
            .or_default();
    }

    // Populate canonical embedded input schemas (stripping `conformance_only` when !conformance).
    for (schema_id, slot) in &mut embedded_schemas {
        if !slot.is_empty() {
            continue;
        }
        let fname = schema_file_name(schema_id);
        let raw = ctx.schemas.get(&fname).ok_or_else(|| {
            LintError::new(
                "<schemas>",
                "RL-05",
                "schema-id-form",
                &fname,
                "schema disappeared during canonicalization",
            )
        })?;
        let adm = admit(raw, &Limits::default()).map_err(|e| {
            LintError::new(
                "<schemas>",
                "RL-05",
                "schema-id-form",
                &fname,
                format!("schema JSON admission failed: {e}"),
            )
        })?;
        let strips = input_schema_strips.get(schema_id);
        let rendered = if let Some(to_strip) = strips
            && !to_strip.is_empty()
        {
            canonical_json_strip_properties(adm.value(), to_strip).map_err(|e| {
                LintError::new(
                    "<schemas>",
                    "RL-24",
                    "conformance-only-not-an-input",
                    schema_id,
                    format!("failed stripping conformance_only properties: {e}"),
                )
            })?
        } else {
            write_json(adm.value())
                .map_err(|e| {
                    LintError::new(
                        "<schemas>",
                        "RL-05",
                        "schema-id-form",
                        schema_id,
                        format!("schema canonicalization failed: {e}"),
                    )
                })?
                .as_str()
                .to_owned()
        };
        *slot = rendered;
    }

    // Post-strip NJL-01 lints against the served embedded input schemas:
    // `common-property-shadowed` and `RL-11` (`description-too-long`).
    for entry in &reg.verbs {
        let served_in = embedded_schemas
            .get(&entry.input_schema)
            .map(String::as_str)
            .unwrap_or("{}");
        if let Err(err) = entry.check_shadowing(served_in) {
            return Err(LintError::new(
                &entry.name,
                "NJL-01",
                "common-property-shadowed",
                err.property.clone(),
                err.to_string(),
            ));
        }
        if let Err(doc_len) = entry.short_doc_checked_with(served_in, &ctx.refusals) {
            return Err(LintError::new(
                &entry.name,
                RESERVED_RL11.0,
                RESERVED_RL11.1,
                format!("{doc_len} chars"),
                format!(
                    "derived `short_doc` is {doc_len} characters, exceeding {MAX_DESCRIPTION_CHARS}"
                ),
            ));
        }
    }

    let largest_task_cluster = task_cluster_counts.values().copied().max().unwrap_or(0);
    Ok(LintReport {
        warnings,
        embedded_schemas,
        largest_task_cluster,
        cluster_count: clusters.len(),
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
    ty: Option<String>,
    enum_values: Option<Vec<String>>,
    identity_prefix: Option<String>,
    is_positional: bool,
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
            if let Some(t) = member.value.get("type").and_then(|v| v.as_str()) {
                meta.ty = Some(t.to_owned());
            }
            if let Some(en) = member.value.get("enum").and_then(|v| v.as_array()) {
                let vals: Vec<String> = en
                    .iter()
                    .filter_map(|v| v.as_str().map(str::to_owned))
                    .collect();
                meta.enum_values = Some(vals);
            }
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
            {
                meta.is_positional = true;
                if !info.positionals.contains(&member.name) {
                    info.positionals.push(member.name.clone());
                }
            }
            info.properties.insert(member.name.clone(), meta);
        }
    }
    info
}

fn validate_example(
    entry: &OwnedVerbEntry,
    kind: Kind,
    example: &str,
    schema: &InputSchemaInfo,
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

    // Build set of common flags allowed by this entry's kind, fields, and actions.
    let mut common_flags: BTreeMap<&'static str, bool> = BTreeMap::new(); // flag -> takes_value
    for &(f, takes_val) in PROCESS_FLAGS {
        common_flags.insert(f, takes_val);
    }
    let has_run = kind == Kind::Run
        || entry
            .actions
            .iter()
            .any(|a| Kind::parse(&a.kind_raw) == Some(Kind::Run));
    let has_mutate = kind == Kind::Mutate
        || entry
            .actions
            .iter()
            .any(|a| Kind::parse(&a.kind_raw) == Some(Kind::Mutate));
    let has_paginated = entry.paginated || entry.actions.iter().any(|a| a.paginated);
    if has_run {
        common_flags.insert("key", true);
        common_flags.insert("budget", true);
        common_flags.insert("estimate", false);
        common_flags.insert("sync-wait-ms", true);
        common_flags.insert("on-cancel", true);
        common_flags.insert("envelope-scope", true);
    }
    if has_mutate {
        common_flags.insert("key", true);
        common_flags.insert("expected-parent", true);
    }
    if entry.estimable {
        common_flags.insert("estimate", false);
    }
    if has_paginated {
        common_flags.insert("cursor", true);
        common_flags.insert("limit", true);
        common_flags.insert("follow", false);
    }
    if !entry.actions.is_empty() {
        common_flags.insert("action", true);
    }

    // Map kebab-case CLI flags to schema property names.
    let mut flag_to_prop: BTreeMap<String, String> = BTreeMap::new();
    for prop in schema.properties.keys() {
        flag_to_prop.insert(prop.replace('_', "-"), prop.clone());
        flag_to_prop.insert(prop.clone(), prop.clone());
    }

    let mut provided_props: BTreeSet<String> = BTreeSet::new();
    let mut positionals: Vec<String> = Vec::new();

    let mut idx = 1 + expected_path.len();
    while idx < tokens.len() {
        let tok = &tokens[idx];
        if let Some(flag_raw) = tok.strip_prefix("--") {
            let (flag_name, inline_val) = match flag_raw.split_once('=') {
                Some((k, v)) => (k, Some(v)),
                None => (flag_raw, None),
            };
            if let Some(&takes_val) = common_flags.get(flag_name) {
                let val_opt = if takes_val {
                    if let Some(v) = inline_val {
                        Some(v.to_owned())
                    } else {
                        idx += 1;
                        let v = tokens.get(idx).ok_or_else(|| {
                            LintError::new(
                                &entry.name,
                                "RL-07",
                                "empty-examples",
                                tok,
                                format!("flag `{tok}` requires a value"),
                            )
                        })?;
                        Some(v.clone())
                    }
                } else {
                    if inline_val.is_some() {
                        return Err(LintError::new(
                            &entry.name,
                            "RL-07",
                            "empty-examples",
                            tok,
                            format!("boolean flag `--{flag_name}` does not take a value"),
                        ));
                    }
                    None
                };
                if flag_name == "action"
                    && let Some(act_val) = &val_opt
                    && !entry.actions.iter().any(|a| a.name == *act_val)
                {
                    return Err(LintError::new(
                        &entry.name,
                        "RL-07",
                        "empty-examples",
                        act_val,
                        format!("unknown action `{act_val}` in example"),
                    ));
                }
                if let Some(val) = val_opt {
                    check_abbreviated_identity(&entry.name, &val, None)?;
                }
            } else if let Some(prop_name) = flag_to_prop.get(flag_name) {
                let meta = &schema.properties[prop_name];
                let is_bool = meta.ty.as_deref() == Some("boolean");
                let val_str = if is_bool && inline_val.is_none() {
                    "true".to_owned()
                } else if let Some(v) = inline_val {
                    v.to_owned()
                } else {
                    idx += 1;
                    tokens
                        .get(idx)
                        .ok_or_else(|| {
                            LintError::new(
                                &entry.name,
                                "RL-07",
                                "empty-examples",
                                tok,
                                format!("flag `{tok}` requires a value"),
                            )
                        })?
                        .clone()
                };
                validate_prop_value(&entry.name, prop_name, &val_str, meta)?;
                provided_props.insert(prop_name.clone());
            } else {
                return Err(LintError::new(
                    &entry.name,
                    "RL-07",
                    "empty-examples",
                    tok,
                    format!("unknown flag `{tok}` not in `{}`", entry.input_schema),
                ));
            }
        } else if tok.starts_with('-') && tok.len() > 1 {
            return Err(LintError::new(
                &entry.name,
                "RL-07",
                "empty-examples",
                tok,
                format!("short/unknown flag `{tok}` is not part of the generated CLI grammar"),
            ));
        } else {
            positionals.push(tok.clone());
        }
        idx += 1;
    }

    // Bind positional arguments against declared positional metadata (or ordered required
    // properties when no explicit positional metadata is declared).
    let positional_targets: Vec<String> = if !schema.positionals.is_empty() {
        schema.positionals.clone()
    } else {
        schema
            .required
            .iter()
            .filter(|p| !provided_props.contains(*p))
            .cloned()
            .collect()
    };

    if positionals.len() > positional_targets.len() {
        return Err(LintError::new(
            &entry.name,
            "RL-07",
            "empty-examples",
            positionals.join(" "),
            format!(
                "example supplies {} positional argument(s), schema accepts at most {}",
                positionals.len(),
                positional_targets.len()
            ),
        ));
    }

    for (pos_val, target_prop) in positionals.iter().zip(positional_targets.iter()) {
        if let Some(meta) = schema.properties.get(target_prop) {
            validate_prop_value(&entry.name, target_prop, pos_val, meta)?;
        } else {
            check_abbreviated_identity(&entry.name, pos_val, None)?;
        }
        provided_props.insert(target_prop.clone());
    }

    for req in &schema.required {
        if !provided_props.contains(req) {
            return Err(LintError::new(
                &entry.name,
                "RL-07",
                "empty-examples",
                example,
                format!("example omits required input property `{req}`"),
            ));
        }
    }

    Ok(())
}

fn validate_prop_value(
    entry_name: &str,
    prop_name: &str,
    raw_val: &str,
    meta: &PropMeta,
) -> Result<(), LintError> {
    let normalized =
        check_abbreviated_identity(entry_name, raw_val, meta.identity_prefix.as_deref())?;
    if let Some(ty) = meta.ty.as_deref() {
        match ty {
            "integer" => {
                if normalized.parse::<i64>().is_err() {
                    return Err(LintError::new(
                        entry_name,
                        "RL-07",
                        "empty-examples",
                        raw_val,
                        format!("property `{prop_name}` expects an integer"),
                    ));
                }
            }
            "number" => {
                if normalized
                    .parse::<f64>()
                    .ok()
                    .filter(|v| v.is_finite())
                    .is_none()
                {
                    return Err(LintError::new(
                        entry_name,
                        "RL-07",
                        "empty-examples",
                        raw_val,
                        format!("property `{prop_name}` expects a finite number"),
                    ));
                }
            }
            "boolean" => {
                if !matches!(normalized.as_str(), "true" | "false") {
                    return Err(LintError::new(
                        entry_name,
                        "RL-07",
                        "empty-examples",
                        raw_val,
                        format!("property `{prop_name}` expects a boolean"),
                    ));
                }
            }
            _ => {}
        }
    }
    if let Some(allowed) = &meta.enum_values
        && !allowed.is_empty()
        && !allowed.contains(&normalized)
    {
        return Err(LintError::new(
            entry_name,
            "RL-07",
            "empty-examples",
            raw_val,
            format!("property `{prop_name}` value must be one of {allowed:?}"),
        ));
    }
    Ok(())
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
