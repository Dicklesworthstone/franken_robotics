//! Conformance tests for `frob_core::registry::toml` against the vendored
//! `toml-lang/toml-test` 1.0.0 corpus (MIT license; bead N34p2-01).
//!
//! For every case listed in `files-toml-1.0.0`:
//! - `valid/*.toml`: MUST either parse to the exact expected value in the
//!   sibling `.json` file OR fail with `TomlErrorKind::OutsideSubset(_)`
//!   (never `Syntax`, `DuplicateKey`, `TableRedefined`, or a wrong value).
//! - `invalid/*.toml`: MUST return `Err(_)`.
//!
//! When `FROB_TOML_CORPUS_REPORT` is set in the environment, writes one TSV
//! line per case (`case\texpected\tobserved\tstatus\terror_kind\tpos\tpath`)
//! plus summary metadata for `scripts/e2e/toml_reader.sh`.

use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use frob_core::registry::toml::{Table, TomlErrorKind, Value, parse};

fn corpus_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/toml-test")
}

// ============================================================================
// Minimal std-only JSON reader for toml-test expected .json files
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
enum Json {
    Str(String),
    Array(Vec<Json>),
    Obj(BTreeMap<String, Json>),
}

struct JsonParser<'a> {
    src: &'a str,
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> JsonParser<'a> {
    const fn new(src: &'a str) -> Self {
        Self {
            src,
            bytes: src.as_bytes(),
            pos: 0,
        }
    }

    fn peek(&self) -> Option<u8> {
        self.bytes.get(self.pos).copied()
    }

    fn skip_ws(&mut self) {
        while matches!(self.peek(), Some(b' ' | b'\t' | b'\r' | b'\n')) {
            self.pos += 1;
        }
    }

    fn parse_value(&mut self) -> Result<Json, String> {
        self.skip_ws();
        match self.peek() {
            Some(b'"') => self.parse_string().map(Json::Str),
            Some(b'[') => self.parse_array(),
            Some(b'{') => self.parse_object(),
            other => Err(format!("unexpected JSON token {:?} at {}", other, self.pos)),
        }
    }

    fn parse_string(&mut self) -> Result<String, String> {
        if self.peek() != Some(b'"') {
            return Err(format!("expected '\"' at {}", self.pos));
        }
        self.pos += 1;
        let mut out = String::new();
        while self.pos < self.bytes.len() {
            let ch = self.src[self.pos..]
                .chars()
                .next()
                .ok_or_else(|| "unexpected EOF in JSON string".to_owned())?;
            self.pos += ch.len_utf8();
            match ch {
                '"' => return Ok(out),
                '\\' => {
                    let esc = self.src[self.pos..]
                        .chars()
                        .next()
                        .ok_or_else(|| "truncated JSON escape".to_owned())?;
                    self.pos += esc.len_utf8();
                    match esc {
                        '"' => out.push('"'),
                        '\\' => out.push('\\'),
                        '/' => out.push('/'),
                        'b' => out.push('\u{0008}'),
                        'f' => out.push('\u{000C}'),
                        'n' => out.push('\n'),
                        'r' => out.push('\r'),
                        't' => out.push('\t'),
                        'u' => {
                            let u1 = self.parse_u16_hex()?;
                            if (0xD800..=0xDBFF).contains(&u1) {
                                if self.src.get(self.pos..self.pos + 2) != Some("\\u") {
                                    return Err("unpaired high surrogate in JSON".to_owned());
                                }
                                self.pos += 2;
                                let u2 = self.parse_u16_hex()?;
                                if !(0xDC00..=0xDFFF).contains(&u2) {
                                    return Err("invalid low surrogate in JSON".to_owned());
                                }
                                let cp = 0x10000
                                    + ((u32::from(u1) - 0xD800) << 10)
                                    + (u32::from(u2) - 0xDC00);
                                let scalar =
                                    char::from_u32(cp).ok_or_else(|| "bad surrogate pair".to_owned())?;
                                out.push(scalar);
                            } else {
                                let scalar = char::from_u32(u32::from(u1))
                                    .ok_or_else(|| format!("bad JSON codepoint {u1:04x}"))?;
                                out.push(scalar);
                            }
                        }
                        other => return Err(format!("unknown JSON escape \\{other}")),
                    }
                }
                c => out.push(c),
            }
        }
        Err("unterminated JSON string".to_owned())
    }

    fn parse_u16_hex(&mut self) -> Result<u16, String> {
        let slice = self
            .src
            .get(self.pos..self.pos + 4)
            .ok_or_else(|| "truncated \\uXXXX".to_owned())?;
        self.pos += 4;
        u16::from_str_radix(slice, 16).map_err(|e| format!("bad \\uXXXX `{slice}`: {e}"))
    }

    fn parse_array(&mut self) -> Result<Json, String> {
        self.pos += 1;
        self.skip_ws();
        let mut items = Vec::new();
        if self.peek() == Some(b']') {
            self.pos += 1;
            return Ok(Json::Array(items));
        }
        loop {
            items.push(self.parse_value()?);
            self.skip_ws();
            match self.peek() {
                Some(b',') => self.pos += 1,
                Some(b']') => {
                    self.pos += 1;
                    return Ok(Json::Array(items));
                }
                other => return Err(format!("expected ',' or ']' in JSON array, got {other:?}")),
            }
        }
    }

    fn parse_object(&mut self) -> Result<Json, String> {
        self.pos += 1;
        self.skip_ws();
        let mut map = BTreeMap::new();
        if self.peek() == Some(b'}') {
            self.pos += 1;
            return Ok(Json::Obj(map));
        }
        loop {
            self.skip_ws();
            let k = self.parse_string()?;
            self.skip_ws();
            if self.peek() != Some(b':') {
                return Err(format!("expected ':' in JSON object at {}", self.pos));
            }
            self.pos += 1;
            let v = self.parse_value()?;
            map.insert(k, v);
            self.skip_ws();
            match self.peek() {
                Some(b',') => self.pos += 1,
                Some(b'}') => {
                    self.pos += 1;
                    return Ok(Json::Obj(map));
                }
                other => return Err(format!("expected ',' or '}}' in JSON object, got {other:?}")),
            }
        }
    }
}

