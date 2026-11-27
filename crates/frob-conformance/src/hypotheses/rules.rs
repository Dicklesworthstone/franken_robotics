//! Append-only change rule for `crates/frob-conformance/registry/hypotheses.toml` (`N00-10` step 2).
//!
//! Compares a current register against its git base (`HEAD` when the working tree differs from
//! `HEAD`, otherwise `HEAD~1`, matching `N00-03`).

use std::fmt;
use std::path::Path;
use std::process::Command;

use crate::hypotheses::schema::{HypothesisRegister, RegisterError, VerdictState};

pub const RULE_ROW_REMOVED: &str = "hyp.row-removed";
pub const RULE_DISAGREEMENT_REMOVED: &str = "hyp.disagreement-removed";
pub const RULE_HISTORY_EDITED: &str = "hyp.history-edited";
pub const RULE_STATE_HISTORY_MISMATCH: &str = "hyp.state-history-mismatch";
pub const RULE_NEW_ENTRY_MISSING_RECORD: &str = "hyp.new-entry-missing-record";
pub const RULE_STATE_CHANGE_WITHOUT_ENTRY: &str = "hyp.state-change-without-entry";
pub const RULE_PROPOSAL_ONLY_CANNOT_MOVE: &str = "hyp.proposal-only-cannot-move";
pub const RULE_REVERSAL_WITHOUT_OWNER_DECISION: &str = "hyp.reversal-without-owner-decision";
pub const RULE_REVERSAL_EDITED: &str = "hyp.reversal-edited";
pub const RULE_REFUTED_MOVE_WITHOUT_REVERSAL: &str = "hyp.refuted-move-without-reversal";

/// One append-only change-rule violation between a base register and a current register.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangeRuleViolation {
    pub rule: &'static str,
    pub row_id: String,
    pub message: String,
}

impl fmt::Display for ChangeRuleViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} [{}]: {}", self.rule, self.row_id, self.message)
    }
}

impl std::error::Error for ChangeRuleViolation {}

