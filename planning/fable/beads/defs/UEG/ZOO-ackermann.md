---
key: "ZOO-ackermann"
title: "zoo/ackermann.fbot: archetype 8, car-like base with a steering linkage: C₂, growth vector (2, 3, 4), L/tan δ"
type: "task"
priority: 2
milestone: "M3"
labels: ["m3", "skeleton", "frob-design", "solid", "test", "zoo"]
parent: "WP-38"
deps: ["NEG-30", "N14b-03", "BD-054", "BD-055", "BD-056", "BD-101", "BD-106", "N14b-11"]
needs: []
related: ["SPEC-C-07", "N14b-07", "BD-246", "NEG-31"]
gates: []
plan_refs: ["Appendix G.4 row 8", "§14.10", "C-07", "§28.4 A-01, A-07", "Appendix M KA-266, KA-270"]
---
## Background

`ackermann` is a car-like base: a driven rear axle and a front axle steered through a linkage. It is the first
archetype with a kinematic loop (the steering trapezoid: two steering arms joined by a tie rod), so it is the
fixture for loop closure and mobility analysis (A-01) on a mechanism an engineer can check by hand. It also has
the deepest growth vector of the catalogue, (2, 3, 4): a car has two inputs (drive and steer) on a
four-dimensional configuration space (planar pose plus steer angle), reaches a third direction at bracket depth
two and the fourth, sideways, only at depth three, which is why parallel parking takes many moves. Its turning
radius has a closed form. No reference robot is on file; its scorecard is its own template. It lands at M3.

## Scope

In: `zoo/ackermann.fbot`, its expectation file and controls. Out: the template (N14b-03); mobility by exact
rank (BD-101); the growth vector and maneuverability (BD-106, N14b-11); the steered-base controller (N14b-07).

## Technical approach

**Morphology.** Chassis; rear axle `mirror(xz, driven wheel)`; front axle `mirror(xz, steered wheel)` with two
kingpin joints joined by a tie rod through a `closure`; one steering actuator and the rear drive (two wheel
actuators, or one through a differential, as the template declares). Actuated 3. Generalized velocities: 6
base + 2 rear spins + 2 front spins + the steering loop's coordinates (two kingpins and the tie rod, closed to
one degree of freedom); the manifest records the count for each lowering of the loop. End effectors: 4 wheels
with law `wheel-rolling`.

**Symmetry.** Γ_mech as built: C₂ (sagittal mirror) at zero steer. Default Γ_obs: C₂.

**What preflight must report (stages 1 to 6 at defaults; no refusal).** Stage 3: one loop; steering linkage
mobility 1 by the rank of the loop-closure Jacobian at exact rational sample points (planar four-bar:
3·(4 − 1) − 2·4 = 1); no over-constraint. Stage 4: growth vector (2, 3, 4); minimum turning radius from the
steer limit. Stage 5: C₂. Stage 6: four-wheel support margin; steering torque at standstill inside the
envelope.

**Known answers.** Fixture: wheelbase L = 0.50 m, track w = 0.40 m, steer angle of the virtual centre wheel
δ = 20°.

| Name | Value | Tolerance |
|---|---|---|
| Turning radius R = L/tan δ | 1.373739 m | rel 10⁻⁹ |
| Inner and outer wheel angles | tan δ_i = L/(R − w/2), tan δ_o = L/(R + w/2): 23.07° and 17.63° | abs 0.01° |
| Ackermann condition | cot δ_o − cot δ_i = w/L = 0.8 | rel 10⁻⁹ |
| Linkage mobility | 1 | exact |
| Growth vector | (2, 3, 4) | exact |
| Γ_mech | C₂ | exact |
| Minimum radius at the steer limit δ_max = 35° | 0.50/tan 35° = 0.714074 m | rel 10⁻⁹ |

**Control variants.** (a) `variants/double-tie-rod.fbot`: a second tie rod of a different length between the
same steering arms locks the linkage (mobility 0) → refused with FROB-REF-110 (a closure that over-constrains
the mechanism); (b) `variants/wrong-closure.fbot`: the tie rod attached to the wrong arm (a parallelogram in
place of the trapezoid): mobility is still 1 and the wheels steer in parallel, so cot δ_o − cot δ_i = 0 where
0.8 is required; the turning-geometry row catches what the mobility row cannot; (c) a curvature test used in
place of the growth vector at stage 4 is the wrong test for a wheel layout and is rejected by the stage's
dispatch (wheeled bases get the growth vector).

## Acceptance criteria

- [ ] The zoo instance equals the template default; `DesignId` pinned and platform-independent.
- [ ] Stage 3 reports mobility 1 with the loop named; stage 4 reports (2, 3, 4); the fixture rows hold.
- [ ] The double tie rod is refused with FROB-REF-110; the wrong closure fails the Ackermann condition.

## Test plan

### Unit tests

- `zoo_ackermann` (generated): identity, morphology, symmetry, stages, known answers, controls.
- `zoo_ackermann_radius`: 0.50/tan 20° = 1.373739 m; `zoo_ackermann_inner_outer`: 23.07°, 17.63°;
  `zoo_ackermann_condition`: 0.8.
- `zoo_ackermann_linkage_mobility_one`; `zoo_ackermann_double_tie_rod_refused_110`.
- `zoo_ackermann_growth_vector_2_3_4`: the flag dimensions 2, 3, 4 from the two input fields and their
  brackets at a generic configuration; at δ = 90° (a singular point of the distribution) the report flags
  non-regularity.
- `zoo_ackermann_wrong_closure_parallel_steer`.

### End-to-end script and logging

`scripts/e2e/zoo_goldens.sh` (NEG-31) at M3: instance, symmetry, stages 1 to 6 with the stage-3 mobility and
stage-4 growth vector read from the report, the two variants; log lines carry mobility, the growth vector, the
radius in metres and the wheel angles in degrees.

### Known answers and negative controls

KA-266 (turning radius L/tan δ; growth vector (2, 3, 4); control: a steering linkage with the wrong closure);
KA-270 (Γ_mech: C₂). The radius tolerance is filed here as 10⁻⁹ relative for the closed form. Determinism:
closed forms D0; mobility by exact rank.

## Considerations

- A real trapezoid satisfies the Ackermann condition only approximately across the steer range; the template's
  linkage is parameterized so that the fixture angle satisfies it to the stated tolerance, and the error
  elsewhere is reported, not hidden.
- On the referee, exact loops wait for an upstream loop-constraint item; until then the loop is lowered as a
  compliant equality row and the manifest says which lowering produced each number.

## References

Appendix G.4 row 8; §14.10; C-07; A-01; A-07; KA-266, KA-270; FROB-REF-110.
