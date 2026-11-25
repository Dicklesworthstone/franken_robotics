//! Strict `franken-stack.lock` (`frob.stack-lock/1`) schema, typed model,
//! parser, validation rules 1–11, canonical serializer, stack-lock digest
//! (`fs-blake3`), admission checks, and Draft 2020-12 JSON Schema emitter
//! (bead `N33a-03`).
//!
//! Uses only `std`, `frob_core::registry::toml` (single TOML reader, `N34p2-01`),
//! `fs_blake3`, and `crate::stack::posture` (`N33a-01`). Pure (D0), zero
//! `unsafe`, prints nothing (rule L-7), and `load`/`parse_bytes` never read any
//! admission record or directory.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::fs;
use std::path::Path;

use frob_core::registry::toml::{self, Fields, Spanned, Table, TomlError, TomlErrorKind, Value};

use crate::stack::posture::{Milestone, Posture, Tier, Use, is_exact_semver};

/// Schema identifier and BLAKE3 domain tag for `franken-stack.lock`.
pub const STACK_LOCK_SCHEMA: &str = "frob.stack-lock/1";

/// Valid FrankenSim (`FD-FSIM`) airlock feature-group names (`§33.5`, rule 7).
pub const FSIM_AIRLOCK_FEATURE_GROUPS: &[&str] = &[
    "referee", "geometry", "ledger", "optimize", "stats",
    "spectral", "actuation", "structure", "multiphysics",
];

/// Default FrankenSim airlock feature-group list compiled by default (rule 7).
pub const FSIM_DEFAULT_FEATURES: &[&str] = &["referee", "geometry", "ledger"];

/// Closed top-level keys accepted by `frob.stack-lock/1`.
pub const TOP_LEVEL_KEYS: &[&str] = &[
    "toolchain", "foundation", "freeze", "json_library", "auditor",
];

/// Closed `[[foundation]]` row keys accepted by `frob.stack-lock/1`.
pub const FOUNDATION_KEYS: &[&str] = &[
    "id", "name", "posture", "uses", "path", "git_rev", "version", "toolchain",
    "features", "build_env", "confined_to", "admission_gate", "receipt",
    "fallback", "first_needed", "critical_path", "note",
    "required_at", "optional_at", "firewall_cells",
];

/// Closed keys accepted inside each `[[foundation.uses]]` entry.
pub const USE_KEYS: &[&str] = &["posture", "purpose", "confined_to", "tier", "first_needed"];
/// Closed keys accepted inside `[auditor]` (`N34-25`).
pub const AUDITOR_KEYS: &[&str] = &["stable"];
/// Closed keys accepted inside `[freeze]` (`N33b-03`).
pub const FREEZE_KEYS: &[&str] = &["campaign", "receipt", "opened", "closed", "reason"];
/// Closed keys accepted inside `[json_library]` (`N34-04`).
pub const JSON_LIBRARY_KEYS: &[&str] = &["package", "version", "allowed_in", "features"];

/// Structured validation or load error (`rule`: `1..=11`; `0` for I/O or TOML syntax).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LockError {
    pub rule: u8,
    pub kind: &'static str,
    pub row: String,
    pub key: String,
    pub message: String,
}

impl LockError {
    #[must_use]
    pub fn new(
        rule: u8,
        kind: &'static str,
        row: impl Into<String>,
        key: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self { rule, kind, row: row.into(), key: key.into(), message: message.into() }
    }
}

impl fmt::Display for LockError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "rule {} ({}) in `{}` key `{}`: {}", self.rule, self.kind, self.row, self.key, self.message)
    }
}

impl std::error::Error for LockError {}

macro_rules! impl_str_newtype {
    ($($ty:ident),+ $(,)?) => {$(
        impl $ty {
            #[must_use]
            pub fn as_str(&self) -> &str { &self.0 }
        }
        impl fmt::Display for $ty {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.write_str(&self.0) }
        }
        impl AsRef<str> for $ty {
            fn as_ref(&self) -> &str { &self.0 }
        }
    )+};
}

/// Validated Rust toolchain channel (`nightly-YYYY-MM-DD` or stable `MAJOR.MINOR[.PATCH]`).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Channel(String);

/// Validated foundation identifier (`"FD-"` + upper-case ASCII letters/digits).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FdId(String);

/// Validated relative sibling path beginning with `../` with no later `..` (rule 4).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RelPath(String);

/// Validated exact Cargo version pin (`=MAJOR.MINOR.PATCH`, rule 6).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExactVersion(String);

/// Admission gate reference (`"INT-…"`, range `"INT-FSIM-01..18"`, or `"none"`).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GateRef(String);

/// Validated 64 lower-case hex BLAKE3 digest (rule 10).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Blake3Hex(String);

impl_str_newtype!(Channel, FdId, RelPath, ExactVersion, GateRef, Blake3Hex);

impl Channel {
    pub fn parse(raw: &str, row: &str, key: &str) -> Result<Self, LockError> {
        if is_valid_nightly_channel(raw) || is_valid_stable_channel(raw) {
            Ok(Self(raw.to_owned()))
        } else {
            Err(LockError::new(9, "invalid-toolchain", row, key,
                format!("invalid toolchain `{raw}` (expected `nightly-YYYY-MM-DD` or stable `MAJOR.MINOR[.PATCH]`)")))
        }
    }

    #[must_use]
    pub fn is_nightly(&self) -> bool { self.0.starts_with("nightly-") }

    #[must_use]
    pub fn is_stable(&self) -> bool { !self.is_nightly() }
}

impl FdId {
    pub fn parse(raw: &str, row: &str) -> Result<Self, LockError> {
        let ok = raw.strip_prefix("FD-").is_some_and(|rest| {
            !rest.is_empty() && rest.bytes().all(|b| b.is_ascii_uppercase() || b.is_ascii_digit())
        });
        if ok {
            Ok(Self(raw.to_owned()))
        } else {
            Err(LockError::new(2, "invalid-id", row, "id",
                format!("invalid foundation id `{raw}` (expected `FD-` + upper-case letters/digits)")))
        }
    }
}

impl RelPath {
    pub fn parse(raw: &str, row: &str) -> Result<Self, LockError> {
        let bad_prefix = raw.is_empty() || raw.starts_with('/') || raw.starts_with('~')
            || raw.contains('\\') || raw.contains("://")
            || raw.bytes().any(|b| b < 0x20 || b == 0x7F);
        let Some(rest) = (!bad_prefix).then(|| raw.strip_prefix("../")).flatten() else {
            return Err(LockError::new(4, "invalid-path", row, "path",
                format!("path `{raw}` must be a relative sibling path starting with `../`")));
        };
        if rest.is_empty() || rest.split('/').any(|s| s.is_empty() || s == "." || s == "..") {
            return Err(LockError::new(4, "invalid-path", row, "path",
                format!("path `{raw}` must contain no empty, `.`, or `..` component after `../`")));
        }
        Ok(Self(raw.to_owned()))
    }
}

/// Pinned sibling commit (40 lower-case hex digits) or literal `"unadmitted"` (rule 3).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GitRev {
    Commit(String),
    Unadmitted,
}

impl GitRev {
    pub fn parse(raw: &str, row: &str) -> Result<Self, LockError> {
        if raw == "unadmitted" {
            Ok(Self::Unadmitted)
        } else if is_lower_hex(raw, 40) {
            Ok(Self::Commit(raw.to_owned()))
        } else {
            Err(LockError::new(3, "invalid-git-rev", row, "git_rev",
                format!("invalid git_rev `{raw}` (expected 40 lower-case hex digits or `unadmitted`)")))
        }
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        match self { Self::Commit(h) => h.as_str(), Self::Unadmitted => "unadmitted" }
    }

    #[must_use]
    pub const fn is_unadmitted(&self) -> bool { matches!(self, Self::Unadmitted) }
}

impl fmt::Display for GitRev {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.write_str(self.as_str()) }
}

impl ExactVersion {
    pub fn parse(raw: &str, row: &str) -> Result<Self, LockError> {
        if is_exact_semver(raw) && semver_suffix_valid(raw) {
            Ok(Self(raw.to_owned()))
        } else {
            Err(LockError::new(6, "invalid-version", row, "version",
                format!("version `{raw}` must be an exact pin `=MAJOR.MINOR.PATCH`")))
        }
    }
}

impl GateRef {
    pub fn parse(raw: &str, row: &str) -> Result<Self, LockError> {
        if raw == "none" || is_valid_int_gate_ref(raw) {
            Ok(Self(raw.to_owned()))
        } else {
            Err(LockError::new(1, "invalid-admission-gate", row, "admission_gate",
                format!("invalid admission_gate `{raw}` (expected `INT-<GROUP>-<NN>[..<MM>]` or `none`)")))
        }
    }

    #[must_use]
    pub fn is_none(&self) -> bool { self.0 == "none" }

    /// Return `true` when this gate ref (or any gate within its numeric range) is in `admitted`.
    #[must_use]
    pub fn is_admitted_in(&self, admitted: &BTreeSet<Self>) -> bool {
        if self.is_none() { return false; }
        if admitted.contains(self) { return true; }
        let Some((left, right)) = self.0.split_once("..") else { return false; };
        let Some((prefix, start_s)) = left.rsplit_once('-') else { return false; };
        let (Ok(start), Ok(end)) = (start_s.parse::<u32>(), right.parse::<u32>()) else { return false; };
        let width = start_s.len();
        admitted.iter().any(|c| {
            c.0.strip_prefix(prefix)
                .and_then(|r| r.strip_prefix('-'))
                .is_some_and(|ns| {
                    ns.len() >= width
                        && ns.bytes().all(|b| b.is_ascii_digit())
                        && ns.parse::<u32>().is_ok_and(|n| n >= start && n <= end)
                })
        })
    }
}

impl Blake3Hex {
    pub fn parse(raw: &str, row: &str, key: &str) -> Result<Self, LockError> {
        if is_lower_hex(raw, 64) {
            Ok(Self(raw.to_owned()))
        } else {
            Err(LockError::new(10, "invalid-receipt", row, key,
                format!("invalid receipt `{raw}` (expected 64 lower-case hex digits)")))
        }
    }
}

/// Hooked `[auditor]` top-level table (`N34-25`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuditorTable {
    pub stable: Channel,
}

/// Hooked `[freeze]` top-level table (`N33b-03`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FreezeTable {
    pub campaign: String,
    pub receipt: Blake3Hex,
    pub opened: String,
    pub closed: String,
    pub reason: String,
}

/// Hooked `[json_library]` top-level table (`N34-04`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsonLibraryTable {
    pub package: String,
    pub version: String,
    pub allowed_in: Vec<String>,
    pub features: BTreeMap<String, Vec<String>>,
}

/// One `[[foundation]]` row of `franken-stack.lock` (`frob.stack-lock/1`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Foundation {
    pub id: FdId,
    pub name: String,
    pub uses: Vec<Use>,
    pub path: Option<RelPath>,
    pub git_rev: Option<GitRev>,
    pub version: Option<ExactVersion>,
    pub toolchain: Option<Channel>,
    pub features: Vec<String>,
    pub build_env: BTreeMap<String, String>,
    pub confined_to: Vec<String>,
    pub admission_gate: GateRef,
    pub receipt: Option<Blake3Hex>,
    pub fallback: String,
    pub first_needed: Option<Milestone>,
    pub critical_path: bool,
    pub note: Option<String>,
    /// Hooked dependency-firewall field (`N33b-01`).
    pub required_at: Option<Vec<Milestone>>,
    /// Hooked dependency-firewall field (`N33b-01`).
    pub optional_at: Option<Vec<Milestone>>,
    /// Hooked dependency-firewall cell text (`N33b-01`).
    pub firewall_cells: Option<BTreeMap<Milestone, String>>,
}

