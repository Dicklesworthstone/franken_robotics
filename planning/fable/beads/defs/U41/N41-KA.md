---
key: "N41-KA"
title: "registry: known answers of the delivery registers (risks, open questions, tracker, first session, done)"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "delivery", "gauntlet", "frob-conformance", "solid", "registry", "test", "review-notes-open"]
parent: "EPIC-M0"
deps: ["BD-025", "N00-KA", "N38a-12", "N38a-40", "STD-E2E", "STD-TEST"]
needs: []
related: ["N41-PM", "N41-REF", "N41-CX", "NJLx-02", "N38a-13", "N39-KA", "N24-KA", "N20-KA"]
gates: ["GATE-WS-07", "GATE-WS-08", "GATE-GAUNT-02"]
plan_refs: ["§41", "§42", "§45.10", "§45.11", "§46", "§47", "Appendix M"]
---
## Background

A **known answer** is a value that can be derived by hand from stated inputs and that a tool must reproduce. The
program's printed known-answer suite is organized by subsystem chapter and has **no section** for the five
delivery chapters this unit carries: risks and the pre-mortem, open questions, the bead graph, the first
implementation session, and the definition of done with the release rubric. Those chapters define no physics.
They do state exact facts: how many risks there are and how severe, how many questions and who closed which, the
shape of the seed graph, the counts a first-session program must print, the number of clauses, rules and rubric
rows. The tools this unit builds (register loaders, the exit-time reviews, the tracker check, the release
checker) are correct only if they reproduce those facts, and a change to a register that alters a count should be
a visible, deliberate act.

Every value below is a FACT about the plan's registers as compiled (counted from its tables while these beads
were drafted), exact arithmetic, or a DESIGN choice of this unit's beads where marked. None is a measurement of
the system. Two gates of this unit are themselves typed "known answer" by the plan: GATE-WS-07 (the four planted
instances) and GATE-WS-08 (the four planted release cells); their rows are here too.

## Scope

In: the rows; their home in the conformance registry; the test that pins each. Out: the planted defects that
pair with them (N41-CX); the reasons a tool prints when a row is violated (N41-REF); physics known answers (the
chapter registries).

## Technical approach

**Records.** One `[[ka]]` record per row, appended to the suite's one known-answer file
`crates/frob-conformance/registry/known_answers.toml` in the row schema of N38a-12; this unit keeps no file of its
own (a `known_answers/delivery.toml`, or any other `known_answers*.toml`, fails the loader with
`ka-second-file`). DESIGN, correcting this bead's earlier draft, which kept `delivery.toml` with private fields
(`pinned_by`, `bead`) and a `pending` state. The columns map as: Row → `slug` (the K41 tag, kept after the number is issued) and `id` (below); Case → `case`;
Truth → `truth` (an integer, a rational, a set or a verdict); `tolerance = { kind = "exact", reason = "rule 1 of
N38b-27" }` on every row except K41-10 and K41-33, whose decimals take `printed-digits` (rule 2); `tier = "G0"`
(a test of the verification program itself) and `sites = ["host"]`; Control → `control.text` (the planted case
of N41-CX named, with its text), `control.nc` filled by N38b-05's seeding from N41-CX's records; "Pinned by" →
the front matter of the row's derivation file (for each row: the pinning bead and its test symbol), and the
runner binding `ka:<id>@host` of N38a-13, supplied by the pinning bead named first; plus `section`, `owner`,
`fixture`, `same_as`, `derivation`, `label`, `due` (below) and `printed` (empty on all 40: no recomputation
disagrees with a plan value). The row stores no status; the loader computes eligibility.

**Numbers.** The printed suite has no section for these chapters, so the rows have no printed numbers: this bead
files one request in the allocation ledger of N38a-40 (`[[block]] registry = "ka", count = 40, owner = "N41-KA",
section = "§41, §42, §45.10, §45.11, §46, §47"`) and, in one commit at M0, maps the tags K41-01 to K41-40 to the
block's numbers in table order (rule 5 of N38a-40). Each tag is kept in its row's `slug`; after the mapping a tag
cited in place of its number in any other field, a test name or a derivation file fails with
`provisional-tag-past-due` (as N40-KA and N16-KA do). This bead prints no number before the block is issued.

