//! Closed SI quantity re-export surface over `fs-qty` (BD-005, N33c-03, rule L-6).
//!
//! Exports 36 named SI types (`<= 64`, OPEN-SUITE-4) plus [`Checked`], and nothing generic
//! over a dimension parameter. Equal-dimension kinds remain distinct types (CX-354):
//!
//! ```compile_fail,E0308
//! use frob_core::qty::{Energy, Torque};
//! let _ = Torque::new(1.0) + Energy::new(1.0);
//! ```
//!
//! ```compile_fail,E0308
//! use frob_core::qty::{Pressure, Time};
//! let _ = Pressure::new(1.0) + Time::new(1.0);
//! ```
//!
//! Determinism D0; prints nothing (L-7); zero `unsafe`.

use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

pub const MAX_ALIASES: usize = 64;
pub const ALIAS_COUNT: usize = 36;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DimensionMismatch { pub observed: [i8; 6], pub required: [i8; 6], pub target_name: &'static str }

impl fmt::Display for DimensionMismatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "dimension mismatch for {}: observed {:?}, required {:?}", self.target_name, self.observed, self.required)
    }
}
impl std::error::Error for DimensionMismatch {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Checked { inner: fs_qty::QtyAny }

impl Checked {
    #[must_use] pub const fn new(value: f64, dimension: [i8; 6]) -> Self { Self { inner: fs_qty::QtyAny::new(value, fs_qty::Dims(dimension)) } }
    #[must_use] pub const fn value(self) -> f64 { self.inner.value }
    #[must_use] pub const fn dimension(self) -> [i8; 6] { self.inner.dims.0 }
    pub fn into_alias<A: QuantityAlias>(self) -> Result<A, DimensionMismatch> {
        if self.dimension() == A::DIMENSION { Ok(A::from_si(self.value())) }
        else { Err(DimensionMismatch { observed: self.dimension(), required: A::DIMENSION, target_name: A::NAME }) }
    }
}

pub trait QuantityAlias: Copy + Sized + private::Sealed {
    const NAME: &'static str;
    const KIND: &'static str;
    const UNIT: &'static str;
    const DIMENSION: [i8; 6];
    fn from_si(value: f64) -> Self;
    fn value(self) -> f64;
    fn to_checked(self) -> Checked { Checked::new(self.value(), Self::DIMENSION) }
}
mod private { pub trait Sealed {} }

macro_rules! define_qty_aliases {
    ($($name:ident = ($m:literal, $kg:literal, $s:literal, $k:literal, $a:literal, $mol:literal), $kind:literal, $unit:literal;)+) => {
        pub const ALIAS_NAMES: &[&str] = &[$(stringify!($name)),+];
        $(
            #[derive(Clone, Copy, PartialEq, PartialOrd, Default)]
            #[repr(transparent)]
            pub struct $name(fs_qty::Qty<$m, $kg, $s, $k, $a, $mol>);
            impl private::Sealed for $name {}
            impl QuantityAlias for $name {
                const NAME: &'static str = stringify!($name);
                const KIND: &'static str = $kind;
                const UNIT: &'static str = $unit;
                const DIMENSION: [i8; 6] = [$m, $kg, $s, $k, $a, $mol];
                #[inline] fn from_si(value: f64) -> Self { Self::new(value) }
                #[inline] fn value(self) -> f64 { self.0.value() }
            }
            impl $name {
                #[inline] #[must_use] pub const fn new(value: f64) -> Self { Self(fs_qty::Qty::new(value)) }
                #[inline] #[must_use] pub const fn value(self) -> f64 { self.0.value() }
                #[inline] #[must_use] pub fn total_cmp(&self, other: &Self) -> Ordering { self.0.total_cmp(&other.0) }
                #[inline] #[must_use] pub const fn to_checked(self) -> Checked { Checked { inner: self.0.erase() } }
            }
            impl Add for $name { type Output = Self; #[inline] fn add(self, rhs: Self) -> Self { Self(self.0 + rhs.0) } }
            impl Sub for $name { type Output = Self; #[inline] fn sub(self, rhs: Self) -> Self { Self(self.0 - rhs.0) } }
            impl Neg for $name { type Output = Self; #[inline] fn neg(self) -> Self { Self(-self.0) } }
            impl Mul<f64> for $name { type Output = Self; #[inline] fn mul(self, rhs: f64) -> Self { Self(self.0 * rhs) } }
            impl Div<f64> for $name { type Output = Self; #[inline] fn div(self, rhs: f64) -> Self { Self(self.0 / rhs) } }
            impl Div<$name> for $name { type Output = Ratio; #[inline] fn div(self, rhs: Self) -> Ratio { Ratio::new(self.value() / rhs.value()) } }
            impl fmt::Debug for $name { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}({}) {}", stringify!($name), self.value(), $unit) } }
            impl fmt::Display for $name { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{} {}", self.value(), $unit) } }
        )+
    };
}

define_qty_aliases! {
    Length = (1, 0, 0, 0, 0, 0), "length", "m";
    Area = (2, 0, 0, 0, 0, 0), "area", "m^2";
    Volume = (3, 0, 0, 0, 0, 0), "volume", "m^3";
    Angle = (0, 0, 0, 0, 0, 0), "angle", "rad";
    Ratio = (0, 0, 0, 0, 0, 0), "ratio", "1";
    Mass = (0, 1, 0, 0, 0, 0), "mass", "kg";
    Density = (-3, 1, 0, 0, 0, 0), "density", "kg·m^-3";
    Time = (0, 0, 1, 0, 0, 0), "time", "s";
    Frequency = (0, 0, -1, 0, 0, 0), "frequency", "Hz";
    AngularVelocity = (0, 0, -1, 0, 0, 0), "angular-velocity", "rad·s^-1";
    AngularAcceleration = (0, 0, -2, 0, 0, 0), "angular-acceleration", "rad·s^-2";
    Velocity = (1, 0, -1, 0, 0, 0), "velocity", "m·s^-1";
    Acceleration = (1, 0, -2, 0, 0, 0), "acceleration", "m·s^-2";
    Force = (1, 1, -2, 0, 0, 0), "force", "N";
    Torque = (2, 1, -2, 0, 0, 0), "torque", "N·m";
    Energy = (2, 1, -2, 0, 0, 0), "energy", "J";
    Power = (2, 1, -3, 0, 0, 0), "power", "W";
    Momentum = (1, 1, -1, 0, 0, 0), "momentum", "kg·m·s^-1";
    Impulse = (1, 1, -1, 0, 0, 0), "impulse", "N·s";
    AngularMomentum = (2, 1, -1, 0, 0, 0), "angular-momentum", "kg·m^2·s^-1";
    MomentOfInertia = (2, 1, 0, 0, 0, 0), "moment-of-inertia", "kg·m^2";
    Stiffness = (0, 1, -2, 0, 0, 0), "stiffness", "N·m^-1";
    AngularStiffness = (2, 1, -2, 0, 0, 0), "angular-stiffness", "N·m·rad^-1";
    Damping = (0, 1, -1, 0, 0, 0), "damping", "N·s·m^-1";
    AngularDamping = (2, 1, -1, 0, 0, 0), "angular-damping", "N·m·s·rad^-1";
    Pressure = (-1, 1, -2, 0, 0, 0), "pressure", "Pa";
    Temperature = (0, 0, 0, 1, 0, 0), "temperature", "K";
    HeatCapacity = (2, 1, -2, -1, 0, 0), "heat-capacity", "J·K^-1";
    ThermalResistance = (-2, -1, 3, 1, 0, 0), "thermal-resistance", "K·W^-1";
    Current = (0, 0, 0, 0, 1, 0), "current", "A";
    Voltage = (2, 1, -3, 0, -1, 0), "voltage", "V";
    Resistance = (2, 1, -3, 0, -2, 0), "resistance", "Ω";
    Inductance = (2, 1, -2, 0, -2, 0), "inductance", "H";
    Charge = (0, 0, 1, 0, 1, 0), "charge", "C";
    TorqueConstant = (2, 1, -2, 0, -1, 0), "torque-constant", "N·m·A^-1";
    FrictionCoefficient = (0, 0, 0, 0, 0, 0), "friction-coefficient", "1";
}

macro_rules! define_qty_ops {
    ($($l:ident * $r:ident => $o:ident;)* ; $($dl:ident / $dr:ident => $do:ident;)*) => {
        $(impl Mul<$r> for $l { type Output = $o; #[inline] fn mul(self, rhs: $r) -> $o { $o::new(self.value() * rhs.value()) } })*
        $(impl Div<$dr> for $dl { type Output = $do; #[inline] fn div(self, rhs: $dr) -> $do { $do::new(self.value() / rhs.value()) } })*
    };
}

define_qty_ops! {
    Length * Length => Area; Area * Length => Volume; Length * Area => Volume; Density * Volume => Mass;
    Velocity * Time => Length; Acceleration * Time => Velocity; Mass * Acceleration => Force;
    Mass * Velocity => Momentum; Force * Time => Impulse; Force * Length => Energy;
    Force * Velocity => Power; Torque * AngularVelocity => Power; Power * Time => Energy;
    MomentOfInertia * AngularAcceleration => Torque; MomentOfInertia * AngularVelocity => AngularMomentum;
    Stiffness * Length => Force; Damping * Velocity => Force; AngularStiffness * Angle => Torque;
    AngularDamping * AngularVelocity => Torque; Pressure * Area => Force; Voltage * Current => Power;
    Resistance * Current => Voltage; Current * Time => Charge; TorqueConstant * Current => Torque;
    ;
    Length / Time => Velocity; Velocity / Time => Acceleration; Angle / Time => AngularVelocity;
    AngularVelocity / Time => AngularAcceleration; Mass / Volume => Density; Force / Area => Pressure;
    Energy / Time => Power; Voltage / Current => Resistance; Power / Current => Voltage;
    Power / Voltage => Current; Torque / Current => TorqueConstant;
}

impl Torque {
    #[inline] #[must_use] pub fn from_force_arm(force: Force, lever_arm: Length) -> Self { Self::new(force.value() * lever_arm.value()) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alias_count_and_closed_products() {
        assert_eq!(ALIAS_NAMES.len(), ALIAS_COUNT);
        assert!(ALIAS_COUNT <= MAX_ALIASES);
        let p: Pressure = Force::new(10.0) / Area::new(2.0);
        assert!((p.value() - 5.0).abs() < 1e-12);
        let v: Velocity = Length::new(6.0) / Time::new(2.0);
        assert!((v.value() - 3.0).abs() < 1e-12);
        let e: Energy = Force::new(4.0) * Length::new(3.0);
        let tau = Torque::from_force_arm(Force::new(4.0), Length::new(3.0));
        assert_eq!(e.value().to_bits(), tau.value().to_bits());
        assert_eq!(Energy::DIMENSION, Torque::DIMENSION);
        assert_ne!(Energy::KIND, Torque::KIND);
    }

    #[test]
    fn checked_roundtrip_and_mismatch_refusal() {
        for raw in [-0.0_f64, 0.0, f64::MIN_POSITIVE, 5e-324, 1.0, -42.25, f64::MAX] {
            let l: Length = Checked::new(raw, Length::DIMENSION).into_alias().unwrap();
            assert_eq!(l.to_checked().value().to_bits(), raw.to_bits());
        }
        let err = Checked::new(3.0, Velocity::DIMENSION).into_alias::<Length>().unwrap_err();
        assert_eq!((err.observed, err.required), (Velocity::DIMENSION, Length::DIMENSION));
    }
}
