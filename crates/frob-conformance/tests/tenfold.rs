#![deny(unsafe_code)]

//! Ten-fold harness skeleton, five records, canonical bytes, exact rendering,
//! append-only store, and scorer boundary lint tests (bead `N39a-01`).
//!
//! Run as: `cargo test -p frob-conformance --test tenfold -- records::`
//! Never deletes files or directories (COMMON.md 9b).

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

/// Authoritative list of scorer-boundary source files relative to
/// `crates/frob-conformance/src/` (`N39a-01`, DESIGN).
///
/// Read by the boundary lint (`records::scorer_boundary_lint`) and matched
/// one-to-one by `#[path]` modules in `kit/scorer/src/lib.rs`.
pub const BOUNDARY_FILES: &[&str] = &[
    "tenfold/ids.rs",
    "tenfold/refusal.rs",
    "tenfold/canon.rs",
    "tenfold/records/mission.rs",
    "tenfold/records/freeze.rs",
    "tenfold/records/ledger.rs",
    "tenfold/records/receipt.rs",
    "tenfold/records/report.rs",
    "tenfold/score/ratio.rs",
    "tenfold/score/bootstrap.rs",
    "tenfold/score/guard.rs",
    "tenfold/score/noninferiority.rs",
    "tenfold/score/verdict.rs",
    "tenfold/score/warm.rs",
    "tenfold/score/monitor.rs",
    "tenfold/score/ceiling.rs",
    "tenfold/score/preanalysis.rs",
    "tenfold/score/normal.rs",
    "tenfold/score/rng.rs",
];

const BANNED_MATH_METHODS: &[&str] = &[
    "ln", "ln_1p", "log", "log2", "log10", "exp", "exp2", "exp_m1", "powf", "powi", "sin", "cos",
    "tan", "asin", "acos", "atan", "atan2", "sinh", "cosh", "tanh", "asinh", "acosh", "atanh",
    "hypot", "cbrt", "gamma", "erf", "erfc",
];

static SCRATCH_SEQ: AtomicU64 = AtomicU64::new(0);

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crates/frob-conformance must live two levels below repo root")
        .to_path_buf()
}

/// Allocate a fresh scratch directory under `std::env::temp_dir()`. Never deletes files.
fn fresh_scratch(tag: &str) -> PathBuf {
    let seq = SCRATCH_SEQ.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!(
        "frob-n39a01-{tag}-{}-{seq}",
        std::process::id()
    ));
    fs::create_dir_all(&dir).expect("create scratch dir");
    dir
}

/// Strip Rust line comments, nested block comments, and string/raw-string/char
/// literals so prose comments or string constants do not cause false positives.
fn strip_rust_comments_and_literals(src: &str) -> String {
    let b = src.as_bytes();
    let n = b.len();
    let mut out = String::with_capacity(n);
    let mut i = 0usize;
    while i < n {
        if i + 1 < n && b[i] == b'/' && b[i + 1] == b'/' {
            while i < n && b[i] != b'\n' {
                i += 1;
            }
            continue;
        }
        if i + 1 < n && b[i] == b'/' && b[i + 1] == b'*' {
            let mut depth = 1usize;
            i += 2;
            while i < n && depth > 0 {
                if i + 1 < n && b[i] == b'/' && b[i + 1] == b'*' {
                    depth += 1;
                    i += 2;
                } else if i + 1 < n && b[i] == b'*' && b[i + 1] == b'/' {
                    depth -= 1;
                    i += 2;
                } else {
                    if b[i] == b'\n' {
                        out.push('\n');
                    }
                    i += 1;
                }
            }
            out.push(' ');
            continue;
        }
        // Raw strings: r#"..."#, r"...", br#"..."#
        let prev_ident = i > 0 && (b[i - 1].is_ascii_alphanumeric() || b[i - 1] == b'_');
        if !prev_ident && (b[i] == b'r' || (b[i] == b'b' && i + 1 < n && b[i + 1] == b'r')) {
            let mut j = if b[i] == b'b' { i + 2 } else { i + 1 };
            let mut hashes = 0usize;
            while j < n && b[j] == b'#' {
                hashes += 1;
                j += 1;
            }
            if j < n && b[j] == b'"' {
                j += 1;
                out.push_str("\"\"");
                while j < n {
                    if b[j] == b'"' {
                        let mut k = j + 1;
                        let mut matched = 0usize;
                        while matched < hashes && k < n && b[k] == b'#' {
                            matched += 1;
                            k += 1;
                        }
                        if matched == hashes {
                            j = k;
                            break;
                        }
                    }
                    if b[j] == b'\n' {
                        out.push('\n');
                    }
                    j += 1;
                }
                i = j;
                continue;
            }
        }
        if b[i] == b'"' {
            i += 1;
            out.push_str("\"\"");
            while i < n {
                if b[i] == b'\\' && i + 1 < n {
                    i += 2;
                    continue;
                }
                if b[i] == b'"' {
                    i += 1;
                    break;
                }
                if b[i] == b'\n' {
                    out.push('\n');
                }
                i += 1;
            }
            continue;
        }
        if b[i] == b'\'' {
            // Distinguish char literal `'a'` or `'\n'` from lifetime `'a`.
            if i + 2 < n && b[i + 1] == b'\\' {
                let mut j = i + 2;
                while j < n && b[j] != b'\'' && b[j] != b'\n' {
                    j += 1;
                }
                if j < n && b[j] == b'\'' {
                    out.push_str("''");
                    i = j + 1;
                    continue;
                }
            } else if i + 2 < n && b[i + 2] == b'\'' && b[i + 1] != b'\n' {
                out.push_str("''");
                i += 3;
                continue;
            }
        }
        let ch = src[i..].chars().next().expect("valid UTF-8");
        out.push(ch);
        i += ch.len_utf8();
    }
    out
}

fn rel_file_to_mod_path(rel: &str) -> String {
    let trimmed = rel.strip_suffix(".rs").unwrap_or(rel);
    let no_mod = trimmed.strip_suffix("/mod").unwrap_or(trimmed);
    no_mod.replace('/', "::")
}

fn expand_use_tree(prefix: &str, tree: &str, out: &mut Vec<String>) {
    let s = tree.trim();
    if s.is_empty() {
        return;
    }
    if let Some(brace_open) = s.find('{') {
        let head = s[..brace_open].trim().trim_end_matches("::");
        let base = if prefix.is_empty() {
            head.to_owned()
        } else if head.is_empty() {
            prefix.to_owned()
        } else {
            format!("{prefix}::{head}")
        };
        let Some(brace_close) = s.rfind('}') else {
            return;
        };
        let inner = &s[brace_open + 1..brace_close];
        let mut depth = 0usize;
        let mut start = 0usize;
        for (idx, ch) in inner.char_indices() {
            match ch {
                '{' => depth += 1,
                '}' => depth = depth.saturating_sub(1),
                ',' if depth == 0 => {
                    expand_use_tree(&base, &inner[start..idx], out);
                    start = idx + 1;
                }
                _ => {}
            }
        }
        expand_use_tree(&base, &inner[start..], out);
    } else {
        let cleaned = s.split(" as ").next().unwrap_or(s).trim();
        let full = if cleaned == "self" {
            prefix.to_owned()
        } else if prefix.is_empty() {
            cleaned.to_owned()
        } else {
            format!("{prefix}::{cleaned}")
        };
        out.push(full);
    }
}

fn is_allowed_internal_module(path: &str, allowed_mods: &BTreeSet<String>) -> bool {
    let segs: Vec<&str> = path.split("::").filter(|s| !s.is_empty()).collect();
    if segs.is_empty() {
        return false;
    }
    // Check if any prefix of `segs` equals one of the listed boundary modules.
    let mut cur = String::new();
    for (idx, seg) in segs.iter().enumerate() {
        if idx > 0 {
            cur.push_str("::");
        }
        cur.push_str(seg);
        if allowed_mods.contains(&cur) {
            return true;
        }
    }
    false
}

