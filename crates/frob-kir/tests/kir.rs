#![deny(unsafe_code)]

//! Unit and property tests for `frob-kir` v0 (`BD-018`):
//! dimension checker (`FROB-REF-250`, T-09, PO-SPARK-01), bounded CSE,
//! canonical `frob.kir/1` round-trip, Rust & dialect emitters, and `K-cay-step`.

use frob_core::evidence::DetClass;
use frob_core::refusal::RefusalCode;
use frob_kir::cay_step::{k_cay_step_body, k_cay_step_program};
use frob_kir::dialect_lint::{CompileOptions, DialectUnit, SourceFile, lint};
use frob_kir::emit::{
    check_no_decimal_float_literals, check_one_op_per_statement, emit_dialect, emit_rust,
};
use frob_kir::ir::{CmpOp, Dim, Op, Program, ProgramBuilder, Ty};

#[test]
fn dim_add_requires_equal() {
    let mut b = ProgramBuilder::new("bad_add", "bad_add", DetClass::D0Exact);
    let p = b.input("p", Ty::F32, Dim::LENGTH).unwrap();
    let a = b.input("a", Ty::F32, Dim::DIMENSIONLESS).unwrap();
    let err = b
        .add(p, a, "p + a")
        .expect_err("length + dimensionless must fail");
    assert_eq!(err.code, RefusalCode::FrobRef250);
    assert!(
        err.pointer.contains("/node/2"),
        "pointer must name node: {}",
        err.pointer
    );
}

#[test]
fn dim_mul_div_exponents() {
    let mut b = ProgramBuilder::new("mul_div", "mul_div", DetClass::D0Exact);
    let v = b.input("v", Ty::F32, Dim::VELOCITY).unwrap();
    let t = b.input("t", Ty::F32, Dim::TIME).unwrap();
    let d = b.input("d", Ty::F32, Dim::LENGTH).unwrap();
    let vt = b.mul(v, t, "v*t").unwrap();
    let dt = b.div(d, t, "d/t").unwrap();
    b.output("dist", Ty::F32, Dim::LENGTH, vt);
    b.output("vel", Ty::F32, Dim::VELOCITY, dt);
    let prog = b.build().expect("dimensionally valid");
    assert_eq!(prog.nodes[vt].dim, Dim::LENGTH);
    assert_eq!(prog.nodes[dt].dim, Dim::VELOCITY);
}

#[test]
fn dim_sqrt_even_only() {
    let mut b = ProgramBuilder::new("sqrt_ok", "sqrt_ok", DetClass::D0Exact);
    let d = b.input("d", Ty::F32, Dim::LENGTH).unwrap();
    let area = b.mul(d, d, "d^2").unwrap();
    let r = b.sqrt(area, "sqrt(d^2)").unwrap();
    b.output("r", Ty::F32, Dim::LENGTH, r);
    let prog = b.build().expect("sqrt(m^2) = m");
    assert_eq!(prog.nodes[r].dim, Dim::LENGTH);

    let mut b2 = ProgramBuilder::new("sqrt_bad", "sqrt_bad", DetClass::D0Exact);
    let p = b2.input("p", Ty::F32, Dim::LENGTH).unwrap();
    let err = b2.sqrt(p, "sqrt(p)").expect_err("sqrt(m) must fail");
    assert_eq!(err.code, RefusalCode::FrobRef250);
    assert!(err.pointer.contains("/node/1"));
}

#[test]
fn dim_select_arms_equal() {
    let mut b = ProgramBuilder::new("sel_bad", "sel_bad", DetClass::D0Exact);
    let p = b.input("p", Ty::F32, Dim::LENGTH).unwrap();
    let t = b.input("t", Ty::F32, Dim::TIME).unwrap();
    let z = b.const_f32(0.0, Dim::LENGTH).unwrap();
    let pred = b.cmp(CmpOp::Gt, p, z, "p > 0").unwrap();
    let err = b
        .select(pred, p, t, "sel(p>0, p, t)")
        .expect_err("select arms must match dim");
    assert_eq!(err.code, RefusalCode::FrobRef250);
}

