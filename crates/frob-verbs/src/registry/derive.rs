//! Kind-based derivation of a verb entry: common input properties, appended refusals,
//! multi-action schema rules, MCP annotations/tags/short_doc, `common-property-shadowed`
//! lint, and the canonical `frob.verbs.entry/1` JSON Schema (bead NJL-01).
//!
//! Pure D0 functions over `registry::entry` types; imports only `std`, `frob_core`, and
//! `super::entry` (never `envelope`, `dispatch`, `handlers`, surface crates, or `frob-fsim`),
//! and never reads `first_milestone`.

#[cfg(all(
    any(
        feature = "gauntlet-mutations",
        feature = "nc-omit-805",
        feature = "nc-key-in-required",
        feature = "nc-append-990"
    ),
    not(debug_assertions)
))]
compile_error!("frob-verbs gauntlet mutation features are forbidden in release builds");

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use frob_core::canon::{CanonError, CanonWriter, Value as CanonValue};
use frob_core::json::{Json, Limits, admit};
use frob_core::refusal::RefusalCode;
use frob_core::registry::refusals::RefusalRegistry;

use super::entry::{Kind, OwnedParamKind, OwnedVerbEntry, ParamKind, Tier, VerbEntry};

/// Process-level CLI options accepted on every `frob` invocation (never an input property).
pub const PROCESS_OPTIONS: &[&str] = &[
    "json",
    "session",
    "registries-root",
    "session-token-file",
    "no-daemon",
];

/// Maximum character length of a verb's derived `short_doc` (`RL-11`).
pub const MAX_SHORT_DOC_CHARS: usize = 800;

const DEFAULT_SYNC_WAIT_MS: i64 = 5_000;
const DEFAULT_SYNC_WAIT_MAX_MS: i64 = 30_000;
const ENVELOPE_SCOPE_PATTERN: &str = "^(gate|campaign|pretrain):[A-Za-z0-9._-]+$";

/// Whether `idempotency_key` is required or optional at admission (`x-frob-key`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyRule {
    Required,
    Optional,
}

impl KeyRule {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Required => "required",
            Self::Optional => "optional",
        }
    }
}

/// Generator rule that introduced a common input property.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Source {
    Kind(Kind),
    Field(&'static str),
    Table(&'static str),
}

impl fmt::Display for Source {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Kind(k) => write!(f, "kind:{k}"),
            Self::Field(name) => write!(f, "field:{name}"),
            Self::Table(name) => write!(f, "table:{name}"),
        }
    }
}

/// Rule origin for an entry in [`ListedRefusal`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RefusalOrigin {
    Chapter,
    Kind(Kind),
    Field(&'static str),
    Table(&'static str),
}

impl fmt::Display for RefusalOrigin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Chapter => f.write_str("chapter"),
            Self::Kind(k) => write!(f, "kind:{k}"),
            Self::Field(name) => write!(f, "field:{name}"),
            Self::Table(name) => write!(f, "table:{name}"),
        }
    }
}

/// Provenance of one code in a verb's full refusal list.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RefusalSource {
    pub origin: RefusalOrigin,
    /// Named action that introduced the code, or `None` for the default action / whole entry.
    pub action: Option<String>,
}

impl fmt::Display for RefusalSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.action {
            Some(act) => write!(f, "{} (action:{act})", self.origin),
            None => write!(f, "{}", self.origin),
        }
    }
}

/// One refusal code in a verb's derived full refusal list, with its provenance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListedRefusal {
    pub code: u16,
    pub source: RefusalSource,
}

impl ListedRefusal {
    #[must_use]
    pub fn code_str(&self) -> String {
        format!("FROB-REF-{}", self.code)
    }

    #[must_use]
    pub fn typed(&self) -> Option<RefusalCode> {
        RefusalCode::parse(&self.code_str())
    }
}

/// One generator-added common input property, in fixed derivation order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommonProperty {
    pub name: &'static str,
    pub cli_flag: &'static str,
    /// Canonical JSON Schema fragment for this property.
    pub schema_json: String,
    /// `Some(Required | Optional)` on `idempotency_key` only (`x-frob-key`).
    pub key_rule: Option<KeyRule>,
    pub source: Source,
    /// Sorted named actions this property applies to (empty if default action only).
    pub actions: Vec<String>,
    /// Whether this property applies when `action` is omitted (the default action).
    pub on_default: bool,
}

/// MCP tool annotations derived from an entry's kinds and flags.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Annotations {
    pub read_only: bool,
    pub idempotent: bool,
    pub destructive: bool,
    pub open_world_hint: bool,
}

/// Failure raised by the `common-property-shadowed` lint.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShadowError {
    pub entry: String,
    pub property: String,
    pub cli_flag: String,
    pub shadowed: String,
}

impl fmt::Display for ShadowError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "entry `{}` input property `{}` (`{}`) shadows {}",
            self.entry, self.property, self.cli_flag, self.shadowed
        )
    }
}

impl std::error::Error for ShadowError {}

/// Failure raised while building a derived JSON Schema document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeriveError {
    InvalidSchema(String),
    Canon(CanonError),
}

impl fmt::Display for DeriveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidSchema(msg) => write!(f, "invalid input schema: {msg}"),
            Self::Canon(err) => write!(f, "canonical JSON error: {err}"),
        }
    }
}

impl std::error::Error for DeriveError {}

impl From<CanonError> for DeriveError {
    fn from(err: CanonError) -> Self {
        Self::Canon(err)
    }
}

// ---------------------------------------------------------------------------
// Unified borrowed view over static VerbEntry and parsed OwnedVerbEntry
// ---------------------------------------------------------------------------

#[derive(Clone, Copy)]
struct WaitView<'a> {
    cli: &'a str,
    mcp_ms: i64,
    mcp_max_ms: i64,
}

#[derive(Clone, Copy)]
struct ActionView<'a> {
    name: &'a str,
    kind: Kind,
    paginated: bool,
    destructive: bool,
}

#[derive(Clone, Copy)]
enum ParamCategory {
    ArtifactInput,
    ArtifactRef,
    ArtifactIdentity,
    ForeignFile,
}

