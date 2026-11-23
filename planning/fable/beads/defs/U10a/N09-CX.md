---
key: "N09-CX"
title: "registry: the three doctrine counterexamples (CX-016 to CX-018) seeded into the negative-control registry"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "gauntlet", "lemma", "spine", "registry", "solid", "frob-conformance", "test"]
parent: "WP-14"
deps: ["BD-025", "STD-TEST", "STD-E2E"]
needs: []
related: ["N09-KA", "N10-CX", "N10a-02", "N10a-05", "N10a-06", "N10a-07", "BD-030", "BD-041", "BD-042", "BD-043", "BD-146"]
gates: ["GATE-GAUNT-01"]
plan_refs: ["Appendix N.1", "Appendix N.6", "§9 D-9", "§9 D-17", "§9 D-18", "§9 D-19", "§38.1", "§38.11"]
---
## Background

FrankenRobotics formulates robot design, simulation and control in algebra and Lie theory, and keeps that
honest with nineteen doctrine rules (D-1 to D-19). Rule D-9 says: a plausible shortcut ships with its
counterexample. Where an obvious simplification is wrong, the project states the smallest case a reader can
check by hand that breaks it, and that case becomes a permanent negative control. The cases are the entries of
the counterexample catalogue, numbered CX-001 onward, append-only: a number is never reused and never
renumbered. Each entry is the seed of at least one record in the negative-control registry of the verification
crate `frob-conformance`: the shortcut is planted as a mutation or as an input, and the gates of the rule it
protects must catch it. The reason is the program's rule that a gate nothing can fail validates nothing: a gate
with no control in the registry cannot record a pass.

The doctrine chapter itself states three counterexamples, CX-016 to CX-018. They protect the three doctrine
rules that rest on recorded planning pilots: a specialized method names the generic method it must beat (D-17),
a strict decision is computed exactly (D-18), and a model assumption is an input with a name (D-19). The
counterexamples of the forty constructions (CX-019 to CX-097) are carried by the sibling registry bead N10-CX;
nothing carried these three, although the doctrine check for D-9 (N10a-05) reads both registries and the
doctrine checks N10a-02, N10a-06 and N10a-07 plant the defects. This bead is the registry of the three: it
carries each entry in full so that no reader needs the plan, seeds one control record per entry, names the bead
that plants the defect, and checks that nothing is lost.

All three cases are FACT about small planning pilots that ran outside this repository. Each is re-run here
from the reproduction bundle before its control is marked active (the re-runs are the M0 seed beads named in
the table); until then the record is seeded and its status says so.

Terms. *Control*: a defect, not a boundary: "the request is refused at the limit" tests the limit, and the
control is the build or input in which the limit was removed. *Control kinds*: `mutation` (a test build in
which the shortcut replaces the rule), `planted-input` (a request that asks for the unsound conclusion and must
be refused), `planted-design`, `planted-world`, `a-a` (a wrong experimental procedure run against itself).
*Generic comparator*: the off-the-shelf method that answers the same question as a specialized one (a lifted
linear program, a satisfiability solver, a reachability tool, successive halving, a plain transformer).

Tier: Solid. Crate: `frob-conformance`. Rejected alternative: folding the three entries into N10-CX; rejected
because the registry of a chapter is checked against that chapter's count (three here, 79 there), and because
these three are needed at M0 by the doctrine checks while the constructions' registry is an M1 bead.

## Scope

In: the three entries below as data; one seeded record per entry in
`frob-conformance/registry/controls.toml`; the consistency checks of the test plan; the map from each entry to
the bead that plants it and to the bead whose re-run licenses it.

Out: the planted defects themselves (N10a-02 for CX-016, N10a-07 for CX-017, N10a-06 for CX-018); the four
known-answer rows of the doctrine chapter that use the same pilots (N09-KA); the re-runs of the pilots (seed
beads BD-042, BD-030, BD-043, BD-041); the 79 entries of the constructions (N10-CX); the registry loader and
the no-control check (BD-025); the join of the catalogue onto the control registry for the M1 batteries
(BD-146); the coverage check from counterexamples to controls and gates (N10a-05).

## Technical approach

### 1. Record format (DESIGN, fixed by the verification subsystem and shared with N10-CX)