impl Foundation {
    #[must_use]
    pub fn requires_sibling_path(&self) -> bool {
        self.uses.iter().any(|u| matches!(u.posture, Posture::ConsumePath | Posture::BuildTime))
    }

    #[must_use]
    pub fn requires_registry_version(&self) -> bool {
        self.uses.iter().any(|u| u.posture == Posture::Consume)
    }
}

/// Parsed, strictly validated `franken-stack.lock` (`frob.stack-lock/1`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StackLock {
    pub toolchain: Channel,
    pub foundations: Vec<Foundation>,
    pub auditor: Option<AuditorTable>,
    pub freeze: Option<FreezeTable>,
    pub json_library: Option<JsonLibraryTable>,
}

/// Extension hook run after core rules 1–11 pass (`N33b-01`, `N33b-03`, `N34-04`, `N34-25`).
pub trait LockHook {
    fn validate(&self, lock: &StackLock) -> Result<(), LockError>;
}

/// Load and strictly validate `franken-stack.lock` from `path` (reads no record or dir).
pub fn load(path: &Path) -> Result<StackLock, LockError> {
    let bytes = fs::read(path).map_err(|err| {
        LockError::new(0, "io-error", "<root>", "file", format!("failed to read `{}`: {err}", path.display()))
    })?;
    parse_bytes(&bytes)
}

/// Load `franken-stack.lock` from `path` and run all supplied extension hooks.
pub fn load_with_hooks(path: &Path, hooks: &[&dyn LockHook]) -> Result<StackLock, LockError> {
    let lock = load(path)?;
    for hook in hooks { hook.validate(&lock)?; }
    Ok(lock)
}

/// Parse and strictly validate raw `franken-stack.lock` UTF-8 bytes (D0; zero I/O).
pub fn parse_bytes(bytes: &[u8]) -> Result<StackLock, LockError> {
    let root = toml::parse(bytes).map_err(|e| toml_err_to_lock(e, "<root>", "toml"))?;
    let mut fields = Fields::new(root, "<root>");

    let raw_toolchain = fields.opt_str("toolchain").map_err(|e| toml_err_to_lock(e, "<root>", "toolchain"))?;
    let raw_foundations = fields.opt_array("foundation").map_err(|e| toml_err_to_lock(e, "<root>", "foundation"))?;
    let raw_freeze = fields.opt_table("freeze").map_err(|e| toml_err_to_lock(e, "<root>", "freeze"))?;
    let raw_json_library = fields.opt_table("json_library").map_err(|e| toml_err_to_lock(e, "<root>", "json_library"))?;
    let raw_auditor = fields.opt_table("auditor").map_err(|e| toml_err_to_lock(e, "<root>", "auditor"))?;

    // Finish Fields BEFORE domain checks so unknown keys/tables always report rule 1.
    fields.finish().map_err(|e| finish_err_to_rule1(e, "<root>"))?;

    let tc_str = raw_toolchain.ok_or_else(|| LockError::new(
        9, "missing-toolchain", "<root>", "toolchain", "missing required top-level `toolchain` key naming T",
    ))?;
    let toolchain = Channel::parse(&tc_str, "<root>", "toolchain")?;
    let auditor = raw_auditor.map(parse_auditor_table).transpose()?;
    let freeze = raw_freeze.map(parse_freeze_table).transpose()?;
    let json_library = raw_json_library.map(parse_json_library_table).transpose()?;

    let mut foundations = Vec::new();
    let mut seen_ids = BTreeSet::new();
    let mut seen_names = BTreeSet::new();

    if let Some(items) = raw_foundations {
        foundations.reserve(items.len());
        for (idx, sp) in items.into_iter().enumerate() {
            let fallback_row = format!("foundation[{idx}]");
            let Value::Table(tbl) = sp.value else {
                return Err(LockError::new(1, "invalid-row", fallback_row, "foundation",
                    format!("foundation[{idx}] at {}:{}: expected table, found {}", sp.line, sp.column, sp.value.type_name())));
            };
            let row = parse_foundation_row(tbl, &fallback_row)?;
            if !seen_ids.insert(row.id.as_str().to_owned()) {
                return Err(LockError::new(2, "duplicate-id", row.id.as_str(), "id",
                    format!("duplicate foundation id `{}`", row.id)));
            }
            if !seen_names.insert(row.name.clone()) {
                return Err(LockError::new(2, "duplicate-name", row.id.as_str(), "name",
                    format!("duplicate foundation name `{}`", row.name)));
            }
            foundations.push(row);
        }
    }

    foundations.sort_by(|a, b| a.id.cmp(&b.id));
    Ok(StackLock { toolchain, foundations, auditor, freeze, json_library })
}

/// Parse raw `franken-stack.lock` bytes and run all supplied extension hooks.
pub fn parse_bytes_with_hooks(bytes: &[u8], hooks: &[&dyn LockHook]) -> Result<StackLock, LockError> {
    let lock = parse_bytes(bytes)?;
    for hook in hooks { hook.validate(&lock)?; }
    Ok(lock)
}

impl StackLock {
    /// Look up a foundation row by its `FD-…` identifier.
    #[must_use]
    pub fn foundation(&self, id: &str) -> Option<&Foundation> {
        self.foundations.iter().find(|f| f.id.as_str() == id)
    }

    /// Deterministically serialize this lock (fixed key order, rows sorted by `id`,
    /// no comments, LF line ends, strict `frob_core::registry::toml` subset, D0).
    #[must_use]
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut out = String::new();
        push_kv_str(&mut out, "toolchain", self.toolchain.as_str());

        if let Some(a) = &self.auditor {
            out.push_str("\n[auditor]\n");
            push_kv_str(&mut out, "stable", a.stable.as_str());
        }
        if let Some(f) = &self.freeze {
            out.push_str("\n[freeze]\n");
            push_kv_str(&mut out, "campaign", &f.campaign);
            push_kv_str(&mut out, "receipt", f.receipt.as_str());
            push_kv_str(&mut out, "opened", &f.opened);
            push_kv_str(&mut out, "closed", &f.closed);
            push_kv_str(&mut out, "reason", &f.reason);
        }
        if let Some(j) = &self.json_library {
            out.push_str("\n[json_library]\n");
            push_kv_str(&mut out, "package", &j.package);
            push_kv_str(&mut out, "version", &j.version);
            push_kv_str_arr(&mut out, "allowed_in", &j.allowed_in);
            out.push_str("\n[json_library.features]\n");
            for (bin, feats) in &j.features {
                push_toml_key(&mut out, bin);
                out.push_str(" = ");
                push_toml_str_array(&mut out, feats);
                out.push('\n');
            }
        }

        let mut rows: Vec<&Foundation> = self.foundations.iter().collect();
        rows.sort_by(|a, b| a.id.cmp(&b.id));

        for row in rows {
            out.push_str("\n[[foundation]]\n");
            push_kv_str(&mut out, "id", row.id.as_str());
            push_kv_str(&mut out, "name", &row.name);
            if let Some(p) = &row.path { push_kv_str(&mut out, "path", p.as_str()); }
            if let Some(r) = &row.git_rev { push_kv_str(&mut out, "git_rev", r.as_str()); }
            if let Some(v) = &row.version { push_kv_str(&mut out, "version", v.as_str()); }
            if let Some(t) = &row.toolchain { push_kv_str(&mut out, "toolchain", t.as_str()); }
            if !row.features.is_empty() { push_kv_str_arr(&mut out, "features", &row.features); }
            if !row.build_env.is_empty() {
                out.push_str("build_env = { ");
                for (i, (k, v)) in row.build_env.iter().enumerate() {
                    if i > 0 { out.push_str(", "); }
                    push_toml_key(&mut out, k);
                    out.push_str(" = ");
                    push_toml_str(&mut out, v);
                }
                out.push_str(" }\n");
            }
            push_kv_str(&mut out, "admission_gate", row.admission_gate.as_str());
            if let Some(r) = &row.receipt { push_kv_str(&mut out, "receipt", r.as_str()); }
            push_kv_str(&mut out, "fallback", &row.fallback);
            if let Some(m) = row.first_needed { push_kv_str(&mut out, "first_needed", m.as_str()); }
            if row.critical_path { out.push_str("critical_path = true\n"); }
            if let Some(n) = &row.note { push_kv_str(&mut out, "note", n); }
            if let Some(req) = &row.required_at { push_kv_milestone_arr(&mut out, "required_at", req); }
            if let Some(opt) = &row.optional_at { push_kv_milestone_arr(&mut out, "optional_at", opt); }
            if let Some(cells) = &row.firewall_cells {
                out.push_str("firewall_cells = {");
                if !cells.is_empty() {
                    out.push(' ');
                    for (i, (m, txt)) in cells.iter().enumerate() {
                        if i > 0 { out.push_str(", "); }
                        out.push_str(m.as_str());
                        out.push_str(" = ");
                        push_toml_str(&mut out, txt);
                    }
                    out.push(' ');
                }
                out.push_str("}\n");
            }
            for u in &row.uses {
                out.push_str("\n[[foundation.uses]]\n");
                push_kv_str(&mut out, "posture", u.posture.as_str());
                push_kv_str(&mut out, "purpose", &u.purpose);
                push_kv_str_arr(&mut out, "confined_to", &u.confined_to);
                push_kv_str(&mut out, "tier", u.tier.as_str());
                if let Some(m) = u.first_needed { push_kv_str(&mut out, "first_needed", m.as_str()); }
            }
        }
        out.into_bytes()
    }

    /// Domain-tagged BLAKE3 stack-lock digest under `"frob.stack-lock/1"`.
    #[must_use]
    pub fn digest(&self) -> [u8; 32] {
        *fs_blake3::hash_domain(STACK_LOCK_SCHEMA, &self.canonical_bytes()).as_bytes()
    }

    /// Check external-state parts of rules 5 and 10 (`admitted` set and `receipts_dir`).
    ///
    /// A row `toolchain` differing from top-level `T` is never a `LockError` (rule 9).
    #[must_use]
    pub fn check_admitted(&self, admitted: &BTreeSet<GateRef>, receipts_dir: &Path) -> Vec<LockError> {
        let mut errors = Vec::new();
        for row in &self.foundations {
            if row.admission_gate.is_admitted_in(admitted) {
                match &row.git_rev {
                    Some(GitRev::Unadmitted) => errors.push(LockError::new(
                        5, "unadmitted-rev-after-admission", row.id.as_str(), "git_rev",
                        format!("`{}` has `git_rev = \"unadmitted\"` although `{}` is admitted; record the 40-hex pin", row.id, row.admission_gate),
                    )),
                    None if row.requires_sibling_path() => errors.push(LockError::new(
                        5, "missing-git-rev", row.id.as_str(), "git_rev",
                        format!("`{}` requires a 40-hex `git_rev` once `{}` is admitted", row.id, row.admission_gate),
                    )),
                    Some(GitRev::Commit(_)) | None => {}
                }
            }
            if let Some(rec) = &row.receipt {
                let hex = rec.as_str();
                if !receipts_dir.join(format!("{hex}.json")).is_file() && !receipts_dir.join(hex).is_file() {
                    errors.push(LockError::new(
                        10, "missing-receipt-file", row.id.as_str(), "receipt",
                        format!("receipt `{hex}` names no file under `{}` (expected `{hex}.json`)", receipts_dir.display()),
                    ));
                }
            }
        }
        errors
    }
}

