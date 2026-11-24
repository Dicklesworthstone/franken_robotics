//! Core foundational library for FrankenRobotics.
//!
//! Currently exposes only `registry` (bead N34p2-01); additional core modules
//! land in BD-005. Prints nothing (rule L-7) and contains no `unsafe` code.

#![deny(unsafe_code)]

pub mod registry;
