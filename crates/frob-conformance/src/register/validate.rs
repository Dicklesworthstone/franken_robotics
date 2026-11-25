#![deny(unsafe_code)]

//! Gate-register (`frob.gates/1`) full register types, loader, nine-rule validator,
//! and ten write-time refusals (`N40-01`, `ADR-GAUNT-04`, `N38-REF`).

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

use frob_core::graph_id::ClaimId;
use frob_core::id::{Blake3, hash_blob};
use frob_core::json::{self, Json, Limits};
use frob_core::registry::toml::{self, Fields, Spanned, Table, Value};
use frob_core::testkit::registry::Controls;

use crate::reasons::HarnessReason;
use crate::register::kill::{
    self, Amendment, KillDecision, KillMode, KillValidationContext, ScopeKind,
};
use crate::register::row::{
    BaseKind, Claim, GATES_SCHEMA, KILL_FORBIDDEN_FIELDS, Recorded, Status, parse_milestone_ord,
};

/// Gate row kind (`N40-01`); identical to [`BaseKind`] from `N38b-18`.
pub type Kind = BaseKind;

/// Authoritative four recurring gates among compiled-table rows (`N40-01` rule 8, `N40-02`).
pub const COMPILED_RECURRING_GATES: &[&str] = &[
    "GATE-GROUND-01",
    "GATE-SAFE-01",
    "GATE-GAUNT-01",
    "GATE-TENFOLD-09",
];

/// Authoritative five recurring gates among late-subsection rows (`N40-01` rule 8, `N40-10`).
pub const LATE_RECURRING_GATES: &[&str] = &[
    "GATE-GAUNT-02",
    "GATE-GAUNT-03",
    "GATE-GAUNT-05",
    "GATE-GAUNT-10",
    "GATE-WS-08",
];

/// Known additive top-level table arrays admitted on `registries/gates.toml` (`N40-01` Scope).
pub const ADDITIVE_TOP_TABLES: &[&str] = &[
    "amendment",
    "shared_fixture",
    "related_threshold",
    "fallback_exit",
    "variant",
];

/// Measurement environment requirement (`N40-01`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum Needs {
    #[default]
    Simulator,
    ImportLane,
}

impl Needs {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Simulator => "simulator",
            Self::ImportLane => "import-lane",
        }
    }

    pub fn parse(raw: &str) -> Option<Self> {
        match raw {
            "simulator" => Some(Self::Simulator),
            "import-lane" => Some(Self::ImportLane),
            _ => None,
        }
    }
}

/// Full `[[gate.part]]` record combining `N38b-18` base fields and `N40-01` provenance fields.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Part {
    pub milestone: String,
    pub scope: Option<String>,
    pub status: Status,
    pub measured: String,
    pub commit: String,
    pub command: String,
    pub receipt: String,
    pub fingerprint: String,
    pub seed: String,
    pub evidence: Vec<String>,
    pub recorded: Recorded,
    pub detail: Option<String>,
    pub kill_gate: Option<String>,
    pub fallback: Option<String>,
    pub open_question: Option<String>,
}

impl Part {
    /// Reconciled human-readable printed status string (`ADR-GAUNT-04`).
    #[must_use]
    pub fn printed_status(&self) -> String {
        let ctx = match self.status {
            Status::Killed => self.fallback.as_deref().or(self.detail.as_deref()),
            Status::Blocked => self.open_question.as_deref().or(self.detail.as_deref()),
            Status::Superseded => self.detail.as_deref(),
            _ => None,
        };
        self.status.printed_form(ctx)
    }

    /// If `scope` begins with `"pilot:"`, return the pilot ID slice after `"pilot:"` trimmed.
    #[must_use]
    pub fn pilot_scope_id(&self) -> Option<&str> {
        let s = self.scope.as_deref()?;
        s.strip_prefix("pilot:").map(str::trim)
    }
}

/// Optional admission-row metadata (`N40-01` step 1).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AdmissionFields {
    pub foundation: Option<String>,
    pub first_needed: Option<String>,
}

/// Kill-row metadata (`N40-01` step 1, `N40-03`).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct KillFields {
    pub trigger: Option<String>,
    pub fallback: Option<String>,
    pub decided: Option<String>,
    pub mode: KillMode,
    /// True iff `mode` was explicitly set in TOML (used by `--live --ids` completeness checks).
    pub mode_explicit: bool,
    pub reads: Vec<String>,
    pub decisions: Vec<KillDecision>,
    /// True iff `decision` was encoded as an array of tables rather than a single table.
    pub decision_as_array: bool,
    pub reread: Vec<String>,
    pub scope_kind: Option<ScopeKind>,
    pub milestone: Option<String>,
}

/// Full `[[gate]]` row combining `N38b-18` base fields and `N40-01` register-layer fields.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateRow {
    pub id: String,
    pub kind: Option<Kind>,
    pub section: String,
    pub consumers: Vec<String>,
    /// Optional source-register citations (`SRC-…`, optional `#figure`; Review Note #5 / `NAB-10`).
    pub sources: Vec<String>,
    pub recurring: bool,
    pub needs: Needs,
    pub blocks_exit: bool,
    // Measured-row fields (strictly `None`/empty on `kind = "kill"` rows):
    pub claim: Option<Claim>,
    pub claim_id: Option<String>,
    pub threshold: Option<String>,
    pub milestone: Option<String>,
    pub tier: Vec<String>,
    pub controls: Vec<String>,
    pub parts: Vec<Part>,
    // Kind-specific fields:
    pub admission: AdmissionFields,
    pub kill: KillFields,
}

impl GateRow {
    /// Explicit kind if specified, otherwise inferred from identifier prefix.
    #[must_use]
    pub fn effective_kind(&self) -> Kind {
        self.kind.unwrap_or_else(|| Kind::infer_from_id(&self.id))
    }

    #[must_use]
    pub fn is_kill(&self) -> bool {
        self.kind == Some(Kind::Kill)
    }

    #[must_use]
    pub fn is_import_lane(&self) -> bool {
        self.needs == Needs::ImportLane
    }

    /// Derived status across all recorded parts (`NotRun` if none recorded).
    #[must_use]
    pub fn derived_status(&self) -> Status {
        self.latest_part().map_or(Status::NotRun, |p| p.status)
    }

    /// Derived status at milestone `due_milestone` (`NotRun` if no part due yet).
    #[must_use]
    pub fn derived_status_at(&self, due_milestone: &str) -> Status {
        self.latest_due_part(due_milestone)
            .map_or(Status::NotRun, |p| p.status)
    }

    /// Reconciled printed status (`"not run"` when no part is recorded or due).
    #[must_use]
    pub fn printed_status(&self, due_milestone: Option<&str>) -> String {
        let part = match due_milestone {
            Some(m) => self.latest_due_part(m),
            None => self.latest_part(),
        };
        part.map_or_else(|| Status::NotRun.printed_form(None), Part::printed_status)
    }

    /// True iff this gate enters the due set of milestone exit `exit_milestone` (`N40-01`, `N40-04`).
    ///
    /// Import-lane gates (`needs = "import-lane"`) and non-blocking sibling admission pilots
    /// (`blocks_exit = false`) are NEVER in any exit's due set.
    #[must_use]
    pub fn is_due_at(&self, exit_milestone: &str) -> bool {
        if self.is_import_lane() || !self.blocks_exit || self.is_kill() {
            return false;
        }
        let Some(exit_ord) = parse_milestone_ord(exit_milestone) else {
            return false;
        };
        if self.recurring
            && let Some(first_m) = self.milestone.as_deref()
            && let Some(first_ord) = parse_milestone_ord(first_m)
            && exit_ord >= first_ord
        {
            return true;
        }
        if self.parts.is_empty() {
            return self.milestone.as_deref() == Some(exit_milestone);
        }
        self.parts.iter().any(|p| p.milestone == exit_milestone)
    }

    fn latest_part(&self) -> Option<&Part> {
        self.parts
            .iter()
            .enumerate()
            .max_by_key(|(idx, p)| (parse_milestone_ord(&p.milestone).unwrap_or(0), *idx))
            .map(|(_, p)| p)
    }

    fn latest_due_part(&self, due_milestone: &str) -> Option<&Part> {
        let due_ord = parse_milestone_ord(due_milestone)?;
        self.parts
            .iter()
            .enumerate()
            .filter_map(|(idx, p)| {
                let ord = parse_milestone_ord(&p.milestone)?;
                if ord <= due_ord {
                    Some((ord, idx, p))
                } else {
                    None
                }
            })
            .max_by_key(|(ord, idx, _)| (*ord, *idx))
            .map(|(_, _, p)| p)
    }
}

/// Complete loaded `registries/gates.toml` register (`frob.gates/1`, `N40-01`, `N40-03`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Register {
    pub schema: String,
    pub retired_ids: Vec<String>,
    pub gates: Vec<GateRow>,
    /// Append-only dated amendments (`[[amendment]]`, `N40-03` step 4).
    pub amendments: Vec<Amendment>,
    /// True if any row explicitly carried register-layer fields (`kind` / `recurring`).
    pub has_register_layer_fields: bool,
}

