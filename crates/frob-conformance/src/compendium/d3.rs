//! Appendix D.3 exact and tolerance oracle library: contact, wheels, and terrain (`N12-43`).
//!
//! Independent reference implementations for the ten rows of Appendix D.3:
//! 1. Friction cone `K_mu = { ||gamma_t|| <= mu * gamma_n }` and dual `K_{1/mu}` (exact over `Q`).
//! 2. Spin-factor Jordan algebra `(x0, xbar) o (y0, ybar)` (exact over `Q`).
//! 3. Fast-tier contact optimization `argmin 1/2 (v - v*)^T A (v - v*) + sum l_c(J_c v)` (exact over `Q`).
//! 4. Four-dimensional friction cone `K = { ||(lambda_t1, lambda_t2, lambda_s / r)|| <= mu * lambda_n }`
//!    with uniform (`2R/3`) and Hertzian (`3*pi*R/16`) patch radii.
//! 5. Joint-level rows: stiction, two-unilateral backlash, and hard stop (exact over `Q`).
//! 6. Nonholonomic growth vector and sub-Riemannian cost (`4 * sqrt(d * b)`).
//! 7. Squeeze film (Stefan's law, Hersey number; `compendium-only`, `Estimated`).
//! 8. Pressure–sinkage (Bekker–Wong) and shear (Janosi–Hanamoto) with symbolic dimensional check
//!    (`compendium-only`, `Estimated`).
//! 9. Swept right-angle corner (`L <= 2 * sqrt(2) * a`).
//! 10. Clearance sandwich `C_in <= C_free <= C_out` on a cubical SE(2) doorway complex.
//!
//! Pure `std` + `crate::compendium::rat` only; zero production crate dependencies, zero `unsafe`.

use std::collections::VecDeque;
use std::f64::consts::{PI, SQRT_2};

use crate::compendium::rat::{
    Mat3, Rat, Vec3, dot3, mat3_add, mat3_inv, mat3_scale, mat3_vec, outer3, r, ri, vec3_add,
    vec3_scale, vec3_sub,
};

// =========================================================================
// Row 1: Friction cone (exact over Q on rational/Pythagorean points)
// =========================================================================

/// 3D contact force/velocity vector `(normal, [t1, t2])` over exact rationals `Q`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cone3 {
    pub n: Rat,
    pub t: [Rat; 2],
}

impl Cone3 {
    #[must_use]
    pub const fn new(n: Rat, t1: Rat, t2: Rat) -> Self {
        Self { n, t: [t1, t2] }
    }

    #[must_use]
    pub fn zero() -> Self {
        Self::new(Rat::zero(), Rat::zero(), Rat::zero())
    }

    #[must_use]
    pub fn tangent_norm_sq(&self) -> Rat {
        &(&self.t[0] * &self.t[0]) + &(&self.t[1] * &self.t[1])
    }

    #[must_use]
    pub fn dot(&self, rhs: &Self) -> Rat {
        &(&self.n * &rhs.n) + &(&(&self.t[0] * &rhs.t[0]) + &(&self.t[1] * &rhs.t[1]))
    }

    #[must_use]
    pub fn sub(&self, rhs: &Self) -> Self {
        Self::new(
            &self.n - &rhs.n,
            &self.t[0] - &rhs.t[0],
            &self.t[1] - &rhs.t[1],
        )
    }

    #[must_use]
    pub fn norm_sq(&self) -> Rat {
        self.dot(self)
    }
}

/// Exact membership test `x in K_mu = { ||gamma_t|| <= mu * gamma_n, gamma_n >= 0 }` (`mu > 0`).
#[must_use]
pub fn in_friction_cone_exact(x: &Cone3, mu: &Rat) -> bool {
    if !mu.is_pos() || x.n.is_neg() {
        return false;
    }
    let rhs = mu * &x.n;
    x.tangent_norm_sq() <= (&rhs * &rhs)
}

/// Exact membership test in the dual cone `K_{1/mu} = { mu * ||y_t|| <= y_n, y_n >= 0 }` (`mu > 0`).
#[must_use]
pub fn in_dual_friction_cone_exact(y: &Cone3, mu: &Rat) -> bool {
    if !mu.is_pos() || y.n.is_neg() {
        return false;
    }
    &(mu * mu) * &y.tangent_norm_sq() <= (&y.n * &y.n)
}

/// Exact membership test in the forbidden linearized box `{ |t1|, |t2| <= mu * n, n >= 0 }`.
#[must_use]
pub fn in_friction_box_exact(x: &Cone3, mu: &Rat) -> bool {
    if !mu.is_pos() || x.n.is_neg() {
        return false;
    }
    let lim = mu * &x.n;
    x.t[0].abs() <= lim && x.t[1].abs() <= lim
}

