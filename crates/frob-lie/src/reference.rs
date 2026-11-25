//! Scalar-generic `SO(3)` and `SE(3)` reference implementation over `fs_ad::Real` (`BD-010`).
//!
//! Operates purely on fixed-size arrays (`[S; 3]`, `[S; 4]`, `[S; 6]`, `[[S; 3]; 3]`,
//! `[[S; 6]; 6]`) with zero private vector, quaternion, or pose structs (ADR-MARROW-01,
//! ADR-MARROW-04). Instantiated at `f64` to cross-check against `fs-ga`, at `fs_ad::Dual`
//! for forward-AD derivative laws, and at test-only `f32` wrappers for single-precision checks.
//!
//! Component conventions match `fs-ga` and `frob-lie` identically:
//! - Quaternion: `[w, x, y, z]`
//! - Pose: `(q, p)` with `q = [w, x, y, z]`, `p = [px, py, pz]`
//! - Twist: `[wx, wy, wz, vx, vy, vz]` (angular first)
//! - Wrench / momentum: `[tx, ty, tz, fx, fy, fz]` (torque / angular momentum first)

use fs_ad::Real;

/// 3-vector addition `a + b`.
#[inline]
#[must_use]
pub fn vec3_add<S: Real>(a: [S; 3], b: [S; 3]) -> [S; 3] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

