use std::io;
use std::process::ExitCode;

use xtask::assembly::COMMANDS;
use xtask::cli;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut stdout = io::stdout().lock();
    let mut stderr = io::stderr().lock();
    cli::run(COMMANDS, &args, &mut stdout, &mut stderr)
}

#[cfg(test)]
mod tests {
    use std::process::ExitCode;

    use xtask::assembly::{COMMANDS, verify_command_table};
    use xtask::cli::{self, EXIT_USAGE};

    #[test]
    fn main_command_table_is_strictly_six_checks() {
        verify_command_table(COMMANDS).expect("main COMMANDS table must be exactly the six checks");
    }

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
