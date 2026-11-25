#![forbid(unsafe_code)]

//! `actuation` airlock group (`N33a-11`, off by default).
//!
//! Trait declarations fix intent only; adapter bodies land with `N33a-19` and
//! `BD-068`. No volatile FrankenSim types appear in any public signature.

use crate::groups::{Group, GroupAbsent};

/// The airlock group represented by this module.
pub const GROUP: Group = Group::Actuation;

/// Whether the `actuation` feature was enabled at compile time.
pub const ENABLED: bool = cfg!(feature = "actuation");

/// Electromagnetic actuator physics at the fidelity levels of the actuator subsystem.
pub trait Magnetics {
    /// Magnetic model fidelity identifier.
    fn model_name(&self) -> &'static str;
}

/// Thermal network physics at the fidelity levels of the actuator subsystem.
pub trait ThermalNetwork {
    /// Number of lumped thermal nodes in the network.
    fn thermal_node_count(&self) -> usize;
}

/// Current-loop dynamics at the fidelity levels of the actuator subsystem.
pub trait CurrentLoop {
    /// Number of electrical phases modeled.
    fn phase_count(&self) -> usize;
}

/// Port-Hamiltonian actuator composition at the fidelity levels of the actuator subsystem.
pub trait PortHamiltonian {
    /// Number of storage elements in the port-Hamiltonian network.
    fn storage_count(&self) -> usize;
}

/// Check whether the `actuation` group is compiled into this binary.
pub fn available() -> Result<(), GroupAbsent> {
    if ENABLED {
        Ok(())
    } else {
        Err(GroupAbsent::new(GROUP))
    }
}

#[cfg(not(feature = "actuation"))]
/// Stub constructor when the `actuation` group is not compiled in.
pub fn new_magnetics() -> Result<(), GroupAbsent> {
    Err(GroupAbsent::new(GROUP))
}

#[cfg(not(feature = "actuation"))]
/// Stub constructor when the `actuation` group is not compiled in.
pub fn new_thermal_network() -> Result<(), GroupAbsent> {
    Err(GroupAbsent::new(GROUP))
}

#[cfg(not(feature = "actuation"))]
/// Stub constructor when the `actuation` group is not compiled in.
pub fn new_current_loop() -> Result<(), GroupAbsent> {
    Err(GroupAbsent::new(GROUP))
}

#[cfg(not(feature = "actuation"))]
/// Stub constructor when the `actuation` group is not compiled in.
pub fn new_port_hamiltonian() -> Result<(), GroupAbsent> {
    Err(GroupAbsent::new(GROUP))
}