```toml
[[control]]
id        = "NC-0017"            # stable, never reused
seed      = "CX-017"             # the catalogue entry it came from
gate      = ["GATE-SPARK-08"]    # every gate that must catch it
kind      = "mutation"           # mutation | planted-input | planted-design | planted-world | a-a
target    = "frob-conformance::g0_doctrine::d18_unguarded_strict_predicate"
defect    = "a strict predicate evaluated in unguarded single precision"
expect    = "452 false admissions and 23,438 false exclusions on the 65,536-row boundary screen"
status    = "seeded: fact-pending-rerun"
rerun     = "BD-030"
last_seen = { caught = true, session = "…", date = "…" }
```

### 2. Rules of the registry that this bead respects

1. A gate with no control in the registry cannot record a pass.
2. A control is run every time its gate is run; a gate run in which a control was not caught is a failure of
   the gate, whatever the passing test says.
3. A control is a defect, not a boundary.
4. A control that has never failed its gate in any recorded run is itself suspect and is reviewed at each
   milestone exit.
5. A case that is FACT from a planning pilot is re-run in this repository before it is registered as an active
   control; until then its status is `seeded: fact-pending-rerun` and no gate may count it.
6. An entry whose rule has no gate is a finding about the plan, reported at the next milestone exit.

### 3. The entries

| CX | The tempting shortcut | The smallest case that breaks it | The rule it protects | Control kind | Planted by | Re-run | Gates |
|---|---|---|---|---|---|---|---|
| CX-016 | Assume a specialized method beats the generic one | On the static-stance family (400 N on four point feet at (±a, ±b) with a = 1/3 m; centre of mass anywhere in a rectangle of half-sizes 150 mm and 75 mm; each foot commanding any force in [0, 160] N; minimize the half-width b) a lifted robust linear program and the specialized projection each reach the optimum in one solve and return the same optimum, b = 3/14 m | A specialized method names the generic method it must beat; if the generic method ties, the project says so and ships the generic method (D-17) | planted-input | N10a-02: a method experiment registered with no generic comparator, and a tie recorded with `ships = "specialized"` | BD-042 | GATE-GAUNT-09 |
| CX-017 | Decide a strict predicate in unguarded single precision | An unguarded `f32` screen of 65,536 boundary cases of one inequality produced 452 false admissions and 23,438 false exclusions; a guarded or exact-integer kernel produced none | Strict predicates, digests and certificate arithmetic run in the exact-integer kernel class, in exact rationals, or in guarded single precision with an exact fallback (D-18; the bet that single precision is adequate on the GPU holds only with this guard) | mutation | N10a-07: a strict predicate evaluated in plain single precision on the boundary screen | BD-030 | GATE-SPARK-08 |
| CX-018 | Leave a model assumption in the prose around a claim | Changing one winding feed-forward term in a sampled wheel controller (resistance times the reference current against resistance times the measured current) moved a one-tick spectral radius from 1.48 to 0.92 and changed 24 of 156 task labels; admitting one extra hidden gain (33/32 beside 1 and 2) reversed every feasible label on the pilot's grid of the docking fixture, where a cart must reach position 1 at rest in three one-second ticks under acceleration g·u | A model assumption is an input with a name: actuator authority, load-sharing law, hidden-parameter support, delivery deadline and controller information are fields of the claim (D-19) | planted-input | N10a-06: a claim with its authority assumption left in a note; a claim whose assumption list was removed; a claim whose output does not change when its controller-information field is perturbed and records no implication | BD-043 (wheel loop), BD-041 (docking) | pending: attached by the planting bead, whose D-19 checks run under GATE-LEMMA-06 and GATE-SAFE-08 |

Notes on the table. The first four columns are the catalogue entries as stated; the stance and docking numbers
are those of the shared fixtures (one fixture identity per physical case: the derivation files
`stance_family.md` and `docking_family.md` hold the constants once, for these entries, for the known-answer
rows KA-033 and KA-036 and for the constructions that reuse them). CX-016 is a tie, not a defeat of either
method: the control is the missing comparator or the wrong `ships` value, never the specialized method itself.

### 4. Procedure

1. Load the three entries as seeded records with `seed`, `kind`, a `defect` sentence from the shortcut column,
   an `expect` sentence from the case column, `rerun`, and `target` left for the planting bead to fill.
2. Attach the gates of the last column; for CX-018 leave the list empty and mark the record
   `seeded: gates pending`.
3. When a re-run bead closes with the recorded counts reproduced, the recording session changes the status of
   the record to active; when the planting bead lands it fills `target` and, for CX-018, the gates.
4. A record still pending at the M1 exit is listed in that exit's report.

