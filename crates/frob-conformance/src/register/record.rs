#![deny(unsafe_code)]

//! Gate recorder (`N38b-19`, `ADR-GAUNT-01`, `INV-GAUNT-02`, `INV-GAUNT-05`, `GATE-GAUNT-10`).
//!
//! Converts one or more canonical hash-named Gauntlet tier receipts
//! (`frob.gauntlet.receipt/1`, `N38a-02`) into a validated `[[gate.part]]` row in
//! `registries/gates.toml`, committing byte-identical receipts (and a
//! `frob.gauntlet.receipt-set/1` manifest for multi-tier parts) under
//! `receipts/gates/<hash>.json`, updating control `last_seen` and
//! `control_history.ndjson`, appending a `frob.beadlog/1` `gate-record` line,
//! and regenerating derived status views.
//!
//! Determinism class D0 apart from `date`; zero `unsafe`; prints nothing (rule L-7).

use std::collections::BTreeSet;
use std::env;
use std::fmt;
use std::fs::{self, OpenOptions};
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use frob_core::canon::{self, CanonError, Canonical, CanonicalBytes, Value};
use frob_core::graph_id::ClaimId;
use frob_core::id::{Blake3, hash_blob};
use frob_core::json::{self, Limits};
use frob_core::stack::build_identity::{GateClass, may_cite_mode_str};
use frob_core::testkit::registry::{Control, Controls, LastSeen};

use crate::control_runner::{CONTROL_HISTORY_PATH, ControlHistoryEntry, append_history_entry};
use crate::controls::{CONTROLS_PATH, serialize_controls_file};
use crate::reasons::HarnessReason;
use crate::register::kill::render_readme_status_table;
use crate::register::row::{Claim, Recorded, Status, parse_milestone_ord};
use crate::register::validate::{
    GateRow, Part, Register, ValidationContext, Violation, is_session_implementer,
    parse_register_bytes, validate,
};
use crate::register::write;
use crate::tiers::{ControlRunOutcome, GauntletReceipt, Tier, read_and_verify_receipt};

/// Schema identifier for a multi-tier receipt set (`N38b-19`).
pub const RECEIPT_SET_SCHEMA: &str = "frob.gauntlet.receipt-set/1";
/// Schema identifier for per-bead delivery log lines (`N29a-09`, `N38b-19`).
pub const BEADLOG_SCHEMA: &str = "frob.beadlog/1";
/// Default lock acquisition timeout.
pub const DEFAULT_LOCK_TIMEOUT: Duration = Duration::from_secs(10);

/// Pure Step-7 hook context passed to every check registered in [`CHECKS`].
#[derive(Debug)]
pub struct RecordHookContext<'a> {
    pub gate_id: &'a str,
    pub gate: &'a GateRow,
    pub part: &'a Part,
    pub candidate: &'a Register,
    pub receipts: &'a [LoadedReceipt],
    pub resolved_evidence: &'a [String],
}

/// Refusal returned by a Step-7 hook registered in [`CHECKS`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordHookRefusal {
    pub reason: HarnessReason,
    pub detail: String,
    pub repair: String,
}

impl RecordHookRefusal {
    #[must_use]
    pub fn new(
        reason: HarnessReason,
        detail: impl Into<String>,
        repair: impl Into<String>,
    ) -> Self {
        Self {
            reason,
            detail: detail.into(),
            repair: repair.into(),
        }
    }
}

/// Signature of a pure Step-7 recording check registered in [`CHECKS`].
pub type RecordHook = fn(&RecordHookContext<'_>) -> Result<(), RecordHookRefusal>;

/// Global ordered hook list run in Step 7 after the `N40-01` register validator (`N38b-19`).
///
/// Downstream beads (`N38a-05`, `N38b-07`, `N21c-34`, `N32ax-03`, `N38a-36`) register their
/// pure checks here; the first refusal exits 3 with `reason=<id>` and writes nothing.
pub static CHECKS: Mutex<Vec<(&'static str, RecordHook)>> = Mutex::new(Vec::new());

/// Append a named pure check to [`CHECKS`] in registration order (deduplicating by name).
pub fn register_check(name: &'static str, hook: RecordHook) {
    let mut guard = CHECKS
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    if !guard.iter().any(|(existing, _)| *existing == name) {
        guard.push((name, hook));
    }
}

/// Snapshot currently registered Step-7 checks in registration order.
#[must_use]
pub fn snapshot_checks() -> Vec<(&'static str, RecordHook)> {
    CHECKS
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .clone()
}

/// Remove a named check from [`CHECKS`] (used by tests restoring hook state).
pub fn unregister_check(name: &str) {
    let mut guard = CHECKS
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    guard.retain(|(n, _)| *n != name);
}

/// Exclusive lock guard on `registries/gates.toml.lock` (`N38b-19`, shared with `reaffirm`).
///
/// Created via atomic `create_new(true)` storing holder session, PID, and start timestamp;
/// removed on drop only by the process that created it. A stale lock whose PID has exited
/// is reported with its holder and never silently broken.
#[derive(Debug)]
pub struct RegistryLock {
    path: PathBuf,
    owned: bool,
}

/// Lock acquisition failure naming the current or stale lock holder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegistryLockError {
    Held {
        path: PathBuf,
        holder: String,
    },
    Stale {
        path: PathBuf,
        holder: String,
        pid: u32,
    },
    Io {
        path: PathBuf,
        detail: String,
    },
}

impl fmt::Display for RegistryLockError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Held { path, holder } => write!(
                f,
                "registry lock `{}` is held after timeout by ({holder})",
                path.display()
            ),
            Self::Stale { path, holder, pid } => write!(
                f,
                "stale registry lock `{}` left by exited pid {pid} ({holder}); inspect and clear explicitly",
                path.display()
            ),
            Self::Io { path, detail } => {
                write!(
                    f,
                    "registry lock I/O error at `{}`: {detail}",
                    path.display()
                )
            }
        }
    }
}

impl std::error::Error for RegistryLockError {}

impl RegistryLock {
    /// Canonical lock path for a given `gates.toml` path (`<path>.lock`).
    #[must_use]
    pub fn lock_path_for(registry_path: &Path) -> PathBuf {
        let mut os = registry_path.as_os_str().to_os_string();
        os.push(".lock");
        PathBuf::from(os)
    }

    /// Acquire the exclusive registry lock, waiting up to `timeout` if a live holder owns it.
    pub fn acquire(
        registry_path: &Path,
        session: &str,
        timeout: Duration,
    ) -> Result<Self, RegistryLockError> {
        let lock_path = Self::lock_path_for(registry_path);
        if let Some(parent) = lock_path.parent() {
            fs::create_dir_all(parent).map_err(|e| RegistryLockError::Io {
                path: lock_path.clone(),
                detail: e.to_string(),
            })?;
        }
        let deadline = Instant::now() + timeout;
        let my_pid = std::process::id();
        let started_unix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |d| d.as_secs());