fn check_single_boundary_source(
    rel_file: &str,
    raw_src: &str,
    allowed_mods: &BTreeSet<String>,
) -> Vec<String> {
    let mut violations = Vec::new();
    let code = strip_rust_comments_and_literals(raw_src);
    let current_mod = rel_file_to_mod_path(rel_file);
    let current_parent_segs: Vec<&str> = {
        let mut s: Vec<&str> = current_mod.split("::").collect();
        s.pop();
        s
    };

    // 1. Parse every `extern crate ...;` and `use ...;` statement.
    let mut i = 0usize;
    let bytes = code.as_bytes();
    while i < bytes.len() {
        let boundary = i == 0 || !(bytes[i - 1].is_ascii_alphanumeric() || bytes[i - 1] == b'_');
        if boundary && code[i..].starts_with("extern") {
            let after = &code[i + 6..];
            if after.starts_with(char::is_whitespace) {
                let rest = after.trim_start();
                if let Some(after_crate) = rest.strip_prefix("crate")
                    && after_crate.starts_with(char::is_whitespace)
                    && let Some(semi) = after_crate.find(';')
                {
                    let cname = after_crate[..semi]
                        .split_whitespace()
                        .next()
                        .unwrap_or("");
                    if !matches!(cname, "std" | "core" | "frob_cert") {
                        violations.push(format!(
                            "{rel_file}: forbidden `extern crate {cname}` outside scorer boundary"
                        ));
                    }
                }
            }
        }

        if boundary && code[i..].starts_with("use") {
            let after = &code[i + 3..];
            if after.starts_with(char::is_whitespace)
                && let Some(semi) = after.find(';')
            {
                let tree = after[..semi].trim();
                let mut paths = Vec::new();
                expand_use_tree("", tree, &mut paths);
                for full in paths {
                    let trimmed = full.trim_start_matches("::");
                    let mut segs = trimmed.split("::");
                    let Some(root) = segs.next() else {
                        continue;
                    };
                    match root {
                        "std" | "core" | "frob_cert" | "self" => {}
                        "crate" => {
                            let rest: Vec<&str> = segs.collect();
                            let rest_joined = rest.join("::");
                            if !is_allowed_internal_module(&rest_joined, allowed_mods) {
                                violations.push(format!(
                                    "{rel_file}: forbidden internal import `crate::{rest_joined}` (not in BOUNDARY_FILES)"
                                ));
                            }
                        }
                        "super" => {
                            let mut base = current_parent_segs.clone();
                            let mut rem = Vec::new();
                            let mut climbing = true;
                            for seg in segs {
                                if climbing && seg == "super" {
                                    base.pop();
                                } else {
                                    climbing = false;
                                    rem.push(seg);
                                }
                            }
                            let mut resolved = base;
                            resolved.extend(rem);
                            let joined = resolved.join("::");
                            if !is_allowed_internal_module(&joined, allowed_mods) {
                                violations.push(format!(
                                    "{rel_file}: forbidden `super::` import resolving to `crate::{joined}` (not in BOUNDARY_FILES)"
                                ));
                            }
                        }
                        other => {
                            violations.push(format!(
                                "{rel_file}: forbidden crate root `{other}` in `use {full}` (only std and frob_cert allowed)"
                            ));
                        }
                    }
                }
                i += 3 + semi + 1;
                continue;
            }
        }
        i += 1;
    }

    // 2. Scan inline path expressions (`crate::...`, `super::...`, `f64::<banned>`, foreign crate::).
    let mut pos = 0usize;
    while pos < bytes.len() {
        let is_ident_start = bytes[pos].is_ascii_alphabetic() || bytes[pos] == b'_';
        let prev_is_ident =
            pos > 0 && (bytes[pos - 1].is_ascii_alphanumeric() || bytes[pos - 1] == b'_');
        if is_ident_start && !prev_is_ident {
            let start = pos;
            let mut end = pos;
            loop {
                while end < bytes.len()
                    && (bytes[end].is_ascii_alphanumeric() || bytes[end] == b'_')
                {
                    end += 1;
                }
                if end + 2 <= bytes.len()
                    && bytes[end] == b':'
                    && bytes[end + 1] == b':'
                    && end + 2 < bytes.len()
                    && (bytes[end + 2].is_ascii_alphabetic() || bytes[end + 2] == b'_')
                {
                    end += 2;
                } else {
                    break;
                }
            }
            let expr = &code[start..end];
            if expr.contains("::") {
                let segs: Vec<&str> = expr.split("::").collect();
                match segs[0] {
                    "crate" => {
                        let rest = segs[1..].join("::");
                        if !is_allowed_internal_module(&rest, allowed_mods) {
                            violations.push(format!(
                                "{rel_file}: forbidden path expression `{expr}` (not in BOUNDARY_FILES)"
                            ));
                        }
                    }
                    "super" => {
                        let mut base = current_parent_segs.clone();
                        let mut rem = Vec::new();
                        let mut climbing = true;
                        for &seg in &segs[1..] {
                            if climbing && seg == "super" {
                                base.pop();
                            } else {
                                climbing = false;
                                rem.push(seg);
                            }
                        }
                        let mut resolved = base;
                        resolved.extend(rem);
                        let joined = resolved.join("::");
                        if !is_allowed_internal_module(&joined, allowed_mods) {
                            violations.push(format!(
                                "{rel_file}: forbidden `super::` path expression `{expr}` -> `crate::{joined}`"
                            ));
                        }
                    }
                    "f64" | "f32" => {
                        if let Some(&method) = segs.get(1)
                            && BANNED_MATH_METHODS.contains(&method)
                        {
                            violations.push(format!(
                                "{rel_file}: forbidden platform math call `{expr}`"
                            ));
                        }
                    }
                    "std" | "core" | "frob_cert" | "self" | "fmt" | "fs" | "io" | "blake3"
                    | "mission" | "freeze" | "ledger" | "receipt" | "report" | "ratio"
                    | "verdict" | "clippy" => {}
                    first if first.chars().next().is_some_and(char::is_uppercase) => {
                        // Type or enum path (`JsonValue::Obj`, `Ordering::Less`, etc.)
                    }
                    first => {
                        // Lowercase external crate or unallowed module root (e.g. `rand::thread_rng`).
                        if !matches!(first, "u8" | "u16" | "u32" | "u64" | "u128" | "usize" | "i8" | "i16" | "i32" | "i64" | "i128" | "isize" | "char" | "str" | "bool") {
                            violations.push(format!(
                                "{rel_file}: forbidden path root `{first}` in `{expr}`"
                            ));
                        }
                    }
                }
            }
            pos = end;
            continue;
        }
        pos += 1;
    }

    // 3. Scan banned transcendental method calls `.ln(`, `.exp(`, `.powf(`, etc.
    for (idx, b) in bytes.iter().enumerate() {
        if *b == b'.' {
            let mut j = idx + 1;
            while j < bytes.len() && bytes[j].is_ascii_whitespace() {
                j += 1;
            }
            let m_start = j;
            while j < bytes.len() && (bytes[j].is_ascii_alphanumeric() || bytes[j] == b'_') {
                j += 1;
            }
            let method = &code[m_start..j];
            let mut k = j;
            while k < bytes.len() && bytes[k].is_ascii_whitespace() {
                k += 1;
            }
            if k < bytes.len() && bytes[k] == b'(' && BANNED_MATH_METHODS.contains(&method) {
                violations.push(format!(
                    "{rel_file}: forbidden platform math method `.{method}()`"
                ));
            }
        }
    }

    violations
}

fn extract_kit_path_modules(kit_lib_rs: &str) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    let needle = "#[path = \"";
    let mut rest = kit_lib_rs;
    while let Some(pos) = rest.find(needle) {
        rest = &rest[pos + needle.len()..];
        let Some(end) = rest.find('"') else {
            break;
        };
        let raw_path = &rest[..end];
        rest = &rest[end + 1..];
        if raw_path == "." {
            continue;
        }
        if let Some(after) = raw_path.strip_prefix("../../../crates/frob-conformance/src/") {
            out.insert(after.to_owned());
        } else {
            out.insert(raw_path.to_owned());
        }
    }
    out
}

fn lint_scorer_boundary(
    src_root: &Path,
    boundary_files: &[&str],
    kit_lib_rs: &str,
    kit_cargo_toml: &str,
) -> Vec<String> {
    let mut violations = Vec::new();
    let allowed_mods: BTreeSet<String> = boundary_files
        .iter()
        .map(|rel| rel_file_to_mod_path(rel))
        .collect();

    // Check every listed file exists and obeys imports + math rules.
    for rel in boundary_files {
        let abs = src_root.join(rel);
        match fs::read_to_string(&abs) {
            Ok(src) => violations.extend(check_single_boundary_source(rel, &src, &allowed_mods)),
            Err(e) => violations.push(format!("{rel}: missing boundary file ({e})")),
        }
    }

    // Check 1-to-1 match with #[path] modules in kit/scorer/src/lib.rs.
    let listed_set: BTreeSet<String> = boundary_files.iter().map(|s| (*s).to_owned()).collect();
    let kit_set = extract_kit_path_modules(kit_lib_rs);
    for missing in listed_set.difference(&kit_set) {
        violations.push(format!(
            "kit/scorer/src/lib.rs: missing #[path] module for listed BOUNDARY_FILES entry `{missing}`"
        ));
    }
    for extra in kit_set.difference(&listed_set) {
        violations.push(format!(
            "kit/scorer/src/lib.rs: unlisted #[path] module `{extra}` not in BOUNDARY_FILES"
        ));
    }

    // Check kit/scorer/Cargo.toml has exactly one dependency: path to frob-cert.
    let mut in_deps = false;
    let mut deps = Vec::new();
    for raw in kit_cargo_toml.lines() {
        let line = raw.split('#').next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }
        if line.starts_with('[') && line.ends_with(']') {
            in_deps = matches!(
                line,
                "[dependencies]" | "[dev-dependencies]" | "[build-dependencies]"
            );
            continue;
        }
        if in_deps && let Some((k, v)) = line.split_once('=') {
            deps.push((k.trim().to_owned(), v.trim().to_owned()));
        }
    }
    if deps.len() != 1 || deps[0].0 != "frob-cert" || !deps[0].1.contains("path") {
        violations.push(format!(
            "kit/scorer/Cargo.toml: must declare exactly one path dependency on `frob-cert`, found: {deps:?}"
        ));
    }

    violations
}

