#![forbid(unsafe_code)]

//! Canonical referee-state digest (`BD-007`, `BD-024`).
//!
//! Single source file with no dependency except the bedrock hash crate `fs-blake3`
//! (and `std`). Included byte-for-byte in both `frob-fsim` and FrankenSim's in-tree
//! oracle probe (`crates/frob-fsim/intree-probe/`) so both sides execute identical
//! hashing code over identically ordered bytes.
//!
//! ## Wire byte layout (D0 deterministic)
//! 1. ASCII domain tag `frob.referee-state/1` (20 bytes) + `0x00` terminator (1 byte).
//! 2. `links`, `nq`, `nv` as 32-bit little-endian unsigned integers (12 bytes).
//! 3. Each element of `q`, then each element of `v`, as its 8-byte IEEE-754 little-endian
//!    bit pattern (`f64::to_bits().to_le_bytes()`). `-0.0` and `+0.0` hash differently.
//! 4. Trailing payload tag:
//!    - `0x00` for a plain state digest (used for rollout digests);
//!    - `0x01` followed by each element of the acceleration vector `qdd` in the same
//!      8-byte IEEE-754 little-endian bit encoding for a forward-dynamics digest.
//! 5. Plain 32-byte BLAKE3 via `fs_blake3::hash_bytes`, rendered as 64 lower-case hex digits.

/// ASCII domain tag including its trailing NUL byte (`21` bytes total).
pub const DOMAIN_TAG: &[u8] = b"frob.referee-state/1\0";

/// Tag byte for a plain state digest (no acceleration vector follows).
pub const TAG_STATE: u8 = 0x00;

/// Tag byte for a forward-dynamics digest (`nv` acceleration coordinates follow).
pub const TAG_FORWARD_DYNAMICS: u8 = 0x01;

/// Encode the canonical byte pre-image for a referee state (with optional acceleration vector).
#[must_use]
pub fn encode_state_bytes(links: u32, q: &[f64], v: &[f64], accel: Option<&[f64]>) -> Vec<u8> {
    let nq = u32::try_from(q.len()).expect("generalized coordinate count exceeds u32::MAX");
    let nv = u32::try_from(v.len()).expect("generalized velocity count exceeds u32::MAX");
    let accel_len = accel.map_or(0, <[f64]>::len);
    let capacity = DOMAIN_TAG.len() + 12 + 8 * (q.len() + v.len() + accel_len) + 1;
    let mut buf = Vec::with_capacity(capacity);
    buf.extend_from_slice(DOMAIN_TAG);
    buf.extend_from_slice(&links.to_le_bytes());
    buf.extend_from_slice(&nq.to_le_bytes());
    buf.extend_from_slice(&nv.to_le_bytes());
    for &x in q {
        buf.extend_from_slice(&x.to_bits().to_le_bytes());
    }
    for &x in v {
        buf.extend_from_slice(&x.to_bits().to_le_bytes());
    }
    match accel {
        None => buf.push(TAG_STATE),
        Some(qdd) => {
            buf.push(TAG_FORWARD_DYNAMICS);
            for &x in qdd {
                buf.extend_from_slice(&x.to_bits().to_le_bytes());
            }
        }
    }
    buf
}

/// Compute the 32-byte BLAKE3 digest of a plain referee state (`tag = 0x00`).
#[must_use]
pub fn state_digest_bytes(links: u32, q: &[f64], v: &[f64]) -> [u8; 32] {
    let bytes = encode_state_bytes(links, q, v, None);
    *fs_blake3::hash_bytes(&bytes).as_bytes()
}

/// Compute the 32-byte BLAKE3 digest of a referee forward-dynamics evaluation (`tag = 0x01`).
#[must_use]
pub fn forward_dynamics_digest_bytes(
    links: u32,
    q: &[f64],
    v: &[f64],
    accel: &[f64],
) -> [u8; 32] {
    let bytes = encode_state_bytes(links, q, v, Some(accel));
    *fs_blake3::hash_bytes(&bytes).as_bytes()
}

/// Render a 32-byte BLAKE3 digest as 64 lower-case hexadecimal digits.
#[must_use]
pub fn to_hex(digest: &[u8; 32]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(64);
    for &b in digest {
        out.push(HEX[usize::from(b >> 4)] as char);
        out.push(HEX[usize::from(b & 0x0f)] as char);
    }
    out
}

/// Compute the 64-hex BLAKE3 digest of arbitrary raw bytes (used for capture-log verification).
#[must_use]
pub fn blake3_hex(bytes: &[u8]) -> String {
    to_hex(fs_blake3::hash_bytes(bytes).as_bytes())
}

/// Compute the 64-hex BLAKE3 digest of a plain referee state (`tag = 0x00`).
#[must_use]
pub fn state_digest_hex(links: u32, q: &[f64], v: &[f64]) -> String {
    to_hex(&state_digest_bytes(links, q, v))
}

/// Compute the 64-hex BLAKE3 digest of a referee forward-dynamics evaluation (`tag = 0x01`).
#[must_use]
pub fn forward_dynamics_digest_hex(links: u32, q: &[f64], v: &[f64], accel: &[f64]) -> String {
    to_hex(&forward_dynamics_digest_bytes(links, q, v, accel))
}
