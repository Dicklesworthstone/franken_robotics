//! Stack resolution receipt (`frob.stack-receipt/1`): typed model, canonical
//! serialization via `frob_core::canon`, `frob.tree/1` streaming BLAKE3 tree
//! hash, FrankenSim-aligned drift classifier, duplicate-package scan, and
//! content-addressed receipt store (bead `N33a-04`, KA-767).
//!
//! Uses `frob_core::canon` (`N34-18`) as the sole canonical JSON writer,
//! `frob_core::json` (`N34-17`) as the sole JSON reader,
//! `frob_core::registry::toml` (`N34p2-01`) as the sole TOML reader, and
//! `fs_blake3` for BLAKE3 hashing. Read-only on every sibling checkout
//! (`git --no-optional-locks` with read-only subcommands only), never invokes
//! `cargo` by absolute path, prints nothing (rule L-7), and uses zero `unsafe`.

use std::collections::{BTreeMap, BTreeSet};
use std::ffi::OsStr;
use std::fmt;
use std::fs::{self, File};
use std::io::{ErrorKind, Read, Write};
use std::path::{Component, Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

use frob_core::canon::{self, CanonError, Canonical, CanonicalBytes, Value};
use frob_core::json::{self, Json, Limits};
use frob_core::registry::toml::{self, Table, Value as TomlValue};

use crate::stack::lock::{self, GitRev};

/// Schema identifier for the stack resolution receipt.
pub const STACK_RECEIPT_SCHEMA: &str = "frob.stack-receipt/1";

/// Domain tag fed first into the `frob.tree/1` streaming BLAKE3 tree hash.
pub const TREE_HASH_DOMAIN: &[u8] = b"frob.tree/1";

/// Known workspaces of this repository scanned for lockfiles and duplicates.
pub const WORKSPACES: &[(&str, &str)] = &[
    ("core", "."),
    ("apps", "apps"),
    ("bindings", "bindings/frob-gymnasium"),
    ("derive", "derive/frob-derive"),
    ("spark-cuda", "spark/frob-spark-cuda"),
    ("spark-metal", "spark/frob-spark-metal"),
    ("worker", "worker"),
];

/// Drift classification shared with FrankenSim's preflight (`§33.5`, `N33a-04`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DriftClass {
    OnPin,
    StaleLock,
    Drift,
}

impl DriftClass {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OnPin => "on-pin",
            Self::StaleLock => "stale-lock",
            Self::Drift => "drift",
        }
    }

    pub fn parse(raw: &str) -> Result<Self, ReceiptError> {
        match raw {
            "on-pin" => Ok(Self::OnPin),
            "stale-lock" => Ok(Self::StaleLock),
            "drift" => Ok(Self::Drift),
            other => Err(ReceiptError::InvalidReceipt(format!("unknown drift class `{other}`"))),
        }
    }
}

impl fmt::Display for DriftClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Sub-reason recorded when [`DriftClass::Drift`] applies (in precedence order).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DriftReason {
    Dirty,
    Retreated,
    Diverged,
    UnknownPin,
}

impl DriftReason {
    /// Precedence order when multiple drift causes apply simultaneously.
    pub const PRECEDENCE: [Self; 4] = [
        Self::Dirty,
        Self::Retreated,
        Self::Diverged,
        Self::UnknownPin,
    ];

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Dirty => "dirty",
            Self::Retreated => "retreated",
            Self::Diverged => "diverged",
            Self::UnknownPin => "unknown-pin",
        }
    }

    pub fn parse(raw: &str) -> Result<Self, ReceiptError> {
        match raw {
            "dirty" => Ok(Self::Dirty),
            "retreated" => Ok(Self::Retreated),
            "diverged" => Ok(Self::Diverged),
            "unknown-pin" => Ok(Self::UnknownPin),
            other => Err(ReceiptError::InvalidReceipt(format!("unknown drift reason `{other}`"))),
        }
    }
}

impl fmt::Display for DriftReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Classification of a sibling's own lock file (`constellation.lock`, `SUITE.lock`, etc.).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnLockEntry {
    pub file: Option<String>,
    pub hash: Option<String>,
    pub agrees: bool,
    pub class: DriftClass,
    pub reason: Option<DriftReason>,
}

/// Resolution snapshot of one sibling foundation checkout, sorted by `fd`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SiblingEntry {
    pub fd: String,
    pub rev: String,
    pub dirty: bool,
    pub tree_hash: Option<String>,
    pub features: Vec<String>,
    pub own_lock: OwnLockEntry,
}

/// Lockfile hash and package counts for one workspace of this repository.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LockfileEntry {
    pub workspace: String,
    pub hash: String,
    pub packages: i64,
    pub registry: i64,
}

/// Record of an external command executed while capturing the receipt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandEntry {
    pub argv: Vec<String>,
    pub exit: i64,
    pub output_hash: String,
}

/// Typed `frob.stack-receipt/1` resolution receipt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StackReceipt {
    pub schema: String,
    pub repo_rev: String,
    pub repo_dirty: bool,
    pub repo_tree_hash: Option<String>,
    pub toolchain: String,
    pub siblings: Vec<SiblingEntry>,
    pub lockfiles: Vec<LockfileEntry>,
    pub patches: Vec<String>,
    pub profile_overrides: Vec<String>,
    pub build_env: BTreeMap<String, String>,
    pub duplicates: Vec<String>,
    pub commands: Vec<CommandEntry>,
}

/// Structured error returned by receipt generation, verification, or storage.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReceiptError {
    Io(String),
    Git(String),
    Cargo(String),
    Lock(String),
    Toml(String),
    Json(String),
    Canon(String),
    AbsolutePatchPath { entry: String, path: String },
    MachinePathLeak { field: String, value: String },
    HashMismatch { expected: String, actual: String },
    ToolchainMismatch { toolchain_file: String, stack_lock: String },
    InvalidReceipt(String),
}

impl fmt::Display for ReceiptError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(m) => write!(f, "receipt I/O error: {m}"),
            Self::Git(m) => write!(f, "git command error: {m}"),
            Self::Cargo(m) => write!(f, "cargo command error: {m}"),
            Self::Lock(m) => write!(f, "stack lock error: {m}"),
            Self::Toml(m) => write!(f, "TOML parse error: {m}"),
            Self::Json(m) => write!(f, "JSON parse error: {m}"),
            Self::Canon(m) => write!(f, "canonical serialization error: {m}"),
            Self::AbsolutePatchPath { entry, path } => write!(
                f,
                "absolute or invalid patch path refused for `{entry}`: `{path}` (must be repository-relative)"
            ),
            Self::MachinePathLeak { field, value } => {
                write!(f, "machine path leaked into receipt at `{field}`: `{value}`")
            }
            Self::HashMismatch { expected, actual } => write!(
                f,
                "receipt BLAKE3 hash mismatch: expected `{expected}`, computed `{actual}`"
            ),
            Self::ToolchainMismatch { toolchain_file, stack_lock } => write!(
                f,
                "toolchain mismatch: rust-toolchain.toml has `{toolchain_file}`, franken-stack.lock has `{stack_lock}`"
            ),
            Self::InvalidReceipt(m) => write!(f, "invalid receipt: {m}"),
        }
    }
}

impl std::error::Error for ReceiptError {}

// ============================================================================
// Canonical JSON serialization via `frob_core::canon` (sole writer, N34-18)
// ============================================================================

impl Canonical for OwnLockEntry {
    fn write(&self, w: Value<'_>) -> Result<(), CanonError> {
        w.object(|o| {
            o.bool("agrees", self.agrees)?;
            o.str("class", self.class.as_str())?;
            match &self.file {
                Some(file) => o.str("file", file)?,
                None => o.null("file")?,
            }
            match &self.hash {
                Some(hash) => o.str("hash", hash)?,
                None => o.null("hash")?,
            }
            match self.reason {
                Some(reason) => o.str("reason", reason.as_str())?,
                None => o.null("reason")?,
            }
            Ok(())
        })
    }
}

impl Canonical for SiblingEntry {
    fn write(&self, w: Value<'_>) -> Result<(), CanonError> {
        w.object(|o| {
            o.bool("dirty", self.dirty)?;
            o.str("fd", &self.fd)?;
            o.array("features", |a| {
                for feat in &self.features {
                    a.push_str(feat);
                }
                Ok(())
            })?;
            o.member_term("own_lock", &self.own_lock)?;
            o.str("rev", &self.rev)?;
            match &self.tree_hash {
                Some(h) => o.str("tree_hash", h)?,
                None => o.null("tree_hash")?,
            }
            Ok(())
        })
    }
}

impl Canonical for LockfileEntry {
    fn write(&self, w: Value<'_>) -> Result<(), CanonError> {
        w.object(|o| {
            o.str("hash", &self.hash)?;
            o.int53("packages", self.packages)?;
            o.int53("registry", self.registry)?;
            o.str("workspace", &self.workspace)?;
            Ok(())
        })
    }
}

impl Canonical for CommandEntry {
    fn write(&self, w: Value<'_>) -> Result<(), CanonError> {
        w.object(|o| {
            o.array("argv", |a| {
                for arg in &self.argv {
                    a.push_str(arg);
                }
                Ok(())
            })?;
            o.int53("exit", self.exit)?;
            o.str("output_hash", &self.output_hash)?;
            Ok(())
        })
    }
}

impl Canonical for StackReceipt {
    fn write(&self, w: Value<'_>) -> Result<(), CanonError> {
        w.object(|o| {
            o.object("build_env", |env_obj| {
                for (k, v) in &self.build_env {
                    env_obj.str(k, v)?;
                }
                Ok(())
            })?;
            o.array("commands", |a| {
                for cmd in &self.commands {
                    a.push_term(cmd)?;
                }
                Ok(())
            })?;
            o.array("duplicates", |a| {
                for dup in &self.duplicates {
                    a.push_str(dup);
                }
                Ok(())
            })?;
            o.array("lockfiles", |a| {
                for lf in &self.lockfiles {
                    a.push_term(lf)?;
                }
                Ok(())
            })?;
            o.array("patches", |a| {
                for p in &self.patches {
                    a.push_str(p);
                }
                Ok(())
            })?;
            o.array("profile_overrides", |a| {
                for ov in &self.profile_overrides {
                    a.push_str(ov);
                }
                Ok(())
            })?;
            o.bool("repo_dirty", self.repo_dirty)?;
            o.str("repo_rev", &self.repo_rev)?;
            match &self.repo_tree_hash {
                Some(h) => o.str("repo_tree_hash", h)?,
                None => o.null("repo_tree_hash")?,
            }
            o.str("schema", &self.schema)?;
            o.array("siblings", |a| {
                for sib in &self.siblings {
                    a.push_term(sib)?;
                }
                Ok(())
            })?;
            o.str("toolchain", &self.toolchain)?;
            Ok(())
        })
    }
}

