---
key: "ROBOT-fourier-n1"
title: "assets/robots/fourier-n1: Fourier N1 from MuJoCo Menagerie (Apache-2.0): import parity only; conventions"
type: "task"
priority: 2
milestone: "M1"
labels: ["m1", "skeleton", "frob-io", "solid", "test", "asset", "robot"]
parent: "WP-08"
deps: ["BD-066", "BD-067", "N14b-20", "NEG-15"]
needs: []
related: ["ROBOT-op3", "ZOO-humanoid"]
gates: []
plan_refs: ["Appendix G.1", "§14.10", "§14.13 (rejection fixture, joint)", "§14.15 GATE-SKEL-01", "Appendix H.3"]
---
## Background

The Fourier N1 is a mid-size humanoid released as an open platform. Its public MuJoCo model in the Menagerie
collection is on file under Apache-2.0 (FACT from the research pass of 2026-11-21; re-checked at admission)
for import parity only: imported and checked kinematically against the pinned oracle, with no preset, no
mass row and no scorecard. Each parity-only robot is on file to widen the range of authoring conventions the
importer has met. This one is used for the conventions that silently corrupt an import when read wrongly:
angle units, Euler sequences, and how inertias are stated. It is never the source of a Reality claim.

## Scope

In: `assets/robots/fourier-n1/` (manifest with `role = "import-parity-only"`, licence text and NOTICE, pinned
source record, import, loss-report and validation goldens); the convention record. Out: the shared layout and
checks (NEG-15); the importer (BD-066); oracle capture (BD-067).

## Technical approach

**Source record.** MuJoCo Menagerie at the project's single pinned commit; directory expected `fourier_n1/`
(recalled, mark M: confirm at the pin). Licence Apache-2.0: vendoring with the licence text and any upstream
NOTICE, files unmodified.

**Expected facts (recalled, mark M; the pinned files govern).** About 23 actuated joints; total mass on the
order of 38 kg.

**Import path.** `frob design import assets/robots/fourier-n1/source/<entry> --format mjcf --name fourier-n1 --key
asset-fourier-n1-<source hash> --json` (NEG-15's import recipe; the verb writes no file, and the golden `import.fbot` is
the design's `term` section).

**What this robot adds to importer coverage (HYPOTHESIS until the file is read).** The source format lets a
file choose its conventions in a compiler block: angles in degrees or radians, an Euler sequence, whether
inertias are given in full, as diagonal inertia with an orientation, or inferred from geometry and density,
and whether a body's frame is at its joint or its centre of mass. The validation records, for this file, each
convention in force and where the importer read it, and then proves the reading by a metamorphic test: a
`synthetic` rewrite of the same file into the opposite convention (degrees ↔ radians; full inertia ↔ diagonal
with orientation; a different but equivalent Euler sequence) must import to the same `DesignId`. Because the
design language stores exact quantity atoms with units, an angle range read in the wrong unit is a factor of
57.3 and would also show as a joint range wider than a full turn, which the limit check flags.

**Validation specifics.** Counts, mass properties (with inertias inferred from geometry reported as
`inferred`, never as stated), joint limits and limb mirror consistency as for every robot.

## Acceptance criteria

- [ ] `assets/robots/fourier-n1/` passes the licence check (Apache-2.0) and validation; `mark = "P"`.
- [ ] `import.fbot` has a pinned `DesignId`; loss report equals its golden; counts equal the manifest.
- [ ] The convention record lists angle unit, Euler sequence, inertia form and frame convention with the
      source location of each; the three convention rewrites import to the same `DesignId`.
- [ ] The robot appears in no mass row and no scorecard.

## Test plan

### Unit tests

- `robot_fourier_n1_manifest_valid`, `robot_fourier_n1_licence_apache2_notice`.
- `robot_fourier_n1_counts` (as computed at the pin).
- `robot_fourier_n1_conventions_recorded`.
- `robot_fourier_n1_rewrite_degrees_radians_same_id`, `…_inertia_form_same_id`, `…_euler_sequence_same_id`
  (inertia and Euler rewrites compared after canonical rounding of the rewritten decimals; tolerance one unit
  in the last stored digit of the source).
- `robot_fourier_n1_no_range_over_full_turn_unless_continuous`.
- `robot_fourier_n1_mass_properties_consistent`, `robot_fourier_n1_loss_report_golden`,
  `robot_fourier_n1_design_id_pinned`.

### End-to-end script and logging

`scripts/e2e/assets_licences.sh` (NEG-16): import, both checks, golden comparison, the three rewrites; logs
commit, licence, counts, the conventions in force and the identity comparison.

### Known answers and negative controls

The import-parity row (10⁻⁹ m, 10⁻⁹ rad, 10⁴ configurations). Controls: an importer that ignores the angle
unit setting (ranges 57.3 times too wide or narrow; the rejection fixture of the joint family is a prismatic
travel in millimetres read as radians, refused with FROB-REF-102); an altered licence hash. Determinism:
goldens are data.

## Considerations

- Rewrites that change decimal representations cannot always round-trip to identical bits; the comparison is
  on canonical quantity atoms with the stated last-digit tolerance, and a larger difference is a finding.
- Recalled figures are expectations only; the reading at the pin replaces them.

## References

Appendix G.1; §14.10; §14.13; §14.15 (GATE-SKEL-01); Appendix H.3; FROB-REF-102.
