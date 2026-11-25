//! Appendix D.4 exact oracle library: actuators and power (`N12-44`).
//!
//! Independent reference implementations for all 14 rows of Appendix D.4 (§15.4–§15.11,
//! §24.8, §29.9) inside `frob-conformance` (layer 7):
//! - Exact arbitrary-precision rational arithmetic (`Rat`) everywhere the statement is rational.
//! - Explicit SI 6-component physical dimensions (`[m, kg, s, K, A, mol]`, matching `frob_core::qty`)
//!   with checked dimensional homogeneity across every oracle's inputs and outputs.
//! - Tolerance rows declare explicit dimensioned tolerances and independent reference paths.
//!
//! Determinism class `D0` for rational rows; zero `unsafe`; prints nothing (L-7);
//! zero dependency on any production crate.

use std::fmt;

use frob_core::qty::{
    Acceleration, AngularAcceleration, AngularDamping, AngularMomentum, AngularVelocity, Charge,
    Current, Energy, HeatCapacity, Inductance, Length, Mass, MomentOfInertia, Power, QuantityAlias,
    Ratio, Resistance, Temperature, Time, Torque, TorqueConstant, Velocity, Voltage,
};

use crate::compendium::rat::{Rat, ri};

// =========================================================================
// SI physical dimension algebra ([m, kg, s, K, A, mol])
// =========================================================================

/// Six-component SI base dimension exponent vector `[m, kg, s, K, A, mol]`,
/// matching `frob_core::qty::QuantityAlias::DIMENSION`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Dim(pub [i8; 6]);

impl Dim {
    pub const ONE: Self = Self(Ratio::DIMENSION);
    pub const LENGTH: Self = Self(Length::DIMENSION);
    pub const MASS: Self = Self(Mass::DIMENSION);
    pub const TIME: Self = Self(Time::DIMENSION);
    pub const TEMPERATURE: Self = Self(Temperature::DIMENSION);
    pub const CURRENT: Self = Self(Current::DIMENSION);
    pub const VELOCITY: Self = Self(Velocity::DIMENSION);
    pub const ACCELERATION: Self = Self(Acceleration::DIMENSION);
    pub const ANGULAR_VELOCITY: Self = Self(AngularVelocity::DIMENSION);
    pub const ANGULAR_ACCEL: Self = Self(AngularAcceleration::DIMENSION);
    pub const TORQUE: Self = Self(Torque::DIMENSION);
    pub const ENERGY: Self = Self(Energy::DIMENSION);
    pub const POWER: Self = Self(Power::DIMENSION);
    pub const INERTIA: Self = Self(MomentOfInertia::DIMENSION);
    pub const ANGULAR_MOMENTUM: Self = Self(AngularMomentum::DIMENSION);
    pub const ANGULAR_DAMPING: Self = Self(AngularDamping::DIMENSION);
    pub const VOLTAGE: Self = Self(Voltage::DIMENSION);
    pub const RESISTANCE: Self = Self(Resistance::DIMENSION);
    pub const INDUCTANCE: Self = Self(Inductance::DIMENSION);
    pub const CAPACITANCE: Self = Self([-2, -1, 4, 0, 2, 0]);
    pub const CHARGE: Self = Self(Charge::DIMENSION);
    pub const FLUX_LINKAGE: Self = Self([2, 1, -2, 0, -1, 0]);
    pub const TORQUE_CONST: Self = Self(TorqueConstant::DIMENSION);
    pub const HEAT_CAPACITY: Self = Self(HeatCapacity::DIMENSION);
    pub const THERMAL_CONDUCTANCE: Self = Self([2, 1, -3, -1, 0, 0]);
    pub const TEMP_COEFF: Self = Self([0, 0, 0, -1, 0, 0]);
    pub const TEMP_RATE: Self = Self([0, 0, -1, 1, 0, 0]);
    pub const ENTROPY_RATE: Self = Self([2, 1, -3, -1, 0, 0]);
    pub const VOLTAGE_RATE: Self = Self([2, 1, -4, 0, -1, 0]);
    pub const CURRENT_RATE: Self = Self([0, 0, -1, 0, 1, 0]);
    pub const FREQUENCY: Self = Self([0, 0, -1, 0, 0, 0]);

    /// Multiply two dimensions (add exponents).
    #[must_use]
    pub const fn mul(self, rhs: Self) -> Self {
        Self([
            self.0[0] + rhs.0[0],
            self.0[1] + rhs.0[1],
            self.0[2] + rhs.0[2],
            self.0[3] + rhs.0[3],
            self.0[4] + rhs.0[4],
            self.0[5] + rhs.0[5],
        ])
    }

    /// Divide two dimensions (subtract exponents).
    #[must_use]
    pub const fn div(self, rhs: Self) -> Self {
        Self([
            self.0[0] - rhs.0[0],
            self.0[1] - rhs.0[1],
            self.0[2] - rhs.0[2],
            self.0[3] - rhs.0[3],
            self.0[4] - rhs.0[4],
            self.0[5] - rhs.0[5],
        ])
    }

    /// Raise a dimension to an integer power.
    #[must_use]
    pub const fn powi(self, n: i8) -> Self {
        Self([
            self.0[0] * n,
            self.0[1] * n,
            self.0[2] * n,
            self.0[3] * n,
            self.0[4] * n,
            self.0[5] * n,
        ])
    }

    /// Checked addition/subtraction homogeneity: requires `self == rhs`.
    pub const fn try_add(self, rhs: Self) -> Result<Self, DimError> {
        if self.0[0] == rhs.0[0]
            && self.0[1] == rhs.0[1]
            && self.0[2] == rhs.0[2]
            && self.0[3] == rhs.0[3]
            && self.0[4] == rhs.0[4]
            && self.0[5] == rhs.0[5]
        {
            Ok(self)
        } else {
            Err(DimError::InhomogeneousSum { lhs: self, rhs })
        }
    }

    /// Checked square root: every exponent must be even.
    pub const fn try_sqrt(self) -> Result<Self, DimError> {
        if (self.0[0] & 1) != 0
            || (self.0[1] & 1) != 0
            || (self.0[2] & 1) != 0
            || (self.0[3] & 1) != 0
            || (self.0[4] & 1) != 0
            || (self.0[5] & 1) != 0
        {
            Err(DimError::OddSqrtExponent { dim: self })
        } else {
            Ok(Self([
                self.0[0] / 2,
                self.0[1] / 2,
                self.0[2] / 2,
                self.0[3] / 2,
                self.0[4] / 2,
                self.0[5] / 2,
            ]))
        }
    }

    /// Checked assertion that a quantity is dimensionless (`Dim::ONE`).
    pub const fn require_dimensionless(self) -> Result<(), DimError> {
        match self.try_add(Self::ONE) {
            Ok(_) => Ok(()),
            Err(_) => Err(DimError::NotDimensionless { observed: self }),
        }
    }
}

/// Dimensional homogeneity error for D.4 expressions and oracle checks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DimError {
    InhomogeneousSum {
        lhs: Dim,
        rhs: Dim,
    },
    OddSqrtExponent {
        dim: Dim,
    },
    NotDimensionless {
        observed: Dim,
    },
    OutputMismatch {
        oracle: &'static str,
        expected: Dim,
        derived: Dim,
    },
}

impl fmt::Display for DimError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InhomogeneousSum { lhs, rhs } => {
                write!(
                    f,
                    "inhomogeneous sum of dimensions {:?} and {:?}",
                    lhs.0, rhs.0
                )
            }
            Self::OddSqrtExponent { dim } => {
                write!(
                    f,
                    "square root of odd dimension exponent vector {:?}",
                    dim.0
                )
            }
            Self::NotDimensionless { observed } => {
                write!(
                    f,
                    "expected dimensionless quantity, observed {:?}",
                    observed.0
                )
            }
            Self::OutputMismatch {
                oracle,
                expected,
                derived,
            } => {
                write!(
                    f,
                    "oracle `{oracle}` output dimension mismatch: expected {:?}, derived {:?}",
                    expected.0, derived.0
                )
            }
        }
    }
}

impl std::error::Error for DimError {}

/// A scalar quantity carrying its exact SI dimension vector for checked homogeneity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DimQty<T> {
    pub value: T,
    pub dim: Dim,
}

impl<T> DimQty<T> {
    #[must_use]
    pub const fn new(value: T, dim: Dim) -> Self {
        Self { value, dim }
    }
}

impl DimQty<Rat> {
    pub fn try_add(&self, rhs: &Self) -> Result<Self, DimError> {
        let dim = self.dim.try_add(rhs.dim)?;
        Ok(Self::new(&self.value + &rhs.value, dim))
    }

    pub fn try_sub(&self, rhs: &Self) -> Result<Self, DimError> {
        let dim = self.dim.try_add(rhs.dim)?;
        Ok(Self::new(&self.value - &rhs.value, dim))
    }

    #[must_use]
    pub fn mul(&self, rhs: &Self) -> Self {
        Self::new(&self.value * &rhs.value, self.dim.mul(rhs.dim))
    }

    #[must_use]
    pub fn div(&self, rhs: &Self) -> Self {
        Self::new(&self.value / &rhs.value, self.dim.div(rhs.dim))
    }
}

impl DimQty<f64> {
    pub fn try_add(self, rhs: Self) -> Result<Self, DimError> {
        let dim = self.dim.try_add(rhs.dim)?;
        Ok(Self::new(self.value + rhs.value, dim))
    }

    pub fn try_sub(self, rhs: Self) -> Result<Self, DimError> {
        let dim = self.dim.try_add(rhs.dim)?;
        Ok(Self::new(self.value - rhs.value, dim))
    }