impl StackReceipt {
    /// Serialize this receipt using `frob_core::canon::to_canonical`.
    pub fn to_canonical_bytes(&self) -> Result<CanonicalBytes, ReceiptError> {
        scan_no_machine_paths(self)?;
        canon::to_canonical(self).map_err(|e| ReceiptError::Canon(e.to_string()))
    }

    /// Plain BLAKE3 lowercase hex digest of the canonical receipt bytes.
    pub fn digest_hex(&self) -> Result<String, ReceiptError> {
        let bytes = self.to_canonical_bytes()?;
        Ok(fs_blake3::hash_bytes(bytes.as_bytes()).to_hex())
    }

    /// Parse and verify canonical JSON receipt bytes using `frob_core::json::admit`.
    pub fn from_canonical_bytes(bytes: &[u8]) -> Result<Self, ReceiptError> {
        let admitted = json::admit(bytes, &Limits::default())
            .map_err(|e| ReceiptError::Json(e.to_string()))?;
        let receipt = decode_receipt_json(admitted.value())?;
        let _ = admitted
            .into_canonical()
            .map_err(|e| ReceiptError::Canon(e.to_string()))?;
        scan_no_machine_paths(&receipt)?;
        Ok(receipt)
    }
}

// ============================================================================
// Tree hash (`frob.tree/1`)
// ============================================================================

/// Compute the deterministic `frob.tree/1` BLAKE3 hash over all git-tracked
/// paths in `repo_dir`. Never writes to `repo_dir` and ignores untracked files
/// and file modification timestamps.
pub fn tree_hash(repo_dir: &Path) -> Result<String, ReceiptError> {
    let ls = git_ro(repo_dir, &["ls-files", "-z"])?;
    let mut paths: Vec<&[u8]> = ls.split(|&b| b == 0).filter(|p| !p.is_empty()).collect();
    paths.sort_unstable();
    paths.dedup();

    let mut hasher = fs_blake3::Blake3::new();
    hasher.update(TREE_HASH_DOMAIN);

    let mut chunk_buf = [0u8; 65_536];
    for rel_bytes in paths {
        hasher.update(&(rel_bytes.len() as u64).to_le_bytes());
        hasher.update(rel_bytes);

        let rel_os = bytes_to_os_str(rel_bytes)?;
        let full = repo_dir.join(rel_os);
        let meta = match fs::symlink_metadata(&full) {
            Ok(m) => m,
            Err(err) if err.kind() == ErrorKind::NotFound => {
                // Deleted in working tree: marker byte 0xFF and no content.
                hasher.update(&[0xFF]);
                continue;
            }
            Err(err) => {
                return Err(ReceiptError::Io(format!(
                    "lstat `{}`: {err}",
                    String::from_utf8_lossy(rel_bytes)
                )));
            }
        };

        if meta.file_type().is_symlink() {
            let target = fs::read_link(&full).map_err(|e| {
                ReceiptError::Io(format!("readlink `{}`: {e}", String::from_utf8_lossy(rel_bytes)))
            })?;
            let target_bytes = os_str_to_bytes(target.as_os_str())?;
            hasher.update(&[0x00]);
            hasher.update(&(target_bytes.len() as u64).to_le_bytes());
            hasher.update(&target_bytes);
        } else if meta.is_dir() {
            // Gitlink / submodule entry: feed staged OID bytes if present.
            let oid = gitlink_staged_oid(repo_dir, rel_bytes);
            hasher.update(&[0x00]);
            hasher.update(&(oid.len() as u64).to_le_bytes());
            hasher.update(&oid);
        } else {
            let exec_byte = regular_file_exec_byte(repo_dir, rel_bytes, &meta);
            let mut file = File::open(&full).map_err(|e| {
                ReceiptError::Io(format!("open `{}`: {e}", String::from_utf8_lossy(rel_bytes)))
            })?;
            let size = meta.len();
            hasher.update(&[exec_byte]);
            hasher.update(&size.to_le_bytes());

            let mut remaining = size;
            while remaining > 0 {
                let to_read = usize::try_from(remaining).unwrap_or(chunk_buf.len()).min(chunk_buf.len());
                let n = file.read(&mut chunk_buf[..to_read]).map_err(|e| {
                    ReceiptError::Io(format!("read `{}`: {e}", String::from_utf8_lossy(rel_bytes)))
                })?;
                if n == 0 {
                    return Err(ReceiptError::Io(format!(
                        "short read while hashing tracked file `{}`",
                        String::from_utf8_lossy(rel_bytes)
                    )));
                }
                hasher.update(&chunk_buf[..n]);
                remaining -= n as u64;
            }
        }
    }

    Ok(hasher.finalize().to_hex())
}

#[cfg(unix)]
fn regular_file_exec_byte(_repo_dir: &Path, _rel_bytes: &[u8], meta: &fs::Metadata) -> u8 {
    use std::os::unix::fs::PermissionsExt;
    u8::from((meta.permissions().mode() & 0o111) != 0)
}

#[cfg(not(unix))]
fn regular_file_exec_byte(repo_dir: &Path, rel_bytes: &[u8], _meta: &fs::Metadata) -> u8 {
    let rel_str = String::from_utf8_lossy(rel_bytes);
    let Ok(out) = git_ro(repo_dir, &["ls-files", "--stage", "-z", "--", &rel_str]) else {
        return 0;
    };
    u8::from(out.starts_with(b"100755 "))
}

fn gitlink_staged_oid(repo_dir: &Path, rel_bytes: &[u8]) -> Vec<u8> {
    let rel_str = String::from_utf8_lossy(rel_bytes);
    let Ok(out) = git_ro(repo_dir, &["ls-files", "--stage", "-z", "--", &rel_str]) else {
        return Vec::new();
    };
    let first_entry = out.split(|&b| b == 0).next().unwrap_or(&[]);
    let header = first_entry.split(|&b| b == b'\t').next().unwrap_or(&[]);
    let mut parts = header.split(|&b| b == b' ');
    let _mode = parts.next();
    parts.next().map_or_else(Vec::new, |oid| oid.to_vec())
}

#[cfg(unix)]
fn bytes_to_os_str(bytes: &[u8]) -> Result<&OsStr, ReceiptError> {
    use std::os::unix::ffi::OsStrExt;
    Ok(OsStr::from_bytes(bytes))
}

#[cfg(not(unix))]
fn bytes_to_os_str(bytes: &[u8]) -> Result<&OsStr, ReceiptError> {
    let s = std::str::from_utf8(bytes)
        .map_err(|e| ReceiptError::Io(format!("non-UTF-8 tracked path: {e}")))?;
    Ok(OsStr::new(s))
}

#[cfg(unix)]
fn os_str_to_bytes(s: &OsStr) -> Result<Vec<u8>, ReceiptError> {
    use std::os::unix::ffi::OsStrExt;
    Ok(s.as_bytes().to_vec())
}

#[cfg(not(unix))]
fn os_str_to_bytes(s: &OsStr) -> Result<Vec<u8>, ReceiptError> {
    let utf8 = s
        .to_str()
        .ok_or_else(|| ReceiptError::Io("non-UTF-8 symlink target".to_owned()))?;
    Ok(utf8.as_bytes().to_vec())
}

// ============================================================================
// Drift classification (`on-pin`, `stale-lock`, `drift` + reason)
// ============================================================================

/// Classify a git checkout at `repo_dir` against expected commit `pin_rev`
/// using strictly read-only `git --no-optional-locks` queries.
pub fn classify_checkout(
    repo_dir: &Path,
    pin_rev: Option<&str>,
) -> Result<(DriftClass, Option<DriftReason>), ReceiptError> {
    // Dirty has highest precedence over retreated/diverged/unknown-pin.
    if is_dirty_tracked(repo_dir)? {
        return Ok((DriftClass::Drift, Some(DriftReason::Dirty)));
    }
    let Some(pin) = pin_rev else {
        return Ok((DriftClass::Drift, Some(DriftReason::UnknownPin)));
    };
    if pin == "unadmitted" {
        return Ok((DriftClass::OnPin, None));
    }
    if !is_lower_hex(pin, 40) {
        return Ok((DriftClass::Drift, Some(DriftReason::UnknownPin)));
    }

    let head = head_rev(repo_dir)?;
    if head == pin {
        return Ok((DriftClass::OnPin, None));
    }

    let commit_spec = format!("{pin}^{{commit}}");
    if !git_ro_status(repo_dir, &["cat-file", "-e", &commit_spec])? {
        return Ok((DriftClass::Drift, Some(DriftReason::UnknownPin)));
    }
    if git_ro_status(repo_dir, &["merge-base", "--is-ancestor", pin, &head])? {
        return Ok((DriftClass::StaleLock, None));
    }
    if git_ro_status(repo_dir, &["merge-base", "--is-ancestor", &head, pin])? {
        return Ok((DriftClass::Drift, Some(DriftReason::Retreated)));
    }
    Ok((DriftClass::Drift, Some(DriftReason::Diverged)))
}

/// Combine multiple `(DriftClass, Option<DriftReason>)` results using precedence
/// `Drift (Dirty > Retreated > Diverged > UnknownPin) > StaleLock > OnPin`.
#[must_use]
pub fn combine_classifications(
    pairs: &[(DriftClass, Option<DriftReason>)],
) -> (DriftClass, Option<DriftReason>) {
    let mut best_drift: Option<DriftReason> = None;
    for &(cls, reason) in pairs {
        if cls == DriftClass::Drift {
            if let Some(r) = reason {
                let rank = |x: DriftReason| {
                    DriftReason::PRECEDENCE
                        .iter()
                        .position(|&p| p == x)
                        .unwrap_or(usize::MAX)
                };
                if best_drift.is_none_or(|cur| rank(r) < rank(cur)) {
                    best_drift = Some(r);
                }
            } else if best_drift.is_none() {
                best_drift = Some(DriftReason::UnknownPin);
            }
        }
    }
    if let Some(reason) = best_drift {
        return (DriftClass::Drift, Some(reason));
    }
    if pairs.iter().any(|&(c, _)| c == DriftClass::StaleLock) {
        return (DriftClass::StaleLock, None);
    }
    (DriftClass::OnPin, None)
}

