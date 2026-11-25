//! Performance target registry (`registries/perf_targets.toml`, `frob.perf.targets/1`),
//! strict loader, consistency lints, and gate helpers (`N36-01`, ADR-PERF-01 to -05).
//!
//! Reuses [`frob_core::perf::units`] (`RateUnit`, `Decimation`, `ExactRational`) and
//! [`frob_core::perf::workload`] (`WorkloadRegistry`, `SettingKind`) rather than
//! re-deriving unit conversions or workload identities. Contains no refusal code strings:
//! `judge` returns typed [`JudgeError`] variants that downstream harness code maps at the
//! reporting boundary.

use std::collections::BTreeMap;
use std::fmt;
use std::fs;
use std::path::Path;

use frob_core::perf::units::{Decimation, ExactRational, RateUnit};
use frob_core::perf::workload::{SettingKind, WorkloadRegistry};
use frob_core::registry::toml::{self, Fields, Table, TomlError, Value};

/// Schema identifier of `registries/perf_targets.toml`.
pub const TARGETS_SCHEMA: &str = "frob.perf.targets/1";

/// Canonical workspace path of the performance targets registry.
pub const PERF_TARGETS_PATH: &str = "registries/perf_targets.toml";

/// Canonical workspace path of the hand-written JSON-Schema for `perf_targets.toml`.
pub const PERF_TARGETS_SCHEMA_PATH: &str = "schemas/registries/perf_targets.schema.json";

/// Canonical workspace path of the ADR-PERF-01..05 decision record.
pub const PERF_ADR_PATH: &str = "docs/adr/ADR-PERF-01..05.md";

/// Total number of target rows defined by §36.3 (rows 1..=19) and §36.15 (rows 20..=22).
pub const EXPECTED_ROW_COUNT: usize = 22;

/// Closed set of units used by rows 1..=22, wrapping [`RateUnit`] for all rate rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TargetUnit {
    Rate(RateUnit),
    Seconds,
    Minutes,
    Ratio,
    Percent,
    DesignsPerDeviceSec,
    PhaseLedger,
}

impl TargetUnit {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Rate(u) => u.as_str(),
            Self::Seconds => "seconds",
            Self::Minutes => "minutes",
            Self::Ratio => "ratio",
            Self::Percent => "percent",
            Self::DesignsPerDeviceSec => "designs_per_device_s",
            Self::PhaseLedger => "phase_ledger",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        if let Some(rate) = RateUnit::parse(s) {
            return Some(Self::Rate(rate));
        }
        match s {
            "seconds" => Some(Self::Seconds),
            "minutes" => Some(Self::Minutes),
            "ratio" => Some(Self::Ratio),
            "percent" => Some(Self::Percent),
            "designs_per_device_s" => Some(Self::DesignsPerDeviceSec),
            "phase_ledger" => Some(Self::PhaseLedger),
            _ => None,
        }
    }

    /// True iff this unit belongs to the step-rate family requiring an explicit [`Decimation`]
    /// (delegated directly to [`RateUnit::is_step_unit`], ADR-PERF-02).
    #[must_use]
    pub const fn requires_decimation(self) -> bool {
        match self {
            Self::Rate(u) => u.is_step_unit(),
            _ => false,
        }
    }
}

impl fmt::Display for TargetUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Classification of how a row's binding tolerance is defined.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ToleranceKind {
    RatioToIncumbent,
    RatioToRow,
    Absolute,
    ReportedOnly,
}

impl ToleranceKind {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RatioToIncumbent => "ratio_to_incumbent",
            Self::RatioToRow => "ratio_to_row",
            Self::Absolute => "absolute",
            Self::ReportedOnly => "reported_only",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "ratio_to_incumbent" => Some(Self::RatioToIncumbent),
            "ratio_to_row" => Some(Self::RatioToRow),
            "absolute" => Some(Self::Absolute),
            "reported_only" => Some(Self::ReportedOnly),
            _ => None,
        }
    }

    #[must_use]
    pub const fn is_ratio(self) -> bool {
        matches!(self, Self::RatioToIncumbent | Self::RatioToRow)
    }
}

/// Comparison orientation of a numeric target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Direction {
    AtLeast,
    AtMost,
}

impl Direction {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AtLeast => "at_least",
            Self::AtMost => "at_most",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "at_least" => Some(Self::AtLeast),
            "at_most" => Some(Self::AtMost),
            _ => None,
        }
    }
}

/// Explicit ADR-PERF-03 status attached to every threshold returned by [`TargetRegistry::threshold`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ThresholdStatus {
    Tolerance,
    InterimFloor,
    ReportedOnly,
}

impl ThresholdStatus {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Tolerance => "tolerance",
            Self::InterimFloor => "interim-floor",
            Self::ReportedOnly => "reported-only",
        }
    }
}

impl fmt::Display for ThresholdStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Status label carried on every number stored in `registries/perf_targets.toml` (ADR-PERF-03).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NumberStatus {
    Tolerance,
    InterimFloor,
    Ambition,
    PlanningValue,
    ReportedOnly,
}

impl NumberStatus {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Tolerance => "tolerance",
            Self::InterimFloor => "interim-floor",
            Self::Ambition => "ambition",
            Self::PlanningValue => "planning-value",
            Self::ReportedOnly => "reported-only",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "tolerance" => Some(Self::Tolerance),
            "interim-floor" => Some(Self::InterimFloor),
            "ambition" => Some(Self::Ambition),
            "planning-value" => Some(Self::PlanningValue),
            "reported-only" => Some(Self::ReportedOnly),
            _ => None,
        }
    }
}

/// Exact non-negative plain decimal value stored as a strict-TOML string.
#[derive(Debug, Clone, PartialEq)]
pub struct DecimalValue {
    raw: String,
    value: f64,
}

impl DecimalValue {
    pub fn parse(s: &str, table: &str, field: &str) -> Result<Self, TargetError> {
        if !is_valid_non_negative_decimal(s) {
            return Err(TargetError::InvalidField {
                table: table.to_owned(),
                field: field.to_owned(),
                reason: format!("expected non-negative plain decimal string, found `{s}`"),
            });
        }
        let value: f64 = s.parse().map_err(|_| TargetError::InvalidField {
            table: table.to_owned(),
            field: field.to_owned(),
            reason: format!("decimal `{s}` cannot be represented as finite f64"),
        })?;
        if !value.is_finite() {
            return Err(TargetError::InvalidField {
                table: table.to_owned(),
                field: field.to_owned(),
                reason: format!("decimal `{s}` is not finite"),
            });
        }
        Ok(Self {
            raw: s.to_owned(),
            value,
        })
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.raw
    }

    #[must_use]
    pub const fn as_f64(&self) -> f64 {
        self.value
    }

    /// Parse into an exact [`ExactRational`] (`num / 10^scale`) for D0 exact consistency arithmetic.
    pub fn to_exact_rational(&self) -> Result<ExactRational, TargetError> {
        let (whole, frac) = self.raw.split_once('.').unwrap_or((&self.raw, ""));
        let scale = u32::try_from(frac.len()).map_err(|_| TargetError::InvalidField {
            table: "decimal".to_owned(),
            field: "value".to_owned(),
            reason: "fractional precision overflow".to_owned(),
        })?;
        let den = 10u128
            .checked_pow(scale)
            .ok_or_else(|| TargetError::InvalidField {
                table: "decimal".to_owned(),
                field: "value".to_owned(),
                reason: "decimal scale overflow".to_owned(),
            })?;
        let digits = format!("{whole}{frac}");
        let num: u128 = digits.parse().map_err(|_| TargetError::InvalidField {
            table: "decimal".to_owned(),
            field: "value".to_owned(),
            reason: format!("decimal `{}` exceeds u128", self.raw),
        })?;
        ExactRational::new(num, den).map_err(|e| TargetError::InvalidField {
            table: "decimal".to_owned(),
            field: "value".to_owned(),
            reason: e.to_string(),
        })
    }
}

fn is_valid_non_negative_decimal(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let (whole, frac_opt) = match s.split_once('.') {
        Some((w, f)) => (w, Some(f)),
        None => (s, None),
    };
    if whole.is_empty() || !whole.bytes().all(|b| b.is_ascii_digit()) {
        return false;
    }
    if whole.len() > 1 && whole.starts_with('0') {
        return false;
    }
    if let Some(frac) = frac_opt {
        if frac.is_empty() || !frac.bytes().all(|b| b.is_ascii_digit()) {
            return false;
        }
    }
    true
}

/// Binding tolerance block of a target row.
#[derive(Debug, Clone, PartialEq)]
pub struct ToleranceSpec {
    pub kind: ToleranceKind,
    pub status: Option<NumberStatus>,
    pub value: Option<DecimalValue>,
    pub direction: Option<Direction>,
    pub comparator: Option<String>,
    pub setting_kind: Option<SettingKind>,
    pub reference_row: Option<u8>,
    pub note: Option<String>,
}

/// Temporary pre-measurement floor (explicitly distinct from `tolerance`).
#[derive(Debug, Clone, PartialEq)]
pub struct InterimFloorSpec {
    pub status: Option<NumberStatus>,
    pub value: DecimalValue,
    pub unit: TargetUnit,
    pub direction: Direction,
    pub note: Option<String>,
}

/// Stretch ambition target (nothing in the plan's argument depends on it).
#[derive(Debug, Clone, PartialEq)]
pub struct AmbitionSpec {
    pub status: Option<NumberStatus>,
    pub kind: ToleranceKind,
    pub value: DecimalValue,
    pub direction: Direction,
    pub unit: Option<TargetUnit>,
    pub note: Option<String>,
}