    #[must_use]
    pub fn mul(self, rhs: Self) -> Self {
        Self::new(self.value * rhs.value, self.dim.mul(rhs.dim))
    }

    #[must_use]
    pub fn div(self, rhs: Self) -> Self {
        Self::new(self.value / rhs.value, self.dim.div(rhs.dim))
    }

    pub fn try_sqrt(self) -> Result<Self, DimError> {
        let dim = self.dim.try_sqrt()?;
        Ok(Self::new(self.value.sqrt(), dim))
    }
}

/// Named port / parameter dimension descriptor for one D.4 oracle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DimSlot {
    pub name: &'static str,
    pub unit: &'static str,
    pub dim: Dim,
}

/// Complete input/output SI dimensional signature for one D.4 row's oracle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OracleDimSignature {
    pub row: &'static str,
    pub oracle: &'static str,
    pub inputs: &'static [DimSlot],
    pub outputs: &'static [DimSlot],
}

/// Dimensional signatures of all 14 Appendix D.4 oracles.
pub const D4_ORACLE_SIGNATURES: &[OracleDimSignature] = &[
    OracleDimSignature {
        row: "Port-Hamiltonian form",
        oracle: "phs_step_derivatives",
        inputs: &[
            DimSlot {
                name: "H",
                unit: "J",
                dim: Dim::ENERGY,
            },
            DimSlot {
                name: "u*y",
                unit: "W",
                dim: Dim::POWER,
            },
            DimSlot {
                name: "grad_H^T*R*grad_H",
                unit: "W",
                dim: Dim::POWER,
            },
        ],
        outputs: &[
            DimSlot {
                name: "H_dot",
                unit: "W",
                dim: Dim::POWER,
            },
            DimSlot {
                name: "supply",
                unit: "W",
                dim: Dim::POWER,
            },
            DimSlot {
                name: "dissipation",
                unit: "W",
                dim: Dim::POWER,
            },
        ],
    },
    OracleDimSignature {
        row: "Motor axis",
        oracle: "motor_axis_locked_rotor_current",
        inputs: &[
            DimSlot {
                name: "V",
                unit: "V",
                dim: Dim::VOLTAGE,
            },
            DimSlot {
                name: "R_w",
                unit: "Ω",
                dim: Dim::RESISTANCE,
            },
            DimSlot {
                name: "L",
                unit: "H",
                dim: Dim::INDUCTANCE,
            },
            DimSlot {
                name: "k",
                unit: "N·m·A^-1",
                dim: Dim::TORQUE_CONST,
            },
            DimSlot {
                name: "J_r",
                unit: "kg·m^2",
                dim: Dim::INERTIA,
            },
            DimSlot {
                name: "b",
                unit: "N·m·s·rad^-1",
                dim: Dim::ANGULAR_DAMPING,
            },
            DimSlot {
                name: "tau_load",
                unit: "N·m",
                dim: Dim::TORQUE,
            },
            DimSlot {
                name: "t",
                unit: "s",
                dim: Dim::TIME,
            },
        ],
        outputs: &[
            DimSlot {
                name: "tau_e",
                unit: "s",
                dim: Dim::TIME,
            },
            DimSlot {
                name: "i_locked",
                unit: "A",
                dim: Dim::CURRENT,
            },
            DimSlot {
                name: "di_dt",
                unit: "A·s^-1",
                dim: Dim::CURRENT_RATE,
            },
            DimSlot {
                name: "domega_dt",
                unit: "rad·s^-2",
                dim: Dim::ANGULAR_ACCEL,
            },
        ],
    },
    OracleDimSignature {
        row: "Motor envelope",
        oracle: "motor_envelope_l1",
        inputs: &[
            DimSlot {
                name: "I_max",
                unit: "A",
                dim: Dim::CURRENT,
            },
            DimSlot {
                name: "V_bus",
                unit: "V",
                dim: Dim::VOLTAGE,
            },
            DimSlot {
                name: "k_e",
                unit: "V·s·rad^-1",
                dim: Dim::TORQUE_CONST,
            },
            DimSlot {
                name: "k_t",
                unit: "N·m·A^-1",
                dim: Dim::TORQUE_CONST,
            },
            DimSlot {
                name: "N",
                unit: "1",
                dim: Dim::ONE,
            },
            DimSlot {
                name: "R",
                unit: "Ω",
                dim: Dim::RESISTANCE,
            },
            DimSlot {
                name: "eta",
                unit: "1",
                dim: Dim::ONE,
            },
            DimSlot {
                name: "tau_friction",
                unit: "N·m",
                dim: Dim::TORQUE,
            },
            DimSlot {
                name: "omega",
                unit: "rad·s^-1",
                dim: Dim::ANGULAR_VELOCITY,
            },
        ],
        outputs: &[
            DimSlot {
                name: "i_limit",
                unit: "A",
                dim: Dim::CURRENT,
            },
            DimSlot {
                name: "tau_out",
                unit: "N·m",
                dim: Dim::TORQUE,
            },
            DimSlot {
                name: "omega_no_load",
                unit: "rad·s^-1",
                dim: Dim::ANGULAR_VELOCITY,
            },
        ],
    },
    OracleDimSignature {
        row: "Voltage limit and field weakening",
        oracle: "voltage_limit_field_weakening",
        inputs: &[
            DimSlot {
                name: "V_bus",
                unit: "V",
                dim: Dim::VOLTAGE,
            },
            DimSlot {
                name: "lambda",
                unit: "V·s",
                dim: Dim::FLUX_LINKAGE,
            },
            DimSlot {
                name: "L_d",
                unit: "H",
                dim: Dim::INDUCTANCE,
            },
            DimSlot {
                name: "L_q",
                unit: "H",
                dim: Dim::INDUCTANCE,
            },
            DimSlot {
                name: "I_max",
                unit: "A",
                dim: Dim::CURRENT,
            },
            DimSlot {
                name: "omega_e",
                unit: "rad·s^-1",
                dim: Dim::ANGULAR_VELOCITY,
            },
        ],
        outputs: &[
            DimSlot {
                name: "V_lim",
                unit: "V",
                dim: Dim::VOLTAGE,
            },
            DimSlot {
                name: "omega_base",
                unit: "rad·s^-1",
                dim: Dim::ANGULAR_VELOCITY,
            },
            DimSlot {
                name: "omega_top",
                unit: "rad·s^-1",
                dim: Dim::ANGULAR_VELOCITY,
            },
        ],
    },
    OracleDimSignature {
        row: "Thermal node",
        oracle: "thermal_node_closed_form",
        inputs: &[
            DimSlot {
                name: "C",
                unit: "J·K^-1",
                dim: Dim::HEAT_CAPACITY,
            },
            DimSlot {
                name: "G_th",
                unit: "W·K^-1",
                dim: Dim::THERMAL_CONDUCTANCE,
            },
            DimSlot {
                name: "R_0",
                unit: "Ω",
                dim: Dim::RESISTANCE,
            },
            DimSlot {
                name: "alpha",
                unit: "K^-1",
                dim: Dim::TEMP_COEFF,
            },
            DimSlot {
                name: "T_0",
                unit: "K",
                dim: Dim::TEMPERATURE,
            },
            DimSlot {
                name: "T_amb",
                unit: "K",
                dim: Dim::TEMPERATURE,
            },
            DimSlot {
                name: "i",
                unit: "A",
                dim: Dim::CURRENT,
            },
            DimSlot {
                name: "t",
                unit: "s",
                dim: Dim::TIME,
            },
        ],
        outputs: &[
            DimSlot {
                name: "T_w",
                unit: "K",
                dim: Dim::TEMPERATURE,
            },
            DimSlot {
                name: "dT_dt",
                unit: "K·s^-1",
                dim: Dim::TEMP_RATE,
            },
            DimSlot {
                name: "a",
                unit: "s^-1",
                dim: Dim::FREQUENCY,
            },
        ],
    },
    OracleDimSignature {
        row: "Thermal runaway condition",
        oracle: "thermal_runaway_analysis",
        inputs: &[
            DimSlot {
                name: "R_0",
                unit: "Ω",
                dim: Dim::RESISTANCE,
            },
            DimSlot {
                name: "alpha",
                unit: "K^-1",
                dim: Dim::TEMP_COEFF,
            },
            DimSlot {
                name: "G_th",
                unit: "W·K^-1",
                dim: Dim::THERMAL_CONDUCTANCE,
            },
            DimSlot {
                name: "C",
                unit: "J·K^-1",
                dim: Dim::HEAT_CAPACITY,
            },
            DimSlot {
                name: "T_0",
                unit: "K",
                dim: Dim::TEMPERATURE,
            },
            DimSlot {
                name: "T_amb",
                unit: "K",
                dim: Dim::TEMPERATURE,
            },
            DimSlot {
                name: "i",
                unit: "A",
                dim: Dim::CURRENT,
            },
        ],
        outputs: &[
            DimSlot {
                name: "s",
                unit: "W·K^-1",
                dim: Dim::THERMAL_CONDUCTANCE,
            },
            DimSlot {
                name: "I_crit",
                unit: "A",
                dim: Dim::CURRENT,
            },
            DimSlot {
                name: "T_eq",
                unit: "K",
                dim: Dim::TEMPERATURE,
            },
        ],
    },
    OracleDimSignature {
        row: "Entropy production",
        oracle: "entropy_production",
        inputs: &[
            DimSlot {
                name: "P_loss",
                unit: "W",
                dim: Dim::POWER,
            },
            DimSlot {
                name: "G_th",
                unit: "W·K^-1",
                dim: Dim::THERMAL_CONDUCTANCE,
            },
            DimSlot {
                name: "T_w",
                unit: "K",
                dim: Dim::TEMPERATURE,
            },
            DimSlot {
                name: "T_amb",
                unit: "K",
                dim: Dim::TEMPERATURE,
            },
        ],
        outputs: &[DimSlot {
            name: "sigma",
            unit: "W·K^-1",
            dim: Dim::ENTROPY_RATE,
        }],
    },
    OracleDimSignature {
        row: "Two-RC battery",
        oracle: "two_rc_battery_step_response",
        inputs: &[
            DimSlot {
                name: "OCV",
                unit: "V",
                dim: Dim::VOLTAGE,
            },
            DimSlot {
                name: "R_0",
                unit: "Ω",
                dim: Dim::RESISTANCE,
            },
            DimSlot {
                name: "R_1",
                unit: "Ω",
                dim: Dim::RESISTANCE,
            },
            DimSlot {
                name: "C_1",
                unit: "F",
                dim: Dim::CAPACITANCE,
            },
            DimSlot {
                name: "R_2",
                unit: "Ω",
                dim: Dim::RESISTANCE,
            },
            DimSlot {
                name: "C_2",
                unit: "F",
                dim: Dim::CAPACITANCE,
            },
            DimSlot {
                name: "Q",
                unit: "C",
                dim: Dim::CHARGE,
            },
            DimSlot {
                name: "I",
                unit: "A",
                dim: Dim::CURRENT,
            },
            DimSlot {
                name: "t",
                unit: "s",
                dim: Dim::TIME,
            },
        ],
        outputs: &[
            DimSlot {
                name: "V",
                unit: "V",
                dim: Dim::VOLTAGE,
            },
            DimSlot {
                name: "V_1",
                unit: "V",
                dim: Dim::VOLTAGE,
            },
            DimSlot {
                name: "V_2",
                unit: "V",
                dim: Dim::VOLTAGE,
            },
            DimSlot {
                name: "z",
                unit: "1",
                dim: Dim::ONE,
            },
        ],
    },
    OracleDimSignature {
        row: "Bus power ceiling",
        oracle: "bus_operating_point",
        inputs: &[
            DimSlot {
                name: "E",
                unit: "V",
                dim: Dim::VOLTAGE,
            },
            DimSlot {
                name: "R",
                unit: "Ω",
                dim: Dim::RESISTANCE,
            },
            DimSlot {
                name: "P",
                unit: "W",
                dim: Dim::POWER,
            },
        ],
        outputs: &[
            DimSlot {
                name: "P_max",
                unit: "W",
                dim: Dim::POWER,
            },
            DimSlot {
                name: "V",
                unit: "V",
                dim: Dim::VOLTAGE,
            },
            DimSlot {
                name: "I",
                unit: "A",
                dim: Dim::CURRENT,
            },
        ],
    },
    OracleDimSignature {
        row: "Four-quadrant transmission",
        oracle: "screw_efficiencies",
        inputs: &[
            DimSlot {
                name: "a",
                unit: "rad",
                dim: Dim::ONE,
            },
            DimSlot {
                name: "phi",
                unit: "rad",
                dim: Dim::ONE,
            },
            DimSlot {
                name: "N",
                unit: "1",
                dim: Dim::ONE,
            },
            DimSlot {
                name: "tau",
                unit: "N·m",
                dim: Dim::TORQUE,
            },
        ],
        outputs: &[
            DimSlot {
                name: "eta_f",
                unit: "1",
                dim: Dim::ONE,
            },
            DimSlot {
                name: "eta_b",
                unit: "1",
                dim: Dim::ONE,
            },
            DimSlot {
                name: "tau_out",
                unit: "N·m",
                dim: Dim::TORQUE,
            },
        ],
    },
    OracleDimSignature {
        row: "Rational gear law",
        oracle: "rational_gear_map",
        inputs: &[
            DimSlot {
                name: "r",
                unit: "1",
                dim: Dim::ONE,
            },
            DimSlot {
                name: "f_1",
                unit: "rad·s^-1",
                dim: Dim::ANGULAR_VELOCITY,
            },
            DimSlot {
                name: "e_2",
                unit: "N·m",
                dim: Dim::TORQUE,
            },
        ],
        outputs: &[
            DimSlot {
                name: "f_2",
                unit: "rad·s^-1",
                dim: Dim::ANGULAR_VELOCITY,
            },
            DimSlot {
                name: "e_1",
                unit: "N·m",
                dim: Dim::TORQUE,
            },
            DimSlot {
                name: "net_power",
                unit: "W",
                dim: Dim::POWER,
            },
        ],
    },
    OracleDimSignature {
        row: "Energy balance",
        oracle: "energy_balance_residual",
        inputs: &[
            DimSlot {
                name: "supplied",
                unit: "J",
                dim: Dim::ENERGY,
            },
            DimSlot {
                name: "delta_stored",
                unit: "J",
                dim: Dim::ENERGY,
            },
            DimSlot {
                name: "dissipated",
                unit: "J",
                dim: Dim::ENERGY,
            },
            DimSlot {
                name: "exported",
                unit: "J",
                dim: Dim::ENERGY,
            },
        ],
        outputs: &[DimSlot {
            name: "residual",
            unit: "J",
            dim: Dim::ENERGY,
        }],
    },
    OracleDimSignature {
        row: "Cost of transport",
        oracle: "cost_of_transport",
        inputs: &[
            DimSlot {
                name: "E",
                unit: "J",
                dim: Dim::ENERGY,
            },
            DimSlot {
                name: "m",
                unit: "kg",
                dim: Dim::MASS,
            },
            DimSlot {
                name: "g",
                unit: "m·s^-2",
                dim: Dim::ACCELERATION,
            },
            DimSlot {
                name: "d",
                unit: "m",
                dim: Dim::LENGTH,
            },
        ],
        outputs: &[DimSlot {
            name: "CoT",
            unit: "1",
            dim: Dim::ONE,
        }],
    },
    OracleDimSignature {
        row: "Froude number",
        oracle: "froude_number",
        inputs: &[
            DimSlot {
                name: "v",
                unit: "m·s^-1",
                dim: Dim::VELOCITY,
            },
            DimSlot {
                name: "g",
                unit: "m·s^-2",
                dim: Dim::ACCELERATION,
            },
            DimSlot {
                name: "L",
                unit: "m",
                dim: Dim::LENGTH,
            },
        ],
        outputs: &[DimSlot {
            name: "Fr",
            unit: "1",
            dim: Dim::ONE,
        }],
    },
];