/// Exact rational Euclidean projection of `x = (x_n, x_t)` onto `K_mu` when `||x_t||` is rational
/// (including all interior, polar-cone, and Pythagorean points); returns `None` if `||x_t||` is
/// irrational in the boundary region (use [`project_friction_cone_f64`] in that case).
#[must_use]
pub fn project_friction_cone_rational(x: &Cone3, mu: &Rat) -> Option<Cone3> {
    if !mu.is_pos() {
        return None;
    }
    if in_friction_cone_exact(x, mu) {
        return Some(x.clone());
    }
    let r_sq = x.tangent_norm_sq();
    // Polar cone -K_{1/mu}: x_n <= -mu * ||x_t||  <=>  x_n <= 0 && x_n^2 >= mu^2 * ||x_t||^2
    if !x.n.is_pos() && &(&x.n * &x.n) >= &(&(mu * mu) * &r_sq) {
        return Some(Cone3::zero());
    }
    let norm_t = r_sq.exact_sqrt()?;
    let denom = &Rat::one() + &(mu * mu);
    let gamma_n = &(&x.n + &(mu * &norm_t)) / &denom;
    let scale = &(mu * &gamma_n) / &norm_t;
    Some(Cone3::new(gamma_n, &scale * &x.t[0], &scale * &x.t[1]))
}

/// Floating-point Euclidean projection of `(x_n, [x_t1, x_t2])` onto `K_mu` (`mu > 0`).
#[must_use]
pub fn project_friction_cone_f64(x_n: f64, x_t: [f64; 2], mu: f64) -> (f64, [f64; 2]) {
    let norm_t = x_t[0].hypot(x_t[1]);
    if x_n >= 0.0 && norm_t <= mu * x_n {
        return (x_n, x_t);
    }
    if x_n <= -mu * norm_t {
        return (0.0, [0.0, 0.0]);
    }
    let gamma_n = (x_n + mu * norm_t) / (1.0 + mu * mu);
    let scale = (mu * gamma_n) / norm_t;
    (gamma_n, [scale * x_t[0], scale * x_t[1]])
}

// =========================================================================
// Row 2: Spin-factor Jordan product (exact over Q)
// =========================================================================

/// Element `(x0, xbar)` of the spin-factor Jordan algebra `R x R^N` over `Q`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpinFactor<const N: usize> {
    pub x0: Rat,
    pub xbar: [Rat; N],
}

impl<const N: usize> SpinFactor<N> {
    #[must_use]
    pub const fn new(x0: Rat, xbar: [Rat; N]) -> Self {
        Self { x0, xbar }
    }

    #[must_use]
    pub fn bar_norm_sq(&self) -> Rat {
        let mut acc = Rat::zero();
        for c in &self.xbar {
            acc = &acc + &(c * c);
        }
        acc
    }

    #[must_use]
    pub fn bar_dot(&self, rhs: &Self) -> Rat {
        let mut acc = Rat::zero();
        for i in 0..N {
            acc = &acc + &(&self.xbar[i] * &rhs.xbar[i]);
        }
        acc
    }
}

/// Spin-factor Jordan product `(x0, xbar) o (y0, ybar) = (x0*y0 + xbar.ybar, x0*ybar + y0*xbar)`.
#[must_use]
pub fn jordan_product<const N: usize>(x: &SpinFactor<N>, y: &SpinFactor<N>) -> SpinFactor<N> {
    let z0 = &(&x.x0 * &y.x0) + &x.bar_dot(y);
    let zbar: [Rat; N] = std::array::from_fn(|i| &(&x.x0 * &y.xbar[i]) + &(&y.x0 * &x.xbar[i]));
    SpinFactor::new(z0, zbar)
}

/// Jordan square `x^2 = x o x = (x0^2 + ||xbar||^2, 2*x0*xbar)`.
#[must_use]
pub fn jordan_square<const N: usize>(x: &SpinFactor<N>) -> SpinFactor<N> {
    jordan_product(x, x)
}

/// Exact membership in the second-order cone `Q^{N+1} = { x0 >= ||xbar|| }`.
#[must_use]
pub fn jordan_in_cone_exact<const N: usize>(x: &SpinFactor<N>) -> bool {
    !x.x0.is_neg() && x.bar_norm_sq() <= (&x.x0 * &x.x0)
}

/// Exact rational spectral eigenvalues `(lambda_min, lambda_max) = (x0 - ||xbar||, x0 + ||xbar||)`
/// when `||xbar||` is rational; returns `None` if `||xbar||` is irrational.
#[must_use]
pub fn jordan_eigenvalues_exact<const N: usize>(x: &SpinFactor<N>) -> Option<(Rat, Rat)> {
    let norm = x.bar_norm_sq().exact_sqrt()?;
    Some((&x.x0 - &norm, &x.x0 + &norm))
}

// =========================================================================
// Row 3: Fast-tier contact problem (exact over Q)
// =========================================================================

