//! FrankenSim build-identity verification helpers and no-default source lint
//! (`N33a-08`, §33.2, §33.5, §33.8, §34.1).
//!
//! Enforces:
//! - Root `.cargo/config.toml` `[env]` holds the same `FRANKENSIM_DEPGRAPH_SALT`
//!   string recorded on the `FD-FSIM` row of `franken-stack.lock`.
//! - No build script, Rust source file, `.cargo/config.toml`, or shell script
//!   in this repository supplies a conditional default for
//!   `FRANKENSIM_DEPGRAPH_SALT` or `FRANKENSIM_DEPGRAPH_RECEIPT`.
//!
//! Pure over its filesystem inputs (determinism class D0), read-only on every
//! repository and sibling directory, prints nothing (rule L-7), zero `unsafe`.

use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

use frob_core::registry::toml::{self, Value as TomlValue};
use frob_core::stack::build_identity::{ENV_RECEIPT, ENV_SALT};

use crate::stack::{lock, receipt};

/// Default project-owned development equivalence-class salt (`N33a-08` Mode 1).
pub const EXPECTED_DEV_SALT: &str = "franken-robotics-dev-v1";

/// Build-identity variables governed by the no-default rule.
pub const BUILD_IDENTITY_VARS: [&str; 2] = [ENV_SALT, ENV_RECEIPT];

/// Directories skipped while walking a repository tree for the no-default lint.
const SKIP_DIRS: &[&str] = &[
    ".beads",
    ".git",
    ".ntm",
    ".rch-tmp",
    "node_modules",
    "planning",
    "receipts",
    "target",
];

/// Classification of a no-default source lint violation (`N33a-08`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NoDefaultKind {
    /// Shell conditional expansion (`${VAR:-...}`, `${VAR-...}`, `${VAR:=...}`, `${VAR=...}`).
    ShellDefault,
    /// Rust `.unwrap_or(...)`, `.unwrap_or_else(...)`, or `.unwrap_or_default()` on a build-identity env lookup.
    RustUnwrapOr,
    /// Rust `option_env!(...)` chained with a fallback value.
    OptionEnvFallback,
    /// `.cargo/config.toml` statically sets `FRANKENSIM_DEPGRAPH_RECEIPT` (Mode 2 is export-only).
    StaticReceiptInConfig,
    /// `.cargo/config.toml` sets a build-identity variable to an empty string.
    EmptyEnvInConfig,
}

impl NoDefaultKind {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ShellDefault => "shell-default",
            Self::RustUnwrapOr => "rust-unwrap-or",
            Self::OptionEnvFallback => "option-env-fallback",
            Self::StaticReceiptInConfig => "static-receipt-in-config",
            Self::EmptyEnvInConfig => "empty-env-in-config",
        }
    }
}

impl fmt::Display for NoDefaultKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// One source-level violation reported by [`lint_no_default_in_repo`].
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct NoDefaultFinding {
    pub path: String,
    pub line: usize,
    pub kind: NoDefaultKind,
    pub variable: &'static str,
    pub message: String,
}

impl fmt::Display for NoDefaultFinding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{}: [{}] {}: {}",
            self.path, self.line, self.kind, self.variable, self.message
        )
    }
}

/// Verify that `<repo_root>/.cargo/config.toml` `[env].FRANKENSIM_DEPGRAPH_SALT`
/// equals the `FD-FSIM` `build_env["FRANKENSIM_DEPGRAPH_SALT"]` entry in
/// `<repo_root>/franken-stack.lock`. Returns the shared salt on success.
pub fn salt_matches_lock(repo_root: &Path) -> Result<String, String> {
    let env_map = receipt::parse_build_env(repo_root)
        .map_err(|e| format!("failed reading .cargo/config.toml: {e}"))?;
    let cfg_salt = env_map
        .get(ENV_SALT)
        .filter(|s| !s.is_empty())
        .ok_or_else(|| format!(".cargo/config.toml [env] is missing non-empty `{ENV_SALT}`"))?;

    let stack_lock = lock::load(&repo_root.join("franken-stack.lock"))
        .map_err(|e| format!("failed loading franken-stack.lock: {e}"))?;
    let fsim = stack_lock
        .foundation("FD-FSIM")
        .ok_or_else(|| "franken-stack.lock is missing the `FD-FSIM` foundation row".to_owned())?;
    let lock_salt = fsim
        .build_env
        .get(ENV_SALT)
        .filter(|s| !s.is_empty())
        .ok_or_else(|| {
            format!("franken-stack.lock FD-FSIM build_env is missing non-empty `{ENV_SALT}`")
        })?;

    if cfg_salt != lock_salt {
        return Err(format!(
            "{ENV_SALT} mismatch: .cargo/config.toml has `{cfg_salt}` but franken-stack.lock FD-FSIM has `{lock_salt}`"
        ));
    }
    Ok(cfg_salt.clone())
}

