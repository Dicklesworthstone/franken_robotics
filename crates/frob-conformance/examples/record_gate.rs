#![deny(unsafe_code)]

//! Gate recorder example target (`N38b-19`, `ADR-GAUNT-01`).
//!
//! Sole writer of `[[gate.part]]` measurements in `registries/gates.toml`:
//!   cargo run -p frob-conformance --example record_gate -- --gate <id> --receipt <hash> [--receipt <hash> ...] [--part <milestone>]
//!   cargo run -p frob-conformance --example record_gate -- reaffirm --registry <path> ...
//!
//! Exits `0` on success (or idempotent no-op) and `3` on any refusal or unknown mode word,
//! printing `reason=<id> ...` and leaving `registries/gates.toml` byte-identical.

use std::collections::BTreeSet;
use std::env;
use std::path::{Path, PathBuf};
use std::process;
use std::time::Duration;

use frob_conformance::register::record::{
    DEFAULT_LOCK_TIMEOUT, RecordOptions, record_gate, run_reaffirm_lock_dispatch,
};
use frob_core::testkit::registry as testkit_registry;

fn usage() -> &'static str {
    "Usage:\n  record_gate [record] --gate <id> --receipt <hash> [--receipt <hash> ...] [--part <milestone>]\n              [--repo-root <dir>] [--registry <path>] [--controls <path>] [--receipts-dir <dir>]\n              [--bead <id>] [--bead-log <path>] [--evidence <id>] [--live-claims <csv>] [--live-evidence <csv>]\n              [--date <YYYY-MM-DD>] [--session <session:name>] [--readme <path>] [--lock-timeout-ms <ms>]\n  record_gate reaffirm --registry <path> [--session <session:name>] [--lock-timeout-ms <ms>]\nModes: (default) record, reaffirm"
}

fn detect_repo_root(registry: Option<&Path>) -> PathBuf {
    if let Some(p) = registry {
        let mut cur = p.parent();
        while let Some(d) = cur {
            if d.join(".git").exists() || d.join("franken-stack.lock").is_file() {
                return d.to_path_buf();
            }
            cur = d.parent();
        }
    }
    testkit_registry::root()
        .or_else(|_| env::current_dir())
        .unwrap_or_else(|_| PathBuf::from("."))
}