/// Closed-form minimizer of the single-contact quadratic fast-tier problem
/// `min_v 1/2 (v - v*)^T A (v - v*) + 1/2 k (J.v)^2`:
/// `v = (A + k * J^T J)^(-1) * A * v*`.
#[must_use]
pub fn fast_tier_quadratic_minimizer(
    a: &Mat3,
    v_star: &Vec3,
    j_row: &Vec3,
    k: &Rat,
) -> Option<Vec3> {
    let hess = mat3_add(a, &mat3_scale(k, &outer3(j_row, j_row)));
    let h_inv = mat3_inv(&hess)?;
    let rhs = mat3_vec(a, v_star);
    Some(mat3_vec(&h_inv, &rhs))
}

/// Gradient `grad f(v) = A (v - v*) + k (J.v) J` of the single-contact quadratic objective.
#[must_use]
pub fn fast_tier_quadratic_gradient(
    a: &Mat3,
    v_star: &Vec3,
    j_row: &Vec3,
    k: &Rat,
    v: &Vec3,
) -> Vec3 {
    let dv = vec3_sub(v, v_star);
    let a_dv = mat3_vec(a, &dv);
    let y = dot3(j_row, v);
    let penalty_grad = vec3_scale(&(k * &y), j_row);
    vec3_add(&a_dv, &penalty_grad)
}

/// Contact impulse `gamma = -grad l(J.v) = -k (J.v)`.
#[must_use]
pub fn fast_tier_quadratic_impulse(j_row: &Vec3, k: &Rat, v: &Vec3) -> Rat {
    -&(k * &dot3(j_row, v))
}

// =========================================================================
// Row 4: Four-dimensional friction cone
// =========================================================================

/// Exact 4D friction force `(lambda_n, lambda_t1, lambda_t2, lambda_s)` over `Q`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cone4Force {
    pub n: Rat,
    pub t1: Rat,
    pub t2: Rat,
    pub spin: Rat,
}

/// Exact 4D contact velocity `(v_n, v_t1, v_t2, omega_n)` over `Q`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cone4Vel {
    pub n: Rat,
    pub t1: Rat,
    pub t2: Rat,
    pub omega_n: Rat,
}

/// Exact pairing `<lambda, v> = lambda_n*v_n + lambda_t1*v_t1 + lambda_t2*v_t2 + lambda_s*omega_n`.
#[must_use]
pub fn cone4d_pairing(f: &Cone4Force, v: &Cone4Vel) -> Rat {
    &(&(&f.n * &v.n) + &(&f.t1 * &v.t1)) + &(&(&f.t2 * &v.t2) + &(&f.spin * &v.omega_n))
}

/// Exact membership `||(lambda_t1, lambda_t2, lambda_s / r)|| <= mu * lambda_n`, `lambda_n >= 0`.
#[must_use]
pub fn in_cone4d_exact(f: &Cone4Force, mu: &Rat, eff_radius: &Rat) -> bool {
    if !mu.is_pos() || !eff_radius.is_pos() || f.n.is_neg() {
        return false;
    }
    let s_scaled = &f.spin / eff_radius;
    let lhs = &(&(&f.t1 * &f.t1) + &(&f.t2 * &f.t2)) + &(&s_scaled * &s_scaled);
    let rhs = &(mu * &f.n) * &(mu * &f.n);
    lhs <= rhs
}

/// Exact dual membership `mu * ||(v_t1, v_t2, r * omega_n)|| <= v_n`, `v_n >= 0`.
#[must_use]
pub fn in_dual_cone4d_exact(v: &Cone4Vel, mu: &Rat, eff_radius: &Rat) -> bool {
    if !mu.is_pos() || !eff_radius.is_pos() || v.n.is_neg() {
        return false;
    }
    let rw = eff_radius * &v.omega_n;
    let norm_sq = &(&(&v.t1 * &v.t1) + &(&v.t2 * &v.t2)) + &(&rw * &rw);
    &(&(mu * mu) * &norm_sq) <= &(&v.n * &v.n)
}

/// Exact uniform-pressure effective patch radius `r = (2/3) * R`.
#[must_use]
pub fn cone4d_uniform_radius_exact(disc_radius: &Rat) -> Rat {
    &r(2, 3) * disc_radius
}

/// Exact pure-spin limiting moment `tau_max = mu * N * (2/3) * R` for uniform pressure on a disc.
#[must_use]
pub fn cone4d_pure_spin_moment_exact(mu: &Rat, normal_load: &Rat, disc_radius: &Rat) -> Rat {
    &(mu * normal_load) * &cone4d_uniform_radius_exact(disc_radius)
}

/// Hertzian contact effective patch radius `r = (3 * pi / 16) * R` (units: m).
#[must_use]
pub fn cone4d_hertz_radius_f64(disc_radius_m: f64) -> f64 {
    (3.0 * PI / 16.0) * disc_radius_m
}

/// Dimensionless coupled tangential friction fraction `|lambda_t| / (mu * lambda_n)`
/// under the 4D ellipsoidal maximum-dissipation law for sliding speed `v_t >= 0`
/// and spin speed `omega_n >= 0` with effective radius `eff_radius > 0`.
///
/// At `v_t = r * omega_n > 0`, returns `1 / sqrt(2)`.
#[must_use]
pub fn cone4d_coupled_tangential_fraction(v_t: f64, omega_n: f64, eff_radius: f64) -> f64 {
    let denom = v_t.hypot(eff_radius * omega_n);
    if denom == 0.0 { 0.0 } else { v_t.abs() / denom }
}