/// Implied planning value(s) derived from published incumbent references.
#[derive(Debug, Clone, PartialEq)]
pub struct PlanningValue {
    pub status: Option<NumberStatus>,
    pub tolerance: Option<DecimalValue>,
    pub ambition: Option<DecimalValue>,
    pub unit: TargetUnit,
    pub note: Option<String>,
}

/// Named sub-threshold for multi-case rows (rows 19, 20, 21).
#[derive(Debug, Clone, PartialEq)]
pub struct SubTarget {
    pub id: String,
    pub label: String,
    pub status: Option<NumberStatus>,
    pub value: Option<DecimalValue>,
    pub direction: Option<Direction>,
    pub percentile: Option<String>,
    pub unit: TargetUnit,
}

/// Reading/measurement lifecycle of an incumbent reference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ReferenceStatus {
    ToReRead,
    Measured,
}

impl ReferenceStatus {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ToReRead => "to-re-read",
            Self::Measured => "measured",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "to-re-read" => Some(Self::ToReRead),
            "measured" => Some(Self::Measured),
            _ => None,
        }
    }
}

/// Published or locally measured incumbent anchor carried by a target row.
#[derive(Debug, Clone, PartialEq)]
pub struct IncumbentReference {
    pub text: String,
    pub source_mark: String,
    pub status: ReferenceStatus,
    pub published_value: Option<DecimalValue>,
    pub published_secondary_value: Option<DecimalValue>,
    pub disclosure_id: Option<String>,
    pub loaded_host_lower_bound: bool,
}

/// One row (1..=22) in `registries/perf_targets.toml`.
#[derive(Debug, Clone, PartialEq)]
pub struct TargetRow {
    pub row: u8,
    pub workload: String,
    pub scope: String,
    pub hardware: String,
    pub unit: TargetUnit,
    pub decimation: Option<Decimation>,
    pub tolerance: ToleranceSpec,
    pub interim_floor: Option<InterimFloorSpec>,
    pub ambition: Option<AmbitionSpec>,
    pub planning_value: Option<PlanningValue>,
    pub sub_targets: Vec<SubTarget>,
    pub incumbent_reference: Option<IncumbentReference>,
    pub gated_in: Vec<String>,
    pub carrier: String,
}

/// Binding threshold returned by [`TargetRegistry::threshold`], always paired with its status.
#[derive(Debug, Clone, PartialEq)]
pub struct Threshold {
    pub row: u8,
    pub status: ThresholdStatus,
    pub kind: ToleranceKind,
    pub direction: Option<Direction>,
    pub value: Option<f64>,
    pub unit: TargetUnit,
}

/// Typed measurement figure offered to [`TargetRegistry::judge`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Figure {
    /// One-sided 95% lower confidence bound (required for ratio rows and valid for `at_least` absolute rows).
    LowerBound(f64),
    /// One-sided upper confidence bound (valid for `at_most` absolute rows).
    UpperBound(f64),
    /// Point estimate (valid only on `absolute` or `reported_only` rows; rejected on `ratio_*` rows).
    Point(f64),
}

/// Pass/fail verdict returned by [`TargetRegistry::judge`].
#[derive(Debug, Clone, PartialEq)]
pub struct Verdict {
    pub row: u8,
    pub passed: bool,
    pub status: ThresholdStatus,
    pub observed: f64,
    pub required: Option<f64>,
}

/// Typed error returned by [`TargetRegistry::threshold`], [`TargetRegistry::judge`], or
/// [`TargetRegistry::planning_value`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JudgeError {
    /// Requested row identifier does not exist in the registry.
    UnknownRow { row: u8 },
    /// A `Figure::Point` was offered on a `ratio_*` row (ADR-PERF-04 requires a paired lower bound).
    PointEstimateOnRatioRow { row: u8 },
    /// A confidence bound on the wrong side was offered (e.g. `UpperBound` on a ratio or `at_least` row,
    /// or `LowerBound` on an `absolute` `at_most` row).
    WrongSide { row: u8 },
    /// Observed value is `NaN` or infinite.
    NonFiniteFigure { row: u8 },
    /// Target row definition is missing its numeric threshold or direction.
    IncompleteTargetRow { row: u8 },
}

impl fmt::Display for JudgeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownRow { row } => write!(f, "unknown performance target row {row}"),
            Self::PointEstimateOnRatioRow { row } => write!(
                f,
                "row {row} is a ratio row and cannot be judged with a point estimate (ADR-PERF-04)"
            ),
            Self::WrongSide { row } => {
                write!(f, "figure bound is on the wrong side for target row {row}")
            }
            Self::NonFiniteFigure { row } => {
                write!(f, "non-finite figure offered for target row {row}")
            }
            Self::IncompleteTargetRow { row } => write!(
                f,
                "target row {row} is missing a required threshold value or direction"
            ),
        }
    }
}

impl std::error::Error for JudgeError {}

/// Errors produced while loading or strictly parsing `registries/perf_targets.toml`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TargetError {
    Io {
        path: String,
        message: String,
    },
    Toml(TomlError),
    UnsupportedSchema {
        expected: &'static str,
        found: String,
    },
    MissingDecimation {
        row: u8,
        unit: &'static str,
    },
    InvalidField {
        table: String,
        field: String,
        reason: String,
    },
}

impl fmt::Display for TargetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io { path, message } => write!(f, "failed to read `{path}`: {message}"),
            Self::Toml(err) => write!(f, "TOML parse error: {err}"),
            Self::UnsupportedSchema { expected, found } => {
                write!(
                    f,
                    "unsupported targets schema `{found}` (expected `{expected}`)"
                )
            }
            Self::MissingDecimation { row, unit } => write!(
                f,
                "row {row} uses step-family unit `{unit}` without a decimation (ADR-PERF-02)"
            ),
            Self::InvalidField {
                table,
                field,
                reason,
            } => write!(f, "invalid field `{field}` in `{table}`: {reason}"),
        }
    }
}

impl std::error::Error for TargetError {}

impl From<TomlError> for TargetError {
    fn from(err: TomlError) -> Self {
        Self::Toml(err)
    }
}

/// Machine-readable reason code for a target registry lint finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TargetFindingKind {
    RowSequenceInvalid,
    MissingCarrier,
    MissingNumberStatus,
    WrongNumberStatus,
    InterimFloorPromotedToTolerance,
    IncompleteToleranceSpec,
    MissingDecimation,
    WorkloadNotFound,
    DecimationDisagreesWithWorkload,
    ComparatorSettingNotFound,
    ReferenceStatusInvalid,
    ConsistencyViolation,
    Row13LoadedHostFlagMissing,
}

/// One semantic lint violation found in a parsed [`TargetRegistry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TargetFinding {
    pub kind: TargetFindingKind,
    pub row: Option<u8>,
    pub message: String,
}

impl TargetFinding {
    #[must_use]
    pub fn new(kind: TargetFindingKind, row: Option<u8>, message: impl Into<String>) -> Self {
        Self {
            kind,
            row,
            message: message.into(),
        }
    }
}

impl fmt::Display for TargetFinding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.row {
            Some(r) => write!(f, "[{:?}] row {r}: {}", self.kind, self.message),
            None => write!(f, "[{:?}] {}", self.kind, self.message),
        }
    }
}

/// Parsed contents of `registries/perf_targets.toml` (`frob.perf.targets/1`).
#[derive(Debug, Clone, PartialEq)]
pub struct TargetRegistry {
    pub schema: String,
    pub rows: BTreeMap<u8, TargetRow>,
}

impl TargetRegistry {
    /// Strictly parse `registries/perf_targets.toml` bytes via `frob_core::registry::toml`.
    pub fn parse_bytes(bytes: &[u8]) -> Result<Self, TargetError> {
        parse_bytes(bytes)
    }

    /// Strictly parse `registries/perf_targets.toml` from a UTF-8 string slice.
    pub fn parse_str(text: &str) -> Result<Self, TargetError> {
        parse_str(text)
    }

    /// Load and strictly parse `registries/perf_targets.toml` from disk.
    pub fn load(path: &Path) -> Result<Self, TargetError> {
        load(path)
    }

    /// Look up one target row by row number (`1..=22`).
    #[must_use]
    pub fn get(&self, row: u8) -> Option<&TargetRow> {
        self.rows.get(&row)
    }

