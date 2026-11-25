#![deny(unsafe_code)]

//! Gauntlet Tier G2: parity with pinned oracles (`BD-025`, `N38a-02`).
//!
//! Run as: `FROB_TEST_SEED=<seed> cargo test -p frob-conformance --test g2_parity -- <filter>`.

use std::process::ExitCode;

use frob_conformance::tiers::{builtins, tier_main};

fn main() -> ExitCode {
    match builtins::g2_table() {
        Ok(table) => tier_main(&table),
        Err(err) => {
            eprintln!("g2_parity table error: {err}");
            ExitCode::from(101)
        }
    }
}
