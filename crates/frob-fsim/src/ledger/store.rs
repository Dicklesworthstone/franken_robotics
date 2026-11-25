#![forbid(unsafe_code)]

//! Private FrankenSQLite-backed adapter (`FsqliteLedger`) implementing the
//! `BD-009` (`INT-FSQL-01`) [`Ledger`] trait over `fs_ledger::Ledger`.
//!
//! No `fs_ledger` or `fsqlite` type is exposed in any public signature.
//! Single-writer exclusion is enforced both at compile time on a single handle
//! (`Txn<'a>` borrows `&'a mut FsqliteLedger`) and across handles/processes
//! via a process-local writer table paired with an OS advisory lock file
//! (`std::fs::File::try_lock`, released automatically by the kernel on
//! `SIGKILL`). Scratch/lock files are never deleted (`COMMON.md` 9b).

use std::collections::BTreeSet;
use std::fs::{self, File, OpenOptions as FsOpenOptions, TryLockError};
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};
use std::thread;
use std::time::{Duration, Instant};

use frob_core::canon::{self, CanonError, Canonical, Value as CanonValue};
use frob_core::json::{self, Limits};
use frob_core::stack::BUILT_UNDER;
use fs_ledger::{ContentHash, ExtensionTable, Ledger as FsLedger, LedgerError};

use super::{
    CommitSeq, DurabilityConfig, IntegrityReport, LEDGER_SCHEMA_ID, Ledger, LedgerRefusal,
    LedgerTxn, OpenOptions, RecoveryReport, SchemaVersion, TableId,
};

/// Active durability settings applied by `fs_ledger::Ledger::open` at the pin
/// (`../frankensim/crates/fs-ledger/src/lib.rs:3718-3721`).
const PIN_DURABILITY: DurabilityConfig = DurabilityConfig {
    journal_mode: "WAL",
    synchronous: "FULL",
    foreign_keys: true,
    engine_busy_timeout_ms: 5000,
};

const ROW_ARTIFACT_KIND: &str = "frob.ledger.row.v0";
const CONTROL_HEADER_KEY: &str = "frob/v0/$header";
const CONTROL_COMMIT_SEQ_KEY: &str = "frob/v0/$commit_seq";

const SQLITE_HEADER_MAGIC: &[u8; 16] = b"SQLite format 3\0";
const WAL_HEADER_SIZE: usize = 32;
const WAL_FRAME_HEADER_SIZE: usize = 24;
const WAL_MAGIC_LE: u32 = 0x377F_0682;
const WAL_MAGIC_BE: u32 = 0x377F_0683;

/// Process-local active writer set ensuring deterministic intra-process
/// exclusion across distinct [`FsqliteLedger`] handles opened on the same path.
fn active_writers() -> &'static Mutex<BTreeSet<PathBuf>> {
    static ACTIVE: OnceLock<Mutex<BTreeSet<PathBuf>>> = OnceLock::new();
    ACTIVE.get_or_init(|| Mutex::new(BTreeSet::new()))
}

struct WriterLockGuard {
    key: PathBuf,
    file: File,
}

impl Drop for WriterLockGuard {
    fn drop(&mut self) {
        let _ = self.file.unlock();
        let mut guard = active_writers()
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        guard.remove(&self.key);
    }
}

fn writer_lock_path(db_path: &Path) -> PathBuf {
    let mut os = db_path.as_os_str().to_os_string();
    os.push(".frob-writer.lock");
    PathBuf::from(os)
}

fn canonical_lock_key(db_path: &Path) -> PathBuf {
    if let Ok(canon) = fs::canonicalize(db_path) {
        return canon;
    }
    if let (Some(parent), Some(name)) = (db_path.parent(), db_path.file_name()) {
        let parent_dir = if parent.as_os_str().is_empty() {
            Path::new(".")
        } else {
            parent
        };
        if let Ok(canon_parent) = fs::canonicalize(parent_dir) {
            return canon_parent.join(name);
        }
    }
    db_path.to_path_buf()
}

fn acquire_writer_lock(
    lock_path: &Path,
    key: &Path,
    timeout: Duration,
    context: &str,
) -> Result<WriterLockGuard, LedgerRefusal> {
    let deadline = Instant::now() + timeout;
    loop {
        let inserted = {
            let mut guard = active_writers()
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            guard.insert(key.to_path_buf())
        };

        if inserted {
            let file = match FsOpenOptions::new()
                .create(true)
                .read(true)
                .write(true)
                .truncate(false)
                .open(lock_path)
            {
                Ok(f) => f,
                Err(e) => {
                    let mut guard = active_writers()
                        .lock()
                        .unwrap_or_else(std::sync::PoisonError::into_inner);
                    guard.remove(key);
                    return Err(LedgerRefusal::io(
                        context,
                        format!("open writer lock `{}`: {e}", lock_path.display()),
                    ));
                }
            };

            match file.try_lock() {
                Ok(()) => {
                    return Ok(WriterLockGuard {
                        key: key.to_path_buf(),
                        file,
                    });
                }
                Err(TryLockError::WouldBlock) => {
                    let mut guard = active_writers()
                        .lock()
                        .unwrap_or_else(std::sync::PoisonError::into_inner);
                    guard.remove(key);
                }
                Err(TryLockError::Error(e)) => {
                    let mut guard = active_writers()
                        .lock()
                        .unwrap_or_else(std::sync::PoisonError::into_inner);
                    guard.remove(key);
                    return Err(LedgerRefusal::io(
                        context,
                        format!("lock `{}`: {e}", lock_path.display()),
                    ));
                }
            }
        }

        if Instant::now() >= deadline {
            return Err(LedgerRefusal::busy(
                context,
                format!(
                    "single-writer lock on `{}` remained held past busy_timeout ({} ms)",
                    key.display(),
                    timeout.as_millis()
                ),
                None,
            ));
        }
        thread::sleep(Duration::from_millis(5));
    }
}

