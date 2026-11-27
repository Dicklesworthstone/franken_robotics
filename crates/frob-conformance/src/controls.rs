#![deny(unsafe_code)]

//! Negative-control registry validation, serialization, and gate check (`BD-025`).
//!
//! `frob_core::testkit::registry::Controls` is the single parser of
//! `crates/frob-conformance/registry/controls.toml` (STD-TEST: one reader per file).
//! This module owns the six loader validation rules, the canonical serializer, and
//! the `GATE-GAUNT-01` v0 registry check (including the milestone `due` filter from
//! `N38b-04` / BD-025 review note 8).

#[cfg(all(feature = "gauntlet-mutations", not(debug_assertions)))]
compile_error!(
    "frob-conformance feature `gauntlet-mutations` is forbidden in release builds \
     (`not(debug_assertions)`); see BD-025 / ADR-GAUNT-03"
);

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::fs;
use std::path::Path;

use frob_core::testkit::registry::{Control, ControlKind, ControlState, Controls};

use crate::gates_v0::{GatePartStatus, GateRegistry, GateRow, parse_milestone_num};

/// Canonical workspace-relative path of the negative-control registry.
pub const CONTROLS_PATH: &str = frob_core::testkit::registry::CONTROLS_PATH;

/// Expected schema header for `registry/controls.toml`.
pub const CONTROLS_SCHEMA: &str = "frob.gauntlet.controls/1";

/// Typed validation and registry-check reasons (`N38-REF` / `BD-025`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ControlReason {
    // Loader reasons (12)
    OutOfOrder,
    Reused,
    Removed,
    Renumbered,
    NoGate,
    UnknownGate,
    KindNotClosed,
    FeatureMissing,
    FixtureMissing,
    DefectOrExpectEmpty,
    BoundaryNotControl,
    UnknownSeedForm,
    PermanentControlRemoved,
    // Registry check reasons (4)
    NoControl,
    DanglingControl,
    OneWayLink,
    UncaughtControl,
    // Raw TOML / schema parse failure outside the 12 loader reasons
    ParseError,
}

impl ControlReason {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OutOfOrder => "out-of-order",
            Self::Reused => "reused",
            Self::Removed => "removed",
            Self::Renumbered => "renumbered",
            Self::NoGate => "no-gate",
            Self::UnknownGate => "unknown-gate",
            Self::KindNotClosed => "kind-not-closed",
            Self::FeatureMissing => "feature-missing",
            Self::FixtureMissing => "fixture-missing",
            Self::DefectOrExpectEmpty => "defect-or-expect-empty",
            Self::BoundaryNotControl => "boundary-not-control",
            Self::UnknownSeedForm => "unknown-seed-form",
            Self::PermanentControlRemoved => "permanent-control-removed",
            Self::NoControl => "no-control",
            Self::DanglingControl => "dangling-control",
            Self::OneWayLink => "one-way-link",
            Self::UncaughtControl => "uncaught-control",
            Self::ParseError => "parse-error",
        }
    }
}

impl fmt::Display for ControlReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// One typed finding naming the record (`NC-nnnn` or `GATE-...`), reason, and detail.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ControlFinding {
    pub record: String,
    pub reason: ControlReason,
    pub detail: String,
}

impl fmt::Display for ControlFinding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] {}: {}",
            self.reason.as_str(),
            self.record,
            self.detail
        )
    }
}

/// A gate part that has no due runnable control and therefore cannot pass yet.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct NotEligiblePart {
    pub gate_id: String,
    pub milestone: String,
    pub reason: String,
}

impl fmt::Display for NotEligiblePart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({}): not-eligible ({})",
            self.gate_id, self.milestone, self.reason
        )
    }
}

/// An obligation control reported beside its gate (does not support a pass on its own).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ObligationNote {
    pub gate_id: String,
    pub milestone: String,
    pub control_id: String,
}

impl fmt::Display for ObligationNote {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({}): obligation {}",
            self.gate_id, self.milestone, self.control_id
        )
    }
}

/// Deterministic report returned by [`check_registry`] (determinism class D0).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegistryCheckReport {
    pub gates_checked: usize,
    pub controls_loaded: usize,
    pub findings: Vec<ControlFinding>,
    pub not_eligible: Vec<NotEligiblePart>,
    pub obligations: Vec<ObligationNote>,
}

impl RegistryCheckReport {
    #[must_use]
    pub fn is_ok(&self) -> bool {
        self.findings.is_empty()
    }

