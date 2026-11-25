#![deny(unsafe_code)]

//! Gauntlet Tier G5: determinism lanes (`BD-025`, `N38a-02`).
//!
//! Run as: `FROB_TEST_SEED=<seed> cargo test -p frob-conformance --test g5_determinism -- <filter>`.

use std::process::ExitCode;

use frob_conformance::tiers::{builtins, tier_main};

fn main() -> ExitCode {
    match builtins::g5_table() {
        Ok(table) => tier_main(&table),
        Err(err) => {
            eprintln!("g5_determinism table error: {err}");
            ExitCode::from(101)
        }
    }
}
