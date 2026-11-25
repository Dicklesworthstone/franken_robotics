#![deny(unsafe_code)]

//! Funding rule (`fundable_mechanism`, `fundable`, `fundable_bead`, `fundable_all`),
//! Moonshot cap enforcement (`<= 4` running tier-`M` pilots per milestone),
//! live plan-time ledger status reads (`rerun_status`), `fallback.also` resolution,
//! and `GATE-GAUNT-08` scan (`N38a-09`, §38.5).
//!
//! Incorporates Round-4 Review Notes 3 and 4:
//! - Review Note 3 (`slice S-02`): a bead not named in any record's `gated_beads`
//!   is answered only by the records of its work package that list no `gated_beads`
//!   at all (and never blocked by a record that lists the bead in `fallback.beads`),
//!   so a held falsifier on a gated mechanism never blocks its fallback work or
//!   unlisted sibling beads in a multi-mechanism package.
//! - Review Note 4 (`slice S-14`): a `tier = "S"` record with `fallback.kill_gate = "none"`
//!   funds on its pilot result alone without blocking on `kill-gate-to-register`.
//!
//! Determinism class D0; prints nothing (rule L-7); zero `unsafe`.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::fs;
use std::path::Path;

use frob_core::registry::toml::{self, Fields, Value};

use crate::e2e::json::{self, JsonValue};
use crate::gates_v0::GateRegistry;
use crate::pilots::schema::{
    MOONSHOT_RUNNING_CAP, PILOTS_PATH, PilotLoadError, PilotRecord, PilotRegister, PilotResult,
    PilotSite, PilotStatus, PilotTier,
};
use crate::pilots::validate::effective_status;
use crate::reasons::HarnessReason;

/// All eleven funding-rule reasons registered in `registry/reasons.toml` (`N38-REF`).
pub const PILOT_FUNDING_REASONS: &[HarnessReason] = &[
    HarnessReason::Missing,
    HarnessReason::Unregistered,
    HarnessReason::Void,
    HarnessReason::Registered,
    HarnessReason::Running,
    HarnessReason::FalsifierHolds,
    HarnessReason::Inconclusive,
    HarnessReason::KillGateToRegister,
    HarnessReason::MoonshotCap,
    HarnessReason::PlanTimeDiffers,
    HarnessReason::PlanTimeNotRun,
];

/// Derived re-run status of a plan-time ledger record (`PILOT-PT-*`, `N38a-09` / `N38a-10`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RerunStatus {
    Reproduced,
    Differs,
    NotRun,
    Void,
}

impl RerunStatus {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Reproduced => "reproduced",
            Self::Differs => "differs",
            Self::NotRun => "not-run",
            Self::Void => "void",
        }
    }
}

impl fmt::Display for RerunStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Pilots are evidence for funding, never for a product gate (`N38a-09` §38.5).
///
/// In particular, a `development-oracle` result funds or kills a work package
/// provisionally and supports no gate of `registries/gates.toml`.
#[must_use]
pub const fn pilot_result_supports_gate(_result: &PilotResult) -> bool {
    false
}

/// Funding verdict for one mechanism or pilot record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MechanismVerdict {
    Fundable {
        /// True when funded from a `development-oracle` result pending CPU referee re-run.
        provisional: bool,
        /// True when queued for a re-run on the CPU referee.
        queued_for_referee: bool,
        /// Always `false`: pilots never support a product gate.
        supports_gate: bool,
        /// Optional explanatory note (e.g. `"pilot-bead"`, `"fallback-bead"`).
        note: Option<String>,
    },
    Blocked {
        reason: HarnessReason,
        fallback: Option<String>,
        kill_gate: Option<String>,
        detail: String,
    },
}

impl MechanismVerdict {
    #[must_use]
    pub const fn is_fundable(&self) -> bool {
        matches!(self, Self::Fundable { .. })
    }

    #[must_use]
    pub const fn reason(&self) -> Option<HarnessReason> {
        match self {
            Self::Fundable { .. } => None,
            Self::Blocked { reason, .. } => Some(*reason),
        }
    }

    #[must_use]
    pub const fn is_provisional(&self) -> bool {
        matches!(
            self,
            Self::Fundable {
                provisional: true,
                ..
            }
        )
    }

    #[must_use]
    pub const fn is_queued_for_referee(&self) -> bool {
        matches!(
            self,
            Self::Fundable {
                queued_for_referee: true,
                ..
            }
        )
    }
}

/// Per-mechanism funding entry inside [`PackageFunding`] and [`BeadFunding`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MechanismFunding {
    pub mechanism: String,
    pub pilot_ids: Vec<String>,
    pub verdict: MechanismVerdict,
}

/// Aggregate funding status for a work package.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PackageFundingStatus {
    Fundable,
    Partial,
    Blocked,
}

impl PackageFundingStatus {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Fundable => "Fundable",
            Self::Partial => "Partial",
            Self::Blocked => "Blocked",
        }
    }
}

impl fmt::Display for PackageFundingStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Package-level funding answer returned by `pilots::fundable(work_package)`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageFunding {
    pub work_package: String,
    pub status: PackageFundingStatus,
    pub mechanisms: Vec<MechanismFunding>,
}

/// Bead-level funding answer returned by `pilots::fundable_bead(key)`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BeadFunding {
    pub bead: String,
    pub work_package: Option<String>,
    pub fundable: bool,
    pub provisional: bool,
    pub queued_for_referee: bool,
    pub note: Option<String>,
    pub reason: Option<HarnessReason>,
    pub fallback: Option<String>,
    pub kill_gate: Option<String>,
    pub detail: String,
    pub mechanisms: Vec<MechanismFunding>,
}

/// Resolution of a record's unconditional `fallback.ships` and conditional `fallback.also`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FallbackResolution {
    pub unconditional: String,
    pub kill_gate: String,
    pub conditional_active: Vec<(String, String)>,
    pub conditional_inactive: Vec<(String, String, HarnessReason)>,
}

/// Counts of pilot records per effective [`PilotStatus`].
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct StatusCounts {
    pub registered: usize,
    pub running: usize,
    pub reproduced: usize,
    pub differs: usize,
    pub falsifier_holds: usize,
    pub falsifier_rejected: usize,
    pub inconclusive: usize,
    pub withdrawn: usize,
    pub void: usize,
}

