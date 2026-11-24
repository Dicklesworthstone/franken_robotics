---
key: "NEGp1-52"
title: "scene asset source `hm3d`: the M5 scan import profile (scale label, unknown mask, levelling, materials, doors)"
type: "task"
priority: 3
milestone: "M5"
labels: ["m5", "habitat", "safety", "frob-scene", "solid", "test", "asset"]
parent: "WP-23"
deps: ["BD-305", "BD-398", "N19a-46", "N19c-02", "N19c-03", "N19c-04", "NEG-23", "STD-E2E"]
needs: []
related: ["NEG-22", "NEGp1-54"]
gates: []
plan_refs: ["Appendix G.2 (row 7)", "§19.9", "§19.13", "§37", "FROB-REF-304", "FROB-REF-305", "Appendix H.3"]
---
## Background

HM3D is a published collection of 3D scans of real buildings under academic, non-commercial terms that each
user accepts individually (FACT from the research pass of 2026-11-21; re-read at admission). NEG-23 (M3) carries
its register row, the per-user acknowledgement, the user-supplied import as a licence-tracked private pack and
its containment, and keeps a scan pack out of every geometric consumer until the scan import profile exists.
This bead is that profile. It needs the scene unit's M5 machinery (scale labels, the three-valued occupancy with
its unknown mask, the real-place ingest adapter), which is why it was split from NEG-23 in polishing round 1:
an M3 bead cannot wait on M5 work.

A scan tells a simulator where surfaces were seen, not what is behind them or what they are made of. Scans have
holes, no articulation, no material semantics, and depict real interiors. The profile turns a scan into a pack
that consumers can use honestly: what was not seen is `unknown`, never free and never solid; what was not
measured is `assumed` with its class; doors are what the scanner saw.

## Scope

In: the HM3D scan profile run by `scene.import` for packs tagged `hm3d`; lifting NEG-23's fail-closed refusal
for packs that went through it (a pack imported at M3 is re-imported, never silently upgraded); the tests below;
the HM3D steps of the M5 scene-asset loop. Out: the register row, acknowledgement and containment (NEG-23); scale
labels and FROB-REF-304 (N19c-02); the unknown mask, its per-consumer treatment and FROB-REF-305 (N19c-03); the
real-place ingest adapter and its loss report (N19c-04); the private root and purge (BD-305); the `place` form
of `scene.import` on both surfaces and the user's own scans of their own places (BD-398, a dependency: this
profile is reached through that form and shares its machinery, not its licence).

## Technical approach

