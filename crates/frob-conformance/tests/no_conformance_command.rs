#![deny(unsafe_code)]

//! Enforcing lint for Gauntlet command forms (`BD-025` / `ADR-GAUNT-01`).
//!
//! Scans every file under `scripts/`, `xtask/`, and `crates/xtask/` line by line
//! (skipping comment lines whose first non-blank character is `#`) and enforces
//! all five command-form rules in order.
//!
//! Supports `FROB_CONFORMANCE_ROOT=<dir>` for scratch workspaces in
//! `scripts/e2e/conformance_v0.sh`. Never deletes files or directories (COMMON.md 9b).

use std::collections::BTreeSet;
use std::env;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use frob_core::testkit::registry;

static SCRATCH_SEQ: AtomicU64 = AtomicU64::new(0);

const SHORT_TIERS: &[(&str, &str)] = &[
    ("g0", "g0_laws"),
    ("g1", "g1_known_answers"),
    ("g2", "g2_parity"),
    ("g3", "g3_metamorphic"),
    ("g4", "g4_chaos"),
    ("g5", "g5_determinism"),
];

#[derive(Debug, Clone, PartialEq, Eq)]
struct CommandFinding {
    rule: u8,
    path: String,
    line: usize,
    offending: String,
    replacement: String,
}

impl fmt::Display for CommandFinding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[rule-{}] {}:{}: `{}` -> use: {}",
            self.rule, self.path, self.line, self.offending, self.replacement
        )
    }
}

#[derive(Debug, Clone, Default)]
struct DeclaredTargets {
    tests: BTreeSet<String>,
    examples: BTreeSet<String>,
    bins: BTreeSet<String>,
}