fn parse_json(text: &str) -> Result<Json, String> {
    let mut p = JsonParser::new(text);
    let val = p.parse_value()?;
    p.skip_ws();
    if p.pos != p.bytes.len() {
        return Err(format!("trailing JSON bytes at {}", p.pos));
    }
    Ok(val)
}

fn toml_value_to_corpus_json(v: &Value) -> Json {
    match v {
        Value::Str(s) => {
            let mut m = BTreeMap::new();
            m.insert("type".to_owned(), Json::Str("string".to_owned()));
            m.insert("value".to_owned(), Json::Str(s.clone()));
            Json::Obj(m)
        }
        Value::Int(n) => {
            let mut m = BTreeMap::new();
            m.insert("type".to_owned(), Json::Str("integer".to_owned()));
            m.insert("value".to_owned(), Json::Str(n.to_string()));
            Json::Obj(m)
        }
        Value::Bool(b) => {
            let mut m = BTreeMap::new();
            m.insert("type".to_owned(), Json::Str("bool".to_owned()));
            m.insert(
                "value".to_owned(),
                Json::Str(if *b { "true" } else { "false" }.to_owned()),
            );
            Json::Obj(m)
        }
        Value::Array(items) => {
            Json::Array(items.iter().map(|sp| toml_value_to_corpus_json(&sp.value)).collect())
        }
        Value::Table(t) => toml_table_to_corpus_json(t),
    }
}

fn toml_table_to_corpus_json(t: &Table) -> Json {
    let mut m = BTreeMap::new();
    for (k, sp) in t.iter() {
        m.insert(k.clone(), toml_value_to_corpus_json(&sp.value));
    }
    Json::Obj(m)
}

fn error_kind_label(kind: &TomlErrorKind) -> String {
    match kind {
        TomlErrorKind::Syntax(_) => "Syntax".to_owned(),
        TomlErrorKind::DuplicateKey(_) => "DuplicateKey".to_owned(),
        TomlErrorKind::TableRedefined(_) => "TableRedefined".to_owned(),
        TomlErrorKind::OutsideSubset(c) => format!("OutsideSubset({c})"),
        TomlErrorKind::IntegerOutOfRange => "IntegerOutOfRange".to_owned(),
        TomlErrorKind::InvalidUtf8 => "InvalidUtf8".to_owned(),
        TomlErrorKind::ByteOrderMark => "ByteOrderMark".to_owned(),
        TomlErrorKind::TooLarge => "TooLarge".to_owned(),
        TomlErrorKind::TooDeep => "TooDeep".to_owned(),
        TomlErrorKind::UnknownKey { .. } => "UnknownKey".to_owned(),
        TomlErrorKind::MissingKey { .. } => "MissingKey".to_owned(),
        TomlErrorKind::TypeMismatch { .. } => "TypeMismatch".to_owned(),
    }
}

fn read_case_list(path: &Path) -> Vec<String> {
    let text = fs::read_to_string(path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    text.lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && l.ends_with(".toml"))
        .map(str::to_owned)
        .collect()
}