impl Register {
    #[must_use]
    pub fn get(&self, id: &str) -> Option<&GateRow> {
        self.gates.iter().find(|g| g.id == id)
    }

    #[must_use]
    pub fn get_mut(&mut self, id: &str) -> Option<&mut GateRow> {
        self.gates.iter_mut().find(|g| g.id == id)
    }

    /// Return every gate in the due set of `exit_milestone` (excluding `import-lane` and
    /// `blocks_exit = false`).
    #[must_use]
    pub fn due_gates(&self, exit_milestone: &str) -> Vec<&GateRow> {
        self.gates
            .iter()
            .filter(|g| g.is_due_at(exit_milestone))
            .collect()
    }

    /// Count measured gates by derived status.
    #[must_use]
    pub fn counts_by_status(&self) -> BTreeMap<Status, usize> {
        let mut out = BTreeMap::new();
        for &s in Status::ALL {
            out.insert(s, 0);
        }
        for g in &self.gates {
            if !g.is_kill() {
                *out.entry(g.derived_status()).or_insert(0) += 1;
            }
        }
        out
    }

    /// Deterministic D0 TOML printer preserving file order (`N40-01` step 5).
    #[must_use]
    pub fn print_toml(&self) -> String {
        let mut out = String::new();
        out.push_str("schema = ");
        push_str(&mut out, &self.schema);
        out.push('\n');

        if !self.retired_ids.is_empty() {
            out.push_str("retired_ids = ");
            push_str_array(&mut out, &self.retired_ids);
            out.push('\n');
        }

        for g in &self.gates {
            out.push_str("\n[[gate]]\n");
            push_kv_str(&mut out, "id", &g.id);
            if let Some(kind) = g.kind {
                push_kv_str(&mut out, "kind", kind.as_str());
            }
            if let Some(claim) = g.claim {
                push_kv_str(&mut out, "claim", claim.as_str());
            }
            if let Some(ref cid) = g.claim_id {
                push_kv_str(&mut out, "claim_id", cid);
            }
            push_kv_str(&mut out, "section", &g.section);
            if !g.consumers.is_empty() {
                out.push_str("consumers = ");
                push_str_array(&mut out, &g.consumers);
                out.push('\n');
            }
            if !g.sources.is_empty() {
                out.push_str("sources = ");
                push_str_array(&mut out, &g.sources);
                out.push('\n');
            }
            if let Some(ref thr) = g.threshold {
                push_kv_str(&mut out, "threshold", thr);
            }
            if let Some(ref ms) = g.milestone {
                push_kv_str(&mut out, "milestone", ms);
            }
            if g.recurring {
                out.push_str("recurring = true\n");
            }
            if g.needs != Needs::Simulator {
                push_kv_str(&mut out, "needs", g.needs.as_str());
            }
            if !g.blocks_exit {
                out.push_str("blocks_exit = false\n");
            }
            if let Some(ref fd) = g.admission.foundation {
                push_kv_str(&mut out, "foundation", fd);
            }
            if let Some(ref fn_m) = g.admission.first_needed {
                push_kv_str(&mut out, "first_needed", fn_m);
            }
            if g.is_kill() {
                if let Some(ref t) = g.kill.trigger {
                    push_kv_str(&mut out, "trigger", t);
                }
                if let Some(ref fb) = g.kill.fallback {
                    push_kv_str(&mut out, "fallback", fb);
                }
                if let Some(ref d) = g.kill.decided {
                    push_kv_str(&mut out, "decided", d);
                }
                if g.kill.mode_explicit || g.kill.mode != KillMode::Exit {
                    push_kv_str(&mut out, "mode", g.kill.mode.as_str());
                }
                if !g.kill.reads.is_empty() {
                    out.push_str("reads = ");
                    push_str_array(&mut out, &g.kill.reads);
                    out.push('\n');
                }
                if !g.kill.reread.is_empty() {
                    out.push_str("reread = ");
                    push_str_array(&mut out, &g.kill.reread);
                    out.push('\n');
                }
                if let Some(sk) = g.kill.scope_kind {
                    push_kv_str(&mut out, "scope_kind", sk.as_str());
                }
                if let Some(ref ms) = g.kill.milestone {
                    push_kv_str(&mut out, "milestone", ms);
                }
                if !g.kill.decisions.is_empty() {
                    if !g.kill.decision_as_array && g.kill.decisions.len() == 1 {
                        out.push_str("decision = ");
                        kill::push_decision_inline(&mut out, &g.kill.decisions[0]);
                        out.push('\n');
                    } else {
                        out.push_str("decision = [\n");
                        for d in &g.kill.decisions {
                            out.push_str("  ");
                            kill::push_decision_inline(&mut out, d);
                            out.push_str(",\n");
                        }
                        out.push_str("]\n");
                    }
                }
            } else {
                out.push_str("tier = ");
                push_str_array(&mut out, &g.tier);
                out.push('\n');
                out.push_str("controls = ");
                push_str_array(&mut out, &g.controls);
                out.push('\n');
            }

            for p in &g.parts {
                out.push_str("\n[[gate.part]]\n");
                push_kv_str(&mut out, "milestone", &p.milestone);
                if let Some(ref sc) = p.scope {
                    push_kv_str(&mut out, "scope", sc);
                }
                push_kv_str(&mut out, "status", p.status.as_str());
                push_kv_str(&mut out, "measured", &p.measured);
                if !p.commit.is_empty() {
                    push_kv_str(&mut out, "commit", &p.commit);
                }
                if !p.command.is_empty() {
                    push_kv_str(&mut out, "command", &p.command);
                }
                if !p.receipt.is_empty() {
                    push_kv_str(&mut out, "receipt", &p.receipt);
                }
                if !p.fingerprint.is_empty() {
                    push_kv_str(&mut out, "fingerprint", &p.fingerprint);
                }
                if !p.seed.is_empty() {
                    push_kv_str(&mut out, "seed", &p.seed);
                }
                out.push_str("evidence = ");
                push_str_array(&mut out, &p.evidence);
                out.push('\n');
                out.push_str("recorded = { date = ");
                push_str(&mut out, &p.recorded.date);
                out.push_str(", session = ");
                push_str(&mut out, &p.recorded.session);
                out.push_str(", implemented_by = ");
                push_str(&mut out, &p.recorded.implemented_by);
                out.push_str(" }\n");
                if let Some(ref kg) = p.kill_gate {
                    push_kv_str(&mut out, "kill_gate", kg);
                }
                if let Some(ref fb) = p.fallback {
                    push_kv_str(&mut out, "fallback", fb);
                }
                if let Some(ref oq) = p.open_question {
                    push_kv_str(&mut out, "open_question", oq);
                }
                if let Some(ref d) = p.detail {
                    push_kv_str(&mut out, "detail", d);
                }
            }
        }
        for a in &self.amendments {
            kill::push_amendment_toml(&mut out, a);
        }
        out
    }
}

/// One typed validation or write-time refusal violation (`N40-01`, `N38-REF`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Violation {
    pub reason: &'static str,
    pub harness_reason: Option<HarnessReason>,
    pub gate_id: Option<String>,
    pub field: Option<String>,
    pub message: String,
    pub repair: &'static str,
}

impl Violation {
    pub(crate) fn from_harness(
        hr: HarnessReason,
        gate_id: Option<String>,
        field: Option<&str>,
        message: impl Into<String>,
        repair: &'static str,
    ) -> Self {
        Self {
            reason: hr.as_str(),
            harness_reason: Some(hr),
            gate_id,
            field: field.map(str::to_owned),
            message: message.into(),
            repair,
        }
    }

    pub(crate) fn schema(
        reason: &'static str,
        gate_id: Option<String>,
        field: Option<&str>,
        message: impl Into<String>,
        repair: &'static str,
    ) -> Self {
        Self {
            reason,
            harness_reason: None,
            gate_id,
            field: field.map(str::to_owned),
            message: message.into(),
            repair,
        }
    }
}

impl fmt::Display for Violation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (&self.gate_id, &self.field) {
            (Some(id), Some(fld)) => write!(
                f,
                "reason={} gate={} field={}: {} (repair: {})",
                self.reason, id, fld, self.message, self.repair
            ),
            (Some(id), None) => write!(
                f,
                "reason={} gate={}: {} (repair: {})",
                self.reason, id, self.message, self.repair
            ),
            (None, Some(fld)) => write!(
                f,
                "reason={} field={}: {} (repair: {})",
                self.reason, fld, self.message, self.repair
            ),
            (None, None) => write!(
                f,
                "reason={}: {} (repair: {})",
                self.reason, self.message, self.repair
            ),
        }
    }
}

impl std::error::Error for Violation {}

