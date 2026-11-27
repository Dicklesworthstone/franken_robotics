#![deny(unsafe_code)]

//! Independent golden corpus harness for `frob.design/1` canonical form and
//! `DesignId` (`N14a-26`).
//!
//! `check(workspace_root, canon)` runs any canonicalizer (bytes in -> canonical
//! UTF-8 bytes + `DesignId`, or refusal reason class out) against every case in
//! `crates/frob-conformance/fixtures/design_golden/manifest.toml` and compares
//! bytes and `DesignId` against expectations produced exclusively by the
//! out-of-process reference canonicalizer under `tools/oracle/design_canon/`.
//!
//! Reads `manifest.toml` through `frob_core::registry::toml` and hashes with
//! `frob_cert::blake3` (never reads or shares `frob_core::json` or
//! `frob_core::canon`). Never deletes files or directories (COMMON.md 9b).

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use frob_cert::blake3::{self, Hasher};
use frob_cert::json::{JsonValue, parse_non_canonical};
use frob_cert::limits::COMPILED_LIMITS;
use frob_core::registry::toml::{self, Fields, Table, Value};

/// Workspace-relative directory holding the `frob.design/1` golden corpus.
pub const CORPUS_REL_DIR: &str = "crates/frob-conformance/fixtures/design_golden";
/// Workspace-relative path of the golden corpus manifest.
pub const MANIFEST_REL_PATH: &str = "crates/frob-conformance/fixtures/design_golden/manifest.toml";
/// Workspace-relative directory of the independent Python reference canonicalizer.
pub const ORACLE_REL_DIR: &str = "tools/oracle/design_canon";
/// Schema identifier of `manifest.toml`.
pub const MANIFEST_SCHEMA: &str = "frob.design.golden-manifest/1";
/// Domain separation context for `DesignId` (BLAKE3 `derive_key`).
pub const DESIGN_DOMAIN: &str = "frob.design/1";

/// Required corpus groups in `manifest.toml`.
pub const REQUIRED_GROUPS: &[&str] = &[
    "worked_designs",
    "key_order",
    "quantities",
    "annotated",
    "wide_values",
    "defaults",
    "constructors",
    "refusals",
];

const ORACLE_SCRIPT_FILES: &[&str] = &["blake3_ref.py", "units.py", "canon.py"];
const ZERO_HEX64: &str = "0000000000000000000000000000000000000000000000000000000000000000";

/// Expected disposition for one corpus entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CaseExpect {
    Accept,
    Refuse,
}

impl CaseExpect {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Accept => "accept",
            Self::Refuse => "refuse",
        }
    }
}

/// One `[[case]]` entry in `crates/frob-conformance/fixtures/design_golden/manifest.toml`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GoldenCase {
    pub id: String,
    pub group: String,
    pub input: String,
    pub purpose: String,
    pub expect: CaseExpect,
    pub input_blake3: String,
    pub canon: String,
    pub canon_blake3: String,
    pub design_id: String,
    pub reason: String,
    pub same_id_as: String,
}

/// Parsed golden corpus manifest (`frob.design.golden-manifest/1`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GoldenManifest {
    pub schema: String,
    pub oracle: String,
    pub oracle_blake3: String,
    pub manifest_blake3: String,
    pub cases: Vec<GoldenCase>,
}

/// Output produced by any canonicalizer under test for one `.fbot` input.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CanonOutcome {
    Accepted {
        canonical_bytes: Vec<u8>,
        design_id: String,
    },
    Refused {
        reason_class: String,
        detail: String,
    },
}

/// Diagnostic describing one mismatch between a canonicalizer under test and the golden corpus.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mismatch {
    pub case_id: String,
    pub group: String,
    pub file: String,
    pub first_diff_offset: Option<usize>,
    pub expected_context: String,
    pub observed_context: String,
    pub detail: String,
}

