use std::io;
use std::process::ExitCode;

use xtask::check_layers;
use xtask::cli::{self, CheckFn};

/// Closed command table for `cargo xtask` (N34p4-01 seeds `check-layers`;
/// BD-003, N34-07, N34-08, N34p4-03, and BD-004 add the remaining five checks).
const COMMANDS: &[(&str, CheckFn)] = &[("check-layers", check_layers::run_command)];

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut stdout = io::stdout().lock();
    let mut stderr = io::stderr().lock();
    cli::run(COMMANDS, &args, &mut stdout, &mut stderr)
}

#[cfg(test)]
mod tests {
    use std::process::ExitCode;

    use super::COMMANDS;
    use xtask::cli::{self, EXIT_USAGE};

    #[test]
    fn no_argument_prints_subcommands_and_exits_3() {
        let mut out = Vec::new();
        let mut err = Vec::new();
        let code = cli::run(COMMANDS, &[], &mut out, &mut err);
        assert_eq!(code, ExitCode::from(EXIT_USAGE));
        assert!(out.is_empty());
        let rendered = String::from_utf8(err).expect("stderr must be valid UTF-8");
        assert!(rendered.contains("usage: cargo xtask"));
        assert!(rendered.contains("check-layers"));
    }

    #[test]
    fn unknown_subcommand_prints_subcommands_and_exits_3() {
        let mut out = Vec::new();
        let mut err = Vec::new();
        let args = vec!["nonexistent".to_owned()];
        let code = cli::run(COMMANDS, &args, &mut out, &mut err);
        assert_eq!(code, ExitCode::from(EXIT_USAGE));
        assert!(out.is_empty());
        let rendered = String::from_utf8(err).expect("stderr must be valid UTF-8");
        assert!(rendered.contains("unknown subcommand"));
        assert!(rendered.contains("check-layers"));
    }
}
