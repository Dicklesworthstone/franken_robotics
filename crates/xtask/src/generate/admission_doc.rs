//! Deterministic generator for `DEPENDENCY_ADMISSION.md` from content-addressed
//! admission records under `receipts/admissions/` (bead `N33a-07`, consumed by
//! `N34p4-03` `check-generated` under rule L-8).
//!
//! Pure function (D0), zero `unsafe`, prints nothing (rule L-7), never deletes
//! files or directories (COMMON.md 9b).

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::fs::{self, File};
use std::io::Write as _;
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};

use crate::stack::admission::{
    ADMISSION_GATES, AdmissionRecord, AdmissionStatus, StoredAdmission, load_all_records,
};

/// Exact first-commit preamble of `DEPENDENCY_ADMISSION.md` shipped by `BD-002`
/// (kept byte-for-byte unchanged before the generated status table).
pub const FIRST_COMMIT_HEADER: &str = "\
# Dependency Admission Ledger (`DEPENDENCY_ADMISSION.md`)

A foundation (a sibling library or external crate consumed by FrankenRobotics) is not used above the airlock crate until its admission gate has a record in this file.

## Admission Rules

1. **Validity rule.** An admission record is valid only if it cites a commit of this repository whose tracked `Cargo.lock` contains the foundation, together with the exact command, its output (and content-addressed output hash), and the stack resolution receipt hash.
2. **One record per admission gate run, passing or failing.** Every gate attempt is recorded honestly; a failing or blocked observation is a record, and no gate `PASS` is recorded while any sibling checkout or the repository working tree is dirty (such observations are recorded as `development build`).
3. **Fields shown by each record.** Gate identifier, foundation identifier, the commit of this repository whose tracked `Cargo.lock` contains the foundation (or core skeleton for bootstrap), the command, its exit code and output excerpt/hash, the resolution receipt hash, measured values against `TARGET` thresholds, and date.
4. **Never inherited.** An admission is never inherited: a crate or feature group first used at a later milestone gets its own record then, at the pin of that day.
";

/// Heading that separates the unchanged BD-002 preamble from the generated table.
pub const STATUS_TABLE_HEADING: &str = "## Admission Status Table";

/// Result of checking or blessing `DEPENDENCY_ADMISSION.md`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdmissionDocOutcome {
    Current,
    Blessed,
}

/// Render `DEPENDENCY_ADMISSION.md` deterministically from all admission records
/// stored under `<repo_root>/receipts/admissions/`.
pub fn render(repo_root: &Path) -> Result<String, String> {
    let (records, parse_findings) = load_all_records(repo_root);
    if !parse_findings.is_empty() {
        let msgs: Vec<String> = parse_findings
            .iter()
            .map(|f| format!("{}: {}", f.path, f.message))
            .collect();
        return Err(format!(
            "cannot render DEPENDENCY_ADMISSION.md with malformed admission record(s): {}",
            msgs.join("; ")
        ));
    }
    Ok(render_from_records(&records))
}

