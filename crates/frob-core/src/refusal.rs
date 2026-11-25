//! Structured refusals, retry policy, ranked repairs, and generated [`RefusalCode`] (BD-005).
//! Determinism D0; prints nothing (L-7); zero `unsafe`.

pub mod code_gen;
pub use code_gen::RefusalCode;

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RetryPolicy {
    Never,
    AfterChange,
    Backoff,
}

impl RetryPolicy {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Never => "never",
            Self::AfterChange => "after-change",
            Self::Backoff => "backoff",
        }
    }
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "never" => Some(Self::Never),
            "after-change" => Some(Self::AfterChange),
            "backoff" => Some(Self::Backoff),
            _ => None,
        }
    }
}
impl fmt::Display for RetryPolicy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Repair {
    pub action: String,
    pub detail: Option<String>,
}

impl Repair {
    #[must_use]
    pub fn new(action: impl Into<String>) -> Self {
        Self {
            action: action.into(),
            detail: None,
        }
    }
    #[must_use]
    pub fn with_detail(action: impl Into<String>, detail: impl Into<String>) -> Self {
        Self {
            action: action.into(),
            detail: Some(detail.into()),
        }
    }
}

/// Device fingerprint block carried on every Spark (`6xx`) refusal (`N21-REF`).
///
/// `digest` may be `None` (`null` on wire) only for `FROB-REF-606`
/// (fingerprint missing or changed since admission).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RefusalFingerprint {
    pub device: String,
    pub digest: Option<String>,
}

impl RefusalFingerprint {
    #[must_use]
    pub fn new(device: impl Into<String>, digest: impl Into<String>) -> Self {
        Self {
            device: device.into(),
            digest: Some(digest.into()),
        }
    }

    /// Fingerprint block with `digest: null`; valid only for `FROB-REF-606`.
    #[must_use]
    pub fn missing(device: impl Into<String>) -> Self {
        Self {
            device: device.into(),
            digest: None,
        }
    }
}

/// Structured refusal (`detail` is omitted when `None`, never serialized as `null`;
/// `fingerprint` is required on every `6xx` refusal and omitted on all other families).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Refusal {
    pub code: RefusalCode,
    pub message: String,
    pub pointer: String,
    pub observed: String,
    pub required: String,
    pub unit: String,
    pub detail: Option<String>,
    pub fingerprint: Option<RefusalFingerprint>,
    pub affected_claims: Vec<String>,
    pub retry: RetryPolicy,
    pub repairs: Vec<Repair>,
}

