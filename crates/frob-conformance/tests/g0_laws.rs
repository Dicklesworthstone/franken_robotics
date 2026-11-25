#![deny(unsafe_code)]

//! Gauntlet Tier G0: laws and self-tests (`BD-025`, `N38a-02`).
//!
//! Run as: `FROB_TEST_SEED=<seed> cargo test -p frob-conformance --test g0_laws -- <filter>`.
//! Builds on host with no device annex (`cuda` / `metal`).

use std::process::ExitCode;

use frob_conformance::tiers::{builtins, tier_main};

fn main() -> ExitCode {
    match builtins::g0_table() {
        Ok(table) => tier_main(&table),
        Err(err) => {
            eprintln!("g0_laws table error: {err}");
            ExitCode::from(101)
        }
    }
}
