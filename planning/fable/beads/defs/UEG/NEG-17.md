---
key: "NEG-17"
title: "scene asset source `frob-scene`: the project's own generators as default source; a pure starter pack"
type: "task"
priority: 2
milestone: "M3"
labels: ["m3", "habitat", "frob-scene", "solid", "test", "asset", "registry"]
parent: "WP-23"
deps: ["N19b-02", "BD-225", "BD-226", "BD-162", "N19a-05"]
needs: []
related: ["NEG-27", "NEG-16", "BD-227", "BD-228", "NEG-28", "N19a-07"]
gates: []
plan_refs: ["Appendix G.2 (row 1)", "§19.7", "§19.9", "§37", "FROB-REF-310"]
---
## Background

Scenes need content, and content has licences. The default source of every scene the project generates is its
own code: procedural generators in `frob-scene` that build seven interior archetypes (home, apartment,
office, warehouse, manufacturing plant, retail store, commercial kitchen) from one typed attribute grammar,
structural proxies first (boxes and fixtures with exact functional clearances, doors, ramps, thresholds,
stairs, film patches, glass and mirror regions, a lighting level). This source is under the project's own
licence, is fully redistributable, and is the only one the starter pack may contain besides synthetic test
fixtures. Third-party assets widen visual and geometric variety later (Frontier, M5) and are never needed for
the critical path. Making the project's own generators an explicit row of the asset-source register, with
the same manifest entry as any other source, means every pack states where all of its content came from,
including the common case "from us", and a pack that claims to be purely procedural can be checked.

## Scope

In: the register row for `frob-scene`; the rule that generated content carries exactly one licence entry
with this source; the purity check for packs and for the starter pack; the lint that keeps third-party files
out of the generator crates. Out: the grammar mechanism and the proxies (BD-225, BD-226); the licence
manifest, the register and the pack-time refusal (N19b-02); the fixture library (N19a-07).

## Technical approach

**Register row (in `assets/licences/sources.toml`).** `key = "frob-scene"`, `licence = "<the project's
licence identifier>"`, `class = "redistributable"`, `default_pipeline = true`, `redistribute = "always"`,
`read = true`, `kind = "procedural-generator"`, `use = "default; seven interior archetypes from one grammar
system, structural proxies first"`.

**One entry per generated pack.** A scene produced only by the generators has a licence manifest with exactly
one entry: source `frob-scene`, class `redistributable`, no notice required. Scene variants derived from it
(a clutter variant, a retrofit variant, a mirrored layout) copy that entry. As soon as an imported asset is
placed in a scene the manifest gains that asset's entry and the pack's class becomes the join.

**Purity check (`pack_is_pure_procedural`).** For a pack: every brick, overlay, fixture, interface card and
optical class in it is reachable from generator calls recorded in the pack's provenance block (generator
identity, grammar version, seed, draws), and no blob in the pack has an asset identifier from another source.
The starter pack must pass it for every scene it ships, apart from files explicitly tagged `synthetic` (test
fixtures authored in the repository).

**No third-party files in the generator crates (`generators_have_no_foreign_assets`).** A lint over the
`frob-scene` crate and `assets/`: every non-source file (mesh, texture, table) has a provenance header or a
sidecar naming its source row; files with source `frob-scene` must have been authored in this repository
(their history starts here) or be outputs of a generator with the generating command recorded. Numeric tables
used by generators (dimension ranges, material parameters) come from the hazard registries and cards, each
with its own provenance (`assumed`, `norm`, `measured`), not from the generator's code.

**Determinism.** The same generator identity, grammar version and seed yield a bit-identical pack and
`SceneId` on Linux and macOS; this is what lets the starter pack be regenerated rather than trusted.

## Acceptance criteria

- [ ] The register has the `frob-scene` row with the fields above and the registry loader accepts it.
- [ ] A generated warehouse and a generated commercial kitchen each have a one-entry licence manifest, pass
      the purity check and export without a refusal.
- [ ] The starter pack passes the purity check; adding a `synthetic` stand-in tagged research-only to it makes
      the check fail and the export refuse with FROB-REF-310.
- [ ] The foreign-asset lint passes on the tree and fails on a planted mesh with no provenance.
- [ ] Regenerating the starter pack from its recorded seeds reproduces every `SceneId`.

## Test plan

### Unit tests

- `asset_source_frob_scene_row_valid`.
- `generated_pack_has_single_licence_entry`: for each of the seven archetypes' proxies.
- `variant_copies_licence_entry`: a mirrored layout and a clutter variant.
- `pack_is_pure_procedural_true_for_generated`, `…_false_with_imported_asset`.
- `generators_have_no_foreign_assets`: with a planted file.
- `starter_pack_regenerates_bit_identical`.

### End-to-end script and logging

`scripts/e2e/assets_licences.sh` (NEG-16), scene-asset loop: `frob scene gen` for two archetypes, `frob scene
show --json` to read the licence manifest, an export, and the starter-pack scan; log lines carry the source
key, class, entry count, `SceneId` and the purity verdict.

### Known answers and negative controls

The rule "the starter pack ships only synthetic and redistributable fixtures". Controls: the planted
research-only stand-in; the planted unprovenanced mesh; a generator that embeds a third-party table without a
sidecar. Determinism: `SceneId` equality across runs and platforms.

## Considerations

- Interior generation is not claimed as new; what the project adds to a generated interior is listed in the
  prior-art bead of this unit, and this source row makes no novelty claim.
- "Redistributable" covers the generated geometry; hazard dimensions inside it remain `assumed` until cited,
  which is a separate count in the pack's manifest and a separate refusal.

## References

Appendix G.2 (row 1); §19.7; §19.9; §37; FROB-REF-310.