impl Refusal {
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        code: RefusalCode,
        message: impl Into<String>,
        pointer: impl Into<String>,
        observed: impl Into<String>,
        required: impl Into<String>,
        unit: impl Into<String>,
        retry: RetryPolicy,
        repairs: Vec<Repair>,
    ) -> Self {
        Self {
            code,
            message: message.into(),
            pointer: pointer.into(),
            observed: observed.into(),
            required: required.into(),
            unit: unit.into(),
            detail: None,
            fingerprint: None,
            affected_claims: Vec::new(),
            retry,
            repairs,
        }
    }

    #[must_use]
    pub fn with_detail(mut self, detail: impl Into<String>) -> Self {
        self.detail = Some(detail.into());
        self
    }
    #[must_use]
    pub fn with_fingerprint(mut self, fingerprint: RefusalFingerprint) -> Self {
        self.fingerprint = Some(fingerprint);
        self
    }
    #[must_use]
    pub fn with_affected_claims(mut self, claims: Vec<String>) -> Self {
        self.affected_claims = claims;
        self
    }

    /// Enforce family-specific payload invariants (`N21-REF`: `6xx` requires
    /// `fingerprint { device, digest | null }`, with `null` digest allowed only for `606`).
    pub fn validate_family_contract(&self) -> Result<(), RefusalParseError> {
        let has_ctrl = |s: &str| s.chars().any(|c| (c as u32) < 0x20 || c == '\u{7f}');
        if self.code.family() == 6 {
            let Some(fp) = &self.fingerprint else {
                return Err(RefusalParseError::MissingField("fingerprint"));
            };
            if fp.device.is_empty() || has_ctrl(&fp.device) {
                return Err(RefusalParseError::InvalidField {
                    field: "fingerprint.device",
                    reason: "must be a non-empty string without control characters".into(),
                });
            }
            match &fp.digest {
                Some(d) if d.is_empty() || has_ctrl(d) => {
                    return Err(RefusalParseError::InvalidField {
                        field: "fingerprint.digest",
                        reason: "must be a non-empty string without control characters".into(),
                    });
                }
                None if self.code != RefusalCode::FrobRef606 => {
                    return Err(RefusalParseError::InvalidField {
                        field: "fingerprint.digest",
                        reason: "null digest is allowed only for FROB-REF-606".into(),
                    });
                }
                _ => {}
            }
        } else if self.fingerprint.is_some() {
            return Err(RefusalParseError::InvalidField {
                field: "fingerprint",
                reason: "fingerprint block is only valid on 6xx refusals".into(),
            });
        }
        Ok(())
    }

    /// Canonical sorted-key JSON; omits `detail` when `None` (never writes `null`).
    #[must_use]
    pub fn to_json(&self) -> String {
        let mut out = String::from("{\"affected_claims\":[");
        for (i, c) in self.affected_claims.iter().enumerate() {
            if i > 0 {
                out.push(',');
            }
            mini_json::write_str(c, &mut out);
        }
        out.push_str("],\"code\":");
        mini_json::write_str(self.code.as_str(), &mut out);
        if let Some(d) = &self.detail {
            out.push_str(",\"detail\":");
            mini_json::write_str(d, &mut out);
        }
        if let Some(fp) = &self.fingerprint {
            out.push_str(",\"fingerprint\":{\"device\":");
            mini_json::write_str(&fp.device, &mut out);
            out.push_str(",\"digest\":");
            match &fp.digest {
                Some(d) => mini_json::write_str(d, &mut out),
                None => out.push_str("null"),
            }
            out.push('}');
        }
        out.push_str(",\"message\":");
        mini_json::write_str(&self.message, &mut out);
        out.push_str(",\"observed\":");
        mini_json::write_str(&self.observed, &mut out);
        out.push_str(",\"pointer\":");
        mini_json::write_str(&self.pointer, &mut out);
        out.push_str(",\"repairs\":[");
        for (i, r) in self.repairs.iter().enumerate() {
            if i > 0 {
                out.push(',');
            }
            out.push_str("{\"action\":");
            mini_json::write_str(&r.action, &mut out);
            if let Some(d) = &r.detail {
                out.push_str(",\"detail\":");
                mini_json::write_str(d, &mut out);
            }
            out.push('}');
        }
        out.push_str("],\"required\":");
        mini_json::write_str(&self.required, &mut out);
        out.push_str(",\"retry\":");
        mini_json::write_str(self.retry.as_str(), &mut out);
        out.push_str(",\"unit\":");
        mini_json::write_str(&self.unit, &mut out);
        out.push('}');
        out
    }

    /// Strict parser rejecting unknown refusal codes, `detail: null`, invalid `6xx`
    /// fingerprint blocks, and unknown/duplicate keys.
    pub fn from_json(input: &str) -> Result<Self, RefusalParseError> {
        let mut map = match mini_json::parse(input).map_err(RefusalParseError::Json)? {
            mini_json::JsonVal::Object(m) => m,
            _ => {
                return Err(RefusalParseError::Json(
                    "refusal must be a JSON object".into(),
                ));
            }
        };
        let code_s = take_str(&mut map, "code")?;
        let code = RefusalCode::parse(&code_s)
            .ok_or_else(|| RefusalParseError::UnknownRefusalCode(code_s))?;
        let (message, pointer, observed, required, unit) = (
            take_str(&mut map, "message")?,
            take_str(&mut map, "pointer")?,
            take_str(&mut map, "observed")?,
            take_str(&mut map, "required")?,
            take_str(&mut map, "unit")?,
        );
        let retry_s = take_str(&mut map, "retry")?;
        let retry =
            RetryPolicy::parse(&retry_s).ok_or_else(|| RefusalParseError::InvalidField {
                field: "retry",
                reason: format!("invalid retry `{retry_s}`"),
            })?;
        let detail = match map.remove("detail") {
            None => None,
            Some(mini_json::JsonVal::Str(s)) => Some(s),
            Some(mini_json::JsonVal::Null) => {
                return Err(RefusalParseError::InvalidField {
                    field: "detail",
                    reason: "must be omitted when absent, never null".into(),
                });
            }
            Some(_) => {
                return Err(RefusalParseError::InvalidField {
                    field: "detail",
                    reason: "expected string".into(),
                });
            }
        };
        let fingerprint = match map.remove("fingerprint") {
            None => None,
            Some(v) => Some(parse_fingerprint(v)?),
        };
        let affected_claims = match map.remove("affected_claims") {
            Some(mini_json::JsonVal::Array(xs)) => xs
                .into_iter()
                .map(|v| match v {
                    mini_json::JsonVal::Str(s) => Ok(s),
                    _ => Err(RefusalParseError::InvalidField {
                        field: "affected_claims",
                        reason: "expected string".into(),
                    }),
                })
                .collect::<Result<Vec<_>, _>>()?,
            Some(_) => {
                return Err(RefusalParseError::InvalidField {
                    field: "affected_claims",
                    reason: "expected array".into(),
                });
            }
            None => return Err(RefusalParseError::MissingField("affected_claims")),
        };
        let repairs = match map.remove("repairs") {
            Some(mini_json::JsonVal::Array(xs)) => xs
                .into_iter()
                .map(parse_repair)
                .collect::<Result<Vec<_>, _>>()?,
            Some(_) => {
                return Err(RefusalParseError::InvalidField {
                    field: "repairs",
                    reason: "expected array".into(),
                });
            }
            None => return Err(RefusalParseError::MissingField("repairs")),
        };
        if let Some((extra, _)) = map.into_iter().next() {
            return Err(RefusalParseError::UnknownField(extra));
        }
        let refusal = Self {
            code,
            message,
            pointer,
            observed,
            required,
            unit,
            detail,
            fingerprint,
            affected_claims,
            retry,
            repairs,
        };
        refusal.validate_family_contract()?;
        Ok(refusal)
    }
}

