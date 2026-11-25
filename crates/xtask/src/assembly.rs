//! Closed six-check assembly, finding-identifier ownership verification, and
//! GATE-WS-04 timing budget table for `cargo xtask` (bead `BD-004`, INV-WS-05,
//! GATE-WS-01, GATE-WS-04).
//!
//! Single authority consumed by:
//! - `crates/xtask/src/main.rs` (executes [`COMMANDS`] directly; no `all` subcommand),
//! - `crates/xtask/tests/plants.rs` (`exactly_six_subcommands`, `unknown_subcommand_is_usage_error`,
//!   `every_finding_id_has_one_owner`, `plant_matrix_diagonal`, `plant_5_on_real_tree_scratch_copy`,
//!   and `in_process_timing`),
//! - downstream E2E scripts (`N34-14`, `scripts/e2e/workspace_checks.sh`).
//!
//! Note on review notes: `check-stack` was registered by `N33a-05` in `BD-003`'s surface and
//! is fixed here as the sixth entry; `BD-022` adds the later `verbs_table.rs` generated-artifact
//! rule to `check-generated`. Determinism class D0, zero `unsafe`, prints nothing (rule L-7).

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::path::Path;
use std::process::ExitCode;

use crate::check_contracts;
use crate::check_deps;
use crate::check_generated;
use crate::check_layers;
use crate::check_unsafe;
use crate::cli::{self, CheckFn};
use crate::report::validate_report_json;
use crate::stack;

/// Exact closed set of six `cargo xtask` subcommand names (INV-WS-05).
///
/// A seventh check requires a named defect class it would have caught and explicit
/// owner approval. No `all` subcommand exists: running all six is a loop in a script.
pub const EXPECTED_SUBCOMMANDS: [&str; 6] = [
    "check-layers",
    "check-deps",
    "check-unsafe",
    "check-generated",
    "check-contracts",
    "check-stack",
];

/// Closed six-check command table executed directly by `src/main.rs` (`BD-004`).
pub const COMMANDS: &[(&str, CheckFn)] = &[
    ("check-layers", check_layers::run_command),
    ("check-deps", check_deps::run_command),
    ("check-unsafe", check_unsafe::run_command),
    ("check-generated", check_generated::run_command),
    ("check-contracts", check_contracts::run_command),
    ("check-stack", stack::verify::run_command),
];

/// Authoritative finding-identifier ownership table across all six checks.
///
/// Every finding rule identifier emitted by `xtask` belongs to strictly one check.
pub const CHECK_OWNERS: &[(&str, &[&str])] = &[
    ("check-layers", check_layers::OWNED_RULES),
    ("check-deps", check_deps::OWNED_RULES),
    ("check-unsafe", check_unsafe::OWNED_RULES),
    ("check-generated", check_generated::OWNED_RULES),
    ("check-contracts", check_contracts::OWNED_RULES),
    ("check-stack", stack::verify::OWNED_RULES),
];

/// The seven canonical plants of `GATE-WS-01` paired with their unique owning check.
pub const GATE_WS_01_PLANTS: &[(&str, &str)] = &[
    ("plant-1-upward-edge", "check-layers"),
    ("plant-2-rank-edge", "check-layers"),
    ("plant-3-second-asupersync", "check-deps"),
    ("plant-4-unregistered-capsule", "check-unsafe"),
    ("plant-5-edited-generated-file", "check-generated"),
    ("plant-6-contract-missing-section", "check-contracts"),
    ("plant-7-stale-stack-lock", "check-stack"),
];

/// Total warm-tree wall-time budget for all six checks combined (`GATE-WS-04`: 60 s).
pub const GATE_WS_04_TOTAL_BUDGET_MS: u64 = 60_000;

/// Combined warm-tree budget share of `check-layers` + `check-deps` (`GATE-WS-04`: 25 s).
pub const GATE_WS_04_LAYERS_AND_DEPS_BUDGET_MS: u64 = 25_000;

/// Breakdown of the 25 s `check-layers` + `check-deps` share (`BD-004` authority table).
pub const GATE_WS_04_LAYERS_AND_DEPS_SUBSHARES: &[(&str, u64)] = &[
    ("check-layers + BD-003 (shared resolved metadata)", 10_000),
    ("N34-03 (suite rules + --all-features resolution)", 7_000),
    ("N34-04 (JSON-library guard cargo tree per binary)", 5_000),
    ("N34-25 (auditor rules and shared-source scan)", 3_000),
];

