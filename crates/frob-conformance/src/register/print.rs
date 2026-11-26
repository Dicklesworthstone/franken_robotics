//! Pure README honest-status table (BD-002, N38b-20, INV-GAUNT-06).
#![deny(unsafe_code)]

use std::collections::BTreeSet;
use std::fmt;

use frob_core::registry::toml::{self, Fields, Spanned, Value};

/// Error returned by status-table generation, README block verification, and
/// registry/schema validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatusTableError {
    pub context: String,
    pub message: String,
}

impl StatusTableError {
    #[must_use]
    pub fn new(context: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            context: context.into(),
            message: message.into(),
        }
    }
}

impl fmt::Display for StatusTableError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.context, self.message)
    }
}

impl std::error::Error for StatusTableError {}

/// The seven closed claim kinds of `frob.gates/1`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ClaimKind {
    Proof,
    KnownAnswer,
    Parity,
    Fidelity,
    DesignComparison,
    Reality,
    Performance,
}

impl ClaimKind {
    pub fn parse(raw: &str) -> Option<Self> {
        match raw {
            "proof" => Some(Self::Proof),
            "known-answer" => Some(Self::KnownAnswer),
            "parity" => Some(Self::Parity),
            "fidelity" => Some(Self::Fidelity),
            "design-comparison" => Some(Self::DesignComparison),
            "reality" => Some(Self::Reality),
            "performance" => Some(Self::Performance),
            _ => None,
        }
    }

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
}

/// The seven closed per-part status tokens of `frob.gates/1`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PartStatus {
    NotRun,
    Pass,
    Fail,
    Killed,
    Blocked,
    NoClaim,
    Superseded,
}

impl PartStatus {
    pub fn parse(raw: &str) -> Option<Self> {
        match raw {
            "not-run" => Some(Self::NotRun),
            "pass" => Some(Self::Pass),
            "fail" => Some(Self::Fail),
            "killed" => Some(Self::Killed),
            "blocked" => Some(Self::Blocked),
            "no-claim" => Some(Self::NoClaim),
            "superseded" => Some(Self::Superseded),
            _ => None,
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
}

/// Session attribution metadata recorded on a gate part.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordedMeta {
    pub date: String,
    pub session: String,
    /// Sorted, comma-joined session identifiers of all implementers.
    pub implemented_by: String,
}

/// One milestone-scoped part (`[[gate.part]]`) of a gate row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GatePart {
    pub milestone: String,
    pub status: PartStatus,
    pub measured: String,
    pub evidence: Vec<String>,
    pub recorded: RecordedMeta,
    pub kill_gate: Option<String>,
    pub fallback: Option<String>,
    pub open_question: Option<String>,
    pub detail: Option<String>,
}

/// One gate entry (`[[gate]]`) of `registries/gates.toml` (schema `frob.gates/1`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateRow {
    pub id: String,
    pub claim: ClaimKind,
    pub claim_id: String,
    pub section: String,
    pub threshold: String,
    pub milestone: String,
    pub tier: Vec<String>,
    pub controls: Vec<String>,
    pub parts: Vec<GatePart>,
}

/// Parsed `registries/gates.toml` document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateRegistry {
    pub schema: String,
    pub gates: Vec<GateRow>,
}

impl Default for GateRegistry {
    fn default() -> Self {
        Self {
            schema: "frob.gates/1".to_owned(),
            gates: Vec::new(),
        }
    }
}

fn milestone_rank(m: &str) -> Option<u8> {
    match m {
        "M0" => Some(0),
        "M1" => Some(1),
        "M2" => Some(2),
        "M3" => Some(3),
        "M4" => Some(4),
        "M5" => Some(5),
        "M6" => Some(6),
        "M7" => Some(7),
        "M8" => Some(8),
        _ => None,
    }
}

