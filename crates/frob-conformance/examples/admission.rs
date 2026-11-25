#![deny(unsafe_code)]

//! CLI wrapper over `frob_conformance::admission` (`N33a-07`).
//!
//! Usage:
//!   cargo run -p frob-conformance --example admission -- record \
//!     --gate <id> --input <path> [--harness-receipt <64-hex>] [--root <dir>]

use std::env;
use std::io;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut stdout = io::stdout().lock();
    let mut stderr = io::stderr().lock();
    match frob_conformance::admission::run_cli(&args, &mut stdout, &mut stderr) {
        Ok(()) => ExitCode::SUCCESS,
        Err(code) => ExitCode::from(code),
    }
}
