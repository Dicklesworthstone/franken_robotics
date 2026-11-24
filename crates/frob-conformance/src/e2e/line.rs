//! Exact decimal/rational math, RFC 3339 timestamps, path/token redaction,
//! CLI flag parsing, `emit`, `run`, `get`, `cmp`, `transcript`, and `schema`.

use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs::{self, OpenOptions};
use std::io::{self, Read, Write};
use std::path::{Component, Path, PathBuf};
use std::process::Command;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use crate::e2e::json::{self, JsonValue, obj};
use crate::e2e::sha256::{sha256_hex, sha256_hex_prefix12, sha256_path};

pub const LOG_SCHEMA_VERSION: &str = "frob.e2elog/1";
const MAX_DECIMAL_EXP: i64 = 50_000;
const MAX_SAFE_INT: i128 = 9_007_199_254_740_992;

pub const VALID_EVENTS: &[&str] = &["start", "cmd", "assert", "metric", "refusal", "end", "summary"];
pub const VALID_ASSERT_KINDS: &[&str] = &[
    "eq", "num", "close", "rational", "digest", "json", "file",
    "exit", "refusal", "negative-control", "determinism", "check",
];
pub const VALID_TIERS: &[&str] = &[
    "g0_laws", "g1_known_answers", "g2_parity", "g3_metamorphic", "g4_chaos", "g5_determinism",
];

// ---------------------------------------------------------------------------
// Shared CLI flag parser (reused by emit, run, lint, list, report)
// ---------------------------------------------------------------------------

#[derive(Default)]
pub struct ParsedFlags {
    pub singles: BTreeMap<String, String>,
    pub multis: BTreeMap<String, Vec<String>>,
    pub bools: BTreeSet<String>,
    pub positionals: Vec<String>,
}

impl ParsedFlags {
    pub fn parse(args: &[String], valued: &[&str], multi: &[&str], bools: &[&str]) -> Result<Self, String> {
        let mut out = Self::default();
        let mut i = 0;
        while i < args.len() {
            let arg = args[i].as_str();
            if bools.contains(&arg) {
                out.bools.insert(arg.to_string());
            } else if valued.contains(&arg) || multi.contains(&arg) {
                i += 1;
                let v = args.get(i).ok_or_else(|| format!("missing value for {arg}"))?.clone();
                if multi.contains(&arg) {
                    out.multis.entry(arg.to_string()).or_default().push(v);
                } else {
                    out.singles.insert(arg.to_string(), v);
                }
            } else if arg.starts_with('-') && arg != "-" {
                return Err(format!("unknown flag: {arg}"));
            } else {
                out.positionals.push(arg.to_string());
            }
            i += 1;
        }
        Ok(out)
    }

    pub fn get(&self, k: &str) -> Option<&str> { self.singles.get(k).map(String::as_str) }
    pub fn get_or<'a>(&'a self, k: &str, def: &'a str) -> &'a str { self.get(k).unwrap_or(def) }
    pub fn has(&self, k: &str) -> bool { self.bools.contains(k) }
    pub fn list(&self, k: &str) -> &[String] { self.multis.get(k).map(Vec::as_slice).unwrap_or(&[]) }
}

// ---------------------------------------------------------------------------
// Arbitrary-precision non-negative integer & exact Rational (no floats)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BigUint { limbs: Vec<u32> }

impl BigUint {
    pub fn zero() -> Self { Self { limbs: Vec::new() } }
    pub fn one() -> Self { Self { limbs: vec![1] } }
    pub fn is_zero(&self) -> bool { self.limbs.is_empty() }
    fn trim(&mut self) { while self.limbs.last() == Some(&0) { self.limbs.pop(); } }
    pub fn from_u64(v: u64) -> Self {
        let mut s = Self::zero();
        if v != 0 { s.limbs.push(v as u32); if (v >> 32) != 0 { s.limbs.push((v >> 32) as u32); } }
        s
    }
    fn bit_len(&self) -> usize {
        self.limbs.last().map_or(0, |&t| (self.limbs.len() - 1) * 32 + (32 - t.leading_zeros() as usize))
    }
    fn bit(&self, idx: usize) -> bool { self.limbs.get(idx / 32).is_some_and(|&v| (v >> (idx % 32)) & 1 != 0) }
    fn set_bit(&mut self, idx: usize) {
        let w = idx / 32;
        if self.limbs.len() <= w { self.limbs.resize(w + 1, 0); }
        self.limbs[w] |= 1u32 << (idx % 32);
    }
    fn shl1_add(&mut self, bit: bool) {
        let mut c = u32::from(bit);
        for l in &mut self.limbs { let n = *l >> 31; *l = (*l << 1) | c; c = n; }
        if c != 0 { self.limbs.push(c); }
    }
    fn mul_add_u32(&mut self, mul: u32, add: u32) {
        let mut c = add as u64;
        for l in &mut self.limbs { let p = (*l as u64) * (mul as u64) + c; *l = p as u32; c = p >> 32; }
        if c != 0 { self.limbs.push(c as u32); }
        self.trim();
    }
    pub fn from_decimal_digits(digits: &str) -> Result<Self, String> {
        if digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()) { return Err(format!("invalid decimal digits: {digits}")); }
        let mut out = Self::zero();
        for b in digits.bytes() { out.mul_add_u32(10, (b - b'0') as u32); }
        Ok(out)
    }
    pub fn cmp_val(&self, o: &Self) -> Ordering {
        self.limbs.len().cmp(&o.limbs.len()).then_with(|| self.limbs.iter().rev().cmp(o.limbs.iter().rev()))
    }
    pub fn add(&self, o: &Self) -> Self {
        let n = self.limbs.len().max(o.limbs.len());
        let (mut out, mut c) = (Vec::with_capacity(n + 1), 0u64);
        for i in 0..n {
            let s = self.limbs.get(i).copied().unwrap_or(0) as u64 + o.limbs.get(i).copied().unwrap_or(0) as u64 + c;
            out.push(s as u32); c = s >> 32;
        }
        if c != 0 { out.push(c as u32); }
        Self { limbs: out }
    }
    pub fn sub(&self, o: &Self) -> Self {
        let (mut out, mut b) = (Vec::with_capacity(self.limbs.len()), 0i64);
        for i in 0..self.limbs.len() {
            let mut d = self.limbs[i] as i64 - o.limbs.get(i).copied().unwrap_or(0) as i64 - b;
            if d < 0 { d += 1i64 << 32; b = 1; } else { b = 0; }
            out.push(d as u32);
        }
        let mut r = Self { limbs: out }; r.trim(); r
    }
    pub fn mul(&self, o: &Self) -> Self {
        if self.is_zero() || o.is_zero() { return Self::zero(); }
        let mut out = vec![0u32; self.limbs.len() + o.limbs.len()];
        for (i, &a) in self.limbs.iter().enumerate() {
            let mut c = 0u64;
            for (j, &b) in o.limbs.iter().enumerate() {
                let p = (a as u64) * (b as u64) + (out[i + j] as u64) + c;
                out[i + j] = p as u32; c = p >> 32;
            }
            out[i + o.limbs.len()] = c as u32;
        }
        let mut r = Self { limbs: out }; r.trim(); r
    }
    pub fn div_rem(&self, d: &Self) -> (Self, Self) {
        assert!(!d.is_zero(), "division by zero");
        if self.cmp_val(d) == Ordering::Less { return (Self::zero(), self.clone()); }
        let (mut q, mut r) = (Self::zero(), Self::zero());
        for idx in (0..self.bit_len()).rev() {
            r.shl1_add(self.bit(idx));
            if r.cmp_val(d) != Ordering::Less { r = r.sub(d); q.set_bit(idx); }
        }
        q.trim(); r.trim(); (q, r)
    }
    pub fn gcd(mut a: Self, mut b: Self) -> Self { while !b.is_zero() { let (_, r) = a.div_rem(&b); a = b; b = r; } a }
    pub fn pow10(mut exp: usize) -> Self {
        let (mut base, mut acc) = (Self::from_u64(10), Self::one());
        while exp > 0 { if exp & 1 == 1 { acc = acc.mul(&base); } exp >>= 1; if exp > 0 { base = base.mul(&base); } }
        acc
    }
    pub fn to_decimal_string(&self) -> String {
        if self.is_zero() { return "0".into(); }
        let (mut cur, base, mut chunks) = (self.clone(), Self::from_u64(1_000_000_000), Vec::new());
        while !cur.is_zero() { let (q, r) = cur.div_rem(&base); chunks.push(r.limbs.first().copied().unwrap_or(0)); cur = q; }
        let mut out = chunks.last().unwrap().to_string();
        for &c in chunks.iter().rev().skip(1) { out.push_str(&format!("{c:09}")); }
        out
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rational { sign: i8, num: BigUint, den: BigUint }

impl Rational {
    pub fn new(sign: i8, num: BigUint, den: BigUint) -> Result<Self, String> {
        if den.is_zero() { return Err("zero denominator in rational".into()); }
        if num.is_zero() || sign == 0 { return Ok(Self { sign: 0, num: BigUint::zero(), den: BigUint::one() }); }
        let g = BigUint::gcd(num.clone(), den.clone());
        Ok(Self { sign: if sign < 0 { -1 } else { 1 }, num: num.div_rem(&g).0, den: den.div_rem(&g).0 })
    }
    pub fn parse_decimal(raw: &str) -> Result<Self, String> {
        let (sign, rest) = split_sign(raw.trim())?;
        let (mant, exp_s) = rest.find(['e', 'E']).map_or((rest, None), |i| (&rest[..i], Some(&rest[i + 1..])));
        let (int_p, frac_p) = match mant.split_once('.') {
            Some((a, b)) if !b.contains('.') => (a, b),
            Some(_) => return Err(format!("multiple decimal points in {raw}")),
            None => (mant, ""),
        };
        if (int_p.is_empty() && frac_p.is_empty()) || !int_p.bytes().all(|b| b.is_ascii_digit()) || !frac_p.bytes().all(|b| b.is_ascii_digit()) {
            return Err(format!("invalid decimal {raw}"));
        }
        let exp: i64 = exp_s.map_or(Ok(0), |e| e.parse().map_err(|_| format!("invalid exponent in {raw}")))?;
        let scale = exp.checked_sub(frac_p.len() as i64).ok_or_else(|| format!("exponent overflow in {raw}"))?;
        if scale.unsigned_abs() > MAX_DECIMAL_EXP as u64 { return Err(format!("exponent magnitude exceeds limit in {raw}")); }
        let mag = BigUint::from_decimal_digits(&format!("{int_p}{frac_p}"))?;
        if mag.is_zero() { return Self::new(0, BigUint::zero(), BigUint::one()); }
        if scale >= 0 { Self::new(sign, mag.mul(&BigUint::pow10(scale as usize)), BigUint::one()) }
        else { Self::new(sign, mag, BigUint::pow10((-scale) as usize)) }
    }
    pub fn parse_rational(raw: &str) -> Result<Self, String> {
        let (ps, qs) = raw.trim().split_once('/').map_or((raw.trim(), "1"), |(p, q)| (p.trim(), q.trim()));
        let ((s1, p), (s2, q)) = (split_sign(ps)?, split_sign(qs)?);
        Self::new(s1 * s2, BigUint::from_decimal_digits(p)?, BigUint::from_decimal_digits(q)?)
    }
    pub fn parse_number_or_rational(raw: &str) -> Result<Self, String> {
        if raw.contains('/') { Self::parse_rational(raw) } else { Self::parse_decimal(raw) }
    }
    pub fn cmp_val(&self, o: &Self) -> Ordering {
        self.sign.cmp(&o.sign).then_with(|| {
            if self.sign == 0 { return Ordering::Equal; }
            let ord = self.num.mul(&o.den).cmp_val(&o.num.mul(&self.den));
            if self.sign > 0 { ord } else { ord.reverse() }
        })
    }
    pub fn abs(&self) -> Self { Self { sign: i8::from(self.sign != 0), num: self.num.clone(), den: self.den.clone() } }
    pub fn mul(&self, o: &Self) -> Self { Self::new(self.sign * o.sign, self.num.mul(&o.num), self.den.mul(&o.den)).unwrap() }
    pub fn abs_diff(&self, o: &Self) -> Self {
        let (a, b, den) = (self.num.mul(&o.den), o.num.mul(&self.den), self.den.mul(&o.den));
        let num = match (self.sign, o.sign) {
            (0, _) => b, (_, 0) => a, (s1, s2) if s1 != s2 => a.add(&b),
            _ => match a.cmp_val(&b) { Ordering::Greater => a.sub(&b), Ordering::Less => b.sub(&a), Ordering::Equal => BigUint::zero() },
        };
        Self::new(1, num, den).unwrap()
    }
    pub fn format_exact(&self) -> String {
        if self.sign == 0 { return "0".into(); }
        let p = if self.sign < 0 { "-" } else { "" };
        if self.den == BigUint::one() { format!("{p}{}", self.num.to_decimal_string()) }
        else { format!("{p}{}/{}", self.num.to_decimal_string(), self.den.to_decimal_string()) }
    }
}

fn split_sign(s: &str) -> Result<(i8, &str), String> {
    if s.is_empty() { Err("empty numeric value".into()) }
    else if let Some(r) = s.strip_prefix('-') { Ok((-1, r)) }
    else if let Some(r) = s.strip_prefix('+') { Ok((1, r)) }
    else { Ok((1, s)) }
}

pub fn compare_op(a: &Rational, op: &str, b: &Rational) -> Result<bool, String> {
    let ord = a.cmp_val(b);
    match op {
        "<" => Ok(ord == Ordering::Less), "<=" => Ok(ord != Ordering::Greater),
        ">" => Ok(ord == Ordering::Greater), ">=" => Ok(ord != Ordering::Less),
        "==" => Ok(ord == Ordering::Equal), "!=" => Ok(ord != Ordering::Equal),
        _ => Err(format!("unsupported comparison operator: {op}")),
    }
}

pub fn compare_close(obs: &Rational, exp: &Rational, mode: &str, tol: &Rational) -> Result<bool, String> {
    if tol.sign < 0 { return Err("tolerance must be non-negative".into()); }
    let diff = obs.abs_diff(exp);
    match mode {
        "abs" => Ok(diff.cmp_val(tol) != Ordering::Greater),
        "rel" => Ok(diff.cmp_val(&tol.mul(&exp.abs())) != Ordering::Greater),
        _ => Err(format!("close mode must be 'abs' or 'rel', got '{mode}'")),
    }
}

// ---------------------------------------------------------------------------
// UTC RFC 3339 timestamps, path redaction & shell quoting
// ---------------------------------------------------------------------------

pub fn now_utc_ms() -> i64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_millis() as i64)
}

fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = (z - era * 146_097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = (if mp < 10 { mp + 3 } else { mp - 9 }) as u32;
    (yoe as i64 + era * 400 + i64::from(m <= 2), m, d)
}

fn days_from_civil(y: i64, m: u32, d: u32) -> i64 {
    let y = y - i64::from(m <= 2);
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = (y - era * 400) as u64;
    let doy = (153 * (if m > 2 { m - 3 } else { m + 9 }) as u64 + 2) / 5 + (d as u64) - 1;
    era * 146_097 + (yoe * 365 + yoe / 4 - yoe / 100 + doy) as i64 - 719_468
}

pub fn format_utc_rfc3339_ms(epoch_ms: i64) -> String {
    let (secs, ms) = (epoch_ms.div_euclid(1000), epoch_ms.rem_euclid(1000));
    let (y, m, d) = civil_from_days(secs.div_euclid(86_400));
    let rem = secs.rem_euclid(86_400);
    format!("{y:04}-{m:02}-{d:02}T{:02}:{:02}:{:02}.{ms:03}Z", rem / 3600, (rem % 3600) / 60, rem % 60)
}

pub fn format_utc_compact(epoch_ms: i64) -> String {
    let secs = epoch_ms.div_euclid(1000);
    let (y, m, d) = civil_from_days(secs.div_euclid(86_400));
    let rem = secs.rem_euclid(86_400);
    format!("{y:04}{m:02}{d:02}T{:02}{:02}{:02}Z", rem / 3600, (rem % 3600) / 60, rem % 60)
}

pub fn parse_utc_rfc3339_ms(s: &str) -> Option<i64> {
    let b = s.as_bytes();
    if b.len() != 24 || b[4] != b'-' || b[7] != b'-' || b[10] != b'T' || b[13] != b':'
        || b[16] != b':' || b[19] != b'.' || b[23] != b'Z'
    {
        return None;
    }
    let (y, m, d): (i64, u32, u32) = (s[0..4].parse().ok()?, s[5..7].parse().ok()?, s[8..10].parse().ok()?);
    let (hh, mm, ss, ms): (i64, i64, i64, i64) = (
        s[11..13].parse().ok()?, s[14..16].parse().ok()?, s[17..19].parse().ok()?, s[20..23].parse().ok()?,
    );
    if !(1..=12).contains(&m) || !(1..=31).contains(&d) || hh > 23 || mm > 59 || ss > 60 { return None; }
    Some((days_from_civil(y, m, d) * 86_400 + hh * 3600 + mm * 60 + ss) * 1000 + ms)
}

pub fn normalize_lexical(path: &Path) -> PathBuf {
    let mut out = PathBuf::new();
    for c in path.components() {
        match c {
            Component::CurDir => {}
            Component::ParentDir => { if !out.pop() { out.push(c.as_os_str()); } }
            other => out.push(other.as_os_str()),
        }
    }
    out
}

pub fn path_is_under_root(candidate: &Path, root: &Path) -> bool {
    let (nr, nc) = (normalize_lexical(root), normalize_lexical(candidate));
    if nc == nr || nc.starts_with(&nr) { return true; }
    matches!((fs::canonicalize(root), fs::canonicalize(candidate)), (Ok(cr), Ok(cc)) if cc == cr || cc.starts_with(&cr))
}

pub fn redact_private_path(input: &str, private_root: Option<&str>) -> String {
    let Some(root_str) = private_root.filter(|r| !r.is_empty()) else { return input.to_string(); };
    let root = Path::new(root_str);
    if let Some((prefix, val)) = input.split_once('=')
        && prefix.starts_with('-') && path_is_under_root(Path::new(val), root)
    {
        return format!("{prefix}=<private:{}>", sha256_hex_prefix12(val.as_bytes()));
    }
    if path_is_under_root(Path::new(input), root) {
        return format!("<private:{}>", sha256_hex_prefix12(input.as_bytes()));
    }
    let trimmed = root_str.trim_end_matches('/');
    if !trimmed.is_empty() && input.contains(trimmed) {
        let mut out = String::with_capacity(input.len());
        let mut rest = input;
        while let Some(idx) = rest.find(trimmed) {
            out.push_str(&rest[..idx]);
            let tail = &rest[idx..];
            let end = tail.find(|c: char| c.is_whitespace() || matches!(c, '\'' | '"' | ',' | ';')).unwrap_or(tail.len());
            out.push_str(&format!("<private:{}>", sha256_hex_prefix12(tail[..end].as_bytes())));
            rest = &tail[end..];
        }
        out.push_str(rest);
        return out;
    }
    input.to_string()
}

pub fn effective_private_root(flag: Option<&str>) -> Option<String> {
    flag.filter(|v| !v.is_empty()).map(str::to_string)
        .or_else(|| env::var("FROB_PRIVATE_ROOT").ok().filter(|v| !v.is_empty()))
}

pub fn shell_quote(arg: &str) -> String {
    if arg.is_empty() { return "''".into(); }
    if arg.bytes().all(|b| b.is_ascii_alphanumeric() || b"_./:@%+=,-".contains(&b)) {
        arg.to_string()
    } else {
        format!("'{}'", arg.replace('\'', "'\\''"))
    }
}

pub fn format_cmdline(argv: &[String]) -> String {
    argv.iter().map(|a| shell_quote(a)).collect::<Vec<_>>().join(" ")
}

// ---------------------------------------------------------------------------
// Log recounting & transcript rendering
// ---------------------------------------------------------------------------

#[derive(Debug, Default)]
pub struct GateTally {
    pub asserts_passed: u64,
    pub asserts_failed: u64,
    pub tagged_controls_caught: u64,
    pub tagged_controls_missed: u64,
}

#[derive(Debug, Default)]
pub struct LogState {
    pub line_count: u64,
    pub body_sha256: String,
    pub script_start_ms: Option<i64>,
    pub scale: String,
    pub step_start_ms: Option<i64>,
    pub step_asserts_passed: u64,
    pub step_asserts_failed: u64,
    pub last_cmd_seq: Option<u64>,
    pub last_cmd_artifacts: Vec<String>,
    pub asserts_passed: u64,
    pub asserts_failed: u64,
    pub asserts_skipped: u64,
    pub controls_caught: u64,
    pub controls_missed: u64,
    pub untagged_controls_caught: u64,
    pub refusals_expected: u64,
    pub refusals_unexpected: u64,
    pub receipts: Vec<String>,
    pub gates: BTreeMap<String, GateTally>,
}

