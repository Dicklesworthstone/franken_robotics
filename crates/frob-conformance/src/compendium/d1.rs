//! Appendix D.1 exact oracle library: groups, frames, and inertia (`N12-41`).
//!
//! Conventions (§11.1–§11.6):
//! - `Pose = (R, p) = a_from_b` acts as `x_a = R·x_b + p`.
//! - `Twist = (ω, v)` (angular part first, body-frame by default).
//! - `Wrench = (τ, f)` (torque first).
//! - `Momentum = (π, ℓ)` (angular momentum first).
//! - `ad*_ξ = ad_ξᵀ` (no minus sign); `Ad_g⁻ᵀ` moves wrenches; `μ̇ = ad*_ξ μ + w`.
//! - Exact rational arithmetic everywhere the statement is rational; rotations
//!   are constructed from rational unit quaternions or Cayley parameters (no `sqrt`).
//! - Tolerance rows (`cayley_angle`, `constant_twist_chord`) declare explicit SI units.

#[cfg(all(feature = "plant-coadjoint-minus", not(debug_assertions)))]
compile_error!("plant-coadjoint-minus is a test-only mutation feature forbidden in release builds");

use crate::compendium::rat::{
    Mat3, Mat4, Mat6, Rat, Vec3, Vec6, cross3, dot3, hat3, mat3_add, mat3_det, mat3_diag, mat3_eye,
    mat3_inv, mat3_mul, mat3_scale, mat3_sub, mat3_trace, mat3_transpose, mat3_vec, mat3_zero,
    mat4_is_spd, mat4_sub, mat4_zero, mat6_from_blocks, mat6_inv, mat6_mul, mat6_transpose,
    mat6_vec, outer3, r, ri, vec3_add, vec3_neg, vec3_scale, vec3_sub, vec3_zero,
};

// ---- Core spatial types (exact over Q) ----

/// Rigid transform `g = (R, p) = a_from_b` acting as `x_a = R·x_b + p`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pose {
    pub r: Mat3,
    pub p: Vec3,
}

impl Pose {
    #[must_use]
    pub fn identity() -> Self {
        Self {
            r: mat3_eye(),
            p: vec3_zero(),
        }
    }

    #[must_use]
    pub fn new(r: Mat3, p: Vec3) -> Self {
        Self { r, p }
    }

    /// Compose `a_from_b * b_from_c = a_from_c`.
    #[must_use]
    pub fn compose(&self, rhs: &Self) -> Self {
        Self {
            r: mat3_mul(&self.r, &rhs.r),
            p: vec3_add(&mat3_vec(&self.r, &rhs.p), &self.p),
        }
    }

    /// Exact inverse `b_from_a = (Rᵀ, -Rᵀ·p)`.
    #[must_use]
    pub fn inverse(&self) -> Self {
        let rt = mat3_transpose(&self.r);
        let p_inv = vec3_neg(&mat3_vec(&rt, &self.p));
        Self { r: rt, p: p_inv }
    }

    /// Action on a point: `x_a = R·x_b + p`.
    #[must_use]
    pub fn act_point(&self, x: &Vec3) -> Vec3 {
        vec3_add(&mat3_vec(&self.r, x), &self.p)
    }
}

/// Exact rational rotation matrix from a non-zero quaternion `(w, x, y, z)` (no square roots).
///
/// Guarantees `RᵀR = I` and `det(R) = 1` identically in `ℚ`.
#[must_use]
pub fn rotation_from_rational_quat(w: Rat, x: Rat, y: Rat, z: Rat) -> Mat3 {
    let s = &(&(&w * &w) + &(&x * &x)) + &(&(&y * &y) + &(&z * &z));
    assert!(!s.is_zero(), "zero quaternion");
    let two = ri(2);
    let ww = &w * &w;
    let xx = &x * &x;
    let yy = &y * &y;
    let zz = &z * &z;
    let wx = &w * &x;
    let wy = &w * &y;
    let wz = &w * &z;
    let xy = &x * &y;
    let xz = &x * &z;
    let yz = &y * &z;
    [
        [
            &(&(&ww + &xx) - &(&yy + &zz)) / &s,
            &(&two * &(&xy - &wz)) / &s,
            &(&two * &(&xz + &wy)) / &s,
        ],
        [
            &(&two * &(&xy + &wz)) / &s,
            &(&(&ww - &xx) + &(&yy - &zz)) / &s,
            &(&two * &(&yz - &wx)) / &s,
        ],
        [
            &(&two * &(&xz - &wy)) / &s,
            &(&two * &(&yz + &wx)) / &s,
            &(&(&ww - &xx) - &(&yy - &zz)) / &s,
        ],
    ]
}

