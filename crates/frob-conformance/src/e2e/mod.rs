//! `std`-only end-to-end log tooling (`STD-E2E`).

pub mod checks;
pub mod json;
pub mod line;
pub mod lint;
pub mod report;
pub mod sha256;

pub const USAGE: &str = "\
usage: frob-e2e <subcommand> [options]

Subcommands:
  emit        Append a validated frob.e2elog/1 line (and optional transcript)
  run         Execute a command verbatim, capture stdout/stderr, log cmd/refusal/assert
  get         Extract a field from JSON by RFC 6901 JSON Pointer
  cmp         Compare values (eq, num, close, rational, digest, sha256, repairs)
  check       Run a registered script check (e.g. single-envelope)
  lint        Validate one or more frob.e2elog/1 logs and their capture artifacts
  report      Aggregate headers and newest logs into gates.json and gates.md
  transcript  Render a frob.e2elog/1 log as a human-readable transcript
  schema      Print schemas/e2e-log.schema.json for frob.e2elog/1
  list        List scripts matching # e2e: header filters
";

pub fn dispatch(args: &[String]) -> i32 {
    let Some(sub) = args.first().map(String::as_str) else {
        eprint!("{USAGE}");
        return 2;
    };
    match sub {
        "-h" | "--help" | "help" => {
            print!("{USAGE}");
            0
        }
        "emit" => line::cmd_emit(&args[1..]),
        "run" => line::cmd_run(&args[1..]),
        "get" => line::cmd_get(&args[1..]),
        "cmp" => line::cmd_cmp(&args[1..]),
        "check" => checks::cmd_check(&args[1..]),
        "lint" => lint::cmd_lint(&args[1..]),
        "report" => report::cmd_report(&args[1..]),
        "transcript" => line::cmd_transcript(&args[1..]),
        "schema" => {
            print!("{}", line::schema_json());
            0
        }
        "list" => report::cmd_list(&args[1..]),
        other => {
            eprintln!("frob-e2e: unknown subcommand '{other}'\n{USAGE}");
            2
        }
    }
}