`frob scene import --kind place <scan> --from mesh --declare <declaration> --source hm3d --user-supplied
--licence-ack <statement> --profile hm3d-scan --key <k> --json`, refused in default mode (NEG-23). The `kind` is
`place`: a scan of a real building goes through the real-place lane, which first answers at M5 (BD-398; before it
`--kind place` is `FROB-REF-724`, which is why NEG-23's M3 import uses `--kind asset --profile scan-store` and
stores the scan without geometry). The adapter is N19c-04's mesh-with-operator-scale adapter (`--from mesh`; the
default `twin` reads a twin package, which an HM3D scan is not), and `mesh` needs `--declare`: the declaration
file states units (metres, the source's statement), frame and handedness and carries no scale witness, so the
pack's scale is a declaration (step 1) and its Reality status `no-claim`, as that adapter labels it. This bead adds
the import profile `hm3d-scan`. After the licence entry, the profile runs
these steps through the M5 machinery and records each in the loss report:

1. **Scale label.** Scans are metric by the source's construction, but the source's statement is a declaration
   without a measured baseline, so under N19c-02 the label is `estimated`, with the statement recorded and its
   declared accuracy as the interval (±5% when none is declared), never `witnessed` (`declared` is not one of
   the four labels, and the manifest reader refuses it). Consequences, all N19c-02's: clearance verdicts use
   u_scale = ε_c·(ρ + max(c, 0)) with ε_c = 0.05, so a doorway fit whose margin lies inside that interval is
   `unresolved` and a verdict that survives it is exploratory only; a task marked `exploratory` binds to the
   pack, and a metric task (a dimensional predicate in its success or reward) is refused with FROB-REF-304; a
   realism or Reality statement on the pack is `no-claim`. A sanity check compares door openings found in the
   scan with the plausible range in the hazard registry and flags a mismatch; a scale the check contradicts is
   reported, never rescaled silently, and the label stays `estimated`.
2. **Unknown mask.** Regions the scan did not observe (holes, the far side of furniture, glass and mirrors that
   scanners miss) are marked `unknown` in N19c-03's three-valued occupancy, never filled. A clearance query
   through an unknown region returns `unknown`; a route that needs one is not admissible; the mask's area
   fraction is reported per floor.
3. **Floor extraction and levelling**, with the residual tilt reported in degrees.
4. **No materials.** Every surface gets interface class `assumed: generic-floor` or `assumed: generic-wall` and
   optical class `unknown`; an analysis that needs friction or optics at a stated provenance refuses under
   strict data, and the `assumed` count enters the pack's provenance block.
5. **No articulation.** Doors are frozen as scanned; a door scanned half open is a static obstacle flagged
   "possible door" for the user, never articulated by guess.
6. The pack keeps NEG-23's licence entry and private mark; nothing here changes containment.

**Stand-in.** NEG-23's small mesh with deliberate holes, extended with a door-sized opening, a half-open door
slab, a 0.5° floor tilt and a mirror-like region with no returns. Tagged `hm3d` in a scratch register; no real
scan is in the tree.

## Acceptance criteria

- [ ] Re-importing the stand-in yields a private pack with an unknown mask whose area fraction is reported, the
      scale label `estimated` (±5%, the source's statement recorded) with the door check's verdict, the
      residual tilt, assumed materials and a loss report naming each step; an exploratory task binds to the pack
      and a metric task is refused with FROB-REF-304.
- [ ] A clearance query across a hole returns `unknown`; a route through it is not admitted; a query in seen
      free space returns `clear`.
- [ ] Under strict data a friction-dependent analysis on the pack refuses; the half-open door is flagged and
      static.
- [ ] A pack imported at M3 without the profile is still refused by geometric consumers (the refusal
      `scan-profile-absent` of NEG-23, the code behind `SCAN_PROFILE_ABSENT`) until re-imported; the re-import
      is a new pack with a new `SceneId`, and the M3 pack is never upgraded in place.

## Test plan

### Unit tests

- `hm3d_import_marks_unknown_regions`: the planted holes and the no-return region are `unknown`; area fraction
  equal to the stand-in's planted fraction to 1%.
- `hm3d_clearance_through_unknown_is_unknown`; `hm3d_route_through_unknown_not_admitted`.
- `hm3d_scale_label_estimated`: the label is `estimated` with interval ±5% and the source's statement, never
  `witnessed`; a manifest carrying `declared` is refused by the reader. `hm3d_scale_check_flags_implausible_door`:
  the stand-in scaled by 1.5 makes the door check flag and the label stay `estimated` with the flag.
- `hm3d_estimated_scale_consequences`: on the stand-in, the free-space clearance query whose margin exceeds
  u_scale is decided (`clear`, exploratory); a doorway fit with ρ = 0.40 m and a 30 mm clearance (u_scale =
  0.05 × 0.43 = 21.5 mm, N19c-02's worked number) is decided, and one with a 10 mm clearance (u_scale = 20.5 mm)
  is `unresolved`; a metric task bound to the pack is refused with FROB-REF-304.
- `hm3d_floor_levelling_residual`: 0.5° planted, residual below 0.05° after levelling.
- `hm3d_materials_assumed_optics_unknown`, `hm3d_strict_data_refuses_friction_dependent_analysis`.
- `hm3d_half_open_door_static_flagged`.
- `hm3d_m3_pack_not_upgraded`: a pack written at M3 stays refused by geometric consumers until re-imported.

### End-to-end script and logging

The HM3D steps of the M5 scene-asset loop of `scripts/e2e/assets_licences.sh` (NEGp1-54), on the stand-in:
user-supplied import, a clearance query across the hole and one in free space, a route request through the
hole, the two doorway fits of `hm3d_estimated_scale_consequences` (decided, `unresolved`), a metric task bound to
the pack (FROB-REF-304), a strict-data friction analysis, and the M3-pack check. Log lines (STD-E2E NDJSON) carry
`bead: "NEGp1-52"`, the command, exit code, wall time, the `evidence` block, the class, the unknown fraction,
the scale label and the door check's verdict, the residual tilt, the `assumed` count, each refusal code, and no
geometry; on failure the first differing value and the pack path.

### Known answers and negative controls

Unknown regions are never treated as free space or as solid (the scene-ingestion rule; INV-HAB-04 through
N19c-03). Controls: an importer that fills holes by interpolation (the clearance test returns `clear` and
fails); a scale silently rescaled to pass the door check; a half-open door articulated by guess. Determinism:
import of the same file gives the same pack identity.

## Considerations

- Scans of real interiors can contain personal details; the private mark and the no-geometry rule for logs
  apply regardless of licence.
- A scanned building is a geometry sample, not a certified place: nothing evaluated in it supports a Reality
  claim.
- Risk: N19c-04's adapter targets the user's own scans; HM3D's file layout may need its own reader. If so, it is
  written here behind the same adapter interface and its loss report says which reader ran.

## References

Appendix G.2 (row 7); §19.9; §19.13; §37; FROB-REF-304, FROB-REF-305; Appendix H.3; NEG-23; N19c-02, N19c-03,
N19c-04.
