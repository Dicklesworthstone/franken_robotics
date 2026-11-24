//! Strict TOML-subset reader for FrankenRobotics registries, locks, and manifests (bead N34p2-01).
//!
//! Accepts a strict subset of TOML 1.0.0 so every accepted document is valid TOML 1.0.0
//! with identical semantics. Constructs valid in TOML 1.0.0 outside the subset are
//! refused with [`TomlErrorKind::OutsideSubset`]; malformed TOML 1.0.0 is refused with
//! [`TomlErrorKind::Syntax`] or another non-subset kind. Pure function of input bytes
//! (determinism class D0); prints nothing (rule L-7) and uses zero `unsafe`.

use std::collections::BTreeMap;
use std::fmt;
use std::num::IntErrorKind;

/// Maximum accepted input size in bytes (16 MiB).
pub const MAX_INPUT_BYTES: usize = 16 * 1024 * 1024;

/// Maximum allowed nesting depth of arrays and inline tables.
pub const MAX_NESTING_DEPTH: usize = 64;

/// A value together with its 1-based source line and Unicode-scalar column.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Spanned<T> {
    pub value: T,
    pub line: u32,
    pub column: u32,
}

impl<T> Spanned<T> {
    #[must_use]
    pub const fn new(value: T, line: u32, column: u32) -> Self {
        Self { value, line, column }
    }
}

/// Strict TOML-subset value tree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Str(String),
    Int(i64),
    Bool(bool),
    Array(Vec<Spanned<Value>>),
    Table(Table),
}

impl Value {
    #[must_use]
    pub const fn type_name(&self) -> &'static str {
        match self {
            Self::Str(_) => "string",
            Self::Int(_) => "integer",
            Self::Bool(_) => "boolean",
            Self::Array(_) => "array",
            Self::Table(_) => "table",
        }
    }

    #[must_use]
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::Str(s) => Some(s),
            _ => None,
        }
    }

    #[must_use]
    pub const fn as_int(&self) -> Option<i64> {
        match self {
            Self::Int(v) => Some(*v),
            _ => None,
        }
    }

    #[must_use]
    pub const fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(v) => Some(*v),
            _ => None,
        }
    }

    #[must_use]
    pub fn as_array(&self) -> Option<&[Spanned<Value>]> {
        match self {
            Self::Array(v) => Some(v),
            _ => None,
        }
    }

    #[must_use]
    pub const fn as_table(&self) -> Option<&Table> {
        match self {
            Self::Table(t) => Some(t),
            _ => None,
        }
    }

    /// Structural value equality ignoring source line/column spans.
    #[must_use]
    pub fn eq_values(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Str(a), Self::Str(b)) => a == b,
            (Self::Int(a), Self::Int(b)) => a == b,
            (Self::Bool(a), Self::Bool(b)) => a == b,
            (Self::Array(a), Self::Array(b)) => {
                a.len() == b.len() && a.iter().zip(b).all(|(x, y)| x.value.eq_values(&y.value))
            }
            (Self::Table(a), Self::Table(b)) => a.eq_values(b),
            _ => false,
        }
    }
}

/// Ordered key-to-spanned-value mapping for a TOML table.
///
/// Key order inside a table carries no semantic meaning; keys are stored in a
/// `BTreeMap` for deterministic iteration (D0) while every value preserves its
/// source `(line, column)` for diagnostics and source-order unknown-key errors.
#[derive(Debug, Clone, Eq)]
pub struct Table {
    span: (u32, u32),
    entries: BTreeMap<String, Spanned<Value>>,
}

impl PartialEq for Table {
    fn eq(&self, other: &Self) -> bool {
        self.entries == other.entries
    }
}

impl Default for Table {
    fn default() -> Self {
        Self::new()
    }
}

impl Table {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            span: (1, 1),
            entries: BTreeMap::new(),
        }
    }

    #[must_use]
    pub fn from_entries(entries: BTreeMap<String, Spanned<Value>>) -> Self {
        Self { span: (1, 1), entries }
    }

    #[must_use]
    pub const fn span(&self) -> (u32, u32) {
        self.span
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    #[must_use]
    pub fn contains_key(&self, key: &str) -> bool {
        self.entries.contains_key(key)
    }

    #[must_use]
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.entries.get(key).map(|s| &s.value)
    }

    #[must_use]
    pub fn get_spanned(&self, key: &str) -> Option<&Spanned<Value>> {
        self.entries.get(key)
    }

    pub fn iter(&self) -> std::collections::btree_map::Iter<'_, String, Spanned<Value>> {
        self.entries.iter()
    }

    pub fn keys(&self) -> std::collections::btree_map::Keys<'_, String, Spanned<Value>> {
        self.entries.keys()
    }

    #[must_use]
    pub fn into_inner(self) -> BTreeMap<String, Spanned<Value>> {
        self.entries
    }

    /// Structural value equality ignoring all source positions.
    #[must_use]
    pub fn eq_values(&self, other: &Self) -> bool {
        if self.entries.len() != other.entries.len() {
            return false;
        }
        self.entries.iter().zip(other.entries.iter()).all(|((ka, va), (kb, vb))| {
            ka == kb && va.value.eq_values(&vb.value)
        })
    }
}

/// Parse or loader error with 1-based line and Unicode-scalar column.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TomlError {
    pub line: u32,
    pub column: u32,
    pub kind: TomlErrorKind,
}

impl TomlError {
    #[must_use]
    pub const fn new(line: u32, column: u32, kind: TomlErrorKind) -> Self {
        Self { line, column, kind }
    }

    /// Render `<path>:<line>:<column>: <message>` for file-backed loaders.
    #[must_use]
    pub fn with_path(&self, path: &str) -> String {
        format!("{path}:{self}")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TomlErrorKind {
    Syntax(String),
    DuplicateKey(String),
    TableRedefined(String),
    OutsideSubset(&'static str),
    IntegerOutOfRange,
    InvalidUtf8,
    ByteOrderMark,
    TooLarge,
    TooDeep,
    UnknownKey { table: String, key: String },
    MissingKey { table: String, key: String },
    TypeMismatch { key: String, expected: &'static str, found: &'static str },
}

fn subset_fix_hint(construct: &str) -> &'static str {
    match construct {
        "dotted key" => "write `serde = { workspace = true }` or a `[section]` header instead",
        "literal string" => "use a basic double-quoted string `\"...\"`",
        "multi-line string" => "use a single-line basic string `\"...\"` with `\\n` escapes",
        "float" => "use a decimal integer or quoted string",
        "date" => "quote dates and timestamps as basic strings",
        "hexadecimal integer" | "octal integer" | "binary integer" => "write the integer in base-10 decimal",
        "underscore in integer" => "write decimal digits without underscores",
        "escape \\b" => "use `\\u0008` instead of `\\b`",
        "escape \\f" => "use `\\u000c` instead of `\\f`",
        "mixed-type array" => "all elements of an array must have the same value type",
        _ => "rewrite using the strict TOML subset",
    }
}

impl fmt::Display for TomlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}: ", self.line, self.column)?;
        match &self.kind {
            TomlErrorKind::Syntax(msg) => write!(f, "syntax error: {msg}"),
            TomlErrorKind::DuplicateKey(key) => {
                write!(f, "duplicate key `{key}` (remove or merge the repeated key)")
            }
            TomlErrorKind::TableRedefined(name) => {
                write!(f, "table `{name}` redefined or conflicts with an existing value")
            }
            TomlErrorKind::OutsideSubset(construct) => write!(
                f,
                "construct outside TOML subset: {construct} ({})",
                subset_fix_hint(construct)
            ),
            TomlErrorKind::IntegerOutOfRange => {
                f.write_str("integer out of signed 64-bit range (i64::MIN..=i64::MAX)")
            }
            TomlErrorKind::InvalidUtf8 => f.write_str("invalid UTF-8 byte sequence"),
            TomlErrorKind::ByteOrderMark => {
                f.write_str("UTF-8 byte-order mark (BOM) is forbidden; re-save as UTF-8 without BOM")
            }
            TomlErrorKind::TooLarge => {
                write!(f, "input exceeds maximum size of {MAX_INPUT_BYTES} bytes (16 MiB)")
            }
            TomlErrorKind::TooDeep => {
                write!(f, "nesting exceeds maximum depth of {MAX_NESTING_DEPTH}")
            }
            TomlErrorKind::UnknownKey { table, key } => {
                write!(f, "unknown key `{key}` in table `{table}` (check spelling or remove)")
            }
            TomlErrorKind::MissingKey { table, key } => {
                write!(f, "missing required key `{key}` in table `{table}`")
            }
            TomlErrorKind::TypeMismatch { key, expected, found } => {
                write!(f, "type mismatch for key `{key}`: expected {expected}, found {found}")
            }
        }
    }
}

impl std::error::Error for TomlError {}

/// Strict typed accessor over a [`Table`] that refuses unconsumed (unknown) keys.
///
/// Loaders consume each known key via `take` / `require_*` / `opt_*` and conclude
/// with [`Fields::finish`]. Dropping `Fields` without calling `finish` (and without
/// an earlier accessor error) panics in debug builds so tests catch forgotten calls.
#[must_use = "call `Fields::finish()` to refuse unknown keys"]
#[derive(Debug)]
pub struct Fields {
    table_name: String,
    table_line: u32,
    table_column: u32,
    remaining: BTreeMap<String, Spanned<Value>>,
    finished: bool,
    failed: bool,
}

impl Fields {
    pub fn new(table: Table, table_name: impl Into<String>) -> Self {
        Self {
            table_name: table_name.into(),
            table_line: table.span.0,
            table_column: table.span.1,
            remaining: table.entries,
            finished: false,
            failed: false,
        }
    }

    pub fn take(&mut self, key: &str) -> Option<Spanned<Value>> {
        self.remaining.remove(key)
    }

    fn missing(&mut self, key: &str) -> TomlError {
        self.failed = true;
        TomlError::new(
            self.table_line,
            self.table_column,
            TomlErrorKind::MissingKey {
                table: self.table_name.clone(),
                key: key.to_owned(),
            },
        )
    }

