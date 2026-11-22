---
key: "N34-CX"
title: "Workspace counterexample registry: CX-358 to CX-363 (identity, auditor and layer-order shortcuts)"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "workspace", "frob-conformance", "solid", "registry", "test"]
parent: "WP-01"
deps: ["BD-025", "STD-TEST"]
needs: ["§38.11: the negative-control registry and the counterexample catalogue seeds in the conformance crate (BD-146 extends them at M1)"]
related: ["N34-KA", "N34-REF", "N34-17", "N34-18", "N34-19", "N34-21", "N34-22", "N34-24", "N34-25", "BD-003", "N34-14"]
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
noticed. Each seeds at least one record in the negative-control registry of the conformance crate, attached to
the gates of the rule it protects. This bead registers the six entries as data and binds each to its controls
and tests. Tier: Solid.

## Scope

In: the six catalogue rows; their negative-control records; the bindings to the tests that exercise them; the
registry self-check for this chapter. Out: the mechanisms that defeat each shortcut and their tests (named per
row).

## Technical approach

| CX | The tempting shortcut | The smallest case that breaks it | The rule it protects | Controls seeded (kind; gate) | Exercised by |
|---|---|---|---|---|---|
| CX-358 | Hash the file as written | Two files that differ in key order or white space, such as `{"b":1,"a":2}` and `{"a":2,"b":1}`, would be two designs, and every memoized analysis of the first would miss on the second | Canonical serialization before hashing | A writer build with key sorting disabled (mutation; GATE-WS-02, GATE-WS-05); an identity function that accepts raw file bytes (mutation; GATE-WS-05) | N34-18, N34-21, N34-22, N34-23 |
| CX-359 | Hash the in-memory value | A hash of a debug rendering or of a struct's layout changes with the compiler, so the same design would change identity at a pin move and every cached result would be orphaned | Identity is the hash of the canonical bytes | Fixture identities recorded at a format's first release and compared after every pin move (planted-input: a fixture whose expected identity was produced by a debug-rendering hash must fail; GATE-WS-02) | N34-18, BD-410 |
| CX-360 | The view is lossless, so hash either form | The S-expression and the JSON of one design are different bytes; a cache keyed on whichever arrived first holds two entries for one design and two verdicts a reader could cite as independent | Only the canonical JSON is hashed; a view has no identity | A build whose identity-required path converts views silently (mutation; GATE-WS-02); a `View` type given an identity method (mutation, caught by a compile-fail test) | N34-19, N34-24, N34-23 |
| CX-361 | Tolerate unknown fields | A reader that ignores an unknown required field such as a contact-law selector simulates a different robot from the one described and reports it under the same identity | Unknown required fields are refused; a reader never parses on a best-effort basis | An owner reader patched to ignore one unknown member (mutation; GATE-WS-02: the mutation clause then finds an accepted change with an unchanged identity) | N34-19, N34-22, BD-141 |
| CX-362 | Reuse `frob-core`'s hashing in the checker | A canonicalization defect in `frob-core` that drops a field before hashing would be reproduced by the checker, and a bundle whose statement differs from the one the proposer evaluated would pass; with two implementations the hashes disagree and the bundle is rejected | The certificate checker depends on the standard library only, and no producer depends on it (the auditor-independence invariants) | A scratch `frob-cert` that depends on `frob-core` (planted-input; GATE-WS-03); a function pasted from `frob-core` into `frob-cert` (planted-input; GATE-WS-03); a defective shared canonicalizer that drops a field, with a bundle whose statement differs (mutation; GATE-WS-03 and the checker-independence gate of the certificate chapter) | N34-25, BD-026, N25b-05 |
| CX-363 | Crates inside one layer need no order | `frob-arena` could depend on `frob-opt` for comparison while `frob-opt` depends on `frob-arena` for batteries; Cargo rejects the cycle only after both are written | The intra-layer rank of rule L-1; the layer check | An edge against rank planted in a scratch copy (planted-input; GATE-WS-01, plant 2 of 7) | N34-02, BD-003, N34-14 |

How the rows become controls (the registry rules that apply):

