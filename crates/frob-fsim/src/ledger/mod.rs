#![forbid(unsafe_code)]

//! `ledger` airlock group (`BD-009`, `N33a-11`, `INT-FSQL-01`, default on).
//!
//! Owns the FrankenRobotics `Ledger` (v0) trait, typed refusals, M0 canonical
//! record helpers (`meta`, `run`, `episode`), the deterministic forced-kill
//! probe vocabulary ([`probe`]), the declared JSON-lines fallback interface
//! ([`JsonlFallbackLedger`]), and the in-process transactional adapter
//! ([`FsqliteLedger`] behind feature `ledger`).
//!
//! No upstream storage or database engine type appears in any public signature.
//! Rows are opaque canonical bytes produced by the caller; the adapter never
//! interprets row payloads. Determinism class `D0` for digests, sequence
//! numbers, and refusal shapes; zero `unsafe`; zero library stdout/stderr
//! printing (rule L-7).

pub mod probe;

#[cfg(feature = "ledger")]
mod store;

#[cfg(feature = "ledger")]
pub use store::{FsqliteLedger, Txn};

#[cfg(all(test, feature = "ledger"))]
mod tests;

use std::fmt;
use std::path::Path;
use std::time::Duration;

use frob_core::canon::{self, CanonError, Canonical, Value as CanonValue};
use frob_core::id::CanonicalBytes;

use crate::groups::{Group, GroupAbsent};

/// The airlock group represented by this module.
pub const GROUP: Group = Group::Ledger;

/// Whether the `ledger` feature was enabled at compile time.
pub const ENABLED: bool = cfg!(feature = "ledger");

/// Schema identifier string reported by the M0 ledger adapter.
pub const LEDGER_SCHEMA_ID: &str = "frob.ledger/0";

/// Default M0 schema version number stored in the `meta` header.
pub const M0_SCHEMA_VERSION: SchemaVersion = SchemaVersion(1);

/// Logical schema version stored in the ledger's `meta` header.
///
/// A binary refuses to open a ledger whose stored [`SchemaVersion`] is strictly
/// newer than the version the binary supports, without writing to the ledger.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SchemaVersion(pub u32);

impl SchemaVersion {
    /// M0 initial schema version (`1`).
    pub const M0: Self = M0_SCHEMA_VERSION;

    /// Construct a schema version (`>= 1` by convention).
    #[must_use]
    pub const fn new(version: u32) -> Self {
        Self(version)
    }

    /// Raw numeric schema version.
    #[must_use]
    pub const fn as_u32(self) -> u32 {
        self.0
    }
}

impl fmt::Display for SchemaVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "v{}", self.0)
    }
}

impl Canonical for SchemaVersion {
    fn write(&self, w: CanonValue<'_>) -> Result<(), CanonError> {
        w.int53(i64::from(self.0))
    }
}

/// Monotone 64-bit commit sequence assigned inside the committing transaction.
///
/// Serialized as a decimal string in every JSON view because a 64-bit counter
/// can exceed IEEE-754's exact `2^53 - 1` integer range.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct CommitSeq(u64);

impl CommitSeq {
    /// Construct from a raw monotone sequence number.
    #[must_use]
    pub const fn new(seq: u64) -> Self {
        Self(seq)
    }

    /// Raw 64-bit commit sequence value.
    #[must_use]
    pub const fn as_u64(self) -> u64 {
        self.0
    }

    /// Decimal string representation for JSON views.
    #[must_use]
    pub fn to_json_string(self) -> String {
        self.0.to_string()
    }
}

impl fmt::Display for CommitSeq {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Canonical for CommitSeq {
    fn write(&self, w: CanonValue<'_>) -> Result<(), CanonError> {
        w.u64str(self.0);
        Ok(())
    }
}

/// Closed set of M0 ledger tables (`meta`, `run`, `episode`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TableId {
    /// Ledger metadata: schema version, creation receipt hash, and probe chain state.
    Meta,
    /// Top-level simulation/evaluation runs (identity, model, tier, stack receipt).
    Run,
    /// Episode rows belonging to a run (final state digest, telemetry summaries).
    Episode,
}

