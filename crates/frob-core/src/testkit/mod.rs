//! Shared unit-test support kit for FrankenRobotics (`STD-TEST`).
//!
//! Built only when feature `test-support` is enabled (dev-dependencies of
//! workspace crates, and as a normal dependency of `frob-conformance` only).
//! Uses the Rust standard library and `frob-core` (`registry::toml`) only.

pub mod det;
pub mod evidence;
pub mod gate;
pub mod prop;
pub mod registry;

pub use det::{CheckpointDigest, D3DecisionResult, DetClass, DigestDiff, Lane, LaneResult, LaneRun};
pub use evidence::{EvidenceKind, EvidenceRecord, EvidenceStatus};
pub use gate::{CmpOp, GateLine, GateOutcome, GateReport, Measurement, NotEligibleReason, Threshold};
pub use registry::{
    CompareKind, Control, ControlCell, ControlKind, ControlObservation, ControlOutcome,
    ControlState, Controls, Counterexample, Counterexamples, KaCheckResult, KaStatus,
    KnownAnswers, LastSeen, ObligationReason, Observed, PlanCase, Plant, RegistryError,
    RelationMutation, RelationRow, Relations, Row, Safe, Tolerance, Truth, assert_ka,
    assert_ka_with_evidence, control, control_expect,
};
