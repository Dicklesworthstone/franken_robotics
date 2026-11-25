#![deny(unsafe_code)]

//! Base gate-register record types, parser, and deterministic D0 TOML printer (`N38b-18`,
//! schema `frob.gates/1`).
//!
//! Depends strictly on `std` and `frob-core` (`frob_core::registry::toml`, single-reader rule
//! `N34p2-01` / `STD-TEST`) so generated-view checkers can load and print base gate records
//! without pulling in the rest of `frob-conformance`.
//!
//! ## Status-per-part architecture (DESIGN, settled with N40-01 / N40-02)
//!
//! Status, `measured`, `evidence`, and `recorded` live on `[[gate.part]]`, one entry per
//! milestone at which a part of the gate is due (40 convergence gates — including
//! `GATE-MARROW-01`, `GATE-HAB-01`, `GATE-HAB-13`, `GATE-ARENA-01`, and `GATE-LEMMA-03` per
//! `N40-02` step 5 and round-4 review note #1198 — plus 3 admission gates carry parts at
//! multiple milestones). A gate's own status is derived and never stored: `not-run` when no
//! part is recorded (or no part is due yet), otherwise the status of its latest due part.
//!
//! ## Validator contract (`N40-01`)
//!
//! This module enforces what types can enforce at parse time: closed seven-variant [`Claim`],
//! closed seven-variant [`Status`], required base fields on measured rows, absence of
//! measurement fields on `kind = "kill"` rows, and duplicate/malformed key rejection.
//! Cross-field and external-resolution rules are enforced by `N40-01`'s validator:
//! - Four `pass` refusal conditions:
//!   (a) `controls` is empty;
//!   (b) any listed control was not caught in the same run as the passing measurement;
//!   (c) `recorded.session` is a member of `recorded.implemented_by` (sorted comma-joined set);
//!   (d) an `evidence` item or `claim_id` does not resolve in the artifact graph (`FROB-REF-869`),
//!       or the part's `receipt` does not name a committed `receipts/gates/<hash>.json` whose
//!       bytes hash to it.
//! - `measured` carries the measured value (non-empty and not byte-equal to `threshold`).
//! - Identifiers are unique and append-only; retired identifiers are tombstoned, never reused.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::fs;
use std::path::Path;

use frob_core::registry::toml::{self, Fields, Spanned, Table, Value};

/// Top-level schema identifier for `registries/gates.toml`.
pub const GATES_SCHEMA: &str = "frob.gates/1";

/// Measurement fields forbidden on `kind = "kill"` rows (`N38b-18`, `N40-01`).
pub const KILL_FORBIDDEN_FIELDS: &[&str] = &[
    "claim",
    "claim_id",
    "threshold",
    "tier",
    "controls",
    "part",
];

/// Closed set of the seven gate claim kinds (`frob.gates/1`, `N38b-18`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Claim {
    Proof,
    KnownAnswer,
    Parity,
    Fidelity,
    DesignComparison,
    Reality,
    Performance,
}

impl Claim {
    /// All seven closed claim kinds in specification order.
    pub const ALL: &'static [Self] = &[
        Self::Proof,
        Self::KnownAnswer,
        Self::Parity,
        Self::Fidelity,
        Self::DesignComparison,
        Self::Reality,
        Self::Performance,
    ];

    /// Canonical TOML / schema token.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Proof => "proof",
            Self::KnownAnswer => "known-answer",
            Self::Parity => "parity",
            Self::Fidelity => "fidelity",
            Self::DesignComparison => "design-comparison",
            Self::Reality => "reality",
            Self::Performance => "performance",
        }
    }

    /// Parse a closed claim token; rejects `"method"` and `"Performance of the method"`.
    pub fn parse(raw: &str, file: &str, gate_id: &str) -> Result<Self, BaseRowError> {
        match raw {
            "proof" => Ok(Self::Proof),
            "known-answer" => Ok(Self::KnownAnswer),
            "parity" => Ok(Self::Parity),
            "fidelity" => Ok(Self::Fidelity),
            "design-comparison" => Ok(Self::DesignComparison),
            "reality" => Ok(Self::Reality),
            "performance" => Ok(Self::Performance),
            other => {
                let lower = other.trim().to_ascii_lowercase();
                let hint = if lower == "method" || lower == "performance of the method" {
                    " (`method` / `Performance of the method` is rejected; edit the row explicitly to `design-comparison`)"
                } else {
                    ""
                };
                Err(BaseRowError::new(
                    file,
                    Some(gate_id),
                    format!(
                        "unknown claim `{other}` (expected one of: proof, known-answer, parity, fidelity, design-comparison, reality, performance){hint}"
                    ),
                ))
            }
        }
    }
}

