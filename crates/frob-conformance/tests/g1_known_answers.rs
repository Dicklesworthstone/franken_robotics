#![deny(unsafe_code)]

//! Gauntlet Tier G1: known answers (`BD-025`, `N38a-02`).
//!
//! Run as: `FROB_TEST_SEED=<seed> cargo test -p frob-conformance --test g1_known_answers -- <filter>`.
//! Builds on host with no device annex (`cuda` / `metal`).

use std::process::ExitCode;

use frob_conformance::tiers::{builtins, tier_main};

fn main() -> ExitCode {
    match builtins::g1_table() {
        Ok(table) => tier_main(&table),
        Err(err) => {
            eprintln!("g1_known_answers table error: {err}");
            ExitCode::from(101)
        }
    }
}
