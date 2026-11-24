---
key: "N32-CX"
title: "Tether counterexample registry: CX-345 to CX-351 with their planted mutants, and the unnumbered cases"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "tether", "frob-conformance", "frob-verbs", "solid", "registry", "test"]
parent: "WP-06"
deps: ["BD-025", "N38a-40", "N38b-05", "STD-E2E", "STD-TEST"]
needs: []
related: ["N32-KA", "N32-REF", "N32a-07", "N32a-08", "N32a-09", "N32a-10", "N32a-20", "N32a-24", "N32b-02", "N32b-03", "N32b-23", "N32b-24", "N32b-25", "N32b-41", "N32b-43", "BD-296", "N38b-04"]
gates: []
plan_refs: ["§32.20", "§32.7", "§32.8.1", "§32.8.2", "§32.10", "§32.11.2", "§32.24", "§32.13", "Appendix N.29", "§0.7", "§38"]
---
## Background

A counterexample, in this project, is the smallest hand-checkable case that breaks a plausible shortcut. The rule is
that a design choice ships with such a case, that the case enters the counterexample catalogue, and that it seeds
the negative-control registry: for each one a deliberately wrong variant of the mechanism (a mutant) is built in a
test build, and the gate that protects the rule must fail on it. A gate that nothing can fail validates nothing.

Tether, the surface subsystem (verb registry, envelope, daemon `frobd`, MCP server, dashboard, viewer), has seven
catalogued counterexamples. They justify the atomic idempotency binding, cursor pagination pinned to a committed
frontier with a total-order key, the two integer schema types, the fixed logical chunk partition with the episode
in its key, and preempt-resume scheduling. This bead is the chapter's registry: each case with its numbers, the
rule it protects, the mutant to plant, the gate that must catch it and the bead that owns the mechanism. It also
records three cases the chapter states or implies without a catalogue number.

## Scope

In: the seven rows and the three unnumbered cases as entries of the catalogue
`crates/frob-conformance/registry/counterexamples.toml`, each with the case as data, the plant identifier, the
expected failing assertion, the gates and the bead that builds the mechanism and its plant; the block request that
gives the unnumbered cases numbers; the registry-level checks and the pure-arithmetic checks of the cases.
Out: the catalogue's record schema and loader, and the seeding tool that writes
`crates/frob-conformance/registry/controls.toml`, allocates `NC-` numbers and proves completeness by the join
(N38b-05): this bead writes no row of `controls.toml`, and neither does any mechanism bead; a control appears there
when the seeding is re-run over these entries, which is append-only and idempotent. Building the mechanisms and
their plants behind test-only flags and catching each plant in the mechanism's own tests (the beads named per row:
N32a-07, N32a-08, N32a-09, N32a-10 at M1; BD-210, BD-211, N24a-41 at M2; BD-296, N32a-20, N32b-02, N32b-03 at M3);
running the same plants through the real `frob` and `frobd` binaries at the daemon-level gates (N32b-24 at M2 for
CX-345 to CX-350 and the packet case; N32a-20 and BD-296 for CX-351 at M3, because the dispatcher does not exist
earlier); the control runner (N38b-04); the number blocks (N38a-40); the known answers of the chapter (the registry
bead `N32-KA`). This registry depends on no mechanism (registries come first), so nothing in its own acceptance
needs a mechanism to exist.

## Technical approach

**The seven catalogued cases.**

