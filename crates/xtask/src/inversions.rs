//! L-10 inversion register and source scanner for `xtask check-layers` (bead N34p4-01).
//!
//! N34p4-01 defines the register row schema, ships an empty table, and implements
//! the four L-10 checks (`L-10.unregistered-inversion`, `L-10.unlisted-implementor`,
//! `L-10.wrong-home`, `L-10.inversion-without-adr`). Bead N34-06 populates the five
//! standing rows of rule L-10 and their table tests.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use crate::crate_map::Finding;
use crate::metadata::{PackageMeta, normalize_rel_path};

/// Rule identifier for a `// frob:inversion <id>` marker with no register row.
pub const RULE_UNREGISTERED_INVERSION: &str = "L-10.unregistered-inversion";
/// Rule identifier for `impl <Trait> for ...` outside the defining and allowed crates.
pub const RULE_UNLISTED_IMPLEMENTOR: &str = "L-10.unlisted-implementor";
/// Rule identifier for a registered trait defined in a crate other than its row's defining crate.
pub const RULE_WRONG_HOME: &str = "L-10.wrong-home";
/// Rule identifier for a non-standing row naming no existing ADR file.
pub const RULE_INVERSION_WITHOUT_ADR: &str = "L-10.inversion-without-adr";

/// Authority admitting an L-10 dependency inversion row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InversionAuthority {
    /// One of the five standing inversions established by rule L-10.
    Standing,
    /// Admitted by an Architecture Decision Record at this repo-relative path.
    Adr(&'static str),
    /// Neither standing nor backed by an ADR (always produces `L-10.inversion-without-adr`).
    None,
}

/// One row of the L-10 inversion register.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InversionRow {
    pub id: &'static str,
    /// Canonical trait path (e.g. `"frob_core::ports::EvidenceSink"`) or bare trait name.
    pub trait_path: &'static str,
    pub defining_crate: &'static str,
    pub allowed_implementors: &'static [&'static str],
    pub authority: InversionAuthority,
}

impl InversionRow {
    /// Final identifier segment of [`Self::trait_path`] (e.g. `"EvidenceSink"`).
    #[must_use]
    pub fn trait_name(&self) -> &'static str {
        self.trait_path.rsplit("::").next().unwrap_or(self.trait_path)
    }

    /// True when `crate_name` may implement this inversion trait (the defining crate
    /// itself or one of [`Self::allowed_implementors`]).
    #[must_use]
    pub fn is_allowed_implementor(&self, crate_name: &str) -> bool {
        crate_name == self.defining_crate || self.allowed_implementors.contains(&crate_name)
    }
}

/// Authoritative L-10 inversion register.
///
/// N34p4-01 ships the row type and an empty table; N34-06 fills the five standing rows.
pub const INVERSIONS: &[InversionRow] = &[];

/// Enforce all four L-10 rules over `members` against `rows`.
#[must_use]
pub fn check_inversions(
    repo_root: &Path,
    members: &[PackageMeta],
    rows: &[InversionRow],
) -> Vec<Finding> {
    let mut findings = Vec::new();
    let row_by_id: BTreeMap<&str, &InversionRow> = rows.iter().map(|r| (r.id, r)).collect();
    let mut rows_by_trait: BTreeMap<&str, Vec<&InversionRow>> = BTreeMap::new();
    for row in rows {
        rows_by_trait.entry(row.trait_name()).or_default().push(row);
    }

    // 1. Authority check: every non-standing row must point to an existing ADR file.
    for row in rows {
        match row.authority {
            InversionAuthority::Standing => {}
            InversionAuthority::Adr(adr_path) => {
                let trimmed = adr_path.trim();
                if trimmed.is_empty() || !repo_root.join(trimmed).is_file() {
                    findings.push(Finding {
                        rule: RULE_INVERSION_WITHOUT_ADR,
                        subject: row.id.to_owned(),
                        path: if trimmed.is_empty() {
                            "crates/xtask/src/inversions.rs".to_owned()
                        } else {
                            trimmed.to_owned()
                        },
                        line: None,
                        message: format!(
                            "inversion `{}` cites ADR `{}` which does not exist on disk",
                            row.id, adr_path
                        ),
                        repair: "add the ADR file under docs/adr/ or remove the unregistered inversion row"
                            .to_owned(),
                    });
                }
            }
            InversionAuthority::None => {
                findings.push(Finding {
                    rule: RULE_INVERSION_WITHOUT_ADR,
                    subject: row.id.to_owned(),
                    path: "crates/xtask/src/inversions.rs".to_owned(),
                    line: None,
                    message: format!(
                        "inversion `{}` is neither standing nor backed by an ADR path",
                        row.id
                    ),
                    repair: "set authority to InversionAuthority::Adr(\"docs/adr/...\") with an existing ADR file"
                        .to_owned(),
                });
            }
        }
    }

    // 2. Scan Rust source files of every workspace member crate.
    for member in members {
        let crate_dir = member.crate_dir();
        let rs_files = collect_crate_rust_files(crate_dir);
        for file_path in rs_files {
            let Ok(src) = fs::read_to_string(&file_path) else {
                continue;
            };
            let rel_file = file_path
                .strip_prefix(crate_dir)
                .ok()
                .map(|r| {
                    let sub = normalize_rel_path(r);
                    if member.rel_dir.is_empty() {
                        sub
                    } else {
                        format!("{}/{sub}", member.rel_dir)
                    }
                })
                .unwrap_or_else(|| normalize_rel_path(&file_path));

            scan_rust_source(
                &member.name,
                &rel_file,
                &src,
                &row_by_id,
                &rows_by_trait,
                &mut findings,
            );
        }
    }

    findings.sort();
    findings.dedup();
    findings
}