/// Dimensionless coupled spin-moment fraction `|lambda_s| / (mu * lambda_n * r)`.
#[must_use]
pub fn cone4d_coupled_spin_fraction(v_t: f64, omega_n: f64, eff_radius: f64) -> f64 {
    let rw = (eff_radius * omega_n).abs();
    let denom = v_t.hypot(rw);
    if denom == 0.0 { 0.0 } else { rw / denom }
}

// =========================================================================
// Row 5: Joint-level rows (stiction, backlash, hard stop; exact over Q)
// =========================================================================

/// Exact breakaway time `t = tau_s / k` under a linear torque ramp `tau(t) = k * t`
/// (`tau_s > 0`, `k > 0`).
#[must_use]
pub fn stiction_breakaway_time(tau_s: &Rat, ramp_rate_k: &Rat) -> Option<Rat> {
    if !tau_s.is_pos() || !ramp_rate_k.is_pos() {
        return None;
    }
    Some(tau_s / ramp_rate_k)
}

/// Check stiction multiplier admissibility `lambda in [-tau_s, +tau_s]`.
#[must_use]
pub fn stiction_multiplier_valid(lambda: &Rat, tau_s: &Rat) -> bool {
    !tau_s.is_neg() && lambda.abs() <= *tau_s
}

/// Quasi-static single-step update of output coordinate `q_out` under two-unilateral backlash
/// `-beta/2 <= q_out - q_motor / gear_ratio <= +beta/2` with output held by friction unless
/// pushed by a unilateral contact face.
#[must_use]
pub fn backlash_step(q_out_prev: &Rat, q_motor_reflected: &Rat, beta: &Rat) -> Rat {
    let half = beta / &ri(2);
    let lo = q_motor_reflected - &half;
    let hi = q_motor_reflected + &half;
    if *q_out_prev < lo {
        lo
    } else if *q_out_prev > hi {
        hi
    } else {
        q_out_prev.clone()
    }
}

/// Verify complementarity of the two unilateral backlash rows:
/// `g_upper = beta/2 - (q_out - q_m_ref) >= 0`, `lambda_upper >= 0`, `lambda_upper * g_upper == 0`,
/// `g_lower = (q_out - q_m_ref) + beta/2 >= 0`, `lambda_lower >= 0`, `lambda_lower * g_lower == 0`.
#[must_use]
pub fn backlash_complementarity_holds(
    q_out: &Rat,
    q_motor_reflected: &Rat,
    beta: &Rat,
    lambda_upper: &Rat,
    lambda_lower: &Rat,
) -> bool {
    let half = beta / &ri(2);
    let diff = q_out - q_motor_reflected;
    let g_upper = &half - &diff;
    let g_lower = &diff + &half;
    !g_upper.is_neg()
        && !g_lower.is_neg()
        && !lambda_upper.is_neg()
        && !lambda_lower.is_neg()
        && (lambda_upper * &g_upper).is_zero()
        && (lambda_lower * &g_lower).is_zero()
}

/// Verify unilateral hard-stop complementarity `gap = q_limit - q >= 0`, `lambda >= 0`, `lambda * gap == 0`.
#[must_use]
pub fn hard_stop_complementarity_holds(q: &Rat, q_limit: &Rat, lambda: &Rat) -> bool {
    let gap = q_limit - q;
    !gap.is_neg() && !lambda.is_neg() && (lambda * &gap).is_zero()
}

// =========================================================================
// Row 6: Growth vector and cost
// =========================================================================

/// Canonical planar mobile-base kinematic families in Appendix D.3.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VehicleKinematics {
    DifferentialDrive,
    CarLike,
    CarWithTrailer,
    Omnidirectional,
}

/// Exact Lie-bracket growth vector at a regular point for each kinematic family.
#[must_use]
pub const fn growth_vector(kind: VehicleKinematics) -> &'static [usize] {
    match kind {
        VehicleKinematics::DifferentialDrive => &[2, 3],
        VehicleKinematics::CarLike => &[2, 3, 4],
        VehicleKinematics::CarWithTrailer => &[2, 3, 4, 5],
        VehicleKinematics::Omnidirectional => &[3],
    }
}

/// Leading-order wheel-travel cost `4 * sqrt(d * b)` (m) for a differential-drive base
/// with half-track `b > 0` (m) to execute a lateral shift `d >= 0` (m).
#[must_use]
pub fn diff_drive_lateral_shift_cost(lateral_shift_m: f64, half_track_m: f64) -> f64 {
    4.0 * (lateral_shift_m * half_track_m).sqrt()
}

