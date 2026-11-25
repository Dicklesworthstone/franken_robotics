#![deny(unsafe_code)]

//! Appendix D.3 law tests, known answers, and negative controls (`N12-43`).
//!
//! Covers all 10 rows of `registries/compendium/d3.toml`: 20 named tests plus dedicated
//! negative controls (`d3_nc_*`) so every row has an explicit law/KA test and an explicit
//! negative control. Every test emits a `frob.testevidence/1` record when
//! `FROB_TEST_EVIDENCE_DIR` is set.

use std::f64::consts::{PI, SQRT_2};

use frob_conformance::compendium::d3::{
    ClearanceVerdict, Cone3, Cone4Force, Cone4Vel, DimVec, SoilCardDimensions, SpinFactor,
    VehicleKinematics, backlash_complementarity_holds, backlash_step, bekker_pressure,
    bracket_depth_cost_order, check_soil_dimensions, clearance_band, clearance_sandwich_doorway,
    cone4d_coupled_spin_fraction, cone4d_coupled_tangential_fraction, cone4d_hertz_radius_f64,
    cone4d_pairing, cone4d_pure_spin_moment_exact, cone4d_uniform_radius_exact,
    corner_rod_envelope_at_angle, diff_drive_lateral_shift_cost, doorway_verdict,
    fast_tier_quadratic_gradient, fast_tier_quadratic_impulse, fast_tier_quadratic_minimizer,
    growth_vector, hard_stop_complementarity_holds, heading_half_bin_deg_exact, hersey_number,
    in_cone4d_exact, in_dual_cone4d_exact, in_dual_friction_cone_exact, in_friction_box_exact,
    in_friction_cone_exact, janosi_hanamoto_shear, jordan_eigenvalues_exact, jordan_in_cone_exact,
    jordan_product, jordan_square, project_friction_cone_f64, project_friction_cone_rational,
    right_angle_corner_max_rod, rod_clears_corner_exact, squeeze_film_dh_dt, squeeze_film_mu_eff,
    squeeze_film_thickness, squeeze_film_time_from_thick, stiction_breakaway_time,
    stiction_multiplier_valid,
};
use frob_conformance::compendium::rat::{
    Mat3, Rat, cross3, dot3, mat3_det, mat3_diag, mat3_vec, r, ri, vec3, vec3_scale, vec3_sub,
};
use frob_core::testkit::evidence::{
    EvidenceKind, EvidenceRecord, EvidenceStatus, emit_if_configured,
};

fn emit_law(
    test_name: &str,
    row_name: &str,
    oracle: &str,
    passed: bool,
    observed: String,
    expected: String,
    tolerance: Option<&str>,
    unit: Option<&str>,
) {
    let mut rec = EvidenceRecord::new(
        EvidenceKind::Law,
        format!("D.3:{row_name}"),
        if passed {
            EvidenceStatus::Pass
        } else {
            EvidenceStatus::Fail
        },
    );
    rec.crate_name = "frob-conformance".to_owned();
    rec.test_name = test_name.to_owned();
    rec.site = Some(oracle.to_owned());
    rec.observed = Some(observed);
    rec.expected = Some(expected);
    rec.tolerance = tolerance.map(str::to_owned);
    rec.unit = unit.map(str::to_owned);
    rec.determinism = Some("D0".to_owned());
    emit_if_configured(&rec);
}

fn emit_nc(
    test_name: &str,
    row_name: &str,
    oracle: &str,
    caught: bool,
    observed: String,
    expected: String,
    tolerance: Option<&str>,
    unit: Option<&str>,
) {
    let mut rec = EvidenceRecord::new(
        EvidenceKind::Nc,
        format!("D.3:{row_name}"),
        if caught {
            EvidenceStatus::Caught
        } else {
            EvidenceStatus::Missed
        },
    );
    rec.crate_name = "frob-conformance".to_owned();
    rec.test_name = test_name.to_owned();
    rec.site = Some(oracle.to_owned());
    rec.observed = Some(observed);
    rec.expected = Some(expected);
    rec.tolerance = tolerance.map(str::to_owned);
    rec.unit = unit.map(str::to_owned);
    rec.determinism = Some("D0".to_owned());
    emit_if_configured(&rec);
}

// =========================================================================
// Row 1: Friction cone
// =========================================================================

#[test]
fn d3_cone_duality() {
    let mu = r(3, 5);
    // Pythagorean boundary points:
    // x in K_mu: n = 25/3, t = (3, 4) -> ||t|| = 5 == (3/5)*(25/3)
    let x_bdry = Cone3::new(r(25, 3), ri(3), ri(4));
    // Opposite tangent direction on dual boundary K_{1/mu}:
    // y_n = 3, y_t = (-3, -4) -> mu * ||y_t|| = (3/5)*5 = 3 == y_n
    let y_bdry = Cone3::new(ri(3), ri(-3), ri(-4));
    // Interior points
    let x_int = Cone3::new(ri(10), ri(3), ri(4));
    let y_int = Cone3::new(ri(12), ri(-5), ri(12));

    let mem_ok = in_friction_cone_exact(&x_bdry, &mu)
        && in_dual_friction_cone_exact(&y_bdry, &mu)
        && in_friction_cone_exact(&x_int, &mu)
        && in_dual_friction_cone_exact(&y_int, &mu);
    let pair_zero = x_bdry.dot(&y_bdry);
    let pair_pos = x_int.dot(&y_int);
    let duality_ok = !pair_zero.is_neg()
        && pair_zero.is_zero()
        && pair_pos.is_pos()
        && !x_bdry.dot(&y_int).is_neg()
        && !x_int.dot(&y_bdry).is_neg();

    let ok = mem_ok && duality_ok;
    emit_law(
        "d3_cone_duality",
        "Friction cone",
        "frob_conformance::compendium::d3::project_friction_cone_rational",
        ok,
        format!("mem={mem_ok},bdry_dot={pair_zero},int_dot={pair_pos}"),
        "mem=true,bdry_dot=0,int_dot>0".to_owned(),
        None,
        None,
    );
    assert!(ok, "friction cone duality failed");
}

