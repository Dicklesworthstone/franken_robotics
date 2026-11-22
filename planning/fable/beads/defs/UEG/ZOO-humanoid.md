---
key: "ZOO-humanoid"
title: "zoo/humanoid.fbot: archetype 6, humanoid with arms and waist: 29 actuated, n_v = 35, C₂, coupled transmissions"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "skeleton", "frob-design", "solid", "test", "zoo"]
parent: "WP-38"
deps: ["NEG-30", "BD-109", "BD-054", "BD-055", "BD-056", "BD-062", "BD-074", "BD-104", "BD-102"]
needs: []
related: ["ROBOT-g1", "ROBOT-fsim-g1", "ROBOT-h1", "ROBOT-booster-t1", "ROBOT-apollo", "ROBOT-fourier-n1", "ROBOT-op3", "ROBOT-talos", "N14b-34", "GB-10", "GB-41", "BD-007", "NEG-31"]
gates: []
plan_refs: ["Appendix G.4 row 6", "§14.10", "§15.9", "§14.15 GATE-SKEL-07", "§14.17", "Appendix M KA-270", "Appendix F.2 (G1 catalogue)"]
---
## Background

`humanoid` is the upper end of what the dense per-world dynamics must handle: 29 actuated joints and 35
generalized velocities, the "H" workload of the performance tables. It exercises four things at once. The
dense mass matrix at its largest planned size; transmissions in which two actuators drive two joints through a
coupling (ankles with pitch and roll through a pair of linear actuators, a waist with coupled axes), so that
torque limits live on actuators and not on joints; self-collision over large joint ranges (arms across the
torso, legs across each other); and import parity against a public model. Its reference robots are the
Unitree G1 and H1; the sibling physics library also ships a G1 builder. It lands at M1 with the first
templates because the referee smoke test of M0 already runs a G1.

A fact about that builder shapes this bead. FACT (read in the sibling's source): its G1 catalogue keeps 30
links whose masses sum to 31.96314202 kg and omits fixed attachments with nonzero mass, while the 38-link
source sums to 33.34014202 kg. The builder's model is therefore a reduced profile (`g1-reduced-30`) and never
the parity reference; mass dropped at import is refused unless a `reduced` block declares it.

## Scope

In: `zoo/humanoid.fbot`, its expectation file and controls. Out: the template and its `@g1`, `@h1` presets
(BD-109); coupling transmissions (BD-074); self-collision analysis (BD-104); reduced models (N14b-34); the
robot imports (ROBOT-g1, ROBOT-fsim-g1, ROBOT-h1 and the import-parity-only humanoids).

## Technical approach

**Morphology.** Pelvis, waist, torso, head; `mirror(xz, arm)`, `mirror(xz, leg)`; coupled ankle and waist
transmissions; feet with law `foot-patch` and a finite patch (spin row active). Actuated 29; floating base 6;
n_v = 35; bodies 30; end effectors 2 feet (hands are plain links at M1).

**Symmetry.** Γ_mech as built: C₂ (sagittal mirror). Default Γ_obs: C₂. Breakers: a payload in one hand →
trivial; a head camera on the mirror plane leaves C₂ intact (a sensor on the plane and aligned with it breaks
nothing), which is itself a row.

**What preflight must report (stages 1 to 6 at defaults; no refusal).**

| Stage | Must report |
|---|---|
| 1 Form | 30 fills, of which the torso and covers are `{shell: t}`; 2 foot laws; 29 actuators; coupling transmissions with their matrices |
| 2 Mass | Total mass and per-limb roll-up; mirrored limbs bit-identical; every pseudo-inertia positive definite |
| 3 Mechanism | 29 actuated revolute joints; tree mobility 35; coupled transmissions are not kinematic loops and add no closure; self-collision pairs over declared ranges listed with their minimum clearance, none negative at the reference pose |
| 4 Motion | not-applicable |
| 5 Structure | Γ_mech = Γ_obs = C₂; port power balance closed through the coupling matrices |
| 6 Statics and sizing | Positive support margin on two flat feet; actuator-space torque limits mapped to joint space as a zonotope per coupled pair; standing, crouching and stepping-up points inside the envelope; thermal and bus rows pass |

**Known answers.**

| Name | Value | Tolerance |
|---|---|---|
| Γ_mech | C₂ | exact |
| n_v; bodies | 35; 30 | exact |
| Mass window of the `@g1` preset | within 15% of 33.34014202 kg: 28.339 kg to 38.341 kg | exact (bounds) |
| Sibling builder roll-up (FACT, re-read at the pin) | 30 links, 31.96314202 kg: the reduced profile, 1.377 kg (4.13%) below the 38-link source | exact (as recorded) |
| Coupled pair, torque map | for coupling matrix A (2 × 2, joint torque = Aᵀ·actuator force), the joint-torque set of |fᵢ| ≤ f_max is the parallelogram with vertices Aᵀ·(±f_max, ±f_max); for A = [[1, 1], [1, −1]]·r it is a diamond with vertices (±2·r·f_max, 0) and (0, ±2·r·f_max) | exact |
| Dense per-world bytes at C = 64, R = 320 | 90,704 before state (cache 9,700; 12·n² 14,700; Jacobian 44,800; rows 15,360; contacts 6,144) | count |
| Shell torso against solid (illustration on a 0.42 × 0.20 × 0.10 m box, 2.5 mm wall, 2,700 kg/m³) | 1.9227375 kg against 22.68 kg | exact |

**Control variants.** (a) `variants/solid-torso.fbot`: the torso's fill changed from shell to `solid`: the
preset's mass leaves its window (the catalogue's mass control); (b) `variants/joint-limits-on-coupled.fbot`:
torque limits written on the ankle joints instead of the actuators: the envelope differs from the zonotope
(a box in joint space) and the test asserts the two differ at a diagonal load; (c) a `g1-reduced-30` import
used as parity reference without a `reduced` block → refused (mass dropped at import).

