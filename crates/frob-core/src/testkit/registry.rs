//! Single-reader registry loaders, exact `assert_ka`, and `control()` (STD-TEST §5).

use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

use crate::registry::toml::{self, Fields, Spanned, Table, Value};
use crate::testkit::evidence::{EvidenceKind, EvidenceRecord, EvidenceStatus, emit_if_configured, emit_to_dir};
use crate::testkit::gate::CmpOp;

pub const KNOWN_ANSWERS_PATH: &str = "crates/frob-conformance/registry/known_answers.toml";
pub const CONTROLS_PATH: &str = "crates/frob-conformance/registry/controls.toml";
pub const RELATIONS_PATH: &str = "crates/frob-conformance/registry/relations.toml";
pub const COUNTEREXAMPLES_PATH: &str = "crates/frob-conformance/registry/counterexamples.toml";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegistryError { pub file: String, pub row: Option<String>, pub message: String }

impl RegistryError {
    fn new(file: &str, row: Option<&str>, msg: impl Into<String>) -> Self {
        Self { file: file.to_owned(), row: row.map(str::to_owned), message: msg.into() }
    }
}

impl fmt::Display for RegistryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.row { Some(r) => write!(f, "{}: row `{r}`: {}", self.file, self.message), None => write!(f, "{}: {}", self.file, self.message) }
    }
}
impl std::error::Error for RegistryError {}

pub fn root() -> Result<PathBuf, RegistryError> {
    let start = env::var_os("CARGO_MANIFEST_DIR").map(PathBuf::from).or_else(|| env::current_dir().ok())
        .ok_or_else(|| RegistryError::new("workspace", None, "cannot determine starting directory"))?;
    let mut cur = start.as_path();
    loop {
        if cur.join("franken-stack.lock").is_file() { return Ok(cur.to_path_buf()); }
        cur = cur.parent().ok_or_else(|| RegistryError::new(&start.display().to_string(), None, "missing `franken-stack.lock`"))?;
    }
}

#[must_use]
pub fn resolve_path(workspace_root: &Path, logical_or_rel: &str) -> PathBuf {
    match logical_or_rel.trim_start_matches("./") {
        "registries/known_answers.toml" => workspace_root.join(KNOWN_ANSWERS_PATH),
        "registries/negative_controls.toml" => workspace_root.join(CONTROLS_PATH),
        other => workspace_root.join(other),
    }
}

fn load_or_empty<T: Default>(root: &Path, rel: &str, parse: fn(&[u8], &str) -> Result<T, RegistryError>) -> Result<(T, bool), RegistryError> {
    let path = root.join(rel);
    if !path.is_file() { return Ok((T::default(), false)); }
    let bytes = fs::read(&path).map_err(|e| RegistryError::new(rel, None, e.to_string()))?;
    Ok((parse(&bytes, rel)?, true))
}

// ---- Exact arbitrary-precision non-negative integer & rational ----

#[derive(Debug, Clone, PartialEq, Eq)]
struct BigNat(Vec<u32>);

impl BigNat {
    fn zero() -> Self { Self(Vec::new()) }
    fn one() -> Self { Self(vec![1]) }
    fn is_zero(&self) -> bool { self.0.is_empty() }
    fn trim(&mut self) { while self.0.last() == Some(&0) { self.0.pop(); } }
    fn from_u64(v: u64) -> Self {
        let mut s = Vec::new();
        if v != 0 { s.push(v as u32); if (v >> 32) != 0 { s.push((v >> 32) as u32); } }
        Self(s)
    }
    fn mul_add(&mut self, mul: u32, add: u32) {
        let mut c = u64::from(add);
        for l in &mut self.0 { let p = u64::from(*l) * u64::from(mul) + c; *l = p as u32; c = p >> 32; }
        if c != 0 { self.0.push(c as u32); }
        self.trim();
    }
    fn shl(&self, bits: u32) -> Self {
        if self.is_zero() || bits == 0 { return self.clone(); }
        let (ws, bs) = ((bits / 32) as usize, bits % 32);
        let mut out = vec![0u32; ws + self.0.len() + 1];
        let mut c = 0u32;
        for (i, &l) in self.0.iter().enumerate() {
            if bs == 0 { out[ws + i] = l; } else { out[ws + i] = (l << bs) | c; c = l >> (32 - bs); }
        }
        if c != 0 { out[ws + self.0.len()] = c; }
        let mut r = Self(out); r.trim(); r
    }
    fn pow10(e: u32) -> Self { let mut r = Self::one(); for _ in 0..e { r.mul_add(10, 0); } r }
    fn from_digits(s: &str) -> Result<Self, String> {
        if s.is_empty() || !s.bytes().all(|b| b.is_ascii_digit()) { return Err(format!("invalid digits `{s}`")); }
        let mut r = Self::zero();
        for b in s.bytes() { r.mul_add(10, u32::from(b - b'0')); }
        Ok(r)
    }
    fn cmp_val(&self, o: &Self) -> Ordering { self.0.len().cmp(&o.0.len()).then_with(|| self.0.iter().rev().cmp(o.0.iter().rev())) }
    fn add(&self, o: &Self) -> Self {
        let n = self.0.len().max(o.0.len());
        let (mut out, mut c) = (Vec::with_capacity(n + 1), 0u64);
        for i in 0..n {
            let s = u64::from(self.0.get(i).copied().unwrap_or(0)) + u64::from(o.0.get(i).copied().unwrap_or(0)) + c;
            out.push(s as u32); c = s >> 32;
        }
        if c != 0 { out.push(c as u32); }
        let mut r = Self(out); r.trim(); r
    }
    fn sub(&self, o: &Self) -> Self {
        let (mut out, mut b) = (Vec::with_capacity(self.0.len()), 0i64);
        for i in 0..self.0.len() {
            let mut d = i64::from(self.0[i]) - i64::from(o.0.get(i).copied().unwrap_or(0)) - b;
            if d < 0 { d += 1i64 << 32; b = 1; } else { b = 0; }
            out.push(d as u32);
        }
        let mut r = Self(out); r.trim(); r
    }
    fn mul(&self, o: &Self) -> Self {
        if self.is_zero() || o.is_zero() { return Self::zero(); }
        let mut out = vec![0u32; self.0.len() + o.0.len()];
        for (i, &a) in self.0.iter().enumerate() {
            let mut c = 0u64;
            for (j, &b) in o.0.iter().enumerate() {
                let p = u64::from(a) * u64::from(b) + u64::from(out[i + j]) + c;
                out[i + j] = p as u32; c = p >> 32;
            }
            out[i + o.0.len()] = c as u32;
        }
        let mut r = Self(out); r.trim(); r
    }
}