/// Spatial twist `ξ = (ω, v)`, angular part first.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Twist {
    pub omega: Vec3,
    pub v: Vec3,
}

impl Twist {
    #[must_use]
    pub const fn new(omega: Vec3, v: Vec3) -> Self {
        Self { omega, v }
    }

    #[must_use]
    pub fn to_vec6(&self) -> Vec6 {
        [
            self.omega[0].clone(),
            self.omega[1].clone(),
            self.omega[2].clone(),
            self.v[0].clone(),
            self.v[1].clone(),
            self.v[2].clone(),
        ]
    }

    #[must_use]
    pub fn from_vec6(v: &Vec6) -> Self {
        Self {
            omega: [v[0].clone(), v[1].clone(), v[2].clone()],
            v: [v[3].clone(), v[4].clone(), v[5].clone()],
        }
    }
}

/// Spatial wrench `w = (τ, f)`, torque first.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Wrench {
    pub tau: Vec3,
    pub f: Vec3,
}

impl Wrench {
    #[must_use]
    pub const fn new(tau: Vec3, f: Vec3) -> Self {
        Self { tau, f }
    }

    #[must_use]
    pub fn to_vec6(&self) -> Vec6 {
        [
            self.tau[0].clone(),
            self.tau[1].clone(),
            self.tau[2].clone(),
            self.f[0].clone(),
            self.f[1].clone(),
            self.f[2].clone(),
        ]
    }

    #[must_use]
    pub fn from_vec6(v: &Vec6) -> Self {
        Self {
            tau: [v[0].clone(), v[1].clone(), v[2].clone()],
            f: [v[3].clone(), v[4].clone(), v[5].clone()],
        }
    }
}

/// Spatial momentum `μ = (π, ℓ)`, angular momentum first.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Momentum {
    pub pi: Vec3,
    pub ell: Vec3,
}

impl Momentum {
    #[must_use]
    pub const fn new(pi: Vec3, ell: Vec3) -> Self {
        Self { pi, ell }
    }

    #[must_use]
    pub fn to_vec6(&self) -> Vec6 {
        [
            self.pi[0].clone(),
            self.pi[1].clone(),
            self.pi[2].clone(),
            self.ell[0].clone(),
            self.ell[1].clone(),
            self.ell[2].clone(),
        ]
    }

    #[must_use]
    pub fn from_vec6(v: &Vec6) -> Self {
        Self {
            pi: [v[0].clone(), v[1].clone(), v[2].clone()],
            ell: [v[3].clone(), v[4].clone(), v[5].clone()],
        }
    }
}

// ---- Row 1: Bracket on se(3) ----

/// Lie bracket on `𝔰𝔢(3)`: `[ξ₁, ξ₂] = (ω₁ × ω₂, ω₁ × v₂ − ω₂ × v₁)`.
#[must_use]
pub fn bracket(xi1: &Twist, xi2: &Twist) -> Twist {
    Twist::new(
        cross3(&xi1.omega, &xi2.omega),
        vec3_sub(&cross3(&xi1.omega, &xi2.v), &cross3(&xi2.omega, &xi1.v)),
    )
}

// ---- Row 2: Adjoint ----

/// 6×6 adjoint matrix `Ad_g = [[R, 0], [p̂·R, R]]`.
#[must_use]
pub fn adjoint_matrix(g: &Pose) -> Mat6 {
    let pr = mat3_mul(&hat3(&g.p), &g.r);
    mat6_from_blocks(&g.r, &mat3_zero(), &pr, &g.r)
}

/// Adjoint action on a twist: `Ad_g(ω, v) = (R·ω, R·v + p × R·ω)`.
#[must_use]
pub fn adjoint_twist(g: &Pose, xi: &Twist) -> Twist {
    let rw = mat3_vec(&g.r, &xi.omega);
    let rv = mat3_vec(&g.r, &xi.v);
    Twist::new(rw.clone(), vec3_add(&rv, &cross3(&g.p, &rw)))
}

// ---- Row 3: Group coadjoint ----

/// 6×6 group coadjoint matrix `Ad_g⁻ᵀ = [[R, p̂·R], [0, R]]` (wrench pushforward).
#[must_use]
pub fn coadjoint_matrix(g: &Pose) -> Mat6 {
    let pr = mat3_mul(&hat3(&g.p), &g.r);
    mat6_from_blocks(&g.r, &pr, &mat3_zero(), &g.r)
}

