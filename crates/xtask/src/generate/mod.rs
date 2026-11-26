//! Deterministic in-process document, schema, and code generators for `xtask`
//! (`N33a-07`, `N34p4-03`, rule L-8).
//!
//! Each generator row is a pure function from repository input globs to output
//! files (`Regenerate` mode) or manifest digests (`Digest` mode). Uses
//! `frob-core`'s strict TOML and JSON readers, `frob_core::canon` for canonical
//! JSON string escaping, and `fs_blake3` for BLAKE3 digests. Determinism class
//! D0, zero `unsafe`, prints nothing (rule L-7), never deletes files.

pub mod admission_doc;

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

use frob_core::canon::{self, Canonical, Value as CanonValue};
use frob_core::json::{Limits, admit};
use frob_core::registry::refusal_allocations::{
    AllocationLedger, render_refusals_markdown, run_cross_chapter_checks,
};
use frob_core::registry::refusals::{
    CODE_GEN_MARKER, Milestone, RefusalRegistry, render_refusal_code_enum,
};
use frob_core::registry::toml::{self, Value as TomlValue};

use crate::report::FactValue;
use crate::stack::lock;
use crate::status_table::{self, FormatRow};

/// Marker embedded in `$comment` of every generated `schemas/frob.*.json` file.
pub const FORMAT_SCHEMA_MARKER: &str = "@generated from registries/formats.toml, registries/verbs.toml by xtask::generate::format_and_verb_schemas; do not edit by hand.";

/// First-line marker on `crates/frob-core/src/qty_gen.rs` (`N33c-03`).
pub const QTY_GEN_MARKER: &str = "// @generated from registries/qty_aliases.toml, registries/qty_products.toml by xtask::generate::quantity_aliases_and_products (N33c-03); do not edit by hand.";

/// Verification mode for a [`GeneratorRow`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeneratorMode {
    /// Regenerate outputs from inputs and compare byte-for-byte; `--write` rewrites them.
    Regenerate,
    /// Compare BLAKE3 digests of committed files against their manifest; `--write` never rewrites them.
    Digest,
}

impl GeneratorMode {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Regenerate => "regenerate",
            Self::Digest => "digest",
        }
    }
}

/// How a regenerated output maps onto its target file on disk.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputKind {
    /// Entire file at `rel_path` is owned by the generator.
    WholeFile,
    /// Only the block between `<!-- status:begin -->` and `<!-- status:end -->` in `rel_path`.
    ReadmeStatusBlock,
}

/// First-line or structural marker required on every generated output (`gen.marker-missing`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExpectedMarker {
    /// First line must begin with this prefix (e.g. `// @generated`).
    LinePrefix(&'static str),
    /// Valid JSON file whose header carries `"$comment"` with `@generated` or `"description"` with `Generated from `.
    JsonGeneratedComment,
    /// Markdown file carrying `<!-- status:begin -->` and `<!-- status:end -->` markers.
    ReadmeStatusMarkers,
    /// `DEPENDENCY_ADMISSION.md` carrying the BD-002 header and `## Admission Status Table`.
    AdmissionDocHeader,
}

/// One regenerable output emitted by a [`GeneratorFn`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratedFile {
    /// Repository-relative POSIX path (e.g. `schemas/frob.design.1.json`).
    pub rel_path: String,
    pub kind: OutputKind,
    pub marker: ExpectedMarker,
    pub bytes: Vec<u8>,
}

/// One digest expectation emitted by a `Digest`-mode [`GeneratorFn`] (e.g. `registries/kernels.toml`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DigestExpectation {
    /// Repository-relative POSIX path under `kernels/`.
    pub rel_path: String,
    /// Manifest identifier (e.g. `K-cay-step`).
    pub subject: String,
    /// Expected 64-hex lowercase BLAKE3 digest (stripped of `b3:`).
    pub expected_blake3: String,
}

/// Output produced by executing one [`GeneratorRow`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GeneratorOutput {
    Regenerated(Vec<GeneratedFile>),
    RegeneratedWithFacts(Vec<GeneratedFile>, BTreeMap<String, FactValue>),
    Digests(Vec<DigestExpectation>),
}

/// Structured error raised by a generator when an input registry is invalid or missing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratorError {
    /// `gen.*` rule identifier (e.g. `gen.status-invalid`, `gen.missing`).
    pub rule: &'static str,
    pub subject: String,
    pub path: String,
    pub message: String,
    pub repair: String,
}

impl fmt::Display for GeneratorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] {} ({}): {}",
            self.rule, self.subject, self.path, self.message
        )
    }
}

impl std::error::Error for GeneratorError {}

/// Pure generator function signature (`(repo_root, expanded_input_paths) -> Result<GeneratorOutput, GeneratorError>`).
pub type GeneratorFn = fn(&Path, &[PathBuf]) -> Result<GeneratorOutput, GeneratorError>;

/// One declarative generator row in `xtask::generate` (`N34p4-03`).
#[derive(Clone, Copy)]
pub struct GeneratorRow {
    pub name: &'static str,
    pub input_globs: &'static [&'static str],
    pub output_globs: &'static [&'static str],
    pub mode: GeneratorMode,
    pub run: GeneratorFn,
}

/// Rule identifier constants shared between `generate` and `check_generated`.
pub const RULE_DIFFERS: &str = "gen.differs";
pub const RULE_MISSING: &str = "gen.missing";
pub const RULE_ORPHAN: &str = "gen.orphan";
pub const RULE_DIGEST: &str = "gen.digest";
pub const RULE_MARKER_MISSING: &str = "gen.marker-missing";
pub const RULE_STATUS_INVALID: &str = "gen.status-invalid";

