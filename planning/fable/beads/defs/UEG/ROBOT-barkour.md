---
key: "ROBOT-barkour"
title: "assets/robots/barkour: Barkour from MuJoCo Menagerie (Apache-2.0): `quad-mammal`, import parity only"
type: "task"
priority: 2
milestone: "M1"
labels: ["m1", "skeleton", "frob-io", "solid", "test", "asset", "robot"]
parent: "WP-08"
deps: ["NEG-15", "BD-066", "BD-067", "N14b-20"]
needs: []
related: ["ZOO-quad-mammal", "N14b-24", "ROBOT-go1", "ROBOT-a1"]
gates: []
plan_refs: ["Appendix G.1", "§14.10", "§14.15 GATE-SKEL-01", "Appendix H.3"]
---
## Background

Barkour is a research quadruped built for agile locomotion benchmarks. Its public MuJoCo model in the
Menagerie collection is on file under Apache-2.0 (FACT from the research pass of 2026-11-21; re-checked at
admission), serving the `quad-mammal` archetype for import parity only: it is imported and checked
kinematically against the pinned oracle; no preset of the template targets it, it has no mass row in the
archetype gate, and it is not a benchmark baseline. It is the one quadruped on file under Apache-2.0 (the six
baseline quadrupeds are BSD-3) and comes from a maker other than those of the six baselines, so it checks that the quadruped import path does not depend on one maker's file conventions
and that the licence join handles two licences inside one archetype's reference set. It is never the source
of a Reality claim.

## Scope

In: `assets/robots/barkour/` (manifest with `role = "import-parity-only"`, licence text and NOTICE, pinned
source record, import, loss-report and validation goldens). Out: the shared layout and checks (NEG-15); the
importer (BD-066); oracle capture (BD-067).

## Technical approach

**Source record.** MuJoCo Menagerie at the project's single pinned commit; one or more directories are
expected for successive versions of the robot (recalled, mark M: confirm at the pin); the manifest names the
one entry used, states its version, and lists the others as not used. Licence Apache-2.0: vendoring with the
licence text and any upstream NOTICE, files unmodified.

**Expected facts (recalled, mark M; the pinned files govern).** 12 actuated revolute joints; 13 bodies;
n_v = 18; total mass on the order of 12 kg.

**Import path.** `frob design import --format mjcf assets/robots/barkour/source/<entry> --out
assets/robots/barkour/import.fbot --json`.

**What this robot adds to coverage (HYPOTHESIS until the file is read).** (1) A fourth quadruped skeleton
from a different author: the parameter-stripped term must equal the Unitree quadrupeds' skeleton or differ
only at paths that name a real structural difference (for example a different order of the hip axes).
(2) Licence join: a `synthetic` report that cites one BSD-3 quadruped and this one must carry both licence
entries, both notices, and the stricter of each obligation; a pack containing both exports both texts.
(3) Versioned directories: the index must not treat two versions of one robot as two robots, and the
manifest's `display` carries the version.

**Validation specifics.** Counts, mass properties, joint limits and leg mirror consistency as for every
quadruped; the skeleton comparison; the licence-join fixture.

## Acceptance criteria

- [ ] `assets/robots/barkour/` passes the licence check (Apache-2.0) and validation; `mark = "P"`; the entry's
      version is recorded.
- [ ] `import.fbot` has a pinned `DesignId`; loss report equals its golden; counts equal the manifest.
- [ ] The skeleton comparison with the Unitree quadrupeds is recorded with any differing paths explained.
- [ ] A report citing this robot and a BSD-3 robot carries both licence entries; the robot appears in no mass
      row and no scorecard.

## Test plan

### Unit tests

- `robot_barkour_manifest_valid`, `robot_barkour_licence_apache2_notice`.
- `robot_barkour_counts`: actuated 12, bodies 13, n_v 18 (as computed at the pin).
- `robot_barkour_skeleton_vs_unitree`.
- `robot_barkour_licence_join_two_licences`: the join of {BSD-3-Clause, Apache-2.0} lists both with both
  notices.
- `robot_barkour_single_index_entry_for_versions`.
- `robot_barkour_mass_properties_consistent`, `robot_barkour_joint_limits_valid`,
  `robot_barkour_leg_mirror_consistency`, `robot_barkour_loss_report_golden`, `robot_barkour_design_id_pinned`.

### End-to-end script and logging

`scripts/e2e/assets_licences.sh` (NEG-16): import, both checks, golden comparison, the two-licence report
fixture; logs commit, entry and version, licence, counts, mass, skeleton digest and the joined licence set.

### Known answers and negative controls

The import-parity row (10⁻⁹ m, 10⁻⁹ rad, 10⁴ configurations). Licences over derivation graphs: the licence
set of a derived artifact equals the join of its inputs' (the safety gate's rule). Controls: a report that
drops the Apache-2.0 entry when a BSD-3 entry is present; the robot added to the mass gate by a planted index
edit; an altered licence hash. Determinism: goldens are data.

## Considerations

- If the collection holds several versions with different morphologies, only one is on file here; adding
  another is a new asset with its own bead.
- Recalled figures are expectations only; the reading at the pin replaces them.

## References

Appendix G.1; §14.10; §14.15 (GATE-SKEL-01); Appendix H.3.
