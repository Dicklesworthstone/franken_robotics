#!/usr/bin/env python3
"""Independent pure-Python stdlib BLAKE3 implementation for N14a-26.

Implemented directly from the BLAKE3 specification (2020) with zero external
dependencies and zero shared code with any Rust crate in the workspace.
Supports plain hashing (`hash_bytes`), key derivation (`derive_key`), and
`frob.design/1` artifact identity (`design_id`, printed as `d:<64 hex>`).
"""

from __future__ import annotations

import struct
from typing import List, Sequence

OUT_LEN = 32
KEY_LEN = 32
BLOCK_LEN = 64
CHUNK_LEN = 1024

CHUNK_START = 1 << 0
CHUNK_END = 1 << 1
PARENT = 1 << 2
ROOT = 1 << 3
KEYED_HASH = 1 << 4
DERIVE_KEY_CONTEXT = 1 << 5
DERIVE_KEY_MATERIAL = 1 << 6

IV: List[int] = [
    0x6A09E667,
    0xBB67AE85,
    0x3C6EF372,
    0xA54FF53A,
    0x510E527F,
    0x9B05688C,
    0x1F83D9AB,
    0x5BE0CD19,
]

MSG_PERMUTATION: List[int] = [2, 6, 3, 10, 7, 0, 4, 13, 1, 11, 12, 5, 9, 14, 15, 8]

DESIGN_DOMAIN = "frob.design/1"


def _mask32(x: int) -> int:
    return x & 0xFFFFFFFF


def _rotr32(x: int, n: int) -> int:
    return ((x >> n) | ((x << (32 - n)) & 0xFFFFFFFF)) & 0xFFFFFFFF


def _g(state: List[int], a: int, b: int, c: int, d: int, mx: int, my: int) -> None:
    state[a] = _mask32(state[a] + state[b] + mx)
    state[d] = _rotr32(state[d] ^ state[a], 16)
    state[c] = _mask32(state[c] + state[d])
    state[b] = _rotr32(state[b] ^ state[c], 12)
    state[a] = _mask32(state[a] + state[b] + my)
    state[d] = _rotr32(state[d] ^ state[a], 8)
    state[c] = _mask32(state[c] + state[d])
    state[b] = _rotr32(state[b] ^ state[c], 7)


def _round_fn(state: List[int], m: Sequence[int]) -> None:
    # Columns
    _g(state, 0, 4, 8, 12, m[0], m[1])
    _g(state, 1, 5, 9, 13, m[2], m[3])
    _g(state, 2, 6, 10, 14, m[4], m[5])
    _g(state, 3, 7, 11, 15, m[6], m[7])
    # Diagonals
    _g(state, 0, 5, 10, 15, m[8], m[9])
    _g(state, 1, 6, 11, 12, m[10], m[11])
    _g(state, 2, 7, 8, 13, m[12], m[13])
    _g(state, 3, 4, 9, 14, m[14], m[15])


def _permute(m: Sequence[int]) -> List[int]:
    return [m[i] for i in MSG_PERMUTATION]


def _compress(
    cv: Sequence[int],
    block_words: Sequence[int],
    counter: int,
    block_len: int,
    flags: int,
) -> List[int]:
    state: List[int] = [
        cv[0],
        cv[1],
        cv[2],
        cv[3],
        cv[4],
        cv[5],
        cv[6],
        cv[7],
        IV[0],
        IV[1],
        IV[2],
        IV[3],
        _mask32(counter),
        _mask32(counter >> 32),
        _mask32(block_len),
        _mask32(flags),
    ]
    m = list(block_words)
    for r in range(7):
        _round_fn(state, m)
        if r < 6:
            m = _permute(m)
    for i in range(8):
        state[i] ^= state[i + 8]
        state[i + 8] ^= cv[i]
    return state


def _words_from_le_bytes(data: bytes) -> List[int]:
    count = len(data) // 4
    return list(struct.unpack(f"<{count}I", data))


class _Output:
    __slots__ = ("input_cv", "block_words", "counter", "block_len", "flags")

    def __init__(
        self,
        input_cv: Sequence[int],
        block_words: Sequence[int],
        counter: int,
        block_len: int,
        flags: int,
    ) -> None:
        self.input_cv = list(input_cv)
        self.block_words = list(block_words)
        self.counter = counter
        self.block_len = block_len
        self.flags = flags

    def chaining_value(self) -> List[int]:
        return _compress(
            self.input_cv,
            self.block_words,
            self.counter,
            self.block_len,
            self.flags,
        )[:8]

    def root_output_bytes(self, out_len: int = OUT_LEN) -> bytes:
        out = bytearray()
        block_counter = 0
        while len(out) < out_len:
            words = _compress(
                self.input_cv,
                self.block_words,
                block_counter,
                self.block_len,
                self.flags | ROOT,
            )
            out.extend(struct.pack("<16I", *words))
            block_counter += 1
        return bytes(out[:out_len])


