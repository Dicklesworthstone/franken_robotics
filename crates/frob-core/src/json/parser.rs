//! Iterative raw-byte RFC 8259 JSON admission parser (bead N34-17).
//!
//! Independent std-only implementation for `frob-core`: explicit heap frame stack
//! (no call-stack recursion), pre-allocation limit checks, strict surrogate-pair
//! decoding, verbatim number lexemes, and O(n log n) duplicate-member rejection
//! after escape decoding by exact Unicode code point (no normalization).

use std::cmp::Ordering;
use std::fmt::Write as _;

use crate::json::types::{
    AdmissionFault, Admitted, FaultKind, Json, Limits, Member, Span, Spanned,
};

const UTF8_BOM: [u8; 3] = [0xEF, 0xBB, 0xBF];

enum Frame<'a> {
    Array {
        start: usize,
        items: Vec<Spanned<'a>>,
    },
    Object {
        start: usize,
        members: Vec<Member<'a>>,
        name: String,
        name_span: Span,
    },
}

struct Parser<'a> {
    bytes: &'a [u8],
    text: &'a str,
    pos: usize,
    limits: Limits,
    frames: Vec<Frame<'a>>,
}

impl<'a> Parser<'a> {
    fn new(bytes: &'a [u8], text: &'a str, limits: Limits) -> Self {
        Self {
            bytes,
            text,
            pos: 0,
            limits,
            frames: Vec::new(),
        }
    }

    /// Build an RFC 6901 JSON Pointer from the active container stack on fault paths only.
    fn pointer(&self) -> String {
        let mut out = String::new();
        for frame in &self.frames {
            out.push('/');
            match frame {
                Frame::Array { items, .. } => {
                    let _ = write!(out, "{}", items.len());
                }
                Frame::Object { name, .. } => {
                    for ch in name.chars() {
                        match ch {
                            '~' => out.push_str("~0"),
                            '/' => out.push_str("~1"),
                            _ => out.push(ch),
                        }
                    }
                }
            }
        }
        out
    }

    fn err(&self, kind: FaultKind, offset: usize, message: &'static str) -> AdmissionFault {
        AdmissionFault::new(kind, offset, self.pointer(), message)
    }

    fn fail(&self, kind: FaultKind, message: &'static str) -> AdmissionFault {
        self.err(kind, self.pos, message)
    }

    fn peek(&self) -> Option<u8> {
        self.bytes.get(self.pos).copied()
    }

    fn skip_ws(&mut self) {
        while matches!(self.peek(), Some(b' ' | b'\t' | b'\n' | b'\r')) {
            self.pos += 1;
        }
    }

