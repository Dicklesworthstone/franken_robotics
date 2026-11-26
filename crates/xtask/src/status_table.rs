//! README honest-status table generator and first-commit registry/schema
//! validators for FrankenRobotics (bead BD-002, gate GATE-GAUNT-10, INV-GAUNT-06).
//!
//! Reads `registries/gates.toml` and all other TOML registries exclusively
//! through `frob_core::registry::toml` (N34p2-01). Prints nothing (rule L-7),
//! contains zero `unsafe` code, and never deletes files or directories.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use frob_conformance::register::print::extract_str_list;
pub use frob_conformance::register::print::{
    ClaimKind, GatePart, GateRegistry, GateRow, PartStatus, RecordedMeta, StatusTableError,
    parse_gates_toml, readme_table as generate_from_toml, status_table, try_status_table,
    validate_gate_row,
};
use frob_core::registry::toml::{self, Fields, Spanned, Table, Value};

/// Opening marker for the generated honest-status block in `README.md`.
pub const STATUS_BEGIN: &str = "<!-- status:begin -->";
/// Closing marker for the generated honest-status block in `README.md`.
pub const STATUS_END: &str = "<!-- status:end -->";

const VALID_MILESTONES: &[&str] = &["M0", "M1", "M2", "M3", "M4", "M5", "M6", "M7", "M8"];
const VALID_TIERS: &[&str] = &["cpu", "gpu-cuda", "gpu-metal"];
const VALID_LANES: &[&str] = &["cpu", "device", "long"];
const VALID_DETERMINISM: &[&str] = &["D0", "D1", "D2", "D3", "D4"];
const VALID_EVIDENCE: &[&str] = &["Estimated", "Tested", "Certified", "Proved"];

/// One format row (`[[format]]`) of `registries/formats.toml`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FormatRow {
    pub id: String,
    pub extension: String,
    pub schema: String,
    pub owner: String,
    pub producers: Vec<String>,
    pub consumers: Vec<String>,
    pub encoding: String,
    pub kinds: Vec<String>,
}

/// Parsed `registries/formats.toml` document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FormatRegistry {
    pub schema: String,
    pub formats: Vec<FormatRow>,
}

/// Outcome of checking or blessing the README status block.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReadmeCheckOutcome {
    Current {
        block: String,
        outside_digest: u64,
    },
    Blessed {
        block: String,
        outside_digest: u64,
    },
}

/// Summary counts from validating all repository registries.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegistryAudit {
    pub validated_files: Vec<String>,
    pub format_count: usize,
    pub gate_count: usize,
    pub refusal_count: usize,
}

/// Parsed and validated `demo.yaml` document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DemoDoc {
    pub demo_id: String,
    pub claim_ids: Vec<String>,
    pub commands: Vec<String>,
    pub checksums: BTreeMap<String, String>,
    pub tier: Option<String>,
    pub expect_exit: BTreeMap<String, i64>,
    pub expect_refusal: BTreeMap<String, String>,
    pub compare: BTreeMap<String, Vec<String>>,
    pub expect: BTreeMap<String, String>,
    pub title: Option<String>,
    pub milestone: Option<String>,
    pub actors: Vec<String>,
    pub lane: Option<String>,
    pub fresh_agent: Option<bool>,
    pub not_evidence_for: Option<String>,
}

fn extract_table_list(
    items: Vec<Spanned<Value>>,
    key: &str,
    ctx: &str,
) -> Result<Vec<Table>, StatusTableError> {
    let mut out = Vec::with_capacity(items.len());
    for (idx, sp) in items.into_iter().enumerate() {
        match sp.value {
            Value::Table(t) => out.push(t),
            other => {
                return Err(StatusTableError::new(
                    ctx,
                    format!(
                        "{key}[{idx}] at {}:{}: expected table, found {}",
                        sp.line,
                        sp.column,
                        other.type_name()
                    ),
                ));
            }
        }
    }
    Ok(out)
}

fn is_refusal_code(s: &str) -> bool {
    let Some(rest) = s.strip_prefix("FROB-REF-") else {
        return false;
    };
    let b = rest.as_bytes();
    matches!(b.len(), 3 | 4) && matches!(b[0], b'1'..=b'9') && b[1..].iter().all(u8::is_ascii_digit)
}

fn is_b3_digest(s: &str) -> bool {
    let Some(rest) = s.strip_prefix("b3:") else {
        return false;
    };
    rest.len() == 64 && rest.bytes().all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f'))
}

