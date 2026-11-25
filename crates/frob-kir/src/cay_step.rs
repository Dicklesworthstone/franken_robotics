//! First derived kernel body `K-cay-step` in `frob.kir/1` (`BD-018`).
//!
//! State: unit quaternion `q = (qw, qx, qy, qz)` (dimensionless), position `p = (px, py, pz)` [m].
//! Inputs: body angular velocity `omega` [s^-1], body linear velocity `v` [m/s], step `h` [s].
//! Outputs: unnormalized updated quaternion `q' = q (x) (1, a/2)`, denominator `d = 1 + |a|^2 / 4`,
//! and updated position `p' = p + R(q) * (I - hat(a)/2)^{-1} (h * v)`.
//!
//! Uses only `+`, `-`, `*`, `/` (no trig, no `mul_add`, no `sqrt`).

use frob_core::evidence::DetClass;
use frob_core::refusal::Refusal;

use crate::ir::{Dim, Program, ProgramBuilder, Ty};

/// Build and typecheck the canonical `K-cay-step` `frob.kir/1` program.
pub fn k_cay_step_program() -> Result<Program, Refusal> {
    let mut b = ProgramBuilder::new(
        "K-cay-step",
        "frob_kir::cay_step::k_cay_step_body",
        DetClass::D2PerBackend,
    );

    let qw = b.input("qw", Ty::F32, Dim::DIMENSIONLESS)?;
    let qx = b.input("qx", Ty::F32, Dim::DIMENSIONLESS)?;
    let qy = b.input("qy", Ty::F32, Dim::DIMENSIONLESS)?;
    let qz = b.input("qz", Ty::F32, Dim::DIMENSIONLESS)?;
    let px = b.input("px", Ty::F32, Dim::LENGTH)?;
    let py = b.input("py", Ty::F32, Dim::LENGTH)?;
    let pz = b.input("pz", Ty::F32, Dim::LENGTH)?;
    let wx = b.input("wx", Ty::F32, Dim::ANGULAR_VELOCITY)?;
    let wy = b.input("wy", Ty::F32, Dim::ANGULAR_VELOCITY)?;
    let wz = b.input("wz", Ty::F32, Dim::ANGULAR_VELOCITY)?;
    let vx = b.input("vx", Ty::F32, Dim::VELOCITY)?;
    let vy = b.input("vy", Ty::F32, Dim::VELOCITY)?;
    let vz = b.input("vz", Ty::F32, Dim::VELOCITY)?;
    let h = b.input("h", Ty::F32, Dim::TIME)?;

    let c1 = b.const_f32(1.0, Dim::DIMENSIONLESS)?;
    let c2 = b.const_f32(2.0, Dim::DIMENSIONLESS)?;
    let half = b.const_f32(0.5, Dim::DIMENSIONLESS)?;
    let quarter = b.const_f32(0.25, Dim::DIMENSIONLESS)?;

    // a = h * omega (dimensionless)
    let ax = b.mul(h, wx, "ax")?;
    let ay = b.mul(h, wy, "ay")?;
    let az = b.mul(h, wz, "az")?;

    // c = (1, ax/2, ay/2, az/2)
    let cx = b.mul(ax, half, "cx")?;
    let cy = b.mul(ay, half, "cy")?;
    let cz = b.mul(az, half, "cz")?;

    // q' = q (x) c (Hamilton product, cw = 1)
    let qx_cx = b.mul(qx, cx, "qx*cx")?;
    let qy_cy = b.mul(qy, cy, "qy*cy")?;
    let qz_cz = b.mul(qz, cz, "qz*cz")?;
    let qw_1 = b.sub(qw, qx_cx, "qw-qx*cx")?;
    let qw_2 = b.sub(qw_1, qy_cy, "qw-qx*cx-qy*cy")?;
    let qw_out = b.sub(qw_2, qz_cz, "qw_out")?;

    let qw_cx = b.mul(qw, cx, "qw*cx")?;
    let qy_cz = b.mul(qy, cz, "qy*cz")?;
    let qz_cy = b.mul(qz, cy, "qz*cy")?;
    let qx_1 = b.add(qw_cx, qx, "qw*cx+qx")?;
    let qx_2 = b.add(qx_1, qy_cz, "qx_2")?;
    let qx_out = b.sub(qx_2, qz_cy, "qx_out")?;

    let qw_cy = b.mul(qw, cy, "qw*cy")?;
    let qx_cz = b.mul(qx, cz, "qx*cz")?;
    let qz_cx = b.mul(qz, cx, "qz*cx")?;
    let qy_1 = b.sub(qw_cy, qx_cz, "qy_1")?;
    let qy_2 = b.add(qy_1, qy, "qy_2")?;
    let qy_out = b.add(qy_2, qz_cx, "qy_out")?;

    let qw_cz = b.mul(qw, cz, "qw*cz")?;
    let qx_cy = b.mul(qx, cy, "qx*cy")?;
    let qy_cx = b.mul(qy, cx, "qy*cx")?;
    let qz_1 = b.add(qw_cz, qx_cy, "qz_1")?;
    let qz_2 = b.sub(qz_1, qy_cx, "qz_2")?;
    let qz_out = b.add(qz_2, qz, "qz_out")?;

    // d = 1 + (a . a) / 4
    let ax2 = b.mul(ax, ax, "ax^2")?;
    let ay2 = b.mul(ay, ay, "ay^2")?;
    let az2 = b.mul(az, az, "az^2")?;
    let aa_xy = b.add(ax2, ay2, "ax^2+ay^2")?;
    let aa = b.add(aa_xy, az2, "a.a")?;
    let aa_q = b.mul(aa, quarter, "a.a/4")?;
    let d = b.add(c1, aa_q, "d")?;

    // hv = h * v [m]
    let hvx = b.mul(h, vx, "h*vx")?;
    let hvy = b.mul(h, vy, "h*vy")?;
    let hvz = b.mul(h, vz, "h*vz")?;

    // cross = a x hv [m]
    let cr_x1 = b.mul(ay, hvz, "ay*hvz")?;
    let cr_x2 = b.mul(az, hvy, "az*hvy")?;
    let cr_x = b.sub(cr_x1, cr_x2, "(a x hv)_x")?;

    let cr_y1 = b.mul(az, hvx, "az*hvx")?;
    let cr_y2 = b.mul(ax, hvz, "ax*hvz")?;
    let cr_y = b.sub(cr_y1, cr_y2, "(a x hv)_y")?;

    let cr_z1 = b.mul(ax, hvy, "ax*hvy")?;
    let cr_z2 = b.mul(ay, hvx, "ay*hvx")?;
    let cr_z = b.sub(cr_z1, cr_z2, "(a x hv)_z")?;

    // dot = a . hv [m]
    let d_x = b.mul(ax, hvx, "ax*hvx")?;
    let d_y = b.mul(ay, hvy, "ay*hvy")?;
    let d_z = b.mul(az, hvz, "az*hvz")?;
    let dot_xy = b.add(d_x, d_y, "dot_xy")?;
    let dot = b.add(dot_xy, d_z, "a.hv")?;
    let q_dot = b.mul(quarter, dot, "0.25*(a.hv)")?;

    // u = (hv + 0.5*(a x hv) + 0.25*a*(a.hv)) / d [m]
    let h_crx = b.mul(half, cr_x, "0.5*cr_x")?;
    let h_cry = b.mul(half, cr_y, "0.5*cr_y")?;
    let h_crz = b.mul(half, cr_z, "0.5*cr_z")?;
    let q_ax = b.mul(ax, q_dot, "0.25*ax*(a.hv)")?;
    let q_ay = b.mul(ay, q_dot, "0.25*ay*(a.hv)")?;
    let q_az = b.mul(az, q_dot, "0.25*az*(a.hv)")?;

    let ux_1 = b.add(hvx, h_crx, "ux_1")?;
    let ux_n = b.add(ux_1, q_ax, "ux_num")?;
    let ux = b.div(ux_n, d, "ux")?;

    let uy_1 = b.add(hvy, h_cry, "uy_1")?;
    let uy_n = b.add(uy_1, q_ay, "uy_num")?;
    let uy = b.div(uy_n, d, "uy")?;

    let uz_1 = b.add(hvz, h_crz, "uz_1")?;
    let uz_n = b.add(uz_1, q_az, "uz_num")?;
    let uz = b.div(uz_n, d, "uz")?;

    // R(q) entries written in products of q components
    let xx = b.mul(qx, qx, "qx^2")?;
    let yy = b.mul(qy, qy, "qy^2")?;
    let zz = b.mul(qz, qz, "qz^2")?;
    let xy = b.mul(qx, qy, "qx*qy")?;
    let xz = b.mul(qx, qz, "qx*qz")?;
    let yz = b.mul(qy, qz, "qy*qz")?;
    let wx_q = b.mul(qw, qx, "qw*qx")?;
    let wy_q = b.mul(qw, qy, "qw*qy")?;
    let wz_q = b.mul(qw, qz, "qw*qz")?;

    let xx2 = b.mul(c2, xx, "2*qx^2")?;
    let yy2 = b.mul(c2, yy, "2*qy^2")?;
    let zz2 = b.mul(c2, zz, "2*qz^2")?;
    let xy2 = b.mul(c2, xy, "2*qx*qy")?;
    let xz2 = b.mul(c2, xz, "2*qx*qz")?;
    let yz2 = b.mul(c2, yz, "2*qy*qz")?;
    let wx2 = b.mul(c2, wx_q, "2*qw*qx")?;
    let wy2 = b.mul(c2, wy_q, "2*qw*qy")?;
    let wz2 = b.mul(c2, wz_q, "2*qw*qz")?;

    let r00_1 = b.sub(c1, yy2, "1-2yy")?;
    let r00 = b.sub(r00_1, zz2, "r00")?;
    let r01 = b.sub(xy2, wz2, "r01")?;
    let r02 = b.add(xz2, wy2, "r02")?;

    let r10 = b.add(xy2, wz2, "r10")?;
    let r11_1 = b.sub(c1, xx2, "1-2xx")?;
    let r11 = b.sub(r11_1, zz2, "r11")?;
    let r12 = b.sub(yz2, wx2, "r12")?;

    let r20 = b.sub(xz2, wy2, "r20")?;
    let r21 = b.add(yz2, wx2, "r21")?;
    let r22_1 = b.sub(c1, xx2, "1-2xx")?;
    let r22 = b.sub(r22_1, yy2, "r22")?;

    // p' = p + R(q) * u [m]
    let rx0 = b.mul(r00, ux, "r00*ux")?;
    let rx1 = b.mul(r01, uy, "r01*uy")?;
    let rx2 = b.mul(r02, uz, "r02*uz")?;
    let dp_x0 = b.add(rx0, rx1, "dp_x0")?;
    let dp_x = b.add(dp_x0, rx2, "dp_x")?;
    let px_out = b.add(px, dp_x, "px_out")?;

    let ry0 = b.mul(r10, ux, "r10*ux")?;
    let ry1 = b.mul(r11, uy, "r11*uy")?;
    let ry2 = b.mul(r12, uz, "r12*uz")?;
    let dp_y0 = b.add(ry0, ry1, "dp_y0")?;
    let dp_y = b.add(dp_y0, ry2, "dp_y")?;
    let py_out = b.add(py, dp_y, "py_out")?;

    let rz0 = b.mul(r20, ux, "r20*ux")?;
    let rz1 = b.mul(r21, uy, "r21*uy")?;
    let rz2 = b.mul(r22, uz, "r22*uz")?;
    let dp_z0 = b.add(rz0, rz1, "dp_z0")?;
    let dp_z = b.add(dp_z0, rz2, "dp_z")?;
    let pz_out = b.add(pz, dp_z, "pz_out")?;

    b.output("qw_out", Ty::F32, Dim::DIMENSIONLESS, qw_out);
    b.output("qx_out", Ty::F32, Dim::DIMENSIONLESS, qx_out);
    b.output("qy_out", Ty::F32, Dim::DIMENSIONLESS, qy_out);
    b.output("qz_out", Ty::F32, Dim::DIMENSIONLESS, qz_out);
    b.output("d", Ty::F32, Dim::DIMENSIONLESS, d);
    b.output("px_out", Ty::F32, Dim::LENGTH, px_out);
    b.output("py_out", Ty::F32, Dim::LENGTH, py_out);
    b.output("pz_out", Ty::F32, Dim::LENGTH, pz_out);
    b.build()
}

