#![forbid(unsafe_code)]

//! Standalone PO-MARROW-01 oracle capture binary (`BD-010`).
//!
//! Links `fs-ga`, `fs-rand`, and `fs-blake3` directly and NEVER depends on
//! `frob-lie`. Draws 1,000 test records (records 0..63 deterministic boundary
//! set; records 64..999 Philox-seeded with `0x4D41_5252_4F57_0001`), evaluates
//! every SO(3) and SE(3) operation directly through `fs-ga`, and emits a
//! self-authenticating strict-TOML oracle fixture (`kind = "oracle"`) with raw
//! IEEE-754 binary64 bit patterns.

use std::env;
use std::fmt::Write as _;
use std::fs;
use std::io::{self, Write as _};
use std::path::PathBuf;
use std::process::{self, Command};

use fs_blake3::Blake3;
use fs_ga::{Motor, Quat, Se3, So3, Twist, Vec3, Wrench};
use fs_rand::{Stream, StreamKey};

const MOTOR_EVEN_BLADES: [usize; 8] = [0, 3, 5, 6, 9, 10, 12, 15];

const SEED: u64 = 0x4D41_5252_4F57_0001;
const RECORD_COUNT: usize = 1000;
const BOUNDARY_COUNT: usize = 64;
const FIELDS_PER_RECORD: usize = 135;

const DEFAULT_ORACLE_ID: &str = "frankensim-referee";
const DEFAULT_ORACLE_PIN: &str = "d02ad9c170d5932667d15359cd362d3198ac4806";
const DEFAULT_DATE: &str = "2026-11-24";
const DEFAULT_FRANKENSIM_COMMIT: &str = "b871716cdca9f0a97dbe4be49895d4fcaa9d3fd3";
const FS_GA_VERSION: &str = "0.0.1";
const SCRIPT_REL_PATH: &str = "tools/oracle/marrow_fsga_capture/src/main.rs";
const ZERO_SHA256: &str = "0000000000000000000000000000000000000000000000000000000000000000";

const SELF_SOURCE_BYTES: &[u8] = include_bytes!("main.rs");

struct RawCase {
    q_g: Quat,
    p_g: Vec3,
    q_h: Quat,
    p_h: Vec3,
    xi: Twist,
    eta: Twist,
    w: Wrench,
}

fn main() {
    if let Err(msg) = run() {
        eprintln!("marrow_fsga_capture error: {msg}");
        process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let mut out_path: Option<PathBuf> = None;
    let mut to_stdout = false;
    let mut records_blake3_only = false;
    let mut frankensim_commit: Option<String> = None;
    let mut oracle_pin = DEFAULT_ORACLE_PIN.to_owned();
    let mut date = DEFAULT_DATE.to_owned();

    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--out" => {
                let p = args.next().ok_or_else(|| "--out requires a path".to_owned())?;
                out_path = Some(PathBuf::from(p));
            }
            "--stdout" => {
                to_stdout = true;
            }
            "--records-blake3" => {
                records_blake3_only = true;
            }
            "--frankensim-commit" => {
                let c = args
                    .next()
                    .ok_or_else(|| "--frankensim-commit requires a commit SHA".to_owned())?;
                frankensim_commit = Some(c);
            }
            "--oracle-pin" => {
                oracle_pin = args
                    .next()
                    .ok_or_else(|| "--oracle-pin requires a pin string".to_owned())?;
            }
            "--date" => {
                date = args
                    .next()
                    .ok_or_else(|| "--date requires YYYY-MM-DD".to_owned())?;
            }
            "--help" | "-h" => {
                println!(
                    "Usage: marrow_fsga_capture [--out <path> | --stdout | --records-blake3] [--frankensim-commit <sha>] [--oracle-pin <pin>] [--date <YYYY-MM-DD>]"
                );
                return Ok(());
            }
            other => {
                return Err(format!("unknown argument `{other}`"));
            }
        }
    }

    let cases = build_all_cases();
    if cases.len() != RECORD_COUNT {
        return Err(format!(
            "internal case count mismatch: expected {RECORD_COUNT}, got {}",
            cases.len()
        ));
    }

    let mut encoded_records: Vec<String> = Vec::with_capacity(RECORD_COUNT);
    let mut blake3 = Blake3::new();
    for (idx, case) in cases.iter().enumerate() {
        let bits = evaluate_case(idx, case)?;
        let line = encode_record_hex(idx, &bits);
        blake3.update(line.as_bytes());
        blake3.update(b"\n");
        encoded_records.push(line);
    }
    let records_blake3 = blake3.finalize().to_hex();

    if records_blake3_only {
        println!("{records_blake3}");
        return Ok(());
    }

    let resolved_commit = frankensim_commit
        .or_else(detect_frankensim_head)
        .unwrap_or_else(|| DEFAULT_FRANKENSIM_COMMIT.to_owned());
    let script_sha256 = sha256_hex(SELF_SOURCE_BYTES);

    let fixture_text = render_fixture_toml(
        &oracle_pin,
        &date,
        &script_sha256,
        &resolved_commit,
        &records_blake3,
        &encoded_records,
    );

    if let Some(path) = out_path {
        if let Some(parent) = path.parent()
            && !parent.as_os_str().is_empty()
        {
            fs::create_dir_all(parent)
                .map_err(|e| format!("failed creating {}: {e}", parent.display()))?;
        }
        fs::write(&path, fixture_text.as_bytes())
            .map_err(|e| format!("failed writing {}: {e}", path.display()))?;
    } else if to_stdout {
        let mut stdout = io::stdout().lock();
        stdout
            .write_all(fixture_text.as_bytes())
            .map_err(|e| format!("stdout write failed: {e}"))?;
    } else {
        return Err("specify one of --out <path>, --stdout, or --records-blake3".to_owned());
    }

    Ok(())
}

