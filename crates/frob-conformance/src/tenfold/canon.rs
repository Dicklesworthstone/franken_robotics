//! Canonical bytes, content addresses, exact decimal rendering, and strict
//! record-reading helpers for the ten-fold harness (bead `N39a-01`, §39.1).
//!
//! Lives inside the scorer boundary (`BOUNDARY_FILES`): imports strictly `std`
//! and `frob_cert` (plus sibling boundary modules). Reuses `frob-cert`'s
//! canonical JSON reader/writer, BLAKE3, `BigInt`, and `Rational` unchanged.
//! Zero floats, zero `unsafe`, prints nothing (rule L-7), determinism class D0.

use std::collections::BTreeSet;

use frob_cert::bigint::BigInt;
use frob_cert::blake3;
use frob_cert::json::{
    JsonError, JsonValue, parse_non_canonical, serialize_canonical,
};
use frob_cert::limits::{
    COMPILED_LIMITS, RULE_JSON_DUPLICATE_KEY, RULE_NUM_FLOAT_IN_BUNDLE, RULE_NUM_INT_FORM,
    RULE_NUM_INT_RANGE,
};
use frob_cert::rational::Rational;

use crate::tenfold::ids::{GateId, Hash32, Millis};
use crate::tenfold::records::freeze;
use crate::tenfold::records::ledger;
use crate::tenfold::records::mission;
use crate::tenfold::records::receipt;
use crate::tenfold::records::report;
use crate::tenfold::refusal::TenfoldRefusal;

/// The five closed harness record schema names (`N39a-01`).
pub const SCHEMA_MISSION: &str = "frob.tenfold.mission/1";
pub const SCHEMA_FREEZE: &str = "frob.tenfold.freeze/1";
pub const SCHEMA_LEDGER: &str = "frob.tenfold.ledger/1";
pub const SCHEMA_RECEIPT: &str = "frob.tenfold.receipt/1";
pub const SCHEMA_REPORT: &str = "frob.tenfold.report/1";

/// Closed list of all five accepted harness record schema names.
pub const FIVE_SCHEMAS: [&str; 5] = [
    SCHEMA_MISSION,
    SCHEMA_FREEZE,
    SCHEMA_LEDGER,
    SCHEMA_RECEIPT,
    SCHEMA_REPORT,
];

/// Closed discrimination of the five harness record schemas.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SchemaKind {
    Mission,
    Freeze,
    Ledger,
    Receipt,
    Report,
}

impl SchemaKind {
    pub const ALL: [Self; 5] = [
        Self::Mission,
        Self::Freeze,
        Self::Ledger,
        Self::Receipt,
        Self::Report,
    ];

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Mission => SCHEMA_MISSION,
            Self::Freeze => SCHEMA_FREEZE,
            Self::Ledger => SCHEMA_LEDGER,
            Self::Receipt => SCHEMA_RECEIPT,
            Self::Report => SCHEMA_REPORT,
        }
    }

    /// Top-level self-hash field blanked to `""` before content-addressing, if any.
    #[must_use]
    pub const fn self_hash_field(self) -> Option<&'static str> {
        match self {
            Self::Mission | Self::Ledger => None,
            Self::Freeze => Some("manifest_hash"),
            Self::Receipt => Some("receipt_hash"),
            Self::Report => Some("report_hash"),
        }
    }

    pub fn parse(raw: &str) -> Result<Self, TenfoldRefusal> {
        match raw {
            SCHEMA_MISSION => Ok(Self::Mission),
            SCHEMA_FREEZE => Ok(Self::Freeze),
            SCHEMA_LEDGER => Ok(Self::Ledger),
            SCHEMA_RECEIPT => Ok(Self::Receipt),
            SCHEMA_REPORT => Ok(Self::Report),
            other => Err(TenfoldRefusal::unknown_schema(
                "/schema",
                format!("{other:?}"),
                "one of frob.tenfold.{mission,freeze,ledger,receipt,report}/1",
            )),
        }
    }
}

/// Common header and no-authorization flags shared by all five harness records.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordHeader {
    pub schema: SchemaKind,
    pub gate: GateId,
}

