#![deny(unsafe_code)]

//! Read-only CLI validator for `registries/gates.toml` (`frob.gates/1`, `N40-01`).
//!
//! Usage:
//!   cargo run -p frob-conformance --example validate_register -- --file <path>
//!     [--repo-root <dir>]
//!     [--controls <path>]
//!     [--live-claims <csv>]
//!     [--live-evidence <csv>]
//!     [--retired <csv>]
//!     [--previous <csv>]
//!     [--enforce-recurring]
//!     [--due-at <M0..M8>]
//!     [--json]
//!
//! Exits `0` when valid and `3` on any load error or rule violation (naming the offending
//! gate row and `N38-REF` reason slug). Never calls `register::record` or `register::write`
//! (one-recorder invariant, `N38b-19`).

use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;

use frob_conformance::register::{
    GATES_SCHEMA, ValidationContext, Violation, load_with_context,
};
use frob_core::canon::{self, Canonical};
use frob_core::testkit::registry::{self as testkit_registry, Controls};

fn parse_csv_set(raw: &str) -> BTreeSet<String> {
    raw.split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_owned)
        .collect()
}

fn detect_repo_root(file: &Path) -> Option<PathBuf> {
    let mut cur = file.parent();
    while let Some(d) = cur {
        if d.join("franken-stack.lock").is_file() || d.join("receipts/gates").is_dir() {
            return Some(d.to_path_buf());
        }
        cur = d.parent();
    }
    testkit_registry::root().ok()
}

struct ValidationReport<'a> {
    due_at: Option<&'a [String]>,
    file: &'a str,
    gate_count: usize,
    ok: bool,
    schema: &'a str,
    violations: &'a [Violation],
}

impl Canonical for ValidationReport<'_> {
    fn write(&self, w: frob_core::canon::Value<'_>) -> Result<(), frob_core::canon::CanonError> {
        w.object(|o| {
            if let Some(due) = self.due_at {
                o.array("due_at", |a| {
                    for id in due {
                        a.push_str(id);
                    }
                    Ok(())
                })?;
                o.int53("due_at_count", due.len() as i64)?;
            }
            o.str("file", self.file)?;
            o.int53("gate_count", self.gate_count as i64)?;
            o.bool("ok", self.ok)?;
            o.str("schema", self.schema)?;
            o.int53("violation_count", self.violations.len() as i64)?;
            o.array("violations", |a| {
                for v in self.violations {
                    a.push_object(|vo| {
                        if let Some(ref fld) = v.field {
                            vo.str("field", fld)?;
                        }
                        if let Some(ref gid) = v.gate_id {
                            vo.str("gate_id", gid)?;
                        }
                        vo.str("message", &v.message)?;
                        vo.str("reason", v.reason)?;
                        vo.str("repair", v.repair)?;
                        Ok(())
                    })?;
                }
                Ok(())
            })?;
            Ok(())
        })
    }
}

