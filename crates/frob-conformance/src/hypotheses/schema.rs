//! Schema, strict TOML loader, state counts, and scope binding for the hypothesis
//! verdict register (`frob.hypothesis/1`, `N00-10`).

use std::collections::BTreeSet;
use std::fmt;
use std::fs;
use std::path::Path;

use frob_core::registry::toml::{self, Fields, Spanned, Table, Value};

use crate::hypotheses::lint::Pattern;

/// Schema identifier of `crates/frob-conformance/registry/hypotheses.toml`.
pub const HYPOTHESES_SCHEMA: &str = "frob.hypothesis/1";

/// Embedded copy of the committed hypothesis verdict register.
pub const EMBEDDED_HYPOTHESES_TOML: &str = include_str!("../../registry/hypotheses.toml");

/// Finding code emitted when a work-package reason matches a REFUTED row's banned pattern.
pub const RULE_REFUTED_BOUND: &str = "hyp.refuted-bound";

/// Finding code emitted when a work-package gate list meets an ACTIVE row's `binds`.
pub const RULE_ACTIVE_UNSETTLED: &str = "hyp.active-unsettled";

/// Closed verdict state set of a hypothesis row (`N00-10`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VerdictState {
    Active,
    Narrowed,
    Refuted,
    Deferred,
}

impl VerdictState {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Active => "ACTIVE",
            Self::Narrowed => "NARROWED",
            Self::Refuted => "REFUTED",
            Self::Deferred => "DEFERRED",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "ACTIVE" => Some(Self::Active),
            "NARROWED" => Some(Self::Narrowed),
            "REFUTED" => Some(Self::Refuted),
            "DEFERRED" => Some(Self::Deferred),
            _ => None,
        }
    }
}

impl fmt::Display for VerdictState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Allowed evidence kinds for a history entry (`N00-10` step 2).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EvidenceKind {
    Pilot,
    GateMeasurement,
    PrimarySourceReread,
    Recomputation,
}

impl EvidenceKind {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pilot => "pilot",
            Self::GateMeasurement => "gate-measurement",
            Self::PrimarySourceReread => "primary-source-reread",
            Self::Recomputation => "recomputation",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "pilot" => Some(Self::Pilot),
            "gate-measurement" => Some(Self::GateMeasurement),
            "primary-source-reread" => Some(Self::PrimarySourceReread),
            "recomputation" => Some(Self::Recomputation),
            _ => None,
        }
    }
}

impl fmt::Display for EvidenceKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// One append-only history entry recording a verdict or reversal adoption.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoryEntry {
    pub date: String,
    pub state: VerdictState,
    pub evidence: String,
    pub kind: EvidenceKind,
    pub record: Option<String>,
    pub reversal_met: bool,
    pub owner_decision: Option<String>,
}

/// One hypothesis row in `frob.hypothesis/1`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HypothesisRow {
    pub id: String,
    pub statement: String,
    pub state: VerdictState,
    pub scope: Option<String>,
    pub survives: String,
    pub binds: Vec<String>,
    pub reversal: Option<String>,
    pub reversal_proposed: Option<String>,
    pub reversal_decision: Option<String>,
    pub banned: Vec<String>,
    pub settled_by: Vec<String>,
    pub history: Vec<HistoryEntry>,
}

impl HypothesisRow {
    /// True iff this row carries a recorded (owner-adopted) `reversal` condition AND has
    /// at least one history entry recording that condition met.
    ///
    /// A proposal-only row (`reversal.is_none()`) always returns `false` (fail-closed).
    #[must_use]
    pub fn has_effective_reversal_met(&self) -> bool {
        self.reversal.is_some() && self.history.iter().any(|h| h.reversal_met)
    }

    /// True iff this REFUTED row holds only `reversal_proposed` and no recorded `reversal`.
    #[must_use]
    pub fn is_proposal_only(&self) -> bool {
        self.reversal.is_none() && self.reversal_proposed.is_some()
    }
}

/// Status of an open disagreement record (`N00-10` step 5).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DisagreementStatus {
    Open,
    Settled,
}