/// Read `<sibling_root>/.cargo/config.toml` `[env].FRANKENSIM_DEPGRAPH_SALT`
/// strictly read-only. Returns `Ok(None)` if the sibling has no config file or
/// does not define the salt.
pub fn read_sibling_salt(sibling_root: &Path) -> Result<Option<String>, String> {
    let env_map = receipt::parse_build_env(sibling_root)
        .map_err(|e| format!("failed reading sibling .cargo/config.toml: {e}"))?;
    Ok(env_map.get(ENV_SALT).filter(|s| !s.is_empty()).cloned())
}

/// Walk `repo_root` and lint every build script (`build.rs`), Rust library/binary
/// source file (`src/**/*.rs`), Cargo config (`.cargo/config.toml` / `.cargo/config`),
/// and shell script (`*.sh` / `*.bash`) for forbidden conditional defaults on
/// `FRANKENSIM_DEPGRAPH_SALT` or `FRANKENSIM_DEPGRAPH_RECEIPT`.
pub fn lint_no_default_in_repo(repo_root: &Path) -> Result<Vec<NoDefaultFinding>, String> {
    let mut files = Vec::new();
    collect_lint_targets(repo_root, repo_root, &mut files)?;
    files.sort();

    let mut findings = Vec::new();
    for (rel, abs) in files {
        let raw = fs::read(&abs).map_err(|e| format!("failed reading `{}`: {e}", abs.display()))?;
        if rel.ends_with(".cargo/config.toml") || rel.ends_with(".cargo/config") {
            findings.extend(lint_cargo_config_bytes(&rel, &raw));
        } else {
            let text = String::from_utf8_lossy(&raw);
            findings.extend(lint_text(&rel, &text));
        }
    }
    findings.sort();
    findings.dedup();
    Ok(findings)
}

/// Lint a single file's contents using its repository-relative path to select
/// shell vs Rust rules.
#[must_use]
pub fn lint_text(rel_path: &str, text: &str) -> Vec<NoDefaultFinding> {
    if is_shell_path(rel_path) {
        lint_shell_text(rel_path, text)
    } else if rel_path.ends_with(".rs") {
        lint_rust_text(rel_path, text)
    } else {
        Vec::new()
    }
}

fn collect_lint_targets(
    repo_root: &Path,
    dir: &Path,
    out: &mut Vec<(String, PathBuf)>,
) -> Result<(), String> {
    let entries = fs::read_dir(dir).map_err(|e| format!("read_dir `{}`: {e}", dir.display()))?;
    for entry in entries {
        let entry = entry.map_err(|e| format!("dir entry in `{}`: {e}", dir.display()))?;
        let path = entry.path();
        let ty = entry
            .file_type()
            .map_err(|e| format!("file_type `{}`: {e}", path.display()))?;
        if ty.is_symlink() {
            continue;
        }
        let name = entry.file_name();
        let name_str = name.to_string_lossy();
        if ty.is_dir() {
            if SKIP_DIRS.contains(&name_str.as_ref()) || name_str.starts_with(".rch-target-") {
                continue;
            }
            collect_lint_targets(repo_root, &path, out)?;
        } else if ty.is_file() {
            let rel = path
                .strip_prefix(repo_root)
                .unwrap_or(&path)
                .to_string_lossy()
                .replace('\\', "/");
            if is_lint_target(&rel) {
                out.push((rel, path));
            }
        }
    }
    Ok(())
}

fn is_lint_target(rel: &str) -> bool {
    if is_shell_path(rel) {
        return true;
    }
    if rel == ".cargo/config.toml"
        || rel.ends_with("/.cargo/config.toml")
        || rel == ".cargo/config"
        || rel.ends_with("/.cargo/config")
    {
        return true;
    }
    if rel == "build.rs" || rel.ends_with("/build.rs") {
        return true;
    }
    if rel.ends_with(".rs") && (rel.starts_with("src/") || rel.contains("/src/")) {
        return true;
    }
    false
}

fn is_shell_path(rel: &str) -> bool {
    rel.ends_with(".sh") || rel.ends_with(".bash")
}