/// The six built-in generator rows of `cargo xtask check-generated` (`N34p4-03`).
pub const BUILTIN_GENERATORS: &[GeneratorRow] = &[
    GeneratorRow {
        name: "format-and-verb-schemas",
        input_globs: &[
            "registries/formats.toml",
            "registries/verbs.toml",
            "schemas/verbs/*.json",
        ],
        output_globs: &["schemas/frob.*.json"],
        mode: GeneratorMode::Regenerate,
        run: run_format_and_verb_schemas,
    },
    GeneratorRow {
        name: "stack-lock-schema",
        input_globs: &["crates/xtask/src/stack/lock.rs"],
        output_globs: &["schemas/frob.stack-lock.1.json"],
        mode: GeneratorMode::Regenerate,
        run: run_stack_lock_schema,
    },
    GeneratorRow {
        name: "admission-document",
        input_globs: &["receipts/admissions/**/*.json"],
        output_globs: &["DEPENDENCY_ADMISSION.md"],
        mode: GeneratorMode::Regenerate,
        run: run_admission_document,
    },
    GeneratorRow {
        name: "refusal-codes",
        input_globs: &["registries/refusals.toml"],
        output_globs: &["crates/frob-core/src/refusal/code_gen.rs"],
        mode: GeneratorMode::Regenerate,
        run: run_refusal_codes,
    },
    GeneratorRow {
        name: "honest-status-table",
        input_globs: &["registries/gates.toml"],
        output_globs: &["README.md"],
        mode: GeneratorMode::Regenerate,
        run: run_honest_status_table,
    },
    GeneratorRow {
        name: "kernel-arithmetic",
        input_globs: &["registries/kernels.toml"],
        output_globs: &["kernels/**"],
        mode: GeneratorMode::Digest,
        run: run_kernel_arithmetic,
    },
    GeneratorRow {
        name: "quantity-aliases-and-products",
        input_globs: &[
            "registries/qty_aliases.toml",
            "registries/qty_products.toml",
        ],
        output_globs: &["crates/frob-core/src/qty_gen.rs"],
        mode: GeneratorMode::Regenerate,
        run: run_quantity_aliases_and_products,
    },
    GeneratorRow {
        name: "refusal-printout",
        input_globs: &[
            "registries/refusals.toml",
            "registries/refusal_allocations.toml",
        ],
        output_globs: &["docs/generated/refusals.md"],
        mode: GeneratorMode::Regenerate,
        run: run_refusal_printout,
    },
];

// ============================================================================
// Deterministic std-only glob expansion (`*` within segment, `**` recursive)
// ============================================================================

/// Expand a list of repository-relative globs under `repo_root`, returning
/// deduplicated repository-relative paths in deterministic sorted order (D0).
#[must_use]
pub fn expand_globs(repo_root: &Path, globs: &[&str]) -> Vec<PathBuf> {
    let mut matched: BTreeSet<String> = BTreeSet::new();
    for &pattern in globs {
        let norm = pattern.replace('\\', "/");
        if !norm.contains('*') {
            let p = repo_root.join(&norm);
            if p.is_file() {
                matched.insert(norm);
            }
            continue;
        }
        // Determine fixed directory prefix before the first wildcard segment to avoid scanning the whole repo.
        let pat_segs: Vec<&str> = norm.split('/').filter(|s| !s.is_empty()).collect();
        let mut prefix_segs = Vec::new();
        for &seg in &pat_segs {
            if seg.contains('*') {
                break;
            }
            prefix_segs.push(seg);
        }
        let scan_dir = if prefix_segs.is_empty() {
            repo_root.to_path_buf()
        } else {
            repo_root.join(prefix_segs.join("/"))
        };
        if !scan_dir.is_dir() {
            continue;
        }
        let recursive = pat_segs.contains(&"**") || pat_segs.len() > prefix_segs.len() + 1;
        let mut candidates = Vec::new();
        collect_files(&scan_dir, recursive, &mut candidates);
        for abs in candidates {
            let rel = rel_posix(repo_root, &abs);
            let rel_segs: Vec<&str> = rel.split('/').filter(|s| !s.is_empty()).collect();
            if glob_match_segments(&pat_segs, &rel_segs) {
                matched.insert(rel);
            }
        }
    }
    matched.into_iter().map(PathBuf::from).collect()
}

fn collect_files(dir: &Path, recursive: bool, out: &mut Vec<PathBuf>) {
    let Ok(rd) = fs::read_dir(dir) else { return };
    let mut entries: Vec<PathBuf> = rd.flatten().map(|e| e.path()).collect();
    entries.sort();
    for p in entries {
        let name = p.file_name().and_then(|s| s.to_str()).unwrap_or("");
        if p.is_dir() {
            if recursive && name != ".git" && name != "target" {
                collect_files(&p, true, out);
            }
        } else if p.is_file() {
            out.push(p);
        }
    }
}

fn glob_match_segments(pat: &[&str], path: &[&str]) -> bool {
    if pat.is_empty() {
        return path.is_empty();
    }
    if pat[0] == "**" {
        if glob_match_segments(&pat[1..], path) {
            return true;
        }
        if !path.is_empty() {
            return glob_match_segments(pat, &path[1..]);
        }
        return false;
    }
    if path.is_empty() {
        return false;
    }
    if wildcard_match(pat[0], path[0]) {
        glob_match_segments(&pat[1..], &path[1..])
    } else {
        false
    }
}

fn wildcard_match(pat: &str, text: &str) -> bool {
    let p = pat.as_bytes();
    let t = text.as_bytes();
    let (mut pi, mut ti) = (0usize, 0usize);
    let (mut star_p, mut star_t) = (None, 0usize);
    while ti < t.len() {
        if pi < p.len() && p[pi] == t[ti] {
            pi += 1;
            ti += 1;
        } else if pi < p.len() && p[pi] == b'*' {
            star_p = Some(pi);
            star_t = ti;
            pi += 1;
        } else if let Some(sp) = star_p {
            pi = sp + 1;
            star_t += 1;
            ti = star_t;
        } else {
            return false;
        }
    }
    while pi < p.len() && p[pi] == b'*' {
        pi += 1;
    }
    pi == p.len()
}

