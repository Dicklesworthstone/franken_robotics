---
key: "ROBOT-booster-t1"
title: "assets/robots/booster-t1: Booster T1 from MuJoCo Menagerie (Apache-2.0): `humanoid`, import parity only"
type: "task"
priority: 2
milestone: "M1"
labels: ["m1", "skeleton", "frob-io", "solid", "test", "asset", "robot"]
parent: "WP-08"
deps: ["BD-066", "BD-067", "BD-109", "N14b-20", "NEG-15"]
needs: []
related: ["ROBOT-fourier-n1", "ROBOT-op3", "ROBOT-talos", "ZOO-humanoid"]
gates: []
plan_refs: ["Appendix G.1", "§14.10", "§14.15 GATE-SKEL-01", "Appendix H.3"]
---
## Background

Five humanoids are on file for import parity only: Booster T1, Apptronik Apollo, Fourier N1, OP3 and TALOS,
all from the MuJoCo Menagerie collection under Apache-2.0 (FACT from the research pass of 2026-11-21;
re-checked at admission). "Import parity only" means the robot is imported and checked kinematically against
the pinned oracle, and that is all: no preset of the `humanoid` template targets it, it has no mass row in
the archetype gate, and it is not a benchmark baseline. Their purpose is breadth: an importer validated on
two robots of one maker has seen one authoring style, and each of these five was authored by a different
team with different conventions. The Booster T1 is a compact humanoid of about child height. None is the
source of a Reality claim.

## Scope

In: `assets/robots/booster-t1/` (manifest with `role = "import-parity-only"`, licence text and NOTICE, pinned
source record, import, loss-report and validation goldens). Out: the shared layout and checks (NEG-15); the
importer (BD-066); oracle capture (BD-067); any preset or scorecard.

## Technical approach

**Source record.** MuJoCo Menagerie at the project's single pinned commit; directory expected `booster_t1/`
(recalled, mark M: confirm at the pin). Licence Apache-2.0: vendoring with the licence text and any upstream
NOTICE, files unmodified.

**Expected facts (recalled, mark M; the pinned files govern).** About 23 actuated joints (legs of six, arms
of four, a waist joint, a two-joint head); total mass on the order of 30 kg.

**Import path.** `frob design import assets/robots/booster-t1/source/<entry> --format mjcf --name booster-t1 --key
asset-booster-t1-<source hash> --json` (NEG-15's import recipe; the verb writes no file, and the golden `import.fbot` is
the design's `term` section).

**What this robot adds to importer coverage (HYPOTHESIS until the file is read).** A head with two joints and
a waist joint between pelvis and torso: a serial chain through the trunk, which exercises parent-frame
handling where a bilateral template would branch. The validation records the trunk chain's joint order and
axes and compares the import's kinematic tree with the `humanoid` template's by a tree edit distance (which
template joints would have to be locked or added to match), as data for deciding whether the template covers
this class of humanoid. No preset is produced.

**Validation specifics.** Counts, mass properties, joint limits and limb mirror consistency as for every
robot; forward-kinematics parity fixtures captured with the others.

## Acceptance criteria

- [ ] `assets/robots/booster-t1/` passes the licence check (Apache-2.0) and validation; `mark = "P"`.
- [ ] `import.fbot` has a pinned `DesignId`; loss report equals its golden; counts equal the manifest.
- [ ] The tree comparison with the `humanoid` template is recorded (locked and missing joints listed).
- [ ] The robot appears in no mass row and no scorecard (the index's `role` excludes it).

## Test plan

### Unit tests

- `robot_booster_t1_manifest_valid`, `robot_booster_t1_licence_apache2_notice`.
- `robot_booster_t1_counts` (as computed at the pin; n_v = actuated + 6).
- `robot_booster_t1_mass_properties_consistent`, `robot_booster_t1_joint_limits_valid`,
  `robot_booster_t1_limb_mirror_consistency`.
- `robot_booster_t1_tree_vs_template_recorded`.
- `robot_booster_t1_role_excludes_mass_row`.
- `robot_booster_t1_loss_report_golden`, `robot_booster_t1_design_id_pinned`.

### End-to-end script and logging

`scripts/e2e/assets_licences.sh` (NEG-16): import, both checks, golden comparison; logs commit, licence,
counts, mass, the tree comparison and findings.

### Known answers and negative controls

The import-parity row (forward kinematics within 10⁻⁹ m and 10⁻⁹ rad over 10⁴ configurations; a loss report
listing every dropped field). Controls: the robot added to the mass gate by a planted index edit (rejected by
role); an altered licence hash. Determinism: goldens are data.

## Considerations

- Recalled figures are expectations only; the reading at the pin replaces them.
- If the tree comparison shows the template cannot reach this class by locking joints, that is a finding for
  the template's owner, not a failure of this asset.

## References

Appendix G.1; §14.10; §14.15 (GATE-SKEL-01); Appendix H.3.
