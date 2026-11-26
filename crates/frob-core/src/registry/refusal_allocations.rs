//! Refusal allocation ledger (`registries/refusal_allocations.toml`), pure
//! order-independent allocator (Rules 1–8 + Review Note 6), interim-use list,
//! cross-chapter checks 1–6, and deterministic Markdown printout generator
//! (`N35-33`, `ADR-NUM-refusal-8xx`).
//!
//! Determinism class D0; std-only; prints nothing (L-7); zero `unsafe`.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::sync::OnceLock;

use crate::registry::refusals::{Milestone, RefusalRegistry, RefusalRow};
use crate::registry::toml::{self, Fields, Spanned, Table, TomlError, Value};

pub const EMBEDDED_REFUSAL_ALLOCATIONS_TOML: &str =
    include_str!("../../../../registries/refusal_allocations.toml");

pub const REFUSALS_MD_MARKER: &str = "<!-- @generated from registries/refusals.toml, registries/refusal_allocations.toml by xtask::generate::refusal_printout (N35-33); do not edit by hand. -->";

/// Count of active printed refusal codes in Appendix O of the plan (296 printed
/// numbers minus retired `FROB-REF-730` and `FROB-REF-882`).
pub const PRINTED_ACTIVE_CODE_COUNT: usize = 294;

/// Code to defining chapter from the plan's Appendix O source.
#[must_use]
pub fn printed_plan_codes() -> &'static BTreeMap<u16, u32> {
    static CODES: OnceLock<BTreeMap<u16, u32>> = OnceLock::new();
    CODES.get_or_init(|| {
        let mut codes = BTreeMap::new();
        for line in include_str!("../../../../planning/fable/blend/APPENDIX_O_REFUSALS.md").lines()
        {
            if !line.starts_with("| FROB-REF-") {
                continue;
            }
            let code = line.split('|').nth(1).expect("Appendix O code").trim();
            let section = line.rsplit('|').nth(1).expect("Appendix O section");
            let chapter = parse_defined_in_key(section).0;
            let number = parse_refusal_number(code).expect("valid Appendix O code");
            assert!(codes.insert(number, chapter).is_none(), "duplicate {code}");
        }
        assert_eq!(codes.len(), PRINTED_ACTIVE_CODE_COUNT);
        codes
    })
}

/// Chapter bands explicitly reserved by the plan (§14/§15, review note 6).
pub const PLAN_RESERVED_BLOCKS: &[(u32, u16, u16)] = &[(14, 101, 129), (15, 130, 139)];

// Milestones from the N<chapter>-REF bead definitions. Membership and counts
// come from Appendix O, independently of which registry rows have landed.
fn registry_due(chapter: u32) -> Result<Milestone, AllocationError> {
    match chapter {
        12 | 13 | 16 | 21 | 32..=35 | 37..=39 | 41 => Ok(Milestone::M0),
        14 | 15 | 18 | 20 | 22 | 24 | 25 | 28..=30 => Ok(Milestone::M1),
        17 | 19 | 23 | 27 | 36 => Ok(Milestone::M2),
        26 => Ok(Milestone::M3),
        31 => Ok(Milestone::M5),
        _ => Err(AllocationError::InvalidField {
            context: format!("chapter {chapter}"),
            detail: "missing refusal registry milestone".into(),
        }),
    }
}

const FAMILY_LABELS: &[(u8, &str, u8)] = &[
    (1, "Design and model", 3),
    (2, "Numerics and integrator admission", 3),
    (3, "Scene", 3),
    (4, "Control and estimation", 3),
    (5, "Learning and evaluation protocol", 3),
    (6, "Backend and device", 6),
    (7, "Budget and capability", 4),
    (8, "Evidence and protocol", 3),
    (9, "Internal", 9),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AllocationStatus {
    Requested,
    Assigned,
    Merged,
    Withdrawn,
    Held,
}

impl AllocationStatus {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Requested => "requested",
            Self::Assigned => "assigned",
            Self::Merged => "merged",
            Self::Withdrawn => "withdrawn",
            Self::Held => "held",
        }
    }

    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "requested" => Some(Self::Requested),
            "assigned" => Some(Self::Assigned),
            "merged" => Some(Self::Merged),
            "withdrawn" => Some(Self::Withdrawn),
            "held" => Some(Self::Held),
            _ => None,
        }
    }
}

impl fmt::Display for AllocationStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InterimSpec {
    None,
    Codes(Vec<String>),
}

impl InterimSpec {
    #[must_use]
    pub fn codes(&self) -> &[String] {
        match self {
            Self::None => &[],
            Self::Codes(xs) => xs.as_slice(),
        }
    }

