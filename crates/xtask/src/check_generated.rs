//! `xtask check-generated`: regenerates registry-derived artifacts and checks
//! committed outputs byte-for-byte (or by manifest BLAKE3 digest for derived
//! kernels) under rule L-8 and INV-SPARK-08 (bead `N34p4-03`, GATE-WS-01 plant 5).
//!
//! Pure in check mode (writes nothing); `--write` atomically rewrites only
//! `Regenerate`-mode outputs in the outputs column and never touches `Digest`
//! files or files outside the outputs column. Determinism class D0, zero
//! `unsafe`, prints nothing (rule L-7), never deletes files (COMMON.md 9b).

use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};

use crate::cli::{CheckContext, CheckError, CheckOutcome};
use crate::crate_map::Finding;
use crate::generate::{
    BUILTIN_GENERATORS, ExpectedMarker, GeneratorMode, GeneratorOutput, GeneratorRow, OutputKind,
    RULE_DIFFERS, RULE_DIGEST, RULE_MARKER_MISSING, RULE_MISSING, RULE_ORPHAN, RULE_STATUS_INVALID,
    admission_doc, expand_globs,
};
use crate::report::FactValue;
use crate::status_table;

/// Schema identifier for the `check-generated` facts block (`N34p4-03`).
pub const CHECK_GENERATED_FACTS_SCHEMA: &str = "frob.check-generated/1";

/// Complete closed set of finding rule identifiers owned exclusively by `check-generated`.
pub const OWNED_RULES: &[&str] = &[
    RULE_DIFFERS,
    RULE_MISSING,
    RULE_ORPHAN,
    RULE_DIGEST,
    RULE_MARKER_MISSING,
    RULE_STATUS_INVALID,
];

/// CLI handler registered in `src/main.rs`'s static `COMMANDS` array.
pub fn run_command(ctx: &CheckContext) -> Result<CheckOutcome, CheckError> {
    let (write_mode, rows) = parse_args(&ctx.extra_args).map_err(CheckError::Usage)?;
    run_check(&ctx.repo_root, rows, write_mode)
}

/// Parse `--write` and optional `--generator <name>` for an isolated check.
fn parse_args(extra: &[String]) -> Result<(bool, &'static [GeneratorRow]), String> {
    let mut write_mode = false;
    let mut rows = BUILTIN_GENERATORS;
    let mut args = extra.iter();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--write" => write_mode = true,
            "--generator" => {
                let name = args.next().ok_or("--generator requires a generator name")?;
                let selected = BUILTIN_GENERATORS
                    .iter()
                    .find(|row| row.name == name)
                    .ok_or_else(|| format!("unknown check-generated generator `{name}`"))?;
                rows = std::slice::from_ref(selected);
            }
            _ => return Err(format!("unknown check-generated flag `{arg}`")),
        }
    }
    Ok((write_mode, rows))
}