fn detect_frankensim_head() -> Option<String> {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let frankensim_dir = manifest_dir.join("../../../../frankensim");
    let output = Command::new("git")
        .arg("-C")
        .arg(&frankensim_dir)
        .arg("rev-parse")
        .arg("HEAD")
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let rev = String::from_utf8(output.stdout).ok()?.trim().to_owned();
    if rev.len() == 40 && rev.bytes().all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f')) {
        Some(rev)
    } else {
        None
    }
}

fn evaluate_case(idx: usize, case: &RawCase) -> Result<[u64; FIELDS_PER_RECORD], String> {
    let so3_g = So3::try_from_quat(case.q_g)
        .map_err(|e| format!("record {idx}: So3::try_from_quat(q_g) failed: {e:?}"))?;
    let so3_h = So3::try_from_quat(case.q_h)
        .map_err(|e| format!("record {idx}: So3::try_from_quat(q_h) failed: {e:?}"))?;

    let se3_g = Se3::from_parts(so3_g, case.p_g)
        .map_err(|e| format!("record {idx}: Se3::from_parts(g) failed: {e:?}"))?;
    let se3_h = Se3::from_parts(so3_h, case.p_h)
        .map_err(|e| format!("record {idx}: Se3::from_parts(h) failed: {e:?}"))?;

    let so3_gh = so3_g
        .compose(so3_h)
        .map_err(|e| format!("record {idx}: So3::compose failed: {e:?}"))?;
    let so3_g_inv = so3_g.inverse();
    let so3_rot_ph = so3_g
        .rotate(case.p_h)
        .map_err(|e| format!("record {idx}: So3::rotate failed: {e:?}"))?;

    let se3_gh = se3_g
        .compose(se3_h)
        .map_err(|e| format!("record {idx}: Se3::compose failed: {e:?}"))?;
    let se3_g_inv = se3_g
        .inverse()
        .map_err(|e| format!("record {idx}: Se3::inverse failed: {e:?}"))?;
    let se3_tx_ph = se3_g
        .transform_point(case.p_h)
        .map_err(|e| format!("record {idx}: Se3::transform_point failed: {e:?}"))?;

    let ad_g_xi = case.xi.transform_by(&se3_g);
    let ad_g_mat_xi = se3_g.adjoint().apply_twist(case.xi);
    let coad_g_w = case.w.transform_by(&se3_g);
    let coad_g_mat_w = se3_g.coadjoint().apply_wrench(case.w);
    let pullback_w = case.w.transform_by(&se3_g_inv);
    let pullback_mat_w = se3_g_inv.coadjoint().apply_wrench(case.w);
    let bracket_xi_eta = case.xi.bracket(case.eta);
    let ad_mat_xi_eta = case.xi.ad().apply_twist(case.eta);
    let pairing_w_xi = case.w.pairing(case.xi);

    let mut out = [0u64; FIELDS_PER_RECORD];
    let mut k = 0usize;
    let mut push = |v: f64| {
        out[k] = v.to_bits();
        k += 1;
    };

    push_quat(&mut push, case.q_g);
    push_vec3(&mut push, case.p_g);
    push_quat(&mut push, case.q_h);
    push_vec3(&mut push, case.p_h);

    push_quat(&mut push, *so3_g.as_quat());
    push_motor(&mut push, *se3_g.as_motor());
    push_quat(&mut push, *so3_h.as_quat());
    push_motor(&mut push, *se3_h.as_motor());

    push_twist(&mut push, case.xi);
    push_twist(&mut push, case.eta);
    push_wrench(&mut push, case.w);

    push_quat(&mut push, *so3_gh.as_quat());
    push_quat(&mut push, *so3_g_inv.as_quat());
    push_vec3(&mut push, so3_rot_ph);

    push_motor(&mut push, *se3_gh.as_motor());
    push_motor(&mut push, *se3_g_inv.as_motor());
    push_vec3(&mut push, se3_tx_ph);

    push_twist(&mut push, ad_g_xi);
    push_twist(&mut push, ad_g_mat_xi);
    push_wrench(&mut push, coad_g_w);
    push_wrench(&mut push, coad_g_mat_w);
    push_wrench(&mut push, pullback_w);
    push_wrench(&mut push, pullback_mat_w);
    push_twist(&mut push, bracket_xi_eta);
    push_twist(&mut push, ad_mat_xi_eta);
    push(pairing_w_xi);

    debug_assert_eq!(k, FIELDS_PER_RECORD);
    Ok(out)
}