    fn parse_document(mut self) -> Result<Admitted<'a>, AdmissionFault> {
        self.skip_ws();
        if self.pos >= self.bytes.len() {
            return Err(self.fail(FaultKind::Syntax, "empty JSON input"));
        }
        let root = self.parse_value_loop()?;
        self.skip_ws();
        if self.pos < self.bytes.len() {
            return Err(self.fail(FaultKind::TrailingData, "trailing data after root value"));
        }
        Ok(Admitted::new(root, self.bytes))
    }

    fn parse_value_loop(&mut self) -> Result<Spanned<'a>, AdmissionFault> {
        let mut completed: Option<Spanned<'a>> = None;
        loop {
            if let Some(value) = completed.take() {
                let Some(frame) = self.frames.pop() else {
                    return Ok(value);
                };
                match frame {
                    Frame::Array { start, mut items } => {
                        items.push(value);
                        self.skip_ws();
                        match self.peek() {
                            Some(b',') => {
                                self.pos += 1;
                                self.skip_ws();
                                if self.peek() == Some(b']') {
                                    return Err(
                                        self.fail(FaultKind::Syntax, "trailing comma in array")
                                    );
                                }
                                self.frames.push(Frame::Array { start, items });
                                continue;
                            }
                            Some(b']') => {
                                self.pos += 1;
                                completed = Some(Spanned::new(
                                    Span::new(start, self.pos),
                                    Json::Arr(items),
                                ));
                                continue;
                            }
                            _ => {
                                return Err(
                                    self.fail(FaultKind::Syntax, "expected ',' or ']' in array")
                                );
                            }
                        }
                    }
                    Frame::Object {
                        start,
                        mut members,
                        name,
                        name_span,
                    } => {
                        members.push(Member {
                            name,
                            name_span,
                            value,
                        });
                        self.skip_ws();
                        match self.peek() {
                            Some(b',') => {
                                self.pos += 1;
                                self.skip_ws();
                                if self.peek() == Some(b'}') {
                                    return Err(
                                        self.fail(FaultKind::Syntax, "trailing comma in object")
                                    );
                                }
                                let (next_name, next_span) = self.parse_key_and_colon()?;
                                self.frames.push(Frame::Object {
                                    start,
                                    members,
                                    name: next_name,
                                    name_span: next_span,
                                });
                                continue;
                            }
                            Some(b'}') => {
                                self.pos += 1;
                                let checked = self.finalize_object(members)?;
                                completed = Some(Spanned::new(
                                    Span::new(start, self.pos),
                                    Json::Obj(checked),
                                ));
                                continue;
                            }
                            _ => {
                                return Err(
                                    self.fail(FaultKind::Syntax, "expected ',' or '}' in object")
                                );
                            }
                        }
                    }
                }
            }

            self.skip_ws();
            let Some(byte) = self.peek() else {
                return Err(self.fail(FaultKind::Syntax, "unexpected end of input"));
            };
            match byte {
                b'{' | b'[' => {
                    let start = self.pos;
                    if self.frames.len() + 1 > usize::from(self.limits.max_depth) {
                        return Err(self.err(
                            FaultKind::TooDeep,
                            start,
                            "container nesting exceeds max_depth",
                        ));
                    }
                    self.pos += 1;
                    self.skip_ws();
                    if byte == b'{' {
                        if self.peek() == Some(b'}') {
                            self.pos += 1;
                            completed = Some(Spanned::new(
                                Span::new(start, self.pos),
                                Json::Obj(Vec::new()),
                            ));
                            continue;
                        }
                        let (name, name_span) = self.parse_key_and_colon()?;
                        self.frames.push(Frame::Object {
                            start,
                            members: Vec::new(),
                            name,
                            name_span,
                        });
                    } else if self.peek() == Some(b']') {
                        self.pos += 1;
                        completed = Some(Spanned::new(
                            Span::new(start, self.pos),
                            Json::Arr(Vec::new()),
                        ));
                    } else {
                        self.frames.push(Frame::Array {
                            start,
                            items: Vec::new(),
                        });
                    }
                }
                b'"' => {
                    let (decoded, span) = self.parse_string()?;
                    completed = Some(Spanned::new(span, Json::Str(decoded)));
                }
                b't' => {
                    completed = Some(Spanned::new(self.parse_literal(b"true")?, Json::Bool(true)));
                }
                b'f' => {
                    completed = Some(Spanned::new(
                        self.parse_literal(b"false")?,
                        Json::Bool(false),
                    ));
                }
                b'n' => {
                    completed = Some(Spanned::new(self.parse_literal(b"null")?, Json::Null));
                }
                b'-' | b'0'..=b'9' => {
                    let (lexeme, span) = self.parse_number()?;
                    completed = Some(Spanned::new(span, Json::Num(lexeme)));
                }
                b'+' | b'.' => {
                    return Err(self.fail(FaultKind::NumberForm, "invalid number start"));
                }
                _ => {
                    return Err(self.fail(FaultKind::Syntax, "unexpected byte in JSON value"));
                }
            }
        }
    }

    fn parse_key_and_colon(&mut self) -> Result<(String, Span), AdmissionFault> {
        if self.peek() != Some(b'"') {
            return Err(self.fail(FaultKind::Syntax, "expected object member name string"));
        }
        let (name, span) = self.parse_string()?;
        self.skip_ws();
        if self.peek() != Some(b':') {
            return Err(self.fail(FaultKind::Syntax, "expected ':' after object member name"));
        }
        self.pos += 1;
        Ok((name, span))
    }

    fn parse_literal(&mut self, lit: &[u8]) -> Result<Span, AdmissionFault> {
        let start = self.pos;
        if !self.bytes.get(start..).is_some_and(|r| r.starts_with(lit)) {
            return Err(self.err(FaultKind::Syntax, start, "invalid JSON literal"));
        }
        self.pos += lit.len();
        Ok(Span::new(start, self.pos))
    }

    fn parse_string(&mut self) -> Result<(String, Span), AdmissionFault> {
        let start = self.pos;
        self.pos += 1;
        let max_string = self.limits.max_string;
        let content_start = self.pos;
        let mut cursor = content_start;

        // Fast path: scan unescaped prefix while checking raw control bytes and max_string
        // before allocating any output buffer.
        while let Some(&b) = self.bytes.get(cursor) {
            if b == b'"' {
                self.pos = cursor + 1;
                let s = self
                    .text
                    .get(content_start..cursor)
                    .unwrap_or("")
                    .to_string();
                return Ok((s, Span::new(start, self.pos)));
            }
            if b == b'\\' {
                break;
            }
            if b < 0x20 {
                return Err(self.err(
                    FaultKind::ControlInString,
                    cursor,
                    "raw control character in string",
                ));
            }
            if cursor - content_start >= max_string {
                return Err(self.err(
                    FaultKind::StringTooLong,
                    cursor,
                    "string exceeds max_string",
                ));
            }
            cursor += 1;
        }
        if cursor >= self.bytes.len() {
            return Err(self.err(FaultKind::Syntax, start, "unterminated string"));
        }

        let mut out = String::with_capacity((cursor - content_start).min(max_string));
        out.push_str(self.text.get(content_start..cursor).unwrap_or(""));
        self.pos = cursor;

        while let Some(&b) = self.bytes.get(self.pos) {
            if b == b'"' {
                self.pos += 1;
                return Ok((out, Span::new(start, self.pos)));
            }
            if b == b'\\' {
                let esc_start = self.pos;
                self.pos += 1;
                let Some(esc) = self.peek() else {
                    return Err(self.err(FaultKind::Syntax, esc_start, "unterminated escape"));
                };
                self.pos += 1;
                let ch = match esc {
                    b'"' => '"',
                    b'\\' => '\\',
                    b'/' => '/',
                    b'b' => '\u{0008}',
                    b'f' => '\u{000C}',
                    b'n' => '\n',
                    b'r' => '\r',
                    b't' => '\t',
                    b'u' => self.parse_unicode_escape(esc_start)?,
                    _ => {
                        return Err(self.err(
                            FaultKind::Syntax,
                            esc_start,
                            "invalid escape sequence",
                        ));
                    }
                };
                if out.len().saturating_add(ch.len_utf8()) > max_string {
                    return Err(self.err(
                        FaultKind::StringTooLong,
                        esc_start,
                        "decoded string exceeds max_string",
                    ));
                }
                out.push(ch);
                continue;
            }
            if b < 0x20 {
                return Err(self.fail(
                    FaultKind::ControlInString,
                    "raw control character in string",
                ));
            }
            let run_start = self.pos;
            while let Some(&rb) = self.bytes.get(self.pos) {
                if rb == b'"' || rb == b'\\' || rb < 0x20 {
                    break;
                }
                if out.len().saturating_add(self.pos - run_start) >= max_string {
                    return Err(self.fail(FaultKind::StringTooLong, "string exceeds max_string"));
                }
                self.pos += 1;
            }
            out.push_str(self.text.get(run_start..self.pos).unwrap_or(""));
        }
        Err(self.err(FaultKind::Syntax, start, "unterminated string"))
    }

    fn parse_hex4(&mut self, esc_start: usize) -> Result<u16, AdmissionFault> {
        let Some(slice) = self.bytes.get(self.pos..self.pos + 4) else {
            return Err(self.err(FaultKind::Syntax, esc_start, "truncated \\uXXXX escape"));
        };
        let mut val: u16 = 0;
        for (i, &b) in slice.iter().enumerate() {
            let digit = match b {
                b'0'..=b'9' => u16::from(b - b'0'),
                b'a'..=b'f' => u16::from(b - b'a' + 10),
                b'A'..=b'F' => u16::from(b - b'A' + 10),
                _ => {
                    return Err(self.err(
                        FaultKind::Syntax,
                        self.pos + i,
                        "non-hexadecimal character in \\uXXXX escape",
                    ));
                }
            };
            val = (val << 4) | digit;
        }
        self.pos += 4;
        Ok(val)
    }

    fn parse_unicode_escape(&mut self, esc_start: usize) -> Result<char, AdmissionFault> {
        let code = self.parse_hex4(esc_start)?;
        if (0xD800..=0xDBFF).contains(&code) {
            if !self
                .bytes
                .get(self.pos..)
                .is_some_and(|r| r.starts_with(b"\\u"))
            {
                return Err(self.err(
                    FaultKind::LoneSurrogate,
                    esc_start,
                    "lone high surrogate escape",
                ));
            }
            let low_start = self.pos;
            self.pos += 2;
            let low = self.parse_hex4(low_start)?;
            if !(0xDC00..=0xDFFF).contains(&low) {
                return Err(self.err(
                    FaultKind::LoneSurrogate,
                    esc_start,
                    "high surrogate not followed by low surrogate",
                ));
            }
            let scalar = 0x1_0000 + ((u32::from(code) - 0xD800) << 10) + (u32::from(low) - 0xDC00);
            char::from_u32(scalar).ok_or_else(|| {
                self.err(
                    FaultKind::LoneSurrogate,
                    esc_start,
                    "invalid surrogate pair scalar",
                )
            })
        } else if (0xDC00..=0xDFFF).contains(&code) {
            Err(self.err(
                FaultKind::LoneSurrogate,
                esc_start,
                "lone low surrogate escape",
            ))
        } else {
            char::from_u32(u32::from(code)).ok_or_else(|| {
                self.err(FaultKind::Syntax, esc_start, "invalid Unicode scalar value")
            })
        }
    }

    fn step_number(&mut self, start: usize) -> Result<(), AdmissionFault> {
        self.pos += 1;
        if self.pos - start > self.limits.max_number_len {
            let off = start + self.limits.max_number_len;
            return Err(self.err(
                FaultKind::NumberTooLong,
                off,
                "number lexeme exceeds max_number_len",
            ));
        }
        Ok(())
    }

    fn eat_digits(&mut self, start: usize) -> Result<(), AdmissionFault> {
        while matches!(self.peek(), Some(b'0'..=b'9')) {
            self.step_number(start)?;
        }
        Ok(())
    }

    fn parse_number(&mut self) -> Result<(&'a str, Span), AdmissionFault> {
        let start = self.pos;
        if self.peek() == Some(b'-') {
            self.step_number(start)?;
        }
        match self.peek() {
            Some(b'0') => {
                self.step_number(start)?;
                if matches!(self.peek(), Some(b'0'..=b'9')) {
                    return Err(self.fail(FaultKind::NumberForm, "leading zero in number"));
                }
            }
            Some(b'1'..=b'9') => self.eat_digits(start)?,
            _ => return Err(self.fail(FaultKind::NumberForm, "expected digit in number")),
        }
        if self.peek() == Some(b'.') {
            self.step_number(start)?;
            if !matches!(self.peek(), Some(b'0'..=b'9')) {
                return Err(self.fail(FaultKind::NumberForm, "expected fractional digit"));
            }
            self.eat_digits(start)?;
        }
        if matches!(self.peek(), Some(b'e' | b'E')) {
            self.step_number(start)?;
            if matches!(self.peek(), Some(b'+' | b'-')) {
                self.step_number(start)?;
            }
            if !matches!(self.peek(), Some(b'0'..=b'9')) {
                return Err(self.fail(FaultKind::NumberForm, "expected exponent digit"));
            }
            self.eat_digits(start)?;
        }
        if matches!(self.peek(), Some(b'.' | b'e' | b'E' | b'0'..=b'9')) {
            return Err(self.fail(FaultKind::NumberForm, "malformed number continuation"));
        }
        let lexeme = self.text.get(start..self.pos).unwrap_or("");
        Ok((lexeme, Span::new(start, self.pos)))
    }

    fn finalize_object(&self, members: Vec<Member<'a>>) -> Result<Vec<Member<'a>>, AdmissionFault> {
        if members.len() <= 1 {
            return Ok(members);
        }
        let mut order: Vec<usize> = (0..members.len()).collect();
        order.sort_by(|&a, &b| match (members.get(a), members.get(b)) {
            (Some(la), Some(rb)) => match self.cmp_key(la).cmp(self.cmp_key(rb)) {
                Ordering::Equal => a.cmp(&b),
                other => other,
            },
            _ => a.cmp(&b),
        });

        #[cfg(any(feature = "dup-first-wins", feature = "dup-last-wins"))]
        {
            let mut keep = vec![true; members.len()];
            for w in order.windows(2) {
                let (Some(&i0), Some(&i1)) = (w.first(), w.get(1)) else {
                    continue;
                };
                if let (Some(a), Some(b)) = (members.get(i0), members.get(i1))
                    && self.cmp_key(a) == self.cmp_key(b)
                {
                    let drop_idx = if cfg!(feature = "dup-first-wins") {
                        i1
                    } else {
                        i0
                    };
                    if let Some(slot) = keep.get_mut(drop_idx) {
                        *slot = false;
                    }
                }
            }
            return Ok(members
                .into_iter()
                .enumerate()
                .filter_map(|(i, m)| keep.get(i).copied().unwrap_or(false).then_some(m))
                .collect());
        }

        #[cfg(not(any(feature = "dup-first-wins", feature = "dup-last-wins")))]
        {
            // Pick the earliest duplicate occurrence (smallest second index, tie-broken by first)
            // so the reported member and offsets are D0-deterministic across all inputs.
            let mut earliest: Option<(usize, usize)> = None;
            for w in order.windows(2) {
                let (Some(&i0), Some(&i1)) = (w.first(), w.get(1)) else {
                    continue;
                };
                if let (Some(a), Some(b)) = (members.get(i0), members.get(i1))
                    && self.cmp_key(a) == self.cmp_key(b)
                    && earliest.is_none_or(|best| i1 < best.1 || (i1 == best.1 && i0 < best.0))
                {
                    earliest = Some((i0, i1));
                }
            }
            if let Some((fi, si)) = earliest
                && let (Some(first), Some(second)) = (members.get(fi), members.get(si))
            {
                return Err(AdmissionFault::duplicate_member(
                    first.name.clone(),
                    first.name_span.start,
                    second.name_span.start,
                    self.pointer(),
                ));
            }
            Ok(members)
        }
    }

    fn cmp_key<'b>(&'b self, member: &'b Member<'a>) -> &'b str {
        #[cfg(feature = "skip-escape-decode")]
        {
            self.text
                .get(member.name_span.start..member.name_span.end)
                .unwrap_or(member.name.as_str())
        }
        #[cfg(not(feature = "skip-escape-decode"))]
        {
            member.name.as_str()
        }
    }
}

/// Admit raw JSON bytes under `limits` before any map or digest is built.
///
/// Checks execute in deterministic order: size limit, valid UTF-8, no UTF-8 BOM,
/// RFC 8259 grammar with pre-allocation depth/string/number bounds, strict escape and
/// surrogate-pair decoding, and duplicate object member rejection after escape decoding
/// by exact code points.
pub fn admit<'a>(bytes: &'a [u8], limits: &Limits) -> Result<Admitted<'a>, AdmissionFault> {
    if bytes.len() > limits.max_bytes {
        return Err(AdmissionFault::new(
            FaultKind::TooLarge,
            limits.max_bytes,
            String::new(),
            "input exceeds max_bytes",
        ));
    }
    let text = std::str::from_utf8(bytes).map_err(|e| {
        AdmissionFault::new(
            FaultKind::NotUtf8,
            e.valid_up_to(),
            String::new(),
            "input is not valid UTF-8",
        )
    })?;
    if bytes.starts_with(&UTF8_BOM) {
        return Err(AdmissionFault::new(
            FaultKind::Bom,
            0,
            String::new(),
            "UTF-8 byte-order mark is forbidden",
        ));
    }
    Parser::new(bytes, text, *limits).parse_document()
}