fn lint_cargo_config_bytes(rel_path: &str, bytes: &[u8]) -> Vec<NoDefaultFinding> {
    let mut findings = Vec::new();
    let Ok(tbl) = toml::parse(bytes) else {
        return findings;
    };
    let Some(env_tbl) = tbl.get("env").and_then(TomlValue::as_table) else {
        return findings;
    };

    let extract_str = |v: &TomlValue| -> Option<String> {
        match v {
            TomlValue::Str(s) => Some(s.clone()),
            TomlValue::Table(sub) => sub
                .get("value")
                .and_then(TomlValue::as_str)
                .map(ToOwned::to_owned),
            _ => None,
        }
    };

    if let Some(sp) = env_tbl.get_spanned(ENV_SALT)
        && extract_str(&sp.value).is_some_and(|s| s.is_empty())
    {
        findings.push(NoDefaultFinding {
            path: rel_path.to_owned(),
            line: usize::try_from(sp.line).unwrap_or(1),
            kind: NoDefaultKind::EmptyEnvInConfig,
            variable: ENV_SALT,
            message: format!("[env] sets `{ENV_SALT}` to an empty string"),
        });
    }
    if let Some(sp) = env_tbl.get_spanned(ENV_RECEIPT) {
        let is_empty = extract_str(&sp.value).is_some_and(|s| s.is_empty());
        let (kind, msg) = if is_empty {
            (
                NoDefaultKind::EmptyEnvInConfig,
                format!("[env] sets `{ENV_RECEIPT}` to an empty string"),
            )
        } else {
            (
                NoDefaultKind::StaticReceiptInConfig,
                format!(
                    "[env] must not statically set `{ENV_RECEIPT}`; Mode 2 receipts are exported explicitly at build time"
                ),
            )
        };
        findings.push(NoDefaultFinding {
            path: rel_path.to_owned(),
            line: usize::try_from(sp.line).unwrap_or(1),
            kind,
            variable: ENV_RECEIPT,
            message: msg,
        });
    }
    findings
}

fn lint_shell_text(rel_path: &str, text: &str) -> Vec<NoDefaultFinding> {
    let mut findings = Vec::new();
    for (idx, raw_line) in text.lines().enumerate() {
        let line_no = idx + 1;
        let code = strip_shell_comment(raw_line);
        for var in BUILD_IDENTITY_VARS {
            let prefix = format!("${{{var}");
            let mut search_from = 0usize;
            while let Some(rel_pos) = code[search_from..].find(&prefix) {
                let after = search_from + rel_pos + prefix.len();
                let rest = &code[after..];
                let is_default_op = rest.starts_with(":-")
                    || rest.starts_with('-')
                    || rest.starts_with(":=")
                    || rest.starts_with('=');
                if is_default_op {
                    findings.push(NoDefaultFinding {
                        path: rel_path.to_owned(),
                        line: line_no,
                        kind: NoDefaultKind::ShellDefault,
                        variable: var,
                        message: format!(
                            "conditional shell default expansion for `{var}` is forbidden (fail-closed policy)"
                        ),
                    });
                }
                search_from = after;
            }
        }
    }
    findings
}

/// Strip an unquoted `# ...` shell comment while preserving double-quoted text
/// (where `${VAR:-...}` is still expanded by the shell) and single-quoted text.
fn strip_shell_comment(line: &str) -> &str {
    let bytes = line.as_bytes();
    let mut in_single = false;
    let mut in_double = false;
    let mut i = 0usize;
    while i < bytes.len() {
        let b = bytes[i];
        if b == b'\\' && !in_single {
            i += 2;
            continue;
        }
        if b == b'\'' && !in_double {
            in_single = !in_single;
        } else if b == b'"' && !in_single {
            in_double = !in_double;
        } else if b == b'#' && !in_single && !in_double {
            let prev_is_boundary = i == 0
                || matches!(
                    bytes[i - 1],
                    b' ' | b'\t' | b';' | b'&' | b'|' | b'(' | b')'
                );
            if prev_is_boundary {
                return &line[..i];
            }
        }
        i += 1;
    }
    line
}

