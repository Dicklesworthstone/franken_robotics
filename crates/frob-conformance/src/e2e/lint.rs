//! Linter (`frob-e2e lint`) for `frob.e2elog/1` NDJSON logs and capture artifacts.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use crate::e2e::json::{self, JsonValue, MAX_LINE_BYTES, obj};
use crate::e2e::line::{
    LOG_SCHEMA_VERSION, ParsedFlags, VALID_ASSERT_KINDS, VALID_EVENTS, VALID_TIERS,
    effective_private_root, parse_utc_rfc3339_ms,
};
use crate::e2e::sha256::sha256_hex;

pub const PRIVATE_CANARY: &str = "FROB-PRIVATE-CANARY-7f3a";
pub const TOKEN_CANARY: &str = "FROB-TOKEN-CANARY-5c1e";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LintFinding { pub rule: String, pub file: String, pub line: usize, pub detail: String }

impl LintFinding {
    pub fn to_json(&self) -> JsonValue {
        obj(vec![
            ("rule", JsonValue::String(self.rule.clone())), ("file", JsonValue::String(self.file.clone())),
            ("line", JsonValue::Number(self.line.to_string())), ("detail", JsonValue::String(self.detail.clone())),
        ])
    }
}

fn push(out: &mut Vec<LintFinding>, rule: &str, file: &str, line: usize, detail: impl Into<String>) {
    out.push(LintFinding { rule: rule.into(), file: file.into(), line, detail: detail.into() });
}

fn has_strs(val: &JsonValue, keys: &[&str]) -> bool {
    keys.iter().all(|k| val.get(k).and_then(JsonValue::as_str).is_some())
}

fn private_root_needles(private_root: Option<&str>) -> Vec<String> {
    let mut needles = Vec::new();
    let Some(raw) = private_root.map(|s| s.trim_end_matches('/')).filter(|s| !s.is_empty()) else { return needles; };
    needles.push(raw.to_string());
    if let Ok(canon) = fs::canonicalize(raw) {
        let cs = canon.to_string_lossy().trim_end_matches('/').to_string();
        if !cs.is_empty() && !needles.contains(&cs) { needles.push(cs); }
    }
    needles
}

fn is_path_boundary(b: Option<u8>) -> bool {
    b.is_none_or(|c| c == b'/' || c.is_ascii_whitespace() || matches!(c, b'\'' | b'"' | b',' | b';' | b')' | b']' | b'}'))
}

fn contains_path_needle(hay: &str, needle: &str) -> bool {
    let mut rest = hay;
    while let Some(idx) = rest.find(needle) {
        let after = &rest[idx + needle.len()..];
        if is_path_boundary(after.as_bytes().first().copied()) { return true; }
        rest = after;
    }
    false
}

fn collect_decoded_strings<'a>(val: &'a JsonValue, out: &mut Vec<&'a str>) {
    match val {
        JsonValue::String(s) => out.push(s.as_str()),
        JsonValue::Array(xs) => for x in xs { collect_decoded_strings(x, out); },
        JsonValue::Object(ps) => for (k, v) in ps { out.push(k.as_str()); collect_decoded_strings(v, out); },
        _ => {}
    }
}

fn collect_files_rec(dir: &Path, out: &mut BTreeSet<PathBuf>) {
    let Ok(rd) = fs::read_dir(dir) else { return; };
    for entry in rd.flatten() {
        let p = entry.path();
        if p.is_dir() { collect_files_rec(&p, out); } else if p.is_file() { out.insert(p); }
    }
}

