---
key: "N09-CX"
title: "registry: the three doctrine counterexamples (CX-016 to CX-018) seeded into the negative-control registry"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "gauntlet", "lemma", "spine", "registry", "solid", "frob-conformance", "test"]
parent: "WP-14"
deps: ["BD-025", "N38b-05", "STD-E2E", "STD-TEST"]
needs: []
related: ["N09-KA", "N10-CX", "N10a-02", "N10a-06", "N10a-07"]
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
enters each entry in full in the counterexample catalogue, so that no reader needs the plan, with the kind of
control, the planted defect and the bead that plants it, from which the seeding tool of N38b-05 derives one
control per entry; and it checks that nothing is lost.

All three cases are FACT about small planning pilots that ran outside this repository. Each is re-run here
from the reproduction bundle before its control may count (the re-runs are the M0 beads named in the
table); until then the entry carries `rerun = false` and the seeding writes its control as an `obligation`.

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

In: the three entries below as data in the counterexample catalogue
`frob-conformance/registry/counterexamples.toml`, in N38b-05's record schema; the consistency checks of the test
plan; the map from each entry to the bead that plants it and to the bead whose re-run licenses it.

Out: the catalogue's record schema and loader, and the seeding tool that turns entries into rows of
`registry/controls.toml`, allocates their `NC-` identifiers, keeps the seed map, resolves rules that name no
gate and proves completeness by the join (N38b-05; this bead writes no row of `controls.toml` by hand); the
planted defects themselves (N10a-02 for CX-016, N10a-07 for CX-017, N10a-06 for CX-018); the four
known-answer rows of the doctrine chapter that use the same pilots (N09-KA); the re-runs of the pilots
(BD-042, BD-030, N38a-11, BD-041); the 79 entries of the constructions (N10-CX); the control record format,
loader and the no-control check (BD-025); running controls and writing `last_seen` (N38b-04); the wiring of
controls into the M1 batteries (BD-146); the coverage check from counterexamples to controls and gates
(N10a-05).

## Technical approach

### 1. Catalogue record (the schema of N38b-05, shared with every chapter's registry bead)

Each of the three is one entry of `registry/counterexamples.toml` with the fields of N38b-05: `id` (the CX
number, append-only), `shortcut`, `case`, `numeric`, `plan_case`, `case_supplied_by`, `rule`, `rule_ids`,
`defined_in`, `same_as`, `label`, `rerun`, `rerun_receipt`, `control_kind` and `plant`. The expected
observation is kept in the `case` text, from which the seeding tool writes the control's `expect`. The seeding
tool writes one control per entry in BD-025's record format (`id`, `seed`, `gate`, `kind`, `target`, `defect`,
`expect`, `state`, `last_seen`). DESIGN, correcting an earlier draft of this bead that wrote control records
straight into `controls.toml` with fields `status` and `rerun = "<bead>"` that the control schema does not
have: that file is written only by the tool, so control identifiers are allocated once and never rewritten; the
re-run state lives on the entry (`rerun`, `rerun_receipt`) and the tool turns it into the control's `state`.

| Field | CX-016 | CX-017 | CX-018 |
|---|---|---|---|
| `numeric`, `plan_case` | true, `numbers` | true, `numbers` | true, `numbers` |
| `rule_ids` | D-17, GATE-GAUNT-09 | D-18, BET-08, GATE-SPARK-08 | D-19 (gates resolved through `registry/rule_gates.toml` once the planting bead maps D-19) |
| `defined_in` | §9 D-17 | §9 D-18; §4 BET-08 | §9 D-19 |
| `same_as` | empty | empty | empty |
| `label` | FACT: planning pilot; re-run by BD-042 | FACT: planning pilot; re-run by BD-030 | FACT: planning pilots; re-run by N38a-11 (wheel loop, ledger row 17) and BD-041 (docking) |
| `rerun` | false until BD-042 reproduces it | false until BD-030 reproduces it | false until both N38a-11 and BD-041 reproduce theirs |
| `control_kind` | `planted-input` | `mutation` | `planted-input` |
| `plant` | N10a-02's planted experiment fixture holding two requests, a method experiment registered with no generic comparator and a tie recorded with `ships = "specialized"`; the control is caught only if `d17_*` refuses both | N10a-07's build in which the strict predicate of the 65,536-row boundary screen is evaluated in unguarded `f32` | N10a-06's planted claims: one with its authority assumption in a note (`d19_note_lint`), one whose output does not change when its `controller-information` field is perturbed and records no implication; caught only if both are refused |