pub fn scan_log_bytes(raw: &[u8], active_step: Option<&str>) -> Result<LogState, String> {
    let mut st = LogState { body_sha256: sha256_hex(raw), scale: "full".into(), ..LogState::default() };
    let text = std::str::from_utf8(raw).map_err(|e| format!("log is not valid UTF-8: {e}"))?;
    for (idx, line) in text.lines().enumerate() {
        if line.is_empty() { continue; }
        st.line_count += 1;
        let val = json::parse_strict(line).map_err(|e| format!("invalid JSON on log line {}: {e}", idx + 1))?;
        let ev = val.get("event").and_then(JsonValue::as_str).unwrap_or("");
        let step = val.get("step").and_then(JsonValue::as_str).unwrap_or("");
        let ts_ms = val.get("ts").and_then(JsonValue::as_str).and_then(parse_utc_rfc3339_ms);
        match ev {
            "start" if step.is_empty() => {
                st.script_start_ms = ts_ms;
                if let Some(sc) = val.get("scale").and_then(JsonValue::as_str) { st.scale = sc.into(); }
            }
            "start" if active_step == Some(step) => {
                st.step_start_ms = ts_ms;
                st.step_asserts_passed = 0;
                st.step_asserts_failed = 0;
            }
            "cmd" => {
                st.last_cmd_seq = val.get("seq").and_then(JsonValue::as_u64);
                st.last_cmd_artifacts.clear();
                for k in ["stdout_path", "stderr_path"] {
                    if let Some(p) = val.get(k).and_then(JsonValue::as_str).filter(|s| !s.is_empty()) {
                        st.last_cmd_artifacts.push(p.into());
                    }
                }
                if let Some(r) = val.get("receipt").and_then(JsonValue::as_str).filter(|s| !s.is_empty())
                    && !st.receipts.iter().any(|x| x == r)
                {
                    st.receipts.push(r.into());
                }
            }
            "assert" => {
                let status = val.get("status").and_then(JsonValue::as_str).unwrap_or("");
                let kind = val.get("kind").and_then(JsonValue::as_str).unwrap_or("");
                let gate = val.get("gate").and_then(JsonValue::as_str).filter(|g| !g.is_empty());
                match status {
                    "pass" => {
                        st.asserts_passed += 1;
                        if active_step == Some(step) { st.step_asserts_passed += 1; }
                        if let Some(g) = gate { st.gates.entry(g.into()).or_default().asserts_passed += 1; }
                    }
                    "fail" => {
                        st.asserts_failed += 1;
                        if active_step == Some(step) { st.step_asserts_failed += 1; }
                        if let Some(g) = gate { st.gates.entry(g.into()).or_default().asserts_failed += 1; }
                    }
                    "skip" => st.asserts_skipped += 1,
                    _ => {}
                }
                if kind == "negative-control" {
                    match val.get("caught").and_then(JsonValue::as_bool) {
                        Some(true) => {
                            st.controls_caught += 1;
                            if let Some(g) = gate { st.gates.entry(g.into()).or_default().tagged_controls_caught += 1; }
                            else { st.untagged_controls_caught += 1; }
                        }
                        Some(false) => {
                            st.controls_missed += 1;
                            if let Some(g) = gate { st.gates.entry(g.into()).or_default().tagged_controls_missed += 1; }
                        }
                        None => {}
                    }
                }
            }
            "metric" => {
                if let Some(g) = val.get("gate").and_then(JsonValue::as_str).filter(|g| !g.is_empty()) {
                    st.gates.entry(g.into()).or_default();
                }
            }
            "refusal" => {
                if val.get("expected").and_then(JsonValue::as_bool) == Some(true) {
                    st.refusals_expected += 1;
                } else {
                    st.refusals_unexpected += 1;
                }
            }
            _ => {}
        }
    }
    Ok(st)
}

fn s_of<'a>(v: &'a JsonValue, k: &str) -> &'a str { v.get(k).and_then(JsonValue::as_str).unwrap_or("") }
fn u_of(v: &JsonValue, k: &str) -> u64 { v.get(k).and_then(JsonValue::as_u64).unwrap_or(0) }
fn compact_disp(v: &JsonValue) -> String { match v { JsonValue::String(s) => s.clone(), o => o.to_json_string() } }
fn repair_str(v: &JsonValue) -> String {
    if let Some(s) = v.as_str() { return s.into(); }
    for k in ["code", "id", "action", "kind"] {
        if let Some(s) = v.get(k).and_then(JsonValue::as_str) { return s.into(); }
    }
    v.to_json_string()
}

pub fn format_transcript_lines(val: &JsonValue) -> Vec<String> {
    let (ts, step, ev) = (s_of(val, "ts"), s_of(val, "step"), s_of(val, "event"));
    let st = if step.is_empty() { "script" } else { step };
    let mut out = Vec::new();
    match ev {
        "start" if step.is_empty() => out.push(format!(
            "[{ts}] START {} ({}) run={} seed={} scale={}",
            s_of(val, "script"), s_of(val, "bead"), s_of(val, "run_id"), s_of(val, "seed"), s_of(val, "scale")
        )),
        "start" => out.push(format!("[{ts}] STEP {step}: {}", s_of(val, "title"))),
        "cmd" => {
            let mut l = format!(
                "[{ts}] [{st}] CMD seq={} exit={} wall={}ms: {}",
                u_of(val, "seq"), val.get("exit").and_then(JsonValue::as_i64).unwrap_or(-1),
                u_of(val, "wall_ms"), s_of(val, "cmdline")
            );
            let r = s_of(val, "receipt");
            if !r.is_empty() { l.push_str(&format!(" receipt={r}")); }
            out.push(l);
        }
        "assert" => {
            let status = s_of(val, "status");
            let tag = match status { "pass" => "PASS", "fail" => "FAIL", _ => "SKIP" };
            let mut l = if status == "skip" {
                let why = val.get("reason").or_else(|| val.get("observed")).map(compact_disp).unwrap_or_default();
                format!("[{ts}] [{st}] {tag} {}: {why}", s_of(val, "name"))
            } else {
                let obs = val.get("observed").map(compact_disp).unwrap_or_default();
                let exp = val.get("expected").map(compact_disp).unwrap_or_default();
                let tol = val.get("tolerance").map(compact_disp).unwrap_or_default();
                let unit = s_of(val, "unit");
                let mut s = format!("[{ts}] [{st}] {tag} {}: {obs} vs {exp}", s_of(val, "name"));
                if !tol.is_empty() || !unit.is_empty() { s.push_str(&format!(" ({tol}, {unit})")); }
                s
            };
            if status == "fail" && let Some(fd) = val.get("first_diff") {
                l.push_str(&format!(
                    " [diff {}: {} != {}]", s_of(fd, "path"),
                    fd.get("observed").map(compact_disp).unwrap_or_default(),
                    fd.get("expected").map(compact_disp).unwrap_or_default()
                ));
            }
            out.push(l);
        }
        "metric" => {
            let mut l = format!(
                "[{ts}] [{st}] METRIC {}={} {}",
                s_of(val, "name"), val.get("value").map(compact_disp).unwrap_or_default(), s_of(val, "unit")
            );
            if let Some(th) = val.get("threshold") {
                l.push_str(&format!(" ({} {})", s_of(th, "op"), th.get("value").map(compact_disp).unwrap_or_default()));
            }
            if val.get("asserted").and_then(JsonValue::as_bool) == Some(false) {
                l.push_str(&format!(" [observe: {}]", s_of(val, "verdict")));
            }
            out.push(l);
        }
        "refusal" => {
            let exp = val.get("expected").and_then(JsonValue::as_bool).unwrap_or(false);
            let code = s_of(val, "code");
            let mut l = format!(
                "[{ts}] [{st}] REFUSAL {code} exit={} expected={exp}: {}",
                val.get("exit").and_then(JsonValue::as_i64).unwrap_or(-1), s_of(val, "message")
            );
            if let Some(first) = val.get("repairs").and_then(JsonValue::as_array).and_then(|a| a.first()) {
                l.push_str(&format!(" repair={}", repair_str(first)));
            }
            out.push(l);
            if !exp && code == "FROB-REF-805" {
                out.push("hint: pass --key \"$(e2e_key <label>)\"".into());
            }
        }
        "end" => out.push(format!(
            "[{ts}] END {st} wall={}ms passed={} failed={}",
            u_of(val, "wall_ms"), u_of(val, "asserts_passed"), u_of(val, "asserts_failed")
        )),
        "summary" => {
            let (cc, cm) = (u_of(val, "controls_caught"), u_of(val, "controls_missed"));
            out.push(format!(
                "[{ts}] SUMMARY exit={} passed={} failed={} skipped={} controls={cc}/{} refusals={}/{} registries={} claimable={} wall={}ms",
                val.get("exit").and_then(JsonValue::as_i64).unwrap_or(-1),
                u_of(val, "asserts_passed"), u_of(val, "asserts_failed"), u_of(val, "asserts_skipped"),
                cc + cm, u_of(val, "refusals_expected"), u_of(val, "refusals_unexpected"),
                s_of(val, "registries"), val.get("gates_claimable").and_then(JsonValue::as_bool).unwrap_or(false),
                u_of(val, "wall_ms")
            ));
        }
        _ => out.push(format!("[{ts}] [{st}] {ev}")),
    }
    out
}

pub fn append_log_line(log_path: &Path, transcript_path: Option<&Path>, val: &JsonValue) -> Result<(), String> {
    let serialized = json::validate_line_for_emit(val)?;
    if let Some(p) = log_path.parent().filter(|p| !p.as_os_str().is_empty()) {
        fs::create_dir_all(p).map_err(|e| format!("create log dir: {e}"))?;
    }
    let mut f = OpenOptions::new().create(true).append(true).open(log_path)
        .map_err(|e| format!("open log {}: {e}", log_path.display()))?;
    f.write_all(serialized.as_bytes()).and_then(|_| f.write_all(b"\n")).map_err(|e| format!("write log: {e}"))?;
    if let Some(tp) = transcript_path {
        if let Some(p) = tp.parent().filter(|p| !p.as_os_str().is_empty()) {
            fs::create_dir_all(p).map_err(|e| format!("create transcript dir: {e}"))?;
        }
        let mut tf = OpenOptions::new().create(true).append(true).open(tp)
            .map_err(|e| format!("open transcript {}: {e}", tp.display()))?;
        for line in format_transcript_lines(val) {
            writeln!(tf, "{line}").map_err(|e| format!("write transcript: {e}"))?;
        }
    }
    Ok(())
}

