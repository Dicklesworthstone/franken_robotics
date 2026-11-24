---
key: "NEG-21"
title: "scene asset source `procthor` (code Apache-2.0; dataset licence not verified): generator reference only"
type: "task"
priority: 2
milestone: "M3"
labels: ["m3", "habitat", "frob-scene", "solid", "test", "asset", "registry"]
parent: "WP-23"
deps: ["N19a-46", "N19b-02", "N37-18", "NAB-14"]
needs: []
related: ["NEG-27", "NEGp1-51"]
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
2. `procthor_not_importable_by_default`: `frob scene import --kind asset <file> --source procthor --profile
   asset-default --key <k> --json` in default mode is refused by the default-pipeline filter with a message
   that the dataset licence is not verified; as a user-supplied import (`--user-supplied`: the inputs of
   `scene.import`, N19a-46) the entry takes the row's class, `unknown`, with `redistribution = NotRead`
   (N19b-02's mapping for a row with `read = false`). A declaration that names a registered row can never make
   the entry less restrictive than that row: N19a-46 refuses a `--licence-class` that would weaken the row's
   class with `FROB-REF-802` (exit 3), naming the flag and the row, and since `unknown` is the most restrictive
   class, every declaration other than `unknown` is refused here. (An earlier text said "unless the declared
   class allows it", which would let a flag override a licence nobody read and contradicted this bead's own
   criterion that the entry cannot be exported. A user who has read the dataset's licence themselves imports
   with `--source user-supplied --licence-class <class>`, names no row, and owns that declaration.) The pack is
   written under the private root, as every `unknown` or research-only import is, so
   an export of a pack or scene variant containing it is refused by the pack export check with FROB-REF-310
   (exit 3), and any other export whose lineage reaches it (a still, a report, a run bundle) is refused at the
   export door of N37-18, exit 5: with FROB-REF-731, since the pack always sits under the private root and the
   door checks the private mark before the licence set, and with FROB-REF-733 (the licence set) only in a
   session holding `private:export` and the flag; one code per request, as N19b-02 divides them.
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
- `procthor_declaration_cannot_weaken`: `--user-supplied --licence-class attribution` naming the row is
  refused with FROB-REF-802 (exit 3), the flag and the row named; with no class the entry is `unknown`, `NotRead`.
- `procthor_user_supplied_entry_unexportable`: FROB-REF-310 (exit 3) on a pack export; FROB-REF-731 at the door
  (exit 5) on a still of a scene containing it, and FROB-REF-733 in a session holding `private:export` and the
  flag; never both codes for one request.
- `procthor_not_a_dependency`: with a planted reference in a scratch manifest.

### End-to-end script and logging

`scripts/e2e/assets_licences.sh` (the M3 scene-asset loop, owned and run by NEGp1-51, which depends on this
bead; this bead lists the steps of its row), on a `synthetic` stand-in (a small house-like mesh authored in the
repository, `tests/fixtures/synthetic/procthor-like/`): the absence scan; `frob --json scene import --kind asset
<stand-in> --source procthor --profile asset-default --key al-procthor-1` (refused by the default-pipeline
filter, the reason naming the unverified dataset licence); the same with `--user-supplied --licence-class
attribution --key al-procthor-2` (refused with `FROB-REF-802`, exit 3, naming `--licence-class` and the row: a
declaration cannot weaken the row); the same with `--user-supplied` and no class (`--key al-procthor-4`: a
private pack whose entry is `unknown`, `NotRead`); `frob --json artifact export <pack> --out exports/ --key
al-procthor-3` (`FROB-REF-310`, exit 3);
a still of a run in a scene containing it, exported (`FROB-REF-731` at the door, exit 5, because the pack lies
under the private root). Log lines carry `ts`, `script`, `bead: "NEG-21"`, `step`, `event`, the exact command,
exit code, wall time, the `evidence` block, the source key, class, `read` flag, each refusal code with its exit
code and raising site (filter, pack check or door), and no geometry.

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