    /// Format a deterministic summary (D0; invariant to `[[gate]]` row order).
    #[must_use]
    pub fn format_summary(&self) -> String {
        let mut out = format!(
            "registry_check: gates={} controls={} findings={} not_eligible={} obligations={}\n",
            self.gates_checked,
            self.controls_loaded,
            self.findings.len(),
            self.not_eligible.len(),
            self.obligations.len()
        );
        for ne in &self.not_eligible {
            out.push_str(&format!("  {ne}\n"));
        }
        for ob in &self.obligations {
            out.push_str(&format!("  {ob}\n"));
        }
        for f in &self.findings {
            out.push_str(&format!("  FAIL {f}\n"));
        }
        out
    }
}

/// Returns `true` if `id` matches `NC-` plus exactly four ASCII digits.
#[must_use]
pub fn parse_nc_id(id: &str) -> Option<u32> {
    let digits = id.strip_prefix("NC-")?;
    if digits.len() != 4 || !digits.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    digits.parse::<u32>().ok()
}

/// Returns `true` if `target` is a seeding placeholder (`<bead>:<plant>` with one colon,
/// or `CX-<digits>`). Real Rust module paths contain `::` and are not placeholders.
#[must_use]
pub fn is_placeholder_target(target: &str) -> bool {
    let t = target.trim();
    if t.is_empty() || t.contains("::") || t.bytes().any(|b| b.is_ascii_whitespace()) {
        return false;
    }
    if let Some(digits) = t.strip_prefix("CX-") {
        return !digits.is_empty() && digits.bytes().all(|b| b.is_ascii_digit());
    }
    let mut parts = t.split(':');
    let Some(bead) = parts.next() else {
        return false;
    };
    let Some(plant) = parts.next() else {
        return false;
    };
    if parts.next().is_some() || bead.is_empty() || plant.is_empty() {
        return false;
    }
    // Bead key starts with an uppercase letter and contains uppercase/lowercase letters, digits, or hyphens.
    let first_ok = bead.as_bytes()[0].is_ascii_uppercase();
    let bead_ok = bead.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-');
    let plant_ok = plant
        .bytes()
        .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_' || b == b'.');
    first_ok && bead_ok && plant_ok && bead.contains('-')
}

/// Checks Rule 6: admitted `seed` identifier forms (`BD-025`).
#[must_use]
pub fn is_valid_seed_form(seed: &str, permanent: bool) -> bool {
    if seed.is_empty() {
        return permanent;
    }
    let all_digits = |s: &str| !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit());
    let is_area = |s: &str| {
        !s.is_empty()
            && s.as_bytes()[0].is_ascii_uppercase()
            && s.bytes()
                .all(|b| b.is_ascii_uppercase() || b.is_ascii_digit())
    };

    if let Some(rest) = seed.strip_prefix("CX-") {
        return all_digits(rest);
    }
    if let Some(rest) = seed.strip_prefix("KA-") {
        return all_digits(rest);
    }
    if let Some(rest) = seed.strip_prefix("MR-C") {
        return all_digits(rest);
    }
    if let Some(rest) = seed.strip_prefix("MR-") {
        return all_digits(rest);
    }
    if let Some(rest) = seed.strip_prefix("CERT-") {
        return all_digits(rest);
    }
    if let Some(rest) = seed.strip_prefix("RISK-") {
        return all_digits(rest);
    }
    if let Some(rest) = seed.strip_prefix("SAFE-X") {
        return all_digits(rest);
    }
    for prefix in ["INV-", "PO-", "GATE-"] {
        if let Some(rest) = seed.strip_prefix(prefix) {
            if let Some((area, num)) = rest.split_once('-') {
                return is_area(area) && all_digits(num);
            }
            return false;
        }
    }
    false
}

/// Parse `controls.toml` via the single parser (`frob_core::testkit::registry::Controls`)
/// and run all six loader validation rules against `previous`, `gates`, and `workspace_root`.
pub fn load_and_validate(
    toml_text: &str,
    file: &str,
    previous: Option<&Controls>,
    gates: &GateRegistry,
    workspace_root: &Path,
) -> Result<Controls, Vec<ControlFinding>> {
    let controls = match Controls::parse_str(toml_text, file) {
        Ok(c) => c,
        Err(err) => {
            let reason = classify_parse_error(&err.message);
            let record = err.row.clone().unwrap_or_else(|| file.to_owned());
            return Err(vec![ControlFinding {
                record,
                reason,
                detail: err.to_string(),
            }]);
        }
    };
    let findings = validate(&controls, previous, gates, workspace_root);
    if findings.is_empty() {
        Ok(controls)
    } else {
        Err(findings)
    }
}

fn classify_parse_error(msg: &str) -> ControlReason {
    if msg.contains("duplicate id") {
        ControlReason::Reused
    } else if msg.contains("out-of-order id") || msg.contains("invalid id") {
        ControlReason::OutOfOrder
    } else if msg.contains("unknown control kind") {
        ControlReason::KindNotClosed
    } else {
        ControlReason::ParseError
    }
}

