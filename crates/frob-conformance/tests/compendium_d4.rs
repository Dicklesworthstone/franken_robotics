#![deny(unsafe_code)]

//! Appendix D.4 mathematical law tests, known answers, dimensional homogeneity checks,
//! and negative controls (`N12-44`: actuators and power).
//!
//! Every one of the 14 rows in `registries/compendium/d4.toml` has at least one law/KA test
//! (`d4_*`) and an explicit negative-control test that verifies the wrong formula fails.
//! Every test emits a `frob.testevidence/1` record when `FROB_TEST_EVIDENCE_DIR` is set.

use frob_conformance::compendium::d4::{
    BusOperatingPoint, D4_ORACLE_SIGNATURES, Dim, DimError, DimQty, EnergyDomain,
    EnergyDomainMismatch, EnergyLedgerStep, FieldWeakeningParams, MotorAxisParams,
    MotorEnvelopeParams, MotorThermalState, PhsSystem, ScrewBackdrive, ThermalNodeParams,
    ThermalRegime, TopSpeed, TwoRcBatteryParams, battery_soc_exact, bus_operating_point,
    bus_power_ceiling_exact, compose_rational_gears, copper_loss, cost_of_transport,
    cost_of_transport_checked, energy_balance_residual, entropy_production,
    field_weakening_boundary_currents, froude_number, is_skew_symmetric, is_symmetric_psd,
    motor_axis_as_phs, motor_axis_derivatives, motor_axis_locked_rotor_current, motor_envelope_l1,
    motor_thermal_exact_step, phs_step_derivatives, rational_gear_map, screw_efficiencies,
    thermal_node_closed_form, thermal_node_rhs, thermal_runaway_analysis,
    thermal_runaway_torque_control_tangency, transmission_backdriven_torque,
    transmission_motoring_torque, two_rc_battery_step_response, verify_all_oracle_dimensions,
    voltage_ellipse_lhs_rhs, voltage_limit_field_weakening,
};
use frob_conformance::compendium::rat::{Rat, r, ri};
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
        format!("D.4:{row_name}"),
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
        format!("D.4:{row_name}"),
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

/// Deterministic SplitMix64 generator for seeded rational fixtures (D0, no external RNG).
struct SplitMix64(u64);

impl SplitMix64 {
    fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    fn rat_small(&mut self) -> Rat {
        let num = (self.next_u64() % 11) as i64 - 5;
        let den = (self.next_u64() % 4) as i64 + 1;
        r(num, den)
    }
}

fn rk4_scalar(mut y: f64, t_end: f64, steps: usize, f: impl Fn(f64) -> f64) -> f64 {
    let h = t_end / (steps as f64);
    for _ in 0..steps {
        let k1 = f(y);
        let k2 = f(y + 0.5 * h * k1);
        let k3 = f(y + 0.5 * h * k2);
        let k4 = f(y + h * k3);
        y += (h / 6.0) * (k1 + 2.0 * k2 + 2.0 * k3 + k4);
    }
    y
}

// =========================================================================
// Row 1: Port-Hamiltonian form
// =========================================================================

#[test]
fn d4_phs_power_balance() {
    let mut rng = SplitMix64(0xD4_0001);
    let mut all_ok = true;

    for n in [2usize, 3, 4] {
        for _case in 0..8 {
            // Random skew-symmetric J
            let mut j = vec![vec![Rat::zero(); n]; n];
            for row in 0..n {
                for col in (row + 1)..n {
                    let v = rng.rat_small();
                    j[row][col] = v.clone();
                    j[col][row] = -&v;
                }
            }
            // Random PSD R = B^T B (always symmetric PSD)
            let mut b = vec![vec![Rat::zero(); n]; n];
            for r_row in &mut b {
                for entry in r_row.iter_mut() {
                    *entry = rng.rat_small();
                }
            }
            let mut r_mat = vec![vec![Rat::zero(); n]; n];
            for row in 0..n {
                for col in 0..n {
                    let mut sum = Rat::zero();
                    for k in 0..n {
                        sum = &sum + &(&b[k][row] * &b[k][col]);
                    }
                    r_mat[row][col] = sum;
                }
            }
            // Random symmetric positive-definite Q = diag(1..n) + S^T S
            let mut q = vec![vec![Rat::zero(); n]; n];
            for i in 0..n {
                q[i][i] = ri((i as i64) + 1);
            }
            for row in 0..n {
                for col in (row + 1)..n {
                    let v = r((rng.next_u64() % 3) as i64, 4);
                    q[row][col] = v.clone();
                    q[col][row] = v;
                }
            }
            // Port matrix G (n x 2), state x, input u
            let m = 2usize;
            let mut g = vec![vec![Rat::zero(); m]; n];
            for g_row in &mut g {
                for entry in g_row.iter_mut() {
                    *entry = rng.rat_small();
                }
            }
            let x: Vec<Rat> = (0..n).map(|_| rng.rat_small()).collect();
            let u: Vec<Rat> = (0..m).map(|_| rng.rat_small()).collect();

            let sys = PhsSystem { j, r: r_mat, g, q };
            let ev = phs_step_derivatives(&sys, &x, &u);
            let ok = is_skew_symmetric(&sys.j)
                && is_symmetric_psd(&sys.r)
                && ev.power_discrepancy.is_zero()
                && ev.h_dot_direct == ev.h_dot_balance
                && !ev.dissipation_power.is_neg();
            if !ok {
                all_ok = false;
            }
        }
    }

    emit_law(
        "d4_phs_power_balance",
        "Port-Hamiltonian form",
        "frob_conformance::compendium::d4::phs_step_derivatives",
        all_ok,
        format!("all_24_exact_cases_ok={all_ok}"),
        "H_dot == u^T y - grad_H^T R grad_H and dissipation >= 0".to_owned(),
        None,
        Some("W"),
    );
    assert!(all_ok, "PHS exact rational power balance failed");
}

#[test]
fn d4_nc_phs_symmetric_j() {
    // Planted defect: add a positive symmetric part S = diag(1, 0) to skew J.
    let sys_bad = PhsSystem {
        j: vec![vec![ri(1), ri(-2)], vec![ri(2), ri(0)]],
        r: vec![vec![ri(0), ri(0)], vec![ri(0), ri(0)]],
        g: vec![vec![ri(1)], vec![ri(0)]],
        q: vec![vec![ri(1), ri(0)], vec![ri(0), ri(1)]],
    };
    let x = vec![ri(3), ri(2)];
    let u = vec![ri(0)];
    let ev = phs_step_derivatives(&sys_bad, &x, &u);
    let caught = !is_skew_symmetric(&sys_bad.j) && ev.power_discrepancy == ri(9) && ev.h_dot_direct.is_pos();

    emit_nc(
        "d4_nc_phs_symmetric_j",
        "Port-Hamiltonian form",
        "frob_conformance::compendium::d4::phs_step_derivatives",
        caught,
        format!("skew={}, discrepancy={} W", is_skew_symmetric(&sys_bad.j), ev.power_discrepancy),
        "non-skew J creates positive energy with u=0, R=0".to_owned(),
        None,
        Some("W"),
    );
    assert!(caught, "symmetric part in J must create energy and fail skew check");
}

// =========================================================================
// Row 2: Motor axis
// =========================================================================

