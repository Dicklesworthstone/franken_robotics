#![deny(unsafe_code)]

//! Negative-control runner and the five rules of the negative-control registry (`N38b-04`).
//!
//! Makes `crates/frob-conformance/registry/controls.toml` operative across all five
//! control kinds (`mutation`, `planted-input`, `planted-design`, `planted-world`, `a-a`):
//! - Rule 1 (`INV-GAUNT-02`): a gate with no due runnable control cannot record `pass`.
//! - Rule 2: every due control runs with its gate; any `missed` or `caught-unexpected`
//!   control makes the gate line `fail`, and any `stale` control makes it `not-eligible`,
//!   decided strictly through [`frob_core::testkit::GateReport::finish`].
//! - Rule 3: a control is a planted defect, never a product boundary check.
//! - Rule 4: never-caught and stale controls at a milestone exit enter the suspect report
//!   (`tests/control_review.rs`, `registry/control_reviews.toml`) and block the exit until
//!   disposed (`replanted`, `gate-strengthened`, or `withdrawn` with reason).
//! - Rule 5: permanent controls (`permanent = true`) cannot be deleted or detached from
//!   any gate (`controls::ControlReason::PermanentControlRemoved`).
//!
//! Annex-workspace controls are dispatched through the extensible [`DRIVERS`] registry
//! (`(workspace, fn(&Control) -> ControlLine)`), returning [`ControlStatus::Stale`] when
//! no driver is registered for that workspace.

pub mod examples;
pub mod guard;
pub mod history;
pub mod mutation;

use std::env;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use frob_core::testkit::evidence::{
    EvidenceKind, EvidenceRecord, EvidenceStatus, emit_if_configured, emit_to_dir,
};
use frob_core::testkit::registry::{Control, ControlKind, ControlOutcome, ControlState, Controls};
use frob_core::testkit::{GateLine, GateReport, Threshold};

use crate::controls::{ControlReason, check_registry, validate};
use crate::gates_v0::{GATES_PATH, GateRegistry, parse_milestone_num};
use crate::reasons::HarnessReason;
use crate::tiers::receipt::ReceiptControl;
use crate::tiers::{ControlRunOutcome, Milestone};

pub use examples::{
    CartDesignSpec, DesignVerdict, FixedNPreanalysis, admit_fixed_n_study, classify_cart_design,
    clopper_pearson_interval_975, docking_three_gains_min_worst_error, docking_two_gains_admitted,
    enumerate_fixed_n_rule, evaluate_aa_campaign, evaluate_planted_design_fixture,
    evaluate_wheel_loop_host_row, paired_lower_bound_95, verify_transported_docking_certificate,
    wheel_loop_spectral_radius_active, wheel_loop_spectral_radius_euler_mutant,
    wheel_loop_spectral_radius_exact, wheel_loop_spectral_radius_r_iref,
};
pub use guard::{
    GuardViolation, MutationGuardReport, PRODUCT_BINARIES, check_mutation_guard,
    verify_release_build_fails,
};
pub use history::{
    CONTROL_HISTORY_PATH, CONTROL_REVIEWS_PATH, CONTROL_REVIEWS_SCHEMA, ControlHistoryEntry,
    ControlReviewRecord, ControlReviews, Disposition, SuspectCause, SuspectEntry, SuspectReport,
    append_history_entry, build_suspect_report, load_history, load_reviews, parse_history_ndjson,
    resolve_review_milestone,
};
pub use mutation::{
    MutationRoute, build_cargo_command, check_manifest_declares_feature, inspect_cargo_args,
    parse_law_suite_target, resolve_mutation_route, run_mutation_control,
};

/// Outcome of evaluating one registered negative control (`N38b-04`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ControlStatus {
    /// Defect was planted and the gate failed with the observable stated in `expect`.
    Caught,
    /// Defect was planted and every entry passed.
    Missed,
    /// Gate failed, but for a reason different from `expect`; counts as `missed`.
    CaughtUnexpected,
    /// Defect could not be planted (missing feature, missing/unheadered fixture, or
    /// unregistered annex driver); never counts as caught and makes the gate `not-eligible`.
    Stale,
    /// Control's `due` milestone is strictly later than the milestone of the part being run;
    /// neither blocks nor supports that part.
    NotYetDue,
    /// Control is staged in `state = "obligation"`; never counts as caught on its own.
    Obligation,
}

