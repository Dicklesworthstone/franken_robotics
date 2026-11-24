---
key: "ZOO-balancer-2w"
title: "zoo/balancer-2w.fbot: archetype 18, two-wheeled inverted pendulum: C₂, the open-loop pole in closed form"
type: "task"
priority: 2
milestone: "M3"
labels: ["m3", "skeleton", "reflex", "frob-design", "solid", "test", "zoo"]
parent: "WP-38"
deps: ["BD-054", "BD-055", "BD-056", "BD-084", "BD-107", "BD-108", "BD-114", "N14b-03", "N14b-07", "N38b-47", "NEG-30", "NEG-31"]
needs: []
related: []
gates: []
plan_refs: ["Appendix G.4 row 18", "§14.10", "§15.6", "§28.4 A-12, A-22", "Appendix M KA-269, KA-270", "Appendix M.37"]
---
## Background

`balancer-2w` is a body balanced on two coaxial driven wheels with no caster: a wheeled inverted pendulum. It
is unstable at rest, so it tests what no statically stable archetype can: the estimator and the baseline
controller must work on the first step, because there is no grace period in which a bad state estimate is
harmless. It is the fixture for the modal analysis on an unstable equilibrium (the open-loop pole must match a
closed form) and for sensitivity to the three imperfections that break real balancers: loop latency, joint
stiction and gear backlash. The specification names the pole's closed form and does not write it, so the
known answer has no value until its derivation file exists; this bead supplies a candidate derivation with
its model stated, to be confirmed in that file. No reference robot is on file. It lands at M3.

## Scope

In: `zoo/balancer-2w.fbot` (the template instantiated at its defaults), its expectation file with the full
known-answer table below, the controls, and the generated golden tests (`zoo_balancer…`), including the pole
from three sources (formula, linearized compiled model, referee free response), the modal block structure and
the sensitivity bisection. Out: the template itself with its manifest, levers, presets and default parameter
values, and the template-level unit test `balancer_pole_example`, in N14b-03: the six fixture values below are
the template's defaults, defined there and repeated here. Out: the derivation file
`docs/derivations/balancer-2w-pole.md`, which N38b-47 writes (model, derivation, the same fixture, error
budget, control) and registers against KA-269; this bead depends on it, restates the model and the closed form
so that the expectation file can be computed, and keeps the pole row `blocked: derivation` until the file is
on record. Also out: modes with symmetry block-diagonalization (BD-107); the invariant filter (BD-114); joint
stiction and backlash rows (BD-084); sampled-loop stability (BD-108); the balancing controller (N14b-07), on
which the first-tick and sensitivity tests here depend (it is a prerequisite, not part of this bead).

## Technical approach

**Morphology.** Body; `mirror(xz, driven wheel)`; no caster. Actuated 2; n_v = 8 (floating base 6 plus two
wheel spins); bodies 3; end effectors 2 wheels with law `wheel-rolling`.

**Symmetry.** Γ_mech as built: C₂ (sagittal mirror exchanging the wheels). Default Γ_obs: C₂. Under it the
linearized dynamics split into a symmetric block (pitch and forward motion) and an antisymmetric block (yaw).

**Closed form for the pole (DESIGN; derived and fixed in `docs/derivations/balancer-2w-pole.md` by N38b-47,
which writes the same expression as √(m_b·g·l/(I_b + m_b·l² − (m_b·l)²/M_e)) with M_e = M + m_b).** Planar
model: body of mass m_b with centre of mass at height l above the axle and pitch inertia I_b about its centre
of mass; two wheels of total mass m_w, total axle inertia I_w and radius r, rolling without slip, zero motor
torque. With x the axle position and θ the pitch from upright, and M = m_w + I_w/r², the linearized equations
are (M + m_b)·ẍ + m_b·l·θ̈ = 0 and m_b·l·ẍ + (I_b + m_b·l²)·θ̈ − m_b·g·l·θ = 0. Eliminating ẍ:
λ² = (M + m_b)·m_b·g·l / ((M + m_b)·(I_b + m_b·l²) − m_b²·l²), and the open-loop poles are ±λ plus a double
pole at zero (free rolling).

**What preflight must report (stages 1 to 6 at defaults).** Stage 3: tree mobility 8. Stage 4: growth vector
(2, 3) for the rolling pair (a differential drive without a caster). Stage 5: C₂. Stage 6: status `passed`
with `findings = ["A-08.no-static-stability"]` and the support segment reported (as for the point-foot biped;
no refusal without a task that requires a static margin); wheel torque to recover from the declared tilt inside the envelope; the
sampled-loop analysis A-22 reports each wheel's current and speed loop with a one-tick spectral radius below 1
at the template's control period. A-22 covers the actuator loops only: the balancing loop's closed-loop
spectral radius and the latency at which it reaches 1 are stated by the baseline controller's synthesis report
(N14b-07: 2 ms control period, stable at 4 ms latency, unstable at 200 ms), which the expectation file records
with the controller's identity and does not attribute to stage 6.

**Known answers.** Fixture: m_b = 10 kg, l = 0.30 m, I_b = 0.20 kg·m², m_w = 2 kg, I_w = 0.01 kg·m²,
r = 0.10 m, so I_b + m_b·l² = 1.1 kg·m². DESIGN: the plan gives no parameters for the balancer, so the
template's defaults (N14b-03) are the one fixture of KA-269; the derivation file of N38b-47 uses the same six
values and reports the same 8.49628 rad/s and the same control 9.1698 rad/s.

