---
key: "SPEC-C-16"
title: "spec C-16 (frob-gait, frob-dyn): hybrid gait orbits: saltation, return map, multipliers, orbit adjoint"
type: "task"
priority: 2
milestone: "M4"
labels: ["m4", "lemma", "spine", "spec", "frontier", "frob-gait", "frob-dyn", "test"]
parent: "WP-64"
deps: ["BD-025", "STD-TEST", "STD-E2E"]
needs: ["§10 preamble: the construction register `frob-conformance/registry/constructions.toml`, the ten-section contract template `docs/math/TEMPLATE.md` and the shared law-suite convention (bead N10a-01 of unit U10a); this bead fills row C-16", "§10 registries, Appendix M.9 and Appendix N.7: the chapter registry beads N10-KA (unit U10a) and N10-CX (unit U10c) that load rows KA-092, KA-093 and CX-041; this bead binds the rows to tests and creates a row only if it is absent"]
related: ["N10b-01", "N10b-02", "N10b-03", "BD-082", "BD-333", "BD-334", "BD-335", "BD-426", "CARD-T-17", "CARD-T-18", "SPEC-C-10", "SPEC-C-09", "BD-081", "BD-332"]
gates: ["GATE-GAUNT-02", "GATE-GAUNT-01"]
plan_refs: ["§10 C-16", "§9 D-8", "§9 D-15", "§12.3 T-17", "§22.9", "Appendix D.2", "Appendix E GB-27", "Appendix M.9 KA-092 KA-093", "Appendix N.7 CX-041"]
---
## Background

FrankenRobotics fixes its mathematics once, in forty constructions (C-01 to C-40), and the subsystem crates
implement them. Construction C-16 is the mathematics of events: what happens to a small perturbation when a foot
touches down, a contact starts to slip, a joint hits its stop or a drive enters protection, and how a periodic
gait is described so that its stability is a number. It replaces two habits: taking gradients through contact on
faith, and judging gait stability by watching a rollout. Tier: Frontier [F]. Owning chapters: Kinesis (dynamics,
crate `frob-dyn`) and Reflex (gaits, crate `frob-gait`). It serves BET-26 (gait orbits as a screening signal)
and doctrine rules D-8 (every domain ships its adjoint or its excuse) and D-15 (a derivative carries its
validity label).

This is a **specification bead**: it writes the construction as a contract (`docs/math/C-16.md`), its law tests
with negative controls, and its registry entries. Mechanism beads (BD-333 return maps and multipliers, BD-334
labeled derivatives, BD-335 continuation) implement it and depend on it. C-16 is long, so it is split: this bead
is the core (events, saltation, return map, multipliers, orbit adjoint); N10b-01 specifies the nine
derivative-validity labels and refusal FROB-REF-263; N10b-02 the symmetry–event identity and stratum counts;
N10b-03 is the paper-scale pilot.

Terms. *Hybrid automaton*: a system with discrete modes, each with a smooth flow, and jumps between modes.
*Guard*: a scalar function whose zero crossing triggers an event. *Reset*: the map applied to the state at an
event. *Saltation matrix*: the first-order map of perturbations across one event. *Poincaré section*: a
hypersurface that the periodic motion crosses once per cycle. *Return map*: the map of the section to itself.
*Floquet multipliers*: the eigenvalues of the derivative of the return map; all inside the unit circle means
local orbital stability. *Relative periodic orbit*: a motion that repeats up to a symmetry (a walker returns to
the same shape one stride further along). *Throughput tiers*: the batched tiers (`batch-cpu`, `gpu-cuda`,
`gpu-metal`), where contact is compliant; the *referee* is the CPU tier that locates events.

