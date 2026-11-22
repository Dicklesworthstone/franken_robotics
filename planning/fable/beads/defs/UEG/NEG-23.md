---
key: "NEG-23"
title: "scene asset source `hm3d` (academic, non-commercial): user-supplied import of scanned buildings only"
type: "task"
priority: 2
milestone: "M3"
labels: ["m3", "habitat", "safety", "frob-scene", "solid", "test", "asset", "registry"]
parent: "WP-23"
deps: ["N19b-02", "BD-305", "N19a-05", "NEG-22"]
needs: ["§19.13: scene.import for user-supplied scanned scenes (unknown mask, scale label, loss report)"]
related: ["NEG-24", "NEG-25", "BD-398", "BD-228", "NEG-16"]
gates: []
plan_refs: ["Appendix G.2 (row 7)", "§19.9", "§19.13", "§37", "FROB-REF-310", "Appendix H.3"]
---
## Background

HM3D is a published collection of 3D scans of real buildings, distributed under academic, non-commercial
terms that each user accepts individually (FACT from the research pass of 2026-11-21; the dataset's page is
in the source register and is re-read at admission). Like every research-only collection it is supported
only as a user-supplied import and is never redistributed. It differs from the synthetic home collections in
two ways that shape this bead. Its terms are an agreement accepted by a named user, so the import records
that the user asserted acceptance and nothing can be shared between users of one installation without each
asserting it. And its content is scans of real places: meshes with holes, no articulation, no material
semantics, and a privacy dimension, since they depict real interiors. A scan tells a simulator where surfaces
were seen, not what is behind them or what they are made of.

## Scope

In: the register row; the user-supplied import profile for scanned buildings; unknown-region handling;
containment; a `synthetic` stand-in. Out: the licence machinery (N19b-02); private store and purge (BD-305);
the general research-only containment rules shared with the synthetic home collection (NEG-22); ingestion of
the user's own scans of their own places (BD-398).

## Technical approach

**Register row.** `key = "hm3d"`, `licence = "academic-non-commercial-agreement"`, `class = "research-only"`,
`default_pipeline = false`, `redistribute = "never"`, `per_user_acceptance = true`, `kind = "scan-collection"`,
`use = "user-supplied import only"`.

**Import.** `frob scene import --source hm3d --user-supplied --licence-ack <statement> <scan> --json`,
refused in default mode. Steps specific to scans:

1. Scale label: scans are metric by construction of the source; the label is `declared: metres` with the
   source's statement recorded, and a sanity check compares door heights found in the scan with the hazard
   registry's plausible range, flagging a mismatch.
2. Unknown mask: regions the scan did not observe (holes, the far side of furniture, glass and mirrors that
   scanners miss) are marked `unknown`, never filled. Clearance queries through unknown regions return
   `unknown`; a route that needs one is not admissible; the mask's area fraction is reported.
3. Floor extraction and levelling, with the residual tilt reported.
4. No materials: every surface gets interface class `assumed: generic-floor` or `assumed: generic-wall` and
   an optical class `unknown`; analyses that need friction or optics at a stated provenance refuse under
   strict data.
5. No articulation: doors are frozen as scanned; a door scanned half open is a static obstacle and is flagged
   as "possible door" for the user.
6. Licence entry per scan with class `research-only`, redistribution forbidden, the user's identity (as the
   capability holder) and the acknowledgement's hash; the pack is `private`.

**Containment.** As for every research-only source (export of the pack, of variants, of stills and of reports
containing them is refused with FROB-REF-310; purge leaves tombstones), plus: a pack imported under one
user's acknowledgement is not readable under another capability holder's namespace unless that holder has
recorded their own acknowledgement.

**Stand-in.** A small mesh with deliberate holes authored in the repository, tagged `hm3d` in a scratch
register.

## Acceptance criteria

- [ ] The register row exists with `per_user_acceptance = true`.
- [ ] Import of the stand-in yields a private pack with an unknown mask whose area fraction is reported, a
      scale label, assumed materials and a loss report.
- [ ] A clearance query across a hole returns `unknown`; a route through it is not admitted.
- [ ] Cross-user read without acknowledgement is refused; every export path is refused with FROB-REF-310.

## Test plan

### Unit tests

- `asset_source_hm3d_row_valid`.
- `hm3d_import_marks_unknown_regions`, `hm3d_clearance_through_unknown_is_unknown`.
- `hm3d_materials_assumed_optics_unknown`, `hm3d_strict_data_refuses_friction_dependent_analysis`.
- `hm3d_per_user_acknowledgement_required`.
- `hm3d_export_paths_refused_310`.
- `hm3d_no_real_data_in_tree`.

### End-to-end script and logging

`scripts/e2e/assets_licences.sh` (NEG-16), scene-asset loop at M3, on the stand-in: refused default import,
user-supplied import, a clearance query across the hole, the cross-user read, the export attempts; log lines
carry class, unknown fraction, scale label, refusal codes, and no geometry.

### Known answers and negative controls

Unknown regions are never treated as free space or as solid (the scene-ingestion rule). Controls: an importer
that fills holes by interpolation (the clearance test then returns `clear` and fails); a pack shared across
namespaces without acknowledgement. Determinism: import of the same file gives the same pack identity.

## Considerations

- Scans of real interiors can contain personal details; the private mark and the no-geometry rule for logs
  apply regardless of licence.
- A scanned building is a geometry sample, not a certified place: nothing evaluated in it supports a Reality
  claim.

## References

Appendix G.2 (row 7); §19.9; §19.13; §37; FROB-REF-310; Appendix H.3.
