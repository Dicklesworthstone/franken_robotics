#![forbid(unsafe_code)]

//! PO-MARROW-01 bit-parity proof obligation and BD-010 unit test suite.
//!
//! Tests required by BD-010:
//! - `po_marrow_01_bit_parity` (KA-256: 1,000 records bit-for-bit against fs-ga capture)
//! - `coad_is_transpose_of_ad` (exact integer-valued `<ad*_xi mu, eta> == <mu, [xi, eta]>`)
//! - `coad_components` (KA-229)
//! - `wrench_pushforward_moment_arm` (KA-193)
//! - `pairing_invariant_small` (64 boundary records within 64 ULP)
//! - `generic_matches_fsga` (reference implementation at f64, Dual64, and f32)
//! - `no_private_vector_types` (ADR-MARROW-01 public API scan)
//! - `sign_rule_matches_fsga` (double-cover hemisphere rule + 1,000 seeded q/-q pairs)
//! - `fallible_ops_refuse` (`LieError::Host(GaError)` without panic or renormalization)

use std::fs;
use std::path::{Path, PathBuf};

use frob_core::id::hash_blob;
use frob_core::registry::toml::{self, Value};
use frob_lie::reference;
use frob_lie::{
    LieAlgebra, LieError, LieGroup, MOTOR_EVEN_BLADES, Motor, Quat, Se3, Se3Ops, So3, So3Ops,
    So3Tangent, Twist, Vec3, Wrench, twist, wrench,
};
use fs_ad::{Dual64, Real, gradient};
use fs_rand::StreamKey;

const EXPECTED_RECORD_COUNT: usize = 1000;
const EXPECTED_BOUNDARY_COUNT: usize = 64;
const FIELDS_PER_RECORD: usize = 135;

const FIELD_NAMES: [&str; FIELDS_PER_RECORD] = [
    // 0..14 raw inputs
    "raw_q_g.w", "raw_q_g.x", "raw_q_g.y", "raw_q_g.z",
    "raw_p_g.x", "raw_p_g.y", "raw_p_g.z",
    "raw_q_h.w", "raw_q_h.x", "raw_q_h.y", "raw_q_h.z",
    "raw_p_h.x", "raw_p_h.y", "raw_p_h.z",
    // 14..38 stored forms
    "stored_so3_g.w", "stored_so3_g.x", "stored_so3_g.y", "stored_so3_g.z",
    "stored_se3_g.m0", "stored_se3_g.m1", "stored_se3_g.m2", "stored_se3_g.m3",
    "stored_se3_g.m4", "stored_se3_g.m5", "stored_se3_g.m6", "stored_se3_g.m7",
    "stored_so3_h.w", "stored_so3_h.x", "stored_so3_h.y", "stored_so3_h.z",
    "stored_se3_h.m0", "stored_se3_h.m1", "stored_se3_h.m2", "stored_se3_h.m3",
    "stored_se3_h.m4", "stored_se3_h.m5", "stored_se3_h.m6", "stored_se3_h.m7",
    // 38..56 xi, eta, w
    "xi.wx", "xi.wy", "xi.wz", "xi.vx", "xi.vy", "xi.vz",
    "eta.wx", "eta.wy", "eta.wz", "eta.vx", "eta.vy", "eta.vz",
    "w.tx", "w.ty", "w.tz", "w.fx", "w.fy", "w.fz",
    // 56..67 so3 outputs
    "so3_compose.w", "so3_compose.x", "so3_compose.y", "so3_compose.z",
    "so3_inverse.w", "so3_inverse.x", "so3_inverse.y", "so3_inverse.z",
    "so3_rotate.x", "so3_rotate.y", "so3_rotate.z",
    // 67..86 se3 outputs
    "se3_compose.m0", "se3_compose.m1", "se3_compose.m2", "se3_compose.m3",
    "se3_compose.m4", "se3_compose.m5", "se3_compose.m6", "se3_compose.m7",
    "se3_inverse.m0", "se3_inverse.m1", "se3_inverse.m2", "se3_inverse.m3",
    "se3_inverse.m4", "se3_inverse.m5", "se3_inverse.m6", "se3_inverse.m7",
    "se3_transform_point.x", "se3_transform_point.y", "se3_transform_point.z",
    // 86..98 Ad_g xi
    "ad_g_xi.wx", "ad_g_xi.wy", "ad_g_xi.wz", "ad_g_xi.vx", "ad_g_xi.vy", "ad_g_xi.vz",
    "ad_g_mat_xi.wx", "ad_g_mat_xi.wy", "ad_g_mat_xi.wz", "ad_g_mat_xi.vx", "ad_g_mat_xi.vy", "ad_g_mat_xi.vz",
    // 98..110 Ad*_g w
    "coad_g_w.tx", "coad_g_w.ty", "coad_g_w.tz", "coad_g_w.fx", "coad_g_w.fy", "coad_g_w.fz",
    "coad_g_mat_w.tx", "coad_g_mat_w.ty", "coad_g_mat_w.tz", "coad_g_mat_w.fx", "coad_g_mat_w.fy", "coad_g_mat_w.fz",
    // 110..122 pullback Ad*_{g^-1} w
    "pullback_w.tx", "pullback_w.ty", "pullback_w.tz", "pullback_w.fx", "pullback_w.fy", "pullback_w.fz",
    "pullback_mat_w.tx", "pullback_mat_w.ty", "pullback_mat_w.tz", "pullback_mat_w.fx", "pullback_mat_w.fy", "pullback_mat_w.fz",
    // 122..134 bracket and ad_xi(eta)
    "bracket_xi_eta.wx", "bracket_xi_eta.wy", "bracket_xi_eta.wz", "bracket_xi_eta.vx", "bracket_xi_eta.vy", "bracket_xi_eta.vz",
    "ad_mat_xi_eta.wx", "ad_mat_xi_eta.wy", "ad_mat_xi_eta.wz", "ad_mat_xi_eta.vx", "ad_mat_xi_eta.vy", "ad_mat_xi_eta.vz",
    // 134 pairing
    "pairing_w_xi",
];

