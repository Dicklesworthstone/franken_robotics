---
key: "ROBOT-stretch3"
title: "assets/robots/stretch3: Stretch 3 from MuJoCo Menagerie (Apache-2.0): `diff-drive` base, `mobile-manipulator`"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "skeleton", "frob-io", "solid", "test", "asset", "robot"]
parent: "WP-08"
deps: ["NEG-15", "BD-066", "BD-067", "N14b-20", "BD-074"]
needs: []
related: ["ZOO-diff-drive", "ZOO-mobile-manipulator", "BD-109", "N14b-04", "N14b-24", "BD-117", "ROBOT-tiago"]
gates: []
plan_refs: ["Appendix G.1", "§14.10", "§14.15 GATE-SKEL-01, GATE-SKEL-07", "Appendix H.3"]
---
## Background

Stretch 3 is a mobile manipulator built on a differential-drive base, with a vertical lift and a telescoping
arm. Its public MuJoCo model in the Menagerie collection is on file under Apache-2.0 (FACT from the research
pass of 2026-11-21; re-checked at admission). It serves two archetypes: its base is the reference for
`diff-drive` (the only wheeled reference available at M1), and the whole robot is one of the references for
`mobile-manipulator` at M3. It is the first asset under Apache-2.0, whose redistribution terms differ from
the BSD and MIT models (a NOTICE file, if upstream ships one, must travel with the files, and changes must be
stated). It is never the source of a Reality claim.

## Scope

In: `assets/robots/stretch3/` (manifest, licence text and NOTICE, pinned source record, import, loss-report
and validation goldens); the base-only sub-design used by `diff-drive@stretch3`; mass-on-file records for the
base and for the whole robot. Out: the shared layout and checks (NEG-15); the importer (BD-066); coupling
transmissions (BD-074); the wheeled controller (BD-117).

## Technical approach

**Source record.** MuJoCo Menagerie at the project's single pinned commit; directory expected
`hello_robot_stretch_3/` (recalled, mark M: confirm at the pin). Licence Apache-2.0: vendoring allowed with
the licence text, any upstream NOTICE, and a statement of modifications if files are changed; the project
vendors them unmodified.

**Expected facts (recalled, mark M; the pinned files govern).** Two driven wheels and a passive caster or
skid; a prismatic lift; a telescoping arm of several prismatic segments that extend together; wrist joints; a
gripper driven through a linkage or tendon; a pan-tilt head with cameras; total mass on the order of 24 kg.

**Import path.** `frob design import --format mjcf assets/robots/stretch3/source/<entry> --out
assets/robots/stretch3/import.fbot --json`. Mapping rules exercised: joints coupled in the source by `tendon`
or `equality joint` elements (the telescoping segments; the gripper fingers) become one actuator driving
several joints through a coupling matrix (a transmission), never independent actuators; cameras become
sensors with frame, rate and clock, the rate `assumed` if the source gives none. Expected loss-report entries
(HYPOTHESIS until read): tendon path geometry beyond its coupling ratio; visual meshes; fitted collision
primitives; unmapped solver parameters.

**Base-only sub-design.** `assets/robots/stretch3/base.fbot`: the import with the lift and everything above it
replaced by one body carrying their summed mass properties at the stowed pose (a typed edit recorded in the
manifest). `diff-drive@stretch3` is compared with its mass.

**Validation specifics.** (1) Coupled joints: the telescoping segments have one independent coordinate; the
arm's extension equals the sum of segment travels at the coupling ratios, exactly. (2) Wheel geometry: track
width and wheel radius extracted and stored, since the wheeled known answers (growth vector, chord step,
threshold oracle) are evaluated on them. (3) Static tip-over with the arm fully extended at mid-lift and the
rated payload, reported as a margin. (4) Mass properties and limits as for every robot.

**Use.** Import parity and dynamics parity rows; mass rows of `diff-drive@stretch3` (M1) and of
`mobile-manipulator@iiwa7-stretch3` (M3, the base's share); wheeled baseline scorecards.

## Acceptance criteria

- [ ] `assets/robots/stretch3/` passes the licence check (Apache-2.0 with NOTICE handling) and validation;
      `mark = "P"`.
- [ ] Coupled joints are imported as transmissions with their ratios; no coupled joint has its own actuator.
- [ ] `base.fbot` is derived by a recorded edit and conserves total mass exactly.
- [ ] Both mass-on-file records are written by validation.

## Test plan

### Unit tests

- `robot_stretch3_manifest_valid`, `robot_stretch3_licence_apache2_notice`.
- `robot_stretch3_telescope_single_coordinate`, `robot_stretch3_gripper_coupled`.
- `robot_stretch3_base_subdesign_conserves_mass`.
- `robot_stretch3_wheel_geometry_extracted`.
- `robot_stretch3_mass_properties_consistent`, `robot_stretch3_joint_limits_valid`.
- `robot_stretch3_loss_report_golden`, `robot_stretch3_design_id_pinned`.

### End-to-end script and logging

`scripts/e2e/assets_licences.sh` (NEG-16): import, both checks, golden comparison, derivation of `base.fbot`;
logs commit, licence and NOTICE status, coupling ratios, wheel geometry, both masses and findings.

### Known answers and negative controls

The import-parity row (10⁻⁹ m, 10⁻⁹ rad, 10⁴ configurations, sampled in independent coordinates). Controls:
segments imported as independent joints (four coordinates where one is right); the NOTICE file removed (the
licence check fails); an altered licence hash. Determinism: goldens are data.

## Considerations

- Whether the passive support is a caster or a skid changes the base's contact model; the manifest records
  what the file has.
- Recalled figures are expectations only; the reading at the pin replaces them.

## References

Appendix G.1; §14.10; §14.15 (GATE-SKEL-01, GATE-SKEL-07); Appendix H.3.
