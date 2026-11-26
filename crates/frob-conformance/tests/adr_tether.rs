#![deny(unsafe_code)]

//! Tether Architecture Decision Record conformance suite (bead N32a-01).
//!
//! Run as `cargo test -p frob-conformance --test adr_tether`.
//! Supports `FROB_CONFORMANCE_ROOT=<dir>` for scratch negative-control verification in
//! `scripts/e2e/tether_adr_record.sh`. Never deletes files or directories (COMMON.md 9b).

use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use frob_core::canon::{CanonWriter, Canonical, Value as CanonValue};
use frob_core::id::{Blake3, SchemaName, identity};
use frob_core::registry::toml::{self, Fields, Spanned, Table, Value};
use frob_core::testkit::registry;

const ADR_INDEX_PATH: &str = "registries/adr/tether.toml";
const ADR_INDEX_SCHEMA: &str = "frob.adr-index/1";
const EXPECTED_ADR_IDS: &[&str] = &[
    "ADR-TETHER-01",
    "ADR-TETHER-02",
    "ADR-TETHER-03",
    "ADR-TETHER-04",
    "ADR-TETHER-05",
    "ADR-TETHER-06",
    "ADR-TETHER-07",
    "ADR-TETHER-08",
    "ADR-TETHER-09",
    "ADR-TETHER-10",
    "ADR-TETHER-11",
];
const REQUIRED_ADR_SECTIONS: &[&str] = &[
    "Decision",
    "Reason",
    "Alternative excluded",
    "Enforced by",
    "Revisit when",
];
const EXPECTED_PRINCIPLE_LINTS: &[&str] = &[
    "principle-no-solver-step-verb",
    "principle-five-explicits",
    "principle-no-per-verb-surface-code",
    "principle-surface-claim-wording",
];

