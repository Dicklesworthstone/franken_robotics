---
key: "N34-KA"
title: "Workspace known-answer registry: KA-781 to KA-784"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "workspace", "frob-conformance", "solid", "registry", "test"]
parent: "WP-01"
deps: ["BD-025", "N38a-12", "N38b-05", "STD-E2E", "STD-TEST"]
needs: []
related: ["N34-REF", "N38a-13"]
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
| KA-781 | The format registry | Canonical identity of the objects `{"b":1,"a":2}` and `{"a":2,"b":1}` | One identity: both are read to the same value, the canonical bytes are `{"a":2,"b":1}`, and the identity is the BLAKE3 hash of those bytes under the schema name as domain string | exact | G0 · host | A reader built with key-order canonicalization disabled must fail (the control of GATE-WS-02) | N34-18 `key_order_identity` and `view_and_canonical_agree`; N34-21 corpus pair 1; N34-22 respelling step; N34-23 step 4 |
| KA-782 | The format registry (GATE-WS-02) | For each of the thirteen formats: 10,000 seeded round trips canonical → value → canonical; where a view exists, 10,000 round trips view → canonical → view; 10,000 single-byte mutations | Zero byte differences in both directions; every mutation either fails to parse or changes the identity; each reader survives 10⁷ fuzz executions with zero panics | exact | G3 · host (the fuzz clause runs in the chaos tier) | Key-order canonicalization disabled | N34-22 (harness); BD-141 (seven formats at M1); N34p1-01 (`.fscene`, M2), N34p1-02 (`.fclear`, `.fmcw`, M3), N34p1-03 (`.fskill`, `.fenergy`, M4), each before its format's first release; BD-410 (`.fbuild` and the thirteen-of-thirteen record at M5); N34-23 (user side) |
| KA-783 | The independent auditor (GATE-WS-03) | The offline certificate checker `frob-cert` and its binary `frob-verify` | `cargo tree -p frob-cert` prints one line; the copied directory builds on its stable compiler with no network access; the standalone binary and the in-daemon verb return identical verdicts on the whole conformance corpus; a system-call trace of an audit shows no network and no process creation | exact | G0 · auditor | A checker that reuses `frob-core`'s hashing reproduces a canonicalization defect and passes a bundle whose statement differs | N34-25 (dependency count, every commit); BD-026 and N25b-05 (offline build, system-call trace); N12-34 (corpus-wide verdict comparison) |
| KA-784 | The layer rules (GATE-WS-01) | Workspace checks | A deliberate upward edge, a deliberate intra-layer edge against rank, a second asupersync package, an unregistered capsule, an edited generated file and a contract missing a section are each caught | exact | G0 · host | The planted violations are the controls | BD-003, N34-03, N34-07, BD-004, N34-08 (one owner each), and N33a-05 for the gate's seventh plant; N34-14 (the plant-by-check matrix) |

Notes that bind the rows:

1. **KA-784 and the seventh plant.** The row lists six planted violations. The gate it belongs to lists seven:
   the six above and a stale stack lock, with the further condition that each plant is caught by the check that
   owns it and by no other, and that the clean tree has no failure. The gate register governs thresholds, so
   the realizing test plants seven, the row's `nc` list names seven controls, and its derivation file carries a
   "gate extends" note naming the seventh plant and its owner (`check-stack`, N33a-05, finding `stack.pin`). The
   row's text is not renumbered or rewritten.
2. **KA-781's value.** The equal identity is a property, not a digest written here. The pinned digest for the
   canonical bytes `{"a":2,"b":1}` under each schema name is captured once from an independent BLAKE3 tool in
   derive-key mode with the schema name as context, stored under `tests/fixtures/oracle/` with its capture
   record, and never regenerated from this project's hashing.
3. **Tiers.** G0 is the tier of laws and self-tests (layer and dependency checks belong to it); G3 is the
   metamorphic tier (relations that hold without knowing the answer: respelling preserves identity, any
   accepted byte change alters it); the fuzz clause of KA-782 runs in G4, the chaos tier. The execution site
   `auditor` means the test runs the standalone `frob-verify` binary.

**Registry record.** One `[[ka]]` per row, appended to the suite's one known-answer file
`crates/frob-conformance/registry/known_answers.toml` in the row schema of N38a-12 (no chapter file of its own;
a second file fails the loader with `ka-second-file`):

| Field | KA-781 | KA-782 | KA-783 | KA-784 |
|---|---|---|---|---|
| `section` (owner) | §34.9 (GATE-WS-02) | §34.9 (GATE-WS-02) | §34.11 (GATE-WS-03) | §34.3 (GATE-WS-01) |
| `fixture` | `key-order-pair` | `format-roundtrip` | `auditor-independence` | `workspace-plants` |
| `truth` kind | equal digests | zero differences; zero panics | zero dependencies; equal verdicts | seven of seven caught |
| `tolerance` | exact | exact | exact | exact |
| `tier`; `sites` | G0; host | G3; host (fuzz clause G4) | G0; auditor | G0; host |
| `due` | M0 | M1 | M1 | M0 |
| `label` | DESIGN | DESIGN | DESIGN | DESIGN |

`control.text` holds the text of the column above. For KA-784 the cell is followed by the seven plants it refers
to, named one per clause (the six of the case and the stale stack lock of note 1). `nc` is filled by the seeding
tool of N38b-05, which issues every control record; this bead writes none and types no `NC-` number. Step 3 of
that tool resolves each named plant separately. Where N34-CX already holds a control with the same plant and
target, it appends that one, and otherwise it issues a control with `seed = "KA-784"`, so KA-784's `nc` has
seven entries after seeding. N34-14's plant matrix runs those plants; it is not a source of records.
`derivation` names `docs/derivations/<fixture>.md`, which states the case,
the truth, why the threshold is exactly it, and (KA-781) the oracle capture of note 2, (KA-784) the seventh plant
of note 1; the row's own `case` text is not rewritten. DESIGN: KA-782 is one row due at M1 whose bound function
covers the formats whose registry rows exist at the run's milestone and logs that count as a measured value;
thirteen of thirteen is the completion of GATE-WS-02 at M5 (BD-410), so a format that lands later never makes
the row `unbound` earlier.