fn ext_table(table: TableId) -> ExtensionTable {
    match table {
        TableId::Meta => ExtensionTable::Requirements,
        TableId::Run => ExtensionTable::Scenarios,
        TableId::Episode => ExtensionTable::Constraints,
    }
}

fn hex_encode(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        out.push(HEX[usize::from(b >> 4)] as char);
        out.push(HEX[usize::from(b & 0x0f)] as char);
    }
    out
}

fn ext_key(table: TableId, key: &[u8]) -> String {
    format!("frob/v0/{}/{}", table.as_str(), hex_encode(key))
}

struct HeaderDoc<'a> {
    schema: u32,
    creation_receipt: &'a str,
}

impl Canonical for HeaderDoc<'_> {
    fn write(&self, w: CanonValue<'_>) -> Result<(), CanonError> {
        w.object(|o| {
            o.str("creation_receipt", self.creation_receipt)?;
            o.int53("schema", i64::from(self.schema))?;
            Ok(())
        })
    }
}

struct CommitSeqDoc {
    commit_seq: u64,
}

impl Canonical for CommitSeqDoc {
    fn write(&self, w: CanonValue<'_>) -> Result<(), CanonError> {
        w.object(|o| {
            o.u64str("commit_seq", self.commit_seq)?;
            Ok(())
        })
    }
}

struct RowIndexDoc<'a> {
    hash_hex: &'a str,
    len: u64,
}

impl Canonical for RowIndexDoc<'_> {
    fn write(&self, w: CanonValue<'_>) -> Result<(), CanonError> {
        w.object(|o| {
            o.str("h", self.hash_hex)?;
            o.u64str("l", self.len)?;
            Ok(())
        })
    }
}

fn parse_header_doc(raw: &str) -> Result<(u32, String), LedgerRefusal> {
    let admitted = json::admit(raw.as_bytes(), &Limits::default()).map_err(|e| {
        LedgerRefusal::integrity(
            "meta.header",
            format!("stored header JSON is malformed: {e}"),
            None,
        )
    })?;
    let v = admitted.value();
    let schema_i = v
        .get("schema")
        .ok_or_else(|| {
            LedgerRefusal::integrity("meta.header", "stored header missing `schema`", None)
        })?
        .as_int53()
        .map_err(|e| {
            LedgerRefusal::integrity("meta.header", format!("invalid `schema`: {e}"), None)
        })?;
    let schema = u32::try_from(schema_i).map_err(|_| {
        LedgerRefusal::integrity(
            "meta.header",
            format!("schema version `{schema_i}` outside u32 range"),
            None,
        )
    })?;
    let receipt = v
        .get("creation_receipt")
        .and_then(|s| s.as_str())
        .ok_or_else(|| {
            LedgerRefusal::integrity(
                "meta.header",
                "stored header missing string `creation_receipt`",
                None,
            )
        })?;
    Ok((schema, receipt.to_owned()))
}

fn parse_commit_seq_doc(raw: &str) -> Result<u64, LedgerRefusal> {
    let admitted = json::admit(raw.as_bytes(), &Limits::default()).map_err(|e| {
        LedgerRefusal::integrity(
            "meta.commit_seq",
            format!("stored commit_seq JSON is malformed: {e}"),
            None,
        )
    })?;
    admitted
        .value()
        .get("commit_seq")
        .ok_or_else(|| {
            LedgerRefusal::integrity(
                "meta.commit_seq",
                "stored commit_seq document missing `commit_seq`",
                None,
            )
        })?
        .as_u64str()
        .map_err(|e| {
            LedgerRefusal::integrity(
                "meta.commit_seq",
                format!("invalid `commit_seq` u64 string: {e}"),
                None,
            )
        })
}