struct ParsedFixture {
    path: PathBuf,
    records_blake3: String,
    records: Vec<[u64; FIELDS_PER_RECORD]>,
}

fn fixture_path() -> PathBuf {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    manifest_dir
        .parent()
        .and_then(Path::parent)
        .expect("workspace root")
        .join("tests/fixtures/oracle/marrow/po_marrow_01.frobfix")
}

fn load_fixture() -> ParsedFixture {
    let path = fixture_path();
    let bytes = fs::read(&path)
        .unwrap_or_else(|e| panic!("failed reading fixture {}: {e}", path.display()));
    let root = toml::parse(&bytes)
        .unwrap_or_else(|e| panic!("failed parsing fixture {}: {e}", path.display()));

    assert_eq!(root.get("kind").and_then(Value::as_str), Some("oracle"));
    assert_eq!(
        root.get("oracle").and_then(Value::as_str),
        Some("frankensim-referee")
    );

    let payload = root
        .get("payload")
        .and_then(Value::as_table)
        .expect("[payload] table must exist in po_marrow_01.frobfix");
    assert_eq!(
        payload.get("record_count").and_then(Value::as_int),
        Some(EXPECTED_RECORD_COUNT as i64)
    );
    assert_eq!(
        payload.get("boundary_count").and_then(Value::as_int),
        Some(EXPECTED_BOUNDARY_COUNT as i64)
    );
    assert_eq!(
        payload.get("fields_per_record").and_then(Value::as_int),
        Some(FIELDS_PER_RECORD as i64)
    );

    let declared_b3 = payload
        .get("records_blake3")
        .and_then(Value::as_str)
        .expect("records_blake3 must exist")
        .to_owned();
    let raw_records = payload
        .get("records")
        .and_then(Value::as_array)
        .expect("records array must exist");
    assert_eq!(raw_records.len(), EXPECTED_RECORD_COUNT);

    let mut canonical_section = String::with_capacity(EXPECTED_RECORD_COUNT * 2170);
    let mut records = Vec::with_capacity(EXPECTED_RECORD_COUNT);

    for (idx, sp) in raw_records.iter().enumerate() {
        let line = sp
            .value
            .as_str()
            .unwrap_or_else(|| panic!("record {idx} must be a string"));
        canonical_section.push_str(line);
        canonical_section.push('\n');
        records.push(decode_record_line(idx, line));
    }

    let computed_b3 = hash_blob(canonical_section.as_bytes()).to_hex();
    assert_eq!(
        computed_b3, declared_b3,
        "PO-MARROW-01 fixture records_blake3 mismatch for {}",
        path.display()
    );

    ParsedFixture {
        path,
        records_blake3: declared_b3,
        records,
    }
}

fn decode_record_line(expected_idx: usize, line: &str) -> [u64; FIELDS_PER_RECORD] {
    let (prefix, hex_body) = line
        .split_once(':')
        .unwrap_or_else(|| panic!("record {expected_idx} missing ':' prefix"));
    let parsed_idx: usize = prefix
        .parse()
        .unwrap_or_else(|e| panic!("record {expected_idx} invalid index `{prefix}`: {e}"));
    assert_eq!(parsed_idx, expected_idx, "record index out of order");
    assert_eq!(
        hex_body.len(),
        FIELDS_PER_RECORD * 16,
        "record {expected_idx} hex payload length mismatch"
    );

    let mut out = [0u64; FIELDS_PER_RECORD];
    for (slot, chunk) in out.iter_mut().zip(hex_body.as_bytes().chunks_exact(16)) {
        let s = std::str::from_utf8(chunk).expect("ascii hex");
        *slot = u64::from_str_radix(s, 16)
            .unwrap_or_else(|e| panic!("record {expected_idx} invalid u64 hex `{s}`: {e}"));
    }
    out
}

fn quat(w: f64, x: f64, y: f64, z: f64) -> Quat {
    Quat { w, x, y, z }
}

fn quat_from_bits(bits: &[u64]) -> Quat {
    quat(
        f64::from_bits(bits[0]),
        f64::from_bits(bits[1]),
        f64::from_bits(bits[2]),
        f64::from_bits(bits[3]),
    )
}

fn vec3_from_bits(bits: &[u64]) -> Vec3 {
    Vec3::new(
        f64::from_bits(bits[0]),
        f64::from_bits(bits[1]),
        f64::from_bits(bits[2]),
    )
}

fn push_quat(out: &mut [u64; FIELDS_PER_RECORD], k: &mut usize, q: Quat) {
    for v in [q.w, q.x, q.y, q.z] {
        out[*k] = v.to_bits();
        *k += 1;
    }
}

fn push_vec3(out: &mut [u64; FIELDS_PER_RECORD], k: &mut usize, v: Vec3) {
    for c in [v.x, v.y, v.z] {
        out[*k] = c.to_bits();
        *k += 1;
    }
}

fn push_motor(out: &mut [u64; FIELDS_PER_RECORD], k: &mut usize, m: Motor) {
    for &blade in &MOTOR_EVEN_BLADES {
        out[*k] = m.0.0[blade].to_bits();
        *k += 1;
    }
}

fn push_twist(out: &mut [u64; FIELDS_PER_RECORD], k: &mut usize, t: Twist) {
    push_vec3(out, k, t.angular);
    push_vec3(out, k, t.linear);
}

fn push_wrench(out: &mut [u64; FIELDS_PER_RECORD], k: &mut usize, w: Wrench) {
    push_vec3(out, k, w.torque);
    push_vec3(out, k, w.force);
}