/// Classify a sibling checkout and, when present, its own lock file
/// (`constellation.lock`, `SUITE.lock`, or `franken-stack.lock`) against peer
/// checkouts located beside `sib_dir`.
pub fn classify_sibling_own_lock(
    sib_dir: &Path,
    lock_pin_rev: Option<&str>,
) -> Result<OwnLockEntry, ReceiptError> {
    let own = parse_own_lock_entries(sib_dir)?;
    let self_cls = classify_checkout(sib_dir, lock_pin_rev)?;
    let Some((file, hash, entries)) = own else {
        return Ok(OwnLockEntry {
            file: None,
            hash: None,
            agrees: self_cls.0 == DriftClass::OnPin,
            class: self_cls.0,
            reason: self_cls.1,
        });
    };

    let mut pairs = vec![self_cls];
    let sib_parent = sib_dir.parent();
    for (dep_name, dep_pin) in entries {
        if dep_name.is_empty()
            || dep_name == "."
            || dep_name == ".."
            || dep_name.contains(['/', '\\'])
        {
            return Err(ReceiptError::InvalidReceipt(format!(
                "invalid sibling dependency name `{dep_name}` in `{file}`"
            )));
        }
        if let Some(parent) = sib_parent {
            let dep_dir = parent.join(&dep_name);
            if dep_dir.is_dir() && dep_dir.join(".git").exists() {
                pairs.push(classify_checkout(&dep_dir, Some(dep_pin.as_str()))?);
            }
        }
    }

    let (class, reason) = combine_classifications(&pairs);
    Ok(OwnLockEntry {
        file: Some(file),
        hash: Some(hash),
        agrees: class == DriftClass::OnPin,
        class,
        reason,
    })
}

fn parse_own_lock_entries(
    sib_dir: &Path,
) -> Result<Option<(String, String, Vec<(String, String)>)>, ReceiptError> {
    let constellation = sib_dir.join("constellation.lock");
    if constellation.is_file() {
        let bytes = read_file_bytes(&constellation)?;
        let hash = fs_blake3::hash_bytes(&bytes).to_hex();
        let admitted = json::admit(&bytes, &Limits::default())
            .map_err(|e| ReceiptError::Json(format!("constellation.lock: {e}")))?;
        let mut entries = Vec::new();
        if let Some(libs) = admitted.value().get("libraries").and_then(|v| v.as_array()) {
            for lib in libs {
                let name = lib.get("lib").and_then(|v| v.as_str()).unwrap_or("");
                let pin = lib.get("git_head").and_then(|v| v.as_str()).unwrap_or("");
                if !name.is_empty() {
                    entries.push((name.to_owned(), pin.to_owned()));
                }
            }
        }
        return Ok(Some(("constellation.lock".to_owned(), hash, entries)));
    }

    let suite_lock = sib_dir.join("SUITE.lock");
    if suite_lock.is_file() {
        let bytes = read_file_bytes(&suite_lock)?;
        let hash = fs_blake3::hash_bytes(&bytes).to_hex();
        let text = std::str::from_utf8(&bytes)
            .map_err(|e| ReceiptError::Io(format!("SUITE.lock UTF-8: {e}")))?;
        let mut entries = Vec::new();
        for raw in text.lines() {
            let line = raw.split('#').next().unwrap_or("").trim();
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 && parts[0] == "suite" {
                let commit = parts[2..]
                    .iter()
                    .find_map(|tok| tok.strip_prefix("commit="))
                    .unwrap_or("");
                entries.push((parts[1].to_owned(), commit.to_owned()));
            }
        }
        return Ok(Some(("SUITE.lock".to_owned(), hash, entries)));
    }

    let sub_stack = sib_dir.join("franken-stack.lock");
    if sub_stack.is_file() {
        let bytes = read_file_bytes(&sub_stack)?;
        let hash = fs_blake3::hash_bytes(&bytes).to_hex();
        let tbl = toml::parse(&bytes)
            .map_err(|e| ReceiptError::Toml(format!("sibling franken-stack.lock: {e}")))?;
        let mut entries = Vec::new();
        if let Some(arr) = tbl.get("foundation").and_then(TomlValue::as_array) {
            for sp in arr {
                let Some(row) = sp.value.as_table() else { continue };
                let Some(rel) = row.get("path").and_then(TomlValue::as_str) else { continue };
                let validated = validate_rel_path("sibling franken-stack.lock", rel)?;
                let dep_name = validated
                    .strip_prefix("../")
                    .unwrap_or(&validated)
                    .split('/')
                    .next()
                    .unwrap_or("");
                let pin = row.get("git_rev").and_then(TomlValue::as_str).unwrap_or("");
                if !dep_name.is_empty() {
                    entries.push((dep_name.to_owned(), pin.to_owned()));
                }
            }
        }
        return Ok(Some(("franken-stack.lock".to_owned(), hash, entries)));
    }

    Ok(None)
}

// ============================================================================
// Manifest, lockfile, build_env, and duplicate parsers
// ============================================================================

/// Parse `[patch.*]` entries from the root `Cargo.toml` AST, refusing any
/// absolute, home-relative, or escaping path (`receipt_refuses_absolute_patch_path`).
pub fn parse_patches(
    cargo_toml: &Table,
) -> Result<(Vec<String>, BTreeMap<String, String>), ReceiptError> {
    let mut patches = Vec::new();
    let mut by_crate = BTreeMap::new();
    let Some(patch_tbl) = cargo_toml.get("patch").and_then(TomlValue::as_table) else {
        return Ok((patches, by_crate));
    };
    for (src_name, src_sp) in patch_tbl.iter() {
        let Some(entries) = src_sp.value.as_table() else { continue };
        for (dep_name, spec_sp) in entries.iter() {
            let label = format!("patch.{src_name}.{dep_name}");
            match &spec_sp.value {
                TomlValue::Table(spec) => {
                    if let Some(raw_path) = spec.get("path").and_then(TomlValue::as_str) {
                        let rel = validate_rel_path(&label, raw_path)?;
                        patches.push(format!("{dep_name} -> {rel}"));
                        by_crate.insert(dep_name.clone(), rel);
                    } else if let Some(git_url) = spec.get("git").and_then(TomlValue::as_str) {
                        patches.push(format!("{dep_name} -> {git_url}"));
                    }
                }
                TomlValue::Str(s) => patches.push(format!("{dep_name} -> {s}")),
                _ => {}
            }
        }
    }
    patches.sort();
    Ok((patches, by_crate))
}

/// Extract `[profile.<prof>.package.<pkg>]` overrides from the root `Cargo.toml`.
#[must_use]
pub fn parse_profile_overrides(cargo_toml: &Table) -> Vec<String> {
    let mut overrides = Vec::new();
    let Some(profiles) = cargo_toml.get("profile").and_then(TomlValue::as_table) else {
        return overrides;
    };
    let mut fsqlite_dev_opt1 = false;
    for (prof_name, prof_sp) in profiles.iter() {
        let Some(prof_tbl) = prof_sp.value.as_table() else { continue };
        let Some(pkgs) = prof_tbl.get("package").and_then(TomlValue::as_table) else { continue };
        for (pkg_name, pkg_sp) in pkgs.iter() {
            let Some(pkg_tbl) = pkg_sp.value.as_table() else { continue };
            if prof_name == "dev"
                && (pkg_name == "fsqlite" || pkg_name.starts_with("fsqlite-"))
                && pkg_tbl.len() == 1
                && pkg_tbl.get("opt-level").and_then(TomlValue::as_int) == Some(1)
            {
                fsqlite_dev_opt1 = true;
                continue;
            }
            for (k, v_sp) in pkg_tbl.iter() {
                let val_str = toml_scalar_display(&v_sp.value);
                if prof_name == "dev" {
                    overrides.push(format!("{pkg_name}: {k} {val_str}"));
                } else {
                    overrides.push(format!("{prof_name}.{pkg_name}: {k} {val_str}"));
                }
            }
        }
    }
    if fsqlite_dev_opt1 {
        overrides.push("fsqlite*: opt-level 1".to_owned());
    }
    overrides.sort();
    overrides
}

/// Parse `[env]` from `<root>/.cargo/config.toml` when present.
pub fn parse_build_env(root_dir: &Path) -> Result<BTreeMap<String, String>, ReceiptError> {
    let cfg_path = root_dir.join(".cargo").join("config.toml");
    if !cfg_path.is_file() {
        return Ok(BTreeMap::new());
    }
    let tbl = parse_toml_file(&cfg_path, ".cargo/config.toml")?;
    let mut out = BTreeMap::new();
    if let Some(env_tbl) = tbl.get("env").and_then(TomlValue::as_table) {
        for (k, sp) in env_tbl.iter() {
            let v = match &sp.value {
                TomlValue::Table(sub) => sub
                    .get("value")
                    .map(toml_scalar_display)
                    .unwrap_or_default(),
                other => toml_scalar_display(other),
            };
            out.insert(k.clone(), v);
        }
    }
    Ok(out)
}

/// Merge `.cargo/config.toml` `[env]` with an optional Mode 2
/// `FRANKENSIM_DEPGRAPH_RECEIPT` override under the precedence rule of `N33a-08`.
///
/// When `depgraph_receipt` is `None`, returns the `.cargo/config.toml` `[env]`
/// entries unchanged (Mode 1). When `Some(receipt_val)` is supplied, resolves
/// the mode fail-closed using [`frob_core::stack::build_identity::resolve_mode`]
/// and records `FRANKENSIM_DEPGRAPH_RECEIPT` in the returned map so the stack
/// receipt hash differs from a development build's.
pub fn parse_build_env_with_receipt(
    root_dir: &Path,
    depgraph_receipt: Option<&str>,
    receipt_wins_when_both: Option<bool>,
) -> Result<BTreeMap<String, String>, ReceiptError> {
    let mut out = parse_build_env(root_dir)?;
    let Some(receipt_val) = depgraph_receipt else {
        return Ok(out);
    };
    let salt = out
        .get(frob_core::stack::build_identity::ENV_SALT)
        .map(String::as_str);
    frob_core::stack::build_identity::resolve_mode(
        salt,
        Some(receipt_val),
        receipt_wins_when_both,
    )
    .map_err(|e| ReceiptError::InvalidReceipt(e.to_string()))?;
    out.insert(
        frob_core::stack::build_identity::ENV_RECEIPT.to_owned(),
        receipt_val.to_owned(),
    );
    Ok(out)
}

