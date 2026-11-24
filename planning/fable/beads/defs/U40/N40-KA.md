---
key: "N40-KA"
title: "registry: known answers of the gate register and milestone layer (counts, tables and arithmetic to reproduce)"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "delivery", "gauntlet", "frob-conformance", "solid", "registry", "test"]
parent: "WP-01"
deps: ["BD-025", "N38a-12", "N38a-40", "STD-E2E", "STD-TEST"]
needs: []
related: ["N38a-13", "N40-CX", "N41-KA"]
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

**1. Registry record.** One `[[ka]]` per row, appended to the suite's one known-answer file
`crates/frob-conformance/registry/known_answers.toml` in the row schema of N38a-12 (no file of this unit's own; a
second file fails the loader with `ka-second-file`). The printed suite has no section for these chapters, so the
rows have no printed numbers: this bead files one request of count 32 in the allocation ledger of N38a-40 (owner
N40-KA, section "§40, §42.4, §43") and, in one commit at M0, maps the tags R-01 to R-32 to the block's numbers in
table order (rule 5 of N38a-40). The tags remain the citations used in this unit's bead texts; the allocator's
tag-to-number table joins the two, and each tag is kept in its row's `slug` field (N38a-12 keeps the provisional
tag there after the number is issued, so that a search for R-12 still finds its row). After that commit a tag
cited in place of its number, in any other field of a registry row, a test name or a derivation file, fails
with `provisional-tag-past-due` (rule 5 of N38a-40, which exempts only the value of a numbered row's own `slug`
field, N38a-12; the mechanics registry N16-KA keeps its tags the same way). Fields: `section` the plan subsection the fact is taken from, with `owner = "GATE-GAUNT-10"`; `fixture`
`gate-register` (no `same_as`); `case` and `truth` from the table below; `tolerance` exact, with `reason` "rule 1 of N38b-27: integers, sets and verdicts"; `tier` G0 (a test of
the verification program itself), `sites = ["host"]`; `control.text` the control column and `control.nc` the
control records that the named N40-CX rows become; `derivation` `docs/derivations/gate-register.md` (one file
listing the thirty-two numbers, the table each fact is counted from and the counting rule); `label` DESIGN for the
defined sets and the arithmetic (R-01 to R-04, R-10, R-14, R-16, R-24, R-32) and FACT, with the plan's register tables
as source, for the counts; `due` as below; `printed` empty; `slug` the R- tag. Where the control column states a
plant instead of naming an N40-CX row (R-09, R-31, R-32), `control.text` is that plant and N38b-05's seeding
issues its record (its step 3 appends an existing control instead when one has the same plant and target).

**Due milestones (from the pinning beads).** M0: R-01 to R-18, R-22, R-28 to R-30, R-32 (N40-03) (23 rows). M1:
R-21 (N40-09), R-23 (its kill-decision half is N40-29's), R-24 (N40-11), R-25, R-26 (N40-30), R-27 (N40-31) (6
rows). M2: R-19, R-20 (N40-08) (2 rows). M4: R-31 (1 row: N40-30's test counts all ten introduced rows only once
N38a-37 has entered its three at M4; the six re-read entries are asserted by N40-04's test from M0 and N40-30's
from M1). DESIGN reason, as for the controls of N40-CX: GATE-GAUNT-02 is recorded at every exit from M0, and a row
whose pinning bead lands at M2, or whose whole truth enters the register only at M4, has no bound test before.

**2. Rows.**

