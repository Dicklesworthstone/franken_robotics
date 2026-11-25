//! Wording lint and pure-std linear-time Thompson NFA pattern matcher (`N00-10` step 3).
//!
//! Enforces three rules across documents, contracts, help text, and report templates:
//! - `hyp.banned-wording`: match against a `banned` pattern of a `REFUTED`, `NARROWED`,
//!   or `ACTIVE` row (shared single pattern list with `NAB-32`).
//! - `hyp.deferred-as-lever`: match against a `banned` pattern of a `DEFERRED` row (never
//!   double-reported as `hyp.banned-wording`).
//! - `hyp.unlabelled-number`: a quantitative claim number attached to an `ACTIVE` or
//!   `NARROWED` row's claim without `HYPOTHESIS`, `TARGET`, or `FACT` label.

use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

use crate::hypotheses::schema::{HypothesisRegister, VerdictState};

pub const RULE_BANNED_WORDING: &str = "hyp.banned-wording";
pub const RULE_DEFERRED_AS_LEVER: &str = "hyp.deferred-as-lever";
pub const RULE_UNLABELLED_NUMBER: &str = "hyp.unlabelled-number";

/// Paths exempt from `hyp.banned-wording` / `hyp.deferred-as-lever` (killed-idea register
/// and the hypothesis register itself, per `N00-10` step 3).
pub const EXEMPT_PATHS: &[&str] = &[
    "registries/killed_ideas.toml",
    "docs/decisions/killed-ideas.md",
    "crates/frob-conformance/registry/hypotheses.toml",
];

/// One wording-lint finding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WordingFinding {
    pub rule: &'static str,
    pub row_id: String,
    pub pattern_index: Option<usize>,
    pub file: String,
    pub line: usize,
    pub excerpt: String,
}

impl fmt::Display for WordingFinding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.pattern_index {
            Some(idx) => write!(
                f,
                "{} [{}:pattern {}] {}:{}: {}",
                self.rule, self.row_id, idx, self.file, self.line, self.excerpt
            ),
            None => write!(
                f,
                "{} [{}] {}:{}: {}",
                self.rule, self.row_id, self.file, self.line, self.excerpt
            ),
        }
    }
}

/// Returns `true` when `rel_path` (normalized with `/`) is exempt from wording-lint rules.
#[must_use]
pub fn is_exempt_path(rel_path: &str) -> bool {
    let norm = rel_path.replace('\\', "/");
    let trimmed = norm.strip_prefix("./").unwrap_or(&norm);
    EXEMPT_PATHS
        .iter()
        .any(|ex| trimmed == *ex || trimmed.ends_with(&format!("/{ex}")))
}

/// Lint a single document's text under `rel_path` against `register`.
#[must_use]
pub fn lint_text(
    register: &HypothesisRegister,
    rel_path: &str,
    content: &str,
) -> Vec<WordingFinding> {
    if is_exempt_path(rel_path) {
        return Vec::new();
    }

    let mut compiled: Vec<(&str, VerdictState, &str, Vec<(usize, Pattern)>)> =
        Vec::with_capacity(register.hypotheses.len());
    for row in &register.hypotheses {
        let mut pats = Vec::new();
        for (idx, p) in row.banned.iter().enumerate() {
            if let Ok(c) = Pattern::compile(p) {
                pats.push((idx, c));
            }
        }
        compiled.push((row.id.as_str(), row.state, row.statement.as_str(), pats));
    }

    let active_or_narrowed: Vec<&str> = register
        .hypotheses
        .iter()
        .filter(|h| matches!(h.state, VerdictState::Active | VerdictState::Narrowed))
        .map(|h| h.id.as_str())
        .collect();

    let mut findings = Vec::new();

    for (line_idx, raw_line) in content.lines().enumerate() {
        let line_no = line_idx + 1;
        let trimmed = raw_line.trim();
        if trimmed.is_empty() {
            continue;
        }

        // 1. Banned patterns (REFUTED/NARROWED/ACTIVE -> hyp.banned-wording;
        //    DEFERRED -> hyp.deferred-as-lever, never double-reported).
        for &(row_id, state, statement, ref pats) in &compiled {
            // Exempt lines that are verbatim the row's own statement header.
            if trimmed.eq_ignore_ascii_case(statement) {
                continue;
            }
            for (pidx, pat) in pats {
                if pat.is_match(raw_line) {
                    let rule = if state == VerdictState::Deferred {
                        RULE_DEFERRED_AS_LEVER
                    } else {
                        RULE_BANNED_WORDING
                    };
                    findings.push(WordingFinding {
                        rule,
                        row_id: row_id.to_owned(),
                        pattern_index: Some(*pidx),
                        file: rel_path.to_owned(),
                        line: line_no,
                        excerpt: trimmed.to_owned(),
                    });
                    break;
                }
            }
        }

        // 2. Unlabelled numbers attached to an ACTIVE or NARROWED row's claim.
        if !has_epistemic_label(raw_line) && has_claim_quantity(raw_line) {
            for &row_id in &active_or_narrowed {
                if contains_token_boundary(raw_line, row_id) {
                    findings.push(WordingFinding {
                        rule: RULE_UNLABELLED_NUMBER,
                        row_id: row_id.to_owned(),
                        pattern_index: None,
                        file: rel_path.to_owned(),
                        line: line_no,
                        excerpt: trimmed.to_owned(),
                    });
                }
            }
        }
    }

    findings
}

