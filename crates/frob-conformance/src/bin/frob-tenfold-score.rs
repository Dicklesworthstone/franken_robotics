//! Boundary-clean ten-fold harness scorer CLI (bead `N39a-01`, §39.1).
//!
//! Reads harness records and writes only the scorer's derived records,
//! append-only, never rewriting an input and never writing `registries/gates.toml`.
//! Delegates to `tenfold::score::verdict::run_scorer_cli` so this binary and the
//! standalone `kit/scorer` binary emit bit-identical bytes.

#![forbid(unsafe_code)]

use std::io::{self, Write};
use std::process::ExitCode;

use frob_conformance::tenfold::score::verdict::run_scorer_cli;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let res = run_scorer_cli(&args);
    if !res.stdout.is_empty() {
        let _ = io::stdout().write_all(res.stdout.as_bytes());
    }
    if !res.stderr.is_empty() {
        let _ = io::stderr().write_all(res.stderr.as_bytes());
    }
    ExitCode::from(res.exit_code)
}