impl TableId {
    /// All M0 tables in canonical declaration order.
    pub const ALL: &'static [Self] = &[Self::Meta, Self::Run, Self::Episode];

    /// Stable canonical table name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Meta => "meta",
            Self::Run => "run",
            Self::Episode => "episode",
        }
    }

    /// Parse a table name (`meta`, `run`, `episode`).
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "meta" => Some(Self::Meta),
            "run" => Some(Self::Run),
            "episode" => Some(Self::Episode),
            _ => None,
        }
    }
}

impl fmt::Display for TableId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Canonical for TableId {
    fn write(&self, w: CanonValue<'_>) -> Result<(), CanonError> {
        w.str(self.as_str());
        Ok(())
    }
}

/// Configuration supplied to [`Ledger::open`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenOptions {
    /// Maximum wait when another writer holds the single-writer transaction lock.
    pub busy_timeout: Duration,
    /// Create the ledger file and initialize M0 metadata if absent.
    pub create_if_missing: bool,
    /// Optional stack receipt hash recorded in `meta` when creating a new ledger.
    /// Defaults to `frob_core::stack::BUILT_UNDER.receipt` when `None`.
    pub creation_receipt: Option<String>,
}

impl Default for OpenOptions {
    fn default() -> Self {
        Self {
            busy_timeout: Duration::from_secs(5),
            create_if_missing: true,
            creation_receipt: None,
        }
    }
}

impl OpenOptions {
    /// Set the single-writer busy timeout.
    #[must_use]
    pub const fn with_busy_timeout(mut self, busy_timeout: Duration) -> Self {
        self.busy_timeout = busy_timeout;
        self
    }

    /// Set whether opening an absent path creates a new ledger.
    #[must_use]
    pub const fn with_create_if_missing(mut self, create_if_missing: bool) -> Self {
        self.create_if_missing = create_if_missing;
        self
    }

    /// Override the creation receipt hash written to `meta` on fresh creation.
    #[must_use]
    pub fn with_creation_receipt(mut self, receipt: impl Into<String>) -> Self {
        self.creation_receipt = Some(receipt.into());
        self
    }
}

/// What [`Ledger::open`] observed in the on-disk log state before opening.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RecoveryReport {
    /// No pending write-ahead log frames or rollback journal were present.
    Clean,
    /// An incomplete transaction (rollback journal or trailing uncommitted WAL
    /// frames) was detected and rolled back on open.
    RolledBackIncompleteTxn,
    /// Complete committed WAL frames were present and replayed on open.
    ReplayedLog,
}

impl RecoveryReport {
    /// Canonical kebab-case label (`clean`, `rolled-back-incomplete-txn`, `replayed-log`).
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Clean => "clean",
            Self::RolledBackIncompleteTxn => "rolled-back-incomplete-txn",
            Self::ReplayedLog => "replayed-log",
        }
    }
}

impl fmt::Display for RecoveryReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Canonical for RecoveryReport {
    fn write(&self, w: CanonValue<'_>) -> Result<(), CanonError> {
        w.str(self.as_str());
        Ok(())
    }
}

/// Structural and cryptographic integrity check outcome returned by
/// [`Ledger::integrity_check`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntegrityReport {
    /// `true` iff every structural, WAL-checksum, metadata, and BLAKE3 content
    /// check passed with zero findings.
    pub clean: bool,
    /// Number of content-addressed row payloads verified against BLAKE3.
    pub artifacts_checked: u64,
    /// Hex digests of any row payloads whose stored bytes disagreed with BLAKE3.
    pub corrupted_artifacts: Vec<String>,
    /// Number of WAL frames inspected during on-disk checksum auditing.
    pub wal_frames_checked: u64,
    /// Ordered human-readable diagnostics for any detected corruption or drift.
    pub findings: Vec<String>,
}