fn push_quat(push: &mut impl FnMut(f64), q: Quat) {
    push(q.w);
    push(q.x);
    push(q.y);
    push(q.z);
}

fn push_vec3(push: &mut impl FnMut(f64), v: Vec3) {
    push(v.x);
    push(v.y);
    push(v.z);
}

fn push_motor(push: &mut impl FnMut(f64), m: Motor) {
    for &idx in &MOTOR_EVEN_BLADES {
        push(m.0.0[idx]);
    }
}

fn push_twist(push: &mut impl FnMut(f64), t: Twist) {
    push_vec3(push, t.angular);
    push_vec3(push, t.linear);
}

fn push_wrench(push: &mut impl FnMut(f64), w: Wrench) {
    push_vec3(push, w.torque);
    push_vec3(push, w.force);
}

fn encode_record_hex(idx: usize, bits: &[u64; FIELDS_PER_RECORD]) -> String {
    let mut s = String::with_capacity(5 + FIELDS_PER_RECORD * 16);
    let _ = write!(s, "{idx:04}:");
    for &word in bits {
        let _ = write!(s, "{word:016x}");
    }
    s
}

fn quat(w: f64, x: f64, y: f64, z: f64) -> Quat {
    Quat { w, x, y, z }
}

fn q_neg(q: Quat) -> Quat {
    quat(-q.w, -q.x, -q.y, -q.z)
}

fn unit_quat_from_raw(w: f64, x: f64, y: f64, z: f64) -> Quat {
    let n = (w * w + x * x + y * y + z * z).sqrt();
    quat(w / n, x / n, y / n, z / n)
}

fn build_all_cases() -> Vec<RawCase> {
    let mut cases = build_boundary_cases();
    assert_eq!(cases.len(), BOUNDARY_COUNT);

    let mut stream = StreamKey {
        seed: SEED,
        kernel: 0,
        tile: 0,
    }
    .stream();

    for idx in BOUNDARY_COUNT..RECORD_COUNT {
        let q_g = draw_unit_quat(&mut stream);
        let q_h = draw_unit_quat(&mut stream);
        let p_scale = if idx % 32 == 0 { 1.0e4 } else { 10.0 };
        let p_g = draw_vec3(&mut stream, p_scale);
        let p_h = draw_vec3(&mut stream, p_scale);
        let xi = Twist::new(draw_vec3(&mut stream, 5.0), draw_vec3(&mut stream, 10.0));
        let eta = Twist::new(draw_vec3(&mut stream, 5.0), draw_vec3(&mut stream, 10.0));
        let w = Wrench::new(draw_vec3(&mut stream, 10.0), draw_vec3(&mut stream, 10.0));
        cases.push(RawCase {
            q_g,
            p_g,
            q_h,
            p_h,
            xi,
            eta,
            w,
        });
    }
    cases
}

fn draw_uniform_sym(stream: &mut Stream, scale: f64) -> f64 {
    (stream.next_f64() * 2.0 - 1.0) * scale
}

fn draw_vec3(stream: &mut Stream, scale: f64) -> Vec3 {
    Vec3::new(
        draw_uniform_sym(stream, scale),
        draw_uniform_sym(stream, scale),
        draw_uniform_sym(stream, scale),
    )
}

fn draw_unit_quat(stream: &mut Stream) -> Quat {
    loop {
        let w = draw_uniform_sym(stream, 1.0);
        let x = draw_uniform_sym(stream, 1.0);
        let y = draw_uniform_sym(stream, 1.0);
        let z = draw_uniform_sym(stream, 1.0);
        let n2 = w * w + x * x + y * y + z * z;
        if n2 > 0.05 && n2 <= 1.0 {
            let inv = 1.0 / n2.sqrt();
            let q = quat(w * inv, x * inv, y * inv, z * inv);
            return if (stream.next_u64() & 1) == 1 {
                q_neg(q)
            } else {
                q
            };
        }
    }
}

fn mk(q_g: Quat, p_g: Vec3, q_h: Quat, p_h: Vec3, xi: Twist, eta: Twist, w: Wrench) -> RawCase {
    RawCase {
        q_g,
        p_g,
        q_h,
        p_h,
        xi,
        eta,
        w,
    }
}