impl fmt::Display for Mismatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.first_diff_offset {
            Some(off) => write!(
                f,
                "[{}] {} (group={}) byte offset {}: {} | expected=`{}` observed=`{}`",
                self.case_id,
                self.file,
                self.group,
                off,
                self.detail,
                self.expected_context,
                self.observed_context
            ),
            None => write!(
                f,
                "[{}] {} (group={}): {}",
                self.case_id, self.file, self.group, self.detail
            ),
        }
    }
}

/// Summary returned when every case in the manifest matches.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckSummary {
    pub total: usize,
    pub accepted: usize,
    pub refused: usize,
    pub oracle_blake3: String,
    pub manifest_blake3: String,
}

/// Compute `DesignId` (`d:<64 hex>`) from canonical UTF-8 bytes via BLAKE3 derive-key.
#[must_use]
pub fn compute_design_id(canonical_bytes: &[u8]) -> String {
    let digest = blake3::derive_key(DESIGN_DOMAIN, canonical_bytes);
    format!("d:{}", blake3::to_hex(&digest))
}

/// Compute plain lowercase 64-hex BLAKE3 digest of raw bytes.
#[must_use]
pub fn hash_bytes_hex(bytes: &[u8]) -> String {
    blake3::to_hex(&blake3::hash(bytes))
}

/// Compute the composite BLAKE3 hash of the reference canonicalizer modules in `tools/oracle/design_canon/`.
pub fn compute_oracle_blake3(workspace_root: &Path) -> Result<String, String> {
    let dir = workspace_root.join(ORACLE_REL_DIR);
    let mut h = Hasher::new();
    for fname in ORACLE_SCRIPT_FILES {
        let p = dir.join(fname);
        let data = fs::read(&p)
            .map_err(|e| format!("failed to read reference script `{}`: {e}", p.display()))?;
        let header = format!("file:{fname}:{}\n", data.len());
        h.update(header.as_bytes());
        h.update(&data);
        h.update(b"\n");
    }
    Ok(blake3::to_hex(&h.finalize()))
}

/// Compute the self-authenticating BLAKE3 hash of `manifest.toml` (with `manifest_blake3` masked to 64 zeros).
pub fn compute_manifest_blake3(manifest_text: &str) -> Result<String, String> {
    let mut replaced = 0usize;
    let mut masked = String::with_capacity(manifest_text.len());
    for chunk in manifest_text.split_inclusive('\n') {
        let trimmed = chunk.trim_start();
        if let Some(rest) = trimmed.strip_prefix("manifest_blake3")
            && rest.trim_start().starts_with('=')
        {
            let indent_len = chunk.len() - trimmed.len();
            masked.push_str(&chunk[..indent_len]);
            masked.push_str("manifest_blake3 = \"");
            masked.push_str(ZERO_HEX64);
            masked.push('"');
            if chunk.ends_with("\r\n") {
                masked.push_str("\r\n");
            } else if chunk.ends_with('\n') {
                masked.push('\n');
            }
            replaced += 1;
        } else {
            masked.push_str(chunk);
        }
    }
    if replaced != 1 {
        return Err(format!(
            "expected exactly 1 `manifest_blake3` line in manifest.toml, found {replaced}"
        ));
    }
    Ok(hash_bytes_hex(masked.as_bytes()))
}

fn is_hex64(s: &str) -> bool {
    s.len() == 64 && s.bytes().all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f'))
}

fn is_design_id(s: &str) -> bool {
    s.strip_prefix("d:").is_some_and(is_hex64)
}

