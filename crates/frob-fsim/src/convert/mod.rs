#![forbid(unsafe_code)]

//! Vocabulary conversions across the `frob-fsim` airlock boundary (`N33a-11`).
//!
//! Adapter beads (`BD-007`, `N33a-13`, `N33c-36`) populate private conversions
//! between FrankenRobotics types and FrankenSim types here; no volatile
//! FrankenSim type ever appears in a public signature.
