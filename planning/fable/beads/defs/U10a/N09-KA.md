---
key: "N09-KA"
title: "frob-conformance: known-answer registry of the doctrine chapter, KA-033 to KA-036"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "gauntlet", "frob-conformance", "solid", "spine", "registry", "test", "review-notes-open"]
parent: "WP-14"
deps: ["BD-025", "N00-KA", "N38a-10", "N38a-12", "N38b-05", "STD-E2E", "STD-TEST"]
needs: []
related: ["BD-030", "BD-041", "BD-042", "N10-KA1", "N10a-02", "N10a-05", "N10a-06", "N10a-07", "N38a-11", "N38b-27", "NEG-10"]
gates: ["GATE-GAUNT-02"]
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

Out: re-running the pilots (the seed beads named above); the row schema, loader, five rules and printout
(N38a-12); the seeding that turns each row's control text into a record of `controls.toml` and fills the row's
`control.nc` (N38b-05; this bead writes no row of `controls.toml`); running rows per site and the gate's
accounting (N38a-13); the doctrine checks that consume these rows (N10a-02, N10a-06, N10a-07).

## Technical approach

**Record format.** One `[[ka]]` row per KA in `crates/frob-conformance/registry/known_answers.toml`, the suite's
one physical file, in the row schema that N38a-12 owns: `id`, `section`, `fixture`, `same_as`, `case`, `truth`,
`tolerance` (`exact`, `abs`, `rel`, `printed-digits`, `gate` or `to-set`), `tier`, `sites`, `control` (`text`,
and the `nc` list filled by N38b-05's seeding), `derivation`, `label`, `due` and `printed`. DESIGN, correcting an
earlier draft of this bead that used a record of its own (`[[known_answer]]` with `where`, `site`, `status` and
`rerun = "<bead>"`): one schema for the one file, read by the one parser.

**Re-run state lives in the pilot register, not in the row** (DESIGN, correcting the previous round's draft,
which added `rerun` and `rerun_receipt` fields to these rows). All four truths are recorded results of plan-time
pilots, and the plan-time pilot ledger (N38a-10) already keeps one record per pilot in
`crates/frob-conformance/registry/pilots.toml` with its re-run owner and the derived status
`pilots::rerun_status(id) -> not-run | reproduced | differs`; it rejected a second file of re-run state because two
places would hold the status of one check. So each row's derivation file names its pilot record in its front
matter (`pilot = "PILOT-PT-nn"`, with the re-run owner), and N38a-12's eligibility computation reads
`pilots::rerun_status` of that record through the register: a FACT row whose pilot is not `reproduced` is
`obligation: rerun` (and, while the pilot `differs`, also `flagged: unreconciled`) and is not counted by any gate.
That is N38a-12's rule for every FACT row: the re-run state is computed from the derivation front matter
(`pilot = "PILOT-…"` read through the register, or `reread = { receipt, session, date }` for a sibling source or a
publication), never stored in a row field, and a `[[ka]]` row carrying `rerun` or `rerun_receipt` is refused as
an unknown field. This bead's meta-test `rerun_gate` checks the rule on these four rows.

| Field | KA-033 | KA-034 | KA-035 | KA-036 |
|---|---|---|---|---|
| `section` | §9 D-17 | §9 D-18 | §9 D-19 | §9 D-19 |
| `fixture`, `same_as` | `stance-family`, KA-019 | `boundary-screen-65536`, KA-018 | `wheel-loop` (fixture 1's wheel-loop row of §24.10; one plant record with shaft damping, NEG-10), none | `docking-fixture`, KA-026 |
| `tier`, `sites` | G2, [`host`] | G1, all tiers | G1, [`host`] | G1, [`host`] |
| `label` | FACT: pilot PILOT-PT-01 (static stance family); re-run by BD-042 | FACT: pilot PILOT-PT-15 (boundary screen); re-run by BD-030 | FACT: pilot PILOT-PT-17 (wheel-loop feedforward); re-run by N38a-11 | FACT: pilot PILOT-PT-07 (docking, three gains); re-run by BD-041 |
| `due` | M0 | M0 | M0 | M0 (all four: the plan-time pilots are re-run and must reproduce at M0, GATE-GAUNT-08) |
| `derivation` | `docs/derivations/stance-family.md` | `docs/derivations/boundary-screen-65536.md` | `docs/derivations/wheel-loop.md` | `docs/derivations/docking-fixture.md` (the file of N10b-08) |

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
| KA-033 | D-17 | Generic comparator on the static-stance family: 400 N on four point feet at (±a, ±b) with a = 1/3 m; centre of mass anywhere in a rectangle of half-sizes 150 mm and 75 mm; each foot commanding any force in [0, 160] N; minimize the half-width b | A lifted robust linear program and the specialized projection each reach the optimum in one solve and return the same optimum, half-width 3/14 m | exact (rational optimum; solve counts) | G2 · host | Printed `none stated`. Set here: the lifted program with its robust load set replaced by the centre of the rectangle (the nominal, non-robust program), in which four 100 N forces balance the load for every b > 0, so it attains no positive optimum; the row must flag it as disagreeing with the projection. (Dropping one vertex is not a control: by the symmetry of the fixture each of the four corners alone forces b ≥ 3/14 m, so the answer would not change.) | BD-042 |
| KA-034 | D-18 | Strict decisions on a screen of 65,536 boundary cases of one inequality | An unguarded `f32` screen produces 452 false admissions and 23,438 false exclusions; a guarded or exact-integer kernel produces none | exact (counts) | G1 · all tiers | The unguarded `f32` comparison, which must show the errors | BD-030 |
| KA-035 | D-19 | One winding feed-forward term in a sampled wheel controller (the wheel-loop fixture: the feed-forward uses the resistance times the reference current in one variant and the resistance times the measured current in the other) | The one-tick spectral radius moves from 1.48 to 0.92 and 24 of 156 task labels change | Printed `to set` for the radii. Set here: ±0.005 on the two-decimal values, and ±5·10⁻⁵ on the four-decimal values 1.4820 and 0.9207 the pilot recorded; exact for the label count | G1 · host | The assumption left as prose and not as a field of the claim: a claim whose feed-forward law is not a typed field must be rejected by the claim-shape check | N38a-11 (row 17) |
| KA-036 | D-19 | One extra hidden gain admitted on the docking fixture (a cart must reach position 1 at rest in three one-second ticks; acceleration g·u with hidden gain g ∈ {1, 2}; the third gain 33/32 is added to the support) | Every feasible label on the pilot's grid reverses (34,590 positive labels per grid in the pilot); exact docking becomes unsatisfiable and the worst-case error is at least 1/65 m | exact (verdict and count) | G1 · host | Inclusions carried across the enlarged world set | BD-041 |

**Derivation files.** `stance-family.md` (shared with the stance rows of other chapters: the moment equations
give f₊₊ − f₋₋ = 90 + 15/b at the positive load corner, so b = 3/14 m with forces (160, 130, 110, 0) N);
`boundary-screen-65536.md` (the inequality, the row generator and the three kernel classes);
`wheel-loop.md` (shared with the wheel-loop rows of the Arena and Atelier chapters: the one-tick map of the
sampled loop with the hold modelled exactly, and why the instantaneous-current model gives the same radius,
2.6917, for both laws and is therefore the wrong model; KA-035's tolerance is `printed-digits`, half a unit in
the last printed digit: ±0.005 on 1.48 and 0.92, ±5·10⁻⁵ on the pilot's 1.4820 and 0.9207);
`docking-fixture.md` (owned by N10b-08 and shared with the docking rows: with first command p the remaining commands are
(1/g − 2p, p − 1/g); a robust policy exists exactly when the sensor error is below (U + ½)/4 with the packet
on time).

**One fixture identity per physical case.** The stance and docking fixtures appear again in the constructions
and in the mission chapter; the registry records the sharing so that constants exist once.

**KA-036 on two grids.** The row's count is the pilot's, on the pilot's grid (PILOT-PT-07: 34,590 positive labels
per grid, 34,590 of 34,590 reversing). On the docking fixture's 561-cell grid the exact three-gain enumeration of
N10b-08 reverses 327 of the 344 two-gain admissions and keeps 17 admitted: the cells with ε = 0 (j = 0), where
the three exact readings p, 33p/32 and 2p identify the gain. "Every feasible label reverses" is therefore the
statement for labels with a positive read error; the registry keeps the printed truth, and the derivation file
`docking-fixture.md` (N10b-08's) records both counts and the 17 exceptions, so that a test on the 561-cell grid
expects 327 and 17, not 344.

## Acceptance criteria

- [ ] Four `[[ka]]` rows exist in `known_answers.toml` with every field of the N38a-12 schema filled and
      validate under its loader; none has tolerance `to-set`; every `control.text` names a mutation or planted
      failure, and after N38b-05's seeding every row's `control.nc` is non-empty.
- [ ] Four derivation files exist, each listing the rows it serves in its front matter with the test that checks
      each, its carrier bead and its pilot record (`PILOT-PT-01`, `-15`, `-17`, `-07`), and stating the value with
      its digits and the error budget.
- [ ] No row carries a re-run field of its own; while `pilots::rerun_status` of its pilot record is `not-run` or
      `differs`, each row is reported `obligation: rerun` and is counted by no gate; once the re-run owner
      (BD-042, BD-030, N38a-11, BD-041) records `reproduced`, the row's eligibility follows the five rules alone.
- [ ] The registry schema test passes, and a planted row with a reused identifier, an empty control text, a
      `to-set` tolerance, a `same_as` naming a row with a different `fixture`, or a derivation file naming no pilot
      record, is rejected or reported ineligible.
- [ ] Each row is selectable by its number through the tier targets of N38a-02
      (`FROB_TEST_SEED=<seed> cargo test -p frob-conformance --test g2_parity -- KA-033`; `--test
      g1_known_answers -- KA-034`, `-- KA-035`, `-- KA-036`); until its carrier binds a function it is reported
      `unbound` or `obligation`, never `pass`. Running the rows against their truths with the controls caught is
      the acceptance of the carriers, which bind the entries `ka:<id>@<site>` (BD-042, BD-030, N38a-11, BD-041).

## Test plan

### Unit tests

`crates/frob-conformance/tests/ka_doctrine.rs` (host, exact rationals; the real registry and scratch copies, no
mocks):
- `ka_records_complete`: the four rows parse through the test kit's parser; every N38a-12 field non-empty
  (`printed` and `slug` may be empty, and so may KA-035's `same_as`, its fixture's first row); identifiers unique
  and in range; `same_as` of KA-033, KA-034 and KA-036 names an earlier row with the same `fixture`.
- `ka033_hand_optimum` (exact rationals, host): at the load corner (60, 30) N·m the moment equations give
  f₊₊ − f₋₋ = 90 + 15/b; with f₊₊ ≤ 160 and f₋₋ ≥ 0 the least half-width is 3/14 m, attained by
  (160, 130, 110, 0) N; each of the four corners alone gives 3/14 m (the symmetry that makes a dropped vertex
  useless as a control); with the load at the centre, four 100 N forces balance for every b > 0 (the nominal
  control attains no positive optimum). The solver-level check (both programs, one solve each) is BD-042's.
- `ka034_counts_recorded`: the row's truth holds 65,536 rows, 452 and 23,438 unguarded, 0 and 0 guarded and
  exact, equal to the expected values of pilot record PILOT-PT-15; the kernel run itself is BD-030's (host) and
  BD-182's (devices).
- `ka035_values_recorded`: 1.48 and 0.92 equal 1.4820 and 0.9207 to the printed digits; 24 and 156 equal the
  expected values of PILOT-PT-17; the plant computation (one plant with shaft damping, NEG-10) is N38a-11's.
- `ka036_hand_bound` (exact rationals): with final positions c and 33c/32, max(|c − 1|, |33c/32 − 1|) is
  least at c = 64/65 with value 1/65; 34,590 equals the expected value of PILOT-PT-07; the grid enumeration is
  BD-041's.
- `rerun_gate`: with the pilot register's record of each row in status `registered`, each row is reported
  `obligation: rerun` and is not counted; on a scratch copy of `pilots.toml` with the record `reproduced` the row
  becomes eligible; a derivation file that names no pilot record is rejected.
- `fixture_names_shared`: `same_as` of KA-033, KA-034 and KA-036 names KA-019, KA-018 and KA-026, whose `fixture`
  is `stance-family`, `boundary-screen-65536` and `docking-fixture` (N00-KA), and the derivation path is equal.

### End-to-end script and logging

`scripts/e2e/ka_doctrine.sh` (header: `bead=N09-KA milestone=M0 labels=gauntlet,registry
gates=GATE-GAUNT-02 plants=no-pilot-record,to-set-back,pilot-not-reproduced`). Sources the STD-E2E helper. Runs
`cargo test -p frob-conformance --test ka_doctrine` (the unit tests above), N38a-12's loader test, and then the
four rows by number through their tier targets (`FROB_TEST_SEED=<seed> cargo test -p frob-conformance --test
g2_parity -- KA-033`, `--test g1_known_answers -- KA-034`, `-- KA-035`, `-- KA-036`), logging each row's outcome
per site as the runner reports it (`pass`, `obligation: rerun`, `unbound`, `skipped: <site unavailable>`); a
device site of KA-034 is `skipped` until BD-182 lands, never passed. Then, on scratch copies of the registry and
of `pilots.toml` under `target/e2e-scratch/ka_doctrine/`, it plants a derivation file with no pilot record,
KA-035 set back to `to-set`, and KA-034's pilot record set to `differs`, and asserts that the first is rejected,
the second reported `obligation: tolerance` and the third `obligation: rerun` with `flagged: unreconciled`. Log
`target/e2e-logs/ka_doctrine/<utc>.ndjson` with a human transcript: every line with `ts`, `script`, `bead`,
`step`, `event`, the exact command line, exit code and wall time; one `assert` line per row with its id, tier,
sites, eligibility, pilot record and status, truth and tolerance; the SHA-256 of `known_answers.toml` and of
`pilots.toml`; on failure the first row and field that differed and the scratch path that reproduces it. Final
`summary` line with rows eligible, rows pending re-run, plants caught, and the gates touched (GATE-GAUNT-02;
GATE-GAUNT-08 is recorded by the re-run owners, not here).

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
