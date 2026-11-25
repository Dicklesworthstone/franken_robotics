#![forbid(unsafe_code)]

//! `stats` airlock group (`N33a-11`, off by default).
//!
//! Trait declarations fix intent only; adapter bodies land with their owning
//! admission beads (`N33b-12`, `N33b-13`). No volatile FrankenSim types appear
//! in any public signature.

use crate::groups::{Group, GroupAbsent};

/// The airlock group represented by this module.
pub const GROUP: Group = Group::Stats;

/// Whether the `stats` feature was enabled at compile time.
pub const ENABLED: bool = cfg!(feature = "stats");

/// Sampling, multilevel estimation, importance weights, and sensitivity effects.
pub trait Uncertainty {
    /// Estimator family identifier.
    fn estimator_name(&self) -> &'static str;
}

/// Ranking-flip probability and expected opportunity loss.
pub trait ValueOfInformation {
    /// Number of candidate arms evaluated.
    fn candidate_count(&self) -> usize;
}

/// Reduced models admitted inside validity bands.
pub trait Surrogate {
    /// Surrogate family identifier.
    fn surrogate_name(&self) -> &'static str;
}

/// Checked belief updates from observations.
pub trait Assimilation {
    /// State dimension tracked by the belief update.
    fn state_dim(&self) -> usize;
}

/// Check whether the `stats` group is compiled into this binary.
pub fn available() -> Result<(), GroupAbsent> {
    if ENABLED {
        Ok(())
    } else {
        Err(GroupAbsent::new(GROUP))
    }
}

#[cfg(not(feature = "stats"))]
/// Stub constructor when the `stats` group is not compiled in.
pub fn new_uncertainty() -> Result<(), GroupAbsent> {
    Err(GroupAbsent::new(GROUP))
}

#[cfg(not(feature = "stats"))]
/// Stub constructor when the `stats` group is not compiled in.
pub fn new_value_of_information() -> Result<(), GroupAbsent> {
    Err(GroupAbsent::new(GROUP))
}

#[cfg(not(feature = "stats"))]
/// Stub constructor when the `stats` group is not compiled in.
pub fn new_surrogate() -> Result<(), GroupAbsent> {
    Err(GroupAbsent::new(GROUP))
}

#[cfg(not(feature = "stats"))]
/// Stub constructor when the `stats` group is not compiled in.
pub fn new_assimilation() -> Result<(), GroupAbsent> {
    Err(GroupAbsent::new(GROUP))
}
