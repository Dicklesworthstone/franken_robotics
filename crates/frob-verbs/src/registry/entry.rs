//! Verb registry entry types, strict `frob.verbs/1` TOML-subset parser, TOML emitter
//! (for round-trip property testing), and canonical registry digest computation (bead N32a-02).
//!
//! Closed schema: any unknown top-level table, entry field, action member, or wait member
//! fails parsing as `RL-13` (`unknown-field`). Later-bead fields (`needs_groups`, `threats`,
//! `uses`, `consumes`, `claim_kinds`, `consumes_evidence`, `engines`, `memo`, `reports`,
//! `max_wall_ms`, `same_engine_as`) and top-level `[[admission_check]]` are refused until
//! their owning beads land.

use std::collections::BTreeMap;
use std::fmt::{self, Write as _};

use frob_core::canon::{CanonError, CanonWriter, Canonical, Value as CanonValue};
use frob_core::id::{SchemaName, identity};
use frob_core::refusal::RefusalCode;
use frob_core::registry::toml::{self, Fields, Spanned, Table, TomlError, TomlErrorKind, Value};

use super::names::{cli_path, tool_name};
use super::schema_id::SchemaId;

/// Registry schema identifier for `registries/verbs.toml`.
pub const VERBS_REGISTRY_SCHEMA: &str = "frob.verbs/1";
/// Entry schema identifier for a single `[[verb]]` entry.
pub const VERBS_ENTRY_SCHEMA: &str = "frob.verbs.entry/1";

/// Operational kind of a verb or action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Kind {
    Read,
    Run,
    Mutate,
    Admin,
}

impl Kind {
    /// Parse canonical TOML spelling (`read`, `run`, `mutate`, `admin`).
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "read" => Some(Self::Read),
            "run" => Some(Self::Run),
            "mutate" => Some(Self::Mutate),
            "admin" => Some(Self::Admin),
            _ => None,
        }
    }

    /// Canonical lowercase wire/TOML spelling.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Read => "read",
            Self::Run => "run",
            Self::Mutate => "mutate",
            Self::Admin => "admin",
        }
    }
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Mechanism maturity tier (`S` Solid, `F` Frontier, `M` Moonshot).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Tier {
    S,
    F,
    M,
}

impl Tier {
    /// Parse canonical TOML spelling (`S`, `F`, `M`).
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "S" => Some(Self::S),
            "F" => Some(Self::F),
            "M" => Some(Self::M),
            _ => None,
        }
    }

    /// Canonical single-letter spelling.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::S => "S",
            Self::F => "F",
            Self::M => "M",
        }
    }
}

impl fmt::Display for Tier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Milestone in which a verb first appears (`M0`..=`M8`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Milestone {
    M0,
    M1,
    M2,
    M3,
    M4,
    M5,
    M6,
    M7,
    M8,
}

impl Milestone {
    /// Parse canonical milestone spelling (`M0`..=`M8`).
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "M0" => Some(Self::M0),
            "M1" => Some(Self::M1),
            "M2" => Some(Self::M2),
            "M3" => Some(Self::M3),
            "M4" => Some(Self::M4),
            "M5" => Some(Self::M5),
            "M6" => Some(Self::M6),
            "M7" => Some(Self::M7),
            "M8" => Some(Self::M8),
            _ => None,
        }
    }

    /// Canonical milestone spelling (`"M0"`..`"M8"`).
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::M0 => "M0",
            Self::M1 => "M1",
            Self::M2 => "M2",
            Self::M3 => "M3",
            Self::M4 => "M4",
            Self::M5 => "M5",
            Self::M6 => "M6",
            Self::M7 => "M7",
            Self::M8 => "M8",
        }
    }
}

