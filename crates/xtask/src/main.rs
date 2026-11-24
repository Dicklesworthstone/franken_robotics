use std::io::{self, Write};
use std::process::ExitCode;

/// Exit code reserved for usage errors across the `xtask` CLI surface
/// (0 clean, 2 findings, 3 usage; kept by N34p4-01, BD-003, BD-004).
const EXIT_USAGE: u8 = 3;

const USAGE: &str = "\
usage: cargo xtask <check> [--root <dir>] [--json]

No checks are registered in the BD-001 workspace skeleton yet;
subcommands arrive with N34p4-01, BD-003, and BD-004.
Exit codes: 0 clean, 2 findings, 3 usage error.
";

fn run<W: Write>(_args: &[String], out: &mut W) -> ExitCode {
    let _ = out.write_all(USAGE.as_bytes());
    ExitCode::from(EXIT_USAGE)
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut stderr = io::stderr().lock();
    run(&args, &mut stderr)
}

#[cfg(test)]
mod tests {
    use super::{EXIT_USAGE, USAGE, run};
    use std::process::ExitCode;

    #[test]
    fn xtask_skeleton_prints_usage_and_exits_3() {
        let mut buf = Vec::new();
        let code = run(&[], &mut buf);
        assert_eq!(code, ExitCode::from(EXIT_USAGE));
        let rendered = String::from_utf8(buf).expect("usage text must be valid UTF-8");
        assert_eq!(rendered, USAGE);
        assert!(rendered.contains("usage: cargo xtask"));
    }

    #[test]
    fn xtask_skeleton_unknown_subcommand_prints_usage_and_exits_3() {
        let mut buf = Vec::new();
        let args = vec!["nonexistent".to_string()];
        let code = run(&args, &mut buf);
        assert_eq!(code, ExitCode::from(EXIT_USAGE));
        let rendered = String::from_utf8(buf).expect("usage text must be valid UTF-8");
        assert_eq!(rendered, USAGE);
    }
}
