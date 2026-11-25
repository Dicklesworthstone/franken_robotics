#![deny(unsafe_code)]

//! Kill-gate records, dated amendments, closed scope kinds, decision entries, ADR path
//! enforcement, due lists, and `apply_decisions` propagation (`N40-03`, `GATE-GAUNT-10`,
//! `INV-GAUNT-05`, `INV-GAUNT-06`, `N40-KA` R-10/R-32, `N40-CX` RC-10..RC-13, RC-57, RC-68..RC-70).
//!
//! Incorporates Round-4 review notes on `N40-03`:
//! - Note #1/#7: renaming is asserted on `apply_decisions` and its own README/register views;
//! - Note #2: pilot receipts resolve under `receipts/pilots/<hash>.json` (`N38a-09`);
//! - Note #3: closed scope-kind table includes `morphology` (`wheeled`, `legged`) for `KILL-SHIELD`;
//! - Note #4: `introduce` amendments cover `KILL-SEGMENT`, `KILL-RUNTIMECLOSURE`, `KILL-CONTINUUM`;
//! - Note #5/#6: `cell` covers `KILL-AUGUR-TIE` and `backend` covers `KILL-ENSEMBLE`;
//! - Note #8: `cell` accepts both `cell:<stratum>` and `cell:<stratum>:<regime>` (M7 cold/warm cells).
//!
//! Determinism class D0: pure replay, sorted outputs, zero `unsafe`, prints nothing (rule L-7),
//! reads TOML strictly through `frob_core::registry::toml` with `Fields::fail` on all value rejections.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::fs;
use std::path::Path;

use frob_core::id::{Blake3, hash_blob};
use frob_core::registry::toml::{self, Fields, Spanned, Table, Value};

use crate::reasons::HarnessReason;
use crate::register::row::{Status, parse_milestone_ord};
use crate::register::validate::{GateRow, Register, ValidationContext, Violation};

/// Canonical directory for all decision ADRs (`docs/adr/`, never `docs/delivery/adr/`).
pub const DECISION_ADR_DIR: &str = "docs/adr";

/// Authoritative list of the six standing or rolling kill gates named in `N40-03` step 2 (`N40-KA` R-10).
pub const STANDING_OR_ROLLING_GATES: &[&str] = &[
    "KILL-FSIM",
    "KILL-MCP",
    "KILL-AUGUR-FALSECERT",
    "KILL-AUGUR-REUSE",
    "KILL-TENFOLD-CERT",
    "KILL-TENFOLD-LEAK",
];

/// Closed set of kill-gate evaluation modes (`N40-03` step 2, `N40-KA` R-10).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum KillMode {
    #[default]
    Exit,
    PilotBeforeFunding,
    Standing,
    Rolling,
}

impl KillMode {
    pub const ALL: &'static [Self] = &[
        Self::Exit,
        Self::PilotBeforeFunding,
        Self::Standing,
        Self::Rolling,
    ];

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exit => "exit",
            Self::PilotBeforeFunding => "pilot-before-funding",
            Self::Standing => "standing",
            Self::Rolling => "rolling",
        }
    }

    #[must_use]
    pub fn parse(raw: &str) -> Option<Self> {
        match raw {
            "exit" => Some(Self::Exit),
            "pilot-before-funding" => Some(Self::PilotBeforeFunding),
            "standing" => Some(Self::Standing),
            "rolling" => Some(Self::Rolling),
            _ => None,
        }
    }
}

impl fmt::Display for KillMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Closed set of the four kill-gate decision outcomes (`N40-03` step 3, `N40-KA` R-10).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum KillOutcome {
    NotFired,
    Fired,
    NotReached,
    Deferred,
}

impl KillOutcome {
    pub const ALL: &'static [Self] = &[
        Self::NotFired,
        Self::Fired,
        Self::NotReached,
        Self::Deferred,
    ];

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotFired => "not-fired",
            Self::Fired => "fired",
            Self::NotReached => "not-reached",
            Self::Deferred => "deferred",
        }
    }

    #[must_use]
    pub fn parse(raw: &str) -> Option<Self> {
        match raw {
            "not-fired" => Some(Self::NotFired),
            "fired" => Some(Self::Fired),
            "not-reached" => Some(Self::NotReached),
            "deferred" => Some(Self::Deferred),
            _ => None,
        }
    }
}

impl fmt::Display for KillOutcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Closed table of scope kinds for per-scope kill gates (`N40-03` step 1 + Review Note #3 `morphology`, `N40-KA` R-32).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ScopeKind {
    Cell,
    Algorithm,
    Half,
    Clause,
    Backend,
    Item,
    Resolution,
    Kind,
    Family,
    Mechanism,
    Proofkind,
    Morphology,
}

impl ScopeKind {
    pub const ALL: &'static [Self] = &[
        Self::Cell,
        Self::Algorithm,
        Self::Half,
        Self::Clause,
        Self::Backend,
        Self::Item,
        Self::Resolution,
        Self::Kind,
        Self::Family,
        Self::Mechanism,
        Self::Proofkind,
        Self::Morphology,
    ];

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Cell => "cell",
            Self::Algorithm => "algorithm",
            Self::Half => "half",
            Self::Clause => "clause",
            Self::Backend => "backend",
            Self::Item => "item",
            Self::Resolution => "resolution",
            Self::Kind => "kind",
            Self::Family => "family",
            Self::Mechanism => "mechanism",
            Self::Proofkind => "proofkind",
            Self::Morphology => "morphology",
        }
    }

    #[must_use]
    pub fn parse(raw: &str) -> Option<Self> {
        match raw {
            "cell" => Some(Self::Cell),
            "algorithm" => Some(Self::Algorithm),
            "half" => Some(Self::Half),
            "clause" => Some(Self::Clause),
            "backend" => Some(Self::Backend),
            "item" => Some(Self::Item),
            "resolution" => Some(Self::Resolution),
            "kind" => Some(Self::Kind),
            "family" => Some(Self::Family),
            "mechanism" => Some(Self::Mechanism),
            "proofkind" => Some(Self::Proofkind),
            "morphology" => Some(Self::Morphology),
            _ => None,
        }
    }

    /// Validate a `<kind>:<value>` scope string against this closed scope kind.
    ///
    /// - `backend`: value must be one of `batch-cpu`, `gpu-cuda`, `gpu-metal`.
    /// - `morphology` (Review Note #3): value must be `wheeled` or `legged`.
    /// - `cell` (Review Note #8): accepts `cell:<stratum>` or `cell:<stratum>:<regime>`.
    /// - `family`: response query column family; explicitly refuses device family names (`cuda`, `metal`).
    #[must_use]
    pub fn validates_scope(self, scope: &str) -> bool {
        let Some((prefix, rest)) = scope.split_once(':') else {
            return false;
        };
        if prefix != self.as_str() {
            return false;
        }
        let val = rest.trim();
        if val.is_empty() || val != rest {
            return false;
        }
        match self {
            Self::Backend => matches!(val, "batch-cpu" | "gpu-cuda" | "gpu-metal"),
            Self::Morphology => matches!(val, "wheeled" | "legged"),
            Self::Cell => {
                // Review Note #8: `cell:<stratum>` or `cell:<stratum>:<regime>`.
                let parts: Vec<&str> = val.split(':').collect();
                (parts.len() == 1 || parts.len() == 2)
                    && parts.iter().all(|p| {
                        !p.is_empty()
                            && p.bytes()
                                .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
                    })
            }
            Self::Family => {
                // `family` never names a device family (`cuda`, `metal`, `gpu-cuda`, `gpu-metal`).
                !matches!(val, "cuda" | "metal" | "gpu-cuda" | "gpu-metal")
                    && !val.contains(':')
                    && val
                        .bytes()
                        .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
            }
            Self::Half
            | Self::Clause
            | Self::Item
            | Self::Resolution
            | Self::Kind
            | Self::Algorithm
            | Self::Mechanism
            | Self::Proofkind => {
                !val.contains(':')
                    && val
                        .bytes()
                        .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
            }
        }
    }
}

impl fmt::Display for ScopeKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Optional reading suffix for a second entry for one scope at one exit (`N40-03` step 6).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DecisionReading {
    Final,
    Reopened,
}

impl DecisionReading {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Final => "final",
            Self::Reopened => "reopened",
        }
    }

    #[must_use]
    pub fn parse(raw: &str) -> Option<Self> {
        match raw {
            "final" => Some(Self::Final),
            "reopened" => Some(Self::Reopened),
            _ => None,
        }
    }
}

/// One recorded kill-gate decision entry (`N40-03` step 1 & step 6).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KillDecision {
    pub outcome: KillOutcome,
    pub adr: String,
    pub date: String,
    pub session: String,
    pub receipts: Vec<String>,
    /// Milestone at which this decision was taken (defaults to the row's `decided`).
    pub exit: Option<String>,
    /// Per-scope tag (`<scope_kind>:<value>`) required iff the row defines `scope_kind`.
    pub scope: Option<String>,
    /// Distinguishes a second entry for one scope at one exit (`final` or `reopened`).
    pub reading: Option<DecisionReading>,
    /// Required on `deferred` outcomes; must be strictly later than this entry's exit.
    pub deferred_to: Option<String>,
    /// Required non-empty explanation on `deferred` outcomes.
    pub reason: Option<String>,
    /// Optional form in force on `fired` outcomes; must occur verbatim in the row's `fallback`.
    pub fallback_form: Option<String>,
}

impl KillDecision {
    /// Effective exit milestone of this decision entry (explicit `exit` or row's `decided`).
    #[must_use]
    pub fn effective_exit<'a>(&'a self, row_decided: Option<&'a str>) -> Option<&'a str> {
        self.exit.as_deref().or(row_decided)
    }

    /// Canonical expected ADR path for this decision entry (`docs/adr/ADR-<gate>-<exit>[-<reading>].md`).
    #[must_use]
    pub fn expected_adr_path(&self, gate_id: &str, row_decided: Option<&str>) -> Option<String> {
        let exit = self.effective_exit(row_decided)?;
        Some(canonical_decision_adr_path(gate_id, exit, self.reading))
    }

    /// Deterministic canonical serialization used to detect in-place edits of earlier entries (`RC-57`).
    #[must_use]
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut s = String::new();
        s.push_str("outcome=");
        s.push_str(self.outcome.as_str());
        s.push_str("\nadr=");
        s.push_str(&self.adr);
        s.push_str("\ndate=");
        s.push_str(&self.date);
        s.push_str("\nsession=");
        s.push_str(&self.session);
        s.push_str("\nreceipts=");
        s.push_str(&self.receipts.join(","));
        s.push_str("\nexit=");
        s.push_str(self.exit.as_deref().unwrap_or(""));
        s.push_str("\nscope=");
        s.push_str(self.scope.as_deref().unwrap_or(""));
        s.push_str("\nreading=");
        s.push_str(self.reading.map_or("", DecisionReading::as_str));
        s.push_str("\ndeferred_to=");
        s.push_str(self.deferred_to.as_deref().unwrap_or(""));
        s.push_str("\nreason=");
        s.push_str(self.reason.as_deref().unwrap_or(""));
        s.push_str("\nfallback_form=");
        s.push_str(self.fallback_form.as_deref().unwrap_or(""));
        s.push('\n');
        s.into_bytes()
    }
}

/// Compute the canonical decision ADR path (`N40-03` step 6).
#[must_use]
pub fn canonical_decision_adr_path(
    gate_id: &str,
    exit_milestone: &str,
    reading: Option<DecisionReading>,
) -> String {
    match reading {
        Some(r) => format!(
            "{DECISION_ADR_DIR}/ADR-{gate_id}-{exit_milestone}-{}.md",
            r.as_str()
        ),
        None => format!("{DECISION_ADR_DIR}/ADR-{gate_id}-{exit_milestone}.md"),
    }
}

/// Closed set of fields amendable via `[[amendment]]` (`N40-03` step 4).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AmendmentField {
    Trigger,
    Threshold,
    Fallback,
    Decided,
    Reads,
    Claim,
    Introduce,
}

impl AmendmentField {
    pub const ALL: &'static [Self] = &[
        Self::Trigger,
        Self::Threshold,
        Self::Fallback,
        Self::Decided,
        Self::Reads,
        Self::Claim,
        Self::Introduce,
    ];

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Trigger => "trigger",
            Self::Threshold => "threshold",
            Self::Fallback => "fallback",
            Self::Decided => "decided",
            Self::Reads => "reads",
            Self::Claim => "claim",
            Self::Introduce => "introduce",
        }
    }

    #[must_use]
    pub fn parse(raw: &str) -> Option<Self> {
        match raw {
            "trigger" => Some(Self::Trigger),
            "threshold" => Some(Self::Threshold),
            "fallback" => Some(Self::Fallback),
            "decided" => Some(Self::Decided),
            "reads" => Some(Self::Reads),
            "claim" => Some(Self::Claim),
            "introduce" => Some(Self::Introduce),
            _ => None,
        }
    }
}

/// One append-only `[[amendment]]` row in `registries/gates.toml` (`N40-03` step 4).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Amendment {
    pub id: String,
    pub gate: String,
    pub field: AmendmentField,
    pub old: String,
    pub new: String,
    pub reason: String,
    pub date: String,
    pub commit: String,
}

/// Seeded initial text of a kill row (or threshold of a read convergence gate) for amendment replay.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SeededKillRow {
    pub trigger: String,
    pub fallback: String,
    pub decided: Option<String>,
    pub mode: KillMode,
    pub reads: Vec<String>,
}

/// Context for kill-gate amendment replay, receipt ordering, implementer checks, and RC-57 immutability.
#[derive(Debug, Clone, Default)]
pub struct KillValidationContext {
    /// Seeded kill-row texts from the fallback ledgers (`gate_id -> SeededKillRow`).
    /// When `Some`, every kill row must replay cleanly from its seed or an `introduce` amendment.
    pub seeded_kill_rows: Option<BTreeMap<String, SeededKillRow>>,
    /// Seeded convergence-gate thresholds (`gate_id -> threshold`) for threshold amendment replay.
    pub seeded_thresholds: BTreeMap<String, String>,
    /// Earliest governing receipt timestamps by receipt hash (`hash -> ISO-8601 UTC timestamp`).
    pub receipt_timestamps: BTreeMap<String, String>,
    /// Earliest governing receipt timestamps directly by read gate ID (`gate_id -> ISO-8601 UTC timestamp`).
    pub gate_earliest_receipt_ts: BTreeMap<String, String>,
    /// Implementer sessions per gate (`gate_id -> comma-joined or set of implementer session IDs`).
    pub implementers_by_gate: BTreeMap<String, BTreeSet<String>>,
    /// Global implementer sessions (checked in addition to per-gate implementers).
    pub global_implementers: BTreeSet<String>,
    /// Previously committed decision entries per kill gate for immutability checks (`RC-57`).
    pub previous_decisions: BTreeMap<String, Vec<KillDecision>>,
    /// Whether to require `adr` files to exist on disk under `repo_root` (default: true when `repo_root.is_some()`).
    pub check_adr_files_exist: bool,
}

/// Status of one kill gate at a milestone exit in the due list (`N40-03` step 2 & step 3).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KillDueStatus {
    Decided {
        outcomes: Vec<KillOutcome>,
        scopes_decided: Vec<String>,
    },
    Deferred {
        deferred_to: String,
        reason: String,
    },
    DueWhenFundingRequested,
    Missing,
}

impl KillDueStatus {
    #[must_use]
    pub const fn is_missing(&self) -> bool {
        matches!(self, Self::Missing)
    }
}

/// One entry in the kill-gate due list for a milestone exit (`N40-03` -> read by `N40-04`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KillDueItem {
    pub gate_id: String,
    pub mode: KillMode,
    pub effective_decided: String,
    pub exit: String,
    pub status: KillDueStatus,
    pub scope_kind: Option<ScopeKind>,
    pub requires_reread_line_in_adr: bool,
}

/// Context for evaluating kill-gate due lists at a milestone exit.
#[derive(Debug, Clone, Default)]
pub struct KillDueContext {
    /// Kill gates whose `pilot-before-funding` work package has a recorded funding request.
    pub funding_requested: BTreeSet<String>,
}