/// Pure renderer from a slice of loaded admission records to Markdown (LF endings).
#[must_use]
pub fn render_from_records(records: &[StoredAdmission]) -> String {
    let mut sorted: Vec<&StoredAdmission> = records.iter().collect();
    sorted.sort_by(|a, b| {
        a.record
            .gate
            .cmp(&b.record.gate)
            .then_with(|| a.record.recorded.date.cmp(&b.record.recorded.date))
            .then_with(|| a.hash.cmp(&b.hash))
            .then_with(|| a.rel_path.cmp(&b.rel_path))
    });

    // Newest gate-own-scope record per gate (Review Notes 2 & 4: first-use records
    // never overwrite a register gate's own status-table row).
    let mut own_by_gate: BTreeMap<&str, &StoredAdmission> = BTreeMap::new();
    let mut extra_gates: BTreeMap<&str, &StoredAdmission> = BTreeMap::new();
    for &stored in &sorted {
        if !stored.record.is_gate_own_scope() {
            continue;
        }
        let g = stored.record.gate.as_str();
        if ADMISSION_GATES.iter().any(|spec| spec.id == g) {
            own_by_gate.insert(g, stored);
        } else {
            extra_gates.insert(g, stored);
        }
    }

    let mut out = String::with_capacity(4096);
    out.push_str(FIRST_COMMIT_HEADER);
    out.push('\n');
    out.push_str(STATUS_TABLE_HEADING);
    out.push_str("\n\n");
    out.push_str("| Gate | Foundation | Status | Measured vs Threshold | Receipt | Date |\n");
    out.push_str("|---|---|---|---|---|---|\n");

    for spec in ADMISSION_GATES {
        if let Some(stored) = own_by_gate.get(spec.id) {
            write_status_row(&mut out, &stored.record, &stored.hash);
        } else {
            let _ = writeln!(
                out,
                "| `{}` | `{}` | not run | — | — | — |",
                spec.id, spec.foundation
            );
        }
    }
    for stored in extra_gates.values() {
        write_status_row(&mut out, &stored.record, &stored.hash);
    }

    if !sorted.is_empty() {
        out.push_str("\n## Admission Records\n");
        for stored in &sorted {
            write_record_section(&mut out, stored);
        }
    }

    // Toolchain matrix and choice of T (populated when INT-TOOLCHAIN-01 records exist).
    let toolchain_records: Vec<&StoredAdmission> = sorted
        .iter()
        .copied()
        .filter(|s| s.record.gate == "INT-TOOLCHAIN-01")
        .collect();
    if !toolchain_records.is_empty() {
        out.push_str("\n## Toolchain Matrix and Choice of T\n\n");
        out.push_str("| Record | Toolchain T | Status | Measured | Date |\n");
        out.push_str("|---|---|---|---|---|\n");
        for s in toolchain_records {
            let measured = format_measured_inline(&s.record);
            let _ = writeln!(
                out,
                "| `{}` | `{}` | {} | {} | `{}` |",
                short_hash(&s.hash),
                escape_cell(&s.record.toolchain),
                status_display(s.record.status),
                escape_cell(&measured),
                escape_cell(&s.record.recorded.date),
            );
        }
    }

    out
}

/// Compare `<repo_root>/DEPENDENCY_ADMISSION.md` with fresh generation.
///
/// - When `bless == false`: returns `Ok(AdmissionDocOutcome::Current)` if
///   byte-identical, or `Err` naming the 1-indexed first differing line.
/// - When `bless == true` (`FROB_BLESS=1`): atomically rewrites
///   `DEPENDENCY_ADMISSION.md` and touches no other file.
pub fn check_or_bless(repo_root: &Path, bless: bool) -> Result<AdmissionDocOutcome, String> {
    let expected = render(repo_root)?;
    let doc_path = repo_root.join("DEPENDENCY_ADMISSION.md");

    if bless {
        atomic_write_text(&doc_path, &expected)?;
        return Ok(AdmissionDocOutcome::Blessed);
    }

    let actual = fs::read_to_string(&doc_path)
        .map_err(|e| format!("failed to read DEPENDENCY_ADMISSION.md: {e}"))?;
    if actual == expected {
        return Ok(AdmissionDocOutcome::Current);
    }

    let diff_line = first_differing_line(&actual, &expected);
    Err(format!(
        "DEPENDENCY_ADMISSION.md differs from generated output at line {diff_line} (re-run with `FROB_BLESS=1` to update)"
    ))
}

/// Find the 1-indexed line number of the first difference between two texts.
#[must_use]
pub fn first_differing_line(actual: &str, expected: &str) -> usize {
    let mut a_lines = actual.lines();
    let mut e_lines = expected.lines();
    let mut line_no = 1usize;
    loop {
        match (a_lines.next(), e_lines.next()) {
            (Some(a), Some(e)) if a == e => line_no += 1,
            (None, None) => return line_no,
            _ => return line_no,
        }
    }
}

fn write_status_row(out: &mut String, rec: &AdmissionRecord, hash: &str) {
    let measured = format_measured_inline(rec);
    let _ = writeln!(
        out,
        "| `{}` | `{}` | {} | {} | `{}` (`{}`) | `{}` |",
        rec.gate,
        rec.foundation,
        status_display(rec.status),
        escape_cell(&measured),
        short_hash(hash),
        short_hash(rec.stack_receipt_hex()),
        escape_cell(&rec.recorded.date),
    );
}

