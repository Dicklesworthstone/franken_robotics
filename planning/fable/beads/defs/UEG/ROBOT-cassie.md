---
key: "ROBOT-cassie"
title: "assets/robots/cassie: Cassie from MuJoCo Menagerie (MIT): closed-loop legs, reference for `biped-point`"
type: "task"
priority: 2
milestone: "M3"
labels: ["m3", "skeleton", "frob-io", "solid", "test", "asset", "robot"]
parent: "WP-38"
deps: ["BD-056", "BD-066", "BD-067", "BD-101", "N14b-20", "NEG-15"]
needs: []
related: ["ZOO-biped-point"]
gates: []
plan_refs: ["Appendix G.1", "§14.10", "§14.12", "§14.15 GATE-SKEL-01, GATE-SKEL-07", "Appendix H.3"]
---
## Background

Cassie is a bipedal robot with small feet whose legs contain closed kinematic loops (rods that couple the
knee, shin and tarsus) and leaf springs in series with two joints per leg. Its public MuJoCo model in the
Menagerie collection is on file under the MIT licence (FACT from the research pass of 2026-11-21; re-checked
at admission). It is the only reference robot of the `biped-point` archetype and the only reference robot on
file with loop closures and passive elastic joints, so it is the importer's hardest case: equality constraints
of the source must become `closure` constructors or be reported as losses, never dropped in silence. The
archetype's template is a tree and does not reproduce these loops; only the `@cassie` preset's mass is compared
with this robot. It is never the source of a Reality claim.

## Scope

In: `assets/robots/cassie/` (manifest, licence text, pinned source record, import, loss-report and validation
goldens); the `biped-point@cassie` mass-on-file record; the loop and spring mapping record. Out: the shared
layout and checks (NEG-15); the importer (BD-066); mobility analysis (BD-101); the biped controller (N14b-09).

## Technical approach

**Source record.** MuJoCo Menagerie at the project's single pinned commit; directory expected
`agility_cassie/` (recalled, mark M: confirm at the pin). Licence MIT: vendoring allowed with the copyright
and permission notice included.

**Expected facts (recalled, mark M; the pinned files govern).** 10 actuators (five per leg: hip roll, hip yaw,
hip pitch, knee, toe); additional unactuated joints per leg in the loop (shin, tarsus and rod joints); two
loop closures per leg written as equality constraints; two spring joints per leg with stiffness; total mass on
the order of 32 kg.

**Import path.** `frob design import assets/robots/cassie/source/<entry> --format mjcf --name cassie --key
asset-cassie-<source hash> --json` (NEG-15's import recipe: `design.import` is a `mutate` verb that writes no
file and has no output-path option; the key makes a re-run an idempotent replay with the same `DesignId`). The
golden `assets/robots/cassie/import.fbot` is the design's `term` section (`frob design show <DesignId> --section
term --json`, the canonical JSON bytes), and the loss report is read with `frob artifact read <report> --json`.
Mapping rules exercised here: an `equality` of kind `connect`
between two bodies becomes a `closure` with a spherical or revolute constraint at the stated anchor; a joint
with `stiffness` and `springref` becomes a passive joint with an elastic element and its rest angle; anything
the importer cannot map (for example a constraint's solver softness) is listed in the loss report with the
source's values. Expected loss-report entries (HYPOTHESIS until read): constraint softness parameters; visual
meshes; fitted collision primitives.

**Validation specifics.** (1) Mobility: with the loops closed, the leg's mobility computed by exact rank
equals the count of independent leg coordinates the source implies (actuated joints plus spring deflections);
the number is computed at the pin and stored. (2) Closure residual at the source's reference pose: zero to
10⁻⁹ m. (3) Passive joints are counted as passive, with their stiffness carried. (4) Mass properties and joint
limits as for every robot. (5) Forward-kinematics parity is evaluated on configurations that satisfy the
closures (sampled in independent coordinates and solved for the dependent ones by the oracle), since
arbitrary joint vectors violate the loops.

**Use.** Import parity with closures (a GATE-SKEL-01 row); the mass row of `biped-point@cassie`; a fixture for
loop handling on the referee, which lowers loops as compliant equality rows until the upstream loop-constraint
item lands.

## Acceptance criteria

- [ ] `assets/robots/cassie/` passes the licence check (MIT) and validation; `mark = "P"`.
- [ ] Every equality constraint of the source is a `closure` in the import or an entry of the loss report;
      none is missing from both.
- [ ] Mobility, closure residual and passive-joint counts are recorded and reproduce.
- [ ] The mass on file is written by validation into the manifest field that the `biped-point@cassie` preset row reads; the preset and its
      GATE-SKEL-07 row are N14b-02's, which reads the pinned fixture, so this bead checks the field (present, sourced,
      equal to the validated total) and does not wait on the preset.

## Test plan

### Unit tests

- `robot_cassie_manifest_valid`, `robot_cassie_licence_mit_matches`.
- `robot_cassie_equalities_accounted`: count in the source = closures + loss-report entries.
- `robot_cassie_closure_residual_zero_at_reference`.
- `robot_cassie_mobility_recorded`; `robot_cassie_springs_are_passive_with_stiffness`.
- `robot_cassie_mass_properties_consistent`, `robot_cassie_joint_limits_valid`.
- `robot_cassie_loss_report_golden`, `robot_cassie_design_id_pinned`.

### End-to-end script and logging

`scripts/e2e/assets_licences.sh` (NEG-16): import, both checks, golden comparison; logs commit, licence,
closure and spring counts, mobility, residual, mass and findings.

### Known answers and negative controls

The import-parity row on closure-consistent configurations (10⁻⁹ m, 10⁻⁹ rad). Controls: an importer that
drops `equality` elements (the tree has too many free coordinates, caught by the accounting and mobility
tests); springs imported as rigid joints; an altered licence hash. Determinism: goldens are data.

## Considerations

- Until exact loops exist on the referee, dynamic parity with the oracle is not claimed for this robot; the
  kinematic rows stand alone.
- Recalled figures are expectations only; the reading at the pin replaces them.

## References

Appendix G.1; §14.10; §14.12; §14.15 (GATE-SKEL-01, GATE-SKEL-07); Appendix H.3.