fn lint_rust_text(rel_path: &str, text: &str) -> Vec<NoDefaultFinding> {
    let cleaned = strip_rust_comments_preserving_lines(text);
    let mut findings = Vec::new();

    for var in BUILD_IDENTITY_VARS {
        let needle = format!("\"{var}\"");
        let mut cursor = 0usize;
        while let Some(rel_pos) = cleaned[cursor..].find(&needle) {
            let start = cursor + rel_pos;
            let after_needle = start + needle.len();
            let line_no = cleaned[..start].bytes().filter(|&b| b == b'\n').count() + 1;

            let stmt_start = find_stmt_start(&cleaned, start);
            let prefix = &cleaned[stmt_start..start];
            let tail = &cleaned[after_needle..];

            let is_option_env = is_option_env_call_before(prefix);
            let chain_methods = collect_chained_methods_after_arg(tail);

            if is_option_env {
                let has_fallback_method = chain_methods.iter().any(|m| {
                    matches!(
                        m.as_str(),
                        "unwrap_or" | "unwrap_or_else" | "unwrap_or_default" | "or" | "or_else"
                    )
                });
                if has_fallback_method || has_match_literal_fallback(prefix, tail) {
                    findings.push(NoDefaultFinding {
                        path: rel_path.to_owned(),
                        line: line_no,
                        kind: NoDefaultKind::OptionEnvFallback,
                        variable: var,
                        message: format!(
                            "`option_env!(\"{var}\")` must not carry a fallback default (fail-closed policy)"
                        ),
                    });
                }
            } else {
                let has_unwrap_or = chain_methods.iter().any(|m| {
                    matches!(
                        m.as_str(),
                        "unwrap_or" | "unwrap_or_else" | "unwrap_or_default"
                    )
                });
                if has_unwrap_or {
                    findings.push(NoDefaultFinding {
                        path: rel_path.to_owned(),
                        line: line_no,
                        kind: NoDefaultKind::RustUnwrapOr,
                        variable: var,
                        message: format!(
                            "`unwrap_or*` fallback on `{var}` lookup is forbidden (fail-closed policy)"
                        ),
                    });
                }
            }

            cursor = after_needle;
        }
    }

    findings
}

/// Replace Rust `//` and `/* ... */` comments with spaces while keeping every
/// `\n` intact so line numbers stay exact and string literals are untouched.
fn strip_rust_comments_preserving_lines(src: &str) -> String {
    let bytes = src.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0usize;
    let mut block_depth = 0usize;
    let mut in_line_comment = false;
    let mut in_string = false;

    while i < bytes.len() {
        let b = bytes[i];
        if in_line_comment {
            if b == b'\n' {
                in_line_comment = false;
                out.push(b'\n');
            } else {
                out.push(b' ');
            }
            i += 1;
            continue;
        }
        if block_depth > 0 {
            if b == b'/' && i + 1 < bytes.len() && bytes[i + 1] == b'*' {
                block_depth += 1;
                out.push(b' ');
                out.push(b' ');
                i += 2;
                continue;
            }
            if b == b'*' && i + 1 < bytes.len() && bytes[i + 1] == b'/' {
                block_depth -= 1;
                out.push(b' ');
                out.push(b' ');
                i += 2;
                continue;
            }
            out.push(if b == b'\n' { b'\n' } else { b' ' });
            i += 1;
            continue;
        }
        if in_string {
            out.push(b);
            if b == b'\\' && i + 1 < bytes.len() {
                out.push(bytes[i + 1]);
                i += 2;
                continue;
            }
            if b == b'"' {
                in_string = false;
            }
            i += 1;
            continue;
        }

        if b == b'"' {
            in_string = true;
            out.push(b);
            i += 1;
        } else if b == b'/' && i + 1 < bytes.len() && bytes[i + 1] == b'/' {
            in_line_comment = true;
            out.push(b' ');
            out.push(b' ');
            i += 2;
        } else if b == b'/' && i + 1 < bytes.len() && bytes[i + 1] == b'*' {
            block_depth = 1;
            out.push(b' ');
            out.push(b' ');
            i += 2;
        } else {
            out.push(b);
            i += 1;
        }
    }
    String::from_utf8_lossy(&out).into_owned()
}

fn find_stmt_start(src: &str, pos: usize) -> usize {
    src[..pos].rfind([';', '{', '}']).map_or(0, |idx| idx + 1)
}

fn is_option_env_call_before(prefix: &str) -> bool {
    let trimmed = prefix.trim_end();
    let Some(before_paren) = trimmed.strip_suffix('(') else {
        return false;
    };
    let before_bang = before_paren.trim_end();
    let Some(ident_end) = before_bang.strip_suffix('!') else {
        return false;
    };
    let ident_trimmed = ident_end.trim_end();
    if !ident_trimmed.ends_with("option_env") {
        return false;
    }
    let idx = ident_trimmed.len() - "option_env".len();
    idx == 0
        || (!ident_trimmed.as_bytes()[idx - 1].is_ascii_alphanumeric()
            && ident_trimmed.as_bytes()[idx - 1] != b'_')
}