1. Each row yields at least one record in the negative-control registry: a stable identifier, the catalogue
   entry as its `seed`, the gates that must catch it, a `kind` (`mutation`, `planted-input`), the `target` (the
   function or check), the `defect` in one sentence, and the expected observation.
2. A control is a defect, not a boundary. "A view is refused where an identity is required" tests the limit; the
   control is the build in which the limit was removed and the script must then fail.
3. A control is run every time its gate is run; a gate run in which a control was not caught is a failure of
   the gate, whatever the passing test says.
4. Mutant builds are selected by a custom compiler configuration flag and never by a Cargo feature, so that
   feature unification cannot switch a mutant on in an unrelated build.
5. Entries are never renumbered; a new workspace counterexample appends a row. If a false identity, a false
   certificate acceptance or a real dependency cycle is found in use, the case that exposed it becomes a
   permanent row here.

A seventh shortcut is defeated in this chapter without a catalogue number: "a passing build means the JSON
library behaves the same everywhere". Its smallest case (`{"b": 1, "a": 2}` re-serialized through a library map
is `{"a":2,"b":1}` in one build and `{"b":1,"a":2}` in another when an unrelated crate turns on insertion-order
maps) is carried as a control of GATE-WS-05 by the four-way build bead; it is recorded here so that the
catalogue's printout can pick it up when its owner assigns a number.

## Acceptance criteria

- [ ] The six rows are in the counterexample catalogue with every column above; identifiers CX-358 to CX-363
      are neither reused nor renumbered.
- [ ] Each row has at least one record in the negative-control registry whose `seed` is its identifier and
      whose `gate` list is non-empty: in total at least ten records for this chapter (two for CX-358, one for
      CX-359, two for CX-360, one for CX-361, three for CX-362, one for CX-363).
- [ ] Each control's `target` exists in the tree once its bead lands; before that the control is listed as
      unbound and its gate cannot record a pass.
- [ ] The registry self-check fails when a row's last control is removed.
- [ ] The unnumbered JSON-library case is present as a control of GATE-WS-05 with a note that it has no
      catalogue number.

## Test plan

### Unit tests

`cargo test -p frob-conformance registry_cx_workspace`: `six_rows_present`; `ids_are_358_to_363`;
`every_row_seeds_a_control`; `every_control_names_a_gate`; `control_targets_exist` (once landed);
`mutant_selection_is_cfg_not_feature` (no Cargo feature in the workspace is named after a control);
`removing_last_control_fails_check`.

### End-to-end script and logging

No script of its own. The controls run inside `scripts/e2e/workspace_checks.sh` (CX-363),
`scripts/e2e/canonical_json.sh` (CX-358, CX-359, CX-360), `scripts/e2e/format_registry.sh` (CX-358, CX-360,
CX-361) and `scripts/e2e/auditor_independence.sh` (CX-362). Each script logs, for every control it runs, the
control's identifier, its catalogue seed, whether it was caught, and the gate it belongs to, in the structured
format of STD-E2E; the `summary` line counts controls run and caught.

### Known answers and negative controls

The six rows are the content. Hand-checkable cases restated: the two key orders of one two-member object; the
S-expression and JSON of one design; the arena and optimizer crates in one layer. Negative control of the
registry itself: a control whose expected observation is "the test passes" (a boundary, not a defect) is
rejected by the registry's schema.

## Considerations

- CX-362's strongest control needs a deliberately defective shared canonicalizer; it is expensive to keep
  building and is therefore run at gate time, not on every commit. The cheap controls (the dependency plant, the
  pasted function) run on every commit through the dependency check.
- CX-359 can only be observed across a pin move. Its control is therefore a fixture comparison that becomes
  meaningful at the first toolchain train; until then it is registered and reported as not yet exercised, which
  the milestone exit review reads.
- The catalogue is data and the appendix is its printout; the row text here is the registry's, and a chapter
  that restates a case repeats it from the registry.

## References

Appendix N.31 (CX-358 to CX-363), §34.3, §34.9, §34.11, §34.14, §38.11, §9 (D-9); GATE-WS-01, GATE-WS-02,
GATE-WS-03, GATE-WS-05; INV-LEMMA-05, INV-WS-02.