/// Parse `cargo tree -d` stdout into deduplicated `"<pkg> <ver>"` entries with
/// `asupersync` duplicates listed first, then remaining entries sorted bytewise.
#[must_use]
pub fn parse_duplicates(tree_d_stdout: &[u8]) -> Vec<String> {
    let text = String::from_utf8_lossy(tree_d_stdout);
    let mut seen: BTreeSet<(bool, String, String)> = BTreeSet::new();
    for line in text.lines() {
        let Some(first_ch) = line.chars().next() else { continue };
        if first_ch.is_whitespace() || matches!(first_ch, '├' | '└' | '│' | '(') {
            continue;
        }
        let mut toks = line.split_whitespace();
        let (Some(name), Some(ver_tok)) = (toks.next(), toks.next()) else { continue };
        let Some(ver) = ver_tok.strip_prefix('v') else { continue };
        if !name.bytes().all(|b| b.is_ascii_alphanumeric() || matches!(b, b'_' | b'-')) {
            continue;
        }
        if !ver.bytes().next().is_some_and(|b| b.is_ascii_digit()) || !ver.contains('.') {
            continue;
        }
        seen.insert((name != "asupersync", name.to_owned(), ver.to_owned()));
    }
    seen.into_iter().map(|(_, n, v)| format!("{n} {v}")).collect()
}

fn parse_cargo_lock_counts(lock_path: &Path) -> Result<(i64, i64), ReceiptError> {
    let tbl = parse_toml_file(lock_path, "Cargo.lock")?;
    let Some(pkgs) = tbl.get("package").and_then(TomlValue::as_array) else {
        return Ok((0, 0));
    };
    let total = i64::try_from(pkgs.len()).unwrap_or(i64::MAX);
    let registry = pkgs
        .iter()
        .filter(|sp| {
            sp.value
                .as_table()
                .and_then(|t| t.get("source"))
                .and_then(TomlValue::as_str)
                .is_some_and(|s| s.starts_with("registry+"))
        })
        .count();
    Ok((total, i64::try_from(registry).unwrap_or(i64::MAX)))
}

// ============================================================================
// Receipt construction, content-addressed store, and verification
// ============================================================================

/// Compute the full `frob.stack-receipt/1` receipt in memory for `root_dir` and
/// `lock_path`, returning `(receipt, canonical_bytes, blake3_hex)`.
///
/// Writes nothing to `root_dir` or any sibling checkout. Invokes `cargo` only
/// via `PATH` (never by absolute path) and `git` only with `--no-optional-locks`
/// read-only subcommands.
pub fn build_receipt(
    root_dir: &Path,
    lock_path: &Path,
) -> Result<(StackReceipt, CanonicalBytes, String), ReceiptError> {
    let cargo_toml = parse_toml_file(&root_dir.join("Cargo.toml"), "Cargo.toml")?;
    let toolchain_toml =
        parse_toml_file(&root_dir.join("rust-toolchain.toml"), "rust-toolchain.toml")?;
    let stack_lock = lock::load(lock_path).map_err(|e| ReceiptError::Lock(e.to_string()))?;

    let tc_channel = toolchain_toml
        .get("toolchain")
        .and_then(TomlValue::as_table)
        .and_then(|t| t.get("channel"))
        .and_then(TomlValue::as_str)
        .filter(|s| !s.is_empty())
        .ok_or_else(|| {
            ReceiptError::InvalidReceipt("rust-toolchain.toml missing [toolchain].channel".to_owned())
        })?;
    if tc_channel != stack_lock.toolchain.as_str() {
        return Err(ReceiptError::ToolchainMismatch {
            toolchain_file: tc_channel.to_owned(),
            stack_lock: stack_lock.toolchain.as_str().to_owned(),
        });
    }

    let (patches, patch_paths_by_crate) = parse_patches(&cargo_toml)?;
    let profile_overrides = parse_profile_overrides(&cargo_toml);
    let env_receipt = match std::env::var(frob_core::stack::build_identity::ENV_RECEIPT) {
        Ok(v) => Some(v),
        Err(std::env::VarError::NotPresent) => None,
        Err(std::env::VarError::NotUnicode(_)) => {
            return Err(ReceiptError::InvalidReceipt(format!(
                "{} must be valid UTF-8",
                frob_core::stack::build_identity::ENV_RECEIPT
            )));
        }
    };
    let build_env = parse_build_env_with_receipt(
        root_dir,
        env_receipt.as_deref(),
        frob_core::stack::build_identity::RECEIPT_WINS_WHEN_BOTH,
    )?;

    let repo_rev = head_rev(root_dir)?;
    let repo_dirty = is_dirty_tracked(root_dir)?;
    let repo_tree_hash = if repo_dirty { Some(tree_hash(root_dir)?) } else { None };

    let mut lockfiles = Vec::new();
    let mut commands = Vec::new();
    let mut dup_set: BTreeSet<(bool, String)> = BTreeSet::new();

    for &(ws_name, ws_rel) in WORKSPACES {
        let ws_dir = if ws_rel == "." {
            root_dir.to_path_buf()
        } else {
            root_dir.join(ws_rel)
        };
        let ws_manifest = ws_dir.join("Cargo.toml");
        let ws_lock = ws_dir.join("Cargo.lock");
        if ws_name == "core" {
            if !ws_lock.is_file() {
                return Err(ReceiptError::InvalidReceipt(
                    "core Cargo.lock is missing".to_owned(),
                ));
            }
        } else {
            if !ws_manifest.is_file() {
                continue;
            }
            if !ws_lock.is_file() {
                return Err(ReceiptError::InvalidReceipt(format!(
                    "annex workspace `{ws_name}` ({ws_rel}/Cargo.toml) is missing Cargo.lock"
                )));
            }
        }

        // Verify lockfile consistency through PATH `cargo` (never absolute path).
        let meta = run_cargo_ro(&ws_dir, &["metadata", "--locked", "--offline", "--format-version", "1"])?;
        if meta.exit != 0 {
            return Err(ReceiptError::Cargo(format!(
                "cargo metadata --locked failed in workspace `{ws_name}`: {}",
                String::from_utf8_lossy(&meta.stderr).trim()
            )));
        }

        let (packages, registry) = parse_cargo_lock_counts(&ws_lock)?;
        let lock_hash = blake3_file(&ws_lock)?;
        lockfiles.push(LockfileEntry {
            workspace: ws_name.to_owned(),
            hash: lock_hash,
            packages,
            registry,
        });

        let tree = run_cargo_ro(&ws_dir, &["tree", "--locked", "--offline", "-d"])?;
        if tree.exit != 0 {
            return Err(ReceiptError::Cargo(format!(
                "cargo tree -d failed in workspace `{ws_name}`: {}",
                String::from_utf8_lossy(&tree.stderr).trim()
            )));
        }
        let norm_stdout = normalize_command_stdout(&tree.stdout, root_dir);
        let argv = if ws_name == "core" {
            vec!["cargo".to_owned(), "tree".to_owned(), "-d".to_owned()]
        } else {
            vec![
                "cargo".to_owned(),
                "tree".to_owned(),
                "-d".to_owned(),
                "--manifest-path".to_owned(),
                format!("{ws_rel}/Cargo.toml"),
            ]
        };
        commands.push(CommandEntry {
            argv,
            exit: i64::from(tree.exit),
            output_hash: fs_blake3::hash_bytes(&norm_stdout).to_hex(),
        });
        for dup in parse_duplicates(&norm_stdout) {
            let not_asup = !dup.starts_with("asupersync ");
            dup_set.insert((not_asup, dup));
        }
    }

    lockfiles.sort_by(|a, b| a.workspace.cmp(&b.workspace));
    let duplicates: Vec<String> = dup_set.into_iter().map(|(_, s)| s).collect();

    let mut siblings = Vec::new();
    for row in &stack_lock.foundations {
        let rel_path = match row.path.as_ref().map(|p| p.as_str().to_owned()) {
            Some(p) => p,
            None => match patch_paths_by_crate.get(&row.name) {
                Some(p) => p.clone(),
                None => continue,
            },
        };
        let validated = validate_rel_path(&format!("foundation.{}.path", row.id), &rel_path)?;
        let sib_dir = normalize_joined_path(root_dir, &validated);
        if !sib_dir.is_dir() {
            return Err(ReceiptError::InvalidReceipt(format!(
                "sibling directory for `{}` (`{validated}`) does not exist",
                row.id
            )));
        }

        let rev = head_rev(&sib_dir)?;
        let dirty = is_dirty_tracked(&sib_dir)?;
        let sib_tree_hash = if dirty { Some(tree_hash(&sib_dir)?) } else { None };
        let pin_str = row.git_rev.as_ref().map(GitRev::as_str);
        let own_lock = classify_sibling_own_lock(&sib_dir, pin_str)?;

        siblings.push(SiblingEntry {
            fd: row.id.as_str().to_owned(),
            rev,
            dirty,
            tree_hash: sib_tree_hash,
            features: row.features.clone(),
            own_lock,
        });
    }
    siblings.sort_by(|a, b| a.fd.cmp(&b.fd));

    let receipt = StackReceipt {
        schema: STACK_RECEIPT_SCHEMA.to_owned(),
        repo_rev,
        repo_dirty,
        repo_tree_hash,
        toolchain: tc_channel.to_owned(),
        siblings,
        lockfiles,
        patches,
        profile_overrides,
        build_env,
        duplicates,
        commands,
    };
    let canon = receipt.to_canonical_bytes()?;
    let digest = fs_blake3::hash_bytes(canon.as_bytes()).to_hex();
    Ok((receipt, canon, digest))
}

/// Atomically store `canon` under `out_dir/<64-hex>.json` (write aside, sync,
/// rename). Leaves an intact existing file untouched; refuses a tampered file.
pub fn write_receipt(
    out_dir: &Path,
    canon: &CanonicalBytes,
) -> Result<(PathBuf, String), ReceiptError> {
    let digest = fs_blake3::hash_bytes(canon.as_bytes()).to_hex();
    fs::create_dir_all(out_dir)
        .map_err(|e| ReceiptError::Io(format!("create_dir_all `{}`: {e}", out_dir.display())))?;
    let dest = out_dir.join(format!("{digest}.json"));
    if dest.exists() {
        let existing = read_file_bytes(&dest)?;
        let actual = fs_blake3::hash_bytes(&existing).to_hex();
        if existing == canon.as_bytes() && actual == digest {
            return Ok((dest, digest));
        }
        return Err(ReceiptError::HashMismatch {
            expected: digest,
            actual,
        });
    }

    static TMP_SEQ: AtomicU64 = AtomicU64::new(0);
    let seq = TMP_SEQ.fetch_add(1, Ordering::Relaxed);
    let tmp = out_dir.join(format!(".receipt-{}-{seq}.tmp", std::process::id()));
    let mut f = File::create(&tmp)
        .map_err(|e| ReceiptError::Io(format!("create temp `{}`: {e}", tmp.display())))?;
    f.write_all(canon.as_bytes())
        .and_then(|()| f.sync_all())
        .map_err(|e| ReceiptError::Io(format!("write/sync temp `{}`: {e}", tmp.display())))?;
    drop(f);
    fs::rename(&tmp, &dest)
        .map_err(|e| ReceiptError::Io(format!("rename `{}` -> `{}`: {e}", tmp.display(), dest.display())))?;
    Ok((dest, digest))
}

