---
key: "NEG-23"
title: "scene asset source `hm3d` (academic, non-commercial): register row, per-user acknowledgement, containment"
type: "task"
priority: 2
milestone: "M3"
labels: ["m3", "habitat", "safety", "frob-scene", "solid", "test", "asset", "registry"]
parent: "WP-23"
deps: ["BD-305", "N19a-05", "N19a-46", "N19b-02", "N37-18", "NAB-13", "NEG-22"]
needs: []
related: ["NEG-24", "NEG-25", "NEGp1-51", "NEGp1-52"]
gates: []
plan_refs: ["Appendix G.2 (row 7)", "§19.9", "§19.13", "§37", "FROB-REF-310", "FROB-REF-731", "FROB-REF-733", "Appendix H.3"]
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

In (M3): the register row; the per-user acknowledgement rule and its namespace check; the user-supplied
import of an HM3D scan as a licence-tracked private pack through `scene.import` (N19a-46); containment (the pack
export check and the export door); the import profile `scan-store`; the fail-closed rule that keeps a scan pack
out of every consumer that would need the scan profile until it lands, as one check in the pack reader with
the refusal `scan-profile-absent`; a `synthetic` stand-in. Out: the scan import profile (scale label and its
sanity check, the unknown mask and clearance through it, floor levelling, assumed materials, frozen doors) and
its tests, which need the M5 machinery of the scene unit (scale labels N19c-02, the unknown mask N19c-03,
real-place ingest N19c-04) and are carried by NEGp1-52 at M5, split from this bead in polishing round 1 so
that this M3 bead depends on nothing later; the licence machinery (N19b-02); the export door (N37-18); private
store and purge (BD-305); the research-only rules shared with the synthetic home collection (NEG-22); ingestion
of the user's own scans of their own places (BD-398).

## Technical approach

**Register row.** `key = "hm3d"`, `licence = "academic-non-commercial-agreement"`, `class = "research-only"`,
`default_pipeline = false`, `redistribute = "never"`, `per_user_acceptance = true`, `kind = "scan-collection"`,
`use = "user-supplied import only"`.

**Import at M3.** `frob scene import --kind asset <scan> --source hm3d --user-supplied --licence-ack <statement>
--profile scan-store --key <k> --json` (the verb and its inputs are N19a-46's; `kind` is `asset` at M3 because
the `place` form answers `FROB-REF-724` until M5), refused in default mode. This bead adds the import profile
`scan-store` (DESIGN): it runs the source, licence and private-root steps of the asset pipeline and the decode
under limits, and none of the geometry steps (no watertightness check, no bricks, no cards, no acceptance), so
a scan with holes is neither refused as an open mesh nor repaired. At M3 the import therefore writes the scan
under the private root as a pack that holds the scan as one opaque blob with its licence entry (step 6 below)
and a loss report that lists every scan-profile step as not performed; the pack has no field and is marked
`scan-profile: absent` in its provenance block.

**The fail-closed refusal.** One check carries it: the pack reader's geometry accessors (the field, the
overlays, the maps and the route graph of `ScenePack`) return a refusal for a pack marked `scan-profile:
absent`, so every consumer that would read its geometry (`scene compile` from it, `scene.fit` and every
clearance query, routes, runs, analyses, stills) refuses without knowing anything about scans, and none can
treat a hole as free space. The refusal: "a scan pack was imported without its scan profile and has no
geometry a consumer may read"; exit 3; the pointer names the pack and `provenance.scan_profile`; ranked
repairs: 1. re-import the scan with the scan profile (from M5: `--kind place --profile hm3d-scan`, NEGp1-52);
2. use another scene. The plan gives it no number: it is the row `scan-profile-absent` (family `3xx`, a scene
refusal, no proposal) of the allocation ledger `registries/refusal_allocations.toml` (N35x-02, under N35-33's
rule; a bead never picks one). That run assigns the number at M0, before this bead lands, so the reader raises the
assigned number from the start, behind the one named constant `SCAN_PROFILE_ABSENT`; the ledger's interim
(`FROB-REF-305` with `detail` = `scan-profile-absent`, the nearest existing meaning: what the consumer needs lies
in a region nothing is known about) applies only to a build that predates the run. The tests and the script
assert on that constant's code, never on a literal. Reading the manifest, the licence entry and the loss report
(`scene.show`, `scene.list`) is not refused.

