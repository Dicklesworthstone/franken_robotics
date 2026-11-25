//! Standalone ten-fold scorer binary built from `kit/scorer/Cargo.toml`
//! (bead `N39a-01`, §39.1). Emits identical output bytes to `frob-tenfold-score`.

#![forbid(unsafe_code)]

use std::io::{self, Write};
use std::process::ExitCode;

use frob_tenfold_scorer::tenfold::score::verdict::run_scorer_cli;

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
