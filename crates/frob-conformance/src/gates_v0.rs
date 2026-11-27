#![deny(unsafe_code)]

//! Minimal v0 reader for `registries/gates.toml` (`frob.gates/1`, BD-025).
//!
//! Uses `frob_core::registry::toml` as the single TOML parser (N34p2-01 / STD-TEST).
//! Status is recorded per `[[gate.part]]` and derived for the gate; additive fields
//! owned by `N38b-18` and `N40-01` are accepted so the v0 reader remains compatible
//! as the register grows.

use std::collections::BTreeSet;
use std::fmt;
use std::fs;
use std::path::Path;

use frob_core::registry::toml::{self, Fields, Spanned, Table, Value};

/// Workspace-relative path of the gate register.
pub const GATES_PATH: &str = "registries/gates.toml";

/// Expected top-level schema identifier for `registries/gates.toml`.
pub const GATES_SCHEMA: &str = "frob.gates/1";

/// Error returned when loading or parsing `registries/gates.toml`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateLoadError {
    pub file: String,
    pub row: Option<String>,
    pub message: String,
}

impl GateLoadError {
    fn new(file: &str, row: Option<&str>, message: impl Into<String>) -> Self {
        Self {
            file: file.to_owned(),
            row: row.map(str::to_owned),
            message: message.into(),
        }
    }
}

impl fmt::Display for GateLoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.row {
            Some(r) => write!(f, "{}: gate `{r}`: {}", self.file, self.message),
            None => write!(f, "{}: {}", self.file, self.message),
        }
    }
}

impl std::error::Error for GateLoadError {}

/// Status of one `[[gate.part]]` record (`frob.gates/1`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum GatePartStatus {
    NotRun,
    Pass,
    Fail,
    Killed,
    Blocked,
    NoClaim,
    Superseded,
}

impl GatePartStatus {
    pub fn parse(s: &str, file: &str, row: &str) -> Result<Self, GateLoadError> {
        match s {
            "not-run" => Ok(Self::NotRun),
            "pass" => Ok(Self::Pass),
            "fail" => Ok(Self::Fail),
            "killed" => Ok(Self::Killed),
            "blocked" => Ok(Self::Blocked),
            "no-claim" => Ok(Self::NoClaim),
            "superseded" => Ok(Self::Superseded),
            other => Err(GateLoadError::new(
                file,
                Some(row),
                format!("unknown gate part status `{other}`"),
            )),
        }
    }

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

    #[must_use]
    pub const fn is_not_run(self) -> bool {
        matches!(self, Self::NotRun)
    }

    #[must_use]
    pub const fn is_pass(self) -> bool {
        matches!(self, Self::Pass)
    }
}

/// Provenance block on a `[[gate.part]]` entry.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Recorded {
    pub date: String,
    pub session: String,
    pub implemented_by: String,
}

/// One milestone part of a gate record (`[[gate.part]]`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GatePart {
    pub milestone: String,
    pub status: GatePartStatus,
    pub measured: String,
    pub evidence: Vec<String>,
    pub recorded: Recorded,
    pub kill_gate: Option<String>,
    pub fallback: Option<String>,
    pub open_question: Option<String>,
    pub detail: Option<String>,
    pub receipt: Option<String>,
}

/// One `[[gate]]` row from `registries/gates.toml`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateRow {
    pub id: String,
    pub claim: String,
    pub claim_id: String,
    pub section: String,
    pub threshold: String,
    pub milestone: String,
    pub tier: Vec<String>,
    pub controls: Vec<String>,
    pub parts: Vec<GatePart>,
}

impl GateRow {
    /// Returns the gate's parts, synthesizing one `not-run` part at `self.milestone`
    /// when no `[[gate.part]]` entries are recorded yet.
    #[must_use]
    pub fn effective_parts(&self) -> Vec<GatePart> {
        if !self.parts.is_empty() {
            return self.parts.clone();
        }
        vec![GatePart {
            milestone: self.milestone.clone(),
            status: GatePartStatus::NotRun,
            measured: String::new(),
            evidence: Vec::new(),
            recorded: Recorded::default(),
            kill_gate: None,
            fallback: None,
            open_question: None,
            detail: None,
            receipt: None,
        }]
    }

    /// True if any part of this gate has a status other than `not-run`.
    #[must_use]
    pub fn has_non_not_run_part(&self) -> bool {
        self.parts.iter().any(|p| !p.status.is_not_run())
    }

    /// Derived gate status from its parts (BD-002 / N38b-18).
    #[must_use]
    pub fn derived_status(&self) -> GatePartStatus {
        match self.parts.last() {
            Some(p) => p.status,
            None => GatePartStatus::NotRun,
        }
    }
}

