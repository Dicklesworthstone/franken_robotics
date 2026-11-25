//! Operator tool of the adjudicator and gate runner for the FrankenRobotics
//! ten-fold comparative benchmark harness (bead `N39a-01`, §39.1).
//!
//! Not a `frob` product verb and never exposed over MCP (`ADR-GAUNT-01`).
//! Reads and writes only the gate store `tenfold/<gate>/` and harness records;
//! never reads or writes `registries/gates.toml`.

#![forbid(unsafe_code)]

use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::ExitCode;

use frob_cert::json::{JsonValue, serialize_canonical};
use frob_conformance::tenfold::canon::{
    SchemaKind, check_record_bytes, content_address, parse_json_strict, require_str,
};
use frob_conformance::tenfold::ids::GateId;
use frob_conformance::tenfold::refusal::EXIT_REFUSAL_INPUT;
use frob_conformance::tenfold::store::GateStore;

const HELP: &str = "\
usage: frob-tenfold <command> [subcommand] [args]

Operator tool of the adjudicator and gate runner for the ten-fold harness (N39a-01).
Not a product `frob` verb; never writes `registries/gates.toml`.

Commands:
  record check <file>                     Strictly validate a harness record; exit 0 or refusal code (3/9)
  record address [--allow-blank] <file>   Print the `b3:` BLAKE3 content address of a harness record
  record canon <file>                     Emit unique canonical JSON bytes for a valid harness record
  store init <dir> --gate <gate>          Create the append-only gate store directory layout
  --help, -h                              Show this help message

Exit codes:
  0  valid / success
  3  usage error or structural/schema refusal
  9  integrity refusal (record-hash-mismatch or store-overwrite)
";

fn write_stdout(text: &str) {
    let _ = io::stdout().write_all(text.as_bytes());
}

fn write_stderr(text: &str) {
    let _ = io::stderr().write_all(text.as_bytes());
}

