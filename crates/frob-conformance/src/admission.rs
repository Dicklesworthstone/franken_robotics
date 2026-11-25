#![deny(unsafe_code)]

//! Admission record writer (`frob.admission/1`) and CLI entrypoint for
//! `cargo run -p frob-conformance --example admission -- record ...`
//! (bead `N33a-07`, §33.6, INV-SUITE-03, GATE-GAUNT-10).
//!
//! Uses `frob_core::canon` (`N34-18`) as the sole canonical JSON writer,
//! `frob_core::json` (`N34-17`) as the sole JSON reader,
//! `frob_core::id::hash_blob` for plain BLAKE3 content addressing, and
//! `frob_core::registry::toml` (`N34p2-01`) for TOML reads.
//!
//! Never writes `registries/gates.toml` (only `examples/record_gate.rs` writes
//! gate register rows under `N38b-19`), refuses to overwrite existing records,
//! runs commands through `PATH` under a bounded timeout, enforces rules 4–8
//! before writing `status = "pass"`, uses zero `unsafe`, and never deletes
//! files or directories (COMMON.md 9b).

use std::collections::BTreeSet;
use std::fmt;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;
use std::time::{Duration, Instant};

use frob_core::canon::{self, CanonError, Canonical, CanonicalBytes, Value};
use frob_core::id::hash_blob;
use frob_core::json::{self, Json, Limits};
use frob_core::registry::toml::{self, Value as TomlValue};

/// Schema identifier for admission records (`N33a-07`).
pub const ADMISSION_SCHEMA: &str = "frob.admission/1";

/// Maximum length in bytes of `output_excerpt` (`4 KiB`).
pub const MAX_EXCERPT_BYTES: usize = 4096;

/// Default wall-time timeout for an admission command (30 minutes, within machine envelope).
pub const DEFAULT_TIMEOUT_SECS: u64 = 1800;

/// Known admission gates mapped to their foundation and required own-scope thresholds.
struct GateDefault {
    id: &'static str,
    foundation: &'static str,
    own_crates: &'static [&'static str],
    own_groups: &'static [&'static str],
    own_tool: Option<&'static str>,
    required_thresholds: &'static [(&'static str, &'static str)],
}

const KNOWN_GATES: &[GateDefault] = &[
    GateDefault { id: "INT-TOOLCHAIN-01", foundation: "WORKSPACE", own_crates: &[], own_groups: &[], own_tool: None, required_thresholds: &[] },
    GateDefault {
        id: "INT-FSIM-01",
        foundation: "FD-FSIM",
        own_crates: &["fs-contact", "fs-ga", "fs-mbd", "fs-query", "fs-time"],
        own_groups: &[],
        own_tool: None,
        required_thresholds: &[("cold_check_s", "s")],
    },
    GateDefault { id: "INT-FSIM-02", foundation: "FD-FSIM", own_crates: &["fs-ga"], own_groups: &[], own_tool: None, required_thresholds: &[] },
    GateDefault { id: "INT-FSIM-03", foundation: "FD-FSIM", own_crates: &["fs-mbd"], own_groups: &[], own_tool: None, required_thresholds: &[] },
    GateDefault { id: "INT-FSIM-04", foundation: "FD-FSIM", own_crates: &["fs-contact"], own_groups: &[], own_tool: None, required_thresholds: &[] },
    GateDefault { id: "INT-FSIM-05", foundation: "FD-FSIM", own_crates: &["fs-mbd"], own_groups: &[], own_tool: None, required_thresholds: &[] },
    GateDefault { id: "INT-FSIM-06", foundation: "FD-FSIM", own_crates: &["fs-g1-train"], own_groups: &[], own_tool: None, required_thresholds: &[] },
    GateDefault { id: "INT-FSIM-07", foundation: "FD-FSIM", own_crates: &["fs-contact"], own_groups: &[], own_tool: None, required_thresholds: &[] },
    GateDefault { id: "INT-FSIM-08", foundation: "FD-FSIM", own_crates: &["fs-time"], own_groups: &[], own_tool: None, required_thresholds: &[] },
    GateDefault { id: "INT-FSIM-09", foundation: "FD-FSIM", own_crates: &["fs-matdb", "fs-phs", "fs-tribo"], own_groups: &[], own_tool: None, required_thresholds: &[] },
    GateDefault { id: "INT-FSIM-10", foundation: "FD-FSIM", own_crates: &["fs-query"], own_groups: &[], own_tool: None, required_thresholds: &[] },
    GateDefault { id: "INT-FSIM-11", foundation: "FD-FSIM", own_crates: &["fs-mbd"], own_groups: &[], own_tool: None, required_thresholds: &[] },
    GateDefault { id: "INT-FSIM-13", foundation: "FD-FSIM", own_crates: &[], own_groups: &["spectral"], own_tool: None, required_thresholds: &[] },
    GateDefault { id: "INT-FSIM-14", foundation: "FD-FSIM", own_crates: &[], own_groups: &["stats"], own_tool: None, required_thresholds: &[] },
    GateDefault { id: "INT-FSIM-15", foundation: "FD-FSIM", own_crates: &[], own_groups: &["structure"], own_tool: None, required_thresholds: &[] },
    GateDefault { id: "INT-FSIM-16", foundation: "FD-FSIM", own_crates: &["fs-recompute"], own_groups: &[], own_tool: None, required_thresholds: &[] },
    GateDefault { id: "INT-FSIM-17", foundation: "FD-FSIM", own_crates: &[], own_groups: &["multiphysics"], own_tool: None, required_thresholds: &[] },
    GateDefault { id: "INT-FSIM-18", foundation: "FD-FSIM", own_crates: &[], own_groups: &[], own_tool: Some("frob-fsim-worker"), required_thresholds: &[] },
    GateDefault { id: "INT-ASUP-01", foundation: "FD-ASUP", own_crates: &["asupersync"], own_groups: &[], own_tool: None, required_thresholds: &[] },
    GateDefault { id: "INT-ASUP-02", foundation: "FD-ASUP", own_crates: &["asupersync"], own_groups: &[], own_tool: None, required_thresholds: &[] },
    GateDefault { id: "INT-FSQL-01", foundation: "FD-FSQL", own_crates: &["fsqlite"], own_groups: &[], own_tool: None, required_thresholds: &[] },
    GateDefault { id: "INT-FSQL-02", foundation: "FD-FSQL", own_crates: &["fsqlite"], own_groups: &[], own_tool: None, required_thresholds: &[] },
    GateDefault { id: "INT-FT-01", foundation: "FD-FT", own_crates: &["ft-kernel-metal"], own_groups: &[], own_tool: None, required_thresholds: &[] },
    GateDefault { id: "INT-FT-02", foundation: "FD-FT", own_crates: &["ft-kernel-metal"], own_groups: &[], own_tool: None, required_thresholds: &[] },
    GateDefault { id: "INT-CUDA-01", foundation: "FD-CUDA", own_crates: &["cudarc"], own_groups: &[], own_tool: None, required_thresholds: &[] },
    GateDefault { id: "INT-FSYM-01", foundation: "FD-FSYM", own_crates: &["fsym-core"], own_groups: &[], own_tool: None, required_thresholds: &[] },
    GateDefault { id: "INT-FSYM-02", foundation: "FD-FSYM", own_crates: &["fsym-core"], own_groups: &[], own_tool: None, required_thresholds: &[] },
    GateDefault { id: "INT-FLN-01", foundation: "FD-FLN", own_crates: &[], own_groups: &[], own_tool: Some("fln"), required_thresholds: &[] },
    GateDefault { id: "INT-FLN-02", foundation: "FD-FLN", own_crates: &[], own_groups: &[], own_tool: Some("fln"), required_thresholds: &[] },
    GateDefault { id: "INT-FMCP-01", foundation: "FD-FMCP", own_crates: &["fastmcp"], own_groups: &[], own_tool: None, required_thresholds: &[] },
    GateDefault { id: "INT-FNP-01", foundation: "FD-FNP", own_crates: &["fnp-io"], own_groups: &[], own_tool: None, required_thresholds: &[] },
    GateDefault { id: "INT-FNP-02", foundation: "FD-FNP", own_crates: &["fnp-io"], own_groups: &[], own_tool: None, required_thresholds: &[] },
    GateDefault { id: "INT-FTUI-01", foundation: "FD-FTUI", own_crates: &["ftui"], own_groups: &[], own_tool: None, required_thresholds: &[] },
    GateDefault { id: "INT-F3D-01", foundation: "FD-F3D", own_crates: &[], own_groups: &[], own_tool: Some("franken_threed"), required_thresholds: &[] },
    GateDefault { id: "INT-FDGR-01", foundation: "FD-FDGR", own_crates: &[], own_groups: &[], own_tool: Some("fdgr"), required_thresholds: &[] },
    GateDefault { id: "INT-FJAX-01", foundation: "FD-FJAX", own_crates: &[], own_groups: &[], own_tool: None, required_thresholds: &[] },
    GateDefault { id: "INT-FTX-01", foundation: "FD-FTX", own_crates: &[], own_groups: &[], own_tool: Some("ftx-drape"), required_thresholds: &[] },
];

