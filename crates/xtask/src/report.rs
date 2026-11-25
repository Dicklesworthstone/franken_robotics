//! Common `frob.xtask.report/1` report model and canonical writer for `xtask` (bead N34p4-01).
//!
//! All six `xtask` checks emit this report under `--json`. Findings are sorted
//! deterministically by `(rule, subject, path, line, message, repair)` (D0) and
//! serialized exclusively through `frob_core::canon` (N34-18).

use std::collections::BTreeMap;
use std::fmt;
use std::fmt::Write as _;

use frob_core::canon::{CanonError, Canonical, CanonicalBytes, Value, to_canonical};
use frob_core::json::{INT53_MAX, Json, Limits, admit};

pub use crate::crate_map::Finding;

/// Schema identifier carried by every `cargo xtask <check> --json` report.
pub const REPORT_SCHEMA: &str = "frob.xtask.report/1";

/// Typed JSON value for check-specific `facts` payloads in `frob.xtask.report/1`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FactValue {
    Null,
    Bool(bool),
    Int(i64),
    Str(String),
    Array(Vec<FactValue>),
    Object(BTreeMap<String, FactValue>),
}

impl FactValue {
    #[must_use]
    pub fn str(s: impl Into<String>) -> Self {
        Self::Str(s.into())
    }

    #[must_use]
    pub fn int_usize(n: usize) -> Self {
        Self::Int(i64::try_from(n).unwrap_or(INT53_MAX))
    }

    #[must_use]
    pub fn str_array<I, S>(items: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self::Array(items.into_iter().map(|s| Self::Str(s.into())).collect())
    }
}

impl Canonical for FactValue {
    fn write(&self, w: Value<'_>) -> Result<(), CanonError> {
        match self {
            Self::Null => {
                w.null();
                Ok(())
            }
            Self::Bool(b) => {
                w.bool(*b);
                Ok(())
            }
            Self::Int(n) => w.int53(*n),
            Self::Str(s) => {
                w.str(s);
                Ok(())
            }
            Self::Array(items) => w.array(|arr| {
                for item in items {
                    arr.push_term(item)?;
                }
                Ok(())
            }),
            Self::Object(map) => w.object(|obj| {
                for (k, v) in map {
                    obj.member_term(k, v)?;
                }
                Ok(())
            }),
        }
    }
}

impl Canonical for Finding {
    fn write(&self, w: Value<'_>) -> Result<(), CanonError> {
        w.object(|obj| {
            obj.str("rule", self.rule)?;
            obj.str("subject", &self.subject)?;
            obj.str("path", &self.path)?;
            match self.line {
                Some(line) => obj.int53("line", i64::from(line))?,
                None => obj.null("line")?,
            }
            obj.str("message", &self.message)?;
            obj.str("repair", &self.repair)?;
            Ok(())
        })
    }
}

/// Error serializing an `xtask` report.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReportError {
    Int53OutOfRange(i64),
    Canon(CanonError),
}

impl From<CanonError> for ReportError {
    fn from(err: CanonError) -> Self {
        Self::Canon(err)
    }
}

impl fmt::Display for ReportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int53OutOfRange(v) => write!(f, "integer {v} outside int53 bounds"),
            Self::Canon(err) => write!(f, "canonical serialization error: {err}"),
        }
    }
}

impl std::error::Error for ReportError {}

/// One `frob.xtask.report/1` report produced by an `xtask` check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckReport {
    pub schema: String,
    pub check: String,
    pub ok: bool,
    pub root: String,
    pub findings: Vec<Finding>,
    pub facts: BTreeMap<String, FactValue>,
    pub elapsed_ms: u64,
}

impl Canonical for CheckReport {
    fn write(&self, w: Value<'_>) -> Result<(), CanonError> {
        let elapsed = i64::try_from(self.elapsed_ms).map_err(|_| CanonError::Int53OutOfRange {
            value: i64::MAX,
        })?;
        w.object(|obj| {
            obj.str("schema", &self.schema)?;
            obj.str("check", &self.check)?;
            obj.bool("ok", self.ok)?;
            obj.str("root", &self.root)?;
            obj.array("findings", |arr| {
                for finding in &self.findings {
                    arr.push_term(finding)?;
                }
                Ok(())
            })?;
            obj.object("facts", |facts_obj| {
                for (k, v) in &self.facts {
                    facts_obj.member_term(k, v)?;
                }
                Ok(())
            })?;
            obj.int53("elapsed_ms", elapsed)?;
            Ok(())
        })
    }
}

