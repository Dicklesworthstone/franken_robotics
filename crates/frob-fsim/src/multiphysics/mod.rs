#![forbid(unsafe_code)]

//! `multiphysics` airlock group (`N33a-11`, off by default; gated bridge
//! compiled only by M6 evaluation-tier lanes).
//!
//! Trait declarations fix intent only; adapter bodies land with `N33b-16`.
//! No volatile FrankenSim types appear in any public signature.

use crate::groups::{Group, GroupAbsent};

/// The airlock group represented by this module.
pub const GROUP: Group = Group::Multiphysics;

/// Whether the `multiphysics` feature was enabled at compile time.
pub const ENABLED: bool = cfg!(feature = "multiphysics");

/// Whole-field fluid island exchange with resolved matter at declared instants, evaluation tier only.
pub trait FluidIsland {
    /// Fluid solver lattice identifier.
    fn lattice_name(&self) -> &'static str;
}

/// Coupled multiphysics island exchange with resolved matter at declared instants, evaluation tier only.
pub trait CoupledIsland {
    /// Number of coupled physical fields exchanged across the island boundary.
    fn coupled_field_count(&self) -> usize;
}

/// Check whether the `multiphysics` group is compiled into this binary.
pub fn available() -> Result<(), GroupAbsent> {
    if ENABLED {
        Ok(())
    } else {
        Err(GroupAbsent::new(GROUP))
    }
}

#[cfg(not(feature = "multiphysics"))]
/// Stub constructor when the `multiphysics` group is not compiled in.
pub fn new_fluid_island() -> Result<(), GroupAbsent> {
    Err(GroupAbsent::new(GROUP))
}

#[cfg(not(feature = "multiphysics"))]
/// Stub constructor when the `multiphysics` group is not compiled in.
pub fn new_coupled_island() -> Result<(), GroupAbsent> {
    Err(GroupAbsent::new(GROUP))
}
