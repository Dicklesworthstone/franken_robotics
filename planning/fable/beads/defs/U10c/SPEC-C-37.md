---
key: "SPEC-C-37"
title: "spec C-37: the four-dimensional contact cone (slide and spin) and joint rows for stiction, backlash, stops"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "lemma", "spine", "spec", "solid", "traction", "sinew", "frob-contact", "frob-actuate", "test"]
parent: "WP-10"
deps: ["N10a-01", "STD-TEST", "STD-E2E"]
needs: []
related: ["BD-084", "BD-115", "BD-154", "BD-155", "BD-167", "N10c-08", "N10-CX", "N10c-10", "CARD-T-07", "BD-050", "BD-069", "BD-079"]
gates: ["GATE-SINEW-04", "GATE-TRAC-04"]
plan_refs: ["§10 C-37", "§10 C-06", "§18.6", "§18.7", "§15.6", "§12.3 T-07", "Appendix D.3", "Appendix E GB-20", "Appendix I.1", "Appendix M.9", "Appendix N.7"]
---
## Background

Construction C-37 of the FrankenRobotics spine extends the contact construction C-06 (contact is a cone program:
the impulse at a contact lies in a friction cone, a linear image of the second-order cone) in two directions.
First, torsional friction becomes the fourth dimension of the contact cone, so sliding and spinning share one
budget. Second, nonsmooth effects inside a joint (a hard stop, Coulomb stiction, gear backlash) become rows of the
same solve as ground contact. Tier Solid [S]. Owners: the Traction subsystem (`frob-contact`, with the `Cone`
abstraction in `frob-algebra`) and the Sinew subsystem (`frob-actuate`).

What it replaces: torsional friction with its own coefficient and no coupling, and friction and backlash buried
in an actuator card. Payoff: one cone for sliding and spin, so a foot that pivots while it slides slips where it
should; and drivetrain stiction and lost motion interact with ground contact in one solve, so backlash that grows
with wear changes a gait in the model that found the gait.

Rejected shortcuts, each with a hand-checkable refutation carried below: independent sliding and torsional rows
(they grant 41% more friction in magnitude than the surface supplies); a pyramid cone (the same 41% along its
diagonals); backlash modelled as a dead zone on the torque command (no impact when the gap closes and no hunting
in a position loop, the two effects a backlash model exists to show). FACT (FrankenSim `483d09c682f2`, observed
2026-11-21): the siblings have elliptic friction and a friction-law library, used as cross-checks.

Terms. λ_n, λ_t = (λ_t1, λ_t2), λ_s: the normal impulse, the two tangential impulses and the spin moment impulse
about the normal at a contact patch. v_n, v_t, ω_n: the corresponding relative velocities. r: the effective patch
radius. μ: the friction coefficient. For a joint: q_out the output coordinate, q_motor the rotor coordinate, N the
gear ratio, β the backlash gap, τ_s the stiction torque. Proof classes: Proved, Certified (instance certificate
re-checked exactly), Tested, Estimated; status HYPOTHESIS.

## Scope

In: `docs/math/C-37.md`; law tests in `crates/frob-contact/tests/laws_c37_cone4.rs` and
`crates/frob-actuate/tests/laws_c37_joint_rows.rs`; registry entries for KA-178 to KA-182 and the control seeded
from CX-089.

Out: joint-level rows in the referee solve (BD-084, at M1) and on the batched tiers (BD-155); the four-dimensional
cone in the `Cone` trait and the cone kernels (BD-154, at M2); rolling and torsional rows with interface cards
(BD-167); the cone-program solver (BD-115); recording GATE-SINEW-04 and GATE-TRAC-04 and deciding
KILL-SINEW-ROWS. The production bindings of this law suite therefore arrive in two stages: rows at M1, cone at M2.