pub fn lint_log_file(path: &Path, extra_canaries: &[String], private_root: Option<&str>) -> Result<Vec<LintFinding>, String> {
    let raw = fs::read(path).map_err(|e| format!("read {}: {e}", path.display()))?;
    let fstr = path.to_string_lossy().into_owned();
    let Ok(text) = std::str::from_utf8(&raw) else {
        let mut f = Vec::new(); push(&mut f, "schema", &fstr, 0, "log is not valid UTF-8"); return Ok(f);
    };
    let mut canaries: Vec<&str> = vec![PRIVATE_CANARY, TOKEN_CANARY];
    canaries.extend(extra_canaries.iter().map(String::as_str).filter(|s| !s.is_empty()));
    let priv_needles = private_root_needles(private_root);

    let (mut lines, mut off, mut phys_ln) = (Vec::new(), 0usize, 0usize);
    for chunk in text.split_inclusive('\n') {
        phys_ln += 1;
        let no_lf = chunk.strip_suffix('\n').unwrap_or(chunk);
        let t = no_lf.strip_suffix('\r').unwrap_or(no_lf);
        if !t.is_empty() { lines.push((phys_ln, off, t)); }
        off += chunk.len();
    }
    let mut out = Vec::new();
    if lines.is_empty() {
        push(&mut out, "missing-start", &fstr, 0, "log file is empty");
        push(&mut out, "missing-summary", &fstr, 0, "log file has no summary");
        return Ok(out);
    }

    let (mut scale, mut bead, mut last_event) = ("full".to_string(), String::new(), String::new());
    let (mut ap, mut af, mut ask, mut cc, mut cm, mut re, mut ru) = (0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut artifacts = BTreeSet::<PathBuf>::new();

    for (idx, &(ln, byte_off, raw_line)) in lines.iter().enumerate() {
        if raw_line.len() > MAX_LINE_BYTES { push(&mut out, "redaction-violation", &fstr, ln, format!("line exceeds {MAX_LINE_BYTES} bytes")); }
        let (mut seen_c, mut seen_p) = (BTreeSet::<&str>::new(), BTreeSet::<&str>::new());
        for &c in &canaries { if raw_line.contains(c) && seen_c.insert(c) { push(&mut out, "redaction-violation", &fstr, ln, format!("canary leaked: {c}")); } }
        for n in &priv_needles { if contains_path_needle(raw_line, n) && seen_p.insert(n.as_str()) { push(&mut out, "redaction-violation", &fstr, ln, format!("unredacted private root path: {n}")); } }
        let val = match json::parse_strict(raw_line) {
            Ok(v) if v.as_object().is_some() => v,
            Ok(_) => { push(&mut out, "schema", &fstr, ln, "line must be a JSON object"); continue; }
            Err(e) => { push(&mut out, "schema", &fstr, ln, e.to_string()); continue; }
        };
        if let Err(e) = json::check_redaction_tree(&val, "") { push(&mut out, "redaction-violation", &fstr, ln, e); }
        let mut decoded = Vec::new();
        collect_decoded_strings(&val, &mut decoded);
        for s in decoded {
            for &c in &canaries { if s.contains(c) && seen_c.insert(c) { push(&mut out, "redaction-violation", &fstr, ln, format!("canary leaked (JSON-escaped): {c}")); } }
            for n in &priv_needles { if contains_path_needle(s, n) && seen_p.insert(n.as_str()) { push(&mut out, "redaction-violation", &fstr, ln, format!("unredacted private root path (JSON-escaped): {n}")); } }
        }
        check_common(&val, idx as u64, &fstr, ln, &mut out);

        let ev = val.get("event").and_then(JsonValue::as_str).unwrap_or("");
        let step = val.get("step").and_then(JsonValue::as_str).unwrap_or("");
        last_event = ev.into();
        if idx == 0 {
            if ev != "start" || !step.is_empty() { push(&mut out, "missing-start", &fstr, ln, "first line must be script-level start"); }
            if let Some(s) = val.get("scale").and_then(JsonValue::as_str) { scale = s.into(); }
            if let Some(b) = val.get("bead").and_then(JsonValue::as_str) { bead = b.into(); }
        }

        match ev {
            "start" if step.is_empty() => {
                if !has_strs(&val, &["commit", "toolchain", "frob_version", "scale", "plant"])
                    || val.get("dirty").and_then(JsonValue::as_bool).is_none()
                    || !matches!(val.get("scale").and_then(JsonValue::as_str), Some("full" | "small"))
                    || val.get("devices").and_then(JsonValue::as_array).is_none()
                    || val.get("argv").and_then(JsonValue::as_array).is_none()
                    || !val.get("host").is_some_and(|h| has_strs(h, &["os", "arch", "fingerprint"]))
                {
                    push(&mut out, "schema", &fstr, ln, "script start missing required provenance fields");
                }
            }
            "start" => if val.get("title").and_then(JsonValue::as_str).is_none() { push(&mut out, "schema", &fstr, ln, "step start missing title"); },
            "cmd" => {
                if val.get("exit").and_then(JsonValue::as_i64).is_none() || val.get("wall_ms").and_then(JsonValue::as_u64).is_none() {
                    push(&mut out, "cmd-incomplete", &fstr, ln, "cmd requires integer exit and wall_ms");
                }
                if val.get("argv").and_then(JsonValue::as_array).is_none() || val.get("cmdline").and_then(JsonValue::as_str).is_none() {
                    push(&mut out, "schema", &fstr, ln, "cmd missing argv or cmdline");
                }
                for k in ["stdout_path", "stderr_path"] {
                    if let Some(p) = val.get(k).and_then(JsonValue::as_str).filter(|s| !s.is_empty()) { artifacts.insert(PathBuf::from(p)); }
                }
                if !matches!(bead.as_str(), "N38b-19" | "N38b-21") && is_self_recording(&val) {
                    push(&mut out, "self-recording", &fstr, ln, "script invokes record_gate or writes registries/gates.toml");
                }
                if is_tier_cmd(&val) && val.get("receipt").and_then(JsonValue::as_str).is_none_or(str::is_empty) {
                    push(&mut out, "tier-without-receipt", &fstr, ln, "e2e_tier cmd lacks receipt");
                }
            }
            "assert" => {
                let (kind, status) = (val.get("kind").and_then(JsonValue::as_str).unwrap_or(""), val.get("status").and_then(JsonValue::as_str).unwrap_or(""));
                if val.get("name").and_then(JsonValue::as_str).is_none() || !VALID_ASSERT_KINDS.contains(&kind) || !matches!(status, "pass" | "fail" | "skip") {
                    push(&mut out, "schema", &fstr, ln, "assert missing valid name/kind/status");
                }
                match status {
                    "pass" => ap += 1,
                    "fail" => {
                        af += 1;
                        let fd_ok = val.get("first_diff").is_some_and(|d| d.get("path").and_then(JsonValue::as_str).is_some() && d.get("observed").is_some() && d.get("expected").is_some());
                        let rp_ok = val.get("repro").is_some_and(|r| r.get("cmd_seq").and_then(JsonValue::as_u64).is_some() && r.get("artifacts").and_then(JsonValue::as_array).is_some());
                        if !fd_ok || !rp_ok { push(&mut out, "failed-assert-incomplete", &fstr, ln, "failed assert requires first_diff and repro"); }
                        if let Some(arr) = val.pointer("/repro/artifacts").and_then(JsonValue::as_array) {
                            artifacts.extend(arr.iter().filter_map(JsonValue::as_str).filter(|s| !s.is_empty()).map(PathBuf::from));
                        }
                    }
                    "skip" => ask += 1,
                    _ => {}
                }
                if kind == "negative-control" {
                    match val.get("caught").and_then(JsonValue::as_bool) {
                        Some(true) => cc += 1, Some(false) => cm += 1,
                        None => push(&mut out, "schema", &fstr, ln, "negative-control requires boolean caught"),
                    }
                }
            }
            "metric" => {
                if !has_strs(&val, &["name", "unit"]) || !matches!(val.get("value"), Some(JsonValue::Number(_) | JsonValue::String(_))) {
                    push(&mut out, "schema", &fstr, ln, "metric requires name, value, unit");
                }
                if val.get("asserted").is_some_and(|v| v.as_bool().is_none()) || val.get("verdict").is_some_and(|v| !matches!(v.as_str(), Some("met" | "not-met"))) {
                    push(&mut out, "schema", &fstr, ln, "invalid metric asserted/verdict");
                }
                if val.get("gate").and_then(JsonValue::as_str).is_some_and(|g| !g.is_empty())
                    && !val.get("threshold").is_some_and(|t| t.get("op").and_then(JsonValue::as_str).is_some() && matches!(t.get("value"), Some(JsonValue::Number(_) | JsonValue::String(_))))
                {
                    push(&mut out, "metric-no-threshold", &fstr, ln, "metric tagged with gate requires threshold");
                }
            }
            "refusal" => {
                if val.get("code").and_then(JsonValue::as_str).is_none() || val.get("exit").and_then(JsonValue::as_i64).is_none() || val.get("expected").and_then(JsonValue::as_bool).is_none() {
                    push(&mut out, "schema", &fstr, ln, "refusal requires code, exit, expected");
                }
                if val.get("expected").and_then(JsonValue::as_bool) == Some(true) { re += 1; } else { ru += 1; }
            }
            "end" => if ["wall_ms", "asserts_passed", "asserts_failed"].iter().any(|k| val.get(k).and_then(JsonValue::as_u64).is_none()) {
                push(&mut out, "schema", &fstr, ln, "end requires wall_ms, asserts_passed, asserts_failed");
            },
            "summary" => {
                if idx + 1 != lines.len() { push(&mut out, "schema", &fstr, ln, "summary must be the final line"); }
                let exp_sha = sha256_hex(&raw[..byte_off]);
                if val.get("body_sha256").and_then(JsonValue::as_str) != Some(exp_sha.as_str()) {
                    push(&mut out, "body-sha256-mismatch", &fstr, ln, format!("body_sha256 mismatch (expected {exp_sha})"));
                }
                for (field, actual) in [("asserts_passed", ap), ("asserts_failed", af), ("asserts_skipped", ask), ("controls_caught", cc), ("controls_missed", cm), ("refusals_expected", re), ("refusals_unexpected", ru)] {
                    if val.get(field).and_then(JsonValue::as_u64) != Some(actual) {
                        push(&mut out, "summary-recount", &fstr, ln, format!("summary '{field}' disagrees with recount ({actual})"));
                    }
                }
                let claimable = val.get("gates_claimable").and_then(JsonValue::as_bool).unwrap_or(false);
                let reg = val.get("registries").and_then(JsonValue::as_str).unwrap_or("");
                if !matches!(reg, "of-record" | "override") { push(&mut out, "schema", &fstr, ln, "invalid summary registries"); }
                if claimable && scale == "small" { push(&mut out, "small-claims-gate", &fstr, ln, "log claims gates under scale=small"); }
                if claimable && reg == "override" { push(&mut out, "scratch-registries", &fstr, ln, "log claims gates under registries=override"); }
                match val.get("gates").and_then(JsonValue::as_array) {
                    Some(gates) => for g in gates {
                        let (gid, mok, gcc) = (g.get("id").and_then(JsonValue::as_str).filter(|s| !s.is_empty()), g.get("measured_ok").and_then(JsonValue::as_bool), g.get("controls_caught").and_then(JsonValue::as_u64));
                        if gid.is_none() || mok.is_none() || gcc.is_none() { push(&mut out, "schema", &fstr, ln, "summary gate requires id, measured_ok, controls_caught"); }
                        if mok == Some(true) && gcc.unwrap_or(0) == 0 { push(&mut out, "no-control", &fstr, ln, format!("gate '{}' has measured_ok=true with controls_caught=0", gid.unwrap_or("?"))); }
                    },
                    None => push(&mut out, "schema", &fstr, ln, "summary missing gates array"),
                }
            }
            _ => {}
        }
    }

    if last_event != "summary" { push(&mut out, "missing-summary", &fstr, lines.last().map_or(0, |l| l.0), "last line must be summary"); }
    let mut transcript_path = None;
    if let Some(stem) = fstr.strip_suffix(".ndjson") {
        let tr = PathBuf::from(format!("{stem}.transcript.txt"));
        if tr.is_file() { transcript_path = Some(tr.clone()); artifacts.insert(tr); }
        let adir = PathBuf::from(format!("{stem}.artifacts"));
        if adir.is_dir() { collect_files_rec(&adir, &mut artifacts); }
    }
    for art in artifacts {
        if let Ok(bytes) = fs::read(&art) {
            for &c in &canaries {
                if bytes.windows(c.len()).any(|w| w == c.as_bytes()) {
                    push(&mut out, "redaction-violation", &art.to_string_lossy(), 0, format!("canary leaked in artifact: {c}"));
                }
            }
            if transcript_path.as_ref() == Some(&art) && let Ok(tr_text) = std::str::from_utf8(&bytes) {
                for n in &priv_needles {
                    if contains_path_needle(tr_text, n) {
                        push(&mut out, "redaction-violation", &art.to_string_lossy(), 0, format!("unredacted private root path in transcript: {n}"));
                    }
                }
            }
        }
    }
    Ok(out)
}

fn check_common(val: &JsonValue, exp_seq: u64, file: &str, ln: usize, out: &mut Vec<LintFinding>) {
    if val.get("schema").and_then(JsonValue::as_str) != Some(LOG_SCHEMA_VERSION) { push(out, "schema", file, ln, "wrong schema"); }
    if val.get("ts").and_then(JsonValue::as_str).and_then(parse_utc_rfc3339_ms).is_none() { push(out, "schema", file, ln, "invalid ts"); }
    if val.get("seq").and_then(JsonValue::as_u64) != Some(exp_seq) { push(out, "seq-gap", file, ln, format!("expected seq={exp_seq}")); }
    for r in ["run_id", "script", "bead"] {
        if val.get(r).and_then(JsonValue::as_str).is_none_or(str::is_empty) { push(out, "schema", file, ln, format!("missing '{r}'")); }
    }
    if val.get("step").and_then(JsonValue::as_str).is_none() { push(out, "schema", file, ln, "missing step"); }
    if val.get("event").and_then(JsonValue::as_str).is_none_or(|e| !VALID_EVENTS.contains(&e)) { push(out, "schema", file, ln, "invalid event"); }
    if val.get("seed").and_then(JsonValue::as_str).is_none_or(|s| s.is_empty() || !s.bytes().all(|b| b.is_ascii_digit())) {
        push(out, "schema", file, ln, "seed must be decimal string");
    }
}

fn is_self_recording(val: &JsonValue) -> bool {
    let cmdline = val.get("cmdline").and_then(JsonValue::as_str).unwrap_or("");
    let argv: Vec<&str> = val.get("argv").and_then(JsonValue::as_array).map(|a| a.iter().filter_map(JsonValue::as_str).collect()).unwrap_or_default();
    if argv.iter().any(|a| *a == "record_gate" || a.ends_with("/record_gate")) || cmdline.contains("record_gate") { return true; }
    if cmdline.contains("registries/gates.toml") || argv.iter().any(|a| a.contains("registries/gates.toml")) {
        if cmdline.contains('>') { return true; }
        let writers = ["tee", "cp", "mv", "install", "sed", "perl", "patch", "truncate", "dd"];
        if argv.first().is_some_and(|c| writers.contains(&c.rsplit('/').next().unwrap_or(c))) { return true; }
    }
    false
}

fn is_tier_cmd(val: &JsonValue) -> bool {
    let argv: Vec<&str> = val.get("argv").and_then(JsonValue::as_array).map(|a| a.iter().filter_map(JsonValue::as_str).collect()).unwrap_or_default();
    argv.iter().any(|a| a.ends_with("cargo")) && argv.contains(&"test") && argv.contains(&"frob-conformance")
        && argv.windows(2).any(|w| w[0] == "--test" && VALID_TIERS.contains(&w[1]))
}

pub fn cmd_lint(args: &[String]) -> i32 {
    let f = match ParsedFlags::parse(args, &["--private-root"], &["--canary"], &["--json"]) {
        Ok(f) if !f.positionals.is_empty() => f,
        Ok(_) => { eprintln!("usage: frob-e2e lint [--canary <str>] [--private-root <dir>] [--json] <log>..."); return 2; }
        Err(e) => { eprintln!("frob-e2e lint: {e}"); return 2; }
    };
    let pr = effective_private_root(f.get("--private-root"));
    let mut all = Vec::new();
    for p in &f.positionals {
        match lint_log_file(Path::new(p), f.list("--canary"), pr.as_deref()) {
            Ok(mut res) => all.append(&mut res),
            Err(e) => { eprintln!("frob-e2e lint: {e}"); return 2; }
        }
    }
    if f.has("--json") {
        println!("{}", obj(vec![("ok", JsonValue::Bool(all.is_empty())), ("findings", JsonValue::Array(all.iter().map(LintFinding::to_json).collect()))]).to_json_string());
    } else {
        for x in &all { eprintln!("{}:{}: [{}] {}", x.file, x.line, x.rule, x.detail); }
    }
    if all.is_empty() { 0 } else { 1 }
}