fn metric_value_json(raw: &str) -> JsonValue {
    if raw.contains('/') { return JsonValue::String(raw.into()); }
    if let Ok(JsonValue::Number(n)) = json::parse_strict(raw) {
        if !n.contains(['.', 'e', 'E']) && n.parse::<i128>().is_ok_and(|i| i.abs() > MAX_SAFE_INT) {
            return JsonValue::String(raw.into());
        }
        return JsonValue::Number(n);
    }
    JsonValue::String(raw.into())
}

fn parse_bool(raw: &str, flag: &str) -> Result<bool, String> {
    match raw { "true" | "1" => Ok(true), "false" | "0" => Ok(false), _ => Err(format!("invalid boolean for {flag}: {raw}")) }
}

fn base_line(ts: String, seq: u64, run_id: &str, script: &str, bead: &str, step: &str, ev: &str, seed: &str) -> Vec<(String, JsonValue)> {
    vec![
        ("schema".into(), JsonValue::String(LOG_SCHEMA_VERSION.into())), ("ts".into(), JsonValue::String(ts)),
        ("seq".into(), JsonValue::Number(seq.to_string())), ("run_id".into(), JsonValue::String(run_id.into())),
        ("script".into(), JsonValue::String(script.into())), ("bead".into(), JsonValue::String(bead.into())),
        ("step".into(), JsonValue::String(step.into())), ("event".into(), JsonValue::String(ev.into())),
        ("seed".into(), JsonValue::String(seed.into())),
    ]
}

fn fail_meta(path: &str, obs: String, exp: String, cmd_seq: u64, arts: Vec<JsonValue>) -> Vec<(String, JsonValue)> {
    vec![
        ("first_diff".into(), obj(vec![("path", JsonValue::String(path.into())), ("observed", JsonValue::String(obs)), ("expected", JsonValue::String(exp))])),
        ("repro".into(), obj(vec![("cmd_seq", JsonValue::Number(cmd_seq.to_string())), ("artifacts", JsonValue::Array(arts))])),
    ]
}

pub fn cmd_emit(args: &[String]) -> i32 {
    match run_emit(args) { Ok(out) => { print!("{out}"); 0 } Err(e) => { eprintln!("frob-e2e emit: {e}"); 2 } }
}

pub fn run_emit(args: &[String]) -> Result<String, String> {
    const VALUED: &[&str] = &[
        "--log", "--transcript", "--event", "--run-id", "--script", "--bead", "--step", "--seed",
        "--ts", "--seq", "--private-root", "--raw-json", "--commit", "--dirty", "--toolchain",
        "--frob-version", "--host-os", "--host-arch", "--host-fingerprint", "--scale", "--devices",
        "--plant", "--title", "--name", "--kind", "--status", "--observed", "--expected", "--op",
        "--tolerance", "--unit", "--gate", "--ka", "--cx", "--nc", "--mr", "--caught",
        "--diff-path", "--diff-observed", "--diff-expected", "--cmd-seq", "--reason", "--value",
        "--threshold", "--verdict", "--interval-lo", "--interval-hi", "--n", "--code", "--message",
        "--pointer", "--required", "--retry", "--repairs", "--registries", "--gates", "--exit", "--wall-ms",
    ];
    let f = ParsedFlags::parse(args, VALUED, &["--arg", "--artifact"], &["--observe"])?;
    if !f.positionals.is_empty() { return Err(format!("unexpected positional arg: {}", f.positionals[0])); }
    let (log_path, tr_path) = (PathBuf::from(f.get("--log").ok_or("missing --log")?), f.get("--transcript").map(PathBuf::from));
    let priv_root = effective_private_root(f.get("--private-root"));
    let pr = priv_root.as_deref();
    if let Some(raw) = f.get("--raw-json") {
        let val = json::parse_strict(raw).map_err(|e| format!("--raw-json: {e}"))?;
        if val.as_object().is_none() { return Err("--raw-json must be a JSON object".into()); }
        append_log_line(&log_path, tr_path.as_deref(), &val)?;
        return Ok(String::new());
    }
    let ev = f.get("--event").ok_or("missing --event")?;
    if !VALID_EVENTS.contains(&ev) { return Err(format!("unknown --event: {ev}")); }
    let seed = f.get_or("--seed", "0");
    seed.parse::<u64>().map_err(|_| format!("invalid --seed: {seed}"))?;
    let step = f.get_or("--step", "");
    let existing = if log_path.exists() { fs::read(&log_path).map_err(|e| format!("read log: {e}"))? } else { Vec::new() };
    let state = scan_log_bytes(&existing, (!step.is_empty()).then_some(step))?;
    let now_ms = now_utc_ms();
    let ts = f.get("--ts").map_or_else(|| format_utc_rfc3339_ms(now_ms), str::to_string);
    let seq = f.get("--seq").map_or(Ok(state.line_count), |s| s.parse::<u64>().map_err(|_| format!("invalid --seq: {s}")))?;
    let mut pairs = base_line(ts, seq, f.get_or("--run-id", ""), f.get_or("--script", ""), f.get_or("--bead", ""), step, ev, seed);
    let mut stdout_out = String::new();

    match ev {
        "start" if step.is_empty() => {
            let scale = f.get_or("--scale", "full");
            if !matches!(scale, "full" | "small") { return Err(format!("invalid --scale: {scale}")); }
            let dirty = f.get("--dirty").map(|v| parse_bool(v, "--dirty")).transpose()?.unwrap_or(false);
            let devices = f.get_or("--devices", "").split(',').filter(|s| !s.is_empty()).map(|s| JsonValue::String(s.into())).collect();
            let argv = f.list("--arg").iter().map(|a| JsonValue::String(redact_private_path(a, pr))).collect();
            pairs.extend([
                ("commit".into(), JsonValue::String(f.get_or("--commit", "unknown").into())), ("dirty".into(), JsonValue::Bool(dirty)),
                ("toolchain".into(), JsonValue::String(f.get_or("--toolchain", "unknown").into())),
                ("frob_version".into(), JsonValue::String(f.get_or("--frob-version", "unknown").into())),
                ("host".into(), obj(vec![
                    ("os", JsonValue::String(f.get_or("--host-os", env::consts::OS).into())),
                    ("arch", JsonValue::String(f.get_or("--host-arch", env::consts::ARCH).into())),
                    ("fingerprint", JsonValue::String(f.get_or("--host-fingerprint", "unknown").into())),
                ])),
                ("scale".into(), JsonValue::String(scale.into())), ("devices".into(), JsonValue::Array(devices)),
                ("plant".into(), JsonValue::String(f.get_or("--plant", "").into())), ("argv".into(), JsonValue::Array(argv)),
            ]);
        }
        "start" => pairs.push(("title".into(), JsonValue::String(redact_private_path(f.get_or("--title", ""), pr)))),
        "end" => {
            let start_ms = if step.is_empty() { state.script_start_ms } else { state.step_start_ms };
            let wall = f.get("--wall-ms").map_or_else(|| Ok(start_ms.map_or(0, |s| (now_ms - s).max(0) as u64)), |w| w.parse::<u64>().map_err(|_| format!("invalid --wall-ms: {w}")))?;
            let (ap, af) = if step.is_empty() { (state.asserts_passed, state.asserts_failed) } else { (state.step_asserts_passed, state.step_asserts_failed) };
            pairs.extend([("wall_ms".into(), JsonValue::Number(wall.to_string())), ("asserts_passed".into(), JsonValue::Number(ap.to_string())), ("asserts_failed".into(), JsonValue::Number(af.to_string()))]);
            if step.is_empty() && let Some(ex) = f.get("--exit") {
                ex.parse::<i64>().map_err(|_| format!("invalid --exit: {ex}"))?;
                pairs.push(("exit".into(), JsonValue::Number(ex.into())));
            }
        }
        "assert" => {
            let (aname, akind, astatus) = (f.get("--name").ok_or("assert requires --name")?, f.get("--kind").ok_or("assert requires --kind")?, f.get("--status").ok_or("assert requires --status")?);
            if !VALID_ASSERT_KINDS.contains(&akind) { return Err(format!("invalid assert --kind: {akind}")); }
            if !matches!(astatus, "pass" | "fail" | "skip") { return Err(format!("invalid assert --status: {astatus}")); }
            pairs.extend([("name".into(), JsonValue::String(aname.into())), ("kind".into(), JsonValue::String(akind.into())), ("status".into(), JsonValue::String(astatus.into()))]);
            for (fl, k, red) in [("--observed", "observed", true), ("--expected", "expected", true), ("--op", "op", false), ("--tolerance", "tolerance", false), ("--unit", "unit", false), ("--gate", "gate", false), ("--ka", "ka", false), ("--cx", "cx", false), ("--nc", "nc", false), ("--mr", "mr", false), ("--reason", "reason", true)] {
                if let Some(v) = f.get(fl) { pairs.push((k.into(), JsonValue::String(if red { redact_private_path(v, pr) } else { v.into() }))); }
            }
            if let Some(c) = f.get("--caught") { pairs.push(("caught".into(), JsonValue::Bool(parse_bool(c, "--caught")?))); }
            if astatus == "fail" {
                let cseq = f.get("--cmd-seq").map_or(Ok(state.last_cmd_seq.unwrap_or(0)), |s| s.parse::<u64>().map_err(|_| format!("invalid --cmd-seq: {s}")))?;
                let arts = if f.list("--artifact").is_empty() { state.last_cmd_artifacts.into_iter().map(JsonValue::String).collect() }
                    else { f.list("--artifact").iter().map(|a| JsonValue::String(redact_private_path(a, pr))).collect() };
                pairs.extend(fail_meta(f.get_or("--diff-path", ""), redact_private_path(f.get("--diff-observed").or_else(|| f.get("--observed")).unwrap_or(""), pr), redact_private_path(f.get("--diff-expected").or_else(|| f.get("--expected")).unwrap_or(""), pr), cseq, arts));
            }
        }
        "metric" => {
            let (mname, mval, munit) = (f.get("--name").ok_or("metric requires --name")?, f.get("--value").ok_or("metric requires --value")?, f.get("--unit").ok_or("metric requires --unit")?);
            pairs.extend([("name".into(), JsonValue::String(mname.into())), ("value".into(), metric_value_json(mval)), ("unit".into(), JsonValue::String(munit.into()))]);
            let mut verdict = f.get("--verdict").map(str::to_string);
            if let (Some(mop), Some(mth)) = (f.get("--op"), f.get("--threshold")) {
                pairs.push(("threshold".into(), obj(vec![("op", JsonValue::String(mop.into())), ("value", metric_value_json(mth))])));
                if f.has("--observe") && verdict.is_none() {
                    let met = compare_op(&Rational::parse_number_or_rational(mval)?, mop, &Rational::parse_number_or_rational(mth)?)?;
                    verdict = Some(if met { "met" } else { "not-met" }.into());
                }
            }
            if f.has("--observe") { pairs.push(("asserted".into(), JsonValue::Bool(false))); }
            if let Some(v) = verdict {
                if !matches!(v.as_str(), "met" | "not-met") { return Err(format!("invalid --verdict: {v}")); }
                pairs.push(("verdict".into(), JsonValue::String(v)));
            }
            if let (Some(lo), Some(hi)) = (f.get("--interval-lo"), f.get("--interval-hi")) {
                pairs.push(("interval".into(), obj(vec![("lo", metric_value_json(lo)), ("hi", metric_value_json(hi))])));
            }
            if let Some(n) = f.get("--n") { n.parse::<u64>().map_err(|_| format!("invalid --n: {n}"))?; pairs.push(("n".into(), JsonValue::Number(n.into()))); }
            if let Some(g) = f.get("--gate") { pairs.push(("gate".into(), JsonValue::String(g.into()))); }
        }
        "refusal" => {
            let code = f.get("--code").ok_or("refusal requires --code")?;
            let ex = f.get("--exit").map(|v| v.parse::<i64>().map_err(|_| format!("invalid --exit: {v}"))).transpose()?.unwrap_or(3);
            let retry = f.get("--retry").map(|v| parse_bool(v, "--retry")).transpose()?.unwrap_or(false);
            let expected = f.get("--expected").map(|v| parse_bool(v, "--expected")).transpose()?.unwrap_or(false);
            let repairs = f.get_or("--repairs", "").split(',').filter(|s| !s.is_empty()).map(|s| JsonValue::String(s.into())).collect();
            pairs.extend([
                ("code".into(), JsonValue::String(code.into())), ("exit".into(), JsonValue::Number(ex.to_string())),
                ("message".into(), JsonValue::String(redact_private_path(f.get_or("--message", ""), pr))),
                ("pointer".into(), JsonValue::String(f.get_or("--pointer", "").into())),
                ("observed".into(), JsonValue::String(redact_private_path(f.get_or("--observed", ""), pr))),
                ("required".into(), JsonValue::String(f.get_or("--required", "").into())),
                ("unit".into(), JsonValue::String(f.get_or("--unit", "").into())), ("retry".into(), JsonValue::Bool(retry)),
                ("repairs".into(), JsonValue::Array(repairs)), ("expected".into(), JsonValue::Bool(expected)),
            ]);
        }
        "summary" => {
            let registries = f.get_or("--registries", "of-record");
            if !matches!(registries, "of-record" | "override") { return Err(format!("invalid --registries: {registries}")); }
            let mut gate_ids: BTreeSet<String> = state.gates.keys().cloned().collect();
            gate_ids.extend(f.get_or("--gates", "").split(',').filter(|s| !s.is_empty()).map(str::to_string));
            let (mut gate_objs, mut all_ok) = (Vec::new(), !gate_ids.is_empty());
            for gid in &gate_ids {
                let (gp, gf, tcc, tcm) = state.gates.get(gid).map_or((0, 0, 0, 0), |t| (t.asserts_passed, t.asserts_failed, t.tagged_controls_caught, t.tagged_controls_missed));
                let cc = tcc + state.untagged_controls_caught;
                let ok = if gf > 0 || tcm > 0 || state.controls_missed > 0 || state.refusals_unexpected > 0 { false }
                    else if gp > 0 { true } else { state.asserts_passed > 0 && state.asserts_failed == 0 };
                if !ok || cc == 0 { all_ok = false; }
                gate_objs.push(obj(vec![("id", JsonValue::String(gid.clone())), ("measured_ok", JsonValue::Bool(ok)), ("controls_caught", JsonValue::Number(cc.to_string()))]));
            }
            let claimable = state.scale == "full" && registries == "of-record" && state.asserts_failed == 0 && state.controls_missed == 0 && state.refusals_unexpected == 0 && all_ok;
            let exit_code = match f.get("--exit") {
                Some(v) => v.parse::<i64>().map_err(|_| format!("invalid --exit: {v}"))?,
                None if state.asserts_failed > 0 || state.controls_missed > 0 || state.refusals_unexpected > 0 => 1,
                None if state.asserts_passed > 0 => 0, None if state.asserts_skipped > 0 => 77, None => 2,
            };
            let wall = f.get("--wall-ms").map_or_else(|| Ok(state.script_start_ms.map_or(0, |s| (now_ms - s).max(0) as u64)), |v| v.parse::<u64>().map_err(|_| format!("invalid --wall-ms: {v}")))?;
            pairs.extend([
                ("asserts_passed".into(), JsonValue::Number(state.asserts_passed.to_string())), ("asserts_failed".into(), JsonValue::Number(state.asserts_failed.to_string())),
                ("asserts_skipped".into(), JsonValue::Number(state.asserts_skipped.to_string())), ("controls_caught".into(), JsonValue::Number(state.controls_caught.to_string())),
                ("controls_missed".into(), JsonValue::Number(state.controls_missed.to_string())), ("refusals_expected".into(), JsonValue::Number(state.refusals_expected.to_string())),
                ("refusals_unexpected".into(), JsonValue::Number(state.refusals_unexpected.to_string())), ("gates".into(), JsonValue::Array(gate_objs)),
                ("gates_claimable".into(), JsonValue::Bool(claimable)), ("registries".into(), JsonValue::String(registries.into())),
                ("receipts".into(), JsonValue::Array(state.receipts.into_iter().map(JsonValue::String).collect())),
                ("exit".into(), JsonValue::Number(exit_code.to_string())), ("wall_ms".into(), JsonValue::Number(wall.to_string())),
                ("body_sha256".into(), JsonValue::String(state.body_sha256)),
            ]);
            stdout_out = format!(
                "E2E_SUMMARY_EXIT={exit_code}\nE2E_ASSERTS_PASSED={}\nE2E_ASSERTS_FAILED={}\nE2E_ASSERTS_SKIPPED={}\nE2E_CONTROLS_CAUGHT={}\nE2E_CONTROLS_MISSED={}\n",
                state.asserts_passed, state.asserts_failed, state.asserts_skipped, state.controls_caught, state.controls_missed
            );
        }
        "cmd" => return Err("use `frob-e2e run` or `--raw-json` for cmd events".into()),
        _ => unreachable!(),
    }
    append_log_line(&log_path, tr_path.as_deref(), &JsonValue::Object(pairs))?;
    Ok(stdout_out)
}