    /// Return the binding threshold and its explicit [`ThresholdStatus`] for `row` (`N36-01`).
    ///
    /// - For a `ratio_to_incumbent` row: when `measured_incumbent` is `Some(inc)`, returns
    ///   `(tol.value * inc, Tolerance)`; when `None` and an `interim_floor` exists (e.g.
    ///   row 1: 2.0 M world-steps/s; row 14: 275 k world-steps/s), returns
    ///   `(floor.value, InterimFloor)`; otherwise returns the binding ratio with `Tolerance`.
    /// - For a `ratio_to_row` row: when `measured_incumbent` carries the reference row's
    ///   measured value, returns `(tol.value * ref_val, Tolerance)`; when `None` and an
    ///   `interim_floor` exists (rows 2 and 3), returns `(floor.value, InterimFloor)`;
    ///   otherwise returns the binding ratio with `Tolerance`.
    /// - For an `absolute` row: returns `(tol.value, Tolerance)`.
    /// - For a `reported_only` row: returns `(None, ReportedOnly)`.
    pub fn threshold(
        &self,
        row: u8,
        measured_incumbent: Option<f64>,
    ) -> Result<Threshold, JudgeError> {
        let Some(target) = self.rows.get(&row) else {
            return Err(JudgeError::UnknownRow { row });
        };
        if let Some(m) = measured_incumbent
            && (!m.is_finite() || m <= 0.0)
        {
            return Err(JudgeError::NonFiniteFigure { row });
        }

        let tol = &target.tolerance;
        match tol.kind {
            ToleranceKind::ReportedOnly => Ok(Threshold {
                row,
                status: ThresholdStatus::ReportedOnly,
                kind: ToleranceKind::ReportedOnly,
                direction: None,
                value: None,
                unit: target.unit,
            }),
            ToleranceKind::Absolute => {
                let val = tol
                    .value
                    .as_ref()
                    .ok_or(JudgeError::IncompleteTargetRow { row })?
                    .as_f64();
                let dir = tol
                    .direction
                    .ok_or(JudgeError::IncompleteTargetRow { row })?;
                Ok(Threshold {
                    row,
                    status: ThresholdStatus::Tolerance,
                    kind: ToleranceKind::Absolute,
                    direction: Some(dir),
                    value: Some(val),
                    unit: target.unit,
                })
            }
            ToleranceKind::RatioToIncumbent | ToleranceKind::RatioToRow => {
                let ratio = tol
                    .value
                    .as_ref()
                    .ok_or(JudgeError::IncompleteTargetRow { row })?
                    .as_f64();
                let dir = tol
                    .direction
                    .ok_or(JudgeError::IncompleteTargetRow { row })?;
                if let Some(inc) = measured_incumbent {
                    Ok(Threshold {
                        row,
                        status: ThresholdStatus::Tolerance,
                        kind: tol.kind,
                        direction: Some(dir),
                        value: Some(ratio * inc),
                        unit: target.unit,
                    })
                } else if let Some(ref floor) = target.interim_floor {
                    Ok(Threshold {
                        row,
                        status: ThresholdStatus::InterimFloor,
                        kind: ToleranceKind::Absolute,
                        direction: Some(floor.direction),
                        value: Some(floor.value.as_f64()),
                        unit: floor.unit,
                    })
                } else {
                    Ok(Threshold {
                        row,
                        status: ThresholdStatus::Tolerance,
                        kind: tol.kind,
                        direction: Some(dir),
                        value: Some(ratio),
                        unit: TargetUnit::Ratio,
                    })
                }
            }
        }
    }

    /// Judge a typed [`Figure`] against `row`'s binding tolerance (`N36-01`, ADR-PERF-04).
    ///
    /// - On any `ratio_*` row:
    ///   - `Figure::Point` returns [`JudgeError::PointEstimateOnRatioRow`].
    ///   - `Figure::UpperBound` returns [`JudgeError::WrongSide`].
    ///   - `Figure::LowerBound(l)` is judged on the ratio oriented so larger is better:
    ///     `l >= v` for `at_least` and `l >= 1.0 / v` for `at_most` (e.g. row 4: `0.80`).
    /// - On an `absolute` row:
    ///   - `at_least` accepts `LowerBound` or `Point` (`x >= v`) and rejects `UpperBound` with `WrongSide`.
    ///   - `at_most` accepts `UpperBound` or `Point` (`x <= v`) and rejects `LowerBound` with `WrongSide`.
    /// - On a `reported_only` row: any finite figure passes with `ThresholdStatus::ReportedOnly`.
    pub fn judge(&self, row: u8, figure: Figure) -> Result<Verdict, JudgeError> {
        let Some(target) = self.rows.get(&row) else {
            return Err(JudgeError::UnknownRow { row });
        };

        let raw_val = match figure {
            Figure::LowerBound(x) | Figure::UpperBound(x) | Figure::Point(x) => x,
        };
        if !raw_val.is_finite() {
            return Err(JudgeError::NonFiniteFigure { row });
        }

        let tol = &target.tolerance;
        match tol.kind {
            ToleranceKind::ReportedOnly => Ok(Verdict {
                row,
                passed: true,
                status: ThresholdStatus::ReportedOnly,
                observed: raw_val,
                required: None,
            }),
            ToleranceKind::RatioToIncumbent | ToleranceKind::RatioToRow => {
                let v = tol
                    .value
                    .as_ref()
                    .ok_or(JudgeError::IncompleteTargetRow { row })?
                    .as_f64();
                let dir = tol
                    .direction
                    .ok_or(JudgeError::IncompleteTargetRow { row })?;
                match figure {
                    Figure::Point(_) => Err(JudgeError::PointEstimateOnRatioRow { row }),
                    Figure::UpperBound(_) => Err(JudgeError::WrongSide { row }),
                    Figure::LowerBound(l) => {
                        let required = match dir {
                            Direction::AtLeast => v,
                            Direction::AtMost => {
                                if v <= 0.0 {
                                    return Err(JudgeError::IncompleteTargetRow { row });
                                }
                                1.0 / v
                            }
                        };
                        Ok(Verdict {
                            row,
                            passed: l >= required,
                            status: ThresholdStatus::Tolerance,
                            observed: l,
                            required: Some(required),
                        })
                    }
                }
            }
            ToleranceKind::Absolute => {
                let v = tol
                    .value
                    .as_ref()
                    .ok_or(JudgeError::IncompleteTargetRow { row })?
                    .as_f64();
                let dir = tol
                    .direction
                    .ok_or(JudgeError::IncompleteTargetRow { row })?;
                match (dir, figure) {
                    (Direction::AtLeast, Figure::UpperBound(_))
                    | (Direction::AtMost, Figure::LowerBound(_)) => {
                        Err(JudgeError::WrongSide { row })
                    }
                    (Direction::AtLeast, Figure::LowerBound(x) | Figure::Point(x)) => Ok(Verdict {
                        row,
                        passed: x >= v,
                        status: ThresholdStatus::Tolerance,
                        observed: x,
                        required: Some(v),
                    }),
                    (Direction::AtMost, Figure::UpperBound(x) | Figure::Point(x)) => Ok(Verdict {
                        row,
                        passed: x <= v,
                        status: ThresholdStatus::Tolerance,
                        observed: x,
                        required: Some(v),
                    }),
                }
            }
        }
    }

    /// Return the planning-value block for `row`, or `Err(JudgeError::UnknownRow)` if absent.
    pub fn planning_value(&self, row: u8) -> Result<Option<&PlanningValue>, JudgeError> {
        let Some(target) = self.rows.get(&row) else {
            return Err(JudgeError::UnknownRow { row });
        };
        Ok(target.planning_value.as_ref())
    }
}

/// Strictly parse `registries/perf_targets.toml` bytes via `frob_core::registry::toml`.
pub fn parse_bytes(bytes: &[u8]) -> Result<TargetRegistry, TargetError> {
    let root = toml::parse(bytes)?;
    let mut top = Fields::new(root, "perf_targets.toml");
    let schema = top.require_str("schema")?;
    let target_items = top.require_array("target")?;
    top.finish()?;

    if schema != TARGETS_SCHEMA {
        return Err(TargetError::UnsupportedSchema {
            expected: TARGETS_SCHEMA,
            found: schema,
        });
    }

    let mut rows = BTreeMap::new();
    for (idx, spanned) in target_items.into_iter().enumerate() {
        let label = format!("target[{idx}]");
        let Value::Table(tbl) = spanned.value else {
            return Err(TargetError::InvalidField {
                table: "perf_targets.toml".to_owned(),
                field: label,
                reason: format!("expected table, found {}", spanned.value.type_name()),
            });
        };
        let row = parse_target_row(&label, tbl)?;
        if rows.contains_key(&row.row) {
            return Err(TargetError::InvalidField {
                table: label,
                field: "row".to_owned(),
                reason: format!("duplicate target row {}", row.row),
            });
        }
        rows.insert(row.row, row);
    }

    Ok(TargetRegistry { schema, rows })
}

/// Strictly parse `registries/perf_targets.toml` from a UTF-8 string slice.
pub fn parse_str(text: &str) -> Result<TargetRegistry, TargetError> {
    parse_bytes(text.as_bytes())
}

/// Load and strictly parse `registries/perf_targets.toml` from disk.
pub fn load(path: &Path) -> Result<TargetRegistry, TargetError> {
    let bytes = fs::read(path).map_err(|err| TargetError::Io {
        path: path.display().to_string(),
        message: err.to_string(),
    })?;
    parse_bytes(&bytes)
}

