---
key: "ROBOT-anymal-c"
title: "assets/robots/anymal-c: ANYmal C from MuJoCo Menagerie (BSD-3): heaviest baseline for `quad-mammal`"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "skeleton", "frob-io", "solid", "test", "asset", "robot"]
parent: "WP-08"
deps: ["BD-066", "BD-067", "BD-109", "N14b-20", "NEG-15", "ROBOT-anymal-b"]
needs: []
related: ["ZOO-quad-mammal"]
gates: []
plan_refs: ["Appendix G.1", "§14.10", "§14.15 GATE-SKEL-01, GATE-SKEL-07", "Appendix H.3"]
---
## Background

ANYmal C is the industrial successor of ANYmal B and the heaviest quadruped on file. It is widely used in
published rough-terrain locomotion results, so it is a natural second baseline for the terrain task family,
and it fixes the upper end of the mass range the `quad-mammal` template must reach by parameters. Its public
MuJoCo model in the Menagerie collection is on file under BSD-3 (FACT from the research pass of 2026-11-21;
re-checked at admission). It is a parity fixture and baseline, never the source of a Reality claim.

## Scope

In: `assets/robots/anymal-c/` (manifest, licence text, pinned source record, import, loss-report and
validation goldens) and the `quad-mammal@anymal-c` mass-on-file record. Out: the shared layout and checks
(NEG-15); the importer (BD-066); oracle capture (BD-067); the terrain task family (BD-201).

## Technical approach

**Source record.** MuJoCo Menagerie at the project's single pinned commit; directory expected
`anybotics_anymal_c/` (recalled, mark M: confirm at the pin). Licence BSD-3-Clause: vendoring allowed with
notice, licence text and the no-endorsement clause.

**Expected facts (recalled, mark M; the pinned files govern).** 12 actuated revolute joints; 13 bodies;
n_v = 18; total mass on the order of 50 kg; actuator force ranges stated per joint; an X leg configuration.

**Import path.** `frob design import assets/robots/anymal-c/source/<entry> --format mjcf --name anymal-c --key
asset-anymal-c-<source hash> --json` (NEG-15's import recipe; the verb writes no file, and the golden `import.fbot` is
the design's `term` section). Expected loss-report entries (HYPOTHESIS until read): visual
meshes; fitted collision primitives with recorded error (this model's collision geometry is likely more
detailed than the small quadrupeds', so the fitting error is reported per body and the worst is flagged if
above 5 mm); unmapped solver and contact parameters. Not losses, by BD-066's table: actuator force ranges
(overrides on the bound `assumed` card), `armature` (an override of that card's rotor inertia), and joint
`damping` and `frictionloss` (the joint's own damping and friction).

**Validation specifics.** Mass ratio check against `anymal-b`: both imports are compared body by body (trunk,
hip, thigh, shank) and the ratios are recorded; they feed a property test of the template: scaling the
`quad-mammal` template by its declared levers from the `@anymal-b` preset toward `@anymal-c` must change the
compiled mass monotonically. Inertia consistency is checked body by body as for every robot; a heavy trunk
with sensors and batteries modelled as one body is where public models most often carry an inertia that
violates the triangle inequalities, and such a finding is reported with its pivot, not repaired.

**Use.** Import parity and contact-free dynamics parity rows; the mass row of `quad-mammal@anymal-c`; a second
reference scorecard for terrain traversal at M2.

## Acceptance criteria

- [ ] `assets/robots/anymal-c/` passes the licence check and validation; `mark = "P"` with commit and date.
- [ ] `import.fbot` has a pinned `DesignId`; loss report equals its golden with per-body fitting errors.
- [ ] Body-by-body mass ratios against `anymal-b` are recorded; the template's monotonicity test passes.
- [ ] The mass on file is written by validation into the manifest field that the `quad-mammal@anymal-c` preset row reads; the preset and its
      GATE-SKEL-07 row are BD-109's, which reads the pinned fixture, so this bead checks the field (present, sourced,
      equal to the validated total) and does not wait on the preset.

## Test plan

### Unit tests

- `robot_anymal_c_manifest_valid`, `robot_anymal_c_licence_bsd3_matches`.
- `robot_anymal_c_counts`: actuated 12, bodies 13, n_v 18 (as computed at the pin).
- `robot_anymal_c_mass_properties_consistent`, `robot_anymal_c_joint_limits_valid`.
- `robot_anymal_c_collision_fit_error_reported`: every fitted primitive has a recorded error.
- `robot_anymal_b_to_c_template_monotone`.
- `robot_anymal_c_loss_report_golden`, `robot_anymal_c_design_id_pinned`.

### End-to-end script and logging

`scripts/e2e/assets_licences.sh` (NEG-16): import, both checks, golden comparison; logs commit, licence,
counts, mass, worst fitting error in millimetres and findings.

### Known answers and negative controls

The import-parity row (10⁻⁹ m, 10⁻⁹ rad, 10⁴ configurations). Controls: a fitted primitive with its error
field omitted; an inertia violation silently repaired by the importer (the test asserts a finding instead);
an altered licence hash. Determinism: goldens are data.

## Considerations

- A 50 kg quadruped changes actuator sizing: the mass spiral's loop gain for this preset is reported by the
  co-design closure analysis and may be close to 1 for long-legged variants.
- Recalled figures are expectations only; the reading at the pin replaces them.

## References

Appendix G.1; §14.10; §14.15 (GATE-SKEL-01, GATE-SKEL-07); Appendix H.3.
