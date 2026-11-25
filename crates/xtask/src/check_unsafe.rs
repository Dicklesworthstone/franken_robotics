//! `xtask check-unsafe`: core deny lint, annex capsule registry with `SAFETY.md`,
//! lint-table inheritance, and doorway scan (bead N34-07).
//!
//! Enforces eleven ordered rules over workspace manifests, `registries/capsules.toml`
//! (read strictly through `frob_core::registry::toml`), `SAFETY.md` files, and Rust
//! source files without compiling anything or invoking `cargo`:
//! 1. `unsafe.core-lint`: root `[workspace.lints.rust]` has `unsafe_code = "deny"` and
//!    no core `.rs` file allows `unsafe_code`.
//! 2. `unsafe.core-token`: keyword `unsafe` never appears in any core `.rs` file
//!    (tokenizer skips comments/strings/char literals, never skips `#[cfg(...)]`).
//! 3. `unsafe.cert-forbid`: `crates/frob-cert/src/lib.rs` and every binary root of
//!    `frob-cert` carry crate-level `#![forbid(unsafe_code)]`.
//! 4. `capsule.unregistered`: every annex `.rs` file containing an `unsafe` construct
//!    has a covering `[[capsule]]` row in `registries/capsules.toml`.
//! 5. `capsule.over-limit`: every registered capsule file has strictly fewer than 300 lines.
//! 6. `capsule.count-mismatch`: a capsule row's `lines` and `unsafe_blocks` equal the
//!    measured counts in its file.
//! 7. `capsule.no-safety-doc`: `safety_doc` exists and has all four required sections,
//!    each non-empty.
//! 8. `capsule.unsafe-in-api`: no item named in `safe_api` is `unsafe fn` or returns
//!    a raw pointer (`*const` / `*mut`).
//! 9. `capsule.stale-row`: every capsule row's `path` exists on disk.
//! 10. `lints.not-inherited`: every core crate (except standalone `frob-cert`, ADR-WS-02)
//!     inherits `[lints] workspace = true`, and no core library target overrides
//!     `deprecated`, `clippy::print_stdout`, `clippy::print_stderr`, or `clippy::dbg_macro`
//!     at crate level.
//! 11. `doorway.unlisted-allow`: no `allow(deprecated)` attribute exists anywhere in core
//!     sources outside the compiled [`DOORWAY_SITES`] table.
//!
//! Determinism class D0: findings and report facts are strictly sorted and repeatable.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

use frob_core::registry::toml::{self, Fields, Spanned, Table, Value};

use crate::cli::{CheckContext, CheckError, CheckOutcome};
use crate::crate_map::Finding;
use crate::metadata::{WorkspaceKind, WorkspaceRoot, canonical_or_clean, normalize_rel_path, normalize_rel_str};
use crate::report::FactValue;

/// Rule 1: root `[workspace.lints.rust]` lacks `unsafe_code = "deny"` or a core file allows `unsafe_code`.
pub const RULE_UNSAFE_CORE_LINT: &str = "unsafe.core-lint";
/// Rule 2: keyword `unsafe` appears in a core source file (including behind `#[cfg(...)]`).
pub const RULE_UNSAFE_CORE_TOKEN: &str = "unsafe.core-token"; // ubs:ignore (rule identifier, not a secret)
/// Rule 3: `crates/frob-cert` library or binary root lacks `#![forbid(unsafe_code)]`.
pub const RULE_UNSAFE_CERT_FORBID: &str = "unsafe.cert-forbid";
/// Rule 4: an annex source file uses `unsafe` without a covering row in `registries/capsules.toml`.
pub const RULE_CAPSULE_UNREGISTERED: &str = "capsule.unregistered";
/// Rule 5: a registered capsule file has 300 lines or more.
pub const RULE_CAPSULE_OVER_LIMIT: &str = "capsule.over-limit";
/// Rule 6: a capsule row's `lines` or `unsafe_blocks` disagrees with the measured file counts.
pub const RULE_CAPSULE_COUNT_MISMATCH: &str = "capsule.count-mismatch";
/// Rule 7: a capsule's `SAFETY.md` is missing, lacks a required heading, or has an empty section.
pub const RULE_CAPSULE_NO_SAFETY_DOC: &str = "capsule.no-safety-doc";
/// Rule 8: an item listed in `safe_api` is `unsafe fn` or returns a raw pointer.
pub const RULE_CAPSULE_UNSAFE_IN_API: &str = "capsule.unsafe-in-api";
/// Rule 9: a capsule registry row points to a file that does not exist.
pub const RULE_CAPSULE_STALE_ROW: &str = "capsule.stale-row";
/// Rule 10: a core crate lacks `[lints] workspace = true` or overrides guarded lints at library crate level.
pub const RULE_LINTS_NOT_INHERITED: &str = "lints.not-inherited";
/// Rule 11: an `allow(deprecated)` attribute appears in core sources outside [`DOORWAY_SITES`].
pub const RULE_DOORWAY_UNLISTED_ALLOW: &str = "doorway.unlisted-allow";

/// Complete list of finding rule identifiers owned exclusively by `check-unsafe`.
pub const OWNED_RULES: &[&str] = &[
    RULE_UNSAFE_CORE_LINT,
    RULE_UNSAFE_CORE_TOKEN,
    RULE_UNSAFE_CERT_FORBID,
    RULE_CAPSULE_UNREGISTERED,
    RULE_CAPSULE_OVER_LIMIT,
    RULE_CAPSULE_COUNT_MISMATCH,
    RULE_CAPSULE_NO_SAFETY_DOC,
    RULE_CAPSULE_UNSAFE_IN_API,
    RULE_CAPSULE_STALE_ROW,
    RULE_LINTS_NOT_INHERITED,
    RULE_DOORWAY_UNLISTED_ALLOW,
];

/// Hard per-capsule file-length limit: a capsule must have strictly fewer than 300 lines.
pub const CAPSULE_MAX_LINES: usize = 300;

/// Schema version required in `registries/capsules.toml`.
pub const CAPSULES_SCHEMA: &str = "frob.capsules/1";

/// Required section headings of every capsule `SAFETY.md` (rule 7).
pub const SAFETY_DOC_SECTIONS: &[&str] = &[
    "What is unsafe here",
    "Invariants the safe API maintains",
    "Why safe Rust cannot do this",
    "How it is tested",
];

/// Lints guarded against crate-level overrides in core library targets (rule 10).
pub const GUARDED_LIBRARY_LINTS: &[&str] = &[
    "deprecated",
    "clippy::print_stdout",
    "clippy::print_stderr",
    "clippy::dbg_macro",
];

/// One permitted `allow(deprecated)` site in the compiled doorway table (rule 11).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DoorwaySite {
    pub crate_name: &'static str,
    /// Exact repo-relative file path or directory prefix ending in `/`.
    pub path_patterns: &'static [&'static str],
    /// Doorway constructors in `frob-core` callable from this site.
    pub constructors: &'static [&'static str],
    pub description: &'static str,
}

impl DoorwaySite {
    #[must_use]
    pub fn matches_path(&self, rel_path: &str) -> bool {
        self.path_patterns.iter().any(|pat| {
            if pat.ends_with('/') {
                rel_path.starts_with(pat)
            } else {
                rel_path == *pat
            }
        })
    }
}

/// Initial compiled table of permitted doorway `allow(deprecated)` call sites (N34-07 rule 11).
///
/// The doorway constructors live in `frob-core::evidence`, are `#[doc(hidden)]`, and are
/// marked `#[deprecated]`. Only these core modules may opt out of `deprecated`:
/// - `frob-lemma` issuing module for Proof-claim constructors (`doorway_certified`,
///   `doorway_proved`, `proof_certified`, `proof_proved`);
/// - `frob-conformance` harness module and `frob-arena` study runner for Tested-leaf
///   constructors (`doorway_tested`, `tested`).
pub const DOORWAY_SITES: &[DoorwaySite] = &[
    DoorwaySite {
        crate_name: "frob-lemma",
        path_patterns: &[
            "crates/frob-lemma/src/issue.rs",
            "crates/frob-lemma/src/issue/",
        ],
        constructors: &[
            "Evidence::doorway_certified",
            "Evidence::doorway_proved",
            "ClaimProduct::proof_certified",
            "ClaimProduct::proof_proved",
        ],
        description: "Proof-claim evidence issuing module of frob-lemma",
    },
    DoorwaySite {
        crate_name: "frob-conformance",
        path_patterns: &[
            "crates/frob-conformance/src/harness.rs",
            "crates/frob-conformance/src/harness/",
        ],
        constructors: &["Evidence::doorway_tested", "ClaimProduct::tested"],
        description: "Gauntlet harness module of frob-conformance",
    },
    DoorwaySite {
        crate_name: "frob-arena",
        path_patterns: &[
            "crates/frob-arena/src/runner.rs",
            "crates/frob-arena/src/study_runner.rs",
            "crates/frob-arena/src/runner/",
        ],
        constructors: &["Evidence::doorway_tested", "ClaimProduct::tested"],
        description: "Study runner module of frob-arena",
    },
];

/// One parsed `[[capsule]]` row from `registries/capsules.toml`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapsuleRow {
    pub id: String,
    pub workspace: String,
    pub path: String,
    pub safe_api: Vec<String>,
    pub lines: i64,
    pub unsafe_blocks: i64,
    pub safety_doc: String,
    pub reason: String,
    pub toml_line: u32,
}

/// Report-fact record for one registered capsule.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapsuleFact {
    pub id: String,
    pub workspace: String,
    pub path: String,
    pub lines: usize,
    pub unsafe_blocks: usize,
    pub safe_api: Vec<String>,
}

/// Aggregated annex capsule totals (no threshold applied).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct AnnexTotals {
    pub capsules: usize,
    pub lines: usize,
    pub unsafe_blocks: usize,
}

/// CLI handler plugged into the static `COMMANDS` array in `src/main.rs`.
pub fn run_command(ctx: &CheckContext) -> Result<CheckOutcome, CheckError> {
    if !ctx.extra_args.is_empty() {
        return Err(CheckError::Usage(format!(
            "check-unsafe accepts no extra arguments: {}",
            ctx.extra_args.join(" ")
        )));
    }
    check_unsafe(&ctx.repo_root, &ctx.roots)
}

/// Run `check-unsafe` against `repo_root` and `roots` using the compiled [`DOORWAY_SITES`].
pub fn check_unsafe(repo_root: &Path, roots: &[WorkspaceRoot]) -> Result<CheckOutcome, CheckError> {
    check_unsafe_with_doorways(repo_root, roots, DOORWAY_SITES)
}

