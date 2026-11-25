#![forbid(unsafe_code)]

//! `ledger` airlock group (`N33a-11`, default on).
//!
//! Trait declarations fix intent only; adapter bodies land with their owning
//! admission beads (`BD-009`, `N33a-17`, `N33b-15`). No volatile FrankenSim
//! types appear in any public signature.

use crate::groups::{Group, GroupAbsent};

/// The airlock group represented by this module.
pub const GROUP: Group = Group::Ledger;

/// Whether the `ledger` feature was enabled at compile time.
pub const ENABLED: bool = cfg!(feature = "ledger");

/// The transactional metadata store on FrankenSQLite.
pub trait Ledger {
    /// Schema identifier of this ledger adapter.
    fn ledger_schema(&self) -> &'static str;
}

/// Packaged, replayable sessions.
pub trait Session {
    /// Canonical session package version string.
    fn session_version(&self) -> &'static str;
}

/// A content-addressed graph of results with skip certificates and invalidation.
pub trait Recompute {
    /// Number of nodes tracked in the recompute graph.
    fn node_count(&self) -> usize;
}

/// Check whether the `ledger` group is compiled into this binary.
pub fn available() -> Result<(), GroupAbsent> {
    if ENABLED {
        Ok(())
    } else {
        Err(GroupAbsent::new(GROUP))
    }
}

#[cfg(not(feature = "ledger"))]
/// Stub constructor when the `ledger` group is not compiled in.
pub fn new_ledger() -> Result<(), GroupAbsent> {
    Err(GroupAbsent::new(GROUP))
}

#[cfg(not(feature = "ledger"))]
/// Stub constructor when the `ledger` group is not compiled in.
pub fn new_session() -> Result<(), GroupAbsent> {
    Err(GroupAbsent::new(GROUP))
}

#[cfg(not(feature = "ledger"))]
/// Stub constructor when the `ledger` group is not compiled in.
pub fn new_recompute() -> Result<(), GroupAbsent> {
    Err(GroupAbsent::new(GROUP))
}
