---
key: "NEGp1-54"
title: "e2e: M5 run of `assets_licences.sh`: three permissive sources, the HM3D scan profile, populated-interior licences"
type: "task"
priority: 3
milestone: "M5"
labels: ["m5", "habitat", "safety", "frob-scene", "frob-io", "frontier", "test", "e2e"]
parent: "WP-23"
deps: ["BD-399", "N19a-40", "NEG-18", "NEG-19", "NEG-20", "NEGp1-51", "NEGp1-52", "STD-E2E"]
needs: []
related: ["NEG-16", "N19a-41"]
gates: []
plan_refs: ["Appendix G.2", "§19.7", "§19.9", "§19.13", "§37"]
---
## Background

Three scene-asset sources enter the default pipeline only at M5, after their admission reading: Infinigen
indoors (BSD-3, an offline external generator whose recorded output sets are imported, NEG-18), Google Scanned
Objects (CC BY 4.0, household clutter with attribution in every export, NEG-19) and RoboCasa (assets CC BY 4.0,
code MIT, kitchen fixtures and layouts, NEG-20). At M5 the HM3D scan profile also lands (NEGp1-52), and the
interiors that place third-party assets do too (BD-399, with the dressing pilot N19a-40). Before polishing
round 1 the three M5 rows pointed at NEG-16's script, an M1 bead, for their end-to-end steps, and nothing
depended on them. This bead is the M5 run of that script: it gives each M5 row its end-to-end steps and checks
the property the asset-and-licence capability exists for at M5: a licence entry travels from the source row,
through import and placement in a populated interior, into the pack's manifest and into every exported report's
notices, and a research-only scan never leaves the private root.

## Scope

In: the M5 steps of `scripts/e2e/assets_licences.sh` for NEG-18, NEG-19, NEG-20 and NEGp1-52, each as its bead
lists them; the populated-interior licence trace; the full M5 run (robots, every register row) and its log;
helper tests. Out: the rows, the scan profile and their mechanisms (those beads); the catalogue and placement
of assets in interiors (BD-399) and the dressing pilot with its kill criterion (N19a-40, N19a-41); the M1 and M3
runs (NEG-16, NEGp1-51).

## Technical approach

**Run.** `scripts/e2e/assets_licences.sh --milestone M5`, offline against vendored records and recorded hashes,
with a per-run private root purged at the end (`scene purge` in its dry-run and `--dry-run=false --confirm
<digest>` form, no key). Every `run` verb call of the M5 steps (`scene.import`, `scene.compile`, `scene.gen`,
`scene.accept`, `sim.run`, `artifact.export`) carries its own `--key`, distinct per call: the row steps of
NEG-18, NEG-19 and NEG-20 use the keys those beads list (`al-infi-<n>`, `al-gso-…`, `al-rbc-…`), and the HM3D
and trace steps use `al-m5-<row>-<step>`; a step that compares two results of one request uses two keys, because
a repeated key returns the stored receipt. Before any run binds a pack the script calls `e2e_scene_ready <pack>`. The scan
steps import with NEGp1-52's command (`--kind place --from mesh --declare <declaration> --profile hm3d-scan`); the asset steps with `--kind asset` and the
row's profile: `populated-interior` (NEG-18), `small-rigid` with `--behaviour free` (NEG-19), `kitchen-fixture`
with `--behaviour articulated` and `kitchen-layout` (NEG-20). All M1 and M3 steps run again (regression); the M5
steps are, each exactly as its bead lists it:

1. `infinigen-indoors`: with `read = false`, the import refused by the default-pipeline filter with its reason;
   after the admission reading, import of the recorded output set (generator release, seed, room-type
   configuration recorded); licence manifest read back with `scene show`, class `attribution` and the BSD-3
   notice; an output set without its record (refused); an imported set whose launch shape exceeds the proxies'
   (refused for the batched tiers, with the reason); export of the pack with the notice.
2. `google-scanned-objects`: import of the selection; a non-selected object and a branded one (each refused by
   the default pipeline); a scene with five objects dropped on a floor and settled on the referee; export of the pack and of a report, both embedding the attribution and change statement;
   with the `notice` field removed, the pack check fails and the door refuses the report (N37-18).