fn ulp_distance(a: f64, b: f64) -> u64 {
    if a == b {
        return 0;
    }
    assert!(a.is_finite() && b.is_finite(), "non-finite value in ulp_distance({a}, {b})");
    let ua = f64_to_ordered_u64(a);
    let ub = f64_to_ordered_u64(b);
    ua.abs_diff(ub)
}

fn f64_to_ordered_u64(x: f64) -> u64 {
    let bits = x.to_bits();
    if (bits >> 63) == 0 {
        bits | (1u64 << 63)
    } else {
        !bits
    }
}

#[test]
fn po_marrow_01_bit_parity() {
    let fixture = load_fixture();
    assert_eq!(fixture.records_blake3.len(), 64);

    for (rec_idx, expected) in fixture.records.iter().enumerate() {
        let raw_q_g = quat_from_bits(&expected[0..4]);
        let raw_p_g = vec3_from_bits(&expected[4..7]);
        let raw_q_h = quat_from_bits(&expected[7..11]);
        let raw_p_h = vec3_from_bits(&expected[11..14]);

        let xi = twist(vec3_from_bits(&expected[38..41]), vec3_from_bits(&expected[41..44]));
        let eta = twist(vec3_from_bits(&expected[44..47]), vec3_from_bits(&expected[47..50]));
        let w = wrench(vec3_from_bits(&expected[50..53]), vec3_from_bits(&expected[53..56]));

        let so3_g = So3::try_from_quat_lie(raw_q_g)
            .unwrap_or_else(|e| panic!("record {rec_idx}: So3::try_from_quat_lie(g) failed: {e}"));
        let so3_h = So3::try_from_quat_lie(raw_q_h)
            .unwrap_or_else(|e| panic!("record {rec_idx}: So3::try_from_quat_lie(h) failed: {e}"));

        let se3_g = Se3::from_parts_lie(so3_g, raw_p_g)
            .unwrap_or_else(|e| panic!("record {rec_idx}: Se3::from_parts_lie(g) failed: {e}"));
        let se3_h = Se3::from_parts_lie(so3_h, raw_p_h)
            .unwrap_or_else(|e| panic!("record {rec_idx}: Se3::from_parts_lie(h) failed: {e}"));

        // Verify no -0.0 ever leaks into canonical stored quaternions.
        let neg_zero_bits = (-0.0_f64).to_bits();
        for q in [*so3_g.as_quat(), *so3_h.as_quat()] {
            for c in [q.w, q.x, q.y, q.z] {
                assert_ne!(
                    c.to_bits(),
                    neg_zero_bits,
                    "record {rec_idx}: stored quaternion retained -0.0 sign bit"
                );
            }
        }

        let so3_gh = <So3 as LieGroup>::compose(so3_g, so3_h)
            .unwrap_or_else(|e| panic!("record {rec_idx}: So3::compose failed: {e}"));
        let so3_g_inv = <So3 as LieGroup>::inverse(so3_g)
            .unwrap_or_else(|e| panic!("record {rec_idx}: So3::inverse failed: {e}"));
        let so3_rot_ph = so3_g
            .rotate_vec(raw_p_h)
            .unwrap_or_else(|e| panic!("record {rec_idx}: So3::rotate_vec failed: {e}"));

        let se3_gh = <Se3 as LieGroup>::compose(se3_g, se3_h)
            .unwrap_or_else(|e| panic!("record {rec_idx}: Se3::compose failed: {e}"));
        let se3_g_inv = <Se3 as LieGroup>::inverse(se3_g)
            .unwrap_or_else(|e| panic!("record {rec_idx}: Se3::inverse failed: {e}"));
        let se3_tx_ph = se3_g
            .transform_point_lie(raw_p_h)
            .unwrap_or_else(|e| panic!("record {rec_idx}: Se3::transform_point_lie failed: {e}"));

        let ad_g_xi = se3_g.adjoint_twist(xi);
        let ad_g_mat_xi = <Se3 as LieGroup>::adjoint(se3_g).apply_twist(xi);
        let coad_g_w = se3_g.coadjoint_wrench(w);
        let coad_g_mat_w = se3_g.coadjoint_matrix().apply_wrench(w);
        let pullback_w = se3_g
            .pullback_wrench(w)
            .unwrap_or_else(|e| panic!("record {rec_idx}: Se3::pullback_wrench failed: {e}"));
        let pullback_mat_w = se3_g
            .pullback_wrench_matrix(w)
            .unwrap_or_else(|e| panic!("record {rec_idx}: Se3::pullback_wrench_matrix failed: {e}"));
        let bracket_xi_eta = <Twist as LieAlgebra>::bracket(xi, eta);
        let ad_mat_xi_eta = <Twist as LieAlgebra>::ad(xi).apply_twist(eta);
        let pairing_w_xi = <Twist as LieAlgebra>::pair(xi, w).value();

        let mut actual = [0u64; FIELDS_PER_RECORD];
        let mut k = 0usize;
        push_quat(&mut actual, &mut k, raw_q_g);
        push_vec3(&mut actual, &mut k, raw_p_g);
        push_quat(&mut actual, &mut k, raw_q_h);
        push_vec3(&mut actual, &mut k, raw_p_h);

        push_quat(&mut actual, &mut k, *so3_g.as_quat());
        push_motor(&mut actual, &mut k, *se3_g.as_motor());
        push_quat(&mut actual, &mut k, *so3_h.as_quat());
        push_motor(&mut actual, &mut k, *se3_h.as_motor());

        push_twist(&mut actual, &mut k, xi);
        push_twist(&mut actual, &mut k, eta);
        push_wrench(&mut actual, &mut k, w);

        push_quat(&mut actual, &mut k, *so3_gh.as_quat());
        push_quat(&mut actual, &mut k, *so3_g_inv.as_quat());
        push_vec3(&mut actual, &mut k, so3_rot_ph);

        push_motor(&mut actual, &mut k, *se3_gh.as_motor());
        push_motor(&mut actual, &mut k, *se3_g_inv.as_motor());
        push_vec3(&mut actual, &mut k, se3_tx_ph);

        push_twist(&mut actual, &mut k, ad_g_xi);
        push_twist(&mut actual, &mut k, ad_g_mat_xi);
        push_wrench(&mut actual, &mut k, coad_g_w);
        push_wrench(&mut actual, &mut k, coad_g_mat_w);
        push_wrench(&mut actual, &mut k, pullback_w);
        push_wrench(&mut actual, &mut k, pullback_mat_w);
        push_twist(&mut actual, &mut k, bracket_xi_eta);
        push_twist(&mut actual, &mut k, ad_mat_xi_eta);
        actual[k] = pairing_w_xi.to_bits();
        k += 1;
        debug_assert_eq!(k, FIELDS_PER_RECORD);

        for field_idx in 0..FIELDS_PER_RECORD {
            if actual[field_idx] != expected[field_idx] {
                panic!(
                    "PO-MARROW-01 bit-parity mismatch at record={rec_idx} field={} expected=0x{:016x} ({}) actual=0x{:016x} ({}) fixture={}",
                    FIELD_NAMES[field_idx],
                    expected[field_idx],
                    f64::from_bits(expected[field_idx]),
                    actual[field_idx],
                    f64::from_bits(actual[field_idx]),
                    fixture.path.display(),
                );
            }
        }
    }

    // Verify boundary (q, -q) double-cover pairs yield bit-identical stored So3 & Se3.
    for (a, b) in [(13usize, 14usize), (15, 16), (17, 18), (19, 20), (21, 22)] {
        assert_eq!(
            &fixture.records[a][14..26],
            &fixture.records[b][14..26],
            "boundary pair ({a}, {b}) must store bit-identical So3 and Se3 for q and -q"
        );
    }
}