`rerun_receipt` is empty while `rerun = false`; for CX-018 it holds both receipt hashes in re-run order
(DESIGN: one entry, one control, two licensing re-runs, because the catalogue has one number for it).

### 2. Rules of the registry that this bead respects

1. A gate with no control in the registry cannot record a pass.
2. A control is run every time its gate is run; a gate run in which a control was not caught is a failure of
   the gate, whatever the passing test says.
3. A control is a defect, not a boundary.
4. A control that has never failed its gate in any recorded run is itself suspect and is reviewed at each
   milestone exit.
5. A case that is FACT from a planning pilot is re-run in this repository before its control is runnable;
   until then the entry has `rerun = false`, the seeding writes its control with `state = "obligation"`, and
   no gate may count it.
6. An entry whose rule has no gate is a finding about the plan, written by the seeding tool to
   `registry/seed_findings.toml` with a requested disposition (`gate-added` or `withdrawn`) and reported at the
   next milestone exit.

### 3. The entries

| CX | The tempting shortcut | The smallest case that breaks it | The rule it protects | Control kind | Planted by | Re-run | Gates (in `rule_ids`) |
|---|---|---|---|---|---|---|---|
| CX-016 | Assume a specialized method beats the generic one | On the static-stance family (400 N on four point feet at (±a, ±b) with a = 1/3 m; centre of mass anywhere in a rectangle of half-sizes 150 mm and 75 mm; each foot commanding any force in [0, 160] N; minimize the half-width b) a lifted robust linear program and the specialized projection each reach the optimum in one solve and return the same optimum, b = 3/14 m | A specialized method names the generic method it must beat; if the generic method ties, the project says so and ships the generic method (D-17) | planted-input | N10a-02: a method experiment registered with no generic comparator, and a tie recorded with `ships = "specialized"` | BD-042 | GATE-GAUNT-09 |
| CX-017 | Decide a strict predicate in unguarded single precision | An unguarded `f32` screen of 65,536 boundary cases of one inequality produced 452 false admissions and 23,438 false exclusions; a guarded or exact-integer kernel produced none | Strict predicates, digests and certificate arithmetic run in the exact-integer kernel class, in exact rationals, or in guarded single precision with an exact fallback (D-18; the bet that single precision is adequate on the GPU holds only with this guard) | mutation | N10a-07: a strict predicate evaluated in plain single precision on the boundary screen | BD-030 | GATE-SPARK-08 |
| CX-018 | Leave a model assumption in the prose around a claim | Changing one winding feed-forward term in a sampled wheel controller (resistance times the reference current against resistance times the measured current) moved a one-tick spectral radius from 1.48 to 0.92 and changed 24 of 156 task labels; admitting one extra hidden gain (33/32 beside 1 and 2) reversed every feasible label on the pilot's grid of the docking fixture, where a cart must reach position 1 at rest in three one-second ticks under acceleration g·u | A model assumption is an input with a name: actuator authority, load-sharing law, hidden-parameter support, delivery deadline and controller information are fields of the claim (D-19) | planted-input | N10a-06: a claim with its authority assumption left in a note; a claim whose output does not change when its controller-information field is perturbed and records no implication. (A bundle whose assumption list was removed is the claim-rule battery N00-27's control, KA-031, which N10a-06 reads and does not plant again.) | N38a-11 (wheel loop), BD-041 (docking) | pending: attached by the planting bead, whose D-19 checks record under GATE-LEMMA-06 and GATE-LEMMA-03 |

