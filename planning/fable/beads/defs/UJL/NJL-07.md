---
key: "NJL-07"
title: "fixtures/golden/appendix-l: layout, three layers, identity map, story-consistency check, the blessing rule"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "gauntlet", "frob-conformance", "solid", "test", "registry", "delivery", "review-notes-open"]
parent: "WP-14"
deps: ["BD-005", "N34p4-02", "N40-05", "STD-E2E", "STD-TEST"]
needs: []
related: ["NJL-02", "NJLx-02"]
gates: []
plan_refs: ["Appendix L (preamble, L.1 to L.9)", "Appendix H.5", "§34.1", "§34.9", "§38"]
---
## Background

FrankenRobotics (a robotics simulation and design system reached through the command line `frob`, the daemon
`frobd` and the MCP server `frob-mcp`) identifies every artifact by the hash of its canonical bytes. Its
specification closes with nine **worked artifacts** that tell one story: a quadruped that carries a tray
through a commercial-kitchen galley and sets it down at a pass counter. They are DESIGN specimens: every number
illustrates a format and is not a result, except that wherever an artifact contains arithmetic, the arithmetic
is exact (it was recomputed when the specification was written). Each artifact uses the schema of the chapter
that owns it.

The story. A static mission query asks which stance half-widths and knee ratios can support the loaded tray;
its answer has an inner set, an outer set and a band of unresolved cells. A design is created at the centre of
an inner cell, with its leg as a reusable module. The dynamic task is the tray delivery. A search for
adversarial worlds finds a failure and shrinks it to three edits; an interventional autopsy says which of them
can be undone to rescue the run. After one design edit a paired study compares the versions, an energy budget
bounds what a stop at the counter can deliver, the blinded lane qualifies the design, and the certificates and
the receipt travel in one bundle.

Each artifact becomes a golden fixture with its own bead (`ART-L1` to `ART-L9`): the artifact carried
completely, its schema validation, its round-trip test and the end-to-end script that produces it from the
command line. This bead builds what the nine share: where golden fixtures live, how a printed specimen with
abbreviated identities (`d:7a3e…`) becomes a file that parses, how the nine are kept consistent with one
another, and who may change a golden file.

Why the layers (DESIGN). A golden file that is regenerated from the program it tests proves nothing, and a
specimen copied from a specification cannot be compared byte for byte with real output, because its hashes are
abbreviated and some of its numbers are illustrative. Three layers separate the cases. Rejected: one golden
file per artifact regenerated on demand (a regression would be blessed by the next regeneration); specimens
kept only in documentation (nothing would notice when a format drifts away from them).

## Scope

In: the directory layout; the three layers and the four field classes; `MANIFEST.toml` and `IDENTITIES.toml`;
the tool `frob-golden` (`check`, `resolve`, `bless`); the story-consistency check across the nine artifacts;
the story runner script and its record in the exit runner's check table. Out: the content, schema validation and end-to-end script of each artifact (the
`ART-L*` beads); the format registry and its round-trip gate (BD-141, BD-410); help-schema snapshots of verbs,
which live beside these fixtures under `fixtures/golden/verbs/` (bead NJL-02).

## Technical approach

**Layout.**

```
fixtures/golden/appendix-l/
  MANIFEST.toml      one row per file: artifact, layer, schema, sha256, bead, milestone
  IDENTITIES.toml    abbreviation -> full identity, the artifact that defines it, the blessing record
  L1/ … L9/          one directory per artifact; file names are fixed by the artifact's bead
fixtures/golden/verbs/<cluster>/<verb>.help.json      (owned by the verb conformance harness)
```