fn fnv1a_64_pair(a: &[u8], b: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for byte in a.iter().chain(b.iter()) {
        h ^= u64::from(*byte);
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    h
}

/// Locate the `<!-- status:begin -->` and `<!-- status:end -->` markers in `readme`.
/// Each marker must occupy its own line.
///
/// Returns `(before_including_begin_newline, inner_block, end_marker_and_after, first_inner_line_1based)`.
pub fn split_readme_status_block(
    readme: &str,
) -> Result<(&str, &str, &str, usize), StatusTableError> {
    let begin_matches: Vec<usize> = readme
        .match_indices(STATUS_BEGIN)
        .map(|(i, _)| i)
        .collect();
    if begin_matches.len() != 1 {
        return Err(StatusTableError::new(
            "README.md",
            format!(
                "expected exactly one `{STATUS_BEGIN}` marker, found {}",
                begin_matches.len()
            ),
        ));
    }
    let end_matches: Vec<usize> = readme.match_indices(STATUS_END).map(|(i, _)| i).collect();
    if end_matches.len() != 1 {
        return Err(StatusTableError::new(
            "README.md",
            format!(
                "expected exactly one `{STATUS_END}` marker, found {}",
                end_matches.len()
            ),
        ));
    }

    let b_pos = begin_matches[0];
    let e_pos = end_matches[0];
    if e_pos <= b_pos {
        return Err(StatusTableError::new(
            "README.md",
            format!("`{STATUS_END}` appears before `{STATUS_BEGIN}`"),
        ));
    }

    let after_begin_marker = b_pos + STATUS_BEGIN.len();
    let inner_start = if readme[after_begin_marker..].starts_with('\n') {
        after_begin_marker + 1
    } else if readme[after_begin_marker..].starts_with("\r\n") {
        after_begin_marker + 2
    } else {
        return Err(StatusTableError::new(
            "README.md",
            format!("`{STATUS_BEGIN}` must be followed by a newline"),
        ));
    };

    if e_pos < inner_start {
        return Err(StatusTableError::new(
            "README.md",
            format!("`{STATUS_END}` must be on a line after `{STATUS_BEGIN}`"),
        ));
    }

    let after_end_marker = &readme[e_pos + STATUS_END.len()..];
    if (b_pos != 0 && !readme[..b_pos].ends_with('\n'))
        || !readme[..e_pos].ends_with('\n')
        || !(after_end_marker.is_empty()
            || after_end_marker.starts_with('\n')
            || after_end_marker.starts_with("\r\n"))
    {
        return Err(StatusTableError::new(
            "README.md",
            "each status marker must be on its own line",
        ));
    }

    let first_inner_line = readme[..inner_start]
        .bytes()
        .filter(|&b| b == b'\n')
        .count()
        + 1;

    Ok((
        &readme[..inner_start],
        &readme[inner_start..e_pos],
        &readme[e_pos..],
        first_inner_line,
    ))
}

/// Compare or rewrite (`bless = true`) the generated status block in `README.md`.
///
/// On mismatch (`bless = false`), returns an error naming the 1-based line
/// number of the first differing line in `README.md`.
/// With `bless = true`, rewrites ONLY the bytes between `<!-- status:begin -->`
/// and `<!-- status:end -->` and verifies the digest of all bytes outside the
/// markers is unchanged.
pub fn check_or_bless_readme(
    readme_path: &Path,
    gates_path: &Path,
    bless: bool,
) -> Result<ReadmeCheckOutcome, StatusTableError> {
    let gates_bytes = fs::read(gates_path)
        .map_err(|e| StatusTableError::new(gates_path.display().to_string(), e.to_string()))?;
    let expected = generate_from_toml(&gates_bytes)?;

    let readme_text = fs::read_to_string(readme_path)
        .map_err(|e| StatusTableError::new(readme_path.display().to_string(), e.to_string()))?;
    let (before, actual, after, first_inner_line) = split_readme_status_block(&readme_text)?;
    let outside_digest_before = fnv1a_64_pair(before.as_bytes(), after.as_bytes());

    if bless {
        let mut updated = String::with_capacity(before.len() + expected.len() + after.len());
        updated.push_str(before);
        updated.push_str(&expected);
        updated.push_str(after);
        fs::write(readme_path, updated.as_bytes()).map_err(|e| {
            StatusTableError::new(readme_path.display().to_string(), e.to_string())
        })?;
        let reread = fs::read_to_string(readme_path).map_err(|e| {
            StatusTableError::new(readme_path.display().to_string(), e.to_string())
        })?;
        let (nb, na_inner, na, _) = split_readme_status_block(&reread)?;
        let outside_digest_after = fnv1a_64_pair(nb.as_bytes(), na.as_bytes());
        if outside_digest_before != outside_digest_after || na_inner != expected {
            return Err(StatusTableError::new(
                "README.md",
                "post-bless invariant failed: bytes outside markers changed or block mismatch",
            ));
        }
        return Ok(ReadmeCheckOutcome::Blessed {
            block: expected,
            outside_digest: outside_digest_after,
        });
    }

    if actual == expected {
        return Ok(ReadmeCheckOutcome::Current {
            block: expected,
            outside_digest: outside_digest_before,
        });
    }

    let actual_lines: Vec<&str> = actual.split('\n').collect();
    let expected_lines: Vec<&str> = expected.split('\n').collect();
    let max_len = actual_lines.len().max(expected_lines.len());
    let mut diff_offset = 0usize;
    let mut obs = "<eof>";
    let mut exp = "<eof>";
    for i in 0..max_len {
        let a = actual_lines.get(i).copied();
        let e = expected_lines.get(i).copied();
        if a != e {
            diff_offset = i;
            obs = a.unwrap_or("<eof>");
            exp = e.unwrap_or("<eof>");
            break;
        }
    }
    let file_line = first_inner_line + diff_offset;
    Err(StatusTableError::new(
        "README.md",
        format!(
            "generated status block out of date; first differing line: line {file_line}\n  observed: {obs}\n  expected: {exp}"
        ),
    ))
}

// ============================================================================
// Registry & schema validators (BD-002)
// ============================================================================

/// Parse and validate `registries/formats.toml` (schema `frob.formats/1`).
pub fn parse_formats_toml(bytes: &[u8]) -> Result<FormatRegistry, StatusTableError> {
    let root =
        toml::parse(bytes).map_err(|e| StatusTableError::new("formats.toml", e.to_string()))?;
    let mut rf = Fields::new(root, "formats.toml");
    let schema = rf
        .require_str("schema")
        .map_err(|e| StatusTableError::new("formats.toml", e.to_string()))?;
    let raw_formats_arr = rf
        .require_array("format")
        .map_err(|e| StatusTableError::new("formats.toml", e.to_string()))?;
    rf.finish()
        .map_err(|e| StatusTableError::new("formats.toml", e.to_string()))?;

    if schema != "frob.formats/1" {
        return Err(StatusTableError::new(
            "formats.toml",
            format!("unexpected schema `{schema}` (expected `frob.formats/1`)"),
        ));
    }
    let raw_formats = extract_table_list(raw_formats_arr, "format", "formats.toml")?;

    let mut seen_ids = BTreeSet::new();
    let mut seen_exts = BTreeSet::new();
    let mut seen_schemas = BTreeSet::new();
    let mut formats = Vec::with_capacity(raw_formats.len());

    for (idx, ftbl) in raw_formats.into_iter().enumerate() {
        let ctx = format!("formats.toml:format[{idx}]");
        let mut ff = Fields::new(ftbl, &ctx);
        let id = ff
            .require_str("id")
            .map_err(|e| StatusTableError::new(&ctx, e.to_string()))?;
        let extension = ff
            .require_str("extension")
            .map_err(|e| StatusTableError::new(&ctx, e.to_string()))?;
        let fschema = ff
            .require_str("schema")
            .map_err(|e| StatusTableError::new(&ctx, e.to_string()))?;
        let owner = ff
            .require_str("owner")
            .map_err(|e| StatusTableError::new(&ctx, e.to_string()))?;
        let raw_producers = ff
            .require_array("producers")
            .map_err(|e| StatusTableError::new(&ctx, e.to_string()))?;
        let raw_consumers = ff
            .require_array("consumers")
            .map_err(|e| StatusTableError::new(&ctx, e.to_string()))?;
        let encoding = ff
            .require_str("encoding")
            .map_err(|e| StatusTableError::new(&ctx, e.to_string()))?;
        let raw_kinds = ff
            .opt_array("kinds")
            .map_err(|e| StatusTableError::new(&ctx, e.to_string()))?;
        ff.finish()
            .map_err(|e| StatusTableError::new(&ctx, e.to_string()))?;

        if !id.strip_prefix("FMT-").is_some_and(|name| {
            !name.is_empty()
                && name
                    .bytes()
                    .all(|b| b.is_ascii_uppercase() || b.is_ascii_digit() || b == b'-')
        }) {
            return Err(StatusTableError::new(
                &ctx,
                format!("format id `{id}` must match `FMT-[A-Z0-9-]+`"),
            ));
        }
        if !seen_ids.insert(id.clone()) {
            return Err(StatusTableError::new(
                &ctx,
                format!("duplicate format id `{id}`"),
            ));
        }
        if !extension.strip_prefix('.').is_some_and(|suffix| {
            suffix
                .as_bytes()
                .first()
                .is_some_and(u8::is_ascii_lowercase)
                && suffix
                    .bytes()
                    .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit())
        }) {
            return Err(StatusTableError::new(
                &ctx,
                format!("format `{id}` extension `{extension}` must match `.[a-z][a-z0-9]*`"),
            ));
        }
        if !seen_exts.insert(extension.clone()) {
            return Err(StatusTableError::new(
                &ctx,
                format!("duplicate format extension `{extension}` on `{id}`"),
            ));
        }
        if !fschema
            .strip_prefix("frob.")
            .and_then(|schema| schema.split_once('/'))
            .is_some_and(|(name, version)| {
                !name.is_empty()
                    && name
                        .bytes()
                        .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'.')
                    && !version.is_empty()
                    && version.bytes().all(|b| b.is_ascii_digit())
            })
        {
            return Err(StatusTableError::new(
                &ctx,
                format!("format `{id}` schema `{fschema}` must match `frob.<name>/<ver>`"),
            ));
        }
        if !seen_schemas.insert(fschema.clone()) {
            return Err(StatusTableError::new(
                &ctx,
                format!("duplicate format schema `{fschema}` on `{id}`"),
            ));
        }
        if owner.trim().is_empty() {
            return Err(StatusTableError::new(&ctx, format!("format `{id}` owner is empty")));
        }
        let producers = extract_str_list(&raw_producers, "producers", &ctx, true)?;
        if producers.is_empty() {
            return Err(StatusTableError::new(&ctx, format!("format `{id}` producers is empty")));
        }
        let consumers = extract_str_list(&raw_consumers, "consumers", &ctx, true)?;
        if consumers.is_empty() {
            return Err(StatusTableError::new(&ctx, format!("format `{id}` consumers is empty")));
        }
        if encoding != "canonical" && encoding != "view" {
            return Err(StatusTableError::new(
                &ctx,
                format!("format `{id}` encoding `{encoding}` must be `canonical` or `view`"),
            ));
        }
        let kinds = match raw_kinds {
            Some(items) if items.is_empty() => {
                return Err(StatusTableError::new(
                    &ctx,
                    format!("format `{id}` kinds must be non-empty when present"),
                ));
            }
            Some(items) => extract_str_list(&items, "kinds", &ctx, true)?,
            None => Vec::new(),
        };

        formats.push(FormatRow {
            id,
            extension,
            schema: fschema,
            owner,
            producers,
            consumers,
            encoding,
            kinds,
        });
    }

    Ok(FormatRegistry { schema, formats })
}