impl StatusCounts {
    #[must_use]
    pub fn from_register(
        reg: &PilotRegister,
        root: Option<&Path>,
        previous: Option<&PilotRegister>,
    ) -> Self {
        let mut out = Self::default();
        for rec in &reg.pilots {
            match effective_status(rec, root, previous) {
                PilotStatus::Registered => out.registered += 1,
                PilotStatus::Running => out.running += 1,
                PilotStatus::Reproduced => out.reproduced += 1,
                PilotStatus::Differs => out.differs += 1,
                PilotStatus::FalsifierHolds => out.falsifier_holds += 1,
                PilotStatus::FalsifierRejected => out.falsifier_rejected += 1,
                PilotStatus::Inconclusive => out.inconclusive += 1,
                PilotStatus::Withdrawn => out.withdrawn += 1,
                PilotStatus::Void => out.void += 1,
            }
        }
        out
    }

    #[must_use]
    pub const fn total(&self) -> usize {
        self.registered
            + self.running
            + self.reproduced
            + self.differs
            + self.falsifier_holds
            + self.falsifier_rejected
            + self.inconclusive
            + self.withdrawn
            + self.void
    }
}

/// Summary returned by `fundable --all`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AllPackagesFunding {
    pub packages: Vec<PackageFunding>,
    pub fundable_count: usize,
    pub partial_count: usize,
    pub refused_count: usize,
    pub status_counts: StatusCounts,
    pub moonshots_running_by_milestone: BTreeMap<String, usize>,
}

/// Violation reported by a `GATE-GAUNT-08` scan of funded upper-tier work packages.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateGaunt08Violation {
    pub work_package: String,
    pub mechanism: String,
    pub reason: HarnessReason,
    pub detail: String,
}

/// Result of scanning every funded Frontier/Moonshot work package for `GATE-GAUNT-08`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateGaunt08Scan {
    pub packages_scanned: usize,
    pub violations: Vec<GateGaunt08Violation>,
}

/// Metadata about work packages and bead-to-package ownership read from the workspace.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct WorkspacePackages {
    /// Upper-tier (`[F]` or `[M]`) work packages: `WP-nn` -> info.
    pub upper_tier_packages: BTreeMap<String, PackageMeta>,
    /// Milestone of every known work package (`WP-nn` -> `"M0"`..`"M8"`).
    pub package_milestones: BTreeMap<String, String>,
    /// Map from bead key (e.g. `"BD-333"`) to its parent work package (`"WP-64"`).
    pub bead_to_package: BTreeMap<String, String>,
}

/// Metadata for one upper-tier work package.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageMeta {
    pub work_package: String,
    pub title: String,
    pub tiers: Vec<PilotTier>,
    pub milestone: String,
    /// Declared upper-tier mechanism names for this work package (if enumerated).
    pub mechanisms: Vec<String>,
}

impl WorkspacePackages {
    /// Discover work-package tier tags, milestones, and bead-to-package parents
    /// under `root` from whichever of these files exist:
    /// - `.beads/issues.jsonl` (tracker export with `WP-nn` labels and `parent-child` edges)
    /// - `docs/delivery/work-packages.plan.tsv` (package milestones)
    /// - `crates/frob-conformance/registry/work_packages.toml` (optional scratch manifest)
    /// - `registries/capabilities.toml`, `registries/kernels.toml`, `registries/verbs.toml`
    #[must_use]
    pub fn load(root: &Path) -> Self {
        let mut ws = Self::default();
        ws.load_plan_tsv(root);
        ws.load_beads_jsonl(root);
        ws.load_scratch_manifest(root);
        ws.load_upper_tier_registry_mechanisms(root);
        ws
    }

    /// Add or update an upper-tier work package (used by tests and scratch workspaces).
    pub fn add_upper_tier_package(
        &mut self,
        wp: impl Into<String>,
        tier: PilotTier,
        milestone: impl Into<String>,
        mechanisms: Vec<String>,
    ) {
        let wp = wp.into();
        let ms = milestone.into();
        self.package_milestones.insert(wp.clone(), ms.clone());
        let entry = self
            .upper_tier_packages
            .entry(wp.clone())
            .or_insert_with(|| PackageMeta {
                work_package: wp.clone(),
                title: wp,
                tiers: Vec::new(),
                milestone: ms,
                mechanisms: Vec::new(),
            });
        if !entry.tiers.contains(&tier) {
            entry.tiers.push(tier);
            entry.tiers.sort();
        }
        for m in mechanisms {
            if !entry.mechanisms.contains(&m) {
                entry.mechanisms.push(m);
            }
        }
    }

    /// Register a bead key's parent work package.
    pub fn map_bead(&mut self, bead: impl Into<String>, work_package: impl Into<String>) {
        self.bead_to_package
            .insert(bead.into(), work_package.into());
    }

    fn load_plan_tsv(&mut self, root: &Path) {
        let path = root.join("docs/delivery/work-packages.plan.tsv");
        let Ok(text) = fs::read_to_string(&path) else {
            return;
        };
        for (idx, line) in text.lines().enumerate() {
            if idx == 0 || line.trim().is_empty() {
                continue;
            }
            let mut cols = line.split('\t');
            let Some(wp) = cols.next().map(str::trim) else {
                continue;
            };
            let Some(ms) = cols.next().map(str::trim) else {
                continue;
            };
            if wp.starts_with("WP-") && !ms.is_empty() && ms != "none" {
                self.package_milestones
                    .entry(wp.to_owned())
                    .or_insert_with(|| ms.to_owned());
            }
        }
    }

