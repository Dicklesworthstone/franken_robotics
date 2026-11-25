#![deny(unsafe_code)]

//! Negative-control run history (`registry/control_history.ndjson`), milestone-exit
//! suspect-control review (`registry/control_reviews.toml`, Rule 4), and the
//! permanent-control procedure (Rule 5) (`N38b-04`).

use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fmt;
use std::fs::{self, OpenOptions};
use std::io::Write as _;
use std::path::Path;

use frob_core::canon::{self, CanonError, Canonical, Value as CanonValue};
use frob_core::json::{self, Limits};
use frob_core::registry::toml::{self, Fields, Value as TomlValue};
use frob_core::testkit::registry::{Control, ControlState, Controls};

use crate::gates_v0::{GateRegistry, parse_milestone_num};
use crate::reasons::HarnessReason;
use crate::tiers::Milestone;

/// Canonical workspace-relative path of the append-only control history log.
pub const CONTROL_HISTORY_PATH: &str = "crates/frob-conformance/registry/control_history.ndjson";

/// Canonical workspace-relative path of the milestone-exit suspect-control review file.
pub const CONTROL_REVIEWS_PATH: &str = "crates/frob-conformance/registry/control_reviews.toml";

/// Expected schema header for `registry/control_reviews.toml`.
pub const CONTROL_REVIEWS_SCHEMA: &str = "frob.gauntlet.control_reviews/1";

/// One append-only NDJSON record in `registry/control_history.ndjson` (`N38b-04`).
///
/// Written strictly by a recording session together with the gate row and
/// `last_seen`, never by the session that implemented the capability.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlHistoryEntry {
    pub id: String,
    pub gate: String,
    pub receipt: String,
    pub outcome: String,
    pub commit: String,
    pub session: String,
    pub date: String,
}

impl Canonical for ControlHistoryEntry {
    fn write(&self, w: CanonValue<'_>) -> Result<(), CanonError> {
        w.object(|o| {
            o.str("commit", &self.commit)?;
            o.str("date", &self.date)?;
            o.str("gate", &self.gate)?;
            o.str("id", &self.id)?;
            o.str("outcome", &self.outcome)?;
            o.str("receipt", &self.receipt)?;
            o.str("session", &self.session)?;
            Ok(())
        })
    }
}

impl ControlHistoryEntry {
    /// Serialize as one canonical JSON line (no trailing newline).
    pub fn to_canonical_line(&self) -> Result<String, String> {
        let cb =
            canon::to_canonical(self).map_err(|e| format!("canonicalize history line: {e}"))?;
        std::str::from_utf8(cb.as_bytes())
            .map(str::to_owned)
            .map_err(|e| format!("history UTF-8: {e}"))
    }
}

/// Append one canonical history record to `path` (creates parent dirs if needed; never truncates).
pub fn append_history_entry(path: &Path, entry: &ControlHistoryEntry) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("create history dir `{}`: {e}", parent.display()))?;
    }
    let line = entry.to_canonical_line()?;
    let mut f = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .map_err(|e| format!("open `{}`: {e}", path.display()))?;
    writeln!(f, "{line}").map_err(|e| format!("write `{}`: {e}", path.display()))?;
    f.sync_all()
        .map_err(|e| format!("sync `{}`: {e}", path.display()))?;
    Ok(())
}

/// Parse NDJSON bytes of `control_history.ndjson` strictly through `frob_core::json::admit`.
pub fn parse_history_ndjson(raw: &str, file: &str) -> Result<Vec<ControlHistoryEntry>, String> {
    let mut out = Vec::new();
    for (idx, line) in raw.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let line_no = idx + 1;
        let doc = json::admit(trimmed.as_bytes(), &Limits::default())
            .map_err(|e| format!("{file}:{line_no}: invalid JSON: {e}"))?;
        let root = doc.value();
        let req = |k: &str| -> Result<String, String> {
            root.get(k)
                .and_then(|v| v.as_str())
                .filter(|s| !s.is_empty())
                .map(str::to_owned)
                .ok_or_else(|| format!("{file}:{line_no}: missing or empty string `{k}`"))
        };
        let outcome = req("outcome")?;
        if !matches!(
            outcome.as_str(),
            "caught" | "missed" | "caught-unexpected" | "stale" | "not-yet-due" | "obligation"
        ) {
            return Err(format!(
                "{file}:{line_no}: invalid history outcome `{outcome}`"
            ));
        }
        out.push(ControlHistoryEntry {
            id: req("id")?,
            gate: req("gate")?,
            receipt: req("receipt")?,
            outcome,
            commit: req("commit")?,
            session: req("session")?,
            date: req("date")?,
        });
    }
    Ok(out)
}