impl CheckReport {
    /// Construct a deterministically sorted report (`ok == findings.is_empty()`).
    #[must_use]
    pub fn new(
        check: impl Into<String>,
        root: impl Into<String>,
        mut findings: Vec<Finding>,
        facts: BTreeMap<String, FactValue>,
        elapsed_ms: u64,
    ) -> Self {
        findings.sort();
        findings.dedup();
        let ok = findings.is_empty();
        Self {
            schema: REPORT_SCHEMA.to_owned(),
            check: check.into(),
            ok,
            root: root.into(),
            findings,
            facts,
            elapsed_ms,
        }
    }

    /// Encode this report via `frob_core::canon::to_canonical`, yielding [`CanonicalBytes`].
    pub fn to_canonical_bytes(&self) -> Result<CanonicalBytes, ReportError> {
        Ok(to_canonical(self)?)
    }

    /// Encode this report as a canonical JSON UTF-8 string via `frob_core::canon`.
    pub fn to_canonical_json(&self) -> Result<String, ReportError> {
        let bytes = self.to_canonical_bytes()?;
        Ok(bytes.as_str().to_owned())
    }

    /// Render a concise human-readable report for non-`--json` invocations.
    #[must_use]
    pub fn format_human(&self) -> String {
        let mut out = String::new();
        let status = if self.ok { "OK" } else { "FAIL" };
        let _ = writeln!(
            out,
            "{}: {} ({} finding{}, {} ms, root: {})",
            self.check,
            status,
            self.findings.len(),
            if self.findings.len() == 1 { "" } else { "s" },
            self.elapsed_ms,
            self.root
        );
        for f in &self.findings {
            match f.line {
                Some(line) => {
                    let _ = writeln!(
                        out,
                        "  [{}] {} ({}:{}): {}",
                        f.rule, f.subject, f.path, line, f.message
                    );
                }
                None => {
                    let _ = writeln!(
                        out,
                        "  [{}] {} ({}): {}",
                        f.rule, f.subject, f.path, f.message
                    );
                }
            }
            let _ = writeln!(out, "    repair: {}", f.repair);
        }
        if let Some(FactValue::Array(capsules)) = self.facts.get("capsules") {
            let _ = writeln!(out, "capsules ({}):", capsules.len());
            if capsules.is_empty() {
                out.push_str("  (none)\n");
            } else {
                for item in capsules {
                    if let FactValue::Object(map) = item {
                        let id = match map.get("id") {
                            Some(FactValue::Str(s)) => s.as_str(),
                            _ => "?",
                        };
                        let ws = match map.get("workspace") {
                            Some(FactValue::Str(s)) => s.as_str(),
                            _ => "?",
                        };
                        let lines = match map.get("lines") {
                            Some(FactValue::Int(n)) => *n,
                            _ => 0,
                        };
                        let blocks = match map.get("unsafe_blocks") {
                            Some(FactValue::Int(n)) => *n,
                            _ => 0,
                        };
                        let api: Vec<&str> = match map.get("safe_api") {
                            Some(FactValue::Array(arr)) => arr
                                .iter()
                                .filter_map(|v| match v {
                                    FactValue::Str(s) => Some(s.as_str()),
                                    _ => None,
                                })
                                .collect(),
                            _ => Vec::new(),
                        };
                        let _ = writeln!(
                            out,
                            "  {id}  workspace={ws}  lines={lines}/299  unsafe_blocks={blocks}  safe_api=[{}]",
                            api.join(", ")
                        );
                    }
                }
            }
            if let Some(FactValue::Object(totals)) = self.facts.get("totals_per_annex")
                && !totals.is_empty()
            {
                out.push_str("totals_per_annex:\n");
                for (ws, val) in totals {
                    if let FactValue::Object(t) = val {
                        let caps = match t.get("capsules") {
                            Some(FactValue::Int(n)) => *n,
                            _ => 0,
                        };
                        let lines = match t.get("lines") {
                            Some(FactValue::Int(n)) => *n,
                            _ => 0,
                        };
                        let blocks = match t.get("unsafe_blocks") {
                            Some(FactValue::Int(n)) => *n,
                            _ => 0,
                        };
                        let _ = writeln!(
                            out,
                            "  {ws}: capsules={caps} lines={lines} unsafe_blocks={blocks}"
                        );
                    }
                }
            }
        }
        if matches!(self.facts.get("schema"), Some(FactValue::Str(s)) if s == "frob.check-stack/1")
        {
            if let Some(FactValue::Str(wrote)) = self.facts.get("wrote_receipt") {
                let _ = writeln!(out, "receipt: {wrote}");
            }
            if let Some(FactValue::Bool(pass)) = self.facts.get("may_record_pass") {
                let _ = writeln!(out, "may_record_pass: {pass}");
            }
            if let Some(FactValue::Array(sibs)) = self.facts.get("siblings") {
                let _ = writeln!(out, "siblings ({}):", sibs.len());
                for item in sibs {
                    if let FactValue::Object(m) = item {
                        let fd = match m.get("fd") { Some(FactValue::Str(s)) => s.as_str(), _ => "?" };
                        let rev = match m.get("rev") { Some(FactValue::Str(s)) => s.as_str(), _ => "?" };
                        let cls = match m.get("class") { Some(FactValue::Str(s)) => s.as_str(), _ => "?" };
                        let reason = match m.get("reason") { Some(FactValue::Str(s)) => Some(s.as_str()), _ => None };
                        let dirty = match m.get("dirty") { Some(FactValue::Bool(b)) => *b, _ => false };
                        match reason {
                            Some(r) => {
                                let _ = writeln!(out, "  {fd}  rev={rev}  class={cls}:{r}  dirty={dirty}");
                            }
                            None => {
                                let _ = writeln!(out, "  {fd}  rev={rev}  class={cls}  dirty={dirty}");
                            }
                        }
                    }
                }
            }
            if let Some(FactValue::Array(dups)) = self.facts.get("duplicates") {
                let list: Vec<&str> = dups
                    .iter()
                    .filter_map(|v| match v { FactValue::Str(s) => Some(s.as_str()), _ => None })
                    .collect();
                if list.is_empty() {
                    out.push_str("duplicates: (none)\n");
                } else {
                    let _ = writeln!(out, "duplicates: [{}]", list.join(", "));
                }
            }
        }
        if let (Some(FactValue::Array(crates)), Some(FactValue::Object(ws_total))) =
            (self.facts.get("crates"), self.facts.get("workspace_total"))
        {
            let _ = writeln!(out, "crates ({}):", crates.len());
            for item in crates {
                if let FactValue::Object(m) = item {
                    let name = match m.get("crate") {
                        Some(FactValue::Str(s)) => s.as_str(),
                        _ => "?",
                    };
                    let lines = match m.get("lines") {
                        Some(FactValue::Int(n)) => *n,
                        _ => 0,
                    };
                    let cap = match m.get("cap_lines") {
                        Some(FactValue::Int(n)) => n.to_string(),
                        _ => "none".to_owned(),
                    };
                    let ratio = match m.get("ratio") {
                        Some(FactValue::Str(s)) => s.as_str(),
                        _ => "n/a",
                    };
                    let status = match m.get("status") {
                        Some(FactValue::Str(s)) => s.as_str(),
                        _ => "?",
                    };
                    let _ = writeln!(
                        out,
                        "  {name}  lines={lines}  cap={cap}  ratio={ratio}  status={status}"
                    );
                }
            }
            let total_lines = match ws_total.get("lines") {
                Some(FactValue::Int(n)) => *n,
                _ => 0,
            };
            let cap_kloc = match ws_total.get("cap_kloc") {
                Some(FactValue::Int(n)) => *n,
                _ => 0,
            };
            let trigger_kloc = match ws_total.get("trigger_kloc") {
                Some(FactValue::Int(n)) => *n,
                _ => 0,
            };
            let _ = writeln!(
                out,
                "workspace_total: lines={total_lines} cap_kloc={cap_kloc} trigger_kloc={trigger_kloc}"
            );
        }
        out
    }
}