#[test]
fn d4_motor_locked_rotor_tau_10ms() {
    // Exact rational KA: R_w = 1 Ω, L = 10 mH = 1/100 H => tau = 10 ms = 1/100 s.
    let params = MotorAxisParams {
        l: r(1, 100),
        r_w: ri(1),
        k_t: r(1, 10),
        k_e: r(1, 10),
        j_r: r(1, 1000),
        b: r(1, 1000),
    };
    let deriv = motor_axis_derivatives(&params, &Rat::zero(), &Rat::zero(), &ri(12), &Rat::zero());
    let tau_exact_ok = deriv.tau_electrical == r(1, 100) && deriv.di_dt == ri(1200);

    // Locked-rotor closed form vs RK4 integration at t = tau = 0.01 s and t = 0.025 s.
    let v = 12.0_f64;
    let r_w = 1.0_f64;
    let l = 0.01_f64;
    let (i_at_tau, tau_f) = motor_axis_locked_rotor_current(v, r_w, l, 0.01);
    let expected_at_tau = (v / r_w) * (1.0 - (-1.0_f64).exp());
    let rk4_at_025 = rk4_scalar(0.0, 0.025, 2_000, |i| (v - r_w * i) / l);
    let (i_at_025, _) = motor_axis_locked_rotor_current(v, r_w, l, 0.025);

    let tol = 1e-12_f64;
    let ok = tau_exact_ok
        && (tau_f - 0.01).abs() <= 1e-15
        && (i_at_tau - expected_at_tau).abs() <= tol
        && (i_at_025 - rk4_at_025).abs() <= tol;

    emit_law(
        "d4_motor_locked_rotor_tau_10ms",
        "Motor axis",
        "frob_conformance::compendium::d4::motor_axis_locked_rotor_current",
        ok,
        format!("tau_exact={}, i(tau)={i_at_tau:.12} A, err_rk4={:.3e} A", deriv.tau_electrical, (i_at_025 - rk4_at_025).abs()),
        format!("tau=1/100 s (10 ms), i(tau)={expected_at_tau:.12} A"),
        Some("1e-12 A"),
        Some("A"),
    );
    assert!(ok, "locked-rotor tau=10ms / step response failed");
}

#[test]
fn d4_motor_is_phs_with_gyrator() {
    let params = MotorAxisParams {
        l: r(1, 100),
        r_w: r(3, 5),
        k_t: r(7, 50),
        k_e: r(7, 50),
        j_r: r(1, 500),
        b: r(1, 200),
    };
    let i = r(5, 2);
    let omega = ri(40);
    let v = ri(24);
    let tau_load = r(1, 5);

    let deriv = motor_axis_derivatives(&params, &i, &omega, &v, &tau_load);
    let phs = motor_axis_as_phs(&params);
    let phi = &params.l * &i;
    let p_ang = &params.j_r * &omega;
    let ev = phs_step_derivatives(&phs, &[phi, p_ang], &[v.clone(), -&tau_load]);

    let expected_dphi = &params.l * &deriv.di_dt;
    let expected_dp = &params.j_r * &deriv.domega_dt;
    let expected_diss = &(&params.r_w * &(&i * &i)) + &(&params.b * &(&omega * &omega));
    let expected_supply = &(&v * &i) - &(&tau_load * &omega);

    let ok = is_skew_symmetric(&phs.j)
        && is_symmetric_psd(&phs.r)
        && ev.x_dot == vec![expected_dphi, expected_dp]
        && ev.grad_h == vec![i, omega]
        && ev.dissipation_power == expected_diss
        && ev.supply_power == expected_supply
        && ev.power_discrepancy.is_zero();

    emit_law(
        "d4_motor_is_phs_with_gyrator",
        "Motor axis",
        "frob_conformance::compendium::d4::motor_axis_locked_rotor_current",
        ok,
        format!("discrepancy={}, diss={} W", ev.power_discrepancy, ev.dissipation_power),
        "exact PHS match with skew gyrator and dissipation R_w*i^2 + b*omega^2".to_owned(),
        None,
        Some("W"),
    );
    assert!(ok, "motor axis PHS equivalence failed");
}

#[test]
fn d4_nc_motor_kt_ne_ke() {
    // Planted defect: k_t != k_e breaks gyrator skew-symmetry and creates power (k_t - k_e)*i*omega.
    let bad = MotorAxisParams {
        l: r(1, 100),
        r_w: r(1, 2),
        k_t: r(1, 5),
        k_e: r(1, 10),
        j_r: r(1, 1000),
        b: r(1, 1000),
    };
    let i = ri(4);
    let omega = ri(30);
    let phs = motor_axis_as_phs(&bad);
    let ev = phs_step_derivatives(
        &phs,
        &[&bad.l * &i, &bad.j_r * &omega],
        &[ri(12), ri(0)],
    );
    let expected_spurious = &(&(&bad.k_t - &bad.k_e) * &i) * &omega;
    let caught = !is_skew_symmetric(&phs.j)
        && ev.power_discrepancy == expected_spurious
        && !ev.power_discrepancy.is_zero();

    emit_nc(
        "d4_nc_motor_kt_ne_ke",
        "Motor axis",
        "frob_conformance::compendium::d4::motor_axis_locked_rotor_current",
        caught,
        format!("spurious_power={} W (expected {} W)", ev.power_discrepancy, expected_spurious),
        "k_t != k_e makes gyrator create/lose power (k_t - k_e)*i*omega != 0".to_owned(),
        None,
        Some("W"),
    );
    assert!(caught, "k_t != k_e must create/destroy spurious power");
}

// =========================================================================
// Row 3: Motor envelope (compendium-only)
// =========================================================================

#[test]
fn d4_envelope_no_load_speed_and_stall() {
    let params = MotorEnvelopeParams {
        i_max: ri(30),
        v_bus: ri(48),
        k_e: r(1, 10),
        k_t: r(1, 10),
        gear_ratio_n: ri(8),
        resistance: r(1, 2),
        eta: r(9, 10),
        tau_friction: r(1, 4),
    };
    // Stall at omega = 0: V_bus/R = 96 A > I_max = 30 A => i = 30 A.
    let stall = motor_envelope_l1(&params, &Rat::zero());
    let expected_omega_nl = ri(60); // 48 / (0.1 * 8) = 60 rad/s
    let expected_tau_stall = &(&(&params.eta * &params.gear_ratio_n) * &(&params.k_t * &params.i_max))
        - &params.tau_friction;

    // No-load speed omega = V_bus / (k_e * N) = 60 rad/s => voltage-limited current == 0.
    let nl = motor_envelope_l1(&params, &expected_omega_nl);

    // Mid-high speed (50 rad/s): V_bus - k_e*N*omega = 48 - 40 = 8 V => i_v = 16 A < 30 A.
    let high = motor_envelope_l1(&params, &ri(50));

    let ok = stall.omega_no_load == expected_omega_nl
        && stall.i_limit == ri(30)
        && stall.tau_out == expected_tau_stall
        && nl.i_voltage_limit.is_zero()
        && nl.i_limit.is_zero()
        && high.i_limit == ri(16);

    emit_law(
        "d4_envelope_no_load_speed_and_stall",
        "Motor envelope",
        "frob_conformance::compendium::d4::motor_envelope_l1",
        ok,
        format!("omega_nl={} rad/s, tau_stall={} N*m, i(omega_nl)={} A", stall.omega_no_load, stall.tau_out, nl.i_limit),
        format!("omega_nl=60 rad/s, tau_stall={expected_tau_stall} N*m, i(omega_nl)=0 A"),
        None,
        Some("N·m"),
    );
    assert!(ok, "L1 motor envelope stall and no-load checks failed");
}