/// Load `crates/frob-conformance/registry/control_history.ndjson` under `workspace_root`.
/// Missing file yields an empty history vector.
pub fn load_history(workspace_root: &Path) -> Result<Vec<ControlHistoryEntry>, String> {
    let path = workspace_root.join(CONTROL_HISTORY_PATH);
    if !path.is_file() {
        return Ok(Vec::new());
    }
    let raw = fs::read_to_string(&path).map_err(|e| format!("read `{}`: {e}", path.display()))?;
    parse_history_ndjson(&raw, CONTROL_HISTORY_PATH)
}

/// Typed disposition for a suspect control at a milestone exit (`N38b-04`, Rule 4).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Disposition {
    /// Defect re-stated / replanted so the gate observes it.
    Replanted,
    /// Gate test strengthened so it now catches the defect.
    GateStrengthened,
    /// Control withdrawn with an explicit recorded reason.
    Withdrawn,
}

impl Disposition {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Replanted => "replanted",
            Self::GateStrengthened => "gate-strengthened",
            Self::Withdrawn => "withdrawn",
        }
    }

    #[must_use]
    pub fn parse(raw: &str) -> Option<Self> {
        match raw.trim() {
            "replanted" => Some(Self::Replanted),
            "gate-strengthened" => Some(Self::GateStrengthened),
            "withdrawn" => Some(Self::Withdrawn),
            _ => None,
        }
    }
}

impl fmt::Display for Disposition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// One `[[review]]` entry in `registry/control_reviews.toml`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlReviewRecord {
    pub id: String,
    pub disposition: Disposition,
    pub reason: String,
    pub milestone: String,
    pub session: String,
    pub date: String,
}

/// Parsed `registry/control_reviews.toml`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ControlReviews {
    rows: Vec<ControlReviewRecord>,
}

impl ControlReviews {
    /// Parse `control_reviews.toml` strictly via `frob_core::registry::toml`.
    pub fn parse_str(input: &str, file: &str) -> Result<Self, String> {
        let root = toml::parse(input.as_bytes()).map_err(|e| format!("{file}: {e}"))?;
        let mut top = Fields::new(root, "control_reviews");
        let schema = top
            .require_str("schema")
            .map_err(|e| format!("{file}: {e}"))?;
        let items = top
            .opt_array("review")
            .map_err(|e| format!("{file}: {e}"))?
            .unwrap_or_default();
        top.finish().map_err(|e| format!("{file}: {e}"))?;
        if schema != CONTROL_REVIEWS_SCHEMA {
            return Err(format!(
                "{file}: expected schema `{CONTROL_REVIEWS_SCHEMA}`, found `{schema}`"
            ));
        }

        let mut rows = Vec::with_capacity(items.len());
        for (idx, sp) in items.into_iter().enumerate() {
            let tbl = match sp.value {
                TomlValue::Table(t) => t,
                other => {
                    return Err(format!(
                        "{file}: review[{idx}] expected table, found {}",
                        other.type_name()
                    ));
                }
            };
            let mut rf = Fields::new(tbl, format!("review[{idx}]"));
            let id = rf.require_str("id").map_err(|e| format!("{file}: {e}"))?;
            let disp_raw = rf
                .require_str("disposition")
                .map_err(|e| format!("{file}: {e}"))?;
            let reason = rf
                .opt_str("reason")
                .map_err(|e| format!("{file}: {e}"))?
                .unwrap_or_default();
            let milestone = rf
                .opt_str("milestone")
                .map_err(|e| format!("{file}: {e}"))?
                .unwrap_or_default();
            let session = rf
                .opt_str("session")
                .map_err(|e| format!("{file}: {e}"))?
                .unwrap_or_default();
            let date = rf
                .opt_str("date")
                .map_err(|e| format!("{file}: {e}"))?
                .unwrap_or_default();
            rf.finish().map_err(|e| format!("{file}: {e}"))?;

            let Some(disposition) = Disposition::parse(&disp_raw) else {
                return Err(format!(
                    "{file}: review[{idx}] (`{id}`): unknown disposition `{disp_raw}` \
                     (expected `replanted`, `gate-strengthened`, or `withdrawn`)"
                ));
            };
            if disposition == Disposition::Withdrawn && reason.trim().is_empty() {
                return Err(format!(
                    "{file}: review[{idx}] (`{id}`): `withdrawn` disposition requires a non-empty `reason`"
                ));
            }
            rows.push(ControlReviewRecord {
                id,
                disposition,
                reason,
                milestone,
                session,
                date,
            });
        }
        Ok(Self { rows })
    }