#[derive(Debug, Clone, PartialEq, Eq)]
struct EnforcedCheck {
    id: String,
    kind: String,
    state: String,
    milestone: String,
    source: Option<String>,
    owner_bead: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct AdrRecord {
    id: String,
    doc: String,
    status: String,
    superseded_by: Option<String>,
    bead: String,
    first_milestone: String,
    enforced_by: Vec<EnforcedCheck>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PrincipleLint {
    id: String,
    principle: i64,
    kind: String,
    description: String,
    state: String,
    milestone: String,
    owner_bead: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct AdrIndex {
    schema: String,
    subsystem: String,
    current_milestone: String,
    principles_doc: String,
    ownership_doc: String,
    records: Vec<AdrRecord>,
    principle_lints: Vec<PrincipleLint>,
}

impl Canonical for EnforcedCheck {
    fn write(&self, w: CanonValue<'_>) -> Result<(), frob_core::canon::CanonError> {
        w.object(|o| {
            o.str("id", &self.id)?;
            o.str("kind", &self.kind)?;
            o.str("state", &self.state)?;
            o.str("milestone", &self.milestone)?;
            match &self.source {
                Some(s) => o.str("source", s)?,
                None => o.null("source")?,
            }
            match &self.owner_bead {
                Some(b) => o.str("owner_bead", b)?,
                None => o.null("owner_bead")?,
            }
            Ok(())
        })
    }
}

impl Canonical for AdrRecord {
    fn write(&self, w: CanonValue<'_>) -> Result<(), frob_core::canon::CanonError> {
        w.object(|o| {
            o.str("id", &self.id)?;
            o.str("doc", &self.doc)?;
            o.str("status", &self.status)?;
            match &self.superseded_by {
                Some(s) => o.str("superseded_by", s)?,
                None => o.null("superseded_by")?,
            }
            o.str("bead", &self.bead)?;
            o.str("first_milestone", &self.first_milestone)?;
            o.array("enforced_by", |a| {
                for chk in &self.enforced_by {
                    a.push_term(chk)?;
                }
                Ok(())
            })?;
            Ok(())
        })
    }
}

impl Canonical for PrincipleLint {
    fn write(&self, w: CanonValue<'_>) -> Result<(), frob_core::canon::CanonError> {
        w.object(|o| {
            o.str("id", &self.id)?;
            o.int53("principle", self.principle)?;
            o.str("kind", &self.kind)?;
            o.str("description", &self.description)?;
            o.str("state", &self.state)?;
            o.str("milestone", &self.milestone)?;
            o.str("owner_bead", &self.owner_bead)?;
            Ok(())
        })
    }
}

impl Canonical for AdrIndex {
    fn write(&self, w: CanonValue<'_>) -> Result<(), frob_core::canon::CanonError> {
        w.object(|o| {
            o.str("schema", &self.schema)?;
            o.str("subsystem", &self.subsystem)?;
            o.str("current_milestone", &self.current_milestone)?;
            o.str("principles_doc", &self.principles_doc)?;
            o.str("ownership_doc", &self.ownership_doc)?;
            o.array("records", |a| {
                let mut sorted: Vec<&AdrRecord> = self.records.iter().collect();
                sorted.sort_by(|l, r| l.id.cmp(&r.id));
                for rec in sorted {
                    a.push_term(rec)?;
                }
                Ok(())
            })?;
            o.array("principle_lints", |a| {
                let mut sorted: Vec<&PrincipleLint> = self.principle_lints.iter().collect();
                sorted.sort_by(|l, r| l.id.cmp(&r.id));
                for pl in sorted {
                    a.push_term(pl)?;
                }
                Ok(())
            })?;
            Ok(())
        })
    }
}

fn workspace_root() -> PathBuf {
    if let Some(ov) = env::var_os("FROB_CONFORMANCE_ROOT") {
        return PathBuf::from(ov);
    }
    registry::root().expect("locate workspace root via franken-stack.lock")
}

fn milestone_order(m: &str) -> Result<u8, String> {
    let rest = m
        .strip_prefix('M')
        .ok_or_else(|| format!("invalid milestone `{m}`: must match M0..M8"))?;
    let n: u8 = rest
        .parse()
        .map_err(|_| format!("invalid milestone `{m}`: must match M0..M8"))?;
    if n > 8 {
        return Err(format!("milestone `{m}` out of range M0..M8"));
    }
    Ok(n)
}

fn expect_table(sp: Spanned<Value>, ctx: &str) -> Result<Table, String> {
    match sp.value {
        Value::Table(t) => Ok(t),
        other => Err(format!(
            "{}:{}: expected table for `{ctx}`, found {}",
            sp.line,
            sp.column,
            other.type_name()
        )),
    }
}

fn parse_adr_index(bytes: &[u8]) -> Result<AdrIndex, String> {
    let root = toml::parse(bytes).map_err(|e| format!("TOML parse error: {e}"))?;
    let mut top = Fields::new(root, ADR_INDEX_PATH);
    let schema = top.require_str("schema").map_err(|e| e.to_string())?;
    if schema != ADR_INDEX_SCHEMA {
        return Err(top.fail(format!(
            "invalid schema `{schema}`, expected `{ADR_INDEX_SCHEMA}`"
        )));
    }
    let subsystem = top.require_str("subsystem").map_err(|e| e.to_string())?;
    let current_milestone = top
        .require_str("current_milestone")
        .map_err(|e| e.to_string())?;
    let principles_doc = top
        .require_str("principles_doc")
        .map_err(|e| e.to_string())?;
    let ownership_doc = top
        .require_str("ownership_doc")
        .map_err(|e| e.to_string())?;
    let raw_records = top.require_array("record").map_err(|e| e.to_string())?;
    let raw_lints = top
        .require_array("principle_lint")
        .map_err(|e| e.to_string())?;
    top.finish().map_err(|e| e.to_string())?;
    let _ = milestone_order(&current_milestone)?;

    let mut records = Vec::with_capacity(raw_records.len());
    for (idx, sp) in raw_records.into_iter().enumerate() {
        let tbl = expect_table(sp, &format!("record[{idx}]"))?;
        let mut f = Fields::new(tbl, format!("record[{idx}]"));
        let id = f.require_str("id").map_err(|e| e.to_string())?;
        let doc = f.require_str("doc").map_err(|e| e.to_string())?;
        let status = f.require_str("status").map_err(|e| e.to_string())?;
        let superseded_by = f.opt_str("superseded_by").map_err(|e| e.to_string())?;
        let bead = f.require_str("bead").map_err(|e| e.to_string())?;
        let first_milestone = f
            .require_str("first_milestone")
            .map_err(|e| e.to_string())?;
        let raw_checks = f.require_array("enforced_by").map_err(|e| e.to_string())?;
        f.finish().map_err(|e| e.to_string())?;

        if !matches!(status.as_str(), "accepted" | "superseded") {
            return Err(format!(
                "record `{id}` has invalid status `{status}` (expected `accepted` or `superseded`)"
            ));
        }
        if status == "superseded" && superseded_by.as_deref().unwrap_or("").is_empty() {
            return Err(format!(
                "superseded record `{id}` must declare `superseded_by`"
            ));
        }
        let _ = milestone_order(&first_milestone)?;

        let mut enforced_by = Vec::with_capacity(raw_checks.len());
        for (cidx, csp) in raw_checks.into_iter().enumerate() {
            let ctbl = expect_table(csp, &format!("{id}.enforced_by[{cidx}]"))?;
            let mut cf = Fields::new(ctbl, format!("{id}.enforced_by[{cidx}]"));
            let cid = cf.require_str("id").map_err(|e| e.to_string())?;
            let kind = cf.require_str("kind").map_err(|e| e.to_string())?;
            let state = cf.require_str("state").map_err(|e| e.to_string())?;
            let milestone = cf.require_str("milestone").map_err(|e| e.to_string())?;
            let source = cf.opt_str("source").map_err(|e| e.to_string())?;
            let owner_bead = cf.opt_str("owner_bead").map_err(|e| e.to_string())?;
            cf.finish().map_err(|e| e.to_string())?;

            if !matches!(kind.as_str(), "test" | "lint" | "gate") {
                return Err(format!(
                    "record `{id}` check `{cid}` has invalid kind `{kind}`"
                ));
            }
            if !matches!(state.as_str(), "active" | "pending") {
                return Err(format!(
                    "record `{id}` check `{cid}` has invalid state `{state}`"
                ));
            }
            let _ = milestone_order(&milestone)?;

            enforced_by.push(EnforcedCheck {
                id: cid,
                kind,
                state,
                milestone,
                source,
                owner_bead,
            });
        }

        records.push(AdrRecord {
            id,
            doc,
            status,
            superseded_by,
            bead,
            first_milestone,
            enforced_by,
        });
    }

    let mut principle_lints = Vec::with_capacity(raw_lints.len());
    for (idx, sp) in raw_lints.into_iter().enumerate() {
        let tbl = expect_table(sp, &format!("principle_lint[{idx}]"))?;
        let mut f = Fields::new(tbl, format!("principle_lint[{idx}]"));
        let id = f.require_str("id").map_err(|e| e.to_string())?;
        let principle = f.require_int("principle").map_err(|e| e.to_string())?;
        let kind = f.require_str("kind").map_err(|e| e.to_string())?;
        let description = f.require_str("description").map_err(|e| e.to_string())?;
        let state = f.require_str("state").map_err(|e| e.to_string())?;
        let milestone = f.require_str("milestone").map_err(|e| e.to_string())?;
        let owner_bead = f.require_str("owner_bead").map_err(|e| e.to_string())?;
        f.finish().map_err(|e| e.to_string())?;

        if !(1..=6).contains(&principle) {
            return Err(format!(
                "principle_lint `{id}` has principle `{principle}` outside 1..=6"
            ));
        }
        let _ = milestone_order(&milestone)?;

        principle_lints.push(PrincipleLint {
            id,
            principle,
            kind,
            description,
            state,
            milestone,
            owner_bead,
        });
    }

    Ok(AdrIndex {
        schema,
        subsystem,
        current_milestone,
        principles_doc,
        ownership_doc,
        records,
        principle_lints,
    })
}

fn validate_index_completeness(index: &AdrIndex) -> Result<(), String> {
    let mut seen = BTreeSet::new();
    for rec in &index.records {
        if !seen.insert(rec.id.as_str()) {
            return Err(format!("duplicate ADR record `{}` in index", rec.id));
        }
        if !EXPECTED_ADR_IDS.contains(&rec.id.as_str()) {
            return Err(format!(
                "unexpected ADR record `{}` in Tether index",
                rec.id
            ));
        }
    }
    for &expected in EXPECTED_ADR_IDS {
        if !seen.contains(expected) {
            return Err(format!("missing ADR record `{expected}` in Tether index"));
        }
    }
    if index.records.len() != EXPECTED_ADR_IDS.len() {
        return Err(format!(
            "expected {} Tether ADR records, found {}",
            EXPECTED_ADR_IDS.len(),
            index.records.len()
        ));
    }
    Ok(())
}

fn load_registered_gate_ids(root: &Path) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    let path = root.join("registries/gates.toml");
    let Ok(bytes) = fs::read(&path) else {
        return out;
    };
    let Ok(tbl) = toml::parse(&bytes) else {
        return out;
    };
    if let Some(arr) = tbl.get("gate").and_then(Value::as_array) {
        for item in arr {
            if let Some(id) = item
                .value
                .as_table()
                .and_then(|t| t.get("id"))
                .and_then(Value::as_str)
            {
                out.insert(id.to_owned());
            }
        }
    }
    out
}

fn validate_enforced_by(index: &AdrIndex, root: &Path) -> Result<(), String> {
    let gate_ids = load_registered_gate_ids(root);

    for rec in &index.records {
        let rec_ms = milestone_order(&rec.first_milestone)?;
        if rec.status == "accepted" && rec.enforced_by.is_empty() {
            return Err(format!(
                "accepted record `{}` has empty `enforced_by` list",
                rec.id
            ));
        }
        for chk in &rec.enforced_by {
            let chk_ms = milestone_order(&chk.milestone)?;
            if chk.state == "active" {
                match chk.kind.as_str() {
                    "test" => {
                        let rel = chk.source.as_deref().ok_or_else(|| {
                            format!(
                                "record `{}` active test `{}` must specify `source`",
                                rec.id, chk.id
                            )
                        })?;
                        let full = root.join(rel);
                        let src = fs::read_to_string(&full).map_err(|e| {
                            format!(
                                "record `{}` active test `{}` source `{rel}` unreadable: {e}",
                                rec.id, chk.id
                            )
                        })?;
                        let leaf = chk.id.rsplit("::").next().unwrap_or(&chk.id);
                        let needle = format!("fn {leaf}(");
                        if !src.contains(&needle) {
                            return Err(format!(
                                "record `{}` active test `{}` not found (`{needle}` absent from `{rel}`)",
                                rec.id, chk.id
                            ));
                        }
                    }
                    "gate" => {
                        if !gate_ids.contains(&chk.id) {
                            return Err(format!(
                                "record `{}` active gate `{}` is not registered in registries/gates.toml",
                                rec.id, chk.id
                            ));
                        }
                    }
                    "lint" => {
                        let rel = chk.source.as_deref().ok_or_else(|| {
                            format!(
                                "record `{}` active lint `{}` must specify `source`",
                                rec.id, chk.id
                            )
                        })?;
                        let full = root.join(rel);
                        let src = fs::read_to_string(&full).map_err(|e| {
                            format!(
                                "record `{}` active lint `{}` source `{rel}` unreadable: {e}",
                                rec.id, chk.id
                            )
                        })?;
                        if !src.contains(&chk.id) {
                            return Err(format!(
                                "record `{}` active lint `{}` not found in `{rel}`",
                                rec.id, chk.id
                            ));
                        }
                    }
                    other => {
                        return Err(format!(
                            "record `{}` check `{}` has unknown kind `{other}`",
                            rec.id, chk.id
                        ));
                    }
                }
            } else {
                // Pending check: must carry owner_bead and milestone <= record.first_milestone.
                if chk.owner_bead.as_deref().unwrap_or("").trim().is_empty() {
                    return Err(format!(
                        "record `{}` pending check `{}` must name `owner_bead`",
                        rec.id, chk.id
                    ));
                }
                if rec.status == "accepted" && chk_ms > rec_ms {
                    return Err(format!(
                        "accepted record `{}` (first_milestone {}) has check `{}` pending beyond its milestone ({})",
                        rec.id, rec.first_milestone, chk.id, chk.milestone
                    ));
                }
            }
        }
    }
    Ok(())
}

fn validate_principle_lints(index: &AdrIndex, root: &Path) -> Result<(), String> {
    let principles_text = fs::read_to_string(root.join(&index.principles_doc))
        .map_err(|e| format!("failed to read `{}`: {e}", index.principles_doc))?;
    let registered: BTreeSet<&str> = index
        .principle_lints
        .iter()
        .map(|l| l.id.as_str())
        .collect();
    if index.principle_lints.len() != registered.len() {
        return Err("duplicate principle_lint identifier in Tether ADR index".to_owned());
    }
    for &expected in EXPECTED_PRINCIPLE_LINTS {
        if !registered.contains(expected) {
            return Err(format!(
                "principle lint `{expected}` is missing from registries/adr/tether.toml"
            ));
        }
        if !principles_text.contains(expected) {
            return Err(format!(
                "principle lint `{expected}` is not cited in {}",
                index.principles_doc
            ));
        }
    }
    Ok(())
}

fn parse_markdown_sections(doc: &str) -> Vec<(String, String)> {
    let mut sections: Vec<(String, String)> = Vec::new();
    let mut current: Option<(String, String)> = None;
    for line in doc.lines() {
        if let Some(heading) = line.strip_prefix("## ") {
            if let Some(prev) = current.take() {
                sections.push(prev);
            }
            current = Some((heading.trim().to_owned(), String::new()));
        } else if let Some((_, ref mut body)) = current {
            body.push_str(line);
            body.push('\n');
        }
    }
    if let Some(prev) = current {
        sections.push(prev);
    }
    sections
}

fn parse_ownership_table_rows(doc: &str) -> Vec<Vec<String>> {
    let mut rows = Vec::new();
    let mut in_table = false;
    for line in doc.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("| Artifact |") {
            in_table = true;
            continue;
        }
        if !in_table {
            continue;
        }
        if !trimmed.starts_with('|') {
            break;
        }
        // Skip header separator row (`|---|---|---|---|---|`).
        if trimmed
            .trim_matches('|')
            .chars()
            .all(|c| matches!(c, '-' | '|' | ' ' | ':'))
        {
            continue;
        }
        let cols: Vec<String> = trimmed
            .trim_matches('|')
            .split('|')
            .map(|c| c.trim().to_owned())
            .collect();
        rows.push(cols);
    }
    rows
}

fn index_canonical_digest(index: &AdrIndex) -> Blake3 {
    let mut writer = CanonWriter::new();
    writer.write_term(index).expect("canonical write ADR index");
    let canon = writer.finish().expect("finish canonical ADR index");
    let schema = SchemaName::parse(ADR_INDEX_SCHEMA).expect("valid schema name");
    identity(&schema, &canon)
}

#[test]
fn adr_docs_complete_and_flat() {
    let root = workspace_root();
    assert!(
        !root.join("docs/adr/tether").exists(),
        "per-chapter directory docs/adr/tether/ must not exist"
    );

    let index_bytes = fs::read(root.join(ADR_INDEX_PATH)).expect("read registries/adr/tether.toml");
    let index = parse_adr_index(&index_bytes).expect("parse registries/adr/tether.toml");

    for rec in &index.records {
        let expected_rel = format!("docs/adr/{}.md", rec.id);
        assert_eq!(rec.doc, expected_rel);
        let text = fs::read_to_string(root.join(&rec.doc))
            .unwrap_or_else(|e| panic!("missing ADR doc `{}`: {e}", rec.doc));
        let sections = parse_markdown_sections(&text);
        for &req in REQUIRED_ADR_SECTIONS {
            let found = sections
                .iter()
                .find(|(h, _)| h == req)
                .unwrap_or_else(|| panic!("{} missing required section `## {req}`", rec.doc));
            assert!(
                !found.1.trim().is_empty(),
                "{} section `## {req}` must be non-empty",
                rec.doc
            );
        }
    }

    // Verify required revisit triggers and OPEN-TETHER-1 decision statements.
    let adr02 = fs::read_to_string(root.join("docs/adr/ADR-TETHER-02.md")).unwrap();
    let adr03 = fs::read_to_string(root.join("docs/adr/ADR-TETHER-03.md")).unwrap();
    let adr06 = fs::read_to_string(root.join("docs/adr/ADR-TETHER-06.md")).unwrap();
    let adr07 = fs::read_to_string(root.join("docs/adr/ADR-TETHER-07.md")).unwrap();
    assert!(adr02.contains("KILL-MCP"));
    assert!(adr03.contains("OPEN-TETHER-1") && adr03.contains("KILL-MCP"));
    assert!(adr06.contains("KILL-MCPTASKS"));
    assert!(adr07.contains("KILL-SLOT"));

    // Six principles with FACT and DESIGN labels.
    let principles = fs::read_to_string(root.join(&index.principles_doc)).unwrap();
    let principle_headings: Vec<_> = parse_markdown_sections(&principles)
        .into_iter()
        .filter(|(h, _)| h.chars().next().is_some_and(|c| c.is_ascii_digit()) && h.contains(". "))
        .collect();
    assert_eq!(
        principle_headings.len(),
        6,
        "docs/adr/TETHER-PRINCIPLES.md must declare exactly 6 principles"
    );
    assert!(principles.contains("FACT"));
    assert!(principles.contains("DESIGN"));

    // Seven-row ownership map with tiers, first milestones, and DESIGN daemon split note.
    let ownership = fs::read_to_string(root.join(&index.ownership_doc)).unwrap();
    let rows = parse_ownership_table_rows(&ownership);
    assert_eq!(
        rows.len(),
        7,
        "docs/adr/TETHER-OWNERSHIP.md must hold exactly 7 ownership rows"
    );
    for row in &rows {
        assert_eq!(row.len(), 5, "each ownership row must have 5 columns");
        assert!(row[3].contains("Solid"));
        assert!(row[4].contains('M'));
    }
    assert!(ownership.contains("DESIGN"));
}

#[test]
fn adr_index_complete() {
    let root = workspace_root();
    let bytes = fs::read(root.join(ADR_INDEX_PATH)).expect("read registries/adr/tether.toml");
    let index = parse_adr_index(&bytes).expect("parse registries/adr/tether.toml");
    assert_eq!(index.records.len(), 11);
    validate_index_completeness(&index).expect("committed Tether ADR index must be complete");

    // Negative control 1: duplicate ADR record fails naming the duplicate.
    let mut dup = index.clone();
    dup.records.push(index.records[2].clone());
    let err_dup = validate_index_completeness(&dup).unwrap_err();
    assert!(
        err_dup.contains("ADR-TETHER-03"),
        "expected duplicate error naming ADR-TETHER-03, got: {err_dup}"
    );

    // Negative control 2: missing ADR record (gap) fails naming the gap.
    let mut gap = index.clone();
    gap.records.retain(|r| r.id != "ADR-TETHER-07");
    let err_gap = validate_index_completeness(&gap).unwrap_err();
    assert!(
        err_gap.contains("ADR-TETHER-07"),
        "expected gap error naming ADR-TETHER-07, got: {err_gap}"
    );

    // Negative control 3: unknown key in strict TOML fails parse.
    let with_unknown = format!(
        "{}\nunknown_extra_field = true\n",
        String::from_utf8_lossy(&bytes)
    );
    assert!(parse_adr_index(with_unknown.as_bytes()).is_err());
}

#[test]
fn adr_index_malformed_top_level_returns_err_without_fields_drop_panic() {
    let root = workspace_root();
    let base =
        fs::read_to_string(root.join(ADR_INDEX_PATH)).expect("read registries/adr/tether.toml");

    // 1. Wrong schema header must return Err without panicking in Fields::drop.
    let bad_schema = base.replace(
        r#"schema = "frob.adr-index/1""#,
        r#"schema = "frob.adr-index/99""#,
    );
    assert_ne!(
        bad_schema, base,
        "fixture must contain expected schema line"
    );
    let err_schema = parse_adr_index(bad_schema.as_bytes())
        .expect_err("wrong schema must return Err without Fields drop panic");
    assert!(
        err_schema.contains("invalid schema") && err_schema.contains("frob.adr-index/99"),
        "unexpected schema error: {err_schema}"
    );

    // 2. Out-of-range and malformed current_milestone must return Err without Fields::drop panic.
    for (bad_val, needle) in [("M9", "out of range"), ("bad-ms", "invalid milestone")] {
        let bad_ms = base.replace(
            r#"current_milestone = "M0""#,
            &format!(r#"current_milestone = "{bad_val}""#),
        );
        assert_ne!(bad_ms, base, "fixture must contain current_milestone = M0");
        let err_ms = parse_adr_index(bad_ms.as_bytes())
            .expect_err("invalid current_milestone must return Err without Fields drop panic");
        assert!(
            err_ms.contains(needle) && err_ms.contains(bad_val),
            "unexpected milestone error for `{bad_val}`: {err_ms}"
        );
    }
}

#[test]
fn adr_enforced_by_resolves() {
    let root = workspace_root();
    let bytes = fs::read(root.join(ADR_INDEX_PATH)).expect("read registries/adr/tether.toml");
    let index = parse_adr_index(&bytes).expect("parse registries/adr/tether.toml");
    validate_enforced_by(&index, &root).expect("all enforced_by entries must resolve cleanly");

    // Negative control 1: deleting enforced_by on an accepted record fails naming that record.
    let mut empty_enforced = index.clone();
    empty_enforced.records[1].enforced_by.clear();
    let err_empty = validate_enforced_by(&empty_enforced, &root).unwrap_err();
    assert!(
        err_empty.contains("ADR-TETHER-02"),
        "expected empty enforced_by error naming ADR-TETHER-02, got: {err_empty}"
    );

    // Negative control 2: active check naming a missing test fails naming record and check.
    let mut missing_test = index.clone();
    missing_test.records[1].enforced_by[0].id =
        String::from("frob-verbs::conformance::does_not_exist_planted");
    let err_missing = validate_enforced_by(&missing_test, &root).unwrap_err();
    assert!(
        err_missing.contains("ADR-TETHER-02") && err_missing.contains("does_not_exist_planted"),
        "expected missing test error, got: {err_missing}"
    );

    // Negative control 3: accepted record with check pending beyond its first_milestone fails.
    let mut past_due = index.clone();
    past_due.records[2].enforced_by[0].milestone = String::from("M2"); // ADR-TETHER-03 is M0
    let err_past = validate_enforced_by(&past_due, &root).unwrap_err();
    assert!(
        err_past.contains("ADR-TETHER-03") && err_past.contains("beyond its milestone"),
        "expected pending-beyond-milestone error naming ADR-TETHER-03, got: {err_past}"
    );
}

#[test]
fn principle_lints_registered() {
    let root = workspace_root();
    let bytes = fs::read(root.join(ADR_INDEX_PATH)).expect("read registries/adr/tether.toml");
    let index = parse_adr_index(&bytes).expect("parse registries/adr/tether.toml");
    assert_eq!(index.principle_lints.len(), 4);
    validate_principle_lints(&index, &root).expect("all 4 principle lints must be registered");

    // Negative control: removing one principle lint fails naming it.
    let mut dropped = index.clone();
    dropped
        .principle_lints
        .retain(|l| l.id != "principle-five-explicits");
    let err = validate_principle_lints(&dropped, &root).unwrap_err();
    assert!(
        err.contains("principle-five-explicits"),
        "expected missing principle lint error, got: {err}"
    );
}

fn strip_toml_comment(line: &str) -> &str {
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
        } else if ch == '#' {
            return &line[..idx];
        }
    }
    line
}

