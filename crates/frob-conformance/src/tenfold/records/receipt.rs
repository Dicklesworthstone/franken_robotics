//! Submission receipt record (`frob.tenfold.receipt/1`, skeleton in `N39a-01`,
//! full submission & retry semantics in `N39a-09`).
//!
//! Carries the self-hash field `receipt_hash`, hashed with `receipt_hash = ""`.
//! Lives inside the scorer boundary (`BOUNDARY_FILES`).

use frob_cert::json::JsonValue;

use crate::tenfold::canon::{
    RecordHeader, SchemaKind, canonical_bytes, check_allowed_keys, parse_json_strict,
    require_duration, require_int, require_str, stamp_self_hash, validate_common_header_and_flags,
    verify_self_hash,
};
use crate::tenfold::ids::{Arm, CampaignIndex, FamilyId, Hash32, Millis, MissionHash, StratumId};
use crate::tenfold::refusal::TenfoldRefusal;

pub const ALLOWED_KEYS: &[&str] = &[
    "arm",
    "campaign",
    "elapsed_s",
    "execution_authorized",
    "family_id",
    "gate",
    "hardware_safety_signoff",
    "mission_hash",
    "reality",
    "receipt_hash",
    "schema",
    "sim_to_real_certified",
    "stratum",
    "submission_index",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReceiptRecord {
    pub header: RecordHeader,
    pub arm: Arm,
    pub stratum: StratumId,
    pub family_id: FamilyId,
    pub campaign: CampaignIndex,
    pub mission_hash: MissionHash,
    pub elapsed: Millis,
    pub receipt_hash: Hash32,
    pub submission_index: Option<u32>,
}

impl ReceiptRecord {
    fn to_json_value_with_hash(&self, hash_str: String) -> JsonValue {
        let mut entries = vec![
            ("arm".to_owned(), JsonValue::Str(self.arm.as_str().to_owned())),
            ("campaign".to_owned(), JsonValue::Int(i64::from(self.campaign.0))),
            (
                "elapsed_s".to_owned(),
                JsonValue::Str(self.elapsed.to_canonical_seconds()),
            ),
            ("execution_authorized".to_owned(), JsonValue::Bool(false)),
            ("family_id".to_owned(), JsonValue::Str(self.family_id.as_string())),
            ("gate".to_owned(), JsonValue::Str(self.header.gate.as_string())),
            ("hardware_safety_signoff".to_owned(), JsonValue::Bool(false)),
            (
                "mission_hash".to_owned(),
                JsonValue::Str(self.mission_hash.to_b3_string()),
            ),
            ("reality".to_owned(), JsonValue::Str("no-claim".to_owned())),
            ("receipt_hash".to_owned(), JsonValue::Str(hash_str)),
            (
                "schema".to_owned(),
                JsonValue::Str(SchemaKind::Receipt.as_str().to_owned()),
            ),
            ("sim_to_real_certified".to_owned(), JsonValue::Bool(false)),
            (
                "stratum".to_owned(),
                JsonValue::Str(self.stratum.as_str().to_owned()),
            ),
        ];
        if let Some(idx) = self.submission_index {
            entries.push(("submission_index".to_owned(), JsonValue::Int(i64::from(idx))));
        }
        JsonValue::Obj(entries)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn build_stamped(
        header: RecordHeader,
        arm: Arm,
        stratum: StratumId,
        family_id: FamilyId,
        campaign: CampaignIndex,
        mission_hash: MissionHash,
        elapsed: Millis,
        submission_index: Option<u32>,
    ) -> Result<Self, TenfoldRefusal> {
        let draft = Self {
            header,
            arm,
            stratum,
            family_id,
            campaign,
            mission_hash,
            elapsed,
            receipt_hash: Hash32([0_u8; 32]),
            submission_index,
        };
        let (_, addr) = stamp_self_hash(
            &draft.to_json_value_with_hash(String::new()),
            SchemaKind::Receipt,
        )?;
        Ok(Self {
            receipt_hash: addr,
            ..draft
        })
    }

    #[must_use]
    pub fn to_json_value(&self) -> JsonValue {
        self.to_json_value_with_hash(self.receipt_hash.to_b3_string())
    }

    #[must_use]
    pub fn to_canonical_bytes(&self) -> Vec<u8> {
        canonical_bytes(&self.to_json_value())
    }
}

pub fn parse_value(val: &JsonValue) -> Result<ReceiptRecord, TenfoldRefusal> {
    let _ = check_allowed_keys(val, "", ALLOWED_KEYS)?;
    let header = validate_common_header_and_flags(val, SchemaKind::Receipt)?;
    let arm = Arm::parse(require_str(val, "", "arm")?, "/arm")?;
    let stratum = StratumId::parse(require_str(val, "", "stratum")?, "/stratum")?;
    let family_id = FamilyId::parse(require_str(val, "", "family_id")?, "/family_id")?;
    let campaign = CampaignIndex::from_i64(require_int(val, "", "campaign")?, "/campaign")?;
    let mission_hash = MissionHash::parse(require_str(val, "", "mission_hash")?, "/mission_hash")?;
    let elapsed = require_duration(val, "", "elapsed_s")?;
    let submission_index = match val.get("submission_index") {
        None => None,
        Some(v) => {
            let Some(n) = v.as_i64() else {
                return Err(TenfoldRefusal::noncanonical_number(
                    "/submission_index",
                    format!("{v:?}"),
                    "non-negative integer",
                    vec!["Provide a non-negative integer for submission_index".to_owned()],
                ));
            };
            Some(CampaignIndex::from_i64(n, "/submission_index")?.0)
        }
    };
    let receipt_hash = verify_self_hash(val, SchemaKind::Receipt)?;
    Ok(ReceiptRecord {
        header,
        arm,
        stratum,
        family_id,
        campaign,
        mission_hash,
        elapsed,
        receipt_hash,
        submission_index,
    })
}

pub fn parse_bytes(bytes: &[u8]) -> Result<ReceiptRecord, TenfoldRefusal> {
    let val = parse_json_strict(bytes)?;
    parse_value(&val)
}