/// Group coadjoint (wrench pushforward): `(τ, f) ↦ (R·τ + p × R·f, R·f)`.
#[must_use]
pub fn coadjoint_wrench(g: &Pose, w: &Wrench) -> Wrench {
    let rt = mat3_vec(&g.r, &w.tau);
    let rf = mat3_vec(&g.r, &w.f);
    Wrench::new(vec3_add(&rt, &cross3(&g.p, &rf)), rf)
}

// ---- Row 4: Algebra adjoint and coadjoint ----

/// 6×6 algebra adjoint matrix `ad_ξ = [[ω̂, 0], [v̂, ω̂]]`.
#[must_use]
pub fn ad_matrix(xi: &Twist) -> Mat6 {
    let wh = hat3(&xi.omega);
    let vh = hat3(&xi.v);
    mat6_from_blocks(&wh, &mat3_zero(), &vh, &wh)
}

/// Algebra adjoint action `ad_ξ η = [ξ, η] = (ω × η_ω, ω × η_v + v × η_ω)`.
#[must_use]
pub fn ad_twist(xi: &Twist, eta: &Twist) -> Twist {
    Twist::new(
        cross3(&xi.omega, &eta.omega),
        vec3_add(&cross3(&xi.omega, &eta.v), &cross3(&xi.v, &eta.omega)),
    )
}

/// Algebra coadjoint dual map `ad*_ξ μ = ad_ξᵀ μ = (π × ω + ℓ × v, ℓ × ω)`,
/// defined by `⟨ad*_ξ μ, η⟩ = ⟨μ, [ξ, η]⟩` (no minus sign).
#[must_use]
pub fn ad_star(xi: &Twist, mu: &Momentum) -> Momentum {
    let pi_out = vec3_add(&cross3(&mu.pi, &xi.omega), &cross3(&mu.ell, &xi.v));
    let ell_out = cross3(&mu.ell, &xi.omega);
    #[cfg(feature = "plant-coadjoint-minus")]
    {
        Momentum::new(vec3_neg(&pi_out), vec3_neg(&ell_out))
    }
    #[cfg(not(feature = "plant-coadjoint-minus"))]
    {
        Momentum::new(pi_out, ell_out)
    }
}

// ---- Row 5: Three typed frame maps ----

/// Typed twist map `Ad_g` (moves a twist from `b` to `a`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TwistMap(Mat6);

impl TwistMap {
    #[must_use]
    pub fn from_pose(g: &Pose) -> Self {
        Self(adjoint_matrix(g))
    }

    #[must_use]
    pub fn apply(&self, xi: &Twist) -> Twist {
        Twist::from_vec6(&mat6_vec(&self.0, &xi.to_vec6()))
    }

    #[must_use]
    pub fn matrix(&self) -> &Mat6 {
        &self.0
    }
}

/// Typed wrench pushforward `Ad_g⁻ᵀ` (moves a wrench from `b` to `a`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WrenchPushforward(Mat6);

impl WrenchPushforward {
    #[must_use]
    pub fn from_pose(g: &Pose) -> Self {
        Self(coadjoint_matrix(g))
    }

    #[must_use]
    pub fn apply(&self, w: &Wrench) -> Wrench {
        Wrench::from_vec6(&mat6_vec(&self.0, &w.to_vec6()))
    }

    #[must_use]
    pub fn matrix(&self) -> &Mat6 {
        &self.0
    }
}

/// 6×6 wrench pullback matrix `Ad_gᵀ` (inverse of pushforward; moves a wrench from `a` to `b`).
#[must_use]
pub fn wrench_pullback_matrix(g: &Pose) -> Mat6 {
    mat6_transpose(&adjoint_matrix(g))
}

/// Typed wrench pullback `Ad_gᵀ` (inverse of [`WrenchPushforward`]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WrenchPullback(Mat6);

impl WrenchPullback {
    #[must_use]
    pub fn from_pose(g: &Pose) -> Self {
        Self(wrench_pullback_matrix(g))
    }

    #[must_use]
    pub fn apply(&self, w: &Wrench) -> Wrench {
        Wrench::from_vec6(&mat6_vec(&self.0, &w.to_vec6()))
    }

    #[must_use]
    pub fn matrix(&self) -> &Mat6 {
        &self.0
    }
}

// ---- Row 6: Power pairing ----

/// Power pairing `⟨w, ξ⟩ = τ·ω + f·v` (in watts when SI units are used).
#[must_use]
pub fn power_pairing(w: &Wrench, xi: &Twist) -> Rat {
    &dot3(&w.tau, &xi.omega) + &dot3(&w.f, &xi.v)
}