fn rel_posix(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

// ============================================================================
// Row 1: Format and verb schemas (`registries/formats.toml`, `registries/verbs.toml`)
// ============================================================================

/// Convert a schema identifier like `frob.design/1` into `schemas/frob.design.1.json`.
#[must_use]
pub fn schema_id_to_rel_path(schema_id: &str) -> String {
    format!("schemas/{}.json", schema_id.replace('/', "."))
}

fn run_format_and_verb_schemas(
    repo_root: &Path,
    _inputs: &[PathBuf],
) -> Result<GeneratorOutput, GeneratorError> {
    let formats_path = repo_root.join("registries/formats.toml");
    let formats_bytes = fs::read(&formats_path).map_err(|e| GeneratorError {
        rule: RULE_MISSING,
        subject: "format-and-verb-schemas".to_owned(),
        path: "registries/formats.toml".to_owned(),
        message: format!("failed to read registries/formats.toml: {e}"),
        repair: "restore registries/formats.toml".to_owned(),
    })?;
    let format_reg =
        status_table::parse_formats_toml(&formats_bytes).map_err(|e| GeneratorError {
            rule: RULE_DIFFERS,
            subject: "format-and-verb-schemas".to_owned(),
            path: "registries/formats.toml".to_owned(),
            message: format!("failed to parse registries/formats.toml: {e}"),
            repair: "fix registries/formats.toml so it conforms to frob.formats/1".to_owned(),
        })?;

    let mut files_by_path: BTreeMap<String, GeneratedFile> = BTreeMap::new();
    let mut format_rows: Vec<&FormatRow> = format_reg.formats.iter().collect();
    format_rows.sort_by(|a, b| a.schema.cmp(&b.schema).then_with(|| a.id.cmp(&b.id)));

    for row in format_rows {
        let rel_path = schema_id_to_rel_path(&row.schema);
        let bytes = render_format_schema(row)?;
        files_by_path.insert(
            rel_path.clone(),
            GeneratedFile {
                rel_path,
                kind: OutputKind::WholeFile,
                marker: ExpectedMarker::JsonGeneratedComment,
                bytes,
            },
        );
    }

    let verbs_path = repo_root.join("registries/verbs.toml");
    if verbs_path.is_file() {
        let verbs_bytes = fs::read(&verbs_path).map_err(|e| GeneratorError {
            rule: RULE_MISSING,
            subject: "format-and-verb-schemas".to_owned(),
            path: "registries/verbs.toml".to_owned(),
            message: format!("failed to read registries/verbs.toml: {e}"),
            repair: "restore registries/verbs.toml".to_owned(),
        })?;
        let verb_schemas = parse_verb_schema_ids(&verbs_bytes)?;
        for (schema_id, verb_names) in verb_schemas {
            let rel_path = schema_id_to_rel_path(&schema_id);
            if files_by_path.contains_key(&rel_path) {
                continue;
            }
            // Authored verb payload schemas under `schemas/verbs/<dot-id>.json`
            // (owned by `N32a-02` / `VERBS-*`) take precedence over stub generation,
            // provided the path resolves inside `repo_root` to a regular file that
            // passes JSON admission and declares the matching `$id`.
            let authored_verb_rel = format!("schemas/verbs/{}.json", schema_id.replace('/', "."));
            let authored_verb_abs = repo_root.join(&authored_verb_rel);
            let authored_present = match fs::symlink_metadata(&authored_verb_abs) {
                Ok(_) => true,
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => false,
                Err(e) => {
                    return Err(GeneratorError {
                        rule: RULE_DIFFERS,
                        subject: schema_id.clone(),
                        path: authored_verb_rel,
                        message: format!("failed to inspect authored verb schema: {e}"),
                        repair:
                            "replace the authored verb schema with a regular in-tree file under schemas/verbs/"
                                .to_owned(),
                    });
                }
            };
            if authored_present {
                let canon_root = fs::canonicalize(repo_root).map_err(|e| GeneratorError {
                    rule: RULE_DIFFERS,
                    subject: schema_id.clone(),
                    path: authored_verb_rel.clone(),
                    message: format!("failed to canonicalize repository root: {e}"),
                    repair: "ensure repository root is accessible".to_owned(),
                })?;
                let canon_authored =
                    fs::canonicalize(&authored_verb_abs).map_err(|e| GeneratorError {
                        rule: RULE_DIFFERS,
                        subject: schema_id.clone(),
                        path: authored_verb_rel.clone(),
                        message: format!(
                            "authored verb schema resolves outside repository or is dangling: {e}"
                        ),
                        repair:
                            "replace the symlink with a regular in-tree file under schemas/verbs/"
                                .to_owned(),
                    })?;
                if !canon_authored.starts_with(&canon_root) {
                    return Err(GeneratorError {
                        rule: RULE_DIFFERS,
                        subject: schema_id.clone(),
                        path: authored_verb_rel,
                        message: format!(
                            "authored verb schema resolves outside repository root (`{}`)",
                            canon_authored.display()
                        ),
                        repair:
                            "replace the out-of-tree symlink with a regular in-tree file under schemas/verbs/"
                                .to_owned(),
                    });
                }
                if !canon_authored.is_file() {
                    return Err(GeneratorError {
                        rule: RULE_DIFFERS,
                        subject: schema_id.clone(),
                        path: authored_verb_rel,
                        message: "authored verb schema is not a regular file".to_owned(),
                        repair: "replace the path with a regular JSON file under schemas/verbs/"
                            .to_owned(),
                    });
                }
                let authored_bytes = fs::read(&canon_authored).map_err(|e| GeneratorError {
                    rule: RULE_DIFFERS,
                    subject: schema_id.clone(),
                    path: authored_verb_rel.clone(),
                    message: format!("failed to read authored verb schema: {e}"),
                    repair: "restore the authored verb schema under schemas/verbs/".to_owned(),
                })?;
                let admitted =
                    admit(&authored_bytes, &Limits::default()).map_err(|e| GeneratorError {
                        rule: RULE_DIFFERS,
                        subject: schema_id.clone(),
                        path: authored_verb_rel.clone(),
                        message: format!("authored verb schema failed JSON admission: {e}"),
                        repair: "fix invalid JSON in authored verb schema under schemas/verbs/"
                            .to_owned(),
                    })?;
                let declared_id = admitted
                    .value()
                    .get("$id")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                if declared_id != schema_id {
                    return Err(GeneratorError {
                        rule: RULE_DIFFERS,
                        subject: schema_id.clone(),
                        path: authored_verb_rel,
                        message: format!(
                            "authored verb schema `$id` `{declared_id}` does not match `{schema_id}`"
                        ),
                        repair:
                            "set `$id` in the authored verb schema to match registries/verbs.toml"
                                .to_owned(),
                    });
                }
                continue;
            }
            let bytes = render_verb_schema(&schema_id, &verb_names)?;
            files_by_path.insert(
                rel_path.clone(),
                GeneratedFile {
                    rel_path,
                    kind: OutputKind::WholeFile,
                    marker: ExpectedMarker::JsonGeneratedComment,
                    bytes,
                },
            );
        }
    }

    Ok(GeneratorOutput::Regenerated(
        files_by_path.into_values().collect(),
    ))
}

fn parse_verb_schema_ids(
    bytes: &[u8],
) -> Result<BTreeMap<String, BTreeSet<String>>, GeneratorError> {
    let tbl = toml::parse(bytes).map_err(|e| GeneratorError {
        rule: RULE_DIFFERS,
        subject: "format-and-verb-schemas".to_owned(),
        path: "registries/verbs.toml".to_owned(),
        message: format!("failed to parse registries/verbs.toml: {e}"),
        repair: "fix registries/verbs.toml so it conforms to frob.verbs/1".to_owned(),
    })?;
    let mut by_schema: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let Some(verbs) = tbl.get("verb").and_then(TomlValue::as_array) else {
        return Ok(by_schema);
    };
    for sp in verbs {
        let Some(v) = sp.value.as_table() else {
            continue;
        };
        let name = v.get("name").and_then(TomlValue::as_str).unwrap_or("");
        for key in ["input_schema", "output_schema"] {
            if let Some(sid) = v.get(key).and_then(TomlValue::as_str)
                && !sid.is_empty()
            {
                by_schema
                    .entry(sid.to_owned())
                    .or_default()
                    .insert(name.to_owned());
            }
        }
    }
    Ok(by_schema)
}

struct JsonStr<'a>(&'a str);

