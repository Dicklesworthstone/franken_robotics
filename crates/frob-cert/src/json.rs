//! Strict canonical JSON reader and re-serializer for `frob-cert` (BD-026).
//!
//! Enforces rules in the exact order required by BD-026 item 1:
//! - 1a: file size <= `max_file_bytes`, valid UTF-8, no BOM (`json.bom`), valid JSON
//!   grammar (`json.syntax`), nesting depth <= `max_json_depth` (`limits.exceeded`),
//!   numeric entry count <= `max_numeric_entries` (`limits.exceeded`).
//! - 1b: no duplicate object keys at any nesting level, checked after escape decoding
//!   (`json.duplicate-key`).
//! - 1c: integers only within `±(2^53 - 1)`, no decimal point or exponent
//!   (`num.float-in-bundle`), no `-0` (`num.int-form`), no out-of-range integer
//!   (`num.int-range`).
//! - 1d: canonical form — keys in ascending Unicode code-point / UTF-8 byte order
//!   (`json.key-order`), zero insignificant whitespace including trailing newline
//!   (`json.whitespace`), minimal string escapes (`\"`, `\\`, and lowercase `\u00xx`
//!   for code points below U+0020; nothing else escaped, `json.escape`), and final
//!   re-serialize byte equality against the input.

use std::collections::HashSet;
use std::fmt;

use crate::bigint::{ArithError, BigInt};
use crate::limits::{
    Limits, MAX_JSON_INT, REFUSAL_ARITHMETIC_MALFORMED, RULE_JSON_BOM, RULE_JSON_DUPLICATE_KEY,
    RULE_JSON_ESCAPE, RULE_JSON_KEY_ORDER, RULE_JSON_SYNTAX, RULE_JSON_WHITESPACE,
    RULE_LIMITS_EXCEEDED, RULE_NUM_FLOAT_IN_BUNDLE, RULE_NUM_INT_FORM, RULE_NUM_INT_RANGE,
};
use crate::rational::Rational;

/// Value tree produced by the strict canonical JSON reader (no floats).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Int(i64),
    Str(String),
    Arr(Vec<JsonValue>),
    /// Object members preserved in parsed order (strictly ascending code-point order when canonical).
    Obj(Vec<(String, JsonValue)>),
}

impl JsonValue {
    pub fn as_obj(&self) -> Option<&[(String, JsonValue)]> {
        match self {
            Self::Obj(entries) => Some(entries),
            _ => None,
        }
    }

    pub fn get(&self, key: &str) -> Option<&JsonValue> {
        self.as_obj()?
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v)
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::Str(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Self::Int(n) => Some(*n),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(b) => Some(*b),
            _ => None,
        }
    }

    pub fn as_arr(&self) -> Option<&[JsonValue]> {
        match self {
            Self::Arr(items) => Some(items),
            _ => None,
        }
    }

    /// Parse an exact integer from either a canonical JSON `Int` or a strict decimal `Str`.
    pub fn parse_bigint(&self) -> Result<BigInt, ArithError> {
        match self {
            Self::Int(n) => Ok(BigInt::from_i64(*n)),
            Self::Str(s) => BigInt::parse_strict(s),
            _ => Err(ArithError::InvalidIntForm(
                "expected integer or decimal string".to_string(),
            )),
        }
    }

    /// Parse a strict pair-encoded rational `["p", "q"]` (BD-026 item 2).
    pub fn parse_rational_pair(&self) -> Result<Rational, ArithError> {
        let items = self.as_arr().ok_or_else(|| {
            ArithError::InvalidRationalForm("expected 2-element string array".to_string())
        })?;
        if items.len() != 2 {
            return Err(ArithError::InvalidRationalForm(format!(
                "expected 2 elements in rational pair, got {}",
                items.len()
            )));
        }
        let p = items[0].as_str().ok_or_else(|| {
            ArithError::InvalidRationalForm("rational pair numerator must be string".to_string())
        })?;
        let q = items[1].as_str().ok_or_else(|| {
            ArithError::InvalidRationalForm("rational pair denominator must be string".to_string())
        })?;
        Rational::parse_pair_strict(p, q)
    }

    /// Parse a strict slash-encoded rational `"p/q"` or whole `"p"` (BD-026 item 2).
    pub fn parse_rational_slash(&self) -> Result<Rational, ArithError> {
        let s = self.as_str().ok_or_else(|| {
            ArithError::InvalidRationalForm("expected string for slash rational".to_string())
        })?;
        Rational::parse_slash_strict(s)
    }
}