impl IntegrityReport {
    /// True iff no corruption or structural anomaly was reported.
    #[must_use]
    pub fn is_clean(&self) -> bool {
        self.clean && self.corrupted_artifacts.is_empty() && self.findings.is_empty()
    }
}

/// Durability settings active on the underlying transactional engine at the pin.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DurabilityConfig {
    /// Active journal mode (e.g. `"WAL"`).
    pub journal_mode: &'static str,
    /// Active synchronous durability level (e.g. `"FULL"`).
    pub synchronous: &'static str,
    /// Whether foreign key enforcement is enabled on the engine connection.
    pub foreign_keys: bool,
    /// Engine-level busy timeout in milliseconds.
    pub engine_busy_timeout_ms: u32,
}

/// Classification of every refusal surfaced by [`Ledger`] and [`LedgerTxn`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LedgerRefusalKind {
    /// The `ledger` airlock group is not compiled into this binary.
    GroupAbsent,
    /// Opening or initializing the ledger failed.
    OpenFailed,
    /// The ledger file carries a schema version newer than this binary supports.
    NewerSchema { found: u32, supported: u32 },
    /// Retryable single-writer or engine contention (bounded by `busy_timeout`).
    Busy,
    /// Structural, checksum, or content-hash corruption; NEVER retried.
    Integrity,
    /// Filesystem I/O failure.
    Io,
    /// Caller supplied an invalid path, key, or schema parameter.
    InvalidInput,
    /// Any other non-retryable upstream engine error.
    UpstreamEngine,
}

/// Typed refusal returned by all [`Ledger`] and [`LedgerTxn`] operations.
///
/// Every upstream engine error is converted into a `LedgerRefusal` carrying the
/// upstream reason text. Corruption (`LedgerRefusalKind::Integrity`) is never
/// marked retryable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LedgerRefusal {
    /// Typed refusal category.
    pub kind: LedgerRefusalKind,
    /// Operation context where the refusal occurred.
    pub context: String,
    /// Human-readable diagnostic summary.
    pub detail: String,
    /// Optional raw diagnostic text from the underlying engine.
    pub upstream_reason: Option<String>,
    /// Whether retrying the operation is permitted (`false` for corruption and schema mismatch).
    pub retryable: bool,
}

impl LedgerRefusal {
    /// Construct a `Busy` refusal (retryable).
    #[must_use]
    pub fn busy(
        context: impl Into<String>,
        detail: impl Into<String>,
        upstream_reason: Option<String>,
    ) -> Self {
        Self {
            kind: LedgerRefusalKind::Busy,
            context: context.into(),
            detail: detail.into(),
            upstream_reason,
            retryable: true,
        }
    }

    /// Construct an `Integrity` refusal (never retryable).
    #[must_use]
    pub fn integrity(
        context: impl Into<String>,
        detail: impl Into<String>,
        upstream_reason: Option<String>,
    ) -> Self {
        Self {
            kind: LedgerRefusalKind::Integrity,
            context: context.into(),
            detail: detail.into(),
            upstream_reason,
            retryable: false,
        }
    }

    /// Construct a `NewerSchema` refusal (never retryable).
    #[must_use]
    pub fn newer_schema(found: u32, supported: u32, upstream_reason: Option<String>) -> Self {
        Self {
            kind: LedgerRefusalKind::NewerSchema { found, supported },
            context: "open".to_owned(),
            detail: format!(
                "ledger schema version v{found} is newer than supported v{supported}; refused without writing"
            ),
            upstream_reason,
            retryable: false,
        }
    }

    /// Construct an `OpenFailed` refusal.
    #[must_use]
    pub fn open_failed(
        context: impl Into<String>,
        detail: impl Into<String>,
        upstream_reason: Option<String>,
    ) -> Self {
        Self {
            kind: LedgerRefusalKind::OpenFailed,
            context: context.into(),
            detail: detail.into(),
            upstream_reason,
            retryable: false,
        }
    }

