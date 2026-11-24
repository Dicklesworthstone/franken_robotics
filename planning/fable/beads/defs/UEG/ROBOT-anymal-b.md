---
key: "ROBOT-anymal-b"
title: "assets/robots/anymal-b: ANYmal B from MuJoCo Menagerie (BSD-3): mid-size baseline for `quad-mammal`"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "skeleton", "frob-io", "solid", "test", "asset", "robot"]
parent: "WP-08"
deps: ["BD-066", "BD-067", "BD-102", "BD-109", "N14b-20", "NEG-15"]
needs: []
related: ["ROBOT-anymal-c", "ZOO-quad-mammal"]
gates: []
plan_refs: ["Appendix G.1", "§14.10", "§14.15 GATE-SKEL-01, GATE-SKEL-07", "Appendix H.3"]
---
## Background

ANYmal B is a research quadruped roughly twice the mass of the small Unitree machines, driven by series-
elastic actuators. On file it widens the mass range that the `quad-mammal` presets must span: one template
has to approximate a 12 kg and a 30 kg robot by parameters alone, which tests that the template's levers
(link lengths, fills, actuator cards) are sufficient and that the mass gate is evaluated per preset. Its
public MuJoCo model in the Menagerie collection is on file under BSD-3 (FACT from the research pass of
2026-11-21; re-checked at admission). It is a parity fixture and baseline, never the source of a Reality
claim.

## Scope

In: `assets/robots/anymal-b/` (manifest, licence text, pinned source record, import, loss-report and
validation goldens) and the `quad-mammal@anymal-b` mass-on-file record. Out: the shared layout and checks
(NEG-15); the importer (BD-066); oracle capture (BD-067).

## Technical approach

**Source record.** MuJoCo Menagerie at the project's single pinned commit; directory expected
`anybotics_anymal_b/` (recalled, mark M: confirm at the pin). Licence BSD-3-Clause: vendoring allowed with
notice, licence text and the no-endorsement clause.

**Expected facts (recalled, mark M; the pinned files govern).** 12 actuated revolute joints (hip abduction,
hip flexion, knee per leg); 13 bodies; n_v = 18; total mass on the order of 30 kg; legs in an X configuration
(front knees bend backwards, rear knees forwards), unlike the Unitree layout.

**Import path.** `frob design import assets/robots/anymal-b/source/<entry> --format mjcf --name anymal-b --key
asset-anymal-b-<source hash> --json` (NEG-15's import recipe; the verb writes no file, and the golden `import.fbot` is
the design's `term` section). Expected loss-report entries (HYPOTHESIS until read): visual
meshes; fitted collision primitives with recorded error; unmapped solver and contact parameters. Not losses,
by BD-066's table: joint `damping` and `frictionloss` (the joint's own damping and friction) and `armature` (an
override of the bound `assumed` card's rotor inertia). The source models a rigid transmission: the
series elasticity of the real actuator is absent from the public model, so the import has no compliant
element and the loss report must not claim one; an elastic element can only come from a calibrated card.

**Validation specifics.** The knee configuration changes the mirror structure: the transverse mirror maps a
front leg to a rear leg only if the knee directions are mirror images, which in the X configuration they are.
The validation reports the computed Γ_mech of the import (expected C₂ × C₂ if masses and inertias respect
both mirrors, C₂ otherwise) with the breaking element named.

**Use.** Import parity and contact-free dynamics parity rows; the mass row of `quad-mammal@anymal-b`
(a preset near the upper end of the template's range).

## Acceptance criteria

- [ ] `assets/robots/anymal-b/` passes the licence check and validation; `mark = "P"` with commit and date.
- [ ] `import.fbot` has a pinned `DesignId`; loss report equals its golden; counts equal the manifest.
- [ ] The computed Γ_mech of the import is recorded with its breaker, if any.
- [ ] The preset `quad-mammal@anymal-b` reaches the mass window using declared levers only.

## Test plan

### Unit tests

- `robot_anymal_b_manifest_valid`, `robot_anymal_b_licence_bsd3_matches`.
- `robot_anymal_b_counts`: actuated 12, bodies 13, n_v 18 (as computed at the pin).
- `robot_anymal_b_mass_properties_consistent`, `robot_anymal_b_joint_limits_valid`.
- `robot_anymal_b_symmetry_reported`: Γ_mech of the import and its breaker.
- `robot_anymal_b_no_invented_compliance`: the imported design has no elastic transmission element.
- `robot_anymal_b_loss_report_golden`, `robot_anymal_b_design_id_pinned`.

### End-to-end script and logging

`scripts/e2e/assets_licences.sh` (NEG-16): import, both checks, golden comparison; logs commit, licence,
counts, mass, the computed group and findings.

### Known answers and negative controls

The import-parity row (10⁻⁹ m, 10⁻⁹ rad, 10⁴ configurations). Controls: an importer that adds a default
spring to every actuator (caught by the no-compliance test); an altered licence hash. Determinism: goldens are
data.

## Considerations

- Series elasticity matters for this robot's real behaviour; the public model omits it, which is one more
  reason nothing here supports a Reality claim.
- Recalled figures are expectations only; the reading at the pin replaces them.

## References

Appendix G.1; §14.10; §14.15 (GATE-SKEL-01, GATE-SKEL-07); Appendix H.3.
