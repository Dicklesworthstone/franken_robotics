---
key: "N09-KA"
title: "frob-conformance: known-answer registry of the doctrine chapter, KA-033 to KA-036"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "gauntlet", "frob-conformance", "solid", "spine", "registry", "test"]
parent: "WP-14"
deps: ["BD-025", "STD-TEST", "STD-E2E"]
needs: ["§25.22: the plan-time pilots re-run from the reproduction bundle, which turn these four rows from FACT about a pilot into rows a gate may cite: BD-042 (static stance family), BD-030 (boundary screen of 65,536 rows), BD-043 (remaining pilots, including the wheel-loop feed-forward term), BD-041 (cart docking with two gains and with three)"]
related: ["BD-030", "BD-041", "BD-042", "BD-043", "BD-146", "N10a-02", "N10a-06", "N10a-07", "N10-KA1", "N09-CX"]
gates: ["GATE-GAUNT-02", "GATE-GAUNT-08"]
plan_refs: ["§9 D-17 D-18 D-19", "§38.5", "§38.6", "Appendix M.1", "Appendix M.8", "Appendix M.37"]
---
## Background

The known-answer suite is the project's list of cases with a truth a test can compare against. Each row has a
case, a truth, a tolerance, a tier and execution site, and a negative control that must make it fail. The
registry file is the authority and the printed appendix is generated from it; a number is never reused or
renumbered, and new rows append.

The doctrine chapter states four known answers. All four are FACT about small planning pilots that ran outside
this repository: each is re-run here from the reproduction bundle before any gate cites it, and until then the
row is an obligation, not a test. They matter because three doctrine rules rest on them: a specialized method
must name the generic method it has to beat (D-17), strict decisions are computed exactly (D-18), and a model
assumption is an input with a name (D-19).

Chapter summary at plan time: 4 rows (KA-033 to KA-036), of which 1 has tolerance printed `to set` and 1 has
negative control printed `none stated`; 3 counterexample entries (CX-016 to CX-018) seed controls.

## Scope

In: the four registry records with every field; a tolerance for the row printed `to set`; a control for the
row printed `none stated`; one derivation file per row under `docs/derivations/`; the link from each row to
the pilot re-run that licenses it.

Out: re-running the pilots (the seed beads named above); the registry's schema checks and the join onto the
control registry (Gauntlet beads); the doctrine checks that consume these rows (N10a-02, N10a-06, N10a-07).

## Technical approach

**Record format** (`frob-conformance/registry/known_answers.toml`; DESIGN, one record per row):

```toml
[[known_answer]]
id         = "KA-034"
where      = "§9 D-18"
case       = "…"
truth      = "…"
tolerance  = "exact"                 # exact | a value with its unit and reason | to-set (not eligible to pass)
tier       = "G1"
site       = "all tiers"
control    = ["NC-…"]                # empty = obligation; the row supports no gate
derivation = "docs/derivations/boundary_screen_65536.md"
status     = "fact-pending-rerun"    # fact-pending-rerun | registered | passing
rerun      = "BD-030"                # the bead whose run licenses the row
```

**Rules every row obeys.** (1) A derivation file states the model, the derivation, the value with its digits
and the error budget of the test. (2) A tolerance carries a reason: `exact` for integers, rationals, Booleans,
verdicts and digests; a row printed `to set` is not eligible to pass until its derivation file states one.
(3) A negative control is a mutation or planted failure, never a boundary behaviour; a row printed
`none stated` cannot support a gate until a control is registered. (4) A row runs first on the weakest site
that can decide it and then on every tier its gate names. (5) Values are not improved in transit: the row
carries the chapter's printed value; a recomputation that disagrees is recorded beside it, never substituted.

**The rows.**

| KA | Rule | The case | The truth | Tolerance | Tier · site | Negative control | Re-run |
|---|---|---|---|---|---|---|---|
| KA-033 | D-17 | Generic comparator on the static-stance family: 400 N on four point feet at (±a, ±b) with a = 1/3 m; centre of mass anywhere in a rectangle of half-sizes 150 mm and 75 mm; each foot commanding any force in [0, 160] N; minimize the half-width b | A lifted robust linear program and the specialized projection each reach the optimum in one solve and return the same optimum, half-width 3/14 m | exact (rational optimum; solve counts) | G2 · host | Printed `none stated`. Set here: the lifted program with one vertex of the load rectangle dropped, which returns a smaller half-width and must be flagged as disagreeing with the projection | BD-042 |
| KA-034 | D-18 | Strict decisions on a screen of 65,536 boundary cases of one inequality | An unguarded `f32` screen produces 452 false admissions and 23,438 false exclusions; a guarded or exact-integer kernel produces none | exact (counts) | G1 · all tiers | The unguarded `f32` comparison, which must show the errors | BD-030 |
| KA-035 | D-19 | One winding feed-forward term in a sampled wheel controller (the wheel-loop fixture: the feed-forward uses the resistance times the reference current in one variant and the resistance times the measured current in the other) | The one-tick spectral radius moves from 1.48 to 0.92 and 24 of 156 task labels change | Printed `to set` for the radii. Set here: ±0.005 on the two-decimal values, and ±5·10⁻⁵ on the four-decimal values 1.4820 and 0.9207 the pilot recorded; exact for the label count | G1 · host | The assumption left as prose and not as a field of the claim: a claim whose feed-forward law is not a typed field must be rejected by the claim-shape check | BD-043 |
| KA-036 | D-19 | One extra hidden gain admitted on the docking fixture (a cart must reach position 1 at rest in three one-second ticks; acceleration g·u with hidden gain g ∈ {1, 2}; the third gain 33/32 is added to the support) | Every feasible label on the pilot's grid reverses (34,590 positive labels per grid in the pilot); exact docking becomes unsatisfiable and the worst-case error is at least 1/65 m | exact (verdict and count) | G1 · host | Inclusions carried across the enlarged world set | BD-041 |

