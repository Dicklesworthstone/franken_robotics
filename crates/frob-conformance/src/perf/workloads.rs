//! Conformance lint and KA-815 checks for `registries/workloads.toml` (`N36-02`).
//!
//! Built strictly on top of [`frob_core::perf::workload`]: this module contains zero
//! second parser and checks semantic invariants over the typed records returned by
//! `frob-core`.

use std::collections::BTreeSet;
use std::fmt;
use std::path::Path;

use frob_core::perf::workload::{
    CONTROL_PERIOD_20MS, ComparatorSetting, FLOATING_BASE_DOF, SettingKind, Workload,
    WorkloadError, WorkloadId, WorkloadRegistry, WorkloadStatus, checked_row_cap,
    checked_row_cap_raw,
};

/// Canonical workspace path of the reference workload registry.
pub const WORKLOADS_PATH: &str = "registries/workloads.toml";

/// Required reference workload names in `registries/workloads.toml`.
pub const REQUIRED_REFERENCE_WORKLOADS: &[&str] = &["Q", "H"];

/// Parity-baseline comparator arms expected in the committed workload registry
/// (`N36-02`, `BD-039`, `N36-25`, §36.3).
pub const PARITY_BASELINE_COMPARATORS: &[&str] = &[
    "mujoco-warp",
    "mujoco-playground",
    "mujoco-cpu",
    "newton",
    "isaac-lab",
    "genesis",
    "drake",
    "maniskill",
];

/// Machine-readable reason code for a workload registry lint finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WorkloadFindingKind {
    MissingReferenceWorkload,
    DofMismatch,
    RowCapOverflow,
    RowCapRuleMismatch,
    OverrideRuleMismatch,
    EmptyOverrideReason,
    NonIntegerDecimation,
    ControlPeriodMismatch,
    FrozenMissingAdr,
    EmptyGrounds,
    DuplicateGround,
    SettingUnknownWorkload,
    DuplicateSetting,
    NativeSettingMissingSource,
    MatchedSettingMissingDifference,
    EmptySettingDifference,
    IncompleteComparatorSettings,
    MissingParityComparator,
    Ka815Mismatch,
}

/// One semantic lint violation found in a parsed [`WorkloadRegistry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkloadFinding {
    pub kind: WorkloadFindingKind,
    pub subject: String,
    pub message: String,
}

impl WorkloadFinding {
    #[must_use]
    pub fn new(
        kind: WorkloadFindingKind,
        subject: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            subject: subject.into(),
            message: message.into(),
        }
    }
}

impl fmt::Display for WorkloadFinding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{:?}] {}: {}", self.kind, self.subject, self.message)
    }
}

/// Combined parse + semantic lint failure returned by [`lint_bytes`] / [`load_and_lint`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorkloadLintError {
    Parse(WorkloadError),
    Findings(Vec<WorkloadFinding>),
}

impl fmt::Display for WorkloadLintError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parse(err) => write!(f, "{err}"),
            Self::Findings(items) => {
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        f.write_str("\n")?;
                    }
                    write!(f, "{item}")?;
                }
                Ok(())
            }
        }
    }
}

impl std::error::Error for WorkloadLintError {}

impl From<WorkloadError> for WorkloadLintError {
    fn from(err: WorkloadError) -> Self {
        Self::Parse(err)
    }
}

/// Verified KA-815 row-capacity accounting for one reference workload.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ka815Row {
    pub name: String,
    pub id: WorkloadId,
    pub contact_cap: u32,
    pub actuated: u32,
    pub raw_sum: u32,
    pub rule_cap: u32,
    pub registered_cap: u32,
    pub override_reason: Option<String>,
}

/// Verified KA-815 report for reference workloads Q and H.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ka815Report {
    pub q: Ka815Row,
    pub h: Ka815Row,
}

impl Ka815Report {
    /// Format a deterministic summary including the intermediate sums `152` and `314`,
    /// rule values `152` and `320`, registered capacities `160` and `320`, and identities.
    #[must_use]
    pub fn format_summary(&self) -> String {
        let q_ov = self
            .q
            .override_reason
            .as_deref()
            .unwrap_or("none");
        format!(
            "KA-815 Q ({id_q}): 4*{c_q} + 2*{a_q} = {raw_q} -> rule {rule_q} -> registered {reg_q} (override: {q_ov})\n\
             KA-815 H ({id_h}): 4*{c_h} + 2*{a_h} = {raw_h} -> rule {rule_h} -> registered {reg_h}",
            id_q = self.q.id,
            c_q = self.q.contact_cap,
            a_q = self.q.actuated,
            raw_q = self.q.raw_sum,
            rule_q = self.q.rule_cap,
            reg_q = self.q.registered_cap,
            id_h = self.h.id,
            c_h = self.h.contact_cap,
            a_h = self.h.actuated,
            raw_h = self.h.raw_sum,
            rule_h = self.h.rule_cap,
            reg_h = self.h.registered_cap,
        )
    }
}