mod records {
    use super::*;
    use frob_cert::json::{JsonValue, serialize_canonical};
    use frob_cert::rational::Rational;
    use frob_conformance::tenfold::canon::{
        FIVE_SCHEMAS, RecordHeader, RoundingMode, SCHEMA_FREEZE, SCHEMA_LEDGER, SCHEMA_MISSION,
        SCHEMA_RECEIPT, SCHEMA_REPORT, SchemaKind, canonical_bytes, canonicalize_json_bytes,
        check_record_bytes, content_address, render_rational_decimal,
    };
    use frob_conformance::tenfold::ids::{
        Arm, CampaignIndex, FamilyId, GateId, Hash32, Millis, MissionHash, Phase, Regime,
        StratumId,
    };
    use frob_conformance::tenfold::records::{freeze, ledger, mission, receipt, report};
    use frob_conformance::tenfold::refusal::RefusalReason;
    use frob_conformance::tenfold::score::ratio::{exact_ratio, render_ratio};
    use frob_conformance::tenfold::store::{GateStore, WriteOutcome};

    const HEX_A: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
    const HEX_B: &str = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";

    fn sample_family() -> FamilyId {
        FamilyId::parse(&format!("f:{HEX_A}"), "/family_id").expect("valid family id")
    }

    fn sample_mission_hash() -> MissionHash {
        MissionHash::parse(&format!("b3:{HEX_B}"), "/mission_hash").expect("valid mission hash")
    }

    fn sample_mission_record() -> mission::MissionRecord {
        mission::MissionRecord {
            header: RecordHeader {
                schema: SchemaKind::Mission,
                gate: GateId::Rehearsal("m0-smoke".to_owned()),
            },
            family_id: sample_family(),
            stratum: StratumId::LeggedQuad,
            deadline: Millis(1_500_000),
            mission_id: Some("m-001".to_owned()),
            regime: Some(Regime::Cold),
        }
    }

    fn sample_freeze_record() -> freeze::FreezeRecord {
        freeze::FreezeRecord::build_stamped(
            RecordHeader {
                schema: SchemaKind::Freeze,
                gate: GateId::M0Baseline,
            },
            Arm::A,
            StratumId::LeggedQuad,
            Some(Regime::Cold),
            Some(Millis(14_400_000)),
        )
        .expect("build stamped freeze record")
    }

    fn sample_ledger_record() -> ledger::LedgerRecord {
        // Known answer from N39a-01 / N39a-23:
        // [600, 360, 20, 0, 12, 96, 150, 60, 40, 0, 30, 42, 90, 0] s = 1500 s exact.
        let secs: [u64; 14] = [600, 360, 20, 0, 12, 96, 150, 60, 40, 0, 30, 42, 90, 0];
        let mut phases = BTreeMap::new();
        for (p, s) in Phase::ALL.into_iter().zip(secs) {
            phases.insert(p, Millis(s * 1_000));
        }
        ledger::LedgerRecord {
            header: RecordHeader {
                schema: SchemaKind::Ledger,
                gate: GateId::M0Baseline,
            },
            arm: Arm::A,
            stratum: StratumId::LeggedQuad,
            family_id: sample_family(),
            campaign: CampaignIndex(1),
            regime: Regime::Cold,
            clock: Millis(1_500_000),
            phases,
        }
    }

    fn sample_receipt_record() -> receipt::ReceiptRecord {
        receipt::ReceiptRecord::build_stamped(
            RecordHeader {
                schema: SchemaKind::Receipt,
                gate: GateId::M0Baseline,
            },
            Arm::A,
            StratumId::LeggedQuad,
            sample_family(),
            CampaignIndex(1),
            sample_mission_hash(),
            Millis(1_500),
            Some(0),
        )
        .expect("build stamped receipt record")
    }

    fn sample_report_record() -> report::ReportRecord {
        let ratio = exact_ratio(54_000, 20_400).expect("54000/20400 -> 45/17");
        let lo = exact_ratio(15_255, 130_000).expect("0.11734615...");
        let hi = exact_ratio(15_255, 130_000).expect("0.11734615...");
        report::ReportRecord::build_stamped(
            RecordHeader {
                schema: SchemaKind::Report,
                gate: GateId::M0Baseline,
            },
            Regime::Cold,
            ratio,
            3,
            Some((lo, hi, 4)),
        )
        .expect("build stamped report record")
    }

    #[test]
    fn schema_names_are_five() {
        assert_eq!(FIVE_SCHEMAS.len(), 5);
        assert_eq!(SchemaKind::ALL.len(), 5);
        let expected = [
            SCHEMA_MISSION,
            SCHEMA_FREEZE,
            SCHEMA_LEDGER,
            SCHEMA_RECEIPT,
            SCHEMA_REPORT,
        ];
        assert_eq!(FIVE_SCHEMAS, expected);
        for name in expected {
            let parsed = SchemaKind::parse(name).expect("known schema must parse");
            assert_eq!(parsed.as_str(), name);
        }
        let err = SchemaKind::parse("frob.tenfold.other/1")
            .expect_err("unknown schema must be refused");
        assert_eq!(err.reason, RefusalReason::RecordUnknownSchema);
        assert_eq!(err.exit_code(), 3);
        assert_eq!(err.pointer, "/schema");
    }

    #[test]
    fn seconds_exact() {
        assert_eq!(
            Millis::parse_canonical_seconds("1500", "/d").unwrap(),
            Millis(1_500_000)
        );
        assert_eq!(
            Millis::parse_canonical_seconds("0.001", "/d").unwrap(),
            Millis(1)
        );
        assert_eq!(
            Millis::parse_canonical_seconds("1.5", "/d").unwrap(),
            Millis(1_500)
        );
        assert_eq!(
            Millis::parse_canonical_seconds("14400", "/d").unwrap(),
            Millis(14_400_000)
        );
        assert_eq!(
            Millis::parse_canonical_seconds("0", "/d").unwrap(),
            Millis(0)
        );

        for bad in ["1.0005", "1.50", "1.500", "1.", "01", "-1", "1e3", "", ".5"] {
            let err = Millis::parse_canonical_seconds(bad, "/deadline_s")
                .expect_err(&format!("non-canonical duration `{bad}` must be refused"));
            assert_eq!(err.reason, RefusalReason::RecordNoncanonicalNumber);
            assert_eq!(err.exit_code(), 3);
            assert_eq!(err.pointer, "/deadline_s");
        }

        // JSON number `1500` offered for a duration field must be refused with pointer.
        let mut num_mission = sample_mission_record().to_json_value();
        if let JsonValue::Obj(ref mut entries) = num_mission {
            for (k, v) in entries.iter_mut() {
                if k == "deadline_s" {
                    *v = JsonValue::Int(1500);
                }
            }
        }
        let err = check_record_bytes(serialize_canonical(&num_mission).as_bytes())
            .expect_err("JSON number duration must be refused");
        assert_eq!(err.reason, RefusalReason::RecordNoncanonicalNumber);
        assert_eq!(err.exit_code(), 3);
        assert_eq!(err.pointer, "/deadline_s");
    }

    #[test]
    fn writer_emits_canonical_seconds() {
        for ms in 0_u64..=100_000 {
            let m = Millis(ms);
            let s = m.to_canonical_seconds();
            let back = Millis::parse_canonical_seconds(&s, "/t")
                .unwrap_or_else(|e| panic!("writer emitted non-canonical `{s}` for {ms} ms: {e}"));
            assert_eq!(back, m);
        }
        let m14400 = Millis(14_400_000);
        assert_eq!(m14400.to_canonical_seconds(), "14400");
        assert_eq!(
            Millis::parse_canonical_seconds("14400", "/t").unwrap(),
            m14400
        );
    }

