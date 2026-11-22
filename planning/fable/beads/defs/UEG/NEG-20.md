---
key: "NEG-20"
title: "scene asset source `robocasa` (code MIT, assets CC BY 4.0): kitchen fixtures and layouts, assets only"
type: "task"
priority: 3
milestone: "M5"
labels: ["m5", "habitat", "frob-scene", "frontier", "test", "asset", "registry"]
parent: "WP-32"
deps: ["N19b-02", "N19a-40", "BD-307", "N19a-05", "N19a-07"]
needs: ["§19.13: scene.import for single assets at M3 (decode, compile, loss report, licence manifest entry)"]
related: ["BD-399", "BD-225", "NEG-18", "NEG-19", "NEG-24", "NEG-16"]
gates: []
plan_refs: ["Appendix G.2 (row 4)", "§19.7", "§19.9", "Appendix H.3"]
---
## Background

The commercial kitchen is one of the two interior archetypes the M3 scenarios need first, and no published
generator for commercial kitchens was found. RoboCasa is a published simulation framework of home kitchens
with a large set of fixtures (cabinets, drawers, appliances, counters) and layouts. It is on file with two
licences: its code under MIT and its assets under CC BY 4.0 (FACT from the research pass of 2026-11-21; the
repository URL is in the source register and is re-read at admission). The project uses only the assets
(kitchen fixtures and layouts), never the code: no line of the framework is linked, executed or copied. It is
the one source on file where code and assets have different licences, so the register row and the admission
reading must keep the two apart. It is a Frontier source for populated interiors at M5, funded after the
dressing pilot.

## Scope

In: the register row (assets only); the admission reading; the import profile for articulated kitchen
fixtures and for layouts; validation; `assets/scenes/robocasa/` with its manifest and selection list. Out:
the licence manifest and pack-time checks (N19b-02); the project's own fixture library (N19a-07) and kitchen
proxy (BD-225); populated interiors (BD-399).

## Technical approach

**Register row.** `key = "robocasa"`, `licence = "CC-BY-4.0"` for assets, with a separate field
`code_licence = "MIT"` and `code_used = false`; `class = "attribution"`; `default_pipeline = true` after the
admission reading; `redistribute = "with-attribution"`; `kind = "fixture-and-layout-collection"`;
`use = "kitchen fixtures and layouts"`.

**Admission reading.** Read both licence statements at a pinned release. Record: which files each covers;
whether the asset set bundles third-party objects under other terms (collections often aggregate; any such
subset is excluded and listed); the attribution wording. Confirm in writing that layouts (the arrangement
data) are covered by the asset licence and not the code licence.

**Import profile, fixtures.** `frob scene import --source robocasa --profile kitchen-fixture <asset> --json`:
decode geometry and the articulation description; map each door, drawer and knob to the project's fixture
types (a hinged leaf with limits; a prismatic drawer with travel and a stop; a rotary knob), with joint
axes, ranges and, where the source gives them, friction and closing torque, otherwise `assumed` from the
fixture library's parameter tables; build collision shapes with recorded error; assign interface and optical
classes (stainless steel maps to optical class `mirror`, glass fronts to `glass`); write licence entries with
attribution and change statement.

**Import profile, layouts.** A layout becomes a parameter set for the kitchen grammar (positions and sizes of
counters, appliances and aisles), not a fixed mesh: the grammar then generates the structure, so functional
clearances are exact and the hazard registry's ranges apply. Aisle widths outside the registry's galley range
(0.9 to 1.2 m for the commercial kitchen; a home kitchen is narrower) are tagged with the archetype they
actually fit.

**Validation.** Every articulated fixture opens and closes through its range without self-collision; joint
limits finite; handles reachable (a height inside the task's declared reach heights or tagged); the imported
scene passes scene acceptance; exports embed the attribution.

## Acceptance criteria

- [ ] The admission record separates code and asset licences and lists any excluded third-party subset.
- [ ] No file of the framework's code exists in the tree and no dependency references it (lint).
- [ ] Five fixtures (a hinged cabinet, a drawer stack, an oven door, a dishwasher door, a knob) import as
      typed fixtures with limits and validate; one layout imports as grammar parameters.
- [ ] Exports embed the CC BY attribution and the change statement.

## Test plan

### Unit tests

- `asset_source_robocasa_row_valid`: asset and code licences distinct; `code_used = false`.
- `robocasa_fixture_maps_to_typed_joint`: axes, ranges, stops.
- `robocasa_missing_torque_is_assumed`.
- `robocasa_layout_to_grammar_parameters`: clearances regenerated exactly.
- `robocasa_no_code_in_tree` (lint).
- `robocasa_export_embeds_attribution`.

### End-to-end script and logging

`scripts/e2e/assets_licences.sh` (NEG-16), scene-asset loop at M5: import the five fixtures and one layout,
open and close each fixture on the referee, export; log lines carry asset identifiers, fixture types, joint
ranges, provenance of each parameter, attribution presence.

### Known answers and negative controls

The licence-join rule. Controls: a fixture imported with a door that passes through its frame (validation);
the code licence recorded as the asset licence (admission check); a layout imported as a frozen mesh (the
clearance test detects non-exact aisles). Determinism: same files, same identifiers.

## Considerations

- Home-kitchen assets do not make a commercial kitchen: the commercial archetype's hazards (drains, wet and
  greasy films, swing doors, low shelves, steam) come from the hazard registry and the project's grammar.
- Articulation parameters in public assets are rarely measured; every `assumed` value is counted.

## References

Appendix G.2 (row 4); §19.7; §19.9; Appendix H.3.
