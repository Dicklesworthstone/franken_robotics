#![deny(unsafe_code)]

//! Unit and property tests for `frob-kir` v0 (`BD-018`):
//! dimension checker (`FROB-REF-250`, T-09, PO-SPARK-01), bounded CSE,
//! canonical `frob.kir/1` round-trip, Rust & dialect emitters, and `K-cay-step`.

use frob_core::evidence::DetClass;
use frob_core::refusal::RefusalCode;
use frob_core::testkit::prop;
use frob_kir::cay_step::{k_cay_step_body, k_cay_step_program};
use frob_kir::dialect_lint::{CompileOptions, DialectUnit, SourceFile, lint};
use frob_kir::emit::{emit_dialect, emit_rust};
use frob_kir::ir::{CmpOp, Dim, Program, ProgramBuilder, Ty};

mod kir_support;
use kir_support::audit;

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
    let abs = b.abs(a, "abs").unwrap();
    b.output("mn", Ty::F32, Dim::DIMENSIONLESS, mn);
    b.output("mx", Ty::F32, Dim::DIMENSIONLESS, mx);
    b.output("mag", Ty::F32, Dim::DIMENSIONLESS, abs);
    let p = b.build().unwrap();
    for inputs in [[-0.0, 0.0], [0.0, -0.0]] {
        assert_eq!(
            p.eval_f32(&inputs)
                .unwrap()
                .iter()
                .map(|v| v.to_bits())
                .collect::<Vec<_>>(),
            [0x8000_0000, 0, 0]
        );
    }
    let neg_nan = f32::from_bits(0xffc0_0042);
    let dialect = emit_dialect(&p).unwrap();
    audit(&dialect).unwrap();
    kir_support::assert_dialect_outputs(
        &p,
        &dialect,
        &[
            [-0.0, 0.0],
            [0.0, -0.0],
            [0.0, 0.0],
            [-0.0, -0.0],
            [neg_nan, 1.0],
            [1.0, neg_nan],
            [-2.0, 3.0],
        ],
    );
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

fn generated_body(input: [f32; 14]) -> [f32; 8] {
    let [qw, qx, qy, qz, px, py, pz, wx, wy, wz, vx, vy, vz, h] = input;
    let (qw, qx, qy, qz, d, px, py, pz) =
        k_cay_step_body(qw, qx, qy, qz, px, py, pz, wx, wy, wz, vx, vy, vz, h);
    [qw, qx, qy, qz, d, px, py, pz]
}

// Oracle: round each indicated arithmetic operation (including sqrt and
// division for normalization) individually to IEEE binary32, nearest ties-even.
#[test]
fn cay_step_exact_fixture() {
    let inp = [
        1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.5, 0.0, 0.0, 0.0, 1.0,
    ];
    let out = generated_body(inp);
    assert_eq!(
        out.map(f32::to_bits),
        [0x3f80_0000, 0, 0, 0x3f40_0000, 0x3fc8_0000, 0, 0, 0]
    );
    let norm = out[4].sqrt();
    assert_eq!(
        [(out[0] / norm).to_bits(), (out[3] / norm).to_bits()],
        [0x3f4c_cccd, 0x3f19_999a]
    );
}

#[test]
fn cay_step_small_angle_fixture() {
    let inp = [
        1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.2, 0.0, 0.0, 0.0, 1.0,
    ];
    let out = generated_body(inp);
    assert_eq!(
        out.map(f32::to_bits),
        [0x3f80_0000, 0, 0, 0x3dcc_cccd, 0x3f81_47ae, 0, 0, 0]
    );
    let norm = out[4].sqrt();
    assert_eq!(
        [(out[0] / norm).to_bits(), (out[3] / norm).to_bits()],
        [0x3f7e_bac1, 0x3dcb_c89b]
    );
}

#[test]
fn cay_step_translation_fixture() {
    let inp = [
        1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 0.002,
    ];
    assert_eq!(
        generated_body(inp).map(f32::to_bits),
        [
            0x3f80_0000,
            0,
            0,
            0,
            0x3f80_0000,
            0x3b03_126f,
            0x3b83_126f,
            0x3bc4_9ba6
        ]
    );
}

#[test]
fn emit_rust_matches_compiled_fixture() {
    let rust = emit_rust(&k_cay_step_program().unwrap()).unwrap();
    assert_eq!(rust, include_str!("generated/k_cay_step.rs"));
    audit(&rust).unwrap();
    assert!(audit("let v5: f32 = v1*v2+v3;").is_err());
    assert!(audit("let v5: f32 = 1e-3;").is_err());
    assert!(audit("const f32 v5 = .25;").is_err());
}

