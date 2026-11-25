//! Typed throughput units, rate definitions, exact unit conversions, comparability,
//! wire-form admission, and the INV-PERF-01 ordering check (`N36-03`, ADR-PERF-02,
//! KA-806, CX-374).
//!
//! Pure host arithmetic: determinism class D0 (exact) for conversions on counts;
//! prints nothing (rule L-7); zero `unsafe`; no external JSON library (uses
//! [`crate::json`] and [`crate::canon`]).

#[cfg(all(
    any(
        feature = "units-without-decimation",
        feature = "nc-ordering-compare-raw"
    ),
    any(not(feature = "gauntlet-mutations"), not(debug_assertions))
))]
compile_error!(
    "frob_core::perf::units mutation features require a debug build with `--features gauntlet-mutations`"
);

use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::fmt;
use std::str::FromStr;

use crate::canon::{self, CanonError, Canonical, Value as CanonValue};
use crate::id::CanonicalBytes;
use crate::json::{self, INT53_MAX, Json, Limits, Member, Spanned};

/// Positive reduced exact rational shared with [`crate::perf::workload`] (`N36-02`, `N36-03`).
pub use crate::perf::workload::Rational;

/// Refusal detail slug for INV-PERF-01 ordering violations (`N36-REF`, N35-33 rule 7).
pub const ORDERING_DETAIL: &str = "rates-violate-unit-ordering";

/// Refusal pointer for INV-PERF-01 ordering violations (`N36-REF`, N35-33 rule 7).
pub const ORDERING_POINTER: &str = "rates.ordering";

/// Interim refusal code raised at disclosure/report boundaries (`N36-13`, `BD-216`, `N36-REF`).
pub const ORDERING_INTERIM_REFUSAL_CODE: &str = "FROB-REF-862";

/// Standard repair text for a step rate missing its decimation (`N36-03`).
pub const MISSING_DECIMATION_REPAIR: &str = "restate the figure with its decimation";

/// Standard repair text for an INV-PERF-01 ordering violation (`N36-03`).
pub const ORDERING_REPAIR: &str = "more pairs, or a longer duration";

const fn gcd_u32(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

const fn gcd_u128(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

/// Exact 256-bit unsigned product `(hi, lo)` of two `u128` values for overflow-free
/// rational comparison.
const fn mul_u128_wide(a: u128, b: u128) -> (u128, u128) {
    let a_lo = a & 0xFFFF_FFFF_FFFF_FFFF;
    let a_hi = a >> 64;
    let b_lo = b & 0xFFFF_FFFF_FFFF_FFFF;
    let b_hi = b >> 64;

    let ll = a_lo * b_lo;
    let lh = a_lo * b_hi;
    let hl = a_hi * b_lo;
    let hh = a_hi * b_hi;

    let mid = (ll >> 64) + (lh & 0xFFFF_FFFF_FFFF_FFFF) + (hl & 0xFFFF_FFFF_FFFF_FFFF);
    let lo = (ll & 0xFFFF_FFFF_FFFF_FFFF) | (mid << 64);
    let hi = hh + (lh >> 64) + (hl >> 64) + (mid >> 64);
    (hi, lo)
}

/// Reduced non-negative exact rational `num / den` (`den >= 1`, `gcd(num, den) == 1`) used
/// for exact D0 throughput unit conversions on integer counts and rates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExactRational {
    num: u128,
    den: u128,
}

impl ExactRational {
    /// Construct a reduced non-negative rational `num / den` (`den > 0`).
    pub fn new(num: u128, den: u128) -> Result<Self, UnitsError> {
        if den == 0 {
            return Err(UnitsError::InvalidDecimation {
                reason: "rational denominator must be strictly positive".to_owned(),
            });
        }
        if num == 0 {
            return Ok(Self { num: 0, den: 1 });
        }
        let g = gcd_u128(num, den);
        Ok(Self {
            num: num / g,
            den: den / g,
        })
    }

    #[must_use]
    pub const fn from_u64(n: u64) -> Self {
        Self {
            num: n as u128,
            den: 1,
        }
    }

    #[must_use]
    pub const fn num(self) -> u128 {
        self.num
    }

    #[must_use]
    pub const fn den(self) -> u128 {
        self.den
    }

    #[must_use]
    pub const fn is_integer(self) -> bool {
        self.den == 1
    }

    /// Return the exact integer value if `den == 1` and `num` fits in `u64`.
    #[must_use]
    pub fn exact_u64(self) -> Option<u64> {
        if self.den == 1 {
            u64::try_from(self.num).ok()
        } else {
            None
        }
    }

    /// Approximate as `f64`.
    #[must_use]
    pub fn to_f64(self) -> f64 {
        (self.num as f64) / (self.den as f64)
    }

    /// Exact reduced product `self * (num / den)`.
    pub fn checked_mul_ratio(self, num: u128, den: u128) -> Result<Self, UnitsError> {
        if den == 0 {
            return Err(UnitsError::InvalidDecimation {
                reason: "denominator must be strictly positive".to_owned(),
            });
        }
        if self.num == 0 || num == 0 {
            return Ok(Self { num: 0, den: 1 });
        }
        let g1 = gcd_u128(self.num, den);
        let g2 = gcd_u128(num, self.den);
        let rn = (self.num / g1)
            .checked_mul(num / g2)
            .ok_or(UnitsError::ArithmeticOverflow)?;
        let rd = (self.den / g2)
            .checked_mul(den / g1)
            .ok_or(UnitsError::ArithmeticOverflow)?;
        Ok(Self { num: rn, den: rd })
    }
}

impl Ord for ExactRational {
    fn cmp(&self, other: &Self) -> Ordering {
        let lhs = mul_u128_wide(self.num, other.den);
        let rhs = mul_u128_wide(other.num, self.den);
        lhs.cmp(&rhs)
    }
}

impl PartialOrd for ExactRational {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for ExactRational {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.num, self.den)
    }
}

/// Error returned when constructing typed performance units or rates.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnitsError {
    InvalidDecimation { reason: String },
    InvalidSettings { reason: String },
    ScopeInclusionMismatch { scope: StepScope, inclusion: Inclusion },
    SettingsMismatch { reason: String },
    InvalidWallTime { reason: String },
    InvalidCounts { reason: String },
    UnsupportedConversion { from: RateUnit, to: StepUnit },
    ArithmeticOverflow,
}

impl fmt::Display for UnitsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidDecimation { reason } => write!(f, "invalid decimation: {reason}"),
            Self::InvalidSettings { reason } => write!(f, "invalid settings: {reason}"),
            Self::ScopeInclusionMismatch { scope, inclusion } => write!(
                f,
                "step scope `{}` disagrees with inclusion flags (observe_reward_reset={}, policy_inference={}, learning={})",
                scope.as_str(),
                inclusion.observe_reward_reset,
                inclusion.policy_inference,
                inclusion.learning
            ),
            Self::SettingsMismatch { reason } => write!(f, "rate/settings mismatch: {reason}"),
            Self::InvalidWallTime { reason } => write!(f, "invalid wall_s interval: {reason}"),
            Self::InvalidCounts { reason } => write!(f, "invalid step counts: {reason}"),
            Self::UnsupportedConversion { from, to } => write!(
                f,
                "cannot convert `{}` to `{}` (not a step-family unit)",
                from.as_str(),
                to.as_unit().as_str()
            ),
            Self::ArithmeticOverflow => f.write_str("exact rational conversion overflowed u128"),
        }
    }
}

impl std::error::Error for UnitsError {}

/// Decimation `d = control_period / h`, stored as a reduced rational with `num >= den >= 1`.
///
/// For all reference workloads `den == 1`; non-integer clock ratios are carried exactly
/// (e.g. `25/2` for a 25 ms control period over a 2 ms physics step).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Decimation {
    num: u32,
    den: u32,
}

impl Decimation {
    /// Construct a reduced decimation `num / den` (`den >= 1`, `num >= den`).
    pub fn new(num: u32, den: u32) -> Result<Self, UnitsError> {
        if den == 0 {
            return Err(UnitsError::InvalidDecimation {
                reason: "denominator must be >= 1".to_owned(),
            });
        }
        if num < den {
            return Err(UnitsError::InvalidDecimation {
                reason: format!("decimation {num}/{den} must satisfy num >= den >= 1"),
            });
        }
        let g = gcd_u32(num, den);
        Ok(Self {
            num: num / g,
            den: den / g,
        })
    }

    /// Construct an integer decimation `d / 1` (`d >= 1`).
    pub fn integer(d: u32) -> Result<Self, UnitsError> {
        Self::new(d, 1)
    }

    /// Derive exact reduced decimation `control_period_s / h_s`.
    pub fn from_clocks(control_period_s: Rational, h_s: Rational) -> Result<Self, UnitsError> {
        let num_wide = u128::from(control_period_s.num())
            .checked_mul(u128::from(h_s.den()))
            .ok_or(UnitsError::ArithmeticOverflow)?;
        let den_wide = u128::from(control_period_s.den())
            .checked_mul(u128::from(h_s.num()))
            .ok_or(UnitsError::ArithmeticOverflow)?;
        let g = gcd_u128(num_wide, den_wide);
        let rn = num_wide / g;
        let rd = den_wide / g;
        if rn < rd {
            return Err(UnitsError::InvalidDecimation {
                reason: format!("control period {control_period_s} is shorter than physics step {h_s}"),
            });
        }
        let num = u32::try_from(rn).map_err(|_| UnitsError::ArithmeticOverflow)?;
        let den = u32::try_from(rd).map_err(|_| UnitsError::ArithmeticOverflow)?;
        Ok(Self { num, den })
    }

    /// Parse strict reduced decimation `"p/q"` or integer `"p"` (`p >= q >= 1`, `gcd(p, q) == 1`).
    pub fn parse(s: &str) -> Result<Self, UnitsError> {
        let r = Rational::parse(s).map_err(|e| UnitsError::InvalidDecimation {
            reason: e.to_string(),
        })?;
        if r.num() < r.den() {
            return Err(UnitsError::InvalidDecimation {
                reason: format!("decimation `{s}` must satisfy num >= den >= 1"),
            });
        }
        let num = u32::try_from(r.num()).map_err(|_| UnitsError::InvalidDecimation {
            reason: format!("decimation numerator in `{s}` exceeds u32"),
        })?;
        let den = u32::try_from(r.den()).map_err(|_| UnitsError::InvalidDecimation {
            reason: format!("decimation denominator in `{s}` exceeds u32"),
        })?;
        Ok(Self { num, den })
    }