fn write_json(report: &ValidationReport<'_>) -> String {
    match canon::to_canonical(report) {
        Ok(b) => b.as_str().to_owned(),
        Err(e) => format!("{{\"ok\":false,\"error\":\"{e}\"}}"),
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut file_path: Option<PathBuf> = None;
    let mut repo_root: Option<PathBuf> = None;
    let mut controls_path: Option<PathBuf> = None;
    let mut live_claims: Option<BTreeSet<String>> = None;
    let mut live_evidence: Option<BTreeSet<String>> = None;
    let mut retired_ids = BTreeSet::new();
    let mut previous_ids: Option<BTreeSet<String>> = None;
    let mut enforce_recurring = false;
    let mut due_at_milestone: Option<String> = None;
    let mut as_json = false;

    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--file" => {
                i += 1;
                if i >= args.len() {
                    eprintln!("validate_register: --file requires a path");
                    process::exit(3);
                }
                file_path = Some(PathBuf::from(&args[i]));
            }
            "--repo-root" => {
                i += 1;
                if i >= args.len() {
                    eprintln!("validate_register: --repo-root requires a directory");
                    process::exit(3);
                }
                repo_root = Some(PathBuf::from(&args[i]));
            }
            "--controls" => {
                i += 1;
                if i >= args.len() {
                    eprintln!("validate_register: --controls requires a path");
                    process::exit(3);
                }
                controls_path = Some(PathBuf::from(&args[i]));
            }
            "--live-claims" => {
                i += 1;
                if i >= args.len() {
                    eprintln!("validate_register: --live-claims requires a comma-separated list");
                    process::exit(3);
                }
                live_claims = Some(parse_csv_set(&args[i]));
            }
            "--live-evidence" => {
                i += 1;
                if i >= args.len() {
                    eprintln!("validate_register: --live-evidence requires a comma-separated list");
                    process::exit(3);
                }
                live_evidence = Some(parse_csv_set(&args[i]));
            }
            "--retired" => {
                i += 1;
                if i >= args.len() {
                    eprintln!("validate_register: --retired requires a comma-separated list");
                    process::exit(3);
                }
                retired_ids.extend(parse_csv_set(&args[i]));
            }
            "--previous" => {
                i += 1;
                if i >= args.len() {
                    eprintln!("validate_register: --previous requires a comma-separated list");
                    process::exit(3);
                }
                previous_ids = Some(parse_csv_set(&args[i]));
            }
            "--due-at" => {
                i += 1;
                if i >= args.len() {
                    eprintln!("validate_register: --due-at requires a milestone (M0..M8)");
                    process::exit(3);
                }
                due_at_milestone = Some(args[i].clone());
            }
            "--enforce-recurring" => enforce_recurring = true,
            "--json" => as_json = true,
            other => {
                eprintln!("validate_register: unknown argument `{other}`");
                process::exit(3);
            }
        }
        i += 1;
    }

    let Some(file) = file_path else {
        eprintln!("validate_register: missing required --file <path>");
        process::exit(3);
    };

    let resolved_root = repo_root.or_else(|| detect_repo_root(&file));
    let controls = if let Some(ref cp) = controls_path {
        let bytes = match fs::read(cp) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("validate_register: failed reading controls {}: {e}", cp.display());
                process::exit(3);
            }
        };
        match Controls::parse_bytes(&bytes, &cp.to_string_lossy()) {
            Ok(c) => Some(c),
            Err(e) => {
                eprintln!("validate_register: failed parsing controls {}: {e}", cp.display());
                process::exit(3);
            }
        }
    } else if let Some(ref root) = resolved_root {
        match Controls::load(root) {
            Ok((c, true)) => Some(c),
            Ok((_, false)) => None,
            Err(e) => {
                eprintln!("validate_register: failed loading workspace controls: {e}");
                process::exit(3);
            }
        }
    } else {
        None
    };

    let ctx = ValidationContext {
        repo_root: resolved_root,
        controls,
        live_claims,
        live_evidence,
        retired_ids,
        previous_ids,
        enforce_recurring_flags: enforce_recurring,
    };

    let file_str = file.to_string_lossy().into_owned();
    match load_with_context(&file, &ctx) {
        Ok(reg) => {
            let due_ids: Option<Vec<String>> = due_at_milestone.as_deref().map(|m| {
                reg.due_gates(m)
                    .into_iter()
                    .map(|g| g.id.clone())
                    .collect()
            });
            if as_json {
                let report = ValidationReport {
                    due_at: due_ids.as_deref(),
                    file: &file_str,
                    gate_count: reg.gates.len(),
                    ok: true,
                    schema: &reg.schema,
                    violations: &[],
                };
                println!("{}", write_json(&report));
            } else {
                println!(
                    "validate_register: ok ({} gates, schema {})",
                    reg.gates.len(),
                    reg.schema
                );
                if let Some(ref due) = due_ids {
                    println!("due: {}", due.join(","));
                }
            }
            process::exit(0);
        }
        Err(violations) => {
            if as_json {
                let report = ValidationReport {
                    due_at: None,
                    file: &file_str,
                    gate_count: 0,
                    ok: false,
                    schema: GATES_SCHEMA,
                    violations: &violations,
                };
                println!("{}", write_json(&report));
            } else {
                for v in &violations {
                    eprintln!("{v}");
                }
            }
            process::exit(3);
        }
    }
}