    fn mismatch(&mut self, key: &str, spanned: &Spanned<Value>, expected: &'static str) -> TomlError {
        self.failed = true;
        TomlError::new(
            spanned.line,
            spanned.column,
            TomlErrorKind::TypeMismatch {
                key: key.to_owned(),
                expected,
                found: spanned.value.type_name(),
            },
        )
    }

    pub fn opt_str(&mut self, key: &str) -> Result<Option<String>, TomlError> {
        let Some(sp) = self.take(key) else { return Ok(None) };
        match sp.value {
            Value::Str(s) => Ok(Some(s)),
            _ => Err(self.mismatch(key, &sp, "string")),
        }
    }

    pub fn require_str(&mut self, key: &str) -> Result<String, TomlError> {
        self.opt_str(key)?.ok_or_else(|| self.missing(key))
    }

    pub fn opt_int(&mut self, key: &str) -> Result<Option<i64>, TomlError> {
        let Some(sp) = self.take(key) else { return Ok(None) };
        match sp.value {
            Value::Int(v) => Ok(Some(v)),
            _ => Err(self.mismatch(key, &sp, "integer")),
        }
    }

    pub fn require_int(&mut self, key: &str) -> Result<i64, TomlError> {
        self.opt_int(key)?.ok_or_else(|| self.missing(key))
    }

    pub fn opt_bool(&mut self, key: &str) -> Result<Option<bool>, TomlError> {
        let Some(sp) = self.take(key) else { return Ok(None) };
        match sp.value {
            Value::Bool(v) => Ok(Some(v)),
            _ => Err(self.mismatch(key, &sp, "boolean")),
        }
    }

    pub fn require_bool(&mut self, key: &str) -> Result<bool, TomlError> {
        self.opt_bool(key)?.ok_or_else(|| self.missing(key))
    }

    pub fn opt_array(&mut self, key: &str) -> Result<Option<Vec<Spanned<Value>>>, TomlError> {
        let Some(sp) = self.take(key) else { return Ok(None) };
        match sp.value {
            Value::Array(v) => Ok(Some(v)),
            _ => Err(self.mismatch(key, &sp, "array")),
        }
    }

    pub fn require_array(&mut self, key: &str) -> Result<Vec<Spanned<Value>>, TomlError> {
        self.opt_array(key)?.ok_or_else(|| self.missing(key))
    }

    pub fn opt_table(&mut self, key: &str) -> Result<Option<Table>, TomlError> {
        let Some(sp) = self.take(key) else { return Ok(None) };
        match sp.value {
            Value::Table(t) => Ok(Some(t)),
            _ => Err(self.mismatch(key, &sp, "table")),
        }
    }

    pub fn require_table(&mut self, key: &str) -> Result<Table, TomlError> {
        self.opt_table(key)?.ok_or_else(|| self.missing(key))
    }

    /// Refuse any unconsumed key, reporting the first leftover key in source order.
    pub fn finish(mut self) -> Result<(), TomlError> {
        self.finished = true;
        if let Some((key, sp)) = self
            .remaining
            .iter()
            .min_by_key(|(k, v)| (v.line, v.column, k.as_str()))
        {
            return Err(TomlError::new(
                sp.line,
                sp.column,
                TomlErrorKind::UnknownKey {
                    table: self.table_name.clone(),
                    key: key.clone(),
                },
            ));
        }
        Ok(())
    }
}

impl Drop for Fields {
    fn drop(&mut self) {
        #[cfg(debug_assertions)]
        if !self.finished && !self.failed && !std::thread::panicking() {
            panic!(
                "registry::toml::Fields for table `{}` dropped without calling finish()",
                self.table_name
            );
        }
    }
}

/// Parse UTF-8 bytes in the strict FrankenRobotics TOML subset.
pub fn parse(bytes: &[u8]) -> Result<Table, TomlError> {
    if bytes.len() > MAX_INPUT_BYTES {
        return Err(TomlError::new(1, 1, TomlErrorKind::TooLarge));
    }
    if bytes.starts_with(&[0xEF, 0xBB, 0xBF]) {
        return Err(TomlError::new(1, 1, TomlErrorKind::ByteOrderMark));
    }
    let text = match std::str::from_utf8(bytes) {
        Ok(s) => s,
        Err(err) => {
            let prefix = std::str::from_utf8(&bytes[..err.valid_up_to()]).unwrap_or("");
            let (line, col) = line_col_of(prefix);
            return Err(TomlError::new(line, col, TomlErrorKind::InvalidUtf8));
        }
    };
    let mut parser = Parser::new(text);
    let root = parser.parse_document()?;
    Ok(root.into_table())
}

fn line_col_of(s: &str) -> (u32, u32) {
    let (mut line, mut col) = (1u32, 1u32);
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\n' {
            line = line.saturating_add(1);
            col = 1;
        } else if c == '\r' && chars.peek() == Some(&'\n') {
            chars.next();
            line = line.saturating_add(1);
            col = 1;
        } else {
            col = col.saturating_add(1);
        }
    }
    (line, col)
}

// ============================================================================
// Internal mutable AST with TOML 1.0.0 table/array origin tracking
// ============================================================================

#[derive(Clone, Copy, PartialEq, Eq)]
enum TableOrigin {
    Root,
    Implicit,
    Explicit,
    Inline,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ValueTag {
    Str,
    Int,
    Bool,
    Array,
    Table,
}

enum BuildNode {
    Str(String),
    Int(i64),
    Bool(bool),
    Array {
        items: Vec<Spanned<BuildNode>>,
        from_aot: bool,
    },
    Table(BuildTable),
}

impl BuildNode {
    const fn tag(&self) -> ValueTag {
        match self {
            Self::Str(_) => ValueTag::Str,
            Self::Int(_) => ValueTag::Int,
            Self::Bool(_) => ValueTag::Bool,
            Self::Array { .. } => ValueTag::Array,
            Self::Table(_) => ValueTag::Table,
        }
    }

    fn into_value(self) -> Value {
        match self {
            Self::Str(s) => Value::Str(s),
            Self::Int(v) => Value::Int(v),
            Self::Bool(v) => Value::Bool(v),
            Self::Array { items, .. } => Value::Array(
                items
                    .into_iter()
                    .map(|sp| Spanned::new(sp.value.into_value(), sp.line, sp.column))
                    .collect(),
            ),
            Self::Table(t) => Value::Table(t.into_table()),
        }
    }
}

struct BuildTable {
    origin: TableOrigin,
    span: (u32, u32),
    entries: BTreeMap<String, Spanned<BuildNode>>,
}

impl BuildTable {
    fn new(origin: TableOrigin, span: (u32, u32)) -> Self {
        Self {
            origin,
            span,
            entries: BTreeMap::new(),
        }
    }

    fn insert_kv(
        &mut self,
        key: String,
        node: BuildNode,
        line: u32,
        column: u32,
    ) -> Result<(), TomlError> {
        if self.entries.contains_key(&key) {
            return Err(TomlError::new(line, column, TomlErrorKind::DuplicateKey(key)));
        }
        self.entries.insert(key, Spanned::new(node, line, column));
        Ok(())
    }

    fn into_table(self) -> Table {
        let entries = self
            .entries
            .into_iter()
            .map(|(k, sp)| (k, Spanned::new(sp.value.into_value(), sp.line, sp.column)))
            .collect();
        Table {
            span: self.span,
            entries,
        }
    }
}

// ============================================================================
// Recursive-descent parser (strict TOML 1.0.0 subset)
// ============================================================================

struct Parser<'a> {
    src: &'a str,
    bytes: &'a [u8],
    pos: usize,
    line: u32,
    col: u32,
}

impl<'a> Parser<'a> {
    const fn new(src: &'a str) -> Self {
        Self {
            src,
            bytes: src.as_bytes(),
            pos: 0,
            line: 1,
            col: 1,
        }
    }

    fn peek_byte(&self) -> Option<u8> {
        self.bytes.get(self.pos).copied()
    }

    fn peek_byte_at(&self, offset: usize) -> Option<u8> {
        self.bytes.get(self.pos + offset).copied()
    }

