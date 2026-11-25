#![deny(unsafe_code)]

//! Declared-binaries, recorder-isolation, manifest-layer, and `gauntlet-mutations`
//! umbrella check (`BD-025`).
//!
//! Run as `cargo test -p frob-conformance --test binaries_declared`.
//! Supports `FROB_CONFORMANCE_ROOT=<dir>` for scratch workspaces.
//! Never deletes files or directories (COMMON.md 9b).

use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use frob_core::testkit::registry::{self, Controls};

static SCRATCH_SEQ: AtomicU64 = AtomicU64::new(0);

const FORBIDDEN_RECORDER_CALLS: &[&str] = &[
    "register::record",
    "register::write",
    "recorder::reaffirm",
];

const REQUIRED_CONTRACT_SECTIONS: &[&str] = &[
    "purpose",
    "inputs",
    "outputs",
    "invariants",
    "refusals",
    "determinism class",
    "evidence class of outputs",
    "validity domain",
    "oracles",
    "known gaps",
];

#[derive(Debug, Clone, PartialEq, Eq)]
struct BinCheckFinding {
    file: String,
    entry: String,
    detail: String,
}

impl fmt::Display for BinCheckFinding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: [{}] {}", self.file, self.entry, self.detail)
    }
}

#[derive(Debug, Clone, Default)]
struct BinDecl {
    name: String,
    path: String,
}

#[derive(Debug, Clone, Default)]
struct ParsedManifest {
    name: Option<String>,
    autobins: Option<bool>,
    default_run: Option<String>,
    layer: Option<i64>,
    bins: Vec<BinDecl>,
    features: BTreeMap<String, Vec<String>>,
    deps: BTreeSet<String>,
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

fn parse_string_array(raw: &str) -> Vec<String> {
    let mut out = Vec::new();
    let bytes = raw.as_bytes();
    let mut i = 0usize;
    while i < bytes.len() {
        if bytes[i] == b'"' || bytes[i] == b'\'' {
            let q = bytes[i];
            i += 1;
            let start = i;
            while i < bytes.len() && bytes[i] != q {
                if bytes[i] == b'\\' && q == b'"' && i + 1 < bytes.len() {
                    i += 2;
                    continue;
                }
                i += 1;
            }
            out.push(raw[start..i].to_owned());
            if i < bytes.len() {
                i += 1;
            }
            continue;
        }
        i += 1;
    }
    out
}

fn parse_manifest(content: &str) -> ParsedManifest {
    let mut m = ParsedManifest::default();
    let mut section = String::new();
    let mut cur_bin: Option<BinDecl> = None;
    let mut open_feature: Option<(String, String)> = None;

    let flush_bin = |bins: &mut Vec<BinDecl>, cur: &mut Option<BinDecl>| {
        if let Some(b) = cur.take() {
            bins.push(b);
        }
    };
    let flush_feature = |feats: &mut BTreeMap<String, Vec<String>>, open: &mut Option<(String, String)>| {
        if let Some((k, buf)) = open.take() {
            feats.insert(k, parse_string_array(&buf));
        }
    };

    for raw_line in content.lines() {
        let line = strip_toml_comment(raw_line).trim();
        if line.is_empty() {
            continue;
        }
        if let Some((_, ref mut buf)) = open_feature {
            buf.push(' ');
            buf.push_str(line);
            if line.contains(']') {
                flush_feature(&mut m.features, &mut open_feature);
            }
            continue;
        }
        if line.starts_with('[') && line.ends_with(']') {
            flush_bin(&mut m.bins, &mut cur_bin);
            section = line.to_owned();
            if section == "[[bin]]" {
                cur_bin = Some(BinDecl::default());
            }
            continue;
        }
        let Some((k_raw, v_raw)) = line.split_once('=') else {
            continue;
        };
        let key = k_raw.trim().trim_matches('"').trim_matches('\'');
        let val = v_raw.trim();

        match section.as_str() {
            "[package]" => match key {
                "name" => m.name = Some(val.trim_matches('"').trim_matches('\'').to_owned()),
                "autobins" => m.autobins = val.parse::<bool>().ok(),
                "default-run" => m.default_run = Some(val.trim_matches('"').trim_matches('\'').to_owned()),
                _ => {}
            },
            "[package.metadata.frob]" => {
                if key == "layer" {
                    m.layer = val.parse::<i64>().ok();
                }
            }
            "[[bin]]" => {
                if let Some(ref mut b) = cur_bin {
                    let s = val.trim_matches('"').trim_matches('\'').to_owned();
                    if key == "name" {
                        b.name = s;
                    } else if key == "path" {
                        b.path = s;
                    }
                }
            }
            "[features]" => {
                if val.contains('[') && !val.contains(']') {
                    open_feature = Some((key.to_owned(), val.to_owned()));
                } else {
                    m.features.insert(key.to_owned(), parse_string_array(val));
                }
            }
            "[dependencies]" | "[dev-dependencies]" | "[build-dependencies]" => {
                m.deps.insert(key.to_owned());
                if val.contains("package") && val.contains("frob-conformance") {
                    m.deps.insert("frob-conformance".to_owned());
                }
            }
            _ => {
                if section.contains("dependencies") && section.contains("frob-conformance") {
                    m.deps.insert("frob-conformance".to_owned());
                }
            }
        }
    }
    flush_bin(&mut m.bins, &mut cur_bin);
    flush_feature(&mut m.features, &mut open_feature);
    m
}

/// Strip Rust line comments, block comments, and string literals (replacing strings with `""`)
/// so prose comments naming `register::record` are never mistaken for code calls.
fn strip_rust_comments_and_strings(src: &str) -> String {
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
        let ch = src[i..].chars().next().expect("valid utf8");
        out.push(ch);
        i += ch.len_utf8();
    }
    out
}