/// Structured error returned by the canonical JSON reader.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsonError {
    pub rule: &'static str,
    pub refusal: Option<&'static str>,
    pub path: String,
    pub offset: usize,
    pub message: String,
}

impl fmt::Display for JsonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} at {} (byte {}): {}",
            self.rule, self.path, self.offset, self.message
        )
    }
}

impl std::error::Error for JsonError {}

#[derive(Debug, Clone, Copy)]
struct ParseOptions {
    enforce_canonical: bool,
    skip_key_order: bool,
    dup_first_wins: bool,
}

/// Parse strict canonical JSON under `limits`.
pub fn parse_canonical(bytes: &[u8], limits: &Limits) -> Result<JsonValue, JsonError> {
    let opts = ParseOptions {
        enforce_canonical: true,
        skip_key_order: cfg!(feature = "nc-skip-key-order"),
        dup_first_wins: cfg!(feature = "nc-dup-key-first-wins"),
    };
    parse_with_options(bytes, limits, opts)
}

/// Parse JSON enforcing rules 1a (UTF-8, no BOM, grammar, size/depth/count limits),
/// 1b (decoded duplicate keys), and 1c (integers only in `±(2^53 - 1)`), while
/// allowing non-canonical whitespace, escapes, and unsorted object keys.
pub fn parse_non_canonical(bytes: &[u8], limits: &Limits) -> Result<JsonValue, JsonError> {
    let opts = ParseOptions {
        enforce_canonical: false,
        skip_key_order: true,
        dup_first_wins: false,
    };
    parse_with_options(bytes, limits, opts)
}

/// Admit non-canonical JSON (enforcing 1a–1c) and emit its unique canonical bytes.
pub fn canonicalize_bytes(bytes: &[u8], limits: &Limits) -> Result<Vec<u8>, JsonError> {
    let val = parse_non_canonical(bytes, limits)?;
    Ok(serialize_canonical(&val).into_bytes())
}

/// Negative-control mutant: canonical JSON reader with key-order checking disabled.
#[cfg(any(test, feature = "nc-skip-key-order"))]
pub fn parse_skip_key_order_mutant(bytes: &[u8], limits: &Limits) -> Result<JsonValue, JsonError> {
    let opts = ParseOptions {
        enforce_canonical: true,
        skip_key_order: true,
        dup_first_wins: false,
    };
    parse_with_options(bytes, limits, opts)
}

/// Negative-control mutant: canonical JSON reader that silently keeps the first duplicate key.
#[cfg(any(test, feature = "nc-dup-key-first-wins"))]
pub fn parse_dup_first_wins_mutant(bytes: &[u8], limits: &Limits) -> Result<JsonValue, JsonError> {
    let opts = ParseOptions {
        enforce_canonical: true,
        skip_key_order: false,
        dup_first_wins: true,
    };
    parse_with_options(bytes, limits, opts)
}

/// Serialize any `JsonValue` into canonical JSON bytes (sorted keys by Unicode code point,
/// zero insignificant whitespace, minimal string escapes).
pub fn serialize_canonical(value: &JsonValue) -> String {
    let mut out = String::new();
    write_canonical_value(&mut out, value);
    out
}

