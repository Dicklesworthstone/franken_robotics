//! Mathematics compendium exact oracle library and registry validator (`N12-41`).
//!
//! Independent, exact reference implementations for Appendix D formulas live here
//! inside `frob-conformance` (layer 7); no production crate may depend on this module,
//! and no oracle value is regenerated from candidate production code.

pub mod d1;
pub mod d3;
pub mod rat;
pub mod registry;