/// Context for cross-file resolution checks (`receipts/gates/`, `controls.toml`, live
/// artifact-graph IDs, retired/previous gate IDs, and kill-gate amendment/decision checks).
#[derive(Debug, Clone, Default)]
pub struct ValidationContext {
    pub repo_root: Option<PathBuf>,
    pub controls: Option<Controls>,
    pub live_claims: Option<BTreeSet<String>>,
    pub live_evidence: Option<BTreeSet<String>>,
    pub retired_ids: BTreeSet<String>,
    pub previous_ids: Option<BTreeSet<String>>,
    /// When `true` (or when register rows carry register-layer fields), enforce missing
    /// `recurring = true` on all nine recurring gate IDs (rule 8).
    pub enforce_recurring_flags: bool,
    /// Kill-gate validation context (`N40-03`: amendment seeds, receipt timestamps,
    /// implementer sessions, prior decisions for RC-57, and ADR file existence check).
    pub kill: KillValidationContext,
}

/// Parse and validate `registries/gates.toml` from disk (`N40-01` step 5).
pub fn load(path: &Path) -> Result<Register, Vec<Violation>> {
    let ctx = ValidationContext {
        repo_root: path.parent().and_then(Path::parent).map(Path::to_path_buf),
        ..ValidationContext::default()
    };
    load_with_context(path, &ctx)
}

/// Parse and validate a `frob.gates/1` file from disk under `ctx`.
pub fn load_with_context(path: &Path, ctx: &ValidationContext) -> Result<Register, Vec<Violation>> {
    let bytes = fs::read(path).map_err(|e| {
        vec![Violation::schema(
            "io-error",
            None,
            None,
            format!("failed reading {}: {e}", path.display()),
            "check the register file path",
        )]
    })?;
    parse_and_validate_bytes(&bytes, &path.to_string_lossy(), ctx)
}

/// Parse and validate raw TOML bytes of `frob.gates/1` under `ctx`.
pub fn parse_and_validate_bytes(
    bytes: &[u8],
    file: &str,
    ctx: &ValidationContext,
) -> Result<Register, Vec<Violation>> {
    let reg = parse_register_bytes(bytes, file)?;
    let violations = validate(&reg, ctx);
    if violations.is_empty() {
        Ok(reg)
    } else {
        Err(violations)
    }
}

/// Parse `frob.gates/1` TOML bytes into [`Register`], enforcing structural schema rules
/// (closed claims, closed statuses, kill-row field bans, unknown keys, duplicate IDs).
pub fn parse_register_bytes(bytes: &[u8], file: &str) -> Result<Register, Vec<Violation>> {
    let root = toml::parse(bytes).map_err(|e| {
        vec![Violation::schema(
            "toml-parse-error",
            None,
            None,
            format!("{file}: {e}"),
            "fix TOML syntax using the strict frob-core TOML subset",
        )]
    })?;

    let mut top = Fields::new(root, "gates");
    let schema = top.require_str("schema").map_err(|e| {
        vec![Violation::schema(
            "schema-invalid",
            None,
            Some("schema"),
            format!("{file}: {e}"),
            "set schema = \"frob.gates/1\"",
        )]
    })?;
    if schema != GATES_SCHEMA {
        return Err(top.fail(vec![Violation::schema(
            "schema-invalid",
            None,
            Some("schema"),
            format!("expected `{GATES_SCHEMA}`, got `{schema}`"),
            "set schema = \"frob.gates/1\"",
        )]));
    }

    let retired_ids = match top.opt_array("retired_ids") {
        Ok(Some(arr)) => parse_str_vec(arr, None, "retired_ids").map_err(|e| top.fail(e))?,
        Ok(None) => Vec::new(),
        Err(e) => {
            return Err(vec![Violation::schema(
                "schema-invalid",
                None,
                Some("retired_ids"),
                e.to_string(),
                "encode retired_ids as an array of strings",
            )]);
        }
    };

    let raw_gates = match top.opt_array("gate") {
        Ok(Some(arr)) => arr,
        Ok(None) => Vec::new(),
        Err(e) => {
            return Err(vec![Violation::schema(
                "schema-invalid",
                None,
                Some("gate"),
                e.to_string(),
                "encode [[gate]] entries as an array of tables",
            )]);
        }
    };

    let raw_amendments = match top.opt_array("amendment") {
        Ok(Some(arr)) => arr,
        Ok(None) => Vec::new(),
        Err(e) => {
            return Err(vec![Violation::schema(
                "schema-invalid",
                None,
                Some("amendment"),
                e.to_string(),
                "encode [[amendment]] entries as an array of tables",
            )]);
        }
    };

    // Admit remaining additive top-level table arrays defined by sibling N40 beads; any other
    // unknown top-level table or field fails `top.finish()`.
    for &table_name in ADDITIVE_TOP_TABLES {
        if table_name == "amendment" {
            continue;
        }
        if let Some(sp) = top.take(table_name)
            && !matches!(sp.value, Value::Array(_))
        {
            return Err(top.fail(vec![Violation::schema(
                "schema-invalid",
                None,
                Some(table_name),
                format!("top-level `{table_name}` must be an array of tables"),
                "use [[table]] array-of-tables syntax",
            )]));
        }
    }

    if let Err(e) = top.finish() {
        return Err(vec![Violation::schema(
            "unknown-field",
            None,
            None,
            format!("{file}: {e}"),
            "remove unknown top-level table or field from gates.toml",
        )]);
    }

    let mut violations = Vec::new();
    let mut gates = Vec::with_capacity(raw_gates.len());
    let mut seen_ids = BTreeSet::new();
    let mut has_register_layer_fields = false;

    for (idx, sp) in raw_gates.into_iter().enumerate() {
        let tbl = match sp.value {
            Value::Table(t) => t,
            other => {
                violations.push(Violation::schema(
                    "schema-invalid",
                    Some(format!("gate[{idx}]")),
                    None,
                    format!("expected table, got {}", other.type_name()),
                    "encode each [[gate]] entry as a TOML table",
                ));
                continue;
            }
        };
        match parse_gate_row(tbl, idx, file) {
            Ok((row, row_has_reg)) => {
                if row_has_reg {
                    has_register_layer_fields = true;
                }
                if !seen_ids.insert(row.id.clone()) {
                    violations.push(Violation::from_harness(
                        HarnessReason::DuplicateId,
                        Some(row.id.clone()),
                        Some("id"),
                        format!("duplicate gate identifier `{}`", row.id),
                        "Restore the original gate identifier or append a new unique identifier",
                    ));
                }
                gates.push(row);
            }
            Err(mut row_errs) => violations.append(&mut row_errs),
        }
    }

    let mut amendments = Vec::with_capacity(raw_amendments.len());
    for (idx, sp) in raw_amendments.into_iter().enumerate() {
        let tbl = match sp.value {
            Value::Table(t) => t,
            other => {
                violations.push(Violation::schema(
                    "schema-invalid",
                    Some(format!("amendment[{idx}]")),
                    None,
                    format!("expected table, got {}", other.type_name()),
                    "encode each [[amendment]] entry as a TOML table",
                ));
                continue;
            }
        };
        match kill::parse_amendment_table(tbl, idx) {
            Ok(amd) => amendments.push(amd),
            Err(mut errs) => violations.append(&mut errs),
        }
    }

    if !violations.is_empty() {
        return Err(violations);
    }

    Ok(Register {
        schema,
        retired_ids,
        gates,
        amendments,
        has_register_layer_fields,
    })
}

/// Run all nine register rules, write-time refusal checks, and kill-gate amendment/decision
/// checks on `register` under `ctx` (`N40-01`, `N40-03`).
#[must_use]
pub fn validate(register: &Register, ctx: &ValidationContext) -> Vec<Violation> {
    let mut violations = Vec::new();

    // 1. Append-only ID uniqueness and retirement checks (`duplicate-id` / `reused-id`).
    let mut seen = BTreeSet::new();
    let mut all_retired: BTreeSet<&str> = register.retired_ids.iter().map(String::as_str).collect();
    for r in &ctx.retired_ids {
        all_retired.insert(r.as_str());
    }
    let all_gate_ids: BTreeSet<&str> = register.gates.iter().map(|g| g.id.as_str()).collect();

    for g in &register.gates {
        if !seen.insert(g.id.as_str()) {
            violations.push(Violation::from_harness(
                HarnessReason::DuplicateId,
                Some(g.id.clone()),
                Some("id"),
                format!("duplicate gate identifier `{}`", g.id),
                "Identifiers are append-only",
            ));
        }
        if all_retired.contains(g.id.as_str()) {
            violations.push(Violation::from_harness(
                HarnessReason::ReusedId,
                Some(g.id.clone()),
                Some("id"),
                format!("retired gate identifier `{}` was reused", g.id),
                "Identifiers are append-only; never reuse a retired gate ID",
            ));
        }
    }

    if let Some(ref prev) = ctx.previous_ids {
        for prev_id in prev {
            if !all_gate_ids.contains(prev_id.as_str()) && !all_retired.contains(prev_id.as_str()) {
                violations.push(Violation::from_harness(
                    HarnessReason::ReusedId,
                    Some(prev_id.clone()),
                    Some("id"),
                    format!("previously registered gate `{prev_id}` was removed or renumbered"),
                    "Identifiers are append-only; retire rather than remove or renumber",
                ));
            }
        }
    }

    let check_missing_recurring =
        ctx.enforce_recurring_flags || register.has_register_layer_fields;

    for g in &register.gates {
        validate_gate_row(g, &all_gate_ids, ctx, check_missing_recurring, &mut violations);
    }

    kill::validate_kill_register(register, ctx, &mut violations);

    violations
}

