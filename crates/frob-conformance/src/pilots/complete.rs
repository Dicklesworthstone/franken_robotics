#![deny(unsafe_code)]

//! Completeness check (`pilots::complete`) and multi-mechanism `gated_beads`
//! warning pass (`N38a-09`, §38.5).
//!
//! Reads whichever of the capability (`registries/capabilities.toml`), kernel
//! (`registries/kernels.toml`), and verb (`registries/verbs.toml`) registries
//! exist at the commit, verifies every upper-tier (`F` or `M`) entry has a
//! matching pilot record, reports every unregistered row of
//! `crates/frob-conformance/registry/pilots_expected.toml`, and emits
//! non-failing warnings for records without `gated_beads` in a work package
//! holding more than one upper-tier mechanism.
//!
//! Determinism class D0; prints nothing (rule L-7); zero `unsafe`.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::fs;
use std::path::Path;

use frob_core::registry::toml::{self, Value};

use crate::pilots::schema::{
    PilotLoadError, PilotRecord, PilotRegister, PilotTier, PilotsExpected, load_pilots_expected,
};

/// One upper-tier mechanism declared in a workspace registry that has no
/// matching pilot record in `crates/frob-conformance/registry/pilots.toml`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MissingPilot {
    pub source: String,
    pub mechanism: String,
    pub tier: PilotTier,
    pub work_package: Option<String>,
}

impl fmt::Display for MissingPilot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.work_package {
            Some(wp) => write!(
                f,
                "reason=missing {}: upper-tier [{}] `{}` ({wp}) has no pilot record",
                self.source, self.tier, self.mechanism
            ),
            None => write!(
                f,
                "reason=missing {}: upper-tier [{}] `{}` has no pilot record",
                self.source, self.tier, self.mechanism
            ),
        }
    }
}

/// One row of `pilots_expected.toml` that has not yet been registered in `pilots.toml`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct UnregisteredSeedRow {
    pub identifier: String,
    pub pilot: Option<String>,
    pub kill_gate: String,
    pub mechanism: String,
    pub falsifier_threshold: Option<String>,
}

impl fmt::Display for UnregisteredSeedRow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "unregistered seed pilot `{}` (kill_gate={}, mechanism={})",
            self.identifier, self.kill_gate, self.mechanism
        )
    }
}

/// Non-failing warning for a record without `gated_beads` in a work package
/// holding more than one upper-tier mechanism.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct UngatedMultiMechanismWarning {
    pub work_package: String,
    pub pilot_id: String,
    pub pilot_bead: String,
    pub mechanism: String,
    pub other_mechanisms: Vec<String>,
}

impl fmt::Display for UngatedMultiMechanismWarning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "warning: pilot `{}` (bead `{}`, package `{}`) has no `gated_beads` in a multi-mechanism package alongside [{}]",
            self.pilot_id,
            self.pilot_bead,
            self.work_package,
            self.other_mechanisms.join(", ")
        )
    }
}

/// Full result of [`check_completeness`].
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CompletenessReport {
    /// Failing items: upper-tier entries in existing registries without a pilot row.
    pub missing: Vec<MissingPilot>,
    /// Tracking items: seed rows of `pilots_expected.toml` not yet registered in `pilots.toml`.
    pub seed_unregistered: Vec<UnregisteredSeedRow>,
    /// Non-failing warnings: records without `gated_beads` in multi-mechanism packages.
    pub warnings: Vec<UngatedMultiMechanismWarning>,
}

impl CompletenessReport {
    /// True iff no upper-tier registry entry is missing a pilot record.
    ///
    /// Note that `warnings` and `seed_unregistered` do not fail `is_ok()`.
    #[must_use]
    pub fn is_ok(&self) -> bool {
        self.missing.is_empty()
    }
}

/// Run the completeness check over `reg` against whichever registries exist under `root`.
pub fn check_completeness(
    reg: &PilotRegister,
    root: &Path,
) -> Result<CompletenessReport, PilotLoadError> {
    let expected = load_pilots_expected(root)?;
    Ok(check_completeness_with_expected(reg, &expected, root))
}