The repository layout so far has `tests/fixtures/oracle/` for captures from pinned external oracles. Golden
fixtures are a different thing (they come from the specification and from blessed output of this program), so
they get their own root, `fixtures/golden/`, which this bead adds to the layout table of the workspace
documentation (N34p4-02's table, a dependency).

**Three layers per artifact.**
1. `*.specimen.*`: the artifact exactly as specified, abbreviated identities included. Hand-entered once from
   the artifact's bead, pinned by SHA-256 in the manifest, never regenerated and never edited. It is validated
   leniently: every field must be known to the schema, and an abbreviated identity must match its pattern
   (`^[a-z]+:(b3:)?[0-9a-f]{4}…$` and the like).
2. `*.resolved.*`: the specimen with every abbreviated identity replaced by the full identity from
   `IDENTITIES.toml`. Written by `frob-golden resolve`. This is the file that must parse strictly, validate
   against its schema, and round-trip (canonical → value → canonical with zero byte differences).
3. Produced output: what the artifact's end-to-end script obtains from `frob`. It is compared with the resolved
   layer field by field according to the field's class.

**Four field classes** (declared per artifact in `L<n>/fields.toml`, by JSON pointer; unlisted fields default
to `exact`):

| Class | Comparison | Typical fields |
|---|---|---|
| `exact` | Produced value equals the resolved value (strings byte for byte; rationals after reduction) | Authored parameters, cell boxes, witness forces, counts of the pre-analysis, integer energy terms |
| `identity` | Produced value equals the entry of the identity map | Design, task, mission, module and bundle hashes |
| `structural` | Type, shape, range and the stated invariants hold; the value is free | Outcomes of simulated episodes (seeds failing, margins, cost in GPU seconds), job handles |
| `illustrative` | Not compared; listed so that a reader knows the specimen's value is a format example | Estimates, wall times, token-budget texts |

**The identity map.** The specimens share identities; the map makes the sharing explicit and checkable.

```toml
[[identity]]
abbrev = "d:7a3e…"
kind = "design"
defined_by = "L1"            # the artifact whose canonical bytes hash to it
used_by = ["L8", "L9"]
full = ""                    # empty until blessed
blessed = { by = "", commit = "", log = "" }
```

Abbreviations and where they recur (the consistency check requires each to resolve to one value everywhere):
`d:7a3e…` the design of L.1 (L.8 arm A, L.9); `d:c05b…` the design after the foot edit (L.8 arm B, L.9);
`q:3c9d…` the mission of L.4 (L.1's `meta.source`, L.9), whose digest L.7 prints as `b3:3c9d…` in
`query.blake3` (a row of its own, defined by L4, whose full identity the check requires to carry the same hex
digest as `q:3c9d…`); `b3:e17a…` the admitted world set of L.4 (its `world` section; L.7 entry e2's `world-set`
assumption, defined by L4); `b3:9a4e…` the module digest of
L.2; `t:kitchen-tray@88c1…` the task of L.3 (L.6, L.8, L.9); `split:b3:60d3…` the scene split (L.3, L.8);
`m:5d20…` the model and `c:2be8…` the baseline controller (L.6, L.9); `w:3fa9…` the minimal counterexample
world (L.6, L.9) and its passing neighbours `w:51b0…`, `w:e7c4…`, `w:0a9d…` (L.6, L.9); `w:9d04…` and
`r:19e2…` the worst world and its run (L.9); `a:0e51…` and `a:8b27…` the first and refined mission answers
(L.9); `b3:61f0…` and `b3:d4a9…` their bundle roots, the second being the Merkle root of L.7 (L.7, L.9);
`b3:e2a0…` the qualification receipt (L.7 entry e6, L.9); `plan:b3:52e8…` and `b3:5d10…` the study plan and
its pre-analysis (L.8). Cell names (`c:03`, `c:06a`) are labels, not hashes, and are `exact`.

The map has one reader outside this bead: the verb conformance harness (NJL-02) keeps
`fixtures/verbs/identities.toml` for the abbreviations that registry examples use (`r:19e2`, `b3:61f0`,
`d:7a3e`), and from M1 its test `example_identities_agree` requires every abbreviation that both files list to
carry the same full identity. A row renamed, re-blessed or removed here is therefore changed in that file in the
same commit.

**The story-consistency check** (`frob-golden check --story`; also a unit test). Facts that tie the artifacts
together and must hold on the resolved layer:
1. L.1 sits at the centre of the inner cell c:06a of L.4: the cell is v ∈ (6, 8], r ∈ [11/5, 5/2] in the
   chart v = 1/half-width; its centre is v = 7, r = 47/20; L.1 has half-width 1/7 m (the y coordinate of
   `hip_mount`) and knee belt ratio 2.35 = 47/20.
2. The body of L.2 equals the limb chain of L.1 link for link (three revolute joints and a foot), and four
   modules assuming 0.4 W/K each meet L.1's shared sink of 1.6 W/K with nothing to spare.
3. The task identity of L.3 is the one L.6, L.8 and L.9 name; the study defaults of L.3 (fixed-n, n = 64,
   margin 0.05, rule `paired-clopper-pearson/1`, level 0.05) are the study of L.8.
4. The binding atom and margin of L.6 (`upright`, −0.42) are consistent with L.3's atom `upright` (scale 12°):
   a tilt of 17.04° gives (12 − 17.04)/12 = −0.42.
5. The three edits of L.6 are the three interventions of L.9, and each touches a quantity that L.3's
   randomization draws (floor film, door closer torque, tray fill).
6. The cells of L.7's entries e1 to e5 are the cells of L.4's refined answer, thirteen in all; the forces of
   entry e2 at v = 8 are permutations of (88, 32, 40, 0) N; the Merkle root of L.7 is the bundle root of
   L.9's refined answer and of its `prove_bundle` result.
7. The designs of L.8 are L.1 and its edited version; the receipt of L.9 is entry e6 of L.7 (index 1, level
   0.05/2, 200 units).
8. Every body named in L.5's `load_path` resolves to a body or limb instance of the compiled L.1 design.

**Tool** (`crates/frob-conformance/src/bin/frob-golden.rs`; it may link the owner crates' readers, never a
second parser of its own). It is an operator binary of the verification crate under BD-025's rule: a `[[bin]]`
entry of the crate's manifest with `name = "frob-golden"` and `path = "src/bin/frob-golden.rs"` (the manifest
sets `autobins = false`, so a binary exists only by declaration), with a row in the binary table of
`crates/frob-conformance/CONTRACT.md`: owning bead NJL-07; reads the fixture tree, `MANIFEST.toml`,
`IDENTITIES.toml`, the `fields.toml` files and end-to-end logs; writes the resolved layer and, with `bless`,
identities and blessing records in `IDENTITIES.toml`; it runs no tier and writes no gate row. `frob-golden check [--artifact Ln] [--story]`: manifest hashes of the specimen layer;
strict parse, schema validation and round trip of the resolved layer through the owner's reader and writer;
the story facts. `frob-golden resolve`: rewrites the resolved layer from specimen and map; refuses while a
needed identity is empty. `frob-golden bless --artifact Ln --from <e2e log>`: copies full identities from a
passing end-to-end log into the map. It refuses when the log's commit is dirty, when the log failed or was
run in small mode, and when the blessing session is the session that implemented the artifact's owner crate
(the session identifier is read from the delivery log); it appends a blessing record.

**Story runner.** `scripts/e2e/appendix_l_story.sh` runs the artifact scripts that are due at the build's
milestone, in story order (L.4, L.1, L.2, L.3, then L.6, L.9, L.8, L.5, L.7), inside one scratch root so that
identities produced by one step are the inputs of the next, then runs `frob-golden check --story`.

**What is tested when.** This bead closes before any artifact is entered: the nine `ART-L*` beads depend on it
for the layout, the map and the tool. Its own tests therefore run on a test tree
`crates/frob-conformance/tests/data/golden_appendix_l/` (labelled `synthetic`; not golden files and not in
`MANIFEST.toml`) that holds, for each story fact, only the fields the fact reads, entered from Appendix L's
printed values, plus one specimen and resolved pair in a format whose reader is already a dependency (the
canonical JSON of `frob-core`, BD-005) for the manifest, resolve and round-trip paths, and the planted variants.
On the real tree a fact is evaluated only when every artifact it reads is in the manifest; otherwise it is
reported `not-due` with the missing artifacts, never as a pass (facts 1 and 2 read L.1, L.2 and L.4 and become
evaluable at M1; facts 3 to 7 at M3; fact 8 at M4 with L.5). The story runner is attached to step 4 of the
exit runner `scripts/exit/review.sh` (N40-05, a dependency) the way N40-05 attaches every owner's check, by one
record in its table `scripts/exit/checks.toml` written in this bead's change: `id = "appendix-l-story"`, `owner
= "NJL-07"`, `step = 4`, `since = "M1"`, `command = "scripts/exit/appendix-l-story.sh"`. That command wraps the
story script (run as `<command> <Mn> --root <dir> --json`) and prints the check's contract object of N40-05
(`section`: the table of artifact scripts and facts with their outcomes; one `blocks` entry per failed script or
failed fact, naming it), exiting 0 whenever it ran, so a failed script or fact is a blocking item of the exit
record and only a story run that could not start is `not-run`. Every exit from M1 thus runs the artifact
scripts due at the candidate commit and the facts they make evaluable.

## Acceptance criteria

- [ ] The layout exists; `MANIFEST.toml` lists every golden file with its SHA-256, and `frob-golden check`
      fails when one byte of a specimen changes (negative control).
- [ ] `frob-golden resolve` produces resolved files that parse strictly and round-trip with zero byte
      differences (on the test tree at this bead's close; on the real tree for every artifact due at each exit
      from M1); it refuses to write while an identity is empty.
- [ ] `frob-golden check --story` passes on the test tree and fails, naming the fact, on each planted
      inconsistency: a half-width of 1/6 m in L.1 (fact 1); a fourth edit in L.6 (fact 5); a changed force in
      entry e2 (fact 6); a study with n = 96 in L.8 (fact 3). On the real tree a fact whose artifacts are not all
      present is reported `not-due` naming them, never `pass`.
- [ ] An abbreviation resolves to one full identity everywhere it occurs; a planted second value fails.
- [ ] `frob-golden bless` refuses a dirty commit, a failed log, a small-mode log and a self-blessing session,
      each with a named reason; a valid blessing appends its record.
- [ ] The tool contains no parser for any registered artifact format (a source scan; it calls the owner's
      reader), so the single-parser rule of the format registry holds.
- [ ] `frob-golden` is a declared `[[bin]]` with explicit `name` and `path` and has its row in the binary table
      of `CONTRACT.md`; BD-025's test `binaries_declared` passes with it.
- [ ] `scripts/exit/checks.toml` holds the record `appendix-l-story` (step 4, since M1) and
      `scripts/exit/review.sh M1` runs it; a planted failed fact appears among the exit record's blocking parts
      with the check's `id` and `owner`. At this bead's close, with no artifact
      entered, `scripts/e2e/appendix_l_story.sh` logs every artifact and fact as a skip with its milestone and exits 0. At the M1 exit it runs
      the artifact scripts due at M1 (L.4, L.1, L.2) in one scratch root with facts 1 and 2 evaluated and passes;
      L.3 is due at M1 only through its library-level lane (`cargo test -p frob-spec --test golden_l3`, which
      the runner calls as a step), because its script uses the task verbs of M3; a failure blocks the exit;
      artifacts and scripts of later milestones are logged as skips, never as passes.

## Test plan

### Unit tests

`cargo test -p frob-conformance --test golden_appendix_l` (real files in a temporary copy of the test tree
`tests/data/golden_appendix_l/`, and of the real fixture tree where artifacts are present; no mocks):
- `manifest_hashes_match`; `manifest_detects_changed_specimen` (one flipped byte).
- `identity_pattern_lenient`: `d:7a3e…` is accepted in a specimen and refused in a resolved file.
- `resolve_replaces_every_abbreviation`: after `resolve`, no resolved file contains the character `…`.
- `resolve_refuses_empty_identity`.
- `field_classes_default_exact`: an unlisted field is compared exactly; a `structural` field accepts any value
  of the right shape and refuses a wrong type.
- `story_fact_1_cell_centre`: v = 7 and r = 47/20 are inside (6, 8] × [11/5, 5/2] and are its centre; 1/7 m
  and 2.35 match; a planted 1/6 m fails.
- `story_fact_2_module_equals_limb`, `story_fact_3_study_defaults`, `story_fact_4_margin` (−0.42 from
  17.04° and 12°, exactly as decimals), `story_fact_5_three_edits`, `story_fact_6_partition_thirteen`,
  `story_fact_7_receipt_index`, `story_fact_8_load_path`.
- `bless_refusals`: the four refusals, one fixture log each.
- `fact_not_due_when_artifact_missing`: with L.2 removed from a copy of the manifest, fact 2 is `not-due` naming
  L.2, and the summary counts it apart from the passes.

### End-to-end script and logging

`scripts/e2e/appendix_l_story.sh` (header: `bead=NJL-07 milestone=M1 labels=gauntlet,golden,appendix-l seed=1
needs=cpu plants=specimen-byte,story-fact`). It calls the due artifact scripts as steps, passing the scratch
root, and logs through the shared helper to `target/e2e-logs/appendix_l_story/<utc>.ndjson`: per step the
script name, its exit code and wall time and the identities it produced (digests, logged with
`e2e_assert_digest_eq` against the map once blessed); the `frob-golden check --story` command with one
assertion per story fact (observed and expected values, for example `7` and `47/20` against the cell bounds as
rationals); skips for artifacts not due; and the final `summary`. With `E2E_PLANT_FAILURE=specimen-byte` or
`story-fact` the script must exit 1 with a failed assertion naming the plant. Facts not yet evaluable are logged
`skip` with the missing artifacts. No gate is claimed by this script; it runs at every exit from M1 as a step of
the exit runner.

### Known answers and negative controls

- Known answers (exact, hand-checkable): the cell centre (v, r) = (7, 47/20); 4 × 0.4 W/K = 1.6 W/K;
  (12 − 17.04)/12 = −0.42; thirteen cells after the refinement of twelve (one cell split in two).
- The specification's own check of these artifacts (FACT, recorded at plan time): the stance family in exact
  rationals on an 81 × 31 grid against its closed form with zero mismatches; the paving with one inner, three
  unresolved and eight excluded cells; the binomial bounds 0.0432, 0.0504, 0.0425 and 0.0505.
- Negative controls: the changed specimen byte; the four planted story inconsistencies; the duplicate
  identity; the four refused blessings.
- Determinism: `resolve` is a pure function of specimen and map; two runs give identical bytes on Linux and
  macOS (class D0).

## Considerations

- The specimens' field spellings are in places proposals (the specification says so for the task's eight
  free sections and for the study plan). When an owner bead fixes a different spelling, the specimen layer is
  re-entered once from the amended artifact bead and its manifest hash changes in the same commit; that is a
  reviewed change, not a regeneration.
- Privacy: none of the nine artifacts derives from a private scene; the kitchen is a generated archetype. The
  blessing tool nevertheless refuses a log that carries the private mark.
- The blessing rule depends on the delivery log naming sessions; until that log exists, blessing requires an
  explicit `--reviewed-by` and records it.
- Risk: the story facts could be weakened to make a regression pass. Each fact has a planted violation that
  must stay caught, so a weakened fact shows up as an uncaught control.

## References

Appendix L (preamble and the nine artifacts), Appendix H.5, §34.1, §34.9, §38; INV-WS-03 (every registered
extension has exactly one owner crate, and no other crate contains a parser for it), which the tool's
no-parser criterion keeps; GATE-WS-02 (round trips, measured by the format-registry beads).
