---
key: "SPEC-C-20"
title: "spec C-20 (frob-sense, frob-analyze): observability as a Gramian modulo a gauge; the global ambiguity rule"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "lemma", "spine", "spec", "solid", "frob-sense", "frob-analyze", "test"]
parent: "WP-49"
deps: ["BD-025", "BD-078", "STD-TEST", "STD-E2E"]
needs: ["§10 preamble: the construction register `frob-conformance/registry/constructions.toml`, the ten-section contract template and the shared law-suite convention (bead N10a-01 of unit U10a); this bead fills row C-20", "§10 registries, Appendix M.9 and Appendix N.7: the chapter registry beads N10-KA (unit U10a) and N10-CX (unit U10c) that load rows KA-116 to KA-118 and CX-055; this bead binds them to tests, sets the tolerance the appendix leaves `to set` and registers the controls it leaves unstated"]
related: ["BD-120", "BD-239", "BD-242", "BD-244", "BD-390", "SPEC-C-13", "SPEC-C-19", "SPEC-C-21", "SPEC-C-31", "SPEC-C-39"]
gates: ["GATE-GAUNT-02", "GATE-GAUNT-01"]
plan_refs: ["§10 C-20", "§10 C-13", "§20.11", "§28.4 A-17", "Appendix A", "Appendix D.6", "Appendix M.9 KA-116 to KA-118", "Appendix N.7 CX-055"]
---
## Background

Sensor placement is usually judged by coverage geometry alone, and calibration by the size of its residuals.
Neither says where a design is blind. Construction C-20 fixes the object that does: the **observability
Gramian** of a trajectory, reported by its smallest singular value after the directions that are unobservable
by construction (the *gauge*) have been removed. It gives a map of where a design is blind: feature-starved
corridors and repetitive aisles show as a collapsing index before any estimator drifts. The index feeds sensor
placement, the assumption monitors of the safety construction (C-31) and the probe design of the experiment
construction (C-39). Fisher information on parameters is the same object with parameters in place of states.

The construction also states its own limit and makes it a rule: a Gramian is **local**. It says nothing about
two distant states that produce the same readings, so every calibration and placement analysis also runs a
global ambiguity search and reports the alternatives that would change a mission decision.

Tier: Solid [S]. Owning chapters: Sensorium (crate `frob-sense`, verb `sense.observability`) and the static
analyses (analysis A-17 in `frob-analyze`). No pilot is required for a Solid construction.

Terms. *Error-state transition map* Φ_k: the linear map that carries a small estimation error at the start of
the window to step k, in Lie-algebra coordinates of the invariant error η = X̂⁻¹·X (the error definition of the
estimation construction C-13). *Measurement Jacobian* H_k: the derivative of the sensor outputs at step k with
respect to the error state. *Noise covariance* R_k. *Whitened*: outputs divided by the square root of R_k, so
that every output has unit noise. *Gauge subspace*: directions of the state that no admitted sensor can see,
for example global position and yaw under proprioception and gravity.

Rejected alternatives: coverage geometry alone (it counts rays, not information); residual size (a small
residual on an unobservable direction means nothing); the unreduced Gramian's smallest singular value (always
zero when a gauge exists, so it distinguishes nothing).

## Scope

In: `docs/math/C-20.md` (statements S1 to S6); the law suite `crates/frob-sense/tests/laws_c20.rs` with
reference model and mutants; registry rows KA-116, KA-117, KA-118 with derivation files; controls from CX-055;
row C-20 of the construction register.

Out: the linearized Gramian on the CPU as a verb (BD-120); the empirical Gramian on batched tiers (BD-239); the
alias-witness search that implements the global ambiguity search (BD-240); calibration (BD-242); estimator
consistency, which the Gramian does not see and which the estimation construction tests separately.

## Technical approach

### 1. Statements (status HYPOTHESIS; class once the route has run)