**Sections and owners.** `section`: K41-01 §41.1; K41-02 §41.2; K41-03 and K41-04 §41.3, §41.5; K41-05 to K41-07
§41.3, §41.4; K41-08 to K41-10 §41.5, §45.12; K41-11 to K41-14 §42.3; K41-15 and K41-16 §42.2; K41-17 §42.4; K41-18
§42.1; K41-19 to K41-25 §45.10; K41-26 §45.11; K41-27 to K41-33 §46; K41-34 §47.1; K41-35 §47.2; K41-36 §47.3;
K41-37 and K41-38 §47.4; K41-39 §47.4, §45.12; K41-40 §47.1, §47.3. `owner`: GATE-WS-07 on K41-08, GATE-WS-08 on
K41-39, empty on the others (no gate or obligation names them).

**Fixtures, derivation files and shared rows.** One fixture and one derivation file per register (DESIGN names):
`delivery-premortem` (K41-01, K41-02), `delivery-risk-register` (K41-03 to K41-07), `ws07-planted` (K41-08,
K41-09), `rare-tail-1pct` (K41-10), `delivery-open-questions` (K41-11 to K41-18), `delivery-seed-graph` (K41-19
to K41-26), `delivery-first-session` (K41-27 to K41-30), `delivery-done` (K41-34 to K41-40); each at
`docs/derivations/<fixture>.md`, listing in its front matter the rows it serves, the register table each count
is read from, the counting rule, and for each row its pinning bead and test. Four rows restate the printed
constants of an earlier chapter's case and share that case's fixture (the appendix legend: "the registry holds
the complete map"; N38a-12: one fixture identity per physical case): K41-31 (the 561-cell docking grid,
admitted when ε < (U + 1/2)/4) and K41-32 (the witness sequences (1/4, 1/2, −3/4) and (1/4, 0, −1/4), the
late packet and the third gain) take `docking-fixture` with `same_as = "KA-024"` and `"KA-023"`, rows of N00-KA
(a dependency) whose cases they enumerate, and append themselves to `docs/derivations/docking-fixture.md`
(N10b-08's file); K41-33 (fixture 1's masses, tread caps, lip levers and friction demands, KA-602, KA-603,
KA-605 and the friction row of §24.10) takes `cart-sill-dock` and its derivation directory, and since N24-KA's
rows land at M1, after this bead, its `same_as = "KA-602"` is set in N24-KA's commit that adds KA-602, so that
the loader's rule that `same_as` names an existing earlier row holds at every commit;
K41-10 shares `rare-tail-1pct` with TF-18 of the ten-fold chapter (N39-KA): both are above KA-835 and numbered
from two blocks, so the row that receives the higher number carries `same_as` naming the lower (DESIGN, the
rule N39-KA states; the derivation file is written by whichever bead lands first, this one at M0, and the other
appends). The shared-fixture count of the registry changes by these rows and is reported row by row (N38a-12).

**Labels.** FACT, with the plan's register tables as source and the note "re-counted by the pinning test", for
the register counts (K41-01 to K41-06, K41-11 to K41-16, K41-19 to K41-25, K41-27, K41-34 to K41-36); FACT from
outside the program, "to be re-read at the pinned source before a gate cites it", for K41-09 (the sibling's
planted linear-phase sampler, 0.6655) and K41-30 (the published 410 s and 45 s); DESIGN for the defined sets,
the rules and the arithmetic (K41-07, K41-08, K41-10, K41-17, K41-18, K41-26, K41-28, K41-29, K41-31 to K41-33,
K41-37 to K41-40).

