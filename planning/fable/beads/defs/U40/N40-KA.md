---
key: "N40-KA"
title: "registry: known answers of the gate register and milestone layer (counts, tables and arithmetic to reproduce)"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "delivery", "gauntlet", "frob-conformance", "solid", "registry", "test"]
parent: "WP-01"
deps: ["BD-025", "STD-TEST"]
needs: ["Appendix M: the printed known-answer suite has no section for the gate register or the milestones; if these rows are to receive KA numbers, the unit that owns the suite's numbering assigns them (numbers are append-only)"]
related: ["N40-CX", "N40-01", "N40-02", "N40-03", "N40-04", "N40-05", "N40-06", "N40-07", "N40-08", "N40-09", "N40-10", "N40-11", "N40-30", "N40-31", "N40-32", "N40-35"]
gates: ["GATE-GAUNT-10", "GATE-GAUNT-02"]
plan_refs: ["§40", "§43", "§38.6", "Appendix M"]
---
## Background

A **known answer** is a value that can be derived by hand from stated inputs and that a tool must reproduce; it
is the cheapest strong test there is. The program's printed known-answer suite is organized by subsystem chapter
and has **no section for the gate register or the milestone chapter**: those two chapters define no physics and no
numerics. They do, however, state a number of exact facts (how many gates there are, how many parts are due at
each exit, which gates recur, what the planning estimates add up to), and the tooling this unit builds is correct
only if it reproduces them. This registry collects those facts as rows, so that each mechanism bead of the unit can
cite a row and so that a change to the register which alters a count is a visible, deliberate act.

Every value below is a FACT about the plan's register as compiled (counted mechanically from its tables while
these beads were drafted) or arithmetic on DESIGN planning estimates. None is a measurement of the system. The
ten-fold test's own known answers (ratios, critical counts, the pre-analysis numbers) belong to the ten-fold
chapter's registry; the program-gate beads N40-12 to N40-22 state inline the values they need.

## Scope

In: the rows; their home in the conformance registry; the test that pins each. Out: negative controls (N40-CX);
physics known answers (the chapter registries).

## Technical approach

**1. File.** `crates/frob-conformance/registry/known_answers/register.toml` (DESIGN path, beside the suite's
`known_answers.toml`): `id`, `case`, `truth`, `tolerance`, `tier`, `control`, `pinned_by` (test symbol), `bead`.
Tier of every row: G0 (a test of the verification program itself), host. Tolerance: exact unless stated.

**2. Rows.**