#[derive(Debug, Clone)]
struct BigRat { neg: bool, num: BigNat, den: BigNat }

impl BigRat {
    fn from_i64(v: i64) -> Self { Self { neg: v < 0, num: BigNat::from_u64(v.unsigned_abs()), den: BigNat::one() } }
    fn from_f64_exact(v: f64) -> Result<Self, String> {
        if !v.is_finite() { return Err(format!("non-finite f64 `{v}`")); }
        let bits = v.to_bits();
        let (neg, exp_b, frac) = ((bits >> 63) != 0, ((bits >> 52) & 0x7FF) as i32, bits & ((1u64 << 52) - 1));
        if exp_b == 0 && frac == 0 { return Ok(Self { neg: false, num: BigNat::zero(), den: BigNat::one() }); }
        let (mant, exp2) = if exp_b == 0 { (frac, -1074) } else { ((1u64 << 52) | frac, exp_b - 1075) };
        let tz = mant.trailing_zeros();
        let (m, e) = (mant >> tz, exp2 + tz as i32);
        Ok(if e >= 0 { Self { neg, num: BigNat::from_u64(m).shl(e as u32), den: BigNat::one() } }
           else { Self { neg, num: BigNat::from_u64(m), den: BigNat::one().shl((-e) as u32) } })
    }
    fn from_str_exact(raw: &str) -> Result<Self, String> {
        let s = raw.trim();
        if let Some((a, b)) = s.split_once('/') {
            let (na, nb) = (Self::from_dec(a.trim())?, Self::from_dec(b.trim())?);
            if nb.num.is_zero() { return Err("zero denominator".to_owned()); }
            return Ok(Self { neg: (na.neg ^ nb.neg) && !na.num.is_zero(), num: na.num.mul(&nb.den), den: na.den.mul(&nb.num) });
        }
        Self::from_dec(s)
    }
    fn from_dec(s: &str) -> Result<Self, String> {
        let (neg, rest) = s.strip_prefix('-').map(|r| (true, r)).or_else(|| s.strip_prefix('+').map(|r| (false, r))).unwrap_or((false, s));
        let (mant, exp10) = match rest.find(['e', 'E']) {
            Some(i) => (&rest[..i], rest[i + 1..].parse::<i32>().map_err(|e| e.to_string())?),
            None => (rest, 0),
        };
        let (digits, fl) = match mant.split_once('.') { Some((a, b)) => (format!("{a}{b}"), b.len() as i32), None => (mant.to_owned(), 0) };
        let num = BigNat::from_digits(&digits)?;
        if num.is_zero() { return Ok(Self { neg: false, num, den: BigNat::one() }); }
        let net = exp10 - fl;
        Ok(if net >= 0 { Self { neg, num: num.mul(&BigNat::pow10(net as u32)), den: BigNat::one() } }
           else { Self { neg, num, den: BigNat::pow10((-net) as u32) } })
    }
    fn abs(&self) -> Self { Self { neg: false, num: self.num.clone(), den: self.den.clone() } }
    fn abs_diff(&self, o: &Self) -> Self {
        let (a, b, den) = (self.num.mul(&o.den), o.num.mul(&self.den), self.den.mul(&o.den));
        let num = if self.neg == o.neg {
            match a.cmp_val(&b) { Ordering::Greater => a.sub(&b), Ordering::Less => b.sub(&a), Ordering::Equal => BigNat::zero() }
        } else { a.add(&b) };
        Self { neg: false, num, den }
    }
    fn mul(&self, o: &Self) -> Self {
        let num = self.num.mul(&o.num);
        Self { neg: (self.neg ^ o.neg) && !num.is_zero(), num, den: self.den.mul(&o.den) }
    }
    fn cmp_val(&self, o: &Self) -> Ordering {
        if self.num.is_zero() && o.num.is_zero() { return Ordering::Equal; }
        match (self.neg, o.neg) {
            (true, false) => Ordering::Less, (false, true) => Ordering::Greater,
            (false, false) => self.num.mul(&o.den).cmp_val(&o.num.mul(&self.den)),
            (true, true) => o.num.mul(&self.den).cmp_val(&self.num.mul(&o.den)),
        }
    }
}

pub(crate) fn exact_compare(lhs: &str, op: CmpOp, rhs: &str) -> Result<bool, String> {
    let ord = BigRat::from_str_exact(lhs)?.cmp_val(&BigRat::from_str_exact(rhs)?);
    Ok(match op {
        CmpOp::Lt => ord == Ordering::Less, CmpOp::Le => ord != Ordering::Greater,
        CmpOp::Gt => ord == Ordering::Greater, CmpOp::Ge => ord != Ordering::Less, CmpOp::Eq => ord == Ordering::Equal,
    })
}

