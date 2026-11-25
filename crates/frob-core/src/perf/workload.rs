//! Reference workload records (`frob.workload/1`), row-capacity rule, strict parser,
//! and canonical identities (`N36-02`, §36.2, §36.3, §16.6, KA-815).
//!
//! This module is the single parser and record owner for `registries/workloads.toml`
//! across the workspace. Run verbs (`N36-33`), memory estimators (`BD-219`), capacity
//! tables (`N16a-14`), target registers (`N36-01`), comparator harnesses (`BD-039`),
//! and conformance lints (`frob_conformance::perf::workloads`) all read through this
//! API with zero duplicate parsers.
//!
//! Determinism class D0; prints nothing (rule L-7); zero `unsafe`.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::fs;
use std::path::Path;
use std::str::FromStr;

use crate::canon::{self, CanonError, CanonInt, Canonical, Value as CanonValue};
use crate::id::{Blake3, CanonicalBytes, IdError, SchemaName};
use crate::registry::toml::{self, Fields, Spanned, Table, TomlError, Value};

/// Schema identifier and BLAKE3 derive-key domain for reference workload records.
pub const WORKLOAD_SCHEMA: &str = "frob.workload/1";

/// Schema identifier and BLAKE3 derive-key domain for comparator setting records.
pub const SETTING_SCHEMA: &str = "frob.workload.setting/1";

/// Schema identifier and BLAKE3 derive-key domain for the full workload registry snapshot.
pub const WORKLOAD_REGISTRY_SCHEMA: &str = "frob.workload.registry/1";

/// Rounding granule of the workspace row-capacity rule (§16.6, §36.3, KA-815).
pub const ROW_CAP_GRANULE: u32 = 8;

/// Degrees of freedom contributed by a 3D floating base.
pub const FLOATING_BASE_DOF: u32 = 6;

/// Reference control period of 20 ms (`1/50` s, i.e. 50 Hz control).
pub const CONTROL_PERIOD_20MS: Rational = Rational { num: 1, den: 50 };

/// Unrounded constraint row count: `4 * contact_cap + 2 * actuated`.
///
/// Four rows per contact (normal, two tangential, spin) plus one limit row and
/// one stiction/backlash row per actuated joint (KA-815). Returns `None` on `u32`
/// arithmetic overflow.
#[must_use]
pub const fn checked_row_cap_raw(contact_cap: u32, actuated: u32) -> Option<u32> {
    let Some(contact_rows) = contact_cap.checked_mul(4) else {
        return None;
    };
    let Some(joint_rows) = actuated.checked_mul(2) else {
        return None;
    };
    contact_rows.checked_add(joint_rows)
}

/// Unrounded constraint row count: `4 * contact_cap + 2 * actuated`.
///
/// Uses saturating arithmetic to guarantee panic-free evaluation; use
/// [`checked_row_cap_raw`] when overflow must be detected explicitly.
#[must_use]
pub const fn row_cap_raw(contact_cap: u32, actuated: u32) -> u32 {
    contact_cap
        .saturating_mul(4)
        .saturating_add(actuated.saturating_mul(2))
}

/// Workspace row-capacity rule: `round_up(4 * contact_cap + 2 * actuated, 8)`.
///
/// Returns `None` if either the intermediate sum or the rounding overflows `u32`.
#[must_use]
pub const fn checked_row_cap(contact_cap: u32, actuated: u32) -> Option<u32> {
    let Some(raw) = checked_row_cap_raw(contact_cap, actuated) else {
        return None;
    };
    let Some(bumped) = raw.checked_add(ROW_CAP_GRANULE - 1) else {
        return None;
    };
    Some((bumped / ROW_CAP_GRANULE) * ROW_CAP_GRANULE)
}

/// Workspace row-capacity rule: `round_up(4 * contact_cap + 2 * actuated, 8)` (KA-815).
///
/// Confirmed across all six rows of the dynamics capacity table:
/// `(32, 12) -> 152`, `(32, 18) -> 168`, `(64, 29) -> 320`,
/// `(16, 4) -> 72`, `(32, 16) -> 160`, `(64, 4) -> 264`.
///
/// Uses saturating rounding (capped at the largest `u32` multiple of 8) to remain
/// panic-free; use [`checked_row_cap`] when overflow must be detected explicitly.
#[must_use]
pub const fn row_cap(contact_cap: u32, actuated: u32) -> u32 {
    match checked_row_cap(contact_cap, actuated) {
        Some(v) => v,
        None => (u32::MAX / ROW_CAP_GRANULE) * ROW_CAP_GRANULE,
    }
}

const fn gcd_u64(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

/// Strictly positive, already-reduced exact rational `num / den` (`num > 0`, `den > 0`,
/// `gcd(num, den) == 1`). Never silently reduces unreduced inputs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Rational {
    num: u64,
    den: u64,
}

impl Rational {
    /// Construct a reduced positive rational, refusing zero numerator, zero denominator,
    /// or unreduced `(num, den)` pairs (`gcd(num, den) != 1`).
    pub fn new(num: u64, den: u64) -> Result<Self, WorkloadError> {
        if num == 0 {
            return Err(WorkloadError::InvalidField {
                table: "rational".to_owned(),
                field: "numerator".to_owned(),
                reason: "numerator must be strictly positive".to_owned(),
            });
        }
        if den == 0 {
            return Err(WorkloadError::InvalidField {
                table: "rational".to_owned(),
                field: "denominator".to_owned(),
                reason: "denominator must be strictly positive".to_owned(),
            });
        }
        if gcd_u64(num, den) != 1 {
            return Err(WorkloadError::InvalidField {
                table: "rational".to_owned(),
                field: "value".to_owned(),
                reason: format!("rational `{num}/{den}` must already be reduced (gcd != 1)"),
            });
        }
        Ok(Self { num, den })
    }

