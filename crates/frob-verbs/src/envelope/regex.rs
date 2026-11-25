//! Minimal backtracking matcher for the JSON Schema `pattern` keyword (N32a-03).
//!
//! Supports the ECMA-262 subset the envelope schemas use: literals, `.`, `^`, `$`,
//! character classes (`[a-z0-9-]`, `[^...]`), the escapes `\d \D \w \W \s \S \uXXXX` and
//! escaped punctuation, groups with alternation `( | )`, and the quantifiers `* + ? {n}
//! {n,} {n,m}` (greedy). Anything else (look-around, back-references, lazy quantifiers,
//! word boundaries) is a compile error, so a schema can never assert less than it says.
//! Matching is unanchored (JSON Schema semantics); patterns anchor with `^`/`$`.
//! One step budget covers all starting offsets, branches and repetition attempts.

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PatternError {
    pub pattern: String,
    pub reason: String,
}

impl fmt::Display for PatternError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "unsupported or malformed pattern {:?}: {}",
            self.pattern, self.reason
        )
    }
}

impl std::error::Error for PatternError {}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Node {
    Char(char),
    Any,
    Class {
        ranges: Vec<(char, char)>,
        negated: bool,
    },
    Start,
    End,
    Group(Vec<Vec<Node>>),
    Repeat {
        node: Box<Node>,
        min: u32,
        max: Option<u32>,
    },
}

/// A compiled `pattern`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pattern {
    source: String,
    alts: Vec<Vec<Node>>,
}

impl Pattern {
    /// Compile `source`; unsupported syntax is an error.
    pub fn compile(source: &str) -> Result<Self, PatternError> {
        let chars: Vec<char> = source.chars().collect();
        let mut p = Parser {
            chars: &chars,
            pos: 0,
            source,
        };
        let alts = p.alternation()?;
        if p.pos != chars.len() {
            return Err(p.err("unbalanced `)`"));
        }
        Ok(Self {
            source: source.to_owned(),
            alts,
        })
    }

    #[must_use]
    pub fn source(&self) -> &str {
        &self.source
    }