impl DisagreementStatus {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Settled => "settled",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "open" => Some(Self::Open),
            "settled" => Some(Self::Settled),
            _ => None,
        }
    }
}

/// One `[[disagreement]]` record tracking an unresolved open disagreement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DisagreementRow {
    pub id: String,
    pub question: String,
    pub settled_by: Vec<String>,
    pub status: DisagreementStatus,
    pub record: Option<String>,
}

/// Exact row counts by [`VerdictState`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StateCounts {
    pub refuted: usize,
    pub deferred: usize,
    pub active: usize,
    pub narrowed: usize,
    pub total: usize,
}

/// Complete loaded hypothesis verdict register (`frob.hypothesis/1`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HypothesisRegister {
    pub schema: String,
    pub hypotheses: Vec<HypothesisRow>,
    pub disagreements: Vec<DisagreementRow>,
}

/// Load or schema validation error for `frob.hypothesis/1`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegisterError {
    Io(String),
    Toml(String),
    WrongSchema {
        observed: String,
    },
    DuplicateId {
        kind: &'static str,
        id: String,
    },
    InvalidField {
        row_id: String,
        field: String,
        reason: String,
    },
    EmptyHistory {
        row_id: String,
    },
    StateHistoryMismatch {
        row_id: String,
        state: VerdictState,
        last_history: VerdictState,
    },
    RefutedMissingReversal {
        row_id: String,
    },
    InvalidPattern {
        row_id: String,
        index: usize,
        pattern: String,
        reason: String,
    },
    SettledDisagreementMissingRecord {
        id: String,
    },
}

impl fmt::Display for RegisterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(e) => write!(f, "failed reading hypothesis register: {e}"),
            Self::Toml(e) => write!(f, "TOML parse/schema error: {e}"),
            Self::WrongSchema { observed } => {
                write!(f, "expected schema `{HYPOTHESES_SCHEMA}`, got `{observed}`")
            }
            Self::DuplicateId { kind, id } => write!(f, "duplicate {kind} id `{id}`"),
            Self::InvalidField {
                row_id,
                field,
                reason,
            } => {
                write!(f, "row `{row_id}` field `{field}`: {reason}")
            }
            Self::EmptyHistory { row_id } => {
                write!(f, "row `{row_id}` must carry at least one history entry")
            }
            Self::StateHistoryMismatch {
                row_id,
                state,
                last_history,
            } => write!(
                f,
                "row `{row_id}` state `{state}` disagrees with last history entry state `{last_history}`"
            ),
            Self::RefutedMissingReversal { row_id } => write!(
                f,
                "REFUTED row `{row_id}` must specify `reversal`, `reversal_proposed`, or both"
            ),
            Self::InvalidPattern {
                row_id,
                index,
                pattern,
                reason,
            } => write!(f, "row `{row_id}` banned[{index}] (`{pattern}`): {reason}"),
            Self::SettledDisagreementMissingRecord { id } => {
                write!(
                    f,
                    "settled disagreement `{id}` requires a non-empty `record`"
                )
            }
        }
    }
}

impl std::error::Error for RegisterError {}

/// Finding returned by [`scope_with_register`] and [`crate::hypotheses::scope`] (`N00-10` step 4).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScopeFinding {
    /// A work package's reason matches a `banned` pattern of a REFUTED row whose history
    /// has no effective reversal entry.
    RefutedBound {
        rule: &'static str,
        row_id: String,
        pattern_index: usize,
        pattern: String,
        reversal: Option<String>,
        reversal_proposed: Option<String>,
        proposal_only: bool,
    },
    /// A work package's listed gate/kill-gate identifier meets an ACTIVE row's `binds`.
    ActiveUnsettled {
        rule: &'static str,
        row_id: String,
        matched_bind: String,
        settled_by: Vec<String>,
    },
}

impl ScopeFinding {
    #[must_use]
    pub const fn rule(&self) -> &'static str {
        match self {
            Self::RefutedBound { rule, .. } | Self::ActiveUnsettled { rule, .. } => rule,
        }
    }

    #[must_use]
    pub fn row_id(&self) -> &str {
        match self {
            Self::RefutedBound { row_id, .. } | Self::ActiveUnsettled { row_id, .. } => row_id,
        }
    }
}

