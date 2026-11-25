//! CLI harness driver for `scripts/e2e/evidence_ledger.sh` (`N00-10`).
//!
//! Subcommands:
//!   --print-table [--register <path>] [--json]
//!   --lint-tree <dir> [--register <path>] [--json]
//!   --check-change-rule <base_path> <current_path> [--json]

use std::env;
use std::path::{Path, PathBuf};
use std::process;

use frob_conformance::hypotheses::{
    HypothesisRegister, check_change_rule, lint_tree, render_exit_review_report,
};
use frob_core::canon::{self, Canonical};

fn default_register_path() -> PathBuf {
    PathBuf::from("crates/frob-conformance/registry/hypotheses.toml")
}

fn load_register(path: Option<&Path>) -> Result<HypothesisRegister, String> {
    match path {
        Some(p) => HypothesisRegister::from_path(p).map_err(|e| e.to_string()),
        None => {
            let p = default_register_path();
            if p.is_file() {
                HypothesisRegister::from_path(&p).map_err(|e| e.to_string())
            } else {
                HypothesisRegister::load_embedded().map_err(|e| e.to_string())
            }
        }
    }
}

fn write_json_canonical(value: &impl Canonical) -> String {
    match canon::to_canonical(value) {
        Ok(b) => b.as_str().to_owned(),
        Err(e) => format!("{{\"ok\":false,\"error\":\"{e}\"}}"),
    }
}

struct TableReport<'a> {
    reg: &'a HypothesisRegister,
}

impl Canonical for TableReport<'_> {
    fn write(&self, w: frob_core::canon::Value<'_>) -> Result<(), frob_core::canon::CanonError> {
        let c = self.reg.counts_by_state();
        let open = self.reg.open_disagreements();
        w.object(|o| {
            o.int53("active", c.active as i64)?;
            o.int53("deferred", c.deferred as i64)?;
            o.int53("disagreements_open", open.len() as i64)?;
            o.int53("disagreements_total", self.reg.disagreements.len() as i64)?;
            o.int53("narrowed", c.narrowed as i64)?;
            o.bool("ok", true)?;
            o.int53("refuted", c.refuted as i64)?;
            o.str("schema", &self.reg.schema)?;
            o.int53("total", c.total as i64)?;
            Ok(())
        })
    }
}

struct LintReport<'a> {
    findings: &'a [frob_conformance::hypotheses::WordingFinding],
}

impl Canonical for LintReport<'_> {
    fn write(&self, w: frob_core::canon::Value<'_>) -> Result<(), frob_core::canon::CanonError> {
        w.object(|o| {
            o.int53("finding_count", self.findings.len() as i64)?;
            o.array("findings", |a| {
                for f in self.findings {
                    a.push_object(|fo| {
                        fo.str("excerpt", &f.excerpt)?;
                        fo.str("file", &f.file)?;
                        fo.int53("line", f.line as i64)?;
                        if let Some(pidx) = f.pattern_index {
                            fo.int53("pattern_index", pidx as i64)?;
                        }
                        fo.str("row_id", &f.row_id)?;
                        fo.str("rule", f.rule)?;
                        Ok(())
                    })?;
                }
                Ok(())
            })?;
            o.bool("ok", self.findings.is_empty())?;
            Ok(())
        })
    }
}

struct ChangeReport<'a> {
    ok: bool,
    load_error: Option<&'a str>,
    violations: &'a [frob_conformance::hypotheses::ChangeRuleViolation],
}