    fn load_beads_jsonl(&mut self, root: &Path) {
        let path = root.join(".beads/issues.jsonl");
        let Ok(text) = fs::read_to_string(&path) else {
            return;
        };

        // Pass 1: collect WP issue ids -> WP external_ref and upper-tier tags.
        let mut wp_id_to_ref: BTreeMap<String, String> = BTreeMap::new();
        let mut bead_parent_ids: Vec<(String, String)> = Vec::new();

        for line in text.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            let Ok(val) = json::parse(trimmed) else {
                continue;
            };
            let ext_ref = val.get("external_ref").and_then(JsonValue::as_str);
            let id = val.get("id").and_then(JsonValue::as_str);
            let title = val
                .get("title")
                .and_then(JsonValue::as_str)
                .unwrap_or_default();

            let labels: Vec<&str> = val
                .get("labels")
                .and_then(JsonValue::as_array)
                .map(|arr| arr.iter().filter_map(JsonValue::as_str).collect())
                .unwrap_or_default();

            if let Some(wp_ref) = ext_ref
                && wp_ref.starts_with("WP-")
            {
                if let Some(issue_id) = id {
                    wp_id_to_ref.insert(issue_id.to_owned(), wp_ref.to_owned());
                }
                let milestone = labels
                    .iter()
                    .find_map(|l| parse_milestone_label(l))
                    .or_else(|| self.package_milestones.get(wp_ref).cloned())
                    .unwrap_or_else(|| "M0".to_owned());
                self.package_milestones
                    .insert(wp_ref.to_owned(), milestone.clone());

                let mut tiers = Vec::new();
                if labels.iter().any(|l| l.eq_ignore_ascii_case("frontier")) {
                    tiers.push(PilotTier::F);
                }
                if labels.iter().any(|l| l.eq_ignore_ascii_case("moonshot")) {
                    tiers.push(PilotTier::M);
                }
                if !tiers.is_empty() {
                    self.upper_tier_packages.insert(
                        wp_ref.to_owned(),
                        PackageMeta {
                            work_package: wp_ref.to_owned(),
                            title: title.to_owned(),
                            tiers,
                            milestone,
                            mechanisms: Vec::new(),
                        },
                    );
                }
            } else if let Some(bead_ref) = ext_ref
                && let Some(deps) = val.get("dependencies").and_then(JsonValue::as_array)
            {
                for dep in deps {
                    if dep.get("type").and_then(JsonValue::as_str) == Some("parent-child")
                        && let Some(parent_id) =
                            dep.get("depends_on_id").and_then(JsonValue::as_str)
                    {
                        bead_parent_ids.push((bead_ref.to_owned(), parent_id.to_owned()));
                    }
                }
            }
        }

        // Pass 2: resolve parent issue ids to WP-nn external refs.
        for (bead_ref, parent_id) in bead_parent_ids {
            if let Some(wp_ref) = wp_id_to_ref.get(&parent_id) {
                self.bead_to_package.insert(bead_ref, wp_ref.clone());
            }
        }
    }

    fn load_scratch_manifest(&mut self, root: &Path) {
        let path = root.join("crates/frob-conformance/registry/work_packages.toml");
        let Ok(bytes) = fs::read(&path) else {
            return;
        };
        let Ok(tbl) = toml::parse(&bytes) else {
            return;
        };
        let mut top = Fields::new(tbl, "work_packages");
        let _ = top.opt_str("schema");
        let pkgs = top.opt_array("package").ok().flatten().unwrap_or_default();
        let _ = top.finish();

        for sp in pkgs {
            let Value::Table(ptbl) = sp.value else {
                continue;
            };
            let mut pf = Fields::new(ptbl, "package");
            let Ok(wp) = pf.require_str("work_package") else {
                continue;
            };
            let milestone = pf
                .opt_str("milestone")
                .ok()
                .flatten()
                .unwrap_or_else(|| "M0".to_owned());
            let tiers: Vec<PilotTier> = pf
                .opt_array("tier")
                .ok()
                .flatten()
                .unwrap_or_default()
                .into_iter()
                .filter_map(|s| s.value.as_str().and_then(PilotTier::parse_external))
                .collect();
            let mechanisms: Vec<String> = pf
                .opt_array("mechanisms")
                .ok()
                .flatten()
                .unwrap_or_default()
                .into_iter()
                .filter_map(|s| s.value.as_str().map(str::to_owned))
                .collect();
            let beads: Vec<String> = pf
                .opt_array("beads")
                .ok()
                .flatten()
                .unwrap_or_default()
                .into_iter()
                .filter_map(|s| s.value.as_str().map(str::to_owned))
                .collect();
            let _ = pf.finish();

            self.package_milestones
                .insert(wp.clone(), milestone.clone());
            for b in beads {
                self.bead_to_package.insert(b, wp.clone());
            }
            for t in tiers {
                if t.is_upper() {
                    self.add_upper_tier_package(
                        wp.clone(),
                        t,
                        milestone.clone(),
                        mechanisms.clone(),
                    );
                }
            }
        }
    }

    fn load_upper_tier_registry_mechanisms(&mut self, root: &Path) {
        for (rel, array_key) in [
            ("registries/capabilities.toml", "capability"),
            ("registries/kernels.toml", "kernel"),
            ("registries/verbs.toml", "verb"),
        ] {
            let path = root.join(rel);
            let Ok(bytes) = fs::read(&path) else {
                continue;
            };
            let Ok(tbl) = toml::parse(&bytes) else {
                continue;
            };
            let Some(items) = tbl.get(array_key).and_then(Value::as_array) else {
                continue;
            };
            for sp in items {
                let Some(row) = sp.value.as_table() else {
                    continue;
                };
                let tier = row
                    .get("tier")
                    .and_then(Value::as_str)
                    .and_then(PilotTier::parse_external);
                let Some(t) = tier.filter(|t| t.is_upper()) else {
                    continue;
                };
                let Some(wp) = row.get("work_package").and_then(Value::as_str) else {
                    continue;
                };
                let mech = row
                    .get("mechanism")
                    .or_else(|| row.get("id"))
                    .or_else(|| row.get("name"))
                    .and_then(Value::as_str)
                    .unwrap_or(wp);
                let ms = row
                    .get("milestone")
                    .and_then(Value::as_str)
                    .map(str::to_owned)
                    .or_else(|| self.package_milestones.get(wp).cloned())
                    .unwrap_or_else(|| "M0".to_owned());
                self.add_upper_tier_package(wp.to_owned(), t, ms, vec![mech.to_owned()]);
            }
        }
    }

    /// Resolve a record's milestone for Moonshot cap accounting.
    #[must_use]
    pub fn milestone_for_record(&self, rec: &PilotRecord) -> String {
        if let Some(ref m) = rec.milestone
            && !m.trim().is_empty()
        {
            return m.trim().to_owned();
        }
        self.package_milestones
            .get(&rec.work_package)
            .cloned()
            .unwrap_or_else(|| "M0".to_owned())
    }
}

fn parse_milestone_label(label: &str) -> Option<String> {
    let b = label.as_bytes();
    if b.len() == 2 && (b[0] == b'm' || b[0] == b'M') && b[1].is_ascii_digit() {
        Some(format!("M{}", b[1] as char))
    } else {
        None
    }
}

