#![deny(unsafe_code)]

//! Milestone-exit suspect-control review (`N38b-04`, Rule 4).
//!
//! Run as:
//! `FROB_MILESTONE=<Mn> cargo test -p frob-conformance --test control_review`
//!
//! The milestone under review is ALWAYS read from `FROB_MILESTONE` (defaulting to
//! the current milestone in `registries/gates.toml` when unset, never "no controls"),
//! never from a cargo test-name filter after `--`.
//!
//! At a milestone exit (`FROB_MILESTONE_EXIT=1`, any scratch override via
//! `FROB_CONFORMANCE_ROOT`/`FROB_REGISTRY_ROOT`, or whenever any due gate has already
//! recorded `status = "pass"`), an undisposed suspect control fails `GATE-GAUNT-01`
//! with `reason=suspect-undisposed`.

use std::collections::BTreeSet;
use std::env;

use frob_conformance::control_runner::{
    CONTROL_REVIEWS_PATH, ControlReviews, build_suspect_report, load_history, load_reviews,
    resolve_review_milestone,
};
use frob_conformance::gates_v0::{GatePartStatus, GateRegistry};
use frob_conformance::reasons::{HarnessReason, workspace_root};
use frob_conformance::tiers::Milestone;
use frob_core::testkit::registry::Controls;

#[test]
fn control_review_workspace() {
    let root = workspace_root();
    let gates = GateRegistry::load(&root).expect("load registries/gates.toml");
    let (controls, _) = Controls::load(&root).expect("load registry/controls.toml");
    let history = load_history(&root).expect("load registry/control_history.ndjson");
    let reviews = load_reviews(&root).expect("load registry/control_reviews.toml");

    let milestone = resolve_review_milestone(None, &gates)
        .expect("resolve review milestone from FROB_MILESTONE or gates.toml");

    let report = build_suspect_report(
        milestone,
        &gates,
        &controls,
        &history,
        &reviews,
        &BTreeSet::new(),
    );
    print!("{}", report.format_report());

    assert!(
        !report.due_controls.is_empty(),
        "control_review at {milestone} must select due controls, never zero"
    );

    let exit_enforced = env::var("FROB_MILESTONE_EXIT").is_ok_and(|v| v == "1" || v == "true")
        || env::var_os("FROB_CONFORMANCE_ROOT").is_some()
        || env::var_os("FROB_REGISTRY_ROOT").is_some()
        || gates.all().iter().any(|g| {
            report.due_gates.contains(&g.id)
                && g.effective_parts()
                    .iter()
                    .any(|p| p.status == GatePartStatus::Pass)
        });

    if exit_enforced && report.blocks_exit() {
        panic!(
            "reason={}: milestone {milestone} exit blocked by {} undisposed suspect control(s):\n{}",
            HarnessReason::SuspectUndisposed.as_str(),
            report.undisposed.len(),
            report.format_report()
        );
    }
}

#[test]
fn review_rejects_invalid_disposition() {
    let bad = r#"schema = "frob.gauntlet.control_reviews/1"

[[review]]
id = "NC-0001"
disposition = "ignored-forever"
reason = "not a valid disposition"
"#;
    assert!(ControlReviews::parse_str(bad, CONTROL_REVIEWS_PATH).is_err());

    let no_reason = r#"schema = "frob.gauntlet.control_reviews/1"

[[review]]
id = "NC-0001"
disposition = "withdrawn"
reason = "   "
"#;
    assert!(ControlReviews::parse_str(no_reason, CONTROL_REVIEWS_PATH).is_err());
}

#[test]
fn review_milestone_env_never_empty() {
    let root = workspace_root();
    let gates = GateRegistry::load(&root).expect("load registries/gates.toml");
    let m0 =
        resolve_review_milestone(Some(""), &gates).expect("unset defaults to current milestone");
    let m1 = resolve_review_milestone(Some("M1"), &gates).expect("explicit M1");
    assert_eq!(m0, Milestone::M0);
    assert_eq!(m1, Milestone::M1);
}