fn write_canonical_value(out: &mut String, value: &JsonValue) {
    match value {
        JsonValue::Null => out.push_str("null"),
        JsonValue::Bool(true) => out.push_str("true"),
        JsonValue::Bool(false) => out.push_str("false"),
        JsonValue::Int(n) => out.push_str(&n.to_string()),
        JsonValue::Str(s) => write_canonical_string(out, s),
        JsonValue::Arr(items) => {
            out.push('[');
            for (idx, item) in items.iter().enumerate() {
                if idx > 0 {
                    out.push(',');
                }
                write_canonical_value(out, item);
            }
            out.push(']');
        }
        JsonValue::Obj(entries) => {
            out.push('{');
            let mut order: Vec<usize> = (0..entries.len()).collect();
            // Sort by UTF-8 byte order, which equals Unicode scalar / code-point order.
            order.sort_by(|&a, &b| entries[a].0.as_bytes().cmp(entries[b].0.as_bytes()));
            for (pos, idx) in order.into_iter().enumerate() {
                if pos > 0 {
                    out.push(',');
                }
                let (k, v) = &entries[idx];
                write_canonical_string(out, k);
                out.push(':');
                write_canonical_value(out, v);
            }
            out.push('}');
        }
    }
}

fn write_canonical_string(out: &mut String, s: &str) {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    out.push('"');
    let mut chunk_start = 0;
    for (idx, ch) in s.char_indices() {
        let code = ch as u32;
        if ch == '"' || ch == '\\' || code < 0x20 {
            if idx > chunk_start {
                out.push_str(&s[chunk_start..idx]);
            }
            match ch {
                '"' => out.push_str("\\\""),
                '\\' => out.push_str("\\\\"),
                _ => {
                    let b = code as u8;
                    out.push_str("\\u00");
                    out.push(HEX[(b >> 4) as usize] as char);
                    out.push(HEX[(b & 0x0f) as usize] as char);
                }
            }
            chunk_start = idx + ch.len_utf8();
        }
    }
    if chunk_start < s.len() {
        out.push_str(&s[chunk_start..]);
    }
    out.push('"');
}

fn parse_with_options(
    bytes: &[u8],
    limits: &Limits,
    opts: ParseOptions,
) -> Result<JsonValue, JsonError> {
    // 1a: size limit first so oversized inputs are refused without scanning.
    if bytes.len() > limits.max_file_bytes {
        return Err(JsonError {
            rule: RULE_LIMITS_EXCEEDED,
            refusal: None,
            path: "$".to_string(),
            offset: 0,
            message: format!(
                "file size {} bytes exceeds limit {} bytes",
                bytes.len(),
                limits.max_file_bytes
            ),
        });
    }
    // 1a: refuse UTF-8 BOM explicitly.
    if bytes.starts_with(&[0xEF, 0xBB, 0xBF]) {
        return Err(JsonError {
            rule: RULE_JSON_BOM,
            refusal: None,
            path: "$".to_string(),
            offset: 0,
            message: "UTF-8 byte-order mark is forbidden".to_string(),
        });
    }
    // 1a: valid UTF-8.
    let src = std::str::from_utf8(bytes).map_err(|e| JsonError {
        rule: RULE_JSON_SYNTAX,
        refusal: None,
        path: "$".to_string(),
        offset: e.valid_up_to(),
        message: format!("invalid UTF-8: {e}"),
    })?;

    let mut parser = Parser {
        src,
        bytes,
        pos: 0,
        depth: 0,
        numeric_entries: 0,
        limits: *limits,
        opts,
        path: Vec::new(),
        first_canon_err: None,
    };

    parser.skip_ws();
    let val = parser.parse_value()?;
    parser.skip_ws();
    if parser.pos != parser.bytes.len() {
        return Err(parser.err(
            RULE_JSON_SYNTAX,
            None,
            parser.pos,
            "trailing bytes after JSON document".to_string(),
        ));
    }

    // 1d: only surfaced after 1a, 1b, and 1c have all succeeded.
    if opts.enforce_canonical {
        if let Some(canon_err) = parser.first_canon_err {
            return Err(canon_err);
        }
        // Final belt-and-suspenders byte-exactness check (skipped only under nc-* mutants).
        if !opts.skip_key_order && !opts.dup_first_wins {
            let reserialized = serialize_canonical(&val);
            if reserialized.as_bytes() != bytes {
                return Err(JsonError {
                    rule: RULE_JSON_SYNTAX,
                    refusal: None,
                    path: "$".to_string(),
                    offset: 0,
                    message: "re-serialized canonical JSON differed from input bytes".to_string(),
                });
            }
        }
    }

    Ok(val)
}