impl ControlStatus {
    /// Canonical wire spelling (`N38b-04` / `N38-REF`).
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Caught => "caught",
            Self::Missed => HarnessReason::Missed.as_str(),
            Self::CaughtUnexpected => HarnessReason::CaughtUnexpected.as_str(),
            Self::Stale => HarnessReason::Stale.as_str(),
            Self::NotYetDue => HarnessReason::NotYetDue.as_str(),
            Self::Obligation => "obligation",
        }
    }

    /// Whether this outcome counts as `caught` toward `GATE-GAUNT-01`.
    #[must_use]
    pub const fn counts_as_caught(self) -> bool {
        matches!(self, Self::Caught)
    }

    /// Whether this outcome counts as `missed` (forcing gate line `fail`).
    #[must_use]
    pub const fn counts_as_missed(self) -> bool {
        matches!(self, Self::Missed | Self::CaughtUnexpected)
    }

    /// Convert to `frob_core::testkit::ControlOutcome` for `GateReport`.
    ///
    /// `NotYetDue` returns `None` because a future-due control neither blocks nor
    /// supports an earlier milestone part. `CaughtUnexpected` maps to `Missed`.
    #[must_use]
    pub const fn to_gate_report_outcome(self) -> Option<ControlOutcome> {
        match self {
            Self::Caught => Some(ControlOutcome::Caught),
            Self::Missed | Self::CaughtUnexpected => Some(ControlOutcome::Missed),
            Self::Stale => Some(ControlOutcome::Stale),
            Self::Obligation => Some(ControlOutcome::Obligation),
            Self::NotYetDue => None,
        }
    }

    /// Convert to the tier receipt's `ControlRunOutcome`.
    #[must_use]
    pub const fn to_tier_outcome(self) -> ControlRunOutcome {
        match self {
            Self::Caught => ControlRunOutcome::Caught,
            Self::Missed | Self::CaughtUnexpected => ControlRunOutcome::Missed,
            Self::Stale => ControlRunOutcome::Stale,
            Self::NotYetDue => ControlRunOutcome::NotYetDue,
            Self::Obligation => ControlRunOutcome::Obligation,
        }
    }
}

impl fmt::Display for ControlStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// One emitted negative-control result line (`N38b-04` step 4 & annex driver return type).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlLine {
    pub id: String,
    pub gate: String,
    pub status: ControlStatus,
    pub observed: String,
    pub expected: String,
    pub due: Option<String>,
    pub detail: String,
}

impl ControlLine {
    /// Construct a `ControlLine` from a control record and status.
    #[must_use]
    pub fn new(
        control: &Control,
        gate: impl Into<String>,
        status: ControlStatus,
        observed: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            id: control.id.clone(),
            gate: gate.into(),
            status,
            observed: observed.into(),
            expected: control.expect.clone(),
            due: control.due.clone(),
            detail: detail.into(),
        }
    }

    /// Convert to a `frob.testevidence/1` unit-test evidence stream record (`kind = nc`).
    #[must_use]
    pub fn to_evidence_record(&self) -> EvidenceRecord {
        let (ev_status, reason) = match self.status {
            ControlStatus::Caught => (EvidenceStatus::Caught, None),
            ControlStatus::Missed => (
                EvidenceStatus::Missed,
                Some(HarnessReason::Missed.as_str().to_owned()),
            ),
            ControlStatus::CaughtUnexpected => (
                EvidenceStatus::Missed,
                Some(HarnessReason::CaughtUnexpected.as_str().to_owned()),
            ),
            ControlStatus::Stale => (
                EvidenceStatus::Stale,
                Some(HarnessReason::Stale.as_str().to_owned()),
            ),
            ControlStatus::NotYetDue => (
                EvidenceStatus::Skip,
                Some(HarnessReason::NotYetDue.as_str().to_owned()),
            ),
            ControlStatus::Obligation => (EvidenceStatus::Obligation, None),
        };
        let mut rec = EvidenceRecord::new(EvidenceKind::Nc, &self.id, ev_status);
        rec.reason = reason;
        rec.observed = Some(self.observed.clone());
        rec.expected = Some(self.expected.clone());
        rec
    }

    /// Convert to the tier receipt's `ReceiptControl` row.
    #[must_use]
    pub fn to_receipt_control(&self) -> ReceiptControl {
        ReceiptControl {
            id: self.id.clone(),
            gate: self.gate.clone(),
            outcome: self.status.to_tier_outcome(),
        }
    }

    /// Format a single human-readable status line (including `due=<Mn>` for `not-yet-due`).
    #[must_use]
    pub fn format_line(&self) -> String {
        if self.status == ControlStatus::NotYetDue {
            let due = self.due.as_deref().unwrap_or("unknown");
            format!(
                "nc {} gate={} status={} due={due} detail={}",
                self.id,
                self.gate,
                self.status.as_str(),
                self.detail
            )
        } else {
            format!(
                "nc {} gate={} status={} observed={} expected={}",
                self.id,
                self.gate,
                self.status.as_str(),
                self.observed,
                self.expected
            )
        }
    }
}

/// Signature of an annex-workspace control driver (`N38b-04`, `INV-SPARK-01`).
pub type AnnexDriver = fn(&Control) -> ControlLine;

/// Process-wide registry of annex-workspace control drivers (`control_runner::DRIVERS`).
///
/// Later annex beads (such as Spark's `N21c-30`) register `(workspace, driver)` here so
/// the core conformance crate never depends on an annex workspace directly.
pub struct DriverRegistry {
    entries: Mutex<Vec<(String, AnnexDriver)>>,
}