#[test]
fn coad_is_transpose_of_ad() {
    // Exact integer arithmetic in [-8, 8]: every product and sum is exact in IEEE-754 f64.
    let vals: [f64; 7] = [-8.0, -5.0, -2.0, 0.0, 1.0, 3.0, 8.0];
    let mut tested_so3 = 0usize;
    let mut tested_se3 = 0usize;

    for (i, &a) in vals.iter().enumerate() {
        for (j, &b) in vals.iter().enumerate() {
            let c = vals[(i + j * 3) % vals.len()];
            let d = vals[(i * 2 + j + 1) % vals.len()];
            let e = vals[(i + j + 2) % vals.len()];
            let f = vals[(i * 3 + j * 2 + 4) % vals.len()];

            // so(3) check: <coad_omega(mu), eta> == <mu, [omega, eta]>
            let omega = So3Tangent::new(Vec3::new(a, b, c));
            let eta_so3 = So3Tangent::new(Vec3::new(d, e, f));
            let mu_so3 = Vec3::new(c - a, b + d, f - e);
            let lhs_so3 = eta_so3.pair(omega.coad(mu_so3)).value();
            let rhs_so3 = omega.bracket(eta_so3).pair(mu_so3).value();
            assert_eq!(
                lhs_so3, rhs_so3,
                "so(3) coad != ad^T for omega={omega:?}, eta={eta_so3:?}, mu={mu_so3:?}"
            );
            tested_so3 += 1;

            // se(3) check: <coad_xi(mu), eta> == <mu, [xi, eta]>
            let xi = Twist::new(Vec3::new(a, b, c), Vec3::new(d, e, f));
            let eta = Twist::new(Vec3::new(f, -a, b), Vec3::new(-c, d, -e));
            let mu = Wrench::new(Vec3::new(b, -d, a), Vec3::new(e, c, -f));
            let lhs = eta.pair(xi.coad(mu)).value();
            let rhs = xi.bracket(eta).pair(mu).value();
            assert_eq!(
                lhs, rhs,
                "se(3) coad != ad^T for xi={xi:?}, eta={eta:?}, mu={mu:?}"
            );

            // Also verify against reference::se3_coad and reference::se3_ad_matrix transpose.
            let xi_arr = [a, b, c, d, e, f];
            let mu_arr = [b, -d, a, e, c, -f];
            let coad_ref = reference::se3_coad(xi_arr, mu_arr);
            let ad_mat = reference::se3_ad_matrix(xi_arr);
            for row in 0..6 {
                let mut col_dot = 0.0;
                for col in 0..6 {
                    col_dot += ad_mat[col][row] * mu_arr[col];
                }
                assert_eq!(coad_ref[row], col_dot, "se3_coad != (ad_xi)^T mu at row {row}");
            }
            tested_se3 += 1;
        }
    }
    assert_eq!(tested_so3, 49);
    assert_eq!(tested_se3, 49);
}

#[test]
fn coad_components() {
    // KA-229: mu = ((1, 2, 0), (0, 0, 0)), xi = ((1, 1, 0), (0, 0, 0))
    // => ad*_xi mu = ((0, 0, -1), (0, 0, 0)).
    let xi = Twist::new(Vec3::new(1.0, 1.0, 0.0), Vec3::new(0.0, 0.0, 0.0));
    let mu = Wrench::new(Vec3::new(1.0, 2.0, 0.0), Vec3::new(0.0, 0.0, 0.0));
    let res = xi.coad(mu);
    assert_eq!(res.torque.x, 0.0);
    assert_eq!(res.torque.y, 0.0);
    assert_eq!(res.torque.z, -1.0);
    assert_eq!(res.force.x, 0.0);
    assert_eq!(res.force.y, 0.0);
    assert_eq!(res.force.z, 0.0);

    let so3_res = So3Tangent::new(Vec3::new(1.0, 1.0, 0.0)).coad(Vec3::new(1.0, 2.0, 0.0));
    assert_eq!([so3_res.x, so3_res.y, so3_res.z], [0.0, 0.0, -1.0]);

    let ref_res = reference::se3_coad([1.0, 1.0, 0.0, 0.0, 0.0, 0.0], [1.0, 2.0, 0.0, 0.0, 0.0, 0.0]);
    assert_eq!(ref_res, [0.0, 0.0, -1.0, 0.0, 0.0, 0.0]);
}