/// Deterministic Draft 2020-12 JSON Schema for `frob.stack-lock/1` derived from
/// this module's closed key tables and enumerations (consumed by `N34p4-03`).
#[must_use]
pub fn json_schema() -> Vec<u8> {
    let join_quoted = |xs: &[&str]| {
        let mut s = String::new();
        for (i, x) in xs.iter().enumerate() {
            if i > 0 { s.push_str(", "); }
            s.push('"');
            s.push_str(x);
            s.push('"');
        }
        s
    };
    let postures = join_quoted(&Posture::ALL.map(Posture::as_str));
    let tiers = join_quoted(&Tier::ALL.map(Tier::as_str));
    let milestones = join_quoted(&Milestone::ALL.map(Milestone::as_str));
    let fsim_groups = join_quoted(FSIM_AIRLOCK_FEATURE_GROUPS);

    const TEMPLATE: &str = r##"{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "frob.stack-lock/1",
  "title": "FrankenRobotics Stack Lock (frob.stack-lock/1)",
  "description": "Generated from xtask::stack::lock::json_schema() (N33a-03).",
  "type": "object",
  "additionalProperties": false,
  "required": ["toolchain"],
  "properties": {
    "toolchain": { "$ref": "#/$defs/channel" },
    "foundation": { "type": "array", "items": { "$ref": "#/$defs/foundation" } },
    "freeze": { "$ref": "#/$defs/freeze" },
    "json_library": { "$ref": "#/$defs/json_library" },
    "auditor": { "$ref": "#/$defs/auditor" }
  },
  "$defs": {
    "posture": { "type": "string", "enum": [__POSTURES__] },
    "tier": { "type": "string", "enum": [__TIERS__] },
    "milestone": { "type": "string", "enum": [__MILESTONES__] },
    "channel": { "type": "string", "pattern": "^(nightly-[0-9]{4}-[0-9]{2}-[0-9]{2}|(0|[1-9][0-9]*)\\.(0|[1-9][0-9]*)(\\.(0|[1-9][0-9]*))?)$" },
    "stable_channel": { "type": "string", "pattern": "^(0|[1-9][0-9]*)\\.(0|[1-9][0-9]*)(\\.(0|[1-9][0-9]*))?$" },
    "fd_id": { "type": "string", "pattern": "^FD-[A-Z0-9]+$" },
    "rel_path": { "type": "string", "pattern": "^\\.\\./[^/\\\\]+(?:/[^/\\\\]+)*$" },
    "git_rev": { "type": "string", "pattern": "^([0-9a-f]{40}|unadmitted)$" },
    "exact_version": { "type": "string", "pattern": "^=(0|[1-9][0-9]*)\\.(0|[1-9][0-9]*)\\.(0|[1-9][0-9]*)(-[0-9A-Za-z.-]+)?(\\+[0-9A-Za-z.-]+)?$" },
    "gate_ref": { "type": "string", "pattern": "^(none|INT-[A-Z0-9]+(?:-[A-Z0-9]+)*-[0-9]+(?:\\.\\.[0-9]+)?)$" },
    "blake3_hex": { "type": "string", "pattern": "^[0-9a-f]{64}$" },
    "use_entry": {
      "type": "object",
      "additionalProperties": false,
      "required": ["posture"],
      "properties": {
        "posture": { "$ref": "#/$defs/posture" },
        "purpose": { "type": "string" },
        "confined_to": { "type": "array", "items": { "type": "string", "minLength": 1 } },
        "tier": { "$ref": "#/$defs/tier" },
        "first_needed": { "$ref": "#/$defs/milestone" }
      }
    },
    "foundation": {
      "type": "object",
      "additionalProperties": false,
      "required": ["id", "name", "admission_gate", "fallback"],
      "properties": {
        "id": { "$ref": "#/$defs/fd_id" },
        "name": { "type": "string", "minLength": 1 },
        "posture": { "$ref": "#/$defs/posture" },
        "uses": { "type": "array", "minItems": 1, "items": { "$ref": "#/$defs/use_entry" } },
        "path": { "$ref": "#/$defs/rel_path" },
        "git_rev": { "$ref": "#/$defs/git_rev" },
        "version": { "$ref": "#/$defs/exact_version" },
        "toolchain": { "$ref": "#/$defs/channel" },
        "features": { "type": "array", "items": { "type": "string", "minLength": 1 } },
        "build_env": {
          "type": "object",
          "propertyNames": { "pattern": "^[A-Z][A-Z0-9_]*$" },
          "additionalProperties": { "type": "string", "pattern": "^[\\u0020-\\u007e]*$" }
        },
        "confined_to": { "type": "array", "items": { "type": "string", "minLength": 1 } },
        "admission_gate": { "$ref": "#/$defs/gate_ref" },
        "receipt": { "$ref": "#/$defs/blake3_hex" },
        "fallback": { "type": "string", "minLength": 1 },
        "first_needed": { "$ref": "#/$defs/milestone" },
        "critical_path": { "type": "boolean" },
        "note": { "type": "string" },
        "required_at": { "type": "array", "items": { "$ref": "#/$defs/milestone" } },
        "optional_at": { "type": "array", "items": { "$ref": "#/$defs/milestone" } },
        "firewall_cells": {
          "type": "object",
          "propertyNames": { "$ref": "#/$defs/milestone" },
          "additionalProperties": { "type": "string" }
        }
      },
      "oneOf": [
        { "required": ["posture"], "not": { "required": ["uses"] } },
        { "required": ["uses"], "not": { "anyOf": [{ "required": ["posture"] }, { "required": ["confined_to"] }] } }
      ],
      "allOf": [
        {
          "if": { "properties": { "id": { "const": "FD-FSIM" } }, "required": ["id"] },
          "then": { "properties": { "features": { "type": "array", "items": { "type": "string", "enum": [__FSIM_GROUPS__] } } } }
        }
      ]
    },
    "auditor": {
      "type": "object",
      "additionalProperties": false,
      "required": ["stable"],
      "properties": { "stable": { "$ref": "#/$defs/stable_channel" } }
    },
    "freeze": {
      "type": "object",
      "additionalProperties": false,
      "required": ["campaign", "receipt", "opened", "closed", "reason"],
      "properties": {
        "campaign": { "type": "string", "minLength": 1 },
        "receipt": { "$ref": "#/$defs/blake3_hex" },
        "opened": { "type": "string", "minLength": 1 },
        "closed": { "type": "string" },
        "reason": { "type": "string", "minLength": 1 }
      }
    },
    "json_library": {
      "type": "object",
      "additionalProperties": false,
      "required": ["package", "version", "allowed_in", "features"],
      "properties": {
        "package": { "type": "string", "minLength": 1 },
        "version": { "type": "string", "minLength": 1 },
        "allowed_in": { "type": "array", "items": { "type": "string", "minLength": 1 } },
        "features": {
          "type": "object",
          "additionalProperties": { "type": "array", "items": { "type": "string", "minLength": 1 } }
        }
      }
    }
  }
}
"##;

    TEMPLATE
        .replace("__POSTURES__", &postures)
        .replace("__TIERS__", &tiers)
        .replace("__MILESTONES__", &milestones)
        .replace("__FSIM_GROUPS__", &fsim_groups)
        .into_bytes()
}

// ============================================================================
// Internal parsers & validation helpers
// ============================================================================

fn parse_auditor_table(table: Table) -> Result<AuditorTable, LockError> {
    let mut f = Fields::new(table, "auditor");
    let raw_stable = f.opt_str("stable").map_err(|e| toml_err_to_lock(e, "auditor", "stable"))?;
    f.finish().map_err(|e| finish_err_to_rule1(e, "auditor"))?;
    let s = raw_stable.ok_or_else(|| LockError::new(1, "missing-field", "auditor", "stable", "missing `[auditor].stable`"))?;
    let stable = Channel::parse(&s, "auditor", "stable")?;
    if !stable.is_stable() {
        return Err(LockError::new(9, "invalid-toolchain", "auditor", "stable",
            format!("`[auditor].stable` must be a stable channel, not `{stable}`")));
    }
    Ok(AuditorTable { stable })
}

fn parse_freeze_table(table: Table) -> Result<FreezeTable, LockError> {
    let mut f = Fields::new(table, "freeze");
    let campaign = f.opt_str("campaign").map_err(|e| toml_err_to_lock(e, "freeze", "campaign"))?;
    let receipt = f.opt_str("receipt").map_err(|e| toml_err_to_lock(e, "freeze", "receipt"))?;
    let opened = f.opt_str("opened").map_err(|e| toml_err_to_lock(e, "freeze", "opened"))?;
    let closed = f.opt_str("closed").map_err(|e| toml_err_to_lock(e, "freeze", "closed"))?;
    let reason = f.opt_str("reason").map_err(|e| toml_err_to_lock(e, "freeze", "reason"))?;
    f.finish().map_err(|e| finish_err_to_rule1(e, "freeze"))?;

    let campaign = require_non_empty(campaign, "freeze", "campaign", 1)?;
    let receipt_s = receipt.ok_or_else(|| LockError::new(10, "missing-receipt", "freeze", "receipt", "missing `[freeze].receipt`"))?;
    let receipt = Blake3Hex::parse(&receipt_s, "freeze", "receipt")?;
    let opened = require_non_empty(opened, "freeze", "opened", 1)?;
    let closed = closed.ok_or_else(|| LockError::new(1, "missing-field", "freeze", "closed", "missing `[freeze].closed`"))?;
    let reason = require_non_empty(reason, "freeze", "reason", 1)?;
    Ok(FreezeTable { campaign, receipt, opened, closed, reason })
}

fn parse_json_library_table(table: Table) -> Result<JsonLibraryTable, LockError> {
    let mut f = Fields::new(table, "json_library");
    let package = f.opt_str("package").map_err(|e| toml_err_to_lock(e, "json_library", "package"))?;
    let version = f.opt_str("version").map_err(|e| toml_err_to_lock(e, "json_library", "version"))?;
    let allowed = f.opt_array("allowed_in").map_err(|e| toml_err_to_lock(e, "json_library", "allowed_in"))?;
    let raw_feats = f.opt_table("features").map_err(|e| toml_err_to_lock(e, "json_library", "features"))?;
    f.finish().map_err(|e| finish_err_to_rule1(e, "json_library"))?;

    let package = require_non_empty(package, "json_library", "package", 1)?;
    let version = require_non_empty(version, "json_library", "version", 1)?;
    let allowed_items = allowed.ok_or_else(|| LockError::new(1, "missing-field", "json_library", "allowed_in", "missing `allowed_in`"))?;
    let allowed_in = parse_non_empty_str_vec(allowed_items, "json_library", "allowed_in", 1)?;
    let feats_tbl = raw_feats.ok_or_else(|| LockError::new(1, "missing-field", "json_library", "features", "missing `[json_library.features]`"))?;
    let mut features = BTreeMap::new();
    for (bin, sp) in feats_tbl.into_inner() {
        let Value::Array(arr) = sp.value else {
            return Err(LockError::new(1, "invalid-field-type", "json_library.features", &bin,
                format!("expected array of feature names, found {}", sp.value.type_name())));
        };
        features.insert(bin.clone(), parse_non_empty_str_vec(arr, "json_library.features", &bin, 1)?);
    }
    Ok(JsonLibraryTable { package, version, allowed_in, features })
}

