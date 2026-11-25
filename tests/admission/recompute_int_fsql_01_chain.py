#!/usr/bin/env python3
"""Independent verifier for the BD-009 (INT-FSQL-01) uninterrupted reference chain digest.

Recomputes the BLAKE3 row payloads and chained digest c_0..c_N directly from the
row function in pure Python (standard BLAKE3 specification) and cross-checks
selected digests against the `b3sum` CLI when present on PATH.

Zero third-party dependencies. Never deletes files.
"""

from __future__ import annotations

import argparse
import shutil
import struct
import subprocess
import sys

# Standard BLAKE3 constants (RFC / BLAKE3 specification).
_IV = (
    0x6A09E667,
    0xBB67AE85,
    0x3C6EF372,
    0xA54FF53A,
    0x510E527F,
    0x9B05688C,
    0x1F83D9AB,
    0x5BE0CD19,
)
_MSG_PERM = (2, 6, 3, 10, 7, 0, 4, 13, 1, 11, 12, 5, 9, 14, 15, 8)
_CHUNK_START = 1 << 0
_CHUNK_END = 1 << 1
_PARENT = 1 << 2
_ROOT = 1 << 3
_MASK32 = 0xFFFFFFFF

ROW_DOMAIN = b"frob.ledger.probe.row/1"
ROW_PREFIX = b"frob.row/1:"
KILL_DOMAIN = b"frob.ledger.probe.kill/1"


def _rotr32(x: int, n: int) -> int:
    return ((x >> n) | ((x << (32 - n)) & _MASK32)) & _MASK32


def _g(state: list[int], a: int, b: int, c: int, d: int, mx: int, my: int) -> None:
    state[a] = (state[a] + state[b] + mx) & _MASK32
    state[d] = _rotr32(state[d] ^ state[a], 16)
    state[c] = (state[c] + state[d]) & _MASK32
    state[b] = _rotr32(state[b] ^ state[c], 12)
    state[a] = (state[a] + state[b] + my) & _MASK32
    state[d] = _rotr32(state[d] ^ state[a], 8)
    state[c] = (state[c] + state[d]) & _MASK32
    state[b] = _rotr32(state[b] ^ state[c], 7)


def _round(state: list[int], m: list[int]) -> None:
    _g(state, 0, 4, 8, 12, m[0], m[1])
    _g(state, 1, 5, 9, 13, m[2], m[3])
    _g(state, 2, 6, 10, 14, m[4], m[5])
    _g(state, 3, 7, 11, 15, m[6], m[7])
    _g(state, 0, 5, 10, 15, m[8], m[9])
    _g(state, 1, 6, 11, 12, m[10], m[11])
    _g(state, 2, 7, 8, 13, m[12], m[13])
    _g(state, 3, 4, 9, 14, m[14], m[15])


def _compress(
    cv: tuple[int, ...], block_words: list[int], counter: int, block_len: int, flags: int
) -> list[int]:
    state = [
        cv[0], cv[1], cv[2], cv[3], cv[4], cv[5], cv[6], cv[7],
        _IV[0], _IV[1], _IV[2], _IV[3],
        counter & _MASK32,
        (counter >> 32) & _MASK32,
        block_len,
        flags,
    ]
    m = list(block_words)
    for r in range(7):
        _round(state, m)
        if r < 6:
            m = [m[i] for i in _MSG_PERM]
    for i in range(8):
        state[i] ^= state[i + 8]
        state[i + 8] ^= cv[i]
    return state


def _words_from_bytes(block: bytes) -> list[int]:
    padded = block + bytes(64 - len(block))
    return list(struct.unpack("<16I", padded))