    #[must_use]
    pub const fn num(self) -> u32 {
        self.num
    }

    #[must_use]
    pub const fn den(self) -> u32 {
        self.den
    }

    #[must_use]
    pub const fn is_integer(self) -> bool {
        self.den == 1
    }

    /// Convert to [`Rational`].
    #[must_use]
    pub fn as_rational(self) -> Rational {
        Rational::new(u64::from(self.num), u64::from(self.den))
            .expect("Decimation invariant guarantees reduced positive Rational")
    }

    /// Render in explicit `"p/q"` form (e.g. `"10/1"`, `"25/2"`).
    #[must_use]
    pub fn to_slash_string(self) -> String {
        format!("{}/{}", self.num, self.den)
    }
}

impl Ord for Decimation {
    fn cmp(&self, other: &Self) -> Ordering {
        let lhs = u64::from(self.num) * u64::from(other.den);
        let rhs = u64::from(other.num) * u64::from(self.den);
        lhs.cmp(&rhs)
    }
}

impl PartialOrd for Decimation {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for Decimation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.num, self.den)
    }
}

impl FromStr for Decimation {
    type Err = UnitsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl Canonical for Decimation {
    fn write(&self, w: CanonValue<'_>) -> Result<(), CanonError> {
        w.str(&self.to_slash_string());
        Ok(())
    }
}

/// What work is included inside the timed loop of a throughput measurement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StepScope {
    PhysicsOnly,
    FullEnvStep,
    Training,
}

impl StepScope {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PhysicsOnly => "physics_only",
            Self::FullEnvStep => "full_env_step",
            Self::Training => "training",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "physics_only" => Some(Self::PhysicsOnly),
            "full_env_step" => Some(Self::FullEnvStep),
            "training" => Some(Self::Training),
            _ => None,
        }
    }
}

impl fmt::Display for StepScope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Canonical for StepScope {
    fn write(&self, w: CanonValue<'_>) -> Result<(), CanonError> {
        w.str(self.as_str());
        Ok(())
    }
}

/// Explicit boolean flags declaring what non-physics work is inside the timed loop.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Inclusion {
    pub observe_reward_reset: bool,
    pub policy_inference: bool,
    pub learning: bool,
}

impl Inclusion {
    #[must_use]
    pub const fn physics_only() -> Self {
        Self {
            observe_reward_reset: false,
            policy_inference: false,
            learning: false,
        }
    }

    #[must_use]
    pub const fn full_env_step() -> Self {
        Self {
            observe_reward_reset: true,
            policy_inference: false,
            learning: false,
        }
    }

    #[must_use]
    pub const fn full_env_step_with_inference() -> Self {
        Self {
            observe_reward_reset: true,
            policy_inference: true,
            learning: false,
        }
    }

    #[must_use]
    pub const fn training() -> Self {
        Self {
            observe_reward_reset: true,
            policy_inference: true,
            learning: true,
        }
    }

    /// True iff `self` agrees with `scope` per N36-03 constructor rule 3.
    #[must_use]
    pub const fn matches_scope(self, scope: StepScope) -> bool {
        match scope {
            StepScope::PhysicsOnly => {
                !self.observe_reward_reset && !self.policy_inference && !self.learning
            }
            StepScope::FullEnvStep => self.observe_reward_reset && !self.learning,
            StepScope::Training => {
                self.observe_reward_reset && self.policy_inference && self.learning
            }
        }
    }
}

impl Canonical for Inclusion {
    fn write(&self, w: CanonValue<'_>) -> Result<(), CanonError> {
        w.object(|o| {
            o.bool("learning", self.learning)?;
            o.bool("observe_reward_reset", self.observe_reward_reset)?;
            o.bool("policy_inference", self.policy_inference)?;
            Ok(())
        })
    }
}

/// How a comparator or engine stack was configured for a throughput measurement.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Settings {
    /// Matched settings: the incumbent run at our `h`, `d`, and Newton iteration count.
    Matched {
        h: Rational,
        d: Decimation,
        newton_iterations: u8,
    },
    /// Native settings: each stack run at its own recommended step and decimation.
    Native {
        h: Rational,
        d: Decimation,
        source: String,
    },
}

impl Settings {
    #[must_use]
    pub const fn h(&self) -> Rational {
        match self {
            Self::Matched { h, .. } | Self::Native { h, .. } => *h,
        }
    }

    #[must_use]
    pub const fn d(&self) -> Decimation {
        match self {
            Self::Matched { d, .. } | Self::Native { d, .. } => *d,
        }
    }

    #[must_use]
    pub const fn kind_str(&self) -> &'static str {
        match self {
            Self::Matched { .. } => "matched",
            Self::Native { .. } => "native",
        }
    }

    fn validate(&self) -> Result<(), UnitsError> {
        if let Self::Native { source, .. } = self
            && (source.trim().is_empty() || json::validate_identifier(source).is_err())
        {
            return Err(UnitsError::InvalidSettings {
                reason: "native settings `source` must be a non-empty printable string".to_owned(),
            });
        }
        Ok(())
    }
}

impl Canonical for Settings {
    fn write(&self, w: CanonValue<'_>) -> Result<(), CanonError> {
        match self {
            Self::Matched {
                h,
                d,
                newton_iterations,
            } => w.object(|o| {
                o.str("d", &d.to_slash_string())?;
                o.str("h_s", &h.to_slash_string())?;
                o.str("kind", "matched")?;
                o.int53("newton_iterations", i64::from(*newton_iterations))?;
                Ok(())
            }),
            Self::Native { h, d, source } => w.object(|o| {
                o.str("d", &d.to_slash_string())?;
                o.str("h_s", &h.to_slash_string())?;
                o.str("kind", "native")?;
                o.str("source", source)?;
                Ok(())
            }),
        }
    }
}

/// Complete closed set of performance rate units (ADR-PERF-02).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RateUnit {
    WorldStepsPerSec,
    EnvStepsPerSec,
    WorldSecPerWallSec,
    EpisodesPerSec,
    RealTimeMultiple,
}

impl RateUnit {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorldStepsPerSec => "world_steps_per_s",
            Self::EnvStepsPerSec => "env_steps_per_s",
            Self::WorldSecPerWallSec => "world_s_per_wall_s",
            Self::EpisodesPerSec => "episodes_per_s",
            Self::RealTimeMultiple => "real_time_multiple",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "world_steps_per_s" => Some(Self::WorldStepsPerSec),
            "env_steps_per_s" => Some(Self::EnvStepsPerSec),
            "world_s_per_wall_s" => Some(Self::WorldSecPerWallSec),
            "episodes_per_s" => Some(Self::EpisodesPerSec),
            "real_time_multiple" => Some(Self::RealTimeMultiple),
            _ => None,
        }
    }

    /// True for step-family units that require an explicit [`Decimation`] and physics step `h`.
    #[must_use]
    pub const fn is_step_unit(self) -> bool {
        matches!(
            self,
            Self::WorldStepsPerSec | Self::EnvStepsPerSec | Self::WorldSecPerWallSec
        )
    }
}

impl fmt::Display for RateUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Canonical for RateUnit {
    fn write(&self, w: CanonValue<'_>) -> Result<(), CanonError> {
        w.str(self.as_str());
        Ok(())
    }
}

/// Step-family units requiring a non-optional [`Decimation`] and physics step `h`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StepUnit {
    WorldStepsPerSec,
    EnvStepsPerSec,
    WorldSecPerWallSec,
}

impl StepUnit {
    #[must_use]
    pub const fn as_unit(self) -> RateUnit {
        match self {
            Self::WorldStepsPerSec => RateUnit::WorldStepsPerSec,
            Self::EnvStepsPerSec => RateUnit::EnvStepsPerSec,
            Self::WorldSecPerWallSec => RateUnit::WorldSecPerWallSec,
        }
    }
}

/// Completed work counts separated by world health; `degraded` and `faulted` steps are
/// tracked beside `valid` and never enter a rate numerator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StepCounts {
    pub valid: u64,
    pub degraded: u64,
    pub faulted: u64,
}

impl StepCounts {
    /// Partition a total step count into `valid = total - degraded - faulted`, refusing
    /// inputs where `degraded + faulted > total`.
    pub fn from_total(total: u64, degraded: u64, faulted: u64) -> Result<Self, UnitsError> {
        let excluded = degraded
            .checked_add(faulted)
            .ok_or_else(|| UnitsError::InvalidCounts {
                reason: "degraded + faulted overflowed u64".to_owned(),
            })?;
        let valid = total
            .checked_sub(excluded)
            .ok_or_else(|| UnitsError::InvalidCounts {
                reason: format!("degraded ({degraded}) + faulted ({faulted}) exceeds total ({total})"),
            })?;
        Ok(Self {
            valid,
            degraded,
            faulted,
        })
    }

    #[must_use]
    pub const fn total(self) -> Option<u64> {
        let Some(a) = self.valid.checked_add(self.degraded) else {
            return None;
        };
        a.checked_add(self.faulted)
    }
}

fn write_u64_json_field(
    o: &mut canon::Object<'_>,
    key: &str,
    v: u64,
) -> Result<(), CanonError> {
    if v <= INT53_MAX as u64 {
        o.int53(key, v as i64)
    } else {
        o.str(key, &v.to_string())
    }
}

impl Canonical for StepCounts {
    fn write(&self, w: CanonValue<'_>) -> Result<(), CanonError> {
        w.object(|o| {
            write_u64_json_field(o, "degraded", self.degraded)?;
            write_u64_json_field(o, "faulted", self.faulted)?;
            write_u64_json_field(o, "valid", self.valid)?;
            Ok(())
        })
    }
}

