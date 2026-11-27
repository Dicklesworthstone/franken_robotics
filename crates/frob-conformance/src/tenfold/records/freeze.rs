//! Freeze manifest record (`frob.tenfold.freeze/1`, skeleton in `N39a-01`, full
//! arm freeze fields in `N39a-18`).
//!
//! Carries the self-hash field `manifest_hash`, hashed with `manifest_hash = ""`.
//! Lives inside the scorer boundary (`BOUNDARY_FILES`).

use frob_cert::json::JsonValue;

use crate::tenfold::canon::{
    RecordHeader, SchemaKind, canonical_bytes, check_allowed_keys, opt_duration, parse_json_strict,
    require_str, stamp_self_hash, validate_common_header_and_flags, verify_self_hash,
};
use crate::tenfold::ids::{Arm, Hash32, Millis, Regime, StratumId};
use crate::tenfold::refusal::TenfoldRefusal;

pub const ALLOWED_KEYS: &[&str] = &[
    "arm",
    "execution_authorized",
    "gate",
    "hardware_safety_signoff",
    "manifest_hash",
    "reality",
    "regime",
    "schema",
    "sim_to_real_certified",
    "stratum",
    "wall_budget_s",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FreezeRecord {
    pub header: RecordHeader,
    pub arm: Arm,
    pub stratum: StratumId,
    pub manifest_hash: Hash32,
    pub regime: Option<Regime>,
    pub wall_budget: Option<Millis>,
}

impl FreezeRecord {
    fn to_json_value_with_hash(&self, hash_str: String) -> JsonValue {
        let mut entries = vec![
            (
                "arm".to_owned(),
                JsonValue::Str(self.arm.as_str().to_owned()),
            ),
            ("execution_authorized".to_owned(), JsonValue::Bool(false)),
            (
                "gate".to_owned(),
                JsonValue::Str(self.header.gate.as_string()),
            ),
            ("hardware_safety_signoff".to_owned(), JsonValue::Bool(false)),
            ("manifest_hash".to_owned(), JsonValue::Str(hash_str)),
            ("reality".to_owned(), JsonValue::Str("no-claim".to_owned())),
            (
                "schema".to_owned(),
                JsonValue::Str(SchemaKind::Freeze.as_str().to_owned()),
            ),
            ("sim_to_real_certified".to_owned(), JsonValue::Bool(false)),
            (
                "stratum".to_owned(),
                JsonValue::Str(self.stratum.as_str().to_owned()),
            ),
        ];
        if let Some(r) = self.regime {
            entries.push(("regime".to_owned(), JsonValue::Str(r.as_str().to_owned())));
        }
        if let Some(wb) = self.wall_budget {
            entries.push((
                "wall_budget_s".to_owned(),
                JsonValue::Str(wb.to_canonical_seconds()),
            ));
        }
        JsonValue::Obj(entries)
    }

    /// Construct a freeze record and compute its exact blanked `manifest_hash`.
    pub fn build_stamped(
        header: RecordHeader,
        arm: Arm,
        stratum: StratumId,
        regime: Option<Regime>,
        wall_budget: Option<Millis>,
    ) -> Result<Self, TenfoldRefusal> {
        let draft = Self {
            header,
            arm,
            stratum,
            manifest_hash: Hash32([0_u8; 32]),
            regime,
            wall_budget,
        };
        let (_, addr) = stamp_self_hash(
            &draft.to_json_value_with_hash(String::new()),
            SchemaKind::Freeze,
        )?;
        Ok(Self {
            manifest_hash: addr,
            ..draft
        })
    }

    #[must_use]
    pub fn to_json_value(&self) -> JsonValue {
        self.to_json_value_with_hash(self.manifest_hash.to_b3_string())
    }

    #[must_use]
    pub fn to_canonical_bytes(&self) -> Vec<u8> {
        canonical_bytes(&self.to_json_value())
    }
}

pub fn parse_value(val: &JsonValue) -> Result<FreezeRecord, TenfoldRefusal> {
    let _ = check_allowed_keys(val, "", ALLOWED_KEYS)?;
    let header = validate_common_header_and_flags(val, SchemaKind::Freeze)?;
    let arm = Arm::parse(require_str(val, "", "arm")?, "/arm")?;
    let stratum = StratumId::parse(require_str(val, "", "stratum")?, "/stratum")?;
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
    let wall_budget = opt_duration(val, "", "wall_budget_s")?;
    let manifest_hash = verify_self_hash(val, SchemaKind::Freeze)?;
    Ok(FreezeRecord {
        header,
        arm,
        stratum,
        manifest_hash,
        regime,
        wall_budget,
    })
}

pub fn parse_bytes(bytes: &[u8]) -> Result<FreezeRecord, TenfoldRefusal> {
    let val = parse_json_strict(bytes)?;
    parse_value(&val)
}
