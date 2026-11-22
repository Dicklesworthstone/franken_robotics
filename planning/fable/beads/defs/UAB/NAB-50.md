---
key: "NAB-50"
title: "e2e scripts/e2e/decision_records.sh: matrix, queue, killed ideas, idea passes, sweeps; every lint bites"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "delivery", "gauntlet", "frob-conformance", "solid", "test", "e2e", "decision-record"]
parent: "EPIC-M0"
deps: ["STD-E2E", "NAB-17", "NAB-18", "NAB-19", "NAB-20", "NAB-21", "NAB-22", "NAB-32", "NAB-33", "NAB-34", "NAB-35", "NAB-36", "NAB-37", "NAB-38", "NAB-39", "NAB-40", "NAB-41", "NAB-42", "NAB-43", "NAB-44", "NAB-45", "NAB-46", "NAB-47", "NAB-48", "NAB-49"]
needs: ["§38.13: a `registries/gates.toml` with the kill-gate records and their deciding milestones, read by the due-list step (N40-01, N40-03, N40-30)", "§45.11: the tracker export of bead keys, read by the carrier checks"]
related: ["NAB-23", "NAB-24", "NAB-25", "NAB-26", "NAB-27", "NAB-28", "NAB-29", "NAB-30", "NAB-31", "N40-36", "N40-37"]
gates: []
plan_refs: ["Appendix B", "Appendix C", "§0.7", "§38"]
---
## Background

FrankenRobotics keeps its decisions about ideas as data, in four registries with generated documents: the
expected-value matrix (`registries/ev_matrix.toml`: 143 scored families of mechanism and a queue of 21
Moonshot items), the killed-idea register (`registries/killed_ideas.toml`: 67 entries with their refuting
arithmetic and the condition that would revive each, plus a few parked and declined records), the idea-pass
register (`registries/ideas.toml`: 180 records that say where each idea lives and which beads carry it), and
the discrepancy pins. Lints in `frob-conformance` keep them honest: scores are recomputed, rejected ideas
cannot be rebuilt without citing their record, an idea is never cited as evidence, a withdrawn figure cannot
return, and at every milestone exit a sweep must account for every row whose evidence arrived.

This bead is the end-to-end script for that capability. It drives the whole pipeline from the command line on
the real tree, checks the known counts and arithmetic, plants each defect the lints exist to catch, and leaves
a structured log. With `--exit M<n>` it runs the milestone sweep's due list and completeness check, and is the
script the nine sweep beads call. Tier: Solid; a test bead.

## Scope

In: `scripts/e2e/decision_records.sh`, its `--exit` mode, its planted fixtures and its log. Out: the registries,
generators and lints (NAB-17 to NAB-22, NAB-32 to NAB-49); the sweeps' judgments (NAB-23 to NAB-31).

## Technical approach

Plain shell, sourcing `scripts/e2e/lib.sh`, idempotent, offline, seed-free, on a scratch copy under
`target/e2e-work/decision_records/`. Header: `bead=NAB-50 milestone=M0 labels=gauntlet,decision-records`.

**Default mode.**

1. `e2e_init decision_records NAB-50`; copy `registries/`, `crates/frob-conformance/registry/`,
   `docs/decisions/`, contracts and the tracker export into the scratch tree.
2. **Matrix.** `e2e_cmd -- cargo test -p frob-conformance --test ev_matrix -- --nocapture`. Assertions:
   `families == 143` (41, 59, 43); `score_mismatches == 0`; `unsorted == 0`; rows below 2.0 are 0, 16 and 32
   per table; `constructions_missing == 0` for C-01 to C-40; distinct kill gates named `== 82`, all resolving;
   `queue_items == 21`; `next_entries` per milestone equals the frozen lists.
3. **Killed ideas.** `e2e_cmd -- cargo test -p frob-conformance --test killed_ideas -- --nocapture`.
   Assertions: `ki_records == 67` in groups 9, 7, 9, 7, 14, 10, 3, 4, 4; every `calculation` recomputed
   (`mismatches == 0`); 29 records joined to a hypothesis; 31 aliases resolving; `KD` records 4 and `KP`
   records 4 present; spot values logged as metrics: 4,801; 2.34; 0.37; 75,411; 152; 45.6 GB; 2,025,000; 299;
   2,145; 0.157671; 27; 1.464; 29.96.
4. **Idea passes.** `e2e_cmd -- cargo test -p frob-conformance --test idea_register -- --nocapture`.
   Assertions: 30 `FP`, 88 `IP`, 19 `CT`, 27 `BS`, 16 `UP`; tally 90 + 13 + 9 = 112; unresolved carriers 0;
   unresolved gates 0; the orbit count 86,185.
5. **Discrepancy pins.** `e2e_cmd -- cargo test -p frob-conformance --test discrepancy_pins`; 17 records; no
   withdrawn figure in context.
6. **Revive lint on the real tree.** `e2e_cmd -- cargo test -p frob-conformance --test revive_lint`; metrics:
   matches by case (surviving form, revival, exempt), findings `== 0`.
7. **Generated documents are current.** `e2e_cmd -- cargo xtask check-generated --json`; no `gen.differs` for
   `docs/decisions/ev-matrix.md`, `killed-ideas.md`, `idea-passes.md`; `--write` twice gives identical digests.