fn toml_bracket_delta(line: &str) -> i32 {
    let (mut in_double, mut in_single, mut escaped) = (false, false, false);
    let mut delta = 0i32;
    for ch in line.chars() {
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
                '[' | '{' => delta += 1,
                ']' | '}' => delta -= 1,
                _ => {}
            }
        }
    }
    delta
}

/// Read `[package].name` from a standard Cargo manifest (`Cargo.toml`).
///
/// Cargo manifests may use standard TOML dotted keys (e.g. `version.workspace = true`),
/// which sit outside `frob_core::registry::toml`'s strict registry subset.
fn read_cargo_package_name(text: &str) -> Result<Option<String>, String> {
    let mut in_package = false;
    let mut bracket_depth = 0i32;
    let mut found: Option<String> = None;

    for raw_line in text.lines() {
        let trimmed = strip_toml_comment(raw_line).trim();
        if trimmed.is_empty() {
            continue;
        }
        if bracket_depth <= 0 && trimmed.starts_with('[') {
            in_package = trimmed
                .strip_prefix('[')
                .and_then(|s| s.strip_suffix(']'))
                .is_some_and(|inner| inner.trim() == "package");
            continue;
        }
        if in_package
            && bracket_depth <= 0
            && let Some((lhs, _)) = trimmed.split_once('=')
            && lhs.trim() == "name"
        {
            if found.is_some() {
                return Err("duplicate `[package].name` key".to_owned());
            }
            let tbl = toml::parse(trimmed.as_bytes()).map_err(|e| e.to_string())?;
            let name = tbl
                .get("name")
                .and_then(Value::as_str)
                .ok_or_else(|| "`[package].name` must be a string".to_owned())?;
            found = Some(name.to_owned());
        }
        bracket_depth += toml_bracket_delta(trimmed);
    }

    Ok(found)
}