/// Parse an `[[amendment]]` table using `Fields::fail` on every value rejection.
pub fn parse_amendment_table(tbl: Table, idx: usize) -> Result<Amendment, Vec<Violation>> {
    let label = tbl
        .get("id")
        .and_then(Value::as_str)
        .filter(|s| !s.trim().is_empty())
        .map_or_else(|| format!("amendment[{idx}]"), str::to_owned);
    let mut f = Fields::new(tbl, format!("amendment[{label}]"));
    let toml_err = |fld: &'static str, e: toml::TomlError| {
        vec![Violation::schema(
            "schema-invalid",
            Some(label.clone()),
            Some(fld),
            e.to_string(),
            "fix [[amendment]] field type or presence",
        )]
    };

    let id = f.require_str("id").map_err(|e| toml_err("id", e))?;
    if id.trim().is_empty() {
        return Err(f.fail(vec![Violation::schema(
            "schema-invalid",
            Some(label),
            Some("id"),
            "`amendment.id` must be non-empty",
            "assign a unique append-only amendment identifier (e.g. AMD-0001)",
        )]));
    }
    let gate = f.require_str("gate").map_err(|e| toml_err("gate", e))?;
    if gate.trim().is_empty() {
        return Err(f.fail(vec![Violation::schema(
            "schema-invalid",
            Some(id),
            Some("gate"),
            "`amendment.gate` must be non-empty",
            "name the amended gate identifier",
        )]));
    }
    let field_raw = f.require_str("field").map_err(|e| toml_err("field", e))?;
    let Some(field) = AmendmentField::parse(&field_raw) else {
        return Err(f.fail(vec![Violation::schema(
            "invalid-amendment-field",
            Some(gate),
            Some("field"),
            format!(
                "unknown amendment field `{field_raw}` (expected: trigger, threshold, fallback, decided, reads, claim, introduce)"
            ),
            "use one of the seven closed amendment field names",
        )]));
    };
    let old = f.require_str("old").map_err(|e| toml_err("old", e))?;
    let new = f.require_str("new").map_err(|e| toml_err("new", e))?;
    let reason = f.require_str("reason").map_err(|e| toml_err("reason", e))?;
    if reason.trim().is_empty() {
        return Err(f.fail(vec![Violation::schema(
            "schema-invalid",
            Some(gate),
            Some("reason"),
            format!("amendment `{id}` requires a non-empty `reason`"),
            "state why the amendment was made before measurement",
        )]));
    }
    let date = f.require_str("date").map_err(|e| toml_err("date", e))?;
    if parse_iso_timestamp_key(&date).is_none() {
        return Err(f.fail(vec![Violation::schema(
            "invalid-amendment-date",
            Some(gate),
            Some("date"),
            format!("amendment `{id}` date `{date}` is not a valid ISO-8601 date/timestamp"),
            "use ISO-8601 UTC timestamp such as 2027-02-03T10:00:00Z",
        )]));
    }
    let commit = f.require_str("commit").map_err(|e| toml_err("commit", e))?;
    if commit.trim().is_empty() {
        return Err(f.fail(vec![Violation::schema(
            "schema-invalid",
            Some(gate),
            Some("commit"),
            format!("amendment `{id}` requires a non-empty `commit` hash"),
            "record the commit hash that introduced this amendment",
        )]));
    }

    if let Err(e) = f.finish() {
        return Err(vec![Violation::schema(
            "unknown-field",
            Some(gate),
            None,
            e.to_string(),
            "remove unknown fields from [[amendment]]",
        )]);
    }

    Ok(Amendment {
        id,
        gate,
        field,
        old,
        new,
        reason,
        date,
        commit,
    })
}

/// Parse a `decision` value on a kill row: either a single table or an array of tables.
///
/// An all-empty placeholder table (`outcome = ""`, `adr = ""`, `date = ""`, `session = ""`,
/// `receipts = []`, no optional keys) is normalized to an empty decision list (`not-run`).
pub fn parse_decision_value(
    sp: Spanned<Value>,
    gate_id: &str,
) -> Result<(Vec<KillDecision>, bool), Vec<Violation>> {
    match sp.value {
        Value::Table(tbl) => {
            if is_empty_decision_placeholder(&tbl) {
                // Consume all fields cleanly to verify no unexpected keys on placeholder.
                let mut f = Fields::new(tbl, format!("{gate_id}.decision"));
                for key in ["outcome", "adr", "date", "session", "receipts"] {
                    let _ = f.take(key);
                }
                if let Err(e) = f.finish() {
                    return Err(vec![Violation::schema(
                        "unknown-field",
                        Some(gate_id.to_owned()),
                        Some("decision"),
                        e.to_string(),
                        "remove unknown fields from placeholder decision table",
                    )]);
                }
                return Ok((Vec::new(), false));
            }
            let entry = parse_decision_entry(tbl, gate_id, None)?;
            Ok((vec![entry], false))
        }
        Value::Array(arr) => {
            let mut entries = Vec::with_capacity(arr.len());
            for (didx, item) in arr.into_iter().enumerate() {
                let Value::Table(tbl) = item.value else {
                    return Err(vec![Violation::schema(
                        "schema-invalid",
                        Some(gate_id.to_owned()),
                        Some("decision"),
                        format!(
                            "decision[{didx}] expected table, found {}",
                            item.value.type_name()
                        ),
                        "encode each decision entry as a table",
                    )]);
                };
                entries.push(parse_decision_entry(tbl, gate_id, Some(didx))?);
            }
            Ok((entries, true))
        }
        other => Err(vec![Violation::schema(
            "schema-invalid",
            Some(gate_id.to_owned()),
            Some("decision"),
            format!(
                "`decision` must be a table or array of tables, found {}",
                other.type_name()
            ),
            "encode `decision` as `{ outcome, adr, date, session, receipts }` or an array of such tables",
        )]),
    }
}

fn is_empty_decision_placeholder(tbl: &Table) -> bool {
    let outcome_empty = tbl.get("outcome").and_then(Value::as_str) == Some("");
    let adr_empty = tbl.get("adr").and_then(Value::as_str).unwrap_or("").is_empty();
    let date_empty = tbl.get("date").and_then(Value::as_str).unwrap_or("").is_empty();
    let session_empty = tbl.get("session").and_then(Value::as_str).unwrap_or("").is_empty();
    let receipts_empty = tbl
        .get("receipts")
        .and_then(Value::as_array)
        .is_none_or(<[_]>::is_empty);
    let no_extras = !tbl.iter().any(|(k, _)| {
        !matches!(
            k.as_str(),
            "outcome" | "adr" | "date" | "session" | "receipts"
        )
    });
    outcome_empty && adr_empty && date_empty && session_empty && receipts_empty && no_extras
}

fn parse_decision_entry(
    tbl: Table,
    gate_id: &str,
    didx: Option<usize>,
) -> Result<KillDecision, Vec<Violation>> {
    let ctx = match didx {
        Some(i) => format!("{gate_id}.decision[{i}]"),
        None => format!("{gate_id}.decision"),
    };
    let mut f = Fields::new(tbl, ctx);
    let toml_err = |fld: &'static str, e: toml::TomlError| {
        vec![Violation::schema(
            "schema-invalid",
            Some(gate_id.to_owned()),
            Some(fld),
            e.to_string(),
            "fix decision entry field type or presence",
        )]
    };

    let outcome_raw = f
        .require_str("outcome")
        .map_err(|e| toml_err("decision.outcome", e))?;
    let Some(outcome) = KillOutcome::parse(&outcome_raw) else {
        return Err(f.fail(vec![Violation::schema(
            "invalid-outcome",
            Some(gate_id.to_owned()),
            Some("decision.outcome"),
            format!(
                "unknown kill-gate decision outcome `{outcome_raw}` (expected: not-fired, fired, not-reached, deferred)"
            ),
            "use one of the four closed outcomes: not-fired, fired, not-reached, deferred",
        )]));
    };

    let adr = f
        .require_str("adr")
        .map_err(|e| toml_err("decision.adr", e))?;
    let date = f
        .require_str("date")
        .map_err(|e| toml_err("decision.date", e))?;
    if parse_iso_timestamp_key(&date).is_none() {
        return Err(f.fail(vec![Violation::schema(
            "schema-invalid",
            Some(gate_id.to_owned()),
            Some("decision.date"),
            format!("invalid ISO decision date `{date}`"),
            "supply an ISO-8601 date (YYYY-MM-DD or YYYY-MM-DDTHH:MM:SSZ)",
        )]));
    }
    let session = f
        .require_str("session")
        .map_err(|e| toml_err("decision.session", e))?;
    if session.trim().is_empty() {
        return Err(f.fail(vec![Violation::from_harness(
            HarnessReason::SelfCertified,
            Some(gate_id.to_owned()),
            Some("decision.session"),
            "decision entry requires a non-empty recording `session`",
            "record the fresh-context session that took the decision",
        )]));
    }

    let receipts_raw = f
        .opt_array("receipts")
        .map_err(|e| toml_err("decision.receipts", e))?
        .unwrap_or_default();
    let mut receipts = Vec::with_capacity(receipts_raw.len());
    for sp in receipts_raw {
        match sp.value {
            Value::Str(s) => receipts.push(s),
            other => {
                return Err(f.fail(vec![Violation::schema(
                    "schema-invalid",
                    Some(gate_id.to_owned()),
                    Some("decision.receipts"),
                    format!(
                        "`decision.receipts` elements must be strings, got {}",
                        other.type_name()
                    ),
                    "list 64-hex receipt hashes as strings",
                )]));
            }
        }
    }

    let exit = f
        .opt_str("exit")
        .map_err(|e| toml_err("decision.exit", e))?;
    if let Some(ref ex) = exit
        && parse_milestone_ord(ex).is_none()
    {
        return Err(f.fail(vec![Violation::schema(
            "invalid-milestone",
            Some(gate_id.to_owned()),
            Some("decision.exit"),
            format!("invalid decision `exit` milestone `{ex}` (expected M0..M8)"),
            "use M0..M8",
        )]));
    }

    let scope = f
        .opt_str("scope")
        .map_err(|e| toml_err("decision.scope", e))?;

    let reading = match f
        .opt_str("reading")
        .map_err(|e| toml_err("decision.reading", e))?
    {
        Some(raw) => match DecisionReading::parse(&raw) {
            Some(r) => Some(r),
            None => {
                return Err(f.fail(vec![Violation::schema(
                    "decision-reading-invalid",
                    Some(gate_id.to_owned()),
                    Some("decision.reading"),
                    format!(
                        "invalid decision `reading` `{raw}` (expected `final` or `reopened`)"
                    ),
                    "set reading to `final` or `reopened` on a second entry at one exit",
                )]));
            }
        },
        None => None,
    };

    let deferred_to = f
        .opt_str("deferred_to")
        .map_err(|e| toml_err("decision.deferred_to", e))?;
    let reason = f
        .opt_str("reason")
        .map_err(|e| toml_err("decision.reason", e))?;
    let fallback_form = f
        .opt_str("fallback_form")
        .map_err(|e| toml_err("decision.fallback_form", e))?;

    if let Err(e) = f.finish() {
        return Err(vec![Violation::schema(
            "unknown-field",
            Some(gate_id.to_owned()),
            Some("decision"),
            e.to_string(),
            "remove unknown fields from decision entry",
        )]);
    }

    Ok(KillDecision {
        outcome,
        adr,
        date,
        session,
        receipts,
        exit,
        scope,
        reading,
        deferred_to,
        reason,
        fallback_form,
    })
}

/// Validate all kill-gate rows and `[[amendment]]` records on `register` under `ctx` (`N40-03`).
pub fn validate_kill_register(
    register: &Register,
    ctx: &ValidationContext,
    out: &mut Vec<Violation>,
) {
    // 1. Append-only unique amendment IDs.
    let mut seen_amd = BTreeSet::new();
    for amd in &register.amendments {
        if !seen_amd.insert(amd.id.as_str()) {
            out.push(Violation::schema(
                "duplicate-amendment-id",
                Some(amd.gate.clone()),
                Some("amendment.id"),
                format!("duplicate amendment identifier `{}`", amd.id),
                "amendment IDs are append-only and unique",
            ));
        }
    }

    // Group amendments by gate ID in file order.
    let mut amds_by_gate: BTreeMap<&str, Vec<&Amendment>> = BTreeMap::new();
    for amd in &register.amendments {
        amds_by_gate.entry(amd.gate.as_str()).or_default().push(amd);
    }

    // Collect every convergence/measured gate referenced in any kill gate's `reads`.
    let mut read_measured_gates: BTreeSet<&str> = BTreeSet::new();
    for g in &register.gates {
        if g.is_kill() {
            for r in &g.kill.reads {
                read_measured_gates.insert(r.as_str());
            }
        }
    }

    // 2. Validate every kill row's fields, reread ordering, decisions, scopes, ADRs, receipts,
    //    immutability (RC-57), and amendments.
    for g in &register.gates {
        if !g.is_kill() {
            continue;
        }
        validate_single_kill_row(
            g,
            register,
            amds_by_gate.get(g.id.as_str()).map(Vec::as_slice).unwrap_or(&[]),
            ctx,
            out,
        );
    }

    // 3. Validate threshold amendments on any convergence gate named in `reads`.
    for &gid in &read_measured_gates {
        let Some(gate) = register.get(gid) else {
            continue;
        };
        if gate.is_kill() {
            continue;
        }
        let amds = amds_by_gate.get(gid).map(Vec::as_slice).unwrap_or(&[]);
        let threshold_amds: Vec<&Amendment> = amds
            .iter()
            .copied()
            .filter(|a| a.field == AmendmentField::Threshold)
            .collect();
        if threshold_amds.is_empty() && !ctx.kill.seeded_thresholds.contains_key(gid) {
            continue;
        }
        let earliest_ts = earliest_receipt_ts_for_gate(gate, ctx);
        for amd in &threshold_amds {
            if let Some(ref rts) = earliest_ts {
                check_amendment_precedes(amd, rts, out);
            }
        }
        if let Some(seed_thr) = ctx.kill.seeded_thresholds.get(gid) {
            let mut cur = seed_thr.clone();
            let mut ok = true;
            for amd in &threshold_amds {
                if amd.old != cur {
                    out.push(Violation::schema(
                        "amendment-old-mismatch",
                        Some(gid.to_owned()),
                        Some("threshold"),
                        format!(
                            "amendment `{}` `old` (`{}`) does not match current threshold (`{cur}`)",
                            amd.id, amd.old
                        ),
                        "amendment `old` must be byte-equal to the row before the amendment",
                    ));
                    ok = false;
                    break;
                }
                cur = amd.new.clone();
            }
            if ok && gate.threshold.as_deref() != Some(cur.as_str()) {
                out.push(Violation::schema(
                    "changed-without-amendment",
                    Some(gid.to_owned()),
                    Some("threshold"),
                    format!(
                        "threshold of `{gid}` (`{}`) differs from replayed text (`{cur}`)",
                        gate.threshold.as_deref().unwrap_or("")
                    ),
                    "record a dated [[amendment]] before measurement for every threshold change",
                ));
            }
        }
    }
}