#[test]
fn toml_test_1_0_0_conformance_corpus() {
    let root = corpus_root();
    assert!(
        root.join("LICENSE").is_file() && root.join("PIN.txt").is_file(),
        "vendored corpus must include LICENSE and PIN.txt"
    );

    let cases_10 = read_case_list(&root.join("files-toml-1.0.0"));
    let cases_11 = read_case_list(&root.join("files-toml-1.1.0"));
    let set_10: BTreeSet<&str> = cases_10.iter().map(String::as_str).collect();
    let excluded_11_count = cases_11
        .iter()
        .filter(|c| !set_10.contains(c.as_str()))
        .count();

    assert!(
        cases_10.len() >= 600,
        "expected full toml-test 1.0.0 corpus, found only {} cases",
        cases_10.len()
    );

    let mut report_lines: Vec<String> = Vec::with_capacity(cases_10.len() + 2);
    let mut valid_parsed = 0usize;
    let mut valid_outside_subset = 0usize;
    let mut invalid_refused = 0usize;
    let mut failures: Vec<String> = Vec::new();

    for rel in &cases_10 {
        let toml_path = root.join(rel);
        let bytes = fs::read(&toml_path)
            .unwrap_or_else(|e| panic!("missing vendored fixture {}: {e}", toml_path.display()));
        let case_name = rel.strip_suffix(".toml").unwrap_or(rel);

        if rel.starts_with("valid/") {
            let json_path = root.join(format!("{case_name}.json"));
            let json_text = fs::read_to_string(&json_path)
                .unwrap_or_else(|e| panic!("missing expected JSON {}: {e}", json_path.display()));
            let expected_json = parse_json(&json_text)
                .unwrap_or_else(|e| panic!("failed parsing expected JSON {}: {e}", json_path.display()));

            match parse(&bytes) {
                Ok(table) => {
                    let actual_json = toml_table_to_corpus_json(&table);
                    if actual_json == expected_json {
                        valid_parsed += 1;
                        report_lines.push(format!(
                            "{case_name}\taccept\taccept\tpass\tOk\t0:0\t{rel}"
                        ));
                    } else {
                        failures.push(format!(
                            "VALID VALUE MISMATCH {rel}: expected={expected_json:?} actual={actual_json:?}"
                        ));
                        report_lines.push(format!(
                            "{case_name}\taccept\tmismatch\tfail\tValueMismatch\t0:0\t{rel}"
                        ));
                    }
                }
                Err(err) => {
                    let kind_s = error_kind_label(&err.kind);
                    let pos = format!("{}:{}", err.line, err.column);
                    if matches!(err.kind, TomlErrorKind::OutsideSubset(_)) {
                        valid_outside_subset += 1;
                        report_lines.push(format!(
                            "{case_name}\taccept\toutside-subset\tpass\t{kind_s}\t{pos}\t{rel}"
                        ));
                    } else {
                        failures.push(format!(
                            "VALID REFUSED AS NON-SUBSET {rel}: {err} ({kind_s})"
                        ));
                        report_lines.push(format!(
                            "{case_name}\taccept\trefused-non-subset\tfail\t{kind_s}\t{pos}\t{rel}"
                        ));
                    }
                }
            }
        } else if rel.starts_with("invalid/") {
            match parse(&bytes) {
                Err(err) => {
                    invalid_refused += 1;
                    let kind_s = error_kind_label(&err.kind);
                    let pos = format!("{}:{}", err.line, err.column);
                    report_lines.push(format!(
                        "{case_name}\trefuse\trefuse\tpass\t{kind_s}\t{pos}\t{rel}"
                    ));
                }
                Ok(table) => {
                    failures.push(format!(
                        "INVALID ACCEPTED {rel}: parsed into {table:?}"
                    ));
                    report_lines.push(format!(
                        "{case_name}\trefuse\taccept\tfail\tAccepted\t0:0\t{rel}"
                    ));
                }
            }
        } else {
            panic!("unexpected case path in files-toml-1.0.0: {rel}");
        }
    }

    if let Ok(report_path) = env::var("FROB_TOML_CORPUS_REPORT")
        && !report_path.is_empty()
    {
        let mut f = fs::File::create(&report_path)
            .unwrap_or_else(|e| panic!("create corpus report {report_path}: {e}"));
        writeln!(
            f,
            "#SUMMARY\ttotal={}\tvalid_parsed={valid_parsed}\tvalid_outside_subset={valid_outside_subset}\tinvalid_refused={invalid_refused}\tfailed={}\texcluded_toml_1_1={excluded_11_count}",
            cases_10.len(),
            failures.len()
        )
        .expect("write summary");
        for line in &report_lines {
            writeln!(f, "{line}").expect("write case line");
        }
    }

    println!(
        "TOML_CORPUS_SUMMARY total={} valid_parsed={} valid_outside_subset={} invalid_refused={} failed={} excluded_toml_1_1={}",
        cases_10.len(),
        valid_parsed,
        valid_outside_subset,
        invalid_refused,
        failures.len(),
        excluded_11_count
    );

    assert!(
        failures.is_empty(),
        "toml-test 1.0.0 conformance failures ({} / {}):\n{}",
        failures.len(),
        cases_10.len(),
        failures.join("\n")
    );
}

#[test]
fn toml_reads_repo_subset_files() {
    let crate_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let repo_root = crate_dir.parent().and_then(Path::parent).expect("repo root");
    let subset_files = [
        crate_dir.join("Cargo.toml"),
        repo_root.join("Cargo.toml"),
        repo_root.join("rust-toolchain.toml"),
        repo_root.join("franken-stack.lock"),
        repo_root.join("registries/siblings.toml"),
        repo_root.join(".cargo/config.toml"),
        repo_root.join("Cargo.lock"),
    ];
    for path in &subset_files {
        assert!(path.is_file(), "expected repo TOML file: {}", path.display());
        let bytes = fs::read(path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
        parse(&bytes).unwrap_or_else(|e| panic!("repo file {} failed strict TOML subset: {e}", path.display()));
    }
}