/// Sub-Riemannian asymptotic path-length order `eps^(1/w)` for displacement `eps >= 0`
/// along a direction first reached at Lie-bracket depth `w >= 1`.
#[must_use]
pub fn bracket_depth_cost_order(eps: f64, depth_w: u32) -> f64 {
    assert!(depth_w >= 1, "bracket depth must be >= 1");
    eps.powf(1.0 / f64::from(depth_w))
}

// =========================================================================
// Row 7: Squeeze film (`compendium-only`, `Estimated`, §18.8)
// =========================================================================

/// Closed-form Stefan squeeze-film result (evidence class `Estimated`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SqueezeFilmResult {
    pub thickness_m: f64,
    pub dh_dt_mps: f64,
    pub evidence_class: &'static str,
}

/// Stefan's ODE rate `dh/dt = -2 * N * h^3 / (3 * pi * eta * a^4)` (m/s).
#[must_use]
pub fn squeeze_film_dh_dt(h_m: f64, load_n: f64, eta_pas: f64, radius_a_m: f64) -> f64 {
    -(2.0 * load_n * h_m.powi(3)) / (3.0 * PI * eta_pas * radius_a_m.powi(4))
}

/// Closed-form Stefan squeeze-film thickness at time `t >= 0` from initial thickness `h0 > 0`:
/// `1/h^2 = 1/h0^2 + 4 * N * t / (3 * pi * eta * a^4)`.
#[must_use]
pub fn squeeze_film_thickness(
    h0_m: f64,
    load_n: f64,
    eta_pas: f64,
    radius_a_m: f64,
    t_s: f64,
) -> SqueezeFilmResult {
    let inv_h2 =
        (1.0 / (h0_m * h0_m)) + (4.0 * load_n * t_s) / (3.0 * PI * eta_pas * radius_a_m.powi(4));
    let h = 1.0 / inv_h2.sqrt();
    let dh_dt = squeeze_film_dh_dt(h, load_n, eta_pas, radius_a_m);
    SqueezeFilmResult {
        thickness_m: h,
        dh_dt_mps: dh_dt,
        evidence_class: "Estimated",
    }
}

/// Asymptotic squeeze time from thick film (`h0 -> inf`) to thickness `h > 0`:
/// `t = 3 * pi * eta * a^4 / (4 * N * h^2)` (s).
#[must_use]
pub fn squeeze_film_time_from_thick(
    target_h_m: f64,
    load_n: f64,
    eta_pas: f64,
    radius_a_m: f64,
) -> f64 {
    (3.0 * PI * eta_pas * radius_a_m.powi(4)) / (4.0 * load_n * target_h_m * target_h_m)
}

/// Hersey regime indicator `Pi = eta * v / (p * a)` where `p = N / (pi * a^2)` (dimensionless).
#[must_use]
pub fn hersey_number(eta_pas: f64, slide_speed_mps: f64, load_n: f64, radius_a_m: f64) -> f64 {
    let pressure_pa = load_n / (PI * radius_a_m * radius_a_m);
    (eta_pas * slide_speed_mps) / (pressure_pa * radius_a_m)
}

/// Effective wet-floor friction evaluation with asperity blend `psi(h/sigma)` and calibration flag.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SqueezeFilmFriction {
    pub mu_eff: f64,
    pub psi: f64,
    pub hersey: f64,
    pub extrapolated: bool,
    pub evidence_class: &'static str,
}

/// Compute `mu_eff = psi * mu_b + (1 - psi) * eta * v * pi * a^2 / (h * N)` (`psi = 1` for
/// `h <= sigma`, `psi = 0` for `h >= 3*sigma`, monotone linear blend between; dry limit `h == 0`
/// returns `mu_d`). Flags `extrapolated` when Hersey number falls outside `[hersey_min, hersey_max]`.
#[must_use]
#[allow(clippy::too_many_arguments)]
pub fn squeeze_film_mu_eff(
    h_m: f64,
    sigma_m: f64,
    mu_b: f64,
    mu_d: f64,
    eta_pas: f64,
    slide_speed_mps: f64,
    load_n: f64,
    radius_a_m: f64,
    hersey_cal_range: (f64, f64),
) -> SqueezeFilmFriction {
    let hersey = hersey_number(eta_pas, slide_speed_mps, load_n, radius_a_m);
    if h_m == 0.0 {
        return SqueezeFilmFriction {
            mu_eff: mu_d,
            psi: 1.0,
            hersey,
            extrapolated: false,
            evidence_class: "Estimated",
        };
    }
    let psi = if h_m <= sigma_m {
        1.0
    } else if h_m >= 3.0 * sigma_m {
        0.0
    } else {
        (3.0 * sigma_m - h_m) / (2.0 * sigma_m)
    };
    let couette = (eta_pas * slide_speed_mps * PI * radius_a_m * radius_a_m) / (h_m * load_n);
    let mu_eff = psi * mu_b + (1.0 - psi) * couette;
    let extrapolated = hersey < hersey_cal_range.0 || hersey > hersey_cal_range.1;
    SqueezeFilmFriction {
        mu_eff,
        psi,
        hersey,
        extrapolated,
        evidence_class: "Estimated",
    }
}

