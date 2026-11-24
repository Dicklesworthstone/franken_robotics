---
key: "NEG-18"
title: "scene asset source `infinigen-indoors` (BSD-3): offline external generator, never in the production graph"
type: "task"
priority: 3
milestone: "M5"
labels: ["m5", "habitat", "frob-scene", "frontier", "test", "asset", "registry"]
parent: "WP-32"
deps: ["BD-307", "N19a-05", "N19a-40", "N19a-46", "N19b-02", "N37-18", "NAB-13"]
needs: []
related: ["NEG-19", "NEG-20", "NEG-27", "NEGp1-54"]
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

In: the values of the register row; the admission reading of the licence (code and, separately, any bundled or
generated asset content); the offline-run recipe and its pin; the import profile for a whole generated
interior; validation of imported interiors; `assets/scenes/infinigen-indoors/` with its per-source manifest
(the list of admitted output sets with their records and hashes; the generated files themselves are not in
the repository: like every asset they live under the configured asset root).

Out, with the bead that has it: the licence manifest type, the register schema and loader and the pack-time
refusal (N19b-02); the population of a grammar proxy's slots with single assets drawn from the per-asset
catalogue `registries/assets.toml`, the constraints that keep such a scene inside its proxy's launch shape
and determinism class, and the paired measurements KILL-DRESSING reads (BD-399; its catalogue rows for
objects of this source are created from this bead's manifest and take their class and notice from this row,
and its source table restates this row without redefining it); the pilot and the kill decision (N19a-40,
N19a-41); importer limits and fuzzing (BD-307). The two beads meet at one limit: a populated scene must stay
inside the proxies' launch shape. BD-399 keeps it for a slot asset by substituting the proxy box; this bead
keeps it for a whole interior, which has no proxy box behind it, by refusing the import for the batched
tiers.

## Technical approach

**Register row.** `key = "infinigen-indoors"`, `licence = "BSD-3-Clause"`, `class = "attribution"` (notice
required), `default_pipeline = true` only after the admission reading, `redistribute = "with-notice"`,
`kind = "external-offline-generator"`, `use = "offline external generator for populated interiors"`. The
class matters downstream: a caller of the populate step who permits only `redistributable` assets
(`scene gen --populated --assets redistributable`, BD-399) draws nothing from this source; `--assets
attribution` is needed, and the resulting pack has the licence set `attribution` and exports with the BSD-3
notice.

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

**Import profile.** `frob --json scene import --kind asset <dir> --source infinigen-indoors --profile
populated-interior --key <k>` (the verb and its inputs are N19a-46's: `kind` is required and a `run` verb takes an
idempotency key; this bead adds the import profile `populated-interior`, which admits a directory holding one
generated interior with its object files and its output record, raises the decode limits of `asset-default` to
the profile's own, and takes each object's behaviour from the generator's metadata (movable → `free`, jointed →
`articulated`, otherwise `static`; an object the metadata leaves open is `FROB-REF-315`), recorded in
`registries/import-profiles.toml`; an output set without its record is refused). The import: decode meshes; assign each object a collision representation (fitted primitives or a convex decomposition
with recorded error) and a material from the project's interface and optical maps (the generator's shading
materials do not determine friction; friction class is `assumed` per object category); mark movable against
fixed objects from the generator's metadata; write the licence manifest entry per asset with this source key;
emit a loss report (dropped shaders, lights mapped to a lighting level, unsupported geometry).

**Validation of an imported interior.** Scale check (door heights and counter heights inside the hazard
registry's ranges, else flagged); every movable object has positive mass and a consistent inertia from a
declared density (`assumed`); no object interpenetrates the structure at rest (settling test on the referee);
the scene passes scene acceptance; the pack's launch shape (contact capacity, brick count) stays inside the
proxies' declared shape or the import is refused for the batched tiers with the reason (the limit is the one
BD-399 enforces in its populate step; there an offending asset is replaced by its proxy box, which a whole
interior does not have).

**Manifest and catalogue.** `assets/scenes/infinigen-indoors/manifest.toml` (DESIGN name) has one entry per
admitted output set: the record above, the hashes of its files, the object list with per-object hashes, and
the notice text. It holds identifiers, records and digests, no generated bytes. An object of an admitted set
that the populate step may place gets a row in `registries/assets.toml` (BD-399) pointing at its entry here.

## Acceptance criteria

- [ ] The admission record answers the three licence questions from primary text; the row's `read` flag and
      hash are set by it.
- [ ] The dependency lint shows no reference to the tool from any workspace.
- [ ] One generated home and one generated kitchen import with a loss report, pass validation and carry one
      licence entry per asset with the BSD-3 notice; exports embed the notice.
- [ ] With `read = false` the default pipeline excludes the source and says so: the import is refused with
      `asset-source-not-default-pipeline` (exit 3, nothing written), and an output set without its record is
      refused with `FROB-REF-303`.

## Test plan

### Unit tests

- `asset_source_infinigen_row_valid`; `…_excluded_until_read`.
- `infinigen_import_writes_licence_entries`: on a `synthetic` output set of three objects.
- `infinigen_import_friction_class_assumed`.
- `infinigen_import_scale_flags_out_of_range_door`.
- `infinigen_no_dependency_reference` (lint).
- `infinigen_output_record_complete`: release, seed, configuration hash, command, hashes.

### End-to-end script and logging

`scripts/e2e/assets_licences.sh` (the M5 scene-asset loop, owned and run by NEGp1-54, which depends on this
bead; this bead lists the steps of its row), on the `synthetic` recorded output set, every `run` verb call with
its own `--key`: with `read = false`, the import as written above (`--key al-infi-1`; refused by the
default-pipeline filter of N19b-02 with the worded refusal `asset-source-not-default-pipeline`, exit 3, behind the
constant `ASSET_SOURCE_NOT_DEFAULT`, whose code is the interim `FROB-REF-802` until the ledger assigns its number;
`observed` names `infinigen-indoors`, its class and `read = false`; nothing is written); after the admission
reading, the same import (`--key al-infi-2`; a pack with a loss report, one licence entry per asset with source
`infinigen-indoors`, class `attribution` and the BSD-3 notice); the licence manifest read back with `frob --json
scene show <pack>`; an output set without its record (`--key al-infi-3`, refused with `FROB-REF-303`, exit 3,
the pack reader's one code for an input that cannot be decoded as written, `observed` naming the missing output
record); an imported set whose launch shape exceeds the
proxies' (`--key al-infi-4`, refused for the batched tiers with the reason); `frob --json artifact export <pack>
--out exports/ --key al-infi-5` (the export embeds the notice). Log lines carry `ts`, `script`, `bead:
"NEG-18"`, `step`, `event`, the exact command, exit code, wall time, the `evidence` block, the source key,
release, seed, asset count, class, notice presence, each refusal code with its exit code, and no geometry.

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