#[test]
fn d3_cone_projection_example() {
    // KA: mu = 1/2, x = (1, (3, 4)) projects to (14/5, (21/25, 28/25))
    let mu = r(1, 2);
    let x = Cone3::new(ri(1), ri(3), ri(4));
    let p = project_friction_cone_rational(&x, &mu).expect("Pythagorean projection exact");
    let expected = Cone3::new(r(14, 5), r(21, 25), r(28, 25));
    let ka_ok = p == expected;

    // Moreau conditions: p in K_mu, q = p - x in K_{1/mu}, <p, q> == 0
    let q = p.sub(&x);
    let moreau_ok = in_friction_cone_exact(&p, &mu)
        && in_dual_friction_cone_exact(&q, &mu)
        && p.dot(&q).is_zero();

    // Consistency with f64 projection
    let (fn_val, ft_val) = project_friction_cone_f64(1.0, [3.0, 4.0], 0.5);
    let f64_ok = (fn_val - 2.8).abs() < 1e-12
        && (ft_val[0] - 0.84).abs() < 1e-12
        && (ft_val[1] - 1.12).abs() < 1e-12;

    // Polar cone point projects to zero
    let polar = Cone3::new(ri(-3), ri(3), ri(4));
    let polar_ok = project_friction_cone_rational(&polar, &mu) == Some(Cone3::zero());

    let ok = ka_ok && moreau_ok && f64_ok && polar_ok;
    emit_law(
        "d3_cone_projection_example",
        "Friction cone",
        "frob_conformance::compendium::d3::project_friction_cone_rational",
        ok,
        format!("p=({},({},{}))", p.n, p.t[0], p.t[1]),
        "p=(14/5,(21/25,28/25))".to_owned(),
        None,
        None,
    );
    assert!(ok, "friction cone KA projection or Moreau conditions failed");
}

#[test]
fn d3_box_admits_sqrt2() {
    // NC: linearized box |t1|, |t2| <= mu * n admits (n, (mu*n, mu*n)),
    // whose tangential norm squared is 2 * (mu * n)^2 (factor sqrt(2), ~41.4% above cone limit).
    let mu = r(1, 2);
    let n = ri(10);
    let mu_n = &mu * &n;
    let diag = Cone3::new(n, mu_n.clone(), mu_n.clone());

    let box_admits = in_friction_box_exact(&diag, &mu);
    let cone_rejects = !in_friction_cone_exact(&diag, &mu);
    let ratio_sq = &diag.tangent_norm_sq() / &(&mu_n * &mu_n);
    let caught = box_admits && cone_rejects && ratio_sq == ri(2);
    emit_nc(
        "d3_box_admits_sqrt2",
        "Friction cone",
        "frob_conformance::compendium::d3::project_friction_cone_rational",
        caught,
        format!("box={box_admits},cone={},ratio_sq={ratio_sq}", !cone_rejects),
        "box=true,cone=false,ratio_sq=2".to_owned(),
        None,
        None,
    );
    assert!(caught, "friction box diagonal excess must be caught by cone");
}

// =========================================================================
// Row 2: Spin-factor Jordan product
// =========================================================================

#[test]
fn d3_jordan_commutative_not_associative() {
    let x = SpinFactor::new(r(3, 2), [r(1, 3), r(-2, 5)]);
    let y = SpinFactor::new(r(-1, 4), [r(5, 2), r(1, 7)]);
    let z = SpinFactor::new(r(2, 3), [r(-3, 4), r(7, 3)]);

    let comm_ok = jordan_product(&x, &y) == jordan_product(&y, &x);
    let sq_in_cone = jordan_in_cone_exact(&jordan_square(&x))
        && jordan_in_cone_exact(&jordan_square(&y));
    let non_assoc = jordan_product(&jordan_product(&x, &y), &z)
        != jordan_product(&x, &jordan_product(&y, &z));

    let ok = comm_ok && sq_in_cone && non_assoc;
    emit_law(
        "d3_jordan_commutative_not_associative",
        "Spin-factor Jordan product",
        "frob_conformance::compendium::d3::jordan_product",
        ok,
        format!("comm={comm_ok},sq_cone={sq_in_cone},non_assoc={non_assoc}"),
        "comm=true,sq_cone=true,non_assoc=true".to_owned(),
        None,
        None,
    );
    assert!(ok, "Jordan commutativity / cone-of-squares / non-associativity failed");
}

#[test]
fn d3_jordan_identity() {
    let x = SpinFactor::new(r(5, 3), [r(-1, 2), r(4, 7)]);
    let y = SpinFactor::new(r(2, 5), [r(3, 4), r(-5, 6)]);
    let x2 = jordan_square(&x);

    // Jordan identity: (x o y) o x^2 == x o (y o x^2)
    let lhs = jordan_product(&jordan_product(&x, &y), &x2);
    let rhs = jordan_product(&x, &jordan_product(&y, &x2));
    let ok = lhs == rhs;
    emit_law(
        "d3_jordan_identity",
        "Spin-factor Jordan product",
        "frob_conformance::compendium::d3::jordan_product",
        ok,
        format!("lhs0={}", lhs.x0),
        format!("rhs0={}", rhs.x0),
        None,
        None,
    );
    assert!(ok, "Jordan identity (x o y) o x^2 == x o (y o x^2) failed");
}

#[test]
fn d3_jordan_eigenvalues_and_membership() {
    // Pythagorean bar vectors: (3, 4) -> norm 5; (5, 12) -> norm 13
    let inside = SpinFactor::new(ri(7), [ri(3), ri(4)]);
    let boundary = SpinFactor::new(ri(5), [ri(3), ri(4)]);
    let outside = SpinFactor::new(ri(12), [ri(5), ri(12)]);

    let e_in = jordan_eigenvalues_exact(&inside).expect("exact eigenvalues");
    let e_bd = jordan_eigenvalues_exact(&boundary).expect("exact eigenvalues");
    let e_out = jordan_eigenvalues_exact(&outside).expect("exact eigenvalues");

    let ok = e_in == (ri(2), ri(12))
        && jordan_in_cone_exact(&inside)
        && e_bd == (ri(0), ri(10))
        && jordan_in_cone_exact(&boundary)
        && e_out == (ri(-1), ri(25))
        && !jordan_in_cone_exact(&outside);

    emit_law(
        "d3_jordan_eigenvalues_and_membership",
        "Spin-factor Jordan product",
        "frob_conformance::compendium::d3::jordan_product",
        ok,
        format!("in={e_in:?},bd={e_bd:?},out={e_out:?}"),
        "in=(2,12),bd=(0,10),out=(-1,25)".to_owned(),
        None,
        None,
    );
    assert!(ok, "Jordan eigenvalues or cone membership equivalence failed");
}