| CX | The tempting shortcut | The smallest case that breaks it | The rule it protects | The mutant to plant; what must fail |
|---|---|---|---|---|
| CX-345 | Check idempotency by looking up the key, then inserting, in two statements | Requests A and B carry the same key and payload. A looks up and misses; B looks up and misses; A inserts; B inserts. Two jobs run and the budget is charged twice | One transaction with a unique index on (principal, verb, key) binds key, canonical request digest, budget reservation and job | A build flag replaces the immediate transaction by a lookup statement and a separate insert, with no unique index. GATE-TETHER-05 must then observe two jobs or two charges in its 1,000 races, and the exhaustive interleaving check of proof obligation PO-TETHER-01 must find the A-miss, B-miss, A-insert, B-insert schedule |
| CX-346 | Paginate by offset | Rows at t = 1, 2, 3, 4, page size 2. Page one returns {1, 2}. A chunk from a slower world then commits a row at t = 1.5. Offset 2 now returns {2, 3}: row 2 is repeated | Pagination pinned to the committed frontier of the first page; later pages read only rows at or below that frontier and strictly after the last key; refusals `FROB-REF-806` (cursor does not match the query) and `-807` (cursor's frontier no longer retained) | A paginator that keeps (offset, limit). GATE-TETHER-06 must report a repeated row on the planted append; with the correct paginator page two is {3, 4} |
| CX-347 | Use a cursor on time alone | Two worlds each have a sample at t = 2 and the page ends on the first; "after t = 2" skips the second | The total-order key (instant, world address, channel, sample index), so rows that share a timestamp still have one order | A cursor that stores only the instant. GATE-TETHER-06 must report a skipped row on the equal-timestamp fixture |
| CX-348 | Send 64-bit integers as JSON numbers | 2⁵³ + 1 = 9,007,199,254,740,993 decodes to 9,007,199,254,740,992 in a client that reads numbers as IEEE doubles | Two schema types replace "integer": `int53` (a JSON number refused outside ±(2⁵³ − 1), for counts) and `u64str` / `i64str` (decimal strings, for world addresses, seeds, stream counters, tick indices, nanosecond instants); `FROB-REF-808` | A schema that declares a tick index as a bare 64-bit integer. The schema lint must fail, and the golden envelope set passed through a double-decoding client must show a changed value (GATE-TETHER-07) |
| CX-349 | Key telemetry chunks by tick range | A retry splits a shard, and ranges [0, 100) and [50, 150) both cover ticks 50 to 99; with range keys both commit | A fixed logical chunk partition per (world, episode): chunk k covers ticks [k·C, (k + 1)·C); a second submission of a committed extent is a duplicate (original receipt) or a conflict (`FROB-REF-810`) | A store keyed by (world, episode, first tick, last tick). GATE-TETHER-08 must observe overlapping committed extents on the planted shard split |
| CX-350 | Key telemetry by (world, tick) | A reset reuses tick zero, and the second episode overwrites the first | The episode index is part of the chunk key | A key without the episode. GATE-TETHER-08 must observe the first episode's chunk replaced after a reset |
| CX-351 | Priority without preemption protects interactive jobs | One device, 90% of jobs 10 s and 10% of jobs 30 min: E[S] = 0.9·10 + 0.1·1,800 = 189 s, E[S²] = 0.9·100 + 0.1·3,240,000 = 324,090 s². M/G/1 at utilization 0.7: a short job waits 2,001 s first-come-first-served, 621 s under non-preemptive priority, and 0.17 s plus one quantum under preempt-resume. The wait under priority is the remainder of the long job already on the device | Slot dispatch with preemption at kernel-batch boundaries (the dispatcher's decision core, DC-09) | A dispatcher with the preemption arm removed. The discrete-event check of KA-758 must then return the priority-only column (621 s at 0.7), and GATE-TETHER-10's limit of 6 s on the 95th-percentile interactive wait must fail |

**Who builds each mechanism and its plant** (the plant is built in the mechanism's crate behind a test-only Cargo
feature reachable only under `gauntlet-mutations`, BD-025, by that bead, and named by the entry's `plant`
identifier): CX-345 `idem-two-statement` by N32a-09 (the admission transaction; BD-211 runs the gate on the daemon);
CX-346 `cursor-offset` and CX-347 `cursor-time-only` by N32a-10 (the cursor; BD-211 on the daemon); CX-348
`schema-bare-int64` by N32a-07 (number types and the integer lint; BD-211 on the daemon); CX-349
`chunk-key-tick-range` and CX-350 `chunk-key-no-episode` by N32a-08 (the job ledger and chunk keys; BD-210 and
N32a-18 on the daemon under seeded faults); CX-351 `dispatch-no-preempt` by N32a-20 (the queueing model and event
simulator, priority-only policy) and BD-296 (the real dispatcher with preemption disabled by the same plant);
`packet-binary-budget` by N32b-24 against the packet writer N24a-41 (M2, which lists the control on
GATE-TETHER-16), run against the same seeded control by N32b-41 (the 16-fork over-cap request, M3) and N32b-43 (the
GATE-TETHER-16 gate run, M3); `derived-token-copy` by N32b-02; `leaderboard-arrival-ties` by N32b-03. Each plant's
`id` is `<planting bead>:<identifier>` (N38b-05), the planting bead being the first bead named here:
`N32a-09:idem-two-statement`, `N32a-10:cursor-offset`, `N32a-10:cursor-time-only`, `N32a-07:schema-bare-int64`,
`N32a-08:chunk-key-tick-range`, `N32a-08:chunk-key-no-episode`, `N32a-20:dispatch-no-preempt`,
`N32b-24:packet-binary-budget`, `N32b-02:derived-token-copy` and `N32b-03:leaderboard-arrival-ties`; the name after
the colon is the Cargo feature.

**One control per entry (DESIGN, following N38b-05).** The seeding tool of N38b-05 derives exactly one control from
each entry, with `seed` = the CX number, `kind = "mutation"`, the entry's gates, and the target, defect and expected
observation of the row; it allocates the `NC-` number. No mechanism bead writes or allocates a control row. The bead
that first builds a plant and catches it in its own test (N32a-09 for CX-345, N32a-10 for CX-346 and CX-347,
N32a-07 for CX-348, N32a-08 for CX-349 and CX-350, all at M1; N32b-24 for the packet case at M2; N32a-20 for CX-351
at M3; N32b-02 and N32b-03 for the two DESIGN additions at M3) binds its test to that control by name (prefix
`nc_<nnnn>_`, STD-TEST); a later bead that runs the same plant at another level (N32b-24 through the binaries at M2;
BD-211 and BD-210 on the daemon; BD-296 for CX-351) binds its own test to the same control and adds no row.
GATE-TETHER-07 is an M1 gate, so CX-348's control must be caught at M1 by N32a-07's test (a gate with no caught
control prints `not-eligible`, STD-TEST); GATE-TETHER-05, -06 and -08 are M2 gates, caught by N32b-24 and BD-211;
GATE-TETHER-10 and -16 are M3 gates.

**How the M/G/1 numbers are obtained** (so the control can be re-derived). With arrival rate λ and utilization
ρ = λ·E[S]: W₀ = λ·E[S²]/2. First-come-first-served wait: W₀/(1 − ρ). Non-preemptive priority for the short class:
W₀/(1 − ρ₁) with ρ₁ = λ·0.9·10. Preempt-resume, short class: λ₁·E[S₁²]/(2·(1 − ρ₁)) = λ·0.9·100/(2·(1 − ρ₁)), plus at
most one quantum (5 s, TARGET). At ρ = 0.7: λ = 0.7/189 = 0.0037037 per second, W₀ = 600.2 s, ρ₁ = 0.03333; the three
waits are 2,000.6 s, 620.9 s and 0.172 s. The slowdown of long jobs under preempt-resume is ρ₁/(1 − ρ₁): 2.44%,
3.45%, 3.96%, 4.48% at ρ = 0.5, 0.7, 0.8, 0.9 (printed 2.4%, 3.4%, 4.0%, 4.5%).

**Cases the chapter states without a catalogue number.** They are entered with provisional tags and receive
numbers through the allocation ledger of N38a-40 (`crates/frob-conformance/registry/allocations.toml`): this bead
commits one request `[[block]] registry = "cx", count = 3, owner = "N32-CX", section = "§32.24, §32.13"`; the
allocator fills `first` and `last`; the tags are then mapped to numbers in table order in one commit, and a tag left
in a registry file, test name or derivation file at this bead's milestone fails the check
`provisional-tag-past-due`. Numbers are append-only; a case withdrawn later is marked `void` in the block, never
reissued.

| Tag | The tempting shortcut | The case | The rule | Gate | Plant; planting bead |
|---|---|---|---|---|---|
| `tether-packet-payload-budget` | Budget a single-file replay by its binary payload size | 6,200,000 bytes of timeline rows are under an 8 MiB (8,388,608-byte) cap as binary, 8,266,667 bytes once Base64-encoded (× 4/3) and 8,866,667 bytes with the 600,000-byte page: over | The packet writer computes the encoded size before it writes anything; over the cap it refuses with the size table and three reductions; nothing is truncated | GATE-TETHER-16 | `packet-binary-budget` (a writer that budgets by binary payload); N32b-24 against N24a-41, with N32b-41's packets |
| `tether-derived-budget-copy` (DESIGN addition) | Give a derived token a copy of the budget it asks for, without debiting the parent | A parent with 600 GPU-seconds derives three sub-agent tokens of 600 each: 2,400 GPU-seconds are spendable under a 600-second grant | The budget of a derived token is debited from its parent in the transaction that creates it | none at plan time (finding) | `derived-token-copy`; N32b-02 |
| `tether-leaderboard-arrival-order` (DESIGN addition) | Break leaderboard ties by arrival time | Entries P and Q have equal metric. Daemon X saw P then Q; daemon Y saw Q then P. After merging, the two leaderboards differ | Ties break by a total order: metric, then content address | none at plan time (finding) | `leaderboard-arrival-ties`; N32b-03 |

The two DESIGN additions protect rules whose beads (N32b-02, N32b-03) carry no gate in the register. Their entries
are kept (the shortcut and its case are real, and a rule without a gate is exactly what the catalogue exists to
expose): the seeding tool writes each to `registry/seed_findings.toml` with the disposition required at the M3
exit, `gate-added` (naming the gate) or `withdrawn` (with the reason), as the plan's rule for an entry whose rule has
no gate prescribes. They are the only entries of this bead allowed in the findings file.

**Registry record.** One entry per case in `crates/frob-conformance/registry/counterexamples.toml` (the name
`registries/counterexamples.toml` in some documents is a logical alias the loader resolves to this file; a second
physical copy is a lint failure), in the record schema of N38b-05 and with no other field: `id` (the CX number, or
the tag until its block is issued), `shortcut`, `case` (the numbers above as data), `numeric = true`,
`plan_case = "numbers"`, `case_supplied_by` (empty), `rule`, `rule_ids` (the gates, PO-TETHER-01 for CX-345, DC-09
for CX-351 and the refusal codes of the rule column), `defined_in`, `same_as` (empty), `label` (DESIGN),
`control_kind = "mutation"`, `plant` (the plant `id` above), `due` and, for an appended entry, `slug`. The target,
defect and expected observation of each row's mutant column are the texts the seeding tool copies into the
control's `target`, `defect` and `expect`. DESIGN, correcting an earlier draft of this bead that used private
fields (`chapter`, `mutant`, `controls`, `gates`, `owner_bead`, `control_by`, and a private `due`) and let the
mechanism beads allocate control rows: the catalogue's loader admits only its schema's fields, and only the seeding
tool writes `controls.toml`, so that a control identifier is allocated once. Each entry's `due` is the milestone
from which a caught control is required: M1 for CX-345 to CX-350, whose mechanism beads catch their plants at M1;
M2 for the packet case, whose writer N24a-41 lands at M2; M3 for CX-351 and for the two DESIGN additions, whose
beads N32b-02 and N32b-03 are M3. The seeding tool copies it to the control's optional `due` field (BD-025). The
planting bead of each entry is kept in one compiled-in table keyed by CX number or tag in this bead's test file.
Plants live behind a test-only cargo feature of the mechanism's crate, reachable only under `gauntlet-mutations`; a
release build never contains one (checked by `cargo tree -e features` on each binary showing no plant feature, the
same check STD-TEST uses for `test-support`).

**Registry checks (this bead's executable part).** `tether_cx_registry` loads the file through the STD-TEST loader
and asserts: the seven numbers once each, contiguous; every entry has non-empty `shortcut`, `case`, `rule`,
`plant`, at least one gate among `rule_ids` (the two DESIGN additions excepted, which must instead appear in the
findings file with a disposition requested); every planting bead of the compiled-in table is live in the bead
index; every entry whose `due` milestone has been reached has a control with status `caught` (before that it
reports `not yet due`, never `pass`); the case data reproduce the stated arithmetic. Case data are also written to
`fixtures/tether/counterexamples.json` (header `synthetic`) for the end-to-end scripts.

## Acceptance criteria

- [ ] Seven catalogue entries CX-345 to CX-351 exist in `crates/frob-conformance/registry/counterexamples.toml`
      with the case numbers above as machine-readable data, each with its `plant` and gate(s) in the schema of
      N38b-05 and no other field, each with its `due`; the planting bead of each is in the test file's table; three
      provisional entries carry tags and a committed block request for three numbers; after issue they carry
      the block's numbers in table order with their tags in `slug`, and no tag remains as an identifier at M1.
- [ ] After the seeding tool of N38b-05 is re-run (`cargo test -p frob-conformance --test seed_controls --
      --ignored seed`), each of the ten entries has exactly one control in `controls.toml` whose `seed` names it;
      the second run changes no byte; the join passes; only the two DESIGN additions appear in the findings file.
      No record of `controls.toml` is written by this bead or by a mechanism bead.
- [ ] `cargo test -p frob-conformance --test tether_counterexamples` passes: the registry checks above, and the
      pure-arithmetic cases recomputed from first principles (not copied): the double rounding of CX-348, the
      sixteen M/G/1 cells and four slowdowns of CX-351 from the two service times and the mix, the packet-budget
      chain, the derived-budget sum.
- [ ] The join required by the conformance registry (no counterexample without a control and a gate) holds for
      every entry that is due; an entry past due with no caught control fails the check (planted).
- [ ] The executed controls (plant fails, real mechanism passes) are recorded by the planting beads named above
      (N32a-07 to N32a-10 at M1, with N32b-24's binary-level runs added at M2; N32a-20 and BD-296 for CX-351); this
      registry reads their `caught` status from the evidence stream and does not re-run them; a counterexample with
      two control rows fails the check (`one_control_row_per_mutant`).
- [ ] `fixtures/tether/counterexamples.json` exists, is generated from the registry, and differs from it in no
      value (checked).

## Test plan

### Unit tests
`crates/frob-conformance/tests/tether_counterexamples.rs` (names follow the STD-TEST prefixes; real registry
files, no mocks):
- `tether_cx_registry_complete`: CX-345 to CX-351 present once each with every field; three tagged entries and one
  block request of count 3 owned by N32-CX.
- `cx_348_double_rounding`: `9007199254740993_f64 as u64 == 9007199254740992`; 2⁵³ − 1 = 9,007,199,254,740,991
  survives the round trip; the regular expression `^(0|[1-9][0-9]{0,19})$` accepts `"9007199254740993"` and refuses
  `"09"` and `"-0"` for `u64str`.
- `cx_351_mg1_table`: closed-form values from λ = ρ/189, W₀ = λ·324,090/2, ρ₁ = 9λ: 857, 2,001, 3,430, 7,716 s
  (first-come); 439, 621, 713, 806 s (priority only); 0.12, 0.17, 0.20, 0.22 s (preempt-resume) at ρ = 0.5, 0.7,
  0.8, 0.9, to the printed digits; slowdowns 2.4%, 3.4%, 4.0%, 4.5%; and the unrounded 2,000.6, 620.9, 0.172 s
  at ρ = 0.7.
- `cx_346_case_data`, `cx_347_case_data`, `cx_349_case_data`, `cx_350_case_data`: the case data reproduce their
  own statement (rows {1, 2, 3, 4} with page size 2 and an append at 1.5; two samples at t = 2 in worlds w₁ and w₂;
  extents [0, 100) and [50, 150) with C = 50 map to chunks {0, 1} and {1, 2}, sharing chunk 1; two episodes each
  with tick 0).
- `tether_packet_payload_budget`: 6,200,000 × 4/3 = 8,266,666.67, printed 8,266,667; + 600,000 = 8,866,667 >
  8,388,608.
- `tether_derived_budget_copy_sum`: three children of 600 each without debiting = 2,400 spendable against 600.
- `tether_cx_join_past_due_fails`: a scratch registry with an entry past due and no caught control fails the join
  (negative control of the registry).
- `one_control_row_per_mutant`: a scratch `controls.toml` with two rows seeded by CX-345 for the same plant fails;
  one row bound by two tests (the M1 unit test and the M2 binary run, both named `nc_<nnnn>_…`) passes.
- `due_matches_planting_bead`: each entry's `due` equals the milestone of the first
  planting bead, read from the bead index (CX-345 to CX-350: N32a-07 to N32a-10, M1; packet case: N32b-24, M2;
  CX-351: N32a-20, M3; the two additions: N32b-02 and N32b-03, M3); a planted `due = "M1"` on the packet case fails.
- `schema_fields_only`: a scratch entry carrying a private field (`owner_bead`) is rejected by the catalogue loader.
- `design_additions_are_findings`: after seeding a scratch copy, the two DESIGN additions are in
  `seed_findings.toml` with a disposition requested and no other entry of this chapter is.
- `tether_cx_fixture_matches_registry`: the generated JSON equals the registry values.

The executed controls are named here so that their owners use these names: `cx_345_two_statement_double_charges`,
`cx_346_offset_repeats_row`, `cx_347_time_cursor_skips`, `cx_348_double_decoder_envelope`,
`cx_349_overlapping_ranges`, `cx_350_reset_overwrites` (N32b-24 at M2); `cx_<nnn>_packet_payload_budget`, with the
number issued to `tether-packet-payload-budget` (N32b-24 at M2; the same control is run by N32b-41 and N32b-43 at
M3); `cx_351_priority_only_fails_gate` (N32a-20 and BD-296 at M3); the two DESIGN additions in N32b-02 and N32b-03. The M1 unit-level tests of the
mechanism beads (N32a-09's `two_statement_variant_fails`, N32a-10's offset and time-cursor variants, N32a-07's lint
and serializer plants, N32a-08's range-keyed and tick-keyed builds) are the first tests of the same control rows.

### End-to-end script and logging
`scripts/e2e/tether_counterexamples.sh` (header `bead=N32-CX milestone=M1 labels=tether,registry`) sources the
STD-E2E helper, runs `cargo test -p frob-conformance --test tether_counterexamples`, regenerates the fixture and
compares digests, re-runs the seeding tool and the join (`cargo test -p frob-conformance --test seed_join`) on a
scratch copy of `registry/` and asserts by digest that a second seeding run is a no-op, and plants in scratch
copies one past-due entry with no caught control and one entry carrying a private field, expecting each to fail. It
writes `target/e2e-logs/tether_counterexamples/<utc-timestamp>.ndjson` and a transcript: per case an `assert` line
with `ts`, `script`, `bead`, `step` (the CX number or tag), the exact command line, exit code, wall time, the
recomputed value, the stated value and the tolerance; a `metric` line with entries present (7 + 3), entries due,
entries with a caught control; the digest of the registry file and of the fixture; on failure the first differing
field and the registry path; a `summary` line. The mutant runs through the real `frob` and `frobd` binaries
(`expected: fail` on the mutant, `expected: pass` on the release build) are logged by N32b-24's script, which reads
the fixture this bead writes.

### Known answers and negative controls
- The cases above are the negative controls of KA-756, KA-758, KA-763, KA-764 and KA-765.
- Controls on the registry itself: a catalogue entry with no plant; an entry past due with no caught control; an
  entry carrying a private field; a case value edited to a wrong digit (621 → 612); a provisional tag left past
  due. Each must fail its check.
- Determinism: the checks are pure arithmetic and file reads, class D0, identical twice on one machine, in debug
  and release, and on both platforms.

## Considerations

- A mutant is code that is wrong on purpose. Keeping it buildable costs maintenance; the alternative, a control
  that exists only on paper, has been seen to rot into a test that cannot fail.
- CX-351 is a queueing model with Poisson arrivals, not a measurement. A swarm's arrivals burst; the daemon's
  capacity table scales these waits by Kingman's factor (c_a² + c_s²)/(1 + c_s²) from its own trace. The control is
  the direction and size of the gap, not the third digit.
- The two DESIGN additions are proposals of this unit, numbered with the packet case from one block of three (the
  owner files its exact count, N38a-40). Their rules have no gate at plan time, so they sit in the seeding tool's
  findings file until the M3 exit disposes of them: a gate added (the natural homes are a Tether gate on token
  attenuation and one on archive merges) or the entry withdrawn with its reason; a withdrawn entry's number is
  recorded `void`, never reissued.
- The executed controls run in other beads at later milestones; until they report `caught`, the gates they protect
  cannot print `pass` (STD-TEST's `GateReport`), which is the intended pressure, not a defect of this registry.

## References

§32.20 (list of counterexamples), §32.7, §32.8.1, §32.8.2, §32.10, §32.11.2, §32.24, §32.13; Appendix N.29
(CX-345 to CX-351); KA-756, KA-758, KA-763 to KA-765; GATE-TETHER-05 to -08, -10; PO-TETHER-01; DC-09.