impl HypothesisRegister {
    /// Load and validate the embedded committed register.
    pub fn load_embedded() -> Result<Self, RegisterError> {
        Self::from_bytes(EMBEDDED_HYPOTHESES_TOML.as_bytes())
    }

    /// Load and validate a register file from disk.
    pub fn from_path(path: &Path) -> Result<Self, RegisterError> {
        let bytes =
            fs::read(path).map_err(|e| RegisterError::Io(format!("{}: {e}", path.display())))?;
        Self::from_bytes(&bytes)
    }

    /// Parse and validate a `frob.hypothesis/1` register from raw TOML bytes.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, RegisterError> {
        let root = toml::parse(bytes).map_err(|e| RegisterError::Toml(e.to_string()))?;
        let mut top = Fields::new(root, "hypotheses.toml");

        let schema = top
            .require_str("schema")
            .map_err(|e| RegisterError::Toml(e.to_string()))?
            .to_owned();
        if schema != HYPOTHESES_SCHEMA {
            return Err(top.fail(RegisterError::WrongSchema { observed: schema }));
        }

        let hyp_arr = top
            .opt_array("hypothesis")
            .map_err(|e| RegisterError::Toml(e.to_string()))?
            .unwrap_or_default();
        let dis_arr = top
            .opt_array("disagreement")
            .map_err(|e| RegisterError::Toml(e.to_string()))?
            .unwrap_or_default();
        top.finish()
            .map_err(|e| RegisterError::Toml(e.to_string()))?;

        let mut hypotheses = Vec::with_capacity(hyp_arr.len());
        let mut seen_hyp = BTreeSet::new();
        for (idx, sp) in hyp_arr.iter().enumerate() {
            let tbl = sp
                .value
                .as_table()
                .ok_or_else(|| RegisterError::InvalidField {
                    row_id: format!("hypothesis[{idx}]"),
                    field: "hypothesis".to_owned(),
                    reason: "expected table".to_owned(),
                })?;
            let row = parse_hypothesis_row(tbl.clone(), idx)?;
            if !seen_hyp.insert(row.id.clone()) {
                return Err(RegisterError::DuplicateId {
                    kind: "hypothesis",
                    id: row.id,
                });
            }
            hypotheses.push(row);
        }

        let mut disagreements = Vec::with_capacity(dis_arr.len());
        let mut seen_dis = BTreeSet::new();
        for (idx, sp) in dis_arr.iter().enumerate() {
            let tbl = sp
                .value
                .as_table()
                .ok_or_else(|| RegisterError::InvalidField {
                    row_id: format!("disagreement[{idx}]"),
                    field: "disagreement".to_owned(),
                    reason: "expected table".to_owned(),
                })?;
            let row = parse_disagreement_row(tbl.clone(), idx)?;
            if !seen_dis.insert(row.id.clone()) {
                return Err(RegisterError::DuplicateId {
                    kind: "disagreement",
                    id: row.id,
                });
            }
            disagreements.push(row);
        }

        Ok(Self {
            schema,
            hypotheses,
            disagreements,
        })
    }

    #[must_use]
    pub fn get(&self, id: &str) -> Option<&HypothesisRow> {
        self.hypotheses.iter().find(|h| h.id == id)
    }

    #[must_use]
    pub fn get_mut(&mut self, id: &str) -> Option<&mut HypothesisRow> {
        self.hypotheses.iter_mut().find(|h| h.id == id)
    }

    #[must_use]
    pub fn banned_patterns(&self, id: &str) -> Option<&[String]> {
        self.get(id).map(|h| h.banned.as_slice())
    }

    #[must_use]
    pub fn counts_by_state(&self) -> StateCounts {
        let mut counts = StateCounts {
            refuted: 0,
            deferred: 0,
            active: 0,
            narrowed: 0,
            total: self.hypotheses.len(),
        };
        for h in &self.hypotheses {
            match h.state {
                VerdictState::Refuted => counts.refuted += 1,
                VerdictState::Deferred => counts.deferred += 1,
                VerdictState::Active => counts.active += 1,
                VerdictState::Narrowed => counts.narrowed += 1,
            }
        }
        counts
    }

    #[must_use]
    pub fn open_disagreements(&self) -> Vec<&DisagreementRow> {
        self.disagreements
            .iter()
            .filter(|d| d.status == DisagreementStatus::Open)
            .collect()
    }
}