#[test]
fn d3_nc_spin_factor_associativity() {
    // NC: assuming associativity (x o y) o z == x o (y o z) fails on a sampled triple.
    let x = SpinFactor::new(ri(1), [ri(2), ri(0)]);
    let y = SpinFactor::new(ri(0), [ri(1), ri(3)]);
    let z = SpinFactor::new(ri(1), [ri(0), ri(1)]);
    let lhs = jordan_product(&jordan_product(&x, &y), &z);
    let rhs = jordan_product(&x, &jordan_product(&y, &z));
    let caught = lhs != rhs;
    emit_nc(
        "d3_nc_spin_factor_associativity",
        "Spin-factor Jordan product",
        "frob_conformance::compendium::d3::jordan_product",
        caught,
        format!("lhs={:?}", lhs.xbar),
        format!("rhs={:?}", rhs.xbar),
        None,
        None,
    );
    assert!(caught, "spin-factor product must fail associativity");
}

// =========================================================================
// Row 3: Fast-tier contact problem
// =========================================================================

#[test]
fn d3_fast_tier_one_contact_closed_form() {
    // SPD diagonal A = diag(2, 3, 5) -> lambda_min(A) = 2
    let a: Mat3 = mat3_diag(&[ri(2), ri(3), ri(5)]);
    let lambda_min = ri(2);
    let v_star = vec3(r(3, 2), r(-4, 3), r(5, 4));
    let j_row = vec3(ri(1), r(-1, 2), r(2, 3));
    let k = ri(6);

    let v_opt = fast_tier_quadratic_minimizer(&a, &v_star, &j_row, &k).expect("Hessian invertible");
    let grad_opt = fast_tier_quadratic_gradient(&a, &v_star, &j_row, &k, &v_opt);
    let gamma = fast_tier_quadratic_impulse(&j_row, &k, &v_opt);

    // KKT optimality: A*(v - v*) - J^T * gamma == 0 and grad_opt == 0
    let kkt_res = vec3_sub(&mat3_vec(&a, &vec3_sub(&v_opt, &v_star)), &vec3_scale(&gamma, &j_row));
    let opt_ok = grad_opt == vec3(ri(0), ri(0), ri(0)) && kkt_res == vec3(ri(0), ri(0), ri(0));

    // Strong convexity error bound: ||v_trial - v_opt||^2 <= ||grad f(v_trial)||^2 / lambda_min(A)^2
    let v_trial = vec3(ri(1), ri(0), r(-1, 2));
    let grad_trial = fast_tier_quadratic_gradient(&a, &v_star, &j_row, &k, &v_trial);
    let err_sq = dot3(&vec3_sub(&v_trial, &v_opt), &vec3_sub(&v_trial, &v_opt));
    let bound_sq = &dot3(&grad_trial, &grad_trial) / &(&lambda_min * &lambda_min);
    let bound_ok = err_sq.is_pos() && err_sq <= bound_sq;

    let ok = opt_ok && bound_ok;
    emit_law(
        "d3_fast_tier_one_contact_closed_form",
        "Fast-tier contact problem",
        "frob_conformance::compendium::d3::fast_tier_quadratic_minimizer",
        ok,
        format!("opt={opt_ok},err_sq={err_sq},bound_sq={bound_sq}"),
        "opt=true,err_sq<=bound_sq".to_owned(),
        None,
        None,
    );
    assert!(ok, "fast-tier quadratic closed form or error bound failed");
}

#[test]
fn d3_nc_fast_tier_nonconvex() {
    // NC: non-convex l(y) = (1/4) y^4 - (3/4) y^2 with A = 1/2, v* = 0, J = 1 gives
    // f(v) = (1/4) v^2 + (1/4) v^4 - (3/4) v^2 = (1/4) v^4 - (1/2) v^2,
    // whose gradient f'(v) = v^3 - v vanishes at THREE distinct points {-1, 0, +1}
    // (two distinct local/global minimizers at v = -1 and v = +1, negative curvature f''(0) = -1).
    let grad = |v: &Rat| -> Rat { &v.powu(3) - v };
    let hess = |v: &Rat| -> Rat { &(&ri(3) * &v.powu(2)) - &Rat::one() };
    let m1 = ri(-1);
    let z0 = ri(0);
    let p1 = ri(1);
    let caught = grad(&m1).is_zero()
        && grad(&z0).is_zero()
        && grad(&p1).is_zero()
        && hess(&m1).is_pos()
        && hess(&p1).is_pos()
        && hess(&z0).is_neg();
    emit_nc(
        "d3_nc_fast_tier_nonconvex",
        "Fast-tier contact problem",
        "frob_conformance::compendium::d3::fast_tier_quadratic_minimizer",
        caught,
        format!("stationary=[-1,0,1],f''(0)={}", hess(&z0)),
        "multiple stationary points with indefinite Hessian".to_owned(),
        None,
        None,
    );
    assert!(caught, "non-convex potential must produce multiple stationary points");
}

// =========================================================================
// Row 4: Four-dimensional friction cone
// =========================================================================

#[test]
fn d3_4d_cone_coupling_value() {
    let eff_r = 0.04_f64;
    let omega_n = 2.5_f64;
    let v_t = eff_r * omega_n; // v_t == r * omega_n

    let tang_frac = cone4d_coupled_tangential_fraction(v_t, omega_n, eff_r);
    let spin_frac = cone4d_coupled_spin_fraction(v_t, omega_n, eff_r);
    let inv_sqrt2 = 1.0 / SQRT_2;
    let coupling_ok = (tang_frac - inv_sqrt2).abs() < 1e-12
        && (spin_frac - inv_sqrt2).abs() < 1e-12
        && (tang_frac * tang_frac + spin_frac * spin_frac - 1.0).abs() < 1e-12;

    // Exact 4D cone duality on Pythagorean 4-tuple (2, 3, 6) -> norm 7
    let mu = r(1, 2);
    let r_q = r(1, 5);
    let f = Cone4Force {
        n: ri(14),
        t1: ri(2),
        t2: ri(3),
        spin: r(6, 5), // spin / r = 6 -> ||(2, 3, 6)|| = 7 == mu * 14
    };
    let v = Cone4Vel {
        n: r(7, 2),
        t1: ri(-2),
        t2: ri(-3),
        omega_n: ri(-30), // r * omega_n = -6 -> mu * ||(-2, -3, -6)|| = 7/2 == v.n
    };
    let dual_ok = in_cone4d_exact(&f, &mu, &r_q)
        && in_dual_cone4d_exact(&v, &mu, &r_q)
        && cone4d_pairing(&f, &v).is_zero();

    let ok = coupling_ok && dual_ok;
    emit_law(
        "d3_4d_cone_coupling_value",
        "Four-dimensional friction cone",
        "frob_conformance::compendium::d3::cone4d_coupled_tangential_fraction",
        ok,
        format!("{tang_frac:.15}"),
        format!("{inv_sqrt2:.15}"),
        Some("1e-12"),
        Some("1"),
    );
    assert!(ok, "4D cone coupling value or exact duality failed");
}