struct EntryView<'a> {
    name: &'a str,
    cluster: &'a str,
    kind: Kind,
    tier: Tier,
    refusals: Vec<u16>,
    idempotent: bool,
    cost_model: &'a str,
    task_clusters: Vec<&'a str>,
    examples: Vec<&'a str>,
    purpose: &'a str,
    use_when: &'a str,
    wait: Option<WaitView<'a>>,
    actions: Vec<ActionView<'a>>,
    paginated: bool,
    estimable: bool,
    destructive: bool,
    key_optional: bool,
    params: Vec<(&'a str, ParamCategory)>,
}

impl<'a> EntryView<'a> {
    fn from_static(e: &'a VerbEntry) -> Self {
        let mut actions: Vec<ActionView<'a>> = e
            .actions
            .iter()
            .map(|a| ActionView {
                name: a.name,
                kind: a.kind,
                paginated: a.paginated,
                destructive: a.destructive,
            })
            .collect();
        actions.sort_by(|l, r| l.name.as_bytes().cmp(r.name.as_bytes()));

        let mut params: Vec<(&'a str, ParamCategory)> = e
            .params
            .iter()
            .map(|(k, pk)| {
                let cat = match pk {
                    ParamKind::ArtifactInput(_) => ParamCategory::ArtifactInput,
                    ParamKind::ArtifactRef(_) => ParamCategory::ArtifactRef,
                    ParamKind::ArtifactIdentity(_) => ParamCategory::ArtifactIdentity,
                    ParamKind::ForeignFile => ParamCategory::ForeignFile,
                };
                (*k, cat)
            })
            .collect();
        params.sort_by(|l, r| l.0.as_bytes().cmp(r.0.as_bytes()));

        Self {
            name: e.name,
            cluster: e.cluster,
            kind: e.kind,
            tier: e.tier,
            refusals: e.refusals.iter().map(|c| *c as u16).collect(),
            idempotent: e.idempotent,
            cost_model: e.cost_model,
            task_clusters: e.task_clusters.to_vec(),
            examples: e.examples.to_vec(),
            purpose: e.purpose,
            use_when: e.use_when,
            wait: e.wait.map(|w| WaitView {
                cli: w.cli,
                mcp_ms: i64::from(w.mcp_ms),
                mcp_max_ms: i64::from(w.mcp_max_ms),
            }),
            actions,
            paginated: e.paginated,
            estimable: e.estimable,
            destructive: e.destructive,
            key_optional: e.key_optional,
            params,
        }
    }

    fn from_owned(e: &'a OwnedVerbEntry) -> Self {
        let mut actions: Vec<ActionView<'a>> = e
            .actions
            .iter()
            .map(|a| ActionView {
                name: a.name.as_str(),
                kind: Kind::parse(&a.kind_raw).unwrap_or(Kind::Read),
                paginated: a.paginated,
                destructive: a.effective_destructive(),
            })
            .collect();
        actions.sort_by(|l, r| l.name.as_bytes().cmp(r.name.as_bytes()));

        let params: Vec<(&'a str, ParamCategory)> = e
            .params
            .iter()
            .map(|(k, pk)| {
                let cat = match pk {
                    OwnedParamKind::ArtifactInput(_) => ParamCategory::ArtifactInput,
                    OwnedParamKind::ArtifactRef(_) => ParamCategory::ArtifactRef,
                    OwnedParamKind::ArtifactIdentity(_) => ParamCategory::ArtifactIdentity,
                    OwnedParamKind::ForeignFile => ParamCategory::ForeignFile,
                };
                (k.as_str(), cat)
            })
            .collect();

        let refusals = e
            .refusals
            .iter()
            .filter_map(|s| s.strip_prefix("FROB-REF-")?.parse::<u16>().ok())
            .collect();

        Self {
            name: e.name.as_str(),
            cluster: e.cluster.as_str(),
            kind: Kind::parse(&e.kind_raw).unwrap_or(Kind::Read),
            tier: Tier::parse(&e.tier_raw).unwrap_or(Tier::S),
            refusals,
            idempotent: e.idempotent,
            cost_model: e.cost_model.as_str(),
            task_clusters: e.task_clusters.iter().map(String::as_str).collect(),
            examples: e.examples.iter().map(String::as_str).collect(),
            purpose: e.purpose.as_str(),
            use_when: e.use_when.as_str(),
            wait: e.wait.as_ref().map(|w| WaitView {
                cli: w.cli.as_str(),
                mcp_ms: w.mcp_ms,
                mcp_max_ms: w.mcp_max_ms,
            }),
            actions,
            paginated: e.paginated,
            estimable: e.estimable,
            destructive: e.effective_destructive(),
            key_optional: e.key_optional,
            params,
        }
    }

    fn wait_bounds(&self) -> WaitView<'a> {
        self.wait.unwrap_or(WaitView {
            cli: "to-completion",
            mcp_ms: DEFAULT_SYNC_WAIT_MS,
            mcp_max_ms: DEFAULT_SYNC_WAIT_MAX_MS,
        })
    }

    fn has_kind(&self, k: Kind) -> bool {
        self.kind == k || self.actions.iter().any(|a| a.kind == k)
    }
}

// ---------------------------------------------------------------------------
// Core pure derivation functions
// ---------------------------------------------------------------------------

struct PropScope {
    source: Source,
    on_default: bool,
    actions: Vec<String>,
}

impl PropScope {
    fn new(source: Source) -> Self {
        Self {
            source,
            on_default: false,
            actions: Vec::new(),
        }
    }

    fn mark(&mut self, action: Option<&str>) {
        match action {
            None => self.on_default = true,
            Some(a) => {
                if !self.actions.iter().any(|x| x == a) {
                    self.actions.push(a.to_owned());
                }
            }
        }
    }

    fn active(&self) -> bool {
        self.on_default || !self.actions.is_empty()
    }