fn parse_target_row(label: &str, tbl: Table) -> Result<TargetRow, TargetError> {
    let mut f = Fields::new(tbl, label);
    let row_i = f.require_int("row")?;
    let workload = f.require_str("workload")?;
    let scope = f.require_str("scope")?;
    let hardware = f.require_str("hardware")?;
    let unit_raw = f.require_str("unit")?;
    let decimation_raw = f.opt_str("decimation")?;
    let tol_tbl = f.require_table("tolerance")?;
    let floor_tbl = f.opt_table("interim_floor")?;
    let amb_tbl = f.opt_table("ambition")?;
    let plan_tbl = f.opt_table("planning_value")?;
    let sub_arr = f.opt_array("sub_targets")?;
    let ref_tbl = f.opt_table("incumbent_reference")?;
    let gated_arr = f.require_array("gated_in")?;
    let carrier = f.require_str("carrier")?;
    f.finish()?;

    let row = parse_row_u8(row_i, label, "row")?;
    let row_label = format!("target[row={row}]");

    if !matches!(workload.as_str(), "Q" | "H" | "mixed-3" | "none") {
        return Err(TargetError::InvalidField {
            table: row_label,
            field: "workload".to_owned(),
            reason: format!("unexpected workload `{workload}`"),
        });
    }
    if scope.trim().is_empty() {
        return Err(TargetError::InvalidField {
            table: row_label,
            field: "scope".to_owned(),
            reason: "scope must be non-empty".to_owned(),
        });
    }
    if hardware.trim().is_empty() {
        return Err(TargetError::InvalidField {
            table: row_label,
            field: "hardware".to_owned(),
            reason: "hardware must be non-empty".to_owned(),
        });
    }

    let unit = TargetUnit::parse(&unit_raw).ok_or_else(|| TargetError::InvalidField {
        table: row_label.clone(),
        field: "unit".to_owned(),
        reason: format!("unknown target unit `{unit_raw}`"),
    })?;

    let decimation = match decimation_raw {
        Some(s) => Some(
            Decimation::parse(&s).map_err(|e| TargetError::InvalidField {
                table: row_label.clone(),
                field: "decimation".to_owned(),
                reason: e.to_string(),
            })?,
        ),
        None => None,
    };

    // ADR-PERF-02 loader check: any step-family unit (world_steps_per_s, env_steps_per_s,
    // world_s_per_wall_s) without a decimation fails at load time.
    if unit.requires_decimation() && decimation.is_none() {
        return Err(TargetError::MissingDecimation {
            row,
            unit: unit.as_str(),
        });
    }

    let tolerance = parse_tolerance_table(&format!("{row_label}.tolerance"), tol_tbl)?;
    let interim_floor = match floor_tbl {
        Some(t) => Some(parse_interim_floor_table(
            &format!("{row_label}.interim_floor"),
            t,
        )?),
        None => None,
    };
    let ambition = match amb_tbl {
        Some(t) => Some(parse_ambition_table(&format!("{row_label}.ambition"), t)?),
        None => None,
    };
    let planning_value = match plan_tbl {
        Some(t) => Some(parse_planning_table(
            &format!("{row_label}.planning_value"),
            t,
        )?),
        None => None,
    };

    let mut sub_targets = Vec::new();
    if let Some(items) = sub_arr {
        for (i, sp) in items.into_iter().enumerate() {
            let sub_label = format!("{row_label}.sub_targets[{i}]");
            let Value::Table(t) = sp.value else {
                return Err(TargetError::InvalidField {
                    table: row_label,
                    field: sub_label,
                    reason: format!("expected table, found {}", sp.value.type_name()),
                });
            };
            sub_targets.push(parse_sub_target_table(&sub_label, t)?);
        }
    }

    let incumbent_reference = match ref_tbl {
        Some(t) => Some(parse_incumbent_ref_table(
            &format!("{row_label}.incumbent_reference"),
            t,
        )?),
        None => None,
    };

    let mut gated_in = Vec::with_capacity(gated_arr.len());
    for (i, sp) in gated_arr.into_iter().enumerate() {
        let Some(g) = sp.value.as_str() else {
            return Err(TargetError::InvalidField {
                table: row_label,
                field: format!("gated_in[{i}]"),
                reason: format!("expected string, found {}", sp.value.type_name()),
            });
        };
        if g.trim().is_empty() {
            return Err(TargetError::InvalidField {
                table: row_label,
                field: format!("gated_in[{i}]"),
                reason: "gate identifier must be non-empty".to_owned(),
            });
        }
        gated_in.push(g.to_owned());
    }

    Ok(TargetRow {
        row,
        workload,
        scope,
        hardware,
        unit,
        decimation,
        tolerance,
        interim_floor,
        ambition,
        planning_value,
        sub_targets,
        incumbent_reference,
        gated_in,
        carrier,
    })
}

fn parse_tolerance_table(label: &str, tbl: Table) -> Result<ToleranceSpec, TargetError> {
    let mut f = Fields::new(tbl, label);
    let kind_raw = f.require_str("kind")?;
    let status_raw = f.opt_str("status")?;
    let val_raw = f.opt_str("value")?;
    let dir_raw = f.opt_str("direction")?;
    let comparator = f.opt_str("comparator")?;
    let setting_kind_raw = f.opt_str("setting_kind")?;
    let ref_row_i = f.opt_int("reference_row")?;
    let note = f.opt_str("note")?;
    f.finish()?;

    let kind = ToleranceKind::parse(&kind_raw).ok_or_else(|| TargetError::InvalidField {
        table: label.to_owned(),
        field: "kind".to_owned(),
        reason: format!("unknown tolerance kind `{kind_raw}`"),
    })?;
    let status = parse_opt_number_status(status_raw.as_deref(), label, "status")?;
    let value = match val_raw {
        Some(s) => Some(DecimalValue::parse(&s, label, "value")?),
        None => None,
    };
    let direction = match dir_raw {
        Some(s) => Some(
            Direction::parse(&s).ok_or_else(|| TargetError::InvalidField {
                table: label.to_owned(),
                field: "direction".to_owned(),
                reason: format!("unknown direction `{s}`"),
            })?,
        ),
        None => None,
    };
    let setting_kind = match setting_kind_raw {
        Some(s) => Some(
            SettingKind::parse(&s).map_err(|e| TargetError::InvalidField {
                table: label.to_owned(),
                field: "setting_kind".to_owned(),
                reason: e.to_string(),
            })?,
        ),
        None => None,
    };
    let reference_row = match ref_row_i {
        Some(n) => Some(parse_row_u8(n, label, "reference_row")?),
        None => None,
    };

    Ok(ToleranceSpec {
        kind,
        status,
        value,
        direction,
        comparator,
        setting_kind,
        reference_row,
        note,
    })
}

fn parse_interim_floor_table(label: &str, tbl: Table) -> Result<InterimFloorSpec, TargetError> {
    let mut f = Fields::new(tbl, label);
    let status_raw = f.opt_str("status")?;
    let val_raw = f.require_str("value")?;
    let unit_raw = f.require_str("unit")?;
    let dir_raw = f.require_str("direction")?;
    let note = f.opt_str("note")?;
    f.finish()?;

    let status = parse_opt_number_status(status_raw.as_deref(), label, "status")?;
    let value = DecimalValue::parse(&val_raw, label, "value")?;
    let unit = TargetUnit::parse(&unit_raw).ok_or_else(|| TargetError::InvalidField {
        table: label.to_owned(),
        field: "unit".to_owned(),
        reason: format!("unknown unit `{unit_raw}`"),
    })?;
    let direction = Direction::parse(&dir_raw).ok_or_else(|| TargetError::InvalidField {
        table: label.to_owned(),
        field: "direction".to_owned(),
        reason: format!("unknown direction `{dir_raw}`"),
    })?;

    Ok(InterimFloorSpec {
        status,
        value,
        unit,
        direction,
        note,
    })
}

fn parse_ambition_table(label: &str, tbl: Table) -> Result<AmbitionSpec, TargetError> {
    let mut f = Fields::new(tbl, label);
    let status_raw = f.opt_str("status")?;
    let kind_raw = f.require_str("kind")?;
    let val_raw = f.require_str("value")?;
    let dir_raw = f.require_str("direction")?;
    let unit_raw = f.opt_str("unit")?;
    let note = f.opt_str("note")?;
    f.finish()?;

    let status = parse_opt_number_status(status_raw.as_deref(), label, "status")?;
    let kind = ToleranceKind::parse(&kind_raw).ok_or_else(|| TargetError::InvalidField {
        table: label.to_owned(),
        field: "kind".to_owned(),
        reason: format!("unknown ambition kind `{kind_raw}`"),
    })?;
    let value = DecimalValue::parse(&val_raw, label, "value")?;
    let direction = Direction::parse(&dir_raw).ok_or_else(|| TargetError::InvalidField {
        table: label.to_owned(),
        field: "direction".to_owned(),
        reason: format!("unknown direction `{dir_raw}`"),
    })?;
    let unit = match unit_raw {
        Some(s) => Some(
            TargetUnit::parse(&s).ok_or_else(|| TargetError::InvalidField {
                table: label.to_owned(),
                field: "unit".to_owned(),
                reason: format!("unknown unit `{s}`"),
            })?,
        ),
        None => None,
    };

    Ok(AmbitionSpec {
        status,
        kind,
        value,
        direction,
        unit,
        note,
    })
}

fn parse_planning_table(label: &str, tbl: Table) -> Result<PlanningValue, TargetError> {
    let mut f = Fields::new(tbl, label);
    let status_raw = f.opt_str("status")?;
    let tol_raw = f.opt_str("tolerance")?;
    let amb_raw = f.opt_str("ambition")?;
    let unit_raw = f.require_str("unit")?;
    let note = f.opt_str("note")?;
    f.finish()?;

    let status = parse_opt_number_status(status_raw.as_deref(), label, "status")?;
    let tolerance = match tol_raw {
        Some(s) => Some(DecimalValue::parse(&s, label, "tolerance")?),
        None => None,
    };
    let ambition = match amb_raw {
        Some(s) => Some(DecimalValue::parse(&s, label, "ambition")?),
        None => None,
    };
    let unit = TargetUnit::parse(&unit_raw).ok_or_else(|| TargetError::InvalidField {
        table: label.to_owned(),
        field: "unit".to_owned(),
        reason: format!("unknown unit `{unit_raw}`"),
    })?;

    Ok(PlanningValue {
        status,
        tolerance,
        ambition,
        unit,
        note,
    })
}