impl Canonical for JsonStr<'_> {
    fn write(&self, w: CanonValue<'_>) -> Result<(), canon::CanonError> {
        w.str(self.0);
        Ok(())
    }
}

fn canon_str(s: &str) -> String {
    canon::to_canonical(&JsonStr(s))
        .map(|cb| cb.as_str().to_owned())
        .unwrap_or_else(|_| "\"\"".to_owned())
}

fn canon_str_list(items: &[String]) -> String {
    let mut sorted: Vec<&str> = items.iter().map(String::as_str).collect();
    sorted.sort_unstable();
    sorted.dedup();
    let rendered: Vec<String> = sorted.into_iter().map(canon_str).collect();
    format!("[{}]", rendered.join(", "))
}

/// Render a deterministic Draft 2020-12 JSON Schema for one `[[format]]` row.
pub fn render_format_schema(row: &FormatRow) -> Result<Vec<u8>, GeneratorError> {
    let desc = format!(
        "FrankenRobotics artifact format {} ({}, owner {}, encoding {}).",
        row.id, row.extension, row.owner, row.encoding
    );
    let title = format!("{} ({})", row.id, row.schema);

    let mut out = String::with_capacity(1024);
    out.push_str("{\n");
    out.push_str(&format!(
        "  \"$comment\": {},\n",
        canon_str(FORMAT_SCHEMA_MARKER)
    ));
    out.push_str(&format!("  \"$id\": {},\n", canon_str(&row.schema)));
    out.push_str("  \"$schema\": \"https://json-schema.org/draft/2020-12/schema\",\n");
    out.push_str("  \"additionalProperties\": true,\n");
    out.push_str(&format!("  \"description\": {},\n", canon_str(&desc)));
    out.push_str("  \"properties\": {\n");
    out.push_str("    \"execution_authorized\": { \"const\": false },\n");
    out.push_str("    \"hardware_safety_signoff\": { \"const\": false },\n");
    if !row.kinds.is_empty() {
        out.push_str(&format!(
            "    \"kind\": {{ \"enum\": {} }},\n",
            canon_str_list(&row.kinds)
        ));
    }
    out.push_str(&format!(
        "    \"schema\": {{ \"const\": {} }},\n",
        canon_str(&row.schema)
    ));
    out.push_str("    \"sim_to_real_certified\": { \"const\": false }\n");
    out.push_str("  },\n");
    out.push_str("  \"required\": [\n");
    out.push_str("    \"execution_authorized\",\n");
    out.push_str("    \"hardware_safety_signoff\",\n");
    if !row.kinds.is_empty() {
        out.push_str("    \"kind\",\n");
    }
    out.push_str("    \"schema\",\n");
    out.push_str("    \"sim_to_real_certified\"\n");
    out.push_str("  ],\n");
    out.push_str(&format!("  \"title\": {},\n", canon_str(&title)));
    out.push_str("  \"type\": \"object\",\n");
    out.push_str("  \"x-frob\": {\n");
    out.push_str(&format!(
        "    \"consumers\": {},\n",
        canon_str_list(&row.consumers)
    ));
    out.push_str(&format!(
        "    \"encoding\": {},\n",
        canon_str(&row.encoding)
    ));
    out.push_str(&format!(
        "    \"extension\": {},\n",
        canon_str(&row.extension)
    ));
    out.push_str(&format!("    \"id\": {},\n", canon_str(&row.id)));
    out.push_str(&format!("    \"owner\": {},\n", canon_str(&row.owner)));
    out.push_str(&format!(
        "    \"producers\": {}\n",
        canon_str_list(&row.producers)
    ));
    out.push_str("  }\n");
    out.push_str("}\n");

    // Verify well-formedness via `frob_core::json::admit` (single JSON reader).
    let bytes = out.into_bytes();
    admit(&bytes, &Limits::default()).map_err(|e| GeneratorError {
        rule: RULE_DIFFERS,
        subject: row.schema.clone(),
        path: schema_id_to_rel_path(&row.schema),
        message: format!("generated schema failed JSON admission: {e}"),
        repair: "fix format schema generator".to_owned(),
    })?;
    Ok(bytes)
}