/// Loaded `registries/gates.toml` table.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct GateRegistry {
    rows: Vec<GateRow>,
}

impl GateRegistry {
    /// Load `registries/gates.toml` under `workspace_root`.
    pub fn load(workspace_root: &Path) -> Result<Self, GateLoadError> {
        let path = workspace_root.join(GATES_PATH);
        let bytes = fs::read(&path)
            .map_err(|e| GateLoadError::new(GATES_PATH, None, format!("read failed: {e}")))?;
        Self::parse_bytes(&bytes, GATES_PATH)
    }

    /// Parse `gates.toml` text directly.
    pub fn parse_str(input: &str, file: &str) -> Result<Self, GateLoadError> {
        Self::parse_bytes(input.as_bytes(), file)
    }

    /// Parse `gates.toml` bytes using `frob_core::registry::toml`.
    pub fn parse_bytes(bytes: &[u8], file: &str) -> Result<Self, GateLoadError> {
        let root = toml::parse(bytes).map_err(|e| GateLoadError::new(file, None, e.to_string()))?;
        let mut top = Fields::new(root, "gates");
        let schema = top
            .require_str("schema")
            .map_err(|e| GateLoadError::new(file, None, e.to_string()))?;
        if schema != GATES_SCHEMA {
            return Err(top.fail(GateLoadError::new(
                file,
                None,
                format!("expected schema `{GATES_SCHEMA}`, found `{schema}`"),
            )));
        }
        let raw_gates = top
            .opt_array("gate")
            .map_err(|e| GateLoadError::new(file, None, e.to_string()))?
            .unwrap_or_default();
        top.finish()
            .map_err(|e| GateLoadError::new(file, None, e.to_string()))?;

        let mut rows = Vec::with_capacity(raw_gates.len());
        let mut seen_ids = BTreeSet::new();
        for (idx, sp) in raw_gates.into_iter().enumerate() {
            let tbl = match sp.value {
                Value::Table(t) => t,
                other => {
                    return Err(GateLoadError::new(
                        file,
                        Some(&format!("gate[{idx}]")),
                        format!("expected table, found {}", other.type_name()),
                    ));
                }
            };
            let row = parse_gate_row(tbl, idx, file)?;
            if !seen_ids.insert(row.id.clone()) {
                return Err(GateLoadError::new(
                    file,
                    Some(&row.id),
                    format!("duplicate gate id `{}`", row.id),
                ));
            }
            rows.push(row);
        }
        Ok(Self { rows })
    }

