//! Gate verdict synthesis and boundary-clean CLI dispatch shared by
//! `frob-tenfold-score` and the standalone `kit/scorer` binary (`N39a-01`
//! skeleton; full verdict synthesis lands in its owning bead).
//!
//! Lives inside the scorer boundary (`BOUNDARY_FILES`): imports strictly `std`,
//! `frob_cert`, and listed boundary modules. Prints nothing directly (writes
//! through the caller-provided output buffers so library code satisfies L-7).

use std::fs;

use frob_cert::json::{JsonValue, serialize_canonical};

use crate::tenfold::canon::{RoundingMode, check_record_bytes};
use crate::tenfold::refusal::EXIT_REFUSAL_INPUT;
use crate::tenfold::score::ratio::{exact_ratio, render_ratio};

/// Help text for `frob-tenfold-score --help` and `kit/scorer --help`.
pub const SCORER_HELP: &str = "\
usage: frob-tenfold-score <command> [args]

Boundary-clean ten-fold harness scorer (std + frob-cert only, D0).
Reads harness records and writes only derived scorer outputs (append-only).

Commands:
  check <file>                                         Strictly check one harness record and print its b3: address
  ratio <num> <den> [--places <n>] [--mode <m>]        Reduce exact rational and render decimal (point|upper|lower)
  version                                              Print scorer version and trusted base
  --help, -h                                           Show this help message

Exit codes:
  0  success / valid record
  3  usage error or structural/schema refusal
  9  integrity refusal (record-hash-mismatch)
";

/// Result of running one `frob-tenfold-score` CLI invocation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScorerCliOutput {
    pub exit_code: u8,
    pub stdout: String,
    pub stderr: String,
}

/// Pure, boundary-clean CLI dispatcher shared by `src/bin/frob-tenfold-score.rs`
/// and `kit/scorer/src/main.rs` so both builds emit identical bytes.
#[must_use]
pub fn run_scorer_cli(args: &[String]) -> ScorerCliOutput {
    let Some(first) = args.first().map(String::as_str) else {
        return ScorerCliOutput {
            exit_code: EXIT_REFUSAL_INPUT,
            stdout: String::new(),
            stderr: SCORER_HELP.to_owned(),
        };
    };

    match first {
        "--help" | "-h" | "help" => ScorerCliOutput {
            exit_code: 0,
            stdout: SCORER_HELP.to_owned(),
            stderr: String::new(),
        },
        "version" => {
            let obj = JsonValue::Obj(vec![
                ("binary".to_owned(), JsonValue::Str("frob-tenfold-score".to_owned())),
                ("determinism".to_owned(), JsonValue::Str("D0".to_owned())),
                (
                    "trusted_base".to_owned(),
                    JsonValue::Str("std + frob-cert".to_owned()),
                ),
            ]);
            ScorerCliOutput {
                exit_code: 0,
                stdout: format!("{}\n", serialize_canonical(&obj)),
                stderr: String::new(),
            }
        }
        "check" => {
            if args.len() != 2 {
                return ScorerCliOutput {
                    exit_code: EXIT_REFUSAL_INPUT,
                    stdout: String::new(),
                    stderr: "usage: frob-tenfold-score check <file>\n".to_owned(),
                };
            }
            let path = &args[1];
            let bytes = match fs::read(path) {
                Ok(b) => b,
                Err(e) => {
                    return ScorerCliOutput {
                        exit_code: EXIT_REFUSAL_INPUT,
                        stdout: String::new(),
                        stderr: format!("frob-tenfold-score: unreadable file `{path}`: {e}\n"),
                    };
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
                    ScorerCliOutput {
                        exit_code: 0,
                        stdout: format!("{}\n", serialize_canonical(&obj)),
                        stderr: String::new(),
                    }
                }
                Err(refusal) => ScorerCliOutput {
                    exit_code: refusal.exit_code(),
                    stdout: format!("{}\n", refusal.to_canonical_json()),
                    stderr: String::new(),
                },
            }
        }
        "ratio" => run_ratio_cmd(&args[1..]),
        _ => ScorerCliOutput {
            exit_code: EXIT_REFUSAL_INPUT,
            stdout: String::new(),
            stderr: format!("frob-tenfold-score: unknown command `{first}`\n{SCORER_HELP}"),
        },
    }
}