/// Pairing of a spatial momentum with a twist: `⟨μ, ξ⟩ = π·ω + ℓ·v`.
#[must_use]
pub fn momentum_pairing(mu: &Momentum, xi: &Twist) -> Rat {
    &dot3(&mu.pi, &xi.omega) + &dot3(&mu.ell, &xi.v)
}

// ---- Row 7: Reflection parity ----

/// Proper/improper orthogonal transform `T(P) = diag(det(P)·P, P)` on twists.
#[must_use]
pub fn parity_transform_twist(p: &Mat3, xi: &Twist) -> Twist {
    let det = mat3_det(p);
    let axial_mat = mat3_scale(&det, p);
    Twist::new(mat3_vec(&axial_mat, &xi.omega), mat3_vec(p, &xi.v))
}

/// Proper/improper orthogonal transform `T(P) = diag(det(P)·P, P)` on wrenches.
#[must_use]
pub fn parity_transform_wrench(p: &Mat3, w: &Wrench) -> Wrench {
    let det = mat3_det(p);
    let axial_mat = mat3_scale(&det, p);
    Wrench::new(mat3_vec(&axial_mat, &w.tau), mat3_vec(p, &w.f))
}

/// Axial vector image under orthogonal matrix `P`: `det(P)·P·a`.
#[must_use]
pub fn parity_axial(p: &Mat3, a: &Vec3) -> Vec3 {
    let det = mat3_det(p);
    vec3_scale(&det, &mat3_vec(p, a))
}

// ---- Row 8: Euler-Poincare, free body ----

/// Free-body Euler–Poincaré equation `μ̇ = ad*_ξ μ + w`:
/// `π̇ = π × ω + ℓ × v + τ`, `ℓ̇ = ℓ × ω + f`.
#[must_use]
pub fn euler_poincare_free_body(xi: &Twist, mu: &Momentum, w: &Wrench) -> Momentum {
    let ad = ad_star(xi, mu);
    Momentum::new(vec3_add(&ad.pi, &w.tau), vec3_add(&ad.ell, &w.f))
}

// ---- Row 9: Momentum map ----

/// Locked-inertia momentum map `μ = 𝕀·(ξ + 𝔸·ṡ)` for a general 1-DOF shape rate `s_dot`.
#[must_use]
pub fn momentum_map(
    locked_inertia: &Mat6,
    connection_col: &Vec6,
    xi: &Twist,
    s_dot: &Rat,
) -> Momentum {
    let xi_v = xi.to_vec6();
    let shifted: Vec6 = std::array::from_fn(|i| &xi_v[i] + &(&connection_col[i] * s_dot));
    Momentum::from_vec6(&mat6_vec(locked_inertia, &shifted))
}

/// Compute mechanical connection column `𝔸 = 𝕀⁻¹·𝕀_BS` from locked inertia and coupling block.
#[must_use]
pub fn mechanical_connection_col(locked_inertia: &Mat6, coupling_bs: &Vec6) -> Option<Vec6> {
    let inv = mat6_inv(locked_inertia)?;
    Some(mat6_vec(&inv, coupling_bs))
}

// ---- Row 10: Reconstruction ----

/// Reconstruction equation `ξ = −𝔸·ṡ + 𝕀⁻¹·p` for a 1-DOF shape rate `s_dot` and group momentum `p`.
#[must_use]
pub fn reconstruct_body_velocity(
    locked_inertia: &Mat6,
    connection_col: &Vec6,
    s_dot: &Rat,
    group_momentum: &Momentum,
) -> Option<Twist> {
    let inv = mat6_inv(locked_inertia)?;
    let dyn_term = mat6_vec(&inv, &group_momentum.to_vec6());
    let xi_v: Vec6 = std::array::from_fn(|i| &dyn_term[i] - &(&connection_col[i] * s_dot));
    Some(Twist::from_vec6(&xi_v))
}

// ---- Row 11: Curvature of the connection ----

/// Connection curvature on `𝔰𝔬(3)`: `(D𝔸)ᵢⱼ = ∂ᵢ𝔸ⱼ − ∂ⱼ𝔸ᵢ − [𝔸ᵢ, 𝔸ⱼ]`.
///
/// For a small loop of area `da` in the `(i, j)` plane, holonomy is `exp(-(D𝔸)ᵢⱼ · da)`.
#[must_use]
pub fn connection_curvature_so3(d_i_a_j: &Vec3, d_j_a_i: &Vec3, a_i: &Vec3, a_j: &Vec3) -> Vec3 {
    let curl = vec3_sub(d_i_a_j, d_j_a_i);
    let br = cross3(a_i, a_j);
    vec3_sub(&curl, &br)
}