#[test]
fn d4_nc_envelope_no_back_emf() {
    let params = MotorEnvelopeParams {
        i_max: ri(30),
        v_bus: ri(48),
        k_e: r(1, 10),
        k_t: r(1, 10),
        gear_ratio_n: ri(8),
        resistance: r(1, 2),
        eta: r(9, 10),
        tau_friction: r(1, 4),
    };
    let overspeed = ri(120); // 2 * omega_no_load
    let real_eval = motor_envelope_l1(&params, &overspeed);

    // Wrong model: omit back-EMF => i_bad = min(I_max, V_bus/R) = 30 A at any speed.
    let v_over_r = &params.v_bus / &params.resistance;
    let i_bad = if v_over_r < params.i_max { v_over_r } else { params.i_max.clone() };
    let tau_bad = &(&(&params.eta * &params.gear_ratio_n) * &(&params.k_t * &i_bad))
        - &params.tau_friction;

    let caught = real_eval.i_limit.is_neg() && real_eval.tau_out.is_neg() && tau_bad.is_pos();
    emit_nc(
        "d4_nc_envelope_no_back_emf",
        "Motor envelope",
        "frob_conformance::compendium::d4::motor_envelope_l1",
        caught,
        format!("real_tau(2*w_nl)={} N*m vs no_bemf_tau={} N*m", real_eval.tau_out, tau_bad),
        "omitting back-EMF yields positive torque above no-load speed".to_owned(),
        None,
        Some("N·m"),
    );
    assert!(caught, "no-back-EMF mutant must erroneously produce positive torque at overspeed");
}

// =========================================================================
// Row 4: Voltage limit and field weakening (compendium-only)
// =========================================================================

#[test]
fn d4_voltage_circle_base_and_top_speed() {
    // Plan §15.8 worked card: lambda = 0.010 V*s, L = 100 uH, I_max = 40 A, V_bus = 48 V.
    let p = FieldWeakeningParams {
        v_bus: 48.0,
        lambda: 0.010,
        l_d: 100e-6,
        l_q: 100e-6,
        i_max: 40.0,
    };
    let ev = voltage_limit_field_weakening(&p);
    let TopSpeed::Finite(omega_top) = ev.omega_top else {
        panic!("expected finite top speed since lambda=0.010 > L*I_max=0.004");
    };

    // At base speed with (i_d = 0, i_q = I_max), voltage limit is met with equality.
    let (base_lhs, base_rhs) = voltage_ellipse_lhs_rhs(&p, 0.0, p.i_max, ev.omega_base);
    // At top speed with (i_d = -I_max, i_q = 0), voltage limit is met with equality.
    let (top_lhs, top_rhs) = voltage_ellipse_lhs_rhs(&p, -p.i_max, 0.0, omega_top);

    // At 1.5 * base speed, boundary currents satisfy BOTH current circle and voltage ellipse.
    let w_mid = 1.5 * ev.omega_base;
    let (id_mid, iq_mid) = field_weakening_boundary_currents(&p, w_mid).expect("feasible at 1.5*w_base");
    let (mid_lhs, mid_rhs) = voltage_ellipse_lhs_rhs(&p, id_mid, iq_mid, w_mid);
    let cur_err = ((id_mid * id_mid + iq_mid * iq_mid).sqrt() - p.i_max).abs();

    let ok = (base_lhs - base_rhs).abs() <= 1e-15
        && (top_lhs - top_rhs).abs() <= 1e-15
        && (mid_lhs - mid_rhs).abs() <= 1e-15
        && cur_err <= 1e-12
        && (ev.v_lim - 48.0 / 2.0_f64.sqrt()).abs() <= 1e-12
        && (ev.omega_base - 3151.351_110_639_467).abs() <= 1e-6
        && (omega_top - 5656.854_249_492_38).abs() <= 1e-6
        && (iq_mid / p.i_max - 0.592).abs() < 0.01;

    emit_law(
        "d4_voltage_circle_base_and_top_speed",
        "Voltage limit and field weakening",
        "frob_conformance::compendium::d4::voltage_limit_field_weakening",
        ok,
        format!("w_base={:.3} rad/s, w_top={:.3} rad/s, base_diff={:.2e}, top_diff={:.2e}", ev.omega_base, omega_top, (base_lhs - base_rhs).abs(), (top_lhs - top_rhs).abs()),
        "equality at (0, I_max) for base speed and (-I_max, 0) for top speed".to_owned(),
        Some("1e-9 rad/s"),
        Some("rad/s"),
    );
    assert!(ok, "field weakening base/top speed equality checks failed");
}

#[test]
fn d4_inscribed_circle_margin() {
    let p = FieldWeakeningParams {
        v_bus: 48.0,
        lambda: 0.010,
        l_d: 100e-6,
        l_q: 100e-6,
        i_max: 40.0,
    };
    let ev = voltage_limit_field_weakening(&p);
    let expected = 1.0 - 3.0_f64.sqrt() / 2.0; // 0.13397459621556135...
    let ok = (ev.inscribed_margin - expected).abs() <= 1e-15
        && (ev.inscribed_margin - 0.134).abs() < 1e-3;

    emit_law(
        "d4_inscribed_circle_margin",
        "Voltage limit and field weakening",
        "frob_conformance::compendium::d4::voltage_limit_field_weakening",
        ok,
        format!("{:.12}", ev.inscribed_margin),
        format!("{expected:.12} (~0.134)"),
        Some("1e-9"),
        Some("1"),
    );
    assert!(ok, "inscribed circle margin 1 - sqrt(3)/2 mismatch");
}

#[test]
fn d4_nc_field_weakening_infinite_top_speed() {
    // When lambda <= L * I_max, the finite top-speed formula V_lim/(lambda - L*I_max) does NOT apply.
    let p_eq = FieldWeakeningParams {
        v_bus: 48.0,
        lambda: 0.004,
        l_d: 100e-6,
        l_q: 100e-6,
        i_max: 40.0, // L*I_max == 0.004 == lambda
    };
    let p_lt = FieldWeakeningParams {
        v_bus: 48.0,
        lambda: 0.003,
        l_d: 100e-6,
        l_q: 100e-6,
        i_max: 40.0, // L*I_max == 0.004 > lambda
    };
    let ev_eq = voltage_limit_field_weakening(&p_eq);
    let ev_lt = voltage_limit_field_weakening(&p_lt);

    // Naive formula produces +inf (eq) and negative speed (lt); oracle explicitly reports Unbounded.
    let naive_eq = ev_eq.v_lim / (p_eq.lambda - p_eq.l_d * p_eq.i_max);
    let naive_lt = ev_lt.v_lim / (p_lt.lambda - p_lt.l_d * p_lt.i_max);
    // At i_d = -lambda/L, i_q = 0, current lies inside circle and voltage ellipse holds at 1e6 rad/s.
    let (lhs_lt, rhs_lt) = voltage_ellipse_lhs_rhs(&p_lt, -p_lt.lambda / p_lt.l_d, 0.0, 1e6);

    let caught = ev_eq.omega_top == TopSpeed::Unbounded
        && ev_lt.omega_top == TopSpeed::Unbounded
        && !naive_eq.is_finite()
        && naive_lt < 0.0
        && lhs_lt <= rhs_lt;

    emit_nc(
        "d4_nc_field_weakening_infinite_top_speed",
        "Voltage limit and field weakening",
        "frob_conformance::compendium::d4::voltage_limit_field_weakening",
        caught,
        format!("eq={:?}, lt={:?}, naive_lt={naive_lt:.1}", ev_eq.omega_top, ev_lt.omega_top),
        "oracle returns Unbounded when lambda <= L*I_max".to_owned(),
        Some("1e-9 rad/s"),
        Some("rad/s"),
    );
    assert!(caught, "lambda <= L*I_max must report Unbounded top speed");
}