fn write_record_section(out: &mut String, stored: &StoredAdmission) {
    let rec = &stored.record;
    let scope_note = if rec.is_gate_own_scope() {
        "gate scope"
    } else {
        "first-use scope"
    };
    let _ = writeln!(
        out,
        "\n### {} — `{}` ({}, {})\n",
        rec.gate,
        short_hash(&stored.hash),
        status_display(rec.status),
        scope_note
    );
    let _ = writeln!(out, "- **Foundation:** `{}`", rec.foundation);
    let _ = writeln!(out, "- **Record file:** `{}`", stored.rel_path);
    let _ = writeln!(out, "- **Status:** `{}`", rec.status.as_str());
    let _ = writeln!(out, "- **Date:** `{}`", rec.recorded.date);
    let _ = writeln!(
        out,
        "- **Attribution:** session=`{}`, implemented_by=`{}`",
        rec.recorded.session, rec.recorded.implemented_by
    );
    let _ = writeln!(
        out,
        "- **Repository commit / workspace:** `{}` (`{}`)",
        rec.repo_rev, rec.workspace
    );
    let _ = writeln!(out, "- **Stack receipt:** `{}`", rec.stack_receipt);
    let _ = writeln!(
        out,
        "- **Sibling pin:** `{}`",
        rec.sibling_rev.as_deref().unwrap_or("none")
    );
    let _ = writeln!(out, "- **Toolchain:** `{}`", rec.toolchain);
    let _ = writeln!(
        out,
        "- **Harness receipt:** `{}`",
        rec.harness_receipt.as_deref().unwrap_or("null")
    );
    let _ = writeln!(
        out,
        "- **Covers:** crates=[{}], groups=[{}], tool={}",
        rec.covers.crates.join(", "),
        rec.covers.groups.join(", "),
        match &rec.covers.tool {
            Some(t) => format!(
                "`{}` (rev={}, binary={})",
                t.name,
                t.rev.as_deref().unwrap_or("unpinned"),
                t.binary_hash.as_deref().unwrap_or("unpinned")
            ),
            None => "none".to_owned(),
        }
    );
    let _ = writeln!(
        out,
        "- **Command (exit {}):** `{}`",
        rec.exit,
        rec.command.join(" ")
    );
    let _ = writeln!(
        out,
        "- **Output blob:** `receipts/outputs/{}.txt`",
        rec.output_hash
    );
    let _ = writeln!(
        out,
        "- **Measured:** {}",
        format_measured_inline(rec)
    );
    if !rec.controls.is_empty() {
        let ctrls: Vec<String> = rec
            .controls
            .iter()
            .map(|c| format!("`{}` (caught={})", c.id, c.caught))
            .collect();
        let _ = writeln!(out, "- **Controls:** {}", ctrls.join(", "));
    }
    if !rec.facts_reread.is_empty() {
        out.push_str("- **Re-read facts:**\n");
        for f in &rec.facts_reread {
            let _ = writeln!(out, "  - `{}` (`{}`): `{}`", f.fact, f.path, f.value);
        }
    }
    if !rec.output_excerpt.is_empty() {
        out.push_str("\n```text\n");
        out.push_str(rec.output_excerpt.trim_end_matches('\n'));
        out.push_str("\n```\n");
    }
}

fn format_measured_inline(rec: &AdmissionRecord) -> String {
    if rec.measured.is_empty() {
        return "—".to_owned();
    }
    let parts: Vec<String> = rec
        .measured
        .iter()
        .map(|m| {
            let val = m
                .value
                .map_or_else(|| "<missing>".to_owned(), |v| v.to_string());
            format!(
                "{}={} {} (target {} {} {})",
                m.name,
                val,
                m.unit,
                m.relation.as_str(),
                m.threshold,
                m.unit
            )
        })
        .collect();
    parts.join("; ")
}

fn status_display(s: AdmissionStatus) -> &'static str {
    match s {
        AdmissionStatus::Pass => "pass",
        AdmissionStatus::Fail => "fail",
        AdmissionStatus::NotRun => "not run",
        AdmissionStatus::Killed => "killed",
    }
}

fn short_hash(hex: &str) -> &str {
    hex.get(..12).unwrap_or(hex)
}

