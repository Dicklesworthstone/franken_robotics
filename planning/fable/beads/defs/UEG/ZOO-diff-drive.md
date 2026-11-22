---
key: "ZOO-diff-drive"
title: "zoo/diff-drive.fbot: archetype 7, two driven wheels and a caster: growth vector (2, 3), chord step, threshold"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "skeleton", "frob-design", "solid", "test", "zoo"]
parent: "WP-38"
deps: ["NEG-30", "BD-109", "BD-054", "BD-055", "BD-056", "BD-106", "N14b-11", "BD-033", "BD-102"]
needs: []
related: ["ROBOT-stretch3", "GB-05", "BD-117", "BD-086", "SPEC-C-07", "NEG-31"]
gates: []
plan_refs: ["Appendix G.4 row 7", "§14.10", "C-07", "§28.4 A-07", "Appendix F.2 row 2", "Appendix M KA-265, KA-267, KA-270", "Appendix N CX-134"]
---
## Background

`diff-drive` is the simplest wheeled base: two driven wheels on a common axle and a passive caster. It is the
reference nonholonomic body. Its two rolling constraints leave two controllable directions (forward and turn)
and the sideways direction is reached only by a bracket of the two, so its growth vector is (2, 3); that one
fact fixes the cost of small lateral corrections and the base's behaviour in tight interiors. The archetype
also carries two closed-form oracles: the pose update over one step as a constant-twist chord (the SE(2)
exponential), and the push needed to raise a wheel over a threshold lip. The chord formula is the oracle
because the sibling library's planar-drive routine has a reported defect near straight-line motion (FACT to
recheck; an admission test and an upstream work item cover it), so that routine must not be used as the known
answer. The reference robot is the Stretch 3 base (Apache-2.0 model). It lands at M1.

## Scope

In: `zoo/diff-drive.fbot`, its expectation file and controls. Out: the template (BD-109); A-07 (BD-106) and
stage 4 (N14b-11); the wheeled baseline controller (BD-117); the admission test of the sibling routine
(BD-033); thin-barrier and sill fixtures on the referee (BD-086).

## Technical approach

**Morphology.** Chassis; `mirror(xz, driven wheel)`; one passive caster on the mirror plane (a swivel joint
and a rolling joint). Actuated 2; n_v = 10 (floating base 6, two wheel spins, caster swivel and roll); bodies
5; end effectors: 2 wheels with law `wheel-rolling` and 1 caster wheel.

**Symmetry.** Γ_mech as built: C₂ (the sagittal mirror exchanging the wheels; the caster lies on the plane).
Default Γ_obs: C₂.

**What preflight must report (stages 1 to 6 at defaults; no refusal).** Stage 1: 5 fills, wheel laws present.
Stage 2: mirrored wheel subtrees equal. Stage 3: tree mobility 10, no loops. Stage 4: rolling distribution of
rank 2 on the planar pose space; growth vector (2, 3); maneuverability report with the sideways cost. Stage 5:
C₂. Stage 6: three-point support margin positive; wheel torque for the declared ramp and threshold inside the
envelope; bus feasible.

**Known answers.**

| Name | Value | Tolerance |
|---|---|---|
| Growth vector | (2, 3) | exact |
| Chord step | Δp = v·Δt·sinc(δ/2)·(cos(θ + δ/2), sin(θ + δ/2)), δ = ω·Δt, sinc(0) = 1 | rel 10⁻¹² |
| Half turn | v = 1 m/s, ω = π rad/s, Δt = 1 s, θ = 0 → Δp = (0, 2/π) m = (0, 0.6366197724) | rel 10⁻¹² |
| Near-straight case | θ = 32 rad, wheel speeds 1 and 1 + 6 × 10⁻¹³ m/s, track 0.5 m, 400 steps of 1/400 s → translation (0.8342, 0.5514) m, never zero | abs 10⁻⁴ (cos 32 = 0.834223, sin 32 = 0.551427) |
| Sideways cost, half-track 0.20 m | shift 0.01 m: 0.178512 m exact (0.178885 small-angle); shift 0.10 m: 0.553669 m exact at α = 0.722734 rad (0.565685 small-angle) | rel 10⁻⁶ |
| Raised threshold, lip h = 0.012 m | lever arm √(2·r·h − h²): 36.0 mm at r = 0.060 m, 40.69 mm at r = 0.075 m | rel 10⁻⁹ |
| Push over axle load | lever arm/(r − h): 0.750 and 0.6459 | rel 10⁻⁴ |
| Γ_mech | C₂ | exact |

Exact values of the threshold rows: √(2·0.060·0.012 − 0.012²) = √0.001296 = 0.036 m and 0.036/0.048 = 0.75;
√(2·0.075·0.012 − 0.000144) = √0.001656 = 0.0406940 m and 0.0406940/0.063 = 0.645937.

**Control variants.** (a) the sibling planar-drive routine used as the oracle for the near-straight case: it
returns zero translation, and the test asserts the closed form does not (the control of the chord known
answer); (b) a wheel edit judged by one of its two effects: enlarging the wheel from 0.060 to 0.075 m lowers
the push ratio from 0.750 to 0.646 and raises the lever arm from 36.0 to 40.7 mm, and also changes gearing at
the ground (torque per unit force grows with r); the test asserts both numbers are reported together;
(c) a caster placed off the mirror plane with `assert_mech: C2` → FROB-REF-109.

## Acceptance criteria

- [ ] The zoo instance equals the template default; `DesignId` pinned and platform-independent; `n_v` = 10.
- [ ] Stage 4 reports growth vector (2, 3) and the sideways costs above; every other row passes.
- [ ] The chord formula, not the sibling routine, is the oracle in every test of this archetype.

## Test plan

### Unit tests

- `zoo_diff_drive` (generated): identity, morphology, symmetry, stages, known answers, controls.
- `zoo_diff_drive_chord_half_turn`, `zoo_diff_drive_near_straight_not_zero`.
- `zoo_diff_drive_chord_composes`: two half steps equal one full step to 10⁻¹² (constant twist).
- `zoo_diff_drive_threshold_rows`: 0.036 m, 0.75; 0.0406940 m, 0.645937.
- `zoo_diff_drive_sideways_costs`.

### End-to-end script and logging

`scripts/e2e/zoo_goldens.sh` (NEG-31) at M1: instance, symmetry, stages 1 to 6 with the stage-4 report read
for the growth vector and costs; log lines carry the growth vector, the costs in metres, the lever arms in
millimetres and the push ratios.

### Known answers and negative controls

KA-265 (raised threshold: 36.0 mm and 40.7 mm; push over axle load 0.750 and 0.646; control: a wheel edit
judged by one of its two effects, CX-134); KA-267 (the chord step, growth vector (2, 3) and the sideways cost;
control: the sibling planar-drive routine used as the oracle); KA-270 (Γ_mech: C₂). Determinism: closed forms
D0.

## Considerations

- The threshold oracle is quasi-static and assumes no slip at the lip; the dynamic sill fixture on the
  transactional referee step is a separate test and showed at plan time that many static rejections succeed
  dynamically, so the static row is a conservative screen, not a verdict.
- The Stretch 3 base enters by import as itself; the `@stretch3` preset only approximates its mass.

## References

Appendix G.4 row 7; §14.10; C-07; A-07; KA-265, KA-267, KA-270; CX-134; Appendix F.2 row 2.