        loop {
            match OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&lock_path)
            {
                Ok(mut f) => {
                    let payload =
                        format!("pid={my_pid}\nsession={session}\nstarted_unix={started_unix}\n");
                    let _ = f.write_all(payload.as_bytes()).and_then(|()| f.sync_all());
                    return Ok(Self {
                        path: lock_path,
                        owned: true,
                    });
                }
                Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
                    let info = fs::read_to_string(&lock_path).unwrap_or_default();
                    let holder = summarize_lock_holder(&info);
                    if let Some(pid) = parse_lock_pid(&info)
                        && !is_pid_alive(pid)
                    {
                        return Err(RegistryLockError::Stale {
                            path: lock_path,
                            holder,
                            pid,
                        });
                    }
                    if Instant::now() >= deadline {
                        return Err(RegistryLockError::Held {
                            path: lock_path,
                            holder,
                        });
                    }
                    thread::sleep(Duration::from_millis(25));
                }
                Err(e) => {
                    return Err(RegistryLockError::Io {
                        path: lock_path,
                        detail: e.to_string(),
                    });
                }
            }
        }
    }

    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for RegistryLock {
    fn drop(&mut self) {
        if self.owned {
            let _ = fs::remove_file(&self.path);
            self.owned = false;
        }
    }
}

fn parse_lock_pid(info: &str) -> Option<u32> {
    info.lines()
        .find_map(|l| l.strip_prefix("pid="))
        .and_then(|s| s.trim().parse::<u32>().ok())
}

fn summarize_lock_holder(info: &str) -> String {
    let parts: Vec<&str> = info
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .collect();
    if parts.is_empty() {
        "unknown holder".to_owned()
    } else {
        parts.join(", ")
    }
}

fn is_pid_alive(pid: u32) -> bool {
    if pid == 0 {
        return false;
    }
    if pid == std::process::id() {
        return true;
    }
    if Path::new("/proc").is_dir() {
        return Path::new(&format!("/proc/{pid}")).exists();
    }
    match Command::new("kill")
        .arg("-0")
        .arg(pid.to_string())
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .output()
    {
        Ok(out) => {
            if out.status.success() {
                true
            } else {
                let err = String::from_utf8_lossy(&out.stderr).to_ascii_lowercase();
                err.contains("operation not permitted") || err.contains("not permitted")
            }
        }
        Err(_) => true,
    }
}

/// Verified receipt loaded from disk together with its raw canonical bytes and BLAKE3 hex.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadedReceipt {
    pub hash: String,
    pub raw_bytes: Vec<u8>,
    pub receipt: GauntletReceipt,
}

/// Canonical multi-tier receipt set (`frob.gauntlet.receipt-set/1`, `N38b-19`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReceiptSet {
    pub schema: String,
    pub gate: String,
    pub part: String,
    pub members: Vec<String>,
}

impl ReceiptSet {
    #[must_use]
    pub fn new(gate: impl Into<String>, part: impl Into<String>, members: Vec<String>) -> Self {
        Self {
            schema: RECEIPT_SET_SCHEMA.to_owned(),
            gate: gate.into(),
            part: part.into(),
            members,
        }
    }

    pub fn to_canonical_bytes(&self) -> Result<CanonicalBytes, CanonError> {
        canon::to_canonical(self)
    }

    pub fn digest_hex(&self) -> Result<String, CanonError> {
        let bytes = self.to_canonical_bytes()?;
        Ok(hash_blob(bytes.as_bytes()).to_hex())
    }
}

impl Canonical for ReceiptSet {
    fn write(&self, w: Value<'_>) -> Result<(), CanonError> {
        w.object(|o| {
            o.str("gate", &self.gate)?;
            o.array("members", |a| {
                for m in &self.members {
                    a.push_str(m);
                }
                Ok(())
            })?;
            o.str("part", &self.part)?;
            o.str("schema", &self.schema)?;
            Ok(())
        })
    }
}

/// One `gate-record` line of `frob.beadlog/1` (`N29a-09`, `N38b-19`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BeadLogGateRecord {
    pub schema: String,
    pub kind: String,
    pub gate: String,
    pub part: String,
    pub verdict: String,
    pub claim_ids: Vec<String>,
    pub evidence: Vec<String>,
    pub receipt: String,
    pub recorded_by: String,
    pub date: String,
}

impl Canonical for BeadLogGateRecord {
    fn write(&self, w: Value<'_>) -> Result<(), CanonError> {
        w.object(|o| {
            o.array("claim_ids", |a| {
                for c in &self.claim_ids {
                    a.push_str(c);
                }
                Ok(())
            })?;
            o.str("date", &self.date)?;
            o.array("evidence", |a| {
                for e in &self.evidence {
                    a.push_str(e);
                }
                Ok(())
            })?;
            o.str("gate", &self.gate)?;
            o.str("kind", &self.kind)?;
            o.str("part", &self.part)?;
            o.str("receipt", &self.receipt)?;
            o.str("recorded_by", &self.recorded_by)?;
            o.str("schema", &self.schema)?;
            o.str("verdict", &self.verdict)?;
            Ok(())
        })
    }
}

/// Options governing one [`record_gate`] invocation.
#[derive(Debug, Clone)]
pub struct RecordOptions {
    pub repo_root: PathBuf,
    pub registry_path: Option<PathBuf>,
    pub controls_path: Option<PathBuf>,
    pub control_history_path: Option<PathBuf>,
    pub receipts_dir: Option<PathBuf>,
    pub gate_id: String,
    pub part_milestone: Option<String>,
    pub receipt_hashes: Vec<String>,
    /// Recording session (`session:<name>`); falls back to `FROB_SESSION`.
    pub session: Option<String>,
    /// Explicit recording date (`YYYY-MM-DD`); defaults to current UTC date.
    pub date: Option<String>,
    /// Evidence item identifiers cited by this part; defaults to the cited receipt hash(es).
    pub evidence: Vec<String>,
    /// Optional delivering bead ID for appending `docs/delivery/logs/<bead>.ndjson`.
    pub bead_id: Option<String>,
    /// Optional delivering bead log files scanned for `implement` session lines.
    pub bead_logs: Vec<PathBuf>,
    /// Optional capability paths passed to `git log` when scanning `Frob-Session:` trailers.
    pub delivering_paths: Vec<PathBuf>,
    /// Optional explicit live claim IDs for Step 6 resolution.
    pub live_claims: Option<BTreeSet<String>>,
    /// Optional explicit live evidence IDs for Step 6 resolution.
    pub live_evidence: Option<BTreeSet<String>>,
    /// Tombstoned claim/evidence nodes refused in Step 6 (`evidence-unresolved`).
    pub tombstoned_nodes: BTreeSet<String>,
    /// Entry IDs retired from registration (exempt from `part-rows-dropped`).
    pub retired_entries: BTreeSet<String>,
    /// Optional README file path whose honest-status table is regenerated.
    pub readme_path: Option<PathBuf>,
    /// Lock wait timeout.
    pub lock_timeout: Duration,
    /// Optional extra per-call hooks executed after global [`CHECKS`].
    pub extra_hooks: Vec<(&'static str, RecordHook)>,
    /// Whether to skip global [`CHECKS`] (default `false`).
    pub skip_global_checks: bool,
}

impl RecordOptions {
    #[must_use]
    pub fn new(
        repo_root: impl Into<PathBuf>,
        gate_id: impl Into<String>,
        receipt_hashes: Vec<String>,
    ) -> Self {
        Self {
            repo_root: repo_root.into(),
            registry_path: None,
            controls_path: None,
            control_history_path: None,
            receipts_dir: None,
            gate_id: gate_id.into(),
            part_milestone: None,
            receipt_hashes,
            session: None,
            date: None,
            evidence: Vec::new(),
            bead_id: None,
            bead_logs: Vec::new(),
            delivering_paths: Vec::new(),
            live_claims: None,
            live_evidence: None,
            tombstoned_nodes: BTreeSet::new(),
            retired_entries: BTreeSet::new(),
            readme_path: None,
            lock_timeout: DEFAULT_LOCK_TIMEOUT,
            extra_hooks: Vec::new(),
            skip_global_checks: false,
        }
    }
}

/// Structured refusal returned when any step of [`record_gate`] declines to write (exit 3).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordRefusal {
    pub reason: String,
    pub harness_reason: Option<HarnessReason>,
    pub detail: String,
    pub repair: String,
}

