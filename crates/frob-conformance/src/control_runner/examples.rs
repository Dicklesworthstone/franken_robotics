#![deny(unsafe_code)]

//! Working physics and mathematical evaluators for all five negative-control kinds (`N38b-04`):
//!
//! 1. `mutation` — Sampled 75 mm wheel-loop stability (`NC-0412`, `CX-262`, Plan §24.10):
//!    exact continuous hold with `R·i` feedforward gives spectral radius `0.920719...` (`0.9207`);
//!    explicit Euler updating current before wheel speed at `0.5 ms` gives `1.497250...` (`1.497`).
//! 2. `planted-input` — Fixed-n paired Clopper–Pearson pre-analysis (`NC-0398`, `KA-002`,
//!    `KA-612`, Plan §24.10/§24.12): practical tie unreachable at `n = 64` (interval `[-0.0662, +0.0662]`,
//!    refused with `FROB-REF-532`), first reachable at `n = 86`.
//! 3. `planted-design` — Quasi-static cart front lift-off torque screen (Plan §24.9/§24.10).
//! 4. `planted-world` — Docking hypothesis support enlarged with gain `33/32` (`CX-282`,
//!    Plan §20.19/§28.13): exact rational minimax worst-case error `1/65 m`, transported
//!    two-gain certificate rejected.
//! 5. `a-a` — One-sided 95% paired lower bound on ten equal arms (`GATE-GAUNT-09` / `PO-GAUNT-01`,
//!    Plan §38.12): exact `1.0`, does not clear `1.02`.

use frob_core::registry::toml::{self, Value as TomlValue};

use crate::compendium::rat::{Rat, r};

// ============================================================================
// 1. Mutation example: Sampled 75 mm wheel loop (NC-0412 / CX-262, Plan §24.10)
// ============================================================================

/// 75 mm wheel total reflected inertia: `8.7890625e-4 + 2e-4 = 0.00107890625 kg·m²`.
pub const WHEEL_J: f64 = 0.001_078_906_25;
/// Motor torque constant `k_t = 0.2 N·m/A`.
pub const WHEEL_KT: f64 = 0.2;
/// Viscous shaft damping `b = 0.002 N·m·s/rad`.
pub const WHEEL_B: f64 = 0.002;
/// Winding inductance `L = 10 mH = 0.010 H`.
pub const WHEEL_L: f64 = 0.010;
/// Winding resistance `R = 1 Ω`.
pub const WHEEL_R: f64 = 1.0;
/// Inner current-loop proportional gain `K_i = 4 Ω`.
pub const WHEEL_KI: f64 = 4.0;
/// Outer speed-loop proportional gain `K_ω = 2 A·s/rad`.
pub const WHEEL_KW: f64 = 2.0;
/// Outer loop hold period `T = 10 ms = 0.010 s` (100 Hz).
pub const WHEEL_HOLD_S: f64 = 0.010;
/// Physics sub-step `h = 0.5 ms = 0.0005 s` (20 sub-steps per 10 ms hold).
pub const WHEEL_SUBSTEP_S: f64 = 0.0005;
/// Number of 0.5 ms physics ticks per 10 ms outer-loop hold.
pub const WHEEL_SUBSTEPS: usize = 20;

fn spectral_radius_2x2(m: [[f64; 2]; 2]) -> f64 {
    let tr = m[0][0] + m[1][1];
    let det = m[0][0] * m[1][1] - m[0][1] * m[1][0];
    let disc = tr * tr - 4.0 * det;
    if disc >= 0.0 {
        let s = disc.sqrt();
        (0.5 * (tr + s)).abs().max((0.5 * (tr - s)).abs())
    } else {
        det.max(0.0).sqrt()
    }
}