/// Validate parsed `Controls` rows against Rules 1–6 of `BD-025`.
#[must_use]
pub fn validate(
    controls: &Controls,
    previous: Option<&Controls>,
    gates: &GateRegistry,
    workspace_root: &Path,
) -> Vec<ControlFinding> {
    let mut findings = Vec::new();

    // The latest recorded part determines which plants must already exist.
    let mut active_gates = BTreeMap::new();
    let mut active_controls = BTreeMap::<&str, u32>::new();
    for gate in gates.all() {
        let milestone = gate
            .parts
            .iter()
            .filter(|part| !part.status.is_not_run())
            .filter_map(|part| parse_milestone_num(&part.milestone))
            .max();
        if let Some(milestone) = milestone {
            active_gates.insert(gate.id.as_str(), milestone);
            for control_id in &gate.controls {
                active_controls
                    .entry(control_id.as_str())
                    .and_modify(|latest| *latest = (*latest).max(milestone))
                    .or_insert(milestone);
            }
        }
    }

    // Rule 1a: NC-nnnn format, uniqueness, strictly increasing order.
    let mut seen = BTreeSet::new();
    let mut prev_num: Option<u32> = None;
    let mut current_by_id: BTreeMap<&str, &Control> = BTreeMap::new();
    for c in controls.all() {
        match parse_nc_id(&c.id) {
            None => findings.push(ControlFinding {
                record: c.id.clone(),
                reason: ControlReason::OutOfOrder,
                detail: format!(
                    "control id `{}` does not match `NC-nnnn` (four digits)",
                    c.id
                ),
            }),
            Some(num) => {
                if !seen.insert(c.id.as_str()) {
                    findings.push(ControlFinding {
                        record: c.id.clone(),
                        reason: ControlReason::Reused,
                        detail: format!("duplicate control id `{}`", c.id),
                    });
                } else if let Some(prev) = prev_num
                    && num <= prev
                {
                    findings.push(ControlFinding {
                        record: c.id.clone(),
                        reason: ControlReason::OutOfOrder,
                        detail: format!(
                            "control id `{}` is not strictly greater than NC-{prev:04}",
                            c.id
                        ),
                    });
                } else {
                    prev_num = Some(num);
                }
            }
        }
        current_by_id.insert(c.id.as_str(), c);
    }

    // Rule 1b & N38b-04 Rule 5: diff against previous commit (append-only; one-time
    // placeholder target replacement; permanent controls cannot be deleted or detached).
    if let Some(prev) = previous {
        for old in prev.all() {
            match current_by_id.get(old.id.as_str()) {
                None => {
                    if old.permanent {
                        findings.push(ControlFinding {
                            record: old.id.clone(),
                            reason: ControlReason::PermanentControlRemoved,
                            detail: format!(
                                "permanent control `{}` from previous commit cannot be deleted",
                                old.id
                            ),
                        });
                    } else {
                        findings.push(ControlFinding {
                            record: old.id.clone(),
                            reason: ControlReason::Removed,
                            detail: format!(
                                "control `{}` from previous commit was removed",
                                old.id
                            ),
                        });
                    }
                }
                Some(new) => {
                    if old.permanent {
                        if !new.permanent {
                            findings.push(ControlFinding {
                                record: old.id.clone(),
                                reason: ControlReason::PermanentControlRemoved,
                                detail: format!(
                                    "permanent control `{}` cannot clear `permanent = true`",
                                    old.id
                                ),
                            });
                        }
                        for old_gate in &old.gate {
                            if !new.gate.iter().any(|g| g == old_gate) {
                                findings.push(ControlFinding {
                                    record: old.id.clone(),
                                    reason: ControlReason::PermanentControlRemoved,
                                    detail: format!(
                                        "permanent control `{}` cannot be detached from gate `{old_gate}`",
                                        old.id
                                    ),
                                });
                            }
                        }
                    }
                    if new.seed != old.seed {
                        findings.push(ControlFinding {
                            record: old.id.clone(),
                            reason: ControlReason::Renumbered,
                            detail: format!(
                                "control `{}` changed seed from `{}` to `{}`",
                                old.id, old.seed, new.seed
                            ),
                        });
                    }
                    if new.kind != old.kind {
                        findings.push(ControlFinding {
                            record: old.id.clone(),
                            reason: ControlReason::Renumbered,
                            detail: format!(
                                "control `{}` changed kind from `{}` to `{}`",
                                old.id,
                                kind_str(old.kind),
                                kind_str(new.kind)
                            ),
                        });
                    }
                    if new.target != old.target {
                        let placeholder_upgrade = old.last_seen.is_none()
                            && is_placeholder_target(&old.target)
                            && !is_placeholder_target(&new.target);
                        if !placeholder_upgrade {
                            findings.push(ControlFinding {
                                record: old.id.clone(),
                                reason: ControlReason::Renumbered,
                                detail: format!(
                                    "control `{}` changed target from `{}` to `{}` (not an eligible one-time placeholder replacement)",
                                    old.id, old.target, new.target
                                ),
                            });
                        }
                    }
                }
            }
        }
    }

    // Rules 2–6 per control row.
    for c in controls.all() {
        // Rule 2: gate non-empty and every entry exists in registries/gates.toml.
        if c.gate.is_empty() {
            findings.push(ControlFinding {
                record: c.id.clone(),
                reason: ControlReason::NoGate,
                detail: "control `gate` list is empty".to_owned(),
            });
        } else {
            for gid in &c.gate {
                if gates.get(gid).is_none() {
                    findings.push(ControlFinding {
                        record: c.id.clone(),
                        reason: ControlReason::UnknownGate,
                        detail: format!("gate `{gid}` is absent from `registries/gates.toml`"),
                    });
                }
            }
        }

        // Rule 3: mutation names a feature; other four kinds name a fixture.
        // A plant must exist if it is due for any part with a recorded status.
        let active_gate = active_controls
            .get(c.id.as_str())
            .into_iter()
            .chain(
                c.gate
                    .iter()
                    .filter_map(|gate| active_gates.get(gate.as_str())),
            )
            .any(|milestone| !is_control_deferred_for_part(c, Some(*milestone)));

        match c.kind {
            ControlKind::Mutation => match c.feature.as_deref().map(str::trim) {
                None | Some("") => findings.push(ControlFinding {
                    record: c.id.clone(),
                    reason: ControlReason::FeatureMissing,
                    detail: "mutation control requires a non-empty `feature` field".to_owned(),
                }),
                Some(feat) if active_gate => {
                    if let Err(why) = verify_crate_declares_feature(workspace_root, &c.target, feat)
                    {
                        findings.push(ControlFinding {
                            record: c.id.clone(),
                            reason: ControlReason::FeatureMissing,
                            detail: why,
                        });
                    }
                }
                Some(_) => {}
            },
            ControlKind::PlantedInput
            | ControlKind::PlantedDesign
            | ControlKind::PlantedWorld
            | ControlKind::Aa => match c.fixture.as_deref().map(str::trim) {
                None | Some("") => findings.push(ControlFinding {
                    record: c.id.clone(),
                    reason: ControlReason::FixtureMissing,
                    detail: format!(
                        "{} control requires a non-empty `fixture` field",
                        kind_str(c.kind)
                    ),
                }),
                Some(fx) if active_gate => {
                    if let Err(why) = verify_fixture_exists_with_header(workspace_root, fx) {
                        findings.push(ControlFinding {
                            record: c.id.clone(),
                            reason: ControlReason::FixtureMissing,
                            detail: why,
                        });
                    }
                }
                Some(_) => {}
            },
        }

        // Rule 4: defect and expect non-empty; expect states an observable.
        if c.defect.trim().is_empty()
            || c.expect.trim().is_empty()
            || !expect_states_observable(&c.expect)
        {
            findings.push(ControlFinding {
                record: c.id.clone(),
                reason: ControlReason::DefectOrExpectEmpty,
                detail: "`defect` and `expect` must be non-empty and `expect` must state an observable (value, refusal code, or failing row)".to_owned(),
            });
        }

        // Rule 5: a control is a defect, not a boundary.
        if c.kind == ControlKind::PlantedInput
            && c.removed_check
                .as_deref()
                .is_none_or(|s| s.trim().is_empty())
            && (defect_restates_boundary_refusal(&c.defect)
                || expect_is_product_boundary_refusal(&c.expect))
        {
            findings.push(ControlFinding {
                record: c.id.clone(),
                reason: ControlReason::BoundaryNotControl,
                detail: "planted-input control restates a product boundary refusal without naming `removed_check`".to_owned(),
            });
        }

        // Rule 6: seed form closed.
        if !is_valid_seed_form(&c.seed, c.permanent) {
            findings.push(ControlFinding {
                record: c.id.clone(),
                reason: ControlReason::UnknownSeedForm,
                detail: format!(
                    "unrecognized seed form `{}` (permanent={})",
                    c.seed, c.permanent
                ),
            });
        }
    }

    findings.sort();
    findings.dedup();
    findings
}