#[test]
fn ownership_map_matches_crate_map() {
    let root = workspace_root();
    let ownership =
        fs::read_to_string(root.join("docs/adr/TETHER-OWNERSHIP.md")).expect("read ownership map");
    let rows = parse_ownership_table_rows(&ownership);
    assert_eq!(rows.len(), 7);

    let crate_map_src =
        fs::read_to_string(root.join("crates/xtask/src/crate_map.rs")).expect("read crate_map.rs");

    // Each crate named in TETHER-OWNERSHIP.md must exist in crate_map.rs; if its Cargo.toml
    // has already landed in the workspace, validate its package name strictly.
    let owned_crates: &[(&str, &str, &str)] = &[
        ("frob-verbs", "crates/frob-verbs/Cargo.toml", "N32a-02"),
        ("frob-cli", "crates/frob-cli/Cargo.toml", "BD-022"),
        ("frob-mcp", "surface/frob-mcp/Cargo.toml", "BD-023"),
        ("frob-tui", "crates/frob-tui/Cargo.toml", "BD-300"),
    ];

    let mut landed_checked = 0usize;
    let mut skipped_not_yet_landed: Vec<(&str, &str)> = Vec::new();
    for &(crate_name, manifest_rel, owner_bead) in owned_crates {
        assert!(
            ownership.contains(crate_name),
            "TETHER-OWNERSHIP.md must name `{crate_name}`"
        );
        assert!(
            crate_map_src.contains(&format!("\"{crate_name}\"")),
            "crate_map.rs must declare `{crate_name}`"
        );
        let manifest_path = root.join(manifest_rel);
        if manifest_path.is_file() {
            let manifest_text = fs::read_to_string(&manifest_path)
                .unwrap_or_else(|e| panic!("failed to read `{manifest_rel}`: {e}"));
            let pkg_name = read_cargo_package_name(&manifest_text).unwrap_or_else(|e| {
                panic!("failed to read `[package].name` in `{manifest_rel}`: {e}")
            });
            assert_eq!(
                pkg_name.as_deref(),
                Some(crate_name),
                "manifest `{manifest_rel}` package.name must equal `{crate_name}`"
            );
            landed_checked += 1;
        } else {
            skipped_not_yet_landed.push((crate_name, owner_bead));
        }
    }

    assert!(
        landed_checked >= 1,
        "expected at least frob-verbs to be landed and verified; skipped: {skipped_not_yet_landed:?}"
    );
}

#[test]
fn adr_index_digest_deterministic() {
    let root = workspace_root();
    let bytes = fs::read(root.join(ADR_INDEX_PATH)).expect("read registries/adr/tether.toml");
    let index1 = parse_adr_index(&bytes).expect("parse 1");
    let index2 = parse_adr_index(&bytes).expect("parse 2");

    let d1 = index_canonical_digest(&index1);
    let d2 = index_canonical_digest(&index2);
    assert_eq!(
        d1, d2,
        "ADR index canonical digest must be D0 deterministic"
    );

    // Mutating one field changes the digest.
    let mut mutated = index1.clone();
    mutated.records[0].bead = String::from("N32a-99");
    let d3 = index_canonical_digest(&mutated);
    assert_ne!(d1, d3, "digest must change when an ADR record changes");
}