fn parse_case_row(tbl: Table, idx: usize) -> Result<GoldenCase, String> {
    let ctx = format!("case[{idx}]");
    let mut f = Fields::new(tbl, ctx.as_str());
    let err = |e: toml::TomlError| format!("{ctx}: {e}");
    let id = f.require_str("id").map_err(err)?;
    let group = f.require_str("group").map_err(err)?;
    let input = f.require_str("input").map_err(err)?;
    let purpose = f.require_str("purpose").map_err(err)?;
    let expect_s = f.require_str("expect").map_err(err)?;
    let input_blake3 = f.require_str("input_blake3").map_err(err)?;
    let canon = f.require_str("canon").map_err(err)?;
    let canon_blake3 = f.require_str("canon_blake3").map_err(err)?;
    let design_id = f.require_str("design_id").map_err(err)?;
    let reason = f.require_str("reason").map_err(err)?;
    let same_id_as = f.require_str("same_id_as").map_err(err)?;
    f.finish().map_err(err)?;

    let expect = match expect_s.as_str() {
        "accept" => CaseExpect::Accept,
        "refuse" => CaseExpect::Refuse,
        other => {
            return Err(format!(
                "{ctx}: invalid expect `{other}` (must be accept|refuse)"
            ));
        }
    };
    if id.is_empty() {
        return Err(format!("{ctx}: empty case id"));
    }
    if !REQUIRED_GROUPS.contains(&group.as_str()) {
        return Err(format!("{ctx}: unknown group `{group}`"));
    }
    if purpose.trim().is_empty() {
        return Err(format!("{ctx} ({id}): purpose must be non-empty"));
    }
    if !is_hex64(&input_blake3) {
        return Err(format!(
            "{ctx} ({id}): invalid input_blake3 `{input_blake3}`"
        ));
    }
    match expect {
        CaseExpect::Accept => {
            if canon.is_empty() || !canon.ends_with(".canon") {
                return Err(format!(
                    "{ctx} ({id}): accepted case requires `.canon` path"
                ));
            }
            if !is_hex64(&canon_blake3) {
                return Err(format!(
                    "{ctx} ({id}): invalid canon_blake3 `{canon_blake3}`"
                ));
            }
            if !is_design_id(&design_id) {
                return Err(format!("{ctx} ({id}): invalid design_id `{design_id}`"));
            }
            if !reason.is_empty() {
                return Err(format!(
                    "{ctx} ({id}): accepted case must have empty reason"
                ));
            }
        }
        CaseExpect::Refuse => {
            if !canon.is_empty() || !canon_blake3.is_empty() || !design_id.is_empty() {
                return Err(format!(
                    "{ctx} ({id}): refusal case must have empty canon/canon_blake3/design_id"
                ));
            }
            if reason.trim().is_empty() {
                return Err(format!(
                    "{ctx} ({id}): refusal case must state reason class"
                ));
            }
        }
    }

    Ok(GoldenCase {
        id,
        group,
        input,
        purpose,
        expect,
        input_blake3,
        canon,
        canon_blake3,
        design_id,
        reason,
        same_id_as,
    })
}

/// Parse `manifest.toml` using `frob_core::registry::toml` without modifying files.
pub fn load_manifest(manifest_path: &Path) -> Result<GoldenManifest, String> {
    let text = fs::read_to_string(manifest_path)
        .map_err(|e| format!("failed to read `{}`: {e}", manifest_path.display()))?;
    let root = toml::parse(text.as_bytes())
        .map_err(|e| format!("failed to parse `{}`: {e}", manifest_path.display()))?;
    let mut f = Fields::new(root, "design_golden/manifest.toml");
    let err = |e: toml::TomlError| format!("manifest.toml: {e}");
    let schema = f.require_str("schema").map_err(err)?;
    let oracle = f.require_str("oracle").map_err(err)?;
    let oracle_blake3 = f.require_str("oracle_blake3").map_err(err)?;
    let manifest_blake3 = f.require_str("manifest_blake3").map_err(err)?;
    let raw_cases = f.require_array("case").map_err(err)?;
    f.finish().map_err(err)?;

    if schema != MANIFEST_SCHEMA {
        return Err(format!(
            "manifest.toml: expected schema `{MANIFEST_SCHEMA}`, got `{schema}`"
        ));
    }
    if !is_hex64(&oracle_blake3) {
        return Err(format!(
            "manifest.toml: invalid oracle_blake3 `{oracle_blake3}`"
        ));
    }
    if !is_hex64(&manifest_blake3) {
        return Err(format!(
            "manifest.toml: invalid manifest_blake3 `{manifest_blake3}`"
        ));
    }

    let computed_manifest_hash = compute_manifest_blake3(&text)?;
    if computed_manifest_hash != manifest_blake3 {
        return Err(format!(
            "manifest.toml: manifest_blake3 mismatch (stored `{manifest_blake3}`, computed `{computed_manifest_hash}`)"
        ));
    }

    let mut cases = Vec::with_capacity(raw_cases.len());
    let mut seen_ids = BTreeSet::new();
    for (idx, item) in raw_cases.into_iter().enumerate() {
        let Value::Table(tbl) = item.value else {
            return Err(format!("manifest.toml: case[{idx}] must be a table"));
        };
        let row = parse_case_row(tbl, idx)?;
        if !seen_ids.insert(row.id.clone()) {
            return Err(format!("manifest.toml: duplicate case id `{}`", row.id));
        }
        cases.push(row);
    }

    Ok(GoldenManifest {
        schema,
        oracle,
        oracle_blake3,
        manifest_blake3,
        cases,
    })
}