impl RecordRefusal {
    #[must_use]
    pub fn harness(
        reason: HarnessReason,
        detail: impl Into<String>,
        repair: impl Into<String>,
    ) -> Self {
        Self {
            reason: reason.as_str().to_owned(),
            harness_reason: Some(reason),
            detail: detail.into(),
            repair: repair.into(),
        }
    }

    #[must_use]
    pub fn custom(
        reason: impl Into<String>,
        detail: impl Into<String>,
        repair: impl Into<String>,
    ) -> Self {
        Self {
            reason: reason.into(),
            harness_reason: None,
            detail: detail.into(),
            repair: repair.into(),
        }
    }

    /// Canonical first diagnostic line (`reason=<id> <detail>`).
    #[must_use]
    pub fn first_line(&self) -> String {
        format!("reason={} {}", self.reason, self.detail)
    }
}

impl fmt::Display for RecordRefusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "reason={} {} (repair: {})",
            self.reason, self.detail, self.repair
        )
    }
}

impl std::error::Error for RecordRefusal {}

/// Successful recording result returned by [`record_gate`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordOutput {
    pub gate_id: String,
    pub part: Part,
    pub committed_receipts: Vec<PathBuf>,
    pub not_yet_due_controls: Vec<String>,
    pub regenerated_files: Vec<PathBuf>,
    pub bead_log_path: Option<PathBuf>,
    pub idempotent_noop: bool,
}

