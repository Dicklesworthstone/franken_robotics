#![deny(unsafe_code)]

//! CLI driver for the Gauntlet pilot register (`N38a-09`, `GATE-GAUNT-08`).
//!
//! Usage:
//!   cargo run -p frob-conformance --example pilots -- fundable --all --json
//!   cargo run -p frob-conformance --example pilots -- fundable --package <WP-nn> --json
//!   cargo run -p frob-conformance --example pilots -- fundable --bead <key> --json
//!   cargo run -p frob-conformance --example pilots -- complete --json
//!   cargo run -p frob-conformance --example pilots -- check [--prev <path>] --json
//!   cargo run -p frob-conformance --example pilots -- set-status --id <PILOT-ID> --status <status>
//!   cargo run -p frob-conformance --example pilots -- printout [--write | --check]
//!   cargo run -p frob-conformance --example pilots -- hash <path>

use std::env;
use std::io;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut stdout = io::stdout().lock();
    let mut stderr = io::stderr().lock();
    match frob_conformance::pilots::run_cli(&args, &mut stdout, &mut stderr) {
        Ok(()) => ExitCode::SUCCESS,
        Err(code) => ExitCode::from(code),
    }
}