/// Pure append-only ID transition validator for property testing (`ids_append_only`, `INV-GAUNT-07`).
#[must_use]
pub fn check_append_only_ids(
    previous: &[String],
    retired: &BTreeSet<String>,
    current: &[String],
) -> Vec<Violation> {
    let mut violations = Vec::new();
    let mut seen = BTreeSet::new();
    let cur_set: BTreeSet<&str> = current.iter().map(String::as_str).collect();

    for id in current {
        if !seen.insert(id.as_str()) {
            violations.push(Violation::from_harness(
                HarnessReason::DuplicateId,
                Some(id.clone()),
                Some("id"),
                format!("duplicate gate identifier `{id}`"),
                "Identifiers are append-only",
            ));
        }
        if retired.contains(id) {
            violations.push(Violation::from_harness(
                HarnessReason::ReusedId,
                Some(id.clone()),
                Some("id"),
                format!("retired gate identifier `{id}` returned"),
                "Never reuse a retired gate identifier",
            ));
        }
    }
    for (idx, prev_id) in previous.iter().enumerate() {
        if !cur_set.contains(prev_id.as_str()) {
            violations.push(Violation::from_harness(
                HarnessReason::ReusedId,
                Some(prev_id.clone()),
                Some("id"),
                format!("gate `{prev_id}` was removed or renumbered"),
                "Identifiers are append-only",
            ));
        } else if current.get(idx).map(String::as_str) != Some(prev_id.as_str()) {
            violations.push(Violation::from_harness(
                HarnessReason::ReusedId,
                Some(prev_id.clone()),
                Some("id"),
                format!("gate `{prev_id}` moved from index {idx} (renumbered)"),
                "Preserve existing gate order; append new IDs at the end",
            ));
        }
    }
    violations
}

fn validate_gate_row(
    g: &GateRow,
    all_gate_ids: &BTreeSet<&str>,
    ctx: &ValidationContext,
    check_missing_recurring: bool,
    out: &mut Vec<Violation>,
) {
    // Sources grammar check (Review Note #5 / NAB-10): SRC-... with optional #figure.
    for src in &g.sources {
        if !is_valid_source_citation(src) {
            out.push(Violation::schema(
                "invalid-source-citation",
                Some(g.id.clone()),
                Some("sources"),
                format!("invalid source citation `{src}` (expected `SRC-...` with optional `#figure`)"),
                "cite a registered SRC-... identifier",
            ));
        }
    }

    // `blocks_exit = false` allowed only on admission rows (`N40-01` step 1).
    if !g.blocks_exit && g.effective_kind() != Kind::Admission {
        out.push(Violation::schema(
            "blocks-exit-non-admission",
            Some(g.id.clone()),
            Some("blocks_exit"),
            format!(
                "`blocks_exit = false` is valid only on `admission` rows, got `{}`",
                g.effective_kind().as_str()
            ),
            "set blocks_exit = true or remove the field",
        ));
    }

    // Kill rows measure nothing (rule 3 does not apply to kill rows).
    if g.is_kill() {
        return;
    }

    // `needs = "import-lane"` requires `claim = "reality"` (`N40-01` step 1).
    if g.needs == Needs::ImportLane && g.claim != Some(Claim::Reality) {
        out.push(Violation::from_harness(
            HarnessReason::NoClaimOnNonReality,
            Some(g.id.clone()),
            Some("needs"),
            "`needs = \"import-lane\"` is allowed only on gates with `claim = \"reality\"`",
            "Use `needs = \"simulator\"` on non-reality gates",
        ));
    }

    // Rule 8: recurring set check.
    let should_recur = is_known_recurring_gate(&g.id);
    if g.recurring && !should_recur {
        out.push(Violation::schema(
            "recurring-flag-mismatch",
            Some(g.id.clone()),
            Some("recurring"),
            format!("gate `{}` is not in the recurring set of rule 8", g.id),
            "remove `recurring = true` from non-recurring gates",
        ));
    } else if check_missing_recurring && should_recur && !g.recurring {
        out.push(Violation::schema(
            "recurring-flag-mismatch",
            Some(g.id.clone()),
            Some("recurring"),
            format!("recurring gate `{}` is missing `recurring = true` (rule 8)", g.id),
            "set `recurring = true` on every rule-8 recurring gate",
        ));
    }

    // Validate `clm:` claim_id syntax whenever prefixed with `clm:`.
    if let Some(ref cid) = g.claim_id
        && cid.starts_with("clm:")
        && ClaimId::parse(cid).is_err()
    {
        out.push(Violation::from_harness(
            HarnessReason::EvidenceUnresolved,
            Some(g.id.clone()),
            Some("claim_id"),
            format!("malformed ClaimId `{cid}` (expected `clm:<64 lower-case hex>`)"),
            "Cite a valid N34-11 ClaimId",
        ));
    }

    let threshold = g.threshold.as_deref().unwrap_or("");
    for (pidx, part) in g.parts.iter().enumerate() {
        validate_part(g, part, pidx, threshold, all_gate_ids, ctx, out);
    }
}