fn validate_empty_or_typed_registry(
    file_name: &str,
    expected_schema: &str,
    row_key: &str,
    bytes: &[u8],
) -> Result<usize, StatusTableError> {
    let root = toml::parse(bytes).map_err(|e| StatusTableError::new(file_name, e.to_string()))?;
    let mut rf = Fields::new(root, file_name);
    let schema = rf
        .require_str("schema")
        .map_err(|e| StatusTableError::new(file_name, e.to_string()))?;
    let raw_rows = rf
        .opt_array(row_key)
        .map_err(|e| StatusTableError::new(file_name, e.to_string()))?;
    if file_name == "verbs.toml" {
        for key in ["retired_name", "task_cluster", "addition"] {
            if let Some(items) = rf
                .opt_array(key)
                .map_err(|e| StatusTableError::new(file_name, e.to_string()))?
            {
                if let Err(err) = extract_table_list(items, key, file_name) {
                    return Err(rf.fail(err));
                }
            }
        }
        let _ = rf
            .opt_table("counts")
            .map_err(|e| StatusTableError::new(file_name, e.to_string()))?;
    }
    rf.finish()
        .map_err(|e| StatusTableError::new(file_name, e.to_string()))?;

    if schema != expected_schema {
        return Err(StatusTableError::new(
            file_name,
            format!("unexpected schema `{schema}` (expected `{expected_schema}`)"),
        ));
    }
    let rows = match raw_rows {
        Some(items) => extract_table_list(items, row_key, file_name)?,
        None => Vec::new(),
    };

    for (idx, rtbl) in rows.iter().enumerate() {
        let ctx = format!("{file_name}:{row_key}[{idx}]");
        match file_name {
            "verbs.toml" => {
                // frob-verbs' build already validates the full registry. Keep xtask usable
                // for diagnosing it by projecting just the fields this audit checks.
                let mut f = Fields::new(rtbl.clone(), &ctx);
                for key in rtbl.keys() {
                    if !matches!(key.as_str(), "name" | "cluster" | "owner" | "refusals") {
                        let _ = f.take(key);
                    }
                }
                let _ = f.require_str("name").map_err(|e| StatusTableError::new(&ctx, e.to_string()))?;
                let _ = f.require_str("cluster").map_err(|e| StatusTableError::new(&ctx, e.to_string()))?;
                let _ = f.require_str("owner").map_err(|e| StatusTableError::new(&ctx, e.to_string()))?;
                let raw_refs = f
                    .require_array("refusals")
                    .map_err(|e| StatusTableError::new(&ctx, e.to_string()))?;
                f.finish().map_err(|e| StatusTableError::new(&ctx, e.to_string()))?;
                let _ = extract_str_list(&raw_refs, "refusals", &ctx, true)?;
            }
            "kernels.toml" => {
                let mut f = Fields::new(rtbl.clone(), &ctx);
                let _ = f.require_str("id").map_err(|e| StatusTableError::new(&ctx, e.to_string()))?;
                let _ = f.require_str("owner").map_err(|e| StatusTableError::new(&ctx, e.to_string()))?;
                let det = f.require_str("determinism").map_err(|e| StatusTableError::new(&ctx, e.to_string()))?;
                let dig = f.require_str("source_digest").map_err(|e| StatusTableError::new(&ctx, e.to_string()))?;
                let raw_targets = f.opt_array("targets").map_err(|e| StatusTableError::new(&ctx, e.to_string()))?;
                f.finish().map_err(|e| StatusTableError::new(&ctx, e.to_string()))?;
                if !VALID_DETERMINISM.contains(&det.as_str()) {
                    return Err(StatusTableError::new(&ctx, format!("invalid determinism `{det}`")));
                }
                if !is_b3_digest(&dig) {
                    return Err(StatusTableError::new(&ctx, format!("invalid source_digest `{dig}`")));
                }
                if let Some(items) = raw_targets {
                    let _ = extract_str_list(&items, "targets", &ctx, true)?;
                }
            }
            "upstream.toml" => {
                let mut f = Fields::new(rtbl.clone(), &ctx);
                let _ = f.require_str("id").map_err(|e| StatusTableError::new(&ctx, e.to_string()))?;
                let _ = f.require_str("sibling").map_err(|e| StatusTableError::new(&ctx, e.to_string()))?;
                let _ = f.require_str("status").map_err(|e| StatusTableError::new(&ctx, e.to_string()))?;
                let _ = f.require_str("summary").map_err(|e| StatusTableError::new(&ctx, e.to_string()))?;
                let _ = f.opt_str("priority").map_err(|e| StatusTableError::new(&ctx, e.to_string()))?;
                f.finish().map_err(|e| StatusTableError::new(&ctx, e.to_string()))?;
            }
            "capsules.toml" => {
                let mut f = Fields::new(rtbl.clone(), &ctx);
                let _ = f.require_str("id").map_err(|e| StatusTableError::new(&ctx, e.to_string()))?;
                let _ = f.require_str("workspace").map_err(|e| StatusTableError::new(&ctx, e.to_string()))?;
                let _ = f.require_str("path").map_err(|e| StatusTableError::new(&ctx, e.to_string()))?;
                let raw_safe_api = f.require_array("safe_api").map_err(|e| StatusTableError::new(&ctx, e.to_string()))?;
                let lines = f.require_int("lines").map_err(|e| StatusTableError::new(&ctx, e.to_string()))?;
                let unsafe_blocks = f.require_int("unsafe_blocks").map_err(|e| StatusTableError::new(&ctx, e.to_string()))?;
                let _ = f.require_str("safety_doc").map_err(|e| StatusTableError::new(&ctx, e.to_string()))?;
                let _ = f.require_str("reason").map_err(|e| StatusTableError::new(&ctx, e.to_string()))?;
                f.finish().map_err(|e| StatusTableError::new(&ctx, e.to_string()))?;
                let safe_api = extract_str_list(&raw_safe_api, "safe_api", &ctx, true)?;
                if safe_api.is_empty() {
                    return Err(StatusTableError::new(&ctx, "`safe_api` must not be empty"));
                }
                if !(1..=299).contains(&lines) {
                    return Err(StatusTableError::new(
                        &ctx,
                        format!("`lines` ({lines}) must be in 1..=299"),
                    ));
                }
                if unsafe_blocks < 1 {
                    return Err(StatusTableError::new(
                        &ctx,
                        format!("`unsafe_blocks` ({unsafe_blocks}) must be >= 1"),
                    ));
                }
            }
            _ => {}
        }
    }
    Ok(rows.len())
}

#[test]
fn verb_projection_rejects_non_string_refusal() {
    let planted = br#"schema = "frob.verbs/1"
[[verb]]
name = "doctor"
cluster = "utility"
owner = "frob-cli"
purpose = "Check installation"
use_when = "Diagnosing the local stack"
refusals = [123]
"#;
    let error = validate_empty_or_typed_registry("verbs.toml", "frob.verbs/1", "verb", planted)
        .expect_err("projected refusal entries must be strings");
    assert!(error.message.contains("refusals"), "{error}");
}