impl fmt::Display for Claim {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Closed set of the seven per-part gate statuses (`frob.gates/1`, `N38b-18`).
///
/// Reconciled union of the six base statuses (`not-run`, `pass`, `fail`, `killed`,
/// `no-claim`, `superseded`) plus `blocked` (owned by the register layer `N40-01`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Status {
    NotRun,
    Pass,
    Fail,
    Killed,
    Blocked,
    NoClaim,
    Superseded,
}

impl Status {
    /// All seven closed statuses in specification order.
    pub const ALL: &'static [Self] = &[
        Self::NotRun,
        Self::Pass,
        Self::Fail,
        Self::Killed,
        Self::Blocked,
        Self::NoClaim,
        Self::Superseded,
    ];

    /// Canonical TOML / schema token.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotRun => "not-run",
            Self::Pass => "pass",
            Self::Fail => "fail",
            Self::Killed => "killed",
            Self::Blocked => "blocked",
            Self::NoClaim => "no-claim",
            Self::Superseded => "superseded",
        }
    }

    /// Parse a closed status token.
    pub fn parse(raw: &str, file: &str, gate_id: &str) -> Result<Self, BaseRowError> {
        match raw {
            "not-run" => Ok(Self::NotRun),
            "pass" => Ok(Self::Pass),
            "fail" => Ok(Self::Fail),
            "killed" => Ok(Self::Killed),
            "blocked" => Ok(Self::Blocked),
            "no-claim" => Ok(Self::NoClaim),
            "superseded" => Ok(Self::Superseded),
            other => Err(BaseRowError::new(
                file,
                Some(gate_id),
                format!(
                    "unknown status `{other}` (expected one of: not-run, pass, fail, killed, blocked, no-claim, superseded)"
                ),
            )),
        }
    }

    /// Reconciled human-readable printed form (`N38b-18` status reconciliation ADR).
    #[must_use]
    pub fn printed_form(self, detail: Option<&str>) -> String {
        match self {
            Self::NotRun => "not run".to_owned(),
            Self::Pass => "pass".to_owned(),
            Self::Fail => "fail".to_owned(),
            Self::Killed => match detail.map(str::trim).filter(|s| !s.is_empty()) {
                Some(fb) => format!("killed: fallback adopted ({fb})"),
                None => "killed: fallback adopted".to_owned(),
            },
            Self::Blocked => match detail.map(str::trim).filter(|s| !s.is_empty()) {
                Some(q) => format!("blocked: {q}"),
                None => "blocked".to_owned(),
            },
            Self::NoClaim => "no-claim".to_owned(),
            Self::Superseded => match detail.map(str::trim).filter(|s| !s.is_empty()) {
                Some(succ) => format!("superseded: {succ}"),
                None => "superseded".to_owned(),
            },
        }
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Optional gate row kind (`N38b-18` / `N40-01`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BaseKind {
    Admission,
    Convergence,
    Program,
    Kill,
}

impl BaseKind {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Admission => "admission",
            Self::Convergence => "convergence",
            Self::Program => "program",
            Self::Kill => "kill",
        }
    }

    pub fn parse(raw: &str, file: &str, gate_id: &str) -> Result<Self, BaseRowError> {
        match raw {
            "admission" => Ok(Self::Admission),
            "convergence" => Ok(Self::Convergence),
            "program" => Ok(Self::Program),
            "kill" => Ok(Self::Kill),
            other => Err(BaseRowError::new(
                file,
                Some(gate_id),
                format!("unknown gate kind `{other}` (expected: admission, convergence, program, kill)"),
            )),
        }
    }

    /// Infer kind from gate identifier prefix when `kind` is omitted on pre-N40-01 base rows.
    #[must_use]
    pub fn infer_from_id(id: &str) -> Self {
        if id.starts_with("INT-") {
            Self::Admission
        } else if id.starts_with("KILL-") {
            Self::Kill
        } else if id.starts_with("PROG-") {
            Self::Program
        } else {
            Self::Convergence
        }
    }
}