/// Run the completeness check with an already-loaded [`PilotsExpected`] seed list.
#[must_use]
pub fn check_completeness_with_expected(
    reg: &PilotRegister,
    expected: &PilotsExpected,
    root: &Path,
) -> CompletenessReport {
    let covered_mechanisms: BTreeSet<&str> =
        reg.pilots.iter().map(|p| p.mechanism.as_str()).collect();
    let covered_ids: BTreeSet<&str> = reg.pilots.iter().map(|p| p.id.as_str()).collect();
    let covered_kill_gates: BTreeSet<&str> = reg
        .pilots
        .iter()
        .map(|p| p.fallback.kill_gate.as_str())
        .collect();

    // 1. Upper-tier entries in capabilities, kernels, and verbs registries (whichever exist).
    let mut missing = Vec::new();
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
            let Some(mech) = row
                .get("mechanism")
                .or_else(|| row.get("id"))
                .or_else(|| row.get("name"))
                .and_then(Value::as_str)
            else {
                continue;
            };
            let alt_id = row.get("id").and_then(Value::as_str);
            let alt_name = row.get("name").and_then(Value::as_str);
            let is_covered = covered_mechanisms.contains(mech)
                || alt_id.is_some_and(|s| covered_mechanisms.contains(s))
                || alt_name.is_some_and(|s| covered_mechanisms.contains(s));
            if !is_covered {
                missing.push(MissingPilot {
                    source: rel.to_owned(),
                    mechanism: mech.to_owned(),
                    tier: t,
                    work_package: row
                        .get("work_package")
                        .and_then(Value::as_str)
                        .map(str::to_owned),
                });
            }
        }
    }
    missing.sort();
    missing.dedup();

    // 2. Seed list rows not yet registered in `pilots.toml`.
    let mut seed_unregistered = Vec::new();
    for row in &expected.rows {
        let registered = row
            .pilot
            .as_deref()
            .is_some_and(|pid| covered_ids.contains(pid))
            || covered_kill_gates.contains(row.kill_gate.as_str())
            || covered_mechanisms.contains(row.mechanism.as_str());
        if !registered {
            let identifier = row.pilot.clone().unwrap_or_else(|| row.kill_gate.clone());
            seed_unregistered.push(UnregisteredSeedRow {
                identifier,
                pilot: row.pilot.clone(),
                kill_gate: row.kill_gate.clone(),
                mechanism: row.mechanism.clone(),
                falsifier_threshold: row.falsifier_threshold.clone(),
            });
        }
    }

    // 3. Warnings for records without `gated_beads` in packages holding >1 upper-tier mechanism.
    let warnings = collect_ungated_multi_mechanism_warnings(reg);

    CompletenessReport {
        missing,
        seed_unregistered,
        warnings,
    }
}

fn collect_ungated_multi_mechanism_warnings(
    reg: &PilotRegister,
) -> Vec<UngatedMultiMechanismWarning> {
    let mut by_wp: BTreeMap<&str, Vec<&PilotRecord>> = BTreeMap::new();
    for rec in &reg.pilots {
        if rec.tier.is_upper() && !rec.work_package.trim().is_empty() {
            by_wp
                .entry(rec.work_package.as_str())
                .or_default()
                .push(rec);
        }
    }

    let mut warnings = Vec::new();
    for (wp, records) in by_wp {
        let distinct_mechs: BTreeSet<&str> = records.iter().map(|r| r.mechanism.as_str()).collect();
        if distinct_mechs.len() <= 1 {
            continue;
        }
        for rec in records {
            if rec.gated_beads.is_empty() {
                let other_mechanisms: Vec<String> = distinct_mechs
                    .iter()
                    .copied()
                    .filter(|&m| m != rec.mechanism.as_str())
                    .map(str::to_owned)
                    .collect();
                warnings.push(UngatedMultiMechanismWarning {
                    work_package: wp.to_owned(),
                    pilot_id: rec.id.clone(),
                    pilot_bead: rec.bead.clone(),
                    mechanism: rec.mechanism.clone(),
                    other_mechanisms,
                });
            }
        }
    }
    warnings.sort();
    warnings
}