impl fmt::Display for Milestone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Capability scope identifier required on a session token (e.g. `"design:read"`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Scope(pub &'static str);

impl Scope {
    /// Borrow the raw scope identifier string.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

impl fmt::Display for Scope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}

/// Per-surface bounded synchronous wait defaults for a `run` verb.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WaitDefault {
    pub cli: &'static str,
    pub mcp_ms: u32,
    pub mcp_max_ms: u32,
}

/// Artifact / foreign-file parameter kind declared in `params` (N34-24).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ParamKind {
    ArtifactInput(&'static str),
    ArtifactRef(&'static str),
    ArtifactIdentity(&'static str),
    ForeignFile,
}

impl ParamKind {
    /// Render canonical TOML/wire spelling.
    #[must_use]
    pub fn to_wire_string(self) -> String {
        match self {
            Self::ArtifactInput(f) => format!("artifact-input<{f}>"),
            Self::ArtifactRef(f) => format!("artifact-ref<{f}>"),
            Self::ArtifactIdentity(f) => format!("artifact-identity<{f}>"),
            Self::ForeignFile => String::from("foreign-file"),
        }
    }
}

/// Owned counterpart of [`ParamKind`] produced while parsing `registries/verbs.toml`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OwnedParamKind {
    ArtifactInput(String),
    ArtifactRef(String),
    ArtifactIdentity(String),
    ForeignFile,
}

impl OwnedParamKind {
    /// Parse one `params.<arg>` value string.
    pub fn parse(s: &str) -> Result<Self, &'static str> {
        if s == "foreign-file" {
            return Ok(Self::ForeignFile);
        }
        if let Some(inner) = strip_angle_arg(s, "artifact-input") {
            return Ok(Self::ArtifactInput(inner.to_owned()));
        }
        if let Some(inner) = strip_angle_arg(s, "artifact-ref") {
            return Ok(Self::ArtifactRef(inner.to_owned()));
        }
        if let Some(inner) = strip_angle_arg(s, "artifact-identity") {
            return Ok(Self::ArtifactIdentity(inner.to_owned()));
        }
        Err("expected artifact-input<F>, artifact-ref<F>, artifact-identity<F>, or foreign-file")
    }

    /// Render canonical TOML/wire spelling.
    #[must_use]
    pub fn to_wire_string(&self) -> String {
        match self {
            Self::ArtifactInput(f) => format!("artifact-input<{f}>"),
            Self::ArtifactRef(f) => format!("artifact-ref<{f}>"),
            Self::ArtifactIdentity(f) => format!("artifact-identity<{f}>"),
            Self::ForeignFile => String::from("foreign-file"),
        }
    }
}

fn strip_angle_arg<'a>(s: &'a str, prefix: &str) -> Option<&'a str> {
    let rest = s
        .strip_prefix(prefix)?
        .strip_prefix('<')?
        .strip_suffix('>')?;
    if rest.is_empty() || rest.contains(['<', '>', ' ', '\t', '\n']) {
        return None;
    }
    Some(rest)
}

/// One additional named action on a multi-action verb entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Action {
    pub name: &'static str,
    pub kind: Kind,
    pub capabilities: &'static [Scope],
    pub paginated: bool,
    /// `true` by default on an `admin` action; `false` on non-`admin` actions.
    pub destructive: bool,
}

/// Resolution result for a verb's default action (`None`) or named action (`Some(name)`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResolvedAction {
    pub name: Option<&'static str>,
    pub kind: Kind,
    pub capabilities: &'static [Scope],
    pub paginated: bool,
    pub destructive: bool,
    pub needs_key: bool,
}

/// Static task-cluster declaration emitted into `TASK_CLUSTERS` (empty at M0; populated by NJL-05).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TaskCluster {
    pub name: &'static str,
    pub activity: &'static str,
    pub default: bool,
    pub tools: &'static [&'static str],
}

/// One compiled static entry in `VERBS`.
///
/// Later-bead members added by their owning beads (each refused as `unknown-field` until then):
/// `needs_groups` (N33c-32), `threats` (N37-01), `uses` (N35-07), `consumes` and `claim_kinds`
/// (BD-308), `consumes_evidence` (N37-26), `engines` (N32ax-02), `memo` (BD-111), `reports`
/// (BD-139), `max_wall_ms` (N37-13), `same_engine_as` (N28c-29).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VerbEntry {
    pub name: &'static str,
    pub cluster: &'static str,
    pub kind: Kind,
    pub tier: Tier,
    pub capabilities: &'static [Scope],
    pub input_schema: SchemaId,
    pub output_schema: SchemaId,
    pub refusals: &'static [RefusalCode],
    pub idempotent: bool,
    pub cost_model: &'static str,
    pub first_milestone: Milestone,
    pub task_clusters: &'static [&'static str],
    pub examples: &'static [&'static str],
    pub purpose: &'static str,
    pub use_when: &'static str,
    pub owner: &'static str,
    pub wait: Option<WaitDefault>,
    pub actions: &'static [Action],
    pub schema_version: u32,
    pub paginated: bool,
    pub exports: bool,
    pub estimable: bool,
    pub destructive: bool,
    pub public: bool,
    pub key_optional: bool,
    pub option_capabilities: &'static [(&'static str, &'static [Scope])],
    pub spends_physics: Option<bool>,
    pub released: bool,
    pub params: &'static [(&'static str, ParamKind)],
    pub conformance_only: &'static [&'static str],
}

impl VerbEntry {
    /// MCP tool name (`"."` and `"-"` mapped to `"_"`).
    #[must_use]
    pub fn tool_name(&self) -> String {
        tool_name(self.name)
    }

