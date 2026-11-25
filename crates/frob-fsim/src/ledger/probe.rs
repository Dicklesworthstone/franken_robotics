#![forbid(unsafe_code)]

//! Deterministic row, chain-digest, and kill-point vocabulary shared by the
//! `BD-009` (`INT-FSQL-01`) forced-kill harness and `ledger_kill_probe`.
//!
//! All functions are pure `D0` arithmetic over `frob_core::id::hash_blob`
//! (standard BLAKE3) so every seed reproduces identically across platforms,
//! debug/release builds, and independent Python (`b3sum`) verification.

use std::fmt;

use frob_core::id::{Blake3, hash_blob};

use super::TableId;

/// Default transaction count per forced-kill trial (`BD-009`).
pub const DEFAULT_TXN_COUNT: u64 = 200;

/// Default rows per transaction (`BD-009`).
pub const DEFAULT_ROWS_PER_TXN: u32 = 8;

/// Fixed byte length of each deterministic synthetic row payload.
pub const ROW_VALUE_LEN: usize = 64;

/// Golden BLAKE3 chain digest for `seed = 0, N = 200, k = 8`, captured and
/// independently verified by `tests/admission/recompute_int_fsql_01_chain.py`
/// against `b3sum 1.8.7`.
pub const GOLDEN_SEED_0_N200_K8: &str =
    "622fea544d7e709ed724e853d5a25c356584618a00b9e0d02bcd3395d5994b53";

/// Golden BLAKE3 chain digest for `seed = 17, N = 200, k = 8`, verified by the
/// same independent script.
pub const GOLDEN_SEED_17_N200_K8: &str =
    "b380835b152e3bc5f7cb2428f929e395cff82c422cb7fea2dc8614980d6cf1d6";

const ROW_DOMAIN: &[u8] = b"frob.ledger.probe.row/1";
const ROW_PREFIX: &[u8] = b"frob.row/1:";
const KILL_DOMAIN: &[u8] = b"frob.ledger.probe.kill/1";

/// Target M0 table for row `row_idx` inside a probe transaction.
#[must_use]
pub const fn table_for_row(row_idx: u32) -> TableId {
    if row_idx == 0 {
        TableId::Run
    } else {
        TableId::Episode
    }
}

/// Deterministic key bytes for `(seed, txn_idx, row_idx)`.
#[must_use]
pub fn row_key(seed: u64, txn_idx: u64, row_idx: u32) -> Vec<u8> {
    format!("s{seed:016x}/t{txn_idx:06}/r{row_idx:02}").into_bytes()
}

/// Deterministic key bytes for transaction `txn_idx`'s chain digest row in `TableId::Meta`.
#[must_use]
pub fn chain_key(seed: u64, txn_idx: u64) -> Vec<u8> {
    format!("chain/s{seed:016x}/t{txn_idx:06}").into_bytes()
}

/// Key bytes for the latest committed chain head in `TableId::Meta`.
pub const CHAIN_HEAD_KEY: &[u8] = b"chain/head";

/// Pure 64-byte row payload derived from `(seed, txn_idx, row_idx)`.
#[must_use]
pub fn row_value(seed: u64, txn_idx: u64, row_idx: u32) -> [u8; ROW_VALUE_LEN] {
    let mut preimage = Vec::with_capacity(ROW_DOMAIN.len() + 20);
    preimage.extend_from_slice(ROW_DOMAIN);
    preimage.extend_from_slice(&seed.to_le_bytes());
    preimage.extend_from_slice(&txn_idx.to_le_bytes());
    preimage.extend_from_slice(&row_idx.to_le_bytes());
    let tag = hash_blob(&preimage);

    let mut out = [0u8; ROW_VALUE_LEN];
    out[0..11].copy_from_slice(ROW_PREFIX);
    out[11..19].copy_from_slice(&seed.to_le_bytes());
    out[19..27].copy_from_slice(&txn_idx.to_le_bytes());
    out[27..31].copy_from_slice(&row_idx.to_le_bytes());
    out[31] = 0;
    out[32..64].copy_from_slice(tag.as_bytes());
    out
}

/// Initial zero chain digest `c_0 = 0^32`.
#[must_use]
pub const fn zero_chain() -> Blake3 {
    Blake3::from_bytes([0u8; 32])
}