fn render_verb_schema(
    schema_id: &str,
    verbs: &BTreeSet<String>,
) -> Result<Vec<u8>, GeneratorError> {
    let verb_list: Vec<String> = verbs.iter().cloned().collect();
    let desc = format!(
        "FrankenRobotics verb payload schema {schema_id} (referenced by {}).",
        verb_list.join(", ")
    );
    let mut out = String::with_capacity(512);
    out.push_str("{\n");
    out.push_str(&format!(
        "  \"$comment\": {},\n",
        canon_str(FORMAT_SCHEMA_MARKER)
    ));
    out.push_str(&format!("  \"$id\": {},\n", canon_str(schema_id)));
    out.push_str("  \"$schema\": \"https://json-schema.org/draft/2020-12/schema\",\n");
    out.push_str("  \"additionalProperties\": true,\n");
    out.push_str(&format!("  \"description\": {},\n", canon_str(&desc)));
    out.push_str("  \"properties\": {\n");
    out.push_str(&format!(
        "    \"schema\": {{ \"const\": {} }}\n",
        canon_str(schema_id)
    ));
    out.push_str("  },\n");
    out.push_str("  \"required\": [\"schema\"],\n");
    out.push_str(&format!("  \"title\": {},\n", canon_str(schema_id)));
    out.push_str("  \"type\": \"object\",\n");
    out.push_str("  \"x-frob\": {\n");
    out.push_str(&format!("    \"verbs\": {}\n", canon_str_list(&verb_list)));
    out.push_str("  }\n");
    out.push_str("}\n");

    let bytes = out.into_bytes();
    admit(&bytes, &Limits::default()).map_err(|e| GeneratorError {
        rule: RULE_DIFFERS,
        subject: schema_id.to_owned(),
        path: schema_id_to_rel_path(schema_id),
        message: format!("generated verb schema failed JSON admission: {e}"),
        repair: "fix verb schema generator".to_owned(),
    })?;
    Ok(bytes)
}

// ============================================================================
// Row 2: Stack-lock schema (`stack::lock::json_schema`, N33a-03)
// ============================================================================

fn run_stack_lock_schema(
    _repo_root: &Path,
    _inputs: &[PathBuf],
) -> Result<GeneratorOutput, GeneratorError> {
    let bytes = lock::json_schema();
    Ok(GeneratorOutput::Regenerated(vec![GeneratedFile {
        rel_path: "schemas/frob.stack-lock.1.json".to_owned(),
        kind: OutputKind::WholeFile,
        marker: ExpectedMarker::JsonGeneratedComment,
        bytes,
    }]))
}

// ============================================================================
// Row 3: Admission document (`generate::admission_doc`, N33a-07)
// ============================================================================

fn run_admission_document(
    repo_root: &Path,
    _inputs: &[PathBuf],
) -> Result<GeneratorOutput, GeneratorError> {
    let rendered = admission_doc::render(repo_root).map_err(|msg| GeneratorError {
        rule: RULE_DIFFERS,
        subject: "admission-document".to_owned(),
        path: "DEPENDENCY_ADMISSION.md".to_owned(),
        message: msg,
        repair: "fix malformed admission records under receipts/admissions/ and re-run `--write`"
            .to_owned(),
    })?;
    Ok(GeneratorOutput::Regenerated(vec![GeneratedFile {
        rel_path: "DEPENDENCY_ADMISSION.md".to_owned(),
        kind: OutputKind::WholeFile,
        marker: ExpectedMarker::AdmissionDocHeader,
        bytes: rendered.into_bytes(),
    }]))
}

// ============================================================================
// Row 4: Refusal codes (`registries/refusals.toml` -> `frob-core/src/refusal/code_gen.rs`)
// ============================================================================

fn run_refusal_codes(
    repo_root: &Path,
    _inputs: &[PathBuf],
) -> Result<GeneratorOutput, GeneratorError> {
    let path = repo_root.join("registries/refusals.toml");
    let bytes = fs::read(&path).map_err(|e| GeneratorError {
        rule: RULE_MISSING,
        subject: "refusal-codes".to_owned(),
        path: "registries/refusals.toml".to_owned(),
        message: format!("failed to read registries/refusals.toml: {e}"),
        repair: "restore registries/refusals.toml".to_owned(),
    })?;
    let reg = RefusalRegistry::from_bytes(&bytes).map_err(|e| GeneratorError {
        rule: RULE_DIFFERS,
        subject: "refusal-codes".to_owned(),
        path: "registries/refusals.toml".to_owned(),
        message: format!("invalid registries/refusals.toml: {e}"),
        repair: "fix registries/refusals.toml so it validates against frob.refusals/1".to_owned(),
    })?;
    let rendered = render_refusal_code_enum(&reg);
    let _ = CODE_GEN_MARKER;
    Ok(GeneratorOutput::Regenerated(vec![GeneratedFile {
        rel_path: "crates/frob-core/src/refusal/code_gen.rs".to_owned(),
        kind: OutputKind::WholeFile,
        marker: ExpectedMarker::LinePrefix("// @generated"),
        bytes: rendered.into_bytes(),
    }]))
}

// ============================================================================
// Row 5: Honest-status table (`registries/gates.toml` -> `README.md` markers)
// ============================================================================

fn run_honest_status_table(
    repo_root: &Path,
    _inputs: &[PathBuf],
) -> Result<GeneratorOutput, GeneratorError> {
    let gates_path = repo_root.join("registries/gates.toml");
    let bytes = fs::read(&gates_path).map_err(|e| GeneratorError {
        rule: RULE_MISSING,
        subject: "honest-status-table".to_owned(),
        path: "registries/gates.toml".to_owned(),
        message: format!("failed to read registries/gates.toml: {e}"),
        repair: "restore registries/gates.toml".to_owned(),
    })?;
    let block = status_table::generate_from_toml(&bytes).map_err(|e| GeneratorError {
        rule: RULE_STATUS_INVALID,
        subject: e.context.clone(),
        path: "registries/gates.toml".to_owned(),
        message: e.to_string(),
        repair: "fix the invalid gate row in registries/gates.toml (e.g. provide a measured value for pass)"
            .to_owned(),
    })?;
    Ok(GeneratorOutput::Regenerated(vec![GeneratedFile {
        rel_path: "README.md".to_owned(),
        kind: OutputKind::ReadmeStatusBlock,
        marker: ExpectedMarker::ReadmeStatusMarkers,
        bytes: block.into_bytes(),
    }]))
}

// ============================================================================
// Row 6: Kernel arithmetic digests (`registries/kernels.toml` -> `kernels/**`)
// ============================================================================