/// Verify the append-only change rule between `base` and `current` (`N00-10` step 2).
#[must_use]
pub fn check_change_rule(
    base: &HypothesisRegister,
    current: &HypothesisRegister,
) -> Vec<ChangeRuleViolation> {
    let mut violations = Vec::new();

    for base_row in &base.hypotheses {
        let Some(cur_row) = current.get(&base_row.id) else {
            violations.push(ChangeRuleViolation {
                rule: RULE_ROW_REMOVED,
                row_id: base_row.id.clone(),
                message: "hypothesis row removed (register is append-only)".to_owned(),
            });
            continue;
        };

        // 1. Existing history entries are strictly immutable and cannot shrink.
        if cur_row.history.len() < base_row.history.len()
            || cur_row.history[..base_row.history.len()] != base_row.history[..]
        {
            violations.push(ChangeRuleViolation {
                rule: RULE_HISTORY_EDITED,
                row_id: base_row.id.clone(),
                message: "existing history entry was edited or removed (history is append-only)"
                    .to_owned(),
            });
            continue;
        }

        // 2. State must match the last history entry.
        if let Some(last) = cur_row.history.last()
            && last.state != cur_row.state
        {
            violations.push(ChangeRuleViolation {
                rule: RULE_STATE_HISTORY_MISMATCH,
                row_id: cur_row.id.clone(),
                message: format!(
                    "row state `{}` disagrees with last history state `{}`",
                    cur_row.state, last.state
                ),
            });
        }

        let new_entries = &cur_row.history[base_row.history.len()..];

        // 3. Changing state requires at least one new history entry.
        if cur_row.state != base_row.state && new_entries.is_empty() {
            violations.push(ChangeRuleViolation {
                rule: RULE_STATE_CHANGE_WITHOUT_ENTRY,
                row_id: cur_row.id.clone(),
                message: format!(
                    "state changed from `{}` to `{}` with no new history entry",
                    base_row.state, cur_row.state
                ),
            });
        }

        // 4. Every new history entry must cite a ledger, gate, or source-register record.
        for (offset, entry) in new_entries.iter().enumerate() {
            let cites_record = entry
                .record
                .as_deref()
                .is_some_and(|s| !s.trim().is_empty());
            let cites_owner_only_for_reversal = entry.owner_decision.is_some()
                && base_row.reversal.is_none()
                && cur_row.reversal.is_some()
                && entry.state == VerdictState::Refuted;
            if !cites_record && !cites_owner_only_for_reversal {
                violations.push(ChangeRuleViolation {
                    rule: RULE_NEW_ENTRY_MISSING_RECORD,
                    row_id: cur_row.id.clone(),
                    message: format!(
                        "new history entry {} (kind `{}`) must cite a ledger, gate, or source-register `record`",
                        base_row.history.len() + offset,
                        entry.kind
                    ),
                });
            }
        }

        // 5. `reversal` immutability and owner-decision adoption rules.
        match (&base_row.reversal, &cur_row.reversal) {
            (Some(old_rev), Some(new_rev)) if old_rev != new_rev => {
                violations.push(ChangeRuleViolation {
                    rule: RULE_REVERSAL_EDITED,
                    row_id: cur_row.id.clone(),
                    message: "recorded `reversal` condition cannot be edited once set".to_owned(),
                });
            }
            (Some(_), None) => {
                violations.push(ChangeRuleViolation {
                    rule: RULE_REVERSAL_EDITED,
                    row_id: cur_row.id.clone(),
                    message: "recorded `reversal` condition cannot be removed once set".to_owned(),
                });
            }
            (None, Some(_)) => {
                let has_owner_decision = cur_row
                    .reversal_decision
                    .as_deref()
                    .is_some_and(|s| !s.trim().is_empty())
                    || new_entries.iter().any(|e| {
                        e.owner_decision
                            .as_deref()
                            .is_some_and(|s| !s.trim().is_empty())
                    });
                if !has_owner_decision {
                    violations.push(ChangeRuleViolation {
                        rule: RULE_REVERSAL_WITHOUT_OWNER_DECISION,
                        row_id: cur_row.id.clone(),
                        message: "setting `reversal` from absent requires citing an owner decision record (`reversal_decision` or `owner_decision`)"
                            .to_owned(),
                    });
                }
            }
            (None, None) => {}
            _ => {}
        }

        // 6. REFUTED row movement rules.
        if base_row.state == VerdictState::Refuted {
            if base_row.reversal.is_none() && cur_row.reversal.is_none() {
                // Proposal-only row: cannot move at all, and any appended history entry attempting
                // to move or cite reversal on a proposal-only row fails closed.
                if cur_row.state != VerdictState::Refuted || !new_entries.is_empty() {
                    violations.push(ChangeRuleViolation {
                        rule: RULE_PROPOSAL_ONLY_CANNOT_MOVE,
                        row_id: cur_row.id.clone(),
                        message: "REFUTED row holding only `reversal_proposed` cannot move until an owner decision records `reversal`"
                            .to_owned(),
                    });
                }
            } else if cur_row.state != VerdictState::Refuted {
                // Recorded reversal present (either before or adopted via owner decision):
                // moving away from REFUTED requires a new entry citing reversal_met = true + record.
                let has_reversal_entry = new_entries.iter().any(|e| {
                    e.reversal_met && e.record.as_deref().is_some_and(|s| !s.trim().is_empty())
                });
                if !has_reversal_entry {
                    violations.push(ChangeRuleViolation {
                        rule: RULE_REFUTED_MOVE_WITHOUT_REVERSAL,
                        row_id: cur_row.id.clone(),
                        message: format!(
                            "REFUTED row moved to `{}` without a new history entry setting `reversal_met = true` with a cited `record`",
                            cur_row.state
                        ),
                    });
                }
            }
        }
    }

    for base_dis in &base.disagreements {
        if !current.disagreements.iter().any(|d| d.id == base_dis.id) {
            violations.push(ChangeRuleViolation {
                rule: RULE_DISAGREEMENT_REMOVED,
                row_id: base_dis.id.clone(),
                message: "disagreement record removed (register is append-only)".to_owned(),
            });
        }
    }

    violations
}

/// Load the comparison base register from git per `N00-10` step 2 (`HEAD` if the working
/// tree differs from `HEAD`, otherwise `HEAD~1`). Returns `Ok(None)` if the file did not
/// exist at the base commit (initial introduction of the register).
pub fn load_base_from_git(
    repo_root: &Path,
    rel_path: &str,
) -> Result<Option<HypothesisRegister>, RegisterError> {
    let status = Command::new("git")
        .current_dir(repo_root)
        .args(["status", "--porcelain", "--", rel_path])
        .output()
        .map_err(|e| RegisterError::Io(format!("git status failed: {e}")))?;
    if !status.status.success() {
        return Ok(None);
    }
    let dirty = !String::from_utf8_lossy(&status.stdout).trim().is_empty();
    let base_rev = if dirty { "HEAD" } else { "HEAD~1" };
    let spec = format!("{base_rev}:{rel_path}");

    let show = Command::new("git")
        .current_dir(repo_root)
        .args(["show", &spec])
        .output()
        .map_err(|e| RegisterError::Io(format!("git show {spec} failed: {e}")))?;
    if !show.status.success() {
        return Ok(None);
    }
    Ok(Some(HypothesisRegister::from_bytes(&show.stdout)?))
}