fn build_boundary_cases() -> Vec<RawCase> {
    let qi = Quat::identity();
    let z3 = Vec3::new(0.0, 0.0, 0.0);
    let nz3 = Vec3::new(-0.0, -0.0, -0.0);
    let ex = Vec3::new(1.0, 0.0, 0.0);
    let ey = Vec3::new(0.0, 1.0, 0.0);
    let ez = Vec3::new(0.0, 0.0, 1.0);

    let xi_std = Twist::new(Vec3::new(1.0, -2.0, 3.0), Vec3::new(0.5, -1.5, 2.5));
    let eta_std = Twist::new(Vec3::new(-0.25, 1.75, -0.5), Vec3::new(2.0, 0.0, -3.0));
    let w_std = Wrench::new(Vec3::new(1.5, -0.75, 2.25), Vec3::new(-3.0, 4.0, -1.0));

    let q_half_neg = quat(-0.5, 0.5, 0.5, 0.5);
    let q_half_pos = quat(0.5, -0.5, -0.5, -0.5);
    let q_all_pos = quat(0.5, 0.5, 0.5, 0.5);
    let q_all_neg = quat(-0.5, -0.5, -0.5, -0.5);
    let q_345_pos = quat(0.6, 0.8, 0.0, 0.0);
    let q_345_neg = quat(-0.6, -0.8, 0.0, 0.0);
    let q_tie_xy_pos = quat(0.0, 0.6, -0.8, 0.0);
    let q_tie_xy_neg = quat(0.0, -0.6, 0.8, 0.0);
    let q_gen_a = unit_quat_from_raw(0.3, -0.4, 0.5, -0.7);
    let q_gen_b = unit_quat_from_raw(-0.2, 0.6, -0.1, 0.75);

    let a30: f64 = 2.0_f64.powi(-30);
    let a52: f64 = 2.0_f64.powi(-52);
    let q_a30_x = Quat::from_axis_angle(ex, a30);
    let q_a30_nx = Quat::from_axis_angle(ex, -a30);
    let q_a30_y = Quat::from_axis_angle(ey, a30);
    let q_a30_ny = Quat::from_axis_angle(ey, -a30);
    let q_a30_z = Quat::from_axis_angle(ez, a30);
    let q_a30_nz = Quat::from_axis_angle(ez, -a30);
    let q_a52_x = Quat::from_axis_angle(ex, a52);
    let q_a52_nx = Quat::from_axis_angle(ex, -a52);
    let q_a52_y = Quat::from_axis_angle(ey, a52);
    let q_a52_ny = Quat::from_axis_angle(ey, -a52);
    let q_a52_z = Quat::from_axis_angle(ez, a52);
    let q_a52_nz = Quat::from_axis_angle(ez, -a52);

    let big_x = Vec3::new(1.0e6, 0.0, 0.0);
    let big_ny = Vec3::new(0.0, -1.0e6, 0.0);
    let big_z = Vec3::new(0.0, 0.0, 1.0e6);
    let big_diag = Vec3::new(1.0e6, -1.0e6, 1.0e6);
    let big_diag_neg = Vec3::new(-1.0e6, 1.0e6, -1.0e6);

    let q_90x = Quat::from_axis_angle(ex, core::f64::consts::FRAC_PI_2);
    let q_90y = Quat::from_axis_angle(ey, core::f64::consts::FRAC_PI_2);
    let q_90z = Quat::from_axis_angle(ez, core::f64::consts::FRAC_PI_2);

    let mut v = Vec::with_capacity(BOUNDARY_COUNT);

    // 0: pure identity
    v.push(mk(qi, z3, qi, z3, xi_std, eta_std, w_std));

    // 1..=12: pi rotations about +-x, +-y, +-z (w = 0 tie-break & -0.0 variants)
    v.push(mk(quat(0.0, 1.0, 0.0, 0.0), ex, qi, ey, xi_std, eta_std, w_std));
    v.push(mk(quat(0.0, -1.0, 0.0, 0.0), ex, qi, ey, xi_std, eta_std, w_std));
    v.push(mk(quat(-0.0, 1.0, 0.0, 0.0), ey, qi, ez, xi_std, eta_std, w_std));
    v.push(mk(quat(-0.0, -1.0, 0.0, 0.0), ey, qi, ez, xi_std, eta_std, w_std));
    v.push(mk(quat(0.0, 0.0, 1.0, 0.0), ez, qi, ex, xi_std, eta_std, w_std));
    v.push(mk(quat(0.0, 0.0, -1.0, 0.0), ez, qi, ex, xi_std, eta_std, w_std));
    v.push(mk(quat(-0.0, 0.0, -1.0, 0.0), ex, q_90z, ey, xi_std, eta_std, w_std));
    v.push(mk(quat(0.0, -0.0, -1.0, -0.0), ex, q_90z, ey, xi_std, eta_std, w_std));
    v.push(mk(quat(0.0, 0.0, 0.0, 1.0), Vec3::new(1.0, 2.0, 3.0), q_90x, ez, xi_std, eta_std, w_std));
    v.push(mk(quat(0.0, 0.0, 0.0, -1.0), Vec3::new(1.0, 2.0, 3.0), q_90x, ez, xi_std, eta_std, w_std));
    v.push(mk(quat(-0.0, 0.0, -0.0, -1.0), nz3, quat(-0.0, 0.0, -0.0, -1.0), nz3, xi_std, eta_std, w_std));
    v.push(mk(quat(-0.0, -0.0, -0.0, -1.0), nz3, quat(0.0, 0.0, 0.0, 1.0), z3, xi_std, eta_std, w_std));

    // 13..=22: explicit (q, -q) double-cover pairs (5 pairs)
    v.push(mk(q_half_neg, Vec3::new(1.0, -2.0, 0.5), q_gen_a, Vec3::new(-1.0, 0.5, 2.0), xi_std, eta_std, w_std));
    v.push(mk(q_half_pos, Vec3::new(1.0, -2.0, 0.5), q_neg(q_gen_a), Vec3::new(-1.0, 0.5, 2.0), xi_std, eta_std, w_std));
    v.push(mk(q_all_pos, Vec3::new(-3.0, 1.0, 4.0), q_gen_b, Vec3::new(2.0, -1.0, 0.25), xi_std, eta_std, w_std));
    v.push(mk(q_all_neg, Vec3::new(-3.0, 1.0, 4.0), q_neg(q_gen_b), Vec3::new(2.0, -1.0, 0.25), xi_std, eta_std, w_std));
    v.push(mk(q_345_pos, Vec3::new(0.25, -0.5, 0.75), q_90y, Vec3::new(1.0, 1.0, 1.0), xi_std, eta_std, w_std));
    v.push(mk(q_345_neg, Vec3::new(0.25, -0.5, 0.75), q_neg(q_90y), Vec3::new(1.0, 1.0, 1.0), xi_std, eta_std, w_std));
    v.push(mk(q_tie_xy_pos, Vec3::new(2.0, 3.0, -1.0), q_tie_xy_neg, Vec3::new(-2.0, 1.0, 3.0), xi_std, eta_std, w_std));
    v.push(mk(q_tie_xy_neg, Vec3::new(2.0, 3.0, -1.0), q_tie_xy_pos, Vec3::new(-2.0, 1.0, 3.0), xi_std, eta_std, w_std));
    v.push(mk(qi, Vec3::new(0.5, 1.5, -2.5), q_gen_a, Vec3::new(-0.5, 2.5, 1.5), xi_std, eta_std, w_std));
    v.push(mk(q_neg(qi), Vec3::new(0.5, 1.5, -2.5), q_neg(q_gen_a), Vec3::new(-0.5, 2.5, 1.5), xi_std, eta_std, w_std));

    // 23..=34: small angles 2^-30 and 2^-52 about +-x, +-y, +-z (12 records)
    v.push(mk(q_a30_x, ex, q_a30_nx, ey, xi_std, eta_std, w_std));
    v.push(mk(q_a30_nx, ey, q_a30_x, ez, xi_std, eta_std, w_std));
    v.push(mk(q_a30_y, ez, q_a30_ny, ex, xi_std, eta_std, w_std));
    v.push(mk(q_a30_ny, ex, q_a30_y, ey, xi_std, eta_std, w_std));
    v.push(mk(q_a30_z, ey, q_a30_nz, ez, xi_std, eta_std, w_std));
    v.push(mk(q_a30_nz, ez, q_a30_z, ex, xi_std, eta_std, w_std));
    v.push(mk(q_a52_x, ex, q_a52_nx, ey, xi_std, eta_std, w_std));
    v.push(mk(q_a52_nx, ey, q_a52_x, ez, xi_std, eta_std, w_std));
    v.push(mk(q_a52_y, ez, q_a52_ny, ex, xi_std, eta_std, w_std));
    v.push(mk(q_a52_ny, ex, q_a52_y, ey, xi_std, eta_std, w_std));
    v.push(mk(q_a52_z, ey, q_a52_nz, ez, xi_std, eta_std, w_std));
    v.push(mk(q_a52_nz, ez, q_a52_z, ex, xi_std, eta_std, w_std));

    // 35..=44: large translations of 10^6 m (10 records)
    v.push(mk(qi, big_x, qi, z3, xi_std, eta_std, w_std));
    v.push(mk(qi, big_ny, q_90z, big_z, xi_std, eta_std, w_std));
    v.push(mk(q_90x, big_z, q_90y, big_x, xi_std, eta_std, w_std));
    v.push(mk(q_half_pos, big_diag, q_half_neg, big_diag_neg, xi_std, eta_std, w_std));
    v.push(mk(q_gen_a, big_diag, q_gen_b, ex, xi_std, eta_std, w_std));
    v.push(mk(quat(0.0, 1.0, 0.0, 0.0), big_x, quat(0.0, 0.0, -1.0, 0.0), big_ny, xi_std, eta_std, w_std));
    v.push(mk(q_a30_x, big_diag, q_a52_z, big_z, xi_std, eta_std, w_std));
    v.push(mk(q_345_neg, big_diag_neg, q_345_pos, big_diag, xi_std, eta_std, w_std));
    v.push(mk(q_90y, big_x, qi, big_diag, Twist::new(ex, z3), Twist::new(ey, ez), Wrench::new(z3, ez)));
    v.push(mk(q_gen_b, Vec3::new(-1.0e6, 0.0, 1.0e6), q_gen_a, Vec3::new(0.0, 1.0e6, -1.0e6), xi_std, eta_std, w_std));

    // 45..=54: -0.0 components in quaternions, translations, twists, and wrenches (10 records)
    v.push(mk(
        quat(1.0, -0.0, -0.0, -0.0),
        nz3,
        quat(-1.0, -0.0, 0.0, -0.0),
        nz3,
        Twist::new(nz3, nz3),
        Twist::new(nz3, nz3),
        Wrench::new(nz3, nz3),
    ));
    v.push(mk(
        quat(-1.0, 0.0, -0.0, 0.0),
        Vec3::new(-0.0, 1.0, -0.0),
        quat(1.0, -0.0, 0.0, -0.0),
        Vec3::new(0.0, -0.0, -1.0),
        Twist::new(Vec3::new(-0.0, 2.0, -0.0), Vec3::new(-1.0, -0.0, 3.0)),
        Twist::new(Vec3::new(1.0, -0.0, -2.0), Vec3::new(-0.0, -0.0, 1.0)),
        Wrench::new(Vec3::new(-0.0, -0.0, 4.0), Vec3::new(2.0, -0.0, -0.0)),
    ));
    v.push(mk(
        quat(0.6, -0.0, 0.8, -0.0),
        Vec3::new(-0.0, -0.0, 5.0),
        quat(-0.6, -0.0, -0.8, -0.0),
        Vec3::new(5.0, -0.0, -0.0),
        Twist::new(Vec3::new(-0.0, -0.0, 1.0), nz3),
        Twist::new(nz3, Vec3::new(1.0, -0.0, -0.0)),
        Wrench::new(nz3, Vec3::new(-0.0, 1.0, -0.0)),
    ));
    v.push(mk(
        quat(-0.0, 0.0, 0.6, -0.8),
        nz3,
        quat(0.0, -0.0, -0.6, 0.8),
        z3,
        Twist::new(ex, nz3),
        Twist::new(nz3, ey),
        Wrench::new(ez, nz3),
    ));
    v.push(mk(
        quat(0.0, -0.0, 1.0, -0.0),
        Vec3::new(-0.0, -2.0, -0.0),
        quat(-0.0, -0.0, -1.0, -0.0),
        Vec3::new(2.0, -0.0, -0.0),
        xi_std,
        eta_std,
        w_std,
    ));
    v.push(mk(
        q_a52_x,
        nz3,
        q_neg(q_a52_x),
        nz3,
        Twist::new(Vec3::new(-0.0, 0.0, -0.0), Vec3::new(0.0, -0.0, 0.0)),
        eta_std,
        w_std,
    ));
    v.push(mk(
        q_a30_y,
        Vec3::new(-1.0e6, -0.0, -0.0),
        q_neg(q_a30_y),
        Vec3::new(-0.0, -0.0, 1.0e6),
        Twist::new(nz3, ex),
        Twist::new(ey, nz3),
        Wrench::new(nz3, ez),
    ));
    v.push(mk(
        quat(0.0, 0.0, -0.0, 1.0),
        Vec3::new(0.0, -0.0, 0.0),
        quat(-0.0, -0.0, 0.0, -1.0),
        Vec3::new(-0.0, 0.0, -0.0),
        xi_std,
        Twist::new(nz3, nz3),
        Wrench::new(Vec3::new(-0.0, 0.0, -0.0), Vec3::new(0.0, -0.0, 0.0)),
    ));
    v.push(mk(
        q_half_neg,
        nz3,
        q_half_pos,
        z3,
        Twist::new(Vec3::new(-0.0, 1.0, -0.0), Vec3::new(0.0, -0.0, -1.0)),
        Twist::new(Vec3::new(1.0, -0.0, 0.0), Vec3::new(-0.0, 1.0, -0.0)),
        Wrench::new(Vec3::new(1.0, -0.0, -0.0), Vec3::new(-0.0, -0.0, 1.0)),
    ));
    v.push(mk(
        q_gen_a,
        Vec3::new(-0.0, 3.5, -0.0),
        q_gen_b,
        Vec3::new(-2.5, -0.0, 1.5),
        Twist::new(Vec3::new(-0.0, -0.5, 0.25), Vec3::new(1.25, -0.0, -0.75)),
        eta_std,
        Wrench::new(Vec3::new(-0.0, 2.0, -1.0), Vec3::new(0.5, -0.0, 3.0)),
    ));

    // 55..=63: known-answer & structured edge combinations (9 records)
    // 55: KA-193 moment arm: R = I, p = (1,0,0), w = (0, (0,0,1)) -> tau = (0,-1,0)
    v.push(mk(
        qi,
        ex,
        qi,
        z3,
        Twist::new(ex, ey),
        Twist::new(ey, ez),
        Wrench::new(z3, ez),
    ));
    // 56: KA-229 coad/bracket components
    v.push(mk(
        qi,
        z3,
        q_90z,
        ex,
        Twist::new(Vec3::new(1.0, 1.0, 0.0), z3),
        Twist::new(Vec3::new(0.0, 1.0, 1.0), Vec3::new(1.0, 0.0, -1.0)),
        Wrench::new(Vec3::new(1.0, 2.0, 0.0), z3),
    ));
    // 57..=59: 90-deg rotations about x, y, z with integer vectors
    v.push(mk(q_90x, Vec3::new(1.0, 2.0, 3.0), q_90y, Vec3::new(-1.0, 0.0, 2.0), xi_std, eta_std, w_std));
    v.push(mk(q_90y, Vec3::new(-2.0, 3.0, 1.0), q_90z, Vec3::new(3.0, -1.0, 0.0), xi_std, eta_std, w_std));
    v.push(mk(q_90z, Vec3::new(3.0, -1.0, -2.0), q_90x, Vec3::new(0.0, 2.0, -3.0), xi_std, eta_std, w_std));
    // 60..=63: mixed scale combinations & opposite-sign diagonal quaternions
    v.push(mk(
        quat(0.5, -0.5, 0.5, -0.5),
        Vec3::new(1.0e-6, -2.0e-6, 3.0e-6),
        quat(-0.5, 0.5, -0.5, 0.5),
        Vec3::new(1.0e3, -2.0e3, 3.0e3),
        xi_std,
        eta_std,
        w_std,
    ));
    v.push(mk(
        quat(-0.5, 0.5, -0.5, 0.5),
        Vec3::new(1.0e3, -2.0e3, 3.0e3),
        quat(0.5, -0.5, 0.5, -0.5),
        Vec3::new(1.0e-6, -2.0e-6, 3.0e-6),
        Twist::new(Vec3::new(1.0e-3, -2.0e-3, 3.0e-3), Vec3::new(4.0, -5.0, 6.0)),
        Twist::new(Vec3::new(-3.0, 2.0, -1.0), Vec3::new(1.0e-3, 2.0e-3, -3.0e-3)),
        Wrench::new(Vec3::new(7.0, -8.0, 9.0), Vec3::new(-1.0e-3, 2.0e-3, -3.0e-3)),
    ));
    v.push(mk(
        unit_quat_from_raw(1.0, 2.0, 3.0, 4.0),
        Vec3::new(-50.0, 125.0, -250.0),
        unit_quat_from_raw(-4.0, 3.0, -2.0, 1.0),
        Vec3::new(75.0, -150.0, 300.0),
        Twist::new(Vec3::new(2.0, -3.0, 5.0), Vec3::new(-7.0, 11.0, -13.0)),
        Twist::new(Vec3::new(-1.0, 4.0, -2.0), Vec3::new(6.0, -8.0, 10.0)),
        Wrench::new(Vec3::new(3.0, -5.0, 7.0), Vec3::new(-2.0, 4.0, -6.0)),
    ));
    v.push(mk(
        unit_quat_from_raw(0.0, 1.0, 1.0, 1.0),
        Vec3::new(10.0, -20.0, 30.0),
        unit_quat_from_raw(0.0, -1.0, -1.0, -1.0),
        Vec3::new(-10.0, 20.0, -30.0),
        xi_std,
        eta_std,
        w_std,
    ));

    v
}