/// Lint a single file on disk using `display_path` for reporting and exempt-path checks.
pub fn lint_file(
    register: &HypothesisRegister,
    path: &Path,
    display_path: &str,
) -> Result<Vec<WordingFinding>, String> {
    if is_exempt_path(display_path) {
        return Ok(Vec::new());
    }
    let content =
        fs::read_to_string(path).map_err(|e| format!("failed reading {}: {e}", path.display()))?;
    Ok(lint_text(register, display_path, &content))
}

/// Recursively lint documents, contracts, help text, and report templates under `root`.
///
/// Scans `docs/` (`.md`, `.txt`), `crates/*/CONTRACT.md`, and any top-level `.md` files
/// inside `root` (excluding `COMPREHENSIVE_PLAN_FOR_FRANKEN_ROBOTICS.md` provenance spec
/// and `EXEMPT_PATHS`).
pub fn lint_tree(
    register: &HypothesisRegister,
    root: &Path,
) -> Result<Vec<WordingFinding>, String> {
    let mut files = Vec::new();
    collect_lint_targets(root, root, &mut files)?;
    files.sort();

    let mut all = Vec::new();
    for abs in files {
        let rel = abs
            .strip_prefix(root)
            .unwrap_or(&abs)
            .to_string_lossy()
            .replace('\\', "/");
        if is_exempt_path(&rel) {
            continue;
        }
        let mut found = lint_file(register, &abs, &rel)?;
        all.append(&mut found);
    }
    Ok(all)
}

fn collect_lint_targets(root: &Path, dir: &Path, out: &mut Vec<PathBuf>) -> Result<(), String> {
    if !dir.is_dir() {
        return Ok(());
    }
    let mut entries: Vec<PathBuf> = fs::read_dir(dir)
        .map_err(|e| format!("failed reading dir {}: {e}", dir.display()))?
        .filter_map(Result::ok)
        .map(|e| e.path())
        .collect();
    entries.sort();

    for path in entries {
        let rel = path
            .strip_prefix(root)
            .unwrap_or(&path)
            .to_string_lossy()
            .replace('\\', "/");
        if path.is_dir() {
            let name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
            if matches!(
                name,
                ".git" | "target" | ".beads" | "planning" | "node_modules" | ".ntm" | ".rch-tmp"
            ) || name.starts_with(".rch-target")
            {
                continue;
            }
            if rel == "docs"
                || rel.starts_with("docs/")
                || rel == "crates"
                || (rel.starts_with("crates/") && rel.matches('/').count() == 1)
                || rel == "templates"
                || rel.starts_with("templates/")
            {
                collect_lint_targets(root, &path, out)?;
            }
        } else if path.is_file() {
            let name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
            let is_doc = (rel.starts_with("docs/") || rel.starts_with("templates/"))
                && (name.ends_with(".md") || name.ends_with(".txt"));
            let is_contract = rel.starts_with("crates/") && name == "CONTRACT.md";
            let is_scratch_doc = !rel.contains('/')
                && name.ends_with(".md")
                && name != "COMPREHENSIVE_PLAN_FOR_FRANKEN_ROBOTICS.md"
                && name != "AGENTS.md"
                && name != "CLAUDE.md"
                && name != "CHANGELOG.md";
            if is_doc || is_contract || is_scratch_doc {
                out.push(path);
            }
        }
    }
    Ok(())
}

fn has_epistemic_label(line: &str) -> bool {
    contains_word(line, "HYPOTHESIS")
        || contains_word(line, "TARGET")
        || contains_word(line, "FACT")
}