fn run_record(args: &[String]) -> u8 {
    let Some(sub) = args.first().map(String::as_str) else {
        write_stderr("usage: frob-tenfold record <check|address|canon> <file>\n");
        return EXIT_REFUSAL_INPUT;
    };

    match sub {
        "check" => {
            if args.len() != 2 {
                write_stderr("usage: frob-tenfold record check <file>\n");
                return EXIT_REFUSAL_INPUT;
            }
            let path = &args[1];
            let bytes = match fs::read(path) {
                Ok(b) => b,
                Err(e) => {
                    write_stderr(&format!("frob-tenfold: unreadable file `{path}`: {e}\n"));
                    return EXIT_REFUSAL_INPUT;
                }
            };
            match check_record_bytes(&bytes) {
                Ok(checked) => {
                    let obj = JsonValue::Obj(vec![
                        (
                            "address".to_owned(),
                            JsonValue::Str(checked.address.to_b3_string()),
                        ),
                        ("gate".to_owned(), JsonValue::Str(checked.gate.as_string())),
                        (
                            "schema".to_owned(),
                            JsonValue::Str(checked.schema.as_str().to_owned()),
                        ),
                        ("status".to_owned(), JsonValue::Str("ok".to_owned())),
                    ]);
                    write_stdout(&format!("{}\n", serialize_canonical(&obj)));
                    0
                }
                Err(refusal) => {
                    write_stdout(&format!("{}\n", refusal.to_canonical_json()));
                    refusal.exit_code()
                }
            }
        }
        "address" => {
            let (allow_blank, path) = match args.len() {
                2 => (false, &args[1]),
                3 if args[1] == "--allow-blank" => (true, &args[2]),
                _ => {
                    write_stderr("usage: frob-tenfold record address [--allow-blank] <file>\n");
                    return EXIT_REFUSAL_INPUT;
                }
            };
            let bytes = match fs::read(path) {
                Ok(b) => b,
                Err(e) => {
                    write_stderr(&format!("frob-tenfold: unreadable file `{path}`: {e}\n"));
                    return EXIT_REFUSAL_INPUT;
                }
            };
            if allow_blank {
                let val = match parse_json_strict(&bytes) {
                    Ok(v) => v,
                    Err(refusal) => {
                        write_stdout(&format!("{}\n", refusal.to_canonical_json()));
                        return refusal.exit_code();
                    }
                };
                let schema_str = match require_str(&val, "", "schema") {
                    Ok(s) => s,
                    Err(refusal) => {
                        write_stdout(&format!("{}\n", refusal.to_canonical_json()));
                        return refusal.exit_code();
                    }
                };
                let schema = match SchemaKind::parse(schema_str) {
                    Ok(s) => s,
                    Err(refusal) => {
                        write_stdout(&format!("{}\n", refusal.to_canonical_json()));
                        return refusal.exit_code();
                    }
                };
                match content_address(&val, schema) {
                    Ok(addr) => {
                        write_stdout(&format!("{}\n", addr.to_b3_string()));
                        0
                    }
                    Err(refusal) => {
                        write_stdout(&format!("{}\n", refusal.to_canonical_json()));
                        refusal.exit_code()
                    }
                }
            } else {
                match check_record_bytes(&bytes) {
                    Ok(checked) => {
                        write_stdout(&format!("{}\n", checked.address.to_b3_string()));
                        0
                    }
                    Err(refusal) => {
                        write_stdout(&format!("{}\n", refusal.to_canonical_json()));
                        refusal.exit_code()
                    }
                }
            }
        }
        "canon" => {
            if args.len() != 2 {
                write_stderr("usage: frob-tenfold record canon <file>\n");
                return EXIT_REFUSAL_INPUT;
            }
            let path = &args[1];
            let bytes = match fs::read(path) {
                Ok(b) => b,
                Err(e) => {
                    write_stderr(&format!("frob-tenfold: unreadable file `{path}`: {e}\n"));
                    return EXIT_REFUSAL_INPUT;
                }
            };
            match check_record_bytes(&bytes) {
                Ok(checked) => {
                    let _ = io::stdout().write_all(&checked.canonical_bytes);
                    0
                }
                Err(refusal) => {
                    write_stdout(&format!("{}\n", refusal.to_canonical_json()));
                    refusal.exit_code()
                }
            }
        }
        other => {
            write_stderr(&format!("frob-tenfold: unknown record subcommand `{other}`\n{HELP}"));
            EXIT_REFUSAL_INPUT
        }
    }
}

fn run_store(args: &[String]) -> u8 {
    if args.len() != 4 || args[0] != "init" || args[2] != "--gate" {
        write_stderr("usage: frob-tenfold store init <dir> --gate <gate>\n");
        return EXIT_REFUSAL_INPUT;
    }
    let root = PathBuf::from(&args[1]);
    let gate = match GateId::parse(&args[3], "/gate") {
        Ok(g) => g,
        Err(refusal) => {
            write_stdout(&format!("{}\n", refusal.to_canonical_json()));
            return refusal.exit_code();
        }
    };
    let store = GateStore::new(root, gate);
    match store.init_dirs() {
        Ok(()) => {
            let obj = JsonValue::Obj(vec![
                (
                    "gate_dir".to_owned(),
                    JsonValue::Str(store.gate_dir().display().to_string()),
                ),
                ("status".to_owned(), JsonValue::Str("ok".to_owned())),
            ]);
            write_stdout(&format!("{}\n", serialize_canonical(&obj)));
            0
        }
        Err(refusal) => {
            write_stdout(&format!("{}\n", refusal.to_canonical_json()));
            refusal.exit_code()
        }
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let Some(first) = args.first().map(String::as_str) else {
        write_stderr(HELP);
        return ExitCode::from(EXIT_REFUSAL_INPUT);
    };

    let code = match first {
        "--help" | "-h" | "help" => {
            write_stdout(HELP);
            0
        }
        "record" => run_record(&args[1..]),
        "store" => run_store(&args[1..]),
        other => {
            write_stderr(&format!("frob-tenfold: unknown command `{other}`\n{HELP}"));
            EXIT_REFUSAL_INPUT
        }
    };
    ExitCode::from(code)
}