// =========================================================================
// Row 5: Thermal node
// =========================================================================

#[test]
fn d4_thermal_node_linear_solution() {
    // §15.7 worked card: R_0 = 0.20 Ω, T_0 = T_amb = 298.15 K (25 °C), alpha = 0.00393 K^-1, G = 0.80 W/K, C = 120 J/K.
    let p = ThermalNodeParams {
        c_th: 120.0,
        g_th: 0.80,
        r_0: 0.20,
        alpha: 0.00393,
        t_0: 298.15,
        t_amb: 298.15,
    };
    let i_crit = (p.g_th / (p.r_0 * p.alpha)).sqrt();
    let mut max_err = 0.0_f64;

    // Test stable (20 A), exact critical (I_crit), and runaway (35 A) regimes vs RK4.
    for &(i, t_end) in &[(20.0, 100.0), (i_crit, 40.0), (35.0, 50.0)] {
        let cf = thermal_node_closed_form(&p, i, p.t_amb, t_end);
        let num = rk4_scalar(p.t_amb, t_end, 4_000, |tw| thermal_node_rhs(&p, i, tw));
        let err = (cf.t_w - num).abs();
        if err > max_err {
            max_err = err;
        }
        let expected_a = (i * i * p.r_0 * p.alpha - p.g_th) / p.c_th;
        assert!((cf.a - expected_a).abs() <= 1e-15);
    }

    // §15.7 worked number: 20 s from cold at 20 A rises by 12.8 K.
    let at_20s = thermal_node_closed_form(&p, 20.0, p.t_amb, 20.0);
    let rise_20s = at_20s.t_w - p.t_amb;
    let ok = max_err <= 1e-6 && (rise_20s - 12.81).abs() < 0.05;

    emit_law(
        "d4_thermal_node_linear_solution",
        "Thermal node",
        "frob_conformance::compendium::d4::thermal_node_closed_form",
        ok,
        format!("max_rk4_err={max_err:.3e} K, rise_20s={rise_20s:.3} K"),
        "closed form matches RK4 within 1e-6 K across stable, critical, runaway".to_owned(),
        Some("1e-6 K"),
        Some("K"),
    );
    assert!(ok, "thermal node closed form diverged from RK4: max_err={max_err}");
}

#[test]
fn d4_nc_thermal_node_wrong_sign_alpha() {
    let p_true = ThermalNodeParams {
        c_th: 120.0,
        g_th: 0.80,
        r_0: 0.20,
        alpha: 0.00393,
        t_0: 298.15,
        t_amb: 298.15,
    };
    let p_bad = ThermalNodeParams { alpha: -p_true.alpha, ..p_true };
    let t_true = thermal_node_closed_form(&p_true, 20.0, p_true.t_amb, 500.0).t_w;
    let t_bad = thermal_node_closed_form(&p_bad, 20.0, p_true.t_amb, 500.0).t_w;
    let diff = (t_true - t_bad).abs();
    let caught = diff > 50.0 && t_bad < t_true;

    emit_nc(
        "d4_nc_thermal_node_wrong_sign_alpha",
        "Thermal node",
        "frob_conformance::compendium::d4::thermal_node_closed_form",
        caught,
        format!("t_true={t_true:.2} K vs wrong_sign_alpha={t_bad:.2} K (diff={diff:.2} K)"),
        "negating alpha grossly underpredicts winding temperature".to_owned(),
        Some("1e-6 K"),
        Some("K"),
    );
    assert!(caught, "negated alpha mutant must diverge significantly from true solution");
}

// =========================================================================
// Row 6: Thermal runaway condition
// =========================================================================

#[test]
fn d4_runaway_critical_current_35_8() {
    // N12-44 KA: R_0 = 0.1 Ω, alpha = 0.0039 K^-1, G_th = 0.5 W/K => I_crit = sqrt(0.5/0.00039) ~ 35.8057 A.
    let p = ThermalNodeParams {
        c_th: 100.0,
        g_th: 0.5,
        r_0: 0.1,
        alpha: 0.0039,
        t_0: 298.15,
        t_amb: 300.0,
    };
    let expected_icrit = (0.5_f64 / 0.00039_f64).sqrt();

    let below = thermal_runaway_analysis(&p, 30.0);
    let at_crit = thermal_runaway_analysis(&p, expected_icrit);
    let above = thermal_runaway_analysis(&p, 40.0);

    let stable_ok = match below.regime {
        ThermalRegime::Stable { t_eq, .. } => {
            let p_amb = copper_loss(&p, 30.0, p.t_amb);
            let s = 30.0 * 30.0 * p.r_0 * p.alpha;
            let expected_eq = p.t_amb + p_amb / (p.g_th - s);
            let long_t = thermal_node_closed_form(&p, 30.0, p.t_amb, 10_000.0).t_w;
            (t_eq - expected_eq).abs() <= 1e-12 && (long_t - t_eq).abs() <= 1e-3
        }
        ThermalRegime::Runaway { .. } => false,
    };

    let runaway_ok = matches!(at_crit.regime, ThermalRegime::Runaway { .. })
        && match above.regime {
            ThermalRegime::Runaway { growth_rate } => {
                let t1 = thermal_node_closed_form(&p, 40.0, p.t_amb, 500.0).t_w;
                let t2 = thermal_node_closed_form(&p, 40.0, p.t_amb, 1_000.0).t_w;
                growth_rate > 0.0 && t2 - p.t_amb > 2.0 * (t1 - p.t_amb)
            }
            ThermalRegime::Stable { .. } => false,
        };

    // Torque-control convex tangency (§15.7 worked card: 16.5 A < slope estimate 25.5 A).
    let card = ThermalNodeParams {
        c_th: 120.0,
        g_th: 0.80,
        r_0: 0.20,
        alpha: 0.00393,
        t_0: 298.15,
        t_amb: 298.15,
    };
    let (i0_tan, _t_tan, i0_slope) = thermal_runaway_torque_control_tangency(&card, -0.0011);
    let torque_ok = (i0_tan - 16.508).abs() < 0.05 && (i0_slope - 25.54).abs() < 0.1 && i0_tan < i0_slope;

    let ok = (below.i_crit - expected_icrit).abs() <= 1e-9
        && (below.i_crit - 35.805_743_701_971_64).abs() <= 1e-9
        && stable_ok
        && runaway_ok
        && torque_ok;

    emit_law(
        "d4_runaway_critical_current_35_8",
        "Thermal runaway condition",
        "frob_conformance::compendium::d4::thermal_runaway_analysis",
        ok,
        format!("I_crit={:.6} A, i0_torque_tangency={i0_tan:.3} A < slope={i0_slope:.3} A", below.i_crit),
        format!("I_crit={expected_icrit:.6} A (~35.8 A); torque tangency ~16.5 A < 25.5 A"),
        Some("1e-9 A"),
        Some("A"),
    );
    assert!(ok, "thermal runaway KA / torque-control tangency failed");
}