    fn rest(&self) -> &'a str {
        self.src.get(self.pos..).unwrap_or("")
    }

    fn peek_char(&self) -> Option<char> {
        self.rest().chars().next()
    }

    fn next_char(&mut self) -> Option<char> {
        let ch = self.peek_char()?;
        self.pos += ch.len_utf8();
        if ch == '\n' {
            self.line = self.line.saturating_add(1);
            self.col = 1;
        } else {
            self.col = self.col.saturating_add(1);
        }
        Some(ch)
    }

    fn err(&self, kind: TomlErrorKind) -> TomlError {
        TomlError::new(self.line, self.col, kind)
    }

    fn syntax(&self, msg: impl Into<String>) -> TomlError {
        self.err(TomlErrorKind::Syntax(msg.into()))
    }

    fn skip_ws(&mut self) {
        while matches!(self.peek_byte(), Some(b' ' | b'\t')) {
            self.pos += 1;
            self.col = self.col.saturating_add(1);
        }
    }

    fn consume_newline(&mut self) -> Result<bool, TomlError> {
        match self.peek_byte() {
            Some(b'\n') => {
                self.pos += 1;
                self.line = self.line.saturating_add(1);
                self.col = 1;
                Ok(true)
            }
            Some(b'\r') => {
                if self.peek_byte_at(1) == Some(b'\n') {
                    self.pos += 2;
                    self.line = self.line.saturating_add(1);
                    self.col = 1;
                    Ok(true)
                } else {
                    Err(self.syntax("bare carriage return (use LF or CRLF)"))
                }
            }
            _ => Ok(false),
        }
    }

    fn skip_comment(&mut self) -> Result<(), TomlError> {
        if self.peek_byte() != Some(b'#') {
            return Ok(());
        }
        self.pos += 1;
        self.col = self.col.saturating_add(1);
        while let Some(ch) = self.peek_char() {
            if ch == '\n' {
                break;
            }
            if ch == '\r' {
                if self.peek_byte_at(1) == Some(b'\n') {
                    break;
                }
                return Err(self.syntax("bare carriage return inside comment"));
            }
            if is_disallowed_comment_char(ch) {
                return Err(self.syntax(format!(
                    "disallowed control character U+{:04X} in comment",
                    ch as u32
                )));
            }
            self.next_char();
        }
        Ok(())
    }

    fn skip_ws_newlines_comments(&mut self) -> Result<(), TomlError> {
        loop {
            self.skip_ws();
            match self.peek_byte() {
                Some(b'#') => self.skip_comment()?,
                Some(b'\n' | b'\r') => {
                    self.consume_newline()?;
                }
                _ => return Ok(()),
            }
        }
    }

    fn expect_eol_or_eof(&mut self) -> Result<(), TomlError> {
        self.skip_ws();
        if self.peek_byte() == Some(b'#') {
            self.skip_comment()?;
        }
        match self.peek_byte() {
            None => Ok(()),
            Some(b'\n' | b'\r') => {
                self.consume_newline()?;
                Ok(())
            }
            Some(_) => Err(self.syntax("unexpected trailing characters at end of line")),
        }
    }

    fn parse_document(&mut self) -> Result<BuildTable, TomlError> {
        let mut root = BuildTable::new(TableOrigin::Root, (1, 1));
        self.parse_table_body(&mut root)?;

        while self.peek_byte() == Some(b'[') {
            let (hdr_line, hdr_col) = (self.line, self.col);
            let (is_aot, path) = self.parse_header()?;
            let target = open_header_table(&mut root, &path, is_aot, (hdr_line, hdr_col))?;
            self.parse_table_body(target)?;
        }

        self.skip_ws_newlines_comments()?;
        if self.peek_byte().is_some() {
            return Err(self.syntax("unexpected trailing token"));
        }
        Ok(root)
    }

    fn parse_table_body(&mut self, table: &mut BuildTable) -> Result<(), TomlError> {
        loop {
            self.skip_ws_newlines_comments()?;
            match self.peek_byte() {
                None | Some(b'[') => return Ok(()),
                Some(_) => {
                    let (k_line, k_col) = (self.line, self.col);
                    let key = self.parse_single_key()?;
                    self.skip_ws();
                    if self.peek_byte() == Some(b'.') {
                        self.validate_dotted_key_remainder()?;
                        return Err(TomlError::new(
                            k_line,
                            k_col,
                            TomlErrorKind::OutsideSubset("dotted key"),
                        ));
                    }
                    if self.peek_byte() != Some(b'=') {
                        return Err(self.syntax("expected `=` after key"));
                    }
                    self.pos += 1;
                    self.col = self.col.saturating_add(1);
                    self.skip_ws();
                    let val = self.parse_value(0)?;
                    self.expect_eol_or_eof()?;
                    table.insert_kv(key, val, k_line, k_col)?;
                }
            }
        }
    }

    fn parse_header(&mut self) -> Result<(bool, Vec<String>), TomlError> {
        debug_assert_eq!(self.peek_byte(), Some(b'['));
        self.pos += 1;
        self.col = self.col.saturating_add(1);
        let is_aot = if self.peek_byte() == Some(b'[') {
            self.pos += 1;
            self.col = self.col.saturating_add(1);
            true
        } else {
            false
        };

        let mut path = Vec::new();
        loop {
            self.skip_ws();
            let seg = self.parse_single_key()?;
            path.push(seg);
            if path.len() > MAX_NESTING_DEPTH {
                return Err(self.err(TomlErrorKind::TooDeep));
            }
            self.skip_ws();
            if self.peek_byte() == Some(b'.') {
                self.pos += 1;
                self.col = self.col.saturating_add(1);
                continue;
            }
            break;
        }

        if is_aot {
            if self.peek_byte() != Some(b']') || self.peek_byte_at(1) != Some(b']') {
                return Err(self.syntax("expected `]]` to close array-of-tables header"));
            }
            self.pos += 2;
            self.col = self.col.saturating_add(2);
        } else {
            if self.peek_byte() != Some(b']') {
                return Err(self.syntax("expected `]` to close table header"));
            }
            self.pos += 1;
            self.col = self.col.saturating_add(1);
        }
        self.expect_eol_or_eof()?;
        Ok((is_aot, path))
    }

    /// After seeing `.` following an initial key, verify the remainder forms a valid
    /// TOML 1.0.0 dotted key chain up to `=` so malformed dotted keys report `Syntax`.
    fn validate_dotted_key_remainder(&mut self) -> Result<(), TomlError> {
        while self.peek_byte() == Some(b'.') {
            self.pos += 1;
            self.col = self.col.saturating_add(1);
            self.skip_ws();
            self.parse_key_any_toml10()?;
            self.skip_ws();
        }
        if self.peek_byte() != Some(b'=') {
            return Err(self.syntax("expected `=` after dotted key"));
        }
        Ok(())
    }

    /// Parse a single key segment (bare or basic-quoted; literal-quoted -> `OutsideSubset`).
    fn parse_single_key(&mut self) -> Result<String, TomlError> {
        match self.peek_byte() {
            Some(b'"') => {
                if self.rest().starts_with("\"\"\"") {
                    return Err(self.syntax("multi-line basic string cannot be used as a key"));
                }
                self.parse_basic_string()
            }
            Some(b'\'') => {
                if self.rest().starts_with("'''") {
                    return Err(self.syntax("multi-line literal string cannot be used as a key"));
                }
                let (l, c) = (self.line, self.col);
                self.scan_literal_string()?;
                Err(TomlError::new(l, c, TomlErrorKind::OutsideSubset("literal string")))
            }
            Some(b) if is_bare_key_byte(b) => Ok(self.parse_bare_key()),
            _ => Err(self.syntax("expected key (bare identifier or double-quoted string)")),
        }
    }

    /// Parse any valid TOML 1.0.0 key segment (used only while checking dotted-key syntax).
    fn parse_key_any_toml10(&mut self) -> Result<(), TomlError> {
        match self.peek_byte() {
            Some(b'"') => {
                if self.rest().starts_with("\"\"\"") {
                    return Err(self.syntax("multi-line basic string cannot be used as a key"));
                }
                self.scan_basic_string_toml10()
            }
            Some(b'\'') => {
                if self.rest().starts_with("'''") {
                    return Err(self.syntax("multi-line literal string cannot be used as a key"));
                }
                self.scan_literal_string()
            }
            Some(b) if is_bare_key_byte(b) => {
                let _ = self.parse_bare_key();
                Ok(())
            }
            _ => Err(self.syntax("expected key segment after `.`")),
        }
    }

    fn parse_bare_key(&mut self) -> String {
        let start = self.pos;
        while let Some(b) = self.peek_byte() {
            if is_bare_key_byte(b) {
                self.pos += 1;
                self.col = self.col.saturating_add(1);
            } else {
                break;
            }
        }
        self.src.get(start..self.pos).unwrap_or("").to_owned()
    }

    fn parse_value(&mut self, depth: usize) -> Result<BuildNode, TomlError> {
        match self.peek_byte() {
            None => Err(self.syntax("unexpected end of input where a value was expected")),
            Some(b'"') => {
                if self.rest().starts_with("\"\"\"") {
                    let (l, c) = (self.line, self.col);
                    self.scan_multiline_basic_string()?;
                    return Err(TomlError::new(
                        l,
                        c,
                        TomlErrorKind::OutsideSubset("multi-line string"),
                    ));
                }
                Ok(BuildNode::Str(self.parse_basic_string()?))
            }
            Some(b'\'') => {
                let (l, c) = (self.line, self.col);
                if self.rest().starts_with("'''") {
                    self.scan_multiline_literal_string()?;
                    return Err(TomlError::new(
                        l,
                        c,
                        TomlErrorKind::OutsideSubset("multi-line string"),
                    ));
                }
                self.scan_literal_string()?;
                Err(TomlError::new(l, c, TomlErrorKind::OutsideSubset("literal string")))
            }
            Some(b'[') => self.parse_array(depth),
            Some(b'{') => self.parse_inline_table(depth),
            Some(_) => self.parse_unquoted_scalar(),
        }
    }

    fn parse_array(&mut self, depth: usize) -> Result<BuildNode, TomlError> {
        let next_depth = depth + 1;
        if next_depth > MAX_NESTING_DEPTH {
            return Err(self.err(TomlErrorKind::TooDeep));
        }
        self.pos += 1;
        self.col = self.col.saturating_add(1);

        let mut items: Vec<Spanned<BuildNode>> = Vec::new();
        let mut expected_tag: Option<ValueTag> = None;

        self.skip_ws_newlines_comments()?;
        if self.peek_byte() == Some(b']') {
            self.pos += 1;
            self.col = self.col.saturating_add(1);
            return Ok(BuildNode::Array { items, from_aot: false });
        }

        loop {
            let (e_line, e_col) = (self.line, self.col);
            let elem = self.parse_value(next_depth)?;
            let tag = elem.tag();
            match expected_tag {
                None => expected_tag = Some(tag),
                Some(first) if first != tag => {
                    return Err(TomlError::new(
                        e_line,
                        e_col,
                        TomlErrorKind::OutsideSubset("mixed-type array"),
                    ));
                }
                Some(_) => {}
            }
            items.push(Spanned::new(elem, e_line, e_col));

            self.skip_ws_newlines_comments()?;
            match self.peek_byte() {
                Some(b']') => {
                    self.pos += 1;
                    self.col = self.col.saturating_add(1);
                    return Ok(BuildNode::Array { items, from_aot: false });
                }
                Some(b',') => {
                    self.pos += 1;
                    self.col = self.col.saturating_add(1);
                    self.skip_ws_newlines_comments()?;
                    if self.peek_byte() == Some(b']') {
                        self.pos += 1;
                        self.col = self.col.saturating_add(1);
                        return Ok(BuildNode::Array { items, from_aot: false });
                    }
                }
                _ => return Err(self.syntax("expected `,` or `]` in array")),
            }
        }
    }

    fn parse_inline_table(&mut self, depth: usize) -> Result<BuildNode, TomlError> {
        let next_depth = depth + 1;
        if next_depth > MAX_NESTING_DEPTH {
            return Err(self.err(TomlErrorKind::TooDeep));
        }
        let (t_line, t_col) = (self.line, self.col);
        self.pos += 1;
        self.col = self.col.saturating_add(1);

        let mut table = BuildTable::new(TableOrigin::Inline, (t_line, t_col));
        self.skip_ws();
        if self.peek_byte() == Some(b'}') {
            self.pos += 1;
            self.col = self.col.saturating_add(1);
            return Ok(BuildNode::Table(table));
        }

        loop {
            match self.peek_byte() {
                Some(b'\n' | b'\r') => {
                    return Err(self.syntax("inline table must be on a single line in TOML 1.0"));
                }
                Some(b'#') => {
                    return Err(self.syntax("comments are not allowed inside inline tables"));
                }
                _ => {}
            }
            let (k_line, k_col) = (self.line, self.col);
            let key = self.parse_single_key()?;
            self.skip_ws();
            if self.peek_byte() == Some(b'.') {
                self.validate_dotted_key_remainder()?;
                return Err(TomlError::new(
                    k_line,
                    k_col,
                    TomlErrorKind::OutsideSubset("dotted key"),
                ));
            }
            if self.peek_byte() != Some(b'=') {
                return Err(self.syntax("expected `=` in inline table"));
            }
            self.pos += 1;
            self.col = self.col.saturating_add(1);
            self.skip_ws();
            let val = self.parse_value(next_depth)?;
            table.insert_kv(key, val, k_line, k_col)?;

            self.skip_ws();
            match self.peek_byte() {
                Some(b'}') => {
                    self.pos += 1;
                    self.col = self.col.saturating_add(1);
                    return Ok(BuildNode::Table(table));
                }
                Some(b',') => {
                    self.pos += 1;
                    self.col = self.col.saturating_add(1);
                    self.skip_ws();
                    if self.peek_byte() == Some(b'}') {
                        return Err(self.syntax(
                            "trailing comma in inline table is forbidden in TOML 1.0",
                        ));
                    }
                }
                Some(b'\n' | b'\r') => {
                    return Err(self.syntax("inline table must be on a single line in TOML 1.0"));
                }
                _ => return Err(self.syntax("expected `,` or `}` in inline table")),
            }
        }
    }

    fn parse_basic_string(&mut self) -> Result<String, TomlError> {
        debug_assert_eq!(self.peek_byte(), Some(b'"'));
        self.pos += 1;
        self.col = self.col.saturating_add(1);
        let mut out = String::new();
        let mut outside_escape: Option<(u32, u32, &'static str)> = None;

        while let Some(ch) = self.peek_char() {
            match ch {
                '"' => {
                    self.next_char();
                    if let Some((l, c, name)) = outside_escape {
                        return Err(TomlError::new(l, c, TomlErrorKind::OutsideSubset(name)));
                    }
                    return Ok(out);
                }
                '\\' => {
                    let (e_line, e_col) = (self.line, self.col);
                    self.next_char();
                    let Some(esc) = self.next_char() else {
                        return Err(self.syntax("unterminated escape in basic string"));
                    };
                    match esc {
                        '"' => out.push('"'),
                        '\\' => out.push('\\'),
                        'n' => out.push('\n'),
                        't' => out.push('\t'),
                        'r' => out.push('\r'),
                        'b' => {
                            if outside_escape.is_none() {
                                outside_escape = Some((e_line, e_col, "escape \\b"));
                            }
                        }
                        'f' => {
                            if outside_escape.is_none() {
                                outside_escape = Some((e_line, e_col, "escape \\f"));
                            }
                        }
                        'u' => out.push(self.parse_hex_scalar(4)?),
                        'U' => out.push(self.parse_hex_scalar(8)?),
                        other => {
                            return Err(TomlError::new(
                                e_line,
                                e_col,
                                TomlErrorKind::Syntax(format!("unknown escape `\\{other}`")),
                            ));
                        }
                    }
                }
                '\n' | '\r' => {
                    return Err(self.syntax("unescaped newline inside basic string"));
                }
                c if is_disallowed_basic_char(c) => {
                    return Err(self.syntax(format!(
                        "disallowed control character U+{:04X} in basic string",
                        c as u32
                    )));
                }
                c => {
                    self.next_char();
                    out.push(c);
                }
            }
        }
        Err(self.syntax("unterminated basic string"))
    }

    fn scan_basic_string_toml10(&mut self) -> Result<(), TomlError> {
        match self.parse_basic_string() {
            Ok(_) => Ok(()),
            Err(TomlError {
                kind: TomlErrorKind::OutsideSubset("escape \\b" | "escape \\f"),
                ..
            }) => Ok(()),
            Err(e) => Err(e),
        }
    }

    fn parse_hex_scalar(&mut self, digits: usize) -> Result<char, TomlError> {
        let (start_line, start_col) = (self.line, self.col);
        let mut val = 0u32;
        for _ in 0..digits {
            let Some(ch) = self.next_char() else {
                return Err(TomlError::new(
                    start_line,
                    start_col,
                    TomlErrorKind::Syntax("truncated unicode escape".to_owned()),
                ));
            };
            let Some(d) = ch.to_digit(16) else {
                return Err(TomlError::new(
                    start_line,
                    start_col,
                    TomlErrorKind::Syntax(format!("non-hex character `{ch}` in unicode escape")),
                ));
            };
            val = (val << 4) | d;
        }
        char::from_u32(val).ok_or_else(|| {
            TomlError::new(
                start_line,
                start_col,
                TomlErrorKind::Syntax(format!("invalid Unicode scalar value U+{val:04X}")),
            )
        })
    }

    fn scan_literal_string(&mut self) -> Result<(), TomlError> {
        debug_assert_eq!(self.peek_byte(), Some(b'\''));
        self.pos += 1;
        self.col = self.col.saturating_add(1);
        while let Some(ch) = self.peek_char() {
            if ch == '\'' {
                self.next_char();
                return Ok(());
            }
            if ch == '\n' || ch == '\r' {
                return Err(self.syntax("newline inside single-line literal string"));
            }
            if is_disallowed_basic_char(ch) {
                return Err(self.syntax(format!(
                    "disallowed control character U+{:04X} in literal string",
                    ch as u32
                )));
            }
            self.next_char();
        }
        Err(self.syntax("unterminated literal string"))
    }

    fn scan_multiline_basic_string(&mut self) -> Result<(), TomlError> {
        self.pos += 3;
        self.col = self.col.saturating_add(3);
        // Optional immediate newline after opening `"""`.
        let _ = self.consume_newline()?;

        while let Some(ch) = self.peek_char() {
            match ch {
                '"' => {
                    let mut count = 0usize;
                    while self.peek_byte() == Some(b'"') {
                        self.pos += 1;
                        self.col = self.col.saturating_add(1);
                        count += 1;
                    }
                    if count < 3 {
                        continue;
                    }
                    if count <= 5 {
                        return Ok(());
                    }
                    return Err(self.syntax("six or more consecutive quotes in multi-line string"));
                }
                '\\' => {
                    self.next_char();
                    match self.peek_byte() {
                        Some(b' ' | b'\t' | b'\n' | b'\r') => {
                            self.skip_ws();
                            if !self.consume_newline()? {
                                return Err(self.syntax(
                                    "whitespace after line-ending backslash without newline",
                                ));
                            }
                            loop {
                                self.skip_ws();
                                if !self.consume_newline()? {
                                    break;
                                }
                            }
                        }
                        Some(_) => {
                            let esc = self.next_char().unwrap_or('\0');
                            match esc {
                                '"' | '\\' | 'b' | 'f' | 'n' | 'r' | 't' => {}
                                'u' => {
                                    let _ = self.parse_hex_scalar(4)?;
                                }
                                'U' => {
                                    let _ = self.parse_hex_scalar(8)?;
                                }
                                other => {
                                    return Err(self.syntax(format!(
                                        "unknown escape `\\{other}` in multi-line string"
                                    )));
                                }
                            }
                        }
                        None => return Err(self.syntax("unterminated escape in multi-line string")),
                    }
                }
                '\n' | '\r' => {
                    self.consume_newline()?;
                }
                c if is_disallowed_multiline_char(c) => {
                    return Err(self.syntax(format!(
                        "disallowed control character U+{:04X} in multi-line string",
                        c as u32
                    )));
                }
                _ => {
                    self.next_char();
                }
            }
        }
        Err(self.syntax("unterminated multi-line basic string"))
    }

    fn scan_multiline_literal_string(&mut self) -> Result<(), TomlError> {
        self.pos += 3;
        self.col = self.col.saturating_add(3);
        let _ = self.consume_newline()?;

        while let Some(ch) = self.peek_char() {
            match ch {
                '\'' => {
                    let mut count = 0usize;
                    while self.peek_byte() == Some(b'\'') {
                        self.pos += 1;
                        self.col = self.col.saturating_add(1);
                        count += 1;
                    }
                    if count < 3 {
                        continue;
                    }
                    if count <= 5 {
                        return Ok(());
                    }
                    return Err(
                        self.syntax("six or more consecutive single quotes in multi-line literal string")
                    );
                }
                '\n' | '\r' => {
                    self.consume_newline()?;
                }
                c if is_disallowed_multiline_char(c) => {
                    return Err(self.syntax(format!(
                        "disallowed control character U+{:04X} in multi-line literal string",
                        c as u32
                    )));
                }
                _ => {
                    self.next_char();
                }
            }
        }
        Err(self.syntax("unterminated multi-line literal string"))
    }

    fn parse_unquoted_scalar(&mut self) -> Result<BuildNode, TomlError> {
        let (start_line, start_col) = (self.line, self.col);
        let start = self.pos;

        while let Some(b) = self.peek_byte() {
            if is_value_delimiter(b) {
                break;
            }
            self.next_char();
        }
        let mut end = self.pos;

        // Space-separated TOML 1.0.0 local/offset date-time (`YYYY-MM-DD HH:MM:SS...`).
        let initial = self.src.get(start..end).unwrap_or("");
        if initial.len() == 10
            && looks_like_date_prefix(initial.as_bytes())
            && self.peek_byte() == Some(b' ')
            && self.peek_byte_at(1).is_some_and(|b| b.is_ascii_digit())
            && self.peek_byte_at(2).is_some_and(|b| b.is_ascii_digit())
            && self.peek_byte_at(3) == Some(b':')
        {
            self.pos += 1;
            self.col = self.col.saturating_add(1);
            while let Some(b) = self.peek_byte() {
                if is_value_delimiter(b) {
                    break;
                }
                self.next_char();
            }
            end = self.pos;
        }

        let token = self.src.get(start..end).unwrap_or("");
        if token.is_empty() {
            return Err(TomlError::new(
                start_line,
                start_col,
                TomlErrorKind::Syntax("expected a value".to_owned()),
            ));
        }

        match token {
            "true" => return Ok(BuildNode::Bool(true)),
            "false" => return Ok(BuildNode::Bool(false)),
            "inf" | "+inf" | "-inf" | "nan" | "+nan" | "-nan" => {
                return Err(TomlError::new(
                    start_line,
                    start_col,
                    TomlErrorKind::OutsideSubset("float"),
                ));
            }
            _ => {}
        }

        if is_valid_toml10_datetime_or_time(token) {
            return Err(TomlError::new(
                start_line,
                start_col,
                TomlErrorKind::OutsideSubset("date"),
            ));
        }

        if let Some(rest) = token.strip_prefix("0x") {
            if is_valid_radix_digits(rest, |b| b.is_ascii_hexdigit()) {
                return Err(TomlError::new(
                    start_line,
                    start_col,
                    TomlErrorKind::OutsideSubset("hexadecimal integer"),
                ));
            }
            return Err(TomlError::new(
                start_line,
                start_col,
                TomlErrorKind::Syntax(format!("invalid hexadecimal integer `{token}`")),
            ));
        }
        if let Some(rest) = token.strip_prefix("0o") {
            if is_valid_radix_digits(rest, |b| matches!(b, b'0'..=b'7')) {
                return Err(TomlError::new(
                    start_line,
                    start_col,
                    TomlErrorKind::OutsideSubset("octal integer"),
                ));
            }
            return Err(TomlError::new(
                start_line,
                start_col,
                TomlErrorKind::Syntax(format!("invalid octal integer `{token}`")),
            ));
        }
        if let Some(rest) = token.strip_prefix("0b") {
            if is_valid_radix_digits(rest, |b| matches!(b, b'0' | b'1')) {
                return Err(TomlError::new(
                    start_line,
                    start_col,
                    TomlErrorKind::OutsideSubset("binary integer"),
                ));
            }
            return Err(TomlError::new(
                start_line,
                start_col,
                TomlErrorKind::Syntax(format!("invalid binary integer `{token}`")),
            ));
        }

        if is_valid_toml10_float(token) {
            return Err(TomlError::new(
                start_line,
                start_col,
                TomlErrorKind::OutsideSubset("float"),
            ));
        }

        parse_decimal_int(token, start_line, start_col).map(BuildNode::Int)
    }
}

