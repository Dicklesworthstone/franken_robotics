#![forbid(unsafe_code)]

//! Standalone test-helper executable `ledger_kill_probe` (`[[test]]`,
//! `harness = false`, `required-features = ["ledger"]`) for `BD-009`
//! (`INT-FSQL-01`). Ships in no product binary.
//!
//! When invoked directly with probe CLI flags (`--db`, `--seed`, `--mode`,
//! `--plant`, ...), executes the forced-kill probe strictly. When invoked by
//! `cargo test -p frob-fsim` with test-filter arguments (no probe flags),
//! exits 0 immediately so the test sweep passes cleanly.

mod runner;

use std::env;
use std::io::{self, Write};
use std::process::ExitCode;

fn has_probe_flag(args: &[String]) -> bool {
    args.iter().any(|a| {
        matches!(
            a.as_str(),
            "--db" | "--seed" | "--txns" | "--rows" | "--mode" | "--start-from" | "--plant"
        )
    })
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    if !has_probe_flag(&args) {
        return ExitCode::SUCCESS;
    }
    match runner::run_from_args(args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(msg) => {
            let _ = writeln!(io::stderr().lock(), "ledger_kill_probe error: {msg}");
            ExitCode::FAILURE
        }
    }
}