/// Decimal rendering mode for exact rationals (`N39a-01`, §39.1).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoundingMode {
    /// Point estimates: round half-to-even at the printed precision.
    HalfToEven,
    /// Upper confidence bound: round outward (up, towards +∞).
    OutwardUpper,
    /// Lower confidence bound: round outward (down, towards -∞).
    OutwardLower,
}

/// Validated harness record together with its canonical bytes and `b3:` content address.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckedRecord {
    pub schema: SchemaKind,
    pub gate: GateId,
    pub canonical_bytes: Vec<u8>,
    pub address: Hash32,
}

/// Escape one RFC 6901 JSON Pointer reference token.
#[must_use]
pub fn escape_pointer_token(token: &str) -> String {
    let mut out = String::with_capacity(token.len());
    for ch in token.chars() {
        match ch {
            '~' => out.push_str("~0"),
            '/' => out.push_str("~1"),
            c => out.push(c),
        }
    }
    out
}

/// Format a child RFC 6901 JSON Pointer from `parent` and `token`.
#[must_use]
pub fn join_pointer(parent: &str, token: &str) -> String {
    format!("{parent}/{}", escape_pointer_token(token))
}

/// Convert a `frob-cert` JSON path (`$.a.b[0].c` or `$`) into an RFC 6901 JSON Pointer (`/a/b/0/c` or `""`).
#[must_use]
pub fn cert_path_to_pointer(cert_path: &str) -> String {
    let rest = cert_path.strip_prefix('$').unwrap_or(cert_path);
    if rest.is_empty() {
        return String::new();
    }
    let mut out = String::new();
    let bytes = rest.as_bytes();
    let mut i = 0usize;
    while i < bytes.len() {
        if bytes[i] == b'.' {
            i += 1;
            let start = i;
            while i < bytes.len() && bytes[i] != b'.' && bytes[i] != b'[' {
                i += 1;
            }
            if i > start {
                out.push('/');
                out.push_str(&escape_pointer_token(&rest[start..i]));
            }
        } else if bytes[i] == b'[' {
            i += 1;
            let start = i;
            while i < bytes.len() && bytes[i] != b']' {
                i += 1;
            }
            if i > start {
                out.push('/');
                out.push_str(&escape_pointer_token(&rest[start..i]));
            }
            if i < bytes.len() && bytes[i] == b']' {
                i += 1;
            }
        } else {
            let start = i;
            while i < bytes.len() && bytes[i] != b'.' && bytes[i] != b'[' {
                i += 1;
            }
            out.push('/');
            out.push_str(&escape_pointer_token(&rest[start..i]));
        }
    }
    out
}

/// Map a `frob_cert::json::JsonError` into the appropriate `TenfoldRefusal`.
#[must_use]
pub fn map_json_error(err: &JsonError) -> TenfoldRefusal {
    let pointer = cert_path_to_pointer(&err.path);
    match err.rule {
        RULE_JSON_DUPLICATE_KEY => TenfoldRefusal::duplicate_key(pointer, err.message.clone()),
        RULE_NUM_FLOAT_IN_BUNDLE | RULE_NUM_INT_FORM | RULE_NUM_INT_RANGE => {
            TenfoldRefusal::noncanonical_number(
                pointer,
                err.message.clone(),
                "small integer within ±(2^53 - 1) or canonical string quantity",
                vec!["Encode quantities as canonical strings and small counts as integers within ±(2^53 - 1)".to_owned()],
            )
        }
        _ => TenfoldRefusal::noncanonical_number(
            pointer,
            err.message.clone(),
            "valid UTF-8 JSON without BOM, floats, or duplicate keys",
            vec!["Fix the JSON syntax or limit violation at the reported pointer".to_owned()],
        ),
    }
}

/// Parse incoming JSON bytes strictly (rejecting BOM, invalid UTF-8, duplicate
/// keys, floats, `-0`, and out-of-range integers) while accepting any key order
/// or whitespace so the record can be canonicalized.
pub fn parse_json_strict(bytes: &[u8]) -> Result<JsonValue, TenfoldRefusal> {
    parse_non_canonical(bytes, &COMPILED_LIMITS).map_err(|e| map_json_error(&e))
}