/// Collect `*.rs` files belonging to one crate, excluding `target/`, `tests/fixtures/`,
/// and any nested subdirectory that holds its own `Cargo.toml` (e.g. fixture workspaces).
fn collect_crate_rust_files(crate_dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    walk_crate_dir(crate_dir, crate_dir, &mut out);
    out.sort();
    out
}

fn walk_crate_dir(crate_root: &Path, dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    let mut paths: Vec<PathBuf> = entries.filter_map(|e| e.ok().map(|x| x.path())).collect();
    paths.sort();
    for path in paths {
        let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
            continue;
        };
        if path.is_dir() {
            if name == "target" || name == ".git" {
                continue;
            }
            if dir != crate_root && path.join("Cargo.toml").is_file() {
                continue;
            }
            if name == "fixtures" && dir.file_name().and_then(|n| n.to_str()) == Some("tests") {
                continue;
            }
            // Skip nested sub-crates directly under crate_root if they hold their own Cargo.toml.
            if dir == crate_root && path.join("Cargo.toml").is_file() {
                continue;
            }
            walk_crate_dir(crate_root, &path, out);
        } else if path.extension().and_then(|e| e.to_str()) == Some("rs") {
            out.push(path);
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum TokKind {
    Ident(String),
    Punct(char),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Token {
    kind: TokKind,
    line: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct MarkerHit {
    id: String,
    line: u32,
}

fn scan_rust_source(
    crate_name: &str,
    rel_file: &str,
    src: &str,
    row_by_id: &BTreeMap<&str, &InversionRow>,
    rows_by_trait: &BTreeMap<&str, Vec<&InversionRow>>,
    findings: &mut Vec<Finding>,
) {
    let (tokens, markers) = lex_rust(src);

    for hit in markers {
        if hit.id.is_empty() || !row_by_id.contains_key(hit.id.as_str()) {
            findings.push(Finding {
                rule: RULE_UNREGISTERED_INVERSION,
                subject: if hit.id.is_empty() {
                    crate_name.to_owned()
                } else {
                    hit.id.clone()
                },
                path: rel_file.to_owned(),
                line: Some(hit.line),
                message: format!(
                    "marker `// frob:inversion {}` in `{crate_name}` has no row in crates/xtask/src/inversions.rs",
                    hit.id
                ),
                repair: "register the inversion with an ADR in crates/xtask/src/inversions.rs or remove the marker"
                    .to_owned(),
            });
        }
    }

    if rows_by_trait.is_empty() {
        return;
    }

    // Track already-reported (rule, trait_name, line) per file for cleanliness.
    let mut reported: BTreeSet<(&'static str, String, u32)> = BTreeSet::new();

    for i in 0..tokens.len() {
        match &tokens[i].kind {
            TokKind::Ident(word) if word == "trait" => {
                if is_trait_definition(&tokens, i)
                    && let Some(TokKind::Ident(trait_name)) = tokens.get(i + 1).map(|t| &t.kind)
                    && let Some(matching_rows) = rows_by_trait.get(trait_name.as_str())
                {
                    let line = tokens[i].line;
                    for row in matching_rows {
                        if crate_name != row.defining_crate
                            && reported.insert((RULE_WRONG_HOME, row.id.to_owned(), line))
                        {
                            findings.push(Finding {
                                rule: RULE_WRONG_HOME,
                                subject: format!("{crate_name}::{}", row.trait_name()),
                                path: rel_file.to_owned(),
                                line: Some(line),
                                message: format!(
                                    "registered inversion trait `{}` (row `{}`) is defined in `{crate_name}`, expected `{}`",
                                    row.trait_name(),
                                    row.id,
                                    row.defining_crate
                                ),
                                repair: format!(
                                    "define `{}` in `{}` or update the register row in crates/xtask/src/inversions.rs",
                                    row.trait_name(),
                                    row.defining_crate
                                ),
                            });
                        }
                    }
                }
            }
            TokKind::Ident(word) if word == "impl" => {
                if let Some(trait_ident) = parse_impl_trait_name(&tokens, i)
                    && let Some(matching_rows) = rows_by_trait.get(trait_ident.as_str())
                {
                    let line = tokens[i].line;
                    for row in matching_rows {
                        if !row.is_allowed_implementor(crate_name)
                            && reported.insert((RULE_UNLISTED_IMPLEMENTOR, row.id.to_owned(), line))
                        {
                            findings.push(Finding {
                                rule: RULE_UNLISTED_IMPLEMENTOR,
                                subject: format!("{crate_name} -> {}", row.id),
                                path: rel_file.to_owned(),
                                line: Some(line),
                                message: format!(
                                    "crate `{crate_name}` implements registered inversion trait `{}` (row `{}`), allowed only in `{}` and {:?}",
                                    row.trait_name(),
                                    row.id,
                                    row.defining_crate,
                                    row.allowed_implementors
                                ),
                                repair: format!(
                                    "move the implementation into an allowed crate or add `{crate_name}` to `{}` in crates/xtask/src/inversions.rs with an ADR",
                                    row.id
                                ),
                            });
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

fn is_trait_definition(tokens: &[Token], idx: usize) -> bool {
    // Reject `dyn trait` / `impl trait` / field names after `.` or `:`.
    if idx > 0 {
        match &tokens[idx - 1].kind {
            TokKind::Ident(prev) if matches!(prev.as_str(), "dyn" | "impl" | "use" | "as") => {
                return false;
            }
            TokKind::Punct('.' | ':') => return false,
            _ => {}
        }
    }
    // Must be followed by `<Ident>` and then one of `{`, `<`, `:`, `;`, `=`, `where`.
    let Some(TokKind::Ident(_)) = tokens.get(idx + 1).map(|t| &t.kind) else {
        return false;
    };
    match tokens.get(idx + 2).map(|t| &t.kind) {
        Some(TokKind::Punct('{' | '<' | ':' | ';' | '=')) => true,
        Some(TokKind::Ident(w)) if w == "where" => true,
        _ => false,
    }
}

/// If `tokens[impl_idx]` starts a trait impl header `impl [<...>] [const] [!] <TraitPath> for ...`,
/// return the final identifier segment of `<TraitPath>`. Returns `None` for inherent impls and
/// `impl Trait` types (which have no top-level `for` before `{`, `;`, `,`, `)`, or `where`).
fn parse_impl_trait_name(tokens: &[Token], impl_idx: usize) -> Option<String> {
    let mut j = impl_idx + 1;

    // Skip optional `impl<...>` generic parameter list.
    if matches!(tokens.get(j).map(|t| &t.kind), Some(TokKind::Punct('<'))) {
        j = skip_angle_group(tokens, j)?;
    }

    // Skip optional higher-ranked trait bound `for<...>` (e.g. `impl for<'a> Trait for Type`).
    if matches!(tokens.get(j).map(|t| &t.kind), Some(TokKind::Ident(w)) if w == "for")
        && matches!(tokens.get(j + 1).map(|t| &t.kind), Some(TokKind::Punct('<')))
    {
        j = skip_angle_group(tokens, j + 1)?;
    }

    // Skip optional `const` / `!` prefix before trait path.
    while let Some(tok) = tokens.get(j) {
        match &tok.kind {
            TokKind::Ident(w) if w == "const" => j += 1,
            TokKind::Punct('!') => j += 1,
            _ => break,
        }
    }

    // Scan trait path up to top-level `for`, tracking angle/paren/bracket nesting.
    let mut angle_depth: i32 = 0;
    let mut paren_depth: i32 = 0;
    let mut bracket_depth: i32 = 0;
    let mut last_top_ident: Option<&str> = None;

    while let Some(tok) = tokens.get(j) {
        let at_top = angle_depth == 0 && paren_depth == 0 && bracket_depth == 0;
        match &tok.kind {
            TokKind::Ident(w) if at_top && w == "for" => {
                return last_top_ident.map(str::to_owned);
            }
            TokKind::Ident(w) if at_top && w == "where" => return None,
            TokKind::Punct('{' | ';') if at_top => return None,
            TokKind::Punct(',' | ')') if at_top => return None,
            TokKind::Punct('<') => angle_depth += 1,
            TokKind::Punct('>') => {
                // Do not decrement on `->` return-type arrow.
                let is_arrow = j > 0 && matches!(tokens[j - 1].kind, TokKind::Punct('-'));
                if !is_arrow && angle_depth > 0 {
                    angle_depth -= 1;
                }
            }
            TokKind::Punct('(') => paren_depth += 1,
            TokKind::Punct(')') if paren_depth > 0 => paren_depth -= 1,
            TokKind::Punct('[') => bracket_depth += 1,
            TokKind::Punct(']') if bracket_depth > 0 => bracket_depth -= 1,
            TokKind::Ident(w) if at_top => {
                last_top_ident = Some(w.as_str());
            }
            _ => {}
        }
        j += 1;
    }
    None
}

fn skip_angle_group(tokens: &[Token], start: usize) -> Option<usize> {
    let mut depth = 0i32;
    let mut i = start;
    while let Some(tok) = tokens.get(i) {
        match tok.kind {
            TokKind::Punct('<') => depth += 1,
            TokKind::Punct('>') => {
                let is_arrow = i > 0 && matches!(tokens[i - 1].kind, TokKind::Punct('-'));
                if !is_arrow {
                    depth -= 1;
                    if depth == 0 {
                        return Some(i + 1);
                    }
                }
            }
            TokKind::Punct('{' | ';') => return None,
            _ => {}
        }
        i += 1;
    }
    None
}

/// Minimal Rust lexer that strips comments, strings, raw strings, and char literals
/// while capturing `// frob:inversion <id>` marker lines and accurate 1-based line numbers.
fn lex_rust(src: &str) -> (Vec<Token>, Vec<MarkerHit>) {
    let bytes = src.as_bytes();
    let len = bytes.len();
    let mut i = 0usize;
    let mut line = 1u32;
    let mut only_ws_on_line = true;
    let mut tokens = Vec::new();
    let mut markers = Vec::new();

    while i < len {
        let b = bytes[i];

        if b == b'\n' {
            line = line.saturating_add(1);
            only_ws_on_line = true;
            i += 1;
            continue;
        }
        if matches!(b, b' ' | b'\t' | b'\r') {
            i += 1;
            continue;
        }

        // Line comment: // ...
        if b == b'/' && bytes.get(i + 1) == Some(&b'/') {
            let comment_line = line;
            let is_doc = matches!(bytes.get(i + 2), Some(b'/' | b'!'));
            let start = i + 2;
            i += 2;
            while i < len && bytes[i] != b'\n' {
                i += 1;
            }
            if only_ws_on_line && !is_doc {
                let body = src[start..i].trim();
                if let Some(rest) = body.strip_prefix("frob:inversion")
                    && (rest.is_empty() || rest.starts_with(char::is_whitespace))
                {
                    let id = rest.split_whitespace().next().unwrap_or("").to_owned();
                    markers.push(MarkerHit {
                        id,
                        line: comment_line,
                    });
                }
            }
            continue;
        }

        // Nested block comment: /* ... */
        if b == b'/' && bytes.get(i + 1) == Some(&b'*') {
            let mut depth = 1usize;
            i += 2;
            while i < len && depth > 0 {
                if bytes[i] == b'\n' {
                    line = line.saturating_add(1);
                    only_ws_on_line = true;
                    i += 1;
                } else if bytes[i] == b'/' && bytes.get(i + 1) == Some(&b'*') {
                    depth += 1;
                    i += 2;
                } else if bytes[i] == b'*' && bytes.get(i + 1) == Some(&b'/') {
                    depth -= 1;
                    i += 2;
                } else {
                    i += 1;
                }
            }
            continue;
        }

        only_ws_on_line = false;

        // Raw string literals: r"...", r#"..."#, br#"..."#, cr#"..."#
        if let Some(next_i) = try_skip_raw_string(bytes, i, &mut line) {
            i = next_i;
            continue;
        }

        // Byte/C normal string literals: b"...", c"..."
        if matches!(b, b'b' | b'c') && bytes.get(i + 1) == Some(&b'"') {
            i = skip_quoted_string(bytes, i + 1, &mut line);
            continue;
        }

        // Normal string literal: "..."
        if b == b'"' {
            i = skip_quoted_string(bytes, i, &mut line);
            continue;
        }

        // Byte literal: b'x' or b'\n'
        if b == b'b' && bytes.get(i + 1) == Some(&b'\'') {
            if let Some(next_i) = try_skip_char_literal(bytes, i + 1, &mut line) {
                i = next_i;
                continue;
            }
        }

        // Char literal vs lifetime: 'x', '\n', '\u{...}' vs 'a / 'static
        if b == b'\'' {
            if let Some(next_i) = try_skip_char_literal(bytes, i, &mut line) {
                i = next_i;
                continue;
            }
            // Lifetime: skip leading `'` and any immediately following ident chars.
            i += 1;
            while i < len && is_ident_continue(bytes[i]) {
                i += 1;
            }
            continue;
        }

        // Raw identifier `r#ident`
        if b == b'r'
            && bytes.get(i + 1) == Some(&b'#')
            && bytes.get(i + 2).copied().is_some_and(is_ident_start)
        {
            let tok_line = line;
            let start = i + 2;
            i += 3;
            while i < len && is_ident_continue(bytes[i]) {
                i += 1;
            }
            tokens.push(Token {
                kind: TokKind::Ident(src[start..i].to_owned()),
                line: tok_line,
            });
            continue;
        }

        // Identifier
        if is_ident_start(b) {
            let tok_line = line;
            let start = i;
            i += 1;
            while i < len && is_ident_continue(bytes[i]) {
                i += 1;
            }
            tokens.push(Token {
                kind: TokKind::Ident(src[start..i].to_owned()),
                line: tok_line,
            });
            continue;
        }

        // ASCII punctuation
        if b.is_ascii_punctuation() {
            tokens.push(Token {
                kind: TokKind::Punct(b as char),
                line,
            });
            i += 1;
            continue;
        }

        // Skip digits or non-ASCII UTF-8 bytes safely.
        i += 1;
    }

    (tokens, markers)
}

const fn is_ident_start(b: u8) -> bool {
    b.is_ascii_alphabetic() || b == b'_'
}

const fn is_ident_continue(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

fn skip_quoted_string(bytes: &[u8], open_quote: usize, line: &mut u32) -> usize {
    let mut i = open_quote + 1;
    while i < bytes.len() {
        match bytes[i] {
            b'\\' => {
                if bytes.get(i + 1) == Some(&b'\n') {
                    *line = line.saturating_add(1);
                }
                i = (i + 2).min(bytes.len());
            }
            b'"' => return i + 1,
            b'\n' => {
                *line = line.saturating_add(1);
                i += 1;
            }
            _ => i += 1,
        }
    }
    bytes.len()
}

fn try_skip_raw_string(bytes: &[u8], start: usize, line: &mut u32) -> Option<usize> {
    let mut i = start;
    if matches!(bytes.get(i), Some(b'b' | b'c')) {
        i += 1;
    }
    if bytes.get(i) != Some(&b'r') {
        return None;
    }
    i += 1;
    let mut hashes = 0usize;
    while bytes.get(i) == Some(&b'#') {
        hashes += 1;
        i += 1;
    }
    if bytes.get(i) != Some(&b'"') {
        return None;
    }
    i += 1;
    while i < bytes.len() {
        if bytes[i] == b'\n' {
            *line = line.saturating_add(1);
            i += 1;
        } else if bytes[i] == b'"' {
            let mut matched = 0usize;
            while matched < hashes && bytes.get(i + 1 + matched) == Some(&b'#') {
                matched += 1;
            }
            if matched == hashes {
                return Some(i + 1 + hashes);
            }
            i += 1;
        } else {
            i += 1;
        }
    }
    Some(bytes.len())
}

fn try_skip_char_literal(bytes: &[u8], open_quote: usize, line: &mut u32) -> Option<usize> {
    let next = *bytes.get(open_quote + 1)?;
    if next == b'\\' {
        let mut i = open_quote + 2;
        if i >= bytes.len() {
            return None;
        }
        // Handle `\xNN`, `\u{...}`, or single escaped char like `\'`, `\n`.
        i += 1;
        while i < bytes.len() && bytes[i] != b'\'' && bytes[i] != b'\n' {
            i += 1;
        }
        if bytes.get(i) == Some(&b'\'') {
            return Some(i + 1);
        }
        return None;
    }
    // Non-escaped single UTF-8 scalar followed immediately by `'`.
    let rest = std::str::from_utf8(&bytes[open_quote + 1..]).ok()?;
    let ch = rest.chars().next()?;
    let after = open_quote + 1 + ch.len_utf8();
    if bytes.get(after) == Some(&b'\'') {
        if ch == '\n' {
            *line = line.saturating_add(1);
        }
        return Some(after + 1);
    }
    None
}