// ---------------------------------------------------------------------------
// Subcommand: run
// ---------------------------------------------------------------------------

pub fn cmd_run(args: &[String]) -> i32 {
    match run_exec(args) {
        Ok(out) => { print!("{out}"); 0 }
        Err(e) => { eprintln!("frob-e2e run: {e}"); 2 }
    }
}

pub fn run_exec(args: &[String]) -> Result<String, String> {
    let sep = args.iter().position(|a| a == "--").ok_or("missing `--` before command")?;
    let (flag_args, child_argv) = (&args[..sep], &args[sep + 1..]);
    if child_argv.is_empty() { return Err("missing command after `--`".into()); }
    const VALUED: &[&str] = &[
        "--log", "--transcript", "--artifacts-dir", "--scratch-root", "--run-id", "--script",
        "--bead", "--step", "--seed", "--name", "--expect-exit", "--expect-refusal",
        "--expect-caught", "--gate", "--ka", "--cx", "--nc", "--mr", "--tier-receipts-dir",
        "--secret", "--secret-out", "--private-root",
    ];
    let f = ParsedFlags::parse(flag_args, VALUED, &["--unset-env"], &[])?;
    if !f.positionals.is_empty() { return Err(format!("unexpected arg before `--`: {}", f.positionals[0])); }
    let log_path = PathBuf::from(f.get("--log").ok_or("missing --log")?);
    let tr_path = f.get("--transcript").map(PathBuf::from);
    let arts_dir = PathBuf::from(f.get("--artifacts-dir").ok_or("missing --artifacts-dir")?);
    let scratch_root = f.get("--scratch-root").map(PathBuf::from).unwrap_or_else(|| env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));
    let priv_root = effective_private_root(f.get("--private-root"));
    let pr = priv_root.as_deref();
    if f.get("--secret").is_some() && f.get("--secret-out").is_none() { return Err("--secret requires --secret-out".into()); }
    let seed = f.get_or("--seed", "0");
    seed.parse::<u64>().map_err(|_| format!("invalid --seed: {seed}"))?;
    let (run_id, script, bead, step) = (f.get_or("--run-id", ""), f.get_or("--script", ""), f.get_or("--bead", ""), f.get_or("--step", ""));
    let exp_ref = f.get("--expect-refusal");
    let exp_caught = f.get("--expect-caught");
    let expected_exits: Vec<i32> = match f.get("--expect-exit") {
        Some(csv) => csv.split(',').filter(|s| !s.is_empty()).map(|s| s.parse::<i32>().map_err(|_| format!("invalid exit code: {s}"))).collect::<Result<_, _>>()?,
        None => vec![if exp_ref.is_some() { 3 } else { 0 }],
    };
    if expected_exits.is_empty() { return Err("--expect-exit cannot be empty".into()); }
    let label = f.get("--name").map(str::to_string).unwrap_or_else(|| {
        Path::new(&child_argv[0]).file_name().map_or_else(|| child_argv[0].clone(), |n| n.to_string_lossy().into_owned())
    });

    let tier_dir = f.get("--tier-receipts-dir").map(PathBuf::from);
    let before_receipts = snapshot_dir_names(tier_dir.as_deref());
    let existing = if log_path.exists() { fs::read(&log_path).map_err(|e| format!("read log: {e}"))? } else { Vec::new() };
    let mut seq = scan_log_bytes(&existing, None)?.line_count;
    let cmd_seq = seq;

    let cwd_rel = env::current_dir().ok().map_or_else(|| ".".into(), |cwd| {
        let (nc, ns) = (normalize_lexical(&cwd), normalize_lexical(&scratch_root));
        nc.strip_prefix(&ns).map_or_else(
            |_| redact_private_path(&nc.to_string_lossy(), pr),
            |r| { let s = r.to_string_lossy().into_owned(); if s.is_empty() { ".".into() } else { s } },
        )
    });

    let mut command = Command::new(&child_argv[0]);
    command.args(&child_argv[1..]);
    for v in f.list("--unset-env") { command.env_remove(v); }
    let started = Instant::now();
    let output = command.output().map_err(|e| format!("failed to execute `{}`: {e}", child_argv[0]))?;
    let wall_ms = started.elapsed().as_millis() as u64;
    let child_exit = status_to_code(&output.status);

    let (mut stdout_bytes, mut stderr_bytes) = (output.stdout, output.stderr);
    let (mut secret_written, mut tok_redaction) = (0u8, None);
    if let (Some(ptr), Some(out_p)) = (f.get("--secret"), f.get("--secret-out"))
        && let Ok(text) = std::str::from_utf8(&stdout_bytes)
        && let Ok(parsed) = json::parse_strict(text)
        && let Some(JsonValue::String(tok)) = parsed.pointer(ptr)
        && !tok.is_empty()
    {
        write_secret_file(Path::new(out_p), tok.as_bytes())?;
        let tag = format!("<token:{}>", sha256_hex_prefix12(tok.as_bytes()));
        stdout_bytes = replace_bytes(&stdout_bytes, tok.as_bytes(), tag.as_bytes());
        stderr_bytes = replace_bytes(&stderr_bytes, tok.as_bytes(), tag.as_bytes());
        let mut esc = String::new();
        json::write_escaped_string(tok, &mut esc);
        let inner = &esc[1..esc.len() - 1];
        if inner != tok {
            stdout_bytes = replace_bytes(&stdout_bytes, inner.as_bytes(), tag.as_bytes());
            stderr_bytes = replace_bytes(&stderr_bytes, inner.as_bytes(), tag.as_bytes());
        }
        secret_written = 1;
        tok_redaction = Some((tok.clone(), tag));
    }

    let env_opt = std::str::from_utf8(&stdout_bytes).ok().and_then(|t| json::parse_strict(t).ok()).filter(is_frob_envelope);
    let is_priv = env_opt.as_ref().is_some_and(envelope_is_private);
    let stdout_sha256 = sha256_hex(&stdout_bytes);
    let step_tag = if step.is_empty() { "script" } else { step };
    let (out_name, err_name) = (format!("{step_tag}.{cmd_seq}.stdout"), format!("{step_tag}.{cmd_seq}.stderr"));
    let (stdout_p, stderr_p) = if is_priv {
        if let Some(root) = pr {
            let cap = Path::new(root).join("captures").join(run_id);
            fs::create_dir_all(&cap).map_err(|e| format!("create private capture dir: {e}"))?;
            fs::write(cap.join(&out_name), &stdout_bytes).map_err(|e| format!("write private stdout: {e}"))?;
            fs::write(cap.join(&err_name), &stderr_bytes).map_err(|e| format!("write private stderr: {e}"))?;
        }
        (String::new(), String::new())
    } else {
        fs::create_dir_all(&arts_dir).map_err(|e| format!("create artifacts dir: {e}"))?;
        let (op, ep) = (arts_dir.join(&out_name), arts_dir.join(&err_name));
        fs::write(&op, &stdout_bytes).map_err(|e| format!("write stdout capture: {e}"))?;
        fs::write(&ep, &stderr_bytes).map_err(|e| format!("write stderr capture: {e}"))?;
        (op.to_string_lossy().into_owned(), ep.to_string_lossy().into_owned())
    };

    let (mut receipt_hash, mut receipt_path, mut receipt_bad) = (None, String::new(), None);
    if let Some(rdir) = tier_dir.as_deref() {
        let mut added: Vec<String> = snapshot_dir_names(Some(rdir)).difference(&before_receipts).cloned().collect();
        added.sort();
        if added.len() == 1 {
            receipt_hash = Some(added[0].strip_suffix(".json").unwrap_or(&added[0]).into());
            receipt_path = rdir.join(&added[0]).to_string_lossy().into_owned();
        } else {
            receipt_bad = Some(added.len());
        }
    }

    let redacted_argv: Vec<String> = child_argv.iter().map(|a| {
        let mut s = redact_private_path(a, pr);
        if let Some((tok, tag)) = &tok_redaction { s = s.replace(tok, tag); }
        s
    }).collect();
    let cmdline = format_cmdline(&redacted_argv);
    let mut cmd_pairs = base_line(format_utc_rfc3339_ms(now_utc_ms()), seq, run_id, script, bead, step, "cmd", seed);
    cmd_pairs.extend([
        ("argv".into(), JsonValue::Array(redacted_argv.into_iter().map(JsonValue::String).collect())),
        ("cmdline".into(), JsonValue::String(cmdline)),
        ("cwd".into(), JsonValue::String(cwd_rel)),
        ("exit".into(), JsonValue::Number(child_exit.to_string())),
        ("expected_exit".into(), JsonValue::Array(expected_exits.iter().map(|c| JsonValue::Number(c.to_string())).collect())),
        ("wall_ms".into(), JsonValue::Number(wall_ms.to_string())),
        ("stdout_sha256".into(), JsonValue::String(stdout_sha256)),
    ]);
    if !is_priv {
        cmd_pairs.push(("stdout_path".into(), JsonValue::String(stdout_p.clone())));
        cmd_pairs.push(("stderr_path".into(), JsonValue::String(stderr_p.clone())));
    }
    if let Some(rh) = receipt_hash { cmd_pairs.push(("receipt".into(), JsonValue::String(rh))); }
    if let Some(ev) = &env_opt {
        cmd_pairs.push(("envelope".into(), obj(vec![
            ("schema", JsonValue::String(ev.get("schema").and_then(JsonValue::as_str).unwrap_or("frob.envelope/1").into())),
            ("verb", JsonValue::String(s_of(ev, "verb").into())),
            ("ok", JsonValue::Bool(ev.get("ok").and_then(JsonValue::as_bool).unwrap_or(false))),
            ("exit", JsonValue::Number(ev.get("exit").and_then(JsonValue::as_i64).unwrap_or(child_exit as i64).to_string())),
        ])));
        if let Some(eb) = ev.get("evidence") {
            let mut ev_out = eb.clone();
            if let JsonValue::Object(ref mut fields) = ev_out
                && !fields.iter().any(|(k, _)| k == "identities")
                && (ev.get("bound").is_some() || ev.get("model_id").is_some())
            {
                let mut ids = Vec::new();
                if let Some(b) = ev.get("bound") { ids.push(("bound".into(), b.clone())); }
                if let Some(m) = ev.get("model_id") { ids.push(("model_id".into(), m.clone())); }
                fields.push(("identities".into(), JsonValue::Object(ids)));
            }
            if json::check_redaction_tree(&ev_out, "/evidence").is_ok() { cmd_pairs.push(("evidence".into(), ev_out)); }
        }
        if !is_priv {
            for k in ["cost", "job"] {
                if let Some(v) = ev.get(k) && json::check_redaction_tree(v, k).is_ok() { cmd_pairs.push((k.into(), v.clone())); }
            }
        }
    }
    append_log_line(&log_path, tr_path.as_deref(), &JsonValue::Object(cmd_pairs))?;
    seq += 1;

    let repro_arts: Vec<JsonValue> = [&stdout_p, &stderr_p].into_iter().filter(|s| !s.is_empty()).map(|s| JsonValue::String(s.clone())).collect();
    let emit_assert = |seq: &mut u64, name: String, kind: &str, status: &str, obs: String, exp: String, caught: Option<bool>, diff_path: Option<&str>| -> Result<(), String> {
        let mut p = base_line(format_utc_rfc3339_ms(now_utc_ms()), *seq, run_id, script, bead, step, "assert", seed);
        p.extend([
            ("name".into(), JsonValue::String(name)),
            ("kind".into(), JsonValue::String(kind.into())),
            ("status".into(), JsonValue::String(status.into())),
            ("observed".into(), JsonValue::String(obs.clone())),
            ("expected".into(), JsonValue::String(exp.clone())),
        ]);
        if let Some(c) = caught { p.push(("caught".into(), JsonValue::Bool(c))); }
        for (fl, key) in [("--gate", "gate"), ("--ka", "ka"), ("--cx", "cx"), ("--nc", "nc"), ("--mr", "mr")] {
            if let Some(v) = f.get(fl) { p.push((key.into(), JsonValue::String(v.into()))); }
        }
        if let Some(dp) = diff_path { p.extend(fail_meta(dp, obs, exp, cmd_seq, repro_arts.clone())); }
        append_log_line(&log_path, tr_path.as_deref(), &JsonValue::Object(p))?;
        *seq += 1;
        Ok(())
    };

    let refusal_obj = env_opt.as_ref().filter(|e| e.get("ok").and_then(JsonValue::as_bool) == Some(false))
        .and_then(|e| e.get("refusal")).filter(|r| r.as_object().is_some());
    if let Some(rv) = refusal_obj {
        let rcode = rv.get("code").and_then(JsonValue::as_str).unwrap_or("UNKNOWN");
        let rexit = rv.get("exit").and_then(JsonValue::as_i64)
            .or_else(|| env_opt.as_ref().and_then(|e| e.get("exit")).and_then(JsonValue::as_i64))
            .unwrap_or(child_exit as i64);
        let is_expected = exp_ref.map_or(exp_caught.is_some(), |ec| rcode == ec && expected_exits.contains(&child_exit));
        let mut rp = base_line(format_utc_rfc3339_ms(now_utc_ms()), seq, run_id, script, bead, step, "refusal", seed);
        rp.extend([
            ("code".into(), JsonValue::String(rcode.into())),
            ("exit".into(), JsonValue::Number(rexit.to_string())),
            ("message".into(), JsonValue::String(redact_private_path(s_of(rv, "message"), pr))),
            ("pointer".into(), JsonValue::String(s_of(rv, "pointer").into())),
            ("observed".into(), JsonValue::String(redact_private_path(&rv.get("observed").map(compact_disp).unwrap_or_default(), pr))),
            ("required".into(), JsonValue::String(rv.get("required").map(compact_disp).unwrap_or_default())),
            ("unit".into(), JsonValue::String(s_of(rv, "unit").into())),
            ("retry".into(), JsonValue::Bool(rv.get("retry").and_then(JsonValue::as_bool).unwrap_or(false))),
            ("repairs".into(), rv.get("repairs").filter(|v| v.as_array().is_some()).cloned().unwrap_or_else(|| JsonValue::Array(Vec::new()))),
            ("expected".into(), JsonValue::Bool(is_expected)),
        ]);
        append_log_line(&log_path, tr_path.as_deref(), &JsonValue::Object(rp))?;
        seq += 1;

        if let Some(ec) = exp_ref {
            let dp = (!is_expected).then_some(if rcode != ec { "/refusal/code" } else { "/exit" });
            emit_assert(&mut seq, format!("refusal:{ec}"), "refusal", if is_expected { "pass" } else { "fail" },
                format!("{rcode} (exit {child_exit})"), format!("{ec} (exit {expected_exits:?})"), None, dp)?;
        } else if let Some(ctrl) = exp_caught {
            emit_assert(&mut seq, ctrl.into(), "negative-control", "pass", format!("refused:{rcode}"), "caught".into(), Some(true), None)?;
        } else {
            emit_assert(&mut seq, format!("refusal:{label}"), "refusal", "fail", rcode.into(), "ok:true".into(), None, Some("/refusal/code"))?;
        }
    } else if let Some(ec) = exp_ref {
        emit_assert(&mut seq, format!("refusal:{ec}"), "refusal", "fail",
            format!("no-refusal (exit {child_exit})"), format!("{ec} (exit {expected_exits:?})"), None, Some("/ok"))?;
    } else if let Some(ctrl) = exp_caught {
        let ok = child_exit != 0;
        emit_assert(&mut seq, ctrl.into(), "negative-control", if ok { "pass" } else { "fail" },
            format!("exit:{child_exit}"), "non-zero".into(), Some(ok), (!ok).then_some("/exit"))?;
    } else if !expected_exits.contains(&child_exit) {
        let exp_s = expected_exits.iter().map(i32::to_string).collect::<Vec<_>>().join(",");
        emit_assert(&mut seq, format!("exit:{label}"), "exit", "fail", child_exit.to_string(), exp_s, None, Some("/exit"))?;
    }

    if let Some(cnt) = receipt_bad {
        emit_assert(&mut seq, format!("receipt:{label}"), "file", "fail", format!("{cnt} new receipts"), "1 new receipt".into(), None, Some("/receipt"))?;
    }

    Ok(format!(
        "E2E_LAST_EXIT={child_exit}\nE2E_LAST_STDOUT={stdout_p}\nE2E_LAST_STDERR={stderr_p}\n\
         E2E_LAST_CMD_SEQ={cmd_seq}\nE2E_LAST_RECEIPT={receipt_path}\nE2E_LAST_SECRET_WRITTEN={secret_written}\n"
    ))
}