    #[must_use]
    pub fn display(&self) -> String {
        match self {
            Self::None => "none".into(),
            Self::Codes(xs) => xs.join(", "),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HoldEntry {
    pub family: u8,
    pub reason: String,
    pub ruling: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RetiredEntry {
    pub code: String,
    pub number: u16,
    pub replaced_by: String,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NeighbourPair {
    pub left: String,
    pub right: String,
    pub left_site: String,
    pub right_site: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AllocationRequest {
    pub request: String,
    pub requested_by: String,
    pub defined_in: String,
    pub order_hint: u32,
    pub condition: String,
    pub family: u8,
    pub subfamily: Option<String>,
    pub proposed: Option<u16>,
    pub interim: InterimSpec,
    pub interim_detail: Option<String>,
    pub interim_pointer: Option<String>,
    pub assigned: Option<String>,
    pub merged_into: Vec<String>,
    pub status: AllocationStatus,
    pub reason: Option<String>,
}

impl AllocationRequest {
    #[must_use]
    pub fn effective_detail(&self) -> &str {
        self.interim_detail.as_deref().unwrap_or(&self.request)
    }

    #[must_use]
    pub fn assigned_number(&self) -> Option<u16> {
        self.assigned.as_deref().and_then(parse_refusal_number)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AllocationLedger {
    pub schema: String,
    pub holds: Vec<HoldEntry>,
    pub retired: Vec<RetiredEntry>,
    pub neighbour_pairs: Vec<NeighbourPair>,
    pub requests: Vec<AllocationRequest>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AllocationError {
    Toml(TomlError),
    InvalidSchema {
        observed: String,
    },
    InvalidField {
        context: String,
        detail: String,
    },
    DuplicateRequest {
        slug: String,
    },
    MissingSubfamily {
        slug: String,
    },
    InvalidSubfamily {
        slug: String,
        subfamily: String,
    },
    InvalidInterim {
        slug: String,
        value: String,
        reason: String,
    },
    DuplicateInterimPair {
        code: String,
        detail: String,
        first: String,
        second: String,
    },
    DuplicateAssignment {
        code: String,
        first: String,
        second: String,
    },
    RetiredCodeReused {
        code: String,
        context: String,
    },
    RegistryOrderOrDuplicate {
        detail: String,
    },
    RowMetadataIncomplete {
        code: String,
        detail: String,
    },
    ExitMismatch {
        code: String,
        detail: String,
    },
    LedgerRegistryMismatch {
        code: String,
        detail: String,
    },
    NeighbourPairError {
        left: String,
        right: String,
        detail: String,
    },
    MilestoneCountShortfall {
        bead: String,
        detail: String,
    },
}

impl From<TomlError> for AllocationError {
    fn from(e: TomlError) -> Self {
        Self::Toml(e)
    }
}

impl fmt::Display for AllocationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Toml(e) => fmt::Display::fmt(e, f),
            Self::InvalidSchema { observed } => {
                write!(f, "invalid refusal allocation schema `{observed}`")
            }
            Self::InvalidField { context, detail } => write!(f, "{context}: {detail}"),
            Self::DuplicateRequest { slug } => write!(f, "duplicate ledger request `{slug}`"),
            Self::MissingSubfamily { slug } => {
                write!(
                    f,
                    "7xx request `{slug}` must declare a `subfamily` decade (e.g. `76x`)"
                )
            }
            Self::InvalidSubfamily { slug, subfamily } => {
                write!(f, "request `{slug}` has invalid subfamily `{subfamily}`")
            }
            Self::InvalidInterim {
                slug,
                value,
                reason,
            } => {
                write!(
                    f,
                    "request `{slug}` has invalid interim `{value}`: {reason}"
                )
            }
            Self::DuplicateInterimPair {
                code,
                detail,
                first,
                second,
            } => write!(
                f,
                "unsettled requests `{first}` and `{second}` share interim pair ({code}, `{detail}`)"
            ),
            Self::DuplicateAssignment {
                code,
                first,
                second,
            } => {
                write!(
                    f,
                    "requests `{first}` and `{second}` share assigned code `{code}`"
                )
            }
            Self::RetiredCodeReused { code, context } => {
                write!(f, "retired code `{code}` reused in {context}")
            }
            Self::RegistryOrderOrDuplicate { detail } => write!(f, "check 1 failed: {detail}"),
            Self::RowMetadataIncomplete { code, detail } => {
                write!(f, "check 2 failed for `{code}`: {detail}")
            }
            Self::ExitMismatch { code, detail } => {
                write!(f, "check 3 failed for `{code}`: {detail}")
            }
            Self::LedgerRegistryMismatch { code, detail } => {
                write!(f, "check 4 failed for `{code}`: {detail}")
            }
            Self::NeighbourPairError {
                left,
                right,
                detail,
            } => {
                write!(f, "check 5 failed for ({left}, {right}): {detail}")
            }
            Self::MilestoneCountShortfall { bead, detail } => {
                write!(f, "check 6 failed for `{bead}`: {detail}")
            }
        }
    }
}
impl std::error::Error for AllocationError {}

/// Whole-token canonical code parser matching `^FROB-REF-[1-9][0-9]{2,3}$` (`ADR-NUM-refusal-8xx`).
#[must_use]
pub fn parse_refusal_number(code: &str) -> Option<u16> {
    let rest = code.strip_prefix("FROB-REF-")?;
    if !(rest.len() == 3 || rest.len() == 4) || !rest.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    if rest.starts_with('0') {
        return None;
    }
    rest.parse::<u16>().ok()
}

/// Whole-token refusal-code equality helper (`ADR-NUM-refusal-8xx`: `FROB-REF-810` != `FROB-REF-8100`).
#[must_use]
pub fn refusal_code_token_eq(a: &str, b: &str) -> bool {
    parse_refusal_number(a).is_some_and(|na| parse_refusal_number(b) == Some(na))
}

/// Whole-token search for `code` inside text (rejects prefix matches like `FROB-REF-810` inside `FROB-REF-8100`).
#[must_use]
pub fn text_contains_refusal_token(haystack: &str, code: &str) -> bool {
    if parse_refusal_number(code).is_none() {
        return false;
    }
    let bytes = haystack.as_bytes();
    let needle = code.as_bytes();
    let mut start = 0usize;
    while let Some(pos) = haystack[start..].find(code) {
        let abs = start + pos;
        let end = abs + needle.len();
        let left_ok = abs == 0 || !bytes[abs - 1].is_ascii_alphanumeric();
        let right_ok = end == bytes.len() || !bytes[end].is_ascii_alphanumeric();
        if left_ok && right_ok {
            return true;
        }
        start = abs + 1;
    }
    false
}

#[must_use]
pub fn format_refusal_code(num: u16) -> String {
    format!("FROB-REF-{num}")
}

#[must_use]
pub const fn code_family(num: u16) -> u8 {
    if num >= 1000 {
        (num / 1000) as u8
    } else {
        (num / 100) as u8
    }
}

/// Canonical `(chapter, subsection, sub_subsection)` sort key parsed from `defined_in` (e.g. `§13.11`).
#[must_use]
pub fn parse_defined_in_key(s: &str) -> (u32, u32, u32, String) {
    let trimmed = s.trim().trim_start_matches('§').trim_start_matches('S');
    let mut parts = trimmed.split('.');
    let parse_num = |p: Option<&str>| -> u32 {
        p.map(|tok| {
            let digits: String = tok.chars().take_while(char::is_ascii_digit).collect();
            digits.parse::<u32>().unwrap_or(u32::MAX)
        })
        .unwrap_or(0)
    };
    let c = parse_num(parts.next());
    let sub = parse_num(parts.next());
    let sub2 = parse_num(parts.next());
    (c, sub, sub2, s.to_owned())
}

fn parse_subfamily_range(family: u8, subfamily: Option<&str>) -> Option<(u16, u16)> {
    if family == 7 {
        let sf = subfamily?;
        let b = sf.as_bytes();
        if b.len() == 3 && b[0] == b'7' && b[1].is_ascii_digit() && b[2] == b'x' {
            let dec = u16::from(b[1] - b'0');
            let lo = 700 + dec * 10 + u16::from(dec == 0);
            let hi = 700 + dec * 10 + 9;
            return Some((lo, hi));
        }
        return None;
    }
    if let Some(sf) = subfamily {
        let b = sf.as_bytes();
        if b.len() == 3 && b[0] == b'0' + family && b[1].is_ascii_digit() && b[2] == b'x' {
            let dec = u16::from(b[1] - b'0');
            let base = u16::from(family) * 100 + dec * 10;
            return Some((base + u16::from(dec == 0), base + 9));
        }
        return None;
    }
    let lo = u16::from(family) * 100 + 1;
    let hi = u16::from(family) * 100 + 99;
    Some((lo, hi))
}

impl AllocationLedger {
    pub fn load() -> Result<Self, AllocationError> {
        Self::from_bytes(EMBEDDED_REFUSAL_ALLOCATIONS_TOML.as_bytes())
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, AllocationError> {
        let mut root = Fields::new(toml::parse(bytes)?, "<refalloc>");
        let ledger = Self::parse_fields(&mut root).map_err(|err| root.fail(err))?;
        root.finish()?;
        Ok(ledger)
    }

    fn parse_fields(root: &mut Fields) -> Result<Self, AllocationError> {
        let schema = root.require_str("schema")?;
        if schema != "frob.refalloc/1" {
            return Err(AllocationError::InvalidSchema { observed: schema });
        }

        let mut holds = Vec::new();
        for (idx, sp) in root
            .opt_array("hold")?
            .unwrap_or_default()
            .into_iter()
            .enumerate()
        {
            let tbl = expect_table(sp, &format!("hold[{idx}]"))?;
            let mut hf = Fields::new(tbl, format!("hold[{idx}]"));
            let fam = hf.require_int("family")?;
            if !(1..=9).contains(&fam) {
                return Err(hf.fail(AllocationError::InvalidField {
                    context: format!("hold[{idx}].family"),
                    detail: format!("family must be 1..=9, got {fam}"),
                }));
            }
            let reason = hf.require_str("reason")?;
            let ruling = hf.opt_str("ruling")?;
            hf.finish()?;
            holds.push(HoldEntry {
                family: fam as u8,
                reason,
                ruling,
            });
        }

        let mut retired = Vec::new();
        for (idx, sp) in root
            .opt_array("retired")?
            .unwrap_or_default()
            .into_iter()
            .enumerate()
        {
            let tbl = expect_table(sp, &format!("retired[{idx}]"))?;
            let mut rf = Fields::new(tbl, format!("retired[{idx}]"));
            let code = rf.require_str("code")?;
            let number = parse_refusal_number(&code).ok_or_else(|| {
                rf.fail(AllocationError::InvalidField {
                    context: format!("retired[{idx}].code"),
                    detail: format!("invalid refusal code `{code}`"),
                })
            })?;
            let replaced_by = rf.require_str("replaced_by")?;
            if parse_refusal_number(&replaced_by).is_none() {
                return Err(rf.fail(AllocationError::InvalidField {
                    context: format!("retired[{idx}].replaced_by"),
                    detail: format!("invalid replacement code `{replaced_by}`"),
                }));
            }
            let reason = rf.require_str("reason")?;
            rf.finish()?;
            retired.push(RetiredEntry {
                code,
                number,
                replaced_by,
                reason,
            });
        }

        let mut neighbour_pairs = Vec::new();
        for (idx, sp) in root
            .opt_array("neighbour_pair")?
            .unwrap_or_default()
            .into_iter()
            .enumerate()
        {
            let tbl = expect_table(sp, &format!("neighbour_pair[{idx}]"))?;
            let mut nf = Fields::new(tbl, format!("neighbour_pair[{idx}]"));
            let left = nf.require_str("left")?;
            let right = nf.require_str("right")?;
            let left_site = nf.require_str("left_site")?;
            let right_site = nf.require_str("right_site")?;
            nf.finish()?;
            neighbour_pairs.push(NeighbourPair {
                left,
                right,
                left_site,
                right_site,
            });
        }

        let mut requests = Vec::new();
        let mut seen_slugs = BTreeSet::new();
        for (idx, sp) in root
            .opt_array("request")?
            .unwrap_or_default()
            .into_iter()
            .enumerate()
        {
            let tbl = expect_table(sp, &format!("request[{idx}]"))?;
            let mut qf = Fields::new(tbl, format!("request[{idx}]"));
            let slug = qf.require_str("request")?;
            if !seen_slugs.insert(slug.clone()) {
                return Err(qf.fail(AllocationError::DuplicateRequest { slug }));
            }
            let requested_by = qf.require_str("requested_by")?;
            let defined_in = qf.require_str("defined_in")?;
            let order_hint = qf
                .opt_int("order_hint")?
                .map(|n| u32::try_from(n).unwrap_or(0))
                .unwrap_or(0);
            let condition = qf.require_str("condition")?;
            let fam_i = qf.require_int("family")?;
            if !(1..=9).contains(&fam_i) {
                return Err(qf.fail(AllocationError::InvalidField {
                    context: format!("request `{slug}`.family"),
                    detail: format!("family must be 1..=9, got {fam_i}"),
                }));
            }
            let family = fam_i as u8;
            let subfamily = qf.opt_str("subfamily")?;
            if family == 7 && subfamily.is_none() {
                return Err(qf.fail(AllocationError::MissingSubfamily { slug }));
            }
            if parse_subfamily_range(family, subfamily.as_deref()).is_none() {
                return Err(qf.fail(AllocationError::InvalidSubfamily {
                    slug,
                    subfamily: subfamily.unwrap_or_default(),
                }));
            }
            let proposed = match qf.opt_int("proposed")? {
                Some(n) => {
                    let u = u16::try_from(n).map_err(|_| {
                        qf.fail(AllocationError::InvalidField {
                            context: format!("request `{slug}`.proposed"),
                            detail: format!("out-of-range proposed number {n}"),
                        })
                    })?;
                    Some(u)
                }
                None => None,
            };
            let interim = parse_interim_field(&mut qf, &slug).map_err(|err| qf.fail(err))?;
            let interim_detail = qf.opt_str("interim_detail")?;
            let interim_pointer = qf.opt_str("interim_pointer")?;
            let assigned = qf.opt_str("assigned")?;
            if let Some(ref code) = assigned
                && parse_refusal_number(code).is_none()
            {
                return Err(qf.fail(AllocationError::InvalidField {
                    context: format!("request `{slug}`.assigned"),
                    detail: format!("invalid refusal code `{code}`"),
                }));
            }
            let merged_into = match qf.opt_array("merged_into")? {
                Some(arr) => extract_str_vec(&format!("request `{slug}`.merged_into"), arr)
                    .map_err(|err| qf.fail(err))?,
                None => Vec::new(),
            };
            let status_str = qf.require_str("status")?;
            let status = AllocationStatus::parse(&status_str).ok_or_else(|| {
                qf.fail(AllocationError::InvalidField {
                    context: format!("request `{slug}`.status"),
                    detail: format!("unknown status `{status_str}`"),
                })
            })?;
            let reason = qf.opt_str("reason")?;
            qf.finish()?;

            requests.push(AllocationRequest {
                request: slug,
                requested_by,
                defined_in,
                order_hint,
                condition,
                family,
                subfamily,
                proposed,
                interim,
                interim_detail,
                interim_pointer,
                assigned,
                merged_into,
                status,
                reason,
            });
        }
        Ok(Self {
            schema,
            holds,
            retired,
            neighbour_pairs,
            requests,
        })
    }

    #[must_use]
    pub fn retired_numbers(&self) -> BTreeSet<u16> {
        self.retired.iter().map(|r| r.number).collect()
    }

    #[must_use]
    pub fn is_family_held(&self, family: u8) -> bool {
        self.holds.iter().any(|h| h.family == family)
    }
}

fn expect_table(sp: Spanned<Value>, ctx: &str) -> Result<Table, AllocationError> {
    match sp.value {
        Value::Table(t) => Ok(t),
        other => Err(AllocationError::InvalidField {
            context: ctx.to_owned(),
            detail: format!("expected table, found {}", other.type_name()),
        }),
    }
}

fn extract_str_vec(ctx: &str, items: Vec<Spanned<Value>>) -> Result<Vec<String>, AllocationError> {
    let mut out = Vec::with_capacity(items.len());
    for (i, sp) in items.into_iter().enumerate() {
        match sp.value {
            Value::Str(s) => out.push(s),
            other => {
                return Err(AllocationError::InvalidField {
                    context: format!("{ctx}[{i}]"),
                    detail: format!("expected string, found {}", other.type_name()),
                });
            }
        }
    }
    Ok(out)
}

fn parse_interim_field(qf: &mut Fields, slug: &str) -> Result<InterimSpec, AllocationError> {
    let Some(sp) = qf.take("interim") else {
        return Ok(InterimSpec::None);
    };
    match sp.value {
        Value::Str(s) => {
            if s == "none" {
                Ok(InterimSpec::None)
            } else if parse_refusal_number(&s).is_some() {
                Ok(InterimSpec::Codes(vec![s]))
            } else {
                Err(AllocationError::InvalidInterim {
                    slug: slug.to_owned(),
                    value: s,
                    reason: "must be `none` or a registered `FROB-REF-...` code (never a bare slug or number)".into(),
                })
            }
        }
        Value::Array(arr) => {
            let codes = extract_str_vec(&format!("request `{slug}`.interim"), arr)?;
            if codes.is_empty() {
                return Ok(InterimSpec::None);
            }
            for c in &codes {
                if parse_refusal_number(c).is_none() {
                    return Err(AllocationError::InvalidInterim {
                        slug: slug.to_owned(),
                        value: c.clone(),
                        reason: "interim array element must be a canonical `FROB-REF-...` code".into(),
                    });
                }
            }
            Ok(InterimSpec::Codes(codes))
        }
        Value::Int(n) => Err(AllocationError::InvalidInterim {
            slug: slug.to_owned(),
            value: n.to_string(),
            reason: "bare numeric interim is forbidden; use a registered `FROB-REF-...` string or `none`".into(),
        }),
        other => Err(AllocationError::InvalidInterim {
            slug: slug.to_owned(),
            value: other.type_name().to_owned(),
            reason: "expected `none`, a `FROB-REF-...` string, or an array of `FROB-REF-...` strings".into(),
        }),
    }
}

// ============================================================================
// Pure Allocator (Rules 1–8 + Review Note 6)
// ============================================================================

/// Existing code entry fed into [`allocate_round`] (registry + plan-time blocks).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExistingCode {
    pub number: u16,
    pub chapter: u32,
    pub condition: String,
}

/// Input to the pure deterministic allocation function [`allocate_round`].
#[derive(Debug, Clone)]
pub struct AllocationInput {
    pub existing: Vec<ExistingCode>,
    pub reserved_blocks: Vec<(u32, u16, u16)>,
    pub retired: BTreeSet<u16>,
    pub holds: BTreeSet<u8>,
    pub allow_8xx_allocation: bool,
    pub requests: Vec<AllocationRequest>,
}

impl AllocationInput {
    /// Build the canonical plan-time allocation context (all 294 printed codes,
    /// plan chapter blocks including Review Note 6, and `ledger`'s retired/hold sets).
    #[must_use]
    pub fn from_plan_and_ledger(reg: &RefusalRegistry, ledger: &AllocationLedger) -> Self {
        let mut by_num: BTreeMap<u16, ExistingCode> = BTreeMap::new();
        for (&num, &chapter) in printed_plan_codes() {
            by_num.insert(
                num,
                ExistingCode {
                    number: num,
                    chapter,
                    condition: String::new(),
                },
            );
        }
        for row in reg.rows() {
            by_num
                .entry(row.number)
                .and_modify(|code| code.condition.clone_from(&row.meaning))
                .or_insert_with(|| ExistingCode {
                    number: row.number,
                    chapter: parse_defined_in_key(&row.defined_in).0,
                    condition: row.meaning.clone(),
                });
        }
        for req in &ledger.requests {
            if req.status == AllocationStatus::Assigned
                && let Some(number) = req.assigned_number()
            {
                by_num.entry(number).or_insert_with(|| ExistingCode {
                    number,
                    chapter: parse_defined_in_key(&req.defined_in).0,
                    condition: String::new(),
                });
            }
        }
        Self {
            existing: by_num.into_values().collect(),
            reserved_blocks: PLAN_RESERVED_BLOCKS.to_vec(),
            retired: ledger.retired_numbers(),
            holds: ledger.holds.iter().map(|h| h.family).collect(),
            allow_8xx_allocation: false,
            requests: ledger.requests.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AllocationOutcome {
    pub results: Vec<AllocationRequest>,
}

impl AllocationOutcome {
    #[must_use]
    pub fn by_slug(&self, slug: &str) -> Option<&AllocationRequest> {
        self.results.iter().find(|r| r.request == slug)
    }
}

fn normalize_condition(s: &str) -> String {
    s.split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_ascii_lowercase()
}

/// Pure, order-independent allocation function implementing Rules 1–8 + Review Note 6.
pub fn allocate_round(input: &AllocationInput) -> Result<AllocationOutcome, AllocationError> {
    let mut taken: BTreeSet<u16> = input.existing.iter().map(|e| e.number).collect();
    let retired = &input.retired;
    taken.extend(
        input
            .requests
            .iter()
            .filter(|req| req.status == AllocationStatus::Assigned)
            .filter_map(AllocationRequest::assigned_number),
    );

    // Build chapter blocks per family/subfamily from existing codes + reserved blocks.
    let mut chapter_spans: BTreeMap<(u32, u8), (u16, u16)> = BTreeMap::new();
    for e in &input.existing {
        if e.chapter == 0 {
            continue;
        }
        let fam = code_family(e.number);
        chapter_spans
            .entry((e.chapter, fam))
            .and_modify(|(lo, hi)| {
                *lo = (*lo).min(e.number);
                *hi = (*hi).max(e.number);
            })
            .or_insert((e.number, e.number));
    }
    for &(ch, blo, bhi) in &input.reserved_blocks {
        let fam = code_family(blo);
        chapter_spans
            .entry((ch, fam))
            .and_modify(|(lo, hi)| {
                *lo = (*lo).min(blo);
                *hi = (*hi).max(bhi);
            })
            .or_insert((blo, bhi));
    }

    // Condition map for Rule 1 deduplication.
    let mut condition_to_code: BTreeMap<String, u16> = BTreeMap::new();
    for e in &input.existing {
        let norm = normalize_condition(&e.condition);
        if !norm.is_empty() {
            condition_to_code.entry(norm).or_insert(e.number);
        }
    }

    // Validate and sort requests canonically by Rule 3 (defined_in -> order_hint -> slug).
    let mut sorted = input.requests.clone();
    for req in &sorted {
        if req.family == 7 && req.subfamily.is_none() {
            return Err(AllocationError::MissingSubfamily {
                slug: req.request.clone(),
            });
        }
        if parse_subfamily_range(req.family, req.subfamily.as_deref()).is_none() {
            return Err(AllocationError::InvalidSubfamily {
                slug: req.request.clone(),
                subfamily: req.subfamily.clone().unwrap_or_default(),
            });
        }
    }
    sorted.sort_by(|a, b| {
        parse_defined_in_key(&a.defined_in)
            .cmp(&parse_defined_in_key(&b.defined_in))
            .then_with(|| a.order_hint.cmp(&b.order_hint))
            .then_with(|| a.request.cmp(&b.request))
    });
    let in_other_block = |n, chapter, family| {
        chapter_spans.iter().any(|(&(owner, fam), &(lo, hi))| {
            fam == family && owner != chapter && (lo..=hi).contains(&n)
        })
    };

    // Rule 5 DESIGN: collect all valid in-range proposals of eligible requests so Rule 4
    // never offers another request's proposed number during this run.
    let mut round_proposals: BTreeSet<u16> = BTreeSet::new();
    for req in &sorted {
        if matches!(
            req.status,
            AllocationStatus::Assigned | AllocationStatus::Merged | AllocationStatus::Withdrawn
        ) {
            continue;
        }
        if req.family == 8 && !input.allow_8xx_allocation {
            continue;
        }
        if let Some(p) = req.proposed
            && let Some((lo, hi)) = parse_subfamily_range(req.family, req.subfamily.as_deref())
            && (lo..=hi).contains(&p)
            && !taken.contains(&p)
            && !retired.contains(&p)
            && !in_other_block(p, parse_defined_in_key(&req.defined_in).0, req.family)
        {
            round_proposals.insert(p);
        }
    }

    let mut out = Vec::with_capacity(sorted.len());
    for mut req in sorted {
        if req.status == AllocationStatus::Assigned {
            out.push(req);
            continue;
        }
        // Pre-recorded withdrawals stay withdrawn.
        if req.status == AllocationStatus::Withdrawn {
            req.assigned = None;
            out.push(req);
            continue;
        }

        // Rule 1: Deduplicate before allocating.
        if req.status == AllocationStatus::Merged && !req.merged_into.is_empty() {
            req.assigned = None;
            out.push(req);
            continue;
        }
        let norm_cond = normalize_condition(&req.condition);
        if !norm_cond.is_empty()
            && let Some(&existing_num) = condition_to_code.get(&norm_cond)
        {
            req.status = AllocationStatus::Merged;
            req.assigned = None;
            req.merged_into = vec![format_refusal_code(existing_num)];
            req.reason = None;
            out.push(req);
            continue;
        }

        // Rule 8: No 8xx number in first round before owner decision/second round.
        if req.family == 8 && !input.allow_8xx_allocation {
            req.assigned = None;
            if input.holds.contains(&8) {
                req.status = AllocationStatus::Held;
                req.reason = Some("owner-decision".into());
            } else {
                req.status = AllocationStatus::Requested;
                req.reason = Some("owner-decision".into());
            }
            out.push(req);
            continue;
        }
        if input.holds.contains(&req.family) {
            req.assigned = None;
            req.status = AllocationStatus::Held;
            req.reason = Some("hold".into());
            out.push(req);
            continue;
        }

        let (lo, hi) = parse_subfamily_range(req.family, req.subfamily.as_deref()).unwrap();
        let req_chapter = parse_defined_in_key(&req.defined_in).0;
        let in_any_block = |n: u16| -> bool {
            chapter_spans
                .iter()
                .any(|(&(_, fam), &(blo, bhi))| fam == req.family && (blo..=bhi).contains(&n))
        };

        // Rule 5: Take proposal if in range and free (earlier in Rule 3 order wins ties).
        let mut chosen: Option<u16> = None;
        if let Some(p) = req.proposed
            && (lo..=hi).contains(&p)
            && !taken.contains(&p)
            && !retired.contains(&p)
            && !in_other_block(p, req_chapter, req.family)
        {
            chosen = Some(p);
        }

        // Rule 4: First try lowest free above defining chapter's highest code in range,
        // otherwise lowest free in range outside every chapter block (and not reserved by
        // another request's proposal in this run).
        if chosen.is_none() {
            let chapter_hi = input
                .existing
                .iter()
                .filter(|e| e.chapter == req_chapter && (lo..=hi).contains(&e.number))
                .map(|e| e.number)
                .max();
            if let Some(ch_max) = chapter_hi {
                for candidate in ch_max.saturating_add(1)..=hi {
                    if in_other_block(candidate, req_chapter, req.family) {
                        break;
                    }
                    if !taken.contains(&candidate)
                        && !retired.contains(&candidate)
                        && !round_proposals.contains(&candidate)
                    {
                        chosen = Some(candidate);
                        break;
                    }
                }
            }
        }
        if chosen.is_none() {
            for cand in lo..=hi {
                if !taken.contains(&cand)
                    && !retired.contains(&cand)
                    && !round_proposals.contains(&cand)
                    && !in_any_block(cand)
                {
                    chosen = Some(cand);
                    break;
                }
            }
        }
        // ADR-NUM-refusal-8xx: when allow_8xx_allocation is true and 836..=839 are exhausted,
        // continue at 4-digit FROB-REF-8100..=8999.
        if chosen.is_none() && req.family == 8 && input.allow_8xx_allocation {
            for cand in 8100u16..=8999u16 {
                if !taken.contains(&cand) && !retired.contains(&cand) {
                    chosen = Some(cand);
                    break;
                }
            }
        }

        if let Some(num) = chosen {
            taken.insert(num);
            if !norm_cond.is_empty() {
                condition_to_code.insert(norm_cond, num);
            }
            req.assigned = Some(format_refusal_code(num));
            req.status = AllocationStatus::Assigned;
            req.reason = None;
        } else {
            req.assigned = None;
            req.status = AllocationStatus::Requested;
            req.reason = Some("range-full".into());
        }
        out.push(req);
    }

    Ok(AllocationOutcome { results: out })
}

// ============================================================================
// Interim List & Ledger Validation (Rule 7)
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InterimEntry {
    pub slug: String,
    pub codes: Vec<String>,
    pub detail: String,
    pub pointer: String,
    pub owner: String,
    pub status: AllocationStatus,
}

impl InterimEntry {
    #[must_use]
    pub fn format_line(&self) -> String {
        let code_col = if self.codes.is_empty() {
            "none".to_owned()
        } else {
            self.codes.join(", ")
        };
        format!(
            "{} | {} | detail={} | pointer={} | owner={} | status={}",
            self.slug, code_col, self.detail, self.pointer, self.owner, self.status
        )
    }
}

/// Validate `ledger` against `reg` (Rules 5, 6, 7, 8) and produce the Rule-7 interim list.
pub fn validate_ledger_and_interims(
    reg: &RefusalRegistry,
    ledger: &AllocationLedger,
) -> Result<Vec<InterimEntry>, AllocationError> {
    let retired_nums = ledger.retired_numbers();
    let mut assigned_owners: BTreeMap<u16, String> = BTreeMap::new();
    let mut interim_pairs: BTreeMap<(String, String), String> = BTreeMap::new();
    let mut interims = Vec::new();

    for req in &ledger.requests {
        if req.status == AllocationStatus::Assigned && req.assigned.is_none() {
            return Err(AllocationError::InvalidField {
                context: format!("request `{}`.assigned", req.request),
                detail: "assigned request requires a code".into(),
            });
        }
        if let Some(num) = req.assigned_number() {
            let code = format_refusal_code(num);
            if retired_nums.contains(&num) {
                return Err(AllocationError::RetiredCodeReused {
                    code,
                    context: format!("ledger request `{}`", req.request),
                });
            }
            if let Some(prev) = assigned_owners.insert(num, req.request.clone()) {
                return Err(AllocationError::DuplicateAssignment {
                    code,
                    first: prev,
                    second: req.request.clone(),
                });
            }
            if let Some((lo, hi)) = parse_subfamily_range(req.family, req.subfamily.as_deref()) {
                let in_range =
                    (lo..=hi).contains(&num) || (req.family == 8 && (8100..=8999).contains(&num));
                if !in_range {
                    return Err(AllocationError::InvalidField {
                        context: format!("request `{}`.assigned", req.request),
                        detail: format!("{code} lies outside expected range {lo}..={hi}"),
                    });
                }
            }
        }

        // Rule 7: every interim code must exist in the registry (never proposed/held/absent).
        for code in req.interim.codes() {
            if reg
                .lookup(code, &format!("interim of `{}`", req.request))
                .is_err()
            {
                return Err(AllocationError::InvalidInterim {
                    slug: req.request.clone(),
                    value: code.clone(),
                    reason: "interim code is not registered in registries/refusals.toml".into(),
                });
            }
        }

        let is_unsettled = matches!(
            req.status,
            AllocationStatus::Requested | AllocationStatus::Held
        );
        if is_unsettled {
            let detail = req.effective_detail().to_owned();
            for code in req.interim.codes() {
                let key = (code.clone(), detail.clone());
                if let Some(first) = interim_pairs.insert(key, req.request.clone()) {
                    return Err(AllocationError::DuplicateInterimPair {
                        code: code.clone(),
                        detail,
                        first,
                        second: req.request.clone(),
                    });
                }
            }
            interims.push(InterimEntry {
                slug: req.request.clone(),
                codes: req.interim.codes().to_vec(),
                detail,
                pointer: req.interim_pointer.clone().unwrap_or_else(|| "-".into()),
                owner: req.requested_by.clone(),
                status: req.status,
            });
        }
    }

    Ok(interims)
}

// ============================================================================
// Cross-Chapter Checks 1–6 (N35-33)
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrossChapterReport {
    pub registry_rows: usize,
    pub printed_target: usize,
    pub assigned_count: usize,
    pub merged_count: usize,
    pub pending_chapters: Vec<(String, Milestone, usize)>,
    pub pending_assigned: Vec<(String, String)>,
    pub interims: Vec<InterimEntry>,
}

/// Run all six N35-33 cross-chapter checks on `(reg, ledger)`.
///
/// During development (`None`), missing rows are reported as pending.
/// At a milestone exit, a missing row is an error once its registry is due.
pub fn run_cross_chapter_checks(
    reg: &RefusalRegistry,
    ledger: &AllocationLedger,
    milestone_exit: Option<Milestone>,
) -> Result<CrossChapterReport, AllocationError> {
    let interims = validate_ledger_and_interims(reg, ledger)?;
    let retired_nums = ledger.retired_numbers();
    let printed = printed_plan_codes();

    // Check 1: Codes unique, sorted numerically by family then number, none retired.
    let mut prev_sort_key: Option<(u8, u16)> = None;
    let mut seen_nums = BTreeSet::new();
    let mut rows_by_code: BTreeMap<&str, &RefusalRow> = BTreeMap::new();
    for row in reg.rows() {
        if !seen_nums.insert(row.number) {
            return Err(AllocationError::RegistryOrderOrDuplicate {
                detail: format!("duplicate refusal code `{}`", row.code),
            });
        }
        if retired_nums.contains(&row.number) {
            return Err(AllocationError::RetiredCodeReused {
                code: row.code.clone(),
                context: "registries/refusals.toml".into(),
            });
        }
        let sk = (code_family(row.number), row.number);
        if let Some(prev) = prev_sort_key
            && sk < prev
        {
            return Err(AllocationError::RegistryOrderOrDuplicate {
                detail: format!(
                    "code `{}` is out of family-then-numeric sort order",
                    row.code
                ),
            });
        }
        prev_sort_key = Some(sk);
        rows_by_code.insert(row.code.as_str(), row);

        // Check 2: defined_in, meaning, >= 2 repairs (or explicit rationale), raised_by.
        if row.defined_in.trim().is_empty() {
            return Err(AllocationError::RowMetadataIncomplete {
                code: row.code.clone(),
                detail: "empty `defined_in`".into(),
            });
        }
        if row.meaning.trim().is_empty() {
            return Err(AllocationError::RowMetadataIncomplete {
                code: row.code.clone(),
                detail: "empty `meaning`".into(),
            });
        }
        if row.raised_by.is_empty() {
            return Err(AllocationError::RowMetadataIncomplete {
                code: row.code.clone(),
                detail: "empty `raised_by` (name a bead or `unbound`)".into(),
            });
        }
        if row.repairs.is_empty() {
            return Err(AllocationError::RowMetadataIncomplete {
                code: row.code.clone(),
                detail: "empty `repairs`".into(),
            });
        }

        // Check 3: Exit equals sub-family / exception rule.
        let (expected_exit, rule_name) =
            reg.exit_rule().expected_exit(row.number).map_err(|e| {
                AllocationError::ExitMismatch {
                    code: row.code.clone(),
                    detail: e.to_string(),
                }
            })?;
        if row.exit != expected_exit {
            return Err(AllocationError::ExitMismatch {
                code: row.code.clone(),
                detail: format!(
                    "observed exit {}, expected {expected_exit} ({rule_name})",
                    row.exit
                ),
            });
        }
    }

    // Check 4: Ledger <-> Registry correspondence.
    let mut assigned_nums: BTreeMap<u16, &AllocationRequest> = BTreeMap::new();
    let mut assigned_count = 0usize;
    let mut merged_count = 0usize;
    let mut pending_assigned = Vec::new();
    for req in &ledger.requests {
        match req.status {
            AllocationStatus::Assigned => {
                assigned_count += 1;
                if let Some(num) = req.assigned_number() {
                    assigned_nums.insert(num, req);
                    if !seen_nums.contains(&num) {
                        let chapter = parse_defined_in_key(&req.defined_in).0;
                        let owner = format!("N{chapter}-REF");
                        let due = registry_due(chapter)?;
                        if milestone_exit.is_some_and(|current| current >= due) {
                            return Err(AllocationError::LedgerRegistryMismatch {
                                code: format_refusal_code(num),
                                detail: format!(
                                    "assigned ledger row `{}` is missing from {owner}, due {due}",
                                    req.request
                                ),
                            });
                        }
                        pending_assigned.push((format_refusal_code(num), owner));
                    }
                }
            }
            AllocationStatus::Merged => merged_count += 1,
            _ => {}
        }
    }
    for row in reg.rows() {
        if !printed.contains_key(&row.number) && !assigned_nums.contains_key(&row.number) {
            return Err(AllocationError::LedgerRegistryMismatch {
                code: row.code.clone(),
                detail: "registry row outside the printed 294 has no assigned row in registries/refusal_allocations.toml".into(),
            });
        }
    }

    // Check 5: Neighbour pairs — symmetric `related` when both sides exist in `reg`,
    // and distinct raising-site sentences on every declared `[[neighbour_pair]]`.
    for row in reg.rows() {
        for rel in &row.related {
            if let Some(other) = rows_by_code.get(rel.as_str()) {
                let back = other
                    .related
                    .iter()
                    .any(|c| refusal_code_token_eq(c, &row.code));
                if !back {
                    return Err(AllocationError::NeighbourPairError {
                        left: row.code.clone(),
                        right: other.code.clone(),
                        detail: format!(
                            "`{}` lists `{}` in `related` but `{}` does not list `{}`",
                            row.code, other.code, other.code, row.code
                        ),
                    });
                }
            }
        }
    }
    for pair in &ledger.neighbour_pairs {
        if let (Some(left), Some(right)) = (
            rows_by_code.get(pair.left.as_str()),
            rows_by_code.get(pair.right.as_str()),
        ) && !(left.related.contains(&pair.right) && right.related.contains(&pair.left))
        {
            return Err(AllocationError::NeighbourPairError {
                left: pair.left.clone(),
                right: pair.right.clone(),
                detail: "both registered codes must list the other in `related`".into(),
            });
        }
        let ls = pair.left_site.trim();
        let rs = pair.right_site.trim();
        if ls.is_empty() || rs.is_empty() {
            return Err(AllocationError::NeighbourPairError {
                left: pair.left.clone(),
                right: pair.right.clone(),
                detail: "neighbour pair must carry one non-empty raising-site sentence per side"
                    .into(),
            });
        }
        if normalize_condition(ls) == normalize_condition(rs) {
            return Err(AllocationError::NeighbourPairError {
                left: pair.left.clone(),
                right: pair.right.clone(),
                detail: "both sides of neighbour pair name the exact same raising site".into(),
            });
        }
    }

    // Check 6: Printed-294 count check by chapter, staged by milestone.
    let mut missing_by_chapter: BTreeMap<u32, Vec<String>> = BTreeMap::new();
    for (&number, &chapter) in printed {
        if !seen_nums.contains(&number) {
            missing_by_chapter
                .entry(chapter)
                .or_default()
                .push(format_refusal_code(number));
        }
    }
    let mut pending_chapters = Vec::new();
    for (chapter, missing) in missing_by_chapter {
        let bead = format!("N{chapter}-REF");
        let due = registry_due(chapter)?;
        if milestone_exit.is_some_and(|current| current >= due) {
            return Err(AllocationError::MilestoneCountShortfall {
                bead,
                detail: format!("due {due}, missing {}", missing.join(", ")),
            });
        }
        pending_chapters.push((bead, due, missing.len()));
    }

    Ok(CrossChapterReport {
        registry_rows: reg.rows().len(),
        printed_target: PRINTED_ACTIVE_CODE_COUNT,
        assigned_count,
        merged_count,
        pending_chapters,
        pending_assigned,
        interims,
    })
}

// ============================================================================
// Deterministic Markdown Printout Generator (`docs/generated/refusals.md`)
// ============================================================================

fn md_escape_cell(s: &str) -> String {
    s.replace('|', "\\|").replace('\n', " ")
}

/// Pure D0 Markdown printout generator for `docs/generated/refusals.md` (`N35-33`, `ADR-NUM-refusal-8xx`).
#[must_use]
pub fn render_refusals_markdown(reg: &RefusalRegistry, ledger: &AllocationLedger) -> String {
    let mut out = String::new();
    out.push_str(REFUSALS_MD_MARKER);
    out.push_str("\n\n# FrankenRobotics Refusal Code Registry\n\n");
    out.push_str(
        "Generated from `registries/refusals.toml` (`frob.refusals/1`) and `registries/refusal_allocations.toml` (`frob.refalloc/1`).\n",
    );
    out.push_str("Sorted by family (`1xx`..`9xx`), then numerically; codes are compared as whole tokens (`FROB-REF-[1-9][0-9]{2,3}`).\n\n");

    // Family capacity table (ADR-NUM-refusal-8xx).
    let mut occupied = ledger.retired_numbers();
    occupied.extend(printed_plan_codes().keys().copied());
    occupied.extend(reg.rows().iter().map(|row| row.number));
    occupied.extend(
        ledger
            .requests
            .iter()
            .filter(|req| req.status == AllocationStatus::Assigned)
            .filter_map(AllocationRequest::assigned_number),
    );
    out.push_str("## Family Capacity Table\n\n");
    out.push_str(
        "| Family | Scope | Default Exit | Printed (Plan) | Live Rows | Open Requests | Status |\n",
    );
    out.push_str("|---|---|---:|---:|---:|---:|---|\n");
    for &(fam, label, def_exit) in FAMILY_LABELS {
        let printed_in_fam = printed_plan_codes()
            .keys()
            .filter(|&&n| code_family(n) == fam)
            .count();
        let live_in_fam = reg
            .rows()
            .iter()
            .filter(|r| code_family(r.number) == fam)
            .count();
        let open_in_fam = ledger
            .requests
            .iter()
            .filter(|r| {
                r.family == fam
                    && matches!(
                        r.status,
                        AllocationStatus::Requested | AllocationStatus::Held
                    )
            })
            .count();
        let base = u16::from(fam) * 100;
        let free_3digit = 99 - occupied.range(base + 1..=base + 99).count();
        let status = if fam == 8 || open_in_fam > free_3digit {
            format!(
                "over capacity ({free_3digit} free 3-digit; continues at FROB-REF-{fam}100 per ADR-NUM-refusal-8xx)"
            )
        } else {
            format!("ok ({free_3digit} free 3-digit)")
        };
        out.push_str(&format!(
            "| `{fam}xx` | {label} | {def_exit} | {printed_in_fam} | {live_in_fam} | {open_in_fam} | {status} |\n"
        ));
    }
    out.push('\n');

    // Per-family tables sorted by family, then numerically.
    for &(fam, label, _) in FAMILY_LABELS {
        let mut fam_rows: Vec<&RefusalRow> = reg
            .rows()
            .iter()
            .filter(|r| code_family(r.number) == fam)
            .collect();
        fam_rows.sort_by_key(|r| r.number);

        out.push_str(&format!("## Family `{fam}xx` — {label}\n\n"));
        if fam_rows.is_empty() {
            out.push_str("_No rows registered yet at current milestone._\n\n");
            continue;
        }
        out.push_str("| Code | Meaning | Defined in | Exit |\n");
        out.push_str("|---|---|---|---:|\n");
        for r in fam_rows {
            out.push_str(&format!(
                "| `{}` | {} | {} | {} |\n",
                r.code,
                md_escape_cell(&r.meaning),
                md_escape_cell(&r.defined_in),
                r.exit
            ));
        }
        out.push('\n');
    }

    // Unsettled interim list (Rule 7).
    out.push_str("## Unsettled Allocation Requests & Interim Map\n\n");
    out.push_str("| Request | Interim Code(s) | Detail | Pointer | Owner | Status |\n");
    out.push_str("|---|---|---|---|---|---|\n");
    for req in &ledger.requests {
        if matches!(
            req.status,
            AllocationStatus::Requested | AllocationStatus::Held
        ) {
            out.push_str(&format!(
                "| `{}` | `{}` | `{}` | `{}` | `{}` | `{}` |\n",
                req.request,
                req.interim.display(),
                md_escape_cell(req.effective_detail()),
                md_escape_cell(req.interim_pointer.as_deref().unwrap_or("-")),
                req.requested_by,
                req.status.as_str()
            ));
        }
    }
    out.push('\n');

    // Retired codes.
    out.push_str("## Retired Codes (Never Reused)\n\n");
    out.push_str("| Code | Replaced By | Reason |\n");
    out.push_str("|---|---|---|\n");
    for r in &ledger.retired {
        out.push_str(&format!(
            "| `{}` | `{}` | {} |\n",
            r.code,
            r.replaced_by,
            md_escape_cell(&r.reason)
        ));
    }
    out.push('\n');

    // Neighbour pairs.
    out.push_str("## Cross-Chapter Neighbour Pairs\n\n");
    out.push_str("| Left | Left Raising Site | Right | Right Raising Site |\n");
    out.push_str("|---|---|---|---|\n");
    for p in &ledger.neighbour_pairs {
        out.push_str(&format!(
            "| `{}` | {} | `{}` | {} |\n",
            p.left,
            md_escape_cell(&p.left_site),
            p.right,
            md_escape_cell(&p.right_site)
        ));
    }

    out
}
