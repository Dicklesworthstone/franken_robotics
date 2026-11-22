---
key: "ZOO-tri-wheel-stair"
title: "zoo/tri-wheel-stair.fbot: archetype 14, two tri-wheel clusters: C₂ outside, C₃ or D₃ inside, riser and tread"
type: "task"
priority: 2
milestone: "M3"
labels: ["m3", "skeleton", "frob-design", "solid", "test", "zoo"]
parent: "WP-38"
deps: ["NEG-30", "N14b-04", "BD-054", "BD-055", "BD-056", "BD-048", "BD-102", "BD-101"]
needs: ["§33: the admission test of the sibling's tri-wheel cluster geometry module before it is used as an oracle"]
related: ["BD-368", "N14b-07", "BD-246", "NEG-31"]
gates: []
plan_refs: ["Appendix G.4 row 14", "§14.10", "§13.6", "Appendix M KA-270"]
---
## Background

`tri-wheel-stair` is a stair-climbing chassis with a cluster of three wheels on each side. Each cluster is a
carrier that rotates about a central axle with three wheels at its tips; on flat ground two wheels of a
cluster roll, and at a step the carrier rotates and "walks" the cluster up. The archetype has two purposes.
Geometric: whether a cluster climbs a given stair is a relation between the cluster's size and the stair's
riser and tread, a quick check an agent should see before simulating. Algebraic: its symmetry is nested, a
three-fold group inside each cluster and a mirror that exchanges the clusters, so the compiler must report a
structured group (a wreath-like product) and not just the outer mirror. FACT: the sibling physics library has
a tri-wheel cluster geometry module; it may serve as the oracle for the stepping kinematics only after its own
admission test. No reference robot is on file. It lands at M3; stair climbing as a task family is M4.

## Scope

In: `zoo/tri-wheel-stair.fbot`, its expectation file and controls. Out: the template (N14b-04); finite groups
and wreath products (BD-048, BD-049 as used by the symmetry analysis BD-102); the stair task family (BD-368).

## Technical approach

**Morphology.** Chassis; `mirror(xz, cluster)`; cluster = carrier joint (about the cluster axle) with
`radial(3, axle, wheel)`; wheels with law `wheel-rolling`; a rear caster or skid as the template declares.
Actuated 4 (two carrier drives, two cluster wheel drives through a shared train); generalized velocities 6
base + 2 carriers + 6 wheel spins = 14 (wheels of one cluster are geared together in the default transmission,
which the manifest records); bodies 9 plus the rear support; end effectors 6 wheels.

**Symmetry.** Γ_mech as built: the outer C₂ (sagittal mirror exchanging the clusters); inside each cluster C₃
(wheels handed by their drive train) or D₃ (unhanded). As one group acting on the six wheels: C₃ ≀ C₂ of order
3²·2 = 18, or D₃ ≀ C₂ of order 6²·2 = 72 when the clusters may turn independently; with both carriers at the
same angle and locked together the group that fixes the pose is the diagonal subgroup, of order 6 (C₃ case)
or 12 (D₃ case). The expectation file records the generators (one cluster's 3-cycle, the swap) and the three
orders with the condition for each.

**Cluster geometry (DESIGN relation).** Wheels of radius r_w on a carrier circle of radius ρ: adjacent wheel
centres are √3·ρ apart. To place the next wheel on the next tread while the lower wheel sits against the
riser, the spacing should match the step diagonal √(h² + t²) for riser h and tread t; the template exposes ρ
as a lever. Necessary clearances: r_w < t (the wheel fits on a tread) and the carrier must not strike the
nosing, ρ·cos 30° > r_w (the chord between two wheel centres clears the wheel on the tread).

**What preflight must report (stages 1 to 6 at defaults; no refusal).** Stage 3: tree mobility as recorded;
carrier rotation free of self-collision over 360°. Stage 4: base kind `wheeled (cluster)`; growth vector (2, 3)
on flat ground with both lower wheels rolling (a differential drive). Stage 5: the nested group with its
orders. Stage 6: support margin on four ground wheels plus the rear support; carrier torque to lift the
chassis by one riser inside the envelope: τ ≥ (W_side)·ρ·cos 30° at the worst carrier angle, for the weight
W_side carried by one cluster.

**Known answers.** Fixture: riser h = 0.175 m, tread t = 0.260 m.

| Name | Value | Tolerance |
|---|---|---|
| Step diagonal | √(0.175² + 0.260²) = 0.313409 m | rel 10⁻⁹ |
| Carrier radius matching it | ρ = 0.313409/√3 = 0.180947 m | rel 10⁻⁹ |
| Spacing at ρ = 0.181 m | √3·0.181 = 0.313501 m (0.03% above the diagonal) | rel 10⁻⁹ |
| Group orders | 18 or 72 (independent clusters, C₃ or D₃ inside); 6 or 12 (clusters locked in phase) | exact |
| Flat-ground growth vector | (2, 3) | exact |
| Carrier torque to lift, W_side = 150 N, ρ = 0.181 m | 150·0.181·cos 30° = 23.51 N·m | rel 10⁻⁴ |
| Γ_mech outer | C₂ | exact |

**Control variants.** (a) a symmetry routine that reports only the outer mirror (order 2): the nested-order
row catches it; (b) `variants/small-cluster.fbot`: ρ = 0.12 m (spacing 0.208 m, far below the diagonal): the
geometry row flags "cluster cannot span the step" as a finding with the lever to change; (c) the sibling
geometry module used as an oracle before admission: the row is recorded `blocked: admission`, never `pass`.

## Acceptance criteria

- [ ] The zoo instance equals the template default; `DesignId` pinned and platform-independent.
- [ ] The nested group is reported with generators and the orders above; the geometry rows hold.
- [ ] The small-cluster variant produces the finding; the oracle row is `blocked: admission` until admitted.

## Test plan

### Unit tests

- `zoo_tri_wheel` (generated): identity, morphology, symmetry, stages, known answers, controls.
- `zoo_tri_wheel_group_nested`: the outer generator swaps clusters, the inner generator cycles wheels; orders
  18 and 6 for the handed default.
- `zoo_tri_wheel_spacing`: 0.313501 m against 0.313409 m.
- `zoo_tri_wheel_lift_torque`: 23.51 N·m.
- `zoo_tri_wheel_flat_growth_vector`.

### End-to-end script and logging

`scripts/e2e/zoo_goldens.sh` (NEG-31) at M3: instance, symmetry, stages 1 to 6, the small-cluster variant;
log lines carry the group structure and orders, the spacing and diagonal in metres, the lift torque, and
`blocked: admission` for the oracle row.

### Known answers and negative controls

KA-270 (Γ_mech: C₂ with C₃ or D₃ in each cluster). Controls as listed. Determinism: closed forms and exact
group computations, D0.

## Considerations

- The spacing relation is a design heuristic, labelled DESIGN; the stepping oracle is the sibling geometry
  module after admission, and the stair-climb verdict comes from the M4 task family.
- The orders 18 and 72 count symmetries of the mechanism with clusters free; campaign quotienting uses the
  pose-fixing subgroup that applies to the task.

## References

Appendix G.4 row 14; §14.10; §13.6; KA-270.
