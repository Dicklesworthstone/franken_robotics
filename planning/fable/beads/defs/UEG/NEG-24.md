---
key: "NEG-24"
title: "scene asset source `partnet-mobility` (non-commercial research): user-supplied articulated objects only"
type: "task"
priority: 2
milestone: "M3"
labels: ["m3", "habitat", "safety", "frob-scene", "solid", "test", "asset", "registry", "review-notes-open"]
parent: "WP-23"
deps: ["BD-305", "N19a-07", "N19a-46", "N19b-02", "N37-18", "NEG-22"]
needs: []
related: ["NEG-25", "NEGp1-51"]
gates: []
plan_refs: ["Appendix G.2 (row 8)", "§19.9", "§37", "FROB-REF-310", "FROB-REF-731", "FROB-REF-733"]
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

**Import.** `frob scene import --kind asset <object> --source partnet-mobility --user-supplied --licence-ack
<statement> --profile articulated-object --behaviour articulated --scale <factor> --key <k> --json` (the verb
and its inputs are N19a-46's; this bead adds the import profile `articulated-object`, which reads the
collection's joint annotations), refused in default mode. Mapping:

| Source annotation | Fixture mapping | Parameters |
|---|---|---|
| Revolute joint with limits | Hinged leaf (door, lid, oven door) | Axis and range from the source; closing torque, friction and latch force `assumed` from the fixture library's table for the object category |
| Prismatic joint with limits | Drawer or sliding door | Axis and travel from the source; rail friction and end-stop stiffness `assumed` |
| Continuous revolute | Knob, wheel, tap handle | Axis from the source; detents and resistance `assumed` |
| Fixed | Folded into the parent by the pseudo-inertia sum | — |
| Screw or other | Not mapped: listed in the loss report; the part is frozen | — |

Masses: the source has none; each part gets mass from volume and a density `assumed` per category, inertia
from the solid part mesh. Collision: per-part convex decomposition with recorded error. Units and scale: the
collection's objects are normalized, so the import requires `--scale <factor>` or `--target-dimension
<axis>=<length>` from the user (N19a-46's inputs) and records the scale declaration as `user-declared` (N19a-46's
declaration record, not a scale label; from M5 a declaration maps to the label `estimated`, never `witnessed`,
N19c-02); without either the import refuses (`FROB-REF-802`, naming the missing input).

**Validation.** Each joint sweeps its range without the parts interpenetrating beyond the recorded collision
error: the profile moves each joint kinematically from its lower to its upper limit and back, in steps of at most
1° for a revolute joint and at most 5 mm for a prismatic one, against the per-part convex decompositions of the
object; a penetration deeper than the recorded decomposition error refuses the import with the part and the joint
value named, and each jointed part's sweep (joint, range, step, minimum clearance, result) is written beside the
loss report's per-part articulation row (N19a-46), so the import envelope carries it. Limits finite; the count of
`assumed` parameters is written to the pack, and every such parameter carries provenance `assumed`, so an analysis
run under strict data that needs one (a closing torque, a latch force) refuses by the strict-data rule rather than
use it.

**Containment.** As for every research-only source: private pack, entries with redistribution forbidden,
export of the pack or a scene variant refused by the pack export check with FROB-REF-310 (exit 3); a still,
report, replay or run bundle whose lineage reaches an imported object refused at the export door of N37-18
with FROB-REF-731 while the private mark applies and FROB-REF-733 for the licence set in a session holding
`private:export` and the flag (exit 5); one code per request (N19b-02's division); purge with tombstones.

**Stand-in.** A two-drawer cabinet with a hinged door authored in the repository, tagged `partnet-mobility`
in a scratch register.

## Acceptance criteria

- [ ] The register row exists; the admission reading records the primary host and the terms' text hash.
- [ ] The stand-in imports with one hinged leaf and two drawers as typed fixtures; every dynamic parameter is
      `assumed` and counted.
- [ ] Import without a scale is refused; a screw joint is frozen and listed in the loss report.
- [ ] Every joint's sweep is recorded in the import envelope with its minimum clearance, and the stand-in's
      sweeps pass.
- [ ] Pack and variant exports are refused with FROB-REF-310 (exit 3); a still and the report of a run in the
      pack are refused at the door with FROB-REF-731, or FROB-REF-733 with `private:export` and the flag (exit 5).

## Test plan

### Unit tests

- `asset_source_partnet_row_valid`.
- `partnet_joint_mapping_table`: one test per row of the mapping.
- `partnet_import_requires_scale`.
- `partnet_dynamics_parameters_assumed_and_counted`.
- `partnet_sweep_no_interpenetration`: the stand-in's three joints swept at 1° and 5 mm steps pass, each with its
  minimum clearance recorded; a planted drawer whose travel runs it through the cabinet's back is refused with the
  part and the joint value named.
- `partnet_pack_and_variant_export_refused_310`; `partnet_still_and_report_refused_at_door` (731, and 733 with
  the private flag); `partnet_no_real_data_in_tree`.

### End-to-end script and logging

`scripts/e2e/assets_licences.sh` (the M3 scene-asset loop, owned and run by NEGp1-51, which depends on this
bead; this bead lists the steps of its row), on the stand-in, every `run` verb call with its own `--key`:
refused default import (`--key al-pm-1`); the user-supplied import as written above without a scale (`--key
al-pm-2`; refused) and with `--scale 1.0` (`--key al-pm-3`; a private pack with one hinged leaf and two drawers);
each joint's sweep read from the import envelope (every row passed, no interpenetration beyond the recorded
collision error); `frob --json scene show <pack>` (each closing torque, friction and latch force with provenance
`assumed`, and the pack's `assumed` count equal to their number); after `e2e_scene_ready <pack>`, a referee run
in the pack (`frob --json sim run quad-a --scene <pack> --tier referee --seconds 2 --key al-pm-run`); the export
attempts, each `frob --json artifact export <ref> --out exports/ --key al-pm-x<n>` (the pack, and a variant made by
`frob --json scene compile --base <pack> --edits fixtures/hab/edits/threshold_ramp.json --key al-pm-v`:
`FROB-REF-310`, exit 3; a still and the run's report: `FROB-REF-731`, exit 5). Log lines carry `ts`, `script`, `bead: "NEG-24"`, `step`,
`event`, the exact command, exit code, wall time, the `evidence` block, fixture types, ranges, the `assumed`
count, and each refusal code with its exit code and raising site (pack check or door).

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

Appendix G.2 (row 8); §19.9; §37; FROB-REF-310, FROB-REF-731, FROB-REF-733.