#[test]
fn cse_commutative() {
    let mut b = ProgramBuilder::new("cse_comm", "cse_comm", DetClass::D0Exact);
    let x = b.input("x", Ty::F32, Dim::DIMENSIONLESS).unwrap();
    let y = b.input("y", Ty::F32, Dim::DIMENSIONLESS).unwrap();
    let m1 = b.mul(x, y, "x*y").unwrap();
    let m2 = b.mul(y, x, "y*x").unwrap();
    assert_eq!(m1, m2, "Mul(x, y) and Mul(y, x) must share one node");
    let sum = b.add(m1, m2, "(x*y)+(y*x)").unwrap();
    let s1 = b.sub(x, y, "x-y").unwrap();
    let s2 = b.sub(y, x, "y-x").unwrap();
    assert_ne!(s1, s2, "Sub(x, y) and Sub(y, x) must be distinct nodes");
    b.output("out", Ty::F32, Dim::DIMENSIONLESS, sum);
    b.output("d1", Ty::F32, Dim::DIMENSIONLESS, s1);
    b.output("d2", Ty::F32, Dim::DIMENSIONLESS, s2);
    let prog = b.build().unwrap();
    assert_eq!(prog.meta.op_counts.mul, 1);
}

#[test]
fn cse_does_not_reassociate() {
    let build_assoc = |left: bool| -> Program {
        let mut b = ProgramBuilder::new("assoc", "assoc", DetClass::D2PerBackend);
        let a = b.input("a", Ty::F32, Dim::DIMENSIONLESS).unwrap();
        let bb = b.input("b", Ty::F32, Dim::DIMENSIONLESS).unwrap();
        let c = b.input("c", Ty::F32, Dim::DIMENSIONLESS).unwrap();
        let out = if left {
            let ab = b.add(a, bb, "a+b").unwrap();
            b.add(ab, c, "(a+b)+c").unwrap()
        } else {
            let bc = b.add(bb, c, "b+c").unwrap();
            b.add(a, bc, "a+(b+c)").unwrap()
        };
        b.output("out", Ty::F32, Dim::DIMENSIONLESS, out);
        b.build().unwrap()
    };
    let p_left = build_assoc(true);
    let p_right = build_assoc(false);
    assert_ne!(
        p_left.content_hash().unwrap(),
        p_right.content_hash().unwrap()
    );
    let inp = [1.0e8_f32, -1.0e8_f32, 1.0_f32];
    assert_eq!(p_left.eval_f32(&inp).unwrap()[0], 1.0_f32);
    assert_eq!(p_right.eval_f32(&inp).unwrap()[0], 0.0_f32);
}

#[test]
fn const_signed_zero_distinct() {
    let mut b = ProgramBuilder::new("zeros", "zeros", DetClass::D0Exact);
    let pz = b.const_f32(0.0_f32, Dim::DIMENSIONLESS).unwrap();
    let nz = b.const_f32(-0.0_f32, Dim::DIMENSIONLESS).unwrap();
    assert_ne!(pz, nz, "+0.0 and -0.0 must be distinct constant nodes");
    b.output("pz", Ty::F32, Dim::DIMENSIONLESS, pz);
    b.output("nz", Ty::F32, Dim::DIMENSIONLESS, nz);
    let p = b.build().unwrap();
    let out = p.eval_f32(&[]).unwrap();
    assert_eq!(out[0].to_bits(), 0x0000_0000);
    assert_eq!(out[1].to_bits(), 0x8000_0000);
}

#[test]
fn min_max_nan_and_zero() {
    let mut b = ProgramBuilder::new("minmax", "minmax", DetClass::D0Exact);
    let a = b.input("a", Ty::F32, Dim::DIMENSIONLESS).unwrap();
    let bb = b.input("b", Ty::F32, Dim::DIMENSIONLESS).unwrap();
    let mn = b.min(a, bb, "min").unwrap();
    let mx = b.max(a, bb, "max").unwrap();
    b.output("mn", Ty::F32, Dim::DIMENSIONLESS, mn);
    b.output("mx", Ty::F32, Dim::DIMENSIONLESS, mx);
    let p = b.build().unwrap();

    let z = p.eval_f32(&[-0.0_f32, 0.0_f32]).unwrap();
    assert_eq!(z[0].to_bits(), (-0.0_f32).to_bits(), "min(-0, +0) = -0");
    assert_eq!(z[1].to_bits(), (0.0_f32).to_bits(), "max(-0, +0) = +0");

    let n = p.eval_f32(&[f32::NAN, 1.0_f32]).unwrap();
    assert!(n[0].is_nan(), "min(NaN, 1) is NaN");
    assert!(n[1].is_nan(), "max(NaN, 1) is NaN");
}

