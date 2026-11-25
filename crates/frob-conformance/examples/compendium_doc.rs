#![deny(unsafe_code)]

//! Deterministic renderer for `docs/math/compendium.md` from `registries/compendium/d*.toml` (`N12-41`).
//!
//! Shared across all ten compendium sections (`D.1` through `D.10`).
//! Usage:
//!   cargo run -p frob-conformance --example compendium_doc -- --out <path> [--root <dir>]

use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::ExitCode;

use frob_conformance::compendium::registry;

fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut out_path: Option<PathBuf> = None;
    let mut root_override: Option<PathBuf> = None;

    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--out" => {
                i += 1;
                let v = args.get(i).ok_or("--out requires a path")?;
                out_path = Some(PathBuf::from(v));
            }
            "--root" => {
                i += 1;
                let v = args.get(i).ok_or("--root requires a directory")?;
                root_override = Some(PathBuf::from(v));
            }
            "-h" | "--help" => {
                println!(
                    "Usage: compendium_doc [--out <path>] [--root <dir>]\n\
                     Renders docs/math/compendium.md deterministically from registries/compendium/d*.toml."
                );
                return Ok(());
            }
            other => {
                return Err(format!("unknown argument: {other}"));
            }
        }
        i += 1;
    }

    let root = match root_override {
        Some(r) => r,
        None => registry::workspace_root()?,
    };
    let rendered = registry::render_compendium_doc(&root)?;

    match out_path {
        Some(path) => {
            if let Some(parent) = path.parent()
                && !parent.as_os_str().is_empty()
            {
                fs::create_dir_all(parent)
                    .map_err(|e| format!("create parent dir {}: {e}", parent.display()))?;
            }
            fs::write(&path, rendered.as_bytes())
                .map_err(|e| format!("write {}: {e}", path.display()))?;
        }
        None => {
            io::stdout()
                .write_all(rendered.as_bytes())
                .map_err(|e| format!("write stdout: {e}"))?;
        }
    }
    Ok(())
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("compendium_doc: {err}");
            ExitCode::from(2)
        }
    }
}
