---
key: "NEG-17"
title: "scene asset source `frob-scene`: the project's own generators as default source; a pure starter pack"
type: "task"
priority: 2
milestone: "M3"
labels: ["m3", "habitat", "frob-scene", "solid", "test", "asset", "registry"]
parent: "WP-23"
deps: ["BD-162", "BD-225", "BD-226", "N19a-05", "N19b-02"]
needs: []
related: ["NEG-18", "NEG-19", "NEG-21", "NEG-27", "NEG-28", "NEGp1-51"]
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

In: the values of the register row for `frob-scene` and the tests of that row; the rule that generated
content carries exactly one licence entry with this source; the purity check of a pack against its provenance
block (`pack_is_pure_procedural`), applied to generated packs and to every scene of the starter pack; the
regeneration of the starter pack from its recorded seeds; the lint that keeps third-party files out of the
generator crates.

Out, with the bead that has it: the licence manifest type, the schema and loader of the register
`assets/licences/sources.toml` that this row is checked against, the pack export check with its refusal
`FROB-REF-310`, and the licence-manifest check of the starter pack (join class `redistributable`, every entry
of source `frob-scene` or of a repository file tagged `synthetic`), all in N19b-02, on which this bead
depends. The two starter-pack checks are layered: N19b-02's reads the licence manifests, this bead's then
proves from the provenance block that the content really came from the generators. Also out: the grammar
mechanism and the proxies (BD-225, BD-226); the fixture library (N19a-07).

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

**The starter-pack rule (DESIGN, stated identically in N19b-02).** The plan's rule is "the starter pack ships
only synthetic and redistributable fixtures". "Redistributable" is read as the licence class of that name,
and within it as this source alone: the starter pack holds content of source `frob-scene` and repository
files tagged `synthetic`, and nothing else. Reason: the pack then carries no notice obligation and every
scene in it can be regenerated bit for bit from its recorded seeds, so it is checked and not trusted. An
`attribution` asset from another default-pipeline source (a scanned object, a kitchen fixture) is refused
by the starter-pack check of N19b-02 and fails the purity check here, although the same asset may be
exported in an ordinary pack with its notice.

**The row's fields.** The row uses the schema of N19b-02 (`key`, `licence`, `class`, `default_pipeline`,
`redistribute`, `read`, `kind`, `use`, `citation`, `read_on`) with no source-specific field;
`redistribute = "always"` gives every entry of this source `redistribution = Allowed`.

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
      the check fail and the export of the pack refuse with FROB-REF-310 (raised by the pack export check of
      N19b-02; exit 3); adding a stand-in tagged `attribution` from another default-pipeline source makes the
      purity check fail too, with no export refusal.
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

`scripts/e2e/assets_licences.sh` (the M3 scene-asset loop, owned and run by NEGp1-51, which depends on this bead
and sources the STD-E2E helper; this bead lists the steps of its row and writes no script of its own):
1. `frob --json scene gen --family warehouse --seed 1 --key al-frob-scene-w` and `frob --json scene gen --family
   kitchen-commercial/galley --seed 1 --key al-frob-scene-k` (a `run` verb: each call its own key).
2. `frob --json scene show <id>` on each: the licence manifest has exactly one entry, source `frob-scene`, class
   `redistributable`, redistribution `Allowed`, no notice.
3. `frob --json artifact export <id> --out exports/ --key al-frob-scene-x-<n>` on each: exit 0, no notice item.
4. The purity check on both packs and on every scene of the starter pack, and the regeneration of the starter
   pack from its recorded seeds with every `SceneId` reproduced (the checks of this bead's unit tests, run by
   the script as `cargo test -p frob-scene purity` and logged as one command with its exit code).
5. The planted cases through `e2e_expect_caught`: a research-only stand-in added to a scratch copy of the starter
   pack (the purity check fails and the export answers `FROB-REF-310`, exit 3); an `attribution` stand-in from
   another default-pipeline source (the purity check fails, no export refusal); a mesh with no provenance
   sidecar (the foreign-asset lint fails).
Log lines carry `ts`, `script`, `bead: "NEG-17"`, `step`, `event`, the exact command, exit code, wall time, the
`evidence` block, the source key, class, entry count, `SceneId`, the purity verdict, each refusal code with its
exit code, and on failure the first blob whose origin is not a recorded generator call with the pack path.

### Known answers and negative controls

The rule "the starter pack ships only synthetic and redistributable fixtures", in the reading of the DESIGN
note above. Controls: the planted research-only stand-in; the planted `attribution` stand-in from another
default-pipeline source; the planted unprovenanced mesh; a generator that embeds a third-party table without a
sidecar. Determinism: `SceneId` equality across runs and platforms.

## Considerations

- Interior generation is not claimed as new; what the project adds to a generated interior is listed in the
  prior-art bead of this unit, and this source row makes no novelty claim.
- "Redistributable" covers the generated geometry; hazard dimensions inside it remain `assumed` until cited,
  which is a separate count in the pack's manifest and a separate refusal.

## References

Appendix G.2 (row 1); §19.7; §19.9; §37; FROB-REF-310.