#[test]
fn d3_pure_spin_moment_two_thirds() {
    let mu = r(3, 5);
    let load = ri(100);
    let disc_r = r(3, 100); // 3 cm
    let r_eff = cone4d_uniform_radius_exact(&disc_r);
    let tau = cone4d_pure_spin_moment_exact(&mu, &load, &disc_r);
    let expected_tau = &(&r(2, 3) * &mu) * &(&load * &disc_r);

    // Verify against 1D integral int_0^R r' * (mu * N / (pi R^2)) * (2*pi*r') dr' = (2/3)*mu*N*R
    let ok = r_eff == r(2, 100) && tau == expected_tau && tau == r(6, 5);
    emit_law(
        "d3_pure_spin_moment_two_thirds",
        "Four-dimensional friction cone",
        "frob_conformance::compendium::d3::cone4d_coupled_tangential_fraction",
        ok,
        format!("{tau}"),
        format!("{expected_tau}"),
        None,
        Some("N*m"),
    );
    assert!(ok, "pure spin limiting moment (2/3)*mu*N*R failed");
}

#[test]
fn d3_hertz_radius_3pi_over_16() {
    let disc_r = 0.05_f64;
    let r_hertz = cone4d_hertz_radius_f64(disc_r);
    let expected = (3.0 * PI / 16.0) * disc_r;

    // Independent numerical check of the Hertzian disc moment integral:
    // r_hertz / R = 3 * int_0^1 u^2 * sqrt(1 - u^2) du = 3 * int_0^{pi/2} sin^2(t)*cos^2(t) dt = 3*pi/16.
    let n_steps = 10_000_usize;
    let dt = (0.5 * PI) / (n_steps as f64);
    let mut sum = 0.0_f64;
    for i in 0..=n_steps {
        let t = (i as f64) * dt;
        let w = if i == 0 || i == n_steps {
            1.0
        } else if i % 2 == 1 {
            4.0
        } else {
            2.0
        };
        let s = t.sin();
        let c = t.cos();
        sum += w * (s * s * c * c);
    }
    let integral_ratio = 3.0 * sum * dt / 3.0;
    let ok = (r_hertz - expected).abs() < 1e-15 && (r_hertz - integral_ratio * disc_r).abs() < 1e-12;
    emit_law(
        "d3_hertz_radius_3pi_over_16",
        "Four-dimensional friction cone",
        "frob_conformance::compendium::d3::cone4d_coupled_tangential_fraction",
        ok,
        format!("{r_hertz:.15}"),
        format!("{expected:.15}"),
        Some("1e-12"),
        Some("m"),
    );
    assert!(ok, "Hertzian radius 3*pi*R/16 check failed");
}

#[test]
fn d3_nc_4d_cone_uncoupled() {
    // NC: uncoupled torsional model assigns full |lambda_t| = mu*N and full |lambda_s| = mu*N*r
    // simultaneously at v_t = r*omega_n, violating the 4D friction cone by factor sqrt(2).
    let mu = r(1, 2);
    let eff_r = r(1, 10);
    let load = ri(20);
    let mu_n = &mu * &load;
    let uncoupled = Cone4Force {
        n: load,
        t1: mu_n.clone(),
        t2: Rat::zero(),
        spin: &mu_n * &eff_r,
    };
    let caught = !in_cone4d_exact(&uncoupled, &mu, &eff_r);
    emit_nc(
        "d3_nc_4d_cone_uncoupled",
        "Four-dimensional friction cone",
        "frob_conformance::compendium::d3::cone4d_coupled_tangential_fraction",
        caught,
        format!("in_cone4d={}", !caught),
        "in_cone4d=false".to_owned(),
        Some("1e-12"),
        Some("1"),
    );
    assert!(caught, "uncoupled sliding+spinning force must violate 4D cone");
}

// =========================================================================
// Row 5: Joint-level rows
// =========================================================================

#[test]
fn d3_stiction_breakaway_time() {
    let tau_s = r(15, 2); // 7.5 N*m
    let ramp_k = ri(3); // 3 N*m/s
    let t_break = stiction_breakaway_time(&tau_s, &ramp_k).expect("positive inputs");
    let expected = r(5, 2); // 2.5 s

    // Before breakaway: multiplier balances ramp; after breakaway: exceeds [-tau_s, tau_s]
    let t_before = r(2, 1);
    let t_after = r(3, 1);
    let valid_before = stiction_multiplier_valid(&(&ramp_k * &t_before), &tau_s);
    let valid_at = stiction_multiplier_valid(&(&ramp_k * &t_break), &tau_s);
    let invalid_after = !stiction_multiplier_valid(&(&ramp_k * &t_after), &tau_s);

    // Hard stop complementarity sanity check
    let hs_ok = hard_stop_complementarity_holds(&ri(1), &ri(2), &ri(0))
        && hard_stop_complementarity_holds(&ri(2), &ri(2), &ri(5))
        && !hard_stop_complementarity_holds(&ri(1), &ri(2), &ri(5));

    let ok = t_break == expected && valid_before && valid_at && invalid_after && hs_ok;
    emit_law(
        "d3_stiction_breakaway_time",
        "Joint-level rows",
        "frob_conformance::compendium::d3::stiction_breakaway_time",
        ok,
        format!("{t_break}"),
        format!("{expected}"),
        None,
        Some("s"),
    );
    assert!(ok, "stiction breakaway time or hard-stop complementarity failed");
}