fn check_out_dim(oracle: &'static str, expected: Dim, derived: Dim) -> Result<(), DimError> {
    if expected == derived {
        Ok(())
    } else {
        Err(DimError::OutputMismatch {
            oracle,
            expected,
            derived,
        })
    }
}

/// Symbolically verify dimensional homogeneity of all 14 Appendix D.4 formulas
/// from SI base dimensions (`[m, kg, s, K, A, mol]`).
pub fn verify_all_oracle_dimensions() -> Result<(), DimError> {
    // 1. Port-Hamiltonian form: H_dot = u^T y - grad_H^T R grad_H
    let phs_hdot = Dim::POWER.try_add(Dim::POWER)?;
    check_out_dim("phs_step_derivatives", Dim::POWER, phs_hdot)?;

    // 2. Motor axis: L*di/dt = V - R_w*i - k*omega; J_r*domega/dt = k*i - b*omega - tau_load; tau = L/R_w
    let v_rw_i = Dim::RESISTANCE.mul(Dim::CURRENT);
    let v_bemf = Dim::TORQUE_CONST.mul(Dim::ANGULAR_VELOCITY);
    let v_rhs = Dim::VOLTAGE.try_add(v_rw_i)?.try_add(v_bemf)?;
    let di_dt = v_rhs.div(Dim::INDUCTANCE);
    check_out_dim("motor_axis_locked_rotor_current", Dim::CURRENT_RATE, di_dt)?;

    let tau_em = Dim::TORQUE_CONST.mul(Dim::CURRENT);
    let tau_visc = Dim::ANGULAR_DAMPING.mul(Dim::ANGULAR_VELOCITY);
    let tau_rhs = tau_em.try_add(tau_visc)?.try_add(Dim::TORQUE)?;
    let domega_dt = tau_rhs.div(Dim::INERTIA);
    check_out_dim(
        "motor_axis_locked_rotor_current",
        Dim::ANGULAR_ACCEL,
        domega_dt,
    )?;

    let tau_e = Dim::INDUCTANCE.div(Dim::RESISTANCE);
    check_out_dim("motor_axis_locked_rotor_current", Dim::TIME, tau_e)?;
    Dim::TIME.div(tau_e).require_dimensionless()?;
    let i_locked = Dim::VOLTAGE.div(Dim::RESISTANCE);
    check_out_dim("motor_axis_locked_rotor_current", Dim::CURRENT, i_locked)?;

    // 3. Motor envelope: i <= min(I_max, (V_bus - k_e*N*omega)/R); tau = eta*N*k_t*i - tau_friction
    let env_v = Dim::VOLTAGE.try_add(Dim::TORQUE_CONST.mul(Dim::ONE).mul(Dim::ANGULAR_VELOCITY))?;
    let env_i = Dim::CURRENT.try_add(env_v.div(Dim::RESISTANCE))?;
    check_out_dim("motor_envelope_l1", Dim::CURRENT, env_i)?;
    let env_tau = Dim::ONE
        .mul(Dim::ONE)
        .mul(Dim::TORQUE_CONST)
        .mul(env_i)
        .try_add(Dim::TORQUE)?;
    check_out_dim("motor_envelope_l1", Dim::TORQUE, env_tau)?;
    let omega_nl = Dim::VOLTAGE.div(Dim::TORQUE_CONST.mul(Dim::ONE));
    check_out_dim("motor_envelope_l1", Dim::ANGULAR_VELOCITY, omega_nl)?;

    // 4. Voltage limit and field weakening: (lambda + L_d*i_d)^2 + (L_q*i_q)^2 <= (V_lim/omega_e)^2
    let flux_d = Dim::FLUX_LINKAGE.try_add(Dim::INDUCTANCE.mul(Dim::CURRENT))?;
    let flux_q = Dim::INDUCTANCE.mul(Dim::CURRENT);
    let flux_sq = flux_d.powi(2).try_add(flux_q.powi(2))?;
    let v_over_w_sq = Dim::VOLTAGE.div(Dim::ANGULAR_VELOCITY).powi(2);
    flux_sq.try_add(v_over_w_sq)?;
    let omega_base = Dim::VOLTAGE.div(flux_sq.try_sqrt()?);
    check_out_dim(
        "voltage_limit_field_weakening",
        Dim::ANGULAR_VELOCITY,
        omega_base,
    )?;
    let omega_top = Dim::VOLTAGE.div(flux_d);
    check_out_dim(
        "voltage_limit_field_weakening",
        Dim::ANGULAR_VELOCITY,
        omega_top,
    )?;

    // 5. Thermal node: C*dT/dt = i^2*R_0*(1 + alpha*(T_w - T_0)) - G_th*(T_w - T_amb)
    let dt_ref = Dim::TEMPERATURE.try_add(Dim::TEMPERATURE)?;
    Dim::TEMP_COEFF.mul(dt_ref).require_dimensionless()?;
    let p_cu = Dim::CURRENT.powi(2).mul(Dim::RESISTANCE);
    let p_amb = Dim::THERMAL_CONDUCTANCE.mul(dt_ref);
    let p_net = p_cu.try_add(p_amb)?;
    let dtemp_dt = p_net.div(Dim::HEAT_CAPACITY);
    check_out_dim("thermal_node_closed_form", Dim::TEMP_RATE, dtemp_dt)?;
    let a_rate = Dim::CURRENT
        .powi(2)
        .mul(Dim::RESISTANCE)
        .mul(Dim::TEMP_COEFF)
        .try_add(Dim::THERMAL_CONDUCTANCE)?
        .div(Dim::HEAT_CAPACITY);
    check_out_dim("thermal_node_closed_form", Dim::FREQUENCY, a_rate)?;
    a_rate.mul(Dim::TIME).require_dimensionless()?;

    // 6. Thermal runaway: s = i^2*R_0*alpha; I_crit = sqrt(G_th/(R_0*alpha)); T_eq = T_amb + P_amb/(G_th - s)
    let s_dim = Dim::CURRENT
        .powi(2)
        .mul(Dim::RESISTANCE)
        .mul(Dim::TEMP_COEFF);
    let g_minus_s = Dim::THERMAL_CONDUCTANCE.try_add(s_dim)?;
    check_out_dim(
        "thermal_runaway_analysis",
        Dim::THERMAL_CONDUCTANCE,
        g_minus_s,
    )?;
    let i_crit = Dim::THERMAL_CONDUCTANCE
        .div(Dim::RESISTANCE.mul(Dim::TEMP_COEFF))
        .try_sqrt()?;
    check_out_dim("thermal_runaway_analysis", Dim::CURRENT, i_crit)?;
    let t_eq = Dim::TEMPERATURE.try_add(p_cu.div(g_minus_s))?;
    check_out_dim("thermal_runaway_analysis", Dim::TEMPERATURE, t_eq)?;

    // 7. Entropy production: sigma = P_loss/T_w + G_th*(T_w - T_amb)^2/(T_w*T_amb)
    let term1 = Dim::POWER.div(Dim::TEMPERATURE);
    let term2 = Dim::THERMAL_CONDUCTANCE
        .mul(dt_ref.powi(2))
        .div(Dim::TEMPERATURE.mul(Dim::TEMPERATURE));
    let sigma = term1.try_add(term2)?;
    check_out_dim("entropy_production", Dim::ENTROPY_RATE, sigma)?;

    // 8. Two-RC battery: V = OCV - R_0*I - V_1 - V_2; dV_k/dt = -V_k/(R_k*C_k) + I/C_k; dz/dt = -I/Q
    let v_term = Dim::VOLTAGE
        .try_add(Dim::RESISTANCE.mul(Dim::CURRENT))?
        .try_add(Dim::VOLTAGE)?
        .try_add(Dim::VOLTAGE)?;
    check_out_dim("two_rc_battery_step_response", Dim::VOLTAGE, v_term)?;
    let rc_tau = Dim::RESISTANCE.mul(Dim::CAPACITANCE);
    check_out_dim("two_rc_battery_step_response", Dim::TIME, rc_tau)?;
    let dv_dt = Dim::VOLTAGE
        .div(rc_tau)
        .try_add(Dim::CURRENT.div(Dim::CAPACITANCE))?;
    check_out_dim("two_rc_battery_step_response", Dim::VOLTAGE_RATE, dv_dt)?;
    let dz = Dim::CURRENT.mul(Dim::TIME).div(Dim::CHARGE);
    dz.require_dimensionless()?;

    // 9. Bus power ceiling: P_max = E^2/(4*R); V = (E + sqrt(E^2 - 4*R*P))/2
    let p_max = Dim::VOLTAGE.powi(2).div(Dim::RESISTANCE);
    check_out_dim("bus_operating_point", Dim::POWER, p_max)?;
    let disc = Dim::VOLTAGE
        .powi(2)
        .try_add(Dim::RESISTANCE.mul(Dim::POWER))?;
    let v_bus = Dim::VOLTAGE.try_add(disc.try_sqrt()?)?;
    check_out_dim("bus_operating_point", Dim::VOLTAGE, v_bus)?;
    let i_bus = Dim::POWER.div(v_bus);
    check_out_dim("bus_operating_point", Dim::CURRENT, i_bus)?;

    // 10. Four-quadrant transmission: tau_joint = eta_f*N*tau_motor; tau_motor = eta_b*tau_joint/N
    let tau_j = Dim::ONE.mul(Dim::ONE).mul(Dim::TORQUE);
    let tau_m = Dim::ONE.mul(Dim::TORQUE).div(Dim::ONE);
    check_out_dim("screw_efficiencies", Dim::TORQUE, tau_j.try_add(tau_m)?)?;

    // 11. Rational gear law: f_2 = r*f_1; e_1 = -r*e_2; e_1*f_1 + e_2*f_2 = 0
    let f2 = Dim::ONE.mul(Dim::ANGULAR_VELOCITY);
    let e1 = Dim::ONE.mul(Dim::TORQUE);
    let p_gear = e1.mul(Dim::ANGULAR_VELOCITY).try_add(Dim::TORQUE.mul(f2))?;
    check_out_dim("rational_gear_map", Dim::POWER, p_gear)?;

    // 12. Energy balance: supplied - (delta_stored + dissipated + exported)
    let e_bal = Dim::ENERGY
        .try_add(Dim::ENERGY)?
        .try_add(Dim::ENERGY)?
        .try_add(Dim::ENERGY)?;
    check_out_dim("energy_balance_residual", Dim::ENERGY, e_bal)?;

    // 13. Cost of transport: E / (m * g * d)
    let cot = Dim::ENERGY.div(Dim::MASS.mul(Dim::ACCELERATION).mul(Dim::LENGTH));
    cot.require_dimensionless()?;
    check_out_dim("cost_of_transport", Dim::ONE, cot)?;

    // 14. Froude number: v^2 / (g * L)
    let fr = Dim::VELOCITY
        .powi(2)
        .div(Dim::ACCELERATION.mul(Dim::LENGTH));
    fr.require_dimensionless()?;
    check_out_dim("froude_number", Dim::ONE, fr)?;

    Ok(())
}