fn render_fixture_toml(
    oracle_pin: &str,
    date: &str,
    script_sha256: &str,
    frankensim_commit: &str,
    records_blake3: &str,
    encoded_records: &[String],
) -> String {
    let mut body = String::with_capacity(encoded_records.len() * 2180 + 2048);
    body.push_str("# PO-MARROW-01 bit-parity oracle fixture (BD-010).\n");
    body.push_str("# Captured directly from fs-ga via tools/oracle/marrow_fsga_capture (never frob-lie).\n");
    body.push_str("# Strict frob_core::registry::toml subset; self-authenticating via bytes_sha256.\n\n");
    let _ = writeln!(body, "kind = \"oracle\"");
    let _ = writeln!(body, "oracle = \"{DEFAULT_ORACLE_ID}\"");
    let _ = writeln!(body, "pin = \"{oracle_pin}\"");
    let _ = writeln!(body, "captured = \"{date}\"");
    let _ = writeln!(
        body,
        "script = {{ path = \"{SCRIPT_REL_PATH}\", sha256 = \"{script_sha256}\" }}"
    );
    body.push_str("inputs = []\n");
    body.push_str("units = \"SI (rad, m, rad/s, m/s, N*m, N, W); raw IEEE-754 binary64 hex\"\n");
    body.push_str("tolerance_note = \"PO-MARROW-01 bit-exact (0 ULP) parity between fs-ga capture and frob-lie f64 host path\"\n");
    let _ = writeln!(body, "bytes_sha256 = \"{ZERO_SHA256}\"");
    body.push_str("status = \"active\"\n\n");
    body.push_str("[payload]\n");
    let _ = writeln!(body, "frankensim_commit = \"{frankensim_commit}\"");
    let _ = writeln!(body, "fs_ga_version = \"{FS_GA_VERSION}\"");
    let _ = writeln!(body, "date = \"{date}\"");
    let _ = writeln!(body, "seed = \"0x{SEED:016x}\"");
    let _ = writeln!(body, "record_count = {RECORD_COUNT}");
    let _ = writeln!(body, "boundary_count = {BOUNDARY_COUNT}");
    let _ = writeln!(body, "fields_per_record = {FIELDS_PER_RECORD}");
    let _ = writeln!(body, "records_blake3 = \"{records_blake3}\"");
    body.push_str("records = [\n");
    for rec in encoded_records {
        let _ = writeln!(body, "    \"{rec}\",");
    }
    body.push_str("]\n");

    let digest = sha256_hex(body.as_bytes());
    replace_bytes_sha256_line(&body, &digest)
}

