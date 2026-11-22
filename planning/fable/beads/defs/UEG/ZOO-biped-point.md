---
key: "ZOO-biped-point"
title: "zoo/biped-point.fbot: archetype 5, point-foot biped: C₂, no static stability reported and not refused"
type: "task"
priority: 2
milestone: "M3"
labels: ["m3", "skeleton", "frob-design", "solid", "test", "zoo"]
parent: "WP-38"
deps: ["NEG-30", "N14b-02", "BD-054", "BD-055", "BD-056", "BD-100", "BD-082", "BD-102"]
needs: []
related: ["ROBOT-cassie", "N14b-09", "GB-27", "BD-246", "NEG-31"]
gates: []
plan_refs: ["Appendix G.4 row 5", "§14.10", "§28.4 A-08", "§16 (impact events)", "Appendix M KA-270"]
---
## Background

`biped-point` is a biped with point feet: two legs of three joints each and no ankle or foot surface. It is
the archetype with no static stability at all (two point contacts span a segment, not a polygon) and it is
underactuated in stance (no torque can be applied about the line of the feet). It exists to check three
things. The static stability analysis (A-08) must report "no static stability" as a finding with its margin
and must not refuse the design, because a refusal is only warranted when a task requires a static margin and
none does here. Impact events must be located and resolved, since every step ends in one. And it is the
hardest case for the baseline controller, which must balance dynamically from the first step. Its reference
robot is Cassie (MIT-licensed model). It lands at M3; its baseline controller is Frontier.

## Scope

In: `zoo/biped-point.fbot`, its expectation file and controls. Out: the template and its `@cassie` preset
(N14b-02); A-08 (BD-100); event location and impulses (BD-082); the biped balance controller (N14b-09); the
Cassie import (ROBOT-cassie).

## Technical approach

**Morphology.** Pelvis; `mirror(xz, leg)`; leg = hip abduction, hip pitch, knee; point foot with law
`foot-patch` at zero patch radius (the spin row is masked: a point contact returns the three-dimensional
cone). Actuated 6; floating base 6; n_v = 12; bodies 7; end effectors 2. The template is a tree; it does not
reproduce Cassie's closed-loop leg, which belongs to the import of the reference model.

**Symmetry.** Γ_mech as built: C₂ (the sagittal mirror, exchanging the legs). Default Γ_obs: C₂. Breaker: a
one-sided payload → trivial group.

**What preflight must report.**

| Stage | Status | Must report |
|---|---|---|
| 1 Form | pass | 7 fills; 2 point feet whose torsion radius is zero and not above the foot's own contact radius |
| 2 Mass | pass | Two equal leg subtrees |
| 3 Mechanism | pass | 6 revolute joints, no loops, tree mobility 12 |
| 4 Motion | not-applicable | Legged |
| 5 Structure | pass | Γ_mech = Γ_obs = C₂ |
| 6 Statics and sizing | finding, not refused | "no static stability": the support set is a segment; the margin is reported as the distance from the projected centre of mass to the segment and is zero or negative in area terms; double-support degrees of underactuation = 1; envelope, thermal and bus rows at the standing point pass |

**Known answers.**

| Name | Value | Tolerance |
|---|---|---|
| Γ_mech | C₂ | exact |
| Stage-6 outcome with no task | status `finding` with code-free text "no static stability"; no refusal | exact (verdict) |
| Stage-6 outcome with a task that requires a static margin | refused with FROB-REF-124 (a static margin the task requires is not met) | exact (verdict) |
| Support set | a segment of length equal to the foot separation; area 0 | exact |
| Contact strata, two feet × {open, stick, slip} under C₂ | (3² + 3)/2 = 6 | count |
| Unstable mode of the stance leg as an inverted pendulum (centre of mass at height ℓ above a point foot) | λ = √(g/ℓ); for ℓ = 0.8 m, 3.5018 rad/s | rel 10⁻⁹ against the closed form; within 2% from the linearized model at the reference pose after reducing to the pendulum mode |
| Impact at touchdown | one event located per step; post-impact foot velocity zero under a plastic impact; angular momentum about the impact point conserved to 10⁻⁹ on the referee | exact (event count); rel 10⁻⁹ |

**Control variants.** (a) an A-08 that refuses whenever the support polygon is degenerate: the default must
pass with a finding; (b) `variants/torsion-foot.fbot`: a point foot given a torsion radius above its contact
radius (it silently acquires torsional support) → refused at stage 1 with FROB-REF-121; (c) a build whose
impact resolution leaves a normal velocity at the stance foot: caught by the post-impact velocity row.

## Acceptance criteria

- [ ] The zoo instance equals the template default; `DesignId` pinned and platform-independent.
- [ ] Stage 6 returns a finding and no refusal at defaults, and FROB-REF-124 when a task requires a static
      margin.
- [ ] Every known-answer row passes; the three controls behave as stated.

## Test plan

### Unit tests

- `zoo_biped_point` (generated): identity, morphology, symmetry, stages, known answers, controls.
- `zoo_biped_a08_reports_not_refuses`; `zoo_biped_a08_refuses_124_when_task_requires_margin`.
- `zoo_biped_pendulum_pole`: √(9.81/0.8) = 3.50178 rad/s.
- `zoo_biped_point_foot_masks_spin_row`: the contact solve equals the three-dimensional cone bit for bit.
- `zoo_biped_touchdown_event`: one event per step; momentum about the impact point conserved.

### End-to-end script and logging

`scripts/e2e/zoo_goldens.sh` (NEG-31) at M3: stages 1 to 6 with and without a `synthetic` task that requires a
static margin; the torsion-foot variant; a 2 s referee run from a tilted start to count impact events; log
lines carry the stage-6 finding text, the refusal code in the task case and the event count.

### Known answers and negative controls

KA-270 (Γ_mech as built: C₂). First rejection fixture of the foot family (a point foot that silently acquires
torsional support, refused at stage 1 with FROB-REF-121). Controls as listed. Determinism: static rows D0; the
impact rows within the referee's class.

## Considerations

- The standing row of the catalogue gate exists for this archetype only under its baseline controller, which
  is Frontier; without control it falls within a second, and that is the expected behaviour, not a defect.
- The `@cassie` preset approximates mass only; kinematic parity with Cassie is a property of the imported
  model, never of this template.

## References

Appendix G.4 row 5; §14.10; A-08; FROB-REF-121, FROB-REF-124; KA-270.