/// Execute `check-generated` over `rows` on `repo_root`.
///
/// Parameterized by `rows` so both the six built-in rows and synthetic
/// multi-glob test rows execute through the exact same code path.
pub fn run_check(
    repo_root: &Path,
    rows: &[GeneratorRow],
    write_mode: bool,
) -> Result<CheckOutcome, CheckError> {
    let mut findings: Vec<Finding> = Vec::new();
    let mut expected_outputs: BTreeSet<String> = BTreeSet::new();
    let mut written_files: Vec<String> = Vec::new();
    let mut row_facts: Vec<FactValue> = Vec::with_capacity(rows.len());
    let mut checked_files: usize = 0;
    let mut digest_files_checked: usize = 0;

    for row in rows {
        let inputs = expand_globs(repo_root, row.input_globs);
        let mut produced_paths: Vec<String> = Vec::new();
        let mut rf = BTreeMap::new();

        match (row.run)(repo_root, &inputs) {
            Ok(
                output @ (GeneratorOutput::Regenerated(_)
                | GeneratorOutput::RegeneratedWithFacts(..)),
            ) => {
                let files = match output {
                    GeneratorOutput::Regenerated(files) => files,
                    GeneratorOutput::RegeneratedWithFacts(files, facts) => {
                        rf = facts;
                        files
                    }
                    GeneratorOutput::Digests(_) => unreachable!(),
                };
                for gf in files {
                    expected_outputs.insert(gf.rel_path.clone());
                    produced_paths.push(gf.rel_path.clone());
                    checked_files += 1;

                    let abs = repo_root.join(&gf.rel_path);
                    match gf.kind {
                        OutputKind::WholeFile => {
                            if write_mode {
                                atomic_write_bytes(&abs, &gf.bytes).map_err(|e| {
                                    CheckError::Fault(format!(
                                        "failed writing `{}`: {e}",
                                        gf.rel_path
                                    ))
                                })?;
                                written_files.push(gf.rel_path.clone());
                            } else if !abs.is_file() {
                                findings.push(Finding {
                                    rule: RULE_MISSING,
                                    subject: row.name.to_owned(),
                                    path: gf.rel_path.clone(),
                                    line: None,
                                    message: format!(
                                        "generated file `{}` is missing (generator `{}`)",
                                        gf.rel_path, row.name
                                    ),
                                    repair: "run `cargo xtask check-generated --write` to regenerate outputs"
                                        .to_owned(),
                                });
                            } else {
                                let actual = fs::read(&abs).map_err(|e| {
                                    CheckError::Fault(format!(
                                        "failed reading `{}`: {e}",
                                        gf.rel_path
                                    ))
                                })?;
                                if !has_expected_marker(&actual, &gf.marker) {
                                    findings.push(Finding {
                                        rule: RULE_MARKER_MISSING,
                                        subject: row.name.to_owned(),
                                        path: gf.rel_path.clone(),
                                        line: Some(1),
                                        message: format!(
                                            "generated file `{}` is missing its generator marker (generator `{}`)",
                                            gf.rel_path, row.name
                                        ),
                                        repair: "run `cargo xtask check-generated --write` to restore the generated header and contents".to_owned(),
                                    });
                                }
                                if actual != gf.bytes {
                                    let line_no = first_diff_line_bytes(&actual, &gf.bytes);
                                    findings.push(Finding {
                                        rule: RULE_DIFFERS,
                                        subject: row.name.to_owned(),
                                        path: gf.rel_path.clone(),
                                        line: Some(line_no),
                                        message: format!(
                                            "generated file `{}` differs from generator `{}` at line {line_no}",
                                            gf.rel_path, row.name
                                        ),
                                        repair: "do not edit generated files by hand; run `cargo xtask check-generated --write`".to_owned(),
                                    });
                                }
                            }
                        }
                        OutputKind::ReadmeStatusBlock => {
                            let expected_block = String::from_utf8(gf.bytes).map_err(|e| {
                                CheckError::Fault(format!("non-UTF-8 status block: {e}"))
                            })?;
                            if !abs.is_file() {
                                findings.push(Finding {
                                    rule: RULE_MISSING,
                                    subject: row.name.to_owned(),
                                    path: gf.rel_path.clone(),
                                    line: None,
                                    message: format!(
                                        "target file `{}` is missing for generator `{}`",
                                        gf.rel_path, row.name
                                    ),
                                    repair: "restore README.md with `<!-- status:begin -->` and `<!-- status:end -->` markers".to_owned(),
                                });
                                continue;
                            }
                            let readme_text = fs::read_to_string(&abs).map_err(|e| {
                                CheckError::Fault(format!("failed reading `{}`: {e}", gf.rel_path))
                            })?;
                            let Ok((before, actual_block, after, first_inner_line)) =
                                status_table::split_readme_status_block(&readme_text)
                            else {
                                findings.push(Finding {
                                    rule: RULE_MARKER_MISSING,
                                    subject: row.name.to_owned(),
                                    path: gf.rel_path.clone(),
                                    line: Some(1),
                                    message: format!(
                                        "`{}` is missing valid `<!-- status:begin -->` / `<!-- status:end -->` markers",
                                        gf.rel_path
                                    ),
                                    repair: "restore the `<!-- status:begin -->` and `<!-- status:end -->` markers in README.md".to_owned(),
                                });
                                continue;
                            };
                            if write_mode {
                                let mut updated = String::with_capacity(
                                    before.len() + expected_block.len() + after.len(),
                                );
                                updated.push_str(before);
                                updated.push_str(&expected_block);
                                updated.push_str(after);
                                atomic_write_bytes(&abs, updated.as_bytes()).map_err(|e| {
                                    CheckError::Fault(format!(
                                        "failed writing `{}`: {e}",
                                        gf.rel_path
                                    ))
                                })?;
                                written_files.push(gf.rel_path.clone());
                            } else if actual_block != expected_block {
                                let offset = first_diff_offset_str(actual_block, &expected_block);
                                let line_no =
                                    u32::try_from(first_inner_line + offset).unwrap_or(u32::MAX);
                                findings.push(Finding {
                                    rule: RULE_DIFFERS,
                                    subject: row.name.to_owned(),
                                    path: gf.rel_path.clone(),
                                    line: Some(line_no),
                                    message: format!(
                                        "generated status block in `{}` differs from registries/gates.toml at line {line_no}",
                                        gf.rel_path
                                    ),
                                    repair: "do not edit the README status block by hand; run `cargo xtask check-generated --write`".to_owned(),
                                });
                            }
                        }
                    }
                }
            }
            Ok(GeneratorOutput::Digests(expectations)) => {
                // Digest mode NEVER rewrites files, even under `--write`.
                for exp in expectations {
                    expected_outputs.insert(exp.rel_path.clone());
                    produced_paths.push(exp.rel_path.clone());
                    digest_files_checked += 1;

                    let abs = repo_root.join(&exp.rel_path);
                    if !abs.is_file() {
                        findings.push(Finding {
                            rule: RULE_MISSING,
                            subject: exp.subject.clone(),
                            path: exp.rel_path.clone(),
                            line: None,
                            message: format!(
                                "derived kernel file `{}` declared in registries/kernels.toml is missing",
                                exp.rel_path
                            ),
                            repair: "re-run the symbolic derivation lane (`derive/frob-derive`) to regenerate the kernel".to_owned(),
                        });
                        continue;
                    }
                    let actual = fs::read(&abs).map_err(|e| {
                        CheckError::Fault(format!("failed reading `{}`: {e}", exp.rel_path))
                    })?;
                    if !has_kernel_generated_marker(&actual) {
                        findings.push(Finding {
                            rule: RULE_MARKER_MISSING,
                            subject: exp.subject.clone(),
                            path: exp.rel_path.clone(),
                            line: Some(1),
                            message: format!(
                                "generated kernel file `{}` lacks a `@generated` first-line marker",
                                exp.rel_path
                            ),
                            repair: "regenerate the kernel from derive/frob-derive so line 1 carries `@generated`".to_owned(),
                        });
                    }
                    let actual_hex = fs_blake3::hash_bytes(&actual).to_hex();
                    if actual_hex != exp.expected_blake3 {
                        findings.push(Finding {
                            rule: RULE_DIGEST,
                            subject: exp.subject.clone(),
                            path: exp.rel_path.clone(),
                            line: None,
                            message: format!(
                                "kernel `{}` (`{}`) BLAKE3 `b3:{actual_hex}` differs from manifest digest `b3:{}`",
                                exp.subject, exp.rel_path, exp.expected_blake3
                            ),
                            repair: "never hand-edit generated kernel arithmetic; re-derive through derive/frob-derive (INV-SPARK-08)".to_owned(),
                        });
                    }
                }
            }
            Err(err) => {
                findings.push(Finding {
                    rule: err.rule,
                    subject: err.subject,
                    path: err.path,
                    line: None,
                    message: err.message,
                    repair: err.repair,
                });
            }
        }

        rf.insert("name".to_owned(), FactValue::str(row.name));
        rf.insert("mode".to_owned(), FactValue::str(row.mode.as_str()));
        rf.insert(
            "inputs".to_owned(),
            FactValue::str_array(row.input_globs.iter().copied()),
        );
        rf.insert(
            "outputs".to_owned(),
            FactValue::str_array(produced_paths.iter().cloned()),
        );
        row_facts.push(FactValue::Object(rf));
    }

    // Orphan check: strictly over wildcard output_globs declared by rows (never touches
    // hand-written schemas under schemas/registries/, schemas/demo.schema.json,
    // schemas/contract.schema.json, schemas/xtask.report.schema.json, etc.).
    let mut wildcard_globs: Vec<&str> = Vec::new();
    for row in rows {
        for &g in row.output_globs {
            if g.contains('*') {
                wildcard_globs.push(g);
            }
        }
    }
    for rel_pb in expand_globs(repo_root, &wildcard_globs) {
        let rel = rel_pb.to_string_lossy().replace('\\', "/");
        if !expected_outputs.contains(&rel) {
            findings.push(Finding {
                rule: RULE_ORPHAN,
                subject: rel.clone(),
                path: rel.clone(),
                line: None,
                message: format!(
                    "generated-directory file `{rel}` is not produced by any registered generator row"
                ),
                repair: "add a registry entry producing this output or remove the stale generated file with owner approval".to_owned(),
            });
        }
    }

    // Ensure rows in facts are deterministically ordered and written_files sorted.
    written_files.sort();
    written_files.dedup();
    let _ = GeneratorMode::Regenerate;

    let mut facts = BTreeMap::new();
    facts.insert(
        "schema".to_owned(),
        FactValue::str(CHECK_GENERATED_FACTS_SCHEMA),
    );
    facts.insert("write_mode".to_owned(), FactValue::Bool(write_mode));
    facts.insert("rows".to_owned(), FactValue::Array(row_facts));
    facts.insert(
        "checked_files".to_owned(),
        FactValue::int_usize(checked_files),
    );
    facts.insert(
        "digest_files_checked".to_owned(),
        FactValue::int_usize(digest_files_checked),
    );
    facts.insert(
        "written_files".to_owned(),
        FactValue::str_array(written_files),
    );

    Ok(CheckOutcome { findings, facts })
}