    #[must_use]
    pub fn all(&self) -> &[ControlReviewRecord] {
        &self.rows
    }

    /// Find the latest disposition recorded for control `id` applicable at `milestone`.
    #[must_use]
    pub fn disposition_for(&self, id: &str, milestone: Milestone) -> Option<&ControlReviewRecord> {
        let m_num = u32::from(milestone.number());
        self.rows.iter().rev().find(|r| {
            if r.id != id {
                return false;
            }
            if r.milestone.trim().is_empty() {
                return true;
            }
            parse_milestone_num(&r.milestone).is_some_and(|rm| rm <= m_num)
        })
    }
}

/// Load `crates/frob-conformance/registry/control_reviews.toml` under `workspace_root`.
pub fn load_reviews(workspace_root: &Path) -> Result<ControlReviews, String> {
    let path = workspace_root.join(CONTROL_REVIEWS_PATH);
    if !path.is_file() {
        return Ok(ControlReviews::default());
    }
    let raw = fs::read_to_string(&path).map_err(|e| format!("read `{}`: {e}", path.display()))?;
    ControlReviews::parse_str(&raw, CONTROL_REVIEWS_PATH)
}

/// Reason a control is flagged as suspect in [`SuspectReport`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SuspectCause {
    /// Zero `caught` records exist in `control_history.ndjson` for this control.
    NeverCaught,
    /// Control is currently stale (missing feature/fixture or unregistered annex driver).
    Stale,
}

impl SuspectCause {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NeverCaught => "never-caught",
            Self::Stale => HarnessReason::Stale.as_str(),
        }
    }
}

/// One control listed in the milestone-exit suspect report.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SuspectEntry {
    pub id: String,
    pub gates: Vec<String>,
    pub due: Option<String>,
    pub cause: SuspectCause,
    pub disposition: Option<Disposition>,
}

/// Deterministic milestone-exit suspect-control report (Rule 4, `tests/control_review.rs`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SuspectReport {
    pub milestone: Milestone,
    pub due_gates: Vec<String>,
    pub due_controls: Vec<String>,
    pub suspects: Vec<SuspectEntry>,
    pub undisposed: Vec<SuspectEntry>,
}

impl SuspectReport {
    /// Returns `true` when at least one suspect control lacks a typed disposition,
    /// blocking the milestone exit (`GATE-GAUNT-01`, `reason=suspect-undisposed`).
    #[must_use]
    pub fn blocks_exit(&self) -> bool {
        !self.undisposed.is_empty()
    }

    /// Render a deterministic text summary of the review report.
    #[must_use]
    pub fn format_report(&self) -> String {
        let mut out = format!(
            "control_review milestone={} due_gates={} due_controls={} suspects={} undisposed={}\n",
            self.milestone,
            self.due_gates.len(),
            self.due_controls.len(),
            self.suspects.len(),
            self.undisposed.len()
        );
        for s in &self.suspects {
            let disp = match s.disposition {
                Some(d) => format!("disposition={}", d.as_str()),
                None => format!("reason={}", HarnessReason::SuspectUndisposed.as_str()),
            };
            out.push_str(&format!(
                "  suspect {} cause={} gates=[{}] {disp}\n",
                s.id,
                s.cause.as_str(),
                s.gates.join(",")
            ));
        }
        out
    }
}