/// Bit-exact Rust CPU twin of `K-cay-step` matching `emit_rust(&k_cay_step_program())`
/// operation for operation (one operation per statement, bit-encoded constants).
#[allow(clippy::too_many_arguments)]
#[inline]
#[must_use]
pub fn k_cay_step_body(
    qw: f32,
    qx: f32,
    qy: f32,
    qz: f32,
    px: f32,
    py: f32,
    pz: f32,
    wx: f32,
    wy: f32,
    wz: f32,
    vx: f32,
    vy: f32,
    vz: f32,
    h: f32,
) -> (f32, f32, f32, f32, f32, f32, f32, f32) {
    let c1 = f32::from_bits(0x3f80_0000u32);
    let c2 = f32::from_bits(0x4000_0000u32);
    let half = f32::from_bits(0x3f00_0000u32);
    let quarter = f32::from_bits(0x3e80_0000u32);

    let ax = h * wx;
    let ay = h * wy;
    let az = h * wz;

    let cx = ax * half;
    let cy = ay * half;
    let cz = az * half;

    let qx_cx = qx * cx;
    let qy_cy = qy * cy;
    let qz_cz = qz * cz;
    let qw_1 = qw - qx_cx;
    let qw_2 = qw_1 - qy_cy;
    let qw_out = qw_2 - qz_cz;

    let qw_cx = qw * cx;
    let qy_cz = qy * cz;
    let qz_cy = qz * cy;
    let qx_1 = qw_cx + qx;
    let qx_2 = qx_1 + qy_cz;
    let qx_out = qx_2 - qz_cy;

    let qw_cy = qw * cy;
    let qx_cz = qx * cz;
    let qz_cx = qz * cx;
    let qy_1 = qw_cy - qx_cz;
    let qy_2 = qy_1 + qy;
    let qy_out = qy_2 + qz_cx;

    let qw_cz = qw * cz;
    let qx_cy = qx * cy;
    let qy_cx = qy * cx;
    let qz_1 = qw_cz + qx_cy;
    let qz_2 = qz_1 - qy_cx;
    let qz_out = qz_2 + qz;

    let ax2 = ax * ax;
    let ay2 = ay * ay;
    let az2 = az * az;
    let aa_xy = ax2 + ay2;
    let aa = aa_xy + az2;
    let aa_q = aa * quarter;
    let d = c1 + aa_q;

    let hvx = h * vx;
    let hvy = h * vy;
    let hvz = h * vz;

    let cr_x1 = ay * hvz;
    let cr_x2 = az * hvy;
    let cr_x = cr_x1 - cr_x2;
    let cr_y1 = az * hvx;
    let cr_y2 = ax * hvz;
    let cr_y = cr_y1 - cr_y2;
    let cr_z1 = ax * hvy;
    let cr_z2 = ay * hvx;
    let cr_z = cr_z1 - cr_z2;

    let d_x = ax * hvx;
    let d_y = ay * hvy;
    let d_z = az * hvz;
    let dot_xy = d_x + d_y;
    let dot = dot_xy + d_z;
    let q_dot = quarter * dot;

    let h_crx = half * cr_x;
    let h_cry = half * cr_y;
    let h_crz = half * cr_z;
    let q_ax = ax * q_dot;
    let q_ay = ay * q_dot;
    let q_az = az * q_dot;

    let ux_1 = hvx + h_crx;
    let ux_n = ux_1 + q_ax;
    let ux = ux_n / d;

    let uy_1 = hvy + h_cry;
    let uy_n = uy_1 + q_ay;
    let uy = uy_n / d;

    let uz_1 = hvz + h_crz;
    let uz_n = uz_1 + q_az;
    let uz = uz_n / d;

    let xx = qx * qx;
    let yy = qy * qy;
    let zz = qz * qz;
    let xy = qx * qy;
    let xz = qx * qz;
    let yz = qy * qz;
    let wx_q = qw * qx;
    let wy_q = qw * qy;
    let wz_q = qw * qz;

    let xx2 = c2 * xx;
    let yy2 = c2 * yy;
    let zz2 = c2 * zz;
    let xy2 = c2 * xy;
    let xz2 = c2 * xz;
    let yz2 = c2 * yz;
    let wx2 = c2 * wx_q;
    let wy2 = c2 * wy_q;
    let wz2 = c2 * wz_q;

    let r00_1 = c1 - yy2;
    let r00 = r00_1 - zz2;
    let r01 = xy2 - wz2;
    let r02 = xz2 + wy2;

    let r10 = xy2 + wz2;
    let r11_1 = c1 - xx2;
    let r11 = r11_1 - zz2;
    let r12 = yz2 - wx2;

    let r20 = xz2 - wy2;
    let r21 = yz2 + wx2;
    let r22_1 = c1 - xx2;
    let r22 = r22_1 - yy2;

    let rx0 = r00 * ux;
    let rx1 = r01 * uy;
    let rx2 = r02 * uz;
    let dp_x0 = rx0 + rx1;
    let dp_x = dp_x0 + rx2;
    let px_out = px + dp_x;

    let ry0 = r10 * ux;
    let ry1 = r11 * uy;
    let ry2 = r12 * uz;
    let dp_y0 = ry0 + ry1;
    let dp_y = dp_y0 + ry2;
    let py_out = py + dp_y;

    let rz0 = r20 * ux;
    let rz1 = r21 * uy;
    let rz2 = r22 * uz;
    let dp_z0 = rz0 + rz1;
    let dp_z = dp_z0 + rz2;
    let pz_out = pz + dp_z;

    (qw_out, qx_out, qy_out, qz_out, d, px_out, py_out, pz_out)
}