/// Run `check-unsafe` with an explicit doorway table (also usable by unit tests).
pub fn check_unsafe_with_doorways(
    repo_root: &Path,
    roots: &[WorkspaceRoot],
    doorways: &[DoorwaySite],
) -> Result<CheckOutcome, CheckError> {
    let core_root = roots
        .iter()
        .find(|w| w.kind == WorkspaceKind::Core)
        .map_or_else(|| repo_root.to_path_buf(), |w| w.root_dir.clone());
    let root_manifest_path = core_root.join("Cargo.toml");
    if !root_manifest_path.is_file() {
        return Err(CheckError::Usage(format!(
            "core workspace manifest not found at {}",
            root_manifest_path.display()
        )));
    }

    let root_manifest_text = fs::read_to_string(&root_manifest_path).map_err(|e| {
        CheckError::Fault(format!("failed reading {}: {e}", root_manifest_path.display()))
    })?;
    let root_manifest = parse_cargo_manifest(&root_manifest_text);

    let mut findings: Vec<Finding> = Vec::new();

    // Rule 1 (manifest half): root `[workspace.lints.rust]` must set `unsafe_code = "deny"` (or `"forbid"`).
    check_root_unsafe_lint(&root_manifest, &mut findings);

    // Discover core crates from root `[workspace] members` (without calling `cargo`).
    let core_crates = discover_core_crates(repo_root, &core_root, &root_manifest)?;

    // Rules 1 (source half), 2, 3, 10, and 11 across every core crate.
    for crate_info in &core_crates {
        check_core_crate(crate_info, doorways, &mut findings)?;
    }

    // Parse `registries/capsules.toml` strictly through `frob_core::registry::toml`.
    let capsules_path = repo_root.join("registries").join("capsules.toml");
    let capsule_rows = if capsules_path.is_file() {
        let raw_bytes = fs::read(&capsules_path).map_err(|e| {
            CheckError::Fault(format!("failed reading {}: {e}", capsules_path.display()))
        })?;
        parse_capsules_toml(&raw_bytes).map_err(|e| {
            CheckError::Fault(format!("invalid registries/capsules.toml: {e}"))
        })?
    } else {
        Vec::new()
    };

    // Index registered capsules by normalized repo-relative path.
    let mut registered_paths: BTreeSet<String> = BTreeSet::new();
    for row in &capsule_rows {
        registered_paths.insert(row.path.clone());
    }

    // Discover annex `.rs` files from explicit Annex roots and `<repo_root>/spark/`.
    let annex_files = discover_annex_rs_files(repo_root, roots)?;

    // Rule 4: any annex `.rs` file containing `unsafe` without a covering capsule row fails.
    for annex_file in &annex_files {
        if registered_paths.contains(&annex_file.rel_path) {
            continue;
        }
        let text = fs::read_to_string(&annex_file.abs_path).map_err(|e| {
            CheckError::Fault(format!("failed reading {}: {e}", annex_file.abs_path.display()))
        })?;
        let tokens = tokenize_rust(&text);
        let unsafe_lines = find_unsafe_keyword_lines(&tokens);
        if let Some(&first_line) = unsafe_lines.first() {
            findings.push(Finding {
                rule: RULE_CAPSULE_UNREGISTERED,
                subject: annex_file.rel_path.clone(),
                path: annex_file.rel_path.clone(),
                line: Some(first_line),
                message: format!(
                    "annex source `{}` contains {} `unsafe` construct(s) with no covering `[[capsule]]` row in registries/capsules.toml",
                    annex_file.rel_path,
                    unsafe_lines.len()
                ),
                repair: "register a `[[capsule]]` row in `registries/capsules.toml` and add a `SAFETY.md` copied from `registries/capsules.SAFETY.template.md`".to_owned(),
            });
        }
    }

    // Rules 5, 6, 7, 8, and 9 for every registered `[[capsule]]` row, plus report facts.
    let mut capsule_facts: Vec<CapsuleFact> = Vec::with_capacity(capsule_rows.len());
    let mut totals_per_annex: BTreeMap<String, AnnexTotals> = BTreeMap::new();

    for row in &capsule_rows {
        let abs_capsule = repo_root.join(&row.path);
        if !abs_capsule.is_file() {
            // Rule 9: stale row.
            findings.push(Finding {
                rule: RULE_CAPSULE_STALE_ROW,
                subject: row.id.clone(),
                path: row.path.clone(),
                line: Some(row.toml_line),
                message: format!(
                    "capsule `{}` references missing file `{}`",
                    row.id, row.path
                ),
                repair: format!(
                    "remove stale `[[capsule]]` row `{}` from `registries/capsules.toml` or restore `{}`",
                    row.id, row.path
                ),
            });
            continue;
        }

        let source = fs::read_to_string(&abs_capsule).map_err(|e| {
            CheckError::Fault(format!("failed reading {}: {e}", abs_capsule.display()))
        })?;
        let measured_lines = count_lines(&source);
        let tokens = tokenize_rust(&source);
        let measured_unsafe = find_unsafe_keyword_lines(&tokens).len();

        // Rule 5: 300 lines or more fails (299 passes).
        if measured_lines >= CAPSULE_MAX_LINES {
            findings.push(Finding {
                rule: RULE_CAPSULE_OVER_LIMIT,
                subject: row.id.clone(),
                path: row.path.clone(),
                line: None,
                message: format!(
                    "capsule `{}` has {measured_lines} lines (limit is strictly < {CAPSULE_MAX_LINES}, blank and comment lines included)",
                    row.id
                ),
                repair: format!(
                    "split `{}` along its safe API so each capsule file has fewer than {CAPSULE_MAX_LINES} lines",
                    row.path
                ),
            });
        }

        // Rule 6: declared `lines` and `unsafe_blocks` must equal measured counts.
        let declared_lines = i64::try_from(measured_lines).unwrap_or(i64::MAX);
        let declared_unsafe = i64::try_from(measured_unsafe).unwrap_or(i64::MAX);
        if row.lines != declared_lines || row.unsafe_blocks != declared_unsafe {
            findings.push(Finding {
                rule: RULE_CAPSULE_COUNT_MISMATCH,
                subject: row.id.clone(),
                path: "registries/capsules.toml".to_owned(),
                line: Some(row.toml_line),
                message: format!(
                    "capsule `{}` declares lines={} unsafe_blocks={}, but `{}` measures lines={measured_lines} unsafe_blocks={measured_unsafe}",
                    row.id, row.lines, row.unsafe_blocks, row.path
                ),
                repair: format!(
                    "review `{}` and update `lines = {measured_lines}` and `unsafe_blocks = {measured_unsafe}` in `registries/capsules.toml`",
                    row.path
                ),
            });
        }

        // Rule 7: `SAFETY.md` exists and has all 4 required non-empty sections.
        check_safety_doc(repo_root, row, &mut findings)?;

        // Rule 8: no item listed in `safe_api` may be `unsafe fn` or return a raw pointer.
        check_safe_api_items(row, &tokens, &mut findings);

        let totals = totals_per_annex.entry(row.workspace.clone()).or_default();
        totals.capsules += 1;
        totals.lines += measured_lines;
        totals.unsafe_blocks += measured_unsafe;

        capsule_facts.push(CapsuleFact {
            id: row.id.clone(),
            workspace: row.workspace.clone(),
            path: row.path.clone(),
            lines: measured_lines,
            unsafe_blocks: measured_unsafe,
            safe_api: row.safe_api.clone(),
        });
    }

    findings.sort();
    findings.dedup();
    capsule_facts.sort_by(|a, b| a.id.cmp(&b.id).then_with(|| a.path.cmp(&b.path)));

    let facts = build_facts(
        &capsule_facts,
        &totals_per_annex,
        core_crates.len(),
        annex_files.len(),
    );

    Ok(CheckOutcome { findings, facts })
}

/// Render a deterministic human-readable per-capsule table and per-annex totals.
#[must_use]
pub fn format_capsule_table(
    capsules: &[CapsuleFact],
    totals: &BTreeMap<String, AnnexTotals>,
) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "capsules ({}):", capsules.len());
    if capsules.is_empty() {
        out.push_str("  (none)\n");
    } else {
        for c in capsules {
            let _ = writeln!(
                out,
                "  {}  workspace={}  lines={}/{}  unsafe_blocks={}  safe_api=[{}]",
                c.id,
                c.workspace,
                c.lines,
                CAPSULE_MAX_LINES - 1,
                c.unsafe_blocks,
                c.safe_api.join(", ")
            );
        }
    }
    if !totals.is_empty() {
        out.push_str("totals_per_annex:\n");
        for (ws, t) in totals {
            let _ = writeln!(
                out,
                "  {ws}: capsules={} lines={} unsafe_blocks={}",
                t.capsules, t.lines, t.unsafe_blocks
            );
        }
    }
    out
}

fn build_facts(
    capsules: &[CapsuleFact],
    totals_per_annex: &BTreeMap<String, AnnexTotals>,
    core_crates_checked: usize,
    annex_files_scanned: usize,
) -> BTreeMap<String, FactValue> {
    let mut facts = BTreeMap::new();

    let capsule_vals: Vec<FactValue> = capsules
        .iter()
        .map(|c| {
            let mut obj = BTreeMap::new();
            obj.insert("id".to_owned(), FactValue::str(c.id.clone()));
            obj.insert("workspace".to_owned(), FactValue::str(c.workspace.clone()));
            obj.insert("path".to_owned(), FactValue::str(c.path.clone()));
            obj.insert("lines".to_owned(), FactValue::int_usize(c.lines));
            obj.insert(
                "unsafe_blocks".to_owned(),
                FactValue::int_usize(c.unsafe_blocks),
            );
            obj.insert(
                "safe_api".to_owned(),
                FactValue::str_array(c.safe_api.iter().cloned()),
            );
            FactValue::Object(obj)
        })
        .collect();

    let mut annex_map = BTreeMap::new();
    for (ws, t) in totals_per_annex {
        let mut obj = BTreeMap::new();
        obj.insert("capsules".to_owned(), FactValue::int_usize(t.capsules));
        obj.insert("lines".to_owned(), FactValue::int_usize(t.lines));
        obj.insert(
            "unsafe_blocks".to_owned(),
            FactValue::int_usize(t.unsafe_blocks),
        );
        annex_map.insert(ws.clone(), FactValue::Object(obj));
    }

    facts.insert(
        "annex_files_scanned".to_owned(),
        FactValue::int_usize(annex_files_scanned),
    );
    facts.insert(
        "capsule_count".to_owned(),
        FactValue::int_usize(capsules.len()),
    );
    facts.insert("capsules".to_owned(), FactValue::Array(capsule_vals));
    facts.insert(
        "core_crates_checked".to_owned(),
        FactValue::int_usize(core_crates_checked),
    );
    facts.insert(
        "totals_per_annex".to_owned(),
        FactValue::Object(annex_map),
    );
    facts
}

// ============================================================================
// Capsule registry TOML parser (strictly via frob_core::registry::toml)
// ============================================================================