#[test]
fn select_keeps_infinity_out() {
    // KA-533: Select(true, 1, +inf) = 1 exactly.
    let mut b = ProgramBuilder::new("ka533", "ka533", DetClass::D0Exact);
    let a = b.input("a", Ty::F32, Dim::DIMENSIONLESS).unwrap();
    let inf = b.input("inf", Ty::F32, Dim::DIMENSIONLESS).unwrap();
    let zero = b.const_f32(0.0, Dim::DIMENSIONLESS).unwrap();
    let pred = b.cmp(CmpOp::Gt, a, zero, "a > 0").unwrap();
    let out = b.select(pred, a, inf, "select").unwrap();
    b.output("out", Ty::F32, Dim::DIMENSIONLESS, out);
    let p = b.build().unwrap();
    let got = p.eval_f32(&[1.0_f32, f32::INFINITY]).unwrap()[0];
    assert_eq!(got.to_bits(), 1.0_f32.to_bits());
}

#[test]
fn cay_step_exact_fixture() {
    let p = k_cay_step_program().expect("K-cay-step builds");
    let inp = [
        1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.5, 0.0, 0.0, 0.0, 1.0,
    ];
    let out = p.eval_f32(&inp).unwrap();
    assert_eq!(out[0].to_bits(), 1.0_f32.to_bits());
    assert_eq!(out[1].to_bits(), 0.0_f32.to_bits());
    assert_eq!(out[2].to_bits(), 0.0_f32.to_bits());
    assert_eq!(out[3].to_bits(), 0.75_f32.to_bits());
    assert_eq!(out[4].to_bits(), 1.5625_f32.to_bits());
    let inv_s = out[4].sqrt();
    assert_eq!(inv_s.to_bits(), 1.25_f32.to_bits());
    assert_eq!((out[0] / inv_s).to_bits(), 0.8_f32.to_bits());
    assert_eq!((out[3] / inv_s).to_bits(), 0.6_f32.to_bits());
}

#[test]
fn cay_step_small_angle_fixture() {
    let p = k_cay_step_program().unwrap();
    let inp = [
        1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.2, 0.0, 0.0, 0.0, 1.0,
    ];
    let out = p.eval_f32(&inp).unwrap();
    let s = out[4].sqrt();
    let qw = out[0] / s;
    let qz = out[3] / s;
    assert!((qw - 0.9950372_f32).abs() <= 1e-6, "qw={qw}");
    assert!((qz - 0.0995037_f32).abs() <= 1e-6, "qz={qz}");
    let angle = 2.0_f64 * f64::from(qz).atan2(f64::from(qw));
    let expected_angle = 2.0_f64 * (0.1_f64).atan();
    assert!(
        (angle - expected_angle).abs() < 1e-6,
        "angle={angle} expected={expected_angle}"
    );
}

#[test]
fn cay_step_translation_fixture() {
    let p = k_cay_step_program().unwrap();
    let inp = [
        1.0, 0.0, 0.0, 0.0, 10.0, -5.0, 2.0, 0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 0.002,
    ];
    let out = p.eval_f32(&inp).unwrap();
    assert_eq!(
        (out[5] - 10.0_f32).to_bits(),
        ((10.0_f32 + 0.002_f32) - 10.0_f32).to_bits()
    );
    let inp0 = [
        1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 0.002,
    ];
    let out0 = p.eval_f32(&inp0).unwrap();
    assert_eq!(out0[5].to_bits(), 0.002_f32.to_bits());
    assert_eq!(out0[6].to_bits(), 0.004_f32.to_bits());
    assert_eq!(out0[7].to_bits(), 0.006_f32.to_bits());
}

