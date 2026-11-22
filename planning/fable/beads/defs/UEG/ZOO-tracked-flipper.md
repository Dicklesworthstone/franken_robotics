---
key: "ZOO-tracked-flipper"
title: "zoo/tracked-flipper.fbot: archetype 12, two tracks with four flippers: C₂ × C₂, belt law, scrub work in a pivot"
type: "task"
priority: 2
milestone: "M3"
labels: ["m3", "skeleton", "traction", "frob-design", "frontier", "test", "zoo"]
parent: "WP-38"
deps: ["NEG-30", "N14b-04", "BD-054", "BD-055", "BD-056", "BD-245", "BD-072", "BD-103", "BD-102"]
needs: []
related: ["BD-365", "N14b-07", "N14a-33", "BD-246", "BD-247", "NEG-31"]
gates: []
plan_refs: ["Appendix G.4 row 12", "§14.10", "§18.9", "§14.13 (rejection fixture, track)", "Appendix M KA-270, KA-282, KA-418"]
---
## Background

`tracked-flipper` is a tracked base with a flipper (a short auxiliary track on a pivot) at each corner. It is
the archetype for contact that is neither a foot nor a rolling wheel. A track is modelled as a set of road
wheels plus a belt: a contact surface with a prescribed tangential velocity and an anisotropic friction
ellipse, with distributed patches. Its defining property is skid steering: a tracked base cannot turn without
slipping, so a turn in place must dissipate energy by scrubbing, and a model in which it costs nothing is
wrong. The flippers add stair capability (they bring the track onto a stair nosing), and a long climb is a
thermal duty test for the drives. The belt law is Frontier: the archetype compiles and passes preflight at M3
and stands and rolls under its controller only at M4, when the law closes on every tier; that split is part of
the gates and not a slip. No reference robot is on file.

## Scope

In: `zoo/tracked-flipper.fbot`, its expectation file and controls. Out: the template (N14b-04); the laws
`belt` and `skid` on all tiers (BD-245); tyre, track and hybrid laws at M4 (BD-365); the skid-steer controller
(N14b-07).

## Technical approach

**Morphology.** Chassis; `mirror(xz, track)`; `mirror(xz, mirror(yz, flipper))`; end effectors of class
`track` with law `belt`. Actuated 6 (two track drives, four flipper pivots); n_v = 12; bodies 7; end effectors
6 (two main tracks, four flipper tracks).

**Symmetry.** Γ_mech as built: C₂ × C₂ (sagittal and transverse mirrors, with flippers at equal angles).
Default Γ_obs: C₂ × C₂. A front pair of flippers raised alone breaks the transverse mirror: Γ_mech = C₂, and
the flipper angle levers are reported as the breaker.

**Skid kinematics.** Under exact lateral no-slip at contact stations a ahead of and a behind the centre,
v_y + a·ω = 0 and v_y − a·ω = 0, hence ω = 0 and v_y = 0: with no slip a tracked base cannot yaw at all. A
pivot turn therefore slips everywhere along the tracks. For a uniformly loaded track pair of contact length ℓ
carrying weight W with lateral friction μ, the scrub moment about the centre is M = μ·W·ℓ/4, and a pivot
through angle φ dissipates E = M·φ (DESIGN derivation: lateral friction μ·W/ℓ per unit length acting at
distance |x| from the centre, ∫ over [−ℓ/2, ℓ/2] of (μ·W/ℓ)·|x| dx = μ·W·ℓ/4).

**What preflight must report (stages 1 to 6 at defaults; no refusal).** Stage 1: 6 `belt` laws with friction
ellipses. Stage 3: tree mobility 12; flipper sweep free of self-collision over its declared range. Stage 4:
base kind `tracked`; growth vector reported for the slipping model with the note "turns only by slipping; the
rolling model does not apply". Stage 5: C₂ × C₂. Stage 6: support margin on the track contact rectangle;
pivot-turn drive torque including scrub inside the envelope; thermal steady state on the declared climb.

**Known answers.** Fixture: W = 300 N, ℓ = 0.50 m, μ = 0.8, pivot of 90°.

| Name | Value | Tolerance |
|---|---|---|
| Yaw rate under exact lateral no-slip (a = 0.25 m) | ω = 0, v_y = 0 | exact |
| Scrub moment | 0.8·300·0.50/4 = 30 N·m | exact |
| Scrub work of a 90° pivot | 30·π/2 = 47.1239 J | rel 10⁻⁹ against the closed form; the simulated pivot under the belt law within 10% and strictly positive |
| Scrub work with the belt law replaced by ideal rolling wheels | 0 (the planted defect) | exact |
| Flipper angle that puts the flipper tip on a nosing of rise h | sin φ = (h − r_t)/ℓ_f for flipper length ℓ_f and track-wheel radius r_t; h = 0.175 m, ℓ_f = 0.30 m, r_t = 0.05 m → φ = 24.62° | abs 0.01° |
| Long climb | the climb length at which the drive winding reaches its limit is finite for the declared slope and equals speed × time-to-limit from the thermal library | rel 0.5% against the library |
| Γ_mech | C₂ × C₂ | exact |

**Control variants.** (a) `variants/zero-scrub.fbot`: tracks modelled as rows of ideal rolling wheels: a turn
in place at zero scrub work on a frictional floor, the first rejection fixture of the track family, caught by
the scrub-work row; (b) a contact cap below the track's patch count: a comparison of this design with a
point-foot design under that cap returns `capacity-biased` and no winner; (c) flippers at unequal angles with
`assert_mech: C2xC2` → FROB-REF-109.

## Acceptance criteria

- [ ] The zoo instance equals the template default; `DesignId` pinned and platform-independent.
- [ ] At M3: compile and stages 1 to 6 pass with no refusal; the no-slip row and the closed-form scrub rows
      hold; dynamic rows are logged `not run: contact law`.
- [ ] At M4: the simulated pivot's scrub work is positive and within 10% of 47.12 J; the zero-scrub variant is
      caught.

## Test plan

### Unit tests

- `zoo_tracked_flipper` (generated): identity, morphology, symmetry, stages, known answers, controls.
- `zoo_tracked_no_slip_forces_zero_yaw`.
- `zoo_tracked_scrub_moment_and_work`: 30 N·m; 47.1239 J.
- `zoo_tracked_flipper_angle`: asin(0.125/0.30) = 24.62°.
- `zoo_tracked_pivot_scrub_positive` (M4): simulated 90° pivot.
- `zoo_tracked_capacity_biased_comparison_refused`.

### End-to-end script and logging

`scripts/e2e/zoo_goldens.sh` (NEG-31): at M3 instance, symmetry and stages 1 to 6; at M4 a referee run of the
pivot with `frob telemetry energy --json` read for dissipated scrub work; log lines carry the scrub moment,
the work in joules against its window, and `not run: contact law` before M4.

### Known answers and negative controls

KA-282 (rejection fixture, track: turning in place at zero scrub work on a frictional floor is caught);
KA-418 (a skid-steer base under exact lateral no-slip has zero yaw rate, so a turn in place costs scrub work);
KA-270 (Γ_mech: C₂ × C₂). Determinism: static rows D0; the pivot within its tier's class.

## Considerations

- The uniform-pressure scrub moment is an idealization; real pressure concentrates under road wheels and the
  10% window on the simulated value reflects that. The closed form is the reference lane's value only.
- The flipper relation is a design heuristic labelled DESIGN; the stair-climb verdict comes from simulation on
  the M4 task family, not from this row.

## References

Appendix G.4 row 12; §14.10; §14.13; §18.9; KA-270, KA-282, KA-418; FROB-REF-109.
