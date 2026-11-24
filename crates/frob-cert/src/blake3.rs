//! Portable std-only BLAKE3 implementation for `frob-cert` (BD-026).
//!
//! Implements the exact BLAKE3 specification: 7-round compression, 1,024-byte
//! chunks, binary Merkle tree chaining, and arbitrary-length XOF output in
//! plain hash, keyed hash, and derive-key modes.

/// Standard BLAKE3 output length in bytes (256 bits).
pub const OUT_LEN: usize = 32;
/// BLAKE3 keyed-hash key length in bytes (256 bits).
pub const KEY_LEN: usize = 32;
/// Compression block size in bytes.
pub const BLOCK_LEN: usize = 64;
/// Leaf chunk size in bytes.
pub const CHUNK_LEN: usize = 1024;

const CHUNK_START: u32 = 1 << 0;
const CHUNK_END: u32 = 1 << 1;
const PARENT: u32 = 1 << 2;
const ROOT: u32 = 1 << 3;
const KEYED_HASH: u32 = 1 << 4;
const DERIVE_KEY_CONTEXT: u32 = 1 << 5;
const DERIVE_KEY_MATERIAL: u32 = 1 << 6;

const IV: [u32; 8] = [
    0x6A09_E667,
    0xBB67_AE85,
    0x3C6E_F372,
    0xA54F_F53A,
    0x510E_527F,
    0x9B05_688C,
    0x1F83_D9AB,
    0x5BE0_CD19,
];

const MSG_PERMUTATION: [usize; 16] = [2, 6, 3, 10, 7, 0, 4, 13, 1, 11, 12, 5, 9, 14, 15, 8];

#[inline]
fn g(state: &mut [u32; 16], a: usize, b: usize, c: usize, d: usize, mx: u32, my: u32) {
    state[a] = state[a].wrapping_add(state[b]).wrapping_add(mx);
    state[d] = (state[d] ^ state[a]).rotate_right(16);
    state[c] = state[c].wrapping_add(state[d]);
    state[b] = (state[b] ^ state[c]).rotate_right(12);
    state[a] = state[a].wrapping_add(state[b]).wrapping_add(my);
    state[d] = (state[d] ^ state[a]).rotate_right(8);
    state[c] = state[c].wrapping_add(state[d]);
    state[b] = (state[b] ^ state[c]).rotate_right(7);
}

#[inline]
fn round(state: &mut [u32; 16], m: &[u32; 16]) {
    // Column step.
    g(state, 0, 4, 8, 12, m[0], m[1]);
    g(state, 1, 5, 9, 13, m[2], m[3]);
    g(state, 2, 6, 10, 14, m[4], m[5]);
    g(state, 3, 7, 11, 15, m[6], m[7]);
    // Diagonal step.
    g(state, 0, 5, 10, 15, m[8], m[9]);
    g(state, 1, 6, 11, 12, m[10], m[11]);
    g(state, 2, 7, 8, 13, m[12], m[13]);
    g(state, 3, 4, 9, 14, m[14], m[15]);
}

#[inline]
fn permute(m: &mut [u32; 16]) {
    let mut permuted = [0_u32; 16];
    let mut i = 0;
    while i < 16 {
        permuted[i] = m[MSG_PERMUTATION[i]];
        i += 1;
    }
    *m = permuted;
}

fn compress(
    chaining_value: &[u32; 8],
    block_words: &[u32; 16],
    counter: u64,
    block_len: u32,
    flags: u32,
) -> [u32; 16] {
    let counter_low = counter as u32;
    let counter_high = (counter >> 32) as u32;
    let mut state = [
        chaining_value[0],
        chaining_value[1],
        chaining_value[2],
        chaining_value[3],
        chaining_value[4],
        chaining_value[5],
        chaining_value[6],
        chaining_value[7],
        IV[0],
        IV[1],
        IV[2],
        IV[3],
        counter_low,
        counter_high,
        block_len,
        flags,
    ];
    let mut block = *block_words;

    round(&mut state, &block); // round 1
    permute(&mut block);
    round(&mut state, &block); // round 2
    permute(&mut block);
    round(&mut state, &block); // round 3
    permute(&mut block);
    round(&mut state, &block); // round 4
    permute(&mut block);
    round(&mut state, &block); // round 5
    permute(&mut block);
    round(&mut state, &block); // round 6
    permute(&mut block);
    round(&mut state, &block); // round 7

    let mut i = 0;
    while i < 8 {
        state[i] ^= state[i + 8];
        state[i + 8] ^= chaining_value[i];
        i += 1;
    }
    state
}