// =========================================================================
// Exact small dynamic matrix / vector helpers over Q (for PHS)
// =========================================================================

fn dot_n(a: &[Rat], b: &[Rat]) -> Rat {
    assert_eq!(a.len(), b.len(), "vector length mismatch");
    let mut acc = Rat::zero();
    for (x, y) in a.iter().zip(b.iter()) {
        acc = &acc + &(x * y);
    }
    acc
}

fn mat_vec_n(m: &[Vec<Rat>], v: &[Rat]) -> Vec<Rat> {
    m.iter().map(|row| dot_n(row, v)).collect()
}

fn mat_t_vec_n(m: &[Vec<Rat>], v: &[Rat]) -> Vec<Rat> {
    if m.is_empty() {
        return Vec::new();
    }
    let cols = m[0].len();
    assert_eq!(m.len(), v.len(), "transpose matrix-vector mismatch");
    (0..cols)
        .map(|j| {
            let mut acc = Rat::zero();
            for (i, row) in m.iter().enumerate() {
                acc = &acc + &(&row[j] * &v[i]);
            }
            acc
        })
        .collect()
}

/// Check whether an `n × n` rational matrix is skew-symmetric (`J + Jᵀ == 0`).
#[must_use]
pub fn is_skew_symmetric(j: &[Vec<Rat>]) -> bool {
    let n = j.len();
    for (r_idx, row) in j.iter().enumerate() {
        if row.len() != n {
            return false;
        }
        for (c_idx, val) in row.iter().enumerate() {
            if !(val + &j[c_idx][r_idx]).is_zero() {
                return false;
            }
        }
    }
    true
}