/// One disjoint budget share entry in the `GATE-WS-04` 60 s table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BudgetShare {
    pub label: &'static str,
    pub checks: &'static [&'static str],
    pub budget_ms: u64,
}

/// Authoritative disjoint `GATE-WS-04` budget shares summing to `60_000 ms`.
pub const GATE_WS_04_SHARES: &[BudgetShare] = &[
    BudgetShare {
        label: "check-layers+check-deps",
        checks: &["check-layers", "check-deps"],
        budget_ms: 25_000,
    },
    BudgetShare {
        label: "check-unsafe",
        checks: &["check-unsafe"],
        budget_ms: 5_000,
    },
    BudgetShare {
        label: "check-generated",
        checks: &["check-generated"],
        budget_ms: 10_000,
    },
    BudgetShare {
        label: "check-contracts",
        checks: &["check-contracts"],
        budget_ms: 10_000,
    },
    BudgetShare {
        label: "check-stack",
        checks: &["check-stack"],
        budget_ms: 10_000,
    },
];

/// Solo upper bound for each individual check (`GATE-WS-04`).
#[must_use]
pub fn check_solo_budget_ms(check: &str) -> Option<u64> {
    match check {
        "check-layers" => Some(10_000),
        "check-deps" => Some(25_000),
        "check-unsafe" => Some(5_000),
        "check-generated" => Some(10_000),
        "check-contracts" => Some(10_000),
        "check-stack" => Some(10_000),
        _ => None,
    }
}

/// Failure when validating the closed six-subcommand table (INV-WS-05).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandTableError {
    WrongCount {
        expected: usize,
        actual: usize,
        names: Vec<String>,
    },
    DuplicateName(String),
    NameSetMismatch {
        missing: Vec<String>,
        unexpected: Vec<String>,
    },
}

impl fmt::Display for CommandTableError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WrongCount {
                expected,
                actual,
                names,
            } => write!(
                f,
                "INV-WS-05 violated: expected exactly {expected} xtask subcommands, found {actual}: {names:?}"
            ),
            Self::DuplicateName(name) => {
                write!(f, "INV-WS-05 violated: duplicate subcommand `{name}` in command table")
            }
            Self::NameSetMismatch {
                missing,
                unexpected,
            } => write!(
                f,
                "INV-WS-05 violated: subcommand set mismatch (missing={missing:?}, unexpected={unexpected:?})"
            ),
        }
    }
}

impl std::error::Error for CommandTableError {}

/// Verify that `commands` contains strictly the six subcommands in [`EXPECTED_SUBCOMMANDS`].
///
/// Compares both exact length and the set of names so adding a seventh check, renaming a check,
/// or duplicating an entry all fail.
pub fn verify_command_table(commands: &[(&str, CheckFn)]) -> Result<(), CommandTableError> {
    let expected: BTreeSet<&str> = EXPECTED_SUBCOMMANDS.into_iter().collect();
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    let mut names: Vec<String> = Vec::with_capacity(commands.len());

    for &(name, _) in commands {
        names.push(name.to_owned());
        if !seen.insert(name) {
            return Err(CommandTableError::DuplicateName(name.to_owned()));
        }
    }

    let missing: Vec<String> = expected
        .difference(&seen)
        .map(|s| (*s).to_owned())
        .collect();
    let unexpected: Vec<String> = seen
        .difference(&expected)
        .map(|s| (*s).to_owned())
        .collect();

    if !missing.is_empty() || !unexpected.is_empty() {
        return Err(CommandTableError::NameSetMismatch {
            missing,
            unexpected,
        });
    }

    if commands.len() != EXPECTED_SUBCOMMANDS.len() {
        return Err(CommandTableError::WrongCount {
            expected: EXPECTED_SUBCOMMANDS.len(),
            actual: commands.len(),
            names,
        });
    }

    Ok(())
}

/// Failure when validating unique finding-identifier ownership across checks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OwnershipError {
    CheckSetMismatch {
        missing: Vec<String>,
        unexpected: Vec<String>,
    },
    EmptyRuleId {
        owner: String,
    },
    DuplicateWithinOwner {
        owner: String,
        rule: String,
    },
    DuplicateAcrossOwners {
        rule: String,
        first_owner: String,
        second_owner: String,
    },
}

