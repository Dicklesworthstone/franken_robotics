#![deny(unsafe_code)]

//! Read-only CLI validator for `registries/gates.toml` (`frob.gates/1`, `N40-01`, `N40-03`).
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
//!     [--kill-due <M0..M8>]
//!     [--funded <csv>]
//!     [--apply-decisions]
//!     [--live-ids <path>]
//!     [--print-seed-hashes]
//!     [--seeds <path>]
//!     [--receipt-times <path>]
//!     [--implementers <csv>]
//!     [--check-adr-files]
//!     [--json]
//!
//! Exits `0` when valid and `3` on any load error, rule violation, or `--live-ids` mismatch
//! (naming the offending gate row and reason slug). Never calls `register::record` or
//! `register::write` (one-recorder invariant, `N38b-19`).

use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;

use frob_conformance::register::{
    GATES_SCHEMA, KillDueContext, KillDueItem, KillDueStatus, KillMode, KillValidationContext,
    SeededKillRow, ValidationContext, Violation, due_kill_gates, kill_row_seed_hash,
    load_with_context, parse_live_ids_file, parse_register_bytes, render_readme_status_table,
    render_register_printout, validate, verify_live_kill_ids,
};
use frob_core::canon::{self, Canonical};
use frob_core::registry::toml::{self, Fields, Value};
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

fn load_seeds_file(path: &Path) -> Result<BTreeMap<String, SeededKillRow>, String> {
    let bytes = fs::read(path).map_err(|e| format!("failed reading seeds {}: {e}", path.display()))?;
    let root = toml::parse(&bytes)
        .map_err(|e| format!("failed parsing seeds {}: {e}", path.display()))?;
    let mut top = Fields::new(root, "seeds");
    let raw_seeds = top
        .opt_array("seed")
        .map_err(|e| e.to_string())?
        .unwrap_or_default();
    top.finish().map_err(|e| e.to_string())?;

    let mut map = BTreeMap::new();
    for (idx, sp) in raw_seeds.into_iter().enumerate() {
        let Value::Table(tbl) = sp.value else {
            return Err(format!("seed[{idx}] must be a table"));
        };
        let mut f = Fields::new(tbl, format!("seed[{idx}]"));
        let id = f.require_str("id").map_err(|e| e.to_string())?;
        let trigger = f.require_str("trigger").map_err(|e| e.to_string())?;
        let fallback = f.require_str("fallback").map_err(|e| e.to_string())?;
        let decided = f.opt_str("decided").map_err(|e| e.to_string())?;
        let mode = match f.opt_str("mode").map_err(|e| e.to_string())? {
            Some(raw) => match KillMode::parse(&raw) {
                Some(m) => m,
                None => return Err(f.fail(format!("seed[{idx}] unknown kill mode `{raw}`"))),
            },
            None => KillMode::Exit,
        };
        let reads_raw = f
            .opt_array("reads")
            .map_err(|e| e.to_string())?
            .unwrap_or_default();
        let mut reads = Vec::with_capacity(reads_raw.len());
        for rsp in reads_raw {
            match rsp.value {
                Value::Str(s) => reads.push(s),
                other => {
                    return Err(f.fail(format!(
                        "seed[{idx}] reads elements must be strings, got {}",
                        other.type_name()
                    )));
                }
            }
        }
        f.finish().map_err(|e| e.to_string())?;
        map.insert(
            id,
            SeededKillRow {
                trigger,
                fallback,
                decided,
                mode,
                reads,
            },
        );
    }
    Ok(map)
}

fn load_receipt_times_file(path: &Path) -> Result<BTreeMap<String, String>, String> {
    let text = fs::read_to_string(path)
        .map_err(|e| format!("failed reading receipt-times {}: {e}", path.display()))?;
    let mut map = BTreeMap::new();
    for (idx, raw) in text.lines().enumerate() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut parts = line.split_whitespace();
        let Some(gate_id) = parts.next() else {
            continue;
        };
        let Some(ts) = parts.next() else {
            return Err(format!(
                "{}:{}: expected `<gate_id> <iso_timestamp>`",
                path.display(),
                idx + 1
            ));
        };
        map.insert(gate_id.to_owned(), ts.to_owned());
    }
    Ok(map)
}