#[test]
fn d4_nc_runaway_constant_resistance() {
    let p_true = ThermalNodeParams {
        c_th: 100.0,
        g_th: 0.5,
        r_0: 0.1,
        alpha: 0.0039,
        t_0: 298.15,
        t_amb: 300.0,
    };
    let p_const_r = ThermalNodeParams { alpha: 0.0, ..p_true };
    let i_over = 50.0; // well above I_crit = 35.8 A
    let true_eval = thermal_runaway_analysis(&p_true, i_over);
    // Constant-resistance model (alpha=0) always has a < 0 and settles at T_amb + i^2*R_0/G_th.
    let const_r_late = thermal_node_closed_form(&p_const_r, i_over, p_true.t_amb, 10_000.0);
    let const_r_eq = p_true.t_amb + i_over * i_over * p_true.r_0 / p_true.g_th;
    let caught = matches!(true_eval.regime, ThermalRegime::Runaway { .. })
        && const_r_late.a < 0.0
        && (const_r_late.t_w - const_r_eq).abs() < 1e-3;

    emit_nc(
        "d4_nc_runaway_constant_resistance",
        "Thermal runaway condition",
        "frob_conformance::compendium::d4::thermal_runaway_analysis",
        caught,
        format!("true={:?}, const_R_t_eq={const_r_eq:.1} K (a={:.4})", true_eval.regime, const_r_late.a),
        "constant-resistance model misses runaway above I_crit".to_owned(),
        Some("1e-9 A"),
        Some("A"),
    );
    assert!(caught, "constant-resistance NC must fail to detect runaway");
}

// =========================================================================
// Row 7: Entropy production
// =========================================================================

#[test]
fn d4_entropy_production_nonnegative() {
    let g_th = r(4, 5);
    let cases = [
        (ri(0), ri(300), ri(300)),
        (ri(50), ri(350), ri(300)),
        (ri(0), ri(350), ri(300)),
        (ri(0), ri(280), ri(300)),
        (r(125, 4), r(745, 2), r(597, 2)),
    ];
    let mut all_ok = true;
    for (p_loss, t_w, t_amb) in &cases {
        let sigma = entropy_production(p_loss, &g_th, t_w, t_amb);
        if sigma.is_neg() {
            all_ok = false;
        }
        if (p_loss.is_pos() || t_w != t_amb) && !sigma.is_pos() {
            all_ok = false;
        }
    }

    emit_law(
        "d4_entropy_production_nonnegative",
        "Entropy production",
        "frob_conformance::compendium::d4::entropy_production",
        all_ok,
        format!("all_{}_cases_nonnegative={all_ok}", cases.len()),
        "sigma >= 0 for all P_loss >= 0, G_th >= 0, T_w > 0, T_amb > 0".to_owned(),
        None,
        Some("W·K^-1"),
    );
    assert!(all_ok, "entropy production must be nonnegative");
}

#[test]
fn d4_negated_loss_gives_negative_entropy() {
    // Planted defect: regenerative mode mistakenly negates copper loss (P_loss = -i^2*R < 0).
    let p_loss_bad = ri(-25);
    let g_th = r(1, 2);
    let t_w = ri(300);
    let t_amb = ri(300);
    let sigma = entropy_production(&p_loss_bad, &g_th, &t_w, &t_amb);
    let caught = sigma.is_neg() && sigma == r(-1, 12);

    emit_nc(
        "d4_negated_loss_gives_negative_entropy",
        "Entropy production",
        "frob_conformance::compendium::d4::entropy_production",
        caught,
        format!("sigma={sigma} W/K"),
        "sigma < 0 (-1/12 W/K) catches negated copper loss".to_owned(),
        None,
        Some("W·K^-1"),
    );
    assert!(caught, "negated copper loss must produce negative entropy");
}

// =========================================================================
// Row 8: Two-RC battery (compendium-only)
// =========================================================================

fn linear_ocv(z: f64) -> f64 {
    36.0 + 12.0 * z
}

#[test]
fn d4_two_rc_step_response() {
    let p = TwoRcBatteryParams {
        r_0: 0.05,
        r_1: 0.02,
        c_1: 500.0,  // tau_1 = 10 s
        r_2: 0.03,
        c_2: 4000.0, // tau_2 = 120 s
        q_coulombs: 36_000.0, // 10 Ah
    };
    let i_dis = 20.0;
    let z_0 = 0.8;

    let at_t1 = two_rc_battery_step_response(&p, linear_ocv, 0.0, 0.0, z_0, i_dis, 10.0);
    let at_t2 = two_rc_battery_step_response(&p, linear_ocv, 0.0, 0.0, z_0, i_dis, 120.0);
    let at_long = two_rc_battery_step_response(&p, linear_ocv, 0.0, 0.0, z_0, i_dis, 1200.0);

    // RK4 verification of both RC polarization ODEs at t = 25 s.
    let v1_rk4 = rk4_scalar(0.0, 25.0, 4_000, |v| -v / (p.r_1 * p.c_1) + i_dis / p.c_1);
    let v2_rk4 = rk4_scalar(0.0, 25.0, 4_000, |v| -v / (p.r_2 * p.c_2) + i_dis / p.c_2);
    let at_25 = two_rc_battery_step_response(&p, linear_ocv, 0.0, 0.0, z_0, i_dis, 25.0);

    // Exact rational charge balance check at t = 180 s: 0.8 - 20*180/36000 = 0.8 - 0.1 = 0.7.
    let z_exact = battery_soc_exact(&r(4, 5), &ri(20), &ri(180), &ri(36_000));
    let at_180 = two_rc_battery_step_response(&p, linear_ocv, 0.0, 0.0, z_0, i_dis, 180.0);

    let one_m_inv_e = 1.0 - (-1.0_f64).exp();
    let ok = (at_t1.tau_1 - 10.0).abs() <= 1e-12
        && (at_t2.tau_2 - 120.0).abs() <= 1e-12
        && (at_t1.v_1 - i_dis * p.r_1 * one_m_inv_e).abs() <= 1e-12
        && (at_t2.v_2 - i_dis * p.r_2 * one_m_inv_e).abs() <= 1e-12
        && (at_long.v_1 - i_dis * p.r_1).abs() <= 1e-12
        && (at_long.v_2 - i_dis * p.r_2).abs() <= 1e-4
        && (at_25.v_1 - v1_rk4).abs() <= 1e-9
        && (at_25.v_2 - v2_rk4).abs() <= 1e-9
        && z_exact == r(7, 10)
        && (at_180.z - 0.7).abs() <= 1e-12
        && (at_25.v_terminal - (linear_ocv(at_25.z) - p.r_0 * i_dis - at_25.v_1 - at_25.v_2)).abs() <= 1e-12;

    emit_law(
        "d4_two_rc_step_response",
        "Two-RC battery",
        "frob_conformance::compendium::d4::two_rc_battery_step_response",
        ok,
        format!("tau1={} s, tau2={} s, z(180s)={z_exact}, err_v1={:.2e} V", at_t1.tau_1, at_t2.tau_2, (at_25.v_1 - v1_rk4).abs()),
        "V_k -> I*R_k with tau_k = R_k*C_k and z(t) = z_0 - I*t/Q".to_owned(),
        Some("1e-9 V"),
        Some("V"),
    );
    assert!(ok, "two-RC battery step response failed");
}