fn open_header_table<'a>(
    root: &'a mut BuildTable,
    path: &[String],
    is_aot: bool,
    span: (u32, u32),
) -> Result<&'a mut BuildTable, TomlError> {
    use std::collections::btree_map::Entry;

    let full_name = path.join(".");
    let Some((last, parents)) = path.split_last() else {
        return Err(TomlError::new(
            span.0,
            span.1,
            TomlErrorKind::Syntax("empty table header".to_owned()),
        ));
    };

    let mut cur = root;
    for seg in parents {
        let entry = cur.entries.entry(seg.clone()).or_insert_with(|| {
            Spanned::new(
                BuildNode::Table(BuildTable::new(TableOrigin::Implicit, span)),
                span.0,
                span.1,
            )
        });
        match &mut entry.value {
            BuildNode::Table(sub) => {
                if sub.origin == TableOrigin::Inline {
                    return Err(TomlError::new(
                        span.0,
                        span.1,
                        TomlErrorKind::TableRedefined(full_name),
                    ));
                }
                cur = sub;
            }
            BuildNode::Array { items, from_aot: true } => {
                let Some(last_item) = items.last_mut() else {
                    return Err(TomlError::new(
                        span.0,
                        span.1,
                        TomlErrorKind::TableRedefined(full_name),
                    ));
                };
                match &mut last_item.value {
                    BuildNode::Table(sub) => cur = sub,
                    _ => {
                        return Err(TomlError::new(
                            span.0,
                            span.1,
                            TomlErrorKind::TableRedefined(full_name),
                        ));
                    }
                }
            }
            _ => {
                return Err(TomlError::new(
                    span.0,
                    span.1,
                    TomlErrorKind::TableRedefined(full_name),
                ));
            }
        }
    }

    if is_aot {
        let entry = cur.entries.entry(last.clone()).or_insert_with(|| {
            Spanned::new(
                BuildNode::Array {
                    items: Vec::new(),
                    from_aot: true,
                },
                span.0,
                span.1,
            )
        });
        match &mut entry.value {
            BuildNode::Array { items, from_aot: true } => {
                items.push(Spanned::new(
                    BuildNode::Table(BuildTable::new(TableOrigin::Explicit, span)),
                    span.0,
                    span.1,
                ));
                if let Some(Spanned {
                    value: BuildNode::Table(t),
                    ..
                }) = items.last_mut()
                {
                    Ok(t)
                } else {
                    Err(TomlError::new(
                        span.0,
                        span.1,
                        TomlErrorKind::TableRedefined(full_name),
                    ))
                }
            }
            _ => Err(TomlError::new(
                span.0,
                span.1,
                TomlErrorKind::TableRedefined(full_name),
            )),
        }
    } else {
        match cur.entries.entry(last.clone()) {
            Entry::Vacant(slot) => {
                let inserted = slot.insert(Spanned::new(
                    BuildNode::Table(BuildTable::new(TableOrigin::Explicit, span)),
                    span.0,
                    span.1,
                ));
                if let BuildNode::Table(t) = &mut inserted.value {
                    Ok(t)
                } else {
                    Err(TomlError::new(
                        span.0,
                        span.1,
                        TomlErrorKind::TableRedefined(full_name),
                    ))
                }
            }
            Entry::Occupied(slot) => {
                let entry = slot.into_mut();
                match &mut entry.value {
                    BuildNode::Table(sub) if sub.origin == TableOrigin::Implicit => {
                        sub.origin = TableOrigin::Explicit;
                        sub.span = span;
                        entry.line = span.0;
                        entry.column = span.1;
                        Ok(sub)
                    }
                    _ => Err(TomlError::new(
                        span.0,
                        span.1,
                        TomlErrorKind::TableRedefined(full_name),
                    )),
                }
            }
        }
    }
}