    #[test]
    fn canon_idempotent() {
        // 1,000 deterministically generated records: canon(canon(x)) == canon(x)
        // and key order of the input does not change the content address.
        let strata = StratumId::ALL;
        let arms = Arm::ALL;
        for idx in 0_u64..1_000 {
            let stratum = strata[(idx as usize) % strata.len()];
            let arm = arms[(idx as usize) % arms.len()];
            let deadline_ms = (idx * 37) % 100_001;
            let rec = mission::MissionRecord {
                header: RecordHeader {
                    schema: SchemaKind::Mission,
                    gate: GateId::Rehearsal(format!("r{idx}")),
                },
                family_id: sample_family(),
                stratum,
                deadline: Millis(deadline_ms),
                mission_id: Some(format!("mission-{arm}-{idx}")),
                regime: Some(if idx % 2 == 0 {
                    Regime::Cold
                } else {
                    Regime::Warm
                }),
            };
            let val = rec.to_json_value();
            let c1 = canonical_bytes(&val);
            let c2 = canonicalize_json_bytes(&c1).expect("canon(canon(x))");
            assert_eq!(c1, c2, "canon must be idempotent at record {idx}");

            // Reverse top-level key order and add pretty whitespace.
            let JsonValue::Obj(mut entries) = val else {
                panic!("mission must serialize to object");
            };
            entries.reverse();
            let mut permuted = String::from("{\n");
            for (pos, (k, v)) in entries.iter().enumerate() {
                if pos > 0 {
                    permuted.push_str(",\n");
                }
                permuted.push_str("  \"");
                permuted.push_str(k);
                permuted.push_str("\" : ");
                permuted.push_str(&serialize_canonical(v));
            }
            permuted.push_str("\n}\n");

            let checked_orig = check_record_bytes(&c1).expect("check canonical");
            let checked_perm = check_record_bytes(permuted.as_bytes()).expect("check permuted");
            assert_eq!(checked_orig.canonical_bytes, checked_perm.canonical_bytes);
            assert_eq!(checked_orig.address, checked_perm.address);
        }
    }

    #[test]
    fn self_hash_rule() {
        let rec = sample_freeze_record();
        let val = rec.to_json_value();
        let recomputed = content_address(&val, SchemaKind::Freeze).expect("blanked content address");
        assert_eq!(recomputed, rec.manifest_hash);

        let checked = check_record_bytes(&rec.to_canonical_bytes()).expect("valid freeze record");
        assert_eq!(checked.address, rec.manifest_hash);

        // Changing any other field changes the computed hash and fails check with exit 9.
        let mut tampered = val.clone();
        if let JsonValue::Obj(ref mut entries) = tampered {
            for (k, v) in entries.iter_mut() {
                if k == "arm" {
                    *v = JsonValue::Str("B".to_owned());
                }
            }
        }
        let tampered_recomputed =
            content_address(&tampered, SchemaKind::Freeze).expect("recompute tampered");
        assert_ne!(tampered_recomputed, rec.manifest_hash);
        let err = check_record_bytes(serialize_canonical(&tampered).as_bytes())
            .expect_err("tampered freeze record must fail self-hash");
        assert_eq!(err.reason, RefusalReason::RecordHashMismatch);
        assert_eq!(err.exit_code(), 9);
        assert_eq!(err.pointer, "/manifest_hash");

        // Negative control: stored hash computed WITHOUT blanking must fail with exit 9.
        let mut unblanked = val;
        if let JsonValue::Obj(ref mut entries) = unblanked {
            for (k, v) in entries.iter_mut() {
                if k == "manifest_hash" {
                    *v = JsonValue::Str(format!("b3:{HEX_A}"));
                }
            }
        }
        let wrong_hash = Hash32::from_bytes(frob_cert::blake3::hash(&canonical_bytes(&unblanked)));
        if let JsonValue::Obj(ref mut entries) = unblanked {
            for (k, v) in entries.iter_mut() {
                if k == "manifest_hash" {
                    *v = JsonValue::Str(wrong_hash.to_b3_string());
                }
            }
        }
        let err2 = check_record_bytes(serialize_canonical(&unblanked).as_bytes())
            .expect_err("hash computed without blanking must fail");
        assert_eq!(err2.reason, RefusalReason::RecordHashMismatch);
        assert_eq!(err2.exit_code(), 9);
        assert_eq!(err2.pointer, "/manifest_hash");
    }

    #[test]
    fn unknown_field_refused() {
        let mut val = sample_mission_record().to_json_value();
        if let JsonValue::Obj(ref mut entries) = val {
            entries.push(("mystery_field".to_owned(), JsonValue::Str("nope".to_owned())));
        }
        let err = check_record_bytes(serialize_canonical(&val).as_bytes())
            .expect_err("unknown field must be refused");
        assert_eq!(err.reason, RefusalReason::RecordUnknownField);
        assert_eq!(err.exit_code(), 3);
        assert_eq!(err.pointer, "/mystery_field");
    }

    #[test]
    fn duplicate_key_refused() {
        let raw = format!(
            "{{\"deadline_s\":\"1500\",\"deadline_s\":\"1500\",\"execution_authorized\":false,\"family_id\":\"f:{HEX_A}\",\"gate\":\"M0-baseline\",\"hardware_safety_signoff\":false,\"reality\":\"no-claim\",\"schema\":\"frob.tenfold.mission/1\",\"sim_to_real_certified\":false,\"stratum\":\"legged-quad\"}}"
        );
        let err = check_record_bytes(raw.as_bytes()).expect_err("duplicate key must be refused");
        assert_eq!(err.reason, RefusalReason::RecordDuplicateKey);
        assert_eq!(err.exit_code(), 3);
        assert!(
            err.pointer.contains("deadline_s") || err.observed.contains("deadline_s"),
            "refusal must identify duplicate key: {err}"
        );
    }

    #[test]
    fn missing_field_refused() {
        let mut val = sample_mission_record().to_json_value();
        if let JsonValue::Obj(ref mut entries) = val {
            entries.retain(|(k, _)| k != "deadline_s");
        }
        let err = check_record_bytes(serialize_canonical(&val).as_bytes())
            .expect_err("missing field must be refused");
        assert_eq!(err.reason, RefusalReason::RecordMissingField);
        assert_eq!(err.exit_code(), 3);
        assert_eq!(err.pointer, "/deadline_s");
    }

    #[test]
    fn render_rules() {
        let reduced = exact_ratio(54_000, 20_400).expect("54000/20400");
        assert_eq!(reduced.to_slash_string(), "45/17");

        let (exact_s, dec_s) =
            render_ratio(&reduced, 3, RoundingMode::HalfToEven).expect("render 45/17");
        assert_eq!(exact_s, "45/17");
        assert_eq!(dec_s, "2.647");

        // 15255 / 130000 = 0.11734615384615...
        let bound = Rational::from_i64_pair(15_255, 130_000).expect("exact bound rational");
        let upper = render_rational_decimal(&bound, 4, RoundingMode::OutwardUpper)
            .expect("outward upper");
        let lower = render_rational_decimal(&bound, 4, RoundingMode::OutwardLower)
            .expect("outward lower");
        assert_eq!(upper, "0.1174");
        assert_eq!(lower, "0.1173");

        // Exact half-to-even tie-breaking checks.
        let half = Rational::from_i64_pair(1, 2).unwrap();
        let three_halves = Rational::from_i64_pair(3, 2).unwrap();
        assert_eq!(
            render_rational_decimal(&half, 0, RoundingMode::HalfToEven).unwrap(),
            "0"
        );
        assert_eq!(
            render_rational_decimal(&three_halves, 0, RoundingMode::HalfToEven).unwrap(),
            "2"
        );
    }

