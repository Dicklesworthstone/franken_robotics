---
key: "NEG-15"
title: "assets/robots: layout, manifest `frob.robot.asset/1`, licence check and validation for reference robots"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "skeleton", "safety", "frob-io", "frob-conformance", "solid", "test", "registry"]
parent: "WP-08"
deps: ["BD-066", "BD-065", "BD-067", "N14b-20", "N14a-13", "BD-005", "BD-025"]
needs: ["§37.7: the licence join over derivation graphs (a report or pack carries the join of its inputs' licences), as the rule this manifest feeds"]
related: ["NEG-16", "N14b-24", "N14b-34", "BD-089", "BD-109", "BD-246", "BD-305", "N19b-02"]
gates: []
plan_refs: ["Appendix G (preamble)", "Appendix G.1", "§14.10", "§14.12", "§14.15 GATE-SKEL-01, GATE-SKEL-07", "Appendix H.1", "Appendix H.3 (robot models and scene assets)", "§37"]
---
## Background

Reference robots are imported public models used as parity fixtures and benchmark baselines. They are never
the source of a Reality claim: agreeing with a public model shows that the importer and the dynamics reproduce
a reference tool, nothing about hardware. Nineteen are on file: seventeen from the MuJoCo Menagerie collection
under three licences (BSD-3, MIT, Apache-2.0) and two from the sibling physics library's own robot catalogue
(under the suite's licence, with provenance). Each serves one or more of the eighteen archetypes as a preset
target and a baseline, or serves import parity only. Twelve archetypes have no permissively licensed reference
model on file.

Licences recorded here are FACT from a research pass on 2026-11-21 and are re-checked at admission: the
collection's repository URL is in the source register with a mark that means "fetched; treat as summarized",
so every licence and every per-robot figure is a reading task performed at a pinned commit before anything
cites it. An asset whose licence was not read is marked so and stays outside the default pipeline. This bead
builds what the nineteen robot beads (`ROBOT-<name>`) share: where a robot lives in the repository, what its
manifest says, how its licence is checked, and how its model is validated after import.

## Scope

In: the directory layout; the manifest schema; the licence check; the validation suite (mass properties, joint
limits, counts); the mass-on-file record that the archetype mass gate reads; the catalogue index. Out: the
importers (BD-065, BD-066); oracle fixtures and their capture (BD-067); kinematic and dynamic parity tests
(N14b-24, BD-089); reduced models (N14b-34); each robot's own facts (the `ROBOT-…` beads).

## Technical approach

**Layout (DESIGN).**

```
assets/robots/<name>/manifest.toml     # frob.robot.asset/1 (below)
assets/robots/<name>/LICENSE           # verbatim licence text of the model, as at the pinned commit
assets/robots/<name>/source/           # the model files as fetched (MJCF, meshes), only if redistributable
assets/robots/<name>/import.fbot       # golden: output of `frob design import` on source/, canonical JSON
assets/robots/<name>/loss_report.json  # golden: every dropped or approximated field
assets/robots/<name>/validation.json   # golden: counts, total mass, per-body inertia verdicts, limit table
assets/robots/INDEX.toml               # name → source, licence, archetypes, role, first_milestone, DesignId
```

**Manifest `frob.robot.asset/1`.**

```toml
schema = "frob.robot.asset/1"
name = "go2"                      # key of this robot everywhere
display = "Unitree Go2"
source = { collection = "mujoco_menagerie", repository = "<URL from the source register>", commit = "<40 hex>",
           path = "<directory at that commit>", entry = "<model file>" , read_on = "<UTC date>", mark = "P" }
licence = { spdx = "BSD-3-Clause", text_sha256 = "<hash of LICENSE>", copyright = "<holder line>",
            redistribution = "allowed-with-notice", notice_required = true, read_on = "<UTC date>" }
role = "parity-and-baseline"      # or "import-parity-only", or "sibling-catalogue"
archetypes = ["quad-mammal"]
facts = { actuated = 12, bodies = 13, n_v = 18, mass_kg = "<decimal string>", actuators = "<kind and count>" }
mass_on_file = { value_kg = "<decimal string>", from = "sum of body masses at the pinned commit", read_on = "<date>" }
reality_claim = "never"
```

Every entry of `facts` and `mass_on_file` is computed from the files at the pinned commit by the validation
step and written by it; a human-typed figure without `read_on` is a validation error. Figures recalled from
memory may appear in a robot bead as expectations, labelled as such; they never enter the manifest.

**Licence check (`robot_licence_check`), per robot.** (1) `LICENSE` exists and its hash equals
`text_sha256`. (2) The text matches the SPDX identifier by the standard licence-matching rule (normalized
whitespace, the template's replaceable fields ignored). (3) The identifier is on the admitted list
{BSD-3-Clause, MIT, Apache-2.0, the suite's own licence}; anything else, or `licence = "not-read"`, sets
`default_pipeline = false` and the robot is skipped by every default battery. (4) If `source/` is committed,
`redistribution` must allow it and the notice requirements are met (the licence file and copyright line travel
with the files; for Apache-2.0 a NOTICE file if upstream has one). (5) The entry is exported into the licence
join: any pack, fixture or report that contains this robot's geometry or parameters carries its licence entry,
and an export that would violate a term is refused at pack time. (6) `mark` must be `P` (read from the primary
text at a tag or commit) before a gate cites the robot; otherwise the robot's rows are `not citable`.

**Validation (`robot_validate`), per robot, on the imported design.**

1. Counts: actuated coordinates, bodies, generalized velocities, end effectors, loops; written to
   `validation.json` and compared with the golden.
2. Mass properties: every body has mass > 0 or is declared `massless` with a reason; every body's
   pseudo-inertia is positive semidefinite by exact LDLᵀ on the imported rationals (equivalently: principal
   moments positive and satisfying the triangle inequalities); the centre of mass lies inside the body's
   bounding box inflated by 10%; violations are listed with the offending pivot (the refusal a native design
   would get is FROB-REF-104). Imported models are not silently repaired: a violation is a finding in the loss
   report, and the robot is usable only with an explicit, recorded repair edit.
3. Total mass: the sum of body masses in exact decimal, written to `mass_on_file`.
4. Joint limits: every limited joint has lower < upper, both finite; continuous joints are declared so;
   velocity and effort limits are present or reported missing; an empty range would be FROB-REF-107 in a
   native design and is a finding here.
5. Loss report: every source field the importer did not carry is listed by a field-coverage diff; the golden
   pins the list so that an importer change that drops more is visible.
6. Identity: `import.fbot` has a pinned `DesignId`, identical on Linux and macOS.

**Catalogue index.** `assets/robots/INDEX.toml` has exactly the nineteen names; the archetype → robots mapping
derived from it equals the mapping on file: `quad-mammal`: go1, go2, a1, anymal-b, anymal-c, spot (and barkour
for import parity only); `humanoid`: h1, g1, fsim-g1 (and booster-t1, apollo, fourier-n1, op3, talos for
import parity only); `biped-point`: cassie; `diff-drive`: stretch3 (base); `mecanum`: tiago (omni base);
`mobile-manipulator`: iiwa7 (arm), stretch3, tiago.

## Acceptance criteria

- [ ] Schema, licence check and validation exist and pass on a `synthetic` robot directory; each rule has a
      failing fixture (wrong hash, licence not on the list, `not-read`, human-typed mass, non-positive inertia,
      empty joint range, missing loss-report golden).
- [ ] `INDEX.toml` lists nineteen robots at M3 (those due at M1 at M1) and reproduces the archetype mapping.
- [ ] No test, fixture or report labels a result on a reference robot as Reality; a planted Parity result
      offered as Reality is refused.
- [ ] A robot with `licence = "not-read"` is skipped by default batteries and listed as skipped.

## Test plan

### Unit tests

- `robot_manifest_schema`, `robot_manifest_rejects_typed_fact`.
- `robot_licence_hash_mismatch`, `robot_licence_not_admitted`, `robot_licence_not_read_skipped`,
  `robot_notice_required_missing`.
- `robot_validate_inertia_triangle_violation`: principal moments (1, 1, 3) are reported with the pivot.
- `robot_validate_joint_range_empty`, `robot_validate_counts_golden`.
- `robot_index_mapping_matches_file`.
- `robot_reality_claim_refused`.

### End-to-end script and logging

`scripts/e2e/assets_licences.sh` (NEG-16) drives `frob design import` and the checks for every robot directory
and logs manifest fields, licence verdicts, validation results and the loss-report diff.

### Known answers and negative controls

The import-parity row of the design-language gates (forward kinematics within 10⁻⁹ m and 10⁻⁹ rad over 10⁴
configurations against the pinned oracle; a loss report listing every dropped field) is measured elsewhere on
these assets. This bead's controls are the failing fixtures above. Determinism: validation is exact-rational
and its JSON is byte-identical between runs and platforms.

## Considerations

- A known risk: reference-robot imports hide lossy contact mappings (warning sign: fields missing from a loss
  report). The loss-report golden is the guard; contact parity with an imported contact model is labelled
  parity-only by its owner.
- Vendoring `source/` is convenient and adds licence obligations; a robot may instead keep only the manifest
  and a fetch script that verifies the commit and file hashes, and the manifest records which.
- Upstream models change; the pin is a commit, and moving it is a deliberate event that regenerates the three
  goldens in one change.

## References

Appendix G (preamble), G.1; §14.10; §14.12; §14.15 (GATE-SKEL-01, GATE-SKEL-07); Appendix H.1, H.3; §37.