Why this design. The reset Jacobian alone is the tempting event derivative and it is wrong (sign, coupling and
area change, shown below). Unrolling a rollout and differentiating gives a gradient with no statement of when it
is invalid. The construction instead composes exact first-order maps at isolated transverse events and refuses
by name where no derivative exists. Rejected alternatives: forcing an order on simultaneous impacts (it can
change the outgoing velocity and a gait's basin, and trot, bound and pronk are simultaneous by construction);
judging stability from a rollout (a judgment, not a number); smoothing every contact (a different system, which
must then be labeled `smoothed`).

## Scope

In: `docs/math/C-16.md` sections for the core statements S1 to S9 below; the law suite
`crates/frob-gait/tests/laws_c16.rs` (return map, multipliers, adjoint) and
`crates/frob-dyn/tests/laws_c16_saltation.rs` (saltation, event-time shift), each with reference model and
mutants; registry rows KA-092 and KA-093 with derivation files; controls seeded from CX-041; row C-16 of the
construction register.

Out: labels and refusals (N10b-01); the symmetry–event identity and strata (N10b-02); the pilot (N10b-03); the
shooting solver, monodromy assembly, `gait.orbit`, `gait.gradient`, `gait.continue` and their gates (BD-333,
BD-334, BD-335); event location itself (BD-082); the gait atlas (C-10, BD-332).

Order of work (DESIGN; the specification-first rule stated in section 6 of the construction register bead
N10a-01). This bead is written before the code it specifies and does not depend on BD-081 (the transactional
referee step of `frob-dyn`) or BD-332 (the gait atlas, the first seed in `frob-gait`): those beads are listed in
`related` and are not prerequisites. The law suite is generic over its subject trait and closes on the exact
reference model and the mutants in its own test support. If `frob-dyn` or `frob-gait` do not exist when this bead
starts, the suite and its support files are written under `crates/frob-conformance/tests/` with the same file
names (that crate exists from BD-025) and the commands below read `-p frob-conformance`; the bead that implements
the construction moves them into the owning crate, binds its production types to the subject trait and runs the
unchanged suite as part of its own acceptance. A criterion or script step below that names production code or a
verb of an implementing bead is owed at that bead's acceptance and is logged `pending` here until then.

## Technical approach

Status of every statement today: HYPOTHESIS. The class named is the one the statement carries once its route has
run. Proof classes are the project's four: `Proved` (general statement accepted by a Lean kernel), `Certified`
(an instance accepted by a checker with an offline-verifiable certificate), `Tested` (held on a stated set of
cases), `Estimated`.

### 1. Object and definitions (contract section 1)

- **Hybrid automaton.** Modes m. Each mode has a domain D_m (a stratum of the state space), a smooth flow
  ẋ = f_m(x), and on each outgoing edge e a guard g_e(x, t) and a reset R_e(x, t).
- **Mode.** A set of active inequalities and bilateral constraints, with a stick or slip label per contact and
  the protection state of each drive.
- **Events.** Touchdown and liftoff; stick to slip and back; joint-limit impact; wheel–leg deployment; electrical
  protection. Each event is keyed by stable geometry identity and clock identity, so the same physical event has
  the same key in two runs.
- **Simultaneity rule (S9).** Events that are simultaneous are resolved by one coupled solve. An order is never
  forced on them and simultaneity alone is never a reason to refuse a gait.

### 2. Statements (contract section 4)

| Id | Statement | Class and route |
|---|---|---|
| C-16.S1 | For an isolated event with transverse crossing, first-order perturbations cross it through Ξ_e = D_x R + (f⁺ − D_x R·f⁻ − D_t R)·(D_x g)/(D_t g + D_x g·f⁻) | `Certified` per event class by theorem card T-17 (polynomial identity after clearing the denominator, checker: FrankenSymPy proof kernel); `Tested` until that certificate exists |
| C-16.S2 | The event time moves by δt = −(D_x g·δx)/(D_t g + D_x g·f⁻) | `Tested` (events re-located on both sides of a finite perturbation) |
| C-16.S3 | Bouncing ball: Ξ = [[−r, 0], [−(1 + r)·g/v⁻, −r]], det Ξ = r² | `Certified` (identity in r, g, v⁻ over ℚ, v⁻ ≠ 0) |
| C-16.S4 | A periodic gait is a fixed point x* = P(x*) of the return map P on a Poincaré section placed at a repeatable transverse event; DP is the ordered product of flow tangent maps and saltation matrices over one cycle, restricted to the section, with phase and the group drift of a relative periodic orbit removed: for x(T) = h·x(0) the monodromy is D(h⁻¹)·DΦ_T | `Tested` |
| C-16.S5 | The Floquet multipliers are the eigenvalues of DP, and local orbital stability is reported as these numbers with intervals. Rimless wheel: multiplier cos²(2α), exactly ½ for eight spokes | `Tested` (closed form on the fixture) |
| C-16.S6 | On the throughput tiers contact is compliant, the step map is continuous and piecewise differentiable, and the monodromy is the product of step Jacobians with no saltation matrix | `Tested` |
| C-16.S7 | Orbit adjoint: at a fixed point, (I − D_x P)ᵀ·λ = ∇_x J; then dJ/dθ = ∂J/∂θ + λᵀ·∂P/∂θ for a design parameter θ. One linear solve per objective. Requires that 1 is not a multiplier | `Tested`; the implicit-function theorem is a named assumption, not re-proved |
| C-16.S8 | Multipliers probe local stability only. For a non-normal monodromy the transient growth max over k ≥ 1 of ‖DPᵏ‖₂ is reported beside the spectral radius | `Tested` (reporting rule) |
| C-16.S9 | The simultaneity rule above | `Tested` (a coupled-solve fixture belongs to the mechanism bead BD-082; here: the rule and its counterexample) |

### 3. Formulas, every symbol defined (contract section 5)

Saltation. x is the state; t time; f⁻ = f_m(x⁻) the flow just before the event at pre-event state x⁻; x⁺ =
R(x⁻, t) the post-event state; f⁺ = f_m′(x⁺) the flow just after; g the guard (scalar); D_x g its row-vector
gradient; D_t g its explicit time derivative; D_x R and D_t R the Jacobians of the reset. The scalar
D_t g + D_x g·f⁻ is the **transversality number**: the rate at which the trajectory crosses the guard.

Derivation (to be reproduced in the contract). A perturbed trajectory x⁻ + δx reaches the guard at t + δt:
0 = D_x g·(δx + f⁻·δt) + D_t g·δt, hence S2. Comparing both trajectories at the same time after the event,
δx⁺ = D_x R·(δx + f⁻·δt) + D_t R·δt − f⁺·δt. Substituting δt gives S1.

Bouncing ball (S3, KA-092). State (y, v); flow (v, −g); guard y = 0; reset (y, −r·v); impact speed v⁻ < 0.
Then D_x R = diag(1, −r), f⁻ = (v⁻, −g), f⁺ = (−r·v⁻, −g), D_x g = (1, 0), D_t g = 0, transversality number v⁻,
f⁺ − D_x R·f⁻ = (−(1 + r)·v⁻, −(1 + r)·g), and Ξ = [[−r, 0], [−(1 + r)·g/v⁻, −r]] with determinant r². The
event time moves by δt = −δy/v⁻. Illustrative numbers: r = 0.8, v⁻ = −2 m/s, g = 9.81 m/s² give
Ξ = [[−0.8, 0], [8.829 s⁻¹, −0.8]], determinant 0.64.

Rimless wheel (S5, KA-093). A hub of point mass on massless spokes of length ℓ, spoke half-angle α (2α between
spokes), rolling down a slope of angle γ with inelastic, non-slipping impacts. Between impacts energy is
conserved: ω₋² = ω₊² + (2g/ℓ)·(cos(γ − α) − cos(γ + α)) = ω₊² + (4g/ℓ)·sin α·sin γ. At impact angular momentum
about the new contact point is conserved: ω₊ = cos(2α)·ω₋. Hence the return map on the post-impact rate
ω₊² ↦ cos²(2α)·(ω₊² + (4g/ℓ)·sin α·sin γ), whose derivative, the multiplier, is cos²(2α). Eight spokes
(α = π/8) give exactly ½; six spokes ¼; twelve spokes ¾. Fixed point z* = c·K/(1 − c) with c = cos²(2α) and
K = (4g/ℓ)·sin α·sin γ; for eight spokes, ℓ = 0.5 m, γ = 4°, g = 9.81 m/s²: z* = 2.095 s⁻², ω₊ = 1.447 rad/s,
and a perturbation of ω₊² halves at every step.

Orbit adjoint (S7). x* = P(x*, θ) gives dx*/dθ = (I − D_x P)⁻¹·∂P/∂θ, so for a metric J(x*, θ):
dJ/dθ = ∂J/∂θ + ∇_x Jᵀ·dx*/dθ = ∂J/∂θ + λᵀ·∂P/∂θ with (I − D_x P)ᵀ·λ = ∇_x J. Rimless-wheel check with
J = z* and θ = γ: λ = 1/(1 − c), ∂P/∂γ = c·(4g/ℓ)·sin α·cos γ, so dJ/dγ = c/(1 − c)·(4g/ℓ)·sin α·cos γ; for
the eight-spoke numbers above, 29.96 s⁻² per radian. It replaces 2·n finite-difference rollouts for n design
parameters by one linear solve per objective.

### 4. Conventions fixed (section 3), failure cases (section 7), non-claims (section 8)

- Perturbations are compared at equal time (not equal phase); Ξ maps pre-event to post-event perturbations.
- D_x g is a row vector; Ξ is n × n for an n-dimensional state; the section restriction removes one dimension.
- The monodromy is always computed on the full reduced state, never on a symmetric subspace alone (the rule is
  construction C-10's; this contract repeats it because S4 depends on it).
- Failure cases: a transversality number near zero (grazing), near-simultaneous events not tied by a symmetry,
  an active-set change under the perturbation, frictional degeneracy: all refused by name (N10b-01). A
  multiplier at 1: S7 is refused (a fold).
- **Retired when.** Event order under perturbation is wrong, or a guard is not transverse: the amortized tangent
  work is withdrawn for that orbit and a derivative-free search takes over.
- Not claimed: a stable orbit on a treadmill says nothing about recovery on a staircase; the recovery basin is
  tested with finite disturbances. An orbit is not an accepted controller. No symplectic or global statement.

### 5. Foundations (section 9; FACT, observed 2026-11-21 at FrankenSim `483d09c682f2`)

`fs-time` has hybrid event location with a transverse, grazing or unresolved crossing classification
(`hybrid.rs`) and event-time sensitivities for a transverse guard (`hybrid_sensitivity.rs`);
`fs-spectral::floquet` analyses a supplied monodromy. No sibling composes them into a return map. Admission is
by gate INT-FSIM-13; none is admitted today.

### 6. Law-suite layout (DESIGN, following the shared convention)

- `crates/frob-dyn/tests/laws_c16_saltation.rs` and `crates/frob-gait/tests/laws_c16.rs`: one test per
  statement and per known answer, generic over the trait
  `C16Subject { fn saltation(&self, ev: &EventData) -> Mat; fn event_time_shift(&self, ev, dx) -> Q;
  fn return_map_jacobian(&self, orbit) -> Mat; fn orbit_adjoint(&self, orbit, grad_j) -> Vec }`, so the same
  laws run on the reference model now and on the kernels of BD-333 and BD-334 when they exist (one fixture
  identity per physical case: the host row and the subsystem row share constants).
- `tests/support/c16_ref.rs`: the reference model in exact rational arithmetic (a reduced fraction over `i128`
  with checked overflow; trigonometric fixtures use exact values, cos²(π/4) = ½). It calls no code under test.
- `tests/support/c16_mutants.rs`: one mutant per negative control, compiled under the cargo feature `controls`
  and selected by its `NC-` identifier; the test `controls_are_caught` asserts each mutant fails its named law.
- Each test emits one structured line through the STD-TEST helper: row id, statement id, measured, expected,
  tolerance.

### 7. Registry entries and implementation map (section 10)

- `frob-conformance/registry/constructions.toml` row C-16: `rust_type` = the saltation and return-map types of
  `frob-gait` (filled when BD-333 lands), `verb = "gait.orbit"`, `failing_test` = `frob-dyn::laws_c16_saltation::
  s3_bouncing_ball` with the reset-Jacobian control, `payoff.gate = "GATE-GAIT-02"`, `cards = ["T-17", "T-18"]`,
  `known_answers = KA-092 … KA-097`, `pilot = "PILOT-C-16"`, `generic_comparator = "central finite differences
  with events re-located on both sides"`, `retired_when` as above.
- `known_answers.toml`: KA-092, KA-093 bound to tests, derivation files `docs/derivations/bouncing-ball.md` and
  `docs/derivations/rimless-wheel.md`. `controls.toml`: controls seeded from CX-041 and the two set below.
- Implementers: `frob-dyn` (events, per-step Jacobians), `frob-gait` (return map, multipliers, adjoint).
  Gates that measure the construction (recorded by the mechanism beads, not here): GATE-GAIT-01 (rimless-wheel
  fixed point and multiplier within 10⁻⁶; compass-gait multipliers within 10⁻⁶ of an independent high-precision
  computation; a planted transverse instability found), GATE-GAIT-02 (100 cases: at least 72 of 80 regular
  cases with gradient cosine at least 0.95 against central differences; all 20 grazing and fold controls labeled
  or refused; no case refused for simultaneity alone), GATE-GAIT-03 (continuation and rank agreement), kill
  gate KILL-GAIT. Theorem cards: T-17 here, T-18 in N10b-02.

## Acceptance criteria

- [ ] `docs/math/C-16.md` exists with the ten sections of the template, statements C-16.S1 to S9 with classes,
      the two derivations, the foundations line with repository, path, pin and date.
- [ ] `cargo test -p frob-dyn --test laws_c16_saltation` and `cargo test -p frob-gait --test laws_c16` pass on
      the reference model; with `--features controls` the test `controls_are_caught` passes (every mutant fails).
- [ ] KA-092: Ξ equals [[−r, 0], [−(1 + r)·g/v⁻, −r]] exactly in ℚ for r ∈ {0, 1/2, 4/5, 1}, v⁻ ∈ {−2, −1/3},
      g = 981/100; det = r²; δt = −δy/v⁻.
- [ ] KA-093: multiplier exactly ½ for eight spokes, ¼ for six, ¾ for twelve; fixed point c·K/(1 − c).
- [ ] The reset-Jacobian mutant (CX-041) is rejected by three independent assertions: first entry sign,
      second-row coupling, determinant (−1 against +1 at r = 1).
- [ ] KA-092 and KA-093 have derivation files, a tolerance that is not `to set` and a registered control that is
      not `none stated`; row C-16 of the construction register names an existing failing test.
- [ ] `scripts/e2e/spec_c16.sh` exits 0 with no failed assertion in its log.

## Test plan

### Unit tests

- `s1_saltation_general_form`: random rational linear flows, affine guards and affine resets in dimension 2 to
  4 (seeded generator, 200 cases); the formula's Ξ equals the matrix obtained by solving the two linearized
  equations of the derivation directly. Expected: equality in ℚ.
- `s2_event_time_shift`: ball with v⁻ = −2 m/s, δy = 1/1000 m: δt = 1/2000 s exactly.
- `s3_bouncing_ball`: the acceptance values above; also r = 4/5, v⁻ = −2, g = 981/100 gives second-row first
  entry 8829/1000 s⁻¹.
- `s4_drift_removed`: rimless wheel with hub position x kept in the state: the unreduced Jacobian has eigenvalue
  1 (drift 2ℓ·sin α per step); after removing the drift the only multiplier is c.
- `s5_rimless_multiplier`: c = cos²(2α) for 6, 8, 12 spokes = ¼, ½, ¾; z* for the eight-spoke numbers within
  10⁻¹² of 2.0949958810 s⁻² in `f64`.
- `s6_no_saltation_on_compliant_map`: a piecewise-linear continuous step map (synthetic): the product of step
  Jacobians equals the finite-difference Jacobian of the composed map away from kinks; inserting a Ξ factor
  breaks the equality.
- `s7_orbit_adjoint`: rimless wheel, J = z*, θ = γ: λ = 2, dJ/dγ = (4g/ℓ)·sin α·cos γ at c = ½; agreement with
  a central difference in γ to 10⁻⁸ relative. Fold: c → 1 is refused, not solved.
- `s8_transient_growth`: M = [[1/2, 10], [0, 1/2]]: spectral radius ½, reported growth max_k ‖Mᵏ‖₂ ≥ 10.
- `controls_are_caught`: every mutant below fails its law.

### End-to-end script and logging

`scripts/e2e/spec_c16.sh` (idempotent, seed-explicit: generator seed 16 printed). Steps: `frob --json doctor`;
`cargo test -p frob-dyn --test laws_c16_saltation`; `cargo test -p frob-gait --test laws_c16`; both again with
`--features controls`; `cargo test -p frob-conformance --test g1 -- KA-092 KA-093`; `frob --json prove list`
and assert card T-17 is listed with a class no stronger than its recorded route supports; check that
`docs/math/C-16.md` and the two derivation files exist and that the register row resolves. It sources the
STD-E2E helper and writes `target/e2e-logs/spec_c16/<utc>.ndjson` and a transcript. Every line has `ts`,
`script`, `bead` (`SPEC-C-16`), `step`, `event` (`start`, `cmd`, `assert`, `metric`, `refusal`, `end`), the exact
command line, exit code, wall time, the evidence block (claim Known answer, class, tier G1 host, determinism D0,
identities, seed), measured and expected values with units and thresholds, and on failure the first differing
matrix entry and the fixture path. The final `summary` line counts assertions and names GATE-GAUNT-02.

### Known answers and negative controls

| Row | Truth | Tolerance | Tier | Control |
|---|---|---|---|---|
| KA-092 | Ξ and det r² as above; δt formula | exact (symbolic, ℚ) | G1 · host here; G1 · referee in BD-333 | CX-041: the reset Jacobian diag(1, −r) used as the event derivative |
| KA-093 | Return map and multiplier cos²(2α); ½ for eight spokes | exact | G1 · host here; G1 · referee in BD-333 | Registered here (the appendix row states none): multiplier cos(2α) in place of cos²(2α) (gives 1/√2); energy gain with cos α in place of sin α |

Further controls: a saltation with the transversality denominator D_x g·f⁺ (post-event flow) in place of
D_x g·f⁻; a drift-not-removed return map (neutral multiplier 1 reported). Metamorphic relations: a change of
state coordinates y = T·x conjugates Ξ to T·Ξ·T⁻¹ and leaves multipliers unchanged; composing two disjoint
events multiplies their saltation matrices in time order. Determinism: the suites run twice on one machine and
in debug and release with identical output lines; the reference model is integer arithmetic (class D0).

## Considerations

- A wrong known answer here propagates to every gait gate; each number above is exact arithmetic that a reader
  can redo by hand, and the derivation files carry the steps.
- T-17 is Frontier and its certificate budget is unmeasured; until it exists S1 is `Tested`, and no report may
  print `Certified` for it (the class of a result is that of its weakest input).
- Kill gate KILL-GAIT (decided at M4): if continuation and rank agreement fail (GATE-GAIT-03), orbits serve as
  stability diagnostics and warm starts only and no design is ranked by an orbit. The contract and law tests
  stay valid in that fallback.
- The referee route for articulated models waits on an upstream FrankenSim work item (hybrid multiple shooting
  on articulated models does not exist there); the compliant-tier route (S6) comes first.
- Revisit if a published or measured case shows the coupled solve of simultaneous impacts disagreeing with
  every ordering limit; the simultaneity rule would then need a stated selection.

## References

Plan §10 C-16; §9 D-8, D-15; §12.3 T-17; §22.9; Appendix D.2 (saltation, return map, orbit gradient, rimless
wheel); Appendix E GB-27; KA-092, KA-093; CX-041, CX-034, CX-035; BET-26; KILL-GAIT; GATE-GAIT-01 to -03.