    #[test]
    fn scorer_boundary_lint() {
        let root = repo_root();
        let src_root = root.join("crates/frob-conformance/src");
        let kit_lib = fs::read_to_string(root.join("kit/scorer/src/lib.rs"))
            .expect("kit/scorer/src/lib.rs must exist");
        let kit_cargo = fs::read_to_string(root.join("kit/scorer/Cargo.toml"))
            .expect("kit/scorer/Cargo.toml must exist");

        // 1. Real repository tree must pass with zero violations.
        let clean = lint_scorer_boundary(&src_root, BOUNDARY_FILES, &kit_lib, &kit_cargo);
        assert!(
            clean.is_empty(),
            "real tree failed scorer boundary lint:\n{}",
            clean.join("\n")
        );

        let allowed_mods: BTreeSet<String> = BOUNDARY_FILES
            .iter()
            .map(|r| rel_file_to_mod_path(r))
            .collect();

        // 2. Planted `use rand` under score/ must fail.
        let v_rand = check_single_boundary_source(
            "tenfold/score/rng.rs",
            "use rand::Rng;\npub fn x() {}\n",
            &allowed_mods,
        );
        assert!(
            v_rand.iter().any(|m| m.contains("rand")),
            "planted `use rand` must fail lint: {v_rand:?}"
        );

        // 3. Planted `x.ln()` under score/ must fail.
        let v_ln = check_single_boundary_source(
            "tenfold/score/normal.rs",
            "pub fn bad(x: f64) -> f64 { x.ln() }\n",
            &allowed_mods,
        );
        assert!(
            v_ln.iter().any(|m| m.contains(".ln()")),
            "planted `x.ln()` must fail lint: {v_ln:?}"
        );

        // 4. Planted call from score/ to crate::tenfold::store (not in BOUNDARY_FILES) must fail naming path.
        let v_store = check_single_boundary_source(
            "tenfold/score/verdict.rs",
            "use crate::tenfold::store::write_once;\npub fn bad() { let _ = crate::tenfold::store::write_once; }\n",
            &allowed_mods,
        );
        assert!(
            v_store.iter().any(|m| m.contains("crate::tenfold::store")),
            "call to unlisted crate::tenfold::store must fail naming path: {v_store:?}"
        );

        // 5. Scratch file entered in BOUNDARY_FILES with `use rand` AND missing from kit/scorer/src/lib.rs fails both.
        let scratch = fresh_scratch("boundary_lint");
        let scratch_src = scratch.join("src");
        for rel in BOUNDARY_FILES {
            let dst = scratch_src.join(rel);
            fs::create_dir_all(dst.parent().unwrap()).expect("mkdir");
            fs::copy(src_root.join(rel), &dst).expect("copy boundary file");
        }
        let planted_rel = "tenfold/score/planted_bad.rs";
        fs::write(
            scratch_src.join(planted_rel),
            "use rand::thread_rng;\npub fn z() {}\n",
        )
        .expect("write planted scratch file");
        let mut extended = BOUNDARY_FILES.to_vec();
        extended.push(planted_rel);

        let v_ext = lint_scorer_boundary(&scratch_src, &extended, &kit_lib, &kit_cargo);
        assert!(
            v_ext.iter().any(|m| m.contains("rand") && m.contains(planted_rel)),
            "scratch file with `use rand` must fail lint: {v_ext:?}"
        );
        assert!(
            v_ext.iter().any(|m| m.contains("missing #[path]") && m.contains(planted_rel)),
            "listed file with no #[path] module in kit/scorer/ must fail lint: {v_ext:?}"
        );
    }

    #[test]
    fn append_only_store() {
        let dir = fresh_scratch("store");
        let store = GateStore::new(&dir, GateId::Rehearsal("store-test".to_owned()));
        store.init_dirs().expect("init store dirs");

        let report_path = store.report_path();
        let bytes_a = sample_report_record().to_canonical_bytes();

        // First write creates the file.
        let w1 = store
            .write_once(&report_path, &bytes_a)
            .expect("first write must succeed");
        assert_eq!(w1, WriteOutcome::Created);

        // Second write of identical bytes is a no-op.
        let w2 = store
            .write_once(&report_path, &bytes_a)
            .expect("identical bytes must be a no-op");
        assert_eq!(w2, WriteOutcome::IdenticalNoOp);

        // Second write of different bytes is refused with store-overwrite (exit 9).
        let bytes_b = b"{\"different\":true}";
        let err = store
            .write_once(&report_path, bytes_b)
            .expect_err("different bytes must be refused");
        assert_eq!(err.reason, RefusalReason::StoreOverwrite);
        assert_eq!(err.exit_code(), 9);
        assert_eq!(fs::read(&report_path).unwrap(), bytes_a);
    }

    #[test]
    fn known_answer_phase_ledger_sums_to_1500() {
        let rec = sample_ledger_record();
        assert_eq!(rec.phase_sum().unwrap(), Millis(1_500_000));
        assert_eq!(rec.clock, Millis(1_500_000));

        let bytes = rec.to_canonical_bytes();
        let checked = check_record_bytes(&bytes).expect("valid 1500s ledger must check");
        assert_eq!(checked.schema, SchemaKind::Ledger);

        // Mutating clock_s so phases no longer sum to clock_s must fail.
        let mut bad = rec.to_json_value();
        if let JsonValue::Obj(ref mut entries) = bad {
            for (k, v) in entries.iter_mut() {
                if k == "clock_s" {
                    *v = JsonValue::Str("1501".to_owned());
                }
            }
        }
        let err = check_record_bytes(serialize_canonical(&bad).as_bytes())
            .expect_err("mismatched phase sum must fail");
        assert_eq!(err.reason, RefusalReason::RecordNoncanonicalNumber);
        assert_eq!(err.pointer, "/clock_s");
    }

    #[test]
    fn all_five_schemas_and_auth_flags() {
        let fixtures: [(SchemaKind, Vec<u8>); 5] = [
            (SchemaKind::Mission, sample_mission_record().to_canonical_bytes()),
            (SchemaKind::Freeze, sample_freeze_record().to_canonical_bytes()),
            (SchemaKind::Ledger, sample_ledger_record().to_canonical_bytes()),
            (SchemaKind::Receipt, sample_receipt_record().to_canonical_bytes()),
            (SchemaKind::Report, sample_report_record().to_canonical_bytes()),
        ];
        for (kind, bytes) in fixtures {
            let checked = check_record_bytes(&bytes)
                .unwrap_or_else(|e| panic!("valid {} fixture failed: {e}", kind.as_str()));
            assert_eq!(checked.schema, kind);
            assert_eq!(checked.canonical_bytes, bytes);
        }

        // Setting execution_authorized = true must be refused.
        let mut bad = sample_mission_record().to_json_value();
        if let JsonValue::Obj(ref mut entries) = bad {
            for (k, v) in entries.iter_mut() {
                if k == "execution_authorized" {
                    *v = JsonValue::Bool(true);
                }
            }
        }
        let err = check_record_bytes(serialize_canonical(&bad).as_bytes())
            .expect_err("execution_authorized = true must be refused");
        assert_eq!(err.exit_code(), 3);
        assert_eq!(err.pointer, "/execution_authorized");
    }
}

/// Phase vocabulary, raw interval record, turn-phase rule, and exclusive
/// critical-path attribution tests (bead `N39a-23`, §39.6).
///
/// Run as: `cargo test -p frob-conformance --test tenfold -- phase::exclusive::`
mod phase {
    pub mod exclusive {
        use frob_conformance::tenfold::ids::{Millis, Phase};
        use frob_conformance::tenfold::phase::{
            AttributionWindow, FLAG_CENSORED_REMAINDER, Interval, IntervalKind,
            PRODUCT_PHASE_MAP, ProductVocabulary, ResourceInfo, attribute_exclusive,
            diagnose_instant, map_product_phase, parse_intervals_ndjson, product_names_for_phase,
            turn_phase, write_intervals_ndjson,
        };
        use frob_conformance::tenfold::refusal::RefusalReason;

        /// Deterministic SplitMix64 PRNG (class D0, zero external crates).
        struct SplitMix64(u64);

        impl SplitMix64 {
            fn next_u64(&mut self) -> u64 {
                self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
                let mut z = self.0;
                z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
                z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
                z ^ (z >> 31)
            }

            fn range(&mut self, bound: u64) -> u64 {
                if bound == 0 {
                    0
                } else {
                    self.next_u64() % bound
                }
            }

            fn shuffle<T>(&mut self, slice: &mut [T]) {
                let n = slice.len();
                for i in (1..n).rev() {
                    let j = (self.next_u64() as usize) % (i + 1);
                    slice.swap(i, j);
                }
            }
        }

        fn worked_fixture_intervals() -> Vec<Interval> {
            // N39a-23 §5: t0 = 0, t_stop = 100 s (100_000 ms).
            // Turn 1 [0, 10s) propose;
            // Block [10s, 60s) awaiting job 1 (compile [10s, 22s), screen [22s, 40s)) and job 2 (train [12s, 60s));
            // Turn 2 [60s, 70s) propose;
            // Block [70s, 90s) awaiting adjudication [70s, 90s);
            // Nothing in [90s, 100s).
            vec![
                Interval::turn(1, Phase::Propose, 0, 10_000),
                Interval::block(2, 10_000, 60_000, vec![3, 4, 5]),
                Interval::work(3, Phase::Compile, 10_000, 22_000, Some("job-1".to_owned())),
                Interval::work(4, Phase::Screen, 22_000, 40_000, Some("job-1".to_owned())),
                Interval::work(5, Phase::Train, 12_000, 60_000, Some("job-2".to_owned())),
                Interval::turn(6, Phase::Propose, 60_000, 70_000),
                Interval::block(7, 70_000, 90_000, vec![8]),
                Interval::adjudication(8, 70_000, 90_000),
            ]
        }