Notes on the table. The first four columns are the catalogue entries as stated; the stance and docking numbers
are those of the shared fixtures (one fixture identity per physical case: the derivation files
`stance_family.md` and `docking_family.md` hold the constants once, for these entries, for the known-answer
rows KA-033 and KA-036 and for the constructions that reuse them). CX-016 is a tie, not a defeat of either
method: the control is the missing comparator or the wrong `ships` value, never the specialized method itself.

### 4. Procedure

1. Enter the three entries in `registry/counterexamples.toml` with every field of section 1 (`rerun = false`).
2. Run N38b-05's seeding tool: it allocates one NC number per entry, writes three controls with `seed` = the
   CX number, `kind` = `control_kind`, `defect` from the shortcut, `expect` from the case and `state =
   "obligation"`, and attaches the gates resolved from `rule_ids` (GATE-GAUNT-09 for CX-016, GATE-SPARK-08
   for CX-017). CX-018's rule resolves to no gate yet and is written to `seed_findings.toml` with `gate-added`
   requested.
3. When a re-run bead closes with the recorded counts reproduced, the recording session sets `rerun = true` and
   `rerun_receipt` on the entry; the next seeding makes that control runnable with the same NC number. When
   N10a-06 maps D-19 to the gates its checks record under (in `registry/rule_gates.toml`), the next seeding
   attaches them to CX-018's control and closes its finding as `gate-added`. When a planting bead lands it
   completes the entry's `plant` with the code path and the feature or fixture it built (the CX number and
   every other field stay as entered), and the next seeding copies them into the control's `target` and
   `feature` or `fixture`.
4. A control still an obligation, or a finding still open, at the M1 exit is listed in that exit's report.

Determinism class: D0 (data, integers, strings). Nothing here crosses the agent boundary, so no claim product
is emitted; the test writes a hash-named receipt as every target of the verification crate does.

## Acceptance criteria

- [ ] `registry/counterexamples.toml` holds the three entries CX-016, CX-017 and CX-018 with every field of
      section 1; re-running N38b-05's seeding gives exactly one control per entry (three) and a second run
      changes no byte; no row of `controls.toml` was written by hand.
- [ ] No CX number is missing, duplicated or renumbered; the doctrine chapter's count in the catalogue loader
      is 3.
- [ ] Every entry's `plant` names its planting bead; the three planting beads exist.
- [ ] After seeding, the controls of CX-016 and CX-017 carry GATE-GAUNT-09 and GATE-SPARK-08; CX-018 appears
      in `seed_findings.toml` with `gate-added` requested until D-19 resolves to a gate.
- [ ] No control is runnable while its entry has `rerun = false`: the three are `obligation` until their
      re-runs, and a `pass` row that cites only one of them is refused by the registry check (`no-control`).
- [ ] A printout generated from the registry reproduces the first four columns of the table cell for cell.
- [ ] `scripts/e2e/registry_n09_cx.sh` exits 0.

## Test plan

### Unit tests

In `crates/frob-conformance/tests/`, under the laws-and-self-tests tier:

- `cx_doctrine_count`: exactly three entries in the range CX-016 to CX-018, none outside it with `defined_in`
  under §9.
- `cx_doctrine_append_only`: the id set equals the frozen list; a removed or renumbered entry fails.
- `cx_doctrine_entry_shape`: every entry has every field of section 1, a `control_kind` from the closed list, a
  non-empty `plant`, `label` FACT with a `rerun` field present (a FACT entry without it is a loader error), and
  `rerun = true` only with a `rerun_receipt` that names a receipt on file.
- `cx_doctrine_planted_by_resolves`: N10a-02, N10a-06 and N10a-07 exist in the bead graph export.
- `cx_doctrine_seeded_once`: after seeding, exactly one control per entry with `seed` = its CX number and `kind`
  = its `control_kind`; a second seeding changes no byte of `controls.toml`.
- `cx_doctrine_obligation_until_rerun`: with `rerun = false` each control is `obligation` and a synthetic gate
  record (labelled `synthetic`) with status `pass` citing it alone is refused (`no-control`); with `rerun =
  true` and a receipt (on a scratch copy) the same NC number becomes runnable.