class _ChunkState:
    __slots__ = ("cv", "chunk_counter", "buf", "blocks_compressed", "flags")

    def __init__(self, key_words: Sequence[int], chunk_counter: int, flags: int) -> None:
        self.cv = list(key_words)
        self.chunk_counter = chunk_counter
        self.buf = bytearray()
        self.blocks_compressed = 0
        self.flags = flags

    def length(self) -> int:
        return BLOCK_LEN * self.blocks_compressed + len(self.buf)

    def _start_flag(self) -> int:
        return CHUNK_START if self.blocks_compressed == 0 else 0

    def update(self, data: memoryview) -> None:
        pos = 0
        n = len(data)
        while pos < n:
            if len(self.buf) == BLOCK_LEN:
                block_words = _words_from_le_bytes(bytes(self.buf))
                self.cv = _compress(
                    self.cv,
                    block_words,
                    self.chunk_counter,
                    BLOCK_LEN,
                    self.flags | self._start_flag(),
                )[:8]
                self.blocks_compressed += 1
                self.buf.clear()
            take = min(BLOCK_LEN - len(self.buf), n - pos)
            self.buf.extend(data[pos : pos + take])
            pos += take

    def output(self) -> _Output:
        padded = bytes(self.buf) + b"\x00" * (BLOCK_LEN - len(self.buf))
        block_words = _words_from_le_bytes(padded)
        return _Output(
            self.cv,
            block_words,
            self.chunk_counter,
            len(self.buf),
            self.flags | self._start_flag() | CHUNK_END,
        )


def _parent_output(
    left_cv: Sequence[int],
    right_cv: Sequence[int],
    key_words: Sequence[int],
    flags: int,
) -> _Output:
    block_words = list(left_cv) + list(right_cv)
    return _Output(key_words, block_words, 0, BLOCK_LEN, PARENT | flags)


def _parent_cv(
    left_cv: Sequence[int],
    right_cv: Sequence[int],
    key_words: Sequence[int],
    flags: int,
) -> List[int]:
    return _parent_output(left_cv, right_cv, key_words, flags).chaining_value()


class Blake3Hasher:
    """Streaming BLAKE3 hasher implementing plain, keyed, and derive-key modes."""

    def __init__(self, key_words: Sequence[int] | None = None, flags: int = 0) -> None:
        kw = list(IV if key_words is None else key_words)
        self._key_words = kw
        self._flags = flags
        self._chunk_state = _ChunkState(kw, 0, flags)
        self._cv_stack: List[List[int]] = []

    def _add_chunk_cv(self, new_cv: List[int], total_chunks: int) -> None:
        while (total_chunks & 1) == 0:
            new_cv = _parent_cv(
                self._cv_stack.pop(), new_cv, self._key_words, self._flags
            )
            total_chunks >>= 1
        self._cv_stack.append(new_cv)

    def update(self, data: bytes | bytearray | memoryview) -> "Blake3Hasher":
        view = memoryview(data)
        pos = 0
        n = len(view)
        while pos < n:
            if self._chunk_state.length() == CHUNK_LEN:
                chunk_cv = self._chunk_state.output().chaining_value()
                total_chunks = self._chunk_state.chunk_counter + 1
                self._add_chunk_cv(chunk_cv, total_chunks)
                self._chunk_state = _ChunkState(
                    self._key_words, total_chunks, self._flags
                )
            take = min(CHUNK_LEN - self._chunk_state.length(), n - pos)
            self._chunk_state.update(view[pos : pos + take])
            pos += take
        return self

    def finalize(self, out_len: int = OUT_LEN) -> bytes:
        output = self._chunk_state.output()
        for parent_idx in range(len(self._cv_stack) - 1, -1, -1):
            output = _parent_output(
                self._cv_stack[parent_idx],
                output.chaining_value(),
                self._key_words,
                self._flags,
            )
        return output.root_output_bytes(out_len)


def hash_bytes(data: bytes, out_len: int = OUT_LEN) -> bytes:
    """Plain BLAKE3 hash of raw bytes."""
    return Blake3Hasher().update(data).finalize(out_len)


def hash_hex(data: bytes) -> str:
    """Lowercase 64-hex plain BLAKE3 digest."""
    return hash_bytes(data, OUT_LEN).hex()