    /// CLI command path segments (`"design.check"` → `["design", "check"]`).
    #[must_use]
    pub fn cli_path(&self) -> Vec<&'static str> {
        cli_path(self.name)
    }

    /// Return `true` when the default action requires an idempotency key (`run`/`mutate`
    /// unless `key_optional = true`).
    #[must_use]
    pub const fn needs_key(&self) -> bool {
        match self.kind {
            Kind::Run | Kind::Mutate => !self.key_optional,
            Kind::Read | Kind::Admin => false,
        }
    }

    /// Return `true` if either the default action or any named action has kind `kind`.
    #[must_use]
    pub fn has_kind(&self, kind: Kind) -> bool {
        self.kind == kind || self.actions.iter().any(|a| a.kind == kind)
    }

    /// Resolve `action` (`None` for the unnamed default action, `Some(name)` for a declared
    /// named action). Unknown action names fail with [`RefusalCode::FrobRef802`] (NJL-01 action rule 1).
    pub fn resolve_action(&self, action: Option<&str>) -> Result<ResolvedAction, RefusalCode> {
        match action {
            None => Ok(ResolvedAction {
                name: None,
                kind: self.kind,
                capabilities: self.capabilities,
                paginated: self.paginated,
                destructive: self.destructive,
                needs_key: self.needs_key(),
            }),
            Some(requested) => {
                let found = self
                    .actions
                    .iter()
                    .find(|a| a.name == requested)
                    .ok_or(RefusalCode::FrobRef802)?;
                let needs_key = match found.kind {
                    Kind::Run | Kind::Mutate => !self.key_optional,
                    Kind::Read | Kind::Admin => false,
                };
                Ok(ResolvedAction {
                    name: Some(found.name),
                    kind: found.kind,
                    capabilities: found.capabilities,
                    paginated: found.paginated,
                    destructive: found.destructive,
                    needs_key,
                })
            }
        }
    }
}

/// Owned `wait` table parsed from TOML.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnedWaitDefault {
    pub cli: String,
    pub mcp_ms: i64,
    pub mcp_max_ms: i64,
}

/// Owned `actions[*]` table parsed from TOML.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnedAction {
    pub name: String,
    pub kind_raw: String,
    pub capabilities: Vec<String>,
    pub paginated: bool,
    /// `Some(b)` when written explicitly in TOML; `None` when omitted.
    pub destructive_explicit: Option<bool>,
}

impl OwnedAction {
    /// Effective `destructive` flag: explicit value if given, otherwise `true` for `admin`
    /// actions and `false` for non-`admin` actions.
    #[must_use]
    pub fn effective_destructive(&self) -> bool {
        match self.destructive_explicit {
            Some(b) => b,
            None => self.kind_raw == "admin",
        }
    }
}

/// Owned `[[task_cluster]]` row parsed from TOML.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnedTaskCluster {
    pub name: String,
    pub activity: String,
    pub default: bool,
    pub tools: Vec<String>,
}

/// Owned `[[retired_name]]` row parsed from TOML (NJL-04, RL-03).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RetiredName {
    pub name: String,
    pub use_instead: Vec<String>,
    pub reason: String,
}

/// Owned `[[addition]]` row parsed from TOML (NJL-03 / NJLx-51).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdditionRecord {
    pub name: String,
    pub added_by: String,
    pub defined_in: String,
}

/// Owned `[counts]` table parsed from TOML (NJLx-51).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CountsRecord {
    pub total: i64,
    pub clusters: i64,
    pub by_kind: BTreeMap<String, i64>,
}

/// Owned `[[verb]]` entry parsed from TOML.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnedVerbEntry {
    pub name: String,
    pub cluster: String,
    pub kind_raw: String,
    pub tier_raw: String,
    pub capabilities: Vec<String>,
    pub input_schema: String,
    pub output_schema: String,
    pub refusals: Vec<String>,
    pub idempotent: bool,
    pub cost_model: String,
    pub first_milestone_raw: String,
    pub task_clusters: Vec<String>,
    pub examples: Vec<String>,
    pub purpose: String,
    pub use_when: String,
    pub owner: String,
    pub wait: Option<OwnedWaitDefault>,
    pub actions: Vec<OwnedAction>,
    pub schema_version: i64,
    pub paginated: bool,
    pub exports: bool,
    pub estimable: bool,
    /// `Some(b)` when explicitly present in TOML; `None` when omitted.
    pub destructive_explicit: Option<bool>,
    pub public: bool,
    pub key_optional: bool,
    pub option_capabilities: BTreeMap<String, Vec<String>>,
    pub spends_physics: Option<bool>,
    pub released: bool,
    pub params: BTreeMap<String, OwnedParamKind>,
    pub conformance_only: Vec<String>,
}

impl OwnedVerbEntry {
    /// Effective `destructive` boolean emitted to [`VerbEntry`].
    #[must_use]
    pub fn effective_destructive(&self) -> bool {
        self.destructive_explicit.unwrap_or(false)
    }

    /// MCP tool name (`"."` and `"-"` mapped to `"_"`).
    #[must_use]
    pub fn tool_name(&self) -> String {
        tool_name(&self.name)
    }

    /// CLI command path segments.
    #[must_use]
    pub fn cli_path(&self) -> Vec<&str> {
        cli_path(&self.name)
    }