// ---- Row 12: Cayley map ----

/// Exact rational `SO(3)` Cayley map: `R = I + (4/(4 + ‖ω‖²))·(ω̂ + ω̂²/2)`.
#[must_use]
pub fn cayley_so3(omega: &Vec3) -> Mat3 {
    let w_hat = hat3(omega);
    let w_hat2 = mat3_mul(&w_hat, &w_hat);
    let denom = &ri(4) + &dot3(omega, omega);
    let coeff = &ri(4) / &denom;
    let inner = mat3_add(&w_hat, &mat3_scale(&r(1, 2), &w_hat2));
    mat3_add(&mat3_eye(), &mat3_scale(&coeff, &inner))
}

/// Exact inverse `(I − ω̂/2)⁻¹ = I + (2·ω̂ + ω̂²)/(4 + ‖ω‖²)`.
#[must_use]
pub fn inv_eye_minus_half_hat(omega: &Vec3) -> Mat3 {
    let w_hat = hat3(omega);
    let w_hat2 = mat3_mul(&w_hat, &w_hat);
    let denom = &ri(4) + &dot3(omega, omega);
    let inv_d = &Rat::one() / &denom;
    let numer = mat3_add(&mat3_scale(&ri(2), &w_hat), &w_hat2);
    mat3_add(&mat3_eye(), &mat3_scale(&inv_d, &numer))
}

/// Exact rational `SE(3)` Cayley map: `R = cay_SO3(ω)`, `p = (I − ω̂/2)⁻¹·v`.
#[must_use]
pub fn cayley_se3(x: &Twist) -> Pose {
    let rot = cayley_so3(&x.omega);
    let t_mat = inv_eye_minus_half_hat(&x.omega);
    let p = mat3_vec(&t_mat, &x.v);
    Pose::new(rot, p)
}

// ---- Row 13: Cayley angle (tolerance row with explicit SI unit `rad`) ----

/// Design cap on `‖ω‖·h` per substep (`0.2 rad`, §13.9 / Appendix D.1).
pub const CAYLEY_STEP_LIMIT_RAD: f64 = 0.2;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CayleyAngleReport {
    pub angle_rad: f64,
    pub rel_error: f64,
    pub approx_rel_error: f64,
    pub tolerance_rad: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CayleyStepRefused {
    pub observed_rad: f64,
    pub limit_rad: f64,
}

/// Compute Cayley rotation angle `2·atan(s/2)` for step norm `s = ‖ω‖·h` (rad).
/// Refuses beyond `‖ω‖·h = 0.2 rad` (DESIGN).
pub fn cayley_angle(step_norm_rad: f64) -> Result<CayleyAngleReport, CayleyStepRefused> {
    if !step_norm_rad.is_finite() || step_norm_rad < 0.0 || step_norm_rad > CAYLEY_STEP_LIMIT_RAD {
        return Err(CayleyStepRefused {
            observed_rad: step_norm_rad,
            limit_rad: CAYLEY_STEP_LIMIT_RAD,
        });
    }
    let angle_rad = 2.0 * (step_norm_rad * 0.5).atan();
    let rel_error = if step_norm_rad == 0.0 {
        0.0
    } else {
        (step_norm_rad - angle_rad).abs() / step_norm_rad
    };
    let approx_rel_error = (step_norm_rad * step_norm_rad) / 12.0;
    Ok(CayleyAngleReport {
        angle_rad,
        rel_error,
        approx_rel_error,
        tolerance_rad: 1e-6,
    })
}

// ---- Row 14: Planar Cayley ----

/// Unit complex number `(c, s)` representing a planar rotation coordinate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnitComplex {
    pub c: Rat,
    pub s: Rat,
}

impl UnitComplex {
    #[must_use]
    pub const fn new(c: Rat, s: Rat) -> Self {
        Self { c, s }
    }

    #[must_use]
    pub fn norm_sq(&self) -> Rat {
        &(&self.c * &self.c) + &(&self.s * &self.s)
    }
}

/// Exact rational planar Cayley update: `z₊ = z · ((1 − a²/4) + i·a) / (1 + a²/4)`.
#[must_use]
pub fn planar_cayley_step(z: &UnitComplex, a: &Rat) -> UnitComplex {
    let a2_over_4 = &(a * a) / &ri(4);
    let denom = &Rat::one() + &a2_over_4;
    let re = &(&Rat::one() - &a2_over_4) / &denom;
    let im = a / &denom;
    UnitComplex::new(&(&z.c * &re) - &(&z.s * &im), &(&z.c * &im) + &(&z.s * &re))
}