struct ValidationReport<'a> {
    due_at: Option<&'a [String]>,
    file: &'a str,
    gate_count: usize,
    kill_due: Option<&'a [KillDueItem]>,
    live_ids_error: Option<&'a str>,
    ok: bool,
    readme_table: Option<&'a str>,
    register_printout: Option<&'a str>,
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
            if let Some(kdue) = self.kill_due {
                let missing = kdue.iter().filter(|it| it.status.is_missing()).count();
                o.array("kill_due", |a| {
                    for item in kdue {
                        a.push_object(|ko| {
                            ko.str("effective_decided", &item.effective_decided)?;
                            ko.str("exit", &item.exit)?;
                            ko.str("gate_id", &item.gate_id)?;
                            ko.str("mode", item.mode.as_str())?;
                            let status_str = match &item.status {
                                KillDueStatus::Decided { .. } => "decided",
                                KillDueStatus::Deferred { .. } => "deferred",
                                KillDueStatus::DueWhenFundingRequested => {
                                    "due-when-funding-requested"
                                }
                                KillDueStatus::Missing => "missing",
                            };
                            ko.str("status", status_str)?;
                            Ok(())
                        })?;
                    }
                    Ok(())
                })?;
                o.int53("kill_due_count", kdue.len() as i64)?;
                o.int53("kill_due_missing_count", missing as i64)?;
            }
            if let Some(err) = self.live_ids_error {
                o.str("live_ids_error", err)?;
            }
            o.bool("ok", self.ok)?;
            if let Some(tbl) = self.readme_table {
                o.str("readme_table", tbl)?;
            }
            if let Some(pr) = self.register_printout {
                o.str("register_printout", pr)?;
            }
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
    let mut kill_due_milestone: Option<String> = None;
    let mut funded = BTreeSet::new();
    let mut apply_decisions_flag = false;
    let mut live_ids_path: Option<PathBuf> = None;
    let mut print_seed_hashes = false;
    let mut seeds_path: Option<PathBuf> = None;
    let mut receipt_times_path: Option<PathBuf> = None;
    let mut implementers = BTreeSet::new();
    let mut check_adr_files = false;
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
            "--kill-due" => {
                i += 1;
                if i >= args.len() {
                    eprintln!("validate_register: --kill-due requires a milestone (M0..M8)");
                    process::exit(3);
                }
                kill_due_milestone = Some(args[i].clone());
            }
            "--funded" => {
                i += 1;
                if i >= args.len() {
                    eprintln!("validate_register: --funded requires a comma-separated list");
                    process::exit(3);
                }
                funded.extend(parse_csv_set(&args[i]));
            }
            "--apply-decisions" => apply_decisions_flag = true,
            "--live-ids" => {
                i += 1;
                if i >= args.len() {
                    eprintln!("validate_register: --live-ids requires a path");
                    process::exit(3);
                }
                live_ids_path = Some(PathBuf::from(&args[i]));
            }
            "--print-seed-hashes" => print_seed_hashes = true,
            "--seeds" => {
                i += 1;
                if i >= args.len() {
                    eprintln!("validate_register: --seeds requires a path");
                    process::exit(3);
                }
                seeds_path = Some(PathBuf::from(&args[i]));
            }
            "--receipt-times" => {
                i += 1;
                if i >= args.len() {
                    eprintln!("validate_register: --receipt-times requires a path");
                    process::exit(3);
                }
                receipt_times_path = Some(PathBuf::from(&args[i]));
            }
            "--implementers" => {
                i += 1;
                if i >= args.len() {
                    eprintln!("validate_register: --implementers requires a comma-separated list");
                    process::exit(3);
                }
                implementers.extend(parse_csv_set(&args[i]));
            }
            "--check-adr-files" => check_adr_files = true,
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

    let file_str = file.to_string_lossy().into_owned();

    if print_seed_hashes {
        let bytes = match fs::read(&file) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("validate_register: failed reading {}: {e}", file.display());
                process::exit(3);
            }
        };
        let reg = match parse_register_bytes(&bytes, &file_str) {
            Ok(r) => r,
            Err(errs) => {
                for v in &errs {
                    eprintln!("{v}");
                }
                process::exit(3);
            }
        };
        for g in &reg.gates {
            if g.is_kill() {
                let trig = g.kill.trigger.as_deref().unwrap_or("");
                let fb = g.kill.fallback.as_deref().unwrap_or("");
                println!("{} {}", g.id, kill_row_seed_hash(trig, fb));
            }
        }
        process::exit(0);
    }

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

    let mut kill_ctx = KillValidationContext {
        global_implementers: implementers,
        check_adr_files_exist: check_adr_files,
        ..KillValidationContext::default()
    };
    if let Some(ref sp) = seeds_path {
        match load_seeds_file(sp) {
            Ok(m) => kill_ctx.seeded_kill_rows = Some(m),
            Err(e) => {
                eprintln!("validate_register: {e}");
                process::exit(3);
            }
        }
    }
    if let Some(ref rtp) = receipt_times_path {
        match load_receipt_times_file(rtp) {
            Ok(m) => kill_ctx.gate_earliest_receipt_ts = m,
            Err(e) => {
                eprintln!("validate_register: {e}");
                process::exit(3);
            }
        }
    }

    let ctx = ValidationContext {
        repo_root: resolved_root,
        controls,
        live_claims,
        live_evidence,
        retired_ids,
        previous_ids,
        enforce_recurring_flags: enforce_recurring,
        kill: kill_ctx,
    };

    if let Some(ref ids_file) = live_ids_path {
        let ids_src = match fs::read_to_string(ids_file) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("validate_register: failed reading --live-ids {}: {e}", ids_file.display());
                process::exit(3);
            }
        };
        let expectations = match parse_live_ids_file(&ids_src) {
            Ok(exps) => exps,
            Err(e) => {
                eprintln!("validate_register: invalid --live-ids {}: {e}", ids_file.display());
                process::exit(3);
            }
        };
        let bytes = match fs::read(&file) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("validate_register: failed reading {}: {e}", file.display());
                process::exit(3);
            }
        };
        let reg = match parse_register_bytes(&bytes, &file_str) {
            Ok(r) => r,
            Err(violations) => {
                if as_json {
                    let report = ValidationReport {
                        due_at: None,
                        file: &file_str,
                        gate_count: 0,
                        kill_due: None,
                        live_ids_error: violations.first().map(|v| v.message.as_str()),
                        ok: false,
                        readme_table: None,
                        register_printout: None,
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
        };
        if let Err(err_msg) = verify_live_kill_ids(&reg, &expectations) {
            if as_json {
                let report = ValidationReport {
                    due_at: None,
                    file: &file_str,
                    gate_count: reg.gates.len(),
                    kill_due: None,
                    live_ids_error: Some(&err_msg),
                    ok: false,
                    readme_table: None,
                    register_printout: None,
                    schema: &reg.schema,
                    violations: &[],
                };
                println!("{}", write_json(&report));
            } else {
                eprintln!("live-ids check failed: {err_msg}");
            }
            process::exit(3);
        }
        let violations = validate(&reg, &ctx);
        if !violations.is_empty() {
            if as_json {
                let report = ValidationReport {
                    due_at: None,
                    file: &file_str,
                    gate_count: reg.gates.len(),
                    kill_due: None,
                    live_ids_error: None,
                    ok: false,
                    readme_table: None,
                    register_printout: None,
                    schema: &reg.schema,
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
        if as_json {
            let report = ValidationReport {
                due_at: None,
                file: &file_str,
                gate_count: reg.gates.len(),
                kill_due: None,
                live_ids_error: None,
                ok: true,
                readme_table: None,
                register_printout: None,
                schema: &reg.schema,
                violations: &[],
            };
            println!("{}", write_json(&report));
        } else {
            println!(
                "validate_register: live-ids ok ({} checked, {} total gates)",
                expectations.len(),
                reg.gates.len()
            );
        }
        process::exit(0);
    }

    match load_with_context(&file, &ctx) {
        Ok(reg) => {
            let due_ids: Option<Vec<String>> = due_at_milestone.as_deref().map(|m| {
                reg.due_gates(m)
                    .into_iter()
                    .map(|g| g.id.clone())
                    .collect()
            });
            let kill_due: Option<Vec<KillDueItem>> = kill_due_milestone.as_deref().map(|m| {
                let kctx = KillDueContext {
                    funding_requested: funded.clone(),
                };
                due_kill_gates(&reg, m, &kctx)
            });
            let readme_tbl = apply_decisions_flag.then(|| render_readme_status_table(&reg));
            let reg_printout = apply_decisions_flag.then(|| render_register_printout(&reg));
            if as_json {
                let report = ValidationReport {
                    due_at: due_ids.as_deref(),
                    file: &file_str,
                    gate_count: reg.gates.len(),
                    kill_due: kill_due.as_deref(),
                    live_ids_error: None,
                    ok: true,
                    readme_table: readme_tbl.as_deref(),
                    register_printout: reg_printout.as_deref(),
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
                if let Some(ref kdue) = kill_due {
                    let missing = kdue.iter().filter(|it| it.status.is_missing()).count();
                    println!("kill_due: {} total, {} missing", kdue.len(), missing);
                }
                if let Some(ref tbl) = readme_tbl {
                    println!("{tbl}");
                }
                if let Some(ref pr) = reg_printout {
                    println!("{pr}");
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
                    kill_due: None,
                    live_ids_error: None,
                    ok: false,
                    readme_table: None,
                    register_printout: None,
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