fn replace_bytes_sha256_line(content: &str, digest: &str) -> String {
    let mut out = String::with_capacity(content.len());
    let mut replaced = false;
    for chunk in content.split_inclusive('\n') {
        if !replaced && chunk.trim_start().starts_with("bytes_sha256")
            && let Some(first_q) = chunk.find('"')
            && let Some(rel_end) = chunk[first_q + 1..].find('"')
        {
            let end_q = first_q + 1 + rel_end;
            out.push_str(&chunk[..=first_q]);
            out.push_str(digest);
            out.push_str(&chunk[end_q..]);
            replaced = true;
            continue;
        }
        out.push_str(chunk);
    }
    out
}

// Pure-std FIPS 180-4 SHA-256 (identical algorithm to frob-conformance::e2e::sha256).
const SHA_H0: [u32; 8] = [
    0x6a09_e667, 0xbb67_ae85, 0x3c6e_f372, 0xa54f_f53a,
    0x510e_527f, 0x9b05_688c, 0x1f83_d9ab, 0x5be0_cd19,
];
const SHA_K: [u32; 64] = [
    0x428a_2f98, 0x7137_4491, 0xb5c0_fbcf, 0xe9b5_dba5, 0x3956_c25b, 0x59f1_11f1, 0x923f_82a4, 0xab1c_5ed5,
    0xd807_aa98, 0x1283_5b01, 0x2431_85be, 0x550c_7dc3, 0x72be_5d74, 0x80de_b1fe, 0x9bdc_06a7, 0xc19b_f174,
    0xe49b_69c1, 0xefbe_4786, 0x0fc1_9dc6, 0x240c_a1cc, 0x2de9_2c6f, 0x4a74_84aa, 0x5cb0_a9dc, 0x76f9_88da,
    0x983e_5152, 0xa831_c66d, 0xb003_27c8, 0xbf59_7fc7, 0xc6e0_0bf3, 0xd5a7_9147, 0x06ca_6351, 0x1429_2967,
    0x27b7_0a85, 0x2e1b_2138, 0x4d2c_6dfc, 0x5338_0d13, 0x650a_7354, 0x766a_0abb, 0x81c2_c92e, 0x9272_2c85,
    0xa2bf_e8a1, 0xa81a_664b, 0xc24b_8b70, 0xc76c_51a3, 0xd192_e819, 0xd699_0624, 0xf40e_3585, 0x106a_a070,
    0x19a4_c116, 0x1e37_6c08, 0x2748_774c, 0x34b0_bcb5, 0x391c_0cb3, 0x4ed8_aa4a, 0x5b9c_ca4f, 0x682e_6ff3,
    0x748f_82ee, 0x78a5_636f, 0x84c8_7814, 0x8cc7_0208, 0x90be_fffa, 0xa450_6ceb, 0xbef9_a3f7, 0xc671_78f2,
];

