//! Strict `std`-only RFC 8259 JSON parser, canonical writer, RFC 6901 pointer,
//! and redaction tree checker for `frob-e2e`.

use std::collections::BTreeSet;
use std::fmt;

pub const FORBIDDEN_MEMBERS: &[&str] = &[
    "voxels", "occupancy", "points", "mesh", "image", "pixels",
    "depth_map", "rgb", "scan", "floorplan", "pose_trace",
];
pub const MAX_STRING_BYTES: usize = 4096;
pub const MAX_LINE_BYTES: usize = 65536;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JsonValue {
    Null, Bool(bool), Number(String), String(String),
    Array(Vec<JsonValue>), Object(Vec<(String, JsonValue)>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsonError { pub offset: usize, pub message: String }

impl fmt::Display for JsonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "JSON error at byte {}: {}", self.offset, self.message) }
}
impl std::error::Error for JsonError {}

impl JsonValue {
    pub fn as_str(&self) -> Option<&str> { match self { Self::String(s) => Some(s), _ => None } }
    pub fn as_bool(&self) -> Option<bool> { match self { Self::Bool(b) => Some(*b), _ => None } }
    pub fn as_num_str(&self) -> Option<&str> { match self { Self::Number(n) => Some(n), _ => None } }
    pub fn as_i64(&self) -> Option<i64> { self.as_num_str()?.parse().ok() }
    pub fn as_u64(&self) -> Option<u64> { self.as_num_str()?.parse().ok() }
    pub fn as_array(&self) -> Option<&[JsonValue]> { match self { Self::Array(a) => Some(a), _ => None } }
    pub fn as_object(&self) -> Option<&[(String, JsonValue)]> { match self { Self::Object(o) => Some(o), _ => None } }
    pub fn get(&self, key: &str) -> Option<&JsonValue> { self.as_object()?.iter().find(|(k, _)| k == key).map(|(_, v)| v) }

    pub fn pointer(&self, ptr: &str) -> Option<&JsonValue> {
        if ptr.is_empty() { return Some(self); }
        let mut cur = self;
        for raw_token in ptr.strip_prefix('/')?.split('/') {
            let token = unescape_pointer_token(raw_token)?;
            match cur {
                Self::Object(pairs) => cur = pairs.iter().find(|(k, _)| k == &token).map(|(_, v)| v)?,
                Self::Array(items) => {
                    if token.is_empty() || !token.bytes().all(|b| b.is_ascii_digit()) || (token.len() > 1 && token.starts_with('0')) { return None; }
                    cur = items.get(token.parse::<usize>().ok()?)?;
                }
                _ => return None,
            }
        }
        Some(cur)
    }

    pub fn to_json_string(&self) -> String { let mut out = String::new(); self.write_compact(&mut out); out }
    pub fn to_json_pretty(&self) -> String { let mut out = String::new(); self.write_pretty(&mut out, 0); out }

    fn write_compact(&self, out: &mut String) {
        match self {
            Self::Null => out.push_str("null"),
            Self::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
            Self::Number(n) => out.push_str(n),
            Self::String(s) => write_escaped_string(s, out),
            Self::Array(items) => {
                out.push('[');
                for (i, v) in items.iter().enumerate() { if i > 0 { out.push(','); } v.write_compact(out); }
                out.push(']');
            }
            Self::Object(pairs) => {
                out.push('{');
                for (i, (k, v)) in pairs.iter().enumerate() {
                    if i > 0 { out.push(','); }
                    write_escaped_string(k, out); out.push(':'); v.write_compact(out);
                }
                out.push('}');
            }
        }
    }

    fn write_pretty(&self, out: &mut String, indent: usize) {
        match self {
            Self::Array(items) if !items.is_empty() => {
                out.push_str("[\n");
                for (i, v) in items.iter().enumerate() {
                    if i > 0 { out.push_str(",\n"); }
                    out.push_str(&" ".repeat(indent + 2));
                    v.write_pretty(out, indent + 2);
                }
                out.push('\n'); out.push_str(&" ".repeat(indent)); out.push(']');
            }
            Self::Object(pairs) if !pairs.is_empty() => {
                out.push_str("{\n");
                for (i, (k, v)) in pairs.iter().enumerate() {
                    if i > 0 { out.push_str(",\n"); }
                    out.push_str(&" ".repeat(indent + 2));
                    write_escaped_string(k, out); out.push_str(": ");
                    v.write_pretty(out, indent + 2);
                }
                out.push('\n'); out.push_str(&" ".repeat(indent)); out.push('}');
            }
            _ => self.write_compact(out),
        }
    }
}

fn unescape_pointer_token(raw: &str) -> Option<String> {
    let (mut out, mut chars) = (String::with_capacity(raw.len()), raw.chars());
    while let Some(ch) = chars.next() {
        if ch == '~' {
            match chars.next() { Some('0') => out.push('~'), Some('1') => out.push('/'), _ => return None }
        } else { out.push(ch); }
    }
    Some(out)
}