impl DriverRegistry {
    /// Create an empty driver registry.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            entries: Mutex::new(Vec::new()),
        }
    }

    /// Register or replace the driver for `workspace`.
    pub fn register(&self, workspace: impl Into<String>, driver: AnnexDriver) {
        let ws = workspace.into();
        let mut guard = self
            .entries
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some((_, slot)) = guard.iter_mut().find(|(name, _)| *name == ws) {
            *slot = driver;
        } else {
            guard.push((ws, driver));
        }
    }

    /// Remove the registered driver for `workspace` (if present).
    pub fn unregister(&self, workspace: &str) {
        let mut guard = self
            .entries
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        guard.retain(|(name, _)| name != workspace);
    }

    /// Look up the driver registered for `workspace`.
    #[must_use]
    pub fn get(&self, workspace: &str) -> Option<AnnexDriver> {
        let guard = self
            .entries
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        guard
            .iter()
            .find(|(name, _)| name == workspace)
            .map(|(_, f)| *f)
    }

    /// Snapshot all registered workspace names in deterministic sorted order.
    #[must_use]
    pub fn workspaces(&self) -> Vec<String> {
        let guard = self
            .entries
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let mut names: Vec<String> = guard.iter().map(|(n, _)| n.clone()).collect();
        names.sort();
        names
    }
}

impl Default for DriverRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Global annex driver hook (`N38b-04`: `control_runner::DRIVERS`).
pub static DRIVERS: DriverRegistry = DriverRegistry::new();

/// Detect whether `target` names an annex workspace (e.g. `annex:spark::...`,
/// `spark::...`, `frob-spark::...`, or any workspace name currently registered in
/// [`DRIVERS`]).
#[must_use]
pub fn detect_annex_workspace(target: &str) -> Option<String> {
    let trimmed = target.trim();
    if let Some(rest) = trimmed.strip_prefix("annex:") {
        let ws = rest
            .split("::")
            .next()
            .unwrap_or(rest)
            .split(':')
            .next()
            .unwrap_or(rest)
            .trim();
        if !ws.is_empty() {
            return Some(ws.to_owned());
        }
    }
    let head = trimmed
        .split("::")
        .next()
        .unwrap_or(trimmed)
        .split('/')
        .next()
        .unwrap_or(trimmed)
        .trim();
    if head == "spark" || head == "frob-spark" || head.starts_with("frob-spark-") {
        return Some("spark".to_owned());
    }
    if DRIVERS.get(head).is_some() {
        return Some(head.to_owned());
    }
    None
}

/// Raw outcome of executing one plantable control before `expect` comparison.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ControlEval {
    /// Defect was planted, and every gate entry passed (becomes [`ControlStatus::Missed`]).
    DidNotFail { detail: String },
    /// Defect was planted, and the gate failed with `observed` (checked against `expect`).
    Failed { observed: String },
    /// Defect could not be planted (becomes [`ControlStatus::Stale`]).
    Stale { detail: String },
    /// Pre-classified line returned verbatim by an annex workspace driver.
    DriverLine(ControlLine),
}

/// Configuration for running the negative controls of a gate.
#[derive(Debug, Clone)]
pub struct ControlRunnerConfig {
    pub workspace_root: PathBuf,
    pub milestone: Milestone,
    pub tier_target: String,
    pub cargo_target_dir: Option<PathBuf>,
    pub evidence_dir: Option<PathBuf>,
    pub offline: bool,
    /// When `true` and `crates/frob-actuate` does not yet exist (M0), allow the
    /// `NC-0412` wheel-loop host stand-in row in `frob-conformance`.
    pub allow_m0_host_standin: bool,
}

impl ControlRunnerConfig {
    /// Build configuration rooted at `workspace_root` for `milestone`.
    #[must_use]
    pub fn new(workspace_root: PathBuf, milestone: Milestone) -> Self {
        let evidence_dir = env::var_os("FROB_TEST_EVIDENCE_DIR")
            .filter(|v| !v.is_empty())
            .map(PathBuf::from);
        Self {
            workspace_root,
            milestone,
            tier_target: "g1_known_answers".to_owned(),
            cargo_target_dir: None,
            evidence_dir,
            offline: true,
            allow_m0_host_standin: true,
        }
    }

    /// Build configuration from environment (`FROB_MILESTONE` defaulting to `M0`).
    #[must_use]
    pub fn from_env(workspace_root: PathBuf) -> Self {
        let milestone = env::var("FROB_MILESTONE")
            .ok()
            .and_then(|s| Milestone::parse(&s))
            .unwrap_or(Milestone::M0);
        Self::new(workspace_root, milestone)
    }
}

/// Result of running all registered negative controls for one gate (`N38b-04`).
#[derive(Debug, Clone)]
pub struct GateControlVerdict {
    pub gate_id: String,
    pub milestone: Milestone,
    pub lines: Vec<ControlLine>,
    /// Sealed gate verdict computed strictly through `GateReport::finish()`.
    pub gate_line: GateLine,
}

impl GateControlVerdict {
    /// Count of due controls that were caught.
    #[must_use]
    pub fn caught_count(&self) -> usize {
        self.lines
            .iter()
            .filter(|l| l.status == ControlStatus::Caught)
            .count()
    }