fn run_kernel_arithmetic(
    repo_root: &Path,
    _inputs: &[PathBuf],
) -> Result<GeneratorOutput, GeneratorError> {
    let kernels_path = repo_root.join("registries/kernels.toml");
    let bytes = fs::read(&kernels_path).map_err(|e| GeneratorError {
        rule: RULE_MISSING,
        subject: "kernel-arithmetic".to_owned(),
        path: "registries/kernels.toml".to_owned(),
        message: format!("failed to read registries/kernels.toml: {e}"),
        repair: "restore registries/kernels.toml".to_owned(),
    })?;
    let tbl = toml::parse(&bytes).map_err(|e| GeneratorError {
        rule: RULE_DIGEST,
        subject: "kernel-arithmetic".to_owned(),
        path: "registries/kernels.toml".to_owned(),
        message: format!("failed to parse registries/kernels.toml: {e}"),
        repair: "fix registries/kernels.toml so it conforms to frob.kernels/1".to_owned(),
    })?;
    let schema = tbl.get("schema").and_then(TomlValue::as_str).unwrap_or("");
    if schema != "frob.kernels/1" {
        return Err(GeneratorError {
            rule: RULE_DIGEST,
            subject: "kernel-arithmetic".to_owned(),
            path: "registries/kernels.toml".to_owned(),
            message: format!("expected schema `frob.kernels/1`, found `{schema}`"),
            repair: "set `schema = \"frob.kernels/1\"` in registries/kernels.toml".to_owned(),
        });
    }

    let mut expectations = Vec::new();
    let Some(items) = tbl.get("kernel").and_then(TomlValue::as_array) else {
        return Ok(GeneratorOutput::Digests(expectations));
    };

    let existing_kernel_files = expand_globs(repo_root, &["kernels/**"]);

    for (idx, sp) in items.iter().enumerate() {
        let Some(row) = sp.value.as_table() else {
            return Err(GeneratorError {
                rule: RULE_DIGEST,
                subject: format!("kernel[{idx}]"),
                path: "registries/kernels.toml".to_owned(),
                message: "expected table for [[kernel]] entry".to_owned(),
                repair: "fix [[kernel]] entry in registries/kernels.toml".to_owned(),
            });
        };
        let id = row
            .get("id")
            .and_then(TomlValue::as_str)
            .filter(|s| !s.is_empty())
            .ok_or_else(|| GeneratorError {
                rule: RULE_DIGEST,
                subject: format!("kernel[{idx}]"),
                path: "registries/kernels.toml".to_owned(),
                message: "[[kernel]] missing non-empty `id`".to_owned(),
                repair: "set `id = \"K-...\"` on the [[kernel]] row".to_owned(),
            })?;
        let raw_digest = row
            .get("source_digest")
            .and_then(TomlValue::as_str)
            .ok_or_else(|| GeneratorError {
                rule: RULE_DIGEST,
                subject: id.to_owned(),
                path: "registries/kernels.toml".to_owned(),
                message: format!("kernel `{id}` missing `source_digest`"),
                repair: "record `source_digest = \"b3:<64-hex>\"` from the derivation lane"
                    .to_owned(),
            })?;
        let hex = raw_digest
            .strip_prefix("b3:")
            .filter(|h| h.len() == 64 && h.bytes().all(|b| b.is_ascii_digit() || matches!(b, b'a'..=b'f')))
            .ok_or_else(|| GeneratorError {
                rule: RULE_DIGEST,
                subject: id.to_owned(),
                path: "registries/kernels.toml".to_owned(),
                message: format!("kernel `{id}` has invalid source_digest `{raw_digest}` (expected `b3:<64-hex>`)"),
                repair: "record a valid `b3:<64-hex>` BLAKE3 digest in registries/kernels.toml".to_owned(),
            })?;

        let rel_path = if let Some(explicit) = row
            .get("path")
            .or_else(|| row.get("file"))
            .and_then(TomlValue::as_str)
        {
            explicit.to_owned()
        } else if let Some(found) = existing_kernel_files.iter().find(|p| {
            p.file_stem()
                .and_then(|s| s.to_str())
                .is_some_and(|stem| stem == id)
        }) {
            found.to_string_lossy().replace('\\', "/")
        } else {
            format!("kernels/{id}.rs")
        };

        expectations.push(DigestExpectation {
            rel_path,
            subject: id.to_owned(),
            expected_blake3: hex.to_owned(),
        });
    }

    expectations.sort_by(|a, b| {
        a.rel_path
            .cmp(&b.rel_path)
            .then_with(|| a.subject.cmp(&b.subject))
    });
    Ok(GeneratorOutput::Digests(expectations))
}

// ============================================================================
// Row 7: Quantity aliases and products (`registries/qty_*.toml` -> `qty_gen.rs`, N33c-03)
// ============================================================================

fn qty_gen_err(path: &str, msg: impl Into<String>) -> GeneratorError {
    GeneratorError {
        rule: RULE_DIFFERS,
        subject: "quantity-aliases-and-products".to_owned(),
        path: path.to_owned(),
        message: msg.into(),
        repair: "fix registries/qty_aliases.toml and registries/qty_products.toml, then run `cargo xtask check-generated --write`".to_owned(),
    }
}

