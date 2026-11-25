//! Provenance-lift registry (`registries/lifts.toml`, schema `frob.lifts/1`)
//! parser and validator for FrankenRobotics (bead `N33a-01`, rule L-4,
//! KILL-FSIM fallback (b)).
//!
//! Exposed as a library function [`validate`] called by `xtask check-stack`
//! (`N33a-05`); adds no `xtask` subcommand. Pure function of the repository
//! tree (determinism class D0), reads TOML strictly through
//! `frob_core::registry::toml`, prints nothing (rule L-7), uses zero `unsafe`
//! code, and writes or deletes nothing.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::fs;
use std::path::Path;

use frob_core::registry::toml::{self, Fields, Table, Value};

/// Schema identifier declared at the top of `registries/lifts.toml`.
pub const LIFTS_SCHEMA_ID: &str = "frob.lifts/1";

/// Required first-line prefix for every lifted source file.
pub const LIFT_HEADER_PREFIX: &str = "// LIFTED from ";

/// Exact set of record keys accepted by [`parse_lifts_toml`] and declared by
/// `schemas/registries/lifts.schema.json`.
pub const LIFT_RECORD_KEYS: &[&str] = &[
    "file",
    "foundation",
    "source_repo",
    "source_path",
    "source_rev",
    "source_hash",
    "lifted_hash",
    "golden_test",
    "reason",
    "date",
];

/// Directories skipped when scanning a workspace tree for lifted files or
/// Cargo manifests.
const SKIP_SCAN_DIRS: &[&str] = &[
    ".git",
    ".beads",
    ".ntm",
    "target",
    "node_modules",
    "planning",
    "docs",
    "receipts",
    "registries",
    "schemas",
    "repro",
];

/// Maximum file size read during workspace lift scanning (16 MiB).
const MAX_SCAN_FILE_BYTES: u64 = 16 * 1024 * 1024;

/// Parse or I/O error raised while loading `registries/lifts.toml`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiftError {
    pub context: String,
    pub message: String,
}

impl LiftError {
    #[must_use]
    pub fn new(context: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            context: context.into(),
            message: message.into(),
        }
    }
}

impl fmt::Display for LiftError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.context, self.message)
    }
}

impl std::error::Error for LiftError {}

/// One `[[lift]]` record in `registries/lifts.toml` (`frob.lifts/1`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiftRecord {
    pub file: String,
    pub foundation: String,
    pub source_repo: String,
    pub source_path: String,
    pub source_rev: String,
    pub source_hash: String,
    pub lifted_hash: String,
    pub golden_test: String,
    pub reason: String,
    pub date: String,
}

impl LiftRecord {
    /// Expected 4-line provenance header checked byte-for-byte against the
    /// lifted file's opening lines.
    #[must_use]
    pub fn expected_header_lines(&self) -> [String; 4] {
        [
            format!(
                "// LIFTED from {}/{} at {}",
                self.source_repo, self.source_path, self.source_rev
            ),
            format!("// source BLAKE3 {}", self.source_hash),
            "// registry: registries/lifts.toml".to_owned(),
            format!(
                "// re-owned here: edits are ours and are tested by {}",
                self.golden_test
            ),
        ]
    }
}

/// Parsed `registries/lifts.toml` document (`frob.lifts/1`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiftRegistry {
    pub schema: String,
    pub lifts: Vec<LiftRecord>,
}

/// A foundation whose posture is `lift`, checked under validator rule 4 to
/// ensure no workspace manifest still names a crate of that sibling.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiftFoundation {
    pub id: String,
    pub name: String,
    pub path: Option<String>,
    pub crate_prefixes: Vec<String>,
}

impl LiftFoundation {
    /// Construct a [`LiftFoundation`] with default known sibling crate prefixes.
    #[must_use]
    pub fn from_sibling(
        id: impl Into<String>,
        name: impl Into<String>,
        path: Option<String>,
    ) -> Self {
        let name_s = name.into();
        let crate_prefixes = default_sibling_crate_prefixes(&name_s);
        Self {
            id: id.into(),
            name: name_s,
            path,
            crate_prefixes,
        }
    }
}

/// Deterministic validation finding (`{rule, file, message}` plus validator
/// rule number `1..=4`).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct LiftFinding {
    pub rule_num: u8,
    pub rule: String,
    pub file: String,
    pub message: String,
}

impl LiftFinding {
    #[must_use]
    pub fn new(
        rule_num: u8,
        rule: impl Into<String>,
        file: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            rule_num,
            rule: rule.into(),
            file: file.into(),
            message: message.into(),
        }
    }
}

impl fmt::Display for LiftFinding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}: {}", self.rule, self.file, self.message)
    }
}

fn is_lower_hex(s: &str, expected_len: usize) -> bool {
    s.len() == expected_len
        && s.bytes()
            .all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f'))
}

fn is_valid_fd_id(s: &str) -> bool {
    let Some(rest) = s.strip_prefix("FD-") else {
        return false;
    };
    !rest.is_empty()
        && rest
            .bytes()
            .all(|b| b.is_ascii_uppercase() || b.is_ascii_digit() || b == b'-')
}

fn is_valid_repo_name(s: &str) -> bool {
    !s.is_empty()
        && s != "."
        && s != ".."
        && s.bytes()
            .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'_' | b'-' | b'.'))
}

fn is_valid_iso_date(s: &str) -> bool {
    let bytes = s.as_bytes();
    if bytes.len() != 10 || bytes[4] != b'-' || bytes[7] != b'-' {
        return false;
    }
    let year_ok = bytes[0..4].iter().all(u8::is_ascii_digit);
    let month_ok = bytes[5..7].iter().all(u8::is_ascii_digit);
    let day_ok = bytes[8..10].iter().all(u8::is_ascii_digit);
    if !(year_ok && month_ok && day_ok) {
        return false;
    }
    let month: u32 = s[5..7].parse().unwrap_or(0);
    let day: u32 = s[8..10].parse().unwrap_or(0);
    (1..=12).contains(&month) && (1..=31).contains(&day)
}

