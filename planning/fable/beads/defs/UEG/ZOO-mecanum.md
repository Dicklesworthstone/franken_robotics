---
key: "ZOO-mecanum"
title: "zoo/mecanum.fbot: archetype 9, four mecanum wheels: C₂ × C₂, growth vector (3), parallel rollers are rank 2"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "skeleton", "frob-design", "solid", "test", "zoo"]
parent: "WP-38"
deps: ["NEG-30", "BD-109", "BD-054", "BD-055", "BD-056", "BD-106", "N14b-11", "BD-102"]
needs: []
related: ["ROBOT-tiago", "BD-245", "BD-117", "SPEC-C-07", "NEG-31"]
gates: []
plan_refs: ["Appendix G.4 row 9", "§14.10", "§14.11 (stage 4 needs both tests)", "§13.3", "C-07", "Appendix M KA-268, KA-270", "Appendix N CX-131"]
---
## Background

`mecanum` is a rectangular base on four mecanum wheels, each a wheel whose rim carries free rollers at an
angle to the axle. With the rollers arranged in the usual X pattern the base can translate in any direction
and rotate: it is holonomic, with growth vector (3). The archetype exists for three reasons. Its contact law
is anisotropic (free along the roller axis, gripping across it), a law class no point foot exercises. It tests
reflection parity: a mirror maps a roller angle α to −α, so a design built with nested `mirror` constructors
gets the working pattern only if the constructor flips the angle; forgetting the flip yields four parallel
roller axes. And that defect is instructive: with all rollers parallel the base loses one direction, its
growth vector drops to (2, 3), and a curvature test still passes, which is why preflight stage 4 runs the
growth vector on wheel layouts and curvature only on shape-changers. The reference robot is the TIAGo omni
base (Apache-2.0 model). It lands at M1; its rolling row waits for the roller law on the referee.

## Scope

In: `zoo/mecanum.fbot`, its expectation file and controls. Out: the template (BD-109); the `roller-wheel`
contact law on all tiers (BD-245); stage 4 (N14b-11) and A-07 (BD-106); the wheeled controller (BD-117).

## Technical approach

**Morphology.** Chassis; `mirror(xz, mirror(yz, mecanum wheel))` with law `roller-wheel` and a roller angle.
Actuated 4; n_v = 10 (floating base 6 plus four wheel spins); bodies 5; end effectors 4.

**Twist-to-wheel map (DESIGN derivation under ideal roller no-slip).** Wheel i at (xᵢ, yᵢ), radius r, roller
axis at angle αᵢ from the rolling direction, tᵢ = tan αᵢ: ωᵢ = (1/r)·(v_x + tᵢ·v_y + (tᵢ·xᵢ − yᵢ)·ω_z). The
working pattern is tᵢ = −sign(xᵢ·yᵢ) (α = ±45°).

**Symmetry.** Γ_mech as built: C₂ × C₂ (both mirrors, each acting on wheels by a permutation and on roller
angles by a sign flip that the pattern absorbs). Default Γ_obs: C₂ × C₂.

**What preflight must report (stages 1 to 6 at defaults; no refusal).** Stage 1: 4 `roller-wheel` laws with
their angles. Stage 4: twist-to-wheel map of rank 3; growth vector (3); class `omnidirectional` confirmed.
Stage 5: C₂ × C₂, with the reflection action on roller angles reported. Stage 6: four-wheel support margin;
wheel torques for pure lateral translation inside the envelope (lateral motion costs more wheel torque per
unit force than forward motion at 45° rollers).

**Known answers.** Fixture: r = 0.05 m, wheels at (±0.25, ±0.20) m, order FL (+, +), FR (+, −), RL (−, +),
RR (−, −).

| Name | Value | Tolerance |
|---|---|---|
| t for (FL, FR, RL, RR) | (−1, +1, +1, −1) | exact |
| Wheel rates for v_x = 0.1 m/s | (2, 2, 2, 2) rad/s | exact |
| Wheel rates for v_y = 0.1 m/s | (−2, 2, 2, −2) rad/s | exact |
| Wheel rates for ω_z = 1 rad/s | (tᵢ·xᵢ − yᵢ)/r = (−9, 9, −9, 9) rad/s | exact |
| Rank of the 4 × 3 map; growth vector | 3; (3) | exact |
| Consistency condition on wheel rates (the map's left null vector) | ω_FL + ω_FR − ω_RL − ω_RR = 0 | exact |
| All tᵢ = 1 (parallel rollers) | rank 2; null twist (v_x, v_y, ω_z) ∝ (−1, 1, 0): the base slides freely along it; growth vector (2, 3) | exact |
| Γ_mech | C₂ × C₂ | exact |

**Control variants.** (a) `variants/parallel-rollers.fbot` (the build with the mirror's parity flip disabled):
refused at stage 4 with FROB-REF-118 (the base cannot produce a motion the task class requires) against the
declared class, with the missing direction and the roller edit that restores it as the repair; (b) the same
variant under a curvature test: it passes, as a differential drive does, which shows curvature is the wrong
test for a wheel layout; (c) one wheel's roller angle set to 0° (a plain wheel): rank stays 3 for this
geometry but the lateral authority changes, and the stage-6 lateral-torque row changes by the recorded amount.

## Acceptance criteria

- [ ] The zoo instance equals the template default; `DesignId` pinned and platform-independent.
- [ ] The nested-mirror template yields tᵢ = −sign(xᵢ·yᵢ); the map rows and rank hold exactly.
- [ ] The parallel-roller variant is refused with FROB-REF-118 at stage 4 and passes a curvature test.

## Test plan

### Unit tests

- `zoo_mecanum` (generated): identity, morphology, symmetry, stages, known answers, controls.
- `zoo_mecanum_map_values`: the three rows of wheel rates above.
- `zoo_mecanum_null_vector`: (1, 1, −1, −1) annihilates the map's image.
- `zoo_mecanum_parallel_rollers_rank_two`: rank 2, null twist (−1, 1, 0).
- `zoo_mecanum_mirror_flips_roller_angle`.
- `zoo_mecanum_inverse_map`: the pseudo-inverse recovers (v_x, v_y, ω_z) = (0.1, −0.05, 0.3) from its wheel
  rates to 10⁻¹².

### End-to-end script and logging

`scripts/e2e/zoo_goldens.sh` (NEG-31) at M1: instance, symmetry, stages 1 to 6 with the stage-4 rank and
growth vector, the parallel-roller variant expecting FROB-REF-118; log lines carry the map's rank, the growth
vector, the refusal code and its repair.

### Known answers and negative controls

KA-268 and CX-131 (mecanum base with all four roller axes parallel: twist-to-wheel map of rank 2; growth
vector (2, 3) where an omnidirectional base must have (3); stage 4 refuses against the declared class; a
curvature test on the same base passes; control: curvature used as the wheel-layout test); KA-270 (Γ_mech:
C₂ × C₂). Determinism: exact rational arithmetic for the map.

## Considerations

- If the anisotropic roller law is not available on the referee at M1, the archetype still compiles and its
  static known answers hold; its rolling row waits for the law.
- Ideal roller no-slip is a regime; on a film or a soft floor the kinematic map is withdrawn and the contact
  law governs.

## References

Appendix G.4 row 9; §14.10; §14.11; §13.3; C-07; KA-268, KA-270; CX-131; FROB-REF-118.