/// Verify manifest hash, reference script hash, every input/expectation hash,
/// `same_id_as` pairs, group coverage, and zero stray files in the corpus dir.
pub fn verify_manifest_integrity(workspace_root: &Path) -> Result<GoldenManifest, Vec<String>> {
    verify_manifest_integrity_in(workspace_root, &workspace_root.join(CORPUS_REL_DIR))
}

/// Verify manifest integrity for an arbitrary corpus directory (supports scratch copies).
pub fn verify_manifest_integrity_in(
    workspace_root: &Path,
    corpus_dir: &Path,
) -> Result<GoldenManifest, Vec<String>> {
    let manifest_path = corpus_dir.join("manifest.toml");
    let manifest = match load_manifest(&manifest_path) {
        Ok(m) => m,
        Err(e) => return Err(vec![e]),
    };

    let mut errors = Vec::new();
    match compute_oracle_blake3(workspace_root) {
        Ok(actual_oracle) => {
            if actual_oracle != manifest.oracle_blake3 {
                errors.push(format!(
                    "oracle_blake3 mismatch: manifest has `{}`, tree reference has `{actual_oracle}`",
                    manifest.oracle_blake3
                ));
            }
        }
        Err(e) => errors.push(e),
    }

    if manifest.cases.len() < 40 {
        errors.push(format!(
            "corpus must hold at least 40 inputs, found {}",
            manifest.cases.len()
        ));
    }

    let mut groups_seen = BTreeSet::new();
    let mut expected_files = BTreeSet::new();
    expected_files.insert("manifest.toml".to_owned());
    expected_files.insert("PIN.txt".to_owned());

    let by_id: BTreeMap<&str, &GoldenCase> =
        manifest.cases.iter().map(|c| (c.id.as_str(), c)).collect();

    for c in &manifest.cases {
        groups_seen.insert(c.group.as_str());
        expected_files.insert(c.input.clone());

        let inp_path = corpus_dir.join(&c.input);
        match fs::read(&inp_path) {
            Err(e) => errors.push(format!("missing input `{}`: {e}", c.input)),
            Ok(bytes) => {
                let got = hash_bytes_hex(&bytes);
                if got != c.input_blake3 {
                    errors.push(format!(
                        "input `{}` blake3 mismatch: manifest `{}`, disk `{got}`",
                        c.input, c.input_blake3
                    ));
                }
            }
        }

        if c.expect == CaseExpect::Accept {
            expected_files.insert(c.canon.clone());
            let canon_path = corpus_dir.join(&c.canon);
            match fs::read(&canon_path) {
                Err(e) => errors.push(format!("missing expectation `{}`: {e}", c.canon)),
                Ok(bytes) => {
                    let got_hash = hash_bytes_hex(&bytes);
                    if got_hash != c.canon_blake3 {
                        errors.push(format!(
                            "expectation `{}` blake3 mismatch: manifest `{}`, disk `{got_hash}`",
                            c.canon, c.canon_blake3
                        ));
                    }
                    let got_id = compute_design_id(&bytes);
                    if got_id != c.design_id {
                        errors.push(format!(
                            "expectation `{}` DesignId mismatch: manifest `{}`, derived `{got_id}`",
                            c.canon, c.design_id
                        ));
                    }
                }
            }
        }

        if !c.same_id_as.is_empty() {
            match by_id.get(c.same_id_as.as_str()) {
                None => errors.push(format!(
                    "case `{}` references unknown same_id_as `{}`",
                    c.id, c.same_id_as
                )),
                Some(peer) => {
                    if c.design_id != peer.design_id || c.canon_blake3 != peer.canon_blake3 {
                        errors.push(format!(
                            "pair `{}` and `{}` must share one DesignId and canonical bytes (`{}` vs `{}`)",
                            c.id, peer.id, c.design_id, peer.design_id
                        ));
                    }
                }
            }
        }
    }

    for &g in REQUIRED_GROUPS {
        if !groups_seen.contains(g) {
            errors.push(format!("manifest missing required corpus group `{g}`"));
        }
    }

    // Check for stray files in corpus_dir.
    match fs::read_dir(corpus_dir) {
        Err(e) => errors.push(format!("failed to read `{}`: {e}", corpus_dir.display())),
        Ok(rd) => {
            let mut entries: Vec<PathBuf> = rd.flatten().map(|e| e.path()).collect();
            entries.sort();
            for p in entries {
                let name = p
                    .file_name()
                    .map(|s| s.to_string_lossy().into_owned())
                    .unwrap_or_default();
                if p.is_dir() || !expected_files.contains(&name) {
                    errors.push(format!("stray entry in design_golden corpus: `{name}`"));
                }
            }
        }
    }

    if errors.is_empty() {
        Ok(manifest)
    } else {
        Err(errors)
    }
}

