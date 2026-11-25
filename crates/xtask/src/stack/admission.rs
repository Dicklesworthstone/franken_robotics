//! Foundation admission records (`frob.admission/1`): strict parser, canonical
//! serializer (`frob_core::canon`), 8-rule record validator, single-source
//! `never_inherited` check, `admitted_gates` query, and `status` summary
//! (bead `N33a-07`, §33.6, INV-SUITE-03, GATE-GAUNT-10).
//!
//! Review notes 1–5 (polishing round 4) resolved:
//! - Note 1 (F-U34): admission-document generator lives at
//!   `xtask::generate::admission_doc` for `N34p4-03` (`check-generated`).
//! - Notes 2 & 4 (S-23, S-24): first-use / partial-scope records filed under a
//!   register gate (e.g. `INT-FSIM-01` or `INT-FSIM-14`) whose `covers` differ
//!   from the gate's own scope are checked against their own declared
//!   thresholds, read by `never_inherited` via `covers`, listed by `status()`
//!   (for `frob doctor`), and never overwrite the gate's own newest record in
//!   `admitted_gates` or the main gate status table.
//! - Note 3 (S-23): `never_inherited` inspects direct resolved workspace-member
//!   dependencies and enabled `frob-fsim` airlock feature groups from
//!   `cargo metadata --format-version 1` (never `Cargo.lock` unenabled optional
//!   deps, and never transitive internal sibling closures).
//! - Note 5 (S-26): rowless third-party exception gates (`ADMIT-*`, e.g.
//!   `ADMIT-CAP-STD`) carry an explicit `foundation` (such as `FD-FMCP`) with
//!   no gate-row thresholds required.
//!
//! Determinism class D0, zero `unsafe`, prints nothing (rule L-7), read-only on
//! sibling checkouts (`git --no-optional-locks`, INV-SUITE-04).

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::Command;

use frob_core::canon::{self, CanonError, Canonical, CanonicalBytes, Value};
use frob_core::json::{self, Json, Limits};
use frob_core::registry::toml::{self, Value as TomlValue};

use crate::metadata::{self, DepKind, ResolvedWorkspace, WorkspaceRoot};
use crate::stack::guard;
use crate::stack::lock::{self, FSIM_AIRLOCK_FEATURE_GROUPS, Foundation, GateRef, GitRev, StackLock};
use crate::stack::posture::BEDROCK_DIRECT_CRATES;
use crate::stack::receipt::{self, DriftClass, DriftReason, StackReceipt, WORKSPACES};

/// Schema identifier for admission records (`N33a-07`).
pub const ADMISSION_SCHEMA: &str = "frob.admission/1";

/// Maximum length in bytes of `output_excerpt` (`4 KiB`).
pub const MAX_EXCERPT_BYTES: usize = 4096;

/// Check-stack finding identifier for never-inherited violations (`unadmitted-use`, `admission-stale`).
pub const RULE_ADMISSION: &str = "stack.admission";

/// Check-stack finding identifier for admission record validity failures (rules 1–8 + integrity).
pub const RULE_ADMISSION_RECORD: &str = "stack.admission-record";

/// Every finding kind emitted under `stack.admission` or `stack.admission-record`.
pub const ADMISSION_KIND_TO_RULE: &[(&str, &str)] = &[
    ("unadmitted-use", RULE_ADMISSION),
    ("admission-stale", RULE_ADMISSION),
    ("lockfile-lacks-foundation", RULE_ADMISSION_RECORD),
    ("commit-unreachable", RULE_ADMISSION_RECORD),
    ("receipt-missing", RULE_ADMISSION_RECORD),
    ("output-missing", RULE_ADMISSION_RECORD),
    ("pass-on-dirty", RULE_ADMISSION_RECORD),
    ("pass-without-control", RULE_ADMISSION_RECORD),
    ("self-certified", RULE_ADMISSION_RECORD),
    ("tool-unpinned", RULE_ADMISSION_RECORD),
    ("measured-missing", RULE_ADMISSION_RECORD),
    ("hash-mismatch", RULE_ADMISSION_RECORD),
    ("parse-error", RULE_ADMISSION_RECORD),
];

/// Authoritative specification of one register admission gate (`§33.6`, `N33a-07`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdmissionGateSpec {
    pub id: &'static str,
    pub foundation: &'static str,
    pub own_crates: &'static [&'static str],
    pub own_groups: &'static [&'static str],
    pub own_tool: Option<&'static str>,
    /// Required `(metric_name, unit)` thresholds on gate-own-scope records (rule 8).
    pub required_thresholds: &'static [(&'static str, &'static str)],
}

/// Complete closed catalog of the 37 admission gates of `§33.6` (`INT-FSIM-12` is reserved/unused).
pub const ADMISSION_GATES: &[AdmissionGateSpec] = &[
    AdmissionGateSpec { id: "INT-TOOLCHAIN-01", foundation: "WORKSPACE", own_crates: &[], own_groups: &[], own_tool: None, required_thresholds: &[] },
    AdmissionGateSpec {
        id: "INT-FSIM-01",
        foundation: "FD-FSIM",
        own_crates: &["fs-contact", "fs-ga", "fs-mbd", "fs-query", "fs-time"],
        own_groups: &[],
        own_tool: None,
        required_thresholds: &[("cold_check_s", "s")],
    },
    AdmissionGateSpec { id: "INT-FSIM-02", foundation: "FD-FSIM", own_crates: &["fs-ga"], own_groups: &[], own_tool: None, required_thresholds: &[] },
    AdmissionGateSpec { id: "INT-FSIM-03", foundation: "FD-FSIM", own_crates: &["fs-mbd"], own_groups: &[], own_tool: None, required_thresholds: &[] },
    AdmissionGateSpec { id: "INT-FSIM-04", foundation: "FD-FSIM", own_crates: &["fs-contact"], own_groups: &[], own_tool: None, required_thresholds: &[] },
    AdmissionGateSpec { id: "INT-FSIM-05", foundation: "FD-FSIM", own_crates: &["fs-mbd"], own_groups: &[], own_tool: None, required_thresholds: &[] },
    AdmissionGateSpec { id: "INT-FSIM-06", foundation: "FD-FSIM", own_crates: &["fs-g1-train"], own_groups: &[], own_tool: None, required_thresholds: &[] },
    AdmissionGateSpec { id: "INT-FSIM-07", foundation: "FD-FSIM", own_crates: &["fs-contact"], own_groups: &[], own_tool: None, required_thresholds: &[] },
    AdmissionGateSpec { id: "INT-FSIM-08", foundation: "FD-FSIM", own_crates: &["fs-time"], own_groups: &[], own_tool: None, required_thresholds: &[] },
    AdmissionGateSpec { id: "INT-FSIM-09", foundation: "FD-FSIM", own_crates: &["fs-matdb", "fs-phs", "fs-tribo"], own_groups: &[], own_tool: None, required_thresholds: &[] },
    AdmissionGateSpec { id: "INT-FSIM-10", foundation: "FD-FSIM", own_crates: &["fs-query"], own_groups: &[], own_tool: None, required_thresholds: &[] },
    AdmissionGateSpec { id: "INT-FSIM-11", foundation: "FD-FSIM", own_crates: &["fs-mbd"], own_groups: &[], own_tool: None, required_thresholds: &[] },
    // INT-FSIM-12 is reserved and unused (§33.6).
    AdmissionGateSpec { id: "INT-FSIM-13", foundation: "FD-FSIM", own_crates: &[], own_groups: &["spectral"], own_tool: None, required_thresholds: &[] },
    AdmissionGateSpec { id: "INT-FSIM-14", foundation: "FD-FSIM", own_crates: &[], own_groups: &["stats"], own_tool: None, required_thresholds: &[] },
    AdmissionGateSpec { id: "INT-FSIM-15", foundation: "FD-FSIM", own_crates: &[], own_groups: &["structure"], own_tool: None, required_thresholds: &[] },
    AdmissionGateSpec { id: "INT-FSIM-16", foundation: "FD-FSIM", own_crates: &["fs-recompute"], own_groups: &[], own_tool: None, required_thresholds: &[] },
    AdmissionGateSpec { id: "INT-FSIM-17", foundation: "FD-FSIM", own_crates: &[], own_groups: &["multiphysics"], own_tool: None, required_thresholds: &[] },
    AdmissionGateSpec { id: "INT-FSIM-18", foundation: "FD-FSIM", own_crates: &[], own_groups: &[], own_tool: Some("frob-fsim-worker"), required_thresholds: &[] },
    AdmissionGateSpec { id: "INT-ASUP-01", foundation: "FD-ASUP", own_crates: &["asupersync"], own_groups: &[], own_tool: None, required_thresholds: &[] },
    AdmissionGateSpec { id: "INT-ASUP-02", foundation: "FD-ASUP", own_crates: &["asupersync"], own_groups: &[], own_tool: None, required_thresholds: &[] },
    AdmissionGateSpec { id: "INT-FSQL-01", foundation: "FD-FSQL", own_crates: &["fsqlite"], own_groups: &[], own_tool: None, required_thresholds: &[] },
    AdmissionGateSpec { id: "INT-FSQL-02", foundation: "FD-FSQL", own_crates: &["fsqlite"], own_groups: &[], own_tool: None, required_thresholds: &[] },
    AdmissionGateSpec { id: "INT-FT-01", foundation: "FD-FT", own_crates: &["ft-kernel-metal"], own_groups: &[], own_tool: None, required_thresholds: &[] },
    AdmissionGateSpec { id: "INT-FT-02", foundation: "FD-FT", own_crates: &["ft-kernel-metal"], own_groups: &[], own_tool: None, required_thresholds: &[] },
    AdmissionGateSpec { id: "INT-CUDA-01", foundation: "FD-CUDA", own_crates: &["cudarc"], own_groups: &[], own_tool: None, required_thresholds: &[] },
    AdmissionGateSpec { id: "INT-FSYM-01", foundation: "FD-FSYM", own_crates: &["fsym-core"], own_groups: &[], own_tool: None, required_thresholds: &[] },
    AdmissionGateSpec { id: "INT-FSYM-02", foundation: "FD-FSYM", own_crates: &["fsym-core"], own_groups: &[], own_tool: None, required_thresholds: &[] },
    AdmissionGateSpec { id: "INT-FLN-01", foundation: "FD-FLN", own_crates: &[], own_groups: &[], own_tool: Some("fln"), required_thresholds: &[] },
    AdmissionGateSpec { id: "INT-FLN-02", foundation: "FD-FLN", own_crates: &[], own_groups: &[], own_tool: Some("fln"), required_thresholds: &[] },
    AdmissionGateSpec { id: "INT-FMCP-01", foundation: "FD-FMCP", own_crates: &["fastmcp"], own_groups: &[], own_tool: None, required_thresholds: &[] },
    AdmissionGateSpec { id: "INT-FNP-01", foundation: "FD-FNP", own_crates: &["fnp-io"], own_groups: &[], own_tool: None, required_thresholds: &[] },
    AdmissionGateSpec { id: "INT-FNP-02", foundation: "FD-FNP", own_crates: &["fnp-io"], own_groups: &[], own_tool: None, required_thresholds: &[] },
    AdmissionGateSpec { id: "INT-FTUI-01", foundation: "FD-FTUI", own_crates: &["ftui"], own_groups: &[], own_tool: None, required_thresholds: &[] },
    AdmissionGateSpec { id: "INT-F3D-01", foundation: "FD-F3D", own_crates: &[], own_groups: &[], own_tool: Some("franken_threed"), required_thresholds: &[] },
    AdmissionGateSpec { id: "INT-FDGR-01", foundation: "FD-FDGR", own_crates: &[], own_groups: &[], own_tool: Some("fdgr"), required_thresholds: &[] },
    AdmissionGateSpec { id: "INT-FJAX-01", foundation: "FD-FJAX", own_crates: &[], own_groups: &[], own_tool: None, required_thresholds: &[] },
    AdmissionGateSpec { id: "INT-FTX-01", foundation: "FD-FTX", own_crates: &[], own_groups: &[], own_tool: Some("ftx-drape"), required_thresholds: &[] },
];

/// Look up an admission gate specification by identifier.
#[must_use]
pub fn gate_spec(gate_id: &str) -> Option<&'static AdmissionGateSpec> {
    ADMISSION_GATES.iter().find(|g| g.id == gate_id)
}

// ============================================================================
// Typed model for `frob.admission/1`
// ============================================================================

/// Numeric value carried in a `measured` entry (exact int53 or finite IEEE-754 f64).
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

impl fmt::Display for MeasureNum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int(n) => write!(f, "{n}"),
            Self::Float(x) => write!(f, "{x}"),
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