/// Check whether an `n × n` rational matrix is symmetric positive semidefinite
/// via exact pivoted congruent elimination (Sylvester/Bunch–Kaufman for diagonals).
#[must_use]
pub fn is_symmetric_psd(r_mat: &[Vec<Rat>]) -> bool {
    let n = r_mat.len();
    for (i, row) in r_mat.iter().enumerate() {
        if row.len() != n {
            return false;
        }
        for (j, val) in row.iter().enumerate() {
            if val != &r_mat[j][i] {
                return false;
            }
        }
    }
    let mut a: Vec<Vec<Rat>> = r_mat.to_vec();
    for k in 0..n {
        // Find a strictly positive diagonal entry in k..n if one exists;
        // if any diagonal is negative, not PSD; if all remaining diagonals are zero,
        // all remaining off-diagonals must also be zero.
        let mut piv_opt = None;
        for i in k..n {
            if a[i][i].is_neg() {
                return false;
            }
            if a[i][i].is_pos() && piv_opt.is_none() {
                piv_opt = Some(i);
            }
        }
        let Some(piv) = piv_opt else {
            for i in k..n {
                for j in k..n {
                    if !a[i][j].is_zero() {
                        return false;
                    }
                }
            }
            return true;
        };
        if piv != k {
            a.swap(k, piv);
            for row in &mut a {
                row.swap(k, piv);
            }
        }
        let d = a[k][k].clone();
        for i in (k + 1)..n {
            let factor = &a[i][k] / &d;
            for j in (k + 1)..n {
                let delta = &factor * &a[k][j];
                a[i][j] = &a[i][j] - &delta;
            }
        }
        for i in (k + 1)..n {
            a[i][k] = Rat::zero();
            a[k][i] = Rat::zero();
        }
    }
    true
}

// =========================================================================
// Row 1: Port-Hamiltonian form (exact-rational)
// =========================================================================

/// Exact rational Port-Hamiltonian system with quadratic energy `H(x) = ½ xᵀ Q x`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhsSystem {
    /// Interconnection matrix `J` (`n × n`, skew-symmetric for valid PHS).
    pub j: Vec<Vec<Rat>>,
    /// Dissipation matrix `R` (`n × n`, symmetric PSD for valid PHS).
    pub r: Vec<Vec<Rat>>,
    /// Input port matrix `G` (`n × m`).
    pub g: Vec<Vec<Rat>>,
    /// Symmetric energy Hessian `Q` (`n × n`, so `∇H(x) = Q·x`).
    pub q: Vec<Vec<Rat>>,
}

/// Evaluation of a Port-Hamiltonian system at state `x` and port input `u`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhsEval {
    /// State derivative `ẋ = (J - R)∇H + G·u`.
    pub x_dot: Vec<Rat>,
    /// Energy gradient `∇H = Q·x`.
    pub grad_h: Vec<Rat>,
    /// Collocated port output `y = Gᵀ·∇H`.
    pub y: Vec<Rat>,
    /// Direct energy rate `Ḣ = ∇Hᵀ·ẋ`.
    pub h_dot_direct: Rat,
    /// Port supply rate `uᵀ·y`.
    pub supply_power: Rat,
    /// Internal dissipation rate `∇Hᵀ·R·∇H`.
    pub dissipation_power: Rat,
    /// Power-balance RHS `uᵀ·y - ∇Hᵀ·R·∇H`.
    pub h_dot_balance: Rat,
    /// Discrepancy `Ḣ_direct - (uᵀ·y - ∇Hᵀ·R·∇H)` (`= ∇Hᵀ·J·∇H`; zero iff `J` skew).
    pub power_discrepancy: Rat,
}

/// Row 1 oracle: evaluate Port-Hamiltonian dynamics and exact power balance over `ℚ`.
#[must_use]
pub fn phs_step_derivatives(sys: &PhsSystem, x: &[Rat], u: &[Rat]) -> PhsEval {
    let n = sys.q.len();
    assert_eq!(x.len(), n, "state length mismatch");
    let grad_h = mat_vec_n(&sys.q, x);
    let j_gh = mat_vec_n(&sys.j, &grad_h);
    let r_gh = mat_vec_n(&sys.r, &grad_h);
    let g_u = mat_vec_n(&sys.g, u);

    let x_dot: Vec<Rat> = (0..n).map(|i| &(&j_gh[i] - &r_gh[i]) + &g_u[i]).collect();
    let y = mat_t_vec_n(&sys.g, &grad_h);
    let h_dot_direct = dot_n(&grad_h, &x_dot);
    let supply_power = dot_n(u, &y);
    let dissipation_power = dot_n(&grad_h, &r_gh);
    let h_dot_balance = &supply_power - &dissipation_power;
    let power_discrepancy = &h_dot_direct - &h_dot_balance;

    PhsEval {
        x_dot,
        grad_h,
        y,
        h_dot_direct,
        supply_power,
        dissipation_power,
        h_dot_balance,
        power_discrepancy,
    }
}

// =========================================================================
// Row 2: Motor axis (exact-rational PHS + tolerance locked-rotor transient)
// =========================================================================

/// Exact rational parameters of a brushed / q-axis motor (`k_t = k_e = k` in SI).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MotorAxisParams {
    pub l: Rat,
    pub r_w: Rat,
    pub k_t: Rat,
    pub k_e: Rat,
    pub j_r: Rat,
    pub b: Rat,
}

/// Motor axis time derivatives `(di/dt, dω/dt)` and electrical time constant `τ = L/R_w`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MotorAxisDerivatives {
    pub di_dt: Rat,
    pub domega_dt: Rat,
    pub tau_electrical: Rat,
}

/// Exact rational motor-axis equations:
/// `L·di/dt = V - R_w·i - k_e·ω`, `J_r·dω/dt = k_t·i - b·ω - τ_load`, `τ = L/R_w`.
#[must_use]
pub fn motor_axis_derivatives(
    p: &MotorAxisParams,
    i: &Rat,
    omega: &Rat,
    v: &Rat,
    tau_load: &Rat,
) -> MotorAxisDerivatives {
    assert!(p.l.is_pos() && p.r_w.is_pos() && p.j_r.is_pos());
    let di_dt = &(&(v - &(&p.r_w * i)) - &(&p.k_e * omega)) / &p.l;
    let domega_dt = &(&(&(&p.k_t * i) - &(&p.b * omega)) - tau_load) / &p.j_r;
    let tau_electrical = &p.l / &p.r_w;
    MotorAxisDerivatives {
        di_dt,
        domega_dt,
        tau_electrical,
    }
}

/// Build the exact Port-Hamiltonian representation of a motor axis with state
/// `x = (φ, p) = (L·i, J_r·ω)`, `H = φ²/(2L) + p²/(2J_r)`, gyrator `J = [[0, -k_e], [k_t, 0]]`,
/// dissipation `R = diag(R_w, b)`, and `G = I₂` (with `u = (V, -τ_load)`).
#[must_use]
pub fn motor_axis_as_phs(p: &MotorAxisParams) -> PhsSystem {
    assert!(p.l.is_pos() && p.j_r.is_pos());
    let one = Rat::one();
    let zero = Rat::zero();
    PhsSystem {
        j: vec![
            vec![zero.clone(), -&p.k_e],
            vec![p.k_t.clone(), zero.clone()],
        ],
        r: vec![
            vec![p.r_w.clone(), zero.clone()],
            vec![zero.clone(), p.b.clone()],
        ],
        g: vec![
            vec![one.clone(), zero.clone()],
            vec![zero.clone(), one.clone()],
        ],
        q: vec![vec![&one / &p.l, zero.clone()], vec![zero, &one / &p.j_r]],
    }
}

/// Row 2 oracle: locked-rotor step response `(V / R_w) · (1 - e^(-t / τ))` with `τ = L / R_w`.
///
/// Returns `(current_a, tau_s)`. Tolerance: `1e-12 A`.
#[must_use]
pub fn motor_axis_locked_rotor_current(v: f64, r_w: f64, l: f64, t: f64) -> (f64, f64) {
    assert!(
        r_w > 0.0 && l > 0.0 && t >= 0.0,
        "positive R_w, L and non-negative t required"
    );
    let tau = l / r_w;
    let current = (v / r_w) * (-(-t / tau).exp_m1());
    (current, tau)
}

// =========================================================================
// Row 3: Motor envelope (exact-rational, compendium-only)
// =========================================================================

/// Level-L1 steady-state motor envelope parameters (exact rationals).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MotorEnvelopeParams {
    pub i_max: Rat,
    pub v_bus: Rat,
    pub k_e: Rat,
    pub k_t: Rat,
    pub gear_ratio_n: Rat,
    pub resistance: Rat,
    pub eta: Rat,
    pub tau_friction: Rat,
}

/// Level-L1 motor envelope output at joint speed `ω`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MotorEnvelopeEval {
    /// Current ceiling `min(I_max, (V_bus - k_e·N·ω) / R)`.
    pub i_limit: Rat,
    /// Voltage-limited current `(V_bus - k_e·N·ω) / R` before clamping to `I_max`.
    pub i_voltage_limit: Rat,
    /// Output joint torque `η·N·k_t·i - τ_friction` at `i = i_limit`.
    pub tau_out: Rat,
    /// No-load joint speed `ω_nl = V_bus / (k_e·N)` where `i_voltage_limit == 0`.
    pub omega_no_load: Rat,
}

/// Row 3 oracle: level-L1 steady-state motor envelope (`compendium-only`).
#[must_use]
pub fn motor_envelope_l1(p: &MotorEnvelopeParams, omega: &Rat) -> MotorEnvelopeEval {
    assert!(
        p.i_max.is_pos()
            && p.v_bus.is_pos()
            && p.k_e.is_pos()
            && p.gear_ratio_n.is_pos()
            && p.resistance.is_pos()
    );
    let back_emf = &(&p.k_e * &p.gear_ratio_n) * omega;
    let i_voltage_limit = &(&p.v_bus - &back_emf) / &p.resistance;
    let i_limit = if i_voltage_limit < p.i_max {
        i_voltage_limit.clone()
    } else {
        p.i_max.clone()
    };
    let tau_gross = &(&(&p.eta * &p.gear_ratio_n) * &p.k_t) * &i_limit;
    let tau_out = &tau_gross - &p.tau_friction;
    let omega_no_load = &p.v_bus / &(&p.k_e * &p.gear_ratio_n);
    MotorEnvelopeEval {
        i_limit,
        i_voltage_limit,
        tau_out,
        omega_no_load,
    }
}