fn sha256_hex(mut data: &[u8]) -> String {
    let mut state = SHA_H0;
    let total_len = data.len() as u64;
    while data.len() >= 64 {
        let mut blk = [0u8; 64];
        blk.copy_from_slice(&data[..64]);
        sha256_compress(&mut state, &blk);
        data = &data[64..];
    }
    let mut buf = [0u8; 64];
    let rem = data.len();
    buf[..rem].copy_from_slice(data);
    buf[rem] = 0x80;
    if rem + 1 > 56 {
        sha256_compress(&mut state, &buf);
        buf = [0u8; 64];
    }
    let bit_len = total_len.wrapping_mul(8);
    buf[56..64].copy_from_slice(&bit_len.to_be_bytes());
    sha256_compress(&mut state, &buf);

    let mut out = String::with_capacity(64);
    for w in state {
        let _ = write!(out, "{w:08x}");
    }
    out
}

fn sha256_compress(state: &mut [u32; 8], block: &[u8; 64]) {
    let mut w = [0u32; 64];
    for i in 0..16 {
        w[i] = u32::from_be_bytes([
            block[i * 4],
            block[i * 4 + 1],
            block[i * 4 + 2],
            block[i * 4 + 3],
        ]);
    }
    for i in 16..64 {
        let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
        let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
        w[i] = w[i - 16]
            .wrapping_add(s0)
            .wrapping_add(w[i - 7])
            .wrapping_add(s1);
    }
    let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h] = *state;
    for i in 0..64 {
        let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
        let t1 = h
            .wrapping_add(s1)
            .wrapping_add((e & f) ^ ((!e) & g))
            .wrapping_add(SHA_K[i])
            .wrapping_add(w[i]);
        let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
        let t2 = s0.wrapping_add((a & b) ^ (a & c) ^ (b & c));
        h = g;
        g = f;
        f = e;
        e = d.wrapping_add(t1);
        d = c;
        c = b;
        b = a;
        a = t1.wrapping_add(t2);
    }
    for (slot, delta) in state.iter_mut().zip([a, b, c, d, e, f, g, h]) {
        *slot = slot.wrapping_add(delta);
    }
}