// ---- Row 15: Inverse Cayley differential ----

/// Rotation block of left-trivialized `(Dcay_x)⁻¹`: `I + ½·ω̂ + ¼·ω·ωᵀ`.
#[must_use]
pub fn dcay_inv_so3(omega: &Vec3) -> Mat3 {
    let w_hat = hat3(omega);
    let wwt = outer3(omega, omega);
    mat3_add(
        &mat3_add(&mat3_eye(), &mat3_scale(&r(1, 2), &w_hat)),
        &mat3_scale(&r(1, 4), &wwt),
    )
}

/// Exact inverse of the rotation block: `Dcay_ω = 2/(4 + ‖ω‖²) · (2I − ω̂)`.
#[must_use]
pub fn dcay_so3(omega: &Vec3) -> Mat3 {
    let denom = &ri(4) + &dot3(omega, omega);
    let coeff = &ri(2) / &denom;
    let inner = mat3_sub(&mat3_scale(&ri(2), &mat3_eye()), &hat3(omega));
    mat3_scale(&coeff, &inner)
}

/// Left-trivialized `SE(3)` inverse Cayley differential:
/// `(Dcay_x)⁻¹ = [[I + ½·ω̂ + ¼·ω·ωᵀ, 0], [½·v̂ + ¼·ω̂·v̂, I + ½·ω̂]]`.
#[must_use]
pub fn dcay_inv_se3(x: &Twist) -> Mat6 {
    let w_hat = hat3(&x.omega);
    let v_hat = hat3(&x.v);
    let tl = dcay_inv_so3(&x.omega);
    let tr = mat3_zero();
    let bl = mat3_add(
        &mat3_scale(&r(1, 2), &v_hat),
        &mat3_scale(&r(1, 4), &mat3_mul(&w_hat, &v_hat)),
    );
    let br = mat3_add(&mat3_eye(), &mat3_scale(&r(1, 2), &w_hat));
    mat6_from_blocks(&tl, &tr, &bl, &br)
}

// ---- Row 16: Pseudo-inertia ----

/// Convert rotational inertia `Ī` about the frame origin into second-moment matrix `Σ = ½·tr(Ī)·I − Ī`.
#[must_use]
pub fn sigma_from_rotational_inertia(i_bar: &Mat3) -> Mat3 {
    let half_tr = &mat3_trace(i_bar) * &r(1, 2);
    mat3_sub(&mat3_scale(&half_tr, &mat3_eye()), i_bar)
}

/// Recover rotational inertia about the origin from `Σ`: `Ī = tr(Σ)·I − Σ`.
#[must_use]
pub fn rotational_inertia_from_sigma(sigma: &Mat3) -> Mat3 {
    let tr = mat3_trace(sigma);
    mat3_sub(&mat3_scale(&tr, &mat3_eye()), sigma)
}

/// Construct 4×4 pseudo-inertia `𝖯 = [[Σ, h], [hᵀ, m]]` from `(m, c, Ī)`.
#[must_use]
pub fn pseudo_inertia_from_rotational(mass: &Rat, com: &Vec3, i_bar: &Mat3) -> Mat4 {
    let sigma = sigma_from_rotational_inertia(i_bar);
    let h = vec3_scale(mass, com);
    let mut p = mat4_zero();
    for i in 0..3 {
        for j in 0..3 {
            p[i][j] = sigma[i][j].clone();
        }
        p[i][3] = h[i].clone();
        p[3][i] = h[i].clone();
    }
    p[3][3] = mass.clone();
    p
}

/// Physical consistency test: `𝖯 ≻ 0` (strictly positive definite over `ℚ`).
#[must_use]
pub fn pseudo_inertia_is_consistent(p: &Mat4) -> bool {
    mat4_is_spd(p)
}

// ---- Row 17: Shell rule ----

/// Exact rational pseudo-inertia of a centred solid box of side lengths `(lx, ly, lz)` and density `ρ`.
#[must_use]
pub fn solid_box_pseudo_inertia(lx: &Rat, ly: &Rat, lz: &Rat, rho: &Rat) -> Mat4 {
    let vol = &(lx * ly) * lz;
    let mass = &vol * rho;
    let twelve = ri(12);
    let sx = &(&mass * &(lx * lx)) / &twelve;
    let sy = &(&mass * &(ly * ly)) / &twelve;
    let sz = &(&mass * &(lz * lz)) / &twelve;
    let mut p = mat4_zero();
    p[0][0] = sx;
    p[1][1] = sy;
    p[2][2] = sz;
    p[3][3] = mass;
    p
}