impl DeclaredTargets {
    fn load(root: &Path) -> Self {
        let mut out = Self::default();
        let crate_dir = root.join("crates/frob-conformance");
        for (subdir, set) in [("tests", &mut out.tests), ("examples", &mut out.examples)] {
            let dir = crate_dir.join(subdir);
            if let Ok(rd) = fs::read_dir(&dir) {
                for entry in rd.flatten().map(|e| e.path()) {
                    if entry.is_file()
                        && entry.extension().and_then(|s| s.to_str()) == Some("rs")
                        && let Some(stem) = entry.file_stem().and_then(|s| s.to_str())
                    {
                        set.insert(stem.to_owned());
                    }
                }
            }
        }
        if let Ok(manifest) = fs::read_to_string(crate_dir.join("Cargo.toml")) {
            let mut section = "";
            for raw in manifest.lines() {
                let line = strip_toml_comment(raw).trim();
                if line.is_empty() {
                    continue;
                }
                if line.starts_with('[') && line.ends_with(']') {
                    section = match line {
                        "[[test]]" => "test",
                        "[[example]]" => "example",
                        "[[bin]]" => "bin",
                        _ => "",
                    };
                    continue;
                }
                if !section.is_empty()
                    && let Some((k, v)) = line.split_once('=')
                    && k.trim() == "name"
                {
                    let name = v.trim().trim_matches('"').trim_matches('\'').to_owned();
                    if !name.is_empty() {
                        match section {
                            "test" => {
                                out.tests.insert(name);
                            }
                            "example" => {
                                out.examples.insert(name);
                            }
                            "bin" => {
                                out.bins.insert(name);
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
        out
    }

    fn format_list(set: &BTreeSet<String>) -> String {
        if set.is_empty() {
            "(none declared)".to_owned()
        } else {
            set.iter().cloned().collect::<Vec<_>>().join(", ")
        }
    }
}

fn strip_toml_comment(line: &str) -> &str {
    let (mut in_dq, mut in_sq, mut esc) = (false, false, false);
    for (i, b) in line.bytes().enumerate() {
        if esc {
            esc = false;
            continue;
        }
        if in_dq {
            if b == b'\\' {
                esc = true;
            } else if b == b'"' {
                in_dq = false;
            }
        } else if in_sq {
            if b == b'\'' {
                in_sq = false;
            }
        } else if b == b'"' {
            in_dq = true;
        } else if b == b'\'' {
            in_sq = true;
        } else if b == b'#' {
            return &line[..i];
        }
    }
    line
}

fn workspace_root() -> PathBuf {
    if let Some(ov) = env::var_os("FROB_CONFORMANCE_ROOT") {
        return PathBuf::from(ov);
    }
    registry::root().expect("locate workspace root via franken-stack.lock")
}

fn scratch_dir(label: &str) -> PathBuf {
    let base = env::var_os("BIG_SCRATCH")
        .or_else(|| env::var_os("TMPDIR"))
        .map(PathBuf::from)
        .unwrap_or_else(env::temp_dir);
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let seq = SCRATCH_SEQ.fetch_add(1, Ordering::Relaxed);
    let dir = base
        .join("frob-BronzePelican")
        .join(format!("no_conformance_command-{label}-{nanos}-{seq}"));
    fs::create_dir_all(&dir).expect("create scratch directory");
    dir
}

fn rel_display(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

fn walk_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(rd) = fs::read_dir(dir) else { return };
    let mut entries: Vec<PathBuf> = rd.flatten().map(|e| e.path()).collect();
    entries.sort();
    for p in entries {
        let name = p.file_name().and_then(|s| s.to_str()).unwrap_or("");
        if p.is_dir() {
            if name != ".git"
                && name != "target"
                && name != ".rch-tmp"
                && !name.starts_with(".rch-target-")
            {
                walk_files(&p, out);
            }
        } else if p.is_file() {
            out.push(p);
        }
    }
}

/// Tokenize one shell line into command segments separated by `;`, `&&`, `||`, `|`, `$(`, `` ` ``.
fn split_shell_segments(line: &str) -> Vec<Vec<String>> {
    let mut segments: Vec<Vec<String>> = Vec::new();
    let mut cur_seg: Vec<String> = Vec::new();
    let mut cur_tok = String::new();
    let bytes = line.as_bytes();
    let n = bytes.len();
    let mut i = 0usize;
    let mut in_sq = false;
    let mut in_dq = false;

    let flush_tok = |seg: &mut Vec<String>, tok: &mut String| {
        if !tok.is_empty() {
            seg.push(std::mem::take(tok));
        }
    };
    let flush_seg = |segs: &mut Vec<Vec<String>>, seg: &mut Vec<String>, tok: &mut String| {
        flush_tok(seg, tok);
        if !seg.is_empty() {
            segs.push(std::mem::take(seg));
        }
    };

    while i < n {
        let b = bytes[i];
        if in_sq {
            cur_tok.push(b as char);
            if b == b'\'' {
                in_sq = false;
            }
            i += 1;
            continue;
        }
        if in_dq {
            if b == b'\\' && i + 1 < n {
                cur_tok.push(b as char);
                cur_tok.push(bytes[i + 1] as char);
                i += 2;
                continue;
            }
            cur_tok.push(b as char);
            if b == b'"' {
                in_dq = false;
            }
            i += 1;
            continue;
        }

        // Unquoted inline comment `#` at token boundary terminates the line.
        if b == b'#' && cur_tok.is_empty() {
            break;
        }
        if b == b'\'' {
            in_sq = true;
            cur_tok.push('\'');
            i += 1;
            continue;
        }
        if b == b'"' {
            in_dq = true;
            cur_tok.push('"');
            i += 1;
            continue;
        }
        if b == b'\\' && i + 1 < n {
            cur_tok.push(bytes[i + 1] as char);
            i += 2;
            continue;
        }
        if b == b';' || b == b'`' {
            flush_seg(&mut segments, &mut cur_seg, &mut cur_tok);
            i += 1;
            continue;
        }
        if b == b'|' {
            flush_seg(&mut segments, &mut cur_seg, &mut cur_tok);
            i += if i + 1 < n && bytes[i + 1] == b'|' { 2 } else { 1 };
            continue;
        }
        if b == b'&' && i + 1 < n && bytes[i + 1] == b'&' {
            flush_seg(&mut segments, &mut cur_seg, &mut cur_tok);
            i += 2;
            continue;
        }
        if b == b'$' && i + 1 < n && bytes[i + 1] == b'(' {
            flush_seg(&mut segments, &mut cur_seg, &mut cur_tok);
            i += 2;
            continue;
        }
        if b == b')' {
            flush_seg(&mut segments, &mut cur_seg, &mut cur_tok);
            i += 1;
            continue;
        }
        if b.is_ascii_whitespace() {
            flush_tok(&mut cur_seg, &mut cur_tok);
            i += 1;
            continue;
        }
        let ch = line[i..].chars().next().expect("valid utf8");
        cur_tok.push(ch);
        i += ch.len_utf8();
    }
    flush_seg(&mut segments, &mut cur_seg, &mut cur_tok);
    segments
}

fn unquote_token(tok: &str) -> &str {
    if tok.len() >= 2
        && ((tok.starts_with('"') && tok.ends_with('"'))
            || (tok.starts_with('\'') && tok.ends_with('\'')))
    {
        &tok[1..tok.len() - 1]
    } else {
        tok
    }
}

fn is_env_assignment(tok: &str) -> bool {
    let raw = unquote_token(tok);
    let Some((name, _)) = raw.split_once('=') else {
        return false;
    };
    !name.is_empty()
        && (name.as_bytes()[0].is_ascii_alphabetic() || name.as_bytes()[0] == b'_')
        && name
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'_')
}

/// Returns index of the command word within a shell segment after skipping
/// leading shell keywords (`then`, `do`, `else`, `elif`, `exec`, `!`, `time`, `command`)
/// and leading `VAR=val` assignments (and `env VAR=val ...` wrappers).
fn command_word_index(seg: &[String]) -> Option<usize> {
    let mut i = 0usize;
    while i < seg.len() {
        let tok = seg[i].as_str();
        if matches!(tok, "then" | "do" | "else" | "elif" | "exec" | "!" | "time" | "command") {
            i += 1;
            continue;
        }
        if is_env_assignment(tok) {
            i += 1;
            continue;
        }
        if tok == "env" {
            i += 1;
            while i < seg.len() && (seg[i].starts_with('-') || is_env_assignment(&seg[i])) {
                i += 1;
            }
            continue;
        }
        // Handle wrappers such as `e2e_cmd --name foo -- <cmd...>` or `frob_envelope_run ... -- <cmd...>`.
        if matches!(tok, "e2e_cmd" | "e2e_expect_caught" | "frob_envelope_run") {
            if let Some(pos) = seg[i..].iter().position(|t| t == "--") {
                i = i + pos + 1;
                continue;
            }
        }
        return Some(i);
    }
    None
}

fn check_line(
    rel_path: &str,
    line_no: usize,
    raw_line: &str,
    declared: &DeclaredTargets,
    findings: &mut Vec<CommandFinding>,
) {
    let trimmed = raw_line.trim();
    if trimmed.is_empty()
        || trimmed.starts_with('#')
        || (rel_path.ends_with(".rs") && trimmed.starts_with("//"))
    {
        return;
    }

    let segments = split_shell_segments(raw_line);
    for seg in &segments {
        let Some(cmd_idx) = command_word_index(seg) else {
            continue;
        };
        let cmd_slice = &seg[cmd_idx..];
        if cmd_slice.is_empty() {
            continue;
        }

        let cmd_word = unquote_token(&cmd_slice[0]);
        let cmd_base = cmd_word.rsplit('/').next().unwrap_or(cmd_word);

        // Rule 1: `frob conformance` followed by anything.
        if cmd_base == "frob"
            && cmd_slice
                .get(1)
                .is_some_and(|t| unquote_token(t) == "conformance")
        {
            findings.push(CommandFinding {
                rule: 1,
                path: rel_path.to_owned(),
                line: line_no,
                offending: trimmed.to_owned(),
                replacement: "FROB_TEST_SEED=<seed> cargo test -p frob-conformance --test <tier> -- <gate|row>".to_owned(),
            });
            continue;
        }

        // Rule 2: `frob-conformance` in command position.
        // Quoted crate names inside paths or arguments are NOT in command position.
        let is_quoted_literal = (cmd_slice[0].starts_with('"') && cmd_slice[0].ends_with('"'))
            || (cmd_slice[0].starts_with('\'') && cmd_slice[0].ends_with('\''));
        if !is_quoted_literal && cmd_base == "frob-conformance" {
            findings.push(CommandFinding {
                rule: 2,
                path: rel_path.to_owned(),
                line: line_no,
                offending: trimmed.to_owned(),
                replacement: concat!(
                    "no `frob-conformance` binary exists; use ",
                    "`FROB_TEST_SEED=<seed> cargo test -p frob-conformance --test <tier> -- <gate|row>` (tier), ",
                    "`cargo run -p frob-conformance --example record_gate -- --gate <id> --receipt <hash>` (recording), or ",
                    "`cargo run -p frob-conformance --example <name> -- ...` (harness driver)"
                )
                .to_owned(),
            });
            continue;
        }

        // Rules 3, 4, 5 apply to `cargo` commands targeting `-p frob-conformance` (and Rule 4 also
        // catches short `--test g0`..`g5` in any cargo test command).
        if cmd_base == "cargo" {
            check_cargo_command(rel_path, line_no, trimmed, cmd_slice, declared, findings);
        }
    }
}

fn check_cargo_command(
    rel_path: &str,
    line_no: usize,
    trimmed_line: &str,
    cmd: &[String],
    declared: &DeclaredTargets,
    findings: &mut Vec<CommandFinding>,
) {
    // Locate cargo subcommand (skipping `+toolchain` and flags before subcommand).
    let mut sub_idx = 1usize;
    while sub_idx < cmd.len() {
        let t = unquote_token(&cmd[sub_idx]);
        if t.starts_with('+') || t.starts_with('-') {
            sub_idx += 1;
            continue;
        }
        break;
    }
    let subcmd = cmd.get(sub_idx).map(|s| unquote_token(s)).unwrap_or("");
    let args = if sub_idx + 1 <= cmd.len() {
        &cmd[sub_idx + 1..]
    } else {
        &[]
    };

    // Split cargo args at `--` so binary/test filter args are separated from cargo options.
    let dd_pos = args.iter().position(|t| t == "--");
    let cargo_opts = match dd_pos {
        Some(p) => &args[..p],
        None => args,
    };
    let trailing = match dd_pos {
        Some(p) => &args[p + 1..],
        None => &[],
    };

    let mut targets_frob_conformance = false;
    let mut has_bin = false;
    let mut has_example = false;
    let mut test_targets: Vec<String> = Vec::new();
    let mut example_targets: Vec<String> = Vec::new();
    let mut bin_targets: Vec<String> = Vec::new();

    let mut i = 0usize;
    while i < cargo_opts.len() {
        let raw = &cargo_opts[i];
        let tok = unquote_token(raw);
        if tok == "-p" || tok == "--package" {
            if let Some(pkg) = cargo_opts.get(i + 1) {
                if unquote_token(pkg) == "frob-conformance" {
                    targets_frob_conformance = true;
                }
                i += 2;
                continue;
            }
        } else if let Some(pkg) = tok.strip_prefix("--package=").or_else(|| tok.strip_prefix("-p"))
        {
            if !pkg.is_empty() && pkg == "frob-conformance" {
                targets_frob_conformance = true;
            }
        } else if tok == "--bin" {
            has_bin = true;
            if let Some(val) = cargo_opts.get(i + 1) {
                bin_targets.push(unquote_token(val).to_owned());
                i += 2;
                continue;
            }
        } else if let Some(val) = tok.strip_prefix("--bin=") {
            has_bin = true;
            bin_targets.push(unquote_token(val).to_owned());
        } else if tok == "--example" {
            has_example = true;
            if let Some(val) = cargo_opts.get(i + 1) {
                example_targets.push(unquote_token(val).to_owned());
                i += 2;
                continue;
            }
        } else if let Some(val) = tok.strip_prefix("--example=") {
            has_example = true;
            example_targets.push(unquote_token(val).to_owned());
        } else if tok == "--test" {
            if let Some(val) = cargo_opts.get(i + 1) {
                test_targets.push(unquote_token(val).to_owned());
                i += 2;
                continue;
            }
        } else if let Some(val) = tok.strip_prefix("--test=") {
            test_targets.push(unquote_token(val).to_owned());
        }
        i += 1;
    }

    // Rule 3: `cargo run` with `-p frob-conformance` and neither `--bin` nor `--example`.
    if subcmd == "run" && targets_frob_conformance && !has_bin && !has_example {
        let suggested = trailing
            .first()
            .map(|t| unquote_token(t))
            .filter(|t| !t.is_empty() && !t.starts_with('-'))
            .map_or_else(
                || "cargo run -p frob-conformance --example <name> -- ... (or --bin <name>)".to_owned(),
                |first| format!("cargo run -p frob-conformance --example {first} -- ..."),
            );
        findings.push(CommandFinding {
            rule: 3,
            path: rel_path.to_owned(),
            line: line_no,
            offending: trimmed_line.to_owned(),
            replacement: suggested,
        });
        return;
    }

    // Rule 4: `--test g0`..`--test g5` short tier names (exact token match implies not followed
    // by a letter, digit, or `_`).
    let mut fired_rule4 = false;
    for t in &test_targets {
        if let Some((_, long_name)) = SHORT_TIERS.iter().find(|(short, _)| *short == t.as_str()) {
            findings.push(CommandFinding {
                rule: 4,
                path: rel_path.to_owned(),
                line: line_no,
                offending: trimmed_line.to_owned(),
                replacement: format!("--test {long_name} (short tier name `{t}` is not a target)"),
            });
            fired_rule4 = true;
        }
    }
    if fired_rule4 {
        return;
    }

    // Rule 5: `--test <t>`, `--example <e>`, or `--bin <b>` with `-p frob-conformance`
    // must match a target declared by the crate.
    if !targets_frob_conformance {
        return;
    }

    for t in &test_targets {
        if t.starts_with('$') {
            continue;
        }
        if !declared.tests.contains(t) {
            findings.push(CommandFinding {
                rule: 5,
                path: rel_path.to_owned(),
                line: line_no,
                offending: trimmed_line.to_owned(),
                replacement: format!(
                    "unknown-target test `{t}`; declared tests: [{}]",
                    DeclaredTargets::format_list(&declared.tests)
                ),
            });
        }
    }
    for e in &example_targets {
        if e.starts_with('$') {
            continue;
        }
        // STD-E2E's internal fakebin selftest (`scripts/e2e/selftest/st_self_record.sh`)
        // invokes `--example record_gate` under a stub `fakebin/cargo` before N38b-19
        // creates `examples/record_gate.rs`.
        if rel_path == "scripts/e2e/selftest/st_self_record.sh" && e == "record_gate" {
            continue;
        }
        if !declared.examples.contains(e) {
            findings.push(CommandFinding {
                rule: 5,
                path: rel_path.to_owned(),
                line: line_no,
                offending: trimmed_line.to_owned(),
                replacement: format!(
                    "unknown-target example `{e}`; declared examples: [{}]",
                    DeclaredTargets::format_list(&declared.examples)
                ),
            });
        }
    }
    for b in &bin_targets {
        if b.starts_with('$') {
            continue;
        }
        if !declared.bins.contains(b) {
            findings.push(CommandFinding {
                rule: 5,
                path: rel_path.to_owned(),
                line: line_no,
                offending: trimmed_line.to_owned(),
                replacement: format!(
                    "unknown-target bin `{b}`; declared binaries: [{}]",
                    DeclaredTargets::format_list(&declared.bins)
                ),
            });
        }
    }
}

fn scan_workspace(root: &Path) -> (usize, Vec<CommandFinding>) {
    let declared = DeclaredTargets::load(root);
    let mut files = Vec::new();
    for dir_rel in ["scripts", "xtask", "crates/xtask"] {
        let dir = root.join(dir_rel);
        if dir.is_dir() {
            walk_files(&dir, &mut files);
        }
    }
    files.sort();
    files.dedup();

    let mut findings = Vec::new();
    let mut scanned = 0usize;
    for path in &files {
        let Ok(content) = fs::read_to_string(path) else {
            continue;
        };
        scanned += 1;
        let rel = rel_display(root, path);
        for (idx, line) in content.lines().enumerate() {
            check_line(&rel, idx + 1, line, &declared, &mut findings);
        }
    }
    (scanned, findings)
}

#[test]
fn no_conformance_command_clean_tree() {
    let root = workspace_root();
    let (scanned, findings) = scan_workspace(&root);
    println!("no_conformance_command: scripts_scanned={scanned} findings={}", findings.len());
    assert!(
        findings.is_empty(),
        "forbidden conformance command forms found under {}:\n{}",
        root.display(),
        findings
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join("\n")
    );
}

#[test]
fn no_conformance_command() {
    let ws = scratch_dir("five_rules");
    // Minimal declared targets in scratch workspace so admitted forms pass and unknown targets fail.
    let crate_dir = ws.join("crates/frob-conformance");
    fs::create_dir_all(crate_dir.join("tests")).expect("mkdir tests");
    fs::create_dir_all(crate_dir.join("examples")).expect("mkdir examples");
    fs::write(
        crate_dir.join("Cargo.toml"),
        r#"[package]
name = "frob-conformance"
version = "0.0.1"
edition = "2024"
autobins = false

[[bin]]
name = "frob-e2e"
path = "src/bin/frob-e2e.rs"
"#,
    )
    .expect("write Cargo.toml");
    for t in [
        "g0_laws.rs",
        "g1_known_answers.rs",
        "g2_parity.rs",
        "g3_metamorphic.rs",
        "g4_chaos.rs",
        "g5_determinism.rs",
    ] {
        fs::write(crate_dir.join("tests").join(t), "// test\n").expect("write test");
    }
    for ex in ["record_gate.rs", "pilots.rs", "atelier_harness.rs"] {
        fs::write(crate_dir.join("examples").join(ex), "fn main() {}\n").expect("write example");
    }

    // 1) Accepted forms produce zero findings.
    let scripts_dir = ws.join("scripts");
    fs::create_dir_all(&scripts_dir).expect("mkdir scripts");
    let accepted = r#"#!/usr/bin/env bash
# there is no frob-conformance run
FROB_TEST_SEED=0 cargo test -p frob-conformance --test g1_known_answers -- GATE-ARENA-05
cargo run -p frob-conformance --example atelier_harness -- ground.identify --json
cargo run -p frob-conformance --example record_gate -- --gate GATE-X --receipt deadbeef
cargo run -p frob-conformance --example pilots -- fundable --all --json
cargo test -p frob-conformance --test g1_known_answers -- KA-501
cargo run -p frob-conformance --bin frob-e2e -- schema
cargo test -p frob-conformance tenfold::baseline
cat crates/frob-conformance/registry/controls.toml
"#;
    fs::write(scripts_dir.join("accepted.sh"), accepted).expect("write accepted.sh");
    let xtask_src = ws.join("crates/xtask/src");
    fs::create_dir_all(&xtask_src).expect("mkdir crates/xtask/src");
    fs::write(
        xtask_src.join("doc_comment.rs"),
        "/// - `frob-conformance` harness module and `frob-arena` study runner.\n\
         // Also ignore `frob conformance` in plain Rust line comments.\n\
         pub const OK: bool = true;\n",
    )
    .expect("write doc_comment.rs");
    let (_, ok_findings) = scan_workspace(&ws);
    assert!(
        ok_findings.is_empty(),
        "admitted forms must not be flagged:\n{}",
        ok_findings
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join("\n")
    );

    // 2) All 7 planted bad lines across the 5 rules fail with file, line, and replacement.
    let bad = r#"#!/usr/bin/env bash
frob conformance run GATE-ARENA-05
frob-conformance run ground.identify --json
x=1 frob-conformance record --gate GATE-X
cargo run -p frob-conformance -- pilots fundable --all --json
cargo test -p frob-conformance --test g1 -- KA-501
cargo run -p frob-conformance --example no_such_tool
cargo run -p frob-conformance --bin frob-conformance
"#;
    fs::write(scripts_dir.join("planted_bad.sh"), bad).expect("write planted_bad.sh");
    let (_, findings) = scan_workspace(&ws);
    assert_eq!(
        findings.len(),
        7,
        "expected exactly 7 findings for 7 planted lines, got:\n{}",
        findings
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join("\n")
    );

    // Line 2: Rule 1 (`frob conformance run GATE-ARENA-05`)
    assert_eq!(findings[0].path, "scripts/planted_bad.sh");
    assert_eq!(findings[0].line, 2);
    assert_eq!(findings[0].rule, 1);
    assert!(findings[0].replacement.contains("--test <tier>"));

    // Line 3: Rule 2 (`frob-conformance run ground.identify --json`)
    assert_eq!(findings[1].line, 3);
    assert_eq!(findings[1].rule, 2);
    assert!(findings[1].replacement.contains("--example record_gate"));
    assert!(findings[1].replacement.contains("--example <name>"));

    // Line 4: Rule 2 (`x=1 frob-conformance record --gate GATE-X`)
    assert_eq!(findings[2].line, 4);
    assert_eq!(findings[2].rule, 2);
    assert!(findings[2].replacement.contains("--example record_gate"));
    assert!(findings[2].replacement.contains("--example <name>"));

    // Line 5: Rule 3 (`cargo run -p frob-conformance -- pilots fundable --all --json`)
    assert_eq!(findings[3].line, 5);
    assert_eq!(findings[3].rule, 3);
    assert!(findings[3].replacement.contains("--example pilots"));

    // Line 6: Rule 4 (`cargo test -p frob-conformance --test g1 -- KA-501`)
    assert_eq!(findings[4].line, 6);
    assert_eq!(findings[4].rule, 4);
    assert!(findings[4].replacement.contains("g1_known_answers"));

    // Line 7: Rule 5 (`cargo run -p frob-conformance --example no_such_tool`)
    assert_eq!(findings[5].line, 7);
    assert_eq!(findings[5].rule, 5);
    assert!(findings[5].replacement.contains("unknown-target"));
    assert!(findings[5].replacement.contains("record_gate"));

    // Line 8: Rule 5 (`cargo run -p frob-conformance --bin frob-conformance`)
    assert_eq!(findings[6].line, 8);
    assert_eq!(findings[6].rule, 5);
    assert!(findings[6].replacement.contains("unknown-target"));
    assert!(findings[6].replacement.contains("frob-e2e"));
}