/// Execute Steps 1–10 of the gate recorder under the exclusive registry lock (`N38b-19`).
///
/// On any refusal, no file on disk is modified (`registries/gates.toml`, `controls.toml`,
/// `control_history.ndjson`, `receipts/gates/`, and `docs/delivery/logs/` stay byte-identical).
pub fn record_gate(opts: &RecordOptions) -> Result<RecordOutput, RecordRefusal> {
    let registry_path = opts
        .registry_path
        .clone()
        .unwrap_or_else(|| opts.repo_root.join("registries/gates.toml"));
    let controls_path = opts
        .controls_path
        .clone()
        .unwrap_or_else(|| opts.repo_root.join(CONTROLS_PATH));
    let history_path = opts
        .control_history_path
        .clone()
        .unwrap_or_else(|| opts.repo_root.join(CONTROL_HISTORY_PATH));
    let receipts_dir = opts
        .receipts_dir
        .clone()
        .unwrap_or_else(|| opts.repo_root.join("target/gauntlet/receipts"));

    // Resolve recording session early so the lock file records the holder.
    let session_raw = opts
        .session
        .clone()
        .or_else(|| env::var("FROB_SESSION").ok())
        .unwrap_or_default();
    let session = session_raw.trim().to_owned();
    let lock_holder = if session.is_empty() {
        "session:unset"
    } else {
        session.as_str()
    };

    let _lock =
        RegistryLock::acquire(&registry_path, lock_holder, opts.lock_timeout).map_err(|e| {
            RecordRefusal::custom(
                "registry-lock",
                e.to_string(),
                "Wait for the holding recorder or inspect and clear a stale lock explicitly",
            )
        })?;

    // Parse current registries/gates.toml.
    let reg_bytes = fs::read(&registry_path).map_err(|e| {
        RecordRefusal::custom(
            "io-error",
            format!("failed reading `{}`: {e}", registry_path.display()),
            "Ensure registries/gates.toml exists",
        )
    })?;
    let current_reg = parse_register_bytes(&reg_bytes, &registry_path.to_string_lossy())
        .map_err(|vs| violation_to_refusal(&vs[0]))?;
    let gate = current_reg
        .get(&opts.gate_id)
        .ok_or_else(|| {
            RecordRefusal::harness(
                HarnessReason::UnknownGate,
                format!("gate `{}` does not exist in register", opts.gate_id),
                "Register the gate row in registries/gates.toml before recording",
            )
        })?
        .clone();
    if gate.is_kill() {
        return Err(RecordRefusal::custom(
            "kill-row-measurement-field",
            format!("cannot record a [[gate.part]] on kill row `{}`", gate.id),
            "Record kill-gate decisions via N40-03 decision fields",
        ));
    }

    let part_milestone = opts
        .part_milestone
        .clone()
        .or_else(|| gate.milestone.clone())
        .or_else(|| gate.parts.first().map(|p| p.milestone.clone()))
        .ok_or_else(|| {
            RecordRefusal::custom(
                "missing-part-milestone",
                format!(
                    "gate `{}` has no milestone and no --part was given",
                    gate.id
                ),
                "Pass --part <M0..M8>",
            )
        })?;

    // Round-4 Review Note 2(1): refuse recording on any `pilot:` scoped part.
    if let Some(existing_part) = gate.parts.iter().find(|p| p.milestone == part_milestone)
        && existing_part
            .scope
            .as_deref()
            .is_some_and(|s| s.starts_with("pilot:"))
    {
        return Err(RecordRefusal::custom(
            "pilot-part-not-recordable",
            format!(
                "gate `{}` part `{part_milestone}` has pilot scope `{}` and cannot be recorded here",
                gate.id,
                existing_part.scope.as_deref().unwrap_or("")
            ),
            "Record pilot verdicts strictly in registry/pilots.toml (N38a-09)",
        ));
    }

    // Step 1: Load & BLAKE3-verify every supplied receipt.
    if opts.receipt_hashes.is_empty() {
        return Err(RecordRefusal::harness(
            HarnessReason::ReceiptSetInvalid,
            "at least one --receipt <hash> is required",
            "Pass one --receipt <64-hex> per tier target",
        ));
    }
    let mut loaded = Vec::with_capacity(opts.receipt_hashes.len());
    for raw_hash in &opts.receipt_hashes {
        let hex = raw_hash
            .trim()
            .strip_prefix("b3:")
            .unwrap_or(raw_hash.trim());
        loaded.push(load_and_verify_receipt(
            &opts.repo_root,
            &receipts_dir,
            hex,
        )?);
    }

    // Validate multi-tier receipt set invariants (single commit, distinct tiers).
    let common_commit = loaded[0].receipt.commit.clone();
    let mut seen_tiers: BTreeSet<Tier> = BTreeSet::new();
    for item in &loaded {
        if item.receipt.commit != common_commit {
            return Err(RecordRefusal::harness(
                HarnessReason::ReceiptSetInvalid,
                format!(
                    "receipts span multiple commits (`{common_commit}` vs `{}`)",
                    item.receipt.commit
                ),
                "Give one receipt per tier, all from one commit",
            ));
        }
        if !seen_tiers.insert(item.receipt.tier) {
            return Err(RecordRefusal::harness(
                HarnessReason::ReceiptSetInvalid,
                format!("duplicate receipts for tier `{}`", item.receipt.tier.code()),
                "Give at most one receipt per tier target",
            ));
        }
    }
    loaded.sort_by_key(|item| item.receipt.tier);

    // Step 2: Ancestor commit, clean tree, and `stack_dev` guards (for ANY status).
    for item in &loaded {
        if item.receipt.dirty {
            return Err(RecordRefusal::harness(
                HarnessReason::ReceiptDirtyTree,
                format!(
                    "receipt `{}` was produced on a dirty working tree",
                    item.hash
                ),
                "Commit changes and re-run the tier on a clean tree",
            ));
        }
        if !git_is_ancestor(&opts.repo_root, &item.receipt.commit) {
            return Err(RecordRefusal::harness(
                HarnessReason::ReceiptCommitUnknown,
                format!(
                    "receipt `{}` commit `{}` is not an ancestor of HEAD",
                    item.hash, item.receipt.commit
                ),
                "Re-run the tier on a commit reachable from HEAD",
            ));
        }
        if item.receipt.stack_dev {
            return Err(RecordRefusal::harness(
                HarnessReason::ReceiptStackDev,
                format!("receipt `{}` carries stack_dev=true", item.hash),
                "Re-run the tier from a binary built under a clean stack receipt",
            ));
        }
    }

    // Step 3: Gate line extraction, no-claim rule, reduced-count rule, and build-identity rule.
    let mut gate_lines = Vec::with_capacity(loaded.len());
    for item in &loaded {
        let gline = item
            .receipt
            .gates
            .iter()
            .find(|g| g.id == gate.id)
            .ok_or_else(|| {
                RecordRefusal::harness(
                    HarnessReason::GateLineNotEligible,
                    format!(
                        "receipt `{}` ({}) carries no gate line for `{}`",
                        item.hash,
                        item.receipt.tier.code(),
                        gate.id
                    ),
                    "Run the tier filter that measures this gate",
                )
            })?;
        gate_lines.push((item, gline));
    }

    let (status, detail) = combine_gate_lines(&gate, &gate_lines)?;

    // Build-identity citation check applies to `pass` (a `fail` is always recorded).
    if status == Status::Pass {
        let gate_class = gate
            .claim
            .map(claim_to_gate_class)
            .unwrap_or(GateClass::KnownAnswer);
        for item in &loaded {
            if may_cite_mode_str(&item.receipt.build_identity, gate_class).is_err() {
                return Err(RecordRefusal::harness(
                    HarnessReason::DevBuildIdentity,
                    format!(
                        "gate `{}` (claim `{}`) cannot cite receipt `{}` built under `{}`",
                        gate.id,
                        gate_class.as_str(),
                        item.hash,
                        item.receipt.build_identity
                    ),
                    "Rebuild under depgraph-receipt and re-run; or record without citing as pass",
                ));
            }
        }
    }

    // Re-recording guard: refuse if any entry measured by the replaced recording was dropped.
    if let Some(existing_part) = gate.parts.iter().find(|p| p.milestone == part_milestone)
        && !existing_part.receipt.is_empty()
    {
        check_no_dropped_entries(
            &opts.repo_root,
            &existing_part.receipt,
            &loaded,
            &opts.retired_entries,
        )?;
    }

    // Load controls.toml if present.
    let loaded_controls: Option<Controls> = if controls_path.is_file() {
        let bytes = fs::read(&controls_path).map_err(|e| {
            RecordRefusal::custom(
                "io-error",
                format!("failed reading `{}`: {e}", controls_path.display()),
                "Check controls.toml path",
            )
        })?;
        Some(
            Controls::parse_bytes(&bytes, &controls_path.to_string_lossy()).map_err(|e| {
                RecordRefusal::custom("controls-parse-error", e.to_string(), "Fix controls.toml")
            })?,
        )
    } else {
        None
    };

    // Step 4: Same-run negative-control rule (applies to `pass`; not to `no-claim` or `fail`).
    let mut due_controls: Vec<String> = Vec::new();
    let mut not_yet_due_controls: Vec<String> = Vec::new();
    if status == Status::Pass {
        if gate.controls.is_empty() {
            return Err(RecordRefusal::harness(
                HarnessReason::PassWithoutControl,
                format!("gate `{}` has an empty controls list", gate.id),
                "Register the negative control first (ADR-GAUNT-03)",
            ));
        }
        let part_ord = parse_milestone_ord(&part_milestone).unwrap_or(0);
        for cid in &gate.controls {
            let is_deferred = loaded_controls
                .as_ref()
                .and_then(|c| c.get(cid))
                .and_then(|ctrl| ctrl.due.as_deref())
                .and_then(parse_milestone_ord)
                .is_some_and(|due_ord| due_ord > part_ord);
            if is_deferred {
                not_yet_due_controls.push(cid.clone());
            } else {
                due_controls.push(cid.clone());
            }
        }
        // Round-4 Review Note 1: if all listed controls are not yet due, refuse pass-without-control.
        if due_controls.is_empty() {
            return Err(RecordRefusal::harness(
                HarnessReason::PassWithoutControl,
                format!(
                    "gate `{}` part `{part_milestone}` has no due control (all listed controls are not-yet-due: {})",
                    gate.id,
                    not_yet_due_controls.join(", ")
                ),
                "Add a runnable control due at or before this milestone",
            ));
        }
        for cid in &due_controls {
            let matching: Vec<ControlRunOutcome> = loaded
                .iter()
                .flat_map(|r| r.receipt.controls.iter())
                .filter(|c| c.id == *cid && c.gate == gate.id)
                .map(|c| c.outcome)
                .collect();
            if matching.is_empty() {
                return Err(RecordRefusal::harness(
                    HarnessReason::ControlMissingFromReceipt,
                    format!(
                        "due control `{cid}` for gate `{}` is absent from the receipt(s)",
                        gate.id
                    ),
                    "Re-run the tier with negative controls enabled so the receipt records every due control",
                ));
            }
            if let Some(bad) = matching.iter().find(|&&o| o != ControlRunOutcome::Caught) {
                return Err(RecordRefusal::harness(
                    HarnessReason::ControlNotCaught,
                    format!(
                        "due control `{cid}` for gate `{}` had outcome `{}` (expected `caught`)",
                        gate.id,
                        bad.as_str()
                    ),
                    "Fix the gate's test or plant so every due control is caught in the same run",
                ));
            }
        }
    }

    // Step 5: Compute session identities and enforce INV-GAUNT-05 (`self-certified`).
    if session.is_empty() {
        return Err(RecordRefusal::harness(
            HarnessReason::SelfCertified,
            "recording session identity is empty (FROB_SESSION unset)",
            "Export FROB_SESSION=session:<name> from a fresh-context recording session",
        ));
    }
    let implemented_by = compute_implemented_by(
        &opts.repo_root,
        &common_commit,
        &opts.delivering_paths,
        &opts.bead_logs,
    );
    if status == Status::Pass && session_in_implementers(&session, &implemented_by) {
        return Err(RecordRefusal::harness(
            HarnessReason::SelfCertified,
            format!("recording session `{session}` is in implemented_by (`{implemented_by}`)"),
            "Record from a fresh-context session that did not implement the capability",
        ));
    }

    // Compute canonical receipt hash (single receipt or multi-tier receipt set).
    let (part_receipt_hash, receipt_set_bytes) = if loaded.len() == 1 {
        (loaded[0].hash.clone(), None)
    } else {
        let members: Vec<String> = loaded.iter().map(|r| r.hash.clone()).collect();
        let set = ReceiptSet::new(&gate.id, &part_milestone, members);
        let canon = set.to_canonical_bytes().map_err(|e| {
            RecordRefusal::harness(
                HarnessReason::ReceiptSetInvalid,
                format!("failed canonicalizing receipt set: {e}"),
                "Verify receipt hashes",
            )
        })?;
        let set_hash = hash_blob(canon.as_bytes()).to_hex();
        (set_hash, Some(canon.as_bytes().to_vec()))
    };

    // Build measured string strictly from receipt values (never from gate.threshold).
    let measured = build_measured_value(status, &loaded, detail.as_deref());
    let command = loaded
        .iter()
        .map(|r| format!("{} -- {}", r.receipt.tier.target_name(), r.receipt.filter))
        .collect::<Vec<_>>()
        .join("; ");
    let fingerprint = loaded
        .iter()
        .map(|r| r.receipt.fingerprint.as_str())
        .collect::<Vec<_>>()
        .join("; ");
    let seed = loaded
        .iter()
        .map(|r| r.receipt.seed.to_string())
        .collect::<Vec<_>>()
        .join("; ");
    let evidence = if opts.evidence.is_empty() {
        loaded.iter().map(|r| r.hash.clone()).collect()
    } else {
        opts.evidence.clone()
    };

    // Step 6: Resolve claim_id and every evidence item (dangling or tombstoned -> evidence-unresolved).
    if status == Status::Pass {
        let claim_id = gate.claim_id.as_deref().unwrap_or("");
        if claim_id.is_empty() || ClaimId::parse(claim_id).is_err() {
            return Err(RecordRefusal::harness(
                HarnessReason::EvidenceUnresolved,
                format!(
                    "gate `{}` claim_id `{claim_id}` is not a valid N34-11 ClaimId",
                    gate.id
                ),
                "Cite a live N34-11 ClaimId (clm:<64 hex>)",
            ));
        }
        if opts.tombstoned_nodes.contains(claim_id) {
            return Err(RecordRefusal::harness(
                HarnessReason::EvidenceUnresolved,
                format!("claim_id `{claim_id}` is tombstoned"),
                "Cite a live, non-tombstoned claim",
            ));
        }
        if let Some(ref live) = opts.live_claims
            && !live.contains(claim_id)
        {
            return Err(RecordRefusal::harness(
                HarnessReason::EvidenceUnresolved,
                format!("claim_id `{claim_id}` does not resolve in the artifact graph"),
                "Cite a live claim in the artifact graph",
            ));
        }
        for ev in &evidence {
            if opts.tombstoned_nodes.contains(ev) {
                return Err(RecordRefusal::harness(
                    HarnessReason::EvidenceUnresolved,
                    format!("evidence item `{ev}` is tombstoned"),
                    "Re-run and cite live, non-tombstoned evidence",
                ));
            }
            if let Some(ref live_ev) = opts.live_evidence
                && !live_ev.contains(ev)
            {
                return Err(RecordRefusal::harness(
                    HarnessReason::EvidenceUnresolved,
                    format!("evidence item `{ev}` does not resolve in the artifact graph"),
                    "Re-run and cite live artifact-graph evidence",
                ));
            }
        }
    }

    let date = opts.date.clone().unwrap_or_else(utc_today_iso);
    let existing_part = gate.parts.iter().find(|p| p.milestone == part_milestone);
    let candidate_part = Part {
        milestone: part_milestone.clone(),
        scope: existing_part.and_then(|p| p.scope.clone()),
        status,
        measured,
        commit: common_commit.clone(),
        command,
        receipt: part_receipt_hash.clone(),
        fingerprint,
        seed,
        evidence: evidence.clone(),
        recorded: Recorded {
            date: date.clone(),
            session: session.clone(),
            implemented_by: implemented_by.clone(),
        },
        detail,
        kill_gate: None,
        fallback: None,
        open_question: None,
    };

    // Idempotent no-op check: same receipt, status, measured, evidence, and session already committed.
    if let Some(prev) = existing_part
        && parts_match_except_date(prev, &candidate_part)
        && receipts_already_committed(
            &opts.repo_root,
            &loaded,
            &part_receipt_hash,
            receipt_set_bytes.as_deref(),
        )
    {
        return Ok(RecordOutput {
            gate_id: gate.id,
            part: prev.clone(),
            committed_receipts: committed_receipt_paths(
                &opts.repo_root,
                &loaded,
                &part_receipt_hash,
                receipt_set_bytes.is_some(),
            ),
            not_yet_due_controls,
            regenerated_files: Vec::new(),
            bead_log_path: opts.bead_id.as_ref().map(|b| {
                opts.repo_root
                    .join(format!("docs/delivery/logs/{b}.ndjson"))
            }),
            idempotent_noop: true,
        });
    }

    // Build candidate register in memory and projected controls for Step 7 validation (ZERO disk writes yet).
    let mut candidate_reg = current_reg.clone();
    if let Some(g) = candidate_reg.get_mut(&gate.id) {
        if let Some(slot) = g
            .parts
            .iter_mut()
            .find(|p| p.milestone == candidate_part.milestone)
        {
            *slot = candidate_part.clone();
        } else {
            g.parts.push(candidate_part.clone());
        }
    }

    let projected_controls = loaded_controls.as_ref().map(|ctrls| {
        if status == Status::Pass {
            let updated_rows =
                update_controls_last_seen(ctrls.all(), &due_controls, &session, &date);
            Controls::parse_str(&serialize_controls_file(&updated_rows), "controls.toml")
                .unwrap_or_else(|_| ctrls.clone())
        } else {
            ctrls.clone()
        }
    });

    // Step 7a: Run N40-01 register validator in memory (repo_root = None so disk receipt check runs
    // after Steps 1-7 succeed; all receipt bytes were already BLAKE3-verified in Step 1).
    let in_mem_ctx = ValidationContext {
        repo_root: None,
        controls: projected_controls.clone(),
        live_claims: opts.live_claims.clone(),
        live_evidence: opts.live_evidence.clone(),
        ..ValidationContext::default()
    };
    let violations = validate(&candidate_reg, &in_mem_ctx);
    if let Some(first) = violations.first() {
        return Err(violation_to_refusal(first));
    }

    // Step 7b: Run every Step-7 hook in registration order; first refusal stops without writing.
    let hook_ctx = RecordHookContext {
        gate_id: &gate.id,
        gate: &gate,
        part: &candidate_part,
        candidate: &candidate_reg,
        receipts: &loaded,
        resolved_evidence: &evidence,
    };
    if !opts.skip_global_checks {
        for (_name, hook) in snapshot_checks() {
            if let Err(hr) = hook(&hook_ctx) {
                return Err(RecordRefusal::harness(hr.reason, hr.detail, hr.repair));
            }
        }
    }
    for (_name, hook) in &opts.extra_hooks {
        if let Err(hr) = hook(&hook_ctx) {
            return Err(RecordRefusal::harness(hr.reason, hr.detail, hr.repair));
        }
    }

    // Steps 8 & 9: All checks passed — commit receipts, update controls & history, write gates.toml,
    // append bead log line, and regenerate status views.
    let gates_receipts_dir = opts.repo_root.join("receipts/gates");
    fs::create_dir_all(&gates_receipts_dir).map_err(|e| {
        RecordRefusal::custom(
            "io-error",
            format!("create `{}`: {e}", gates_receipts_dir.display()),
            "Check directory permissions",
        )
    })?;
    let mut committed_receipts = Vec::new();
    for item in &loaded {
        let dest = gates_receipts_dir.join(format!("{}.json", item.hash));
        write_bytes_atomic(&dest, &item.raw_bytes)?;
        committed_receipts.push(dest);
    }
    if let Some(ref set_bytes) = receipt_set_bytes {
        let set_dest = gates_receipts_dir.join(format!("{part_receipt_hash}.json"));
        write_bytes_atomic(&set_dest, set_bytes)?;
        committed_receipts.push(set_dest);
    }

    // Update controls.toml last_seen and append control_history.ndjson on pass.
    if status == Status::Pass
        && let Some(ref ctrls) = loaded_controls
    {
        let updated_rows = update_controls_last_seen(ctrls.all(), &due_controls, &session, &date);
        let serialized = serialize_controls_file(&updated_rows);
        write_bytes_atomic(&controls_path, serialized.as_bytes())?;
        for cid in &due_controls {
            let entry = ControlHistoryEntry {
                id: cid.clone(),
                gate: gate.id.clone(),
                receipt: part_receipt_hash.clone(),
                outcome: "caught".to_owned(),
                commit: common_commit.clone(),
                session: session.clone(),
                date: date.clone(),
            };
            append_history_entry(&history_path, &entry).map_err(|e| {
                RecordRefusal::custom("io-error", e, "Check control_history.ndjson permissions")
            })?;
        }
    }

    // Write gates.toml atomically via register::write::record with full on-disk ValidationContext.
    let final_ctx = ValidationContext {
        repo_root: Some(opts.repo_root.clone()),
        controls: projected_controls,
        live_claims: opts.live_claims.clone(),
        live_evidence: opts.live_evidence.clone(),
        ..ValidationContext::default()
    };
    let updated_reg = write::record(&registry_path, &gate.id, candidate_part.clone(), &final_ctx)
        .map_err(|vs| violation_to_refusal(&vs[0]))?;

    // Append `gate-record` line (`frob.beadlog/1`) to delivering bead log when configured.
    let bead_log_path = if let Some(ref bead_id) = opts.bead_id {
        let path = opts
            .repo_root
            .join(format!("docs/delivery/logs/{bead_id}.ndjson"));
        let line = BeadLogGateRecord {
            schema: BEADLOG_SCHEMA.to_owned(),
            kind: "gate-record".to_owned(),
            gate: gate.id.clone(),
            part: part_milestone,
            verdict: status.as_str().to_owned(),
            claim_ids: gate.claim_id.iter().cloned().collect(),
            evidence,
            receipt: part_receipt_hash,
            recorded_by: session,
            date,
        };
        let cb = canon::to_canonical(&line).map_err(|e| {
            RecordRefusal::custom("canon-error", e.to_string(), "Fix bead log fields")
        })?;
        append_line(&path, cb.as_str())?;
        Some(path)
    } else {
        None
    };

    // Regenerate README status view if requested.
    let mut regenerated_files = Vec::new();
    if let Some(ref readme_path) = opts.readme_path {
        let table = render_readme_status_table(&updated_reg);
        write_bytes_atomic(readme_path, table.as_bytes())?;
        regenerated_files.push(readme_path.clone());
    }

    Ok(RecordOutput {
        gate_id: gate.id,
        part: candidate_part,
        committed_receipts,
        not_yet_due_controls,
        regenerated_files,
        bead_log_path,
        idempotent_noop: false,
    })
}