#[derive(Debug, Clone)]
enum PathSeg {
    Key(String),
    Index(usize),
}

struct Parser<'a> {
    src: &'a str,
    bytes: &'a [u8],
    pos: usize,
    depth: usize,
    numeric_entries: usize,
    limits: Limits,
    opts: ParseOptions,
    path: Vec<PathSeg>,
    first_canon_err: Option<JsonError>,
}

impl<'a> Parser<'a> {
    fn format_path(&self) -> String {
        let mut out = String::from("$");
        for seg in &self.path {
            match seg {
                PathSeg::Key(k) => {
                    out.push('.');
                    out.push_str(k);
                }
                PathSeg::Index(i) => {
                    out.push('[');
                    out.push_str(&i.to_string());
                    out.push(']');
                }
            }
        }
        out
    }

    fn err(
        &self,
        rule: &'static str,
        refusal: Option<&'static str>,
        offset: usize,
        message: String,
    ) -> JsonError {
        JsonError {
            rule,
            refusal,
            path: self.format_path(),
            offset,
            message,
        }
    }

    fn record_canon_err(&mut self, rule: &'static str, offset: usize, message: String) {
        if self.opts.enforce_canonical && self.first_canon_err.is_none() {
            self.first_canon_err = Some(self.err(rule, None, offset, message));
        }
    }

    fn peek(&self) -> Option<u8> {
        self.bytes.get(self.pos).copied()
    }

    fn skip_ws(&mut self) {
        let start = self.pos;
        while let Some(b) = self.peek() {
            if matches!(b, b' ' | b'\t' | b'\r' | b'\n') {
                self.pos += 1;
            } else {
                break;
            }
        }
        if self.pos > start {
            self.record_canon_err(
                RULE_JSON_WHITESPACE,
                start,
                "insignificant whitespace outside strings is forbidden".to_string(),
            );
        }
    }

    fn parse_value(&mut self) -> Result<JsonValue, JsonError> {
        let Some(b) = self.peek() else {
            return Err(self.err(
                RULE_JSON_SYNTAX,
                None,
                self.pos,
                "unexpected end of input".to_string(),
            ));
        };
        match b {
            b'{' => self.parse_object(),
            b'[' => self.parse_array(),
            b'"' => self.parse_string().map(JsonValue::Str),
            b't' => self.parse_literal(b"true", JsonValue::Bool(true)),
            b'f' => self.parse_literal(b"false", JsonValue::Bool(false)),
            b'n' => self.parse_literal(b"null", JsonValue::Null),
            b'-' | b'0'..=b'9' => self.parse_number(),
            other => Err(self.err(
                RULE_JSON_SYNTAX,
                None,
                self.pos,
                format!("unexpected byte 0x{other:02x} expecting JSON value"),
            )),
        }
    }

    fn parse_literal(&mut self, expected: &[u8], value: JsonValue) -> Result<JsonValue, JsonError> {
        let start = self.pos;
        if self.bytes[start..].starts_with(expected) {
            self.pos += expected.len();
            Ok(value)
        } else {
            Err(self.err(
                RULE_JSON_SYNTAX,
                None,
                start,
                format!("invalid literal at byte {start}"),
            ))
        }
    }