fn first_diff_byte(expected: &[u8], observed: &[u8]) -> Option<usize> {
    let min_len = expected.len().min(observed.len());
    for i in 0..min_len {
        if expected[i] != observed[i] {
            return Some(i);
        }
    }
    if expected.len() != observed.len() {
        Some(min_len)
    } else {
        None
    }
}

fn format_byte_window(bytes: &[u8], offset: usize) -> String {
    let start = offset.saturating_sub(10);
    let end = (offset + 10).min(bytes.len());
    let slice = &bytes[start..end];
    String::from_utf8_lossy(slice)
        .chars()
        .map(|c| {
            if c.is_ascii_graphic() || c == ' ' {
                c.to_string()
            } else {
                format!("\\x{:02x}", c as u32)
            }
        })
        .collect()
}

/// Compare any canonicalizer against every case in a loaded manifest and corpus directory.
pub fn check_manifest<F>(
    manifest: &GoldenManifest,
    corpus_dir: &Path,
    mut canon: F,
) -> Result<CheckSummary, Vec<Mismatch>>
where
    F: FnMut(&[u8]) -> CanonOutcome,
{
    let mut mismatches = Vec::new();
    let mut accepted = 0usize;
    let mut refused = 0usize;

    for c in &manifest.cases {
        let inp_path = corpus_dir.join(&c.input);
        let inp_bytes = match fs::read(&inp_path) {
            Ok(b) => b,
            Err(e) => {
                mismatches.push(Mismatch {
                    case_id: c.id.clone(),
                    group: c.group.clone(),
                    file: c.input.clone(),
                    first_diff_offset: None,
                    expected_context: String::new(),
                    observed_context: String::new(),
                    detail: format!("failed to read input file: {e}"),
                });
                continue;
            }
        };

        let outcome = canon(&inp_bytes);
        match c.expect {
            CaseExpect::Accept => {
                accepted += 1;
                let canon_path = corpus_dir.join(&c.canon);
                let expected_bytes = match fs::read(&canon_path) {
                    Ok(b) => b,
                    Err(e) => {
                        mismatches.push(Mismatch {
                            case_id: c.id.clone(),
                            group: c.group.clone(),
                            file: c.canon.clone(),
                            first_diff_offset: None,
                            expected_context: String::new(),
                            observed_context: String::new(),
                            detail: format!("failed to read expectation file: {e}"),
                        });
                        continue;
                    }
                };
                match outcome {
                    CanonOutcome::Refused {
                        reason_class,
                        detail,
                    } => {
                        mismatches.push(Mismatch {
                            case_id: c.id.clone(),
                            group: c.group.clone(),
                            file: c.input.clone(),
                            first_diff_offset: None,
                            expected_context: format!("accept ({})", c.design_id),
                            observed_context: format!("refuse ({reason_class})"),
                            detail: format!("unexpected refusal `{reason_class}`: {detail}"),
                        });
                    }
                    CanonOutcome::Accepted {
                        canonical_bytes,
                        design_id,
                    } => {
                        if let Some(off) = first_diff_byte(&expected_bytes, &canonical_bytes) {
                            mismatches.push(Mismatch {
                                case_id: c.id.clone(),
                                group: c.group.clone(),
                                file: c.canon.clone(),
                                first_diff_offset: Some(off),
                                expected_context: format_byte_window(&expected_bytes, off),
                                observed_context: format_byte_window(&canonical_bytes, off),
                                detail: format!(
                                    "canonical bytes differ (expected {} B, observed {} B)",
                                    expected_bytes.len(),
                                    canonical_bytes.len()
                                ),
                            });
                        } else if design_id != c.design_id {
                            mismatches.push(Mismatch {
                                case_id: c.id.clone(),
                                group: c.group.clone(),
                                file: c.canon.clone(),
                                first_diff_offset: Some(0),
                                expected_context: c.design_id.clone(),
                                observed_context: design_id.clone(),
                                detail: format!(
                                    "DesignId mismatch despite identical bytes (expected `{}`, observed `{design_id}`)",
                                    c.design_id
                                ),
                            });
                        }
                    }
                }
            }
            CaseExpect::Refuse => {
                refused += 1;
                match outcome {
                    CanonOutcome::Accepted { design_id, .. } => {
                        mismatches.push(Mismatch {
                            case_id: c.id.clone(),
                            group: c.group.clone(),
                            file: c.input.clone(),
                            first_diff_offset: None,
                            expected_context: format!("refuse ({})", c.reason),
                            observed_context: format!("accept ({design_id})"),
                            detail: format!(
                                "refusal case unexpectedly accepted with DesignId `{design_id}`"
                            ),
                        });
                    }
                    CanonOutcome::Refused {
                        reason_class,
                        detail,
                    } => {
                        if reason_class != c.reason {
                            mismatches.push(Mismatch {
                                case_id: c.id.clone(),
                                group: c.group.clone(),
                                file: c.input.clone(),
                                first_diff_offset: None,
                                expected_context: c.reason.clone(),
                                observed_context: reason_class.clone(),
                                detail: format!(
                                    "wrong refusal reason class (expected `{}`, got `{reason_class}`: {detail})",
                                    c.reason
                                ),
                            });
                        }
                    }
                }
            }
        }
    }

    if mismatches.is_empty() {
        Ok(CheckSummary {
            total: manifest.cases.len(),
            accepted,
            refused,
            oracle_blake3: manifest.oracle_blake3.clone(),
            manifest_blake3: manifest.manifest_blake3.clone(),
        })
    } else {
        Err(mismatches)
    }
}