fn parse_row_index_doc(raw: &str) -> Result<(ContentHash, u64), LedgerRefusal> {
    let admitted = json::admit(raw.as_bytes(), &Limits::default()).map_err(|e| {
        LedgerRefusal::integrity(
            "row.index",
            format!("stored row index JSON is malformed: {e}"),
            None,
        )
    })?;
    let v = admitted.value();
    let hash_hex = v
        .get("h")
        .and_then(|x| x.as_str())
        .ok_or_else(|| LedgerRefusal::integrity("row.index", "row index missing `h`", None))?;
    let hash = ContentHash::from_hex(hash_hex).ok_or_else(|| {
        LedgerRefusal::integrity(
            "row.index",
            format!("invalid row content hash `{hash_hex}`"),
            None,
        )
    })?;
    let len = v
        .get("l")
        .ok_or_else(|| LedgerRefusal::integrity("row.index", "row index missing `l`", None))?
        .as_u64str()
        .map_err(|e| {
            LedgerRefusal::integrity("row.index", format!("invalid row length `l`: {e}"), None)
        })?;
    Ok((hash, len))
}

fn map_fs_ledger_err(context: &str, err: LedgerError) -> LedgerRefusal {
    let upstream = err.to_string();
    match err {
        LedgerError::Busy { detail, .. } => LedgerRefusal::busy(context, detail, Some(upstream)),
        LedgerError::FutureSchema { found, supported } => LedgerRefusal::newer_schema(
            u32::try_from(found).unwrap_or(u32::MAX),
            u32::try_from(supported).unwrap_or(u32::MAX),
            Some(upstream),
        ),
        LedgerError::Corrupt { detail, .. } => {
            LedgerRefusal::integrity(context, detail, Some(upstream))
        }
        LedgerError::Open { detail, .. } => {
            if looks_like_corruption(&detail) {
                LedgerRefusal::integrity(context, detail, Some(upstream))
            } else {
                LedgerRefusal::open_failed(context, detail, Some(upstream))
            }
        }
        LedgerError::Sql { detail, .. } => {
            if looks_like_corruption(&detail) {
                LedgerRefusal::integrity(context, detail, Some(upstream))
            } else {
                LedgerRefusal::upstream(context, detail, upstream)
            }
        }
        other => {
            let text = other.to_string();
            if looks_like_corruption(&text) {
                LedgerRefusal::integrity(context, text, Some(upstream))
            } else {
                LedgerRefusal::upstream(context, text, upstream)
            }
        }
    }
}

fn looks_like_corruption(msg: &str) -> bool {
    let lower = msg.to_ascii_lowercase();
    lower.contains("corrupt")
        || lower.contains("malformed")
        || lower.contains("not a database")
        || lower.contains("checksum")
        || lower.contains("hash mismatch")
        || lower.contains("schema mismatch")
        || lower.contains("instance identity")
}

// ---------------------------------------------------------------------------
// Read-only SQLite / WAL file auditing (recovery classification + corruption)
// ---------------------------------------------------------------------------

fn sidecar_path(db_path: &Path, suffix: &str) -> PathBuf {
    let mut os = db_path.as_os_str().to_os_string();
    os.push(suffix);
    PathBuf::from(os)
}

fn read_be_u32(slice: &[u8]) -> u32 {
    u32::from_be_bytes([slice[0], slice[1], slice[2], slice[3]])
}

fn wal_checksum_step(data: &[u8], init: (u32, u32), big_endian: bool) -> (u32, u32) {
    let (mut s0, mut s1) = init;
    for chunk in data.chunks_exact(8) {
        let w0 = if big_endian {
            u32::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3]])
        } else {
            u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]])
        };
        let w1 = if big_endian {
            u32::from_be_bytes([chunk[4], chunk[5], chunk[6], chunk[7]])
        } else {
            u32::from_le_bytes([chunk[4], chunk[5], chunk[6], chunk[7]])
        };
        s0 = s0.wrapping_add(w0).wrapping_add(s1);
        s1 = s1.wrapping_add(w1).wrapping_add(s0);
    }
    (s0, s1)
}

fn is_valid_sqlite_page_size(page_size: u32) -> bool {
    (512..=65536).contains(&page_size) && page_size.is_power_of_two()
}

struct WalInspection {
    frames_checked: u64,
    saw_commit_frame: bool,
    uncommitted_after_commit: bool,
    corruption_findings: Vec<String>,
}