#[test]
fn d4_nc_two_rc_reversed_current_sign() {
    let z_0 = r(4, 5);
    let i_dis = ri(20);
    let t = ri(180);
    let q = ri(36_000);
    let z_true = battery_soc_exact(&z_0, &i_dis, &t, &q);
    // Wrong sign: dz/dt = +I/Q causes battery to charge while discharging (I > 0).
    let z_bad = &z_0 + &(&(&i_dis * &t) / &q);
    let caught = z_true < z_0 && z_bad > z_0;

    emit_nc(
        "d4_nc_two_rc_reversed_current_sign",
        "Two-RC battery",
        "frob_conformance::compendium::d4::two_rc_battery_step_response",
        caught,
        format!("z_true={z_true} (< {z_0}) vs z_reversed={z_bad} (> {z_0})"),
        "reversed current sign increases SoC during positive discharge".to_owned(),
        None,
        Some("1"),
    );
    assert!(caught, "reversed current sign must erroneously increase SoC on discharge");
}

// =========================================================================
// Row 9: Bus power ceiling (compendium-only)
// =========================================================================

#[test]
fn d4_bus_ceiling_e2_over_4r() {
    // N12-44 KA: E = 48 V, R = 0.1 Ω => ceiling 5760 W at V = 24 V; at P = 2880 W, V = 24*(1 + sqrt(1/2)) ~ 40.97 V.
    let (p_max_exact, v_crit_exact) = bus_power_ceiling_exact(&ri(48), &r(1, 10));
    let at_ceil = bus_operating_point(48.0, 0.1, 5760.0);
    let at_half = bus_operating_point(48.0, 0.1, 2880.0);

    let ceil_ok = matches!(
        at_ceil,
        BusOperatingPoint::Feasible { voltage, current, ceiling }
            if (ceiling - 5760.0).abs() <= 1e-12
            && (voltage - 24.0).abs() <= 1e-9
            && (current - 240.0).abs() <= 1e-9
    );
    let expected_v_half = 24.0 * (1.0 + 0.5_f64.sqrt());
    let half_ok = matches!(
        at_half,
        BusOperatingPoint::Feasible { voltage, current, .. }
            if (voltage - expected_v_half).abs() <= 1e-9
            && (voltage * current - 2880.0).abs() <= 1e-9
            && (48.0 - 0.1 * current - voltage).abs() <= 1e-9
    );

    let ok = p_max_exact == ri(5760) && v_crit_exact == ri(24) && ceil_ok && half_ok;

    emit_law(
        "d4_bus_ceiling_e2_over_4r",
        "Bus power ceiling",
        "frob_conformance::compendium::d4::bus_operating_point",
        ok,
        format!("P_max={p_max_exact} W, V_crit={v_crit_exact} V, at_half={at_half:?}"),
        format!("P_max=5760 W at 24 V; V(2880 W)={expected_v_half:.6} V"),
        Some("1e-9 V"),
        Some("V"),
    );
    assert!(ok, "bus power ceiling KA failed");
}

#[test]
fn d4_bus_brownout_above_ceiling() {
    let op = bus_operating_point(48.0, 0.1, 6000.0);
    let caught = matches!(
        op,
        BusOperatingPoint::Brownout { requested, ceiling }
            if (requested - 6000.0).abs() <= 1e-12 && (ceiling - 5760.0).abs() <= 1e-12
    );

    emit_nc(
        "d4_bus_brownout_above_ceiling",
        "Bus power ceiling",
        "frob_conformance::compendium::d4::bus_operating_point",
        caught,
        format!("{op:?}"),
        "Brownout { requested: 6000.0, ceiling: 5760.0 }".to_owned(),
        Some("1e-9 V"),
        Some("W"),
    );
    assert!(caught, "6000 W load on 48 V / 0.1 Ω bus must return Brownout");
}

// =========================================================================
// Row 10: Four-quadrant transmission (compendium-only)
// =========================================================================

#[test]
fn d4_screw_efficiencies_and_lock() {
    let deg = core::f64::consts::PI / 180.0;
    let ev = screw_efficiencies(10.0 * deg, 5.0 * deg);
    let exp_f = (10.0 * deg).tan() / (15.0 * deg).tan(); // ~0.65807786
    let exp_b = (5.0 * deg).tan() / (10.0 * deg).tan();  // ~0.49617750

    let back_ok = match ev.backdrive {
        ScrewBackdrive::Backdrivable(eb) => {
            (eb - exp_b).abs() <= 1e-12 && (ev.eta_f * eb) < 1.0 && (eb - 0.496).abs() < 1e-3
        }
        ScrewBackdrive::SelfLocking => false,
    };

    // Exact self-locking when phi >= a.
    let lock_eq = screw_efficiencies(10.0 * deg, 10.0 * deg);
    let lock_gt = screw_efficiencies(10.0 * deg, 12.0 * deg);

    // Exact rational round-trip check: motoring then back-driving returns eta_f * eta_b < 1.
    let eta_f = r(4, 5);
    let eta_b = r(3, 4);
    let n = ri(10);
    let tau_m_in = ri(5);
    let tau_j = transmission_motoring_torque(&eta_f, &n, &tau_m_in);
    let tau_m_out = transmission_backdriven_torque(&eta_b, &n, &tau_j);
    let roundtrip_ratio = &tau_m_out / &tau_m_in;

    let ok = (ev.eta_f - exp_f).abs() <= 1e-12
        && (ev.eta_f - 0.658).abs() < 1e-3
        && back_ok
        && lock_eq.backdrive == ScrewBackdrive::SelfLocking
        && lock_gt.backdrive == ScrewBackdrive::SelfLocking
        && roundtrip_ratio == (&eta_f * &eta_b)
        && roundtrip_ratio < Rat::one();

    emit_law(
        "d4_screw_efficiencies_and_lock",
        "Four-quadrant transmission",
        "frob_conformance::compendium::d4::screw_efficiencies",
        ok,
        format!("eta_f={:.6}, backdrive={:?}, roundtrip={roundtrip_ratio}", ev.eta_f, ev.backdrive),
        format!("eta_f={exp_f:.6} (~0.658), eta_b={exp_b:.6} (~0.496), phi>=a locks"),
        Some("1e-9"),
        Some("1"),
    );
    assert!(ok, "four-quadrant screw efficiencies / locking check failed");
}