fn validate_single_kill_row(
    g: &GateRow,
    register: &Register,
    amds: &[&Amendment],
    ctx: &ValidationContext,
    out: &mut Vec<Violation>,
) {
    let kf = &g.kill;

    // Trigger and fallback must be present and non-empty.
    if kf.trigger.as_deref().map_or(true, |s| s.trim().is_empty()) {
        out.push(Violation::schema(
            "kill-row-missing-field",
            Some(g.id.clone()),
            Some("trigger"),
            format!("kill row `{}` requires a non-empty `trigger`", g.id),
            "specify the pre-registered trigger condition before any measurement",
        ));
    }
    if kf.fallback.as_deref().map_or(true, |s| s.trim().is_empty()) {
        out.push(Violation::schema(
            "kill-row-missing-field",
            Some(g.id.clone()),
            Some("fallback"),
            format!("kill row `{}` requires a non-empty `fallback`", g.id),
            "specify the fallback plan before any measurement",
        ));
    }

    let eff_decided = effective_decided_milestone(g, register);

    // `reread` exits must each be strictly later than `decided`.
    if let Some(ref dec_m) = eff_decided
        && let Some(dec_ord) = parse_milestone_ord(dec_m)
    {
        for rr in &kf.reread {
            match parse_milestone_ord(rr) {
                None => out.push(Violation::schema(
                    "invalid-reread",
                    Some(g.id.clone()),
                    Some("reread"),
                    format!("invalid `reread` milestone `{rr}` (expected M0..M8)"),
                    "list valid M0..M8 milestones strictly after `decided`",
                )),
                Some(rr_ord) if rr_ord <= dec_ord => out.push(Violation::schema(
                    "invalid-reread",
                    Some(g.id.clone()),
                    Some("reread"),
                    format!(
                        "`reread` milestone `{rr}` is at or before `decided` (`{dec_m}`)"
                    ),
                    "every `reread` milestone must be strictly later than `decided`",
                )),
                Some(_) => {}
            }
        }
    }

    // RC-57: earlier committed decision entries are append-only and never edited in place.
    if let Some(prev_entries) = ctx.kill.previous_decisions.get(&g.id) {
        if kf.decisions.len() < prev_entries.len() {
            out.push(Violation::schema(
                "decision-entry-edited",
                Some(g.id.clone()),
                Some("decision"),
                format!(
                    "earlier decision entries on `{}` were removed (had {}, now {})",
                    g.id,
                    prev_entries.len(),
                    kf.decisions.len()
                ),
                "keep all earlier decision entries verbatim and append new entries",
            ));
        } else {
            for (idx, prev) in prev_entries.iter().enumerate() {
                if kf.decisions[idx].canonical_bytes() != prev.canonical_bytes() {
                    out.push(Violation::schema(
                        "decision-entry-edited",
                        Some(g.id.clone()),
                        Some("decision"),
                        format!(
                            "earlier decision entry[{idx}] on `{}` was modified in place (RC-57)",
                            g.id
                        ),
                        "never edit an earlier decision entry; append a new entry with a later exit or distinct reading",
                    ));
                }
            }
        }
    }

    // Collect implementer sessions for this kill gate (explicit context + read gates' implemented_by).
    let implementers = collect_implementers_for_kill_gate(g, register, ctx);

    // Validate each decision entry and check `(exit, scope, reading)` uniqueness.
    let mut seen_keys: BTreeSet<(String, Option<String>, Option<DecisionReading>)> =
        BTreeSet::new();
    let fallback_text = kf.fallback.as_deref().unwrap_or("");

    for (didx, entry) in kf.decisions.iter().enumerate() {
        let entry_exit = entry.effective_exit(eff_decided.as_deref());

        // 1. Scope kind enforcement (`decision-scope-invalid`, RC-68).
        match (kf.scope_kind, entry.scope.as_deref()) {
            (None, Some(sc)) => out.push(Violation::schema(
                "decision-scope-invalid",
                Some(g.id.clone()),
                Some("decision.scope"),
                format!(
                    "decision[{didx}] specifies scope `{sc}` on kill row `{}` which has no `scope_kind`",
                    g.id
                ),
                "remove `scope` or declare the row's closed `scope_kind`",
            )),
            (Some(sk), None) => out.push(Violation::schema(
                "decision-scope-invalid",
                Some(g.id.clone()),
                Some("decision.scope"),
                format!(
                    "decision[{didx}] is missing required `scope` for `scope_kind = \"{sk}\"`"
                ),
                "supply `<scope_kind>:<value>` matching the row's `scope_kind`",
            )),
            (Some(sk), Some(sc)) => {
                if !sk.validates_scope(sc) {
                    out.push(Violation::schema(
                        "decision-scope-invalid",
                        Some(g.id.clone()),
                        Some("decision.scope"),
                        format!(
                            "decision[{didx}] scope `{sc}` is invalid for `scope_kind = \"{sk}\"`"
                        ),
                        "use `<scope_kind>:<value>` from the closed scope-kind table",
                    ));
                }
            }
            (None, None) => {}
        }

        // 2. Duplicate `(exit, scope, reading)` detection (`decision-entry-duplicate`, RC-69).
        if let Some(ex) = entry_exit {
            let key = (ex.to_owned(), entry.scope.clone(), entry.reading);
            if !seen_keys.insert(key) {
                out.push(Violation::schema(
                    "decision-entry-duplicate",
                    Some(g.id.clone()),
                    Some("decision"),
                    format!(
                        "duplicate decision entry for exit `{ex}`, scope {:?}, reading {:?}",
                        entry.scope, entry.reading
                    ),
                    "a second entry for one scope at one exit requires a distinct `reading` (`final` or `reopened`)",
                ));
            }
        }

        // 3. ADR path check (`decision-adr-path`): strictly under `docs/adr/`, canonical name,
        //    and file committed on disk when `repo_root` is provided.
        let expected_adr = entry.expected_adr_path(&g.id, eff_decided.as_deref());
        let path_matches = expected_adr
            .as_deref()
            .is_some_and(|exp| entry.adr == exp);
        if !path_matches || !entry.adr.starts_with("docs/adr/") {
            out.push(Violation::schema(
                "decision-adr-path",
                Some(g.id.clone()),
                Some("decision.adr"),
                format!(
                    "decision[{didx}] ADR path `{}` is invalid (expected `{}`)",
                    entry.adr,
                    expected_adr.as_deref().unwrap_or("docs/adr/ADR-<gate>-<milestone>.md")
                ),
                "store decision ADRs under docs/adr/ADR-<gate>-<milestone>[-<reading>].md",
            ));
        } else if let Some(ref root) = ctx.repo_root
            && ctx.kill.check_adr_files_exist
            && !root.join(&entry.adr).is_file()
        {
            out.push(Violation::schema(
                "decision-adr-path",
                Some(g.id.clone()),
                Some("decision.adr"),
                format!(
                    "decision[{didx}] ADR `{}` does not name a committed file under `{}`",
                    entry.adr,
                    root.display()
                ),
                "commit the decision ADR file at docs/adr/ADR-<gate>-<milestone>[-<reading>].md",
            ));
        }

        // 4. Implementer-session separation (`self-certified`, INV-GAUNT-05).
        let sess = entry.session.trim();
        if implementers.contains(sess) {
            out.push(Violation::from_harness(
                HarnessReason::SelfCertified,
                Some(g.id.clone()),
                Some("decision.session"),
                format!(
                    "decision[{didx}] session `{sess}` is listed as an implementer of `{}`",
                    g.id
                ),
                "record the decision ADR from a fresh-context session that did not implement the mechanism",
            ));
        }

        // 5. Per-outcome rules.
        match entry.outcome {
            KillOutcome::Deferred => {
                let reason_ok = entry
                    .reason
                    .as_deref()
                    .is_some_and(|r| !r.trim().is_empty());
                if !reason_ok {
                    out.push(Violation::schema(
                        "decision-deferred-invalid",
                        Some(g.id.clone()),
                        Some("decision.reason"),
                        format!("decision[{didx}] `deferred` requires a non-empty `reason`"),
                        "state why the decision is deferred in `reason`",
                    ));
                }
                match (entry.deferred_to.as_deref(), entry_exit) {
                    (None, _) => out.push(Violation::schema(
                        "decision-deferred-invalid",
                        Some(g.id.clone()),
                        Some("decision.deferred_to"),
                        format!("decision[{didx}] `deferred` requires `deferred_to`"),
                        "set `deferred_to` to a milestone strictly later than the entry's exit",
                    )),
                    (Some(dto), Some(ex)) => {
                        let dto_ord = parse_milestone_ord(dto);
                        let ex_ord = parse_milestone_ord(ex);
                        if dto_ord.is_none() || ex_ord.is_none() || dto_ord <= ex_ord {
                            out.push(Violation::schema(
                                "decision-deferred-invalid",
                                Some(g.id.clone()),
                                Some("decision.deferred_to"),
                                format!(
                                    "decision[{didx}] `deferred_to` (`{dto}`) must be a valid milestone strictly later than exit (`{ex}`)"
                                ),
                                "set `deferred_to` strictly later than the decision exit",
                            ));
                        }
                    }
                    (Some(dto), None) => {
                        if parse_milestone_ord(dto).is_none() {
                            out.push(Violation::schema(
                                "decision-deferred-invalid",
                                Some(g.id.clone()),
                                Some("decision.deferred_to"),
                                format!("decision[{didx}] invalid `deferred_to` milestone `{dto}`"),
                                "use M0..M8",
                            ));
                        }
                    }
                }
            }
            KillOutcome::NotFired => {
                // `not-fired` requires at least one resolving receipt (RC-13).
                if entry.receipts.is_empty() {
                    out.push(Violation::from_harness(
                        HarnessReason::EvidenceUnresolved,
                        Some(g.id.clone()),
                        Some("decision.receipts"),
                        format!(
                            "decision[{didx}] `not-fired` requires at least one cited receipt hash (RC-13)"
                        ),
                        "cite the receipt hash(es) under receipts/gates/ or receipts/pilots/ that establish the trigger was not met",
                    ));
                }
                if entry.deferred_to.is_some() {
                    out.push(Violation::schema(
                        "decision-deferred-invalid",
                        Some(g.id.clone()),
                        Some("decision.deferred_to"),
                        format!("decision[{didx}] `deferred_to` is valid only on `deferred` entries"),
                        "remove `deferred_to` from non-deferred decision entries",
                    ));
                }
            }
            KillOutcome::Fired | KillOutcome::NotReached => {
                if entry.deferred_to.is_some() {
                    out.push(Violation::schema(
                        "decision-deferred-invalid",
                        Some(g.id.clone()),
                        Some("decision.deferred_to"),
                        format!("decision[{didx}] `deferred_to` is valid only on `deferred` entries"),
                        "remove `deferred_to` from non-deferred decision entries",
                    ));
                }
            }
        }

        // 6. `fallback_form` check: valid only on `fired` and must occur verbatim in `fallback`.
        if let Some(ref form) = entry.fallback_form {
            if entry.outcome != KillOutcome::Fired {
                out.push(Violation::schema(
                    "decision-fallback-form-invalid",
                    Some(g.id.clone()),
                    Some("fallback_form"),
                    format!(
                        "decision[{didx}] `fallback_form` is valid only on `fired` entries, found outcome `{}`",
                        entry.outcome
                    ),
                    "remove `fallback_form` from non-fired decision entries",
                ));
            } else if form.trim().is_empty() || !fallback_text.contains(form.as_str()) {
                out.push(Violation::schema(
                    "decision-fallback-form-invalid",
                    Some(g.id.clone()),
                    Some("fallback_form"),
                    format!(
                        "decision[{didx}] `fallback_form` (`{form}`) does not occur verbatim in `fallback`"
                    ),
                    "set `fallback_form` to a verbatim substring of the row's `fallback` text",
                ));
            }
        }

        // 7. Receipt resolution check for every cited receipt hash.
        for rhash in &entry.receipts {
            verify_kill_decision_receipt(&g.id, didx, kf.mode, rhash, ctx, out);
        }
    }

    // 8. Amendment ordering (`amended-after-measurement`) and replay (`changed-without-amendment`,
    //    `amendment-old-mismatch`, `introduce` seeding).
    validate_kill_amendments(g, register, amds, ctx, out);
}

fn verify_kill_decision_receipt(
    gate_id: &str,
    didx: usize,
    mode: KillMode,
    raw_hash: &str,
    ctx: &ValidationContext,
    out: &mut Vec<Violation>,
) {
    let hex = raw_hash.strip_prefix("b3:").unwrap_or(raw_hash);
    if Blake3::from_hex(hex).is_err() {
        out.push(Violation::from_harness(
            HarnessReason::EvidenceUnresolved,
            Some(gate_id.to_owned()),
            Some("decision.receipts"),
            format!(
                "decision[{didx}] receipt `{raw_hash}` is not a 64 lower-case hex BLAKE3 digest"
            ),
            "cite a valid 64-hex BLAKE3 receipt hash",
        ));
        return;
    }

    let Some(ref root) = ctx.repo_root else {
        return;
    };

    let candidate_dirs: &[&str] = if mode == KillMode::PilotBeforeFunding {
        &["receipts/pilots", "receipts/gates"]
    } else {
        &["receipts/gates", "receipts/pilots"]
    };

    let mut found_path = None;
    for dir in candidate_dirs {
        let p = root.join(dir).join(format!("{hex}.json"));
        if p.is_file() {
            found_path = Some((dir, p));
            break;
        }
    }

    let Some((dir, path)) = found_path else {
        out.push(Violation::from_harness(
            HarnessReason::EvidenceUnresolved,
            Some(gate_id.to_owned()),
            Some("decision.receipts"),
            format!(
                "decision[{didx}] receipt `{hex}` does not resolve under receipts/gates/ or receipts/pilots/"
            ),
            "commit the referenced receipt file under receipts/gates/<hash>.json or receipts/pilots/<hash>.json",
        ));
        return;
    };

    match fs::read(&path) {
        Ok(bytes) => {
            let actual = hash_blob(&bytes).to_hex();
            if actual != hex {
                out.push(Violation::from_harness(
                    HarnessReason::EvidenceUnresolved,
                    Some(gate_id.to_owned()),
                    Some("decision.receipts"),
                    format!(
                        "decision[{didx}] receipt `{dir}/{hex}.json` hashed to `{actual}` instead of `{hex}`"
                    ),
                    "commit unmodified receipt bytes whose BLAKE3 hash matches the filename",
                ));
            }
        }
        Err(e) => {
            out.push(Violation::from_harness(
                HarnessReason::EvidenceUnresolved,
                Some(gate_id.to_owned()),
                Some("decision.receipts"),
                format!("decision[{didx}] failed reading `{dir}/{hex}.json`: {e}"),
                "ensure the committed receipt file is readable",
            ));
        }
    }
}

fn validate_kill_amendments(
    g: &GateRow,
    register: &Register,
    amds: &[&Amendment],
    ctx: &ValidationContext,
    out: &mut Vec<Violation>,
) {
    // Determine earliest governing receipt timestamp across all gates in `reads`.
    let earliest_read_ts = earliest_receipt_ts_for_kill_reads(g, register, ctx);

    // Check `amendment_precedes_measurement` for every amendment on this kill row.
    for amd in amds {
        if let Some(ref rts) = earliest_read_ts {
            check_amendment_precedes(amd, rts, out);
        }
    }

    // Determine initial seed state (from ledger seeds or an `introduce` amendment).
    let intro_amd = amds.iter().copied().find(|a| a.field == AmendmentField::Introduce);
    let seed_state: Option<SeededKillRow> = match &ctx.kill.seeded_kill_rows {
        Some(seed_map) => {
            if let Some(seeded) = seed_map.get(&g.id) {
                Some(seeded.clone())
            } else if let Some(intro) = intro_amd {
                if !intro.old.is_empty() {
                    out.push(Violation::schema(
                        "amendment-old-mismatch",
                        Some(g.id.clone()),
                        Some("amendment.old"),
                        format!(
                            "`introduce` amendment `{}` must have `old = \"\"`, found `{}`",
                            intro.id, intro.old
                        ),
                        "set `old = \"\"` on `introduce` amendments",
                    ));
                    None
                } else {
                    match parse_introduce_inline_table(&intro.new, &g.id, &intro.id) {
                        Ok(s) => Some(s),
                        Err(mut errs) => {
                            out.append(&mut errs);
                            None
                        }
                    }
                }
            } else {
                // RC-70: kill row in no ledger seed and with no `introduce` amendment.
                out.push(Violation::schema(
                    "changed-without-amendment",
                    Some(g.id.clone()),
                    Some("amendment"),
                    format!(
                        "kill row `{}` has neither a ledger seed nor an `introduce` amendment (RC-70)",
                        g.id
                    ),
                    "enter unseeded kill rows via an `introduce` amendment dated before measurement",
                ));
                None
            }
        }
        None => {
            // Even without a full ledger seed map, if an `introduce` amendment is present,
            // validate and replay from it!
            if let Some(intro) = intro_amd {
                if !intro.old.is_empty() {
                    out.push(Violation::schema(
                        "amendment-old-mismatch",
                        Some(g.id.clone()),
                        Some("amendment.old"),
                        format!(
                            "`introduce` amendment `{}` must have `old = \"\"`, found `{}`",
                            intro.id, intro.old
                        ),
                        "set `old = \"\"` on `introduce` amendments",
                    ));
                    None
                } else {
                    match parse_introduce_inline_table(&intro.new, &g.id, &intro.id) {
                        Ok(s) => Some(s),
                        Err(mut errs) => {
                            out.append(&mut errs);
                            None
                        }
                    }
                }
            } else {
                None
            }
        }
    };

    let Some(mut replayed) = seed_state else {
        return;
    };

    // Replay all non-introduce amendments in append order.
    let mut replay_ok = true;
    for amd in amds {
        if amd.field == AmendmentField::Introduce {
            continue;
        }
        match amd.field {
            AmendmentField::Trigger => {
                if amd.old != replayed.trigger {
                    out.push(Violation::schema(
                        "amendment-old-mismatch",
                        Some(g.id.clone()),
                        Some("trigger"),
                        format!(
                            "amendment `{}` `old` (`{}`) does not match current `trigger` (`{}`)",
                            amd.id, amd.old, replayed.trigger
                        ),
                        "set amendment `old` byte-equal to the row's text before this amendment",
                    ));
                    replay_ok = false;
                    break;
                }
                replayed.trigger = amd.new.clone();
            }
            AmendmentField::Fallback => {
                if amd.old != replayed.fallback {
                    out.push(Violation::schema(
                        "amendment-old-mismatch",
                        Some(g.id.clone()),
                        Some("fallback"),
                        format!(
                            "amendment `{}` `old` (`{}`) does not match current `fallback` (`{}`)",
                            amd.id, amd.old, replayed.fallback
                        ),
                        "set amendment `old` byte-equal to the row's text before this amendment",
                    ));
                    replay_ok = false;
                    break;
                }
                replayed.fallback = amd.new.clone();
            }
            AmendmentField::Decided => {
                let cur_dec = replayed.decided.as_deref().unwrap_or("");
                if amd.old != cur_dec {
                    out.push(Violation::schema(
                        "amendment-old-mismatch",
                        Some(g.id.clone()),
                        Some("decided"),
                        format!(
                            "amendment `{}` `old` (`{}`) does not match current `decided` (`{cur_dec}`)",
                            amd.id, amd.old
                        ),
                        "set amendment `old` byte-equal to the prior `decided` milestone",
                    ));
                    replay_ok = false;
                    break;
                }
                replayed.decided = if amd.new.is_empty() {
                    None
                } else {
                    Some(amd.new.clone())
                };
            }
            AmendmentField::Reads => {
                let cur_reads = format_reads_value(&replayed.reads);
                if amd.old != cur_reads {
                    out.push(Violation::schema(
                        "amendment-old-mismatch",
                        Some(g.id.clone()),
                        Some("reads"),
                        format!(
                            "amendment `{}` `old` (`{}`) does not match current `reads` (`{cur_reads}`)",
                            amd.id, amd.old
                        ),
                        "set amendment `old` byte-equal to the prior `reads` list",
                    ));
                    replay_ok = false;
                    break;
                }
                match parse_reads_amendment_str(&amd.new) {
                    Some(parsed) => replayed.reads = parsed,
                    None => {
                        out.push(Violation::schema(
                            "schema-invalid",
                            Some(g.id.clone()),
                            Some("reads"),
                            format!(
                                "amendment `{}` `new` reads `{}` is not a valid TOML string array",
                                amd.id, amd.new
                            ),
                            "encode `reads` amendments as TOML string arrays (e.g. `[\"GATE-A\"]`)",
                        ));
                        replay_ok = false;
                        break;
                    }
                }
            }
            AmendmentField::Threshold | AmendmentField::Claim | AmendmentField::Introduce => {}
        }
    }

    if !replay_ok {
        return;
    }

    // Compare replayed state against the row's live fields (`changed-without-amendment`, RC-11).
    let actual_trigger = g.kill.trigger.as_deref().unwrap_or("");
    if actual_trigger != replayed.trigger {
        out.push(Violation::schema(
            "changed-without-amendment",
            Some(g.id.clone()),
            Some("trigger"),
            format!(
                "kill row `{}` `trigger` (`{actual_trigger}`) differs from replayed text (`{}`)",
                g.id, replayed.trigger
            ),
            "record a dated [[amendment]] before measurement whenever changing `trigger`",
        ));
    }
    let actual_fallback = g.kill.fallback.as_deref().unwrap_or("");
    if actual_fallback != replayed.fallback {
        out.push(Violation::schema(
            "changed-without-amendment",
            Some(g.id.clone()),
            Some("fallback"),
            format!(
                "kill row `{}` `fallback` (`{actual_fallback}`) differs from replayed text (`{}`)",
                g.id, replayed.fallback
            ),
            "record a dated [[amendment]] before measurement whenever changing `fallback`",
        ));
    }
    if g.kill.decided != replayed.decided {
        out.push(Violation::schema(
            "changed-without-amendment",
            Some(g.id.clone()),
            Some("decided"),
            format!(
                "kill row `{}` `decided` ({:?}) differs from replayed value ({:?})",
                g.id, g.kill.decided, replayed.decided
            ),
            "record a dated [[amendment]] before measurement whenever changing `decided`",
        ));
    }
    if g.kill.reads != replayed.reads {
        out.push(Violation::schema(
            "changed-without-amendment",
            Some(g.id.clone()),
            Some("reads"),
            format!(
                "kill row `{}` `reads` ({:?}) differs from replayed value ({:?})",
                g.id, g.kill.reads, replayed.reads
            ),
            "record a dated [[amendment]] before measurement whenever changing `reads`",
        ));
    }
}