fn parse_sub_target_table(label: &str, tbl: Table) -> Result<SubTarget, TargetError> {
    let mut f = Fields::new(tbl, label);
    let id = f.require_str("id")?;
    let sub_label = f.require_str("label")?;
    let status_raw = f.opt_str("status")?;
    let val_raw = f.opt_str("value")?;
    let dir_raw = f.opt_str("direction")?;
    let percentile = f.opt_str("percentile")?;
    let unit_raw = f.require_str("unit")?;
    f.finish()?;

    let status = parse_opt_number_status(status_raw.as_deref(), label, "status")?;
    let value = match val_raw {
        Some(s) => Some(DecimalValue::parse(&s, label, "value")?),
        None => None,
    };
    let direction = match dir_raw {
        Some(s) => Some(
            Direction::parse(&s).ok_or_else(|| TargetError::InvalidField {
                table: label.to_owned(),
                field: "direction".to_owned(),
                reason: format!("unknown direction `{s}`"),
            })?,
        ),
        None => None,
    };
    let unit = TargetUnit::parse(&unit_raw).ok_or_else(|| TargetError::InvalidField {
        table: label.to_owned(),
        field: "unit".to_owned(),
        reason: format!("unknown unit `{unit_raw}`"),
    })?;

    Ok(SubTarget {
        id,
        label: sub_label,
        status,
        value,
        direction,
        percentile,
        unit,
    })
}

fn parse_incumbent_ref_table(label: &str, tbl: Table) -> Result<IncumbentReference, TargetError> {
    let mut f = Fields::new(tbl, label);
    let text = f.require_str("text")?;
    let source_mark = f.require_str("source_mark")?;
    let status_raw = f.require_str("status")?;
    let pub_raw = f.opt_str("published_value")?;
    let pub2_raw = f.opt_str("published_secondary_value")?;
    let disclosure_id = f.opt_str("disclosure_id")?;
    let loaded_host_lower_bound = f.opt_bool("loaded_host_lower_bound")?.unwrap_or(false);
    f.finish()?;

    if !matches!(
        source_mark.as_str(),
        "P" | "S" | "F" | "R" | "M" | "U" | "local" | "none"
    ) {
        return Err(TargetError::InvalidField {
            table: label.to_owned(),
            field: "source_mark".to_owned(),
            reason: format!("unknown Appendix H source mark `{source_mark}`"),
        });
    }
    let status = ReferenceStatus::parse(&status_raw).ok_or_else(|| TargetError::InvalidField {
        table: label.to_owned(),
        field: "status".to_owned(),
        reason: format!("expected `to-re-read` or `measured`, found `{status_raw}`"),
    })?;
    let published_value = match pub_raw {
        Some(s) => Some(DecimalValue::parse(&s, label, "published_value")?),
        None => None,
    };
    let published_secondary_value = match pub2_raw {
        Some(s) => Some(DecimalValue::parse(&s, label, "published_secondary_value")?),
        None => None,
    };

    Ok(IncumbentReference {
        text,
        source_mark,
        status,
        published_value,
        published_secondary_value,
        disclosure_id,
        loaded_host_lower_bound,
    })
}

fn parse_opt_number_status(
    raw: Option<&str>,
    table: &str,
    field: &str,
) -> Result<Option<NumberStatus>, TargetError> {
    match raw {
        None => Ok(None),
        Some(s) => NumberStatus::parse(s)
            .map(Some)
            .ok_or_else(|| TargetError::InvalidField {
                table: table.to_owned(),
                field: field.to_owned(),
                reason: format!("unknown number status `{s}`"),
            }),
    }
}

fn parse_row_u8(n: i64, table: &str, field: &str) -> Result<u8, TargetError> {
    if !(1..=22).contains(&n) {
        return Err(TargetError::InvalidField {
            table: table.to_owned(),
            field: field.to_owned(),
            reason: format!("row number {n} outside 1..=22"),
        });
    }
    Ok(n as u8)
}

/// Run all N36-01 semantic and consistency lints over `registry`, cross-checking
/// reference workloads and comparator settings against `workloads` when provided.
#[must_use]
pub fn lint_registry(
    registry: &TargetRegistry,
    workloads: Option<&WorkloadRegistry>,
) -> Vec<TargetFinding> {
    let mut findings = Vec::new();

    // 1. Exact rows 1..=22 present.
    let expected_rows: Vec<u8> = (1..=22).collect();
    let actual_rows: Vec<u8> = registry.rows.keys().copied().collect();
    if actual_rows != expected_rows {
        findings.push(TargetFinding::new(
            TargetFindingKind::RowSequenceInvalid,
            None,
            format!("expected exact rows 1..=22, found {actual_rows:?}"),
        ));
    }

    for (&row_num, target) in &registry.rows {
        // 2. Every row names its carrier bead.
        if target.carrier.trim().is_empty() {
            findings.push(TargetFinding::new(
                TargetFindingKind::MissingCarrier,
                Some(row_num),
                "target row must name a non-empty carrier bead",
            ));
        }

        // 3. Step-family units require a decimation (ADR-PERF-02).
        if target.unit.requires_decimation() && target.decimation.is_none() {
            findings.push(TargetFinding::new(
                TargetFindingKind::MissingDecimation,
                Some(row_num),
                format!("unit `{}` requires an explicit decimation", target.unit),
            ));
        }

        // 4. Every number declares its ADR-PERF-03 status cleanly.
        lint_row_statuses(row_num, target, &mut findings);

        // 5. Row 1 interim-floor guard: row 1's binding tolerance must be `ratio_to_incumbent`
        //    (the 2.0 M interim floor may never be promoted to status `tolerance`).
        if row_num == 1 && target.tolerance.kind != ToleranceKind::RatioToIncumbent {
            findings.push(TargetFinding::new(
                TargetFindingKind::InterimFloorPromotedToTolerance,
                Some(1),
                format!(
                    "row 1 binding tolerance must be `ratio_to_incumbent`, not `{}` (the 2.0 M interim floor is not the tolerance)",
                    target.tolerance.kind.as_str()
                ),
            ));
        }

        // 6. Cross-check reference workloads (`Q`, `H`) and comparator settings against N36-02.
        if let Some(wl_reg) = workloads
            && matches!(target.workload.as_str(), "Q" | "H")
        {
            match wl_reg.get(&target.workload) {
                None => findings.push(TargetFinding::new(
                    TargetFindingKind::WorkloadNotFound,
                    Some(row_num),
                    format!("workload `{}` not found in workloads.toml", target.workload),
                )),
                Some(wl) => {
                    if let Some(dec) = target.decimation
                        && dec.as_rational() != wl.decimation
                    {
                        findings.push(TargetFinding::new(
                            TargetFindingKind::DecimationDisagreesWithWorkload,
                            Some(row_num),
                            format!(
                                "row decimation `{dec}` disagrees with workload `{}` decimation `{}`",
                                wl.name, wl.decimation
                            ),
                        ));
                    }
                    if target.tolerance.kind == ToleranceKind::RatioToIncumbent
                        && let (Some(comp), Some(kind)) = (
                            target.tolerance.comparator.as_deref(),
                            target.tolerance.setting_kind,
                        )
                        && wl_reg.setting(comp, &wl.name, kind).is_none()
                    {
                        findings.push(TargetFinding::new(
                            TargetFindingKind::ComparatorSettingNotFound,
                            Some(row_num),
                            format!(
                                "comparator setting ({comp}, {}, {}) not found in workloads.toml",
                                wl.name,
                                kind.as_str()
                            ),
                        ));
                    }
                }
            }
        }

        // 7. Incumbent reference status hygiene (`measured` requires a disclosure_id).
        if let Some(ref inc) = target.incumbent_reference
            && inc.status == ReferenceStatus::Measured
            && inc
                .disclosure_id
                .as_deref()
                .is_none_or(|s| s.trim().is_empty())
        {
            findings.push(TargetFinding::new(
                TargetFindingKind::ReferenceStatusInvalid,
                Some(row_num),
                "incumbent_reference with status `measured` requires a non-empty `disclosure_id`",
            ));
        }
    }

    // 8. Consistency notes 1 & 2 (all 9 arithmetic products + row 13 loaded-host lower-bound note).
    lint_consistency_notes(registry, &mut findings);

    findings
}