    /// Search `text`, distinguishing a non-match from exhausted search work.
    pub(super) fn is_match(&self, text: &str) -> MatchResult {
        let chars: Vec<char> = text.chars().collect();
        let mut state = MatchState {
            text: &chars,
            // Leave room for a linear scan of large values while bounding extra search work.
            remaining: 65_536usize.saturating_add(chars.len().saturating_mul(4)),
        };
        let anchored = |seq: &[Node]| matches!(seq.first(), Some(Node::Start));
        let last_start = if self.alts.iter().all(|seq| anchored(seq)) {
            0
        } else {
            chars.len()
        };
        for start in 0..=last_start {
            for seq in &self.alts {
                if (start == 0 || !anchored(seq))
                    && match_seq(seq, start, &mut state, &mut |_, _| Ok(true))?
                {
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }
}

struct Parser<'a> {
    chars: &'a [char],
    pos: usize,
    source: &'a str,
}

impl Parser<'_> {
    fn err(&self, reason: &str) -> PatternError {
        PatternError {
            pattern: self.source.to_owned(),
            reason: format!("{reason} at char {}", self.pos),
        }
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    fn alternation(&mut self) -> Result<Vec<Vec<Node>>, PatternError> {
        let mut alts = vec![self.sequence()?];
        while self.peek() == Some('|') {
            self.pos += 1;
            alts.push(self.sequence()?);
        }
        Ok(alts)
    }

    fn sequence(&mut self) -> Result<Vec<Node>, PatternError> {
        let mut seq = Vec::new();
        while let Some(c) = self.peek() {
            if c == '|' || c == ')' {
                break;
            }
            let atom = self.atom()?;
            let node = self.quantified(atom)?;
            seq.push(node);
        }
        Ok(seq)
    }

    fn atom(&mut self) -> Result<Node, PatternError> {
        let c = self.peek().ok_or_else(|| self.err("unexpected end"))?;
        self.pos += 1;
        match c {
            '(' => {
                if self.peek() == Some('?') {
                    return Err(self.err("group modifiers `(?` are not supported"));
                }
                let alts = self.alternation()?;
                if self.peek() != Some(')') {
                    return Err(self.err("missing `)`"));
                }
                self.pos += 1;
                Ok(Node::Group(alts))
            }
            '[' => self.class(),
            '.' => Ok(Node::Any),
            '^' => Ok(Node::Start),
            '$' => Ok(Node::End),
            '\\' => self.escape(false).map(|e| match e {
                Escaped::Char(ch) => Node::Char(ch),
                Escaped::Class(ranges, negated) => Node::Class { ranges, negated },
            }),
            '*' | '+' | '?' | '{' => Err(self.err("quantifier without operand")),
            ']' | '}' => Err(self.err("unbalanced bracket")),
            other => Ok(Node::Char(other)),
        }
    }

    fn quantified(&mut self, atom: Node) -> Result<Node, PatternError> {
        let (min, max) = match self.peek() {
            Some('*') => {
                self.pos += 1;
                (0, None)
            }
            Some('+') => {
                self.pos += 1;
                (1, None)
            }
            Some('?') => {
                self.pos += 1;
                (0, Some(1))
            }
            Some('{') => {
                self.pos += 1;
                let min = self.number()?;
                let max = if self.peek() == Some(',') {
                    self.pos += 1;
                    if self.peek() == Some('}') {
                        None
                    } else {
                        Some(self.number()?)
                    }
                } else {
                    Some(min)
                };
                if self.peek() != Some('}') {
                    return Err(self.err("missing `}`"));
                }
                self.pos += 1;
                if max.is_some_and(|m| m < min) {
                    return Err(self.err("quantifier max below min"));
                }
                (min, max)
            }
            _ => return Ok(atom),
        };
        if matches!(atom, Node::Start | Node::End) {
            return Err(self.err("quantified anchor"));
        }
        if matches!(atom, Node::Group(_)) && min > MAX_GROUP_REPEAT {
            return Err(self.err("a group repeated more than 64 times at minimum is not supported"));
        }
        if matches!(self.peek(), Some('?' | '+' | '*' | '{')) {
            return Err(self.err("lazy, possessive or stacked quantifiers are not supported"));
        }
        Ok(Node::Repeat {
            node: Box::new(atom),
            min,
            max,
        })
    }

    fn number(&mut self) -> Result<u32, PatternError> {
        let start = self.pos;
        while self.peek().is_some_and(|c| c.is_ascii_digit()) {
            self.pos += 1;
        }
        if start == self.pos {
            return Err(self.err("expected digits"));
        }
        let text: String = self.chars[start..self.pos].iter().collect();
        text.parse()
            .map_err(|_| self.err("quantifier bound out of range"))
    }

    fn class(&mut self) -> Result<Node, PatternError> {
        let negated = if self.peek() == Some('^') {
            self.pos += 1;
            true
        } else {
            false
        };
        let mut ranges = Vec::new();
        let mut first = true;
        loop {
            let c = self.peek().ok_or_else(|| self.err("missing `]`"))?;
            if c == ']' && !first {
                self.pos += 1;
                break;
            }
            first = false;
            self.pos += 1;
            let lo = if c == '\\' {
                match self.escape(true)? {
                    Escaped::Char(ch) => ch,
                    Escaped::Class(rs, false) => {
                        ranges.extend(rs);
                        continue;
                    }
                    Escaped::Class(_, true) => {
                        return Err(self.err("negated escape inside a class is not supported"));
                    }
                }
            } else {
                c
            };
            if self.peek() == Some('-') && self.chars.get(self.pos + 1).is_some_and(|&n| n != ']') {
                self.pos += 1;
                let hi_c = self.peek().ok_or_else(|| self.err("missing range end"))?;
                self.pos += 1;
                let hi = if hi_c == '\\' {
                    match self.escape(true)? {
                        Escaped::Char(ch) => ch,
                        Escaped::Class(..) => return Err(self.err("class escape as range end")),
                    }
                } else {
                    hi_c
                };
                if hi < lo {
                    return Err(self.err("reversed range"));
                }
                ranges.push((lo, hi));
            } else {
                ranges.push((lo, lo));
            }
        }
        Ok(Node::Class { ranges, negated })
    }

    /// Parse the character after a backslash (`self.pos` points just past the `\`).
    fn escape(&mut self, in_class: bool) -> Result<Escaped, PatternError> {
        let c = self.peek().ok_or_else(|| self.err("dangling `\\`"))?;
        self.pos += 1;
        let digits = vec![('0', '9')];
        let word = vec![('a', 'z'), ('A', 'Z'), ('0', '9'), ('_', '_')];
        let space = vec![
            (' ', ' '),
            ('\t', '\r'),
            ('\u{a0}', '\u{a0}'),
            ('\u{2028}', '\u{2029}'),
            ('\u{feff}', '\u{feff}'),
        ];
        Ok(match c {
            'd' => Escaped::Class(digits, false),
            'D' => Escaped::Class(digits, true),
            'w' => Escaped::Class(word, false),
            'W' => Escaped::Class(word, true),
            's' => Escaped::Class(space, false),
            'S' => Escaped::Class(space, true),
            'n' => Escaped::Char('\n'),
            'r' => Escaped::Char('\r'),
            't' => Escaped::Char('\t'),
            'u' => {
                let hex: String = self
                    .chars
                    .get(self.pos..self.pos + 4)
                    .ok_or_else(|| self.err("short \\u escape"))?
                    .iter()
                    .collect();
                let v = u32::from_str_radix(&hex, 16).map_err(|_| self.err("bad \\u escape"))?;
                self.pos += 4;
                Escaped::Char(char::from_u32(v).ok_or_else(|| self.err("surrogate in \\u escape"))?)
            }
            'b' if in_class => Escaped::Char('\u{8}'),
            c if c.is_ascii_alphanumeric() => {
                return Err(self.err("unsupported escape (back-reference, boundary or class)"));
            }
            other => Escaped::Char(other),
        })
    }
}

enum Escaped {
    Char(char),
    Class(Vec<(char, char)>, bool),
}

fn class_has(ranges: &[(char, char)], negated: bool, c: char) -> bool {
    ranges.iter().any(|&(lo, hi)| lo <= c && c <= hi) != negated
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct MatchLimit;

type MatchResult = Result<bool, MatchLimit>;
type Continuation<'a, 'text> = dyn FnMut(usize, &mut MatchState<'text>) -> MatchResult + 'a;

struct MatchState<'text> {
    text: &'text [char],
    remaining: usize,
}

impl MatchState<'_> {
    fn step(&mut self) -> Result<(), MatchLimit> {
        self.remaining = self.remaining.checked_sub(1).ok_or(MatchLimit)?;
        Ok(())
    }
}

fn match_seq<'text>(
    seq: &[Node],
    pos: usize,
    state: &mut MatchState<'text>,
    k: &mut Continuation<'_, 'text>,
) -> MatchResult {
    state.step()?;
    match seq.split_first() {
        None => k(pos, state),
        Some((first, rest)) => match_node(first, pos, state, &mut |p, state| {
            match_seq(rest, p, state, k)
        }),
    }
}

fn match_node<'text>(
    node: &Node,
    pos: usize,
    state: &mut MatchState<'text>,
    k: &mut Continuation<'_, 'text>,
) -> MatchResult {
    let next = match node {
        Node::Char(c) if state.text.get(pos) == Some(c) => pos + 1,
        Node::Any if state.text.get(pos).is_some_and(|&c| c != '\n' && c != '\r') => pos + 1,
        Node::Class { ranges, negated }
            if state
                .text
                .get(pos)
                .is_some_and(|&c| class_has(ranges, *negated, c)) =>
        {
            pos + 1
        }
        Node::Start if pos == 0 => pos,
        Node::End if pos == state.text.len() => pos,
        Node::Group(alts) => {
            for seq in alts {
                if match_seq(seq, pos, state, k)? {
                    return Ok(true);
                }
            }
            return Ok(false);
        }
        Node::Repeat { node, min, max } => return match_repeat(node, *min, *max, 0, pos, state, k),
        _ => return Ok(false),
    };
    k(next, state)
}