/// Run the v0 `GATE-GAUNT-01` registry check (BD-025 + Review Note 8 due filter).
///
/// Output is strictly deterministic (class D0) and invariant to the row ordering
/// of `[[gate]]` in `registries/gates.toml`.
#[must_use]
pub fn check_registry(gates: &GateRegistry, controls: &Controls) -> RegistryCheckReport {
    let mut gates_by_id: BTreeMap<&str, &GateRow> = BTreeMap::new();
    for g in gates.all() {
        gates_by_id.insert(g.id.as_str(), g);
    }

    let mut controls_by_id: BTreeMap<&str, &Control> = BTreeMap::new();
    for c in controls.all() {
        controls_by_id.insert(c.id.as_str(), c);
    }

    let mut findings = Vec::new();
    let mut not_eligible = Vec::new();
    let mut obligations = Vec::new();

    for (gate_id, gate) in &gates_by_id {
        // Bidirectional link integrity (gate -> control) for every gate regardless of status.
        for cid in &gate.controls {
            match controls_by_id.get(cid.as_str()) {
                None => findings.push(ControlFinding {
                    record: (*gate_id).to_owned(),
                    reason: ControlReason::DanglingControl,
                    detail: format!("gate `{gate_id}` lists missing control `{cid}`"),
                }),
                Some(ctrl) => {
                    if !ctrl.gate.iter().any(|g| g == *gate_id) {
                        findings.push(ControlFinding {
                            record: (*gate_id).to_owned(),
                            reason: ControlReason::OneWayLink,
                            detail: format!(
                                "gate `{gate_id}` lists `{cid}`, but `{cid}` does not list `{gate_id}`"
                            ),
                        });
                    }
                }
            }
        }

        // Per-part evaluation with N38b-04 / Review Note 8 milestone `due` filter.
        for part in gate.effective_parts() {
            let part_m = parse_milestone_num(&part.milestone);
            let mut due_runnable: Vec<&Control> = Vec::new();
            let mut due_obligation: Vec<&Control> = Vec::new();
            let mut deferred_count = 0usize;

            for cid in &gate.controls {
                let Some(&ctrl) = controls_by_id.get(cid.as_str()) else {
                    continue;
                };
                if is_control_deferred_for_part(ctrl, part_m) {
                    deferred_count += 1;
                    continue;
                }
                match ctrl.state {
                    ControlState::Runnable => due_runnable.push(ctrl),
                    ControlState::Obligation => {
                        due_obligation.push(ctrl);
                        obligations.push(ObligationNote {
                            gate_id: (*gate_id).to_owned(),
                            milestone: part.milestone.clone(),
                            control_id: ctrl.id.clone(),
                        });
                    }
                }
            }

            match part.status {
                GatePartStatus::NotRun => {
                    if due_runnable.is_empty() {
                        let reason = if gate.controls.is_empty() {
                            "no listed controls".to_owned()
                        } else if deferred_count > 0 && due_obligation.is_empty() {
                            format!(
                                "all {deferred_count} listed control(s) due after {}",
                                part.milestone
                            )
                        } else {
                            "only obligation controls due at this milestone".to_owned()
                        };
                        not_eligible.push(NotEligiblePart {
                            gate_id: (*gate_id).to_owned(),
                            milestone: part.milestone.clone(),
                            reason,
                        });
                    }
                }
                GatePartStatus::Pass => {
                    if due_runnable.is_empty() {
                        let reason_detail = if gate.controls.is_empty() {
                            format!(
                                "gate `{gate_id}` ({}) has status `pass` with empty `controls`",
                                part.milestone
                            )
                        } else if !due_obligation.is_empty() {
                            format!(
                                "gate `{gate_id}` ({}) has status `pass` with only `obligation` controls and no due `runnable` control",
                                part.milestone
                            )
                        } else {
                            format!(
                                "gate `{gate_id}` ({}) has status `pass` with no control due at or before {}",
                                part.milestone, part.milestone
                            )
                        };
                        findings.push(ControlFinding {
                            record: (*gate_id).to_owned(),
                            reason: ControlReason::NoControl,
                            detail: reason_detail,
                        });
                    } else {
                        for ctrl in &due_runnable {
                            if let Err(msg) = check_control_caught_for_part(ctrl, &part) {
                                findings.push(ControlFinding {
                                    record: (*gate_id).to_owned(),
                                    reason: ControlReason::UncaughtControl,
                                    detail: format!(
                                        "gate `{gate_id}` ({}) control `{}`: {msg}",
                                        part.milestone, ctrl.id
                                    ),
                                });
                            }
                        }
                    }
                }
                _ => {
                    // Non-not-run statuses (fail, killed, blocked, no-claim, superseded):
                    // structural half of INV-GAUNT-02 with due filter.
                    if due_runnable.is_empty() && due_obligation.is_empty() {
                        findings.push(ControlFinding {
                            record: (*gate_id).to_owned(),
                            reason: ControlReason::NoControl,
                            detail: format!(
                                "gate `{gate_id}` ({}) has status `{}` without a due control",
                                part.milestone,
                                part.status.as_str()
                            ),
                        });
                    }
                }
            }
        }
    }

    // Bidirectional link integrity (control -> gate) for controls pointing at existing gates.
    for ctrl in controls.all() {
        for gid in &ctrl.gate {
            if let Some(gate) = gates_by_id.get(gid.as_str())
                && !gate.controls.iter().any(|c| c == &ctrl.id)
            {
                findings.push(ControlFinding {
                    record: gate.id.clone(),
                    reason: ControlReason::OneWayLink,
                    detail: format!(
                        "control `{}` lists gate `{}`, but `{}` does not list `{}`",
                        ctrl.id, gate.id, gate.id, ctrl.id
                    ),
                });
            }
        }
    }

    findings.sort();
    findings.dedup();
    not_eligible.sort();
    not_eligible.dedup();
    obligations.sort();
    obligations.dedup();

    RegistryCheckReport {
        gates_checked: gates_by_id.len(),
        controls_loaded: controls.all().len(),
        findings,
        not_eligible,
        obligations,
    }
}

