//! Phase ledger record (`frob.tenfold.ledger/1`, skeleton in `N39a-01`, full
//! exclusive/resource tables and 2% closure rule in `N39a-23` / `N39a-24`).
//!
//! Enforces the fourteen closed phase buckets (`Phase::ALL`: thirteen phases
//! plus `unattributed`) and verifies that the exclusive phase durations sum
//! exactly in integer milliseconds to `clock_s`.
//! Lives inside the scorer boundary (`BOUNDARY_FILES`).

use std::collections::BTreeMap;

use frob_cert::json::JsonValue;

use crate::tenfold::canon::{
    RecordHeader, SchemaKind, canonical_bytes, check_allowed_keys, content_address,
    parse_json_strict, require_duration, require_field, require_int, require_str,
    validate_common_header_and_flags,
};
use crate::tenfold::ids::{Arm, CampaignIndex, FamilyId, Hash32, Millis, Phase, Regime, StratumId};
use crate::tenfold::refusal::TenfoldRefusal;

pub const ALLOWED_KEYS: &[&str] = &[
    "arm",
    "campaign",
    "clock_s",
    "execution_authorized",
    "family_id",
    "gate",
    "hardware_safety_signoff",
    "phases",
    "reality",
    "regime",
    "schema",
    "sim_to_real_certified",
    "stratum",
];

const PHASE_KEYS: &[&str] = &[
    "adjudicate",
    "author",
    "certify",
    "check",
    "compile",
    "diagnose",
    "evaluate",
    "propose",
    "qualify",
    "screen",
    "synthesize",
    "train",
    "unattributed",
    "wait",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LedgerRecord {
    pub header: RecordHeader,
    pub arm: Arm,
    pub stratum: StratumId,
    pub family_id: FamilyId,
    pub campaign: CampaignIndex,
    pub regime: Regime,
    pub clock: Millis,
    pub phases: BTreeMap<Phase, Millis>,
}

impl LedgerRecord {
    /// Exact sum of all fourteen exclusive phase buckets in milliseconds.
    pub fn phase_sum(&self) -> Result<Millis, TenfoldRefusal> {
        let mut total = Millis(0);
        for &ms in self.phases.values() {
            total = total.checked_add(ms).ok_or_else(|| {
                TenfoldRefusal::noncanonical_number(
                    "/phases",
                    "phase sum overflow",
                    "sum fitting in u64 milliseconds",
                    vec!["Verify all phase durations fit within u64 milliseconds".to_owned()],
                )
            })?;
        }
        Ok(total)
    }

    #[must_use]
    pub fn to_json_value(&self) -> JsonValue {
        let mut phase_entries = Vec::with_capacity(Phase::ALL.len());
        for p in Phase::ALL {
            let ms = self.phases.get(&p).copied().unwrap_or(Millis(0));
            phase_entries.push((
                p.as_str().to_owned(),
                JsonValue::Str(ms.to_canonical_seconds()),
            ));
        }
        JsonValue::Obj(vec![
            ("arm".to_owned(), JsonValue::Str(self.arm.as_str().to_owned())),
            ("campaign".to_owned(), JsonValue::Int(i64::from(self.campaign.0))),
            (
                "clock_s".to_owned(),
                JsonValue::Str(self.clock.to_canonical_seconds()),
            ),
            ("execution_authorized".to_owned(), JsonValue::Bool(false)),
            ("family_id".to_owned(), JsonValue::Str(self.family_id.as_string())),
            ("gate".to_owned(), JsonValue::Str(self.header.gate.as_string())),
            ("hardware_safety_signoff".to_owned(), JsonValue::Bool(false)),
            ("phases".to_owned(), JsonValue::Obj(phase_entries)),
            ("reality".to_owned(), JsonValue::Str("no-claim".to_owned())),
            (
                "regime".to_owned(),
                JsonValue::Str(self.regime.as_str().to_owned()),
            ),
            (
                "schema".to_owned(),
                JsonValue::Str(SchemaKind::Ledger.as_str().to_owned()),
            ),
            ("sim_to_real_certified".to_owned(), JsonValue::Bool(false)),
            (
                "stratum".to_owned(),
                JsonValue::Str(self.stratum.as_str().to_owned()),
            ),
        ])
    }

    #[must_use]
    pub fn to_canonical_bytes(&self) -> Vec<u8> {
        canonical_bytes(&self.to_json_value())
    }

    pub fn address(&self) -> Result<Hash32, TenfoldRefusal> {
        content_address(&self.to_json_value(), SchemaKind::Ledger)
    }
}

pub fn parse_value(val: &JsonValue) -> Result<LedgerRecord, TenfoldRefusal> {
    let _ = check_allowed_keys(val, "", ALLOWED_KEYS)?;
    let header = validate_common_header_and_flags(val, SchemaKind::Ledger)?;
    let arm = Arm::parse(require_str(val, "", "arm")?, "/arm")?;
    let stratum = StratumId::parse(require_str(val, "", "stratum")?, "/stratum")?;
    let family_id = FamilyId::parse(require_str(val, "", "family_id")?, "/family_id")?;
    let campaign = CampaignIndex::from_i64(require_int(val, "", "campaign")?, "/campaign")?;
    let regime = Regime::parse(require_str(val, "", "regime")?, "/regime")?;
    let clock = require_duration(val, "", "clock_s")?;

    let phases_val = require_field(val, "", "phases")?;
    let _ = check_allowed_keys(phases_val, "/phases", PHASE_KEYS)?;

    let mut phases = BTreeMap::new();
    let mut sum = Millis(0);
    for p in Phase::ALL {
        let ms = require_duration(phases_val, "/phases", p.as_str())?;
        sum = sum.checked_add(ms).ok_or_else(|| {
            TenfoldRefusal::noncanonical_number(
                "/phases",
                "phase duration sum overflows u64",
                "exclusive phase durations summing to clock_s",
                vec!["Ensure phase durations sum within u64 milliseconds".to_owned()],
            )
        })?;
        phases.insert(p, ms);
    }

    if sum != clock {
        return Err(TenfoldRefusal::noncanonical_number(
            "/clock_s",
            format!(
                "clock_s={} != sum(phases)={}",
                clock.to_canonical_seconds(),
                sum.to_canonical_seconds()
            ),
            "exclusive phase table summing exactly to clock_s in milliseconds",
            vec!["Ensure exclusive phase durations sum exactly to clock_s".to_owned()],
        ));
    }

    Ok(LedgerRecord {
        header,
        arm,
        stratum,
        family_id,
        campaign,
        regime,
        clock,
        phases,
    })
}

pub fn parse_bytes(bytes: &[u8]) -> Result<LedgerRecord, TenfoldRefusal> {
    let val = parse_json_strict(bytes)?;
    parse_value(&val)
}