// ============================================================================
// Lexical / TOML 1.0.0 scalar grammar helpers
// ============================================================================

const fn is_bare_key_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_' || b == b'-'
}

const fn is_value_delimiter(b: u8) -> bool {
    matches!(b, b' ' | b'\t' | b'\n' | b'\r' | b'#' | b',' | b']' | b'}')
}

const fn is_disallowed_comment_char(c: char) -> bool {
    let cp = c as u32;
    (cp <= 0x08) || (cp >= 0x0A && cp <= 0x1F) || cp == 0x7F
}

const fn is_disallowed_basic_char(c: char) -> bool {
    let cp = c as u32;
    (cp <= 0x08) || (cp >= 0x0A && cp <= 0x1F) || cp == 0x7F
}

const fn is_disallowed_multiline_char(c: char) -> bool {
    let cp = c as u32;
    (cp <= 0x08) || cp == 0x0B || cp == 0x0C || (cp >= 0x0E && cp <= 0x1F) || cp == 0x7F
}

fn is_valid_radix_digits(s: &str, pred: impl Fn(u8) -> bool) -> bool {
    let bytes = s.as_bytes();
    if bytes.is_empty() || !pred(bytes[0]) || !pred(bytes[bytes.len() - 1]) {
        return false;
    }
    let mut prev_underscore = false;
    for &b in bytes {
        if b == b'_' {
            if prev_underscore {
                return false;
            }
            prev_underscore = true;
        } else if pred(b) {
            prev_underscore = false;
        } else {
            return false;
        }
    }
    true
}

fn is_valid_dec_int_part(s: &str) -> bool {
    let bytes = s.as_bytes();
    if bytes.is_empty() {
        return false;
    }
    if bytes[0] == b'0' {
        return bytes.len() == 1;
    }
    is_valid_radix_digits(s, |b| b.is_ascii_digit())
}

fn is_valid_toml10_float(token: &str) -> bool {
    let body = token
        .strip_prefix('+')
        .or_else(|| token.strip_prefix('-'))
        .unwrap_or(token);
    let (mantissa, exp_part) = match body.find(['e', 'E']) {
        Some(idx) => (&body[..idx], Some(&body[idx + 1..])),
        None => (body, None),
    };
    let (int_part, frac_part) = match mantissa.find('.') {
        Some(idx) => (&mantissa[..idx], Some(&mantissa[idx + 1..])),
        None => (mantissa, None),
    };
    if frac_part.is_none() && exp_part.is_none() {
        return false;
    }
    if !is_valid_dec_int_part(int_part) {
        return false;
    }
    if let Some(frac) = frac_part
        && !is_valid_radix_digits(frac, |b| b.is_ascii_digit())
    {
        return false;
    }
    if let Some(exp) = exp_part {
        let exp_digits = exp
            .strip_prefix('+')
            .or_else(|| exp.strip_prefix('-'))
            .unwrap_or(exp);
        if !is_valid_radix_digits(exp_digits, |b| b.is_ascii_digit()) {
            return false;
        }
    }
    true
}

fn parse_decimal_int(token: &str, line: u32, col: u32) -> Result<i64, TomlError> {
    let body = token
        .strip_prefix('+')
        .or_else(|| token.strip_prefix('-'))
        .unwrap_or(token);
    if !is_valid_dec_int_part(body) {
        return Err(TomlError::new(
            line,
            col,
            TomlErrorKind::Syntax(format!("invalid value `{token}`")),
        ));
    }
    if body.contains('_') {
        return Err(TomlError::new(
            line,
            col,
            TomlErrorKind::OutsideSubset("underscore in integer"),
        ));
    }
    if body == "0" {
        return Ok(0);
    }
    match token.parse::<i64>() {
        Ok(v) => Ok(v),
        Err(err) => match err.kind() {
            IntErrorKind::PosOverflow | IntErrorKind::NegOverflow => {
                Err(TomlError::new(line, col, TomlErrorKind::IntegerOutOfRange))
            }
            _ => Err(TomlError::new(
                line,
                col,
                TomlErrorKind::Syntax(format!("invalid integer `{token}`")),
            )),
        },
    }
}