// =========================================================================
// Row 4: Voltage limit and field weakening (tolerance, compendium-only)
// =========================================================================

/// Non-salient (`L_d = L_q = L`) PMSM voltage-limit parameters in power-invariant dq coordinates.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FieldWeakeningParams {
    pub v_bus: f64,
    pub lambda: f64,
    pub l_d: f64,
    pub l_q: f64,
    pub i_max: f64,
}

/// Top electrical speed under lossless field weakening.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TopSpeed {
    /// Finite limit `V_lim / (λ - L·I_max)` when `λ > L·I_max`.
    Finite(f64),
    /// No finite top speed from the lossless voltage limit when `λ <= L·I_max`
    /// (the ellipse centre `(-λ/L, 0)` lies inside or on the current circle).
    Unbounded,
}

/// Result of the lossless dq voltage-limit and field-weakening oracle.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FieldWeakeningEval {
    /// Inscribed-circle voltage radius `V_lim = V_bus / √2`.
    pub v_lim: f64,
    /// Conservative inscribed-circle voltage margin `1 - √3/2 ≈ 0.133974596`.
    pub inscribed_margin: f64,
    /// Base electrical speed `V_lim / √(λ² + (L·I_max)²)`.
    pub omega_base: f64,
    /// No-load speed without field weakening `V_lim / λ`.
    pub omega_no_load_no_fw: f64,
    /// Top speed with field weakening (`Finite` iff `λ > L·I_max`).
    pub omega_top: TopSpeed,
}

/// Row 4 oracle: voltage limit and field weakening (`compendium-only`, tolerance `1e-9 rad/s`).
#[must_use]
pub fn voltage_limit_field_weakening(p: &FieldWeakeningParams) -> FieldWeakeningEval {
    assert!(
        p.v_bus > 0.0 && p.lambda > 0.0 && p.l_d > 0.0 && p.l_q > 0.0 && p.i_max > 0.0,
        "positive motor voltage-limit parameters required"
    );
    let v_lim = p.v_bus / 2.0_f64.sqrt();
    let inscribed_margin = 1.0 - 3.0_f64.sqrt() / 2.0;
    let l_q_i = p.l_q * p.i_max;
    let omega_base = v_lim / (p.lambda * p.lambda + l_q_i * l_q_i).sqrt();
    let omega_no_load_no_fw = v_lim / p.lambda;
    let denom = p.lambda - p.l_d * p.i_max;
    let omega_top = if denom > 0.0 {
        TopSpeed::Finite(v_lim / denom)
    } else {
        TopSpeed::Unbounded
    };
    FieldWeakeningEval {
        v_lim,
        inscribed_margin,
        omega_base,
        omega_no_load_no_fw,
        omega_top,
    }
}

/// Evaluate the lossless dq voltage ellipse LHS `(λ + L_d·i_d)² + (L_q·i_q)²`
/// and RHS `(V_lim / ω_e)²`.
#[must_use]
pub fn voltage_ellipse_lhs_rhs(
    p: &FieldWeakeningParams,
    i_d: f64,
    i_q: f64,
    omega_e: f64,
) -> (f64, f64) {
    assert!(omega_e > 0.0);
    let v_lim = p.v_bus / 2.0_f64.sqrt();
    let fd = p.lambda + p.l_d * i_d;
    let fq = p.l_q * i_q;
    let lhs = fd * fd + fq * fq;
    let rhs = (v_lim / omega_e) * (v_lim / omega_e);
    (lhs, rhs)
}

/// Closed-form field-weakening currents `(i_d, i_q)` on the boundary above base speed
/// for a non-salient motor (`L_d = L_q = L`). Returns `None` above finite top speed.
#[must_use]
pub fn field_weakening_boundary_currents(
    p: &FieldWeakeningParams,
    omega_e: f64,
) -> Option<(f64, f64)> {
    assert!(
        (p.l_d - p.l_q).abs() <= 1e-15 * p.l_d.max(1.0),
        "non-salient closed form requires L_d == L_q"
    );
    assert!(omega_e > 0.0);
    let eval = voltage_limit_field_weakening(p);
    if omega_e <= eval.omega_base {
        return Some((0.0, p.i_max));
    }
    if let TopSpeed::Finite(top) = eval.omega_top
        && omega_e > top
    {
        return None;
    }
    let l = p.l_d;
    let vr = eval.v_lim / omega_e;
    let i_d =
        (vr * vr - p.lambda * p.lambda - (l * p.i_max) * (l * p.i_max)) / (2.0 * p.lambda * l);
    let iq_sq = p.i_max * p.i_max - i_d * i_d;
    if iq_sq < -1e-12 * p.i_max * p.i_max {
        return None;
    }
    Some((i_d, iq_sq.max(0.0).sqrt()))
}

// =========================================================================
// Row 5: Thermal node (tolerance)
// =========================================================================

/// Lumped single-node winding thermal parameters (`C·Ṫ_w = i²·R₀·(1 + α(T_w - T₀)) - G_th·(T_w - T_amb)`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ThermalNodeParams {
    pub c_th: f64,
    pub g_th: f64,
    pub r_0: f64,
    pub alpha: f64,
    pub t_0: f64,
    pub t_amb: f64,
}

/// Closed-form linear thermal-node evaluation at constant current `i` and time `t`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ThermalNodeEval {
    /// Linear slope `a = (i²·R₀·α - G_th) / C` (`s⁻¹`).
    pub a: f64,
    /// Affine term `c = (i²·R₀·(1 - α·T₀) + G_th·T_amb) / C` (`K·s⁻¹`).
    pub c: f64,
    /// Temperature `T_w(t)` (`K`).
    pub t_w: f64,
    /// Temperature rate `Ṫ_w(t) = a·T_w(t) + c` (`K·s⁻¹`).
    pub dt_dt: f64,
}

/// Instantaneous copper loss `P_loss = i²·R₀·(1 + α·(T_w - T₀))`.
#[must_use]
pub fn copper_loss(p: &ThermalNodeParams, i: f64, t_w: f64) -> f64 {
    i * i * p.r_0 * (1.0 + p.alpha * (t_w - p.t_0))
}

/// Instantaneous temperature derivative `Ṫ_w = (P_loss - G_th·(T_w - T_amb)) / C`.
#[must_use]
pub fn thermal_node_rhs(p: &ThermalNodeParams, i: f64, t_w: f64) -> f64 {
    (copper_loss(p, i, t_w) - p.g_th * (t_w - p.t_amb)) / p.c_th
}

/// Row 5 oracle: exact closed-form exponential solution of the constant-current thermal node.
///
/// Uses `expm1(a·t) / a` so it stays accurate across `a < 0`, `a == 0`, and `a > 0`.
#[must_use]
pub fn thermal_node_closed_form(
    p: &ThermalNodeParams,
    i: f64,
    t_init: f64,
    t: f64,
) -> ThermalNodeEval {
    assert!(p.c_th > 0.0 && p.g_th > 0.0 && p.r_0 > 0.0 && t >= 0.0);
    let s = i * i * p.r_0 * p.alpha;
    let a = (s - p.g_th) / p.c_th;
    let c = (i * i * p.r_0 * (1.0 - p.alpha * p.t_0) + p.g_th * p.t_amb) / p.c_th;
    let rate_0 = a * t_init + c;
    let t_w = if (a * t).abs() < 1e-12 {
        t_init + rate_0 * t * (1.0 + 0.5 * a * t + (a * t) * (a * t) / 6.0)
    } else {
        t_init + rate_0 * (a * t).exp_m1() / a
    };
    let dt_dt = a * t_w + c;
    ThermalNodeEval { a, c, t_w, dt_dt }
}

// =========================================================================
// Row 6: Thermal runaway condition (tolerance)
// =========================================================================

/// Constant-current thermal stability classification.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ThermalRegime {
    /// `s < G_th`: unique stable equilibrium `T_eq = T_amb + P_amb / (G_th - s)`.
    Stable {
        t_eq: f64,
        time_constant: f64,
        stability_margin: f64,
    },
    /// `s >= G_th`: no stable equilibrium; temperature grows without bound.
    Runaway { growth_rate: f64 },
}

/// Complete constant-current thermal runaway report.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ThermalRunawayEval {
    /// Positive thermal feedback slope `s = i²·R₀·α` (`W/K`).
    pub s: f64,
    /// Critical runaway current `I_crit = √(G_th / (R₀·α))` (`A`).
    pub i_crit: f64,
    /// Stability regime at current `i`.
    pub regime: ThermalRegime,
}

/// Row 6 oracle: constant-current thermal runaway threshold and equilibrium (`I_crit = √(G_th/(R₀·α))`).
#[must_use]
pub fn thermal_runaway_analysis(p: &ThermalNodeParams, i: f64) -> ThermalRunawayEval {
    assert!(p.c_th > 0.0 && p.g_th > 0.0 && p.r_0 > 0.0 && p.alpha > 0.0);
    let s = i * i * p.r_0 * p.alpha;
    let i_crit = (p.g_th / (p.r_0 * p.alpha)).sqrt();
    let regime = if s < p.g_th {
        let p_amb = copper_loss(p, i, p.t_amb);
        let denom = p.g_th - s;
        ThermalRegime::Stable {
            t_eq: p.t_amb + p_amb / denom,
            time_constant: p.c_th / denom,
            stability_margin: denom,
        }
    } else {
        ThermalRegime::Runaway {
            growth_rate: (s - p.g_th) / p.c_th,
        }
    };
    ThermalRunawayEval { s, i_crit, regime }
}