/// Validate and extract the strings of a TOML array for a registry reader.
pub fn extract_str_list(
    items: &[Spanned<Value>],
    key: &str,
    ctx: &str,
    non_empty_items: bool,
) -> Result<Vec<String>, StatusTableError> {
    let mut out = Vec::with_capacity(items.len());
    for (idx, sp) in items.iter().enumerate() {
        let Some(s) = sp.value.as_str() else {
            return Err(StatusTableError::new(
                ctx,
                format!(
                    "{key}[{idx}] at {}:{}: expected string, found {}",
                    sp.line,
                    sp.column,
                    sp.value.type_name()
                ),
            ));
        };
        if non_empty_items && s.trim().is_empty() {
            return Err(StatusTableError::new(
                ctx,
                format!("{key}[{idx}] must be a non-empty string"),
            ));
        }
        out.push(s.to_owned());
    }
    Ok(out)
}

/// Validate every schema rule on one [`GateRow`] and all of its [`GatePart`]s.
pub fn validate_gate_row(gate: &GateRow) -> Result<(), StatusTableError> {
    let ctx = format!("gate `{}`", gate.id);
    if gate.id.trim().is_empty() {
        return Err(StatusTableError::new("gate", "gate id must be non-empty"));
    }
    if gate.claim_id.trim().is_empty() {
        return Err(StatusTableError::new(&ctx, "claim_id must be non-empty"));
    }
    if gate.section.trim().is_empty() {
        return Err(StatusTableError::new(&ctx, "section must be non-empty"));
    }
    if gate.threshold.trim().is_empty() {
        return Err(StatusTableError::new(&ctx, "threshold must be non-empty"));
    }
    if milestone_rank(&gate.milestone).is_none() {
        return Err(StatusTableError::new(
            &ctx,
            format!("invalid milestone `{}` (expected M0..=M8)", gate.milestone),
        ));
    }
    if gate.tier.is_empty() {
        return Err(StatusTableError::new(&ctx, "tier list must be non-empty"));
    }

    for (idx, part) in gate.parts.iter().enumerate() {
        let pctx = format!("gate `{}` part[{idx}] ({})", gate.id, part.milestone);
        if milestone_rank(&part.milestone).is_none() {
            return Err(StatusTableError::new(
                &pctx,
                format!(
                    "invalid part milestone `{}` (expected M0..=M8)",
                    part.milestone
                ),
            ));
        }

        match part.status {
            PartStatus::Pass => {
                if gate.controls.is_empty() {
                    return Err(StatusTableError::new(
                        &pctx,
                        format!(
                            "status `pass` on gate `{}` is invalid when `controls` is empty",
                            gate.id
                        ),
                    ));
                }
                if part.measured.trim().is_empty() {
                    return Err(StatusTableError::new(
                        &pctx,
                        format!(
                            "status `pass` on gate `{}` is invalid when `measured` is empty",
                            gate.id
                        ),
                    ));
                }
                if part.measured == gate.threshold {
                    return Err(StatusTableError::new(
                        &pctx,
                        "status `pass` requires an actual measurement; `measured` is byte-equal to `threshold`",
                    ));
                }
                let session = part.recorded.session.trim();
                if session.is_empty() {
                    return Err(StatusTableError::new(
                        &pctx,
                        format!(
                            "status `pass` on gate `{}` requires non-empty `recorded.session`",
                            gate.id
                        ),
                    ));
                }
                let self_certified = part
                    .recorded
                    .implemented_by
                    .split(',')
                    .map(str::trim)
                    .any(|imp| !imp.is_empty() && imp == session);
                if self_certified {
                    return Err(StatusTableError::new(
                        &pctx,
                        format!(
                            "self-certified pass refused for gate `{}`: recorded.session `{session}` appears in recorded.implemented_by `{}`",
                            gate.id, part.recorded.implemented_by
                        ),
                    ));
                }
            }
            PartStatus::Killed => {
                let has_pair = part
                    .kill_gate
                    .as_deref()
                    .is_some_and(|s| !s.trim().is_empty())
                    && part
                        .fallback
                        .as_deref()
                        .is_some_and(|s| !s.trim().is_empty());
                let has_detail = part.detail.as_deref().is_some_and(|s| !s.trim().is_empty());
                if !has_pair && !has_detail {
                    return Err(StatusTableError::new(
                        &pctx,
                        format!(
                            "status `killed` on gate `{}` must name the kill gate and fallback (via `kill_gate` + `fallback` or `detail`)",
                            gate.id
                        ),
                    ));
                }
            }
            PartStatus::Blocked => {
                let has_q = part
                    .open_question
                    .as_deref()
                    .is_some_and(|s| !s.trim().is_empty());
                let has_detail = part.detail.as_deref().is_some_and(|s| !s.trim().is_empty());
                if !has_q && !has_detail {
                    return Err(StatusTableError::new(
                        &pctx,
                        format!(
                            "status `blocked` on gate `{}` must name the blocking open question (via `open_question` or `detail`)",
                            gate.id
                        ),
                    ));
                }
            }
            PartStatus::NoClaim => {
                if gate.claim != ClaimKind::Reality {
                    return Err(StatusTableError::new(
                        &pctx,
                        format!(
                            "status `no-claim` on gate `{}` is valid only for `reality` gates, found `{}`",
                            gate.id,
                            gate.claim.as_str()
                        ),
                    ));
                }
            }
            PartStatus::NotRun | PartStatus::Fail | PartStatus::Superseded => {}
        }
    }
    Ok(())
}