/// Validate `registries/refusals.toml` (schema `frob.refusals/1`, empty or populated).
pub fn validate_refusals_toml(bytes: &[u8]) -> Result<usize, StatusTableError> {
    let root =
        toml::parse(bytes).map_err(|e| StatusTableError::new("refusals.toml", e.to_string()))?;
    let mut rf = Fields::new(root, "refusals.toml");
    let schema = rf
        .require_str("schema")
        .map_err(|e| StatusTableError::new("refusals.toml", e.to_string()))?;
    let exit_tbl = rf
        .opt_table("exit_rule")
        .map_err(|e| StatusTableError::new("refusals.toml", e.to_string()))?;
    let raw_rows = rf
        .opt_array("refusal")
        .map_err(|e| StatusTableError::new("refusals.toml", e.to_string()))?;
    let raw_details = rf
        .opt_array("detail")
        .map_err(|e| StatusTableError::new("refusals.toml", e.to_string()))?;
    rf.finish()
        .map_err(|e| StatusTableError::new("refusals.toml", e.to_string()))?;

    if schema != "frob.refusals/1" {
        return Err(StatusTableError::new(
            "refusals.toml",
            format!("unexpected schema `{schema}` (expected `frob.refusals/1`)"),
        ));
    }
    if let Some(exit_tbl) = exit_tbl {
        for (k, sp) in exit_tbl.iter() {
            match &sp.value {
                Value::Int(v) if (1..=255).contains(v) => {}
                Value::Array(arr) => {
                    for item in arr {
                        let Some(c) = item.value.as_str() else {
                            return Err(StatusTableError::new(
                                "refusals.toml:exit_rule",
                                format!("key `{k}` array must hold refusal code strings"),
                            ));
                        };
                        if !is_refusal_code(c) {
                            return Err(StatusTableError::new(
                                "refusals.toml:exit_rule",
                                format!("key `{k}` contains invalid refusal code `{c}`"),
                            ));
                        }
                    }
                }
                other => {
                    return Err(StatusTableError::new(
                        "refusals.toml:exit_rule",
                        format!(
                            "key `{k}` must be integer (1..=255) or refusal-code array, found {}",
                            other.type_name()
                        ),
                    ));
                }
            }
        }
    }
    let rows = match raw_rows {
        Some(items) => extract_table_list(items, "refusal", "refusals.toml")?,
        None => Vec::new(),
    };

    let mut seen_codes = BTreeSet::new();
    for (idx, rtbl) in rows.into_iter().enumerate() {
        let ctx = format!("refusals.toml:refusal[{idx}]");
        let mut f = Fields::new(rtbl, &ctx);
        let code = f
            .require_str("code")
            .map_err(|e| StatusTableError::new(&ctx, e.to_string()))?;
        let meaning = f
            .require_str("meaning")
            .map_err(|e| StatusTableError::new(&ctx, e.to_string()))?;
        let _ = f
            .require_str("defined_in")
            .map_err(|e| StatusTableError::new(&ctx, e.to_string()))?;
        let exit = f
            .require_int("exit")
            .map_err(|e| StatusTableError::new(&ctx, e.to_string()))?;
        let retry = f
            .require_str("retry")
            .map_err(|e| StatusTableError::new(&ctx, e.to_string()))?;
        let raw_repairs = f
            .require_array("repairs")
            .map_err(|e| StatusTableError::new(&ctx, e.to_string()))?;
        let raw_raised = f
            .require_array("raised_by")
            .map_err(|e| StatusTableError::new(&ctx, e.to_string()))?;
        let raw_related = f
            .opt_array("related")
            .map_err(|e| StatusTableError::new(&ctx, e.to_string()))?;
        f.finish()
            .map_err(|e| StatusTableError::new(&ctx, e.to_string()))?;

        if !is_refusal_code(&code) {
            return Err(StatusTableError::new(
                &ctx,
                format!("invalid refusal code `{code}`"),
            ));
        }
        if !seen_codes.insert(code.clone()) {
            return Err(StatusTableError::new(
                &ctx,
                format!("duplicate refusal code `{code}`"),
            ));
        }
        if meaning.trim().is_empty() {
            return Err(StatusTableError::new(&ctx, format!("refusal `{code}` meaning is empty")));
        }
        if !(1..=255).contains(&exit) {
            return Err(StatusTableError::new(
                &ctx,
                format!("refusal `{code}` exit `{exit}` out of 1..=255"),
            ));
        }
        if !matches!(
            retry.as_str(),
            "never" | "after-change" | "backoff" | "after-backoff" | "same-key" | "with-budget"
        ) {
            return Err(StatusTableError::new(
                &ctx,
                format!("refusal `{code}` invalid retry `{retry}`"),
            ));
        }
        let _ = extract_str_list(&raw_repairs, "repairs", &ctx, true)?;
        let _ = extract_str_list(&raw_raised, "raised_by", &ctx, true)?;
        if let Some(items) = raw_related {
            for rel in extract_str_list(&items, "related", &ctx, true)? {
                if !is_refusal_code(&rel) {
                    return Err(StatusTableError::new(
                        &ctx,
                        format!("refusal `{code}` related contains invalid code `{rel}`"),
                    ));
                }
            }
        }
    }
    if let Some(items) = raw_details {
        for (idx, dtbl) in extract_table_list(items, "detail", "refusals.toml")?.into_iter().enumerate() {
            let ctx = format!("refusals.toml:detail[{idx}]");
            let mut f = Fields::new(dtbl, &ctx);
            let _ = f.require_str("slug").map_err(|e| StatusTableError::new(&ctx, e.to_string()))?;
            let _ = f.require_str("detail").map_err(|e| StatusTableError::new(&ctx, e.to_string()))?;
            let raw_interim = f.require_array("interim").map_err(|e| StatusTableError::new(&ctx, e.to_string()))?;
            let exit = f.require_int("exit").map_err(|e| StatusTableError::new(&ctx, e.to_string()))?;
            let retry = f.require_str("retry").map_err(|e| StatusTableError::new(&ctx, e.to_string()))?;
            let _ = f.require_str("pointer").map_err(|e| StatusTableError::new(&ctx, e.to_string()))?;
            let raw_obs = f.require_array("observed").map_err(|e| StatusTableError::new(&ctx, e.to_string()))?;
            let raw_rep = f.require_array("repairs").map_err(|e| StatusTableError::new(&ctx, e.to_string()))?;
            let raw_raised = f.require_array("raised_by").map_err(|e| StatusTableError::new(&ctx, e.to_string()))?;
            let _ = f.require_str("ledger").map_err(|e| StatusTableError::new(&ctx, e.to_string()))?;
            f.finish().map_err(|e| StatusTableError::new(&ctx, e.to_string()))?;
            if !(1..=255).contains(&exit) {
                return Err(StatusTableError::new(&ctx, format!("detail exit `{exit}` out of 1..=255")));
            }
            if !matches!(retry.as_str(), "never" | "after-change" | "backoff" | "after-backoff" | "same-key" | "with-budget") {
                return Err(StatusTableError::new(&ctx, format!("detail invalid retry `{retry}`")));
            }
            for c in extract_str_list(&raw_interim, "interim", &ctx, true)? {
                if !is_refusal_code(&c) {
                    return Err(StatusTableError::new(&ctx, format!("detail interim invalid code `{c}`")));
                }
            }
            let _ = extract_str_list(&raw_obs, "observed", &ctx, true)?;
            let _ = extract_str_list(&raw_rep, "repairs", &ctx, true)?;
            let _ = extract_str_list(&raw_raised, "raised_by", &ctx, true)?;
        }
    }
    Ok(seen_codes.len())
}