/// Validate that `path` is strictly repository-relative: never absolute (`/`),
/// never home-directory (`~`, `/Users/...`, `/home/...`, `Users/...`, `home/...`),
/// never Windows-style (`\` or `C:`), and containing no `.` or `..` traversal.
pub fn validate_repo_relative_path(ctx: &str, field: &str, path: &str) -> Result<(), LiftError> {
    if path.is_empty() || path.trim() != path {
        return Err(LiftError::new(
            ctx,
            format!("`{field}` must be a non-empty repository-relative path without surrounding whitespace"),
        ));
    }
    if path.starts_with('/') {
        return Err(LiftError::new(
            ctx,
            format!("`{field}` `{path}` must be repository-relative, not an absolute path"),
        ));
    }
    if path.starts_with('~') {
        return Err(LiftError::new(
            ctx,
            format!("`{field}` `{path}` must be repository-relative, not a home-directory path"),
        ));
    }
    if path.contains('\\') {
        return Err(LiftError::new(
            ctx,
            format!("`{field}` `{path}` must use forward slashes only"),
        ));
    }
    let bytes = path.as_bytes();
    if bytes.len() >= 2 && bytes[0].is_ascii_alphabetic() && bytes[1] == b':' {
        return Err(LiftError::new(
            ctx,
            format!("`{field}` `{path}` must not contain a drive-letter prefix"),
        ));
    }
    let segments: Vec<&str> = path.split('/').collect();
    if segments
        .iter()
        .any(|seg| seg.is_empty() || *seg == "." || *seg == "..")
    {
        return Err(LiftError::new(
            ctx,
            format!("`{field}` `{path}` must not contain empty, `.`, or `..` path segments"),
        ));
    }
    if segments.len() >= 2 && matches!(segments[0], "Users" | "home") {
        return Err(LiftError::new(
            ctx,
            format!("`{field}` `{path}` must not contain a machine home-directory prefix"),
        ));
    }
    Ok(())
}

/// Parse and validate `registries/lifts.toml` strictly through
/// `frob_core::registry::toml`, refusing unknown keys, missing `golden_test`,
/// malformed digests/revisions, duplicate files, and non-repo-relative paths.
pub fn parse_lifts_toml(bytes: &[u8]) -> Result<LiftRegistry, LiftError> {
    let root = toml::parse(bytes).map_err(|e| LiftError::new("lifts.toml", e.to_string()))?;
    let mut rf = Fields::new(root, "lifts.toml");
    let schema = rf
        .require_str("schema")
        .map_err(|e| LiftError::new("lifts.toml", e.to_string()))?;
    let raw_lifts = rf
        .opt_array("lift")
        .map_err(|e| LiftError::new("lifts.toml", e.to_string()))?;
    rf.finish()
        .map_err(|e| LiftError::new("lifts.toml", e.to_string()))?;

    if schema != LIFTS_SCHEMA_ID {
        return Err(LiftError::new(
            "lifts.toml",
            format!("unexpected schema `{schema}` (expected `{LIFTS_SCHEMA_ID}`)"),
        ));
    }

    let mut lifts = Vec::new();
    let mut seen_files = BTreeSet::new();
    if let Some(items) = raw_lifts {
        lifts.reserve(items.len());
        for (idx, sp) in items.into_iter().enumerate() {
            let ctx = format!("lifts.toml:lift[{idx}]");
            let tbl = match sp.value {
                Value::Table(t) => t,
                other => {
                    return Err(LiftError::new(
                        &ctx,
                        format!(
                            "at {}:{}: expected table, found {}",
                            sp.line,
                            sp.column,
                            other.type_name()
                        ),
                    ));
                }
            };
            let rec = parse_lift_record(tbl, &ctx)?;
            if !seen_files.insert(rec.file.clone()) {
                return Err(LiftError::new(
                    &ctx,
                    format!("duplicate lift record for file `{}`", rec.file),
                ));
            }
            lifts.push(rec);
        }
    }

    Ok(LiftRegistry { schema, lifts })
}

fn parse_lift_record(tbl: Table, ctx: &str) -> Result<LiftRecord, LiftError> {
    let mut f = Fields::new(tbl, ctx);
    let file = f
        .require_str("file")
        .map_err(|e| LiftError::new(ctx, e.to_string()))?;
    let foundation = f
        .require_str("foundation")
        .map_err(|e| LiftError::new(ctx, e.to_string()))?;
    let source_repo = f
        .require_str("source_repo")
        .map_err(|e| LiftError::new(ctx, e.to_string()))?;
    let source_path = f
        .require_str("source_path")
        .map_err(|e| LiftError::new(ctx, e.to_string()))?;
    let source_rev = f
        .require_str("source_rev")
        .map_err(|e| LiftError::new(ctx, e.to_string()))?;
    let source_hash = f
        .require_str("source_hash")
        .map_err(|e| LiftError::new(ctx, e.to_string()))?;
    let lifted_hash = f
        .require_str("lifted_hash")
        .map_err(|e| LiftError::new(ctx, e.to_string()))?;
    let golden_test = f
        .require_str("golden_test")
        .map_err(|e| LiftError::new(ctx, e.to_string()))?;
    let reason = f
        .require_str("reason")
        .map_err(|e| LiftError::new(ctx, e.to_string()))?;
    let date = f
        .require_str("date")
        .map_err(|e| LiftError::new(ctx, e.to_string()))?;

    // Finish Fields BEFORE domain validation so Fields::drop never fires in debug builds.
    f.finish()
        .map_err(|e| LiftError::new(ctx, e.to_string()))?;

    validate_repo_relative_path(ctx, "file", &file)?;
    if !is_valid_fd_id(&foundation) {
        return Err(LiftError::new(
            ctx,
            format!("invalid foundation id `{foundation}` (expected `FD-...`)"),
        ));
    }
    if !is_valid_repo_name(&source_repo) {
        return Err(LiftError::new(
            ctx,
            format!("invalid `source_repo` `{source_repo}`"),
        ));
    }
    validate_repo_relative_path(ctx, "source_path", &source_path)?;
    if !is_lower_hex(&source_rev, 40) {
        return Err(LiftError::new(
            ctx,
            format!("invalid `source_rev` `{source_rev}` (expected 40 lower-case hex digits)"),
        ));
    }
    if !is_lower_hex(&source_hash, 64) {
        return Err(LiftError::new(
            ctx,
            format!("invalid `source_hash` `{source_hash}` (expected 64 lower-case hex digits)"),
        ));
    }
    if !is_lower_hex(&lifted_hash, 64) {
        return Err(LiftError::new(
            ctx,
            format!("invalid `lifted_hash` `{lifted_hash}` (expected 64 lower-case hex digits)"),
        ));
    }
    validate_repo_relative_path(ctx, "golden_test", &golden_test)?;
    if reason.trim().is_empty() {
        return Err(LiftError::new(ctx, "`reason` must be a non-empty string"));
    }
    if !is_valid_iso_date(&date) {
        return Err(LiftError::new(
            ctx,
            format!("invalid `date` `{date}` (expected ISO YYYY-MM-DD)"),
        ));
    }

    Ok(LiftRecord {
        file,
        foundation,
        source_repo,
        source_path,
        source_rev,
        source_hash,
        lifted_hash,
        golden_test,
        reason,
        date,
    })
}