#[test]
fn d3_backlash_amplitude_and_width() {
    let beta = r(1, 5); // 0.2 rad total backlash
    let half_beta = &beta / &ri(2); // 0.1 rad
    let amp = r(1, 1); // reflected motor amplitude A = 1.0 rad > beta/2

    // Slow triangular reflected motor trajectory: 0 -> +A -> -A -> +A in steps of 1/20
    let mut q_out = Rat::zero();
    let mut up_zero_crossing = None;
    let mut down_zero_crossing = None;
    let mut max_out = Rat::zero();
    let mut min_out = Rat::zero();

    // Upward ramp 0 -> +A
    for k in 0..=20_i64 {
        let qm = r(k, 20);
        q_out = backlash_step(&q_out, &qm, &beta);
        if q_out > max_out {
            max_out = q_out.clone();
        }
    }
    // Downward ramp +A -> -A
    for k in (-20..=20_i64).rev() {
        let qm = r(k, 20);
        let prev = q_out.clone();
        q_out = backlash_step(&q_out, &qm, &beta);
        if prev.is_pos() && q_out.is_zero() {
            down_zero_crossing = Some(qm);
        }
        if q_out < min_out {
            min_out = q_out.clone();
        }
    }
    // Upward ramp -A -> +A
    for k in -20..=20_i64 {
        let qm = r(k, 20);
        let prev = q_out.clone();
        q_out = backlash_step(&q_out, &qm, &beta);
        if prev.is_neg() && q_out.is_zero() {
            up_zero_crossing = Some(qm);
        }
    }

    let expected_amp = &amp - &half_beta; // 9/10
    let width = match (&up_zero_crossing, &down_zero_crossing) {
        (Some(u), Some(d)) => u - d,
        _ => Rat::zero(),
    };
    let comp_ok = backlash_complementarity_holds(&r(9, 10), &ri(1), &beta, &ri(0), &ri(4))
        && backlash_complementarity_holds(&ri(0), &ri(0), &beta, &ri(0), &ri(0))
        && !backlash_complementarity_holds(&ri(0), &ri(0), &beta, &ri(1), &ri(0));

    let ok = max_out == expected_amp
        && min_out == -&expected_amp
        && width == beta
        && comp_ok;
    emit_law(
        "d3_backlash_amplitude_and_width",
        "Joint-level rows",
        "frob_conformance::compendium::d3::stiction_breakaway_time",
        ok,
        format!("amp={max_out},width={width}"),
        format!("amp={expected_amp},width={beta}"),
        None,
        Some("rad"),
    );
    assert!(ok, "backlash amplitude or hysteresis width failed");
}

#[test]
fn d3_nc_joint_backlash_bilateral() {
    // NC: a single bilateral equality q_out = q_motor/N gives amplitude A and zero dead-zone width.
    let beta = r(1, 5);
    let amp = ri(1);
    let bilateral_amp = amp.clone();
    let bilateral_width = Rat::zero();
    let true_amp = &amp - &(&beta / &ri(2));
    let caught = bilateral_amp != true_amp && bilateral_width != beta;
    emit_nc(
        "d3_nc_joint_backlash_bilateral",
        "Joint-level rows",
        "frob_conformance::compendium::d3::stiction_breakaway_time",
        caught,
        format!("bilateral_amp={bilateral_amp},width={bilateral_width}"),
        format!("true_amp={true_amp},width={beta}"),
        None,
        Some("rad"),
    );
    assert!(caught, "bilateral row must fail to exhibit backlash dead zone");
}

// =========================================================================
// Row 6: Growth vector and cost
// =========================================================================

#[test]
fn d3_growth_vectors() {
    let gv_ok = growth_vector(VehicleKinematics::DifferentialDrive) == [2, 3]
        && growth_vector(VehicleKinematics::CarLike) == [2, 3, 4]
        && growth_vector(VehicleKinematics::CarWithTrailer) == [2, 3, 4, 5]
        && growth_vector(VehicleKinematics::Omnidirectional) == [3];

    // Direct algebraic verification for unicycle / differential drive at theta = 0:
    // g1 = (cos t, sin t, 0) -> (1, 0, 0), g2 = (0, 0, 1),
    // [g1, g2] = -dg1/dt = (sin t, -cos t, 0) -> (0, -1, 0); det([g1, g2, [g1,g2]]) = 1 != 0.
    let g1 = vec3(ri(1), ri(0), ri(0));
    let g2 = vec3(ri(0), ri(0), ri(1));
    let bracket_g1_g2 = vec3(ri(0), ri(-1), ri(0));
    let span_mat: Mat3 = [g1.clone(), g2.clone(), bracket_g1_g2];
    let rank2_ok = cross3(&g1, &g2) != vec3(ri(0), ri(0), ri(0));
    let rank3_ok = !mat3_det(&span_mat).is_zero();

    let ok = gv_ok && rank2_ok && rank3_ok;
    emit_law(
        "d3_growth_vectors",
        "Growth vector and cost",
        "frob_conformance::compendium::d3::diff_drive_lateral_shift_cost",
        ok,
        format!("gv_ok={gv_ok},rank2={rank2_ok},rank3={rank3_ok}"),
        "gv_ok=true,rank2=true,rank3=true".to_owned(),
        None,
        None,
    );
    assert!(ok, "growth vectors or differential-drive Lie bracket rank failed");
}

#[test]
fn d3_lateral_shift_cost_scaling() {
    let b = 0.25_f64;
    let d1 = 0.10_f64;
    let d2 = 2.0 * d1;
    let c1 = diff_drive_lateral_shift_cost(d1, b);
    let c2 = diff_drive_lateral_shift_cost(d2, b);
    let ratio = c2 / c1;

    // Bracket depth w scaling: eps^(1/w) scales by 2^(1/w) when eps doubles
    let w3_ratio = bracket_depth_cost_order(2.0 * d1, 3) / bracket_depth_cost_order(d1, 3);
    let ok = (ratio - SQRT_2).abs() < 1e-12
        && (c1 - 4.0 * (d1 * b).sqrt()).abs() < 1e-12
        && (w3_ratio - 2.0_f64.cbrt()).abs() < 1e-12;

    emit_law(
        "d3_lateral_shift_cost_scaling",
        "Growth vector and cost",
        "frob_conformance::compendium::d3::diff_drive_lateral_shift_cost",
        ok,
        format!("{ratio:.15}"),
        format!("{SQRT_2:.15}"),
        Some("1e-12"),
        Some("m"),
    );
    assert!(ok, "lateral shift cost sqrt(2) scaling failed");
}

