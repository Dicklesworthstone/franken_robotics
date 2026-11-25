//! Check a captured `frob --json` standard output against the one-envelope rule and
//! `frob.envelope/1` (N32a-03). Used by `scripts/e2e/tether_envelope_exit.sh`.
//!
//! Usage: `envelope_check <stdout-file>`. Prints one JSON line: on success
//! `{"ok":true,"progress_lines":N,"exit":E,"envelope_ok":B,"code":C,"evidence":{...}}`
//! (`code` only on a refusal), otherwise
//! `{"ok":false,"error":"...","first_field":"..."}`. `first_field` is a JSON pointer within
//! the invalid line and is absent for stream-level failures. Exits 0 when the output is valid,
//! 1 when it is not, 2 on a usage or I/O error.

use std::process::ExitCode;

use frob_core::canon::{self, CanonError, Canonical, Value};
use frob_verbs::envelope::{Tree, check_stream};

struct Report<'a> {
    ok: bool,
    progress_lines: usize,
    exit: Option<u8>,
    envelope_ok: Option<bool>,
    code: Option<&'a str>,
    evidence: Option<&'a Tree>,
    error: Option<String>,
    first_field: Option<String>,
}

impl Canonical for Report<'_> {
    fn write(&self, w: Value<'_>) -> Result<(), CanonError> {
        w.object(|o| {
            o.bool("ok", self.ok)?;
            if self.ok {
                o.int53(
                    "progress_lines",
                    i64::try_from(self.progress_lines).unwrap_or(i64::MAX >> 11),
                )?;
            }
            if let Some(e) = self.exit {
                o.int53("exit", i64::from(e))?;
            }
            if let Some(b) = self.envelope_ok {
                o.bool("envelope_ok", b)?;
            }
            if let Some(c) = self.code {
                o.str("code", c)?;
            }
            if let Some(ev) = self.evidence {
                o.member_term("evidence", ev)?;
            }
            if let Some(e) = &self.error {
                o.str("error", e)?;
            }
            if let Some(f) = &self.first_field {
                o.str("first_field", f)?;
            }
            Ok(())
        })
    }
}

fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);
    let (Some(path), None) = (args.next(), args.next()) else {
        eprintln!("usage: envelope_check <stdout-file>");
        return ExitCode::from(2);
    };
    let bytes = match std::fs::read(&path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("envelope_check: {path}: {e}");
            return ExitCode::from(2);
        }
    };
    let (report, valid) = match check_stream(&bytes) {
        Ok(summary) => {
            let env = summary.envelope;
            let line = canon::to_canonical(&Report {
                ok: true,
                progress_lines: summary.progress_lines,
                exit: Some(env.exit().code()),
                envelope_ok: env.tree().get("ok").and_then(Tree::as_bool),
                code: env.refusal_code(),
                evidence: env.tree().get("evidence"),
                error: None,
                first_field: None,
            });
            (line, true)
        }
        Err(e) => {
            let line = canon::to_canonical(&Report {
                ok: false,
                progress_lines: 0,
                exit: None,
                envelope_ok: None,
                code: None,
                evidence: None,
                error: Some(e.to_string()),
                first_field: e.first_field().map(str::to_owned),
            });
            (line, false)
        }
    };
    match report {
        Ok(line) => {
            println!("{}", line.as_str());
            if valid {
                ExitCode::SUCCESS
            } else {
                ExitCode::from(1)
            }
        }
        Err(e) => {
            eprintln!("envelope_check: {e}");
            ExitCode::from(2)
        }
    }
}