/// Given the Rust source immediately following `"FRANKENSIM_DEPGRAPH_*"`,
/// expect the closing `)` of the call (`env::var(...)` / `option_env!(...)`)
/// and collect every method name chained directly on that expression via
/// `.method(...)` (skipping balanced argument lists and turbofish `<...>`).
/// Stops at `{`, `;`, `,`, unmatched `)`, or any non-postfix token so bodies
/// of `if let` blocks and subsequent statements never false-positive.
fn collect_chained_methods_after_arg(tail: &str) -> Vec<String> {
    let bytes = tail.as_bytes();
    let mut i = skip_ws(bytes, 0);
    // Allow optional trailing comma inside the argument list before `)`.
    if i < bytes.len() && bytes[i] == b',' {
        i = skip_ws(bytes, i + 1);
    }
    if i >= bytes.len() || bytes[i] != b')' {
        return Vec::new();
    }
    i += 1;

    let mut methods = Vec::new();
    loop {
        i = skip_ws(bytes, i);
        if i >= bytes.len() {
            break;
        }
        if bytes[i] == b'?' {
            i += 1;
            continue;
        }
        if bytes[i] != b'.' {
            break;
        }
        i = skip_ws(bytes, i + 1);
        let ident_start = i;
        while i < bytes.len() && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') {
            i += 1;
        }
        if i == ident_start {
            break;
        }
        let ident = tail[ident_start..i].to_owned();
        i = skip_ws(bytes, i);
        // Skip optional turbofish `::<...>`.
        if i + 2 <= bytes.len() && &bytes[i..i + 2] == b"::" {
            i = skip_ws(bytes, i + 2);
            if i < bytes.len() && bytes[i] == b'<' {
                let Some(next) = skip_balanced(bytes, i, b'<', b'>') else {
                    break;
                };
                i = skip_ws(bytes, next);
            }
        }
        if i < bytes.len() && bytes[i] == b'(' {
            methods.push(ident);
            let Some(next) = skip_balanced(bytes, i, b'(', b')') else {
                break;
            };
            i = next;
        } else {
            // Field access (e.g. `.0`) — keep walking in case a method follows.
            continue;
        }
    }
    methods
}

/// Detect `match option_env!("FRANKENSIM_DEPGRAPH_*") { ... None => "..." }`
/// (or `_ => "..."`) directly matching the macro result against a string literal
/// fallback.
fn has_match_literal_fallback(prefix: &str, tail: &str) -> bool {
    let trimmed_prefix = prefix.trim_end();
    // Require `match` right before `option_env!(`.
    let Some(before_opt) = trimmed_prefix.rfind("option_env") else {
        return false;
    };
    let head = trimmed_prefix[..before_opt].trim_end();
    let ends_with_match = head.ends_with("match")
        && (head.len() == "match".len()
            || (!head.as_bytes()[head.len() - "match".len() - 1].is_ascii_alphanumeric()
                && head.as_bytes()[head.len() - "match".len() - 1] != b'_'));
    if !ends_with_match {
        return false;
    }

    let bytes = tail.as_bytes();
    let mut i = skip_ws(bytes, 0);
    if i >= bytes.len() || bytes[i] != b')' {
        return false;
    }
    i = skip_ws(bytes, i + 1);
    if i >= bytes.len() || bytes[i] != b'{' {
        return false;
    }
    let Some(end) = skip_balanced(bytes, i, b'{', b'}') else {
        return false;
    };
    let body = &tail[i..end];
    let compact: String = body.chars().filter(|c| !c.is_ascii_whitespace()).collect();
    compact.contains("None=>\"") || compact.contains("_=>\"")
}

fn skip_ws(bytes: &[u8], mut i: usize) -> usize {
    while i < bytes.len() && bytes[i].is_ascii_whitespace() {
        i += 1;
    }
    i
}

fn skip_balanced(bytes: &[u8], start: usize, open: u8, close: u8) -> Option<usize> {
    if start >= bytes.len() || bytes[start] != open {
        return None;
    }
    let mut depth = 0usize;
    let mut in_str = false;
    let mut i = start;
    while i < bytes.len() {
        let b = bytes[i];
        if in_str {
            if b == b'\\' && i + 1 < bytes.len() {
                i += 2;
                continue;
            }
            if b == b'"' {
                in_str = false;
            }
            i += 1;
            continue;
        }
        if b == b'"' {
            in_str = true;
        } else if b == open {
            depth += 1;
        } else if b == close {
            depth -= 1;
            if depth == 0 {
                return Some(i + 1);
            }
        }
        i += 1;
    }
    None
}