#[inline]
fn first_8_words(out: [u32; 16]) -> [u32; 8] {
    [
        out[0], out[1], out[2], out[3], out[4], out[5], out[6], out[7],
    ]
}

fn words_from_le_bytes_32(bytes: &[u8; 32]) -> [u32; 8] {
    let mut words = [0_u32; 8];
    let mut i = 0;
    while i < 8 {
        let base = i * 4;
        words[i] = u32::from_le_bytes([
            bytes[base],
            bytes[base + 1],
            bytes[base + 2],
            bytes[base + 3],
        ]);
        i += 1;
    }
    words
}

fn words_from_le_bytes_64(bytes: &[u8; 64]) -> [u32; 16] {
    let mut words = [0_u32; 16];
    let mut i = 0;
    while i < 16 {
        let base = i * 4;
        words[i] = u32::from_le_bytes([
            bytes[base],
            bytes[base + 1],
            bytes[base + 2],
            bytes[base + 3],
        ]);
        i += 1;
    }
    words
}

#[derive(Clone)]
struct Output {
    input_chaining_value: [u32; 8],
    block_words: [u32; 16],
    counter: u64,
    block_len: u32,
    flags: u32,
}

impl Output {
    fn chaining_value(&self) -> [u32; 8] {
        first_8_words(compress(
            &self.input_chaining_value,
            &self.block_words,
            self.counter,
            self.block_len,
            self.flags,
        ))
    }

    fn root_output_bytes(&self, out_slice: &mut [u8]) {
        let mut output_block_counter = 0_u64;
        for out_block in out_slice.chunks_mut(2 * OUT_LEN) {
            let words = compress(
                &self.input_chaining_value,
                &self.block_words,
                output_block_counter,
                self.block_len,
                self.flags | ROOT,
            );
            let mut offset = 0_usize;
            for word in words {
                if offset >= out_block.len() {
                    break;
                }
                let word_bytes = word.to_le_bytes();
                let take = (out_block.len() - offset).min(4);
                out_block[offset..offset + take].copy_from_slice(&word_bytes[..take]);
                offset += take;
            }
            output_block_counter += 1;
        }
    }
}

#[derive(Clone)]
struct ChunkState {
    chaining_value: [u32; 8],
    chunk_counter: u64,
    block: [u8; BLOCK_LEN],
    block_len: u8,
    blocks_compressed: u8,
    flags: u32,
}

impl ChunkState {
    fn new(key_words: [u32; 8], chunk_counter: u64, flags: u32) -> Self {
        Self {
            chaining_value: key_words,
            chunk_counter,
            block: [0_u8; BLOCK_LEN],
            block_len: 0,
            blocks_compressed: 0,
            flags,
        }
    }

    fn len(&self) -> usize {
        BLOCK_LEN * (self.blocks_compressed as usize) + (self.block_len as usize)
    }

    fn start_flag(&self) -> u32 {
        if self.blocks_compressed == 0 {
            CHUNK_START
        } else {
            0
        }
    }

    fn update(&mut self, mut input: &[u8]) {
        while !input.is_empty() {
            if self.block_len as usize == BLOCK_LEN {
                let block_words = words_from_le_bytes_64(&self.block);
                self.chaining_value = first_8_words(compress(
                    &self.chaining_value,
                    &block_words,
                    self.chunk_counter,
                    BLOCK_LEN as u32,
                    self.flags | self.start_flag(),
                ));
                self.blocks_compressed += 1;
                self.block = [0_u8; BLOCK_LEN];
                self.block_len = 0;
            }
            let want = BLOCK_LEN - (self.block_len as usize);
            let take = want.min(input.len());
            let start = self.block_len as usize;
            self.block[start..start + take].copy_from_slice(&input[..take]);
            self.block_len += take as u8;
            input = &input[take..];
        }
    }