fn run_quantity_aliases_and_products(
    repo_root: &Path,
    _inputs: &[PathBuf],
) -> Result<GeneratorOutput, GeneratorError> {
    let aliases_rel = "registries/qty_aliases.toml";
    let products_rel = "registries/qty_products.toml";
    let aliases_bytes = fs::read(repo_root.join(aliases_rel)).map_err(|e| GeneratorError {
        rule: RULE_MISSING,
        subject: "quantity-aliases-and-products".to_owned(),
        path: aliases_rel.to_owned(),
        message: format!("failed to read {aliases_rel}: {e}"),
        repair: "restore registries/qty_aliases.toml".to_owned(),
    })?;
    let products_bytes = fs::read(repo_root.join(products_rel)).map_err(|e| GeneratorError {
        rule: RULE_MISSING,
        subject: "quantity-aliases-and-products".to_owned(),
        path: products_rel.to_owned(),
        message: format!("failed to read {products_rel}: {e}"),
        repair: "restore registries/qty_products.toml".to_owned(),
    })?;

    let a_tbl = toml::parse(&aliases_bytes)
        .map_err(|e| qty_gen_err(aliases_rel, format!("parse error: {e}")))?;
    if a_tbl.get("schema").and_then(TomlValue::as_str) != Some("frob.qty_aliases/1") {
        return Err(qty_gen_err(
            aliases_rel,
            "expected schema `frob.qty_aliases/1`",
        ));
    }
    let alias_items = a_tbl
        .get("alias")
        .and_then(TomlValue::as_array)
        .ok_or_else(|| qty_gen_err(aliases_rel, "missing [[alias]] array"))?;

    let mut aliases: Vec<(String, [i8; 6], String, String)> = Vec::with_capacity(alias_items.len());
    let mut alias_dims: BTreeMap<String, [i8; 6]> = BTreeMap::new();
    for (idx, sp) in alias_items.iter().enumerate() {
        let row = sp
            .value
            .as_table()
            .ok_or_else(|| qty_gen_err(aliases_rel, format!("alias[{idx}] must be a table")))?;
        let name = row
            .get("name")
            .and_then(TomlValue::as_str)
            .filter(|s| !s.is_empty())
            .ok_or_else(|| qty_gen_err(aliases_rel, format!("alias[{idx}] missing `name`")))?;
        if alias_dims.contains_key(name) {
            return Err(qty_gen_err(
                aliases_rel,
                format!("duplicate alias `{name}`"),
            ));
        }
        let dims_arr = row
            .get("dimension")
            .and_then(TomlValue::as_array)
            .filter(|a| a.len() == 6)
            .ok_or_else(|| {
                qty_gen_err(
                    aliases_rel,
                    format!("alias `{name}` needs 6-element `dimension`"),
                )
            })?;
        let mut dim = [0i8; 6];
        for (i, d) in dims_arr.iter().enumerate() {
            let v = d
                .value
                .as_int()
                .and_then(|n| i8::try_from(n).ok())
                .ok_or_else(|| {
                    qty_gen_err(aliases_rel, format!("alias `{name}` dim[{i}] not i8"))
                })?;
            dim[i] = v;
        }
        let kind = row
            .get("kind")
            .and_then(TomlValue::as_str)
            .filter(|s| !s.is_empty())
            .ok_or_else(|| qty_gen_err(aliases_rel, format!("alias `{name}` missing `kind`")))?;
        let unit = row
            .get("unit")
            .and_then(TomlValue::as_str)
            .filter(|s| !s.is_empty())
            .ok_or_else(|| qty_gen_err(aliases_rel, format!("alias `{name}` missing `unit`")))?;
        alias_dims.insert(name.to_owned(), dim);
        aliases.push((name.to_owned(), dim, kind.to_owned(), unit.to_owned()));
    }
    if aliases.len() > 64 {
        return Err(qty_gen_err(
            aliases_rel,
            format!(
                "alias count {} exceeds cap 64 (OPEN-SUITE-4)",
                aliases.len()
            ),
        ));
    }

    let p_tbl = toml::parse(&products_bytes)
        .map_err(|e| qty_gen_err(products_rel, format!("parse error: {e}")))?;
    if p_tbl.get("schema").and_then(TomlValue::as_str) != Some("frob.qty_products/1") {
        return Err(qty_gen_err(
            products_rel,
            "expected schema `frob.qty_products/1`",
        ));
    }
    let prod_items = p_tbl
        .get("product")
        .and_then(TomlValue::as_array)
        .ok_or_else(|| qty_gen_err(products_rel, "missing [[product]] array"))?;

    let mut muls: Vec<(String, String, String)> = Vec::new();
    let mut divs: Vec<(String, String, String)> = Vec::new();
    let mut seen_ops: BTreeSet<(String, String, String)> = BTreeSet::new();
    for (idx, sp) in prod_items.iter().enumerate() {
        let row = sp
            .value
            .as_table()
            .ok_or_else(|| qty_gen_err(products_rel, format!("product[{idx}] must be a table")))?;
        let lhs = row
            .get("lhs")
            .and_then(TomlValue::as_str)
            .filter(|s| !s.is_empty())
            .ok_or_else(|| qty_gen_err(products_rel, format!("product[{idx}] missing `lhs`")))?;
        let op = row
            .get("op")
            .and_then(TomlValue::as_str)
            .ok_or_else(|| qty_gen_err(products_rel, format!("product[{idx}] missing `op`")))?;
        let rhs = row
            .get("rhs")
            .and_then(TomlValue::as_str)
            .filter(|s| !s.is_empty())
            .ok_or_else(|| qty_gen_err(products_rel, format!("product[{idx}] missing `rhs`")))?;
        let out = row
            .get("out")
            .and_then(TomlValue::as_str)
            .filter(|s| !s.is_empty())
            .ok_or_else(|| qty_gen_err(products_rel, format!("product[{idx}] missing `out`")))?;
        let dl = *alias_dims.get(lhs).ok_or_else(|| {
            qty_gen_err(products_rel, format!("product[{idx}] unknown lhs `{lhs}`"))
        })?;
        let dr = *alias_dims.get(rhs).ok_or_else(|| {
            qty_gen_err(products_rel, format!("product[{idx}] unknown rhs `{rhs}`"))
        })?;
        let dout = *alias_dims.get(out).ok_or_else(|| {
            qty_gen_err(products_rel, format!("product[{idx}] unknown out `{out}`"))
        })?;
        if !seen_ops.insert((lhs.to_owned(), op.to_owned(), rhs.to_owned())) {
            return Err(qty_gen_err(
                products_rel,
                format!("product[{idx}] duplicate ({lhs}, {op}, {rhs})"),
            ));
        }
        let mut expected = [0i8; 6];
        for i in 0..6 {
            expected[i] = match op {
                "mul" => dl[i].checked_add(dr[i]),
                "div" => dl[i].checked_sub(dr[i]),
                other => {
                    return Err(qty_gen_err(
                        products_rel,
                        format!("product[{idx}] invalid op `{other}` (expected `mul` or `div`)"),
                    ));
                }
            }
            .ok_or_else(|| {
                qty_gen_err(
                    products_rel,
                    format!("product[{idx}] ({lhs} {op} {rhs}) dimension exponent overflow"),
                )
            })?;
        }
        if expected != dout {
            return Err(qty_gen_err(
                products_rel,
                format!(
                    "product[{idx}] `{lhs} {op} {rhs} => {out}` dimension mismatch: expected {expected:?}, got {dout:?}"
                ),
            ));
        }
        match op {
            "mul" => muls.push((lhs.to_owned(), rhs.to_owned(), out.to_owned())),
            "div" => {
                if lhs == rhs {
                    return Err(qty_gen_err(
                        products_rel,
                        format!(
                            "product[{idx}] `{lhs} / {rhs}` duplicates blanket self-division into Ratio"
                        ),
                    ));
                }
                divs.push((lhs.to_owned(), rhs.to_owned(), out.to_owned()));
            }
            _ => unreachable!(),
        }
    }

    let mut out = format!(
        "{QTY_GEN_MARKER}\n\npub const ALIAS_COUNT: usize = {};\n\ndefine_qty_aliases! {{\n",
        aliases.len()
    );
    for (name, d, kind, unit) in &aliases {
        out.push_str(&format!(
            "    {name} = ({}, {}, {}, {}, {}, {}), \"{}\", \"{}\";\n",
            d[0], d[1], d[2], d[3], d[4], d[5], kind, unit
        ));
    }
    out.push_str("}\n\ndefine_qty_ops! {\n");
    for (l, r, o) in &muls {
        out.push_str(&format!("    {l} * {r} => {o};\n"));
    }
    out.push_str("    ;\n");
    for (l, r, o) in &divs {
        out.push_str(&format!("    {l} / {r} => {o};\n"));
    }
    out.push_str("}\n");

    Ok(GeneratorOutput::Regenerated(vec![GeneratedFile {
        rel_path: "crates/frob-core/src/qty_gen.rs".to_owned(),
        kind: OutputKind::WholeFile,
        marker: ExpectedMarker::LinePrefix("// @generated"),
        bytes: out.into_bytes(),
    }]))
}