    fn description_with_scope(&self, base: &str, total_actions: usize) -> String {
        if total_actions == 0 || (self.on_default && self.actions.len() == total_actions) {
            return base.to_owned();
        }
        match (self.on_default, self.actions.as_slice()) {
            (true, []) => format!("{base} Applies to the default action."),
            (true, acts) => format!(
                "{base} Applies to the default action and action(s): {}.",
                acts.join(", ")
            ),
            (false, [one]) => format!("{base} Applies to action `{one}`."),
            (false, acts) => format!("{base} Applies to actions: {}.", acts.join(", ")),
        }
    }
}

fn derive_common_properties(v: &EntryView<'_>) -> Vec<CommonProperty> {
    let mut sync_wait = PropScope::new(Source::Kind(Kind::Run));
    let mut key_prop = PropScope::new(if v.has_kind(Kind::Run) {
        Source::Kind(Kind::Run)
    } else {
        Source::Kind(Kind::Mutate)
    });
    let mut budget = PropScope::new(Source::Kind(Kind::Run));
    let mut estimate = PropScope::new(if v.has_kind(Kind::Run) {
        Source::Kind(Kind::Run)
    } else {
        Source::Field("estimable")
    });
    let mut on_cancel = PropScope::new(Source::Kind(Kind::Run));
    let mut env_scope = PropScope::new(Source::Kind(Kind::Run));
    let mut expected_parent = PropScope::new(Source::Kind(Kind::Mutate));
    let mut cursor = PropScope::new(Source::Field("paginated"));
    let mut limit = PropScope::new(Source::Field("paginated"));
    let mut follow = PropScope::new(Source::Field("paginated"));

    let mut visit = |kind: Kind, paginated: bool, act: Option<&str>| {
        match kind {
            Kind::Run => {
                sync_wait.mark(act);
                key_prop.mark(act);
                budget.mark(act);
                estimate.mark(act);
                on_cancel.mark(act);
                env_scope.mark(act);
            }
            Kind::Mutate => {
                key_prop.mark(act);
                expected_parent.mark(act);
            }
            Kind::Read => {
                if paginated {
                    cursor.mark(act);
                    limit.mark(act);
                    follow.mark(act);
                }
            }
            Kind::Admin => {}
        }
        if kind != Kind::Run && v.estimable {
            estimate.mark(act);
        }
    };

    visit(v.kind, v.paginated, None);
    for a in &v.actions {
        visit(a.kind, a.paginated, Some(a.name));
    }

    let total_actions = v.actions.len();
    let wait = v.wait_bounds();
    let key_rule = if v.key_optional {
        KeyRule::Optional
    } else {
        KeyRule::Required
    };

    let mut out = Vec::new();
    if sync_wait.active() {
        let desc = sync_wait.description_with_scope(
            "Bounded synchronous wait in milliseconds (CLI waits to completion).",
            total_actions,
        );
        out.push(CommonProperty {
            name: "sync_wait_ms",
            cli_flag: "--sync-wait-ms",
            schema_json: canon_obj(|o| {
                o.int53("default", wait.mcp_ms)?;
                o.str("description", &desc)?;
                o.int53("maximum", wait.mcp_max_ms)?;
                o.int53("minimum", 0)?;
                o.str("type", "integer")?;
                o.str("x-frob-cli-default", wait.cli)?;
                o.str("x-frob-int-type", "int53")
            }),
            key_rule: None,
            source: sync_wait.source,
            actions: sync_wait.actions,
            on_default: sync_wait.on_default,
        });
    }
    if key_prop.active() {
        let desc =
            key_prop.description_with_scope("Caller-supplied idempotency key.", total_actions);
        out.push(CommonProperty {
            name: "idempotency_key",
            cli_flag: "--key",
            schema_json: canon_obj(|o| {
                o.str("description", &desc)?;
                o.int53("minLength", 1)?;
                o.str("type", "string")?;
                o.str("x-frob-key", key_rule.as_str())
            }),
            key_rule: Some(key_rule),
            source: key_prop.source,
            actions: key_prop.actions,
            on_default: key_prop.on_default,
        });
    }
    if budget.active() {
        let desc = budget.description_with_scope(
            "Per-unit compute budget caps (e.g. gpu_s, cpu_core_s).",
            total_actions,
        );
        out.push(CommonProperty {
            name: "budget",
            cli_flag: "--budget",
            schema_json: canon_obj(|o| {
                o.object("additionalProperties", |ao| {
                    ao.int53("minimum", 0)?;
                    ao.str("type", "number")
                })?;
                o.str("description", &desc)?;
                o.str("type", "object")
            }),
            key_rule: None,
            source: budget.source,
            actions: budget.actions,
            on_default: budget.on_default,
        });
    }
    if estimate.active() {
        let desc = estimate.description_with_scope(
            "Return resource estimate interval without executing work.",
            total_actions,
        );
        out.push(CommonProperty {
            name: "estimate_only",
            cli_flag: "--estimate",
            schema_json: canon_obj(|o| {
                o.bool("default", false)?;
                o.str("description", &desc)?;
                o.str("type", "boolean")
            }),
            key_rule: None,
            source: estimate.source,
            actions: estimate.actions,
            on_default: estimate.on_default,
        });
    }
    if on_cancel.active() {
        let desc = on_cancel.description_with_scope(
            "Behaviour on interrupted synchronous wait (leave or cancel_job).",
            total_actions,
        );
        out.push(CommonProperty {
            name: "on_cancel",
            cli_flag: "--on-cancel",
            schema_json: canon_obj(|o| {
                o.str("default", "leave")?;
                o.str("description", &desc)?;
                o.array("enum", |a| {
                    a.push_str("leave");
                    a.push_str("cancel_job");
                    Ok(())
                })?;
                o.str("type", "string")
            }),
            key_rule: None,
            source: on_cancel.source,
            actions: on_cancel.actions,
            on_default: on_cancel.on_default,
        });
    }
    if env_scope.active() {
        let desc = env_scope.description_with_scope(
            "Approved compute-envelope scope (gate:<id>, campaign:<id>, or pretrain:<run>).",
            total_actions,
        );
        out.push(CommonProperty {
            name: "envelope_scope",
            cli_flag: "--envelope-scope",
            schema_json: canon_obj(|o| {
                o.str("description", &desc)?;
                o.str("pattern", ENVELOPE_SCOPE_PATTERN)?;
                o.str("type", "string")
            }),
            key_rule: None,
            source: env_scope.source,
            actions: env_scope.actions,
            on_default: env_scope.on_default,
        });
    }
    if expected_parent.active() {
        let desc = expected_parent.description_with_scope(
            "Expected lineage head identifier; stale values fail with FROB-REF-804.",
            total_actions,
        );
        out.push(CommonProperty {
            name: "expected_parent",
            cli_flag: "--expected-parent",
            schema_json: canon_obj(|o| {
                o.str("description", &desc)?;
                o.int53("minLength", 1)?;
                o.str("type", "string")
            }),
            key_rule: None,
            source: expected_parent.source,
            actions: expected_parent.actions,
            on_default: expected_parent.on_default,
        });
    }
    if cursor.active() {
        let desc = cursor.description_with_scope("Opaque pagination cursor.", total_actions);
        out.push(CommonProperty {
            name: "cursor",
            cli_flag: "--cursor",
            schema_json: canon_obj(|o| {
                o.str("description", &desc)?;
                o.int53("minLength", 1)?;
                o.str("type", "string")
            }),
            key_rule: None,
            source: cursor.source,
            actions: cursor.actions,
            on_default: cursor.on_default,
        });
    }
    if limit.active() {
        let desc = limit.description_with_scope("Maximum items per page.", total_actions);
        out.push(CommonProperty {
            name: "limit",
            cli_flag: "--limit",
            schema_json: canon_obj(|o| {
                o.str("description", &desc)?;
                o.int53("minimum", 1)?;
                o.str("type", "integer")?;
                o.str("x-frob-int-type", "int53")
            }),
            key_rule: None,
            source: limit.source,
            actions: limit.actions,
            on_default: limit.on_default,
        });
    }
    if follow.active() {
        let desc = follow.description_with_scope(
            "Continue streaming new items after the current frontier.",
            total_actions,
        );
        out.push(CommonProperty {
            name: "follow",
            cli_flag: "--follow",
            schema_json: canon_obj(|o| {
                o.bool("default", false)?;
                o.str("description", &desc)?;
                o.str("type", "boolean")
            }),
            key_rule: None,
            source: follow.source,
            actions: follow.actions,
            on_default: follow.on_default,
        });
    }
    if !v.actions.is_empty() {
        let names: Vec<String> = v.actions.iter().map(|a| a.name.to_owned()).collect();
        out.push(CommonProperty {
            name: "action",
            cli_flag: "--action",
            schema_json: canon_obj(|o| {
                o.str(
                    "description",
                    "Named action to invoke; omit for default action.",
                )?;
                o.array("enum", |a| {
                    for n in &names {
                        a.push_str(n);
                    }
                    Ok(())
                })?;
                o.str("type", "string")
            }),
            key_rule: None,
            source: Source::Field("actions"),
            actions: names,
            on_default: true,
        });
    }
    out
}

fn derive_input_schema_json(
    v: &EntryView<'_>,
    raw_input_schema: &str,
) -> Result<String, DeriveError> {
    let admitted = admit(raw_input_schema.as_bytes(), &Limits::default())
        .map_err(|e| DeriveError::InvalidSchema(e.to_string()))?;
    let members = admitted
        .value()
        .as_object()
        .ok_or_else(|| DeriveError::InvalidSchema("root must be a JSON object".into()))?;

    let common = derive_common_properties(v);
    let mut common_parsed = Vec::with_capacity(common.len());
    for cp in &common {
        let adm = admit(cp.schema_json.as_bytes(), &Limits::default())
            .map_err(|e| DeriveError::InvalidSchema(e.to_string()))?;
        common_parsed.push((cp.name, adm));
    }

    // Cross-action exclusion rules (`allOf`) so properties of another action fail as 802 at `/<prop>`.
    let mut action_rules: Vec<(Option<&str>, Vec<&'static str>)> = Vec::new();
    if !v.actions.is_empty() {
        let def_forbid: Vec<&'static str> = common
            .iter()
            .filter(|cp| cp.name != "action" && !cp.on_default)
            .map(|cp| cp.name)
            .collect();
        if !def_forbid.is_empty() {
            action_rules.push((None, def_forbid));
        }
        for act in &v.actions {
            let act_forbid: Vec<&'static str> = common
                .iter()
                .filter(|cp| cp.name != "action" && !cp.actions.iter().any(|x| x == act.name))
                .map(|cp| cp.name)
                .collect();
            if !act_forbid.is_empty() {
                action_rules.push((Some(act.name), act_forbid));
            }
        }
    }

    #[cfg(feature = "nc-key-in-required")]
    let plant_key_required = common.iter().any(|cp| cp.name == "idempotency_key");
    #[cfg(not(feature = "nc-key-in-required"))]
    let plant_key_required = false;

    let has_props = members.iter().any(|m| m.name == "properties");
    let has_all_of = members.iter().any(|m| m.name == "allOf");
    let has_required = members.iter().any(|m| m.name == "required");

    let mut w = CanonWriter::new();
    w.value()?.object(|o| {
        for m in members {
            match m.name.as_str() {
                "properties" => {
                    let row_props =
                        m.value
                            .as_object()
                            .ok_or_else(|| CanonError::InvalidInteger {
                                input: "properties".into(),
                                reason: "properties must be an object",
                            })?;
                    o.object("properties", |po| {
                        for rp in row_props {
                            po.member_term(&rp.name, &rp.value)?;
                        }
                        for (name, adm) in &common_parsed {
                            if !row_props.iter().any(|rp| rp.name == *name) {
                                po.member_term(name, adm.value())?;
                            }
                        }
                        Ok(())
                    })?;
                }
                "allOf" if !action_rules.is_empty() => {
                    o.array("allOf", |a| {
                        if let Some(items) = m.value.as_array() {
                            for it in items {
                                a.push_term(it)?;
                            }
                        }
                        write_action_rules(a, &action_rules)
                    })?;
                }
                "required" if plant_key_required => {
                    o.array("required", |a| {
                        let mut seen_key = false;
                        if let Some(items) = m.value.as_array() {
                            for it in items {
                                if it.as_str() == Some("idempotency_key") {
                                    seen_key = true;
                                }
                                a.push_term(it)?;
                            }
                        }
                        if !seen_key {
                            a.push_str("idempotency_key");
                        }
                        Ok(())
                    })?;
                }
                _ => o.member_term(&m.name, &m.value)?,
            }
        }
        if !has_props && !common_parsed.is_empty() {
            o.object("properties", |po| {
                for (name, adm) in &common_parsed {
                    po.member_term(name, adm.value())?;
                }
                Ok(())
            })?;
        }
        if !has_all_of && !action_rules.is_empty() {
            o.array("allOf", |a| write_action_rules(a, &action_rules))?;
        }
        if !has_required && plant_key_required {
            o.array("required", |a| {
                a.push_str("idempotency_key");
                Ok(())
            })?;
        }
        Ok(())
    })?;
    Ok(w.finish()?.as_str().to_owned())
}

fn write_action_rules(
    a: &mut frob_core::canon::Array<'_>,
    rules: &[(Option<&str>, Vec<&'static str>)],
) -> Result<(), CanonError> {
    for (act_opt, forbidden) in rules {
        a.push_object(|ro| {
            match act_opt {
                None => ro.object("if", |io| {
                    io.object("not", |no| {
                        no.array("required", |ra| {
                            ra.push_str("action");
                            Ok(())
                        })
                    })
                })?,
                Some(act) => ro.object("if", |io| {
                    io.object("properties", |po| {
                        po.object("action", |ao| ao.str("const", act))
                    })?;
                    io.array("required", |ra| {
                        ra.push_str("action");
                        Ok(())
                    })
                })?,
            }
            ro.object("then", |to| {
                to.object("properties", |po| {
                    for &prop in forbidden {
                        po.bool(prop, false)?;
                    }
                    Ok(())
                })
            })
        })?;
    }
    Ok(())
}

fn push_kind_refusals(
    map: &mut BTreeMap<u16, RefusalSource>,
    kind: Kind,
    key_optional: bool,
    paginated: bool,
    action: Option<&str>,
) {
    let mut add = |code: u16, origin: RefusalOrigin| {
        map.entry(code).or_insert_with(|| RefusalSource {
            origin,
            action: action.map(str::to_owned),
        });
    };
    let origin = RefusalOrigin::Kind(kind);
    match kind {
        Kind::Read | Kind::Admin => {
            for c in [720, 723, 724, 802] {
                add(c, origin);
            }
        }
        Kind::Run => {
            for c in [701, 702, 703, 704, 720, 723, 724, 761, 802, 803] {
                add(c, origin);
            }
            #[cfg(not(feature = "nc-omit-805"))]
            if !key_optional {
                add(805, origin);
            }
        }
        Kind::Mutate => {
            for c in [720, 723, 724, 725, 802, 803, 804] {
                add(c, origin);
            }
            if !key_optional {
                add(805, origin);
            }
        }
    }
    #[cfg(feature = "nc-append-990")]
    add(990, origin);

    if kind == Kind::Read && paginated {
        add(806, RefusalOrigin::Field("paginated"));
        add(807, RefusalOrigin::Field("paginated"));
    }
}

fn derive_all_refusals(
    v: &EntryView<'_>,
    raw_input_schema: &str,
    refusals: &RefusalRegistry,
) -> Vec<ListedRefusal> {
    let mut map: BTreeMap<u16, RefusalSource> = BTreeMap::new();
    for &code in &v.refusals {
        map.entry(code).or_insert(RefusalSource {
            origin: RefusalOrigin::Chapter,
            action: None,
        });
    }
    push_kind_refusals(&mut map, v.kind, v.key_optional, v.paginated, None);
    for act in &v.actions {
        push_kind_refusals(
            &mut map,
            act.kind,
            v.key_optional,
            act.paginated,
            Some(act.name),
        );
    }

    // Variation 4: any int53 property at any depth of generated input schema (sync_wait_ms, limit, or row schema).
    let has_int53 =
        |kind: Kind, paginated: bool| kind == Kind::Run || (kind == Kind::Read && paginated);
    let named_int53 = v.actions.iter().find(|a| has_int53(a.kind, a.paginated));
    let default_int53 = has_int53(v.kind, v.paginated) || schema_str_has_int53(raw_input_schema);
    if default_int53 || named_int53.is_some() {
        map.entry(808).or_insert(RefusalSource {
            origin: RefusalOrigin::Field("int53"),
            action: named_int53
                .filter(|_| !default_int53)
                .map(|a| a.name.to_owned()),
        });
    }

    // Variations 7, 8, 9: params-driven codes (734/828 gated by presence in refusal table).
    let mut has_path_param = false;
    for &(_, cat) in &v.params {
        let mut add_param = |code: u16| {
            map.entry(code).or_insert(RefusalSource {
                origin: RefusalOrigin::Field("params"),
                action: None,
            });
        };
        match cat {
            ParamCategory::ArtifactInput => {
                add_param(891);
                has_path_param = true;
            }
            ParamCategory::ArtifactRef => add_param(891),
            ParamCategory::ArtifactIdentity => {
                add_param(891);
                add_param(892);
                has_path_param = true;
            }
            ParamCategory::ForeignFile => {
                add_param(893);
                has_path_param = true;
            }
        }
    }
    if has_path_param {
        for code in [734u16, 828u16] {
            let token = format!("FROB-REF-{code}");
            if refusals.lookup(&token, "derive::all_refusals").is_ok() {
                map.entry(code).or_insert(RefusalSource {
                    origin: RefusalOrigin::Field("params"),
                    action: None,
                });
            }
        }
    }

    map.into_iter()
        .map(|(code, source)| ListedRefusal { code, source })
        .collect()
}

fn schema_str_has_int53(raw: &str) -> bool {
    let Ok(adm) = admit(raw.as_bytes(), &Limits::default()) else {
        return false;
    };
    schema_node_has_int53(adm.value())
}

fn schema_node_has_int53(node: &Json<'_>) -> bool {
    let Some(members) = node.as_object() else {
        return false;
    };
    if let Some(int_ty) = node.get("x-frob-int-type").and_then(|v| v.as_str()) {
        return !matches!(int_ty, "u64str" | "i64str");
    }
    if let Some(r) = node.get("$ref").and_then(|v| v.as_str())
        && r.contains("int53")
    {
        return true;
    }
    if let Some(ty) = node.get("type") {
        let is_int_or_unknown = |s: &str| match s {
            "integer" => true,
            "string" | "boolean" | "number" | "object" | "array" | "null" => false,
            _ => true, // Fail-closed default per NJL-01 Considerations.
        };
        if let Some(s) = ty.as_str() {
            if is_int_or_unknown(s) {
                return true;
            }
        } else if let Some(arr) = ty.as_array()
            && arr
                .iter()
                .any(|x| x.as_str().is_some_and(is_int_or_unknown))
        {
            return true;
        }
    }
    for m in members {
        match m.name.as_str() {
            "properties" | "patternProperties" | "$defs" => {
                if let Some(dict) = m.value.as_object()
                    && dict.iter().any(|p| schema_node_has_int53(&p.value))
                {
                    return true;
                }
            }
            "items" | "additionalProperties" | "propertyNames" | "not" | "if" | "then" | "else" => {
                if schema_node_has_int53(&m.value) {
                    return true;
                }
            }
            "allOf" | "anyOf" | "oneOf" | "prefixItems" => {
                if let Some(arr) = m.value.as_array()
                    && arr.iter().any(|item| schema_node_has_int53(&item.value))
                {
                    return true;
                }
            }
            _ => {}
        }
    }
    false
}

fn derive_short_doc(
    v: &EntryView<'_>,
    raw_input_schema: &str,
    refusals: &RefusalRegistry,
) -> String {
    let cost_raw = v.cost_model.trim();
    let cost_body = if let Some(rest) = cost_raw.strip_prefix("static:") {
        rest.trim().trim_end_matches('.').to_owned()
    } else {
        format!("estimated ({cost_raw})")
    };
    let codes: Vec<String> = derive_all_refusals(v, raw_input_schema, refusals)
        .into_iter()
        .map(|r| r.code.to_string())
        .collect();
    let mut parts = vec![
        v.purpose.trim().to_owned(),
        v.use_when.trim().to_owned(),
        format!("Cost: {cost_body}."),
        format!("Refusals (FROB-REF-): {}.", codes.join(", ")),
    ];
    if let Some(ex) = v.examples.first() {
        parts.push(format!("Example: {}", ex.trim()));
    }
    parts.join(" ")
}

fn derive_tags(v: &EntryView<'_>) -> Vec<String> {
    let mut out = vec![format!("cluster:{}", v.cluster)];
    let mut kinds = BTreeSet::new();
    kinds.insert(v.kind.as_str());
    for a in &v.actions {
        kinds.insert(a.kind.as_str());
    }
    for k in kinds {
        out.push(format!("kind:{k}"));
    }
    let mut seen_tc = BTreeSet::new();
    for &tc in &v.task_clusters {
        if seen_tc.insert(tc) {
            out.push(format!("task:{tc}"));
        }
    }
    out.push(format!("tier:{}", v.tier.as_str()));
    out
}

fn derive_annotations(v: &EntryView<'_>) -> Annotations {
    let read_only = v.kind == Kind::Read && v.actions.iter().all(|a| a.kind == Kind::Read);
    let idempotent = v.idempotent && !v.has_kind(Kind::Run) && !v.has_kind(Kind::Mutate);
    let destructive = (v.kind == Kind::Admin && v.destructive)
        || v.actions
            .iter()
            .any(|a| a.kind == Kind::Admin && a.destructive);
    Annotations {
        read_only,
        idempotent,
        destructive,
        open_world_hint: false,
    }
}

fn derive_check_shadowing(v: &EntryView<'_>, served_input_schema: &str) -> Result<(), ShadowError> {
    let Ok(adm) = admit(served_input_schema.as_bytes(), &Limits::default()) else {
        return Ok(());
    };
    let Some(props) = adm.value().get("properties").and_then(|p| p.as_object()) else {
        return Ok(());
    };
    let common = derive_common_properties(v);
    for prop in props {
        let bare_flag = prop.name.replace('_', "-");
        let cli_flag = format!("--{bare_flag}");
        for cp in &common {
            if prop.name == cp.name || cli_flag == cp.cli_flag {
                return Err(ShadowError {
                    entry: v.name.to_owned(),
                    property: prop.name.clone(),
                    cli_flag,
                    shadowed: format!(
                        "common property `{}` (flag `{}`, from {})",
                        cp.name, cp.cli_flag, cp.source
                    ),
                });
            }
        }
        if PROCESS_OPTIONS.contains(&bare_flag.as_str()) {
            return Err(ShadowError {
                entry: v.name.to_owned(),
                property: prop.name.clone(),
                cli_flag: cli_flag.clone(),
                shadowed: format!("process option `{cli_flag}`"),
            });
        }
        if bare_flag == "help" {
            return Err(ShadowError {
                entry: v.name.to_owned(),
                property: prop.name.clone(),
                cli_flag,
                shadowed: "built-in CLI option `--help`".to_owned(),
            });
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Methods on VerbEntry and OwnedVerbEntry
// ---------------------------------------------------------------------------

impl VerbEntry {
    /// Common input properties in fixed order (kinds table, then variations 1..5).
    #[must_use]
    pub fn common_properties(&self) -> Vec<CommonProperty> {
        derive_common_properties(&EntryView::from_static(self))
    }

    /// Canonical JSON of `raw_input_schema` merged with [`Self::common_properties`] and
    /// cross-action exclusion rules.
    pub fn input_schema_with_common_fields_json(
        &self,
        raw_input_schema: &str,
    ) -> Result<String, DeriveError> {
        derive_input_schema_json(&EntryView::from_static(self), raw_input_schema)
    }

    /// Full refusal list (chapter + appended kind/variation codes), ascending, deduplicated.
    #[must_use]
    pub fn all_refusals_with(
        &self,
        raw_input_schema: &str,
        refusals: &RefusalRegistry,
    ) -> Vec<ListedRefusal> {
        derive_all_refusals(&EntryView::from_static(self), raw_input_schema, refusals)
    }

    /// Generated description (`short_doc`), at most 800 characters on a linted registry.
    #[must_use]
    pub fn short_doc_with(&self, raw_input_schema: &str, refusals: &RefusalRegistry) -> String {
        derive_short_doc(&EntryView::from_static(self), raw_input_schema, refusals)
    }

    /// Generated MCP tags (`cluster:*`, `kind:*`, `task:*`, `tier:*`).
    #[must_use]
    pub fn tags(&self) -> Vec<String> {
        derive_tags(&EntryView::from_static(self))
    }

    /// Derived MCP annotations (`read_only`, `idempotent`, `destructive`, `open_world_hint`).
    #[must_use]
    pub fn annotations(&self) -> Annotations {
        derive_annotations(&EntryView::from_static(self))
    }

    /// Run `common-property-shadowed` against the post-strip served input schema.
    pub fn check_shadowing(&self, served_input_schema: &str) -> Result<(), ShadowError> {
        derive_check_shadowing(&EntryView::from_static(self), served_input_schema)
    }
}

impl OwnedVerbEntry {
    /// Common input properties in fixed order (kinds table, then variations 1..5).
    #[must_use]
    pub fn common_properties(&self) -> Vec<CommonProperty> {
        derive_common_properties(&EntryView::from_owned(self))
    }

    /// Canonical JSON of `raw_input_schema` merged with [`Self::common_properties`] and
    /// cross-action exclusion rules.
    pub fn input_schema_with_common_fields_json(
        &self,
        raw_input_schema: &str,
    ) -> Result<String, DeriveError> {
        derive_input_schema_json(&EntryView::from_owned(self), raw_input_schema)
    }

    /// Full refusal list (chapter + appended kind/variation codes), ascending, deduplicated.
    #[must_use]
    pub fn all_refusals_with(
        &self,
        raw_input_schema: &str,
        refusals: &RefusalRegistry,
    ) -> Vec<ListedRefusal> {
        derive_all_refusals(&EntryView::from_owned(self), raw_input_schema, refusals)
    }

    /// Generated description (`short_doc`), at most 800 characters on a linted registry.
    #[must_use]
    pub fn short_doc_with(&self, raw_input_schema: &str, refusals: &RefusalRegistry) -> String {
        derive_short_doc(&EntryView::from_owned(self), raw_input_schema, refusals)
    }

    /// Same as [`Self::short_doc_with`], returning `Err(char_len)` if it exceeds 800 chars (`RL-11`).
    pub fn short_doc_checked_with(
        &self,
        raw_input_schema: &str,
        refusals: &RefusalRegistry,
    ) -> Result<String, usize> {
        let doc = self.short_doc_with(raw_input_schema, refusals);
        let len = doc.chars().count();
        if len > MAX_SHORT_DOC_CHARS {
            Err(len)
        } else {
            Ok(doc)
        }
    }

    /// Generated MCP tags (`cluster:*`, `kind:*`, `task:*`, `tier:*`).
    #[must_use]
    pub fn tags(&self) -> Vec<String> {
        derive_tags(&EntryView::from_owned(self))
    }

    /// Derived MCP annotations (`read_only`, `idempotent`, `destructive`, `open_world_hint`).
    #[must_use]
    pub fn annotations(&self) -> Annotations {
        derive_annotations(&EntryView::from_owned(self))
    }

    /// Run `common-property-shadowed` against the post-strip served input schema.
    pub fn check_shadowing(&self, served_input_schema: &str) -> Result<(), ShadowError> {
        derive_check_shadowing(&EntryView::from_owned(self), served_input_schema)
    }
}

// ---------------------------------------------------------------------------
// Canonical JSON Schema for `frob.verbs.entry/1` (schemas/frob.verbs.entry.1.json)
// ---------------------------------------------------------------------------

/// Generate the canonical D0 JSON Schema document for one `[[verb]]` entry (`frob.verbs.entry/1`),
/// derived from N32a-02's closed field table with an embedded minimal valid example.
#[must_use]
pub fn entry_schema_json() -> String {
    canon_obj(|o| {
        o.str("$id", "frob.verbs.entry/1")?;
        o.str("$schema", "https://json-schema.org/draft/2020-12/schema")?;
        o.bool("additionalProperties", false)?;
        o.str(
            "description",
            "Closed schema of one [[verb]] entry in registries/verbs.toml (N32a-02, NJL-01).",
        )?;
        o.array("examples", |ea| {
            ea.push_object(|ex| {
                ex.array("capabilities", |a| {
                    a.push_str("design:read");
                    Ok(())
                })?;
                ex.str("cluster", "design")?;
                ex.str("cost_model", "static: under 2 s for template-sized designs")?;
                ex.array("examples", |a| {
                    a.push_str("frob design check quad-a --json");
                    Ok(())
                })?;
                ex.str("first_milestone", "M1")?;
                ex.bool("idempotent", true)?;
                ex.str("input_schema", "frob.design.check.in/1")?;
                ex.str("kind", "read")?;
                ex.str("name", "design.check")?;
                ex.str("output_schema", "frob.design.check.out/1")?;
                ex.str("owner", "frob-design")?;
                ex.str(
                    "purpose",
                    "Run preflight stages and analyses marked for check; return findings with ranked repairs.",
                )?;
                ex.array("refusals", |a| {
                    a.push_str("FROB-REF-101");
                    Ok(())
                })?;
                ex.array("task_clusters", |a| {
                    a.push_str("author");
                    Ok(())
                })?;
                ex.str("tier", "S")?;
                ex.str("use_when", "After any edit and before any simulation.")
            })
        })?;
        o.object("properties", |p| {
            p.object("actions", |a| {
                a.object("items", |it| {
                    it.bool("additionalProperties", false)?;
                    it.object("properties", |ap| {
                        ap.object("capabilities", |c| str_array(c, 1, None))?;
                        ap.object("destructive", |d| d.str("type", "boolean"))?;
                        ap.object("kind", |k| kind_enum(k))?;
                        ap.object("name", |n| {
                            n.str("pattern", "^[a-z][a-z0-9-]*$")?;
                            n.str("type", "string")
                        })?;
                        ap.object("paginated", |pg| pg.str("type", "boolean"))
                    })?;
                    it.array("required", |r| {
                        r.push_str("name");
                        r.push_str("kind");
                        r.push_str("capabilities");
                        Ok(())
                    })?;
                    it.str("type", "object")
                })?;
                a.str("type", "array")
            })?;
            p.object("capabilities", |c| str_array(c, 1, None))?;
            p.object("cluster", |c| {
                c.str("pattern", "^[a-z][a-z0-9-]*$")?;
                c.str("type", "string")
            })?;
            p.object("conformance_only", |c| str_array(c, 1, None))?;
            p.object("cost_model", |c| non_empty_str(c))?;
            p.object("destructive", |d| d.str("type", "boolean"))?;
            p.object("estimable", |e| e.str("type", "boolean"))?;
            p.object("examples", |e| str_array(e, 1, Some(1)))?;
            p.object("exports", |e| e.str("type", "boolean"))?;
            p.object("first_milestone", |m| {
                m.array("enum", |a| {
                    for ms in ["M0", "M1", "M2", "M3", "M4", "M5", "M6", "M7", "M8"] {
                        a.push_str(ms);
                    }
                    Ok(())
                })?;
                m.str("type", "string")
            })?;
            p.object("idempotent", |i| i.str("type", "boolean"))?;
            p.object("input_schema", |s| schema_id_str(s))?;
            p.object("key_optional", |k| k.str("type", "boolean"))?;
            p.object("kind", |k| kind_enum(k))?;
            p.object("name", |n| {
                n.str(
                    "pattern",
                    "^[a-z][a-z0-9]*(-[a-z0-9]+)*(\\.[a-z][a-z0-9]*(-[a-z0-9]+)*)?$",
                )?;
                n.str("type", "string")
            })?;
            p.object("option_capabilities", |oc| {
                oc.object("additionalProperties", |ap| str_array(ap, 1, None))?;
                oc.str("type", "object")
            })?;
            p.object("output_schema", |s| schema_id_str(s))?;
            p.object("owner", |ow| non_empty_str(ow))?;
            p.object("paginated", |pg| pg.str("type", "boolean"))?;
            p.object("params", |pm| {
                pm.object("additionalProperties", |ap| {
                    ap.str(
                        "pattern",
                        "^(foreign-file|artifact-(input|ref|identity)<[^<> ]+>)$",
                    )?;
                    ap.str("type", "string")
                })?;
                pm.str("type", "object")
            })?;
            p.object("public", |pb| pb.str("type", "boolean"))?;
            p.object("purpose", |pu| {
                pu.int53("maxLength", 200)?;
                pu.int53("minLength", 1)?;
                pu.str("type", "string")
            })?;
            p.object("refusals", |rf| {
                rf.object("items", |it| {
                    it.str("pattern", "^FROB-REF-[1-9][0-9]{2,3}$")?;
                    it.str("type", "string")
                })?;
                rf.str("type", "array")
            })?;
            p.object("released", |rl| rl.str("type", "boolean"))?;
            p.object("schema_version", |sv| {
                sv.int53("minimum", 1)?;
                sv.str("type", "integer")
            })?;
            p.object("spends_physics", |sp| {
                sp.array("type", |a| {
                    a.push_str("boolean");
                    a.push_str("null");
                    Ok(())
                })
            })?;
            p.object("task_clusters", |tc| str_array(tc, 1, Some(1)))?;
            p.object("tier", |t| {
                t.array("enum", |a| {
                    a.push_str("S");
                    a.push_str("F");
                    a.push_str("M");
                    Ok(())
                })?;
                t.str("type", "string")
            })?;
            p.object("use_when", |uw| non_empty_str(uw))?;
            p.object("wait", |wt| {
                wt.bool("additionalProperties", false)?;
                wt.object("properties", |wp| {
                    wp.object("cli", |c| c.str("const", "to-completion"))?;
                    wp.object("mcp_max_ms", |m| {
                        m.int53("maximum", DEFAULT_SYNC_WAIT_MAX_MS)?;
                        m.int53("minimum", 0)?;
                        m.str("type", "integer")
                    })?;
                    wp.object("mcp_ms", |m| {
                        m.int53("maximum", DEFAULT_SYNC_WAIT_MAX_MS)?;
                        m.int53("minimum", 0)?;
                        m.str("type", "integer")
                    })
                })?;
                wt.array("required", |r| {
                    r.push_str("cli");
                    r.push_str("mcp_ms");
                    r.push_str("mcp_max_ms");
                    Ok(())
                })?;
                wt.array("type", |a| {
                    a.push_str("object");
                    a.push_str("null");
                    Ok(())
                })
            })
        })?;
        o.array("required", |r| {
            for k in [
                "name",
                "cluster",
                "kind",
                "tier",
                "capabilities",
                "input_schema",
                "output_schema",
                "refusals",
                "idempotent",
                "cost_model",
                "first_milestone",
                "task_clusters",
                "examples",
                "purpose",
                "use_when",
                "owner",
            ] {
                r.push_str(k);
            }
            Ok(())
        })?;
        o.str("title", "frob.verbs.entry/1")?;
        o.str("type", "object")
    })
}

fn non_empty_str(o: &mut frob_core::canon::Object<'_>) -> Result<(), CanonError> {
    o.int53("minLength", 1)?;
    o.str("type", "string")
}

fn schema_id_str(o: &mut frob_core::canon::Object<'_>) -> Result<(), CanonError> {
    o.str("pattern", "^frob(\\.[a-z0-9-]+)+/[1-9][0-9]*$")?;
    o.str("type", "string")
}

fn kind_enum(o: &mut frob_core::canon::Object<'_>) -> Result<(), CanonError> {
    o.array("enum", |a| {
        for k in ["read", "run", "mutate", "admin"] {
            a.push_str(k);
        }
        Ok(())
    })?;
    o.str("type", "string")
}

fn str_array(
    o: &mut frob_core::canon::Object<'_>,
    item_min_len: i64,
    min_items: Option<i64>,
) -> Result<(), CanonError> {
    o.object("items", |it| {
        it.int53("minLength", item_min_len)?;
        it.str("type", "string")
    })?;
    if let Some(mi) = min_items {
        o.int53("minItems", mi)?;
    }
    o.str("type", "array")
}

fn canon_obj<F>(f: F) -> String
where
    F: FnOnce(&mut frob_core::canon::Object<'_>) -> Result<(), CanonError>,
{
    let mut w = CanonWriter::new();
    let val: CanonValue<'_> = w.value().expect("fresh CanonWriter");
    val.object(f).expect("infallible canonical object schema");
    w.finish().expect("canonical bytes").as_str().to_owned()
}