fn validate_part(
    g: &GateRow,
    part: &Part,
    pidx: usize,
    threshold: &str,
    all_gate_ids: &BTreeSet<&str>,
    ctx: &ValidationContext,
    out: &mut Vec<Violation>,
) {
    // Review Note #4: `scope` starting with `pilot:` must carry valid N38a-09 pilot ID and stay `not-run`.
    if let Some(raw_scope) = part.scope.as_deref()
        && let Some(after_colon) = raw_scope.strip_prefix("pilot:")
    {
        let pid = after_colon.trim();
        if !is_valid_pilot_id(pid) {
            out.push(Violation::schema(
                "invalid-pilot-scope",
                Some(g.id.clone()),
                Some("scope"),
                format!("part[{pidx}] pilot scope `{raw_scope}` does not match N38a-09 PILOT-id grammar"),
                "use `pilot: <PILOT-id>` matching N38a-09 grammar",
            ));
        }
        if part.status != Status::NotRun {
            out.push(Violation::schema(
                "pilot-scope-must-be-not-run",
                Some(g.id.clone()),
                Some("status"),
                format!(
                    "part[{pidx}] with pilot scope `{raw_scope}` must stay `not-run` (read from pilot register by N40-04)"
                ),
                "keep status = \"not-run\" on pilot-backed parts",
            ));
        }
    }

    match part.status {
        Status::NotRun => {}
        Status::Pass => {
            // 1. Required fields + target-as-measurement.
            if part.measured.trim().is_empty() || part.measured == threshold {
                out.push(Violation::from_harness(
                    HarnessReason::TargetAsMeasurement,
                    Some(g.id.clone()),
                    Some("measured"),
                    format!(
                        "part[{pidx}] `measured` is empty or byte-equal to `threshold`"
                    ),
                    "Record the actual measured value",
                ));
            }
            for (fld, val) in [
                ("commit", part.commit.as_str()),
                ("command", part.command.as_str()),
                ("fingerprint", part.fingerprint.as_str()),
                ("seed", part.seed.as_str()),
            ] {
                if val.trim().is_empty() {
                    out.push(Violation::schema(
                        "missing-pass-field",
                        Some(g.id.clone()),
                        Some(fld),
                        format!("part[{pidx}] `pass` requires non-empty `{fld}`"),
                        "Record complete pass provenance fields (commit, command, receipt, fingerprint, seed)",
                    ));
                }
            }
            if part.receipt.trim().is_empty() {
                out.push(Violation::from_harness(
                    HarnessReason::EvidenceUnresolved,
                    Some(g.id.clone()),
                    Some("receipt"),
                    format!("part[{pidx}] `pass` requires non-empty `receipt`"),
                    "Commit the receipt under receipts/gates/<hash>.json and cite its hash",
                ));
            } else {
                verify_receipt(&g.id, pidx, &part.receipt, ctx, out);
            }

            // 2. Pass refusal (a): controls non-empty.
            if g.controls.is_empty() {
                out.push(Violation::from_harness(
                    HarnessReason::PassWithoutControl,
                    Some(g.id.clone()),
                    Some("controls"),
                    format!("part[{pidx}] `pass` refused: `controls` list is empty"),
                    "Register the negative control first (ADR-GAUNT-03)",
                ));
            } else if let Some(ref controls_reg) = ctx.controls {
                // Pass refusal (b) + Review Note #3(1): every DUE control must be caught.
                let part_ord = parse_milestone_ord(&part.milestone).unwrap_or(0);
                for ctrl_id in &g.controls {
                    match controls_reg.get(ctrl_id) {
                        None => {
                            out.push(Violation::from_harness(
                                HarnessReason::ControlNotCaught,
                                Some(g.id.clone()),
                                Some("controls"),
                                format!("control `{ctrl_id}` is not registered in controls.toml"),
                                "Register and run the control with the gate until caught",
                            ));
                        }
                        Some(ctrl) => {
                            let is_due = match ctrl.due.as_deref().and_then(parse_milestone_ord) {
                                Some(due_ord) => due_ord <= part_ord,
                                None => true,
                            };
                            if is_due && !ctrl.last_seen.as_ref().is_some_and(|ls| ls.caught) {
                                out.push(Violation::from_harness(
                                    HarnessReason::ControlNotCaught,
                                    Some(g.id.clone()),
                                    Some("controls"),
                                    format!(
                                        "due control `{ctrl_id}` was not caught in the passing run"
                                    ),
                                    "Fix the gate's test or the plant so the control is caught",
                                ));
                            }
                        }
                    }
                }
            }

            // 3. Pass refusal (c): self-certification check (session in comma-joined implemented_by).
            let rec_sess = part.recorded.session.trim();
            if rec_sess.is_empty() {
                out.push(Violation::from_harness(
                    HarnessReason::SelfCertified,
                    Some(g.id.clone()),
                    Some("recorded.session"),
                    format!("part[{pidx}] `pass` requires a non-empty `recorded.session`"),
                    "Record from an identified fresh-context session",
                ));
            } else if is_session_implementer(rec_sess, &part.recorded.implemented_by) {
                out.push(Violation::from_harness(
                    HarnessReason::SelfCertified,
                    Some(g.id.clone()),
                    Some("recorded.session"),
                    format!(
                        "recording session `{rec_sess}` is in `implemented_by` (`{}`)",
                        part.recorded.implemented_by
                    ),
                    "Record from a fresh-context session that did not implement the capability",
                ));
            }

            // 4. Pass refusal (d) + Review Notes #1 & #3(2): ClaimId parsing + evidence resolution.
            match g.claim_id.as_deref() {
                None => {
                    out.push(Violation::from_harness(
                        HarnessReason::EvidenceUnresolved,
                        Some(g.id.clone()),
                        Some("claim_id"),
                        "missing `claim_id` on `pass` gate",
                        "Cite a live N34-11 ClaimId (clm:<64 hex>)",
                    ));
                }
                Some(cid) => {
                    if ClaimId::parse(cid).is_err() {
                        out.push(Violation::from_harness(
                            HarnessReason::EvidenceUnresolved,
                            Some(g.id.clone()),
                            Some("claim_id"),
                            format!(
                                "`pass` requires `claim_id` in N34-11 `clm:<64 lower-case hex>` form, got `{cid}`"
                            ),
                            "Cite a valid N34-11 ClaimId resolving in the artifact graph",
                        ));
                    } else if let Some(ref live) = ctx.live_claims
                        && !live.contains(cid)
                    {
                        out.push(Violation::from_harness(
                            HarnessReason::EvidenceUnresolved,
                            Some(g.id.clone()),
                            Some("claim_id"),
                            format!("claim_id `{cid}` does not resolve in the artifact graph"),
                            "Cite a live, non-tombstoned claim in the artifact graph",
                        ));
                    }
                }
            }
            if part.evidence.is_empty() {
                out.push(Violation::from_harness(
                    HarnessReason::EvidenceUnresolved,
                    Some(g.id.clone()),
                    Some("evidence"),
                    format!("part[{pidx}] `pass` requires at least one evidence item"),
                    "Cite live artifact-graph evidence identifiers",
                ));
            } else if let Some(ref live_ev) = ctx.live_evidence {
                for ev in &part.evidence {
                    if !live_ev.contains(ev) {
                        out.push(Violation::from_harness(
                            HarnessReason::EvidenceUnresolved,
                            Some(g.id.clone()),
                            Some("evidence"),
                            format!("evidence item `{ev}` does not resolve in the artifact graph"),
                            "Re-run and cite live, non-tombstoned evidence",
                        ));
                    }
                }
            }
        }
        Status::Fail => {
            // Rule 1 (`fail_is_a_record`, RC-67): fail requires measured, commit, command, resolving receipt.
            if part.measured.trim().is_empty() || part.measured == threshold {
                out.push(Violation::from_harness(
                    HarnessReason::TargetAsMeasurement,
                    Some(g.id.clone()),
                    Some("measured"),
                    format!("part[{pidx}] `fail` requires non-empty `measured` distinct from `threshold`"),
                    "Record the actual failing measurement",
                ));
            }
            for (fld, val) in [
                ("commit", part.commit.as_str()),
                ("command", part.command.as_str()),
            ] {
                if val.trim().is_empty() {
                    out.push(Violation::schema(
                        "missing-fail-field",
                        Some(g.id.clone()),
                        Some(fld),
                        format!("part[{pidx}] `fail` requires non-empty `{fld}`"),
                        "Record commit, command, and receipt on every fail record",
                    ));
                }
            }
            if part.receipt.trim().is_empty() {
                out.push(Violation::from_harness(
                    HarnessReason::EvidenceUnresolved,
                    Some(g.id.clone()),
                    Some("receipt"),
                    format!("part[{pidx}] `fail` requires non-empty `receipt`"),
                    "Commit the failing receipt under receipts/gates/<hash>.json",
                ));
            } else {
                verify_receipt(&g.id, pidx, &part.receipt, ctx, out);
            }
        }
        Status::NoClaim => {
            let detail_ok = part
                .detail
                .as_deref()
                .is_some_and(|d| !d.trim().is_empty());
            if g.claim != Some(Claim::Reality) || !detail_ok {
                out.push(Violation::from_harness(
                    HarnessReason::NoClaimOnNonReality,
                    Some(g.id.clone()),
                    Some("status"),
                    format!(
                        "part[{pidx}] `no-claim` is valid only on `claim = \"reality\"` gates with a non-empty reason in `detail`"
                    ),
                    "Use `not-run` or `fail` on non-reality gates; on a Reality gate name the reason (e.g. `no-dataset-admitted`) in `detail`",
                ));
            }
        }
        Status::Blocked => {
            let candidate = part
                .detail
                .as_deref()
                .filter(|s| !s.trim().is_empty())
                .or_else(|| part.open_question.as_deref().filter(|s| !s.trim().is_empty()));
            if !candidate.is_some_and(is_valid_blocked_detail) {
                out.push(Violation::from_harness(
                    HarnessReason::BlockedWithoutQuestion,
                    Some(g.id.clone()),
                    Some("detail"),
                    format!(
                        "part[{pidx}] `blocked` requires an `OPEN-...` identifier or `claim-reading: <bead>` in `detail`"
                    ),
                    "Name the blocking OPEN-... question or `claim-reading: <bead>` in `detail`",
                ));
            }
        }
        Status::Killed => {
            if !has_valid_killed_decision(g, part) {
                out.push(Violation::from_harness(
                    HarnessReason::KilledWithoutDecision,
                    Some(g.id.clone()),
                    Some("detail"),
                    format!(
                        "part[{pidx}] `killed` requires kill gate (or program bet), adopted fallback, and ADR in `detail`"
                    ),
                    "Record the kill-gate decision, adopted fallback, and ADR in `detail` (N40-03)",
                ));
            }
        }
        Status::Superseded => {
            let target = part.detail.as_deref().map(str::trim).unwrap_or("");
            if target.is_empty() || target == g.id || !all_gate_ids.contains(target) {
                out.push(Violation::schema(
                    "superseded-target-missing",
                    Some(g.id.clone()),
                    Some("detail"),
                    format!(
                        "part[{pidx}] `superseded` detail `{target}` must name an existing successor gate in gates.toml"
                    ),
                    "Set `detail` to the existing successor gate identifier",
                ));
            }
        }
    }
}