/// Parse `registries/capsules.toml` using `frob_core::registry::toml` (N34p2-01).
pub fn parse_capsules_toml(bytes: &[u8]) -> Result<Vec<CapsuleRow>, String> {
    let root = toml::parse(bytes).map_err(|e| e.to_string())?;
    let mut rf = Fields::new(root, "capsules.toml");
    let schema = rf.require_str("schema").map_err(|e| e.to_string())?;
    let raw_rows = rf.opt_array("capsule").map_err(|e| e.to_string())?;
    rf.finish().map_err(|e| e.to_string())?;

    if schema != CAPSULES_SCHEMA {
        return Err(format!(
            "unexpected schema `{schema}` (expected `{CAPSULES_SCHEMA}`)"
        ));
    }

    let Some(items) = raw_rows else {
        return Ok(Vec::new());
    };

    let mut rows = Vec::with_capacity(items.len());
    let mut seen_ids = BTreeSet::new();
    let mut seen_paths = BTreeSet::new();

    for (idx, item) in items.into_iter().enumerate() {
        let ctx = format!("capsules.toml:capsule[{idx}]");
        let toml_line = item.line;
        let tbl: Table = match item.value {
            Value::Table(t) => t,
            other => {
                return Err(format!(
                    "{ctx}: expected table, found {}",
                    other.type_name()
                ));
            }
        };
        let mut f = Fields::new(tbl, &ctx);
        let id = f.require_str("id").map_err(|e| e.to_string())?;
        let workspace = f.require_str("workspace").map_err(|e| e.to_string())?;
        let raw_path = f.require_str("path").map_err(|e| e.to_string())?;
        let raw_safe_api = f.require_array("safe_api").map_err(|e| e.to_string())?;
        let lines = f.require_int("lines").map_err(|e| e.to_string())?;
        let unsafe_blocks = f.require_int("unsafe_blocks").map_err(|e| e.to_string())?;
        let raw_safety_doc = f.require_str("safety_doc").map_err(|e| e.to_string())?;
        let reason = f.require_str("reason").map_err(|e| e.to_string())?;
        f.finish().map_err(|e| e.to_string())?;

        if id.trim().is_empty() {
            return Err(format!("{ctx}: `id` must not be empty"));
        }
        if workspace.trim().is_empty() {
            return Err(format!("{ctx}: `workspace` must not be empty"));
        }
        let path = normalize_rel_str(&raw_path.replace('\\', "/"));
        if path.is_empty() {
            return Err(format!("{ctx}: `path` must not be empty"));
        }
        let safety_doc = normalize_rel_str(&raw_safety_doc.replace('\\', "/"));
        if safety_doc.is_empty() {
            return Err(format!("{ctx}: `safety_doc` must not be empty"));
        }
        if reason.trim().is_empty() {
            return Err(format!("{ctx}: `reason` must not be empty"));
        }
        let safe_api = extract_non_empty_strings(raw_safe_api, &ctx, "safe_api")?;
        if safe_api.is_empty() {
            return Err(format!("{ctx}: `safe_api` must not be empty"));
        }
        if !seen_ids.insert(id.clone()) {
            return Err(format!("{ctx}: duplicate capsule id `{id}`"));
        }
        if !seen_paths.insert(path.clone()) {
            return Err(format!("{ctx}: duplicate capsule path `{path}`"));
        }

        rows.push(CapsuleRow {
            id,
            workspace,
            path,
            safe_api,
            lines,
            unsafe_blocks,
            safety_doc,
            reason,
            toml_line,
        });
    }

    Ok(rows)
}

fn extract_non_empty_strings(
    items: Vec<Spanned<Value>>,
    ctx: &str,
    field: &str,
) -> Result<Vec<String>, String> {
    let mut out = Vec::with_capacity(items.len());
    for (idx, sp) in items.into_iter().enumerate() {
        match sp.value {
            Value::Str(s) => {
                if s.trim().is_empty() {
                    return Err(format!("{ctx}: `{field}[{idx}]` must not be empty"));
                }
                out.push(s);
            }
            other => {
                return Err(format!(
                    "{ctx}: `{field}[{idx}]` expected string, found {}",
                    other.type_name()
                ));
            }
        }
    }
    Ok(out)
}

// ============================================================================
// Core & annex workspace discovery (zero-compile, reads manifests/files only)
// ============================================================================

#[derive(Debug, Clone)]
struct CoreCrateInfo {
    name: String,
    rel_dir: String,
    rel_manifest: String,
    manifest: ParsedCargoManifest,
    lib_root_rel: Option<String>,
    bin_roots_rel: BTreeSet<String>,
    rs_files: Vec<DiscoveredFile>,
}

#[derive(Debug, Clone)]
struct DiscoveredFile {
    rel_path: String,
    abs_path: PathBuf,
}

fn discover_core_crates(
    repo_root: &Path,
    core_root: &Path,
    root_manifest: &ParsedCargoManifest,
) -> Result<Vec<CoreCrateInfo>, CheckError> {
    let canon_repo = canonical_or_clean(repo_root);
    let excludes: BTreeSet<String> = root_manifest
        .workspace_exclude
        .iter()
        .map(|e| normalize_rel_str(&e.replace('\\', "/")))
        .collect();

    let mut crate_dirs: BTreeMap<String, PathBuf> = BTreeMap::new();

    for member in &root_manifest.workspace_members {
        let pattern = normalize_rel_str(&member.replace('\\', "/"));
        if pattern.is_empty() {
            continue;
        }
        if let Some(prefix) = pattern.strip_suffix("/*") {
            let parent = if prefix.is_empty() {
                core_root.to_path_buf()
            } else {
                core_root.join(prefix)
            };
            if !parent.is_dir() {
                continue;
            }
            let entries = fs::read_dir(&parent).map_err(|e| {
                CheckError::Fault(format!("failed reading dir {}: {e}", parent.display()))
            })?;
            for entry_res in entries {
                let entry = entry_res.map_err(|e| {
                    CheckError::Fault(format!("failed reading dir entry in {}: {e}", parent.display()))
                })?;
                let child = entry.path();
                if !child.is_dir() {
                    continue;
                }
                if let Some(fname) = child.file_name().and_then(|s| s.to_str())
                    && (fname.starts_with('.') || fname == "target")
                {
                    continue;
                }
                if !child.join("Cargo.toml").is_file() {
                    continue;
                }
                let rel = rel_inside_or_fallback(&canon_repo, &child);
                if is_excluded(&rel, &excludes) {
                    continue;
                }
                crate_dirs.insert(rel, child);
            }
        } else {
            let dir = core_root.join(&pattern);
            if dir.join("Cargo.toml").is_file() {
                let rel = rel_inside_or_fallback(&canon_repo, &dir);
                if !is_excluded(&rel, &excludes) {
                    crate_dirs.insert(rel, dir);
                }
            }
        }
    }

    // If root manifest itself is also a package (non-virtual workspace), include it too.
    if root_manifest.package_name.is_some() {
        let rel = rel_inside_or_fallback(&canon_repo, core_root);
        crate_dirs.entry(rel).or_insert_with(|| core_root.to_path_buf());
    }

    let mut out = Vec::with_capacity(crate_dirs.len());
    for (rel_dir, abs_dir) in crate_dirs {
        let manifest_path = abs_dir.join("Cargo.toml");
        let manifest_text = fs::read_to_string(&manifest_path).map_err(|e| {
            CheckError::Fault(format!("failed reading {}: {e}", manifest_path.display()))
        })?;
        let manifest = parse_cargo_manifest(&manifest_text);
        let name = manifest.package_name.clone().unwrap_or_else(|| {
            abs_dir
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown")
                .to_owned()
        });
        let rel_manifest = if rel_dir.is_empty() {
            "Cargo.toml".to_owned()
        } else {
            format!("{rel_dir}/Cargo.toml")
        };

        let lib_root_rel = resolve_lib_root(&canon_repo, &abs_dir, &manifest);
        let bin_roots_rel = resolve_bin_roots(&canon_repo, &abs_dir, &manifest)?;
        let mut rs_files = Vec::new();
        collect_rs_files(&canon_repo, &abs_dir, &mut rs_files)?;
        rs_files.sort_by(|a, b| a.rel_path.cmp(&b.rel_path));

        out.push(CoreCrateInfo {
            name,
            rel_dir,
            rel_manifest,
            manifest,
            lib_root_rel,
            bin_roots_rel,
            rs_files,
        });
    }

    out.sort_by(|a, b| a.name.cmp(&b.name).then_with(|| a.rel_dir.cmp(&b.rel_dir)));
    Ok(out)
}

fn resolve_lib_root(
    canon_repo: &Path,
    crate_dir: &Path,
    manifest: &ParsedCargoManifest,
) -> Option<String> {
    if let Some(custom) = &manifest.lib_path {
        let p = crate_dir.join(custom);
        if p.is_file() {
            return Some(rel_inside_or_fallback(canon_repo, &p));
        }
        return None;
    }
    let default_lib = crate_dir.join("src").join("lib.rs");
    if default_lib.is_file() {
        Some(rel_inside_or_fallback(canon_repo, &default_lib))
    } else {
        None
    }
}

fn resolve_bin_roots(
    canon_repo: &Path,
    crate_dir: &Path,
    manifest: &ParsedCargoManifest,
) -> Result<BTreeSet<String>, CheckError> {
    let mut roots = BTreeSet::new();
    let main_rs = crate_dir.join("src").join("main.rs");
    if main_rs.is_file() {
        roots.insert(rel_inside_or_fallback(canon_repo, &main_rs));
    }

    let bin_dir = crate_dir.join("src").join("bin");
    if bin_dir.is_dir() {
        let entries = fs::read_dir(&bin_dir).map_err(|e| {
            CheckError::Fault(format!("failed reading {}: {e}", bin_dir.display()))
        })?;
        for entry_res in entries {
            let entry = entry_res.map_err(|e| {
                CheckError::Fault(format!("failed reading dir entry in {}: {e}", bin_dir.display()))
            })?;
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("rs") {
                roots.insert(rel_inside_or_fallback(canon_repo, &path));
            } else if path.is_dir() {
                let nested_main = path.join("main.rs");
                if nested_main.is_file() {
                    roots.insert(rel_inside_or_fallback(canon_repo, &nested_main));
                }
            }
        }
    }

    for custom in &manifest.bin_paths {
        let p = crate_dir.join(custom);
        if p.is_file() {
            roots.insert(rel_inside_or_fallback(canon_repo, &p));
        }
    }

    Ok(roots)
}

fn discover_annex_rs_files(
    repo_root: &Path,
    roots: &[WorkspaceRoot],
) -> Result<Vec<DiscoveredFile>, CheckError> {
    let canon_repo = canonical_or_clean(repo_root);
    let mut by_rel: BTreeMap<String, PathBuf> = BTreeMap::new();

    for ws in roots {
        if ws.kind == WorkspaceKind::Annex && ws.root_dir.is_dir() {
            let mut found = Vec::new();
            collect_rs_files(&canon_repo, &ws.root_dir, &mut found)?;
            for f in found {
                by_rel.insert(f.rel_path, f.abs_path);
            }
        }
    }

    // Auto-discover annex trees under `<repo_root>/spark/` even when `cli::run` passes only core root.
    let spark_dir = repo_root.join("spark");
    if spark_dir.is_dir() {
        let mut found = Vec::new();
        collect_rs_files(&canon_repo, &spark_dir, &mut found)?;
        for f in found {
            by_rel.insert(f.rel_path, f.abs_path);
        }
    }

    Ok(by_rel
        .into_iter()
        .map(|(rel_path, abs_path)| DiscoveredFile { rel_path, abs_path })
        .collect())
}