/// Shared-lock dispatch entrypoint for `record_gate -- reaffirm ...` (`N29bp1-01`).
///
/// Acquires the same exclusive [`RegistryLock`] as [`record_gate`] so re-affirmation and
/// recording never interleave on `registries/gates.toml`. The full graph walk lands in `N29bp1-01`.
pub fn run_reaffirm_lock_dispatch(
    registry_path: &Path,
    session: &str,
    timeout: Duration,
) -> Result<PathBuf, RecordRefusal> {
    let lock = RegistryLock::acquire(registry_path, session, timeout).map_err(|e| {
        RecordRefusal::custom(
            "registry-lock",
            e.to_string(),
            "Wait for the holding recorder or inspect and clear a stale lock explicitly",
        )
    })?;
    let path = lock.path().to_path_buf();
    drop(lock);
    Ok(path)
}

fn load_and_verify_receipt(
    repo_root: &Path,
    receipts_dir: &Path,
    hex: &str,
) -> Result<LoadedReceipt, RecordRefusal> {
    if Blake3::from_hex(hex).is_err() {
        return Err(RecordRefusal::harness(
            HarnessReason::ReceiptHashMismatch,
            format!("receipt hash `{hex}` is not 64 lower-case hex digits"),
            "Pass the 64 lower-case hex BLAKE3 digest of a Gauntlet receipt",
        ));
    }
    let primary = receipts_dir.join(format!("{hex}.json"));
    let committed = repo_root.join("receipts/gates").join(format!("{hex}.json"));
    let path = if primary.is_file() {
        primary
    } else if committed.is_file() {
        committed
    } else {
        return Err(RecordRefusal::harness(
            HarnessReason::ReceiptHashMismatch,
            format!(
                "receipt `{hex}` not found under `{}` or `receipts/gates/`",
                receipts_dir.display()
            ),
            "Re-run the tier target to emit target/gauntlet/receipts/<hash>.json",
        ));
    };

    let raw_bytes = fs::read(&path).map_err(|e| {
        RecordRefusal::harness(
            HarnessReason::ReceiptHashMismatch,
            format!("failed reading receipt `{}`: {e}", path.display()),
            "Re-run the tier target",
        )
    })?;
    let actual_hex = hash_blob(&raw_bytes).to_hex();
    if actual_hex != hex {
        return Err(RecordRefusal::harness(
            HarnessReason::ReceiptHashMismatch,
            format!(
                "receipt `{}` BLAKE3 `{actual_hex}` does not match filename `{hex}`",
                path.display()
            ),
            "Re-run the tier target; never edit a receipt file",
        ));
    }
    let (receipt, _) = read_and_verify_receipt(&path).map_err(|e| {
        RecordRefusal::harness(
            HarnessReason::ReceiptHashMismatch,
            format!("invalid receipt `{hex}`: {e}"),
            "Re-run the tier target to emit a canonical receipt",
        )
    })?;
    Ok(LoadedReceipt {
        hash: hex.to_owned(),
        raw_bytes,
        receipt,
    })
}