// ---- Strict field helper & append-only ID checker ----

struct Rf<'a> { f: Fields, file: &'a str, row: Option<&'a str> }

impl<'a> Rf<'a> {
    fn new(tbl: Table, name: impl Into<String>, file: &'a str, row: Option<&'a str>) -> Self { Self { f: Fields::new(tbl, name), file, row } }
    fn e(&self, err: toml::TomlError) -> RegistryError { RegistryError::new(self.file, self.row, err.to_string()) }
    fn err(&mut self, msg: impl Into<String>) -> RegistryError {
        let e = RegistryError::new(self.file, self.row, msg);
        self.f.fail(e)
    }
    fn check<T>(&mut self, res: Result<T, RegistryError>) -> Result<T, RegistryError> {
        res.map_err(|e| self.f.fail(e))
    }
    fn req_str(&mut self, k: &str) -> Result<String, RegistryError> { self.f.require_str(k).map_err(|e| self.e(e)) }
    fn opt_str(&mut self, k: &str) -> Result<Option<String>, RegistryError> { self.f.opt_str(k).map_err(|e| self.e(e)) }
    fn req_bool(&mut self, k: &str) -> Result<bool, RegistryError> { self.f.require_bool(k).map_err(|e| self.e(e)) }
    fn opt_bool(&mut self, k: &str) -> Result<Option<bool>, RegistryError> { self.f.opt_bool(k).map_err(|e| self.e(e)) }
    fn opt_int(&mut self, k: &str) -> Result<Option<i64>, RegistryError> { self.f.opt_int(k).map_err(|e| self.e(e)) }
    fn req_tbl(&mut self, k: &str) -> Result<Table, RegistryError> { self.f.require_table(k).map_err(|e| self.e(e)) }
    fn opt_tbl(&mut self, k: &str) -> Result<Option<Table>, RegistryError> { self.f.opt_table(k).map_err(|e| self.e(e)) }
    fn opt_arr(&mut self, k: &str) -> Result<Option<Vec<Spanned<Value>>>, RegistryError> { self.f.opt_array(k).map_err(|e| self.e(e)) }
    fn req_strs(&mut self, k: &str) -> Result<Vec<String>, RegistryError> {
        let items = self.f.require_array(k).map_err(|e| self.e(e))?;
        str_list(self.file, self.row, k, items).map_err(|e| self.f.fail(e))
    }
    fn opt_strs(&mut self, k: &str) -> Result<Option<Vec<String>>, RegistryError> {
        match self.opt_arr(k)? { Some(v) => Ok(Some(str_list(self.file, self.row, k, v).map_err(|e| self.f.fail(e))?)), None => Ok(None) }
    }
    fn finish(self) -> Result<(), RegistryError> { let (f, file, row) = (self.f, self.file, self.row); f.finish().map_err(|e| RegistryError::new(file, row, e.to_string())) }
}

fn str_list(file: &str, row: Option<&str>, key: &str, items: Vec<Spanned<Value>>) -> Result<Vec<String>, RegistryError> {
    items.into_iter().map(|sp| match sp.value {
        Value::Str(s) => Ok(s),
        o => Err(RegistryError::new(file, row, format!("`{key}` element expected string, found {}", o.type_name()))),
    }).collect()
}

fn top_array(bytes: &[u8], file: &str, root_name: &str, arr_key: &str) -> Result<Vec<Table>, RegistryError> {
    let root = toml::parse(bytes).map_err(|e| RegistryError::new(file, None, e.to_string()))?;
    let mut top = Rf::new(root, root_name, file, None);
    let _ = top.opt_str("schema")?;
    let items = top.opt_arr(arr_key)?.unwrap_or_default();
    top.finish()?;
    items.into_iter().enumerate().map(|(i, sp)| match sp.value {
        Value::Table(t) => Ok(t),
        o => Err(RegistryError::new(file, Some(&format!("{arr_key}[{i}]")), format!("expected table, found {}", o.type_name()))),
    }).collect()
}

fn check_append_only(file: &str, id: &str, prefix: &str, seen: &mut BTreeSet<String>, last: &mut Option<u64>) -> Result<(), RegistryError> {
    let rest = id.strip_prefix(prefix).filter(|r| !r.is_empty() && r.bytes().all(|b| b.is_ascii_digit()))
        .ok_or_else(|| RegistryError::new(file, Some(id), format!("invalid id `{id}` (expected `{prefix}<digits>`)")))?;
    let num: u64 = rest.parse().map_err(|_| RegistryError::new(file, Some(id), "id number overflow"))?;
    if !seen.insert(id.to_owned()) { return Err(RegistryError::new(file, Some(id), format!("duplicate id `{id}`"))); }
    if let Some(prev) = *last && num <= prev {
        return Err(RegistryError::new(file, Some(id), format!("out-of-order id `{id}` after {prev}")));
    }
    *last = Some(num);
    Ok(())
}

// ============================================================================
// 1. KnownAnswers (`known_answers.toml`, N38a-12 + Review Notes 10 & 11)
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Safe { Higher, Lower }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Truth { pub kind: String, pub value: Value }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tolerance { Exact, Abs(String), Rel(String), PrintedDigits(u32), Gate(String), ToSet }

