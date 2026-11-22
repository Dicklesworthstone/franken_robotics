---
key: "N32-CX"
title: "Tether counterexample registry: CX-345 to CX-351 with their planted mutants, and the unnumbered cases"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "tether", "frob-conformance", "frob-verbs", "solid", "registry", "test"]
parent: "WP-06"
deps: ["BD-025"]
needs: ["§38.9: the negative-control registry's record format and its append-only numbering of controls (`NC-…`), into which these counterexamples are entered", "Appendix N: allocation of counterexample numbers for the three cases this chapter states without one (owner of the counterexample catalogue)"]
related: ["N32-REF", "N32-KA", "N32b-24", "N32b-25", "BD-211", "BD-210", "BD-296", "BD-146", "N32b-02", "N32b-03", "N32b-41"]
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

In: the seven rows; for each, the executable case, the mutant specification and the expected failing assertion;
the catalogue entries (`registries/counterexamples.toml`) and their links to controls; the unnumbered cases.
Out: building the mechanisms (the beads named per row); wiring the controls to gates and running them (N32b-24);
the known answers of the chapter (the registry bead `N32-KA`).

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

**How the M/G/1 numbers are obtained** (so the control can be re-derived). With arrival rate λ and utilization
ρ = λ·E[S]: W₀ = λ·E[S²]/2. First-come-first-served wait: W₀/(1 − ρ). Non-preemptive priority for the short class:
W₀/(1 − ρ₁) with ρ₁ = λ·0.9·10. Preempt-resume, short class: λ₁·E[S₁²]/(2·(1 − ρ₁)) = λ·0.9·100/(2·(1 − ρ₁)), plus at
most one quantum (5 s, TARGET). At ρ = 0.7: λ = 0.7/189 = 0.0037037 per second, W₀ = 600.2 s, ρ₁ = 0.03333; the three
waits are 2,000.6 s, 620.9 s and 0.172 s.

**Cases the chapter states without a catalogue number.** They are entered with provisional local names and receive
numbers from the catalogue owner (numbers are append-only).

| Local name | The tempting shortcut | The case | The rule | Owner bead |
|---|---|---|---|---|
| `tether-packet-payload-budget` | Budget a single-file replay by its binary payload size | 6,200,000 bytes of timeline rows are under an 8 MiB (8,388,608-byte) cap as binary, 8,266,667 bytes once Base64-encoded (× 4/3) and 8,866,667 bytes with the 600,000-byte page: over | The packet writer computes the encoded size before it writes anything; over the cap it refuses with the size table and three reductions; nothing is truncated | N32b-41 and the packet writer |
| `tether-derived-budget-copy` (DESIGN addition) | Give a derived token a copy of the budget it asks for, without debiting the parent | A parent with 600 GPU-seconds derives three sub-agent tokens of 600 each: 2,400 GPU-seconds are spendable under a 600-second grant | The budget of a derived token is debited from its parent in the transaction that creates it | N32b-02 |
| `tether-leaderboard-arrival-order` (DESIGN addition) | Break leaderboard ties by arrival time | Entries P and Q have equal metric. Daemon X saw P then Q; daemon Y saw Q then P. After merging, the two leaderboards differ | Ties break by a total order: metric, then content address | N32b-03 |

**Registry record (DESIGN).** One entry per case in `registries/counterexamples.toml`: `id`, `chapter = 32`,
`shortcut`, `case` (the numbers above as data), `rule`, `mutant` (the build flag or test-only type that realizes
the shortcut), `controls` (the `NC-…` numbers allocated by N32b-24), `gates`, `owner_bead`. Mutants live behind
`#[cfg(test)]` or a `negative-control` cargo feature that no release profile enables; `xtask check-deps` refuses a
release build with the feature on.

## Acceptance criteria

- [ ] Seven catalogue entries CX-345 to CX-351 exist with the case numbers above as machine-readable data, and
      three provisional entries with local names.
- [ ] Each entry names a mutant that exists in the tree behind the test-only feature, and at least one control and
      one gate (the join required by the conformance registry: no counterexample without a control and a gate).
