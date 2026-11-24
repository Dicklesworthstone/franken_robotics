---
key: "ROBOT-go2"
title: "assets/robots/go2: Unitree Go2 from MuJoCo Menagerie (BSD-3): the default baseline for `quad-mammal`"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "skeleton", "frob-io", "solid", "test", "asset", "robot"]
parent: "WP-08"
deps: ["BD-066", "BD-067", "N14b-20", "NEG-15"]
needs: []
related: ["ZOO-quad-mammal"]
gates: []
plan_refs: ["Appendix G.1", "§14.10", "§14.15 GATE-SKEL-01, GATE-SKEL-07", "Appendix H.3", "§36.3"]
---
## Background

The Unitree Go2 is the successor of the Go1 and the quadruped most often used in published locomotion
training results, which makes it the natural comparison point for the project's own quadruped workload: the
template preset `quad-mammal@go2` is the one named in the examples of the archetype catalogue, and the
reference quadruped of the training-time parity rows is a quadruped of this class. Its public MuJoCo model in
the Menagerie collection is on file under BSD-3 (FACT from the research pass of 2026-11-21; re-checked at
admission). It is an imported parity fixture and benchmark baseline, never the source of a Reality claim.

## Scope

In: `assets/robots/go2/` (manifest, licence text, pinned source record, import golden, loss-report golden,
validation golden) and the `quad-mammal@go2` mass-on-file record. Out: the shared layout and checks (NEG-15);
the importer (BD-066); oracle capture (BD-067); training and its parity rows (BD-198, BD-218).

## Technical approach

**Source record.** MuJoCo Menagerie at the project's single pinned commit; directory expected `unitree_go2/`
(recalled, mark M: confirm at the pin; the directory may hold more than one variant of the model, for example
one prepared for a GPU engine, and the manifest names the entry file used and why). Licence BSD-3-Clause:
vendoring allowed with notice, licence text and the no-endorsement clause.

**Expected facts (recalled, mark M; the pinned files govern).** 12 actuated revolute joints; 13 bodies;
n_v = 18; total mass on the order of 15 kg; torque-style actuators with control ranges that differ between
hip, thigh and calf joints; sphere feet.

**Import path.** `frob design import assets/robots/go2/source/<entry> --format mjcf --name go2 --key
asset-go2-<source hash> --json` (NEG-15's import recipe; the verb writes no file, and the golden `import.fbot` is
the design's `term` section). Expected loss-report entries (HYPOTHESIS until read): visual meshes as
visuals only; mesh collision replaced by fitted primitives with recorded error; source contact and solver
parameters listed as unmapped; joint `armature`, `damping`, `frictionloss` mapped to an `assumed` actuator
card; per-joint actuator force ranges mapped to torque limits on the card, with the calf's larger range
(a knee reduction) preserved and not averaged; IMU and joint sensors mapped to native sensors with frame,
rate and clock, the rate being `assumed` where the source gives none.

**Validation specifics.** Mirror consistency of legs as for every quadruped; in addition the per-joint
actuator limits must be reported per joint class, and the import must not silently unify them. If a variant
file for a GPU engine is present, its collision geometry differs from the main file's; only the chosen entry
is imported and the other is listed as not used.

**Use.** Import parity and contact-free dynamics parity rows; the mass row of `quad-mammal@go2`; the reference
robot of the M2 velocity-tracking scorecard and of comparisons with published training times (those figures
are Parity against a publication, at search-summary level until re-read).

## Acceptance criteria

- [ ] `assets/robots/go2/` passes the licence check and validation; `mark = "P"` with commit and read date.
- [ ] `import.fbot` has a pinned `DesignId`; loss report equals its golden; counts equal the manifest.
- [ ] Actuator limits are carried per joint; the entry-file choice is recorded with its reason.
- [ ] The mass on file is written by validation into the manifest field that the `quad-mammal@go2` preset row reads; the preset and its
      GATE-SKEL-07 row are BD-109's, which reads the pinned fixture, so this bead checks the field (present, sourced,
      equal to the validated total) and does not wait on the preset.

## Test plan

### Unit tests

- `robot_go2_manifest_valid`, `robot_go2_licence_bsd3_matches`.
- `robot_go2_counts`: actuated 12, bodies 13, n_v 18 (as computed at the pin).
- `robot_go2_mass_properties_consistent`, `robot_go2_joint_limits_valid`.
- `robot_go2_actuator_limits_per_joint_class`: hip, thigh and calf limits distinct if the source's are.
- `robot_go2_leg_mirror_consistency`, `robot_go2_loss_report_golden`, `robot_go2_design_id_pinned`.

### End-to-end script and logging

`scripts/e2e/assets_licences.sh` (NEG-16): import, both checks, golden comparison; logs commit, entry file,
licence, counts, mass, per-class actuator limits, findings, `DesignId`.

### Known answers and negative controls

The import-parity row (10⁻⁹ m, 10⁻⁹ rad, 10⁴ configurations). Controls: actuator limits averaged across joint
classes by a planted importer defect (caught by the per-class test); an altered licence hash; the unused
variant file imported in place of the entry (the `DesignId` differs from the pinned one). Determinism: goldens
are data.

## Considerations

- Published training times for this robot are anchors for Parity at M3 and are at search-summary level; they
  are re-read from the primary source before any gate cites them.
- Recalled figures are expectations only; the reading at the pin replaces them.

## References

Appendix G.1; §14.10; §14.15 (GATE-SKEL-01, GATE-SKEL-07); Appendix H.3; §36.3.