3. `robocasa`: import of the five fixtures and one layout; the door that passes through its frame (refused by
   the sweep with `import-articulation-interpenetrates`: the code of `IMPORT_ARTICULATION_INTERPENETRATES`, exit
   3, the part and the joint value named); each fixture's sweep through its range read from its import envelope (NEG-20's validation rows)
   and a referee run in each fixture pack in which no joint leaves its range; the layout's aisle widths read back
   with `scene show`; export with attribution; the code licence (MIT) recorded apart from the asset licence.
4. `hm3d` scan profile (NEGp1-52) on its stand-in: unknown fraction, scale label `estimated` and door check,
   residual tilt, assumed materials, a clearance query through a hole returning `unknown`, a route refused, the
   doorway fits and the metric-task refusal of NEGp1-52; the division rows of NEGp1-51 for the scan pack, now
   all reachable: the pack and a variant (310, exit 3), a still, a report and a scorecard over a run in it (731,
   and 733 with `private:export` and the flag, exit 5), including the cells NEGp1-51 could not reach at M3.
5. Populated-interior licence trace: an interior populated by BD-399 at defaults with assets from steps 1 to 3
   placed; for every placed asset, its catalogue row names a third-party source key, its licence entry is in
   the compiled pack's manifest, the pack's licence set is the join of its entries (`attribution` here), and an
   exported report of a run in that interior carries every required notice once. Control: a placed asset whose
   catalogue row lost its source key makes the pack check fail, naming the asset.

## Acceptance criteria

- [ ] `--milestone M5` passes for all nineteen robots and all thirteen register rows; no row is `not run`.
- [ ] Each M5 row's steps pass as listed; the notice-removal control is caught for the pack and the report.
- [ ] The populated-interior trace holds for every placed asset, and the lost-source-key control is caught.
- [ ] The HM3D scan steps pass and no scan geometry leaves the private root; the purge leaves only tombstones.

## Test plan

### Unit tests

Helper tests in `scripts/e2e/tests/`:
- `m5_rows_cover_register`: every register row has steps and none is `not run` at M5.
- `notice_once_per_source`: a report over an interior with assets of three attribution sources carries three
  notices, each once.
- `trace_join_matches_entries`: the pack's licence set equals the join of its entries for planted sets
  ({redistributable, attribution} → attribution; {attribution, research-only} → research-only).
- `lost_source_key_fails`: the planted catalogue row without a source key fails the run.

### End-to-end script and logging

This bead is the M5 run of the script. Log: `target/e2e-logs/assets_licences/<utc>.ndjson` with a transcript,
through STD-E2E's helper. Every line: `ts`, `script`, `bead` (the row's key, or `"NEGp1-54"` for the trace),
`step`, `event`, the exact command, exit code, wall time, the `evidence` block, source key, release and seed
where recorded, asset identifiers and hashes, licence class, notice presence and the notice text's hash, the
pack's licence set, per-fixture joint ranges and the open-close result, settle error for dropped objects,
unknown fraction and scale label for the scan, each refusal code with its exit code and raising site, and on
failure the first differing field and the paths needed to reproduce. The `summary` line: robots and rows
passed, failed and skipped; placed assets traced; controls caught; `gates: []`. No geometry in any line.

### Known answers and negative controls

The licence join order (redistributable < attribution < research-only < proprietary < unknown) is the known
answer for the trace. Controls: notice removed; source key lost; a hole-filling importer for the scan.
Determinism: two runs give identical logs apart from timestamps and wall times.

## Considerations

- If the dressing pilot's kill criterion fires (N19a-41), interiors are populated without third-party assets
  and step 5 runs on the project's own generated objects; the trace then shows a one-entry licence set and the
  log says which branch ran.
- Admission readings can move a row's `default_pipeline` flag; the run reads the flag from the register, never
  from this bead.

## References

Appendix G.2; §19.7; §19.9; §19.13; §37; NEG-18, NEG-19, NEG-20, NEGp1-52; BD-399; N19a-40.
