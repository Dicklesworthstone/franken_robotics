//! Address-keyed property testing, deterministic shrinking, and regressions (STD-TEST §4).

use std::collections::BTreeMap;
use std::env;
use std::fmt;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

pub const DEFAULT_CASES: usize = 256;
pub const MAX_SHRINK_STEPS: usize = 1_000;

const B3_IV: [u32; 8] = [0x6A09_E667, 0xBB67_AE85, 0x3C6E_F372, 0xA54F_F53A, 0x510E_527F, 0x9B05_688C, 0x1F83_D9AB, 0x5BE0_CD19];
const B3_PERM: [usize; 16] = [2, 6, 3, 10, 7, 0, 4, 13, 1, 11, 12, 5, 9, 14, 15, 8];

#[inline]
fn b3_g(s: &mut [u32; 16], a: usize, b: usize, c: usize, d: usize, mx: u32, my: u32) {
    s[a] = s[a].wrapping_add(s[b]).wrapping_add(mx); s[d] = (s[d] ^ s[a]).rotate_right(16);
    s[c] = s[c].wrapping_add(s[d]);                  s[b] = (s[b] ^ s[c]).rotate_right(12);
    s[a] = s[a].wrapping_add(s[b]).wrapping_add(my); s[d] = (s[d] ^ s[a]).rotate_right(8);
    s[c] = s[c].wrapping_add(s[d]);                  s[b] = (s[b] ^ s[c]).rotate_right(7);
}

fn b3_compress(cv: &[u32; 8], block: &[u32; 16], counter: u64, block_len: u32, flags: u32) -> [u32; 16] {
    let mut s = [cv[0], cv[1], cv[2], cv[3], cv[4], cv[5], cv[6], cv[7], B3_IV[0], B3_IV[1], B3_IV[2], B3_IV[3], counter as u32, (counter >> 32) as u32, block_len, flags];
    let mut m = *block;
    for r in 0..7 {
        b3_g(&mut s, 0, 4, 8, 12, m[0], m[1]);  b3_g(&mut s, 1, 5, 9, 13, m[2], m[3]);
        b3_g(&mut s, 2, 6, 10, 14, m[4], m[5]); b3_g(&mut s, 3, 7, 11, 15, m[6], m[7]);
        b3_g(&mut s, 0, 5, 10, 15, m[8], m[9]); b3_g(&mut s, 1, 6, 11, 12, m[10], m[11]);
        b3_g(&mut s, 2, 7, 8, 13, m[12], m[13]); b3_g(&mut s, 3, 4, 9, 14, m[14], m[15]);
        if r < 6 { m = std::array::from_fn(|i| m[B3_PERM[i]]); }
    }
    for i in 0..8 { s[i] ^= s[i + 8]; s[i + 8] ^= cv[i]; }
    s
}

fn b3_parent(l: [u32; 8], r: [u32; 8], flags: u32) -> [u32; 16] {
    let mut blk = [0u32; 16];
    blk[..8].copy_from_slice(&l);
    blk[8..].copy_from_slice(&r);
    b3_compress(&B3_IV, &blk, 0, 64, 4 | flags)
}

fn words_to_bytes(w: &[u32]) -> [u8; 32] {
    let mut out = [0u8; 32];
    for (i, &v) in w.iter().take(8).enumerate() { out[4 * i..4 * i + 4].copy_from_slice(&v.to_le_bytes()); }
    out
}

/// Exact standard BLAKE3 32-byte digest (pure `std`, D0).
#[must_use]
pub fn blake3_hash(input: &[u8]) -> [u8; 32] {
    let mut cvs: Vec<[u32; 8]> = Vec::new();
    let total_chunks = if input.is_empty() { 1 } else { input.len().div_ceil(1024) };
    for ci in 0..total_chunks {
        let chunk = &input[ci * 1024..((ci + 1) * 1024).min(input.len())];
        let mut cv = B3_IV;
        let blocks = if chunk.is_empty() { 1 } else { chunk.len().div_ceil(64) };
        for b in 0..blocks {
            let sl = &chunk[b * 64..((b + 1) * 64).min(chunk.len())];
            let mut buf = [0u8; 64];
            buf[..sl.len()].copy_from_slice(sl);
            let blk = std::array::from_fn(|i| u32::from_le_bytes([buf[4 * i], buf[4 * i + 1], buf[4 * i + 2], buf[4 * i + 3]]));
            let fl = u32::from(b == 0) | (u32::from(b + 1 == blocks) << 1) | (u32::from(total_chunks == 1 && b + 1 == blocks) << 3);
            let out = b3_compress(&cv, &blk, ci as u64, sl.len() as u32, fl);
            if total_chunks == 1 && b + 1 == blocks { return words_to_bytes(&out[..8]); }
            cv.copy_from_slice(&out[..8]);
        }
        cvs.push(cv);
        let mut t = ci + 1;
        while t % 2 == 0 && ci + 1 < total_chunks {
            let (r, l) = (cvs.pop().expect("r"), cvs.pop().expect("l"));
            let mut p = [0u32; 8]; p.copy_from_slice(&b3_parent(l, r, 0)[..8]);
            cvs.push(p);
            t /= 2;
        }
    }
    while cvs.len() > 1 {
        let (r, l) = (cvs.pop().expect("r"), cvs.pop().expect("l"));
        let root_fl = if cvs.is_empty() { 8 } else { 0 };
        let out = b3_parent(l, r, root_fl);
        if cvs.is_empty() { return words_to_bytes(&out[..8]); }
        let mut p = [0u32; 8]; p.copy_from_slice(&out[..8]);
        cvs.push(p);
    }
    words_to_bytes(&cvs[0])
}