**Due (`due`: the milestone at which every part of the row is bound).** M0, 27 rows: K41-01 to K41-05, K41-07,
K41-11 to K41-16, K41-18 to K41-25, K41-27 to K41-33. M1, 8 rows: K41-26 (N41-17), K41-34 to K41-40 (N41-26 to
N41-30). M2, 2 rows: K41-09 (N41-38), K41-17 (its blocking-default part, N41-15). M3, 2 rows: K41-08 (its suite
part, N41-31), K41-10 (N41-31). M5, 1 row: K41-06 (N41-37, the last plants). DESIGN reason, as in N40-KA:
GATE-GAUNT-02 counts a row as passing only when its whole truth is checked, so a row is not due before its last
part has a bound test; the earlier parts are asserted by their beads' own tests as those land (K41-06 at each
exit from M0, K41-08's build rows at M0 and sampler row at M2, K41-17's naming check at M0). The earlier draft
dated each row by its first bead (30, 8, 1, 1) and asserted the later parts through a `pending` state that the
schema does not have.

**Rows.**

| Row | Case | Truth | Control | Pinned by |
|---|---|---|---|---|
| K41-01 | Pre-mortem stories | 27: stories 1 to 15 carried, 16 to 27 new (five about the thesis, 16 to 20; three about the evidence, 22 to 24; four about the program, 21 and 25 to 27) | X41-48 | N41-PM |
| K41-02 | Reasons every gate could pass while the claim fails | 6, ranked by how early the signal is available; reason 1 is readable at the M0 exit | X41-48 | N41-PM |
| K41-03 | Size of the risk register | 59 rows with no gap: RISK-01 to RISK-55 as printed, RISK-56 to RISK-59 assigned to the four late rows | X41-13 | N41-01 |
| K41-04 | Severity tally | 28 High, 29 Medium, 2 Low (printed rows 25, 28, 2; late rows 3 High, 1 Medium); by part of the register 7/6/2, 10/5/0, 4/11/0, 7/7/0 | Supplied here (DESIGN; the earlier draft had none, which N38a-12 reports as `obligation: control`): a tally over the printed rows only, which gives 25 High, 28 Medium, 2 Low and drops the four late rows; N41-CX's entry X41-52, plant `N41-01:severity-tally-printed-rows-only` | N41-01, N41-RISK1 to N41-RISK4 |
| K41-05 | Rows first readable per exit | M0 21, M1 9, M2 11, M3 15, M4 1, M5 2, later 0; reviewed at the M0, M2 and M3 exits: 21, 41, 56 | X41-12 | N41-01, N41-04 |
| K41-06 | Primary plants runnable per exit | M0 20, M1 30, M2 39, M3 54, M4 57, M5 59; further observations due 1, 5, 3, 1 at M1 to M4; the full sweep fires 69 | X41-15, X41-17 | N41-32 to N41-37 |
| K41-07 | Outcomes of a fired signal | Exactly three: a kill gate decided; an ADR that changes a mechanism or a scope; a written reason why neither applies | X41-11 | N41-01 |
| K41-08 | GATE-WS-07 | Four planted instances, each raising its signal in 100 of 100 runs; parts at M0 (two build rows), M2 (sampler row), M3 (suite row); zero alarms on the clean graph in 100 runs per mode | X41-01, X41-06 | N41-02, N41-38, N41-31 |
| K41-09 | Sampler battery band | Pairwise sign-agreement frequency inside ½ ± 0.005; the planted linear-phase sampler measures 0.6655 | X41-07 | N41-38 |
| K41-10 | Rare-tail arm | Deadline 1,000 s; the rival always 20 s; the candidate 1 s on 99% of families and 999 s on 1%. Candidate mean 0.99·1 + 0.01·999 = 10.98 s, population ratio 20 ÷ 10.98 = 1.8215. A 24-family suite misses the tail with probability 0.99²⁴ = 0.78568, and a percentile bootstrap on such a sample reports 20 | X41-08 | N41-31 |
| K41-11 | Size of the open-question register | 161 = 147 + 14; 147 = 141 raised by the subsystem chapters + 6 at program level; 14 adopted (OPEN-GAUNT-2 to -8, OPEN-TENFOLD-3 to -9); OPEN-TENFOLD-1 and -2 are aliases of OPEN-PROG-2 | X41-21 | N41-03 |
| K41-12 | Rows by subsystem | Lemma 6, Marrow 3, Skeleton 4, Sinew 4, Kinesis 6, Phylum 5, Traction 5, Habitat 8, Sensorium 5, Spark 6, Reflex with gaits and shields 8, Cortex 6, Arena 5, Augur 11, Nemesis 5, Autopsy 4, Atelier 6, Chronicle 5, Ground 5, Foundry 6, Tether 5, suite consumption 6, workspace 5, numerics 4, performance 4, safety 4 (sum 141); program 6. By part of the register: 17, 16, 19, 19, 20, 27, 23, 20 | X41-21 | N41-OPEN1 to N41-OPEN8 |
| K41-13 | Closed and decided rows | Four: OPEN-SKEL-1 (closed by ADR-SKEL-02), OPEN-AUGUR-6 (settled by ADR-WS-02), OPEN-SUITE-2 and OPEN-TETHER-1 (decided by the owner on 2026-11-22) | X41-21 | N41-03, N41-14 |
| K41-14 | Defaults confirmed by the gate register | Four: OPEN-NUM-3 (cell side 8 m, offset in [−4, 4) m), OPEN-PERF-1 (tolerance 0.67; KILL-PARITY below 0.33), OPEN-REFLEX-2 (the pair 0.7 and 0.5), OPEN-ATEL-4 (two verbs, one engine) | X41-20 | N41-03 |
| K41-15 | The owner's table | 22 rows naming 32 questions: 17 single rows and five grouped rows of 2, 3, 4, 2 and 4. One is a recorded decision (OPEN-SUITE-2). Every one of the 32 resolves to a bead labelled `owner-decision`: 15 in this unit's packets, 17 in chapter units | X41-19 | This bead's `owner_table_22_rows_32_questions` (the arithmetic, on the transcribed table); N41-OPEN8's `owner_table_questions_resolve` (the 32 on the real rows and the tracker export) |
| K41-16 | The six that matter most, in due order | OPEN-SPARK-1 and OPEN-SPARK-2 at M0; OPEN-PROG-4 at M1; OPEN-PROG-2 at M2; OPEN-CORTEX-1 at M3; OPEN-PROG-1 before the pins are frozen for M7 | X41-19 | N41-03 |
| K41-17 | The all-defaults product | Seven limits (CPU-first with exploratory GPU results; no family policy; cheap rungs that only discard; generic cards and assumed norms; no Reality claim; a ratio labelled `self-adjudicated`; a private repository); exits M0 to M5 and M7 still reachable; one default can hold an exit (OPEN-TENFOLD-4 at M3, while no stratum of the partial gate has a record; with the strata that fit the default class recorded and the others `not run` with the reason, M3 exits under the default) | X41-24 | N41-03, N40-35 (the naming check), N41-15 |
| K41-18 | Asked-at rule | One milestone before the deadline; a deadline of M0 is asked at M0 | X41-19 | N41-03 |
| K41-19 | Seed graph size | 442 beads = 406 tasks + 27 epics + 9 exit reviews; by milestone 44, 103, 77, 95, 59, 34, 15, 7, 8; 1,091 dependency edges; one root, BD-001; every dependency points to a lower number | X41-25 | N41-16 |
| K41-20 | Exit chain | BD-044 → BD-147 → BD-224 → BD-319 → BD-378 → BD-412 → BD-427 → BD-434 → BD-442 (the cumulative milestone sizes); BD-442 has 441 ancestors | X41-25 | N41-16 |
| K41-21 | Beads with no consumer but exit reviews | 44: by milestone 3, 1, 0, 5, 10, 5, 12, 1, 7 | X41-49 | N41-16 |
| K41-22 | Longest chains | 37 beads with exit reviews, BD-001 to BD-442; 33 without, ending BD-317 → BD-428 → BD-430 → BD-432 → BD-433 | X41-25 | N41-16 |
| K41-23 | Widest beads | BD-312 and BD-376, 24 dependencies each; exit reviews between 2 and 16 | Supplied here (DESIGN; the earlier draft had none): a width computed on the reversed edge direction (dependents instead of dependencies), which names other beads; N41-CX's entry X41-53, plant `N41-16:widest-by-dependents` | N41-16 |
| K41-24 | Gate coverage of the seed | 421 identifiers = 278 convergence + 111 kill + 32 admission, each cited by a bead, with the five sibling admission gates and GATE-GAUNT-01; 118 beads cite none | X41-28 | N41-16 |
| K41-25 | Gates that close in parts | 20 table rows covering 32 gate identifiers, each with a bead per part (the table in N41-16) | X41-28 | N41-16 |
| K41-26 | Close reasons and the burst rule | A closed set of six (`pass`, `recorded`, `fallback`, `superseded`, `duplicate`, `withdrawn`; the plan names `pass`, `fallback` and `superseded`, the other three are DESIGN of N41-17's rule 2); more than five closes by one session in 60 s are listed (DESIGN); the recorded incident is 87 closes in 12 s with one reason | X41-27 | N41-17 |
| K41-27 | First session, shape | Eight steps in order (1 to 5 and 8 carried, 6 and 7 new); six rows of "what each observation decides" over seven observations; it ends with four measurements, two sets of exact known answers with a control caught, and a two-second rollout | X41-33 | N41-18 to N41-25 |
| K41-28 | First session, envelope | 16 GiB, 8 cores, one hour, one device; every build through the offload shim | X41-32 | N41-18 |
| K41-29 | First session, kernel steps | 10⁶ quaternions through a twenty-line Cayley step, bit for bit against a Rust loop on Metal and on both CUDA cards; the square-root comparison over all 2³² inputs | X41-50 | N41-21, N41-22 |
| K41-30 | First session, incumbent step | Four phases on one RTX 4090; disclosure block of six fields (revision, driver, card, host load, cache warmth, repetition count); published context about 410 s to threshold, about 45 s of it compilation (a published FACT, not a value to reproduce) | X41-37 | N41-23 |
| K41-31 | First session, docking family | Grid 17 actuator bounds U at spacing 1/32 on [1/2, 1] by 33 error levels ε at spacing 1/64: 561 cells, admitted when ε < (U + 1/2)/4. 344 admitted, 217 excluded, 9 excluded cells exactly on the boundary. Check: writing U = 1/2 + k/32 and ε = j/64, a cell is admitted when 2j < 32 + k; even k gives 16 + k/2 cells (sum 180) and one boundary cell each (9), odd k gives 17 + (k − 1)/2 cells (sum 164) | X41-34 | N41-24 |
| K41-32 | First session, docking witnesses | Sequences (1/4, 1/2, −3/4) and (1/4, 0, −1/4) integrate exactly; at U = 3/4 and ε = 5/16 both worlds can read 15/16; every late-packet cell is excluded; the three-gain amendment's value is 1/65 m | X41-34 | N41-24 |
| K41-33 | First session, analytic rows of the cart, sill and dock fixture | Total masses 8.000 kg and 8.225 kg; tread force caps 40 N and 32 N; lip levers 36.000 mm and 40.694 mm at a 12 mm sill; friction demands 0.7500 and 0.6459 at front lift-off (tolerance: the printed digits) | X41-51 | N41-24 |
| K41-34 | Definition of done | 16 clauses; three things not required (no Moonshot item; no particular ratio; no calibration against hardware) | X41-38, X41-46 | N41-26 |
| K41-35 | Rules for any release | Nine; "implemented" is eight conditions per capability; a demo's commands finish in under 60 s on the CPU tier | X41-40 to X41-43 | N41-27 |
| K41-36 | The rubric | Nine rows, M0 to M8, cumulative; a withdrawn capability stays visible as withdrawn with its ADR | X41-44, X41-45 | N41-28 |
| K41-37 | Kinds of evidence, in order | Four: authorable at once (from M0); pinned external comparison (from the M0 phase ledger); operator or dataset (when the import lane has an admitted dataset); native device (when that device ran it, with no fallback on any claimed stage) | X41-02, X41-03 | N41-29 |
| K41-38 | Complete dispositions | Four beside `settled`: `tied`, `infeasible`, `incomparable`, `inconclusive` | X41-05 | N41-29 |
| K41-39 | GATE-WS-08 | Four planted cells, each rejected; the corrected manifest passes; recorded at M1 and re-run at every release | X41-02 to X41-05 | N41-29, N41-30 |
| K41-40 | Wording thresholds repeated by the done clauses | "Ten-fold" only where the decision rule confirms it; a ratio between 3 and 10 is described by its value; under 3 by what a reader can audit; referee coverage under 80% names an adjudicator | X41-38 | N41-26, N41-28 |

**Loading and binding.** Each row is loaded by the test kit's one parser (STD-TEST) and validated by N38a-12's
loader; it is bound (`ka:<id>@host`, N38a-13) by the named test in the bead of its last column. A row due at or
before the selected milestone with no binding is `unbound` and fails GATE-GAUNT-02; a row due later is reported
`not yet due: <bead>` and blocks nothing; a binding whose test no longer exists, or a row whose control (N41-CX)
is not registered, fails the registry check.

## Acceptance criteria

- [ ] `crates/frob-conformance/registry/known_answers.toml` holds the forty rows with every field of N38a-12's
      schema, under the numbers of this bead's N38a-40 block mapped from K41-01 to K41-40 in table order, each
      tag in `slug`; no other known-answer file exists (`ka-second-file`), and `delivery.toml` is not created.
- [ ] Every row due by the current milestone is bound by its pinning test in the bead named, asserting the value
      above; at M0 that is 27 rows, and the other 13 are reported `not yet due` with their beads (8 at M1, 2 at
      M2, 2 at M3, 1 at M5).
- [ ] Every row has a control: K41-04 and K41-23 carry the controls supplied above, each cell naming its plant:
      the mutants `N41-01:severity-tally-printed-rows-only` and `N41-16:widest-by-dependents`, built by N41-01 and
      N41-16 under those features and registered by N41-CX as X41-52 and X41-53, so that N38b-05's step 3 appends
      the NC of the control seeded from each entry instead of allocating a second; a row whose control record does
      not exist is reported `obligation: control`, never `eligible`.
- [ ] K41-31 and K41-32 carry `same_as` KA-024 and KA-023 with `docking-fixture`; K41-33 carries `cart-sill-dock`
      (its link to KA-602 set when that row lands); K41-10 and TF-18 share `rare-tail-1pct` by the higher-number
      rule; the change in the shared-fixture count is reported row by row.
- [ ] Arithmetic rows are recomputed by the test and not copied: 28 + 29 + 2 = 59; 21 + 9 + 11 + 15 + 1 + 2 = 59;
      147 + 14 = 161 and 141 + 6 = 147; the 26 subsystem counts sum to 141; 406 + 27 + 9 = 442 and the nine
      milestone sizes sum to 442; 278 + 111 + 32 = 421; 17 × 33 = 561 and 344 + 217 = 561; 20 ÷ 10.98 = 1.8215 and
      0.99²⁴ = 0.78568 to the printed digits.
- [ ] A row whose value is changed in the registry without the corresponding register changing fails its pin.
- [ ] Rows K41-08 and K41-39 agree with the records of GATE-WS-07 and GATE-WS-08 in the gate register.

## Test plan

### Unit tests
`FROB_TEST_SEED=0 cargo test -p frob-conformance --test known_answers_delivery`: `forty_rows_load` (every
N38a-12 field present; `slug` K41-01 to K41-40; tolerance reasons naming rule 1 or 2); `every_row_bound` (rows
due by the selected milestone); `due_counts_27_8_2_2_1`; `unbound_past_due_fails` (K41-09 with no binding on a
scratch M2 exit is `unbound`; the same row at M1 is `not yet due`); `every_row_control_registered` (a scratch
copy with K41-04's control removed is `obligation: control`); `tags_mapped_in_table_order` (once the block is
issued; a planted test name citing K41-19 after the mapping fails `provisional-tag-past-due`, the tag in `slug`
does not); `shared_fixtures` (K41-31 → KA-024, K41-32 → KA-023 on `docking-fixture`; K41-33 on
`cart-sill-dock`; the rare-tail rule applied to a scratch ledger in each order); `no_private_file` (a planted
`registry/known_answers/delivery.toml` fails `ka-second-file`); `arithmetic_rows_recomputed` (the sums above, computed from the registers);
`docking_grid_count_by_parity` (even k: 180 admitted and 9 boundary; odd k: 164; total 344);
`rare_tail_numbers` (10.98, 1.8215, 0.78568, 20); `exit_chain_is_cumulative_sizes` (44, 147, 224, 319, 378, 412,
427, 434, 442); `owner_table_22_rows_32_questions`.

### End-to-end script and logging
`scripts/e2e/ka_delivery_registry.sh` (this bead, M0; sources the shared helper of STD-E2E; `--milestone <Mn>`,
default the current milestone) checks what this bead delivers: it runs the unit tests above, then the rows due
by the milestone with their controls through the G0 tier target (`FROB_TEST_SEED=0 FROB_MILESTONE=<Mn> cargo test
-p frob-conformance --features gauntlet-mutations --test g0_laws -- ch:41`, and likewise `ch:42`, `ch:45`, `ch:46`
and `ch:47`, the chapter selection of N38a-13), and the printout regeneration (`cargo test -p frob-conformance
--test known_answers_printout`); it logs one `assert` line per row with the number and slug, the fixture, the
truth, the value recomputed from the register file, the tolerance and its rule, the binding and its outcome
(`pass`, `fail`, `unbound`, `control-missed`, or `not yet due: <bead>`), and the control; a `summary` line gives
rows loaded, rows due, rows passing, rows not yet due by milestone, and the tags still unmapped. The rows are
also asserted inside the unit's end-to-end scripts: `registers_exit.sh` (K41-01 to
K41-18), `risk_plants.sh` (K41-06), `ws07_build_plants.sh` (K41-08, M0 part) and `ws07_late_plants.sh` (K41-08 at
M2 and M3, K41-09, K41-10), `tracker_hygiene.sh` (K41-19 to K41-25) and `bead_lifecycle.sh` (K41-26), the
first-session step scripts and `first_session_record.sh` (K41-27 to K41-33), `done_check.sh` and
`release_check.sh` (K41-34 to K41-40). Each logs, through the shared helper of STD-E2E, a `metric` line with the
row identifier, the observed value, the expected value, the unit where there is one, the tolerance and the
verdict; a mismatch also logs the first differing element and the path of the register it was read from.

