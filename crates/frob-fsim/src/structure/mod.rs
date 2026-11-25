#![forbid(unsafe_code)]

//! `structure` airlock group (`N33a-11`, off by default).
//!
//! Trait declarations fix intent only; adapter bodies land with `N33b-14`.
//! No volatile FrankenSim types appear in any public signature.

use crate::groups::{Group, GroupAbsent};

/// The airlock group represented by this module.
pub const GROUP: Group = Group::Structure;

/// Whether the `structure` feature was enabled at compile time.
pub const ENABLED: bool = cfg!(feature = "structure");

/// Structural topology and shape optimization.
pub trait StructuralOpt {
    /// Structural optimization formulation identifier.
    fn formulation_name(&self) -> &'static str;
}

/// Tolerance allocation by variance propagation.
pub trait ToleranceAllocation {
    /// Number of dimensional tolerances in the stack.
    fn tolerance_count(&self) -> usize;
}

/// Registration of as-built scan data against a nominal design.
pub trait AsBuilt {
    /// Registration method identifier.
    fn registration_name(&self) -> &'static str;
}

/// Fabrication constraints and manufacturability checks.
pub trait FabricationRules {
    /// Number of active fabrication rules evaluated.
    fn rule_count(&self) -> usize;
}

/// Check whether the `structure` group is compiled into this binary.
pub fn available() -> Result<(), GroupAbsent> {
    if ENABLED {
        Ok(())
    } else {
        Err(GroupAbsent::new(GROUP))
    }
}

#[cfg(not(feature = "structure"))]
/// Stub constructor when the `structure` group is not compiled in.
pub fn new_structural_opt() -> Result<(), GroupAbsent> {
    Err(GroupAbsent::new(GROUP))
}

#[cfg(not(feature = "structure"))]
/// Stub constructor when the `structure` group is not compiled in.
pub fn new_tolerance_allocation() -> Result<(), GroupAbsent> {
    Err(GroupAbsent::new(GROUP))
}

#[cfg(not(feature = "structure"))]
/// Stub constructor when the `structure` group is not compiled in.
pub fn new_as_built() -> Result<(), GroupAbsent> {
    Err(GroupAbsent::new(GROUP))
}

#[cfg(not(feature = "structure"))]
/// Stub constructor when the `structure` group is not compiled in.
pub fn new_fabrication_rules() -> Result<(), GroupAbsent> {
    Err(GroupAbsent::new(GROUP))
}