pub fn write_escaped_string(s: &str, out: &mut String) {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    out.push('"');
    for ch in s.chars() {
        match ch {
            '"' => out.push_str("\\\""), '\\' => out.push_str("\\\\"), '\u{0008}' => out.push_str("\\b"),
            '\u{000C}' => out.push_str("\\f"), '\n' => out.push_str("\\n"), '\r' => out.push_str("\\r"), '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => {
                let code = c as u32;
                out.push_str("\\u00");
                out.push(HEX[((code >> 4) & 0xf) as usize] as char);
                out.push(HEX[(code & 0xf) as usize] as char);
            }
            c => out.push(c),
        }
    }
    out.push('"');
}

pub fn obj(pairs: Vec<(&str, JsonValue)>) -> JsonValue {
    JsonValue::Object(pairs.into_iter().map(|(k, v)| (k.to_string(), v)).collect())
}

pub fn skip_ws(bytes: &[u8], mut pos: usize) -> usize {
    while pos < bytes.len() && matches!(bytes[pos], b' ' | b'\t' | b'\r' | b'\n') { pos += 1; }
    pos
}

pub fn parse_strict(input: &str) -> Result<JsonValue, JsonError> {
    let (val, end) = parse_prefix_with_offset(input)?;
    let trailing = skip_ws(input.as_bytes(), end);
    if trailing != input.len() { return Err(JsonError { offset: trailing, message: "trailing content after JSON value".into() }); }
    Ok(val)
}

pub fn parse_prefix_with_offset(input: &str) -> Result<(JsonValue, usize), JsonError> {
    let mut p = Parser { src: input, bytes: input.as_bytes(), pos: 0, depth: 0 };
    p.skip_ws();
    let val = p.parse_value()?;
    Ok((val, p.pos))
}

struct Parser<'a> { src: &'a str, bytes: &'a [u8], pos: usize, depth: usize }

impl<'a> Parser<'a> {
    fn err(&self, msg: impl Into<String>) -> JsonError { JsonError { offset: self.pos, message: msg.into() } }
    fn skip_ws(&mut self) { self.pos = skip_ws(self.bytes, self.pos); }
    fn peek(&self) -> Option<u8> { self.bytes.get(self.pos).copied() }

    fn parse_value(&mut self) -> Result<JsonValue, JsonError> {
        if self.depth > 128 { return Err(self.err("JSON nesting depth exceeds 128")); }
        match self.peek().ok_or_else(|| self.err("unexpected end of input"))? {
            b'n' => self.parse_lit(b"null", JsonValue::Null),
            b't' => self.parse_lit(b"true", JsonValue::Bool(true)),
            b'f' => self.parse_lit(b"false", JsonValue::Bool(false)),
            b'"' => self.parse_string().map(JsonValue::String),
            b'[' => self.parse_array(),
            b'{' => self.parse_object(),
            b'-' | b'0'..=b'9' => self.parse_number(),
            b => Err(self.err(format!("unexpected byte 0x{b:02x}"))),
        }
    }

    fn parse_lit(&mut self, lit: &[u8], val: JsonValue) -> Result<JsonValue, JsonError> {
        if self.bytes[self.pos..].starts_with(lit) { self.pos += lit.len(); Ok(val) } else { Err(self.err("invalid literal")) }
    }

    fn parse_number(&mut self) -> Result<JsonValue, JsonError> {
        let start = self.pos;
        if self.peek() == Some(b'-') { self.pos += 1; }
        match self.peek() {
            Some(b'0') => { self.pos += 1; if matches!(self.peek(), Some(b'0'..=b'9')) { return Err(self.err("leading zero in number")); } }
            Some(b'1'..=b'9') => { self.pos += 1; while matches!(self.peek(), Some(b'0'..=b'9')) { self.pos += 1; } }
            _ => return Err(self.err("invalid number")),
        }
        if self.peek() == Some(b'.') {
            self.pos += 1;
            if !matches!(self.peek(), Some(b'0'..=b'9')) { return Err(self.err("missing fractional digits")); }
            while matches!(self.peek(), Some(b'0'..=b'9')) { self.pos += 1; }
        }
        if matches!(self.peek(), Some(b'e' | b'E')) {
            self.pos += 1;
            if matches!(self.peek(), Some(b'+' | b'-')) { self.pos += 1; }
            if !matches!(self.peek(), Some(b'0'..=b'9')) { return Err(self.err("missing exponent digits")); }
            while matches!(self.peek(), Some(b'0'..=b'9')) { self.pos += 1; }
        }
        Ok(JsonValue::Number(self.src[start..self.pos].to_string()))
    }

    fn parse_hex4(&mut self) -> Result<u16, JsonError> {
        if self.pos + 4 > self.bytes.len() { return Err(self.err("truncated \\u escape")); }
        let mut val = 0u16;
        for i in 0..4 {
            let d = match self.bytes[self.pos + i] {
                b @ b'0'..=b'9' => (b - b'0') as u16,
                b @ b'a'..=b'f' => (b - b'a' + 10) as u16,
                b @ b'A'..=b'F' => (b - b'A' + 10) as u16,
                _ => return Err(self.err("invalid hex digit in \\u escape")),
            };
            val = (val << 4) | d;
        }
        self.pos += 4;
        Ok(val)
    }