fn parse_foundation_row(table: Table, fallback_row: &str) -> Result<Foundation, LockError> {
    let mut f = Fields::new(table, fallback_row);
    let raw_id = f.opt_str("id").map_err(|e| toml_err_to_lock(e, fallback_row, "id"))?;
    let row = raw_id.as_deref().unwrap_or(fallback_row).to_owned();

    let raw_name = f.opt_str("name").map_err(|e| toml_err_to_lock(e, &row, "name"))?;
    let raw_posture = f.opt_str("posture").map_err(|e| toml_err_to_lock(e, &row, "posture"))?;
    let raw_uses = f.opt_array("uses").map_err(|e| toml_err_to_lock(e, &row, "uses"))?;
    let raw_path = f.opt_str("path").map_err(|e| toml_err_to_lock(e, &row, "path"))?;
    let raw_git_rev = f.opt_str("git_rev").map_err(|e| toml_err_to_lock(e, &row, "git_rev"))?;
    let raw_version = f.opt_str("version").map_err(|e| toml_err_to_lock(e, &row, "version"))?;
    let raw_toolchain = f.opt_str("toolchain").map_err(|e| toml_err_to_lock(e, &row, "toolchain"))?;
    let raw_features = f.opt_array("features").map_err(|e| toml_err_to_lock(e, &row, "features"))?;
    let raw_build_env = f.opt_table("build_env").map_err(|e| toml_err_to_lock(e, &row, "build_env"))?;
    let raw_confined_to = f.opt_array("confined_to").map_err(|e| toml_err_to_lock(e, &row, "confined_to"))?;
    let raw_gate = f.opt_str("admission_gate").map_err(|e| toml_err_to_lock(e, &row, "admission_gate"))?;
    let raw_receipt = f.opt_str("receipt").map_err(|e| toml_err_to_lock(e, &row, "receipt"))?;
    let raw_fallback = f.opt_str("fallback").map_err(|e| toml_err_to_lock(e, &row, "fallback"))?;
    let raw_first_needed = f.opt_str("first_needed").map_err(|e| toml_err_to_lock(e, &row, "first_needed"))?;
    let raw_critical_path = f.opt_bool("critical_path").map_err(|e| toml_err_to_lock(e, &row, "critical_path"))?;
    let raw_note = f.opt_str("note").map_err(|e| toml_err_to_lock(e, &row, "note"))?;
    let raw_required_at = f.opt_array("required_at").map_err(|e| toml_err_to_lock(e, &row, "required_at"))?;
    let raw_optional_at = f.opt_array("optional_at").map_err(|e| toml_err_to_lock(e, &row, "optional_at"))?;
    let raw_fw_cells = f.opt_table("firewall_cells").map_err(|e| toml_err_to_lock(e, &row, "firewall_cells"))?;

    // Rule 1: refuse unknown keys before domain checks so typos like `psture` name themselves.
    f.finish().map_err(|e| finish_err_to_rule1(e, &row))?;

    let id_str = raw_id.ok_or_else(|| LockError::new(2, "missing-id", &row, "id", "missing required `id`"))?;
    let id = FdId::parse(&id_str, &row)?;
    let name = require_non_empty(raw_name, id.as_str(), "name", 2)?;

    let uses = match (raw_posture, raw_uses, raw_confined_to) {
        (Some(_), Some(_), _) => return Err(LockError::new(1, "unknown-field", id.as_str(), "posture",
            "row cannot specify both `posture` and `uses`")),
        (None, Some(_), Some(_)) => return Err(LockError::new(1, "unknown-field", id.as_str(), "confined_to",
            "row cannot specify row-level `confined_to` beside `uses`")),
        (Some(p_str), None, opt_conf) => {
            let posture = Posture::parse(&p_str, id.as_str())
                .map_err(|e| LockError::new(1, "invalid-posture", id.as_str(), "posture", e.message))?;
            let confined_to = match opt_conf {
                Some(items) => parse_non_empty_str_vec(items, id.as_str(), "confined_to", 1)?,
                None => Vec::new(),
            };
            vec![Use { posture, purpose: String::new(), confined_to, tier: Tier::Solid, first_needed: None }]
        }
        (None, Some(items), None) => {
            if items.is_empty() {
                return Err(LockError::new(1, "empty-uses", id.as_str(), "uses", "`uses` must be non-empty"));
            }
            parse_lock_uses_array(items, id.as_str())?
        }
        (None, None, _) => return Err(LockError::new(1, "missing-posture", id.as_str(), "posture",
            "missing required `posture` or `[[foundation.uses]]`")),
    };

    let confined_to = derive_row_confined_to(&uses);
    let path = raw_path.map(|p| RelPath::parse(&p, id.as_str())).transpose()?;
    let git_rev = raw_git_rev.map(|r| GitRev::parse(&r, id.as_str())).transpose()?;
    let version = raw_version.map(|v| ExactVersion::parse(&v, id.as_str())).transpose()?;

    if uses.iter().any(|u| matches!(u.posture, Posture::ConsumePath | Posture::BuildTime)) && path.is_none() {
        return Err(LockError::new(5, "missing-path", id.as_str(), "path",
            "`consume-path` / `build-time` requires a relative sibling `path`"));
    }
    if uses.iter().any(|u| u.posture == Posture::Consume) && version.is_none() {
        return Err(LockError::new(6, "missing-version", id.as_str(), "version",
            "registry `consume` requires an exact `version` pin"));
    }

    let toolchain = raw_toolchain.map(|t| Channel::parse(&t, id.as_str(), "toolchain")).transpose()?;
    let features = match raw_features {
        Some(items) => parse_non_empty_str_vec(items, id.as_str(), "features", 7)?,
        None => Vec::new(),
    };
    if id.as_str() == "FD-FSIM" {
        for feat in &features {
            if !FSIM_AIRLOCK_FEATURE_GROUPS.contains(&feat.as_str()) {
                return Err(LockError::new(7, "invalid-fsim-feature", id.as_str(), "features",
                    format!("unknown FD-FSIM airlock feature group `{feat}` (allowed: {})", FSIM_AIRLOCK_FEATURE_GROUPS.join(", "))));
            }
        }
    }

    let build_env = match raw_build_env {
        Some(tbl) => parse_build_env(tbl, id.as_str())?,
        None => BTreeMap::new(),
    };
    let gate_s = raw_gate.ok_or_else(|| LockError::new(1, "missing-field", id.as_str(), "admission_gate", "missing `admission_gate`"))?;
    let admission_gate = GateRef::parse(&gate_s, id.as_str())?;
    let receipt = raw_receipt.map(|r| Blake3Hex::parse(&r, id.as_str(), "receipt")).transpose()?;

    let fallback = match raw_fallback {
        Some(fb) if !fb.trim().is_empty() => fb,
        _ => return Err(LockError::new(11, "empty-fallback", id.as_str(), "fallback",
            "every foundation row requires a non-empty `fallback`")),
    };

    let first_needed = raw_first_needed
        .map(|m| Milestone::parse(&m, id.as_str()).map_err(|e| LockError::new(1, "invalid-milestone", id.as_str(), "first_needed", e.message)))
        .transpose()?;
    let required_at = raw_required_at.map(|items| parse_milestone_vec(items, id.as_str(), "required_at")).transpose()?;
    let optional_at = raw_optional_at.map(|items| parse_milestone_vec(items, id.as_str(), "optional_at")).transpose()?;
    if let (Some(req), Some(opt)) = (&required_at, &optional_at) {
        for m in req {
            if opt.contains(m) {
                return Err(LockError::new(1, "conflicting-firewall-milestone", id.as_str(), "optional_at",
                    format!("milestone `{m}` cannot be in both `required_at` and `optional_at`")));
            }
        }
    }
    let firewall_cells = raw_fw_cells.map(|tbl| parse_firewall_cells(tbl, id.as_str())).transpose()?;

    Ok(Foundation {
        id, name, uses, path, git_rev, version, toolchain, features, build_env,
        confined_to, admission_gate, receipt, fallback, first_needed,
        critical_path: raw_critical_path.unwrap_or(false),
        note: raw_note, required_at, optional_at, firewall_cells,
    })
}

fn parse_lock_uses_array(items: Vec<Spanned<Value>>, row: &str) -> Result<Vec<Use>, LockError> {
    let mut out = Vec::with_capacity(items.len());
    for (idx, sp) in items.into_iter().enumerate() {
        let entry = format!("{row}.uses[{idx}]");
        let Value::Table(tbl) = sp.value else {
            return Err(LockError::new(1, "invalid-use-entry", &entry, "uses",
                format!("expected table, found {}", sp.value.type_name())));
        };
        let mut f = Fields::new(tbl, &entry);
        let raw_posture = f.opt_str("posture").map_err(|e| toml_err_to_lock(e, &entry, "posture"))?;
        let raw_purpose = f.opt_str("purpose").map_err(|e| toml_err_to_lock(e, &entry, "purpose"))?;
        let raw_conf = f.opt_array("confined_to").map_err(|e| toml_err_to_lock(e, &entry, "confined_to"))?;
        let raw_tier = f.opt_str("tier").map_err(|e| toml_err_to_lock(e, &entry, "tier"))?;
        let raw_fn = f.opt_str("first_needed").map_err(|e| toml_err_to_lock(e, &entry, "first_needed"))?;
        f.finish().map_err(|e| finish_err_to_rule1(e, &entry))?;

        let p_s = raw_posture.ok_or_else(|| LockError::new(1, "missing-posture", &entry, "posture", "missing `posture`"))?;
        let posture = Posture::parse(&p_s, &entry).map_err(|e| LockError::new(1, "invalid-posture", &entry, "posture", e.message))?;
        let tier = match raw_tier.as_deref() {
            Some(t) => Tier::parse(t, &entry).map_err(|e| LockError::new(1, "invalid-tier", &entry, "tier", e.message))?,
            None => Tier::Solid,
        };
        let first_needed = raw_fn
            .as_deref()
            .map(|m| Milestone::parse(m, &entry).map_err(|e| LockError::new(1, "invalid-milestone", &entry, "first_needed", e.message)))
            .transpose()?;
        let confined_to = match raw_conf {
            Some(c) => parse_non_empty_str_vec(c, &entry, "confined_to", 1)?,
            None => Vec::new(),
        };
        out.push(Use { posture, purpose: raw_purpose.unwrap_or_default(), confined_to, tier, first_needed });
    }
    Ok(out)
}