fn collect_rs_files(
    canon_repo: &Path,
    dir: &Path,
    out: &mut Vec<DiscoveredFile>,
) -> Result<(), CheckError> {
    let entries = fs::read_dir(dir).map_err(|e| {
        CheckError::Fault(format!("failed reading directory {}: {e}", dir.display()))
    })?;
    let mut paths: Vec<PathBuf> = Vec::new();
    for entry_res in entries {
        let entry = entry_res.map_err(|e| {
            CheckError::Fault(format!("failed reading dir entry in {}: {e}", dir.display()))
        })?;
        paths.push(entry.path());
    }
    paths.sort();

    for path in paths {
        let Some(name) = path.file_name().and_then(|s| s.to_str()) else {
            continue;
        };
        if name.starts_with('.') || name == "target" {
            continue;
        }
        if path.is_dir() {
            collect_rs_files(canon_repo, &path, out)?;
        } else if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("rs") {
            let rel_path = rel_inside_or_fallback(canon_repo, &path);
            out.push(DiscoveredFile {
                rel_path,
                abs_path: path,
            });
        }
    }
    Ok(())
}

fn rel_inside_or_fallback(canon_repo: &Path, target: &Path) -> String {
    let canon_target = canonical_or_clean(target);
    if let Ok(rel) = canon_target.strip_prefix(canon_repo) {
        normalize_rel_path(rel)
    } else {
        normalize_rel_path(target)
    }
}

fn is_excluded(rel_dir: &str, excludes: &BTreeSet<String>) -> bool {
    excludes.iter().any(|ex| {
        !ex.is_empty()
            && (rel_dir == ex
                || rel_dir
                    .strip_prefix(ex.as_str())
                    .is_some_and(|rest| rest.starts_with('/')))
    })
}

// ============================================================================
// Core rules (Rules 1, 2, 3, 10, 11)
// ============================================================================

fn check_root_unsafe_lint(root_manifest: &ParsedCargoManifest, findings: &mut Vec<Finding>) {
    match &root_manifest.workspace_rust_unsafe_code {
        Some((level, _)) if level == "deny" || level == "forbid" => {}
        Some((level, line)) => {
            findings.push(Finding {
                rule: RULE_UNSAFE_CORE_LINT,
                subject: "workspace".to_owned(),
                path: "Cargo.toml".to_owned(),
                line: Some(*line),
                message: format!(
                    "[workspace.lints.rust] sets unsafe_code = \"{level}\", expected \"deny\""
                ),
                repair: "set `unsafe_code = \"deny\"` under `[workspace.lints.rust]` in `Cargo.toml`".to_owned(),
            });
        }
        None => {
            findings.push(Finding {
                rule: RULE_UNSAFE_CORE_LINT,
                subject: "workspace".to_owned(),
                path: "Cargo.toml".to_owned(),
                line: None,
                message: "[workspace.lints.rust] is missing `unsafe_code = \"deny\"`".to_owned(),
                repair: "add `[workspace.lints.rust]\\nunsafe_code = \"deny\"` to root `Cargo.toml`".to_owned(),
            });
        }
    }
}

fn check_core_crate(
    crate_info: &CoreCrateInfo,
    doorways: &[DoorwaySite],
    findings: &mut Vec<Finding>,
) -> Result<(), CheckError> {
    let is_frob_cert =
        crate_info.name == "frob-cert" || crate_info.rel_dir == "crates/frob-cert";

    // Rule 10 (manifest half): every core crate except standalone `frob-cert` (ADR-WS-02)
    // must inherit `[lints] workspace = true`.
    // Per rule 1, a missing `[lints] workspace = true` emits `lints.not-inherited` only.
    if !is_frob_cert {
        match crate_info.manifest.lints_workspace {
            Some((true, _)) => {}
            Some((false, line)) => {
                findings.push(Finding {
                    rule: RULE_LINTS_NOT_INHERITED,
                    subject: crate_info.name.clone(),
                    path: crate_info.rel_manifest.clone(),
                    line: Some(line),
                    message: format!(
                        "core crate `{}` sets `[lints] workspace = false` instead of `true`",
                        crate_info.name
                    ),
                    repair: format!(
                        "set `workspace = true` under `[lints]` in `{}`",
                        crate_info.rel_manifest
                    ),
                });
            }
            None => {
                findings.push(Finding {
                    rule: RULE_LINTS_NOT_INHERITED,
                    subject: crate_info.name.clone(),
                    path: crate_info.rel_manifest.clone(),
                    line: None,
                    message: format!(
                        "core crate `{}` does not inherit the workspace lint table",
                        crate_info.name
                    ),
                    repair: format!(
                        "add `[lints]\\nworkspace = true` to `{}`",
                        crate_info.rel_manifest
                    ),
                });
            }
        }
    }

    // Rule 10 (manifest override half): a core library crate cannot override guarded lints in Cargo.toml.
    if crate_info.lib_root_rel.is_some() {
        for ov in &crate_info.manifest.crate_lint_overrides {
            if is_guarded_library_lint(&ov.lint) && is_weakening_level(&ov.level) {
                findings.push(Finding {
                    rule: RULE_LINTS_NOT_INHERITED,
                    subject: crate_info.name.clone(),
                    path: crate_info.rel_manifest.clone(),
                    line: Some(ov.line),
                    message: format!(
                        "core library crate `{}` overrides `{}` to `{}` in manifest",
                        crate_info.name, ov.lint, ov.level
                    ),
                    repair: format!(
                        "remove the `{}` override from `{}` and inherit `[lints] workspace = true`",
                        ov.lint, crate_info.rel_manifest
                    ),
                });
            }
        }
    }

    // Scan every `.rs` source file in this core crate.
    for file in &crate_info.rs_files {
        let src = fs::read_to_string(&file.abs_path).map_err(|e| {
            CheckError::Fault(format!("failed reading {}: {e}", file.abs_path.display()))
        })?;
        let tokens = tokenize_rust(&src);
        let directives = parse_lint_directives(&tokens);

        let is_lib_root = crate_info
            .lib_root_rel
            .as_deref()
            .is_some_and(|p| p == file.rel_path);
        let is_bin_root = crate_info.bin_roots_rel.contains(&file.rel_path);

        // Rule 2: keyword `unsafe` anywhere in core source (including behind `#[cfg(...)]`).
        for line in find_unsafe_keyword_lines(&tokens) {
            findings.push(Finding {
                rule: RULE_UNSAFE_CORE_TOKEN,
                subject: crate_info.name.clone(),
                path: file.rel_path.clone(),
                line: Some(line),
                message: format!(
                    "keyword `unsafe` appears in core crate `{}` source file `{}`",
                    crate_info.name, file.rel_path
                ),
                repair: format!(
                    "remove `unsafe` from `{}` (core workspace forbids unsafe code; device bindings live in annex capsules)",
                    file.rel_path
                ),
            });
        }

        // Rule 3: `frob-cert` library and binary roots must carry crate-level `#![forbid(unsafe_code)]`.
        if is_frob_cert && (is_lib_root || is_bin_root) {
            let has_forbid = directives.iter().any(|d| {
                d.is_inner
                    && d.brace_depth == 0
                    && d.level == "forbid"
                    && is_unsafe_code_lint(&d.lint)
            });
            if !has_forbid {
                findings.push(Finding {
                    rule: RULE_UNSAFE_CERT_FORBID,
                    subject: crate_info.name.clone(),
                    path: file.rel_path.clone(),
                    line: Some(1),
                    message: format!(
                        "`frob-cert` crate root `{}` lacks crate-level `#![forbid(unsafe_code)]`",
                        file.rel_path
                    ),
                    repair: format!(
                        "add `#![forbid(unsafe_code)]` at the top of `{}`",
                        file.rel_path
                    ),
                });
            }
        }

        // Inspect lint attributes for Rules 1, 10, and 11.
        for d in &directives {
            // Rule 1 (source half): `#![allow(unsafe_code)]` or `#[allow(unsafe_code)]`.
            if is_unsafe_code_lint(&d.lint) && is_weakening_level(&d.level) {
                findings.push(Finding {
                    rule: RULE_UNSAFE_CORE_LINT,
                    subject: crate_info.name.clone(),
                    path: file.rel_path.clone(),
                    line: Some(d.line),
                    message: format!(
                        "core crate `{}` overrides `unsafe_code` with `{}` in `{}`",
                        crate_info.name, d.level, file.rel_path
                    ),
                    repair: format!(
                        "remove `{}` for `unsafe_code` from `{}`",
                        d.level, file.rel_path
                    ),
                });
            }

            // Rule 10 (library crate-level override half):
            // Crate-level (`#![...]`, brace_depth == 0) overrides of `deprecated` or L-7 print lints
            // are forbidden in a library target root (`src/lib.rs`), while allowed in binary roots.
            if is_lib_root
                && !is_bin_root
                && d.is_inner
                && d.brace_depth == 0
                && is_guarded_library_lint(&d.lint)
                && is_weakening_level(&d.level)
            {
                findings.push(Finding {
                    rule: RULE_LINTS_NOT_INHERITED,
                    subject: crate_info.name.clone(),
                    path: file.rel_path.clone(),
                    line: Some(d.line),
                    message: format!(
                        "core library `{}` overrides `{}` with `#![{}({})]` at crate level",
                        crate_info.name, d.lint, d.level, d.lint
                    ),
                    repair: format!(
                        "remove `#![{}({})]` from `{}` (libraries must stay silent under rule L-7 and keep `deprecated` denied)",
                        d.level, d.lint, file.rel_path
                    ),
                });
            }

            // Rule 11: `allow(deprecated)` anywhere in core outside `DOORWAY_SITES`.
            if is_deprecated_lint(&d.lint) && is_weakening_level(&d.level) {
                let permitted = doorways
                    .iter()
                    .any(|site| site.matches_path(&file.rel_path));
                if !permitted {
                    findings.push(Finding {
                        rule: RULE_DOORWAY_UNLISTED_ALLOW,
                        subject: crate_info.name.clone(),
                        path: file.rel_path.clone(),
                        line: Some(d.line),
                        message: format!(
                            "unlisted `{}` for `deprecated` in core file `{}` (not in xtask doorway table)",
                            d.level, file.rel_path
                        ),
                        repair: format!(
                            "remove `{}(deprecated)` from `{}` (only listed doorway sites in frob-lemma, frob-conformance, and frob-arena may allow `deprecated`)",
                            d.level, file.rel_path
                        ),
                    });
                }
            }
        }
    }

    // Edge case: `crates/frob-cert` exists with Cargo.toml, has no `src/lib.rs` (missing lib root) -> flag.
    if is_frob_cert && crate_info.lib_root_rel.is_none() && crate_info.bin_roots_rel.is_empty() {
        let expected_lib = format!("{}/src/lib.rs", crate_info.rel_dir);
        findings.push(Finding {
            rule: RULE_UNSAFE_CERT_FORBID,
            subject: crate_info.name.clone(),
            path: expected_lib.clone(),
            line: None,
            message: "`frob-cert` has no library or binary root carrying `#![forbid(unsafe_code)]`"
                .to_owned(),
            repair: format!("add `#![forbid(unsafe_code)]` to `{expected_lib}`"),
        });
    }

    Ok(())
}