Register row (the shared rule of N10a-01). The construction register
`frob-conformance/registry/constructions.toml` is created by N10a-01 with forty typed rows whose obligation fields
are empty; this bead fills row C-37: `rust_type` (the subject trait of the law suite below, replaced by the
production type when the implementing bead binds it), `verb` (`sim.run`; also `contact.laws`), `failing_test` (a
test of `frob-contact::laws_c37_cone4` with its registered control), `payoff.gate` (GATE-SINEW-04), `statements`,
`known_answers` (KA-178 to KA-182), `cards` (T-07), `gates`, `pilot` left empty (the construction is Solid) and
`generic_comparator` only if this bead claims an advantage over a named generic method. Doctrine D-2 (no
ornamental mathematics) is checked on these fields: a construction with no verb or no failing test is moved to the
parked register by a decision record. The contract `docs/math/C-37.md` follows the ten-section template
`docs/math/TEMPLATE.md` of the same bead.

Order of work (DESIGN; the specification-first rule stated in section 6 of the construction register bead
N10a-01). This bead is written before the code it specifies and does not depend on BD-050 (the `Cone` trait of
`frob-algebra`), BD-069 (actuator cards, the first seed in `frob-actuate`) or BD-079 (collision primitives, the
first seed in `frob-contact`): those beads are listed in `related` and are not prerequisites. The law suite is
generic over its subject trait and closes on the exact reference model and the mutants in its own test support. If
`frob-contact` or `frob-actuate` do not exist when this bead starts, the suite and its support files are written
under `crates/frob-conformance/tests/` with the same file names (that crate exists from BD-025) and the commands
below read `-p frob-conformance`; the bead that implements the construction moves them into the owning crate,
binds its production types to the subject trait and runs the unchanged suite as part of its own acceptance. A
criterion or script step below that names production code or a verb of an implementing bead is owed at that bead's
acceptance and is logged `pending` here until then.

## Technical approach

### A. The four-dimensional cone

```text
𝒦  = { (λ_n, λ_t1, λ_t2, λ_s) : ‖(λ_t1, λ_t2, λ_s/r)‖ ≤ μ·λ_n }
𝒦* = { (v_n, v_t1, v_t2, ω_n) : μ·‖(v_t1, v_t2, r·ω_n)‖ ≤ v_n }
```

| Id | Statement | Class |
|---|---|---|
| A1 | 𝒦 is a linear image of the second-order cone in ℝ⁴, so it drops into the one `Cone` abstraction (Jordan product, spectral decomposition, projection, barrier) and no kernel family is added. Its dual is the set 𝒦* above. A contact with r = 0 masks its spin row and is the three-dimensional cone. | Tested (pairing is nonnegative on 𝒦* × 𝒦 and reaches zero on the boundary) |
| A2 | Maximal dissipation makes the friction impulse oppose the generalized sliding velocity (v_t, r·ω_n). With sliding speed v and spin ω the tangential force is μ·N·v/√(v² + (r·ω)²) and the spin moment is μ·N·r·(r·ω)/√(v² + (r·ω)²): a spinning contact resists sliding less. | Tested |
| A3 | The radius r is a property of the pressure distribution: 2·R/3 for uniform pressure on a disc of radius R, and 3·π·R/16 for a Hertzian distribution. Derivation (uniform): the torsional capacity is ∫ μ·p·ρ dA = (2/3)·μ·N·R. | Tested (quadrature); r is a card parameter and Estimated unless calibrated |
| A4 | Projection identities (theorem card T-07) hold for the four-dimensional cone: P(x) ∈ 𝒦, P(x) − x ∈ 𝒦*, and ⟨P(x), P(x) − x⟩ = 0. | Certified per case (polynomial identities and inequalities over ℚ); residual tolerance in floating point |
| A5 | Anisotropic form, carried from the owning chapter: √((λ_t1/μ₁)² + (λ_t2/μ₂)² + (λ_s/(μ_s·r))²) ≤ λ_n, also a linear image of the second-order cone. | Tested |