impl fmt::Display for Tolerance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Exact => f.write_str("exact"), Self::Abs(q) => write!(f, "abs({q})"),
            Self::Rel(q) => write!(f, "rel({q})"), Self::PrintedDigits(n) => write!(f, "printed-digits({n})"),
            Self::Gate(id) => write!(f, "gate({id})"), Self::ToSet => f.write_str("to-set"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlCell { pub text: String, pub nc: Vec<String>, pub removed_check: Option<String> }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Row {
    pub id: String, pub section: String, pub owner: String, pub fixture: String, pub same_as: String,
    pub case: String, pub truth: Truth, pub tolerance: Tolerance, pub tier: String, pub sites: Vec<String>,
    pub control: ControlCell, pub derivation: String, pub label: String, pub due: String,
    pub printed: String, pub slug: String, pub safe: Option<Safe>,
}

#[derive(Debug, Clone, Default)]
pub struct KnownAnswers { rows: Vec<Row> }

impl KnownAnswers {
    pub fn load(root: &Path) -> Result<(Self, bool), RegistryError> { load_or_empty(root, KNOWN_ANSWERS_PATH, Self::parse_bytes) }
    pub fn parse_str(input: &str, file: &str) -> Result<Self, RegistryError> { Self::parse_bytes(input.as_bytes(), file) }
    #[must_use] pub fn all(&self) -> &[Row] { &self.rows }
    #[must_use] pub fn get(&self, key: &str) -> Option<&Row> {
        if key.is_empty() { return None; }
        self.rows.iter().find(|r| r.id == key || (!r.slug.is_empty() && r.slug == key))
    }

    pub fn parse_bytes(bytes: &[u8], file: &str) -> Result<Self, RegistryError> {
        let tables = top_array(bytes, file, "known_answers", "ka")?;
        let (mut rows, mut seen_ids, mut seen_slugs, mut last) = (Vec::with_capacity(tables.len()), BTreeSet::new(), BTreeSet::new(), None);
        for (i, tbl) in tables.into_iter().enumerate() {
            let rl = tbl.get("id").and_then(Value::as_str).filter(|s| !s.is_empty())
                .or_else(|| tbl.get("slug").and_then(Value::as_str).filter(|s| !s.is_empty()))
                .map_or_else(|| format!("ka[{i}]"), str::to_owned);
            let mut rf = Rf::new(tbl, format!("ka[{rl}]"), file, Some(&rl));
            let id = rf.req_str("id")?;
            let slug = rf.opt_str("slug")?.unwrap_or_default();
            if id.is_empty() && slug.is_empty() { return Err(rf.err("empty `id` and empty `slug`")); }
            if !id.is_empty() { rf.check(check_append_only(file, &id, "KA-", &mut seen_ids, &mut last))?; }
            if !slug.is_empty() && !seen_slugs.insert(slug.clone()) { return Err(rf.err(format!("duplicate slug `{slug}`"))); }
            let section = rf.req_str("section")?;
            let owner = rf.opt_str("owner")?.unwrap_or_default();
            let fixture = rf.req_str("fixture")?;
            let same_as = rf.opt_str("same_as")?.unwrap_or_default();
            let case = rf.req_str("case")?;
            let truth_tbl = rf.req_tbl("truth")?;
            let truth = rf.check(parse_truth(file, &rl, truth_tbl))?;
            let tol_tbl = rf.req_tbl("tolerance")?;
            let tolerance = rf.check(parse_tol(file, &rl, tol_tbl))?;
            let tier = rf.req_str("tier")?;
            let sites = rf.req_strs("sites")?;
            let ctrl_tbl = rf.req_tbl("control")?;
            let control = rf.check(parse_ctrl_cell(file, &rl, ctrl_tbl))?;
            let derivation = rf.req_str("derivation")?;
            let label = rf.req_str("label")?;
            let due = rf.opt_str("due")?.unwrap_or_default();
            let printed = rf.opt_str("printed")?.unwrap_or_default();
            let safe = match rf.opt_str("safe")?.as_deref() {
                None => None, Some("higher") => Some(Safe::Higher), Some("lower") => Some(Safe::Lower),
                Some(b) => return Err(rf.err(format!("invalid `safe` `{b}`"))),
            };
            rf.finish()?;
            rows.push(Row { id, section, owner, fixture, same_as, case, truth, tolerance, tier, sites, control, derivation, label, due, printed, slug, safe });
        }
        Ok(Self { rows })
    }
}

fn parse_truth(file: &str, row: &str, tbl: Table) -> Result<Truth, RegistryError> {
    let mut rf = Rf::new(tbl, format!("{row}.truth"), file, Some(row));
    let kind = rf.req_str("kind")?;
    let Some(sp) = rf.f.take("value") else { return Err(rf.err("missing `truth.value`")); };
    let value = sp.value;
    rf.finish()?;
    Ok(Truth { kind, value })
}

fn parse_tol(file: &str, row: &str, tbl: Table) -> Result<Tolerance, RegistryError> {
    let mut rf = Rf::new(tbl, format!("{row}.tolerance"), file, Some(row));
    let kind = rf.req_str("kind")?;
    let num_str = |rf: &mut Rf<'_>, k: &str| -> Result<String, RegistryError> {
        let Some(sp) = rf.f.take(k) else { return Err(rf.err(format!("missing `{k}`"))); };
        let s = match sp.value { Value::Str(s) => s, Value::Int(i) => i.to_string(), o => return Err(rf.err(format!("`{k}` expected string/int, found {}", o.type_name()))) };
        if let Err(e) = BigRat::from_str_exact(&s) { return Err(rf.err(e)); }
        Ok(s)
    };
    let tol = match kind.as_str() {
        "exact" => Tolerance::Exact,
        "to-set" | "to set" => Tolerance::ToSet,
        "abs" => Tolerance::Abs(num_str(&mut rf, "value")?),
        "rel" => Tolerance::Rel(num_str(&mut rf, "value")?),
        "printed-digits" => {
            let n = match rf.opt_int("digits")? {
                Some(v) => v,
                None => match rf.opt_int("value")? {
                    Some(v) => v,
                    None => return Err(rf.err("printed-digits requires integer `digits`/`value`")),
                },
            };
            let d = match u32::try_from(n) { Ok(v) => v, Err(_) => return Err(rf.err("negative printed-digits")) };
            Tolerance::PrintedDigits(d)
        }
        "gate" => {
            let gid = match rf.opt_str("id")? {
                Some(v) => v,
                None => match rf.opt_str("value")? {
                    Some(v) => v,
                    None => return Err(rf.err("gate tolerance requires `id`/`value`")),
                },
            };
            Tolerance::Gate(gid)
        }
        other => return Err(rf.err(format!("unknown tolerance kind `{other}`"))),
    };
    rf.finish()?;
    Ok(tol)
}