fn is_control_deferred_for_part(ctrl: &Control, part_milestone: Option<u32>) -> bool {
    let (Some(pm), Some(due_str)) = (part_milestone, ctrl.due.as_deref()) else {
        return false;
    };
    parse_milestone_num(due_str).is_some_and(|dm| dm > pm)
}

fn check_control_caught_for_part(
    ctrl: &Control,
    part: &crate::gates_v0::GatePart,
) -> Result<(), String> {
    let Some(ls) = &ctrl.last_seen else {
        return Err("missing `last_seen` record".to_owned());
    };
    if !ls.caught {
        return Err("`last_seen.caught` is false".to_owned());
    }
    if ls.session.trim().is_empty() || ls.date.trim().is_empty() {
        return Err("`last_seen` has empty `session` or `date`".to_owned());
    }
    if !part.recorded.session.trim().is_empty() && ls.session != part.recorded.session {
        return Err(format!(
            "`last_seen.session` `{}` does not match gate part recorded session `{}`",
            ls.session, part.recorded.session
        ));
    }
    Ok(())
}

/// Canonical TOML serializer for one `[[control]]` record (used by `control_roundtrip`).
#[must_use]
pub fn serialize_control(c: &Control) -> String {
    let mut out = String::new();
    out.push_str("[[control]]\n");
    push_str_kv(&mut out, "id", &c.id);
    push_str_kv(&mut out, "seed", &c.seed);
    out.push_str("gate = [");
    for (i, g) in c.gate.iter().enumerate() {
        if i > 0 {
            out.push_str(", ");
        }
        push_escaped_str(&mut out, g);
    }
    out.push_str("]\n");
    push_str_kv(&mut out, "kind", kind_str(c.kind));
    push_str_kv(&mut out, "target", &c.target);
    push_str_kv(&mut out, "defect", &c.defect);
    push_str_kv(&mut out, "expect", &c.expect);
    if let Some(ls) = &c.last_seen {
        out.push_str("last_seen = { caught = ");
        out.push_str(if ls.caught { "true" } else { "false" });
        out.push_str(", session = ");
        push_escaped_str(&mut out, &ls.session);
        out.push_str(", date = ");
        push_escaped_str(&mut out, &ls.date);
        out.push_str(" }\n");
    }
    if let Some(f) = &c.feature {
        push_str_kv(&mut out, "feature", f);
    }
    if let Some(fx) = &c.fixture {
        push_str_kv(&mut out, "fixture", fx);
    }
    if let Some(d) = &c.due {
        push_str_kv(&mut out, "due", d);
    }
    if c.permanent {
        out.push_str("permanent = true\n");
    }
    if c.state == ControlState::Obligation {
        push_str_kv(&mut out, "state", "obligation");
    }
    if let Some(rc) = &c.removed_check {
        push_str_kv(&mut out, "removed_check", rc);
    }
    out
}