/// Live read of a plan-time ledger record's re-run status (`N38a-09` / `N38a-10`).
#[must_use]
pub fn rerun_status_in(
    reg: &PilotRegister,
    id: &str,
    root: Option<&Path>,
    previous: Option<&PilotRegister>,
) -> Option<RerunStatus> {
    let rec = reg.get(id)?;
    let eff = effective_status(rec, root, previous);
    Some(match eff {
        PilotStatus::Reproduced => RerunStatus::Reproduced,
        PilotStatus::Differs => RerunStatus::Differs,
        PilotStatus::Registered | PilotStatus::Running => RerunStatus::NotRun,
        PilotStatus::Void => RerunStatus::Void,
        PilotStatus::FalsifierHolds => RerunStatus::Differs,
        PilotStatus::FalsifierRejected => RerunStatus::Reproduced,
        PilotStatus::Inconclusive | PilotStatus::Withdrawn => RerunStatus::NotRun,
    })
}

/// Count currently `running` tier-`M` (Moonshot) pilots per milestone.
///
/// Tier-`S` and tier-`F` records are never counted by the Moonshot cap.
#[must_use]
pub fn moonshots_running_by_milestone(
    reg: &PilotRegister,
    root: Option<&Path>,
    previous: Option<&PilotRegister>,
    ws: &WorkspacePackages,
) -> BTreeMap<String, usize> {
    let mut counts: BTreeMap<String, usize> = BTreeMap::new();
    for rec in &reg.pilots {
        if rec.tier == PilotTier::M && effective_status(rec, root, previous) == PilotStatus::Running
        {
            let ms = ws.milestone_for_record(rec);
            *counts.entry(ms).or_insert(0) += 1;
        }
    }
    counts
}

/// Enforce the Moonshot cap (`<= 4` running tier-`M` pilots in one milestone) at a
/// status change (`N38a-09`, doctrine D-6).
///
/// Refuses a fifth `running` tier-`M` pilot in the same milestone with
/// `HarnessReason::MoonshotCap` (`reason=moonshot-cap`). Tier-`S` and tier-`F`
/// transitions are never counted by this cap.
pub fn check_moonshot_status_change(
    reg: &PilotRegister,
    id: &str,
    new_status: PilotStatus,
    milestone_override: Option<&str>,
    root: Option<&Path>,
    previous: Option<&PilotRegister>,
    ws: &WorkspacePackages,
) -> Result<(), PilotLoadError> {
    let rec = reg.get(id).ok_or_else(|| {
        PilotLoadError::with_reason(
            PILOTS_PATH,
            Some(id.to_owned()),
            HarnessReason::Missing,
            format!("pilot record `{id}` not found"),
        )
    })?;

    if rec.tier != PilotTier::M || new_status != PilotStatus::Running {
        return Ok(());
    }

    let target_ms = milestone_override
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_owned)
        .unwrap_or_else(|| ws.milestone_for_record(rec));

    let running_others: Vec<&str> = reg
        .pilots
        .iter()
        .filter(|other| {
            other.id != rec.id
                && other.tier == PilotTier::M
                && effective_status(other, root, previous) == PilotStatus::Running
                && ws.milestone_for_record(other) == target_ms
        })
        .map(|other| other.id.as_str())
        .collect();

    if running_others.len() >= MOONSHOT_RUNNING_CAP {
        return Err(PilotLoadError::with_reason(
            PILOTS_PATH,
            Some(rec.id.clone()),
            HarnessReason::MoonshotCap,
            format!(
                "cannot set Moonshot pilot `{}` to `running` in milestone `{target_ms}`: cap of {MOONSHOT_RUNNING_CAP} already full ({})",
                rec.id,
                running_others.join(", ")
            ),
        ));
    }

    Ok(())
}

/// Compute the set of pilot ids that exceed the per-milestone Moonshot cap
/// (any 5th+ `running` tier-`M` record in the same milestone).
fn over_cap_moonshot_ids(
    reg: &PilotRegister,
    root: Option<&Path>,
    previous: Option<&PilotRegister>,
    ws: &WorkspacePackages,
) -> BTreeSet<String> {
    let mut seen_per_ms: BTreeMap<String, usize> = BTreeMap::new();
    let mut over = BTreeSet::new();
    for rec in &reg.pilots {
        if rec.tier == PilotTier::M && effective_status(rec, root, previous) == PilotStatus::Running
        {
            let ms = ws.milestone_for_record(rec);
            let n = seen_per_ms.entry(ms).or_insert(0);
            *n += 1;
            if *n > MOONSHOT_RUNNING_CAP {
                over.insert(rec.id.clone());
            }
        }
    }
    over
}