| Row | Case | Truth | Negative control | Pinned by (bead) |
|---|---|---|---|---|
| R-01 | Statuses of a gate part | Seven: `not-run`, `pass`, `fail`, `killed`, `blocked`, `no-claim`, `superseded` | RC-01 | N40-01 |
| R-02 | Claim kinds | Seven: Proof, Known answer, Parity, Fidelity, Design comparison, Reality, Performance; exactly one per gate | RC-07 | N40-01 |
| R-03 | Recurring gates of the compiled register | Four: GATE-GROUND-01, GATE-SAFE-01, GATE-GAUNT-01, GATE-TENFOLD-09 | RC-14 | N40-01 |
| R-04 | Write-time refusals | Ten reasons (pass without control; control not caught; self-certified; evidence unresolved; target as measurement; claim not one of seven; no-claim on a non-reality gate; blocked without question; killed without decision; duplicate or reused identifier) | RC-01 to RC-06 | N40-01 |
| R-05 | Rows per table | Admission 32 and sibling admission 5; Lemma 7; Marrow 5; Skeleton 8; Sinew 9; Kinesis and fidelity 15; Phylum 8; Traction 10; Habitat 14; Sensorium 14; Spark, numerics and performance 23; Reflex, gaits and shields 17; Cortex 17; Arena and qualification 12; Augur 24; Nemesis 8; Autopsy 8; Atelier 16; Chronicle 13; Ground 10; Foundry 12; Tether 14; safety 10; workspace 4; Gauntlet 1 (sum 279 convergence rows); ten-fold 13 | RC-09 | N40-02 |
| R-06 | Claims over the 279 convergence rows | Known answer 197; Design comparison 34; Performance 16; Fidelity 13; Parity 10; Proof 7; Reality 2 | RC-07 | N40-02 |
| R-07 | Gates with parts at more than one exit | 35 convergence rows; 3 admission rows (INT-FSIM-07, INT-FMCP-01, INT-FDGR-01) | RC-08 | N40-02 |
| R-08 | Sibling admission gates that never block | Five: INT-FTUI-01, INT-F3D-01, INT-FDGR-01, INT-FJAX-01, INT-FTX-01 | — (structural) | N40-02 |
| R-09 | Reserved identifier | INT-FSIM-12 is reserved and unused; a row with it is rejected | A row `INT-FSIM-12` (must be rejected) | N40-02 |
| R-10 | Kill-row modes and outcomes | Modes: `exit`, `pilot-before-funding`, `standing`, `rolling`. Outcomes: `not-fired`, `fired`, `not-reached`, `deferred`. Standing or rolling in the compiled register and the ten-fold test: KILL-FSIM (M0, then rolling), KILL-MCP (M0, re-read at M1), KILL-AUGUR-FALSECERT (from M1), KILL-AUGUR-REUSE (from M3), KILL-TENFOLD-CERT and KILL-TENFOLD-LEAK (from M3) | RC-10, RC-11 | N40-03 |
| R-11 | Parts due per exit, compiled tables, M0 to M8 | Admission 23, 4, 2, 6, 0, 4, 2, 0, 0; convergence 11, 59, 80, 77, 49, 25, 7, 0, 7; kill decisions 2, 4, 14, 28, 30, 18, 9, 0, 8 | RC-14 | N40-04 |
| R-12 | Sum of kill decisions | 113 for 112 gates (KILL-MCP at M0 and M1) | — | N40-04 |
| R-13 | Due sets of the recurring gates | GATE-GROUND-01 and GATE-SAFE-01 at M1 to M8; GATE-GAUNT-01 at M0 to M8; GATE-TENFOLD-09 at M1 to M8 | RC-14 | N40-04 |
| R-14 | The exit protocol | Five requirements; twelve steps; six dispositions; at most four active Moonshot items | RC-15 to RC-19 | N40-05 |
| R-15 | The milestone rows | Nine; names, critical-path flags and exit observables as in N40-06; M7 needs M4 and neither M5 nor M6 | RC-20 | N40-06 |
| R-16 | Planning arithmetic | Critical path M0, M1, M2, M3, M4, M7: 2 + 5 + 6 + 6 + 8 + 6 = 33 weeks; M0 + M1 = 7 weeks; schedule threshold 1.5 × 7 = 10.5 weeks | RC-37 | N40-06, N40-11 |
| R-17 | The firewall rows | Nine rows of needs, uses-if-admitted and if-missing, as in N40-07 | RC-23 | N40-07 |
| R-18 | Admission parts first needed per exit | 23, 4, 2, 6, 0, 4, 2, 0, 0, with the identifiers of N40-07 | RC-22 | N40-07 |
| R-19 | Shared fixtures | Four groups of sizes 6, 3, 2, 2 | RC-25 | N40-08 |
| R-20 | Related thresholds | Ten rows; numbers 1.25 and 1.6; 0.70 and 0.5; 2%; 10⁻³ and 10⁻⁴; 0.7 and 0.5; 1 and 6 GPU-minutes; 1.25 and 1.5; the audit bound 1 − δ^(1/m) (0.0950 at δ = 0.05, m = 30) | RC-26, RC-27 | N40-08 |
| R-21 | Recorded discrepancies | 25 resolved rows: 6 kill gates stated in two chapters; 4 admission gates stated twice; KILL-MCP stated once; 11 rows against the milestone skeleton; 3 found when the milestone lists were compared with the register | RC-28 to RC-32 | N40-09 |
| R-22 | Gates of later subsections | 76 convergence gates and 25 kill gates; totals 37 admission, 355 convergence, 13 ten-fold, 1 schedule, 144 kill: 550 identifiers | RC-33 | N40-10 |
| R-23 | Late parts and late kill decisions per exit, M0 to M8 | Parts 8, 12, 22, 21, 16, 3, 4, 0, 3 (89 parts; two gates with three parts, nine with two); kill decisions 0, 1, 4, 6, 9, 0, 4, 0, 2 (26 for 25 gates) | RC-43 | N40-10, N40-29 |
| R-24 | Schedule-gate branches | T = 7.0 → met; T = 10.5 → met; T = 11.5 with 3.0 qualifying weeks → not met; T = 13.0 with 1.0 → met with recorded slip; qualifying time stops when the upstream item lands or an interim posture applies; overlapping intervals count once | RC-36 | N40-11 |
| R-25 | Kill gates in the ledger | 144 = 13 + 35 + 17 + 28 + 19 + 7 + 25 | RC-38 | N40-30 |
| R-26 | Kill decisions by exit, all sources | Compiled 2, 4, 14, 28, 30, 18, 9, 0, 8; late 0, 1, 4, 6, 9, 0, 4, 0, 2; ten-fold rules at M1, M2, M3, M7 | RC-45 | N40-30 |
| R-27 | Bets | 37 rows; four with no kill gate (BET-08, BET-10, BET-12, BET-21); decisions due per milestone 2, 3, 7, 20, 14, 3, 1, 2, 1 | RC-46 to RC-48 | N40-31 |
| R-28 | Fallback exits | 20 rows; 21 rehearsals; under all defaults the program exits M0 to M5 and M7 | RC-49, RC-50 | N40-32 |
| R-29 | Owner input | The six questions that matter most fall due at M0, M0, M1, M2, M3 and before the M7 freeze; the all-defaults product has seven printed limits (exploratory GPU results; no family policy; cheap rungs that only discard; generic cards and assumed norms; no Reality claim; `self-adjudicated`; private) | RC-52 to RC-54 | N40-35 |