/// Validated throughput measurement whose invariants are enforced at construction.
///
/// Fields are private so an ill-formed rate (missing decimation on a step unit, mismatched
/// scope/inclusion, or pre-divided value bypassing `counts.valid / wall_s`) cannot be
/// constructed:
///
/// ```compile_fail,E0308
/// use frob_core::perf::units::{
///     Decimation, Inclusion, Rate, Rational, Settings, StepCounts, StepScope, StepUnit,
/// };
/// let h = Rational::new(1, 500).unwrap();
/// let d = Decimation::integer(10).unwrap();
/// let settings = Settings::Matched { h, d, newton_iterations: 4 };
/// let counts = StepCounts { valid: 400_000, degraded: 0, faulted: 0 };
/// let _bad = Rate::steps(
///     StepUnit::EnvStepsPerSec,
///     None::<Decimation>,
///     h,
///     StepScope::PhysicsOnly,
///     Inclusion::physics_only(),
///     settings,
///     counts,
///     1.0,
/// );
/// ```
///
/// ```compile_fail,E0451
/// use frob_core::perf::units::{
///     Inclusion, Rate, RateUnit, Rational, Decimation, Settings, StepCounts, StepScope,
/// };
/// let h = Rational::new(1, 500).unwrap();
/// let d = Decimation::integer(10).unwrap();
/// let _bad = Rate {
///     value: 400_000.0,
///     unit: RateUnit::EnvStepsPerSec,
///     decimation: None,
///     h: None,
///     scope: StepScope::PhysicsOnly,
///     inclusion: Inclusion::physics_only(),
///     settings: Settings::Matched { h, d, newton_iterations: 4 },
///     counts: StepCounts { valid: 400_000, degraded: 0, faulted: 0 },
///     wall_s: 1.0,
/// };
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Rate {
    value: f64,
    unit: RateUnit,
    decimation: Option<Decimation>,
    h: Option<Rational>,
    scope: StepScope,
    inclusion: Inclusion,
    settings: Settings,
    counts: StepCounts,
    wall_s: f64,
}

impl Rate {
    /// Construct a step-family rate (`WorldStepsPerSec`, `EnvStepsPerSec`, or
    /// `WorldSecPerWallSec`) with a mandatory [`Decimation`] and physics step `h`.
    ///
    /// Computes `value` strictly from `counts.valid` and `wall_s`; `degraded` and `faulted`
    /// never enter the numerator.
    #[allow(clippy::too_many_arguments)]
    pub fn steps(
        unit: StepUnit,
        decimation: Decimation,
        h: Rational,
        scope: StepScope,
        inclusion: Inclusion,
        settings: Settings,
        counts: StepCounts,
        wall_s: f64,
    ) -> Result<Self, UnitsError> {
        validate_common(scope, inclusion, &settings, wall_s)?;
        if settings.d() != decimation {
            return Err(UnitsError::SettingsMismatch {
                reason: format!(
                    "rate decimation {decimation} differs from settings decimation {}",
                    settings.d()
                ),
            });
        }
        if settings.h() != h {
            return Err(UnitsError::SettingsMismatch {
                reason: format!("rate h {} differs from settings h {}", h, settings.h()),
            });
        }
        let value = match unit {
            StepUnit::WorldStepsPerSec | StepUnit::EnvStepsPerSec => (counts.valid as f64) / wall_s,
            StepUnit::WorldSecPerWallSec => world_steps_to_world_sec_f64(counts.valid, h, wall_s),
        };
        if !value.is_finite() {
            return Err(UnitsError::InvalidWallTime {
                reason: "computed rate is not finite".to_owned(),
            });
        }
        Ok(Self {
            value,
            unit: unit.as_unit(),
            decimation: Some(decimation),
            h: Some(h),
            scope,
            inclusion,
            settings,
            counts,
            wall_s,
        })
    }

    /// Construct a single-world `RealTimeMultiple` rate (no decimation applies).
    ///
    /// `counts.valid` counts completed valid world-steps of the single referee world;
    /// `value = (counts.valid * settings.h()) / wall_s`.
    pub fn real_time_multiple(
        scope: StepScope,
        inclusion: Inclusion,
        settings: Settings,
        counts: StepCounts,
        wall_s: f64,
    ) -> Result<Self, UnitsError> {
        validate_common(scope, inclusion, &settings, wall_s)?;
        let value = world_steps_to_world_sec_f64(counts.valid, settings.h(), wall_s);
        if !value.is_finite() {
            return Err(UnitsError::InvalidWallTime {
                reason: "computed real-time multiple is not finite".to_owned(),
            });
        }
        Ok(Self {
            value,
            unit: RateUnit::RealTimeMultiple,
            decimation: None,
            h: None,
            scope,
            inclusion,
            settings,
            counts,
            wall_s,
        })
    }

    /// Construct an `EpisodesPerSec` rate (`value = counts.valid / wall_s`, no decimation).
    pub fn episodes(
        scope: StepScope,
        inclusion: Inclusion,
        settings: Settings,
        counts: StepCounts,
        wall_s: f64,
    ) -> Result<Self, UnitsError> {
        validate_common(scope, inclusion, &settings, wall_s)?;
        let value = (counts.valid as f64) / wall_s;
        if !value.is_finite() {
            return Err(UnitsError::InvalidWallTime {
                reason: "computed episode rate is not finite".to_owned(),
            });
        }
        Ok(Self {
            value,
            unit: RateUnit::EpisodesPerSec,
            decimation: None,
            h: None,
            scope,
            inclusion,
            settings,
            counts,
            wall_s,
        })
    }

    #[must_use]
    pub const fn value(&self) -> f64 {
        self.value
    }

    #[must_use]
    pub const fn unit(&self) -> RateUnit {
        self.unit
    }

    #[must_use]
    pub const fn decimation(&self) -> Option<Decimation> {
        self.decimation
    }

    #[must_use]
    pub const fn h(&self) -> Option<Rational> {
        self.h
    }

    #[must_use]
    pub const fn scope(&self) -> StepScope {
        self.scope
    }

    #[must_use]
    pub const fn inclusion(&self) -> Inclusion {
        self.inclusion
    }

    #[must_use]
    pub const fn settings(&self) -> &Settings {
        &self.settings
    }

    #[must_use]
    pub const fn counts(&self) -> StepCounts {
        self.counts
    }

    #[must_use]
    pub const fn wall_s(&self) -> f64 {
        self.wall_s
    }

    /// Exact D0 valid work count expressed as world-steps (exact reduced rational).
    pub fn exact_world_steps(&self) -> Result<ExactRational, UnitsError> {
        match self.unit {
            RateUnit::WorldStepsPerSec | RateUnit::WorldSecPerWallSec => {
                Ok(ExactRational::from_u64(self.counts.valid))
            }
            RateUnit::EnvStepsPerSec => {
                let d = self.decimation.ok_or(UnitsError::InvalidDecimation {
                    reason: "missing decimation on env-step rate".to_owned(),
                })?;
                env_steps_to_world_steps(self.counts.valid, d)
            }
            other => Err(UnitsError::UnsupportedConversion {
                from: other,
                to: StepUnit::WorldStepsPerSec,
            }),
        }
    }

    /// Exact D0 valid work count expressed as env-steps (exact reduced rational).
    pub fn exact_env_steps(&self) -> Result<ExactRational, UnitsError> {
        match self.unit {
            RateUnit::EnvStepsPerSec => Ok(ExactRational::from_u64(self.counts.valid)),
            RateUnit::WorldStepsPerSec | RateUnit::WorldSecPerWallSec => {
                let d = self.decimation.ok_or(UnitsError::InvalidDecimation {
                    reason: "missing decimation on world-step rate".to_owned(),
                })?;
                world_steps_to_env_steps(self.counts.valid, d)
            }
            other => Err(UnitsError::UnsupportedConversion {
                from: other,
                to: StepUnit::EnvStepsPerSec,
            }),
        }
    }

    /// Exact D0 simulated world-seconds completed (`valid_world_steps * h`).
    pub fn exact_world_seconds(&self) -> Result<ExactRational, UnitsError> {
        let h = self.h.ok_or(UnitsError::UnsupportedConversion {
            from: self.unit,
            to: StepUnit::WorldSecPerWallSec,
        })?;
        let ws = self.exact_world_steps()?;
        ws.checked_mul_ratio(u128::from(h.num()), u128::from(h.den()))
    }

    /// Convert any step-family rate into `env-steps/s` without crossing [`StepScope`].
    pub fn as_env_steps_per_s(&self) -> Result<f64, UnitsError> {
        let exact = self.exact_env_steps()?;
        Ok(exact_over_wall_f64(exact, self.wall_s))
    }

    /// Convert any step-family rate into `world-steps/s` without crossing [`StepScope`].
    pub fn as_world_steps_per_s(&self) -> Result<f64, UnitsError> {
        let exact = self.exact_world_steps()?;
        Ok(exact_over_wall_f64(exact, self.wall_s))
    }

    /// Convert any step-family rate into `world-seconds / wall-second` without crossing [`StepScope`].
    pub fn as_world_sec_per_wall_s(&self) -> Result<f64, UnitsError> {
        let exact = self.exact_world_seconds()?;
        Ok(exact_over_wall_f64(exact, self.wall_s))
    }

    /// Serialize this rate to canonical JSON bytes via [`crate::canon::to_canonical`].
    pub fn to_canonical_json(&self) -> Result<CanonicalBytes, CanonError> {
        canon::to_canonical(self)
    }

    /// Parse and validate one rate object from raw JSON bytes using [`crate::json::admit`].
    pub fn from_json_bytes(bytes: &[u8]) -> Result<Self, RateSchemaError> {
        let admitted = json::admit(bytes, &Limits::default()).map_err(|f| RateSchemaError {
            kind: RateSchemaErrorKind::JsonAdmission,
            pointer: f.pointer.clone(),
            message: f.to_string(),
            repair: "supply valid strict RFC 8259 JSON without duplicate keys",
        })?;
        parse_rate_at(admitted.root(), "")
    }
}

impl Canonical for Rate {
    fn write(&self, w: CanonValue<'_>) -> Result<(), CanonError> {
        w.object(|o| {
            o.member_term("counts", &self.counts)?;
            if let Some(d) = self.decimation {
                o.member_term("decimation", &d)?;
            }
            if let Some(h) = self.h {
                o.str("h_s", &h.to_slash_string())?;
            }
            o.member_term("inclusion", &self.inclusion)?;
            o.member_term("scope", &self.scope)?;
            o.member_term("settings", &self.settings)?;
            o.member_term("unit", &self.unit)?;
            o.str("value", &format_canonical_f64_str(self.value))?;
            o.str("wall_s", &format_canonical_f64_str(self.wall_s))?;
            Ok(())
        })
    }
}

fn validate_common(
    scope: StepScope,
    inclusion: Inclusion,
    settings: &Settings,
    wall_s: f64,
) -> Result<(), UnitsError> {
    if !wall_s.is_finite() || wall_s <= 0.0 {
        return Err(UnitsError::InvalidWallTime {
            reason: format!("wall_s must be finite and strictly positive, got {wall_s}"),
        });
    }
    if !inclusion.matches_scope(scope) {
        return Err(UnitsError::ScopeInclusionMismatch { scope, inclusion });
    }
    settings.validate()
}

