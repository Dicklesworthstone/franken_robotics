#![forbid(unsafe_code)]

//! `referee` airlock group (`N33a-11`, default on).
//!
//! Trait declarations fix intent only; adapter bodies land with their owning
//! admission beads (`BD-007`, `N33a-14`, `N33a-15`). No volatile FrankenSim
//! types appear in any public signature.

use crate::groups::{Group, GroupAbsent};

/// The airlock group represented by this module.
pub const GROUP: Group = Group::Referee;

/// Whether the `referee` feature was enabled at compile time.
pub const ENABLED: bool = cfg!(feature = "referee");

/// Build an articulated model and evaluate forward dynamics and steps in `f64`.
pub trait RefereeDynamics {
    /// Human-readable adapter label.
    fn adapter_name(&self) -> &'static str;
}

/// A contact step over a caller-supplied contact set, with its capacity as data.
pub trait RefereeContact {
    /// Maximum contact rows supported by this adapter.
    fn contact_capacity(&self) -> usize;
}

/// Structure-preserving steps on groups and swept motion charts.
pub trait Integrator {
    /// Integrator family identifier.
    fn integrator_name(&self) -> &'static str;
}

/// Composition of components through typed effort and flow ports.
pub trait PortSystem {
    /// Number of declared typed ports on this system.
    fn port_count(&self) -> usize;
}

/// Check whether the `referee` group is compiled into this binary.
pub fn available() -> Result<(), GroupAbsent> {
    if ENABLED {
        Ok(())
    } else {
        Err(GroupAbsent::new(GROUP))
    }
}

#[cfg(not(feature = "referee"))]
/// Stub constructor when the `referee` group is not compiled in.
pub fn new_referee_dynamics() -> Result<(), GroupAbsent> {
    Err(GroupAbsent::new(GROUP))
}

#[cfg(not(feature = "referee"))]
/// Stub constructor when the `referee` group is not compiled in.
pub fn new_referee_contact() -> Result<(), GroupAbsent> {
    Err(GroupAbsent::new(GROUP))
}

#[cfg(not(feature = "referee"))]
/// Stub constructor when the `referee` group is not compiled in.
pub fn new_integrator() -> Result<(), GroupAbsent> {
    Err(GroupAbsent::new(GROUP))
}

#[cfg(not(feature = "referee"))]
/// Stub constructor when the `referee` group is not compiled in.
pub fn new_port_system() -> Result<(), GroupAbsent> {
    Err(GroupAbsent::new(GROUP))
}
