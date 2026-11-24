---
key: "N34-CX"
title: "Workspace counterexample registry: CX-358 to CX-363 (identity, auditor and layer-order shortcuts)"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "workspace", "frob-conformance", "solid", "registry", "test", "review-notes-open"]
parent: "WP-01"
deps: ["BD-025", "N38a-40", "N38b-05", "STD-E2E", "STD-TEST"]
needs: []
related: ["BD-003", "BD-026", "BD-136", "BD-141", "BD-410", "N25b-05", "N34-02", "N34-04", "N34-14", "N34-17", "N34-18", "N34-19", "N34-21", "N34-22", "N34-23", "N34-24", "N34-25", "N34-KA", "N34-REF", "N38b-04", "N41-CX"]
gates: []
plan_refs: ["Appendix N.31", "§34.3", "§34.9", "§34.11", "§34.14", "§38.11", "§9 (D-9)"]
---
## Background

Counterexamples are first-class in this project: where a plausible shortcut is wrong, the smallest case a reader
can check by hand that breaks it is written down, and that case becomes a permanent negative control. A rule
that ships without such a case has not been thought through, and a gate with no planted failure it must catch
validates nothing.

The workspace chapter has six entries, CX-358 to CX-363. Four are the shortcuts by which an artifact's identity
could come to depend on who wrote or read its bytes; one is the shortcut by which a certificate checker could
inherit its producer's defect; one is the shortcut by which a dependency cycle could be built before anyone
noticed. A seventh is stated by the chapter without a catalogue number (a passing build taken to mean the JSON
library behaves the same everywhere). Each entry seeds at least one record in the negative-control registry of the
conformance crate, attached to the gates of the rule it protects. This bead registers the seven entries as data,
with the plants that become their controls and the beads that build and catch them. Tier: Solid.

## Scope

In: the six catalogue entries and the appended seventh in `crates/frob-conformance/registry/counterexamples.toml`,
each with its plants (kind, plant identifier, target, defect, expected observation, gates); the block request for
the seventh's number; the registry test and script for this chapter. Out, with the bead that has it: the
catalogue's record schema and loader, and the seeding tool that writes
`crates/frob-conformance/registry/controls.toml`, allocates `NC-` numbers and proves completeness by the join
(N38b-05): this bead writes no row of `controls.toml`. The control runner (N38b-04). The number blocks (N38a-40).
The mechanisms that defeat each shortcut, their planted variants and their tests (named per row); the chapter's
known answers (N34-KA).

## Technical approach

**Data.** Each row is one entry in the catalogue's record schema (N38b-05): `id` (the CX number, or the tag for
the appended entry until its block is issued), `shortcut`, `case`, `numeric = true`, `plan_case = "numbers"`,
`case_supplied_by` (empty), `rule`, `rule_ids` (the gates of the row with INV-LEMMA-05, INV-WS-02, rule L-1 and
D-9 where the rule names them), `defined_in`, `same_as` (empty: the catalogue marks none of the six ≡), `label`
(DESIGN), `control_kind`, `plant` and, for the appended entry, `slug`. Four entries have more than one plant
(CX-358 four, CX-360 two, CX-361 four, CX-362 three): the entry's `plant` is the list of its plant records, and
the seeding tool derives one control per plant, each with its own kind, target, defect, expected observation and
gates as tabulated. Each plant's `id` is `<planting bead>:<plant identifier>` (N38b-05, whose own example is
`N34-18:writer-no-key-sort`), the planting bead being the bead the last column names as building it; the name
after the colon is the Cargo feature or the fixture. The texts of the control column are what the seeding tool copies into each control's `target`, `defect`
and `expect`.