/// Serialize a `JsonValue` into its unique canonical UTF-8 JSON bytes
/// (bytewise sorted keys, zero insignificant whitespace, minimal string escapes).
#[must_use]
pub fn canonical_bytes(value: &JsonValue) -> Vec<u8> {
    serialize_canonical(value).into_bytes()
}

/// Parse and emit unique canonical JSON bytes (`canon(canon(x)) == canon(x)`).
pub fn canonicalize_json_bytes(bytes: &[u8]) -> Result<Vec<u8>, TenfoldRefusal> {
    let val = parse_json_strict(bytes)?;
    Ok(canonical_bytes(&val))
}

/// Return a clone of object `value` with top-level `field` set to `""`.
fn blank_top_level_field(value: &JsonValue, field: &str) -> Result<JsonValue, TenfoldRefusal> {
    let Some(entries) = value.as_obj() else {
        return Err(TenfoldRefusal::noncanonical_number(
            "",
            "non-object JSON root",
            "JSON object",
            vec!["Harness records must have a JSON object root".to_owned()],
        ));
    };
    let mut out = Vec::with_capacity(entries.len());
    let mut found = false;
    for (k, v) in entries {
        if k == field {
            found = true;
            out.push((k.clone(), JsonValue::Str(String::new())));
        } else {
            out.push((k.clone(), v.clone()));
        }
    }
    if !found {
        return Err(TenfoldRefusal::missing_field(
            join_pointer("", field),
            format!("required self-hash field `{field}`"),
        ));
    }
    Ok(JsonValue::Obj(out))
}

/// Compute the `b3:` BLAKE3 content address of `value`, blanking the schema's
/// self-hash field (`manifest_hash`, `receipt_hash`, or `report_hash`) to `""`
/// before canonicalizing and hashing.
pub fn content_address(value: &JsonValue, schema: SchemaKind) -> Result<Hash32, TenfoldRefusal> {
    let bytes = match schema.self_hash_field() {
        Some(field) => {
            let blanked = blank_top_level_field(value, field)?;
            canonical_bytes(&blanked)
        }
        None => canonical_bytes(value),
    };
    Ok(Hash32::from_bytes(blake3::hash(&bytes)))
}

/// Stamp the computed self-hash (`manifest_hash`, `receipt_hash`, or `report_hash`)
/// into `value` and return `(stamped_value, address)`.
pub fn stamp_self_hash(
    value: &JsonValue,
    schema: SchemaKind,
) -> Result<(JsonValue, Hash32), TenfoldRefusal> {
    let address = content_address(value, schema)?;
    let Some(field) = schema.self_hash_field() else {
        return Ok((value.clone(), address));
    };
    let Some(entries) = value.as_obj() else {
        return Err(TenfoldRefusal::noncanonical_number(
            "",
            "non-object JSON root",
            "JSON object",
            vec!["Harness records must have a JSON object root".to_owned()],
        ));
    };
    let mut out = Vec::with_capacity(entries.len());
    for (k, v) in entries {
        if k == field {
            out.push((k.clone(), JsonValue::Str(address.to_b3_string())));
        } else {
            out.push((k.clone(), v.clone()));
        }
    }
    Ok((JsonValue::Obj(out), address))
}

/// Verify that the self-hash field of `value` equals the recomputed blanked content address.
pub fn verify_self_hash(value: &JsonValue, schema: SchemaKind) -> Result<Hash32, TenfoldRefusal> {
    let computed = content_address(value, schema)?;
    let Some(field) = schema.self_hash_field() else {
        return Ok(computed);
    };
    let ptr = join_pointer("", field);
    let Some(raw) = value.get(field) else {
        return Err(TenfoldRefusal::missing_field(
            ptr,
            format!("required self-hash field `{field}`"),
        ));
    };
    let Some(stored_str) = raw.as_str() else {
        return Err(TenfoldRefusal::hash_mismatch(
            ptr,
            format!("{raw:?}"),
            computed.to_b3_string(),
            vec!["Encode the self-hash field as a `b3:<64 hex>` string computed with the field blanked to \"\"".to_owned()],
        ));
    };
    let stored = Hash32::parse_b3(stored_str, &ptr)?;
    if stored != computed {
        return Err(TenfoldRefusal::hash_mismatch(
            ptr,
            stored.to_b3_string(),
            computed.to_b3_string(),
            vec![format!(
                "Recompute `{field}` as BLAKE3 over canonical JSON with `{field}` set to \"\""
            )],
        ));
    }
    Ok(computed)
}