fn is_unsafe_code_lint(lint: &str) -> bool {
    lint == "unsafe_code" || lint == "rust::unsafe_code"
}

fn is_deprecated_lint(lint: &str) -> bool {
    lint == "deprecated" || lint == "rust::deprecated"
}

fn is_guarded_library_lint(lint: &str) -> bool {
    is_deprecated_lint(lint)
        || lint == "clippy::print_stdout"
        || lint == "clippy::print_stderr"
        || lint == "clippy::dbg_macro"
}

fn is_weakening_level(level: &str) -> bool {
    matches!(level, "allow" | "warn" | "expect")
}

// ============================================================================
// Rule 7: SAFETY.md four-section validator
// ============================================================================

fn check_safety_doc(
    repo_root: &Path,
    row: &CapsuleRow,
    findings: &mut Vec<Finding>,
) -> Result<(), CheckError> {
    let abs_doc = repo_root.join(&row.safety_doc);
    if !abs_doc.is_file() {
        findings.push(Finding {
            rule: RULE_CAPSULE_NO_SAFETY_DOC,
            subject: row.id.clone(),
            path: row.safety_doc.clone(),
            line: None,
            message: format!(
                "capsule `{}` is missing its SAFETY.md at `{}`",
                row.id, row.safety_doc
            ),
            repair: format!(
                "copy `registries/capsules.SAFETY.template.md` to `{}` and fill all four sections",
                row.safety_doc
            ),
        });
        return Ok(());
    }

    let doc_text = fs::read_to_string(&abs_doc).map_err(|e| {
        CheckError::Fault(format!("failed reading {}: {e}", abs_doc.display()))
    })?;
    let parsed = parse_markdown_sections(&doc_text);

    for &required in SAFETY_DOC_SECTIONS {
        let norm_req = normalize_heading(required);
        match parsed.get(&norm_req) {
            None => {
                findings.push(Finding {
                    rule: RULE_CAPSULE_NO_SAFETY_DOC,
                    subject: row.id.clone(),
                    path: row.safety_doc.clone(),
                    line: None,
                    message: format!(
                        "capsule `{}` SAFETY.md `{}` is missing required section `{required}`",
                        row.id, row.safety_doc
                    ),
                    repair: format!(
                        "add a non-empty `## {required}` section to `{}` (see `registries/capsules.SAFETY.template.md`)",
                        row.safety_doc
                    ),
                });
            }
            Some(sec) if strip_html_comments(&sec.body).trim().is_empty() => {
                findings.push(Finding {
                    rule: RULE_CAPSULE_NO_SAFETY_DOC,
                    subject: row.id.clone(),
                    path: row.safety_doc.clone(),
                    line: Some(sec.heading_line),
                    message: format!(
                        "capsule `{}` SAFETY.md `{}` has an empty `{required}` section",
                        row.id, row.safety_doc
                    ),
                    repair: format!(
                        "fill the `## {required}` section in `{}` with concrete invariants/evidence",
                        row.safety_doc
                    ),
                });
            }
            Some(_) => {}
        }
    }

    Ok(())
}

#[derive(Debug, Clone)]
struct MarkdownSection {
    heading_line: u32,
    body: String,
}

fn parse_markdown_sections(text: &str) -> BTreeMap<String, MarkdownSection> {
    let mut sections: BTreeMap<String, MarkdownSection> = BTreeMap::new();
    let mut active: Option<(String, usize, u32, String)> = None;
    let mut in_fence: Option<char> = None;

    for (idx, raw_line) in text.lines().enumerate() {
        let line_no = u32::try_from(idx + 1).unwrap_or(u32::MAX);
        let trimmed_start = raw_line.trim_start();

        if let Some(fence_ch) = in_fence {
            if is_fence_delimiter(trimmed_start, Some(fence_ch)) {
                in_fence = None;
            }
            if let Some((_, _, _, ref mut body)) = active {
                body.push_str(raw_line);
                body.push('\n');
            }
            continue;
        }
        if let Some(fence_ch) = detect_fence_start(trimmed_start) {
            in_fence = Some(fence_ch);
            if let Some((_, _, _, ref mut body)) = active {
                body.push_str(raw_line);
                body.push('\n');
            }
            continue;
        }

        if let Some((level, title)) = parse_atx_heading(trimmed_start) {
            if let Some((key, active_level, heading_line, body)) = active.take() {
                if level <= active_level {
                    sections
                        .entry(key)
                        .or_insert(MarkdownSection { heading_line, body });
                } else {
                    // Subheading inside active section stays part of the section body.
                    let mut extended = body;
                    extended.push_str(raw_line);
                    extended.push('\n');
                    active = Some((key, active_level, heading_line, extended));
                    continue;
                }
            }
            let norm = normalize_heading(title);
            active = Some((norm, level, line_no, String::new()));
            continue;
        }

        if let Some((_, _, _, ref mut body)) = active {
            body.push_str(raw_line);
            body.push('\n');
        }
    }

    if let Some((key, _, heading_line, body)) = active {
        sections
            .entry(key)
            .or_insert(MarkdownSection { heading_line, body });
    }

    sections
}

fn detect_fence_start(trimmed: &str) -> Option<char> {
    if trimmed.starts_with("```") {
        Some('`')
    } else if trimmed.starts_with("~~~") {
        Some('~')
    } else {
        None
    }
}

fn is_fence_delimiter(trimmed: &str, ch: Option<char>) -> bool {
    let Some(c) = ch else {
        return false;
    };
    let count = trimmed.chars().take_while(|&x| x == c).count();
    count >= 3 && trimmed[count..].trim().is_empty()
}

fn parse_atx_heading(trimmed: &str) -> Option<(usize, &str)> {
    let level = trimmed.bytes().take_while(|&b| b == b'#').count();
    if !(1..=6).contains(&level) {
        return None;
    }
    let rest = &trimmed[level..];
    if !rest.is_empty() && !rest.starts_with(' ') && !rest.starts_with('\t') {
        return None;
    }
    let title = rest.trim().trim_end_matches('#').trim();
    if title.is_empty() {
        return None;
    }
    Some((level, title))
}

fn normalize_heading(raw: &str) -> String {
    raw.split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_ascii_lowercase()
}

fn strip_html_comments(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut rest = input;
    while let Some(start) = rest.find("<!--") {
        out.push_str(&rest[..start]);
        let after = &rest[start + 4..];
        if let Some(end) = after.find("-->") {
            rest = &after[end + 3..];
        } else {
            return out;
        }
    }
    out.push_str(rest);
    out
}

// ============================================================================
// Rule 8: `safe_api` signature audit (`unsafe fn` or raw-pointer return)
// ============================================================================

#[derive(Debug, Clone)]
struct ParsedFnDecl {
    impl_type: Option<String>,
    name: String,
    line: u32,
    is_unsafe: bool,
    returns_raw_pointer: bool,
}

fn check_safe_api_items(row: &CapsuleRow, tokens: &[Token], findings: &mut Vec<Finding>) {
    let decls = extract_fn_decls(tokens);

    for api_item in &row.safe_api {
        let (want_type, want_fn) = split_safe_api_item(api_item);
        let matched: Vec<&ParsedFnDecl> = match want_type {
            Some(ty) => decls
                .iter()
                .filter(|d| d.name == want_fn && d.impl_type.as_deref() == Some(ty))
                .collect(),
            None => {
                let free: Vec<&ParsedFnDecl> = decls
                    .iter()
                    .filter(|d| d.name == want_fn && d.impl_type.is_none())
                    .collect();
                if !free.is_empty() {
                    free
                } else {
                    decls.iter().filter(|d| d.name == want_fn).collect()
                }
            }
        };

        for decl in matched {
            if decl.is_unsafe {
                findings.push(Finding {
                    rule: RULE_CAPSULE_UNSAFE_IN_API,
                    subject: row.id.clone(),
                    path: row.path.clone(),
                    line: Some(decl.line),
                    message: format!(
                        "capsule `{}` safe_api item `{api_item}` is declared `unsafe fn`",
                        row.id
                    ),
                    repair: format!(
                        "wrap the unsafe operation internally so `{api_item}` in `{}` is a safe `fn`",
                        row.path
                    ),
                });
            }
            if decl.returns_raw_pointer {
                findings.push(Finding {
                    rule: RULE_CAPSULE_UNSAFE_IN_API,
                    subject: row.id.clone(),
                    path: row.path.clone(),
                    line: Some(decl.line),
                    message: format!(
                        "capsule `{}` safe_api item `{api_item}` returns a raw pointer",
                        row.id
                    ),
                    repair: format!(
                        "return an owned or borrowed safe wrapper type instead of `*const`/`*mut` from `{api_item}` in `{}`",
                        row.path
                    ),
                });
            }
        }
    }
}