## Acceptance criteria

- [ ] The zoo instance equals the template default; `DesignId` pinned and platform-independent; `n_v` = 35.
- [ ] Every preflight and known-answer row passes; self-collision pairs are listed with clearances.
- [ ] The three controls behave as stated; the reduced-profile fact is recorded with its source path and pin.

## Test plan

### Unit tests

- `zoo_humanoid` (generated): identity, morphology, symmetry, stages, known answers, controls.
- `zoo_humanoid_mass_window`: 28.339 ≤ mass(`@g1`) ≤ 38.341 kg.
- `zoo_humanoid_coupling_zonotope`: the diamond's vertices; a box-limited model admits (2·r·f_max, 2·r·f_max)
  which the zonotope excludes.
- `zoo_humanoid_camera_on_plane_keeps_c2`.
- `zoo_humanoid_self_collision_pairs_listed`.
- `zoo_humanoid_bytes_per_world`: 90,704.

### End-to-end script and logging

`scripts/e2e/zoo_goldens.sh` (NEG-31) at M1: instance, show (model and mass sections), symmetry, stages 1 to 6,
the three variants; `frob sim estimate --workload H` for the byte row at M2; log lines carry mass in kg with
its window, `n_v`, group, clearances and the refusal for the reduced import.

### Known answers and negative controls

KA-270 (Γ_mech: C₂). The mass window is the humanoid row of the catalogue's mass gate (within 15% of the mass
on file for the reference robot). Controls as listed. Determinism: pinned identities; exact rational mass
rows.

## Considerations

- The 38-link source mass is reported and to be re-read from the primary source before a gate cites it; until
  then the window row carries that caveat.
- Hands, dexterous or otherwise, are outside the eighteen archetypes and off the critical path.

## References

Appendix G.4 row 6; §14.10; §14.15 (GATE-SKEL-07); §14.17; §15.9; KA-270; Appendix F.2.