/// Derive the candidate Rust module path references for a lifted file path so
/// validator step 2 can verify that `golden_test` names the lifted file.
fn lifted_module_references(file_rel: &str) -> Vec<String> {
    let mut refs = Vec::new();
    let trimmed = file_rel.strip_suffix(".rs").unwrap_or(file_rel);
    if let Some((before_src, after_src)) = trimmed.split_once("/src/") {
        let crate_ident = before_src
            .rsplit('/')
            .next()
            .unwrap_or(before_src)
            .replace('-', "_");
        let mod_inner = after_src
            .strip_suffix("/mod")
            .unwrap_or(after_src)
            .replace('/', "::");
        if !mod_inner.is_empty() {
            if !crate_ident.is_empty() {
                refs.push(format!("{crate_ident}::{mod_inner}"));
            }
            refs.push(format!("crate::{mod_inner}"));
            if mod_inner.contains("::") {
                refs.push(mod_inner);
            }
        }
    }
    refs.push(file_rel.to_owned());
    refs
}

fn golden_names_lifted_file(golden_text: &str, file_rel: &str) -> bool {
    lifted_module_references(file_rel)
        .iter()
        .any(|needle| golden_text.contains(needle.as_str()))
}

fn default_sibling_crate_prefixes(sibling_name: &str) -> Vec<String> {
    let mut prefixes = vec![sibling_name.to_owned(), sibling_name.replace('_', "-")];
    let extras: &[&str] = match sibling_name {
        "frankensim" => &["fs-"],
        "frankensqlite" => &["fsqlite"],
        "frankentorch" => &["ft-"],
        "franken_numpy" => &["fnp-"],
        "frankenscipy" => &["fsci-"],
        "frankensympy" => &["fsym-"],
        "fastmcp_rust" => &["fastmcp"],
        "frankentui" => &["ftui"],
        _ => &[],
    };
    for e in extras {
        prefixes.push((*e).to_owned());
    }
    prefixes.sort();
    prefixes.dedup();
    prefixes
}

/// Read `franken-stack.lock` (if present under `repo_root`) through
/// `frob_core::registry::toml` and return every foundation row whose posture
/// is `lift` (either directly or inside `uses`).
fn load_lift_foundations_from_lock(repo_root: &Path) -> Result<Vec<LiftFoundation>, LiftError> {
    let lock_path = repo_root.join("franken-stack.lock");
    if !lock_path.is_file() {
        return Ok(Vec::new());
    }
    let bytes = fs::read(&lock_path)
        .map_err(|e| LiftError::new("franken-stack.lock", e.to_string()))?;
    let root = toml::parse(&bytes)
        .map_err(|e| LiftError::new("franken-stack.lock", e.to_string()))?;

    let mut out = Vec::new();
    let Some(Value::Array(foundations)) = root.get("foundation") else {
        return Ok(out);
    };

    for sp in foundations {
        let Some(tbl) = sp.value.as_table() else {
            continue;
        };
        let has_direct_lift = tbl
            .get("posture")
            .and_then(Value::as_str)
            .is_some_and(|p| p == "lift");
        let has_use_lift = tbl
            .get("uses")
            .and_then(Value::as_array)
            .is_some_and(|arr| {
                arr.iter().any(|u| {
                    u.value
                        .as_table()
                        .and_then(|ut| ut.get("posture"))
                        .and_then(Value::as_str)
                        .is_some_and(|p| p == "lift")
                })
            });
        if !(has_direct_lift || has_use_lift) {
            continue;
        }
        let id = tbl
            .get("id")
            .and_then(Value::as_str)
            .unwrap_or("<unknown>")
            .to_owned();
        let name = tbl
            .get("name")
            .and_then(Value::as_str)
            .unwrap_or("<unknown>")
            .to_owned();
        let path = tbl.get("path").and_then(Value::as_str).map(str::to_owned);
        out.push(LiftFoundation::from_sibling(id, name, path));
    }
    Ok(out)
}

fn walk_sorted_files(root: &Path, dir: &Path, out: &mut Vec<String>) -> Result<(), LiftError> {
    let rd = fs::read_dir(dir).map_err(|e| LiftError::new(dir.display().to_string(), e.to_string()))?;
    let mut entries: Vec<(String, std::path::PathBuf, std::fs::FileType)> = Vec::new();
    for item in rd {
        let entry =
            item.map_err(|e| LiftError::new(dir.display().to_string(), e.to_string()))?;
        let fname = entry.file_name().to_string_lossy().into_owned();
        let ftype = entry
            .file_type()
            .map_err(|e| LiftError::new(entry.path().display().to_string(), e.to_string()))?;
        entries.push((fname, entry.path(), ftype));
    }
    entries.sort_by(|a, b| a.0.as_bytes().cmp(b.0.as_bytes()));

    for (name, path, ftype) in entries {
        if ftype.is_symlink() {
            continue;
        }
        if ftype.is_dir() {
            if SKIP_SCAN_DIRS.contains(&name.as_str()) {
                continue;
            }
            if let Ok(rel) = path.strip_prefix(root) {
                let rel_s = rel_path_to_slash(rel);
                if rel_s == "tests/admission" {
                    continue;
                }
            }
            walk_sorted_files(root, &path, out)?;
        } else if ftype.is_file()
            && let Ok(rel) = path.strip_prefix(root)
        {
            out.push(rel_path_to_slash(rel));
        }
    }
    Ok(())
}

fn rel_path_to_slash(rel: &Path) -> String {
    let mut parts = Vec::new();
    for comp in rel.components() {
        parts.push(comp.as_os_str().to_string_lossy().into_owned());
    }
    parts.join("/")
}

