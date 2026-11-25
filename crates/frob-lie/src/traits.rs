//! Core Lie-group and Lie-algebra traits and error vocabulary (`BD-010`).

use core::fmt;

use frob_core::qty::Power;
use fs_ga::GaError;

/// Error returned by fallible Lie-group operations in `frob-lie`.
///
/// Wraps `fs_ga::GaError` directly so non-finite inputs, degenerate norms, and
/// off-manifold defects above `fs_ga::UNIT_TOLERANCE` are always refused without
/// panicking or silent renormalization.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LieError {
    /// Underlying `fs-ga` host representation refusal.
    Host(GaError),
    /// Operation declared in `BD-010` whose implementation lands in a later bead.
    DeferredOperation {
        op: &'static str,
        bead: &'static str,
    },
}

impl LieError {
    /// Return the underlying `fs_ga::GaError` when this refusal came from `fs-ga`.
    #[must_use]
    pub const fn ga_error(self) -> Option<GaError> {
        match self {
            Self::Host(err) => Some(err),
            Self::DeferredOperation { .. } => None,
        }
    }
}

impl From<GaError> for LieError {
    #[inline]
    fn from(err: GaError) -> Self {
        Self::Host(err)
    }
}

impl fmt::Display for LieError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Host(err) => write!(f, "fs-ga refusal: {err}"),
            Self::DeferredOperation { op, bead } => {
                write!(f, "operation `{op}` is deferred to bead {bead}")
            }
        }
    }
}

impl std::error::Error for LieError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Host(err) => Some(err),
            Self::DeferredOperation { .. } => None,
        }
    }
}

/// A finite-dimensional matrix/rigid-body Lie group backed by `fs-ga`.
///
/// Fallible operations (`compose`, `inverse`, `exp`, `log`, `cay`, `cay_inv`)
/// return `Result<_, LieError>` carrying `fs_ga::GaError` as cause; they never
/// panic and never silently renormalize invalid host representations.
pub trait LieGroup: Copy + Sized {
    /// Associated Lie algebra `g`.
    type Algebra: LieAlgebra;

    /// Real dimension of the group manifold (`3` for `SO(3)`, `6` for `SE(3)`).
    const DIM: usize;

    /// Group identity element `e`.
    #[must_use]
    fn identity() -> Self;

    /// Group composition `self * rhs` (reads left-to-right cancelling inner frames:
    /// `a_from_b.compose(b_from_c) = a_from_c`).
    fn compose(self, rhs: Self) -> Result<Self, LieError>;

    /// Group inverse `self^{-1}`.
    fn inverse(self) -> Result<Self, LieError>;

    /// Exponential map `exp: g -> G` via deterministic elementary functions.
    fn exp(x: Self::Algebra) -> Result<Self, LieError>;

    /// Logarithm map `log: G -> g` (chart-cut refusals land in `BD-011`).
    fn log(self) -> Result<Self::Algebra, LieError>;

    /// Cayley retraction `cay: g -> G` (algebraic `+,-,*,/,sqrt`; lands in `BD-011`).
    fn cay(x: Self::Algebra) -> Result<Self, LieError>;

    /// Inverse Cayley retraction `cay^{-1}: G -> g` (lands in `BD-011`).
    fn cay_inv(self) -> Result<Self::Algebra, LieError>;

    /// Adjoint representation `Ad_g: g -> g`.
    #[must_use]
    fn adjoint(self) -> <Self::Algebra as LieAlgebra>::Operator;
}

/// A Lie algebra `g` together with its dual `g*`, adjoint operator, infinitesimal
/// coadjoint `ad* = ad^T` (ADR-MARROW-05), and SI power pairing `<mu, x>`.
pub trait LieAlgebra: Copy + Sized {
    /// Dual space `g*` (torques / angular momenta on `so(3)*`, wrenches / body momenta on `se(3)*`).
    type Dual: Copy;

    /// Linear operator representation on `g` (`Mat3` for `so(3)`, `Mat6` for `se(3)`).
    type Operator;

    /// Lie bracket `[self, rhs] = ad_self(rhs)`.
    #[must_use]
    fn bracket(self, rhs: Self) -> Self;

    /// Infinitesimal adjoint matrix `ad_self` satisfying `ad_self(rhs) = [self, rhs]`.
    #[must_use]
    fn ad(self) -> Self::Operator;

    /// Infinitesimal coadjoint `ad*_self(mu) = (ad_self)^T mu` (NO minus sign; ADR-MARROW-05).
    ///
    /// Uniquely characterized by `<ad*_x mu, y> = <mu, [x, y]>` for all `y in g`.
    #[must_use]
    fn coad(self, mu: Self::Dual) -> Self::Dual;

    /// Natural dual-algebra pairing `<mu, self>` returning mechanical power in watts.
    #[must_use]
    fn pair(self, mu: Self::Dual) -> Power;
}
