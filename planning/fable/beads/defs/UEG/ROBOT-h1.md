---
key: "ROBOT-h1"
title: "assets/robots/h1: Unitree H1 from MuJoCo Menagerie (BSD-3): full-size parity fixture and baseline for `humanoid`"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "skeleton", "frob-io", "solid", "test", "asset", "robot"]
parent: "WP-08"
deps: ["NEG-15", "BD-066", "BD-067", "N14b-20"]
needs: []
related: ["ZOO-humanoid", "BD-109", "N14b-24", "BD-089", "ROBOT-g1"]
gates: []
plan_refs: ["Appendix G.1", "§14.10", "§14.15 GATE-SKEL-01, GATE-SKEL-07", "Appendix H.3"]
---
## Background

The Unitree H1 is a full-size humanoid and, with the G1, one of the two reference robots that serve the
`humanoid` archetype as both parity fixture and baseline. It is taller and heavier than the G1 and has fewer
joints, so the pair brackets the archetype: the `humanoid` template must reach both by presets, which tests
the template's lockable joints (a preset for a robot with fewer joints locks the ones it lacks) as well as its
dimension levers. Its public MuJoCo model in the Menagerie collection is on file under BSD-3 (FACT from the
research pass of 2026-11-21; re-checked at admission). It is never the source of a Reality claim.

## Scope

In: `assets/robots/h1/` (manifest, licence text, pinned source record, import, loss-report and validation
goldens) and the `humanoid@h1` mass-on-file record. Out: the shared layout and checks (NEG-15); the importer
(BD-066); oracle capture (BD-067).

## Technical approach

**Source record.** MuJoCo Menagerie at the project's single pinned commit; directory expected `unitree_h1/`
(recalled, mark M: confirm at the pin). Licence BSD-3-Clause: vendoring allowed with notice, licence text and
the no-endorsement clause.

**Expected facts (recalled, mark M; the pinned files govern).** About 19 actuated joints (five per leg, one
torso yaw, four per arm); about 20 bodies; n_v about 25; total mass on the order of 47 kg; ankle with a single
pitch axis in the public model.

**Import path.** `frob design import --format mjcf assets/robots/h1/source/<entry> --out
assets/robots/h1/import.fbot --json`. Expected loss-report entries (HYPOTHESIS until read): visual meshes;
fitted collision primitives with recorded error; unmapped solver and contact parameters; `armature`,
`damping`, `frictionloss` to an `assumed` card; foot collision geometry (boxes or several spheres) mapped to a
foot patch with a stated patch radius, the mapping listed with its error.

**Validation specifics.** Preset by locking: `humanoid@h1` is the template with joints locked so that the
actuated count equals this robot's; the validation records which template joints the preset locks and checks
that the locked template's kinematic tree is isomorphic to the import's (same parent relation and joint axes
up to the template's conventions). Arm and leg mirror consistency as for every bilateral robot.

**Use.** Import parity and contact-free dynamics parity rows; the mass row of `humanoid@h1`.

## Acceptance criteria

- [ ] `assets/robots/h1/` passes the licence check and validation; `mark = "P"` with commit and date.
- [ ] `import.fbot` has a pinned `DesignId`; loss report equals its golden; counts equal the manifest.
- [ ] The `humanoid@h1` preset's locked tree is isomorphic to the import's, with the locked joints listed.
- [ ] The mass on file is written by validation and consumed by the `humanoid@h1` preset row.

## Test plan

### Unit tests

- `robot_h1_manifest_valid`, `robot_h1_licence_bsd3_matches`.
- `robot_h1_counts`: actuated, bodies and n_v as computed at the pin (n_v = actuated + 6).
- `robot_h1_mass_properties_consistent`, `robot_h1_joint_limits_valid`, `robot_h1_limb_mirror_consistency`.
- `robot_h1_preset_tree_isomorphic`.
- `robot_h1_foot_patch_mapping_reported`.
- `robot_h1_loss_report_golden`, `robot_h1_design_id_pinned`.

### End-to-end script and logging

`scripts/e2e/assets_licences.sh` (NEG-16): import, both checks, golden comparison; logs commit, licence,
counts, mass, the locked-joint list of the preset and findings.

### Known answers and negative controls

The import-parity row (10⁻⁹ m, 10⁻⁹ rad, 10⁴ configurations). Controls: a preset that removes joints instead
of locking them (the tree comparison still passes but the template's `DesignId` family changes, which the
preset test rejects); an altered licence hash. Determinism: goldens are data.

## Considerations

- The joint count is recalled and may differ at the pin (public models are revised); the manifest's computed
  count governs and the preset is derived from it.
- Recalled figures are expectations only; the reading at the pin replaces them.

## References

Appendix G.1; §14.10; §14.15 (GATE-SKEL-01, GATE-SKEL-07); Appendix H.3.