    /// Parse a strict positive reduced rational string `"p/q"` (or bare integer `"p"`).
    ///
    /// Rejects whitespace, signs, leading zeros, zero values, and unreduced fractions.
    pub fn parse(s: &str) -> Result<Self, WorkloadError> {
        if s.is_empty() {
            return Err(WorkloadError::InvalidField {
                table: "rational".to_owned(),
                field: "value".to_owned(),
                reason: "empty rational string".to_owned(),
            });
        }
        match s.split_once('/') {
            Some((num_s, den_s)) => {
                if den_s.contains('/') {
                    return Err(WorkloadError::InvalidField {
                        table: "rational".to_owned(),
                        field: "value".to_owned(),
                        reason: format!("malformed rational `{s}`: multiple `/` separators"),
                    });
                }
                let num = parse_positive_decimal(num_s, s)?;
                let den = parse_positive_decimal(den_s, s)?;
                Self::new(num, den)
            }
            None => {
                let num = parse_positive_decimal(s, s)?;
                Ok(Self { num, den: 1 })
            }
        }
    }

    #[must_use]
    pub const fn num(self) -> u64 {
        self.num
    }

    #[must_use]
    pub const fn den(self) -> u64 {
        self.den
    }

    /// True iff this reduced rational is an integer (`den == 1`).
    #[must_use]
    pub const fn is_integer(self) -> bool {
        self.den == 1
    }

    /// Exact reduced product `self * rhs`, cross-reducing before multiplication to avoid
    /// spurious overflow. Returns `None` on `u64` overflow.
    #[must_use]
    pub const fn checked_mul(self, rhs: Self) -> Option<Self> {
        let g1 = gcd_u64(self.num, rhs.den);
        let g2 = gcd_u64(rhs.num, self.den);
        let Some(num) = (self.num / g1).checked_mul(rhs.num / g2) else {
            return None;
        };
        let Some(den) = (self.den / g2).checked_mul(rhs.den / g1) else {
            return None;
        };
        Some(Self { num, den })
    }

    /// Render in explicit `"p/q"` form (including `"/1"` for integers, matching TOML files).
    #[must_use]
    pub fn to_slash_string(self) -> String {
        format!("{}/{}", self.num, self.den)
    }
}

fn parse_positive_decimal(part: &str, full: &str) -> Result<u64, WorkloadError> {
    if part.is_empty()
        || !part.bytes().all(|b| b.is_ascii_digit())
        || part.starts_with('0')
    {
        return Err(WorkloadError::InvalidField {
            table: "rational".to_owned(),
            field: "value".to_owned(),
            reason: format!(
                "invalid positive decimal component `{part}` in rational `{full}`"
            ),
        });
    }
    part.parse::<u64>().map_err(|_| WorkloadError::InvalidField {
        table: "rational".to_owned(),
        field: "value".to_owned(),
        reason: format!("integer component `{part}` out of u64 range in `{full}`"),
    })
}

impl fmt::Display for Rational {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.num, self.den)
    }
}

impl FromStr for Rational {
    type Err = WorkloadError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl Canonical for Rational {
    fn write(&self, w: CanonValue<'_>) -> Result<(), CanonError> {
        let p = CanonInt::from_u64(self.num);
        let q = CanonInt::from_u64(self.den);
        w.rational_slash(&p, &q)
    }
}

/// Content-addressed BLAKE3 identity of a single canonical `frob.workload/1` record.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WorkloadId(Blake3);

impl WorkloadId {
    /// Text prefix when serialized as a `b3:` content hash reference.
    pub const PREFIX: &'static str = "b3:";

    #[must_use]
    pub const fn from_digest(digest: Blake3) -> Self {
        Self(digest)
    }

    #[must_use]
    pub const fn digest(self) -> Blake3 {
        self.0
    }

    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; Blake3::LEN] {
        self.0.as_bytes()
    }

    /// Lower-case 64-hex digest without prefix.
    #[must_use]
    pub fn to_hex(&self) -> String {
        self.0.to_hex()
    }

    /// Parse 64 bare lower-case hex digits.
    pub fn from_hex(s: &str) -> Result<Self, IdError> {
        Ok(Self(Blake3::from_hex(s)?))
    }

    /// Render as `b3:<64 lower-case hex digits>`.
    #[must_use]
    pub fn to_b3_string(&self) -> String {
        format!("{}{}", Self::PREFIX, self.0)
    }

    /// Parse `b3:<64 lower-case hex digits>`.
    pub fn parse(s: &str) -> Result<Self, IdError> {
        let hex = s.strip_prefix(Self::PREFIX).ok_or_else(|| IdError::InvalidPrefix {
            expected: Self::PREFIX,
            input: s.to_owned(),
        })?;
        Self::from_hex(hex)
    }
}

impl fmt::Display for WorkloadId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", Self::PREFIX, self.0)
    }
}

impl fmt::Debug for WorkloadId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "WorkloadId({}{})", Self::PREFIX, self.0)
    }
}

impl FromStr for WorkloadId {
    type Err = IdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl Canonical for WorkloadId {
    fn write(&self, w: CanonValue<'_>) -> Result<(), CanonError> {
        w.str(&self.to_b3_string());
        Ok(())
    }
}

/// Content-addressed BLAKE3 identity of a single canonical comparator `[[setting]]` record.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SettingId(Blake3);

impl SettingId {
    /// Text prefix matching `comparators.toml` `configurations.{matched,native}` references.
    pub const PREFIX: &'static str = "b3:";

    #[must_use]
    pub const fn from_digest(digest: Blake3) -> Self {
        Self(digest)
    }

    #[must_use]
    pub const fn digest(self) -> Blake3 {
        self.0
    }

    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; Blake3::LEN] {
        self.0.as_bytes()
    }

    /// Lower-case 64-hex digest without prefix.
    #[must_use]
    pub fn to_hex(&self) -> String {
        self.0.to_hex()
    }

    /// Parse 64 bare lower-case hex digits.
    pub fn from_hex(s: &str) -> Result<Self, IdError> {
        Ok(Self(Blake3::from_hex(s)?))
    }

    /// Render as `b3:<64 lower-case hex digits>`.
    #[must_use]
    pub fn to_b3_string(&self) -> String {
        format!("{}{}", Self::PREFIX, self.0)
    }

    /// Parse `b3:<64 lower-case hex digits>`.
    pub fn parse(s: &str) -> Result<Self, IdError> {
        let hex = s.strip_prefix(Self::PREFIX).ok_or_else(|| IdError::InvalidPrefix {
            expected: Self::PREFIX,
            input: s.to_owned(),
        })?;
        Self::from_hex(hex)
    }
}

impl fmt::Display for SettingId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", Self::PREFIX, self.0)
    }
}