/// Evaluate the funding verdict of a single pilot record.
#[must_use]
pub fn evaluate_record(
    reg: &PilotRegister,
    rec: &PilotRecord,
    gates: Option<&GateRegistry>,
    root: Option<&Path>,
    previous: Option<&PilotRegister>,
    ws: &WorkspacePackages,
) -> MechanismVerdict {
    if !rec.is_registered() {
        return MechanismVerdict::Blocked {
            reason: HarnessReason::Unregistered,
            fallback: Some(rec.fallback.ships.clone()),
            kill_gate: Some(rec.fallback.kill_gate.clone()),
            detail: format!("pilot `{}` has no frozen protocol", rec.id),
        };
    }

    let eff = effective_status(rec, root, previous);
    if eff == PilotStatus::Void {
        return MechanismVerdict::Blocked {
            reason: HarnessReason::Void,
            fallback: Some(rec.fallback.ships.clone()),
            kill_gate: Some(rec.fallback.kill_gate.clone()),
            detail: format!(
                "pilot `{}` is void (protocol file or frozen value changed after registration)",
                rec.id
            ),
        };
    }

    // Moonshot cap check for 5th+ running tier-M pilot in one milestone.
    let over_cap = over_cap_moonshot_ids(reg, root, previous, ws);
    if over_cap.contains(&rec.id) {
        return MechanismVerdict::Blocked {
            reason: HarnessReason::MoonshotCap,
            fallback: Some(rec.fallback.ships.clone()),
            kill_gate: Some(rec.fallback.kill_gate.clone()),
            detail: format!(
                "pilot `{}` exceeds Moonshot running cap of {MOONSHOT_RUNNING_CAP} in milestone `{}`",
                rec.id,
                ws.milestone_for_record(rec)
            ),
        };
    }

    // Plan-time linked ledger records (read live; never copied into `rec`).
    if !rec.plan_time_records.is_empty() {
        for pt_id in &rec.plan_time_records {
            match rerun_status_in(reg, pt_id, root, previous) {
                Some(RerunStatus::Reproduced) => {}
                Some(RerunStatus::Differs) => {
                    return MechanismVerdict::Blocked {
                        reason: HarnessReason::PlanTimeDiffers,
                        fallback: Some(rec.fallback.ships.clone()),
                        kill_gate: Some(rec.fallback.kill_gate.clone()),
                        detail: format!(
                            "pilot `{}` blocked because linked plan-time record `{pt_id}` has status `differs`",
                            rec.id
                        ),
                    };
                }
                Some(RerunStatus::Void) => {
                    return MechanismVerdict::Blocked {
                        reason: HarnessReason::Void,
                        fallback: Some(rec.fallback.ships.clone()),
                        kill_gate: Some(rec.fallback.kill_gate.clone()),
                        detail: format!(
                            "pilot `{}` blocked because linked plan-time record `{pt_id}` is `void`",
                            rec.id
                        ),
                    };
                }
                Some(RerunStatus::NotRun) | None => {
                    return MechanismVerdict::Blocked {
                        reason: HarnessReason::PlanTimeNotRun,
                        fallback: Some(rec.fallback.ships.clone()),
                        kill_gate: Some(rec.fallback.kill_gate.clone()),
                        detail: format!(
                            "pilot `{}` blocked because linked plan-time record `{pt_id}` is not yet reproduced",
                            rec.id
                        ),
                    };
                }
            }
        }

        // All linked plan-time records are `reproduced`. If this record has no own
        // to-run part in progress or appended results, check kill gate and fund.
        if rec.results.is_empty()
            && matches!(eff, PilotStatus::Registered | PilotStatus::Reproduced)
        {
            return check_kill_gate_and_fund(rec, gates, false, false);
        }
    }

    // Own result / status evaluation (referee/non-dev-oracle overrides development-oracle).
    if let Some(auth) = rec.authoritative_result() {
        let provisional = auth.site == PilotSite::DevelopmentOracle;
        let queued = provisional;
        match auth.verdict {
            PilotStatus::FalsifierRejected | PilotStatus::Reproduced => {
                check_kill_gate_and_fund(rec, gates, provisional, queued)
            }
            PilotStatus::FalsifierHolds => MechanismVerdict::Blocked {
                reason: HarnessReason::FalsifierHolds,
                fallback: Some(rec.fallback.ships.clone()),
                kill_gate: Some(rec.fallback.kill_gate.clone()),
                detail: format!(
                    "pilot `{}` falsifier holds; fallback ships: `{}` (kill gate: `{}`)",
                    rec.id, rec.fallback.ships, rec.fallback.kill_gate
                ),
            },
            PilotStatus::Inconclusive => {
                let decisive = auth
                    .decisive_note
                    .as_deref()
                    .or_else(|| auth.limits.first().map(String::as_str))
                    .unwrap_or("re-run with decisive power");
                MechanismVerdict::Blocked {
                    reason: HarnessReason::Inconclusive,
                    fallback: Some(rec.fallback.ships.clone()),
                    kill_gate: Some(rec.fallback.kill_gate.clone()),
                    detail: format!(
                        "pilot `{}` result is inconclusive; to decide: {decisive}",
                        rec.id
                    ),
                }
            }
            PilotStatus::Differs => MechanismVerdict::Blocked {
                reason: HarnessReason::PlanTimeDiffers,
                fallback: Some(rec.fallback.ships.clone()),
                kill_gate: Some(rec.fallback.kill_gate.clone()),
                detail: format!(
                    "pilot `{}` re-run differs from recorded plan-time values",
                    rec.id
                ),
            },
            PilotStatus::Registered => MechanismVerdict::Blocked {
                reason: HarnessReason::Registered,
                fallback: Some(rec.fallback.ships.clone()),
                kill_gate: Some(rec.fallback.kill_gate.clone()),
                detail: format!("pilot `{}` has no decisive result yet", rec.id),
            },
            PilotStatus::Running => MechanismVerdict::Blocked {
                reason: HarnessReason::Running,
                fallback: Some(rec.fallback.ships.clone()),
                kill_gate: Some(rec.fallback.kill_gate.clone()),
                detail: format!("pilot `{}` is still running", rec.id),
            },
            PilotStatus::Void | PilotStatus::Withdrawn => MechanismVerdict::Blocked {
                reason: HarnessReason::Void,
                fallback: Some(rec.fallback.ships.clone()),
                kill_gate: Some(rec.fallback.kill_gate.clone()),
                detail: format!("pilot `{}` is `{}`", rec.id, eff.as_str()),
            },
        }
    } else {
        // No appended `[[pilot.result]]` yet.
        match eff {
            PilotStatus::Registered => MechanismVerdict::Blocked {
                reason: HarnessReason::Registered,
                fallback: Some(rec.fallback.ships.clone()),
                kill_gate: Some(rec.fallback.kill_gate.clone()),
                detail: format!("pilot `{}` is registered with no result yet", rec.id),
            },
            PilotStatus::Running => MechanismVerdict::Blocked {
                reason: HarnessReason::Running,
                fallback: Some(rec.fallback.ships.clone()),
                kill_gate: Some(rec.fallback.kill_gate.clone()),
                detail: format!("pilot `{}` is running with no result yet", rec.id),
            },
            PilotStatus::FalsifierHolds => MechanismVerdict::Blocked {
                reason: HarnessReason::FalsifierHolds,
                fallback: Some(rec.fallback.ships.clone()),
                kill_gate: Some(rec.fallback.kill_gate.clone()),
                detail: format!(
                    "pilot `{}` falsifier holds; fallback ships: `{}` (kill gate: `{}`)",
                    rec.id, rec.fallback.ships, rec.fallback.kill_gate
                ),
            },
            PilotStatus::Inconclusive => MechanismVerdict::Blocked {
                reason: HarnessReason::Inconclusive,
                fallback: Some(rec.fallback.ships.clone()),
                kill_gate: Some(rec.fallback.kill_gate.clone()),
                detail: format!("pilot `{}` is inconclusive", rec.id),
            },
            PilotStatus::Differs => MechanismVerdict::Blocked {
                reason: HarnessReason::PlanTimeDiffers,
                fallback: Some(rec.fallback.ships.clone()),
                kill_gate: Some(rec.fallback.kill_gate.clone()),
                detail: format!("pilot `{}` differs from plan-time values", rec.id),
            },
            PilotStatus::Withdrawn => MechanismVerdict::Blocked {
                reason: HarnessReason::Missing,
                fallback: Some(rec.fallback.ships.clone()),
                kill_gate: Some(rec.fallback.kill_gate.clone()),
                detail: format!("pilot `{}` was withdrawn", rec.id),
            },
            PilotStatus::Void => MechanismVerdict::Blocked {
                reason: HarnessReason::Void,
                fallback: Some(rec.fallback.ships.clone()),
                kill_gate: Some(rec.fallback.kill_gate.clone()),
                detail: format!("pilot `{}` is void", rec.id),
            },
            PilotStatus::FalsifierRejected | PilotStatus::Reproduced => MechanismVerdict::Blocked {
                reason: HarnessReason::Registered,
                fallback: Some(rec.fallback.ships.clone()),
                kill_gate: Some(rec.fallback.kill_gate.clone()),
                detail: format!(
                    "pilot `{}` has status `{}` but no appended `[[pilot.result]]`",
                    rec.id,
                    eff.as_str()
                ),
            },
        }
    }
}