- [ ] For each of the seven, a test builds the mutant and shows the protecting assertion failing, and shows it
      passing on the real mechanism.
- [ ] The M/G/1 figures are recomputed by a host test from the two service times and the mix, not copied.
- [ ] No release build can enable a mutant (planted attempt refused).

## Test plan

### Unit tests
`crates/frob-conformance/tests/tether_counterexamples.rs`:
- `cx345_two_statement_double_charges`: deterministic schedule A-lookup, B-lookup, A-insert, B-insert on the mutant
  yields 2 jobs and 2 charges; on the real binding yields 1 and 1, the second caller receiving
  `idempotent_replay: true`.
- `cx346_offset_repeats_row`: rows {1, 2, 3, 4}, page size 2, append 1.5 after page one: mutant page two = {2, 3};
  real page two = {3, 4}; concatenation equals the frontier snapshot {1, 2, 3, 4}.
- `cx347_time_cursor_skips`: worlds w₁ and w₂ each with a sample at t = 2, page ends on (2, w₁): mutant skips
  (2, w₂); real returns it next.
- `cx348_double_decoder`: 9,007,199,254,740,993 as a bare number decodes to …992; as `int53` it is refused with
  `FROB-REF-808`; as `u64str` it round-trips exactly.
- `cx349_overlapping_ranges`: extents [0, 100) and [50, 150) with C = 50: the real store sees chunks 0, 1 then 1, 2;
  chunk 1 is a duplicate (same bytes, original receipt) or `FROB-REF-810` (different bytes); the mutant commits both.
- `cx350_reset_overwrites`: episode 0 and episode 1 both write tick 0: real keeps two chunks; mutant keeps one.
- `cx351_mg1_table`: closed-form values 857, 2,001, 3,430, 7,716 s (first-come); 439, 621, 713, 806 s (priority
  only); 0.12, 0.17, 0.20, 0.22 s (preempt-resume) at ρ = 0.5, 0.7, 0.8, 0.9, to the printed digits.
- `packet_payload_budget`: 6,200,000 → 8,266,667 → 8,866,667 > 8,388,608.
- `derived_budget_copy` and `leaderboard_arrival_order`: the two DESIGN additions, on their mutants.

### End-to-end script and logging
`scripts/e2e/tether_known_answers.sh` (owned by N32b-24) runs the mutant builds through the real `frob` and
`frobd` binaries where a surface exists (idempotency, pagination, integers, chunks) and records, per case, an
NDJSON `assert` line with `expected: "fail"` on the mutant and `expected: "pass"` on the release build, the command
line, exit code, the evidence block, the first differing value and the digests compared. This bead contributes the
case data file `fixtures/tether/counterexamples.json` that the script reads.

### Known answers and negative controls
- The cases above are the negative controls of KA-756, KA-758, KA-763, KA-764 and KA-765.
- Control on the registry itself: a catalogue entry with no mutant, or a mutant no gate fails on, must fail the
  registry join.
- Determinism: the interleaving cases run on the deterministic runtime with explicit schedules, so they are
  identical twice on one machine and across platforms.

## Considerations

- A mutant is code that is wrong on purpose. Keeping it buildable costs maintenance; the alternative, a control
  that exists only on paper, has been seen to rot into a test that cannot fail.
- CX-351 is a queueing model with Poisson arrivals, not a measurement. A swarm's arrivals burst; the daemon's
  capacity table scales these waits by Kingman's factor (c_a² + c_s²)/(1 + c_s²) from its own trace. The control is
  the direction and size of the gap, not the third digit.
- The two DESIGN additions are proposals of this unit; if the catalogue owner declines numbers for them they stay
  as unit tests of their beads.

## References

§32.20 (list of counterexamples), §32.7, §32.8.1, §32.8.2, §32.10, §32.11.2, §32.24, §32.13; Appendix N.29
(CX-345 to CX-351); KA-756, KA-758, KA-763 to KA-765; GATE-TETHER-05 to -08, -10; PO-TETHER-01; DC-09.
