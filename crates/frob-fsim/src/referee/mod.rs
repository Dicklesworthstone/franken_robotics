#![forbid(unsafe_code)]

//! `referee` airlock group (`N33a-11`, `BD-007`, default on).
//!
//! Exposes FrankenRobotics-native traits, coordinate-layout metadata, canonical
//! state digest ([`digest`]), and structured [`RefereeRefusal`] errors. No
//! volatile upstream crate or type appears in any public signature.

use std::fmt;

use crate::groups::{Group, GroupAbsent};

pub mod digest;

#[cfg(feature = "referee")]
mod dynamics;

#[cfg(feature = "referee")]
pub use dynamics::InProcRefereeDynamics;

/// The airlock group represented by this module.
pub const GROUP: Group = Group::Referee;

/// Whether the `referee` feature was enabled at compile time.
pub const ENABLED: bool = cfg!(feature = "referee");

/// Standard ISO 80000 world gravity magnitude (`m/s^2`) used by the referee dynamics lane.
pub const STANDARD_GRAVITY_M_S2: f64 = 9.806_65;

/// Canonical orientation parameterization recorded in [`CoordLayout`].
pub const ORIENTATION_PARAMETERIZATION: &str = "unit-quaternion-wxyz";

/// Built-in catalog robot models reachable through the referee airlock (`BD-007`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CatalogRobot {
    /// Unitree G1 mode-11 29-DoF whole-body humanoid (30 retained links, 31.96314202 kg).
    G1,
}

impl CatalogRobot {
    /// Canonical lower-case identifier string.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::G1 => "g1",
        }
    }
}

/// Positive exact rational step duration `num / den` in seconds (`BD-007`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Rational {
    num: u64,
    den: u64,
}

impl Rational {
    /// Standard M0 referee rollout timestep `1/480` s.
    pub const ONE_OVER_480: Self = Self { num: 1, den: 480 };

    /// Construct a rational step `num / den` (`num > 0`, `den > 0`).
    #[must_use]
    pub const fn new(num: u64, den: u64) -> Option<Self> {
        if num == 0 || den == 0 {
            None
        } else {
            Some(Self { num, den })
        }
    }

    /// Construct from raw numerator and denominator (validated when passed to `step_free`).
    #[must_use]
    pub const fn from_raw(num: u64, den: u64) -> Self {
        Self { num, den }
    }

    /// Numerator.
    #[must_use]
    pub const fn num(self) -> u64 {
        self.num
    }

    /// Denominator.
    #[must_use]
    pub const fn den(self) -> u64 {
        self.den
    }

    /// Correctly rounded IEEE-754 `f64` quotient `(num as f64) / (den as f64)` when valid.
    #[must_use]
    pub fn to_f64(self) -> Option<f64> {
        if self.num == 0 || self.den == 0 {
            return None;
        }
        let val = (self.num as f64) / (self.den as f64);
        if val.is_finite() && val > 0.0 {
            Some(val)
        } else {
            None
        }
    }
}

impl From<frob_core::perf::workload::Rational> for Rational {
    fn from(value: frob_core::perf::workload::Rational) -> Self {
        Self {
            num: value.num(),
            den: value.den(),
        }
    }
}

/// Classification of one coordinate slot in [`CoordLayout`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CoordKind {
    BasePosition,
    BaseOrientationQuatWxyz,
    BaseAngularVelocityBody,
    BaseLinearVelocityBody,
    RevoluteJoint,
    PrismaticJoint,
    HelicalJoint,
}

impl CoordKind {
    /// Canonical hyphenated kind name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BasePosition => "base-position",
            Self::BaseOrientationQuatWxyz => "base-orientation-quat-wxyz",
            Self::BaseAngularVelocityBody => "base-angular-velocity-body",
            Self::BaseLinearVelocityBody => "base-linear-velocity-body",
            Self::RevoluteJoint => "revolute-joint",
            Self::PrismaticJoint => "prismatic-joint",
            Self::HelicalJoint => "helical-joint",
        }
    }
}

/// Self-describing entry for one generalized position or velocity coordinate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoordEntry {
    pub index: u32,
    pub name: String,
    pub kind: CoordKind,
    pub unit: &'static str,
}

/// Complete generalized-coordinate layout read from the underlying model at the pin.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoordLayout {
    pub orientation_parameterization: &'static str,
    pub position: Vec<CoordEntry>,
    pub velocity: Vec<CoordEntry>,
    pub actuators: Vec<String>,
    pub link_names: Vec<String>,
}

/// Model structural facts reported by [`RefereeDynamics::describe`].
#[derive(Debug, Clone, PartialEq)]
pub struct ModelFacts {
    pub links: u32,
    pub nq: u32,
    pub nv: u32,
    pub actuators: u32,
    pub layout: CoordLayout,
    pub mass_kg: f64,
}

/// Plain `f64` generalized position and velocity state for the referee tier (`BD-007`).
#[derive(Debug, Clone, PartialEq)]
pub struct RefereeState {
    pub q: Vec<f64>,
    pub v: Vec<f64>,
}

/// Machine-readable category of a [`RefereeRefusal`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RefereeRefusalKind {
    /// A coordinate, velocity, or torque element is `NaN` or infinite (refused before call).
    NonFinite {
        field: &'static str,
        index: usize,
    },
    /// A state or torque slice length disagrees with `nq`, `nv`, or `actuators`.
    LengthMismatch {
        field: &'static str,
        expected: usize,
        actual: usize,
    },
    /// Step duration is zero, infinite, or non-positive.
    InvalidStep,
    /// Upstream solver or kinematics boundary refused; original reason text preserved.
    Upstream,
}

