---
key: "SPEC-C-13"
title: "frob-control: C-13 specified — estimation and control on groups, invariant filter, scoring gauge; law tests"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "lemma", "reflex", "frob-control", "solid", "frontier", "spine", "spec", "test"]
parent: "WP-12"
deps: ["BD-025", "N10a-01", "STD-TEST", "STD-E2E"]
needs: ["§22: the bead that creates crate frob-control (BD-114 or BD-115 lands first; the law suite is created with the crate skeleton if neither has)", "§10 C-20: observability modulo a gauge, which states that position and yaw are gauge freedoms (SPEC-C-20, another unit)", "§10 C-31 and C-32: barrier certificates for reduced-order templates and the separating dual witness (SPEC-C-31, SPEC-C-32, another unit)"]
related: ["BD-114", "BD-115", "BD-116", "BD-117", "BD-118", "BD-249", "BD-251", "N10a-20", "SPEC-C-04", "SPEC-C-06", "N10-KA1", "BD-045"]
gates: ["GATE-REFLEX-01", "GATE-REFLEX-03", "GATE-REFLEX-05"]
plan_refs: ["§10 C-13", "§22.3", "§22.4", "§22.5", "§6 GOAL-13", "§4 BET-06", "Appendix D.7", "Appendix M KA-081 to KA-084", "Appendix N CX-038"]
---
## Background

Construction C-13 puts state estimation and model-based control on the same groups as the dynamics. It
replaces Euler-angle PD loops and estimators that become inconsistent (an estimator is inconsistent when its
reported covariance is smaller than its real error).

Payoff:

- Every design receives an estimator and a baseline controller synthesized from its model (the goal that every
  design gets day-one feedback without training).
- Policies can be evaluated on estimated state instead of privileged state.
- The control programs reuse the cone solver of the contact construction, so one kernel family serves physics
  and control.
- When a requested task wrench lies outside a body's attainable set, the same solver returns a separating dual
  witness before any training.

Tier: Solid for geometric PD and the invariant filter; Frontier for controller synthesis across morphologies
and for reduced-order barrier certificates. Pilot for synthesis across morphologies: N10a-20. Fallback named in
advance: the three-outcome report says `not-synthesizable` and the design waits for a learned or sampling
controller.

Foundations (FACT, FrankenSim `483d09c682f2`, 2026-11-21; existence is not admission): `fs-control` is motor
current and speed control only (about 2.6 thousand lines); `fs-ga::lie_opt` has geodesic means and
`se3_optimize`; model-predictive synthesis in FrankenSim is proposed, not built.

## Scope

In: `docs/math/C-13.md`; the law suite `crates/frob-control/tests/laws_c13.rs`; fixtures for the invariant
error, the attitude error near π, a small cone program with its optimality conditions, and the scoring gauge;
the register row; tolerances and controls for KA-081 to KA-084.

Out: the invariant filter and its verb (BD-114); the cone-program solver (BD-115); baseline controllers and
synthesis (BD-116, BD-117, BD-118); control programs with layer records (BD-251); the synthesis pilot
(N10a-20); observability and gauge analysis (construction C-20); certificates (constructions C-31, C-32).

Order of work (DESIGN; the specification-first rule stated in section 6 of the construction register bead
N10a-01). This bead is written before the code it specifies and does not depend on BD-045 (the extended pose
groups of `frob-lie`): that bead is listed in `related` and is not a prerequisite. The law suite is generic over
its subject trait and closes on the exact reference model and the mutants in its own test support. If
`frob-control` does not exist when this bead starts, the suite and its support files are written under
`crates/frob-conformance/tests/` with the same file names (that crate exists from BD-025) and the commands below
read `-p frob-conformance`; the bead that implements the construction moves them into the owning crate, binds its
production types to the subject trait and runs the unchanged suite as part of its own acceptance. A criterion or
script step below that names production code or a verb of an implementing bead is owed at that bead's acceptance
and is logged `pending` here until then.

## Technical approach

**The object.**