fn check_kill_gate_and_fund(
    rec: &PilotRecord,
    gates: Option<&GateRegistry>,
    provisional: bool,
    queued_for_referee: bool,
) -> MechanismVerdict {
    let kg = rec.fallback.kill_gate.trim();
    // Round-4 Review Note 4 (slice S-14): tier-S pilot with `kill_gate = "none"`
    // funds on the result alone.
    if kg == "none" && rec.tier == PilotTier::S {
        return MechanismVerdict::Fundable {
            provisional,
            queued_for_referee,
            supports_gate: false,
            note: None,
        };
    }

    if kg == "to-register" {
        return MechanismVerdict::Blocked {
            reason: HarnessReason::KillGateToRegister,
            fallback: Some(rec.fallback.ships.clone()),
            kill_gate: Some(rec.fallback.kill_gate.clone()),
            detail: format!(
                "pilot `{}` kill gate is still `to-register`; register the kill gate in registries/gates.toml before funding",
                rec.id
            ),
        };
    }

    if let Some(gate_reg) = gates
        && gate_reg.get(kg).is_none()
    {
        return MechanismVerdict::Blocked {
            reason: HarnessReason::KillGateToRegister,
            fallback: Some(rec.fallback.ships.clone()),
            kill_gate: Some(rec.fallback.kill_gate.clone()),
            detail: format!(
                "pilot `{}` kill gate `{kg}` is not registered in registries/gates.toml",
                rec.id
            ),
        };
    }

    MechanismVerdict::Fundable {
        provisional,
        queued_for_referee,
        supports_gate: false,
        note: None,
    }
}

/// Evaluate funding for a single `(work_package, mechanism)` pair (`pilots::fundable_mechanism`).
#[must_use]
pub fn fundable_mechanism_in(
    reg: &PilotRegister,
    work_package: &str,
    mechanism: &str,
    gates: Option<&GateRegistry>,
    root: Option<&Path>,
    previous: Option<&PilotRegister>,
    ws: &WorkspacePackages,
) -> MechanismFunding {
    let matching: Vec<&PilotRecord> = reg
        .pilots
        .iter()
        .filter(|p| p.work_package == work_package && p.mechanism == mechanism)
        .collect();

    if matching.is_empty() {
        return MechanismFunding {
            mechanism: mechanism.to_owned(),
            pilot_ids: Vec::new(),
            verdict: MechanismVerdict::Blocked {
                reason: HarnessReason::Missing,
                fallback: None,
                kill_gate: None,
                detail: format!(
                    "mechanism `{mechanism}` in `{work_package}` has no pilot record in pilots.toml"
                ),
            },
        };
    }

    let pilot_ids: Vec<String> = matching.iter().map(|p| p.id.clone()).collect();
    let mut any_provisional = false;
    let mut any_queued = false;
    let mut first_blocked: Option<MechanismVerdict> = None;

    for rec in matching {
        let v = evaluate_record(reg, rec, gates, root, previous, ws);
        match v {
            MechanismVerdict::Fundable {
                provisional,
                queued_for_referee,
                ..
            } => {
                any_provisional |= provisional;
                any_queued |= queued_for_referee;
            }
            blocked => {
                // Prioritize `falsifier-holds` over other blocked verdicts when a mechanism has multiple pilots.
                let replace = match &first_blocked {
                    None => true,
                    Some(MechanismVerdict::Blocked { reason, .. }) => {
                        *reason != HarnessReason::FalsifierHolds
                            && blocked.reason() == Some(HarnessReason::FalsifierHolds)
                    }
                    Some(_) => false,
                };
                if replace {
                    first_blocked = Some(blocked);
                }
            }
        }
    }

    let verdict = first_blocked.unwrap_or(MechanismVerdict::Fundable {
        provisional: any_provisional,
        queued_for_referee: any_queued,
        supports_gate: false,
        note: None,
    });

    MechanismFunding {
        mechanism: mechanism.to_owned(),
        pilot_ids,
        verdict,
    }
}

/// Evaluate funding for a work package (`pilots::fundable`).
#[must_use]
pub fn fundable_package_in(
    reg: &PilotRegister,
    work_package: &str,
    gates: Option<&GateRegistry>,
    root: Option<&Path>,
    previous: Option<&PilotRegister>,
    ws: &WorkspacePackages,
) -> PackageFunding {
    let mut mech_names: BTreeSet<String> = BTreeSet::new();

    // 1. Every mechanism with a record in `pilots.toml` for this package (including tier S).
    for rec in &reg.pilots {
        if rec.work_package == work_package {
            mech_names.insert(rec.mechanism.clone());
        }
    }

    // 2. Any declared upper-tier mechanisms for this package from workspace registries.
    if let Some(meta) = ws.upper_tier_packages.get(work_package) {
        if meta.mechanisms.is_empty() {
            if mech_names.is_empty() {
                // Package is tagged [F] or [M] with no pilot records at all.
                mech_names.insert(meta.title.clone());
            }
        } else {
            for m in &meta.mechanisms {
                mech_names.insert(m.clone());
            }
        }
    }

    let mechanisms: Vec<MechanismFunding> = mech_names
        .into_iter()
        .map(|m| fundable_mechanism_in(reg, work_package, &m, gates, root, previous, ws))
        .collect();

    let status = if mechanisms.is_empty() {
        PackageFundingStatus::Fundable
    } else {
        let fundable_n = mechanisms
            .iter()
            .filter(|m| m.verdict.is_fundable())
            .count();
        if fundable_n == mechanisms.len() {
            PackageFundingStatus::Fundable
        } else if fundable_n == 0 {
            PackageFundingStatus::Blocked
        } else {
            PackageFundingStatus::Partial
        }
    };

    PackageFunding {
        work_package: work_package.to_owned(),
        status,
        mechanisms,
    }
}