/// Comparison relation on a measured threshold.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

    pub fn parse(raw: &str) -> Result<Self, AdmissionParseError> {
        match raw {
            "<=" => Ok(Self::Le),
            "<" => Ok(Self::Lt),
            ">=" => Ok(Self::Ge),
            ">" => Ok(Self::Gt),
            "==" => Ok(Self::Eq),
            other => Err(AdmissionParseError::new(
                "measured.relation",
                format!("unknown relation `{other}` (expected <=, <, >=, >, ==)"),
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

/// Subprocess tool specification inside `covers.tool`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ToolSpec {
    pub name: String,
    /// 40-hex commit of the tool's repository (required by rule 7).
    pub rev: Option<String>,
    /// 64-hex BLAKE3 digest of the executed tool binary (required by rule 7).
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

/// Scope covered by an admission record (`crates`, airlock `groups`, or subprocess `tool`).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct AdmissionCovers {
    pub crates: Vec<String>,
    pub groups: Vec<String>,
    pub tool: Option<ToolSpec>,
}

impl AdmissionCovers {
    /// Canonical normalized `(sorted crates, sorted groups, tool name)` for scope matching.
    #[must_use]
    pub fn normalized_key(&self) -> (BTreeSet<String>, BTreeSet<String>, Option<String>) {
        (
            self.crates.iter().cloned().collect(),
            self.groups.iter().cloned().collect(),
            self.tool.as_ref().map(|t| t.name.clone()),
        )
    }

    #[must_use]
    pub fn covers_crate(&self, name: &str) -> bool {
        self.crates.iter().any(|c| c == name)
    }

    #[must_use]
    pub fn covers_group(&self, group: &str) -> bool {
        self.groups.iter().any(|g| g == group)
    }

    #[must_use]
    pub fn covers_tool(&self, tool: &str) -> bool {
        self.tool.as_ref().is_some_and(|t| t.name == tool)
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

/// One measured quantity recorded against a threshold.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MeasuredEntry {
    pub name: String,
    /// `None` when omitted or null (caught by validator rule 8 `measured-missing`).
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

/// One negative-control outcome in an admission record.
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

/// One sibling source fact re-read at the pinned commit (`N33a-09`).
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

/// Closed status enumeration of `frob.admission/1`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

    pub fn parse(raw: &str) -> Result<Self, AdmissionParseError> {
        match raw {
            "pass" => Ok(Self::Pass),
            "fail" => Ok(Self::Fail),
            "not-run" => Ok(Self::NotRun),
            "killed" => Ok(Self::Killed),
            other => Err(AdmissionParseError::new(
                "status",
                format!("unknown admission status `{other}` (expected pass, fail, not-run, killed)"),
            )),
        }
    }
}

impl fmt::Display for AdmissionStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Execution host metadata (`trj` or `m5-ultra` plus optional 64-hex fingerprint).
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

/// Provenance block recording who ran and implemented the gate.
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

/// Complete typed `frob.admission/1` admission record.
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
        // Explicit UTF-8 code-point key order (also verified by frob_core::canon).
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
    /// Serialize to canonical JSON exclusively through `frob_core::canon`.
    pub fn to_canonical_bytes(&self) -> Result<CanonicalBytes, AdmissionParseError> {
        if self.output_excerpt.len() > MAX_EXCERPT_BYTES {
            return Err(AdmissionParseError::new(
                "output_excerpt",
                format!(
                    "output_excerpt length {} exceeds {MAX_EXCERPT_BYTES} bytes",
                    self.output_excerpt.len()
                ),
            ));
        }
        canon::to_canonical(self)
            .map_err(|e| AdmissionParseError::new("canon", e.to_string()))
    }

    /// Plain BLAKE3 lowercase hex digest of the canonical record bytes (its filename stem).
    pub fn digest_hex(&self) -> Result<String, AdmissionParseError> {
        let bytes = self.to_canonical_bytes()?;
        Ok(fs_blake3::hash_bytes(bytes.as_bytes()).to_hex())
    }

    /// Raw 64-hex stack receipt hash stripped of the `"sr:"` prefix.
    #[must_use]
    pub fn stack_receipt_hex(&self) -> &str {
        self.stack_receipt
            .strip_prefix("sr:")
            .unwrap_or(&self.stack_receipt)
    }

    /// True if this record's `covers` equals the gate's own scope (Review Notes 2 & 4).
    ///
    /// First-use / partial-scope records filed under a gate with a fixed own scope
    /// (such as `INT-FSIM-01` or `INT-FSIM-14`) return `false` here so they do not
    /// require the parent gate row's thresholds or overwrite `admitted_gates`.
    #[must_use]
    pub fn is_gate_own_scope(&self) -> bool {
        let Some(spec) = gate_spec(&self.gate) else {
            // Rowless third-party gate (e.g. ADMIT-CAP-STD): every record is its own scope.
            return true;
        };
        if spec.foundation != "WORKSPACE" && self.foundation != spec.foundation {
            return false;
        }
        if spec.own_crates.is_empty() && spec.own_groups.is_empty() && spec.own_tool.is_none() {
            return true;
        }
        let expected_crates: BTreeSet<String> =
            spec.own_crates.iter().map(|s| (*s).to_owned()).collect();
        let expected_groups: BTreeSet<String> =
            spec.own_groups.iter().map(|s| (*s).to_owned()).collect();
        let expected_tool = spec.own_tool.map(ToOwned::to_owned);
        self.covers.normalized_key() == (expected_crates, expected_groups, expected_tool)
    }

    /// Parse raw canonical JSON bytes via `frob_core::json::admit` (N34-17).
    pub fn from_json_bytes(bytes: &[u8]) -> Result<Self, AdmissionParseError> {
        let admitted = json::admit(bytes, &Limits::default())
            .map_err(|e| AdmissionParseError::new("json", e.to_string()))?;
        let rec = decode_admission_json(admitted.value())?;
        Ok(rec)
    }
}

/// Structured parse error for `frob.admission/1`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdmissionParseError {
    pub field: String,
    pub message: String,
}

impl AdmissionParseError {
    #[must_use]
    pub fn new(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            message: message.into(),
        }
    }
}

impl fmt::Display for AdmissionParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "admission parse error at `{}`: {}", self.field, self.message)
    }
}

impl std::error::Error for AdmissionParseError {}

/// One loaded admission record file from `receipts/admissions/<gate>/<hash>.json`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoredAdmission {
    /// Repository-relative path (`receipts/admissions/<gate>/<hash>.json`).
    pub rel_path: String,
    /// Full filesystem path.
    pub abs_path: PathBuf,
    /// 64-hex BLAKE3 hash computed from the stored bytes.
    pub hash: String,
    /// Filename stem before `.json`.
    pub stem: String,
    pub record: AdmissionRecord,
}

/// Structured finding returned by [`validate_record`] or [`never_inherited`].
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AdmissionFinding {
    /// Check-stack rule identifier (`stack.admission` or `stack.admission-record`).
    pub rule: &'static str,
    /// Precise condition kind (`lockfile-lacks-foundation`, `unadmitted-use`, etc.).
    pub kind: &'static str,
    /// Subject (gate, foundation, crate/group, or record path).
    pub subject: String,
    /// Repository-relative path to the offending file.
    pub path: String,
    /// Human-readable detail message.
    pub message: String,
    /// Actionable repair hint.
    pub repair: String,
    /// Optional `FD-…` identifier for `check-stack` `facts.kinds`.
    pub fd: Option<String>,
    /// True when this finding fails `check-stack` (false for gate-test-only informational uses).
    pub fails: bool,
}

impl AdmissionFinding {
    #[must_use]
    pub fn to_xtask_finding(&self) -> crate::crate_map::Finding {
        crate::crate_map::Finding {
            rule: self.rule,
            subject: self.subject.clone(),
            path: self.path.clone(),
            line: None,
            message: format!("{}: {}", self.kind, self.message),
            repair: self.repair.clone(),
        }
    }
}

/// One active foundation use target checked by [`never_inherited`].
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UseTarget {
    /// A direct foundation crate dependency (e.g. `fs-mbd`, `ft-nn`, `ft-kernel-metal`).
    Crate { fd: String, crate_name: String },
    /// An enabled `frob-fsim` airlock feature group (e.g. `stats`, `optimize`, `structure`).
    Group { fd: String, group: String },
    /// A subprocess foundation tool (e.g. `fln`, `ftx-drape`).
    Tool { fd: String, tool: String },
}

impl UseTarget {
    #[must_use]
    pub fn fd(&self) -> &str {
        match self {
            Self::Crate { fd, .. } | Self::Group { fd, .. } | Self::Tool { fd, .. } => fd,
        }
    }

    #[must_use]
    pub fn label(&self) -> String {
        match self {
            Self::Crate { crate_name, .. } => crate_name.clone(),
            Self::Group { group, .. } => group.clone(),
            Self::Tool { tool, .. } => tool.clone(),
        }
    }

    #[must_use]
    pub fn is_covered_by(&self, covers: &AdmissionCovers) -> bool {
        match self {
            Self::Crate { crate_name, .. } => covers.covers_crate(crate_name),
            Self::Group { group, .. } => covers.covers_group(group),
            Self::Tool { tool, .. } => covers.covers_tool(tool),
        }
    }
}

/// One enumerated active use along with its consuming workspace members and covering records.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumeratedUse {
    pub target: UseTarget,
    /// Workspace member crate names that directly enable/depend on this use.
    pub consumers: BTreeSet<String>,
    /// True if every consumer is a gate-test-only crate (`frob-conformance`).
    pub gate_test_only: bool,
    /// True if the target is a `BEDROCK_DIRECT_CRATES` crate used only by layer-0/tooling (`frob-core`, `xtask`).
    pub bedrock_bootstrap_only: bool,
}

/// One covering admission record returned oldest-first per use (for `N40-07`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoveringRecordRef {
    pub gate: String,
    pub hash: String,
    pub repo_rev: String,
    pub sibling_rev: Option<String>,
    pub date: String,
    pub status: AdmissionStatus,
}

/// Output of [`never_inherited`]: findings plus per-use covering records oldest first.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NeverInheritedReport {
    pub uses: Vec<EnumeratedUse>,
    pub findings: Vec<AdmissionFinding>,
    /// Per-use valid passing covering records ordered oldest first (`(date, hash)` ascending).
    pub covering_records: BTreeMap<UseTarget, Vec<CoveringRecordRef>>,
}

/// Summary of one gate's newest own-scope record (for `status()` / `frob doctor`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateStatusEntry {
    pub gate: String,
    pub status: AdmissionStatus,
    pub record_hash: Option<String>,
    pub date: Option<String>,
    pub sibling_rev: Option<String>,
    pub valid: bool,
}

/// Summary of one foundation from the 18-foundation ledger (for `status()` / `frob doctor`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationStatus {
    pub fd: String,
    pub name: String,
    pub uses: Vec<String>,
    pub pin: Option<String>,
    pub admission_gate: String,
    pub drift_class: Option<DriftClass>,
    pub drift_reason: Option<DriftReason>,
    pub gates: Vec<GateStatusEntry>,
    pub first_use_records: Vec<CoveringRecordRef>,
}

/// Complete admission status snapshot returned by [`status`] (`frob doctor`, M0 artifact).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StackAdmissionStatus {
    pub newest_stack_receipt: Option<String>,
    pub foundations: Vec<FoundationStatus>,
}

// ============================================================================
// Loading records from `receipts/admissions/<gate>/<64-hex>.json`
// ============================================================================

/// Load and hash-check one admission record file.
pub fn load_record_file(repo_root: &Path, abs_path: &Path) -> Result<StoredAdmission, AdmissionFinding> {
    let rel_path = rel_posix(repo_root, abs_path);
    let fname = abs_path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("");
    let stem = fname.strip_suffix(".json").unwrap_or(fname).to_owned();
    let bytes = fs::read(abs_path).map_err(|e| AdmissionFinding {
        rule: RULE_ADMISSION_RECORD,
        kind: "parse-error",
        subject: rel_path.clone(),
        path: rel_path.clone(),
        message: format!("failed to read admission record `{rel_path}`: {e}"),
        repair: "ensure the admission record file is readable UTF-8 JSON".to_owned(),
        fd: None,
        fails: true,
    })?;
    let hash = fs_blake3::hash_bytes(&bytes).to_hex();
    let record = AdmissionRecord::from_json_bytes(&bytes).map_err(|e| AdmissionFinding {
        rule: RULE_ADMISSION_RECORD,
        kind: "parse-error",
        subject: rel_path.clone(),
        path: rel_path.clone(),
        message: e.to_string(),
        repair: "rewrite the admission record via `frob_conformance::admission::record`".to_owned(),
        fd: None,
        fails: true,
    })?;
    Ok(StoredAdmission {
        rel_path,
        abs_path: abs_path.to_path_buf(),
        hash,
        stem,
        record,
    })
}

/// Discover and load all `receipts/admissions/<gate>/*.json` files in deterministic order.
pub fn load_all_records(repo_root: &Path) -> (Vec<StoredAdmission>, Vec<AdmissionFinding>) {
    let dir = repo_root.join("receipts/admissions");
    let mut records = Vec::new();
    let mut parse_findings = Vec::new();
    if !dir.is_dir() {
        return (records, parse_findings);
    }

    let mut json_paths = Vec::new();
    collect_json_files(&dir, &mut json_paths);
    json_paths.sort();

    for path in json_paths {
        match load_record_file(repo_root, &path) {
            Ok(stored) => records.push(stored),
            Err(finding) => parse_findings.push(finding),
        }
    }
    // Deterministic ordering: oldest first by (date, hash, rel_path).
    records.sort_by(|a, b| {
        a.record
            .recorded
            .date
            .cmp(&b.record.recorded.date)
            .then_with(|| a.hash.cmp(&b.hash))
            .then_with(|| a.rel_path.cmp(&b.rel_path))
    });
    (records, parse_findings)
}

fn collect_json_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(rd) = fs::read_dir(dir) else { return };
    let mut entries: Vec<PathBuf> = rd.flatten().map(|e| e.path()).collect();
    entries.sort();
    for p in entries {
        if p.is_dir() {
            collect_json_files(&p, out);
        } else if p.is_file() && p.extension().and_then(|s| s.to_str()) == Some("json") {
            out.push(p);
        }
    }
}

// ============================================================================
// Validator: rules 1–8 + filename-hash integrity (`stack::admission::validate`)
// ============================================================================