fn split_safe_api_item(item: &str) -> (Option<&str>, &str) {
    let trimmed = item.trim();
    let parts: Vec<&str> = trimmed.split("::").collect();
    if parts.len() >= 2 {
        let fn_name = parts[parts.len() - 1].trim();
        let ty_name = parts[parts.len() - 2].trim();
        // If `ty_name` begins with an uppercase ASCII letter, treat as `Type::method`;
        // otherwise treat as `module::free_fn`.
        if ty_name.chars().next().is_some_and(|c| c.is_ascii_uppercase()) {
            (Some(ty_name), fn_name)
        } else {
            (None, fn_name)
        }
    } else {
        (None, trimmed)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum BraceScope {
    Mod,
    Impl(String),
    Extern,
    Other,
}

fn extract_fn_decls(tokens: &[Token]) -> Vec<ParsedFnDecl> {
    let mut decls = Vec::new();
    let mut scopes: Vec<BraceScope> = Vec::new();
    let mut pending_scope: Option<BraceScope> = None;

    let mut i = 0usize;
    while i < tokens.len() {
        let tok = &tokens[i];
        match &tok.kind {
            TokenKind::Punct('{') => {
                scopes.push(pending_scope.take().unwrap_or(BraceScope::Other));
                i += 1;
                continue;
            }
            TokenKind::Punct('}') => {
                scopes.pop();
                pending_scope = None;
                i += 1;
                continue;
            }
            TokenKind::Punct(';') => {
                pending_scope = None;
                i += 1;
                continue;
            }
            TokenKind::Ident(word) if word == "mod" => {
                if only_module_scopes(&scopes) {
                    // `mod name { ... }` vs `mod name;`
                    if i + 2 < tokens.len() && matches!(tokens[i + 1].kind, TokenKind::Ident(_)) {
                        if matches!(tokens[i + 2].kind, TokenKind::Punct('{')) {
                            pending_scope = Some(BraceScope::Mod);
                        }
                    }
                }
                i += 1;
                continue;
            }
            TokenKind::Ident(word) if word == "impl" => {
                if only_module_scopes(&scopes) {
                    if let Some((ty_name, brace_idx)) = parse_impl_target_type(tokens, i + 1) {
                        pending_scope = Some(BraceScope::Impl(ty_name));
                        i = brace_idx;
                        continue;
                    }
                }
                i += 1;
                continue;
            }
            TokenKind::Ident(word) if word == "extern" => {
                // Detect `extern { ... }` or `extern "C" { ... }` (FFI block, not `extern "C" fn`).
                let mut j = i + 1;
                if j < tokens.len() && matches!(tokens[j].kind, TokenKind::StringLit) {
                    j += 1;
                }
                if j < tokens.len() && matches!(tokens[j].kind, TokenKind::Punct('{')) {
                    pending_scope = Some(BraceScope::Extern);
                    i = j;
                    continue;
                }
                i += 1;
                continue;
            }
            TokenKind::Ident(word) if word == "fn" => {
                // Real function declaration has an identifier immediately after `fn` (not `fn(...)` type).
                if let Some(Token {
                    kind: TokenKind::Ident(fn_name),
                    line: fn_line,
                }) = tokens.get(i + 1)
                {
                    let (in_valid_scope, impl_type, in_extern) = classify_fn_scope(&scopes);
                    if in_valid_scope {
                        let is_unsafe = in_extern || has_preceding_unsafe_modifier(tokens, i);
                        let (returns_raw_pointer, next_idx) =
                            inspect_fn_return_type(tokens, i + 2);
                        decls.push(ParsedFnDecl {
                            impl_type,
                            name: fn_name.clone(),
                            line: *fn_line,
                            is_unsafe,
                            returns_raw_pointer,
                        });
                        i = next_idx;
                        continue;
                    }
                }
                i += 1;
                continue;
            }
            _ => {
                i += 1;
            }
        }
    }

    decls
}

fn only_module_scopes(scopes: &[BraceScope]) -> bool {
    scopes.iter().all(|s| *s == BraceScope::Mod)
}

fn classify_fn_scope(scopes: &[BraceScope]) -> (bool, Option<String>, bool) {
    match scopes.last() {
        None => (true, None, false),
        Some(BraceScope::Mod) => (only_module_scopes(scopes), None, false),
        Some(BraceScope::Impl(ty)) => {
            let outer_ok = only_module_scopes(&scopes[..scopes.len() - 1]);
            (outer_ok, Some(ty.clone()), false)
        }
        Some(BraceScope::Extern) => {
            let outer_ok = only_module_scopes(&scopes[..scopes.len() - 1]);
            (outer_ok, None, true)
        }
        Some(BraceScope::Other) => (false, None, false),
    }
}

fn parse_impl_target_type(tokens: &[Token], start: usize) -> Option<(String, usize)> {
    let mut i = start;
    // Skip optional generics `impl<...>`
    if matches!(tokens.get(i).map(|t| &t.kind), Some(TokenKind::Punct('<'))) {
        i = skip_angle_brackets(tokens, i);
    }

    // Collect tokens until `{` or `where` (stopping at `;` if invalid).
    let mut depth_angle = 0usize;
    let mut depth_paren = 0usize;
    let mut depth_bracket = 0usize;
    let mut for_idx: Option<usize> = None;
    let mut end_type_idx: Option<usize> = None;
    let mut brace_idx: Option<usize> = None;

    let mut j = i;
    while j < tokens.len() {
        match &tokens[j].kind {
            TokenKind::Punct('<') => depth_angle += 1,
            TokenKind::Punct('>') => depth_angle = depth_angle.saturating_sub(1),
            TokenKind::Punct('(') => depth_paren += 1,
            TokenKind::Punct(')') => depth_paren = depth_paren.saturating_sub(1),
            TokenKind::Punct('[') => depth_bracket += 1,
            TokenKind::Punct(']') => depth_bracket = depth_bracket.saturating_sub(1),
            TokenKind::Ident(w)
                if w == "for" && depth_angle == 0 && depth_paren == 0 && depth_bracket == 0 =>
            {
                for_idx = Some(j);
            }
            TokenKind::Ident(w)
                if w == "where" && depth_angle == 0 && depth_paren == 0 && depth_bracket == 0 =>
            {
                if end_type_idx.is_none() {
                    end_type_idx = Some(j);
                }
            }
            TokenKind::Punct('{')
                if depth_angle == 0 && depth_paren == 0 && depth_bracket == 0 =>
            {
                if end_type_idx.is_none() {
                    end_type_idx = Some(j);
                }
                brace_idx = Some(j);
                break;
            }
            TokenKind::Punct(';')
                if depth_angle == 0 && depth_paren == 0 && depth_bracket == 0 =>
            {
                return None;
            }
            _ => {}
        }
        j += 1;
    }

    let end_idx = end_type_idx?;
    let brace = brace_idx?;
    let type_slice = match for_idx {
        Some(f) => &tokens[f + 1..end_idx],
        None => &tokens[i..end_idx],
    };

    // Extract the primary type identifier outside generic `<...>`.
    let mut angle = 0usize;
    let mut last_ident: Option<String> = None;
    for tok in type_slice {
        match &tok.kind {
            TokenKind::Punct('<') => angle += 1,
            TokenKind::Punct('>') => angle = angle.saturating_sub(1),
            TokenKind::Ident(id) if angle == 0 => {
                last_ident = Some(id.clone());
            }
            _ => {}
        }
    }

    last_ident.map(|ty| (ty, brace))
}

fn has_preceding_unsafe_modifier(tokens: &[Token], fn_idx: usize) -> bool {
    let mut j = fn_idx;
    while j > 0 {
        j -= 1;
        match &tokens[j].kind {
            TokenKind::Ident(w) if w == "unsafe" => return true,
            TokenKind::Ident(w) if matches!(w.as_str(), "const" | "async" | "extern") => continue,
            TokenKind::StringLit => continue, // ABI string in `extern "C" fn`
            _ => break,
        }
    }
    false
}

fn inspect_fn_return_type(tokens: &[Token], start: usize) -> (bool, usize) {
    let mut i = start;
    // Skip optional generics `fn foo<...>`
    if matches!(tokens.get(i).map(|t| &t.kind), Some(TokenKind::Punct('<'))) {
        i = skip_angle_brackets(tokens, i);
    }
    // Skip parameter list `(...)`
    if matches!(tokens.get(i).map(|t| &t.kind), Some(TokenKind::Punct('('))) {
        i = skip_balanced(tokens, i, '(', ')');
    } else {
        return (false, i);
    }

    // Check for return arrow `->`
    let has_arrow = matches!(
        (tokens.get(i).map(|t| &t.kind), tokens.get(i + 1).map(|t| &t.kind)),
        (Some(TokenKind::Punct('-')), Some(TokenKind::Punct('>')))
    );
    if !has_arrow {
        return (false, i);
    }
    i += 2;

    // Scan return type tokens until `{`, `;`, or top-level `where`.
    let ret_start = i;
    let mut depth_angle = 0usize;
    let mut depth_paren = 0usize;
    let mut depth_bracket = 0usize;

    while i < tokens.len() {
        match &tokens[i].kind {
            TokenKind::Punct('<') => depth_angle += 1,
            TokenKind::Punct('>') => depth_angle = depth_angle.saturating_sub(1),
            TokenKind::Punct('(') => depth_paren += 1,
            TokenKind::Punct(')') => depth_paren = depth_paren.saturating_sub(1),
            TokenKind::Punct('[') => depth_bracket += 1,
            TokenKind::Punct(']') => depth_bracket = depth_bracket.saturating_sub(1),
            TokenKind::Punct('{') | TokenKind::Punct(';')
                if depth_angle == 0 && depth_paren == 0 && depth_bracket == 0 =>
            {
                break;
            }
            TokenKind::Ident(w)
                if w == "where" && depth_angle == 0 && depth_paren == 0 && depth_bracket == 0 =>
            {
                break;
            }
            _ => {}
        }
        i += 1;
    }

    let ret_slice = &tokens[ret_start..i];
    let returns_raw_pointer = ret_slice.windows(2).any(|pair| {
        matches!(pair[0].kind, TokenKind::Punct('*'))
            && matches!(&pair[1].kind, TokenKind::Ident(w) if w == "const" || w == "mut")
    });

    (returns_raw_pointer, i)
}

fn skip_angle_brackets(tokens: &[Token], start: usize) -> usize {
    let mut depth = 0usize;
    let mut i = start;
    while i < tokens.len() {
        match tokens[i].kind {
            TokenKind::Punct('<') => depth += 1,
            TokenKind::Punct('>') => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    return i + 1;
                }
            }
            _ => {}
        }
        i += 1;
    }
    i
}

fn skip_balanced(tokens: &[Token], start: usize, open: char, close: char) -> usize {
    let mut depth = 0usize;
    let mut i = start;
    while i < tokens.len() {
        match tokens[i].kind {
            TokenKind::Punct(c) if c == open => depth += 1,
            TokenKind::Punct(c) if c == close => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    return i + 1;
                }
            }
            _ => {}
        }
        i += 1;
    }
    i
}