#[test]
fn wrench_pushforward_moment_arm() {
    // KA-193: R = I, p = (1, 0, 0) m, w = (0, (0, 0, 1) N)
    // => Ad*_g w has torque (0, -1, 0) N*m and force (0, 0, 1) N.
    let g = Se3::from_parts_lie(So3::identity(), Vec3::new(1.0, 0.0, 0.0)).expect("valid Se3");
    let w = wrench(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));

    let pushed = g.coadjoint_wrench(w);
    assert_eq!(pushed.torque.x, 0.0);
    assert_eq!(pushed.torque.y, -1.0);
    assert_eq!(pushed.torque.z, 0.0);
    assert_eq!(pushed.force.x, 0.0);
    assert_eq!(pushed.force.y, 0.0);
    assert_eq!(pushed.force.z, 1.0);

    let pushed_mat = g.coadjoint_matrix().apply_wrench(w);
    assert_eq!(pushed_mat.torque.x, 0.0);
    assert_eq!(pushed_mat.torque.y, -1.0);
    assert_eq!(pushed_mat.torque.z, 0.0);
    assert_eq!(pushed_mat.force.x, 0.0);
    assert_eq!(pushed_mat.force.y, 0.0);
    assert_eq!(pushed_mat.force.z, 1.0);
}

#[test]
fn pairing_invariant_small() {
    // Check <Ad*_g w, Ad_g xi> == <w, xi> across all 64 boundary records within 64 ULP
    // of the intermediate contraction scale (handles both O(1) cancelling dot products
    // and 10^6 m moment-arm cancellations honestly without losing sign/algebra sensitivity).
    let fixture = load_fixture();

    for (idx, rec) in fixture.records.iter().take(EXPECTED_BOUNDARY_COUNT).enumerate() {
        let q_g = quat_from_bits(&rec[0..4]);
        let p_g = vec3_from_bits(&rec[4..7]);
        let xi = Twist::new(vec3_from_bits(&rec[38..41]), vec3_from_bits(&rec[41..44]));
        let w = Wrench::new(vec3_from_bits(&rec[50..53]), vec3_from_bits(&rec[53..56]));

        let so3_g = So3::try_from_quat_lie(q_g).expect("valid So3");
        let g = Se3::from_parts_lie(so3_g, p_g).expect("valid Se3");

        let ad_xi = g.adjoint_twist(xi);
        let coad_w = g.coadjoint_wrench(w);
        let lhs = ad_xi.pair(coad_w).value();
        let rhs = xi.pair(w).value();

        let ad_arr = ad_xi.to_array();
        let coad_arr = coad_w.to_array();
        let xi_arr = xi.to_array();
        let w_arr = w.to_array();
        let contraction_scale: f64 = ad_arr
            .iter()
            .zip(coad_arr.iter())
            .map(|(a, b)| (a * b).abs())
            .sum::<f64>()
            + xi_arr
                .iter()
                .zip(w_arr.iter())
                .map(|(a, b)| (a * b).abs())
                .sum::<f64>();
        let tol = 64.0 * f64::EPSILON * contraction_scale.max(1.0);
        let diff = (lhs - rhs).abs();
        assert!(
            diff <= tol,
            "boundary record {idx}: |<Ad*_g w, Ad_g xi> - <w, xi>| = |{lhs} - {rhs}| = {diff} > 64 ULP of contraction scale {contraction_scale} (tol={tol})"
        );
    }
}

/// Test-only `f32` scalar wrapper implementing `fs_ad::Real` to exercise
/// `frob_lie::reference` at single precision without modifying `../frankensim`
/// or introducing a `Se3f32` struct (ADR-MARROW-04).
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct TestF32(f32);

impl std::ops::Add for TestF32 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self { Self(self.0 + rhs.0) }
}
impl std::ops::Sub for TestF32 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self { Self(self.0 - rhs.0) }
}
impl std::ops::Mul for TestF32 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self { Self(self.0 * rhs.0) }
}
impl std::ops::Div for TestF32 {
    type Output = Self;
    fn div(self, rhs: Self) -> Self { Self(self.0 / rhs.0) }
}
impl std::ops::Neg for TestF32 {
    type Output = Self;
    fn neg(self) -> Self { Self(-self.0) }
}

impl Real for TestF32 {
    fn zero() -> Self { Self(0.0) }
    fn one() -> Self { Self(1.0) }
    fn from_f64(v: f64) -> Self { Self(v as f32) }
    fn value(self) -> f64 { f64::from(self.0) }
    fn mul_add(self, a: Self, b: Self) -> Self { Self(self.0.mul_add(a.0, b.0)) }
    fn recip(self) -> Self { Self(self.0.recip()) }
    fn sqrt(self) -> Self { Self(self.0.sqrt()) }
    fn abs(self) -> Self { Self(self.0.abs()) }
    fn exp(self) -> Self { Self(self.0.exp()) }
    fn ln(self) -> Self { Self(self.0.ln()) }
    fn sin(self) -> Self { Self(self.0.sin()) }
    fn cos(self) -> Self { Self(self.0.cos()) }
    fn tanh(self) -> Self { Self(self.0.tanh()) }
    fn asin(self) -> Self { Self(self.0.asin()) }
    fn acos(self) -> Self { Self(self.0.acos()) }
    fn atan(self) -> Self { Self(self.0.atan()) }
    fn atan2(self, x: Self) -> Self { Self(self.0.atan2(x.0)) }
    fn powi(self, n: i32) -> Self { Self(self.0.powi(n)) }
}