// ============================================================================
// Marker checking, line-diff calculation, and atomic writing helpers
// ============================================================================

fn has_expected_marker(bytes: &[u8], marker: &ExpectedMarker) -> bool {
    let Ok(text) = std::str::from_utf8(bytes) else {
        return false;
    };
    match marker {
        ExpectedMarker::LinePrefix(prefix) => {
            text.lines().next().is_some_and(|l| l.starts_with(prefix))
        }
        ExpectedMarker::JsonGeneratedComment => text.lines().take(8).any(|l| {
            (l.contains("\"$comment\"") && l.contains("@generated"))
                || (l.contains("\"description\"") && l.contains("Generated from "))
        }),
        ExpectedMarker::ReadmeStatusMarkers => {
            status_table::split_readme_status_block(text).is_ok()
        }
        ExpectedMarker::AdmissionDocHeader => {
            text.starts_with(admission_doc::FIRST_COMMIT_HEADER)
                && text.contains(admission_doc::STATUS_TABLE_HEADING)
        }
    }
}

fn has_kernel_generated_marker(bytes: &[u8]) -> bool {
    let Ok(text) = std::str::from_utf8(bytes) else {
        return false;
    };
    text.lines().next().is_some_and(|first| {
        let t = first.trim_start();
        (t.starts_with("//") || t.starts_with("/*") || t.starts_with('#'))
            && t.contains("@generated")
    })
}