### Known answers and negative controls
This bead is the known-answer registry. Negative controls: a row edited to a wrong value (its pin fails); a row
with its pin deleted (the registry check fails); the planted cases of N41-CX named in the control column of the
row each protects, X41-52 (K41-04) and X41-53 (K41-23) among them. Determinism: exact; identical on both platforms.

## Considerations

- Counts are FACTs about the registers as compiled. When a register changes by design (a new risk, a new
  question, a bead discovered during implementation) the row changes in the same commit; a count that changes
  without its row changing is the drift these rows exist to catch.
- The seed-graph rows (K41-19 to K41-25) describe the 442 seed beads, not the converted graph, which is larger.
  They are the self-test of the tracker check and are not required of the live graph.
- Rows K41-30 and K41-09 quote figures from outside the program (a published training time; a sibling's defect).
  They are context to be re-read at the pinned sources, not values the program must reproduce.
- The first-session rows repeat known answers owned by the mission and task chapters; if those chapters correct a
  value, the first-session program follows the chapter's registry. The `same_as` links of K41-31 to K41-33 make
  that dependency visible to the loader: a correction lands in the shared derivation file once.
- The K41-29 clause "the square-root comparison over all 2³² inputs" is the same sweep as KA-540's first clause
  (§21.24); it is a clause of a different case (the first session's kernel step), so the row keeps its own
  fixture and cites KA-540 in its derivation file.

## References

§41.1 to §41.5, §42.1 to §42.4, §45.10 to §45.12, §46, §47.1 to §47.4; GATE-WS-07, GATE-WS-08; Appendix M (the
printed suite, which has no section for these chapters; KA-023, KA-024, KA-602, the shared rows); N38a-12 (row
schema, one file), N38a-40 (the block), N38a-13 (bindings), N38b-27 (tolerance rules).