/// Read `<64-hex>.json`, verify its content BLAKE3 matches its filename stem,
/// verify strict canonicality via `frob_core::json::admit` + `into_canonical`,
/// and decode [`StackReceipt`].
pub fn load_and_verify_receipt(
    path: &Path,
) -> Result<(StackReceipt, CanonicalBytes, String), ReceiptError> {
    let fname = path
        .file_name()
        .and_then(OsStr::to_str)
        .ok_or_else(|| ReceiptError::InvalidReceipt("receipt path has no UTF-8 filename".to_owned()))?;
    let expected = fname.strip_suffix(".json").ok_or_else(|| {
        ReceiptError::InvalidReceipt(format!("receipt filename `{fname}` must end with `.json`"))
    })?;
    if !is_lower_hex(expected, 64) {
        return Err(ReceiptError::InvalidReceipt(format!(
            "receipt filename stem `{expected}` is not 64 lower-case hex digits"
        )));
    }
    let bytes = read_file_bytes(path)?;
    let actual = fs_blake3::hash_bytes(&bytes).to_hex();
    if actual != expected {
        return Err(ReceiptError::HashMismatch {
            expected: expected.to_owned(),
            actual,
        });
    }
    let receipt = StackReceipt::from_canonical_bytes(&bytes)?;
    let canon = receipt.to_canonical_bytes()?;
    if canon.as_bytes() != bytes.as_slice() {
        return Err(ReceiptError::Canon(
            "re-serialized receipt bytes differ from stored bytes".to_owned(),
        ));
    }
    Ok((receipt, canon, actual))
}

// ============================================================================
// Internal JSON decoder, path guards, and read-only git/cargo helpers
// ============================================================================

fn decode_receipt_json(val: &Json<'_>) -> Result<StackReceipt, ReceiptError> {
    let obj = val
        .as_object()
        .ok_or_else(|| ReceiptError::InvalidReceipt("receipt root must be JSON object".to_owned()))?;
    const EXPECTED_KEYS: &[&str] = &[
        "build_env", "commands", "duplicates", "lockfiles", "patches",
        "profile_overrides", "repo_dirty", "repo_rev", "repo_tree_hash",
        "schema", "siblings", "toolchain",
    ];
    for m in obj {
        if !EXPECTED_KEYS.contains(&m.name.as_str()) {
            return Err(ReceiptError::InvalidReceipt(format!("unknown receipt field `{}`", m.name)));
        }
    }

    let schema = req_json_str(val, "schema")?;
    if schema != STACK_RECEIPT_SCHEMA {
        return Err(ReceiptError::InvalidReceipt(format!(
            "unexpected receipt schema `{schema}` (expected `{STACK_RECEIPT_SCHEMA}`)"
        )));
    }
    let repo_rev = req_json_str(val, "repo_rev")?;
    let repo_dirty = req_json_bool(val, "repo_dirty")?;
    let repo_tree_hash = opt_json_str(val, "repo_tree_hash")?;
    let toolchain = req_json_str(val, "toolchain")?;
    let patches = req_json_str_vec(val, "patches")?;
    let profile_overrides = req_json_str_vec(val, "profile_overrides")?;
    let duplicates = req_json_str_vec(val, "duplicates")?;

    let mut build_env = BTreeMap::new();
    let env_members = val
        .get("build_env")
        .and_then(|v| v.as_object())
        .ok_or_else(|| ReceiptError::InvalidReceipt("missing or invalid `build_env`".to_owned()))?;
    for m in env_members {
        let s = m
            .value
            .as_str()
            .ok_or_else(|| ReceiptError::InvalidReceipt(format!("build_env.`{}` must be string", m.name)))?;
        build_env.insert(m.name.clone(), s.to_owned());
    }

    let mut siblings = Vec::new();
    let sib_arr = val
        .get("siblings")
        .and_then(|v| v.as_array())
        .ok_or_else(|| ReceiptError::InvalidReceipt("missing or invalid `siblings`".to_owned()))?;
    for item in sib_arr {
        let ol = item
            .get("own_lock")
            .ok_or_else(|| ReceiptError::InvalidReceipt("sibling missing `own_lock`".to_owned()))?;
        let own_lock = OwnLockEntry {
            file: opt_json_str(ol, "file")?,
            hash: opt_json_str(ol, "hash")?,
            agrees: req_json_bool(ol, "agrees")?,
            class: DriftClass::parse(&req_json_str(ol, "class")?)?,
            reason: opt_json_str(ol, "reason")?.map(|s| DriftReason::parse(&s)).transpose()?,
        };
        siblings.push(SiblingEntry {
            fd: req_json_str(item, "fd")?,
            rev: req_json_str(item, "rev")?,
            dirty: req_json_bool(item, "dirty")?,
            tree_hash: opt_json_str(item, "tree_hash")?,
            features: req_json_str_vec(item, "features")?,
            own_lock,
        });
    }

    let mut lockfiles = Vec::new();
    let lf_arr = val
        .get("lockfiles")
        .and_then(|v| v.as_array())
        .ok_or_else(|| ReceiptError::InvalidReceipt("missing or invalid `lockfiles`".to_owned()))?;
    for item in lf_arr {
        lockfiles.push(LockfileEntry {
            workspace: req_json_str(item, "workspace")?,
            hash: req_json_str(item, "hash")?,
            packages: req_json_int53(item, "packages")?,
            registry: req_json_int53(item, "registry")?,
        });
    }

    let mut commands = Vec::new();
    let cmd_arr = val
        .get("commands")
        .and_then(|v| v.as_array())
        .ok_or_else(|| ReceiptError::InvalidReceipt("missing or invalid `commands`".to_owned()))?;
    for item in cmd_arr {
        commands.push(CommandEntry {
            argv: req_json_str_vec(item, "argv")?,
            exit: req_json_int53(item, "exit")?,
            output_hash: req_json_str(item, "output_hash")?,
        });
    }

    Ok(StackReceipt {
        schema,
        repo_rev,
        repo_dirty,
        repo_tree_hash,
        toolchain,
        siblings,
        lockfiles,
        patches,
        profile_overrides,
        build_env,
        duplicates,
        commands,
    })
}

fn req_json_str(obj: &Json<'_>, key: &str) -> Result<String, ReceiptError> {
    obj.get(key)
        .and_then(|v| v.as_str())
        .map(ToOwned::to_owned)
        .ok_or_else(|| ReceiptError::InvalidReceipt(format!("missing/non-string `{key}`")))
}

fn opt_json_str(obj: &Json<'_>, key: &str) -> Result<Option<String>, ReceiptError> {
    let Some(v) = obj.get(key) else {
        return Err(ReceiptError::InvalidReceipt(format!("missing `{key}`")));
    };
    if v.is_null() {
        Ok(None)
    } else {
        v.as_str()
            .map(|s| Some(s.to_owned()))
            .ok_or_else(|| ReceiptError::InvalidReceipt(format!("`{key}` must be string or null")))
    }
}

fn req_json_bool(obj: &Json<'_>, key: &str) -> Result<bool, ReceiptError> {
    obj.get(key)
        .and_then(|v| v.as_bool())
        .ok_or_else(|| ReceiptError::InvalidReceipt(format!("missing/non-bool `{key}`")))
}

fn req_json_int53(obj: &Json<'_>, key: &str) -> Result<i64, ReceiptError> {
    obj.get(key)
        .ok_or_else(|| ReceiptError::InvalidReceipt(format!("missing `{key}`")))?
        .as_int53()
        .map_err(|e| ReceiptError::InvalidReceipt(format!("`{key}` int53: {e}")))
}

fn req_json_str_vec(obj: &Json<'_>, key: &str) -> Result<Vec<String>, ReceiptError> {
    let arr = obj
        .get(key)
        .and_then(|v| v.as_array())
        .ok_or_else(|| ReceiptError::InvalidReceipt(format!("missing/non-array `{key}`")))?;
    arr.iter()
        .map(|it| {
            it.as_str()
                .map(ToOwned::to_owned)
                .ok_or_else(|| ReceiptError::InvalidReceipt(format!("`{key}` element must be string")))
        })
        .collect()
}

fn validate_rel_path(label: &str, path: &str) -> Result<String, ReceiptError> {
    if path.is_empty()
        || path.starts_with('/')
        || path.starts_with('~')
        || path.contains('\\')
        || Path::new(path).is_absolute()
    {
        return Err(ReceiptError::AbsolutePatchPath {
            entry: label.to_owned(),
            path: path.to_owned(),
        });
    }
    let parts: Vec<&str> = path.split('/').filter(|p| !p.is_empty()).collect();
    if parts.is_empty() || parts == ["."] || parts == [".."] || parts[1..].contains(&"..") {
        return Err(ReceiptError::AbsolutePatchPath {
            entry: label.to_owned(),
            path: path.to_owned(),
        });
    }
    Ok(path.to_owned())
}

fn normalize_joined_path(base: &Path, rel: &str) -> PathBuf {
    let mut out = base.to_path_buf();
    for comp in Path::new(rel).components() {
        match comp {
            Component::CurDir => {}
            Component::ParentDir => {
                let _ = out.pop();
            }
            Component::Normal(seg) => out.push(seg),
            Component::RootDir | Component::Prefix(_) => out.push(comp.as_os_str()),
        }
    }
    out
}

