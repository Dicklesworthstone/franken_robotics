---
key: "ROBOT-apollo"
title: "assets/robots/apollo: Apptronik Apollo from MuJoCo Menagerie (Apache-2.0): import parity only; size stress"
type: "task"
priority: 2
milestone: "M1"
labels: ["m1", "skeleton", "frob-io", "solid", "test", "asset", "robot"]
parent: "WP-08"
deps: ["NEG-15", "BD-066", "BD-067", "N14b-20"]
needs: []
related: ["ZOO-humanoid", "N14b-24", "BD-307", "ROBOT-booster-t1", "ROBOT-talos"]
gates: []
plan_refs: ["Appendix G.1", "§14.10", "§14.15 GATE-SKEL-01", "Appendix H.3", "§37 (importer limits)"]
---
## Background

Apollo is a full-size general-purpose humanoid. Its public MuJoCo model in the Menagerie collection is on
file under Apache-2.0 (FACT from the research pass of 2026-11-21; re-checked at admission) for import parity
only: it is imported and checked kinematically against the pinned oracle; no preset of the `humanoid`
template targets it, it has no mass row and no scorecard. Among the five parity-only humanoids it is expected
to be one of the largest models by joint count and by mesh data, which makes it the natural fixture for the
importer's resource limits and for import time. It is never the source of a Reality claim.

## Scope

In: `assets/robots/apollo/` (manifest with `role = "import-parity-only"`, licence text and NOTICE, pinned
source record, import, loss-report and validation goldens); the import resource record. Out: the shared
layout and checks (NEG-15); the importer and its limits (BD-066, and the importer fuzzing and limits of the
safety work); oracle capture (BD-067).

## Technical approach

**Source record.** MuJoCo Menagerie at the project's single pinned commit; directory expected
`apptronik_apollo/` (recalled, mark M: confirm at the pin). Licence Apache-2.0: vendoring with the licence
text and any upstream NOTICE, files unmodified.

**Expected facts (recalled, mark M; the pinned files govern).** On the order of 30 or more actuated joints
(legs, a multi-joint torso, arms with wrists, a neck); total mass on the order of 70 kg; many visual and
collision meshes.

**Import path.** `frob design import --format mjcf assets/robots/apollo/source/<entry> --out
assets/robots/apollo/import.fbot --json`.

**What this robot adds to importer coverage (HYPOTHESIS until the file is read).** Size. The importer has
declared limits (maximum file size, element count, nesting depth, mesh vertex count, total mesh bytes) and
must refuse beyond them with a clear message rather than exhaust memory; a legitimate large model must stay
inside them. This asset's `validation.json` therefore records the import's resource use: source bytes,
element count, maximum nesting depth, mesh count and vertices, peak memory and wall time on the reference
host. Targets (TARGET): inside every declared limit with at least a factor of two to spare; wall time under
5 s cold. If the generalized-velocity count exceeds the largest default size bucket (64), the manifest says
so and the robot is a fixture for the over-capacity refusal of the batched tiers, not a batched workload.

**Validation specifics.** Counts, mass properties, joint limits and limb mirror consistency as for every
robot; the fitted-primitive error per body, with the worst reported.

## Acceptance criteria

- [ ] `assets/robots/apollo/` passes the licence check (Apache-2.0) and validation; `mark = "P"`.
- [ ] `import.fbot` has a pinned `DesignId`; loss report equals its golden; counts equal the manifest.
- [ ] The resource record exists and every figure is inside the importer's declared limits by a factor ≥ 2.
- [ ] The robot appears in no mass row and no scorecard.

## Test plan

### Unit tests

- `robot_apollo_manifest_valid`, `robot_apollo_licence_apache2_notice`.
- `robot_apollo_counts` (as computed at the pin).
- `robot_apollo_mass_properties_consistent`, `robot_apollo_joint_limits_valid`,
  `robot_apollo_limb_mirror_consistency`.
- `robot_apollo_import_within_limits`: every resource figure against its limit.
- `robot_apollo_bucket_or_refusal`: n_v ≤ 64 selects a bucket; otherwise the batched admission refuses by
  name.
- `robot_apollo_loss_report_golden`, `robot_apollo_design_id_pinned`.

### End-to-end script and logging

`scripts/e2e/assets_licences.sh` (NEG-16): import with timing, both checks, golden comparison; logs commit,
licence, counts, mass, resource figures against limits, wall time and findings.

### Known answers and negative controls

The import-parity row (10⁻⁹ m, 10⁻⁹ rad, 10⁴ configurations). Controls: a `synthetic` copy of the model with
one mesh duplicated until it exceeds the mesh-byte limit is refused with the limit named; an altered licence
hash. Determinism: goldens are data; resource figures are measurements and are compared against limits, not
for equality.

## Considerations

- Wall time and memory are measurements on the reference host and are logged, not gated, beyond the limits.
- Recalled figures are expectations only; the reading at the pin replaces them.

## References

Appendix G.1; §14.10; §14.15 (GATE-SKEL-01); Appendix H.3; §37.