impl fmt::Display for OwnershipError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CheckSetMismatch {
                missing,
                unexpected,
            } => write!(
                f,
                "ownership table check set mismatch (missing={missing:?}, unexpected={unexpected:?})"
            ),
            Self::EmptyRuleId { owner } => {
                write!(f, "check `{owner}` declares an empty rule identifier")
            }
            Self::DuplicateWithinOwner { owner, rule } => {
                write!(f, "check `{owner}` lists rule `{rule}` more than once")
            }
            Self::DuplicateAcrossOwners {
                rule,
                first_owner,
                second_owner,
            } => write!(
                f,
                "finding rule `{rule}` has two owners: `{first_owner}` and `{second_owner}`"
            ),
        }
    }
}

impl std::error::Error for OwnershipError {}

/// Verify that all six checks are represented in `owners` and that the union of finding
/// identifiers across all checks has zero duplicates.
pub fn verify_finding_id_ownership(
    owners: &[(&str, &[&str])],
) -> Result<BTreeMap<String, String>, OwnershipError> {
    let expected: BTreeSet<&str> = EXPECTED_SUBCOMMANDS.into_iter().collect();
    let observed: BTreeSet<&str> = owners.iter().map(|&(name, _)| name).collect();
    if expected != observed {
        return Err(OwnershipError::CheckSetMismatch {
            missing: expected
                .difference(&observed)
                .map(|s| (*s).to_owned())
                .collect(),
            unexpected: observed
                .difference(&expected)
                .map(|s| (*s).to_owned())
                .collect(),
        });
    }

    let mut owner_by_rule: BTreeMap<String, String> = BTreeMap::new();
    for &(check_name, rules) in owners {
        let mut local_seen: BTreeSet<&str> = BTreeSet::new();
        for &rule in rules {
            if rule.is_empty() {
                return Err(OwnershipError::EmptyRuleId {
                    owner: check_name.to_owned(),
                });
            }
            if !local_seen.insert(rule) {
                return Err(OwnershipError::DuplicateWithinOwner {
                    owner: check_name.to_owned(),
                    rule: rule.to_owned(),
                });
            }
            if let Some(prev) = owner_by_rule.insert(rule.to_owned(), check_name.to_owned()) {
                return Err(OwnershipError::DuplicateAcrossOwners {
                    rule: rule.to_owned(),
                    first_owner: prev,
                    second_owner: check_name.to_owned(),
                });
            }
        }
    }
    Ok(owner_by_rule)
}

/// One check or share that exceeded its `GATE-WS-04` budget share.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimingOverrun {
    pub name: String,
    pub elapsed_ms: u64,
    pub budget_ms: u64,
}

impl fmt::Display for TimingOverrun {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {} ms exceeds budget share {} ms",
            self.name, self.elapsed_ms, self.budget_ms
        )
    }
}

/// Evaluation of all six check wall times against `GATE-WS-04` shares and 60 s total.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimingEvaluation {
    pub total_ms: u64,
    pub total_budget_ms: u64,
    pub within_total_budget: bool,
    pub per_check_ms: BTreeMap<String, u64>,
    pub layers_plus_deps_ms: u64,
    pub overruns: Vec<TimingOverrun>,
}

impl TimingEvaluation {
    /// True when the total is within 60 s AND no individual check or share overran.
    #[must_use]
    pub fn passed(&self) -> bool {
        self.within_total_budget && self.overruns.is_empty()
    }

    /// Format a clear summary listing total wall time and every named overrun.
    #[must_use]
    pub fn format_summary(&self) -> String {
        let mut lines = vec![format!(
            "GATE-WS-04 total: {} ms / {} ms (layers+deps: {} ms / {} ms)",
            self.total_ms,
            self.total_budget_ms,
            self.layers_plus_deps_ms,
            GATE_WS_04_LAYERS_AND_DEPS_BUDGET_MS
        )];
        for (check, ms) in &self.per_check_ms {
            let share = check_solo_budget_ms(check).unwrap_or(0);
            lines.push(format!("  {check}: {ms} ms (solo share {share} ms)"));
        }
        if !self.overruns.is_empty() {
            lines.push("overruns:".to_owned());
            for ov in &self.overruns {
                lines.push(format!("  {ov}"));
            }
        }
        lines.join("\n")
    }
}