| Row | Case | Truth | Negative control | Pinned by (bead) |
|---|---|---|---|---|
| R-01 | Statuses of a gate part | Seven: `not-run`, `pass`, `fail`, `killed`, `blocked`, `no-claim`, `superseded` | RC-01 | N40-01 |
| R-02 | Claim kinds | Seven: Proof, Known answer, Parity, Fidelity, Design comparison, Reality, Performance; exactly one per gate | RC-07 | N40-01 |
| R-03 | Recurring gates of the compiled register | Four: GATE-GROUND-01, GATE-SAFE-01, GATE-GAUNT-01, GATE-TENFOLD-09 | RC-14 | N40-01 |
| R-04 | Write-time refusals | Ten reasons (pass without control; control not caught; self-certified; evidence unresolved; target as measurement; claim not one of seven; no-claim on a non-reality gate; blocked without question; killed without decision; duplicate or reused identifier) | RC-01 to RC-06 | N40-01 |
| R-05 | Rows per table | Admission 32 and sibling admission 5; Lemma 7; Marrow 5; Skeleton 8; Sinew 9; Kinesis and fidelity 15; Phylum 8; Traction 10; Habitat 14; Sensorium 14; Spark, numerics and performance 23; Reflex, gaits and shields 17; Cortex 17; Arena and qualification 12; Augur 24; Nemesis 8; Autopsy 8; Atelier 16; Chronicle 13; Ground 10; Foundry 12; Tether 14; safety 10; workspace 4; Gauntlet 1 (sum 279 convergence rows); ten-fold 13 | RC-09 | N40-02 |
| R-06 | Claims over the 279 convergence rows | Known answer 197; Design comparison 34; Performance 16; Fidelity 13; Parity 10; Proof 7; Reality 2 | RC-07 | N40-02 |
| R-07 | Gates with parts at more than one exit | 36 convergence rows (the 35 of the compiled tables and GATE-MARROW-01, whose parts are M0 and M1); 3 admission rows (INT-FSIM-07, INT-FMCP-01, INT-FDGR-01) | RC-08 | N40-02 |
| R-08 | Sibling admission gates that never block | Five: INT-FTUI-01, INT-F3D-01, INT-FDGR-01, INT-FJAX-01, INT-FTX-01 | RC-60 | N40-02 |
| R-09 | Reserved identifier | INT-FSIM-12 is reserved and unused; a row with it is rejected | A row `INT-FSIM-12` (must be rejected) | N40-02 |
| R-10 | Kill-row modes and outcomes | Modes: `exit`, `pilot-before-funding`, `standing`, `rolling`. Outcomes: `not-fired`, `fired`, `not-reached`, `deferred`. Standing or rolling in the compiled register and the ten-fold test: KILL-FSIM (M0, then rolling), KILL-MCP (M0, re-read at M1), KILL-AUGUR-FALSECERT (from M1), KILL-AUGUR-REUSE (from M3), KILL-TENFOLD-CERT and KILL-TENFOLD-LEAK (from M3) | RC-10, RC-11 | N40-03 |
| R-11 | Parts due per exit, compiled tables, M0 to M8 | Admission 23, 4, 2, 6, 0, 4, 2, 0, 0; convergence 11, 60, 80, 77, 49, 25, 7, 0, 7 (the plan's load table prints 59 at M1; GATE-MARROW-01's M1 part adds one, N40-02, N40-04); kill decisions 2, 4, 14, 28, 30, 18, 9, 0, 8 | RC-14 | N40-04 |
| R-12 | Sum of kill decisions | 113 for 112 gates (KILL-MCP at M0 and M1) | RC-61 | N40-04 |
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
| R-23 | Late parts and late kill decisions per exit, M0 to M8 | Parts 8, 12, 23, 21, 16, 3, 4, 0, 3 (90 parts over 76 gates; GATE-FID-09 and GATE-WS-07 have three parts each, ten gates have two, GATE-MARROW-06 among them with its M1 and M2 parts); kill decisions 0, 1, 4, 6, 9, 0, 4, 0, 2 (26 for 25 gates) | RC-43 | N40-10, N40-29 |
| R-24 | Schedule-gate branches | T = 7.0 → met; T = 10.5 → met; T = 11.5 with 3.0 qualifying weeks → not met; T = 13.0 with 1.0 → met with recorded slip; qualifying time stops when the upstream item lands or an interim posture applies; overlapping intervals count once | RC-36 | N40-11 |
| R-25 | Kill gates in the ledger | 144 = 13 + 35 + 17 + 28 + 19 + 7 + 25 | RC-38 | N40-30 |
| R-26 | Kill decisions by exit, all sources | Compiled 2, 4, 14, 28, 30, 18, 9, 0, 8; late 0, 1, 4, 6, 9, 0, 4, 0, 2; ten-fold rules at M1, M2, M3, M7 | RC-45 | N40-30 |
| R-27 | Bets | 37 rows; four with no kill gate (BET-08, BET-10, BET-12, BET-21); decisions due per milestone 2, 3, 7, 20, 14, 3, 1, 2, 1 | RC-46 to RC-48 | N40-31 |
| R-28 | Fallback exits | 20 rows; 21 rehearsals; under all defaults the program exits M0 to M5 and M7 | RC-49, RC-50 | N40-32 |
| R-29 | Owner input | The six questions that matter most fall due at M0, M0, M1, M2, M3 and before the M7 freeze; the all-defaults product has seven printed limits (exploratory GPU results; no family policy; cheap rungs that only discard; generic cards and assumed norms; no Reality claim; `self-adjudicated`; private) | RC-52 to RC-54 | N40-35 |
| R-30 | Recurring gates of later subsections | Five: GATE-GAUNT-02, -05 and -10 (every exit from M0), GATE-GAUNT-03 (every exit from M2), GATE-WS-08 (every release); with R-03 they are every row whose `recurring` flag is set | RC-14 | N40-01, N40-04 |
| R-31 | Kill entries due beside the decision points of R-26 (re-read entries and introduced rows), M0 to M8 | Re-read entries of `exit` rows 0, 0, 0, 3, 1, 0, 2, 0, 0 (six: KILL-POOL, KILL-LOOPELIM and KILL-LIMBKERNEL at M3; KILL-AUGUR-REGION at M4; KILL-DRESSING and KILL-HARDCONTACT at M6); rows entered by `introduce` amendments: ten (KILL-RUT, KILL-TACTILE, KILL-TAXEL of N38a-37; KILL-SINEW-SEA of N40-23; KILL-HULL, KILL-STRIBECK, KILL-TYRE, KILL-GRANULAR of N40-24; KILL-AUGUR-SOS, KILL-AUGUR-FAMILYKERNEL of N40-26), decided 3 at M4 and 7 at M6. Both are due and block like decisions, and are counted apart: the decision counts of R-11, R-12 and R-26 include neither, and the seeded counts of R-22 and R-25 (550 and 144) do not include the introduced rows | Supplied here (DESIGN): KILL-POOL entered with `reread = []` (the re-read row reads 0, 0, 0, 2, 1, 0, 2, 0, 0) | N40-30, N40-04 |
| R-32 | Decision scope kinds (beside R-10) | Eleven, a closed table: `cell` (`cell:<stratum>`; KILL-TENFOLD-SPEED, -TIE, -ADJ, -REACH); `algorithm` (KILL-ALTALGO); `half` (`cuts`, `quotient`; KILL-AUGUR-CUTS); `clause` (`finite`, `tray`; KILL-COUPLING; `screening`, `comparison`; KILL-SYNTH-REGRESSOR); `backend` (`batch-cpu`, `gpu-cuda`, `gpu-metal`; KILL-D1 per device family, KILL-SHADOW, KILL-HARDCONTACT, KILL-RUT); `item` (`assets`, `actors`; KILL-DRESSING); `resolution` (`2x2`, `4x4`; KILL-TACTILE); `kind` (`fluid`, `cloth`, `rod`; KILL-LIVING); `family` (`family:<name>`, a column family of the response query: `counterfactual`, `adjoint`, `stencil`; KILL-RESPONSE); `mechanism` (`chain-cord`, `hinged-rug`; KILL-PROXY); `proofkind` (`proofkind:<kind>`, a proof kind of the enabled registry of `frob-verify`; KILL-AUGUR-FALSECERT). A `scope` of another kind than its row's `scope_kind`, or on a row with none, is refused `decision-scope-invalid` | Supplied here (DESIGN): `scope = "family:cuda"` accepted on a `backend` row (the kind check removed) | N40-03 |

