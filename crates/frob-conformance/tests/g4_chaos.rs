#![deny(unsafe_code)]

//! Gauntlet Tier G4: chaos and fault injection (`BD-025`, `N38a-02`).
//!
//! Run as: `FROB_TEST_SEED=<seed> cargo test -p frob-conformance --test g4_chaos -- <filter>`.

use std::process::ExitCode;

use frob_conformance::tiers::{builtins, tier_main};

fn main() -> ExitCode {
    match builtins::g4_table() {
        Ok(table) => tier_main(&table),
        Err(err) => {
            eprintln!("g4_chaos table error: {err}");
            ExitCode::from(101)
        }
    }
}