/// Validate raw JSON bytes against the `frob.xtask.report/1` schema contract
/// using `frob_core::json::admit` and `Admitted::into_canonical`.
pub fn validate_report_json(raw: &[u8]) -> Result<(), String> {
    let admitted = admit(raw, &Limits::default()).map_err(|e| format!("JSON admission: {e}"))?;
    let obj = admitted
        .value()
        .as_object()
        .ok_or_else(|| "report root must be a JSON object".to_owned())?;

    const EXPECTED_KEYS: &[&str] = &[
        "check",
        "elapsed_ms",
        "facts",
        "findings",
        "ok",
        "root",
        "schema",
    ];
    let observed_keys: Vec<&str> = obj.iter().map(|m| m.name.as_str()).collect();
    if observed_keys != EXPECTED_KEYS {
        return Err(format!(
            "report keys {observed_keys:?} do not match canonical sorted schema keys {EXPECTED_KEYS:?}"
        ));
    }

    let root = admitted.value();
    let schema = root
        .get("schema")
        .and_then(|v| v.as_str())
        .ok_or_else(|| "`schema` must be a string".to_owned())?;
    if schema != REPORT_SCHEMA {
        return Err(format!("`schema` must be `{REPORT_SCHEMA}`, got `{schema}`"));
    }

    let check = root
        .get("check")
        .and_then(|v| v.as_str())
        .ok_or_else(|| "`check` must be a string".to_owned())?;
    if check.is_empty() {
        return Err("`check` must be non-empty".to_owned());
    }

    let ok = root
        .get("ok")
        .and_then(|v| v.as_bool())
        .ok_or_else(|| "`ok` must be a boolean".to_owned())?;

    let root_str = root
        .get("root")
        .and_then(|v| v.as_str())
        .ok_or_else(|| "`root` must be a string".to_owned())?;
    if root_str.is_empty() {
        return Err("`root` must be non-empty".to_owned());
    }

    let elapsed = root
        .get("elapsed_ms")
        .ok_or_else(|| "missing `elapsed_ms`".to_owned())?
        .as_int53()
        .map_err(|e| format!("`elapsed_ms` invalid int53: {e}"))?;
    if elapsed < 0 {
        return Err(format!("`elapsed_ms` must be >= 0, got {elapsed}"));
    }

    if root.get("facts").and_then(|v| v.as_object()).is_none() {
        return Err("`facts` must be a JSON object".to_owned());
    }

    let findings = root
        .get("findings")
        .and_then(|v| v.as_array())
        .ok_or_else(|| "`findings` must be a JSON array".to_owned())?;
    if ok != findings.is_empty() {
        return Err(format!(
            "`ok` ({ok}) disagrees with findings count ({})",
            findings.len()
        ));
    }

    const EXPECTED_FINDING_KEYS: &[&str] =
        &["line", "message", "path", "repair", "rule", "subject"];
    for (idx, item) in findings.iter().enumerate() {
        let members = item
            .as_object()
            .ok_or_else(|| format!("findings[{idx}] must be an object"))?;
        let keys: Vec<&str> = members.iter().map(|m| m.name.as_str()).collect();
        if keys != EXPECTED_FINDING_KEYS {
            return Err(format!(
                "findings[{idx}] keys {keys:?} do not match canonical sorted keys {EXPECTED_FINDING_KEYS:?}"
            ));
        }
        let Some(line_sp) = item.get("line") else {
            return Err(format!("findings[{idx}] missing `line`"));
        };
        match &line_sp.value {
            Json::Null => {}
            Json::Num(_) => {
                let n = line_sp
                    .as_int53()
                    .map_err(|e| format!("findings[{idx}].line invalid: {e}"))?;
                if n < 1 {
                    return Err(format!("findings[{idx}].line must be >= 1, got {n}"));
                }
            }
            _ => return Err(format!("findings[{idx}].line must be integer or null")),
        }
        for req_non_empty in ["rule", "message", "repair"] {
            let s = item
                .get(req_non_empty)
                .and_then(|v| v.as_str())
                .ok_or_else(|| format!("findings[{idx}].{req_non_empty} must be a string"))?;
            if s.is_empty() {
                return Err(format!("findings[{idx}].{req_non_empty} must be non-empty"));
            }
        }
        for req_str in ["subject", "path"] {
            if item.get(req_str).and_then(|v| v.as_str()).is_none() {
                return Err(format!("findings[{idx}].{req_str} must be a string"));
            }
        }
    }

    admitted
        .into_canonical()
        .map_err(|e| format!("report bytes are not canonical (N34-18): {e}"))?;
    Ok(())
}