fn assert_within_64_ulp(ctx: &str, actual: f64, expected: f64, scale: f64) {
    assert!(
        actual.is_finite() && expected.is_finite(),
        "{ctx}: non-finite value reference={actual} vs fs-ga={expected}"
    );
    let eff_scale = scale.max(actual.abs()).max(expected.abs()).max(1.0e-30);
    let tol = 64.0 * f64::EPSILON * eff_scale;
    let diff = (actual - expected).abs();
    assert!(
        diff <= tol,
        "{ctx}: reference={actual} vs fs-ga={expected} diff={diff} > 64 ULP of scale={eff_scale} (tol={tol}, raw_ulp={})",
        ulp_distance(actual, expected)
    );
}

#[test]
fn generic_matches_fsga() {
    let fixture = load_fixture();

    for (idx, rec) in fixture.records.iter().enumerate() {
        let q_g = [
            f64::from_bits(rec[14]),
            f64::from_bits(rec[15]),
            f64::from_bits(rec[16]),
            f64::from_bits(rec[17]),
        ];
        let p_g = [
            f64::from_bits(rec[4]),
            f64::from_bits(rec[5]),
            f64::from_bits(rec[6]),
        ];
        let q_h = [
            f64::from_bits(rec[26]),
            f64::from_bits(rec[27]),
            f64::from_bits(rec[28]),
            f64::from_bits(rec[29]),
        ];
        let p_h = [
            f64::from_bits(rec[11]),
            f64::from_bits(rec[12]),
            f64::from_bits(rec[13]),
        ];
        let xi_arr = [
            f64::from_bits(rec[38]),
            f64::from_bits(rec[39]),
            f64::from_bits(rec[40]),
            f64::from_bits(rec[41]),
            f64::from_bits(rec[42]),
            f64::from_bits(rec[43]),
        ];
        let eta_arr = [
            f64::from_bits(rec[44]),
            f64::from_bits(rec[45]),
            f64::from_bits(rec[46]),
            f64::from_bits(rec[47]),
            f64::from_bits(rec[48]),
            f64::from_bits(rec[49]),
        ];
        let w_arr = [
            f64::from_bits(rec[50]),
            f64::from_bits(rec[51]),
            f64::from_bits(rec[52]),
            f64::from_bits(rec[53]),
            f64::from_bits(rec[54]),
            f64::from_bits(rec[55]),
        ];

        // SO(3) compose, inverse, rotate
        let so3_gh_ref = reference::so3_compose(q_g, q_h);
        for c in 0..4 {
            assert_within_64_ulp(
                &format!("rec {idx} so3_compose[{c}]"),
                so3_gh_ref[c],
                f64::from_bits(rec[56 + c]),
                1.0,
            );
        }
        let so3_inv_ref = reference::so3_inverse(q_g);
        for c in 0..4 {
            assert_within_64_ulp(
                &format!("rec {idx} so3_inverse[{c}]"),
                so3_inv_ref[c],
                f64::from_bits(rec[60 + c]),
                1.0,
            );
        }
        let ph_scale = (p_h[0].abs() + p_h[1].abs() + p_h[2].abs()).max(1.0);
        let so3_rot_ref = reference::so3_rotate(q_g, p_h);
        for c in 0..3 {
            assert_within_64_ulp(
                &format!("rec {idx} so3_rotate[{c}]"),
                so3_rot_ref[c],
                f64::from_bits(rec[64 + c]),
                ph_scale,
            );
        }

        // SE(3) compose (rotation & translation), inverse (rotation & translation), transform_point
        let so3_g = So3::try_from_quat_lie(quat_from_bits(&rec[0..4])).expect("valid So3");
        let so3_h = So3::try_from_quat_lie(quat_from_bits(&rec[7..11])).expect("valid So3");
        let se3_g = Se3::from_parts_lie(so3_g, vec3_from_bits(&rec[4..7])).expect("valid Se3");
        let se3_h = Se3::from_parts_lie(so3_h, vec3_from_bits(&rec[11..14])).expect("valid Se3");
        let se3_gh = <Se3 as LieGroup>::compose(se3_g, se3_h).expect("compose");
        let se3_g_inv = <Se3 as LieGroup>::inverse(se3_g).expect("inverse");

        let pg_scale = (p_g[0].abs() + p_g[1].abs() + p_g[2].abs()).max(1.0);
        let (gh_q_ref, gh_p_ref) = reference::se3_compose(q_g, p_g, q_h, p_h);
        let gh_q_fs = *se3_gh.rotation().as_quat();
        let gh_p_fs = se3_gh.translation();
        for (c, (r, f)) in gh_q_ref.iter().zip([gh_q_fs.w, gh_q_fs.x, gh_q_fs.y, gh_q_fs.z]).enumerate() {
            assert_within_64_ulp(&format!("rec {idx} se3_compose.q[{c}]"), *r, f, 1.0);
        }
        for (c, (r, f)) in gh_p_ref.iter().zip([gh_p_fs.x, gh_p_fs.y, gh_p_fs.z]).enumerate() {
            assert_within_64_ulp(&format!("rec {idx} se3_compose.p[{c}]"), *r, f, pg_scale + ph_scale);
        }

        let (inv_q_ref, inv_p_ref) = reference::se3_inverse(q_g, p_g);
        let inv_q_fs = *se3_g_inv.rotation().as_quat();
        let inv_p_fs = se3_g_inv.translation();
        for (c, (r, f)) in inv_q_ref.iter().zip([inv_q_fs.w, inv_q_fs.x, inv_q_fs.y, inv_q_fs.z]).enumerate() {
            assert_within_64_ulp(&format!("rec {idx} se3_inverse.q[{c}]"), *r, f, 1.0);
        }
        for (c, (r, f)) in inv_p_ref.iter().zip([inv_p_fs.x, inv_p_fs.y, inv_p_fs.z]).enumerate() {
            assert_within_64_ulp(&format!("rec {idx} se3_inverse.p[{c}]"), *r, f, pg_scale);
        }

        let tx_ref = reference::se3_transform_point(q_g, p_g, p_h);
        for c in 0..3 {
            assert_within_64_ulp(
                &format!("rec {idx} se3_transform_point[{c}]"),
                tx_ref[c],
                f64::from_bits(rec[83 + c]),
                pg_scale + ph_scale,
            );
        }

        // Ad_g xi, Ad*_g w, Ad*_{g^-1} w, [xi, eta], <w, xi>
        let xi_scale = xi_arr.iter().map(|v| v.abs()).sum::<f64>().max(1.0);
        let w_scale = w_arr.iter().map(|v| v.abs()).sum::<f64>().max(1.0);
        let eta_scale = eta_arr.iter().map(|v| v.abs()).sum::<f64>().max(1.0);

        let ad_ref = reference::se3_adjoint_twist(q_g, p_g, xi_arr);
        for c in 0..6 {
            let s = if c < 3 { xi_scale } else { xi_scale * (1.0 + pg_scale) };
            assert_within_64_ulp(&format!("rec {idx} ad_g_xi[{c}]"), ad_ref[c], f64::from_bits(rec[86 + c]), s);
        }
        let coad_ref = reference::se3_coadjoint_wrench(q_g, p_g, w_arr);
        for c in 0..6 {
            let s = if c < 3 { w_scale * (1.0 + pg_scale) } else { w_scale };
            assert_within_64_ulp(&format!("rec {idx} coad_g_w[{c}]"), coad_ref[c], f64::from_bits(rec[98 + c]), s);
        }
        let pb_ref = reference::se3_pullback_wrench(q_g, p_g, w_arr);
        for c in 0..6 {
            let s = if c < 3 { w_scale * (1.0 + pg_scale) } else { w_scale };
            assert_within_64_ulp(&format!("rec {idx} pullback_w[{c}]"), pb_ref[c], f64::from_bits(rec[110 + c]), s);
        }
        let br_ref = reference::se3_bracket(xi_arr, eta_arr);
        for c in 0..6 {
            assert_within_64_ulp(
                &format!("rec {idx} bracket[{c}]"),
                br_ref[c],
                f64::from_bits(rec[122 + c]),
                xi_scale * eta_scale,
            );
        }
        let pair_ref = reference::se3_pair(xi_arr, w_arr);
        assert_within_64_ulp(
            &format!("rec {idx} pairing"),
            pair_ref,
            f64::from_bits(rec[134]),
            xi_scale * w_scale,
        );
    }

    // Verify generic reference instantiates cleanly at fs_ad::Dual64 (exact derivative of pairing wrt p_x).
    let q_const = [
        Dual64::<1>::constant(1.0),
        Dual64::<1>::constant(0.0),
        Dual64::<1>::constant(0.0),
        Dual64::<1>::constant(0.0),
    ];
    let w_const = [0.0, 0.0, 0.0, 0.0, 0.0, 1.0].map(Dual64::<1>::constant);
    let (_, grad_px) = gradient([1.0], |[px]| {
        let p = [px, Dual64::zero(), Dual64::zero()];
        let pushed = reference::se3_coadjoint_wrench(q_const, p, w_const);
        pushed[1] // tau_y = -px * f_z => d(tau_y)/d(px) = -1.0
    });
    assert!((grad_px[0] + 1.0).abs() < 1e-15, "Dual64 derivative expected -1.0, got {}", grad_px[0]);

    // Verify generic reference instantiates cleanly at single precision (TestF32).
    let q32 = [0.5_f64, 0.5, 0.5, 0.5].map(TestF32::from_f64);
    let p32 = [1.0_f64, -2.0, 3.0].map(TestF32::from_f64);
    let w32 = [0.5_f64, -1.0, 2.0, 3.0, -4.0, 5.0].map(TestF32::from_f64);
    let xi32 = [1.0_f64, 0.5, -0.25, -2.0, 1.5, 0.75].map(TestF32::from_f64);
    let p_lhs = reference::se3_pair(
        reference::se3_adjoint_twist(q32, p32, xi32),
        reference::se3_coadjoint_wrench(q32, p32, w32),
    )
    .value();
    let p_rhs = reference::se3_pair(xi32, w32).value();
    assert!(
        (p_lhs - p_rhs).abs() < 1e-4,
        "f32 pairing invariance failed: {p_lhs} vs {p_rhs}"
    );
}