/// Validate one stored admission record against all 8 rules of `N33a-07`,
/// filename-to-BLAKE3 integrity, `git show` commit reachability, and the stack
/// receipt + dirty-tree PASS guard (`N33a-05`).
pub fn validate(
    repo_root: &Path,
    stored: &StoredAdmission,
    stack_lock: &StackLock,
) -> Vec<AdmissionFinding> {
    let mut findings = Vec::new();
    let rec = &stored.record;
    let fd_opt = if rec.foundation.starts_with("FD-") {
        Some(rec.foundation.clone())
    } else {
        None
    };

    // 0. Content-addressed filename integrity.
    if !is_lower_hex(&stored.stem, 64) || stored.stem != stored.hash {
        findings.push(AdmissionFinding {
            rule: RULE_ADMISSION_RECORD,
            kind: "hash-mismatch",
            subject: rec.gate.clone(),
            path: stored.rel_path.clone(),
            message: format!(
                "record filename stem `{}` does not match content BLAKE3 `{}`",
                stored.stem, stored.hash
            ),
            repair: "restore the untampered admission record file named by its BLAKE3 hash".to_owned(),
            fd: fd_opt.clone(),
            fails: true,
        });
    }

    // Rule 1 (`lockfile-lacks-foundation` / `commit-unreachable`):
    // For each crate in `covers.crates`, `git show <repo_rev>:<ws_lockfile>` must
    // contain a `[[package]]` with that name.
    if !rec.covers.crates.is_empty() {
        let lock_rel = workspace_lockfile_rel(&rec.workspace);
        match git_commit_exists_ro(repo_root, &rec.repo_rev) {
            false => {
                findings.push(AdmissionFinding {
                    rule: RULE_ADMISSION_RECORD,
                    kind: "commit-unreachable",
                    subject: rec.gate.clone(),
                    path: stored.rel_path.clone(),
                    message: format!(
                        "cited repo_rev `{}` is absent from local git history (shallow clone or unknown commit)",
                        rec.repo_rev
                    ),
                    repair: "fetch full git history or record the admission against a reachable commit".to_owned(),
                    fd: fd_opt.clone(),
                    fails: true,
                });
            }
            true => match git_show_ro(repo_root, &rec.repo_rev, &lock_rel) {
                Ok(lock_bytes) => {
                    let pkg_names = parse_cargo_lock_package_names(&lock_bytes);
                    for crate_name in &rec.covers.crates {
                        if !pkg_names.contains(crate_name.as_str()) {
                            findings.push(AdmissionFinding {
                                rule: RULE_ADMISSION_RECORD,
                                kind: "lockfile-lacks-foundation",
                                subject: rec.gate.clone(),
                                path: stored.rel_path.clone(),
                                message: format!(
                                    "covered crate `{crate_name}` is absent from `{}` at commit `{}`",
                                    lock_rel, rec.repo_rev
                                ),
                                repair: "add the dependency to the workspace, commit Cargo.lock, and re-record the admission".to_owned(),
                                fd: fd_opt.clone(),
                                fails: true,
                            });
                        }
                    }
                }
                Err(detail) => {
                    findings.push(AdmissionFinding {
                        rule: RULE_ADMISSION_RECORD,
                        kind: "lockfile-lacks-foundation",
                        subject: rec.gate.clone(),
                        path: stored.rel_path.clone(),
                        message: format!(
                            "could not read `{lock_rel}` at commit `{}`: {detail}",
                            rec.repo_rev
                        ),
                        repair: "ensure the workspace Cargo.lock is committed at repo_rev".to_owned(),
                        fd: fd_opt.clone(),
                        fails: true,
                    });
                }
            },
        }
    } else if !git_commit_exists_ro(repo_root, &rec.repo_rev) {
        findings.push(AdmissionFinding {
            rule: RULE_ADMISSION_RECORD,
            kind: "commit-unreachable",
            subject: rec.gate.clone(),
            path: stored.rel_path.clone(),
            message: format!(
                "cited repo_rev `{}` is absent from local git history",
                rec.repo_rev
            ),
            repair: "fetch full git history or record the admission against a reachable commit".to_owned(),
            fd: fd_opt.clone(),
            fails: true,
        });
    }

    // Rule 2 (`receipt-missing`): `stack_receipt` names an intact file under `receipts/`
    // whose entry for `foundation` has `rev == sibling_rev`.
    let receipt_hex = rec.stack_receipt_hex();
    let receipt_path = repo_root.join("receipts").join(format!("{receipt_hex}.json"));
    let loaded_receipt: Option<StackReceipt> = if !receipt_path.is_file() {
        findings.push(AdmissionFinding {
            rule: RULE_ADMISSION_RECORD,
            kind: "receipt-missing",
            subject: rec.gate.clone(),
            path: stored.rel_path.clone(),
            message: format!(
                "cited stack_receipt `{}` names no file at `receipts/{receipt_hex}.json`",
                rec.stack_receipt
            ),
            repair: "commit the resolution receipt under receipts/ before recording an admission".to_owned(),
            fd: fd_opt.clone(),
            fails: true,
        });
        None
    } else {
        match receipt::load_and_verify_receipt(&receipt_path) {
            Ok((sr, _, _)) => {
                // Verify sibling_rev matches the stack receipt entry for sibling foundations.
                let needs_sibling = stack_lock
                    .foundation(&rec.foundation)
                    .is_some_and(|row| row.path.is_some());
                if let Some(sib_entry) = sr.siblings.iter().find(|s| s.fd == rec.foundation) {
                    if rec.sibling_rev.as_deref() != Some(sib_entry.rev.as_str()) {
                        findings.push(AdmissionFinding {
                            rule: RULE_ADMISSION_RECORD,
                            kind: "receipt-missing",
                            subject: rec.gate.clone(),
                            path: stored.rel_path.clone(),
                            message: format!(
                                "stack_receipt `{receipt_hex}` records `{}` at rev `{}`, but admission cites `{}`",
                                rec.foundation,
                                sib_entry.rev,
                                rec.sibling_rev.as_deref().unwrap_or("<null>")
                            ),
                            repair: "re-record the admission against a stack receipt matching sibling_rev".to_owned(),
                            fd: fd_opt.clone(),
                            fails: true,
                        });
                    }
                } else if needs_sibling {
                    findings.push(AdmissionFinding {
                        rule: RULE_ADMISSION_RECORD,
                        kind: "receipt-missing",
                        subject: rec.gate.clone(),
                        path: stored.rel_path.clone(),
                        message: format!(
                            "stack_receipt `{receipt_hex}` contains no sibling entry for `{}`",
                            rec.foundation
                        ),
                        repair: "refresh the stack receipt with `cargo xtask check-stack --write-receipt`".to_owned(),
                        fd: fd_opt.clone(),
                        fails: true,
                    });
                }
                Some(sr)
            }
            Err(err) => {
                findings.push(AdmissionFinding {
                    rule: RULE_ADMISSION_RECORD,
                    kind: "receipt-missing",
                    subject: rec.gate.clone(),
                    path: stored.rel_path.clone(),
                    message: format!(
                        "cited stack_receipt `{receipt_hex}` failed integrity verification: {err}"
                    ),
                    repair: "restore the untampered stack receipt file".to_owned(),
                    fd: fd_opt.clone(),
                    fails: true,
                });
                None
            }
        }
    };

    // Rule 3 (`output-missing`): `receipts/outputs/<output_hash>.txt` exists and hashes to its name.
    let output_path = repo_root
        .join("receipts/outputs")
        .join(format!("{}.txt", rec.output_hash));
    if !output_path.is_file() {
        findings.push(AdmissionFinding {
            rule: RULE_ADMISSION_RECORD,
            kind: "output-missing",
            subject: rec.gate.clone(),
            path: stored.rel_path.clone(),
            message: format!(
                "cited output blob `receipts/outputs/{}.txt` does not exist",
                rec.output_hash
            ),
            repair: "store the full command output under receipts/outputs/<output_hash>.txt".to_owned(),
            fd: fd_opt.clone(),
            fails: true,
        });
    } else {
        match blake3_file(&output_path) {
            Ok(actual) if actual == rec.output_hash => {}
            Ok(actual) => {
                findings.push(AdmissionFinding {
                    rule: RULE_ADMISSION_RECORD,
                    kind: "output-missing",
                    subject: rec.gate.clone(),
                    path: stored.rel_path.clone(),
                    message: format!(
                        "output blob `receipts/outputs/{}.txt` hashes to `{actual}` instead of its name",
                        rec.output_hash
                    ),
                    repair: "restore the untampered command output blob".to_owned(),
                    fd: fd_opt.clone(),
                    fails: true,
                });
            }
            Err(err) => {
                findings.push(AdmissionFinding {
                    rule: RULE_ADMISSION_RECORD,
                    kind: "output-missing",
                    subject: rec.gate.clone(),
                    path: stored.rel_path.clone(),
                    message: format!("failed to read output blob: {err}"),
                    repair: "ensure receipts/outputs/<output_hash>.txt is readable".to_owned(),
                    fd: fd_opt.clone(),
                    fails: true,
                });
            }
        }
    }

    // Rules 4, 5, 6 apply when `status == Pass`.
    if rec.status == AdmissionStatus::Pass {
        // Rule 4 (`pass-on-dirty`): delegates exclusively to `stack::guard::may_record_pass` (N33a-05).
        if let Some(ref sr) = loaded_receipt {
            if let Err(refusal) = guard::may_record_pass_receipt(sr) {
                findings.push(AdmissionFinding {
                    rule: RULE_ADMISSION_RECORD,
                    kind: "pass-on-dirty",
                    subject: rec.gate.clone(),
                    path: stored.rel_path.clone(),
                    message: format!(
                        "status `pass` refused by dirty-tree guard ({}): {}",
                        refusal.reason, refusal.message
                    ),
                    repair: "record on a clean tree and clean siblings with no asupersync duplicate, or record as fail/not-run".to_owned(),
                    fd: fd_opt.clone(),
                    fails: true,
                });
            }
        } else if let Err(refusal) = guard::may_record_pass(&receipt_path) {
            findings.push(AdmissionFinding {
                rule: RULE_ADMISSION_RECORD,
                kind: "pass-on-dirty",
                subject: rec.gate.clone(),
                path: stored.rel_path.clone(),
                message: format!(
                    "status `pass` refused by dirty-tree guard ({}): {}",
                    refusal.reason, refusal.message
                ),
                repair: "record against a valid clean stack receipt".to_owned(),
                fd: fd_opt.clone(),
                fails: true,
            });
        }

        // Rule 5 (`pass-without-control`): at least one control with `caught == true` and none with `caught == false`.
        let any_caught = rec.controls.iter().any(|c| c.caught);
        let any_missed = rec.controls.iter().any(|c| !c.caught);
        if !any_caught || any_missed {
            findings.push(AdmissionFinding {
                rule: RULE_ADMISSION_RECORD,
                kind: "pass-without-control",
                subject: rec.gate.clone(),
                path: stored.rel_path.clone(),
                message: if !any_caught {
                    "status `pass` requires at least one negative control with `caught: true`".to_owned()
                } else {
                    "status `pass` refused because at least one negative control has `caught: false`".to_owned()
                },
                repair: "run and catch every negative control for the gate before recording `pass`".to_owned(),
                fd: fd_opt.clone(),
                fails: true,
            });
        }

        // Rule 6 (`self-certified`): `recorded.session != recorded.implemented_by`, both non-empty.
        let sess = rec.recorded.session.trim();
        let impl_by = rec.recorded.implemented_by.trim();
        if sess.is_empty() || impl_by.is_empty() || sess == impl_by {
            findings.push(AdmissionFinding {
                rule: RULE_ADMISSION_RECORD,
                kind: "self-certified",
                subject: rec.gate.clone(),
                path: stored.rel_path.clone(),
                message: format!(
                    "status `pass` refused: self-certification or blank attribution (session=`{sess}`, implemented_by=`{impl_by}`)"
                ),
                repair: "have an independent verification session record `pass` (`recorded.session != recorded.implemented_by`)".to_owned(),
                fd: fd_opt.clone(),
                fails: true,
            });
        }
    }

    // Rule 7 (`tool-unpinned`): Subprocess records (`covers.tool.is_some()`) require both 40-hex rev and 64-hex binary_hash.
    if let Some(tool) = &rec.covers.tool
        && !tool.is_pinned()
    {
        findings.push(AdmissionFinding {
            rule: RULE_ADMISSION_RECORD,
            kind: "tool-unpinned",
            subject: rec.gate.clone(),
            path: stored.rel_path.clone(),
            message: format!(
                "subprocess tool `{}` lacks a pinned 40-hex repository revision and/or 64-hex binary BLAKE3 hash",
                tool.name
            ),
            repair: "populate `covers.tool = {\"name\": ..., \"rev\": \"<40-hex>\", \"binary_hash\": \"<64-hex>\"}`".to_owned(),
            fd: fd_opt.clone(),
            fails: true,
        });
    }

    // Rule 8 (`measured-missing`):
    // - No `measured` entry may omit `value` (target written as measurement) or have empty `unit`.
    // - Gate-own-scope records must include every required threshold of the gate row (Review Notes 2 & 4).
    // - First-use records on `pass` must include at least one valid measured entry.
    // - Rowless `ADMIT-*` gates (Review Note 5) require no gate-row thresholds beyond valid entries.
    for entry in &rec.measured {
        if entry.value.is_none() || entry.unit.trim().is_empty() {
            findings.push(AdmissionFinding {
                rule: RULE_ADMISSION_RECORD,
                kind: "measured-missing",
                subject: rec.gate.clone(),
                path: stored.rel_path.clone(),
                message: format!(
                    "measured entry `{}` is missing a numeric `value` or non-empty `unit` (targets must never stand in for measurements)",
                    entry.name
                ),
                repair: "record the actual measured numeric value and unit from the gate run".to_owned(),
                fd: fd_opt.clone(),
                fails: true,
            });
        }
    }
    if let Some(spec) = gate_spec(&rec.gate) {
        if rec.is_gate_own_scope() {
            for &(req_name, req_unit) in spec.required_thresholds {
                let satisfied = rec.measured.iter().any(|m| {
                    m.name == req_name
                        && m.value.is_some()
                        && !m.unit.trim().is_empty()
                        && (req_unit.is_empty() || m.unit == req_unit)
                });
                if !satisfied {
                    findings.push(AdmissionFinding {
                        rule: RULE_ADMISSION_RECORD,
                        kind: "measured-missing",
                        subject: rec.gate.clone(),
                        path: stored.rel_path.clone(),
                        message: format!(
                            "gate `{}` requires measured threshold `{req_name}` (unit `{req_unit}`) with a numeric value",
                            rec.gate
                        ),
                        repair: format!(
                            "include a measured entry `{req_name}` with numeric value and unit `{req_unit}`"
                        ),
                        fd: fd_opt.clone(),
                        fails: true,
                    });
                }
            }
        } else if rec.status == AdmissionStatus::Pass {
            let has_valid_measure = rec
                .measured
                .iter()
                .any(|m| m.value.is_some() && !m.unit.trim().is_empty());
            if !has_valid_measure {
                findings.push(AdmissionFinding {
                    rule: RULE_ADMISSION_RECORD,
                    kind: "measured-missing",
                    subject: rec.gate.clone(),
                    path: stored.rel_path.clone(),
                    message: format!(
                        "first-use passing record under `{}` requires at least one measured known answer with value, unit, and threshold",
                        rec.gate
                    ),
                    repair: "include at least one measured entry with numeric value, unit, and threshold".to_owned(),
                    fd: fd_opt.clone(),
                    fails: true,
                });
            }
        }
    }

    findings.sort();
    findings.dedup();
    findings
}

