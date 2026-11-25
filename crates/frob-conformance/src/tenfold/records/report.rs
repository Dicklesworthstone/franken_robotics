//! Ten-fold gate report record (`frob.tenfold.report/1`, skeleton in `N39a-01`,
//! full descriptive tables and no-bare-ratio checks in `N39a-36`).
//!
//! Carries exact reduced rational ratios (`ratio_exact`) beside their decimal
//! rendering (`ratio_decimal`), optional outward-rounded confidence bounds, and
//! the self-hash field `report_hash` hashed with `report_hash = ""`.
//! Lives inside the scorer boundary (`BOUNDARY_FILES`).

use frob_cert::json::JsonValue;
use frob_cert::rational::Rational;

use crate::tenfold::canon::{
    RecordHeader, RoundingMode, SchemaKind, canonical_bytes, check_allowed_keys,
    parse_exact_rational, parse_json_strict, render_rational_decimal, require_str, stamp_self_hash,
    validate_common_header_and_flags, verify_self_hash,
};
use crate::tenfold::ids::{Hash32, Regime};
use crate::tenfold::refusal::TenfoldRefusal;

pub const ALLOWED_KEYS: &[&str] = &[
    "ci_lower_decimal",
    "ci_lower_exact",
    "ci_upper_decimal",
    "ci_upper_exact",
    "execution_authorized",
    "gate",
    "hardware_safety_signoff",
    "ratio_decimal",
    "ratio_exact",
    "reality",
    "regime",
    "report_hash",
    "schema",
    "sim_to_real_certified",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReportRecord {
    pub header: RecordHeader,
    pub regime: Regime,
    pub ratio_exact: Rational,
    pub ratio_decimal: String,
    pub ci_lower_exact: Option<Rational>,
    pub ci_lower_decimal: Option<String>,
    pub ci_upper_exact: Option<Rational>,
    pub ci_upper_decimal: Option<String>,
    pub report_hash: Hash32,
}

fn decimal_places_of(s: &str, pointer: &str) -> Result<usize, TenfoldRefusal> {
    let body = s.strip_prefix('-').unwrap_or(s);
    if body.is_empty() {
        return Err(TenfoldRefusal::noncanonical_number(
            pointer,
            format!("{s:?}"),
            "decimal rendering string",
            vec!["Provide a non-empty decimal string".to_owned()],
        ));
    }
    match body.split_once('.') {
        None => {
            if !body.bytes().all(|b| b.is_ascii_digit()) {
                return Err(TenfoldRefusal::noncanonical_number(
                    pointer,
                    format!("{s:?}"),
                    "decimal rendering string",
                    vec!["Use ASCII digits with an optional single decimal point".to_owned()],
                ));
            }
            Ok(0)
        }
        Some((w, f)) => {
            if w.is_empty()
                || f.is_empty()
                || !w.bytes().all(|b| b.is_ascii_digit())
                || !f.bytes().all(|b| b.is_ascii_digit())
            {
                return Err(TenfoldRefusal::noncanonical_number(
                    pointer,
                    format!("{s:?}"),
                    "decimal rendering string",
                    vec!["Use ASCII digits on both sides of the decimal point".to_owned()],
                ));
            }
            Ok(f.len())
        }
    }
}

impl ReportRecord {
    fn to_json_value_with_hash(&self, hash_str: String) -> JsonValue {
        let mut entries = vec![
            ("execution_authorized".to_owned(), JsonValue::Bool(false)),
            ("gate".to_owned(), JsonValue::Str(self.header.gate.as_string())),
            ("hardware_safety_signoff".to_owned(), JsonValue::Bool(false)),
            (
                "ratio_decimal".to_owned(),
                JsonValue::Str(self.ratio_decimal.clone()),
            ),
            (
                "ratio_exact".to_owned(),
                JsonValue::Str(self.ratio_exact.to_slash_string()),
            ),
            ("reality".to_owned(), JsonValue::Str("no-claim".to_owned())),
            (
                "regime".to_owned(),
                JsonValue::Str(self.regime.as_str().to_owned()),
            ),
            ("report_hash".to_owned(), JsonValue::Str(hash_str)),
            (
                "schema".to_owned(),
                JsonValue::Str(SchemaKind::Report.as_str().to_owned()),
            ),
            ("sim_to_real_certified".to_owned(), JsonValue::Bool(false)),
        ];
        if let Some(r) = &self.ci_lower_exact {
            entries.push((
                "ci_lower_exact".to_owned(),
                JsonValue::Str(r.to_slash_string()),
            ));
        }
        if let Some(d) = &self.ci_lower_decimal {
            entries.push(("ci_lower_decimal".to_owned(), JsonValue::Str(d.clone())));
        }
        if let Some(r) = &self.ci_upper_exact {
            entries.push((
                "ci_upper_exact".to_owned(),
                JsonValue::Str(r.to_slash_string()),
            ));
        }
        if let Some(d) = &self.ci_upper_decimal {
            entries.push(("ci_upper_decimal".to_owned(), JsonValue::Str(d.clone())));
        }
        JsonValue::Obj(entries)
    }

    pub fn build_stamped(
        header: RecordHeader,
        regime: Regime,
        ratio_exact: Rational,
        point_places: usize,
        ci_bounds: Option<(Rational, Rational, usize)>,
    ) -> Result<Self, TenfoldRefusal> {
        let ratio_decimal =
            render_rational_decimal(&ratio_exact, point_places, RoundingMode::HalfToEven)?;
        let (ci_lower_exact, ci_lower_decimal, ci_upper_exact, ci_upper_decimal) = match ci_bounds {
            None => (None, None, None, None),
            Some((lo, hi, places)) => (
                Some(lo.clone()),
                Some(render_rational_decimal(&lo, places, RoundingMode::OutwardLower)?),
                Some(hi.clone()),
                Some(render_rational_decimal(&hi, places, RoundingMode::OutwardUpper)?),
            ),
        };
        let draft = Self {
            header,
            regime,
            ratio_exact,
            ratio_decimal,
            ci_lower_exact,
            ci_lower_decimal,
            ci_upper_exact,
            ci_upper_decimal,
            report_hash: Hash32([0_u8; 32]),
        };
        let (_, addr) = stamp_self_hash(
            &draft.to_json_value_with_hash(String::new()),
            SchemaKind::Report,
        )?;
        Ok(Self {
            report_hash: addr,
            ..draft
        })
    }

    #[must_use]
    pub fn to_json_value(&self) -> JsonValue {
        self.to_json_value_with_hash(self.report_hash.to_b3_string())
    }

    #[must_use]
    pub fn to_canonical_bytes(&self) -> Vec<u8> {
        canonical_bytes(&self.to_json_value())
    }
}

fn check_optional_bound(
    val: &JsonValue,
    exact_key: &str,
    dec_key: &str,
    mode: RoundingMode,
) -> Result<(Option<Rational>, Option<String>), TenfoldRefusal> {
    match (val.get(exact_key), val.get(dec_key)) {
        (None, None) => Ok((None, None)),
        (Some(_), None) => Err(TenfoldRefusal::missing_field(
            format!("/{dec_key}"),
            format!("decimal rendering paired with `{exact_key}`"),
        )),
        (None, Some(_)) => Err(TenfoldRefusal::missing_field(
            format!("/{exact_key}"),
            format!("exact reduced rational paired with `{dec_key}`"),
        )),
        (Some(ev), Some(dv)) => {
            let e_ptr = format!("/{exact_key}");
            let d_ptr = format!("/{dec_key}");
            let Some(es) = ev.as_str() else {
                return Err(TenfoldRefusal::noncanonical_number(
                    e_ptr,
                    format!("{ev:?}"),
                    "exact rational string",
                    vec![format!("Encode `{exact_key}` as an exact rational string")],
                ));
            };
            let Some(ds) = dv.as_str() else {
                return Err(TenfoldRefusal::noncanonical_number(
                    d_ptr,
                    format!("{dv:?}"),
                    "decimal string",
                    vec![format!("Encode `{dec_key}` as a decimal string")],
                ));
            };
            let r = parse_exact_rational(es, &e_ptr)?;
            let places = decimal_places_of(ds, &d_ptr)?;
            let expected = render_rational_decimal(&r, places, mode)?;
            if ds != expected {
                return Err(TenfoldRefusal::noncanonical_number(
                    d_ptr,
                    format!("{ds:?}"),
                    format!("{expected:?} (outward rounded)"),
                    vec![format!("Render `{dec_key}` with outward rounding to `{expected}`")],
                ));
            }
            Ok((Some(r), Some(ds.to_owned())))
        }
    }
}

pub fn parse_value(val: &JsonValue) -> Result<ReportRecord, TenfoldRefusal> {
    let _ = check_allowed_keys(val, "", ALLOWED_KEYS)?;
    let header = validate_common_header_and_flags(val, SchemaKind::Report)?;
    let regime = Regime::parse(require_str(val, "", "regime")?, "/regime")?;

    let ratio_exact = parse_exact_rational(require_str(val, "", "ratio_exact")?, "/ratio_exact")?;
    let ratio_decimal = require_str(val, "", "ratio_decimal")?.to_owned();
    let places = decimal_places_of(&ratio_decimal, "/ratio_decimal")?;
    let expected_dec = render_rational_decimal(&ratio_exact, places, RoundingMode::HalfToEven)?;
    if ratio_decimal != expected_dec {
        return Err(TenfoldRefusal::noncanonical_number(
            "/ratio_decimal",
            format!("{ratio_decimal:?}"),
            format!("{expected_dec:?} (half-to-even from ratio_exact)"),
            vec![format!(
                "Render ratio_decimal half-to-even from ratio_exact as `{expected_dec}`"
            )],
        ));
    }

    let (ci_lower_exact, ci_lower_decimal) = check_optional_bound(
        val,
        "ci_lower_exact",
        "ci_lower_decimal",
        RoundingMode::OutwardLower,
    )?;
    let (ci_upper_exact, ci_upper_decimal) = check_optional_bound(
        val,
        "ci_upper_exact",
        "ci_upper_decimal",
        RoundingMode::OutwardUpper,
    )?;

    let report_hash = verify_self_hash(val, SchemaKind::Report)?;
    Ok(ReportRecord {
        header,
        regime,
        ratio_exact,
        ratio_decimal,
        ci_lower_exact,
        ci_lower_decimal,
        ci_upper_exact,
        ci_upper_decimal,
        report_hash,
    })
}

pub fn parse_bytes(bytes: &[u8]) -> Result<ReportRecord, TenfoldRefusal> {
    let val = parse_json_strict(bytes)?;
    parse_value(&val)
}
