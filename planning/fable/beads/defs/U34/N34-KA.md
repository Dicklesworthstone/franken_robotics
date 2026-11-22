---
key: "N34-KA"
title: "Workspace known-answer registry: KA-781 to KA-784"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "workspace", "frob-conformance", "solid", "registry", "test"]
parent: "WP-01"
deps: ["BD-025", "STD-TEST"]
needs: ["§38.6: the loader of the known-answer registry in the conformance crate, which binds each row to its realizing test (BD-146 extends it at M1)"]
related: ["N34-CX", "N34-REF", "N34-14", "N34-21", "N34-22", "N34-23", "N34-25", "BD-141", "BD-410", "BD-026", "N25b-05"]
gates: []
plan_refs: ["Appendix M.33", "§34.3", "§34.9", "§34.11", "§34.13", "§38.2", "§38.6"]
---
## Background

A known answer is an analytic or synthetic truth the implementation must recover, with a derivation on file, a
tolerance, a verification tier and, wherever one exists, a negative control: a planted defect the test must
catch, because a gate that nothing can fail validates nothing. The known-answer suite is carried by the
conformance crate's registry; mechanism beads cite the numbers and restate the values they need.

The workspace chapter (where code lives, which crate may depend on which, which artifact formats exist and who
owns each) contributes four rows, KA-781 to KA-784. They are structural rather than numerical: an identity that
must be equal, byte differences that must be zero, a dependency count that must be zero, planted violations
that must be caught. Each is the stated truth of one workspace gate. This bead registers the four rows as data,
binds each to the test that realizes it, and checks that every stated negative control is registered.
Tier: Solid.

## Scope

In: the four rows in the conformance crate's known-answer registry; their bindings to realizing tests and
negative controls; the registry self-check for this chapter. Out: the tests themselves (named per row); known
answers of the suite-consumption chapter that the workspace checks also realize (the stack receipt's
determinism, KA-767, is registered by that chapter and realized by the stack check).

## Technical approach

| KA | Where stated | The case | The truth | Tolerance | Tier · site | Negative control | Realized by |
|---|---|---|---|---|---|---|---|
| KA-781 | The format registry | Canonical identity of the objects `{"b":1,"a":2}` and `{"a":2,"b":1}` | One identity: both are read to the same value, the canonical bytes are `{"a":2,"b":1}`, and the identity is the BLAKE3 hash of those bytes under the schema name as domain string | exact | G0 · host | A reader built with key-order canonicalization disabled must fail (the control of GATE-WS-02) | N34-17 `view_and_canonical_agree`; N34-18 `key_order_identity`; N34-21 corpus pair 1; N34-22 respelling step; N34-23 step 4 |
| KA-782 | The format registry (GATE-WS-02) | For each of the thirteen formats: 10,000 seeded round trips canonical → value → canonical; where a view exists, 10,000 round trips view → canonical → view; 10,000 single-byte mutations | Zero byte differences in both directions; every mutation either fails to parse or changes the identity; each reader survives 10⁷ fuzz executions with zero panics | exact | G3 · host (the fuzz clause runs in the chaos tier) | Key-order canonicalization disabled | N34-22 (harness); BD-141 (seven formats at M1); BD-410 (all thirteen at M5); N34-23 (user side) |
| KA-783 | The independent auditor (GATE-WS-03) | The offline certificate checker `frob-cert` and its binary `frob-verify` | `cargo tree -p frob-cert` prints one line; the copied directory builds on its stable compiler with no network access; the standalone binary and the in-daemon verb return identical verdicts on the whole conformance corpus; a system-call trace of an audit shows no network and no process creation | exact | G0 · auditor | A checker that reuses `frob-core`'s hashing reproduces a canonicalization defect and passes a bundle whose statement differs | N34-25 (dependency count, every commit); BD-026 and N25b-05 (offline build, system-call trace); N12-34 (corpus-wide verdict comparison) |
| KA-784 | The layer rules (GATE-WS-01) | Workspace checks | A deliberate upward edge, a deliberate intra-layer edge against rank, a second asupersync package, an unregistered capsule, an edited generated file and a contract missing a section are each caught | exact | G0 · host | The planted violations are the controls | BD-003, N34-03, N34-07, BD-004, N34-08 (one owner each); N34-14 (the plant-by-check matrix) |

