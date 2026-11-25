//! Core foundational library for FrankenRobotics.
//!
//! Exposes `registry` (bead N34p2-01) and, behind feature `test-support`,
//! `testkit` (bead STD-TEST); additional core modules land in BD-005.
//! Prints nothing (rule L-7) and contains no `unsafe` code.

#![deny(unsafe_code)]

pub mod registry;

#[cfg(feature = "test-support")]
pub mod testkit;