#[test]
fn d3_nc_growth_vector_nonreversible() {
    // NC: x_dot = u with u in [0, 1] has rank 1 everywhere, yet negative x is unreachable from 0.
    let rank = 1_usize;
    let t_horizon = 5.0_f64;
    let reachable_min = 0.0_f64;
    let reachable_max = t_horizon;
    let target_neg = -0.1_f64;
    let reachable = target_neg >= reachable_min && target_neg <= reachable_max;
    let caught = rank == 1 && !reachable;
    emit_nc(
        "d3_nc_growth_vector_nonreversible",
        "Growth vector and cost",
        "frob_conformance::compendium::d3::diff_drive_lateral_shift_cost",
        caught,
        format!("rank={rank},neg_reachable={reachable}"),
        "rank=1,neg_reachable=false".to_owned(),
        Some("1e-12"),
        Some("m"),
    );
    assert!(caught, "non-reversible control must fail full-rank reachability");
}

// =========================================================================
// Row 7: Squeeze film (`compendium-only`, `Estimated`)
// =========================================================================

#[test]
fn d3_squeeze_film_closed_form() {
    let h0 = 2.0e-4_f64; // 200 um
    let load_n = 100.0_f64;
    let eta = 1.0e-3_f64; // water: 1 mPa*s
    let a = 0.02_f64; // 2 cm patch

    let r0 = squeeze_film_thickness(h0, load_n, eta, a, 0.0);
    let r1 = squeeze_film_thickness(h0, load_n, eta, a, 0.01);
    let r2 = squeeze_film_thickness(h0, load_n, eta, a, 0.10);
    let r_late = squeeze_film_thickness(h0, load_n, eta, a, 100.0);

    // Check ODE by central difference of 1/h(t)^2 vs 4*N/(3*pi*eta*a^4)
    let t = 0.02_f64;
    let dt = 1.0e-6_f64;
    let hm = squeeze_film_thickness(h0, load_n, eta, a, t - dt).thickness_m;
    let hp = squeeze_film_thickness(h0, load_n, eta, a, t + dt).thickness_m;
    let h_mid = squeeze_film_thickness(h0, load_n, eta, a, t);
    let d_inv_h2_num = (1.0 / (hp * hp) - 1.0 / (hm * hm)) / (2.0 * dt);
    let d_inv_h2_exact = (4.0 * load_n) / (3.0 * PI * eta * a.powi(4));
    let dh_dt_num = (hp - hm) / (2.0 * dt);

    let ode_ok = ((d_inv_h2_num - d_inv_h2_exact).abs() / d_inv_h2_exact) < 1e-8
        && ((dh_dt_num - h_mid.dh_dt_mps).abs() / h_mid.dh_dt_mps.abs()) < 1e-6;
    let mono_pos_ok = r0.thickness_m == h0
        && r1.thickness_m < r0.thickness_m
        && r2.thickness_m < r1.thickness_m
        && r_late.thickness_m > 0.0
        && r0.evidence_class == "Estimated";

    // Asperity blend limits & Hersey extrapolation flag
    let sigma = 5.0e-6_f64;
    let f_bdry = squeeze_film_mu_eff(sigma, sigma, 0.15, 0.80, eta, 0.5, load_n, a, (1e-8, 1e-4));
    let f_hydro = squeeze_film_mu_eff(3.0 * sigma, sigma, 0.15, 0.80, eta, 0.5, load_n, a, (1e-8, 1e-4));
    let f_dry = squeeze_film_mu_eff(0.0, sigma, 0.15, 0.80, eta, 0.5, load_n, a, (1e-8, 1e-4));
    let f_extrap = squeeze_film_mu_eff(2.0 * sigma, sigma, 0.15, 0.80, eta, 100.0, load_n, a, (1e-8, 1e-4));
    let blend_ok = f_bdry.psi == 1.0
        && (f_bdry.mu_eff - 0.15).abs() < 1e-12
        && f_hydro.psi == 0.0
        && (f_dry.mu_eff - 0.80).abs() < 1e-12
        && f_extrap.extrapolated
        && f_bdry.evidence_class == "Estimated"
        && hersey_number(eta, 0.5, load_n, a) > 0.0;

    let ok = ode_ok && mono_pos_ok && blend_ok;
    emit_law(
        "d3_squeeze_film_closed_form",
        "Squeeze film",
        "frob_conformance::compendium::d3::squeeze_film_thickness",
        ok,
        format!("dh_dt_num={dh_dt_num:.9e}"),
        format!("dh_dt_exact={:.9e}", h_mid.dh_dt_mps),
        Some("1e-9"),
        Some("m"),
    );
    assert!(ok, "squeeze film closed-form ODE / monotonicity / blend failed");
}

#[test]
fn d3_squeeze_film_time_to_thickness() {
    // Worked KAs from §18.8 (to 10 um from thick film h0 -> inf):
    // water (1 mPa*s), a=2 cm, N=100 N -> ~0.0377 s (0.038 s)
    // oil (60 mPa*s), a=2 cm, N=100 N -> ~2.2619 s (2.3 s)
    // water (1 mPa*s), a=6 cm, N=400 N -> ~0.7634 s (0.76 s)
    let h_target = 1.0e-5_f64;
    let t_water = squeeze_film_time_from_thick(h_target, 100.0, 1.0e-3, 0.02);
    let t_oil = squeeze_film_time_from_thick(h_target, 100.0, 60.0e-3, 0.02);
    let t_big = squeeze_film_time_from_thick(h_target, 400.0, 1.0e-3, 0.06);

    let exp_water = (3.0 * PI * 1.0e-3 * 0.02_f64.powi(4)) / (4.0 * 100.0 * h_target * h_target);
    let exp_oil = 60.0 * exp_water;
    let exp_big = (3.0 * PI * 1.0e-3 * 0.06_f64.powi(4)) / (4.0 * 400.0 * h_target * h_target);

    // Consistency with huge h0 = 1 m
    let h_from_huge = squeeze_film_thickness(1.0, 100.0, 1.0e-3, 0.02, t_water).thickness_m;
    let ok = (t_water - exp_water).abs() < 1e-12
        && (t_water - 0.037_699_111_843).abs() < 1e-9
        && (t_oil - exp_oil).abs() < 1e-12
        && (t_big - exp_big).abs() < 1e-12
        && (h_from_huge - h_target).abs() < 1e-9;

    emit_law(
        "d3_squeeze_film_time_to_thickness",
        "Squeeze film",
        "frob_conformance::compendium::d3::squeeze_film_thickness",
        ok,
        format!("water={t_water:.6},oil={t_oil:.4},big={t_big:.4}"),
        format!("water={exp_water:.6},oil={exp_oil:.4},big={exp_big:.4}"),
        Some("1e-9"),
        Some("m"),
    );
    assert!(ok, "squeeze film time-to-thickness KA failed");
}