| Id | Statement | Class and route |
|---|---|---|
| C-20.S1 | Along a trajectory, the local observability Gramian over a window is W = Σ_k Φ_kᵀ·H_kᵀ·R_k⁻¹·H_k·Φ_k | Definition; `Tested` against the empirical form |
| C-20.S2 | Empirical form: perturb the initial state by ±δ along each basis direction eᵢ, roll out, and accumulate whitened output differences: W_emp[i, j] = (1/(4δ²))·Σ_k (y_k⁺ⁱ − y_k⁻ⁱ)ᵀ·R_k⁻¹·(y_k⁺ʲ − y_k⁻ʲ). It needs no derivative program and converges to W as δ → 0 | `Tested` (agreement with S1 within the gate's 5% on non-zero eigenvalues; exact on linear fixtures) |
| C-20.S3 | Under proprioception and gravity, global position and yaw form a four-dimensional gauge subspace: the null space of W has dimension four and equals the span of the three translations and yaw | `Tested` (analytic and by finite perturbation) |
| C-20.S4 | The reported index is the smallest singular value of W restricted to the complement of the declared gauge, in whitened units | Definition; `Tested` |
| C-20.S5 | Fisher information on parameters is the same object with parameters in place of states: Σ_k (∂y_k/∂θ)ᵀ·R_k⁻¹·(∂y_k/∂θ) | Definition; `Tested` |
| C-20.S6 | Locality: full local rank does not imply distinguishability. Every calibration and placement analysis also runs a global ambiguity search and reports the alternatives that would change a mission decision | `Certified` on the range-anchor fixture (exact); a rule for analyses |

### 2. Formulas and conventions (contract sections 3 and 5)

- The perturbation of S2 is applied on the group: X₀·exp(±δ·eᵢ) for group-valued states, x₀ ± δ·eᵢ for vector
  parts; y_k⁺ⁱ is the output at step k of the run perturbed along +eᵢ.
- Restriction of S4: with Q a matrix whose columns are an orthonormal basis of the complement of the declared
  gauge, the index is the smallest singular value of Qᵀ·W·Q. The gauge is **declared** by the sensor suite and
  checked: the computed null space must contain the declared gauge, and a null direction outside it is reported
  as a blind direction, not absorbed.
- Units. Outputs are whitened by R_k. States are made dimensionless with declared scales before the singular
  values are taken (DESIGN: the canonical scales of the design, length ℓ₀, time t₀ = √(ℓ₀/g), angle one radian),
  because a singular value of a matrix with mixed units has no meaning.
- Windows are half-open, (a, b]; the window length and the trajectory are part of the result.

### 3. Worked fixtures

**Gauge (KA-116; a synthetic reduced model, labelled `synthetic`).** State: world position p ∈ ℝ³, attitude
(yaw ψ, pitch θ, roll φ), body velocity v ∈ ℝ³. Dynamics over the window: ṗ = R(ψ, θ, φ)·v, attitude and v
constant. Outputs: the accelerometer at rest reads gravity in the body frame, Rᵀ·(0, 0, g); leg odometry from
the joint encoders reads the body-frame velocity v. Neither output depends on p or on ψ at any time (a
rotation about the vertical leaves the body-frame gravity vector unchanged), and pitch, roll and v are read
directly. Hence the null space of W is exactly span{p_x, p_y, p_z, ψ}: dimension four.

**Global ambiguity (KA-117; exact).** Range anchors at (−1, 0) and (1, 0). From (0, 1) both ranges are √2, and
from (0, −1) both are √2. The measurement Jacobian rows are the unit vectors from the anchors: at (0, 1),
(1, 1)/√2 and (−1, 1)/√2, so HᵀH = ½·[[1, 1], [1, 1]] + ½·[[1, −1], [−1, 1]] = I; at (0, −1) likewise I. With
unit variances the information matrix is the identity at both points: full local rank, index 1. Yet a
controller that must pick the side fails, because the two points give identical readings.

**Third anchor (KA-118; exact).** A third anchor at (0, 2) reads 1 from (0, 1) and 3 from (0, −1). Closed
reading intervals of half-width ε, [1 − ε, 1 + ε] and [3 − ε, 3 + ε], are disjoint exactly when ε < 1.

**Corridor (illustration of the collapsing index; exact).** A planar robot between parallel walls y = ±1 with
two range sensors, one to each wall: H = [[0, −1], [0, 1]], HᵀH = diag(0, 2). The index along the corridor
axis is 0: the robot is blind along x although both sensors return valid ranges.

### 4. Honest limits and non-claims

- Full local rank does not imply distinguishability (the fixture above).
- An estimator that reports small uncertainty while drifting is a failure the Gramian does not see;
  consistency is tested separately (normalized estimation error inside its χ² bounds).
- The index is a property of a trajectory and a window, not of a design alone; a placement comparison fixes
  both.
- Foundations (FACT, observed 2026-11-21 at FrankenSim `483d09c682f2`): none in the siblings beyond `fs-ad`
  dual numbers for the Jacobians; not admitted for this use until its admission gate passes.

### 5. Law suite and registry (DESIGN)

- `crates/frob-sense/tests/laws_c20.rs`, generic over `C20Subject { fn gramian(&self, traj: &Trajectory) -> Mat;
  fn empirical_gramian(&self, traj, delta: f64) -> Mat; fn index(&self, w: &Mat, gauge: &Subspace) -> f64;
  fn ambiguities(&self, problem: &RangeProblem) -> Vec<Alternative> }`. `tests/support/c20_ref.rs`: the reduced
  legged model, the range-anchor problems in exact rationals with √2 handled symbolically (squared ranges), a
  symmetric eigen-solver in `f64` (Jacobi) for the gauge test. `tests/support/c20_mutants.rs`: mutants under
  the cargo feature `controls`.
- Register row C-20: `verb = "sense.observability"`, `failing_test` =
  `frob-sense::laws_c20::s6_full_rank_is_not_distinguishability`, `payoff.gate = "GATE-SENS-08"`,
  `known_answers = KA-116, KA-117, KA-118`, no pilot (Solid), `generic_comparator = "sensor coverage geometry
  (analysis A-14)"`.
- Implementers: `frob-sense` (linearized form at M1, empirical form on batched tiers at M3), `frob-analyze`
  (analysis A-17, preflight stage 8). Gate that measures the construction (recorded by BD-120 and BD-239):
  GATE-SENS-08 (the corridor case and the four-dimensional gauge, by the linearized and the empirical form,
  agreeing to within 5% in the non-zero eigenvalues; first rows at M1, the rest at M3). Related gates:
  GATE-SENS-09 (alias witnesses) for the global search.

## Acceptance criteria

- [ ] `docs/math/C-20.md` has the ten sections, statements C-20.S1 to S6 with classes, the formulas and the
      four fixtures.
- [ ] `cargo test -p frob-sense --test laws_c20` passes; with `--features controls` every mutant fails.
- [ ] KA-116: the null space of W on the reduced legged model has dimension exactly four (singular values
      below 10⁻¹⁰·σ_max counted as zero) and its largest principal angle to span{p_x, p_y, p_z, ψ} is at most
      10⁻⁸ rad; the empirical form at δ = 10⁻⁴ gives the same dimension and non-zero eigenvalues within 5%.
- [ ] KA-117: the information matrix is exactly the identity at (0, 1) and at (0, −1); the ambiguity search
      returns both points as alternatives.
- [ ] KA-118: separation holds for ε = 63/64 and fails for ε = 1 (closed intervals share the point 2).
- [ ] The corridor fixture reports index 0 with the blind direction (1, 0).
- [ ] KA-116's tolerance and the controls of KA-116 and KA-118 are recorded in the registry (the appendix
      leaves them `to set` and `none stated`).
