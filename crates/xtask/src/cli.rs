//! Command-line surface and exit-code contract for `cargo xtask` (bead N34p4-01).
//!
//! Surface: `cargo xtask <check> [--root <dir>] [--json]`
//! Exit codes:
//! - `0`: clean (zero findings)
//! - `2`: one or more findings reported
//! - `3`: usage error (no argument, unknown subcommand, invalid flag, missing `--root`,
//!        or offline `cargo metadata` requiring `cargo fetch`)
//! - `1`: internal tool fault

use std::collections::BTreeMap;
use std::fmt;
use std::io::Write;
use std::path::PathBuf;
use std::process::ExitCode;
use std::time::Instant;

use crate::metadata::WorkspaceRoot;
use crate::report::{CheckReport, FactValue, Finding};
use crate::workspaces;

/// Clean run with zero findings.
pub const EXIT_OK: u8 = 0;
/// Internal tool fault (unexpected I/O or execution error).
pub const EXIT_FAULT: u8 = 1;
/// Check completed and reported one or more findings.
pub const EXIT_FINDINGS: u8 = 2;
/// Usage or offline-cache error.
pub const EXIT_USAGE: u8 = 3;

/// Execution context passed to every `xtask` check subcommand.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckContext {
    pub repo_root: PathBuf,
    pub roots: Vec<WorkspaceRoot>,
    pub extra_args: Vec<String>,
}

/// Successful check evaluation outcome (findings + check-specific facts).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckOutcome {
    pub findings: Vec<Finding>,
    pub facts: BTreeMap<String, FactValue>,
}

/// Error returned when a check cannot complete.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CheckError {
    Usage(String),
    Fault(String),
}

impl fmt::Display for CheckError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Usage(msg) | Self::Fault(msg) => f.write_str(msg),
        }
    }
}

impl std::error::Error for CheckError {}

/// Uniform function signature for every row of the static `COMMANDS` array in `src/main.rs`.
pub type CheckFn = fn(&CheckContext) -> Result<CheckOutcome, CheckError>;

/// Parsed CLI invocation for `cargo xtask <check> [--root <dir>] [--json]`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedInvocation {
    pub check_name: String,
    pub root: PathBuf,
    pub root_display: String,
    pub json: bool,
    pub extra_args: Vec<String>,
}

/// Render usage text listing every registered subcommand name.
#[must_use]
pub fn format_usage(commands: &[(&str, CheckFn)]) -> String {
    let names: Vec<&str> = commands.iter().map(|(name, _)| *name).collect();
    let list = if names.is_empty() {
        "(none)".to_owned()
    } else {
        names.join(", ")
    };
    format!(
        "usage: cargo xtask <check> [--root <dir>] [--json]\n\
         \n\
         Subcommands: {list}\n\
         Exit codes: 0 clean, 2 findings, 3 usage error.\n"
    )
}

/// Parse CLI arguments against `commands` without executing the check.
pub fn parse_args(
    commands: &[(&str, CheckFn)],
    args: &[String],
) -> Result<(ParsedInvocation, CheckFn), String> {
    let Some(first) = args.first() else {
        return Err("missing subcommand".to_owned());
    };
    if first == "-h" || first == "--help" {
        return Err("help requested".to_owned());
    }
    let Some(&(_, handler)) = commands.iter().find(|(name, _)| *name == first.as_str()) else {
        return Err(format!("unknown subcommand `{first}`"));
    };

    let mut root_arg: Option<String> = None;
    let mut json = false;
    let mut extra_args = Vec::new();
    let mut i = 1usize;

    while i < args.len() {
        let arg = &args[i];
        if arg == "--json" {
            json = true;
            i += 1;
        } else if arg == "--root" {
            let Some(val) = args.get(i + 1) else {
                return Err("`--root` requires a directory argument".to_owned());
            };
            if val.is_empty() {
                return Err("`--root` directory must not be empty".to_owned());
            }
            root_arg = Some(val.clone());
            i += 2;
        } else if let Some(val) = arg.strip_prefix("--root=") {
            if val.is_empty() {
                return Err("`--root` directory must not be empty".to_owned());
            }
            root_arg = Some(val.to_owned());
            i += 1;
        } else if arg == "-h" || arg == "--help" {
            return Err("help requested".to_owned());
        } else {
            extra_args.push(arg.clone());
            i += 1;
        }
    }

    let (root, root_display) = match root_arg {
        Some(raw) => (PathBuf::from(&raw), raw),
        None => {
            let cwd = std::env::current_dir()
                .map_err(|e| format!("failed to read current working directory: {e}"))?;
            let display = cwd.to_string_lossy().into_owned();
            (cwd, display)
        }
    };

    if !root.is_dir() {
        return Err(format!("`--root` directory does not exist: {}", root.display()));
    }

    Ok((
        ParsedInvocation {
            check_name: first.clone(),
            root,
            root_display,
            json,
            extra_args,
        },
        handler,
    ))
}

/// Execute `cargo xtask` with explicit command table and I/O streams.
pub fn run<WOut: Write, WErr: Write>(
    commands: &[(&str, CheckFn)],
    args: &[String],
    stdout: &mut WOut,
    stderr: &mut WErr,
) -> ExitCode {
    let (parsed, handler) = match parse_args(commands, args) {
        Ok(pair) => pair,
        Err(reason) => {
            if reason != "help requested" {
                let _ = writeln!(stderr, "error: {reason}");
            }
            let _ = stderr.write_all(format_usage(commands).as_bytes());
            return ExitCode::from(EXIT_USAGE);
        }
    };

    let ctx = CheckContext {
        roots: workspaces::workspace_roots(&parsed.root),
        repo_root: parsed.root,
        extra_args: parsed.extra_args,
    };

    let started = Instant::now();
    let outcome = match handler(&ctx) {
        Ok(o) => o,
        Err(CheckError::Usage(msg)) => {
            let _ = writeln!(stderr, "error: {msg}");
            let _ = stderr.write_all(format_usage(commands).as_bytes());
            return ExitCode::from(EXIT_USAGE);
        }
        Err(CheckError::Fault(msg)) => {
            let _ = writeln!(stderr, "fault: {msg}");
            return ExitCode::from(EXIT_FAULT);
        }
    };

    let elapsed_ms = u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX);
    let report = CheckReport::new(
        parsed.check_name,
        parsed.root_display,
        outcome.findings,
        outcome.facts,
        elapsed_ms,
    );

    if parsed.json {
        match report.to_canonical_json() {
            Ok(json_str) => {
                if let Err(err) = stdout.write_all(json_str.as_bytes()) {
                    let _ = writeln!(stderr, "fault: failed writing JSON report: {err}");
                    return ExitCode::from(EXIT_FAULT);
                }
            }
            Err(err) => {
                let _ = writeln!(stderr, "fault: failed encoding JSON report: {err}");
                return ExitCode::from(EXIT_FAULT);
            }
        }
    } else if let Err(err) = stdout.write_all(report.format_human().as_bytes()) {
        let _ = writeln!(stderr, "fault: failed writing report: {err}");
        return ExitCode::from(EXIT_FAULT);
    }

    if report.ok {
        ExitCode::from(EXIT_OK)
    } else {
        ExitCode::from(EXIT_FINDINGS)
    }
}