    /// Resolve `action` identically to [`VerbEntry::resolve_action`] for owned entries.
    pub fn resolve_owned_action(
        &self,
        action: Option<&str>,
    ) -> Result<(Kind, Vec<String>, bool, bool), RefusalCode> {
        match action {
            None => {
                let kind = Kind::parse(&self.kind_raw).ok_or(RefusalCode::FrobRef802)?;
                Ok((
                    kind,
                    self.capabilities.clone(),
                    self.paginated,
                    self.effective_destructive(),
                ))
            }
            Some(req) => {
                let found = self
                    .actions
                    .iter()
                    .find(|a| a.name == req)
                    .ok_or(RefusalCode::FrobRef802)?;
                let kind = Kind::parse(&found.kind_raw).ok_or(RefusalCode::FrobRef802)?;
                Ok((
                    kind,
                    found.capabilities.clone(),
                    found.paginated,
                    found.effective_destructive(),
                ))
            }
        }
    }

    /// Serialize this entry back into strict `frob-core::registry::toml` subset syntax.
    #[must_use]
    pub fn to_toml_string(&self) -> String {
        let mut out = String::new();
        out.push_str("[[verb]]\n");
        write_toml_str(&mut out, "name", &self.name);
        write_toml_str(&mut out, "cluster", &self.cluster);
        write_toml_str(&mut out, "kind", &self.kind_raw);
        write_toml_str(&mut out, "tier", &self.tier_raw);
        write_toml_str_array(&mut out, "capabilities", &self.capabilities);
        write_toml_str(&mut out, "input_schema", &self.input_schema);
        write_toml_str(&mut out, "output_schema", &self.output_schema);
        write_toml_str_array(&mut out, "refusals", &self.refusals);
        let _ = writeln!(out, "idempotent = {}", self.idempotent);
        write_toml_str(&mut out, "cost_model", &self.cost_model);
        write_toml_str(&mut out, "first_milestone", &self.first_milestone_raw);
        write_toml_str_array(&mut out, "task_clusters", &self.task_clusters);
        write_toml_str_array(&mut out, "examples", &self.examples);
        write_toml_str(&mut out, "purpose", &self.purpose);
        write_toml_str(&mut out, "use_when", &self.use_when);
        write_toml_str(&mut out, "owner", &self.owner);
        if let Some(wait) = &self.wait {
            let _ = writeln!(
                out,
                "wait = {{ cli = {}, mcp_ms = {}, mcp_max_ms = {} }}",
                toml_quote(&wait.cli),
                wait.mcp_ms,
                wait.mcp_max_ms
            );
        }
        if !self.actions.is_empty() {
            out.push_str("actions = [");
            for (idx, act) in self.actions.iter().enumerate() {
                if idx > 0 {
                    out.push_str(", ");
                }
                let _ = write!(
                    out,
                    "{{ name = {}, kind = {}, capabilities = {}",
                    toml_quote(&act.name),
                    toml_quote(&act.kind_raw),
                    toml_str_array_inline(&act.capabilities)
                );
                if act.paginated {
                    out.push_str(", paginated = true");
                }
                if let Some(dest) = act.destructive_explicit {
                    let _ = write!(out, ", destructive = {dest}");
                }
                out.push_str(" }");
            }
            out.push_str("]\n");
        }
        let _ = writeln!(out, "schema_version = {}", self.schema_version);
        if self.paginated {
            out.push_str("paginated = true\n");
        }
        if self.exports {
            out.push_str("exports = true\n");
        }
        if self.estimable {
            out.push_str("estimable = true\n");
        }
        if let Some(dest) = self.destructive_explicit {
            let _ = writeln!(out, "destructive = {dest}");
        }
        if self.public {
            out.push_str("public = true\n");
        }
        if self.key_optional {
            out.push_str("key_optional = true\n");
        }
        if !self.option_capabilities.is_empty() {
            out.push_str("option_capabilities = { ");
            for (idx, (k, scopes)) in self.option_capabilities.iter().enumerate() {
                if idx > 0 {
                    out.push_str(", ");
                }
                let _ = write!(out, "{} = {}", toml_key(k), toml_str_array_inline(scopes));
            }
            out.push_str(" }\n");
        }
        if let Some(sp) = self.spends_physics {
            let _ = writeln!(out, "spends_physics = {sp}");
        }
        if self.released {
            out.push_str("released = true\n");
        }
        if !self.params.is_empty() {
            out.push_str("params = { ");
            for (idx, (k, pk)) in self.params.iter().enumerate() {
                if idx > 0 {
                    out.push_str(", ");
                }
                let _ = write!(
                    out,
                    "{} = {}",
                    toml_key(k),
                    toml_quote(&pk.to_wire_string())
                );
            }
            out.push_str(" }\n");
        }
        if !self.conformance_only.is_empty() {
            write_toml_str_array(&mut out, "conformance_only", &self.conformance_only);
        }
        out
    }
}