/// Format an `introduce` amendment's canonical TOML inline table in the exact key order
/// mandated by `N40-03` step 4: `trigger`, `fallback`, `decided`, `mode`, `reads`.
#[must_use]
pub fn format_introduce_inline_table(seed: &SeededKillRow) -> String {
    let mut out = String::from("{ trigger = ");
    push_toml_str(&mut out, &seed.trigger);
    out.push_str(", fallback = ");
    push_toml_str(&mut out, &seed.fallback);
    out.push_str(", decided = ");
    push_toml_str(&mut out, seed.decided.as_deref().unwrap_or(""));
    out.push_str(", mode = ");
    push_toml_str(&mut out, seed.mode.as_str());
    out.push_str(", reads = ");
    out.push_str(&format_reads_value(&seed.reads));
    out.push_str(" }");
    out
}

/// Parse an `introduce` amendment's `new` inline table (`trigger`, `fallback`, `decided`, `mode`, `reads`).
pub fn parse_introduce_inline_table(
    raw_inline: &str,
    gate_id: &str,
    amd_id: &str,
) -> Result<SeededKillRow, Vec<Violation>> {
    // Verify canonical key order: trigger, fallback, decided, mode, reads.
    let expected_order = ["trigger", "fallback", "decided", "mode", "reads"];
    let mut prev_pos = 0usize;
    for key in expected_order {
        let needle = format!("{key}");
        let Some(rel) = raw_inline[prev_pos..].find(&needle) else {
            return Err(vec![Violation::schema(
                "schema-invalid",
                Some(gate_id.to_owned()),
                Some("amendment.new"),
                format!(
                    "`introduce` amendment `{amd_id}` must list keys in exact order `trigger, fallback, decided, mode, reads` (missing `{key}`)"
                ),
                "format `introduce` `new` as `{ trigger = ..., fallback = ..., decided = ..., mode = ..., reads = [...] }`",
            )]);
        };
        prev_pos += rel + needle.len();
    }

    let doc_src = format!("seed = {raw_inline}\n");
    let root = toml::parse(doc_src.as_bytes()).map_err(|e| {
        vec![Violation::schema(
            "schema-invalid",
            Some(gate_id.to_owned()),
            Some("amendment.new"),
            format!("`introduce` amendment `{amd_id}` inline table failed TOML parse: {e}"),
            "encode `introduce` `new` as a valid TOML inline table",
        )]
    })?;
    let mut top = Fields::new(root, format!("introduce[{amd_id}]"));
    let tbl = top.require_table("seed").map_err(|e| {
        vec![Violation::schema(
            "schema-invalid",
            Some(gate_id.to_owned()),
            Some("amendment.new"),
            e.to_string(),
            "encode `introduce` `new` as an inline table",
        )]
    })?;
    if let Err(e) = top.finish() {
        return Err(vec![Violation::schema(
            "schema-invalid",
            Some(gate_id.to_owned()),
            Some("amendment.new"),
            e.to_string(),
            "encode `introduce` `new` as a single inline table",
        )]);
    }

    let mut f = Fields::new(tbl, format!("introduce[{amd_id}].seed"));
    let merr = |fld: &'static str, e: toml::TomlError| {
        vec![Violation::schema(
            "schema-invalid",
            Some(gate_id.to_owned()),
            Some(fld),
            format!("`introduce` amendment `{amd_id}`: {e}"),
            "supply trigger, fallback, decided, mode, reads in `introduce` inline table",
        )]
    };

    let trigger = f.require_str("trigger").map_err(|e| merr("trigger", e))?;
    let fallback = f.require_str("fallback").map_err(|e| merr("fallback", e))?;
    let decided_raw = f.require_str("decided").map_err(|e| merr("decided", e))?;
    let decided = if decided_raw.is_empty() {
        None
    } else if parse_milestone_ord(&decided_raw).is_some() {
        Some(decided_raw)
    } else {
        return Err(f.fail(vec![Violation::schema(
            "invalid-milestone",
            Some(gate_id.to_owned()),
            Some("decided"),
            format!("`introduce` amendment `{amd_id}` invalid decided `{decided_raw}`"),
            "use M0..M8",
        )]));
    };
    let mode_raw = f.require_str("mode").map_err(|e| merr("mode", e))?;
    let Some(mode) = KillMode::parse(&mode_raw) else {
        return Err(f.fail(vec![Violation::schema(
            "invalid-kill-mode",
            Some(gate_id.to_owned()),
            Some("mode"),
            format!("`introduce` amendment `{amd_id}` unknown mode `{mode_raw}`"),
            "use exit, pilot-before-funding, standing, or rolling",
        )]));
    };
    let reads_arr = f.require_array("reads").map_err(|e| merr("reads", e))?;
    let mut reads = Vec::with_capacity(reads_arr.len());
    for sp in reads_arr {
        match sp.value {
            Value::Str(s) => reads.push(s),
            other => {
                return Err(f.fail(vec![Violation::schema(
                    "schema-invalid",
                    Some(gate_id.to_owned()),
                    Some("reads"),
                    format!(
                        "`introduce` reads must be strings, found {}",
                        other.type_name()
                    ),
                    "encode reads as string array",
                )]));
            }
        }
    }
    if let Err(e) = f.finish() {
        return Err(vec![Violation::schema(
            "unknown-field",
            Some(gate_id.to_owned()),
            Some("amendment.new"),
            e.to_string(),
            "include only trigger, fallback, decided, mode, reads in `introduce`",
        )]);
    }

    Ok(SeededKillRow {
        trigger,
        fallback,
        decided,
        mode,
        reads,
    })
}

fn format_reads_value(reads: &[String]) -> String {
    let mut out = String::from("[");
    for (i, r) in reads.iter().enumerate() {
        if i > 0 {
            out.push_str(", ");
        }
        push_toml_str(&mut out, r);
    }
    out.push(']');
    out
}

fn parse_reads_amendment_str(raw: &str) -> Option<Vec<String>> {
    let doc = format!("reads = {raw}\n");
    let root = toml::parse(doc.as_bytes()).ok()?;
    let mut top = Fields::new(root, "amendment.reads");
    let arr = top.require_array("reads").ok()?;
    top.finish().ok()?;
    let mut out = Vec::with_capacity(arr.len());
    for sp in arr {
        match sp.value {
            Value::Str(s) => out.push(s),
            _ => return None,
        }
    }
    Some(out)
}

fn check_amendment_precedes(amd: &Amendment, earliest_receipt_ts: &str, out: &mut Vec<Violation>) {
    let Some(amd_key) = parse_iso_timestamp_key(&amd.date) else {
        return;
    };
    let Some(rcpt_key) = parse_iso_timestamp_key(earliest_receipt_ts) else {
        return;
    };
    if amd_key >= rcpt_key {
        out.push(Violation::schema(
            "amended-after-measurement",
            Some(amd.gate.clone()),
            Some("amendment.date"),
            format!(
                "amendment `{}` dated `{}` is not earlier than earliest governing receipt `{earliest_receipt_ts}` (RC-10)",
                amd.id, amd.date
            ),
            "triggers, thresholds, and fallbacks may only be amended before the measurement they govern",
        ));
    }
}

fn earliest_receipt_ts_for_kill_reads(
    g: &GateRow,
    register: &Register,
    ctx: &ValidationContext,
) -> Option<String> {
    let mut earliest: Option<(String, String)> = None;
    let mut consider = |ts: &str| {
        if let Some(key) = parse_iso_timestamp_key(ts)
            && earliest.as_ref().is_none_or(|(ek, _)| key < *ek)
        {
            earliest = Some((key, ts.to_owned()));
        }
    };

    for read_id in &g.kill.reads {
        if let Some(ts) = ctx.kill.gate_earliest_receipt_ts.get(read_id) {
            consider(ts);
        }
        if let Some(read_gate) = register.get(read_id)
            && let Some(ts) = earliest_receipt_ts_for_gate(read_gate, ctx)
        {
            consider(&ts);
        }
    }
    earliest.map(|(_, raw)| raw)
}

fn earliest_receipt_ts_for_gate(gate: &GateRow, ctx: &ValidationContext) -> Option<String> {
    let mut earliest: Option<(String, String)> = None;
    let mut consider = |ts: &str| {
        if let Some(key) = parse_iso_timestamp_key(ts)
            && earliest.as_ref().is_none_or(|(ek, _)| key < *ek)
        {
            earliest = Some((key, ts.to_owned()));
        }
    };

    if let Some(ts) = ctx.kill.gate_earliest_receipt_ts.get(&gate.id) {
        consider(ts);
    }
    for part in &gate.parts {
        let hex = part.receipt.strip_prefix("b3:").unwrap_or(&part.receipt);
        if !hex.is_empty() {
            if let Some(ts) = ctx.kill.receipt_timestamps.get(hex) {
                consider(ts);
            } else if let Some(ts) = ctx.kill.receipt_timestamps.get(&part.receipt) {
                consider(ts);
            }
        }
        if !part.recorded.date.trim().is_empty() && !part.receipt.trim().is_empty() {
            consider(&part.recorded.date);
        }
    }
    earliest.map(|(_, raw)| raw)
}

fn collect_implementers_for_kill_gate(
    g: &GateRow,
    register: &Register,
    ctx: &ValidationContext,
) -> BTreeSet<String> {
    let mut set: BTreeSet<String> = ctx.kill.global_implementers.clone();
    if let Some(per_gate) = ctx.kill.implementers_by_gate.get(&g.id) {
        for s in per_gate {
            set.insert(s.clone());
        }
    }
    for read_id in &g.kill.reads {
        if let Some(per_read) = ctx.kill.implementers_by_gate.get(read_id) {
            for s in per_read {
                set.insert(s.clone());
            }
        }
        if let Some(read_gate) = register.get(read_id) {
            for part in &read_gate.parts {
                for tok in part.recorded.implemented_by.split(',') {
                    let t = tok.trim();
                    if !t.is_empty() {
                        set.insert(t.to_owned());
                    }
                }
            }
        }
    }
    set
}

/// Parse an ISO-8601 date (`YYYY-MM-DD`) or UTC timestamp (`YYYY-MM-DDTHH:MM[:SS][Z]`)
/// into a canonical lexicographically comparable 19-char UTC key `YYYY-MM-DDTHH:MM:SS`.
#[must_use]
pub fn parse_iso_timestamp_key(raw: &str) -> Option<String> {
    let s = raw.trim();
    if s.len() < 10 {
        return None;
    }
    let b = s.as_bytes();
    let is_d = |i: usize| b.get(i).is_some_and(u8::is_ascii_digit);
    if !(is_d(0)
        && is_d(1)
        && is_d(2)
        && is_d(3)
        && b.get(4) == Some(&b'-')
        && is_d(5)
        && is_d(6)
        && b.get(7) == Some(&b'-')
        && is_d(8)
        && is_d(9))
    {
        return None;
    }
    let month: u32 = s[5..7].parse().ok()?;
    let day: u32 = s[8..10].parse().ok()?;
    if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
        return None;
    }
    let date_part = &s[0..10];
    if s.len() == 10 {
        return Some(format!("{date_part}T00:00:00"));
    }
    if b.get(10) != Some(&b'T') {
        return None;
    }
    let time_raw = s[11..].strip_suffix('Z').unwrap_or(&s[11..]);
    let time_parts: Vec<&str> = time_raw.split(':').collect();
    if time_parts.len() < 2 || time_parts.len() > 3 {
        return None;
    }
    let hh: u32 = time_parts[0].parse().ok()?;
    let mm: u32 = time_parts[1].parse().ok()?;
    let ss_raw = time_parts.get(2).copied().unwrap_or("00");
    let ss_int = ss_raw.split('.').next().unwrap_or("");
    let ss: u32 = ss_int.parse().ok()?;
    if hh > 23 || mm > 59 || ss > 59 {
        return None;
    }
    Some(format!("{date_part}T{hh:02}:{mm:02}:{ss:02}"))
}

// ============================================================================
// Effective milestone, in-force decisions, due lists, and `apply_decisions`
// ============================================================================

/// Resolve a kill row's effective `decided` milestone: its explicit `decided` field, or for a
/// late-subsection kill row omitting `decided`, the milestone of its neighbour gate (first via
/// `reads`, then via matching `section`).
#[must_use]
pub fn effective_decided_milestone(g: &GateRow, register: &Register) -> Option<String> {
    if let Some(ref d) = g.kill.decided {
        return Some(d.clone());
    }
    for read_id in &g.kill.reads {
        if let Some(rg) = register.get(read_id)
            && let Some(ref m) = rg.milestone
        {
            return Some(m.clone());
        }
    }
    for other in &register.gates {
        if !other.is_kill() && other.section == g.section {
            if let Some(ref m) = other.milestone {
                return Some(m.clone());
            }
        }
    }
    None
}

/// Return the decision entry currently in force per scope (key `None` for whole-row gates,
/// `Some(scope)` for per-scope gates). Earlier entries are kept in file order; the latest
/// entry per scope wins (by milestone order, then `reading`, then array index).
#[must_use]
pub fn decisions_in_force<'a>(
    g: &'a GateRow,
    register: &Register,
) -> BTreeMap<Option<String>, &'a KillDecision> {
    let eff_dec = effective_decided_milestone(g, register);
    let mut best: BTreeMap<Option<String>, (u32, u8, usize, &KillDecision)> = BTreeMap::new();

    for (idx, d) in g.kill.decisions.iter().enumerate() {
        let ord = d
            .effective_exit(eff_dec.as_deref())
            .and_then(parse_milestone_ord)
            .unwrap_or(0);
        let reading_rank = match d.reading {
            None => 0,
            Some(DecisionReading::Final) => 1,
            Some(DecisionReading::Reopened) => 2,
        };
        let key = d.scope.clone();
        let replace = match best.get(&key) {
            None => true,
            Some(&(bo, br, bi, _)) => (ord, reading_rank, idx) >= (bo, br, bi),
        };
        if replace {
            best.insert(key, (ord, reading_rank, idx, d));
        }
    }
    best.into_iter().map(|(k, (_, _, _, d))| (k, d)).collect()
}

/// Return the decision entry in force for a specific scope (or `None` for whole-row gates)
/// at or before `at_exit` (`M0..M8`).
#[must_use]
pub fn decisions_in_force_at<'a>(
    g: &'a GateRow,
    register: &Register,
    at_exit: &str,
) -> BTreeMap<Option<String>, &'a KillDecision> {
    let Some(at_ord) = parse_milestone_ord(at_exit) else {
        return BTreeMap::new();
    };
    let eff_dec = effective_decided_milestone(g, register);
    let mut best: BTreeMap<Option<String>, (u32, u8, usize, &'a KillDecision)> = BTreeMap::new();

    for (idx, d) in g.kill.decisions.iter().enumerate() {
        let Some(ord) = d
            .effective_exit(eff_dec.as_deref())
            .and_then(parse_milestone_ord)
        else {
            continue;
        };
        if ord > at_ord {
            continue;
        }
        let reading_rank = match d.reading {
            None => 0,
            Some(DecisionReading::Final) => 1,
            Some(DecisionReading::Reopened) => 2,
        };
        let key = d.scope.clone();
        let replace = match best.get(&key) {
            None => true,
            Some(&(bo, br, bi, _)) => (ord, reading_rank, idx) >= (bo, br, bi),
        };
        if replace {
            best.insert(key, (ord, reading_rank, idx, d));
        }
    }
    best.into_iter().map(|(k, (_, _, _, d))| (k, d)).collect()
}

/// Compute the effective fallback text for a `fired` decision entry.
///
/// When `entry.fallback_form` is set, returns it verbatim. Otherwise, if the row's fallback
/// names `"not at all"` as its withdrawal form (e.g. `KILL-PROXY`), returns `"not at all"`,
/// else returns the row's full `fallback` text.
#[must_use]
pub fn effective_fallback_text(row: &GateRow, entry: &KillDecision) -> String {
    if let Some(ref form) = entry.fallback_form {
        return form.clone();
    }
    let fb = row.kill.fallback.as_deref().unwrap_or("");
    if fb.contains("not at all") {
        return "not at all".to_owned();
    }
    fb.to_owned()
}