Notes that bind the rows:

1. **KA-784 and the seventh plant.** The row lists six planted violations. The gate it belongs to lists seven:
   the six above and a stale stack lock, with the further condition that each plant is caught by the check that
   owns it and by no other, and that the clean tree has no failure. The gate register governs thresholds, so
   the realizing test plants seven, and the registry row carries a `gate_extends` note naming the seventh plant
   and its owner (`check-stack`, N34-13). The row's text is not renumbered or rewritten.
2. **KA-781's value.** The equal identity is a property, not a digest written here. The pinned digest for the
   canonical bytes `{"a":2,"b":1}` under each schema name is captured once from an independent BLAKE3 tool in
   derive-key mode with the schema name as context, stored under `tests/fixtures/oracle/` with its capture
   record, and never regenerated from this project's hashing.
3. **Tiers.** G0 is the tier of laws and self-tests (layer and dependency checks belong to it); G3 is the
   metamorphic tier (relations that hold without knowing the answer: respelling preserves identity, any
   accepted byte change alters it); the fuzz clause of KA-782 runs in G4, the chaos tier. The execution site
   `auditor` means the test runs the standalone `frob-verify` binary.

**Registry record** (one per row, DESIGN, in the conformance crate's known-answer registry): `id`, `chapter`,
`case`, `truth`, `tolerance`, `tier`, `site`, `gates` (KA-781 and KA-782: GATE-WS-02; KA-783: GATE-WS-03;
KA-784: GATE-WS-01), `controls` (identifiers in the negative-control registry), `realized_by` (test paths), and
for KA-784 `gate_extends`. The registry's own check fails when a row has no realizing test that exists, when a
stated negative control has no registered control, or when a realizing test does not mention its row's
identifier.

## Acceptance criteria

- [ ] The four rows are in the registry with every column above; identifiers KA-781 to KA-784 are neither
      reused nor renumbered.
- [ ] Each row's `realized_by` paths exist once their beads land; before that the row is listed as unbound and
      its gate cannot record a pass.
- [ ] Each stated negative control has a record in the negative-control registry attached to the row's gate:
      key-order canonicalization disabled (GATE-WS-02); the checker that reuses the proposer's hashing
      (GATE-WS-03); the seven planted violations (GATE-WS-01).
- [ ] KA-784's record carries the note about the seventh plant, and the plant matrix test asserts seven.
- [ ] The pinned digest fixture for KA-781 has a capture record naming an external tool and its version.
- [ ] The registry self-check passes on the clean tree and fails when a control is removed.

## Test plan

### Unit tests

`cargo test -p frob-conformance registry_ka_workspace`: `four_rows_present`; `ids_are_781_to_784`;
`every_row_names_a_gate`; `every_negative_control_is_registered`; `realizing_tests_mention_their_ka`;
`ka784_gate_extension_names_stale_stack_lock`; `unbound_row_blocks_gate_pass` (a `synthetic` registry copy with
one binding removed makes the gate's pass invalid).

### End-to-end script and logging

No script of its own. The rows are realized by `scripts/e2e/workspace_checks.sh` (KA-784),
`scripts/e2e/canonical_json.sh` (KA-781), `scripts/e2e/format_registry.sh` (KA-781, KA-782) and
`scripts/e2e/auditor_independence.sh` (KA-783); each logs the known-answer identifier in its `assert` lines with
the measured value against the stated truth, in the structured format of STD-E2E.

### Known answers and negative controls

The four rows above are the content. Negative controls of the registry itself: a row with an empty `controls`
list; a realizing test renamed without updating the row. Determinism: the registry check is a pure function of
the registry files and the tree.

## Considerations

- These rows are thresholds of gates and repeat them; where a row and the gate register differ (as KA-784 does
  on the number of plants), the register governs and the difference is recorded, not smoothed over.
- A control that has never failed its gate in any recorded run is itself suspect and is reviewed at each
  milestone exit; for KA-783 that review looks at the checker-reuse control in particular, because it needs a
  deliberately defective build to exercise.
- The registry is data and the appendix is its printout; adding a workspace known answer later appends a row
  with the next free number.

## References

Appendix M.33 (KA-781 to KA-784), §34.3, §34.9, §34.11, §34.13, §38.2, §38.6; GATE-WS-01, GATE-WS-02,
GATE-WS-03.
