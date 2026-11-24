---
key: "ZOO-omni-3"
title: "zoo/omni-3.fbot: archetype 10, three omni wheels at 120°: D₃, growth vector (3), the 3 × 3 wheel map"
type: "task"
priority: 2
milestone: "M3"
labels: ["m3", "skeleton", "frob-design", "solid", "test", "zoo"]
parent: "WP-38"
deps: ["BD-048", "BD-054", "BD-055", "BD-056", "BD-106", "BD-245", "N14b-03", "N14b-11", "NEG-30", "NEG-31"]
needs: []
related: []
gates: []
plan_refs: ["Appendix G.4 row 10", "§14.10", "C-07", "§13.5", "Appendix M KA-270"]
---
## Background

`omni-3` is a round base on three omni wheels spaced 120° apart, each driving tangentially and sliding freely
along its axle. It is holonomic like the mecanum base, and it is in the catalogue for a different reason: its
symmetry group is D₃, of order 6, with a rotation of odd order. Every mirror-product robot has a group whose
elements are involutions; code paths written for those (pair up left and right, flip a sign) silently fail on
a three-fold rotation, whose real representation on the plane is irreducible and two-dimensional and whose
action on the wheels is a 3-cycle. The archetype also has the smallest square wheel map, three wheels for
three body velocities, so its inverse is exact and checkable by hand. No reference robot is on file. It lands
at M3.

## Scope

In: `zoo/omni-3.fbot` (the template instantiated at its defaults), its expectation file with the full
known-answer table below, the variant file of the parallel-wheel control, and the generated golden tests
(`zoo_omni3…`). Out: the template itself with its manifest, levers, presets and default parameter values, and
the template-level unit tests of the wheel map (`omni3_map_inverse_identity`, `omni3_pure_rotation`), all in
N14b-03: the wheel angles 90°, 210°, 330° and the 3 × 3 map are the template's, defined there and repeated
here. Also out: the roller law (BD-245); stage 4 and A-07 (N14b-11, BD-106); the three-wheel omnidirectional
controller (N14b-07).

## Technical approach

**Morphology.** Hub; `radial(3, z, omni wheel)` with law `roller-wheel` at a roller angle of 90° (rollers
along the rim). Actuated 3; n_v = 9 (floating base 6 plus three wheel spins); bodies 4; end effectors 3.

**Wheel map.** Wheels on a circle of radius R at angles θᵢ = 90°, 210°, 330°; wheel radius r. Rim speed
r·ωᵢ = −sin θᵢ·v_x + cos θᵢ·v_y + R·ω_z. As a matrix M with rows (−1, 0, R), (½, −√3/2, R), (½, √3/2, R).
Inverse: v_x = (r/3)·(−2ω₁ + ω₂ + ω₃), v_y = (r/√3)·(−ω₂ + ω₃), ω_z = (r/(3R))·(ω₁ + ω₂ + ω₃).
det M = (3√3/2)·R, non-zero for every R > 0.

**Symmetry.** Γ_mech as built: D₃ (order 6): the rotation by 120° (wheel 3-cycle) and three reflections, each
through one wheel. Default Γ_obs: D₃. Breaker: a forward sensor leaves the one reflection through the forward
wheel (C₂) if a wheel lies on the forward axis, and nothing otherwise.

**What preflight must report (stages 1 to 6 at defaults; no refusal).** Stage 3: tree mobility 9. Stage 4: map
of rank 3; growth vector (3); class `omnidirectional`. Stage 5: D₃ with generators; the assertion accepted.
Stage 6: three-point support margin (the incircle of the wheel triangle: R/2 from the centre) positive;
wheel torques for translation in the worst direction inside the envelope.

**Known answers.** Fixture: R = 0.20 m, r = 0.05 m.

| Name | Value | Tolerance |
|---|---|---|
| M·M⁻¹ | identity, exactly in rationals extended by √3 | exact |
| Pure rotation ω_z = 1 rad/s | rim speeds (R, R, R) = (0.2, 0.2, 0.2) m/s; wheel rates (4, 4, 4) rad/s | exact |
| Pure translation v_x = 0.1 m/s | rim speeds (−0.1, 0.05, 0.05) m/s; wheel rates (−2, 1, 1) rad/s | exact |
| Pure translation v_y = 0.1 m/s | rim speeds (0, −0.0866025, 0.0866025) m/s | rel 10⁻⁹ |
| det M | (3√3/2)·0.2 = 0.5196152 | rel 10⁻⁹ |
| Growth vector | (3) | exact |
| Γ_mech; its irreducibles | D₃ (order 6); trivial, sign and one of dimension 2 (1 + 1 + 4 = 6) | exact |
| Wheel permutation representation of D₃ | trivial ⊕ the two-dimensional irreducible | exact |
| Support margin | R/2 = 0.10 m for a centred mass | rel 10⁻⁹ |
| Colourings of three wheel slots with k = 10 types under D₃ | (10³ + 2·10 + 3·10²)/6 = 220 | count |

**Control variants.** (a) `variants/two-parallel.fbot`: one wheel rotated so that two drive directions are
parallel: rank 2, growth vector (2, 3), refused at stage 4 with FROB-REF-118; a curvature test on it passes;
(b) the design asserting `C2xC2` → FROB-REF-109 (no such subgroup of D₃); (c) a symmetry routine that pairs
wheels as mirror images only finds C₂ and misses the 3-cycle: the row "order 6" catches it.

## Acceptance criteria

- [ ] The zoo instance equals the template default; `DesignId` pinned and platform-independent.
- [ ] The map and its inverse multiply to the identity exactly; stage 4 reports rank 3 and (3).
- [ ] Γ_mech is D₃ of order 6 with a generator of order 3; the three controls behave as stated.

## Test plan

### Unit tests

- `zoo_omni3` (generated): identity, morphology, symmetry, stages, known answers, controls.
- `zoo_omni3_map_inverse_identity`: exact arithmetic in ℚ(√3).
- `zoo_omni3_rows`: rotation, x-translation and y-translation rows.
- `zoo_omni3_group_order_six_with_three_cycle`.
- `zoo_omni3_permutation_rep_decomposition`: multiplicities (1, 0, 1) for (trivial, sign, 2-dimensional).
- `zoo_omni3_two_parallel_refused_118`.

### End-to-end script and logging

`scripts/e2e/zoo_goldens.sh` (NEG-31) at M3: instance, symmetry, stages 1 to 6, `frob algebra irreps` on the
wheel permutation representation, the parallel-wheel variant; log lines carry the rank, growth vector, group
order and generators, and the refusal with its repair.

### Known answers and negative controls

KA-270 (Γ_mech as built: D₃). Growth vector (3) for an ideal omnidirectional base from the rolling-
distribution construction; control: a curvature test in place of the growth vector on the parallel-wheel
variant. Determinism: exact arithmetic; pinned identities.

## Considerations

- A three-wheel base has no redundancy: a single wheel fault removes controllability, and the archetype has no
  degraded mode to test.
- With the centre of mass off-centre the support margin is the distance to the nearest edge of the wheel
  triangle; the row is evaluated for the default (centred) instance.

## References

Appendix G.4 row 10; §14.10; C-07; §13.5; KA-270; FROB-REF-109, FROB-REF-118.