        #[test]
        fn worked_fixture() {
            let ivs = worked_fixture_intervals();
            let table = attribute_exclusive(&ivs, AttributionWindow::new(0, 100_000))
                .expect("worked fixture must attribute cleanly");

            assert_eq!(table.clock_ms, 100_000);
            assert_eq!(table.sum_ms(), 100_000);
            assert_eq!(table.ms(Phase::Propose), 20_000);
            assert_eq!(table.ms(Phase::Wait), 2_000);
            assert_eq!(table.ms(Phase::Train), 48_000);
            assert_eq!(table.ms(Phase::Adjudicate), 20_000);
            assert_eq!(table.ms(Phase::Unattributed), 10_000);
            for p in [
                Phase::Author,
                Phase::Check,
                Phase::Synthesize,
                Phase::Compile,
                Phase::Screen,
                Phase::Evaluate,
                Phase::Qualify,
                Phase::Certify,
                Phase::Diagnose,
            ] {
                assert_eq!(table.ms(p), 0, "phase {p} must be 0 in worked fixture");
            }

            // Round-trip through NDJSON parser and verify identical canonical table JSON.
            let ndjson = write_intervals_ndjson(&ivs);
            let reparsed = parse_intervals_ndjson(&ndjson).expect("roundtrip ndjson");
            let table2 = attribute_exclusive(&reparsed, AttributionWindow::new(0, 100_000))
                .expect("re-attribute");
            assert_eq!(table.to_canonical_bytes(), table2.to_canonical_bytes());

            // Verify failure diagnosis on [10s, 12s) (owner job-2 hasn't started -> wait).
            let diag = diagnose_instant(&ivs, AttributionWindow::new(0, 100_000), 11_000)
                .expect("diagnose 11s");
            assert_eq!(diag.attributed_phase, Phase::Wait);
            assert_eq!(diag.rule, "rule-2b:block-owner");
            assert!(diag.covering_interval_ids.contains(&2));
            assert!(diag.covering_interval_ids.contains(&3));
        }

        #[test]
        fn sums_to_clock_property() {
            let mut rng = SplitMix64(0x39A2_3000_0000_0001);
            let phases13 = Phase::PHASES;

            for trial in 0..10_000_u64 {
                let t0 = rng.range(200);
                let span = rng.range(2_000);
                let t_stop = t0 + span;
                let count = rng.range(18) as usize;

                let mut ivs: Vec<Interval> = Vec::with_capacity(count);
                let mut awaitable_ids: Vec<u64> = Vec::new();
                let mut next_id = 1_u64 + trial * 32;

                // First emit work/harness/adjudication so blocks can await real ids (plus occasional missing/multi-phase).
                let work_count = count / 2;
                for k in 0..work_count {
                    let a = rng.range(t_stop + 250);
                    let b = a + rng.range(400);
                    let id = next_id;
                    next_id += 1;
                    let iv = match rng.range(5) {
                        0 => Interval::harness(id, a, b),
                        1 => Interval::adjudication(id, a, b),
                        _ => {
                            let phase = phases13[rng.range(13) as usize];
                            let proc_opt = if k > 0 && rng.range(3) == 0 {
                                Some(format!("p{}", rng.range(3)))
                            } else {
                                None
                            };
                            Interval::work(id, phase, a, b, proc_opt)
                        }
                    };
                    awaitable_ids.push(id);
                    ivs.push(iv);
                }

                for _ in work_count..count {
                    let a = rng.range(t_stop + 250);
                    let b = a + rng.range(400);
                    let id = next_id;
                    next_id += 1;
                    if rng.range(2) == 0 {
                        let phase = phases13[rng.range(13) as usize];
                        ivs.push(Interval::turn(id, phase, a, b));
                    } else {
                        let mut awaits = Vec::new();
                        if !awaitable_ids.is_empty() {
                            let k_aw = rng.range(4) as usize;
                            for _ in 0..k_aw {
                                let pick = awaitable_ids[rng.range(awaitable_ids.len() as u64) as usize];
                                awaits.push(pick);
                            }
                        }
                        ivs.push(Interval::block(id, a, b, awaits));
                    }
                }

                let table = attribute_exclusive(&ivs, AttributionWindow::new(t0, t_stop))
                    .expect("random interval set must attribute");
                assert_eq!(
                    table.sum_ms(),
                    t_stop - t0,
                    "trial {trial}: sum(phases) must equal T={}",
                    t_stop - t0
                );
                assert_eq!(table.clock_ms, t_stop - t0);
            }
        }

        #[test]
        fn turn_beats_block() {
            let ivs = vec![
                Interval::block(1, 0, 50_000, vec![2]),
                Interval::work(2, Phase::Train, 0, 50_000, Some("j-train".to_owned())),
                Interval::turn(3, Phase::Diagnose, 15_000, 35_000),
            ];
            let table = attribute_exclusive(&ivs, AttributionWindow::new(0, 50_000))
                .expect("turn_beats_block");
            assert_eq!(table.ms(Phase::Diagnose), 20_000);
            assert_eq!(table.ms(Phase::Train), 30_000);
            assert_eq!(table.sum_ms(), 50_000);
        }

        #[test]
        fn last_finisher_owns() {
            // Awaited works ending at 40 and 60 -> the second owns [start, 60).
            let ivs = vec![
                Interval::block(1, 10_000, 60_000, vec![2, 3]),
                Interval::work(2, Phase::Compile, 10_000, 40_000, Some("j1".to_owned())),
                Interval::work(3, Phase::Train, 10_000, 60_000, Some("j2".to_owned())),
            ];
            let table = attribute_exclusive(&ivs, AttributionWindow::new(10_000, 60_000))
                .expect("last_finisher_owns");
            assert_eq!(table.ms(Phase::Train), 50_000);
            assert_eq!(table.ms(Phase::Compile), 0);
            assert_eq!(table.sum_ms(), 50_000);
        }

        #[test]
        fn tie_break_lower_id() {
            // Equal ends (60_000) -> lower id owns the block regardless of input order.
            let ivs_a = vec![
                Interval::block(1, 0, 60_000, vec![20, 10]),
                Interval::work(20, Phase::Train, 0, 60_000, Some("j20".to_owned())),
                Interval::work(10, Phase::Evaluate, 0, 60_000, Some("j10".to_owned())),
            ];
            let ivs_b = vec![
                Interval::work(10, Phase::Evaluate, 0, 60_000, Some("j10".to_owned())),
                Interval::work(20, Phase::Train, 0, 60_000, Some("j20".to_owned())),
                Interval::block(1, 0, 60_000, vec![10, 20]),
            ];
            let ta = attribute_exclusive(&ivs_a, AttributionWindow::new(0, 60_000)).unwrap();
            let tb = attribute_exclusive(&ivs_b, AttributionWindow::new(0, 60_000)).unwrap();
            assert_eq!(ta.ms(Phase::Evaluate), 60_000);
            assert_eq!(ta.ms(Phase::Train), 0);
            assert_eq!(ta, tb);
        }

        #[test]
        fn adjudication_owner() {
            // Block [20, 80) awaiting a training job [20, 50) and an adjudication [30, 80) ->
            // adjudication ends last and owns the block: [20, 30) is wait, [30, 80) is adjudicate;
            // training job's 30 s appear only in the resource table.
            let ivs = vec![
                Interval::block(1, 20_000, 80_000, vec![2, 3]),
                Interval::work(2, Phase::Train, 20_000, 50_000, Some("train-1".to_owned())),
                Interval::adjudication(3, 30_000, 80_000),
            ];
            let table = attribute_exclusive(&ivs, AttributionWindow::new(20_000, 80_000))
                .expect("adjudication_owner");
            assert_eq!(table.clock_ms, 60_000);
            assert_eq!(table.sum_ms(), 60_000);
            assert_eq!(table.ms(Phase::Wait), 10_000);
            assert_eq!(table.ms(Phase::Adjudicate), 50_000);
            assert_eq!(table.ms(Phase::Train), 0);
        }

        #[test]
        fn turn_phase_rule() {
            assert_eq!(turn_phase(&[Phase::Author, Phase::Author]), Phase::Author);
            assert_eq!(turn_phase(&[Phase::Author, Phase::Screen]), Phase::Propose);
            assert_eq!(turn_phase(&[]), Phase::Propose);
            assert_eq!(turn_phase(&[Phase::Diagnose]), Phase::Diagnose);
            assert_eq!(
                turn_phase(&[Phase::Diagnose, Phase::Diagnose]),
                Phase::Diagnose
            );
            assert_eq!(
                turn_phase(&[Phase::Author, Phase::Diagnose]),
                Phase::Propose
            );
            assert_eq!(turn_phase(&[Phase::Train]), Phase::Propose);
        }