    fn parse_array(&mut self) -> Result<JsonValue, JsonError> {
        let open_pos = self.pos;
        self.depth += 1;
        if self.depth > self.limits.max_json_depth {
            return Err(self.err(
                RULE_LIMITS_EXCEEDED,
                None,
                open_pos,
                format!(
                    "JSON nesting depth {} exceeds limit {}",
                    self.depth, self.limits.max_json_depth
                ),
            ));
        }
        self.pos += 1; // consume '['
        self.skip_ws();
        let mut items = Vec::new();
        if self.peek() == Some(b']') {
            self.pos += 1;
            self.depth -= 1;
            return Ok(JsonValue::Arr(items));
        }
        let mut idx = 0_usize;
        loop {
            self.path.push(PathSeg::Index(idx));
            let val = self.parse_value()?;
            self.path.pop();
            items.push(val);
            idx += 1;
            self.skip_ws();
            match self.peek() {
                Some(b',') => {
                    self.pos += 1;
                    self.skip_ws();
                }
                Some(b']') => {
                    self.pos += 1;
                    self.depth -= 1;
                    return Ok(JsonValue::Arr(items));
                }
                _ => {
                    return Err(self.err(
                        RULE_JSON_SYNTAX,
                        None,
                        self.pos,
                        "expected ',' or ']' in JSON array".to_string(),
                    ));
                }
            }
        }
    }

    fn parse_object(&mut self) -> Result<JsonValue, JsonError> {
        let open_pos = self.pos;
        self.depth += 1;
        if self.depth > self.limits.max_json_depth {
            return Err(self.err(
                RULE_LIMITS_EXCEEDED,
                None,
                open_pos,
                format!(
                    "JSON nesting depth {} exceeds limit {}",
                    self.depth, self.limits.max_json_depth
                ),
            ));
        }
        self.pos += 1; // consume '{'
        self.skip_ws();
        let mut entries: Vec<(String, JsonValue)> = Vec::new();
        if self.peek() == Some(b'}') {
            self.pos += 1;
            self.depth -= 1;
            return Ok(JsonValue::Obj(entries));
        }
        let mut seen: HashSet<String> = HashSet::new();
        let mut prev_key: Option<String> = None;
        loop {
            let key_pos = self.pos;
            if self.peek() != Some(b'"') {
                return Err(self.err(
                    RULE_JSON_SYNTAX,
                    None,
                    key_pos,
                    "expected string key in JSON object".to_string(),
                ));
            }
            let key = self.parse_string()?;
            self.skip_ws();
            if self.peek() != Some(b':') {
                return Err(self.err(
                    RULE_JSON_SYNTAX,
                    None,
                    self.pos,
                    "expected ':' after JSON object key".to_string(),
                ));
            }
            self.pos += 1; // consume ':'
            self.skip_ws();

            let is_dup = seen.contains(&key);
            if is_dup && !self.opts.dup_first_wins {
                return Err(self.err(
                    RULE_JSON_DUPLICATE_KEY,
                    None,
                    key_pos,
                    format!("duplicate object key {key:?} (after escape decoding)"),
                ));
            }

            if !is_dup {
                if !self.opts.skip_key_order {
                    if let Some(prev) = &prev_key {
                        if key.as_bytes() < prev.as_bytes() {
                            self.record_canon_err(
                                RULE_JSON_KEY_ORDER,
                                key_pos,
                                format!("object key {key:?} precedes previous key {prev:?}"),
                            );
                        }
                    }
                }
                seen.insert(key.clone());
                prev_key = Some(key.clone());
            }

            self.path.push(PathSeg::Key(key.clone()));
            let val = self.parse_value()?;
            self.path.pop();

            if !is_dup {
                entries.push((key, val));
            }

            self.skip_ws();
            match self.peek() {
                Some(b',') => {
                    self.pos += 1;
                    self.skip_ws();
                }
                Some(b'}') => {
                    self.pos += 1;
                    self.depth -= 1;
                    return Ok(JsonValue::Obj(entries));
                }
                _ => {
                    return Err(self.err(
                        RULE_JSON_SYNTAX,
                        None,
                        self.pos,
                        "expected ',' or '}' in JSON object".to_string(),
                    ));
                }
            }
        }
    }