fn parse_ctrl_cell(file: &str, row: &str, tbl: Table) -> Result<ControlCell, RegistryError> {
    let mut rf = Rf::new(tbl, format!("{row}.control"), file, Some(row));
    let text = rf.req_str("text")?;
    let nc = rf.opt_strs("nc")?.unwrap_or_default();
    let removed_check = rf.opt_str("removed_check")?;
    rf.finish()?;
    Ok(ControlCell { text, nc, removed_check })
}

#[derive(Debug, Clone)]
pub enum Observed { Int(i64), F64(f64), Exact(String), Bool(bool), Counts(BTreeMap<String, i64>) }
impl From<i64> for Observed { fn from(v: i64) -> Self { Self::Int(v) } }
impl From<f64> for Observed { fn from(v: f64) -> Self { Self::F64(v) } }
impl From<&str> for Observed { fn from(v: &str) -> Self { Self::Exact(v.to_owned()) } }
impl From<String> for Observed { fn from(v: String) -> Self { Self::Exact(v) } }
impl From<bool> for Observed { fn from(v: bool) -> Self { Self::Bool(v) } }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObligationReason { Tolerance, Control }
impl ObligationReason {
    #[must_use] pub const fn as_str(self) -> &'static str { match self { Self::Tolerance => "tolerance", Self::Control => "control" } }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KaStatus { Pass, Fail, Obligation(ObligationReason) }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KaCheckResult { pub id: String, pub site: String, pub status: KaStatus, pub within_tolerance: Option<bool>, pub detail: String }

#[must_use]
pub fn assert_ka(row: &Row, site: &str, observed: impl Into<Observed>) -> KaCheckResult {
    assert_ka_with_evidence(row, site, observed, None)
}

#[must_use]
pub fn assert_ka_with_evidence(row: &Row, site: &str, observed: impl Into<Observed>, ev_dir: Option<&Path>) -> KaCheckResult {
    let obs = observed.into();
    let key = if !row.id.is_empty() { row.id.clone() } else { row.slug.clone() };
    let (status, within, detail) = if matches!(row.tolerance, Tolerance::ToSet) {
        (KaStatus::Obligation(ObligationReason::Tolerance), None, format!("{key}@{site}: obligation (tolerance)"))
    } else {
        let ok = compare_truth(&obs, &row.truth.value, &row.tolerance).unwrap_or(false);
        let no_ctrl = row.control.text.trim().eq_ignore_ascii_case("none stated") || row.control.nc.is_empty();
        if no_ctrl { (KaStatus::Obligation(ObligationReason::Control), Some(ok), format!("{key}@{site}: obligation (control; ok={ok})")) }
        else if ok { (KaStatus::Pass, Some(true), format!("{key}@{site}: pass")) }
        else { (KaStatus::Fail, Some(false), format!("{key}@{site}: fail (outside {})", row.tolerance)) }
    };
    let (st, reason) = match status {
        KaStatus::Pass => (EvidenceStatus::Pass, None),
        KaStatus::Fail => (EvidenceStatus::Fail, None),
        KaStatus::Obligation(r) => (EvidenceStatus::Obligation, Some(r.as_str().to_owned())),
    };
    let mut rec = EvidenceRecord::new(EvidenceKind::Ka, &key, st);
    rec.reason = reason; rec.observed = Some(format!("{obs:?}")); rec.expected = Some(format!("{:?}", row.truth.value));
    rec.tolerance = Some(row.tolerance.to_string()); rec.tier = Some(row.tier.clone()); rec.site = Some(site.to_owned());
    match ev_dir { Some(d) => { let _ = emit_to_dir(d, &rec); } None => emit_if_configured(&rec) }
    KaCheckResult { id: key, site: site.to_owned(), status, within_tolerance: within, detail }
}

fn compare_truth(obs: &Observed, exp: &Value, tol: &Tolerance) -> Result<bool, String> {
    if let (Observed::Bool(a), Value::Bool(b)) = (obs, exp) { return Ok(matches!(tol, Tolerance::Exact) && a == b); }
    if let (Observed::Counts(a), Value::Table(t)) = (obs, exp) {
        return Ok(matches!(tol, Tolerance::Exact) && a.len() == t.len() && a.iter().all(|(k, &v)| t.get(k).and_then(Value::as_int) == Some(v)));
    }
    let or = match obs { Observed::Int(v) => BigRat::from_i64(*v), Observed::F64(v) => BigRat::from_f64_exact(*v)?, Observed::Exact(s) => BigRat::from_str_exact(s)?, _ => return Ok(false) };
    let er = match exp { Value::Int(v) => BigRat::from_i64(*v), Value::Str(s) => BigRat::from_str_exact(s)?, _ => return Ok(false) };
    let diff = or.abs_diff(&er);
    Ok(match tol {
        Tolerance::Exact | Tolerance::Gate(_) => diff.num.is_zero(),
        Tolerance::Abs(q) => diff.cmp_val(&BigRat::from_str_exact(q)?.abs()) != Ordering::Greater,
        Tolerance::Rel(q) => diff.cmp_val(&BigRat::from_str_exact(q)?.abs().mul(&er.abs())) != Ordering::Greater,
        Tolerance::PrintedDigits(n) => diff.cmp_val(&BigRat { neg: false, num: BigNat::one(), den: BigNat::from_u64(2).mul(&BigNat::pow10(*n)) }) != Ordering::Greater,
        Tolerance::ToSet => false,
    })
}

// ============================================================================
// 2. Controls (`controls.toml`, BD-025) & `control()`
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlKind { Mutation, PlantedInput, PlantedDesign, PlantedWorld, Aa }
impl ControlKind {
    fn parse(s: &str, file: &str, row: &str) -> Result<Self, RegistryError> {
        match s {
            "mutation" => Ok(Self::Mutation), "planted-input" => Ok(Self::PlantedInput),
            "planted-design" => Ok(Self::PlantedDesign), "planted-world" => Ok(Self::PlantedWorld),
            "a-a" => Ok(Self::Aa), o => Err(RegistryError::new(file, Some(row), format!("unknown control kind `{o}`"))),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)] pub enum ControlState { Runnable, Obligation }
#[derive(Debug, Clone, PartialEq, Eq)] pub struct LastSeen { pub caught: bool, pub session: String, pub date: String }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Control {
    pub id: String, pub seed: String, pub gate: Vec<String>, pub kind: ControlKind,
    pub target: String, pub defect: String, pub expect: String, pub last_seen: Option<LastSeen>,
    pub feature: Option<String>, pub fixture: Option<String>, pub due: Option<String>,
    pub permanent: bool, pub state: ControlState, pub removed_check: Option<String>,
}

#[derive(Debug, Clone, Default)] pub struct Controls { rows: Vec<Control> }

impl Controls {
    pub fn load(root: &Path) -> Result<(Self, bool), RegistryError> { load_or_empty(root, CONTROLS_PATH, Self::parse_bytes) }
    pub fn parse_str(input: &str, file: &str) -> Result<Self, RegistryError> { Self::parse_bytes(input.as_bytes(), file) }
    #[must_use] pub fn all(&self) -> &[Control] { &self.rows }
    #[must_use] pub fn get(&self, id: &str) -> Option<&Control> { self.rows.iter().find(|c| c.id == id) }
    #[must_use] pub fn for_gate(&self, gate: &str) -> Vec<&Control> { self.rows.iter().filter(|c| c.gate.iter().any(|g| g == gate)).collect() }