/// Strictly require that `value` is a JSON object whose keys are all in `allowed`.
pub fn check_allowed_keys<'a>(
    value: &'a JsonValue,
    parent_pointer: &str,
    allowed: &[&str],
) -> Result<&'a [(String, JsonValue)], TenfoldRefusal> {
    let Some(entries) = value.as_obj() else {
        return Err(TenfoldRefusal::noncanonical_number(
            parent_pointer,
            "non-object value",
            "JSON object",
            vec!["Provide a JSON object at this pointer".to_owned()],
        ));
    };
    let allowed_set: BTreeSet<&str> = allowed.iter().copied().collect();
    for (k, _) in entries {
        if !allowed_set.contains(k.as_str()) {
            return Err(TenfoldRefusal::unknown_field(
                join_pointer(parent_pointer, k),
                format!("{k:?}"),
                format!("key in {:?}", allowed),
            ));
        }
    }
    Ok(entries)
}

/// Require a field on `obj`, refusing absence with `record-missing-field`.
pub fn require_field<'a>(
    obj: &'a JsonValue,
    parent_pointer: &str,
    key: &str,
) -> Result<&'a JsonValue, TenfoldRefusal> {
    obj.get(key).ok_or_else(|| {
        TenfoldRefusal::missing_field(
            join_pointer(parent_pointer, key),
            format!("required field `{key}`"),
        )
    })
}

/// Require a string field on `obj`.
pub fn require_str<'a>(
    obj: &'a JsonValue,
    parent_pointer: &str,
    key: &str,
) -> Result<&'a str, TenfoldRefusal> {
    let val = require_field(obj, parent_pointer, key)?;
    val.as_str().ok_or_else(|| {
        TenfoldRefusal::noncanonical_number(
            join_pointer(parent_pointer, key),
            format!("{val:?}"),
            "JSON string",
            vec![format!("Encode `{key}` as a JSON string")],
        )
    })
}

/// Require a canonical seconds duration string field and parse to exact [`Millis`].
pub fn require_duration(
    obj: &JsonValue,
    parent_pointer: &str,
    key: &str,
) -> Result<Millis, TenfoldRefusal> {
    let ptr = join_pointer(parent_pointer, key);
    let val = require_field(obj, parent_pointer, key)?;
    let Some(s) = val.as_str() else {
        return Err(TenfoldRefusal::noncanonical_number(
            ptr,
            format!("{val:?}"),
            "canonical seconds string (JSON numbers are forbidden for quantities)",
            vec![format!("Encode `{key}` as a canonical decimal string, never a JSON number")],
        ));
    };
    Millis::parse_canonical_seconds(s, &ptr)
}

/// Parse an optional canonical seconds duration string field.
pub fn opt_duration(
    obj: &JsonValue,
    parent_pointer: &str,
    key: &str,
) -> Result<Option<Millis>, TenfoldRefusal> {
    let Some(val) = obj.get(key) else {
        return Ok(None);
    };
    let ptr = join_pointer(parent_pointer, key);
    let Some(s) = val.as_str() else {
        return Err(TenfoldRefusal::noncanonical_number(
            ptr,
            format!("{val:?}"),
            "canonical seconds string (JSON numbers are forbidden for quantities)",
            vec![format!("Encode `{key}` as a canonical decimal string, never a JSON number")],
        ));
    };
    Millis::parse_canonical_seconds(s, &ptr).map(Some)
}

/// Require a small-count integer field (`i64` within `±(2^53 - 1)`).
pub fn require_int(
    obj: &JsonValue,
    parent_pointer: &str,
    key: &str,
) -> Result<i64, TenfoldRefusal> {
    let ptr = join_pointer(parent_pointer, key);
    let val = require_field(obj, parent_pointer, key)?;
    val.as_i64().ok_or_else(|| {
        TenfoldRefusal::noncanonical_number(
            ptr,
            format!("{val:?}"),
            "small integer count",
            vec![format!("Provide an integer value for `{key}`")],
        )
    })
}

