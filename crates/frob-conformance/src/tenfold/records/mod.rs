//! The five ten-fold harness record types (bead `N39a-01`).
//!
//! Each record module lives inside the scorer boundary (`BOUNDARY_FILES`) and
//! imports strictly `std`, `frob_cert`, and listed boundary modules. Detailed
//! domain fields are extended by their owning beads (`N39a-05`, `N39a-18`,
//! `N39a-24`, `N39a-09`, `N39a-36`).

pub mod freeze;
pub mod ledger;
pub mod mission;
pub mod receipt;
pub mod report;