/// Validate every stored admission record under `receipts/admissions/`.
pub fn validate_all(repo_root: &Path, stack_lock: &StackLock) -> Vec<AdmissionFinding> {
    let (records, mut findings) = load_all_records(repo_root);
    for stored in &records {
        findings.extend(validate(repo_root, stored, stack_lock));
    }
    findings.sort();
    findings.dedup();
    findings
}

// ============================================================================
// `admitted_gates`: gates whose newest own-scope record validates as `pass`
// ============================================================================

/// Return the set of `GateRef`s whose newest gate-own-scope record under
/// `receipts/admissions/` validates with zero findings and `status == "pass"`.
///
/// Passed to `StackLock::check_admitted` by `check-stack` (`N33a-03`, `N33a-05`).
/// First-use / partial-scope records filed under a register gate never overwrite
/// the gate's own newest record (Review Notes 2 & 4).
pub fn admitted_gates(repo_root: &Path, stack_lock: &StackLock) -> BTreeSet<GateRef> {
    let (records, _) = load_all_records(repo_root);
    let mut newest_own_scope: BTreeMap<String, &StoredAdmission> = BTreeMap::new();
    for stored in &records {
        if !stored.record.is_gate_own_scope() {
            continue;
        }
        // `records` is sorted oldest first by (date, hash, rel_path), so later overwrites earlier.
        newest_own_scope.insert(stored.record.gate.clone(), stored);
    }

    let mut out = BTreeSet::new();
    for (gate_id, stored) in newest_own_scope {
        if stored.record.status != AdmissionStatus::Pass {
            continue;
        }
        if !validate(repo_root, stored, stack_lock).is_empty() {
            continue;
        }
        if let Ok(gref) = GateRef::parse(&gate_id, "admitted_gates") {
            out.insert(gref);
        }
    }
    out
}

// ============================================================================
// Single authoritative `never_inherited` check (used by check-stack, N33b-01, N40-07)
// ============================================================================

/// Evaluate the never-inherited admission rule over `repo_root`.
///
/// - When `restrict_uses` is `None`, enumerates active direct foundation crate
///   uses and enabled `frob-fsim` airlock feature groups from resolved
///   `cargo metadata --format-version 1 --locked --offline` across all present
///   workspaces (never from `Cargo.lock` unenabled optional deps, and never from
///   internal transitive sibling closures — Review Note 3).
/// - When `restrict_uses` is `Some(targets)`, checks only those specified targets
///   (used by `N33b-01` and `N40-07`).
/// - Returns findings (`unadmitted-use`, `admission-stale`) under `stack.admission`
///   and per-use valid passing covering records ordered oldest first.
pub fn never_inherited(
    repo_root: &Path,
    stack_lock: &StackLock,
    restrict_uses: Option<&[UseTarget]>,
) -> Result<NeverInheritedReport, String> {
    let enumerated = match restrict_uses {
        Some(targets) => targets
            .iter()
            .cloned()
            .map(|target| EnumeratedUse {
                target,
                consumers: BTreeSet::new(),
                gate_test_only: false,
                bedrock_bootstrap_only: false,
            })
            .collect(),
        None => enumerate_active_uses(repo_root, stack_lock)?,
    };

    let admissions_dir_present = repo_root.join("receipts/admissions").is_dir();
    let (records, _) = load_all_records(repo_root);
    // Pre-validate every record once so invalid records can never admit a use.
    let valid_pass_records: Vec<&StoredAdmission> = records
        .iter()
        .filter(|s| {
            s.record.status == AdmissionStatus::Pass
                && validate(repo_root, s, stack_lock).is_empty()
        })
        .collect();

    let mut findings = Vec::new();
    let mut covering_records: BTreeMap<UseTarget, Vec<CoveringRecordRef>> = BTreeMap::new();

    for used in &enumerated {
        let fd = used.target.fd();
        let label = used.target.label();
        let matching: Vec<&StoredAdmission> = valid_pass_records
            .iter()
            .copied()
            .filter(|s| {
                (s.record.foundation == fd || fd.is_empty())
                    && used.target.is_covered_by(&s.record.covers)
            })
            .collect();

        // `valid_pass_records` is already sorted oldest-first by (date, hash, rel_path).
        let refs: Vec<CoveringRecordRef> = matching
            .iter()
            .map(|s| CoveringRecordRef {
                gate: s.record.gate.clone(),
                hash: s.hash.clone(),
                repo_rev: s.record.repo_rev.clone(),
                sibling_rev: s.record.sibling_rev.clone(),
                date: s.record.recorded.date.clone(),
                status: s.record.status,
            })
            .collect();
        covering_records.insert(used.target.clone(), refs);

        let fails = !used.gate_test_only
            && !(used.bedrock_bootstrap_only && !admissions_dir_present);

        if matching.is_empty() {
            let subject = match &used.target {
                UseTarget::Group { group, .. } => format!("{fd}:{group}"),
                UseTarget::Crate { crate_name, .. } => format!("{fd}:{crate_name}"),
                UseTarget::Tool { tool, .. } => format!("{fd}:{tool}"),
            };
            findings.push(AdmissionFinding {
                rule: RULE_ADMISSION,
                kind: "unadmitted-use",
                subject,
                path: "franken-stack.lock".to_owned(),
                message: format!(
                    "active use `{label}` of `{fd}` has no valid passing admission record covering it"
                ),
                repair: format!(
                    "run and record an admission gate covering `{label}` via `cargo run -p frob-conformance --example admission -- record ...`"
                ),
                fd: Some(fd.to_owned()),
                fails,
            });
            continue;
        }

        // Check newest covering record's sibling_rev against the current lock pin.
        let newest = matching[matching.len() - 1];
        if let Some(row) = stack_lock.foundation(fd)
            && let Some(GitRev::Commit(pin)) = &row.git_rev
            && newest.record.sibling_rev.as_deref() != Some(pin.as_str())
        {
            let subject = match &used.target {
                UseTarget::Group { group, .. } => format!("{fd}:{group}"),
                UseTarget::Crate { crate_name, .. } => format!("{fd}:{crate_name}"),
                UseTarget::Tool { tool, .. } => format!("{fd}:{tool}"),
            };
            findings.push(AdmissionFinding {
                rule: RULE_ADMISSION,
                kind: "admission-stale",
                subject,
                path: newest.rel_path.clone(),
                message: format!(
                    "newest admission record `{}` for `{label}` ({fd}) covers pin `{}`, but franken-stack.lock pins `{pin}`",
                    newest.hash,
                    newest.record.sibling_rev.as_deref().unwrap_or("<null>")
                ),
                repair: "re-run the admission gate at the new pin as part of the compatibility train".to_owned(),
                fd: Some(fd.to_owned()),
                fails,
            });
        }
    }

    findings.sort();
    findings.dedup();
    Ok(NeverInheritedReport {
        uses: enumerated,
        findings,
        covering_records,
    })
}

/// Enumerate active direct foundation crate uses and enabled `frob-fsim` airlock
/// feature groups from resolved `cargo metadata` across all present workspaces.
pub fn enumerate_active_uses(
    repo_root: &Path,
    stack_lock: &StackLock,
) -> Result<Vec<EnumeratedUse>, String> {
    let mut by_target: BTreeMap<UseTarget, BTreeSet<String>> = BTreeMap::new();

    for &(ws_name, ws_rel) in WORKSPACES {
        let ws_dir = if ws_rel == "." {
            repo_root.to_path_buf()
        } else {
            repo_root.join(ws_rel)
        };
        if !ws_dir.join("Cargo.toml").is_file() || !ws_dir.join("Cargo.lock").is_file() {
            continue;
        }
        let ws = if ws_name == "core" {
            WorkspaceRoot::core(&ws_dir)
        } else {
            WorkspaceRoot::annex(ws_name, ws_rel, &ws_dir)
        };
        let resolved = metadata::load_resolved_metadata(repo_root, &ws)
            .map_err(|e| format!("failed loading resolved metadata for workspace `{ws_name}`: {e}"))?;
        collect_uses_from_resolved(&resolved, stack_lock, &mut by_target);
    }

    let mut out = Vec::with_capacity(by_target.len());
    for (target, consumers) in by_target {
        let gate_test_only = !consumers.is_empty()
            && consumers.iter().all(|c| is_gate_test_crate(c));
        let bedrock_bootstrap_only = matches!(
            &target,
            UseTarget::Crate { crate_name, .. } if BEDROCK_DIRECT_CRATES.contains(&crate_name.as_str())
        ) && !consumers.is_empty()
            && consumers
                .iter()
                .all(|c| matches!(c.as_str(), "frob-core" | "xtask" | "frob-conformance"));
        out.push(EnumeratedUse {
            target,
            consumers,
            gate_test_only,
            bedrock_bootstrap_only,
        });
    }
    Ok(out)
}

fn collect_uses_from_resolved(
    resolved: &ResolvedWorkspace,
    stack_lock: &StackLock,
    out: &mut BTreeMap<UseTarget, BTreeSet<String>>,
) {
    let fsim_fd = stack_lock
        .foundations
        .iter()
        .find(|f| f.id.as_str() == "FD-FSIM" || f.name == "frankensim")
        .map_or("FD-FSIM", |f| f.id.as_str());

    for member_id in &resolved.workspace_members {
        let Some(member_pkg) = resolved.packages.get(member_id) else {
            continue;
        };
        let Some(member_node) = resolved.nodes.get(member_id) else {
            continue;
        };

        // 1. Enabled airlock feature groups on `frob-fsim`.
        if member_pkg.name == "frob-fsim" {
            for feat in &member_node.features {
                if FSIM_AIRLOCK_FEATURE_GROUPS.contains(&feat.as_str()) {
                    let target = UseTarget::Group {
                        fd: fsim_fd.to_owned(),
                        group: feat.clone(),
                    };
                    // Record any workspace member depending on `frob-fsim` (or `frob-fsim` itself).
                    let consumers = out.entry(target).or_default();
                    consumers.insert(member_pkg.name.clone());
                }
            }
        }

        // 2. Direct resolved dependency edges from this workspace member onto foundation crates.
        for dep_edge in &member_node.deps {
            if resolved.workspace_members.contains(&dep_edge.pkg) {
                continue;
            }
            let Some(dep_pkg) = resolved.packages.get(&dep_edge.pkg) else {
                continue;
            };
            // Ignore dev-only dependencies on the gate test crate? Keep consumer attribution so
            // `gate_test_only` marks `frob-conformance` uses informational!
            let _ = dep_edge.dep_kinds.iter().any(|k| *k == DepKind::Dev);
            if let Some(row) = match_resolved_pkg_to_foundation(dep_pkg, stack_lock) {
                let target = UseTarget::Crate {
                    fd: row.id.as_str().to_owned(),
                    crate_name: dep_pkg.name.clone(),
                };
                out.entry(target)
                    .or_default()
                    .insert(member_pkg.name.clone());
            }
        }
    }
}

fn match_resolved_pkg_to_foundation<'a>(
    pkg: &metadata::ResolvedPackage,
    stack_lock: &'a StackLock,
) -> Option<&'a Foundation> {
    if let Some(sib_path) = pkg.sibling_rel_path.as_deref() {
        for row in &stack_lock.foundations {
            if let Some(row_path) = row.path.as_ref() {
                let rp = row_path.as_str().trim_end_matches('/');
                if sib_path == rp || sib_path.starts_with(&format!("{rp}/")) {
                    return Some(row);
                }
            }
        }
    }
    let pkg_norm = pkg.name.replace('-', "_");
    for row in &stack_lock.foundations {
        if row.name == pkg.name || row.name.replace('-', "_") == pkg_norm {
            return Some(row);
        }
    }
    // Well-known foundation crate prefixes (e.g. `ft-*` -> FD-FT, `fnp-*` -> FD-FNP, `fsym-*` -> FD-FSYM).
    if BEDROCK_DIRECT_CRATES.contains(&pkg.name.as_str()) {
        return stack_lock.foundation("FD-FSIM");
    }
    if pkg.name.starts_with("ft-") {
        return stack_lock.foundation("FD-FT");
    }
    if pkg.name.starts_with("fnp-") {
        return stack_lock.foundation("FD-FNP");
    }
    if pkg.name.starts_with("fsym-") {
        return stack_lock.foundation("FD-FSYM");
    }
    if pkg.name == "fsqlite" || pkg.name.starts_with("fsqlite-") {
        return stack_lock.foundation("FD-FSQL");
    }
    if pkg.name == "asupersync" {
        return stack_lock.foundation("FD-ASUP");
    }
    if pkg.name == "cudarc" {
        return stack_lock.foundation("FD-CUDA");
    }
    None
}

fn is_gate_test_crate(name: &str) -> bool {
    name == "frob-conformance" || name.ends_with("-gate-test")
}

// ============================================================================
// `status`: foundation admission summary for `frob doctor` (M0 artifact)
// ============================================================================