fn exact_over_wall_f64(q: ExactRational, wall_s: f64) -> f64 {
    if q.den() == 1 {
        (q.num() as f64) / wall_s
    } else {
        (q.num() as f64) / ((q.den() as f64) * wall_s)
    }
}

fn world_steps_to_world_sec_f64(valid_world_steps: u64, h: Rational, wall_s: f64) -> f64 {
    match world_steps_to_world_seconds(valid_world_steps, h) {
        Ok(q) => exact_over_wall_f64(q, wall_s),
        Err(_) => ((valid_world_steps as f64) * (h.num() as f64)) / ((h.den() as f64) * wall_s),
    }
}

fn format_canonical_f64_str(x: f64) -> String {
    if x == 0.0 {
        "0".to_owned()
    } else {
        format!("{x}")
    }
}

/// Exact D0 conversion: `world-steps = env-steps * d`.
pub fn env_steps_to_world_steps(
    env_steps: u64,
    d: Decimation,
) -> Result<ExactRational, UnitsError> {
    ExactRational::from_u64(env_steps).checked_mul_ratio(u128::from(d.num()), u128::from(d.den()))
}

/// Exact D0 conversion: `env-steps = world-steps / d`.
pub fn world_steps_to_env_steps(
    world_steps: u64,
    d: Decimation,
) -> Result<ExactRational, UnitsError> {
    ExactRational::from_u64(world_steps).checked_mul_ratio(u128::from(d.den()), u128::from(d.num()))
}

/// Exact D0 conversion: `world-seconds = world-steps * h`.
pub fn world_steps_to_world_seconds(
    world_steps: u64,
    h: Rational,
) -> Result<ExactRational, UnitsError> {
    ExactRational::from_u64(world_steps).checked_mul_ratio(u128::from(h.num()), u128::from(h.den()))
}

/// Hardware/workload key identifying one INV-PERF-01 ordering group (`N36-03`).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HardwareKey {
    pub machine: String,
    pub device: String,
    pub world_count: u32,
    pub workload: String,
    pub settings: Settings,
}

impl fmt::Display for HardwareKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "machine={} device={} worlds={} workload={} settings={}({},{})",
            self.machine,
            self.device,
            self.world_count,
            self.workload,
            self.settings.kind_str(),
            self.settings.h(),
            self.settings.d()
        )
    }
}

/// A throughput rate tagged with the hardware key under which it was measured.
#[derive(Debug, Clone, PartialEq)]
pub struct KeyedRate {
    pub key: HardwareKey,
    pub rate: Rate,
}

/// Violation of INV-PERF-01 (`training <= full_env_step <= physics_only` in `env-steps/s`
/// on one hardware key), naming both rates, both values in both units, and the hardware key.
#[derive(Debug, Clone, PartialEq)]
pub struct OrderingViolation {
    pub hardware_key: HardwareKey,
    pub slower_scope: StepScope,
    pub faster_scope: StepScope,
    pub slower_rate: Rate,
    pub faster_rate: Rate,
    pub slower_env_steps_per_s: f64,
    pub slower_world_steps_per_s: f64,
    pub faster_env_steps_per_s: f64,
    pub faster_world_steps_per_s: f64,
    pub detail: &'static str,
    pub pointer: &'static str,
    pub repair: &'static str,
}

impl fmt::Display for OrderingViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "INV-PERF-01 violation ({}) at `{}` on [{}]: `{}` ({} env-steps/s, {} world-steps/s) \
             exceeds `{}` ({} env-steps/s, {} world-steps/s); repair: {}",
            self.detail,
            self.pointer,
            self.hardware_key,
            self.slower_scope,
            self.slower_env_steps_per_s,
            self.slower_world_steps_per_s,
            self.faster_scope,
            self.faster_env_steps_per_s,
            self.faster_world_steps_per_s,
            self.repair
        )
    }
}

impl std::error::Error for OrderingViolation {}

/// Check INV-PERF-01 across a slice of hardware-keyed rates.
///
/// Groups step rates by [`HardwareKey`] and enforces
/// `training <= full_env_step <= physics_only` in `env-steps/s` strictly as reported (no
/// noise tolerance). Non-step rates (`RealTimeMultiple`, `EpisodesPerSec`) do not enter the
/// step-scope ordering inequality.
pub fn check_ordering(rates: &[KeyedRate]) -> Result<(), OrderingViolation> {
    let mut groups: Vec<(&HardwareKey, Vec<&Rate>)> = Vec::new();
    for kr in rates {
        if !kr.rate.unit.is_step_unit() {
            continue;
        }
        if let Some((_, bucket)) = groups.iter_mut().find(|(k, _)| *k == &kr.key) {
            bucket.push(&kr.rate);
        } else {
            groups.push((&kr.key, vec![&kr.rate]));
        }
    }
    for (key, bucket) in groups {
        check_group_ordering_refs(key, &bucket)?;
    }
    Ok(())
}

/// Check INV-PERF-01 for step rates belonging to one [`HardwareKey`].
pub fn check_group_ordering(key: &HardwareKey, rates: &[Rate]) -> Result<(), OrderingViolation> {
    let refs: Vec<&Rate> = rates.iter().filter(|r| r.unit.is_step_unit()).collect();
    check_group_ordering_refs(key, &refs)
}

fn check_group_ordering_refs(
    key: &HardwareKey,
    rates: &[&Rate],
) -> Result<(), OrderingViolation> {
    let mut training = Vec::new();
    let mut full = Vec::new();
    let mut physics = Vec::new();
    for &r in rates {
        match r.scope {
            StepScope::Training => training.push(r),
            StepScope::FullEnvStep => full.push(r),
            StepScope::PhysicsOnly => physics.push(r),
        }
    }

    for &t in &training {
        for &f in &full {
            check_pair_ordered(key, t, f)?;
        }
    }
    for &f in &full {
        for &p in &physics {
            check_pair_ordered(key, f, p)?;
        }
    }
    for &t in &training {
        for &p in &physics {
            check_pair_ordered(key, t, p)?;
        }
    }
    Ok(())
}

fn check_pair_ordered(
    key: &HardwareKey,
    slower: &Rate,
    faster: &Rate,
) -> Result<(), OrderingViolation> {
    let slower_env = slower.as_env_steps_per_s().unwrap_or(slower.value);
    let slower_world = slower.as_world_steps_per_s().unwrap_or(slower.value);
    let faster_env = faster.as_env_steps_per_s().unwrap_or(faster.value);
    let faster_world = faster.as_world_steps_per_s().unwrap_or(faster.value);

    #[cfg(feature = "nc-ordering-compare-raw")]
    let violates = slower.value > faster.value;

    #[cfg(not(feature = "nc-ordering-compare-raw"))]
    let violates = slower_env > faster_env;

    if violates {
        return Err(OrderingViolation {
            hardware_key: key.clone(),
            slower_scope: slower.scope,
            faster_scope: faster.scope,
            slower_rate: slower.clone(),
            faster_rate: faster.clone(),
            slower_env_steps_per_s: slower_env,
            slower_world_steps_per_s: slower_world,
            faster_env_steps_per_s: faster_env,
            faster_world_steps_per_s: faster_world,
            detail: ORDERING_DETAIL,
            pointer: ORDERING_POINTER,
            repair: ORDERING_REPAIR,
        });
    }
    Ok(())
}

/// Comparability predicate (`N36-03`, CX-374).
///
/// Returns `true` iff `a` and `b` share a [`StepScope`], can be expressed in one unit after
/// conversion, and either:
/// 1. both carry `Settings::Matched` with equal `h`, `d`, and `newton_iterations`, or
/// 2. both are [`RateUnit::EnvStepsPerSec`] under `Settings::Native`.
///
/// Under `--features units-without-decimation` (CX-374 negative-control plant, debug +
/// `gauntlet-mutations` only), decimation and settings checks are bypassed so that tests
/// guarding CX-374 must fail.
#[must_use]
pub fn comparable(a: &Rate, b: &Rate) -> bool {
    #[cfg(feature = "units-without-decimation")]
    {
        (a.unit.is_step_unit() && b.unit.is_step_unit()) || a.unit == b.unit
    }
    #[cfg(not(feature = "units-without-decimation"))]
    {
        if a.scope != b.scope {
            return false;
        }
        match (&a.settings, &b.settings) {
            (
                Settings::Matched {
                    h: ha,
                    d: da,
                    newton_iterations: ia,
                },
                Settings::Matched {
                    h: hb,
                    d: db,
                    newton_iterations: ib,
                },
            ) => {
                ha == hb
                    && da == db
                    && ia == ib
                    && ((a.unit.is_step_unit() && b.unit.is_step_unit()) || a.unit == b.unit)
            }
            (Settings::Native { .. }, Settings::Native { .. }) => {
                a.unit == RateUnit::EnvStepsPerSec && b.unit == RateUnit::EnvStepsPerSec
            }
            _ => false,
        }
    }
}

/// Classification of wire-form schema errors returned while reading JSON rate objects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RateSchemaErrorKind {
    JsonAdmission,
    WrongType,
    MissingField,
    MissingDecimation,
    UnknownField,
    InvalidValue,
    InvariantViolation,
}

/// Typed schema error returned by [`Rate::from_json_bytes`], [`parse_rate_at`], and
/// [`parse_rates_json`].
///
/// Carries the exact field pointer (e.g. `rates[0].decimation`) and repair guidance; this
/// module constructs no refusal envelope itself (`N36-13` and `BD-216` map it to
/// `FROB-REF-862`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RateSchemaError {
    pub kind: RateSchemaErrorKind,
    pub pointer: String,
    pub message: String,
    pub repair: &'static str,
}

impl fmt::Display for RateSchemaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "rate schema error ({:?}) at `{}`: {} (repair: {})",
            self.kind, self.pointer, self.message, self.repair
        )
    }
}

impl std::error::Error for RateSchemaError {}

fn join_ptr(base: &str, field: &str) -> String {
    if base.is_empty() {
        field.to_owned()
    } else {
        format!("{base}.{field}")
    }
}

fn schema_err(
    kind: RateSchemaErrorKind,
    pointer: String,
    message: impl Into<String>,
    repair: &'static str,
) -> RateSchemaError {
    RateSchemaError {
        kind,
        pointer,
        message: message.into(),
        repair,
    }
}