fn parse_contract_binaries(contract_md: &str) -> BTreeMap<String, (String, String)> {
    // Returns map: binary_name -> (path, owning_bead)
    let mut out = BTreeMap::new();
    let mut in_binaries = false;
    for raw in contract_md.lines() {
        let line = raw.trim();
        if line.starts_with("### Binaries") {
            in_binaries = true;
            continue;
        }
        if in_binaries && (line.starts_with("## ") || line.starts_with("### ")) {
            break;
        }
        if !in_binaries || !line.starts_with('|') {
            continue;
        }
        let cells: Vec<&str> = line
            .trim_matches('|')
            .split('|')
            .map(|c| c.trim().trim_matches('`'))
            .collect();
        if cells.len() < 3 {
            continue;
        }
        if cells[0].eq_ignore_ascii_case("binary") || cells[0].chars().all(|c| c == '-' || c == ':') {
            continue;
        }
        if !cells[0].is_empty() {
            out.insert(
                cells[0].to_owned(),
                (cells[1].to_owned(), cells[2].to_owned()),
            );
        }
    }
    out
}

fn walk_rs_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(rd) = fs::read_dir(dir) else { return };
    let mut entries: Vec<PathBuf> = rd.flatten().map(|e| e.path()).collect();
    entries.sort();
    for p in entries {
        if p.is_dir() {
            walk_rs_files(&p, out);
        } else if p.extension().and_then(|s| s.to_str()) == Some("rs") {
            out.push(p);
        }
    }
}