impl fmt::Debug for SettingId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SettingId({}{})", Self::PREFIX, self.0)
    }
}

impl FromStr for SettingId {
    type Err = IdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl Canonical for SettingId {
    fn write(&self, w: CanonValue<'_>) -> Result<(), CanonError> {
        w.str(&self.to_b3_string());
        Ok(())
    }
}

/// Explicit record of a deliberate departure from `row_cap(contact_cap, actuated)`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RowCapOverride {
    /// The un-overridden rule value `row_cap(contact_cap, actuated)`.
    pub rule: u32,
    /// Written rationale explaining why `row_cap` differs from `rule`.
    pub reason: String,
}

impl Canonical for RowCapOverride {
    fn write(&self, w: CanonValue<'_>) -> Result<(), CanonError> {
        w.object(|o| {
            o.int53("rule", i64::from(self.rule))?;
            o.str("reason", &self.reason)?;
            Ok(())
        })
    }
}

/// Lifecycle state of a reference workload record (`OPEN-SPARK-3`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WorkloadStatus {
    Default,
    Frozen,
}

impl WorkloadStatus {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Frozen => "frozen",
        }
    }

    pub fn parse(s: &str) -> Result<Self, WorkloadError> {
        match s {
            "default" => Ok(Self::Default),
            "frozen" => Ok(Self::Frozen),
            other => Err(WorkloadError::InvalidField {
                table: "workload".to_owned(),
                field: "status".to_owned(),
                reason: format!("expected `default` or `frozen`, found `{other}`"),
            }),
        }
    }
}

impl fmt::Display for WorkloadStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Canonical for WorkloadStatus {
    fn write(&self, w: CanonValue<'_>) -> Result<(), CanonError> {
        w.str(self.as_str());
        Ok(())
    }
}

/// One pinned reference workload record (`frob.workload/1`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Workload {
    pub name: String,
    pub robot: String,
    pub actuated: u32,
    pub dof: u32,
    pub bodies: u32,
    pub foot_samples: Option<u32>,
    pub contact_cap: u32,
    pub row_cap: u32,
    pub row_cap_override: Option<RowCapOverride>,
    pub h_s: Rational,
    pub newton_iterations: u32,
    pub warm_start: bool,
    pub decimation: Rational,
    pub worlds: u32,
    pub grounds: Vec<String>,
    pub status: WorkloadStatus,
    pub adr: Option<String>,
}

impl Workload {
    /// Canonical JSON bytes of this workload record via `frob_core::canon`.
    pub fn canonical_bytes(&self) -> Result<CanonicalBytes, CanonError> {
        canon::to_canonical(self)
    }

    /// Domain-separated BLAKE3 identity under `"frob.workload/1"`.
    pub fn id(&self) -> Result<WorkloadId, CanonError> {
        let schema = workload_schema_name();
        let digest = canon::canonical_identity(&schema, self)?;
        Ok(WorkloadId::from_digest(digest))
    }

    /// Unrounded row count `4 * contact_cap + 2 * actuated` (`None` on `u32` overflow).
    #[must_use]
    pub const fn row_cap_raw(&self) -> Option<u32> {
        checked_row_cap_raw(self.contact_cap, self.actuated)
    }

    /// Rule-computed row capacity `round_up(4 * contact_cap + 2 * actuated, 8)`.
    #[must_use]
    pub const fn row_cap_rule(&self) -> Option<u32> {
        checked_row_cap(self.contact_cap, self.actuated)
    }

    /// Control period `h_s * decimation` in seconds (`None` on `u64` overflow).
    #[must_use]
    pub const fn control_period(&self) -> Option<Rational> {
        self.h_s.checked_mul(self.decimation)
    }
}

impl Canonical for Workload {
    fn write(&self, w: CanonValue<'_>) -> Result<(), CanonError> {
        w.object(|o| {
            o.str("schema", WORKLOAD_SCHEMA)?;
            o.str("name", &self.name)?;
            o.str("robot", &self.robot)?;
            o.int53("actuated", i64::from(self.actuated))?;
            o.int53("dof", i64::from(self.dof))?;
            o.int53("bodies", i64::from(self.bodies))?;
            if let Some(samples) = self.foot_samples {
                o.int53("foot_samples", i64::from(samples))?;
            }
            o.int53("contact_cap", i64::from(self.contact_cap))?;
            o.int53("row_cap", i64::from(self.row_cap))?;
            if let Some(ref ov) = self.row_cap_override {
                o.member_term("row_cap_override", ov)?;
            }
            o.member_term("h_s", &self.h_s)?;
            o.int53("newton_iterations", i64::from(self.newton_iterations))?;
            o.bool("warm_start", self.warm_start)?;
            o.member_term("decimation", &self.decimation)?;
            o.int53("worlds", i64::from(self.worlds))?;
            o.array("grounds", |a| {
                for g in &self.grounds {
                    a.push_str(g);
                }
                Ok(())
            })?;
            o.member_term("status", &self.status)?;
            if let Some(ref adr) = self.adr {
                o.str("adr", adr)?;
            }
            Ok(())
        })
    }
}

/// Kind of comparator step setting (`§36.2`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SettingKind {
    /// Incumbent run at our `h_s`, `decimation`, and iteration count where admitted.
    Matched,
    /// Incumbent run at its own recommended step for the task, with source citation.
    Native,
}

impl SettingKind {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Matched => "matched",
            Self::Native => "native",
        }
    }

    pub fn parse(s: &str) -> Result<Self, WorkloadError> {
        match s {
            "matched" => Ok(Self::Matched),
            "native" => Ok(Self::Native),
            other => Err(WorkloadError::InvalidField {
                table: "setting".to_owned(),
                field: "kind".to_owned(),
                reason: format!("expected `matched` or `native`, found `{other}`"),
            }),
        }
    }
}

impl fmt::Display for SettingKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Canonical for SettingKind {
    fn write(&self, w: CanonValue<'_>) -> Result<(), CanonError> {
        w.str(self.as_str());
        Ok(())
    }
}

/// One comparator step setting record (`[[setting]]` in `registries/workloads.toml`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComparatorSetting {
    pub comparator: String,
    pub workload: String,
    pub kind: SettingKind,
    pub h_s: Rational,
    pub decimation: Rational,
    pub iterations: u32,
    pub source: Option<String>,
    pub difference: Option<String>,
}