/// Lint one parsed [`Workload`] record against the N36-02 invariants.
#[must_use]
pub fn lint_workload(workload: &Workload) -> Vec<WorkloadFinding> {
    let mut findings = Vec::new();
    let subject = format!("workload.{}", workload.name);

    // 1. dof == 6 + actuated (floating base + actuated joints).
    match workload.actuated.checked_add(FLOATING_BASE_DOF) {
        Some(expected_dof) if workload.dof == expected_dof => {}
        Some(expected_dof) => findings.push(WorkloadFinding::new(
            WorkloadFindingKind::DofMismatch,
            &subject,
            format!(
                "dof = {} does not equal 6 + actuated ({} + {} = {expected_dof})",
                workload.dof, FLOATING_BASE_DOF, workload.actuated
            ),
        )),
        None => findings.push(WorkloadFinding::new(
            WorkloadFindingKind::DofMismatch,
            &subject,
            format!("6 + actuated ({}) overflows u32", workload.actuated),
        )),
    }

    // 2. row_cap matches row_cap(contact_cap, actuated), or carries a valid row_cap_override.
    match (
        checked_row_cap_raw(workload.contact_cap, workload.actuated),
        checked_row_cap(workload.contact_cap, workload.actuated),
    ) {
        (Some(raw), Some(rule_val)) => {
            if workload.row_cap == rule_val {
                if let Some(ref ov) = workload.row_cap_override {
                    if ov.rule != rule_val {
                        findings.push(WorkloadFinding::new(
                            WorkloadFindingKind::OverrideRuleMismatch,
                            &subject,
                            format!(
                                "row_cap_override.rule = {} does not match row_cap({}, {}) = {rule_val} (raw {raw})",
                                ov.rule, workload.contact_cap, workload.actuated
                            ),
                        ));
                    }
                    if ov.reason.trim().is_empty() {
                        findings.push(WorkloadFinding::new(
                            WorkloadFindingKind::EmptyOverrideReason,
                            &subject,
                            "row_cap_override.reason must be non-empty",
                        ));
                    }
                }
            } else {
                match &workload.row_cap_override {
                    None => findings.push(WorkloadFinding::new(
                        WorkloadFindingKind::RowCapRuleMismatch,
                        &subject,
                        format!(
                            "row_cap = {} differs from row_cap({}, {}) = {rule_val} (raw {raw}) with no row_cap_override",
                            workload.row_cap, workload.contact_cap, workload.actuated
                        ),
                    )),
                    Some(ov) => {
                        if ov.rule != rule_val {
                            findings.push(WorkloadFinding::new(
                                WorkloadFindingKind::OverrideRuleMismatch,
                                &subject,
                                format!(
                                    "row_cap_override.rule = {} does not match row_cap({}, {}) = {rule_val} (raw {raw})",
                                    ov.rule, workload.contact_cap, workload.actuated
                                ),
                            ));
                        }
                        if ov.reason.trim().is_empty() {
                            findings.push(WorkloadFinding::new(
                                WorkloadFindingKind::EmptyOverrideReason,
                                &subject,
                                "row_cap_override.reason must be non-empty when row_cap overrides the rule",
                            ));
                        }
                    }
                }
            }
        }
        _ => findings.push(WorkloadFinding::new(
            WorkloadFindingKind::RowCapOverflow,
            &subject,
            format!(
                "row_cap({}, {}) overflows u32",
                workload.contact_cap, workload.actuated
            ),
        )),
    }

    // 3. Decimation must be an integer for every reference workload (§36.2).
    if !workload.decimation.is_integer() {
        findings.push(WorkloadFinding::new(
            WorkloadFindingKind::NonIntegerDecimation,
            &subject,
            format!(
                "decimation `{}` must be an integer for reference workloads",
                workload.decimation
            ),
        ));
    }

    // 4. Control period h_s * decimation == 1/50 s (20 ms, 50 Hz control).
    match workload.control_period() {
        Some(period) if period == CONTROL_PERIOD_20MS => {}
        Some(period) => findings.push(WorkloadFinding::new(
            WorkloadFindingKind::ControlPeriodMismatch,
            &subject,
            format!(
                "control period h_s * decimation = {} * {} = {period} s; expected {} s (20 ms)",
                workload.h_s, workload.decimation, CONTROL_PERIOD_20MS
            ),
        )),
        None => findings.push(WorkloadFinding::new(
            WorkloadFindingKind::ControlPeriodMismatch,
            &subject,
            format!(
                "control period h_s * decimation ({} * {}) overflowed u64",
                workload.h_s, workload.decimation
            ),
        )),
    }

    // 5. status = "frozen" requires a non-empty ADR reference.
    if workload.status == WorkloadStatus::Frozen {
        let missing_adr = workload
            .adr
            .as_deref()
            .is_none_or(|s| s.trim().is_empty());
        if missing_adr {
            findings.push(WorkloadFinding::new(
                WorkloadFindingKind::FrozenMissingAdr,
                &subject,
                "status = \"frozen\" requires a non-empty `adr` reference",
            ));
        }
    }

    // 6. Grounds must be non-empty and deduplicated.
    if workload.grounds.is_empty() {
        findings.push(WorkloadFinding::new(
            WorkloadFindingKind::EmptyGrounds,
            &subject,
            "grounds must list at least one ground variant",
        ));
    } else {
        let mut seen = BTreeSet::new();
        for g in &workload.grounds {
            if !seen.insert(g.as_str()) {
                findings.push(WorkloadFinding::new(
                    WorkloadFindingKind::DuplicateGround,
                    &subject,
                    format!("duplicate ground variant `{g}`"),
                ));
            }
        }
    }

    findings
}