/// Error returned while loading or parsing base gate records.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BaseRowError {
    pub file: String,
    pub gate_id: Option<String>,
    pub message: String,
}

impl BaseRowError {
    #[must_use]
    pub fn new(file: &str, gate_id: Option<&str>, message: impl Into<String>) -> Self {
        Self {
            file: file.to_owned(),
            gate_id: gate_id.map(str::to_owned),
            message: message.into(),
        }
    }
}

impl fmt::Display for BaseRowError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.gate_id {
            Some(id) => write!(f, "{}: gate `{id}`: {}", self.file, self.message),
            None => write!(f, "{}: {}", self.file, self.message),
        }
    }
}

impl std::error::Error for BaseRowError {}

/// Parse milestone string (`"M0"`..`"M8"`) to its ordinal `0..=8`.
#[must_use]
pub fn parse_milestone_ord(m: &str) -> Option<u32> {
    let digits = m.trim().strip_prefix('M')?;
    if digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    let n = digits.parse::<u32>().ok()?;
    if n <= 8 { Some(n) } else { None }
}

fn validate_milestone(m: &str, file: &str, gate_id: &str, field: &str) -> Result<(), BaseRowError> {
    if parse_milestone_ord(m).is_some() {
        Ok(())
    } else {
        Err(BaseRowError::new(
            file,
            Some(gate_id),
            format!("invalid {field} `{m}` (expected M0..M8)"),
        ))
    }
}

/// Session attribution metadata recorded on a `[[gate.part]]` entry.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Recorded {
    pub date: String,
    pub session: String,
    /// Sorted, comma-joined set of implementing session identifiers (rule 3(c)).
    pub implemented_by: String,
}

/// Base fields of one `[[gate.part]]` milestone record (`N38b-18`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartBase {
    pub milestone: String,
    pub status: Status,
    pub measured: String,
    pub evidence: Vec<String>,
    pub recorded: Recorded,
    /// Optional detail string used by `killed` (kill gate + fallback), `blocked` (`OPEN-…`),
    /// and `superseded` (successor gate identifier).
    pub detail: Option<String>,
    pub kill_gate: Option<String>,
    pub fallback: Option<String>,
    pub open_question: Option<String>,
}

impl PartBase {
    /// Reconciled printed status string for this part.
    #[must_use]
    pub fn printed_status(&self) -> String {
        let context = match self.status {
            Status::Killed => self
                .fallback
                .as_deref()
                .or(self.detail.as_deref()),
            Status::Blocked => self
                .open_question
                .as_deref()
                .or(self.detail.as_deref()),
            Status::Superseded => self.detail.as_deref(),
            _ => None,
        };
        self.status.printed_form(context)
    }
}

/// Measurement base fields present on `admission`, `convergence`, and `program` rows
/// (strictly absent on `kill` rows).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MeasuredFields {
    pub claim: Claim,
    /// Raw claim identifier (`clm:<64 hex>` validated and resolved by `N40-01`).
    pub claim_id: String,
    pub threshold: String,
    pub milestone: String,
    pub tier: Vec<String>,
    pub controls: Vec<String>,
    pub parts: Vec<PartBase>,
}

/// Base record of one `[[gate]]` row in `registries/gates.toml` (`frob.gates/1`, `N38b-18`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateBase {
    pub id: String,
    pub kind: Option<BaseKind>,
    pub section: String,
    /// Present on measured rows (`admission` / `convergence` / `program` / untyped);
    /// strictly `None` on `kind = "kill"` rows.
    pub measured: Option<MeasuredFields>,
    /// Optional `decided` milestone on `kind = "kill"` rows (`N40-01` / `N40-03`).
    pub decided: Option<String>,
}

impl GateBase {
    /// Effective kind of this row (explicit `kind` if present, otherwise inferred from `id`).
    #[must_use]
    pub fn effective_kind(&self) -> BaseKind {
        self.kind.unwrap_or_else(|| BaseKind::infer_from_id(&self.id))
    }

    /// True when this row is a kill gate carrying no measurement fields.
    #[must_use]
    pub fn is_kill(&self) -> bool {
        self.kind == Some(BaseKind::Kill)
    }

