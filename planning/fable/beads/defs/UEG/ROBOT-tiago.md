---
key: "ROBOT-tiago"
title: "assets/robots/tiago: TIAGo from MuJoCo Menagerie (Apache-2.0): omni base for `mecanum`, `mobile-manipulator`"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "skeleton", "frob-io", "solid", "test", "asset", "robot"]
parent: "WP-08"
deps: ["BD-066", "BD-067", "BD-105", "N14b-20", "NEG-15"]
needs: []
related: ["ZOO-mecanum", "ZOO-mobile-manipulator"]
gates: []
plan_refs: ["Appendix G.1", "§14.10", "§14.15 GATE-SKEL-01, GATE-SKEL-07", "Appendix H.3"]
---
## Background

TIAGo is a mobile manipulator with a lifting torso and a seven-joint arm. Its public MuJoCo model in the
Menagerie collection is on file under Apache-2.0 (FACT from the research pass of 2026-11-21; re-checked at
admission), listed as the reference for the `mecanum` archetype through its omnidirectional base and as one of
the references for `mobile-manipulator`. It is the only holonomic-base reference on file. It is never the
source of a Reality claim.

One point must be settled by reading the file, not assumed: the robot exists with a differential-drive base
and with an omnidirectional base, and the collection may hold either or both. The archetype mapping on file
says "omni base". If the pinned model's base is differential, that is a finding: the mapping row is amended
by a dated record (the robot then serves `diff-drive` and `mobile-manipulator`, and `mecanum` has no
reference), never changed in silence.

## Scope

In: `assets/robots/tiago/` (manifest, licence text and NOTICE, pinned source record, import, loss-report and
validation goldens); the base-only sub-design for the `mecanum@tiago` preset; mass-on-file records for base
and whole robot; the base-kind determination. Out: the shared layout and checks (NEG-15); the importer
(BD-066); the roller contact law (BD-245).

## Technical approach

**Source record.** MuJoCo Menagerie at the project's single pinned commit; directory expected `pal_tiago/`,
possibly with a second directory for a two-arm variant (recalled, mark M: confirm at the pin; the manifest
names the single-arm entry and lists others as not used). Licence Apache-2.0: vendoring with the licence text,
any upstream NOTICE, unmodified files.

**Expected facts (recalled, mark M; the pinned files govern).** A prismatic torso lift; a seven-revolute arm;
a gripper; a pan-tilt head; a base with either two driven wheels and casters or four mecanum wheels; total
mass on the order of 70 kg.

**Import path.** `frob design import assets/robots/tiago/source/<entry> --format mjcf --name tiago --key
asset-tiago-<source hash> --json` (NEG-15's import recipe; the verb writes no file, and the golden `import.fbot` is
the design's `term` section). Base-kind determination (step 1 of validation): count driven wheels
and inspect how the source models them. Mecanum wheels in public models are written in one of three ways:
explicit roller bodies on each wheel; an anisotropic friction setting on a plain wheel; or a planar joint with
velocity actuators that bypasses wheel contact. The importer maps the first two to `roller-wheel` with the
roller angle recovered (and its sign per wheel checked against the working pattern tᵢ = −sign(xᵢ·yᵢ)); the
third is not a wheel model at all and is reported in the loss report as "base motion prescribed, no wheel
contact", in which case the base serves kinematic parity only.

**Validation specifics.** (2) If mecanum: the twist-to-wheel map built from the imported wheel positions and
roller angles has rank 3. The map is the one BD-109 states for the `mecanum` template (wheel i at (xᵢ, yᵢ) with
radius r and roller angle αᵢ to its rolling direction, tᵢ = tan αᵢ): ωᵢ = (1/r)·(v_x + tᵢ·v_y + (tᵢ·xᵢ − yᵢ)·ω_z),
four rows in (v_x, v_y, ω_z); the validation builds it from the imported xᵢ, yᵢ, r and the recovered αᵢ and
computes its rank exactly at rational wheel positions (the positions rounded to the file's printed digits),
and with all four tᵢ equal the first two columns are parallel and the rank is 2. (3) The arm: seven revolute joints, Jacobian rank 6 at the reference pose. (4)
Base-only sub-design `base.fbot` by a recorded edit, conserving mass. (5) Mass properties and limits as for
every robot.

**Use.** Import parity and dynamics parity rows; the mass row of `mecanum@tiago` (M1, if the base is omni);
the base's share of `mobile-manipulator@iiwa7-tiago` (M3).

## Acceptance criteria

- [ ] `assets/robots/tiago/` passes the licence check (Apache-2.0) and validation; `mark = "P"`.
- [ ] The base kind is determined from the file and recorded; a mismatch with the mapping on file produces the
      dated amendment, and the archetype index is regenerated from it.
- [ ] If mecanum, roller angles are recovered with the working sign pattern and the map has rank 3.
- [ ] Mass-on-file records for base and whole robot are written by validation.

## Test plan

### Unit tests

- `robot_tiago_manifest_valid`, `robot_tiago_licence_apache2_notice`.
- `robot_tiago_base_kind_determined`: one of `mecanum`, `differential`, `prescribed-planar`.
- `robot_tiago_roller_pattern_and_rank` (conditional on `mecanum`).
- `robot_tiago_arm_rank_six`.
- `robot_tiago_base_subdesign_conserves_mass`.
- `robot_tiago_mass_properties_consistent`, `robot_tiago_joint_limits_valid`.
- `robot_tiago_loss_report_golden`, `robot_tiago_design_id_pinned`.

### End-to-end script and logging

`scripts/e2e/assets_licences.sh` (NEG-16): import, both checks, golden comparison; logs commit, entry file,
licence, base kind with the evidence for it, roller angles, masses and findings.

### Known answers and negative controls

The import-parity row (10⁻⁹ m, 10⁻⁹ rad, 10⁴ configurations). Controls: roller angles imported with equal
signs (rank 2, the parallel-roller defect); a prescribed planar base reported as a wheel model; an altered
licence hash. Determinism: goldens are data.

## Considerations

- The mapping question is the kind of fact the source register marks as fetched but not primary; this bead is
  the reading task that settles it.
- Recalled figures are expectations only; the reading at the pin replaces them.

## References

Appendix G.1; §14.10; §14.15 (GATE-SKEL-01, GATE-SKEL-07); Appendix H.3.