fn combine_gate_lines(
    gate: &GateRow,
    lines: &[(&LoadedReceipt, &crate::tiers::ReceiptGate)],
) -> Result<(Status, Option<String>), RecordRefusal> {
    // Check `no-claim: <reason>` first.
    for (_, gline) in lines {
        if let Some(reason) = gline.line.strip_prefix("no-claim:") {
            let trimmed = reason.trim();
            if gate.claim != Some(Claim::Reality) || trimmed.is_empty() {
                return Err(RecordRefusal::harness(
                    HarnessReason::NoClaimOnNonReality,
                    format!(
                        "gate `{}` (claim `{:?}`) cannot record `{}`",
                        gate.id, gate.claim, gline.line
                    ),
                    "Use `no-claim: <reason>` only on `claim = \"reality\"` gates",
                ));
            }
            return Ok((Status::NoClaim, Some(trimmed.to_owned())));
        }
    }
    // Any failing tier makes the combined part `fail`.
    if lines.iter().any(|(_, g)| g.line == "fail") {
        return Ok((Status::Fail, None));
    }
    // Every tier must be `pass` to record `pass`.
    if let Some((item, gline)) = lines.iter().find(|(_, g)| g.line != "pass") {
        return Err(RecordRefusal::harness(
            HarnessReason::GateLineNotEligible,
            format!(
                "receipt `{}` ({}) reports gate line `{}` for `{}`",
                item.hash,
                item.receipt.tier.code(),
                gline.line,
                gate.id
            ),
            "Ensure all due controls and entries are eligible and passing",
        ));
    }
    // Reduced-count check: never record pass against a full-count threshold.
    for (item, gline) in lines {
        let reduced_entry = item.receipt.entries.iter().find_map(|e| e.reduced);
        if gline.reduced == Some(true) || reduced_entry.is_some() {
            let desc = reduced_entry.map_or_else(
                || "reduced count".to_owned(),
                |r| format!("reduced: {} of {}", r.count, r.target),
            );
            return Err(RecordRefusal::harness(
                HarnessReason::ReducedNotRecordable,
                format!(
                    "receipt `{}` for `{}` ran below target count ({desc})",
                    item.hash, gate.id
                ),
                "Run the full count; never record a reduced run as a pass at full count",
            ));
        }
    }
    Ok((Status::Pass, None))
}