fn derive_row_confined_to(uses: &[Use]) -> Vec<String> {
    if let [single] = uses {
        return single.confined_to.clone();
    }
    let mut out = Vec::new();
    for u in uses {
        for name in &u.confined_to {
            if !out.contains(name) { out.push(name.clone()); }
        }
    }
    out
}

fn parse_build_env(table: Table, row: &str) -> Result<BTreeMap<String, String>, LockError> {
    let mut out = BTreeMap::new();
    for (k, sp) in table.into_inner() {
        if !is_upper_env_ident(&k) {
            return Err(LockError::new(8, "invalid-build-env", row, "build_env",
                format!("build_env key `{k}` must match `[A-Z][A-Z0-9_]*`")));
        }
        let Value::Str(val) = sp.value else {
            return Err(LockError::new(8, "invalid-build-env", row, "build_env",
                format!("build_env `{k}` must be a string, found {}", sp.value.type_name())));
        };
        if !val.bytes().all(|b| (0x20..=0x7E).contains(&b)) {
            return Err(LockError::new(8, "invalid-build-env", row, "build_env",
                format!("build_env `{k}` value must be printable ASCII (0x20..=0x7E)")));
        }
        out.insert(k, val);
    }
    Ok(out)
}

fn parse_milestone_vec(items: Vec<Spanned<Value>>, row: &str, key: &str) -> Result<Vec<Milestone>, LockError> {
    items.into_iter().enumerate().map(|(i, sp)| {
        let Some(s) = sp.value.as_str() else {
            return Err(LockError::new(1, "invalid-milestone", row, key,
                format!("{key}[{i}]: expected milestone string, found {}", sp.value.type_name())));
        };
        Milestone::parse(s, row).map_err(|e| LockError::new(1, "invalid-milestone", row, key, e.message))
    }).collect()
}

fn parse_firewall_cells(table: Table, row: &str) -> Result<BTreeMap<Milestone, String>, LockError> {
    let mut out = BTreeMap::new();
    for (k, sp) in table.into_inner() {
        let m = Milestone::parse(&k, row)
            .map_err(|e| LockError::new(1, "invalid-milestone", row, "firewall_cells", e.message))?;
        let Value::Str(s) = sp.value else {
            return Err(LockError::new(1, "invalid-field-type", row, "firewall_cells",
                format!("firewall_cells.{k}: expected string, found {}", sp.value.type_name())));
        };
        out.insert(m, s);
    }
    Ok(out)
}

fn parse_non_empty_str_vec(items: Vec<Spanned<Value>>, row: &str, key: &str, rule: u8) -> Result<Vec<String>, LockError> {
    items.into_iter().enumerate().map(|(i, sp)| {
        let Some(s) = sp.value.as_str() else {
            return Err(LockError::new(rule, "invalid-field-type", row, key,
                format!("{key}[{i}]: expected string, found {}", sp.value.type_name())));
        };
        if s.trim().is_empty() {
            return Err(LockError::new(rule, "empty-string-entry", row, key, format!("{key}[{i}] must be non-empty")));
        }
        Ok(s.to_owned())
    }).collect()
}

fn require_non_empty(val: Option<String>, row: &str, key: &str, rule: u8) -> Result<String, LockError> {
    match val {
        Some(s) if !s.trim().is_empty() => Ok(s),
        Some(_) => Err(LockError::new(rule, "empty-field", row, key, format!("`{key}` must be non-empty"))),
        None => Err(LockError::new(rule, "missing-field", row, key, format!("missing required `{key}`"))),
    }
}

fn finish_err_to_rule1(err: TomlError, row: &str) -> LockError {
    match &err.kind {
        TomlErrorKind::UnknownKey { key, .. } => LockError::new(1, "unknown-field", row, key.clone(), err.to_string()),
        _ => LockError::new(1, "unknown-field", row, "unknown", err.to_string()),
    }
}

fn toml_err_to_lock(err: TomlError, row: &str, default_key: &str) -> LockError {
    match &err.kind {
        TomlErrorKind::UnknownKey { key, .. } => LockError::new(1, "unknown-field", row, key.clone(), err.to_string()),
        TomlErrorKind::TypeMismatch { key, .. } => {
            let rule = match key.as_str() {
                "git_rev" => 3, "path" => 4, "version" => 6, "features" => 7,
                "build_env" => 8, "toolchain" => 9, "receipt" => 10, "fallback" => 11,
                _ => 1,
            };
            LockError::new(rule, "invalid-field-type", row, key.clone(), err.to_string())
        }
        _ => LockError::new(0, "toml-syntax", row, default_key, err.to_string()),
    }
}

fn is_lower_hex(s: &str, len: usize) -> bool {
    s.len() == len && s.bytes().all(|b| b.is_ascii_digit() || matches!(b, b'a'..=b'f'))
}

fn is_upper_env_ident(s: &str) -> bool {
    s.as_bytes().split_first().is_some_and(|(&f, r)| {
        f.is_ascii_uppercase() && r.iter().all(|b| b.is_ascii_uppercase() || b.is_ascii_digit() || *b == b'_')
    })
}

fn is_valid_nightly_channel(s: &str) -> bool {
    let Some(date) = s.strip_prefix("nightly-") else { return false; };
    let mut parts = date.split('-');
    let (Some(ys), Some(ms), Some(ds), None) = (parts.next(), parts.next(), parts.next(), parts.next()) else {
        return false;
    };
    if ys.len() != 4 || ms.len() != 2 || ds.len() != 2 { return false; }
    let (Ok(y), Ok(m), Ok(d)) = (ys.parse::<u32>(), ms.parse::<u32>(), ds.parse::<u32>()) else { return false; };
    if y < 1970 || !(1..=12).contains(&m) { return false; }
    let max_d = match m {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => if (y % 4 == 0 && y % 100 != 0) || (y % 400 == 0) { 29 } else { 28 },
        _ => return false,
    };
    (1..=max_d).contains(&d)
}

fn is_valid_stable_channel(s: &str) -> bool {
    let mut p = s.split('.');
    let (Some(a), Some(b), c, None) = (p.next(), p.next(), p.next(), p.next()) else { return false; };
    let dec = |x: &str| !x.is_empty() && x.bytes().all(|ch| ch.is_ascii_digit()) && (x.len() == 1 || !x.starts_with('0'));
    dec(a) && dec(b) && c.is_none_or(dec)
}

fn semver_suffix_valid(raw: &str) -> bool {
    let rest = raw.strip_prefix('=').unwrap_or(raw);
    let Some(pos) = rest.find(['-', '+']) else { return true; };
    let suf = &rest[pos + 1..];
    !suf.is_empty() && suf.bytes().all(|b| b.is_ascii_alphanumeric() || matches!(b, b'.' | b'-' | b'+'))
}

fn is_valid_int_gate_ref(raw: &str) -> bool {
    let Some(after) = raw.strip_prefix("INT-") else { return false; };
    let (base, range_end) = match after.split_once("..") { Some((l, r)) => (l, Some(r)), None => (after, None) };
    if base.contains("..") { return false; }
    let Some((grp, start_s)) = base.rsplit_once('-') else { return false; };
    let grp_ok = !grp.is_empty() && grp.split('-')
        .all(|seg| !seg.is_empty() && seg.bytes().all(|b| b.is_ascii_uppercase() || b.is_ascii_digit()));
    if !grp_ok || start_s.is_empty() || !start_s.bytes().all(|b| b.is_ascii_digit()) { return false; }
    match range_end {
        None => true,
        Some(end_s) => !end_s.is_empty() && end_s.bytes().all(|b| b.is_ascii_digit())
            && matches!((start_s.parse::<u32>(), end_s.parse::<u32>()), (Ok(a), Ok(b)) if b >= a),
    }
}

fn push_toml_key(out: &mut String, key: &str) {
    if !key.is_empty() && key.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-') {
        out.push_str(key);
    } else {
        push_toml_str(out, key);
    }
}

