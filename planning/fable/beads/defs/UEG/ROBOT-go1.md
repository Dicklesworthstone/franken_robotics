---
key: "ROBOT-go1"
title: "assets/robots/go1: Unitree Go1 from MuJoCo Menagerie (BSD-3): parity fixture and baseline for `quad-mammal`"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "skeleton", "frob-io", "solid", "test", "asset", "robot"]
parent: "WP-08"
deps: ["BD-066", "BD-067", "N14b-20", "NEG-15"]
needs: []
related: ["ROBOT-go2", "ZOO-quad-mammal"]
gates: []
plan_refs: ["Appendix G.1", "§14.10", "§14.15 GATE-SKEL-01, GATE-SKEL-07", "Appendix H.3"]
---
## Background

The Unitree Go1 is a small commercial quadruped with three actuated joints per leg. Its public MuJoCo model in
the Menagerie collection is on file under BSD-3 (FACT from the research pass of 2026-11-21; re-checked at
admission). It is one of six reference robots that serve the `quad-mammal` archetype: an imported model that
the importer must reproduce kinematically against a pinned oracle, a baseline whose scorecard the archetype's
presets are compared with, and a target for the mass row of the archetype gate (a preset of the template must
come within 15% of this robot's mass on file). It is never the source of a Reality claim.

## Scope

In: `assets/robots/go1/` with its manifest, licence text, pinned source record, import golden, loss-report
golden and validation golden; the `quad-mammal@go1` mass-on-file record. Out: the shared layout and checks
(NEG-15); the MJCF importer (BD-066); oracle capture (BD-067); the preset itself (BD-109).

## Technical approach

**Source record.** Collection: MuJoCo Menagerie (repository URL from the source register). Directory expected
`unitree_go1/` with a scene file and a robot file (recalled, mark M: confirm the path at the pinned commit).
Pin: one commit hash for the whole collection, shared by all Menagerie robots of this project so that their
fixtures are mutually consistent. Licence: BSD-3-Clause; redistribution of the model files is allowed with the
copyright notice, the licence text and the no-endorsement clause preserved, so `source/` may be vendored with
`LICENSE` beside it.

**Expected facts (recalled, mark M; the values computed from the pinned files govern and are what the manifest
stores).** 12 actuated revolute joints (hip abduction, hip pitch, knee per leg); 13 bodies (trunk and three
links per leg); floating base, so n_v = 18; total mass on the order of 12 kg; one actuator per joint with a
control range; foot contact by a sphere geom per foot.

**Import path.** `frob design import assets/robots/go1/source/<entry> --format mjcf --name go1 --key
asset-go1-<source hash> --json` (NEG-15's import recipe; the verb writes no file, and the golden `import.fbot` is
the design's `term` section). Expected loss-report entries (HYPOTHESIS until the file is read):
visual meshes carried as visuals only; mesh collision geometry on the trunk and thighs replaced by fitted
primitives with a recorded fitting error; solver and contact parameters of the source (`solref`, `solimp`,
`condim`) not mapped to a native contact law and listed; joint `armature`, `damping` and `frictionloss` mapped
to reflected inertia, viscous loss and stiction of an `assumed` actuator card; keyframes mapped to named
poses; default classes expanded.

**Validation specifics.** Left and right legs must be mirror images: masses equal and inertias related by the
mirror, to the precision of the source; a discrepancy is a finding (public models sometimes carry asymmetric
rounding) and decides whether the imported design's computed Γ_mech is C₂ × C₂ or smaller. Front and rear legs
are compared the same way.

**Use.** Import parity (forward kinematics within 10⁻⁹ m and 10⁻⁹ rad over 10⁴ configurations against the
pinned oracle; GATE-SKEL-01 row); contact-free dynamics parity (joint accelerations within 10⁻⁹ relative in
`f64`); the mass row of `quad-mammal@go1` (GATE-SKEL-07); reference scorecards of the velocity-tracking and
terrain task families at M2.

## Acceptance criteria

- [ ] `assets/robots/go1/` passes `robot_licence_check` and `robot_validate`; `mark = "P"` with the commit and
      read date recorded.
- [ ] `import.fbot` has a pinned `DesignId`; the loss report equals its golden; counts equal the manifest.
- [ ] Mirror consistency of the legs is reported, with the computed Γ_mech of the import stated.
- [ ] The mass on file is written by validation into the manifest field that the `quad-mammal@go1` preset row reads; the preset and its
      GATE-SKEL-07 row are BD-109's, which reads the pinned fixture, so this bead checks the field (present, sourced,
      equal to the validated total) and does not wait on the preset.

## Test plan

### Unit tests

- `robot_go1_manifest_valid`, `robot_go1_licence_bsd3_matches`.
- `robot_go1_counts`: actuated 12, bodies 13, n_v 18 (as computed at the pin).
- `robot_go1_mass_properties_consistent`: every body's pseudo-inertia positive semidefinite; total mass equals
  `mass_on_file`.
- `robot_go1_joint_limits_valid`: 12 limited joints, lower < upper.
- `robot_go1_leg_mirror_consistency`.
- `robot_go1_loss_report_golden`, `robot_go1_design_id_pinned`.

### End-to-end script and logging

`scripts/e2e/assets_licences.sh` (NEG-16) imports the robot, runs both checks and compares the three goldens;
kinematic parity against the oracle fixture runs in the import-and-export script of the design-language unit.
Log lines carry commit, licence identifier and hash, counts, mass in kg, findings and `DesignId`.

### Known answers and negative controls

Import parity for each reference robot (forward kinematics within 10⁻⁹ m and 10⁻⁹ rad over 10⁴ configurations;
the loss report lists every dropped field found by a field-coverage diff). Controls: a manifest whose licence
hash is altered; an import with the loss report suppressed; a foot sphere dropped by the importer (the
end-effector count changes). Determinism: goldens are data; identities equal across platforms.

## Considerations

- The figures above are recollections and may be wrong in detail; nothing downstream reads them. The reading
  at the pin replaces them, and a difference is recorded, not treated as a failure.
- The source's contact parameters are not a native contact law: contact parity with the imported settings is
  labelled parity-only by its owner.

## References

Appendix G.1; §14.10; §14.15 (GATE-SKEL-01, GATE-SKEL-07); Appendix H.3.