/// Build the complete foundation admission status summary over `repo_root`
/// (all foundations from `franken-stack.lock`, newest gate-own-scope record per
/// gate or `not-run`, first-use records, drift class from `receipts/HEAD`, and
/// the newest stack receipt hash).
pub fn status(repo_root: &Path) -> Result<StackAdmissionStatus, String> {
    let lock_path = repo_root.join("franken-stack.lock");
    let stack_lock =
        lock::load(&lock_path).map_err(|e| format!("failed to load franken-stack.lock: {e}"))?;

    let receipts_dir = repo_root.join("receipts");
    let newest_stack_receipt = fs::read_to_string(receipts_dir.join("HEAD"))
        .ok()
        .map(|s| s.trim().to_owned())
        .filter(|h| is_lower_hex(h, 64));

    let loaded_sr = newest_stack_receipt.as_deref().and_then(|h| {
        let p = receipts_dir.join(format!("{h}.json"));
        receipt::load_and_verify_receipt(&p).ok().map(|(sr, _, _)| sr)
    });

    let (records, _) = load_all_records(repo_root);
    let mut own_by_gate: BTreeMap<String, &StoredAdmission> = BTreeMap::new();
    let mut first_use_by_fd: BTreeMap<String, Vec<CoveringRecordRef>> = BTreeMap::new();

    for stored in &records {
        if stored.record.is_gate_own_scope() {
            own_by_gate.insert(stored.record.gate.clone(), stored);
        } else {
            first_use_by_fd
                .entry(stored.record.foundation.clone())
                .or_default()
                .push(CoveringRecordRef {
                    gate: stored.record.gate.clone(),
                    hash: stored.hash.clone(),
                    repo_rev: stored.record.repo_rev.clone(),
                    sibling_rev: stored.record.sibling_rev.clone(),
                    date: stored.record.recorded.date.clone(),
                    status: stored.record.status,
                });
        }
    }

    let mut foundations = Vec::with_capacity(stack_lock.foundations.len());
    for row in &stack_lock.foundations {
        let fd = row.id.as_str().to_owned();
        let uses: Vec<String> = row
            .uses
            .iter()
            .map(|u| {
                if u.purpose.is_empty() {
                    u.posture.as_str().to_owned()
                } else {
                    format!("{}: {}", u.posture.as_str(), u.purpose)
                }
            })
            .collect();
        let pin = row.git_rev.as_ref().map(|r| r.as_str().to_owned());

        let (drift_class, drift_reason) = loaded_sr
            .as_ref()
            .and_then(|sr| sr.siblings.iter().find(|s| s.fd == fd))
            .map_or((None, None), |s| (Some(s.own_lock.class), s.own_lock.reason));

        // Expand gate range or collect all ADMISSION_GATES belonging to this foundation.
        let gate_ids = gates_for_foundation(row);
        let mut gates = Vec::with_capacity(gate_ids.len());
        for gid in gate_ids {
            if let Some(stored) = own_by_gate.get(&gid) {
                let valid = validate(repo_root, stored, &stack_lock).is_empty();
                gates.push(GateStatusEntry {
                    gate: gid,
                    status: stored.record.status,
                    record_hash: Some(stored.hash.clone()),
                    date: Some(stored.record.recorded.date.clone()),
                    sibling_rev: stored.record.sibling_rev.clone(),
                    valid,
                });
            } else {
                gates.push(GateStatusEntry {
                    gate: gid,
                    status: AdmissionStatus::NotRun,
                    record_hash: None,
                    date: None,
                    sibling_rev: None,
                    valid: true,
                });
            }
        }

        let first_use_records = first_use_by_fd.remove(&fd).unwrap_or_default();
        foundations.push(FoundationStatus {
            fd,
            name: row.name.clone(),
            uses,
            pin,
            admission_gate: row.admission_gate.as_str().to_owned(),
            drift_class,
            drift_reason,
            gates,
            first_use_records,
        });
    }

    Ok(StackAdmissionStatus {
        newest_stack_receipt,
        foundations,
    })
}

fn gates_for_foundation(row: &Foundation) -> Vec<String> {
    let fd = row.id.as_str();
    let spec_gates: Vec<String> = ADMISSION_GATES
        .iter()
        .filter(|g| g.foundation == fd)
        .map(|g| g.id.to_owned())
        .collect();
    if !spec_gates.is_empty() {
        return spec_gates;
    }
    let raw = row.admission_gate.as_str();
    if raw == "none" {
        Vec::new()
    } else {
        vec![raw.to_owned()]
    }
}

// ============================================================================
// Strict JSON decoding via `frob_core::json` (N34-17)
// ============================================================================