/// Typed refusal returned by [`RefereeDynamics`] operations (`BD-007`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RefereeRefusal {
    pub kind: RefereeRefusalKind,
    pub reason: String,
}

impl RefereeRefusal {
    /// Construct a typed refusal with its human-readable reason string.
    #[must_use]
    pub fn new(kind: RefereeRefusalKind, reason: impl Into<String>) -> Self {
        Self {
            kind,
            reason: reason.into(),
        }
    }

    /// Construct a pre-call non-finite refusal naming the offending slice and index.
    #[must_use]
    pub fn non_finite(field: &'static str, index: usize) -> Self {
        Self::new(
            RefereeRefusalKind::NonFinite { field, index },
            format!("non-finite value in `{field}` at index {index}"),
        )
    }

    /// Construct a pre-call length mismatch refusal naming expected and supplied lengths.
    #[must_use]
    pub fn length_mismatch(field: &'static str, expected: usize, actual: usize) -> Self {
        Self::new(
            RefereeRefusalKind::LengthMismatch {
                field,
                expected,
                actual,
            },
            format!("length mismatch for `{field}`: expected {expected}, got {actual}"),
        )
    }
}

impl fmt::Display for RefereeRefusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.reason)
    }
}

impl std::error::Error for RefereeRefusal {}

/// Opaque airlock-owned handle to a validated articulated model (`BD-007`).
#[derive(Debug, Clone)]
pub struct RefereeModel {
    #[cfg(feature = "referee")]
    inner: dynamics::RefereeModelInner,
    #[cfg(not(feature = "referee"))]
    _unavailable: (),
}

impl RefereeModel {
    #[cfg(feature = "referee")]
    fn from_inner(inner: dynamics::RefereeModelInner) -> Self {
        Self { inner }
    }

    #[cfg(feature = "referee")]
    fn inner(&self) -> &dynamics::RefereeModelInner {
        &self.inner
    }
}

/// Build an articulated model and evaluate forward dynamics and contact-free steps in `f64`.
pub trait RefereeDynamics {
    /// Human-readable adapter label.
    fn adapter_name(&self) -> &'static str;

    /// Load a built-in catalog robot model through the airlock.
    fn catalog_model(&self, which: CatalogRobot) -> Result<RefereeModel, RefereeRefusal>;

    /// Return the catalog default reference pose (stationary identity base, neutral joints).
    fn reference_state(&self, m: &RefereeModel) -> RefereeState;

    /// Compute free-floating forward-dynamics generalized accelerations (`len == nv`).
    fn forward_dynamics(
        &self,
        m: &RefereeModel,
        s: &RefereeState,
        tau: &[f64],
    ) -> Result<Vec<f64>, RefereeRefusal>;

    /// Advance the contact-free free-floating articulated state by exact rational step `h`.
    fn step_free(
        &self,
        m: &RefereeModel,
        s: &RefereeState,
        tau: &[f64],
        h: Rational,
    ) -> Result<RefereeState, RefereeRefusal>;

    /// Report link count, coordinate layout, actuator names, and total mass read at the pin.
    fn describe(&self, m: &RefereeModel) -> ModelFacts;
}

/// A contact step over a caller-supplied contact set, with its capacity as data.
pub trait RefereeContact {
    /// Maximum contact rows supported by this adapter.
    fn contact_capacity(&self) -> usize;
}

/// Structure-preserving steps on groups and swept motion charts.
pub trait Integrator {
    /// Integrator family identifier.
    fn integrator_name(&self) -> &'static str;
}

/// Composition of components through typed effort and flow ports.
pub trait PortSystem {
    /// Number of declared typed ports on this system.
    fn port_count(&self) -> usize;
}

/// Check whether the `referee` group is compiled into this binary.
pub fn available() -> Result<(), GroupAbsent> {
    if ENABLED {
        Ok(())
    } else {
        Err(GroupAbsent::new(GROUP))
    }
}

#[cfg(feature = "referee")]
/// Construct the in-process `RefereeDynamics` adapter (`BD-007`).
pub fn new_referee_dynamics() -> Result<InProcRefereeDynamics, GroupAbsent> {
    Ok(InProcRefereeDynamics::new())
}

#[cfg(not(feature = "referee"))]
/// Stub constructor when the `referee` group is not compiled in.
pub fn new_referee_dynamics() -> Result<(), GroupAbsent> {
    Err(GroupAbsent::new(GROUP))
}

#[cfg(not(feature = "referee"))]
/// Stub constructor when the `referee` group is not compiled in.
pub fn new_referee_contact() -> Result<(), GroupAbsent> {
    Err(GroupAbsent::new(GROUP))
}

#[cfg(not(feature = "referee"))]
/// Stub constructor when the `referee` group is not compiled in.
pub fn new_integrator() -> Result<(), GroupAbsent> {
    Err(GroupAbsent::new(GROUP))
}

#[cfg(not(feature = "referee"))]
/// Stub constructor when the `referee` group is not compiled in.
pub fn new_port_system() -> Result<(), GroupAbsent> {
    Err(GroupAbsent::new(GROUP))
}