- *State as a group element.* The floating-base state (rotation R, velocity v, position p) is an element of
  SE₂(3), the group of 5×5 matrices [[R, v, p], [0, 1, 0], [0, 0, 1]]. With k contact points d₁ … d_k appended
  as further columns it is SE₂₊ₖ(3).
- *Group-affine dynamics.* IMU-driven dynamics are group-affine, so the invariant error obeys an exactly
  linear equation and the invariant extended Kalman filter applies (Barrau and Bonnabel; Hartley et al.). With
  estimate X̂ and truth X the invariant error is η = X̂⁻¹·X (or X·X̂⁻¹ for the other invariance), and for
  group-affine dynamics log η obeys a linear differential equation whose matrix does not depend on the
  estimated trajectory. Noise enters to first order.
- *Control errors are group errors.* The configuration error is g_d⁻¹·g and its logarithm. The attitude error
  is e_R = ½·(R_dᵀ·R − Rᵀ·R_d)^∨, with ^∨ the map from a skew matrix to its vector. It is almost-global: it
  vanishes at R = R_d and also at rotations by π from the target, which are excluded.
- *Centroidal model-predictive control* is a program over contact wrenches in the friction cones subject to
  the reduced (centroidal momentum) equation of the dynamics construction.
- *Whole-body control* is an inverse-dynamics program over the same cones.
- *Flat wheeled bases* are tracked through their flat outputs.

