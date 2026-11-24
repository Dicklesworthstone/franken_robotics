---
key: "NEG-20"
title: "scene asset source `robocasa` (code MIT, assets CC BY 4.0): kitchen fixtures and layouts, assets only"
type: "task"
priority: 3
milestone: "M5"
labels: ["m5", "habitat", "frob-scene", "frontier", "test", "asset", "registry", "review-notes-open"]
parent: "WP-32"
deps: ["BD-307", "N19a-05", "N19a-07", "N19a-40", "N19a-46", "N19b-02", "N37-18", "NAB-13"]
needs: []
related: ["NEG-24", "NEG-27", "NEGp1-54"]
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

**Import profile, fixtures.** `frob --json scene import --kind asset <asset> --source robocasa --profile
kitchen-fixture --behaviour articulated --key <k>` (the verb and its inputs are N19a-46's: `kind` is required, a
`run` verb takes an idempotency key, and a part that can move needs its declared behaviour; this bead adds the two
import profiles `kitchen-fixture` and `kitchen-layout`, recorded in `registries/import-profiles.toml`). The
import: decode geometry and the articulation description; map each door, drawer and knob to the project's fixture
types (a hinged leaf with limits; a prismatic drawer with travel and a stop; a rotary knob), with joint
axes, ranges and, where the source gives them, friction and closing torque, otherwise `assumed` from the
fixture library's parameter tables; build collision shapes with recorded error; assign interface and optical
classes (stainless steel maps to optical class `mirror`, glass fronts to `glass`); write licence entries with
attribution and change statement.

**Import profile, layouts.** `frob --json scene import --kind asset <layout> --source robocasa --profile
kitchen-layout --key <k>`. A layout becomes a parameter set for the kitchen grammar (positions and sizes of
counters, appliances and aisles), not a fixed mesh: the grammar then generates the structure, so functional
clearances are exact and the hazard registry's ranges apply. The import's result carries the parameter set (with
its digest, the source key and the licence entry of the layout) and the pack the grammar generated from it, whose
provenance cites the layout; a layout whose geometry cannot be read as counters, appliances and aisles is refused
with the offending element named, never imported as a mesh. Aisle widths outside the registry's galley range
(0.9 to 1.2 m for the commercial kitchen; a home kitchen is narrower) are tagged with the archetype they
actually fit.

**Validation.** Every articulated fixture opens and closes through its range without self-collision. The
profile sweeps each joint kinematically from its lower to its upper limit and back, in steps of at most 1° for a
hinged leaf or a knob and at most 5 mm for a drawer, against the fitted collision shapes of the fixture's own parts
and frame; an interpenetration deeper than the recorded fitting error refuses the import with the part and the
joint value named, and each jointed part's sweep (joint, range, step, minimum clearance, result) is written beside
the loss report's per-part articulation row (N19a-46), so the import envelope carries it. Joint
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
- `robocasa_fixture_sweep_through_range`: a hinged leaf swept at 1° steps and a drawer at 5 mm steps report their
  minimum clearance per part; the door that passes through its frame is refused with the part and joint value.
- `robocasa_layout_to_grammar_parameters`: clearances regenerated exactly.
- `robocasa_no_code_in_tree` (lint).
- `robocasa_export_embeds_attribution`.

### End-to-end script and logging

`scripts/e2e/assets_licences.sh` (the M5 scene-asset loop, owned and run by NEGp1-54, which depends on this
bead; this bead lists the steps of its row), every `run` verb call with its own `--key`: the import of the five
fixtures as written above (`--key al-rbc-f<n>`) and of one layout (`--key al-rbc-l1`); a fixture whose door passes
through its frame (`--key al-rbc-x1`; the sweep refuses it, naming the part and the joint value); each fixture's
sweep read from its import envelope (every jointed part's row passed, at most 1° or 5 mm per step); after
`e2e_scene_ready <pack>`, a referee run in each fixture pack (`frob --json sim run quad-a --scene <pack> --tier
referee --seconds 2 --key al-rbc-run-<n>`) in which no fixture joint leaves its range; `frob --json scene show <layout pack>` (the aisle widths equal the parameter set's);
`frob --json artifact export <pack> --out exports/ --key al-rbc-exp` (attribution and change statement embedded).
Log lines carry `ts`, `script`, `bead: "NEG-20"`, `step`, `event`, the exact command, exit code, wall time, the
`evidence` block, asset identifiers, fixture types, joint ranges, provenance of each parameter, attribution
presence, each refusal code with its exit code, and no geometry.

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