fn looks_like_date_prefix(b: &[u8]) -> bool {
    b.len() >= 10
        && b[0].is_ascii_digit()
        && b[1].is_ascii_digit()
        && b[2].is_ascii_digit()
        && b[3].is_ascii_digit()
        && b[4] == b'-'
        && b[5].is_ascii_digit()
        && b[6].is_ascii_digit()
        && b[7] == b'-'
        && b[8].is_ascii_digit()
        && b[9].is_ascii_digit()
}

fn parse_2digits(b: &[u8]) -> Option<u32> {
    if b.len() == 2 && b[0].is_ascii_digit() && b[1].is_ascii_digit() {
        Some(u32::from(b[0] - b'0') * 10 + u32::from(b[1] - b'0'))
    } else {
        None
    }
}

fn parse_4digits(b: &[u8]) -> Option<u32> {
    if b.len() == 4 && b.iter().all(u8::is_ascii_digit) {
        Some(
            u32::from(b[0] - b'0') * 1000
                + u32::from(b[1] - b'0') * 100
                + u32::from(b[2] - b'0') * 10
                + u32::from(b[3] - b'0'),
        )
    } else {
        None
    }
}

fn is_valid_date_part(b: &[u8]) -> bool {
    if b.len() != 10 || b[4] != b'-' || b[7] != b'-' {
        return false;
    }
    let (Some(year), Some(month), Some(day)) = (
        parse_4digits(&b[0..4]),
        parse_2digits(&b[5..7]),
        parse_2digits(&b[8..10]),
    ) else {
        return false;
    };
    if !(1..=12).contains(&month) || day == 0 {
        return false;
    }
    let is_leap = (year % 4 == 0 && year % 100 != 0) || year % 400 == 0;
    let max_day = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap {
                29
            } else {
                28
            }
        }
        _ => return false,
    };
    day <= max_day
}

fn parse_time_part(b: &[u8]) -> Option<usize> {
    if b.len() < 8 || b[2] != b':' || b[5] != b':' {
        return None;
    }
    let hh = parse_2digits(&b[0..2])?;
    let mm = parse_2digits(&b[3..5])?;
    let ss = parse_2digits(&b[6..8])?;
    if hh > 23 || mm > 59 || ss > 59 {
        return None;
    }
    let mut consumed = 8;
    if b.get(consumed) == Some(&b'.') {
        consumed += 1;
        let frac_start = consumed;
        while b.get(consumed).is_some_and(u8::is_ascii_digit) {
            consumed += 1;
        }
        if consumed == frac_start {
            return None;
        }
    }
    Some(consumed)
}

fn is_valid_tz_offset(b: &[u8]) -> bool {
    if b == b"Z" || b == b"z" {
        return true;
    }
    if b.len() == 6 && matches!(b[0], b'+' | b'-') && b[3] == b':' {
        if let (Some(hh), Some(mm)) = (parse_2digits(&b[1..3]), parse_2digits(&b[4..6])) {
            return hh <= 23 && mm <= 59;
        }
    }
    false
}

fn is_valid_toml10_datetime_or_time(token: &str) -> bool {
    let b = token.as_bytes();
    if looks_like_date_prefix(b) {
        if !is_valid_date_part(&b[..10]) {
            return false;
        }
        if b.len() == 10 {
            return true;
        }
        if !matches!(b[10], b'T' | b't' | b' ') {
            return false;
        }
        let rest = &b[11..];
        let Some(tlen) = parse_time_part(rest) else {
            return false;
        };
        let tz = &rest[tlen..];
        return tz.is_empty() || is_valid_tz_offset(tz);
    }
    if let Some(tlen) = parse_time_part(b) {
        return tlen == b.len();
    }
    false
}

// ============================================================================
// Unit, property, and release speed tests (N34p2-01 Test Plan)
// ============================================================================

#[cfg(test)]
mod tests {
    use super::{
        Fields, MAX_INPUT_BYTES, MAX_NESTING_DEPTH, Spanned, Table, TomlError, TomlErrorKind,
        Value, parse,
    };
    use std::collections::BTreeMap;
    use std::time::Instant;

    #[test]
    fn toml_duplicate_key_is_error() {
        let err = parse(b"a = 1\na = 2\n").expect_err("duplicate key must fail");
        assert_eq!(err.line, 2);
        assert_eq!(err.column, 1);
        assert_eq!(err.kind, TomlErrorKind::DuplicateKey("a".to_owned()));
    }

    #[test]
    fn toml_unknown_key_is_error() {
        let table = parse(b"psture = \"consume\"\n").expect("valid subset");
        let mut f = Fields::new(table, "foundation");
        let _ = f.opt_str("posture").expect("optional missing is Ok(None)");
        let err = f.finish().expect_err("leftover key must fail");
        assert_eq!(err.line, 1);
        assert_eq!(err.column, 1);
        assert_eq!(
            err.kind,
            TomlErrorKind::UnknownKey {
                table: "foundation".to_owned(),
                key: "psture".to_owned(),
            }
        );
    }

    #[test]
    fn toml_outside_subset_is_error_with_position() {
        let dotted = "\n\n\n\n\n\nserde.workspace = true\n";
        let err = parse(dotted.as_bytes()).expect_err("dotted key must fail");
        assert_eq!((err.line, err.column), (7, 1));
        assert_eq!(err.kind, TomlErrorKind::OutsideSubset("dotted key"));
        assert!(err.to_string().contains("serde = { workspace = true }"));

        let cases: &[(&str, &str)] = &[
            ("x = 'y'\n", "literal string"),
            ("x = 1.5\n", "float"),
            ("x = inf\n", "float"),
            ("x = nan\n", "float"),
            ("x = 1979-05-27\n", "date"),
            ("x = 1979-05-27T07:32:00Z\n", "date"),
            ("x = 07:32:00\n", "date"),
            ("x = 0x1F\n", "hexadecimal integer"),
            ("x = 0o755\n", "octal integer"),
            ("x = 0b1010\n", "binary integer"),
            ("x = \"\"\"a\"\"\"\n", "multi-line string"),
            ("x = '''a'''\n", "multi-line string"),
            ("x = 1_000\n", "underscore in integer"),
            ("x = \"\\b\"\n", "escape \\b"),
            ("x = \"\\f\"\n", "escape \\f"),
        ];
        for &(input, expected_construct) in cases {
            let e = parse(input.as_bytes()).expect_err(input);
            assert_eq!(
                e.kind,
                TomlErrorKind::OutsideSubset(expected_construct),
                "input `{input}` gave {e:?}"
            );
            assert_eq!(e.line, 1);
            assert!(e.column >= 1);
        }
    }

    #[test]
    fn toml_reads_a_manifest() {
        let manifest = r#"
# Synthetic crate manifest inside the strict subset
[package]
name = "frob-demo"
version = { workspace = true }
edition = { workspace = true }
publish = false

[package.metadata.frob]
layer = 2
rank = 3

[lints]
workspace = true

[dependencies]
serde = { workspace = true }
frob-core = { path = "../frob-core" }
"#;
        let doc = parse(manifest.as_bytes()).expect("parse manifest");
        let mut root = Fields::new(doc, "root");
        let pkg = root.require_table("package").expect("package");
        let lints = root.require_table("lints").expect("lints");
        let deps = root.require_table("dependencies").expect("dependencies");
        root.finish().expect("no leftover root keys");

        let mut pkg_f = Fields::new(pkg, "package");
        assert_eq!(pkg_f.require_str("name").unwrap(), "frob-demo");
        assert!(!pkg_f.require_bool("publish").unwrap());
        let _ = pkg_f.require_table("version").unwrap();
        let _ = pkg_f.require_table("edition").unwrap();
        let meta = pkg_f.require_table("metadata").unwrap();
        pkg_f.finish().unwrap();

        let mut meta_f = Fields::new(meta, "package.metadata");
        let frob = meta_f.require_table("frob").unwrap();
        meta_f.finish().unwrap();
        let mut frob_f = Fields::new(frob, "package.metadata.frob");
        assert_eq!(frob_f.require_int("layer").unwrap(), 2);
        assert_eq!(frob_f.require_int("rank").unwrap(), 3);
        frob_f.finish().unwrap();

        let mut lints_f = Fields::new(lints, "lints");
        assert!(lints_f.require_bool("workspace").unwrap());
        lints_f.finish().unwrap();

        let mut deps_f = Fields::new(deps, "dependencies");
        let serde_tbl = deps_f.require_table("serde").unwrap();
        let _ = deps_f.require_table("frob-core").unwrap();
        deps_f.finish().unwrap();
        let mut serde_f = Fields::new(serde_tbl, "dependencies.serde");
        assert!(serde_f.require_bool("workspace").unwrap());
        serde_f.finish().unwrap();
    }