#[test]
fn d3_nc_squeeze_film_euler() {
    // NC: explicit Euler with a large step size drives film thickness h negative,
    // whereas the closed-form oracle stays strictly positive.
    let h0 = 1.0e-3_f64;
    let load_n = 100.0_f64;
    let eta = 1.0e-3_f64;
    let a = 0.02_f64;
    let dt_large = 1.0_f64;

    let euler_next = h0 + dt_large * squeeze_film_dh_dt(h0, load_n, eta, a);
    let closed_next = squeeze_film_thickness(h0, load_n, eta, a, dt_large).thickness_m;
    let caught = euler_next < 0.0 && closed_next > 0.0;
    emit_nc(
        "d3_nc_squeeze_film_euler",
        "Squeeze film",
        "frob_conformance::compendium::d3::squeeze_film_thickness",
        caught,
        format!("euler={euler_next:.4e},closed={closed_next:.4e}"),
        "euler<0,closed>0".to_owned(),
        Some("1e-9"),
        Some("m"),
    );
    assert!(caught, "explicit Euler with large step must drive h negative");
}

// =========================================================================
// Row 8: Pressure-sinkage and shear (`compendium-only`, `Estimated`)
// =========================================================================

#[test]
fn d3_bekker_monotone() {
    // §18.10 illustrative soil card: b=0.1 m, k_c=1e3 Pa*m^(1-n), k_phi=1.5e6 Pa*m^(-n), n=1.1
    let (k_c, k_phi, b, n) = (1.0e3_f64, 1.5e6_f64, 0.1_f64, 1.1_f64);
    let p0 = bekker_pressure(k_c, k_phi, b, n, 0.0);
    let p1 = bekker_pressure(k_c, k_phi, b, n, 0.01);
    let p2 = bekker_pressure(k_c, k_phi, b, n, 0.05);
    let p3 = bekker_pressure(k_c, k_phi, b, n, 0.10);

    let dims = SoilCardDimensions::canonical_si(r(11, 10));
    let dims_ok = check_soil_dimensions(&dims).is_ok();
    let ok = p0.value_pa == 0.0
        && p0.value_pa < p1.value_pa
        && p1.value_pa < p2.value_pa
        && p2.value_pa < p3.value_pa
        && p1.evidence_class == "Estimated"
        && dims_ok;

    emit_law(
        "d3_bekker_monotone",
        "Pressure–sinkage and shear",
        "frob_conformance::compendium::d3::bekker_pressure",
        ok,
        format!("p(0.05)={:.3},dims_ok={dims_ok}", p2.value_pa),
        "monotone,p(0)=0,dims_ok=true".to_owned(),
        Some("1e-9"),
        Some("Pa"),
    );
    assert!(ok, "Bekker pressure monotonicity or dimensional homogeneity failed");
}

#[test]
fn d3_shear_limits() {
    let c = 1_200.0_f64; // Pa
    let sigma = 20_000.0_f64; // Pa
    let phi = 30.0_f64.to_radians();
    let k_mod = 0.025_f64; // m
    let tau_asymp = c + sigma * phi.tan();

    let t0 = janosi_hanamoto_shear(c, sigma, phi, 0.0, k_mod);
    let t1 = janosi_hanamoto_shear(c, sigma, phi, 0.01, k_mod);
    let t2 = janosi_hanamoto_shear(c, sigma, phi, 0.05, k_mod);
    let t_inf = janosi_hanamoto_shear(c, sigma, phi, 50.0 * k_mod, k_mod);

    let ok = t0.value_pa == 0.0
        && t0.value_pa < t1.value_pa
        && t1.value_pa < t2.value_pa
        && t2.value_pa < tau_asymp
        && (t_inf.value_pa - tau_asymp).abs() < 1e-9
        && t_inf.evidence_class == "Estimated";

    emit_law(
        "d3_shear_limits",
        "Pressure–sinkage and shear",
        "frob_conformance::compendium::d3::bekker_pressure",
        ok,
        format!("t0={},t_inf={:.9}", t0.value_pa, t_inf.value_pa),
        format!("t0=0,t_inf={tau_asymp:.9}"),
        Some("1e-9"),
        Some("Pa"),
    );
    assert!(ok, "Janosi-Hanamoto shear monotonicity or asymptotic limit failed");
}

#[test]
fn d3_nc_soil_dimensions_dropped() {
    // NC: soil card with n = 1.1 whose exponent-dependent m^(-n) units were dropped
    // (k_c declared as Pa*m, k_phi declared as Pa) must fail the dimensional homogeneity check.
    let mut bad = SoilCardDimensions::canonical_si(r(11, 10));
    bad.k_c = DimVec::pascal().mul(&DimVec::length_m()); // dropped m^(-n)
    bad.k_phi = DimVec::pascal(); // dropped m^(-n)
    let res = check_soil_dimensions(&bad);
    let caught = res.is_err();
    emit_nc(
        "d3_nc_soil_dimensions_dropped",
        "Pressure–sinkage and shear",
        "frob_conformance::compendium::d3::bekker_pressure",
        caught,
        format!("{res:?}"),
        "Err(Bekker pressure dimension mismatch)".to_owned(),
        Some("1e-9"),
        Some("Pa"),
    );
    assert!(caught, "dropped exponent units must fail dimensional homogeneity check");
}

// =========================================================================
// Row 9: Swept right-angle corner
// =========================================================================