fn inspect_wal_bytes(wal_bytes: &[u8], label: &str) -> WalInspection {
    let mut out = WalInspection {
        frames_checked: 0,
        saw_commit_frame: false,
        uncommitted_after_commit: false,
        corruption_findings: Vec::new(),
    };
    if wal_bytes.is_empty() {
        return out;
    }
    if wal_bytes.len() < WAL_HEADER_SIZE {
        out.uncommitted_after_commit = true;
        out.corruption_findings.push(format!(
            "{label}: truncated WAL header ({} bytes < {WAL_HEADER_SIZE})",
            wal_bytes.len()
        ));
        return out;
    }

    let magic = read_be_u32(&wal_bytes[0..4]);
    if magic != WAL_MAGIC_LE && magic != WAL_MAGIC_BE {
        out.corruption_findings
            .push(format!("{label}: invalid WAL magic {magic:#010x}"));
        return out;
    }
    let big_endian = magic == WAL_MAGIC_BE;
    let page_size_u32 = read_be_u32(&wal_bytes[8..12]);
    if !is_valid_sqlite_page_size(page_size_u32) {
        out.corruption_findings
            .push(format!("{label}: invalid WAL page size {page_size_u32}"));
        return out;
    }
    let page_size = page_size_u32 as usize;

    let hdr_salt1 = read_be_u32(&wal_bytes[16..20]);
    let hdr_salt2 = read_be_u32(&wal_bytes[20..24]);
    let hdr_stored = (
        read_be_u32(&wal_bytes[24..28]),
        read_be_u32(&wal_bytes[28..32]),
    );
    let hdr_calc = wal_checksum_step(&wal_bytes[0..24], (0, 0), big_endian);
    if hdr_stored != hdr_calc {
        out.corruption_findings.push(format!(
            "{label}: WAL header checksum mismatch (stored {hdr_stored:?}, computed {hdr_calc:?})"
        ));
        return out;
    }

    let frame_size = WAL_FRAME_HEADER_SIZE + page_size;
    let mut offset = WAL_HEADER_SIZE;
    let mut running = hdr_calc;
    let mut frame_index: u64 = 0;

    while offset + frame_size <= wal_bytes.len() {
        let frame = &wal_bytes[offset..offset + frame_size];
        let page_no = read_be_u32(&frame[0..4]);
        let db_size_after_commit = read_be_u32(&frame[4..8]);
        let salt1 = read_be_u32(&frame[8..12]);
        let salt2 = read_be_u32(&frame[12..16]);

        // Frames from an earlier checkpointed generation carry different salts;
        // zeroed/unallocated trailing space carries page_no == 0.
        if page_no == 0 || salt1 != hdr_salt1 || salt2 != hdr_salt2 {
            break;
        }

        let stored = (read_be_u32(&frame[16..20]), read_be_u32(&frame[20..24]));
        let step1 = wal_checksum_step(&frame[0..8], running, big_endian);
        let computed = wal_checksum_step(&frame[WAL_FRAME_HEADER_SIZE..], step1, big_endian);
        out.frames_checked = out.frames_checked.saturating_add(1);

        if stored != computed {
            // Distinguish a torn trailing uncommitted frame (which SIGKILL can
            // produce during an incomplete write) from corruption inside a
            // committed transaction (where this frame or any later current-salt
            // frame carries db_size_after_commit > 0).
            let mut committed_affected = db_size_after_commit > 0;
            if !committed_affected {
                let mut probe_off = offset + frame_size;
                while probe_off + frame_size <= wal_bytes.len() {
                    let f = &wal_bytes[probe_off..probe_off + frame_size];
                    let pno = read_be_u32(&f[0..4]);
                    let s1 = read_be_u32(&f[8..12]);
                    let s2 = read_be_u32(&f[12..16]);
                    if pno == 0 || s1 != hdr_salt1 || s2 != hdr_salt2 {
                        break;
                    }
                    if read_be_u32(&f[4..8]) > 0 {
                        committed_affected = true;
                        break;
                    }
                    probe_off += frame_size;
                }
            }
            if committed_affected {
                out.corruption_findings.push(format!(
                    "{label}: WAL frame {frame_index} (page {page_no}) checksum mismatch \
                     (stored {stored:?}, computed {computed:?})"
                ));
            } else {
                out.uncommitted_after_commit = true;
            }
            break;
        }

        running = computed;
        if db_size_after_commit > 0 {
            out.saw_commit_frame = true;
            out.uncommitted_after_commit = false;
        } else {
            out.uncommitted_after_commit = true;
        }
        offset += frame_size;
        frame_index = frame_index.saturating_add(1);
    }

    if offset < wal_bytes.len() && wal_bytes.len() - offset < frame_size {
        // Partial trailing frame bytes beyond the last complete frame.
        out.uncommitted_after_commit = true;
    }

    out
}

fn validate_sqlite_main_header(db_path: &Path) -> Result<(), LedgerRefusal> {
    if !db_path.exists() {
        return Ok(());
    }
    let bytes = fs::read(db_path).map_err(|e| {
        LedgerRefusal::io(
            "sqlite.header",
            format!("read `{}`: {e}", db_path.display()),
        )
    })?;
    if bytes.is_empty() {
        return Ok(());
    }
    if bytes.len() < 100 {
        return Err(LedgerRefusal::integrity(
            "sqlite.header",
            format!(
                "database file `{}` is truncated ({} bytes < 100-byte SQLite header)",
                db_path.display(),
                bytes.len()
            ),
            None,
        ));
    }
    if &bytes[0..16] != SQLITE_HEADER_MAGIC {
        return Err(LedgerRefusal::integrity(
            "sqlite.header",
            format!(
                "database file `{}` failed SQLite magic header check",
                db_path.display()
            ),
            None,
        ));
    }
    let raw_ps = u16::from_be_bytes([bytes[16], bytes[17]]);
    let page_size = if raw_ps == 1 {
        65536u32
    } else {
        u32::from(raw_ps)
    };
    if !is_valid_sqlite_page_size(page_size) {
        return Err(LedgerRefusal::integrity(
            "sqlite.header",
            format!(
                "database file `{}` declares invalid page size {page_size}",
                db_path.display()
            ),
            None,
        ));
    }
    Ok(())
}