/// Top-level comparison harness (`frob_conformance::design_golden::check`) over
/// any canonicalizer against the repository's golden corpus.
pub fn check<F>(workspace_root: &Path, canon: F) -> Result<CheckSummary, Vec<Mismatch>>
where
    F: FnMut(&[u8]) -> CanonOutcome,
{
    let corpus_dir = workspace_root.join(CORPUS_REL_DIR);
    let manifest = load_manifest(&corpus_dir.join("manifest.toml")).map_err(|e| {
        vec![Mismatch {
            case_id: "manifest".to_owned(),
            group: "manifest".to_owned(),
            file: MANIFEST_REL_PATH.to_owned(),
            first_diff_offset: None,
            expected_context: String::new(),
            observed_context: String::new(),
            detail: e,
        }]
    })?;
    check_manifest(&manifest, &corpus_dir, canon)
}

fn decode_hex(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 {
        return Err("odd hex length".to_owned());
    }
    let bytes = s.as_bytes();
    let mut out = Vec::with_capacity(bytes.len() / 2);
    let mut i = 0;
    while i < bytes.len() {
        let hi = hex_nibble(bytes[i])?;
        let lo = hex_nibble(bytes[i + 1])?;
        out.push((hi << 4) | lo);
        i += 2;
    }
    Ok(out)
}