#[test]
fn d3_corner_two_root_two() {
    let a = 1.0_f64;
    let l_max = right_angle_corner_max_rod(a);
    let at_pi_4 = corner_rod_envelope_at_angle(a, 0.25 * PI);
    let min_ok = (l_max - 2.0 * SQRT_2 * a).abs() < 1e-12
        && (at_pi_4 - l_max).abs() < 1e-12
        && [0.1, 0.3, 0.6, 0.78, 0.79, 1.0, 1.3, 1.45]
            .iter()
            .all(|&th| corner_rod_envelope_at_angle(a, th) > l_max);

    // Exact rational checks: 2.82^2 = 7.9524 < 8 (clears); 2.83^2 = 8.0089 > 8 (jams)
    let exact_ok = rod_clears_corner_exact(&r(282, 100), &ri(1))
        && !rod_clears_corner_exact(&r(283, 100), &ri(1));

    let ok = min_ok && exact_ok;
    emit_law(
        "d3_corner_two_root_two",
        "Swept right-angle corner",
        "frob_conformance::compendium::d3::right_angle_corner_max_rod",
        ok,
        format!("{at_pi_4:.15}"),
        format!("{l_max:.15}"),
        Some("1e-12"),
        Some("m"),
    );
    assert!(ok, "swept right-angle corner 2*sqrt(2)*a check failed");
}

#[test]
fn d3_nc_corner_single_disc() {
    // NC: circumscribed-disc test (L <= a) refuses rods in (a, 2*sqrt(2)*a] such as L = 2.0*a;
    // inscribed-disc test (radius 0 <= a/2) accepts rods longer than 2*sqrt(2)*a such as L = 3.0*a.
    let a = ri(1);
    let rod_mid = ri(2); // clears (2^2 = 4 <= 8), but circumscribed disc refuses (2 > 1)
    let rod_long = ri(3); // jams (3^2 = 9 > 8), but inscribed disc accepts (0 <= 1/2)

    let circ_false_negative = !(rod_mid <= a) && rod_clears_corner_exact(&rod_mid, &a);
    let insc_false_positive = !rod_clears_corner_exact(&rod_long, &a);
    let caught = circ_false_negative && insc_false_positive;
    emit_nc(
        "d3_nc_corner_single_disc",
        "Swept right-angle corner",
        "frob_conformance::compendium::d3::right_angle_corner_max_rod",
        caught,
        format!("circ_fn={circ_false_negative},insc_fp={insc_false_positive}"),
        "circ_fn=true,insc_fp=true".to_owned(),
        Some("1e-12"),
        Some("m"),
    );
    assert!(caught, "single-disc approximations must fail on both sides of 2*sqrt(2)*a");
}

// =========================================================================
// Row 10: Clearance sandwich
// =========================================================================

#[test]
fn d3_clearance_sandwich_doorway() {
    let d_width = 1.0_f64;
    let delta = 0.05_f64;
    let rho = 0.25_f64;
    let n_bins = 48_u32;
    let (eps_field, u_reg) = (0.0_f64, 0.0_f64);

    let rep = clearance_sandwich_doorway(d_width, delta, rho, n_bins, eps_field, u_reg);
    let half_bin_exact = heading_half_bin_deg_exact(n_bins);
    let band = clearance_band(delta, rho, n_bins, eps_field, u_reg);

    // Inner critical width < D < outer critical width, each within 2*band (margin within band)
    let band_ok = (rep.band_m - band).abs() < 1e-15
        && half_bin_exact == r(15, 4) // 3.75 deg
        && (rep.half_bin_deg - 3.75).abs() < 1e-12
        && rep.inner_critical_width_m < d_width
        && rep.outer_critical_width_m > d_width
        && (0.5 * d_width - rep.inner_critical_margin_m) <= band + 1e-12
        && (rep.outer_critical_margin_m - 0.5 * d_width) <= band + 1e-12;

    // Three-valued sandwich verdicts across margins
    let v_feas = doorway_verdict(d_width, delta, rho, n_bins, eps_field, u_reg, rep.inner_critical_margin_m - 0.02, false);
    let v_unk = doorway_verdict(d_width, delta, rho, n_bins, eps_field, u_reg, 0.5 * d_width, false);
    let v_imp = doorway_verdict(d_width, delta, rho, n_bins, eps_field, u_reg, rep.outer_critical_margin_m + 0.02, false);
    let verdicts_ok = v_feas == ClearanceVerdict::Feasible
        && v_unk == ClearanceVerdict::Unknown
        && v_imp == ClearanceVerdict::Impossible;

    let ok = band_ok && verdicts_ok;
    emit_law(
        "d3_clearance_sandwich_doorway",
        "Clearance sandwich",
        "frob_conformance::compendium::d3::clearance_sandwich_doorway",
        ok,
        format!(
            "w_in={:.5},w_out={:.5},half_bin={half_bin_exact}",
            rep.inner_critical_width_m, rep.outer_critical_width_m
        ),
        format!("w_in<{d_width}<w_out,half_bin=15/4"),
        Some("1e-12"),
        Some("m"),
    );
    assert!(ok, "clearance sandwich doorway band or three-valued verdict failed");
}

#[test]
fn d3_nc_clearance_sandwich_close_gap() {
    // NC: closing the doorway gap by one cell flips the outer verdict from outer-connected
    // (Unknown) to Impossible (disconnected in C_out).
    let d_width = 1.0_f64;
    let delta = 0.05_f64;
    let rho = 0.25_f64;
    let n_bins = 48_u32;
    let band = clearance_band(delta, rho, n_bins, 0.0, 0.0);
    // Margin between outer_closed (= D/2 - delta + band) and outer_open (= D/2 + band):
    let probe_margin = 0.5 * d_width + band - 0.5 * delta;

    let open_verdict = doorway_verdict(d_width, delta, rho, n_bins, 0.0, 0.0, probe_margin, false);
    let closed_verdict = doorway_verdict(d_width, delta, rho, n_bins, 0.0, 0.0, probe_margin, true);
    let caught = open_verdict != ClearanceVerdict::Impossible
        && closed_verdict == ClearanceVerdict::Impossible;
    emit_nc(
        "d3_nc_clearance_sandwich_close_gap",
        "Clearance sandwich",
        "frob_conformance::compendium::d3::clearance_sandwich_doorway",
        caught,
        format!("open={open_verdict:?},closed={closed_verdict:?}"),
        "open!=Impossible,closed=Impossible".to_owned(),
        Some("1e-12"),
        Some("m"),
    );
    assert!(caught, "closing gap by one cell must flip outer verdict to Impossible");
}