/// Lint a parsed [`WorkloadRegistry`] (workloads, settings, and per-comparator coverage).
#[must_use]
pub fn lint_registry(registry: &WorkloadRegistry) -> Vec<WorkloadFinding> {
    let mut findings = Vec::new();

    for &req in REQUIRED_REFERENCE_WORKLOADS {
        if !registry.workloads.contains_key(req) {
            findings.push(WorkloadFinding::new(
                WorkloadFindingKind::MissingReferenceWorkload,
                format!("workload.{req}"),
                format!("required reference workload `{req}` is missing"),
            ));
        }
    }

    for workload in registry.workloads.values() {
        findings.extend(lint_workload(workload));
    }

    let mut seen_settings: BTreeSet<(&str, &str, SettingKind)> = BTreeSet::new();
    for setting in &registry.settings {
        lint_setting(setting, registry, &mut seen_settings, &mut findings);
    }

    // Every comparator declared in `[[setting]]` must provide both `matched` and `native`
    // for every reference workload (`Q` and `H`).
    for comparator in registry.comparators() {
        for &wl in REQUIRED_REFERENCE_WORKLOADS {
            for kind in [SettingKind::Matched, SettingKind::Native] {
                if !seen_settings.contains(&(comparator, wl, kind)) {
                    findings.push(WorkloadFinding::new(
                        WorkloadFindingKind::IncompleteComparatorSettings,
                        format!("setting.{comparator}.{wl}.{}", kind.as_str()),
                        format!(
                            "comparator `{comparator}` is missing `{}` setting for reference workload `{wl}`",
                            kind.as_str()
                        ),
                    ));
                }
            }
        }
    }

    findings
}

fn lint_setting<'a>(
    setting: &'a ComparatorSetting,
    registry: &WorkloadRegistry,
    seen: &mut BTreeSet<(&'a str, &'a str, SettingKind)>,
    findings: &mut Vec<WorkloadFinding>,
) {
    let subject = format!(
        "setting.{}.{}.{}",
        setting.comparator,
        setting.workload,
        setting.kind.as_str()
    );

    if !seen.insert((&setting.comparator, &setting.workload, setting.kind)) {
        findings.push(WorkloadFinding::new(
            WorkloadFindingKind::DuplicateSetting,
            &subject,
            "duplicate (comparator, workload, kind) setting record",
        ));
    }

    let target = registry.get(&setting.workload);
    if target.is_none() {
        findings.push(WorkloadFinding::new(
            WorkloadFindingKind::SettingUnknownWorkload,
            &subject,
            format!("setting references unknown workload `{}`", setting.workload),
        ));
    }

    match setting.kind {
        SettingKind::Native => {
            let missing_source = setting
                .source
                .as_deref()
                .is_none_or(|s| s.trim().is_empty());
            if missing_source {
                findings.push(WorkloadFinding::new(
                    WorkloadFindingKind::NativeSettingMissingSource,
                    &subject,
                    "native comparator setting requires a non-empty `source` citation",
                ));
            }
        }
        SettingKind::Matched => {
            if let Some(ref diff) = setting.difference
                && diff.trim().is_empty()
            {
                findings.push(WorkloadFinding::new(
                    WorkloadFindingKind::EmptySettingDifference,
                    &subject,
                    "matched setting `difference`, when present, must be non-empty",
                ));
            }
            if let Some(wl) = target {
                let differs = setting.h_s != wl.h_s
                    || setting.decimation != wl.decimation
                    || setting.iterations != wl.newton_iterations;
                if differs
                    && setting
                        .difference
                        .as_deref()
                        .is_none_or(|s| s.trim().is_empty())
                {
                    findings.push(WorkloadFinding::new(
                        WorkloadFindingKind::MatchedSettingMissingDifference,
                        &subject,
                        format!(
                            "matched setting ({}, {}, {} iters) differs from workload `{}` ({}, {}, {} iters) without a non-empty `difference` note",
                            setting.h_s,
                            setting.decimation,
                            setting.iterations,
                            wl.name,
                            wl.h_s,
                            wl.decimation,
                            wl.newton_iterations
                        ),
                    ));
                }
            }
        }
    }
}