fn lookup_gate(gate_id: &str) -> Option<&'static GateDefault> {
    KNOWN_GATES.iter().find(|g| g.id == gate_id)
}

// ============================================================================
// Wire types matching `frob.admission/1` byte-for-byte with `xtask::stack::admission`
// ============================================================================

/// Numeric value in a measured entry (exact int53 or finite IEEE-754 f64).
#[derive(Debug, Clone, Copy)]
pub enum MeasureNum {
    Int(i64),
    Float(f64),
}

impl PartialEq for MeasureNum {
    fn eq(&self, other: &Self) -> bool {
        match (*self, *other) {
            (Self::Int(a), Self::Int(b)) => a == b,
            (Self::Float(a), Self::Float(b)) => a.to_bits() == b.to_bits(),
            _ => false,
        }
    }
}

impl Eq for MeasureNum {}

impl MeasureNum {
    #[must_use]
    pub fn as_f64(self) -> f64 {
        match self {
            Self::Int(n) => n as f64,
            Self::Float(x) => x,
        }
    }
}

impl Canonical for MeasureNum {
    fn write(&self, w: Value<'_>) -> Result<(), CanonError> {
        match *self {
            Self::Int(n) => w.int53(n),
            Self::Float(x) => w.f64_shortest(x),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MeasureRelation {
    Le,
    Lt,
    Ge,
    Gt,
    Eq,
}

impl MeasureRelation {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Le => "<=",
            Self::Lt => "<",
            Self::Ge => ">=",
            Self::Gt => ">",
            Self::Eq => "==",
        }
    }

    pub fn parse(raw: &str) -> Result<Self, AdmissionError> {
        match raw {
            "<=" => Ok(Self::Le),
            "<" => Ok(Self::Lt),
            ">=" => Ok(Self::Ge),
            ">" => Ok(Self::Gt),
            "==" => Ok(Self::Eq),
            other => Err(AdmissionError::new(
                "parse-error",
                format!("unknown measured relation `{other}`"),
            )),
        }
    }

    #[must_use]
    pub fn satisfied(self, value: MeasureNum, threshold: MeasureNum) -> bool {
        let a = value.as_f64();
        let b = threshold.as_f64();
        match self {
            Self::Le => a <= b,
            Self::Lt => a < b,
            Self::Ge => a >= b,
            Self::Gt => a > b,
            Self::Eq => (a - b).abs() == 0.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolSpec {
    pub name: String,
    pub rev: Option<String>,
    pub binary_hash: Option<String>,
}

impl ToolSpec {
    #[must_use]
    pub fn is_pinned(&self) -> bool {
        !self.name.is_empty()
            && self.rev.as_deref().is_some_and(|r| is_lower_hex(r, 40))
            && self.binary_hash.as_deref().is_some_and(|h| is_lower_hex(h, 64))
    }
}

impl Canonical for ToolSpec {
    fn write(&self, w: Value<'_>) -> Result<(), CanonError> {
        if self.rev.is_none() && self.binary_hash.is_none() {
            w.str(&self.name);
            return Ok(());
        }
        w.object(|o| {
            match &self.binary_hash {
                Some(h) => o.str("binary_hash", h)?,
                None => o.null("binary_hash")?,
            }
            o.str("name", &self.name)?;
            match &self.rev {
                Some(r) => o.str("rev", r)?,
                None => o.null("rev")?,
            }
            Ok(())
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AdmissionCovers {
    pub crates: Vec<String>,
    pub groups: Vec<String>,
    pub tool: Option<ToolSpec>,
}

impl AdmissionCovers {
    fn normalized_key(&self) -> (BTreeSet<String>, BTreeSet<String>, Option<String>) {
        (
            self.crates.iter().cloned().collect(),
            self.groups.iter().cloned().collect(),
            self.tool.as_ref().map(|t| t.name.clone()),
        )
    }
}

impl Canonical for AdmissionCovers {
    fn write(&self, w: Value<'_>) -> Result<(), CanonError> {
        w.object(|o| {
            o.array("crates", |a| {
                for c in &self.crates {
                    a.push_str(c);
                }
                Ok(())
            })?;
            o.array("groups", |a| {
                for g in &self.groups {
                    a.push_str(g);
                }
                Ok(())
            })?;
            match &self.tool {
                Some(t) => o.member_term("tool", t)?,
                None => o.null("tool")?,
            }
            Ok(())
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MeasuredEntry {
    pub name: String,
    pub value: Option<MeasureNum>,
    pub unit: String,
    pub threshold: MeasureNum,
    pub relation: MeasureRelation,
}

impl Canonical for MeasuredEntry {
    fn write(&self, w: Value<'_>) -> Result<(), CanonError> {
        w.object(|o| {
            o.str("name", &self.name)?;
            o.str("relation", self.relation.as_str())?;
            o.member_term("threshold", &self.threshold)?;
            o.str("unit", &self.unit)?;
            match &self.value {
                Some(v) => o.member_term("value", v)?,
                None => o.null("value")?,
            }
            Ok(())
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlResult {
    pub id: String,
    pub caught: bool,
}

impl Canonical for ControlResult {
    fn write(&self, w: Value<'_>) -> Result<(), CanonError> {
        w.object(|o| {
            o.bool("caught", self.caught)?;
            o.str("id", &self.id)?;
            Ok(())
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FactReread {
    pub fact: String,
    pub path: String,
    pub value: String,
}

impl Canonical for FactReread {
    fn write(&self, w: Value<'_>) -> Result<(), CanonError> {
        w.object(|o| {
            o.str("fact", &self.fact)?;
            o.str("path", &self.path)?;
            o.str("value", &self.value)?;
            Ok(())
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdmissionStatus {
    Pass,
    Fail,
    NotRun,
    Killed,
}

impl AdmissionStatus {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pass => "pass",
            Self::Fail => "fail",
            Self::NotRun => "not-run",
            Self::Killed => "killed",
        }
    }

    pub fn parse(raw: &str) -> Result<Self, AdmissionError> {
        match raw {
            "pass" => Ok(Self::Pass),
            "fail" => Ok(Self::Fail),
            "not-run" => Ok(Self::NotRun),
            "killed" => Ok(Self::Killed),
            other => Err(AdmissionError::new(
                "parse-error",
                format!("unknown admission status `{other}`"),
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostInfo {
    pub class: String,
    pub fingerprint: Option<String>,
}

impl Canonical for HostInfo {
    fn write(&self, w: Value<'_>) -> Result<(), CanonError> {
        w.object(|o| {
            o.str("class", &self.class)?;
            match &self.fingerprint {
                Some(fp) => o.str("fingerprint", fp)?,
                None => o.null("fingerprint")?,
            }
            Ok(())
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordedBy {
    pub date: String,
    pub session: String,
    pub implemented_by: String,
}

impl Canonical for RecordedBy {
    fn write(&self, w: Value<'_>) -> Result<(), CanonError> {
        w.object(|o| {
            o.str("date", &self.date)?;
            o.str("implemented_by", &self.implemented_by)?;
            o.str("session", &self.session)?;
            Ok(())
        })
    }
}

/// Complete `frob.admission/1` record produced by [`record`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdmissionRecord {
    pub schema: String,
    pub gate: String,
    pub foundation: String,
    pub covers: AdmissionCovers,
    pub repo_rev: String,
    pub workspace: String,
    pub stack_receipt: String,
    pub sibling_rev: Option<String>,
    pub toolchain: String,
    pub harness_receipt: Option<String>,
    pub command: Vec<String>,
    pub exit: i64,
    pub output_hash: String,
    pub output_excerpt: String,
    pub measured: Vec<MeasuredEntry>,
    pub controls: Vec<ControlResult>,
    pub facts_reread: Vec<FactReread>,
    pub status: AdmissionStatus,
    pub host: HostInfo,
    pub recorded: RecordedBy,
}

impl Canonical for AdmissionRecord {
    fn write(&self, w: Value<'_>) -> Result<(), CanonError> {
        w.object(|o| {
            o.array("command", |a| {
                for arg in &self.command {
                    a.push_str(arg);
                }
                Ok(())
            })?;
            o.array("controls", |a| {
                for c in &self.controls {
                    a.push_term(c)?;
                }
                Ok(())
            })?;
            o.member_term("covers", &self.covers)?;
            o.int53("exit", self.exit)?;
            o.array("facts_reread", |a| {
                for f in &self.facts_reread {
                    a.push_term(f)?;
                }
                Ok(())
            })?;
            o.str("foundation", &self.foundation)?;
            o.str("gate", &self.gate)?;
            match &self.harness_receipt {
                Some(h) => o.str("harness_receipt", h)?,
                None => o.null("harness_receipt")?,
            }
            o.member_term("host", &self.host)?;
            o.array("measured", |a| {
                for m in &self.measured {
                    a.push_term(m)?;
                }
                Ok(())
            })?;
            o.str("output_excerpt", &self.output_excerpt)?;
            o.str("output_hash", &self.output_hash)?;
            o.member_term("recorded", &self.recorded)?;
            o.str("repo_rev", &self.repo_rev)?;
            o.str("schema", &self.schema)?;
            match &self.sibling_rev {
                Some(r) => o.str("sibling_rev", r)?,
                None => o.null("sibling_rev")?,
            }
            o.str("stack_receipt", &self.stack_receipt)?;
            o.str("status", self.status.as_str())?;
            o.str("toolchain", &self.toolchain)?;
            o.str("workspace", &self.workspace)?;
            Ok(())
        })
    }
}

impl AdmissionRecord {
    pub fn to_canonical_bytes(&self) -> Result<CanonicalBytes, AdmissionError> {
        canon::to_canonical(self)
            .map_err(|e| AdmissionError::new("canon-error", e.to_string()))
    }

    pub fn from_json_bytes(bytes: &[u8]) -> Result<Self, AdmissionError> {
        let admitted = json::admit(bytes, &Limits::default())
            .map_err(|e| AdmissionError::new("parse-error", e.to_string()))?;
        decode_record(admitted.value())
    }
}

/// Refusal or fault returned by the admission writer or CLI.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdmissionError {
    /// Machine-readable finding kind (`pass-on-dirty`, `pass-without-control`,
    /// `self-certified`, `tool-unpinned`, `measured-missing`, `record-exists`, etc.).
    pub kind: &'static str,
    pub message: String,
}

impl AdmissionError {
    #[must_use]
    pub fn new(kind: &'static str, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
        }
    }
}

impl fmt::Display for AdmissionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.kind, self.message)
    }
}

impl std::error::Error for AdmissionError {}

/// Input payload parsed from `--input <path>` JSON or constructed directly in Rust tests.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordInput {
    pub covers: AdmissionCovers,
    pub command: Vec<String>,
    pub measured: Vec<MeasuredEntry>,
    pub controls: Vec<ControlResult>,
    pub facts_reread: Vec<FactReread>,
    /// Optional explicit foundation (required for rowless `ADMIT-*` gates per Review Note 5,
    /// or cross-foundation first-use records).
    pub foundation: Option<String>,
    /// Optional workspace name (defaults to `"core"`).
    pub workspace: Option<String>,
    /// Optional explicit status override (`fail`, `not-run`, `killed`). When `None`,
    /// derived from command exit + threshold checks (`pass` if exit == 0 and all
    /// thresholds satisfied; otherwise `fail`).
    pub status: Option<AdmissionStatus>,
    /// Optional host metadata (defaults to `m5-ultra` / `trj`).
    pub host: Option<HostInfo>,
    /// Optional attribution block (defaults from environment / CLI flags).
    pub recorded: Option<RecordedBy>,
}

/// Options controlling [`record`] execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordOptions {
    pub repo_root: PathBuf,
    pub gate: String,
    pub harness_receipt: Option<String>,
    pub timeout_secs: u64,
    pub date_override: Option<String>,
    pub session_override: Option<String>,
    pub implemented_by_override: Option<String>,
    pub host_class_override: Option<String>,
}

impl RecordOptions {
    #[must_use]
    pub fn new(repo_root: impl Into<PathBuf>, gate: impl Into<String>) -> Self {
        Self {
            repo_root: repo_root.into(),
            gate: gate.into(),
            harness_receipt: None,
            timeout_secs: DEFAULT_TIMEOUT_SECS,
            date_override: None,
            session_override: None,
            implemented_by_override: None,
            host_class_override: None,
        }
    }
}

/// Result returned when [`record`] atomically stores an admission record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordOutcome {
    /// 64-hex BLAKE3 digest of the canonical admission record bytes.
    pub record_hash: String,
    /// Repository-relative path (`receipts/admissions/<gate>/<hash>.json`).
    pub rel_path: String,
    /// Absolute path written.
    pub abs_path: PathBuf,
    /// Final recorded status.
    pub status: AdmissionStatus,
    /// Stored canonical record.
    pub record: AdmissionRecord,
}

impl Canonical for RecordOutcome {
    fn write(&self, w: Value<'_>) -> Result<(), CanonError> {
        w.object(|o| {
            o.str("path", &self.rel_path)?;
            o.str("record", &self.record_hash)?;
            o.str("status", self.status.as_str())?;
            Ok(())
        })
    }
}

// ============================================================================
// Core writer (`frob_conformance::admission::record`)
// ============================================================================

/// Run the admission command under a bounded timeout, capture and store its
/// content-addressed output, read and verify the current stack receipt, enforce
/// validity rules 4–8 before allowing `status = "pass"`, and atomically write
/// `receipts/admissions/<gate>/<hash>.json`.
///
/// Never writes `registries/gates.toml`, refuses to overwrite an existing
/// record file, and writes NOTHING to `receipts/admissions/` when a `pass`
/// guard refuses.
pub fn record(
    opts: &RecordOptions,
    input: &RecordInput,
) -> Result<RecordOutcome, AdmissionError> {
    if !is_valid_gate_id(&opts.gate) {
        return Err(AdmissionError::new(
            "invalid-gate",
            format!("invalid gate identifier `{}`", opts.gate),
        ));
    }
    if let Some(hr) = &opts.harness_receipt
        && !is_lower_hex(hr, 64)
    {
        return Err(AdmissionError::new(
            "invalid-harness-receipt",
            format!("`--harness-receipt` must be 64 lower-case hex digits, got `{hr}`"),
        ));
    }
    if input.command.is_empty() {
        return Err(AdmissionError::new(
            "invalid-command",
            "`command` must be a non-empty argv array",
        ));
    }

    let repo_root = &opts.repo_root;
    let workspace = input.workspace.as_deref().unwrap_or("core").to_owned();
    let foundation = resolve_foundation(&opts.gate, input.foundation.as_deref())?;

    // 1. Read repo HEAD and stack receipt (`receipts/HEAD` + `receipts/<hash>.json`).
    let repo_rev = git_head_ro(repo_root)?;
    let (receipt_hash, sr_info) = load_head_stack_receipt(repo_root)?;
    let sibling_rev = sr_info
        .siblings
        .iter()
        .find(|s| s.fd == foundation)
        .map(|s| s.rev.clone());

    // 2. Run the measuring command through PATH under bounded timeout.
    let (exit_code, raw_output) =
        run_command_with_timeout(repo_root, &input.command, Duration::from_secs(opts.timeout_secs))?;

    // 3. Determine target status.
    let all_thresholds_met = input.measured.iter().all(|m| {
        m.value
            .is_some_and(|v| m.relation.satisfied(v, m.threshold))
    });
    let status = match input.status {
        Some(s) => s,
        None => {
            if exit_code == 0 && all_thresholds_met {
                AdmissionStatus::Pass
            } else {
                AdmissionStatus::Fail
            }
        }
    };

    // 4. Resolve host and attribution.
    let host = input.host.clone().unwrap_or_else(|| HostInfo {
        class: opts
            .host_class_override
            .clone()
            .unwrap_or_else(default_host_class),
        fingerprint: None,
    });
    let mut recorded = input.recorded.clone().unwrap_or_else(|| RecordedBy {
        date: "2026-11-24T00:00:00Z".to_owned(),
        session: std::env::var("FROB_SESSION").unwrap_or_default(),
        implemented_by: std::env::var("FROB_IMPLEMENTED_BY").unwrap_or_default(),
    });
    if let Some(d) = &opts.date_override {
        recorded.date = d.clone();
    }
    if let Some(s) = &opts.session_override {
        recorded.session = s.clone();
    }
    if let Some(ib) = &opts.implemented_by_override {
        recorded.implemented_by = ib.clone();
    }

    // 5. Enforce rules 4–8 BEFORE writing any output or record file.
    //    If a `pass` is refused, return Err naming the exact finding and write NOTHING!
    if status == AdmissionStatus::Pass {
        // Rule 4 (`pass-on-dirty`): INV-SUITE-03 dirty-tree + asupersync-duplicate guard.
        if sr_info.repo_dirty {
            return Err(AdmissionError::new(
                "pass-on-dirty",
                "dirty-stack: repository working tree was dirty in stack receipt (INV-SUITE-03)",
            ));
        }
        let dirty_sibs: Vec<&str> = sr_info
            .siblings
            .iter()
            .filter(|s| s.dirty)
            .map(|s| s.fd.as_str())
            .collect();
        if !dirty_sibs.is_empty() {
            return Err(AdmissionError::new(
                "pass-on-dirty",
                format!(
                    "dirty-stack: sibling checkout(s) were dirty in stack receipt: {} (INV-SUITE-03)",
                    dirty_sibs.join(", ")
                ),
            ));
        }
        if sr_info
            .duplicates
            .iter()
            .any(|d| d == "asupersync" || d.starts_with("asupersync "))
        {
            return Err(AdmissionError::new(
                "pass-on-dirty",
                "dirty-stack: stack receipt records duplicate asupersync package(s) (INV-SUITE-03)",
            ));
        }

        // Rule 5 (`pass-without-control`).
        let any_caught = input.controls.iter().any(|c| c.caught);
        let any_missed = input.controls.iter().any(|c| !c.caught);
        if !any_caught || any_missed {
            return Err(AdmissionError::new(
                "pass-without-control",
                "status `pass` requires at least one caught control and zero uncaught controls",
            ));
        }

        // Rule 6 (`self-certified`).
        let sess = recorded.session.trim();
        let impl_by = recorded.implemented_by.trim();
        if sess.is_empty() || impl_by.is_empty() || sess == impl_by {
            return Err(AdmissionError::new(
                "self-certified",
                format!(
                    "status `pass` refused: session (`{sess}`) must be non-empty and differ from implemented_by (`{impl_by}`)"
                ),
            ));
        }
    }

    // Rule 7 (`tool-unpinned`): Subprocess covers require pinned rev + binary_hash.
    if let Some(tool) = &input.covers.tool
        && !tool.is_pinned()
    {
        return Err(AdmissionError::new(
            "tool-unpinned",
            format!(
                "subprocess tool `{}` must specify 40-hex rev and 64-hex binary_hash",
                tool.name
            ),
        ));
    }

    // Rule 8 (`measured-missing`): reject target-without-value and missing required gate-row thresholds.
    for entry in &input.measured {
        if entry.value.is_none() || entry.unit.trim().is_empty() {
            return Err(AdmissionError::new(
                "measured-missing",
                format!(
                    "measured entry `{}` is missing numeric `value` or non-empty `unit`",
                    entry.name
                ),
            ));
        }
    }
    if let Some(spec) = lookup_gate(&opts.gate) {
        let is_own_scope = is_gate_own_scope_spec(spec, &foundation, &input.covers);
        if is_own_scope {
            for &(req_name, req_unit) in spec.required_thresholds {
                let ok = input.measured.iter().any(|m| {
                    m.name == req_name
                        && m.value.is_some()
                        && (req_unit.is_empty() || m.unit == req_unit)
                });
                if !ok {
                    return Err(AdmissionError::new(
                        "measured-missing",
                        format!(
                            "gate `{}` requires measured threshold `{req_name}` ({req_unit})",
                            opts.gate
                        ),
                    ));
                }
            }
        } else if status == AdmissionStatus::Pass
            && !input
                .measured
                .iter()
                .any(|m| m.value.is_some() && !m.unit.trim().is_empty())
        {
            return Err(AdmissionError::new(
                "measured-missing",
                format!(
                    "first-use passing record under `{}` requires at least one measured value with unit",
                    opts.gate
                ),
            ));
        }
    }

    // 6. Compute output hash & UTF-8-safe tail excerpt (<= 4 KiB), build canonical record.
    let output_hash = hash_blob(&raw_output).to_hex();
    let output_excerpt = utf8_tail_excerpt(&raw_output, MAX_EXCERPT_BYTES);

    let admission_record = AdmissionRecord {
        schema: ADMISSION_SCHEMA.to_owned(),
        gate: opts.gate.clone(),
        foundation,
        covers: input.covers.clone(),
        repo_rev,
        workspace,
        stack_receipt: format!("sr:{receipt_hash}"),
        sibling_rev,
        toolchain: sr_info.toolchain,
        harness_receipt: opts.harness_receipt.clone(),
        command: input.command.clone(),
        exit: exit_code,
        output_hash: output_hash.clone(),
        output_excerpt,
        measured: input.measured.clone(),
        controls: input.controls.clone(),
        facts_reread: input.facts_reread.clone(),
        status,
        host,
        recorded,
    };

    let canon = admission_record.to_canonical_bytes()?;
    let record_hash = hash_blob(canon.as_bytes()).to_hex();
    let rel_path = format!("receipts/admissions/{}/{record_hash}.json", opts.gate);
    let abs_path = repo_root.join(&rel_path);

    // Refuse to overwrite an existing admission record (append-only).
    if abs_path.exists() {
        return Err(AdmissionError::new(
            "record-exists",
            format!("refusing to overwrite existing admission record `{rel_path}`"),
        ));
    }

    // 7. Atomically store content-addressed command output (`receipts/outputs/<hash>.txt`),
    //    leaving an intact identical blob untouched and refusing mismatched existing blobs.
    let output_path = repo_root
        .join("receipts/outputs")
        .join(format!("{output_hash}.txt"));
    if output_path.exists() {
        let existing = fs::read(&output_path)
            .map_err(|e| AdmissionError::new("io-error", format!("read existing output: {e}")))?;
        if existing != raw_output {
            return Err(AdmissionError::new(
                "output-collision",
                format!("existing output blob `{}` differs in bytes", output_path.display()),
            ));
        }
    } else {
        atomic_write_new(&output_path, &raw_output)?;
    }

    // 8. Atomically store canonical admission record (`receipts/admissions/<gate>/<hash>.json`).
    atomic_write_new(&abs_path, canon.as_bytes())?;

    Ok(RecordOutcome {
        record_hash,
        rel_path,
        abs_path,
        status,
        record: admission_record,
    })
}

/// Parse `--input <path>` JSON object via `frob_core::json::admit`.
pub fn parse_input_json(bytes: &[u8]) -> Result<RecordInput, AdmissionError> {
    let admitted = json::admit(bytes, &Limits::default())
        .map_err(|e| AdmissionError::new("parse-error", e.to_string()))?;
    let root = admitted.value();
    let obj = root
        .as_object()
        .ok_or_else(|| AdmissionError::new("parse-error", "input JSON must be an object"))?;
    const ALLOWED: &[&str] = &[
        "command",
        "controls",
        "covers",
        "facts_reread",
        "foundation",
        "host",
        "measured",
        "recorded",
        "status",
        "workspace",
    ];
    for m in obj {
        if !ALLOWED.contains(&m.name.as_str()) {
            return Err(AdmissionError::new(
                "parse-error",
                format!("unknown input field `{}`", m.name),
            ));
        }
    }

    let covers = decode_covers(
        root.get("covers")
            .ok_or_else(|| AdmissionError::new("parse-error", "missing `covers`"))?,
    )?;
    let command = req_str_vec(root, "command")?;
    let measured = decode_measured_vec(
        root.get("measured")
            .ok_or_else(|| AdmissionError::new("parse-error", "missing `measured`"))?,
    )?;
    let controls = decode_controls_vec(
        root.get("controls")
            .ok_or_else(|| AdmissionError::new("parse-error", "missing `controls`"))?,
    )?;
    let facts_reread = decode_facts_vec(
        root.get("facts_reread")
            .ok_or_else(|| AdmissionError::new("parse-error", "missing `facts_reread`"))?,
    )?;
    let foundation = opt_nullable_str(root, "foundation")?;
    let workspace = opt_nullable_str(root, "workspace")?;
    let status = match opt_nullable_str(root, "status")? {
        Some(s) => Some(AdmissionStatus::parse(&s)?),
        None => None,
    };
    let host = match root.get("host") {
        None => None,
        Some(v) if v.is_null() => None,
        Some(v) => Some(decode_host(v)?),
    };
    let recorded = match root.get("recorded") {
        None => None,
        Some(v) if v.is_null() => None,
        Some(v) => Some(decode_recorded(v)?),
    };
    Ok(RecordInput {
        covers,
        command,
        measured,
        controls,
        facts_reread,
        foundation,
        workspace,
        status,
        host,
        recorded,
    })
}

/// CLI entrypoint behind `examples/admission.rs`.
///
/// Usage:
/// `admission record --gate <id> --input <path> [--harness-receipt <hash>] [--root <dir>] [--date <ISO>] [--session <id>] [--implemented-by <id>] [--host-class <trj|m5-ultra>] [--timeout <secs>]`
///
/// Prints canonical JSON `{"path":"...","record":"<hash>","status":"..."}` on
/// `stdout` and returns `Ok(())`. On refusal/usage error, writes the finding on
/// `stderr` and returns `Err(exit_code)`.
pub fn run_cli(args: &[String], stdout: &mut dyn Write, stderr: &mut dyn Write) -> Result<(), u8> {
    if args.is_empty() || matches!(args[0].as_str(), "-h" | "--help") {
        let _ = writeln!(
            stderr,
            "Usage: admission record --gate <id> --input <path> [--harness-receipt <64-hex>] [--root <dir>]"
        );
        return if args.is_empty() { Err(3) } else { Ok(()) };
    }
    if args[0] != "record" {
        let _ = writeln!(stderr, "unknown subcommand `{}` (expected `record`)", args[0]);
        return Err(3);
    }

    let mut gate: Option<String> = None;
    let mut input_path: Option<PathBuf> = None;
    let mut harness_receipt: Option<String> = None;
    let mut root_dir: Option<PathBuf> = None;
    let mut date_ov: Option<String> = None;
    let mut session_ov: Option<String> = None;
    let mut impl_ov: Option<String> = None;
    let mut host_ov: Option<String> = None;
    let mut timeout_secs = DEFAULT_TIMEOUT_SECS;

    let mut i = 1usize;
    while i < args.len() {
        let arg = args[i].as_str();
        let next_val = |idx: &mut usize, flag: &str| -> Result<String, String> {
            *idx += 1;
            args.get(*idx)
                .cloned()
                .ok_or_else(|| format!("`{flag}` requires a value"))
        };
        let parsed = match arg {
            "--gate" => next_val(&mut i, "--gate").map(|v| gate = Some(v)),
            "--input" => next_val(&mut i, "--input").map(|v| input_path = Some(PathBuf::from(v))),
            "--harness-receipt" => {
                next_val(&mut i, "--harness-receipt").map(|v| harness_receipt = Some(v))
            }
            "--root" => next_val(&mut i, "--root").map(|v| root_dir = Some(PathBuf::from(v))),
            "--date" => next_val(&mut i, "--date").map(|v| date_ov = Some(v)),
            "--session" => next_val(&mut i, "--session").map(|v| session_ov = Some(v)),
            "--implemented-by" => next_val(&mut i, "--implemented-by").map(|v| impl_ov = Some(v)),
            "--host-class" => next_val(&mut i, "--host-class").map(|v| host_ov = Some(v)),
            "--timeout" => next_val(&mut i, "--timeout").and_then(|raw| {
                timeout_secs = raw
                    .parse::<u64>()
                    .map_err(|_| format!("invalid `--timeout` value `{raw}`"))?;
                Ok(())
            }),
            other => Err(format!("unknown flag `{other}`")),
        };
        if let Err(msg) = parsed {
            let _ = writeln!(stderr, "{msg}");
            return Err(3);
        }
        i += 1;
    }

    let Some(gate) = gate else {
        let _ = writeln!(stderr, "missing required `--gate <id>`");
        return Err(3);
    };
    let Some(input_path) = input_path else {
        let _ = writeln!(stderr, "missing required `--input <path>`");
        return Err(3);
    };
    let repo_root = match root_dir {
        Some(r) => r,
        None => std::env::current_dir().map_err(|e| {
            let _ = writeln!(stderr, "failed to get cwd: {e}");
            2u8
        })?,
    };

    let input_bytes = fs::read(&input_path).map_err(|e| {
        let _ = writeln!(stderr, "failed to read input `{}`: {e}", input_path.display());
        2u8
    })?;
    let input = parse_input_json(&input_bytes).map_err(|e| {
        let _ = writeln!(stderr, "{e}");
        2u8
    })?;

    let opts = RecordOptions {
        repo_root,
        gate,
        harness_receipt,
        timeout_secs,
        date_override: date_ov,
        session_override: session_ov,
        implemented_by_override: impl_ov,
        host_class_override: host_ov,
    };

    let outcome = record(&opts, &input).map_err(|e| {
        let _ = writeln!(stderr, "{e}");
        2u8
    })?;
    let json = canon::to_canonical(&outcome).map_err(|e| {
        let _ = writeln!(stderr, "canonical serialize error: {e}");
        2u8
    })?;
    let _ = writeln!(stdout, "{}", json.as_str());
    Ok(())
}

// ============================================================================
// Internal stack-receipt reader, command runner, atomic writer, JSON decoders
// ============================================================================

struct SiblingSnap {
    fd: String,
    rev: String,
    dirty: bool,
}

struct StackReceiptSnap {
    repo_dirty: bool,
    toolchain: String,
    siblings: Vec<SiblingSnap>,
    duplicates: Vec<String>,
}

fn load_head_stack_receipt(repo_root: &Path) -> Result<(String, StackReceiptSnap), AdmissionError> {
    let receipts_dir = repo_root.join("receipts");
    let head = fs::read_to_string(receipts_dir.join("HEAD"))
        .map_err(|e| AdmissionError::new("receipt-missing", format!("cannot read receipts/HEAD: {e}")))?
        .trim()
        .to_owned();
    if !is_lower_hex(&head, 64) {
        return Err(AdmissionError::new(
            "receipt-missing",
            format!("receipts/HEAD is not a 64-hex hash (`{head}`)"),
        ));
    }
    let path = receipts_dir.join(format!("{head}.json"));
    let bytes = fs::read(&path).map_err(|e| {
        AdmissionError::new(
            "receipt-missing",
            format!("cannot read stack receipt `receipts/{head}.json`: {e}"),
        )
    })?;
    let actual = hash_blob(&bytes).to_hex();
    if actual != head {
        return Err(AdmissionError::new(
            "receipt-missing",
            format!("stack receipt `receipts/{head}.json` tampered (actual BLAKE3 `{actual}`)"),
        ));
    }
    let admitted = json::admit(&bytes, &Limits::default())
        .map_err(|e| AdmissionError::new("receipt-missing", format!("invalid receipt JSON: {e}")))?;
    let v = admitted.value();
    let repo_dirty = v
        .get("repo_dirty")
        .and_then(|x| x.as_bool())
        .ok_or_else(|| AdmissionError::new("receipt-missing", "receipt missing bool `repo_dirty`"))?;
    let toolchain = req_str(v, "toolchain")?;
    let duplicates = req_str_vec(v, "duplicates")?;
    let sib_arr = v
        .get("siblings")
        .and_then(|x| x.as_array())
        .ok_or_else(|| AdmissionError::new("receipt-missing", "receipt missing array `siblings`"))?;
    let mut siblings = Vec::with_capacity(sib_arr.len());
    for item in sib_arr {
        let fd = req_str(item, "fd")?;
        let rev = req_str(item, "rev")?;
        let dirty = item
            .get("dirty")
            .and_then(|x| x.as_bool())
            .ok_or_else(|| AdmissionError::new("receipt-missing", "sibling missing bool `dirty`"))?;
        siblings.push(SiblingSnap { fd, rev, dirty });
    }
    Ok((
        head,
        StackReceiptSnap {
            repo_dirty,
            toolchain,
            siblings,
            duplicates,
        },
    ))
}

fn resolve_foundation(gate: &str, explicit: Option<&str>) -> Result<String, AdmissionError> {
    if let Some(fd) = explicit {
        if !is_valid_foundation_id(fd) {
            return Err(AdmissionError::new(
                "invalid-foundation",
                format!("invalid foundation identifier `{fd}`"),
            ));
        }
        return Ok(fd.to_owned());
    }
    if let Some(spec) = lookup_gate(gate) {
        return Ok(spec.foundation.to_owned());
    }
    Err(AdmissionError::new(
        "missing-foundation",
        format!(
            "rowless gate `{gate}` requires an explicit `foundation` member in `--input` (e.g. `FD-FMCP`)"
        ),
    ))
}

fn is_gate_own_scope_spec(
    spec: &GateDefault,
    foundation: &str,
    covers: &AdmissionCovers,
) -> bool {
    if spec.foundation != "WORKSPACE" && foundation != spec.foundation {
        return false;
    }
    if spec.own_crates.is_empty() && spec.own_groups.is_empty() && spec.own_tool.is_none() {
        return true;
    }
    let exp_crates: BTreeSet<String> = spec.own_crates.iter().map(|s| (*s).to_owned()).collect();
    let exp_groups: BTreeSet<String> = spec.own_groups.iter().map(|s| (*s).to_owned()).collect();
    let exp_tool = spec.own_tool.map(ToOwned::to_owned);
    covers.normalized_key() == (exp_crates, exp_groups, exp_tool)
}

fn run_command_with_timeout(
    repo_root: &Path,
    argv: &[String],
    timeout: Duration,
) -> Result<(i64, Vec<u8>), AdmissionError> {
    let (prog, rest) = argv
        .split_first()
        .ok_or_else(|| AdmissionError::new("invalid-command", "empty argv"))?;
    // Never invoke cargo by absolute path; resolve `prog` via PATH so cargo flows through rch.
    let mut child = Command::new(prog)
        .args(rest)
        .current_dir(repo_root)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| AdmissionError::new("spawn-failed", format!("spawn `{prog}`: {e}")))?;

    let mut out_pipe = child.stdout.take();
    let mut err_pipe = child.stderr.take();
    let t_out = thread::spawn(move || {
        let mut buf = Vec::new();
        if let Some(ref mut r) = out_pipe {
            let _ = std::io::Read::read_to_end(r, &mut buf);
        }
        buf
    });
    let t_err = thread::spawn(move || {
        let mut buf = Vec::new();
        if let Some(ref mut r) = err_pipe {
            let _ = std::io::Read::read_to_end(r, &mut buf);
        }
        buf
    });

    let start = Instant::now();
    let exit_code = loop {
        match child.try_wait() {
            Ok(Some(status)) => break i64::from(status.code().unwrap_or(1)),
            Ok(None) => {
                if start.elapsed() >= timeout {
                    let _ = child.kill();
                    let _ = child.wait();
                    break 124;
                }
                thread::sleep(Duration::from_millis(20));
            }
            Err(e) => {
                return Err(AdmissionError::new("wait-failed", format!("wait `{prog}`: {e}")));
            }
        }
    };

    let mut combined = t_out.join().unwrap_or_default();
    let stderr_bytes = t_err.join().unwrap_or_default();
    if !stderr_bytes.is_empty() {
        if !combined.is_empty() && !combined.ends_with(b"\n") {
            combined.push(b'\n');
        }
        combined.extend_from_slice(&stderr_bytes);
    }
    Ok((exit_code, combined))
}

fn utf8_tail_excerpt(raw: &[u8], max_bytes: usize) -> String {
    let text = String::from_utf8_lossy(raw);
    if text.len() <= max_bytes {
        return text.into_owned();
    }
    let mut start = text.len() - max_bytes;
    while start < text.len() && !text.is_char_boundary(start) {
        start += 1;
    }
    text[start..].to_owned()
}

fn atomic_write_new(dest: &Path, bytes: &[u8]) -> Result<(), AdmissionError> {
    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| AdmissionError::new("io-error", format!("create_dir_all: {e}")))?;
    }
    static SEQ: AtomicU64 = AtomicU64::new(0);
    let seq = SEQ.fetch_add(1, Ordering::Relaxed);
    let name = dest.file_name().and_then(|s| s.to_str()).unwrap_or("rec");
    let tmp = dest.with_file_name(format!(".{name}-{}-{seq}.tmp", std::process::id()));
    let mut f = File::create(&tmp)
        .map_err(|e| AdmissionError::new("io-error", format!("create temp: {e}")))?;
    f.write_all(bytes)
        .and_then(|()| f.sync_all())
        .map_err(|e| AdmissionError::new("io-error", format!("write/sync temp: {e}")))?;
    drop(f);
    fs::rename(&tmp, dest)
        .map_err(|e| AdmissionError::new("io-error", format!("rename temp: {e}")))?;
    Ok(())
}

fn git_head_ro(repo_root: &Path) -> Result<String, AdmissionError> {
    let out = Command::new("git")
        .arg("--no-optional-locks")
        .arg("-C")
        .arg(repo_root)
        .args(["rev-parse", "HEAD"])
        .output()
        .map_err(|e| AdmissionError::new("git-error", format!("spawn git rev-parse: {e}")))?;
    if !out.status.success() {
        return Err(AdmissionError::new(
            "git-error",
            format!(
                "git rev-parse HEAD failed: {}",
                String::from_utf8_lossy(&out.stderr).trim()
            ),
        ));
    }
    let rev = String::from_utf8_lossy(&out.stdout).trim().to_owned();
    if !is_lower_hex(&rev, 40) {
        return Err(AdmissionError::new(
            "git-error",
            format!("HEAD is not 40-hex commit: `{rev}`"),
        ));
    }
    Ok(rev)
}

fn default_host_class() -> String {
    if cfg!(target_os = "macos") {
        "m5-ultra".to_owned()
    } else {
        "trj".to_owned()
    }
}

fn is_valid_gate_id(gate: &str) -> bool {
    if let Some(rest) = gate.strip_prefix("INT-") {
        let parts: Vec<&str> = rest.split('-').collect();
        return parts.len() >= 2
            && parts[..parts.len() - 1]
                .iter()
                .all(|p| !p.is_empty() && p.bytes().all(|b| b.is_ascii_uppercase() || b.is_ascii_digit()))
            && !parts[parts.len() - 1].is_empty()
            && parts[parts.len() - 1].bytes().all(|b| b.is_ascii_digit());
    }
    if let Some(rest) = gate.strip_prefix("ADMIT-") {
        return !rest.is_empty()
            && rest.split('-').all(|p| {
                !p.is_empty() && p.bytes().all(|b| b.is_ascii_uppercase() || b.is_ascii_digit())
            });
    }
    false
}

fn is_valid_foundation_id(fd: &str) -> bool {
    if fd == "WORKSPACE" {
        return true;
    }
    fd.strip_prefix("FD-").is_some_and(|rest| {
        !rest.is_empty() && rest.bytes().all(|b| b.is_ascii_uppercase() || b.is_ascii_digit())
    })
}

fn is_lower_hex(s: &str, len: usize) -> bool {
    s.len() == len && s.bytes().all(|b| b.is_ascii_digit() || matches!(b, b'a'..=b'f'))
}

// ============================================================================
// JSON decoding helpers (`frob_core::json`)
// ============================================================================

fn decode_record(root: &Json<'_>) -> Result<AdmissionRecord, AdmissionError> {
    let members = root
        .as_object()
        .ok_or_else(|| AdmissionError::new("parse-error", "record must be JSON object"))?;
    const ALLOWED_KEYS: &[&str] = &[
        "command", "controls", "covers", "exit", "facts_reread", "foundation", "gate",
        "harness_receipt", "host", "measured", "output_excerpt", "output_hash", "recorded",
        "repo_rev", "schema", "sibling_rev", "stack_receipt", "status", "toolchain", "workspace",
    ];
    for m in members {
        if !ALLOWED_KEYS.contains(&m.name.as_str()) {
            return Err(AdmissionError::new(
                "parse-error",
                format!("unknown record field `{}`", m.name),
            ));
        }
    }

    let schema = req_str(root, "schema")?;
    if schema != ADMISSION_SCHEMA {
        return Err(AdmissionError::new("parse-error", format!("unexpected schema `{schema}`")));
    }
    let gate = req_str(root, "gate")?;
    let foundation = req_str(root, "foundation")?;
    let covers = decode_covers(
        root.get("covers")
            .ok_or_else(|| AdmissionError::new("parse-error", "missing `covers`"))?,
    )?;
    let repo_rev = req_str(root, "repo_rev")?;
    let workspace = req_str(root, "workspace")?;
    let stack_receipt = req_str(root, "stack_receipt")?;
    let sibling_rev = opt_nullable_str(root, "sibling_rev")?;
    let toolchain = req_str(root, "toolchain")?;

    // `harness_receipt` key is REQUIRED on every record (`null` or 64-hex string).
    let harness_receipt = match root.get("harness_receipt") {
        None => {
            return Err(AdmissionError::new(
                "parse-error",
                "missing required `harness_receipt` key",
            ));
        }
        Some(v) if v.is_null() => None,
        Some(v) => {
            let s = v.as_str().ok_or_else(|| {
                AdmissionError::new("parse-error", "`harness_receipt` must be null or 64-hex")
            })?;
            if !is_lower_hex(s, 64) {
                return Err(AdmissionError::new(
                    "parse-error",
                    format!("`harness_receipt` must be 64 lower-case hex digits, got `{s}`"),
                ));
            }
            Some(s.to_owned())
        }
    };

    let command = req_str_vec(root, "command")?;
    let exit = root
        .get("exit")
        .ok_or_else(|| AdmissionError::new("parse-error", "missing `exit`"))?
        .as_int53()
        .map_err(|e| AdmissionError::new("parse-error", format!("invalid `exit`: {e}")))?;
    let output_hash = req_str(root, "output_hash")?;
    let output_excerpt = req_str(root, "output_excerpt")?;
    let measured = decode_measured_vec(
        root.get("measured")
            .ok_or_else(|| AdmissionError::new("parse-error", "missing `measured`"))?,
    )?;
    let controls = decode_controls_vec(
        root.get("controls")
            .ok_or_else(|| AdmissionError::new("parse-error", "missing `controls`"))?,
    )?;
    let facts_reread = decode_facts_vec(
        root.get("facts_reread")
            .ok_or_else(|| AdmissionError::new("parse-error", "missing `facts_reread`"))?,
    )?;
    let status = AdmissionStatus::parse(&req_str(root, "status")?)?;
    let host = decode_host(
        root.get("host")
            .ok_or_else(|| AdmissionError::new("parse-error", "missing `host`"))?,
    )?;
    let recorded = decode_recorded(
        root.get("recorded")
            .ok_or_else(|| AdmissionError::new("parse-error", "missing `recorded`"))?,
    )?;

    // Confirm TOML reader helper is wired when reading lockfiles/registries if needed.
    let _ = toml::parse as fn(&[u8]) -> Result<toml::Table, toml::TomlError>;
    let _ = TomlValue::as_str;

    Ok(AdmissionRecord {
        schema,
        gate,
        foundation,
        covers,
        repo_rev,
        workspace,
        stack_receipt,
        sibling_rev,
        toolchain,
        harness_receipt,
        command,
        exit,
        output_hash,
        output_excerpt,
        measured,
        controls,
        facts_reread,
        status,
        host,
        recorded,
    })
}

fn decode_covers(node: &Json<'_>) -> Result<AdmissionCovers, AdmissionError> {
    let crates = req_str_vec(node, "crates")?;
    let groups = req_str_vec(node, "groups")?;
    let tool = match node
        .get("tool")
        .ok_or_else(|| AdmissionError::new("parse-error", "missing `covers.tool`"))?
    {
        v if v.is_null() => None,
        v if v.as_str().is_some() => Some(ToolSpec {
            name: v.as_str().unwrap_or_default().to_owned(),
            rev: None,
            binary_hash: None,
        }),
        v if v.as_object().is_some() => Some(ToolSpec {
            name: req_str(v, "name")?,
            rev: opt_nullable_str(v, "rev")?,
            binary_hash: opt_nullable_str(v, "binary_hash")?,
        }),
        _ => {
            return Err(AdmissionError::new(
                "parse-error",
                "`covers.tool` must be null, string, or object",
            ));
        }
    };
    Ok(AdmissionCovers { crates, groups, tool })
}

fn decode_measured_vec(node: &Json<'_>) -> Result<Vec<MeasuredEntry>, AdmissionError> {
    let arr = node
        .as_array()
        .ok_or_else(|| AdmissionError::new("parse-error", "`measured` must be array"))?;
    let mut out = Vec::with_capacity(arr.len());
    for item in arr {
        let name = req_str(item, "name")?;
        let value = match item.get("value") {
            None => None,
            Some(v) if v.is_null() => None,
            Some(v) => Some(decode_measure_num(v)?),
        };
        let unit = item
            .get("unit")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_owned();
        let threshold = decode_measure_num(
            item.get("threshold")
                .ok_or_else(|| AdmissionError::new("parse-error", "missing `measured.threshold`"))?,
        )?;
        let relation = MeasureRelation::parse(&req_str(item, "relation")?)?;
        out.push(MeasuredEntry {
            name,
            value,
            unit,
            threshold,
            relation,
        });
    }
    Ok(out)
}

fn decode_measure_num(node: &Json<'_>) -> Result<MeasureNum, AdmissionError> {
    if let Ok(n) = node.as_int53() {
        return Ok(MeasureNum::Int(n));
    }
    let (_lex, x) = node
        .as_f64_lexeme()
        .map_err(|e| AdmissionError::new("parse-error", format!("invalid number: {e}")))?;
    if !x.is_finite() {
        return Err(AdmissionError::new("parse-error", "non-finite number"));
    }
    Ok(MeasureNum::Float(x))
}

fn decode_controls_vec(node: &Json<'_>) -> Result<Vec<ControlResult>, AdmissionError> {
    let arr = node
        .as_array()
        .ok_or_else(|| AdmissionError::new("parse-error", "`controls` must be array"))?;
    let mut out = Vec::with_capacity(arr.len());
    for item in arr {
        let id = req_str(item, "id")?;
        let caught = item
            .get("caught")
            .and_then(|v| v.as_bool())
            .ok_or_else(|| AdmissionError::new("parse-error", "missing bool `controls.caught`"))?;
        out.push(ControlResult { id, caught });
    }
    Ok(out)
}

fn decode_facts_vec(node: &Json<'_>) -> Result<Vec<FactReread>, AdmissionError> {
    let arr = node
        .as_array()
        .ok_or_else(|| AdmissionError::new("parse-error", "`facts_reread` must be array"))?;
    let mut out = Vec::with_capacity(arr.len());
    for item in arr {
        out.push(FactReread {
            fact: req_str(item, "fact")?,
            path: req_str(item, "path")?,
            value: req_str(item, "value")?,
        });
    }
    Ok(out)
}

fn decode_host(node: &Json<'_>) -> Result<HostInfo, AdmissionError> {
    let class = req_str(node, "class")?;
    let fingerprint = opt_nullable_str(node, "fingerprint")?;
    Ok(HostInfo { class, fingerprint })
}

fn decode_recorded(node: &Json<'_>) -> Result<RecordedBy, AdmissionError> {
    Ok(RecordedBy {
        date: req_str(node, "date")?,
        session: req_str(node, "session")?,
        implemented_by: req_str(node, "implemented_by")?,
    })
}

fn req_str(obj: &Json<'_>, key: &str) -> Result<String, AdmissionError> {
    obj.get(key)
        .and_then(|v| v.as_str())
        .map(ToOwned::to_owned)
        .ok_or_else(|| AdmissionError::new("parse-error", format!("missing string `{key}`")))
}

fn opt_nullable_str(obj: &Json<'_>, key: &str) -> Result<Option<String>, AdmissionError> {
    match obj.get(key) {
        None => Ok(None),
        Some(v) if v.is_null() => Ok(None),
        Some(v) => v
            .as_str()
            .map(|s| Some(s.to_owned()))
            .ok_or_else(|| AdmissionError::new("parse-error", format!("`{key}` must be string or null"))),
    }
}

fn req_str_vec(obj: &Json<'_>, key: &str) -> Result<Vec<String>, AdmissionError> {
    let arr = obj
        .get(key)
        .and_then(|v| v.as_array())
        .ok_or_else(|| AdmissionError::new("parse-error", format!("missing array `{key}`")))?;
    arr.iter()
        .map(|it| {
            it.as_str()
                .map(ToOwned::to_owned)
                .ok_or_else(|| AdmissionError::new("parse-error", format!("`{key}` item must be string")))
        })
        .collect()
}

// ============================================================================
// Unit tests (`cargo test -p frob-conformance admission`; synthetic repos)
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn fresh_dir(tag: &str) -> PathBuf {
        static CTR: AtomicU64 = AtomicU64::new(0);
        let dir = std::env::temp_dir().join(format!(
            "frob_conf_admission_{tag}_{}_{}",
            std::process::id(),
            CTR.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(&dir).expect("mkdir conf admission fixture");
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
            "git {:?} failed: {}",
            args,
            String::from_utf8_lossy(&out.stderr)
        );
        String::from_utf8_lossy(&out.stdout).trim().to_owned()
    }

    fn build_synthetic_repo(tag: &str, sibling_dirty: bool) -> (PathBuf, String) {
        let repo = fresh_dir(tag);
        git_fixture(&repo, &["init", "-q", "-b", "main"]);
        fs::create_dir_all(repo.join("registries")).expect("registries");
        fs::write(
            repo.join("registries/gates.toml"),
            "schema = \"frob.gates/1\"\n# sentinel: must remain untouched\n",
        )
        .expect("gates.toml");
        fs::write(
            repo.join("Cargo.lock"),
            "# synthetic\nversion = 4\n\n\
[[package]]\nname = \"fs-contact\"\nversion = \"0.1.0\"\n\n\
[[package]]\nname = \"fs-ga\"\nversion = \"0.1.0\"\n\n\
[[package]]\nname = \"fs-mbd\"\nversion = \"0.1.0\"\n\n\
[[package]]\nname = \"fs-query\"\nversion = \"0.1.0\"\n\n\
[[package]]\nname = \"fs-time\"\nversion = \"0.1.0\"\n",
        )
        .expect("Cargo.lock");
        git_fixture(&repo, &["add", "."]);
        git_fixture(&repo, &["commit", "-q", "-m", "init"]);
        let repo_rev = git_fixture(&repo, &["rev-parse", "HEAD"]);

        let sib_rev = "1".repeat(40);
        let receipt_json = format!(
            "{{\"build_env\":{{}},\"commands\":[],\"duplicates\":[],\"lockfiles\":[],\"patches\":[],\
\"profile_overrides\":[],\"repo_dirty\":false,\"repo_rev\":\"{repo_rev}\",\"repo_tree_hash\":null,\
\"schema\":\"frob.stack-receipt/1\",\"siblings\":[{{\"dirty\":{sibling_dirty},\"fd\":\"FD-FSIM\",\
\"features\":[\"referee\"],\"own_lock\":{{\"agrees\":true,\"class\":\"on-pin\",\"file\":null,\
\"hash\":null,\"reason\":null}},\"rev\":\"{sib_rev}\",\"tree_hash\":null}}],\"toolchain\":\"nightly-2026-08-31\"}}"
        );
        let sr_hash = hash_blob(receipt_json.as_bytes()).to_hex();
        fs::create_dir_all(repo.join("receipts")).expect("receipts");
        fs::write(
            repo.join("receipts").join(format!("{sr_hash}.json")),
            receipt_json.as_bytes(),
        )
        .expect("write stack receipt");
        fs::write(repo.join("receipts/HEAD"), format!("{sr_hash}\n")).expect("write HEAD");
        (repo, sr_hash)
    }

    fn sample_input() -> RecordInput {
        RecordInput {
            covers: AdmissionCovers {
                crates: vec![
                    "fs-contact".to_owned(),
                    "fs-ga".to_owned(),
                    "fs-mbd".to_owned(),
                    "fs-query".to_owned(),
                    "fs-time".to_owned(),
                ],
                groups: Vec::new(),
                tool: None,
            },
            command: vec!["true".to_owned()],
            measured: vec![MeasuredEntry {
                name: "cold_check_s".to_owned(),
                value: Some(MeasureNum::Int(12)),
                unit: "s".to_owned(),
                threshold: MeasureNum::Int(3600),
                relation: MeasureRelation::Le,
            }],
            controls: vec![ControlResult {
                id: "NC-FSIM-01".to_owned(),
                caught: true,
            }],
            facts_reread: vec![FactReread {
                fact: "fs-contact MAX_CONTACTS".to_owned(),
                path: "crates/fs-contact/src/sustained/solve.rs:26".to_owned(),
                value: "21".to_owned(),
            }],
            foundation: None,
            workspace: Some("core".to_owned()),
            status: None,
            host: Some(HostInfo {
                class: "m5-ultra".to_owned(),
                fingerprint: None,
            }),
            recorded: Some(RecordedBy {
                date: "2026-11-24T22:00:00Z".to_owned(),
                session: "session:reviewer".to_owned(),
                implemented_by: "session:author".to_owned(),
            }),
        }
    }

    fn sample_input_json() -> &'static str {
        r#"{
  "covers": {
    "crates": ["fs-contact", "fs-ga", "fs-mbd", "fs-query", "fs-time"],
    "groups": [],
    "tool": null
  },
  "command": ["true"],
  "measured": [
    {"name": "cold_check_s", "value": 12, "unit": "s", "threshold": 3600, "relation": "<="}
  ],
  "controls": [
    {"id": "NC-FSIM-01", "caught": true}
  ],
  "facts_reread": [
    {"fact": "fs-contact MAX_CONTACTS", "path": "crates/fs-contact/src/sustained/solve.rs:26", "value": "21"}
  ],
  "workspace": "core",
  "host": {"class": "m5-ultra", "fingerprint": null},
  "recorded": {"date": "2026-11-24T22:00:00Z", "session": "session:reviewer", "implemented_by": "session:author"}
}"#
    }

    #[test]
    fn record_roundtrip_canonical() {
        let (repo, _) = build_synthetic_repo("roundtrip", false);
        let opts = RecordOptions::new(&repo, "INT-FSIM-01");
        let out = record(&opts, &sample_input()).expect("record pass");

        let stored_bytes = fs::read(&out.abs_path).expect("read stored");
        assert_eq!(hash_blob(&stored_bytes).to_hex(), out.record_hash);
        assert!(out.rel_path.ends_with(&format!("{}.json", out.record_hash)));

        let parsed = AdmissionRecord::from_json_bytes(&stored_bytes).expect("parse stored");
        assert_eq!(parsed, out.record);
        let reser = parsed.to_canonical_bytes().expect("reserialize");
        assert_eq!(stored_bytes.as_slice(), reser.as_bytes());

        // Refuses overwrite on second call with identical input.
        let dup_err = record(&opts, &sample_input()).expect_err("must refuse overwrite");
        assert_eq!(dup_err.kind, "record-exists");
    }

    #[test]
    fn writer_leaves_gates_toml_alone() {
        let (repo, _) = build_synthetic_repo("gates_untouched", false);
        let gates_path = repo.join("registries/gates.toml");
        let before = fs::read(&gates_path).expect("read gates before");

        let harness = "f".repeat(64);
        let mut opts = RecordOptions::new(&repo, "INT-FSIM-01");
        opts.harness_receipt = Some(harness.clone());
        let out = record(&opts, &sample_input()).expect("record pass");

        let after = fs::read(&gates_path).expect("read gates after");
        assert_eq!(before, after, "writer must never touch registries/gates.toml");
        assert_eq!(out.record.harness_receipt.as_deref(), Some(harness.as_str()));
    }

    #[test]
    fn example_record_matches_library() {
        let (repo_lib, _) = build_synthetic_repo("ex_lib", false);
        let (repo_cli, _) = build_synthetic_repo("ex_cli", false);

        // Library call (omitted harness_receipt -> null).
        let opts = RecordOptions::new(&repo_lib, "INT-FSIM-01");
        let lib_out = record(&opts, &sample_input()).expect("lib record");
        let lib_bytes = fs::read(&lib_out.abs_path).expect("lib bytes");
        assert_eq!(lib_out.record.harness_receipt, None);

        // Example CLI call on second copy with identical input.
        let in_path = repo_cli.join("input.json");
        fs::write(&in_path, sample_input_json()).expect("write input.json");
        let args = vec![
            "record".to_owned(),
            "--gate".to_owned(),
            "INT-FSIM-01".to_owned(),
            "--input".to_owned(),
            in_path.to_string_lossy().into_owned(),
            "--root".to_owned(),
            repo_cli.to_string_lossy().into_owned(),
        ];
        let mut stdout = Vec::new();
        let mut stderr = Vec::new();
        run_cli(&args, &mut stdout, &mut stderr).expect("cli record must succeed");
        let stdout_s = String::from_utf8(stdout).expect("utf8");
        assert!(stdout_s.contains(&lib_out.record_hash), "stdout must print record hash: {stdout_s}");
        let cli_bytes = fs::read(repo_cli.join(&lib_out.rel_path)).expect("cli bytes");
        assert_eq!(lib_bytes, cli_bytes, "example and library must write identical canonical bytes");

        // Dirty receipt -> non-zero exit naming `pass-on-dirty` and writes NOTHING.
        let (repo_dirty, _) = build_synthetic_repo("ex_dirty", true);
        let dirty_in = repo_dirty.join("input.json");
        fs::write(&dirty_in, sample_input_json()).expect("write dirty input");
        let dirty_args = vec![
            "record".to_owned(),
            "--gate".to_owned(),
            "INT-FSIM-01".to_owned(),
            "--input".to_owned(),
            dirty_in.to_string_lossy().into_owned(),
            "--root".to_owned(),
            repo_dirty.to_string_lossy().into_owned(),
        ];
        let mut d_out = Vec::new();
        let mut d_err = Vec::new();
        let res = run_cli(&dirty_args, &mut d_out, &mut d_err);
        assert!(res.is_err(), "dirty receipt pass must exit non-zero");
        let d_err_s = String::from_utf8(d_err).expect("utf8");
        assert!(
            d_err_s.contains("pass-on-dirty"),
            "stderr must name pass-on-dirty, got: {d_err_s}"
        );
        assert!(
            !repo_dirty.join("receipts/admissions").exists(),
            "refused write must create no admission record"
        );
    }

    #[test]
    fn rule4_pass_on_dirty_refused() {
        let (repo, _) = build_synthetic_repo("r4", true);
        let err = record(&RecordOptions::new(&repo, "INT-FSIM-01"), &sample_input())
            .expect_err("dirty receipt must be refused");
        assert_eq!(err.kind, "pass-on-dirty");
    }

    #[test]
    fn rule5_pass_without_control_refused() {
        let (repo, _) = build_synthetic_repo("r5", false);
        let mut inp = sample_input();
        inp.controls.clear();
        let err = record(&RecordOptions::new(&repo, "INT-FSIM-01"), &inp)
            .expect_err("missing control must be refused");
        assert_eq!(err.kind, "pass-without-control");
    }

    #[test]
    fn rule6_self_certified_refused() {
        let (repo, _) = build_synthetic_repo("r6", false);
        let mut inp = sample_input();
        inp.recorded = Some(RecordedBy {
            date: "2026-11-24T22:00:00Z".to_owned(),
            session: "same-agent".to_owned(),
            implemented_by: "same-agent".to_owned(),
        });
        let err = record(&RecordOptions::new(&repo, "INT-FSIM-01"), &inp)
            .expect_err("self-certification must be refused");
        assert_eq!(err.kind, "self-certified");
    }

    #[test]
    fn rule7_tool_unpinned_refused() {
        let (repo, _) = build_synthetic_repo("r7", false);
        let mut inp = sample_input();
        inp.covers = AdmissionCovers {
            crates: Vec::new(),
            groups: Vec::new(),
            tool: Some(ToolSpec {
                name: "fln".to_owned(),
                rev: None,
                binary_hash: None,
            }),
        };
        let err = record(&RecordOptions::new(&repo, "INT-FLN-01"), &inp)
            .expect_err("unpinned tool must be refused");
        assert_eq!(err.kind, "tool-unpinned");

        inp.covers.tool = Some(ToolSpec {
            name: "fln".to_owned(),
            rev: Some("a".repeat(40)),
            binary_hash: Some("b".repeat(64)),
        });
        let ok = record(&RecordOptions::new(&repo, "INT-FLN-01"), &inp).expect("pinned tool ok");
        assert_eq!(ok.status, AdmissionStatus::Pass);
    }

    #[test]
    fn rule8_target_as_measurement_refused() {
        let (repo, _) = build_synthetic_repo("r8", false);
        let mut inp = sample_input();
        inp.measured[0].value = None;
        let err = record(&RecordOptions::new(&repo, "INT-FSIM-01"), &inp)
            .expect_err("missing measured value must be refused");
        assert_eq!(err.kind, "measured-missing");

        // Failing gate run records `status = "fail"` honestly with its measured values.
        let mut fail_inp = sample_input();
        fail_inp.command = vec!["false".to_owned()];
        let fail_out = record(&RecordOptions::new(&repo, "INT-FSIM-01"), &fail_inp)
            .expect("failing run must still write fail record");
        assert_eq!(fail_out.status, AdmissionStatus::Fail);
        assert_eq!(fail_out.record.exit, 1);
    }

    #[test]
    fn harness_receipt_key_required() {
        let (repo, _) = build_synthetic_repo("hr_req", false);
        let out = record(&RecordOptions::new(&repo, "INT-FSIM-01"), &sample_input()).expect("ok");
        let raw = fs::read_to_string(&out.abs_path).expect("read");
        assert!(raw.contains("\"harness_receipt\":null"));
        let stripped = raw.replace("\"harness_receipt\":null,", "");
        assert!(AdmissionRecord::from_json_bytes(stripped.as_bytes()).is_err());
        let bad = raw.replace("\"harness_receipt\":null", "\"harness_receipt\":\"bad\"");
        assert!(AdmissionRecord::from_json_bytes(bad.as_bytes()).is_err());
    }
}