8. **Look-ups from the user's side.** `grep`-free checks through the printout: the proposal name
   "copy-on-write forks of device state" resolves to KI-10 by the alias table; idea IP-33 lists its carriers;
   the family "half precision on matrix accelerators" shows `rejected` with `KI-01` and `FROB-REF-245`.
9. **Planted defects** (each through `e2e_expect_caught`, restored afterwards):
   - matrix: a score edited to "2.2" for (3, 3, 4) → `ev.score-mismatch`; a Solid row at 1.5 with no
     `needed_by` → `ev.below-two-unjustified`; a rejected row with an empty reason; an "admitted" row with no
     queue entry; a fifth active Moonshot in the queue function.
   - killed ideas: a calculation's value changed (2.75 for the refinement ratio at n = 18); a record deleted
     (`ki.record-removed`); a bead "Copy-on-write forks for device state" with no citation
     (`ki.match-without-citation`); a revival ADR of the wrong evidence kind; a gate threshold stating a strong
     form (`ki.strong-form-as-goal`).
   - idea passes: an `IP` record with no carrier; a gate record citing `IP-33` as evidence
     (`idea.cited-as-evidence`); a ratio with no label; a `CT` record marked settled with no gate record.
   - pins: the sentence "a ceiling of about 7.8 at a step share of 87%" in a contract
     (`dx.withdrawn-figure`).
10. `summary`.

**Sweep mode (`--exit M<n>`).**

1. `e2e_cmd -- cargo run -p frob-conformance --bin decision-sweep -- due --exit M<n> --json`; assert the count
   of family-and-kill-gate pairs equals the known answer for that exit (1, 2, 8, 22, 24, 11, 8, 0, 8 for M0
   to M8) and log the due killed ideas, queue items, ideas and reading tasks.
2. If `docs/delivery/exits/M<n>/decision-records.json` exists: `… -- check --exit M<n>`; assert no finding;
   log outcomes by kind, `active_after` (threshold `<= 4`) and open reading tasks (threshold `== 0`).
   Otherwise write a skeleton into the scratch tree, fill it from the fixture exit report under
   `tests/fixtures/sweep/M<n>/`, and run `check` on that, so the path is exercised before the real exit.
3. Plant the sweep findings: a due family removed (`sweep.row-not-swept`); a change with no evidence; a fired
   kill gate with the family still "adopted"; a sweep by an implementer session; a row moved without an ADR.

## Acceptance criteria

- [ ] The script exits 0 on a clean tree in default mode and writes
      `target/e2e-logs/decision_records/<utc-timestamp>.ndjson` with a transcript beside it.
- [ ] Every planted defect of step 9 is caught (15 plants), and with a lint stubbed to always pass the script
      itself fails, which shows the catches are real.
- [ ] `--exit M<n>` runs for each of M0 to M8 on fixtures before any real exit, with the due-pair counts 1, 2,
      8, 22, 24, 11, 8, 0, 8.
- [ ] The script touches no gate (the decision records measure none) and leaves the working tree unchanged.
- [ ] Under 120 s in default mode on a warm build on the CPU tier (TARGET).

## Test plan

### Unit tests

None of its own; it drives the test targets `ev_matrix`, `killed_ideas`, `revive_lint`, `idea_register`,
`discrepancy_pins` and `decision_sweep`. Self-test `scripts/e2e/selftest/st_decision_records_restore.sh`: the
scratch tree's digest after the last plant equals its digest before the first.

### End-to-end script and logging

This bead is the script. Per STD-E2E every line carries `ts`, `script`, `bead`, `step`, `event`; `cmd` lines
the exact command line, exit code and wall time; `assert` and `metric` lines the observed value, the expected
value or threshold and the unit; digests of the four registries and three generated documents; on failure the
first differing value, the record id and the scratch path. No `frob` verb is called, so the `evidence` block
is recorded as absent with reason `no-verb`. In sweep mode the log carries the exit, the exit report's receipt
id and the commit. The log quotes no sealed-suite content and no private-scene data.

### Known answers and negative controls

Known answers: 143 families (41, 59, 43); 0, 16 and 32 below 2.0; 82 kill gates named; 21 queue items; 67
killed ideas with 4 declined and 4 parked records beside them; 180 idea records (30, 88, 19, 27, 16); tally
112 = 90 + 13 + 9; 17 pins; due pairs per exit 1, 2, 8, 22, 24, 11, 8, 0, 8 (84 pairs; 72 family-exit pairs
over 69 families). Negative controls: the fifteen plants of step 9 and the five of the sweep mode.
Determinism lanes: twice on one machine with identical assertion results and document digests; identical
digests on macOS (Apple silicon) and Linux (x86-64).

## Considerations

- The script is long because it covers four registries; each step is independent and can be run alone with
  `--only <step>`.
- It must stay cheap: decision records are bookkeeping, and a slow check of bookkeeping is the first thing an
  agent learns to skip. The brute-force orbit count (10⁶ labelings) is the heaviest step and runs in under a
  second.
- Before the gate register and the tracker export exist, the carrier and kill-gate resolution steps record
  `skip` with the reason; the first exit review re-runs the script.

## References

Appendix B and C; §0.7 (a gate that nothing can fail validates nothing; process is never the product); §38.