/// Evaluate measured per-check `elapsed_ms` values against `GATE-WS-04`.
///
/// Any check or combined share exceeding its budget is reported by name in `overruns`
/// even when `total_ms <= GATE_WS_04_TOTAL_BUDGET_MS`.
pub fn evaluate_gate_ws_04_timing(
    elapsed_by_check: &BTreeMap<String, u64>,
) -> Result<TimingEvaluation, String> {
    for &check in &EXPECTED_SUBCOMMANDS {
        if !elapsed_by_check.contains_key(check) {
            return Err(format!(
                "missing elapsed_ms measurement for required check `{check}`"
            ));
        }
    }

    let mut total_ms: u64 = 0;
    let mut per_check_ms: BTreeMap<String, u64> = BTreeMap::new();
    let mut overruns: Vec<TimingOverrun> = Vec::new();

    for &check in &EXPECTED_SUBCOMMANDS {
        let ms = elapsed_by_check[check];
        total_ms = total_ms.saturating_add(ms);
        per_check_ms.insert(check.to_owned(), ms);

        // Solo check bound (for check-layers <= 10s, check-deps <= 25s, and each standalone share).
        if let Some(solo_budget) = check_solo_budget_ms(check)
            && ms > solo_budget
        {
            overruns.push(TimingOverrun {
                name: check.to_owned(),
                elapsed_ms: ms,
                budget_ms: solo_budget,
            });
        }
    }

    let layers_plus_deps_ms = per_check_ms["check-layers"].saturating_add(per_check_ms["check-deps"]);
    if layers_plus_deps_ms > GATE_WS_04_LAYERS_AND_DEPS_BUDGET_MS {
        overruns.push(TimingOverrun {
            name: "check-layers+check-deps".to_owned(),
            elapsed_ms: layers_plus_deps_ms,
            budget_ms: GATE_WS_04_LAYERS_AND_DEPS_BUDGET_MS,
        });
    }

    Ok(TimingEvaluation {
        total_ms,
        total_budget_ms: GATE_WS_04_TOTAL_BUDGET_MS,
        within_total_budget: total_ms <= GATE_WS_04_TOTAL_BUDGET_MS,
        per_check_ms,
        layers_plus_deps_ms,
        overruns,
    })
}

/// Result of running one `xtask` subcommand in process through `cli::run(COMMANDS, ...)`.
#[derive(Debug, Clone)]
pub struct CliJsonRun {
    pub check: String,
    pub exit_code: ExitCode,
    pub stdout: String,
    pub stderr: String,
}

/// Run one subcommand from [`COMMANDS`] against `root` with `--json` in process.
#[must_use]
pub fn run_check_cli_json(root: &Path, check: &str, extra: &[&str]) -> CliJsonRun {
    let mut args = vec![
        check.to_owned(),
        "--root".to_owned(),
        root.to_string_lossy().into_owned(),
        "--json".to_owned(),
    ];
    args.extend(extra.iter().map(|s| (*s).to_owned()));
    let mut out = Vec::new();
    let mut err = Vec::new();
    let exit_code = cli::run(COMMANDS, &args, &mut out, &mut err);
    CliJsonRun {
        check: check.to_owned(),
        exit_code,
        stdout: String::from_utf8(out).unwrap_or_default(),
        stderr: String::from_utf8(err).unwrap_or_default(),
    }
}

/// Validate a canonical `frob.xtask.report/1` JSON report and zero its top-level
/// `"elapsed_ms"` field so two runs can be compared byte-for-byte for D0 determinism.
pub fn normalize_report_json_for_determinism(raw_json: &str) -> Result<String, String> {
    validate_report_json(raw_json.as_bytes()).map_err(|e| format!("invalid report JSON: {e}"))?;
    let needle = "\"elapsed_ms\":";
    let Some(idx) = raw_json.find(needle) else {
        return Err("canonical report JSON missing `\"elapsed_ms\":` field".to_owned());
    };
    let num_start = idx + needle.len();
    let digit_len = raw_json[num_start..]
        .bytes()
        .take_while(u8::is_ascii_digit)
        .count();
    if digit_len == 0 {
        return Err("canonical report JSON `elapsed_ms` is not an unsigned integer".to_owned());
    }
    let mut normalized = String::with_capacity(raw_json.len());
    normalized.push_str(&raw_json[..num_start]);
    normalized.push('0');
    normalized.push_str(&raw_json[num_start + digit_len..]);
    Ok(normalized)
}