fn object_map<'a>(
    node: &'a Spanned<'a>,
    ptr: &str,
) -> Result<BTreeMap<&'a str, &'a Spanned<'a>>, RateSchemaError> {
    let members: &'a [Member<'a>] = node.value.as_object().ok_or_else(|| {
        schema_err(
            RateSchemaErrorKind::WrongType,
            ptr.to_owned(),
            "expected JSON object",
            "provide a JSON object matching the rate schema",
        )
    })?;
    let mut map = BTreeMap::new();
    for m in members {
        map.insert(m.name.as_str(), &m.value);
    }
    Ok(map)
}

fn reject_unknown_keys(
    map: &BTreeMap<&str, &Spanned<'_>>,
    allowed: &[&str],
    base: &str,
) -> Result<(), RateSchemaError> {
    for &k in map.keys() {
        if !allowed.contains(&k) {
            return Err(schema_err(
                RateSchemaErrorKind::UnknownField,
                join_ptr(base, k),
                format!("unknown field `{k}`"),
                "remove the unknown field from the rate object",
            ));
        }
    }
    Ok(())
}

fn require_str<'a>(
    map: &BTreeMap<&str, &'a Spanned<'a>>,
    field: &str,
    base: &str,
) -> Result<&'a str, RateSchemaError> {
    let ptr = join_ptr(base, field);
    let node = map.get(field).ok_or_else(|| {
        schema_err(
            RateSchemaErrorKind::MissingField,
            ptr.clone(),
            format!("missing required field `{field}`"),
            "supply all required fields of the rate object",
        )
    })?;
    node.value.as_str().ok_or_else(|| {
        schema_err(
            RateSchemaErrorKind::WrongType,
            ptr,
            format!("field `{field}` must be a string"),
            "encode the field as a JSON string",
        )
    })
}

fn require_bool(
    map: &BTreeMap<&str, &Spanned<'_>>,
    field: &str,
    base: &str,
) -> Result<bool, RateSchemaError> {
    let ptr = join_ptr(base, field);
    let node = map.get(field).ok_or_else(|| {
        schema_err(
            RateSchemaErrorKind::MissingField,
            ptr.clone(),
            format!("missing required boolean field `{field}`"),
            "supply all three inclusion flags explicitly",
        )
    })?;
    match node.value {
        Json::Bool(b) => Ok(b),
        _ => Err(schema_err(
            RateSchemaErrorKind::WrongType,
            ptr,
            format!("field `{field}` must be a boolean"),
            "use JSON true or false",
        )),
    }
}

fn parse_u64_flexible(node: &Spanned<'_>, ptr: &str) -> Result<u64, RateSchemaError> {
    match &node.value {
        Json::Num(_) => {
            let n = node.as_int53().map_err(|e| {
                schema_err(
                    RateSchemaErrorKind::InvalidValue,
                    ptr.to_owned(),
                    e.message,
                    "encode integers above 2^53 - 1 as canonical decimal strings",
                )
            })?;
            u64::try_from(n).map_err(|_| {
                schema_err(
                    RateSchemaErrorKind::InvalidValue,
                    ptr.to_owned(),
                    "step count must be non-negative",
                    "supply a non-negative integer count",
                )
            })
        }
        Json::Str(_) => node.as_u64str().map_err(|e| {
            schema_err(
                RateSchemaErrorKind::InvalidValue,
                ptr.to_owned(),
                e.message,
                "supply a canonical unsigned 64-bit decimal string",
            )
        }),
        _ => Err(schema_err(
            RateSchemaErrorKind::WrongType,
            ptr.to_owned(),
            "expected non-negative integer or decimal integer string",
            "supply a non-negative integer or canonical u64 string",
        )),
    }
}

fn parse_f64_flexible(node: &Spanned<'_>, ptr: &str) -> Result<f64, RateSchemaError> {
    match &node.value {
        Json::Num(_) => {
            let (_, x) = node.as_f64_lexeme().map_err(|e| {
                schema_err(
                    RateSchemaErrorKind::InvalidValue,
                    ptr.to_owned(),
                    e.message,
                    "supply a finite decimal number",
                )
            })?;
            Ok(x)
        }
        Json::Str(s) => {
            if s.trim() != s.as_str() || s.is_empty() {
                return Err(schema_err(
                    RateSchemaErrorKind::InvalidValue,
                    ptr.to_owned(),
                    format!("malformed numeric string `{s}`"),
                    "supply a finite decimal string without surrounding whitespace",
                ));
            }
            let x: f64 = s.parse().map_err(|_| {
                schema_err(
                    RateSchemaErrorKind::InvalidValue,
                    ptr.to_owned(),
                    format!("cannot parse `{s}` as finite f64"),
                    "supply a finite decimal string",
                )
            })?;
            if !x.is_finite() {
                return Err(schema_err(
                    RateSchemaErrorKind::InvalidValue,
                    ptr.to_owned(),
                    format!("non-finite numeric string `{s}`"),
                    "supply a finite number",
                ));
            }
            Ok(x)
        }
        _ => Err(schema_err(
            RateSchemaErrorKind::WrongType,
            ptr.to_owned(),
            "expected finite JSON number or numeric string",
            "supply a finite JSON number or decimal string",
        )),
    }
}

fn parse_inclusion(node: &Spanned<'_>, base: &str) -> Result<Inclusion, RateSchemaError> {
    let map = object_map(node, base)?;
    reject_unknown_keys(
        &map,
        &["observe_reward_reset", "policy_inference", "learning"],
        base,
    )?;
    Ok(Inclusion {
        observe_reward_reset: require_bool(&map, "observe_reward_reset", base)?,
        policy_inference: require_bool(&map, "policy_inference", base)?,
        learning: require_bool(&map, "learning", base)?,
    })
}

fn parse_counts(node: &Spanned<'_>, base: &str) -> Result<StepCounts, RateSchemaError> {
    let map = object_map(node, base)?;
    reject_unknown_keys(&map, &["valid", "degraded", "faulted"], base)?;
    let valid_ptr = join_ptr(base, "valid");
    let degraded_ptr = join_ptr(base, "degraded");
    let faulted_ptr = join_ptr(base, "faulted");
    let valid = parse_u64_flexible(
        map.get("valid").ok_or_else(|| {
            schema_err(
                RateSchemaErrorKind::MissingField,
                valid_ptr.clone(),
                "missing `counts.valid`",
                "supply valid, degraded, and faulted counts",
            )
        })?,
        &valid_ptr,
    )?;
    let degraded = parse_u64_flexible(
        map.get("degraded").ok_or_else(|| {
            schema_err(
                RateSchemaErrorKind::MissingField,
                degraded_ptr.clone(),
                "missing `counts.degraded`",
                "supply valid, degraded, and faulted counts",
            )
        })?,
        &degraded_ptr,
    )?;
    let faulted = parse_u64_flexible(
        map.get("faulted").ok_or_else(|| {
            schema_err(
                RateSchemaErrorKind::MissingField,
                faulted_ptr.clone(),
                "missing `counts.faulted`",
                "supply valid, degraded, and faulted counts",
            )
        })?,
        &faulted_ptr,
    )?;
    Ok(StepCounts {
        valid,
        degraded,
        faulted,
    })
}

fn parse_settings(node: &Spanned<'_>, base: &str) -> Result<Settings, RateSchemaError> {
    let map = object_map(node, base)?;
    let kind = require_str(&map, "kind", base)?;
    let h_str = require_str(&map, "h_s", base)?;
    let h = Rational::parse(h_str).map_err(|e| {
        schema_err(
            RateSchemaErrorKind::InvalidValue,
            join_ptr(base, "h_s"),
            e.to_string(),
            "encode h_s as a reduced positive rational string like \"1/500\"",
        )
    })?;
    let d_str = require_str(&map, "d", base)?;
    let d = Decimation::parse(d_str).map_err(|e| {
        schema_err(
            RateSchemaErrorKind::InvalidValue,
            join_ptr(base, "d"),
            e.to_string(),
            MISSING_DECIMATION_REPAIR,
        )
    })?;
    match kind {
        "matched" => {
            reject_unknown_keys(&map, &["kind", "h_s", "d", "newton_iterations"], base)?;
            let it_ptr = join_ptr(base, "newton_iterations");
            let it_node = map.get("newton_iterations").ok_or_else(|| {
                schema_err(
                    RateSchemaErrorKind::MissingField,
                    it_ptr.clone(),
                    "missing `settings.newton_iterations`",
                    "supply `newton_iterations` for matched settings",
                )
            })?;
            let it_u64 = parse_u64_flexible(it_node, &it_ptr)?;
            let newton_iterations = u8::try_from(it_u64).map_err(|_| {
                schema_err(
                    RateSchemaErrorKind::InvalidValue,
                    it_ptr,
                    "newton_iterations must fit in u8",
                    "supply a u8 iteration count",
                )
            })?;
            Ok(Settings::Matched {
                h,
                d,
                newton_iterations,
            })
        }
        "native" => {
            reject_unknown_keys(&map, &["kind", "h_s", "d", "source"], base)?;
            let source = require_str(&map, "source", base)?.to_owned();
            let s = Settings::Native { h, d, source };
            s.validate().map_err(|e| {
                schema_err(
                    RateSchemaErrorKind::InvalidValue,
                    join_ptr(base, "source"),
                    e.to_string(),
                    "supply a non-empty printable source identifier",
                )
            })?;
            Ok(s)
        }
        other => Err(schema_err(
            RateSchemaErrorKind::InvalidValue,
            join_ptr(base, "kind"),
            format!("unknown settings kind `{other}`"),
            "use \"matched\" or \"native\"",
        )),
    }
}

