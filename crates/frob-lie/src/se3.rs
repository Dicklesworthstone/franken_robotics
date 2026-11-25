//! `SE(3)` and `se(3)` implementations bound directly onto `fs-ga` (`BD-010`).
//!
//! `Se3`, `Twist`, and `Wrench` are `fs_ga` types directly (ADR-MARROW-01).
//! Normative conventions:
//! - `g = (R, p)` is `a_from_b` acting on points as `x_a = R x_b + p`.
//! - Twist `xi = (omega, v)` is angular-first (`fs_ga::Twist::new(angular, linear)`).
//! - Wrench `w = (tau, f)` is torque-first (`fs_ga::Wrench::new(torque, force)`).
//! - Adjoint `Ad_g(omega, v) = (R omega, R v + p x R omega)`.
//! - Group coadjoint `Ad*_g = (Ad_g)^{-T}`: `Ad*_g(tau, f) = (R tau + p x R f, R f)`.
//! - Bracket `[xi, eta] = (omega x eta_omega, omega x eta_v + v x eta_omega)`.
//! - Infinitesimal coadjoint `ad*_xi = (ad_xi)^T` (no minus sign, ADR-MARROW-05):
//!   `ad*_xi(pi, ell) = (pi x omega + ell x v, ell x omega)`.

use frob_core::qty::Power;
use fs_ga::{Mat6, Motor, Se3, So3, Twist, Vec3, Wrench};

use crate::traits::{LieAlgebra, LieError, LieGroup};

/// Construct a twist `xi = (omega, v)` in `frob-lie`'s normative angular-first
/// order (`fs_ga::Twist::new(angular, linear)`).
#[inline]
#[must_use]
pub fn twist(angular: Vec3, linear: Vec3) -> Twist {
    #[cfg(feature = "twist-swap")]
    {
        Twist::new(linear, angular)
    }
    #[cfg(not(feature = "twist-swap"))]
    {
        Twist::new(angular, linear)
    }
}

/// Construct a wrench `w = (tau, f)` in `frob-lie`'s normative torque-first
/// order (`fs_ga::Wrench::new(torque, force)`).
#[inline]
#[must_use]
pub fn wrench(torque: Vec3, force: Vec3) -> Wrench {
    Wrench::new(torque, force)
}

/// Extension operations on `fs_ga::Se3` surfacing fallible `fs-ga` refusals as
/// [`LieError`] and providing bit-exact adjoint, group coadjoint, and pull-back maps.
pub trait Se3Ops: Sized {
    /// Construct from `(rotation, translation)` using `fs-ga`'s unit/finiteness check.
    fn from_parts_lie(rotation: So3, translation: Vec3) -> Result<Self, LieError>;

    /// Construct from a raw PGA motor using `fs-ga`'s unit/study check.
    fn try_from_motor_lie(motor: Motor) -> Result<Self, LieError>;

    /// Infallible `SE(3)` logarithm matching `fs_ga::Se3::log`.
    #[must_use]
    fn log_exact(self) -> Twist;

    /// Transform a 3D point `x_b` to `x_a = R x_b + p`.
    fn transform_point_lie(self, point: Vec3) -> Result<Vec3, LieError>;

    /// Transform a free 3D vector `v_b` to `v_a = R v_b`.
    fn transform_vector_lie(self, vec: Vec3) -> Result<Vec3, LieError>;

    /// Push forward a twist `xi_b` to `xi_a = Ad_g xi_b` via `fs_ga::Twist::transform_by`.
    #[must_use]
    fn adjoint_twist(self, xi: Twist) -> Twist;

    /// Group coadjoint matrix `Ad*_g = (Ad_g)^{-T}` via `fs_ga::Se3::coadjoint`.
    #[must_use]
    fn coadjoint_matrix(self) -> Mat6;

    /// Push forward a wrench `w_b` to `w_a = Ad*_g w_b` via `fs_ga::Wrench::transform_by`.
    #[must_use]
    fn coadjoint_wrench(self, w: Wrench) -> Wrench;

    /// Pull back a wrench `w_a` to `w_b = Ad*_{g^{-1}} w_a` by pushing forward along `g^{-1}`.
    fn pullback_wrench(self, w: Wrench) -> Result<Wrench, LieError>;

    /// Pull back a wrench `w_a` via `g^{-1}.coadjoint_matrix().apply_wrench(w_a)`.
    fn pullback_wrench_matrix(self, w: Wrench) -> Result<Wrench, LieError>;
}

impl Se3Ops for Se3 {
    #[inline]
    fn from_parts_lie(rotation: So3, translation: Vec3) -> Result<Self, LieError> {
        Self::from_parts(rotation, translation).map_err(LieError::from)
    }