/// Advance the chain digest: `c_i = BLAKE3(c_{i-1} || row_0 || ... || row_{k-1})`.
#[must_use]
pub fn chain_step(prev: Blake3, rows: &[&[u8]]) -> Blake3 {
    let total_len = 32 + rows.iter().map(|r| r.len()).sum::<usize>();
    let mut buf = Vec::with_capacity(total_len);
    buf.extend_from_slice(prev.as_bytes());
    for row in rows {
        buf.extend_from_slice(row);
    }
    hash_blob(&buf)
}

/// Pure uninterrupted reference chain value for `(seed, n_txns, k_rows)`.
#[must_use]
pub fn reference_chain_digest(seed: u64, n_txns: u64, k_rows: u32) -> Blake3 {
    let mut chain = zero_chain();
    for txn_idx in 1..=n_txns {
        let owned: Vec<[u8; ROW_VALUE_LEN]> =
            (0..k_rows).map(|r| row_value(seed, txn_idx, r)).collect();
        let slices: Vec<&[u8]> = owned.iter().map(|r| r.as_slice()).collect();
        chain = chain_step(chain, &slices);
    }
    chain
}

/// Encode `(txn_idx, chain_digest)` into the 40-byte `CHAIN_HEAD_KEY` payload.
#[must_use]
pub fn encode_chain_head(txn_idx: u64, digest: Blake3) -> [u8; 40] {
    let mut out = [0u8; 40];
    out[0..8].copy_from_slice(&txn_idx.to_le_bytes());
    out[8..40].copy_from_slice(digest.as_bytes());
    out
}

/// Decode the 40-byte `CHAIN_HEAD_KEY` payload.
#[must_use]
pub fn decode_chain_head(bytes: &[u8]) -> Option<(u64, Blake3)> {
    if bytes.len() != 40 {
        return None;
    }
    let mut idx_bytes = [0u8; 8];
    idx_bytes.copy_from_slice(&bytes[0..8]);
    let mut hash_bytes = [0u8; 32];
    hash_bytes.copy_from_slice(&bytes[8..40]);
    Some((
        u64::from_le_bytes(idx_bytes),
        Blake3::from_bytes(hash_bytes),
    ))
}

/// Phase within transaction `j` where the forced-kill child pauses for SIGKILL.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KillPhase {
    /// Pause before calling `begin()` for transaction `j` (expected `m == a`).
    BeforeBegin,
    /// Pause inside transaction `j` after `put_count` (`1..=k`) `put` calls, before `commit()` (expected `m == a`).
    InsideAfterPut { put_count: u32 },
    /// Pause after `commit()` of transaction `j` returned `Ok`, before writing `C j` (expected `m == a + 1`).
    AfterCommitBeforeAck,
}

impl KillPhase {
    /// Canonical protocol token written on the `AT_KILL` announcement line.
    #[must_use]
    pub fn label(self) -> String {
        match self {
            Self::BeforeBegin => "before-begin".to_owned(),
            Self::InsideAfterPut { put_count } => format!("inside-after-put:{put_count}"),
            Self::AfterCommitBeforeAck => "after-commit-before-ack".to_owned(),
        }
    }
}

impl fmt::Display for KillPhase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.label())
    }
}

/// Deterministic kill schedule derived identically by parent and child from `seed`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KillPoint {
    /// 1-based transaction index in `1..=n_txns`.
    pub txn_idx: u64,
    /// Phase within `txn_idx` where `SIGKILL` lands.
    pub phase: KillPhase,
}

/// Derive a deterministic [`KillPoint`] from `(seed, n_txns, k_rows)`.
#[must_use]
pub fn kill_point_for_seed(seed: u64, n_txns: u64, k_rows: u32) -> KillPoint {
    let n = n_txns.max(1);
    let k = k_rows.max(1);
    let mut preimage = Vec::with_capacity(KILL_DOMAIN.len() + 8);
    preimage.extend_from_slice(KILL_DOMAIN);
    preimage.extend_from_slice(&seed.to_le_bytes());
    let h = hash_blob(&preimage);
    let bytes = h.as_bytes();

    let mut idx_raw = [0u8; 8];
    idx_raw.copy_from_slice(&bytes[0..8]);
    let txn_idx = 1 + (u64::from_le_bytes(idx_raw) % n);

    let phase = match bytes[8] % 3 {
        0 => KillPhase::BeforeBegin,
        1 => {
            let put_count = 1 + (u32::from(bytes[9]) % k);
            KillPhase::InsideAfterPut { put_count }
        }
        _ => KillPhase::AfterCommitBeforeAck,
    };

    KillPoint { txn_idx, phase }
}