fn check_binaries_and_umbrella(root: &Path) -> (ParsedManifest, BTreeMap<String, (String, String)>, Vec<BinCheckFinding>) {
    let mut findings = Vec::new();
    let crate_dir = root.join("crates/frob-conformance");
    let manifest_rel = "crates/frob-conformance/Cargo.toml";
    let contract_rel = "crates/frob-conformance/CONTRACT.md";

    let manifest_text = fs::read_to_string(crate_dir.join("Cargo.toml"))
        .unwrap_or_else(|e| panic!("failed to read {manifest_rel}: {e}"));
    let manifest = parse_manifest(&manifest_text);

    // 1. autobins = false, no default-run, layer = 7.
    if manifest.autobins != Some(false) {
        findings.push(BinCheckFinding {
            file: manifest_rel.to_owned(),
            entry: "autobins".to_owned(),
            detail: "manifest must set `autobins = false`".to_owned(),
        });
    }
    if let Some(dr) = &manifest.default_run {
        findings.push(BinCheckFinding {
            file: manifest_rel.to_owned(),
            entry: "default-run".to_owned(),
            detail: format!("manifest must not set `default-run` (found `{dr}`)"),
        });
    }
    if manifest.layer != Some(7) {
        findings.push(BinCheckFinding {
            file: manifest_rel.to_owned(),
            entry: "package.metadata.frob.layer".to_owned(),
            detail: format!("frob-conformance must declare layer = 7 (found {:?})", manifest.layer),
        });
    }

    // 2. Declared [[bin]] checks + CONTRACT.md table & ten sections.
    let contract_text = fs::read_to_string(crate_dir.join("CONTRACT.md"))
        .unwrap_or_else(|e| panic!("failed to read {contract_rel}: {e}"));
    let contract_bins = parse_contract_binaries(&contract_text);

    let contract_lower = contract_text.to_ascii_lowercase();
    for sec in REQUIRED_CONTRACT_SECTIONS {
        let found = contract_lower
            .lines()
            .any(|l| l.trim_start().starts_with("## ") && l.contains(sec));
        if !found {
            findings.push(BinCheckFinding {
                file: contract_rel.to_owned(),
                entry: (*sec).to_owned(),
                detail: format!("CONTRACT.md is missing required section `{sec}`"),
            });
        }
    }
    if !contract_lower.contains("frob conformance") || !contract_lower.contains("frob-conformance") {
        findings.push(BinCheckFinding {
            file: contract_rel.to_owned(),
            entry: "ADR-GAUNT-01".to_owned(),
            detail: "CONTRACT.md must state that no `frob conformance` command and no `frob-conformance` binary exist".to_owned(),
        });
    }

    let mut declared_bin_paths = BTreeSet::new();
    for b in &manifest.bins {
        if b.name == "frob-conformance" {
            findings.push(BinCheckFinding {
                file: manifest_rel.to_owned(),
                entry: "frob-conformance".to_owned(),
                detail: "no binary may be named `frob-conformance` (ADR-GAUNT-01)".to_owned(),
            });
        }
        if b.path.is_empty() {
            findings.push(BinCheckFinding {
                file: manifest_rel.to_owned(),
                entry: b.name.clone(),
                detail: "[[bin]] entry must specify an explicit `path`".to_owned(),
            });
        } else {
            declared_bin_paths.insert(b.path.replace('\\', "/"));
            if !crate_dir.join(&b.path).is_file() {
                findings.push(BinCheckFinding {
                    file: manifest_rel.to_owned(),
                    entry: b.name.clone(),
                    detail: format!("declared [[bin]] path `{}` does not exist", b.path),
                });
            }
        }
        match contract_bins.get(&b.name) {
            None => findings.push(BinCheckFinding {
                file: contract_rel.to_owned(),
                entry: b.name.clone(),
                detail: format!("declared binary `{}` has no row in CONTRACT.md Binaries table", b.name),
            }),
            Some((c_path, bead)) => {
                if c_path != &b.path || bead.is_empty() {
                    findings.push(BinCheckFinding {
                        file: contract_rel.to_owned(),
                        entry: b.name.clone(),
                        detail: format!(
                            "CONTRACT.md row for `{}` must match path `{}` and name an owning bead",
                            b.name, b.path
                        ),
                    });
                }
            }
        }
    }

    // Every file under src/bin/ must have a [[bin]] entry.
    let mut bin_files = Vec::new();
    let src_bin_dir = crate_dir.join("src/bin");
    if src_bin_dir.is_dir() {
        walk_rs_files(&src_bin_dir, &mut bin_files);
    }
    for f in &bin_files {
        let rel_in_crate = f
            .strip_prefix(&crate_dir)
            .unwrap_or(f)
            .to_string_lossy()
            .replace('\\', "/");
        if !declared_bin_paths.contains(&rel_in_crate) {
            let stem = f.file_stem().and_then(|s| s.to_str()).unwrap_or(&rel_in_crate);
            findings.push(BinCheckFinding {
                file: format!("crates/frob-conformance/{rel_in_crate}"),
                entry: stem.to_owned(),
                detail: format!("undeclared binary file `{rel_in_crate}` missing [[bin]] entry in Cargo.toml"),
            });
        }
    }

    // 3. Recorder write isolation: no file under src/bin/ or examples/ other than
    // examples/record_gate.rs may call register::record, register::write, or recorder::reaffirm.
    let mut scan_files = bin_files;
    let examples_dir = crate_dir.join("examples");
    if examples_dir.is_dir() {
        walk_rs_files(&examples_dir, &mut scan_files);
    }
    for f in scan_files {
        let rel_in_crate = f
            .strip_prefix(&crate_dir)
            .unwrap_or(&f)
            .to_string_lossy()
            .replace('\\', "/");
        if rel_in_crate == "examples/record_gate.rs" {
            continue;
        }
        let Ok(src) = fs::read_to_string(&f) else {
            continue;
        };
        let code = strip_rust_comments_and_strings(&src);
        for call in FORBIDDEN_RECORDER_CALLS {
            if code.contains(call) {
                findings.push(BinCheckFinding {
                    file: format!("crates/frob-conformance/{rel_in_crate}"),
                    entry: (*call).to_owned(),
                    detail: format!(
                        "`{rel_in_crate}` calls `{call}`; only `examples/record_gate.rs` may write gate rows"
                    ),
                });
            }
        }
    }

    // 4. No workspace crate depends on frob-conformance; every crate declaring a `conformance`
    // feature is listed in `gauntlet-mutations`; no `nc-*` or control `feature` is in `gauntlet-mutations`.
    let umbrella: BTreeSet<String> = manifest
        .features
        .get("gauntlet-mutations")
        .cloned()
        .unwrap_or_default()
        .into_iter()
        .collect();
    if manifest
        .features
        .get("default")
        .is_some_and(|d| d.iter().any(|x| x == "gauntlet-mutations"))
    {
        findings.push(BinCheckFinding {
            file: manifest_rel.to_owned(),
            entry: "gauntlet-mutations".to_owned(),
            detail: "`default` feature must not enable `gauntlet-mutations`".to_owned(),
        });
    }

    let crates_dir = root.join("crates");
    if let Ok(rd) = fs::read_dir(&crates_dir) {
        let mut dirs: Vec<_> = rd.flatten().map(|e| e.path()).collect();
        dirs.sort();
        for dir in dirs {
            let mpath = dir.join("Cargo.toml");
            if !mpath.is_file() {
                continue;
            }
            let Ok(txt) = fs::read_to_string(&mpath) else {
                continue;
            };
            let pm = parse_manifest(&txt);
            let pkg_name = pm.name.clone().unwrap_or_else(|| {
                dir.file_name()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unknown")
                    .to_owned()
            });
            if pkg_name == "frob-conformance" {
                continue;
            }
            if pm.deps.contains("frob-conformance") {
                findings.push(BinCheckFinding {
                    file: format!(
                        "crates/{}/Cargo.toml",
                        dir.file_name().and_then(|s| s.to_str()).unwrap_or(&pkg_name)
                    ),
                    entry: "frob-conformance".to_owned(),
                    detail: format!("crate `{pkg_name}` must not depend on layer-7 `frob-conformance`"),
                });
            }
            if pm.features.contains_key("conformance") {
                let expected_entry = format!("{pkg_name}/conformance");
                if !umbrella.contains(&expected_entry) {
                    findings.push(BinCheckFinding {
                        file: manifest_rel.to_owned(),
                        entry: expected_entry.clone(),
                        detail: format!(
                            "crate `{pkg_name}` declares a `conformance` feature missing from `gauntlet-mutations` (expected `{expected_entry}`)"
                        ),
                    });
                }
            }
        }
    }

    // Collect all control `feature` values from registry/controls.toml.
    let mut control_features: BTreeSet<String> = BTreeSet::new();
    if let Ok((ctrls, _)) = Controls::load(root) {
        for c in ctrls.all() {
            if let Some(feat) = &c.feature {
                let trimmed = feat.trim();
                if !trimmed.is_empty() {
                    control_features.insert(trimmed.to_owned());
                    if let Some((_, bare)) = trimmed.split_once('/') {
                        control_features.insert(bare.to_owned());
                    }
                }
            }
        }
    }

    for entry in &umbrella {
        let bare = entry.split_once('/').map_or(entry.as_str(), |(_, f)| f);
        if bare.starts_with("nc-") {
            findings.push(BinCheckFinding {
                file: manifest_rel.to_owned(),
                entry: entry.clone(),
                detail: format!(
                    "`gauntlet-mutations` must never list per-control mutation feature `{entry}`"
                ),
            });
        } else if control_features.contains(entry) || control_features.contains(bare) {
            findings.push(BinCheckFinding {
                file: manifest_rel.to_owned(),
                entry: entry.clone(),
                detail: format!(
                    "`gauntlet-mutations` must never list mutation feature `{entry}` named by a control record"
                ),
            });
        }
    }

    // 5. No entry of registries/verbs.toml belongs to frob-conformance.
    let verbs_path = root.join("registries/verbs.toml");
    if verbs_path.is_file()
        && let Ok(vtxt) = fs::read_to_string(&verbs_path)
    {
        for raw in vtxt.lines() {
            let line = strip_toml_comment(raw).trim();
            if line.contains("\"frob-conformance\"")
                || line.contains("cluster = \"conformance\"")
            {
                findings.push(BinCheckFinding {
                    file: "registries/verbs.toml".to_owned(),
                    entry: "frob-conformance".to_owned(),
                    detail: "no verb registry entry may belong to `frob-conformance` or cluster `conformance`".to_owned(),
                });
            }
        }
    }

    (manifest, contract_bins, findings)
}