fn escape_cell(raw: &str) -> String {
    raw.replace('|', "\\|").replace('\n', " ")
}

fn atomic_write_text(dest: &Path, content: &str) -> Result<(), String> {
    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("create_dir_all: {e}"))?;
    }
    static SEQ: AtomicU64 = AtomicU64::new(0);
    let seq = SEQ.fetch_add(1, Ordering::Relaxed);
    let name = dest
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("DEPENDENCY_ADMISSION.md");
    let tmp = dest.with_file_name(format!(".{name}-{}-{seq}.tmp", std::process::id()));
    let mut f = File::create(&tmp).map_err(|e| format!("create temp: {e}"))?;
    f.write_all(content.as_bytes())
        .and_then(|()| f.sync_all())
        .map_err(|e| format!("write/sync temp: {e}"))?;
    drop(f);
    fs::rename(&tmp, dest).map_err(|e| format!("rename temp: {e}"))?;
    Ok(())
}

// ============================================================================
// Unit tests (`cargo test -p xtask generate::admission_doc`)
// ============================================================================

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use crate::stack::admission::{
        ADMISSION_SCHEMA, AdmissionCovers, ControlResult, FactReread, HostInfo, MeasureNum,
        MeasureRelation, MeasuredEntry, RecordedBy,
    };

    fn fresh_dir(tag: &str) -> PathBuf {
        static CTR: AtomicU64 = AtomicU64::new(0);
        let dir = std::env::temp_dir().join(format!(
            "frob_xtask_admdoc_{tag}_{}_{}",
            std::process::id(),
            CTR.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(&dir).expect("mkdir admdoc fixture");
        dir
    }

    fn repo_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("crates")
            .parent()
            .expect("workspace root")
            .to_path_buf()
    }

    fn sample_record(gate: &str, status: AdmissionStatus) -> StoredAdmission {
        let (foundation, covers) = match crate::stack::admission::gate_spec(gate) {
            Some(spec) => (
                spec.foundation.to_owned(),
                AdmissionCovers {
                    crates: spec.own_crates.iter().map(|s| (*s).to_owned()).collect(),
                    groups: spec.own_groups.iter().map(|s| (*s).to_owned()).collect(),
                    tool: spec.own_tool.map(|name| crate::stack::admission::ToolSpec {
                        name: name.to_owned(),
                        rev: Some("c".repeat(40)),
                        binary_hash: Some("e".repeat(64)),
                    }),
                },
            ),
            None => (
                "FD-FSIM".to_owned(),
                AdmissionCovers {
                    crates: vec![
                        "fs-contact".to_owned(),
                        "fs-ga".to_owned(),
                        "fs-mbd".to_owned(),
                        "fs-query".to_owned(),
                        "fs-time".to_owned(),
                    ],
                    groups: Vec::new(),
                    tool: None,
                },
            ),
        };
        let rec = AdmissionRecord {
            schema: ADMISSION_SCHEMA.to_owned(),
            gate: gate.to_owned(),
            foundation,
            covers,
            repo_rev: "a".repeat(40),
            workspace: "core".to_owned(),
            stack_receipt: format!("sr:{}", "b".repeat(64)),
            sibling_rev: Some("c".repeat(40)),
            toolchain: "nightly-2026-08-31".to_owned(),
            harness_receipt: None,
            command: vec!["cargo".to_owned(), "check".to_owned()],
            exit: if status == AdmissionStatus::Pass { 0 } else { 1 },
            output_hash: "d".repeat(64),
            output_excerpt: "sample output\n".to_owned(),
            measured: vec![MeasuredEntry {
                name: "cold_check_s".to_owned(),
                value: Some(MeasureNum::Int(120)),
                unit: "s".to_owned(),
                threshold: MeasureNum::Int(3600),
                relation: MeasureRelation::Le,
            }],
            controls: vec![ControlResult {
                id: "NC-0001".to_owned(),
                caught: true,
            }],
            facts_reread: vec![FactReread {
                fact: "MAX_CONTACTS".to_owned(),
                path: "crates/fs-contact/src/lib.rs:1".to_owned(),
                value: "21".to_owned(),
            }],
            status,
            host: HostInfo {
                class: "m5-ultra".to_owned(),
                fingerprint: None,
            },
            recorded: RecordedBy {
                date: "2026-11-24T22:00:00Z".to_owned(),
                session: "session:reviewer".to_owned(),
                implemented_by: "session:author".to_owned(),
            },
        };
        let canon = rec.to_canonical_bytes().expect("canon");
        let hash = fs_blake3::hash_bytes(canon.as_bytes()).to_hex();
        StoredAdmission {
            rel_path: format!("receipts/admissions/{gate}/{hash}.json"),
            abs_path: PathBuf::from(format!("receipts/admissions/{gate}/{hash}.json")),
            hash: hash.clone(),
            stem: hash,
            record: rec,
        }
    }

    #[test]
    fn generator_prints_not_run() {
        let rendered = render_from_records(&[]);
        for spec in ADMISSION_GATES {
            let expected_row = format!("| `{}` | `{}` | not run |", spec.id, spec.foundation);
            assert!(
                rendered.contains(&expected_row),
                "missing not-run row for `{}`",
                spec.id
            );
        }
        assert!(
            !rendered.contains("| pass |"),
            "no gate may print as pass when zero records exist"
        );
    }

    #[test]
    fn generator_keeps_first_commit_header() {
        let rendered = render_from_records(&[]);
        let (before, _after) = rendered
            .split_once(STATUS_TABLE_HEADING)
            .expect("status table heading present");
        let expected_prefix = format!("{FIRST_COMMIT_HEADER}\n");
        assert_eq!(
            before.as_bytes(),
            expected_prefix.as_bytes(),
            "bytes before the status table must equal the BD-002 first-commit header"
        );
    }

    #[test]
    fn generator_is_deterministic() {
        let r1 = sample_record("INT-FSIM-01", AdmissionStatus::Pass);
        let r2 = sample_record("INT-FSIM-02", AdmissionStatus::Fail);
        let out_a = render_from_records(&[r1.clone(), r2.clone()]);
        let out_b = render_from_records(&[r2, r1]);
        assert_eq!(out_a.as_bytes(), out_b.as_bytes());
        assert!(!out_a.contains('\r'), "must use LF line endings only");
        assert!(out_a.contains("cold_check_s=120 s"));
        assert!(out_a.contains("| `INT-FSIM-02` | `FD-FSIM` | fail |"));
    }

    #[test]
    fn admission_doc_is_current() {
        let root = repo_root();
        let bless = std::env::var("FROB_BLESS").as_deref() == Ok("1");
        let outcome = check_or_bless(&root, bless)
            .unwrap_or_else(|e| panic!("DEPENDENCY_ADMISSION.md check failed: {e}"));
        if bless {
            assert_eq!(outcome, AdmissionDocOutcome::Blessed);
        } else {
            assert_eq!(outcome, AdmissionDocOutcome::Current);
        }

        // Hand-edit test on a scratch copy: fails naming the exact first differing line,
        // and bless rewrites ONLY DEPENDENCY_ADMISSION.md while leaving bystander files untouched.
        let scratch = fresh_dir("hand_edit");
        let bystander = scratch.join("BYSTANDER.txt");
        fs::write(&bystander, "untouched\n").expect("write bystander");
        let fresh = render(&scratch).expect("render empty");
        let edited = fresh.replace("| `INT-FSIM-01` | `FD-FSIM` | not run |", "| `INT-FSIM-01` | `FD-FSIM` | pass |");
        fs::write(scratch.join("DEPENDENCY_ADMISSION.md"), &edited).expect("write edited");

        let err = check_or_bless(&scratch, false).expect_err("hand edit must fail");
        let expected_line = first_differing_line(&edited, &fresh);
        assert!(
            err.contains(&format!("line {expected_line}")),
            "error must name first differing line ({expected_line}), got: {err}"
        );

        let blessed = check_or_bless(&scratch, true).expect("bless must succeed");
        assert_eq!(blessed, AdmissionDocOutcome::Blessed);
        assert_eq!(
            fs::read_to_string(scratch.join("DEPENDENCY_ADMISSION.md")).expect("read blessed"),
            fresh
        );
        assert_eq!(
            fs::read_to_string(&bystander).expect("read bystander"),
            "untouched\n"
        );
    }
}