    /// Parts slice (empty on kill rows or unrecorded gates).
    #[must_use]
    pub fn parts(&self) -> &[PartBase] {
        match &self.measured {
            Some(m) => &m.parts,
            None => &[],
        }
    }

    /// Derived status across all recorded parts (`NotRun` when no parts exist; otherwise the
    /// status of the latest part by milestone order).
    #[must_use]
    pub fn derived_status(&self) -> Status {
        self.latest_part().map_or(Status::NotRun, |p| p.status)
    }

    /// Derived status at milestone `due_milestone` (`M0`..`M8`): `NotRun` when no part with
    /// `part.milestone <= due_milestone` is recorded; otherwise the status of the latest due part.
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
        part.map_or_else(|| Status::NotRun.printed_form(None), PartBase::printed_status)
    }

    fn latest_part(&self) -> Option<&PartBase> {
        let parts = self.parts();
        if parts.is_empty() {
            return None;
        }
        parts
            .iter()
            .enumerate()
            .max_by_key(|(idx, p)| (parse_milestone_ord(&p.milestone).unwrap_or(0), *idx))
            .map(|(_, p)| p)
    }

    fn latest_due_part(&self, due_milestone: &str) -> Option<&PartBase> {
        let Some(due_ord) = parse_milestone_ord(due_milestone) else {
            return None;
        };
        self.parts()
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

/// Record counts grouped by gate kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct KindCounts {
    pub convergence: usize,
    pub kill: usize,
    pub admission: usize,
    pub program: usize,
    pub total: usize,
}

/// Parsed base-record view of a `frob.gates/1` document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BaseDocument {
    pub schema: String,
    pub gates: Vec<GateBase>,
}

impl BaseDocument {
    /// Load and parse a `frob.gates/1` TOML file from disk.
    pub fn load(path: &Path) -> Result<Self, BaseRowError> {
        let label = path.to_string_lossy();
        let bytes = fs::read(path)
            .map_err(|e| BaseRowError::new(&label, None, format!("read failed: {e}")))?;
        Self::parse_bytes(&bytes, &label)
    }

    /// Parse a `frob.gates/1` TOML string.
    pub fn parse_str(src: &str, file: &str) -> Result<Self, BaseRowError> {
        Self::parse_bytes(src.as_bytes(), file)
    }

    /// Parse `frob.gates/1` bytes strictly through `frob_core::registry::toml` (D0).
    pub fn parse_bytes(bytes: &[u8], file: &str) -> Result<Self, BaseRowError> {
        let root = toml::parse(bytes).map_err(|e| BaseRowError::new(file, None, e.to_string()))?;
        let mut top = Fields::new(root, "gates");
        let schema = top
            .require_str("schema")
            .map_err(|e| BaseRowError::new(file, None, e.to_string()))?;
        if schema != GATES_SCHEMA {
            return Err(top.fail(BaseRowError::new(
                file,
                None,
                format!("unexpected schema `{schema}` (expected `{GATES_SCHEMA}`)"),
            )));
        }
        let raw_gates = top
            .opt_array("gate")
            .map_err(|e| BaseRowError::new(file, None, e.to_string()))?
            .unwrap_or_default();
        top.finish()
            .map_err(|e| BaseRowError::new(file, None, e.to_string()))?;

        let mut gates = Vec::with_capacity(raw_gates.len());
        let mut seen = BTreeSet::new();
        for (idx, sp) in raw_gates.into_iter().enumerate() {
            let tbl = match sp.value {
                Value::Table(t) => t,
                other => {
                    return Err(BaseRowError::new(
                        file,
                        Some(&format!("gate[{idx}]")),
                        format!("expected table, found {}", other.type_name()),
                    ));
                }
            };
            let row = parse_gate_row(tbl, idx, file)?;
            if !seen.insert(row.id.clone()) {
                return Err(BaseRowError::new(
                    file,
                    Some(&row.id),
                    format!("duplicate gate identifier `{}`", row.id),
                ));
            }
            gates.push(row);
        }
        Ok(Self { schema, gates })
    }