impl ComparatorSetting {
    /// Canonical JSON bytes of this setting record via `frob_core::canon`.
    pub fn canonical_bytes(&self) -> Result<CanonicalBytes, CanonError> {
        canon::to_canonical(self)
    }

    /// Domain-separated BLAKE3 content hash under `"frob.workload.setting/1"`.
    pub fn id(&self) -> Result<SettingId, CanonError> {
        let schema = setting_schema_name();
        let digest = canon::canonical_identity(&schema, self)?;
        Ok(SettingId::from_digest(digest))
    }
}

impl Canonical for ComparatorSetting {
    fn write(&self, w: CanonValue<'_>) -> Result<(), CanonError> {
        w.object(|o| {
            o.str("schema", SETTING_SCHEMA)?;
            o.str("comparator", &self.comparator)?;
            o.str("workload", &self.workload)?;
            o.member_term("kind", &self.kind)?;
            o.member_term("h_s", &self.h_s)?;
            o.member_term("decimation", &self.decimation)?;
            o.int53("iterations", i64::from(self.iterations))?;
            if let Some(ref src) = self.source {
                o.str("source", src)?;
            }
            if let Some(ref diff) = self.difference {
                o.str("difference", diff)?;
            }
            Ok(())
        })
    }
}

/// Parsed contents of `registries/workloads.toml`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkloadRegistry {
    pub schema: String,
    pub workloads: BTreeMap<String, Workload>,
    pub settings: Vec<ComparatorSetting>,
}

impl WorkloadRegistry {
    /// Parse UTF-8 TOML bytes strictly in the `frob_core::registry::toml` subset.
    pub fn parse_bytes(bytes: &[u8]) -> Result<Self, WorkloadError> {
        parse_bytes(bytes)
    }

    /// Parse a UTF-8 TOML string strictly in the `frob_core::registry::toml` subset.
    pub fn parse_str(text: &str) -> Result<Self, WorkloadError> {
        parse_str(text)
    }

    /// Read and strictly parse a workload registry file from disk.
    pub fn load(path: &Path) -> Result<Self, WorkloadError> {
        load(path)
    }

    /// Look up a workload record by name (e.g. `"Q"` or `"H"`).
    #[must_use]
    pub fn get(&self, name: &str) -> Option<&Workload> {
        self.workloads.get(name)
    }

    /// Compute the [`WorkloadId`] of the named workload, if present.
    pub fn workload_id(&self, name: &str) -> Result<Option<WorkloadId>, CanonError> {
        match self.workloads.get(name) {
            Some(w) => Ok(Some(w.id()?)),
            None => Ok(None),
        }
    }

    /// Borrow all parsed workloads keyed deterministically by name.
    #[must_use]
    pub const fn workloads(&self) -> &BTreeMap<String, Workload> {
        &self.workloads
    }

    /// Borrow all parsed comparator setting records.
    #[must_use]
    pub fn settings(&self) -> &[ComparatorSetting] {
        &self.settings
    }

    /// Look up the setting record for `(comparator, workload, kind)`.
    #[must_use]
    pub fn setting(
        &self,
        comparator: &str,
        workload: &str,
        kind: SettingKind,
    ) -> Option<&ComparatorSetting> {
        self.settings.iter().find(|s| {
            s.comparator == comparator && s.workload == workload && s.kind == kind
        })
    }

    /// Return all setting records targeting `workload`.
    #[must_use]
    pub fn settings_for_workload(&self, workload: &str) -> Vec<&ComparatorSetting> {
        self.settings
            .iter()
            .filter(|s| s.workload == workload)
            .collect()
    }

    /// Distinct comparator names appearing in `[[setting]]` records, sorted.
    #[must_use]
    pub fn comparators(&self) -> BTreeSet<&str> {
        self.settings
            .iter()
            .map(|s| s.comparator.as_str())
            .collect()
    }

    /// Domain-separated BLAKE3 digest over the full canonical registry (sorted workloads
    /// and sorted settings), invariant to TOML key/entry order.
    pub fn registry_digest(&self) -> Result<Blake3, CanonError> {
        let schema = registry_schema_name();
        canon::canonical_identity(&schema, self)
    }
}

impl Canonical for WorkloadRegistry {
    fn write(&self, w: CanonValue<'_>) -> Result<(), CanonError> {
        let mut sorted_settings: Vec<&ComparatorSetting> = self.settings.iter().collect();
        sorted_settings.sort_by(|a, b| {
            a.comparator
                .cmp(&b.comparator)
                .then_with(|| a.workload.cmp(&b.workload))
                .then_with(|| a.kind.cmp(&b.kind))
        });

        w.object(|o| {
            o.str("schema", &self.schema)?;
            o.object("workloads", |wo| {
                for (name, workload) in &self.workloads {
                    wo.member_term(name, workload)?;
                }
                Ok(())
            })?;
            o.array("settings", |sa| {
                for setting in sorted_settings {
                    sa.push_term(setting)?;
                }
                Ok(())
            })?;
            Ok(())
        })
    }
}

/// Errors produced while reading or strictly parsing a workload registry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorkloadError {
    Io {
        path: String,
        message: String,
    },
    Toml(TomlError),
    UnsupportedSchema {
        expected: &'static str,
        found: String,
    },
    InvalidField {
        table: String,
        field: String,
        reason: String,
    },
    Canon(CanonError),
}

impl fmt::Display for WorkloadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io { path, message } => write!(f, "failed to read `{path}`: {message}"),
            Self::Toml(err) => write!(f, "TOML parse error: {err}"),
            Self::UnsupportedSchema { expected, found } => {
                write!(f, "unsupported workload schema `{found}` (expected `{expected}`)")
            }
            Self::InvalidField {
                table,
                field,
                reason,
            } => write!(f, "invalid field `{field}` in `{table}`: {reason}"),
            Self::Canon(err) => write!(f, "canonical encoding error: {err}"),
        }
    }
}

impl std::error::Error for WorkloadError {}

impl From<TomlError> for WorkloadError {
    fn from(err: TomlError) -> Self {
        Self::Toml(err)
    }
}

impl From<CanonError> for WorkloadError {
    fn from(err: CanonError) -> Self {
        Self::Canon(err)
    }
}

