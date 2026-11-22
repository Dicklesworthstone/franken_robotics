---
key: "NEG-18"
title: "scene asset source `infinigen-indoors` (BSD-3): offline external generator, never in the production graph"
type: "task"
priority: 3
milestone: "M5"
labels: ["m5", "habitat", "frob-scene", "frontier", "test", "asset", "registry"]
parent: "WP-32"
deps: ["N19b-02", "N19a-40", "BD-307", "N19a-05"]
needs: ["§19.13: scene.import for single assets and whole generated interiors at M3 (decode, compile, loss report, licence manifest entry)"]
related: ["BD-399", "N19a-41", "NEG-17", "NEG-19", "NEG-20", "NEG-16"]
gates: []
plan_refs: ["Appendix G.2 (row 2)", "§19.7", "§19.9", "KILL-DRESSING", "Appendix H.3"]
---
## Background

Structural proxies carry the project's scenes through M3. Scenes populated with realistic furniture and
objects are Frontier and land at M5: they add nothing to decision time and widen geometric and visual
variety. Infinigen Indoors is a published procedural generator of furnished interior scenes, on file under
BSD-3 (FACT from the research pass of 2026-11-21; the repository URL is in the source register with a mark
that means the text was fetched and not confirmed as primary, so the licence is re-read before use). Its role
here is narrow: an offline external generator. It is run out of process, on a developer's or a build
machine, to produce interiors that are then imported like any other asset; it is never a dependency, never
linked, never executed by `frob`, and nothing of it enters the production dependency graph. The whole tier is
gated: its paper-scale pilot lists, for one archetype, the failure classes that a populated scene could
provoke and a structural proxy cannot, and if the list is empty the tier is not funded; kill gate
KILL-DRESSING falls back to proxies and static clutter if populated scenes break the fixed launch shape or
the determinism class of a batch, add more than 25% to step cost, or expose no failure class beyond the
proxies on the M5 battery.

## Scope

In: the register row; the admission reading of the licence (code and, separately, any bundled or generated
asset content); the offline-run recipe and its pin; the import profile; validation of imported interiors;
`assets/scenes/infinigen-indoors/` with its manifest. Out: the licence manifest, register mechanics and
pack-time refusal (N19b-02); populated interiors inside the proxies' launch shape (BD-399); the pilot and the
kill decision (N19a-40, N19a-41); importer limits and fuzzing (BD-307).

## Technical approach

**Register row.** `key = "infinigen-indoors"`, `licence = "BSD-3-Clause"`, `class = "attribution"` (notice
required), `default_pipeline = true` only after the admission reading, `redistribute = "with-notice"`,
`kind = "external-offline-generator"`, `use = "offline external generator for populated interiors"`.

**Admission reading (the first task).** Read the licence text at a pinned release from the primary
repository and record its hash and date. Answer in writing, in the admission record: (1) does BSD-3 cover the
generator's code only, or also the procedural asset definitions and any bundled textures or meshes? (2) are
there third-party components inside the generator with other licences that would attach to outputs? (3) do
generated scenes carry any restriction? Until all three are answered from primary text the row has
`read = false` and the default pipeline excludes it.

**Offline-run recipe.** `tools/scene-gen/infinigen/README` and a pinned container digest or environment
record: generator release, seed, room-type configuration, export format. The recipe runs outside the
workspace's build; its outputs are files. Each output set is stored with a record: release, seed,
configuration hash, command, date, output hashes. Nothing under `crates/`, `surface/`, `spark/` or `apps/`
may reference the tool (a dependency lint enforces it).

**Import profile.** `frob scene import --source infinigen-indoors --profile populated-interior <dir> --json`:
decode meshes; assign each object a collision representation (fitted primitives or a convex decomposition
with recorded error) and a material from the project's interface and optical maps (the generator's shading
materials do not determine friction; friction class is `assumed` per object category); mark movable against
fixed objects from the generator's metadata; write the licence manifest entry per asset with this source key;
emit a loss report (dropped shaders, lights mapped to a lighting level, unsupported geometry).

**Validation of an imported interior.** Scale check (door heights and counter heights inside the hazard
registry's ranges, else flagged); every movable object has positive mass and a consistent inertia from a
declared density (`assumed`); no object interpenetrates the structure at rest (settling test on the referee);
the scene passes scene acceptance; the pack's launch shape (contact capacity, brick count) stays inside the
proxies' declared shape or the import is refused for the batched tiers with the reason.

## Acceptance criteria

- [ ] The admission record answers the three licence questions from primary text; the row's `read` flag and
      hash are set by it.
- [ ] The dependency lint shows no reference to the tool from any workspace.
- [ ] One generated home and one generated kitchen import with a loss report, pass validation and carry one
      licence entry per asset with the BSD-3 notice; exports embed the notice.
- [ ] With `read = false` the default pipeline excludes the source and says so.

## Test plan

### Unit tests

- `asset_source_infinigen_row_valid`; `…_excluded_until_read`.
- `infinigen_import_writes_licence_entries`: on a `synthetic` output set of three objects.
- `infinigen_import_friction_class_assumed`.
- `infinigen_import_scale_flags_out_of_range_door`.
- `infinigen_no_dependency_reference` (lint).
- `infinigen_output_record_complete`: release, seed, configuration hash, command, hashes.

### End-to-end script and logging

`scripts/e2e/assets_licences.sh` (NEG-16), scene-asset loop at M5: import of the recorded output set, licence
manifest read-back, export with notice, the exclusion check with `read = false`; log lines carry the source
key, release, seed, asset count, class, notice presence and refusal codes.

### Known answers and negative controls

The pilot's falsifier (an empty list of new failure classes means the tier is not funded). Controls: an
imported set whose launch shape exceeds the proxies' (refused for batched tiers); an output set with no
record (import refused); the tool referenced from a crate manifest (lint). Determinism: an import of the same
files yields the same `SceneId`; the generator's own determinism is not assumed, which is why outputs are
stored, not regenerated on demand.

## Considerations

- Nothing here is on the critical path; if the pilot or the kill gate goes against dressing, this row stays
  in the register as `not-used` with the decision recorded.
- Visual realism is not evidence of physical realism: friction, mass and optical classes of imported objects
  are assigned by the project and labelled `assumed`.

## References

Appendix G.2 (row 2); §19.7; §19.9; KILL-DRESSING; Appendix H.3.