    fn parse_string(&mut self) -> Result<String, JsonError> {
        debug_assert_eq!(self.peek(), Some(b'"'));
        self.pos += 1; // consume opening '"'
        let mut out = String::new();
        let mut chunk_start = self.pos;

        while self.pos < self.bytes.len() {
            let b = self.bytes[self.pos];
            if b == b'"' {
                if self.pos > chunk_start {
                    out.push_str(&self.src[chunk_start..self.pos]);
                }
                self.pos += 1;
                return Ok(out);
            }
            if b < 0x20 {
                return Err(self.err(
                    RULE_JSON_SYNTAX,
                    None,
                    self.pos,
                    format!("unescaped control byte 0x{b:02x} in JSON string"),
                ));
            }
            if b == b'\\' {
                if self.pos > chunk_start {
                    out.push_str(&self.src[chunk_start..self.pos]);
                }
                let esc_pos = self.pos;
                self.pos += 1;
                let Some(esc) = self.peek() else {
                    return Err(self.err(
                        RULE_JSON_SYNTAX,
                        None,
                        esc_pos,
                        "unterminated escape sequence in JSON string".to_string(),
                    ));
                };
                self.pos += 1;
                match esc {
                    b'"' => out.push('"'),
                    b'\\' => out.push('\\'),
                    b'/' => {
                        self.record_canon_err(
                            RULE_JSON_ESCAPE,
                            esc_pos,
                            "non-minimal escape '\\/' (must be raw '/')".to_string(),
                        );
                        out.push('/');
                    }
                    b'b' => {
                        self.record_canon_err(
                            RULE_JSON_ESCAPE,
                            esc_pos,
                            "non-minimal escape '\\b' (must be '\\u0008')".to_string(),
                        );
                        out.push('\u{0008}');
                    }
                    b'f' => {
                        self.record_canon_err(
                            RULE_JSON_ESCAPE,
                            esc_pos,
                            "non-minimal escape '\\f' (must be '\\u000c')".to_string(),
                        );
                        out.push('\u{000C}');
                    }
                    b'n' => {
                        self.record_canon_err(
                            RULE_JSON_ESCAPE,
                            esc_pos,
                            "non-minimal escape '\\n' (must be '\\u000a')".to_string(),
                        );
                        out.push('\n');
                    }
                    b'r' => {
                        self.record_canon_err(
                            RULE_JSON_ESCAPE,
                            esc_pos,
                            "non-minimal escape '\\r' (must be '\\u000d')".to_string(),
                        );
                        out.push('\r');
                    }
                    b't' => {
                        self.record_canon_err(
                            RULE_JSON_ESCAPE,
                            esc_pos,
                            "non-minimal escape '\\t' (must be '\\u0009')".to_string(),
                        );
                        out.push('\t');
                    }
                    b'u' => {
                        let (code, all_lower) = self.parse_hex4(esc_pos)?;
                        if (0xD800..=0xDBFF).contains(&code) {
                            // High surrogate: must be immediately followed by \uXXXX low surrogate.
                            if self.pos + 2 > self.bytes.len()
                                || self.bytes[self.pos] != b'\\'
                                || self.bytes[self.pos + 1] != b'u'
                            {
                                return Err(self.err(
                                    RULE_JSON_SYNTAX,
                                    None,
                                    esc_pos,
                                    "unpaired high UTF-16 surrogate in JSON string".to_string(),
                                ));
                            }
                            self.pos += 2;
                            let (low, _) = self.parse_hex4(esc_pos)?;
                            if !(0xDC00..=0xDFFF).contains(&low) {
                                return Err(self.err(
                                    RULE_JSON_SYNTAX,
                                    None,
                                    esc_pos,
                                    "invalid low UTF-16 surrogate in JSON string".to_string(),
                                ));
                            }
                            let scalar = 0x10000 + (((code - 0xD800) << 10) | (low - 0xDC00));
                            let ch = char::from_u32(scalar).ok_or_else(|| {
                                self.err(
                                    RULE_JSON_SYNTAX,
                                    None,
                                    esc_pos,
                                    "invalid Unicode scalar from surrogate pair".to_string(),
                                )
                            })?;
                            self.record_canon_err(
                                RULE_JSON_ESCAPE,
                                esc_pos,
                                "non-minimal surrogate escape (must be raw UTF-8)".to_string(),
                            );
                            out.push(ch);
                        } else if (0xDC00..=0xDFFF).contains(&code) {
                            return Err(self.err(
                                RULE_JSON_SYNTAX,
                                None,
                                esc_pos,
                                "unpaired low UTF-16 surrogate in JSON string".to_string(),
                            ));
                        } else {
                            let ch = char::from_u32(code).ok_or_else(|| {
                                self.err(
                                    RULE_JSON_SYNTAX,
                                    None,
                                    esc_pos,
                                    "invalid Unicode scalar in \\uXXXX escape".to_string(),
                                )
                            })?;
                            if code >= 0x20 || !all_lower {
                                self.record_canon_err(
                                    RULE_JSON_ESCAPE,
                                    esc_pos,
                                    format!(
                                        "non-minimal \\u escape for U+{code:04X} (only lowercase \\u00xx for < 0x20)"
                                    ),
                                );
                            }
                            out.push(ch);
                        }
                    }
                    other => {
                        return Err(self.err(
                            RULE_JSON_SYNTAX,
                            None,
                            esc_pos,
                            format!("invalid escape byte 0x{other:02x} in JSON string"),
                        ));
                    }
                }
                chunk_start = self.pos;
                continue;
            }
            self.pos += 1;
        }

        Err(self.err(
            RULE_JSON_SYNTAX,
            None,
            self.pos,
            "unterminated JSON string".to_string(),
        ))
    }