    fn output(&self) -> Output {
        let block_words = words_from_le_bytes_64(&self.block);
        Output {
            input_chaining_value: self.chaining_value,
            block_words,
            counter: self.chunk_counter,
            block_len: u32::from(self.block_len),
            flags: self.flags | self.start_flag() | CHUNK_END,
        }
    }
}

fn parent_output(
    left_child_cv: [u32; 8],
    right_child_cv: [u32; 8],
    key_words: [u32; 8],
    flags: u32,
) -> Output {
    let mut block_words = [0_u32; 16];
    block_words[..8].copy_from_slice(&left_child_cv);
    block_words[8..].copy_from_slice(&right_child_cv);
    Output {
        input_chaining_value: key_words,
        block_words,
        counter: 0,
        block_len: BLOCK_LEN as u32,
        flags: PARENT | flags,
    }
}

fn parent_cv(
    left_child_cv: [u32; 8],
    right_child_cv: [u32; 8],
    key_words: [u32; 8],
    flags: u32,
) -> [u32; 8] {
    parent_output(left_child_cv, right_child_cv, key_words, flags).chaining_value()
}

/// Incremental BLAKE3 hasher supporting plain, keyed, and derive-key modes.
#[derive(Clone)]
pub struct Hasher {
    chunk_state: ChunkState,
    key_words: [u32; 8],
    cv_stack: [[u32; 8]; 54],
    cv_stack_len: u8,
    flags: u32,
}

impl Default for Hasher {
    fn default() -> Self {
        Self::new()
    }
}

impl Hasher {
    fn new_internal(key_words: [u32; 8], flags: u32) -> Self {
        Self {
            chunk_state: ChunkState::new(key_words, 0, flags),
            key_words,
            cv_stack: [[0_u32; 8]; 54],
            cv_stack_len: 0,
            flags,
        }
    }

    /// Construct a hasher in standard (unkeyed) BLAKE3 mode.
    pub fn new() -> Self {
        Self::new_internal(IV, 0)
    }

    /// Construct a hasher in keyed MAC mode with a 32-byte key.
    pub fn new_keyed(key: &[u8; KEY_LEN]) -> Self {
        let key_words = words_from_le_bytes_32(key);
        Self::new_internal(key_words, KEYED_HASH)
    }

    /// Construct a hasher in key-derivation mode for the given domain/context string.
    pub fn new_derive_key(context: &str) -> Self {
        let mut context_hasher = Self::new_internal(IV, DERIVE_KEY_CONTEXT);
        context_hasher.update(context.as_bytes());
        let context_key = context_hasher.finalize();
        let context_key_words = words_from_le_bytes_32(&context_key);
        Self::new_internal(context_key_words, DERIVE_KEY_MATERIAL)
    }

    fn push_stack(&mut self, cv: [u32; 8]) {
        self.cv_stack[self.cv_stack_len as usize] = cv;
        self.cv_stack_len += 1;
    }

    fn pop_stack(&mut self) -> [u32; 8] {
        self.cv_stack_len -= 1;
        self.cv_stack[self.cv_stack_len as usize]
    }

    fn add_chunk_chaining_value(&mut self, mut new_cv: [u32; 8], mut total_chunks: u64) {
        while total_chunks & 1 == 0 {
            let left = self.pop_stack();
            new_cv = parent_cv(left, new_cv, self.key_words, self.flags);
            total_chunks >>= 1;
        }
        self.push_stack(new_cv);
    }