fn inspect_recovery_before_open(
    db_path: &Path,
) -> Result<(RecoveryReport, Vec<String>), LedgerRefusal> {
    if !db_path.exists() {
        return Ok((RecoveryReport::Clean, Vec::new()));
    }
    validate_sqlite_main_header(db_path)?;

    let journal = sidecar_path(db_path, "-journal");
    if let Ok(meta) = fs::metadata(&journal)
        && meta.len() > 0
    {
        return Ok((RecoveryReport::RolledBackIncompleteTxn, Vec::new()));
    }

    let mut report = RecoveryReport::Clean;
    let mut pre_open_findings = Vec::new();
    for suffix in ["-wal", ".fsqlite-wal"] {
        let wal = sidecar_path(db_path, suffix);
        if wal.exists() {
            let bytes = fs::read(&wal).map_err(|e| {
                LedgerRefusal::io("open.wal_inspect", format!("read `{}`: {e}", wal.display()))
            })?;
            if !bytes.is_empty() {
                let insp = inspect_wal_bytes(&bytes, suffix);
                pre_open_findings.extend(insp.corruption_findings);
                if insp.uncommitted_after_commit {
                    report = RecoveryReport::RolledBackIncompleteTxn;
                } else if insp.saw_commit_frame && report == RecoveryReport::Clean {
                    report = RecoveryReport::ReplayedLog;
                }
            }
        }
    }

    Ok((report, pre_open_findings))
}

// ---------------------------------------------------------------------------
// FsqliteLedger + Txn
// ---------------------------------------------------------------------------

/// In-process FrankenSQLite-backed [`Ledger`] adapter (`BD-009`, `INT-FSQL-01`).
pub struct FsqliteLedger {
    inner: FsLedger,
    path: PathBuf,
    lock_path: PathBuf,
    lock_key: PathBuf,
    busy_timeout: Duration,
    schema: SchemaVersion,
    recovered: RecoveryReport,
    pre_open_wal_findings: Vec<String>,
    cached_commit_seq: u64,
}

impl std::fmt::Debug for FsqliteLedger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FsqliteLedger")
            .field("path", &self.path)
            .field("schema", &self.schema)
            .field("recovered", &self.recovered)
            .field("cached_commit_seq", &self.cached_commit_seq)
            .finish_non_exhaustive()
    }
}

impl FsqliteLedger {
    /// Open or create the ledger at `path` for `schema`.
    pub fn open(
        path: &Path,
        schema: SchemaVersion,
        opts: OpenOptions,
    ) -> Result<Self, LedgerRefusal> {
        Self::open_with_recovery_checkpoint(path, schema, opts, |_| Ok(()))
    }