    #[test]
    fn toml_reads_stack_lock_template() {
        let lock = r#"
toolchain = "nightly-2026-08-31"

[[foundation]]
id = "FD-FSIM"
name = "frankensim"
posture = "consume-path"
path = "../frankensim"
git_rev = "d02ad9c170d5932667d15359cd362d3198ac4806"
features = ["referee", "geometry", "ledger"]
build_env = { FRANKENSIM_DEPGRAPH_SALT = "franken-robotics-dev-v1" }

[[foundation]]
id = "FD-ASUP"
name = "asupersync"
posture = "consume"
path = "../asupersync"
git_rev = "70a37b2cfb78ce39dc001739101e0a1a7fbee416"
version = "=0.7.0"
"#;
        let doc = parse(lock.as_bytes()).expect("parse lock");
        let mut root = Fields::new(doc, "stack-lock");
        assert_eq!(root.require_str("toolchain").unwrap(), "nightly-2026-08-31");
        let fds = root.require_array("foundation").unwrap();
        root.finish().unwrap();
        assert_eq!(fds.len(), 2);

        let f0 = fds[0].value.as_table().expect("table").clone();
        let mut f0_fields = Fields::new(f0, "foundation[0]");
        assert_eq!(f0_fields.require_str("id").unwrap(), "FD-FSIM");
        assert_eq!(f0_fields.require_str("name").unwrap(), "frankensim");
        assert_eq!(f0_fields.require_str("posture").unwrap(), "consume-path");
        assert_eq!(f0_fields.require_str("path").unwrap(), "../frankensim");
        assert_eq!(f0_fields.require_str("git_rev").unwrap().len(), 40);
        assert_eq!(f0_fields.require_array("features").unwrap().len(), 3);
        let env = f0_fields.require_table("build_env").unwrap();
        f0_fields.finish().unwrap();
        let mut env_f = Fields::new(env, "foundation[0].build_env");
        assert_eq!(
            env_f.require_str("FRANKENSIM_DEPGRAPH_SALT").unwrap(),
            "franken-robotics-dev-v1"
        );
        env_f.finish().unwrap();

        let f1 = fds[1].value.as_table().expect("table");
        assert_eq!(f1.get("id").and_then(Value::as_str), Some("FD-ASUP"));

        // Also parse a synthetic [[gate]] registry row and rust-toolchain.toml.
        let gate_reg = "schema = \"frob.gates/1\"\n[[gate]]\nid = \"GATE-WS-01\"\ntier = \"G0\"\n";
        let gdoc = parse(gate_reg.as_bytes()).expect("parse gate registry");
        assert_eq!(gdoc.get("gate").and_then(Value::as_array).map(<[_]>::len), Some(1));

        let tc = "[toolchain]\nchannel = \"nightly-2026-08-31\"\ncomponents = [\"rustfmt\", \"clippy\"]\n";
        let tdoc = parse(tc.as_bytes()).expect("parse rust-toolchain.toml");
        assert_eq!(
            tdoc.get("toolchain")
                .and_then(Value::as_table)
                .and_then(|t| t.get("channel"))
                .and_then(Value::as_str),
            Some("nightly-2026-08-31")
        );
    }

    #[test]
    fn toml_reads_a_cargo_lock() {
        let cargo_lock = r#"# This file is automatically @generated by Cargo.
# It is not intended for manual editing.
version = 4

[[package]]
name = "frob-core"
version = "0.0.1"

[[package]]
name = "frob-cert"
version = "0.0.1"

[[package]]
name = "xtask"
version = "0.0.1"
dependencies = [
 "frob-core",
 "frob-cert",
]
"#;
        let doc = parse(cargo_lock.as_bytes()).expect("parse Cargo.lock");
        assert_eq!(doc.get("version").and_then(Value::as_int), Some(4));
        let pkgs = doc.get("package").and_then(Value::as_array).expect("packages");
        let names: Vec<&str> = pkgs
            .iter()
            .map(|p| p.value.as_table().unwrap().get("name").unwrap().as_str().unwrap())
            .collect();
        assert_eq!(names, vec!["frob-core", "frob-cert", "xtask"]);
    }

    #[test]
    fn toml_table_redefinition_is_error() {
        let e1 = parse(b"[a]\nx = 1\n[a]\ny = 2\n").expect_err("redefined table");
        assert_eq!(e1.line, 3);
        assert_eq!(e1.kind, TomlErrorKind::TableRedefined("a".to_owned()));

        let e2 = parse(b"a = 1\n[a]\nx = 2\n").expect_err("value then table");
        assert_eq!(e2.line, 2);
        assert_eq!(e2.kind, TomlErrorKind::TableRedefined("a".to_owned()));
    }

    #[test]
    fn toml_mixed_array_refused() {
        let err = parse(b"x = [1, \"a\"]\n").expect_err("mixed array");
        assert_eq!(err.line, 1);
        assert_eq!(err.column, 9);
        assert_eq!(err.kind, TomlErrorKind::OutsideSubset("mixed-type array"));
    }

    #[test]
    fn toml_inline_table_rules() {
        let e1 = parse(b"t = { a = 1, }\n").expect_err("trailing comma");
        assert!(matches!(e1.kind, TomlErrorKind::Syntax(_)), "got {e1:?}");

        let e2 = parse(b"t = {\n  a = 1\n}\n").expect_err("multiline inline table");
        assert!(matches!(e2.kind, TomlErrorKind::Syntax(_)), "got {e2:?}");
    }

    #[test]
    fn toml_escapes() {
        let doc = parse(b"a = \"\\u00e9\"\nb = \"\\U000000e9\"\nc = \"\\n\\t\\r\\\"\\\\\"\n")
            .expect("valid escapes");
        assert_eq!(doc.get("a").and_then(Value::as_str), Some("é"));
        assert_eq!(doc.get("b").and_then(Value::as_str), Some("é"));
        assert_eq!(doc.get("c").and_then(Value::as_str), Some("\n\t\r\"\\"));

        let eb = parse(b"a = \"\\b\"\n").expect_err("\\b");
        assert_eq!(eb.kind, TomlErrorKind::OutsideSubset("escape \\b"));

        let ef = parse(b"a = \"\\f\"\n").expect_err("\\f");
        assert_eq!(ef.kind, TomlErrorKind::OutsideSubset("escape \\f"));

        let ex = parse(b"a = \"\\x41\"\n").expect_err("\\x41");
        assert!(matches!(ex.kind, TomlErrorKind::Syntax(_)), "got {ex:?}");
    }

    #[test]
    fn toml_integer_range() {
        let doc = parse(b"max = 9223372036854775807\nmin = -9223372036854775808\nz1 = 0\nz2 = +0\nz3 = -0\n")
            .expect("i64 bounds");
        assert_eq!(doc.get("max").and_then(Value::as_int), Some(i64::MAX));
        assert_eq!(doc.get("min").and_then(Value::as_int), Some(i64::MIN));
        assert_eq!(doc.get("z1").and_then(Value::as_int), Some(0));
        assert_eq!(doc.get("z2").and_then(Value::as_int), Some(0));
        assert_eq!(doc.get("z3").and_then(Value::as_int), Some(0));

        let ov = parse(b"x = 9223372036854775808\n").expect_err("overflow");
        assert_eq!(ov.kind, TomlErrorKind::IntegerOutOfRange);

        let lz = parse(b"x = 007\n").expect_err("leading zero");
        assert!(matches!(lz.kind, TomlErrorKind::Syntax(_)), "got {lz:?}");

        let us = parse(b"x = 1_000\n").expect_err("underscore");
        assert_eq!(us.kind, TomlErrorKind::OutsideSubset("underscore in integer"));
    }

    #[test]
    fn toml_error_kind_rule() {
        let outside_subset_fixtures = [
            "a.b = 1\n",
            "x = 'lit'\n",
            "x = \"\"\"ml\"\"\"\n",
            "x = '''ml'''\n",
            "x = 3.14\n",
            "x = -inf\n",
            "x = +nan\n",
            "x = 2026-11-24\n",
            "x = 2026-11-24T12:00:00Z\n",
            "x = 0xdead\n",
            "x = 0o755\n",
            "x = 0b1010\n",
            "x = 1_000_000\n",
            "x = \"\\b\"\n",
            "x = \"\\f\"\n",
            "x = [1, \"two\"]\n",
        ];
        for src in outside_subset_fixtures {
            let err = parse(src.as_bytes()).expect_err(src);
            assert!(
                matches!(err.kind, TomlErrorKind::OutsideSubset(_)),
                "valid TOML outside subset `{src}` must yield OutsideSubset, got {err:?}"
            );
        }

        let toml_invalid_fixtures = [
            "x = 007\n",
            "x = \"\\x41\"\n",
            "t = { a = 1, }\n",
            "t = {\n a = 1 }\n",
            "a = 1\na = 2\n",
            "[a]\n[a]\n",
            "x = 1.\n",
            "x = .5\n",
            "x = 0x_1\n",
            "x = 1__0\n",
            "x = 2026-02-30\n",
            "x = \"unterminated\n",
        ];
        for src in toml_invalid_fixtures {
            let err = parse(src.as_bytes()).expect_err(src);
            assert!(
                !matches!(err.kind, TomlErrorKind::OutsideSubset(_)),
                "invalid TOML `{src}` must NOT yield OutsideSubset, got {err:?}"
            );
        }
    }

    #[test]
    fn toml_bom_refused() {
        let mut bytes = vec![0xEF, 0xBB, 0xBF];
        bytes.extend_from_slice(b"a = 1\n");
        let err = parse(&bytes).expect_err("BOM must fail");
        assert_eq!((err.line, err.column), (1, 1));
        assert_eq!(err.kind, TomlErrorKind::ByteOrderMark);
    }

    #[test]
    fn toml_crlf_equals_lf() {
        let lf = "# header\n[package]\nname = \"frob-core\"\narr = [\n  1,\n  2,\n]\n";
        let crlf = lf.replace('\n', "\r\n");
        let t_lf = parse(lf.as_bytes()).expect("LF parse");
        let t_crlf = parse(crlf.as_bytes()).expect("CRLF parse");
        assert_eq!(t_lf, t_crlf);
    }

    #[test]
    fn toml_depth_cap() {
        let ok_src = format!(
            "a = {}1{}\n",
            "[".repeat(MAX_NESTING_DEPTH),
            "]".repeat(MAX_NESTING_DEPTH)
        );
        assert!(parse(ok_src.as_bytes()).is_ok(), "depth 64 must parse");

        let deep_src = format!(
            "a = {}1{}\n",
            "[".repeat(MAX_NESTING_DEPTH + 1),
            "]".repeat(MAX_NESTING_DEPTH + 1)
        );
        let err = parse(deep_src.as_bytes()).expect_err("depth 65 must fail");
        assert_eq!(err.kind, TomlErrorKind::TooDeep);

        let deep_hdr = format!(
            "[{}]\nx = 1\n",
            std::iter::repeat_n("a", MAX_NESTING_DEPTH + 1)
                .collect::<Vec<_>>()
                .join(".")
        );
        let hdr_err = parse(deep_hdr.as_bytes()).expect_err("65-segment header must fail");
        assert_eq!(hdr_err.kind, TomlErrorKind::TooDeep);
    }