#[must_use]
pub fn base_seed(crate_name: &str, test_name: &str, env_seed: u64) -> u64 {
    let mut buf = Vec::with_capacity(crate_name.len() + 1 + test_name.len());
    buf.extend_from_slice(crate_name.as_bytes());
    buf.push(0x00);
    buf.extend_from_slice(test_name.as_bytes());
    let h = blake3_hash(&buf);
    u64::from_le_bytes([h[0], h[1], h[2], h[3], h[4], h[5], h[6], h[7]]) ^ env_seed
}

#[must_use]
pub const fn mix(seed: u64, case: u64, stream: u64, index: u64) -> u64 {
    let mut z = seed.wrapping_mul(0x9E37_79B9_7F4A_7C15)
        ^ case.wrapping_add(1).wrapping_mul(0xBF58_476D_1CE4_E5B9)
        ^ stream.wrapping_add(1).wrapping_mul(0x94D0_49BB_1331_11EB)
        ^ index.wrapping_add(1).wrapping_mul(0xD6E8_FEB8_6659_FD93);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

#[derive(Debug, Clone)]
pub struct Gen { seed: u64, case: u64, cursors: BTreeMap<u64, u64> }

impl Gen {
    #[must_use] pub fn new(seed: u64, case: u64) -> Self { Self { seed, case, cursors: BTreeMap::new() } }
    #[must_use] pub const fn seed(&self) -> u64 { self.seed }
    #[must_use] pub const fn case(&self) -> u64 { self.case }
    #[must_use] pub const fn draw_at(&self, stream: u64, index: u64) -> u64 { mix(self.seed, self.case, stream, index) }

    pub fn next_u64(&mut self, stream: u64) -> u64 {
        let idx = self.cursors.entry(stream).or_insert(0);
        let v = mix(self.seed, self.case, stream, *idx);
        *idx = idx.wrapping_add(1);
        v
    }

    pub fn u64_range(&mut self, stream: u64, lo: u64, hi: u64) -> u64 {
        assert!(lo <= hi, "invalid u64_range [{lo}, {hi}]");
        let span = hi - lo;
        if span == u64::MAX { self.next_u64(stream) } else { lo + (self.next_u64(stream) % (span + 1)) }
    }

    pub fn i64_range(&mut self, stream: u64, lo: i64, hi: i64) -> i64 {
        assert!(lo <= hi, "invalid i64_range [{lo}, {hi}]");
        let span = hi.abs_diff(lo);
        if span == u64::MAX { self.next_u64(stream) as i64 } else { lo.wrapping_add((self.next_u64(stream) % (span + 1)) as i64) }
    }

    pub fn bool(&mut self, stream: u64) -> bool { (self.next_u64(stream) & 1) == 1 }
    pub fn f64_unit(&mut self, stream: u64) -> f64 { ((self.next_u64(stream) >> 11) as f64) * (1.0 / ((1u64 << 53) as f64)) }
}

pub trait Shrink: Clone + fmt::Debug { fn shrink_candidates(&self) -> Vec<Self>; }

fn shrink_u64(v: u64) -> Vec<u64> {
    if v == 0 { return Vec::new(); }
    let mut out = vec![0];
    let mut d = v / 2;
    while d > 0 {
        let c = v - d;
        if c != 0 && out.last() != Some(&c) { out.push(c); }
        d /= 2;
    }
    if v > 1 && out.last() != Some(&(v - 1)) { out.push(v - 1); }
    out
}

fn shrink_i64(v: i64) -> Vec<i64> {
    if v == 0 { return Vec::new(); }
    let mut out = vec![0];
    if v < 0 && v != i64::MIN { out.push(-v); }
    let mut d = v / 2;
    while d != 0 {
        let c = v - d;
        if c != 0 && !out.contains(&c) { out.push(c); }
        d /= 2;
    }
    let s = if v > 0 { v - 1 } else { v + 1 };
    if s != 0 && !out.contains(&s) { out.push(s); }
    out
}

impl Shrink for u64 { fn shrink_candidates(&self) -> Vec<Self> { shrink_u64(*self) } }
impl Shrink for u32 { fn shrink_candidates(&self) -> Vec<Self> { shrink_u64(u64::from(*self)).into_iter().map(|v| v as u32).collect() } }
impl Shrink for usize { fn shrink_candidates(&self) -> Vec<Self> { shrink_u64(*self as u64).into_iter().map(|v| v as usize).collect() } }
impl Shrink for i64 { fn shrink_candidates(&self) -> Vec<Self> { shrink_i64(*self) } }
impl Shrink for i32 { fn shrink_candidates(&self) -> Vec<Self> { shrink_i64(i64::from(*self)).into_iter().map(|v| v as i32).collect() } }
impl Shrink for bool { fn shrink_candidates(&self) -> Vec<Self> { if *self { vec![false] } else { Vec::new() } } }

impl Shrink for f64 {
    fn shrink_candidates(&self) -> Vec<Self> {
        if !self.is_finite() || *self == 0.0 { return Vec::new(); }
        let mut out = vec![0.0];
        if self.abs() > 1.0 { out.push(1.0); out.push(-1.0); }
        let half = *self * 0.5;
        if half.to_bits() != self.to_bits() && half != 0.0 { out.push(half); }
        let tr = self.trunc();
        if tr.to_bits() != self.to_bits() && !out.iter().any(|x| x.to_bits() == tr.to_bits()) { out.push(tr); }
        out
    }
}

impl<T: Shrink> Shrink for Vec<T> {
    fn shrink_candidates(&self) -> Vec<Self> {
        if self.is_empty() { return Vec::new(); }
        let mut out = Vec::new();
        if self.len() >= 2 { let m = self.len() / 2; out.push(self[..m].to_vec()); out.push(self[m..].to_vec()); }
        for i in 0..self.len() { let mut c = self.clone(); c.remove(i); out.push(c); }
        for (i, item) in self.iter().enumerate() {
            for cand in item.shrink_candidates() { let mut c = self.clone(); c[i] = cand; out.push(c); }
        }
        out
    }
}

impl<A: Shrink, B: Shrink> Shrink for (A, B) {
    fn shrink_candidates(&self) -> Vec<Self> {
        let mut out = Vec::new();
        for a in self.0.shrink_candidates() { out.push((a, self.1.clone())); }
        for b in self.1.shrink_candidates() { out.push((self.0.clone(), b)); }
        out
    }
}

pub fn shrink<T: Shrink>(initial: T, mut pred: impl FnMut(&T) -> bool) -> (T, usize) {
    let (mut cur, mut steps) = (initial, 0usize);
    while steps < MAX_SHRINK_STEPS {
        let mut better = false;
        for cand in cur.shrink_candidates() {
            steps += 1;
            if !pred(&cand) { cur = cand; better = true; break; }
            if steps >= MAX_SHRINK_STEPS { break; }
        }
        if !better { break; }
    }
    (cur, steps)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PropFailure<T> {
    pub crate_name: String, pub test_name: String, pub env_seed: u64, pub seed: u64,
    pub case: u64, pub original: T, pub shrunk: T, pub shrink_steps: usize,
    pub message: String, pub replay_cmd: String,
}

impl<T: fmt::Debug> fmt::Display for PropFailure<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "property `{}` failed (seed={}, case={}): {}\n  original: {:?}\n  shrunk ({} steps): {:?}\n  replay: {}",
            self.test_name, self.seed, self.case, self.message, self.original, self.shrink_steps, self.shrunk, self.replay_cmd)
    }
}