**3. Binding check.** The binding the runner reads is an entry `ka:<id>@host` (N38a-13) supplied by the pinning
bead of the last column. A harness test (`register_known_answers`) asserts that every row due at or before the
selected milestone has that binding and a pinning test that exists and passes, and that each row is failed by its
control (the named N40-CX row's mutant, or the plant the control column states). A due row with no binding is `unbound` and fails GATE-GAUNT-02; a row due
later is `not yet due: <bead>` and blocks nothing. Every row is eligible under N38a-12 (derivation on file, exact
tolerance, registered control); no row is counted otherwise.

## Acceptance criteria

- [ ] The thirty-two rows are in `crates/frob-conformance/registry/known_answers.toml` under the numbers of their
      N38a-40 block, mapped from R-01 to R-32 in table order, with the truths above and every field of N38a-12's
      schema; each loads as `eligible`; no other known-answer file exists; each row's `slug` is its R- tag, and no tag
      appears anywhere else in the file, in a test name or in a derivation file.
- [ ] At each exit every row due by it (23 at M0, 29 at M1, 31 at M2 and M3, 32 from M4) is bound by a named test of the bead of
      its last column, and that test passes; the rows due later are reported `not yet due` with their beads.
- [ ] Every row with a control is failed by its control in the binding check.
- [ ] A change to `registries/gates.toml` that alters a count makes exactly the affected rows fail, naming them.
- [ ] Row identifiers are append-only: a diff that renumbers or reuses one fails CI.

## Test plan

### Unit tests

`cargo test -p frob-conformance register_known_answers`: `all_rows_pinned` (rows due at the selected milestone);
`all_controls_bite`; `ids_append_only`; `due_counts` (23, 6, 2 and 1 at M0, M1, M2 and M4; a planted row due at
M0 with no binding is `unbound`, the same row with `due = "M1"` is `not yet due` at M0); `block_and_tags` (one
block of 32 owned by N40-KA; R-01 to R-32 map to its numbers in table order, each tag equal to its row's `slug`; a tag cited in a
row's `case` or in a test name fails with `provisional-tag-past-due`, the same tag in `slug` does not); and one arithmetic test per derived row: `sum_279` (7+5+8+9+15+8+10+14+14+23+17+17+12+24+8+8+
16+13+10+12+14+10+4+1); `claims_sum_279` (197+34+16+13+10+7+2); `kills_sum_112` (13+35+45+19) and `144`
(112+7+25); `load_sums` (admission 41; convergence 316; kill 113); `late_sums` (90 and 26); `total_550`
(37+355+13+1+144); `weeks_33_7_10_5`; `reread_and_introduced` (0+0+0+3+1+0+2+0+0 = 6 re-read entries; 3 + 7 =
10 introduced rows; neither changes `load_sums`, `late_sums`, 144 or 550); `scope_kinds_eleven` (the eleven kinds
of R-32 in N40-03's order, each naming at least one kill row).

### End-to-end script and logging

`scripts/e2e/register_known_answers.sh` (this bead, M0; sources the shared helper of STD-E2E): runs the unit
tests above (`FROB_TEST_SEED=0 cargo test -p frob-conformance register_known_answers`) and then the binding check
of the rows due, with their controls, through the G0 tier target (`FROB_TEST_SEED=0 FROB_MILESTONE=<Mn> cargo
test -p frob-conformance --features gauntlet-mutations --test g0_laws -- register_known_answers`; the rows are
tier G0), and prints one line per row with its number, its tag (`slug`), its truth, the recomputed value and the
pinning test's result; with `--milestone <Mn>` (default: the current
milestone) asserts every row due by that exit passes (23 of 23 at M0; 32 of 32 from M4), the rows not yet due
listed with their beads and counted apart; then plants the off-by-one row below and asserts `sum_279` fails. Logs through the STD-E2E helper to
`target/e2e-logs/register_known_answers/<utc>.ndjson`: per step `ts`, `script`, `bead`, `step`, `event`, command,
exit code, wall time, the evidence block `{claim: known-answer, class: Tested, tier: G0:host, determinism: D0,
seed: 0}`, per row the expected and recomputed values, the register digest, and on failure the first differing
row; the `summary` line counts rows passed against rows due and names GATE-GAUNT-02 and GATE-GAUNT-10.

### Known answers and negative controls

This bead is the registry of known answers for its unit. Its own control: a planted row whose truth is off by one
(for example 278 convergence rows) must make `sum_279` fail.

## Considerations

- These are facts about a document's tables, so they move when the register is amended. That is intended: the
  amendment that adds a gate updates the row in the same change, and the review sees both.
- An earlier count of 421 identifiers was taken before the later subsections were written; it is not a row.
- The `R-` tags are this unit's citations; the numbers come from the N38a-40 block, and the allocator's tag table
  is the one place the two are joined. Nothing is renumbered after the mapping commit.

## References

Plan §40 (all tables), §43.1, §43.2, §43.12, §38.6. Appendix M (no section for these chapters). Gates
GATE-GAUNT-02, GATE-GAUNT-10.