fn exact_wheel_loop_spectral_radius_for_gain(gain_i: f64) -> f64 {
    // Linearized error state x = [dw, di]^T under zero-order hold u = -K_w * dw(0):
    //   di/dt = -a * i + a * u,          a = gain_i / L
    //   dw/dt = -c * w + (k_t / J) * i,  c = b / J
    let a = gain_i / WHEEL_L;
    let c = WHEEL_B / WHEEL_J;
    let ea = (-a * WHEEL_HOLD_S).exp();
    let ec = (-c * WHEEL_HOLD_S).exp();
    let i_exp = (ec - ea) / (a - c);
    let i_step = (1.0 - ec) / c - i_exp;
    let m = [
        [
            ec - (WHEEL_KT / WHEEL_J) * WHEEL_KW * i_step,
            (WHEEL_KT / WHEEL_J) * i_exp,
        ],
        [-(1.0 - ea) * WHEEL_KW, ea],
    ];
    spectral_radius_2x2(m)
}

/// Exact one-tick spectral radius of the sampled 75 mm wheel loop with specified
/// measured-current feedforward `R·i` (`gain_i = K_i = 4 Ω`): `0.920719...` (`0.9207`).
#[must_use]
pub fn wheel_loop_spectral_radius_exact() -> f64 {
    exact_wheel_loop_spectral_radius_for_gain(WHEEL_KI)
}

/// Exact one-tick spectral radius with target-current feedforward `R·i_ref`
/// (`gain_i = K_i + R = 5 Ω`): `1.482029...` (`1.4820`).
#[must_use]
pub fn wheel_loop_spectral_radius_r_iref() -> f64 {
    exact_wheel_loop_spectral_radius_for_gain(WHEEL_KI + WHEEL_R)
}

/// Mutant one-tick spectral radius (`CX-262` / `NC-0412`): explicit Euler updating
/// current before wheel speed at `h = 0.5 ms` over 20 ticks (`1.497250...` -> `1.497`).
#[must_use]
pub fn wheel_loop_spectral_radius_euler_mutant() -> f64 {
    let a = WHEEL_KI / WHEEL_L;
    let mut cols = [[0.0_f64; 2]; 2];
    for (col_idx, (w0, i0)) in [(1.0_f64, 0.0_f64), (0.0_f64, 1.0_f64)]
        .into_iter()
        .enumerate()
    {
        let mut w = w0;
        let mut i = i0;
        let u = -WHEEL_KW * w0;
        for _ in 0..WHEEL_SUBSTEPS {
            // Planted defect: update current BEFORE wheel speed at 0.5 ms.
            let i_next = i + WHEEL_SUBSTEP_S * a * (u - i);
            let w_next = w + WHEEL_SUBSTEP_S * (WHEEL_KT * i_next - WHEEL_B * w) / WHEEL_J;
            i = i_next;
            w = w_next;
        }
        cols[col_idx] = [w, i];
    }
    let m = [[cols[0][0], cols[1][0]], [cols[0][1], cols[1][1]]];
    spectral_radius_2x2(m)
}

/// Active wheel-loop spectral radius compiled under the current feature set:
/// evaluates the `CX-262` Euler-order mutant when `feature = "cx-262"` is enabled,
/// and the exact continuous-hold map (`0.9207`) otherwise.
#[must_use]
pub fn wheel_loop_spectral_radius_active() -> f64 {
    #[cfg(feature = "cx-262")]
    {
        wheel_loop_spectral_radius_euler_mutant()
    }
    #[cfg(not(feature = "cx-262"))]
    {
        wheel_loop_spectral_radius_exact()
    }
}

/// Evaluate the M0 host stand-in row for `NC-0412` (`GATE-ARENA-04` wheel-loop stability).
///
/// Passes when `rho` matches `0.9207` to printed digits (`|rho - 0.9207| <= 5e-4`)
/// and stays strictly stable (`rho < 1.0`). Fails with the exact observable from
/// `controls.toml` otherwise.
pub fn evaluate_wheel_loop_host_row(rho: f64) -> Result<String, String> {
    let truth = 0.9207_f64;
    if (rho - truth).abs() <= 5e-4 && rho < 1.0 {
        Ok(format!(
            "wheel-loop row spectral radius {rho:.4} matches 0.9207"
        ))
    } else {
        Err(format!(
            "wheel-loop row reports spectral radius {rho:.3} where 0.9207 is true; the row fails"
        ))
    }
}