fn decode_admission_json(root: &Json<'_>) -> Result<AdmissionRecord, AdmissionParseError> {
    let members = root
        .as_object()
        .ok_or_else(|| AdmissionParseError::new("<root>", "admission record must be a JSON object"))?;

    const ALLOWED_KEYS: &[&str] = &[
        "command",
        "controls",
        "covers",
        "exit",
        "facts_reread",
        "foundation",
        "gate",
        "harness_receipt",
        "host",
        "measured",
        "output_excerpt",
        "output_hash",
        "recorded",
        "repo_rev",
        "schema",
        "sibling_rev",
        "stack_receipt",
        "status",
        "toolchain",
        "workspace",
    ];
    for m in members {
        if !ALLOWED_KEYS.contains(&m.name.as_str()) {
            return Err(AdmissionParseError::new(
                &m.name,
                format!("unknown field `{}` in frob.admission/1", m.name),
            ));
        }
    }

    let schema = req_str(root, "schema")?;
    if schema != ADMISSION_SCHEMA {
        return Err(AdmissionParseError::new(
            "schema",
            format!("unexpected schema `{schema}` (expected `{ADMISSION_SCHEMA}`)"),
        ));
    }

    let gate = req_str(root, "gate")?;
    if !is_valid_gate_id(&gate) {
        return Err(AdmissionParseError::new(
            "gate",
            format!("invalid admission gate identifier `{gate}`"),
        ));
    }

    let foundation = req_str(root, "foundation")?;
    if !is_valid_foundation_id(&foundation) {
        return Err(AdmissionParseError::new(
            "foundation",
            format!("invalid foundation identifier `{foundation}`"),
        ));
    }

    let covers = decode_covers(
        root.get("covers")
            .ok_or_else(|| AdmissionParseError::new("covers", "missing required `covers`"))?,
    )?;

    let repo_rev = req_str(root, "repo_rev")?;
    if !is_lower_hex(&repo_rev, 40) {
        return Err(AdmissionParseError::new(
            "repo_rev",
            format!("repo_rev `{repo_rev}` must be 40 lower-case hex digits"),
        ));
    }

    let workspace = req_str(root, "workspace")?;
    if !WORKSPACES.iter().any(|&(w, _)| w == workspace) {
        return Err(AdmissionParseError::new(
            "workspace",
            format!("unknown workspace `{workspace}`"),
        ));
    }

    let stack_receipt = req_str(root, "stack_receipt")?;
    let Some(sr_hex) = stack_receipt.strip_prefix("sr:") else {
        return Err(AdmissionParseError::new(
            "stack_receipt",
            format!("stack_receipt `{stack_receipt}` must begin with `sr:`"),
        ));
    };
    if !is_lower_hex(sr_hex, 64) {
        return Err(AdmissionParseError::new(
            "stack_receipt",
            format!("stack_receipt hash `{sr_hex}` must be 64 lower-case hex digits"),
        ));
    }

    let sibling_rev = match root
        .get("sibling_rev")
        .ok_or_else(|| AdmissionParseError::new("sibling_rev", "missing required `sibling_rev`"))?
    {
        v if v.is_null() => None,
        v => {
            let s = v.as_str().ok_or_else(|| {
                AdmissionParseError::new("sibling_rev", "`sibling_rev` must be 40-hex string or null")
            })?;
            if !is_lower_hex(s, 40) {
                return Err(AdmissionParseError::new(
                    "sibling_rev",
                    format!("sibling_rev `{s}` must be 40 lower-case hex digits"),
                ));
            }
            Some(s.to_owned())
        }
    };

    let toolchain = req_str(root, "toolchain")?;
    if toolchain.trim().is_empty() {
        return Err(AdmissionParseError::new("toolchain", "`toolchain` must be non-empty"));
    }

    // `harness_receipt` key is REQUIRED on every record (`null` or 64-hex string; any other value or omission is a parse error).
    let harness_receipt = match root.get("harness_receipt") {
        None => {
            return Err(AdmissionParseError::new(
                "harness_receipt",
                "missing required `harness_receipt` key (`null` or 64 lower-case hex digits)",
            ));
        }
        Some(v) if v.is_null() => None,
        Some(v) => {
            let s = v.as_str().ok_or_else(|| {
                AdmissionParseError::new(
                    "harness_receipt",
                    "`harness_receipt` must be null or 64 lower-case hex digits",
                )
            })?;
            if !is_lower_hex(s, 64) {
                return Err(AdmissionParseError::new(
                    "harness_receipt",
                    format!("`harness_receipt` must be 64 lower-case hex digits, got `{s}`"),
                ));
            }
            Some(s.to_owned())
        }
    };

    let command = req_str_vec(root, "command")?;
    if command.is_empty() {
        return Err(AdmissionParseError::new("command", "`command` must be non-empty"));
    }

    let exit = root
        .get("exit")
        .ok_or_else(|| AdmissionParseError::new("exit", "missing required `exit`"))?
        .as_int53()
        .map_err(|e| AdmissionParseError::new("exit", format!("`exit` must be int53: {e}")))?;

    let output_hash = req_str(root, "output_hash")?;
    if !is_lower_hex(&output_hash, 64) {
        return Err(AdmissionParseError::new(
            "output_hash",
            format!("output_hash `{output_hash}` must be 64 lower-case hex digits"),
        ));
    }

    let output_excerpt = req_str(root, "output_excerpt")?;
    if output_excerpt.len() > MAX_EXCERPT_BYTES {
        return Err(AdmissionParseError::new(
            "output_excerpt",
            format!(
                "output_excerpt length {} exceeds {MAX_EXCERPT_BYTES} bytes",
                output_excerpt.len()
            ),
        ));
    }

    let measured = decode_measured_vec(
        root.get("measured")
            .ok_or_else(|| AdmissionParseError::new("measured", "missing required `measured`"))?,
    )?;
    let controls = decode_controls_vec(
        root.get("controls")
            .ok_or_else(|| AdmissionParseError::new("controls", "missing required `controls`"))?,
    )?;
    let facts_reread = decode_facts_vec(
        root.get("facts_reread")
            .ok_or_else(|| AdmissionParseError::new("facts_reread", "missing required `facts_reread`"))?,
    )?;

    let status = AdmissionStatus::parse(&req_str(root, "status")?)?;
    let host = decode_host(
        root.get("host")
            .ok_or_else(|| AdmissionParseError::new("host", "missing required `host`"))?,
    )?;
    let recorded = decode_recorded(
        root.get("recorded")
            .ok_or_else(|| AdmissionParseError::new("recorded", "missing required `recorded`"))?,
    )?;

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

fn decode_covers(node: &Json<'_>) -> Result<AdmissionCovers, AdmissionParseError> {
    let members = node
        .as_object()
        .ok_or_else(|| AdmissionParseError::new("covers", "`covers` must be JSON object"))?;
    for m in members {
        if !matches!(m.name.as_str(), "crates" | "groups" | "tool") {
            return Err(AdmissionParseError::new(
                "covers",
                format!("unknown field `covers.{}`", m.name),
            ));
        }
    }
    let crates = req_str_vec(node, "crates")?;
    let groups = req_str_vec(node, "groups")?;
    let tool = match node
        .get("tool")
        .ok_or_else(|| AdmissionParseError::new("covers.tool", "missing required `covers.tool`"))?
    {
        v if v.is_null() => None,
        v if v.as_str().is_some() => {
            let name = v.as_str().unwrap_or_default();
            if name.trim().is_empty() {
                return Err(AdmissionParseError::new("covers.tool", "`covers.tool` must be non-empty"));
            }
            Some(ToolSpec {
                name: name.to_owned(),
                rev: None,
                binary_hash: None,
            })
        }
        v if v.as_object().is_some() => {
            let obj = v.as_object().unwrap_or(&[]);
            for m in obj {
                if !matches!(m.name.as_str(), "name" | "rev" | "binary_hash") {
                    return Err(AdmissionParseError::new(
                        "covers.tool",
                        format!("unknown field `covers.tool.{}`", m.name),
                    ));
                }
            }
            let name = req_str(v, "name")?;
            let rev = opt_nullable_str(v, "rev")?;
            let binary_hash = opt_nullable_str(v, "binary_hash")?;
            Some(ToolSpec {
                name,
                rev,
                binary_hash,
            })
        }
        _ => {
            return Err(AdmissionParseError::new(
                "covers.tool",
                "`covers.tool` must be null, tool name string, or `{name, rev, binary_hash}` object",
            ));
        }
    };
    Ok(AdmissionCovers { crates, groups, tool })
}

fn decode_measured_vec(node: &Json<'_>) -> Result<Vec<MeasuredEntry>, AdmissionParseError> {
    let arr = node
        .as_array()
        .ok_or_else(|| AdmissionParseError::new("measured", "`measured` must be JSON array"))?;
    let mut out = Vec::with_capacity(arr.len());
    for (idx, item) in arr.iter().enumerate() {
        let ctx = format!("measured[{idx}]");
        let obj = item
            .as_object()
            .ok_or_else(|| AdmissionParseError::new(&ctx, "measured item must be object"))?;
        for m in obj {
            if !matches!(m.name.as_str(), "name" | "value" | "unit" | "threshold" | "relation") {
                return Err(AdmissionParseError::new(
                    &ctx,
                    format!("unknown field `{}` in measured entry", m.name),
                ));
            }
        }
        let name = req_str(item, "name")?;
        let value = match item.get("value") {
            None => None,
            Some(v) if v.is_null() => None,
            Some(v) => Some(decode_measure_num(v, &format!("{ctx}.value"))?),
        };
        let unit = item
            .get("unit")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_owned();
        let threshold_node = item
            .get("threshold")
            .ok_or_else(|| AdmissionParseError::new(&ctx, "missing `threshold`"))?;
        let threshold = decode_measure_num(threshold_node, &format!("{ctx}.threshold"))?;
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

fn decode_measure_num(node: &Json<'_>, ctx: &str) -> Result<MeasureNum, AdmissionParseError> {
    if let Ok(n) = node.as_int53() {
        return Ok(MeasureNum::Int(n));
    }
    let (_lex, x) = node
        .as_f64_lexeme()
        .map_err(|e| AdmissionParseError::new(ctx, format!("expected JSON number: {e}")))?;
    if !x.is_finite() {
        return Err(AdmissionParseError::new(ctx, "number must be finite"));
    }
    Ok(MeasureNum::Float(x))
}

fn decode_controls_vec(node: &Json<'_>) -> Result<Vec<ControlResult>, AdmissionParseError> {
    let arr = node
        .as_array()
        .ok_or_else(|| AdmissionParseError::new("controls", "`controls` must be JSON array"))?;
    let mut out = Vec::with_capacity(arr.len());
    for (idx, item) in arr.iter().enumerate() {
        let ctx = format!("controls[{idx}]");
        let obj = item
            .as_object()
            .ok_or_else(|| AdmissionParseError::new(&ctx, "control item must be object"))?;
        for m in obj {
            if !matches!(m.name.as_str(), "id" | "caught") {
                return Err(AdmissionParseError::new(
                    &ctx,
                    format!("unknown field `{}` in control entry", m.name),
                ));
            }
        }
        let id = req_str(item, "id")?;
        let caught = item
            .get("caught")
            .and_then(|v| v.as_bool())
            .ok_or_else(|| AdmissionParseError::new(&ctx, "missing boolean `caught`"))?;
        out.push(ControlResult { id, caught });
    }
    Ok(out)
}

fn decode_facts_vec(node: &Json<'_>) -> Result<Vec<FactReread>, AdmissionParseError> {
    let arr = node
        .as_array()
        .ok_or_else(|| AdmissionParseError::new("facts_reread", "`facts_reread` must be JSON array"))?;
    let mut out = Vec::with_capacity(arr.len());
    for (idx, item) in arr.iter().enumerate() {
        let ctx = format!("facts_reread[{idx}]");
        let obj = item
            .as_object()
            .ok_or_else(|| AdmissionParseError::new(&ctx, "facts_reread item must be object"))?;
        for m in obj {
            if !matches!(m.name.as_str(), "fact" | "path" | "value") {
                return Err(AdmissionParseError::new(
                    &ctx,
                    format!("unknown field `{}` in facts_reread entry", m.name),
                ));
            }
        }
        out.push(FactReread {
            fact: req_str(item, "fact")?,
            path: req_str(item, "path")?,
            value: req_str(item, "value")?,
        });
    }
    Ok(out)
}

fn decode_host(node: &Json<'_>) -> Result<HostInfo, AdmissionParseError> {
    let obj = node
        .as_object()
        .ok_or_else(|| AdmissionParseError::new("host", "`host` must be JSON object"))?;
    for m in obj {
        if !matches!(m.name.as_str(), "class" | "fingerprint") {
            return Err(AdmissionParseError::new(
                "host",
                format!("unknown field `host.{}`", m.name),
            ));
        }
    }
    let class = req_str(node, "class")?;
    if !matches!(class.as_str(), "trj" | "m5-ultra") {
        return Err(AdmissionParseError::new(
            "host.class",
            format!("invalid host class `{class}` (expected `trj` or `m5-ultra`)"),
        ));
    }
    let fingerprint = opt_nullable_str(node, "fingerprint")?;
    if let Some(fp) = &fingerprint
        && !is_lower_hex(fp, 64)
    {
        return Err(AdmissionParseError::new(
            "host.fingerprint",
            format!("host.fingerprint `{fp}` must be 64 lower-case hex digits"),
        ));
    }
    Ok(HostInfo { class, fingerprint })
}

fn decode_recorded(node: &Json<'_>) -> Result<RecordedBy, AdmissionParseError> {
    let obj = node
        .as_object()
        .ok_or_else(|| AdmissionParseError::new("recorded", "`recorded` must be JSON object"))?;
    for m in obj {
        if !matches!(m.name.as_str(), "date" | "session" | "implemented_by") {
            return Err(AdmissionParseError::new(
                "recorded",
                format!("unknown field `recorded.{}`", m.name),
            ));
        }
    }
    Ok(RecordedBy {
        date: req_str(node, "date")?,
        session: req_str(node, "session")?,
        implemented_by: req_str(node, "implemented_by")?,
    })
}

fn req_str(obj: &Json<'_>, key: &str) -> Result<String, AdmissionParseError> {
    obj.get(key)
        .and_then(|v| v.as_str())
        .map(ToOwned::to_owned)
        .ok_or_else(|| AdmissionParseError::new(key, format!("missing or non-string `{key}`")))
}

fn opt_nullable_str(obj: &Json<'_>, key: &str) -> Result<Option<String>, AdmissionParseError> {
    match obj.get(key) {
        None => Ok(None),
        Some(v) if v.is_null() => Ok(None),
        Some(v) => v
            .as_str()
            .map(|s| Some(s.to_owned()))
            .ok_or_else(|| AdmissionParseError::new(key, format!("`{key}` must be string or null"))),
    }
}

fn req_str_vec(obj: &Json<'_>, key: &str) -> Result<Vec<String>, AdmissionParseError> {
    let arr = obj
        .get(key)
        .and_then(|v| v.as_array())
        .ok_or_else(|| AdmissionParseError::new(key, format!("missing or non-array `{key}`")))?;
    arr.iter()
        .map(|it| {
            it.as_str()
                .map(ToOwned::to_owned)
                .ok_or_else(|| AdmissionParseError::new(key, format!("`{key}` element must be string")))
        })
        .collect()
}

// ============================================================================
// Internal helpers (strictly read-only git / hashing / path utilities)
// ============================================================================

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

fn workspace_lockfile_rel(ws_name: &str) -> String {
    match WORKSPACES.iter().find(|&&(name, _)| name == ws_name) {
        Some(&(_, ".")) | None => "Cargo.lock".to_owned(),
        Some(&(_, rel)) => format!("{rel}/Cargo.lock"),
    }
}

fn parse_cargo_lock_package_names(bytes: &[u8]) -> BTreeSet<String> {
    let mut names = BTreeSet::new();
    let Ok(tbl) = toml::parse(bytes) else {
        return names;
    };
    let Some(pkgs) = tbl.get("package").and_then(TomlValue::as_array) else {
        return names;
    };
    for sp in pkgs {
        if let Some(name) = sp
            .value
            .as_table()
            .and_then(|t| t.get("name"))
            .and_then(TomlValue::as_str)
        {
            names.insert(name.to_owned());
        }
    }
    names
}

fn git_commit_exists_ro(repo_root: &Path, rev: &str) -> bool {
    let spec = format!("{rev}^{{commit}}");
    Command::new("git")
        .arg("--no-optional-locks")
        .arg("-C")
        .arg(repo_root)
        .args(["cat-file", "-e", &spec])
        .output()
        .is_ok_and(|o| o.status.success())
}

fn git_show_ro(repo_root: &Path, rev: &str, rel_path: &str) -> Result<Vec<u8>, String> {
    let spec = format!("{rev}:{rel_path}");
    let out = Command::new("git")
        .arg("--no-optional-locks")
        .arg("-C")
        .arg(repo_root)
        .args(["show", &spec])
        .output()
        .map_err(|e| e.to_string())?;
    if !out.status.success() {
        return Err(String::from_utf8_lossy(&out.stderr).trim().to_owned());
    }
    Ok(out.stdout)
}

fn blake3_file(path: &Path) -> Result<String, String> {
    let mut f = File::open(path).map_err(|e| e.to_string())?;
    let mut hasher = fs_blake3::Blake3::new();
    let mut buf = [0u8; 65_536];
    loop {
        let n = f.read(&mut buf).map_err(|e| e.to_string())?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }
    Ok(hasher.finalize().to_hex())
}

fn rel_posix(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

// ============================================================================
// Unit tests (`cargo test -p xtask stack::admission`; synthetic repositories)
// ============================================================================

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicU64, Ordering};

    use super::*;
    use crate::stack::verify;

    fn fresh_dir(tag: &str) -> PathBuf {
        static CTR: AtomicU64 = AtomicU64::new(0);
        let dir = std::env::temp_dir().join(format!(
            "frob_xtask_admission_{tag}_{}_{}",
            std::process::id(),
            CTR.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(&dir).expect("mkdir admission fixture");
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

    struct AdmissionFixture {
        base: PathBuf,
        repo: PathBuf,
        sib_rev: String,
        repo_rev: String,
        receipt_hash: String,
        output_hash: String,
    }

    fn build_fixture(tag: &str) -> AdmissionFixture {
        let base = fresh_dir(tag);
        let sib = base.join("frankensim");
        fs::create_dir_all(sib.join("crates/fs-mbd/src")).expect("mkdir sib");
        git_fixture(&sib, &["init", "-q", "-b", "main"]);
        for name in ["fs-ga", "fs-mbd", "fs-contact", "fs-time", "fs-query", "fs-uq", "fs-mesh"] {
            let cdir = sib.join("crates").join(name);
            fs::create_dir_all(cdir.join("src")).expect("crate src");
            fs::write(
                cdir.join("Cargo.toml"),
                format!("[package]\nname = \"{name}\"\nversion = \"0.1.0\"\nedition = \"2024\"\n"),
            )
            .expect("crate manifest");
            fs::write(cdir.join("src/lib.rs"), "pub const OK: u32 = 1;\n").expect("crate lib");
        }
        git_fixture(&sib, &["add", "."]);
        git_fixture(&sib, &["commit", "-q", "-m", "init sib"]);
        let sib_rev = git_fixture(&sib, &["rev-parse", "HEAD"]);

        let ft_sib = base.join("frankentorch");
        fs::create_dir_all(ft_sib.join("crates/ft-kernel-metal/src")).expect("ft dir");
        fs::create_dir_all(ft_sib.join("crates/ft-nn/src")).expect("ft-nn dir");
        git_fixture(&ft_sib, &["init", "-q", "-b", "main"]);
        for name in ["ft-kernel-metal", "ft-nn"] {
            let cdir = ft_sib.join("crates").join(name);
            fs::write(
                cdir.join("Cargo.toml"),
                format!("[package]\nname = \"{name}\"\nversion = \"0.1.0\"\nedition = \"2024\"\n"),
            )
            .expect("ft manifest");
            fs::write(cdir.join("src/lib.rs"), "pub const OK: u32 = 1;\n").expect("ft lib");
        }
        git_fixture(&ft_sib, &["add", "."]);
        git_fixture(&ft_sib, &["commit", "-q", "-m", "init ft"]);
        let ft_rev = git_fixture(&ft_sib, &["rev-parse", "HEAD"]);

        let repo = base.join("repo");
        fs::create_dir_all(repo.join("crates/frob-fsim/src")).expect("mkdir repo");
        git_fixture(&repo, &["init", "-q", "-b", "main"]);
        fs::write(
            repo.join("rust-toolchain.toml"),
            "[toolchain]\nchannel = \"nightly-2026-08-31\"\n",
        )
        .expect("toolchain");
        fs::create_dir_all(repo.join(".cargo")).expect(".cargo");
        fs::write(
            repo.join(".cargo/config.toml"),
            "[env]\nFRANKENSIM_DEPGRAPH_SALT = \"synthetic-v1\"\n",
        )
        .expect("config");
        fs::write(
            repo.join("Cargo.toml"),
            "[workspace]\nresolver = \"3\"\nmembers = [\"crates/frob-fsim\"]\n",
        )
        .expect("root Cargo.toml");
        fs::write(
            repo.join("crates/frob-fsim/Cargo.toml"),
            "[package]\nname = \"frob-fsim\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n\
[features]\ndefault = [\"referee\"]\nreferee = []\ngeometry = []\nledger = []\n\
stats = [\"dep:fs-uq\"]\nstructure = [\"dep:fs-mesh\"]\n\n\
[dependencies]\n\
fs-ga = { path = \"../../../frankensim/crates/fs-ga\" }\n\
fs-mbd = { path = \"../../../frankensim/crates/fs-mbd\" }\n\
fs-contact = { path = \"../../../frankensim/crates/fs-contact\" }\n\
fs-time = { path = \"../../../frankensim/crates/fs-time\" }\n\
fs-query = { path = \"../../../frankensim/crates/fs-query\" }\n\
fs-uq = { path = \"../../../frankensim/crates/fs-uq\", optional = true }\n\
fs-mesh = { path = \"../../../frankensim/crates/fs-mesh\", optional = true }\n",
        )
        .expect("frob-fsim manifest");
        fs::write(repo.join("crates/frob-fsim/src/lib.rs"), "pub const V: u32 = fs_ga::OK;\n")
            .expect("frob-fsim lib");
        fs::write(
            repo.join("Cargo.lock"),
            "# This file is automatically @generated by Cargo.\nversion = 4\n\n\
[[package]]\nname = \"frob-fsim\"\nversion = \"0.1.0\"\ndependencies = [\n \
\"fs-contact\",\n \"fs-ga\",\n \"fs-mbd\",\n \"fs-mesh\",\n \"fs-query\",\n \"fs-time\",\n \"fs-uq\",\n]\n\n\
[[package]]\nname = \"fs-contact\"\nversion = \"0.1.0\"\n\n\
[[package]]\nname = \"fs-ga\"\nversion = \"0.1.0\"\n\n\
[[package]]\nname = \"fs-mbd\"\nversion = \"0.1.0\"\n\n\
[[package]]\nname = \"fs-mesh\"\nversion = \"0.1.0\"\n\n\
[[package]]\nname = \"fs-query\"\nversion = \"0.1.0\"\n\n\
[[package]]\nname = \"fs-time\"\nversion = \"0.1.0\"\n\n\
[[package]]\nname = \"fs-uq\"\nversion = \"0.1.0\"\n",
        )
        .expect("Cargo.lock");
        fs::create_dir_all(repo.join("registries")).expect("registries");
        fs::write(repo.join("registries/lifts.toml"), "schema = \"frob.lifts/1\"\n")
            .expect("lifts");
        fs::write(
            repo.join("franken-stack.lock"),
            format!(
                "toolchain = \"nightly-2026-08-31\"\n\n\
[[foundation]]\nid = \"FD-FSIM\"\nname = \"frankensim\"\nposture = \"consume-path\"\n\
path = \"../frankensim\"\ngit_rev = \"{sib_rev}\"\nfeatures = [\"referee\", \"geometry\", \"ledger\"]\n\
build_env = {{ FRANKENSIM_DEPGRAPH_SALT = \"synthetic-v1\" }}\n\
confined_to = [\"frob-fsim\"]\nadmission_gate = \"INT-FSIM-01..18\"\nfallback = \"KILL-FSIM\"\n\n\
[[foundation]]\nid = \"FD-FT\"\nname = \"frankentorch\"\nposture = \"consume\"\n\
path = \"../frankentorch\"\ngit_rev = \"{ft_rev}\"\nversion = \"=0.1.3\"\n\
confined_to = [\"frob-learn\"]\nadmission_gate = \"INT-FT-01..02\"\nfallback = \"CPU kernels\"\n"
            ),
        )
        .expect("stack lock");

        git_fixture(&repo, &["add", "."]);
        git_fixture(&repo, &["commit", "-q", "-m", "init repo"]);
        let repo_rev = git_fixture(&repo, &["rev-parse", "HEAD"]);

        let out = verify::run_write_receipt(&repo, &verify::VerifyOptions::default())
            .expect("write initial receipt");
        let receipt_hash = match out.facts.get("wrote_receipt") {
            Some(crate::report::FactValue::Str(s)) => s.clone(),
            other => panic!("expected wrote_receipt, got {other:?}"),
        };

        let output_bytes = b"Checking frob-fsim v0.1.0\nFinished dev profile in 1.2s\n";
        let output_hash = fs_blake3::hash_bytes(output_bytes).to_hex();
        fs::create_dir_all(repo.join("receipts/outputs")).expect("outputs dir");
        fs::write(
            repo.join("receipts/outputs").join(format!("{output_hash}.txt")),
            output_bytes,
        )
        .expect("write output blob");

        AdmissionFixture {
            base,
            repo,
            sib_rev,
            repo_rev,
            receipt_hash,
            output_hash,
        }
    }

    fn valid_fsim01_record(fx: &AdmissionFixture) -> AdmissionRecord {
        AdmissionRecord {
            schema: ADMISSION_SCHEMA.to_owned(),
            gate: "INT-FSIM-01".to_owned(),
            foundation: "FD-FSIM".to_owned(),
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
            repo_rev: fx.repo_rev.clone(),
            workspace: "core".to_owned(),
            stack_receipt: format!("sr:{}", fx.receipt_hash),
            sibling_rev: Some(fx.sib_rev.clone()),
            toolchain: "nightly-2026-08-31".to_owned(),
            harness_receipt: Some("a".repeat(64)),
            command: vec![
                "cargo".to_owned(),
                "check".to_owned(),
                "--all-targets".to_owned(),
                "-p".to_owned(),
                "frob-fsim".to_owned(),
            ],
            exit: 0,
            output_hash: fx.output_hash.clone(),
            output_excerpt: "Finished dev profile in 1.2s\n".to_owned(),
            measured: vec![MeasuredEntry {
                name: "cold_check_s".to_owned(),
                value: Some(MeasureNum::Int(42)),
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
            status: AdmissionStatus::Pass,
            host: HostInfo {
                class: "m5-ultra".to_owned(),
                fingerprint: None,
            },
            recorded: RecordedBy {
                date: "2026-11-24T22:00:00Z".to_owned(),
                session: "session:reviewer".to_owned(),
                implemented_by: "session:author".to_owned(),
            },
        }
    }

    fn store_record(repo: &Path, rec: &AdmissionRecord) -> StoredAdmission {
        let canon = rec.to_canonical_bytes().expect("canonicalize admission");
        let hash = fs_blake3::hash_bytes(canon.as_bytes()).to_hex();
        let dir = repo.join("receipts/admissions").join(&rec.gate);
        fs::create_dir_all(&dir).expect("mkdir gate dir");
        let path = dir.join(format!("{hash}.json"));
        fs::write(&path, canon.as_bytes()).expect("write admission record");
        load_record_file(repo, &path).expect("reload stored admission")
    }

    #[test]
    fn record_roundtrip_canonical() {
        let fx = build_fixture("roundtrip");
        let rec = valid_fsim01_record(&fx);
        let stored = store_record(&fx.repo, &rec);
        assert_eq!(stored.stem, stored.hash);
        let bytes1 = fs::read(&stored.abs_path).expect("read stored");
        let parsed = AdmissionRecord::from_json_bytes(&bytes1).expect("parse");
        assert_eq!(parsed, rec);
        let bytes2 = parsed.to_canonical_bytes().expect("reserialize");
        assert_eq!(bytes1.as_slice(), bytes2.as_bytes());

        let stack_lock = lock::load(&fx.repo.join("franken-stack.lock")).expect("lock");
        let findings = validate(&fx.repo, &stored, &stack_lock);
        assert!(findings.is_empty(), "expected valid record, got {findings:?}");
    }

    #[test]
    fn rule1_lockfile_lacks_foundation() {
        let fx = build_fixture("rule1");
        let mut rec = valid_fsim01_record(&fx);
        rec.covers.crates = vec!["fs-nonexistent-crate".to_owned()];
        let stored = store_record(&fx.repo, &rec);
        let stack_lock = lock::load(&fx.repo.join("franken-stack.lock")).expect("lock");
        let findings = validate(&fx.repo, &stored, &stack_lock);
        assert!(
            findings.iter().any(|f| f.rule == RULE_ADMISSION_RECORD && f.kind == "lockfile-lacks-foundation"),
            "expected lockfile-lacks-foundation, got {findings:?}"
        );
    }

    #[test]
    fn rule2_receipt_rev_mismatch() {
        let fx = build_fixture("rule2");
        let mut rec = valid_fsim01_record(&fx);
        rec.sibling_rev = Some("b".repeat(40));
        let stored = store_record(&fx.repo, &rec);
        let stack_lock = lock::load(&fx.repo.join("franken-stack.lock")).expect("lock");
        let findings = validate(&fx.repo, &stored, &stack_lock);
        assert!(
            findings.iter().any(|f| f.rule == RULE_ADMISSION_RECORD && f.kind == "receipt-missing"),
            "expected receipt-missing, got {findings:?}"
        );
    }

    #[test]
    fn rule3_output_missing_refused() {
        let fx = build_fixture("rule3");
        let stack_lock = lock::load(&fx.repo.join("franken-stack.lock")).expect("lock");

        // Subcase 1: missing output file.
        let mut rec1 = valid_fsim01_record(&fx);
        rec1.output_hash = "c".repeat(64);
        let s1 = store_record(&fx.repo, &rec1);
        let f1 = validate(&fx.repo, &s1, &stack_lock);
        assert!(f1.iter().any(|f| f.kind == "output-missing"), "got {f1:?}");

        // Subcase 2: existing output file whose bytes do not hash to its name.
        let bad_hash = "d".repeat(64);
        fs::write(
            fx.repo.join("receipts/outputs").join(format!("{bad_hash}.txt")),
            b"tampered output",
        )
        .expect("write bad output");
        let mut rec2 = valid_fsim01_record(&fx);
        rec2.output_hash = bad_hash;
        let s2 = store_record(&fx.repo, &rec2);
        let f2 = validate(&fx.repo, &s2, &stack_lock);
        assert!(f2.iter().any(|f| f.kind == "output-missing"), "got {f2:?}");
    }

    #[test]
    fn rule4_pass_on_dirty_refused() {
        let fx = build_fixture("rule4");
        fs::write(
            fx.base.join("frankensim/crates/fs-ga/src/lib.rs"),
            "pub const OK: u32 = 99;\n",
        )
        .expect("dirty sibling");
        let out = verify::run_write_receipt(&fx.repo, &verify::VerifyOptions::default())
            .expect("write dirty receipt");
        let dirty_hash = match out.facts.get("wrote_receipt") {
            Some(crate::report::FactValue::Str(s)) => s.clone(),
            other => panic!("expected dirty wrote_receipt, got {other:?}"),
        };
        let mut rec = valid_fsim01_record(&fx);
        rec.stack_receipt = format!("sr:{dirty_hash}");
        let stored = store_record(&fx.repo, &rec);
        let stack_lock = lock::load(&fx.repo.join("franken-stack.lock")).expect("lock");
        let findings = validate(&fx.repo, &stored, &stack_lock);
        assert!(
            findings.iter().any(|f| f.kind == "pass-on-dirty"),
            "expected pass-on-dirty, got {findings:?}"
        );
    }

    #[test]
    fn rule5_pass_without_control_refused() {
        let fx = build_fixture("rule5");
        let stack_lock = lock::load(&fx.repo.join("franken-stack.lock")).expect("lock");

        let mut rec1 = valid_fsim01_record(&fx);
        rec1.controls.clear();
        let s1 = store_record(&fx.repo, &rec1);
        assert!(validate(&fx.repo, &s1, &stack_lock).iter().any(|f| f.kind == "pass-without-control"));

        let mut rec2 = valid_fsim01_record(&fx);
        rec2.controls.push(ControlResult { id: "NC-MISSED".to_owned(), caught: false });
        let s2 = store_record(&fx.repo, &rec2);
        assert!(validate(&fx.repo, &s2, &stack_lock).iter().any(|f| f.kind == "pass-without-control"));
    }

    #[test]
    fn rule6_self_certified_refused() {
        let fx = build_fixture("rule6");
        let stack_lock = lock::load(&fx.repo.join("franken-stack.lock")).expect("lock");
        let mut rec = valid_fsim01_record(&fx);
        rec.recorded.session = "session:same".to_owned();
        rec.recorded.implemented_by = "session:same".to_owned();
        let stored = store_record(&fx.repo, &rec);
        assert!(
            validate(&fx.repo, &stored, &stack_lock)
                .iter()
                .any(|f| f.kind == "self-certified")
        );
    }

    #[test]
    fn rule7_tool_unpinned_refused() {
        let fx = build_fixture("rule7");
        let stack_lock = lock::load(&fx.repo.join("franken-stack.lock")).expect("lock");

        // Bare string `covers.tool = "fln"` (no rev, no binary_hash) -> tool-unpinned.
        let mut rec_bare = valid_fsim01_record(&fx);
        rec_bare.gate = "INT-FLN-01".to_owned();
        rec_bare.foundation = "FD-FLN".to_owned();
        rec_bare.covers = AdmissionCovers {
            crates: Vec::new(),
            groups: Vec::new(),
            tool: Some(ToolSpec { name: "fln".to_owned(), rev: None, binary_hash: None }),
        };
        rec_bare.sibling_rev = None;
        let s_bare = store_record(&fx.repo, &rec_bare);
        assert!(validate(&fx.repo, &s_bare, &stack_lock).iter().any(|f| f.kind == "tool-unpinned"));

        // Has rev, missing binary_hash -> tool-unpinned.
        let mut rec_no_bin = rec_bare.clone();
        rec_no_bin.covers.tool = Some(ToolSpec {
            name: "fln".to_owned(),
            rev: Some("a".repeat(40)),
            binary_hash: None,
        });
        let s_nobin = store_record(&fx.repo, &rec_no_bin);
        assert!(validate(&fx.repo, &s_nobin, &stack_lock).iter().any(|f| f.kind == "tool-unpinned"));

        // Both rev and binary_hash -> valid (zero findings).
        let mut rec_ok = rec_bare;
        rec_ok.covers.tool = Some(ToolSpec {
            name: "fln".to_owned(),
            rev: Some("a".repeat(40)),
            binary_hash: Some("b".repeat(64)),
        });
        let s_ok = store_record(&fx.repo, &rec_ok);
        let findings_ok = validate(&fx.repo, &s_ok, &stack_lock);
        assert!(findings_ok.is_empty(), "expected pinned tool valid, got {findings_ok:?}");
    }

    #[test]
    fn rule8_target_as_measurement_refused() {
        let fx = build_fixture("rule8");
        let stack_lock = lock::load(&fx.repo.join("franken-stack.lock")).expect("lock");

        // Subcase 1: `value` omitted from measured entry -> measured-missing.
        let rec = valid_fsim01_record(&fx);
        let raw_json = rec.to_canonical_bytes().expect("canon").as_str().to_owned();
        let no_val = raw_json.replace(",\"value\":42", "");
        let parsed = AdmissionRecord::from_json_bytes(no_val.as_bytes()).expect("parses with missing value");
        assert!(parsed.measured[0].value.is_none());
        let s = store_record(&fx.repo, &parsed);
        let findings = validate(&fx.repo, &s, &stack_lock);
        assert!(
            findings.iter().any(|f| f.kind == "measured-missing"),
            "expected measured-missing, got {findings:?}"
        );

        // Subcase 2 (Review Notes 2 & 4): first-use record under INT-FSIM-01 does NOT require cold_check_s.
        let mut first_use = valid_fsim01_record(&fx);
        first_use.covers.crates = vec!["fs-ga".to_owned()];
        first_use.measured = vec![MeasuredEntry {
            name: "ka_rotor_err".to_owned(),
            value: Some(MeasureNum::Float(1e-15)),
            unit: "rad".to_owned(),
            threshold: MeasureNum::Float(1e-12),
            relation: MeasureRelation::Le,
        }];
        let s_fu = store_record(&fx.repo, &first_use);
        let f_fu = validate(&fx.repo, &s_fu, &stack_lock);
        assert!(f_fu.is_empty(), "first-use record should validate cleanly: {f_fu:?}");

        // Subcase 3 (Review Note 5): rowless third-party gate ADMIT-CAP-STD with no thresholds.
        let mut third_party = valid_fsim01_record(&fx);
        third_party.gate = "ADMIT-CAP-STD".to_owned();
        third_party.foundation = "FD-FSIM".to_owned();
        third_party.covers.crates = vec!["fs-ga".to_owned()];
        third_party.measured.clear();
        let s_tp = store_record(&fx.repo, &third_party);
        let f_tp = validate(&fx.repo, &s_tp, &stack_lock);
        assert!(f_tp.is_empty(), "ADMIT-CAP-STD with no thresholds should validate: {f_tp:?}");
    }

    #[test]
    fn harness_receipt_key_required() {
        let fx = build_fixture("harness_key");
        let rec = valid_fsim01_record(&fx);
        let raw = rec.to_canonical_bytes().expect("canon").as_str().to_owned();

        // Missing key -> parse error.
        let missing = raw.replace(&format!("\"harness_receipt\":\"{}\",", "a".repeat(64)), "");
        assert!(AdmissionRecord::from_json_bytes(missing.as_bytes()).is_err());

        // `null` -> parses.
        let null_val = raw.replace(&format!("\"{}\"", "a".repeat(64)), "null");
        let p_null = AdmissionRecord::from_json_bytes(null_val.as_bytes()).expect("null parses");
        assert_eq!(p_null.harness_receipt, None);

        // 64-hex -> parses.
        let p_hex = AdmissionRecord::from_json_bytes(raw.as_bytes()).expect("64-hex parses");
        assert_eq!(p_hex.harness_receipt.as_deref(), Some("a".repeat(64).as_str()));

        // Invalid string -> parse error.
        let bad = raw.replace(&format!("\"{}\"", "a".repeat(64)), "\"not-hex\"");
        assert!(AdmissionRecord::from_json_bytes(bad.as_bytes()).is_err());
    }

    #[test]
    fn commit_unreachable_is_a_finding() {
        let fx = build_fixture("unreachable");
        let stack_lock = lock::load(&fx.repo.join("franken-stack.lock")).expect("lock");
        let mut rec = valid_fsim01_record(&fx);
        rec.repo_rev = "9".repeat(40);
        let stored = store_record(&fx.repo, &rec);
        let findings = validate(&fx.repo, &stored, &stack_lock);
        assert!(
            findings.iter().any(|f| f.kind == "commit-unreachable"),
            "expected commit-unreachable, got {findings:?}"
        );
        assert!(!admitted_gates(&fx.repo, &stack_lock).contains(&GateRef::parse("INT-FSIM-01", "t").expect("g")));
    }

    #[test]
    fn admitted_gates_only_validating_passes() {
        let fx = build_fixture("admitted_only");
        let stack_lock = lock::load(&fx.repo.join("franken-stack.lock")).expect("lock");

        // 1. Valid passing INT-FSIM-01.
        let pass_rec = valid_fsim01_record(&fx);
        let _ = store_record(&fx.repo, &pass_rec);

        // 2. Newer FAILING first-use record under INT-FSIM-01 must NOT un-admit INT-FSIM-01 (Review Notes 2 & 4).
        let mut fu_fail = valid_fsim01_record(&fx);
        fu_fail.covers.crates = vec!["fs-ga".to_owned()];
        fu_fail.status = AdmissionStatus::Fail;
        fu_fail.recorded.date = "2026-11-25T00:00:00Z".to_owned();
        let _ = store_record(&fx.repo, &fu_fail);

        // 3. Failing INT-FSIM-02.
        let mut fail_rec = valid_fsim01_record(&fx);
        fail_rec.gate = "INT-FSIM-02".to_owned();
        fail_rec.covers.crates = vec!["fs-ga".to_owned()];
        fail_rec.status = AdmissionStatus::Fail;
        let _ = store_record(&fx.repo, &fail_rec);

        // 4. Passing INT-FSIM-03 with rule-4 dirty-sibling defect.
        fs::write(
            fx.base.join("frankensim/crates/fs-ga/src/lib.rs"),
            "pub const OK: u32 = 2;\n",
        )
        .expect("dirty");
        let dirty_out = verify::run_write_receipt(&fx.repo, &verify::VerifyOptions::default())
            .expect("dirty receipt");
        let dirty_h = match dirty_out.facts.get("wrote_receipt") {
            Some(crate::report::FactValue::Str(s)) => s.clone(),
            other => panic!("expected dirty hash, got {other:?}"),
        };
        let mut bad_pass = valid_fsim01_record(&fx);
        bad_pass.gate = "INT-FSIM-03".to_owned();
        bad_pass.covers.crates = vec!["fs-mbd".to_owned()];
        bad_pass.stack_receipt = format!("sr:{dirty_h}");
        let _ = store_record(&fx.repo, &bad_pass);

        let admitted = admitted_gates(&fx.repo, &stack_lock);
        assert_eq!(
            admitted,
            BTreeSet::from([GateRef::parse("INT-FSIM-01", "t").expect("g")])
        );
    }

    #[test]
    fn check_stack_ids_are_namespaced() {
        for &(kind, rule) in ADMISSION_KIND_TO_RULE {
            assert!(
                rule == RULE_ADMISSION || rule == RULE_ADMISSION_RECORD,
                "kind `{kind}` must map to stack.admission or stack.admission-record, got `{rule}`"
            );
            assert!(verify::OWNED_RULES.contains(&rule));
            assert!(
                verify::KIND_TO_RULE.iter().any(|&(k, r)| k == kind && r == rule),
                "kind `{kind}` -> `{rule}` must be registered in verify::KIND_TO_RULE"
            );
        }
    }

    #[test]
    fn never_inherited_group() {
        let fx = build_fixture("ni_group");
        // Enable `stats` in frob-fsim default features and commit.
        let mpath = fx.repo.join("crates/frob-fsim/Cargo.toml");
        let mtxt = fs::read_to_string(&mpath).expect("read");
        fs::write(&mpath, mtxt.replace("default = [\"referee\"]", "default = [\"referee\", \"stats\"]"))
            .expect("enable stats");
        git_fixture(&fx.repo, &["add", "."]);
        git_fixture(&fx.repo, &["commit", "-q", "-m", "enable stats"]);
        let _ = verify::run_write_receipt(&fx.repo, &verify::VerifyOptions::default()).expect("receipt");

        // Record covers referee, geometry, ledger — NOT stats.
        let mut rec = valid_fsim01_record(&fx);
        rec.covers.groups = vec!["referee".to_owned(), "geometry".to_owned(), "ledger".to_owned()];
        let _ = store_record(&fx.repo, &rec);

        let stack_lock = lock::load(&fx.repo.join("franken-stack.lock")).expect("lock");
        let rep = never_inherited(&fx.repo, &stack_lock, None).expect("never_inherited");
        assert!(
            rep.findings.iter().any(|f| {
                f.rule == RULE_ADMISSION && f.kind == "unadmitted-use" && f.subject.contains("stats")
            }),
            "expected unadmitted-use for stats, got {:?}",
            rep.findings
        );
    }

    #[test]
    fn never_inherited_ignores_unenabled_optional() {
        let fx = build_fixture("ni_opt");
        let stack_lock = lock::load(&fx.repo.join("franken-stack.lock")).expect("lock");
        // Cover the 5 base crates and `referee` group; `fs-mesh` / `structure` is in Cargo.lock but unenabled.
        let mut rec = valid_fsim01_record(&fx);
        rec.covers.groups = vec!["referee".to_owned()];
        let _ = store_record(&fx.repo, &rec);

        let rep_before = never_inherited(&fx.repo, &stack_lock, None).expect("before");
        assert!(
            rep_before.findings.is_empty(),
            "unenabled optional crate fs-mesh must not trigger unadmitted-use: {:?}",
            rep_before.findings
        );

        // Now enable `structure` (which activates `dep:fs-mesh`).
        let mpath = fx.repo.join("crates/frob-fsim/Cargo.toml");
        let mtxt = fs::read_to_string(&mpath).expect("read");
        fs::write(&mpath, mtxt.replace("default = [\"referee\"]", "default = [\"referee\", \"structure\"]"))
            .expect("enable structure");

        let rep_after = never_inherited(&fx.repo, &stack_lock, None).expect("after");
        assert!(
            rep_after.findings.iter().any(|f| f.kind == "unadmitted-use" && f.subject.contains("structure")),
            "expected unadmitted-use for group structure, got {:?}",
            rep_after.findings
        );
        assert!(
            rep_after.findings.iter().any(|f| f.kind == "unadmitted-use" && f.subject.contains("fs-mesh")),
            "expected unadmitted-use for crate fs-mesh, got {:?}",
            rep_after.findings
        );
    }

    #[test]
    fn never_inherited_pin_move() {
        let fx = build_fixture("ni_pin");
        let mut rec = valid_fsim01_record(&fx);
        rec.covers.groups = vec!["referee".to_owned()];
        let _ = store_record(&fx.repo, &rec);

        // Change FD-FSIM pin in franken-stack.lock without a new record.
        let lock_path = fx.repo.join("franken-stack.lock");
        let orig = fs::read_to_string(&lock_path).expect("read lock");
        let new_pin = "e".repeat(40);
        fs::write(&lock_path, orig.replace(&fx.sib_rev, &new_pin)).expect("move pin");

        let stack_lock = lock::load(&lock_path).expect("reload lock");
        let rep = never_inherited(&fx.repo, &stack_lock, None).expect("never_inherited");
        assert!(
            rep.findings.iter().any(|f| f.rule == RULE_ADMISSION && f.kind == "admission-stale"),
            "expected admission-stale after pin move, got {:?}",
            rep.findings
        );
    }

    #[test]
    fn never_inherited_does_not_widen() {
        let fx = build_fixture("ni_widen");
        // Add ft-kernel-metal (optional) and ft-nn (enabled) to workspace member so Cargo.lock
        // is --locked-valid while INT-FT-01 covers only ft-kernel-metal.
        let mpath = fx.repo.join("crates/frob-fsim/Cargo.toml");
        let mut mtxt = fs::read_to_string(&mpath).expect("read");
        mtxt.push_str(
            "ft-kernel-metal = { path = \"../../../frankentorch/crates/ft-kernel-metal\", optional = true }\n\
ft-nn = { path = \"../../../frankentorch/crates/ft-nn\" }\n",
        );
        fs::write(&mpath, mtxt).expect("add ft-nn and optional ft-kernel-metal");

        let lpath = fx.repo.join("Cargo.lock");
        let mut ltxt = fs::read_to_string(&lpath).expect("read lock");
        ltxt = ltxt.replace(
            "\"fs-uq\",\n]",
            "\"fs-uq\",\n \"ft-kernel-metal\",\n \"ft-nn\",\n]",
        );
        ltxt.push_str(
            "\n[[package]]\nname = \"ft-kernel-metal\"\nversion = \"0.1.0\"\n\n\
[[package]]\nname = \"ft-nn\"\nversion = \"0.1.0\"\n",
        );
        fs::write(&lpath, ltxt).expect("update Cargo.lock");
        git_fixture(&fx.repo, &["add", "."]);
        git_fixture(&fx.repo, &["commit", "-q", "-m", "add ft-nn"]);
        let new_repo_rev = git_fixture(&fx.repo, &["rev-parse", "HEAD"]);
        let out = verify::run_write_receipt(&fx.repo, &verify::VerifyOptions::default()).expect("receipt");
        let new_sr = match out.facts.get("wrote_receipt") {
            Some(crate::report::FactValue::Str(s)) => s.clone(),
            other => panic!("got {other:?}"),
        };
        let stack_lock = lock::load(&fx.repo.join("franken-stack.lock")).expect("lock");
        let ft_rev = stack_lock
            .foundation("FD-FT")
            .and_then(|r| r.git_rev.as_ref())
            .map(|r| r.as_str().to_owned());

        let mut ft_rec = valid_fsim01_record(&fx);
        ft_rec.gate = "INT-FT-01".to_owned();
        ft_rec.foundation = "FD-FT".to_owned();
        ft_rec.repo_rev = new_repo_rev;
        ft_rec.stack_receipt = format!("sr:{new_sr}");
        ft_rec.sibling_rev = ft_rev;
        ft_rec.covers = AdmissionCovers {
            crates: vec!["ft-kernel-metal".to_owned()],
            groups: Vec::new(),
            tool: None,
        };
        let _ = store_record(&fx.repo, &ft_rec);

        let rep = never_inherited(&fx.repo, &stack_lock, None).expect("never_inherited");
        assert!(
            rep.findings.iter().any(|f| f.kind == "unadmitted-use" && f.subject.contains("ft-nn")),
            "INT-FT-01 covering ft-kernel-metal must not widen to ft-nn: {:?}",
            rep.findings
        );
    }

    #[test]
    fn never_inherited_restricted_to_uses() {
        let fx = build_fixture("ni_restricted");
        let stack_lock = lock::load(&fx.repo.join("franken-stack.lock")).expect("lock");

        // Store two passing records covering `stats` with distinct dates.
        let mut r1 = valid_fsim01_record(&fx);
        r1.gate = "INT-FSIM-14".to_owned();
        r1.covers = AdmissionCovers { crates: Vec::new(), groups: vec!["stats".to_owned()], tool: None };
        r1.recorded.date = "2026-11-20T00:00:00Z".to_owned();
        let s1 = store_record(&fx.repo, &r1);

        let mut r2 = r1.clone();
        r2.recorded.date = "2026-11-24T00:00:00Z".to_owned();
        let s2 = store_record(&fx.repo, &r2);

        let target = UseTarget::Group {
            fd: "FD-FSIM".to_owned(),
            group: "stats".to_owned(),
        };
        let rep = never_inherited(&fx.repo, &stack_lock, Some(std::slice::from_ref(&target)))
            .expect("restricted");
        assert_eq!(rep.uses.len(), 1);
        assert!(rep.findings.is_empty(), "stats is covered: {:?}", rep.findings);
        let covering = rep.covering_records.get(&target).expect("covering");
        assert_eq!(covering.len(), 2);
        assert_eq!(covering[0].hash, s1.hash, "oldest record must come first");
        assert_eq!(covering[1].hash, s2.hash, "newest record must come second");
    }

    #[test]
    fn status_lists_every_foundation() {
        let fx = build_fixture("status_18");
        // Copy the real 18-foundation ledger into the synthetic fixture and set FD-FSIM pin to fx.sib_rev.
        let repo_root_real = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("crates")
            .parent()
            .expect("root")
            .to_path_buf();
        let mut real_lock = fs::read_to_string(repo_root_real.join("franken-stack.lock"))
            .expect("read real franken-stack.lock");
        // Keep FD-FSIM's pin aligned with our synthetic sibling while preserving all 18 rows.
        if let Some(idx) = real_lock.find("id = \"FD-FSIM\"") {
            if let Some(rev_pos) = real_lock[idx..].find("git_rev = \"") {
                let start = idx + rev_pos + "git_rev = \"".len();
                real_lock.replace_range(start..start + 40, &fx.sib_rev);
            }
        }
        fs::write(fx.repo.join("franken-stack.lock"), &real_lock).expect("write 18-row lock");

        let rec = valid_fsim01_record(&fx);
        let stored = store_record(&fx.repo, &rec);

        let st = status(&fx.repo).expect("status");
        assert_eq!(st.newest_stack_receipt.as_deref(), Some(fx.receipt_hash.as_str()));
        assert_eq!(st.foundations.len(), 18, "must list all 18 foundations");

        let fsim = st
            .foundations
            .iter()
            .find(|f| f.fd == "FD-FSIM")
            .expect("FD-FSIM present");
        let g01 = fsim
            .gates
            .iter()
            .find(|g| g.gate == "INT-FSIM-01")
            .expect("INT-FSIM-01 present");
        assert_eq!(g01.status, AdmissionStatus::Pass);
        assert_eq!(g01.record_hash.as_deref(), Some(stored.hash.as_str()));

        // All other 17 foundations have zero records -> every gate is NotRun.
        for other in st.foundations.iter().filter(|f| f.fd != "FD-FSIM") {
            for g in &other.gates {
                assert_eq!(
                    g.status,
                    AdmissionStatus::NotRun,
                    "expected NotRun for {} / {}",
                    other.fd,
                    g.gate
                );
            }
        }
    }
}
