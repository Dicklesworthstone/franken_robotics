---
key: "NEG-21"
title: "scene asset source `procthor` (code Apache-2.0; dataset licence not verified): generator reference only"
type: "task"
priority: 2
milestone: "M3"
labels: ["m3", "habitat", "frob-scene", "solid", "test", "asset", "registry"]
parent: "WP-23"
deps: ["N19b-02"]
needs: []
related: ["NEG-27", "NEG-17", "BD-225", "NEG-16"]
gates: []
plan_refs: ["Appendix G.2 (row 5)", "§19.7", "§19.9", "Appendix H.6"]
---
## Background

ProcTHOR is a published procedural generator of house layouts with a large released dataset of generated
houses. On file its code is Apache-2.0 and the licence of its dataset is not verified (FACT from the research
pass of 2026-11-21; the source register lists the dataset's licence among the items not verified and not
relied on). Its role is the narrowest of all sources: generator reference only. It was read as prior art for
how a procedural house generator is organized; no code, no asset and no dataset record of it is used, and
none may enter the tree. The project still gives it a register row, because the register is where an agent or
a contributor looks to learn whether a source may be used, and the honest answer for a source whose dataset
licence nobody read is "no, and here is why".

## Scope

In: the register row with `read = false` for the dataset; the exclusion checks; the prior-art note. Out: the
licence manifest and register mechanics (N19b-02); the project's own interior grammar (BD-225); the list of
other prior-art generators (NEG-27).

## Technical approach

**Register row.** `key = "procthor"`, `licence = "Apache-2.0"` with `covers = "code"`,
`dataset_licence = "not-verified"`, `class = "unknown"` (the join treats unknown as most restrictive),
`default_pipeline = false`, `redistribute = "never"`, `read = false`, `kind = "generator-reference"`,
`use = "generator reference only"`.

**Exclusion checks.**

1. `procthor_absent_from_tree`: no file in the repository has this source key in a provenance header or
   sidecar; no pack's licence manifest has an entry with it.
2. `procthor_not_importable_by_default`: `frob scene import --source procthor …` in default mode is refused
   by the default-pipeline filter with a message that the dataset licence is not verified; as a user-supplied
   import (the user explicitly names the source and declares a licence class) the entry is written with what
   the user declares, or `unknown`, and `redistribution = Forbidden` unless the declared class allows it, so
   any export containing it is refused with FROB-REF-310.
3. `procthor_not_a_dependency`: no workspace manifest, script or container recipe references the generator.

**What "reference only" permits.** Reading the publication and code to learn structure (rooms from a floor
plan, doors placed on shared walls, object placement by semantic rules) and citing it as prior art. It does
not permit porting code (Apache-2.0 would allow that with notice, but the project's rule for scene generators
is own code only, so that the default source stays under one licence) or copying generated houses.

**Path to use.** If the dataset is ever wanted, the path is an admission reading of its licence from the
primary host, recorded with date and hash; the row then changes class by a dated record. Until then nothing
changes.

## Acceptance criteria

- [ ] The register has the row with `read = false`, class `unknown`, `default_pipeline = false`.
- [ ] The three exclusion checks pass on the tree and each fails on its planted violation.
- [ ] A default-mode import naming this source is refused with the reason; a user-supplied import produces an
      entry that cannot be exported.
- [ ] The prior-art note cites the generator without quoting numbers from it that were not re-read.

## Test plan

### Unit tests

- `asset_source_procthor_row_valid`.
- `procthor_absent_from_tree`: with a planted sidecar naming the source.
- `procthor_not_importable_by_default`: refusal text names the unverified dataset licence.
- `procthor_user_supplied_entry_unexportable`: FROB-REF-310 on export.
- `procthor_not_a_dependency`: with a planted reference in a scratch manifest.

### End-to-end script and logging

`scripts/e2e/assets_licences.sh` (NEG-16), scene-asset loop at M3: the absence scan, the refused default
import, the user-supplied stand-in and its refused export; log lines carry the source key, class, `read`
flag and refusal codes.

### Known answers and negative controls

The rule "an asset whose licence was not read is marked so and stays outside the default pipeline". Controls:
the three planted violations. Determinism: scans are pure functions of the tree.

## Considerations

- The code licence being permissive is not the point: the dataset's is unknown, and a generator's outputs are
  what a scene pipeline would consume.
- Keeping a row for an unused source costs a few lines and prevents a well-meaning contributor from importing
  it on the strength of the code licence alone.

## References

Appendix G.2 (row 5); §19.7; §19.9; Appendix H.6.