    pub fn parse_bytes(bytes: &[u8], file: &str) -> Result<Self, RegistryError> {
        let tables = top_array(bytes, file, "controls", "control")?;
        let (mut rows, mut seen, mut last) = (Vec::with_capacity(tables.len()), BTreeSet::new(), None);
        for (i, tbl) in tables.into_iter().enumerate() {
            let rl = tbl.get("id").and_then(Value::as_str).map_or_else(|| format!("control[{i}]"), str::to_owned);
            let mut rf = Rf::new(tbl, format!("control[{rl}]"), file, Some(&rl));
            let id = rf.req_str("id")?;
            rf.check(check_append_only(file, &id, "NC-", &mut seen, &mut last))?;
            let seed = rf.req_str("seed")?;
            let gate = rf.req_strs("gate")?;
            let kind_s = rf.req_str("kind")?;
            let kind = rf.check(ControlKind::parse(&kind_s, file, &rl))?;
            let (target, defect, expect) = (rf.req_str("target")?, rf.req_str("defect")?, rf.req_str("expect")?);
            let last_seen = match rf.opt_tbl("last_seen")? {
                None => None,
                Some(t) => Some(rf.check(parse_last_seen(file, &rl, t))?),
            };
            let (feature, fixture, due) = (rf.opt_str("feature")?, rf.opt_str("fixture")?, rf.opt_str("due")?);
            let permanent = rf.opt_bool("permanent")?.unwrap_or(false);
            let state = match rf.opt_str("state")?.as_deref() {
                None | Some("runnable") => ControlState::Runnable, Some("obligation") => ControlState::Obligation,
                Some(b) => return Err(rf.err(format!("invalid state `{b}`"))),
            };
            let removed_check = rf.opt_str("removed_check")?;
            rf.finish()?;
            rows.push(Control { id, seed, gate, kind, target, defect, expect, last_seen, feature, fixture, due, permanent, state, removed_check });
        }
        Ok(Self { rows })
    }
}

fn parse_last_seen(file: &str, row: &str, tbl: Table) -> Result<LastSeen, RegistryError> {
    let mut lf = Rf::new(tbl, format!("{row}.last_seen"), file, Some(row));
    let ls = LastSeen { caught: lf.req_bool("caught")?, session: lf.req_str("session")?, date: lf.req_str("date")? };
    lf.finish()?;
    Ok(ls)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)] pub enum ControlOutcome { Caught, Missed, Stale, Obligation }
#[derive(Debug, Clone, PartialEq, Eq)] pub enum ControlObservation { FailedWith(String), DidNotFail, Stale(String) }

pub fn control(record: &Control, check: impl FnOnce() -> ControlOutcome) -> ControlOutcome {
    let out = if record.state == ControlState::Obligation { ControlOutcome::Obligation } else { check() };
    emit_ctrl_ev(record, out, None);
    out
}

