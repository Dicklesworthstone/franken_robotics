#![forbid(unsafe_code)]

//! Absent airlock feature-group refusal (`FROB-REF-790`, `N33a-11`, §33.16).
//!
//! Raises `FROB-REF-790` (`foundation-group-absent`, exit class 5,
//! `retry: after-change`) before any work or budget is charged when a caller
//! requests an airlock feature group that was not compiled into this binary.
//! Ranked repairs live in `registries/refusals.toml` (`N33-REF`) and are
//! attached by the surface envelope builder (`N33c-32`).

use frob_core::refusal::{Refusal, RefusalCode, RetryPolicy};

use crate::groups::{BUILT_WITH, Group, built_with_names};

/// Canonical refusal code for an absent airlock feature group (`FROB-REF-790`).
pub const REFUSAL_CODE_GROUP_ABSENT: RefusalCode = RefusalCode::FrobRef790;

/// CLI exit class for `FROB-REF-790` (capability not implemented or not admitted).
pub const EXIT_CLASS_GROUP_ABSENT: u8 = 5;

/// Construct the `FROB-REF-790` refusal value for a missing compiled-in group.
///
/// Sets `pointer = "foundation/<group>/compiled"`, `observed` to the compiled
/// `BUILT_WITH` set, `required` to the missing group name, and
/// `retry = RetryPolicy::AfterChange`. Ranked repairs are not restated here;
/// `N33c-32` attaches them from `registries/refusals.toml`.
#[must_use]
pub fn group_absent_refusal(missing: Group) -> Refusal {
    let message = match missing {
        Group::Structure => format!(
            "airlock feature group `{}` is not compiled into this binary (enabling adds ~13 crates / ~412k lines beyond M1)",
            missing.as_str()
        ),
        Group::Multiphysics => format!(
            "airlock feature group `{}` is not compiled into this binary (enabling adds ~19 crates / ~2.31M lines beyond M1)",
            missing.as_str()
        ),
        _ => format!(
            "airlock feature group `{}` is not compiled into this binary",
            missing.as_str()
        ),
    };
    Refusal::new(
        REFUSAL_CODE_GROUP_ABSENT,
        message,
        missing.compiled_pointer(),
        built_with_names(),
        missing.as_str(),
        "airlock-group",
        RetryPolicy::AfterChange,
        Vec::new(),
    )
}

/// Verify that every group in `groups` was compiled into this binary.
///
/// Evaluates `groups` strictly in declared order and returns `FROB-REF-790` on
/// the first absent group without allocating work or charging any budget.
pub fn require(groups: &[Group]) -> Result<(), Refusal> {
    for &group in groups {
        if !BUILT_WITH.contains(&group) {
            return Err(group_absent_refusal(group));
        }
    }
    Ok(())
}