    /// Count of due controls that were missed (including `caught-unexpected`).
    #[must_use]
    pub fn missed_count(&self) -> usize {
        self.lines
            .iter()
            .filter(|l| l.status.counts_as_missed())
            .count()
    }

    /// Count of due controls that were stale.
    #[must_use]
    pub fn stale_count(&self) -> usize {
        self.lines
            .iter()
            .filter(|l| l.status == ControlStatus::Stale)
            .count()
    }

    /// Count of controls skipped as `not-yet-due`.
    #[must_use]
    pub fn not_yet_due_count(&self) -> usize {
        self.lines
            .iter()
            .filter(|l| l.status == ControlStatus::NotYetDue)
            .count()
    }

    /// Receipt-compatible control rows for embedding into `frob.gauntlet.receipt/1`.
    #[must_use]
    pub fn receipt_controls(&self) -> Vec<ReceiptControl> {
        self.lines
            .iter()
            .map(ControlLine::to_receipt_control)
            .collect()
    }
}

/// Returns `true` when `ctrl.due` is strictly after `run_milestone`.
#[must_use]
pub fn is_not_yet_due(ctrl: &Control, run_milestone: Milestone) -> bool {
    let Some(due_raw) = ctrl.due.as_deref() else {
        return false;
    };
    parse_milestone_num(due_raw).is_some_and(|dm| dm > u32::from(run_milestone.number()))
}

/// Verify that a non-mutation control's fixture file exists under `workspace_root`
/// and starts with a `synthetic` kind header (first 8 lines).
pub fn verify_synthetic_fixture(
    workspace_root: &Path,
    fixture_rel: &str,
) -> Result<PathBuf, String> {
    let trimmed = fixture_rel.trim();
    if trimmed.is_empty() {
        return Err("control omits required `fixture` path".to_owned());
    }
    let rel = Path::new(trimmed);
    if rel.is_absolute()
        || rel
            .components()
            .any(|c| matches!(c, std::path::Component::ParentDir))
    {
        return Err(format!(
            "fixture path `{trimmed}` must be workspace-relative without `..`"
        ));
    }
    let path = workspace_root.join(rel);
    if !path.is_file() {
        return Err(format!("fixture `{trimmed}` not found"));
    }
    let text = fs::read_to_string(&path)
        .map_err(|e| format!("failed to read fixture `{trimmed}`: {e}"))?;
    if !has_synthetic_header(&text) {
        return Err(format!(
            "fixture `{trimmed}` lacks required `synthetic` kind header"
        ));
    }
    Ok(path)
}

/// Returns `true` if the first 8 lines of `content` carry a `synthetic` kind header.
#[must_use]
pub fn has_synthetic_header(content: &str) -> bool {
    let head = content
        .lines()
        .take(8)
        .collect::<Vec<_>>()
        .join("\n")
        .to_ascii_lowercase();
    head.contains("# synthetic")
        || head.contains("#synthetic")
        || head.contains("// synthetic")
        || head.contains("kind: synthetic")
        || head.contains("kind:synthetic")
        || head.contains("kind = \"synthetic\"")
        || head.contains("fixture: synthetic")
        || head.contains("fixture_kind = \"synthetic\"")
        || head.contains("\"kind\":\"synthetic\"")
        || head.contains("\"kind\": \"synthetic\"")
}