    /// Construct an `InvalidInput` refusal.
    #[must_use]
    pub fn invalid_input(context: impl Into<String>, detail: impl Into<String>) -> Self {
        Self {
            kind: LedgerRefusalKind::InvalidInput,
            context: context.into(),
            detail: detail.into(),
            upstream_reason: None,
            retryable: false,
        }
    }

    /// Construct an `Io` refusal.
    #[must_use]
    pub fn io(context: impl Into<String>, detail: impl Into<String>) -> Self {
        Self {
            kind: LedgerRefusalKind::Io,
            context: context.into(),
            detail: detail.into(),
            upstream_reason: None,
            retryable: false,
        }
    }

    /// Construct a non-retryable `UpstreamEngine` refusal.
    #[must_use]
    pub fn upstream(
        context: impl Into<String>,
        detail: impl Into<String>,
        upstream_reason: String,
    ) -> Self {
        Self {
            kind: LedgerRefusalKind::UpstreamEngine,
            context: context.into(),
            detail: detail.into(),
            upstream_reason: Some(upstream_reason),
            retryable: false,
        }
    }

    /// Construct a `GroupAbsent` refusal when feature `ledger` is disabled.
    #[must_use]
    pub fn group_absent() -> Self {
        Self {
            kind: LedgerRefusalKind::GroupAbsent,
            context: "ledger".to_owned(),
            detail: "airlock group `ledger` is not compiled into this binary".to_owned(),
            upstream_reason: None,
            retryable: false,
        }
    }

    /// True iff this refusal is a `Busy` contention refusal.
    #[must_use]
    pub fn is_busy(&self) -> bool {
        matches!(self.kind, LedgerRefusalKind::Busy)
    }

    /// True iff this refusal is an `Integrity` corruption refusal.
    #[must_use]
    pub fn is_integrity(&self) -> bool {
        matches!(self.kind, LedgerRefusalKind::Integrity)
    }

    /// True iff this refusal is a `NewerSchema` refusal.
    #[must_use]
    pub fn is_newer_schema(&self) -> bool {
        matches!(self.kind, LedgerRefusalKind::NewerSchema { .. })
    }

    /// True iff retrying the operation is permitted. Corruption and schema
    /// refusals are never retryable.
    #[must_use]
    pub fn is_retryable(&self) -> bool {
        self.retryable && !self.is_integrity() && !self.is_newer_schema()
    }
}

impl fmt::Display for LedgerRefusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ledger refusal ({:?}) at {}: {}",
            self.kind, self.context, self.detail
        )?;
        if let Some(ref upstream) = self.upstream_reason {
            write!(f, " [upstream: {upstream}]")?;
        }
        Ok(())
    }
}

impl std::error::Error for LedgerRefusal {}

/// Minimal M0 `meta` row describing ledger schema version and creation stack receipt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MetaRecord {
    pub schema_version: SchemaVersion,
    pub creation_receipt_hash: String,
}

impl MetaRecord {
    /// Encode as canonical JSON bytes (`frob_core::canon`).
    pub fn to_canonical_bytes(&self) -> Result<CanonicalBytes, CanonError> {
        canon::to_canonical(self)
    }
}

impl Canonical for MetaRecord {
    fn write(&self, w: CanonValue<'_>) -> Result<(), CanonError> {
        w.object(|o| {
            o.str("creation_receipt_hash", &self.creation_receipt_hash)?;
            o.member_term("schema_version", &self.schema_version)?;
            Ok(())
        })
    }
}

/// Minimal M0 `run` row required by the first referee rollout (`BD-024`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunRecord {
    pub run_id: String,
    pub model_id: String,
    pub tier: String,
    pub stack_receipt_hash: String,
}

impl RunRecord {
    /// Encode as canonical JSON bytes (`frob_core::canon`).
    pub fn to_canonical_bytes(&self) -> Result<CanonicalBytes, CanonError> {
        canon::to_canonical(self)
    }
}