#[test]
fn emit_rust_matches_interpreter() {
    let prog = k_cay_step_program().unwrap();
    let rust_src = emit_rust(&prog).expect("emit_rust");
    check_one_op_per_statement(&rust_src).expect("one op per statement");
    check_no_decimal_float_literals(&rust_src).expect("no decimal float literals");
    assert!(!rust_src.contains("mul_add"));

    // Negative control: compound expression caught by one-op audit.
    assert!(check_one_op_per_statement("    let v5: f32 = v1 * v2 + v3;").is_err());

    // 10^5 seeded random inputs: bit-for-bit equality between interpreter and k_cay_step_body.
    let mut s = 0xBD18_CA70_57E9_0001u64;
    let mut next_f32 = || -> f32 {
        s = s
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1442695040888963407);
        let u = ((s >> 40) as f32) / ((1u32 << 24) as f32);
        u * 2.0 - 1.0
    };
    for idx in 0..100_000usize {
        let mut q = [next_f32(), next_f32(), next_f32(), next_f32()];
        let n = (q[0] * q[0] + q[1] * q[1] + q[2] * q[2] + q[3] * q[3])
            .sqrt()
            .max(1e-3);
        for c in &mut q {
            *c /= n;
        }
        let inp = [
            q[0],
            q[1],
            q[2],
            q[3],
            next_f32() * 5.0,
            next_f32() * 5.0,
            next_f32() * 5.0,
            next_f32() * 10.0,
            next_f32() * 10.0,
            next_f32() * 10.0,
            next_f32() * 3.0,
            next_f32() * 3.0,
            next_f32() * 3.0,
            0.002_f32,
        ];
        let interp = prog.eval_f32(&inp).unwrap();
        let (a0, a1, a2, a3, a4, a5, a6, a7) = k_cay_step_body(
            inp[0], inp[1], inp[2], inp[3], inp[4], inp[5], inp[6], inp[7], inp[8], inp[9],
            inp[10], inp[11], inp[12], inp[13],
        );
        let rust = [a0, a1, a2, a3, a4, a5, a6, a7];
        for k in 0..8 {
            assert_eq!(
                interp[k].to_bits(),
                rust[k].to_bits(),
                "bit mismatch at case {idx} out[{k}]: interp={} rust={}",
                interp[k],
                rust[k]
            );
        }
    }
}

#[test]
fn emit_dialect_is_lint_clean() {
    let prog = k_cay_step_program().unwrap();
    let dialect_src = emit_dialect(&prog).expect("emit_dialect");
    check_one_op_per_statement(&dialect_src).expect("one op per statement");
    check_no_decimal_float_literals(&dialect_src).expect("no decimal float literals");

    let unit =
        DialectUnit::default().with_source(SourceFile::new("k_cay_step_body.h", &dialect_src));
    lint(&unit, &CompileOptions::cuda_safe("sm_89")).expect("CUDA lint clean");
    lint(&unit, &CompileOptions::metal_safe()).expect("Metal lint clean");
}

#[test]
fn emit_dialect_refuses_f64() {
    let mut b = ProgramBuilder::new("f64_prog", "f64_prog", DetClass::D2PerBackend);
    let x = b.input("x", Ty::F64, Dim::DIMENSIONLESS).unwrap();
    let c = b.const_f64(2.0, Dim::DIMENSIONLESS).unwrap();
    let y = b.mul(x, c, "2*x").unwrap();
    b.output("y", Ty::F64, Dim::DIMENSIONLESS, y);
    let p = b.build().unwrap();
    assert!(emit_rust(&p).is_ok());
    assert!(emit_dialect(&p).is_err(), "dialect emitter must refuse F64");
}

#[test]
fn emit_constants_by_bits() {
    let mut b = ProgramBuilder::new("consts", "consts", DetClass::D0Exact);
    let c025 = b.const_f32(0.25_f32, Dim::DIMENSIONLESS).unwrap();
    let cnz = b.const_f32(-0.0_f32, Dim::DIMENSIONLESS).unwrap();
    let c1 = b.const_f32(1.0_f32, Dim::DIMENSIONLESS).unwrap();
    let s1 = b.add(c025, cnz, "s1").unwrap();
    let s2 = b.add(s1, c1, "s2").unwrap();
    b.output("out", Ty::F32, Dim::DIMENSIONLESS, s2);
    let p = b.build().unwrap();

    let dialect = emit_dialect(&p).unwrap();
    assert!(dialect.contains("FROB_F32_BITS(0x3e800000u)"));
    assert!(dialect.contains("FROB_F32_BITS(0x80000000u)"));
    assert!(dialect.contains("FROB_F32_BITS(0x3f800000u)"));
    check_no_decimal_float_literals(&dialect).unwrap();

    let rust = emit_rust(&p).unwrap();
    assert!(rust.contains("f32::from_bits(0x3e800000u32)"));
    assert!(rust.contains("f32::from_bits(0x80000000u32)"));
    assert!(rust.contains("f32::from_bits(0x3f800000u32)"));
    check_no_decimal_float_literals(&rust).unwrap();
}