def derive_key(context: str, key_material: bytes, out_len: int = OUT_LEN) -> bytes:
    """Standard BLAKE3 derive-key mode (domain separation via context string)."""
    ctx_key = (
        Blake3Hasher(IV, DERIVE_KEY_CONTEXT)
        .update(context.encode("utf-8"))
        .finalize(KEY_LEN)
    )
    ctx_words = _words_from_le_bytes(ctx_key)
    return Blake3Hasher(ctx_words, DERIVE_KEY_MATERIAL).update(key_material).finalize(out_len)


def design_id(canonical_bytes: bytes) -> str:
    """Compute the `DesignId` (`d:<64 hex>`) for canonical `frob.design/1` bytes."""
    return "d:" + derive_key(DESIGN_DOMAIN, canonical_bytes, OUT_LEN).hex()


# Published official BLAKE3 test vectors (input bytes = [i % 251 for i in range(n)]).
OFFICIAL_HASH_VECTORS = [
    (0, "af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262"),
    (1, "2d3adedff11b61f14c886e35afa036736dcd87a74d27b5c1510225d0f592e213"),
    (2, "7b7015bb92cf0b318037702a6cdd81dee41224f734684c2c122cd6359cb1ee63"),
    (3, "e1be4d7a8ab5560aa4199eea339849ba8e293d55ca0a81006726d184519e647f"),
    (63, "e9bc37a594daad83be9470df7f7b3798297c3d834ce80ba85d6e207627b7db7b"),
    (64, "4eed7141ea4a5cd4b788606bd23f46e212af9cacebacdc7d1f4c6dc7f2511b98"),
    (65, "de1e5fa0be70df6d2be8fffd0e99ceaa8eb6e8c93a63f2d8d1c30ecb6b263dee"),
    (1023, "10108970eeda3eb932baac1428c7a2163b0e924c9a9e25b35bba72b28f70bd11"),
    (1024, "42214739f095a406f3fc83deb889744ac00df831c10daa55189b5d121c855af7"),
    (1025, "d00278ae47eb27b34faecf67b4fe263f82d5412916c1ffd97c8cb7fb814b8444"),
    (2048, "e776b6028c7cd22a4d0ba182a8bf62205d2ef576467e838ed6f2529b85fba24a"),
    (2049, "5f4d72f40d7a5f82b15ca2b2e44b1de3c2ef86c426c95c1af0b6879522563030"),
    (4097, "9b4052b38f1c5fc8b1f9ff7ac7b27cd242487b3d890d15c96a1c25b8aa0fb995"),
]

OFFICIAL_DERIVE_CONTEXT = "BLAKE3 2019-12-27 16:29:52 test vectors context"
OFFICIAL_DERIVE_VECTORS = [
    (0, "2cc39783c223154fea8dfb7c1b1660f2ac2dcbd1c1de8277b0b0dd39b7e50d7d"),
    (64, "a5c4a7053fa86b64746d4bb688d06ad1f02a18fce9afd3e818fefaa7126bf73e"),
    (1024, "7356cd7720d5b66b6d0697eb3177d9f8d73a4a5c5e968896eb6a689684302706"),
    (1025, "effaa245f065fbf82ac186839a249707c3bddf6d3fdda22d1b95a3c970379bcb"),
    (4097, "aca51029626b55fda7117b42a7c211f8c6e9ba4fe5b7a8ca922f34299500ead8"),
]


def run_blake3_vectors() -> None:
    """Verify hash_bytes and derive_key against official published BLAKE3 vectors."""
    assert hash_hex(b"") == "af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262"
    assert hash_hex(b"abc") == "6437b3ac38465133ffb63b75273a8db548c558465d79db03fd359c6cd5bd9d85"
    assert (
        derive_key("domain", b"payload").hex()
        == "4d50c6393be1879a9767b80ee406c4008d60abd2e76b4241dc97b2b41e2f6cd2"
    )
    for length, expected in OFFICIAL_HASH_VECTORS:
        data = bytes(i % 251 for i in range(length))
        got = hash_hex(data)
        if got != expected:
            raise AssertionError(f"BLAKE3 hash vector len={length}: got {got}, expected {expected}")
    for length, expected in OFFICIAL_DERIVE_VECTORS:
        data = bytes(i % 251 for i in range(length))
        got = derive_key(OFFICIAL_DERIVE_CONTEXT, data).hex()
        if got != expected:
            raise AssertionError(
                f"BLAKE3 derive_key vector len={length}: got {got}, expected {expected}"
            )


if __name__ == "__main__":
    run_blake3_vectors()
    print(
        f"blake3_ref: PASS ({len(OFFICIAL_HASH_VECTORS)} hash vectors + "
        f"{len(OFFICIAL_DERIVE_VECTORS)} derive_key vectors + domain vector)"
    )