fn contains_word(haystack: &str, word: &str) -> bool {
    let bytes = haystack.as_bytes();
    let wb = word.as_bytes();
    if wb.is_empty() || bytes.len() < wb.len() {
        return false;
    }
    for i in 0..=bytes.len() - wb.len() {
        if &bytes[i..i + wb.len()] == wb {
            let left_ok = i == 0 || !bytes[i - 1].is_ascii_alphanumeric();
            let right_ok =
                i + wb.len() == bytes.len() || !bytes[i + wb.len()].is_ascii_alphanumeric();
            if left_ok && right_ok {
                return true;
            }
        }
    }
    false
}

fn contains_token_boundary(haystack: &str, token: &str) -> bool {
    let hb = haystack.as_bytes();
    let tb = token.as_bytes();
    if tb.is_empty() || hb.len() < tb.len() {
        return false;
    }
    for i in 0..=hb.len() - tb.len() {
        if &hb[i..i + tb.len()] == tb {
            let left_ok = i == 0 || !(hb[i - 1].is_ascii_alphanumeric() || hb[i - 1] == b'-');
            let right_ok = i + tb.len() == hb.len()
                || !(hb[i + tb.len()].is_ascii_alphanumeric() || hb[i + tb.len()] == b'-');
            if left_ok && right_ok {
                return true;
            }
        }
    }
    false
}

/// True when `line` contains a quantitative claim figure (multiplier `2.6x`/`2.6×`,
/// percentage `62%`, or decimal ratio/measurement like `1.96`, `2.0 GPU-seconds`, `3-4x`),
/// ignoring hypothesis IDs (`H-22b`), ISO dates (`2026-11-21`), and section numbers (`§0.8`).
fn has_claim_quantity(line: &str) -> bool {
    for raw_tok in line.split_whitespace() {
        let tok = raw_tok.trim_matches(|c: char| matches!(c, '(' | ')' | ',' | '.' | ';' | ':' | '"' | '\'' | '`'));
        if tok.is_empty()
            || tok.starts_with("H-")
            || tok.starts_with("BET-")
            || tok.starts_with("GATE-")
            || tok.starts_with("KILL-")
            || tok.starts_with("KI-")
            || tok.starts_with("KA-")
            || tok.starts_with("CX-")
            || tok.starts_with("OD-")
            || tok.starts_with('§')
        {
            continue;
        }
        // Skip YYYY-MM-DD dates.
        if tok.len() == 10
            && tok.as_bytes()[4] == b'-'
            && tok.as_bytes()[7] == b'-'
            && tok.bytes().filter(u8::is_ascii_digit).count() == 8
        {
            continue;
        }
        let has_digit = tok.bytes().any(|b| b.is_ascii_digit());
        if !has_digit {
            continue;
        }
        if tok.ends_with('%')
            || tok.ends_with('x')
            || tok.ends_with('X')
            || tok.ends_with('×')
            || tok.contains('%')
            || (tok.contains('.') && tok.bytes().next().is_some_and(|b| b.is_ascii_digit()))
        {
            return true;
        }
    }
    false
}

// ============================================================================
// Pure-std Thompson NFA regex engine (case-insensitive ASCII, polynomial O(n*m))
// Supports: literals, '.', character classes [a-z -], \s/\w/\d/\b/escaped chars,
// grouping (...), alternation a|b, quantifiers *, +, ?.
// ============================================================================

#[derive(Debug, Clone)]
enum Matcher {
    Any,
    Char(char),
    Class { negated: bool, chars: Vec<char>, ranges: Vec<(char, char)> },
    Space,
    Word,
    Digit,
}

impl Matcher {
    fn matches(&self, c: char) -> bool {
        let cl = c.to_ascii_lowercase();
        match self {
            Self::Any => c != '\n',
            Self::Char(t) => cl == *t,
            Self::Space => c.is_ascii_whitespace(),
            Self::Word => c.is_ascii_alphanumeric() || c == '_',
            Self::Digit => c.is_ascii_digit(),
            Self::Class {
                negated,
                chars,
                ranges,
            } => {
                let hit = chars.contains(&cl)
                    || ranges.iter().any(|&(lo, hi)| cl >= lo && cl <= hi);
                if *negated { !hit } else { hit }
            }
        }
    }
}

#[derive(Debug, Clone)]
enum State {
    Match,
    Atom(Matcher, usize),
    Split(usize, usize),
    WordBoundary(usize),
}

/// Compiled case-insensitive Thompson NFA pattern.
#[derive(Debug, Clone)]
pub struct Pattern {
    states: Vec<State>,
    start: usize,
}