/// Evaluate funding for one bead (`pilots::fundable_bead`).
///
/// Rules (`N38a-09` + Round-4 Review Note 3):
/// 1. **Pilot's own bead**: if `key` is the `bead` of some record `R`, ignore `R`
///    (even if `R.gated_beads` lists `key`) and ignore package-wide ungated records;
///    check only whether some *other* mechanism's record explicitly lists `key` in
///    its `gated_beads`. If none does, return `Fundable` with `note = Some("pilot-bead")`.
/// 2. **Explicitly gated bead**: if one or more records list `key` in `gated_beads`
///    (excluding any record whose own `bead == key`), answer from those records'
///    mechanisms alone.
/// 3. **Unlisted bead (Round-4 Review Note 3, slice S-02)**: if no record lists `key`
///    in `gated_beads`, look up `key`'s work package. Answer ONLY from the records of
///    that work package that list NO `gated_beads` at all (and do not exempt `key` in
///    `fallback.beads`), plus any unpiloted upper-tier package obligation when the
///    package has no pilot records at all. Consequently:
///    - A single-mechanism package with `gated_beads = []` gates all non-pilot beads.
///    - A multi-mechanism package whose records declare `gated_beads` does NOT block
///      fallback beads (`BD-333` in WP-64) or ungated beads (`BD-364` in WP-66).
///    - A bead named in `fallback.beads` of a package-wide record is likewise exempt.
#[must_use]
pub fn fundable_bead_in(
    reg: &PilotRegister,
    key: &str,
    work_package_hint: Option<&str>,
    gates: Option<&GateRegistry>,
    root: Option<&Path>,
    previous: Option<&PilotRegister>,
    ws: &WorkspacePackages,
) -> BeadFunding {
    let own_pilot_records: Vec<&PilotRecord> =
        reg.pilots.iter().filter(|r| r.bead == key).collect();
    let is_pilot_bead = !own_pilot_records.is_empty();

    // Find every record that explicitly gates `key` (ignoring self-gating on its own `bead`).
    let explicitly_gating: Vec<&PilotRecord> = reg
        .pilots
        .iter()
        .filter(|r| {
            r.bead != key
                && r.gated_beads.iter().any(|b| b == key)
                && !own_pilot_records
                    .iter()
                    .any(|own| own.work_package == r.work_package && own.mechanism == r.mechanism)
        })
        .collect();

    let resolved_wp = work_package_hint
        .map(str::to_owned)
        .or_else(|| own_pilot_records.first().map(|r| r.work_package.clone()))
        .or_else(|| explicitly_gating.first().map(|r| r.work_package.clone()))
        .or_else(|| ws.bead_to_package.get(key).cloned());

    // Case 1: `key` is a pilot's own bead and no other mechanism explicitly gates it.
    if is_pilot_bead && explicitly_gating.is_empty() {
        return BeadFunding {
            bead: key.to_owned(),
            work_package: resolved_wp,
            fundable: true,
            provisional: false,
            queued_for_referee: false,
            note: Some("pilot-bead".to_owned()),
            reason: None,
            fallback: None,
            kill_gate: None,
            detail: format!("bead `{key}` is a pilot's own bead and is never self-gated"),
            mechanisms: Vec::new(),
        };
    }

    // Case 2: explicitly listed in one or more records' `gated_beads`.
    if !explicitly_gating.is_empty() {
        let mut seen_mechs: BTreeSet<(String, String)> = BTreeSet::new();
        let mut mechs = Vec::new();
        for rec in explicitly_gating {
            if seen_mechs.insert((rec.work_package.clone(), rec.mechanism.clone())) {
                mechs.push(fundable_mechanism_in(
                    reg,
                    &rec.work_package,
                    &rec.mechanism,
                    gates,
                    root,
                    previous,
                    ws,
                ));
            }
        }
        return summarize_bead_funding(key, resolved_wp, mechs, None);
    }

    // Case 3: not listed in any `gated_beads` -> apply Review Note 3 rule on its work package.
    let Some(wp) = resolved_wp else {
        return BeadFunding {
            bead: key.to_owned(),
            work_package: None,
            fundable: true,
            provisional: false,
            queued_for_referee: false,
            note: None,
            reason: None,
            fallback: None,
            kill_gate: None,
            detail: format!("bead `{key}` is not gated by any pilot record or upper-tier package"),
            mechanisms: Vec::new(),
        };
    };

    let wp_records: Vec<&PilotRecord> =
        reg.pilots.iter().filter(|r| r.work_package == wp).collect();

    // If the package has NO pilot records at all, check whether the package itself is upper-tier.
    if wp_records.is_empty() {
        let pkg = fundable_package_in(reg, &wp, gates, root, previous, ws);
        return summarize_bead_funding(key, Some(wp), pkg.mechanisms, None);
    }

    // Check if explicitly exempted as a fallback bead on any package record.
    let is_declared_fallback = wp_records
        .iter()
        .any(|r| r.fallback.beads.iter().any(|b| b == key));

    // Only records of this package that list NO `gated_beads` at all (and do not list `key`
    // in `fallback.beads`) govern unlisted beads of this package (Review Note 3, slice S-02).
    let ungated_records: Vec<&PilotRecord> = wp_records
        .iter()
        .copied()
        .filter(|r| r.gated_beads.is_empty() && !r.fallback.beads.iter().any(|b| b == key))
        .collect();

    if ungated_records.is_empty() {
        let note = if is_declared_fallback {
            Some("fallback-bead".to_owned())
        } else {
            Some("ungated-by-package-pilots".to_owned())
        };
        return BeadFunding {
            bead: key.to_owned(),
            work_package: Some(wp.clone()),
            fundable: true,
            provisional: false,
            queued_for_referee: false,
            note,
            reason: None,
            fallback: None,
            kill_gate: None,
            detail: format!(
                "bead `{key}` in `{wp}` is not gated by any pilot record of `{wp}` (every record either declares explicit `gated_beads` or exempts `{key}` as fallback work)"
            ),
            mechanisms: Vec::new(),
        };
    }

    let mut seen_mechs: BTreeSet<String> = BTreeSet::new();
    let mut mechs = Vec::new();
    for rec in ungated_records {
        if seen_mechs.insert(rec.mechanism.clone()) {
            mechs.push(fundable_mechanism_in(
                reg,
                &wp,
                &rec.mechanism,
                gates,
                root,
                previous,
                ws,
            ));
        }
    }

    summarize_bead_funding(key, Some(wp), mechs, None)
}