| CX | The tempting shortcut | The smallest case that breaks it | The rule it protects | Controls (kind; plant; target; defect; expected observation; gates) | Planted and caught by |
|---|---|---|---|---|---|
| CX-358 | Hash the file as written | Two files that differ in key order or white space, such as `{"b":1,"a":2}` and `{"a":2,"b":1}`, would be two designs, and every memoized analysis of the first would miss on the second | Canonical serialization before hashing (§34.9); GATE-WS-02 | (1) mutation; `writer-no-key-sort`; the canonical writer of `frob-core`; key sorting disabled; the two files above receive two identities; GATE-WS-02, GATE-WS-05. (2) mutation; `identity-on-raw-bytes`; the identity function; identity computed on the file's raw bytes; the respelled file changes identity; GATE-WS-05. (3) mutation; `utf16-sort`; the canonical writer of `frob-core` (`frob_core::canon`, writer rule 2); members sorted by UTF-16 code unit instead of by code point; names outside the basic plane sort differently and a file receives a second identity, so the supplementary-plane ordering test fails; GATE-WS-05. (4) mutation; `escape-all-non-ascii`; the canonical writer (`frob_core::canon`, writer rule 3); every non-ASCII code point written as `\uXXXX` instead of the minimal escapes; the same text is written as other bytes and receives another identity, so `minimal_escapes` fails; GATE-WS-05 | N34-18 builds all four; caught by N34-21 (`canonical_json.sh`), N34-22 and N34-23; (3) and (4) also by N34-18's own tests |
| CX-359 | Hash the in-memory value | A hash of a debug rendering or of a struct's layout changes with the compiler, so the same design would change identity at a pin move and every cached result would be orphaned | Identity is the hash of the canonical bytes (§34.9) | planted-input; `debug-render-identity`; the fixture identities of each format; a fixture whose expected identity was produced by a debug-rendering hash; the comparison must fail; GATE-WS-02 | N34-18 (the canonical identity); BD-141 records the identities at the first release (M1) and runs the planted fixture; BD-410 compares them after every pin move (M5) |
| CX-360 | The view is lossless, so hash either form | The S-expression and the JSON of one design are different bytes; a cache keyed on whichever arrived first holds two entries for one design and two verdicts a reader could cite as independent | Only the canonical JSON is hashed; a view has no identity (§34.9) | (1) mutation; `view-converted-silently`; the identity-required parameters of `frob-verbs`; the path converts a view silently; the S-expression and the JSON of one design get two cache entries; GATE-WS-02. (2) mutation; `view-has-identity`; the `View` type; an identity method added; a compile-fail test must not compile; GATE-WS-02 | N34-24 (1) and N34-19 (2); caught by N34-23 |
| CX-361 | Tolerate unknown fields | A reader that ignores an unknown required field such as a contact-law selector simulates a different robot from the one described and reports it under the same identity | Unknown required fields are refused (§34.9); a reader never parses on a best-effort basis | (1) mutation; `reader-ignores-unknown`; an owner reader of the format registry; one unknown required member ignored; the mutation clause of GATE-WS-02 finds an accepted change with an unchanged identity; GATE-WS-02. (2) mutation; `dup-first-wins`; the reader of `frob_core::json` (the duplicate-member rule of §34.14); an object with a repeated name keeps the first value and is admitted; `{"wheel_radius_mm": 60, "wheel_radius_mm": 75}` is admitted where `duplicate-member` must refuse it, `dup_plain` fails and the gate run admits the 160 duplicates; GATE-WS-05. (3) mutation; `dup-last-wins`; the same reader; the last value kept; as (2); GATE-WS-05. (4) mutation; `skip-escape-decode`; the same reader; names compared before escape decoding; `{"a":1,"a":2}` is admitted, `dup_after_escape` fails and the gate run admits the 80 duplicates that differ only by escapes; GATE-WS-05 | (1) N34-22 (the round-trip harness, with N34-19's registry types); BD-141 for the M1 formats. (2) to (4) N34-17 builds them; caught by N34-17's tests and by N34-21 (`canonical_json.sh`) |
| CX-362 | Reuse `frob-core`'s hashing in the checker | A canonicalization defect in `frob-core` that drops a field before hashing would be reproduced by the checker, and a bundle whose statement differs from the one the proposer evaluated would pass; with two implementations the hashes disagree and the bundle is rejected | INV-LEMMA-05: `frob-cert` depends on the standard library only, and no producer depends on it (INV-WS-02); GATE-WS-03 | (1) planted-input; `cert-depends-on-core`; the manifest of a scratch `frob-cert`; a dependency on `frob-core` added; `check-deps` must refuse it; GATE-WS-03. (2) planted-input; `cert-pasted-function`; the source of a scratch `frob-cert`; a function pasted from `frob-core`; the shared-source scan must refuse it; GATE-WS-03. (3) mutation; `shared-canonicalizer-drops-field`; one canonicalizer shared by producer and checker; a field dropped before hashing; a bundle whose statement differs is accepted, where two implementations disagree and reject it; GATE-WS-03 and the certificate chapter's checker-independence gate GATE-AUGUR-06 | N34-25 (1, 2); the script N25b-05 (`auditor_independence.sh`) on BD-026's auditor (3), with BD-136's checker-independence campaign at M1 |
| CX-363 | Crates inside one layer need no order | `frob-arena` could depend on `frob-opt` for comparison while `frob-opt` depends on `frob-arena` for batteries; Cargo rejects the cycle only after both are written | The intra-layer rank of rule L-1; `check-layers`; GATE-WS-01 | planted-input; `edge-against-rank`; a manifest in a scratch copy; an intra-layer edge against rank; `check-layers` must refuse it (plant 2 of the seven of GATE-WS-01); GATE-WS-01 | BD-003 (the check) on N34-02's crate map; caught by N34-14 (`workspace_checks.sh`) |

**The appended entry.** A bead never invents a number above the plan-time maximum (CX-384): this bead files one
`[[block]]` request in `crates/frob-conformance/registry/allocations.toml` (schema `frob.gauntlet.alloc/1`,
N38a-40) with `registry = "cx"`, `count = 1`, `owner = "N34-CX"`, `section = "§34.9"`; the allocator fills `first`
and `last`, and the tag is mapped to the block's number in one commit and kept in `slug`. A tag still used as an
identifier in a registry, a test name or a derivation file at this bead's milestone (M0) fails the ledger check
`provisional-tag-past-due`.

| Tag | The tempting shortcut | The smallest case that breaks it | The rule it protects | Control (kind; plant; target; defect; expected observation; gates) | Planted and caught by |
|---|---|---|---|---|---|
| `ws-json-library-map` | A passing build means the JSON library behaves the same everywhere | `{"b": 1, "a": 2}` re-serialized through a library map is `{"a":2,"b":1}` in one build and `{"b":1,"a":2}` in another, when an unrelated crate turns on insertion-order maps by feature unification | Canonical bytes come from the project's own writer, never from a library map; the JSON library's version and unified feature set are pinned per binary | mutation; `library-map-insertion-order`; the conformance crate's mutant writer that serializes through the library's map (N34-18), built in the four feature configurations; the digests differ across builds under `preserve_order`; GATE-WS-05 | N34-18 (the mutant writer); caught by N34-21 (four-way feature build), with N34-04's JSON-library guard |

The delivery registry (N41-CX) states a related case for the risk register (a dummy dependency enabling
insertion-ordered maps, GATE-WS-07); it protects the risk register's detector, a different rule, and keeps its own
entry and control; the one numbered later carries `same_as` the other.

**How the rows become controls (the registry rules that apply).**

1. The seeding tool derives one control per plant with a stable `NC-` identifier, the catalogue entry as its
   `seed`, the gates that must catch it, its `kind`, `target`, `defect` and expected observation. This bead writes
   none of them.
2. A control is a defect, not a boundary. "A view is refused where an identity is required" tests the limit; the
   control is the build in which the limit was removed and the script must then fail.
3. A control is run every time its gate is run; a gate run in which a control was not caught is a failure of the
   gate, whatever the passing test says.
4. Mutant builds use the suite's one mutation mechanism, the one the control runner (N38b-04) applies: a
   `mutation` control is a Cargo feature at its target site in the owning crate, built only under
   `--features gauntlet-mutations` and that feature. DESIGN (the workspace chapter first drafted a `--cfg
   frob_nc=…` compiler flag instead): one mechanism lets the runner plant every control of every chapter, and a
   per-crate feature rebuilds only the owning crate and its dependents, where a compiler flag set for the whole
   build rebuilds everything per mutant. The risk the flag was meant to close, a mutant switched on by feature
   unification from an unrelated crate, is closed by the runner's guard: a plant feature without
   `gauntlet-mutations`, or under the release profile, is a compile error, and every product binary is checked by
   `cargo tree -e features` to carry no plant feature.
5. Entries are never renumbered; a new workspace counterexample appends an entry through a block. If a false
   identity, a false certificate acceptance or a real dependency cycle is found in use, the case that exposed it
   becomes a permanent entry here.

## Acceptance criteria

- [ ] `counterexamples.toml` holds CX-358 to CX-363 and the appended entry with every column above in the schema
      of N38b-05; identifiers CX-358 to CX-363 are neither reused nor renumbered; every planting bead named is live
      in the bead index.
- [ ] After the seeding tool of N38b-05 is re-run, the entries yield sixteen controls in `controls.toml` (four for
      CX-358, one for CX-359, two for CX-360, four for CX-361, three for CX-362, one for CX-363, one for the
      appended entry), each with its `seed` and a non-empty `gate` list; the second run changes no byte; the join passes;
      no entry ends in the findings file. No record of `controls.toml` is written by this bead.
- [ ] The appended entry carries a number from a block issued to N34-CX, with its tag in `slug`;
      `registry::alloc::validate` passes and no tag remains as an identifier at M0.
- [ ] Before its planting bead lands, a control is reported `stale` by the runner and its gate cannot record a
      pass; the registry test reports it, with the bead, and does not fail this bead for it.
- [ ] The registry test fails when an entry's last plant is removed, and `scripts/e2e/registry_n34_cx.sh` passes
      and rejects each of its planted registry defects.

## Test plan

### Unit tests

`crates/frob-conformance/tests/registry_cx_workspace.rs`, run as `cargo test -p frob-conformance --test
registry_cx_workspace` (real registry files and scratch copies, no mocks): `seven_entries_present` (six by number,
the seventh by tag before issue and by number and `slug` after); `ids_are_358_to_363`;
`every_entry_has_plants_and_gates`; `plants_per_entry` (4, 1, 2, 4, 3, 1, 1); `every_gate_exists` (in
`registries/gates.toml`); `planting_beads_live`; `mutation_plants_use_the_runner_mechanism` (every `mutation`
plant of this chapter whose planting bead has landed names a feature declared in its target crate, and a scratch
build enabling it without `gauntlet-mutations` fails to compile); `no_product_binary_carries_a_mutant`
(`cargo tree -e features` on `frob`, `frobd`, `frob-mcp` and `frob-verify` shows no plant feature);
`removing_last_plant_fails_check`; `boundary_is_not_a_control` (a scratch plant whose expected observation is
"the test passes" is rejected); `key_order_case` (`{"b":1,"a":2}` and `{"a":2,"b":1}` canonicalize to the same
bytes `{"a":2,"b":1}` through the canonical writer and to two digests when hashed as written); `block_request_of_one`.

### End-to-end script and logging

`scripts/e2e/registry_n34_cx.sh` (header `bead=N34-CX milestone=M0 labels=workspace,registry
plants=drop-plant,unknown-gate,boundary-plant,stale-tag`): runs the registry test; re-runs the seeding tool
(`cargo test -p frob-conformance --test seed_controls -- --ignored seed`) and the join (`cargo test -p
frob-conformance --test seed_join`) on a scratch copy of `registry/` and asserts by digest that a second seeding
run is a no-op; plants, one at a time on scratch copies, an entry with its last plant removed, a plant naming a gate
absent from the register, a plant whose expected observation is a boundary, and the tag left as an identifier at
M0, and asserts each is rejected. For each control whose planting bead has landed it runs the gate's tier target in
the form of ADR-GAUNT-01 (`FROB_TEST_SEED=<seed> cargo test -p frob-conformance --test <tier from the gate record>
-- <gate>`, then with `--features gauntlet-mutations` and the plant's feature) and asserts pass, then fail; it calls
no `frob` verb. The controls are also exercised by the scripts of their planting beads: `workspace_checks.sh`
(N34-14: CX-363), `canonical_json.sh` (N34-21: CX-358, CX-359, CX-360, CX-361's plants 2 to 4, the appended entry), `format_registry.sh` (N34-23: CX-358,
CX-360, CX-361) and `auditor_independence.sh` (N25b-05: CX-362), each of which logs the control's identifier, its
catalogue seed, whether it was caught and its gate. This script sources the STD-E2E helper and writes
`target/e2e-logs/registry_n34_cx/<utc-timestamp>.ndjson` with a transcript: every line has `ts`, `script`, `bead`,
`step`, `event`, the exact command line, exit code, wall time and the evidence block (claim Known answer, class
Tested, determinism D0); `metric` lines give entries, plants, controls caught, stale and missed against 16; the
digests of the catalogue and of the scratch `controls.toml` before and after the second seeding; per planted
defect an `assert` with the expected and observed failing check; on failure the first differing entry and the
scratch path; the `summary` line counts assertions and names GATE-GAUNT-01 and the gates touched.

### Known answers and negative controls

The seven entries are the content. Hand-checkable cases restated: the two key orders of one two-member object; the
S-expression and JSON of one design; the arena and optimizer crates in one layer. Negative controls of the registry
itself: a plant whose expected observation is "the test passes" (a boundary, not a defect) is rejected; an entry
with its last plant removed fails the check. Determinism: D0, identical twice on one machine and on both platforms.

## Considerations

- CX-362's strongest control needs a deliberately defective shared canonicalizer; it is expensive to keep
  building and is therefore run at gate time, not on every commit. The cheap controls (the dependency plant, the
  pasted function) run on every commit through the dependency check.
- CX-359 can only be observed across a pin move. Its control is therefore a fixture comparison that becomes
  meaningful at the first toolchain train; until then it is registered and reported as not yet exercised, which
  the milestone exit review reads.
- The catalogue is data and the appendix is its printout; the entry text here is the registry's, and a chapter
  that restates a case repeats it from the registry.
- An entry with several plants writes `plant` as a list of plant records, which N38b-05's schema admits; the
  registry test fails rather than seed one plant and drop the others.

## References

Appendix N.31 (CX-358 to CX-363), §34.3, §34.9, §34.11, §34.14, §38.11, §9 (D-9); GATE-WS-01, GATE-WS-02,
GATE-WS-03, GATE-WS-05; INV-LEMMA-05, INV-WS-02.