    #[test]
    fn toml_size_cap() {
        let mut big = vec![b' '; MAX_INPUT_BYTES + 1];
        big[0] = b'#';
        let err = parse(&big).expect_err("16 MiB + 1 byte must fail");
        assert_eq!(err.kind, TomlErrorKind::TooLarge);
    }

    #[test]
    #[cfg_attr(not(debug_assertions), ignore)]
    #[should_panic(expected = "dropped without calling finish")]
    fn fields_finish_required() {
        let table = parse(b"a = 1\n").expect("parse");
        let mut f = Fields::new(table, "demo");
        let _ = f.require_int("a").expect("int");
        // Deliberately drop `f` without calling `f.finish()`.
    }

    // Deterministic xorshift64* PRNG for property tests (no external crates).
    struct Rng(u64);
    impl Rng {
        fn next_u64(&mut self) -> u64 {
            let mut x = self.0;
            x ^= x >> 12;
            x ^= x << 25;
            x ^= x >> 27;
            self.0 = x;
            x.wrapping_mul(0x2545_F491_4F6C_DD1D)
        }
        fn gen_range(&mut self, bound: usize) -> usize {
            (self.next_u64() as usize) % bound
        }
    }

    fn gen_key(rng: &mut Rng, idx: usize) -> String {
        if rng.gen_range(4) == 0 {
            format!("k.{idx} \"q\"")
        } else {
            format!("k_{idx}")
        }
    }

    fn gen_str(rng: &mut Rng) -> String {
        let palette = ['a', 'z', '0', ' ', '"', '\\', '\n', '\t', '\r', 'é', 'Ω'];
        let len = rng.gen_range(10);
        let mut s = String::with_capacity(len);
        for _ in 0..len {
            s.push(palette[rng.gen_range(palette.len())]);
        }
        s
    }

    fn gen_value(rng: &mut Rng, depth: usize) -> Value {
        let choice = if depth >= 3 { rng.gen_range(3) } else { rng.gen_range(5) };
        match choice {
            0 => Value::Str(gen_str(rng)),
            1 => {
                let raw = rng.next_u64() as i64;
                Value::Int(raw / 2)
            }
            2 => Value::Bool(rng.gen_range(2) == 0),
            3 => {
                let elem_kind = rng.gen_range(3);
                let len = rng.gen_range(4);
                let mut items = Vec::with_capacity(len);
                for _ in 0..len {
                    let v = match elem_kind {
                        0 => Value::Str(gen_str(rng)),
                        1 => Value::Int((rng.next_u64() as i64) / 4),
                        _ => Value::Bool(rng.gen_range(2) == 0),
                    };
                    items.push(Spanned::new(v, 1, 1));
                }
                Value::Array(items)
            }
            _ => Value::Table(gen_table(rng, depth + 1, 3)),
        }
    }

    fn gen_table(rng: &mut Rng, depth: usize, max_keys: usize) -> Table {
        let count = rng.gen_range(max_keys + 1);
        let mut map = BTreeMap::new();
        for i in 0..count {
            let k = gen_key(rng, i);
            map.insert(k, Spanned::new(gen_value(rng, depth), 1, 1));
        }
        Table::from_entries(map)
    }

    fn format_basic_str(s: &str, out: &mut String) {
        out.push('"');
        for ch in s.chars() {
            match ch {
                '"' => out.push_str("\\\""),
                '\\' => out.push_str("\\\\"),
                '\n' => out.push_str("\\n"),
                '\t' => out.push_str("\\t"),
                '\r' => out.push_str("\\r"),
                c => out.push(c),
            }
        }
        out.push('"');
    }

    fn format_key(k: &str, out: &mut String) {
        if !k.is_empty() && k.bytes().all(super::is_bare_key_byte) {
            out.push_str(k);
        } else {
            format_basic_str(k, out);
        }
    }

    fn format_value(v: &Value, out: &mut String) {
        match v {
            Value::Str(s) => format_basic_str(s, out),
            Value::Int(n) => out.push_str(&n.to_string()),
            Value::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
            Value::Array(items) => {
                out.push('[');
                for (i, sp) in items.iter().enumerate() {
                    if i > 0 {
                        out.push_str(", ");
                    }
                    format_value(&sp.value, out);
                }
                out.push(']');
            }
            Value::Table(t) => {
                out.push('{');
                for (i, (k, sp)) in t.iter().enumerate() {
                    if i > 0 {
                        out.push_str(", ");
                    }
                    format_key(k, out);
                    out.push_str(" = ");
                    format_value(&sp.value, out);
                }
                out.push('}');
            }
        }
    }

    fn print_root_table(t: &Table) -> String {
        let mut out = String::new();
        for (k, sp) in t.iter() {
            format_key(k, &mut out);
            out.push_str(" = ");
            format_value(&sp.value, &mut out);
            out.push('\n');
        }
        out
    }

    #[test]
    fn toml_print_parse_roundtrip() {
        let mut rng = Rng(0xF00D_CAFE_1234_5678);
        for case in 0..1000 {
            let tree = gen_table(&mut rng, 0, 5);
            let rendered = print_root_table(&tree);
            let reparsed = parse(rendered.as_bytes())
                .unwrap_or_else(|e| panic!("roundtrip case {case} failed: {e}\n{rendered}"));
            assert!(
                tree.eq_values(&reparsed),
                "roundtrip mismatch on case {case}:\n{rendered}"
            );
        }
    }

    #[test]
    fn toml_no_panic() {
        let seeds: &[&[u8]] = &[
            b"",
            b"a = 1\n",
            b"[a.b]\nc = \"hello\\u0041\"\n",
            b"[[items]]\nx = [1, 2, 3]\nt = { k = true }\n",
            b"x = [1, \"a\"]\n",
            b"serde.workspace = true\n",
            b"x = 1979-05-27T07:32:00Z\n",
            b"x = 1979-05-27 07:32:00.5-07:00\n",
            b"x = 07:32:00\n",
            b"x = \"\"\"multi\"\"\"\n",
            b"x = '''lit'''\n",
            b"x = 'single'\n",
            b"x = 0x1f\n",
            b"x = 0o755\n",
            b"x = 0b1010\n",
            b"x = 1_000\n",
            b"x = 3.14e-2\n",
            b"x = -inf\n",
            b"x = \"\\b\\f\\U000000e9\"\n",
            b"a = 1\na = 2\n",
            b"[a]\n[a]\n",
            b"t = { a = 1, }\n",
            b"\xef\xbb\xbfa = 1\n",
        ];
        let mut rng = Rng(0xA11C_E5ED_9876_5432);
        let mut count = 0usize;
        let mut buf = Vec::with_capacity(64);
        let alphabet: &[u8] = b"ab01_-\"'[]{}.,=# \t\r\n\\uUbfntx0:T+Z\x00\x7f\xef\xbb\xbf";

        // 50,000 byte-level mutations of fixtures + 50,000 random byte inputs = 100,000 total.
        for _ in 0..50_000 {
            let base = seeds[rng.gen_range(seeds.len())];
            buf.clear();
            buf.extend_from_slice(base);
            let mutations = 1 + rng.gen_range(3);
            for _ in 0..mutations {
                match rng.gen_range(3) {
                    0 if !buf.is_empty() => {
                        let idx = rng.gen_range(buf.len());
                        buf[idx] = alphabet[rng.gen_range(alphabet.len())];
                    }
                    1 => {
                        let idx = rng.gen_range(buf.len() + 1);
                        buf.insert(idx, alphabet[rng.gen_range(alphabet.len())]);
                    }
                    _ if !buf.is_empty() => {
                        let idx = rng.gen_range(buf.len());
                        buf.remove(idx);
                    }
                    _ => {}
                }
            }
            let _res: Result<Table, TomlError> = parse(&buf);
            count += 1;
        }
        for _ in 0..50_000 {
            let len = rng.gen_range(48);
            buf.clear();
            for _ in 0..len {
                if rng.gen_range(5) == 0 {
                    buf.push((rng.next_u64() & 0xFF) as u8);
                } else {
                    buf.push(alphabet[rng.gen_range(alphabet.len())]);
                }
            }
            let _res: Result<Table, TomlError> = parse(&buf);
            count += 1;
        }
        assert_eq!(count, 100_000);
    }

    #[test]
    #[ignore = "release-mode speed target (<= 50 ms on 1 MiB); invoked by scripts/e2e/toml_reader.sh"]
    fn toml_parse_speed() {
        let mut doc = String::with_capacity(1_200_000);
        doc.push_str("schema = \"frob.gates/1\"\n");
        for i in 0..8192u32 {
            doc.push_str(&format!(
                "[[gate]]\nid = \"GATE-SYN-{i:04}\"\ntier = \"G0\"\nowner = \"frob-core\"\nrequired = true\nthreshold_ms = 50\ntags = [\"m0\", \"solid\", \"workspace\"]\nmeta = {{ row = {i}, active = true }}\n"
            ));
        }
        assert!(doc.len() >= 1024 * 1024, "generated doc must be >= 1 MiB");

        let bytes = doc.as_bytes();
        // Warmup once, then time the parse.
        let warm = parse(bytes).expect("warmup parse");
        assert_eq!(
            warm.get("gate").and_then(Value::as_array).map(<[_]>::len),
            Some(8192)
        );

        let start = Instant::now();
        let parsed = parse(bytes).expect("timed parse");
        let elapsed_ms = start.elapsed().as_secs_f64() * 1000.0;
        assert_eq!(
            parsed.get("gate").and_then(Value::as_array).map(<[_]>::len),
            Some(8192)
        );
        println!("TOML_PARSE_BYTES={}", bytes.len());
        println!("TOML_PARSE_SPEED_MS={elapsed_ms:.3}");
        assert!(
            elapsed_ms <= 50.0,
            "1 MiB synthetic registry parsed in {elapsed_ms:.3} ms (target <= 50 ms)"
        );
    }
}