/// Torque-control thermal runaway tangency analysis (§15.7):
/// under constant torque with magnet flux coeff `α_B < 0`, copper loss is
/// `P(T) = I₀²·R₀·(1 + α·(T - T₀)) / (1 + α_B·(T - T₀))²`.
///
/// Returns `(i0_tangency_crit, t_tangency, i0_slope_estimate)` where `i0_tangency_crit < i0_slope_estimate`.
#[must_use]
pub fn thermal_runaway_torque_control_tangency(
    p: &ThermalNodeParams,
    alpha_b: f64,
) -> (f64, f64, f64) {
    assert!(p.g_th > 0.0 && p.r_0 > 0.0 && p.alpha > 0.0 && alpha_b < 0.0);
    let d = p.t_amb - p.t_0;
    // Exact quadratic for ΔT = T* - T₀ at tangency:
    // 2·α·α_B·u² + (3·α_B - α·α_B·d)·u + (1 + d·(α - 2·α_B)) = 0
    let qa = 2.0 * p.alpha * alpha_b;
    let qb = 3.0 * alpha_b - p.alpha * alpha_b * d;
    let qc = 1.0 + d * (p.alpha - 2.0 * alpha_b);
    let disc = qb * qb - 4.0 * qa * qc;
    assert!(disc >= 0.0, "tangency discriminant must be non-negative");
    // Since qa < 0 and qc > 0 (near ambient), the physical root with u > d and 1 + alpha_b*u > 0 is (-qb - sqrt(disc)) / (2*qa).
    let u_star = (-qb - disc.sqrt()) / (2.0 * qa);
    let t_star = p.t_0 + u_star;
    let denom = 1.0 + alpha_b * u_star;
    assert!(
        denom > 0.0 && t_star > p.t_amb,
        "physical tangency temperature must satisfy 1 + α_B·ΔT > 0 and T* > T_amb"
    );
    let f_star = (1.0 + p.alpha * u_star) / (denom * denom);
    let i0_tangency = (p.g_th * (t_star - p.t_amb) / (p.r_0 * f_star)).sqrt();
    let i0_slope_est = (p.g_th / (p.r_0 * (p.alpha - 2.0 * alpha_b))).sqrt();
    (i0_tangency, t_star, i0_slope_est)
}

// =========================================================================
// Row 7: Entropy production (exact-rational)
// =========================================================================

/// Row 7 oracle: exact rational entropy production rate
/// `σ = P_loss / T_w + G_th · (T_w - T_amb)² / (T_w · T_amb)`.
#[must_use]
pub fn entropy_production(p_loss: &Rat, g_th: &Rat, t_w: &Rat, t_amb: &Rat) -> Rat {
    assert!(
        t_w.is_pos() && t_amb.is_pos(),
        "absolute temperatures must be strictly positive"
    );
    let dt = t_w - t_amb;
    let dt_sq = &dt * &dt;
    let loss_term = p_loss / t_w;
    let cond_term = &(g_th * &dt_sq) / &(t_w * t_amb);
    &loss_term + &cond_term
}

// =========================================================================
// Row 8: Two-RC battery (tolerance, compendium-only)
// =========================================================================

/// Two-RC equivalent-circuit battery parameters (`I > 0` on discharge).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TwoRcBatteryParams {
    pub r_0: f64,
    pub r_1: f64,
    pub c_1: f64,
    pub r_2: f64,
    pub c_2: f64,
    /// Total charge capacity `Q` in coulombs (`A·s`).
    pub q_coulombs: f64,
}

/// Two-RC battery state at time `t`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TwoRcBatteryState {
    pub v_terminal: f64,
    pub v_1: f64,
    pub v_2: f64,
    pub z: f64,
    pub tau_1: f64,
    pub tau_2: f64,
}

/// Row 8 oracle: closed-form constant-current step response of the two-RC battery (`compendium-only`).
///
/// `V = OCV(z) - R₀·I - V₁ - V₂`, `dV_k/dt = -V_k/(R_k·C_k) + I/C_k`, `dz/dt = -I/Q`.
#[must_use]
pub fn two_rc_battery_step_response(
    p: &TwoRcBatteryParams,
    ocv: fn(f64) -> f64,
    v1_0: f64,
    v2_0: f64,
    z_0: f64,
    i_discharge: f64,
    t: f64,
) -> TwoRcBatteryState {
    assert!(
        p.r_0 >= 0.0
            && p.r_1 > 0.0
            && p.c_1 > 0.0
            && p.r_2 > 0.0
            && p.c_2 > 0.0
            && p.q_coulombs > 0.0
            && t >= 0.0
    );
    let tau_1 = p.r_1 * p.c_1;
    let tau_2 = p.r_2 * p.c_2;
    let v_1 = i_discharge * p.r_1 + (v1_0 - i_discharge * p.r_1) * (-t / tau_1).exp();
    let v_2 = i_discharge * p.r_2 + (v2_0 - i_discharge * p.r_2) * (-t / tau_2).exp();
    let z = z_0 - i_discharge * t / p.q_coulombs;
    let v_terminal = ocv(z) - p.r_0 * i_discharge - v_1 - v_2;
    TwoRcBatteryState {
        v_terminal,
        v_1,
        v_2,
        z,
        tau_1,
        tau_2,
    }
}

/// Exact rational charge balance `z(t) = z₀ - I·t/Q`.
#[must_use]
pub fn battery_soc_exact(z_0: &Rat, i_discharge: &Rat, t: &Rat, q_coulombs: &Rat) -> Rat {
    assert!(q_coulombs.is_pos());
    z_0 - &(&(i_discharge * t) / q_coulombs)
}

// =========================================================================
// Row 9: Bus power ceiling (tolerance + exact rational ceiling, compendium-only)
// =========================================================================

/// Operating point of a DC bus with open-circuit voltage `E`, series resistance `R`,
/// and constant-power demand `P`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BusOperatingPoint {
    /// Feasible operating point (`P <= E²/(4·R)`): high-voltage stable branch
    /// `V = (E + √(E² - 4·R·P)) / 2`, `I = P / V`.
    Feasible {
        voltage: f64,
        current: f64,
        ceiling: f64,
    },
    /// Demand exceeds `E²/(4·R)`: no real operating point exists (brownout).
    Brownout { requested: f64, ceiling: f64 },
}

/// Exact rational bus power ceiling `P_max = E² / (4·R)` and voltage at ceiling `V_crit = E / 2`.
#[must_use]
pub fn bus_power_ceiling_exact(e: &Rat, r: &Rat) -> (Rat, Rat) {
    assert!(e.is_pos() && r.is_pos());
    let p_max = &(e * e) / &(&ri(4) * r);
    let v_crit = e / &ri(2);
    (p_max, v_crit)
}

/// Row 9 oracle: bus power ceiling and constant-power operating point (`compendium-only`).
#[must_use]
pub fn bus_operating_point(e: f64, r: f64, p_load: f64) -> BusOperatingPoint {
    assert!(e > 0.0 && r > 0.0 && p_load >= 0.0);
    let ceiling = (e * e) / (4.0 * r);
    let disc = e * e - 4.0 * r * p_load;
    if p_load > ceiling || disc < 0.0 {
        return BusOperatingPoint::Brownout {
            requested: p_load,
            ceiling,
        };
    }
    let voltage = 0.5 * (e + disc.max(0.0).sqrt());
    let current = (e - voltage) / r;
    BusOperatingPoint::Feasible {
        voltage,
        current,
        ceiling,
    }
}

// =========================================================================
// Row 10: Four-quadrant transmission (tolerance + exact rational quadrant law, compendium-only)
// =========================================================================

/// Back-driving status of a helical screw transmission.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ScrewBackdrive {
    /// `φ < a`: back-drivable with efficiency `η_b = tan(a - φ) / tan(a) > 0`.
    Backdrivable(f64),
    /// `φ >= a`: self-locking (`η_b <= 0`); cannot be back-driven.
    SelfLocking,
}

/// Helical screw efficiencies from lead angle `a` and friction angle `φ` (radians).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScrewEfficiencyEval {
    /// Forward motoring efficiency `η_f = tan(a) / tan(a + φ)`.
    pub eta_f: f64,
    /// Back-driving status and efficiency.
    pub backdrive: ScrewBackdrive,
}

/// Row 10 oracle: helical screw efficiencies and self-locking threshold (`compendium-only`).
#[must_use]
pub fn screw_efficiencies(lead_angle_rad: f64, friction_angle_rad: f64) -> ScrewEfficiencyEval {
    assert!(
        lead_angle_rad > 0.0
            && friction_angle_rad >= 0.0
            && (lead_angle_rad + friction_angle_rad) < core::f64::consts::FRAC_PI_2,
        "angles must satisfy a > 0, phi >= 0, a + phi < pi/2"
    );
    let eta_f = lead_angle_rad.tan() / (lead_angle_rad + friction_angle_rad).tan();
    let backdrive = if friction_angle_rad >= lead_angle_rad {
        ScrewBackdrive::SelfLocking
    } else {
        ScrewBackdrive::Backdrivable(
            (lead_angle_rad - friction_angle_rad).tan() / lead_angle_rad.tan(),
        )
    };
    ScrewEfficiencyEval { eta_f, backdrive }
}

/// Exact rational four-quadrant motoring torque `τ_joint = η_f · N · τ_motor`.
#[must_use]
pub fn transmission_motoring_torque(eta_f: &Rat, ratio_n: &Rat, tau_motor: &Rat) -> Rat {
    &(eta_f * ratio_n) * tau_motor
}