/// Derived printable state for a kill row (`not-run` until a decision is in force, then the
/// outcome in force, per scope where `scope_kind` is present).
#[must_use]
pub fn kill_row_derived_state(g: &GateRow, register: &Register) -> String {
    let in_force = decisions_in_force(g, register);
    if in_force.is_empty() {
        return "not-run".to_owned();
    }
    if g.kill.scope_kind.is_none()
        && let Some(d) = in_force.get(&None)
    {
        return d.outcome.as_str().to_owned();
    }
    let parts: Vec<String> = in_force
        .iter()
        .map(|(sc, d)| match sc {
            Some(s) => format!("{s}={}", d.outcome.as_str()),
            None => d.outcome.as_str().to_owned(),
        })
        .collect();
    parts.join(", ")
}

/// Compute the kill-gate due list at milestone `exit_milestone` (`N40-03` step 2 & step 3,
/// consumed by `N40-04` exit queries).
#[must_use]
pub fn due_kill_gates(
    register: &Register,
    exit_milestone: &str,
    ctx: &KillDueContext,
) -> Vec<KillDueItem> {
    let Some(exit_ord) = parse_milestone_ord(exit_milestone) else {
        return Vec::new();
    };
    let mut items = Vec::new();

    for g in &register.gates {
        if !g.is_kill() {
            continue;
        }
        let Some(eff_dec) = effective_decided_milestone(g, register) else {
            continue;
        };
        let Some(dec_ord) = parse_milestone_ord(&eff_dec) else {
            continue;
        };

        let mut scheduled = match g.kill.mode {
            KillMode::Exit => {
                exit_ord == dec_ord
                    || g.kill
                        .reread
                        .iter()
                        .filter_map(|r| parse_milestone_ord(r))
                        .any(|o| o == exit_ord)
            }
            KillMode::Rolling => {
                if g.kill.reread.is_empty() {
                    exit_ord >= dec_ord
                } else {
                    exit_ord == dec_ord
                        || g.kill
                            .reread
                            .iter()
                            .filter_map(|r| parse_milestone_ord(r))
                            .any(|o| o == exit_ord)
                }
            }
            KillMode::Standing => exit_ord >= dec_ord,
            KillMode::PilotBeforeFunding => {
                exit_ord == dec_ord
                    || (exit_ord > dec_ord && ctx.funding_requested.contains(&g.id))
            }
        };

        // Check whether a prior `deferred` entry moved this row into `exit_milestone`'s due list.
        let deferred_here_from_earlier = g.kill.decisions.iter().any(|d| {
            d.outcome == KillOutcome::Deferred
                && d.deferred_to.as_deref() == Some(exit_milestone)
        });
        if deferred_here_from_earlier {
            scheduled = true;
        }
        if !scheduled {
            continue;
        }

        // Gather entries recorded specifically at `exit_milestone`.
        let entries_at_exit: Vec<&KillDecision> = g
            .kill
            .decisions
            .iter()
            .filter(|d| d.effective_exit(Some(&eff_dec)) == Some(exit_milestone))
            .collect();

        let status = if !entries_at_exit.is_empty() {
            // Check latest entry per scope at this exit.
            let all_deferred = entries_at_exit
                .iter()
                .all(|d| d.outcome == KillOutcome::Deferred);
            if all_deferred {
                let last = entries_at_exit[entries_at_exit.len() - 1];
                KillDueStatus::Deferred {
                    deferred_to: last.deferred_to.clone().unwrap_or_default(),
                    reason: last.reason.clone().unwrap_or_default(),
                }
            } else {
                let outcomes: Vec<KillOutcome> =
                    entries_at_exit.iter().map(|d| d.outcome).collect();
                let scopes_decided: Vec<String> = entries_at_exit
                    .iter()
                    .filter_map(|d| d.scope.clone())
                    .collect();
                KillDueStatus::Decided {
                    outcomes,
                    scopes_decided,
                }
            }
        } else if g.kill.mode == KillMode::PilotBeforeFunding
            && !ctx.funding_requested.contains(&g.id)
        {
            KillDueStatus::DueWhenFundingRequested
        } else {
            KillDueStatus::Missing
        };

        let requires_reread_line_in_adr =
            g.kill.mode == KillMode::Rolling && exit_ord > dec_ord;

        items.push(KillDueItem {
            gate_id: g.id.clone(),
            mode: g.kill.mode,
            effective_decided: eff_dec,
            exit: exit_milestone.to_owned(),
            status,
            scope_kind: g.kill.scope_kind,
            requires_reread_line_in_adr,
        });
    }

    items
}

/// Propagate all in-force `fired` kill-gate decisions onto the register (`N40-03` step 7).
///
/// This is the single place propagation happens; every generated view (README table,
/// register printout, downstream exit report / capabilities view) reads its result.
///
/// When a kill gate's decision in force has outcome `fired`:
/// - Every gate in `reads` with a `fail` part transitions to `Status::Killed`, with
///   `kill_gate = Some(<kill id>)`, `fallback = Some(<effective fallback>)`, and
///   `detail = Some("<kill id>: <effective fallback> (<adr>)")`.
/// - When the kill gate is `deferred`, `reads` stay untouched (`not-run` stays `not-run`).
#[must_use]
pub fn apply_decisions(register: &Register) -> Register {
    let mut out = register.clone();

    // Collect fired propagations first to avoid simultaneous borrows.
    let mut propagations: Vec<(String, String, String, Vec<String>, Option<String>)> = Vec::new();
    for g in &register.gates {
        if !g.is_kill() {
            continue;
        }
        let in_force = decisions_in_force(g, register);
        for (scope, dec) in in_force {
            if dec.outcome == KillOutcome::Fired {
                let fb = effective_fallback_text(g, dec);
                propagations.push((g.id.clone(), fb, dec.adr.clone(), g.kill.reads.clone(), scope));
            }
        }
    }

    for (kill_id, fb, adr, reads, scope) in propagations {
        let scope_prefix = match scope {
            Some(ref sc) => format!(" [{sc}]"),
            None => String::new(),
        };
        let detail = format!("{kill_id}{scope_prefix}: {fb} ({adr})");
        for read_id in &reads {
            if let Some(target) = out.get_mut(read_id) {
                for part in &mut target.parts {
                    if part.status == Status::Fail {
                        let scope_matches = match (&scope, &part.scope) {
                            (Some(ks), Some(ps)) => ps == ks || ps.contains(ks.as_str()),
                            _ => true,
                        };
                        if scope_matches {
                            part.status = Status::Killed;
                            part.kill_gate = Some(kill_id.clone());
                            part.fallback = Some(fb.clone());
                            part.detail = Some(detail.clone());
                        }
                    }
                }
            }
        }
    }

    out
}

/// Render a deterministic Markdown README status table from `apply_decisions(register)` (`N40-03` step 7).
///
/// Fired kill gates are listed under their adopted fallback's name; killed read gates show
/// `killed: fallback adopted (<fallback>)` and never appear as passed or implemented.
#[must_use]
pub fn render_readme_status_table(register: &Register) -> String {
    let effective = apply_decisions(register);
    let mut out = String::from(
        "| Gate | Capability / Fallback | Status | Detail |\n\
         |---|---|---|---|\n",
    );
    for g in &effective.gates {
        if g.is_kill() {
            let in_force = decisions_in_force(g, &effective);
            if in_force.is_empty() {
                out.push_str(&format!("| `{}` | {} | not-run | — |\n", g.id, g.id));
            } else {
                for (sc, dec) in in_force {
                    let scope_suffix = sc
                        .as_deref()
                        .map_or_else(String::new, |s| format!(" ({s})"));
                    if dec.outcome == KillOutcome::Fired {
                        let fb = effective_fallback_text(g, dec);
                        out.push_str(&format!(
                            "| `{}`{} | {} | fired | fallback adopted ({}; {}) |\n",
                            g.id, scope_suffix, fb, fb, dec.adr
                        ));
                    } else {
                        out.push_str(&format!(
                            "| `{}`{} | {} | {} | {} |\n",
                            g.id,
                            scope_suffix,
                            g.id,
                            dec.outcome.as_str(),
                            dec.adr
                        ));
                    }
                }
            }
        } else {
            let status = g.derived_status();
            let printed = g.printed_status(None);
            let cap_col = if status == Status::Killed {
                g.parts
                    .iter()
                    .rev()
                    .find_map(|p| p.fallback.as_deref())
                    .unwrap_or(&g.id)
            } else {
                &g.id
            };
            let detail = g
                .parts
                .iter()
                .rev()
                .find_map(|p| p.detail.as_deref())
                .unwrap_or("—");
            out.push_str(&format!(
                "| `{}` | {} | {} | {} |\n",
                g.id, cap_col, printed, detail
            ));
        }
    }
    out
}

/// Render a deterministic human-readable register printout from `apply_decisions(register)` (`N40-03` step 7).
#[must_use]
pub fn render_register_printout(register: &Register) -> String {
    let effective = apply_decisions(register);
    let mut out = String::new();
    for g in &effective.gates {
        if g.is_kill() {
            let in_force = decisions_in_force(g, &effective);
            if in_force.is_empty() {
                out.push_str(&format!("{} [kill] status=not-run\n", g.id));
            } else {
                for (sc, dec) in in_force {
                    let scope_tag = sc
                        .as_deref()
                        .map_or_else(String::new, |s| format!(" scope={s}"));
                    if dec.outcome == KillOutcome::Fired {
                        let fb = effective_fallback_text(g, dec);
                        out.push_str(&format!(
                            "{} [kill]{} capability=\"{}\" status=fired adr={}\n",
                            g.id, scope_tag, fb, dec.adr
                        ));
                    } else {
                        out.push_str(&format!(
                            "{} [kill]{} status={} adr={}\n",
                            g.id,
                            scope_tag,
                            dec.outcome.as_str(),
                            dec.adr
                        ));
                    }
                }
            }
        } else {
            let status = g.derived_status();
            let printed = g.printed_status(None);
            if status == Status::Killed {
                let fb = g
                    .parts
                    .iter()
                    .rev()
                    .find_map(|p| p.fallback.as_deref())
                    .unwrap_or("");
                let detail = g
                    .parts
                    .iter()
                    .rev()
                    .find_map(|p| p.detail.as_deref())
                    .unwrap_or("");
                out.push_str(&format!(
                    "{} [{}] capability=\"{}\" status=\"{}\" detail=\"{}\"\n",
                    g.id,
                    g.effective_kind().as_str(),
                    fb,
                    printed,
                    detail
                ));
            } else {
                out.push_str(&format!(
                    "{} [{}] status=\"{}\"\n",
                    g.id,
                    g.effective_kind().as_str(),
                    printed
                ));
            }
        }
    }
    out
}

// ============================================================================
// Seeded text hash & `--live --ids <file>` close-time check (`N40-03` Acceptance 11)
// ============================================================================

/// Canonical BLAKE3 digest of a kill row's seeded `(trigger, fallback)` texts (`frob.kill-seed/1`).
#[must_use]
pub fn kill_row_seed_hash(trigger: &str, fallback: &str) -> String {
    let payload = format!("frob.kill-seed/1\ntrigger={trigger}\nfallback={fallback}\n");
    hash_blob(payload.as_bytes()).to_hex()
}

/// One expected kill-row specification read from a `--live --ids <file>` list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiveIdExpectation {
    pub id: String,
    /// Optional combined `kill_row_seed_hash(trigger, fallback)`.
    pub combined_hash: Option<String>,
    /// Optional separate `(trigger_hash, fallback_hash)` pair.
    pub split_hashes: Option<(String, String)>,
}

/// Parse a `--live --ids <file>` specification (ignoring blank lines and `#` comments).
///
/// Supported line forms:
/// - `<KILL-ID> <64-hex-combined-seed-hash>`
/// - `<KILL-ID> <64-hex-trigger-hash> <64-hex-fallback-hash>`
/// - `<KILL-ID>` (presence & completeness check only)
pub fn parse_live_ids_file(src: &str) -> Result<Vec<LiveIdExpectation>, String> {
    let mut out = Vec::new();
    for (idx, raw_line) in src.lines().enumerate() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let tokens: Vec<&str> = line.split_whitespace().collect();
        let id = tokens[0].to_owned();
        let clean_hex = |s: &str| s.strip_prefix("b3:").unwrap_or(s).to_owned();
        match tokens.len() {
            1 => out.push(LiveIdExpectation {
                id,
                combined_hash: None,
                split_hashes: None,
            }),
            2 => out.push(LiveIdExpectation {
                id,
                combined_hash: Some(clean_hex(tokens[1])),
                split_hashes: None,
            }),
            3 => out.push(LiveIdExpectation {
                id,
                combined_hash: None,
                split_hashes: Some((clean_hex(tokens[1]), clean_hex(tokens[2]))),
            }),
            _ => {
                return Err(format!(
                    "line {}: expected `<ID> [combined_hash | trigger_hash fallback_hash]`",
                    idx + 1
                ));
            }
        }
    }
    Ok(out)
}

/// Verify a parsed `Register` against a `--live --ids` list, returning the first failing
/// identifier and exact defect description (`absent`, `duplicated`, `incomplete`, or `text differs`).
///
/// Strictly read-only: writes nothing.
pub fn verify_live_kill_ids(
    register: &Register,
    expectations: &[LiveIdExpectation],
) -> Result<(), String> {
    for exp in expectations {
        let matches: Vec<&GateRow> = register
            .gates
            .iter()
            .filter(|g| g.id == exp.id)
            .collect();
        if matches.is_empty() {
            return Err(format!("{}: absent from register", exp.id));
        }
        if matches.len() > 1 {
            return Err(format!(
                "{}: duplicated ({} rows found)",
                exp.id,
                matches.len()
            ));
        }
        let row = matches[0];
        if !row.is_kill() {
            return Err(format!("{}: incomplete (kind is not `kill`)", exp.id));
        }
        if row.section.trim().is_empty() {
            return Err(format!("{}: incomplete (empty `section`)", exp.id));
        }
        let trigger = row.kill.trigger.as_deref().unwrap_or("");
        if trigger.trim().is_empty() {
            return Err(format!("{}: incomplete (missing or empty `trigger`)", exp.id));
        }
        let fallback = row.kill.fallback.as_deref().unwrap_or("");
        if fallback.trim().is_empty() {
            return Err(format!(
                "{}: incomplete (missing or empty `fallback`)",
                exp.id
            ));
        }
        if !row.kill.mode_explicit {
            return Err(format!("{}: incomplete (missing explicit `mode`)", exp.id));
        }
        if row.kill.decided.is_none() {
            return Err(format!("{}: incomplete (missing `decided` milestone)", exp.id));
        }

        if let Some(ref expected_hex) = exp.combined_hash {
            let actual_hex = kill_row_seed_hash(trigger, fallback);
            if actual_hex != *expected_hex {
                return Err(format!(
                    "{}: text differs (expected seed hash `{expected_hex}`, got `{actual_hex}`)",
                    exp.id
                ));
            }
        }
        if let Some((ref exp_trig, ref exp_fb)) = exp.split_hashes {
            let act_trig = hash_blob(trigger.as_bytes()).to_hex();
            if act_trig != *exp_trig {
                return Err(format!(
                    "{}: trigger text differs (expected `{exp_trig}`, got `{act_trig}`)",
                    exp.id
                ));
            }
            let act_fb = hash_blob(fallback.as_bytes()).to_hex();
            if act_fb != *exp_fb {
                return Err(format!(
                    "{}: fallback text differs (expected `{exp_fb}`, got `{act_fb}`)",
                    exp.id
                ));
            }
        }
    }
    Ok(())
}

/// Format a `KillDecision` as a TOML inline table for deterministic `print_toml` output.
pub(crate) fn push_decision_inline(out: &mut String, d: &KillDecision) {
    out.push_str("{ outcome = ");
    push_toml_str(out, d.outcome.as_str());
    out.push_str(", adr = ");
    push_toml_str(out, &d.adr);
    out.push_str(", date = ");
    push_toml_str(out, &d.date);
    out.push_str(", session = ");
    push_toml_str(out, &d.session);
    out.push_str(", receipts = [");
    for (i, r) in d.receipts.iter().enumerate() {
        if i > 0 {
            out.push_str(", ");
        }
        push_toml_str(out, r);
    }
    out.push(']');
    if let Some(ref ex) = d.exit {
        out.push_str(", exit = ");
        push_toml_str(out, ex);
    }
    if let Some(ref sc) = d.scope {
        out.push_str(", scope = ");
        push_toml_str(out, sc);
    }
    if let Some(rd) = d.reading {
        out.push_str(", reading = ");
        push_toml_str(out, rd.as_str());
    }
    if let Some(ref dto) = d.deferred_to {
        out.push_str(", deferred_to = ");
        push_toml_str(out, dto);
    }
    if let Some(ref rsn) = d.reason {
        out.push_str(", reason = ");
        push_toml_str(out, rsn);
    }
    if let Some(ref ff) = d.fallback_form {
        out.push_str(", fallback_form = ");
        push_toml_str(out, ff);
    }
    out.push_str(" }");
}