fn workspace_root() -> PathBuf {
    if let Some(ov) = env::var_os("FROB_CONFORMANCE_ROOT") {
        return PathBuf::from(ov);
    }
    registry::root().expect("locate workspace root via franken-stack.lock")
}

fn make_clean_scratch_ws(label: &str) -> PathBuf {
    let base = env::var_os("BIG_SCRATCH")
        .or_else(|| env::var_os("TMPDIR"))
        .map(PathBuf::from)
        .unwrap_or_else(env::temp_dir);
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let seq = SCRATCH_SEQ.fetch_add(1, Ordering::Relaxed);
    let ws = base
        .join("frob-BronzePelican")
        .join(format!("binaries_declared-{label}-{nanos}-{seq}"));
    let cdir = ws.join("crates/frob-conformance");
    fs::create_dir_all(cdir.join("src/bin")).expect("mkdir src/bin");
    fs::create_dir_all(cdir.join("examples")).expect("mkdir examples");
    fs::create_dir_all(cdir.join("registry")).expect("mkdir registry");
    fs::write(cdir.join("src/bin/frob-e2e.rs"), "fn main() {}\n").expect("write frob-e2e.rs");
    fs::write(
        cdir.join("registry/controls.toml"),
        "schema = \"frob.gauntlet.controls/1\"\n",
    )
    .expect("write controls.toml");
    fs::write(
        cdir.join("Cargo.toml"),
        r#"[package]
name = "frob-conformance"
version = "0.0.1"
edition = "2024"
autobins = false

[package.metadata.frob]
layer = 7
rank = 4

[[bin]]
name = "frob-e2e"
path = "src/bin/frob-e2e.rs"

[features]
default = []
gauntlet-mutations = []
"#,
    )
    .expect("write Cargo.toml");
    fs::write(
        cdir.join("CONTRACT.md"),
        r#"# `frob-conformance` Contract

## 1. Purpose
No `frob conformance` command and no `frob-conformance` binary exist.
## 2. Inputs
Registries.
## 3. Outputs
Reports.

### Binaries

| Binary | Path | Owning bead | Reads | Writes | Registries |
|---|---|---|---|---|---|
| `frob-e2e` | `src/bin/frob-e2e.rs` | `STD-E2E` | logs | logs | none |

## 4. Invariants
Closed.
## 5. Refusals
Typed.
## 6. Determinism Class
D0.
## 7. Evidence Class of Outputs
Tested.
## 8. Validity Domain
Host.
## 9. Oracles
Controls.
## 10. Known Gaps
v0.
"#,
    )
    .expect("write CONTRACT.md");
    ws
}