// ============================================================================
// 2. Planted-input example: Fixed-n tie verdict at n = 64 vs n = 86 (KA-002 / KA-612)
// ============================================================================

/// Enumeration summary for the fixed-n paired Clopper–Pearson rule (Plan §24.10).
#[derive(Debug, Clone, PartialEq)]
pub struct FixedNPreanalysis {
    pub n: u32,
    pub margin: f64,
    pub total_pairs: u64,
    pub prefer_b: u64,
    pub b_inferior: u64,
    pub practical_tie: u64,
    pub inconclusive: u64,
    /// Upper bound of the simultaneous interval at perfect concordance `(a, b) = (0, 0)`.
    pub perfect_concordance_bound: f64,
}

/// Compute the two-sided 97.5% exact Clopper–Pearson interval `[L, U]` (`alpha = 0.025`,
/// tail probability `0.0125` per side) for `k` events in `n` trials.
#[must_use]
pub fn clopper_pearson_interval_975(k: u32, n: u32) -> (f64, f64) {
    const TAIL: f64 = 0.0125;
    if n == 0 {
        return (0.0, 1.0);
    }
    let lower = if k == 0 {
        0.0
    } else if k == n {
        TAIL.powf(1.0 / f64::from(n))
    } else {
        // Solve P(Bin(n, p) >= k) = TAIL by bisection on p in [0, 1].
        let mut lo = 0.0_f64;
        let mut hi = 1.0_f64;
        for _ in 0..70 {
            let mid = 0.5 * (lo + hi);
            if binom_upper_tail(k, n, mid) < TAIL {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        0.5 * (lo + hi)
    };

    let upper = if k == n {
        1.0
    } else if k == 0 {
        1.0 - TAIL.powf(1.0 / f64::from(n))
    } else {
        // Solve P(Bin(n, p) <= k) = TAIL by bisection on p in [0, 1].
        let mut lo = 0.0_f64;
        let mut hi = 1.0_f64;
        for _ in 0..70 {
            let mid = 0.5 * (lo + hi);
            if binom_lower_tail(k, n, mid) > TAIL {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        0.5 * (lo + hi)
    };

    (lower, upper)
}

fn log_binom_pmf(k: u32, n: u32, p: f64, ln_fact: &[f64]) -> f64 {
    let kf = k as usize;
    let nf = n as usize;
    let log_c = ln_fact[nf] - ln_fact[kf] - ln_fact[nf - kf];
    log_c + f64::from(k) * p.ln() + f64::from(n - k) * (1.0 - p).ln()
}

fn precompute_ln_fact(n: u32) -> Vec<f64> {
    let mut table = vec![0.0_f64; n as usize + 1];
    for i in 1..=(n as usize) {
        table[i] = table[i - 1] + (i as f64).ln();
    }
    table
}

fn binom_lower_tail(k: u32, n: u32, p: f64) -> f64 {
    if p <= 0.0 {
        return 1.0;
    }
    if p >= 1.0 {
        return if k >= n { 1.0 } else { 0.0 };
    }
    let ln_fact = precompute_ln_fact(n);
    let mut sum = 0.0_f64;
    for j in 0..=k {
        sum += log_binom_pmf(j, n, p, &ln_fact).exp();
    }
    sum
}

fn binom_upper_tail(k: u32, n: u32, p: f64) -> f64 {
    if p <= 0.0 {
        return if k == 0 { 1.0 } else { 0.0 };
    }
    if p >= 1.0 {
        return 1.0;
    }
    let ln_fact = precompute_ln_fact(n);
    let mut sum = 0.0_f64;
    for j in k..=n {
        sum += log_binom_pmf(j, n, p, &ln_fact).exp();
    }
    sum
}

/// Enumerate every discordant count pair `(a, b)` with `a + b <= n` under the
/// simultaneous 95% paired Clopper–Pearson rule of Plan §24.10.
#[must_use]
pub fn enumerate_fixed_n_rule(n: u32, margin: f64) -> FixedNPreanalysis {
    let bounds: Vec<(f64, f64)> = (0..=n)
        .map(|k| clopper_pearson_interval_975(k, n))
        .collect();
    let mut total_pairs = 0u64;
    let mut prefer_b = 0u64;
    let mut b_inferior = 0u64;
    let mut practical_tie = 0u64;
    let mut inconclusive = 0u64;

    for a in 0..=n {
        let (la, ua) = bounds[a as usize];
        for b in 0..=(n - a) {
            total_pairs += 1;
            let (lb, ub) = bounds[b as usize];
            let diff_lo = lb - ua;
            let diff_hi = ub - la;
            if diff_lo > margin {
                prefer_b += 1;
            } else if diff_hi < -margin {
                b_inferior += 1;
            } else if diff_lo >= -margin && diff_hi <= margin {
                practical_tie += 1;
            } else {
                inconclusive += 1;
            }
        }
    }

    FixedNPreanalysis {
        n,
        margin,
        total_pairs,
        prefer_b,
        b_inferior,
        practical_tie,
        inconclusive,
        perfect_concordance_bound: bounds[0].1,
    }
}

/// Admit or refuse a fixed-n study plan (`FROB-REF-532` when a requested verdict
/// is unreachable at `(n, margin)`).
///
/// Passing `preanalysis_guard_enabled = false` models the planted mutant where the
/// pre-analysis check is removed; the returned [`FixedNPreanalysis`] still shows
/// `practical_tie == 0` so the reachability enumeration row catches the mutant.
pub fn admit_fixed_n_study(
    n: u32,
    margin: f64,
    requested_verdicts: &[&str],
    preanalysis_guard_enabled: bool,
) -> Result<FixedNPreanalysis, String> {
    let pre = enumerate_fixed_n_rule(n, margin);
    if preanalysis_guard_enabled {
        for &v in requested_verdicts {
            let count = match v {
                "prefer-b" | "promote" => pre.prefer_b,
                "b-inferior" | "inferior" => pre.b_inferior,
                "practical-tie" | "tie" => pre.practical_tie,
                "inconclusive" => pre.inconclusive,
                _ => 0,
            };
            if count == 0 {
                return Err(format!(
                    "FROB-REF-532: fixed-n study at n = {n} requests unreachable verdict `{v}` \
                     (perfect concordance interval [{:.4}, +{:.4}]; practical tie first reachable at n = 86)",
                    -pre.perfect_concordance_bound, pre.perfect_concordance_bound
                ));
            }
        }
    }
    Ok(pre)
}

// ============================================================================
// 3. Planted-design example: Cart front lift-off actuator torque screen (§24.9/§24.10)
// ============================================================================

/// Cart design parameters for the quasi-static front lift-off actuator screen.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CartDesignSpec {
    pub total_mass_kg: f64,
    pub wheel_radius_m: f64,
    pub lip_height_m: f64,
    pub wheelbase_m: f64,
    pub motor_kt_nm_per_a: f64,
    pub current_limit_a: f64,
}

/// Verdict of the exact quasi-static front-axle lift-off actuator classifier.
#[derive(Debug, Clone, PartialEq)]
pub enum DesignVerdict {
    Sound {
        required_torque_nm: f64,
        available_torque_nm: f64,
    },
    Defective {
        required_torque_nm: f64,
        available_torque_nm: f64,
    },
}

/// Classify a cart design by comparing the exact rigid quasi-static front lift-off
/// torque demand `F_0 * x / (1 + x / L)` against `2 * k_t * I_max` (Plan §24.10).
#[must_use]
pub fn classify_cart_design(spec: &CartDesignSpec) -> DesignVerdict {
    const G: f64 = 9.81;
    let f0 = 0.5 * spec.total_mass_kg * G;
    let x = (2.0 * spec.wheel_radius_m * spec.lip_height_m - spec.lip_height_m * spec.lip_height_m)
        .max(0.0)
        .sqrt();
    let required_torque_nm = f0 * x / (1.0 + x / spec.wheelbase_m);
    let available_torque_nm = 2.0 * spec.motor_kt_nm_per_a * spec.current_limit_a;
    if available_torque_nm >= required_torque_nm {
        DesignVerdict::Sound {
            required_torque_nm,
            available_torque_nm,
        }
    } else {
        DesignVerdict::Defective {
            required_torque_nm,
            available_torque_nm,
        }
    }
}

/// Evaluate a synthetic `planted-design` fixture comparing the exact actuator-margin
/// classifier against a wrong mass-only screen.
///
/// Returns `Ok(flagged_report)` when the planted defect is caught (exact classifier
/// says `Defective` while the wrong mass-only screen accepts it).
pub fn evaluate_planted_design_fixture(fixture_text: &str) -> Result<String, String> {
    let tbl =
        toml::parse(fixture_text.as_bytes()).map_err(|e| format!("parse design fixture: {e}"))?;
    let get_f64 = |k: &str| -> Result<f64, String> {
        match tbl.get(k) {
            Some(TomlValue::Str(s)) => s
                .parse::<f64>()
                .map_err(|e| format!("invalid float `{k}`: {e}")),
            Some(TomlValue::Int(i)) => Ok(*i as f64),
            _ => Err(format!("missing numeric key `{k}` in design fixture")),
        }
    };
    let spec = CartDesignSpec {
        total_mass_kg: get_f64("total_mass_kg")?,
        wheel_radius_m: get_f64("wheel_radius_m")?,
        lip_height_m: get_f64("lip_height_m")?,
        wheelbase_m: get_f64("wheelbase_m")?,
        motor_kt_nm_per_a: get_f64("motor_kt_nm_per_a")?,
        current_limit_a: get_f64("current_limit_a")?,
    };
    let naive_mass_screen_accepts = spec.total_mass_kg <= 10.0;
    match classify_cart_design(&spec) {
        DesignVerdict::Defective {
            required_torque_nm,
            available_torque_nm,
        } if naive_mass_screen_accepts => Ok(format!(
            "exact classifier: defective (required torque {required_torque_nm:.3} N·m exceeds \
             available {available_torque_nm:.3} N·m); naive mass screen wrongly accepted"
        )),
        DesignVerdict::Defective {
            required_torque_nm,
            available_torque_nm,
        } => Err(format!(
            "both classifiers rejected design ({required_torque_nm:.3} vs {available_torque_nm:.3} N·m)"
        )),
        DesignVerdict::Sound {
            required_torque_nm,
            available_torque_nm,
        } => Err(format!(
            "design is sound ({available_torque_nm:.3} >= {required_torque_nm:.3} N·m); defect not triggered"
        )),
    }
}

// ============================================================================
// 4. Planted-world example: Docking with third gain 33/32 (CX-282, §20.19 / §28.13)
// ============================================================================

/// Exact rational two-gain docking admissibility (`eps < (U + 1/2) / 4`, Plan §20.19).
#[must_use]
pub fn docking_two_gains_admitted(u: &Rat, eps: &Rat) -> bool {
    let half = r(1, 2);
    let quarter = r(1, 4);
    let limit = &(u + &half) * &quarter;
    eps < &limit
}

/// Exact rational worst-case terminal docking error when a third gain `g3 > 1`
/// (such as `33/32`) aliases with gain `1` at observation error radius `eps` (`1/64`).
///
/// At `g1 = 1`, `g3 = 33/32`, `eps = 1/64`:
/// - `1 + eps = 65/64 = 2080/2048`
/// - `g3 * (1 - eps) = (33/32) * (63/64) = 2079/2048 <= 2080/2048`
/// So the reading `(65/64) * p` is admissible under both `g = 1` and `g = 33/32` for
/// every first command `p`; both worlds receive the same remaining commands, yielding
/// final positions `c` and `g3 * c`, whose minimax error around target `1` is
/// `(g3 - 1) / (g3 + 1) = 1/65` (achieved at `c = 2 / (1 + g3) = 64/65`).
pub fn docking_three_gains_min_worst_error(
    u: &Rat,
    eps: &Rat,
    third_gain: &Rat,
) -> Result<Rat, String> {
    let one = r(1, 1);
    if third_gain <= &one {
        return Err(format!("third gain `{third_gain}` must exceed 1"));
    }
    if !docking_two_gains_admitted(u, eps) {
        return Err("two-gain baseline cell is already excluded".to_owned());
    }
    let g1_upper = &one + eps;
    let g3_lower = third_gain * &(&one - eps);
    if g3_lower <= g1_upper {
        // Worlds g=1 and g=third_gain alias: exact minimax error is (g3 - 1) / (g3 + 1).
        let num = third_gain - &one;
        let den = third_gain + &one;
        Ok(&num / &den)
    } else {
        Ok(Rat::zero())
    }
}

/// Verify whether a docking certificate certified for `cert_gains` remains valid on
/// `world_gains` at `(u, eps)`. Rejects transported certificates whenever support
/// enlargement introduces an aliasing gain (`33/32` -> `1/65 m` worst-case error).
pub fn verify_transported_docking_certificate(
    cert_gains: &[Rat],
    world_gains: &[Rat],
    u: &Rat,
    eps: &Rat,
) -> Result<String, String> {
    for wg in world_gains {
        if !cert_gains.contains(wg) {
            let err = docking_three_gains_min_worst_error(u, eps, wg)?;
            if err > Rat::zero() {
                return Err(format!(
                    "reject: transported certificate invalid on enlarged support (added gain {wg}); \
                     no exact docking; best worst-case error {err} m"
                ));
            }
            return Err(format!(
                "reject: transported certificate support mismatch (extra gain {wg})"
            ));
        }
    }
    if docking_two_gains_admitted(u, eps) {
        Ok("certificate valid on declared two-gain support".to_owned())
    } else {
        Err("reject: cell excluded on declared support".to_owned())
    }
}

pub(crate) fn evaluate_planted_world_docking_fixture(fixture_text: &str) -> Result<String, String> {
    let tbl =
        toml::parse(fixture_text.as_bytes()).map_err(|e| format!("parse world fixture: {e}"))?;
    let u = parse_rat_toml(&tbl, "u").unwrap_or_else(|| r(3, 4));
    let eps = parse_rat_toml(&tbl, "eps").unwrap_or_else(|| r(1, 64));
    let third = parse_rat_toml(&tbl, "third_gain").unwrap_or_else(|| r(33, 32));
    let cert_gains = vec![r(1, 1), r(2, 1)];
    let world_gains = vec![r(1, 1), third, r(2, 1)];
    match verify_transported_docking_certificate(&cert_gains, &world_gains, &u, &eps) {
        Err(rejected) => Ok(rejected),
        Ok(accepted) => Err(format!(
            "transported certificate unexpectedly accepted: {accepted}"
        )),
    }
}

fn parse_rat_toml(tbl: &toml::Table, key: &str) -> Option<Rat> {
    let s = tbl.get(key)?.as_str()?;
    if let Some((a, b)) = s.trim().split_once('/') {
        let p = a.trim().parse::<i64>().ok()?;
        let q = b.trim().parse::<i64>().ok()?;
        if q != 0 {
            return Some(r(p, q));
        }
    }
    None
}

// ============================================================================
// 5. A/A example: One-sided 95% paired lower bound (GATE-GAUNT-09 / PO-GAUNT-01)
// ============================================================================

/// Exact one-sided 95% Student-t critical value `t_{0.95, df}` for `df >= 1`.
fn t_crit_95(df: usize) -> f64 {
    match df {
        1 => 6.313_751_514_800_932,
        2 => 2.919_985_580_355_516,
        3 => 2.353_363_434_801_826_4,
        4 => 2.131_846_786_326_649,
        5 => 2.015_048_373_333_023_3,
        6 => 1.943_180_280_515_302_2,
        7 => 1.894_578_605_090_005,
        8 => 1.859_548_037_522_842_4,
        9 => 1.833_112_932_653_633_5,
        10 => 1.812_461_122_811_675_6,
        _ => {
            // Cornish-Fisher expansion accurate to <1e-5 for df > 10.
            let z: f64 = 1.644_853_626_951_472_2;
            let d = df as f64;
            z + (z.powi(3) + z) / (4.0 * d)
                + (5.0 * z.powi(5) + 16.0 * z.powi(3) + 3.0 * z) / (96.0 * d * d)
        }
    }
}

/// Compute the one-sided 95% paired lower bound of the geometric ratio `arm_b / arm_a`.
///
/// On `n >= 2` pairs of constant ratio `r` (zero variance), returns `r` **exactly**
/// (`1.0` for ten pairs of equal arms, `PO-GAUNT-01`).
pub fn paired_lower_bound_95(arm_a: &[f64], arm_b: &[f64]) -> Result<f64, String> {
    if arm_a.len() != arm_b.len() || arm_a.len() < 2 {
        return Err(format!(
            "paired lower bound requires at least 2 equal-length arms (got {} vs {})",
            arm_a.len(),
            arm_b.len()
        ));
    }
    let mut ratios = Vec::with_capacity(arm_a.len());
    for (&a, &b) in arm_a.iter().zip(arm_b.iter()) {
        if !(a.is_finite() && b.is_finite() && a > 0.0 && b > 0.0) {
            return Err(format!("non-positive or non-finite pair ({a}, {b})"));
        }
        ratios.push(b / a);
    }
    // Exact zero-variance fast path: ten pairs of constant ratio r return r exactly.
    let first = ratios[0];
    if ratios.iter().all(|&x| x == first) {
        return Ok(first);
    }
    let n = ratios.len() as f64;
    let logs: Vec<f64> = ratios.iter().map(|x| x.ln()).collect();
    let mean = logs.iter().sum::<f64>() / n;
    let var = logs.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / (n - 1.0);
    let se = (var / n).sqrt();
    let t = t_crit_95(ratios.len() - 1);
    Ok((mean - t * se).exp())
}

/// Evaluate an A/A campaign against `threshold` (e.g. `1.02`): succeeds (returns
/// `Ok(report)` indicating the A/A arm did NOT clear the threshold) when
/// `paired_lower_bound_95 < threshold`.
pub fn evaluate_aa_campaign(
    arm_a: &[f64],
    arm_b: &[f64],
    threshold: f64,
) -> Result<String, String> {
    let lb = paired_lower_bound_95(arm_a, arm_b)?;
    if lb < threshold {
        Ok(format!(
            "a-a paired lower bound {lb:.4} does not clear {threshold:.2}"
        ))
    } else {
        Err(format!(
            "a-a campaign falsely cleared {threshold:.2} with paired lower bound {lb:.4}"
        ))
    }
}

pub(crate) fn evaluate_aa_fixture(fixture_text: &str) -> Result<String, String> {
    let tbl =
        toml::parse(fixture_text.as_bytes()).map_err(|e| format!("parse a-a fixture: {e}"))?;
    let parse_vec = |k: &str| -> Result<Vec<f64>, String> {
        let arr = tbl
            .get(k)
            .and_then(TomlValue::as_array)
            .ok_or_else(|| format!("missing array `{k}` in a-a fixture"))?;
        arr.iter()
            .map(|sp| match &sp.value {
                TomlValue::Str(s) => s
                    .parse::<f64>()
                    .map_err(|e| format!("bad float in `{k}`: {e}")),
                TomlValue::Int(i) => Ok(*i as f64),
                other => Err(format!(
                    "expected number/string in `{k}`, got {}",
                    other.type_name()
                )),
            })
            .collect()
    };
    let arm_a = parse_vec("arm_a")?;
    let arm_b = parse_vec("arm_b")?;
    let threshold = match tbl.get("threshold") {
        Some(TomlValue::Str(s)) => s.parse::<f64>().unwrap_or(1.02),
        Some(TomlValue::Int(i)) => *i as f64,
        _ => 1.02,
    };
    evaluate_aa_campaign(&arm_a, &arm_b, threshold)
}