pub fn control_expect(record: &Control, check: impl FnOnce() -> ControlObservation) -> ControlOutcome {
    if record.state == ControlState::Obligation { emit_ctrl_ev(record, ControlOutcome::Obligation, None); return ControlOutcome::Obligation; }
    let (out, note) = match check() {
        ControlObservation::Stale(w) => (ControlOutcome::Stale, Some(w)),
        ControlObservation::DidNotFail => (ControlOutcome::Missed, Some("control-did-not-fail".to_owned())),
        ControlObservation::FailedWith(obs) => {
            let exp = record.expect.trim();
            if !obs.trim().is_empty() && (exp.is_empty() || obs.contains(exp)) { (ControlOutcome::Caught, Some(obs)) }
            else { (ControlOutcome::Missed, Some(format!("caught-unexpected: {obs}"))) }
        }
    };
    emit_ctrl_ev(record, out, note);
    out
}

fn emit_ctrl_ev(record: &Control, outcome: ControlOutcome, observed: Option<String>) {
    let st = match outcome {
        ControlOutcome::Caught => EvidenceStatus::Caught, ControlOutcome::Missed => EvidenceStatus::Missed,
        ControlOutcome::Stale => EvidenceStatus::Stale, ControlOutcome::Obligation => EvidenceStatus::Obligation,
    };
    let mut ev = EvidenceRecord::new(EvidenceKind::Nc, &record.id, st);
    ev.observed = observed; ev.expected = Some(record.expect.clone());
    emit_if_configured(&ev);
}

// ============================================================================
// 3. Relations (`relations.toml`, N38a-14)
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompareKind { Bits, Exact, Digest, Bound, Order, LeadingOrder }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelationMutation { pub name: String, pub text: String, pub control: String, pub feature: String }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelationRow {
    pub id: String, pub statement: String, pub source: Vec<String>, pub sites: Vec<String>,
    pub compare: CompareKind, pub bound: String, pub mutations: Vec<RelationMutation>,
    pub entry: String, pub owner: String, pub due: String, pub generated_from: Option<String>,
}

#[derive(Debug, Clone, Default)] pub struct Relations { rows: Vec<RelationRow> }

impl Relations {
    pub fn load(root: &Path) -> Result<(Self, bool), RegistryError> { load_or_empty(root, RELATIONS_PATH, Self::parse_bytes) }
    pub fn parse_str(input: &str, file: &str) -> Result<Self, RegistryError> { Self::parse_bytes(input.as_bytes(), file) }
    #[must_use] pub fn all(&self) -> &[RelationRow] { &self.rows }
    #[must_use] pub fn get(&self, id: &str) -> Option<&RelationRow> { self.rows.iter().find(|r| r.id == id) }

    pub fn parse_bytes(bytes: &[u8], file: &str) -> Result<Self, RegistryError> {
        let tables = top_array(bytes, file, "relations", "relation")?;
        let (mut rows, mut seen, mut last_a, mut last_c) = (Vec::with_capacity(tables.len()), BTreeSet::new(), None, None);
        for (i, tbl) in tables.into_iter().enumerate() {
            let rl = tbl.get("id").and_then(Value::as_str).map_or_else(|| format!("relation[{i}]"), str::to_owned);
            let mut rf = Rf::new(tbl, format!("relation[{rl}]"), file, Some(&rl));
            let id = rf.req_str("id")?;
            if id.starts_with("MR-C") { rf.check(check_append_only(file, &id, "MR-C", &mut seen, &mut last_c))?; }
            else { rf.check(check_append_only(file, &id, "MR-", &mut seen, &mut last_a))?; }
            let statement = rf.req_str("statement")?;
            let (source, sites) = (rf.req_strs("source")?, rf.req_strs("sites")?);
            let compare = match rf.req_str("compare")?.as_str() {
                "bits" => CompareKind::Bits, "exact" => CompareKind::Exact, "digest" => CompareKind::Digest,
                "bound" => CompareKind::Bound, "order" => CompareKind::Order, "leading-order" => CompareKind::LeadingOrder,
                b => return Err(rf.err(format!("unknown compare `{b}`"))),
            };
            let bound = rf.opt_str("bound")?.unwrap_or_default();
            let mutations = match rf.f.take("mutation") {
                Some(Spanned { value: Value::Table(t), .. }) => vec![rf.check(parse_rel_mut(file, &rl, t))?],
                Some(Spanned { value: Value::Array(arr), .. }) => {
                    let parsed = arr.into_iter().map(|s| match s.value {
                        Value::Table(t) => parse_rel_mut(file, &rl, t),
                        o => Err(RegistryError::new(file, Some(&rl), format!("mutation element expected table, found {}", o.type_name()))),
                    }).collect::<Result<Vec<_>, _>>();
                    rf.check(parsed)?
                }
                Some(o) => return Err(rf.err(format!("`mutation` expected table/array, found {}", o.value.type_name()))),
                None => return Err(rf.err("missing required key `mutation`")),
            };
            let entry = rf.req_str("entry")?;
            let owner = rf.opt_str("owner")?.unwrap_or_default();
            let due = rf.opt_str("due")?.unwrap_or_default();
            let generated_from = rf.opt_str("generated_from")?;
            rf.finish()?;
            rows.push(RelationRow { id, statement, source, sites, compare, bound, mutations, entry, owner, due, generated_from });
        }
        Ok(Self { rows })
    }
}

fn parse_rel_mut(file: &str, row: &str, tbl: Table) -> Result<RelationMutation, RegistryError> {
    let mut rf = Rf::new(tbl, format!("{row}.mutation"), file, Some(row));
    let m = RelationMutation { name: rf.req_str("name")?, text: rf.req_str("text")?, control: rf.req_str("control")?, feature: rf.req_str("feature")? };
    rf.finish()?;
    Ok(m)
}