fn scan_no_machine_paths(receipt: &StackReceipt) -> Result<(), ReceiptError> {
    let home = std::env::var("HOME").ok().filter(|h| h.len() > 1);
    let check = |field: &str, value: &str| -> Result<(), ReceiptError> {
        if value.starts_with('/')
            || value.contains("/Users/")
            || value.contains("/home/")
            || value.contains("/private/")
            || home.as_deref().is_some_and(|h| value.contains(h))
        {
            return Err(ReceiptError::MachinePathLeak {
                field: field.to_owned(),
                value: value.to_owned(),
            });
        }
        Ok(())
    };

    check("schema", &receipt.schema)?;
    check("repo_rev", &receipt.repo_rev)?;
    if let Some(h) = &receipt.repo_tree_hash { check("repo_tree_hash", h)?; }
    check("toolchain", &receipt.toolchain)?;
    for p in &receipt.patches { check("patches", p)?; }
    for ov in &receipt.profile_overrides { check("profile_overrides", ov)?; }
    for d in &receipt.duplicates { check("duplicates", d)?; }
    for (k, v) in &receipt.build_env {
        check("build_env.key", k)?;
        check("build_env.val", v)?;
    }
    for lf in &receipt.lockfiles {
        check("lockfiles.workspace", &lf.workspace)?;
        check("lockfiles.hash", &lf.hash)?;
    }
    for cmd in &receipt.commands {
        for arg in &cmd.argv { check("commands.argv", arg)?; }
        check("commands.output_hash", &cmd.output_hash)?;
    }
    for sib in &receipt.siblings {
        check("siblings.fd", &sib.fd)?;
        check("siblings.rev", &sib.rev)?;
        if let Some(h) = &sib.tree_hash { check("siblings.tree_hash", h)?; }
        for f in &sib.features { check("siblings.features", f)?; }
        if let Some(f) = &sib.own_lock.file { check("siblings.own_lock.file", f)?; }
        if let Some(h) = &sib.own_lock.hash { check("siblings.own_lock.hash", h)?; }
    }
    Ok(())
}

