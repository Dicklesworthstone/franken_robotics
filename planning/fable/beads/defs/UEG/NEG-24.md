---
key: "NEG-24"
title: "scene asset source `partnet-mobility` (non-commercial research): user-supplied articulated objects only"
type: "task"
priority: 2
milestone: "M3"
labels: ["m3", "habitat", "safety", "frob-scene", "solid", "test", "asset", "registry"]
parent: "WP-23"
deps: ["N19b-02", "BD-305", "N19a-07", "NEG-22"]
needs: ["§19.13: scene.import for user-supplied articulated assets (joint mapping, loss report)"]
related: ["NEG-20", "NEG-23", "NEG-25", "BD-229", "NEG-16"]
gates: []
plan_refs: ["Appendix G.2 (row 8)", "§19.9", "§37", "FROB-REF-310"]
---
## Background

PartNet-Mobility is a published collection of articulated household objects (cabinets, drawers, doors,
appliances, taps) with part-level joint annotations, distributed under non-commercial research terms (FACT
from the research pass of 2026-11-21; re-checked at admission; no URL for it is in the source register, so
the admission reading starts by recording one). It is supported only as a user-supplied import and is never
redistributed. Its distinguishing content is articulation: each object comes with joints, axes and limits,
which is exactly what scene fixtures need (a robot opens a drawer, pushes a door) and exactly what scanned or
purely visual collections lack. Its articulation annotations are kinematic, though: they say how parts may
move, not what it takes to move them.

## Scope

In: the register row; the user-supplied import profile that maps annotated joints to the project's fixture
types; the rule for missing dynamics parameters; containment; a `synthetic` stand-in. Out: the licence
machinery (N19b-02); the fixture library and its parameter tables (N19a-07); private store and purge
(BD-305); the shared research-only containment rules (NEG-22).

## Technical approach

**Register row.** `key = "partnet-mobility"`, `licence = "non-commercial-research-terms"`,
`class = "research-only"`, `default_pipeline = false`, `redistribute = "never"`,
`kind = "articulated-object-collection"`, `use = "user-supplied import only"`, `url_recorded = false` until
the admission reading records the primary host.

**Import.** `frob scene import --source partnet-mobility --user-supplied --licence-ack <statement> <object>
--json`, refused in default mode. Mapping:

| Source annotation | Fixture mapping | Parameters |
|---|---|---|
| Revolute joint with limits | Hinged leaf (door, lid, oven door) | Axis and range from the source; closing torque, friction and latch force `assumed` from the fixture library's table for the object category |
| Prismatic joint with limits | Drawer or sliding door | Axis and travel from the source; rail friction and end-stop stiffness `assumed` |
| Continuous revolute | Knob, wheel, tap handle | Axis from the source; detents and resistance `assumed` |
| Fixed | Folded into the parent by the pseudo-inertia sum | — |
| Screw or other | Not mapped: listed in the loss report; the part is frozen | — |

Masses: the source has none; each part gets mass from volume and a density `assumed` per category, inertia
from the solid part mesh. Collision: per-part convex decomposition with recorded error. Units and scale: the
collection's objects are normalized, so the import requires `--scale` or a target dimension from the user and
records the scale label as `user-declared`; without it the import refuses.

**Validation.** Each joint sweeps its range without the parts interpenetrating beyond the recorded collision
error; limits finite; the count of `assumed` parameters is written to the pack; a strict-data analysis that
needs closing torque refuses on these objects.

**Containment.** As for every research-only source: private pack, entries with redistribution forbidden,
export of the pack, variants, stills and reports containing them refused with FROB-REF-310, purge with
tombstones.

**Stand-in.** A two-drawer cabinet with a hinged door authored in the repository, tagged `partnet-mobility`
in a scratch register.

## Acceptance criteria

- [ ] The register row exists; the admission reading records the primary host and the terms' text hash.
- [ ] The stand-in imports with one hinged leaf and two drawers as typed fixtures; every dynamic parameter is
      `assumed` and counted.
- [ ] Import without a scale is refused; a screw joint is frozen and listed in the loss report.
- [ ] Every export path is refused with FROB-REF-310.

## Test plan

### Unit tests

- `asset_source_partnet_row_valid`.
- `partnet_joint_mapping_table`: one test per row of the mapping.
- `partnet_import_requires_scale`.
- `partnet_dynamics_parameters_assumed_and_counted`.
- `partnet_sweep_no_interpenetration`.
- `partnet_export_paths_refused_310`, `partnet_no_real_data_in_tree`.

### End-to-end script and logging

`scripts/e2e/assets_licences.sh` (NEG-16), scene-asset loop at M3, on the stand-in: refused default import,
user-supplied import with a scale, sweeps on the referee, strict-data refusal, export attempts; log lines
carry fixture types, ranges, the `assumed` count and refusal codes.

### Known answers and negative controls

The mapping table is the known answer for the stand-in. Controls: closing torque reported as `measured`; a
normalized object imported at unit scale without a declaration; a frozen screw joint reported as mapped.
Determinism: same files and scale, same pack identity.

## Considerations

- The project's own fixture library covers doors, drawers, gates and lifts with parameter tables; this source
  adds shape variety only, and only for users who hold it.
- Kitchen fixtures under a redistributable licence come from another source; nothing here duplicates that
  role.

## References

Appendix G.2 (row 8); §19.9; §37; FROB-REF-310.
