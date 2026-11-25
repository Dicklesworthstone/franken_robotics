#![deny(unsafe_code)]

//! Validated write path for `registries/gates.toml` (`N40-01` step 5).
//!
//! Runs every register rule and all ten write-time refusals against a candidate snapshot
//! BEFORE writing any bytes to disk. On any refusal the target file is left bit-identical.

use std::fs;
use std::path::Path;
use std::process;
use std::sync::atomic::{AtomicU64, Ordering};

use crate::register::validate::{
    Part, Register, ValidationContext, Violation, load_with_context, validate,
};

/// Apply `part` to `gate_id` on an in-memory clone of `register` and validate every rule
/// and refusal under `ctx`.
pub fn record_in_memory(
    register: &Register,
    gate_id: &str,
    part: Part,
    ctx: &ValidationContext,
) -> Result<Register, Vec<Violation>> {
    let mut candidate = register.clone();
    let Some(gate) = candidate.get_mut(gate_id) else {
        return Err(vec![Violation {
            reason: "unknown-gate",
            harness_reason: None,
            gate_id: Some(gate_id.to_owned()),
            field: Some("id".to_owned()),
            message: format!("gate `{gate_id}` does not exist in register"),
            repair: "register the gate row before recording a part",
        }]);
    };
    if gate.is_kill() {
        return Err(vec![Violation {
            reason: "kill-row-measurement-field",
            harness_reason: None,
            gate_id: Some(gate_id.to_owned()),
            field: Some("part".to_owned()),
            message: format!("cannot record a [[gate.part]] on kill row `{gate_id}`"),
            repair: "record kill-gate decisions via N40-03 decision fields",
        }]);
    }

    if let Some(existing) = gate
        .parts
        .iter_mut()
        .find(|p| p.milestone == part.milestone)
    {
        *existing = part;
    } else {
        gate.parts.push(part);
    }

    let violations = validate(&candidate, ctx);
    if violations.is_empty() {
        Ok(candidate)
    } else {
        Err(violations)
    }
}

/// Record a validated `[[gate.part]]` into `path`, refusing and leaving `path` untouched on
/// any rule violation or write-time refusal (`N40-01`).
///
/// Writes canonically printed TOML via sibling staging file + atomic rename so partial
/// writes are impossible.
pub fn record(
    path: &Path,
    gate_id: &str,
    part: Part,
    ctx: &ValidationContext,
) -> Result<Register, Vec<Violation>> {
    let current = load_with_context(path, ctx)?;
    let updated = record_in_memory(&current, gate_id, part, ctx)?;
    write_atomic(path, &updated.print_toml())?;
    Ok(updated)
}

fn write_atomic(path: &Path, content: &str) -> Result<(), Vec<Violation>> {
    static SEQ: AtomicU64 = AtomicU64::new(0);
    let n = SEQ.fetch_add(1, Ordering::Relaxed);
    let pid = process::id();
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    let file_name = path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("gates.toml");
    let staging = parent.join(format!(".{file_name}.tmp.{pid}.{n}"));

    fs::write(&staging, content.as_bytes()).map_err(|e| {
        vec![Violation {
            reason: "io-error",
            harness_reason: None,
            gate_id: None,
            field: None,
            message: format!("failed writing staging file {}: {e}", staging.display()),
            repair: "check directory permissions and disk space",
        }]
    })?;
    fs::rename(&staging, path).map_err(|e| {
        vec![Violation {
            reason: "io-error",
            harness_reason: None,
            gate_id: None,
            field: None,
            message: format!(
                "failed atomically renaming {} -> {}: {e}",
                staging.display(),
                path.display()
            ),
            repair: "check filesystem permissions on target directory",
        }]
    })?;
    Ok(())
}