#[test]
fn emit_rust_matches_interpreter() {
    let prog = k_cay_step_program().unwrap();
    prop::check("emit_rust_matches_interpreter", 100_000, |g| {
        let mut draw = |i| (2.0 * g.f64_unit(i as u64) - 1.0) as f32;
        let q: [f32; 4] = core::array::from_fn(&mut draw);
        let norm = q.iter().map(|c| c * c).sum::<f32>().sqrt().max(1e-3);
        let input = core::array::from_fn(|i| match i {
            0..=3 => q[i] / norm,
            4..=6 => draw(i) * 5.0,
            7..=9 => draw(i) * 10.0,
            10..=12 => draw(i) * 3.0,
            _ => 0.002,
        });
        let interp = prog.eval_f32(&input)?;
        for (i, (got, want)) in generated_body(input).into_iter().zip(interp).enumerate() {
            if got.to_bits() != want.to_bits() {
                return Err(format!(
                    "out[{i}] emitted={:08x} interpreter={:08x}",
                    got.to_bits(),
                    want.to_bits()
                ));
            }
        }
        Ok(())
    });
}

#[test]
fn emit_dialect_is_lint_clean() {
    let prog = k_cay_step_program().unwrap();
    let dialect_src = emit_dialect(&prog).expect("emit_dialect");
    audit(&dialect_src).unwrap();

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
    audit(&dialect).unwrap();

    let rust = emit_rust(&p).unwrap();
    assert!(rust.contains("f32::from_bits(0x3e800000u32)"));
    assert!(rust.contains("f32::from_bits(0x80000000u32)"));
    assert!(rust.contains("f32::from_bits(0x3f800000u32)"));
    audit(&rust).unwrap();
}

#[test]
fn canonical_roundtrip_and_hash_stable() {
    let p = k_cay_step_program().unwrap();
    let bytes1 = p.to_canonical_json().unwrap();
    let parsed = Program::parse_json(bytes1.as_bytes()).expect("parse frob.kir/1");
    assert_eq!(parsed, p);
    let bytes2 = parsed.to_canonical_json().unwrap();
    assert_eq!(bytes1.as_bytes(), bytes2.as_bytes());
    let h1 = p.content_hash().unwrap();
    let h2 = parsed.content_hash().unwrap();
    assert_eq!(h1, h2);
    assert_eq!(
        h1.to_hex(),
        "db0ac8536fc51f00ec3f8c8f743c3c4ab3bfc4bafae920fe44243e6f3de08a5c"
    );

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
    fn check(a: Dim, b: Dim, delta: i8) -> Result<(), String> {
        let sum = Dim(core::array::from_fn(|i| a.0[i] + b.0[i]));
        let mut builder = ProgramBuilder::new("homogeneous", "ref", DetClass::D0Exact);
        let x = builder.input("x", Ty::F32, a).unwrap();
        let y = builder.input("y", Ty::F32, b).unwrap();
        let z = builder.input("z", Ty::F32, sum).unwrap();
        let xy = builder.mul(x, y, "x*y").unwrap();
        let addition = builder.add(xy, z, "x*y + z").unwrap();
        builder.output("result", Ty::F32, sum, addition);
        let mut mutant = builder.build().map_err(|r| r.message)?;

        // Only the right addend changes. The left addend and result annotations
        // stay valid under the planted checker, so no earlier refusal masks it.
        let mut wrong = sum;
        wrong.0[2] += delta;
        assert_ne!(wrong, sum);
        mutant.inputs[2].dim = wrong;
        mutant.nodes[z].dim = wrong;
        match mutant.typecheck() {
            Err(r)
                if r.code == RefusalCode::FrobRef250
                    && r.pointer == format!("/homogeneous/node/{addition}")
                    && r.message.contains("inhomogeneous sum") =>
            {
                Ok(())
            }
            Err(r) => Err(format!("wrong refusal: {}", r.message)),
            Ok(()) => Err(format!("accepted inhomogeneous sum {sum} + {wrong}")),
        }
    }

    prop::check("typecheck_homogeneous_sums", 10_000, |g| {
        let a = Dim(core::array::from_fn(|i| g.i64_range(i as u64, -8, 8) as i8));
        let b = Dim(core::array::from_fn(|i| {
            g.i64_range((i + 6) as u64, -8, 8) as i8
        }));
        let delta = if g.bool(12) { 1 } else { -1 };
        check(a, b, delta)
    });
    // Registered control: length [m] plus velocity [m/s].
    check(Dim::LENGTH, Dim::DIMENSIONLESS, -1).unwrap();
}