impl Pattern {
    /// Compile a pattern string into a linear-time Thompson NFA.
    pub fn compile(src: &str) -> Result<Self, String> {
        let chars: Vec<char> = src.chars().collect();
        let mut p = Parser {
            chars: &chars,
            pos: 0,
            states: Vec::new(),
        };
        let (start, outs) = p.parse_alt()?;
        if p.pos != chars.len() {
            return Err(format!("unexpected trailing `{}` at char {}", chars[p.pos], p.pos));
        }
        let match_id = p.push(State::Match);
        p.patch(&outs, match_id);
        Ok(Self {
            states: p.states,
            start,
        })
    }

    /// Return `true` iff any substring of `haystack` matches this pattern.
    #[must_use]
    pub fn is_match(&self, haystack: &str) -> bool {
        let chars: Vec<char> = haystack.chars().collect();
        let n = self.states.len();
        for start_idx in 0..=chars.len() {
            let mut cur = vec![false; n];
            let mut next = vec![false; n];
            self.add_state(&mut cur, self.start, &chars, start_idx);
            if self.has_match(&cur) {
                return true;
            }
            for pos in start_idx..chars.len() {
                next.fill(false);
                let ch = chars[pos];
                for (sid, &active) in cur.iter().enumerate() {
                    if !active {
                        continue;
                    }
                    if let State::Atom(ref m, target) = self.states[sid]
                        && m.matches(ch)
                    {
                        self.add_state(&mut next, target, &chars, pos + 1);
                    }
                }
                std::mem::swap(&mut cur, &mut next);
                if self.has_match(&cur) {
                    return true;
                }
                if !cur.iter().any(|&a| a) {
                    break;
                }
            }
        }
        false
    }

    fn has_match(&self, active: &[bool]) -> bool {
        active
            .iter()
            .enumerate()
            .any(|(i, &a)| a && matches!(self.states[i], State::Match))
    }

    fn add_state(&self, set: &mut [bool], sid: usize, chars: &[char], pos: usize) {
        if set[sid] {
            return;
        }
        match self.states[sid] {
            State::Split(a, b) => {
                set[sid] = true;
                self.add_state(set, a, chars, pos);
                self.add_state(set, b, chars, pos);
            }
            State::WordBoundary(target) => {
                set[sid] = true;
                let prev_w = pos > 0 && is_word(chars[pos - 1]);
                let next_w = pos < chars.len() && is_word(chars[pos]);
                if prev_w != next_w {
                    self.add_state(set, target, chars, pos);
                }
            }
            State::Match | State::Atom(_, _) => {
                set[sid] = true;
            }
        }
    }
}

fn is_word(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

struct Parser<'a> {
    chars: &'a [char],
    pos: usize,
    states: Vec<State>,
}

const UNPATCHED: usize = usize::MAX;