- `cx_doctrine_gates_resolved`: CX-016 → {GATE-GAUNT-09}, CX-017 → {GATE-SPARK-08}; CX-018 is a finding with
  `gate-added` requested while D-19 has no row in `rule_gates.toml`.
- `cx_doctrine_printout_round_trip`: the generated table equals the stored table byte for byte.
- `cx_doctrine_numbers`: the numbers of the three cases are present in the `case` fields exactly: 3/14 m and
  one solve each; 452 and 23,438 of 65,536; 1.48, 0.92 and 24 of 156; the reversal of every feasible label.

### End-to-end script and logging

`scripts/e2e/registry_n09_cx.sh` (idempotent; no seed needed; it calls only cargo targets of its
dependencies, so no `frob` command): `cargo test -p frob-conformance --test g0_laws -- cx_doctrine`; N38b-05's
seeding tool, `cargo test -p frob-conformance --test seed_controls -- --ignored seed`, run twice (the second run
must leave the digest of the committed `controls.toml` unchanged), then its join, `cargo test -p
frob-conformance --test seed_join`, for this chapter's entries; regenerate the printout and compare it;
list the obligations and findings with the bead each waits for; then, on scratch copies under
`target/e2e-scratch/registry_n09_cx/` (never the committed files), plant the five negative controls below one
at a time and assert that each is rejected by its named test. It sources the STD-E2E helper and writes
`target/e2e-logs/registry_n09_cx/<utc>.ndjson` with a transcript; each line has `ts`, `script`, `bead`
(`N09-CX`), `step`, `event` (`start`, `cmd`, `assert`, `metric`, `refusal`, `end`), the command line, exit
code, wall time, the evidence block (claim Known answer, class Tested, tier G0 host, determinism D0), the
counts of entries and of seeded controls against 3, of runnable controls, obligations and open findings, the
digest of `controls.toml` before and after the second seeding, each planted copy with the test that rejected
it, and on failure the first missing or duplicated entry with the registry path. The `summary` line counts assertions passed and failed and names GATE-GAUNT-01.

### Known answers and negative controls

Known answers: the count 3; the range CX-016 to CX-018; the numbers listed under `cx_doctrine_numbers`.
Negative controls for this bead: a catalogue with one entry deleted (`cx_doctrine_count`,
`cx_doctrine_append_only`), with one entry duplicated (the catalogue loader's duplicate check), with one
`control_kind` misspelled (`cx_doctrine_entry_shape`), with GATE-GAUNT-09 removed from CX-016's `rule_ids`
(`cx_doctrine_gates_resolved`), and with an entry set `rerun = true` with no `rerun_receipt`, that is, licensed
before its re-run (`cx_doctrine_entry_shape`, which requires a receipt that resolves); each must fail its
test. Metamorphic relation: reordering the entries changes no check and no printout order (entries print by CX
number). Determinism: twice on one machine and in debug and release
with identical receipts apart from timestamps.

## Considerations

- The three cases are the same pilots as the known-answer rows KA-033 to KA-036 of N09-KA. The two registries
  are different objects (a known answer is a truth a test compares against; a control is a defect a gate must
  catch) and both are kept; the shared constants live once, in the derivation files.
- Until BD-042, BD-030, N38a-11 and BD-041 have re-run the pilots, the three records support no gate. That is
  the intended fail-closed state, not a defect of this bead.
- CX-018 has two cases in one entry (the wheel loop and the docking fixture); one entry and one control with
  two licensing re-runs are kept, because the catalogue has one number for it.
- A control that cannot be planted as described is a finding, reported at the milestone exit, never a pass.
- Privacy and licence: none of these fixtures uses a private scene or a foreign asset.

## References

Plan Appendix N.1 and N.6; §9 (D-9, D-17, D-18, D-19); §38.1, §38.11; GATE-GAUNT-01, GATE-GAUNT-09,
GATE-SPARK-08; KA-033 to KA-036; BET-08.