// =========================================================================
// Row 8: Pressure-sinkage and shear (`compendium-only`, `Estimated`, §18.10)
// =========================================================================

/// Exact symbolic SI dimension vector `M^m * L^l * T^t` over `Q` (handles fractional soil exponents `n`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DimVec {
    pub mass: Rat,
    pub length: Rat,
    pub time: Rat,
}

impl DimVec {
    #[must_use]
    pub const fn new(mass: Rat, length: Rat, time: Rat) -> Self {
        Self { mass, length, time }
    }

    #[must_use]
    pub fn dimensionless() -> Self {
        Self::new(Rat::zero(), Rat::zero(), Rat::zero())
    }

    #[must_use]
    pub fn length_m() -> Self {
        Self::new(Rat::zero(), Rat::one(), Rat::zero())
    }

    #[must_use]
    pub fn pascal() -> Self {
        Self::new(Rat::one(), ri(-1), ri(-2))
    }

    #[must_use]
    pub fn mul(&self, rhs: &Self) -> Self {
        Self::new(
            &self.mass + &rhs.mass,
            &self.length + &rhs.length,
            &self.time + &rhs.time,
        )
    }

    #[must_use]
    pub fn div(&self, rhs: &Self) -> Self {
        Self::new(
            &self.mass - &rhs.mass,
            &self.length - &rhs.length,
            &self.time - &rhs.time,
        )
    }

    #[must_use]
    pub fn pow_rat(&self, exp: &Rat) -> Self {
        Self::new(&self.mass * exp, &self.length * exp, &self.time * exp)
    }
}

/// Declared symbolic dimensions of a Bekker–Wong / Janosi–Hanamoto soil card.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SoilCardDimensions {
    pub n_exponent: Rat,
    pub k_c: DimVec,
    pub k_phi: DimVec,
    pub width_b: DimVec,
    pub sinkage_z: DimVec,
    pub cohesion_c: DimVec,
    pub normal_stress_sigma: DimVec,
    pub friction_angle_phi: DimVec,
    pub shear_disp_j: DimVec,
    pub shear_modulus_k: DimVec,
}

impl SoilCardDimensions {
    /// Canonical dimensionally homogeneous SI dimensions for exponent `n`:
    /// `k_c`: `Pa * m^(1 - n)`, `k_phi`: `Pa * m^(-n)`, `b, z, j, K`: `m`, `c, sigma`: `Pa`, `phi`: `1`.
    #[must_use]
    pub fn canonical_si(n_exponent: Rat) -> Self {
        let pa = DimVec::pascal();
        let len = DimVec::length_m();
        let neg_n = -&n_exponent;
        let one_minus_n = &Rat::one() - &n_exponent;
        Self {
            n_exponent,
            k_c: pa.mul(&len.pow_rat(&one_minus_n)),
            k_phi: pa.mul(&len.pow_rat(&neg_n)),
            width_b: len.clone(),
            sinkage_z: len.clone(),
            cohesion_c: pa.clone(),
            normal_stress_sigma: pa,
            friction_angle_phi: DimVec::dimensionless(),
            shear_disp_j: len.clone(),
            shear_modulus_k: len,
        }
    }
}

/// Check exact dimensional homogeneity of both `p = (k_c/b + k_phi)*z^n` and
/// `tau = (c + sigma * tan(phi)) * (1 - exp(-j/K))`.
pub fn check_soil_dimensions(d: &SoilCardDimensions) -> Result<(), String> {
    let pa = DimVec::pascal();
    let one = DimVec::dimensionless();

    let kc_over_b = d.k_c.div(&d.width_b);
    if kc_over_b != d.k_phi {
        return Err(format!(
            "Bekker sum mismatch: dim(k_c/b)={kc_over_b:?} != dim(k_phi)={:?}",
            d.k_phi
        ));
    }
    let p_dim = d.k_phi.mul(&d.sinkage_z.pow_rat(&d.n_exponent));
    if p_dim != pa {
        return Err(format!(
            "Bekker pressure dimension mismatch: got {p_dim:?}, expected Pa={pa:?}"
        ));
    }
    if d.friction_angle_phi != one {
        return Err(format!(
            "friction angle phi must be dimensionless, got {:?}",
            d.friction_angle_phi
        ));
    }
    if d.cohesion_c != pa || d.normal_stress_sigma != pa {
        return Err(format!(
            "shear stress terms (c, sigma) must have dimension Pa, got ({:?}, {:?})",
            d.cohesion_c, d.normal_stress_sigma
        ));
    }
    let j_over_k = d.shear_disp_j.div(&d.shear_modulus_k);
    if j_over_k != one {
        return Err(format!(
            "Janosi-Hanamoto exponent j/K must be dimensionless, got {j_over_k:?}"
        ));
    }
    Ok(())
}

/// Semi-empirical soil evaluation output (`Estimated`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SoilScalarResult {
    pub value_pa: f64,
    pub evidence_class: &'static str,
}

