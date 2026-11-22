---
key: "ROBOT-spot"
title: "assets/robots/spot: Spot from MuJoCo Menagerie (BSD-3): parity fixture and baseline for `quad-mammal`, legs only"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "skeleton", "frob-io", "solid", "test", "asset", "robot"]
parent: "WP-08"
deps: ["NEG-15", "BD-066", "BD-067", "N14b-20"]
needs: []
related: ["ZOO-quad-mammal", "ZOO-mobile-manipulator", "BD-109", "N14b-24", "BD-089"]
gates: []
plan_refs: ["Appendix G.1", "§14.10", "§14.15 GATE-SKEL-01, GATE-SKEL-07", "Appendix H.3"]
---
## Background

Spot is the most widely deployed commercial quadruped and the sixth reference robot of the `quad-mammal`
archetype. Its public MuJoCo model in the Menagerie collection is on file under BSD-3 (FACT from the research
pass of 2026-11-21; re-checked at admission). The model's licence covers the model files; it grants nothing
about the maker's name or marks, so assets, fixtures and reports refer to it by the neutral key `spot` and
state the model's source. It is a parity fixture and baseline, never the source of a Reality claim.

## Scope

In: `assets/robots/spot/` (manifest, licence text, pinned source record, import, loss-report and validation
goldens) for the legs-only model, and the `quad-mammal@spot` mass-on-file record. Out: any arm variant (it
would serve `mobile-manipulator` and is not on file for that purpose); the shared layout and checks (NEG-15);
the importer (BD-066); oracle capture (BD-067).

## Technical approach

**Source record.** MuJoCo Menagerie at the project's single pinned commit; directory expected
`boston_dynamics_spot/` (recalled, mark M: confirm at the pin; if the directory holds a legs-only file and a
file with an arm, the manifest names the legs-only entry and lists the other as not used). Licence
BSD-3-Clause: vendoring allowed with notice, licence text and the no-endorsement clause; the no-endorsement
clause is the reason the neutral key is used.

**Expected facts (recalled, mark M; the pinned files govern).** 12 actuated revolute joints; 13 bodies;
n_v = 18; total mass on the order of 32 kg; knee joints with a wide range driven through a linkage on the real
machine and modelled as a plain revolute joint in the public file.

**Import path.** `frob design import --format mjcf assets/robots/spot/source/<entry> --out
assets/robots/spot/import.fbot --json`. Expected loss-report entries (HYPOTHESIS until read): visual meshes;
fitted collision primitives with recorded error; unmapped solver and contact parameters; `armature`,
`damping`, `frictionloss` to an `assumed` card. If the public model states a position-dependent knee torque
limit or gear ratio, it is carried as a transmission modulus table; if it states a constant, the constant is
carried and the loss report notes that the real knee's ratio varies with angle (HYPOTHESIS; decided by
reading the file).

**Validation specifics.** Joint ranges: the knee's wide range is checked against self-collision of the shank
with the body over the declared range at the reference hip pose, and any interpenetration is a finding (a
native design would be refused with FROB-REF-123). Naming: a test asserts that no generated artifact name,
report title or fixture path contains the maker's name; the manifest's `display` field carries the model's
own title for attribution only.

**Use.** Import parity and contact-free dynamics parity rows; the mass row of `quad-mammal@spot`.

## Acceptance criteria

- [ ] `assets/robots/spot/` passes the licence check and validation; `mark = "P"` with commit and date; the
      entry file is the legs-only model.
- [ ] `import.fbot` has a pinned `DesignId`; loss report equals its golden; counts equal the manifest.
- [ ] Knee-range self-collision findings are recorded; the neutral-naming test passes.
- [ ] The mass on file is written by validation and consumed by the `quad-mammal@spot` preset row.

## Test plan

### Unit tests

- `robot_spot_manifest_valid`, `robot_spot_licence_bsd3_matches`.
- `robot_spot_counts`: actuated 12, bodies 13, n_v 18 (as computed at the pin).
- `robot_spot_mass_properties_consistent`, `robot_spot_joint_limits_valid`.
- `robot_spot_knee_range_self_collision_reported`.
- `robot_spot_neutral_naming`: generated names and paths use the key only.
- `robot_spot_loss_report_golden`, `robot_spot_design_id_pinned`.

### End-to-end script and logging

`scripts/e2e/assets_licences.sh` (NEG-16): import, both checks, golden comparison; logs commit, entry file,
licence, counts, mass and findings.

### Known answers and negative controls

The import-parity row (10⁻⁹ m, 10⁻⁹ rad, 10⁴ configurations). Controls: the arm variant imported in place of
the entry (counts differ from the manifest); a report title carrying the maker's name (naming test); an
altered licence hash. Determinism: goldens are data.

## Considerations

- An arm variant, if wanted later as a `mobile-manipulator` reference, is a separate asset with its own bead
  and pin.
- Recalled figures are expectations only; the reading at the pin replaces them.

## References

Appendix G.1; §14.10; §14.15 (GATE-SKEL-01, GATE-SKEL-07); Appendix H.3.