/// Exact rational shell pseudo-inertia `𝖯_shell = 𝖯(Ω, ρ) − 𝖯(Ω_t, ρ)` for a box of wall thickness `t`.
#[must_use]
pub fn box_shell_pseudo_inertia(lx: &Rat, ly: &Rat, lz: &Rat, wall_t: &Rat, rho: &Rat) -> Mat4 {
    let two_t = &ri(2) * wall_t;
    let outer = solid_box_pseudo_inertia(lx, ly, lz, rho);
    let inner = solid_box_pseudo_inertia(&(lx - &two_t), &(ly - &two_t), &(lz - &two_t), rho);
    mat4_sub(&outer, &inner)
}

/// Exact dimensionless inertia ratio `I / (m·r²) = (2/5)·(r⁵ − rᵢ⁵) / ((r³ − rᵢ³)·r²)`
/// of a spherical shell of outer radius `r` and wall thickness `t` (`rᵢ = r − t`).
#[must_use]
pub fn spherical_shell_inertia_ratio(radius: &Rat, thickness: &Rat) -> Rat {
    let r_i = radius - thickness;
    let num = &r(2, 5) * &(&radius.powu(5) - &r_i.powu(5));
    let den = &(&radius.powu(3) - &r_i.powu(3)) * &radius.powu(2);
    &num / &den
}

// ---- Row 18: Mass-support bound ----

/// Left-hand side of the mass-support bound `‖c − o‖² + tr(I_C)/(2m)` (units `m²`).
#[must_use]
pub fn mass_support_lhs(com: &Vec3, center: &Vec3, mass: &Rat, i_com: &Mat3) -> Rat {
    let dc = vec3_sub(com, center);
    let dist_sq = dot3(&dc, &dc);
    let trace_term = &mat3_trace(i_com) / &(&ri(2) * mass);
    &dist_sq + &trace_term
}

/// Check `‖c − o‖² + tr(I_C)/(2m) ≤ ρ²` exactly over `ℚ`.
#[must_use]
pub fn mass_support_holds(
    com: &Vec3,
    center: &Vec3,
    mass: &Rat,
    i_com: &Mat3,
    radius: &Rat,
) -> bool {
    let lhs = mass_support_lhs(com, center, mass, i_com);
    lhs <= (radius * radius)
}

// ---- Row 19: Mobility count ----

/// Grübler–Kutzbach count result; explicitly flagged as a count, never true mobility.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MobilityCount {
    pub count: i64,
    pub is_count_not_mobility: bool,
}

/// Grübler–Kutzbach count `M = 6·(n − 1 − j) + Σ fᵢ` (`n` links including ground, `j` joints).
#[must_use]
pub fn mobility_count(n_links: i64, joint_dofs: &[i64]) -> MobilityCount {
    let j = joint_dofs.len() as i64;
    let sum_f: i64 = joint_dofs.iter().sum();
    MobilityCount {
        count: 6 * (n_links - 1 - j) + sum_f,
        is_count_not_mobility: true,
    }
}

// ---- Row 20: Product of exponentials ----

/// Pure revolute screw axis in the base frame (`‖ω‖² = 1`, `ω·v = 0`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RevoluteScrew {
    pub omega: Vec3,
    pub v: Vec3,
}

impl RevoluteScrew {
    /// Construct from unit axis `ω` and a point `q` on the axis (`v = −ω × q`).
    #[must_use]
    pub fn from_axis_point(omega: Vec3, point_on_axis: &Vec3) -> Self {
        let v = vec3_neg(&cross3(&omega, point_on_axis));
        Self { omega, v }
    }

    #[must_use]
    pub fn to_twist(&self) -> Twist {
        Twist::new(self.omega.clone(), self.v.clone())
    }
}

/// Exact rational `SE(3)` exponential of a pure revolute screw given `(c, s)` with `c² + s² = 1`.
///
/// Uses the polynomial Rodrigues form:
/// `R = I + s·ω̂ + (1 − c)·ω̂²`, `p = s·v + (1 − c)·(ω × v)` (since `ω·v = 0`).
#[must_use]
pub fn exp_revolute_screw(screw: &RevoluteScrew, cos_theta: &Rat, sin_theta: &Rat) -> Pose {
    assert_eq!(
        &(&(cos_theta * cos_theta) + &(sin_theta * sin_theta)),
        &Rat::one(),
        "revolute (c, s) must satisfy c^2 + s^2 = 1"
    );
    let w_hat = hat3(&screw.omega);
    let w_hat2 = mat3_mul(&w_hat, &w_hat);
    let one_minus_c = &Rat::one() - cos_theta;
    let rot = mat3_add(
        &mat3_add(&mat3_eye(), &mat3_scale(sin_theta, &w_hat)),
        &mat3_scale(&one_minus_c, &w_hat2),
    );
    let p = vec3_add(
        &vec3_scale(sin_theta, &screw.v),
        &vec3_scale(&one_minus_c, &cross3(&screw.omega, &screw.v)),
    );
    Pose::new(rot, p)
}

