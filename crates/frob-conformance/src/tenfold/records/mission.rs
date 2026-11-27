//! Mission card record (`frob.tenfold.mission/1`, skeleton in `N39a-01`, full
//! five answer kinds and sealed part in `N39a-05`).
//!
//! Lives inside the scorer boundary (`BOUNDARY_FILES`).

use frob_cert::json::JsonValue;

use crate::tenfold::canon::{
    RecordHeader, SchemaKind, canonical_bytes, check_allowed_keys, content_address,
    parse_json_strict, require_duration, require_str, validate_common_header_and_flags,
};
use crate::tenfold::ids::{FamilyId, Hash32, Millis, Regime, StratumId};
use crate::tenfold::refusal::TenfoldRefusal;

pub const ALLOWED_KEYS: &[&str] = &[
    "deadline_s",
    "execution_authorized",
    "family_id",
    "gate",
    "hardware_safety_signoff",
    "mission_id",
    "reality",
    "regime",
    "schema",
    "sim_to_real_certified",
    "stratum",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MissionRecord {
    pub header: RecordHeader,
    pub family_id: FamilyId,
    pub stratum: StratumId,
    pub deadline: Millis,
    pub mission_id: Option<String>,
    pub regime: Option<Regime>,
}

impl MissionRecord {
    #[must_use]
    pub fn to_json_value(&self) -> JsonValue {
        let mut entries = vec![
            (
                "deadline_s".to_owned(),
                JsonValue::Str(self.deadline.to_canonical_seconds()),
            ),
            ("execution_authorized".to_owned(), JsonValue::Bool(false)),
            (
                "family_id".to_owned(),
                JsonValue::Str(self.family_id.as_string()),
            ),
            (
                "gate".to_owned(),
                JsonValue::Str(self.header.gate.as_string()),
            ),
            ("hardware_safety_signoff".to_owned(), JsonValue::Bool(false)),
            ("reality".to_owned(), JsonValue::Str("no-claim".to_owned())),
            (
                "schema".to_owned(),
                JsonValue::Str(SchemaKind::Mission.as_str().to_owned()),
            ),
            ("sim_to_real_certified".to_owned(), JsonValue::Bool(false)),
            (
                "stratum".to_owned(),
                JsonValue::Str(self.stratum.as_str().to_owned()),
            ),
        ];
        if let Some(mid) = &self.mission_id {
            entries.push(("mission_id".to_owned(), JsonValue::Str(mid.clone())));
        }
        if let Some(r) = self.regime {
            entries.push(("regime".to_owned(), JsonValue::Str(r.as_str().to_owned())));
        }
        JsonValue::Obj(entries)
    }

    #[must_use]
    pub fn to_canonical_bytes(&self) -> Vec<u8> {
        canonical_bytes(&self.to_json_value())
    }

    pub fn address(&self) -> Result<Hash32, TenfoldRefusal> {
        content_address(&self.to_json_value(), SchemaKind::Mission)
    }
}

pub fn parse_value(val: &JsonValue) -> Result<MissionRecord, TenfoldRefusal> {
    let _ = check_allowed_keys(val, "", ALLOWED_KEYS)?;
    let header = validate_common_header_and_flags(val, SchemaKind::Mission)?;
    let family_id = FamilyId::parse(require_str(val, "", "family_id")?, "/family_id")?;
    let stratum = StratumId::parse(require_str(val, "", "stratum")?, "/stratum")?;
    let deadline = require_duration(val, "", "deadline_s")?;
    let mission_id = match val.get("mission_id") {
        None => None,
        Some(v) => {
            let Some(s) = v.as_str() else {
                return Err(TenfoldRefusal::noncanonical_number(
                    "/mission_id",
                    format!("{v:?}"),
                    "non-empty string",
                    vec!["Encode mission_id as a non-empty string".to_owned()],
                ));
            };
            if s.trim().is_empty() {
                return Err(TenfoldRefusal::noncanonical_number(
                    "/mission_id",
                    format!("{s:?}"),
                    "non-empty string",
                    vec!["Provide a non-empty mission_id string".to_owned()],
                ));
            }
            Some(s.to_owned())
        }
    };
    let regime = match val.get("regime") {
        None => None,
        Some(v) => {
            let Some(s) = v.as_str() else {
                return Err(TenfoldRefusal::noncanonical_number(
                    "/regime",
                    format!("{v:?}"),
                    "regime in {cold, warm}",
                    vec!["Encode regime as \"cold\" or \"warm\"".to_owned()],
                ));
            };
            Some(Regime::parse(s, "/regime")?)
        }
    };
    Ok(MissionRecord {
        header,
        family_id,
        stratum,
        deadline,
        mission_id,
        regime,
    })
}

pub fn parse_bytes(bytes: &[u8]) -> Result<MissionRecord, TenfoldRefusal> {
    let val = parse_json_strict(bytes)?;
    parse_value(&val)
}