**Acceptance of a scan pack (DESIGN).** The scan-store pack has no acceptance report (the profile runs no
acceptance), and a run that binds an unaccepted pack would otherwise meet the acceptance check of BD-228 first,
with `scene-pack-not-accepted` (proposal 318) and the repair "run `scene.accept`", which would meet this same
refusal. So the reader's acceptance-report accessor is one of the refused accessors: `scene accept` on the pack,
and the acceptance check that run verbs call, both answer `scan-profile-absent`, whose repair is the one that
works. (BD-228's check reads the report through that accessor, `ScenePack::acceptance_report`, the one
`scene.accept` uses, so what the reader refuses comes before its `scene-pack-not-accepted`.)

**The verbs that reach a refused accessor, and so list the refusal.** `scene.compile` (the derived form on a
scan base), `scene.fit`, `scene.show` with `--probe`, `--route` or `--corridor`, `scene.accept` (the
acceptance-report accessor), `scene.bank --action estimate` and `--action admit` (both read the pack's bricks,
BD-236), `scene.retrofit
--apply --scene <pack>` (the variant is compiled from the pack's geometry), and every run verb that binds the
pack (`sim.run`, `train.run`), through BD-228's acceptance check at admission. `scene.show` without those
options and `scene.list` read only the manifest, licence entry and loss report, and never raise it.

**Scan profile (M5, NEGp1-52; listed here so that the row is complete).** Steps specific to scans:

1. Scale label: scans are metric by construction of the source, but a source's statement is a declaration,
   not a measured baseline, so the label is `estimated` with the source's statement recorded and its declared
   accuracy (±5% when none is declared), never `witnessed` (N19c-02); a sanity check compares door heights found
   in the scan with the hazard registry's plausible range, flagging a mismatch.
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

**Containment.** As for every research-only source, with one code per request (the division DESIGNed in
N19b-02): exporting the pack or a scene variant of it is refused by the pack export check with FROB-REF-310
(exit 3); a still, report, replay or run bundle whose lineage reaches it is refused at the export door of N37-18
with FROB-REF-731 (the pack lies under the private root, and the door checks the private mark before the
licence set), exit 5; purge leaves tombstones. Plus: a pack imported under one
user's acknowledgement is not readable under another capability holder's namespace unless that holder has
recorded their own acknowledgement.

**Stand-in.** A small mesh with deliberate holes authored in the repository, tagged `hm3d` in a scratch
register.

## Acceptance criteria

- [ ] The register row exists with `per_user_acceptance = true`.
- [ ] Import of the stand-in yields a private pack with its licence entry (class `research-only`, the user's
      identity, the acknowledgement's hash) and a loss report listing the scan-profile steps as not performed.
- [ ] Until the scan profile lands, compiling the scan pack, `scene accept` on it, a clearance query
      (`scene.fit`), a route and a referee run over it are each refused with the refusal `scan-profile-absent`
      (the code behind `SCAN_PROFILE_ABSENT`, exit 3, first repair "re-import with the scan profile"), the run
      never with `scene-pack-not-accepted`; none returns `clear`; `scene.show` on the pack still prints its
      manifest, licence entry and loss report.
- [ ] Cross-user read without acknowledgement is refused; the pack and variant exports are refused with
      FROB-REF-310 (exit 3) and a still and a report with FROB-REF-731 (exit 5), one code each (at M3 only the
      pack can be made through the binary, since the reader refuses the compile, render and run that would make
      the others; their rows are unit tests on artifacts whose lineage names the pack, and run end to end at M5 in
      NEGp1-54's step 4).

## Test plan

### Unit tests

- `asset_source_hm3d_row_valid`.
- `hm3d_import_private_pack_with_entry`: class, identity, acknowledgement hash, loss report.
- `hm3d_scan_pack_refused_without_profile`: compile, `scene.accept`, `scene.fit`, a route and a run are each
  refused with the code behind `SCAN_PROFILE_ABSENT`, pointing at `provenance.scan_profile`; the run's refusal is
  that code and not `scene-pack-not-accepted`; `scene.show` succeeds.
- `scan_store_runs_no_geometry_stage`: the stand-in with deliberate holes imports under `scan-store` without
  `FROB-REF-301` and without a repair receipt; the pack has no brick, no overlay and no card; the same file
  under `asset-default` is refused as an open mesh.
- `hm3d_per_user_acknowledgement_required`.
- `hm3d_pack_export_refused_310`, `hm3d_variant_export_refused_310`: exit 3.
- `hm3d_still_and_report_refused_731_at_door`: exit 5; one code per request (never 310 and 731 together).
- (The scan-profile tests `hm3d_import_marks_unknown_regions`, `hm3d_clearance_through_unknown_is_unknown`,
  `hm3d_materials_assumed_optics_unknown` and `hm3d_strict_data_refuses_friction_dependent_analysis` are
  NEGp1-52's.)
- `hm3d_no_real_data_in_tree`.

### End-to-end script and logging

`scripts/e2e/assets_licences.sh` (the M3 scene-asset loop, owned and run by NEGp1-51, which depends on this
bead; this bead lists the steps of its row), on the stand-in, every `run` verb call with its own `--key`: the
import without `--user-supplied` (`--key al-hm3d-1`; refused by the filter); the user-supplied import as written
above (`--key al-hm3d-2`; a private pack marked `scan-profile: absent`); `frob --json scene accept <pack> --key
al-hm3d-acc`, `frob --json scene fit --scene <pack> --footprint rect:0.50x0.70 --from <pose> --to <region> --key
al-hm3d-fit` and a referee run in the pack (`frob --json sim run quad-a --scene <pack> --tier referee --seconds 2
--key al-hm3d-run`, with no `e2e_scene_ready`, since the pack can never be ready) (each
`e2e_expect_refusal <code of SCAN_PROFILE_ABSENT> 3`); `frob --json scene show <pack>` (the manifest, the
licence entry with the acknowledgement's hash, the loss report); the read under a second capability holder
with no acknowledgement (refused); the export of the pack (`frob --json artifact export <pack> --out exports/
--key al-hm3d-x1`; `FROB-REF-310`, exit 3). The variant, still and report export rows cannot be reached through
the binary at M3 (the compile, render and run that would make them are refused above); the log records them as
`not run` with that reason and NEGp1-54 runs them on the profiled scan at M5. Log lines carry `ts`, `script`, `bead: "NEG-23"`, `step`, `event`, the
exact command, exit code, wall time, the `evidence` block, class, the pack's privacy mark, each refusal code
with its exit code and raising site (reader, pack check or door), and no geometry. The M5 steps are
NEGp1-54's.

### Known answers and negative controls

Unknown regions are never treated as free space or as solid (the scene-ingestion rule); at M3 this holds by
refusal. Controls: a build that compiles a scan pack without its profile (a clearance query then returns
`clear` and the test fails); a pack shared across namespaces without acknowledgement; a still exported through
the pack path (310 where the door's 731 is required). Determinism: import of the same file gives the same pack identity.

## Considerations

- Scans of real interiors can contain personal details; the private mark and the no-geometry rule for logs
  apply regardless of licence.
- A scanned building is a geometry sample, not a certified place: nothing evaluated in it supports a Reality
  claim.

## References

Appendix G.2 (row 7); §19.9; §19.13; §37; FROB-REF-310, FROB-REF-731, FROB-REF-733; Appendix H.3; NEGp1-52.