/// 3-vector subtraction `a - b`.
#[inline]
#[must_use]
pub fn vec3_sub<S: Real>(a: [S; 3], b: [S; 3]) -> [S; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

/// 3-vector scaling `k * v`.
#[inline]
#[must_use]
pub fn vec3_scale<S: Real>(v: [S; 3], k: S) -> [S; 3] {
    [v[0] * k, v[1] * k, v[2] * k]
}

/// 3-vector dot product `a . b`.
#[inline]
#[must_use]
pub fn vec3_dot<S: Real>(a: [S; 3], b: [S; 3]) -> S {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

/// 3-vector cross product `a x b`.
#[inline]
#[must_use]
pub fn vec3_cross<S: Real>(a: [S; 3], b: [S; 3]) -> [S; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

/// Canonicalize a quaternion `[w, x, y, z]` under `fs-ga`'s non-negative-scalar
/// hemisphere rule with lexicographic tie-break at `w == 0` and `-0.0` clearing.
#[inline]
#[must_use]
pub fn quat_canonicalize<S: Real>(q: [S; 4]) -> [S; 4] {
    let w = q[0].value();
    let x = q[1].value();
    let y = q[2].value();
    let z = q[3].value();
    let negate = if w < 0.0 {
        true
    } else if w == 0.0 {
        if x != 0.0 {
            x < 0.0
        } else if y != 0.0 {
            y < 0.0
        } else {
            z < 0.0
        }
    } else {
        false
    };
    let signed = if negate {
        [-q[0], -q[1], -q[2], -q[3]]
    } else {
        q
    };
    // Clear negative-zero primal values cleanly without dropping dual tangents.
    [
        clear_neg_zero(signed[0]),
        clear_neg_zero(signed[1]),
        clear_neg_zero(signed[2]),
        clear_neg_zero(signed[3]),
    ]
}

#[inline]
fn clear_neg_zero<S: Real>(x: S) -> S {
    if x.value() == 0.0 && x.value().is_sign_negative() {
        x + S::zero()
    } else {
        x
    }
}

/// Hamilton quaternion product `a * b` in `[w, x, y, z]` order.
#[inline]
#[must_use]
pub fn quat_mul<S: Real>(a: [S; 4], b: [S; 4]) -> [S; 4] {
    let [aw, ax, ay, az] = a;
    let [bw, bx, by, bz] = b;
    [
        aw * bw - ax * bx - ay * by - az * bz,
        aw * bx + ax * bw + ay * bz - az * by,
        aw * by - ax * bz + ay * bw + az * bx,
        aw * bz + ax * by - ay * bx + az * bw,
    ]
}

/// Quaternion conjugate `[w, -x, -y, -z]` (inverse for unit quaternions).
#[inline]
#[must_use]
pub fn quat_conjugate<S: Real>(q: [S; 4]) -> [S; 4] {
    [q[0], -q[1], -q[2], -q[3]]
}

/// Convert a unit quaternion `[w, x, y, z]` into its 3x3 rotation matrix `R` (row-major).
#[inline]
#[must_use]
pub fn quat_to_mat3<S: Real>(q: [S; 4]) -> [[S; 3]; 3] {
    let [w, x, y, z] = q;
    let one = S::one();
    let two = S::from_f64(2.0);
    let xx = two * x * x;
    let yy = two * y * y;
    let zz = two * z * z;
    let xy = two * x * y;
    let xz = two * x * z;
    let yz = two * y * z;
    let wx = two * w * x;
    let wy = two * w * y;
    let wz = two * w * z;
    [
        [one - yy - zz, xy - wz, xz + wy],
        [xy + wz, one - xx - zz, yz - wx],
        [xz - wy, yz + wx, one - xx - yy],
    ]
}

/// Apply a row-major 3x3 matrix `m` to a 3-vector `v`.
#[inline]
#[must_use]
pub fn mat3_apply<S: Real>(m: [[S; 3]; 3], v: [S; 3]) -> [S; 3] {
    [
        m[0][0] * v[0] + m[0][1] * v[1] + m[0][2] * v[2],
        m[1][0] * v[0] + m[1][1] * v[1] + m[1][2] * v[2],
        m[2][0] * v[0] + m[2][1] * v[1] + m[2][2] * v[2],
    ]
}

/// Apply a row-major 6x6 matrix `m` to a 6-vector `v`.
#[inline]
#[must_use]
pub fn mat6_apply<S: Real>(m: [[S; 6]; 6], v: [S; 6]) -> [S; 6] {
    let mut out = [S::zero(); 6];
    for r in 0..6 {
        out[r] = m[r][0] * v[0]
            + m[r][1] * v[1]
            + m[r][2] * v[2]
            + m[r][3] * v[3]
            + m[r][4] * v[4]
            + m[r][5] * v[5];
    }
    out
}

/// Skew-symmetric hat matrix `[v]_x` such that `[v]_x u = v x u`.
#[inline]
#[must_use]
pub fn skew3<S: Real>(v: [S; 3]) -> [[S; 3]; 3] {
    let z = S::zero();
    [[z, -v[2], v[1]], [v[2], z, -v[0]], [-v[1], v[0], z]]
}

/// Rotate a 3-vector `v` by unit quaternion `q`: `R(q) v`.
#[inline]
#[must_use]
pub fn so3_rotate<S: Real>(q: [S; 4], v: [S; 3]) -> [S; 3] {
    mat3_apply(quat_to_mat3(q), v)
}

/// `SO(3)` group composition with canonical hemisphere sign.
#[inline]
#[must_use]
pub fn so3_compose<S: Real>(q_a: [S; 4], q_b: [S; 4]) -> [S; 4] {
    quat_canonicalize(quat_mul(q_a, q_b))
}

/// `SO(3)` group inverse with canonical hemisphere sign.
#[inline]
#[must_use]
pub fn so3_inverse<S: Real>(q: [S; 4]) -> [S; 4] {
    quat_canonicalize(quat_conjugate(q))
}

/// `SO(3)` adjoint matrix `Ad_R = R`.
#[inline]
#[must_use]
pub fn so3_adjoint<S: Real>(q: [S; 4]) -> [[S; 3]; 3] {
    quat_to_mat3(q)
}

/// `so(3)` Lie bracket `[omega, eta] = omega x eta`.
#[inline]
#[must_use]
pub fn so3_bracket<S: Real>(omega: [S; 3], eta: [S; 3]) -> [S; 3] {
    vec3_cross(omega, eta)
}

/// `so(3)` infinitesimal adjoint matrix `ad_omega = [omega]_x`.
#[inline]
#[must_use]
pub fn so3_ad<S: Real>(omega: [S; 3]) -> [[S; 3]; 3] {
    skew3(omega)
}

/// `so(3)` infinitesimal coadjoint `ad*_omega(mu) = (ad_omega)^T mu = mu x omega` (no minus sign).
#[inline]
#[must_use]
pub fn so3_coad<S: Real>(omega: [S; 3], mu: [S; 3]) -> [S; 3] {
    vec3_cross(mu, omega)
}

/// `so(3)` dual pairing `<mu, omega> = mu . omega`.
#[inline]
#[must_use]
pub fn so3_pair<S: Real>(omega: [S; 3], mu: [S; 3]) -> S {
    vec3_dot(mu, omega)
}

/// `SE(3)` group composition: `(R_a, p_a) * (R_b, p_b) = (R_a R_b, R_a p_b + p_a)`.
#[inline]
#[must_use]
pub fn se3_compose<S: Real>(
    q_a: [S; 4],
    p_a: [S; 3],
    q_b: [S; 4],
    p_b: [S; 3],
) -> ([S; 4], [S; 3]) {
    let q = so3_compose(q_a, q_b);
    let p = vec3_add(so3_rotate(q_a, p_b), p_a);
    (q, p)
}

/// `SE(3)` group inverse: `(R, p)^{-1} = (R^T, -R^T p)`.
#[inline]
#[must_use]
pub fn se3_inverse<S: Real>(q: [S; 4], p: [S; 3]) -> ([S; 4], [S; 3]) {
    let q_inv = so3_inverse(q);
    let rp = so3_rotate(q_inv, p);
    (q_inv, [-rp[0], -rp[1], -rp[2]])
}

/// `SE(3)` point action `x_a = R x_b + p`.
#[inline]
#[must_use]
pub fn se3_transform_point<S: Real>(q: [S; 4], p: [S; 3], point: [S; 3]) -> [S; 3] {
    vec3_add(so3_rotate(q, point), p)
}

/// `SE(3)` free-vector action `v_a = R v_b`.
#[inline]
#[must_use]
pub fn se3_transform_vector<S: Real>(q: [S; 4], vec: [S; 3]) -> [S; 3] {
    so3_rotate(q, vec)
}

/// `SE(3)` adjoint matrix `Ad_g = [[R, 0], [[p]_x R, R]]`.
#[inline]
#[must_use]
pub fn se3_adjoint_matrix<S: Real>(q: [S; 4], p: [S; 3]) -> [[S; 6]; 6] {
    let r = quat_to_mat3(q);
    let px = skew3(p);
    let px_r = mat3_mul(px, r);
    let z = S::zero();
    let mut out = [[z; 6]; 6];
    for i in 0..3 {
        for j in 0..3 {
            out[i][j] = r[i][j];
            out[i + 3][j] = px_r[i][j];
            out[i + 3][j + 3] = r[i][j];
        }
    }
    out
}

/// `SE(3)` adjoint action on a twist `xi = (omega, v)`: `Ad_g(omega, v) = (R omega, R v + p x R omega)`.
#[inline]
#[must_use]
pub fn se3_adjoint_twist<S: Real>(q: [S; 4], p: [S; 3], xi: [S; 6]) -> [S; 6] {
    let omega = [xi[0], xi[1], xi[2]];
    let lin = [xi[3], xi[4], xi[5]];
    let r_omega = so3_rotate(q, omega);
    let r_lin = so3_rotate(q, lin);
    let v_out = vec3_add(r_lin, vec3_cross(p, r_omega));
    [r_omega[0], r_omega[1], r_omega[2], v_out[0], v_out[1], v_out[2]]
}

/// `SE(3)` group coadjoint matrix `Ad*_g = (Ad_g)^{-T} = [[R, [p]_x R], [0, R]]`.
#[inline]
#[must_use]
pub fn se3_coadjoint_matrix<S: Real>(q: [S; 4], p: [S; 3]) -> [[S; 6]; 6] {
    let r = quat_to_mat3(q);
    let px_r = mat3_mul(skew3(p), r);
    let z = S::zero();
    let mut out = [[z; 6]; 6];
    for i in 0..3 {
        for j in 0..3 {
            out[i][j] = r[i][j];
            out[i][j + 3] = px_r[i][j];
            out[i + 3][j + 3] = r[i][j];
        }
    }
    out
}

/// `SE(3)` group coadjoint action on a wrench `w = (tau, f)`: `Ad*_g(tau, f) = (R tau + p x R f, R f)`.
#[inline]
#[must_use]
pub fn se3_coadjoint_wrench<S: Real>(q: [S; 4], p: [S; 3], w: [S; 6]) -> [S; 6] {
    let tau = [w[0], w[1], w[2]];
    let f = [w[3], w[4], w[5]];
    let r_tau = so3_rotate(q, tau);
    let r_f = so3_rotate(q, f);
    let tau_out = vec3_add(r_tau, vec3_cross(p, r_f));
    [tau_out[0], tau_out[1], tau_out[2], r_f[0], r_f[1], r_f[2]]
}

/// `SE(3)` wrench pull-back `Ad*_{g^{-1}}(w)`.
#[inline]
#[must_use]
pub fn se3_pullback_wrench<S: Real>(q: [S; 4], p: [S; 3], w: [S; 6]) -> [S; 6] {
    let (q_inv, p_inv) = se3_inverse(q, p);
    se3_coadjoint_wrench(q_inv, p_inv, w)
}

/// `se(3)` Lie bracket `[xi, eta] = (omega x eta_omega, omega x eta_v + v x eta_omega)`.
#[inline]
#[must_use]
pub fn se3_bracket<S: Real>(xi: [S; 6], eta: [S; 6]) -> [S; 6] {
    let w1 = [xi[0], xi[1], xi[2]];
    let v1 = [xi[3], xi[4], xi[5]];
    let w2 = [eta[0], eta[1], eta[2]];
    let v2 = [eta[3], eta[4], eta[5]];
    let w = vec3_cross(w1, w2);
    let v = vec3_add(vec3_cross(w1, v2), vec3_cross(v1, w2));
    [w[0], w[1], w[2], v[0], v[1], v[2]]
}

/// `se(3)` infinitesimal adjoint matrix `ad_xi = [[[omega]_x, 0], [[v]_x, [omega]_x]]`.
#[inline]
#[must_use]
pub fn se3_ad_matrix<S: Real>(xi: [S; 6]) -> [[S; 6]; 6] {
    let wx = skew3([xi[0], xi[1], xi[2]]);
    let vx = skew3([xi[3], xi[4], xi[5]]);
    let z = S::zero();
    let mut out = [[z; 6]; 6];
    for i in 0..3 {
        for j in 0..3 {
            out[i][j] = wx[i][j];
            out[i + 3][j] = vx[i][j];
            out[i + 3][j + 3] = wx[i][j];
        }
    }
    out
}

/// `se(3)` infinitesimal coadjoint `ad*_xi(pi, ell) = (pi x omega + ell x v, ell x omega)`
/// (NO minus sign; ADR-MARROW-05).
#[inline]
#[must_use]
pub fn se3_coad<S: Real>(xi: [S; 6], mu: [S; 6]) -> [S; 6] {
    let omega = [xi[0], xi[1], xi[2]];
    let v = [xi[3], xi[4], xi[5]];
    let pi = [mu[0], mu[1], mu[2]];
    let ell = [mu[3], mu[4], mu[5]];
    let pi_out = vec3_add(vec3_cross(pi, omega), vec3_cross(ell, v));
    let ell_out = vec3_cross(ell, omega);
    [pi_out[0], pi_out[1], pi_out[2], ell_out[0], ell_out[1], ell_out[2]]
}

/// `se(3)` dual pairing `<w, xi> = tau . omega + f . v`.
#[inline]
#[must_use]
pub fn se3_pair<S: Real>(xi: [S; 6], w: [S; 6]) -> S {
    vec3_dot([w[0], w[1], w[2]], [xi[0], xi[1], xi[2]])
        + vec3_dot([w[3], w[4], w[5]], [xi[3], xi[4], xi[5]])
}

#[inline]
fn mat3_mul<S: Real>(a: [[S; 3]; 3], b: [[S; 3]; 3]) -> [[S; 3]; 3] {
    let z = S::zero();
    let mut out = [[z; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            out[i][j] = a[i][0] * b[0][j] + a[i][1] * b[1][j] + a[i][2] * b[2][j];
        }
    }
    out
}