fn parse_csv(raw: &str) -> BTreeSet<String> {
    raw.split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_owned)
        .collect()
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() || matches!(args[0].as_str(), "-h" | "--help") {
        eprintln!("{}", usage());
        process::exit(if args.is_empty() { 3 } else { 0 });
    }

    // Mode-word dispatch: no mode word (first arg starts with `-`) -> record;
    // `reaffirm` -> lock-acquiring reaffirm mode (N29bp1-01); any other leading word -> refuse.
    let (mode, start_idx) = if args[0].starts_with('-') {
        ("record", 0usize)
    } else {
        match args[0].as_str() {
            "record" => ("record", 1usize),
            "reaffirm" => ("reaffirm", 1usize),
            other => {
                eprintln!("record_gate: unknown mode `{other}`\n{}", usage());
                process::exit(3);
            }
        }
    };

    if mode == "reaffirm" {
        let mut registry_path: Option<PathBuf> = None;
        let mut session =
            env::var("FROB_SESSION").unwrap_or_else(|_| "session:reaffirm".to_owned());
        let mut timeout = DEFAULT_LOCK_TIMEOUT;
        let mut i = start_idx;
        while i < args.len() {
            match args[i].as_str() {
                "--registry" => {
                    i += 1;
                    if i >= args.len() {
                        eprintln!("record_gate reaffirm: --registry requires a path");
                        process::exit(3);
                    }
                    registry_path = Some(PathBuf::from(&args[i]));
                }
                "--session" => {
                    i += 1;
                    if i >= args.len() {
                        eprintln!("record_gate reaffirm: --session requires a value");
                        process::exit(3);
                    }
                    session = args[i].clone();
                }
                "--lock-timeout-ms" => {
                    i += 1;
                    if i >= args.len() {
                        eprintln!("record_gate reaffirm: --lock-timeout-ms requires an integer");
                        process::exit(3);
                    }
                    let ms: u64 = args[i].parse().unwrap_or_else(|_| {
                        eprintln!("record_gate reaffirm: invalid --lock-timeout-ms");
                        process::exit(3);
                    });
                    timeout = Duration::from_millis(ms);
                }
                "--logs" | "--store" => {
                    i += 1;
                    if i >= args.len() {
                        eprintln!("record_gate reaffirm: missing argument value");
                        process::exit(3);
                    }
                }
                "--json" => {}
                other => {
                    eprintln!("record_gate reaffirm: unknown argument `{other}`");
                    process::exit(3);
                }
            }
            i += 1;
        }
        let reg =
            registry_path.unwrap_or_else(|| detect_repo_root(None).join("registries/gates.toml"));
        match run_reaffirm_lock_dispatch(&reg, &session, timeout) {
            Ok(lock_path) => {
                println!(
                    "reaffirm: lock acquired and released cleanly ({})",
                    lock_path.display()
                );
                process::exit(0);
            }
            Err(err) => {
                eprintln!("{err}");
                process::exit(3);
            }
        }
    }

    // Record mode.
    let mut gate_id: Option<String> = None;
    let mut part: Option<String> = None;
    let mut receipts: Vec<String> = Vec::new();
    let mut repo_root: Option<PathBuf> = None;
    let mut registry_path: Option<PathBuf> = None;
    let mut controls_path: Option<PathBuf> = None;
    let mut history_path: Option<PathBuf> = None;
    let mut receipts_dir: Option<PathBuf> = None;
    let mut bead_id: Option<String> = None;
    let mut bead_logs: Vec<PathBuf> = Vec::new();
    let mut evidence: Vec<String> = Vec::new();
    let mut live_claims: Option<BTreeSet<String>> = None;
    let mut live_evidence: Option<BTreeSet<String>> = None;
    let mut date: Option<String> = None;
    let mut session: Option<String> = None;
    let mut readme_path: Option<PathBuf> = None;
    let mut timeout = DEFAULT_LOCK_TIMEOUT;

    let mut i = start_idx;
    while i < args.len() {
        let next_val = |idx: &mut usize, flag: &str| -> String {
            *idx += 1;
            if *idx >= args.len() {
                eprintln!("record_gate: {flag} requires a value");
                process::exit(3);
            }
            args[*idx].clone()
        };
        match args[i].as_str() {
            "--gate" => gate_id = Some(next_val(&mut i, "--gate")),
            "--part" => part = Some(next_val(&mut i, "--part")),
            "--receipt" => receipts.push(next_val(&mut i, "--receipt")),
            "--repo-root" => repo_root = Some(PathBuf::from(next_val(&mut i, "--repo-root"))),
            "--registry" => registry_path = Some(PathBuf::from(next_val(&mut i, "--registry"))),
            "--controls" => controls_path = Some(PathBuf::from(next_val(&mut i, "--controls"))),
            "--control-history" => {
                history_path = Some(PathBuf::from(next_val(&mut i, "--control-history")))
            }
            "--receipts-dir" => {
                receipts_dir = Some(PathBuf::from(next_val(&mut i, "--receipts-dir")))
            }
            "--bead" => bead_id = Some(next_val(&mut i, "--bead")),
            "--bead-log" => bead_logs.push(PathBuf::from(next_val(&mut i, "--bead-log"))),
            "--evidence" => evidence.push(next_val(&mut i, "--evidence")),
            "--live-claims" => live_claims = Some(parse_csv(&next_val(&mut i, "--live-claims"))),
            "--live-evidence" => {
                live_evidence = Some(parse_csv(&next_val(&mut i, "--live-evidence")))
            }
            "--date" => date = Some(next_val(&mut i, "--date")),
            "--session" => session = Some(next_val(&mut i, "--session")),
            "--readme" => readme_path = Some(PathBuf::from(next_val(&mut i, "--readme"))),
            "--lock-timeout-ms" => {
                let raw = next_val(&mut i, "--lock-timeout-ms");
                let ms: u64 = raw.parse().unwrap_or_else(|_| {
                    eprintln!("record_gate: invalid --lock-timeout-ms `{raw}`");
                    process::exit(3);
                });
                timeout = Duration::from_millis(ms);
            }
            other => {
                eprintln!("record_gate: unknown argument `{other}`\n{}", usage());
                process::exit(3);
            }
        }
        i += 1;
    }

    let Some(gate_id) = gate_id else {
        eprintln!("record_gate: missing required --gate <id>\n{}", usage());
        process::exit(3);
    };
    let root = repo_root.unwrap_or_else(|| detect_repo_root(registry_path.as_deref()));

    let mut opts = RecordOptions::new(root, gate_id, receipts);
    opts.registry_path = registry_path;
    opts.controls_path = controls_path;
    opts.control_history_path = history_path;
    opts.receipts_dir = receipts_dir;
    opts.part_milestone = part;
    opts.bead_id = bead_id;
    opts.bead_logs = bead_logs;
    opts.evidence = evidence;
    opts.live_claims = live_claims;
    opts.live_evidence = live_evidence;
    opts.date = date;
    opts.session = session;
    opts.readme_path = readme_path;
    opts.lock_timeout = timeout;

    match record_gate(&opts) {
        Ok(out) => {
            println!(
                "recorded gate={} part={} status={} measured=\"{}\" receipt={} idempotent={}",
                out.gate_id,
                out.part.milestone,
                out.part.status.as_str(),
                out.part.measured,
                out.part.receipt,
                out.idempotent_noop
            );
            for f in &out.regenerated_files {
                println!("regenerated: {}", f.display());
            }
            process::exit(0);
        }
        Err(refusal) => {
            eprintln!("{refusal}");
            process::exit(3);
        }
    }
}