    pub(super) fn open_with_recovery_checkpoint(
        path: &Path,
        schema: SchemaVersion,
        opts: OpenOptions,
        checkpoint: impl FnOnce(RecoveryReport) -> std::io::Result<()>,
    ) -> Result<Self, LedgerRefusal> {
        let path_str = path.to_str().ok_or_else(|| {
            LedgerRefusal::invalid_input("open", "ledger path must be valid UTF-8")
        })?;
        if path_str.trim().is_empty() {
            return Err(LedgerRefusal::invalid_input("open", "ledger path is empty"));
        }
        if schema.as_u32() == 0 {
            return Err(LedgerRefusal::invalid_input(
                "open",
                "schema version must be >= 1",
            ));
        }

        let existed = path.exists();
        if !existed && !opts.create_if_missing {
            return Err(LedgerRefusal::open_failed(
                "open",
                format!(
                    "ledger `{}` does not exist and create_if_missing is false",
                    path.display()
                ),
                None,
            ));
        }

        if let Some(parent) = path.parent()
            && !parent.as_os_str().is_empty()
            && !parent.exists()
        {
            if opts.create_if_missing {
                fs::create_dir_all(parent).map_err(|e| {
                    LedgerRefusal::io(
                        "open",
                        format!("create parent directory `{}`: {e}", parent.display()),
                    )
                })?;
            } else {
                return Err(LedgerRefusal::open_failed(
                    "open",
                    format!("parent directory `{}` does not exist", parent.display()),
                    None,
                ));
            }
        }

        // 1. Inspect existing on-disk header & WAL before opening (zero writes).
        let (recovered, pre_open_wal_findings) = inspect_recovery_before_open(path)?;
        checkpoint(recovered)
            .map_err(|e| LedgerRefusal::io("open.recovery_checkpoint", e.to_string()))?;

        // 2. Open underlying FrankenSQLite-backed ledger (refuses newer engine
        //    `PRAGMA user_version` BEFORE applying pragmas or migrating).
        let inner = FsLedger::open(path_str).map_err(|e| map_fs_ledger_err("open", e))?;

        let lock_path = writer_lock_path(path);
        let lock_key = canonical_lock_key(path);

        // 3. Read our M0 header READ-ONLY before writing anything.
        //    If the file holds a newer SchemaVersion, refuse immediately with
        //    zero writes to durable state.
        let existing_header = inner
            .get_extension(ExtensionTable::Requirements, CONTROL_HEADER_KEY)
            .map_err(|e| map_fs_ledger_err("open.read_header", e))?;

        let cached_commit_seq = if let Some(raw_header) = existing_header {
            let (stored_schema, _receipt) = parse_header_doc(&raw_header)?;
            if stored_schema > schema.as_u32() {
                return Err(LedgerRefusal::newer_schema(
                    stored_schema,
                    schema.as_u32(),
                    None,
                ));
            }
            match inner
                .get_extension(ExtensionTable::Requirements, CONTROL_COMMIT_SEQ_KEY)
                .map_err(|e| map_fs_ledger_err("open.read_commit_seq", e))?
            {
                Some(raw_seq) => parse_commit_seq_doc(&raw_seq)?,
                None => 0,
            }
        } else {
            // Brand-new ledger initialization: briefly hold the single-writer lock
            // while stamping M0 header and commit_seq = 0 atomically.
            let _init_guard =
                acquire_writer_lock(&lock_path, &lock_key, opts.busy_timeout, "open.init")?;
            // Re-check after acquiring lock in case a concurrent initializer raced.
            if let Some(raw_header) = inner
                .get_extension(ExtensionTable::Requirements, CONTROL_HEADER_KEY)
                .map_err(|e| map_fs_ledger_err("open.recheck_header", e))?
            {
                let (stored_schema, _receipt) = parse_header_doc(&raw_header)?;
                if stored_schema > schema.as_u32() {
                    return Err(LedgerRefusal::newer_schema(
                        stored_schema,
                        schema.as_u32(),
                        None,
                    ));
                }
                match inner
                    .get_extension(ExtensionTable::Requirements, CONTROL_COMMIT_SEQ_KEY)
                    .map_err(|e| map_fs_ledger_err("open.read_commit_seq", e))?
                {
                    Some(raw_seq) => parse_commit_seq_doc(&raw_seq)?,
                    None => 0,
                }
            } else {
                let receipt = opts
                    .creation_receipt
                    .unwrap_or_else(|| BUILT_UNDER.receipt.to_string());
                let header_bytes = canon::to_canonical(&HeaderDoc {
                    schema: schema.as_u32(),
                    creation_receipt: &receipt,
                })
                .map_err(|e| LedgerRefusal::invalid_input("open.init_header", e.to_string()))?;
                let seq_bytes = canon::to_canonical(&CommitSeqDoc { commit_seq: 0 })
                    .map_err(|e| LedgerRefusal::invalid_input("open.init_seq", e.to_string()))?;

                inner
                    .begin()
                    .map_err(|e| map_fs_ledger_err("open.init_begin", e))?;
                let init_res = (|| -> Result<(), LedgerRefusal> {
                    inner
                        .put_extension(
                            ExtensionTable::Requirements,
                            CONTROL_HEADER_KEY,
                            header_bytes.as_str(),
                        )
                        .map_err(|e| map_fs_ledger_err("open.init_header", e))?;
                    inner
                        .put_extension(
                            ExtensionTable::Requirements,
                            CONTROL_COMMIT_SEQ_KEY,
                            seq_bytes.as_str(),
                        )
                        .map_err(|e| map_fs_ledger_err("open.init_seq", e))?;
                    Ok(())
                })();
                match init_res {
                    Ok(()) => {
                        if let Err(e) = inner.commit() {
                            let _ = inner.rollback();
                            return Err(map_fs_ledger_err("open.init_commit", e));
                        }
                    }
                    Err(err) => {
                        let _ = inner.rollback();
                        return Err(err);
                    }
                }
                0
            }
        };

        Ok(Self {
            inner,
            path: path.to_path_buf(),
            lock_path,
            lock_key,
            busy_timeout: opts.busy_timeout,
            schema,
            recovered,
            pre_open_wal_findings,
            cached_commit_seq,
        })
    }

    /// Begin a single-writer transaction.
    pub fn begin(&mut self) -> Result<Txn<'_>, LedgerRefusal> {
        let writer_lock =
            acquire_writer_lock(&self.lock_path, &self.lock_key, self.busy_timeout, "begin")?;

        self.inner
            .begin()
            .map_err(|e| map_fs_ledger_err("begin", e))?;

        // Read authoritative commit sequence inside the transaction so cross-handle
        // or cross-process commits remain strictly monotone with zero gaps.
        let current_seq = match self
            .inner
            .get_extension(ExtensionTable::Requirements, CONTROL_COMMIT_SEQ_KEY)
        {
            Ok(Some(raw)) => match parse_commit_seq_doc(&raw) {
                Ok(seq) => seq,
                Err(err) => {
                    let _ = self.inner.rollback();
                    return Err(err);
                }
            },
            Ok(None) => 0,
            Err(e) => {
                let _ = self.inner.rollback();
                return Err(map_fs_ledger_err("begin.read_commit_seq", e));
            }
        };
        self.cached_commit_seq = current_seq;