fn is_frob_envelope(v: &JsonValue) -> bool {
    v.as_object().is_some()
        && (s_of(v, "schema") == "frob.envelope/1"
            || (v.get("ok").and_then(JsonValue::as_bool).is_some()
                && (v.get("refusal").is_some() || v.get("evidence").is_some() || v.get("verb").is_some())))
}

fn envelope_is_private(v: &JsonValue) -> bool {
    v.get("private").and_then(JsonValue::as_bool) == Some(true)
        || v.pointer("/evidence/reality").and_then(JsonValue::as_str) == Some("private")
        || v.pointer("/evidence/private").and_then(JsonValue::as_bool) == Some(true)
        || v.pointer("/evidence/mark").and_then(JsonValue::as_str) == Some("private")
}

fn snapshot_dir_names(dir: Option<&Path>) -> BTreeSet<String> {
    let mut set = BTreeSet::new();
    if let Some(d) = dir && let Ok(rd) = fs::read_dir(d) {
        for e in rd.flatten() {
            if e.file_type().is_ok_and(|ft| ft.is_file()) { set.insert(e.file_name().to_string_lossy().into_owned()); }
        }
    }
    set
}

fn replace_bytes(hay: &[u8], needle: &[u8], rep: &[u8]) -> Vec<u8> {
    if needle.is_empty() || hay.len() < needle.len() { return hay.to_vec(); }
    let (mut out, mut i) = (Vec::with_capacity(hay.len()), 0);
    while i + needle.len() <= hay.len() {
        if &hay[i..i + needle.len()] == needle { out.extend_from_slice(rep); i += needle.len(); }
        else { out.push(hay[i]); i += 1; }
    }
    out.extend_from_slice(&hay[i..]);
    out
}

