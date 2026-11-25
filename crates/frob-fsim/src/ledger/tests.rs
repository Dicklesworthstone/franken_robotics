#![forbid(unsafe_code)]

//! Unit tests for `frob_fsim::ledger` (`BD-009`, `INT-FSQL-01`).
//!
//! Scratch databases are placed under `${BIG_SCRATCH:-$TMPDIR}/frob-BrownHeron/`
//! and never deleted (`COMMON.md` 9b).

use std::fs::{self, OpenOptions as FsOpenOptions};
use std::io::{Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use super::{
    CommitSeq, FsqliteLedger, Ledger, LedgerRefusalKind, LedgerTxn, OpenOptions, SchemaVersion,
    TableId,
};

static SCRATCH_SEQ: AtomicU64 = AtomicU64::new(0);

fn scratch_db(tag: &str) -> PathBuf {
    let base = std::env::var_os("BIG_SCRATCH")
        .map(PathBuf::from)
        .unwrap_or_else(std::env::temp_dir);
    let seq = SCRATCH_SEQ.fetch_add(1, Ordering::Relaxed);
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_nanos());
    let dir = base.join("frob-BrownHeron").join(format!(
        "bd009-unit-{tag}-{}-{nanos}-{seq}",
        std::process::id()
    ));
    fs::create_dir_all(&dir).expect("create unit scratch directory");
    dir.join("ledger.db")
}

fn durable_snapshot(db_path: &Path) -> Vec<(String, Vec<u8>)> {
    let dir = db_path.parent().expect("db path has parent");
    let stem = db_path
        .file_name()
        .expect("db path has file name")
        .to_string_lossy()
        .into_owned();
    let mut entries: Vec<(String, Vec<u8>)> = fs::read_dir(dir)
        .expect("read scratch dir")
        .flatten()
        .filter_map(|e| {
            let name = e.file_name().to_string_lossy().into_owned();
            // Exclude volatile lock/shm scratch files (only compare durable db + wal).
            if name.starts_with(&stem)
                && !name.ends_with("-shm")
                && !name.ends_with(".fsqlite-shm")
                && !name.ends_with(".frob-writer.lock")
            {
                fs::read(e.path()).ok().map(|b| (name, b))
            } else {
                None
            }
        })
        .collect();
    entries.sort_by(|a, b| a.0.cmp(&b.0));
    entries
}

fn flip_one_byte_at(path: &Path, offset: u64) {
    let mut f = FsOpenOptions::new()
        .read(true)
        .write(true)
        .open(path)
        .expect("open file to flip one byte");
    let mut orig = [0u8; 1];
    f.seek(SeekFrom::Start(offset)).expect("seek read");
    std::io::Read::read_exact(&mut f, &mut orig).expect("read target byte");
    orig[0] ^= 0x01;
    f.seek(SeekFrom::Start(offset)).expect("seek write");
    f.write_all(&orig).expect("write flipped byte");
    f.sync_all().expect("sync flipped byte");
}

#[test]
fn open_write_read() {
    let db = scratch_db("open-write-read");
    let rows: Vec<(TableId, Vec<u8>, Vec<u8>)> = (0u8..8)
        .map(|i| {
            let table = if i == 0 {
                TableId::Run
            } else if i == 7 {
                TableId::Meta
            } else {
                TableId::Episode
            };
            let key = format!("row-key-{i:02}").into_bytes();
            let val = format!(
                "canonical-payload-{i:02}-{:064x}",
                u64::from(i) * 0x9E37_79B9
            )
            .into_bytes();
            (table, key, val)
        })
        .collect();

    {
        let mut ledger = FsqliteLedger::open(&db, SchemaVersion::M0, OpenOptions::default())
            .expect("open fresh ledger");
        assert_eq!(ledger.durability_config().journal_mode, "WAL");
        assert_eq!(ledger.durability_config().synchronous, "FULL");

        let mut txn = ledger.begin().expect("begin transaction");
        for (table, key, val) in &rows {
            txn.put(*table, key, val).expect("put row");
        }
        // Read-your-own-writes inside the open transaction.
        for (table, key, val) in &rows {
            assert_eq!(
                txn.get(*table, key).expect("get inside txn").as_deref(),
                Some(val.as_slice())
            );
        }
        let seq = txn.commit().expect("commit transaction");
        assert_eq!(seq, CommitSeq::new(1));
        assert_eq!(seq.to_json_string(), "1");
    }

    // Reopen and verify all 8 rows read back byte-identical and integrity_check is clean.
    let reopened =
        FsqliteLedger::open(&db, SchemaVersion::M0, OpenOptions::default()).expect("reopen ledger");
    let report = reopened
        .integrity_check()
        .expect("integrity check after reopen");
    assert!(
        report.is_clean(),
        "expected clean integrity report, got {report:?}"
    );
    for (table, key, val) in &rows {
        let got = reopened
            .get_committed(*table, key)
            .expect("read committed row")
            .expect("row must exist after reopen");
        assert_eq!(
            &got,
            val,
            "row bytes must match byte-for-byte for {:?}",
            String::from_utf8_lossy(key)
        );
    }
}

