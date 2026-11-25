//! `SO(3)` and `so(3)` implementations bound directly onto `fs-ga` (`BD-010`).
//!
//! `So3` and `So3Tangent` are `fs_ga` types directly (ADR-MARROW-01). No wrapper
//! structs are introduced; double-cover canonicalization is `fs-ga`'s rule verbatim.

use frob_core::qty::Power;
use fs_ga::{Mat3, Motor, Quat, So3, So3Tangent, Vec3};

use crate::traits::{LieAlgebra, LieError, LieGroup};

/// Extension operations on `fs_ga::So3` surfacing `fs-ga` refusals as [`LieError`]
/// and exposing the group coadjoint on `so(3)*`.
pub trait So3Ops: Sized {
    /// Construct from a raw quaternion `(w, x, y, z)` using `fs-ga`'s unit check
    /// and canonical hemisphere sign rule (never silently renormalizes).
    fn try_from_quat_lie(q: Quat) -> Result<Self, LieError>;

    /// Construct from a pure-rotation PGA motor via `fs-ga`.
    fn try_from_motor_lie(m: Motor) -> Result<Self, LieError>;

    /// Rotate a 3-vector `v_b` into frame `a`: `v_a = R v_b`.
    fn rotate_vec(self, v: Vec3) -> Result<Vec3, LieError>;

    /// Infallible `SO(3)` inverse matching `fs_ga::So3::inverse`.
    #[must_use]
    fn inverse_exact(self) -> Self;

    /// Infallible `SO(3)` logarithm matching `fs_ga::So3::log`.
    #[must_use]
    fn log_exact(self) -> So3Tangent;

    /// Group coadjoint matrix `Ad*_R = (Ad_R)^{-T} = R` on `so(3)*` (since `R` is orthogonal).
    #[must_use]
    fn coadjoint_matrix(self) -> Mat3;

    /// Push forward a dual element (torque / angular momentum) `mu in so(3)*`: `Ad*_R mu = R mu`.
    #[must_use]
    fn coadjoint_dual(self, mu: Vec3) -> Vec3;
}

impl So3Ops for So3 {
    #[inline]
    fn try_from_quat_lie(q: Quat) -> Result<Self, LieError> {
        Self::try_from_quat(q).map_err(LieError::from)
    }

    #[inline]
    fn try_from_motor_lie(m: Motor) -> Result<Self, LieError> {
        Self::try_from_motor(m).map_err(LieError::from)
    }

    #[inline]
    fn rotate_vec(self, v: Vec3) -> Result<Vec3, LieError> {
        Self::rotate(self, v).map_err(LieError::from)
    }

    #[inline]
    fn inverse_exact(self) -> Self {
        Self::inverse(self)
    }

    #[inline]
    fn log_exact(self) -> So3Tangent {
        Self::log(self)
    }

    #[inline]
    fn coadjoint_matrix(self) -> Mat3 {
        #[cfg(feature = "coadjoint-inverse")]
        {
            Self::inverse(self).matrix()
        }
        #[cfg(not(feature = "coadjoint-inverse"))]
        {
            Self::matrix(self)
        }
    }

    #[inline]
    fn coadjoint_dual(self, mu: Vec3) -> Vec3 {
        self.coadjoint_matrix().apply(mu)
    }
}

impl LieGroup for So3 {
    type Algebra = So3Tangent;
    const DIM: usize = 3;

    #[inline]
    fn identity() -> Self {
        Self::identity()
    }

    #[inline]
    fn compose(self, rhs: Self) -> Result<Self, LieError> {
        Self::compose(self, rhs).map_err(LieError::from)
    }

    #[inline]
    fn inverse(self) -> Result<Self, LieError> {
        Ok(Self::inverse(self))
    }

    #[inline]
    fn exp(x: Self::Algebra) -> Result<Self, LieError> {
        Self::exp(x).map_err(LieError::from)
    }

    #[inline]
    fn log(self) -> Result<Self::Algebra, LieError> {
        Ok(Self::log(self))
    }

    #[inline]
    fn cay(_x: Self::Algebra) -> Result<Self, LieError> {
        Err(LieError::DeferredOperation {
            op: "So3::cay",
            bead: "BD-011",
        })
    }

    #[inline]
    fn cay_inv(self) -> Result<Self::Algebra, LieError> {
        Err(LieError::DeferredOperation {
            op: "So3::cay_inv",
            bead: "BD-011",
        })
    }

    #[inline]
    fn adjoint(self) -> Mat3 {
        #[cfg(feature = "ad-inverse")]
        {
            Self::inverse(self).matrix()
        }
        #[cfg(not(feature = "ad-inverse"))]
        {
            Self::matrix(self)
        }
    }
}

impl LieAlgebra for So3Tangent {
    type Dual = Vec3;
    type Operator = Mat3;

    #[inline]
    fn bracket(self, rhs: Self) -> Self {
        #[cfg(feature = "bracket-swap")]
        {
            Self::new(rhs.angular.cross(self.angular))
        }
        #[cfg(not(feature = "bracket-swap"))]
        {
            Self::new(self.angular.cross(rhs.angular))
        }
    }

    #[inline]
    fn ad(self) -> Mat3 {
        let w = self.angular;
        #[cfg(feature = "bracket-swap")]
        {
            Mat3 {
                m: [0.0, w.z, -w.y, -w.z, 0.0, w.x, w.y, -w.x, 0.0],
            }
        }
        #[cfg(not(feature = "bracket-swap"))]
        {
            Mat3 {
                m: [0.0, -w.z, w.y, w.z, 0.0, -w.x, -w.y, w.x, 0.0],
            }
        }
    }

    #[inline]
    fn coad(self, mu: Vec3) -> Vec3 {
        // ad*_omega(mu) = (ad_omega)^T mu = mu x omega (no minus sign, ADR-MARROW-05).
        // Satisfies <mu x omega, eta> = <mu, omega x eta> = <mu, [omega, eta]>.
        mu.cross(self.angular)
    }

    #[inline]
    fn pair(self, mu: Vec3) -> Power {
        Power::new(mu.dot(self.angular))
    }
}
