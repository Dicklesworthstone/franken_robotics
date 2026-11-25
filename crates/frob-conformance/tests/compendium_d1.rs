#![deny(unsafe_code)]

//! Appendix D.1 exact mathematical law tests, known answers, and negative controls (`N12-41`).
//!
//! Every one of the 21 rows in `registries/compendium/d1.toml` has at least one law/KA test
//! (`d1_*`) and one explicit negative control (`d1_nc_*`) that verifies the wrong formula fails.
//! Every test emits a `frob.testevidence/1` record when `FROB_TEST_EVIDENCE_DIR` is set.

use frob_conformance::compendium::d1::{
    Momentum, Pose, RevoluteScrew, Twist, TwistMap, UnitComplex, Wrench, WrenchPullback,
    WrenchPushforward, ad_matrix, ad_star, ad_twist, adjoint_matrix, adjoint_twist,
    box_shell_pseudo_inertia, bracket, cayley_angle, cayley_se3, cayley_so3, coadjoint_matrix,
    coadjoint_wrench, connection_curvature_so3, constant_twist_chord, dcay_inv_se3, dcay_inv_so3,
    dcay_so3, euler_poincare_free_body, inv_eye_minus_half_hat, mass_support_holds,
    mass_support_lhs, mat3_diag_int, mat3_inverse_exact, mat3_is_so3, mat6_compose,
    mat6_inverse_transpose, mechanical_connection_col, mobility_count, momentum_map,
    momentum_pairing, parity_axial, parity_transform_twist, parity_transform_wrench,
    planar_cayley_step, poe_forward_kinematics, poe_space_jacobian, power_pairing,
    pseudo_inertia_from_rotational, pseudo_inertia_is_consistent, reconstruct_body_velocity,
    rotation_from_rational_quat, rotational_inertia_from_sigma, sigma_from_rotational_inertia,
    solid_box_pseudo_inertia, space_jacobian_to_point_velocity, spherical_shell_inertia_ratio,
    wrench_pullback_matrix,
};
use frob_conformance::compendium::rat::{
    Mat3, Mat6, Rat, Vec6, cross3, dot3, dot6, hat3, mat3_add, mat3_diag, mat3_eye,
    mat3_mul, mat3_scale, mat3_sub, mat3_transpose, mat3_vec, mat3_zero, mat6_eye,
    mat6_from_blocks, mat6_mul, mat6_transpose, mat6_vec, outer3, r, ri, vec3, vec3_add, vec3_neg,
    vec3_scale, vec3_sub, vec3_zero, vee3,
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
        format!("D.1:{row_name}"),
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
        format!("D.1:{row_name}"),
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

fn sample_pose_1() -> Pose {
    let rot = rotation_from_rational_quat(ri(2), ri(1), ri(-1), ri(3));
    Pose::new(rot, vec3(r(3, 2), r(-5, 4), r(7, 3)))
}

fn sample_pose_2() -> Pose {
    let rot = rotation_from_rational_quat(ri(1), ri(-2), ri(3), ri(1));
    Pose::new(rot, vec3(r(-2, 5), r(11, 7), r(-1, 3)))
}

fn sample_twist_a() -> Twist {
    Twist::new(
        vec3(r(1, 2), r(-2, 3), r(5, 4)),
        vec3(r(3, 5), r(-1, 7), r(4, 3)),
    )
}

fn sample_twist_b() -> Twist {
    Twist::new(
        vec3(r(-3, 4), r(7, 5), r(-1, 2)),
        vec3(r(-2, 3), r(5, 6), r(-7, 4)),
    )
}

fn sample_twist_c() -> Twist {
    Twist::new(
        vec3(r(2, 5), r(1, 3), r(-4, 7)),
        vec3(r(5, 2), r(-3, 8), r(1, 5)),
    )
}

fn add_twists(a: &Twist, b: &Twist) -> Twist {
    Twist::new(vec3_add(&a.omega, &b.omega), vec3_add(&a.v, &b.v))
}

// =========================================================================
// Row 1: Bracket on se(3)
// =========================================================================

#[test]
fn d1_bracket_jacobi() {
    let a = sample_twist_a();
    let b = sample_twist_b();
    let c = sample_twist_c();

    // Antisymmetry: [a, b] = -[b, a]
    let ab = bracket(&a, &b);
    let ba = bracket(&b, &a);
    let anti_ok = ab == Twist::new(vec3_neg(&ba.omega), vec3_neg(&ba.v));

    // Jacobi: [a,[b,c]] + [b,[c,a]] + [c,[a,b]] = 0
    let j1 = bracket(&a, &bracket(&b, &c));
    let j2 = bracket(&b, &bracket(&c, &a));
    let j3 = bracket(&c, &bracket(&a, &b));
    let jac = add_twists(&add_twists(&j1, &j2), &j3);
    let zero_twist = Twist::new(vec3_zero(), vec3_zero());
    let jacobi_ok = jac == zero_twist;

    // KA: [(e_x, 0), (e_y, 0)] = (e_z, 0)
    let ex = Twist::new(vec3(ri(1), ri(0), ri(0)), vec3_zero());
    let ey = Twist::new(vec3(ri(0), ri(1), ri(0)), vec3_zero());
    let ez = Twist::new(vec3(ri(0), ri(0), ri(1)), vec3_zero());
    let ka_ok = bracket(&ex, &ey) == ez;

    let ok = anti_ok && jacobi_ok && ka_ok;
    emit_law(
        "d1_bracket_jacobi",
        "Bracket on 𝔰𝔢(3)",
        "frob_conformance::compendium::d1::bracket",
        ok,
        format!("anti={anti_ok},jacobi={jacobi_ok},ka={ka_ok}"),
        "anti=true,jacobi=true,ka=true".to_owned(),
        None,
        None,
    );
    assert!(ok, "bracket antisymmetry/Jacobi/KA failed");
}

#[test]
fn d1_nc_bracket() {
    // Wrong formula: second term uses + instead of -.
    let wrong_bracket = |x1: &Twist, x2: &Twist| -> Twist {
        Twist::new(
            cross3(&x1.omega, &x2.omega),
            vec3_add(&cross3(&x1.omega, &x2.v), &cross3(&x2.omega, &x1.v)),
        )
    };
    let a = sample_twist_a();
    let b = sample_twist_b();
    let c = sample_twist_c();
    let j = add_twists(
        &add_twists(&wrong_bracket(&a, &wrong_bracket(&b, &c)), &wrong_bracket(&b, &wrong_bracket(&c, &a))),
        &wrong_bracket(&c, &wrong_bracket(&a, &b)),
    );
    let caught = j != Twist::new(vec3_zero(), vec3_zero());
    emit_nc(
        "d1_nc_bracket",
        "Bracket on 𝔰𝔢(3)",
        "frob_conformance::compendium::d1::bracket",
        caught,
        format!("jacobi_residual={:?}", j.v),
        "non-zero residual".to_owned(),
        None,
        None,
    );
    assert!(caught, "wrong + sign bracket must fail Jacobi identity");
}

// =========================================================================
// Row 2: Adjoint
// =========================================================================

#[test]
fn d1_adjoint_homomorphism() {
    let g = sample_pose_1();
    let h = sample_pose_2();
    let gh = g.compose(&h);

    let ad_gh = adjoint_matrix(&gh);
    let ad_g = adjoint_matrix(&g);
    let ad_h = adjoint_matrix(&h);
    let homo_ok = ad_gh == mat6_compose(&ad_g, &ad_h);

    let xi1 = sample_twist_a();
    let xi2 = sample_twist_b();
    let action_ok = Twist::from_vec6(&mat6_vec(&ad_g, &xi1.to_vec6())) == adjoint_twist(&g, &xi1);
    let bracket_ok = adjoint_twist(&g, &bracket(&xi1, &xi2))
        == bracket(&adjoint_twist(&g, &xi1), &adjoint_twist(&g, &xi2));

    let ok = homo_ok && action_ok && bracket_ok;
    emit_law(
        "d1_adjoint_homomorphism",
        "Adjoint",
        "frob_conformance::compendium::d1::adjoint_matrix",
        ok,
        format!("homo={homo_ok},action={action_ok},bracket={bracket_ok}"),
        "homo=true,action=true,bracket=true".to_owned(),
        None,
        None,
    );
    assert!(ok, "adjoint homomorphism or bracket preservation failed");
}

#[test]
fn d1_nc_adjoint() {
    let g = sample_pose_1();
    let h = sample_pose_2();
    let ad_gh = adjoint_matrix(&g.compose(&h));
    let reversed = mat6_compose(&adjoint_matrix(&h), &adjoint_matrix(&g));
    let caught = ad_gh != reversed;
    emit_nc(
        "d1_nc_adjoint",
        "Adjoint",
        "frob_conformance::compendium::d1::adjoint_matrix",
        caught,
        format!("equal_reversed={}", !caught),
        "equal_reversed=false".to_owned(),
        None,
        None,
    );
    assert!(caught, "Ad(h)*Ad(g) must differ from Ad(g*h)");
}

// =========================================================================
// Row 3: Group coadjoint
// =========================================================================

#[test]
fn d1_coadjoint_inverse_transpose() {
    let g = sample_pose_1();
    let ad_g = adjoint_matrix(&g);
    let coad_g = coadjoint_matrix(&g);
    let inv_t = mat6_inverse_transpose(&ad_g).expect("Ad_g invertible");
    let inv_ok = coad_g == inv_t && mat6_mul(&mat6_transpose(&ad_g), &coad_g) == mat6_eye();

    // KA: R = I, p = (1, 0, 0) m, wrench (0, (0, 0, 1) N) -> torque (0, -1, 0) N*m, force (0, 0, 1) N
    let g_ka = Pose::new(mat3_eye(), vec3(ri(1), ri(0), ri(0)));
    let w_in = Wrench::new(vec3_zero(), vec3(ri(0), ri(0), ri(1)));
    let w_out = coadjoint_wrench(&g_ka, &w_in);
    let w_exp = Wrench::new(vec3(ri(0), ri(-1), ri(0)), vec3(ri(0), ri(0), ri(1)));
    let ka_ok = w_out == w_exp
        && Wrench::from_vec6(&mat6_vec(&coadjoint_matrix(&g_ka), &w_in.to_vec6())) == w_exp;

    let ok = inv_ok && ka_ok;
    emit_law(
        "d1_coadjoint_inverse_transpose",
        "Group coadjoint",
        "frob_conformance::compendium::d1::coadjoint_matrix",
        ok,
        format!("inv_t={inv_ok},ka_tau={:?}", w_out.tau),
        "inv_t=true,ka_tau=[0, -1, 0]".to_owned(),
        None,
        Some("N*m"),
    );
    assert!(ok, "group coadjoint inverse-transpose or KA failed");
}

#[test]
fn d1_nc_group_coadjoint() {
    // Wrong sign on moment arm: R*tau - p x R*f
    let g_ka = Pose::new(mat3_eye(), vec3(ri(1), ri(0), ri(0)));
    let w_in = Wrench::new(vec3_zero(), vec3(ri(0), ri(0), ri(1)));
    let rf = mat3_vec(&g_ka.r, &w_in.f);
    let wrong_tau = vec3_sub(&mat3_vec(&g_ka.r, &w_in.tau), &cross3(&g_ka.p, &rf));
    let expected_tau = vec3(ri(0), ri(-1), ri(0));
    let caught = wrong_tau != expected_tau;
    emit_nc(
        "d1_nc_group_coadjoint",
        "Group coadjoint",
        "frob_conformance::compendium::d1::coadjoint_matrix",
        caught,
        format!("wrong_tau={wrong_tau:?}"),
        format!("expected_tau={expected_tau:?}"),
        None,
        Some("N*m"),
    );
    assert!(caught, "flipped moment-arm sign must disagree with KA");
}

// =========================================================================
// Row 4: Algebra adjoint and coadjoint
// =========================================================================

#[test]
fn d1_ad_star_pairing() {
    let xi = sample_twist_a();
    let eta = sample_twist_b();
    let mu = Momentum::new(
        vec3(r(5, 3), r(-7, 2), r(4, 5)),
        vec3(r(-1, 4), r(9, 5), r(-3, 2)),
    );

    let ad_ok = ad_twist(&xi, &eta) == bracket(&xi, &eta)
        && Twist::from_vec6(&mat6_vec(&ad_matrix(&xi), &eta.to_vec6())) == bracket(&xi, &eta);

    // Defining 18-variable pairing identity: <ad*_xi mu, eta> == <mu, [xi, eta]>
    let lhs = momentum_pairing(&ad_star(&xi, &mu), &eta);
    let rhs = momentum_pairing(&mu, &bracket(&xi, &eta));
    let pairing_ok = !rhs.is_zero() && lhs == rhs;
    let ok = ad_ok && pairing_ok;

    emit_law(
        "d1_ad_star_pairing",
        "Algebra adjoint and coadjoint",
        "frob_conformance::compendium::d1::ad_star",
        ok,
        format!("{lhs}"),
        format!("{rhs}"),
        None,
        None,
    );
    assert!(
        ok,
        "ad*_xi pairing identity failed: <ad*_xi mu, eta> = {lhs} != <mu, [xi, eta]> = {rhs}"
    );
}

#[test]
fn d1_nc_algebra_adjoint_coadjoint() {
    let xi = sample_twist_a();
    let eta = sample_twist_b();
    let mu = Momentum::new(
        vec3(r(5, 3), r(-7, 2), r(4, 5)),
        vec3(r(-1, 4), r(9, 5), r(-3, 2)),
    );
    // Wrong convention: -ad_xi^T mu
    let ad_t_mu = mat6_vec(&mat6_transpose(&ad_matrix(&xi)), &mu.to_vec6());
    let neg_ad_t_mu = Momentum::from_vec6(&std::array::from_fn(|i| -&ad_t_mu[i]));
    let lhs = momentum_pairing(&neg_ad_t_mu, &eta);
    let rhs = momentum_pairing(&mu, &bracket(&xi, &eta));
    let caught = !rhs.is_zero() && lhs == -&rhs && lhs != rhs;
    emit_nc(
        "d1_nc_algebra_adjoint_coadjoint",
        "Algebra adjoint and coadjoint",
        "frob_conformance::compendium::d1::ad_star",
        caught,
        format!("{lhs}"),
        format!("{rhs}"),
        None,
        None,
    );
    assert!(caught, "-ad_xi^T must fail defining pairing by factor -1");
}

// =========================================================================
// Row 5: Three typed frame maps
// =========================================================================

#[test]
fn d1_frame_maps_inverse() {
    let g = sample_pose_1();
    let twist_map = TwistMap::from_pose(&g);
    let push = WrenchPushforward::from_pose(&g);
    let pull = WrenchPullback::from_pose(&g);

    let inv_ok = mat6_mul(pull.matrix(), push.matrix()) == mat6_eye()
        && mat6_mul(push.matrix(), pull.matrix()) == mat6_eye()
        && *pull.matrix() == wrench_pullback_matrix(&g);

    let xi = sample_twist_a();
    let w = Wrench::new(
        vec3(r(2, 3), r(-5, 4), r(7, 6)),
        vec3(r(-1, 2), r(3, 5), r(11, 4)),
    );
    let roundtrip_ok = pull.apply(&push.apply(&w)) == w && push.apply(&pull.apply(&w)) == w;
    let dual_ok = power_pairing(&push.apply(&w), &twist_map.apply(&xi)) == power_pairing(&w, &xi);

    let ok = inv_ok && roundtrip_ok && dual_ok;
    emit_law(
        "d1_frame_maps_inverse",
        "Three typed frame maps",
        "frob_conformance::compendium::d1::wrench_pullback_matrix",
        ok,
        format!("inv={inv_ok},rt={roundtrip_ok},dual={dual_ok}"),
        "inv=true,rt=true,dual=true".to_owned(),
        None,
        None,
    );
    assert!(ok, "three typed frame maps inverse/duality check failed");
}

#[test]
fn d1_nc_three_typed_frame_maps() {
    // NC: twist map Ad_g applied to the KA wrench (0, (0,0,1) N) at p=(1,0,0) gives zero torque.
    let g_ka = Pose::new(mat3_eye(), vec3(ri(1), ri(0), ri(0)));
    let w_in = Wrench::new(vec3_zero(), vec3(ri(0), ri(0), ri(1)));
    let wrong_moved = Wrench::from_vec6(&mat6_vec(TwistMap::from_pose(&g_ka).matrix(), &w_in.to_vec6()));
    let right_moved = WrenchPushforward::from_pose(&g_ka).apply(&w_in);
    let caught = wrong_moved.tau == vec3_zero() && wrong_moved != right_moved;
    emit_nc(
        "d1_nc_three_typed_frame_maps",
        "Three typed frame maps",
        "frob_conformance::compendium::d1::wrench_pullback_matrix",
        caught,
        format!("twist_map_tau={:?}", wrong_moved.tau),
        format!("pushforward_tau={:?}", right_moved.tau),
        None,
        Some("N*m"),
    );
    assert!(caught, "moving wrench with twist map must lose moment arm");
}

// =========================================================================
// Row 6: Power pairing
// =========================================================================

#[test]
fn d1_pairing_invariant() {
    let g = sample_pose_1();
    let xi = sample_twist_a();
    let w = Wrench::new(
        vec3(r(3, 4), r(-1, 5), r(7, 2)),
        vec3(r(5, 3), r(-2, 7), r(1, 4)),
    );
    let p_b = power_pairing(&w, &xi);
    let p_a = power_pairing(&coadjoint_wrench(&g, &w), &adjoint_twist(&g, &xi));
    let inv_ok = p_a == p_b;

    // KA: planar 2R arm (L1=0.4 m, L2=0.3 m, theta=(0, pi/2), f_tool=(10, 0) N, qdot=(0.2, -0.1) rad/s)
    let screws = [
        RevoluteScrew::from_axis_point(vec3(ri(0), ri(0), ri(1)), &vec3_zero()),
        RevoluteScrew::from_axis_point(vec3(ri(0), ri(0), ri(1)), &vec3(r(2, 5), ri(0), ri(0))),
    ];
    let cs = [(ri(1), ri(0)), (ri(0), ri(1))];
    let home = Pose::new(mat3_eye(), vec3(r(7, 10), ri(0), ri(0)));
    let tool = poe_forward_kinematics(&screws, &cs, &home).p;
    let j_space = poe_space_jacobian(&screws, &cs);
    let j_p = space_jacobian_to_point_velocity(&j_space, &tool);

    let qdot = [r(1, 5), r(-1, 10)];
    let f_tool = vec3(ri(10), ri(0), ri(0));
    let v_tool = vec3_add(&vec3_scale(&qdot[0], &j_p[0]), &vec3_scale(&qdot[1], &j_p[1]));
    let tau_joint = [dot3(&j_p[0], &f_tool), dot3(&j_p[1], &f_tool)];

    let power_task = power_pairing(&Wrench::new(vec3_zero(), f_tool), &Twist::new(vec3_zero(), v_tool));
    let power_joint = &(&tau_joint[0] * &qdot[0]) + &(&tau_joint[1] * &qdot[1]);
    let expected_power = r(-3, 10);
    let ka_ok = power_task == expected_power && power_joint == expected_power;

    let ok = inv_ok && ka_ok;
    emit_law(
        "d1_pairing_invariant",
        "Power pairing",
        "frob_conformance::compendium::d1::power_pairing",
        ok,
        format!("task={power_task},joint={power_joint}"),
        format!("{expected_power}"),
        None,
        Some("W"),
    );
    assert!(ok, "power pairing invariance or planar 2R KA failed");
}

#[test]
fn d1_nc_power_pairing() {
    // Mixed-frame error: wrench in frame b paired with twist in frame a.
    let g = sample_pose_1();
    let xi_b = sample_twist_a();
    let w_b = Wrench::new(
        vec3(r(3, 4), r(-1, 5), r(7, 2)),
        vec3(r(5, 3), r(-2, 7), r(1, 4)),
    );
    let true_power = power_pairing(&w_b, &xi_b);
    let mixed_power = power_pairing(&w_b, &adjoint_twist(&g, &xi_b));
    let caught = mixed_power != true_power;
    emit_nc(
        "d1_nc_power_pairing",
        "Power pairing",
        "frob_conformance::compendium::d1::power_pairing",
        caught,
        format!("{mixed_power}"),
        format!("{true_power}"),
        None,
        Some("W"),
    );
    assert!(caught, "mixed-frame pairing must violate power invariance");
}

// =========================================================================
// Row 7: Reflection parity
// =========================================================================

#[test]
fn d1_parity_preserves_bracket() {
    let mirror_y = mat3_diag_int(1, -1, 1);
    let rot = rotation_from_rational_quat(ri(2), ri(-1), ri(2), ri(1));
    let p = mat3_mul(&mirror_y, &rot);

    let xi1 = sample_twist_a();
    let xi2 = sample_twist_b();
    let w = Wrench::new(
        vec3(r(1, 3), r(-4, 5), r(2, 7)),
        vec3(r(-5, 2), r(3, 4), r(1, 6)),
    );

    let bracket_ok = parity_transform_twist(&p, &bracket(&xi1, &xi2))
        == bracket(&parity_transform_twist(&p, &xi1), &parity_transform_twist(&p, &xi2));
    let pairing_ok = power_pairing(&parity_transform_wrench(&p, &w), &parity_transform_twist(&p, &xi1))
        == power_pairing(&w, &xi1);

    // KA: P = diag(1, -1, 1) -> axial e_y -> e_y, e_x -> -e_x, e_z -> -e_z
    let ex = vec3(ri(1), ri(0), ri(0));
    let ey = vec3(ri(0), ri(1), ri(0));
    let ez = vec3(ri(0), ri(0), ri(1));
    let ka_ok = parity_axial(&mirror_y, &ey) == ey
        && parity_axial(&mirror_y, &ex) == vec3_neg(&ex)
        && parity_axial(&mirror_y, &ez) == vec3_neg(&ez);

    let ok = bracket_ok && pairing_ok && ka_ok;
    emit_law(
        "d1_parity_preserves_bracket",
        "Reflection parity",
        "frob_conformance::compendium::d1::parity_transform_twist",
        ok,
        format!("bracket={bracket_ok},pairing={pairing_ok},ka={ka_ok}"),
        "bracket=true,pairing=true,ka=true".to_owned(),
        None,
        None,
    );
    assert!(ok, "reflection parity preservation or KA failed");
}

#[test]
fn d1_nc_reflection_parity() {
    // Wrong rule: map axial vectors by P (missing det(P) = -1 factor).
    let mirror_y = mat3_diag_int(1, -1, 1);
    let wrong_map = |xi: &Twist| Twist::new(mat3_vec(&mirror_y, &xi.omega), mat3_vec(&mirror_y, &xi.v));
    let xi1 = sample_twist_a();
    let xi2 = sample_twist_b();
    let lhs = wrong_map(&bracket(&xi1, &xi2));
    let rhs = bracket(&wrong_map(&xi1), &wrong_map(&xi2));
    let caught = lhs != rhs;
    emit_nc(
        "d1_nc_reflection_parity",
        "Reflection parity",
        "frob_conformance::compendium::d1::parity_transform_twist",
        caught,
        format!("wrong_lhs_omega={:?}", lhs.omega),
        format!("bracket_mapped_omega={:?}", rhs.omega),
        None,
        None,
    );
    assert!(caught, "polar mapping on axial vectors must break Lie bracket");
}

// =========================================================================
// Row 8: Euler-Poincare, free body
// =========================================================================

#[test]
fn d1_free_body_velocity_constant() {
    let rot = rotation_from_rational_quat(ri(1), ri(2), ri(-1), ri(2));
    let mass = r(7, 2);
    let inertia = mat3_diag_int(1, 2, 3);
    let xi = sample_twist_a();
    let mu = Momentum::new(mat3_vec(&inertia, &xi.omega), vec3_scale(&mass, &xi.v));
    let zero_w = Wrench::new(vec3_zero(), vec3_zero());

    let mu_dot = euler_poincare_free_body(&xi, &mu, &zero_w);
    let v_dot = vec3_scale(&(&Rat::one() / &mass), &mu_dot.ell);
    // d(R*v)/dt = R*(omega x v + v_dot) must be identically 0
    let spatial_acc = mat3_vec(&rot, &vec3_add(&cross3(&xi.omega, &xi.v), &v_dot));
    let vel_const_ok = spatial_acc == vec3_zero();

    // KA: inertia diag(1, 2, 3), omega = (1, 1, 0), v = 0, w = 0 -> pi_dot = (0, 0, -1)
    let w_ka = vec3(ri(1), ri(1), ri(0));
    let mu_ka = Momentum::new(mat3_vec(&inertia, &w_ka), vec3_zero());
    let dot_ka = euler_poincare_free_body(&Twist::new(w_ka, vec3_zero()), &mu_ka, &zero_w);
    let ka_exp = vec3(ri(0), ri(0), ri(-1));
    let ka_ok = dot_ka.pi == ka_exp;

    let ok = vel_const_ok && ka_ok;
    emit_law(
        "d1_free_body_velocity_constant",
        "Euler–Poincaré, free body",
        "frob_conformance::compendium::d1::euler_poincare_free_body",
        ok,
        format!("spatial_acc={spatial_acc:?},ka_pi_dot={:?}", dot_ka.pi),
        format!("spatial_acc=[0, 0, 0],ka_pi_dot={ka_exp:?}"),
        None,
        None,
    );
    assert!(ok, "free-body Euler-Poincare invariance or KA failed");
}

#[test]
fn d1_nc_euler_poincare_free_body() {
    // Opposite sign: mu_dot = -ad*_xi mu gives d(R*v)/dt = 2*R*(omega x v) != 0.
    let rot = rotation_from_rational_quat(ri(1), ri(2), ri(-1), ri(2));
    let mass = r(7, 2);
    let inertia = mat3_diag_int(1, 2, 3);
    let xi = sample_twist_a();
    let mu = Momentum::new(mat3_vec(&inertia, &xi.omega), vec3_scale(&mass, &xi.v));

    let ad = ad_star(&xi, &mu);
    let wrong_ell_dot = vec3_neg(&ad.ell);
    let wrong_v_dot = vec3_scale(&(&Rat::one() / &mass), &wrong_ell_dot);
    let spatial_acc = mat3_vec(&rot, &vec3_add(&cross3(&xi.omega, &xi.v), &wrong_v_dot));
    let two_r_w_x_v = vec3_scale(&ri(2), &mat3_vec(&rot, &cross3(&xi.omega, &xi.v)));
    let caught = spatial_acc != vec3_zero() && spatial_acc == two_r_w_x_v;

    emit_nc(
        "d1_nc_euler_poincare_free_body",
        "Euler–Poincaré, free body",
        "frob_conformance::compendium::d1::euler_poincare_free_body",
        caught,
        format!("{spatial_acc:?}"),
        format!("{two_r_w_x_v:?}"),
        None,
        None,
    );
    assert!(caught, "opposite-sign Euler-Poincare must yield 2*R*(omega x v)");
}

// =========================================================================
// Row 9: Momentum map
// =========================================================================

fn two_body_fixture() -> (Mat6, Vec6, Rat) {
    // Locked inertia (SPD 6x6): rotational diag(5, 6, 7), linear mass 4*I, slight off-diag coupling
    let tl = mat3_diag_int(5, 6, 7);
    let br = mat3_diag_int(4, 4, 4);
    let off: Mat3 = [
        [ri(0), ri(0), ri(1)],
        [ri(0), ri(0), ri(0)],
        [ri(-1), ri(0), ri(0)],
    ];
    let locked = mat6_from_blocks(&tl, &off, &mat3_transpose(&off), &br);
    let coupling_bs: Vec6 = [r(1, 2), r(-1, 3), ri(2), r(1, 4), ri(0), r(-1, 2)];
    let i_ss = ri(3);
    (locked, coupling_bs, i_ss)
}

#[test]
fn d1_momentum_map_two_body() {
    let (locked, coupling_bs, i_ss) = two_body_fixture();
    let conn = mechanical_connection_col(&locked, &coupling_bs).expect("locked inertia invertible");
    let xi = sample_twist_a();
    let s_dot = r(3, 5);

    let mu_conn = momentum_map(&locked, &conn, &xi, &s_dot);
    let xi_v = xi.to_vec6();
    let i_xi = mat6_vec(&locked, &xi_v);
    let mu_direct = Momentum::from_vec6(&std::array::from_fn(|k| &i_xi[k] + &(&coupling_bs[k] * &s_dot)));
    let mu_ok = mu_conn == mu_direct;

    // Kinetic energy block decomposition:
    // 2T = xi^T I xi + 2 xi^T I_BS s_dot + I_SS s_dot^2
    //    = (xi + A s_dot)^T I (xi + A s_dot) + s_dot^2 * (I_SS - I_BS^T A)
    let two_t_block = &(&dot6(&xi_v, &i_xi) + &(&ri(2) * &(&dot6(&xi_v, &coupling_bs) * &s_dot)))
        + &(&i_ss * &(&s_dot * &s_dot));
    let shifted: Vec6 = std::array::from_fn(|k| &xi_v[k] + &(&conn[k] * &s_dot));
    let reduced_i_ss = &i_ss - &dot6(&coupling_bs, &conn);
    let two_t_conn = &dot6(&shifted, &mat6_vec(&locked, &shifted)) + &(&reduced_i_ss * &(&s_dot * &s_dot));
    let ke_ok = two_t_block == two_t_conn;

    let ok = mu_ok && ke_ok;
    emit_law(
        "d1_momentum_map_two_body",
        "Momentum map",
        "frob_conformance::compendium::d1::momentum_map",
        ok,
        format!("mu_ok={mu_ok},2T={two_t_conn}"),
        format!("mu_ok=true,2T={two_t_block}"),
        None,
        None,
    );
    assert!(ok, "momentum map or block kinetic energy consistency failed");
}

#[test]
fn d1_nc_momentum_map() {
    let (locked, coupling_bs, _) = two_body_fixture();
    let conn = mechanical_connection_col(&locked, &coupling_bs).expect("invertible");
    let xi = sample_twist_a();
    let s_dot = r(3, 5);
    let true_mu = momentum_map(&locked, &conn, &xi, &s_dot);
    // Wrong formula: drop the shape coupling A*s_dot
    let wrong_mu = Momentum::from_vec6(&mat6_vec(&locked, &xi.to_vec6()));
    let caught = wrong_mu != true_mu;
    emit_nc(
        "d1_nc_momentum_map",
        "Momentum map",
        "frob_conformance::compendium::d1::momentum_map",
        caught,
        format!("{:?}", wrong_mu.pi),
        format!("{:?}", true_mu.pi),
        None,
        None,
    );
    assert!(caught, "dropping shape coupling must violate momentum map");
}

// =========================================================================
// Row 10: Reconstruction
// =========================================================================

#[test]
fn d1_beanie_reconstruction() {
    // KA: two coaxial bodies (inertias I1=3, I2=2 about z), zero momentum.
    let i1 = ri(3);
    let i2 = ri(2);
    let locked = mat6_from_blocks(
        &mat3_diag(&[ri(1), ri(1), &i1 + &i2]),
        &mat3_zero(),
        &mat3_zero(),
        &mat3_eye(),
    );
    let coupling_bs: Vec6 = [ri(0), ri(0), i2.clone(), ri(0), ri(0), ri(0)];
    let conn = mechanical_connection_col(&locked, &coupling_bs).expect("invertible");
    let delta_psi = r(5, 7);
    let zero_p = Momentum::new(vec3_zero(), vec3_zero());

    let xi = reconstruct_body_velocity(&locked, &conn, &delta_psi, &zero_p).expect("reconstruct");
    let expected_wz = -(&(&i2 / &(&i1 + &i2)) * &delta_psi);
    let ka_ok = xi.omega[2] == expected_wz
        && momentum_map(&locked, &conn, &xi, &delta_psi) == zero_p;

    // Non-zero momentum p round-trip: momentum_map(reconstruct(p)) == p
    let p_nz = Momentum::new(vec3(r(1, 2), r(-3, 4), r(7, 3)), vec3(r(-2, 5), r(1, 3), r(4, 7)));
    let xi_nz = reconstruct_body_velocity(&locked, &conn, &delta_psi, &p_nz).expect("reconstruct nz");
    let nz_ok = momentum_map(&locked, &conn, &xi_nz, &delta_psi) == p_nz;

    let ok = ka_ok && nz_ok;
    emit_law(
        "d1_beanie_reconstruction",
        "Reconstruction",
        "frob_conformance::compendium::d1::reconstruct_body_velocity",
        ok,
        format!("wz={},nz_ok={nz_ok}", xi.omega[2]),
        format!("wz={expected_wz},nz_ok=true"),
        None,
        None,
    );
    assert!(ok, "beanie reconstruction KA or non-zero momentum check failed");
}

#[test]
fn d1_nc_reconstruction() {
    // Wrong sign: +A*s_dot instead of -A*s_dot fails zero momentum conservation.
    let i1 = ri(3);
    let i2 = ri(2);
    let locked = mat6_from_blocks(
        &mat3_diag(&[ri(1), ri(1), &i1 + &i2]),
        &mat3_zero(),
        &mat3_zero(),
        &mat3_eye(),
    );
    let coupling_bs: Vec6 = [ri(0), ri(0), i2.clone(), ri(0), ri(0), ri(0)];
    let conn = mechanical_connection_col(&locked, &coupling_bs).expect("invertible");
    let s_dot = r(5, 7);
    let wrong_xi = Twist::from_vec6(&std::array::from_fn(|k| &conn[k] * &s_dot));
    let residual = momentum_map(&locked, &conn, &wrong_xi, &s_dot);
    let caught = residual != Momentum::new(vec3_zero(), vec3_zero());
    emit_nc(
        "d1_nc_reconstruction",
        "Reconstruction",
        "frob_conformance::compendium::d1::reconstruct_body_velocity",
        caught,
        format!("residual_pi_z={}", residual.pi[2]),
        "0".to_owned(),
        None,
        None,
    );
    assert!(caught, "+A*s_dot must violate zero-momentum conservation");
}

// =========================================================================
// Row 11: Curvature of the connection
// =========================================================================

#[test]
fn d1_curvature_so3_fixture() {
    let zero = vec3_zero();
    let ex = vec3(ri(1), ri(0), ri(0));
    let ey = vec3(ri(0), ri(1), ri(0));
    let ez = vec3(ri(0), ri(0), ri(1));

    // KA: A1 = e_x, A2 = e_y -> (DA)_12 = -e_z, holonomy = exp(-(DA)_12 * da) = exp(+da * e_z)
    let da_12 = connection_curvature_so3(&zero, &zero, &ex, &ey);
    let ka_ok = da_12 == vec3_neg(&ez);

    // Independent small-loop ordered product verification on SO(3):
    // Along each leg d(s_k) = +-h, body velocity is xi = -A_k * s_dot -> increment cay(-A_k * delta_s).
    // Loop (+h along 1, +h along 2, -h along 1, -h along 2):
    // g(end) = g0 * cay(-h*e_x) * cay(-h*e_y) * cay(+h*e_x) * cay(+h*e_y)
    let h = r(1, 100);
    let da = &h * &h;
    let step1 = cayley_so3(&vec3_scale(&(-&h), &ex));
    let step2 = cayley_so3(&vec3_scale(&(-&h), &ey));
    let step3 = cayley_so3(&vec3_scale(&h, &ex));
    let step4 = cayley_so3(&vec3_scale(&h, &ey));
    let g_loop = mat3_mul(&mat3_mul(&mat3_mul(&step1, &step2), &step3), &step4);
    // Extract skew part (g_loop - g_loop^T) / 2 ~ hat(holonomy_vec)
    let skew = mat3_scale(&r(1, 2), &mat3_sub(&g_loop, &mat3_transpose(&g_loop)));
    let hol_vec = vee3(&skew);
    let expected_hol = vec3_scale(&da, &vec3_neg(&da_12)); // +da * e_z
    let err_z = (&hol_vec[2] - &expected_hol[2]).abs();
    let loop_ok = hol_vec[2].is_pos() && err_z < &da * &r(1, 1000);

    let ok = ka_ok && loop_ok;
    emit_law(
        "d1_curvature_so3_fixture",
        "Curvature of the connection",
        "frob_conformance::compendium::d1::connection_curvature_so3",
        ok,
        format!("DA_12={da_12:?},loop_z={}", hol_vec[2]),
        format!("DA_12=[0, 0, -1],loop_z~{}", expected_hol[2]),
        None,
        None,
    );
    assert!(ok, "connection curvature KA or SO(3) loop holonomy failed");
}

#[test]
fn d1_nc_curvature() {
    let zero = vec3_zero();
    let ex = vec3(ri(1), ri(0), ri(0));
    let ey = vec3(ri(0), ri(1), ri(0));
    let true_curv = connection_curvature_so3(&zero, &zero, &ex, &ey);
    // Wrong sign: +[A_i, A_j]
    let wrong_curv = vec3_add(&vec3_sub(&zero, &zero), &cross3(&ex, &ey));
    let diff = vec3_sub(&wrong_curv, &true_curv);
    let caught = wrong_curv == vec3_neg(&true_curv) && diff == vec3(ri(0), ri(0), ri(2));
    emit_nc(
        "d1_nc_curvature",
        "Curvature of the connection",
        "frob_conformance::compendium::d1::connection_curvature_so3",
        caught,
        format!("wrong={wrong_curv:?},diff={diff:?}"),
        format!("true={true_curv:?}"),
        None,
        None,
    );
    assert!(caught, "+[A_i, A_j] must be wrong by twice the answer");
}

// =========================================================================
// Row 12: Cayley map
// =========================================================================

#[test]
fn d1_cayley_on_group() {
    let x = sample_twist_a();
    let g = cayley_se3(&x);
    let on_so3 = mat3_is_so3(&g.r);

    // Check R == (I - w_hat/2)^(-1) * (I + w_hat/2)
    let w_hat = hat3(&x.omega);
    let half_w = mat3_scale(&r(1, 2), &w_hat);
    let i_minus = mat3_sub(&mat3_eye(), &half_w);
    let i_plus = mat3_add(&mat3_eye(), &half_w);
    let inv_formula = inv_eye_minus_half_hat(&x.omega);
    let inv_ok = mat3_mul(&i_minus, &inv_formula) == mat3_eye()
        && mat3_mul(&inv_formula, &i_plus) == g.r
        && mat3_inverse_exact(&i_minus) == Some(inv_formula);

    // Inverse property: cay(-x) == cay(x)^(-1)
    let neg_x = Twist::new(vec3_neg(&x.omega), vec3_neg(&x.v));
    let inv_prop = cayley_se3(&neg_x) == g.inverse();

    // KA: omega = (0, 0, 2) gives quarter turn about z
    let q_rot = cayley_so3(&vec3(ri(0), ri(0), ri(2)));
    let expected_q: Mat3 = [
        [ri(0), ri(-1), ri(0)],
        [ri(1), ri(0), ri(0)],
        [ri(0), ri(0), ri(1)],
    ];
    let ka_ok = q_rot == expected_q;

    let ok = on_so3 && inv_ok && inv_prop && ka_ok;
    emit_law(
        "d1_cayley_on_group",
        "Cayley map",
        "frob_conformance::compendium::d1::cayley_se3",
        ok,
        format!("so3={on_so3},inv={inv_ok},inv_prop={inv_prop},ka={ka_ok}"),
        "so3=true,inv=true,inv_prop=true,ka=true".to_owned(),
        None,
        None,
    );
    assert!(ok, "Cayley map group/inverse/KA check failed");
}

#[test]
fn d1_nc_cayley_map() {
    // Naive first-order step R = I + omega_hat leaves SO(3).
    let w = vec3(ri(0), ri(0), ri(2));
    let naive_r = mat3_add(&mat3_eye(), &hat3(&w));
    let caught = !mat3_is_so3(&naive_r);
    emit_nc(
        "d1_nc_cayley_map",
        "Cayley map",
        "frob_conformance::compendium::d1::cayley_se3",
        caught,
        format!("naive_is_so3={}", !caught),
        "naive_is_so3=false".to_owned(),
        None,
        None,
    );
    assert!(caught, "naive I + omega_hat must leave SO(3)");
}

// =========================================================================
// Row 13: Cayley angle (tolerance row: 1e-6 rad)
// =========================================================================

#[test]
fn d1_cayley_angle_bound() {
    let rep = cayley_angle(0.2).expect("0.2 rad is at the design boundary");
    let expected_angle = 2.0 * (0.1_f64).atan();
    let angle_ok = (rep.angle_rad - expected_angle).abs() < 1e-12
        && (rep.angle_rad - 0.199_337).abs() < 1e-6;
    let rel_ok = (rep.rel_error - 0.003_313_475).abs() < 1e-6
        && (rep.approx_rel_error - 0.04 / 12.0).abs() < 1e-12
        && (rep.rel_error - rep.approx_rel_error).abs() < 3e-5;
    let zero_ok = cayley_angle(0.0).is_ok_and(|z| z.angle_rad == 0.0 && z.rel_error == 0.0);
    let refuse_ok = cayley_angle(0.5).is_err();

    let ok = angle_ok && rel_ok && zero_ok && refuse_ok;
    emit_law(
        "d1_cayley_angle_bound",
        "Cayley angle",
        "frob_conformance::compendium::d1::cayley_angle",
        ok,
        format!("{:.9}", rep.angle_rad),
        format!("{expected_angle:.9}"),
        Some("1e-6"),
        Some("rad"),
    );
    assert!(ok, "Cayley angle KA/bound check failed: {rep:?}");
}

#[test]
fn d1_nc_cayley_angle() {
    // NC: step at ||omega||*h = 0.5 rad must be refused, not silently taken.
    let res = cayley_angle(0.5);
    let caught = res.is_err() && cayley_angle(-0.1).is_err() && cayley_angle(f64::NAN).is_err();
    emit_nc(
        "d1_nc_cayley_angle",
        "Cayley angle",
        "frob_conformance::compendium::d1::cayley_angle",
        caught,
        format!("{res:?}"),
        "Err(CayleyStepRefused { observed_rad: 0.5, limit_rad: 0.2 })".to_owned(),
        Some("1e-6"),
        Some("rad"),
    );
    assert!(caught, "step at ||omega||*h = 0.5 rad must be refused");
}

// =========================================================================
// Row 14: Planar Cayley
// =========================================================================

#[test]
fn d1_planar_cayley_modulus() {
    let mut z = UnitComplex::new(r(3, 5), r(4, 5));
    let mut mod_ok = z.norm_sq() == Rat::one();
    for a in [r(1, 10), r(-3, 7), r(2, 5), r(-11, 13), r(1, 2)] {
        z = planar_cayley_step(&z, &a);
        mod_ok &= z.norm_sq() == Rat::one();
    }

    // KA: a = 2 multiplies by i (quarter turn)
    let q = planar_cayley_step(&UnitComplex::new(ri(1), ri(0)), &ri(2));
    let ka_ok = q == UnitComplex::new(ri(0), ri(1))
        && planar_cayley_step(&UnitComplex::new(r(3, 5), r(4, 5)), &ri(2))
            == UnitComplex::new(r(-4, 5), r(3, 5));

    let ok = mod_ok && ka_ok;
    emit_law(
        "d1_planar_cayley_modulus",
        "Planar Cayley",
        "frob_conformance::compendium::d1::planar_cayley_step",
        ok,
        format!("norm_sq={},ka={ka_ok}", z.norm_sq()),
        "norm_sq=1,ka=true".to_owned(),
        None,
        None,
    );
    assert!(ok, "planar Cayley modulus preservation or KA failed");
}

#[test]
fn d1_nc_planar_cayley() {
    // NC: additive update z + i*a*z changes modulus.
    let z = UnitComplex::new(r(3, 5), r(4, 5));
    let a = r(1, 5);
    let z_bad = UnitComplex::new(&z.c - &(&a * &z.s), &z.s + &(&a * &z.c));
    let expected_bad_norm = &Rat::one() + &(&a * &a);
    let caught = z_bad.norm_sq() != Rat::one() && z_bad.norm_sq() == expected_bad_norm;
    emit_nc(
        "d1_nc_planar_cayley",
        "Planar Cayley",
        "frob_conformance::compendium::d1::planar_cayley_step",
        caught,
        format!("{}", z_bad.norm_sq()),
        "1".to_owned(),
        None,
        None,
    );
    assert!(caught, "additive planar update must fail unit modulus");
}

// =========================================================================
// Row 15: Inverse Cayley differential
// =========================================================================

fn left_trivialized_fd_col(x: &Twist, axis: usize, eps: &Rat) -> Vec6 {
    let mut x_plus = x.to_vec6();
    let mut x_minus = x.to_vec6();
    x_plus[axis] = &x_plus[axis] + eps;
    x_minus[axis] = &x_minus[axis] - eps;

    let g0_inv = cayley_se3(x).inverse();
    let g_p = g0_inv.compose(&cayley_se3(&Twist::from_vec6(&x_plus)));
    let g_m = g0_inv.compose(&cayley_se3(&Twist::from_vec6(&x_minus)));
    let two_eps = &ri(2) * eps;

    let d_r = mat3_scale(&(&Rat::one() / &two_eps), &mat3_sub(&g_p.r, &g_m.r));
    let d_w = vee3(&d_r);
    let d_v = vec3_scale(&(&Rat::one() / &two_eps), &vec3_sub(&g_p.p, &g_m.p));
    [
        d_w[0].clone(),
        d_w[1].clone(),
        d_w[2].clone(),
        d_v[0].clone(),
        d_v[1].clone(),
        d_v[2].clone(),
    ]
}

#[test]
fn d1_inverse_cayley_differential() {
    let x = Twist::new(
        vec3(r(1, 3), r(-1, 2), r(2, 5)),
        vec3(r(3, 4), r(-2, 5), r(1, 2)),
    );

    // Exact SO(3) block inverse identity: (I + 1/2 w_hat + 1/4 w w^T) * (2/(4+||w||^2))*(2I - w_hat) = I
    let so3_exact_ok = mat3_mul(&dcay_inv_so3(&x.omega), &dcay_so3(&x.omega)) == mat3_eye()
        && mat3_mul(&dcay_so3(&x.omega), &dcay_inv_so3(&x.omega)) == mat3_eye();

    // SE(3) matrix times central finite-difference left-trivialized differential -> I as O(eps^2)
    let inv = dcay_inv_se3(&x);
    let eps = r(1, 1000);
    let fd_cols: [Vec6; 6] = std::array::from_fn(|k| left_trivialized_fd_col(&x, k, &eps));
    let fd_mat: Mat6 = std::array::from_fn(|i| std::array::from_fn(|j| fd_cols[j][i].clone()));
    let prod = mat6_mul(&inv, &fd_mat);
    let eye = mat6_eye();
    let tol = r(1, 100_000);
    let mut max_err = Rat::zero();
    for i in 0..6 {
        for j in 0..6 {
            let e = (&prod[i][j] - &eye[i][j]).abs();
            if e > max_err {
                max_err = e;
            }
        }
    }
    let fd_ok = max_err < tol;

    let ok = so3_exact_ok && fd_ok;
    emit_law(
        "d1_inverse_cayley_differential",
        "Inverse Cayley differential",
        "frob_conformance::compendium::d1::dcay_inv_se3",
        ok,
        format!("so3_exact={so3_exact_ok},fd_max_err={:.2e}", max_err.to_f64()),
        "so3_exact=true,fd_max_err<1e-5".to_owned(),
        None,
        None,
    );
    assert!(ok, "inverse Cayley differential SO(3) or SE(3) FD identity failed");
}

#[test]
fn d1_nc_inverse_cayley_differential() {
    // NC: right-trivialized inverse differential (I - 1/2 w_hat + 1/4 w w^T) fails
    // when paired with the left-trivialized differential Dcay_left.
    let x = Twist::new(
        vec3(r(1, 3), r(-1, 2), r(2, 5)),
        vec3(r(3, 4), r(-2, 5), r(1, 2)),
    );
    let w_hat = hat3(&x.omega);
    let wwt = outer3(&x.omega, &x.omega);
    let right_so3_inv = mat3_add(
        &mat3_sub(&mat3_eye(), &mat3_scale(&r(1, 2), &w_hat)),
        &mat3_scale(&r(1, 4), &wwt),
    );
    let wrong_prod = mat3_mul(&right_so3_inv, &dcay_so3(&x.omega));
    let caught = wrong_prod != mat3_eye();
    emit_nc(
        "d1_nc_inverse_cayley_differential",
        "Inverse Cayley differential",
        "frob_conformance::compendium::d1::dcay_inv_se3",
        caught,
        format!("right_inv_times_left_is_I={}", !caught),
        "right_inv_times_left_is_I=false".to_owned(),
        None,
        None,
    );
    assert!(caught, "right-trivialized inverse must fail left-trivialized identity");
}

// =========================================================================
// Row 16: Pseudo-inertia
// =========================================================================

#[test]
fn d1_pseudo_inertia_round_trip() {
    let i_bar: Mat3 = [
        [r(7, 2), r(-1, 4), r(1, 5)],
        [r(-1, 4), r(9, 2), r(1, 3)],
        [r(1, 5), r(1, 3), r(5, 1)],
    ];
    let sigma = sigma_from_rotational_inertia(&i_bar);
    let recovered = rotational_inertia_from_sigma(&sigma);
    let rt_ok = recovered == i_bar;

    // Consistent body: solid box of mass 2, com (1/4, -1/5, 1/3)
    let m = ri(2);
    let c = vec3(r(1, 4), r(-1, 5), r(1, 3));
    let i_c = mat3_diag(&[ri(2), ri(3), ri(4)]);
    // Parallel-axis shift to origin: I_bar = I_C + m * (||c||^2 I - c c^T)
    let shift = mat3_scale(&m, &mat3_sub(&mat3_scale(&dot3(&c, &c), &mat3_eye()), &outer3(&c, &c)));
    let i_origin = mat3_add(&i_c, &shift);
    let p = pseudo_inertia_from_rotational(&m, &c, &i_origin);
    let spd_ok = pseudo_inertia_is_consistent(&p);

    let ok = rt_ok && spd_ok;
    emit_law(
        "d1_pseudo_inertia_round_trip",
        "Pseudo-inertia",
        "frob_conformance::compendium::d1::pseudo_inertia_from_rotational",
        ok,
        format!("round_trip={rt_ok},spd={spd_ok}"),
        "round_trip=true,spd=true".to_owned(),
        None,
        None,
    );
    assert!(ok, "pseudo-inertia round trip or SPD check failed");
}

#[test]
fn d1_nc_pseudo_inertia() {
    // NC: principal moments (1, 1, 3) give Sigma = diag(3/2, 3/2, -1/2), not positive definite.
    let i_bad = mat3_diag_int(1, 1, 3);
    let sigma = sigma_from_rotational_inertia(&i_bad);
    let p_bad = pseudo_inertia_from_rotational(&ri(1), &vec3_zero(), &i_bad);
    let expected_sigma = mat3_diag(&[r(3, 2), r(3, 2), r(-1, 2)]);
    let caught = sigma == expected_sigma && !pseudo_inertia_is_consistent(&p_bad);
    emit_nc(
        "d1_nc_pseudo_inertia",
        "Pseudo-inertia",
        "frob_conformance::compendium::d1::pseudo_inertia_from_rotational",
        caught,
        format!("sigma_22={},spd={}", sigma[2][2], pseudo_inertia_is_consistent(&p_bad)),
        "sigma_22=-1/2,spd=false".to_owned(),
        None,
        None,
    );
    assert!(caught, "unphysical moments (1,1,3) must fail P > 0");
}

// =========================================================================
// Row 17: Shell rule
// =========================================================================

#[test]
fn d1_shell_limit_two_thirds() {
    let rad = ri(1);
    let solid_ratio = spherical_shell_inertia_ratio(&rad, &rad);
    let thin_ratio = spherical_shell_inertia_ratio(&rad, &r(1, 10_000));
    let two_thirds = r(2, 3);
    let limit_ok = solid_ratio == r(2, 5)
        && thin_ratio < two_thirds
        && (&two_thirds - &thin_ratio) < r(1, 1000);

    // KA: aluminium box 0.4 x 0.3 x 0.2 m, rho = 2700 kg/m^3, wall t = 2.5 mm = 1/400 m
    let lx = r(2, 5);
    let ly = r(3, 10);
    let lz = r(1, 5);
    let rho = ri(2700);
    let wall_t = r(1, 400);
    let p_solid = solid_box_pseudo_inertia(&lx, &ly, &lz, &rho);
    let p_shell = box_shell_pseudo_inertia(&lx, &ly, &lz, &wall_t, &rho);
    let m_solid = &p_solid[3][3];
    let m_shell = &p_shell[3][3];
    let ratio = m_solid / m_shell;
    let ka_ok = *m_solid == r(324, 5)
        && *m_shell == r(275_967, 80_000)
        && ratio > r(1878, 100)
        && ratio < r(1879, 100)
        && pseudo_inertia_is_consistent(&p_shell);

    let ok = limit_ok && ka_ok;
    emit_law(
        "d1_shell_limit_two_thirds",
        "Shell rule",
        "frob_conformance::compendium::d1::spherical_shell_inertia_ratio",
        ok,
        format!("solid={solid_ratio},thin={:.6},box_shell_kg={:.4}", thin_ratio.to_f64(), m_shell.to_f64()),
        "solid=2/5,thin->2/3,box_shell_kg=3.4496".to_owned(),
        None,
        Some("kg"),
    );
    assert!(ok, "shell rule limit or aluminium box KA failed");
}

#[test]
fn d1_nc_shell_rule() {
    // NC: solid default for the 2.5 mm enclosure overstates mass by ~18.8x.
    let rho = ri(2700);
    let p_solid = solid_box_pseudo_inertia(&r(2, 5), &r(3, 10), &r(1, 5), &rho);
    let p_shell = box_shell_pseudo_inertia(&r(2, 5), &r(3, 10), &r(1, 5), &r(1, 400), &rho);
    let ratio = &p_solid[3][3] / &p_shell[3][3];
    let caught = p_solid[3][3] != p_shell[3][3] && ratio > ri(18);
    emit_nc(
        "d1_nc_shell_rule",
        "Shell rule",
        "frob_conformance::compendium::d1::spherical_shell_inertia_ratio",
        caught,
        format!("solid_kg={},shell_kg={},ratio={:.2}", p_solid[3][3], p_shell[3][3], ratio.to_f64()),
        "ratio>18".to_owned(),
        None,
        Some("kg"),
    );
    assert!(caught, "solid default must disagree dramatically with shell");
}

// =========================================================================
// Row 18: Mass-support bound
// =========================================================================

#[test]
fn d1_mass_support_fixture() {
    let origin = vec3_zero();
    let m = ri(1);
    let rho = r(1, 10);
    let rho_sq = &rho * &rho; // 1/100 m^2

    // KA part 1: principal moments 0.008 kg*m^2 -> lhs = 0.012 m^2 > 0.010 m^2 (fails)
    let i_fail = mat3_diag(&[r(8, 1000), r(8, 1000), r(8, 1000)]);
    let lhs_fail = mass_support_lhs(&origin, &origin, &m, &i_fail);
    let fail_ok = lhs_fail == r(12, 1000) && !mass_support_holds(&origin, &origin, &m, &i_fail, &rho);

    // KA part 2: thin spherical shell with moments 1/150 kg*m^2 -> lhs = 1/100 == rho^2 (equality)
    let i_shell = mat3_diag(&[r(1, 150), r(1, 150), r(1, 150)]);
    let lhs_shell = mass_support_lhs(&origin, &origin, &m, &i_shell);
    let eq_ok = lhs_shell == rho_sq && mass_support_holds(&origin, &origin, &m, &i_shell, &rho);

    let ok = fail_ok && eq_ok;
    emit_law(
        "d1_mass_support_fixture",
        "Mass-support bound",
        "frob_conformance::compendium::d1::mass_support_lhs",
        ok,
        format!("fail_lhs={lhs_fail},shell_lhs={lhs_shell}"),
        format!("fail_lhs=3/250,shell_lhs={rho_sq}"),
        None,
        Some("m^2"),
    );
    assert!(ok, "mass-support bound KA failed");
}

#[test]
fn d1_nc_mass_support_bound() {
    // NC: checking only ||c - o||^2 <= rho^2 (dropping tr(I_C)/(2m)) falsely accepts the impossible part.
    let origin = vec3_zero();
    let m = ri(1);
    let rho = r(1, 10);
    let i_impossible = mat3_diag(&[r(8, 1000), r(8, 1000), r(8, 1000)]);
    let com_only_passes = dot3(&origin, &origin) <= &rho * &rho;
    let true_bound_holds = mass_support_holds(&origin, &origin, &m, &i_impossible, &rho);
    let caught = com_only_passes && !true_bound_holds;
    emit_nc(
        "d1_nc_mass_support_bound",
        "Mass-support bound",
        "frob_conformance::compendium::d1::mass_support_lhs",
        caught,
        format!("com_only={com_only_passes},true_bound={true_bound_holds}"),
        "com_only=true,true_bound=false".to_owned(),
        None,
        Some("m^2"),
    );
    assert!(caught, "true mass-support bound must reject impossible inertia");
}

// =========================================================================
// Row 19: Mobility count
// =========================================================================

#[test]
fn d1_mobility_count_sarrus_bennett() {
    let sarrus = mobility_count(6, &[1, 1, 1, 1, 1, 1]);
    let bennett = mobility_count(4, &[1, 1, 1, 1]);
    let open_6r = mobility_count(7, &[1, 1, 1, 1, 1, 1]);

    let ok = sarrus.count == 0
        && sarrus.is_count_not_mobility
        && bennett.count == -2
        && bennett.is_count_not_mobility
        && open_6r.count == 6
        && open_6r.is_count_not_mobility;

    emit_law(
        "d1_mobility_count_sarrus_bennett",
        "Mobility count",
        "frob_conformance::compendium::d1::mobility_count",
        ok,
        format!("sarrus={},bennett={},6r={}", sarrus.count, bennett.count, open_6r.count),
        "sarrus=0,bennett=-2,6r=6".to_owned(),
        None,
        None,
    );
    assert!(ok, "Grubler-Kutzbach count KA failed");
}

#[test]
fn d1_nc_mobility_count() {
    // NC: naive rule "count <= 0 implies immobile (mobility = 0)" fails on Sarrus and Bennett (both have mobility 1).
    let sarrus = mobility_count(6, &[1, 1, 1, 1, 1, 1]);
    let bennett = mobility_count(4, &[1, 1, 1, 1]);
    let true_mobility = 1_i64;
    let naive_sarrus = sarrus.count.max(0);
    let naive_bennett = bennett.count.max(0);
    let caught = naive_sarrus != true_mobility && naive_bennett != true_mobility && sarrus.is_count_not_mobility;
    emit_nc(
        "d1_nc_mobility_count",
        "Mobility count",
        "frob_conformance::compendium::d1::mobility_count",
        caught,
        format!("naive_sarrus={naive_sarrus},naive_bennett={naive_bennett}"),
        format!("true_mobility={true_mobility}"),
        None,
        None,
    );
    assert!(caught, "over-constrained linkages must not be conflated with count");
}

// =========================================================================
// Row 20: Product of exponentials
// =========================================================================

#[test]
fn d1_poe_planar_2r() {
    let screws = [
        RevoluteScrew::from_axis_point(vec3(ri(0), ri(0), ri(1)), &vec3_zero()),
        RevoluteScrew::from_axis_point(vec3(ri(0), ri(0), ri(1)), &vec3(r(2, 5), ri(0), ri(0))),
    ];
    let cs = [(ri(1), ri(0)), (ri(0), ri(1))];
    let home = Pose::new(mat3_eye(), vec3(r(7, 10), ri(0), ri(0)));

    let pose = poe_forward_kinematics(&screws, &cs, &home);
    let expected_p = vec3(r(2, 5), r(3, 10), ri(0));
    let expected_r: Mat3 = [
        [ri(0), ri(-1), ri(0)],
        [ri(1), ri(0), ri(0)],
        [ri(0), ri(0), ri(1)],
    ];
    let fk_ok = pose.p == expected_p && pose.r == expected_r && mat3_is_so3(&pose.r);

    let j_space = poe_space_jacobian(&screws, &cs);
    let j_p = space_jacobian_to_point_velocity(&j_space, &pose.p);
    // Point Jacobian: [[-0.3, -0.3], [0.4, 0.0]] m/rad
    let jac_ok = j_p[0] == vec3(r(-3, 10), r(2, 5), ri(0))
        && j_p[1] == vec3(r(-3, 10), ri(0), ri(0));

    let f_tool = vec3(ri(10), ri(0), ri(0));
    let tau = [dot3(&j_p[0], &f_tool), dot3(&j_p[1], &f_tool)];
    let tau_ok = tau == [ri(-3), ri(-3)];

    let ok = fk_ok && jac_ok && tau_ok;
    emit_law(
        "d1_poe_planar_2r",
        "Product of exponentials",
        "frob_conformance::compendium::d1::poe_forward_kinematics",
        ok,
        format!("p={:?},tau={tau:?}", pose.p),
        "p=[2/5, 3/10, 0],tau=[-3, -3]".to_owned(),
        None,
        Some("m"),
    );
    assert!(ok, "PoE planar 2R FK / point Jacobian / torque KA failed");
}

#[test]
fn d1_nc_product_of_exponentials() {
    // NC: lower three rows of the space Jacobian used as point Jacobian give torques (0, 0).
    let screws = [
        RevoluteScrew::from_axis_point(vec3(ri(0), ri(0), ri(1)), &vec3_zero()),
        RevoluteScrew::from_axis_point(vec3(ri(0), ri(0), ri(1)), &vec3(r(2, 5), ri(0), ri(0))),
    ];
    let cs = [(ri(1), ri(0)), (ri(0), ri(1))];
    let j_space = poe_space_jacobian(&screws, &cs);
    let f_tool = vec3(ri(10), ri(0), ri(0));
    let wrong_tau = [dot3(&j_space[0].v, &f_tool), dot3(&j_space[1].v, &f_tool)];
    let caught = wrong_tau == [ri(0), ri(0)] && wrong_tau != [ri(-3), ri(-3)];
    emit_nc(
        "d1_nc_product_of_exponentials",
        "Product of exponentials",
        "frob_conformance::compendium::d1::poe_forward_kinematics",
        caught,
        format!("wrong_tau={wrong_tau:?}"),
        "true_tau=[-3, -3]".to_owned(),
        None,
        Some("N*m"),
    );
    assert!(caught, "space Jacobian lower rows must fail torque KA");
}

// =========================================================================
// Row 21: Constant-twist chord (tolerance row: 1e-12 m)
// =========================================================================

#[test]
fn d1_chord_half_turn() {
    // KA: unit-speed half turn in 1 s gives chord (0, 2/pi) m.
    let (dx, dy) = constant_twist_chord(1.0, std::f64::consts::PI, 1.0, 0.0);
    let exp_y = 2.0 / std::f64::consts::PI;
    let err_x = dx.abs();
    let err_y = (dy - exp_y).abs();
    let (sx, sy) = constant_twist_chord(2.5, 0.0, 0.4, 0.0);
    let straight_ok = (sx - 1.0).abs() < 1e-15 && sy.abs() < 1e-15;

    let ok = err_x < 1e-12 && err_y < 1e-12 && straight_ok;
    emit_law(
        "d1_chord_half_turn",
        "Constant-twist chord",
        "frob_conformance::compendium::d1::constant_twist_chord",
        ok,
        format!("({dx:.15},{dy:.15})"),
        format!("(0.0,{exp_y:.15})"),
        Some("1e-12"),
        Some("m"),
    );
    assert!(ok, "constant-twist half-turn KA failed: ({dx}, {dy}) vs (0, {exp_y})");
}

#[test]
fn d1_chord_near_straight() {
    // Heading 32 rad, wheel speeds 1 and 1 + 6e-13 m/s on a 0.5 m track, 400 steps of 1/400 s.
    let v_l = 1.0_f64;
    let v_r = 1.0_f64 + 6.0e-13;
    let track = 0.5_f64;
    let v = 0.5 * (v_l + v_r);
    let omega = (v_r - v_l) / track;
    let dt = 1.0_f64 / 400.0;
    let mut theta = 32.0_f64;
    let (mut px, mut py) = (0.0_f64, 0.0_f64);
    for _ in 0..400 {
        let (dx, dy) = constant_twist_chord(v, omega, dt, theta);
        px += dx;
        py += dy;
        theta += omega * dt;
    }
    let exp_x = 32.0_f64.cos(); // ~0.8342233605
    let exp_y = 32.0_f64.sin(); // ~0.5514266812
    let err = ((px - exp_x).powi(2) + (py - exp_y).powi(2)).sqrt();
    let ok = err < 1e-11 && (px - 0.8342).abs() < 1e-4 && (py - 0.5514).abs() < 1e-4;
    emit_law(
        "d1_chord_near_straight",
        "Constant-twist chord",
        "frob_conformance::compendium::d1::constant_twist_chord",
        ok,
        format!("({px:.12},{py:.12})"),
        format!("({exp_x:.12},{exp_y:.12})"),
        Some("1e-12"),
        Some("m"),
    );
    assert!(ok, "near-straight chord failed: ({px}, {py}) vs ({exp_x}, {exp_y}), err={err}");
}

#[test]
fn d1_nc_constant_twist_chord() {
    // NC: naive divide-by-omega formula suffers complete cancellation at theta = 32 rad,
    // delta = 3e-15 < ulp(32)/2, returning (0, 0) instead of ~(0.8342, 0.5514) m.
    let v_l = 1.0_f64;
    let v_r = 1.0_f64 + 6.0e-13;
    let track = 0.5_f64;
    let v = 0.5 * (v_l + v_r);
    let omega = (v_r - v_l) / track;
    let dt = 1.0_f64 / 400.0;
    let mut theta = 32.0_f64;
    let (mut naive_x, mut naive_y) = (0.0_f64, 0.0_f64);
    for _ in 0..400 {
        let th_next = theta + omega * dt;
        naive_x += (v / omega) * (th_next.sin() - theta.sin());
        naive_y += -(v / omega) * (th_next.cos() - theta.cos());
        theta = th_next;
    }
    let exp_x = 32.0_f64.cos();
    let exp_y = 32.0_f64.sin();
    let naive_err = ((naive_x - exp_x).powi(2) + (naive_y - exp_y).powi(2)).sqrt();
    let caught = naive_x == 0.0 && naive_y == 0.0 && naive_err > 0.9;
    emit_nc(
        "d1_nc_constant_twist_chord",
        "Constant-twist chord",
        "frob_conformance::compendium::d1::constant_twist_chord",
        caught,
        format!("naive=({naive_x},{naive_y}),err={naive_err:.4}"),
        format!("limit=({exp_x:.4},{exp_y:.4})"),
        Some("1e-12"),
        Some("m"),
    );
    assert!(caught, "naive divide-by-omega formula must fail near-straight chord");
}