fn check_no_dropped_entries(
    repo_root: &Path,
    prior_receipt_hex: &str,
    new_receipts: &[LoadedReceipt],
    retired_entries: &BTreeSet<String>,
) -> Result<(), RecordRefusal> {
    let prior_ids = collect_committed_entry_ids(repo_root, prior_receipt_hex);
    if prior_ids.is_empty() {
        return Ok(());
    }
    let new_ids: BTreeSet<&str> = new_receipts
        .iter()
        .flat_map(|r| r.receipt.entries.iter().map(|e| e.id.as_str()))
        .collect();
    let dropped: Vec<String> = prior_ids
        .into_iter()
        .filter(|id| !retired_entries.contains(id) && !new_ids.contains(id.as_str()))
        .collect();
    if !dropped.is_empty() {
        return Err(RecordRefusal::harness(
            HarnessReason::PartRowsDropped,
            format!(
                "re-recording omits previously measured entry id(s): {}",
                dropped.join(", ")
            ),
            "Add the receipt of the dropped entry's tier",
        ));
    }
    Ok(())
}

fn collect_committed_entry_ids(repo_root: &Path, receipt_hex: &str) -> BTreeSet<String> {
    let mut ids = BTreeSet::new();
    let hex = receipt_hex.strip_prefix("b3:").unwrap_or(receipt_hex);
    let path = repo_root.join("receipts/gates").join(format!("{hex}.json"));
    let Ok(bytes) = fs::read(&path) else {
        return ids;
    };
    if let Ok((r, _)) = read_and_verify_receipt(&path) {
        for e in r.entries {
            ids.insert(e.id);
        }
        return ids;
    }
    if let Ok(admitted) = json::admit(&bytes, &Limits::default())
        && admitted.value().get("schema").and_then(|v| v.as_str()) == Some(RECEIPT_SET_SCHEMA)
        && let Some(members) = admitted.value().get("members").and_then(|v| v.as_array())
    {
        for m in members {
            if let Some(m_hex) = m.as_str() {
                let m_path = repo_root
                    .join("receipts/gates")
                    .join(format!("{m_hex}.json"));
                if let Ok((r, _)) = read_and_verify_receipt(&m_path) {
                    for e in r.entries {
                        ids.insert(e.id);
                    }
                }
            }
        }
    }
    ids
}

fn build_measured_value(
    status: Status,
    receipts: &[LoadedReceipt],
    no_claim_reason: Option<&str>,
) -> String {
    if status == Status::NoClaim {
        return format!("no-claim ({})", no_claim_reason.unwrap_or("unspecified"));
    }
    if status == Status::Fail {
        for r in receipts {
            for e in &r.receipt.entries {
                if e.outcome == "fail" {
                    if let Some(ref diff) = e.first_diff
                        && !diff.trim().is_empty()
                    {
                        return diff.clone();
                    }
                    if let Some(ref m) = e.measured {
                        return format_measured_entry(m);
                    }
                }
            }
        }
    }
    let mut parts = Vec::new();
    for r in receipts {
        for e in &r.receipt.entries {
            if let Some(ref m) = e.measured {
                parts.push(format_measured_entry(m));
            }
        }
    }
    if !parts.is_empty() {
        return parts.join("; ");
    }
    let total_pass: u64 = receipts.iter().map(|r| r.receipt.counts.pass).sum();
    let total_fail: u64 = receipts.iter().map(|r| r.receipt.counts.fail).sum();
    format!("pass={total_pass} fail={total_fail}")
}

fn format_measured_entry(m: &crate::tiers::ReceiptMeasured) -> String {
    if m.unit.trim().is_empty() {
        m.value.clone()
    } else {
        format!("{} ({})", m.value, m.unit)
    }
}