/// Strictly parse `registries/workloads.toml` bytes via `frob_core::registry::toml`.
///
/// Refuses unknown fields on the document root, every `[workload.<name>]` table,
/// `row_cap_override`, and every `[[setting]]` table. Semantic invariants (e.g.
/// `dof == 6 + actuated`, row-capacity rule, 20 ms control period, frozen ADR
/// requirement, native source citations) are checked by
/// `frob_conformance::perf::workloads` on top of this parser.
pub fn parse_bytes(bytes: &[u8]) -> Result<WorkloadRegistry, WorkloadError> {
    let root = toml::parse(bytes)?;
    let mut top = Fields::new(root, "workloads.toml");
    let schema_opt = top.opt_str("schema")?;
    let workload_tbl = top.require_table("workload")?;
    let setting_items = top.opt_array("setting")?;
    top.finish()?;

    let schema = match schema_opt {
        Some(s) => {
            if s != WORKLOAD_SCHEMA {
                return Err(WorkloadError::UnsupportedSchema {
                    expected: WORKLOAD_SCHEMA,
                    found: s,
                });
            }
            s
        }
        None => WORKLOAD_SCHEMA.to_owned(),
    };

    let mut workloads = BTreeMap::new();
    for (name, spanned) in workload_tbl.into_inner() {
        let table_label = format!("workload.{name}");
        if name.trim().is_empty() {
            return Err(WorkloadError::InvalidField {
                table: "workload".to_owned(),
                field: name,
                reason: "workload identifier must be non-empty".to_owned(),
            });
        }
        let Value::Table(tbl) = spanned.value else {
            return Err(WorkloadError::InvalidField {
                table: "workload".to_owned(),
                field: name,
                reason: format!("expected table, found {}", spanned.value.type_name()),
            });
        };
        let parsed = parse_workload_table(&name, &table_label, tbl)?;
        workloads.insert(name, parsed);
    }

    let mut settings = Vec::new();
    if let Some(items) = setting_items {
        for (idx, spanned) in items.into_iter().enumerate() {
            let label = format!("setting[{idx}]");
            let Value::Table(tbl) = spanned.value else {
                return Err(WorkloadError::InvalidField {
                    table: "workloads.toml".to_owned(),
                    field: label,
                    reason: format!("expected table, found {}", spanned.value.type_name()),
                });
            };
            settings.push(parse_setting_table(&label, tbl)?);
        }
    }

    Ok(WorkloadRegistry {
        schema,
        workloads,
        settings,
    })
}

/// Strictly parse `registries/workloads.toml` from a UTF-8 string slice.
pub fn parse_str(text: &str) -> Result<WorkloadRegistry, WorkloadError> {
    parse_bytes(text.as_bytes())
}

/// Read and strictly parse a `workloads.toml` file from disk.
pub fn load(path: &Path) -> Result<WorkloadRegistry, WorkloadError> {
    let bytes = fs::read(path).map_err(|err| WorkloadError::Io {
        path: path.display().to_string(),
        message: err.to_string(),
    })?;
    parse_bytes(&bytes)
}

fn parse_workload_table(
    name: &str,
    table_label: &str,
    tbl: Table,
) -> Result<Workload, WorkloadError> {
    let mut f = Fields::new(tbl, table_label);
    let robot_raw = f.require_str("robot")?;
    let actuated_raw = f.require_int("actuated")?;
    let dof_raw = f.require_int("dof")?;
    let bodies_raw = f.require_int("bodies")?;
    let foot_samples_raw = f.opt_int("foot_samples")?;
    let contact_cap_raw = f.require_int("contact_cap")?;
    let row_cap_raw_i = f.require_int("row_cap")?;
    let row_cap_override_tbl = f.opt_table("row_cap_override")?;
    let h_s_raw = f.require_str("h_s")?;
    let newton_iterations_raw = f.require_int("newton_iterations")?;
    let warm_start = f.require_bool("warm_start")?;
    let decimation_raw = f.require_str("decimation")?;
    let worlds_raw = f.require_int("worlds")?;
    let grounds_raw = f.require_array("grounds")?;
    let status_raw = f.require_str("status")?;
    let adr = f.opt_str("adr")?;
    f.finish()?;

    let robot = check_non_empty_str(robot_raw, table_label, "robot")?;
    let actuated = convert_positive_u32(actuated_raw, table_label, "actuated")?;
    let dof = convert_positive_u32(dof_raw, table_label, "dof")?;
    let bodies = convert_positive_u32(bodies_raw, table_label, "bodies")?;
    let foot_samples = match foot_samples_raw {
        Some(v) => Some(convert_positive_u32(v, table_label, "foot_samples")?),
        None => None,
    };
    let contact_cap = convert_positive_u32(contact_cap_raw, table_label, "contact_cap")?;
    let row_cap_val = convert_positive_u32(row_cap_raw_i, table_label, "row_cap")?;

    let row_cap_override = match row_cap_override_tbl {
        Some(ov_tbl) => {
            let ov_label = format!("{table_label}.row_cap_override");
            let mut ov = Fields::new(ov_tbl, &ov_label);
            let rule_raw = ov.require_int("rule")?;
            let reason = ov.require_str("reason")?;
            ov.finish()?;
            let rule = convert_positive_u32(rule_raw, &ov_label, "rule")?;
            Some(RowCapOverride { rule, reason })
        }
        None => None,
    };

    let h_s = Rational::parse(&h_s_raw).map_err(|err| remap_table_field(err, table_label, "h_s"))?;
    let newton_iterations =
        convert_positive_u32(newton_iterations_raw, table_label, "newton_iterations")?;
    let decimation = Rational::parse(&decimation_raw)
        .map_err(|err| remap_table_field(err, table_label, "decimation"))?;
    let worlds = convert_positive_u32(worlds_raw, table_label, "worlds")?;
    let grounds = parse_string_array(&grounds_raw, table_label, "grounds")?;
    let status = WorkloadStatus::parse(&status_raw)
        .map_err(|err| remap_table_field(err, table_label, "status"))?;

    Ok(Workload {
        name: name.to_owned(),
        robot,
        actuated,
        dof,
        bodies,
        foot_samples,
        contact_cap,
        row_cap: row_cap_val,
        row_cap_override,
        h_s,
        newton_iterations,
        warm_start,
        decimation,
        worlds,
        grounds,
        status,
        adr,
    })
}