impl Canonical for RunRecord {
    fn write(&self, w: CanonValue<'_>) -> Result<(), CanonError> {
        w.object(|o| {
            o.str("model_id", &self.model_id)?;
            o.str("run_id", &self.run_id)?;
            o.str("stack_receipt_hash", &self.stack_receipt_hash)?;
            o.str("tier", &self.tier)?;
            Ok(())
        })
    }
}

/// Minimal M0 `episode` row required by the first referee rollout (`BD-024`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EpisodeRecord {
    pub run_id: String,
    pub episode_index: u64,
    pub model_id: String,
    pub tier: String,
    pub final_state_digest: String,
    pub stack_receipt_hash: String,
}

impl EpisodeRecord {
    /// Encode as canonical JSON bytes (`frob_core::canon`).
    pub fn to_canonical_bytes(&self) -> Result<CanonicalBytes, CanonError> {
        canon::to_canonical(self)
    }
}

impl Canonical for EpisodeRecord {
    fn write(&self, w: CanonValue<'_>) -> Result<(), CanonError> {
        w.object(|o| {
            o.u64str("episode_index", self.episode_index)?;
            o.str("final_state_digest", &self.final_state_digest)?;
            o.str("model_id", &self.model_id)?;
            o.str("run_id", &self.run_id)?;
            o.str("stack_receipt_hash", &self.stack_receipt_hash)?;
            o.str("tier", &self.tier)?;
            Ok(())
        })
    }
}

/// Single-writer transactional write handle borrowed from a [`Ledger`].
///
/// Dropping a `LedgerTxn` without calling [`LedgerTxn::commit`] rolls back all
/// staged writes and releases the single-writer lock.
pub trait LedgerTxn {
    /// Stage a canonical byte row under `(table, key)` inside this transaction.
    fn put(&mut self, table: TableId, key: &[u8], row: &[u8]) -> Result<(), LedgerRefusal>;

    /// Read the canonical byte row currently visible for `(table, key)`.
    fn get(&self, table: TableId, key: &[u8]) -> Result<Option<Vec<u8>>, LedgerRefusal>;

    /// Commit this transaction durably, returning its monotone [`CommitSeq`].
    fn commit(self) -> Result<CommitSeq, LedgerRefusal>;
}

/// Transactional metadata ledger trait (`BD-009`, `INT-FSQL-01`).
///
/// One writer at a time (`begin(&mut self)` borrows mutably on one handle, and
/// cross-handle/cross-process writers synchronize via an advisory writer lock
/// bounded by [`OpenOptions::busy_timeout`]).
pub trait Ledger: Sized {
    /// Active transaction handle tied to `&mut self`.
    type Txn<'a>: LedgerTxn
    where
        Self: 'a;

    /// Open (or create, when permitted) the ledger at `path` for `schema`.
    ///
    /// Refuses any existing ledger whose stored schema version is newer than
    /// `schema`, without writing to durable storage.
    fn open(path: &Path, schema: SchemaVersion, opts: OpenOptions) -> Result<Self, LedgerRefusal>;

    /// Begin a single-writer transaction.
    fn begin(&mut self) -> Result<Self::Txn<'_>, LedgerRefusal>;

    /// Perform structural, WAL-checksum, metadata, and content-hash checks.
    fn integrity_check(&self) -> Result<IntegrityReport, LedgerRefusal>;

    /// Report what [`Ledger::open`] observed in the on-disk log state.
    fn recovered(&self) -> RecoveryReport;

    /// Active durability settings read from the underlying engine at the pin.
    fn durability_config(&self) -> DurabilityConfig;

    /// Schema identifier of this ledger adapter.
    fn ledger_schema(&self) -> &'static str {
        LEDGER_SCHEMA_ID
    }
}

/// Declared interface for the pre-named JSON-lines fallback ledger (`BD-009` Scope).
///
/// Recovery rule: one canonical record per line with a per-line BLAKE3 checksum,
/// discarding a truncated final line on open; queries scan. Declared here to pin
/// the fallback contract at M0; built and wired only if `INT-FSQL-01` fails.
#[derive(Debug)]
pub struct JsonlFallbackLedger {
    _unconstructed: (),
}