/// Format an `[[amendment]]` block for deterministic `print_toml` output.
pub(crate) fn push_amendment_toml(out: &mut String, a: &Amendment) {
    out.push_str("\n[[amendment]]\n");
    out.push_str("id = ");
    push_toml_str(out, &a.id);
    out.push('\n');
    out.push_str("gate = ");
    push_toml_str(out, &a.gate);
    out.push('\n');
    out.push_str("field = ");
    push_toml_str(out, a.field.as_str());
    out.push('\n');
    out.push_str("old = ");
    push_toml_str(out, &a.old);
    out.push('\n');
    out.push_str("new = ");
    push_toml_str(out, &a.new);
    out.push('\n');
    out.push_str("reason = ");
    push_toml_str(out, &a.reason);
    out.push('\n');
    out.push_str("date = ");
    push_toml_str(out, &a.date);
    out.push('\n');
    out.push_str("commit = ");
    push_toml_str(out, &a.commit);
    out.push('\n');
}

fn push_toml_str(out: &mut String, s: &str) {
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

/// Convenience helper for loading a committed ADR file under `repo_root` (used by tests & callers).
#[must_use]
pub fn adr_exists(repo_root: &Path, adr_rel: &str) -> bool {
    adr_rel.starts_with("docs/adr/") && repo_root.join(adr_rel).is_file()
}

// ============================================================================
// Unit tests (`cargo test -p frob-conformance register::kill::`)
// ============================================================================

#[cfg(test)]
mod tests {
    use std::env;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::*;
    use crate::register::validate::parse_and_validate_bytes;

    static CTR: AtomicU64 = AtomicU64::new(0);

    fn scratch_repo(label: &str) -> PathBuf {
        let base = env::var_os("BIG_SCRATCH")
            .or_else(|| env::var_os("TMPDIR"))
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("/tmp"));
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |d| d.as_secs());
        let n = CTR.fetch_add(1, Ordering::Relaxed);
        let dir = base
            .join("frob-BlackOx")
            .join(format!("kill-test-{label}-{ts}-{}-{n}", std::process::id()));
        fs::create_dir_all(&dir).expect("mkdir scratch_repo");
        dir
    }

    fn write_receipt(root: &Path, subdir: &str, body: &str) -> String {
        let hex = hash_blob(body.as_bytes()).to_hex();
        let dir = root.join(subdir);
        fs::create_dir_all(&dir).expect("mkdir receipt dir");
        fs::write(dir.join(format!("{hex}.json")), body.as_bytes()).expect("write receipt");
        hex
    }

    fn write_adr(root: &Path, rel: &str) {
        let p = root.join(rel);
        if let Some(parent) = p.parent() {
            fs::create_dir_all(parent).expect("mkdir adr parent");
        }
        fs::write(&p, format!("# {rel}\n\nSynthetic decision ADR.\n")).expect("write adr");
    }

    fn ctx_with_root(root: &Path) -> ValidationContext {
        let mut ctx = ValidationContext::default();
        ctx.repo_root = Some(root.to_path_buf());
        ctx.kill.check_adr_files_exist = true;
        ctx
    }

    #[test]
    fn kill_row_requires_trigger_and_fallback() {
        let bad = r#"schema = "frob.gates/1"

[[gate]]
id = "KILL-EMPTY-FB"
kind = "kill"
section = "21.24"
consumers = []
trigger = "GATE-SPARK-04 fails"
fallback = ""
decided = "M2"
mode = "exit"
reads = ["GATE-SPARK-04"]
"#;
        let errs = parse_and_validate_bytes(bad.as_bytes(), "empty-fb.toml", &ValidationContext::default())
            .expect_err("empty fallback must fail load");
        assert!(
            errs.iter().any(|v| v.gate_id.as_deref() == Some("KILL-EMPTY-FB")
                && v.field.as_deref() == Some("fallback")),
            "expected error naming row and fallback, got {errs:?}"
        );

        let bad_trig = bad.replace("trigger = \"GATE-SPARK-04 fails\"", "trigger = \"   \"")
            .replace("fallback = \"\"", "fallback = \"CPU fallback\"");
        let errs2 = parse_and_validate_bytes(
            bad_trig.as_bytes(),
            "empty-trig.toml",
            &ValidationContext::default(),
        )
        .expect_err("empty trigger must fail load");
        assert!(
            errs2.iter().any(|v| v.gate_id.as_deref() == Some("KILL-EMPTY-FB")
                && v.field.as_deref() == Some("trigger")),
            "expected error naming row and trigger, got {errs2:?}"
        );
    }

    #[test]
    fn amendment_replay() {
        let mut seeds = BTreeMap::new();
        seeds.insert(
            "KILL-R1".to_owned(),
            SeededKillRow {
                trigger: "below 0.5".to_owned(),
                fallback: "cpu fallback".to_owned(),
                decided: Some("M2".to_owned()),
                mode: KillMode::Exit,
                reads: vec!["GATE-X".to_owned()],
            },
        );
        let mut ctx = ValidationContext::default();
        ctx.kill.seeded_kill_rows = Some(seeds);

        let ok_toml = r#"schema = "frob.gates/1"

[[gate]]
id = "KILL-R1"
kind = "kill"
section = "21.24"
consumers = []
trigger = "below 0.45"
fallback = "cpu fallback"
decided = "M2"
mode = "exit"
reads = ["GATE-X"]

[[amendment]]
id = "AMD-0001"
gate = "KILL-R1"
field = "trigger"
old = "below 0.5"
new = "below 0.45"
reason = "tightened pre-measurement"
date = "2027-02-01T10:00:00Z"
commit = "1111111111111111111111111111111111111111"
"#;
        parse_and_validate_bytes(ok_toml.as_bytes(), "amd-ok.toml", &ctx)
            .expect("matching replayed trigger must pass");

        let drifted = ok_toml.replace("trigger = \"below 0.45\"", "trigger = \"below 0.4\"");
        let errs = parse_and_validate_bytes(drifted.as_bytes(), "amd-drift.toml", &ctx)
            .expect_err("unamended drift must fail");
        assert!(
            errs.iter().any(|v| v.reason == "changed-without-amendment"),
            "expected changed-without-amendment, got {errs:?}"
        );
    }

    #[test]
    fn amendment_after_measurement_refused() {
        let mut ctx = ValidationContext::default();
        ctx.kill
            .gate_earliest_receipt_ts
            .insert("GATE-X".to_owned(), "2027-02-03T12:00:00Z".to_owned());

        let base = |date: &str| {
            format!(
                r#"schema = "frob.gates/1"

[[gate]]
id = "KILL-TIME"
kind = "kill"
section = "21.24"
trigger = "below 0.45"
fallback = "cpu fallback"
decided = "M2"
mode = "exit"
reads = ["GATE-X"]

[[amendment]]
id = "AMD-0002"
gate = "KILL-TIME"
field = "trigger"
old = "below 0.5"
new = "below 0.45"
reason = "pre-measurement calibration"
date = "{date}"
commit = "2222222222222222222222222222222222222222"
"#
            )
        };

        let late = base("2027-02-03T13:00:00Z");
        let errs = parse_and_validate_bytes(late.as_bytes(), "late.toml", &ctx)
            .expect_err("amendment at 13:00Z after 12:00Z receipt must fail");
        assert!(
            errs.iter().any(|v| v.reason == "amended-after-measurement"),
            "expected amended-after-measurement, got {errs:?}"
        );

        let early = base("2027-02-03T11:00:00Z");
        parse_and_validate_bytes(early.as_bytes(), "early.toml", &ctx)
            .expect("amendment at 11:00Z before 12:00Z receipt must pass");
    }

    #[test]
    fn amendment_old_must_match() {
        let mut seeds = BTreeMap::new();
        seeds.insert(
            "KILL-OLD".to_owned(),
            SeededKillRow {
                trigger: "below 0.5".to_owned(),
                fallback: "cpu fallback".to_owned(),
                decided: Some("M2".to_owned()),
                mode: KillMode::Exit,
                reads: vec![],
            },
        );
        let mut ctx = ValidationContext::default();
        ctx.kill.seeded_kill_rows = Some(seeds);

        let bad = r#"schema = "frob.gates/1"

[[gate]]
id = "KILL-OLD"
kind = "kill"
section = "21.24"
trigger = "below 0.45"
fallback = "cpu fallback"
decided = "M2"
mode = "exit"
reads = []

[[amendment]]
id = "AMD-0003"
gate = "KILL-OLD"
field = "trigger"
old = "below 0.6"
new = "below 0.45"
reason = "wrong old text"
date = "2027-02-01T10:00:00Z"
commit = "3333333333333333333333333333333333333333"
"#;
        let errs = parse_and_validate_bytes(bad.as_bytes(), "bad-old.toml", &ctx)
            .expect_err("mismatched old must fail");
        assert!(
            errs.iter().any(|v| v.reason == "amendment-old-mismatch"),
            "expected amendment-old-mismatch, got {errs:?}"
        );
    }

    #[test]
    fn outcome_closed_set() {
        let root = scratch_repo("outcome-closed");
        let rhex = write_receipt(&root, "receipts/gates", "{\"gate\":\"ok\"}\n");
        write_adr(&root, "docs/adr/ADR-KILL-OUT-M2.md");
        let ctx = ctx_with_root(&root);

        for (outcome, extra) in [
            ("not-fired", format!(", receipts = [\"{rhex}\"]")),
            ("fired", ", receipts = []".to_owned()),
            ("not-reached", ", receipts = []".to_owned()),
            (
                "deferred",
                ", receipts = [], deferred_to = \"M3\", reason = \"second repair cycle\"".to_owned(),
            ),
        ] {
            let src = format!(
                r#"schema = "frob.gates/1"

[[gate]]
id = "KILL-OUT"
kind = "kill"
section = "21.24"
trigger = "t"
fallback = "fb"
decided = "M2"
mode = "exit"
reads = []
decision = {{ outcome = "{outcome}", adr = "docs/adr/ADR-KILL-OUT-M2.md", date = "2027-02-01", session = "reviewer"{extra} }}
"#
            );
            parse_and_validate_bytes(src.as_bytes(), "ok-outcome.toml", &ctx)
                .unwrap_or_else(|e| panic!("outcome `{outcome}` must load: {e:?}"));
        }

        for bad_outcome in ["waived", "passed"] {
            let src = format!(
                r#"schema = "frob.gates/1"

[[gate]]
id = "KILL-OUT"
kind = "kill"
section = "21.24"
trigger = "t"
fallback = "fb"
decided = "M2"
mode = "exit"
reads = []
decision = {{ outcome = "{bad_outcome}", adr = "docs/adr/ADR-KILL-OUT-M2.md", date = "2027-02-01", session = "reviewer", receipts = [] }}
"#
            );
            let errs = parse_and_validate_bytes(src.as_bytes(), "bad-outcome.toml", &ctx)
                .expect_err(&format!("outcome `{bad_outcome}` must be rejected"));
            assert!(
                errs.iter().any(|v| v.reason == "invalid-outcome"),
                "expected invalid-outcome for `{bad_outcome}`, got {errs:?}"
            );
        }
    }

    #[test]
    fn fired_propagates() {
        let root = scratch_repo("fired-prop");
        let rhex = write_receipt(&root, "receipts/gates", "{\"gate\":\"GATE-Y\",\"status\":\"fail\"}\n");
        write_adr(&root, "docs/adr/ADR-KILL-X-M2.md");
        let ctx = ctx_with_root(&root);

        let src = format!(
            r#"schema = "frob.gates/1"

[[gate]]
id = "GATE-Y"
kind = "convergence"
claim = "performance"
claim_id = "INV-Y"
section = "21.24"
threshold = "speed >= 2x"
milestone = "M2"
tier = ["G1:host"]
controls = ["NC-0001"]

[[gate.part]]
milestone = "M2"
status = "fail"
measured = "speed = 0.8x"
commit = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
command = "cargo test"
receipt = "{rhex}"
evidence = []
recorded = {{ date = "2027-02-01", session = "s2", implemented_by = "s1" }}

[[gate]]
id = "KILL-X"
kind = "kill"
section = "21.24"
trigger = "GATE-Y fails"
fallback = "batch-cpu training tier"
decided = "M2"
mode = "exit"
reads = ["GATE-Y"]
decision = {{ outcome = "fired", adr = "docs/adr/ADR-KILL-X-M2.md", date = "2027-02-02", session = "s3", receipts = ["{rhex}"] }}
"#
        );

        let reg = parse_and_validate_bytes(src.as_bytes(), "fired.toml", &ctx)
            .expect("valid register before propagation");
        let propagated = apply_decisions(&reg);
        let gy = propagated.get("GATE-Y").expect("GATE-Y");
        assert_eq!(gy.derived_status(), Status::Killed);
        let detail = gy.parts[0].detail.as_deref().unwrap_or("");
        assert!(detail.contains("KILL-X"), "detail must name KILL-X: {detail}");
        assert!(
            detail.contains("batch-cpu training tier"),
            "detail must name fallback: {detail}"
        );

        // Propagated register itself remains completely valid.
        let v_after = crate::register::validate::validate(&propagated, &ctx);
        assert!(v_after.is_empty(), "propagated register must validate cleanly: {v_after:?}");

        let readme = render_readme_status_table(&reg);
        let printout = render_register_printout(&reg);
        for view in [&readme, &printout] {
            assert!(
                view.contains("batch-cpu training tier"),
                "view must print fallback name:\n{view}"
            );
            assert!(
                !view.contains("| `GATE-Y` | GATE-Y | pass"),
                "killed gate must never appear as pass:\n{view}"
            );
        }
    }

    #[test]
    fn standing_gate_evaluated_every_exit() {
        let src = r#"schema = "frob.gates/1"

[[gate]]
id = "KILL-AUGUR-FALSECERT"
kind = "kill"
section = "25.9"
trigger = "any false certificate"
fallback = "withdraw certificate lane"
decided = "M1"
mode = "standing"
reads = []
"#;
        let reg = parse_and_validate_bytes(src.as_bytes(), "standing.toml", &ValidationContext::default())
            .expect("standing gate must load");
        let due_ctx = KillDueContext::default();
        assert!(due_kill_gates(&reg, "M0", &due_ctx).is_empty());
        for m in ["M1", "M2", "M3", "M4", "M5", "M6", "M7", "M8"] {
            let due = due_kill_gates(&reg, m, &due_ctx);
            assert_eq!(due.len(), 1, "standing gate must be due at {m}");
            assert_eq!(due[0].gate_id, "KILL-AUGUR-FALSECERT");
            assert!(due[0].status.is_missing());
        }
    }

    #[test]
    fn late_kill_takes_neighbour_milestone() {
        let src = r#"schema = "frob.gates/1"

[[gate]]
id = "GATE-NEIGHBOUR"
kind = "convergence"
claim = "parity"
claim_id = "INV-N"
section = "28.7"
threshold = "err < 1e-6"
milestone = "M4"
tier = ["G1:host"]
controls = ["NC-0001"]

[[gate]]
id = "KILL-LATE"
kind = "kill"
section = "28.7"
trigger = "GATE-NEIGHBOUR fails"
fallback = "dense solver"
mode = "exit"
reads = []
"#;
        let reg = parse_and_validate_bytes(src.as_bytes(), "late.toml", &ValidationContext::default())
            .expect("late kill row without decided must load");
        assert_eq!(
            effective_decided_milestone(reg.get("KILL-LATE").unwrap(), &reg).as_deref(),
            Some("M4")
        );
        let due_m4 = due_kill_gates(&reg, "M4", &KillDueContext::default());
        assert_eq!(due_m4.len(), 1);
        assert_eq!(due_m4[0].gate_id, "KILL-LATE");
        assert!(due_kill_gates(&reg, "M3", &KillDueContext::default()).is_empty());
    }

    #[test]
    fn adr_by_implementer_refused() {
        let root = scratch_repo("adr-impl");
        let rhex = write_receipt(&root, "receipts/gates", "{\"ok\":true}\n");
        write_adr(&root, "docs/adr/ADR-KILL-IMP-M2.md");
        let mut ctx = ctx_with_root(&root);
        ctx.kill
            .implementers_by_gate
            .entry("KILL-IMP".to_owned())
            .or_default()
            .insert("sess-author".to_owned());

        let src = format!(
            r#"schema = "frob.gates/1"

[[gate]]
id = "KILL-IMP"
kind = "kill"
section = "21.24"
trigger = "t"
fallback = "fb"
decided = "M2"
mode = "exit"
reads = []
decision = {{ outcome = "not-fired", adr = "docs/adr/ADR-KILL-IMP-M2.md", date = "2027-02-01", session = "sess-author", receipts = ["{rhex}"] }}
"#
        );
        let errs = parse_and_validate_bytes(src.as_bytes(), "impl.toml", &ctx)
            .expect_err("implementer recording decision ADR must be refused");
        assert!(
            errs.iter().any(|v| v.harness_reason == Some(HarnessReason::SelfCertified)),
            "expected self-certified refusal, got {errs:?}"
        );
    }

    #[test]
    fn deferred_needs_later_exit_and_reason() {
        let root = scratch_repo("deferred-rules");
        write_adr(&root, "docs/adr/ADR-KILL-DEF-M3.md");
        let ctx = ctx_with_root(&root);

        let make = |extra: &str| {
            format!(
                r#"schema = "frob.gates/1"

[[gate]]
id = "GATE-READ"
kind = "convergence"
claim = "parity"
claim_id = "INV-R"
section = "22.1"
threshold = "x < 1"
milestone = "M3"
tier = ["G1:host"]
controls = ["NC-0001"]

[[gate.part]]
milestone = "M3"
status = "not-run"
measured = ""
evidence = []
recorded = {{ date = "", session = "", implemented_by = "" }}

[[gate]]
id = "KILL-DEF"
kind = "kill"
section = "22.1"
trigger = "GATE-READ fails"
fallback = "fallback-def"
decided = "M3"
mode = "exit"
reads = ["GATE-READ"]
decision = {{ outcome = "deferred", adr = "docs/adr/ADR-KILL-DEF-M3.md", date = "2027-03-01", session = "reviewer", receipts = []{extra} }}
"#
            )
        };

        let ok = make(", deferred_to = \"M4\", reason = \"second repair cycle\"");
        let reg = parse_and_validate_bytes(ok.as_bytes(), "def-ok.toml", &ctx)
            .expect("valid deferred must load");
        let prop = apply_decisions(&reg);
        assert_eq!(prop.get("GATE-READ").unwrap().derived_status(), Status::NotRun);

        for bad_extra in [
            ", deferred_to = \"M3\", reason = \"same exit\"",
            ", deferred_to = \"M2\", reason = \"earlier exit\"",
            ", reason = \"missing deferred_to\"",
            ", deferred_to = \"M4\", reason = \"\"",
        ] {
            let bad = make(bad_extra);
            let errs = parse_and_validate_bytes(bad.as_bytes(), "def-bad.toml", &ctx)
                .expect_err(&format!("bad deferred `{bad_extra}` must fail"));
            assert!(
                errs.iter().any(|v| v.reason == "decision-deferred-invalid"),
                "expected decision-deferred-invalid for `{bad_extra}`, got {errs:?}"
            );
        }
    }

    #[test]
    fn not_fired_needs_receipt() {
        let root = scratch_repo("not-fired-rcpt");
        let pilot_hex = write_receipt(&root, "receipts/pilots", "{\"pilot\":\"PILOT-01\"}\n");
        write_adr(&root, "docs/adr/ADR-KILL-FLEET-M4.md");
        let ctx = ctx_with_root(&root);

        // Empty receipts -> refused (RC-13).
        let empty = r#"schema = "frob.gates/1"

[[gate]]
id = "KILL-FLEET"
kind = "kill"
section = "38.5"
trigger = "pilot fails"
fallback = "no fleet"
decided = "M4"
mode = "pilot-before-funding"
reads = []
decision = { outcome = "not-fired", adr = "docs/adr/ADR-KILL-FLEET-M4.md", date = "2027-04-01", session = "s2", receipts = [] }
"#;
        let errs_empty = parse_and_validate_bytes(empty.as_bytes(), "nf-empty.toml", &ctx)
            .expect_err("not-fired with empty receipts must fail (RC-13)");
        assert!(
            errs_empty.iter().any(|v| v.harness_reason == Some(HarnessReason::EvidenceUnresolved)),
            "got {errs_empty:?}"
        );

        // Non-existent hash -> refused.
        let missing_hex = "0".repeat(64);
        let missing = empty.replace("receipts = []", &format!("receipts = [\"{missing_hex}\"]"));
        let errs_missing = parse_and_validate_bytes(missing.as_bytes(), "nf-missing.toml", &ctx)
            .expect_err("uncommitted receipt hash must fail");
        assert!(
            errs_missing.iter().any(|v| v.harness_reason == Some(HarnessReason::EvidenceUnresolved)),
            "got {errs_missing:?}"
        );

        // Pilot receipt committed at `receipts/pilots/<hash>.json` -> accepted.
        let good = empty.replace("receipts = []", &format!("receipts = [\"{pilot_hex}\"]"));
        parse_and_validate_bytes(good.as_bytes(), "nf-pilot.toml", &ctx)
            .expect("pilot-before-funding not-fired citing receipts/pilots/<hash>.json must load");
    }

    #[test]
    fn pilot_mode_due_at_funding() {
        let src = r#"schema = "frob.gates/1"

[[gate]]
id = "KILL-MEGA"
kind = "kill"
section = "38.5"
trigger = "pilot fails"
fallback = "drop mega"
decided = "M4"
mode = "pilot-before-funding"
reads = []
"#;
        let reg = parse_and_validate_bytes(src.as_bytes(), "pilot-mode.toml", &ValidationContext::default())
            .expect("pilot-before-funding row must load");

        let unfunded = due_kill_gates(&reg, "M4", &KillDueContext::default());
        assert_eq!(unfunded.len(), 1);
        assert_eq!(unfunded[0].status, KillDueStatus::DueWhenFundingRequested);
        assert!(!unfunded[0].status.is_missing());

        let mut funded_ctx = KillDueContext::default();
        funded_ctx.funding_requested.insert("KILL-MEGA".to_owned());
        let funded = due_kill_gates(&reg, "M4", &funded_ctx);
        assert_eq!(funded.len(), 1);
        assert!(funded[0].status.is_missing());
    }

    #[test]
    fn rolling_reread_named_exits() {
        let src = r#"schema = "frob.gates/1"

[[gate]]
id = "KILL-MCP"
kind = "kill"
section = "32.5"
trigger = "second asupersync or MCP failure"
fallback = "CLI only"
decided = "M0"
mode = "rolling"
reread = ["M1"]
reads = []

[[gate]]
id = "KILL-TENFOLD-LEVER"
kind = "kill"
section = "39.10"
trigger = "removable share < 10%"
fallback = "unfunded"
decided = "M1"
mode = "exit"
reread = ["M2"]
reads = []
"#;
        let reg = parse_and_validate_bytes(src.as_bytes(), "reread.toml", &ValidationContext::default())
            .expect("reread rows must load");
        let dc = KillDueContext::default();
        let mcp_due: Vec<&str> = ["M0", "M1", "M2", "M3", "M4", "M5", "M6", "M7", "M8"]
            .into_iter()
            .filter(|m| due_kill_gates(&reg, m, &dc).iter().any(|i| i.gate_id == "KILL-MCP"))
            .collect();
        assert_eq!(mcp_due, vec!["M0", "M1"]);

        let lever_due: Vec<&str> = ["M0", "M1", "M2", "M3", "M4", "M5", "M6", "M7", "M8"]
            .into_iter()
            .filter(|m| {
                due_kill_gates(&reg, m, &dc)
                    .iter()
                    .any(|i| i.gate_id == "KILL-TENFOLD-LEVER")
            })
            .collect();
        assert_eq!(lever_due, vec!["M1", "M2"]);

        // `reread` naming an exit at or before `decided` fails load.
        let bad_rr = src.replace("reread = [\"M2\"]", "reread = [\"M1\"]");
        let errs = parse_and_validate_bytes(bad_rr.as_bytes(), "bad-rr.toml", &ValidationContext::default())
            .expect_err("reread <= decided must fail");
        assert!(errs.iter().any(|v| v.reason == "invalid-reread"));
    }

    #[test]
    fn decision_array() {
        let root = scratch_repo("decision-array");
        let rhex = write_receipt(&root, "receipts/gates", "{\"cell\":\"ok\"}\n");
        write_adr(&root, "docs/adr/ADR-KILL-TENFOLD-SPEED-M3.md");
        write_adr(&root, "docs/adr/ADR-KILL-TENFOLD-SPEED-M7.md");
        let ctx = ctx_with_root(&root);

        // Single decision table loads as 1-entry array with exit = decided.
        let single = format!(
            r#"schema = "frob.gates/1"

[[gate]]
id = "KILL-TENFOLD-SPEED"
kind = "kill"
section = "39.10"
trigger = "below 1.0x"
fallback = "descriptive cell"
decided = "M3"
mode = "exit"
reads = []
decision = {{ outcome = "not-fired", adr = "docs/adr/ADR-KILL-TENFOLD-SPEED-M3.md", date = "2027-03-01", session = "s2", receipts = ["{rhex}"] }}
"#
        );
        let reg1 = parse_and_validate_bytes(single.as_bytes(), "single.toml", &ctx)
            .expect("single decision table loads");
        let g1 = reg1.get("KILL-TENFOLD-SPEED").unwrap();
        assert_eq!(g1.kill.decisions.len(), 1);
        assert_eq!(
            g1.kill.decisions[0].effective_exit(g1.kill.decided.as_deref()),
            Some("M3")
        );

        // Array with two scopes at M3, then M7 supersedes `cell:wheeled` while keeping M3 entry.
        let arr = format!(
            r#"schema = "frob.gates/1"

[[gate]]
id = "KILL-TENFOLD-SPEED"
kind = "kill"
section = "39.10"
trigger = "below 1.0x"
fallback = "descriptive cell"
decided = "M3"
reread = ["M7"]
mode = "exit"
scope_kind = "cell"
reads = []
decision = [
  {{ exit = "M3", scope = "cell:wheeled", outcome = "fired", adr = "docs/adr/ADR-KILL-TENFOLD-SPEED-M3.md", date = "2027-03-01", session = "s2", receipts = ["{rhex}"] }},
  {{ exit = "M3", scope = "cell:tracked", outcome = "not-fired", adr = "docs/adr/ADR-KILL-TENFOLD-SPEED-M3.md", date = "2027-03-01", session = "s2", receipts = ["{rhex}"] }},
  {{ exit = "M7", scope = "cell:wheeled", outcome = "not-fired", adr = "docs/adr/ADR-KILL-TENFOLD-SPEED-M7.md", date = "2027-07-01", session = "s3", receipts = ["{rhex}"] }}
]
"#
        );
        let reg2 = parse_and_validate_bytes(arr.as_bytes(), "arr.toml", &ctx)
            .expect("scoped decision array loads");
        let g2 = reg2.get("KILL-TENFOLD-SPEED").unwrap();
        let m3_force = decisions_in_force_at(g2, &reg2, "M3");
        assert_eq!(m3_force[&Some("cell:wheeled".to_owned())].outcome, KillOutcome::Fired);
        assert_eq!(m3_force[&Some("cell:tracked".to_owned())].outcome, KillOutcome::NotFired);
        let m7_force = decisions_in_force(g2, &reg2);
        assert_eq!(m7_force[&Some("cell:wheeled".to_owned())].outcome, KillOutcome::NotFired);
        assert_eq!(g2.kill.decisions.len(), 3, "earlier M3 entry stays in file");

        // Scope on a row with no `scope_kind` fails load.
        let no_sk = arr.replace("scope_kind = \"cell\"\n", "");
        let errs_sk = parse_and_validate_bytes(no_sk.as_bytes(), "no-sk.toml", &ctx)
            .expect_err("scope without scope_kind must fail");
        assert!(errs_sk.iter().any(|v| v.reason == "decision-scope-invalid"));

        // RC-57: editing an earlier entry in place fails `decision-entry-edited`.
        let mut rc57_ctx = ctx.clone();
        rc57_ctx
            .kill
            .previous_decisions
            .insert("KILL-TENFOLD-SPEED".to_owned(), g2.kill.decisions.clone());
        let tampered = arr.replace("date = \"2027-03-01\", session = \"s2\"", "date = \"2027-03-02\", session = \"s2\"");
        let errs_rc57 = parse_and_validate_bytes(tampered.as_bytes(), "rc57.toml", &rc57_ctx)
            .expect_err("in-place edit of earlier decision must fail RC-57");
        assert!(
            errs_rc57.iter().any(|v| v.reason == "decision-entry-edited"),
            "expected decision-entry-edited, got {errs_rc57:?}"
        );
    }

    #[test]
    fn rolling_reread_every_exit() {
        let src = r#"schema = "frob.gates/1"

[[gate]]
id = "KILL-FSIM"
kind = "kill"
section = "33.1"
trigger = "airlock breaks"
fallback = "pinned worker or lift"
decided = "M0"
mode = "rolling"
reads = []
"#;
        let reg = parse_and_validate_bytes(src.as_bytes(), "rolling.toml", &ValidationContext::default())
            .expect("rolling row without reread loads");
        let dc = KillDueContext::default();
        for m in ["M0", "M1", "M2", "M3", "M4", "M5", "M6", "M7", "M8"] {
            let due = due_kill_gates(&reg, m, &dc);
            assert_eq!(due.len(), 1, "KILL-FSIM must be due at {m}");
            assert_eq!(due[0].requires_reread_line_in_adr, m != "M0");
        }
    }

    #[test]
    fn kill_row_no_measurement_fields() {
        let clean = r#"schema = "frob.gates/1"

[[gate]]
id = "KILL-CLEAN"
kind = "kill"
section = "21.24"
trigger = "fails"
fallback = "cpu"
decided = "M2"
mode = "exit"
reads = []
"#;
        let reg = parse_and_validate_bytes(clean.as_bytes(), "clean.toml", &ValidationContext::default())
            .expect("clean kill row loads");
        assert_eq!(
            kill_row_derived_state(reg.get("KILL-CLEAN").unwrap(), &reg),
            "not-run"
        );

        for (fld, extra) in [
            ("claim", "claim = \"parity\"\n"),
            ("controls", "controls = [\"NC-0001\"]\n"),
            (
                "part",
                "\n[[gate.part]]\nmilestone = \"M2\"\nstatus = \"not-run\"\nmeasured = \"\"\nevidence = []\nrecorded = { date = \"\", session = \"\", implemented_by = \"\" }\n",
            ),
        ] {
            let bad = format!("{clean}{extra}");
            let errs = parse_and_validate_bytes(bad.as_bytes(), "bad.toml", &ValidationContext::default())
                .expect_err(&format!("kill row with `{fld}` must fail"));
            assert!(
                errs.iter().any(|v| v.field.as_deref() == Some(fld)),
                "must name `{fld}`, got {errs:?}"
            );
        }
    }

    #[test]
    fn scope_kinds_closed() {
        let root = scratch_repo("scope-kinds");
        write_adr(&root, "docs/adr/ADR-KILL-SK-M2.md");
        let ctx = ctx_with_root(&root);

        // All 11 original scope kinds + Review Note #3 `morphology` (12 total).
        let valid_pairs = [
            ("cell", "cell:wheeled"),
            ("algorithm", "algorithm:ppo"),
            ("half", "half:cuts"),
            ("clause", "clause:finite"),
            ("backend", "backend:gpu-cuda"),
            ("item", "item:assets"),
            ("resolution", "resolution:2x2"),
            ("kind", "kind:fluid"),
            ("family", "family:counterfactual"),
            ("mechanism", "mechanism:chain-cord"),
            ("proofkind", "proofkind:lp-farkas"),
            ("morphology", "morphology:wheeled"),
        ];
        assert_eq!(valid_pairs.len(), ScopeKind::ALL.len());
        for (sk, sc) in valid_pairs {
            let src = format!(
                r#"schema = "frob.gates/1"

[[gate]]
id = "KILL-SK"
kind = "kill"
section = "21.24"
trigger = "t"
fallback = "fb"
decided = "M2"
mode = "exit"
scope_kind = "{sk}"
reads = []
decision = [{{ exit = "M2", scope = "{sc}", outcome = "fired", adr = "docs/adr/ADR-KILL-SK-M2.md", date = "2027-02-01", session = "s2", receipts = [] }}]
"#
            );
            parse_and_validate_bytes(src.as_bytes(), "sk-ok.toml", &ctx)
                .unwrap_or_else(|e| panic!("scope_kind `{sk}` with `{sc}` must pass: {e:?}"));
        }

        // Negative controls (RC-68): wrong kind, mismatched kind, or scope with no scope_kind.
        for (sk_line, bad_scope) in [
            ("scope_kind = \"backend\"\n", "family:cuda"),
            ("scope_kind = \"cell\"\n", "item:actors"),
            ("", "cell:wheeled"),
            ("scope_kind = \"backend\"\n", "backend:vulkan"),
            ("scope_kind = \"morphology\"\n", "morphology:flying"),
        ] {
            let src = format!(
                r#"schema = "frob.gates/1"

[[gate]]
id = "KILL-SK"
kind = "kill"
section = "21.24"
trigger = "t"
fallback = "fb"
decided = "M2"
mode = "exit"
{sk_line}reads = []
decision = [{{ exit = "M2", scope = "{bad_scope}", outcome = "fired", adr = "docs/adr/ADR-KILL-SK-M2.md", date = "2027-02-01", session = "s2", receipts = [] }}]
"#
            );
            let errs = parse_and_validate_bytes(src.as_bytes(), "sk-bad.toml", &ctx)
                .expect_err(&format!("`{bad_scope}` under `{sk_line}` must fail"));
            assert!(
                errs.iter().any(|v| v.reason == "decision-scope-invalid"),
                "expected decision-scope-invalid for `{bad_scope}`, got {errs:?}"
            );
        }
    }

    #[test]
    fn cell_scope_regime_m7_distinct_scopes() {
        // Review Note #8: `cell:<stratum>:<regime>` permits cold and warm M7 cells on one stratum.
        let root = scratch_repo("cell-regime");
        write_adr(&root, "docs/adr/ADR-KILL-TENFOLD-SPEED-M7.md");
        let ctx = ctx_with_root(&root);

        let src = r#"schema = "frob.gates/1"

[[gate]]
id = "KILL-TENFOLD-SPEED"
kind = "kill"
section = "39.10"
trigger = "speed < 1.0x"
fallback = "descriptive cell"
decided = "M3"
reread = ["M7"]
mode = "exit"
scope_kind = "cell"
reads = []
decision = [
  { exit = "M7", scope = "cell:wheeled:cold", outcome = "fired", adr = "docs/adr/ADR-KILL-TENFOLD-SPEED-M7.md", date = "2027-07-01", session = "s2", receipts = [] },
  { exit = "M7", scope = "cell:wheeled:warm", outcome = "not-reached", adr = "docs/adr/ADR-KILL-TENFOLD-SPEED-M7.md", date = "2027-07-01", session = "s2", receipts = [] }
]
"#;
        let reg = parse_and_validate_bytes(src.as_bytes(), "regime.toml", &ctx)
            .expect("distinct cold/warm regimes on one stratum at M7 must load");
        let force = decisions_in_force(reg.get("KILL-TENFOLD-SPEED").unwrap(), &reg);
        assert_eq!(force.len(), 2);
        assert_eq!(force[&Some("cell:wheeled:cold".to_owned())].outcome, KillOutcome::Fired);
        assert_eq!(force[&Some("cell:wheeled:warm".to_owned())].outcome, KillOutcome::NotReached);
    }

    #[test]
    fn fallback_form_verbatim() {
        let root = scratch_repo("fallback-form");
        let rhex = write_receipt(&root, "receipts/gates", "{\"proxy\":true}\n");
        write_adr(&root, "docs/adr/ADR-KILL-PROXY-M5.md");
        let ctx = ctx_with_root(&root);

        let good = r#"schema = "frob.gates/1"

[[gate]]
id = "KILL-PROXY"
kind = "kill"
section = "19.7"
trigger = "Solid proxies already reproduce articulated failures"
fallback = "articulated proxies in evaluation batches only, or not at all"
decided = "M5"
mode = "exit"
scope_kind = "mechanism"
reads = []
decision = [
  { exit = "M5", scope = "mechanism:chain-cord", outcome = "fired", adr = "docs/adr/ADR-KILL-PROXY-M5.md", date = "2027-05-01", session = "s2", receipts = [], fallback_form = "articulated proxies in evaluation batches only" },
  { exit = "M5", scope = "mechanism:hinged-rug", outcome = "fired", adr = "docs/adr/ADR-KILL-PROXY-M5.md", date = "2027-05-01", session = "s2", receipts = [] }
]
"#;
        let reg = parse_and_validate_bytes(good.as_bytes(), "proxy-ok.toml", &ctx)
            .expect("valid fallback_form must load");
        let g = reg.get("KILL-PROXY").unwrap();
        let force = decisions_in_force(g, &reg);
        assert_eq!(
            effective_fallback_text(g, force[&Some("mechanism:chain-cord".to_owned())]),
            "articulated proxies in evaluation batches only"
        );
        assert_eq!(
            effective_fallback_text(g, force[&Some("mechanism:hinged-rug".to_owned())]),
            "not at all"
        );

        // Non-verbatim fallback_form fails naming `fallback_form`.
        let non_verbatim = good.replace(
            "fallback_form = \"articulated proxies in evaluation batches only\"",
            "fallback_form = \"evaluation only\"",
        );
        let errs1 = parse_and_validate_bytes(non_verbatim.as_bytes(), "nv.toml", &ctx)
            .expect_err("non-verbatim fallback_form must fail");
        assert!(errs1.iter().any(|v| v.field.as_deref() == Some("fallback_form")));

        // `fallback_form` on `not-fired` fails naming `fallback_form`.
        let on_nf = format!(
            r#"schema = "frob.gates/1"

[[gate]]
id = "KILL-PROXY"
kind = "kill"
section = "19.7"
trigger = "t"
fallback = "articulated proxies in evaluation batches only, or not at all"
decided = "M5"
mode = "exit"
scope_kind = "mechanism"
reads = []
decision = [
  {{ exit = "M5", scope = "mechanism:chain-cord", outcome = "not-fired", adr = "docs/adr/ADR-KILL-PROXY-M5.md", date = "2027-05-01", session = "s2", receipts = ["{rhex}"], fallback_form = "not at all" }}
]
"#
        );
        let errs2 = parse_and_validate_bytes(on_nf.as_bytes(), "on-nf.toml", &ctx)
            .expect_err("fallback_form on not-fired must fail");
        assert!(errs2.iter().any(|v| v.field.as_deref() == Some("fallback_form")));
    }

    #[test]
    fn backend_scope_per_family() {
        let root = scratch_repo("backend-scope");
        let rhex = write_receipt(&root, "receipts/gates", "{\"backend\":\"metal\"}\n");
        write_adr(&root, "docs/adr/ADR-KILL-D1-M2.md");
        let ctx = ctx_with_root(&root);

        let src = format!(
            r#"schema = "frob.gates/1"

[[gate]]
id = "KILL-D1"
kind = "kill"
section = "21.24"
trigger = "determinism mismatch"
fallback = "host replay only"
decided = "M2"
mode = "exit"
scope_kind = "backend"
reads = []
decision = [
  {{ exit = "M2", scope = "backend:gpu-cuda", outcome = "fired", adr = "docs/adr/ADR-KILL-D1-M2.md", date = "2027-02-01", session = "s2", receipts = ["{rhex}"] }},
  {{ exit = "M2", scope = "backend:gpu-metal", outcome = "not-fired", adr = "docs/adr/ADR-KILL-D1-M2.md", date = "2027-02-01", session = "s2", receipts = ["{rhex}"] }}
]
"#
        );
        let reg = parse_and_validate_bytes(src.as_bytes(), "d1.toml", &ctx)
            .expect("per-backend entries sharing ADR-KILL-D1-M2.md must load");
        let force = decisions_in_force(reg.get("KILL-D1").unwrap(), &reg);
        assert_eq!(force[&Some("backend:gpu-cuda".to_owned())].outcome, KillOutcome::Fired);
        assert_eq!(force[&Some("backend:gpu-metal".to_owned())].outcome, KillOutcome::NotFired);
    }

    #[test]
    fn item_scope_reread() {
        let root = scratch_repo("item-reread");
        let rhex = write_receipt(&root, "receipts/gates", "{\"item\":\"ok\"}\n");
        write_adr(&root, "docs/adr/ADR-KILL-DRESSING-M5.md");
        write_adr(&root, "docs/adr/ADR-KILL-DRESSING-M6.md");
        let ctx = ctx_with_root(&root);

        let src = format!(
            r#"schema = "frob.gates/1"

[[gate]]
id = "KILL-DRESSING"
kind = "kill"
section = "19.9"
trigger = "dressing overhead"
fallback = "static proxy"
decided = "M5"
reread = ["M6"]
mode = "exit"
scope_kind = "item"
reads = []
decision = [
  {{ exit = "M5", scope = "item:assets", outcome = "fired", adr = "docs/adr/ADR-KILL-DRESSING-M5.md", date = "2027-05-01", session = "s2", receipts = ["{rhex}"] }},
  {{ exit = "M6", scope = "item:actors", outcome = "not-fired", adr = "docs/adr/ADR-KILL-DRESSING-M6.md", date = "2027-06-01", session = "s3", receipts = ["{rhex}"] }}
]
"#
        );
        let reg = parse_and_validate_bytes(src.as_bytes(), "dressing.toml", &ctx)
            .expect("item scope reread loads");
        let force = decisions_in_force_at(reg.get("KILL-DRESSING").unwrap(), &reg, "M6");
        assert_eq!(force[&Some("item:assets".to_owned())].outcome, KillOutcome::Fired);
        assert_eq!(force[&Some("item:actors".to_owned())].outcome, KillOutcome::NotFired);
    }

    #[test]
    fn second_reading_named_final() {
        let root = scratch_repo("second-final");
        let rhex = write_receipt(&root, "receipts/gates", "{\"sg\":\"ok\"}\n");
        write_adr(&root, "docs/adr/ADR-KILL-SHORTGRAD-M6.md");
        write_adr(&root, "docs/adr/ADR-KILL-SHORTGRAD-M6-final.md");
        let ctx = ctx_with_root(&root);

        let ok = format!(
            r#"schema = "frob.gates/1"

[[gate]]
id = "KILL-SHORTGRAD"
kind = "kill"
section = "23.8"
trigger = "short-horizon cosine fails"
fallback = "stencil gradients"
decided = "M6"
mode = "exit"
reads = []
decision = [
  {{ exit = "M6", outcome = "not-fired", adr = "docs/adr/ADR-KILL-SHORTGRAD-M6.md", date = "2027-06-01", session = "s2", receipts = ["{rhex}"] }},
  {{ exit = "M6", reading = "final", outcome = "fired", adr = "docs/adr/ADR-KILL-SHORTGRAD-M6-final.md", date = "2027-06-10", session = "s3", receipts = ["{rhex}"] }}
]
"#
        );
        let reg = parse_and_validate_bytes(ok.as_bytes(), "sg-ok.toml", &ctx)
            .expect("second reading named final must load");
        let force = decisions_in_force(reg.get("KILL-SHORTGRAD").unwrap(), &reg);
        assert_eq!(force[&None].outcome, KillOutcome::Fired);
        assert_eq!(force[&None].reading, Some(DecisionReading::Final));

        // Duplicate M6 entry with no reading -> `decision-entry-duplicate` (RC-69).
        let dup = ok
            .replace("reading = \"final\", ", "")
            .replace("ADR-KILL-SHORTGRAD-M6-final.md", "ADR-KILL-SHORTGRAD-M6.md");
        let errs_dup = parse_and_validate_bytes(dup.as_bytes(), "sg-dup.toml", &ctx)
            .expect_err("second entry without reading must fail RC-69");
        assert!(errs_dup.iter().any(|v| v.reason == "decision-entry-duplicate"));

        // Retired path `docs/delivery/adr/...` -> `decision-adr-path`.
        let bad_adr = ok.replace(
            "docs/adr/ADR-KILL-SHORTGRAD-M6-final.md",
            "docs/delivery/adr/ADR-KILL-SHORTGRAD-M6-final.md",
        );
        let errs_adr = parse_and_validate_bytes(bad_adr.as_bytes(), "sg-bad-adr.toml", &ctx)
            .expect_err("retired docs/delivery/adr/ path must fail");
        assert!(errs_adr.iter().any(|v| v.reason == "decision-adr-path"));
    }

    #[test]
    fn deferred_moves_due() {
        let root = scratch_repo("deferred-moves-due");
        write_adr(&root, "docs/adr/ADR-KILL-MV-M3.md");
        let ctx = ctx_with_root(&root);

        let src = r#"schema = "frob.gates/1"

[[gate]]
id = "KILL-MV"
kind = "kill"
section = "24.1"
trigger = "too few verdicts"
fallback = "narrow scope"
decided = "M3"
mode = "exit"
reads = []
decision = { outcome = "deferred", adr = "docs/adr/ADR-KILL-MV-M3.md", date = "2027-03-01", session = "s2", receipts = [], deferred_to = "M4", reason = "insufficient verdicts" }
"#;
        let reg = parse_and_validate_bytes(src.as_bytes(), "mv.toml", &ctx)
            .expect("deferred decision loads");
        let dc = KillDueContext::default();
        let m3 = due_kill_gates(&reg, "M3", &dc);
        assert_eq!(m3.len(), 1);
        assert!(matches!(m3[0].status, KillDueStatus::Deferred { .. }));
        assert!(!m3[0].status.is_missing());

        let m4 = due_kill_gates(&reg, "M4", &dc);
        assert_eq!(m4.len(), 1);
        assert_eq!(m4[0].gate_id, "KILL-MV");
        assert!(m4[0].status.is_missing());
    }

    #[test]
    fn introduce_seeds_row() {
        let mut ctx = ValidationContext::default();
        ctx.kill.seeded_kill_rows = Some(BTreeMap::new());
        ctx.kill
            .gate_earliest_receipt_ts
            .insert("GATE-RUT-01".to_owned(), "2027-05-01T12:00:00Z".to_owned());

        let intro_seed = SeededKillRow {
            trigger: "rut leak > 0".to_owned(),
            fallback: "reset terrain every episode".to_owned(),
            decided: Some("M5".to_owned()),
            mode: KillMode::Exit,
            reads: vec!["GATE-RUT-01".to_owned()],
        };
        let intro_inline = format_introduce_inline_table(&intro_seed);

        // 1. Valid `introduce` + later trigger amendment -> loads cleanly.
        let ok = format!(
            r#"schema = "frob.gates/1"

[[gate]]
id = "KILL-RUT"
kind = "kill"
section = "18.8"
trigger = "rut leak > 1e-6"
fallback = "reset terrain every episode"
decided = "M5"
mode = "exit"
reads = ["GATE-RUT-01"]

[[amendment]]
id = "AMD-0010"
gate = "KILL-RUT"
field = "introduce"
old = ""
new = '{intro_inline}'
reason = "proposed by PILOT-SOIL-01"
date = "2027-04-01T10:00:00Z"
commit = "4444444444444444444444444444444444444444"

[[amendment]]
id = "AMD-0011"
gate = "KILL-RUT"
field = "trigger"
old = "rut leak > 0"
new = "rut leak > 1e-6"
reason = "numeric floor fixed before measurement"
date = "2027-04-15T10:00:00Z"
commit = "5555555555555555555555555555555555555555"
"#
        );
        // Replace single-quoted string with valid double-quoted TOML string for strict frob-core TOML.
        let mut escaped_inline = String::new();
        push_toml_str(&mut escaped_inline, &intro_inline);
        let ok = ok.replace(&format!("'{intro_inline}'"), &escaped_inline);
        parse_and_validate_bytes(ok.as_bytes(), "intro-ok.toml", &ctx)
            .expect("introduce amendment + later replay must pass");

        // 2. Same unseeded row with no `introduce` amendment -> `changed-without-amendment` (RC-70).
        let no_intro = r#"schema = "frob.gates/1"

[[gate]]
id = "KILL-RUT"
kind = "kill"
section = "18.8"
trigger = "rut leak > 0"
fallback = "reset terrain every episode"
decided = "M5"
mode = "exit"
reads = ["GATE-RUT-01"]
"#;
        let errs_no_intro = parse_and_validate_bytes(no_intro.as_bytes(), "no-intro.toml", &ctx)
            .expect_err("unseeded row without introduce must fail RC-70");
        assert!(
            errs_no_intro
                .iter()
                .any(|v| v.reason == "changed-without-amendment")
        );

        // 3. `introduce` dated after a receipt of a read gate -> `amended-after-measurement`.
        let late_intro = ok.replace("2027-04-01T10:00:00Z", "2027-05-02T10:00:00Z");
        let errs_late = parse_and_validate_bytes(late_intro.as_bytes(), "late-intro.toml", &ctx)
            .expect_err("introduce after receipt must fail");
        assert!(
            errs_late
                .iter()
                .any(|v| v.reason == "amended-after-measurement")
        );
    }

    #[test]
    fn live_ids_check_reports_absent_duplicate_incomplete_and_changed() {
        let h1 = kill_row_seed_hash("t1", "fb1");
        let h2 = kill_row_seed_hash("t2", "fb2");
        let src = format!(
            r#"schema = "frob.gates/1"

[[gate]]
id = "KILL-A"
kind = "kill"
section = "21.1"
trigger = "t1"
fallback = "fb1"
decided = "M2"
mode = "exit"
reads = []

[[gate]]
id = "KILL-B"
kind = "kill"
section = "21.1"
trigger = "t2-changed"
fallback = "fb2"
decided = "M2"
mode = "exit"
reads = []
"#
        );
        let reg = parse_and_validate_bytes(src.as_bytes(), "live.toml", &ValidationContext::default())
            .expect("register loads");

        let ok_spec = parse_live_ids_file(&format!("KILL-A {h1}\n")).expect("parse ids");
        verify_live_kill_ids(&reg, &ok_spec).expect("matching seed passes");

        let changed_spec = parse_live_ids_file(&format!("KILL-A {h1}\nKILL-B {h2}\n")).expect("parse");
        let err_changed = verify_live_kill_ids(&reg, &changed_spec).expect_err("changed trigger fails");
        assert!(err_changed.contains("KILL-B") && err_changed.contains("text differs"));

        let absent_spec = parse_live_ids_file("KILL-MISSING\n").expect("parse");
        let err_absent = verify_live_kill_ids(&reg, &absent_spec).expect_err("absent ID fails");
        assert!(err_absent.contains("KILL-MISSING") && err_absent.contains("absent"));
    }

    #[test]
    fn amendment_replay_is_deterministic_d0() {
        let src = r#"schema = "frob.gates/1"

[[gate]]
id = "KILL-DET"
kind = "kill"
section = "21.1"
trigger = "below 0.45"
fallback = "cpu"
decided = "M2"
mode = "exit"
reads = []

[[amendment]]
id = "AMD-0099"
gate = "KILL-DET"
field = "trigger"
old = "below 0.5"
new = "below 0.45"
reason = "pre-measurement"
date = "2027-01-01T00:00:00Z"
commit = "9999999999999999999999999999999999999999"
"#;
        let mut seeds = BTreeMap::new();
        seeds.insert(
            "KILL-DET".to_owned(),
            SeededKillRow {
                trigger: "below 0.5".to_owned(),
                fallback: "cpu".to_owned(),
                decided: Some("M2".to_owned()),
                mode: KillMode::Exit,
                reads: vec![],
            },
        );
        let mut ctx = ValidationContext::default();
        ctx.kill.seeded_kill_rows = Some(seeds);

        let r1 = parse_and_validate_bytes(src.as_bytes(), "det.toml", &ctx).expect("run 1");
        let r2 = parse_and_validate_bytes(src.as_bytes(), "det.toml", &ctx).expect("run 2");
        assert_eq!(r1.print_toml(), r2.print_toml());
        assert_eq!(render_readme_status_table(&r1), render_readme_status_table(&r2));
        assert_eq!(render_register_printout(&r1), render_register_printout(&r2));
    }
}