fn parse_setting_table(label: &str, tbl: Table) -> Result<ComparatorSetting, WorkloadError> {
    let mut f = Fields::new(tbl, label);
    let comparator_raw = f.require_str("comparator")?;
    let workload_raw = f.require_str("workload")?;
    let kind_raw = f.require_str("kind")?;
    let h_s_raw = f.require_str("h_s")?;
    let decimation_raw = f.require_str("decimation")?;
    let iterations_raw = f.require_int("iterations")?;
    let source = f.opt_str("source")?;
    let difference = f.opt_str("difference")?;
    f.finish()?;

    let comparator = check_non_empty_str(comparator_raw, label, "comparator")?;
    let workload = check_non_empty_str(workload_raw, label, "workload")?;
    let kind = SettingKind::parse(&kind_raw).map_err(|err| remap_table_field(err, label, "kind"))?;
    let h_s = Rational::parse(&h_s_raw).map_err(|err| remap_table_field(err, label, "h_s"))?;
    let decimation = Rational::parse(&decimation_raw)
        .map_err(|err| remap_table_field(err, label, "decimation"))?;
    let iterations = convert_positive_u32(iterations_raw, label, "iterations")?;

    Ok(ComparatorSetting {
        comparator,
        workload,
        kind,
        h_s,
        decimation,
        iterations,
        source,
        difference,
    })
}

fn check_non_empty_str(val: String, table: &str, key: &str) -> Result<String, WorkloadError> {
    if val.trim().is_empty() {
        return Err(WorkloadError::InvalidField {
            table: table.to_owned(),
            field: key.to_owned(),
            reason: "string must be non-empty".to_owned(),
        });
    }
    Ok(val)
}

fn convert_positive_u32(raw: i64, table: &str, key: &str) -> Result<u32, WorkloadError> {
    if raw <= 0 {
        return Err(WorkloadError::InvalidField {
            table: table.to_owned(),
            field: key.to_owned(),
            reason: format!("expected positive integer (> 0), found {raw}"),
        });
    }
    u32::try_from(raw).map_err(|_| WorkloadError::InvalidField {
        table: table.to_owned(),
        field: key.to_owned(),
        reason: format!("integer {raw} exceeds u32::MAX"),
    })
}

fn parse_string_array(
    items: &[Spanned<Value>],
    table: &str,
    key: &str,
) -> Result<Vec<String>, WorkloadError> {
    let mut out = Vec::with_capacity(items.len());
    for (idx, item) in items.iter().enumerate() {
        let Some(s) = item.value.as_str() else {
            return Err(WorkloadError::InvalidField {
                table: table.to_owned(),
                field: format!("{key}[{idx}]"),
                reason: format!("expected string, found {}", item.value.type_name()),
            });
        };
        if s.trim().is_empty() {
            return Err(WorkloadError::InvalidField {
                table: table.to_owned(),
                field: format!("{key}[{idx}]"),
                reason: "string element must be non-empty".to_owned(),
            });
        }
        out.push(s.to_owned());
    }
    Ok(out)
}

fn remap_table_field(err: WorkloadError, table: &str, field: &str) -> WorkloadError {
    match err {
        WorkloadError::InvalidField { reason, .. } => WorkloadError::InvalidField {
            table: table.to_owned(),
            field: field.to_owned(),
            reason,
        },
        other => other,
    }
}

fn workload_schema_name() -> SchemaName {
    SchemaName::parse(WORKLOAD_SCHEMA).expect("static WORKLOAD_SCHEMA is valid")
}

fn setting_schema_name() -> SchemaName {
    SchemaName::parse(SETTING_SCHEMA).expect("static SETTING_SCHEMA is valid")
}