/// Canonical TOML serializer for an entire `controls.toml` document.
#[must_use]
pub fn serialize_controls_file(controls: &[Control]) -> String {
    let mut out = format!("schema = \"{CONTROLS_SCHEMA}\"\n");
    for c in controls {
        out.push('\n');
        out.push_str(&serialize_control(c));
    }
    out
}

#[must_use]
pub const fn kind_str(kind: ControlKind) -> &'static str {
    match kind {
        ControlKind::Mutation => "mutation",
        ControlKind::PlantedInput => "planted-input",
        ControlKind::PlantedDesign => "planted-design",
        ControlKind::PlantedWorld => "planted-world",
        ControlKind::Aa => "a-a",
    }
}

fn push_str_kv(out: &mut String, key: &str, val: &str) {
    out.push_str(key);
    out.push_str(" = ");
    push_escaped_str(out, val);
    out.push('\n');
}

fn push_escaped_str(out: &mut String, val: &str) {
    out.push('"');
    for ch in val.chars() {
        match ch {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c => out.push(c),
        }
    }
    out.push('"');
}

fn expect_states_observable(expect: &str) -> bool {
    if expect.bytes().any(|b| b.is_ascii_digit()) || expect.contains("FROB-REF-") {
        return true;
    }
    let lower = expect.to_ascii_lowercase();
    const OBSERVABLE_TOKENS: &[&str] = &[
        "fail",
        "refus",
        "reject",
        "catch",
        "caught",
        "error",
        "panic",
        "non-zero",
        "nonzero",
        "exit",
        "abort",
        "exceed",
        "mismatch",
        "diverg",
        "report",
        "no-control",
        "dangling-control",
        "one-way-link",
        "uncaught-control",
        "boundary-not-control",
        "unknown-target",
        "invalid",
        "missing",
        "violation",
        "drift",
        "stale",
        "obligation",
        "not-eligible",
        "false",
        "true",
    ];
    OBSERVABLE_TOKENS.iter().any(|tok| lower.contains(tok))
}