        #[test]
        fn order_invariance() {
            let mut ivs = worked_fixture_intervals();
            let baseline = attribute_exclusive(&ivs, AttributionWindow::new(0, 100_000))
                .expect("baseline");
            let mut rng = SplitMix64(0xDEAD_BEEF_CAFE_0023);
            for _ in 0..64 {
                rng.shuffle(&mut ivs);
                let shuffled = attribute_exclusive(&ivs, AttributionWindow::new(0, 100_000))
                    .expect("shuffled");
                assert_eq!(baseline, shuffled);
                assert_eq!(baseline.to_canonical_bytes(), shuffled.to_canonical_bytes());
            }
        }

        #[test]
        fn censored_clip() {
            // Case 1: Deadline censoring at H = 50 s (t0 = 10 s, t_stop = 60 s).
            // Intervals extending past 60 s are clipped to [10 s, 60 s).
            let ivs = vec![
                Interval::turn(1, Phase::Author, 5_000, 25_000), // clipped to [10s, 25s) -> 15s
                Interval::block(2, 25_000, 120_000, vec![3]),    // clipped to [25s, 60s) -> 35s
                Interval::work(3, Phase::Train, 25_000, 120_000, Some("train".to_owned())),
            ];
            let censored = attribute_exclusive(&ivs, AttributionWindow::new(10_000, 60_000))
                .expect("deadline censored");
            assert_eq!(censored.clock_ms, 50_000);
            assert_eq!(censored.sum_ms(), 50_000);
            assert_eq!(censored.ms(Phase::Author), 15_000);
            assert_eq!(censored.ms(Phase::Train), 35_000);
            assert_eq!(censored.censored_remainder_ms, 0);
            assert!(!censored.has_flag(FLAG_CENSORED_REMAINDER));

            // Case 2: Cap breach at breach = 40 s on horizon H = 100 s (t0 = 10 s).
            // [10s, 40s) is attributed (15s author + 15s train = 30s);
            // remainder H - (breach - t0) = 100s - 30s = 70s goes to `wait` with flag `censored-remainder`.
            let breached = attribute_exclusive(
                &ivs,
                AttributionWindow::cap_breach(10_000, 40_000, 100_000),
            )
            .expect("cap breached");
            assert_eq!(breached.clock_ms, 100_000);
            assert_eq!(breached.sum_ms(), 100_000);
            assert_eq!(breached.ms(Phase::Author), 15_000);
            assert_eq!(breached.ms(Phase::Train), 15_000);
            assert_eq!(breached.ms(Phase::Wait), 70_000);
            assert_eq!(breached.censored_remainder_ms, 70_000);
            assert!(breached.has_flag(FLAG_CENSORED_REMAINDER));
            let json = breached.to_canonical_json();
            assert!(json.contains("\"censored-remainder\""));
            assert!(json.contains("\"execution_authorized\":false"));
            assert!(json.contains("\"reality\":\"no-claim\""));
        }

        #[test]
        fn unknown_phase_refused() {
            // Fourteenth phase name refused at read time.
            let line_14th = r#"{"end_ms":"1000","id":1,"kind":"work","phase":"rehearse","start_ms":"0"}"#;
            let err1 = parse_intervals_ndjson(line_14th).expect_err("14th phase must be refused");
            assert_eq!(err1.reason, RefusalReason::RecordUnknownField);
            assert_eq!(err1.exit_code(), 3);
            assert_eq!(err1.pointer, "/phase");

            // `unattributed` is not a phase and must also be refused on an interval at read time.
            let line_unattr =
                r#"{"end_ms":"1000","id":2,"kind":"work","phase":"unattributed","start_ms":"0"}"#;
            let err2 =
                parse_intervals_ndjson(line_unattr).expect_err("unattributed is not a phase");
            assert_eq!(err2.reason, RefusalReason::RecordUnknownField);
            assert_eq!(err2.exit_code(), 3);
            assert_eq!(err2.pointer, "/phase");

            // Exact phase counts: 13 phases + 1 non-phase bucket = 14 total.
            assert_eq!(Phase::PHASES.len(), 13);
            assert_eq!(Phase::ALL.len(), 14);
            assert!(Phase::PHASES.iter().all(|p| p.is_phase()));
            assert!(!Phase::Unattributed.is_phase());
        }

        #[test]
        fn multi_phase_job_compile_then_run_and_gap() {
            // Multi-phase job sharing `proc = "sim-job"`:
            // compile [10s, 22s) up to first device step, gap [22s, 25s) -> wait, screen [25s, 45s).
            // Block awaits only the first interval id (2), and still owns the entire job through 45s.
            let ivs = vec![
                Interval::block(1, 10_000, 45_000, vec![2]),
                Interval::work(2, Phase::Compile, 10_000, 22_000, Some("sim-job".to_owned())),
                Interval::work(3, Phase::Screen, 25_000, 45_000, Some("sim-job".to_owned())),
            ];
            let table = attribute_exclusive(&ivs, AttributionWindow::new(10_000, 45_000))
                .expect("multi-phase job");
            assert_eq!(table.ms(Phase::Compile), 12_000);
            assert_eq!(table.ms(Phase::Wait), 3_000);
            assert_eq!(table.ms(Phase::Screen), 20_000);
            assert_eq!(table.sum_ms(), 35_000);
        }

        #[test]
        fn single_pair_card_known_answers() {
            // Known answers from N39a-23 §Known answers (single-pair card):
            // Arm B (clock 9,000 s): author 900, propose 1080, check 240, synthesize 0,
            //   compile 540, screen 0, train 4920, evaluate 360, qualify 0, certify 0,
            //   diagnose 600, wait 180, adjudicate 180, unattributed 0.
            // Arm A (clock 1,500 s): author 600, propose 360, check 20, synthesize 0,
            //   compile 12, screen 96, train 150, evaluate 60, qualify 40, certify 0,
            //   diagnose 30, wait 42, adjudicate 90, unattributed 0.
            let arm_b = vec![
                Interval::turn(1, Phase::Author, 0, 900_000),
                Interval::turn(2, Phase::Propose, 900_000, 1_980_000),
                Interval::block(3, 1_980_000, 2_220_000, vec![4]),
                Interval::work(4, Phase::Check, 1_980_000, 2_220_000, Some("b-chk".to_owned())),
                // Multi-phase compile + train job with 180s launch delay (wait) + shadow work.
                Interval::block(5, 2_220_000, 7_860_000, vec![6, 7, 8]),
                Interval::work(6, Phase::Compile, 2_400_000, 2_940_000, Some("b-run".to_owned())),
                Interval::work(7, Phase::Train, 2_940_000, 7_860_000, Some("b-run".to_owned())),
                Interval::work(8, Phase::Screen, 2_220_000, 3_000_000, Some("b-shadow".to_owned())),
                Interval::block(9, 7_860_000, 8_220_000, vec![10]),
                Interval::work(10, Phase::Evaluate, 7_860_000, 8_220_000, Some("b-eval".to_owned())),
                Interval::turn(11, Phase::Diagnose, 8_220_000, 8_820_000),
                Interval::block(12, 8_820_000, 9_000_000, vec![13]),
                Interval::adjudication(13, 8_820_000, 9_000_000),
            ];
            let tb = attribute_exclusive(&arm_b, AttributionWindow::new(0, 9_000_000))
                .expect("arm B known answer");
            assert_eq!(tb.sum_ms(), 9_000_000);
            assert_eq!(tb.get(Phase::Author), Millis(900_000));
            assert_eq!(tb.get(Phase::Propose), Millis(1_080_000));
            assert_eq!(tb.get(Phase::Check), Millis(240_000));
            assert_eq!(tb.get(Phase::Synthesize), Millis(0));
            assert_eq!(tb.get(Phase::Compile), Millis(540_000));
            assert_eq!(tb.get(Phase::Screen), Millis(0));
            assert_eq!(tb.get(Phase::Train), Millis(4_920_000));
            assert_eq!(tb.get(Phase::Evaluate), Millis(360_000));
            assert_eq!(tb.get(Phase::Qualify), Millis(0));
            assert_eq!(tb.get(Phase::Certify), Millis(0));
            assert_eq!(tb.get(Phase::Diagnose), Millis(600_000));
            assert_eq!(tb.get(Phase::Wait), Millis(180_000));
            assert_eq!(tb.get(Phase::Adjudicate), Millis(180_000));
            assert_eq!(tb.get(Phase::Unattributed), Millis(0));

            let arm_a = vec![
                Interval::turn(1, Phase::Author, 0, 600_000),
                Interval::turn(2, Phase::Propose, 600_000, 960_000),
                Interval::block(3, 960_000, 980_000, vec![4]),
                Interval::work(4, Phase::Check, 960_000, 980_000, Some("a-chk".to_owned())),
                Interval::block(5, 980_000, 1_130_000, vec![6, 7]),
                Interval::work(6, Phase::Compile, 1_022_000, 1_034_000, Some("a-sim".to_owned())),
                Interval::work(7, Phase::Screen, 1_034_000, 1_130_000, Some("a-sim".to_owned())),
                Interval::block(8, 1_130_000, 1_280_000, vec![9]),
                Interval::work(9, Phase::Train, 1_130_000, 1_280_000, Some("a-tr".to_owned())),
                Interval::block(10, 1_280_000, 1_340_000, vec![11]),
                Interval::work(11, Phase::Evaluate, 1_280_000, 1_340_000, Some("a-ev".to_owned())),
                Interval::block(12, 1_340_000, 1_380_000, vec![13]),
                Interval::work(13, Phase::Qualify, 1_340_000, 1_380_000, Some("a-ql".to_owned())),
                Interval::turn(14, Phase::Diagnose, 1_380_000, 1_410_000),
                Interval::block(15, 1_410_000, 1_500_000, vec![16]),
                Interval::adjudication(16, 1_410_000, 1_500_000),
            ];
            let ta = attribute_exclusive(&arm_a, AttributionWindow::new(0, 1_500_000))
                .expect("arm A known answer");
            assert_eq!(ta.sum_ms(), 1_500_000);
            assert_eq!(ta.get(Phase::Author), Millis(600_000));
            assert_eq!(ta.get(Phase::Propose), Millis(360_000));
            assert_eq!(ta.get(Phase::Check), Millis(20_000));
            assert_eq!(ta.get(Phase::Synthesize), Millis(0));
            assert_eq!(ta.get(Phase::Compile), Millis(12_000));
            assert_eq!(ta.get(Phase::Screen), Millis(96_000));
            assert_eq!(ta.get(Phase::Train), Millis(150_000));
            assert_eq!(ta.get(Phase::Evaluate), Millis(60_000));
            assert_eq!(ta.get(Phase::Qualify), Millis(40_000));
            assert_eq!(ta.get(Phase::Certify), Millis(0));
            assert_eq!(ta.get(Phase::Diagnose), Millis(30_000));
            assert_eq!(ta.get(Phase::Wait), Millis(42_000));
            assert_eq!(ta.get(Phase::Adjudicate), Millis(90_000));
            assert_eq!(ta.get(Phase::Unattributed), Millis(0));
        }