#[test]
fn d4_single_multiplier_creates_energy() {
    // §15.9 exact rational counterexample: eta = 0.8, N = 10, back-driven tau_joint = 10 N*m at 1 rad/s (10 W in).
    let eta = r(4, 5);
    let n = ri(10);
    let tau_joint = ri(10);
    let omega_joint = ri(1);
    let omega_motor = &n * &omega_joint; // 10 rad/s
    let p_in = &tau_joint * &omega_joint; // 10 W

    // Correct four-quadrant back-driven law: tau_motor = eta * tau_joint / N = 0.8 N*m => 8 W out (2 W lost).
    let tau_m_good = transmission_backdriven_torque(&eta, &n, &tau_joint);
    let p_out_good = &tau_m_good * &omega_motor;

    // Broken single-multiplier law inverted: tau_joint = eta * N * tau_motor => tau_motor = tau_joint / (eta * N) = 1.25 N*m => 12.5 W!
    let tau_m_bad = &tau_joint / &(&eta * &n);
    let p_out_bad = &tau_m_bad * &omega_motor;

    let caught = p_out_good == ri(8)
        && p_out_good < p_in
        && p_out_bad == r(25, 2)
        && p_out_bad > p_in;

    emit_nc(
        "d4_single_multiplier_creates_energy",
        "Four-quadrant transmission",
        "frob_conformance::compendium::d4::screw_efficiencies",
        caught,
        format!("p_in={p_in} W, p_good={p_out_good} W, p_single_mult={p_out_bad} W"),
        "single multiplier returns 12.5 W from 10 W input (+2.5 W created)".to_owned(),
        None,
        Some("W"),
    );
    assert!(caught, "single-multiplier transmission must create energy when back-driven");
}

// =========================================================================
// Row 11: Rational gear law
// =========================================================================

#[test]
fn d4_gear_law_identity() {
    let cases = [
        (r(7, 3), r(5, 2), r(-11, 4), r(2, 9)),
        (r(-4, 5), r(13, 7), r(3, 8), r(15, 2)),
        (ri(12), r(-1, 6), ri(9), r(1, 3)),
    ];
    let mut all_ok = true;
    for (ratio_r, ratio_s, f1, e3) in &cases {
        let single = rational_gear_map(ratio_r, f1, e3);
        let (comp_ratio, comp_eval) = compose_rational_gears(ratio_r, ratio_s, f1, e3);
        let direct_comp = rational_gear_map(&comp_ratio, f1, e3);
        if !single.net_power.is_zero()
            || comp_ratio != (ratio_r * ratio_s)
            || !comp_eval.net_power.is_zero()
            || comp_eval != direct_comp
        {
            all_ok = false;
        }
    }

    emit_law(
        "d4_gear_law_identity",
        "Rational gear law",
        "frob_conformance::compendium::d4::rational_gear_map",
        all_ok,
        format!("all_{}_identities_zero_power={all_ok}", cases.len()),
        "e_1*f_1 + e_2*f_2 == 0 and series composition has ratio r*s".to_owned(),
        None,
        Some("W"),
    );
    assert!(all_ok, "rational gear law identity or composition failed");
}

#[test]
fn d4_nc_gear_law_wrong_sign() {
    let ratio_r = r(7, 3);
    let f_1 = r(5, 2);
    let e_2 = r(11, 4);
    let f_2 = &ratio_r * &f_1;
    // Planted defect: e_1 = +r * e_2 (missing minus sign) leaves 2 * r * e_2 * f_1.
    let e_1_bad = &ratio_r * &e_2;
    let net_bad = &(&e_1_bad * &f_1) + &(&e_2 * &f_2);
    let expected_leak = &(&ri(2) * &ratio_r) * &(&e_2 * &f_1);
    let caught = net_bad == expected_leak && !net_bad.is_zero();

    emit_nc(
        "d4_nc_gear_law_wrong_sign",
        "Rational gear law",
        "frob_conformance::compendium::d4::rational_gear_map",
        caught,
        format!("net_bad={net_bad} W (expected 2*r*e_2*f_1={expected_leak} W)"),
        "e_1 = +r*e_2 leaves nonzero 2*r*e_2*f_1 power residual".to_owned(),
        None,
        Some("W"),
    );
    assert!(caught, "wrong-sign gear law must leave 2*r*e_2*f_1 power residual");
}

// =========================================================================
// Row 12: Energy balance
// =========================================================================

#[test]
fn d4_energy_balance_closes() {
    let motor = MotorAxisParams {
        l: r(1, 100),
        r_w: r(1, 2),
        k_t: r(1, 5),
        k_e: r(1, 5),
        j_r: r(1, 500),
        b: r(1, 1000),
    };
    let c_th = ri(120);
    let g_th = r(4, 5);
    let t_amb = ri(300);
    let h = r(1, 1000);
    let v = ri(24);
    let tau_load = r(3, 10);

    let mut state = MotorThermalState {
        current_i: r(3, 2),
        omega: ri(15),
        temp_w: ri(310),
    };
    let mut all_zero = true;
    let mut total_heat = Rat::zero();
    for _ in 0..3 {
        let (next, ledger, q_gen) =
            motor_thermal_exact_step(&motor, &c_th, &g_th, &t_amb, &state, &v, &tau_load, &h);
        let res = energy_balance_residual(&ledger);
        if !res.is_zero() || !q_gen.is_pos() {
            all_zero = false;
        }
        total_heat = &total_heat + &q_gen;
        state = next;
    }

    emit_law(
        "d4_energy_balance_closes",
        "Energy balance",
        "frob_conformance::compendium::d4::energy_balance_residual",
        all_zero,
        format!("3_step_exact_residual_zero={all_zero}, total_q_gen={total_heat} J"),
        "residual == 0/1 J identically in Q across coupled motor-thermal steps".to_owned(),
        None,
        Some("J"),
    );
    assert!(all_zero, "exact rational motor-thermal energy balance failed to close");
}

#[test]
fn d4_double_counted_heat_flagged() {
    let motor = MotorAxisParams {
        l: r(1, 100),
        r_w: r(1, 2),
        k_t: r(1, 5),
        k_e: r(1, 5),
        j_r: r(1, 500),
        b: r(1, 1000),
    };
    let state = MotorThermalState {
        current_i: ri(2),
        omega: ri(10),
        temp_w: ri(305),
    };
    let (_next, ledger, q_gen) = motor_thermal_exact_step(
        &motor,
        &ri(120),
        &r(4, 5),
        &ri(300),
        &state,
        &ri(24),
        &r(1, 5),
        &r(1, 1000),
    );
    // Planted defect: book heat entering the thermal node as BOTH stored AND dissipated.
    let double_counted = EnergyLedgerStep {
        dissipated: q_gen.clone(),
        ..ledger
    };
    let res = energy_balance_residual(&double_counted);
    let caught = q_gen.is_pos() && res == -&q_gen;

    emit_nc(
        "d4_double_counted_heat_flagged",
        "Energy balance",
        "frob_conformance::compendium::d4::energy_balance_residual",
        caught,
        format!("residual={res} J, -q_gen={} J", -&q_gen),
        "double-counted heat produces exact deficit equal to -q_gen".to_owned(),
        None,
        Some("J"),
    );
    assert!(caught, "double-counting stored heat as loss must produce deficit == -q_gen");
}

// =========================================================================
// Row 13: Cost of transport (compendium-only)
// =========================================================================