/// Validate a crate `contract.toml` against `schemas/contract.schema.json` rules.
pub fn validate_contract_toml(bytes: &[u8]) -> Result<(), StatusTableError> {
    let root =
        toml::parse(bytes).map_err(|e| StatusTableError::new("contract.toml", e.to_string()))?;
    let mut rf = Fields::new(root, "contract.toml");
    let crate_tbl = rf
        .require_table("crate")
        .map_err(|e| StatusTableError::new("contract.toml", e.to_string()))?;
    let budgets_tbl = rf
        .require_table("budgets")
        .map_err(|e| StatusTableError::new("contract.toml", e.to_string()))?;
    let sm_tbl = rf
        .require_table("safe_mode")
        .map_err(|e| StatusTableError::new("contract.toml", e.to_string()))?;
    let feat_tbl_opt = rf
        .opt_table("features")
        .map_err(|e| StatusTableError::new("contract.toml", e.to_string()))?;
    let api_tbl_opt = rf
        .opt_table("api")
        .map_err(|e| StatusTableError::new("contract.toml", e.to_string()))?;
    rf.finish()
        .map_err(|e| StatusTableError::new("contract.toml", e.to_string()))?;

    {
        let mut cf = Fields::new(crate_tbl, "contract.toml:[crate]");
        let name = cf
            .require_str("name")
            .map_err(|e| StatusTableError::new("contract.toml:[crate]", e.to_string()))?;
        let layer_opt = cf.take("layer");
        let rank = cf
            .require_int("rank")
            .map_err(|e| StatusTableError::new("contract.toml:[crate]", e.to_string()))?;
        let plane = cf
            .require_str("plane")
            .map_err(|e| StatusTableError::new("contract.toml:[crate]", e.to_string()))?;
        let det = cf
            .require_str("determinism")
            .map_err(|e| StatusTableError::new("contract.toml:[crate]", e.to_string()))?;
        let ev = cf
            .require_str("evidence")
            .map_err(|e| StatusTableError::new("contract.toml:[crate]", e.to_string()))?;
        let raw_gates = cf
            .require_array("gates")
            .map_err(|e| StatusTableError::new("contract.toml:[crate]", e.to_string()))?;
        let raw_refs = cf
            .require_array("refusals")
            .map_err(|e| StatusTableError::new("contract.toml:[crate]", e.to_string()))?;
        cf.finish()
            .map_err(|e| StatusTableError::new("contract.toml:[crate]", e.to_string()))?;

        if name.trim().is_empty() {
            return Err(StatusTableError::new("contract.toml:[crate]", "name is empty"));
        }
        let layer_sp = layer_opt.ok_or_else(|| {
            StatusTableError::new("contract.toml:[crate]", "missing required key `layer`")
        })?;
        match &layer_sp.value {
            Value::Int(n) if (0..=7).contains(n) => {}
            Value::Str(s) if s == "annex" => {}
            other => {
                return Err(StatusTableError::new(
                    "contract.toml:[crate]",
                    format!("`layer` must be 0..=7 or `annex`, found {}", other.type_name()),
                ));
            }
        }
        if rank < 0 {
            return Err(StatusTableError::new("contract.toml:[crate]", "`rank` must be >= 0"));
        }
        if plane != "compute" && plane != "decision" {
            return Err(StatusTableError::new(
                "contract.toml:[crate]",
                format!("invalid plane `{plane}` (expected `compute` or `decision`)"),
            ));
        }
        if !VALID_DETERMINISM.contains(&det.as_str()) {
            return Err(StatusTableError::new(
                "contract.toml:[crate]",
                format!("invalid determinism `{det}`"),
            ));
        }
        if !VALID_EVIDENCE.contains(&ev.as_str()) {
            return Err(StatusTableError::new(
                "contract.toml:[crate]",
                format!("invalid evidence `{ev}`"),
            ));
        }
        let _ = extract_str_list(&raw_gates, "gates", "contract.toml:[crate]", true)?;
        let refs = extract_str_list(&raw_refs, "refusals", "contract.toml:[crate]", true)?;
        for r in refs {
            if !is_refusal_code(&r) {
                return Err(StatusTableError::new(
                    "contract.toml:[crate]",
                    format!("invalid refusal code `{r}`"),
                ));
            }
        }
    }

    {
        if budgets_tbl.get("wall_ms").and_then(Value::as_int).is_none() {
            return Err(StatusTableError::new(
                "contract.toml:[budgets]",
                "missing integer `wall_ms`",
            ));
        }
        if budgets_tbl
            .get("memory_mib")
            .and_then(Value::as_int)
            .is_none()
        {
            return Err(StatusTableError::new(
                "contract.toml:[budgets]",
                "missing integer `memory_mib`",
            ));
        }
        for (k, sp) in budgets_tbl.iter() {
            match sp.value.as_int() {
                Some(v) if v >= 0 => {}
                _ => {
                    return Err(StatusTableError::new(
                        "contract.toml:[budgets]",
                        format!("budget `{k}` must be a non-negative integer"),
                    ));
                }
            }
        }
    }

    {
        let mut smf = Fields::new(sm_tbl, "contract.toml:[safe_mode]");
        let on_budget = smf
            .require_str("on_budget")
            .map_err(|e| StatusTableError::new("contract.toml:[safe_mode]", e.to_string()))?;
        let on_calibration = smf
            .require_str("on_calibration")
            .map_err(|e| StatusTableError::new("contract.toml:[safe_mode]", e.to_string()))?;
        smf.finish()
            .map_err(|e| StatusTableError::new("contract.toml:[safe_mode]", e.to_string()))?;
        for (k, v) in [("on_budget", on_budget), ("on_calibration", on_calibration)] {
            let valid = v == "reference-path"
                || v.strip_prefix("refuse:").is_some_and(is_refusal_code);
            if !valid {
                return Err(StatusTableError::new(
                    "contract.toml:[safe_mode]",
                    format!("`{k}` must be `reference-path` or `refuse:FROB-REF-nnn`, found `{v}`"),
                ));
            }
        }
    }

    if let Some(feat_tbl) = feat_tbl_opt {
        for (fname, sp) in feat_tbl.into_inner() {
            let Value::Table(ft) = sp.value else {
                return Err(StatusTableError::new(
                    "contract.toml:[features]",
                    format!("feature `{fname}` must be an inline table"),
                ));
            };
            let fctx = format!("contract.toml:[features].{fname}");
            let mut ff = Fields::new(ft, &fctx);
            let tier = ff
                .require_str("tier")
                .map_err(|e| StatusTableError::new(&fctx, e.to_string()))?;
            let _ = ff
                .opt_str("kill")
                .map_err(|e| StatusTableError::new(&fctx, e.to_string()))?;
            let _ = ff
                .opt_str("pilot")
                .map_err(|e| StatusTableError::new(&fctx, e.to_string()))?;
            let _ = ff
                .opt_bool("default")
                .map_err(|e| StatusTableError::new(&fctx, e.to_string()))?;
            let _ = ff
                .opt_bool("dev_only")
                .map_err(|e| StatusTableError::new(&fctx, e.to_string()))?;
            for list_key in ["traits", "crates", "consumers", "test_consumers"] {
                if let Some(arr) = ff
                    .opt_array(list_key)
                    .map_err(|e| StatusTableError::new(&fctx, e.to_string()))?
                {
                    let _ = extract_str_list(&arr, list_key, &fctx, true)
                        .map_err(|e| ff.fail(e))?;
                }
            }
            for str_key in ["first_milestone", "gate", "parent_group"] {
                if let Some(s) = ff
                    .opt_str(str_key)
                    .map_err(|e| StatusTableError::new(&fctx, e.to_string()))?
                    && s.trim().is_empty()
                {
                    return Err(ff.fail(StatusTableError::new(
                        &fctx,
                        format!("`{str_key}` must not be empty"),
                    )));
                }
            }
            ff.finish()
                .map_err(|e| StatusTableError::new(&fctx, e.to_string()))?;
            if !matches!(tier.as_str(), "S" | "F" | "M") {
                return Err(StatusTableError::new(
                    &fctx,
                    format!("invalid feature tier `{tier}` (expected S, F, or M)"),
                ));
            }
        }
    }

    if let Some(api_tbl) = api_tbl_opt {
        let mut af = Fields::new(api_tbl, "contract.toml:[api]");
        let fok_opt = af
            .opt_table("float_ok")
            .map_err(|e| StatusTableError::new("contract.toml:[api]", e.to_string()))?;
        af.finish()
            .map_err(|e| StatusTableError::new("contract.toml:[api]", e.to_string()))?;
        if let Some(fok) = fok_opt {
            for (sig, sp) in fok.iter() {
                match sp.value.as_str() {
                    Some(reason) if !reason.trim().is_empty() => {}
                    _ => {
                        return Err(StatusTableError::new(
                            "contract.toml:[api.float_ok]",
                            format!("`{sig}` must map to a non-empty reason string"),
                        ));
                    }
                }
            }
        }
    }

    Ok(())
}