#[test]
fn parse_refuses_invalid_graph_and_metadata() {
    let mut b = ProgramBuilder::new("parse_check", "ref", DetClass::D0Exact);
    let x = b.input("x", Ty::F32, Dim::DIMENSIONLESS).unwrap();
    let sum = b.add(x, x, "x + x").unwrap();
    b.output("result", Ty::F32, Dim::DIMENSIONLESS, sum);
    let bytes = b.build().unwrap().to_canonical_json().unwrap();
    let valid = bytes.as_str();
    for (from, to, diagnostic) in [
        (r#""args":[0,0]"#, r#""args":[0,1]"#, "operand index"),
        (r#""port":0"#, r#""port":1"#, "input port out of range"),
        (r#""node":1"#, r#""node":2"#, "output node index"),
        (
            r#""math_mode":"Ieee754Safe""#,
            r#""math_mode":"Fast""#,
            "math_mode",
        ),
        (r#""op_counts":{"#, r#""wrong":{"#, "unknown field"),
        (r#""add_sub":1"#, r#""add_sub":2"#, "op_counts.add_sub"),
        (
            r#""add_sub":1"#,
            r#""add_sub":1,"extra":0"#,
            "unknown op_counts field",
        ),
        (
            r#""args":[0,0]"#,
            r#""args":[0,0],"port":0"#,
            "unknown node field",
        ),
    ] {
        let input = valid.replacen(from, to, 1);
        assert_ne!(valid, input, "mutation must edit the serialized program");
        let err = Program::parse_json(input.as_bytes()).unwrap_err();
        assert!(err.contains(diagnostic), "{err}");
    }
    let (before, after) = valid.split_once(r#","op_counts":{"#).unwrap();
    let (_, rest) = after.split_once('}').unwrap();
    let missing = format!("{before}{rest}");
    assert!(
        Program::parse_json(missing.as_bytes())
            .unwrap_err()
            .contains("missing field `op_counts`")
    );
}

#[test]
fn eval_respects_port_types() {
    let mut b = ProgramBuilder::new("bools", "ref", DetClass::D0Exact);
    let p = b.input("pred", Ty::Bool, Dim::DIMENSIONLESS).unwrap();
    let a = b.input("left", Ty::Bool, Dim::DIMENSIONLESS).unwrap();
    let c = b.input("right", Ty::Bool, Dim::DIMENSIONLESS).unwrap();
    let selected = b.select(p, a, c, "select").unwrap();
    b.output("result", Ty::Bool, Dim::DIMENSIONLESS, selected);
    let p = b.build().unwrap();
    assert_eq!(p.eval_f32(&[1.0, 1.0, 0.0]).unwrap(), [1.0]);
    assert_eq!(p.eval_f32(&[0.0, 1.0, 0.0]).unwrap(), [0.0]);
    assert!(
        p.eval_f32(&[2.0, 1.0, 0.0])
            .unwrap_err()
            .contains("requires 0 or 1")
    );
    assert!(
        emit_dialect(&p)
            .unwrap()
            .contains("const u32 v3 = FROB_SELECT")
    );

    let mut b = ProgramBuilder::new("wide", "ref", DetClass::D0Exact);
    let x = b.input("x", Ty::F64, Dim::DIMENSIONLESS).unwrap();
    b.output("result", Ty::F64, Dim::DIMENSIONLESS, x);
    assert!(
        b.build()
            .unwrap()
            .eval_f32(&[1.0])
            .unwrap_err()
            .contains("f64 input")
    );
}

#[test]
fn emit_names_are_hygienic() {
    let mut b = ProgramBuilder::new("123-min", "ref", DetClass::D0Exact);
    let x = b.input("x", Ty::F32, Dim::DIMENSIONLESS).unwrap();
    let y = b.input("v0", Ty::F32, Dim::DIMENSIONLESS).unwrap();
    let mn = b.min(x, y, "min").unwrap();
    b.output("result", Ty::F32, Dim::DIMENSIONLESS, mn);
    let mut p = b.build().unwrap();
    let rust = emit_rust(&p).unwrap();
    assert!(rust.contains("fn k_123_min_body(arg0: f32, arg1: f32)"));
    assert!(rust.contains("let v1: f32 = arg1;"));
    assert!(!rust.contains("crate::ir"));
    assert!(emit_dialect(&p).unwrap().contains("const f32 v1 = arg1;"));
    for name in ["x", "fn", "kernel", "FROB_FN"] {
        p.inputs[1].name = name.into();
        assert!(emit_rust(&p).is_err(), "{name}");
        assert!(emit_dialect(&p).is_err(), "{name}");
    }
}