    fn parse_hex4(&mut self, esc_pos: usize) -> Result<(u32, bool), JsonError> {
        if self.pos + 4 > self.bytes.len() {
            return Err(self.err(
                RULE_JSON_SYNTAX,
                None,
                esc_pos,
                "truncated \\uXXXX escape".to_string(),
            ));
        }
        let mut val = 0_u32;
        let mut all_lower = true;
        for i in 0..4 {
            let b = self.bytes[self.pos + i];
            let digit = match b {
                b'0'..=b'9' => u32::from(b - b'0'),
                b'a'..=b'f' => u32::from(b - b'a' + 10),
                b'A'..=b'F' => {
                    all_lower = false;
                    u32::from(b - b'A' + 10)
                }
                _ => {
                    return Err(self.err(
                        RULE_JSON_SYNTAX,
                        None,
                        self.pos + i,
                        "non-hex byte in \\uXXXX escape".to_string(),
                    ));
                }
            };
            val = (val << 4) | digit;
        }
        self.pos += 4;
        Ok((val, all_lower))
    }

    fn parse_number(&mut self) -> Result<JsonValue, JsonError> {
        let start = self.pos;
        let neg = if self.peek() == Some(b'-') {
            self.pos += 1;
            true
        } else {
            false
        };

        let digits_start = self.pos;
        match self.peek() {
            Some(b'0') => {
                self.pos += 1;
                if matches!(self.peek(), Some(b'0'..=b'9')) {
                    return Err(self.err(
                        RULE_JSON_SYNTAX,
                        None,
                        start,
                        "leading zero in JSON number".to_string(),
                    ));
                }
            }
            Some(b'1'..=b'9') => {
                self.pos += 1;
                while matches!(self.peek(), Some(b'0'..=b'9')) {
                    self.pos += 1;
                }
            }
            _ => {
                return Err(self.err(
                    RULE_JSON_SYNTAX,
                    None,
                    start,
                    "expected digit in JSON number".to_string(),
                ));
            }
        }
        let digits_end = self.pos;

        // Check for fraction or exponent: validate grammar, then refuse as float-in-bundle.
        let mut is_float = false;
        if self.peek() == Some(b'.') {
            is_float = true;
            self.pos += 1;
            if !matches!(self.peek(), Some(b'0'..=b'9')) {
                return Err(self.err(
                    RULE_JSON_SYNTAX,
                    None,
                    self.pos,
                    "expected digit after decimal point".to_string(),
                ));
            }
            while matches!(self.peek(), Some(b'0'..=b'9')) {
                self.pos += 1;
            }
        }
        if matches!(self.peek(), Some(b'e' | b'E')) {
            is_float = true;
            self.pos += 1;
            if matches!(self.peek(), Some(b'+' | b'-')) {
                self.pos += 1;
            }
            if !matches!(self.peek(), Some(b'0'..=b'9')) {
                return Err(self.err(
                    RULE_JSON_SYNTAX,
                    None,
                    self.pos,
                    "expected digit in number exponent".to_string(),
                ));
            }
            while matches!(self.peek(), Some(b'0'..=b'9')) {
                self.pos += 1;
            }
        }

        if is_float {
            let raw = &self.src[start..self.pos];
            return Err(self.err(
                RULE_NUM_FLOAT_IN_BUNDLE,
                Some(REFUSAL_ARITHMETIC_MALFORMED),
                start,
                format!("floating-point or exponential JSON number {raw:?} is forbidden"),
            ));
        }

        let digits = &self.src[digits_start..digits_end];
        if neg && digits == "0" {
            return Err(self.err(
                RULE_NUM_INT_FORM,
                Some(REFUSAL_ARITHMETIC_MALFORMED),
                start,
                "negative zero '-0' is forbidden".to_string(),
            ));
        }

        // Enforce ±(2^53 - 1) bound (MAX_JSON_INT = 9007199254740991, 16 digits).
        if digits.len() > 16 {
            let raw = &self.src[start..digits_end];
            return Err(self.err(
                RULE_NUM_INT_RANGE,
                Some(REFUSAL_ARITHMETIC_MALFORMED),
                start,
                format!("JSON integer {raw} exceeds ±(2^53 - 1)"),
            ));
        }
        let mag: u64 = digits.parse().map_err(|_| {
            self.err(
                RULE_NUM_INT_RANGE,
                Some(REFUSAL_ARITHMETIC_MALFORMED),
                start,
                "JSON integer overflow".to_string(),
            )
        })?;
        if mag > MAX_JSON_INT as u64 {
            let raw = &self.src[start..digits_end];
            return Err(self.err(
                RULE_NUM_INT_RANGE,
                Some(REFUSAL_ARITHMETIC_MALFORMED),
                start,
                format!("JSON integer {raw} exceeds ±(2^53 - 1)"),
            ));
        }

        self.numeric_entries += 1;
        if self.numeric_entries > self.limits.max_numeric_entries {
            return Err(self.err(
                RULE_LIMITS_EXCEEDED,
                None,
                start,
                format!(
                    "numeric entry count {} exceeds limit {}",
                    self.numeric_entries, self.limits.max_numeric_entries
                ),
            ));
        }

        let signed = if neg { -(mag as i64) } else { mag as i64 };
        Ok(JsonValue::Int(signed))
    }
}