#[test]
fn rollback_on_drop() {
    let db = scratch_db("rollback-on-drop");
    let mut ledger =
        FsqliteLedger::open(&db, SchemaVersion::M0, OpenOptions::default()).expect("open ledger");

    {
        let mut txn = ledger.begin().expect("begin doomed txn");
        txn.put(TableId::Run, b"doomed-run", b"must-not-persist")
            .expect("put doomed run");
        txn.put(TableId::Episode, b"doomed-ep", b"also-must-not-persist")
            .expect("put doomed episode");
        // Drop `txn` without calling `commit()`.
    }

    assert_eq!(
        ledger
            .get_committed(TableId::Run, b"doomed-run")
            .expect("read doomed run"),
        None,
        "dropped transaction must leave no row in run table"
    );
    assert_eq!(
        ledger
            .get_committed(TableId::Episode, b"doomed-ep")
            .expect("read doomed episode"),
        None,
        "dropped transaction must leave no row in episode table"
    );
    assert_eq!(
        ledger.latest_commit_seq().expect("latest seq"),
        CommitSeq::new(0),
        "rolled-back transaction must not advance commit_seq"
    );

    // Reopen to confirm nothing leaked to durable storage either.
    drop(ledger);
    let reopened =
        FsqliteLedger::open(&db, SchemaVersion::M0, OpenOptions::default()).expect("reopen");
    assert_eq!(
        reopened
            .get_committed(TableId::Run, b"doomed-run")
            .expect("read"),
        None
    );
    assert_eq!(
        reopened
            .get_committed(TableId::Episode, b"doomed-ep")
            .expect("read"),
        None
    );
    assert_eq!(
        reopened.latest_commit_seq().expect("seq"),
        CommitSeq::new(0)
    );
}

#[test]
fn commit_seq_monotone() {
    let db = scratch_db("commit-seq-monotone");
    let mut ledger =
        FsqliteLedger::open(&db, SchemaVersion::M0, OpenOptions::default()).expect("open ledger");

    for expected in 1u64..=1000 {
        let mut txn = ledger.begin().expect("begin");
        let key = format!("k{expected:04}").into_bytes();
        let val = expected.to_le_bytes();
        txn.put(TableId::Episode, &key, &val).expect("put");
        let seq = txn.commit().expect("commit");
        assert_eq!(
            seq,
            CommitSeq::new(expected),
            "commit_seq must advance monotonically with no gap"
        );
    }

    assert_eq!(
        ledger.latest_commit_seq().expect("latest seq"),
        CommitSeq::new(1000)
    );
    drop(ledger);

    let reopened =
        FsqliteLedger::open(&db, SchemaVersion::M0, OpenOptions::default()).expect("reopen");
    assert_eq!(
        reopened.latest_commit_seq().expect("reopened seq"),
        CommitSeq::new(1000)
    );
}

#[test]
fn newer_schema_refused() {
    let db = scratch_db("newer-schema-refused");
    {
        let mut newer = FsqliteLedger::open(&db, SchemaVersion::new(2), OpenOptions::default())
            .expect("create v2 ledger");
        let mut txn = newer.begin().expect("begin v2");
        txn.put(TableId::Run, b"run-v2", b"written-by-v2")
            .expect("put v2");
        txn.commit().expect("commit v2");
    }
    thread::sleep(Duration::from_millis(30));

    let before = durable_snapshot(&db);
    let err = FsqliteLedger::open(&db, SchemaVersion::M0, OpenOptions::default())
        .expect_err("older binary (v1) must refuse v2 ledger");
    assert!(
        err.is_newer_schema(),
        "expected NewerSchema refusal, got: {err:?}"
    );
    assert_eq!(
        err.kind,
        LedgerRefusalKind::NewerSchema {
            found: 2,
            supported: 1
        }
    );
    assert!(!err.is_retryable(), "NewerSchema must never be retryable");

    let after = durable_snapshot(&db);
    assert_eq!(
        before, after,
        "refused newer-schema open must leave durable files byte-identical"
    );
}