/// Lint a parsed [`WorkloadRegistry`] and additionally require every comparator in
/// [`PARITY_BASELINE_COMPARATORS`] to carry both `matched` and `native` settings for Q and H.
#[must_use]
pub fn lint_parity_baseline(registry: &WorkloadRegistry) -> Vec<WorkloadFinding> {
    let mut findings = lint_registry(registry);
    let present = registry.comparators();
    for &comparator in PARITY_BASELINE_COMPARATORS {
        if !present.contains(comparator) {
            findings.push(WorkloadFinding::new(
                WorkloadFindingKind::MissingParityComparator,
                format!("comparator.{comparator}"),
                format!(
                    "parity-baseline comparator `{comparator}` has no matched/native setting records"
                ),
            ));
        }
    }
    findings
}

/// Verify KA-815 against a parsed [`WorkloadRegistry`]:
/// - Every general registry lint rule passes;
/// - Q has `(contact_cap = 32, actuated = 12)`, intermediate sum `152`, rule `152`,
///   registered `row_cap = 160` with an explicit non-empty `row_cap_override`;
/// - H has `(contact_cap = 64, actuated = 29)`, intermediate sum `314`, rule `320`,
///   registered `row_cap = 320` with no override.
pub fn check_ka815(registry: &WorkloadRegistry) -> Result<Ka815Report, Vec<WorkloadFinding>> {
    let mut findings = lint_registry(registry);

    let (Some(q), Some(h)) = (registry.get("Q"), registry.get("H")) else {
        return Err(findings);
    };

    let q_raw = checked_row_cap_raw(q.contact_cap, q.actuated).unwrap_or(0);
    let q_rule = checked_row_cap(q.contact_cap, q.actuated).unwrap_or(0);
    if q.contact_cap != 32 || q.actuated != 12 || q_raw != 152 || q_rule != 152 || q.row_cap != 160
    {
        findings.push(WorkloadFinding::new(
            WorkloadFindingKind::Ka815Mismatch,
            "workload.Q",
            format!(
                "KA-815 pins Q at C=32, actuated=12, raw=152, rule=152, row_cap=160; found C={}, actuated={}, raw={q_raw}, rule={q_rule}, row_cap={}",
                q.contact_cap, q.actuated, q.row_cap
            ),
        ));
    }
    match &q.row_cap_override {
        Some(ov) if ov.rule == 152 && !ov.reason.trim().is_empty() => {}
        _ => findings.push(WorkloadFinding::new(
            WorkloadFindingKind::Ka815Mismatch,
            "workload.Q",
            "KA-815 requires Q to record row_cap_override with rule = 152 and non-empty reason",
        )),
    }

    let h_raw = checked_row_cap_raw(h.contact_cap, h.actuated).unwrap_or(0);
    let h_rule = checked_row_cap(h.contact_cap, h.actuated).unwrap_or(0);
    if h.contact_cap != 64
        || h.actuated != 29
        || h_raw != 314
        || h_rule != 320
        || h.row_cap != 320
        || h.row_cap_override.is_some()
    {
        findings.push(WorkloadFinding::new(
            WorkloadFindingKind::Ka815Mismatch,
            "workload.H",
            format!(
                "KA-815 pins H at C=64, actuated=29, raw=314, rule=320, row_cap=320 (no override); found C={}, actuated={}, raw={h_raw}, rule={h_rule}, row_cap={}, override={:?}",
                h.contact_cap, h.actuated, h.row_cap, h.row_cap_override
            ),
        ));
    }

    if !findings.is_empty() {
        return Err(findings);
    }

    let q_id = q.id().map_err(|err| {
        vec![WorkloadFinding::new(
            WorkloadFindingKind::Ka815Mismatch,
            "workload.Q",
            format!("failed to compute WorkloadId: {err}"),
        )]
    })?;
    let h_id = h.id().map_err(|err| {
        vec![WorkloadFinding::new(
            WorkloadFindingKind::Ka815Mismatch,
            "workload.H",
            format!("failed to compute WorkloadId: {err}"),
        )]
    })?;

    Ok(Ka815Report {
        q: Ka815Row {
            name: q.name.clone(),
            id: q_id,
            contact_cap: q.contact_cap,
            actuated: q.actuated,
            raw_sum: q_raw,
            rule_cap: q_rule,
            registered_cap: q.row_cap,
            override_reason: q.row_cap_override.as_ref().map(|ov| ov.reason.clone()),
        },
        h: Ka815Row {
            name: h.name.clone(),
            id: h_id,
            contact_cap: h.contact_cap,
            actuated: h.actuated,
            raw_sum: h_raw,
            rule_cap: h_rule,
            registered_cap: h.row_cap,
            override_reason: None,
        },
    })
}

