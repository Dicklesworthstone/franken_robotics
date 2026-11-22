---
key: "NEG-22"
title: "scene asset source `hssd` (CC BY-NC 4.0): user-supplied import only, never redistributed"
type: "task"
priority: 2
milestone: "M3"
labels: ["m3", "habitat", "safety", "frob-scene", "solid", "test", "asset", "registry"]
parent: "WP-23"
deps: ["N19b-02", "BD-305", "N19a-05"]
needs: ["§19.13: scene.import for user-supplied scenes at M3 (decode, compile, loss report, scale label, licence manifest entry)"]
related: ["NEG-23", "NEG-24", "NEG-25", "BD-398", "BD-235", "NEG-16"]
gates: []
plan_refs: ["Appendix G.2 (row 6)", "§19.9", "§37", "FROB-REF-310", "Appendix H.3"]
---
## Background

HSSD is a published collection of furnished synthetic home scenes built for embodied-AI research. It is on
file under CC BY-NC 4.0 (FACT from the research pass of 2026-11-21; the dataset's page is in the source
register and is re-read at admission). The non-commercial term decides its role: the project never ships it,
never includes it in a pack that leaves the user's machine, and never draws from it in the default pipeline.
What the project does support is a user who has obtained the collection themselves under its terms and wants
to evaluate a robot in those homes: a user-supplied import. The scenes then live in the user's private store,
carry a licence entry of class `research-only` with redistribution forbidden, and every derived artifact
inherits that class, so a pack or report that would export them is refused at pack time (FROB-REF-310).

## Scope

In: the register row; the user-supplied import profile for whole furnished home scenes; the containment
checks; a `synthetic` stand-in used by tests (the project's tests never contain real HSSD data). Out: the
licence manifest, join and pack-time refusal (N19b-02); the private store, path jail and purge (BD-305);
train and test hygiene for scene groups (BD-235).

## Technical approach

**Register row.** `key = "hssd"`, `licence = "CC-BY-NC-4.0"`, `class = "research-only"`,
`default_pipeline = false`, `redistribute = "never"`, `kind = "scene-collection"`,
`use = "user-supplied import only"`, `read` set by the admission reading (the licence text's hash and date).

**Import.** `frob scene import --source hssd --user-supplied --licence-ack <path-to-user's-copy-of-terms>
<scene> --json`. Without `--user-supplied` the default-pipeline filter refuses. The import: decodes the scene
(structure and objects); compiles it to a private pack under the user's private root; writes one licence
entry per asset with `source = "hssd"`, class `research-only`, `redistribution = Forbidden`; marks the pack
`private`; assigns interface and optical classes (`assumed`); attaches a scale label (`declared` if the
source states units, else `unknown`, in which case dimension-dependent analyses refuse); emits a loss report.

**Containment checks.**

1. Export of the pack, of any scene variant derived from it, or of a report whose artifact graph contains it:
   refused with FROB-REF-310 naming the assets and the source. Repairs: remove the asset; keep the pack local.
2. Aggregated results (a scorecard, a verdict) may be exported only with the licence join shown and with no
   geometry, stills or pixel data from the scene; a still rendered from the scene is itself a derived asset of
   class `research-only`.
3. Logs and telemetry never contain the scene's geometry (the general rule for private scenes).
4. `frob scene purge` removes the pack and every derived blob, leaving tombstones.
5. The scenes of one home, and its edited or mirrored variants, form one group for train and test splits.

**Stand-in.** `tests/fixtures/synthetic/hssd-like/`: a small furnished room authored in this repository,
tagged with source key `hssd` in a scratch register so that tests exercise the research-only path without any
real data.

## Acceptance criteria

- [ ] The register row exists with class `research-only` and `redistribute = "never"`.
- [ ] A default-mode import is refused; a user-supplied import of the stand-in yields a private pack whose
      entries forbid redistribution.
- [ ] Export of the pack, of a derived variant, of a still and of a report containing a still are each
      refused with FROB-REF-310; an aggregate scorecard without geometry exports with the join shown.
- [ ] No real HSSD file exists in the repository or in any test fixture (scan by source key and by the
      absence of that host's file naming pattern).

## Test plan

### Unit tests

- `asset_source_hssd_row_valid`.
- `hssd_default_import_refused`, `hssd_user_supplied_import_is_private_research_only`.
- `hssd_export_pack_refused_310`, `hssd_export_variant_refused_310`, `hssd_export_still_refused_310`.
- `hssd_scorecard_exports_with_join_no_geometry`.
- `hssd_purge_leaves_tombstones_only`.
- `hssd_no_real_data_in_tree`.

### End-to-end script and logging

`scripts/e2e/assets_licences.sh` (NEG-16), scene-asset loop at M3, on the stand-in: refused default import,
user-supplied import, the four export attempts, purge; log lines carry the source key, class, refusal codes
and the absence of geometry in the log itself.

### Known answers and negative controls

The safety gate's licence rule (the licence set of a derived artifact equals the join of its inputs'; a
research-only asset cannot enter an exported report or pack). Controls: a build whose export check ignores
derived variants; a still exported through the report path. Determinism: refusals are deterministic.

## Considerations

- The user is responsible for holding the collection lawfully; the acknowledgement path records what they
  declared, and the no-authorization flags on every artifact say that the tool grants nothing.
- Non-commercial terms bind the user's use too; the project takes no position on what counts as commercial.

## References

Appendix G.2 (row 6); §19.9; §37; FROB-REF-310; Appendix H.3.
