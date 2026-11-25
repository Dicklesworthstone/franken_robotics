//! Portable IEEE-754 basic-operations normal CDF Φ and inverse CDF Φ⁻¹
//! (skeleton in `N39a-01`; full polynomial approximations land in their owning
//! bead). Must NEVER call `f64::ln`, `exp`, `powf`, or any platform math lib.
//!
//! Lives inside the scorer boundary (`BOUNDARY_FILES`).

pub const MODULE_OWNER: &str = "N39a-28";