fn is_dependency_section_header(trimmed: &str) -> bool {
    let Some(inner) = trimmed.strip_prefix('[').and_then(|s| s.strip_suffix(']')) else {
        return false;
    };
    if inner.starts_with('[') {
        return false;
    }
    let header = inner.trim();
    matches!(
        header,
        "dependencies" | "dev-dependencies" | "build-dependencies" | "workspace.dependencies"
    ) || header.starts_with("dependencies.")
        || header.starts_with("dev-dependencies.")
        || header.starts_with("build-dependencies.")
        || header.starts_with("workspace.dependencies.")
        || (header.starts_with("target.")
            && (header.ends_with(".dependencies")
                || header.ends_with(".dev-dependencies")
                || header.ends_with(".build-dependencies")
                || header.contains(".dependencies.")
                || header.contains(".dev-dependencies.")
                || header.contains(".build-dependencies.")))
}

fn normalize_manifest_dep_line(line: &str) -> String {
    let trimmed = line.trim();
    // Normalize `dep.workspace = true` (valid Cargo TOML dotted key outside the
    // strict registry subset) into an inline table accepted by `registry::toml`.
    if let Some((lhs, rhs)) = trimmed.split_once('=') {
        let lhs_t = lhs.trim();
        if let Some((dep, sub)) = lhs_t.split_once('.')
            && !dep.contains('"')
            && !dep.contains('\'')
            && !sub.contains('.')
        {
            return format!("{dep} = {{ {sub} = {} }}", rhs.trim());
        }
    }
    line.to_owned()
}

fn normalize_manifest_header(line: &str) -> String {
    // Convert single-quoted target cfg strings `[target.'cfg(...)'.dependencies]`
    // into basic double-quoted strings for `frob_core::registry::toml`.
    if line.contains('\'') {
        line.replace('\'', "\"")
    } else {
        line.to_owned()
    }
}

/// Extract dependency sections of a Cargo manifest and parse them with
/// `frob_core::registry::toml::parse`.
fn extract_manifest_deps(manifest_text: &str) -> Result<Vec<(String, Option<String>, Option<String>)>, LiftError> {
    let mut filtered = String::new();
    let mut in_dep_section = false;
    for raw_line in manifest_text.lines() {
        let trimmed = raw_line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        if trimmed.starts_with('[') {
            in_dep_section = is_dependency_section_header(trimmed);
            if in_dep_section {
                filtered.push_str(&normalize_manifest_header(trimmed));
                filtered.push('\n');
            }
            continue;
        }
        if in_dep_section {
            filtered.push_str(&normalize_manifest_dep_line(raw_line));
            filtered.push('\n');
        }
    }

    if filtered.trim().is_empty() {
        return Ok(Vec::new());
    }

    let root = toml::parse(filtered.as_bytes())
        .map_err(|e| LiftError::new("Cargo.toml", e.to_string()))?;

    let mut deps = Vec::new();
    collect_dep_tables(&root, &mut deps);
    Ok(deps)
}

fn collect_dep_tables(
    root: &Table,
    out: &mut Vec<(String, Option<String>, Option<String>)>,
) {
    for section in ["dependencies", "dev-dependencies", "build-dependencies"] {
        if let Some(tbl) = root.get(section).and_then(Value::as_table) {
            push_deps_from_table(tbl, out);
        }
    }
    if let Some(ws) = root.get("workspace").and_then(Value::as_table)
        && let Some(tbl) = ws.get("dependencies").and_then(Value::as_table)
    {
        push_deps_from_table(tbl, out);
    }
    if let Some(targets) = root.get("target").and_then(Value::as_table) {
        for (_, sp) in targets.iter() {
            if let Some(target_tbl) = sp.value.as_table() {
                for section in ["dependencies", "dev-dependencies", "build-dependencies"] {
                    if let Some(tbl) = target_tbl.get(section).and_then(Value::as_table) {
                        push_deps_from_table(tbl, out);
                    }
                }
            }
        }
    }
}

fn push_deps_from_table(
    tbl: &Table,
    out: &mut Vec<(String, Option<String>, Option<String>)>,
) {
    for (dep_key, sp) in tbl.iter() {
        let (pkg, path) = match &sp.value {
            Value::Table(spec) => (
                spec.get("package").and_then(Value::as_str).map(str::to_owned),
                spec.get("path").and_then(Value::as_str).map(str::to_owned),
            ),
            _ => (None, None),
        };
        out.push((dep_key.clone(), pkg, path));
    }
}

fn dep_matches_lift_foundation(
    dep_key: &str,
    pkg_override: Option<&str>,
    dep_path: Option<&str>,
    lf: &LiftFoundation,
) -> bool {
    let crate_name = pkg_override.unwrap_or(dep_key);
    let name_hit = lf.crate_prefixes.iter().any(|pref| {
        if pref.ends_with('-') {
            crate_name.starts_with(pref.as_str())
        } else {
            crate_name == pref
                || crate_name.starts_with(&format!("{pref}-"))
                || crate_name.starts_with(&format!("{pref}_"))
        }
    });
    if name_hit {
        return true;
    }

    if let Some(p) = dep_path {
        let norm_p = p.trim().trim_end_matches('/');
        if let Some(lf_path) = lf.path.as_deref() {
            let repo_tail = lf_path
                .trim()
                .trim_end_matches('/')
                .trim_start_matches("../");
            if !repo_tail.is_empty()
                && (norm_p.ends_with(&format!("/{repo_tail}"))
                    || norm_p.contains(&format!("/{repo_tail}/")))
            {
                return true;
            }
        }
        if !lf.name.is_empty()
            && (norm_p.ends_with(&format!("/{}", lf.name))
                || norm_p.contains(&format!("/{}/", lf.name)))
        {
            return true;
        }
    }
    false
}

/// Validate `registries/lifts.toml` and workspace provenance invariants on
/// `repo_root` (validator steps 1..=5 of `N33a-01`, called by `check-stack`).
///
/// Pure function of the repository tree (determinism class D0); writes and
/// deletes nothing.
pub fn validate(repo_root: &Path) -> Result<Vec<LiftFinding>, LiftError> {
    validate_with_lift_foundations(repo_root, &[])
}

