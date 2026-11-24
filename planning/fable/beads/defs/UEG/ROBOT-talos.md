---
key: "ROBOT-talos"
title: "assets/robots/talos: TALOS from MuJoCo Menagerie (Apache-2.0): import parity only; heavy, many joints"
type: "task"
priority: 2
milestone: "M1"
labels: ["m1", "skeleton", "frob-io", "solid", "test", "asset", "robot"]
parent: "WP-08"
deps: ["BD-066", "BD-067", "BD-074", "BD-105", "N14b-20", "NEG-15"]
needs: []
related: ["ZOO-humanoid"]
gates: []
plan_refs: ["Appendix G.1", "§14.10", "§14.15 GATE-SKEL-01", "§17.5", "Appendix H.3"]
---
## Background

TALOS is a full-size torque-controlled research humanoid and the heaviest robot on file. Its public MuJoCo
model in the Menagerie collection is on file under Apache-2.0 (FACT from the research pass of 2026-11-21;
re-checked at admission) for import parity only: imported and checked kinematically against the pinned
oracle, with no preset, no mass row and no scorecard. It complements the small OP3 at the other end of the
scale and is expected to have the highest joint count of the collection's humanoids used here, including
grippers whose fingers are coupled. It is never the source of a Reality claim.

## Scope

In: `assets/robots/talos/` (manifest with `role = "import-parity-only"`, licence text and NOTICE, pinned
source record, import, loss-report and validation goldens); the size-class record. Out: the shared layout and
checks (NEG-15); the importer (BD-066); coupling transmissions (BD-074); size buckets (BD-171).

## Technical approach

**Source record.** MuJoCo Menagerie at the project's single pinned commit; directory expected `pal_talos/`
(recalled, mark M: confirm at the pin). Licence Apache-2.0: vendoring with the licence text and any upstream
NOTICE, files unmodified.

**Expected facts (recalled, mark M; the pinned files govern).** On the order of 32 actuated joints (legs of
six, a two-joint torso, arms of seven, grippers, a two-joint head); total mass on the order of 95 kg.

**Import path.** `frob design import assets/robots/talos/source/<entry> --format mjcf --name talos --key
asset-talos-<source hash> --json` (NEG-15's import recipe; the verb writes no file, and the golden `import.fbot` is
the design's `term` section).

**What this robot adds to coverage (HYPOTHESIS until the file is read).** (1) Size class: with a floating
base its generalized-velocity count is expected near 38 to 45, which lands in the 48 bucket of the default
set {12, 18, 24, 36, 48, 64} and above the reference humanoid's 35; the validation records n_v and, as DATA for the
bucket planner (labelled DESIGN: the default set is the planner's, N17a-05 at M2, and is quoted here, not
implemented), the expected bucket and the padding overhead of this robot in it ((capacity/n_v) to the powers
1, 2, 3). The planner's own assertion on this robot (`frob phylum plan` choosing that bucket) is made at M2 by
step 6a of N17a-08's script, which depends on this bead and reads this manifest; this M1 bead does not wait on
it. (2) Coupled gripper fingers: if the source couples them by an equality or a tendon, they are
imported as one actuator through a coupling matrix; if it writes them as independent position actuators with
the same command, the import keeps them independent and the loss report says that a mechanical coupling may
exist on the real robot and is absent from the file. (3) Seven-joint arms: Jacobian rank 6 with a
one-dimensional null space at the reference pose, per arm.

**Validation specifics.** Counts, mass properties (heavy links: inertia consistency checked exactly, as
always), joint limits and limb mirror consistency as for every robot.

## Acceptance criteria

- [ ] `assets/robots/talos/` passes the licence check (Apache-2.0) and validation; `mark = "P"`.
- [ ] `import.fbot` has a pinned `DesignId`; loss report equals its golden; counts equal the manifest.
- [ ] n_v, the bucket and the three padding overheads are recorded; gripper coupling is imported or reported
      absent, never guessed.
- [ ] The robot appears in no mass row and no scorecard.

## Test plan

### Unit tests

- `robot_talos_manifest_valid`, `robot_talos_licence_apache2_notice`.
- `robot_talos_counts` (as computed at the pin).
- `robot_talos_bucket_and_padding_recorded`: the manifest records n_v, the expected bucket (the smallest capacity
  ≥ n_v in the quoted default set) and the overheads (capacity/n_v)ᵖ for p = 1, 2, 3, each labelled DESIGN data
  for the planner; a planted n_v of 49 makes the recorded bucket 64.
- `robot_talos_gripper_coupling_accounted`.
- `robot_talos_arm_rank_six_null_one` (each arm).
- `robot_talos_mass_properties_consistent`, `robot_talos_joint_limits_valid`,
  `robot_talos_limb_mirror_consistency`, `robot_talos_loss_report_golden`, `robot_talos_design_id_pinned`.

### End-to-end script and logging

`scripts/e2e/assets_licences.sh` (NEG-16): import, both checks, golden comparison; the bucket row is logged
as recorded data (the planner's check is N17a-08's at M2); logs commit, licence, counts, mass, bucket, overheads
and findings.

### Known answers and negative controls

The import-parity row (10⁻⁹ m, 10⁻⁹ rad, 10⁴ configurations). Controls: fingers coupled by the importer
without a coupling element in the source (an invented transmission, caught by the accounting test); an
altered licence hash. Determinism: goldens are data.

## Considerations

- A robot above the reference humanoid's size is a useful check that nothing in the dense path assumes
  n_v ≤ 35; it is not a performance workload.
- Recalled figures are expectations only; the reading at the pin replaces them.

## References

Appendix G.1; §14.10; §14.15 (GATE-SKEL-01); §17.5; Appendix H.3.