fn verify_receipt(
    gate_id: &str,
    pidx: usize,
    raw_receipt: &str,
    ctx: &ValidationContext,
    out: &mut Vec<Violation>,
) {
    let hex = raw_receipt.strip_prefix("b3:").unwrap_or(raw_receipt);
    if Blake3::from_hex(hex).is_err() {
        out.push(Violation::from_harness(
            HarnessReason::EvidenceUnresolved,
            Some(gate_id.to_owned()),
            Some("receipt"),
            format!("part[{pidx}] receipt `{raw_receipt}` is not a 64 lower-case hex BLAKE3 digest"),
            "Cite the 64 lower-case hex BLAKE3 digest of receipts/gates/<hash>.json",
        ));
        return;
    }

    let Some(ref root) = ctx.repo_root else {
        return;
    };
    let receipt_path = root.join("receipts/gates").join(format!("{hex}.json"));
    let bytes = match fs::read(&receipt_path) {
        Ok(b) => b,
        Err(_) => {
            out.push(Violation::from_harness(
                HarnessReason::EvidenceUnresolved,
                Some(gate_id.to_owned()),
                Some("receipt"),
                format!(
                    "part[{pidx}] receipt `{hex}` not found at committed path `receipts/gates/{hex}.json`"
                ),
                "Commit the receipt file under receipts/gates/<hash>.json",
            ));
            return;
        }
    };
    let actual_hex = hash_blob(&bytes).to_hex();
    if actual_hex != hex {
        out.push(Violation::from_harness(
            HarnessReason::EvidenceUnresolved,
            Some(gate_id.to_owned()),
            Some("receipt"),
            format!(
                "part[{pidx}] receipt file `receipts/gates/{hex}.json` hashed to `{actual_hex}` instead of `{hex}`"
            ),
            "Commit exact unmodified receipt bytes matching their BLAKE3 filename",
        ));
        return;
    }

    // If the file is a receipt set (`frob.gauntlet.receipt-set/1`), verify every member too.
    if let Ok(admitted) = json::admit(&bytes, &Limits::default())
        && let Some(members) = admitted.root().value.as_object()
    {
        let is_set = members.iter().any(|m| {
            m.name == "schema"
                && m.value.value.as_str() == Some("frob.gauntlet.receipt-set/1")
        });
        if is_set {
            let member_arr = members
                .iter()
                .find(|m| m.name == "members" || m.name == "receipts")
                .and_then(|m| m.value.value.as_array());
            match member_arr {
                None | Some([]) => {
                    out.push(Violation::from_harness(
                        HarnessReason::EvidenceUnresolved,
                        Some(gate_id.to_owned()),
                        Some("receipt"),
                        format!("part[{pidx}] receipt set `{hex}` lists no member receipts"),
                        "Record all member receipt hashes inside the receipt set",
                    ));
                }
                Some(arr) => {
                    for item in arr {
                        let m_hex_raw = match &item.value {
                            Json::Str(s) => s.as_str(),
                            Json::Obj(obj) => obj
                                .iter()
                                .find(|m| m.name == "receipt" || m.name == "hash")
                                .and_then(|m| m.value.value.as_str())
                                .unwrap_or(""),
                            _ => "",
                        };
                        let m_hex = m_hex_raw.strip_prefix("b3:").unwrap_or(m_hex_raw);
                        if Blake3::from_hex(m_hex).is_err() {
                            out.push(Violation::from_harness(
                                HarnessReason::EvidenceUnresolved,
                                Some(gate_id.to_owned()),
                                Some("receipt"),
                                format!(
                                    "part[{pidx}] receipt set `{hex}` contains invalid member hash `{m_hex_raw}`"
                                ),
                                "Commit valid 64-hex member receipts under receipts/gates/",
                            ));
                            continue;
                        }
                        let m_path = root.join("receipts/gates").join(format!("{m_hex}.json"));
                        match fs::read(&m_path) {
                            Ok(mb) if hash_blob(&mb).to_hex() == m_hex => {}
                            Ok(_) => out.push(Violation::from_harness(
                                HarnessReason::EvidenceUnresolved,
                                Some(gate_id.to_owned()),
                                Some("receipt"),
                                format!(
                                    "part[{pidx}] receipt-set member `{m_hex}` failed BLAKE3 hash verification"
                                ),
                                "Restore unmodified member receipt bytes under receipts/gates/",
                            )),
                            Err(_) => out.push(Violation::from_harness(
                                HarnessReason::EvidenceUnresolved,
                                Some(gate_id.to_owned()),
                                Some("receipt"),
                                format!(
                                    "part[{pidx}] receipt-set member `{m_hex}` missing at `receipts/gates/{m_hex}.json`"
                                ),
                                "Commit every member receipt beside the receipt set",
                            )),
                        }
                    }
                }
            }
        }
    }
}

/// True iff `session` appears in the comma-separated `implemented_by` list (`INV-GAUNT-05`).
#[must_use]
pub fn is_session_implementer(session: &str, implemented_by: &str) -> bool {
    let s = session.trim();
    if s.is_empty() {
        return false;
    }
    implemented_by
        .split(',')
        .map(str::trim)
        .any(|m| !m.is_empty() && m == s)
}

/// True iff `id` is one of the 9 rule-8 recurring gates (4 compiled + 5 late).
#[must_use]
pub fn is_known_recurring_gate(id: &str) -> bool {
    COMPILED_RECURRING_GATES.contains(&id) || LATE_RECURRING_GATES.contains(&id)
}

/// True iff `detail` names a valid blocking cause (`OPEN-...` or `claim-reading: <bead>`).
#[must_use]
pub fn is_valid_blocked_detail(detail: &str) -> bool {
    let d = detail.trim();
    if let Some(rest) = d.strip_prefix("OPEN-") {
        return !rest.is_empty()
            && rest
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'-');
    }
    if let Some(rest) = d.strip_prefix("claim-reading:") {
        return !rest.trim().is_empty();
    }
    false
}

/// True iff a `killed` part names (1) kill gate (or `BET-` for a `program` row),
/// (2) an `ADR-` decision record, and (3) a non-empty adopted fallback.
#[must_use]
pub fn has_valid_killed_decision(g: &GateRow, part: &Part) -> bool {
    let detail = part.detail.as_deref().unwrap_or("");
    let combined = format!(
        "{} {} {}",
        part.kill_gate.as_deref().unwrap_or(""),
        part.fallback.as_deref().unwrap_or(""),
        detail
    );
    let has_trigger = combined.contains("KILL-")
        || (g.effective_kind() == Kind::Program && combined.contains("BET-"));
    let has_adr = combined.contains("ADR-");
    let has_fallback = part
        .fallback
        .as_deref()
        .is_some_and(|s| !s.trim().is_empty())
        || detail
            .to_ascii_lowercase()
            .contains("fallback")
        || detail
            .split(|c: char| matches!(c, ';' | ',' | '|'))
            .filter(|seg| !seg.trim().is_empty())
            .count()
            >= 3;
    has_trigger && has_adr && has_fallback
}

/// Matches `N38a-09`'s exact pilot identifier grammar (Review Note #4).
#[must_use]
pub fn is_valid_pilot_id(id: &str) -> bool {
    let Some(rest) = id.strip_prefix("PILOT-") else {
        return false;
    };
    let b = rest.as_bytes();
    if let Some(after_c) = rest.strip_prefix("C-") {
        let cb = after_c.as_bytes();
        if cb.len() == 2 && cb[0].is_ascii_digit() && cb[1].is_ascii_digit() {
            return true;
        }
        return cb.len() >= 4
            && cb[0].is_ascii_digit()
            && cb[1].is_ascii_digit()
            && cb[2] == b'-'
            && cb[3..].iter().all(u8::is_ascii_lowercase);
    }
    if let Some(after_pt) = rest.strip_prefix("PT-") {
        let pb = after_pt.as_bytes();
        return pb.len() == 2
            && ((pb[0].is_ascii_digit() && pb[1].is_ascii_digit())
                || (pb[0] == b'S' && pb[1].is_ascii_digit()));
    }
    if b.len() < 5 {
        return false;
    }
    let split = b.len() - 3;
    if b[split] != b'-' || !b[split + 1].is_ascii_digit() || !b[split + 2].is_ascii_digit() {
        return false;
    }
    let set = &b[..split];
    (2..=14).contains(&set.len()) && set.iter().all(u8::is_ascii_uppercase)
}

/// Matches `NAB-10` optional source citation (`SRC-…`, optional `#<figure>`; Review Note #5).
#[must_use]
pub fn is_valid_source_citation(s: &str) -> bool {
    let (id_part, fig_part) = match s.split_once('#') {
        Some((a, b)) => (a, Some(b)),
        None => (s, None),
    };
    let Some(rest) = id_part.strip_prefix("SRC-") else {
        return false;
    };
    let id_ok = !rest.is_empty()
        && rest
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_');
    let fig_ok = match fig_part {
        Some(f) => !f.trim().is_empty() && !f.contains('#'),
        None => true,
    };
    id_ok && fig_ok
}

fn parse_str_vec(
    items: Vec<Spanned<Value>>,
    gate_id: Option<&str>,
    field: &str,
) -> Result<Vec<String>, Vec<Violation>> {
    let mut out = Vec::with_capacity(items.len());
    for sp in items {
        match sp.value {
            Value::Str(s) => out.push(s),
            other => {
                return Err(vec![Violation::schema(
                    "schema-invalid",
                    gate_id.map(str::to_owned),
                    Some(field),
                    format!("`{field}` elements must be strings, got {}", other.type_name()),
                    "use string elements",
                )]);
            }
        }
    }
    Ok(out)
}