- [ ] `scripts/e2e/spec_c20.sh` exits 0.

## Test plan

### Unit tests

- `s1_s2_linear_agreement`: a linear system (synthetic, 4 states, 2 outputs, 20 steps, rational entries): the
  empirical Gramian equals the analytic one exactly for any δ.
- `s3_gauge_dimension_four`: the acceptance values; `s3_gauge_by_finite_perturbation`: moving the initial
  state by 1 m in each translation and by 0.3 rad in yaw changes no output sample (differences exactly zero).
- `s3_pitch_roll_observable`: perturbing pitch or roll changes the accelerometer output.
- `s4_index_on_complement`: with the declared four-dimensional gauge the index is positive; with a mis-declared
  gauge of translations only the index is zero and yaw is reported as a blind direction.
- `s5_fisher_same_object`: a scalar parameter entering one output linearly: the Fisher information equals the
  Gramian entry of the equivalent augmented state.
- `s6_full_rank_is_not_distinguishability`: KA-117; `s6_third_anchor`: KA-118; `corridor_blind_axis`.
- `controls_are_caught`.

### End-to-end script and logging

`scripts/e2e/spec_c20.sh` (idempotent; seed 2000 printed): `frob --json doctor`; the law suite with and without
`--features controls`; `cargo test -p frob-conformance --test g1 -- KA-116 KA-117 KA-118`; existence of
`docs/math/C-20.md` and three derivation files. Sources the STD-E2E helper; writes
`target/e2e-logs/spec_c20/<utc>.ndjson` and a transcript; each line has `ts`, `script`, `bead` (`SPEC-C-20`),
`step`, `event` (`start`, `cmd`, `assert`, `metric`, `refusal`, `end`), command line, exit code, wall time,
evidence block (claim Known answer, class Tested or Certified as recorded, tier G1 host, determinism, seed),
the null-space dimension against 4, the principal angle against 10⁻⁸ rad, the information matrices against the
identity, and on failure the first singular value or matrix entry that differs. Final `summary`: assertion
counts and GATE-GAUNT-02.