fn parse_fingerprint(val: mini_json::JsonVal) -> Result<RefusalFingerprint, RefusalParseError> {
    let mut obj = match val {
        mini_json::JsonVal::Object(m) => m,
        _ => {
            return Err(RefusalParseError::InvalidField {
                field: "fingerprint",
                reason: "expected object".into(),
            });
        }
    };
    let device = match obj.remove("device") {
        Some(mini_json::JsonVal::Str(s)) => s,
        Some(_) => {
            return Err(RefusalParseError::InvalidField {
                field: "fingerprint.device",
                reason: "expected string".into(),
            });
        }
        None => return Err(RefusalParseError::MissingField("fingerprint.device")),
    };
    let digest = match obj.remove("digest") {
        Some(mini_json::JsonVal::Str(s)) => Some(s),
        Some(mini_json::JsonVal::Null) => None,
        Some(_) => {
            return Err(RefusalParseError::InvalidField {
                field: "fingerprint.digest",
                reason: "expected string or null".into(),
            });
        }
        None => return Err(RefusalParseError::MissingField("fingerprint.digest")),
    };
    if let Some((extra, _)) = obj.into_iter().next() {
        return Err(RefusalParseError::UnknownField(format!(
            "fingerprint.{extra}"
        )));
    }
    Ok(RefusalFingerprint { device, digest })
}

fn take_str(
    map: &mut BTreeMap<String, mini_json::JsonVal>,
    field: &'static str,
) -> Result<String, RefusalParseError> {
    match map.remove(field) {
        Some(mini_json::JsonVal::Str(s)) => Ok(s),
        Some(_) => Err(RefusalParseError::InvalidField {
            field,
            reason: "expected string".into(),
        }),
        None => Err(RefusalParseError::MissingField(field)),
    }
}