/// Bekker–Wong pressure-sinkage formula `p = (k_c / b + k_phi) * z^n` (Pa; `Estimated`).
#[must_use]
pub fn bekker_pressure(
    k_c: f64,
    k_phi: f64,
    width_b_m: f64,
    n: f64,
    sinkage_z_m: f64,
) -> SoilScalarResult {
    let value_pa = (k_c / width_b_m + k_phi) * sinkage_z_m.powf(n);
    SoilScalarResult {
        value_pa,
        evidence_class: "Estimated",
    }
}

/// Janosi–Hanamoto shear stress `tau = (c + sigma * tan(phi)) * (1 - exp(-j / K))` (Pa; `Estimated`).
#[must_use]
pub fn janosi_hanamoto_shear(
    cohesion_pa: f64,
    normal_stress_pa: f64,
    phi_rad: f64,
    shear_disp_j_m: f64,
    shear_modulus_k_m: f64,
) -> SoilScalarResult {
    let tau_max = cohesion_pa + normal_stress_pa * phi_rad.tan();
    let value_pa = tau_max * (1.0 - (-shear_disp_j_m / shear_modulus_k_m).exp());
    SoilScalarResult {
        value_pa,
        evidence_class: "Estimated",
    }
}

// =========================================================================
// Row 9: Swept right-angle corner
// =========================================================================

/// Exact closed-form maximum length `L_max = 2 * sqrt(2) * a` (m) of a thin rod turning
/// a right-angle corner between two corridors of width `a > 0` (m).
#[must_use]
pub fn right_angle_corner_max_rod(corridor_width_a_m: f64) -> f64 {
    2.0 * SQRT_2 * corridor_width_a_m
}

/// Exact rational clearance predicate `L <= 2 * sqrt(2) * a  <=>  L^2 <= 8 * a^2` (`L >= 0, a > 0`).
#[must_use]
pub fn rod_clears_corner_exact(rod_length: &Rat, corridor_width: &Rat) -> bool {
    if rod_length.is_neg() || !corridor_width.is_pos() {
        return false;
    }
    &(rod_length * rod_length) <= &(&ri(8) * &(corridor_width * corridor_width))
}

/// Envelope function `f(theta) = a / sin(theta) + a / cos(theta)` for `theta in (0, pi/2)`.
/// Minimum is `2 * sqrt(2) * a`, uniquely attained at `theta = pi/4`.
#[must_use]
pub fn corner_rod_envelope_at_angle(corridor_width_a_m: f64, theta_rad: f64) -> f64 {
    corridor_width_a_m / theta_rad.sin() + corridor_width_a_m / theta_rad.cos()
}

// =========================================================================
// Row 10: Clearance sandwich (§19.11)
// =========================================================================

/// Verdict of the clearance sandwich `C_in <= C_free <= C_out` for a given margin `w`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClearanceVerdict {
    Feasible,
    Unknown,
    Impossible,
}

/// Exact rational heading half-bin in degrees: `360 / (2 * n_bins) = 180 / n_bins`.
/// For `n_bins = 48`, returns exact `15/4 = 3.75` degrees.
#[must_use]
pub fn heading_half_bin_deg_exact(n_heading_bins: u32) -> Rat {
    assert!(n_heading_bins > 0, "heading bins must be positive");
    r(180, i64::from(n_heading_bins))
}

/// Cell enclosure half-band `e = (Delta/2) * sqrt(2) + rho * (delta/2) + eps_field + u_reg` (m),
/// where `delta = 2*pi / n_heading_bins` (§19.11).
#[must_use]
pub fn clearance_band(
    cell_delta_m: f64,
    ref_radius_rho_m: f64,
    n_heading_bins: u32,
    eps_field_m: f64,
    u_reg_m: f64,
) -> f64 {
    let half_delta_rad = PI / f64::from(n_heading_bins);
    0.5 * cell_delta_m * SQRT_2 + ref_radius_rho_m * half_delta_rad + eps_field_m + u_reg_m
}

/// Output of the synthetic doorway clearance sandwich oracle.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DoorwaySandwichReport {
    pub band_m: f64,
    pub inner_critical_margin_m: f64,
    pub outer_critical_margin_m: f64,
    pub inner_critical_width_m: f64,
    pub outer_critical_width_m: f64,
    pub half_bin_deg: f64,
}

fn doorway_point_clearance(x: f64, y: f64, doorway_width_d: f64) -> f64 {
    let half_d = 0.5 * doorway_width_d;
    let ay = y.abs();
    if ay <= half_d {
        x.hypot(half_d - ay)
    } else {
        x.abs()
    }
}