fn lint_row_statuses(row_num: u8, target: &TargetRow, findings: &mut Vec<TargetFinding>) {
    let tol = &target.tolerance;
    match tol.kind {
        ToleranceKind::ReportedOnly => {
            if tol.status != Some(NumberStatus::ReportedOnly) {
                findings.push(TargetFinding::new(
                    TargetFindingKind::WrongNumberStatus,
                    Some(row_num),
                    "reported_only tolerance block must carry status = \"reported-only\"",
                ));
            }
        }
        ToleranceKind::RatioToIncumbent | ToleranceKind::RatioToRow | ToleranceKind::Absolute => {
            match tol.status {
                None => findings.push(TargetFinding::new(
                    TargetFindingKind::MissingNumberStatus,
                    Some(row_num),
                    "tolerance value is missing explicit ADR-PERF-03 `status`",
                )),
                Some(NumberStatus::Tolerance) => {}
                Some(other) => findings.push(TargetFinding::new(
                    TargetFindingKind::WrongNumberStatus,
                    Some(row_num),
                    format!(
                        "tolerance block must have status = \"tolerance\", found \"{}\"",
                        other.as_str()
                    ),
                )),
            }
            if tol.value.is_none() || tol.direction.is_none() {
                findings.push(TargetFinding::new(
                    TargetFindingKind::IncompleteToleranceSpec,
                    Some(row_num),
                    "non-reported_only tolerance requires both `value` and `direction`",
                ));
            }
            if tol.kind == ToleranceKind::RatioToIncumbent
                && (tol
                    .comparator
                    .as_deref()
                    .is_none_or(|s| s.trim().is_empty())
                    || tol.setting_kind.is_none())
            {
                findings.push(TargetFinding::new(
                    TargetFindingKind::IncompleteToleranceSpec,
                    Some(row_num),
                    "ratio_to_incumbent tolerance requires `comparator` and `setting_kind`",
                ));
            }
            if tol.kind == ToleranceKind::RatioToRow && tol.reference_row.is_none() {
                findings.push(TargetFinding::new(
                    TargetFindingKind::IncompleteToleranceSpec,
                    Some(row_num),
                    "ratio_to_row tolerance requires `reference_row`",
                ));
            }
        }
    }

    if let Some(ref floor) = target.interim_floor {
        match floor.status {
            None => findings.push(TargetFinding::new(
                TargetFindingKind::MissingNumberStatus,
                Some(row_num),
                "interim_floor is missing explicit `status = \"interim-floor\"`",
            )),
            Some(NumberStatus::InterimFloor) => {}
            Some(NumberStatus::Tolerance) => findings.push(TargetFinding::new(
                TargetFindingKind::InterimFloorPromotedToTolerance,
                Some(row_num),
                "interim_floor must never carry status = \"tolerance\"",
            )),
            Some(other) => findings.push(TargetFinding::new(
                TargetFindingKind::WrongNumberStatus,
                Some(row_num),
                format!(
                    "interim_floor must have status = \"interim-floor\", found \"{}\"",
                    other.as_str()
                ),
            )),
        }
    }

    if let Some(ref amb) = target.ambition {
        match amb.status {
            None => findings.push(TargetFinding::new(
                TargetFindingKind::MissingNumberStatus,
                Some(row_num),
                "ambition is missing explicit `status = \"ambition\"`",
            )),
            Some(NumberStatus::Ambition) => {}
            Some(other) => findings.push(TargetFinding::new(
                TargetFindingKind::WrongNumberStatus,
                Some(row_num),
                format!(
                    "ambition must have status = \"ambition\", found \"{}\"",
                    other.as_str()
                ),
            )),
        }
    }

    if let Some(ref pv) = target.planning_value {
        match pv.status {
            None => findings.push(TargetFinding::new(
                TargetFindingKind::MissingNumberStatus,
                Some(row_num),
                "planning_value is missing explicit `status = \"planning-value\"`",
            )),
            Some(NumberStatus::PlanningValue) => {}
            Some(other) => findings.push(TargetFinding::new(
                TargetFindingKind::WrongNumberStatus,
                Some(row_num),
                format!(
                    "planning_value must have status = \"planning-value\", found \"{}\"",
                    other.as_str()
                ),
            )),
        }
    }

    for sub in &target.sub_targets {
        if sub.status.is_none() {
            findings.push(TargetFinding::new(
                TargetFindingKind::MissingNumberStatus,
                Some(row_num),
                format!("sub_target `{}` is missing explicit `status`", sub.id),
            ));
        }
    }
}