fn defect_restates_boundary_refusal(defect: &str) -> bool {
    let lower = defect.to_ascii_lowercase();
    const PHRASES: &[&str] = &[
        "is refused",
        "refused at",
        "refuses at",
        "is rejected",
        "rejected at",
        "rejects at",
        "refusal at the limit",
    ];
    PHRASES.iter().any(|p| lower.contains(p))
}

fn expect_is_product_boundary_refusal(expect: &str) -> bool {
    if expect.contains("FROB-REF-") {
        return true;
    }
    let lower = expect.to_ascii_lowercase();
    lower.contains("request is refused")
        || lower.contains("refused at the limit")
        || lower.contains("returns product refusal")
}

fn verify_crate_declares_feature(
    workspace_root: &Path,
    target: &str,
    feature: &str,
) -> Result<(), String> {
    let (crate_name, feat_name) = if let Some((c, f)) = feature.split_once('/') {
        (c.trim().replace('_', "-"), f.trim().to_owned())
    } else {
        let inferred = target
            .split("::")
            .next()
            .unwrap_or("")
            .trim()
            .replace('_', "-");
        (inferred, feature.trim().to_owned())
    };
    if crate_name.is_empty() || feat_name.is_empty() {
        return Err(format!(
            "cannot resolve target crate for mutation feature `{feature}` (target `{target}`)"
        ));
    }

    let manifest_path = find_crate_manifest(workspace_root, &crate_name).ok_or_else(|| {
        format!("target crate `{crate_name}` manifest not found for mutation feature `{feat_name}`")
    })?;
    let content = fs::read_to_string(&manifest_path)
        .map_err(|e| format!("failed to read `{}`: {e}", manifest_path.display()))?;
    let features = parse_manifest_feature_names(&content);
    if features.contains(&feat_name) {
        Ok(())
    } else {
        Err(format!(
            "mutation feature `{feat_name}` is not declared under [features] in `{}`",
            manifest_path.display()
        ))
    }
}

fn find_crate_manifest(workspace_root: &Path, crate_name: &str) -> Option<std::path::PathBuf> {
    let direct = workspace_root
        .join("crates")
        .join(crate_name)
        .join("Cargo.toml");
    if direct.is_file() {
        return Some(direct);
    }
    let crates_dir = workspace_root.join("crates");
    let rd = fs::read_dir(crates_dir).ok()?;
    let mut dirs: Vec<_> = rd.flatten().map(|e| e.path()).collect();
    dirs.sort();
    for dir in dirs {
        let m = dir.join("Cargo.toml");
        if !m.is_file() {
            continue;
        }
        if let Ok(txt) = fs::read_to_string(&m)
            && parse_manifest_package_name(&txt).as_deref() == Some(crate_name)
        {
            return Some(m);
        }
    }
    None
}