fn hex_nibble(b: u8) -> Result<u8, String> {
    match b {
        b'0'..=b'9' => Ok(b - b'0'),
        b'a'..=b'f' => Ok(b - b'a' + 10),
        b'A'..=b'F' => Ok(b - b'A' + 10),
        _ => Err(format!("non-hex byte 0x{b:02x}")),
    }
}

fn json_str_field<'a>(obj: &'a [(String, JsonValue)], key: &str) -> Option<&'a str> {
    obj.iter().find_map(|(k, v)| {
        if k == key {
            if let JsonValue::Str(s) = v {
                return Some(s.as_str());
            }
        }
        None
    })
}

/// Invoke the out-of-process reference canonicalizer (`tools/oracle/design_canon/canon.py`)
/// in the chosen mode (`ref`, `utf16-keys`, `bin64-roundtrip`, `bin64-exact`, `no-defaults`).
#[must_use]
pub fn invoke_reference(workspace_root: &Path, mode: &str, input: &[u8]) -> CanonOutcome {
    let script = workspace_root.join(ORACLE_REL_DIR).join("canon.py");
    let mut child = match Command::new("python3")
        .arg(&script)
        .arg("--mode")
        .arg(mode)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(c) => c,
        Err(e) => {
            return CanonOutcome::Refused {
                reason_class: "oracle-spawn-error".to_owned(),
                detail: format!("failed to spawn python3 {}: {e}", script.display()),
            };
        }
    };

    if let Some(mut stdin) = child.stdin.take()
        && let Err(e) = stdin.write_all(input)
    {
        return CanonOutcome::Refused {
            reason_class: "oracle-io-error".to_owned(),
            detail: format!("failed writing stdin to canon.py: {e}"),
        };
    }

    let output = match child.wait_with_output() {
        Ok(o) => o,
        Err(e) => {
            return CanonOutcome::Refused {
                reason_class: "oracle-io-error".to_owned(),
                detail: format!("failed waiting on canon.py: {e}"),
            };
        }
    };

    let parsed = match parse_non_canonical(&output.stdout, &COMPILED_LIMITS) {
        Ok(JsonValue::Obj(pairs)) => pairs,
        Ok(_) => {
            return CanonOutcome::Refused {
                reason_class: "oracle-protocol-error".to_owned(),
                detail: "canon.py did not return a JSON object".to_owned(),
            };
        }
        Err(e) => {
            let stderr_s = String::from_utf8_lossy(&output.stderr);
            return CanonOutcome::Refused {
                reason_class: "oracle-protocol-error".to_owned(),
                detail: format!("invalid JSON from canon.py ({e}); stderr={stderr_s}"),
            };
        }
    };

    match json_str_field(&parsed, "status") {
        Some("ok") => {
            let hex = json_str_field(&parsed, "canon_hex").unwrap_or("");
            let did = json_str_field(&parsed, "design_id")
                .unwrap_or("")
                .to_owned();
            match decode_hex(hex) {
                Ok(canonical_bytes) => CanonOutcome::Accepted {
                    canonical_bytes,
                    design_id: did,
                },
                Err(e) => CanonOutcome::Refused {
                    reason_class: "oracle-protocol-error".to_owned(),
                    detail: format!("bad canon_hex from canon.py: {e}"),
                },
            }
        }
        Some("refused") => CanonOutcome::Refused {
            reason_class: json_str_field(&parsed, "reason")
                .unwrap_or("unknown")
                .to_owned(),
            detail: json_str_field(&parsed, "detail").unwrap_or("").to_owned(),
        },
        other => CanonOutcome::Refused {
            reason_class: "oracle-protocol-error".to_owned(),
            detail: format!("unexpected status `{other:?}` from canon.py"),
        },
    }
}