#[cfg(unix)]
fn status_to_code(s: &std::process::ExitStatus) -> i32 {
    use std::os::unix::process::ExitStatusExt;
    s.code().unwrap_or_else(|| s.signal().map_or(1, |sig| 128 + sig))
}
#[cfg(not(unix))]
fn status_to_code(s: &std::process::ExitStatus) -> i32 { s.code().unwrap_or(1) }

#[cfg(unix)]
fn write_secret_file(path: &Path, bytes: &[u8]) -> Result<(), String> {
    use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};
    if let Some(p) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
        fs::create_dir_all(p).map_err(|e| format!("create secret dir: {e}"))?;
    }
    let mut f = OpenOptions::new().write(true).create(true).truncate(true).mode(0o600).open(path)
        .map_err(|e| format!("open secret file {}: {e}", path.display()))?;
    f.write_all(bytes).map_err(|e| format!("write secret file: {e}"))?;
    fs::set_permissions(path, fs::Permissions::from_mode(0o600)).map_err(|e| format!("chmod 0600: {e}"))
}
#[cfg(not(unix))]
fn write_secret_file(path: &Path, bytes: &[u8]) -> Result<(), String> {
    if let Some(p) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
        fs::create_dir_all(p).map_err(|e| format!("create secret dir: {e}"))?;
    }
    fs::write(path, bytes).map_err(|e| format!("write secret file: {e}"))
}

// ---------------------------------------------------------------------------
// Subcommands: get, cmp, transcript, schema
// ---------------------------------------------------------------------------

pub fn read_input(file: Option<&str>) -> Result<String, String> {
    match file {
        None | Some("-") => {
            let mut s = String::new();
            io::stdin().read_to_string(&mut s).map_err(|e| format!("read stdin: {e}"))?;
            Ok(s)
        }
        Some(p) => fs::read_to_string(p).map_err(|e| format!("read {p}: {e}")),
    }
}

pub fn cmd_get(args: &[String]) -> i32 {
    let f = match ParsedFlags::parse(args, &["--file"], &[], &[]) {
        Ok(f) if f.positionals.len() == 1 => f,
        Ok(_) => { eprintln!("usage: frob-e2e get [--file <path>] <json-pointer>"); return 2; }
        Err(e) => { eprintln!("frob-e2e get: {e}"); return 2; }
    };
    let val = match read_input(f.get("--file")).and_then(|t| json::parse_strict(&t).map_err(|e| e.to_string())) {
        Ok(v) => v,
        Err(e) => { eprintln!("frob-e2e get: {e}"); return 2; }
    };
    match val.pointer(&f.positionals[0]) {
        Some(JsonValue::String(s)) => { println!("{s}"); 0 }
        Some(o) => { println!("{}", o.to_json_string()); 0 }
        None => { eprintln!("frob-e2e get: pointer not found: {}", f.positionals[0]); 1 }
    }
}

pub enum CmpOutcome {
    Plain(String),
    Result { ok: bool, observed: String, expected: String, first_diff: Option<(String, String, String)> },
}

pub fn eval_cmp(mode: &str, pos: &[&str]) -> Result<CmpOutcome, String> {
    match mode {
        "eq" if pos.len() == 2 => {
            let ok = pos[0].as_bytes() == pos[1].as_bytes();
            Ok(CmpOutcome::Result { ok, observed: pos[0].into(), expected: pos[1].into(), first_diff: (!ok).then(|| ("/".into(), pos[0].into(), pos[1].into())) })
        }
        "num" if pos.len() == 3 => {
            let ok = compare_op(&Rational::parse_decimal(pos[0])?, pos[1], &Rational::parse_decimal(pos[2])?)?;
            let exp = format!("{} {}", pos[1], pos[2]);
            Ok(CmpOutcome::Result { ok, observed: pos[0].into(), expected: exp.clone(), first_diff: (!ok).then(|| ("/".into(), pos[0].into(), exp)) })
        }
        "close" if pos.len() == 4 => {
            let (obs, exp, tol) = (Rational::parse_decimal(pos[0])?, Rational::parse_decimal(pos[1])?, Rational::parse_decimal(pos[3])?);
            let ok = compare_close(&obs, &exp, pos[2], &tol)?;
            let exp_d = format!("{} ({} tol {})", pos[1], pos[2], pos[3]);
            Ok(CmpOutcome::Result {
                ok, observed: pos[0].into(), expected: exp_d.clone(),
                first_diff: (!ok).then(|| ("/".into(), format!("{} (diff={})", pos[0], obs.abs_diff(&exp).format_exact()), exp_d)),
            })
        }
        "rational" if pos.len() == 2 => {
            let (a, b) = (Rational::parse_rational(pos[0])?, Rational::parse_rational(pos[1])?);
            let ok = a == b;
            Ok(CmpOutcome::Result { ok, observed: a.format_exact(), expected: b.format_exact(), first_diff: (!ok).then(|| ("/".into(), a.format_exact(), b.format_exact())) })
        }
        "digest" if pos.len() == 2 => {
            let (a, b) = (pos[0].trim(), pos[1].trim());
            if a.is_empty() || b.is_empty() { return Err("digest cannot be empty".into()); }
            let ok = a == b;
            Ok(CmpOutcome::Result { ok, observed: a.into(), expected: b.into(), first_diff: (!ok).then(|| ("/".into(), a.into(), b.into())) })
        }
        "sha256" if pos.len() == 1 => Ok(CmpOutcome::Plain(sha256_path(Path::new(pos[0])).map_err(|e| format!("{}: {e}", pos[0]))?)),
        "sha256" if pos.len() == 2 => {
            let d = sha256_path(Path::new(pos[0])).map_err(|e| format!("{}: {e}", pos[0]))?;
            let ok = d == pos[1];
            Ok(CmpOutcome::Result { ok, observed: d.clone(), expected: pos[1].into(), first_diff: (!ok).then(|| ("/".into(), d, pos[1].into())) })
        }
        "repairs" if pos.len() == 2 => {
            let val = json::parse_strict(&read_input(Some(pos[0]))?).map_err(|e| e.to_string())?;
            let arr = val.pointer("/refusal/repairs").and_then(JsonValue::as_array).ok_or("missing /refusal/repairs array")?;
            let got: Vec<String> = arr.iter().map(repair_str).collect();
            let want: Vec<String> = pos[1].split(',').filter(|s| !s.is_empty()).map(str::to_string).collect();
            let ok = got == want;
            let diff = (!ok).then(|| {
                let idx = (0..got.len().max(want.len())).find(|&k| got.get(k) != want.get(k)).unwrap_or(0);
                (format!("/refusal/repairs/{idx}"), got.get(idx).cloned().unwrap_or_else(|| "<missing>".into()), want.get(idx).cloned().unwrap_or_else(|| "<extra>".into()))
            });
            Ok(CmpOutcome::Result { ok, observed: got.join(","), expected: want.join(","), first_diff: diff })
        }
        _ => Err(format!("invalid cmp mode or argument count for '{mode}'")),
    }
}

