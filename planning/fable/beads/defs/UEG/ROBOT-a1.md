---
key: "ROBOT-a1"
title: "assets/robots/a1: Unitree A1 from MuJoCo Menagerie (BSD-3): parity fixture and baseline for `quad-mammal`"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "skeleton", "frob-io", "solid", "test", "asset", "robot"]
parent: "WP-08"
deps: ["NEG-15", "BD-066", "BD-067", "N14b-20"]
needs: []
related: ["ZOO-quad-mammal", "BD-109", "N14b-24", "BD-089", "ROBOT-go1", "ROBOT-go2"]
gates: []
plan_refs: ["Appendix G.1", "§14.10", "§14.15 GATE-SKEL-01, GATE-SKEL-07", "Appendix H.3"]
---
## Background

The Unitree A1 is the oldest of the three Unitree quadrupeds on file and the robot of much of the early
published work on learned legged locomotion; older training recipes and reward settings that the project may
want to reproduce for comparison were written for it. Its public MuJoCo model in the Menagerie collection is
on file under BSD-3 (FACT from the research pass of 2026-11-21; re-checked at admission). It serves the
`quad-mammal` archetype as an imported parity fixture and a benchmark baseline, and is never the source of a
Reality claim. Having three robots of one maker with near-identical topology is useful in its own right: the
three imports must yield the same design term up to parameters, which tests that the importer's output is a
function of the model's content and not of incidental file layout.

## Scope

In: `assets/robots/a1/` (manifest, licence text, pinned source record, import golden, loss-report golden,
validation golden) and the `quad-mammal@a1` mass-on-file record; the cross-robot structural comparison with
`go1` and `go2`. Out: the shared layout and checks (NEG-15); the importer (BD-066); oracle capture (BD-067).

## Technical approach

**Source record.** MuJoCo Menagerie at the project's single pinned commit; directory expected `unitree_a1/`
(recalled, mark M: confirm at the pin). Licence BSD-3-Clause: vendoring allowed with notice, licence text and
the no-endorsement clause.

**Expected facts (recalled, mark M; the pinned files govern).** 12 actuated revolute joints; 13 bodies;
n_v = 18; total mass on the order of 12 kg; one actuator per joint; sphere feet.

**Import path.** `frob design import --format mjcf assets/robots/a1/source/<entry> --out
assets/robots/a1/import.fbot --json`. Expected loss-report entries (HYPOTHESIS until read): as for the other
Unitree quadrupeds (visual meshes; fitted collision primitives with recorded error; unmapped solver and
contact parameters; `armature`, `damping`, `frictionloss` to an `assumed` actuator card; keyframes to named
poses).

**Validation specifics: structural equality across the three Unitree quadrupeds.** After import, strip every
numeric parameter from the three designs (`go1`, `go2`, `a1`) and compare the remaining term structure
(constructor tree, joint classes and axes up to sign convention, end-effector kinds). The three skeletons must
be identical, or the difference must be a real difference of the robots that the loss report or the manifest
names (an extra fixed link, a different sensor set). A skeleton difference that traces to file layout (body
order, default-class nesting, mesh file naming) is an importer defect.

**Use.** Import parity and contact-free dynamics parity rows; the mass row of `quad-mammal@a1`; the structural
comparison above as a metamorphic test of the importer.

## Acceptance criteria

- [ ] `assets/robots/a1/` passes the licence check and validation; `mark = "P"` with commit and read date.
- [ ] `import.fbot` has a pinned `DesignId`; loss report equals its golden; counts equal the manifest.
- [ ] The parameter-stripped skeletons of `go1`, `go2` and `a1` are identical, or every difference is
      explained by a named field of the sources.
- [ ] The mass on file is written by validation and consumed by the `quad-mammal@a1` preset row.

## Test plan

### Unit tests

- `robot_a1_manifest_valid`, `robot_a1_licence_bsd3_matches`.
- `robot_a1_counts`: actuated 12, bodies 13, n_v 18 (as computed at the pin).
- `robot_a1_mass_properties_consistent`, `robot_a1_joint_limits_valid`, `robot_a1_leg_mirror_consistency`.
- `robot_unitree_quadrupeds_same_skeleton`: the three stripped terms compare equal or differ only at listed
  paths.
- `robot_a1_loss_report_golden`, `robot_a1_design_id_pinned`.

### End-to-end script and logging

`scripts/e2e/assets_licences.sh` (NEG-16): import, both checks, golden comparison, and the skeleton comparison
across the three robots; logs commit, licence, counts, mass, the skeleton digest of each robot and any
differing paths.

### Known answers and negative controls

The import-parity row (10⁻⁹ m, 10⁻⁹ rad, 10⁴ configurations). Controls: a source file with its bodies
reordered (a `synthetic` permutation of sibling elements) must import to the same `DesignId` (canonical forms
do not depend on sibling order where the format says order is irrelevant); an altered licence hash.
Determinism: goldens are data.

## Considerations

- If the three skeletons differ for a real reason, the difference is recorded once here and the presets of the
  template account for it; it is not smoothed over.
- Recalled figures are expectations only; the reading at the pin replaces them.

## References

Appendix G.1; §14.10; §14.15 (GATE-SKEL-01, GATE-SKEL-07); Appendix H.3.