fn parse_repair(val: mini_json::JsonVal) -> Result<Repair, RefusalParseError> {
    let mut obj = match val {
        mini_json::JsonVal::Object(m) => m,
        _ => {
            return Err(RefusalParseError::InvalidField {
                field: "repairs",
                reason: "expected object".into(),
            });
        }
    };
    let action = take_str(&mut obj, "action")?;
    let detail = match obj.remove("detail") {
        None => None,
        Some(mini_json::JsonVal::Str(s)) => Some(s),
        Some(_) => {
            return Err(RefusalParseError::InvalidField {
                field: "repairs[].detail",
                reason: "expected string".into(),
            });
        }
    };
    if let Some((extra, _)) = obj.into_iter().next() {
        return Err(RefusalParseError::UnknownField(format!(
            "repairs[].{extra}"
        )));
    }
    Ok(Repair { action, detail })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RefusalParseError {
    Json(String),
    UnknownRefusalCode(String),
    MissingField(&'static str),
    UnknownField(String),
    InvalidField { field: &'static str, reason: String },
}

impl fmt::Display for RefusalParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Json(m) => write!(f, "refusal JSON error: {m}"),
            Self::UnknownRefusalCode(c) => write!(f, "unknown refusal code `{c}`"),
            Self::MissingField(k) => write!(f, "missing required refusal field `{k}`"),
            Self::UnknownField(k) => write!(f, "unknown refusal field `{k}`"),
            Self::InvalidField { field, reason } => {
                write!(f, "invalid refusal field `{field}`: {reason}")
            }
        }
    }
}
impl std::error::Error for RefusalParseError {}

impl fmt::Display for Refusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} at {}: {}", self.code, self.pointer, self.message)?;
        if let Some(d) = &self.detail {
            write!(f, " [detail={d}]")?;
        }
        Ok(())
    }
}
impl std::error::Error for Refusal {}

pub(crate) mod mini_json {
    use std::collections::BTreeMap;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub(crate) enum JsonVal {
        Null,
        Bool(bool),
        Str(String),
        Array(Vec<JsonVal>),
        Object(BTreeMap<String, JsonVal>),
    }

    pub(crate) fn write_str(s: &str, out: &mut String) {
        const HEX: &[u8; 16] = b"0123456789abcdef";
        out.push('"');
        for c in s.chars() {
            match c {
                '"' => out.push_str("\\\""),
                '\\' => out.push_str("\\\\"),
                c if (c as u32) < 0x20 => {
                    let b = c as u8;
                    out.push_str("\\u00");
                    out.push(HEX[usize::from(b >> 4)] as char);
                    out.push(HEX[usize::from(b & 0x0f)] as char);
                }
                c => out.push(c),
            }
        }
        out.push('"');
    }

    pub(crate) fn parse(input: &str) -> Result<JsonVal, String> {
        let mut p = Parser {
            src: input,
            bytes: input.as_bytes(),
            pos: 0,
            depth: 0,
        };
        p.skip_ws();
        let val = p.parse_val()?;
        p.skip_ws();
        if p.pos != p.bytes.len() {
            return Err(format!("trailing bytes at offset {}", p.pos));
        }
        Ok(val)
    }

