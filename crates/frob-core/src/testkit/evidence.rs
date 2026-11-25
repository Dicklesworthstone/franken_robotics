//! Unit-test evidence stream (`frob.testevidence/1`, STD-TEST §7).

use std::env;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

pub const EVIDENCE_SCHEMA: &str = "frob.testevidence/1";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvidenceKind { Ka, Cx, Nc, Mr, Inv, Po, Det, Law, Gate }
impl EvidenceKind {
    #[must_use] pub const fn as_str(self) -> &'static str {
        match self { Self::Ka => "ka", Self::Cx => "cx", Self::Nc => "nc", Self::Mr => "mr", Self::Inv => "inv", Self::Po => "po", Self::Det => "det", Self::Law => "law", Self::Gate => "gate" }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvidenceStatus { Pass, Fail, Obligation, Caught, Missed, Stale, Skip, NotEligible, NoClaim }
impl EvidenceStatus {
    #[must_use] pub const fn as_str(self) -> &'static str {
        match self { Self::Pass => "pass", Self::Fail => "fail", Self::Obligation => "obligation", Self::Caught => "caught", Self::Missed => "missed", Self::Stale => "stale", Self::Skip => "skip", Self::NotEligible => "not-eligible", Self::NoClaim => "no-claim" }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvidenceRecord {
    pub crate_name: String, pub test_name: String, pub kind: EvidenceKind, pub id: String,
    pub status: EvidenceStatus, pub reason: Option<String>, pub observed: Option<String>,
    pub expected: Option<String>, pub tolerance: Option<String>, pub unit: Option<String>,
    pub tier: Option<String>, pub site: Option<String>, pub seed: u64,
    pub determinism: Option<String>, pub synthetic: bool,
}

impl EvidenceRecord {
    #[must_use]
    pub fn new(kind: EvidenceKind, id: impl Into<String>, status: EvidenceStatus) -> Self {
        Self {
            crate_name: env::var("CARGO_PKG_NAME").unwrap_or_else(|_| "frob-core".to_owned()),
            test_name: std::thread::current().name().unwrap_or("unnamed").to_owned(),
            kind, id: id.into(), status, reason: None, observed: None, expected: None,
            tolerance: None, unit: None, tier: None, site: None,
            seed: env::var("FROB_TEST_SEED").ok().and_then(|s| s.parse().ok()).unwrap_or(0),
            determinism: None, synthetic: false,
        }
    }

    #[must_use]
    pub fn to_ndjson_line(&self) -> String {
        let mut o = String::with_capacity(256);
        o.push('{');
        kv(&mut o, "schema", Some(EVIDENCE_SCHEMA), true);
        kv(&mut o, "ts", Some(&utc_rfc3339_ms()), false);
        kv(&mut o, "crate", Some(&self.crate_name), false);
        kv(&mut o, "test", Some(&self.test_name), false);
        kv(&mut o, "kind", Some(self.kind.as_str()), false);
        kv(&mut o, "id", Some(&self.id), false);
        kv(&mut o, "status", Some(self.status.as_str()), false);
        kv(&mut o, "reason", self.reason.as_deref(), false);
        kv(&mut o, "observed", self.observed.as_deref(), false);
        kv(&mut o, "expected", self.expected.as_deref(), false);
        kv(&mut o, "tolerance", self.tolerance.as_deref(), false);
        kv(&mut o, "unit", self.unit.as_deref(), false);
        kv(&mut o, "tier", self.tier.as_deref(), false);
        kv(&mut o, "site", self.site.as_deref(), false);
        o.push_str(",\"seed\":"); o.push_str(&self.seed.to_string());
        kv(&mut o, "determinism", self.determinism.as_deref(), false);
        o.push_str(if self.synthetic { ",\"synthetic\":true}" } else { ",\"synthetic\":false}" });
        o
    }
}

pub fn emit_if_configured(record: &EvidenceRecord) {
    if let Some(dir) = env::var_os("FROB_TEST_EVIDENCE_DIR") && !dir.is_empty() { let _ = emit_to_dir(Path::new(&dir), record); }
}

pub fn emit_to_dir(dir: &Path, record: &EvidenceRecord) -> std::io::Result<PathBuf> {
    fs::create_dir_all(dir)?;
    let safe: String = record.crate_name.chars().map(|c| if c.is_ascii_alphanumeric() || c == '-' || c == '_' { c } else { '_' }).collect();
    let path = dir.join(format!("{safe}.ndjson"));
    let mut f = OpenOptions::new().create(true).append(true).open(&path)?;
    writeln!(f, "{}", record.to_ndjson_line())?;
    Ok(path)
}

fn kv(o: &mut String, k: &str, v: Option<&str>, first: bool) {
    if !first { o.push(','); }
    jstr(o, k); o.push(':');
    match v { Some(s) => jstr(o, s), None => o.push_str("null") }
}

fn jstr(o: &mut String, s: &str) {
    o.push('"');
    for ch in s.chars() {
        match ch {
            '"' => o.push_str("\\\""), '\\' => o.push_str("\\\\"), '\n' => o.push_str("\\n"), '\r' => o.push_str("\\r"), '\t' => o.push_str("\\t"),
            c if (c as u32) < 0x20 => { use std::fmt::Write as _; let _ = write!(o, "\\u{:04x}", c as u32); }
            c => o.push(c),
        }
    }
    o.push('"');
}

fn utc_rfc3339_ms() -> String {
    let dur = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default();
    let (secs, ms) = (dur.as_secs() as i64, dur.subsec_millis());
    let (days, rem) = (secs.div_euclid(86_400), secs.rem_euclid(86_400));
    let (h, m, s) = (rem / 3600, (rem % 3600) / 60, rem % 60);
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = (z - era * 146_097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let (d, mo) = (doy - (153 * mp + 2) / 5 + 1, if mp < 10 { mp + 3 } else { mp - 9 });
    let y = yoe as i64 + era * 400 + i64::from(mo <= 2);
    format!("{y:04}-{mo:02}-{d:02}T{h:02}:{m:02}:{s:02}.{ms:03}Z")
}