#[derive(Debug, Clone)]
pub struct Runner {
    crate_name: String, test_name: String, cases: usize, env_seed: u64,
    only_case: Option<u64>, regressions_path: Option<PathBuf>,
}

impl Runner {
    #[must_use]
    pub fn new(crate_name: impl Into<String>, test_name: impl Into<String>, cases: usize) -> Self {
        Self {
            crate_name: crate_name.into(),
            test_name: test_name.into(),
            cases: env::var("FROB_PROP_CASES").ok().and_then(|s| s.parse().ok()).unwrap_or(cases),
            env_seed: env::var("FROB_TEST_SEED").ok().and_then(|s| s.parse().ok()).unwrap_or(0),
            only_case: env::var("FROB_PROP_CASE").ok().and_then(|s| s.parse().ok()),
            regressions_path: None,
        }
    }

    #[must_use] pub const fn env_seed(mut self, seed: u64) -> Self { self.env_seed = seed; self }
    #[must_use] pub const fn only_case(mut self, case: u64) -> Self { self.only_case = Some(case); self }
    #[must_use] pub fn regressions_path(mut self, path: impl Into<PathBuf>) -> Self { self.regressions_path = Some(path.into()); self }

    pub fn run_shrink<T: Shrink, G: FnMut(&mut Gen) -> T, P: FnMut(&T) -> Result<(), String>>(
        &self, boundaries: &[T], mut generate: G, mut pred: P,
    ) -> Result<(), PropFailure<T>> {
        if let Some(c) = self.only_case {
            return self.try_case(self.env_seed, c, &mut generate, &mut pred, false);
        }
        if let Some(path) = &self.regressions_path {
            for (s, c) in read_regressions(path) { self.try_case(s, c, &mut generate, &mut pred, false)?; }
        }
        let bseed = base_seed(&self.crate_name, &self.test_name, self.env_seed);
        for (i, b) in boundaries.iter().enumerate() {
            if let Err(msg) = pred(b) {
                let (shrunk, shrink_steps) = shrink(b.clone(), |cand| pred(cand).is_ok());
                return Err(PropFailure {
                    crate_name: self.crate_name.clone(), test_name: self.test_name.clone(), env_seed: self.env_seed,
                    seed: bseed, case: i as u64, original: b.clone(), shrunk, shrink_steps,
                    message: format!("boundary[{i}]: {msg}"), replay_cmd: replay_line(&self.crate_name, &self.test_name, self.env_seed, i as u64),
                });
            }
        }
        for c in 0..self.cases as u64 { self.try_case(self.env_seed, c, &mut generate, &mut pred, true)?; }
        Ok(())
    }