**Binding.** The "Realized by" column is provenance; the binding the runner reads is a table entry
`ka:<id>@<site>` (N38a-13) supplied by the bead that owns the test, plus a developer-level unit test named
`ka_781_…` to `ka_784_…` in the owning crate. A row that is due at the selected milestone and has no bound
function is reported `unbound`, which fails its gate; a realizing entry of a bead due at a later milestone is
reported `not yet due` and never blocks an earlier record. Eligibility follows N38a-12 (derivation on file,
tolerance, registered control).

## Acceptance criteria

- [ ] The four rows are in `crates/frob-conformance/registry/known_answers.toml` with every field of the record
      table and pass N38a-12's loader; identifiers KA-781 to KA-784 are neither reused nor renumbered. After the
      seeding tool of N38b-05 is run, all four are `eligible`; before it, a row without `nc` is reported
      `obligation: control`, never `pass`.
- [ ] At its due milestone each row has a bound runner entry `ka:<id>@<site>` (KA-781 and KA-784 at M0,
      KA-782 and KA-783 at M1); a removed binding of a due row is reported `unbound` (gate failure), while the
      entries of beads due later are reported `not yet due` and block nothing.
- [ ] Each stated negative control is the row's `control.text`, and the seeded record is attached to the row's
      gate: key-order canonicalization disabled (GATE-WS-02); the checker that reuses the proposer's hashing
      (GATE-WS-03); the seven planted violations, named one per clause (GATE-WS-01). This bead writes no
      control record itself.
- [ ] KA-784's derivation file carries the note about the seventh plant, and after seeding its `nc` list has
      seven entries, one per named plant (N34-14's plant matrix test asserts that seven plants are caught).
- [ ] The pinned digest fixture for KA-781 has a capture record naming an external tool and its version.
- [ ] The registry self-check passes on the clean tree and fails when a control is removed.

## Test plan

### Unit tests

`cargo test -p frob-conformance --test ka_workspace` (`crates/frob-conformance/tests/ka_workspace.rs`): `four_rows_present`; `ids_are_781_to_784`;
`every_row_names_a_gate`; `every_negative_control_is_registered`; `realizing_tests_mention_their_ka`;
`ka784_gate_extension_names_stale_stack_lock` (the derivation file names the plant and its owner; the control
text names seven plants and, after seeding, `nc` has seven entries);
`unbound_row_blocks_gate_pass` (a `synthetic` registry copy with one binding of a due row removed makes the
gate's pass invalid); `later_entry_is_not_yet_due` (at M1 the BD-410 entry of KA-782 is reported `not yet due`
and GATE-WS-02's M1 record is not blocked by it); `rows_load_in_the_one_file` (the four rows pass N38a-12's
loader, all four as `eligible` after seeding and as `obligation: control` on a scratch copy with `nc` emptied).

### End-to-end script and logging

`scripts/e2e/workspace_ka_registry.sh` checks the registry only.
- **Header and log.** Its header is `bead=N34-KA milestone=M0 labels=workspace,registry gates=`. It sources the
  STD-E2E helper and writes NDJSON to `target/e2e-logs/workspace_ka_registry/<utc>.ndjson` with a transcript.
- **Steps.** It runs `cargo test -p frob-conformance --test registry_check` and `--test ka_workspace`.
- **Metric lines.** It logs:
  - the four rows with their computed eligibility, due and owner gate;
  - the `control.nc` count per row after seeding (seven for KA-784);
  - the capture record of KA-781's pinned digest;
  - the digest of `known_answers.toml`.
- **Planted scratch defects.** On scratch copies it plants KA-781's control text deleted and KA-784's
  derivation file without its seventh-plant note, and requires each to be caught.
- **Line fields.** Every line carries `ts`, `script`, `bead`, `step`, `event`, the exact command, the exit code
  and the wall time. The `summary` gives the assertions passed and failed and `gates: []`.

The rows themselves are realized by `scripts/e2e/workspace_checks.sh` (KA-784),
`scripts/e2e/canonical_json.sh` (KA-781), `scripts/e2e/format_registry.sh` (KA-781, KA-782) and
`scripts/e2e/auditor_independence.sh` (KA-783); each logs the known-answer identifier in its `assert` lines with
the measured value against the stated truth, in the structured format of STD-E2E.

### Known answers and negative controls

The four rows above are the content. Negative controls of the registry itself: a row with an empty
`control.text`; a realizing test renamed without updating the row's derivation front matter. Determinism: the registry check is a pure function of
the registry files and the tree.

## Considerations

- These rows are thresholds of gates and repeat them; where a row and the gate register differ (as KA-784 does
  on the number of plants), the register governs and the difference is recorded, not smoothed over.
- A control that has never failed its gate in any recorded run is itself suspect and is reviewed at each
  milestone exit; for KA-783 that review looks at the checker-reuse control in particular, because it needs a
  deliberately defective build to exercise.
- The registry is data and the appendix is its printout; adding a workspace known answer later appends a row
  whose number comes from a block this bead requests in `allocations.toml` (N38a-40); nobody types the next
  free number, and the row is cited by a provisional tag until the block is issued.

## References

Appendix M.33 (KA-781 to KA-784), §34.3, §34.9, §34.11, §34.13, §38.2, §38.6; GATE-WS-01, GATE-WS-02,
GATE-WS-03.