    /// Count rows per kind (`convergence`, `kill`, `admission`, `program`, `total`).
    #[must_use]
    pub fn counts_by_kind(&self) -> KindCounts {
        let mut c = KindCounts {
            total: self.gates.len(),
            ..KindCounts::default()
        };
        for g in &self.gates {
            match g.effective_kind() {
                BaseKind::Convergence => c.convergence += 1,
                BaseKind::Kill => c.kill += 1,
                BaseKind::Admission => c.admission += 1,
                BaseKind::Program => c.program += 1,
            }
        }
        c
    }

    /// Count measured gates by derived status (deterministic `BTreeMap` over all 7 statuses).
    #[must_use]
    pub fn counts_by_status(&self) -> BTreeMap<Status, usize> {
        let mut out = BTreeMap::new();
        for &s in Status::ALL {
            out.insert(s, 0);
        }
        for g in &self.gates {
            if g.measured.is_some() {
                let st = g.derived_status();
                *out.entry(st).or_insert(0) += 1;
            }
        }
        out
    }

    /// Count measured gates by claim kind (deterministic `BTreeMap` over all 7 claims).
    #[must_use]
    pub fn counts_by_claim(&self) -> BTreeMap<Claim, usize> {
        let mut out = BTreeMap::new();
        for &cl in Claim::ALL {
            out.insert(cl, 0);
        }
        for g in &self.gates {
            if let Some(ref m) = g.measured {
                *out.entry(m.claim).or_insert(0) += 1;
            }
        }
        out
    }