/// Resolve the milestone under review from `FROB_MILESTONE` (never from CLI positional args).
///
/// When `FROB_MILESTONE` is unset or empty, takes the current milestone from `gates`
/// (minimum milestone present in `registries/gates.toml`, defaulting to `M0`) — never "no controls".
pub fn resolve_review_milestone(
    env_override: Option<&str>,
    gates: &GateRegistry,
) -> Result<Milestone, String> {
    let raw = match env_override {
        Some(v) => Some(v.to_owned()),
        None => env::var("FROB_MILESTONE").ok(),
    };
    if let Some(val) = raw.filter(|s| !s.trim().is_empty()) {
        return Milestone::parse(val.trim())
            .ok_or_else(|| format!("invalid FROB_MILESTONE `{val}` (expected M0..M8)"));
    }
    let current = gates
        .all()
        .iter()
        .filter_map(|g| Milestone::parse(&g.milestone))
        .min()
        .unwrap_or(Milestone::M0);
    Ok(current)
}

/// Build the Rule-4 suspect-control report for `milestone`.
///
/// Lists every runnable control attached to gates due `<= milestone` (whose own
/// `due` is absent or `<= milestone`) that either has zero `caught` records in
/// `history` or is in `live_stale_ids`. Any listed control lacking a valid typed
/// disposition in `reviews` enters `undisposed` and blocks the exit.
#[must_use]
pub fn build_suspect_report(
    milestone: Milestone,
    gates: &GateRegistry,
    controls: &Controls,
    history: &[ControlHistoryEntry],
    reviews: &ControlReviews,
    live_stale_ids: &BTreeSet<String>,
) -> SuspectReport {
    let m_num = u32::from(milestone.number());

    let mut due_gates_set: BTreeSet<String> = BTreeSet::new();
    let mut ctrl_to_due_gates: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();

    for g in gates.all() {
        let gate_due = g
            .effective_parts()
            .iter()
            .any(|p| parse_milestone_num(&p.milestone).is_some_and(|pm| pm <= m_num))
            || parse_milestone_num(&g.milestone).is_some_and(|gm| gm <= m_num);
        if !gate_due {
            continue;
        }
        due_gates_set.insert(g.id.clone());
        for cid in &g.controls {
            ctrl_to_due_gates
                .entry(cid.clone())
                .or_default()
                .insert(g.id.clone());
        }
    }

    // Collect every control that has at least one `caught` line in recorded history.
    let caught_ids: BTreeSet<&str> = history
        .iter()
        .filter(|h| h.outcome == "caught")
        .map(|h| h.id.as_str())
        .collect();

    let mut due_controls = Vec::new();
    let mut suspects = Vec::new();
    let mut undisposed = Vec::new();

    let mut sorted_controls: Vec<&Control> = controls.all().iter().collect();
    sorted_controls.sort_by(|a, b| a.id.cmp(&b.id));

    for ctrl in sorted_controls {
        if ctrl.state == ControlState::Obligation {
            continue;
        }
        // Check if attached to at least one due gate.
        let mut attached_due: BTreeSet<String> =
            ctrl_to_due_gates.get(&ctrl.id).cloned().unwrap_or_default();
        for g in &ctrl.gate {
            if due_gates_set.contains(g) {
                attached_due.insert(g.clone());
            }
        }
        if attached_due.is_empty() {
            continue;
        }
        // Skip controls whose own `due` is strictly after the milestone under review.
        if let Some(due_s) = ctrl.due.as_deref()
            && parse_milestone_num(due_s).is_some_and(|dm| dm > m_num)
        {
            continue;
        }

        due_controls.push(ctrl.id.clone());

        let cause = if live_stale_ids.contains(&ctrl.id) {
            Some(SuspectCause::Stale)
        } else if !caught_ids.contains(ctrl.id.as_str()) {
            Some(SuspectCause::NeverCaught)
        } else {
            None
        };

        if let Some(cause) = cause {
            let disposition = reviews
                .disposition_for(&ctrl.id, milestone)
                .map(|r| r.disposition);
            let entry = SuspectEntry {
                id: ctrl.id.clone(),
                gates: attached_due.into_iter().collect(),
                due: ctrl.due.clone(),
                cause,
                disposition,
            };
            if entry.disposition.is_none() {
                undisposed.push(entry.clone());
            }
            suspects.push(entry);
        }
    }

    SuspectReport {
        milestone,
        due_gates: due_gates_set.into_iter().collect(),
        due_controls,
        suspects,
        undisposed,
    }
}