fn push_toml_str(out: &mut String, s: &str) {
    out.push('"');
    for ch in s.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 || (c as u32) == 0x7F => out.push_str(&format!("\\u{:04X}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
}

fn push_kv_str(out: &mut String, key: &str, val: &str) {
    out.push_str(key);
    out.push_str(" = ");
    push_toml_str(out, val);
    out.push('\n');
}

fn push_toml_str_array(out: &mut String, items: &[String]) {
    out.push('[');
    for (i, it) in items.iter().enumerate() {
        if i > 0 { out.push_str(", "); }
        push_toml_str(out, it);
    }
    out.push(']');
}

fn push_kv_str_arr(out: &mut String, key: &str, items: &[String]) {
    out.push_str(key);
    out.push_str(" = ");
    push_toml_str_array(out, items);
    out.push('\n');
}

fn push_kv_milestone_arr(out: &mut String, key: &str, items: &[Milestone]) {
    out.push_str(key);
    out.push_str(" = [");
    for (i, m) in items.iter().enumerate() {
        if i > 0 { out.push_str(", "); }
        push_toml_str(out, m.as_str());
    }
    out.push_str("]\n");
}

// ============================================================================
// Unit tests (`cargo test -p xtask stack::lock`)
// ============================================================================

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU64, Ordering};

    use super::*;

    const SAMPLE_REV: &str = "d02ad9c170d5932667d15359cd362d3198ac4806";
    const SAMPLE_RECEIPT: &str = "d3b0885b1d5bfbe09039a84ae190c40fce4ad3b0b81b5c28bf946201987462d4";

    fn template_toml() -> String {
        format!(
            "# franken-stack.lock template fixture (N33a-03)\ntoolchain = \"nightly-2026-08-31\"\n\n\
[[foundation]]\n\
id = \"FD-FSIM\"\n\
name = \"frankensim\"\n\
posture = \"consume-path\"\n\
path = \"../frankensim\"\n\
git_rev = \"{SAMPLE_REV}\"\n\
toolchain = \"nightly-2026-08-31\"\n\
features = [\"referee\", \"geometry\", \"ledger\"] # default airlock groups\n\
build_env = {{ FRANKENSIM_DEPGRAPH_SALT = \"franken-robotics-dev-v1\" }}\n\
confined_to = [\"frob-fsim\", \"bedrock-direct\"]\n\
admission_gate = \"INT-FSIM-01\"\n\
receipt = \"{SAMPLE_RECEIPT}\"\n\
fallback = \"KILL-FSIM: (a) pinned worker process; (b) lift the minimal referee set.\"\n\n\
[[foundation]]\n\
id = \"FD-ASUP\"\n\
name = \"asupersync\"\n\
posture = \"consume\"\n\
version = \"=0.7.0\"\n\
admission_gate = \"INT-ASUP-01\"\n\
fallback = \"FD-ASUP-01 aligns the siblings; KILL-MCP fallback 1.\"\n"
        )
    }

    fn fresh_scratch_dir(tag: &str) -> PathBuf {
        static CTR: AtomicU64 = AtomicU64::new(0);
        let dir = std::env::temp_dir().join(format!(
            "frob_xtask_lock_{tag}_{}_{}",
            std::process::id(),
            CTR.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(&dir).expect("create scratch dir");
        dir
    }

    #[test]
    fn lock_parses_template() {
        let lock = parse_bytes(template_toml().as_bytes()).expect("template parses");
        assert_eq!(lock.toolchain.as_str(), "nightly-2026-08-31");
        assert_eq!(lock.foundations.len(), 2);

        let fsim = lock.foundation("FD-FSIM").expect("FD-FSIM exists");
        assert_eq!(fsim.features, vec!["referee", "geometry", "ledger"]);
        assert_eq!(fsim.build_env.len(), 1);
        assert_eq!(fsim.build_env.get("FRANKENSIM_DEPGRAPH_SALT").map(String::as_str), Some("franken-robotics-dev-v1"));
        assert_eq!(fsim.first_needed, None);
        assert!(!fsim.critical_path);

        let asup = lock.foundation("FD-ASUP").expect("FD-ASUP exists");
        assert_eq!(asup.version.as_ref().map(ExactVersion::as_str), Some("=0.7.0"));
        assert_eq!(asup.first_needed, None);
        assert!(!asup.critical_path);

        let repo_lock = load(&PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../franken-stack.lock"))
            .expect("committed repo franken-stack.lock parses");
        assert!(repo_lock.foundation("FD-FSIM").is_some());
    }

    #[test]
    fn lock_rejects_unknown_field() {
        let err = parse_bytes(template_toml().replace("posture = \"consume\"", "psture = \"consume\"").as_bytes())
            .expect_err("psture must fail");
        assert_eq!((err.rule, err.kind, err.key.as_str(), err.row.as_str()), (1, "unknown-field", "psture", "FD-ASUP"));
    }

    #[test]
    fn lock_rejects_unknown_table() {
        let err = parse_bytes(format!("{}\n[audit]\nstable = \"1.85.0\"\n", template_toml()).as_bytes())
            .expect_err("[audit] must fail");
        assert_eq!((err.rule, err.kind, err.key.as_str()), (1, "unknown-field", "audit"));

        let err2 = parse_bytes(format!("{}\n[auditor]\nstable = \"1.85.0\"\nextra = \"x\"\n", template_toml()).as_bytes())
            .expect_err("[auditor] extra key must fail");
        assert_eq!((err2.rule, err2.row.as_str(), err2.key.as_str()), (1, "auditor", "extra"));

        let err3 = parse_bytes(format!("{}\n[auditor]\nchannel = \"1.85.0\"\n", template_toml()).as_bytes())
            .expect_err("[auditor] wrong key must fail");
        assert_eq!((err3.rule, err3.key.as_str()), (1, "channel"));
    }

    #[test]
    fn lock_rejects_short_rev() {
        let err = parse_bytes(template_toml().replace(SAMPLE_REV, "483d09c682f2").as_bytes())
            .expect_err("12-hex rev must fail");
        assert_eq!((err.rule, err.key.as_str(), err.row.as_str()), (3, "git_rev", "FD-FSIM"));
        assert_eq!(parse_bytes(template_toml().replace(SAMPLE_REV, &SAMPLE_REV.to_ascii_uppercase()).as_bytes()).unwrap_err().rule, 3);
    }

    #[test]
    fn lock_rejects_absolute_path() {
        for bad in ["/Users/x/frankensim", "~/frankensim", "../a/../frankensim", "frankensim"] {
            let err = parse_bytes(template_toml().replace("../frankensim", bad).as_bytes())
                .expect_err("bad path must fail");
            assert_eq!((err.rule, err.key.as_str()), (4, "path"));
        }
    }

    #[test]
    fn lock_rejects_inexact_version() {
        for bad in ["^0.6", "0.7.0", "~0.7.0"] {
            let err = parse_bytes(template_toml().replace("=0.7.0", bad).as_bytes())
                .expect_err("inexact version must fail");
            assert_eq!((err.rule, err.key.as_str()), (6, "version"));
        }
    }

    #[test]
    fn lock_rejects_unknown_group() {
        for bad in ["fluid", "harness"] {
            let err = parse_bytes(template_toml().replace("\"ledger\"", &format!("\"{bad}\"")).as_bytes())
                .expect_err("unknown feature group must fail");
            assert_eq!((err.rule, err.key.as_str(), err.row.as_str()), (7, "features", "FD-FSIM"));
        }
    }

    #[test]
    fn lock_rejects_duplicate_id() {
        let dup_id = format!("{}\n[[foundation]]\nid = \"FD-ASUP\"\nname = \"other\"\nposture = \"pattern\"\nadmission_gate = \"none\"\nfallback = \"none\"\n", template_toml());
        let e1 = parse_bytes(dup_id.as_bytes()).expect_err("dup id");
        assert_eq!((e1.rule, e1.key.as_str()), (2, "id"));

        let dup_name = format!("{}\n[[foundation]]\nid = \"FD-OTHER\"\nname = \"asupersync\"\nposture = \"pattern\"\nadmission_gate = \"none\"\nfallback = \"none\"\n", template_toml());
        let e2 = parse_bytes(dup_name.as_bytes()).expect_err("dup name");
        assert_eq!((e2.rule, e2.key.as_str()), (2, "name"));
    }

    #[test]
    fn lock_requires_path_and_version() {
        let e1 = parse_bytes(template_toml().replace("path = \"../frankensim\"\n", "").as_bytes())
            .expect_err("consume-path without path");
        assert_eq!((e1.rule, e1.key.as_str()), (5, "path"));

        let e2 = parse_bytes(template_toml().replace("version = \"=0.7.0\"\n", "").as_bytes())
            .expect_err("consume without version");
        assert_eq!((e2.rule, e2.key.as_str()), (6, "version"));
    }

    #[test]
    fn lock_rejects_bad_build_env() {
        let e1 = parse_bytes(template_toml().replace("FRANKENSIM_DEPGRAPH_SALT", "frankensim_salt").as_bytes())
            .expect_err("lower-case build_env key");
        assert_eq!((e1.rule, e1.key.as_str()), (8, "build_env"));

        let e2 = parse_bytes(template_toml().replace("franken-robotics-dev-v1", "bad\\tvalue").as_bytes())
            .expect_err("tab in build_env value");
        assert_eq!((e2.rule, e2.key.as_str()), (8, "build_env"));
    }

    #[test]
    fn lock_rejects_empty_fallback() {
        let e1 = parse_bytes(template_toml().replace("FD-ASUP-01 aligns the siblings; KILL-MCP fallback 1.", "").as_bytes())
            .expect_err("empty fallback");
        assert_eq!((e1.rule, e1.key.as_str()), (11, "fallback"));

        let e2 = parse_bytes(template_toml().replace("fallback = \"FD-ASUP-01 aligns the siblings; KILL-MCP fallback 1.\"\n", "").as_bytes())
            .expect_err("missing fallback");
        assert_eq!((e2.rule, e2.key.as_str()), (11, "fallback"));
    }

    #[test]
    fn lock_uses_two_forms() {
        let tmpl = parse_bytes(template_toml().as_bytes()).expect("template");
        let fsim = tmpl.foundation("FD-FSIM").expect("FD-FSIM");
        assert_eq!(fsim.uses.len(), 1);
        assert_eq!((fsim.uses[0].posture, fsim.uses[0].purpose.as_str(), fsim.uses[0].tier), (Posture::ConsumePath, "", Tier::Solid));
        assert_eq!((fsim.first_needed, fsim.critical_path), (None, false));

        let sub = "toolchain = \"nightly-2026-08-31\"\n\n[[foundation]]\nid = \"FD-FSQL\"\nname = \"frankensqlite\"\nversion = \"=0.5.1\"\nadmission_gate = \"INT-FSQL-01\"\nfallback = \"JSONL ledger\"\n\n\
[[foundation.uses]]\nposture = \"consume\"\npurpose = \"ledger\"\nconfined_to = [\"frob-chronicle\"]\ntier = \"solid\"\n\n\
[[foundation.uses]]\nposture = \"pattern\"\npurpose = \"pager\"\nconfined_to = []\ntier = \"solid\"\n\n\
[[foundation.uses]]\nposture = \"oracle\"\npurpose = \"pager oracle\"\nconfined_to = [\"frob-conformance\"]\ntier = \"frontier\"\nfirst_needed = \"M2\"\n";

        let inl = "toolchain = \"nightly-2026-08-31\"\n\n[[foundation]]\nid = \"FD-FSQL\"\nname = \"frankensqlite\"\nversion = \"=0.5.1\"\nadmission_gate = \"INT-FSQL-01\"\nfallback = \"JSONL ledger\"\n\
uses = [\n\
  { posture = \"consume\", purpose = \"ledger\", confined_to = [\"frob-chronicle\"], tier = \"solid\" },\n\
  { posture = \"pattern\", purpose = \"pager\", confined_to = [], tier = \"solid\" },\n\
  { posture = \"oracle\", purpose = \"pager oracle\", confined_to = [\"frob-conformance\"], tier = \"frontier\", first_needed = \"M2\" },\n\
]\n";

        let a = parse_bytes(sub.as_bytes()).expect("sub-tables");
        let b = parse_bytes(inl.as_bytes()).expect("inline uses");
        assert_eq!(a, b);
        assert_eq!(a.digest(), b.digest());
        let fsql = a.foundation("FD-FSQL").expect("FD-FSQL");
        assert_eq!(fsql.uses.len(), 3);
        assert_eq!(fsql.uses[2].first_needed, Some(Milestone::M2));

        let mix_p = parse_bytes(sub.replace("version = \"=0.5.1\"", "version = \"=0.5.1\"\nposture = \"consume\"").as_bytes())
            .expect_err("posture + uses must fail");
        assert_eq!((mix_p.rule, mix_p.key.as_str()), (1, "posture"));

        let mix_c = parse_bytes(sub.replace("version = \"=0.5.1\"", "version = \"=0.5.1\"\nconfined_to = [\"frob-chronicle\"]").as_bytes())
            .expect_err("confined_to + uses must fail");
        assert_eq!((mix_c.rule, mix_c.key.as_str()), (1, "confined_to"));
    }

    #[test]
    fn lock_digest_ignores_order_and_comments() {
        let a = parse_bytes(template_toml().as_bytes()).expect("a");
        let b_s = format!(
            "# extra header\ntoolchain = \"nightly-2026-08-31\" # inline\n\n\
[[foundation]]\nfallback = \"FD-ASUP-01 aligns the siblings; KILL-MCP fallback 1.\"\nadmission_gate = \"INT-ASUP-01\"\nversion = \"=0.7.0\"\nposture = \"consume\"\nname = \"asupersync\"\nid = \"FD-ASUP\"\n\n\
[[foundation]]\n# reversed row\nreceipt = \"{SAMPLE_RECEIPT}\"\nadmission_gate = \"INT-FSIM-01\"\nconfined_to = [\"frob-fsim\", \"bedrock-direct\"]\nbuild_env = {{ FRANKENSIM_DEPGRAPH_SALT = \"franken-robotics-dev-v1\" }}\nfeatures = [\"referee\", \"geometry\", \"ledger\"]\ntoolchain = \"nightly-2026-08-31\"\ngit_rev = \"{SAMPLE_REV}\"\npath = \"../frankensim\"\nposture = \"consume-path\"\nname = \"frankensim\"\nid = \"FD-FSIM\"\nfallback = \"KILL-FSIM: (a) pinned worker process; (b) lift the minimal referee set.\"\n"
        );
        let b = parse_bytes(b_s.as_bytes()).expect("b");
        assert_eq!(a.canonical_bytes(), b.canonical_bytes());
        assert_eq!(a.digest(), b.digest());
    }

    #[test]
    fn lock_digest_sees_pin_change() {
        let base = parse_bytes(template_toml().as_bytes()).expect("base").digest();
        let flipped = format!("{}7", &SAMPLE_REV[..39]);
        assert_ne!(base, parse_bytes(template_toml().replace(SAMPLE_REV, &flipped).as_bytes()).expect("rev").digest());
        assert_ne!(base, parse_bytes(template_toml().replace("dev-v1", "dev-v2").as_bytes()).expect("env").digest());
        assert_ne!(base, parse_bytes(template_toml().replacen("nightly-2026-08-31", "nightly-2026-09-01", 1).as_bytes()).expect("tc").digest());
    }

    #[test]
    fn lock_roundtrip_canonical() {
        let rich = format!(
            "{}\n[auditor]\nstable = \"1.85.0\"\n\n\
[freeze]\ncampaign = \"M7\"\nreceipt = \"{SAMPLE_RECEIPT}\"\nopened = \"{SAMPLE_REV}\"\nclosed = \"\"\nreason = \"ten-fold\"\n\n\
[json_library]\npackage = \"serde_json\"\nversion = \"1.0.154\"\nallowed_in = [\"frob-io\"]\n\n\
[json_library.features]\nfrob = [\"std\"]\nfrob-verify = []\n\n\
[[foundation]]\nid = \"FD-FSQL\"\nname = \"frankensqlite\"\nversion = \"=0.5.1\"\nadmission_gate = \"INT-FSQL-01\"\nfallback = \"JSONL\"\nfirst_needed = \"M0\"\ncritical_path = true\nnote = \"ledger\"\nrequired_at = [\"M1\"]\noptional_at = [\"M4\"]\nfirewall_cells = {{ M0 = \"admit\", M1 = \"ledger\" }}\n\n\
[[foundation.uses]]\nposture = \"consume\"\npurpose = \"ledger\"\nconfined_to = [\"frob-chronicle\"]\ntier = \"solid\"\n",
            template_toml()
        );
        let parsed = parse_bytes(rich.as_bytes()).expect("rich parses");
        let canon = parsed.canonical_bytes();
        let reparsed = parse_bytes(&canon).expect("canon reparses");
        assert_eq!(reparsed, parsed);
        assert_eq!(reparsed.canonical_bytes(), canon);
    }

    #[test]
    fn lock_json_schema_agrees_with_parser() {
        let s1 = json_schema();
        assert_eq!(s1, json_schema(), "json_schema() must be deterministic");
        let schema = mini_json::parse(std::str::from_utf8(&s1).expect("utf8")).expect("valid JSON");

        fn keys_of(m: &BTreeMap<String, mini_json::Val>) -> BTreeSet<&str> {
            m.keys().map(String::as_str).collect()
        }
        assert_eq!(keys_of(schema.get_obj("properties").expect("top props")), TOP_LEVEL_KEYS.iter().copied().collect());
        let defs = schema.get_obj("$defs").expect("$defs");
        for (def, expected) in [
            ("foundation", FOUNDATION_KEYS), ("use_entry", USE_KEYS),
            ("auditor", AUDITOR_KEYS), ("freeze", FREEZE_KEYS), ("json_library", JSON_LIBRARY_KEYS),
        ] {
            let props = defs.get(def).and_then(|v| v.get_obj("properties")).expect("def props");
            assert_eq!(keys_of(props), expected.iter().copied().collect(), "mismatch in {def}");
        }

        let tmpl_json = toml_table_to_json(&toml::parse(template_toml().as_bytes()).expect("toml"));
        assert!(mini_json::validate(&tmpl_json, &schema, &schema).is_ok());

        let bad_toml = template_toml().replace("posture = \"consume\"", "psture = \"consume\"");
        let bad_json = toml_table_to_json(&toml::parse(bad_toml.as_bytes()).expect("bad toml"));
        let err = mini_json::validate(&bad_json, &schema, &schema).expect_err("psture must fail schema");
        assert!(err.contains("psture"), "schema error must mention psture: {err}");
    }

    #[test]
    fn lock_unadmitted_rev_depends_on_admitted_set() {
        let lock = parse_bytes(template_toml().replace(SAMPLE_REV, "unadmitted").as_bytes()).expect("unadmitted loads");
        let dir = fresh_scratch_dir("unadmitted");
        fs::write(dir.join(format!("{SAMPLE_RECEIPT}.json")), "{}\n").expect("write receipt");

        assert!(lock.check_admitted(&BTreeSet::new(), &dir).is_empty());
        let mut admitted = BTreeSet::new();
        admitted.insert(GateRef::parse("INT-FSIM-01", "FD-FSIM").expect("gate"));
        let errs = lock.check_admitted(&admitted, &dir);
        assert_eq!(errs.len(), 1);
        assert_eq!((errs[0].rule, errs[0].row.as_str(), errs[0].key.as_str()), (5, "FD-FSIM", "git_rev"));

        let err = parse_bytes(template_toml().replace(SAMPLE_REV, "pending").as_bytes()).expect_err("pending fails");
        assert_eq!((err.rule, err.key.as_str()), (3, "git_rev"));
    }

    #[test]
    fn lock_row_toolchain_may_differ_from_t() {
        let off = template_toml().replace("toolchain = \"nightly-2026-08-31\"\nfeatures =", "toolchain = \"nightly-2026-07-06\"\nfeatures =");
        let lock = parse_bytes(off.as_bytes()).expect("off-T row loads");
        let dir = fresh_scratch_dir("off_t");
        fs::write(dir.join(format!("{SAMPLE_RECEIPT}.json")), "{}\n").expect("write receipt");

        assert!(lock.check_admitted(&BTreeSet::new(), &dir).is_empty());
        let mut admitted = BTreeSet::new();
        admitted.insert(GateRef::parse("INT-TOOLCHAIN-01", "<root>").expect("gate"));
        assert!(lock.check_admitted(&admitted, &dir).is_empty());

        let bad = template_toml().replace("toolchain = \"nightly-2026-08-31\"\nfeatures =", "toolchain = \"nightly-2026-8-31\"\nfeatures =");
        let err = parse_bytes(bad.as_bytes()).expect_err("nightly-2026-8-31 fails rule 9");
        assert_eq!((err.rule, err.key.as_str()), (9, "toolchain"));
    }

    #[test]
    fn lock_receipt_must_name_a_file() {
        let lock = parse_bytes(template_toml().as_bytes()).expect("template loads");
        let dir = fresh_scratch_dir("receipt");
        let errs = lock.check_admitted(&BTreeSet::new(), &dir);
        assert_eq!(errs.len(), 1);
        assert_eq!((errs[0].rule, errs[0].row.as_str(), errs[0].key.as_str()), (10, "FD-FSIM", "receipt"));

        fs::write(dir.join(format!("{SAMPLE_RECEIPT}.json")), "{}\n").expect("create receipt");
        assert!(lock.check_admitted(&BTreeSet::new(), &dir).is_empty());
        assert_eq!(parse_bytes(template_toml().replace(SAMPLE_RECEIPT, "deadbeef").as_bytes()).unwrap_err().rule, 10);
    }

    #[test]
    fn lock_canonical_is_idempotent() {
        let mut rng = XorShift64(0x5A17_F0B0_2026_1124);
        let channels = ["nightly-2026-07-06", "nightly-2026-08-31", "1.85.0", "1.86"];

        for case in 0..1000u32 {
            let mut doc = format!("# case {case}\ntoolchain = \"{}\"\n", channels[rng.range(channels.len())]);
            if rng.coin() { doc.push_str("\n[auditor]\nstable = \"1.85.0\"\n"); }
            if rng.coin() {
                doc.push_str(&format!(
                    "\n[freeze]\ncampaign = \"C-{case}\"\nreceipt = \"{}\"\nopened = \"{}\"\nclosed = \"\"\nreason = \"m7\"\n",
                    rng.hex(64), rng.hex(40)
                ));
            }
            if rng.coin() {
                doc.push_str("\n[json_library]\npackage = \"serde_json\"\nversion = \"1.0.154\"\nallowed_in = [\"frob-io\"]\n\n[json_library.features]\nfrob = [\"std\"]\nfrob-verify = []\n");
            }
            let n = 1 + rng.range(5);
            let mut ids: Vec<String> = (0..n)
                .map(|i| if i == 0 && rng.coin() { "FD-FSIM".to_owned() } else { format!("FD-R{case}N{i}") })
                .collect();
            for i in (1..ids.len()).rev() { let j = rng.range(i + 1); ids.swap(i, j); }

            for (idx, id) in ids.iter().enumerate() {
                let is_fsim = id == "FD-FSIM";
                let name = if is_fsim { "frankensim".to_owned() } else { format!("sib_{case}_{idx}") };
                let p = if is_fsim { Posture::ConsumePath } else { Posture::ALL[rng.range(Posture::ALL.len())] };
                doc.push_str(&format!("\n[[foundation]]\nid = \"{id}\"\nname = \"{name}\"\n"));
                if matches!(p, Posture::ConsumePath | Posture::BuildTime) || rng.coin() {
                    doc.push_str(&format!("path = \"../{name}\"\n"));
                }
                if rng.coin() {
                    let rev = if rng.coin() { rng.hex(40) } else { "unadmitted".to_owned() };
                    doc.push_str(&format!("git_rev = \"{rev}\"\n"));
                }
                if p == Posture::Consume {
                    doc.push_str(&format!("version = \"={}.{}.{}\"\n", rng.range(3), rng.range(20), rng.range(10)));
                }
                if rng.coin() { doc.push_str(&format!("toolchain = \"{}\"\n", channels[rng.range(channels.len())])); }
                if is_fsim && rng.coin() {
                    let g: Vec<String> = FSIM_AIRLOCK_FEATURE_GROUPS.iter().take(1 + rng.range(4)).map(|x| format!("\"{x}\"")).collect();
                    doc.push_str(&format!("features = [{}]\n", g.join(", ")));
                }
                if rng.coin() { doc.push_str(&format!("build_env = {{ SALT_{idx} = \"v-{case}\" }}\n")); }
                doc.push_str(&format!("admission_gate = \"{}\"\n", if rng.coin() { format!("INT-G{idx}-01") } else { "none".to_owned() }));
                if rng.coin() { doc.push_str(&format!("receipt = \"{}\"\n", rng.hex(64))); }
                doc.push_str(&format!("fallback = \"fb {id}\"\n"));
                if rng.coin() { doc.push_str(&format!("first_needed = \"{}\"\n", Milestone::ALL[rng.range(9)].as_str())); }
                if rng.coin() { doc.push_str(&format!("critical_path = {}\n", rng.coin())); }
                if rng.coin() { doc.push_str(&format!("note = \"n {case}\"\n")); }
                if rng.coin() { doc.push_str("required_at = [\"M1\"]\noptional_at = [\"M2\"]\nfirewall_cells = { M0 = \"admit\" }\n"); }

                if !is_fsim && rng.coin() {
                    let p2 = if matches!(p, Posture::Consume | Posture::ConsumePath | Posture::BuildTime) { Posture::Pattern } else { Posture::Oracle };
                    if rng.coin() {
                        doc.push_str(&format!(
                            "uses = [{{ posture = \"{}\", purpose = \"a\", confined_to = [\"frob-core\"], tier = \"solid\" }}, {{ posture = \"{}\", purpose = \"b\", confined_to = [\"frob-conformance\"], tier = \"frontier\", first_needed = \"M2\" }}]\n",
                            p.as_str(), p2.as_str()
                        ));
                    } else {
                        doc.push_str(&format!(
                            "\n[[foundation.uses]]\nposture = \"{}\"\npurpose = \"a\"\nconfined_to = [\"frob-core\"]\ntier = \"solid\"\n\n[[foundation.uses]]\nposture = \"{}\"\npurpose = \"b\"\nconfined_to = [\"frob-conformance\"]\ntier = \"frontier\"\nfirst_needed = \"M2\"\n",
                            p.as_str(), p2.as_str()
                        ));
                    }
                } else {
                    doc.push_str(&format!("posture = \"{}\"\n", p.as_str()));
                    if rng.coin() { doc.push_str("confined_to = [\"frob-fsim\"]\n"); }
                }
            }

            let parsed = parse_bytes(doc.as_bytes()).unwrap_or_else(|e| panic!("case {case}: {e}"));
            let c1 = parsed.canonical_bytes();
            let reparsed = parse_bytes(&c1).unwrap_or_else(|e| panic!("case {case} reparse: {e}"));
            assert_eq!(c1, reparsed.canonical_bytes(), "idempotence failed on case {case}");
            assert_eq!(parsed, reparsed);
            assert_eq!(parsed.digest(), reparsed.digest());
        }
    }

    struct XorShift64(u64);
    impl XorShift64 {
        fn next(&mut self) -> u64 { let mut x = self.0; x ^= x << 13; x ^= x >> 7; x ^= x << 17; self.0 = x; x }
        fn range(&mut self, n: usize) -> usize { (self.next() as usize) % n }
        fn coin(&mut self) -> bool { (self.next() & 1) == 1 }
        fn hex(&mut self, len: usize) -> String {
            const H: &[u8; 16] = b"0123456789abcdef";
            (0..len).map(|_| H[self.range(16)] as char).collect()
        }
    }

    fn toml_table_to_json(tbl: &Table) -> mini_json::Val {
        mini_json::Val::Obj(tbl.iter().map(|(k, sp)| (k.clone(), toml_val_to_json(&sp.value))).collect())
    }
    fn toml_val_to_json(val: &Value) -> mini_json::Val {
        match val {
            Value::Str(s) => mini_json::Val::Str(s.clone()),
            Value::Int(i) => mini_json::Val::Int(*i),
            Value::Bool(b) => mini_json::Val::Bool(*b),
            Value::Array(a) => mini_json::Val::Arr(a.iter().map(|sp| toml_val_to_json(&sp.value)).collect()),
            Value::Table(t) => toml_table_to_json(t),
        }
    }

    mod mini_json {
        use std::collections::BTreeMap;

        #[derive(Debug, Clone, PartialEq, Eq)]
        pub enum Val { Null, Bool(bool), Int(i64), Str(String), Arr(Vec<Val>), Obj(BTreeMap<String, Val>) }
        impl Val {
            pub fn get_obj(&self, k: &str) -> Option<&BTreeMap<String, Val>> {
                if let Self::Obj(m) = self && let Some(Self::Obj(sub)) = m.get(k) { Some(sub) } else { None }
            }
            fn as_str(&self) -> Option<&str> { if let Self::Str(s) = self { Some(s) } else { None } }
            fn as_arr(&self) -> Option<&[Val]> { if let Self::Arr(a) = self { Some(a) } else { None } }
            fn as_obj(&self) -> Option<&BTreeMap<String, Val>> { if let Self::Obj(m) = self { Some(m) } else { None } }
        }

        pub fn parse(input: &str) -> Result<Val, String> {
            let mut p = P { b: input.as_bytes(), i: 0 };
            let v = p.val()?;
            p.ws();
            if p.i < p.b.len() { return Err(format!("trailing bytes at {}", p.i)); }
            Ok(v)
        }
        struct P<'a> { b: &'a [u8], i: usize }
        impl<'a> P<'a> {
            fn peek(&self) -> Option<u8> { self.b.get(self.i).copied() }
            fn ws(&mut self) { while matches!(self.peek(), Some(b' ' | b'\t' | b'\n' | b'\r')) { self.i += 1; } }
            fn val(&mut self) -> Result<Val, String> {
                self.ws();
                match self.peek() {
                    Some(b'{') => self.obj(),
                    Some(b'[') => self.arr(),
                    Some(b'"') => self.str().map(Val::Str),
                    Some(b't') => self.lit(b"true", Val::Bool(true)),
                    Some(b'f') => self.lit(b"false", Val::Bool(false)),
                    Some(b'n') => self.lit(b"null", Val::Null),
                    Some(b'-' | b'0'..=b'9') => self.int(),
                    o => Err(format!("unexpected {o:?} at {}", self.i)),
                }
            }
            fn lit(&mut self, lit: &[u8], v: Val) -> Result<Val, String> {
                if self.b.get(self.i..self.i + lit.len()) == Some(lit) { self.i += lit.len(); Ok(v) }
                else { Err(format!("expected literal at {}", self.i)) }
            }
            fn int(&mut self) -> Result<Val, String> {
                let s = self.i;
                if self.peek() == Some(b'-') { self.i += 1; }
                while self.peek().is_some_and(|c| c.is_ascii_digit()) { self.i += 1; }
                std::str::from_utf8(&self.b[s..self.i]).map_err(|e| e.to_string())?.parse::<i64>().map(Val::Int).map_err(|e| e.to_string())
            }
            fn str(&mut self) -> Result<String, String> {
                if self.peek() != Some(b'"') { return Err("expected quote".into()); }
                self.i += 1;
                let mut out = String::new();
                while let Some(c) = self.peek() {
                    self.i += 1;
                    match c {
                        b'"' => return Ok(out),
                        b'\\' => {
                            let e = self.peek().ok_or("eof in escape")?;
                            self.i += 1;
                            match e {
                                b'"' => out.push('"'), b'\\' => out.push('\\'), b'/' => out.push('/'),
                                b'n' => out.push('\n'), b'r' => out.push('\r'), b't' => out.push('\t'),
                                b'u' => {
                                    let hex = std::str::from_utf8(self.b.get(self.i..self.i + 4).ok_or("bad \\u")?).map_err(|x| x.to_string())?;
                                    self.i += 4;
                                    out.push(char::from_u32(u32::from_str_radix(hex, 16).map_err(|x| x.to_string())?).ok_or("bad scalar")?);
                                }
                                _ => return Err(format!("bad escape \\{}", e as char)),
                            }
                        }
                        x => out.push(x as char),
                    }
                }
                Err("unterminated string".into())
            }
            fn arr(&mut self) -> Result<Val, String> {
                self.i += 1; self.ws();
                let mut xs = Vec::new();
                if self.peek() == Some(b']') { self.i += 1; return Ok(Val::Arr(xs)); }
                loop {
                    xs.push(self.val()?); self.ws();
                    match self.peek() {
                        Some(b',') => self.i += 1,
                        Some(b']') => { self.i += 1; return Ok(Val::Arr(xs)); }
                        _ => return Err("expected `,` or `]`".into()),
                    }
                }
            }
            fn obj(&mut self) -> Result<Val, String> {
                self.i += 1; self.ws();
                let mut m = BTreeMap::new();
                if self.peek() == Some(b'}') { self.i += 1; return Ok(Val::Obj(m)); }
                loop {
                    self.ws(); let k = self.str()?; self.ws();
                    if self.peek() != Some(b':') { return Err("expected `:`".into()); }
                    self.i += 1;
                    let v = self.val()?;
                    if m.insert(k.clone(), v).is_some() { return Err(format!("dup key `{k}`")); }
                    self.ws();
                    match self.peek() {
                        Some(b',') => self.i += 1,
                        Some(b'}') => { self.i += 1; return Ok(Val::Obj(m)); }
                        _ => return Err("expected `,` or `}`".into()),
                    }
                }
            }
        }

        pub fn validate(inst: &Val, schema: &Val, root: &Val) -> Result<(), String> {
            let obj = schema.as_obj().ok_or("schema must be object")?;
            if let Some(r) = obj.get("$ref").and_then(Val::as_str) {
                let mut cur = root;
                for seg in r.strip_prefix("#/").ok_or("bad $ref")?.split('/') {
                    cur = cur.as_obj().and_then(|m| m.get(seg)).ok_or("missing $ref")?;
                }
                return validate(inst, cur, root);
            }
            if let Some(ty) = obj.get("type").and_then(Val::as_str) {
                let ok = match ty {
                    "object" => matches!(inst, Val::Obj(_)), "array" => matches!(inst, Val::Arr(_)),
                    "string" => matches!(inst, Val::Str(_)), "boolean" => matches!(inst, Val::Bool(_)),
                    "integer" => matches!(inst, Val::Int(_)), _ => true,
                };
                if !ok { return Err(format!("expected {ty}")); }
            }
            if let Some(en) = obj.get("enum").and_then(Val::as_arr) && !en.contains(inst) {
                return Err(format!("{inst:?} not in enum"));
            }
            if let Some(c) = obj.get("const") && inst != c { return Err("const mismatch".into()); }
            if let Some(Val::Int(n)) = obj.get("minLength") && let Val::Str(s) = inst && s.chars().count() < (*n as usize) {
                return Err("below minLength".into());
            }
            if let Val::Obj(map) = inst {
                if let Some(reqs) = obj.get("required").and_then(Val::as_arr) {
                    for r in reqs { if let Some(k) = r.as_str() && !map.contains_key(k) { return Err(format!("missing `{k}`")); } }
                }
                let props = obj.get("properties").and_then(Val::as_obj);
                let addl = obj.get("additionalProperties");
                for (k, v) in map {
                    if let Some(ps) = props.and_then(|p| p.get(k)) { validate(v, ps, root)?; }
                    else if matches!(addl, Some(Val::Bool(false))) { return Err(format!("unexpected property `{k}`")); }
                    else if let Some(s @ Val::Obj(_)) = addl { validate(v, s, root)?; }
                }
            }
            if let Val::Arr(items) = inst {
                if let Some(Val::Int(n)) = obj.get("minItems") && items.len() < (*n as usize) { return Err("below minItems".into()); }
                if let Some(it) = obj.get("items") { for x in items { validate(x, it, root)?; } }
            }
            if let Some(ns) = obj.get("not") && validate(inst, ns, root).is_ok() { return Err("matched `not`".into()); }
            if let Some(ao) = obj.get("anyOf").and_then(Val::as_arr) && !ao.iter().any(|s| validate(inst, s, root).is_ok()) {
                return Err("failed `anyOf`".into());
            }
            if let Some(oo) = obj.get("oneOf").and_then(Val::as_arr) && oo.iter().filter(|s| validate(inst, s, root).is_ok()).count() != 1 {
                return Err("failed `oneOf`".into());
            }
            if let Some(all) = obj.get("allOf").and_then(Val::as_arr) { for s in all { validate(inst, s, root)?; } }
            if let (Some(i), Some(t)) = (obj.get("if"), obj.get("then")) && validate(inst, i, root).is_ok() { validate(inst, t, root)?; }
            Ok(())
        }
    }
}
