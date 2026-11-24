//! CLI entrypoint for `frob-e2e` (`STD-E2E`, `std`-only).

use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let code = frob_conformance::e2e::dispatch(&args);
    ExitCode::from(code.clamp(0, 255) as u8)
}