// ============================================================================
// Rust tokenizer & lint-attribute parser (skips comments, strings, char lits)
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
enum TokenKind {
    Ident(String),
    RawIdent,
    StringLit,
    Punct(char),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Token {
    kind: TokenKind,
    line: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct LintDirective {
    is_inner: bool,
    brace_depth: usize,
    level: String,
    lint: String,
    line: u32,
}

fn count_lines(text: &str) -> usize {
    text.lines().count()
}

fn find_unsafe_keyword_lines(tokens: &[Token]) -> Vec<u32> {
    tokens
        .iter()
        .filter_map(|t| match &t.kind {
            TokenKind::Ident(w) if w == "unsafe" => Some(t.line),
            _ => None,
        })
        .collect()
}

fn tokenize_rust(src: &str) -> Vec<Token> {
    let bytes = src.as_bytes();
    let len = bytes.len();
    let mut i = 0usize;
    let mut line: u32 = 1;
    let mut tokens = Vec::new();

    while i < len {
        let b = bytes[i];

        // Whitespace.
        if b == b'\n' {
            line = line.saturating_add(1);
            i += 1;
            continue;
        }
        if b.is_ascii_whitespace() {
            i += 1;
            continue;
        }

        // Line comment `// ...`
        if b == b'/' && i + 1 < len && bytes[i + 1] == b'/' {
            i += 2;
            while i < len && bytes[i] != b'\n' {
                i += 1;
            }
            continue;
        }

        // Nested block comment `/* ... /* ... */ ... */`
        if b == b'/' && i + 1 < len && bytes[i + 1] == b'*' {
            i += 2;
            let mut depth = 1usize;
            while i < len && depth > 0 {
                if bytes[i] == b'\n' {
                    line = line.saturating_add(1);
                    i += 1;
                } else if bytes[i] == b'/' && i + 1 < len && bytes[i + 1] == b'*' {
                    depth += 1;
                    i += 2;
                } else if bytes[i] == b'*' && i + 1 < len && bytes[i + 1] == b'/' {
                    depth -= 1;
                    i += 2;
                } else {
                    i += 1;
                }
            }
            continue;
        }

        // Normal string literal `"..."`
        if b == b'"' {
            let start_line = line;
            i = skip_quoted_string(bytes, i + 1, &mut line);
            tokens.push(Token {
                kind: TokenKind::StringLit,
                line: start_line,
            });
            continue;
        }

        // Raw string literals `r"..."`, `r#"..."#`, `br#"..."#`, `cr#"..."#`,
        // byte/C string literals `b"..."`, `c"..."`, byte char `b'...'`, raw ident `r#ident`.
        if let Some((kind, next_i, tok_line)) = try_prefixed_literal_or_raw_ident(src, bytes, i, &mut line) {
            if let Some(k) = kind {
                tokens.push(Token {
                    kind: k,
                    line: tok_line,
                });
            }
            i = next_i;
            continue;
        }

        // Char literal vs lifetime (`'a` / `'static` / `'_'` vs `'a'` / `'\''` / `'\u{...}'`).
        if b == b'\'' {
            if let Some(next_i) = try_skip_char_literal(src, bytes, i) {
                i = next_i;
                continue;
            }
            tokens.push(Token {
                kind: TokenKind::Punct('\''),
                line,
            });
            i += 1;
            continue;
        }

        // Identifier or keyword (ASCII or Unicode XID).
        if is_ident_start_at(src, i) {
            let start = i;
            let start_line = line;
            let ch = src[i..].chars().next().unwrap_or('\0');
            i += ch.len_utf8();
            while i < len {
                let Some(next_ch) = src[i..].chars().next() else {
                    break;
                };
                if is_ident_continue(next_ch) {
                    i += next_ch.len_utf8();
                } else {
                    break;
                }
            }
            tokens.push(Token {
                kind: TokenKind::Ident(src[start..i].to_owned()),
                line: start_line,
            });
            continue;
        }

        // Punctuation or other single-scalar token.
        let ch = src[i..].chars().next().unwrap_or('\0');
        tokens.push(Token {
            kind: TokenKind::Punct(ch),
            line,
        });
        i += ch.len_utf8();
    }

    tokens
}

fn try_prefixed_literal_or_raw_ident(
    src: &str,
    bytes: &[u8],
    i: usize,
    line: &mut u32,
) -> Option<(Option<TokenKind>, usize, u32)> {
    let len = bytes.len();
    let b = bytes[i];
    let start_line = *line;

    // `br"..."`, `br#"..."#`, `cr"..."`, `cr#"..."#`
    if (b == b'b' || b == b'c') && i + 1 < len && bytes[i + 1] == b'r' {
        if let Some(next_i) = try_skip_raw_string(bytes, i + 2, line) {
            return Some((Some(TokenKind::StringLit), next_i, start_line));
        }
    }

    // `b"..."`, `c"..."`
    if (b == b'b' || b == b'c') && i + 1 < len && bytes[i + 1] == b'"' {
        let next_i = skip_quoted_string(bytes, i + 2, line);
        return Some((Some(TokenKind::StringLit), next_i, start_line));
    }

    // `b'x'` byte char literal
    if b == b'b' && i + 1 < len && bytes[i + 1] == b'\'' {
        if let Some(next_i) = try_skip_char_literal(src, bytes, i + 1) {
            return Some((None, next_i, start_line));
        }
    }

    // `r"..."`, `r#"..."#`, or raw identifier `r#ident`
    if b == b'r' {
        if let Some(next_i) = try_skip_raw_string(bytes, i + 1, line) {
            return Some((Some(TokenKind::StringLit), next_i, start_line));
        }
        if i + 2 < len && bytes[i + 1] == b'#' && is_ident_start_at(src, i + 2) {
            let mut j = i + 2;
            while j < len {
                let Some(ch) = src[j..].chars().next() else {
                    break;
                };
                if is_ident_continue(ch) {
                    j += ch.len_utf8();
                } else {
                    break;
                }
            }
            return Some((Some(TokenKind::RawIdent), j, start_line));
        }
    }

    None
}

fn try_skip_raw_string(bytes: &[u8], start: usize, line: &mut u32) -> Option<usize> {
    let len = bytes.len();
    let mut j = start;
    let mut hashes = 0usize;
    while j < len && bytes[j] == b'#' {
        hashes += 1;
        j += 1;
    }
    if j >= len || bytes[j] != b'"' {
        return None;
    }
    j += 1;
    while j < len {
        if bytes[j] == b'\n' {
            *line = line.saturating_add(1);
            j += 1;
        } else if bytes[j] == b'"' {
            let mut k = j + 1;
            let mut matched = 0usize;
            while matched < hashes && k < len && bytes[k] == b'#' {
                matched += 1;
                k += 1;
            }
            if matched == hashes {
                return Some(k);
            }
            j += 1;
        } else {
            j += 1;
        }
    }
    Some(len)
}

fn skip_quoted_string(bytes: &[u8], start: usize, line: &mut u32) -> usize {
    let len = bytes.len();
    let mut i = start;
    while i < len {
        match bytes[i] {
            b'\\' => {
                if i + 1 < len {
                    if bytes[i + 1] == b'\n' {
                        *line = line.saturating_add(1);
                    }
                    i += 2;
                } else {
                    return len;
                }
            }
            b'"' => return i + 1,
            b'\n' => {
                *line = line.saturating_add(1);
                i += 1;
            }
            _ => i += 1,
        }
    }
    len
}

fn try_skip_char_literal(src: &str, bytes: &[u8], quote_idx: usize) -> Option<usize> {
    let len = bytes.len();
    let next = quote_idx + 1;
    if next >= len {
        return None;
    }
    if bytes[next] == b'\\' {
        // Escaped char literal: `'\''`, `'\\'`, `'\n'`, `'\x7f'`, `'\u{1234}'`
        let mut j = next + 1;
        if j >= len {
            return None;
        }
        j += 1;
        while j < len && bytes[j] != b'\'' && bytes[j] != b'\n' && bytes[j] != b'\r' {
            j += 1;
        }
        if j < len && bytes[j] == b'\'' {
            return Some(j + 1);
        }
        return None;
    }
    let ch = src[next..].chars().next()?;
    if ch == '\'' || ch == '\n' || ch == '\r' {
        return None;
    }
    let after = next + ch.len_utf8();
    if after < len && bytes[after] == b'\'' {
        Some(after + 1)
    } else {
        None
    }
}

fn is_ident_start_at(src: &str, idx: usize) -> bool {
    src[idx..]
        .chars()
        .next()
        .is_some_and(|c| c == '_' || c.is_alphabetic())
}

fn is_ident_continue(ch: char) -> bool {
    ch == '_' || ch.is_alphanumeric()
}

fn parse_lint_directives(tokens: &[Token]) -> Vec<LintDirective> {
    let mut out = Vec::new();
    let mut brace_depth = 0usize;
    let mut i = 0usize;

    while i < tokens.len() {
        match tokens[i].kind {
            TokenKind::Punct('{') => {
                brace_depth += 1;
                i += 1;
            }
            TokenKind::Punct('}') => {
                brace_depth = brace_depth.saturating_sub(1);
                i += 1;
            }
            TokenKind::Punct('#') => {
                let attr_line = tokens[i].line;
                let (is_inner, bracket_idx) = if matches!(
                    (
                        tokens.get(i + 1).map(|t| &t.kind),
                        tokens.get(i + 2).map(|t| &t.kind)
                    ),
                    (Some(TokenKind::Punct('!')), Some(TokenKind::Punct('[')))
                ) {
                    (true, i + 2)
                } else if matches!(tokens.get(i + 1).map(|t| &t.kind), Some(TokenKind::Punct('['))) {
                    (false, i + 1)
                } else {
                    i += 1;
                    continue;
                };

                let end_idx = skip_balanced(tokens, bracket_idx, '[', ']');
                if end_idx > bracket_idx + 1 {
                    let inner_slice = &tokens[bracket_idx + 1..end_idx - 1];
                    extract_lints_from_meta(inner_slice, is_inner, brace_depth, attr_line, &mut out);
                }
                i = end_idx;
            }
            _ => i += 1,
        }
    }

    out
}

fn extract_lints_from_meta(
    meta: &[Token],
    is_inner: bool,
    brace_depth: usize,
    attr_line: u32,
    out: &mut Vec<LintDirective>,
) {
    let Some(Token {
        kind: TokenKind::Ident(head),
        ..
    }) = meta.first()
    else {
        return;
    };

    if !matches!(meta.get(1).map(|t| &t.kind), Some(TokenKind::Punct('('))) {
        return;
    }
    let close = skip_balanced(meta, 1, '(', ')');
    if close <= 2 {
        return;
    }
    let args = &meta[2..close - 1];

    if head == "cfg_attr" {
        let items = split_top_level_commas(args);
        for sub_meta in items.into_iter().skip(1) {
            extract_lints_from_meta(sub_meta, is_inner, brace_depth, attr_line, out);
        }
        return;
    }

    if matches!(head.as_str(), "allow" | "warn" | "expect" | "deny" | "forbid") {
        for item in split_top_level_commas(args) {
            if item.is_empty() {
                continue;
            }
            // Skip key-value metadata such as `reason = "..."`.
            if item.iter().any(|t| matches!(t.kind, TokenKind::Punct('='))) {
                continue;
            }
            if let Some(lint_path) = render_lint_path(item) {
                out.push(LintDirective {
                    is_inner,
                    brace_depth,
                    level: head.clone(),
                    lint: lint_path,
                    line: item.first().map_or(attr_line, |t| t.line),
                });
            }
        }
    }
}

fn split_top_level_commas(tokens: &[Token]) -> Vec<&[Token]> {
    let mut parts = Vec::new();
    let mut start = 0usize;
    let mut depth_paren = 0usize;
    let mut depth_bracket = 0usize;
    let mut depth_brace = 0usize;

    for (idx, tok) in tokens.iter().enumerate() {
        match tok.kind {
            TokenKind::Punct('(') => depth_paren += 1,
            TokenKind::Punct(')') => depth_paren = depth_paren.saturating_sub(1),
            TokenKind::Punct('[') => depth_bracket += 1,
            TokenKind::Punct(']') => depth_bracket = depth_bracket.saturating_sub(1),
            TokenKind::Punct('{') => depth_brace += 1,
            TokenKind::Punct('}') => depth_brace = depth_brace.saturating_sub(1),
            TokenKind::Punct(',')
                if depth_paren == 0 && depth_bracket == 0 && depth_brace == 0 =>
            {
                parts.push(&tokens[start..idx]);
                start = idx + 1;
            }
            _ => {}
        }
    }
    if start < tokens.len() {
        parts.push(&tokens[start..]);
    }
    parts
}

fn render_lint_path(tokens: &[Token]) -> Option<String> {
    let mut segments: Vec<&str> = Vec::new();
    let mut i = 0usize;
    while i < tokens.len() {
        match &tokens[i].kind {
            TokenKind::Ident(seg) => {
                segments.push(seg.as_str());
                i += 1;
                if i < tokens.len() {
                    if matches!(
                        (
                            tokens.get(i).map(|t| &t.kind),
                            tokens.get(i + 1).map(|t| &t.kind)
                        ),
                        (Some(TokenKind::Punct(':')), Some(TokenKind::Punct(':')))
                    ) {
                        i += 2;
                    } else {
                        return None;
                    }
                }
            }
            _ => return None,
        }
    }
    if segments.is_empty() {
        None
    } else {
        Some(segments.join("::"))
    }
}

// ============================================================================
// Lightweight Cargo.toml reader (handles full Cargo TOML incl. dotted keys)
// ============================================================================

#[derive(Debug, Clone)]
struct ManifestLintOverride {
    lint: String,
    level: String,
    line: u32,
}

#[derive(Debug, Clone, Default)]
struct ParsedCargoManifest {
    package_name: Option<String>,
    workspace_members: Vec<String>,
    workspace_exclude: Vec<String>,
    workspace_rust_unsafe_code: Option<(String, u32)>,
    lints_workspace: Option<(bool, u32)>,
    crate_lint_overrides: Vec<ManifestLintOverride>,
    lib_path: Option<String>,
    bin_paths: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MlState {
    None,
    Basic,
    Literal,
}

fn parse_cargo_manifest(text: &str) -> ParsedCargoManifest {
    let mut out = ParsedCargoManifest::default();
    let mut section = String::new();
    let mut ml_state = MlState::None;

    let raw_lines: Vec<&str> = text.lines().collect();
    let mut idx = 0usize;

    while idx < raw_lines.len() {
        let line_no = u32::try_from(idx + 1).unwrap_or(u32::MAX);
        let Some(stripped) = strip_toml_comment(raw_lines[idx], &mut ml_state) else {
            idx += 1;
            continue;
        };
        let trimmed = stripped.trim();
        if trimmed.is_empty() {
            idx += 1;
            continue;
        }
        if trimmed.starts_with('[') {
            if let Some(header) = parse_table_header(trimmed) {
                section = header;
            }
            idx += 1;
            continue;
        }

        let Some((raw_key, first_val)) = split_kv(trimmed) else {
            idx += 1;
            continue;
        };

        // Accumulate multi-line array or inline-table values.
        let mut full_val = first_val.to_owned();
        while !is_balanced_toml_value(&full_val) && idx + 1 < raw_lines.len() {
            idx += 1;
            if let Some(next_stripped) = strip_toml_comment(raw_lines[idx], &mut ml_state) {
                full_val.push(' ');
                full_val.push_str(next_stripped.trim());
            }
        }

        let key = normalize_dotted_key(raw_key);
        let full_key = if section.is_empty() {
            key
        } else {
            format!("{section}.{key}")
        };
        let val = full_val.trim();

        match full_key.as_str() {
            "package.name" => {
                if let Some(s) = parse_toml_string(val) {
                    out.package_name = Some(s);
                }
            }
            "workspace.members" => {
                out.workspace_members = parse_toml_string_array(val);
            }
            "workspace.exclude" => {
                out.workspace_exclude = parse_toml_string_array(val);
            }
            "workspace.lints.rust.unsafe_code" => {
                if let Some(level) = parse_lint_level_value(val) {
                    out.workspace_rust_unsafe_code = Some((level, line_no));
                }
            }
            "workspace.lints.rust" => {
                if let Some(inner) = parse_inline_table_field(val, "unsafe_code")
                    && let Some(level) = parse_lint_level_value(&inner)
                {
                    out.workspace_rust_unsafe_code = Some((level, line_no));
                }
            }
            "lints.workspace" => {
                if let Some(b) = parse_toml_bool(val) {
                    out.lints_workspace = Some((b, line_no));
                }
            }
            "lints" => {
                if let Some(inner) = parse_inline_table_field(val, "workspace")
                    && let Some(b) = parse_toml_bool(&inner)
                {
                    out.lints_workspace = Some((b, line_no));
                }
            }
            "lib.path" => {
                if let Some(s) = parse_toml_string(val) {
                    out.lib_path = Some(s);
                }
            }
            "bin.path" => {
                if let Some(s) = parse_toml_string(val) {
                    out.bin_paths.push(s);
                }
            }
            _ => {
                if let Some(lint_name) = full_key.strip_prefix("lints.rust.") {
                    if let Some(level) = parse_lint_level_value(val) {
                        out.crate_lint_overrides.push(ManifestLintOverride {
                            lint: lint_name.to_owned(),
                            level,
                            line: line_no,
                        });
                    }
                } else if let Some(lint_name) = full_key.strip_prefix("lints.clippy.") {
                    if let Some(level) = parse_lint_level_value(val) {
                        out.crate_lint_overrides.push(ManifestLintOverride {
                            lint: format!("clippy::{lint_name}"),
                            level,
                            line: line_no,
                        });
                    }
                }
            }
        }

        idx += 1;
    }

    out
}

fn strip_toml_comment<'a>(line: &'a str, state: &mut MlState) -> Option<&'a str> {
    let bytes = line.as_bytes();
    let mut i = 0usize;
    if *state != MlState::None {
        advance_ml(bytes, &mut i, state);
        return None;
    }
    let (mut in_double, mut in_single, mut escaped) = (false, false, false);
    while i < bytes.len() {
        let b = bytes[i];
        if in_double {
            if escaped {
                escaped = false;
            } else if b == b'\\' {
                escaped = true;
            } else if b == b'"' {
                in_double = false;
            }
            i += 1;
        } else if in_single {
            if b == b'\'' {
                in_single = false;
            }
            i += 1;
        } else if bytes[i..].starts_with(b"\"\"\"") {
            i += 3;
            *state = MlState::Basic;
            advance_ml(bytes, &mut i, state);
        } else if bytes[i..].starts_with(b"'''") {
            i += 3;
            *state = MlState::Literal;
            advance_ml(bytes, &mut i, state);
        } else if b == b'"' {
            in_double = true;
            i += 1;
        } else if b == b'\'' {
            in_single = true;
            i += 1;
        } else if b == b'#' {
            return Some(&line[..i]);
        } else {
            i += 1;
        }
    }
    Some(line)
}

fn advance_ml(bytes: &[u8], i: &mut usize, state: &mut MlState) {
    let delim: &[u8] = match *state {
        MlState::None => return,
        MlState::Basic => b"\"\"\"",
        MlState::Literal => b"'''",
    };
    while *i < bytes.len() {
        if *state == MlState::Basic && bytes[*i] == b'\\' && *i + 1 < bytes.len() {
            *i += 2;
            continue;
        }
        if bytes[*i..].starts_with(delim) {
            *i += 3;
            *state = MlState::None;
            return;
        }
        *i += 1;
    }
}

fn parse_table_header(line: &str) -> Option<String> {
    let inner = if let Some(s) = line.strip_prefix("[[").and_then(|t| t.strip_suffix("]]")) {
        s
    } else if let Some(s) = line.strip_prefix('[').and_then(|t| t.strip_suffix(']')) {
        s
    } else {
        return None;
    };
    Some(normalize_dotted_key(inner.trim()))
}

fn split_kv(line: &str) -> Option<(&str, &str)> {
    let (mut in_double, mut in_single, mut escaped) = (false, false, false);
    for (idx, ch) in line.char_indices() {
        if in_double {
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == '"' {
                in_double = false;
            }
        } else if in_single {
            if ch == '\'' {
                in_single = false;
            }
        } else if ch == '"' {
            in_double = true;
        } else if ch == '\'' {
            in_single = true;
        } else if ch == '=' {
            return Some((line[..idx].trim(), line[idx + 1..].trim()));
        }
    }
    None
}

fn is_balanced_toml_value(val: &str) -> bool {
    let (mut in_double, mut in_single, mut escaped) = (false, false, false);
    let mut brackets = 0i32;
    let mut braces = 0i32;
    for ch in val.chars() {
        if in_double {
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == '"' {
                in_double = false;
            }
        } else if in_single {
            if ch == '\'' {
                in_single = false;
            }
        } else {
            match ch {
                '"' => in_double = true,
                '\'' => in_single = true,
                '[' => brackets += 1,
                ']' => brackets -= 1,
                '{' => braces += 1,
                '}' => braces -= 1,
                _ => {}
            }
        }
    }
    brackets <= 0 && braces <= 0
}

fn normalize_dotted_key(raw: &str) -> String {
    let mut out = String::new();
    for (idx, seg) in split_unquoted(raw, '.').into_iter().enumerate() {
        if idx > 0 {
            out.push('.');
        }
        let s = seg.trim();
        if let Some(unquoted) = parse_toml_string(s) {
            out.push_str(&unquoted);
        } else {
            out.push_str(s);
        }
    }
    out
}

fn split_unquoted(raw: &str, delim: char) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut start = 0usize;
    let (mut in_double, mut in_single, mut escaped) = (false, false, false);
    let mut depth_bracket = 0usize;
    let mut depth_brace = 0usize;
    for (idx, ch) in raw.char_indices() {
        if in_double {
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == '"' {
                in_double = false;
            }
        } else if in_single {
            if ch == '\'' {
                in_single = false;
            }
        } else {
            match ch {
                '"' => in_double = true,
                '\'' => in_single = true,
                '[' => depth_bracket += 1,
                ']' => depth_bracket = depth_bracket.saturating_sub(1),
                '{' => depth_brace += 1,
                '}' => depth_brace = depth_brace.saturating_sub(1),
                c if c == delim && depth_bracket == 0 && depth_brace == 0 => {
                    parts.push(&raw[start..idx]);
                    start = idx + ch.len_utf8();
                }
                _ => {}
            }
        }
    }
    parts.push(&raw[start..]);
    parts
}