impl Canonical for ChangeReport<'_> {
    fn write(&self, w: frob_core::canon::Value<'_>) -> Result<(), frob_core::canon::CanonError> {
        w.object(|o| {
            if let Some(err) = self.load_error {
                o.str("load_error", err)?;
            }
            o.bool("ok", self.ok)?;
            o.int53("violation_count", self.violations.len() as i64)?;
            o.array("violations", |a| {
                for v in self.violations {
                    a.push_object(|vo| {
                        vo.str("message", &v.message)?;
                        vo.str("row_id", &v.row_id)?;
                        vo.str("rule", v.rule)?;
                        Ok(())
                    })?;
                }
                Ok(())
            })?;
            Ok(())
        })
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut register_path: Option<PathBuf> = None;
    let mut print_table = false;
    let mut lint_dir: Option<PathBuf> = None;
    let mut change_pair: Option<(PathBuf, PathBuf)> = None;
    let mut as_json = false;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--register" => {
                i += 1;
                if i >= args.len() {
                    eprintln!("evidence_ledger: --register requires a path");
                    process::exit(2);
                }
                register_path = Some(PathBuf::from(&args[i]));
            }
            "--print-table" => print_table = true,
            "--lint-tree" => {
                i += 1;
                if i >= args.len() {
                    eprintln!("evidence_ledger: --lint-tree requires a directory");
                    process::exit(2);
                }
                lint_dir = Some(PathBuf::from(&args[i]));
            }
            "--check-change-rule" => {
                if i + 2 >= args.len() {
                    eprintln!("evidence_ledger: --check-change-rule requires <base> <current>");
                    process::exit(2);
                }
                change_pair = Some((PathBuf::from(&args[i + 1]), PathBuf::from(&args[i + 2])));
                i += 2;
            }
            "--json" => as_json = true,
            other => {
                eprintln!("evidence_ledger: unknown argument `{other}`");
                process::exit(2);
            }
        }
        i += 1;
    }

    if let Some((base_path, cur_path)) = change_pair {
        let base = match HypothesisRegister::from_path(&base_path) {
            Ok(r) => r,
            Err(e) => {
                let msg = e.to_string();
                if as_json {
                    println!(
                        "{}",
                        write_json_canonical(&ChangeReport {
                            ok: false,
                            load_error: Some(&msg),
                            violations: &[],
                        })
                    );
                } else {
                    eprintln!("base load error: {msg}");
                }
                process::exit(3);
            }
        };
        let cur = match HypothesisRegister::from_path(&cur_path) {
            Ok(r) => r,
            Err(e) => {
                let msg = e.to_string();
                if as_json {
                    println!(
                        "{}",
                        write_json_canonical(&ChangeReport {
                            ok: false,
                            load_error: Some(&msg),
                            violations: &[],
                        })
                    );
                } else {
                    eprintln!("current load error: {msg}");
                }
                process::exit(3);
            }
        };
        let violations = check_change_rule(&base, &cur);
        let ok = violations.is_empty();
        if as_json {
            println!(
                "{}",
                write_json_canonical(&ChangeReport {
                    ok,
                    load_error: None,
                    violations: &violations,
                })
            );
        } else if ok {
            println!("change-rule ok");
        } else {
            for v in &violations {
                eprintln!("{v}");
            }
        }
        process::exit(if ok { 0 } else { 3 });
    }

    let reg = match load_register(register_path.as_deref()) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("evidence_ledger: {e}");
            process::exit(3);
        }
    };

    if let Some(dir) = lint_dir {
        let findings = match lint_tree(&reg, &dir) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("evidence_ledger: {e}");
                process::exit(3);
            }
        };
        let ok = findings.is_empty();
        if as_json {
            println!("{}", write_json_canonical(&LintReport { findings: &findings }));
        } else if ok {
            println!("wording-lint ok (0 findings)");
        } else {
            for f in &findings {
                eprintln!("{f}");
            }
        }
        process::exit(if ok { 0 } else { 3 });
    }

    if print_table || !as_json {
        if as_json {
            println!("{}", write_json_canonical(&TableReport { reg: &reg }));
        } else {
            print!("{}", render_exit_review_report(&reg, None));
        }
    } else {
        println!("{}", write_json_canonical(&TableReport { reg: &reg }));
    }
}
