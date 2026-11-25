#![forbid(unsafe_code)]

//! `optimize` airlock group and M1 `optimize-race` sub-feature (`N33a-11`,
//! Review Note 1; off by default).
//!
//! Trait declarations fix intent only; adapter bodies land with their owning
//! admission beads (`N33a-18`, `N33a-31`). No volatile FrankenSim types appear
//! in any public signature.

use crate::groups::{Group, GroupAbsent};

/// The airlock group represented by this module.
pub const GROUP: Group = Group::Optimize;

/// Whether the full `optimize` feature was enabled at compile time.
pub const ENABLED: bool = cfg!(feature = "optimize");

/// Whether the M1 `optimize-race` sub-feature (`Racing`) was enabled at compile time.
pub const RACING_ENABLED: bool = cfg!(feature = "optimize-race");

/// Derivative-free and Bayesian search.
pub trait Optimizer {
    /// Optimizer family identifier.
    fn optimizer_name(&self) -> &'static str;
}

/// Quality-diversity archives.
pub trait Archive {
    /// Number of occupied archive niches.
    fn occupied_cells(&self) -> usize;
}

/// Anytime-valid races and e-processes.
pub trait Racing {
    /// Number of active candidates remaining in the race.
    fn active_candidates(&self) -> usize;
}

/// Check whether the full `optimize` group is compiled into this binary.
pub fn available() -> Result<(), GroupAbsent> {
    if ENABLED {
        Ok(())
    } else {
        Err(GroupAbsent::new(GROUP))
    }
}

/// Check whether the M1 `optimize-race` sub-feature (or full `optimize`) is compiled in.
pub fn racing_available() -> Result<(), GroupAbsent> {
    if RACING_ENABLED || ENABLED {
        Ok(())
    } else {
        Err(GroupAbsent::new(GROUP))
    }
}

#[cfg(not(feature = "optimize"))]
/// Stub constructor when the full `optimize` group is not compiled in.
pub fn new_optimizer() -> Result<(), GroupAbsent> {
    Err(GroupAbsent::new(GROUP))
}

#[cfg(not(feature = "optimize"))]
/// Stub constructor when the full `optimize` group is not compiled in.
pub fn new_archive() -> Result<(), GroupAbsent> {
    Err(GroupAbsent::new(GROUP))
}

#[cfg(not(any(feature = "optimize", feature = "optimize-race")))]
/// Stub constructor when neither `optimize` nor `optimize-race` is compiled in.
pub fn new_racing() -> Result<(), GroupAbsent> {
    Err(GroupAbsent::new(GROUP))
}