#[cfg(test)]
mod nc_tests {
    use super::*;
    use crate::limits::COMPILED_LIMITS;

    #[test]
    fn nc_skip_key_order_mutant_accepts_unsorted() {
        let unsorted = br#"{"b":1,"a":2}"#;
        let normal = parse_canonical(unsorted, &COMPILED_LIMITS);
        assert_eq!(normal.unwrap_err().rule, RULE_JSON_KEY_ORDER);

        let mutant = parse_skip_key_order_mutant(unsorted, &COMPILED_LIMITS);
        assert!(
            mutant.is_ok(),
            "skip-key-order mutant must accept unsorted keys so negative control fires"
        );
    }

    #[test]
    fn nc_dup_key_first_wins_mutant_accepts_duplicate() {
        let dup = br#"{"wheel_radius_mm":60,"wheel_radius_mm":75}"#;
        let normal = parse_canonical(dup, &COMPILED_LIMITS);
        assert_eq!(normal.unwrap_err().rule, RULE_JSON_DUPLICATE_KEY);

        let mutant = parse_dup_first_wins_mutant(dup, &COMPILED_LIMITS).unwrap();
        assert_eq!(
            mutant.get("wheel_radius_mm").and_then(JsonValue::as_i64),
            Some(60),
            "first-wins mutant must keep 60 and accept duplicate"
        );
    }
}