/// Validate and decode a single rate object from an already-admitted [`crate::json::Spanned`]
/// node, prefixing error pointers with `base_pointer` (e.g. `"rates[0]"`).
pub fn parse_rate_at(node: &Spanned<'_>, base_pointer: &str) -> Result<Rate, RateSchemaError> {
    let map = object_map(node, base_pointer)?;
    reject_unknown_keys(
        &map,
        &[
            "value",
            "unit",
            "decimation",
            "h_s",
            "scope",
            "inclusion",
            "settings",
            "counts",
            "wall_s",
        ],
        base_pointer,
    )?;

    let unit_str = require_str(&map, "unit", base_pointer)?;
    let unit = RateUnit::parse(unit_str).ok_or_else(|| {
        schema_err(
            RateSchemaErrorKind::InvalidValue,
            join_ptr(base_pointer, "unit"),
            format!("unknown rate unit `{unit_str}`"),
            "use one of world_steps_per_s, env_steps_per_s, world_s_per_wall_s, episodes_per_s, real_time_multiple",
        )
    })?;

    let scope_str = require_str(&map, "scope", base_pointer)?;
    let scope = StepScope::parse(scope_str).ok_or_else(|| {
        schema_err(
            RateSchemaErrorKind::InvalidValue,
            join_ptr(base_pointer, "scope"),
            format!("unknown step scope `{scope_str}`"),
            "use physics_only, full_env_step, or training",
        )
    })?;

    let inc_ptr = join_ptr(base_pointer, "inclusion");
    let inclusion = parse_inclusion(
        map.get("inclusion").ok_or_else(|| {
            schema_err(
                RateSchemaErrorKind::MissingField,
                inc_ptr.clone(),
                "missing `inclusion` object",
                "supply the inclusion flags object",
            )
        })?,
        &inc_ptr,
    )?;

    let set_ptr = join_ptr(base_pointer, "settings");
    let settings = parse_settings(
        map.get("settings").ok_or_else(|| {
            schema_err(
                RateSchemaErrorKind::MissingField,
                set_ptr.clone(),
                "missing `settings` object",
                "supply matched or native settings",
            )
        })?,
        &set_ptr,
    )?;

    let cnt_ptr = join_ptr(base_pointer, "counts");
    let counts = parse_counts(
        map.get("counts").ok_or_else(|| {
            schema_err(
                RateSchemaErrorKind::MissingField,
                cnt_ptr.clone(),
                "missing `counts` object",
                "supply valid, degraded, and faulted step counts",
            )
        })?,
        &cnt_ptr,
    )?;

    let wall_ptr = join_ptr(base_pointer, "wall_s");
    let wall_s = parse_f64_flexible(
        map.get("wall_s").ok_or_else(|| {
            schema_err(
                RateSchemaErrorKind::MissingField,
                wall_ptr.clone(),
                "missing `wall_s` interval",
                "supply submission-to-completion wall_s",
            )
        })?,
        &wall_ptr,
    )?;

    let rate = match unit {
        RateUnit::WorldStepsPerSec | RateUnit::EnvStepsPerSec | RateUnit::WorldSecPerWallSec => {
            let dec_ptr = join_ptr(base_pointer, "decimation");
            let dec_node = map.get("decimation").ok_or_else(|| {
                schema_err(
                    RateSchemaErrorKind::MissingDecimation,
                    dec_ptr.clone(),
                    format!("step rate `{unit}` requires `decimation`"),
                    MISSING_DECIMATION_REPAIR,
                )
            })?;
            let dec_str = dec_node.value.as_str().ok_or_else(|| {
                schema_err(
                    RateSchemaErrorKind::WrongType,
                    dec_ptr.clone(),
                    "`decimation` must be a reduced rational string like \"10/1\"",
                    MISSING_DECIMATION_REPAIR,
                )
            })?;
            let decimation = Decimation::parse(dec_str).map_err(|e| {
                schema_err(
                    RateSchemaErrorKind::InvalidValue,
                    dec_ptr,
                    e.to_string(),
                    MISSING_DECIMATION_REPAIR,
                )
            })?;

            let h_ptr = join_ptr(base_pointer, "h_s");
            let h_str = require_str(&map, "h_s", base_pointer)?;
            let h = Rational::parse(h_str).map_err(|e| {
                schema_err(
                    RateSchemaErrorKind::InvalidValue,
                    h_ptr,
                    e.to_string(),
                    "encode h_s as a reduced positive rational string like \"1/500\"",
                )
            })?;

            let step_unit = match unit {
                RateUnit::WorldStepsPerSec => StepUnit::WorldStepsPerSec,
                RateUnit::EnvStepsPerSec => StepUnit::EnvStepsPerSec,
                RateUnit::WorldSecPerWallSec => StepUnit::WorldSecPerWallSec,
                RateUnit::EpisodesPerSec | RateUnit::RealTimeMultiple => {
                    return Err(schema_err(
                        RateSchemaErrorKind::InvalidValue,
                        join_ptr(base_pointer, "unit"),
                        "non-step unit cannot enter step-unit branch",
                        "use a step-family unit",
                    ));
                }
            };
            Rate::steps(
                step_unit,
                decimation,
                h,
                scope,
                inclusion,
                settings,
                counts,
                wall_s,
            )
            .map_err(|e| {
                schema_err(
                    RateSchemaErrorKind::InvariantViolation,
                    base_pointer.to_owned(),
                    e.to_string(),
                    "fix rate fields so counts, wall_s, scope, inclusion, and settings agree",
                )
            })?
        }
        RateUnit::RealTimeMultiple | RateUnit::EpisodesPerSec => {
            if map.contains_key("decimation") {
                return Err(schema_err(
                    RateSchemaErrorKind::InvalidValue,
                    join_ptr(base_pointer, "decimation"),
                    format!("unit `{unit}` does not carry a decimation"),
                    "omit `decimation` for non-step units",
                ));
            }
            if map.contains_key("h_s") {
                return Err(schema_err(
                    RateSchemaErrorKind::InvalidValue,
                    join_ptr(base_pointer, "h_s"),
                    format!("unit `{unit}` does not carry top-level `h_s`"),
                    "omit `h_s` for non-step units",
                ));
            }
            let built = if unit == RateUnit::RealTimeMultiple {
                Rate::real_time_multiple(scope, inclusion, settings, counts, wall_s)
            } else {
                Rate::episodes(scope, inclusion, settings, counts, wall_s)
            };
            built.map_err(|e| {
                schema_err(
                    RateSchemaErrorKind::InvariantViolation,
                    base_pointer.to_owned(),
                    e.to_string(),
                    "fix rate fields so counts, wall_s, scope, inclusion, and settings agree",
                )
            })?
        }
    };

    // If wire JSON includes `"value"`, verify it agrees with `counts.valid / wall_s`
    // (the constructor always derives `rate.value` from `counts.valid` and `wall_s`).
    if let Some(val_node) = map.get("value") {
        let val_ptr = join_ptr(base_pointer, "value");
        let wire_val = parse_f64_flexible(val_node, &val_ptr)?;
        let expected = rate.value();
        let diff = (wire_val - expected).abs();
        let tol = 1e-9_f64 * expected.abs().max(1.0);
        if diff > tol {
            return Err(schema_err(
                RateSchemaErrorKind::InvariantViolation,
                val_ptr,
                format!(
                    "reported value {wire_val} disagrees with counts.valid/wall_s ({expected})"
                ),
                "recompute value from counts.valid and wall_s without degraded or faulted steps",
            ));
        }
    }

    Ok(rate)
}