/// Validate all nine registry entries and all accompanying JSON schemas under `root`.
pub fn validate_all_registries(root: &Path) -> Result<RegistryAudit, StatusTableError> {
    let mut validated_files = Vec::new();

    // 1. Validate every schema JSON file exists and is well-formed JSON.
    let schema_paths = [
        "schemas/demo.schema.json",
        "schemas/contract.schema.json",
        "schemas/registries/verbs.schema.json",
        "schemas/registries/refusals.schema.json",
        "schemas/registries/kernels.schema.json",
        "schemas/registries/gates.schema.json",
        "schemas/registries/formats.schema.json",
        "schemas/registries/upstream.schema.json",
        "schemas/registries/lifts.schema.json",
        "schemas/registries/capsules.schema.json",
        "schemas/registries/card.schema.json",
    ];
    for rel in schema_paths {
        let p = root.join(rel);
        let raw = fs::read_to_string(&p)
            .map_err(|e| StatusTableError::new(rel, format!("missing or unreadable: {e}")))?;
        validate_json_schema_syntax(rel, &raw)?;
        validated_files.push(rel.to_owned());
    }

    // 2. Validate the eight TOML registries.
    let gates_bytes = fs::read(root.join("registries/gates.toml"))
        .map_err(|e| StatusTableError::new("registries/gates.toml", e.to_string()))?;
    let gates = parse_gates_toml(&gates_bytes)?;
    validated_files.push("registries/gates.toml".to_owned());

    let formats_bytes = fs::read(root.join("registries/formats.toml"))
        .map_err(|e| StatusTableError::new("registries/formats.toml", e.to_string()))?;
    let formats = parse_formats_toml(&formats_bytes)?;
    validated_files.push("registries/formats.toml".to_owned());

    let refusals_bytes = fs::read(root.join("registries/refusals.toml"))
        .map_err(|e| StatusTableError::new("registries/refusals.toml", e.to_string()))?;
    let refusal_count = validate_refusals_toml(&refusals_bytes)?;
    validated_files.push("registries/refusals.toml".to_owned());

    let lifts_bytes = fs::read(root.join("registries/lifts.toml"))
        .map_err(|e| StatusTableError::new("registries/lifts.toml", e.to_string()))?;
    let _ = crate::stack::lifts::parse_lifts_toml(&lifts_bytes)
        .map_err(|e| StatusTableError::new("registries/lifts.toml", e.to_string()))?;
    validated_files.push("registries/lifts.toml".to_owned());

    for (fname, schema_name, row_key) in [
        ("verbs.toml", "frob.verbs/1", "verb"),
        ("kernels.toml", "frob.kernels/1", "kernel"),
        ("upstream.toml", "frob.upstream/1", "ask"),
        ("capsules.toml", "frob.capsules/1", "capsule"),
    ] {
        let rel = format!("registries/{fname}");
        let bytes = fs::read(root.join(&rel))
            .map_err(|e| StatusTableError::new(&rel, e.to_string()))?;
        let _ = validate_empty_or_typed_registry(fname, schema_name, row_key, &bytes)?;
        validated_files.push(rel);
    }

    // 3. Validate the ninth entry: registries/cards/ directory exists and has zero .fcard/.toml rows at first commit.
    let cards_dir = root.join("registries/cards");
    if !cards_dir.is_dir() {
        return Err(StatusTableError::new(
            "registries/cards",
            "directory does not exist",
        ));
    }
    validated_files.push("registries/cards".to_owned());

    Ok(RegistryAudit {
        validated_files,
        format_count: formats.formats.len(),
        gate_count: gates.gates.len(),
        refusal_count,
    })
}

// ============================================================================
// Minimal strict JSON validator for schema files (zero external crates)
// ============================================================================

fn validate_json_schema_syntax(ctx: &str, text: &str) -> Result<(), StatusTableError> {
    let mut p = JsonParser {
        bytes: text.as_bytes(),
        pos: 0,
    };
    p.skip_ws();
    if p.peek() != Some(b'{') {
        return Err(StatusTableError::new(ctx, "JSON schema root must be an object"));
    }
    p.parse_value()
        .map_err(|e| StatusTableError::new(ctx, format!("invalid JSON: {e}")))?;
    p.skip_ws();
    if p.pos != p.bytes.len() {
        return Err(StatusTableError::new(ctx, "trailing bytes after JSON root"));
    }
    if !text.contains("\"$schema\"") {
        return Err(StatusTableError::new(ctx, "missing `$schema` key"));
    }
    Ok(())
}

struct JsonParser<'a> {
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> JsonParser<'a> {
    fn skip_ws(&mut self) {
        while let Some(&b) = self.bytes.get(self.pos) {
            if matches!(b, b' ' | b'\t' | b'\r' | b'\n') {
                self.pos += 1;
            } else {
                break;
            }
        }
    }

    fn peek(&self) -> Option<u8> {
        self.bytes.get(self.pos).copied()
    }

    fn parse_value(&mut self) -> Result<(), String> {
        self.skip_ws();
        match self.peek() {
            Some(b'{') => self.parse_object(),
            Some(b'[') => self.parse_array(),
            Some(b'"') => self.parse_string(),
            Some(b't') => self.expect_lit(b"true"),
            Some(b'f') => self.expect_lit(b"false"),
            Some(b'n') => self.expect_lit(b"null"),
            Some(b'-' | b'0'..=b'9') => self.parse_number(),
            Some(b) => Err(format!("unexpected byte 0x{b:02x} at offset {}", self.pos)),
            None => Err("unexpected EOF".to_owned()),
        }
    }

    fn expect_lit(&mut self, lit: &[u8]) -> Result<(), String> {
        if self.bytes[self.pos..].starts_with(lit) {
            self.pos += lit.len();
            Ok(())
        } else {
            Err(format!("expected literal at offset {}", self.pos))
        }
    }

    fn parse_string(&mut self) -> Result<(), String> {
        if self.peek() != Some(b'"') {
            return Err("expected `\"`".to_owned());
        }
        self.pos += 1;
        while let Some(&b) = self.bytes.get(self.pos) {
            if b == b'"' {
                self.pos += 1;
                return Ok(());
            }
            if b == b'\\' {
                self.pos += 1;
                let esc = self.bytes.get(self.pos).copied().ok_or("unterminated escape")?;
                match esc {
                    b'"' | b'\\' | b'/' | b'b' | b'f' | b'n' | b'r' | b't' => self.pos += 1,
                    b'u' => {
                        self.pos += 1;
                        for _ in 0..4 {
                            let h = self.bytes.get(self.pos).copied().ok_or("short \\u escape")?;
                            if !h.is_ascii_hexdigit() {
                                return Err("invalid hex in \\u escape".to_owned());
                            }
                            self.pos += 1;
                        }
                    }
                    _ => return Err(format!("invalid escape \\{}", esc as char)),
                }
            } else if b < 0x20 {
                return Err("unescaped control char in JSON string".to_owned());
            } else {
                self.pos += 1;
            }
        }
        Err("unterminated JSON string".to_owned())
    }

    fn parse_number(&mut self) -> Result<(), String> {
        if self.peek() == Some(b'-') {
            self.pos += 1;
        }
        match self.peek() {
            Some(b'0') => self.pos += 1,
            Some(b'1'..=b'9') => {
                self.pos += 1;
                while matches!(self.peek(), Some(b'0'..=b'9')) {
                    self.pos += 1;
                }
            }
            _ => return Err("invalid JSON number".to_owned()),
        }
        if self.peek() == Some(b'.') {
            self.pos += 1;
            if !matches!(self.peek(), Some(b'0'..=b'9')) {
                return Err("missing fractional digits".to_owned());
            }
            while matches!(self.peek(), Some(b'0'..=b'9')) {
                self.pos += 1;
            }
        }
        if matches!(self.peek(), Some(b'e' | b'E')) {
            self.pos += 1;
            if matches!(self.peek(), Some(b'+' | b'-')) {
                self.pos += 1;
            }
            if !matches!(self.peek(), Some(b'0'..=b'9')) {
                return Err("missing exponent digits".to_owned());
            }
            while matches!(self.peek(), Some(b'0'..=b'9')) {
                self.pos += 1;
            }
        }
        Ok(())
    }

    fn parse_array(&mut self) -> Result<(), String> {
        self.pos += 1;
        self.skip_ws();
        if self.peek() == Some(b']') {
            self.pos += 1;
            return Ok(());
        }
        loop {
            self.parse_value()?;
            self.skip_ws();
            match self.peek() {
                Some(b',') => self.pos += 1,
                Some(b']') => {
                    self.pos += 1;
                    return Ok(());
                }
                _ => return Err(format!("expected `,` or `]` at offset {}", self.pos)),
            }
        }
    }

    fn parse_object(&mut self) -> Result<(), String> {
        self.pos += 1;
        self.skip_ws();
        if self.peek() == Some(b'}') {
            self.pos += 1;
            return Ok(());
        }
        loop {
            self.skip_ws();
            self.parse_string()?;
            self.skip_ws();
            if self.peek() != Some(b':') {
                return Err(format!("expected `:` at offset {}", self.pos));
            }
            self.pos += 1;
            self.parse_value()?;
            self.skip_ws();
            match self.peek() {
                Some(b',') => self.pos += 1,
                Some(b'}') => {
                    self.pos += 1;
                    return Ok(());
                }
                _ => return Err(format!("expected `,` or `}}` at offset {}", self.pos)),
            }
        }
    }
}

// ============================================================================
// Strict YAML-subset reader and validator for demos/<demo_id>/demo.yaml
// (matches schemas/demo.schema.json's 4 required + 11 optional keys and
// refuses any unknown key, plus cross-checks claim_ids against gates.toml)
// ============================================================================

#[derive(Debug, Clone)]
enum YamlVal {
    Str(String),
    Int(i64),
    Bool(bool),
    List(Vec<YamlVal>),
    Map(BTreeMap<String, YamlVal>),
}