#[test]
fn d4_cost_of_transport_dimensionless() {
    let e = ri(1962);
    let m = ri(20);
    let g = r(981, 100);
    let d = ri(10);
    let cot_base = cost_of_transport(&e, &m, &g, &d);
    assert_eq!(cot_base, ri(1));

    // Exact similarity rescaling fixing g: alpha_M = 3/2, alpha_L = 4 (alpha_T = 2), E scales by alpha_M * alpha_L = 6.
    let alpha_m = r(3, 2);
    let alpha_l = ri(4);
    let cot_scaled = cost_of_transport(
        &(&(&alpha_m * &alpha_l) * &e),
        &(&alpha_m * &m),
        &g,
        &(&alpha_l * &d),
    );

    // Bit-exact invariance under power-of-two scaling in f64.
    let e_f = 512.0_f64;
    let m_f = 16.0_f64;
    let g_f = 8.0_f64;
    let d_f = 4.0_f64;
    let cot_f1 = e_f / (m_f * g_f * d_f);
    let cot_f2 = (e_f * 8.0 * 4.0) / ((m_f * 8.0) * g_f * (d_f * 4.0));

    let dim_ok = Dim::ENERGY
        .div(Dim::MASS.mul(Dim::ACCELERATION).mul(Dim::LENGTH))
        .require_dimensionless()
        .is_ok();
    let domain_ok = cost_of_transport_checked(
        EnergyDomain::Mechanical,
        EnergyDomain::Mechanical,
        &e,
        &m,
        &g,
        &d,
    ) == Ok(cot_base.clone())
        && cost_of_transport_checked(
            EnergyDomain::Electrical,
            EnergyDomain::Electrical,
            &e,
            &m,
            &g,
            &d,
        ) == Ok(cot_base.clone());

    let ok = cot_base == cot_scaled && cot_f1.to_bits() == cot_f2.to_bits() && dim_ok && domain_ok;

    emit_law(
        "d4_cost_of_transport_dimensionless",
        "Cost of transport",
        "frob_conformance::compendium::d4::cost_of_transport",
        ok,
        format!("cot_base={cot_base}, cot_scaled={cot_scaled}, f64_bits_equal={}", cot_f1.to_bits() == cot_f2.to_bits()),
        "dimensionless and invariant under similarity & power-of-two rescaling".to_owned(),
        None,
        Some("1"),
    );
    assert!(ok, "cost of transport dimensionless / similarity invariance failed");
}

#[test]
fn d4_nc_cost_of_transport_mixed_energy() {
    let res = cost_of_transport_checked(
        EnergyDomain::Mechanical,
        EnergyDomain::Electrical,
        &ri(500),
        &ri(20),
        &r(981, 100),
        &ri(10),
    );
    let caught = res
        == Err(EnergyDomainMismatch {
            numerator_domain: EnergyDomain::Mechanical,
            claimed_domain: EnergyDomain::Electrical,
        });

    emit_nc(
        "d4_nc_cost_of_transport_mixed_energy",
        "Cost of transport",
        "frob_conformance::compendium::d4::cost_of_transport",
        caught,
        format!("{res:?}"),
        "mixing mechanical numerator with electrical claim is refused".to_owned(),
        None,
        Some("1"),
    );
    assert!(caught, "mixing mechanical and electrical energy in CoT must be refused");
}

// =========================================================================
// Row 14: Froude number (compendium-only)
// =========================================================================

#[test]
fn d4_froude_dimensionless() {
    let v = r(3, 2);
    let g = r(981, 100);
    let l = r(1, 2);
    let fr_base = froude_number(&v, &g, &l);

    // Dynamic similarity: L' = k^2 * L, v' = k * v => Fr' == Fr identically in Q.
    let k = r(5, 2);
    let fr_sim = froude_number(&(&k * &v), &g, &(&(&k * &k) * &l));
    let dim_ok = Dim::VELOCITY
        .powi(2)
        .div(Dim::ACCELERATION.mul(Dim::LENGTH))
        .require_dimensionless()
        .is_ok();
    let ok = fr_base == fr_sim && dim_ok;

    emit_law(
        "d4_froude_dimensionless",
        "Froude number",
        "frob_conformance::compendium::d4::froude_number",
        ok,
        format!("Fr_base={fr_base}, Fr_similar={fr_sim}"),
        "Fr = v^2/(g*L) is dimensionless and invariant under v proportional to sqrt(L)".to_owned(),
        None,
        Some("1"),
    );
    assert!(ok, "Froude number similarity invariance failed");
}

#[test]
fn d4_nc_froude_linear_v_not_dimensionless() {
    // Planted defect: v / (g * L) has dimension [m/s] / ([m/s^2]*[m]) = [s/m] = [-1, 0, 1, 0, 0, 0].
    let bad_dim = Dim::VELOCITY.div(Dim::ACCELERATION.mul(Dim::LENGTH));
    let err = bad_dim.require_dimensionless();
    let caught = err == Err(DimError::NotDimensionless { observed: Dim([-1, 0, 1, 0, 0, 0]) });

    emit_nc(
        "d4_nc_froude_linear_v_not_dimensionless",
        "Froude number",
        "frob_conformance::compendium::d4::froude_number",
        caught,
        format!("{err:?}"),
        "v/(g*L) has dimension [-1, 0, 1, 0, 0, 0] (s/m) and fails dimensionless check".to_owned(),
        None,
        Some("s·m^-1"),
    );
    assert!(caught, "v/(g*L) must fail the dimensionless check");
}

// =========================================================================
// Cross-cutting acceptance criterion: Dimensional homogeneity of all 14 rows
// =========================================================================

#[test]
fn d4_dimensions_homogeneous() {
    assert_eq!(D4_ORACLE_SIGNATURES.len(), 14, "all 14 D.4 rows must declare SI signatures");
    for sig in D4_ORACLE_SIGNATURES {
        assert!(!sig.inputs.is_empty(), "{}: inputs missing", sig.row);
        assert!(!sig.outputs.is_empty(), "{}: outputs missing", sig.row);
        for slot in sig.inputs.iter().chain(sig.outputs.iter()) {
            assert!(!slot.unit.is_empty(), "{}.{}: empty unit", sig.row, slot.name);
        }
    }

    let all_ok = verify_all_oracle_dimensions().is_ok();

    // Negative controls mandated by N12-44 test plan:
    // 1. Adding voltage (V) to current (A) must be refused.
    let v_plus_i = DimQty::new(ri(12), Dim::VOLTAGE).try_add(&DimQty::new(ri(2), Dim::CURRENT));
    let v_i_refused = v_plus_i
        == Err(DimError::InhomogeneousSum {
            lhs: Dim::VOLTAGE,
            rhs: Dim::CURRENT,
        });

    // 2. Adding torque (N*m) to power (W) must be refused.
    let tau_plus_p = DimQty::new(ri(5), Dim::TORQUE).try_add(&DimQty::new(ri(100), Dim::POWER));
    let tau_p_refused = tau_plus_p
        == Err(DimError::InhomogeneousSum {
            lhs: Dim::TORQUE,
            rhs: Dim::POWER,
        });

    // 3. Taking square root of odd exponent vector (e.g. length [1,0,0,0,0,0]) must be refused.
    let odd_sqrt = DimQty::new(4.0_f64, Dim::LENGTH).try_sqrt();
    let odd_sqrt_refused = odd_sqrt == Err(DimError::OddSqrtExponent { dim: Dim::LENGTH });

    let ok = all_ok && v_i_refused && tau_p_refused && odd_sqrt_refused;

    emit_law(
        "d4_dimensions_homogeneous",
        "Dimensional homogeneity (all 14 rows)",
        "frob_conformance::compendium::d4::verify_all_oracle_dimensions",
        ok,
        format!("all_14_homogeneous={all_ok}, V+I_refused={v_i_refused}, tau+P_refused={tau_p_refused}"),
        "all 14 D.4 oracles dimensionally homogeneous; V+I, tau+P, odd sqrt refused".to_owned(),
        None,
        Some("1"),
    );
    assert!(ok, "dimensional homogeneity check failed");
}