/// Evaluate scope bindings for a work package's stated `reason` and listed `gates` against
/// `register` (`N00-10` step 4, called by `N00-09` funding admission).
#[must_use]
pub fn scope_with_register(
    register: &HypothesisRegister,
    reason: &str,
    gates: &[&str],
) -> Vec<ScopeFinding> {
    let mut findings = Vec::new();

    for row in &register.hypotheses {
        if row.state == VerdictState::Refuted && !row.has_effective_reversal_met() {
            for (idx, pat_src) in row.banned.iter().enumerate() {
                if let Ok(pat) = Pattern::compile(pat_src)
                    && pat.is_match(reason)
                {
                    findings.push(ScopeFinding::RefutedBound {
                        rule: RULE_REFUTED_BOUND,
                        row_id: row.id.clone(),
                        pattern_index: idx,
                        pattern: pat_src.clone(),
                        reversal: row.reversal.clone(),
                        reversal_proposed: row.reversal_proposed.clone(),
                        proposal_only: row.is_proposal_only(),
                    });
                }
            }
        }
    }

    let gate_set: BTreeSet<&str> = gates.iter().copied().collect();
    for row in &register.hypotheses {
        if row.state == VerdictState::Active {
            for bind in &row.binds {
                if gate_set.contains(bind.as_str()) {
                    findings.push(ScopeFinding::ActiveUnsettled {
                        rule: RULE_ACTIVE_UNSETTLED,
                        row_id: row.id.clone(),
                        matched_bind: bind.clone(),
                        settled_by: row.settled_by.clone(),
                    });
                    break;
                }
            }
        }
    }

    findings
}

fn opt_non_empty_str(fields: &mut Fields, key: &str) -> Result<Option<String>, RegisterError> {
    match fields
        .opt_str(key)
        .map_err(|e| RegisterError::Toml(e.to_string()))?
    {
        Some(s) if !s.trim().is_empty() => Ok(Some(s)),
        _ => Ok(None),
    }
}

fn parse_str_array(
    items: &[Spanned<Value>],
    row_id: &str,
    field: &str,
    allow_empty: bool,
) -> Result<Vec<String>, RegisterError> {
    let mut out = Vec::with_capacity(items.len());
    for (i, sp) in items.iter().enumerate() {
        let s = sp
            .value
            .as_str()
            .ok_or_else(|| RegisterError::InvalidField {
                row_id: row_id.to_owned(),
                field: format!("{field}[{i}]"),
                reason: "expected string".to_owned(),
            })?;
        if s.trim().is_empty() {
            return Err(RegisterError::InvalidField {
                row_id: row_id.to_owned(),
                field: format!("{field}[{i}]"),
                reason: "string element must be non-empty".to_owned(),
            });
        }
        out.push(s.to_owned());
    }
    if !allow_empty && out.is_empty() {
        return Err(RegisterError::InvalidField {
            row_id: row_id.to_owned(),
            field: field.to_owned(),
            reason: "list must be non-empty".to_owned(),
        });
    }
    Ok(out)
}

fn is_valid_iso_date(s: &str) -> bool {
    let b = s.as_bytes();
    b.len() == 10
        && b[4] == b'-'
        && b[7] == b'-'
        && b[0..4].iter().all(u8::is_ascii_digit)
        && b[5..7].iter().all(u8::is_ascii_digit)
        && b[8..10].iter().all(u8::is_ascii_digit)
}