/// Parse UTF-8 bytes exclusively via [`frob_core::perf::workload::parse_bytes`] and run
/// [`lint_registry`].
pub fn lint_bytes(bytes: &[u8]) -> Result<WorkloadRegistry, WorkloadLintError> {
    let reg = frob_core::perf::workload::parse_bytes(bytes)?;
    let findings = lint_registry(&reg);
    if !findings.is_empty() {
        return Err(WorkloadLintError::Findings(findings));
    }
    Ok(reg)
}

/// Load a workload registry file exclusively via [`frob_core::perf::workload::load`] and run
/// [`lint_parity_baseline`].
pub fn load_and_lint(path: &Path) -> Result<WorkloadRegistry, WorkloadLintError> {
    let reg = frob_core::perf::workload::load(path)?;
    let findings = lint_parity_baseline(&reg);
    if !findings.is_empty() {
        return Err(WorkloadLintError::Findings(findings));
    }
    Ok(reg)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use frob_core::perf::workload::{load, parse_str, row_cap, row_cap_raw};

    use super::*;

    fn committed_workloads_path() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../registries/workloads.toml")
    }

    fn has_kind(findings: &[WorkloadFinding], kind: WorkloadFindingKind) -> bool {
        findings.iter().any(|f| f.kind == kind)
    }

    fn minimal_valid_toml() -> &'static str {
        r#"schema = "frob.workload/1"

[workload.Q]
robot = "ref-quadruped-12"
actuated = 12
dof = 18
bodies = 13
foot_samples = 16
contact_cap = 32
row_cap = 160
row_cap_override = { rule = 152, reason = "raised to the next multiple of the execution width 32 (reference-workload decision; OPEN-SPARK-3)" }
h_s = "1/500"
newton_iterations = 4
warm_start = true
decimation = "10/1"
worlds = 8192
grounds = ["flat", "rough"]
status = "default"