/// Same as [`validate`], also checking any caller-supplied foundations with
/// posture `lift` in addition to those read from `franken-stack.lock`.
pub fn validate_with_lift_foundations(
    repo_root: &Path,
    extra_lift_foundations: &[LiftFoundation],
) -> Result<Vec<LiftFinding>, LiftError> {
    // Step 1: Parse registries/lifts.toml strictly; refuse unknown fields and
    // malformed records.
    let reg_path = repo_root.join("registries/lifts.toml");
    let reg_bytes =
        fs::read(&reg_path).map_err(|e| LiftError::new("registries/lifts.toml", e.to_string()))?;
    let registry = parse_lifts_toml(&reg_bytes)?;

    let mut findings: Vec<LiftFinding> = Vec::new();
    let mut registered_files: BTreeMap<String, &LiftRecord> = BTreeMap::new();
    for rec in &registry.lifts {
        registered_files.insert(rec.file.clone(), rec);
    }

    // Step 2: For each record (in canonical sorted order by file), verify the
    // file exists, its 4-line header matches byte-for-byte, and golden_test
    // exists and names the lifted file's module path.
    for rec in registered_files.values() {
        let lifted_abs = repo_root.join(&rec.file);
        if !lifted_abs.is_file() {
            findings.push(LiftFinding::new(
                2,
                "lifts.record.missing-file",
                &rec.file,
                format!("lifted file `{}` recorded in registries/lifts.toml does not exist", rec.file),
            ));
        } else {
            let content = fs::read_to_string(&lifted_abs)
                .map_err(|e| LiftError::new(&rec.file, e.to_string()))?;
            let expected = rec.expected_header_lines();
            let actual: Vec<&str> = content.split('\n').take(4).collect();
            for (idx, exp_line) in expected.iter().enumerate() {
                let obs = actual.get(idx).copied().unwrap_or("<EOF>");
                if obs != exp_line.as_str() {
                    findings.push(LiftFinding::new(
                        2,
                        "lifts.record.header-mismatch",
                        &rec.file,
                        format!(
                            "provenance header line {} mismatch: expected `{exp_line}`, found `{obs}`",
                            idx + 1
                        ),
                    ));
                    break;
                }
            }
        }

        let golden_abs = repo_root.join(&rec.golden_test);
        if !golden_abs.is_file() {
            findings.push(LiftFinding::new(
                2,
                "lifts.record.missing-golden-test",
                &rec.file,
                format!("golden_test `{}` does not exist", rec.golden_test),
            ));
        } else {
            let golden_text = fs::read_to_string(&golden_abs)
                .map_err(|e| LiftError::new(&rec.golden_test, e.to_string()))?;
            if !golden_names_lifted_file(&golden_text, &rec.file) {
                let refs = lifted_module_references(&rec.file);
                findings.push(LiftFinding::new(
                    2,
                    "lifts.record.golden-test-missing-ref",
                    &rec.file,
                    format!(
                        "golden_test `{}` does not name the lifted file's module path (expected one of: {})",
                        rec.golden_test,
                        refs.join(", ")
                    ),
                ));
            }
        }
    }

    // Collect lift-posture foundations for Step 4.
    let mut lift_foundations = load_lift_foundations_from_lock(repo_root)?;
    lift_foundations.extend(extra_lift_foundations.iter().cloned());

    // Step 3 & Step 4: Walk the workspace deterministically to check for
    // unregistered `// LIFTED from` files and forbidden Cargo edges to
    // lift-posture foundations.
    let mut rel_files = Vec::new();
    walk_sorted_files(repo_root, repo_root, &mut rel_files)?;

    for rel in rel_files {
        let abs = repo_root.join(&rel);
        let Ok(meta) = fs::metadata(&abs) else {
            continue;
        };
        if meta.len() > MAX_SCAN_FILE_BYTES {
            continue;
        }

        // Step 3: Check whether the file's first line starts with `// LIFTED from`.
        if let Ok(bytes) = fs::read(&abs) {
            let first_line_bytes = bytes.split(|&b| b == b'\n').next().unwrap_or(&[]);
            if first_line_bytes.starts_with(LIFT_HEADER_PREFIX.as_bytes())
                && !registered_files.contains_key(&rel)
            {
                findings.push(LiftFinding::new(
                    3,
                    "lifts.unregistered",
                    &rel,
                    format!(
                        "file `{rel}` begins with `{LIFT_HEADER_PREFIX}` but has no record in registries/lifts.toml"
                    ),
                ));
            }

            // Step 4: If any foundation has posture `lift`, inspect every Cargo.toml.
            if !lift_foundations.is_empty()
                && (rel == "Cargo.toml" || rel.ends_with("/Cargo.toml"))
                && let Ok(text) = std::str::from_utf8(&bytes)
            {
                let deps = extract_manifest_deps(text)
                    .map_err(|e| LiftError::new(&rel, e.message))?;
                for (dep_key, pkg_override, dep_path) in deps {
                    for lf in &lift_foundations {
                        if dep_matches_lift_foundation(
                            &dep_key,
                            pkg_override.as_deref(),
                            dep_path.as_deref(),
                            lf,
                        ) {
                            findings.push(LiftFinding::new(
                                4,
                                "lifts.forbidden-edge",
                                &rel,
                                format!(
                                    "manifest `{rel}` depends on `{dep_key}` from lift-posture foundation `{}` (`{}`)",
                                    lf.id, lf.name
                                ),
                            ));
                        }
                    }
                }
            }
        }
    }

    // Step 5: Deterministic sorted findings (D0, invariant under lifts.toml record order).
    findings.sort();
    findings.dedup();
    Ok(findings)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU64, Ordering};

    static FIXTURE_SEQ: AtomicU64 = AtomicU64::new(0);

    const REV_A: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
    const REV_B: &str = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";
    const HASH_SRC: &str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
    const HASH_LIFT: &str = "fedcba9876543210fedcba9876543210fedcba9876543210fedcba9876543210";

    fn repo_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .expect("crates/xtask must live two levels below repo root")
            .to_path_buf()
    }

    /// Create a fresh scratch tree under `std::env::temp_dir()`. Never deletes
    /// any file or directory (workspace Rule 9b).
    fn fresh_fixture_dir(tag: &str) -> PathBuf {
        let seq = FIXTURE_SEQ.fetch_add(1, Ordering::Relaxed);
        let dir = std::env::temp_dir().join(format!(
            "frob-n33a01-{tag}-{}-{seq}",
            std::process::id()
        ));
        fs::create_dir_all(dir.join("registries")).expect("create scratch registries dir");
        dir
    }

    fn write_fixture_file(root: &Path, rel: &str, contents: &str) {
        let abs = root.join(rel);
        if let Some(parent) = abs.parent() {
            fs::create_dir_all(parent).expect("create parent dir");
        }
        fs::write(&abs, contents).expect("write fixture file");
    }

    fn sample_record_toml(
        file: &str,
        source_path: &str,
        source_rev: &str,
        golden_test: &str,
    ) -> String {
        format!(
            r#"[[lift]]
file = "{file}"
foundation = "FD-FSIM"
source_repo = "frankensim"
source_path = "{source_path}"
source_rev = "{source_rev}"
source_hash = "{HASH_SRC}"
lifted_hash = "{HASH_LIFT}"
golden_test = "{golden_test}"
reason = "KILL-FSIM fallback (b)"
date = "2026-11-24"
"#
        )
    }

    fn sample_lifted_source(source_path: &str, source_rev: &str, golden_test: &str) -> String {
        format!(
            "// LIFTED from frankensim/{source_path} at {source_rev}\n\
// source BLAKE3 {HASH_SRC}\n\
// registry: registries/lifts.toml\n\
// re-owned here: edits are ours and are tested by {golden_test}\n\
\n\
pub fn motor_identity() -> f64 {{ 1.0 }}\n"
        )
    }

    // Minimal std-only JSON value parser for verifying schemas/registries/lifts.schema.json.
    #[derive(Debug)]
    enum JsonVal {
        Obj(BTreeMap<String, JsonVal>),
        Arr(Vec<JsonVal>),
        Str(String),
        Bool(bool),
        Other,
    }

    fn parse_json(text: &str) -> JsonVal {
        struct P<'a> {
            b: &'a [u8],
            i: usize,
        }
        impl<'a> P<'a> {
            fn ws(&mut self) {
                while matches!(self.b.get(self.i), Some(b' ' | b'\t' | b'\r' | b'\n')) {
                    self.i += 1;
                }
            }
            fn parse_str(&mut self) -> String {
                assert_eq!(self.b[self.i], b'"');
                self.i += 1;
                let mut out = String::new();
                while self.i < self.b.len() {
                    let c = self.b[self.i];
                    self.i += 1;
                    if c == b'"' {
                        return out;
                    }
                    if c == b'\\' {
                        let esc = self.b[self.i];
                        self.i += 1;
                        match esc {
                            b'"' => out.push('"'),
                            b'\\' => out.push('\\'),
                            b'/' => out.push('/'),
                            b'n' => out.push('\n'),
                            b'r' => out.push('\r'),
                            b't' => out.push('\t'),
                            other => out.push(other as char),
                        }
                    } else {
                        out.push(c as char);
                    }
                }
                panic!("unterminated JSON string");
            }
            fn parse_val(&mut self) -> JsonVal {
                self.ws();
                match self.b[self.i] {
                    b'{' => {
                        self.i += 1;
                        let mut m = BTreeMap::new();
                        self.ws();
                        if self.b[self.i] == b'}' {
                            self.i += 1;
                            return JsonVal::Obj(m);
                        }
                        loop {
                            self.ws();
                            let k = self.parse_str();
                            self.ws();
                            assert_eq!(self.b[self.i], b':');
                            self.i += 1;
                            let v = self.parse_val();
                            m.insert(k, v);
                            self.ws();
                            if self.b[self.i] == b',' {
                                self.i += 1;
                            } else {
                                assert_eq!(self.b[self.i], b'}');
                                self.i += 1;
                                break;
                            }
                        }
                        JsonVal::Obj(m)
                    }
                    b'[' => {
                        self.i += 1;
                        let mut a = Vec::new();
                        self.ws();
                        if self.b[self.i] == b']' {
                            self.i += 1;
                            return JsonVal::Arr(a);
                        }
                        loop {
                            a.push(self.parse_val());
                            self.ws();
                            if self.b[self.i] == b',' {
                                self.i += 1;
                            } else {
                                assert_eq!(self.b[self.i], b']');
                                self.i += 1;
                                break;
                            }
                        }
                        JsonVal::Arr(a)
                    }
                    b'"' => JsonVal::Str(self.parse_str()),
                    b't' => {
                        assert!(self.b[self.i..].starts_with(b"true"));
                        self.i += 4;
                        JsonVal::Bool(true)
                    }
                    b'f' => {
                        assert!(self.b[self.i..].starts_with(b"false"));
                        self.i += 5;
                        JsonVal::Bool(false)
                    }
                    _ => {
                        while self.i < self.b.len()
                            && !matches!(
                                self.b[self.i],
                                b',' | b']' | b'}' | b' ' | b'\t' | b'\r' | b'\n'
                            )
                        {
                            self.i += 1;
                        }
                        JsonVal::Other
                    }
                }
            }
        }
        let mut p = P {
            b: text.as_bytes(),
            i: 0,
        };
        p.parse_val()
    }

    #[test]
    fn lifts_empty_registry_valid() {
        let root = repo_root();
        let bytes = fs::read(root.join("registries/lifts.toml"))
            .expect("registries/lifts.toml must exist");
        let reg = parse_lifts_toml(&bytes).expect("shipped registries/lifts.toml must parse");
        assert_eq!(reg.schema, LIFTS_SCHEMA_ID);
        assert!(reg.lifts.is_empty(), "shipped lifts.toml must start empty");

        let findings = validate(&root).expect("validate on clean tree must succeed");
        assert!(
            findings.is_empty(),
            "shipped repo must have zero lift findings, got: {findings:?}"
        );
    }

    #[test]
    fn lifts_schema_matches_loader() {
        let root = repo_root();
        let schema_text = fs::read_to_string(root.join("schemas/registries/lifts.schema.json"))
            .expect("schemas/registries/lifts.schema.json must exist");
        let parsed = parse_json(&schema_text);

        let JsonVal::Obj(top) = parsed else {
            panic!("schema root must be an object");
        };
        assert!(matches!(
            top.get("additionalProperties"),
            Some(JsonVal::Bool(false))
        ));
        let JsonVal::Obj(top_props) = top.get("properties").expect("top properties") else {
            panic!("top properties must be object");
        };
        let JsonVal::Obj(lift_prop) = top_props.get("lift").expect("lift property") else {
            panic!("lift property must be object");
        };
        let JsonVal::Obj(items) = lift_prop.get("items").expect("lift items") else {
            panic!("lift items must be object");
        };
        assert!(
            matches!(items.get("additionalProperties"), Some(JsonVal::Bool(false))),
            "schema lift item must set additionalProperties: false so extra key `notes` is refused"
        );

        let JsonVal::Obj(item_props) = items.get("properties").expect("item properties") else {
            panic!("item properties must be object");
        };
        let schema_keys: BTreeSet<String> = item_props.keys().cloned().collect();
        let loader_keys: BTreeSet<String> =
            LIFT_RECORD_KEYS.iter().map(|s| (*s).to_owned()).collect();
        assert_eq!(
            schema_keys, loader_keys,
            "JSON Schema properties must equal LIFT_RECORD_KEYS accepted by loader"
        );

        let JsonVal::Arr(req_arr) = items.get("required").expect("item required") else {
            panic!("item required must be array");
        };
        let schema_required: BTreeSet<String> = req_arr
            .iter()
            .map(|v| match v {
                JsonVal::Str(s) => s.clone(),
                other => panic!("non-string in required: {other:?}"),
            })
            .collect();
        assert_eq!(schema_required, loader_keys);

        // Extra key `notes` is absent from schema properties (refused by additionalProperties: false)
        // AND refused by the strict loader.
        assert!(!schema_keys.contains("notes"));
        let with_notes = format!(
            "schema = \"frob.lifts/1\"\n\n{}notes = \"unexpected\"\n",
            sample_record_toml(
                "crates/frob-fsim/src/lifted/ga_motor.rs",
                "crates/fs-ga/src/motor.rs",
                REV_A,
                "crates/frob-fsim/tests/lift_ga_motor.rs",
            )
        );
        let err = parse_lifts_toml(with_notes.as_bytes())
            .expect_err("extra key `notes` must be refused by loader");
        assert!(
            err.to_string().contains("notes"),
            "loader error must name unknown key `notes`: {err}"
        );
    }

    #[test]
    fn lifts_unregistered_file_caught() {
        let dir = fresh_fixture_dir("unreg");
        write_fixture_file(&dir, "registries/lifts.toml", "schema = \"frob.lifts/1\"\n");
        let file = "crates/frob-fsim/src/lifted/ga_motor.rs";
        write_fixture_file(
            &dir,
            file,
            &sample_lifted_source(
                "crates/fs-ga/src/motor.rs",
                REV_A,
                "crates/frob-fsim/tests/lift_ga_motor.rs",
            ),
        );

        let findings = validate(&dir).expect("validate must run on synthetic fixture");
        assert_eq!(findings.len(), 1, "expected 1 finding, got: {findings:?}");
        assert_eq!(findings[0].rule_num, 3);
        assert_eq!(findings[0].file, file);
    }

    #[test]
    fn lifts_header_mismatch_caught() {
        let dir = fresh_fixture_dir("hdrmismatch");
        let file = "crates/frob-fsim/src/lifted/ga_motor.rs";
        let golden = "crates/frob-fsim/tests/lift_ga_motor.rs";
        let toml = format!(
            "schema = \"frob.lifts/1\"\n\n{}",
            sample_record_toml(file, "crates/fs-ga/src/motor.rs", REV_A, golden)
        );
        write_fixture_file(&dir, "registries/lifts.toml", &toml);
        write_fixture_file(
            &dir,
            file,
            &sample_lifted_source("crates/fs-ga/src/motor.rs", REV_B, golden),
        );
        write_fixture_file(
            &dir,
            golden,
            "use frob_fsim::lifted::ga_motor::motor_identity;\n#[test] fn golden() { assert_eq!(motor_identity(), 1.0); }\n",
        );

        let findings = validate(&dir).expect("validate must run on mismatch fixture");
        assert_eq!(findings.len(), 1, "expected 1 finding, got: {findings:?}");
        assert_eq!(findings[0].rule_num, 2);
        assert_eq!(findings[0].file, file);
        assert!(
            findings[0].message.contains(REV_A) && findings[0].message.contains(REV_B),
            "finding must show expected and observed revision: {}",
            findings[0].message
        );
    }

    #[test]
    fn lifts_golden_test_required() {
        let missing_golden = format!(
            r#"schema = "frob.lifts/1"

[[lift]]
file = "crates/frob-fsim/src/lifted/ga_motor.rs"
foundation = "FD-FSIM"
source_repo = "frankensim"
source_path = "crates/fs-ga/src/motor.rs"
source_rev = "{REV_A}"
source_hash = "{HASH_SRC}"
lifted_hash = "{HASH_LIFT}"
reason = "KILL-FSIM fallback (b)"
date = "2026-11-24"
"#
        );
        let err = parse_lifts_toml(missing_golden.as_bytes())
            .expect_err("record without golden_test must be a parse error");
        assert!(
            err.to_string().contains("golden_test"),
            "error must name missing key golden_test: {err}"
        );
    }

    #[test]
    fn lifts_paths_are_repo_relative() {
        let bad_paths = [
            "/Users/jemanuel/projects/frankensim/crates/fs-ga/src/motor.rs",
            "/home/ubuntu/frankensim/crates/fs-ga/src/motor.rs",
            "~/projects/frankensim/crates/fs-ga/src/motor.rs",
            "Users/jemanuel/frankensim/crates/fs-ga/src/motor.rs",
            "home/ubuntu/frankensim/crates/fs-ga/src/motor.rs",
            "../frankensim/crates/fs-ga/src/motor.rs",
            "crates\\fs-ga\\src\\motor.rs",
        ];
        for bad in bad_paths {
            let toml = format!(
                "schema = \"frob.lifts/1\"\n\n{}",
                sample_record_toml(
                    "crates/frob-fsim/src/lifted/ga_motor.rs",
                    bad,
                    REV_A,
                    "crates/frob-fsim/tests/lift_ga_motor.rs",
                )
            );
            let err = parse_lifts_toml(toml.as_bytes())
                .expect_err(&format!("non-repo-relative source_path `{bad}` must be refused"));
            assert!(
                err.to_string().contains("source_path"),
                "error for `{bad}` must name `source_path`: {err}"
            );
        }
    }

    #[test]
    fn lifts_missing_file_caught() {
        let dir = fresh_fixture_dir("missingfile");
        let file = "crates/frob-fsim/src/lifted/ga_motor.rs";
        let golden = "crates/frob-fsim/tests/lift_ga_motor.rs";
        let toml = format!(
            "schema = \"frob.lifts/1\"\n\n{}",
            sample_record_toml(file, "crates/fs-ga/src/motor.rs", REV_A, golden)
        );
        write_fixture_file(&dir, "registries/lifts.toml", &toml);
        write_fixture_file(
            &dir,
            golden,
            "use frob_fsim::lifted::ga_motor;\n",
        );

        let findings = validate(&dir).expect("validate must run on missing-file fixture");
        assert_eq!(findings.len(), 1, "expected 1 finding, got: {findings:?}");
        assert_eq!(findings[0].rule_num, 2);
        assert_eq!(findings[0].file, file);
    }

    #[test]
    fn lifts_golden_test_must_exist_and_name_module() {
        let dir = fresh_fixture_dir("goldencheck");
        let file = "crates/frob-fsim/src/lifted/ga_motor.rs";
        let golden = "crates/frob-fsim/tests/lift_ga_motor.rs";
        let toml = format!(
            "schema = \"frob.lifts/1\"\n\n{}",
            sample_record_toml(file, "crates/fs-ga/src/motor.rs", REV_A, golden)
        );
        write_fixture_file(&dir, "registries/lifts.toml", &toml);
        write_fixture_file(
            &dir,
            file,
            &sample_lifted_source("crates/fs-ga/src/motor.rs", REV_A, golden),
        );

        // Missing golden test file on disk -> rule 2 finding.
        let findings1 = validate(&dir).expect("validate must run when golden test missing");
        assert_eq!(findings1.len(), 1, "expected 1 finding, got: {findings1:?}");
        assert_eq!(findings1[0].rule_num, 2);

        // Golden test exists but does not name lifted file's module path -> rule 2 finding.
        write_fixture_file(&dir, golden, "#[test] fn unrelated() { assert!(true); }\n");
        let findings2 = validate(&dir).expect("validate must run when golden test omits module");
        assert_eq!(findings2.len(), 1, "expected 1 finding, got: {findings2:?}");
        assert_eq!(findings2[0].rule_num, 2);
        assert_eq!(findings2[0].rule, "lifts.record.golden-test-missing-ref");
    }

    #[test]
    fn lifts_forbidden_cargo_edge_caught() {
        let dir = fresh_fixture_dir("edge");
        write_fixture_file(&dir, "registries/lifts.toml", "schema = \"frob.lifts/1\"\n");
        write_fixture_file(
            &dir,
            "franken-stack.lock",
            r#"toolchain = "nightly-2026-08-31"

[[foundation]]
id = "FD-FSIM"
name = "frankensim"
posture = "lift"
path = "../frankensim"
"#,
        );
        write_fixture_file(
            &dir,
            "crates/frob-fsim/Cargo.toml",
            r#"[package]
name = "frob-fsim"
version.workspace = true
edition.workspace = true

[dependencies]
fs-ga = { path = "../../frankensim/crates/fs-ga", default-features = false }
"#,
        );

        let findings = validate(&dir).expect("validate must catch lift foundation Cargo edge");
        assert_eq!(findings.len(), 1, "expected 1 finding, got: {findings:?}");
        assert_eq!(findings[0].rule_num, 4);
        assert_eq!(findings[0].file, "crates/frob-fsim/Cargo.toml");
        assert!(findings[0].message.contains("fs-ga"));
        assert!(findings[0].message.contains("FD-FSIM"));
    }

    #[test]
    fn lifts_valid_record_passes_and_reorder_invariant() {
        let dir = fresh_fixture_dir("valid_order");
        let f1 = "crates/frob-fsim/src/lifted/ga_motor.rs";
        let g1 = "crates/frob-fsim/tests/lift_ga_motor.rs";
        let f2 = "crates/frob-fsim/src/lifted/ga_rotor.rs";
        let g2 = "crates/frob-fsim/tests/lift_ga_rotor.rs";

        let r1 = sample_record_toml(f1, "crates/fs-ga/src/motor.rs", REV_A, g1);
        let r2 = sample_record_toml(f2, "crates/fs-ga/src/rotor.rs", REV_A, g2);

        write_fixture_file(
            &dir,
            "registries/lifts.toml",
            &format!("schema = \"frob.lifts/1\"\n\n{r1}\n{r2}"),
        );
        write_fixture_file(&dir, f1, &sample_lifted_source("crates/fs-ga/src/motor.rs", REV_A, g1));
        write_fixture_file(&dir, g1, "use frob_fsim::lifted::ga_motor;\n");
        write_fixture_file(&dir, f2, &sample_lifted_source("crates/fs-ga/src/rotor.rs", REV_A, g2));
        write_fixture_file(&dir, g2, "use frob_fsim::lifted::ga_rotor;\n");

        // Clean valid lifted records -> zero findings.
        let clean = validate(&dir).expect("valid lifts must pass");
        assert!(clean.is_empty(), "expected zero findings, got: {clean:?}");

        // Metamorphic relation: plant mismatches in both files and verify
        // reordering records in registries/lifts.toml yields identical findings.
        let dir_ab = fresh_fixture_dir("reorder_ab");
        let dir_ba = fresh_fixture_dir("reorder_ba");
        for (d, toml_body) in [
            (&dir_ab, format!("schema = \"frob.lifts/1\"\n\n{r1}\n{r2}")),
            (&dir_ba, format!("schema = \"frob.lifts/1\"\n\n{r2}\n{r1}")),
        ] {
            write_fixture_file(d, "registries/lifts.toml", &toml_body);
            write_fixture_file(d, f1, &sample_lifted_source("crates/fs-ga/src/motor.rs", REV_B, g1));
            write_fixture_file(d, g1, "use frob_fsim::lifted::ga_motor;\n");
            // Leave f2 missing while g2 exists.
            write_fixture_file(d, g2, "use frob_fsim::lifted::ga_rotor;\n");
        }
        let findings_ab = validate(&dir_ab).expect("validate ab");
        let findings_ba = validate(&dir_ba).expect("validate ba");
        assert_eq!(findings_ab.len(), 2);
        assert_eq!(findings_ab, findings_ba);
    }
}