fn normalize_command_stdout(stdout: &[u8], root_dir: &Path) -> Vec<u8> {
    let mut roots = BTreeSet::new();
    roots.insert(root_dir.to_path_buf());
    if let Ok(canon) = fs::canonicalize(root_dir) {
        roots.insert(canon);
    }
    let mut parents = BTreeSet::new();
    for r in &roots {
        if let Some(p) = r.parent() {
            parents.insert(p.to_path_buf());
        }
    }

    let sep = std::path::MAIN_SEPARATOR_STR;
    let mut pairs: Vec<(Vec<u8>, &'static [u8])> = Vec::new();
    for r in &roots {
        let s = r.to_string_lossy();
        let clean = s.trim_end_matches(sep);
        if !clean.is_empty() {
            pairs.push((format!("{clean}{sep}").into_bytes(), b"./"));
            pairs.push((clean.as_bytes().to_vec(), b"."));
        }
    }
    for p in &parents {
        let s = p.to_string_lossy();
        let clean = s.trim_end_matches(sep);
        if !clean.is_empty() {
            pairs.push((format!("{clean}{sep}").into_bytes(), b"../"));
        }
    }
    pairs.sort_by(|a, b| b.0.len().cmp(&a.0.len()));

    let mut out = stdout.to_vec();
    for (needle, repl) in pairs {
        out = replace_bytes(&out, &needle, repl);
    }
    out
}

fn replace_bytes(haystack: &[u8], needle: &[u8], repl: &[u8]) -> Vec<u8> {
    if needle.is_empty() || needle.len() > haystack.len() {
        return haystack.to_vec();
    }
    let mut out = Vec::with_capacity(haystack.len());
    let mut i = 0;
    while i < haystack.len() {
        if i + needle.len() <= haystack.len() && &haystack[i..i + needle.len()] == needle {
            out.extend_from_slice(repl);
            i += needle.len();
        } else {
            out.push(haystack[i]);
            i += 1;
        }
    }
    out
}

fn parse_toml_file(path: &Path, label: &str) -> Result<Table, ReceiptError> {
    let bytes = read_file_bytes(path)?;
    toml::parse(&bytes).map_err(|e| ReceiptError::Toml(format!("{label}: {e}")))
}

fn read_file_bytes(path: &Path) -> Result<Vec<u8>, ReceiptError> {
    fs::read(path).map_err(|e| ReceiptError::Io(format!("read `{}`: {e}", path.display())))
}

fn blake3_file(path: &Path) -> Result<String, ReceiptError> {
    let mut f = File::open(path)
        .map_err(|e| ReceiptError::Io(format!("open `{}`: {e}", path.display())))?;
    let mut hasher = fs_blake3::Blake3::new();
    let mut buf = [0u8; 65_536];
    loop {
        let n = f
            .read(&mut buf)
            .map_err(|e| ReceiptError::Io(format!("read `{}`: {e}", path.display())))?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }
    Ok(hasher.finalize().to_hex())
}

fn toml_scalar_display(v: &TomlValue) -> String {
    match v {
        TomlValue::Str(s) => s.clone(),
        TomlValue::Int(i) => i.to_string(),
        TomlValue::Bool(b) => b.to_string(),
        TomlValue::Array(_) => "<array>".to_owned(),
        TomlValue::Table(_) => "<table>".to_owned(),
    }
}

fn is_lower_hex(s: &str, len: usize) -> bool {
    s.len() == len && s.bytes().all(|b| b.is_ascii_digit() || matches!(b, b'a'..=b'f'))
}

fn git_ro(repo_dir: &Path, args: &[&str]) -> Result<Vec<u8>, ReceiptError> {
    let out = Command::new("git")
        .arg("--no-optional-locks")
        .arg("-C")
        .arg(repo_dir)
        .args(args)
        .output()
        .map_err(|e| ReceiptError::Git(format!("spawn git {:?}: {e}", args)))?;
    if !out.status.success() {
        return Err(ReceiptError::Git(format!(
            "git {:?} failed: {}",
            args,
            String::from_utf8_lossy(&out.stderr).trim()
        )));
    }
    Ok(out.stdout)
}

fn git_ro_status(repo_dir: &Path, args: &[&str]) -> Result<bool, ReceiptError> {
    let out = Command::new("git")
        .arg("--no-optional-locks")
        .arg("-C")
        .arg(repo_dir)
        .args(args)
        .output()
        .map_err(|e| ReceiptError::Git(format!("spawn git {:?}: {e}", args)))?;
    Ok(out.status.success())
}

fn is_dirty_tracked(repo_dir: &Path) -> Result<bool, ReceiptError> {
    let out = git_ro(repo_dir, &["status", "--porcelain", "--untracked-files=no"])?;
    Ok(!out.iter().all(u8::is_ascii_whitespace))
}

fn head_rev(repo_dir: &Path) -> Result<String, ReceiptError> {
    let out = git_ro(repo_dir, &["rev-parse", "HEAD"])?;
    let rev = std::str::from_utf8(&out)
        .map_err(|e| ReceiptError::Git(format!("HEAD UTF-8: {e}")))?
        .trim()
        .to_owned();
    if !is_lower_hex(&rev, 40) {
        return Err(ReceiptError::Git(format!("HEAD is not 40-hex commit: `{rev}`")));
    }
    Ok(rev)
}

struct CmdOutput {
    exit: i32,
    stdout: Vec<u8>,
    stderr: Vec<u8>,
}

fn run_cargo_ro(ws_dir: &Path, args: &[&str]) -> Result<CmdOutput, ReceiptError> {
    // Always invoke `cargo` via PATH so it flows through the build-offload shim.
    let out = Command::new("cargo")
        .args(args)
        .current_dir(ws_dir)
        .output()
        .map_err(|e| ReceiptError::Cargo(format!("spawn cargo {:?}: {e}", args)))?;
    Ok(CmdOutput {
        exit: out.status.code().unwrap_or(1),
        stdout: out.stdout,
        stderr: out.stderr,
    })
}

// ============================================================================
// Unit tests (`cargo test -p xtask stack::receipt`; synthetic git fixtures)
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn fresh_dir(tag: &str) -> PathBuf {
        static CTR: AtomicU64 = AtomicU64::new(0);
        let dir = std::env::temp_dir().join(format!(
            "frob_xtask_receipt_{tag}_{}_{}",
            std::process::id(),
            CTR.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(&dir).expect("create synthetic fixture dir");
        dir
    }

    fn git_fixture(dir: &Path, args: &[&str]) -> String {
        let out = Command::new("git")
            .arg("-C")
            .arg(dir)
            .args(["-c", "commit.gpgsign=false", "-c", "core.autocrlf=false"])
            .args(args)
            .env("GIT_AUTHOR_NAME", "synthetic")
            .env("GIT_AUTHOR_EMAIL", "synthetic@example.com")
            .env("GIT_AUTHOR_DATE", "2026-11-24T00:00:00Z")
            .env("GIT_COMMITTER_NAME", "synthetic")
            .env("GIT_COMMITTER_EMAIL", "synthetic@example.com")
            .env("GIT_COMMITTER_DATE", "2026-11-24T00:00:00Z")
            .output()
            .expect("spawn synthetic git");
        assert!(
            out.status.success(),
            "synthetic git {:?} failed: {}",
            args,
            String::from_utf8_lossy(&out.stderr)
        );
        String::from_utf8_lossy(&out.stdout).trim().to_owned()
    }

    fn init_git_repo(dir: &Path) {
        fs::create_dir_all(dir).expect("mkdir repo");
        git_fixture(dir, &["init", "-q", "-b", "main"]);
    }

    fn commit_file(dir: &Path, rel: &str, content: &str, msg: &str) -> String {
        let path = dir.join(rel);
        if let Some(p) = path.parent() {
            fs::create_dir_all(p).expect("mkdir parent");
        }
        fs::write(&path, content).expect("write fixture file");
        git_fixture(dir, &["add", "--", rel]);
        git_fixture(dir, &["commit", "-q", "-m", msg]);
        git_fixture(dir, &["rev-parse", "HEAD"])
    }

    /// Build a minimal self-contained synthetic workspace + sibling checkout
    /// that resolves offline with `cargo metadata --locked --offline` and
    /// `cargo tree --locked --offline -d` (zero external dependencies).
    fn build_synthetic_stack(parent: &Path, patch_path: &str) -> (PathBuf, PathBuf, String) {
        let sib_dir = parent.join("sib_crate");
        init_git_repo(&sib_dir);
        fs::write(
            sib_dir.join("Cargo.toml"),
            "[package]\nname = \"sib-crate\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
        )
        .expect("sib Cargo.toml");
        fs::create_dir_all(sib_dir.join("src")).expect("sib src");
        fs::write(sib_dir.join("src/lib.rs"), "pub const X: u32 = 1;\n").expect("sib lib.rs");
        git_fixture(&sib_dir, &["add", "."]);
        git_fixture(&sib_dir, &["commit", "-q", "-m", "init sib"]);
        let sib_rev = git_fixture(&sib_dir, &["rev-parse", "HEAD"]);

        let repo_dir = parent.join("repo");
        init_git_repo(&repo_dir);
        fs::write(
            repo_dir.join("rust-toolchain.toml"),
            "[toolchain]\nchannel = \"nightly-2026-08-31\"\n",
        )
        .expect("toolchain");
        fs::create_dir_all(repo_dir.join(".cargo")).expect(".cargo");
        fs::write(
            repo_dir.join(".cargo/config.toml"),
            "[env]\nFRANKENSIM_DEPGRAPH_SALT = \"synthetic-v1\"\n",
        )
        .expect("cargo config");
        fs::write(
            repo_dir.join("Cargo.toml"),
            format!(
                "[workspace]\nresolver = \"3\"\nmembers = [\"crates/app\"]\n\n\
[patch.crates-io]\nsib-crate = {{ path = \"{patch_path}\" }}\n\n\
[profile.dev.package.fsqlite]\nopt-level = 1\n"
            ),
        )
        .expect("root Cargo.toml");
        fs::create_dir_all(repo_dir.join("crates/app/src")).expect("app src");
        fs::write(
            repo_dir.join("crates/app/Cargo.toml"),
            "[package]\nname = \"app\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\nsib-crate = { path = \"../../../sib_crate\" }\n",
        )
        .expect("app Cargo.toml");
        fs::write(repo_dir.join("crates/app/src/lib.rs"), "pub fn v() -> u32 { sib_crate::X }\n")
            .expect("app lib.rs");
        fs::write(
            repo_dir.join("Cargo.lock"),
            "# This file is automatically @generated by Cargo.\nversion = 4\n\n\
[[package]]\nname = \"app\"\nversion = \"0.1.0\"\ndependencies = [\n \"sib-crate\",\n]\n\n\
[[package]]\nname = \"sib-crate\"\nversion = \"0.1.0\"\n\n\
[[patch.unused]]\nname = \"sib-crate\"\nversion = \"0.1.0\"\n",
        )
        .expect("Cargo.lock");
        fs::write(
            repo_dir.join("franken-stack.lock"),
            format!(
                "toolchain = \"nightly-2026-08-31\"\n\n\
[[foundation]]\nid = \"FD-SIB\"\nname = \"sib-crate\"\nposture = \"consume-path\"\n\
path = \"../sib_crate\"\ngit_rev = \"{sib_rev}\"\nconfined_to = [\"app\"]\n\
admission_gate = \"INT-SIB-01\"\nfallback = \"synthetic fallback\"\n"
            ),
        )
        .expect("stack lock");

        git_fixture(&repo_dir, &["add", "."]);
        git_fixture(&repo_dir, &["commit", "-q", "-m", "init repo"]);
        (repo_dir, sib_dir, sib_rev)
    }

    fn make_three_commit_history(tag: &str) -> (PathBuf, String, String, String) {
        let dir = fresh_dir(tag);
        init_git_repo(&dir);
        let c1 = commit_file(&dir, "a.txt", "one\n", "c1");
        let c2 = commit_file(&dir, "a.txt", "two\n", "c2");
        let c3 = commit_file(&dir, "a.txt", "three\n", "c3");
        (dir, c1, c2, c3)
    }

    #[test]
    fn receipt_idempotent() {
        let base = fresh_dir("idempotent");
        let (repo, _sib, _) = build_synthetic_stack(&base, "../sib_crate");
        let lock = repo.join("franken-stack.lock");
        let (r1, b1, h1) = build_receipt(&repo, &lock).expect("run 1");
        let (r2, b2, h2) = build_receipt(&repo, &lock).expect("run 2");
        assert_eq!(r1, r2);
        assert_eq!(b1.as_bytes(), b2.as_bytes());
        assert_eq!(h1, h2);
        assert!(!r1.repo_dirty);
        assert_eq!(r1.repo_tree_hash, None);
    }

    #[test]
    fn receipt_dirty_flips() {
        let base = fresh_dir("dirty_flips");
        let (repo, sib, _) = build_synthetic_stack(&base, "../sib_crate");
        let lock = repo.join("franken-stack.lock");
        let (clean, _, _) = build_receipt(&repo, &lock).expect("clean");
        assert!(!clean.siblings[0].dirty);
        assert_eq!(clean.siblings[0].tree_hash, None);

        fs::write(sib.join("src/lib.rs"), "pub const X: u32 = 2;\n").expect("edit 1");
        let (d1, _, _) = build_receipt(&repo, &lock).expect("dirty 1");
        assert!(d1.siblings[0].dirty);
        let h1 = d1.siblings[0].tree_hash.clone().expect("dirty tree_hash 1");

        fs::write(sib.join("src/lib.rs"), "pub const X: u32 = 3;\n").expect("edit 2");
        let (d2, _, _) = build_receipt(&repo, &lock).expect("dirty 2");
        assert!(d2.siblings[0].dirty);
        let h2 = d2.siblings[0].tree_hash.clone().expect("dirty tree_hash 2");
        assert_ne!(h1, h2);
    }

    #[test]
    fn tree_hash_ignores_untracked_and_mtime() {
        let dir = fresh_dir("untracked_mtime");
        init_git_repo(&dir);
        let _ = commit_file(&dir, "tracked.txt", "hello\n", "init");
        let h1 = tree_hash(&dir).expect("h1");

        fs::write(dir.join("untracked.tmp"), "noise").expect("add untracked");
        // Rewrite same bytes to advance filesystem mtime without changing tracked content.
        fs::write(dir.join("tracked.txt"), "hello\n").expect("touch tracked");
        let h2 = tree_hash(&dir).expect("h2");
        assert_eq!(h1, h2);
    }

    #[test]
    fn tree_hash_sees_mode_and_deletion() {
        let base = fresh_dir("mode_del");
        let repo = base.join("repo");
        let aside = base.join("aside");
        fs::create_dir_all(&aside).expect("mkdir aside");
        init_git_repo(&repo);
        let _ = commit_file(&repo, "run.sh", "#!/bin/sh\necho hi\n", "init");
        let h_normal = tree_hash(&repo).expect("normal");

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let p = repo.join("run.sh");
            fs::set_permissions(&p, fs::Permissions::from_mode(0o755)).expect("chmod +x");
            let h_exec = tree_hash(&repo).expect("exec");
            assert_ne!(h_normal, h_exec, "executable bit toggle must change tree_hash");
            fs::set_permissions(&p, fs::Permissions::from_mode(0o644)).expect("chmod -x");
            assert_eq!(h_normal, tree_hash(&repo).expect("restored"));
        }

        // Move tracked file outside the repo (simulates working-tree deletion without deleting any file).
        fs::rename(repo.join("run.sh"), aside.join("run.sh")).expect("move aside");
        let h_deleted = tree_hash(&repo).expect("deleted");
        assert_ne!(h_normal, h_deleted, "tracked deletion must change tree_hash");

        // Verify empty file hashes differently from a deleted file (0x00+len0 vs 0xFF).
        fs::write(repo.join("run.sh"), "").expect("empty file");
        let h_empty = tree_hash(&repo).expect("empty");
        assert_ne!(h_deleted, h_empty);
    }

    #[test]
    fn tree_hash_length_prefix_prevents_ambiguity() {
        let r1 = fresh_dir("ambig_1");
        init_git_repo(&r1);
        let _ = commit_file(&r1, "ab", "c", "c1");

        let r2 = fresh_dir("ambig_2");
        init_git_repo(&r2);
        let _ = commit_file(&r2, "a", "bc", "c1");

        assert_ne!(tree_hash(&r1).expect("h1"), tree_hash(&r2).expect("h2"));
    }

    #[test]
    fn drift_on_pin() {
        let (dir, _, _, c3) = make_three_commit_history("drift_on_pin");
        assert_eq!(classify_checkout(&dir, Some(&c3)).expect("classify"), (DriftClass::OnPin, None));
    }

    #[test]
    fn drift_stale_lock_fast_forward() {
        let (dir, _, c2, _) = make_three_commit_history("drift_stale_ff");
        assert_eq!(
            classify_checkout(&dir, Some(&c2)).expect("classify"),
            (DriftClass::StaleLock, None)
        );
    }

    #[test]
    fn drift_retreated() {
        let (dir, _, c2, c3) = make_three_commit_history("drift_retreated");
        git_fixture(&dir, &["switch", "--detach", "-q", &c2]);
        assert_eq!(
            classify_checkout(&dir, Some(&c3)).expect("classify"),
            (DriftClass::Drift, Some(DriftReason::Retreated))
        );
    }

    #[test]
    fn drift_diverged() {
        let (dir, c1, _, c3) = make_three_commit_history("drift_diverged");
        git_fixture(&dir, &["switch", "-q", "-c", "side", &c1]);
        let _side = commit_file(&dir, "a.txt", "side-branch\n", "side");
        assert_eq!(
            classify_checkout(&dir, Some(&c3)).expect("classify"),
            (DriftClass::Drift, Some(DriftReason::Diverged))
        );
    }

    #[test]
    fn drift_dirty() {
        let (dir, _, c2, c3) = make_three_commit_history("drift_dirty");
        // Retreated AND dirty: `dirty` must take precedence over `retreated`.
        git_fixture(&dir, &["switch", "--detach", "-q", &c2]);
        fs::write(dir.join("a.txt"), "uncommitted\n").expect("dirty edit");
        assert_eq!(
            classify_checkout(&dir, Some(&c3)).expect("classify"),
            (DriftClass::Drift, Some(DriftReason::Dirty))
        );
    }

    #[test]
    fn drift_unknown_pin() {
        let (dir, _, _, _) = make_three_commit_history("drift_unknown");
        let missing = "0123456789abcdef0123456789abcdef01234567";
        assert_eq!(
            classify_checkout(&dir, Some(missing)).expect("classify"),
            (DriftClass::Drift, Some(DriftReason::UnknownPin))
        );
    }

    #[test]
    fn own_lock_classified() {
        let parent = fresh_dir("own_lock");
        let dep = parent.join("dep_lib");
        init_git_repo(&dep);
        let d1 = commit_file(&dep, "lib.txt", "v1\n", "d1");
        let d2 = commit_file(&dep, "lib.txt", "v2\n", "d2");

        let sib = parent.join("sib_main");
        init_git_repo(&sib);
        let s1 = commit_file(
            &sib,
            "constellation.lock",
            &format!("{{\"libraries\":[{{\"lib\":\"dep_lib\",\"git_head\":\"{d1}\"}}]}}\n"),
            "pin d1",
        );
        let ol_stale = classify_sibling_own_lock(&sib, Some(&s1)).expect("stale own_lock");
        assert_eq!(ol_stale.file.as_deref(), Some("constellation.lock"));
        assert_eq!(ol_stale.class, DriftClass::StaleLock);
        assert!(!ol_stale.agrees);
        assert_eq!(ol_stale.reason, None);

        let s2 = commit_file(
            &sib,
            "constellation.lock",
            &format!("{{\"libraries\":[{{\"lib\":\"dep_lib\",\"git_head\":\"{d2}\"}}]}}\n"),
            "pin d2",
        );
        let ol_pin = classify_sibling_own_lock(&sib, Some(&s2)).expect("on-pin own_lock");
        assert_eq!(ol_pin.class, DriftClass::OnPin);
        assert!(ol_pin.agrees);
        assert_eq!(ol_pin.reason, None);
    }

    #[test]
    fn receipt_roundtrips_through_core_writer() {
        let base = fresh_dir("roundtrip");
        let (repo, _, _) = build_synthetic_stack(&base, "../sib_crate");
        let (receipt, canon_bytes, _) =
            build_receipt(&repo, &repo.join("franken-stack.lock")).expect("build");

        let parsed = StackReceipt::from_canonical_bytes(canon_bytes.as_bytes()).expect("parse");
        assert_eq!(parsed, receipt);
        let re_written = canon::to_canonical(&parsed).expect("re-write via frob_core::canon");
        assert_eq!(re_written.as_bytes(), canon_bytes.as_bytes());

        // Also round-trip the committed bootstrap receipt through frob_core::canon
        // and verify committed franken-stack.lock rows cite its hash.
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let boot_path = root.join(
            "receipts/d3b0885b1d5bfbe09039a84ae190c40fce4ad3b0b81b5c28bf946201987462d4.json",
        );
        let (boot, boot_canon, boot_hex) = load_and_verify_receipt(&boot_path).expect("bootstrap");
        assert_eq!(boot_hex, "d3b0885b1d5bfbe09039a84ae190c40fce4ad3b0b81b5c28bf946201987462d4");
        assert_eq!(canon::to_canonical(&boot).expect("re-canon").as_bytes(), boot_canon.as_bytes());
        let repo_lock = lock::load(&root.join("franken-stack.lock")).expect("repo stack lock");
        assert!(
            repo_lock
                .foundations
                .iter()
                .any(|f| f.receipt.as_ref().map(|r| r.as_str()) == Some(boot_hex.as_str())),
            "committed franken-stack.lock must cite the bootstrap receipt hash"
        );
    }

    #[cfg(unix)]
    fn set_tree_writable(dir: &Path, writable: bool) {
        use std::os::unix::fs::PermissionsExt;
        let mut dirs = Vec::new();
        let mut stack = vec![dir.to_path_buf()];
        while let Some(cur) = stack.pop() {
            if writable {
                let _ = fs::set_permissions(&cur, fs::Permissions::from_mode(0o755));
            }
            if let Ok(entries) = fs::read_dir(&cur) {
                for ent in entries.flatten() {
                    let p = ent.path();
                    if p.is_dir() {
                        stack.push(p);
                    } else if writable {
                        let _ = fs::set_permissions(&p, fs::Permissions::from_mode(0o644));
                    } else {
                        let _ = fs::set_permissions(&p, fs::Permissions::from_mode(0o444));
                    }
                }
            }
            if !writable {
                dirs.push(cur);
            }
        }
        for d in dirs.into_iter().rev() {
            let _ = fs::set_permissions(&d, fs::Permissions::from_mode(0o555));
        }
    }

    fn digest_entire_dir_including_git(dir: &Path) -> String {
        let mut paths = Vec::new();
        let mut stack = vec![dir.to_path_buf()];
        while let Some(cur) = stack.pop() {
            if let Ok(entries) = fs::read_dir(&cur) {
                for ent in entries.flatten() {
                    let p = ent.path();
                    if p.is_dir() {
                        stack.push(p);
                    } else {
                        paths.push(p);
                    }
                }
            }
        }
        paths.sort();
        let mut hasher = fs_blake3::Blake3::new();
        for p in paths {
            let rel = p.strip_prefix(dir).expect("strip").to_string_lossy();
            hasher.update(rel.as_bytes());
            hasher.update(&[0]);
            if let Ok(bytes) = fs::read(&p) {
                hasher.update(&bytes);
            }
            hasher.update(&[0]);
        }
        hasher.finalize().to_hex()
    }

    #[test]
    fn receipt_no_machine_paths() {
        let root_a = fresh_dir("loc_a").join("a").join("b");
        let root_b = fresh_dir("loc_b").join("c");
        fs::create_dir_all(&root_a).expect("mkdir a");
        fs::create_dir_all(&root_b).expect("mkdir b");

        let (repo_a, sib_a, _) = build_synthetic_stack(&root_a, "../sib_crate");
        let (repo_b, _, _) = build_synthetic_stack(&root_b, "../sib_crate");

        // Verify recursively read-only sibling checkout succeeds and stays byte-identical (.git included).
        let before_sib_digest = digest_entire_dir_including_git(&sib_a);
        #[cfg(unix)]
        set_tree_writable(&sib_a, false);
        let (receipt_a, bytes_a, hash_a) =
            build_receipt(&repo_a, &repo_a.join("franken-stack.lock")).expect("a");
        #[cfg(unix)]
        set_tree_writable(&sib_a, true);
        let after_sib_digest = digest_entire_dir_including_git(&sib_a);
        assert_eq!(before_sib_digest, after_sib_digest, "sibling checkout must remain byte-identical");

        let (_, bytes_b, hash_b) = build_receipt(&repo_b, &repo_b.join("franken-stack.lock")).expect("b");
        assert_eq!(bytes_a.as_bytes(), bytes_b.as_bytes(), "receipt must be path-independent");
        assert_eq!(hash_a, hash_b);

        let text = std::str::from_utf8(bytes_a.as_bytes()).expect("utf8");
        assert!(!text.contains("/Users/"));
        assert!(!text.contains("/home/"));
        assert!(!text.contains("/private/"));
        if let Ok(host) = std::env::var("HOSTNAME")
            && host.len() > 2
        {
            assert!(!text.contains(&host), "hostname leaked into receipt");
        }
        for patch in &receipt_a.patches {
            let rhs = patch.split(" -> ").nth(1).unwrap_or(patch);
            assert!(!rhs.starts_with('/'), "patch path must not begin with '/': {patch}");
        }
    }

    #[test]
    fn receipt_refuses_absolute_patch_path() {
        let base = fresh_dir("abs_patch");
        let (repo, _, _) = build_synthetic_stack(&base, "/abs/asupersync");
        let err = build_receipt(&repo, &repo.join("franken-stack.lock"))
            .expect_err("absolute patch path must fail");
        assert!(
            matches!(err, ReceiptError::AbsolutePatchPath { ref entry, .. } if entry.contains("sib-crate")),
            "expected AbsolutePatchPath naming entry, got {err:?}"
        );

        let good_toml = toml::parse(
            b"[patch.crates-io]\nasupersync = { path = \"../asupersync\" }\n",
        )
        .expect("parse good patch");
        let (patches, _) = parse_patches(&good_toml).expect("relative patch");
        assert_eq!(patches, vec!["asupersync -> ../asupersync"]);
    }

    #[test]
    fn receipt_canonical_keys_sorted() {
        let base = fresh_dir("keys_sorted");
        let (repo, _, _) = build_synthetic_stack(&base, "../sib_crate");
        let (_, bytes, _) = build_receipt(&repo, &repo.join("franken-stack.lock")).expect("build");
        let text = std::str::from_utf8(bytes.as_bytes()).expect("utf8");
        assert!(!text.contains(": "), "canonical JSON must have no space after colon");
        assert!(!text.ends_with('\n'), "canonical JSON must have no trailing newline");

        let admitted = json::admit(bytes.as_bytes(), &Limits::default()).expect("admit");
        fn assert_sorted(v: &Json<'_>) {
            match v {
                Json::Obj(members) => {
                    for win in members.windows(2) {
                        assert!(
                            win[0].name.as_bytes() < win[1].name.as_bytes(),
                            "keys not code-point sorted: {:?} >= {:?}",
                            win[0].name,
                            win[1].name
                        );
                    }
                    for m in members {
                        assert_sorted(&m.value.value);
                    }
                }
                Json::Arr(items) => {
                    for it in items {
                        assert_sorted(&it.value);
                    }
                }
                _ => {}
            }
        }
        assert_sorted(admitted.value());
    }

    #[test]
    fn receipt_duplicates_lists_asupersync_first() {
        let sample = b"serde_json v1.0.151\n\
\xe2\x94\x94\xe2\x94\x80\xe2\x94\x80 ftx-drape v0.1.0\n\n\
serde_json v1.0.154\n\
\xe2\x94\x94\xe2\x94\x80\xe2\x94\x80 frob-io v0.0.1\n\n\
asupersync v0.6.2\n\
\xe2\x94\x94\xe2\x94\x80\xe2\x94\x80 fs-exec v0.1.0\n\n\
asupersync v0.5.0\n\
\xe2\x94\x94\xe2\x94\x80\xe2\x94\x80 frankentui v0.1.0\n";
        let dups = parse_duplicates(sample);
        assert_eq!(
            dups,
            vec![
                "asupersync 0.5.0",
                "asupersync 0.6.2",
                "serde_json 1.0.151",
                "serde_json 1.0.154",
            ]
        );
    }

    #[test]
    fn receipt_store_is_content_addressed() {
        let base = fresh_dir("store_cas");
        let (repo, _, _) = build_synthetic_stack(&base, "../sib_crate");
        let (_, bytes, digest) = build_receipt(&repo, &repo.join("franken-stack.lock")).expect("build");
        let store = base.join("receipts");

        let (p1, h1) = write_receipt(&store, &bytes).expect("write 1");
        let (p2, h2) = write_receipt(&store, &bytes).expect("write 2");
        assert_eq!(p1, p2);
        assert_eq!(h1, digest);
        assert_eq!(h2, digest);

        let count = fs::read_dir(&store).expect("read_dir").count();
        assert_eq!(count, 1, "writing identical receipt twice must create one file");

        let (_, _, verified_hex) = load_and_verify_receipt(&p1).expect("verify intact");
        assert_eq!(verified_hex, digest);

        // Truncate existing receipt file and confirm re-hashing on read detects it.
        fs::write(&p1, &bytes.as_bytes()[..32]).expect("truncate");
        let err = load_and_verify_receipt(&p1).expect_err("truncated receipt must fail");
        assert!(matches!(err, ReceiptError::HashMismatch { .. }), "got {err:?}");
    }
}
