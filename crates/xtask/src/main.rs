use std::io;
use std::process::ExitCode;

use xtask::check_contracts;
use xtask::check_deps;
use xtask::check_generated;
use xtask::check_layers;
use xtask::check_unsafe;
use xtask::cli::{self, CheckFn};
use xtask::stack;

/// Closed six-check command table for `cargo xtask` (N34p4-01 `check-layers`,
/// BD-003 `check-deps`, N34-07 `check-unsafe`, N34p4-03 `check-generated`,
/// N34-08 `check-contracts`, N33a-05 `check-stack`; BD-004 enforces the count
/// and finding-identifier ownership matrix).
const COMMANDS: &[(&str, CheckFn)] = &[
    ("check-layers", check_layers::run_command),
    ("check-deps", check_deps::run_command),
    ("check-unsafe", check_unsafe::run_command),
    ("check-generated", check_generated::run_command),
    ("check-contracts", check_contracts::run_command),
    ("check-stack", stack::verify::run_command),
];

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
        assert!(rendered.contains("check-deps"));
        assert!(rendered.contains("check-unsafe"));
        assert!(rendered.contains("check-generated"));
        assert!(rendered.contains("check-contracts"));
        assert!(rendered.contains("check-stack"));
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
        assert!(rendered.contains("check-deps"));
        assert!(rendered.contains("check-unsafe"));
        assert!(rendered.contains("check-generated"));
        assert!(rendered.contains("check-contracts"));
        assert!(rendered.contains("check-stack"));
    }
}