impl Parser<'_> {
    fn push(&mut self, s: State) -> usize {
        let id = self.states.len();
        self.states.push(s);
        id
    }

    fn patch(&mut self, outs: &[(usize, bool)], target: usize) {
        for &(sid, second) in outs {
            match &mut self.states[sid] {
                State::Atom(_, t) | State::WordBoundary(t) => *t = target,
                State::Split(a, b) => {
                    if second {
                        *b = target;
                    } else {
                        *a = target;
                    }
                }
                State::Match => {}
            }
        }
    }

    fn parse_alt(&mut self) -> Result<(usize, Vec<(usize, bool)>), String> {
        let (mut start, mut outs) = self.parse_concat()?;
        while self.peek() == Some('|') {
            self.pos += 1;
            let (rhs_start, rhs_outs) = self.parse_concat()?;
            let sp = self.push(State::Split(start, rhs_start));
            start = sp;
            outs.extend(rhs_outs);
        }
        Ok((start, outs))
    }

    fn parse_concat(&mut self) -> Result<(usize, Vec<(usize, bool)>), String> {
        let mut frags: Vec<(usize, Vec<(usize, bool)>)> = Vec::new();
        while let Some(c) = self.peek() {
            if c == ')' || c == '|' {
                break;
            }
            frags.push(self.parse_quantified()?);
        }
        if frags.is_empty() {
            let e = self.push(State::Split(UNPATCHED, UNPATCHED));
            return Ok((e, vec![(e, false)]));
        }
        let mut it = frags.into_iter();
        let (start, mut outs) = it.next().expect("non-empty frags");
        for (ns, nouts) in it {
            self.patch(&outs, ns);
            outs = nouts;
        }
        Ok((start, outs))
    }

    fn parse_quantified(&mut self) -> Result<(usize, Vec<(usize, bool)>), String> {
        let (mut start, mut outs) = self.parse_atom()?;
        while let Some(q) = self.peek() {
            match q {
                '*' => {
                    self.pos += 1;
                    let sp = self.push(State::Split(start, UNPATCHED));
                    self.patch(&outs, sp);
                    start = sp;
                    outs = vec![(sp, true)];
                }
                '+' => {
                    self.pos += 1;
                    let sp = self.push(State::Split(start, UNPATCHED));
                    self.patch(&outs, sp);
                    outs = vec![(sp, true)];
                }
                '?' => {
                    self.pos += 1;
                    let sp = self.push(State::Split(start, UNPATCHED));
                    start = sp;
                    outs.push((sp, true));
                }
                _ => break,
            }
        }
        Ok((start, outs))
    }

    fn parse_atom(&mut self) -> Result<(usize, Vec<(usize, bool)>), String> {
        let Some(c) = self.peek() else {
            return Err("unexpected end of pattern".to_owned());
        };
        match c {
            '(' => {
                self.pos += 1;
                let frag = self.parse_alt()?;
                if self.peek() != Some(')') {
                    return Err("unclosed `(` in pattern".to_owned());
                }
                self.pos += 1;
                Ok(frag)
            }
            '[' => self.parse_class(),
            '.' => {
                self.pos += 1;
                let id = self.push(State::Atom(Matcher::Any, UNPATCHED));
                Ok((id, vec![(id, false)]))
            }
            '\\' => {
                self.pos += 1;
                let Some(esc) = self.peek() else {
                    return Err("dangling `\\` at end of pattern".to_owned());
                };
                self.pos += 1;
                match esc {
                    'b' => {
                        let id = self.push(State::WordBoundary(UNPATCHED));
                        Ok((id, vec![(id, false)]))
                    }
                    's' => {
                        let id = self.push(State::Atom(Matcher::Space, UNPATCHED));
                        Ok((id, vec![(id, false)]))
                    }
                    'w' => {
                        let id = self.push(State::Atom(Matcher::Word, UNPATCHED));
                        Ok((id, vec![(id, false)]))
                    }
                    'd' => {
                        let id = self.push(State::Atom(Matcher::Digit, UNPATCHED));
                        Ok((id, vec![(id, false)]))
                    }
                    lit => {
                        let id = self.push(State::Atom(
                            Matcher::Char(lit.to_ascii_lowercase()),
                            UNPATCHED,
                        ));
                        Ok((id, vec![(id, false)]))
                    }
                }
            }
            '*' | '+' | '?' => Err(format!("dangling quantifier `{c}` at char {}", self.pos)),
            lit => {
                self.pos += 1;
                let id = self.push(State::Atom(
                    Matcher::Char(lit.to_ascii_lowercase()),
                    UNPATCHED,
                ));
                Ok((id, vec![(id, false)]))
            }
        }
    }

    fn parse_class(&mut self) -> Result<(usize, Vec<(usize, bool)>), String> {
        self.pos += 1; // consume '['
        let negated = if self.peek() == Some('^') {
            self.pos += 1;
            true
        } else {
            false
        };
        let mut chars = Vec::new();
        let mut ranges = Vec::new();
        let mut first = true;
        while let Some(c) = self.peek() {
            if c == ']' && !first {
                self.pos += 1;
                let id = self.push(State::Atom(
                    Matcher::Class {
                        negated,
                        chars,
                        ranges,
                    },
                    UNPATCHED,
                ));
                return Ok((id, vec![(id, false)]));
            }
            first = false;
            self.pos += 1;
            let ch = if c == '\\' {
                let Some(e) = self.peek() else {
                    return Err("unclosed `[` after `\\`".to_owned());
                };
                self.pos += 1;
                e.to_ascii_lowercase()
            } else {
                c.to_ascii_lowercase()
            };
            if self.peek() == Some('-') && self.chars.get(self.pos + 1).copied() != Some(']') {
                self.pos += 1; // consume '-'
                let Some(hi_raw) = self.peek() else {
                    return Err("unclosed character class range".to_owned());
                };
                self.pos += 1;
                let hi = hi_raw.to_ascii_lowercase();
                if ch > hi {
                    return Err(format!("inverted character range `{ch}-{hi}`"));
                }
                ranges.push((ch, hi));
            } else {
                chars.push(ch);
            }
        }
        Err("unclosed `[` in pattern".to_owned())
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }
}