/// Validate the common header (`schema`, `gate`) and the four mandatory
/// no-authorization / `reality: "no-claim"` fields on every record.
pub fn validate_common_header_and_flags(
    obj: &JsonValue,
    expected_schema: SchemaKind,
) -> Result<RecordHeader, TenfoldRefusal> {
    let schema_str = require_str(obj, "", "schema")?;
    let schema = SchemaKind::parse(schema_str)?;
    if schema != expected_schema {
        return Err(TenfoldRefusal::unknown_schema(
            "/schema",
            format!("{schema_str:?}"),
            expected_schema.as_str(),
        ));
    }

    let gate_str = require_str(obj, "", "gate")?;
    let gate = GateId::parse(gate_str, "/gate")?;

    for flag in [
        "execution_authorized",
        "hardware_safety_signoff",
        "sim_to_real_certified",
    ] {
        let ptr = join_pointer("", flag);
        let val = require_field(obj, "", flag)?;
        if val.as_bool() != Some(false) {
            return Err(TenfoldRefusal::noncanonical_number(
                ptr,
                format!("{val:?}"),
                "false",
                vec![format!("Set `{flag}` to boolean `false`")],
            ));
        }
    }

    let reality = require_str(obj, "", "reality")?;
    if reality != "no-claim" {
        return Err(TenfoldRefusal::noncanonical_number(
            "/reality",
            format!("{reality:?}"),
            "\"no-claim\"",
            vec!["Set `reality` to \"no-claim\" (acceptance occurs in simulation)".to_owned()],
        ));
    }

    Ok(RecordHeader { schema, gate })
}

/// Parse an exact reduced rational string (`"p/q"` or whole `"p"`) via `frob_cert::rational::Rational`.
pub fn parse_exact_rational(raw: &str, pointer: &str) -> Result<Rational, TenfoldRefusal> {
    Rational::parse_slash_strict(raw).map_err(|e| {
        TenfoldRefusal::noncanonical_number(
            pointer,
            format!("{raw:?}"),
            "reduced exact rational string \"p/q\" (or whole \"p\")",
            vec![format!("Provide a reduced exact rational string ({e})")],
        )
    })
}

