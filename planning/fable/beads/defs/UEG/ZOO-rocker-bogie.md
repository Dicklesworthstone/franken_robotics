---
key: "ZOO-rocker-bogie"
title: "zoo/rocker-bogie.fbot: archetype 15, six wheels on a passive rocker-bogie: C₂, differential, mobility 3"
type: "task"
priority: 2
milestone: "M3"
labels: ["m3", "skeleton", "frob-design", "solid", "test", "zoo"]
parent: "WP-38"
deps: ["NEG-30", "N14b-03", "BD-054", "BD-055", "BD-056", "BD-101", "BD-100", "BD-102"]
needs: []
related: ["N14b-07", "BD-246", "BD-368", "NEG-31"]
gates: []
plan_refs: ["Appendix G.4 row 15", "§14.10", "§28.4 A-01, A-08", "Appendix M KA-270"]
---
## Background

`rocker-bogie` is the six-wheeled suspension of planetary rovers: on each side a rocker arm pivots on the body
and carries one wheel at one end and, at the other, a bogie that pivots on the rocker and carries two wheels.
The two rockers are tied through a differential so that when one pitches up the other pitches down by the
same angle and the body sits at their average. Nothing in the suspension is actuated, yet all six wheels stay
on uneven ground. The archetype tests what the other wheeled bodies cannot: passive joints (the mobility
analysis must count them as degrees of freedom and must not count them as actuated), a closure that is a
differential constraint rather than a rigid loop, obstacle climbing without actuated suspension, and static
stability on slopes. No reference robot is on file. It lands at M3.

## Scope

In: `zoo/rocker-bogie.fbot`, its expectation file and controls. Out: the template (N14b-03); mobility (BD-101);
static stability (BD-100); the wheeled controller family (N14b-07); curb and stair task families (BD-368).

## Technical approach

**Morphology.** Body; `mirror(xz, rocker)`; each rocker carries one wheel and a bogie with two wheels; a
differential `closure` between the two rocker pivots (θ_left + θ_right = 0 relative to the body). Six driven
wheels with law `wheel-rolling`. Actuated 6. Generalized velocities: 6 base + 6 wheel spins + suspension
coordinates. Bodies 11 (body, two rockers, two bogies, six wheels); end effectors 6.

**Suspension mobility.** Passive joints: two rocker pivots and two bogie pivots, four coordinates. The
differential removes one: suspension mobility 3 (the common rocker angle and the two bogie angles). With the
body's pitch free on the ground, the differential is what fixes the body's attitude to the mean of the
rockers.

**Symmetry.** Γ_mech as built: C₂ (sagittal mirror). Default Γ_obs: C₂.

**What preflight must report (stages 1 to 6 at defaults; no refusal).**

| Stage | Must report |
|---|---|
| 1 Form | 11 fills; 6 wheel laws; four joints declared `passive` with no actuator card; one `closure` of kind differential with its ratio −1 |
| 3 Mechanism | One closure; suspension mobility 3 by exact rank; actuated count 6; passive count 4 (3 independent); no over-constraint |
| 4 Motion | Base kind `wheeled (skid, six wheels)`: turning needs lateral slip at the corner wheels unless they steer; growth vector for the rolling model reported with that note |
| 5 Structure | Γ_mech = Γ_obs = C₂ |
| 6 Statics and sizing | Six-wheel support polygon; tip-over angles on a side slope and on a fore-aft slope; wheel torque to climb the declared obstacle; envelope, thermal and bus rows pass |

**Known answers.**

| Name | Value | Tolerance |
|---|---|---|
| Suspension mobility | 4 − 1 = 3 | exact |
| Differential law | θ_left = −θ_right relative to the body; body pitch = (rocker_left + rocker_right)/2 in the world | exact on a `synthetic` pose pair (+10°, −10° → body level) |
| Side-slope tip-over angle | atan(half-track / height of the centre of mass); 0.30 m and 0.25 m → 50.194° | abs 0.001° |
| Fore-aft tip-over angle | atan(distance from the centre of mass to the rear wheel contact / height); 0.35 m and 0.25 m → 54.462° | abs 0.001° |
| Wheel load sharing on flat ground (symmetric fixture, bogie pivot midway between its wheels, rocker lever arms a to the single wheel and c to the bogie pivot) | bogie wheels carry equal load; single wheel W_side·c/(a + c), each bogie wheel W_side·a/(2·(a + c)); equal thirds when a = 2c | exact (rational) |
| Climbing a step of height h by a wheel of radius r, quasi-static, with friction μ at the pushing wheels | needs h < r for the kinematic contact and μ above the geometry-dependent bound the analysis reports; reported as a margin, positive at defaults for h = r/2 | sign |
| Γ_mech | C₂ | exact |

**Control variants.** (a) `variants/no-differential.fbot`: the closure removed: mobility 4 and the body pitch
is undetermined on flat ground; stage 3 reports the extra freedom and the standing check flags an
unconstrained body attitude; (b) `variants/actuated-count.fbot`: the passive joints given actuator cards
without the manifest changing: the actuated count becomes 10 and the catalogue's count row (6) catches it;
(c) a rigid tie between the rockers in place of the differential (ratio +1 on a loop that cannot satisfy it on
uneven ground) → refused with FROB-REF-110 when the terrain pose requires opposite angles.

## Acceptance criteria

- [ ] The zoo instance equals the template default; `DesignId` pinned and platform-independent.
- [ ] Stage 3 reports suspension mobility 3, six actuated and four passive joints; the tip-over rows hold.
- [ ] The three controls behave as stated.

## Test plan

### Unit tests

- `zoo_rocker_bogie` (generated): identity, morphology, symmetry, stages, known answers, controls.
- `zoo_rocker_bogie_mobility_three`.
- `zoo_rocker_bogie_differential_levels_body`.
- `zoo_rocker_bogie_tip_angles`: atan(0.30/0.25) = 50.194°; atan(0.35/0.25) = 54.462°.
- `zoo_rocker_bogie_load_sharing_thirds`: a = 2c gives three equal wheel loads per side in exact rationals.
- `zoo_rocker_bogie_passive_not_actuated`.

### End-to-end script and logging

`scripts/e2e/zoo_goldens.sh` (NEG-31) at M3: instance, symmetry, stages 1 to 6 with the stage-3 mobility and
joint counts and the stage-6 tip-over angles; the three variants; log lines carry mobility, counts, angles in
degrees and wheel loads in newtons.

### Known answers and negative controls

KA-270 (Γ_mech: C₂). Controls as listed. Determinism: exact rank and rational statics, D0.

## Considerations

- Exact loop closures on the referee wait for an upstream loop-constraint item; until then the differential is
  a compliant equality row on the fast tiers and the manifest records the generalized-velocity count for each
  lowering.
- The step-climbing row is quasi-static and reported as a margin; the verdict on a real obstacle course comes
  from the M4 task family.

## References

Appendix G.4 row 15; §14.10; A-01; A-08; KA-270; FROB-REF-110.