### Known answers and negative controls

| Row | Truth | Tolerance | Tier | Control |
|---|---|---|---|---|
| KA-116 | The null space of W has dimension four and equals the span of translations and yaw, analytically and by finite perturbation | exact (dimension); principal angle at most 10⁻⁸ rad in `f64` (set here; DESIGN) | G1 · host | Registered here: a Gramian built without the accelerometer row (null space of dimension six); a gauge declared as translations only (index zero) |
| KA-117 | Anchors at (−1, 0) and (1, 0): both read √2 from (0, 1) and (0, −1); the information matrix is the identity at both; a controller that must pick the side fails | exact | G1 · host | CX-055: full local rank read as distinguishability (the mutant analysis returns "observable" with no alternatives) |
| KA-118 | A third anchor at (0, 2) reads 1 or 3; closed intervals of half-width ε separate exactly when ε < 1 | exact | G1 · host | Registered here: open intervals in place of closed ones, which report separation at ε = 1 |

Metamorphic relations: rotating the whole problem about the vertical leaves W's spectrum unchanged; adding an
absolute heading sensor removes yaw from the gauge (null space of dimension three); rescaling an output and its
noise by the same factor changes nothing. Determinism: the rational fixtures are class D0; the `f64` gauge test
is run twice on one machine and in debug and release with identical bits (a fixed-order Jacobi sweep).

## Considerations

- The gauge must be declared and checked, never inferred silently: absorbing every null direction into "the
  gauge" would hide exactly the blind directions the index exists to expose.
- The 5% agreement of the gate between linearized and empirical forms depends on δ; the contract records the δ
  used and the Richardson check at two step sizes.
- Kill gate on the consumer side, named in advance: preflight stage 8 falls back to an advisory report built on
  coverage geometry if its alias witnesses prove unreliable (KILL-SKEL-ALIAS). The Gramian itself is Solid and
  has no kill gate.
- Privacy: a blindness map of a private scene is derived data of that scene and inherits its privacy mark; the
  law suite uses synthetic fixtures only.

## References

Plan §10 C-20, C-13; §20.11; §28.4 (A-17); Appendix A (gauge, observability Gramian); Appendix D.6
(observability Gramian); KA-116 to KA-118; CX-055; GATE-SENS-08.