impl Canonical for OwnedVerbEntry {
    fn write(&self, w: CanonValue<'_>) -> Result<(), CanonError> {
        w.object(|o| {
            o.str("name", &self.name)?;
            o.str("cluster", &self.cluster)?;
            o.str("kind", &self.kind_raw)?;
            o.str("tier", &self.tier_raw)?;
            o.array("capabilities", |a| {
                for s in &self.capabilities {
                    a.push_str(s);
                }
                Ok(())
            })?;
            o.str("input_schema", &self.input_schema)?;
            o.str("output_schema", &self.output_schema)?;
            o.array("refusals", |a| {
                for r in &self.refusals {
                    a.push_str(r);
                }
                Ok(())
            })?;
            o.bool("idempotent", self.idempotent)?;
            o.str("cost_model", &self.cost_model)?;
            o.str("first_milestone", &self.first_milestone_raw)?;
            o.array("task_clusters", |a| {
                for tc in &self.task_clusters {
                    a.push_str(tc);
                }
                Ok(())
            })?;
            o.array("examples", |a| {
                for ex in &self.examples {
                    a.push_str(ex);
                }
                Ok(())
            })?;
            o.str("purpose", &self.purpose)?;
            o.str("use_when", &self.use_when)?;
            o.str("owner", &self.owner)?;
            match &self.wait {
                Some(wait) => o.object("wait", |wo| {
                    wo.str("cli", &wait.cli)?;
                    wo.int53("mcp_ms", wait.mcp_ms)?;
                    wo.int53("mcp_max_ms", wait.mcp_max_ms)?;
                    Ok(())
                })?,
                None => o.null("wait")?,
            }
            o.array("actions", |a| {
                let mut sorted_actions: Vec<&OwnedAction> = self.actions.iter().collect();
                sorted_actions.sort_by(|l, r| l.name.cmp(&r.name));
                for act in sorted_actions {
                    a.push_object(|ao| {
                        ao.str("name", &act.name)?;
                        ao.str("kind", &act.kind_raw)?;
                        ao.array("capabilities", |ca| {
                            for s in &act.capabilities {
                                ca.push_str(s);
                            }
                            Ok(())
                        })?;
                        ao.bool("paginated", act.paginated)?;
                        ao.bool("destructive", act.effective_destructive())?;
                        Ok(())
                    })?;
                }
                Ok(())
            })?;
            o.int53("schema_version", self.schema_version)?;
            o.bool("paginated", self.paginated)?;
            o.bool("exports", self.exports)?;
            o.bool("estimable", self.estimable)?;
            o.bool("destructive", self.effective_destructive())?;
            o.bool("public", self.public)?;
            o.bool("key_optional", self.key_optional)?;
            o.object("option_capabilities", |oo| {
                for (opt, scopes) in &self.option_capabilities {
                    oo.array(opt, |sa| {
                        for s in scopes {
                            sa.push_str(s);
                        }
                        Ok(())
                    })?;
                }
                Ok(())
            })?;
            match self.spends_physics {
                Some(b) => o.bool("spends_physics", b)?,
                None => o.null("spends_physics")?,
            }
            o.bool("released", self.released)?;
            o.object("params", |po| {
                for (k, pk) in &self.params {
                    po.str(k, &pk.to_wire_string())?;
                }
                Ok(())
            })?;
            o.array("conformance_only", |a| {
                for name in &self.conformance_only {
                    a.push_str(name);
                }
                Ok(())
            })?;
            Ok(())
        })
    }
}

/// Complete parsed `registries/verbs.toml` document (schema `frob.verbs/1`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedRegistry {
    pub schema: String,
    pub verbs: Vec<OwnedVerbEntry>,
    pub task_clusters: Vec<OwnedTaskCluster>,
    pub retired_names: Vec<RetiredName>,
    pub additions: Vec<AdditionRecord>,
    pub counts: Option<CountsRecord>,
}

/// Strict parse error returned before or alongside linting.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    /// Strict TOML-subset syntax or type error.
    Toml(TomlError),
    /// Unknown key in closed registry/entry schema (`RL-13` `unknown-field`).
    UnknownField {
        entry: String,
        table: String,
        key: String,
        line: u32,
        column: u32,
    },
    /// Schema header is not `frob.verbs/1`.
    InvalidSchema { observed: String },
    /// Malformed `params.<arg>` value.
    InvalidParamKind {
        entry: String,
        param: String,
        value: String,
        reason: &'static str,
    },
}

impl ParseError {
    /// Return the lint identifier if this parse error maps to a named registry lint.
    #[must_use]
    pub const fn lint_id(&self) -> Option<&'static str> {
        match self {
            Self::UnknownField { .. } => Some("RL-13"),
            _ => None,
        }
    }

    /// Return the error slug if this parse error maps to a named registry lint.
    #[must_use]
    pub const fn slug(&self) -> Option<&'static str> {
        match self {
            Self::UnknownField { .. } => Some("unknown-field"),
            _ => None,
        }
    }
}