fn parse_history_entry(
    tbl: Table,
    row_id: &str,
    idx: usize,
) -> Result<HistoryEntry, RegisterError> {
    let ctx = format!("{row_id}.history[{idx}]");
    let mut f = Fields::new(tbl, &ctx);
    let date = f
        .require_str("date")
        .map_err(|e| RegisterError::Toml(e.to_string()))?;
    if !is_valid_iso_date(&date) {
        return Err(f.fail(RegisterError::InvalidField {
            row_id: row_id.to_owned(),
            field: format!("history[{idx}].date"),
            reason: format!("expected YYYY-MM-DD date, got `{date}`"),
        }));
    }
    let state_s = f
        .require_str("state")
        .map_err(|e| RegisterError::Toml(e.to_string()))?;
    let state = VerdictState::parse(&state_s).ok_or_else(|| {
        f.fail(RegisterError::InvalidField {
            row_id: row_id.to_owned(),
            field: format!("history[{idx}].state"),
            reason: format!("unknown verdict state `{state_s}`"),
        })
    })?;
    let evidence = f
        .require_str("evidence")
        .map_err(|e| RegisterError::Toml(e.to_string()))?;
    if evidence.trim().is_empty() {
        return Err(f.fail(RegisterError::InvalidField {
            row_id: row_id.to_owned(),
            field: format!("history[{idx}].evidence"),
            reason: "evidence must be non-empty".to_owned(),
        }));
    }
    let kind_s = f
        .require_str("kind")
        .map_err(|e| RegisterError::Toml(e.to_string()))?;
    let kind = EvidenceKind::parse(&kind_s).ok_or_else(|| {
        f.fail(RegisterError::InvalidField {
            row_id: row_id.to_owned(),
            field: format!("history[{idx}].kind"),
            reason: format!("unknown evidence kind `{kind_s}`"),
        })
    })?;
    let record = opt_non_empty_str(&mut f, "record")?;
    let reversal_met = f
        .opt_bool("reversal_met")
        .map_err(|e| RegisterError::Toml(e.to_string()))?
        .unwrap_or(false);
    let owner_decision = opt_non_empty_str(&mut f, "owner_decision")?;
    f.finish().map_err(|e| RegisterError::Toml(e.to_string()))?;

    Ok(HistoryEntry {
        date,
        state,
        evidence,
        kind,
        record,
        reversal_met,
        owner_decision,
    })
}