// ============================================================================
// Row 8: refusal-printout (`N35-33`, `ADR-NUM-refusal-8xx`)
// ============================================================================

fn run_refusal_printout(
    repo_root: &Path,
    _inputs: &[PathBuf],
) -> Result<GeneratorOutput, GeneratorError> {
    let alloc_rel = "registries/refusal_allocations.toml";
    let alloc_path = repo_root.join(alloc_rel);
    if !alloc_path.is_file() && !repo_root.join("docs/generated/refusals.md").exists() {
        return Ok(GeneratorOutput::Regenerated(Vec::new()));
    }
    let reg_rel = "registries/refusals.toml";
    let reg_path = repo_root.join(reg_rel);
    let mk_err = |path: &str, message: String, repair: &str| GeneratorError {
        rule: RULE_DIFFERS,
        subject: "refusal-printout".to_owned(),
        path: path.to_owned(),
        message,
        repair: repair.to_owned(),
    };
    let reg_bytes = fs::read(&reg_path).map_err(|e| {
        mk_err(
            reg_rel,
            format!("cannot read `{reg_rel}`: {e}"),
            "Restore registries/refusals.toml and re-run `cargo xtask check-generated --write`.",
        )
    })?;
    let alloc_bytes = fs::read(&alloc_path).map_err(|e| {
        mk_err(
            alloc_rel,
            format!("cannot read `{alloc_rel}`: {e}"),
            "Restore registries/refusal_allocations.toml and re-run `cargo xtask check-generated --write`.",
        )
    })?;
    let reg = RefusalRegistry::from_bytes(&reg_bytes).map_err(|e| {
        mk_err(
            reg_rel,
            format!("invalid refusal registry: {e}"),
            "Fix registries/refusals.toml so it passes frob.refusals/1 validation.",
        )
    })?;
    let ledger = AllocationLedger::from_bytes(&alloc_bytes).map_err(|e| {
        mk_err(
            alloc_rel,
            format!("invalid refusal allocation ledger: {e}"),
            "Fix registries/refusal_allocations.toml so it passes frob.refalloc/1 validation.",
        )
    })?;
    let milestone = std::env::var_os("FROB_MILESTONE")
        .map(|raw| {
            Milestone::parse(&raw.to_string_lossy()).ok_or_else(|| {
                mk_err(
                    alloc_rel,
                    format!("invalid FROB_MILESTONE {:?}", raw),
                    "Set FROB_MILESTONE to the milestone under review (M0 through M6).",
                )
            })
        })
        .transpose()?;
    let report = run_cross_chapter_checks(&reg, &ledger, milestone).map_err(|e| {
        mk_err(
            alloc_rel,
            format!("cross-chapter refusal check failed: {e}"),
            "Resolve the refusal registry/ledger inconsistency reported above and re-run `cargo xtask check-generated --write`.",
        )
    })?;

    let md = render_refusals_markdown(&reg, &ledger);
    let mut facts = BTreeMap::new();
    facts.insert(
        "codes".into(),
        FactValue::str_array(reg.rows().iter().map(|row| &row.code)),
    );
    for (key, value) in [
        ("registry_rows", report.registry_rows),
        ("printed_rows_target", report.printed_target),
        ("assigned", report.assigned_count),
        ("merged", report.merged_count),
        ("interim_unsettled", report.interims.len()),
    ] {
        facts.insert(key.into(), FactValue::int_usize(value));
    }
    facts.insert(
        "pending_chapters".into(),
        FactValue::Object(
            report
                .pending_chapters
                .into_iter()
                .map(|(owner, due, missing)| {
                    (
                        owner,
                        FactValue::Object(BTreeMap::from([
                            ("due".into(), FactValue::str(due.as_str())),
                            ("missing".into(), FactValue::int_usize(missing)),
                        ])),
                    )
                })
                .collect(),
        ),
    );
    facts.insert(
        "pending_assigned".into(),
        FactValue::Object(
            report
                .pending_assigned
                .into_iter()
                .map(|(code, owner)| (code, FactValue::str(owner)))
                .collect(),
        ),
    );
    facts.insert(
        "interims".into(),
        FactValue::Array(
            report
                .interims
                .into_iter()
                .map(|entry| {
                    let interim = match entry.codes.as_slice() {
                        [] => FactValue::str("none"),
                        [code] => FactValue::str(code),
                        codes => FactValue::str_array(codes),
                    };
                    FactValue::Object(BTreeMap::from([
                        ("request".into(), FactValue::str(entry.slug)),
                        ("interim".into(), interim),
                        ("detail".into(), FactValue::str(entry.detail)),
                        ("pointer".into(), FactValue::str(entry.pointer)),
                        ("owner".into(), FactValue::str(entry.owner)),
                        ("status".into(), FactValue::str(entry.status.as_str())),
                    ]))
                })
                .collect(),
        ),
    );
    Ok(GeneratorOutput::RegeneratedWithFacts(
        vec![GeneratedFile {
            rel_path: "docs/generated/refusals.md".to_owned(),
            kind: OutputKind::WholeFile,
            marker: ExpectedMarker::LinePrefix("<!-- @generated"),
            bytes: md.into_bytes(),
        }],
        facts,
    ))
}