        #[test]
        fn product_phase_vocabulary_mapping() {
            assert_eq!(
                map_product_phase(ProductVocabulary::StudioLoop, "authoring", "/p").unwrap(),
                &[Phase::Author][..]
            );
            assert_eq!(
                map_product_phase(ProductVocabulary::StudioLoop, "review", "/p").unwrap(),
                &[Phase::Propose, Phase::Diagnose][..]
            );
            assert_eq!(
                map_product_phase(ProductVocabulary::PerformanceProgram, "compilation", "/p")
                    .unwrap(),
                &[Phase::Compile][..]
            );
            assert_eq!(
                map_product_phase(ProductVocabulary::PerformanceProgram, "admission", "/p")
                    .unwrap(),
                &[Phase::Compile][..]
            );
            assert!(product_names_for_phase(Phase::Certify).is_empty());
            assert!(product_names_for_phase(Phase::Adjudicate).is_empty());
            assert!(product_names_for_phase(Phase::Unattributed).is_empty());
            assert!(
                map_product_phase(ProductVocabulary::StudioLoop, "unknown", "/p").is_err()
            );
            assert_eq!(PRODUCT_PHASE_MAP.len(), 15);
        }

        #[test]
        fn overlapping_turns_first_start_then_lower_id() {
            // Rule 2a: turn starting first owns overlap; equal start -> lower id.
            let ivs = vec![
                Interval::turn(20, Phase::Author, 0, 30_000),
                Interval::turn(10, Phase::Diagnose, 10_000, 40_000),
                // Equal start at 40s: id 5 beats id 15.
                Interval::turn(15, Phase::Propose, 40_000, 60_000),
                Interval::turn(5, Phase::Author, 40_000, 60_000),
            ];
            let t = attribute_exclusive(&ivs, AttributionWindow::new(0, 60_000)).unwrap();
            assert_eq!(t.ms(Phase::Author), 50_000); // [0, 30s) + [40s, 60s)
            assert_eq!(t.ms(Phase::Diagnose), 10_000); // [30s, 40s)
            assert_eq!(t.ms(Phase::Propose), 0);
        }

        #[test]
        fn harness_and_adjudication_without_turn_or_block() {
            // Rule 2c: bare adjudication -> adjudicate (and beats bare harness); bare harness -> wait.
            let ivs = vec![
                Interval::harness(1, 0, 30_000),
                Interval::adjudication(2, 10_000, 20_000),
            ];
            let t = attribute_exclusive(&ivs, AttributionWindow::new(0, 40_000)).unwrap();
            assert_eq!(t.ms(Phase::Wait), 20_000);
            assert_eq!(t.ms(Phase::Adjudicate), 10_000);
            assert_eq!(t.ms(Phase::Unattributed), 10_000);
            assert_eq!(t.sum_ms(), 40_000);
        }

        #[test]
        fn negative_controls_fail_invariants() {
            // Planted buggy attributions named in N39a-23 §Negative controls must fail the invariants.
            let ivs = worked_fixture_intervals();
            let clock = 100_000_u64;

            // NC-1: Summing all activity (concurrent jobs exceed clock: 20+12+18+48+20 = 118s > 100s).
            let sum_all: u64 = ivs
                .iter()
                .filter(|iv| iv.kind != IntervalKind::Block)
                .map(|iv| iv.end_ms - iv.start_ms)
                .sum();
            assert!(sum_all > clock, "sum-all-activity must exceed clock");

            // NC-2: Splitting blocked time evenly among awaited jobs gives train = 24s != 48s.
            let even_split_train = 48_000_u64 / 2;
            let good = attribute_exclusive(&ivs, AttributionWindow::new(0, clock)).unwrap();
            assert_ne!(even_split_train, good.ms(Phase::Train));

            // NC-3: Dropping gaps instead of marking unattributed makes sum fall short (90s < 100s).
            let drop_gaps_sum = good.sum_ms() - good.ms(Phase::Unattributed);
            assert!(drop_gaps_sum < clock, "dropping gaps must fall short of clock");
        }

        #[test]
        fn duplicate_id_and_bad_timestamps_refused() {
            // Duplicate interval id refused.
            let dup = "{\"end_ms\":\"10\",\"id\":7,\"kind\":\"harness\",\"phase\":null,\"start_ms\":\"0\"}\n\
                       {\"end_ms\":\"20\",\"id\":7,\"kind\":\"harness\",\"phase\":null,\"start_ms\":\"10\"}\n";
            let e_dup = parse_intervals_ndjson(dup).expect_err("duplicate id must fail");
            assert_eq!(e_dup.reason, RefusalReason::RecordDuplicateKey);
            assert_eq!(e_dup.pointer, "/id");

            // Inverted timestamps refused.
            let inv = r#"{"end_ms":"5","id":1,"kind":"harness","phase":null,"start_ms":"10"}"#;
            let e_inv = parse_intervals_ndjson(inv).expect_err("end < start must fail");
            assert_eq!(e_inv.reason, RefusalReason::RecordNoncanonicalNumber);
            assert_eq!(e_inv.pointer, "/end_ms");

            // Numeric start_ms refused (must be canonical string).
            let num = r#"{"end_ms":"10","id":1,"kind":"harness","phase":null,"start_ms":0}"#;
            let e_num = parse_intervals_ndjson(num).expect_err("numeric start_ms must fail");
            assert_eq!(e_num.reason, RefusalReason::RecordNoncanonicalNumber);
            assert_eq!(e_num.pointer, "/start_ms");

            // Full rich interval with resource + tokens round-trips cleanly.
            let mut rich = Interval::work(
                17,
                Phase::Train,
                1_000,
                5_000,
                Some("train-worker-1".to_owned()),
            );
            rich.resource = Some(ResourceInfo {
                devices: vec![0, 1],
                cores: Some(8),
                cpu_ms: Some(12_000),
                learner_ms: Some(900),
            });
            rich.tag_rule = Some("rule:train-ppo".to_owned());
            let mut rich_turn = Interval::turn(18, Phase::Propose, 0, 1_000);
            rich_turn.tokens_in = Some(4096);
            rich_turn.tokens_out = Some(512);
            rich_turn.tool_calls = Some(2);
            let nd = write_intervals_ndjson(&[rich.clone(), rich_turn.clone()]);
            let parsed = parse_intervals_ndjson(&nd).expect("rich intervals roundtrip");
            assert_eq!(parsed, vec![rich, rich_turn]);
        }
    }
}