/// Exact rational four-quadrant back-driven motor torque `τ_motor = η_b · τ_joint / N`.
#[must_use]
pub fn transmission_backdriven_torque(eta_b: &Rat, ratio_n: &Rat, tau_joint: &Rat) -> Rat {
    assert!(ratio_n.is_pos());
    &(eta_b * tau_joint) / ratio_n
}

// =========================================================================
// Row 11: Rational gear law (exact-rational)
// =========================================================================

/// Output of an ideal rational transformer `f₂ = r·f₁`, `e₁ = -r·e₂`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GearLawEval {
    pub f_2: Rat,
    pub e_1: Rat,
    /// Net port power `e₁·f₁ + e₂·f₂` (identically zero in `ℚ`).
    pub net_power: Rat,
}

/// Row 11 oracle: ideal rational gear transformer (`f₂ = r·f₁`, `e₁ = -r·e₂`, `e₁·f₁ + e₂·f₂ = 0`).
#[must_use]
pub fn rational_gear_map(ratio_r: &Rat, f_1: &Rat, e_2: &Rat) -> GearLawEval {
    let f_2 = ratio_r * f_1;
    let e_1 = -(&(ratio_r * e_2));
    let net_power = &(&e_1 * f_1) + &(e_2 * &f_2);
    GearLawEval {
        f_2,
        e_1,
        net_power,
    }
}

/// Compose two rational gear transformers of ratios `r` and `s` in series, returning
/// the composite ratio `r·s` and verifying power conservation `e₁·f₁ + e₃·f₃ == 0`.
#[must_use]
pub fn compose_rational_gears(r: &Rat, s: &Rat, f_1: &Rat, e_3: &Rat) -> (Rat, GearLawEval) {
    // Stage 2: (e2_in, f2_in) -> (e3, f3) with f3 = s * f2_in, e2_in = -s * e3.
    // Power-conserving 1-junction: f2_in = f2, e2 = -e2_in = s * e3.
    let f_2 = r * f_1;
    let stage2 = rational_gear_map(s, &f_2, e_3);
    let e_2_stage1 = -&stage2.e_1;
    let stage1 = rational_gear_map(r, f_1, &e_2_stage1);
    let net = &(&stage1.e_1 * f_1) + &(e_3 * &stage2.f_2);
    let composite_ratio = r * s;
    (
        composite_ratio,
        GearLawEval {
            f_2: stage2.f_2,
            e_1: stage1.e_1,
            net_power: net,
        },
    )
}

// =========================================================================
// Row 12: Energy balance (exact-rational)
// =========================================================================

/// Per-step energy ledger line items (all in joules over `ℚ`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnergyLedgerStep {
    pub supplied: Rat,
    pub delta_stored: Rat,
    pub dissipated: Rat,
    pub exported: Rat,
}

/// Row 12 oracle: exact rational energy-balance residual
/// `supplied - (delta_stored + dissipated + exported)`.
#[must_use]
pub fn energy_balance_residual(step: &EnergyLedgerStep) -> Rat {
    let accounted = &(&step.delta_stored + &step.dissipated) + &step.exported;
    &step.supplied - &accounted
}

/// State of an exact discrete-gradient motor–thermal step in `ℚ`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MotorThermalState {
    pub current_i: Rat,
    pub omega: Rat,
    pub temp_w: Rat,
}

/// Exact discrete-gradient integration of one coupled motor + thermal-node step over `ℚ`.
///
/// Solves the midpoint equations:
/// - `L·(i₁ - i₀)/h = V - R_w·i_m - k·ω_m`
/// - `J_r·(ω₁ - ω₀)/h = k·i_m - b·ω_m - τ_load`
/// - `C·(T₁ - T₀) = h·(R_w·i_m² + b·ω_m²) - h·G_th·(T_m - T_amb)`
/// where `x_m = (x₀ + x₁)/2`. Returns `(next_state, ledger_step, heat_into_node)`.
#[must_use]
pub fn motor_thermal_exact_step(
    motor: &MotorAxisParams,
    c_th: &Rat,
    g_th: &Rat,
    t_amb: &Rat,
    state: &MotorThermalState,
    v: &Rat,
    tau_load: &Rat,
    h: &Rat,
) -> (MotorThermalState, EnergyLedgerStep, Rat) {
    assert_eq!(
        motor.k_t, motor.k_e,
        "energy-conserving gyrator requires k_t == k_e"
    );
    assert!(motor.l.is_pos() && motor.j_r.is_pos() && c_th.is_pos() && h.is_pos());
    let two = ri(2);
    let half = Rat::new(1, 2);

    // Solve 2×2 linear system for midpoint (i_m, ω_m):
    // Since i₁ = 2·i_m - i₀, (i₁ - i₀)/h = (2/h)·(i_m - i₀):
    //   (2L/h + R_w)·i_m + k·ω_m = V + (2L/h)·i₀
    //   -k·i_m + (2J_r/h + b)·ω_m = -τ_load + (2J_r/h)·ω₀
    let a11 = &(&(&two * &motor.l) / h) + &motor.r_w;
    let a12 = motor.k_e.clone();
    let a21 = -&motor.k_t;
    let a22 = &(&(&two * &motor.j_r) / h) + &motor.b;
    let rhs1 = v + &(&(&(&two * &motor.l) / h) * &state.current_i);
    let rhs2 = &(-tau_load) + &(&(&(&two * &motor.j_r) / h) * &state.omega);

    let det = &(&a11 * &a22) - &(&a12 * &a21);
    assert!(det.is_pos(), "motor step matrix must be invertible");
    let i_m = &(&(&rhs1 * &a22) - &(&a12 * &rhs2)) / &det;
    let omega_m = &(&(&a11 * &rhs2) - &(&rhs1 * &a21)) / &det;

    let i_1 = &(&two * &i_m) - &state.current_i;
    let omega_1 = &(&two * &omega_m) - &state.omega;

    // Heat generated inside actuator and routed into the thermal node:
    let p_loss = &(&motor.r_w * &(&i_m * &i_m)) + &(&motor.b * &(&omega_m * &omega_m));
    let q_gen = h * &p_loss;

    // Thermal node midpoint step: C·(T₁ - T₀) = q_gen - h·G_th·((T₀ + T₁)/2 - T_amb)
    // => (C + h·G_th/2)·T₁ = (C - h·G_th/2)·T₀ + q_gen + h·G_th·T_amb
    let half_hg = &(&half * h) * g_th;
    let t_1 = &(&(&(c_th - &half_hg) * &state.temp_w) + &(&q_gen + &(h * &(g_th * t_amb))))
        / &(c_th + &half_hg);
    let t_m = &half * &(&state.temp_w + &t_1);
    let q_amb = &(h * g_th) * &(&t_m - t_amb);

    // Exact stored energy increments:
    let dh_mag = &(&half * &motor.l) * &(&(&i_1 * &i_1) - &(&state.current_i * &state.current_i));
    let dh_rot = &(&half * &motor.j_r) * &(&(&omega_1 * &omega_1) - &(&state.omega * &state.omega));
    let du_th = c_th * &(&t_1 - &state.temp_w);
    let delta_stored = &(&dh_mag + &dh_rot) + &du_th;

    // Ports: electrical work in = h·V·i_m; mechanical work out + ambient heat out = exported.
    // Because internal heat q_gen enters the thermal node (stored in du_th), dissipated = 0.
    let supplied = &(h * v) * &i_m;
    let w_mech = &(h * tau_load) * &omega_m;
    let exported = &w_mech + &q_amb;

    let ledger = EnergyLedgerStep {
        supplied,
        delta_stored,
        dissipated: Rat::zero(),
        exported,
    };
    let next = MotorThermalState {
        current_i: i_1,
        omega: omega_1,
        temp_w: t_1,
    };
    (next, ledger, q_gen)
}

// =========================================================================
// Row 13: Cost of transport (exact-rational, compendium-only)
// =========================================================================

/// Energy accounting domain for Cost of Transport (`Mechanical` and `Electrical`
/// must be reported separately and never mixed).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EnergyDomain {
    Mechanical,
    Electrical,
}

/// Error returned when a Cost of Transport claim mixes mechanical and electrical energy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EnergyDomainMismatch {
    pub numerator_domain: EnergyDomain,
    pub claimed_domain: EnergyDomain,
}

/// Row 13 oracle: exact rational Cost of Transport `CoT = E / (m·g·d)` (`compendium-only`).
#[must_use]
pub fn cost_of_transport(energy: &Rat, mass: &Rat, gravity: &Rat, distance: &Rat) -> Rat {
    assert!(mass.is_pos() && gravity.is_pos() && distance.is_pos());
    energy / &(&(mass * gravity) * distance)
}

/// Domain-checked Cost of Transport refusing a mixed mechanical/electrical claim.
pub fn cost_of_transport_checked(
    numerator_domain: EnergyDomain,
    claimed_domain: EnergyDomain,
    energy: &Rat,
    mass: &Rat,
    gravity: &Rat,
    distance: &Rat,
) -> Result<Rat, EnergyDomainMismatch> {
    if numerator_domain != claimed_domain {
        return Err(EnergyDomainMismatch {
            numerator_domain,
            claimed_domain,
        });
    }
    Ok(cost_of_transport(energy, mass, gravity, distance))
}

// =========================================================================
// Row 14: Froude number (exact-rational, compendium-only)
// =========================================================================

/// Row 14 oracle: exact rational Froude number `Fr = v² / (g·L)` (`compendium-only`).
#[must_use]
pub fn froude_number(velocity: &Rat, gravity: &Rat, leg_length: &Rat) -> Rat {
    assert!(gravity.is_pos() && leg_length.is_pos());
    &(velocity * velocity) / &(gravity * leg_length)
}