pub fn cmd_cmp(args: &[String]) -> i32 {
    if args.is_empty() { eprintln!("usage: frob-e2e cmp <eq|num|close|rational|digest|sha256|repairs> [--json] <args...>"); return 2; }
    let (mode, mut as_json, mut pos) = (args[0].as_str(), false, Vec::new());
    for a in &args[1..] { if a == "--json" { as_json = true; } else { pos.push(a.as_str()); } }
    match eval_cmp(mode, &pos) {
        Ok(CmpOutcome::Plain(s)) => { println!("{s}"); 0 }
        Ok(CmpOutcome::Result { ok, observed, expected, first_diff }) => {
            if as_json {
                let fd = first_diff.map_or(JsonValue::Null, |(p, o, e)| obj(vec![
                    ("path", JsonValue::String(p)), ("observed", JsonValue::String(o)), ("expected", JsonValue::String(e)),
                ]));
                println!("{}", obj(vec![("ok", JsonValue::Bool(ok)), ("observed", JsonValue::String(observed)), ("expected", JsonValue::String(expected)), ("first_diff", fd)]).to_json_string());
            } else if !ok && let Some((p, o, e)) = first_diff {
                eprintln!("mismatch at {p}: observed={o} expected={e}");
            }
            if ok { 0 } else { 1 }
        }
        Err(e) => { eprintln!("frob-e2e cmp {mode}: {e}"); 2 }
    }
}

pub fn cmd_transcript(args: &[String]) -> i32 {
    let f = match ParsedFlags::parse(args, &["--out"], &[], &[]) {
        Ok(f) if f.positionals.len() == 1 => f,
        Ok(_) => { eprintln!("usage: frob-e2e transcript <log> [--out <path>]"); return 2; }
        Err(e) => { eprintln!("frob-e2e transcript: {e}"); return 2; }
    };
    let text = match fs::read_to_string(&f.positionals[0]) {
        Ok(s) => s,
        Err(e) => { eprintln!("frob-e2e transcript: {e}"); return 2; }
    };
    let mut rendered = String::new();
    for (idx, line) in text.lines().enumerate() {
        if line.is_empty() { continue; }
        match json::parse_strict(line) {
            Ok(v) => for l in format_transcript_lines(&v) { rendered.push_str(&l); rendered.push('\n'); },
            Err(e) => { eprintln!("frob-e2e transcript: line {}: {e}", idx + 1); return 1; }
        }
    }
    if let Some(op) = f.get("--out") {
        let p = Path::new(op);
        if let Some(par) = p.parent().filter(|x| !x.as_os_str().is_empty()) && let Err(e) = fs::create_dir_all(par) {
            eprintln!("frob-e2e transcript: {e}"); return 2;
        }
        if let Err(e) = fs::write(p, rendered) { eprintln!("frob-e2e transcript: {e}"); return 2; }
    } else {
        print!("{rendered}");
    }
    0
}

/// Exact canonical `schemas/e2e-log.schema.json` for `frob.e2elog/1`.
pub fn schema_json() -> &'static str {
    r#"{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "https://frankenrobotics.local/schemas/frob.e2elog.1.schema.json",
  "title": "FrankenRobotics E2E Log Line (frob.e2elog/1)",
  "description": "One UTF-8 JSON object per NDJSON line, at most 65536 bytes, emitted and validated by frob-e2e (STD-E2E).",
  "type": "object",
  "required": ["schema", "ts", "seq", "run_id", "script", "bead", "step", "event", "seed"],
  "properties": {
    "schema": { "const": "frob.e2elog/1" },
    "ts": { "type": "string", "pattern": "^[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}\\.[0-9]{3}Z$" },
    "seq": { "type": "integer", "minimum": 0 },
    "run_id": { "type": "string", "minLength": 1, "maxLength": 4096 },
    "script": { "type": "string", "minLength": 1, "maxLength": 4096 },
    "bead": { "type": "string", "minLength": 1, "maxLength": 4096 },
    "step": { "type": "string", "maxLength": 4096 },
    "event": { "enum": ["start", "cmd", "assert", "metric", "refusal", "end", "summary"] },
    "seed": { "type": "string", "pattern": "^[0-9]+$" },
    "commit": { "type": "string", "maxLength": 4096 },
    "dirty": { "type": "boolean" },
    "toolchain": { "type": "string", "maxLength": 4096 },
    "frob_version": { "type": "string", "maxLength": 4096 },
    "host": { "type": "object", "required": ["os", "arch", "fingerprint"], "properties": { "os": { "type": "string" }, "arch": { "type": "string" }, "fingerprint": { "type": "string" } } },
    "scale": { "enum": ["full", "small"] },
    "devices": { "type": "array", "items": { "type": "string", "maxLength": 4096 } },
    "plant": { "type": "string", "maxLength": 4096 },
    "argv": { "type": "array", "items": { "type": "string", "maxLength": 4096 } },
    "title": { "type": "string", "maxLength": 4096 },
    "cmdline": { "type": "string", "maxLength": 4096 },
    "cwd": { "type": "string", "maxLength": 4096 },
    "exit": { "type": "integer" },
    "expected_exit": { "type": "array", "items": { "type": "integer" } },
    "wall_ms": { "type": "integer", "minimum": 0 },
    "stdout_path": { "type": "string", "maxLength": 4096 },
    "stderr_path": { "type": "string", "maxLength": 4096 },
    "stdout_sha256": { "type": "string", "pattern": "^[0-9a-f]{64}$" },
    "receipt": { "type": "string", "maxLength": 4096 },
    "envelope": { "type": "object", "required": ["schema", "verb", "ok", "exit"], "properties": { "schema": { "type": "string" }, "verb": { "type": "string" }, "ok": { "type": "boolean" }, "exit": { "type": "integer" } } },
    "evidence": { "type": "object" },
    "cost": { "type": "object" },
    "job": { "type": "object" },
    "name": { "type": "string", "maxLength": 4096 },
    "kind": { "enum": ["eq", "num", "close", "rational", "digest", "json", "file", "exit", "refusal", "negative-control", "determinism", "check"] },
    "status": { "enum": ["pass", "fail", "skip"] },
    "observed": { "type": ["string", "number", "boolean", "null"] },
    "expected": { "type": ["string", "number", "boolean", "null"] },
    "op": { "type": "string", "maxLength": 4096 },
    "tolerance": { "type": ["string", "number"] },
    "unit": { "type": "string", "maxLength": 4096 },
    "gate": { "type": "string", "maxLength": 4096 },
    "ka": { "type": "string", "maxLength": 4096 },
    "cx": { "type": "string", "maxLength": 4096 },
    "nc": { "type": "string", "maxLength": 4096 },
    "mr": { "type": "string", "maxLength": 4096 },
    "caught": { "type": "boolean" },
    "reason": { "type": "string", "maxLength": 4096 },
    "first_diff": { "type": "object", "required": ["path", "observed", "expected"], "properties": { "path": { "type": "string" }, "observed": { "type": ["string", "number", "boolean", "null"] }, "expected": { "type": ["string", "number", "boolean", "null"] } } },
    "repro": { "type": "object", "required": ["cmd_seq", "artifacts"], "properties": { "cmd_seq": { "type": "integer", "minimum": 0 }, "artifacts": { "type": "array", "items": { "type": "string" } } } },
    "value": { "type": ["number", "string"] },
    "threshold": { "type": "object", "required": ["op", "value"], "properties": { "op": { "enum": ["<", "<=", ">", ">=", "==", "!="] }, "value": { "type": ["number", "string"] } } },
    "asserted": { "type": "boolean" },
    "verdict": { "enum": ["met", "not-met"] },
    "interval": { "type": "object", "required": ["lo", "hi"], "properties": { "lo": { "type": ["number", "string"] }, "hi": { "type": ["number", "string"] } } },
    "n": { "type": "integer", "minimum": 0 },
    "code": { "type": "string", "maxLength": 4096 },
    "message": { "type": "string", "maxLength": 4096 },
    "pointer": { "type": "string", "maxLength": 4096 },
    "required": { "type": "string", "maxLength": 4096 },
    "retry": { "type": "boolean" },
    "repairs": { "type": "array" },
    "asserts_passed": { "type": "integer", "minimum": 0 },
    "asserts_failed": { "type": "integer", "minimum": 0 },
    "asserts_skipped": { "type": "integer", "minimum": 0 },
    "controls_caught": { "type": "integer", "minimum": 0 },
    "controls_missed": { "type": "integer", "minimum": 0 },
    "refusals_expected": { "type": "integer", "minimum": 0 },
    "refusals_unexpected": { "type": "integer", "minimum": 0 },
    "gates": { "type": "array", "items": { "type": "object", "required": ["id", "measured_ok", "controls_caught"], "properties": { "id": { "type": "string" }, "measured_ok": { "type": "boolean" }, "controls_caught": { "type": "integer", "minimum": 0 } } } },
    "gates_claimable": { "type": "boolean" },
    "registries": { "enum": ["of-record", "override"] },
    "receipts": { "type": "array", "items": { "type": "string" } },
    "body_sha256": { "type": "string", "pattern": "^[0-9a-f]{64}$" }
  }
}
"#
}