    fn parse_string(&mut self) -> Result<String, JsonError> {
        if self.peek() != Some(b'"') { return Err(self.err("expected '\"'")); }
        self.pos += 1;
        let mut out = String::new();
        while self.pos < self.bytes.len() {
            match self.bytes[self.pos] {
                b'"' => { self.pos += 1; return Ok(out); }
                b'\\' => {
                    self.pos += 1;
                    let esc = self.peek().ok_or_else(|| self.err("unterminated escape"))?;
                    self.pos += 1;
                    match esc {
                        b'"' => out.push('"'), b'\\' => out.push('\\'), b'/' => out.push('/'),
                        b'b' => out.push('\u{0008}'), b'f' => out.push('\u{000C}'),
                        b'n' => out.push('\n'), b'r' => out.push('\r'), b't' => out.push('\t'),
                        b'u' => {
                            let u1 = self.parse_hex4()?;
                            let cp = if (0xD800..=0xDBFF).contains(&u1) {
                                if !self.bytes[self.pos..].starts_with(b"\\u") { return Err(self.err("unpaired high surrogate")); }
                                self.pos += 2;
                                let u2 = self.parse_hex4()?;
                                if !(0xDC00..=0xDFFF).contains(&u2) { return Err(self.err("invalid low surrogate")); }
                                0x10000 + (((u1 as u32 - 0xD800) << 10) | (u2 as u32 - 0xDC00))
                            } else if (0xDC00..=0xDFFF).contains(&u1) { return Err(self.err("unpaired low surrogate")); }
                            else { u1 as u32 };
                            out.push(char::from_u32(cp).ok_or_else(|| self.err("invalid unicode scalar"))?);
                        }
                        _ => return Err(self.err("invalid string escape")),
                    }
                }
                0x00..=0x1F => return Err(self.err("unescaped control character in string")),
                _ => { let ch = self.src[self.pos..].chars().next().unwrap(); out.push(ch); self.pos += ch.len_utf8(); }
            }
        }
        Err(self.err("unterminated string"))
    }

    fn parse_array(&mut self) -> Result<JsonValue, JsonError> {
        self.pos += 1; self.depth += 1; self.skip_ws();
        let mut items = Vec::new();
        if self.peek() == Some(b']') { self.pos += 1; self.depth -= 1; return Ok(JsonValue::Array(items)); }
        loop {
            items.push(self.parse_value()?);
            self.skip_ws();
            match self.peek() {
                Some(b',') => { self.pos += 1; self.skip_ws(); }
                Some(b']') => { self.pos += 1; self.depth -= 1; return Ok(JsonValue::Array(items)); }
                _ => return Err(self.err("expected ',' or ']' in array")),
            }
        }
    }

    fn parse_object(&mut self) -> Result<JsonValue, JsonError> {
        self.pos += 1; self.depth += 1; self.skip_ws();
        let (mut pairs, mut seen) = (Vec::new(), BTreeSet::new());
        if self.peek() == Some(b'}') { self.pos += 1; self.depth -= 1; return Ok(JsonValue::Object(pairs)); }
        loop {
            let key_off = self.pos;
            let key = self.parse_string()?;
            if !seen.insert(key.clone()) { return Err(JsonError { offset: key_off, message: format!("duplicate object member: {key}") }); }
            self.skip_ws();
            if self.peek() != Some(b':') { return Err(self.err("expected ':' after object key")); }
            self.pos += 1; self.skip_ws();
            pairs.push((key, self.parse_value()?));
            self.skip_ws();
            match self.peek() {
                Some(b',') => { self.pos += 1; self.skip_ws(); }
                Some(b'}') => { self.pos += 1; self.depth -= 1; return Ok(JsonValue::Object(pairs)); }
                _ => return Err(self.err("expected ',' or '}' in object")),
            }
        }
    }
}

pub fn check_redaction_tree(val: &JsonValue, path: &str) -> Result<(), String> {
    match val {
        JsonValue::String(s) if s.len() > MAX_STRING_BYTES => Err(format!(
            "string at {} exceeds {MAX_STRING_BYTES} bytes ({} bytes)", if path.is_empty() { "/" } else { path }, s.len()
        )),
        JsonValue::Array(items) => {
            for (i, item) in items.iter().enumerate() { check_redaction_tree(item, &format!("{path}/{i}"))?; }
            Ok(())
        }
        JsonValue::Object(pairs) => {
            for (k, v) in pairs {
                let child = format!("{path}/{k}");
                if FORBIDDEN_MEMBERS.contains(&k.as_str()) { return Err(format!("forbidden private member '{k}' at {child}")); }
                check_redaction_tree(v, &child)?;
            }
            Ok(())
        }
        _ => Ok(()),
    }
}

pub fn validate_line_for_emit(val: &JsonValue) -> Result<String, String> {
    check_redaction_tree(val, "")?;
    let serialized = val.to_json_string();
    if serialized.len() > MAX_LINE_BYTES { return Err(format!("serialized log line exceeds {MAX_LINE_BYTES} bytes ({})", serialized.len())); }
    Ok(serialized)
}