Closed-form projection onto {‖x̄‖ ≤ μ·x₀}, derived here and confirmed by A4 in exact arithmetic: if ‖x̄‖ ≤ μ·x₀
return x; if μ·‖x̄‖ ≤ −x₀ return 0; otherwise with t = (x₀ + μ·‖x̄‖)/(1 + μ²) return (t, μ·t·x̄/‖x̄‖). For 𝒦, x̄ is
(λ_t1, λ_t2, λ_s/r).

### B. Joint-level rows

| Id | Statement | Class |
|---|---|---|
| B1 | A hard stop is a unilateral row on the joint coordinate, with a non-negative multiplier and complementarity. | Tested |
| B2 | Coulomb stiction is a row whose multiplier lies in [−τ_s, τ_s] (a one-dimensional cone). Stribeck dependence on speed enters as a parameter update between steps, never inside a step. | Tested |
| B3 | Gear backlash is the pair of unilateral rows −β/2 ≤ q_out − q_motor/N ≤ β/2, each with a non-negative multiplier. Gap closure is an impact. The rotor is its own degree of freedom. | Tested |
| B4 | These rows are rows of the same solve as ground contact. | Structural; measured by the gates below |

### Worked numbers (hand-checkable)

- Coupling: at v_t = r·ω_n the tangential friction magnitude is μ·λ_n/√2, where an uncoupled model gives μ·λ_n.
  With μ = 0.6, N = 100 N, R = 30 mm, uniform pressure: r = 20 mm; pure sliding 60 N; pure spin 1.2 N·m; at
  v = r·ω, 42.4 N and 0.85 N·m. Independent rows would grant 60 N and 1.2 N·m at once. Hertzian: r = 17.7 mm.
- Pure spin: the limiting moment is μ·λ_n·r, which for uniform pressure is (2/3)·μ·N·R.
- Stiction: under a torque ramp k·t a joint with stiction τ_s first moves at t = τ_s/k.
- Backlash loop: under a slow triangular motor motion of reflected amplitude A > β/2, with the output held by
  friction, the output amplitude is A − β/2 and the hysteresis loop has width β.
- Gap closure: a rotor of inertia 10⁻⁴ kg·m² turning at 5 rad/s behind a ratio of 10 crosses a gap of 0.5° in
  β·N/ω₀ = 17.5 ms and, closing inelastically on an output of inertia 0.05 kg·m², leaves it at
  J·N·ω₀/(J·N² + I_out) = 0.083 rad/s.

### Conventions, failure cases and non-claims

- Sign of the backlash coordinate: this contract writes q_out − q_motor/N; the Sinew and Traction chapters write
  δ = θ/N − q, the negative. The interval is symmetric, so the rows are the same; the contract records both names.
- The ellipsoidal cone approximates the exact sliding–spinning law of a finite patch; its error is measured
  against patch quadrature on the referee and recorded in the fidelity ledger. Rolling resistance and tactile
  micro-patches are separate rows and not part of this cone. A pressure distribution is never inferred from one
  resultant wrench; a patch that must carry a tilting moment keeps at least three non-collinear contacts.
- On the fast tiers stiction is a convex potential with a stated stiction speed and backlash closure a compliant
  row; under a transactional profile both are exact rows with located events. Requesting backlash rows for a card
  with no rotor state is refused (a code of the Traction chapter).

### Law-test layout, registry, implementers and gates

`crates/frob-contact/tests/laws_c37_cone4.rs` with `tests/support/c37_cone_ref.rs` (the cone, its dual, the
closed-form projection and the maximal-dissipation law in exact rationals on inputs with rational norms; pressure
quadrature in `f64`), and `crates/frob-actuate/tests/laws_c37_joint_rows.rs` with `tests/support/c37_rows_ref.rs`
(one-degree-of-freedom reference solutions: the stiction ramp, the play operator of backlash, the inelastic gap
closure, the stop). Generic over traits `C37ConeSubject` and `C37RowsSubject`; mutants in the support modules.
Registry: KA-178 to KA-182 with derivation files stating tolerances; the control from CX-089.