fn registry_schema_name() -> SchemaName {
    SchemaName::parse(WORKLOAD_REGISTRY_SCHEMA).expect("static WORKLOAD_REGISTRY_SCHEMA is valid")
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use crate::registry::toml::TomlErrorKind;

    /// Pinned cross-platform canonical JSON for reference workload Q (`frob.workload/1`).
    const PINNED_Q_CANONICAL_JSON: &str = r#"{"actuated":12,"bodies":13,"contact_cap":32,"decimation":"10","dof":18,"foot_samples":16,"grounds":["flat","rough"],"h_s":"1/500","name":"Q","newton_iterations":4,"robot":"ref-quadruped-12","row_cap":160,"row_cap_override":{"reason":"raised to the next multiple of the execution width 32 (reference-workload decision; OPEN-SPARK-3)","rule":152},"schema":"frob.workload/1","status":"default","warm_start":true,"worlds":8192}"#;

    /// Pinned BLAKE3 derive-key digest of `PINNED_Q_CANONICAL_JSON` under `"frob.workload/1"`.
    const PINNED_Q_WORKLOAD_ID_HEX: &str =
        "6a6c76cd1c442ab371a0065e1f78b97769d7f641eeefd6981a87e420da08eec4";

    /// Pinned canonical JSON for reference workload H (`frob.workload/1`).
    const PINNED_H_CANONICAL_JSON: &str = r#"{"actuated":29,"bodies":30,"contact_cap":64,"decimation":"10","dof":35,"grounds":["flat","heightfield"],"h_s":"1/500","name":"H","newton_iterations":4,"robot":"ref-humanoid-29","row_cap":320,"schema":"frob.workload/1","status":"default","warm_start":true,"worlds":8192}"#;

    /// Pinned BLAKE3 derive-key digest of `PINNED_H_CANONICAL_JSON` under `"frob.workload/1"`.
    const PINNED_H_WORKLOAD_ID_HEX: &str =
        "a3ed3ad27b32a494919c6efd64fd95f7fdab61f8f2b4965175b67f6d845a3c82";

    fn committed_workloads_path() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../registries/workloads.toml")
    }

    fn base_q() -> Workload {
        Workload {
            name: "Q".to_owned(),
            robot: "ref-quadruped-12".to_owned(),
            actuated: 12,
            dof: 18,
            bodies: 13,
            foot_samples: Some(16),
            contact_cap: 32,
            row_cap: 160,
            row_cap_override: Some(RowCapOverride {
                rule: 152,
                reason: "raised to the next multiple of the execution width 32 (reference-workload decision; OPEN-SPARK-3)".to_owned(),
            }),
            h_s: Rational::new(1, 500).expect("1/500"),
            newton_iterations: 4,
            warm_start: true,
            decimation: Rational::new(10, 1).expect("10/1"),
            worlds: 8192,
            grounds: vec!["flat".to_owned(), "rough".to_owned()],
            status: WorkloadStatus::Default,
            adr: None,
        }
    }

    #[test]
    fn identity_stable_across_platforms() {
        let q = base_q();
        let q_canon = q.canonical_bytes().expect("canonical Q");
        assert_eq!(q_canon.as_str(), PINNED_Q_CANONICAL_JSON);
        let q_id = q.id().expect("Q id");
        assert_eq!(q_id.to_hex(), PINNED_Q_WORKLOAD_ID_HEX);
        assert_eq!(
            WorkloadId::parse(&q_id.to_string()).expect("round-trip b3 prefix"),
            q_id
        );

        let reg = load(&committed_workloads_path()).expect("load registries/workloads.toml");
        let file_q = reg.get("Q").expect("Q in committed registry");
        let file_h = reg.get("H").expect("H in committed registry");

        assert_eq!(file_q.canonical_bytes().unwrap().as_str(), PINNED_Q_CANONICAL_JSON);
        assert_eq!(file_q.id().unwrap().to_hex(), PINNED_Q_WORKLOAD_ID_HEX);
        assert_eq!(file_h.canonical_bytes().unwrap().as_str(), PINNED_H_CANONICAL_JSON);
        assert_eq!(file_h.id().unwrap().to_hex(), PINNED_H_WORKLOAD_ID_HEX);
        assert_ne!(file_q.id().unwrap(), file_h.id().unwrap());
    }

    #[test]
    fn identity_changes_with_any_field() {
        let base = base_q();
        let base_id = base.id().expect("base Q id");

        // Exhaustive single-field mutations over every field of Workload.
        let mut mutations: Vec<(&str, Workload)> = Vec::new();

        let mut m = base.clone();
        m.name = "Q-alt".to_owned();
        mutations.push(("name", m));

        let mut m = base.clone();
        m.robot = "ref-quadruped-12b".to_owned();
        mutations.push(("robot", m));

        let mut m = base.clone();
        m.actuated = 13;
        mutations.push(("actuated", m));

        let mut m = base.clone();
        m.dof = 19;
        mutations.push(("dof", m));

        let mut m = base.clone();
        m.bodies = 14;
        mutations.push(("bodies", m));

        let mut m = base.clone();
        m.foot_samples = Some(17);
        mutations.push(("foot_samples value", m));

        let mut m = base.clone();
        m.foot_samples = None;
        mutations.push(("foot_samples removed", m));

        let mut m = base.clone();
        m.contact_cap = 33;
        mutations.push(("contact_cap", m));

        let mut m = base.clone();
        m.row_cap = 152;
        mutations.push(("row_cap", m));

        let mut m = base.clone();
        m.row_cap_override.as_mut().unwrap().rule = 156;
        mutations.push(("row_cap_override.rule", m));

        let mut m = base.clone();
        m.row_cap_override.as_mut().unwrap().reason.push('.');
        mutations.push(("row_cap_override.reason", m));

        let mut m = base.clone();
        m.row_cap_override = None;
        mutations.push(("row_cap_override removed", m));

        let mut m = base.clone();
        m.h_s = Rational::new(1, 250).unwrap();
        mutations.push(("h_s", m));

        let mut m = base.clone();
        m.newton_iterations = 5;
        mutations.push(("newton_iterations", m));

        let mut m = base.clone();
        m.warm_start = false;
        mutations.push(("warm_start", m));

        let mut m = base.clone();
        m.decimation = Rational::new(5, 1).unwrap();
        mutations.push(("decimation", m));

        let mut m = base.clone();
        m.worlds = 4096;
        mutations.push(("worlds", m));

        let mut m = base.clone();
        m.grounds = vec!["rough".to_owned(), "flat".to_owned()];
        mutations.push(("grounds order", m));

        let mut m = base.clone();
        m.grounds.push("heightfield".to_owned());
        mutations.push(("grounds element added", m));

        let mut m = base.clone();
        m.status = WorkloadStatus::Frozen;
        mutations.push(("status", m));

        let mut m = base.clone();
        m.adr = Some("ADR-SPARK-03".to_owned());
        mutations.push(("adr added", m));

        let mut seen_ids = BTreeSet::new();
        seen_ids.insert(base_id);
        for (label, mutated) in &mutations {
            let id = mutated.id().expect("mutated id");
            assert_ne!(id, base_id, "single-field edit `{label}` must change WorkloadId");
            assert!(
                seen_ids.insert(id),
                "single-field edit `{label}` collided with another mutation's WorkloadId"
            );
        }

        // Seeded deterministic property sweep over single-field edits (64 iterations).
        let mut rng: u64 = 0x3602_2026_1124_0001;
        let mut next = || {
            rng = rng.wrapping_add(0x9E37_79B9_7F4A_7C15);
            let mut z = rng;
            z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
            z ^ (z >> 31)
        };
        for step in 0..64u32 {
            let mut w = base.clone();
            let pick = (next() % 12) as u32;
            let delta = 1 + (step % 7);
            match pick {
                0 => w.actuated += delta,
                1 => w.dof += delta,
                2 => w.bodies += delta,
                3 => w.contact_cap += delta,
                4 => w.row_cap += delta,
                5 => w.newton_iterations += delta,
                6 => w.worlds += delta,
                7 => w.foot_samples = Some(16 + delta),
                8 => w.robot = format!("ref-quadruped-{}", 12 + delta),
                9 => w.grounds.push(format!("terrain-{delta}")),
                10 => w.adr = Some(format!("ADR-SPARK-{delta}")),
                _ => w.h_s = Rational::new(1, 500 + u64::from(delta)).unwrap(),
            }
            assert_ne!(
                w.id().unwrap(),
                base_id,
                "seeded mutation step {step} (field {pick}) must change WorkloadId"
            );
        }

        // Metamorphic: reordering keys and changing whitespace in TOML must NOT change WorkloadId.
        let toml_a = r#"
schema = "frob.workload/1"

[workload.Q]
robot = "ref-quadruped-12"
actuated = 12
dof = 18
bodies = 13
foot_samples = 16
contact_cap = 32
row_cap = 160
row_cap_override = { rule = 152, reason = "raised to the next multiple of the execution width 32 (reference-workload decision; OPEN-SPARK-3)" }
h_s = "1/500"
newton_iterations = 4
warm_start = true
decimation = "10/1"
worlds = 8192
grounds = ["flat", "rough"]
status = "default"
"#;
        let toml_b = r#"
[workload.Q]
status            = "default"
worlds            = 8192
grounds           = [ "flat", "rough" ]
warm_start        = true
decimation        = "10/1"
newton_iterations = 4
h_s               = "1/500"
row_cap_override  = { reason = "raised to the next multiple of the execution width 32 (reference-workload decision; OPEN-SPARK-3)", rule = 152 }
row_cap           = 160
contact_cap       = 32
foot_samples      = 16
bodies            = 13
dof               = 18
actuated          = 12
robot             = "ref-quadruped-12"
"#;
        let reg_a = parse_str(toml_a).expect("parse order A");
        let reg_b = parse_str(toml_b).expect("parse order B");
        assert_eq!(
            reg_a.get("Q").unwrap().id().unwrap(),
            reg_b.get("Q").unwrap().id().unwrap(),
            "key order and whitespace must not affect WorkloadId"
        );
        assert_eq!(reg_a.get("Q").unwrap().id().unwrap(), base_id);

        // ComparatorSetting identity changes with any field edit and is stable under reordering.
        let s_base = ComparatorSetting {
            comparator: "mujoco-warp".to_owned(),
            workload: "Q".to_owned(),
            kind: SettingKind::Native,
            h_s: Rational::new(1, 200).unwrap(),
            decimation: Rational::new(4, 1).unwrap(),
            iterations: 4,
            source: Some("Appendix H.3".to_owned()),
            difference: None,
        };
        let s_id = s_base.id().unwrap();
        let mut s_diff = s_base.clone();
        s_diff.iterations = 5;
        assert_ne!(s_diff.id().unwrap(), s_id);
        let mut s_src = s_base.clone();
        s_src.source = Some("Appendix H.3 revised".to_owned());
        assert_ne!(s_src.id().unwrap(), s_id);
    }

    #[test]
    fn unknown_field_refused() {
        let valid_q = r#"
schema = "frob.workload/1"

[workload.Q]
robot = "ref-quadruped-12"
actuated = 12
dof = 18
bodies = 13
foot_samples = 16
contact_cap = 32
row_cap = 160
row_cap_override = { rule = 152, reason = "aligned to 32" }
h_s = "1/500"
newton_iterations = 4
warm_start = true
decimation = "10/1"
worlds = 8192
grounds = ["flat", "rough"]
status = "default"
"#;
        assert!(parse_str(valid_q).is_ok(), "baseline snippet must parse");

        // 1. Unknown top-level key.
        let top_unknown = format!("extra_top = 1\n{valid_q}");
        let err = parse_str(&top_unknown).expect_err("unknown top-level key must fail");
        assert!(
            matches!(
                err,
                WorkloadError::Toml(TomlError {
                    kind: TomlErrorKind::UnknownKey { ref key, .. },
                    ..
                }) if key == "extra_top"
            ),
            "unexpected error: {err}"
        );

        // 2. Unknown key inside [workload.Q].
        let workload_unknown = format!("{valid_q}\nrogue_knob = 42\n");
        let err = parse_str(&workload_unknown).expect_err("unknown workload key must fail");
        assert!(
            matches!(
                err,
                WorkloadError::Toml(TomlError {
                    kind: TomlErrorKind::UnknownKey { ref key, .. },
                    ..
                }) if key == "rogue_knob"
            ),
            "unexpected error: {err}"
        );

        // 3. Unknown key inside row_cap_override inline table.
        let override_unknown = valid_q.replace(
            r#"row_cap_override = { rule = 152, reason = "aligned to 32" }"#,
            r#"row_cap_override = { rule = 152, reason = "aligned to 32", extra = 1 }"#,
        );
        let err = parse_str(&override_unknown).expect_err("unknown override key must fail");
        assert!(
            matches!(
                err,
                WorkloadError::Toml(TomlError {
                    kind: TomlErrorKind::UnknownKey { ref key, .. },
                    ..
                }) if key == "extra"
            ),
            "unexpected error: {err}"
        );

        // 4. Unknown key inside [[setting]].
        let setting_unknown = format!(
            "{valid_q}\n[[setting]]\ncomparator = \"mujoco-warp\"\nworkload = \"Q\"\nkind = \"matched\"\nh_s = \"1/500\"\ndecimation = \"10/1\"\niterations = 4\nunexpected_flag = true\n"
        );
        let err = parse_str(&setting_unknown).expect_err("unknown setting key must fail");
        assert!(
            matches!(
                err,
                WorkloadError::Toml(TomlError {
                    kind: TomlErrorKind::UnknownKey { ref key, .. },
                    ..
                }) if key == "unexpected_flag"
            ),
            "unexpected error: {err}"
        );

        // 5. Wrong schema major version.
        let bad_schema = valid_q.replace("frob.workload/1", "frob.workload/2");
        let err = parse_str(&bad_schema).expect_err("unsupported schema version must fail");
        assert!(
            matches!(err, WorkloadError::UnsupportedSchema { .. }),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn row_cap_formula_and_overflow_safety() {
        assert_eq!(row_cap_raw(32, 12), 152);
        assert_eq!(row_cap(32, 12), 152);
        assert_eq!(row_cap_raw(64, 29), 314);
        assert_eq!(row_cap(64, 29), 320);
        assert_eq!(checked_row_cap_raw(u32::MAX, 0), None);
        assert_eq!(checked_row_cap(u32::MAX / 4, 0), None);
    }

    #[test]
    fn rational_parse_and_exact_mul() {
        let h = Rational::parse("1/500").unwrap();
        let d = Rational::parse("10/1").unwrap();
        assert!(!h.is_integer());
        assert!(d.is_integer());
        assert_eq!(h.checked_mul(d), Some(CONTROL_PERIOD_20MS));

        for bad in ["", "0/1", "1/0", "2/1000", "10/2", "-1/500", "+1/500", "01/500", "1/500/2"] {
            assert!(Rational::parse(bad).is_err(), "expected error for `{bad}`");
        }
    }
}