**Optimality conditions of a control program** (the contract's definition of "KKT residuals"). For a program
minimize ½·xᵀQx + cᵀx subject to A·x = b and G·x + s = h with s in a cone 𝒦 (a product of orthants and
second-order cones), with multipliers y and z ∈ 𝒦*: stationarity Q·x + c + Aᵀy + Gᵀz = 0; primal feasibility
A·x = b, G·x + s = h, s ∈ 𝒦; dual feasibility z ∈ 𝒦*; complementarity ⟨s, z⟩ = 0. The residuals are the norms
of the four violations, each scaled by its own data norm.

**The scoring gauge.** A metric gravity-referenced estimate is aligned with one translation and one yaw per
trajectory and never with a fitted scale. Reason: gravity fixes the vertical and the metric scale, so the only
unobservable freedoms are horizontal position, height origin and heading, four numbers in all; fitting a scale
hides an error, because a 2 m estimate of a 1 m motion has zero residual after a factor of ½.

**Honest limits.**

1. With biases, delays and uncertain contact the error equation is no longer state-independent; the filter is
   labelled imperfect-invariant and its log-linearity is evaluated, not assumed.
2. Position and yaw are gauge freedoms under proprioception and gravity.
3. Barrier and sum-of-squares certificates are available for reduced-order templates only; none is claimed for
   full-order dynamics.
4. The baseline controller is a baseline for screening and for day-one feedback, never the evaluator of
   record (the ceiling of the synthesis lane, stated before any measurement).

**Statements.**

| Id | Statement | Class | Route |
|---|---|---|---|
| C-13.S1 | For group-affine dynamics the logarithm of the invariant error obeys a linear equation independent of the estimate | Tested | Property test: propagate two different estimates with the same initial error; the errors' logarithms agree to first order |
| C-13.S2 | Filter consistency: normalized estimation error squared lies inside its χ² bounds | Tested | Monte Carlo on a simulated IMU |
| C-13.S3 | Geometric PD converges from attitude errors near π | Tested | Referee rollouts |
| C-13.S4 | e_R vanishes at the target and at rotations by π from it, and nowhere else | Certified | Exact evaluation on rational rotations |
| C-13.S5 | The KKT residuals of every control program are inside tolerance, or the program returns an infeasibility certificate that checks exactly | Tested; Certified for the infeasibility witness | Residual computation; exact rational check of the dual witness |
| C-13.S6 | An estimate is scored after alignment by one translation and one yaw, never by a fitted scale | Tested | Planted scale error |
| C-13.S7 | With a planted bias the filter is reported imperfect-invariant and the comparator's `overconfident` flag fires | Tested | Planted gyro bias |
| C-13.S8 | A task wrench outside the attainable set yields a separating dual vector | Certified per instance | Exact rational Farkas-type check |

**Fixtures** (DESIGN constants, set here where the construction names a test without numbers).

- *Consistency level.* N = 50 Monte Carlo runs, error dimension n = 9 (rotation, velocity, position). The
  time-averaged normalized estimation error squared ε̄ satisfies N·ε̄ ~ χ² with N·n = 450 degrees of freedom;
  the two-sided 95% acceptance interval is [χ²₀.₀₂₅(450), χ²₀.₉₇₅(450)]/N ≈ [7.82, 10.18] (normal
  approximation 450 ± 1.96·√900; the derivation file carries the exact quantiles).
- *Attitude near π.* Inertia diag(1, 2, 3) kg·m², initial attitude error 179° about the axis
  (1, 1, 1)/√3, zero initial rate, gains k_R = 20 N·m/rad and k_ω = 8 N·m·s/rad on the unit-inertia axis
  scaled by the inertia; truth: the error falls below 1° within 5 s and never passes through the antipode.
- *Antipodal degeneracy.* R = diag(−1, −1, 1), R_d = I: e_R = 0 exactly although the attitude is wrong by π.
- *Small cone program.* minimize ½·‖x‖² − x₁ subject to x in the friction cone with μ = ½ about the first
  axis (‖(x₂, x₃)‖ ≤ ½·x₁): solution x = (1, 0, 0), multiplier zero, all four residuals zero. And an
  infeasible variant: x₁ ≤ −1 together with the cone constraint, whose dual witness is the unit vector on the
  first axis.
- *Scoring.* A straight 1 m motion estimated as a straight 2 m motion: residual after translation-and-yaw
  alignment is 1 m at the end point; after an additional fitted scale it is 0.

**What it does not claim.** Consistency under unmodelled biases or delays; global attitude convergence
(rotations by π are excluded); any barrier certificate for full-order dynamics; that a baseline controller's
verdict is the evaluator of record.

**Implementation map.** Crate `frob-control` (the groups themselves are in `frob-lie`; the cones in
`frob-algebra`). Verbs: `control.estimate` (replay a run's packets through the invariant filter and its
comparator), `control.synth` and `control.show` (synthesis with three outcomes). Gates: GATE-REFLEX-03
(TARGET, M1: filter consistency inside χ² bounds on the simulated IMU; position drift under 2% of distance on
a flat trot, scored by the gauge rule; the comparator's `overconfident` flag fires on a planted bias);
GATE-REFLEX-01 (each archetype stands for 10 s and traverses 5 m of flat ground under its baseline controller;
M1 on the referee for the first archetypes); GATE-REFLEX-05 (a planted infeasible program yields a certificate
that checks exactly and a fallback event; M3). Bet: the baseline-controller bet (at least 80% of corpus
designs that pass the design check traverse 5 m), whose failure is the falsifier of the synthesis lane; kill
gate KILL-BASELINE.

## Acceptance criteria

- [ ] `docs/math/C-13.md` exists with the groups, the invariant error, the control errors, the optimality
      conditions, the gauge rule, the four limits, the eight statements and the fixtures.
- [ ] `cargo test -p frob-control --test laws_c13` passes on the reference implementations and on production
      code as BD-114 and BD-115 land.
- [ ] Consistency: ε̄ inside [7.82, 10.18] at N = 50 on the nominal IMU; outside it with a planted gyro bias,
      with the `overconfident` flag raised.
- [ ] Attitude: from 179° the error is below 1° within 5 s; at exactly π the test asserts e_R = 0 and makes no
      convergence claim.
- [ ] Cone program: the four residuals are below 10⁻⁸ (scaled, `f64`); the infeasible variant returns a dual
      witness that checks exactly in rationals.
- [ ] Scoring: the scale-fitted score is rejected by the scorer; the gauge-aligned score reports 1 m.
- [ ] KA-081 to KA-084 registered with tolerances and controls set; the register row of C-13 filled.

## Test plan

### Unit tests

- `s1_log_linear_error`: two estimates with the same initial invariant error, different trajectories; the
  propagated log-errors differ by second-order terms only (ratio of difference to error squared bounded).
- `s2_nees_consistency`: the consistency fixture; `s7_planted_bias`: the same with a constant gyro bias of
  0.01 rad/s not in the filter's model.
- `s3_pd_near_pi`: the attitude fixture; `s4_antipode`: e_R = 0 at diag(−1, −1, 1); `s4_zero_set`: 10³ random
  rational rotations, e_R = 0 only at the identity and at rotations by π.
- `s5_kkt_small_program`: residuals; `s5_infeasible_certificate`: dual witness accepted by the exact checker;
  a witness with one multiplier perturbed is rejected.
- `s6_scoring_gauge`: the 1 m against 2 m case; also a trajectory rotated by a yaw of 30° and shifted by
  (3, 4, 0) m scores zero after alignment.
- `s8_attainability_witness`: a requested wrench outside a two-contact attainable set returns a separating
  vector whose inner products certify exclusion.

### End-to-end script and logging

`scripts/e2e/spec_c13.sh`. Sources the STD-E2E helper. Steps: law suite; law suite per control; G1 rows KA-081
to KA-084; when the verbs exist, `frob --json sim run --tier referee` on the reference quadruped standing with
a seeded IMU noise card, then `frob --json control estimate <run>` asserting the consistency statistic, the
filter label (`invariant` or `imperfect-invariant`) and the scoring-gauge fields (one translation, one yaw,
no scale); `frob --json control synth` on a template asserting one of the three outcomes. Evidence block: claim
kind known answer, class `Tested`, tier `referee`, seed. Log `target/e2e-logs/spec_c13/<utc>.ndjson` with
transcript: commands, exit codes, ε̄ against its interval, residuals against 10⁻⁸, convergence time against
5 s, and on failure the first run whose statistic left its interval. Final `summary` line.

### Known answers and negative controls

- KA-081 filter consistency: level set to two-sided 95% over 50 runs; G1 · referee. Control (set here): a
  planted unmodelled gyro bias, on which the consistency test must fail and the `overconfident` flag fire.
- KA-082 geometric PD near π: tolerance set to "below 1° within 5 s from 179°" for the stated gains;
  G1 · referee. Control (set here): the attitude error taken as the vector part of the error quaternion
  without the sign rule, which unwinds the long way and misses the time bound.
- KA-083 control programs: residuals below 10⁻⁸ scaled; G1 · host. Control (set here): complementarity left
  unchecked, so a feasible non-optimal point is accepted; the stationarity residual catches it.
- KA-084 scoring gauge: exact rule; G1 · host. Control: a scale-fitted score (a 2 m estimate of a 1 m motion
  has zero residual after a factor of ½).
- CX-038.
- Metamorphic: a horizontal translation and yaw of the world frame changes no filter innovation and no
  gauge-aligned score.
- Determinism: Monte Carlo runs are keyed by run seed and world address; twice on one machine with equal
  digests.

## Considerations

- The consistency interval depends on N and on the error dimension; with contact states appended the
  dimension grows by three per contact and the derivation file recomputes the interval.
- Risk: a filter that passes consistency on the nominal noise and is used with delays. Limit 1 and S7 require
  the label `imperfect-invariant` whenever a bias, delay or contact-uncertainty state is present.
- Risk: synthesized baselines treated as acceptance evidence. Limit 4 is printed on every synthesis report.
- Interaction: the cone solver's end states and dual certificates are shared with contact and with mission
  queries; a defect there is caught by S5 and by the contact construction's law suite independently.

## References

Plan §10 C-13; §22.3, §22.4, §22.5; §6 (GOAL-13); §4 (BET-06); Appendix D.7 (invariant error, attitude error);
Appendix M KA-081 to KA-084; Appendix N CX-038. Pilot: N10a-20.