fn lint_consistency_notes(registry: &TargetRegistry, findings: &mut Vec<TargetFinding>) {
    // Consistency note 1: rows 1, 2, 3 mutual consistency in env-steps/s at d = 10.
    if let (Some(r1), Some(r2), Some(r3)) = (registry.get(1), registry.get(2), registry.get(3)) {
        let d1 = r1.decimation.map(|d| u128::from(d.num())).unwrap_or(10);
        if let (Some(pv1_amb), Some(fl1)) = (
            r1.planning_value.as_ref().and_then(|p| p.ambition.as_ref()),
            r1.interim_floor.as_ref(),
        ) {
            let env_amb = pv1_amb
                .to_exact_rational()
                .ok()
                .and_then(|r| r.checked_mul_ratio(1, d1).ok());
            let env_fl = fl1
                .value
                .to_exact_rational()
                .ok()
                .and_then(|r| r.checked_mul_ratio(1, d1).ok());

            if let (Some(env_amb), Some(amb2), Some(pv2_amb)) = (
                env_amb,
                r2.ambition
                    .as_ref()
                    .and_then(|a| a.value.to_exact_rational().ok()),
                r2.planning_value
                    .as_ref()
                    .and_then(|p| p.ambition.as_ref())
                    .and_then(|v| v.to_exact_rational().ok()),
            ) && env_amb.checked_mul_ratio(amb2.num(), amb2.den()).ok() != Some(pv2_amb)
            {
                findings.push(TargetFinding::new(
                    TargetFindingKind::ConsistencyViolation,
                    Some(2),
                    "row 2 ambition planning value disagrees with row 2 ambition * row 1 env-steps/s",
                ));
            }
            if let (Some(env_amb), Some(amb3), Some(pv3_amb)) = (
                env_amb,
                r3.ambition
                    .as_ref()
                    .and_then(|a| a.value.to_exact_rational().ok()),
                r3.planning_value
                    .as_ref()
                    .and_then(|p| p.ambition.as_ref())
                    .and_then(|v| v.to_exact_rational().ok()),
            ) && env_amb.checked_mul_ratio(amb3.num(), amb3.den()).ok() != Some(pv3_amb)
            {
                findings.push(TargetFinding::new(
                    TargetFindingKind::ConsistencyViolation,
                    Some(3),
                    "row 3 ambition planning value disagrees with row 3 ambition * row 1 env-steps/s",
                ));
            }
            if let (Some(env_fl), Some(tol2), Some(fl2)) = (
                env_fl,
                r2.tolerance
                    .value
                    .as_ref()
                    .and_then(|v| v.to_exact_rational().ok()),
                r2.interim_floor
                    .as_ref()
                    .and_then(|f| f.value.to_exact_rational().ok()),
            ) && env_fl.checked_mul_ratio(tol2.num(), tol2.den()).ok() != Some(fl2)
            {
                findings.push(TargetFinding::new(
                    TargetFindingKind::ConsistencyViolation,
                    Some(2),
                    "row 2 interim floor disagrees with row 2 tolerance * row 1 env-steps/s interim floor",
                ));
            }
            if let (Some(env_fl), Some(tol3), Some(fl3)) = (
                env_fl,
                r3.tolerance
                    .value
                    .as_ref()
                    .and_then(|v| v.to_exact_rational().ok()),
                r3.interim_floor
                    .as_ref()
                    .and_then(|f| f.value.to_exact_rational().ok()),
            ) && env_fl.checked_mul_ratio(tol3.num(), tol3.den()).ok() != Some(fl3)
            {
                findings.push(TargetFinding::new(
                    TargetFindingKind::ConsistencyViolation,
                    Some(3),
                    "row 3 interim floor disagrees with row 3 tolerance * row 1 env-steps/s interim floor",
                ));
            }
        }
    }

    // Row 4: 1.25 * 410 s = 512.5 s, rounded half-up to 513 s.
    if let Some(r4) = registry.get(4)
        && let (Some(tol4), Some(pv4_amb), Some(pv4_tol)) = (
            r4.tolerance.value.as_ref(),
            r4.planning_value.as_ref().and_then(|p| p.ambition.as_ref()),
            r4.planning_value
                .as_ref()
                .and_then(|p| p.tolerance.as_ref()),
        )
    {
        let exact = tol4.as_f64() * pv4_amb.as_f64();
        let rounded = (exact + 0.5).floor();
        if (pv4_tol.as_f64() - rounded).abs() > 1e-9 {
            findings.push(TargetFinding::new(
                TargetFindingKind::ConsistencyViolation,
                Some(4),
                format!(
                    "row 4 planning tolerance {} disagrees with rounded {} * {} = {rounded}",
                    pv4_tol.as_str(),
                    tol4.as_str(),
                    pv4_amb.as_str()
                ),
            ));
        }
    }

    // Consistency note 2: Row 13 planning tolerance = 0.5 * 550 k = 275 k AND carries
    // `loaded_host_lower_bound = true` because the incumbent rate was measured under load.
    if let Some(r13) = registry.get(13) {
        if let (Some(tol13), Some(pv13_amb), Some(pv13_tol)) = (
            r13.tolerance
                .value
                .as_ref()
                .and_then(|v| v.to_exact_rational().ok()),
            r13.planning_value
                .as_ref()
                .and_then(|p| p.ambition.as_ref())
                .and_then(|v| v.to_exact_rational().ok()),
            r13.planning_value
                .as_ref()
                .and_then(|p| p.tolerance.as_ref())
                .and_then(|v| v.to_exact_rational().ok()),
        ) && pv13_amb.checked_mul_ratio(tol13.num(), tol13.den()).ok() != Some(pv13_tol)
        {
            findings.push(TargetFinding::new(
                TargetFindingKind::ConsistencyViolation,
                Some(13),
                "row 13 planning tolerance disagrees with tolerance * planning ambition",
            ));
        }
        let marked = r13
            .incumbent_reference
            .as_ref()
            .is_some_and(|r| r.loaded_host_lower_bound);
        if !marked {
            findings.push(TargetFinding::new(
                TargetFindingKind::Row13LoadedHostFlagMissing,
                Some(13),
                "row 13 incumbent_reference must mark loaded_host_lower_bound = true (consistency note 2)",
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use frob_core::json::{Limits, admit};
    use frob_core::perf::workload;

    use super::*;

    fn repo_root() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .expect("crates/frob-conformance sits two levels below repo root")
            .to_path_buf()
    }

    fn load_committed() -> (TargetRegistry, WorkloadRegistry) {
        let root = repo_root();
        let targets = load(&root.join(PERF_TARGETS_PATH))
            .expect("load committed registries/perf_targets.toml");
        let workloads = workload::load(&root.join("registries/workloads.toml"))
            .expect("load committed registries/workloads.toml");
        (targets, workloads)
    }

    #[test]
    fn rows_present_1_to_22() {
        let root = repo_root();
        let (reg, wl) = load_committed();
        assert_eq!(reg.rows.len(), EXPECTED_ROW_COUNT);
        for row_num in 1..=22u8 {
            assert!(reg.get(row_num).is_some(), "missing row {row_num}");
        }

        let findings = lint_registry(&reg, Some(&wl));
        assert!(
            findings.is_empty(),
            "committed perf_targets.toml failed lint: {findings:?}"
        );

        // Every incumbent reference starts at status = "to-re-read".
        for (&row_num, row) in &reg.rows {
            if let Some(ref inc) = row.incumbent_reference {
                assert_eq!(
                    inc.status,
                    ReferenceStatus::ToReRead,
                    "row {row_num} incumbent_reference must start as to-re-read"
                );
            }
        }

        // Schema file exists and parses as valid JSON declaring schema const `frob.perf.targets/1`.
        let schema_bytes = fs::read(root.join(PERF_TARGETS_SCHEMA_PATH))
            .expect("schemas/registries/perf_targets.schema.json must exist");
        let admitted = admit(&schema_bytes, &Limits::default())
            .expect("perf_targets.schema.json must be valid JSON");
        assert_eq!(
            admitted
                .value()
                .get("properties")
                .and_then(|p| p.get("schema"))
                .and_then(|s| s.get("const"))
                .and_then(|c| c.as_str()),
            Some(TARGETS_SCHEMA)
        );

        // ADR-PERF-01..05 decision record exists and documents all 5 decisions.
        let adr_text = fs::read_to_string(root.join(PERF_ADR_PATH))
            .expect("docs/adr/ADR-PERF-01..05.md must exist");
        for id in [
            "ADR-PERF-01",
            "ADR-PERF-02",
            "ADR-PERF-03",
            "ADR-PERF-04",
            "ADR-PERF-05",
        ] {
            assert!(adr_text.contains(id), "ADR doc missing {id}");
        }
    }

    #[test]
    fn every_number_has_status() {
        let (reg, wl) = load_committed();
        assert!(lint_registry(&reg, Some(&wl)).is_empty());

        for (&row_num, row) in &reg.rows {
            assert!(
                row.tolerance.status.is_some(),
                "row {row_num} tolerance missing status"
            );
            if let Some(ref floor) = row.interim_floor {
                assert_eq!(floor.status, Some(NumberStatus::InterimFloor));
            }
            if let Some(ref amb) = row.ambition {
                assert_eq!(amb.status, Some(NumberStatus::Ambition));
            }
            if let Some(ref pv) = row.planning_value {
                assert_eq!(pv.status, Some(NumberStatus::PlanningValue));
            }
            for sub in &row.sub_targets {
                assert!(
                    sub.status.is_some(),
                    "row {row_num} sub_target {} missing status",
                    sub.id
                );
            }
        }

        let raw = fs::read_to_string(repo_root().join(PERF_TARGETS_PATH)).expect("read toml");

        // Negative control 1: recording row 1's 2.0 M interim floor as status `tolerance` fails lint.
        let promoted = raw.replacen(
            "tolerance = { kind = \"ratio_to_incumbent\", status = \"tolerance\", value = \"0.67\", direction = \"at_least\", comparator = \"mujoco-warp\", setting_kind = \"matched\", note = \"at least 0.67 of incumbent at matched settings on trj; env-steps/s is one tenth\" }",
            "tolerance = { kind = \"absolute\", status = \"tolerance\", value = \"2000000\", direction = \"at_least\" }",
            1,
        );
        let parsed_promoted = parse_str(&promoted).expect("parse promoted floor scratch");
        let findings = lint_registry(&parsed_promoted, Some(&wl));
        assert!(
            findings
                .iter()
                .any(|f| f.kind == TargetFindingKind::InterimFloorPromotedToTolerance),
            "promoting row 1 interim floor to tolerance must fail lint, got: {findings:?}"
        );

        // Negative control 2: removing `status` from a numeric tolerance block fails lint.
        let no_status = raw.replacen("status = \"ambition\", ", "", 1);
        let parsed_no_status = parse_str(&no_status).expect("parse missing status scratch");
        let findings = lint_registry(&parsed_no_status, Some(&wl));
        assert!(
            findings
                .iter()
                .any(|f| f.kind == TargetFindingKind::MissingNumberStatus),
            "number without status must fail lint, got: {findings:?}"
        );

        // Negative control 3: a world_steps_per_s row with no decimation fails the loader.
        let no_dec = raw.replacen("decimation = \"10/1\"\n", "", 1);
        let err =
            parse_str(&no_dec).expect_err("world_steps_per_s without decimation must fail loader");
        assert!(
            matches!(
                err,
                TargetError::MissingDecimation {
                    row: 1,
                    unit: "world_steps_per_s"
                }
            ),
            "expected MissingDecimation on row 1, got: {err:?}"
        );
    }

    #[test]
    fn row1_interim_floor_then_tolerance() {
        let (reg, _) = load_committed();

        // No incumbent measurement -> (2.0 M, interim-floor).
        let floor = reg.threshold(1, None).expect("row 1 interim floor");
        assert_eq!(floor.status, ThresholdStatus::InterimFloor);
        assert_eq!(floor.value, Some(2_000_000.0));
        assert_eq!(floor.unit, TargetUnit::Rate(RateUnit::WorldStepsPerSec));

        // Incumbent 3.6 M measured -> (0.67 * 3.6 M = 2.412 M, tolerance).
        let tol = reg
            .threshold(1, Some(3_600_000.0))
            .expect("row 1 measured tolerance");
        assert_eq!(tol.status, ThresholdStatus::Tolerance);
        let val = tol.value.expect("measured tolerance value");
        assert!(
            (val - 2_412_000.0).abs() < 1e-6,
            "expected 2_412_000.0, got {val}"
        );
        assert_eq!(tol.unit, TargetUnit::Rate(RateUnit::WorldStepsPerSec));
    }

    #[test]
    fn planning_values_consistent() {
        let (reg, _) = load_committed();

        let r1 = reg.get(1).expect("row 1");
        let r2 = reg.get(2).expect("row 2");
        let r3 = reg.get(3).expect("row 3");
        let r4 = reg.get(4).expect("row 4");
        let r7 = reg.get(7).expect("row 7");
        let r8 = reg.get(8).expect("row 8");
        let r13 = reg.get(13).expect("row 13");

        let d1 = u128::from(r1.decimation.expect("row 1 decimation").num());
        assert_eq!(d1, 10);

        let r1_amb_ws = r1
            .planning_value
            .as_ref()
            .unwrap()
            .ambition
            .as_ref()
            .unwrap()
            .to_exact_rational()
            .unwrap();
        let r1_amb_env = r1_amb_ws.checked_mul_ratio(1, d1).unwrap();
        assert_eq!(r1_amb_env.exact_u64(), Some(400_000));

        let r1_floor_ws = r1
            .interim_floor
            .as_ref()
            .unwrap()
            .value
            .to_exact_rational()
            .unwrap();
        let r1_floor_env = r1_floor_ws.checked_mul_ratio(1, d1).unwrap();
        assert_eq!(r1_floor_env.exact_u64(), Some(200_000));

        // 1. 0.85 * 400 k = 340 k
        let r2_amb_ratio = r2
            .ambition
            .as_ref()
            .unwrap()
            .value
            .to_exact_rational()
            .unwrap();
        let r2_amb_plan = r2
            .planning_value
            .as_ref()
            .unwrap()
            .ambition
            .as_ref()
            .unwrap()
            .to_exact_rational()
            .unwrap();
        assert_eq!(
            r1_amb_env
                .checked_mul_ratio(r2_amb_ratio.num(), r2_amb_ratio.den())
                .unwrap(),
            r2_amb_plan
        );
        assert_eq!(r2_amb_plan.exact_u64(), Some(340_000));

        // 2. 0.70 * 400 k = 280 k
        let r3_amb_ratio = r3
            .ambition
            .as_ref()
            .unwrap()
            .value
            .to_exact_rational()
            .unwrap();
        let r3_amb_plan = r3
            .planning_value
            .as_ref()
            .unwrap()
            .ambition
            .as_ref()
            .unwrap()
            .to_exact_rational()
            .unwrap();
        assert_eq!(
            r1_amb_env
                .checked_mul_ratio(r3_amb_ratio.num(), r3_amb_ratio.den())
                .unwrap(),
            r3_amb_plan
        );
        assert_eq!(r3_amb_plan.exact_u64(), Some(280_000));

        // 3. 0.75 * 200 k = 150 k
        let r2_tol_ratio = r2
            .tolerance
            .value
            .as_ref()
            .unwrap()
            .to_exact_rational()
            .unwrap();
        let r2_floor = r2
            .interim_floor
            .as_ref()
            .unwrap()
            .value
            .to_exact_rational()
            .unwrap();
        assert_eq!(
            r1_floor_env
                .checked_mul_ratio(r2_tol_ratio.num(), r2_tol_ratio.den())
                .unwrap(),
            r2_floor
        );
        assert_eq!(r2_floor.exact_u64(), Some(150_000));

        // 4. 0.60 * 200 k = 120 k
        let r3_tol_ratio = r3
            .tolerance
            .value
            .as_ref()
            .unwrap()
            .to_exact_rational()
            .unwrap();
        let r3_floor = r3
            .interim_floor
            .as_ref()
            .unwrap()
            .value
            .to_exact_rational()
            .unwrap();
        assert_eq!(
            r1_floor_env
                .checked_mul_ratio(r3_tol_ratio.num(), r3_tol_ratio.den())
                .unwrap(),
            r3_floor
        );
        assert_eq!(r3_floor.exact_u64(), Some(120_000));

        // 5. 1.25 * 410 s = 512.5 s, printed 513 s
        let r4_ratio = r4.tolerance.value.as_ref().unwrap().as_f64();
        let r4_amb = r4
            .planning_value
            .as_ref()
            .unwrap()
            .ambition
            .as_ref()
            .unwrap()
            .as_f64();
        let r4_tol_printed = r4
            .planning_value
            .as_ref()
            .unwrap()
            .tolerance
            .as_ref()
            .unwrap()
            .as_f64();
        assert!((r4_ratio * r4_amb - 512.5).abs() < 1e-9);
        assert_eq!((r4_ratio * r4_amb + 0.5).floor(), 513.0);
        assert_eq!(r4_tol_printed, 513.0);

        // 6. 0.5 * 550 k = 275 k (and loaded-host lower-bound flag present)
        let r13_ratio = r13
            .tolerance
            .value
            .as_ref()
            .unwrap()
            .to_exact_rational()
            .unwrap();
        let r13_amb = r13
            .planning_value
            .as_ref()
            .unwrap()
            .ambition
            .as_ref()
            .unwrap()
            .to_exact_rational()
            .unwrap();
        let r13_tol = r13
            .planning_value
            .as_ref()
            .unwrap()
            .tolerance
            .as_ref()
            .unwrap()
            .to_exact_rational()
            .unwrap();
        assert_eq!(
            r13_amb
                .checked_mul_ratio(r13_ratio.num(), r13_ratio.den())
                .unwrap(),
            r13_tol
        );
        assert_eq!(r13_tol.exact_u64(), Some(275_000));
        assert!(
            r13.incumbent_reference
                .as_ref()
                .unwrap()
                .loaded_host_lower_bound
        );

        // 7. 0.67 * 4.0 M = 2.68 M (printed 2.7 M)
        let r1_ratio = r1.tolerance.value.as_ref().unwrap().as_f64();
        let r1_exact = r1_ratio * r1_amb_ws.to_f64();
        assert!((r1_exact - 2_680_000.0).abs() < 1e-6);
        assert_eq!((r1_exact / 100_000.0).round() * 100_000.0, 2_700_000.0);

        // 8. 0.67 * 2.55 M = 1.7085 M (1.71 M; printed 1.7 M)
        let r7_ratio = r7.tolerance.value.as_ref().unwrap().as_f64();
        let r7_amb = r7
            .planning_value
            .as_ref()
            .unwrap()
            .ambition
            .as_ref()
            .unwrap()
            .as_f64();
        let r7_exact = r7_ratio * r7_amb;
        assert!((r7_exact - 1_708_500.0).abs() < 1e-6);
        assert!(((r7_exact / 1_000_000.0) * 100.0).round() / 100.0 == 1.71);

        // 9. 0.67 * 1.68 M = 1.1256 M (1.13 M; printed 1.1 M)
        let r8_ratio = r8.tolerance.value.as_ref().unwrap().as_f64();
        let r8_amb = r8
            .planning_value
            .as_ref()
            .unwrap()
            .ambition
            .as_ref()
            .unwrap()
            .as_f64();
        let r8_exact = r8_ratio * r8_amb;
        assert!((r8_exact - 1_125_600.0).abs() < 1e-6);
        assert!(((r8_exact / 1_000_000.0) * 100.0).round() / 100.0 == 1.13);
    }

    #[test]
    fn row21_derived() {
        let (reg, _) = load_committed();
        let r1 = reg.get(1).expect("row 1");
        let r10 = reg.get(10).expect("row 10");
        let r21 = reg.get(21).expect("row 21");

        // 16 worlds * 10 s / (1/500 s) = 80,000 world-steps per design.
        let steps_per_design = 16.0 * 10.0 * 500.0;
        assert_eq!(steps_per_design, 80_000.0);

        let r1_floor = r1.interim_floor.as_ref().unwrap().value.as_f64();
        let r10_tol = r10.tolerance.value.as_ref().unwrap().as_f64();
        let derived_tol = (r1_floor * r10_tol) / steps_per_design;
        assert!((derived_tol - 17.5).abs() < 1e-9);
        assert_eq!(derived_tol.floor(), 17.0);

        let r1_amb = r1
            .planning_value
            .as_ref()
            .unwrap()
            .ambition
            .as_ref()
            .unwrap()
            .as_f64();
        let r10_amb = r10.ambition.as_ref().unwrap().value.as_f64();
        let derived_amb = (r1_amb * r10_amb) / steps_per_design;
        assert!((derived_amb - 43.0).abs() < 1e-9);

        let sub_tol = r21
            .sub_targets
            .iter()
            .find(|s| s.id == "derived_tolerance")
            .unwrap();
        let sub_amb = r21
            .sub_targets
            .iter()
            .find(|s| s.id == "derived_ambition")
            .unwrap();
        assert_eq!(sub_tol.value.as_ref().unwrap().as_f64(), 17.0);
        assert_eq!(sub_amb.value.as_ref().unwrap().as_f64(), 43.0);
    }

    #[test]
    fn row5_reference_ratio() {
        let (reg, _) = load_committed();
        let inc = reg
            .get(5)
            .expect("row 5")
            .incumbent_reference
            .as_ref()
            .expect("row 5 incumbent reference");
        let one_card = inc.published_value.as_ref().unwrap().as_f64();
        let two_card = inc.published_secondary_value.as_ref().unwrap().as_f64();
        assert_eq!(one_card, 410.0);
        assert_eq!(two_card, 325.0);
        let ratio = one_card / two_card;
        assert!((ratio - 1.2615384615384615).abs() < 1e-9);
        assert_eq!((ratio * 100.0).round() / 100.0, 1.26);
    }

    #[test]
    fn row2_overhead() {
        let (reg, _) = load_committed();
        let inc = reg
            .get(2)
            .expect("row 2")
            .incumbent_reference
            .as_ref()
            .expect("row 2 incumbent reference");
        let full_step = inc.published_value.as_ref().unwrap().as_f64();
        let physics = inc.published_secondary_value.as_ref().unwrap().as_f64();
        assert_eq!(full_step, 347_000.0);
        assert_eq!(physics, 1_200_000.0);
        let overhead = 1.0 - full_step / physics;
        assert!((overhead - 0.7108333333333333).abs() < 1e-9);
        assert_eq!((overhead * 1000.0).round() / 1000.0, 0.711);
    }

    #[test]
    fn ratio_row_point_estimate_is_typed_error() {
        let (reg, _) = load_committed();

        assert_eq!(
            reg.judge(5, Figure::Point(1.30)),
            Err(JudgeError::PointEstimateOnRatioRow { row: 5 })
        );

        let pass = reg
            .judge(5, Figure::LowerBound(1.30))
            .expect("lower bound 1.30 on row 5");
        assert!(pass.passed);
        assert_eq!(pass.status, ThresholdStatus::Tolerance);
        assert_eq!(pass.required, Some(1.25));

        let fail = reg
            .judge(5, Figure::LowerBound(1.20))
            .expect("lower bound 1.20 on row 5");
        assert!(!fail.passed);
        assert_eq!(fail.status, ThresholdStatus::Tolerance);
        assert_eq!(fail.required, Some(1.25));
    }

    #[test]
    fn at_most_ratio_row_judges_oriented_bound() {
        let (reg, _) = load_committed();

        // Row 4: at most 1.25x incumbent time -> oriented threshold 1/1.25 = 0.80.
        let pass = reg
            .judge(4, Figure::LowerBound(0.854))
            .expect("410 / 480 = 0.854 passes row 4");
        assert!(pass.passed);
        assert!((pass.required.unwrap() - 0.80).abs() < 1e-12);

        let fail = reg
            .judge(4, Figure::LowerBound(0.774))
            .expect("410 / 530 = 0.774 fails row 4");
        assert!(!fail.passed);
        assert!((fail.required.unwrap() - 0.80).abs() < 1e-12);

        assert_eq!(
            reg.judge(4, Figure::UpperBound(1.20)),
            Err(JudgeError::WrongSide { row: 4 })
        );

        // WrongSide on absolute rows too: LowerBound on absolute at_most (row 18),
        // UpperBound on absolute at_least (row 16).
        assert_eq!(
            reg.judge(18, Figure::LowerBound(10.0)),
            Err(JudgeError::WrongSide { row: 18 })
        );
        assert_eq!(
            reg.judge(16, Figure::UpperBound(0.5)),
            Err(JudgeError::WrongSide { row: 16 })
        );
    }

    #[test]
    fn unknown_row_refused() {
        let (reg, _) = load_committed();
        assert_eq!(
            reg.threshold(0, None),
            Err(JudgeError::UnknownRow { row: 0 })
        );
        assert_eq!(
            reg.threshold(23, None),
            Err(JudgeError::UnknownRow { row: 23 })
        );
        assert_eq!(
            reg.judge(0, Figure::Point(1.0)),
            Err(JudgeError::UnknownRow { row: 0 })
        );
        assert_eq!(
            reg.judge(99, Figure::LowerBound(1.0)),
            Err(JudgeError::UnknownRow { row: 99 })
        );
        assert_eq!(
            reg.planning_value(23),
            Err(JudgeError::UnknownRow { row: 23 })
        );
    }

    #[test]
    fn row_without_carrier_fails_lint() {
        let raw = fs::read_to_string(repo_root().join(PERF_TARGETS_PATH)).expect("read toml");
        let no_carrier = raw.replacen("carrier = \"BD-217\"", "carrier = \"\"", 1);
        let parsed = parse_str(&no_carrier).expect("parse empty carrier scratch");
        let findings = lint_registry(&parsed, None);
        assert!(
            findings
                .iter()
                .any(|f| f.kind == TargetFindingKind::MissingCarrier && f.row == Some(1)),
            "row without carrier must fail lint, got: {findings:?}"
        );
    }
}