#[test]
fn binaries_declared() {
    // 1. Real repository check: passes and finds `frob-e2e` entry + CONTRACT.md row.
    let root = workspace_root();
    let (manifest, contract_bins, findings) = check_binaries_and_umbrella(&root);
    assert!(
        findings.is_empty(),
        "binaries_declared failed on {}:\n{}",
        root.display(),
        findings
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join("\n")
    );
    assert!(
        manifest
            .bins
            .iter()
            .any(|b| b.name == "frob-e2e" && b.path == "src/bin/frob-e2e.rs"),
        "expected [[bin]] frob-e2e in Cargo.toml"
    );
    assert_eq!(
        contract_bins.get("frob-e2e"),
        Some(&("src/bin/frob-e2e.rs".to_owned(), "STD-E2E".to_owned())),
        "expected frob-e2e row in CONTRACT.md"
    );

    // 2. Plant 1: undeclared src/bin/extra.rs
    let ws1 = make_clean_scratch_ws("undeclared_bin");
    fs::write(ws1.join("crates/frob-conformance/src/bin/extra.rs"), "fn main() {}\n")
        .expect("write extra.rs");
    let (_, _, f1) = check_binaries_and_umbrella(&ws1);
    assert!(
        f1.iter().any(|f| f.file.contains("src/bin/extra.rs") && f.entry == "extra"),
        "expected undeclared src/bin/extra.rs failure, got: {f1:?}"
    );

    // 3. Plant 2: [[bin]] name = "frob-conformance"
    let ws2 = make_clean_scratch_ws("bad_bin_name");
    fs::write(
        ws2.join("crates/frob-conformance/src/bin/frob-conformance.rs"),
        "fn main() {}\n",
    )
    .expect("write bad bin");
    let mut m2 = fs::read_to_string(ws2.join("crates/frob-conformance/Cargo.toml")).unwrap();
    m2.push_str("\n[[bin]]\nname = \"frob-conformance\"\npath = \"src/bin/frob-conformance.rs\"\n");
    fs::write(ws2.join("crates/frob-conformance/Cargo.toml"), m2).unwrap();
    let (_, _, f2) = check_binaries_and_umbrella(&ws2);
    assert!(
        f2.iter()
            .any(|f| f.file.contains("Cargo.toml") && f.entry == "frob-conformance"),
        "expected frob-conformance bin name failure, got: {f2:?}"
    );

    // 4. Plant 3: default-run key
    let ws3 = make_clean_scratch_ws("default_run");
    let m3 = fs::read_to_string(ws3.join("crates/frob-conformance/Cargo.toml"))
        .unwrap()
        .replace("autobins = false", "autobins = false\ndefault-run = \"frob-e2e\"");
    fs::write(ws3.join("crates/frob-conformance/Cargo.toml"), m3).unwrap();
    let (_, _, f3) = check_binaries_and_umbrella(&ws3);
    assert!(
        f3.iter()
            .any(|f| f.file.contains("Cargo.toml") && f.entry == "default-run"),
        "expected default-run failure, got: {f3:?}"
    );

    // 5. Plant 4: scratch example calling register::record (while examples/record_gate.rs is allowed)
    let ws4 = make_clean_scratch_ws("rogue_recorder");
    fs::write(
        ws4.join("crates/frob-conformance/examples/record_gate.rs"),
        "fn main() { let _ = register::record; }\n",
    )
    .unwrap();
    let (_, _, f4_ok) = check_binaries_and_umbrella(&ws4);
    assert!(f4_ok.is_empty(), "examples/record_gate.rs must be permitted: {f4_ok:?}");
    fs::write(
        ws4.join("crates/frob-conformance/examples/sneaky.rs"),
        "fn main() { register::record(); }\n",
    )
    .unwrap();
    let (_, _, f4) = check_binaries_and_umbrella(&ws4);
    assert!(
        f4.iter()
            .any(|f| f.file.contains("examples/sneaky.rs") && f.entry == "register::record"),
        "expected sneaky.rs register::record failure, got: {f4:?}"
    );

    // 6. Plant 5: crate declaring `conformance` feature missing from gauntlet-mutations
    let ws5 = make_clean_scratch_ws("missing_conformance_feat");
    fs::create_dir_all(ws5.join("crates/frob-dummy/src")).unwrap();
    fs::write(
        ws5.join("crates/frob-dummy/Cargo.toml"),
        "[package]\nname = \"frob-dummy\"\nversion = \"0.0.1\"\nedition = \"2024\"\n\n[features]\nconformance = []\n",
    )
    .unwrap();
    let (_, _, f5) = check_binaries_and_umbrella(&ws5);
    assert!(
        f5.iter()
            .any(|f| f.file.contains("Cargo.toml") && f.entry == "frob-dummy/conformance"),
        "expected missing frob-dummy/conformance failure, got: {f5:?}"
    );

    // 7. Plant 6: `nc-0412` listed in gauntlet-mutations
    let ws6 = make_clean_scratch_ws("nc_in_umbrella");
    let m6 = fs::read_to_string(ws6.join("crates/frob-conformance/Cargo.toml"))
        .unwrap()
        .replace("gauntlet-mutations = []", "gauntlet-mutations = [\"frob-actuate/nc-0412\"]");
    fs::write(ws6.join("crates/frob-conformance/Cargo.toml"), m6).unwrap();
    let (_, _, f6) = check_binaries_and_umbrella(&ws6);
    assert!(
        f6.iter()
            .any(|f| f.file.contains("Cargo.toml") && f.entry.contains("nc-0412")),
        "expected nc-0412 in umbrella failure, got: {f6:?}"
    );

    // 8. Plant 7: control's `feature = "cx-262"` listed in gauntlet-mutations
    let ws7 = make_clean_scratch_ws("cx_in_umbrella");
    fs::write(
        ws7.join("crates/frob-conformance/registry/controls.toml"),
        r#"schema = "frob.gauntlet.controls/1"

[[control]]
id = "NC-0412"
seed = "CX-262"
gate = ["GATE-ARENA-04"]
kind = "mutation"
target = "frob-actuate::stage::integrate"
defect = "explicit Euler"
expect = "wheel-loop row reports spectral radius 1.497 where 0.9207 is true; the row fails"
feature = "cx-262"
due = "M1"
"#,
    )
    .unwrap();
    let m7 = fs::read_to_string(ws7.join("crates/frob-conformance/Cargo.toml"))
        .unwrap()
        .replace("gauntlet-mutations = []", "gauntlet-mutations = [\"frob-actuate/cx-262\"]");
    fs::write(ws7.join("crates/frob-conformance/Cargo.toml"), m7).unwrap();
    let (_, _, f7) = check_binaries_and_umbrella(&ws7);
    assert!(
        f7.iter()
            .any(|f| f.file.contains("Cargo.toml") && f.entry.contains("cx-262")),
        "expected cx-262 in umbrella failure, got: {f7:?}"
    );
}