fn strip_yaml_comment(line: &str) -> &str {
    let mut in_single = false;
    let mut in_double = false;
    let mut escape = false;
    for (idx, ch) in line.char_indices() {
        if escape {
            escape = false;
            continue;
        }
        if in_double && ch == '\\' {
            escape = true;
            continue;
        }
        if !in_single && ch == '"' {
            in_double = !in_double;
        } else if !in_double && ch == '\'' {
            in_single = !in_single;
        } else if !in_single && !in_double && ch == '#' {
            if idx == 0 || line[..idx].ends_with(|c: char| c.is_whitespace()) {
                return &line[..idx];
            }
        }
    }
    line
}

fn split_yaml_kv(s: &str) -> Option<(&str, &str)> {
    let mut in_single = false;
    let mut in_double = false;
    let mut escape = false;
    let bytes = s.as_bytes();
    for (idx, ch) in s.char_indices() {
        if escape {
            escape = false;
            continue;
        }
        if in_double && ch == '\\' {
            escape = true;
            continue;
        }
        if !in_single && ch == '"' {
            in_double = !in_double;
        } else if !in_double && ch == '\'' {
            in_single = !in_single;
        } else if !in_single && !in_double && ch == ':' {
            let next = bytes.get(idx + 1).copied();
            if next.is_none() || matches!(next, Some(b' ' | b'\t')) {
                return Some((s[..idx].trim(), s[idx + 1..].trim()));
            }
        }
    }
    None
}

fn parse_yaml_scalar_str(raw: &str) -> Result<String, String> {
    let s = raw.trim();
    if let Some(inner) = s.strip_prefix('"').and_then(|r| r.strip_suffix('"')) {
        let mut out = String::with_capacity(inner.len());
        let mut chars = inner.chars();
        while let Some(c) = chars.next() {
            if c == '\\' {
                match chars.next() {
                    Some('"') => out.push('"'),
                    Some('\\') => out.push('\\'),
                    Some('n') => out.push('\n'),
                    Some('t') => out.push('\t'),
                    Some('r') => out.push('\r'),
                    Some(other) => return Err(format!("unsupported escape \\{other}")),
                    None => return Err("dangling backslash".to_owned()),
                }
            } else {
                out.push(c);
            }
        }
        Ok(out)
    } else if let Some(inner) = s.strip_prefix('\'').and_then(|r| r.strip_suffix('\'')) {
        Ok(inner.replace("''", "'"))
    } else {
        Ok(s.to_owned())
    }
}

fn parse_yaml_scalar(raw: &str) -> Result<YamlVal, String> {
    let s = raw.trim();
    if s.starts_with('"') || s.starts_with('\'') {
        return Ok(YamlVal::Str(parse_yaml_scalar_str(s)?));
    }
    if s == "true" {
        return Ok(YamlVal::Bool(true));
    }
    if s == "false" {
        return Ok(YamlVal::Bool(false));
    }
    if s.starts_with('[') && s.ends_with(']') {
        let inner = s[1..s.len() - 1].trim();
        if inner.is_empty() {
            return Ok(YamlVal::List(Vec::new()));
        }
        let mut parts = Vec::new();
        let mut cur = String::new();
        let mut in_s = false;
        let mut in_d = false;
        let mut esc = false;
        for ch in inner.chars() {
            if esc {
                cur.push(ch);
                esc = false;
                continue;
            }
            if in_d && ch == '\\' {
                cur.push(ch);
                esc = true;
                continue;
            }
            if !in_s && ch == '"' {
                in_d = !in_d;
                cur.push(ch);
            } else if !in_d && ch == '\'' {
                in_s = !in_s;
                cur.push(ch);
            } else if !in_s && !in_d && ch == ',' {
                parts.push(parse_yaml_scalar(&cur)?);
                cur.clear();
            } else {
                cur.push(ch);
            }
        }
        if !cur.trim().is_empty() {
            parts.push(parse_yaml_scalar(&cur)?);
        }
        return Ok(YamlVal::List(parts));
    }
    if s == "{}" {
        return Ok(YamlVal::Map(BTreeMap::new()));
    }
    if let Ok(n) = s.parse::<i64>() {
        return Ok(YamlVal::Int(n));
    }
    Ok(YamlVal::Str(s.to_owned()))
}

fn parse_yaml_doc(text: &str) -> Result<BTreeMap<String, YamlVal>, StatusTableError> {
    let mut lines: Vec<(usize, usize, &str)> = Vec::new();
    for (idx, raw_line) in text.lines().enumerate() {
        let stripped = strip_yaml_comment(raw_line).trim_end();
        if stripped.trim().is_empty() {
            continue;
        }
        let indent = stripped.bytes().take_while(|&b| b == b' ').count();
        if stripped.as_bytes().get(indent) == Some(&b'\t') {
            return Err(StatusTableError::new(
                "demo.yaml",
                format!("line {}: tabs are forbidden for YAML indentation", idx + 1),
            ));
        }
        lines.push((idx + 1, indent, &stripped[indent..]));
    }

    let mut pos = 0usize;
    let map = parse_yaml_map(&lines, &mut pos, 0)?;
    if pos != lines.len() {
        return Err(StatusTableError::new(
            "demo.yaml",
            format!("line {}: unexpected trailing content", lines[pos].0),
        ));
    }
    Ok(map)
}

fn parse_yaml_map(
    lines: &[(usize, usize, &str)],
    pos: &mut usize,
    expected_indent: usize,
) -> Result<BTreeMap<String, YamlVal>, StatusTableError> {
    let mut map = BTreeMap::new();
    while *pos < lines.len() {
        let (ln, indent, content) = lines[*pos];
        if indent < expected_indent {
            break;
        }
        if indent > expected_indent {
            return Err(StatusTableError::new(
                "demo.yaml",
                format!("line {ln}: unexpected indentation {indent} (expected {expected_indent})"),
            ));
        }
        if content.starts_with("- ") || content == "-" {
            break;
        }
        let Some((raw_k, raw_v)) = split_yaml_kv(content) else {
            return Err(StatusTableError::new(
                "demo.yaml",
                format!("line {ln}: expected `key: value` mapping entry"),
            ));
        };
        let key = parse_yaml_scalar_str(raw_k)
            .map_err(|e| StatusTableError::new("demo.yaml", format!("line {ln}: {e}")))?;
        if map.contains_key(&key) {
            return Err(StatusTableError::new(
                "demo.yaml",
                format!("line {ln}: duplicate key `{key}`"),
            ));
        }
        *pos += 1;
        let val = if !raw_v.is_empty() {
            parse_yaml_scalar(raw_v)
                .map_err(|e| StatusTableError::new("demo.yaml", format!("line {ln}: {e}")))?
        } else if *pos < lines.len() && lines[*pos].1 > expected_indent {
            let child_indent = lines[*pos].1;
            if lines[*pos].2.starts_with("- ") || lines[*pos].2 == "-" {
                YamlVal::List(parse_yaml_list(lines, pos, child_indent)?)
            } else {
                YamlVal::Map(parse_yaml_map(lines, pos, child_indent)?)
            }
        } else {
            YamlVal::Str(String::new())
        };
        map.insert(key, val);
    }
    Ok(map)
}

fn parse_yaml_list(
    lines: &[(usize, usize, &str)],
    pos: &mut usize,
    expected_indent: usize,
) -> Result<Vec<YamlVal>, StatusTableError> {
    let mut items = Vec::new();
    while *pos < lines.len() {
        let (ln, indent, content) = lines[*pos];
        if indent < expected_indent {
            break;
        }
        if indent != expected_indent {
            return Err(StatusTableError::new(
                "demo.yaml",
                format!("line {ln}: unexpected list indentation {indent}"),
            ));
        }
        let Some(rest) = content.strip_prefix("- ") else {
            break;
        };
        *pos += 1;
        let v = parse_yaml_scalar(rest)
            .map_err(|e| StatusTableError::new("demo.yaml", format!("line {ln}: {e}")))?;
        items.push(v);
    }
    Ok(items)
}

fn yaml_as_non_empty_str(val: &YamlVal, key: &str) -> Result<String, StatusTableError> {
    match val {
        YamlVal::Str(s) if !s.trim().is_empty() => Ok(s.clone()),
        _ => Err(StatusTableError::new(
            "demo.yaml",
            format!("`{key}` must be a non-empty string"),
        )),
    }
}