fn parse_hypothesis_row(tbl: Table, idx: usize) -> Result<HypothesisRow, RegisterError> {
    let ctx = format!("hypothesis[{idx}]");
    let mut f = Fields::new(tbl, &ctx);
    let id = f
        .require_str("id")
        .map_err(|e| RegisterError::Toml(e.to_string()))?;
    if id.trim().is_empty() {
        return Err(f.fail(RegisterError::InvalidField {
            row_id: ctx,
            field: "id".to_owned(),
            reason: "id must be non-empty".to_owned(),
        }));
    }
    let statement = f
        .require_str("statement")
        .map_err(|e| RegisterError::Toml(e.to_string()))?;
    if statement.trim().is_empty() {
        return Err(f.fail(RegisterError::InvalidField {
            row_id: id,
            field: "statement".to_owned(),
            reason: "statement must be non-empty".to_owned(),
        }));
    }
    let state_s = f
        .require_str("state")
        .map_err(|e| RegisterError::Toml(e.to_string()))?;
    let state = VerdictState::parse(&state_s).ok_or_else(|| {
        f.fail(RegisterError::InvalidField {
            row_id: id.clone(),
            field: "state".to_owned(),
            reason: format!("unknown verdict state `{state_s}`"),
        })
    })?;
    let scope = opt_non_empty_str(&mut f, "scope")?;
    let survives = f
        .require_str("survives")
        .map_err(|e| RegisterError::Toml(e.to_string()))?;
    if survives.trim().is_empty() {
        return Err(f.fail(RegisterError::InvalidField {
            row_id: id,
            field: "survives".to_owned(),
            reason: "survives must be non-empty".to_owned(),
        }));
    }
    let binds_arr = f
        .require_array("binds")
        .map_err(|e| RegisterError::Toml(e.to_string()))?;
    let binds = parse_str_array(&binds_arr, &id, "binds", false).map_err(|e| f.fail(e))?;
    let reversal = opt_non_empty_str(&mut f, "reversal")?;
    let reversal_proposed = opt_non_empty_str(&mut f, "reversal_proposed")?;
    let reversal_decision = opt_non_empty_str(&mut f, "reversal_decision")?;
    let banned_arr = f
        .opt_array("banned")
        .map_err(|e| RegisterError::Toml(e.to_string()))?
        .unwrap_or_default();
    let banned = parse_str_array(&banned_arr, &id, "banned", true).map_err(|e| f.fail(e))?;
    for (pidx, pat) in banned.iter().enumerate() {
        if let Err(reason) = Pattern::compile(pat) {
            return Err(f.fail(RegisterError::InvalidPattern {
                row_id: id,
                index: pidx,
                pattern: pat.clone(),
                reason,
            }));
        }
    }
    let settled_arr = f
        .require_array("settled_by")
        .map_err(|e| RegisterError::Toml(e.to_string()))?;
    let settled_by =
        parse_str_array(&settled_arr, &id, "settled_by", false).map_err(|e| f.fail(e))?;
    let hist_arr = f
        .require_array("history")
        .map_err(|e| RegisterError::Toml(e.to_string()))?;
    if hist_arr.is_empty() {
        return Err(f.fail(RegisterError::EmptyHistory { row_id: id }));
    }
    let mut history = Vec::with_capacity(hist_arr.len());
    for (hidx, sp) in hist_arr.iter().enumerate() {
        let htbl = sp.value.as_table().ok_or_else(|| {
            f.fail(RegisterError::InvalidField {
                row_id: id.clone(),
                field: format!("history[{hidx}]"),
                reason: "expected inline table".to_owned(),
            })
        })?;
        history.push(parse_history_entry(htbl.clone(), &id, hidx).map_err(|e| f.fail(e))?);
    }
    f.finish().map_err(|e| RegisterError::Toml(e.to_string()))?;

    let last_state = history.last().map(|h| h.state).unwrap_or(state);
    if last_state != state {
        return Err(RegisterError::StateHistoryMismatch {
            row_id: id,
            state,
            last_history: last_state,
        });
    }

    if state == VerdictState::Refuted && reversal.is_none() && reversal_proposed.is_none() {
        return Err(RegisterError::RefutedMissingReversal { row_id: id });
    }

    Ok(HypothesisRow {
        id,
        statement,
        state,
        scope,
        survives,
        binds,
        reversal,
        reversal_proposed,
        reversal_decision,
        banned,
        settled_by,
        history,
    })
}

fn parse_disagreement_row(tbl: Table, idx: usize) -> Result<DisagreementRow, RegisterError> {
    let ctx = format!("disagreement[{idx}]");
    let mut f = Fields::new(tbl, &ctx);
    let id = f
        .require_str("id")
        .map_err(|e| RegisterError::Toml(e.to_string()))?;
    let question = f
        .require_str("question")
        .map_err(|e| RegisterError::Toml(e.to_string()))?;
    if question.trim().is_empty() {
        return Err(f.fail(RegisterError::InvalidField {
            row_id: id,
            field: "question".to_owned(),
            reason: "question must be non-empty".to_owned(),
        }));
    }
    let settled_arr = f
        .require_array("settled_by")
        .map_err(|e| RegisterError::Toml(e.to_string()))?;
    let settled_by =
        parse_str_array(&settled_arr, &id, "settled_by", false).map_err(|e| f.fail(e))?;
    let status_s = f
        .require_str("status")
        .map_err(|e| RegisterError::Toml(e.to_string()))?;
    let status = DisagreementStatus::parse(&status_s).ok_or_else(|| {
        f.fail(RegisterError::InvalidField {
            row_id: id.clone(),
            field: "status".to_owned(),
            reason: format!("expected `open` or `settled`, got `{status_s}`"),
        })
    })?;
    let record = opt_non_empty_str(&mut f, "record")?;
    f.finish().map_err(|e| RegisterError::Toml(e.to_string()))?;

    if status == DisagreementStatus::Settled && record.is_none() {
        return Err(RegisterError::SettledDisagreementMissingRecord { id });
    }

    Ok(DisagreementRow {
        id,
        question,
        settled_by,
        status,
        record,
    })
}