impl From<TomlError> for ParseError {
    fn from(e: TomlError) -> Self {
        if let TomlErrorKind::UnknownKey { table, key } = e.kind {
            Self::UnknownField {
                entry: table.clone(),
                table,
                key,
                line: e.line,
                column: e.column,
            }
        } else {
            Self::Toml(e)
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Toml(e) => fmt::Display::fmt(e, f),
            Self::UnknownField {
                entry,
                table,
                key,
                line,
                column,
            } => write!(
                f,
                "{line}:{column}: [RL-13 unknown-field] entry `{entry}` (table `{table}`) contains unknown field `{key}`"
            ),
            Self::InvalidSchema { observed } => write!(
                f,
                "invalid verb registry schema `{observed}`, expected `{VERBS_REGISTRY_SCHEMA}`"
            ),
            Self::InvalidParamKind {
                entry,
                param,
                value,
                reason,
            } => write!(
                f,
                "entry `{entry}` params.{param} has invalid kind `{value}`: {reason}"
            ),
        }
    }
}

impl std::error::Error for ParseError {}

impl ParsedRegistry {
    /// Parse `registries/verbs.toml` bytes under the strict closed `frob.verbs/1` schema.
    pub fn parse_toml(bytes: &[u8]) -> Result<Self, ParseError> {
        let root = toml::parse(bytes)?;
        let mut top = Fields::new(root, "<root>");
        let schema = top.require_str("schema")?;
        if schema != VERBS_REGISTRY_SCHEMA {
            return Err(ParseError::InvalidSchema { observed: schema });
        }

        let raw_verbs = top.opt_array("verb")?.unwrap_or_default();
        let raw_clusters = top.opt_array("task_cluster")?.unwrap_or_default();
        let raw_retired = top.opt_array("retired_name")?.unwrap_or_default();
        let raw_additions = top.opt_array("addition")?.unwrap_or_default();
        let raw_counts = top.opt_table("counts")?;
        top.finish()?;

        let mut verbs = Vec::with_capacity(raw_verbs.len());
        for (idx, sp) in raw_verbs.into_iter().enumerate() {
            let tbl = expect_table(sp, "verb", idx)?;
            verbs.push(parse_verb_entry(tbl, idx)?);
        }

        let mut task_clusters = Vec::with_capacity(raw_clusters.len());
        for (idx, sp) in raw_clusters.into_iter().enumerate() {
            let tbl = expect_table(sp, "task_cluster", idx)?;
            task_clusters.push(parse_task_cluster(tbl, idx)?);
        }

        let mut retired_names = Vec::with_capacity(raw_retired.len());
        for (idx, sp) in raw_retired.into_iter().enumerate() {
            let tbl = expect_table(sp, "retired_name", idx)?;
            retired_names.push(parse_retired_name(tbl, idx)?);
        }

        let mut additions = Vec::with_capacity(raw_additions.len());
        for (idx, sp) in raw_additions.into_iter().enumerate() {
            let tbl = expect_table(sp, "addition", idx)?;
            additions.push(parse_addition(tbl, idx)?);
        }

        let counts = raw_counts.map(parse_counts).transpose()?;

        Ok(Self {
            schema,
            verbs,
            task_clusters,
            retired_names,
            additions,
            counts,
        })
    }

    /// Compute the deterministic BLAKE3 `REGISTRY_DIGEST` over the canonical serialization
    /// of all verb entries sorted by `name` (independent of input file order and of whether
    /// `conformance` strips properties from embedded schemas).
    pub fn registry_digest(&self) -> Result<[u8; 32], CanonError> {
        let mut sorted: Vec<&OwnedVerbEntry> = self.verbs.iter().collect();
        sorted.sort_by(|a, b| a.name.as_bytes().cmp(b.name.as_bytes()));

        let mut writer = CanonWriter::new();
        writer.value()?.object(|o| {
            o.str("schema", &self.schema)?;
            o.array("verbs", |a| {
                for entry in &sorted {
                    a.push_term(*entry)?;
                }
                Ok(())
            })?;
            Ok(())
        })?;
        let canonical = writer.finish()?;
        let schema_name = SchemaName::parse(VERBS_REGISTRY_SCHEMA)
            .expect("VERBS_REGISTRY_SCHEMA is a valid SchemaName");
        Ok(*identity(&schema_name, &canonical).as_bytes())
    }
}

fn expect_table(sp: Spanned<Value>, key: &str, idx: usize) -> Result<Table, ParseError> {
    match sp.value {
        Value::Table(tbl) => Ok(tbl),
        other => Err(ParseError::Toml(TomlError::new(
            sp.line,
            sp.column,
            TomlErrorKind::TypeMismatch {
                key: format!("{key}[{idx}]"),
                expected: "table",
                found: other.type_name(),
            },
        ))),
    }
}

fn finish_for_entry(fields: Fields, entry_name: &str) -> Result<(), ParseError> {
    fields.finish().map_err(|e| {
        if let TomlErrorKind::UnknownKey { table, key } = e.kind {
            ParseError::UnknownField {
                entry: entry_name.to_owned(),
                table,
                key,
                line: e.line,
                column: e.column,
            }
        } else {
            ParseError::Toml(e)
        }
    })
}