**Derivation files.** `stance_family.md` (shared with the stance rows of other chapters: the moment equations
give f₊₊ − f₋₋ = 90 + 15/b at the positive load corner, so b = 3/14 m with forces (160, 130, 110, 0) N);
`boundary_screen_65536.md` (the inequality, the row generator and the three kernel classes);
`wheel_loop_feedforward.md` (the one-tick map of the sampled loop with the hold modelled exactly, and why the
instantaneous-current model gives the same radius for both laws and is therefore the wrong model);
`docking_family.md` (shared with the docking rows: with first command p the remaining commands are
(1/g − 2p, p − 1/g); a robust policy exists exactly when the sensor error is below (U + ½)/4 with the packet
on time).

**One fixture identity per physical case.** The stance and docking fixtures appear again in the constructions
and in the mission chapter; the registry records the sharing so that constants exist once.

## Acceptance criteria

- [ ] Four records exist in `known_answers.toml` with every field filled; none has tolerance `to-set` or an
      empty control list.
- [ ] Four derivation files exist and each states the value with its digits and the error budget.
- [ ] Each record's `status` is `fact-pending-rerun` until its re-run bead closes, after which the recording
      session sets `registered`; no gate may cite a row whose status is `fact-pending-rerun`.
- [ ] The registry schema test passes, and a planted record with a reused identifier, an empty control or a
      `to-set` tolerance marked `passing` fails it.
- [ ] `cargo test -p frob-conformance --test g1 -- KA-033 KA-034 KA-035 KA-036` runs each row against its
      truth once its re-run exists, with its control applied and caught.

## Test plan

### Unit tests

- `ka_records_complete`: the four records parse; fields non-empty; identifiers unique and in range.
- `ka033_optimum`: both programs on the stance family return 3/14 m exactly; the dropped-vertex control
  returns a different value.
- `ka034_counts`: 65,536 rows; unguarded 452 and 23,438; guarded and exact 0 and 0.
- `ka035_radii_and_labels`: spectral radii 1.4820 and 0.9207 within 5·10⁻⁵; 24 of 156 labels differ.
- `ka036_reversal`: with the third gain every label that was feasible on the grid is infeasible; the least
  worst-case error is at least 1/65 m.
- `status_gate`: a gate that cites a `fact-pending-rerun` row is refused by the registry check.

### End-to-end script and logging

`scripts/e2e/ka_doctrine.sh`. Sources the STD-E2E helper. Runs the registry schema test, then the four rows
through the G1 runner (each with its control), and for KA-034 also `frob --json spark probe` on each present
device to reproduce the counts with identical bytes. Log `target/e2e-logs/ka_doctrine/<utc>.ndjson` with
transcript: commands, exit codes, wall times, measured values against the row's truth and tolerance, the
control's outcome, the evidence block where a verb is called, and on failure the first row and value that
differed. Final `summary` line with rows passed, rows still pending re-run, and gate GATE-GAUNT-02.

### Known answers and negative controls

The four rows above are the known answers; their controls are listed in the table. Counterexample entries
that seed the same controls: CX-016 (assume a specialized method beats the generic one), CX-017 (decide a
strict predicate in unguarded single precision), CX-018 (leave a model assumption in the prose around a
claim). Determinism: all four are exact or deterministic computations; KA-034 additionally requires identical
bytes across back ends for the exact and guarded kernels.

## Considerations

- These are facts about pilots, not about this system: no result is claimed until the re-run. A re-run that
  disagrees with a printed value is recorded as a finding with both values; the printed value is not edited.
- KA-034 duplicates the bet-chapter row for the same screen (KA-018); both stay, each protecting its own rule,
  and they share one derivation file.
- The two controls and the tolerance set here are proposals of this bead, registered so the rows can support
  a gate; the owning rule's bead (N10a-02, N10a-06) may refine them.

## References

Plan §9 (D-17, D-18, D-19); §38.5 (plan-time pilots and their results); §38.6 (rules for a row); Appendix M.1,
M.8, M.37. Open questions: OPEN-GAUNT-2, OPEN-GAUNT-3.