    /// Deterministic D0 TOML printer emitting the exact base fields of `frob.gates/1`.
    #[must_use]
    pub fn print_toml(&self) -> String {
        let mut out = String::new();
        out.push_str("schema = ");
        push_toml_str(&mut out, &self.schema);
        out.push('\n');

        for g in &self.gates {
            out.push_str("\n[[gate]]\n");
            out.push_str("id = ");
            push_toml_str(&mut out, &g.id);
            out.push('\n');

            if let Some(kind) = g.kind {
                out.push_str("kind = ");
                push_toml_str(&mut out, kind.as_str());
                out.push('\n');
            }

            if let Some(ref m) = g.measured {
                out.push_str("claim = ");
                push_toml_str(&mut out, m.claim.as_str());
                out.push('\n');

                out.push_str("claim_id = ");
                push_toml_str(&mut out, &m.claim_id);
                out.push('\n');

                out.push_str("section = ");
                push_toml_str(&mut out, &g.section);
                out.push('\n');

                out.push_str("threshold = ");
                push_toml_str(&mut out, &m.threshold);
                out.push('\n');

                out.push_str("milestone = ");
                push_toml_str(&mut out, &m.milestone);
                out.push('\n');

                out.push_str("tier = ");
                push_toml_str_array(&mut out, &m.tier);
                out.push('\n');

                out.push_str("controls = ");
                push_toml_str_array(&mut out, &m.controls);
                out.push('\n');

                for p in &m.parts {
                    out.push_str("\n[[gate.part]]\n");
                    out.push_str("milestone = ");
                    push_toml_str(&mut out, &p.milestone);
                    out.push('\n');

                    out.push_str("status = ");
                    push_toml_str(&mut out, p.status.as_str());
                    out.push('\n');

                    out.push_str("measured = ");
                    push_toml_str(&mut out, &p.measured);
                    out.push('\n');

                    out.push_str("evidence = ");
                    push_toml_str_array(&mut out, &p.evidence);
                    out.push('\n');

                    out.push_str("recorded = { date = ");
                    push_toml_str(&mut out, &p.recorded.date);
                    out.push_str(", session = ");
                    push_toml_str(&mut out, &p.recorded.session);
                    out.push_str(", implemented_by = ");
                    push_toml_str(&mut out, &p.recorded.implemented_by);
                    out.push_str(" }\n");

                    if let Some(ref kg) = p.kill_gate {
                        out.push_str("kill_gate = ");
                        push_toml_str(&mut out, kg);
                        out.push('\n');
                    }
                    if let Some(ref fb) = p.fallback {
                        out.push_str("fallback = ");
                        push_toml_str(&mut out, fb);
                        out.push('\n');
                    }
                    if let Some(ref oq) = p.open_question {
                        out.push_str("open_question = ");
                        push_toml_str(&mut out, oq);
                        out.push('\n');
                    }
                    if let Some(ref d) = p.detail {
                        out.push_str("detail = ");
                        push_toml_str(&mut out, d);
                        out.push('\n');
                    }
                }
            } else {
                out.push_str("section = ");
                push_toml_str(&mut out, &g.section);
                out.push('\n');

                if let Some(ref dec) = g.decided {
                    out.push_str("decided = ");
                    push_toml_str(&mut out, dec);
                    out.push('\n');
                }
            }
        }
        out
    }
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

fn push_toml_str_array(out: &mut String, items: &[String]) {
    out.push('[');
    for (i, item) in items.iter().enumerate() {
        if i > 0 {
            out.push_str(", ");
        }
        push_toml_str(out, item);
    }
    out.push(']');
}

fn str_list(
    file: &str,
    gate_id: &str,
    field: &str,
    items: Vec<Spanned<Value>>,
) -> Result<Vec<String>, BaseRowError> {
    items
        .into_iter()
        .map(|sp| match sp.value {
            Value::Str(s) => Ok(s),
            other => Err(BaseRowError::new(
                file,
                Some(gate_id),
                format!("`{field}` elements must be strings, found {}", other.type_name()),
            )),
        })
        .collect()
}

fn parse_gate_row(tbl: Table, idx: usize, file: &str) -> Result<GateBase, BaseRowError> {
    let label = tbl
        .get("id")
        .and_then(Value::as_str)
        .filter(|s| !s.trim().is_empty())
        .map_or_else(|| format!("gate[{idx}]"), str::to_owned);
    let mut f = Fields::new(tbl, format!("gate[{label}]"));
    let err = |e: toml::TomlError| BaseRowError::new(file, Some(&label), e.to_string());

    let id = f.require_str("id").map_err(err)?;
    if id.trim().is_empty() {
        return Err(f.fail(BaseRowError::new(
            file,
            Some(&label),
            "`id` must be non-empty",
        )));
    }

    let kind = match f.opt_str("kind").map_err(err)? {
        Some(raw) => Some(BaseKind::parse(&raw, file, &id).map_err(|e| f.fail(e))?),
        None => None,
    };

    let section = f.require_str("section").map_err(err)?;
    if section.trim().is_empty() {
        return Err(f.fail(BaseRowError::new(
            file,
            Some(&id),
            "`section` must be non-empty",
        )));
    }

    if kind == Some(BaseKind::Kill) {
        for &forbidden in KILL_FORBIDDEN_FIELDS {
            if f.take(forbidden).is_some() {
                return Err(f.fail(BaseRowError::new(
                    file,
                    Some(&id),
                    format!("kill row must not carry measurement field `{forbidden}`"),
                )));
            }
        }
        let decided = f.opt_str("decided").map_err(err)?;
        if let Some(ref d) = decided {
            validate_milestone(d, file, &id, "decided").map_err(|e| f.fail(e))?;
        }
        // Tolerate register-layer kill fields owned by N40-01 / N40-03 / NAB-10.
        for additive in [
            "consumers",
            "sources",
            "trigger",
            "fallback",
            "mode",
            "reads",
            "decision",
            "reread",
            "scope_kind",
            "milestone",
            "recurring",
            "needs",
            "blocks_exit",
        ] {
            let _ = f.take(additive);
        }
        f.finish().map_err(err)?;
        return Ok(GateBase {
            id,
            kind,
            section,
            measured: None,
            decided,
        });
    }

    // Measured row (`admission` / `convergence` / `program` / untyped): all base fields required.
    let claim_sp = f.take("claim").ok_or_else(|| {
        f.fail(BaseRowError::new(
            file,
            Some(&id),
            "missing required base field `claim`",
        ))
    })?;
    let claim_raw = match claim_sp.value {
        Value::Str(s) => s,
        Value::Array(_) => {
            return Err(f.fail(BaseRowError::new(
                file,
                Some(&id),
                "`claim` must be a single string from the seven closed kinds, not an array",
            )));
        }
        other => {
            return Err(f.fail(BaseRowError::new(
                file,
                Some(&id),
                format!("`claim` must be a string, found {}", other.type_name()),
            )));
        }
    };
    let claim = Claim::parse(&claim_raw, file, &id).map_err(|e| f.fail(e))?;

    let claim_id = f.require_str("claim_id").map_err(err)?;
    if claim_id.trim().is_empty() {
        return Err(f.fail(BaseRowError::new(
            file,
            Some(&id),
            "`claim_id` must be non-empty",
        )));
    }
    let threshold = f.require_str("threshold").map_err(err)?;
    if threshold.trim().is_empty() {
        return Err(f.fail(BaseRowError::new(
            file,
            Some(&id),
            "`threshold` must be non-empty",
        )));
    }
    let milestone = f.require_str("milestone").map_err(err)?;
    validate_milestone(&milestone, file, &id, "milestone").map_err(|e| f.fail(e))?;

    let tier_raw = f.require_array("tier").map_err(err)?;
    let tier = str_list(file, &id, "tier", tier_raw).map_err(|e| f.fail(e))?;
    if tier.is_empty() {
        return Err(f.fail(BaseRowError::new(
            file,
            Some(&id),
            "`tier` must contain at least one entry",
        )));
    }
    let controls_raw = f.require_array("controls").map_err(err)?;
    let controls = str_list(file, &id, "controls", controls_raw).map_err(|e| f.fail(e))?;

    let raw_parts = f.opt_array("part").map_err(err)?.unwrap_or_default();
    let mut parts = Vec::with_capacity(raw_parts.len());
    for (pidx, sp) in raw_parts.into_iter().enumerate() {
        let ptbl = match sp.value {
            Value::Table(t) => t,
            other => {
                return Err(f.fail(BaseRowError::new(
                    file,
                    Some(&id),
                    format!("`part[{pidx}]` expected table, found {}", other.type_name()),
                )));
            }
        };
        parts.push(parse_part_base(ptbl, file, &id, pidx).map_err(|e| f.fail(e))?);
    }

    // Tolerate additive register-layer gate fields owned by N40-01 / NAB-10.
    for additive in [
        "consumers",
        "sources",
        "recurring",
        "needs",
        "blocks_exit",
        "foundation",
        "first_needed",
    ] {
        let _ = f.take(additive);
    }

    f.finish().map_err(err)?;
    Ok(GateBase {
        id,
        kind,
        section,
        measured: Some(MeasuredFields {
            claim,
            claim_id,
            threshold,
            milestone,
            tier,
            controls,
            parts,
        }),
        decided: None,
    })
}

fn parse_part_base(
    tbl: Table,
    file: &str,
    gate_id: &str,
    pidx: usize,
) -> Result<PartBase, BaseRowError> {
    let label = format!("{gate_id}.part[{pidx}]");
    let mut f = Fields::new(tbl, label.clone());
    let err = |e: toml::TomlError| BaseRowError::new(file, Some(&label), e.to_string());

    let milestone = f.require_str("milestone").map_err(err)?;
    validate_milestone(&milestone, file, &label, "milestone").map_err(|e| f.fail(e))?;

    let status_raw = f.require_str("status").map_err(err)?;
    let status = Status::parse(&status_raw, file, &label).map_err(|e| f.fail(e))?;

    let measured = f.require_str("measured").map_err(err)?;
    let ev_raw = f.require_array("evidence").map_err(err)?;
    let evidence = str_list(file, &label, "evidence", ev_raw).map_err(|e| f.fail(e))?;

    let rec_tbl = f.require_table("recorded").map_err(err)?;
    let mut rf = Fields::new(rec_tbl, format!("{label}.recorded"));
    let recorded = Recorded {
        date: rf.require_str("date").map_err(|e| f.fail(err(e)))?,
        session: rf.require_str("session").map_err(|e| f.fail(err(e)))?,
        implemented_by: rf
            .require_str("implemented_by")
            .map_err(|e| f.fail(err(e)))?,
    };
    rf.finish().map_err(|e| f.fail(err(e)))?;

    let detail = f.opt_str("detail").map_err(err)?;
    let kill_gate = f.opt_str("kill_gate").map_err(err)?;
    let fallback = f.opt_str("fallback").map_err(err)?;
    let open_question = f.opt_str("open_question").map_err(err)?;

    // Tolerate additive register-layer part provenance fields owned by N40-01.
    for additive in ["scope", "commit", "command", "receipt", "fingerprint", "seed"] {
        let _ = f.take(additive);
    }

    f.finish().map_err(err)?;
    Ok(PartBase {
        milestone,
        status,
        measured,
        evidence,
        recorded,
        detail,
        kill_gate,
        fallback,
        open_question,
    })
}
