---
key: "ROBOT-op3"
title: "assets/robots/op3: OP3 from MuJoCo Menagerie (Apache-2.0): import parity only; the small-scale fixture"
type: "task"
priority: 2
milestone: "M1"
labels: ["m1", "skeleton", "frob-io", "solid", "test", "asset", "robot"]
parent: "WP-08"
deps: ["BD-066", "BD-067", "N14a-13", "N14b-20", "NEG-15"]
needs: []
related: ["ROBOT-talos", "ZOO-humanoid"]
gates: []
plan_refs: ["Appendix G.1", "§14.10", "§14.15 GATE-SKEL-01", "Appendix H.3"]
---
## Background

OP3 is a small open-platform humanoid, about half a metre tall and a few kilograms, driven by hobby-class
servos. Its public MuJoCo model in the Menagerie collection is on file under Apache-2.0 (FACT from the
research pass of 2026-11-21; re-checked at admission) for import parity only: imported and checked
kinematically against the pinned oracle, with no preset, no mass row and no scorecard. It is on file because
it is small. Every other reference robot weighs between ten and a hundred kilograms; this one is an order of
magnitude lighter with link inertias three orders of magnitude smaller, so it exposes absolute tolerances
and thresholds that were tuned on larger machines. It is never the source of a Reality claim.

## Scope

In: `assets/robots/op3/` (manifest with `role = "import-parity-only"`, licence text and NOTICE, pinned source
record, import, loss-report and validation goldens); the scale record. Out: the shared layout and checks
(NEG-15); the importer (BD-066); inertia consistency checks (N14a-13); oracle capture (BD-067).

## Technical approach

**Source record.** MuJoCo Menagerie at the project's single pinned commit; directory expected `robotis_op3/`
(recalled, mark M: confirm at the pin). Licence Apache-2.0: vendoring with the licence text and any upstream
NOTICE, files unmodified.

**Expected facts (recalled, mark M; the pinned files govern).** 20 actuated joints (legs of six, arms of
three, a two-joint head); height about 0.5 m; total mass about 3.5 kg; position-controlled servos.

**Import path.** `frob design import assets/robots/op3/source/<entry> --format mjcf --name op3 --key
asset-op3-<source hash> --json` (NEG-15's import recipe; the verb writes no file, and the golden `import.fbot` is
the design's `term` section).

**What this robot adds to coverage (HYPOTHESIS until the file is read).** Scale. (1) Inertia consistency must
be decided exactly: a small link's principal moments are of order 10⁻⁵ kg·m², so a check with an absolute
epsilon (for example "eigenvalue above 10⁻⁶") passes or fails for the wrong reason; the project's check is an
exact LDLᵀ on the imported decimals and has no epsilon, and this robot is its fixture. (2) Every tolerance in
the validation and in the parity fixtures is either relative or stated in the quantity's own unit with a
scale; the validation runs once on the model as imported and once on a `synthetic` copy scaled by a factor of
ten in length (mass by 10³, inertia by 10⁵), and every verdict must be identical. (3) Servo actuators given
as position servos with a gain: mapped to an actuator card with a position loop, the gain carried, the
torque limit `assumed` if the source gives only a control range in angle.

**Validation specifics.** Counts, mass properties, joint limits and limb mirror consistency as for every
robot; the scaled-copy comparison; the list of every tolerance used with its kind (relative or scaled
absolute).

## Acceptance criteria

- [ ] `assets/robots/op3/` passes the licence check (Apache-2.0) and validation; `mark = "P"`.
- [ ] `import.fbot` has a pinned `DesignId`; loss report equals its golden; counts equal the manifest.
- [ ] Validation verdicts are identical on the model and on its ten-times scaled copy.
- [ ] No absolute epsilon appears in the inertia consistency check (lint), and the tolerance list is recorded.
- [ ] The robot appears in no mass row and no scorecard.

## Test plan

### Unit tests

- `robot_op3_manifest_valid`, `robot_op3_licence_apache2_notice`.
- `robot_op3_counts` (as computed at the pin).
- `robot_op3_inertia_exact_no_epsilon`: a `synthetic` link with principal moments (1, 1, 1.9) × 10⁻⁷ kg·m²
  passes and (1, 1, 2.1) × 10⁻⁷ is reported (triangle inequality), independent of magnitude.
- `robot_op3_scale_invariance_of_verdicts`.
- `robot_op3_position_servo_mapping`.
- `robot_op3_mass_properties_consistent`, `robot_op3_joint_limits_valid`, `robot_op3_loss_report_golden`,
  `robot_op3_design_id_pinned`.

### End-to-end script and logging

`scripts/e2e/assets_licences.sh` (NEG-16): import, both checks on the model and its scaled copy, golden
comparison; logs commit, licence, counts, mass, smallest principal moment, the tolerance list and the
verdict comparison.

### Known answers and negative controls

The import-parity row (10⁻⁹ m, 10⁻⁹ rad, 10⁴ configurations; for this robot 10⁻⁹ m is 2 × 10⁻⁹ of its height,
still far above `f64` round-off). Controls: an inertia check with an absolute eigenvalue threshold of 10⁻⁶
kg·m² (it rejects the consistent `synthetic` link (1, 1, 1.9) × 10⁻⁷ kg·m², and with it every link of this robot
whose smallest second moment lies below the threshold); an altered licence hash. Determinism: goldens are data.

## Considerations

- Small robots have high natural frequencies; a step size chosen for a 30 kg quadruped may be too coarse
  here, which is a simulation question outside this asset and a reason it is not a scorecard robot.
- Recalled figures are expectations only; the reading at the pin replaces them.

## References

Appendix G.1; §14.10; §14.15 (GATE-SKEL-01); Appendix H.3.