fn summarize_bead_funding(
    key: &str,
    work_package: Option<String>,
    mechanisms: Vec<MechanismFunding>,
    note: Option<String>,
) -> BeadFunding {
    let mut provisional = false;
    let mut queued = false;
    for m in &mechanisms {
        match &m.verdict {
            MechanismVerdict::Fundable {
                provisional: p,
                queued_for_referee: q,
                ..
            } => {
                provisional |= *p;
                queued |= *q;
            }
            MechanismVerdict::Blocked {
                reason,
                fallback,
                kill_gate,
                detail,
            } => {
                return BeadFunding {
                    bead: key.to_owned(),
                    work_package,
                    fundable: false,
                    provisional: false,
                    queued_for_referee: false,
                    note: None,
                    reason: Some(*reason),
                    fallback: fallback.clone(),
                    kill_gate: kill_gate.clone(),
                    detail: detail.clone(),
                    mechanisms,
                };
            }
        }
    }

    BeadFunding {
        bead: key.to_owned(),
        work_package,
        fundable: true,
        provisional,
        queued_for_referee: queued,
        note,
        reason: None,
        fallback: None,
        kill_gate: None,
        detail: format!("bead `{key}` is fundable"),
        mechanisms,
    }
}

/// Evaluate every upper-tier (`[F]` or `[M]`) or pilot-bearing work package (`fundable --all`).
#[must_use]
pub fn fundable_all_in(
    reg: &PilotRegister,
    gates: Option<&GateRegistry>,
    root: Option<&Path>,
    previous: Option<&PilotRegister>,
    ws: &WorkspacePackages,
) -> AllPackagesFunding {
    let mut all_wps: BTreeSet<String> = ws.upper_tier_packages.keys().cloned().collect();
    for rec in &reg.pilots {
        if !rec.work_package.trim().is_empty() {
            all_wps.insert(rec.work_package.clone());
        }
    }

    let mut packages = Vec::with_capacity(all_wps.len());
    let mut fundable_count = 0usize;
    let mut partial_count = 0usize;
    let mut refused_count = 0usize;

    for wp in all_wps {
        let pkg = fundable_package_in(reg, &wp, gates, root, previous, ws);
        match pkg.status {
            PackageFundingStatus::Fundable => fundable_count += 1,
            PackageFundingStatus::Partial => partial_count += 1,
            PackageFundingStatus::Blocked => refused_count += 1,
        }
        packages.push(pkg);
    }

    AllPackagesFunding {
        packages,
        fundable_count,
        partial_count,
        refused_count,
        status_counts: StatusCounts::from_register(reg, root, previous),
        moonshots_running_by_milestone: moonshots_running_by_milestone(reg, root, previous, ws),
    }
}

/// Resolve a record's unconditional `fallback.ships` and conditional `fallback.also`
/// entries: an `also` entry ships only while `fundable_mechanism` answers `Fundable`
/// for that entry's pilot mechanism.
#[must_use]
pub fn resolve_fallback(
    reg: &PilotRegister,
    rec: &PilotRecord,
    gates: Option<&GateRegistry>,
    root: Option<&Path>,
    previous: Option<&PilotRegister>,
    ws: &WorkspacePackages,
) -> FallbackResolution {
    let mut active = Vec::new();
    let mut inactive = Vec::new();
    for item in &rec.fallback.also {
        match reg.get(&item.pilot) {
            Some(target) => {
                let mf = fundable_mechanism_in(
                    reg,
                    &target.work_package,
                    &target.mechanism,
                    gates,
                    root,
                    previous,
                    ws,
                );
                match mf.verdict {
                    MechanismVerdict::Fundable { .. } => {
                        active.push((item.ships.clone(), item.pilot.clone()));
                    }
                    MechanismVerdict::Blocked { reason, .. } => {
                        inactive.push((item.ships.clone(), item.pilot.clone(), reason));
                    }
                }
            }
            None => {
                inactive.push((
                    item.ships.clone(),
                    item.pilot.clone(),
                    HarnessReason::Missing,
                ));
            }
        }
    }

    FallbackResolution {
        unconditional: rec.fallback.ships.clone(),
        kill_gate: rec.fallback.kill_gate.clone(),
        conditional_active: active,
        conditional_inactive: inactive,
    }
}

/// Evaluate `GATE-GAUNT-08` (first clause): scan every funded Frontier or Moonshot
/// work package and verify each has a complete pilot record with a passing result.
#[must_use]
pub fn scan_gate_gaunt_08(
    reg: &PilotRegister,
    funded_upper_tier_packages: &[&str],
    gates: Option<&GateRegistry>,
    root: Option<&Path>,
    previous: Option<&PilotRegister>,
    ws: &WorkspacePackages,
) -> GateGaunt08Scan {
    let mut violations = Vec::new();
    for &wp in funded_upper_tier_packages {
        let pf = fundable_package_in(reg, wp, gates, root, previous, ws);
        if pf.mechanisms.is_empty() {
            violations.push(GateGaunt08Violation {
                work_package: wp.to_owned(),
                mechanism: wp.to_owned(),
                reason: HarnessReason::Missing,
                detail: format!("funded upper-tier work package `{wp}` has no pilot record"),
            });
            continue;
        }
        for mf in pf.mechanisms {
            if let MechanismVerdict::Blocked { reason, detail, .. } = mf.verdict {
                violations.push(GateGaunt08Violation {
                    work_package: wp.to_owned(),
                    mechanism: mf.mechanism,
                    reason,
                    detail,
                });
            }
        }
    }
    GateGaunt08Scan {
        packages_scanned: funded_upper_tier_packages.len(),
        violations,
    }
}