    #[must_use]
    pub fn all(&self) -> &[GateRow] {
        &self.rows
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.rows.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    #[must_use]
    pub fn get(&self, id: &str) -> Option<&GateRow> {
        self.rows.iter().find(|g| g.id == id)
    }
}

/// Parse milestone string (`"M0"`..`"M8"`) to its ordinal number.
#[must_use]
pub fn parse_milestone_num(m: &str) -> Option<u32> {
    let digits = m.trim().strip_prefix('M')?;
    if digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    digits.parse::<u32>().ok()
}

fn validate_milestone(m: &str, file: &str, row: &str) -> Result<(), GateLoadError> {
    match parse_milestone_num(m) {
        Some(n) if n <= 8 => Ok(()),
        _ => Err(GateLoadError::new(
            file,
            Some(row),
            format!("invalid milestone `{m}` (expected M0..M8)"),
        )),
    }
}

fn str_list(
    file: &str,
    row: &str,
    key: &str,
    items: Vec<Spanned<Value>>,
) -> Result<Vec<String>, GateLoadError> {
    items
        .into_iter()
        .map(|sp| match sp.value {
            Value::Str(s) => Ok(s),
            other => Err(GateLoadError::new(
                file,
                Some(row),
                format!(
                    "`{key}` element expected string, found {}",
                    other.type_name()
                ),
            )),
        })
        .collect()
}

fn parse_gate_row(tbl: Table, idx: usize, file: &str) -> Result<GateRow, GateLoadError> {
    let label = tbl
        .get("id")
        .and_then(Value::as_str)
        .filter(|s| !s.is_empty())
        .map_or_else(|| format!("gate[{idx}]"), str::to_owned);
    let mut f = Fields::new(tbl, format!("gate[{label}]"));
    let err = |e: toml::TomlError| GateLoadError::new(file, Some(&label), e.to_string());

    let id = f.require_str("id").map_err(err)?;
    if id.trim().is_empty() {
        return Err(f.fail(GateLoadError::new(
            file,
            Some(&label),
            "`id` must be non-empty",
        )));
    }
    let claim = f.require_str("claim").map_err(err)?;
    if !matches!(
        claim.as_str(),
        "proof"
            | "known-answer"
            | "parity"
            | "fidelity"
            | "design-comparison"
            | "reality"
            | "performance"
    ) {
        return Err(f.fail(GateLoadError::new(
            file,
            Some(&id),
            format!("unknown claim kind `{claim}`"),
        )));
    }
    let claim_id = f.require_str("claim_id").map_err(err)?;
    let section = f.require_str("section").map_err(err)?;
    let threshold = f.require_str("threshold").map_err(err)?;
    let milestone = f.require_str("milestone").map_err(err)?;
    validate_milestone(&milestone, file, &id).map_err(|e| f.fail(e))?;

    let tier_items = f.require_array("tier").map_err(err)?;
    let tier = str_list(file, &id, "tier", tier_items).map_err(|e| f.fail(e))?;
    if tier.is_empty() {
        return Err(f.fail(GateLoadError::new(
            file,
            Some(&id),
            "`tier` must contain at least one entry",
        )));
    }
    let ctrl_items = f.require_array("controls").map_err(err)?;
    let controls = str_list(file, &id, "controls", ctrl_items).map_err(|e| f.fail(e))?;

    let raw_parts = f.opt_array("part").map_err(err)?.unwrap_or_default();
    let mut parts = Vec::with_capacity(raw_parts.len());
    for (pidx, sp) in raw_parts.into_iter().enumerate() {
        let ptbl = match sp.value {
            Value::Table(t) => t,
            other => {
                return Err(f.fail(GateLoadError::new(
                    file,
                    Some(&id),
                    format!("`part[{pidx}]` expected table, found {}", other.type_name()),
                )));
            }
        };
        parts.push(parse_gate_part(ptbl, file, &id, pidx).map_err(|e| f.fail(e))?);
    }

    // Tolerate additive register fields owned by N38b-18 / N40-01.
    for additive in ["kind", "consumers", "recurring", "needs"] {
        let _ = f.take(additive);
    }

    f.finish().map_err(err)?;
    Ok(GateRow {
        id,
        claim,
        claim_id,
        section,
        threshold,
        milestone,
        tier,
        controls,
        parts,
    })
}

fn parse_gate_part(
    tbl: Table,
    file: &str,
    gate_id: &str,
    pidx: usize,
) -> Result<GatePart, GateLoadError> {
    let label = format!("{gate_id}.part[{pidx}]");
    let mut f = Fields::new(tbl, label.clone());
    let err = |e: toml::TomlError| GateLoadError::new(file, Some(&label), e.to_string());

    let milestone = f.require_str("milestone").map_err(err)?;
    validate_milestone(&milestone, file, &label).map_err(|e| f.fail(e))?;
    let status_raw = f.require_str("status").map_err(err)?;
    let status = GatePartStatus::parse(&status_raw, file, &label).map_err(|e| f.fail(e))?;
    let measured = f.require_str("measured").map_err(err)?;
    let ev_items = f.require_array("evidence").map_err(err)?;
    let evidence = str_list(file, &label, "evidence", ev_items).map_err(|e| f.fail(e))?;

    let rec_tbl = f.require_table("recorded").map_err(err)?;
    let recorded = parse_recorded(rec_tbl, file, &label).map_err(|e| f.fail(e))?;

    let kill_gate = f.opt_str("kill_gate").map_err(err)?;
    let fallback = f.opt_str("fallback").map_err(err)?;
    let open_question = f.opt_str("open_question").map_err(err)?;
    let detail = f.opt_str("detail").map_err(err)?;
    let receipt = f.opt_str("receipt").map_err(err)?;

    // Tolerate additive N38b-18 / N40-01 part provenance fields.
    for additive in ["scope", "commit", "command", "fingerprint", "seed"] {
        let _ = f.take(additive);
    }

    f.finish().map_err(err)?;
    Ok(GatePart {
        milestone,
        status,
        measured,
        evidence,
        recorded,
        kill_gate,
        fallback,
        open_question,
        detail,
        receipt,
    })
}

fn parse_recorded(tbl: Table, file: &str, label: &str) -> Result<Recorded, GateLoadError> {
    let mut rf = Fields::new(tbl, format!("{label}.recorded"));
    let err = |e: toml::TomlError| GateLoadError::new(file, Some(label), e.to_string());
    let recorded = Recorded {
        date: rf.require_str("date").map_err(err)?,
        session: rf.require_str("session").map_err(err)?,
        implemented_by: rf.require_str("implemented_by").map_err(err)?,
    };
    rf.finish().map_err(err)?;
    Ok(recorded)
}