#[test]
fn canonical_roundtrip_and_hash_stable() {
    let p = k_cay_step_program().unwrap();
    let bytes1 = p.to_canonical_json().unwrap();
    let parsed = Program::parse_json(bytes1.as_bytes()).expect("parse frob.kir/1");
    let bytes2 = parsed.to_canonical_json().unwrap();
    assert_eq!(bytes1.as_bytes(), bytes2.as_bytes());
    let h1 = p.content_hash().unwrap();
    let h2 = parsed.content_hash().unwrap();
    assert_eq!(h1, h2);
    assert_eq!(h1.to_hex().len(), 64);

    // Metamorphic power-of-two scaling: length x4, time x2 => velocity x2, omega x0.5.
    let base = [
        1.0, 0.0, 0.0, 0.0, 1.5, -2.5, 0.75, 0.25, -0.5, 0.125, 2.0, -1.0, 0.5, 0.0625,
    ];
    let scaled = [
        base[0],
        base[1],
        base[2],
        base[3],
        base[4] * 4.0,
        base[5] * 4.0,
        base[6] * 4.0,
        base[7] * 0.5,
        base[8] * 0.5,
        base[9] * 0.5,
        base[10] * 2.0,
        base[11] * 2.0,
        base[12] * 2.0,
        base[13] * 2.0,
    ];
    let o_base = p.eval_f32(&base).unwrap();
    let o_scaled = p.eval_f32(&scaled).unwrap();
    for k in 0..5 {
        assert_eq!(
            o_base[k].to_bits(),
            o_scaled[k].to_bits(),
            "dimensionless output {k} invariant"
        );
    }
    for k in 5..8 {
        assert_eq!(
            (o_base[k] * 4.0).to_bits(),
            o_scaled[k].to_bits(),
            "length output {k} scales by 4"
        );
    }
}

#[test]
fn typecheck_homogeneous_sums() {
    // Registered negative control: adding length [m] to velocity [m/s] refused with FROB-REF-250.
    let mut bad = ProgramBuilder::new("nc_len_plus_vel", "nc", DetClass::D0Exact);
    let len = bad.input("len", Ty::F32, Dim::LENGTH).unwrap();
    let vel = bad.input("vel", Ty::F32, Dim::VELOCITY).unwrap();
    let err = bad.add(len, vel, "len + vel").expect_err("must refuse");
    assert_eq!(err.code, RefusalCode::FrobRef250);

    // Property test (PO-SPARK-01): 10^4 random well-typed programs pass and 10^4 single-edit
    // input-dimension mutants never pass with an inhomogeneous sum.
    let dims = [
        Dim::DIMENSIONLESS,
        Dim::LENGTH,
        Dim::TIME,
        Dim::VELOCITY,
        Dim::ANGULAR_VELOCITY,
    ];
    let mut rng = 0x5041_524B_0001_0018u64;
    let mut next = || {
        rng = rng.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1);
        (rng >> 33) as usize
    };
    for case in 0..10_000usize {
        let d0 = dims[next() % dims.len()];
        let d1 = dims[next() % dims.len()];
        let mut b = ProgramBuilder::new(format!("p{case}"), "ref", DetClass::D0Exact);
        let x0 = b.input("x0", Ty::F32, d0).unwrap();
        let x1 = b.input("x1", Ty::F32, d1).unwrap();
        let p01 = b.mul(x0, x1, "x0*x1").unwrap();
        let p10 = b.mul(x1, x0, "x1*x0").unwrap();
        let s = b.add(p01, p10, "sum").unwrap();
        let q = b.div(s, x1, "div").unwrap();
        let out = b.add(q, x0, "out").unwrap();
        b.output("out", Ty::F32, d0, out);
        let prog = b.build().expect("well-typed random program passes");

        // Mutate input 0 dimension and verify independent recomputation: no accepted program
        // contains an inhomogeneous Add/Sub/Min/Max/Cmp/Select.
        let mut mutated = prog;
        let new_d = dims[(next() + 1) % dims.len()];
        mutated.inputs[0].dim = new_d;
        mutated.nodes[0].dim = new_d;
        let tc = mutated.typecheck();
        if new_d != d0 {
            assert_eq!(
                tc.expect_err("mutated input dim must be caught").code,
                RefusalCode::FrobRef250
            );
        } else {
            assert!(tc.is_ok());
            // Independently confirm every Add/Sub node has equal operand dimensions.
            for n in &mutated.nodes {
                if let Op::Add(a, bb) | Op::Sub(a, bb) = n.op {
                    assert_eq!(mutated.nodes[a].dim, mutated.nodes[bb].dim);
                }
            }
        }
    }
}