/// Product of exponentials `g(θ) = exp(θ₁·Ŝ₁)·…·exp(θₙ·Ŝₙ)·g₀` in exact rational `(cᵢ, sᵢ)`.
#[must_use]
pub fn poe_forward_kinematics(
    screws: &[RevoluteScrew],
    cos_sin: &[(Rat, Rat)],
    home: &Pose,
) -> Pose {
    assert_eq!(screws.len(), cos_sin.len());
    let mut acc = Pose::identity();
    for (s, (c, sn)) in screws.iter().zip(cos_sin.iter()) {
        acc = acc.compose(&exp_revolute_screw(s, c, sn));
    }
    acc.compose(home)
}

/// Space screw Jacobian columns `J_s[:, i] = Ad_{exp(θ₁Ŝ₁)…exp(θᵢ₋₁Ŝᵢ₋₁)}(Ŝᵢ)`.
#[must_use]
pub fn poe_space_jacobian(screws: &[RevoluteScrew], cos_sin: &[(Rat, Rat)]) -> Vec<Twist> {
    assert_eq!(screws.len(), cos_sin.len());
    let mut prefix = Pose::identity();
    let mut cols = Vec::with_capacity(screws.len());
    for (s, (c, sn)) in screws.iter().zip(cos_sin.iter()) {
        cols.push(adjoint_twist(&prefix, &s.to_twist()));
        prefix = prefix.compose(&exp_revolute_screw(s, c, sn));
    }
    cols
}

/// Linear velocity Jacobian column at spatial point `p` from space screw column `S = (ω, v)`:
/// `J_p[:, i] = v_i + ω_i × p`.
#[must_use]
pub fn space_jacobian_to_point_velocity(space_cols: &[Twist], point: &Vec3) -> Vec<Vec3> {
    space_cols
        .iter()
        .map(|s| vec3_add(&s.v, &cross3(&s.omega, point)))
        .collect()
}

// ---- Row 21: Constant-twist chord (tolerance row with explicit SI unit `m`) ----

/// Unnormalized cardinal sine `sinc(x) = sin(x) / x` with `sinc(0) = 1` and smooth
/// Taylor branch for `|x| < 1e-4` to eliminate `0/0` cancellation.
#[must_use]
pub fn sinc(x: f64) -> f64 {
    if x.abs() < 1e-4 {
        let x2 = x * x;
        1.0 - x2 / 6.0 + (x2 * x2) / 120.0 - (x2 * x2 * x2) / 5040.0
    } else {
        x.sin() / x
    }
}

/// Planar constant-twist chord displacement `(Δx, Δy)` in metres:
/// `Δp = v·Δt·sinc(δ/2)·(cos(θ + δ/2), sin(θ + δ/2))` with `δ = ω·Δt`.
#[must_use]
pub fn constant_twist_chord(v_mps: f64, omega_radps: f64, dt_s: f64, theta_rad: f64) -> (f64, f64) {
    let delta = omega_radps * dt_s;
    let half = 0.5 * delta;
    let scale = v_mps * dt_s * sinc(half);
    let mid = theta_rad + half;
    (scale * mid.cos(), scale * mid.sin())
}

// ---- Helper exports used in unit tests ----

#[must_use]
pub fn mat3_is_so3(r: &Mat3) -> bool {
    mat3_mul(&mat3_transpose(r), r) == mat3_eye() && mat3_det(r) == Rat::one()
}

#[must_use]
pub fn mat3_diag_int(a: i64, b: i64, c: i64) -> Mat3 {
    mat3_diag(&[ri(a), ri(b), ri(c)])
}

#[must_use]
pub fn mat3_inverse_exact(m: &Mat3) -> Option<Mat3> {
    mat3_inv(m)
}

#[must_use]
pub fn mat6_inverse_transpose(m: &Mat6) -> Option<Mat6> {
    mat6_inv(m).map(|inv| mat6_transpose(&inv))
}

#[must_use]
pub fn mat6_compose(a: &Mat6, b: &Mat6) -> Mat6 {
    mat6_mul(a, b)
}