fn parse_toml_string(raw: &str) -> Option<String> {
    let s = raw.trim();
    if let Some(inner) = s.strip_prefix('"').and_then(|t| t.strip_suffix('"')) {
        let mut out = String::with_capacity(inner.len());
        let mut chars = inner.chars();
        while let Some(c) = chars.next() {
            if c == '\\' {
                match chars.next()? {
                    'n' => out.push('\n'),
                    'r' => out.push('\r'),
                    't' => out.push('\t'),
                    '\\' => out.push('\\'),
                    '"' => out.push('"'),
                    other => out.push(other),
                }
            } else {
                out.push(c);
            }
        }
        Some(out)
    } else if let Some(inner) = s.strip_prefix('\'').and_then(|t| t.strip_suffix('\'')) {
        Some(inner.to_owned())
    } else {
        None
    }
}

fn parse_toml_bool(raw: &str) -> Option<bool> {
    match raw.trim() {
        "true" => Some(true),
        "false" => Some(false),
        _ => None,
    }
}

fn parse_toml_string_array(raw: &str) -> Vec<String> {
    let s = raw.trim();
    let Some(inner) = s.strip_prefix('[').and_then(|t| t.strip_suffix(']')) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for item in split_unquoted(inner, ',') {
        let t = item.trim();
        if t.is_empty() {
            continue;
        }
        if let Some(val) = parse_toml_string(t) {
            out.push(val);
        }
    }
    out
}

fn parse_inline_table_field(raw: &str, field: &str) -> Option<String> {
    let s = raw.trim();
    let inner = s.strip_prefix('{')?.strip_suffix('}')?;
    for entry in split_unquoted(inner, ',') {
        let Some((k, v)) = split_kv(entry.trim()) else {
            continue;
        };
        if normalize_dotted_key(k) == field {
            return Some(v.trim().to_owned());
        }
    }
    None
}

fn parse_lint_level_value(raw: &str) -> Option<String> {
    if let Some(s) = parse_toml_string(raw) {
        return Some(s);
    }
    if let Some(level_raw) = parse_inline_table_field(raw, "level") {
        return parse_toml_string(&level_raw);
    }
    None
}