fn parse_gate_row(
    tbl: Table,
    idx: usize,
    file: &str,
) -> Result<(GateRow, bool), Vec<Violation>> {
    let label = tbl
        .get("id")
        .and_then(Value::as_str)
        .filter(|s| !s.trim().is_empty())
        .map_or_else(|| format!("gate[{idx}]"), str::to_owned);
    let mut f = Fields::new(tbl, format!("gate[{label}]"));
    let toml_err = |fld: &'static str, e: toml::TomlError| {
        vec![Violation::schema(
            "schema-invalid",
            Some(label.clone()),
            Some(fld),
            e.to_string(),
            "fix field type or presence",
        )]
    };

    let id = f.require_str("id").map_err(|e| toml_err("id", e))?;
    if id.trim().is_empty() {
        return Err(f.fail(vec![Violation::schema(
            "schema-invalid",
            Some(label),
            Some("id"),
            "`id` must be non-empty",
            "supply a unique non-empty gate identifier",
        )]));
    }

    let mut has_reg = false;
    let kind = match f.opt_str("kind").map_err(|e| toml_err("kind", e))? {
        Some(raw) => {
            has_reg = true;
            match Kind::parse(&raw, file, &id) {
                Ok(k) => Some(k),
                Err(e) => {
                    return Err(f.fail(vec![Violation::schema(
                        "invalid-kind",
                        Some(id),
                        Some("kind"),
                        e.message,
                        "use admission, convergence, program, or kill",
                    )]));
                }
            }
        }
        None => None,
    };

    let section = f
        .require_str("section")
        .map_err(|e| toml_err("section", e))?;
    if section.trim().is_empty() {
        return Err(f.fail(vec![Violation::schema(
            "schema-invalid",
            Some(id),
            Some("section"),
            "`section` must be non-empty",
            "supply the defining plan subsection",
        )]));
    }

    let consumers = match f
        .opt_array("consumers")
        .map_err(|e| toml_err("consumers", e))?
    {
        Some(arr) => parse_str_vec(arr, Some(&id), "consumers").map_err(|e| f.fail(e))?,
        None => Vec::new(),
    };
    let sources = match f.opt_array("sources").map_err(|e| toml_err("sources", e))? {
        Some(arr) => parse_str_vec(arr, Some(&id), "sources").map_err(|e| f.fail(e))?,
        None => Vec::new(),
    };
    let recurring = match f
        .opt_bool("recurring")
        .map_err(|e| toml_err("recurring", e))?
    {
        Some(b) => {
            has_reg = true;
            b
        }
        None => false,
    };
    let needs = match f.opt_str("needs").map_err(|e| toml_err("needs", e))? {
        Some(raw) => match Needs::parse(&raw) {
            Some(n) => n,
            None => {
                return Err(f.fail(vec![Violation::schema(
                    "invalid-needs",
                    Some(id),
                    Some("needs"),
                    format!("unknown `needs` value `{raw}` (expected `simulator` or `import-lane`)"),
                    "use `simulator` or `import-lane`",
                )]));
            }
        },
        None => Needs::Simulator,
    };
    let blocks_exit = f
        .opt_bool("blocks_exit")
        .map_err(|e| toml_err("blocks_exit", e))?
        .unwrap_or(true);

    if kind == Some(Kind::Kill) {
        for &forbidden in KILL_FORBIDDEN_FIELDS {
            if f.take(forbidden).is_some() {
                return Err(f.fail(vec![Violation::schema(
                    "kill-row-measurement-field",
                    Some(id),
                    Some(forbidden),
                    format!("kill row must not carry measurement field `{forbidden}`"),
                    "remove measurement fields from kill rows",
                )]));
            }
        }
        let trigger = f.opt_str("trigger").map_err(|e| toml_err("trigger", e))?;
        let fallback = f.opt_str("fallback").map_err(|e| toml_err("fallback", e))?;
        let decided = f.opt_str("decided").map_err(|e| toml_err("decided", e))?;
        if let Some(ref d) = decided
            && parse_milestone_ord(d).is_none()
        {
            return Err(f.fail(vec![Violation::schema(
                "invalid-milestone",
                Some(id),
                Some("decided"),
                format!("invalid `decided` milestone `{d}`"),
                "use M0..M8",
            )]));
        }
        let (mode, mode_explicit) = match f.opt_str("mode").map_err(|e| toml_err("mode", e))? {
            Some(raw) => match KillMode::parse(&raw) {
                Some(m) => (m, true),
                None => {
                    return Err(f.fail(vec![Violation::schema(
                        "invalid-kill-mode",
                        Some(id),
                        Some("mode"),
                        format!(
                            "unknown kill mode `{raw}` (expected: exit, pilot-before-funding, standing, rolling)"
                        ),
                        "use one of the four closed modes: exit, pilot-before-funding, standing, rolling",
                    )]));
                }
            },
            None => (KillMode::Exit, false),
        };
        let reads = match f.opt_array("reads").map_err(|e| toml_err("reads", e))? {
            Some(arr) => parse_str_vec(arr, Some(&id), "reads").map_err(|e| f.fail(e))?,
            None => Vec::new(),
        };
        let (decisions, decision_as_array) = match f.take("decision") {
            Some(sp) => kill::parse_decision_value(sp, &id).map_err(|e| f.fail(e))?,
            None => (Vec::new(), false),
        };
        let reread = match f.take("reread") {
            Some(sp) => match sp.value {
                Value::Array(arr) => {
                    parse_str_vec(arr, Some(&id), "reread").map_err(|e| f.fail(e))?
                }
                Value::Str(s) => vec![s],
                other => {
                    return Err(f.fail(vec![Violation::schema(
                        "schema-invalid",
                        Some(id),
                        Some("reread"),
                        format!(
                            "`reread` must be an array of milestone strings, got {}",
                            other.type_name()
                        ),
                        "encode `reread` as an array of M0..M8 strings (e.g. `[\"M6\"]`)",
                    )]));
                }
            },
            None => Vec::new(),
        };
        let scope_kind = match f
            .opt_str("scope_kind")
            .map_err(|e| toml_err("scope_kind", e))?
        {
            Some(raw) => match ScopeKind::parse(&raw) {
                Some(sk) => Some(sk),
                None => {
                    return Err(f.fail(vec![Violation::schema(
                        "decision-scope-invalid",
                        Some(id),
                        Some("scope_kind"),
                        format!(
                            "unknown `scope_kind` `{raw}` (not in the closed scope-kind table)"
                        ),
                        "use a closed scope kind: cell, algorithm, half, clause, backend, item, resolution, kind, family, mechanism, proofkind, or morphology",
                    )]));
                }
            },
            None => None,
        };
        let milestone = f
            .opt_str("milestone")
            .map_err(|e| toml_err("milestone", e))?;
        if let Err(e) = f.finish() {
            return Err(vec![Violation::schema(
                "unknown-field",
                Some(id),
                None,
                e.to_string(),
                "remove unknown fields from kill gate row",
            )]);
        }
        return Ok((
            GateRow {
                id,
                kind,
                section,
                consumers,
                sources,
                recurring,
                needs,
                blocks_exit,
                claim: None,
                claim_id: None,
                threshold: None,
                milestone: None,
                tier: Vec::new(),
                controls: Vec::new(),
                parts: Vec::new(),
                admission: AdmissionFields::default(),
                kill: KillFields {
                    trigger,
                    fallback,
                    decided,
                    mode,
                    mode_explicit,
                    reads,
                    decisions,
                    decision_as_array,
                    reread,
                    scope_kind,
                    milestone,
                },
            },
            has_reg,
        ));
    }

    // Measured row (`admission` / `convergence` / `program` / untyped base row).
    let Some(claim_sp) = f.take("claim") else {
        return Err(f.fail(vec![Violation::from_harness(
            HarnessReason::ClaimNotOneOfSeven,
            Some(id),
            Some("claim"),
            "missing required field `claim`",
            "Apply rule 3 of the gate register",
        )]));
    };
    let claim_raw = match claim_sp.value {
        Value::Str(s) => s,
        Value::Array(_) => {
            return Err(f.fail(vec![Violation::from_harness(
                HarnessReason::ClaimNotOneOfSeven,
                Some(id),
                Some("claim"),
                "`claim` must be a single string from the seven closed kinds, not an array",
                "Apply rule 3 of the gate register (the claim of the governing part)",
            )]));
        }
        other => {
            return Err(f.fail(vec![Violation::from_harness(
                HarnessReason::ClaimNotOneOfSeven,
                Some(id),
                Some("claim"),
                format!("`claim` must be a string, got {}", other.type_name()),
                "Apply rule 3 of the gate register",
            )]));
        }
    };
    let claim = match Claim::parse(&claim_raw, file, &id) {
        Ok(c) => c,
        Err(e) => {
            return Err(f.fail(vec![Violation::from_harness(
                HarnessReason::ClaimNotOneOfSeven,
                Some(id),
                Some("claim"),
                e.message,
                "Apply rule 3 of the gate register",
            )]));
        }
    };

    let claim_id = f
        .require_str("claim_id")
        .map_err(|e| toml_err("claim_id", e))?;
    if claim_id.trim().is_empty() {
        return Err(f.fail(vec![Violation::from_harness(
            HarnessReason::EvidenceUnresolved,
            Some(id),
            Some("claim_id"),
            "`claim_id` must be non-empty",
            "Cite a valid claim identifier",
        )]));
    }

    let threshold = f
        .require_str("threshold")
        .map_err(|e| toml_err("threshold", e))?;
    if threshold.trim().is_empty() {
        return Err(f.fail(vec![Violation::schema(
            "schema-invalid",
            Some(id),
            Some("threshold"),
            "`threshold` must be non-empty",
            "state the gate threshold brief",
        )]));
    }

    let milestone = f
        .require_str("milestone")
        .map_err(|e| toml_err("milestone", e))?;
    if parse_milestone_ord(&milestone).is_none() {
        return Err(f.fail(vec![Violation::schema(
            "invalid-milestone",
            Some(id),
            Some("milestone"),
            format!("invalid `milestone` `{milestone}` (expected M0..M8)"),
            "use M0..M8",
        )]));
    }

    let tier_raw = f.require_array("tier").map_err(|e| toml_err("tier", e))?;
    let tier = parse_str_vec(tier_raw, Some(&id), "tier").map_err(|e| f.fail(e))?;
    if tier.is_empty() {
        return Err(f.fail(vec![Violation::schema(
            "schema-invalid",
            Some(id),
            Some("tier"),
            "`tier` must contain at least one entry",
            "list at least one execution tier (e.g. G0:host)",
        )]));
    }

    let controls_raw = f
        .require_array("controls")
        .map_err(|e| toml_err("controls", e))?;
    let controls = parse_str_vec(controls_raw, Some(&id), "controls").map_err(|e| f.fail(e))?;

    let foundation = f
        .opt_str("foundation")
        .map_err(|e| toml_err("foundation", e))?;
    let first_needed = f
        .opt_str("first_needed")
        .map_err(|e| toml_err("first_needed", e))?;
    if let Some(ref fn_m) = first_needed
        && parse_milestone_ord(fn_m).is_none()
    {
        return Err(f.fail(vec![Violation::schema(
            "invalid-milestone",
            Some(id),
            Some("first_needed"),
            format!("invalid `first_needed` `{fn_m}` (expected M0..M8)"),
            "use M0..M8",
        )]));
    }

    let raw_parts = f
        .opt_array("part")
        .map_err(|e| toml_err("part", e))?
        .unwrap_or_default();
    let mut parts = Vec::with_capacity(raw_parts.len());
    for (pidx, sp) in raw_parts.into_iter().enumerate() {
        let ptbl = match sp.value {
            Value::Table(t) => t,
            other => {
                return Err(f.fail(vec![Violation::schema(
                    "schema-invalid",
                    Some(id),
                    Some("part"),
                    format!("part[{pidx}] expected table, got {}", other.type_name()),
                    "use [[gate.part]] tables",
                )]));
            }
        };
        parts.push(parse_part(ptbl, file, &id, pidx).map_err(|e| f.fail(e))?);
    }

    if let Err(e) = f.finish() {
        return Err(vec![Violation::schema(
            "unknown-field",
            Some(id),
            None,
            e.to_string(),
            "remove unknown fields from gate row",
        )]);
    }

    Ok((
        GateRow {
            id,
            kind,
            section,
            consumers,
            sources,
            recurring,
            needs,
            blocks_exit,
            claim: Some(claim),
            claim_id: Some(claim_id),
            threshold: Some(threshold),
            milestone: Some(milestone),
            tier,
            controls,
            parts,
            admission: AdmissionFields {
                foundation,
                first_needed,
            },
            kill: KillFields::default(),
        },
        has_reg,
    ))
}