#[test]
fn no_private_vector_types() {
    let src_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut files = Vec::new();
    collect_rs_files(&src_dir, &mut files);
    assert!(!files.is_empty());

    for file in files {
        let content = fs::read_to_string(&file).expect("read src file");
        for (idx, line) in content.lines().enumerate() {
            let trimmed = line.trim_start();
            if trimmed.starts_with("//") {
                continue;
            }
            assert!(
                !trimmed.starts_with("pub struct ") && !trimmed.starts_with("struct "),
                "ADR-MARROW-01 forbids private vector/quaternion/pose structs; found `{trimmed}` at {}:{}",
                file.display(),
                idx + 1
            );
        }
    }
}

fn collect_rs_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let mut entries: Vec<PathBuf> = fs::read_dir(dir)
        .expect("read_dir")
        .map(|e| e.expect("entry").path())
        .collect();
    entries.sort();
    for p in entries {
        if p.is_dir() {
            collect_rs_files(&p, out);
        } else if p.extension().and_then(|s| s.to_str()) == Some("rs") {
            out.push(p);
        }
    }
}

#[test]
fn sign_rule_matches_fsga() {
    let cases = [
        (quat(-0.5, 0.5, 0.5, 0.5), quat(0.5, -0.5, -0.5, -0.5)),
        (quat(0.0, -1.0, 0.0, 0.0), quat(0.0, 1.0, 0.0, 0.0)),
        (quat(0.0, 0.0, -1.0, 0.0), quat(0.0, 0.0, 1.0, 0.0)),
        (quat(-0.0, 0.0, -0.0, -1.0), quat(0.0, 0.0, 0.0, 1.0)),
    ];
    for (raw, want) in cases {
        let got = *So3::try_from_quat_lie(raw).expect("valid unit quat").as_quat();
        assert_eq!(got.w.to_bits(), want.w.to_bits(), "w bits for {raw:?}");
        assert_eq!(got.x.to_bits(), want.x.to_bits(), "x bits for {raw:?}");
        assert_eq!(got.y.to_bits(), want.y.to_bits(), "y bits for {raw:?}");
        assert_eq!(got.z.to_bits(), want.z.to_bits(), "z bits for {raw:?}");

        // Reference canonicalization agrees bit-for-bit too.
        let ref_q = reference::quat_canonicalize([raw.w, raw.x, raw.y, raw.z]);
        assert_eq!(ref_q[0].to_bits(), want.w.to_bits());
        assert_eq!(ref_q[1].to_bits(), want.x.to_bits());
        assert_eq!(ref_q[2].to_bits(), want.y.to_bits());
        assert_eq!(ref_q[3].to_bits(), want.z.to_bits());
    }

    // 1,000 Philox-seeded q and -q pairs yield bit-identical stored So3 & Se3 values.
    let mut stream = StreamKey {
        seed: 0x4D41_5252_4F57_0001,
        kernel: 1,
        tile: 0,
    }
    .stream();

    for i in 0..1000usize {
        let q = loop {
            let w = stream.next_f64() * 2.0 - 1.0;
            let x = stream.next_f64() * 2.0 - 1.0;
            let y = stream.next_f64() * 2.0 - 1.0;
            let z = stream.next_f64() * 2.0 - 1.0;
            let n2 = w * w + x * x + y * y + z * z;
            if n2 > 0.05 && n2 <= 1.0 {
                let inv = 1.0 / n2.sqrt();
                break quat(w * inv, x * inv, y * inv, z * inv);
            }
        };
        let neg_q = quat(-q.w, -q.x, -q.y, -q.z);
        let p = Vec3::new(
            stream.next_f64() * 20.0 - 10.0,
            stream.next_f64() * 20.0 - 10.0,
            stream.next_f64() * 20.0 - 10.0,
        );

        let r_pos = So3::try_from_quat_lie(q).expect("valid q");
        let r_neg = So3::try_from_quat_lie(neg_q).expect("valid -q");
        let qp = *r_pos.as_quat();
        let qn = *r_neg.as_quat();
        assert_eq!(
            [qp.w.to_bits(), qp.x.to_bits(), qp.y.to_bits(), qp.z.to_bits()],
            [qn.w.to_bits(), qn.x.to_bits(), qn.y.to_bits(), qn.z.to_bits()],
            "seeded pair {i}: So3(q) != So3(-q)"
        );

        let g_pos = Se3::from_parts_lie(r_pos, p).expect("valid Se3(q)");
        let g_neg = Se3::from_parts_lie(r_neg, p).expect("valid Se3(-q)");
        for &blade in &MOTOR_EVEN_BLADES {
            assert_eq!(
                g_pos.as_motor().0.0[blade].to_bits(),
                g_neg.as_motor().0.0[blade].to_bits(),
                "seeded pair {i}: Se3(q) != Se3(-q) at blade {blade}"
            );
        }
    }
}