    struct Parser<'a> {
        src: &'a str,
        bytes: &'a [u8],
        pos: usize,
        depth: usize,
    }

    impl<'a> Parser<'a> {
        fn skip_ws(&mut self) {
            while self
                .bytes
                .get(self.pos)
                .is_some_and(|b| matches!(b, b' ' | b'\t' | b'\r' | b'\n'))
            {
                self.pos += 1;
            }
        }
        fn peek(&self) -> Option<u8> {
            self.bytes.get(self.pos).copied()
        }
        fn expect_lit(&mut self, lit: &[u8], val: JsonVal) -> Result<JsonVal, String> {
            if self.bytes[self.pos..].starts_with(lit) {
                self.pos += lit.len();
                Ok(val)
            } else {
                Err(format!("invalid literal at {}", self.pos))
            }
        }
        fn parse_val(&mut self) -> Result<JsonVal, String> {
            if self.depth >= 32 {
                return Err("JSON nesting depth exceeded".into());
            }
            self.skip_ws();
            match self.peek() {
                Some(b'{') => self.parse_object(),
                Some(b'[') => self.parse_array(),
                Some(b'"') => self.parse_string().map(JsonVal::Str),
                Some(b't') => self.expect_lit(b"true", JsonVal::Bool(true)),
                Some(b'f') => self.expect_lit(b"false", JsonVal::Bool(false)),
                Some(b'n') => self.expect_lit(b"null", JsonVal::Null),
                Some(b) => Err(format!("unexpected byte 0x{b:02x} at {}", self.pos)),
                None => Err("unexpected EOF".into()),
            }
        }
        fn parse_object(&mut self) -> Result<JsonVal, String> {
            self.pos += 1;
            self.depth += 1;
            let mut map = BTreeMap::new();
            self.skip_ws();
            if self.peek() == Some(b'}') {
                self.pos += 1;
                self.depth -= 1;
                return Ok(JsonVal::Object(map));
            }
            loop {
                self.skip_ws();
                if self.peek() != Some(b'"') {
                    return Err(format!("expected key at {}", self.pos));
                }
                let k = self.parse_string()?;
                self.skip_ws();
                if self.peek() != Some(b':') {
                    return Err(format!("expected `:` at {}", self.pos));
                }
                self.pos += 1;
                let v = self.parse_val()?;
                if map.insert(k.clone(), v).is_some() {
                    return Err(format!("duplicate JSON key `{k}`"));
                }
                self.skip_ws();
                match self.peek() {
                    Some(b',') => self.pos += 1,
                    Some(b'}') => {
                        self.pos += 1;
                        self.depth -= 1;
                        return Ok(JsonVal::Object(map));
                    }
                    _ => return Err(format!("expected `,` or `}}` at {}", self.pos)),
                }
            }
        }
        fn parse_array(&mut self) -> Result<JsonVal, String> {
            self.pos += 1;
            self.depth += 1;
            let mut xs = Vec::new();
            self.skip_ws();
            if self.peek() == Some(b']') {
                self.pos += 1;
                self.depth -= 1;
                return Ok(JsonVal::Array(xs));
            }
            loop {
                xs.push(self.parse_val()?);
                self.skip_ws();
                match self.peek() {
                    Some(b',') => self.pos += 1,
                    Some(b']') => {
                        self.pos += 1;
                        self.depth -= 1;
                        return Ok(JsonVal::Array(xs));
                    }
                    _ => return Err(format!("expected `,` or `]` at {}", self.pos)),
                }
            }
        }
        fn parse_string(&mut self) -> Result<String, String> {
            self.pos += 1;
            let mut out = String::new();
            while let Some(&b) = self.bytes.get(self.pos) {
                if b == b'"' {
                    self.pos += 1;
                    return Ok(out);
                }
                if b < 0x20 {
                    return Err(format!("unescaped control char at {}", self.pos));
                }
                if b == b'\\' {
                    self.pos += 1;
                    let esc = *self
                        .bytes
                        .get(self.pos)
                        .ok_or_else(|| "unterminated escape".to_owned())?;
                    self.pos += 1;
                    match esc {
                        b'"' => out.push('"'),
                        b'\\' => out.push('\\'),
                        b'/' => out.push('/'),
                        b'b' => out.push('\u{0008}'),
                        b'f' => out.push('\u{000C}'),
                        b'n' => out.push('\n'),
                        b'r' => out.push('\r'),
                        b't' => out.push('\t'),
                        b'u' => {
                            let c = self.parse_u16()?;
                            let ch = if (0xD800..=0xDBFF).contains(&c) {
                                if self.bytes.get(self.pos..self.pos + 2) != Some(b"\\u") {
                                    return Err("unpaired high surrogate".into());
                                }
                                self.pos += 2;
                                let lo = self.parse_u16()?;
                                if !(0xDC00..=0xDFFF).contains(&lo) {
                                    return Err("invalid low surrogate".into());
                                }
                                char::from_u32(
                                    0x10000
                                        + ((u32::from(c) - 0xD800) << 10)
                                        + (u32::from(lo) - 0xDC00),
                                )
                                .ok_or("invalid scalar")?
                            } else {
                                char::from_u32(u32::from(c)).ok_or("unpaired surrogate")?
                            };
                            out.push(ch);
                        }
                        _ => return Err(format!("invalid escape `\\{}`", esc as char)),
                    }
                } else {
                    let run_start = self.pos;
                    while let Some(&rb) = self.bytes.get(self.pos) {
                        if rb == b'"' || rb == b'\\' || rb < 0x20 {
                            break;
                        }
                        self.pos += 1;
                    }
                    out.push_str(&self.src[run_start..self.pos]);
                }
            }
            Err("unterminated JSON string".into())
        }
        fn parse_u16(&mut self) -> Result<u16, String> {
            let sl = self
                .bytes
                .get(self.pos..self.pos + 4)
                .ok_or("truncated \\uXXXX")?;
            if !sl.iter().all(u8::is_ascii_hexdigit) {
                let raw = std::str::from_utf8(sl).unwrap_or("<non-utf8>");
                return Err(format!("bad hex `{raw}`"));
            }
            let hex = std::str::from_utf8(sl).map_err(|_| "invalid \\uXXXX")?;
            let v = u16::from_str_radix(hex, 16).map_err(|_| format!("bad hex `{hex}`"))?;
            self.pos += 4;
            Ok(v)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn refusal_detail_is_optional() {
        let base = Refusal::new(
            RefusalCode::FrobRef881,
            "quantifier prefix mismatch",
            "/scope/quantifiers",
            "∀world ∃policy",
            "∃policy ∀world",
            "",
            RetryPolicy::AfterChange,
            vec![
                Repair::new("re-run with matching quantifier prefix"),
                Repair::with_detail("check mission query", "see §25.24"),
            ],
        )
        .with_affected_claims(vec!["clm:demo".to_owned()]);

        let no_detail = base.to_json();
        assert!(
            matches!(
                mini_json::parse(&no_detail),
                Ok(mini_json::JsonVal::Object(ref top)) if !top.contains_key("detail")
            ),
            "absent top-level detail must be omitted: {no_detail}"
        );
        let no_detail_plain = Refusal::new(
            RefusalCode::FrobRef881,
            "quantifier prefix mismatch",
            "/scope/quantifiers",
            "∀world ∃policy",
            "∃policy ∀world",
            "",
            RetryPolicy::AfterChange,
            vec![Repair::new("re-run with matching quantifier prefix")],
        )
        .to_json();
        assert!(
            !no_detail_plain.contains("\"detail\""),
            "absent detail must be omitted everywhere: {no_detail_plain}"
        );
        assert_eq!(Refusal::from_json(&no_detail).unwrap(), base);

        let with_detail = base.clone().with_detail("registration-after-outcome");
        let with_json = with_detail.to_json();
        assert!(with_json.contains("\"detail\":\"registration-after-outcome\""));
        assert_eq!(Refusal::from_json(&with_json).unwrap(), with_detail);

        let null_json = with_json.replace(
            "\"detail\":\"registration-after-outcome\"",
            "\"detail\":null",
        );
        assert!(matches!(
            Refusal::from_json(&null_json),
            Err(RefusalParseError::InvalidField {
                field: "detail",
                ..
            })
        ));

        let unknown_json = no_detail.replace("FROB-REF-881", "FROB-REF-999");
        assert!(
            matches!(Refusal::from_json(&unknown_json), Err(RefusalParseError::UnknownRefusalCode(ref c)) if c == "FROB-REF-999")
        );
    }

    #[test]
    fn mini_json_rejects_signed_unicode_escape_and_preserves_utf8() {
        let bad_auth = r#"{"execution_author\u+069zed":false,"hardware_safety_signoff":false,"sim_to_real_certified":false}"#;
        assert_eq!(
            crate::auth::NoAuthorization::parse_json(bad_auth)
                .unwrap_err()
                .code,
            RefusalCode::FrobRef816
        );

        let base = Refusal::new(
            RefusalCode::FrobRef802,
            "alpha ∀β😀",
            "/p",
            "obs",
            "req",
            "",
            RetryPolicy::AfterChange,
            vec![Repair::new("fix")],
        );
        let j = base.to_json();
        assert_eq!(Refusal::from_json(&j).unwrap(), base);
        let bad_esc = j.replace("obs", r#"\u+041"#);
        assert!(matches!(
            Refusal::from_json(&bad_esc),
            Err(RefusalParseError::Json(_))
        ));
    }
}