// ============================================================================
// 4. Counterexamples (`counterexamples.toml`, N38b-05 + Review Note 10)
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)] pub enum PlanCase { Words, Numbers }

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Plant {
    pub id: String, pub kind: Option<ControlKind>, pub target: Option<String>, pub defect: Option<String>,
    pub expect: Option<String>, pub gates: Option<Vec<String>>, pub feature: Option<String>,
    pub fixture: Option<String>, pub removed_check: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Counterexample {
    pub id: String, pub shortcut: String, pub case: String, pub numeric: bool, pub plan_case: PlanCase,
    pub case_supplied_by: String, pub rule: String, pub rule_ids: Vec<String>, pub defined_in: String,
    pub same_as: String, pub label: String, pub rerun: Option<bool>, pub rerun_receipt: String,
    pub control_kind: ControlKind, pub plant: Vec<Plant>, pub due: Option<String>, pub slug: Option<String>,
}

#[derive(Debug, Clone, Default)] pub struct Counterexamples { rows: Vec<Counterexample> }

impl Counterexamples {
    pub fn load(root: &Path) -> Result<(Self, bool), RegistryError> { load_or_empty(root, COUNTEREXAMPLES_PATH, Self::parse_bytes) }
    pub fn parse_str(input: &str, file: &str) -> Result<Self, RegistryError> { Self::parse_bytes(input.as_bytes(), file) }
    #[must_use] pub fn all(&self) -> &[Counterexample] { &self.rows }
    #[must_use] pub fn get(&self, id: &str) -> Option<&Counterexample> {
        self.rows.iter().find(|c| c.id == id || c.slug.as_deref() == Some(id))
    }

    pub fn parse_bytes(bytes: &[u8], file: &str) -> Result<Self, RegistryError> {
        let tables = top_array(bytes, file, "counterexamples", "cx")?;
        let (mut rows, mut seen, mut last) = (Vec::with_capacity(tables.len()), BTreeSet::new(), None);
        for (i, tbl) in tables.into_iter().enumerate() {
            let rl = tbl.get("id").and_then(Value::as_str).map_or_else(|| format!("cx[{i}]"), str::to_owned);
            let mut rf = Rf::new(tbl, format!("cx[{rl}]"), file, Some(&rl));
            let id = rf.req_str("id")?;
            rf.check(check_append_only(file, &id, "CX-", &mut seen, &mut last))?;
            let (shortcut, case, numeric) = (rf.req_str("shortcut")?, rf.req_str("case")?, rf.req_bool("numeric")?);
            let plan_case = match rf.req_str("plan_case")?.as_str() {
                "words" => PlanCase::Words, "numbers" => PlanCase::Numbers,
                b => return Err(rf.err(format!("invalid plan_case `{b}`"))),
            };
            let case_supplied_by = rf.opt_str("case_supplied_by")?.unwrap_or_default();
            let (rule, rule_ids, defined_in) = (rf.req_str("rule")?, rf.req_strs("rule_ids")?, rf.req_str("defined_in")?);
            let same_as = rf.opt_str("same_as")?.unwrap_or_default();
            let label = rf.req_str("label")?;
            let rerun = rf.opt_bool("rerun")?;
            if rerun.is_some() && label.starts_with("DESIGN") {
                return Err(rf.err("`rerun` is permitted on FACT entries only"));
            }
            let rerun_receipt = rf.opt_str("rerun_receipt")?.unwrap_or_default();
            let ck_s = rf.req_str("control_kind")?;
            let control_kind = rf.check(ControlKind::parse(&ck_s, file, &rl))?;
            let plant = match rf.f.take("plant") {
                None => Vec::new(),
                Some(Spanned { value: Value::Str(s), .. }) => if s.is_empty() { Vec::new() } else { vec![Plant { id: s, ..Plant::default() }] },
                Some(Spanned { value: Value::Array(arr), .. }) => {
                    let parsed = arr.into_iter().map(|s| match s.value {
                        Value::Table(t) => parse_plant(file, &rl, t),
                        o => Err(RegistryError::new(file, Some(&rl), format!("plant element expected table, found {}", o.type_name()))),
                    }).collect::<Result<Vec<_>, _>>();
                    rf.check(parsed)?
                }
                Some(o) => return Err(rf.err(format!("`plant` expected string/array, found {}", o.value.type_name()))),
            };
            let (due, slug) = (rf.opt_str("due")?, rf.opt_str("slug")?);
            rf.finish()?;
            rows.push(Counterexample { id, shortcut, case, numeric, plan_case, case_supplied_by, rule, rule_ids, defined_in, same_as, label, rerun, rerun_receipt, control_kind, plant, due, slug });
        }
        Ok(Self { rows })
    }
}

fn parse_plant(file: &str, row: &str, tbl: Table) -> Result<Plant, RegistryError> {
    let mut rf = Rf::new(tbl, format!("{row}.plant"), file, Some(row));
    let id = rf.req_str("id")?;
    if id.is_empty() { return Err(rf.err("plant `id` must be non-empty")); }
    let kind = match rf.opt_str("kind")? { Some(s) => Some(rf.check(ControlKind::parse(&s, file, row))?), None => None };
    let p = Plant {
        id, kind, target: rf.opt_str("target")?, defect: rf.opt_str("defect")?, expect: rf.opt_str("expect")?,
        gates: rf.opt_strs("gates")?, feature: rf.opt_str("feature")?, fixture: rf.opt_str("fixture")?,
        removed_check: rf.opt_str("removed_check")?,
    };
    rf.finish()?;
    Ok(p)
}