    #[inline]
    fn try_from_motor_lie(motor: Motor) -> Result<Self, LieError> {
        Self::try_from_motor(motor).map_err(LieError::from)
    }

    #[inline]
    fn log_exact(self) -> Twist {
        Self::log(self)
    }

    #[inline]
    fn transform_point_lie(self, point: Vec3) -> Result<Vec3, LieError> {
        Self::transform_point(self, point).map_err(LieError::from)
    }

    #[inline]
    fn transform_vector_lie(self, vec: Vec3) -> Result<Vec3, LieError> {
        Self::transform_vector(self, vec).map_err(LieError::from)
    }

    #[inline]
    fn adjoint_twist(self, xi: Twist) -> Twist {
        #[cfg(feature = "ad-inverse")]
        {
            match Self::inverse(self) {
                Ok(inv) => xi.transform_by(&inv),
                Err(_) => xi.transform_by(&self),
            }
        }
        #[cfg(not(feature = "ad-inverse"))]
        {
            xi.transform_by(&self)
        }
    }

    #[inline]
    fn coadjoint_matrix(self) -> Mat6 {
        #[cfg(feature = "coadjoint-inverse")]
        {
            match Self::inverse(self) {
                Ok(inv) => Self::coadjoint(inv),
                Err(_) => Self::coadjoint(self),
            }
        }
        #[cfg(not(feature = "coadjoint-inverse"))]
        {
            Self::coadjoint(self)
        }
    }

    #[inline]
    fn coadjoint_wrench(self, w: Wrench) -> Wrench {
        #[cfg(feature = "coadjoint-inverse")]
        {
            match Self::inverse(self) {
                Ok(inv) => w.transform_by(&inv),
                Err(_) => w.transform_by(&self),
            }
        }
        #[cfg(not(feature = "coadjoint-inverse"))]
        {
            w.transform_by(&self)
        }
    }

    #[inline]
    fn pullback_wrench(self, w: Wrench) -> Result<Wrench, LieError> {
        let inv = Self::inverse(self).map_err(LieError::from)?;
        Ok(inv.coadjoint_wrench(w))
    }

    #[inline]
    fn pullback_wrench_matrix(self, w: Wrench) -> Result<Wrench, LieError> {
        let inv = Self::inverse(self).map_err(LieError::from)?;
        Ok(inv.coadjoint_matrix().apply_wrench(w))
    }
}

impl LieGroup for Se3 {
    type Algebra = Twist;
    const DIM: usize = 6;

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
        Self::inverse(self).map_err(LieError::from)
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
            op: "Se3::cay",
            bead: "BD-011",
        })
    }

    #[inline]
    fn cay_inv(self) -> Result<Self::Algebra, LieError> {
        Err(LieError::DeferredOperation {
            op: "Se3::cay_inv",
            bead: "BD-011",
        })
    }

    #[inline]
    fn adjoint(self) -> Mat6 {
        #[cfg(feature = "ad-inverse")]
        {
            match Self::inverse(self) {
                Ok(inv) => Self::adjoint(inv),
                Err(_) => Self::adjoint(self),
            }
        }
        #[cfg(not(feature = "ad-inverse"))]
        {
            Self::adjoint(self)
        }
    }
}

impl LieAlgebra for Twist {
    type Dual = Wrench;
    type Operator = Mat6;

    #[inline]
    fn bracket(self, rhs: Self) -> Self {
        #[cfg(feature = "bracket-swap")]
        {
            Self::bracket(rhs, self)
        }
        #[cfg(not(feature = "bracket-swap"))]
        {
            Self::bracket(self, rhs)
        }
    }

    #[inline]
    fn ad(self) -> Mat6 {
        #[cfg(feature = "bracket-swap")]
        {
            let mut m = Self::ad(self);
            for slot in &mut m.m {
                *slot = -*slot;
            }
            m
        }
        #[cfg(not(feature = "bracket-swap"))]
        {
            Self::ad(self)
        }
    }

    #[inline]
    fn coad(self, mu: Wrench) -> Wrench {
        // ad*_xi(pi, ell) = (ad_xi)^T (pi, ell) = (pi x omega + ell x v, ell x omega)
        // with NO minus sign (ADR-MARROW-05).
        // Proof: <ad*_xi(pi, ell), (eta_w, eta_v)>
        //      = (pi x omega + ell x v) . eta_w + (ell x omega) . eta_v
        //      = pi . (omega x eta_w) + ell . (v x eta_w + omega x eta_v)
        //      = <(pi, ell), [xi, eta]>.
        let torque = mu.torque.cross(self.angular) + mu.force.cross(self.linear);
        let force = mu.force.cross(self.angular);
        Wrench::new(torque, force)
    }

    #[inline]
    fn pair(self, mu: Wrench) -> Power {
        Power::new(mu.pairing(self))
    }
}