    fn try_case<T: Shrink, G: FnMut(&mut Gen) -> T, P: FnMut(&T) -> Result<(), String>>(
        &self, env_seed: u64, case: u64, generate: &mut G, pred: &mut P, record: bool,
    ) -> Result<(), PropFailure<T>> {
        let seed = base_seed(&self.crate_name, &self.test_name, env_seed);
        let mut g = Gen::new(seed, case);
        let original = generate(&mut g);
        if let Err(message) = pred(&original) {
            let (shrunk, shrink_steps) = shrink(original.clone(), |cand| pred(cand).is_ok());
            if record && let Some(path) = &self.regressions_path { let _ = append_regression(path, env_seed, case); }
            return Err(PropFailure {
                crate_name: self.crate_name.clone(), test_name: self.test_name.clone(), env_seed, seed, case,
                original, shrunk, shrink_steps, message, replay_cmd: replay_line(&self.crate_name, &self.test_name, env_seed, case),
            });
        }
        Ok(())
    }
}

pub fn check<F: FnMut(&mut Gen) -> Result<(), String>>(name: &str, cases: usize, mut f: F) {
    let crate_name = env::var("CARGO_PKG_NAME").unwrap_or_else(|_| "frob-core".to_owned());
    let runner = Runner::new(crate_name, name, cases);
    let derived = base_seed(&runner.crate_name, &runner.test_name, runner.env_seed);
    let range: Vec<u64> = match runner.only_case { Some(c) => vec![c], None => (0..runner.cases as u64).collect() };
    for c in range {
        let mut g = Gen::new(derived, c);
        if let Err(msg) = f(&mut g) {
            panic!("property `{name}` failed (seed={derived}, case={c}): {msg}\n  replay: {}",
                replay_line(&runner.crate_name, name, runner.env_seed, c));
        }
    }
}

#[must_use]
pub fn replay_line(crate_name: &str, test_name: &str, env_seed: u64, case: u64) -> String {
    format!("FROB_TEST_SEED={env_seed} FROB_PROP_CASE={case} cargo test -p {crate_name} {test_name}")
}

#[must_use]
pub fn read_regressions(path: &Path) -> Vec<(u64, u64)> {
    let Ok(text) = fs::read_to_string(path) else { return Vec::new() };
    text.lines().filter_map(|l| {
        let t = l.trim();
        if t.is_empty() || t.starts_with('#') { return None; }
        let mut it = t.split_whitespace();
        Some((it.next()?.parse().ok()?, it.next()?.parse().ok()?))
    }).collect()
}

pub fn append_regression(path: &Path, env_seed: u64, case: u64) -> std::io::Result<()> {
    if read_regressions(path).contains(&(env_seed, case)) { return Ok(()); }
    if let Some(p) = path.parent() { fs::create_dir_all(p)?; }
    let mut f = OpenOptions::new().create(true).append(true).open(path)?;
    writeln!(f, "{env_seed} {case}")
}