Gates: GATE-SINEW-04 (referee at M1, batched tiers at M2): breakaway within one step of τ_s over the ramp rate;
zero output motion inside the gap; loop width within 1% of β; closure speed and energy loss within 1%; and its
other cases. GATE-TRAC-04 at M2: spin stop, stiction breakaway and backlash values within 1%; coupled and diagonal
forces within 0.5% of μ·N/√2 and μ·N; the pyramid and independent-row mutations fail. Kill gate KILL-SINEW-ROWS
(M2): with stiction rows on, the fast-tier solve needs more than 1.5 times the Newton iterations of the
smooth-friction model on the contact battery, or throughput falls by more than 20%, after one repair cycle;
fallback: throughput tiers apply regularized friction as a force, the rows stay on the referee and the strict
profile, and smooth-model results carry a flag.

## Acceptance criteria

- [ ] `docs/math/C-37.md` exists with parts A and B: the cone and its dual, A1 to A5, the projection formula, B1
      to B4, the worked numbers, the sign note and the non-claims.
- [ ] Both law-test targets pass on the reference models; the rows suite passes on the production rows of BD-084
      when they exist, and the cone suite on the production cone of BD-154 when it exists.
- [ ] Every mutant below fails its named law.
- [ ] KA-178 to KA-182 are registered with derivation files, stated tolerances and controls (four of the five
      were printed with no control).
- [ ] The control from CX-089 is attached to GATE-TRAC-04.
- [ ] `scripts/e2e/spec_c37.sh` exits 0.
- [ ] Row C-37 of `frob-conformance/registry/constructions.toml` has its obligation fields filled as the
      Scope states, and `cargo test -p frob-conformance --test g0_doctrine -- d2` reports no failed
      obligation for the row (fields not yet due are reported `pending` under the grace rule of N10a-01).

## Test plan

### Unit tests

- `ka_178_slide_spin_coupling`: at v_t = r·ω_n the tangential magnitude is μ·λ_n/√2 and the spin moment
  μ·λ_n·r/√2; with the worked numbers, 42.426 N and 0.8485 N·m.
- `ka_179_pure_spin`: limiting moment 1.2 N·m for uniform pressure and 1.060 N·m for the Hertzian radius 17.67 mm.
- `patch_radius_quadrature`: numerical integration of uniform and Hertzian pressure gives 2·R/3 and 3·π·R/16 to
  10⁻⁶ relative.
- `maximal_dissipation`: 10³ seeded cases: the law of A2 dissipates at least as much as any other impulse in 𝒦.
- `ka_180_projection_identities`: inputs whose tangential-and-spin part has rational norm (for example (2, 3, 6)
  of norm 7 and (1, 4, 8) of norm 9), rational μ and r, in all three regions: the three identities hold exactly.
- `dual_pairing`: 10³ seeded pairs: ⟨v, λ⟩ ≥ 0 on 𝒦* × 𝒦, with zero attained on boundary pairs.
- `r_zero_is_three_dimensional`: with r = 0 the spin component is masked.
- `ka_181_stiction_ramp`: τ_s = 2 N·m, ramp 4 N·m/s: no motion up to 0.5 s; the first nonzero velocity lies in
  (0.5 s, 0.5 s + one step]; the multiplier stays in [−2, 2] N·m.
- `ka_182_backlash_triangle`: A = 0.02 rad, β = 0.01 rad: output amplitude 0.015 rad; loop width 0.01 rad.
- `backlash_complementarity`: each row's multiplier is non-negative and zero strictly inside the gap.
- `gap_closure_impact`: the worked rotor crosses the gap in 17.45 ms and the output leaves at 0.0833 rad/s.
- `hard_stop_row`: a joint driven into its stop has zero velocity through the stop and a non-negative multiplier.
- `stribeck_updates_between_steps`: τ_s is constant inside a step.