fn first_diff_line_bytes(actual: &[u8], expected: &[u8]) -> u32 {
    let a = String::from_utf8_lossy(actual);
    let e = String::from_utf8_lossy(expected);
    u32::try_from(first_diff_offset_str(&a, &e) + 1).unwrap_or(u32::MAX)
}

fn first_diff_offset_str(actual: &str, expected: &str) -> usize {
    let a_lines: Vec<&str> = actual.split('\n').collect();
    let e_lines: Vec<&str> = expected.split('\n').collect();
    let max_len = a_lines.len().max(e_lines.len());
    for i in 0..max_len {
        if a_lines.get(i) != e_lines.get(i) {
            return i;
        }
    }
    0
}

fn atomic_write_bytes(dest: &Path, bytes: &[u8]) -> Result<(), String> {
    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("create_dir_all: {e}"))?;
    }
    static SEQ: AtomicU64 = AtomicU64::new(0);
    let seq = SEQ.fetch_add(1, Ordering::Relaxed);
    let name = dest
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("generated");
    let tmp = dest.with_file_name(format!(".{name}-{}-{seq}.tmp", std::process::id()));
    let mut f = File::create(&tmp).map_err(|e| format!("create temp: {e}"))?;
    f.write_all(bytes)
        .and_then(|()| f.sync_all())
        .map_err(|e| format!("write/sync temp: {e}"))?;
    drop(f);
    fs::rename(&tmp, dest).map_err(|e| format!("rename temp: {e}"))?;
    Ok(())
}