| Name | Value | Tolerance |
|---|---|---|
| M = m_w + I_w/r² | 3 kg | exact |
| λ² | 13·10·9.81·0.30/(13·1.1 − 9) = 382.59/5.3 = 72.1868 s⁻² | rel 10⁻⁹ |
| λ | 8.49628 rad/s at g = 9.81 m/s² (the formula's and the derivation file's figure) | rel 10⁻⁹ (formula); the linearized compiled model within 10⁻⁶ of the same closed form evaluated at the magnitude of the model's own gravity vector, never at a literal: at the program's standard gravity 9.80665 m/s² (N13-07) that is 8.49483 rad/s (λ² = 72.16214 s⁻²), 1.7·10⁻⁴ relative from 8.49628, so a comparison with the 9.81 figure would fail for the wrong reason (N14b-03); the referee's free response from rest at a 0.01 rad tilt with zero motor torque follows θ(t)/θ(0) = cosh(λ·t) within 1% for t ≤ 0.2 s (2.826 at 0.2 s for either gravity value), compared with cosh and not with a slope fitted to ln θ, which the stable mode biases about 28% low (N14b-03's tolerance, filed with KA-269) |
| Pole with the wheel inertia term dropped (M = 2 kg) | √(12·29.43/(12·1.1 − 9)) = √(353.16/4.2) = 9.1698 rad/s (the planted defect) | must differ |
| Symmetry blocks | the unstable pair lies in the symmetric block; the yaw block has only zero poles | exact |
| Γ_mech | C₂ | exact |
| Sensitivity fixtures | the largest loop latency, stiction torque and backlash angle at which the baseline controller still balances from a 0.05 rad tilt ("balances": back within 0.01 rad of upright within 2 s, N14b-07's recovery criterion), each found by bisection on a lever of the template and recorded with the controller's identity | recorded values, reproduced within 5% run to run |

**Control variants.** (a) a linearization that drops I_w/r² (pole 9.17 rad/s); (b) an estimator that needs
several steps to converge: the balancer falls in the first-step test, because without control the body passes
0.2 rad from the test's 0.05 rad tilt at t = acosh(4)/λ = 2.06344/8.49628 = 0.243 s (from a 0.01 rad tilt it
takes acosh(20)/λ = 0.434 s; θ grows as θ(0)·cosh(λ·t)), so the test asserts the controller acts from the first
tick; (c) the stiction fixture with stiction disabled passes trivially: the recorded largest-stiction value must
be finite and positive.

## Acceptance criteria

- [ ] The zoo instance equals the template default; `DesignId` pinned and platform-independent.
- [ ] The derivation file (N38b-47) is on record and states the model; the formula reproduces the fixture pole
      8.49628 rad/s at g = 9.81 m/s²; the linearized compiled model reproduces the closed form at its own gravity
      magnitude (8.49483 rad/s at 9.80665 m/s²) within 10⁻⁶; the referee's free response follows cosh(λ·t).
- [ ] Stage 6 reports "no static stability" as a finding; the three sensitivity values are recorded; the
      controls behave as stated.

## Test plan

### Unit tests

- `zoo_balancer_2w` (generated): identity, morphology, symmetry, stages, known answers, controls.
- `zoo_balancer_pole_formula`: 8.49628 rad/s at g = 9.81 m/s², 8.49483 rad/s at 9.80665 m/s²;
  `zoo_balancer_pole_without_wheel_inertia_differs`: 9.1698.
- `zoo_balancer_linearized_pole_model_gravity`: the compiled model's linearized unstable pole against the closed
  form at the magnitude of the model's gravity vector (8.49483 rad/s at standard gravity), within 10⁻⁶ relative;
  the same comparison against the literal 9.81 figure is the planted defect and fails.
- `zoo_balancer_modes_block_structure`: unstable pair in the symmetric block.
- `zoo_balancer_free_response_growth`: θ(t)/θ(0) = cosh(λ·t) within 1% for t ≤ 0.2 s on the referee.
- `zoo_balancer_first_tick_control`: with the baseline controller (N14b-07, 2 ms control period) the tilt from
  0.05 rad decays to within 0.01 rad within 2 s; with the controller's first three ticks suppressed (6 ms) it
  still recovers; with its first 500 ticks suppressed (1 s, by which the uncontrolled body has fallen, N14b-03)
  it falls; between the two the test bisects the largest tolerable count of suppressed ticks and records it
  with the controller's identity.
- `zoo_balancer_sensitivity_bisection`: latency, stiction, backlash.

### End-to-end script and logging

`scripts/e2e/zoo_goldens.sh` (NEG-31) at M3: instance, symmetry, stages 1 to 6; a referee run of the free
response and of the controlled recovery; log lines carry the pole from each of the three sources with the
gravity magnitude each was evaluated at, the growth
ratio at 0.2 s (cosh(1.699) = 2.826), and the three sensitivity values with units.

### Known answers and negative controls

KA-269 (`balancer-2w`: the open-loop pole equals its closed form; the expression belongs to the derivation
file; tier G1 · referee); KA-270 (Γ_mech: C₂). Controls as listed. Determinism: formula D0; referee runs
within their class.

## Considerations

- Until the derivation file is on record the pole row is `blocked: derivation` (the row's `blocked` field of
  NEG-30's format; reported, never counted as passed) and supports no gate; the
  candidate here is a starting point, and the file fixes the model (in particular what counts in I_w: rotor
  inertia reflected through the gear ratio belongs in it).
- The sensitivity values depend on the controller's gains and are tied to its identity; a retuned controller
  re-records them.

## References

Appendix G.4 row 18; §14.10; §15.6; A-12; A-22; KA-269, KA-270; Appendix M.37.