/// Most iterations of a repeated group (single-character operands repeat iteratively and
/// have no cap). A pattern whose group must repeat more often than this is refused at
/// compile time; a value needing more iterations than this fails to match (fail-closed).
const MAX_GROUP_REPEAT: u32 = 64;

fn match_repeat<'text>(
    node: &Node,
    min: u32,
    max: Option<u32>,
    count: u32,
    pos: usize,
    state: &mut MatchState<'text>,
    k: &mut Continuation<'_, 'text>,
) -> MatchResult {
    state.step()?;
    // Single-character operands repeat iteratively (no recursion per consumed character),
    // so long strings cannot exhaust the stack.
    let single = |c: char| match node {
        Node::Char(x) => *x == c,
        Node::Any => c != '\n' && c != '\r',
        Node::Class { ranges, negated } => class_has(ranges, *negated, c),
        _ => false,
    };
    if matches!(node, Node::Char(_) | Node::Any | Node::Class { .. }) {
        let limit = max.map_or(usize::MAX, |m| m as usize);
        let mut run = 0usize;
        while run < limit && state.text.get(pos + run).is_some_and(|&c| single(c)) {
            state.step()?;
            run += 1;
        }
        for n in (min as usize..=run).rev() {
            if k(pos + n, state)? {
                return Ok(true);
            }
        }
        return Ok(false);
    }
    // A repeated group recurses once per iteration. Cap the iterations so a hostile input
    // (a tier of thousands of `+gpu-cuda` segments) cannot exhaust the stack: past the cap
    // the repetition stops, the rest of the pattern fails to match, and validation refuses
    // the value (fail-closed). No schema of ours repeats a group more than a few times
    // (a tier has at most three segments); each iteration nests some tens of frames.
    if count < MAX_GROUP_REPEAT && max.is_none_or(|m| count < m) {
        let more = match_node(node, pos, state, &mut |p, state| {
            // An iteration that consumed nothing cannot make progress once `min` is met.
            if p == pos && count >= min {
                return Ok(false);
            }
            match_repeat(node, min, max, count + 1, p, state, k)
        })?;
        if more {
            return Ok(true);
        }
    }
    if count >= min {
        k(pos, state)
    } else {
        Ok(false)
    }
}

