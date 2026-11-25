#![deny(unsafe_code)]

//! Gauntlet Tier G3: metamorphic relations (`BD-025`, `N38a-02`).
//!
//! Run as: `FROB_TEST_SEED=<seed> cargo test -p frob-conformance --test g3_metamorphic -- <filter>`.

use std::process::ExitCode;

use frob_conformance::tiers::{builtins, tier_main};

fn main() -> ExitCode {
    match builtins::g3_table() {
        Ok(table) => tier_main(&table),
        Err(err) => {
            eprintln!("g3_metamorphic table error: {err}");
            ExitCode::from(101)
        }
    }
}
