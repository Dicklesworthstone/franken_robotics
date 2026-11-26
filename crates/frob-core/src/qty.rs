//! Closed SI quantity re-export surface over `fs-qty` (BD-005, N33c-03, rule L-6).
//!
//! Exports 36 named SI types (`<= 64`, OPEN-SUITE-4) generated from
//! `registries/qty_aliases.toml` and `registries/qty_products.toml` plus
//! [`Checked`], and nothing generic over a dimension parameter.
//!
//! Equal-dimension kinds remain distinct types (`CX-354`, `KA-779`):
//!
//! ```compile_fail,E0308
//! use frob_core::qty::{Pressure, Time};
//! let _ = Pressure::new(1.0) + Time::new(1.0);
//! ```
//! ```compile_fail,E0308
//! use frob_core::qty::{Energy, Torque};
//! let _ = Torque::new(1.0) + Energy::new(1.0);
//! ```
//! ```compile_fail,E0308
//! use frob_core::qty::{AngularStiffness, Torque};
//! let _ = Torque::new(1.0) + AngularStiffness::new(1.0);
//! ```
//! ```compile_fail,E0308
//! use frob_core::qty::{AngularStiffness, Energy};
//! let _ = Energy::new(1.0) + AngularStiffness::new(1.0);
//! ```
//! ```compile_fail,E0308
//! use frob_core::qty::{AngularVelocity, Frequency};
//! let _ = Frequency::new(1.0) + AngularVelocity::new(1.0);
//! ```
//! ```compile_fail,E0308
//! use frob_core::qty::{Impulse, Momentum};
//! let _ = Momentum::new(1.0) + Impulse::new(1.0);
//! ```
//! ```compile_fail,E0308
//! use frob_core::qty::{AngularDamping, AngularMomentum};
//! let _ = AngularMomentum::new(1.0) + AngularDamping::new(1.0);
//! ```
//! ```compile_fail,E0308
//! use frob_core::qty::{Angle, Ratio};
//! let _ = Angle::new(1.0) + Ratio::new(1.0);
//! ```
//! ```compile_fail,E0308
//! use frob_core::qty::{Angle, FrictionCoefficient};
//! let _ = Angle::new(1.0) + FrictionCoefficient::new(1.0);
//! ```
//! ```compile_fail,E0308
//! use frob_core::qty::{FrictionCoefficient, Ratio};
//! let _ = Ratio::new(1.0) + FrictionCoefficient::new(1.0);
//! ```
//!
//! Determinism D0; prints nothing (L-7); zero `unsafe`.

use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

pub const MAX_ALIASES: usize = 64;

/// Refusal code assigned to runtime unit/dimension mismatch (`N33c-03`, `N14-REF`).
pub const DIMENSION_MISMATCH_CODE: &str = "FROB-REF-102";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DimensionMismatch {
    pub observed: [i8; 6],
    pub required: [i8; 6],
    pub target_name: &'static str,
}

impl DimensionMismatch {
    /// Canonical refusal code string (`FROB-REF-102`).
    #[must_use]
    pub const fn refusal_code(&self) -> &'static str {
        DIMENSION_MISMATCH_CODE
    }
}

impl fmt::Display for DimensionMismatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: dimension mismatch for {}: observed {:?}, required {:?}",
            DIMENSION_MISMATCH_CODE, self.target_name, self.observed, self.required
        )
    }
}
impl std::error::Error for DimensionMismatch {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Checked {
    inner: fs_qty::QtyAny,
}

impl Checked {
    #[must_use]
    pub const fn new(value: f64, dimension: [i8; 6]) -> Self {
        Self {
            inner: fs_qty::QtyAny::new(value, fs_qty::Dims(dimension)),
        }
    }

    #[must_use]
    pub const fn value(self) -> f64 {
        self.inner.value
    }

    #[must_use]
    pub const fn dimension(self) -> [i8; 6] {
        self.inner.dims.0
    }

    pub fn into_alias<A: QuantityAlias>(self) -> Result<A, DimensionMismatch> {
        if self.dimension() == A::DIMENSION {
            Ok(A::from_si(self.value()))
        } else {
            Err(DimensionMismatch {
                observed: self.dimension(),
                required: A::DIMENSION,
                target_name: A::NAME,
            })
        }
    }
}

pub trait QuantityAlias: Copy + Sized + private::Sealed {
    const NAME: &'static str;
    const KIND: &'static str;
    const UNIT: &'static str;
    const DIMENSION: [i8; 6];
    fn from_si(value: f64) -> Self;
    fn value(self) -> f64;
    fn to_checked(self) -> Checked {
        Checked::new(self.value(), Self::DIMENSION)
    }
}
mod private {
    pub trait Sealed {}
}

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

macro_rules! define_qty_ops {
    ($($l:ident * $r:ident => $o:ident;)* ; $($dl:ident / $dr:ident => $do:ident;)*) => {
        $(
            const _: () = {
                let mut i = 0;
                while i < 6 {
                    let sum = match $l::DIMENSION[i].checked_add($r::DIMENSION[i]) {
                        Some(v) => v,
                        None => panic!("quantity product dimension exponent overflow"),
                    };
                    assert!(sum == $o::DIMENSION[i], "quantity product dimension mismatch");
                    i += 1;
                }
            };
            impl Mul<$r> for $l { type Output = $o; #[inline] fn mul(self, rhs: $r) -> $o { $o::new(self.value() * rhs.value()) } }
        )*
        $(
            const _: () = {
                let mut i = 0;
                while i < 6 {
                    let diff = match $dl::DIMENSION[i].checked_sub($dr::DIMENSION[i]) {
                        Some(v) => v,
                        None => panic!("quantity quotient dimension exponent overflow"),
                    };
                    assert!(diff == $do::DIMENSION[i], "quantity quotient dimension mismatch");
                    i += 1;
                }
            };
            impl Div<$dr> for $dl { type Output = $do; #[inline] fn div(self, rhs: $dr) -> $do { $do::new(self.value() / rhs.value()) } }
        )*
    };
}

include!("qty_gen.rs");

impl Torque {
    #[inline]
    #[must_use]
    pub fn from_force_arm(force: Force, lever_arm: Length) -> Self {
        Self::new(force.value() * lever_arm.value())
    }
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
        for raw in [
            -0.0_f64,
            0.0,
            f64::MIN_POSITIVE,
            5e-324,
            1.0,
            -42.25,
            f64::MAX,
        ] {
            let l: Length = Checked::new(raw, Length::DIMENSION).into_alias().unwrap();
            assert_eq!(l.to_checked().value().to_bits(), raw.to_bits());
        }
        let err = Checked::new(3.0, Velocity::DIMENSION)
            .into_alias::<Length>()
            .unwrap_err();
        assert_eq!(err.refusal_code(), "FROB-REF-102");
        assert_eq!(
            (err.observed, err.required),
            (Velocity::DIMENSION, Length::DIMENSION)
        );
    }
}