        Ok(Txn {
            ledger: self,
            committed: false,
            base_seq: current_seq,
            _writer_lock: writer_lock,
        })
    }

    /// Perform a read-only point lookup outside an explicit write transaction.
    pub fn get_committed(
        &self,
        table: TableId,
        key: &[u8],
    ) -> Result<Option<Vec<u8>>, LedgerRefusal> {
        read_row(&self.inner, table, key)
    }

    /// Last committed sequence number currently visible in this ledger.
    pub fn latest_commit_seq(&self) -> Result<CommitSeq, LedgerRefusal> {
        match self
            .inner
            .get_extension(ExtensionTable::Requirements, CONTROL_COMMIT_SEQ_KEY)
            .map_err(|e| map_fs_ledger_err("latest_commit_seq", e))?
        {
            Some(raw) => Ok(CommitSeq::new(parse_commit_seq_doc(&raw)?)),
            None => Ok(CommitSeq::new(0)),
        }
    }

    /// Run structural, WAL-checksum, metadata, and BLAKE3 content integrity checks.
    pub fn integrity_check(&self) -> Result<IntegrityReport, LedgerRefusal> {
        let mut findings = self.pre_open_wal_findings.clone();
        let mut wal_frames_checked: u64 = 0;

        // 1. Validate main SQLite database header on disk.
        if let Err(err) = validate_sqlite_main_header(&self.path) {
            findings.push(err.to_string());
        }

        // 2. Audit any active WAL sidecar's header + frame cumulative checksums
        //    so a flipped byte in a committed WAL frame is reported rather than
        //    silently truncated by SQLite recovery.
        for suffix in ["-wal", ".fsqlite-wal"] {
            let wal = sidecar_path(&self.path, suffix);
            if wal.exists() {
                match fs::read(&wal) {
                    Ok(bytes) => {
                        let insp = inspect_wal_bytes(&bytes, suffix);
                        wal_frames_checked = wal_frames_checked.saturating_add(insp.frames_checked);
                        findings.extend(insp.corruption_findings);
                    }
                    Err(e) => {
                        findings.push(format!("failed reading WAL sidecar `{suffix}`: {e}"));
                    }
                }
            }
        }

        // 3. Structural & referential hygiene check in the underlying ledger.
        let lint = self
            .inner
            .lint()
            .map_err(|e| map_fs_ledger_err("integrity_check.lint", e))?;
        if !lint.is_clean() {
            findings.push(format!(
                "underlying ledger lint reported anomalies: {lint:?}"
            ));
        }

        // 4. Re-hash every stored row payload against its BLAKE3 content hash.
        let art = self
            .inner
            .verify_artifact_integrity()
            .map_err(|e| map_fs_ledger_err("integrity_check.artifacts", e))?;
        let artifacts_checked = art.checked;
        let corrupted_artifacts = art.corrupted;
        if !corrupted_artifacts.is_empty() {
            findings.push(format!(
                "{} stored row payload(s) failed BLAKE3 verification",
                corrupted_artifacts.len()
            ));
        }

        // 5. Verify M0 control records exist and parse cleanly.
        match self
            .inner
            .get_extension(ExtensionTable::Requirements, CONTROL_HEADER_KEY)
            .map_err(|e| map_fs_ledger_err("integrity_check.header", e))?
        {
            Some(raw) => {
                if let Err(err) = parse_header_doc(&raw) {
                    findings.push(err.to_string());
                }
            }
            None => findings.push("missing M0 control header record".to_owned()),
        }
        match self
            .inner
            .get_extension(ExtensionTable::Requirements, CONTROL_COMMIT_SEQ_KEY)
            .map_err(|e| map_fs_ledger_err("integrity_check.commit_seq", e))?
        {
            Some(raw) => {
                if let Err(err) = parse_commit_seq_doc(&raw) {
                    findings.push(err.to_string());
                }
            }
            None => findings.push("missing M0 control commit_seq record".to_owned()),
        }

        let clean = findings.is_empty() && corrupted_artifacts.is_empty();
        Ok(IntegrityReport {
            clean,
            artifacts_checked,
            corrupted_artifacts,
            wal_frames_checked,
            findings,
        })
    }

    /// Report what [`FsqliteLedger::open`] observed on disk.
    #[must_use]
    pub const fn recovered(&self) -> RecoveryReport {
        self.recovered
    }

    /// Durability settings active on the engine at the pin.
    #[must_use]
    pub const fn durability_config(&self) -> DurabilityConfig {
        PIN_DURABILITY
    }

    /// Schema identifier reported by the M0 ledger adapter.
    #[must_use]
    pub const fn ledger_schema(&self) -> &'static str {
        LEDGER_SCHEMA_ID
    }
}

impl Ledger for FsqliteLedger {
    type Txn<'a> = Txn<'a>;

    fn open(path: &Path, schema: SchemaVersion, opts: OpenOptions) -> Result<Self, LedgerRefusal> {
        Self::open(path, schema, opts)
    }

