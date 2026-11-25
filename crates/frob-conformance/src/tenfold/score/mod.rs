//! Dependency-free ten-fold statistical scorer modules (`N39a-01`, §39.1).
//!
//! Every file under `score/` lies inside `BOUNDARY_FILES`, builds standalone
//! in `kit/scorer/` with `frob-cert` as its sole path dependency, and forbids
//! platform transcendental math calls (`f64::ln`, `exp`, `powf`, etc.).

pub mod bootstrap;
pub mod ceiling;
pub mod guard;
pub mod monitor;
pub mod noninferiority;
pub mod normal;
pub mod preanalysis;
pub mod ratio;
pub mod rng;
pub mod verdict;
pub mod warm;