fn parse_manifest_package_name(content: &str) -> Option<String> {
    let mut in_pkg = false;
    for raw in content.lines() {
        let line = strip_toml_comment(raw).trim();
        if line.is_empty() {
            continue;
        }
        if line.starts_with('[') && line.ends_with(']') {
            in_pkg = line == "[package]";
            continue;
        }
        if in_pkg
            && let Some((k, v)) = line.split_once('=')
            && k.trim() == "name"
        {
            return Some(v.trim().trim_matches('"').trim_matches('\'').to_owned());
        }
    }
    None
}

fn parse_manifest_feature_names(content: &str) -> BTreeSet<String> {
    let mut names = BTreeSet::new();
    let mut in_features = false;
    for raw in content.lines() {
        let line = strip_toml_comment(raw).trim();
        if line.is_empty() {
            continue;
        }
        if line.starts_with('[') && line.ends_with(']') {
            in_features = line == "[features]";
            continue;
        }
        if in_features && let Some((k, _)) = line.split_once('=') {
            let key = k.trim().trim_matches('"').trim_matches('\'');
            if !key.is_empty() {
                names.insert(key.to_owned());
            }
        }
    }
    names
}

fn strip_toml_comment(line: &str) -> &str {
    let (mut in_dq, mut in_sq, mut esc) = (false, false, false);
    for (i, b) in line.bytes().enumerate() {
        if esc {
            esc = false;
            continue;
        }
        if in_dq {
            if b == b'\\' {
                esc = true;
            } else if b == b'"' {
                in_dq = false;
            }
        } else if in_sq {
            if b == b'\'' {
                in_sq = false;
            }
        } else if b == b'"' {
            in_dq = true;
        } else if b == b'\'' {
            in_sq = true;
        } else if b == b'#' {
            return &line[..i];
        }
    }
    line
}

fn verify_fixture_exists_with_header(
    workspace_root: &Path,
    fixture_rel: &str,
) -> Result<(), String> {
    let rel = Path::new(fixture_rel);
    if rel.is_absolute()
        || rel
            .components()
            .any(|c| matches!(c, std::path::Component::ParentDir))
    {
        return Err(format!(
            "fixture path `{fixture_rel}` must be workspace-relative without `..`"
        ));
    }
    let path = workspace_root.join(rel);
    if !path.is_file() {
        return Err(format!("fixture file `{fixture_rel}` does not exist"));
    }
    let content = fs::read_to_string(&path)
        .map_err(|e| format!("failed to read fixture `{fixture_rel}`: {e}"))?;
    if has_fixture_kind_header(&content) {
        Ok(())
    } else {
        Err(format!(
            "fixture `{fixture_rel}` must begin with a kind header (`synthetic`, `oracle`, or `recorded`)"
        ))
    }
}

fn has_fixture_kind_header(content: &str) -> bool {
    let head = content
        .lines()
        .take(8)
        .collect::<Vec<_>>()
        .join("\n")
        .to_ascii_lowercase();
    for kind in ["synthetic", "oracle", "recorded"] {
        if head.contains(&format!("# {kind}"))
            || head.contains(&format!("#{kind}"))
            || head.contains(&format!("// {kind}"))
            || head.contains(&format!("kind: {kind}"))
            || head.contains(&format!("kind:{kind}"))
            || head.contains(&format!("kind = \"{kind}\""))
            || head.contains(&format!("fixture: {kind}"))
            || head.contains(&format!("fixture_kind = \"{kind}\""))
            || head.contains(&format!("\"kind\":\"{kind}\""))
            || head.contains(&format!("\"kind\": \"{kind}\""))
        {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn placeholder_target_detection() {
        assert!(is_placeholder_target("BD-123:cx-262"));
        assert!(is_placeholder_target("N16b-15:plant-a"));
        assert!(is_placeholder_target("CX-301"));
        assert!(!is_placeholder_target("frob-actuate::stage::integrate"));
        assert!(!is_placeholder_target(
            "crates/frob-conformance/tests/registry_check.rs"
        ));
        assert!(!is_placeholder_target(""));
    }

    #[test]
    fn seed_form_unit_cases() {
        for good in [
            "CX-262",
            "KA-501",
            "MR-14",
            "MR-C07",
            "CERT-07",
            "INV-CHRON-14",
            "PO-GAUNT-03",
            "RISK-37",
            "SAFE-X01",
            "GATE-GAUNT-10",
        ] {
            assert!(
                is_valid_seed_form(good, false),
                "expected valid seed `{good}`"
            );
        }
        assert!(is_valid_seed_form("", true));
        assert!(!is_valid_seed_form("", false));
        for bad in [
            "BUG-7",
            "cx-262",
            "MR-X07",
            "INV-bad-1",
            "GATE-10",
            "SAFE-01",
        ] {
            assert!(
                !is_valid_seed_form(bad, false),
                "expected invalid seed `{bad}`"
            );
        }
    }
}
