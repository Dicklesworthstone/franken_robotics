#![deny(unsafe_code)]

//! Enforcing lint for FrankenRobotics unit-test conventions (`STD-TEST` §9).
//!
//! Run as `cargo test -p frob-conformance --test std_test_lint`.
//! Supports `FROB_STD_TEST_LINT_ROOT=<dir>` (for scratch workspaces in
//! `scripts/e2e/std_test_lint.sh`) and `FROB_STD_TEST_LINT_MILESTONE=M<n>` (default `M0`).
//! Never deletes files or directories (COMMON.md 9b).

use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use frob_conformance::e2e::json::{self, JsonValue};
use frob_core::testkit::registry::{
    self, ControlState, Controls, Counterexamples, KnownAnswers, Relations,
};

const FORBIDDEN_MOCK_CRATES: &[&str] = &["mockall", "mockito", "wiremock", "faux", "double"];

#[derive(Debug, Clone, PartialEq, Eq)]
struct Finding {
    rule: &'static str,
    path: String,
    line: usize,
    detail: String,
}

impl std::fmt::Display for Finding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}:{}: {}", self.rule, self.path, self.line, self.detail)
    }
}

fn rel_display(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

fn walk_files(dir: &Path, skip_dirs: &[&str], out: &mut Vec<PathBuf>) {
    let Ok(rd) = fs::read_dir(dir) else { return };
    let mut entries: Vec<PathBuf> = rd.flatten().map(|e| e.path()).collect();
    entries.sort();
    for p in entries {
        let name = p.file_name().and_then(|s| s.to_str()).unwrap_or("");
        if p.is_dir() {
            if !skip_dirs.contains(&name) {
                walk_files(&p, skip_dirs, out);
            }
        } else if p.is_file() {
            out.push(p);
        }
    }
}

// ============================================================================
// Rust source lexical helpers (comment stripping & optional string blanking)
// ============================================================================

fn strip_rust(src: &str, blank_strings: bool) -> String {
    let b = src.as_bytes();
    let n = b.len();
    let mut out = String::with_capacity(n);
    let mut i = 0usize;
    while i < n {
        // Line comment
        if i + 1 < n && b[i] == b'/' && b[i + 1] == b'/' {
            while i < n && b[i] != b'\n' { i += 1; }
            continue;
        }
        // Nested block comment
        if i + 1 < n && b[i] == b'/' && b[i + 1] == b'*' {
            let mut depth = 1usize;
            i += 2;
            while i < n && depth > 0 {
                if i + 1 < n && b[i] == b'/' && b[i + 1] == b'*' { depth += 1; i += 2; }
                else if i + 1 < n && b[i] == b'*' && b[i + 1] == b'/' { depth -= 1; i += 2; }
                else {
                    if b[i] == b'\n' { out.push('\n'); }
                    i += 1;
                }
            }
            out.push(' ');
            continue;
        }
        // Raw string r"..." / r#"..."# / br#"..."#
        let raw_start = if b[i] == b'r' {
            Some(i + 1)
        } else if i + 1 < n && b[i] == b'b' && b[i + 1] == b'r' {
            Some(i + 2)
        } else {
            None
        };
        if let Some(mut j) = raw_start {
            let prev_ident = i > 0 && (b[i - 1].is_ascii_alphanumeric() || b[i - 1] == b'_');
            if !prev_ident {
                let mut hashes = 0usize;
                while j < n && b[j] == b'#' { hashes += 1; j += 1; }
                if j < n && b[j] == b'"' {
                    let content_start = j + 1;
                    j += 1;
                    let mut closed_at = None;
                    while j < n {
                        if b[j] == b'"' {
                            let mut h = 0usize;
                            while j + 1 + h < n && h < hashes && b[j + 1 + h] == b'#' { h += 1; }
                            if h == hashes {
                                closed_at = Some((j, j + 1 + hashes));
                                break;
                            }
                        }
                        j += 1;
                    }
                    if let Some((content_end, after)) = closed_at {
                        if blank_strings {
                            out.push('"');
                            let inner = &src[content_start..content_end];
                            if !inner.is_empty() { out.push('x'); }
                            for ch in inner.bytes() { if ch == b'\n' { out.push('\n'); } }
                            out.push('"');
                        } else {
                            out.push_str(&src[i..after]);
                        }
                        i = after;
                        continue;
                    }
                }
            }
        }
        // Normal string "..."
        if b[i] == b'"' {
            let start = i;
            i += 1;
            let mut has_body = false;
            let mut newlines = 0usize;
            while i < n {
                if b[i] == b'\\' && i + 1 < n {
                    has_body = true;
                    if b[i + 1] == b'\n' { newlines += 1; }
                    i += 2;
                    continue;
                }
                if b[i] == b'"' { i += 1; break; }
                if b[i] == b'\n' { newlines += 1; }
                has_body = true;
                i += 1;
            }
            if blank_strings {
                out.push('"');
                if has_body { out.push('x'); }
                for _ in 0..newlines { out.push('\n'); }
                out.push('"');
            } else {
                out.push_str(&src[start..i]);
            }
            continue;
        }
        // Character literal '\'' (avoid confusing lifetimes like `'a`)
        if b[i] == b'\'' {
            if i + 2 < n && b[i + 1] == b'\\' {
                let mut j = i + 2;
                while j < n && j < i + 12 && b[j] != b'\'' && b[j] != b'\n' { j += 1; }
                if j < n && b[j] == b'\'' {
                    out.push_str(if blank_strings { "'x'" } else { &src[i..=j] });
                    i = j + 1;
                    continue;
                }
            } else if i + 2 < n && b[i + 2] == b'\'' && b[i + 1] != b'\n' {
                out.push_str(if blank_strings { "'x'" } else { &src[i..i + 3] });
                i += 3;
                continue;
            }
        }
        let ch = src[i..].chars().next().expect("valid utf8");
        out.push(ch);
        i += ch.len_utf8();
    }
    out
}

fn line_of_offset(s: &str, offset: usize) -> usize {
    s[..offset.min(s.len())].bytes().filter(|&b| b == b'\n').count() + 1
}

// ============================================================================
// Rule 1: No forbidden mocking crates in any Cargo.toml
// ============================================================================

fn check_forbidden_mocks(root: &Path, findings: &mut Vec<Finding>) {
    let mut files = Vec::new();
    walk_files(root, &["target", ".git"], &mut files);
    for path in files {
        if path.file_name().and_then(|s| s.to_str()) != Some("Cargo.toml") {
            continue;
        }
        let Ok(text) = fs::read_to_string(&path) else { continue };
        let rel = rel_display(root, &path);
        let mut in_dep_section = false;
        for (idx, raw_line) in text.lines().enumerate() {
            let line_no = idx + 1;
            let line = strip_toml_comment(raw_line).trim();
            if line.is_empty() { continue; }
            if line.starts_with('[') && line.ends_with(']') && !line.starts_with("[[") {
                let hdr = line[1..line.len() - 1].trim();
                let segs: Vec<&str> = hdr.split('.').map(str::trim).collect();
                let is_dep_kind = |s: &str| matches!(s, "dependencies" | "dev-dependencies" | "build-dependencies");
                in_dep_section = segs.last().is_some_and(|s| is_dep_kind(s))
                    || (segs.len() >= 2 && is_dep_kind(segs[segs.len() - 2]));
                if segs.len() >= 2 && is_dep_kind(segs[segs.len() - 2]) {
                    let dep = segs[segs.len() - 1].trim_matches('"').trim_matches('\'');
                    if FORBIDDEN_MOCK_CRATES.contains(&dep) {
                        findings.push(Finding {
                            rule: "forbidden-mock-crate",
                            path: rel.clone(),
                            line: line_no,
                            detail: format!("forbidden mocking crate `{dep}` declared in [{hdr}]"),
                        });
                    }
                }
                continue;
            }
            if !in_dep_section { continue; }
            if let Some((lhs, rhs)) = line.split_once('=') {
                let key = lhs.trim().trim_matches('"').trim_matches('\'');
                let first_key = key.split('.').next().unwrap_or(key).trim();
                if FORBIDDEN_MOCK_CRATES.contains(&first_key) {
                    findings.push(Finding {
                        rule: "forbidden-mock-crate",
                        path: rel.clone(),
                        line: line_no,
                        detail: format!("forbidden mocking crate `{first_key}` in manifest"),
                    });
                }
                for &m in FORBIDDEN_MOCK_CRATES {
                    if rhs.contains(&format!("\"{m}\"")) && rhs.contains("package") {
                        findings.push(Finding {
                            rule: "forbidden-mock-crate",
                            path: rel.clone(),
                            line: line_no,
                            detail: format!("forbidden mocking crate `{m}` aliased via package="),
                        });
                    }
                }
            }
        }
    }
}

fn strip_toml_comment(line: &str) -> &str {
    let (mut in_dq, mut in_sq, mut esc) = (false, false, false);
    for (i, b) in line.bytes().enumerate() {
        if esc { esc = false; continue; }
        if in_dq { if b == b'\\' { esc = true; } else if b == b'"' { in_dq = false; } }
        else if in_sq { if b == b'\'' { in_sq = false; } }
        else if b == b'"' { in_dq = true; }
        else if b == b'\'' { in_sq = true; }
        else if b == b'#' { return &line[..i]; }
    }
    line
}

// ============================================================================
// Rule 2: No #[ignore] without a non-empty reason string
// ============================================================================

fn check_bare_ignores(root: &Path, rs_files: &[PathBuf], findings: &mut Vec<Finding>) {
    for path in rs_files {
        let Ok(src) = fs::read_to_string(path) else { continue };
        let stripped = strip_rust(&src, true);
        let bytes = stripped.as_bytes();
        let mut i = 0usize;
        while i + 2 < bytes.len() {
            if bytes[i] == b'#' && bytes[i + 1] == b'[' {
                let start = i;
                let mut j = i + 2;
                let mut depth = 1usize;
                while j < bytes.len() && depth > 0 {
                    if bytes[j] == b'[' { depth += 1; } else if bytes[j] == b']' { depth -= 1; }
                    j += 1;
                }
                let inner = stripped[start + 2..j.saturating_sub(1)].trim();
                if inner == "ignore" {
                    findings.push(Finding {
                        rule: "ignore-without-reason",
                        path: rel_display(root, path),
                        line: line_of_offset(&stripped, start),
                        detail: "`#[ignore]` requires a non-empty reason string (`#[ignore = \"...\"]`)".to_owned(),
                    });
                } else if let Some(rest) = inner.strip_prefix("ignore") {
                    let r = rest.trim();
                    if let Some(val) = r.strip_prefix('=') {
                        let v = val.trim();
                        if v == "\"\"" || v.is_empty() {
                            findings.push(Finding {
                                rule: "ignore-without-reason",
                                path: rel_display(root, path),
                                line: line_of_offset(&stripped, start),
                                detail: "`#[ignore = \"\"]` has an empty reason string".to_owned(),
                            });
                        }
                    }
                }
                i = j;
                continue;
            }
            i += 1;
        }
    }
}

// ============================================================================
// Rules 3 & 4: Registry-bound test names & rows due <= current milestone
// ============================================================================

fn extract_fn_names(stripped: &str) -> Vec<(String, usize)> {
    let bytes = stripped.as_bytes();
    let mut out = Vec::new();
    let mut i = 0usize;
    while i + 3 < bytes.len() {
        let prev_ok = i == 0 || !(bytes[i - 1].is_ascii_alphanumeric() || bytes[i - 1] == b'_');
        if prev_ok && bytes[i] == b'f' && bytes[i + 1] == b'n' && bytes[i + 2].is_ascii_whitespace() {
            let fn_pos = i;
            let mut j = i + 2;
            while j < bytes.len() && bytes[j].is_ascii_whitespace() { j += 1; }
            let id_start = j;
            while j < bytes.len() && (bytes[j].is_ascii_alphanumeric() || bytes[j] == b'_') { j += 1; }
            let id_end = j;
            while j < bytes.len() && bytes[j].is_ascii_whitespace() { j += 1; }
            if id_end > id_start && j < bytes.len() && (bytes[j] == b'(' || bytes[j] == b'<') {
                out.push((stripped[id_start..id_end].to_owned(), line_of_offset(stripped, fn_pos)));
            }
            i = j;
            continue;
        }
        i += 1;
    }
    out
}

fn bound_id_from_fn(name: &str) -> Option<(&'static str, String)> {
    if let Some(rest) = name.strip_prefix("ka_") {
        let (digits, _) = rest.split_once('_')?;
        if !digits.is_empty() && digits.bytes().all(|b| b.is_ascii_digit()) {
            return Some(("ka", format!("KA-{digits}")));
        }
    } else if let Some(rest) = name.strip_prefix("cx_") {
        let (digits, _) = rest.split_once('_')?;
        if !digits.is_empty() && digits.bytes().all(|b| b.is_ascii_digit()) {
            return Some(("cx", format!("CX-{digits}")));
        }
    } else if let Some(rest) = name.strip_prefix("nc_") {
        let (digits, _) = rest.split_once('_')?;
        if !digits.is_empty() && digits.bytes().all(|b| b.is_ascii_digit()) {
            return Some(("nc", format!("NC-{digits}")));
        }
    } else if let Some(rest) = name.strip_prefix("mr_") {
        let (tok, _) = rest.split_once('_')?;
        if let Some(d) = tok.strip_prefix('c') {
            if !d.is_empty() && d.bytes().all(|b| b.is_ascii_digit()) {
                return Some(("mr", format!("MR-C{d}")));
            }
        } else if !tok.is_empty() && tok.bytes().all(|b| b.is_ascii_digit()) {
            return Some(("mr", format!("MR-{tok}")));
        }
    }
    None
}

fn parse_milestone(m: &str) -> Option<u32> {
    m.trim().strip_prefix('M')?.parse::<u32>().ok()
}

fn collect_runner_entries(root: &Path) -> BTreeSet<String> {
    let mut entries = BTreeSet::new();
    let dir = root.join("crates/frob-conformance/tests");
    let Ok(rd) = fs::read_dir(&dir) else { return entries };
    for p in rd.flatten().map(|e| e.path()) {
        let name = p.file_name().and_then(|s| s.to_str()).unwrap_or("");
        if !(name.starts_with('g') && name.ends_with(".rs")) { continue; }
        let Ok(src) = fs::read_to_string(&p) else { continue };
        let clean = strip_rust(&src, false);
        for tok in clean.split(|c: char| !(c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == ':')) {
            let trimmed = tok.strip_prefix("ka:").or_else(|| tok.strip_prefix("mr:")).unwrap_or(tok);
            if trimmed.starts_with("KA-") || trimmed.starts_with("CX-")
                || trimmed.starts_with("NC-") || trimmed.starts_with("MR-")
            {
                entries.insert(trimmed.to_owned());
            }
        }
    }
    entries
}

fn check_registry_bindings(
    root: &Path,
    rs_files: &[PathBuf],
    current_milestone: &str,
    findings: &mut Vec<Finding>,
) {
    let ka = match KnownAnswers::load(root) {
        Ok((v, _)) => v,
        Err(e) => { findings.push(Finding { rule: "registry-load-error", path: registry::KNOWN_ANSWERS_PATH.into(), line: 0, detail: e.to_string() }); KnownAnswers::default() }
    };
    let ctrls = match Controls::load(root) {
        Ok((v, _)) => v,
        Err(e) => { findings.push(Finding { rule: "registry-load-error", path: registry::CONTROLS_PATH.into(), line: 0, detail: e.to_string() }); Controls::default() }
    };
    let rels = match Relations::load(root) {
        Ok((v, _)) => v,
        Err(e) => { findings.push(Finding { rule: "registry-load-error", path: registry::RELATIONS_PATH.into(), line: 0, detail: e.to_string() }); Relations::default() }
    };
    let cxs = match Counterexamples::load(root) {
        Ok((v, _)) => v,
        Err(e) => { findings.push(Finding { rule: "registry-load-error", path: registry::COUNTEREXAMPLES_PATH.into(), line: 0, detail: e.to_string() }); Counterexamples::default() }
    };

    let mut bound_ids = BTreeSet::new();
    let mut all_fn_names = BTreeSet::new();
    for path in rs_files {
        let Ok(src) = fs::read_to_string(path) else { continue };
        let stripped = strip_rust(&src, true);
        let rel = rel_display(root, path);
        for (fname, line) in extract_fn_names(&stripped) {
            all_fn_names.insert(fname.clone());
            if let Some((kind, id)) = bound_id_from_fn(&fname) {
                let exists = match kind {
                    "ka" => ka.get(&id).is_some(),
                    "cx" => cxs.get(&id).is_some(),
                    "nc" => ctrls.get(&id).is_some(),
                    "mr" => rels.get(&id).is_some(),
                    _ => false,
                };
                if !exists {
                    findings.push(Finding {
                        rule: "unbound-registry-test-id",
                        path: rel.clone(),
                        line,
                        detail: format!("test `{fname}` references `{id}` which is absent from its registry"),
                    });
                }
                bound_ids.insert(id);
            }
        }
    }

    let Some(cur_m) = parse_milestone(current_milestone) else { return };
    let runner_entries = collect_runner_entries(root);
    let is_due = |due: &str| parse_milestone(due).is_some_and(|m| m <= cur_m);

    for row in ka.all() {
        if !is_due(&row.due) { continue; }
        let key = if !row.id.is_empty() { row.id.as_str() } else { row.slug.as_str() };
        let slug_fn_prefix = format!("ka_{}_", row.slug.replace('-', "_"));
        let has_slug_fn = !row.slug.is_empty() && all_fn_names.iter().any(|f| f.starts_with(&slug_fn_prefix));
        if !bound_ids.contains(key) && !runner_entries.contains(key) && !has_slug_fn {
            findings.push(Finding {
                rule: "due-row-without-test",
                path: registry::KNOWN_ANSWERS_PATH.into(),
                line: 0,
                detail: format!("known-answer `{key}` due at {} (<= {current_milestone}) has no bound test or runner entry", row.due),
            });
        }
    }
    for c in ctrls.all() {
        if c.state != ControlState::Runnable { continue; }
        if c.due.as_deref().is_some_and(is_due) && !bound_ids.contains(&c.id) && !runner_entries.contains(&c.id) {
            findings.push(Finding {
                rule: "due-row-without-test",
                path: registry::CONTROLS_PATH.into(),
                line: 0,
                detail: format!("control `{}` due at {} has no bound test or runner entry", c.id, c.due.as_deref().unwrap_or("")),
            });
        }
    }
    for r in rels.all() {
        if is_due(&r.due) && !bound_ids.contains(&r.id) && !runner_entries.contains(&r.id) {
            findings.push(Finding {
                rule: "due-row-without-test",
                path: registry::RELATIONS_PATH.into(),
                line: 0,
                detail: format!("relation `{}` due at {} has no bound test or runner entry", r.id, r.due),
            });
        }
    }
    for cx in cxs.all() {
        if cx.due.as_deref().is_some_and(is_due) && !bound_ids.contains(&cx.id) && !runner_entries.contains(&cx.id) {
            findings.push(Finding {
                rule: "due-row-without-test",
                path: registry::COUNTEREXAMPLES_PATH.into(),
                line: 0,
                detail: format!("counterexample `{}` due at {} has no bound test or runner entry", cx.id, cx.due.as_deref().unwrap_or("")),
            });
        }
    }
}

// ============================================================================
// Rule 5: Fixture files carry a kind header (`synthetic`, `oracle`, `recorded`)
// ============================================================================

fn has_valid_pin_ancestor(file: &Path, stop_at: &Path) -> bool {
    let mut cur = file.parent();
    while let Some(d) = cur {
        let pin = d.join("PIN.txt");
        if pin.is_file()
            && let Ok(txt) = fs::read_to_string(&pin)
            && txt.contains("repo:")
            && txt.contains("commit:")
        {
            return true;
        }
        if d == stop_at { break; }
        cur = d.parent();
    }
    false
}

fn has_fixture_kind_header(content: &str) -> bool {
    let head: String = content.lines().take(8).collect::<Vec<_>>().join("\n").to_ascii_lowercase();
    for kind in ["synthetic", "oracle", "recorded"] {
        if head.contains(&format!("# {kind}"))
            || head.contains(&format!("#{kind}"))
            || head.contains(&format!("// {kind}"))
            || head.contains(&format!("kind: {kind}"))
            || head.contains(&format!("kind:{kind}"))
            || head.contains(&format!("kind = \"{kind}\""))
            || head.contains(&format!("fixture: {kind}"))
            || head.contains(&format!("fixture_kind = \"{kind}\""))
            || head.contains(&format!("\"kind\":\"{kind}\""))
            || head.contains(&format!("\"kind\": \"{kind}\""))
            || head.contains(&format!("\"fixture_kind\":\"{kind}\""))
            || head.contains(&format!("\"fixture_kind\": \"{kind}\""))
        {
            return true;
        }
    }
    false
}

fn check_fixture_headers(root: &Path, findings: &mut Vec<Finding>) {
    let mut roots_to_scan = Vec::new();
    let top_fixtures = root.join("fixtures");
    if top_fixtures.is_dir() { roots_to_scan.push(top_fixtures); }
    if let Ok(rd) = fs::read_dir(root.join("crates")) {
        for entry in rd.flatten().map(|e| e.path()) {
            if !entry.is_dir() { continue; }
            for sub in ["fixtures", "tests/fixtures"] {
                let d = entry.join(sub);
                if d.is_dir() { roots_to_scan.push(d); }
            }
        }
    }
    for fdir in roots_to_scan {
        let mut files = Vec::new();
        walk_files(&fdir, &[".git", "target"], &mut files);
        for f in files {
            let fname = f.file_name().and_then(|s| s.to_str()).unwrap_or("");
            if matches!(fname, "README.md" | "LICENSE" | "PIN.txt" | ".gitkeep") { continue; }
            if has_valid_pin_ancestor(&f, &fdir) { continue; }
            let Ok(txt) = fs::read_to_string(&f) else { continue };
            if !has_fixture_kind_header(&txt) {
                findings.push(Finding {
                    rule: "fixture-missing-kind-header",
                    path: rel_display(root, &f),
                    line: 1,
                    detail: "fixture file must begin with a kind header (`synthetic`, `oracle`, or `recorded`)".to_owned(),
                });
            }
        }
    }
}

// ============================================================================
// Rule 6: Gate identifier printed with `pass` outside GateReport
// ============================================================================

fn contains_gate_id(s: &str) -> bool {
    let bytes = s.as_bytes();
    let mut i = 0usize;
    while i + 8 < bytes.len() {
        if &bytes[i..i + 5] == b"GATE-" {
            let mut j = i + 5;
            let area_start = j;
            while j < bytes.len() && (bytes[j].is_ascii_uppercase() || bytes[j].is_ascii_digit()) { j += 1; }
            if j > area_start && j + 1 < bytes.len() && bytes[j] == b'-' && bytes[j + 1].is_ascii_digit() {
                return true;
            }
        }
        i += 1;
    }
    false
}

fn contains_word_pass(s: &str) -> bool {
    let lower = s.to_ascii_lowercase();
    let bytes = lower.as_bytes();
    let mut i = 0usize;
    while i + 4 <= bytes.len() {
        if &bytes[i..i + 4] == b"pass" {
            let left_ok = i == 0 || !(bytes[i - 1].is_ascii_alphanumeric() || bytes[i - 1] == b'_');
            let right_ok = i + 4 == bytes.len() || !(bytes[i + 4].is_ascii_alphanumeric() || bytes[i + 4] == b'_');
            if left_ok && right_ok { return true; }
        }
        i += 1;
    }
    false
}

fn check_gate_pass_prints(root: &Path, rs_files: &[PathBuf], findings: &mut Vec<Finding>) {
    const MACROS: &[&str] = &["println!(", "print!(", "eprintln!(", "eprint!(", "writeln!(", "write!("];
    for path in rs_files {
        let rel = rel_display(root, path);
        if rel == "crates/frob-core/src/testkit/gate.rs" { continue; }
        let Ok(src) = fs::read_to_string(path) else { continue };
        let clean = strip_rust(&src, false);
        let bytes = clean.as_bytes();
        let mut i = 0usize;
        while i < bytes.len() {
            // Skip top-level string literals so macro keywords inside strings are ignored.
            if bytes[i] == b'"' {
                i += 1;
                while i < bytes.len() {
                    if bytes[i] == b'\\' && i + 1 < bytes.len() { i += 2; continue; }
                    if bytes[i] == b'"' { i += 1; break; }
                    i += 1;
                }
                continue;
            }
            let mut matched_len = None;
            let prev_ok = i == 0 || !(bytes[i - 1].is_ascii_alphanumeric() || bytes[i - 1] == b'_');
            if prev_ok {
                for &mac in MACROS {
                    if clean[i..].starts_with(mac) {
                        matched_len = Some(mac.len());
                        break;
                    }
                }
            }
            if let Some(mlen) = matched_len {
                let open = i + mlen - 1;
                let mut j = open + 1;
                let mut depth = 1usize;
                let mut in_str = false;
                let mut esc = false;
                while j < bytes.len() && depth > 0 {
                    if in_str {
                        if esc { esc = false; } else if bytes[j] == b'\\' { esc = true; } else if bytes[j] == b'"' { in_str = false; }
                    } else if bytes[j] == b'"' {
                        in_str = true;
                    } else if bytes[j] == b'(' {
                        depth += 1;
                    } else if bytes[j] == b')' {
                        depth -= 1;
                    }
                    j += 1;
                }
                let call_body = &clean[open + 1..j.saturating_sub(1)];
                if contains_gate_id(call_body) && contains_word_pass(call_body) {
                    findings.push(Finding {
                        rule: "gate-pass-outside-gate-report",
                        path: rel.clone(),
                        line: line_of_offset(&clean, i),
                        detail: "gate identifier printed with `pass` outside `GateReport`".to_owned(),
                    });
                }
                i = j;
                continue;
            }
            i += 1;
        }
    }
}

// ============================================================================
// Rule 7: No second physical copy of any registry
// ============================================================================

fn check_duplicate_registries(root: &Path, findings: &mut Vec<Finding>) {
    let canonical: BTreeSet<&str> = [
        registry::KNOWN_ANSWERS_PATH,
        registry::CONTROLS_PATH,
        registry::RELATIONS_PATH,
        registry::COUNTEREXAMPLES_PATH,
    ]
    .into_iter()
    .collect();

    let mut files = Vec::new();
    walk_files(root, &["target", ".git", "planning", "docs", "toml-test"], &mut files);
    for p in files {
        let rel = rel_display(root, &p);
        if !rel.ends_with(".toml") || canonical.contains(rel.as_str()) { continue; }
        let fname = p.file_name().and_then(|s| s.to_str()).unwrap_or("");
        let parent_dir = p.parent().and_then(|d| d.file_name()).and_then(|s| s.to_str()).unwrap_or("");
        let is_dup_name = fname.starts_with("known_answers")
            || fname.starts_with("known-answers")
            || fname.starts_with("negative_controls")
            || fname.starts_with("negative-controls")
            || fname == "controls.toml"
            || fname == "relations.toml"
            || fname == "counterexamples.toml";
        let is_dup_dir = matches!(
            parent_dir,
            "known_answers" | "known-answers" | "negative_controls" | "negative-controls" | "controls" | "relations" | "counterexamples"
        );
        if is_dup_name || is_dup_dir {
            findings.push(Finding {
                rule: "duplicate-registry-copy",
                path: rel,
                line: 0,
                detail: "second physical registry copy outside `crates/frob-conformance/registry/`".to_owned(),
            });
        }
    }
}

// ============================================================================
// Rule 8: Evidence line kind=gate status=pass requires caught control line
// ============================================================================

fn check_evidence_streams(root: &Path, findings: &mut Vec<Finding>) {
    let mut dirs = Vec::new();
    for rel in ["target/test-evidence", "evidence"] {
        let d = root.join(rel);
        if d.is_dir() { dirs.push(d); }
    }
    for dir in dirs {
        let mut files = Vec::new();
        walk_files(&dir, &[".git"], &mut files);
        for f in files {
            if f.extension().and_then(|s| s.to_str()) != Some("ndjson") { continue; }
            let Ok(txt) = fs::read_to_string(&f) else { continue };
            let rel = rel_display(root, &f);
            let mut caught_gates = BTreeSet::new();
            let mut caught_tests = BTreeSet::new();
            let mut pass_gates: Vec<(usize, String, String)> = Vec::new();

            for (idx, line) in txt.lines().enumerate() {
                if line.trim().is_empty() { continue; }
                let Ok(val) = json::parse_strict(line) else { continue };
                if val.get("schema").and_then(JsonValue::as_str) != Some("frob.testevidence/1") { continue; }
                let kind = val.get("kind").and_then(JsonValue::as_str).unwrap_or("");
                let status = val.get("status").and_then(JsonValue::as_str).unwrap_or("");
                let id = val.get("id").and_then(JsonValue::as_str).unwrap_or("");
                let test = val.get("test").and_then(JsonValue::as_str).unwrap_or("");
                let expected = val.get("expected").and_then(JsonValue::as_str).unwrap_or("");
                if kind == "nc" && status == "caught" {
                    if !expected.is_empty() { caught_gates.insert(expected.to_owned()); }
                    if !test.is_empty() { caught_tests.insert(test.to_owned()); }
                } else if kind == "gate" && status == "pass" {
                    pass_gates.push((idx + 1, id.to_owned(), test.to_owned()));
                }
            }
            for (line_no, gate_id, test_name) in pass_gates {
                let matched = caught_gates.contains(&gate_id)
                    || (!test_name.is_empty() && caught_tests.contains(&test_name));
                if !matched {
                    findings.push(Finding {
                        rule: "gate-pass-evidence-without-caught",
                        path: rel.clone(),
                        line: line_no,
                        detail: format!("evidence line reports `kind: gate, status: pass` for `{gate_id}` without a matching `caught` control line"),
                    });
                }
            }
        }
    }
}

fn run_lint(root: &Path, current_milestone: &str) -> Vec<Finding> {
    let mut findings = Vec::new();
    let mut rs_files = Vec::new();
    let crates_dir = root.join("crates");
    if crates_dir.is_dir() {
        walk_files(&crates_dir, &["target", ".git"], &mut rs_files);
        rs_files.retain(|p| p.extension().and_then(|s| s.to_str()) == Some("rs"));
    }

    check_forbidden_mocks(root, &mut findings);
    check_bare_ignores(root, &rs_files, &mut findings);
    check_registry_bindings(root, &rs_files, current_milestone, &mut findings);
    check_fixture_headers(root, &mut findings);
    check_gate_pass_prints(root, &rs_files, &mut findings);
    check_duplicate_registries(root, &mut findings);
    check_evidence_streams(root, &mut findings);
    findings
}

// ============================================================================
// Tests: clean workspace + all 8 planted violations (STD-TEST §9)
// ============================================================================

static SCRATCH_SEQ: AtomicU64 = AtomicU64::new(0);

fn scratch_ws(label: &str) -> PathBuf {
    let base = env::var_os("BIG_SCRATCH")
        .or_else(|| env::var_os("TMPDIR"))
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("/tmp"));
    let ts = SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_secs());
    let n = SCRATCH_SEQ.fetch_add(1, Ordering::Relaxed);
    let dir = base
        .join("frob-BronzePelican")
        .join(format!("std-test-lint-{label}-{ts}-{}-{n}", std::process::id()));
    fs::create_dir_all(dir.join("crates/demo/src")).expect("create scratch ws");
    fs::write(dir.join("franken-stack.lock"), "schema = \"frob.stack/1\"\n").expect("write lock");
    fs::write(dir.join("Cargo.toml"), "[workspace]\nmembers = [\"crates/*\"]\n").expect("write ws Cargo.toml");
    fs::write(
        dir.join("crates/demo/Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.0.1\"\nedition = \"2024\"\n",
    )
    .expect("write crate Cargo.toml");
    fs::write(dir.join("crates/demo/src/lib.rs"), "pub fn ok() -> u64 { 1 }\n").expect("write lib.rs");
    dir
}

