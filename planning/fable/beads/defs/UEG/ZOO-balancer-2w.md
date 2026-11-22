---
key: "ZOO-balancer-2w"
title: "zoo/balancer-2w.fbot: archetype 18, two-wheeled inverted pendulum: C₂, the open-loop pole in closed form"
type: "task"
priority: 2
milestone: "M3"
labels: ["m3", "skeleton", "reflex", "frob-design", "solid", "test", "zoo"]
parent: "WP-38"
deps: ["NEG-30", "N14b-03", "BD-054", "BD-055", "BD-056", "BD-107", "BD-114", "BD-084", "BD-108"]
needs: ["§38.6: the derivation file for the balancer's open-loop pole, which the specification names and does not write"]
related: ["N14b-07", "N16c-42", "NEG-10", "BD-246", "NEG-31"]
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

In: `zoo/balancer-2w.fbot`, its expectation file and controls; the candidate derivation text for the pole.
Out: the template (N14b-03); modes with symmetry block-diagonalization (BD-107); the invariant filter
(BD-114); joint stiction and backlash rows (BD-084); sampled-loop stability (BD-108); the balancing
controller (N14b-07).

## Technical approach

**Morphology.** Body; `mirror(xz, driven wheel)`; no caster. Actuated 2; n_v = 8 (floating base 6 plus two
wheel spins); bodies 3; end effectors 2 wheels with law `wheel-rolling`.

**Symmetry.** Γ_mech as built: C₂ (sagittal mirror exchanging the wheels). Default Γ_obs: C₂. Under it the
linearized dynamics split into a symmetric block (pitch and forward motion) and an antisymmetric block (yaw).

**Candidate closed form for the pole (DESIGN; to be fixed in `docs/derivations/balancer-2w-pole.md`).** Planar
model: body of mass m_b with centre of mass at height l above the axle and pitch inertia I_b about its centre
of mass; two wheels of total mass m_w, total axle inertia I_w and radius r, rolling without slip, zero motor
torque. With x the axle position and θ the pitch from upright, and M = m_w + I_w/r², the linearized equations
are (M + m_b)·ẍ + m_b·l·θ̈ = 0 and m_b·l·ẍ + (I_b + m_b·l²)·θ̈ − m_b·g·l·θ = 0. Eliminating ẍ:
λ² = (M + m_b)·m_b·g·l / ((M + m_b)·(I_b + m_b·l²) − m_b²·l²), and the open-loop poles are ±λ plus a double
pole at zero (free rolling).

**What preflight must report (stages 1 to 6 at defaults).** Stage 3: tree mobility 8. Stage 4: growth vector
(2, 3) for the rolling pair (a differential drive without a caster). Stage 5: C₂. Stage 6: "no static
stability" as a finding with the support segment (as for the point-foot biped; no refusal without a task that
requires a static margin); wheel torque to recover from the declared tilt inside the envelope; the
sampled-loop analysis at the template's control rate reports a spectral radius below 1 for the baseline gains
and names the latency at which it reaches 1.

**Known answers.** Fixture: m_b = 10 kg, l = 0.30 m, I_b = 0.20 kg·m², m_w = 2 kg, I_w = 0.01 kg·m²,
r = 0.10 m.

| Name | Value | Tolerance |
|---|---|---|
| M = m_w + I_w/r² | 3 kg | exact |
| λ² | 13·10·9.81·0.30/(13·1.1 − 9) = 382.59/5.3 = 72.1868 s⁻² | rel 10⁻⁹ |
| λ | 8.49628 rad/s | rel 10⁻⁹ (formula); the linearized compiled model within 10⁻⁶; the referee's free response from a 0.01 rad tilt grows at that rate within 1% over the first 0.2 s |
| Pole with the wheel inertia term dropped (M = 2 kg) | √(12·29.43/(12·1.1 − 9)) = √(353.16/4.2) = 9.1698 rad/s (the planted defect) | must differ |
| Symmetry blocks | the unstable pair lies in the symmetric block; the yaw block has only zero poles | exact |
| Γ_mech | C₂ | exact |
| Sensitivity fixtures | the largest loop latency, stiction torque and backlash angle at which the baseline controller still balances from a 0.05 rad tilt, each found by bisection on a lever of the template and recorded with the controller's identity | recorded values, reproduced within 5% run to run |

**Control variants.** (a) a linearization that drops I_w/r² (pole 9.17 rad/s); (b) an estimator that needs
several steps to converge: the balancer falls in the first-step test (the body passes 0.2 rad within 0.25 s
without control: θ grows as cosh(λ·t), and 0.01·cosh(8.496·0.435) ≈ 0.2), so the test asserts the controller
acts from the first tick; (c) the stiction fixture with stiction disabled passes trivially: the recorded
largest-stiction value must be finite and positive.

## Acceptance criteria

- [ ] The zoo instance equals the template default; `DesignId` pinned and platform-independent.
- [ ] The derivation file exists and states the model; the fixture pole 8.49628 rad/s is reproduced by the
      formula, by the linearized compiled model and by the referee's free response.
- [ ] Stage 6 reports "no static stability" as a finding; the three sensitivity values are recorded; the
      controls behave as stated.

## Test plan

### Unit tests

- `zoo_balancer_2w` (generated): identity, morphology, symmetry, stages, known answers, controls.
- `zoo_balancer_pole_formula`: 8.49628 rad/s; `zoo_balancer_pole_without_wheel_inertia_differs`: 9.1698.
- `zoo_balancer_modes_block_structure`: unstable pair in the symmetric block.
- `zoo_balancer_free_response_growth`: θ(t)/θ(0) = cosh(λ·t) within 1% for t ≤ 0.2 s on the referee.
- `zoo_balancer_first_tick_control`: with the baseline controller the tilt from 0.05 rad decays; with the
  controller's first three ticks suppressed at a 4 ms tick it still recovers, and with 60 ticks suppressed it
  falls (the test records the largest tolerable count).
- `zoo_balancer_sensitivity_bisection`: latency, stiction, backlash.

### End-to-end script and logging

`scripts/e2e/zoo_goldens.sh` (NEG-31) at M3: instance, symmetry, stages 1 to 6; a referee run of the free
response and of the controlled recovery; log lines carry the pole from each of the three sources, the growth
ratio at 0.2 s (cosh(1.699) = 2.826), and the three sensitivity values with units.

### Known answers and negative controls

KA-269 (`balancer-2w`: the open-loop pole equals its closed form; the expression belongs to the derivation
file; tier G1 · referee); KA-270 (Γ_mech: C₂). Controls as listed. Determinism: formula D0; referee runs
within their class.

## Considerations

- Until the derivation file is on record the pole row is `blocked: derivation` and supports no gate; the
  candidate here is a starting point, and the file fixes the model (in particular what counts in I_w: rotor
  inertia reflected through the gear ratio belongs in it).
- The sensitivity values depend on the controller's gains and are tied to its identity; a retuned controller
  re-records them.

## References

Appendix G.4 row 18; §14.10; §15.6; A-12; A-22; KA-269, KA-270; Appendix M.37.