    fn begin(&mut self) -> Result<Self::Txn<'_>, LedgerRefusal> {
        Self::begin(self)
    }

    fn integrity_check(&self) -> Result<IntegrityReport, LedgerRefusal> {
        Self::integrity_check(self)
    }

    fn recovered(&self) -> RecoveryReport {
        Self::recovered(self)
    }

    fn durability_config(&self) -> DurabilityConfig {
        Self::durability_config(self)
    }
}

fn read_row(
    inner: &FsLedger,
    table: TableId,
    key: &[u8],
) -> Result<Option<Vec<u8>>, LedgerRefusal> {
    let ek = ext_key(table, key);
    let Some(raw_index) = inner
        .get_extension(ext_table(table), &ek)
        .map_err(|e| map_fs_ledger_err("get.index", e))?
    else {
        return Ok(None);
    };
    let (hash, expected_len) = parse_row_index_doc(&raw_index)?;
    let bytes = inner
        .get_artifact(&hash)
        .map_err(|e| map_fs_ledger_err("get.artifact", e))?
        .ok_or_else(|| {
            LedgerRefusal::integrity(
                "get.artifact",
                format!(
                    "indexed row `{ek}` references missing content hash {}",
                    hash.to_hex()
                ),
                None,
            )
        })?;
    if bytes.len() as u64 != expected_len {
        return Err(LedgerRefusal::integrity(
            "get.artifact",
            format!(
                "indexed row `{ek}` length mismatch (expected {expected_len}, read {})",
                bytes.len()
            ),
            None,
        ));
    }
    Ok(Some(bytes))
}

/// Active single-writer transaction handle on [`FsqliteLedger`].
///
/// Dropping `Txn` without calling [`Txn::commit`] rolls back the underlying
/// database transaction and releases the single-writer lock.
pub struct Txn<'a> {
    ledger: &'a mut FsqliteLedger,
    committed: bool,
    base_seq: u64,
    _writer_lock: WriterLockGuard,
}

impl Txn<'_> {
    /// Stage canonical `row` bytes under `(table, key)`.
    pub fn put(&mut self, table: TableId, key: &[u8], row: &[u8]) -> Result<(), LedgerRefusal> {
        let receipt = self
            .ledger
            .inner
            .put_artifact(ROW_ARTIFACT_KIND, row, None)
            .map_err(|e| map_fs_ledger_err("txn.put_artifact", e))?;
        let hash_hex = receipt.hash.to_hex();
        let body = canon::to_canonical(&RowIndexDoc {
            hash_hex: &hash_hex,
            len: receipt.len,
        })
        .map_err(|e| LedgerRefusal::invalid_input("txn.put_index_json", e.to_string()))?;
        self.ledger
            .inner
            .put_extension(ext_table(table), &ext_key(table, key), body.as_str())
            .map_err(|e| map_fs_ledger_err("txn.put_extension", e))?;
        Ok(())
    }

    /// Read the canonical row bytes currently visible for `(table, key)`.
    pub fn get(&self, table: TableId, key: &[u8]) -> Result<Option<Vec<u8>>, LedgerRefusal> {
        read_row(&self.ledger.inner, table, key)
    }

    /// Durably commit this transaction, returning its monotone [`CommitSeq`].
    pub fn commit(mut self) -> Result<CommitSeq, LedgerRefusal> {
        let next_seq = self.base_seq.checked_add(1).ok_or_else(|| {
            LedgerRefusal::invalid_input("txn.commit", "commit sequence overflowed u64::MAX")
        })?;
        let seq_body = canon::to_canonical(&CommitSeqDoc {
            commit_seq: next_seq,
        })
        .map_err(|e| LedgerRefusal::invalid_input("txn.commit_seq_json", e.to_string()))?;

        if let Err(e) = self.ledger.inner.put_extension(
            ExtensionTable::Requirements,
            CONTROL_COMMIT_SEQ_KEY,
            seq_body.as_str(),
        ) {
            let _ = self.ledger.inner.rollback();
            self.committed = true;
            return Err(map_fs_ledger_err("txn.commit_seq_write", e));
        }

        if let Err(e) = self.ledger.inner.commit() {
            let _ = self.ledger.inner.rollback();
            self.committed = true;
            return Err(map_fs_ledger_err("txn.commit", e));
        }

        self.committed = true;
        self.ledger.cached_commit_seq = next_seq;
        Ok(CommitSeq::new(next_seq))
    }
}

impl LedgerTxn for Txn<'_> {
    fn put(&mut self, table: TableId, key: &[u8], row: &[u8]) -> Result<(), LedgerRefusal> {
        Txn::put(self, table, key, row)
    }

    fn get(&self, table: TableId, key: &[u8]) -> Result<Option<Vec<u8>>, LedgerRefusal> {
        Txn::get(self, table, key)
    }

    fn commit(self) -> Result<CommitSeq, LedgerRefusal> {
        Txn::commit(self)
    }
}

impl Drop for Txn<'_> {
    fn drop(&mut self) {
        if !self.committed && self.ledger.inner.in_transaction() {
            let _ = self.ledger.inner.rollback();
        }
    }
}