#[test]
fn lint_workspace_clean() {
    let root = env::var_os("FROB_STD_TEST_LINT_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|| registry::root().expect("workspace root"));
    let milestone = env::var("FROB_STD_TEST_LINT_MILESTONE").unwrap_or_else(|_| "M0".to_owned());
    let findings = run_lint(&root, &milestone);
    assert!(
        findings.is_empty(),
        "std_test_lint found {} violation(s) in {}:\n{}",
        findings.len(),
        root.display(),
        findings.iter().map(ToString::to_string).collect::<Vec<_>>().join("\n")
    );
}

#[test]
fn lint_catches_forbidden_mock_crate() {
    let ws = scratch_ws("mock");
    fs::write(
        ws.join("crates/demo/Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.0.1\"\nedition = \"2024\"\n[dev-dependencies]\nmockall = \"0.13\"\n",
    )
    .expect("write");
    let findings = run_lint(&ws, "M0");
    assert!(findings.iter().any(|f| f.rule == "forbidden-mock-crate"), "expected forbidden-mock-crate, got {findings:?}");
}

#[test]
fn lint_catches_ignore_without_reason() {
    let ws = scratch_ws("ignore");
    fs::create_dir_all(ws.join("crates/demo/tests")).expect("mkdir");
    fs::write(ws.join("crates/demo/tests/t.rs"), "#[test]\n#[ignore]\nfn bare() {}\n").expect("write");
    let findings = run_lint(&ws, "M0");
    assert!(findings.iter().any(|f| f.rule == "ignore-without-reason"), "expected ignore-without-reason, got {findings:?}");
}

#[test]
fn lint_catches_unbound_registry_test_id() {
    let ws = scratch_ws("unbound");
    fs::create_dir_all(ws.join("crates/demo/tests")).expect("mkdir");
    fs::write(ws.join("crates/demo/tests/t.rs"), "#[test]\nfn ka_999_unknown() {}\n").expect("write");
    let findings = run_lint(&ws, "M0");
    assert!(findings.iter().any(|f| f.rule == "unbound-registry-test-id"), "expected unbound-registry-test-id, got {findings:?}");
}

#[test]
fn lint_catches_due_row_without_test() {
    let ws = scratch_ws("due-row");
    fs::create_dir_all(ws.join("crates/frob-conformance/registry")).expect("mkdir");
    fs::write(
        ws.join(registry::KNOWN_ANSWERS_PATH),
        "schema = \"frob.gauntlet.known-answers/1\"\n\
         [[ka]]\nid = \"KA-001\"\nsection = \"§1\"\nfixture = \"f\"\ncase = \"c\"\n\
         truth = { kind = \"int\", value = 1 }\ntolerance = { kind = \"exact\" }\n\
         tier = \"G1\"\nsites = [\"host\"]\ncontrol = { text = \"m\", nc = [\"NC-0001\"] }\n\
         derivation = \"d.md\"\nlabel = \"DESIGN\"\ndue = \"M0\"\n",
    )
    .expect("write");
    let findings = run_lint(&ws, "M0");
    assert!(findings.iter().any(|f| f.rule == "due-row-without-test"), "expected due-row-without-test, got {findings:?}");
}

#[test]
fn lint_catches_fixture_missing_kind_header() {
    let ws = scratch_ws("fixture-header");
    fs::create_dir_all(ws.join("fixtures")).expect("mkdir");
    fs::write(ws.join("fixtures/good.toml"), "# synthetic\nvalue = 1\n").expect("write good");
    fs::write(ws.join("fixtures/bad.toml"), "value = 42\n").expect("write bad");
    let findings = run_lint(&ws, "M0");
    assert!(findings.iter().any(|f| f.rule == "fixture-missing-kind-header" && f.path.ends_with("bad.toml")), "got {findings:?}");
    assert!(!findings.iter().any(|f| f.path.ends_with("good.toml")), "good fixture must not be flagged: {findings:?}");
}

#[test]
fn lint_catches_gate_pass_outside_gate_report() {
    let ws = scratch_ws("gate-pass");
    fs::create_dir_all(ws.join("crates/demo/tests")).expect("mkdir");
    let planted = format!(
        "#[test]\nfn forge() {{\n    println!(\"{}-GAUNT-01: {}\");\n}}\n",
        "GATE", "pass"
    );
    fs::write(ws.join("crates/demo/tests/forge.rs"), planted).expect("write");
    let findings = run_lint(&ws, "M0");
    assert!(findings.iter().any(|f| f.rule == "gate-pass-outside-gate-report"), "got {findings:?}");
}

#[test]
fn lint_catches_duplicate_registry_copy() {
    let ws = scratch_ws("dup-reg");
    fs::create_dir_all(ws.join("crates/frob-conformance/registry")).expect("mkdir");
    fs::create_dir_all(ws.join("registries")).expect("mkdir");
    fs::write(ws.join(registry::KNOWN_ANSWERS_PATH), "schema = \"frob.gauntlet.known-answers/1\"\n").expect("write");
    fs::write(ws.join("registries/known_answers.toml"), "schema = \"frob.gauntlet.known-answers/1\"\n").expect("write");
    let findings = run_lint(&ws, "M0");
    assert!(findings.iter().any(|f| f.rule == "duplicate-registry-copy"), "got {findings:?}");
}

#[test]
fn lint_catches_gate_pass_evidence_without_caught_control() {
    let ws = scratch_ws("evidence-no-caught");
    fs::create_dir_all(ws.join("target/test-evidence")).expect("mkdir");
    let bad_line = "{\"schema\":\"frob.testevidence/1\",\"ts\":\"2026-11-24T00:00:00.000Z\",\"crate\":\"demo\",\"test\":\"t1\",\"kind\":\"gate\",\"id\":\"GATE-GAUNT-01\",\"status\":\"pass\",\"reason\":null,\"observed\":\"0\",\"expected\":\"0\",\"tolerance\":null,\"unit\":null,\"tier\":\"G0\",\"site\":\"host\",\"seed\":0,\"determinism\":\"D0\",\"synthetic\":true}\n";
    fs::write(ws.join("target/test-evidence/demo.ndjson"), bad_line).expect("write");
    let findings = run_lint(&ws, "M0");
    assert!(findings.iter().any(|f| f.rule == "gate-pass-evidence-without-caught"), "got {findings:?}");

    // Positive control: adding a matching caught NC line clears the finding.
    let good_ws = scratch_ws("evidence-with-caught");
    fs::create_dir_all(good_ws.join("target/test-evidence")).expect("mkdir");
    let caught_line = "{\"schema\":\"frob.testevidence/1\",\"ts\":\"2026-11-24T00:00:00.000Z\",\"crate\":\"demo\",\"test\":\"t1\",\"kind\":\"nc\",\"id\":\"NC-0412\",\"status\":\"caught\",\"reason\":null,\"observed\":null,\"expected\":\"GATE-GAUNT-01\",\"tolerance\":null,\"unit\":null,\"tier\":null,\"site\":null,\"seed\":0,\"determinism\":null,\"synthetic\":false}\n";
    fs::write(good_ws.join("target/test-evidence/demo.ndjson"), format!("{caught_line}{bad_line}")).expect("write");
    let good_findings = run_lint(&good_ws, "M0");
    assert!(good_findings.is_empty(), "evidence with caught control must pass: {good_findings:?}");
}