/// Parse a rate array from raw JSON bytes (`{"rates": [...]}` or `[...]`) through
/// [`crate::json::admit`], reporting errors with pointers `rates[i].<field>`.
pub fn parse_rates_json(bytes: &[u8]) -> Result<Vec<Rate>, RateSchemaError> {
    let admitted = json::admit(bytes, &Limits::default()).map_err(|f| RateSchemaError {
        kind: RateSchemaErrorKind::JsonAdmission,
        pointer: f.pointer.clone(),
        message: f.to_string(),
        repair: "supply valid strict RFC 8259 JSON without duplicate keys",
    })?;
    let items: &[Spanned<'_>] = match &admitted.root().value {
        Json::Arr(arr) => arr.as_slice(),
        Json::Obj(_) => {
            let map = object_map(admitted.root(), "")?;
            let node = map.get("rates").ok_or_else(|| {
                schema_err(
                    RateSchemaErrorKind::MissingField,
                    "rates".to_owned(),
                    "missing `rates` array",
                    "supply a `rates` JSON array",
                )
            })?;
            node.value.as_array().ok_or_else(|| {
                schema_err(
                    RateSchemaErrorKind::WrongType,
                    "rates".to_owned(),
                    "`rates` must be a JSON array",
                    "encode `rates` as a JSON array",
                )
            })?
        }
        _ => {
            return Err(schema_err(
                RateSchemaErrorKind::WrongType,
                "rates".to_owned(),
                "expected JSON array or object with `rates` field",
                "supply a JSON array or `{\"rates\": [...]}` object",
            ));
        }
    };

    let mut out = Vec::with_capacity(items.len());
    for (i, item) in items.iter().enumerate() {
        out.push(parse_rate_at(item, &format!("rates[{i}]"))?);
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn h_2ms() -> Rational {
        Rational::new(1, 500).expect("1/500")
    }

    fn d10() -> Decimation {
        Decimation::integer(10).expect("d=10")
    }

    fn matched_q() -> Settings {
        Settings::Matched {
            h: h_2ms(),
            d: d10(),
            newton_iterations: 4,
        }
    }

    fn key_trj_q() -> HardwareKey {
        HardwareKey {
            machine: "trj".to_owned(),
            device: "rtx-4090-0".to_owned(),
            world_count: 4096,
            workload: "Q".to_owned(),
            settings: matched_q(),
        }
    }

    fn rate_env(valid: u64, wall_s: f64, scope: StepScope, inc: Inclusion) -> Rate {
        Rate::steps(
            StepUnit::EnvStepsPerSec,
            d10(),
            h_2ms(),
            scope,
            inc,
            matched_q(),
            StepCounts {
                valid,
                degraded: 0,
                faulted: 0,
            },
            wall_s,
        )
        .expect("valid env rate")
    }

    fn rate_world(valid: u64, wall_s: f64, scope: StepScope, inc: Inclusion) -> Rate {
        Rate::steps(
            StepUnit::WorldStepsPerSec,
            d10(),
            h_2ms(),
            scope,
            inc,
            matched_q(),
            StepCounts {
                valid,
                degraded: 0,
                faulted: 0,
            },
            wall_s,
        )
        .expect("valid world rate")
    }

    #[test]
    fn conversion_world_env() {
        let d_10 = Decimation::integer(10).unwrap();
        let d_4 = Decimation::integer(4).unwrap();
        assert_eq!(
            env_steps_to_world_steps(400_000, d_10).unwrap().exact_u64(),
            Some(4_000_000)
        );
        assert_eq!(
            env_steps_to_world_steps(1_200_000, d_4)
                .unwrap()
                .exact_u64(),
            Some(4_800_000)
        );
        assert_eq!(
            world_steps_to_env_steps(4_800_000, d_4)
                .unwrap()
                .exact_u64(),
            Some(1_200_000)
        );

        let r = rate_env(400_000, 1.0, StepScope::Training, Inclusion::training());
        assert_eq!(r.as_world_steps_per_s().unwrap(), 4_000_000.0);
        assert_eq!(r.scope(), StepScope::Training);
    }

    #[test]
    fn world_seconds() {
        let h = h_2ms();
        assert_eq!(
            world_steps_to_world_seconds(1_700_000, h)
                .unwrap()
                .exact_u64(),
            Some(3_400)
        );
        assert_eq!(
            world_steps_to_world_seconds(275_000, h)
                .unwrap()
                .exact_u64(),
            Some(550)
        );

        let r = rate_world(
            1_700_000,
            1.0,
            StepScope::PhysicsOnly,
            Inclusion::physics_only(),
        );
        assert_eq!(r.as_world_sec_per_wall_s().unwrap(), 3_400.0);
    }

    #[test]
    fn ordering_known_answer() {
        // KA-806: 400k env-steps/s training (d=10 -> 4.0M world-steps/s) beside
        // 2.0M world-steps/s physics (d=10 -> 200k env-steps/s) on one hardware key.
        // Fails under `--features nc-ordering-compare-raw`.
        let key = key_trj_q();
        let training = rate_env(400_000, 1.0, StepScope::Training, Inclusion::training());
        let physics = rate_world(
            2_000_000,
            1.0,
            StepScope::PhysicsOnly,
            Inclusion::physics_only(),
        );
        let violation = check_ordering(&[
            KeyedRate {
                key: key.clone(),
                rate: training,
            },
            KeyedRate {
                key: key.clone(),
                rate: physics,
            },
        ])
        .expect_err("KA-806 report must be rejected by INV-PERF-01");

        assert_eq!(violation.hardware_key, key);
        assert_eq!(violation.slower_scope, StepScope::Training);
        assert_eq!(violation.faster_scope, StepScope::PhysicsOnly);
        assert_eq!(violation.slower_env_steps_per_s, 400_000.0);
        assert_eq!(violation.slower_world_steps_per_s, 4_000_000.0);
        assert_eq!(violation.faster_env_steps_per_s, 200_000.0);
        assert_eq!(violation.faster_world_steps_per_s, 2_000_000.0);
        assert_eq!(violation.detail, ORDERING_DETAIL);
        assert_eq!(violation.pointer, ORDERING_POINTER);
        assert_eq!(violation.repair, ORDERING_REPAIR);
        assert!(violation.to_string().contains("INV-PERF-01"));
    }

    #[test]
    fn ordering_accepts_consistent_triple() {
        let key = key_trj_q();
        let physics = rate_env(400_000, 1.0, StepScope::PhysicsOnly, Inclusion::physics_only());
        let full = rate_env(
            340_000,
            1.0,
            StepScope::FullEnvStep,
            Inclusion::full_env_step(),
        );
        let training = rate_env(280_000, 1.0, StepScope::Training, Inclusion::training());
        check_group_ordering(&key, &[physics, full, training])
            .expect("400k >= 340k >= 280k must pass INV-PERF-01");
    }

    #[test]
    fn ordering_rejects_equal_scope_mismatch() {
        let key = key_trj_q();
        let physics = rate_env(400_000, 1.0, StepScope::PhysicsOnly, Inclusion::physics_only());
        let full = rate_env(
            400_001,
            1.0,
            StepScope::FullEnvStep,
            Inclusion::full_env_step(),
        );
        let err = check_group_ordering(&key, &[physics, full])
            .expect_err("FullEnvStep above PhysicsOnly by 1 step/s must fail strictly");
        assert_eq!(err.slower_scope, StepScope::FullEnvStep);
        assert_eq!(err.faster_scope, StepScope::PhysicsOnly);
        assert_eq!(err.slower_env_steps_per_s, 400_001.0);
        assert_eq!(err.faster_env_steps_per_s, 400_000.0);
    }

    #[test]
    fn missing_decimation_is_schema_error() {
        let raw = br#"{
            "rates": [
                {
                    "value": "400000",
                    "unit": "env_steps_per_s",
                    "h_s": "1/500",
                    "scope": "training",
                    "inclusion": {
                        "observe_reward_reset": true,
                        "policy_inference": true,
                        "learning": true
                    },
                    "settings": {
                        "kind": "matched",
                        "h_s": "1/500",
                        "d": "10/1",
                        "newton_iterations": 4
                    },
                    "counts": { "valid": 400000, "degraded": 0, "faulted": 0 },
                    "wall_s": "1"
                }
            ]
        }"#;
        let err = parse_rates_json(raw).expect_err("missing decimation must fail schema validation");
        assert_eq!(err.kind, RateSchemaErrorKind::MissingDecimation);
        assert_eq!(err.pointer, "rates[0].decimation");
        assert_eq!(err.repair, MISSING_DECIMATION_REPAIR);
    }

    #[test]
    fn ordering_constants() {
        assert_eq!(ORDERING_DETAIL, "rates-violate-unit-ordering");
        assert_eq!(ORDERING_POINTER, "rates.ordering");
        assert_eq!(ORDERING_INTERIM_REFUSAL_CODE, "FROB-REF-862");
    }

    #[test]
    fn rational_decimation() {
        let control_25ms = Rational::new(1, 40).unwrap();
        let h = h_2ms();
        let d = Decimation::from_clocks(control_25ms, h).expect("25ms / 2ms = 25/2");
        assert_eq!(d.num(), 25);
        assert_eq!(d.den(), 2);
        assert!(!d.is_integer());
        assert_eq!(d.to_slash_string(), "25/2");
        assert_eq!(Decimation::parse("25/2").unwrap(), d);
        assert_eq!(
            env_steps_to_world_steps(400, d).unwrap().exact_u64(),
            Some(5_000)
        );

        let settings = Settings::Matched {
            h,
            d,
            newton_iterations: 4,
        };
        let r = Rate::steps(
            StepUnit::EnvStepsPerSec,
            d,
            h,
            StepScope::FullEnvStep,
            Inclusion::full_env_step(),
            settings,
            StepCounts {
                valid: 400,
                degraded: 0,
                faulted: 0,
            },
            1.0,
        )
        .unwrap();
        assert_eq!(r.as_world_steps_per_s().unwrap(), 5_000.0);
    }

    #[test]
    fn numerator_excludes_degraded_and_faulted() {
        let counts = StepCounts::from_total(1_000_000, 300, 0).expect("999,700 valid");
        assert_eq!(counts.valid, 999_700);
        assert_eq!(counts.degraded, 300);
        assert_eq!(counts.faulted, 0);

        let r = Rate::steps(
            StepUnit::WorldStepsPerSec,
            d10(),
            h_2ms(),
            StepScope::PhysicsOnly,
            Inclusion::physics_only(),
            matched_q(),
            counts,
            1.0,
        )
        .unwrap();
        assert_eq!(r.counts().valid, 999_700);
        assert_eq!(r.value(), 999_700.0);

        let with_faulted = StepCounts::from_total(1_000_000, 300, 200).expect("999,500 valid");
        let r2 = Rate::steps(
            StepUnit::WorldStepsPerSec,
            d10(),
            h_2ms(),
            StepScope::PhysicsOnly,
            Inclusion::physics_only(),
            matched_q(),
            with_faulted,
            1.0,
        )
        .unwrap();
        assert_eq!(r2.value(), 999_500.0);
    }

    #[test]
    fn scope_inclusion_agreement() {
        let counts = StepCounts {
            valid: 100_000,
            degraded: 0,
            faulted: 0,
        };
        let bad_phys = Rate::steps(
            StepUnit::WorldStepsPerSec,
            d10(),
            h_2ms(),
            StepScope::PhysicsOnly,
            Inclusion {
                observe_reward_reset: false,
                policy_inference: false,
                learning: true,
            },
            matched_q(),
            counts,
            1.0,
        );
        assert!(matches!(
            bad_phys,
            Err(UnitsError::ScopeInclusionMismatch {
                scope: StepScope::PhysicsOnly,
                ..
            })
        ));

        let bad_train = Rate::steps(
            StepUnit::EnvStepsPerSec,
            d10(),
            h_2ms(),
            StepScope::Training,
            Inclusion::full_env_step(),
            matched_q(),
            counts,
            1.0,
        );
        assert!(matches!(
            bad_train,
            Err(UnitsError::ScopeInclusionMismatch {
                scope: StepScope::Training,
                ..
            })
        ));

        let bad_full = Rate::steps(
            StepUnit::EnvStepsPerSec,
            d10(),
            h_2ms(),
            StepScope::FullEnvStep,
            Inclusion::training(),
            matched_q(),
            counts,
            1.0,
        );
        assert!(matches!(
            bad_full,
            Err(UnitsError::ScopeInclusionMismatch {
                scope: StepScope::FullEnvStep,
                ..
            })
        ));
    }

    #[test]
    fn native_world_steps_not_comparable() {
        // CX-374 incumbent pair: d = 5 (h = 4 ms) vs d = 4 (h = 5 ms), and d = 4 vs d = 10.
        // Fails under `--features units-without-decimation`.
        let h4 = Rational::new(1, 250).unwrap();
        let h5 = Rational::new(1, 200).unwrap();
        let d5 = Decimation::integer(5).unwrap();
        let d4 = Decimation::integer(4).unwrap();
        let nat_d5 = Settings::Native {
            h: h4,
            d: d5,
            source: "incumbent-a-native".to_owned(),
        };
        let nat_d4 = Settings::Native {
            h: h5,
            d: d4,
            source: "incumbent-b-native".to_owned(),
        };
        let nat_d10 = Settings::Native {
            h: h_2ms(),
            d: d10(),
            source: "frob-native".to_owned(),
        };
        let c = StepCounts {
            valid: 4_800_000,
            degraded: 0,
            faulted: 0,
        };

        let w_d5 = Rate::steps(
            StepUnit::WorldStepsPerSec,
            d5,
            h4,
            StepScope::PhysicsOnly,
            Inclusion::physics_only(),
            nat_d5.clone(),
            c,
            1.0,
        )
        .unwrap();
        let w_d4 = Rate::steps(
            StepUnit::WorldStepsPerSec,
            d4,
            h5,
            StepScope::PhysicsOnly,
            Inclusion::physics_only(),
            nat_d4.clone(),
            c,
            1.0,
        )
        .unwrap();
        let w_d10 = Rate::steps(
            StepUnit::WorldStepsPerSec,
            d10(),
            h_2ms(),
            StepScope::PhysicsOnly,
            Inclusion::physics_only(),
            nat_d10.clone(),
            c,
            1.0,
        )
        .unwrap();

        assert!(!comparable(&w_d5, &w_d4));
        assert!(!comparable(&w_d4, &w_d10));

        // Same pair expressed in env-steps/s under Native settings IS comparable.
        let e_d4 = Rate::steps(
            StepUnit::EnvStepsPerSec,
            d4,
            h5,
            StepScope::PhysicsOnly,
            Inclusion::physics_only(),
            nat_d4,
            StepCounts {
                valid: 1_200_000,
                degraded: 0,
                faulted: 0,
            },
            1.0,
        )
        .unwrap();
        let e_d10 = Rate::steps(
            StepUnit::EnvStepsPerSec,
            d10(),
            h_2ms(),
            StepScope::PhysicsOnly,
            Inclusion::physics_only(),
            nat_d10,
            StepCounts {
                valid: 480_000,
                degraded: 0,
                faulted: 0,
            },
            1.0,
        )
        .unwrap();
        assert!(comparable(&e_d4, &e_d10));
        // Mixing env-steps/s and world-steps/s under Native is refused.
        assert!(!comparable(&e_d4, &w_d10));
    }

    #[test]
    fn matched_requires_equal_h_d_iterations() {
        let h2 = h_2ms();
        let h4 = Rational::new(1, 250).unwrap();
        let d10 = d10();
        let d5 = Decimation::integer(5).unwrap();
        let c = StepCounts {
            valid: 1_000_000,
            degraded: 0,
            faulted: 0,
        };

        let base_w = Rate::steps(
            StepUnit::WorldStepsPerSec,
            d10,
            h2,
            StepScope::PhysicsOnly,
            Inclusion::physics_only(),
            Settings::Matched {
                h: h2,
                d: d10,
                newton_iterations: 4,
            },
            c,
            1.0,
        )
        .unwrap();
        let base_e = Rate::steps(
            StepUnit::EnvStepsPerSec,
            d10,
            h2,
            StepScope::PhysicsOnly,
            Inclusion::physics_only(),
            Settings::Matched {
                h: h2,
                d: d10,
                newton_iterations: 4,
            },
            StepCounts {
                valid: 100_000,
                degraded: 0,
                faulted: 0,
            },
            1.0,
        )
        .unwrap();
        assert!(comparable(&base_w, &base_e));

        let diff_iter = Rate::steps(
            StepUnit::WorldStepsPerSec,
            d10,
            h2,
            StepScope::PhysicsOnly,
            Inclusion::physics_only(),
            Settings::Matched {
                h: h2,
                d: d10,
                newton_iterations: 6,
            },
            c,
            1.0,
        )
        .unwrap();
        let diff_h = Rate::steps(
            StepUnit::WorldStepsPerSec,
            d10,
            h4,
            StepScope::PhysicsOnly,
            Inclusion::physics_only(),
            Settings::Matched {
                h: h4,
                d: d10,
                newton_iterations: 4,
            },
            c,
            1.0,
        )
        .unwrap();
        let diff_d = Rate::steps(
            StepUnit::WorldStepsPerSec,
            d5,
            h2,
            StepScope::PhysicsOnly,
            Inclusion::physics_only(),
            Settings::Matched {
                h: h2,
                d: d5,
                newton_iterations: 4,
            },
            c,
            1.0,
        )
        .unwrap();
        assert!(!comparable(&base_w, &diff_iter));
        assert!(!comparable(&base_w, &diff_h));
        assert!(!comparable(&base_w, &diff_d));
    }

    #[test]
    fn wire_form_roundtrip_and_large_counts() {
        let large_valid = (INT53_MAX as u64) + 42;
        let r = Rate::steps(
            StepUnit::EnvStepsPerSec,
            d10(),
            h_2ms(),
            StepScope::Training,
            Inclusion::training(),
            matched_q(),
            StepCounts {
                valid: large_valid,
                degraded: 7,
                faulted: 1,
            },
            1000.0,
        )
        .unwrap();
        let bytes = r.to_canonical_json().expect("canonical rate JSON");
        assert!(bytes.as_str().contains(&format!("\"{large_valid}\"")));
        let decoded = Rate::from_json_bytes(bytes.as_bytes()).expect("roundtrip rate JSON");
        assert_eq!(decoded, r);

        // RealTimeMultiple takes no decimation and roundtrips cleanly.
        let rtm = Rate::real_time_multiple(
            StepScope::PhysicsOnly,
            Inclusion::physics_only(),
            matched_q(),
            StepCounts {
                valid: 5_000,
                degraded: 0,
                faulted: 0,
            },
            2.0,
        )
        .unwrap();
        assert_eq!(rtm.decimation(), None);
        assert_eq!(rtm.value(), 5.0);
        let rtm_bytes = rtm.to_canonical_json().unwrap();
        assert!(!rtm_bytes.as_str().contains("\"decimation\""));
        assert_eq!(Rate::from_json_bytes(rtm_bytes.as_bytes()).unwrap(), rtm);
    }

    fn verify_ordering_triple(
        t_phys_us: u64,
        t_over_us: u64,
        t_learn_us: u64,
        d_idx: u32,
    ) -> Result<(), String> {
        let decs = [
            Decimation::integer(1).unwrap(),
            Decimation::integer(4).unwrap(),
            Decimation::integer(5).unwrap(),
            Decimation::integer(10).unwrap(),
            Decimation::new(25, 2).unwrap(),
        ];
        let d = decs[(d_idx as usize) % decs.len()];
        let h = h_2ms();
        let settings = Settings::Matched {
            h,
            d,
            newton_iterations: 4,
        };
        let key = HardwareKey {
            machine: "prop".to_owned(),
            device: "cpu".to_owned(),
            world_count: 64,
            workload: "Q".to_owned(),
            settings: settings.clone(),
        };
        let n_env = u64::from(d.den()) * 100;
        let n_world = (n_env / u64::from(d.den())) * u64::from(d.num());

        let wall_phys = (t_phys_us as f64) * 1e-6;
        let wall_full = ((t_phys_us + t_over_us) as f64) * 1e-6;
        let wall_train = ((t_phys_us + t_over_us + t_learn_us) as f64) * 1e-6;

        let mk_phys = |wall_s: f64| {
            Rate::steps(
                StepUnit::WorldStepsPerSec,
                d,
                h,
                StepScope::PhysicsOnly,
                Inclusion::physics_only(),
                settings.clone(),
                StepCounts {
                    valid: n_world,
                    degraded: 0,
                    faulted: 0,
                },
                wall_s,
            )
            .unwrap()
        };
        let mk_full = |wall_s: f64| {
            Rate::steps(
                StepUnit::EnvStepsPerSec,
                d,
                h,
                StepScope::FullEnvStep,
                Inclusion::full_env_step(),
                settings.clone(),
                StepCounts {
                    valid: n_env,
                    degraded: 0,
                    faulted: 0,
                },
                wall_s,
            )
            .unwrap()
        };
        let mk_train = |wall_s: f64| {
            Rate::steps(
                StepUnit::EnvStepsPerSec,
                d,
                h,
                StepScope::Training,
                Inclusion::training(),
                settings.clone(),
                StepCounts {
                    valid: n_env,
                    degraded: 0,
                    faulted: 0,
                },
                wall_s,
            )
            .unwrap()
        };

        // Derived triple must always pass INV-PERF-01.
        check_group_ordering(&key, &[mk_phys(wall_phys), mk_full(wall_full), mk_train(wall_train)])
            .map_err(|e| format!("valid derived triple failed: {e}"))?;

        // Swapping physics and full fails iff t_over_us > 0.
        let swap_pf = check_group_ordering(
            &key,
            &[mk_phys(wall_full), mk_full(wall_phys), mk_train(wall_train)],
        );
        if t_over_us > 0 && swap_pf.is_ok() {
            return Err("swapping physics and full passed despite overhead > 0".to_owned());
        }
        if t_over_us == 0 && swap_pf.is_err() {
            return Err("swapping equal physics and full failed".to_owned());
        }

        // Swapping full and training fails iff t_learn_us > 0.
        let swap_ft = check_group_ordering(
            &key,
            &[mk_phys(wall_phys), mk_full(wall_train), mk_train(wall_full)],
        );
        if t_learn_us > 0 && swap_ft.is_ok() {
            return Err("swapping full and training passed despite learner > 0".to_owned());
        }
        if t_learn_us == 0 && swap_ft.is_err() {
            return Err("swapping equal full and training failed".to_owned());
        }

        // Swapping physics and training fails iff t_over_us + t_learn_us > 0.
        let swap_pt = check_group_ordering(
            &key,
            &[mk_phys(wall_train), mk_full(wall_full), mk_train(wall_phys)],
        );
        if (t_over_us + t_learn_us) > 0 && swap_pt.is_ok() {
            return Err("swapping physics and training passed despite overhead+learner > 0".to_owned());
        }
        if t_over_us == 0 && t_learn_us == 0 && swap_pt.is_err() {
            return Err("swapping equal physics and training failed".to_owned());
        }
        Ok(())
    }

    #[test]
    fn ordering_triple_property_deterministic() {
        let mut state: u64 = 0x5EED_CAFE_1234_5678;
        for case in 0..256u32 {
            state = state
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1_442_695_040_888_963_407);
            let t_phys = 1 + (state % 10_000);
            state = state
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1_442_695_040_888_963_407);
            let t_over = if case % 8 == 0 { 0 } else { state % 10_000 };
            state = state
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1_442_695_040_888_963_407);
            let t_learn = if case % 10 == 0 { 0 } else { state % 10_000 };
            assert_eq!(verify_ordering_triple(t_phys, t_over, t_learn, case), Ok(()));
        }
    }

    #[cfg(feature = "test-support")]
    #[test]
    fn prop_inv_perf_01_testkit_runner() {
        let runner = crate::testkit::prop::Runner::new("frob-core", "perf::units::inv_perf_01", 256);
        let boundaries = [
            ((1u64, 0u64), (0u64, 0u32)),
            ((1u64, 1u64), (0u64, 3u32)),
            ((1u64, 0u64), (1u64, 4u32)),
            ((500u64, 250u64), (250u64, 4u32)),
        ];
        let res = runner.run_shrink(
            &boundaries,
            |g| {
                let t_phys = g.u64_range(0, 1, 10_000);
                let t_over = g.u64_range(1, 0, 10_000);
                let t_learn = g.u64_range(2, 0, 10_000);
                let d_idx = g.u64_range(3, 0, 4) as u32;
                ((t_phys, t_over), (t_learn, d_idx))
            },
            |&((t_phys, t_over), (t_learn, d_idx))| {
                verify_ordering_triple(t_phys, t_over, t_learn, d_idx)
            },
        );
        assert!(res.is_ok(), "property runner failed: {res:?}");
    }
}