/// Parse and validate `registries/gates.toml` using `frob-core::registry::toml`.
pub fn parse_gates_toml(bytes: &[u8]) -> Result<GateRegistry, StatusTableError> {
    let root =
        toml::parse(bytes).map_err(|e| StatusTableError::new("gates.toml", e.to_string()))?;
    let mut rf = Fields::new(root, "gates.toml");
    let schema = rf
        .require_str("schema")
        .map_err(|e| StatusTableError::new("gates.toml", e.to_string()))?;
    let raw_gates_arr = rf
        .opt_array("gate")
        .map_err(|e| StatusTableError::new("gates.toml", e.to_string()))?;
    // These extension tables are validated by the conformance register.
    for field in [
        "retired_ids",
        "amendment",
        "shared_fixture",
        "related_threshold",
        "fallback_exit",
        "variant",
    ] {
        let _ = rf
            .opt_array(field)
            .map_err(|e| StatusTableError::new("gates.toml", e.to_string()))?;
    }
    rf.finish()
        .map_err(|e| StatusTableError::new("gates.toml", e.to_string()))?;

    if schema != "frob.gates/1" {
        return Err(StatusTableError::new(
            "gates.toml",
            format!("unexpected schema `{schema}` (expected `frob.gates/1`)"),
        ));
    }
    let raw_gates = match raw_gates_arr {
        Some(items) => {
            let mut out = Vec::with_capacity(items.len());
            for (idx, sp) in items.into_iter().enumerate() {
                match sp.value {
                    Value::Table(t) => out.push(t),
                    other => {
                        return Err(StatusTableError::new(
                            "gates.toml",
                            format!(
                                "gate[{idx}] at {}:{}: expected table, found {}",
                                sp.line,
                                sp.column,
                                other.type_name()
                            ),
                        ));
                    }
                }
            }
            out
        }
        None => Vec::new(),
    };

    let mut seen_ids = BTreeSet::new();
    let mut gates = Vec::with_capacity(raw_gates.len());

    for (idx, gtbl) in raw_gates.into_iter().enumerate() {
        let ctx = format!("gates.toml:gate[{idx}]");
        let mut gf = Fields::new(gtbl, &ctx);
        let id = gf
            .require_str("id")
            .map_err(|e| StatusTableError::new(&ctx, e.to_string()))?;
        let section = gf
            .require_str("section")
            .map_err(|e| StatusTableError::new(&ctx, e.to_string()))?;
        let kind = gf
            .opt_str("kind")
            .map_err(|e| StatusTableError::new(&ctx, e.to_string()))?;
        if !seen_ids.insert(id.clone()) {
            return Err(gf.fail(StatusTableError::new(
                &ctx,
                format!("duplicate gate id `{id}`"),
            )));
        }
        for field in [
            "consumers",
            "sources",
            "recurring",
            "needs",
            "blocks_exit",
            "foundation",
            "first_needed",
            "trigger",
            "fallback",
            "decided",
            "mode",
            "reads",
            "decision",
            "reread",
            "scope_kind",
        ] {
            let _ = gf.take(field);
        }
        match kind.as_deref() {
            Some("kill") => {
                if id.trim().is_empty() || section.trim().is_empty() {
                    return Err(gf.fail(StatusTableError::new(
                        &ctx,
                        "kill row requires non-empty id and section",
                    )));
                }
                // Kill rows carry decisions, not claims, and have no status table row.
                let _ = gf.take("milestone");
                gf.finish()
                    .map_err(|e| StatusTableError::new(&ctx, e.to_string()))?;
                continue;
            }
            None | Some("admission" | "convergence" | "program") => {}
            Some(other) => {
                return Err(gf.fail(StatusTableError::new(
                    &ctx,
                    format!("unknown gate kind `{other}`"),
                )));
            }
        }
        let claim_s = gf
            .require_str("claim")
            .map_err(|e| StatusTableError::new(&ctx, e.to_string()))?;
        let claim_id = gf
            .require_str("claim_id")
            .map_err(|e| StatusTableError::new(&ctx, e.to_string()))?;
        let threshold = gf
            .require_str("threshold")
            .map_err(|e| StatusTableError::new(&ctx, e.to_string()))?;
        let milestone = gf
            .require_str("milestone")
            .map_err(|e| StatusTableError::new(&ctx, e.to_string()))?;
        let raw_tier = gf
            .require_array("tier")
            .map_err(|e| StatusTableError::new(&ctx, e.to_string()))?;
        let raw_controls = gf
            .require_array("controls")
            .map_err(|e| StatusTableError::new(&ctx, e.to_string()))?;
        let raw_parts_arr = gf
            .opt_array("part")
            .map_err(|e| StatusTableError::new(&ctx, e.to_string()))?;
        gf.finish()
            .map_err(|e| StatusTableError::new(&ctx, e.to_string()))?;

        let claim = ClaimKind::parse(&claim_s).ok_or_else(|| {
            StatusTableError::new(&ctx, format!("gate `{id}` has invalid claim `{claim_s}`"))
        })?;
        let tier = extract_str_list(&raw_tier, "tier", &ctx, true)?;
        let controls = extract_str_list(&raw_controls, "controls", &ctx, true)?;
        let raw_parts = match raw_parts_arr {
            Some(items) => {
                let mut out = Vec::with_capacity(items.len());
                for (pidx, sp) in items.into_iter().enumerate() {
                    match sp.value {
                        Value::Table(t) => out.push(t),
                        other => {
                            return Err(StatusTableError::new(
                                &ctx,
                                format!(
                                    "part[{pidx}] at {}:{}: expected table, found {}",
                                    sp.line,
                                    sp.column,
                                    other.type_name()
                                ),
                            ));
                        }
                    }
                }
                out
            }
            None => Vec::new(),
        };

        let mut parts = Vec::with_capacity(raw_parts.len());
        for (pidx, ptbl) in raw_parts.into_iter().enumerate() {
            let pctx = format!("gates.toml:gate[{id}].part[{pidx}]");
            let mut pf = Fields::new(ptbl, &pctx);
            let pmilestone = pf
                .require_str("milestone")
                .map_err(|e| StatusTableError::new(&pctx, e.to_string()))?;
            let status_s = pf
                .require_str("status")
                .map_err(|e| StatusTableError::new(&pctx, e.to_string()))?;
            let measured = pf
                .require_str("measured")
                .map_err(|e| StatusTableError::new(&pctx, e.to_string()))?;
            let raw_evidence = pf
                .require_array("evidence")
                .map_err(|e| StatusTableError::new(&pctx, e.to_string()))?;
            let rec_tbl = pf
                .require_table("recorded")
                .map_err(|e| StatusTableError::new(&pctx, e.to_string()))?;
            let kill_gate = pf
                .opt_str("kill_gate")
                .map_err(|e| StatusTableError::new(&pctx, e.to_string()))?;
            let fallback = pf
                .opt_str("fallback")
                .map_err(|e| StatusTableError::new(&pctx, e.to_string()))?;
            let open_question = pf
                .opt_str("open_question")
                .map_err(|e| StatusTableError::new(&pctx, e.to_string()))?;
            let detail = pf
                .opt_str("detail")
                .map_err(|e| StatusTableError::new(&pctx, e.to_string()))?;
            for field in [
                "scope",
                "commit",
                "command",
                "receipt",
                "fingerprint",
                "seed",
            ] {
                let _ = pf
                    .opt_str(field)
                    .map_err(|e| StatusTableError::new(&pctx, e.to_string()))?;
            }
            pf.finish()
                .map_err(|e| StatusTableError::new(&pctx, e.to_string()))?;

            let status = PartStatus::parse(&status_s).ok_or_else(|| {
                StatusTableError::new(
                    &pctx,
                    format!("gate `{id}` has invalid part status `{status_s}`"),
                )
            })?;
            let evidence = extract_str_list(&raw_evidence, "evidence", &pctx, true)?;

            let rctx = format!("{pctx}.recorded");
            let mut recf = Fields::new(rec_tbl, &rctx);
            let date = recf
                .require_str("date")
                .map_err(|e| StatusTableError::new(&rctx, e.to_string()))?;
            let session = recf
                .require_str("session")
                .map_err(|e| StatusTableError::new(&rctx, e.to_string()))?;
            let implemented_by = recf
                .require_str("implemented_by")
                .map_err(|e| StatusTableError::new(&rctx, e.to_string()))?;
            recf.finish()
                .map_err(|e| StatusTableError::new(&rctx, e.to_string()))?;

            parts.push(GatePart {
                milestone: pmilestone,
                status,
                measured,
                evidence,
                recorded: RecordedMeta {
                    date,
                    session,
                    implemented_by,
                },
                kill_gate,
                fallback,
                open_question,
                detail,
            });
        }

        let row = GateRow {
            id,
            claim,
            claim_id,
            section,
            threshold,
            milestone,
            tier,
            controls,
            parts,
        };
        validate_gate_row(&row)?;
        gates.push(row);
    }

    Ok(GateRegistry { schema, gates })
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum PrintedBucket {
    Open,
    Pass,
    Fail,
    Killed,
    Blocked,
    NoClaim,
    Superseded,
}

fn format_recorded(rec: &RecordedMeta) -> String {
    let d = rec.date.trim();
    let s = rec.session.trim();
    match (d.is_empty(), s.is_empty()) {
        (true, true) => "—".to_owned(),
        (false, false) => format!("{d} ({s})"),
        (false, true) => d.to_owned(),
        (true, false) => s.to_owned(),
    }
}

fn escape_md_cell(s: &str) -> String {
    s.replace('|', "\\|").replace('\n', " ").replace('\r', "")
}

fn pad_cell(s: &str, width: usize) -> String {
    let chars = s.chars().count();
    if chars >= width {
        s.to_owned()
    } else {
        let mut out = String::with_capacity(s.len() + (width - chars));
        out.push_str(s);
        for _ in 0..(width - chars) {
            out.push(' ');
        }
        out
    }
}

/// Fallible honest-status table generator (validates every gate before rendering).
///
/// Output is byte-deterministic (determinism class D0): rows sorted by `id`,
/// fixed deterministic column widths, `\n` line endings, and a trailing summary
/// line counting rows by printed status.
pub fn try_status_table(gates: &GateRegistry) -> Result<String, StatusTableError> {
    let mut seen_ids = BTreeSet::new();
    for g in &gates.gates {
        if !seen_ids.insert(g.id.as_str()) {
            return Err(StatusTableError::new(
                "status_table",
                format!("duplicate gate id `{}`", g.id),
            ));
        }
        validate_gate_row(g)?;
    }

    let mut ordered: Vec<&GateRow> = gates.gates.iter().collect();
    ordered.sort_by(|a, b| a.id.as_bytes().cmp(b.id.as_bytes()));

    let mut rows: Vec<[String; 6]> = Vec::with_capacity(ordered.len());
    let mut c_open = 0usize;
    let mut c_pass = 0usize;
    let mut c_fail = 0usize;
    let mut c_killed = 0usize;
    let mut c_blocked = 0usize;
    let mut c_no_claim = 0usize;
    let mut c_superseded = 0usize;

    for gate in ordered {
        let latest_part = gate
            .parts
            .iter()
            .enumerate()
            .max_by_key(|(idx, p)| (milestone_rank(&p.milestone).unwrap_or(0), *idx));

        let (milestone_cell, status_cell, measured_cell, recorded_cell, bucket) = match latest_part
        {
            None => (
                gate.milestone.clone(),
                "open".to_owned(),
                "—".to_owned(),
                "—".to_owned(),
                PrintedBucket::Open,
            ),
            Some((_, part)) => {
                let measured_trim = part.measured.trim();
                match part.status {
                    PartStatus::NotRun => (
                        part.milestone.clone(),
                        "open".to_owned(),
                        "—".to_owned(),
                        "—".to_owned(),
                        PrintedBucket::Open,
                    ),
                    PartStatus::Pass => {
                        if measured_trim.is_empty() {
                            return Err(StatusTableError::new(
                                "status_table",
                                format!(
                                    "gate `{}` has status `pass` with empty `measured`",
                                    gate.id
                                ),
                            ));
                        }
                        (
                            part.milestone.clone(),
                            "pass".to_owned(),
                            measured_trim.to_owned(),
                            format_recorded(&part.recorded),
                            PrintedBucket::Pass,
                        )
                    }
                    PartStatus::Fail => {
                        if measured_trim.is_empty() {
                            (
                                part.milestone.clone(),
                                "open".to_owned(),
                                "—".to_owned(),
                                "—".to_owned(),
                                PrintedBucket::Open,
                            )
                        } else {
                            (
                                part.milestone.clone(),
                                "fail".to_owned(),
                                measured_trim.to_owned(),
                                format_recorded(&part.recorded),
                                PrintedBucket::Fail,
                            )
                        }
                    }
                    PartStatus::Killed => {
                        let label = match (
                            part.kill_gate
                                .as_deref()
                                .map(str::trim)
                                .filter(|s| !s.is_empty()),
                            part.fallback
                                .as_deref()
                                .map(str::trim)
                                .filter(|s| !s.is_empty()),
                        ) {
                            (Some(kg), Some(fb)) => format!("killed ({kg}: {fb})"),
                            _ => {
                                let d = part.detail.as_deref().unwrap_or("").trim();
                                format!("killed ({d})")
                            }
                        };
                        let meas = if measured_trim.is_empty() {
                            "—".to_owned()
                        } else {
                            measured_trim.to_owned()
                        };
                        (
                            part.milestone.clone(),
                            label,
                            meas,
                            format_recorded(&part.recorded),
                            PrintedBucket::Killed,
                        )
                    }
                    PartStatus::Blocked => {
                        let blocker = part
                            .open_question
                            .as_deref()
                            .map(str::trim)
                            .filter(|s| !s.is_empty())
                            .or_else(|| {
                                part.detail
                                    .as_deref()
                                    .map(str::trim)
                                    .filter(|s| !s.is_empty())
                            })
                            .unwrap_or("");
                        let meas = if measured_trim.is_empty() {
                            "—".to_owned()
                        } else {
                            measured_trim.to_owned()
                        };
                        (
                            part.milestone.clone(),
                            format!("blocked ({blocker})"),
                            meas,
                            format_recorded(&part.recorded),
                            PrintedBucket::Blocked,
                        )
                    }
                    PartStatus::NoClaim => {
                        let meas = if measured_trim.is_empty() {
                            "—".to_owned()
                        } else {
                            measured_trim.to_owned()
                        };
                        (
                            part.milestone.clone(),
                            "no-claim".to_owned(),
                            meas,
                            format_recorded(&part.recorded),
                            PrintedBucket::NoClaim,
                        )
                    }
                    PartStatus::Superseded => {
                        let meas = if measured_trim.is_empty() {
                            "—".to_owned()
                        } else {
                            measured_trim.to_owned()
                        };
                        (
                            part.milestone.clone(),
                            "superseded".to_owned(),
                            meas,
                            format_recorded(&part.recorded),
                            PrintedBucket::Superseded,
                        )
                    }
                }
            }
        };

        match bucket {
            PrintedBucket::Open => c_open += 1,
            PrintedBucket::Pass => c_pass += 1,
            PrintedBucket::Fail => c_fail += 1,
            PrintedBucket::Killed => c_killed += 1,
            PrintedBucket::Blocked => c_blocked += 1,
            PrintedBucket::NoClaim => c_no_claim += 1,
            PrintedBucket::Superseded => c_superseded += 1,
        }

        rows.push([
            escape_md_cell(&gate.id),
            escape_md_cell(gate.claim.as_str()),
            escape_md_cell(&milestone_cell),
            escape_md_cell(&status_cell),
            escape_md_cell(&measured_cell),
            escape_md_cell(&recorded_cell),
        ]);
    }

    let headers = [
        "Gate",
        "Claim",
        "Milestone",
        "Status",
        "Measured",
        "Recorded",
    ];
    let mut widths = [16usize, 17, 9, 10, 12, 12];
    for r in &rows {
        for i in 0..6 {
            let len = r[i].chars().count();
            if len > widths[i] {
                widths[i] = len;
            }
        }
    }

    let mut out = String::new();
    out.push('|');
    for i in 0..6 {
        out.push(' ');
        out.push_str(&pad_cell(headers[i], widths[i]));
        out.push_str(" |");
    }
    out.push('\n');

    out.push('|');
    for w in widths {
        out.push(' ');
        for _ in 0..w {
            out.push('-');
        }
        out.push_str(" |");
    }
    out.push('\n');

    for r in &rows {
        out.push('|');
        for i in 0..6 {
            out.push(' ');
            out.push_str(&pad_cell(&r[i], widths[i]));
            out.push_str(" |");
        }
        out.push('\n');
    }

    out.push('\n');
    out.push_str(&format!(
        "Summary: {} gates (open: {c_open}, pass: {c_pass}, fail: {c_fail}, killed: {c_killed}, blocked: {c_blocked}, no-claim: {c_no_claim}, superseded: {c_superseded})\n",
        rows.len()
    ));

    Ok(out)
}

/// Render the honest-status table from a [`GateRegistry`], returning a schema
/// error naming the offending gate row if any invariant is violated.
pub fn status_table(gates: &GateRegistry) -> Result<String, StatusTableError> {
    try_status_table(gates)
}

/// Parse `gates.toml` bytes and render the honest-status table.
pub fn readme_table(bytes: &[u8]) -> Result<String, StatusTableError> {
    let reg = parse_gates_toml(bytes)?;
    try_status_table(&reg)
}