fn parse_verb_entry(tbl: Table, idx: usize) -> Result<OwnedVerbEntry, ParseError> {
    let label = tbl
        .get("name")
        .and_then(Value::as_str)
        .map_or_else(|| format!("verb[{idx}]"), str::to_owned);
    let mut f = Fields::new(tbl, format!("verb[{idx}]"));

    let name = f.require_str("name")?;
    let cluster = f.require_str("cluster")?;
    let kind_raw = f.require_str("kind")?;
    let tier_raw = f.require_str("tier")?;
    let capabilities = extract_str_vec("capabilities", f.require_array("capabilities")?)?;
    let input_schema = f.require_str("input_schema")?;
    let output_schema = f.require_str("output_schema")?;
    let refusals = extract_str_vec("refusals", f.require_array("refusals")?)?;
    let idempotent = f.require_bool("idempotent")?;
    let cost_model = f.require_str("cost_model")?;
    let first_milestone_raw = f.require_str("first_milestone")?;
    let task_clusters = extract_str_vec("task_clusters", f.require_array("task_clusters")?)?;
    let examples = extract_str_vec("examples", f.require_array("examples")?)?;
    let purpose = f.require_str("purpose")?;
    let use_when = f.require_str("use_when")?;
    let owner = f.require_str("owner")?;

    let wait = f
        .opt_table("wait")?
        .map(|wt| parse_wait(wt, &label))
        .transpose()?;

    let raw_actions = f.opt_array("actions")?.unwrap_or_default();
    let mut actions = Vec::with_capacity(raw_actions.len());
    for (aidx, asp) in raw_actions.into_iter().enumerate() {
        let atbl = expect_table(asp, "actions", aidx)?;
        actions.push(parse_action(atbl, &label, aidx)?);
    }

    let schema_version = f.opt_int("schema_version")?.unwrap_or(1);
    let paginated = f.opt_bool("paginated")?.unwrap_or(false);
    let exports = f.opt_bool("exports")?.unwrap_or(false);
    let estimable = f.opt_bool("estimable")?.unwrap_or(false);
    let destructive_explicit = f.opt_bool("destructive")?;
    let public = f.opt_bool("public")?.unwrap_or(false);
    let key_optional = f.opt_bool("key_optional")?.unwrap_or(false);

    let mut option_capabilities = BTreeMap::new();
    if let Some(opt_tbl) = f.opt_table("option_capabilities")? {
        for (opt_name, sp) in opt_tbl.into_inner() {
            let Value::Array(items) = sp.value else {
                return Err(ParseError::Toml(TomlError::new(
                    sp.line,
                    sp.column,
                    TomlErrorKind::TypeMismatch {
                        key: format!("option_capabilities.{opt_name}"),
                        expected: "array",
                        found: sp.value.type_name(),
                    },
                )));
            };
            let scopes = extract_str_vec(&format!("option_capabilities.{opt_name}"), items)?;
            option_capabilities.insert(opt_name, scopes);
        }
    }

    let spends_physics = f.opt_bool("spends_physics")?;
    let released = f.opt_bool("released")?.unwrap_or(false);

    let mut params = BTreeMap::new();
    if let Some(params_tbl) = f.opt_table("params")? {
        for (param_name, sp) in params_tbl.into_inner() {
            let Value::Str(kind_s) = sp.value else {
                return Err(ParseError::Toml(TomlError::new(
                    sp.line,
                    sp.column,
                    TomlErrorKind::TypeMismatch {
                        key: format!("params.{param_name}"),
                        expected: "string",
                        found: sp.value.type_name(),
                    },
                )));
            };
            let pk =
                OwnedParamKind::parse(&kind_s).map_err(|reason| ParseError::InvalidParamKind {
                    entry: label.clone(),
                    param: param_name.clone(),
                    value: kind_s,
                    reason,
                })?;
            params.insert(param_name, pk);
        }
    }

    let conformance_only = f
        .opt_array("conformance_only")?
        .map(|items| extract_str_vec("conformance_only", items))
        .transpose()?
        .unwrap_or_default();

    finish_for_entry(f, &label)?;

    Ok(OwnedVerbEntry {
        name,
        cluster,
        kind_raw,
        tier_raw,
        capabilities,
        input_schema,
        output_schema,
        refusals,
        idempotent,
        cost_model,
        first_milestone_raw,
        task_clusters,
        examples,
        purpose,
        use_when,
        owner,
        wait,
        actions,
        schema_version,
        paginated,
        exports,
        estimable,
        destructive_explicit,
        public,
        key_optional,
        option_capabilities,
        spends_physics,
        released,
        params,
        conformance_only,
    })
}