fn parse_part(
    tbl: Table,
    file: &str,
    gate_id: &str,
    pidx: usize,
) -> Result<Part, Vec<Violation>> {
    let ctx = format!("{gate_id}.part[{pidx}]");
    let mut f = Fields::new(tbl, ctx);
    let toml_err = |fld: &'static str, e: toml::TomlError| {
        vec![Violation::schema(
            "schema-invalid",
            Some(gate_id.to_owned()),
            Some(fld),
            e.to_string(),
            "fix part field type or presence",
        )]
    };

    let milestone = f
        .require_str("milestone")
        .map_err(|e| toml_err("milestone", e))?;
    if parse_milestone_ord(&milestone).is_none() {
        return Err(f.fail(vec![Violation::schema(
            "invalid-milestone",
            Some(gate_id.to_owned()),
            Some("milestone"),
            format!("part[{pidx}] invalid milestone `{milestone}`"),
            "use M0..M8",
        )]));
    }
    let scope = f.opt_str("scope").map_err(|e| toml_err("scope", e))?;
    let status_raw = f.require_str("status").map_err(|e| toml_err("status", e))?;
    let status = match Status::parse(&status_raw, file, gate_id) {
        Ok(s) => s,
        Err(e) => {
            return Err(f.fail(vec![Violation::schema(
                "invalid-status",
                Some(gate_id.to_owned()),
                Some("status"),
                e.message,
                "use one of: not-run, pass, fail, killed, blocked, no-claim, superseded",
            )]));
        }
    };
    let measured = f
        .require_str("measured")
        .map_err(|e| toml_err("measured", e))?;
    let commit = f
        .opt_str("commit")
        .map_err(|e| toml_err("commit", e))?
        .unwrap_or_default();
    let command = f
        .opt_str("command")
        .map_err(|e| toml_err("command", e))?
        .unwrap_or_default();
    let receipt = f
        .opt_str("receipt")
        .map_err(|e| toml_err("receipt", e))?
        .unwrap_or_default();
    let fingerprint = f
        .opt_str("fingerprint")
        .map_err(|e| toml_err("fingerprint", e))?
        .unwrap_or_default();
    let seed = f
        .opt_str("seed")
        .map_err(|e| toml_err("seed", e))?
        .unwrap_or_default();

    let ev_raw = f
        .require_array("evidence")
        .map_err(|e| toml_err("evidence", e))?;
    let evidence = parse_str_vec(ev_raw, Some(gate_id), "evidence").map_err(|e| f.fail(e))?;

    let rec_tbl = f
        .require_table("recorded")
        .map_err(|e| toml_err("recorded", e))?;
    let mut rf = Fields::new(rec_tbl, format!("{gate_id}.part[{pidx}].recorded"));
    let recorded = Recorded {
        date: rf
            .require_str("date")
            .map_err(|e| f.fail(toml_err("recorded.date", e)))?,
        session: rf
            .require_str("session")
            .map_err(|e| f.fail(toml_err("recorded.session", e)))?,
        implemented_by: rf
            .require_str("implemented_by")
            .map_err(|e| f.fail(toml_err("recorded.implemented_by", e)))?,
    };
    if let Err(e) = rf.finish() {
        return Err(f.fail(vec![Violation::schema(
            "unknown-field",
            Some(gate_id.to_owned()),
            Some("recorded"),
            e.to_string(),
            "remove unknown fields from `recorded` table",
        )]));
    }

    let detail = f.opt_str("detail").map_err(|e| toml_err("detail", e))?;
    let kill_gate = f
        .opt_str("kill_gate")
        .map_err(|e| toml_err("kill_gate", e))?;
    let fallback = f.opt_str("fallback").map_err(|e| toml_err("fallback", e))?;
    let open_question = f
        .opt_str("open_question")
        .map_err(|e| toml_err("open_question", e))?;

    if let Err(e) = f.finish() {
        return Err(vec![Violation::schema(
            "unknown-field",
            Some(gate_id.to_owned()),
            None,
            e.to_string(),
            "remove unknown fields from [[gate.part]]",
        )]);
    }

    Ok(Part {
        milestone,
        scope,
        status,
        measured,
        commit,
        command,
        receipt,
        fingerprint,
        seed,
        evidence,
        recorded,
        detail,
        kill_gate,
        fallback,
        open_question,
    })
}

fn push_kv_str(out: &mut String, key: &str, val: &str) {
    out.push_str(key);
    out.push_str(" = ");
    push_str(out, val);
    out.push('\n');
}

fn push_str(out: &mut String, s: &str) {
    out.push('"');
    for ch in s.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c => out.push(c),
        }
    }
    out.push('"');
}

fn push_str_array(out: &mut String, items: &[String]) {
    out.push('[');
    for (i, item) in items.iter().enumerate() {
        if i > 0 {
            out.push_str(", ");
        }
        push_str(out, item);
    }
    out.push(']');
}