    /// Feed additional input bytes into the hasher.
    pub fn update(&mut self, mut input: &[u8]) {
        while !input.is_empty() {
            if self.chunk_state.len() == CHUNK_LEN {
                let chunk_cv = self.chunk_state.output().chaining_value();
                let total_chunks = self.chunk_state.chunk_counter + 1;
                self.add_chunk_chaining_value(chunk_cv, total_chunks);
                self.chunk_state = ChunkState::new(self.key_words, total_chunks, self.flags);
            }
            let want = CHUNK_LEN - self.chunk_state.len();
            let take = want.min(input.len());
            self.chunk_state.update(&input[..take]);
            input = &input[take..];
        }
    }

    fn root_output(&self) -> Output {
        let mut output = self.chunk_state.output();
        let mut parent_nodes_remaining = self.cv_stack_len as usize;
        while parent_nodes_remaining > 0 {
            parent_nodes_remaining -= 1;
            output = parent_output(
                self.cv_stack[parent_nodes_remaining],
                output.chaining_value(),
                self.key_words,
                self.flags,
            );
        }
        output
    }

    /// Finalize into a 32-byte digest without mutating hasher state.
    pub fn finalize(&self) -> [u8; OUT_LEN] {
        let mut out = [0_u8; OUT_LEN];
        self.finalize_xof(&mut out);
        out
    }

    /// Finalize into an arbitrary-length XOF output slice.
    pub fn finalize_xof(&self, out: &mut [u8]) {
        self.root_output().root_output_bytes(out);
    }

    /// Finalize into a 64-character lowercase hexadecimal string.
    pub fn finalize_hex(&self) -> String {
        to_hex(&self.finalize())
    }
}

/// Encode bytes as lowercase hexadecimal.
pub fn to_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        out.push(HEX[(b >> 4) as usize] as char);
        out.push(HEX[(b & 0x0f) as usize] as char);
    }
    out
}

/// Compute standard 32-byte BLAKE3 digest.
pub fn hash(input: &[u8]) -> [u8; OUT_LEN] {
    let mut h = Hasher::new();
    h.update(input);
    h.finalize()
}

/// Compute standard 64-hex BLAKE3 digest.
pub fn hash_hex(input: &[u8]) -> String {
    to_hex(&hash(input))
}

/// Compute arbitrary-length BLAKE3 XOF output in standard mode.
pub fn hash_xof(input: &[u8], out: &mut [u8]) {
    let mut h = Hasher::new();
    h.update(input);
    h.finalize_xof(out);
}

/// Compute 32-byte keyed BLAKE3 digest.
pub fn keyed_hash(key: &[u8; KEY_LEN], input: &[u8]) -> [u8; OUT_LEN] {
    let mut h = Hasher::new_keyed(key);
    h.update(input);
    h.finalize()
}

/// Compute arbitrary-length keyed BLAKE3 XOF output.
pub fn keyed_hash_xof(key: &[u8; KEY_LEN], input: &[u8], out: &mut [u8]) {
    let mut h = Hasher::new_keyed(key);
    h.update(input);
    h.finalize_xof(out);
}

/// Compute 32-byte derive-key BLAKE3 digest (domain-separated by `context`).
pub fn derive_key(context: &str, key_material: &[u8]) -> [u8; OUT_LEN] {
    let mut h = Hasher::new_derive_key(context);
    h.update(key_material);
    h.finalize()
}

/// Compute arbitrary-length derive-key BLAKE3 XOF output.
pub fn derive_key_xof(context: &str, key_material: &[u8], out: &mut [u8]) {
    let mut h = Hasher::new_derive_key(context);
    h.update(key_material);
    h.finalize_xof(out);
}

/// Compute a FrankenRobotics artifact identity: BLAKE3 derive-key over canonical
/// bytes under a domain string equal to the schema name (BD-026 item 3).
pub fn domain_hash(schema: &str, canonical_bytes: &[u8]) -> [u8; OUT_LEN] {
    derive_key(schema, canonical_bytes)
}

/// Lowercase hex form of `domain_hash`.
pub fn domain_hash_hex(schema: &str, canonical_bytes: &[u8]) -> String {
    to_hex(&domain_hash(schema, canonical_bytes))
}