fn run_ratio_cmd(args: &[String]) -> ScorerCliOutput {
    if args.len() < 2 {
        return ScorerCliOutput {
            exit_code: EXIT_REFUSAL_INPUT,
            stdout: String::new(),
            stderr: "usage: frob-tenfold-score ratio <num> <den> [--places <n>] [--mode point|upper|lower]\n"
                .to_owned(),
        };
    }
    let Ok(num) = args[0].parse::<i64>() else {
        return ScorerCliOutput {
            exit_code: EXIT_REFUSAL_INPUT,
            stdout: String::new(),
            stderr: format!("frob-tenfold-score: invalid integer numerator `{}`\n", args[0]),
        };
    };
    let Ok(den) = args[1].parse::<i64>() else {
        return ScorerCliOutput {
            exit_code: EXIT_REFUSAL_INPUT,
            stdout: String::new(),
            stderr: format!("frob-tenfold-score: invalid integer denominator `{}`\n", args[1]),
        };
    };

    let mut places: usize = 3;
    let mut mode = RoundingMode::HalfToEven;
    let mut idx = 2usize;
    while idx < args.len() {
        match args[idx].as_str() {
            "--places" => {
                let Some(v) = args.get(idx + 1) else {
                    return ScorerCliOutput {
                        exit_code: EXIT_REFUSAL_INPUT,
                        stdout: String::new(),
                        stderr: "frob-tenfold-score: --places requires an integer argument\n".to_owned(),
                    };
                };
                match v.parse::<usize>() {
                    Ok(p) => places = p,
                    Err(_) => {
                        return ScorerCliOutput {
                            exit_code: EXIT_REFUSAL_INPUT,
                            stdout: String::new(),
                            stderr: format!("frob-tenfold-score: invalid --places `{v}`\n"),
                        };
                    }
                }
                idx += 2;
            }
            "--mode" => {
                let Some(v) = args.get(idx + 1) else {
                    return ScorerCliOutput {
                        exit_code: EXIT_REFUSAL_INPUT,
                        stdout: String::new(),
                        stderr: "frob-tenfold-score: --mode requires point|upper|lower\n".to_owned(),
                    };
                };
                mode = match v.as_str() {
                    "point" => RoundingMode::HalfToEven,
                    "upper" => RoundingMode::OutwardUpper,
                    "lower" => RoundingMode::OutwardLower,
                    other => {
                        return ScorerCliOutput {
                            exit_code: EXIT_REFUSAL_INPUT,
                            stdout: String::new(),
                            stderr: format!(
                                "frob-tenfold-score: invalid --mode `{other}` (expected point|upper|lower)\n"
                            ),
                        };
                    }
                };
                idx += 2;
            }
            other => {
                return ScorerCliOutput {
                    exit_code: EXIT_REFUSAL_INPUT,
                    stdout: String::new(),
                    stderr: format!("frob-tenfold-score: unexpected flag `{other}`\n"),
                };
            }
        }
    }

    let r = match exact_ratio(num, den) {
        Ok(r) => r,
        Err(refusal) => {
            return ScorerCliOutput {
                exit_code: refusal.exit_code(),
                stdout: format!("{}\n", refusal.to_canonical_json()),
                stderr: String::new(),
            };
        }
    };
    let (exact_s, dec_s) = match render_ratio(&r, places, mode) {
        Ok(pair) => pair,
        Err(refusal) => {
            return ScorerCliOutput {
                exit_code: refusal.exit_code(),
                stdout: format!("{}\n", refusal.to_canonical_json()),
                stderr: String::new(),
            };
        }
    };
    let obj = JsonValue::Obj(vec![
        ("decimal".to_owned(), JsonValue::Str(dec_s)),
        ("exact".to_owned(), JsonValue::Str(exact_s)),
    ]);
    ScorerCliOutput {
        exit_code: 0,
        stdout: format!("{}\n", serialize_canonical(&obj)),
        stderr: String::new(),
    }
}