### End-to-end script and logging

`scripts/e2e/spec_c37.sh`: `frob --json doctor`; the two cargo test targets; `cargo test -p frob-conformance
--test g0 -- KA-180`; `cargo test -p frob-conformance --test g1 -- KA-178 KA-179 KA-181 KA-182`; `frob --json
prove list` filtered to card T-07; file and registry checks. Log to `target/e2e-logs/spec_c37/<utc>.ndjson` with a
transcript; each line has `ts`, `script`, `bead`, `step`, `event`, command line, exit code, wall time, the
evidence block (claim Known answer, class Tested, tier, determinism D0 for rational rows and the host class for
`f64` rows, seed), measured forces, moments, times and amplitudes with units against expected and thresholds, and
on failure the first differing quantity. The `summary` line counts assertions and names GATE-SINEW-04 and
GATE-TRAC-04.

### Known answers and negative controls

| Row | Truth | Tolerance (stated here where printed `to set`) | Tier | Control |
|---|---|---|---|---|
| KA-178 | At v_t = r·ω_n the tangential friction magnitude is μ·λ_n/√2 | 0.5% (the tolerance of GATE-TRAC-04) | G1 referee | Mutant `independent_rows`: gives μ·λ_n |
| KA-179 | Limiting spin moment μ·λ_n·r; r = 2·R/3 (uniform), 3·π·R/16 (Hertzian) | 1% | G1 referee | Mutant `radius_is_patch_radius` (new): uses r = R |
| KA-180 | P(x) ∈ 𝒦, P(x) − x ∈ 𝒦*, ⟨P(x), P(x) − x⟩ = 0 | exact over ℚ; 10⁻¹² relative in `f64` | G0 host | Mutant `projection_onto_pyramid` (new) |
| KA-181 | Under a torque ramp k·t a joint with stiction τ_s first moves at t = τ_s/k | within one step | G1 referee | Mutant `viscous_only` (new): moves from t = 0 |
| KA-182 | Output amplitude A − β/2; hysteresis loop of width β | 1% | G1 referee | Mutant `torque_dead_zone` (new): no impact at closure and a wrong loop |

Counterexample carried: CX-089 (sliding and torsional friction are not independent rows). Determinism: rational
rows exact; `f64` rows twice on one machine; the production bindings on the referee, then on the batched tiers
within their stated regularization.

## Considerations

- Open questions with fail-closed defaults, owned elsewhere: OPEN-TRAC-2 (where r comes from: the sample geometry
  or the interface card); OPEN-SINEW-2 (the default Stribeck shape and stiction velocity on batched tiers: the
  rational shape and 10⁻³ rad/s, overridable by a card; owner the Sinew lead; decided at GATE-SINEW-04).
- The registry keeps two backlash-closure fixtures with different illustrative cards (this one, 17.5 ms and
  0.083 rad/s; another closing at 0.577 rad/s with 0.0115 J); both are internally consistent and are separate
  rows.
- Risk carried from the risk register: creep of the convex contact model can mislead wheel and stair results; the
  four-dimensional cone and joint rows are named there as part of the mitigation.
- Performance: the rows add columns to the dense solve; the owning gate checks that twelve backlash joints leave
  the dense factorization the size it has with none.

## References

Plan §10 C-37, C-06; §18.6, §18.7, §15.6; theorem card T-07; Appendix D.3 "Four-dimensional friction cone",
"Joint-level rows"; GB-20; Appendix I.1 (`Cone`); KA-178 to KA-182; CX-089; GATE-SINEW-04, -09; GATE-TRAC-04;
KILL-SINEW-ROWS; OPEN-TRAC-2, OPEN-SINEW-2; RISK-05.
