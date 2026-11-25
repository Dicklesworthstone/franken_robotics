#![forbid(unsafe_code)]

//! `geometry` airlock group (`N33a-11`, default on).
//!
//! Trait declarations fix intent only; adapter bodies land with their owning
//! admission beads (`N33a-16`, `N33a-30`). No volatile FrankenSim types appear
//! in any public signature.

use crate::groups::{Group, GroupAbsent};

/// The airlock group represented by this module.
pub const GROUP: Group = Group::Geometry;

/// Whether the `geometry` feature was enabled at compile time.
pub const ENABLED: bool = cfg!(feature = "geometry");

/// Signed distance, convex support, and separation queries.
pub trait GeometryKernel {
    /// Kernel family identifier.
    fn kernel_name(&self) -> &'static str;
}

/// Role-tagged support and keep-out regions of referee scenes.
pub trait SceneQuery {
    /// Number of registered scene regions.
    fn region_count(&self) -> usize;
}

/// Hardened mesh read and write with content hashes.
pub trait MeshExchange {
    /// Supported exchange format identifier.
    fn format_name(&self) -> &'static str;
}

/// Check whether the `geometry` group is compiled into this binary.
pub fn available() -> Result<(), GroupAbsent> {
    if ENABLED {
        Ok(())
    } else {
        Err(GroupAbsent::new(GROUP))
    }
}

#[cfg(not(feature = "geometry"))]
/// Stub constructor when the `geometry` group is not compiled in.
pub fn new_geometry_kernel() -> Result<(), GroupAbsent> {
    Err(GroupAbsent::new(GROUP))
}

#[cfg(not(feature = "geometry"))]
/// Stub constructor when the `geometry` group is not compiled in.
pub fn new_scene_query() -> Result<(), GroupAbsent> {
    Err(GroupAbsent::new(GROUP))
}

#[cfg(not(feature = "geometry"))]
/// Stub constructor when the `geometry` group is not compiled in.
pub fn new_mesh_exchange() -> Result<(), GroupAbsent> {
    Err(GroupAbsent::new(GROUP))
}