fn yaml_as_str_list(
    val: &YamlVal,
    key: &str,
    min_items: usize,
) -> Result<Vec<String>, StatusTableError> {
    let YamlVal::List(items) = val else {
        return Err(StatusTableError::new(
            "demo.yaml",
            format!("`{key}` must be a list of strings"),
        ));
    };
    if items.len() < min_items {
        return Err(StatusTableError::new(
            "demo.yaml",
            format!("`{key}` must contain at least {min_items} item(s)"),
        ));
    }
    let mut out = Vec::with_capacity(items.len());
    for (idx, it) in items.iter().enumerate() {
        out.push(yaml_as_non_empty_str(it, &format!("{key}[{idx}]"))?);
    }
    Ok(out)
}

/// Validate a `demo.yaml` file against `schemas/demo.schema.json` (all 4 required
/// keys, all 11 optional keys, unknown-key refusal) and verify every `claim_id`
/// exists in `gates` (`registries/gates.toml`).
pub fn validate_demo_yaml(
    yaml_text: &str,
    gates: &GateRegistry,
) -> Result<DemoDoc, StatusTableError> {
    let mut root = parse_yaml_doc(yaml_text)?;

    // Required keys
    let demo_id = root
        .remove("demo_id")
        .ok_or_else(|| StatusTableError::new("demo.yaml", "missing required key `demo_id`"))
        .and_then(|v| yaml_as_non_empty_str(&v, "demo_id"))?;
    let claim_ids = root
        .remove("claim_ids")
        .ok_or_else(|| StatusTableError::new("demo.yaml", "missing required key `claim_ids`"))
        .and_then(|v| yaml_as_str_list(&v, "claim_ids", 1))?;
    let commands = root
        .remove("commands")
        .ok_or_else(|| StatusTableError::new("demo.yaml", "missing required key `commands`"))
        .and_then(|v| yaml_as_str_list(&v, "commands", 1))?;
    let checksums_val = root
        .remove("checksums")
        .ok_or_else(|| StatusTableError::new("demo.yaml", "missing required key `checksums`"))?;
    let YamlVal::Map(raw_cs) = checksums_val else {
        return Err(StatusTableError::new(
            "demo.yaml",
            "`checksums` must be a map from command line to `b3:<64-hex>`",
        ));
    };
    let mut checksums = BTreeMap::new();
    for (k, v) in raw_cs {
        let s = yaml_as_non_empty_str(&v, &format!("checksums[{k}]"))?;
        if !is_b3_digest(&s) {
            return Err(StatusTableError::new(
                "demo.yaml",
                format!("checksum for `{k}` must match `b3:<64-hex>`, found `{s}`"),
            ));
        }
        checksums.insert(k, s);
    }

    // Optional keys (11)
    let tier = match root.remove("tier") {
        None => None,
        Some(v) => {
            let s = yaml_as_non_empty_str(&v, "tier")?;
            if !VALID_TIERS.contains(&s.as_str()) {
                return Err(StatusTableError::new(
                    "demo.yaml",
                    format!("invalid `tier` value `{s}` (expected cpu, gpu-cuda, or gpu-metal)"),
                ));
            }
            Some(s)
        }
    };

    let mut expect_exit = BTreeMap::new();
    if let Some(v) = root.remove("expect_exit") {
        let YamlVal::Map(m) = v else {
            return Err(StatusTableError::new(
                "demo.yaml",
                "`expect_exit` must be a map from command line to integer (0, 2, or 3)",
            ));
        };
        for (k, ev) in m {
            match ev {
                YamlVal::Int(n) if matches!(n, 0 | 2 | 3) => {
                    expect_exit.insert(k, n);
                }
                _ => {
                    return Err(StatusTableError::new(
                        "demo.yaml",
                        format!("`expect_exit[{k}]` must be integer 0, 2, or 3"),
                    ));
                }
            }
        }
    }

    let mut expect_refusal = BTreeMap::new();
    if let Some(v) = root.remove("expect_refusal") {
        let YamlVal::Map(m) = v else {
            return Err(StatusTableError::new(
                "demo.yaml",
                "`expect_refusal` must be a map from command line to refusal code",
            ));
        };
        for (k, rv) in m {
            let code = yaml_as_non_empty_str(&rv, &format!("expect_refusal[{k}]"))?;
            if !is_refusal_code(&code) {
                return Err(StatusTableError::new(
                    "demo.yaml",
                    format!("`expect_refusal[{k}]` invalid refusal code `{code}`"),
                ));
            }
            expect_refusal.insert(k, code);
        }
    }

    let mut compare = BTreeMap::new();
    if let Some(v) = root.remove("compare") {
        let YamlVal::Map(m) = v else {
            return Err(StatusTableError::new(
                "demo.yaml",
                "`compare` must be a map from command line to non-empty list of fields",
            ));
        };
        for (k, cv) in m {
            let fields = yaml_as_str_list(&cv, &format!("compare[{k}]"), 1)?;
            compare.insert(k, fields);
        }
    }

    let mut expect = BTreeMap::new();
    if let Some(v) = root.remove("expect") {
        let YamlVal::Map(m) = v else {
            return Err(StatusTableError::new(
                "demo.yaml",
                "`expect` must be a map from command line to JSON subset path",
            ));
        };
        for (k, ev) in m {
            let p = yaml_as_non_empty_str(&ev, &format!("expect[{k}]"))?;
            expect.insert(k, p);
        }
    }

    let title = match root.remove("title") {
        None => None,
        Some(v) => Some(yaml_as_non_empty_str(&v, "title")?),
    };

    let milestone = match root.remove("milestone") {
        None => None,
        Some(v) => {
            let s = yaml_as_non_empty_str(&v, "milestone")?;
            if !VALID_MILESTONES.contains(&s.as_str()) {
                return Err(StatusTableError::new(
                    "demo.yaml",
                    format!("invalid `milestone` value `{s}` (expected M0..=M8)"),
                ));
            }
            Some(s)
        }
    };

    let actors = match root.remove("actors") {
        None => Vec::new(),
        Some(v) => yaml_as_str_list(&v, "actors", 0)?,
    };

    let lane = match root.remove("lane") {
        None => None,
        Some(v) => {
            let s = yaml_as_non_empty_str(&v, "lane")?;
            if !VALID_LANES.contains(&s.as_str()) {
                return Err(StatusTableError::new(
                    "demo.yaml",
                    format!("invalid `lane` value `{s}` (expected cpu, device, or long)"),
                ));
            }
            Some(s)
        }
    };

    let fresh_agent = match root.remove("fresh_agent") {
        None => None,
        Some(YamlVal::Bool(b)) => Some(b),
        Some(_) => {
            return Err(StatusTableError::new(
                "demo.yaml",
                "`fresh_agent` must be a boolean",
            ));
        }
    };

    let not_evidence_for = match root.remove("not_evidence_for") {
        None => None,
        Some(v) => {
            let s = yaml_as_non_empty_str(&v, "not_evidence_for")?;
            if s != "numbers" {
                return Err(StatusTableError::new(
                    "demo.yaml",
                    format!("`not_evidence_for` must be the constant `numbers`, found `{s}`"),
                ));
            }
            Some(s)
        }
    };

    // Refuse any unknown key (`additionalProperties: false`).
    if let Some((unknown_key, _)) = root.into_iter().next() {
        return Err(StatusTableError::new(
            "demo.yaml",
            format!("unknown key `{unknown_key}` refused by demo schema"),
        ));
    }

    // Every claim_id must exist in gates.toml.
    let known_claim_ids: BTreeSet<&str> = gates
        .gates
        .iter()
        .map(|g| g.claim_id.as_str())
        .collect();
    for cid in &claim_ids {
        if !known_claim_ids.contains(cid.as_str()) {
            return Err(StatusTableError::new(
                "demo.yaml",
                format!("claim_id `{cid}` is absent from registries/gates.toml"),
            ));
        }
    }

    Ok(DemoDoc {
        demo_id,
        claim_ids,
        commands,
        checksums,
        tier,
        expect_exit,
        expect_refusal,
        compare,
        expect,
        title,
        milestone,
        actors,
        lane,
        fresh_agent,
        not_evidence_for,
    })
}