def blake3_hash(data: bytes) -> bytes:
    """Standard 32-byte BLAKE3 hash."""
    if len(data) <= 1024:
        cv = _IV
        if not data:
            out = _compress(cv, [0] * 16, 0, 0, _CHUNK_START | _CHUNK_END | _ROOT)
            return struct.pack("<8I", *out[:8])
        n_blocks = (len(data) + 63) // 64
        for idx in range(n_blocks):
            block = data[idx * 64 : (idx + 1) * 64]
            flags = 0
            if idx == 0:
                flags |= _CHUNK_START
            if idx == n_blocks - 1:
                flags |= _CHUNK_END | _ROOT
            out = _compress(cv, _words_from_bytes(block), 0, len(block), flags)
            cv = tuple(out[:8])
        return struct.pack("<8I", *cv)

    # General multi-chunk BLAKE3 tree.
    cv_stack: list[tuple[int, ...]] = []
    n_chunks = (len(data) + 1023) // 1024
    for chunk_idx in range(n_chunks):
        chunk = data[chunk_idx * 1024 : (chunk_idx + 1) * 1024]
        cv = _IV
        n_blocks = (len(chunk) + 63) // 64
        for b_idx in range(n_blocks):
            block = chunk[b_idx * 64 : (b_idx + 1) * 64]
            flags = 0
            if b_idx == 0:
                flags |= _CHUNK_START
            if b_idx == n_blocks - 1:
                flags |= _CHUNK_END
            out = _compress(cv, _words_from_bytes(block), chunk_idx, len(block), flags)
            cv = tuple(out[:8])
        total_chunks = chunk_idx + 1
        while total_chunks & 1 == 0 and chunk_idx + 1 < n_chunks:
            left = cv_stack.pop()
            out = _compress(_IV, list(left) + list(cv), 0, 64, _PARENT)
            cv = tuple(out[:8])
            total_chunks >>= 1
        cv_stack.append(cv)

    cv = cv_stack.pop()
    while len(cv_stack) > 1:
        left = cv_stack.pop()
        out = _compress(_IV, list(left) + list(cv), 0, 64, _PARENT)
        cv = tuple(out[:8])
    left = cv_stack.pop()
    out = _compress(_IV, list(left) + list(cv), 0, 64, _PARENT | _ROOT)
    return struct.pack("<8I", *out[:8])


def row_value(seed: int, txn_idx: int, row_idx: int) -> bytes:
    """Pure 64-byte row payload matching frob_fsim::ledger::probe::row_value."""
    tag = blake3_hash(ROW_DOMAIN + struct.pack("<QQI", seed, txn_idx, row_idx))
    return ROW_PREFIX + struct.pack("<QQIB", seed, txn_idx, row_idx, 0) + tag


def chain_step(prev_chain: bytes, rows: list[bytes]) -> bytes:
    """c_i = BLAKE3(c_{i-1} || row_0 || ... || row_{k-1})."""
    return blake3_hash(prev_chain + b"".join(rows))


def compute_reference_chain(seed: int, n_txns: int, k_rows: int) -> bytes:
    chain = bytes(32)
    for txn_idx in range(1, n_txns + 1):
        rows = [row_value(seed, txn_idx, r) for r in range(k_rows)]
        chain = chain_step(chain, rows)
    return chain


def kill_point(seed: int, n_txns: int, k_rows: int) -> tuple[int, str, int]:
    h = blake3_hash(KILL_DOMAIN + struct.pack("<Q", seed))
    txn_idx = 1 + (struct.unpack("<Q", h[0:8])[0] % n_txns)
    phase_code = h[8] % 3
    put_count = 1 + (h[9] % k_rows)
    if phase_code == 0:
        return txn_idx, "before-begin", 0
    if phase_code == 1:
        return txn_idx, "inside-after-put", put_count
    return txn_idx, "after-commit-before-ack", k_rows


def cross_check_b3sum(seed: int, n_txns: int, k_rows: int) -> None:
    b3_bin = shutil.which("b3sum")
    if b3_bin is None:
        return

    def cli_b3(payload: bytes) -> bytes:
        return subprocess.check_output([b3_bin, "--raw", "--no-names"], input=payload)

    # Cross-check row_value(seed, 1, 0), chain step 1, and kill_point hash.
    r0_in = ROW_DOMAIN + struct.pack("<QQI", seed, 1, 0)
    assert blake3_hash(r0_in) == cli_b3(r0_in), "row tag diverged from b3sum"

    rows_1 = [row_value(seed, 1, r) for r in range(k_rows)]
    c1_in = bytes(32) + b"".join(rows_1)
    assert blake3_hash(c1_in) == cli_b3(c1_in), "chain step 1 diverged from b3sum"

    kp_in = KILL_DOMAIN + struct.pack("<Q", seed)
    assert blake3_hash(kp_in) == cli_b3(kp_in), "kill point hash diverged from b3sum"


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--seed", type=int, default=0)
    parser.add_argument("--txns", type=int, default=200)
    parser.add_argument("--rows", type=int, default=8)
    parser.add_argument("--expect", type=str, default=None)
    args = parser.parse_args()

    cross_check_b3sum(args.seed, args.txns, args.rows)
    digest = compute_reference_chain(args.seed, args.txns, args.rows).hex()
    kp = kill_point(args.seed, args.txns, args.rows)
    print(f"seed={args.seed} N={args.txns} k={args.rows} digest={digest} kill_point={kp}")
    if args.expect is not None and digest != args.expect:
        print(f"MISMATCH: expected {args.expect}, computed {digest}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