fn grid_has_path(
    nx_half: i32,
    ny_half: i32,
    cell_delta: f64,
    doorway_width_d: f64,
    band_offset: f64,
    margin_w: f64,
    block_center_cell: bool,
) -> bool {
    let cols = (2 * nx_half + 1) as usize;
    let rows = (2 * ny_half + 1) as usize;
    let idx = |ix: i32, iy: i32| -> usize {
        ((ix + nx_half) as usize) * rows + ((iy + ny_half) as usize)
    };
    let cell_ok = |ix: i32, iy: i32| -> bool {
        let c = if block_center_cell && ix == 0 && iy == 0 {
            0.0
        } else {
            doorway_point_clearance(
                f64::from(ix) * cell_delta,
                f64::from(iy) * cell_delta,
                doorway_width_d,
            )
        };
        c + band_offset > margin_w
    };

    if !cell_ok(-nx_half, 0) || !cell_ok(nx_half, 0) {
        return false;
    }
    let mut visited = vec![false; cols * rows];
    let mut q = VecDeque::new();
    visited[idx(-nx_half, 0)] = true;
    q.push_back((-nx_half, 0));

    while let Some((cx, cy)) = q.pop_front() {
        if cx == nx_half && cy == 0 {
            return true;
        }
        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let nx = cx + dx;
            let ny = cy + dy;
            if nx < -nx_half || nx > nx_half || ny < -ny_half || ny > ny_half {
                continue;
            }
            let id = idx(nx, ny);
            if !visited[id] && cell_ok(nx, ny) {
                visited[id] = true;
                q.push_back((nx, ny));
            }
        }
    }
    false
}

/// Evaluate the clearance sandwich `C_in <= C_free <= C_out` for margin `margin_w_m` on a
/// synthetic doorway of opening width `doorway_width_d_m` with grid cell size `cell_delta_m`.
/// Setting `block_center_cell = true` closes the bottleneck cell by one cell (negative control).
#[must_use]
#[allow(clippy::too_many_arguments)]
pub fn doorway_verdict(
    doorway_width_d_m: f64,
    cell_delta_m: f64,
    ref_radius_rho_m: f64,
    n_heading_bins: u32,
    eps_field_m: f64,
    u_reg_m: f64,
    margin_w_m: f64,
    block_center_cell: bool,
) -> ClearanceVerdict {
    let band = clearance_band(
        cell_delta_m,
        ref_radius_rho_m,
        n_heading_bins,
        eps_field_m,
        u_reg_m,
    );
    let nx_half = 6;
    let ny_half = ((doorway_width_d_m / cell_delta_m).ceil() as i32).max(4) + 2;

    let in_path = grid_has_path(
        nx_half,
        ny_half,
        cell_delta_m,
        doorway_width_d_m,
        -band,
        margin_w_m,
        block_center_cell,
    );
    if in_path {
        return ClearanceVerdict::Feasible;
    }
    let out_path = grid_has_path(
        nx_half,
        ny_half,
        cell_delta_m,
        doorway_width_d_m,
        band,
        margin_w_m,
        block_center_cell,
    );
    if out_path {
        ClearanceVerdict::Unknown
    } else {
        ClearanceVerdict::Impossible
    }
}

/// Compute the inner and outer critical margins/widths of the clearance sandwich on a synthetic
/// doorway of opening width `doorway_width_d_m`, using exact max-min path bottlenecks over the
/// cubical cell graph.
#[must_use]
pub fn clearance_sandwich_doorway(
    doorway_width_d_m: f64,
    cell_delta_m: f64,
    ref_radius_rho_m: f64,
    n_heading_bins: u32,
    eps_field_m: f64,
    u_reg_m: f64,
) -> DoorwaySandwichReport {
    let band = clearance_band(
        cell_delta_m,
        ref_radius_rho_m,
        n_heading_bins,
        eps_field_m,
        u_reg_m,
    );
    let nx_half = 6;
    let ny_half = ((doorway_width_d_m / cell_delta_m).ceil() as i32).max(4) + 2;

    // Collect distinct cell clearances along the wall/corridor and find the exact max-min bottleneck.
    let mut cands = Vec::new();
    for ix in -nx_half..=nx_half {
        for iy in -ny_half..=ny_half {
            cands.push(doorway_point_clearance(
                f64::from(ix) * cell_delta_m,
                f64::from(iy) * cell_delta_m,
                doorway_width_d_m,
            ));
        }
    }
    cands.sort_by(|a, b| a.total_cmp(b));
    cands.dedup_by(|a, b| (*a - *b).abs() < 1e-15);

    // Largest candidate c* such that a path exists with every cell clearance >= c* (i.e. > c* - eps).
    let mut bottleneck_c = 0.0_f64;
    for &cand in cands.iter().rev() {
        if grid_has_path(
            nx_half,
            ny_half,
            cell_delta_m,
            doorway_width_d_m,
            0.0,
            cand - 1e-12,
            false,
        ) {
            bottleneck_c = cand;
            break;
        }
    }

    let inner_margin = bottleneck_c - band;
    let outer_margin = bottleneck_c + band;
    DoorwaySandwichReport {
        band_m: band,
        inner_critical_margin_m: inner_margin,
        outer_critical_margin_m: outer_margin,
        inner_critical_width_m: 2.0 * inner_margin,
        outer_critical_width_m: 2.0 * outer_margin,
        half_bin_deg: 180.0 / f64::from(n_heading_bins),
    }
}