[workload.H]
robot = "ref-humanoid-29"
actuated = 29
dof = 35
bodies = 30
contact_cap = 64
row_cap = 320
h_s = "1/500"
newton_iterations = 4
warm_start = true
decimation = "10/1"
worlds = 8192
grounds = ["flat", "heightfield"]
status = "default"
"#
    }

    #[test]
    fn row_capacity_known_answer() {
        let reg = load(&committed_workloads_path()).expect("load registries/workloads.toml");
        let report = check_ka815(&reg).expect("committed registry must satisfy KA-815");

        // KA-815 acceptance criterion: print intermediate sums 152 and 314.
        let summary = report.format_summary();
        println!("{summary}");
        assert!(summary.contains("152"), "summary must include Q raw sum 152");
        assert!(summary.contains("314"), "summary must include H raw sum 314");

        // Exact Q and H values.
        assert_eq!(row_cap_raw(32, 12), 152);
        assert_eq!(row_cap(32, 12), 152);
        assert_eq!(report.q.raw_sum, 152);
        assert_eq!(report.q.rule_cap, 152);
        assert_eq!(report.q.registered_cap, 160);
        assert!(report.q.override_reason.is_some());

        assert_eq!(row_cap_raw(64, 29), 314);
        assert_eq!(row_cap(64, 29), 320);
        assert_eq!(report.h.raw_sum, 314);
        assert_eq!(report.h.rule_cap, 320);
        assert_eq!(report.h.registered_cap, 320);
        assert!(report.h.override_reason.is_none());

        // Negative control 1: H with row_cap = 314 (rounding removed) must fail the rule check.
        let h_unrounded_toml = minimal_valid_toml().replace("row_cap = 320", "row_cap = 314");
        let h_unrounded = parse_str(&h_unrounded_toml).expect("parse planted H=314");
        let findings = lint_registry(&h_unrounded);
        assert!(
            has_kind(&findings, WorkloadFindingKind::RowCapRuleMismatch),
            "H with row_cap = 314 must fail rule check, got: {findings:?}"
        );

        // Negative control 2: Q with its raise removed (row_cap = 152, no override) satisfies
        // the general rule check and must fail KA-815, which pins the registered 160.
        let q_unraised_toml = minimal_valid_toml()
            .replace("row_cap = 160", "row_cap = 152")
            .replace(
                "row_cap_override = { rule = 152, reason = \"raised to the next multiple of the execution width 32 (reference-workload decision; OPEN-SPARK-3)\" }\n",
                "",
            );
        let q_unraised = parse_str(&q_unraised_toml).expect("parse planted Q=152");
        assert!(
            lint_registry(&q_unraised).is_empty(),
            "Q with row_cap = 152 and no override must satisfy the general rule"
        );
        let ka_err = check_ka815(&q_unraised)
            .expect_err("Q with row_cap = 152 must fail KA-815");
        assert!(
            has_kind(&ka_err, WorkloadFindingKind::Ka815Mismatch),
            "expected Ka815Mismatch, got: {ka_err:?}"
        );
    }

    #[test]
    fn row_capacity_per_row_delta() {
        for &(c, a) in &[(16, 4), (32, 12), (32, 16), (32, 18), (64, 4), (64, 29)] {
            assert_eq!(
                row_cap_raw(c + 1, a) - row_cap_raw(c, a),
                4,
                "raising contact_cap by 1 from ({c}, {a}) must add exactly 4 rows"
            );
            assert_eq!(
                row_cap_raw(c, a + 1) - row_cap_raw(c, a),
                2,
                "raising actuated by 1 from ({c}, {a}) must add exactly 2 rows"
            );
        }
    }

    #[test]
    fn row_capacity_rule_is_multiple_of_8() {
        // All six rows of the dynamics chapter's capacity table (§16.6, N36-02).
        let table: [(&str, u32, u32, u32, u32, Option<u32>); 6] = [
            ("quadruped", 32, 12, 152, 152, None),
            ("hexapod", 32, 18, 164, 168, Some(192)),
            ("humanoid", 64, 29, 314, 320, None),
            ("wheeled-base", 16, 4, 72, 72, Some(96)),
            ("wheel-leg-quadruped", 32, 16, 160, 160, None),
            ("tracked-base", 64, 4, 264, 264, Some(288)),
        ];
        for (name, c, a, expected_raw, expected_rule, granule32_wrong) in table {
            assert_eq!(row_cap_raw(c, a), expected_raw, "{name}: raw sum");
            let got = row_cap(c, a);
            assert_eq!(got, expected_rule, "{name}: multiple-of-8 rule");
            assert_eq!(got % 8, 0, "{name}: must be a multiple of 8");
            if let Some(wrong_32) = granule32_wrong {
                assert_ne!(
                    got, wrong_32,
                    "{name}: rule must not use granule-of-32 ({wrong_32})"
                );
            }
        }
    }

    #[test]
    fn dof_is_six_plus_actuated() {
        let reg = load(&committed_workloads_path()).expect("load committed registry");
        let q = reg.get("Q").expect("Q");
        let h = reg.get("H").expect("H");
        assert_eq!(q.dof, 18);
        assert_eq!(q.dof, 6 + q.actuated);
        assert_eq!(h.dof, 35);
        assert_eq!(h.dof, 6 + h.actuated);

        // Negative control: planted dof != 6 + actuated fails lint.
        let bad_toml = minimal_valid_toml().replace("dof = 18", "dof = 19");
        let bad = parse_str(&bad_toml).expect("parse bad dof");
        let findings = lint_registry(&bad);
        assert!(
            has_kind(&findings, WorkloadFindingKind::DofMismatch),
            "expected DofMismatch, got: {findings:?}"
        );
    }

    #[test]
    fn control_period_is_20ms() {
        let reg = load(&committed_workloads_path()).expect("load committed registry");
        for name in ["Q", "H"] {
            let wl = reg.get(name).expect("reference workload");
            assert!(wl.decimation.is_integer(), "{name}: integer decimation");
            assert_eq!(
                wl.control_period(),
                Some(CONTROL_PERIOD_20MS),
                "{name}: 1/500 s * 10 = 1/50 s (20 ms)"
            );
        }

        // Negative control 1: wrong period (1/250 * 10 = 1/25 s = 40 ms).
        let bad_period_toml = minimal_valid_toml().replacen("h_s = \"1/500\"", "h_s = \"1/250\"", 1);
        let bad_period = parse_str(&bad_period_toml).expect("parse bad period");
        let findings = lint_registry(&bad_period);
        assert!(
            has_kind(&findings, WorkloadFindingKind::ControlPeriodMismatch),
            "expected ControlPeriodMismatch, got: {findings:?}"
        );

        // Negative control 2: exact 20 ms product (3/1000 * 20/3 = 1/50) with non-integer decimation.
        let non_int_toml = minimal_valid_toml()
            .replacen("h_s = \"1/500\"", "h_s = \"3/1000\"", 1)
            .replacen("decimation = \"10/1\"", "decimation = \"20/3\"", 1);
        let non_int = parse_str(&non_int_toml).expect("parse non-integer decimation");
        let findings = lint_registry(&non_int);
        assert!(
            has_kind(&findings, WorkloadFindingKind::NonIntegerDecimation),
            "expected NonIntegerDecimation, got: {findings:?}"
        );
        assert!(
            !has_kind(&findings, WorkloadFindingKind::ControlPeriodMismatch),
            "3/1000 * 20/3 equals 1/50 s exactly"
        );
    }

    #[test]
    fn override_requires_reason() {
        // Committed Q record is accepted.
        let reg = load(&committed_workloads_path()).expect("load committed registry");
        assert!(
            lint_workload(reg.get("Q").expect("Q")).is_empty(),
            "committed Q must pass lint"
        );

        // 1. Differing row_cap with no override -> rejected.
        let no_ov = minimal_valid_toml().replace(
            "row_cap_override = { rule = 152, reason = \"raised to the next multiple of the execution width 32 (reference-workload decision; OPEN-SPARK-3)\" }\n",
            "",
        );
        let findings = lint_registry(&parse_str(&no_ov).unwrap());
        assert!(
            has_kind(&findings, WorkloadFindingKind::RowCapRuleMismatch),
            "expected RowCapRuleMismatch, got: {findings:?}"
        );

        // 2. Empty or whitespace-only reason -> rejected.
        for bad_reason in ["", "   "] {
            let empty_reason = minimal_valid_toml().replace(
                "raised to the next multiple of the execution width 32 (reference-workload decision; OPEN-SPARK-3)",
                bad_reason,
            );
            let findings = lint_registry(&parse_str(&empty_reason).unwrap());
            assert!(
                has_kind(&findings, WorkloadFindingKind::EmptyOverrideReason),
                "expected EmptyOverrideReason for `{bad_reason}`, got: {findings:?}"
            );
        }

        // 3. Override whose rule != row_cap(contact_cap, actuated) -> rejected.
        let wrong_rule = minimal_valid_toml().replace("rule = 152", "rule = 144");
        let findings = lint_registry(&parse_str(&wrong_rule).unwrap());
        assert!(
            has_kind(&findings, WorkloadFindingKind::OverrideRuleMismatch),
            "expected OverrideRuleMismatch, got: {findings:?}"
        );
    }

    #[test]
    fn frozen_requires_adr() {
        // Frozen without adr -> rejected.
        let frozen_no_adr = minimal_valid_toml().replacen("status = \"default\"", "status = \"frozen\"", 1);
        let findings = lint_registry(&parse_str(&frozen_no_adr).unwrap());
        assert!(
            has_kind(&findings, WorkloadFindingKind::FrozenMissingAdr),
            "expected FrozenMissingAdr, got: {findings:?}"
        );

        // Frozen with empty/blank adr -> rejected.
        let frozen_blank_adr = minimal_valid_toml().replacen(
            "status = \"default\"",
            "status = \"frozen\"\nadr = \"   \"",
            1,
        );
        let findings = lint_registry(&parse_str(&frozen_blank_adr).unwrap());
        assert!(
            has_kind(&findings, WorkloadFindingKind::FrozenMissingAdr),
            "expected FrozenMissingAdr for blank adr, got: {findings:?}"
        );

        // Frozen with valid ADR reference -> accepted.
        let frozen_ok = minimal_valid_toml().replacen(
            "status = \"default\"",
            "status = \"frozen\"\nadr = \"ADR-SPARK-03\"",
            1,
        );
        let findings = lint_registry(&parse_str(&frozen_ok).unwrap());
        assert!(
            findings.is_empty(),
            "frozen record with ADR must pass lint, got: {findings:?}"
        );
    }

    #[test]
    fn native_settings_have_source() {
        let reg = load_and_lint(&committed_workloads_path())
            .expect("committed registries/workloads.toml must pass full parity-baseline lint");

        // Every parity-baseline comparator has matched & native for both Q and H, each with a
        // content hash, and every native setting carries a non-empty source.
        let mut setting_ids = BTreeSet::new();
        for &comp in PARITY_BASELINE_COMPARATORS {
            for &wl in REQUIRED_REFERENCE_WORKLOADS {
                let matched = reg
                    .setting(comp, wl, SettingKind::Matched)
                    .unwrap_or_else(|| panic!("missing matched setting for {comp}/{wl}"));
                let native = reg
                    .setting(comp, wl, SettingKind::Native)
                    .unwrap_or_else(|| panic!("missing native setting for {comp}/{wl}"));
                assert!(
                    native
                        .source
                        .as_deref()
                        .is_some_and(|s| !s.trim().is_empty()),
                    "native setting {comp}/{wl} must have a non-empty source"
                );
                assert!(
                    setting_ids.insert(matched.id().expect("matched SettingId")),
                    "duplicate SettingId for {comp}/{wl} matched"
                );
                assert!(
                    setting_ids.insert(native.id().expect("native SettingId")),
                    "duplicate SettingId for {comp}/{wl} native"
                );
            }
        }

        // Negative control 1: native setting with missing source -> rejected.
        let missing_src = format!(
            "{}\n[[setting]]\ncomparator = \"demo\"\nworkload = \"Q\"\nkind = \"matched\"\nh_s = \"1/500\"\ndecimation = \"10/1\"\niterations = 4\n\n[[setting]]\ncomparator = \"demo\"\nworkload = \"Q\"\nkind = \"native\"\nh_s = \"1/250\"\ndecimation = \"5/1\"\niterations = 4\n\n[[setting]]\ncomparator = \"demo\"\nworkload = \"H\"\nkind = \"matched\"\nh_s = \"1/500\"\ndecimation = \"10/1\"\niterations = 4\n\n[[setting]]\ncomparator = \"demo\"\nworkload = \"H\"\nkind = \"native\"\nh_s = \"1/500\"\ndecimation = \"10/1\"\niterations = 4\nsource = \"Appendix H\"\n",
            minimal_valid_toml()
        );
        let findings = lint_registry(&parse_str(&missing_src).unwrap());
        assert!(
            has_kind(&findings, WorkloadFindingKind::NativeSettingMissingSource),
            "expected NativeSettingMissingSource, got: {findings:?}"
        );

        // Negative control 2: native setting with empty source -> rejected.
        let empty_src = missing_src.replace(
            "source = \"Appendix H\"",
            "source = \"\"",
        );
        let findings = lint_registry(&parse_str(&empty_src).unwrap());
        assert!(
            has_kind(&findings, WorkloadFindingKind::NativeSettingMissingSource),
            "expected NativeSettingMissingSource for empty string, got: {findings:?}"
        );

        // Negative control 3: matched setting differing from workload step without difference -> rejected.
        let bad_matched = format!(
            "{}\n[[setting]]\ncomparator = \"demo\"\nworkload = \"Q\"\nkind = \"matched\"\nh_s = \"1/250\"\ndecimation = \"5/1\"\niterations = 4\n\n[[setting]]\ncomparator = \"demo\"\nworkload = \"Q\"\nkind = \"native\"\nh_s = \"1/250\"\ndecimation = \"5/1\"\niterations = 4\nsource = \"Appendix H\"\n\n[[setting]]\ncomparator = \"demo\"\nworkload = \"H\"\nkind = \"matched\"\nh_s = \"1/500\"\ndecimation = \"10/1\"\niterations = 4\n\n[[setting]]\ncomparator = \"demo\"\nworkload = \"H\"\nkind = \"native\"\nh_s = \"1/500\"\ndecimation = \"10/1\"\niterations = 4\nsource = \"Appendix H\"\n",
            minimal_valid_toml()
        );
        let findings = lint_registry(&parse_str(&bad_matched).unwrap());
        assert!(
            has_kind(&findings, WorkloadFindingKind::MatchedSettingMissingDifference),
            "expected MatchedSettingMissingDifference, got: {findings:?}"
        );
    }
}