#[cfg(test)]
mod tests {
    use super::{MatchLimit, Pattern};

    fn m(p: &str, t: &str) -> bool {
        Pattern::compile(p).expect("pattern compiles").is_match(t) == Ok(true)
    }

    #[test]
    fn pattern_refusal_code_whole_token() {
        let p = "^FROB-REF-[1-9][0-9]{2,3}$";
        assert!(m(p, "FROB-REF-802"));
        assert!(m(p, "FROB-REF-8100"));
        assert!(!m(p, "FROB-REF-81000"));
        assert!(!m(p, "FROB-REF-081"));
        assert!(!m(p, "FROB-REF-80"));
        assert!(!m(p, "xFROB-REF-802"));
        assert!(!m(p, "FROB-REF-802 "));
    }

    #[test]
    fn pattern_stack_receipt_and_tiers() {
        let sr = "^sr:[0-9a-f]{64}$";
        assert!(m(sr, &format!("sr:{}", "ab".repeat(32))));
        assert!(!m(sr, &format!("sr:{}", "AB".repeat(32))));
        assert!(!m(sr, &format!("sl:{}", "ab".repeat(32))));
        assert!(!m(sr, &format!("sr:{}", "ab".repeat(31))));
        let tier =
            "^(static|referee|(batch-cpu|gpu-cuda|gpu-metal)(\\+(batch-cpu|gpu-cuda|gpu-metal))*)$";
        for ok in ["static", "referee", "gpu-cuda", "batch-cpu+gpu-metal"] {
            assert!(m(tier, ok), "{ok}");
        }
        for bad in ["", "static+referee", "gpu", "batch-cpu+", "STATIC"] {
            assert!(!m(tier, bad), "{bad}");
        }
    }

    #[test]
    fn pattern_unanchored_search_and_classes() {
        assert!(m("b+", "abbbc"));
        assert!(!m("^b+", "abbbc"));
        assert!(m("^a|b", "xb"));
        assert!(!m("^a|^b", "xb"));
        assert!(m("(^a|b)$", "xb"));
        assert!(m("^[^/].*$", "fixtures/a.toml"));
        assert!(!m("^[^/].*$", "/abs/a.toml"));
        assert!(m("^[a-z][a-z0-9-]*$", "executor-in-process"));
        assert!(!m("^[a-z][a-z0-9-]*$", "Executor"));
        assert!(m("^(0|[1-9][0-9]*)$", "0"));
        assert!(!m("^(0|[1-9][0-9]*)$", "01"));
        assert!(m("^\\d{3}$", "123") && !m("^\\d{3}$", "12a"));
        assert!(m("^a(b|cd)?e$", "ae") && m("^a(b|cd)?e$", "acde") && !m("^a(b|cd)?e$", "abde"));
        assert!(m("^(a*)*$", "aaaa"));
        let long = "x".repeat(200_000);
        assert!(m("^calibrated:.+$", &format!("calibrated:{long}")));
        assert!(!m("^calibrated:.+$", "calibrated:"));
    }

    #[test]
    fn pattern_backtracking_budget_stops_ambiguous_nonmatch() {
        let pattern = Pattern::compile("^(a|a)*b$").expect("pattern compiles");
        assert_eq!(pattern.is_match("aaaaab"), Ok(true));
        assert_eq!(pattern.is_match(&"a".repeat(32_768)), Err(MatchLimit));
    }

    #[test]
    fn pattern_group_repeat_is_bounded() {
        let tier =
            "^(static|referee|(batch-cpu|gpu-cuda|gpu-metal)(\\+(batch-cpu|gpu-cuda|gpu-metal))*)$";
        // Within the cap: matches.
        assert!(m(tier, &format!("gpu-cuda{}", "+gpu-metal".repeat(60))));
        // A hostile tier of 50 000 segments neither overflows the stack nor matches.
        assert!(!m(
            tier,
            &format!("gpu-cuda{}", "+gpu-metal".repeat(50_000))
        ));
    }

    #[test]
    fn pattern_rejects_unsupported_syntax() {
        for bad in [
            "(?=a)", "\\1", "\\b", "a{2,1}", "a**", "a+?", "[a", "(a", "a)", "*a", "[z-a]",
            "(ab){65}",
        ] {
            assert!(Pattern::compile(bad).is_err(), "{bad} must be refused");
        }
    }
}