fn parse_wait(tbl: Table, entry: &str) -> Result<OwnedWaitDefault, ParseError> {
    let mut f = Fields::new(tbl, format!("{entry}.wait"));
    let cli = f.require_str("cli")?;
    let mcp_ms = f.require_int("mcp_ms")?;
    let mcp_max_ms = f.require_int("mcp_max_ms")?;
    finish_for_entry(f, entry)?;
    Ok(OwnedWaitDefault {
        cli,
        mcp_ms,
        mcp_max_ms,
    })
}

fn parse_action(tbl: Table, entry: &str, idx: usize) -> Result<OwnedAction, ParseError> {
    let mut f = Fields::new(tbl, format!("{entry}.actions[{idx}]"));
    let name = f.require_str("name")?;
    let kind_raw = f.require_str("kind")?;
    let capabilities = extract_str_vec("capabilities", f.require_array("capabilities")?)?;
    let paginated = f.opt_bool("paginated")?.unwrap_or(false);
    let destructive_explicit = f.opt_bool("destructive")?;
    finish_for_entry(f, entry)?;
    Ok(OwnedAction {
        name,
        kind_raw,
        capabilities,
        paginated,
        destructive_explicit,
    })
}

fn parse_task_cluster(tbl: Table, idx: usize) -> Result<OwnedTaskCluster, ParseError> {
    let label = format!("task_cluster[{idx}]");
    let mut f = Fields::new(tbl, label.clone());
    let name = f.require_str("name")?;
    let activity = f.require_str("activity")?;
    let default = f.opt_bool("default")?.unwrap_or(false);
    let tools = f
        .opt_array("tools")?
        .map(|items| extract_str_vec("tools", items))
        .transpose()?
        .unwrap_or_default();
    finish_for_entry(f, &label)?;
    Ok(OwnedTaskCluster {
        name,
        activity,
        default,
        tools,
    })
}

fn parse_retired_name(tbl: Table, idx: usize) -> Result<RetiredName, ParseError> {
    let label = format!("retired_name[{idx}]");
    let mut f = Fields::new(tbl, label.clone());
    let name = f.require_str("name")?;
    let use_instead = extract_str_vec("use", f.require_array("use")?)?;
    let reason = f.require_str("reason")?;
    finish_for_entry(f, &label)?;
    Ok(RetiredName {
        name,
        use_instead,
        reason,
    })
}

fn parse_addition(tbl: Table, idx: usize) -> Result<AdditionRecord, ParseError> {
    let label = format!("addition[{idx}]");
    let mut f = Fields::new(tbl, label.clone());
    let name = f.require_str("name")?;
    let added_by = f.require_str("added_by")?;
    let defined_in = f.require_str("defined_in")?;
    finish_for_entry(f, &label)?;
    Ok(AdditionRecord {
        name,
        added_by,
        defined_in,
    })
}

fn parse_counts(tbl: Table) -> Result<CountsRecord, ParseError> {
    let mut f = Fields::new(tbl, "counts");
    let total = f.require_int("total")?;
    let clusters = f.require_int("clusters")?;
    let mut by_kind = BTreeMap::new();
    if let Some(bk) = f.opt_table("by_kind")? {
        for (k, sp) in bk.into_inner() {
            let Value::Int(n) = sp.value else {
                return Err(ParseError::Toml(TomlError::new(
                    sp.line,
                    sp.column,
                    TomlErrorKind::TypeMismatch {
                        key: format!("counts.by_kind.{k}"),
                        expected: "integer",
                        found: sp.value.type_name(),
                    },
                )));
            };
            by_kind.insert(k, n);
        }
    }
    finish_for_entry(f, "counts")?;
    Ok(CountsRecord {
        total,
        clusters,
        by_kind,
    })
}

fn extract_str_vec(key: &str, items: Vec<Spanned<Value>>) -> Result<Vec<String>, ParseError> {
    items
        .into_iter()
        .map(|sp| match sp.value {
            Value::Str(s) => Ok(s),
            other => Err(ParseError::Toml(TomlError::new(
                sp.line,
                sp.column,
                TomlErrorKind::TypeMismatch {
                    key: key.to_owned(),
                    expected: "string",
                    found: other.type_name(),
                },
            ))),
        })
        .collect()
}

fn toml_quote(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for ch in s.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => {
                let _ = write!(out, "\\u{:04x}", c as u32);
            }
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

fn toml_key(k: &str) -> String {
    if !k.is_empty()
        && k.bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-')
    {
        k.to_owned()
    } else {
        toml_quote(k)
    }
}

fn toml_str_array_inline(items: &[String]) -> String {
    let mut out = String::from("[");
    for (idx, item) in items.iter().enumerate() {
        if idx > 0 {
            out.push_str(", ");
        }
        out.push_str(&toml_quote(item));
    }
    out.push(']');
    out
}

fn write_toml_str(out: &mut String, key: &str, val: &str) {
    let _ = writeln!(out, "{key} = {}", toml_quote(val));
}

fn write_toml_str_array(out: &mut String, key: &str, items: &[String]) {
    let _ = writeln!(out, "{key} = {}", toml_str_array_inline(items));
}