#[test]
fn fallible_ops_refuse() {
    // 1. Quaternion of norm^2 = 1 + 1e-9 (> UNIT_TOLERANCE = 4e-12) must be refused with GaError cause.
    let scale = (1.0_f64 + 1.0e-9_f64).sqrt();
    let bad_q = quat(scale, 0.0, 0.0, 0.0);
    let err = So3::try_from_quat_lie(bad_q)
        .expect_err("non-unit quaternion norm^2 = 1 + 1e-9 must be refused, not renormalized");
    assert!(
        matches!(err, LieError::Host(_)) && err.ga_error().is_some(),
        "expected LieError::Host carrying GaError, got {err:?}"
    );

    // 2. Zero quaternion must be refused.
    let zero_err = So3::try_from_quat_lie(quat(0.0, 0.0, 0.0, 0.0))
        .expect_err("zero quaternion must be refused");
    assert!(zero_err.ga_error().is_some());

    // 3. NaN translation must be refused without panicking.
    let nan_err = Se3::from_parts_lie(So3::identity(), Vec3::new(f64::NAN, 0.0, 0.0))
        .expect_err("NaN translation must be refused");
    assert!(
        matches!(nan_err, LieError::Host(_)) && nan_err.ga_error().is_some(),
        "expected LieError::Host for NaN translation, got {nan_err:?}"
    );

    // 4. Infinite twist in exp must be refused without panicking.
    let inf_twist = Twist::new(Vec3::new(f64::INFINITY, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0));
    let exp_err = <Se3 as LieGroup>::exp(inf_twist)
        .expect_err("non-finite twist in Se3::exp must be refused");
    assert!(exp_err.ga_error().is_some());

    // 5. Deferred BD-011 operations return DeferredOperation without panicking.
    let cay_err = <So3 as LieGroup>::cay(So3Tangent::new(Vec3::new(0.1, 0.2, 0.3)))
        .expect_err("So3::cay is deferred to BD-011");
    assert!(matches!(
        cay_err,
        LieError::DeferredOperation { bead: "BD-011", .. }
    ));
    let se3_cay_err = <Se3 as LieGroup>::cay_inv(Se3::identity())
        .expect_err("Se3::cay_inv is deferred to BD-011");
    assert!(matches!(
        se3_cay_err,
        LieError::DeferredOperation { bead: "BD-011", .. }
    ));
}