/// Render an exact [`Rational`] as a fixed-precision decimal string with
/// `places` digits after the decimal point using exact `BigInt` arithmetic
/// (zero floating point, determinism class D0).
pub fn render_rational_decimal(
    value: &Rational,
    places: usize,
    mode: RoundingMode,
) -> Result<String, TenfoldRefusal> {
    let places_u32 = u32::try_from(places).map_err(|_| {
        TenfoldRefusal::noncanonical_number(
            "",
            places.to_string(),
            "decimal places fitting in u32",
            vec!["Choose a smaller decimal precision".to_owned()],
        )
    })?;
    let scale = BigInt::from_u64(10).pow_u32(places_u32).map_err(|e| {
        TenfoldRefusal::noncanonical_number(
            "",
            places.to_string(),
            "scale within BigInt bit limits",
            vec![format!("Reduce decimal places ({e})")],
        )
    })?;
    let scaled_num = value.num().mul(&scale).map_err(|e| {
        TenfoldRefusal::noncanonical_number(
            "",
            value.to_slash_string(),
            "scaled numerator within BigInt bit limits",
            vec![format!("Exact rational exceeds BigInt limits ({e})")],
        )
    })?;
    let den = value.den();

    let rounded: BigInt = match mode {
        RoundingMode::OutwardUpper => scaled_num.div_ceil(den).map_err(|e| {
            TenfoldRefusal::noncanonical_number(
                "",
                value.to_slash_string(),
                "exact upper bound",
                vec![format!("Exact rounding failed ({e})")],
            )
        })?,
        RoundingMode::OutwardLower => scaled_num.div_floor(den).map_err(|e| {
            TenfoldRefusal::noncanonical_number(
                "",
                value.to_slash_string(),
                "exact lower bound",
                vec![format!("Exact rounding failed ({e})")],
            )
        })?,
        RoundingMode::HalfToEven => {
            let neg = scaled_num.is_negative();
            let abs_num = scaled_num.abs();
            let (q, rem) = abs_num.div_rem_trunc(den).map_err(|e| {
                TenfoldRefusal::noncanonical_number(
                    "",
                    value.to_slash_string(),
                    "exact division",
                    vec![format!("Exact division failed ({e})")],
                )
            })?;
            let two_rem = rem.add(&rem).map_err(|e| {
                TenfoldRefusal::noncanonical_number(
                    "",
                    value.to_slash_string(),
                    "exact remainder doubling",
                    vec![format!("Exact rounding failed ({e})")],
                )
            })?;
            let mag = match two_rem.cmp(den) {
                std::cmp::Ordering::Less => q,
                std::cmp::Ordering::Greater => q.add(&BigInt::one()).map_err(|e| {
                    TenfoldRefusal::noncanonical_number(
                        "",
                        value.to_slash_string(),
                        "exact increment",
                        vec![format!("Exact rounding failed ({e})")],
                    )
                })?,
                std::cmp::Ordering::Equal => {
                    let (_, parity) = q.div_rem_trunc(&BigInt::from_u64(2)).map_err(|e| {
                        TenfoldRefusal::noncanonical_number(
                            "",
                            value.to_slash_string(),
                            "exact parity",
                            vec![format!("Exact rounding failed ({e})")],
                        )
                    })?;
                    if parity.is_zero() {
                        q
                    } else {
                        q.add(&BigInt::one()).map_err(|e| {
                            TenfoldRefusal::noncanonical_number(
                                "",
                                value.to_slash_string(),
                                "exact tie-to-even increment",
                                vec![format!("Exact rounding failed ({e})")],
                            )
                        })?
                    }
                }
            };
            if neg && !mag.is_zero() {
                mag.negate()
            } else {
                mag
            }
        }
    };

    let neg = rounded.is_negative();
    let digits = rounded.abs().to_decimal_string();
    let sign = if neg { "-" } else { "" };
    if places == 0 {
        return Ok(format!("{sign}{digits}"));
    }
    let padded = if digits.len() <= places {
        let zeros = places + 1 - digits.len();
        format!("{}{digits}", "0".repeat(zeros))
    } else {
        digits
    };
    let split = padded.len() - places;
    Ok(format!("{sign}{}.{}", &padded[..split], &padded[split..]))
}

/// Strictly parse, validate, canonicalize, and content-address any of the five
/// ten-fold harness records (`frob.tenfold.{mission,freeze,ledger,receipt,report}/1`).
pub fn check_record_bytes(bytes: &[u8]) -> Result<CheckedRecord, TenfoldRefusal> {
    let val = parse_json_strict(bytes)?;
    if val.as_obj().is_none() {
        return Err(TenfoldRefusal::noncanonical_number(
            "",
            "non-object JSON root",
            "JSON object",
            vec!["Harness records must be JSON objects".to_owned()],
        ));
    }
    let schema_str = require_str(&val, "", "schema")?;
    let schema = SchemaKind::parse(schema_str)?;
    let (gate, address) = match schema {
        SchemaKind::Mission => {
            let rec = mission::parse_value(&val)?;
            (rec.header.gate, content_address(&val, schema)?)
        }
        SchemaKind::Freeze => {
            let rec = freeze::parse_value(&val)?;
            (rec.header.gate, rec.manifest_hash)
        }
        SchemaKind::Ledger => {
            let rec = ledger::parse_value(&val)?;
            (rec.header.gate, content_address(&val, schema)?)
        }
        SchemaKind::Receipt => {
            let rec = receipt::parse_value(&val)?;
            (rec.header.gate, rec.receipt_hash)
        }
        SchemaKind::Report => {
            let rec = report::parse_value(&val)?;
            (rec.header.gate, rec.report_hash)
        }
    };

    Ok(CheckedRecord {
        schema,
        gate,
        canonical_bytes: canonical_bytes(&val),
        address,
    })
}
