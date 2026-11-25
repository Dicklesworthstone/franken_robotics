//! Plain proof objects for mission answers (`N25b-01`, `OPEN-AUGUR-6`, `ADR-WS-02`).
//!
//! Neutral layer-0 data vocabulary shared between proposers (`frob-mission`, layer 5) and
//! certificate writers (`frob-lemma`, layer 6) without linking the auditor (`frob-cert`).
//! Data only: no solver, no checking rule, no `.frobcert` container serialization,
//! zero floating-point fields, zero `unsafe`, determinism class D0.

#[cfg(all(
    any(
        feature = "nc-torque-alias-first-moment",
        feature = "nc-allow-empty-support",
    ),
    any(not(feature = "gauntlet-mutations"), not(debug_assertions))
))]
compile_error!(
    "frob-core proof negative-control features require debug builds with --features gauntlet-mutations"
);

pub mod establishes;
pub mod expr;
pub mod kinds;
pub mod num;
pub mod object;
pub mod payload;
pub mod qty_types;
pub mod support;

pub use establishes::{
    Binder, Bound, BoundEnd, Cell, CellRef, ChartId, Establishes, Horizon, ModeId,
    PolicyClassDigest, PolicyScope, PopulationId, ProofRef, TopologyId, Var,
};
pub use expr::{Equation, ExprBuilder, ExprDag, ExprNode, NodeId};
pub use kinds::{CellPolarity, Expected, ProofKind};
pub use num::{CanonicalEncoder, Interval, ProofError, Ratio};
pub use object::ProofObject;
pub use payload::{
    AliasPairPayload, AliasScope, BoxOutcome, CellEndForces, Comparator, CompositionPayload,
    ConicFarkasPayload, DualVector, FiniteGamePayload, IntervalBoxPayload, LedgerEntry,
    LoadVertexWitness, LpFarkasPayload, LpLaw, LpPrimalPayload, MonotoneTransportPayload,
    PartitionCell, PartitionPayload, ProductUnreachablePayload, ProofPayload,
    SampleCoveragePayload, SosGramPayload, SosMultiplier, StageLink, SymmetryTransportPayload,
    TaylorBoxPayload, TraceWitnessPayload, vertex_force_values,
};
pub use qty_types::{
    AxisConvention, ExactQty, FirstMoment, FirstMomentTag, FrameSpec, Handedness, OrderedPorts,
    PortForceVector, PortId, SignConvention, TorqueTag, TorqueVec, TypedMoment3, Unit,
};
pub use support::{Assumption, AssumptionValue, Axis, FieldPath, ImplicitDep, Polarity, Support};

// Compile-time assertion (N25b-01 AC 2): neither `f32` nor `f64` implements `Eq + Ord + Hash`,
// so requiring these bounds recursively over `ProofObject`, `ProofPayload`, and every supporting
// type guarantees at compile time that no floating-point field exists anywhere in the module.
const fn assert_no_float_types<T: Eq + Ord + std::hash::Hash>() {}

const _: () = {
    assert_no_float_types::<ProofObject>();
    assert_no_float_types::<ProofPayload>();
    assert_no_float_types::<Establishes>();
    assert_no_float_types::<ExprDag>();
    assert_no_float_types::<Equation>();
    assert_no_float_types::<Support>();
    assert_no_float_types::<Assumption>();
    assert_no_float_types::<Ratio>();
    assert_no_float_types::<Interval>();
    assert_no_float_types::<FirstMoment>();
    assert_no_float_types::<TorqueVec>();
    assert_no_float_types::<PortForceVector>();
};
