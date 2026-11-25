#![deny(unsafe_code)]

//! Read-only base-record validator, round-trip checker, and count reporter for
//! `registries/gates.toml` (`N38b-18`, `frob.gates/1`).
//!
//! Strictly read-only: never writes `registries/gates.toml` and never invokes
//! `register::record` or `register::write` (`ADR-GAUNT-01`).
//!
//! Usage:
//!   `cargo run -p frob-conformance --example gate_base_check -- --file <gates.toml> [--counts <gate_counts.toml>] [--roundtrip] [--json]`

use std::fmt::Write as _;
use std::path::PathBuf;
use std::process::ExitCode;

use frob_conformance::register::{BaseDocument, Claim, GateCountsExpectation, Status};

fn json_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for ch in s.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => {
                let _ = write!(out, "\\u{:04x}", c as u32);
            }
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

fn emit_error(json_mode: bool, gate_id: Option<&str>, message: &str) -> ExitCode {
    if json_mode {
        let gate_field = match gate_id {
            Some(id) => json_escape(id),
            None => "null".to_owned(),
        };
        println!(
            "{{\"error\":{},\"gate_id\":{},\"ok\":false}}",
            json_escape(message),
            gate_field
        );
    } else {
        eprintln!("gate_base_check error: {message}");
    }
    ExitCode::from(3)
}

fn main() -> ExitCode {
    let mut file: Option<PathBuf> = None;
    let mut counts_file: Option<PathBuf> = None;
    let mut roundtrip = false;
    let mut json_mode = false;

    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--file" => {
                let Some(p) = args.next() else {
                    return emit_error(json_mode, None, "--file requires a path");
                };
                file = Some(PathBuf::from(p));
            }
            "--counts" => {
                let Some(p) = args.next() else {
                    return emit_error(json_mode, None, "--counts requires a path");
                };
                counts_file = Some(PathBuf::from(p));
            }
            "--roundtrip" => roundtrip = true,
            "--json" => json_mode = true,
            other => {
                return emit_error(json_mode, None, &format!("unknown argument `{other}`"));
            }
        }
    }

    let Some(path) = file else {
        return emit_error(json_mode, None, "missing required --file <path>");
    };

    let doc = match BaseDocument::load(&path) {
        Ok(d) => d,
        Err(e) => return emit_error(json_mode, e.gate_id.as_deref(), &e.to_string()),
    };

    if roundtrip {
        let printed_1 = doc.print_toml();
        let reparsed = match BaseDocument::parse_str(&printed_1, "roundtrip") {
            Ok(d) => d,
            Err(e) => return emit_error(json_mode, e.gate_id.as_deref(), &e.to_string()),
        };
        if reparsed != doc || reparsed.print_toml() != printed_1 {
            return emit_error(json_mode, None, "base record print/parse roundtrip mismatch");
        }
    }

    let counts_ok = true;
    let mut plan_total: Option<usize> = None;
    if let Some(cpath) = counts_file {
        let exp = match GateCountsExpectation::load(&cpath) {
            Ok(e) => e,
            Err(e) => return emit_error(json_mode, e.gate_id.as_deref(), &e.to_string()),
        };
        plan_total = Some(exp.plan_inventory.total);
        if let Err(mismatch) = exp.check_latest(&doc) {
            return emit_error(
                json_mode,
                mismatch.extra_ids.first().map(String::as_str),
                &mismatch.to_string(),
            );
        }
    }

    let by_kind = doc.counts_by_kind();
    let by_status = doc.counts_by_status();
    let by_claim = doc.counts_by_claim();

    if json_mode {
        let status_entries: Vec<String> = Status::ALL
            .iter()
            .map(|s| format!("{}:{}", json_escape(s.as_str()), by_status.get(s).copied().unwrap_or(0)))
            .collect();
        let claim_entries: Vec<String> = Claim::ALL
            .iter()
            .map(|c| format!("{}:{}", json_escape(c.as_str()), by_claim.get(c).copied().unwrap_or(0)))
            .collect();
        let plan_json = match plan_total {
            Some(n) => n.to_string(),
            None => "null".to_owned(),
        };
        println!(
            "{{\"by_claim\":{{{}}},\"by_kind\":{{\"admission\":{},\"convergence\":{},\"kill\":{},\"program\":{},\"total\":{}}},\"by_status\":{{{}}},\"counts_ok\":{},\"ok\":true,\"plan_inventory_total\":{},\"schema\":{}}}",
            claim_entries.join(","),
            by_kind.admission,
            by_kind.convergence,
            by_kind.kill,
            by_kind.program,
            by_kind.total,
            status_entries.join(","),
            counts_ok,
            plan_json,
            json_escape(&doc.schema),
        );
    } else {
        println!(
            "ok schema={} total={} convergence={} kill={} admission={} program={}",
            doc.schema,
            by_kind.total,
            by_kind.convergence,
            by_kind.kill,
            by_kind.admission,
            by_kind.program,
        );
    }

    ExitCode::SUCCESS
}