fn git_is_ancestor(repo_root: &Path, commit: &str) -> bool {
    let c = commit.trim();
    if c.is_empty() || !c.bytes().all(|b| b.is_ascii_hexdigit()) {
        return false;
    }
    Command::new("git")
        .arg("-C")
        .arg(repo_root)
        .args(["merge-base", "--is-ancestor", c, "HEAD"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok_and(|s| s.success())
}

fn compute_implemented_by(
    repo_root: &Path,
    commit: &str,
    delivering_paths: &[PathBuf],
    bead_logs: &[PathBuf],
) -> String {
    let mut sessions = BTreeSet::new();

    // 1. Collect `Frob-Session:` trailers from git commits up to `commit`.
    let mut cmd = Command::new("git");
    cmd.arg("-C").arg(repo_root).args([
        "log",
        "--format=%(trailers:key=Frob-Session,valueonly=true)",
        commit,
    ]);
    if !delivering_paths.is_empty() {
        cmd.arg("--");
        for p in delivering_paths {
            cmd.arg(p);
        }
    }
    if let Ok(out) = cmd.output()
        && out.status.success()
    {
        for line in String::from_utf8_lossy(&out.stdout).lines() {
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                sessions.insert(normalize_session(trimmed));
            }
        }
    }

    // 2. Collect sessions from `implement` lines in delivering bead logs.
    for log_path in bead_logs {
        if let Ok(raw) = fs::read_to_string(log_path) {
            for line in raw.lines() {
                let t = line.trim();
                if t.is_empty() {
                    continue;
                }
                if let Ok(doc) = json::admit(t.as_bytes(), &Limits::default()) {
                    let v = doc.value();
                    let is_impl = v.get("kind").and_then(|x| x.as_str()) == Some("implement")
                        || v.get("action").and_then(|x| x.as_str()) == Some("implement");
                    if is_impl {
                        for key in ["session", "recorded_by", "actor"] {
                            if let Some(s) = v.get(key).and_then(|x| x.as_str())
                                && !s.trim().is_empty()
                            {
                                sessions.insert(normalize_session(s.trim()));
                            }
                        }
                    }
                }
            }
        }
    }

    sessions.into_iter().collect::<Vec<_>>().join(", ")
}

fn normalize_session(s: &str) -> String {
    let trimmed = s.trim();
    if trimmed.starts_with("session:") {
        trimmed.to_owned()
    } else {
        format!("session:{trimmed}")
    }
}

fn session_in_implementers(session: &str, implemented_by: &str) -> bool {
    let norm_rec = normalize_session(session);
    is_session_implementer(session, implemented_by)
        || is_session_implementer(&norm_rec, implemented_by)
}

fn claim_to_gate_class(c: Claim) -> GateClass {
    match c {
        Claim::Proof => GateClass::Proof,
        Claim::KnownAnswer => GateClass::KnownAnswer,
        Claim::Parity => GateClass::Parity,
        Claim::Fidelity => GateClass::Fidelity,
        Claim::DesignComparison => GateClass::DesignComparison,
        Claim::Reality => GateClass::Reality,
        Claim::Performance => GateClass::Performance,
    }
}

fn update_controls_last_seen(
    controls: &[Control],
    due_ids: &[String],
    session: &str,
    date: &str,
) -> Vec<Control> {
    let due_set: BTreeSet<&str> = due_ids.iter().map(String::as_str).collect();
    controls
        .iter()
        .map(|c| {
            let mut copy = c.clone();
            if due_set.contains(c.id.as_str()) {
                copy.last_seen = Some(LastSeen {
                    caught: true,
                    session: session.to_owned(),
                    date: date.to_owned(),
                });
            }
            copy
        })
        .collect()
}

fn parts_match_except_date(a: &Part, b: &Part) -> bool {
    a.milestone == b.milestone
        && a.status == b.status
        && a.measured == b.measured
        && a.commit == b.commit
        && a.command == b.command
        && a.receipt == b.receipt
        && a.fingerprint == b.fingerprint
        && a.seed == b.seed
        && a.evidence == b.evidence
        && a.detail == b.detail
        && a.recorded.session == b.recorded.session
        && a.recorded.implemented_by == b.recorded.implemented_by
}

fn receipts_already_committed(
    repo_root: &Path,
    loaded: &[LoadedReceipt],
    part_receipt_hash: &str,
    set_bytes: Option<&[u8]>,
) -> bool {
    let dir = repo_root.join("receipts/gates");
    for item in loaded {
        let p = dir.join(format!("{}.json", item.hash));
        if fs::read(&p).ok().as_deref() != Some(item.raw_bytes.as_slice()) {
            return false;
        }
    }
    if let Some(sb) = set_bytes {
        let p = dir.join(format!("{part_receipt_hash}.json"));
        if fs::read(&p).ok().as_deref() != Some(sb) {
            return false;
        }
    }
    true
}

fn committed_receipt_paths(
    repo_root: &Path,
    loaded: &[LoadedReceipt],
    part_receipt_hash: &str,
    has_set: bool,
) -> Vec<PathBuf> {
    let dir = repo_root.join("receipts/gates");
    let mut out: Vec<PathBuf> = loaded
        .iter()
        .map(|r| dir.join(format!("{}.json", r.hash)))
        .collect();
    if has_set {
        out.push(dir.join(format!("{part_receipt_hash}.json")));
    }
    out
}

fn violation_to_refusal(v: &Violation) -> RecordRefusal {
    RecordRefusal {
        reason: v.reason.to_owned(),
        harness_reason: v.harness_reason,
        detail: v.message.clone(),
        repair: v.repair.to_owned(),
    }
}

fn write_bytes_atomic(path: &Path, bytes: &[u8]) -> Result<(), RecordRefusal> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| {
            RecordRefusal::custom(
                "io-error",
                format!("create dir `{}`: {e}", parent.display()),
                "Check directory permissions",
            )
        })?;
    }
    if path.is_file() && fs::read(path).ok().as_deref() == Some(bytes) {
        return Ok(());
    }
    static SEQ: AtomicU64 = AtomicU64::new(0);
    let n = SEQ.fetch_add(1, Ordering::Relaxed);
    let pid = std::process::id();
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    let fname = path.file_name().and_then(|s| s.to_str()).unwrap_or("file");
    let tmp = parent.join(format!(".{fname}.rec.{pid}.{n}.tmp"));
    let mut f = fs::File::create(&tmp).map_err(|e| {
        RecordRefusal::custom(
            "io-error",
            format!("create temp `{}`: {e}", tmp.display()),
            "Check filesystem permissions",
        )
    })?;
    f.write_all(bytes)
        .and_then(|()| f.sync_all())
        .map_err(|e| {
            RecordRefusal::custom(
                "io-error",
                format!("write temp `{}`: {e}", tmp.display()),
                "Check disk space and permissions",
            )
        })?;
    drop(f);
    fs::rename(&tmp, path).map_err(|e| {
        RecordRefusal::custom(
            "io-error",
            format!("rename `{}` -> `{}`: {e}", tmp.display(), path.display()),
            "Check directory permissions",
        )
    })
}

fn append_line(path: &Path, line: &str) -> Result<(), RecordRefusal> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| {
            RecordRefusal::custom(
                "io-error",
                format!("create dir `{}`: {e}", parent.display()),
                "Check directory permissions",
            )
        })?;
    }
    let mut f = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .map_err(|e| {
            RecordRefusal::custom(
                "io-error",
                format!("open `{}`: {e}", path.display()),
                "Check permissions",
            )
        })?;
    writeln!(f, "{line}")
        .and_then(|()| f.sync_all())
        .map_err(|e| {
            RecordRefusal::custom(
                "io-error",
                format!("append `{}`: {e}", path.display()),
                "Check disk space",
            )
        })
}

fn utc_today_iso() -> String {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_secs());
    let days = (secs / 86_400) as i64;
    // Civil date from Unix day count (Howard Hinnant's Euclidean algorithm).
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = (z - era * 146_097) as u64;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let y = (yoe as i64) + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = if m <= 2 { y + 1 } else { y };
    format!("{year:04}-{m:02}-{d:02}")
}