**3. Binding check.** A harness test (`register_known_answers`) asserts that every row has a `pinned_by` test that
exists and passes, and that each row with a named control is failed by that control (N40-CX). A row whose
mechanism has not landed is `pending: <bead>`; no row is counted while its tolerance is unset or its control is
"none stated" where one is possible (rows R-08 and R-12 are structural and state why they have none).

## Acceptance criteria

- [ ] The file holds rows R-01 to R-29 with the truths above.
- [ ] Every row is pinned by a named test in the bead of its last column, and that test passes.
- [ ] Every row with a control is failed by its control in the binding check.
- [ ] A change to `registries/gates.toml` that alters a count makes exactly the affected rows fail, naming them.
- [ ] Row identifiers are append-only: a diff that renumbers or reuses one fails CI.

## Test plan

### Unit tests

`cargo test -p frob-conformance register_known_answers`: `all_rows_pinned`; `all_controls_bite`;
`ids_append_only`; and one arithmetic test per derived row: `sum_279` (7+5+8+9+15+8+10+14+14+23+17+17+12+24+8+8+
16+13+10+12+14+10+4+1); `claims_sum_279` (197+34+16+13+10+7+2); `kills_sum_112` (13+35+45+19) and `144`
(112+7+25); `load_sums` (admission 41; convergence 315; kill 113); `late_sums` (89 and 26); `total_550`
(37+355+13+1+144); `weeks_33_7_10_5`.

### End-to-end script and logging

`scripts/e2e/register_known_answers.sh`: runs the binding check and prints one line per row with its truth, the
recomputed value and the pinning test's result; asserts 29 of 29. Logs: command, exit code, wall time, per row the
expected and recomputed values, the register digest, and on failure the first differing row.

### Known answers and negative controls

This bead is the registry of known answers for its unit. Its own control: a planted row whose truth is off by one
(for example 278 convergence rows) must make `sum_279` fail.

## Considerations

- These are facts about a document's tables, so they move when the register is amended. That is intended: the
  amendment that adds a gate updates the row in the same change, and the review sees both.
- An earlier count of 421 identifiers was taken before the later subsections were written; it is not a row.
- If the suite's owner assigns KA numbers, the `R-` identifiers stay as aliases; nothing is renumbered.

## References

Plan §40 (all tables), §43.1, §43.2, §43.12, §38.6. Appendix M (no section for these chapters). Gates
GATE-GAUNT-02, GATE-GAUNT-10.