/// Declared transaction handle for [`JsonlFallbackLedger`].
#[derive(Debug)]
pub struct JsonlFallbackTxn<'a> {
    _owner: &'a mut JsonlFallbackLedger,
}

impl LedgerTxn for JsonlFallbackTxn<'_> {
    fn put(&mut self, _table: TableId, _key: &[u8], _row: &[u8]) -> Result<(), LedgerRefusal> {
        Err(LedgerRefusal::open_failed(
            "jsonl_fallback.put",
            "JSON-lines fallback ledger is declared only and not built while INT-FSQL-01 is active",
            None,
        ))
    }

    fn get(&self, _table: TableId, _key: &[u8]) -> Result<Option<Vec<u8>>, LedgerRefusal> {
        Err(LedgerRefusal::open_failed(
            "jsonl_fallback.get",
            "JSON-lines fallback ledger is declared only and not built while INT-FSQL-01 is active",
            None,
        ))
    }

    fn commit(self) -> Result<CommitSeq, LedgerRefusal> {
        Err(LedgerRefusal::open_failed(
            "jsonl_fallback.commit",
            "JSON-lines fallback ledger is declared only and not built while INT-FSQL-01 is active",
            None,
        ))
    }
}

impl Ledger for JsonlFallbackLedger {
    type Txn<'a> = JsonlFallbackTxn<'a>;

    fn open(
        _path: &Path,
        _schema: SchemaVersion,
        _opts: OpenOptions,
    ) -> Result<Self, LedgerRefusal> {
        Err(LedgerRefusal::open_failed(
            "jsonl_fallback.open",
            "JSON-lines fallback ledger is declared only and built only if INT-FSQL-01 fails",
            None,
        ))
    }

    fn begin(&mut self) -> Result<Self::Txn<'_>, LedgerRefusal> {
        Err(LedgerRefusal::open_failed(
            "jsonl_fallback.begin",
            "JSON-lines fallback ledger is declared only and not built while INT-FSQL-01 is active",
            None,
        ))
    }

    fn integrity_check(&self) -> Result<IntegrityReport, LedgerRefusal> {
        Err(LedgerRefusal::open_failed(
            "jsonl_fallback.integrity_check",
            "JSON-lines fallback ledger is declared only and not built while INT-FSQL-01 is active",
            None,
        ))
    }

    fn recovered(&self) -> RecoveryReport {
        RecoveryReport::Clean
    }

    fn durability_config(&self) -> DurabilityConfig {
        DurabilityConfig {
            journal_mode: "append-only-jsonl",
            synchronous: "fsync-per-commit",
            foreign_keys: false,
            engine_busy_timeout_ms: 0,
        }
    }
}

/// Packaged, replayable sessions (`N33a-17`).
pub trait Session {
    /// Canonical session package version string.
    fn session_version(&self) -> &'static str;
}

/// A content-addressed graph of results with skip certificates and invalidation (`N33b-15`).
pub trait Recompute {
    /// Number of nodes tracked in the recompute graph.
    fn node_count(&self) -> usize;
}

/// Check whether the `ledger` group is compiled into this binary.
pub fn available() -> Result<(), GroupAbsent> {
    if ENABLED {
        Ok(())
    } else {
        Err(GroupAbsent::new(GROUP))
    }
}

#[cfg(not(feature = "ledger"))]
/// Stub constructor when the `ledger` group is not compiled in.
pub fn new_ledger() -> Result<(), GroupAbsent> {
    Err(GroupAbsent::new(GROUP))
}

#[cfg(not(feature = "ledger"))]
/// Stub constructor when the `ledger` group is not compiled in.
pub fn new_session() -> Result<(), GroupAbsent> {
    Err(GroupAbsent::new(GROUP))
}

#[cfg(not(feature = "ledger"))]
/// Stub constructor when the `ledger` group is not compiled in.
pub fn new_recompute() -> Result<(), GroupAbsent> {
    Err(GroupAbsent::new(GROUP))
}