Determinism class: D0 (data, integers, strings). Nothing here crosses the agent boundary, so no claim product
is emitted; the test writes a hash-named receipt as every target of the verification crate does.

## Acceptance criteria

- [ ] `frob-conformance/registry/controls.toml` holds three records seeded from CX-016, CX-017 and CX-018, each
      with `seed`, `kind`, `defect`, `expect` and `rerun`.
- [ ] No seed number is missing, duplicated or renumbered; the registry check fails on a duplicate `id` or
      `seed` pair.
- [ ] Every record names its planting bead in a field the loader preserves; the three planting beads exist.
- [ ] The records of CX-016 and CX-017 carry GATE-GAUNT-09 and GATE-SPARK-08; the record of CX-018 is marked
      `seeded: gates pending` until its planting bead attaches them.
- [ ] No record is active while its `rerun` bead is open: a gate that counts a `fact-pending-rerun` control is
      refused by the registry check.
- [ ] A printout generated from the registry reproduces the first four columns of the table cell for cell.
- [ ] `scripts/e2e/registry_n09_cx.sh` exits 0.

## Test plan

### Unit tests

In `crates/frob-conformance/tests/`, under the laws-and-self-tests tier:

- `cx_doctrine_count`: exactly three seeds in the range CX-016 to CX-018, none outside it filed under the
  doctrine chapter.
- `cx_doctrine_append_only`: the seed set equals the frozen list; a removed or renumbered seed fails.
- `cx_doctrine_record_shape`: every record has a non-empty `defect`, `expect` and `rerun`, a kind from the
  closed list, and a gate list that is non-empty or marked pending.
- `cx_doctrine_planted_by_resolves`: N10a-02, N10a-06 and N10a-07 exist in the bead graph export.
- `cx_doctrine_status_gate`: a synthetic gate record (labelled `synthetic`) that counts a seeded,
  not-yet-re-run control is refused.
- `cx_doctrine_printout_round_trip`: the generated table equals the stored table byte for byte.
- `cx_doctrine_numbers`: the numbers of the three cases are present in the `expect` fields exactly: 3/14 m and
  one solve each; 452 and 23,438 of 65,536; 1.48, 0.92 and 24 of 156; the reversal of every feasible label.

### End-to-end script and logging

`scripts/e2e/registry_n09_cx.sh` (idempotent; no seed needed): `frob --json doctor`; `cargo test -p
frob-conformance --test g0 -- cx_doctrine`; regenerate the printout and compare it; list pending records with
the bead each waits for. It sources the STD-E2E helper and writes
`target/e2e-logs/registry_n09_cx/<utc>.ndjson` with a transcript; each line has `ts`, `script`, `bead`
(`N09-CX`), `step`, `event` (`start`, `cmd`, `assert`, `metric`, `refusal`, `end`), the command line, exit
code, wall time, the evidence block (claim Known answer, class Tested, tier G0 host, determinism D0), the
counts of records, active and pending against 3, and on failure the first missing or duplicated seed with the
registry path. The `summary` line counts assertions passed and failed and names GATE-GAUNT-01.

### Known answers and negative controls

Known answers: the count 3; the range CX-016 to CX-018; the numbers listed under `cx_doctrine_numbers`.
Negative controls for this bead: a registry with one seed deleted, with one seed duplicated, with one kind
misspelled, with one gate removed from an active record, and with a seeded record marked active before its
re-run; each must fail its test. Metamorphic relation: reordering the records changes no check and no
printout order (records print by seed number). Determinism: twice on one machine and in debug and release
with identical receipts apart from timestamps.

## Considerations

- The three cases are the same pilots as the known-answer rows KA-033 to KA-036 of N09-KA. The two registries
  are different objects (a known answer is a truth a test compares against; a control is a defect a gate must
  catch) and both are kept; the shared constants live once, in the derivation files.
- Until BD-042, BD-030, BD-043 and BD-041 have re-run the pilots, the three records support no gate. That is
  the intended fail-closed state, not a defect of this bead.
- CX-018 has two cases in one entry (the wheel loop and the docking fixture); one record with two `rerun`
  beads is kept, because the catalogue has one number for it.
- A control that cannot be planted as described is a finding, reported at the milestone exit, never a pass.
- Privacy and licence: none of these fixtures uses a private scene or a foreign asset.

## References

Plan Appendix N.1 and N.6; §9 (D-9, D-17, D-18, D-19); §38.1, §38.11; GATE-GAUNT-01, GATE-GAUNT-09,
GATE-SPARK-08; KA-033 to KA-036; BET-08.