/// Compare an observed failure output against a control's `expect` specification.
///
/// Returns `true` only when `observed` exhibits the specific refusal code, registered
/// reason/verdict slug, or primary numeric/rational observable named in `expect`.
/// A crash or panic for an unrelated reason returns `false`, turning the outcome into
/// [`ControlStatus::CaughtUnexpected`] (counted as `missed`).
#[must_use]
pub fn expect_matches(expect: &str, observed: &str) -> bool {
    let exp = expect.trim();
    let obs = observed.trim();
    if exp.is_empty() || obs.is_empty() {
        return false;
    }
    let exp_lower = exp.to_ascii_lowercase();
    let obs_lower = obs.to_ascii_lowercase();

    if obs_lower.contains(&exp_lower) {
        return true;
    }

    // 1. Every `FROB-REF-nnn` code named in `expect` must appear in `observed`.
    let ref_codes = extract_ref_codes(exp);
    if !ref_codes.is_empty() {
        return ref_codes.iter().all(|code| obs.contains(code.as_str()));
    }

    // 2. Check distinctive reason/verdict slugs and numeric/rational observables.
    const REASON_SLUGS: &[&str] = &[
        "no-control",
        "dangling-control",
        "one-way-link",
        "uncaught-control",
        "fixture-missing",
        "feature-missing",
        "boundary-not-control",
        "permanent-control-removed",
        "suspect-undisposed",
        "not-eligible",
        "cannot-fail",
        "reject",
        "does not clear",
        "not cleared",
        "unsatisfiable",
        "defective",
    ];
    let exp_slugs: Vec<&str> = REASON_SLUGS
        .iter()
        .copied()
        .filter(|slug| exp_lower.contains(slug))
        .collect();
    let exp_nums = extract_numeric_observables(exp);

    if !exp_slugs.is_empty() || !exp_nums.is_empty() {
        let slugs_ok = exp_slugs.is_empty() || exp_slugs.iter().all(|s| obs_lower.contains(s));
        // For numeric observables, the first distinctive decimal/fraction in `expect`
        // is the mutant's reported value (e.g. `1.497` in "reports 1.497 where 0.9207 is true",
        // or `1/65` in "best worst-case error 1/65 m"). Require it to appear in `observed`.
        let num_ok = exp_nums.first().is_none_or(|n| obs_lower.contains(n));
        return slugs_ok && num_ok;
    }

    // 3. Fallback: require meaningful non-stopword tokens from `expect` to match.
    const STOPWORDS: &[&str] = &[
        "the", "and", "for", "with", "that", "this", "from", "where", "when", "must", "fail",
        "fails", "failed", "row", "gate", "test", "true", "false",
    ];
    let tokens: Vec<&str> = exp_lower
        .split(|c: char| {
            !(c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.' || c == '/')
        })
        .filter(|t| t.len() >= 3 && !STOPWORDS.contains(t))
        .collect();
    if tokens.is_empty() {
        return false;
    }
    let matched = tokens.iter().filter(|t| obs_lower.contains(**t)).count();
    let needed = tokens.len().min(2);
    matched >= needed
}

fn extract_ref_codes(s: &str) -> Vec<String> {
    let mut codes = Vec::new();
    let bytes = s.as_bytes();
    let mut i = 0usize;
    while i + 9 <= bytes.len() {
        if &bytes[i..i + 9] == b"FROB-REF-" {
            let mut j = i + 9;
            while j < bytes.len() && bytes[j].is_ascii_digit() {
                j += 1;
            }
            if j > i + 9 {
                let code = s[i..j].to_owned();
                if !codes.contains(&code) {
                    codes.push(code);
                }
                i = j;
                continue;
            }
        }
        i += 1;
    }
    codes
}

fn extract_numeric_observables(s: &str) -> Vec<String> {
    let mut nums = Vec::new();
    for raw in s.split(|c: char| !(c.is_ascii_alphanumeric() || c == '.' || c == '/')) {
        let tok = raw.trim_matches('.');
        if tok.is_empty() {
            continue;
        }
        let has_digit = tok.bytes().any(|b| b.is_ascii_digit());
        let is_decimal_or_frac = (tok.contains('.') || tok.contains('/'))
            && tok
                .bytes()
                .all(|b| b.is_ascii_digit() || b == b'.' || b == b'/');
        if has_digit && is_decimal_or_frac {
            let low = tok.to_ascii_lowercase();
            if !nums.contains(&low) {
                nums.push(low);
            }
        }
    }
    nums
}

/// Run all negative controls registered for `gate_id` using a custom evaluator
/// callback, enforcing the exact five-step algorithm of `N38b-04`.
///
/// The gate verdict in [`GateControlVerdict::gate_line`] is computed exclusively
/// through [`frob_core::testkit::GateReport::finish`].
pub fn run_controls_for_gate<F>(
    gate_id: &str,
    entries_passed: bool,
    controls: &Controls,
    config: &ControlRunnerConfig,
    mut evaluator: F,
) -> GateControlVerdict
where
    F: FnMut(&Control, &ControlRunnerConfig) -> ControlEval,
{
    let mut gate_controls: Vec<&Control> = controls.for_gate(gate_id);
    gate_controls.sort_by(|a, b| a.id.cmp(&b.id));

    let mut lines: Vec<ControlLine> = Vec::with_capacity(gate_controls.len());
    let mut report = GateReport::new(gate_id);
    if let Some(ref ev_dir) = config.evidence_dir {
        report = report.evidence_dir(ev_dir);
    }
    if entries_passed {
        report = report.measured("entries_passed", "1", "bool", Threshold::eq("1"));
    } else {
        report = report.measured("entries_passed", "0", "bool", Threshold::eq("1"));
    }

    for ctrl in gate_controls {
        // Step 1: Milestone `due` filter (`not-yet-due` neither blocks nor supports).
        if is_not_yet_due(ctrl, config.milestone) {
            let due_str = ctrl.due.clone().unwrap_or_default();
            let line = ControlLine::new(
                ctrl,
                gate_id,
                ControlStatus::NotYetDue,
                format!("not-yet-due (due {due_str} > {})", config.milestone),
                format!("control due at {due_str}, skipped at {}", config.milestone),
            );
            emit_control_evidence(&line, config.evidence_dir.as_deref());
            lines.push(line);
            continue;
        }

        // Obligation controls are reported but never count as caught.
        if ctrl.state == ControlState::Obligation {
            let line = ControlLine::new(
                ctrl,
                gate_id,
                ControlStatus::Obligation,
                "obligation",
                "control is staged as obligation",
            );
            emit_control_evidence(&line, config.evidence_dir.as_deref());
            if let Some(go) = line.status.to_gate_report_outcome() {
                report = report.control(&ctrl.id, go);
            }
            lines.push(line);
            continue;
        }

        // Step 2: Check plantability (annex driver hook, mutation feature, or synthetic fixture).
        let line = if let Some(ws) = detect_annex_workspace(&ctrl.target) {
            if let Some(driver) = DRIVERS.get(&ws) {
                let mut dl = driver(ctrl);
                if dl.gate.is_empty() {
                    dl.gate = gate_id.to_owned();
                }
                dl
            } else {
                ControlLine::new(
                    ctrl,
                    gate_id,
                    ControlStatus::Stale,
                    format!(
                        "no driver registered in control_runner::DRIVERS for annex workspace `{ws}`"
                    ),
                    format!("annex target `{}` has no registered driver", ctrl.target),
                )
            }
        } else {
            let plant_check = check_control_plantable(ctrl, config);
            match plant_check {
                Err(stale_why) => ControlLine::new(
                    ctrl,
                    gate_id,
                    ControlStatus::Stale,
                    stale_why.clone(),
                    stale_why,
                ),
                Ok(()) => {
                    // Step 3: Execute control and check observable against `expect`.
                    match evaluator(ctrl, config) {
                        ControlEval::DriverLine(mut dl) => {
                            if dl.gate.is_empty() {
                                dl.gate = gate_id.to_owned();
                            }
                            dl
                        }
                        ControlEval::Stale { detail } => ControlLine::new(
                            ctrl,
                            gate_id,
                            ControlStatus::Stale,
                            detail.clone(),
                            detail,
                        ),
                        ControlEval::DidNotFail { detail } => ControlLine::new(
                            ctrl,
                            gate_id,
                            ControlStatus::Missed,
                            detail.clone(),
                            format!("reason={}: {detail}", HarnessReason::Missed.as_str()),
                        ),
                        ControlEval::Failed { observed } => {
                            if expect_matches(&ctrl.expect, &observed) {
                                ControlLine::new(
                                    ctrl,
                                    gate_id,
                                    ControlStatus::Caught,
                                    observed,
                                    "caught with expected observable",
                                )
                            } else {
                                ControlLine::new(
                                    ctrl,
                                    gate_id,
                                    ControlStatus::CaughtUnexpected,
                                    observed.clone(),
                                    format!(
                                        "reason={}: failed with `{observed}`, expected `{}`",
                                        HarnessReason::CaughtUnexpected.as_str(),
                                        ctrl.expect
                                    ),
                                )
                            }
                        }
                    }
                }
            }
        };

        // Step 4: Emit unit-test evidence line.
        emit_control_evidence(&line, config.evidence_dir.as_deref());

        // Step 5: Report into GateReport (NotYetDue omitted; CaughtUnexpected -> Missed).
        if let Some(go) = line.status.to_gate_report_outcome() {
            report = report.control(&ctrl.id, go);
        }
        lines.push(line);
    }

    let gate_line = report.finish();
    GateControlVerdict {
        gate_id: gate_id.to_owned(),
        milestone: config.milestone,
        lines,
        gate_line,
    }
}

/// Run all negative controls for `gate_id` using the built-in five-kind dispatcher.
#[must_use]
pub fn run_controls_for_gate_default(
    gate_id: &str,
    entries_passed: bool,
    controls: &Controls,
    config: &ControlRunnerConfig,
) -> GateControlVerdict {
    run_controls_for_gate(gate_id, entries_passed, controls, config, |ctrl, cfg| {
        evaluate_control_default(ctrl, gate_id, cfg)
    })
}

/// Step-2 plantability check for non-annex controls.
fn check_control_plantable(ctrl: &Control, config: &ControlRunnerConfig) -> Result<(), String> {
    match ctrl.kind {
        ControlKind::Mutation => {
            let feat = ctrl
                .feature
                .as_deref()
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .ok_or_else(|| format!("mutation control `{}` has no `feature` field", ctrl.id))?;
            let route = mutation::resolve_mutation_route(
                &config.workspace_root,
                ctrl,
                ctrl.gate.first().map_or("", String::as_str),
                &config.tier_target,
                config.allow_m0_host_standin,
            )?;
            match &route {
                MutationRoute::WorkspaceTier {
                    crate_name,
                    feature,
                    ..
                }
                | MutationRoute::LawSuite {
                    crate_name,
                    feature,
                    ..
                }
                | MutationRoute::HostStandIn {
                    crate_name,
                    feature,
                } => mutation::check_manifest_declares_feature(
                    &config.workspace_root,
                    crate_name,
                    feature,
                ),
                MutationRoute::Annex { workspace, .. } => Err(format!(
                    "annex control `{}` feature `{feat}` requires driver for `{workspace}`",
                    ctrl.id
                )),
            }
        }
        ControlKind::PlantedInput
        | ControlKind::PlantedDesign
        | ControlKind::PlantedWorld
        | ControlKind::Aa => {
            let fx = ctrl
                .fixture
                .as_deref()
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .ok_or_else(|| {
                    format!(
                        "{:?} control `{}` has no `fixture` field",
                        ctrl.kind, ctrl.id
                    )
                })?;
            verify_synthetic_fixture(&config.workspace_root, fx).map(|_| ())
        }
    }
}

/// Built-in evaluator covering all five control kinds (`mutation`, `planted-input`,
/// `planted-design`, `planted-world`, `a-a`) plus the M0 Gauntlet controls.
pub fn evaluate_control_default(
    ctrl: &Control,
    gate_id: &str,
    config: &ControlRunnerConfig,
) -> ControlEval {
    match ctrl.kind {
        ControlKind::Mutation => {
            let route = match mutation::resolve_mutation_route(
                &config.workspace_root,
                ctrl,
                gate_id,
                &config.tier_target,
                config.allow_m0_host_standin,
            ) {
                Ok(r) => r,
                Err(why) => return ControlEval::Stale { detail: why },
            };
            match route {
                MutationRoute::HostStandIn { feature, .. } if feature == "cx-262" => {
                    let rho = examples::wheel_loop_spectral_radius_euler_mutant();
                    match examples::evaluate_wheel_loop_host_row(rho) {
                        Ok(msg) => ControlEval::DidNotFail { detail: msg },
                        Err(obs) => ControlEval::Failed { observed: obs },
                    }
                }
                _ => mutation::run_mutation_control(
                    &config.workspace_root,
                    ctrl,
                    gate_id,
                    &config.tier_target,
                    config.cargo_target_dir.as_deref(),
                    config.offline,
                    config.allow_m0_host_standin,
                ),
            }
        }
        ControlKind::PlantedInput => evaluate_planted_input(ctrl, config),
        ControlKind::PlantedDesign => evaluate_planted_design(ctrl, config),
        ControlKind::PlantedWorld => evaluate_planted_world(ctrl, config),
        ControlKind::Aa => evaluate_aa(ctrl, config),
    }
}

fn evaluate_planted_input(ctrl: &Control, config: &ControlRunnerConfig) -> ControlEval {
    let Some(fx_rel) = ctrl.fixture.as_deref() else {
        return ControlEval::Stale {
            detail: "missing fixture path".to_owned(),
        };
    };
    let path = match verify_synthetic_fixture(&config.workspace_root, fx_rel) {
        Ok(p) => p,
        Err(why) => return ControlEval::Stale { detail: why },
    };
    let text = match fs::read_to_string(&path) {
        Ok(t) => t,
        Err(e) => {
            return ControlEval::Stale {
                detail: format!("read fixture: {e}"),
            };
        }
    };

    // Route by target / fixture content.
    if ctrl.target.contains("check_registry") {
        let gates = match GateRegistry::parse_str(&text, GATES_PATH) {
            Ok(g) => g,
            Err(e) => {
                return ControlEval::Failed {
                    observed: format!("registry_check parse error: {e}"),
                };
            }
        };
        let empty_controls = Controls::default();
        // If the fixture has a companion control row embedded in comments or uses
        // the workspace controls, load workspace controls when available.
        let ws_controls = Controls::load(&config.workspace_root)
            .map(|(c, _)| c)
            .unwrap_or(empty_controls);
        let report = check_registry(&gates, &ws_controls);
        if report.is_ok() {
            return ControlEval::DidNotFail {
                detail: "registry_check unexpectedly accepted planted gate fixture".to_owned(),
            };
        }
        let reasons: Vec<String> = report
            .findings
            .iter()
            .map(|f| format!("{}: {}", f.reason.as_str(), f.detail))
            .collect();
        return ControlEval::Failed {
            observed: format!("registry_check fails with {}", reasons.join("; ")),
        };
    }

    if ctrl.target.contains("controls::validate") {
        // Fixture carries a planted control pointing to a fixture without a kind header.
        let Ok(gates) = GateRegistry::load(&config.workspace_root) else {
            return ControlEval::Stale {
                detail: "failed to load workspace gates.toml".to_owned(),
            };
        };
        // Construct an active pass gate and a control referencing an unheadered temp payload.
        let active_gates_src = r#"schema = "frob.gates/1"

[[gate]]
id = "GATE-GAUNT-05"
claim = "known-answer"
claim_id = "INV-GAUNT-01"
section = "38.15"
threshold = "oracle hygiene"
milestone = "M0"
tier = ["G0:host"]
controls = ["NC-0002"]

[[gate.part]]
milestone = "M0"
status = "pass"
measured = "ok"
evidence = []
recorded = { date = "2026-11-24", session = "probe", implemented_by = "" }
"#;
        let active_gates = GateRegistry::parse_str(active_gates_src, GATES_PATH).unwrap_or(gates);
        let planted_ctrls = match Controls::parse_str(&text, crate::controls::CONTROLS_PATH) {
            Ok(c) => c,
            Err(e) => {
                return ControlEval::Failed {
                    observed: format!("controls validator parse error: {e}"),
                };
            }
        };
        let findings = validate(&planted_ctrls, None, &active_gates, &config.workspace_root);
        if findings.is_empty() {
            return ControlEval::DidNotFail {
                detail: "controls::validate accepted planted missing-header fixture".to_owned(),
            };
        }
        let has_fm = findings
            .iter()
            .any(|f| f.reason == ControlReason::FixtureMissing);
        let summary: Vec<String> = findings.iter().map(ToString::to_string).collect();
        return ControlEval::Failed {
            observed: if has_fm {
                format!(
                    "controls validator fails with fixture-missing: {}",
                    summary.join("; ")
                )
            } else {
                summary.join("; ")
            },
        };
    }

    if ctrl.target.contains("eval-preanalyze") || text.contains("practical-tie") {
        let n = parse_u32_field(&text, "n").unwrap_or(64);
        let margin = parse_f64_field(&text, "margin").unwrap_or(0.05);
        match examples::admit_fixed_n_study(n, margin, &["practical-tie"], true) {
            Ok(plan) => ControlEval::DidNotFail {
                detail: format!("fixed-n study admitted unreachable tie at n={}", plan.n),
            },
            Err(refusal) => ControlEval::Failed { observed: refusal },
        }
    } else {
        ControlEval::Stale {
            detail: format!(
                "no planted-input handler bound for target `{}`",
                ctrl.target
            ),
        }
    }
}

fn evaluate_planted_design(ctrl: &Control, config: &ControlRunnerConfig) -> ControlEval {
    let Some(fx_rel) = ctrl.fixture.as_deref() else {
        return ControlEval::Stale {
            detail: "missing fixture path".to_owned(),
        };
    };
    let path = match verify_synthetic_fixture(&config.workspace_root, fx_rel) {
        Ok(p) => p,
        Err(why) => return ControlEval::Stale { detail: why },
    };
    let text = match fs::read_to_string(&path) {
        Ok(t) => t,
        Err(e) => {
            return ControlEval::Stale {
                detail: format!("read fixture: {e}"),
            };
        }
    };
    match examples::evaluate_planted_design_fixture(&text) {
        Ok(flagged) => ControlEval::Failed { observed: flagged },
        Err(msg) => ControlEval::DidNotFail { detail: msg },
    }
}

fn evaluate_planted_world(ctrl: &Control, config: &ControlRunnerConfig) -> ControlEval {
    let Some(fx_rel) = ctrl.fixture.as_deref() else {
        return ControlEval::Stale {
            detail: "missing fixture path".to_owned(),
        };
    };
    let path = match verify_synthetic_fixture(&config.workspace_root, fx_rel) {
        Ok(p) => p,
        Err(why) => return ControlEval::Stale { detail: why },
    };
    let text = match fs::read_to_string(&path) {
        Ok(t) => t,
        Err(e) => {
            return ControlEval::Stale {
                detail: format!("read fixture: {e}"),
            };
        }
    };
    match examples::evaluate_planted_world_docking_fixture(&text) {
        Ok(rejected_msg) => ControlEval::Failed {
            observed: rejected_msg,
        },
        Err(msg) => ControlEval::DidNotFail { detail: msg },
    }
}

fn evaluate_aa(ctrl: &Control, config: &ControlRunnerConfig) -> ControlEval {
    let Some(fx_rel) = ctrl.fixture.as_deref() else {
        return ControlEval::Stale {
            detail: "missing fixture path".to_owned(),
        };
    };
    let path = match verify_synthetic_fixture(&config.workspace_root, fx_rel) {
        Ok(p) => p,
        Err(why) => return ControlEval::Stale { detail: why },
    };
    let text = match fs::read_to_string(&path) {
        Ok(t) => t,
        Err(e) => {
            return ControlEval::Stale {
                detail: format!("read fixture: {e}"),
            };
        }
    };
    match examples::evaluate_aa_fixture(&text) {
        Ok(not_cleared_msg) => ControlEval::Failed {
            observed: not_cleared_msg,
        },
        Err(msg) => ControlEval::DidNotFail { detail: msg },
    }
}

fn emit_control_evidence(line: &ControlLine, evidence_dir: Option<&Path>) {
    let rec = line.to_evidence_record();
    match evidence_dir {
        Some(dir) => {
            let _ = emit_to_dir(dir, &rec);
        }
        None => emit_if_configured(&rec),
    }
}

fn parse_u32_field(text: &str, key: &str) -> Option<u32> {
    for raw in text.lines() {
        let line = raw.split('#').next().unwrap_or("").trim();
        if let Some((k, v)) = line.split_once('=')
            && k.trim() == key
        {
            return v.trim().trim_matches('"').parse::<u32>().ok();
        }
    }
    None
}

fn parse_f64_field(text: &str, key: &str) -> Option<f64> {
    for raw in text.lines() {
        let line = raw.split('#').next().unwrap_or("").trim();
        if let Some((k, v)) = line.split_once('=')
            && k.trim() == key
        {
            return v.trim().trim_matches('"').parse::<f64>().ok();
        }
    }
    None
}
