#![forbid(unsafe_code)]

//! Marrow Lie-theoretic vocabulary: `LieGroup` and `LieAlgebra` traits,
//! `SO(3)` and `SE(3)` bound directly onto `fs-ga` (ADR-MARROW-01),
//! the infinitesimal coadjoint `coad` (ADR-MARROW-05), and generic `fs_ad::Real`
//! reference kernels (`BD-010`).
//!
//! # Mutation guard (N38b-04)
//!
//! Negative-control mutation features (`twist-swap`, `ad-inverse`,
//! `bracket-swap`, `coadjoint-inverse`) imply `conformance`, are never enabled
//! by default, and fail compilation under release builds (`not(debug_assertions)`).

#[cfg(all(feature = "conformance", not(debug_assertions)))]
compile_error!(
    "frob-lie: feature `conformance` (and its negative-control mutation features) is forbidden under release builds (N38b-04)."
);

pub mod reference;
pub mod se3;
pub mod so3;
pub mod traits;

// ADR-MARROW-01: re-export `fs-ga` host representation directly; define no
// duplicate vector, quaternion, or pose structs in `frob-lie`.
pub use fs_ga::{
    GaError, Mat3, Mat6, Motor, Quat, Se3, So3, So3Tangent, Twist, Vec3, Wrench,
};

/// Even-grade PGA blade indices `(1, e23, e31, e12, e01, e02, e03, e0123)`
/// stored inside `fs_ga::Motor` (`fs_ga::Pga([f64; 16])`).
pub const MOTOR_EVEN_BLADES: [usize; 8] = [0, 3, 5, 6, 9, 10, 12, 15];

/// Unit-versor tolerance enforced by `fs-ga` (`4e-12`).
pub const UNIT_TOLERANCE: f64 = 4.0e-12;

pub use se3::{Se3Ops, twist, wrench};
pub use so3::So3Ops;
pub use traits::{LieAlgebra, LieError, LieGroup};