#[test]
fn corruption_detected() {
    // Subcase A: flip one byte inside a committed data page containing our written row payload.
    let db_data = scratch_db("corruption-data-page");
    let sentinel = b"FROB_CORRUPTION_SENTINEL_PAYLOAD_BD009_INT_FSQL_01_DATA_PAGE";
    {
        let mut ledger = FsqliteLedger::open(&db_data, SchemaVersion::M0, OpenOptions::default())
            .expect("open ledger for data-page corruption test");
        let mut txn = ledger.begin().expect("begin");
        for i in 0u8..8 {
            let mut payload = Vec::with_capacity(512);
            payload.extend_from_slice(sentinel);
            payload.push(i);
            payload.resize(512, i.wrapping_mul(37).wrapping_add(11));
            txn.put(TableId::Episode, format!("ep-{i}").as_bytes(), &payload)
                .expect("put row");
        }
        txn.commit().expect("commit");
    }
    // Wait briefly for detached worker thread shutdown to settle on disk.
    thread::sleep(Duration::from_millis(50));

    let wal_path = {
        let mut os = db_data.as_os_str().to_os_string();
        os.push("-wal");
        PathBuf::from(os)
    };
    let (target_file, offset) = [db_data.as_path(), wal_path.as_path()]
        .into_iter()
        .find_map(|p| {
            let bytes = fs::read(p).ok()?;
            let pos = bytes.windows(sentinel.len()).position(|w| w == sentinel)?;
            Some((p.to_path_buf(), pos as u64 + 12))
        })
        .expect("committed sentinel payload must exist on disk in db or wal file");
    flip_one_byte_at(&target_file, offset);

    match FsqliteLedger::open(&db_data, SchemaVersion::M0, OpenOptions::default()) {
        Err(ref err) => {
            assert!(
                !err.is_retryable(),
                "corruption refusal on open must not be retryable: {err:?}"
            );
        }
        Ok(reopened) => match reopened.integrity_check() {
            Err(ref err) => {
                assert!(
                    err.is_integrity(),
                    "expected Integrity refusal, got: {err:?}"
                );
                assert!(
                    !err.is_retryable(),
                    "Integrity refusal must never be retryable"
                );
            }
            Ok(report) => {
                assert!(
                    !report.is_clean(),
                    "flipped byte in committed data page must be reported, got clean report: {report:?}"
                );
            }
        },
    }

    // Subcase B: flip one byte in the middle of the active closed ledger file.
    let db_mid = scratch_db("corruption-mid-file");
    {
        let mut ledger = FsqliteLedger::open(&db_mid, SchemaVersion::M0, OpenOptions::default())
            .expect("open ledger for mid-file corruption test");
        let mut txn = ledger.begin().expect("begin");
        for i in 0u8..8 {
            let payload = vec![0xA5u8 ^ i; 1024];
            txn.put(TableId::Episode, format!("mid-{i}").as_bytes(), &payload)
                .expect("put mid row");
        }
        txn.commit().expect("commit");
    }
    thread::sleep(Duration::from_millis(50));

    let wal_mid = {
        let mut os = db_mid.as_os_str().to_os_string();
        os.push("-wal");
        PathBuf::from(os)
    };
    let active_file = if fs::metadata(&wal_mid).map_or(0, |m| m.len()) > 32 {
        wal_mid
    } else {
        db_mid.clone()
    };
    let len = fs::metadata(&active_file).expect("stat active file").len();
    assert!(len > 64, "active ledger file must be non-empty");
    flip_one_byte_at(&active_file, len / 2);

    match FsqliteLedger::open(&db_mid, SchemaVersion::M0, OpenOptions::default()) {
        Err(ref err) => {
            assert!(
                !err.is_retryable(),
                "mid-file corruption refusal on open must not be retryable: {err:?}"
            );
        }
        Ok(reopened) => match reopened.integrity_check() {
            Err(ref err) => {
                assert!(
                    err.is_integrity(),
                    "expected Integrity refusal, got: {err:?}"
                );
                assert!(!err.is_retryable());
            }
            Ok(report) => {
                assert!(
                    !report.is_clean(),
                    "flipped byte at middle of closed file ({}) must be detected, got: {report:?}",
                    active_file.display()
                );
            }
        },
    }
}

#[test]
fn second_writer_refused() {
    let db = scratch_db("second-writer-refused");
    let opts = OpenOptions::default().with_busy_timeout(Duration::from_secs(1));

    let mut h1 = FsqliteLedger::open(&db, SchemaVersion::M0, opts.clone()).expect("open handle 1");
    let mut h2 = FsqliteLedger::open(&db, SchemaVersion::M0, opts).expect("open handle 2");

    let mut txn1 = h1.begin().expect("handle 1 begins first");
    txn1.put(TableId::Run, b"run-1", b"from-handle-1")
        .expect("handle 1 puts row");

    // Second handle must fail with typed Busy within bounded wait (~1 s), never hang/deadlock.
    let started = Instant::now();
    let err = h2
        .begin()
        .err()
        .expect("handle 2 begin must be refused while handle 1 holds transaction");
    let elapsed = started.elapsed();
    assert!(err.is_busy(), "expected Busy refusal, got: {err:?}");
    assert!(err.is_retryable(), "Busy refusal must be retryable");
    assert!(
        elapsed >= Duration::from_millis(800) && elapsed < Duration::from_secs(5),
        "expected bounded wait around 1 s busy_timeout, elapsed = {elapsed:?}"
    );

    // After handle 1 commits, handle 2's begin must succeed and observe handle 1's row.
    let seq1 = txn1.commit().expect("handle 1 commits");
    assert_eq!(seq1, CommitSeq::new(1));

    let mut txn2 = h2.begin().expect("handle 2 begins after handle 1 commit");
    assert_eq!(
        txn2.get(TableId::Run, b"run-1")
            .expect("handle 2 reads handle 1 row"),
        Some(b"from-handle-1".to_vec())
    );
    txn2.put(TableId::Run, b"run-2", b"from-handle-2")
        .expect("handle 2 puts row");
    let seq2 = txn2.commit().expect("handle 2 commits");
    assert_eq!(seq2, CommitSeq::new(2));
}
