#![forbid(unsafe_code)]

//! `spectral` airlock group (`N33a-11`, off by default).
//!
//! Trait declarations fix intent only; adapter body lands with `N33b-11`.
//! No volatile FrankenSim types appear in any public signature.

use crate::groups::{Group, GroupAbsent};

/// The airlock group represented by this module.
pub const GROUP: Group = Group::Spectral;

/// Whether the `spectral` feature was enabled at compile time.
pub const ENABLED: bool = cfg!(feature = "spectral");

/// Spectral admission and Floquet analysis of a supplied monodromy.
pub trait Spectral {
    /// Dimension of the analyzed monodromy operator.
    fn monodromy_dim(&self) -> usize;
}

/// Check whether the `spectral` group is compiled into this binary.
pub fn available() -> Result<(), GroupAbsent> {
    if ENABLED {
        Ok(())
    } else {
        Err(GroupAbsent::new(GROUP))
    }
}

#[cfg(not(feature = "spectral"))]
/// Stub constructor when the `spectral` group is not compiled in.
pub fn new_spectral() -> Result<(), GroupAbsent> {
    Err(GroupAbsent::new(GROUP))
}
