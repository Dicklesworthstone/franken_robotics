---
key: "N39-CX"
title: "frob-conformance: the counterexample and negative-control registry of the ten-fold harness"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "tenfold", "gauntlet", "frob-conformance", "solid", "registry", "test"]
parent: "WP-34"
deps: ["BD-025", "N38a-40", "N38b-05", "STD-E2E", "STD-TEST"]
needs: []
related: ["N00-CX", "N38b-04", "N39-KA", "N39-REF", "N39a-07", "N39a-08", "N39a-11", "N39a-13", "N39a-17", "N39a-18", "N39a-24", "N39a-27", "N39a-29", "N39a-30", "N39a-33", "N39a-34", "N39a-35", "N39a-37", "N39a-41", "N39b-01", "N39b-02", "N39b-03", "N39b-08", "N39b-18", "N39b-22", "N39b-23", "N39b-34", "N39b-35", "N39b-36", "N39b-37", "N39b-42", "N39b-43", "N39b-45", "N39bp3-01"]
gates: ["GATE-TENFOLD-03"]
plan_refs: ["§39.17", "§39.16", "§39.20", "§39.9", "§39.14", "§39.21", "§39.22", "Appendix N.3", "§38.11"]
---
## Background

A gate that nothing can fail validates nothing. Every gate of the ten-fold test therefore has planted defects it
must catch, registered before the passing test is written. This bead is that registry for the harness: the
tempting shortcut, the smallest case with its numbers, and the rule the case protects. The program's
counterexample catalogue files two entries that name this chapter under the thesis chapter (CX-003, the
per-brief point-estimate rule; CX-008, reporting speed on the missions that finished); the remaining 27 entries
below are new and are appended to the catalogue through a block of numbers. Until the block is issued they carry
the tags `TF-CX-01` to `TF-CX-27`, which the beads that plant and catch them already cite. A control is a defect,
not a boundary: each entry names the build or input in which the rule was removed.

## Scope

In: the 27 appended entries in `crates/frob-conformance/registry/counterexamples.toml`, each with its plant, the
gates that must catch it and the bead that builds the plant; the block request for their numbers; the derivation
file of the expected values; the registry check and its script. Out, with the bead that has it: the catalogue's
record schema and the seeding tool that writes `crates/frob-conformance/registry/controls.toml`, allocates `NC-`
numbers and proves completeness by the join (N38b-05; this bead writes no row of `controls.toml`); the number
blocks (N38a-40); the planted scorers and planted inputs themselves, each built by the bead that owns its target
(the last column); running the controls (N39b-22 for GATE-TENFOLD-03, the gate beads for the others, the control
runner N38b-04); the entries CX-003 and CX-008 and their controls (N00-CX).

## Technical approach

**Entries.** Each row is one entry in the record schema of N38b-05: `id` (the tag until the block is issued, then
the CX number), `shortcut`, `case`, `numeric = true`, `plan_case = "numbers"` (each case is stated in §39 or
derived from its cards), `case_supplied_by` (empty), `rule`, `rule_ids` (the gates of the row, the INV-TENFOLD
invariants and the refusal codes the rule names), `defined_in`, `same_as` (empty), `label` (DESIGN), `control_kind`
(`mutation` for a planted scorer, `planted-input` for a planted record or fixture), `plant` and `slug` (the tag).
The entry's `plant` is one plant record of N38b-05's form whose `id` is `<planting bead>:<plant>`, the planting
bead being the first named in the last column (`N39a-27:avg_ratios`, `N39bp3-01:pin_moved_in_freeze`). The
expected observation of each row is computed in `docs/derivations/tenfold/tf-cx.md` and is the plant record's
`expect`, which the seeding tool copies into the control's `expect`; the target and defect come from the shortcut
and the planting column; where the expectation is a refusal of the product (TF-CX-27's FROB-REF-862, N39a-34's
refusal of TF-CX-07), the plant record's `removed_check` names the check whose removal would let the case through
(BD-025's rule 5: a control is a defect, not a boundary). Planted scorers are compiled variants of the scoring
module behind a test-only Cargo feature under `gauntlet-mutations`, built by the scorer's bead under the plant identifier (the identifiers of N39a-41's step 3
are reused as they stand); planted inputs are fixture directories
`crates/frob-conformance/fixtures/tenfold/controls/<plant>/`, written by the bead that owns the record's schema.

**Numbers for the tags.** A bead never invents a number above the plan-time maximum (CX-384): this bead files one
`[[block]]` request in `crates/frob-conformance/registry/allocations.toml` (schema `frob.gauntlet.alloc/1`,
N38a-40) with `registry = "cx"`, `count = 27`, `owner = "N39-CX"`, `section = "§39.9, §39.14, §39.16, §39.17,
§39.20, §39.21, §39.22"`; the allocator fills `first` and `last`; the tags are mapped to the block's numbers in
table order in one commit and kept in `slug`. A tag still used as an identifier in a registry, a test name or a
derivation file at this bead's milestone fails the ledger check `provisional-tag-past-due`.

**Milestone.** M0 (moved from M3 in polishing round 3): three of the gates below are due before M3,
GATE-TENFOLD-01 at M0 (TF-CX-27, caught by N39b-08's `pin_drift_stops_before_timing`) and GATE-TENFOLD-02 and -09
at M1 (TF-CX-13, TF-CX-03), and a gate whose control is not in the catalogue when it is measured prints
`not-eligible`. The entries are registered at M0; each control is `stale` until its planting bead lands, which is
the normal state of a registry that comes before its mechanisms.

| Tag | Tempting shortcut | Smallest case | Rule protected | Gates | Kind; plant; planted by |
|---|---|---|---|---|---|
| TF-CX-01 | Average the per-family ratios | The six-family card: mean of 6, 7, 6.857, 7.714, 0.8125, 9 is 6.23; truth 2.647 | The estimate is a ratio of sums over pairs | GATE-TENFOLD-03 | mutation; `avg_ratios`; N39a-27 (caught in N39a-41 step 3) |
| TF-CX-02 | Drop the run that never finished | The same card without family 5: 42,300 ÷ 6,000 = 7.05; truth 2.647 | No run is dropped; a censored run contributes H (INV-TENFOLD-05) | GATE-TENFOLD-03 | mutation; `drop_censored`; N39a-27 (N39a-41; N39b-23's auditor) |
| TF-CX-03 | Compute a ceiling on our own ledger | Single pair: arm A's train share is 10% (ceiling 1.11); the rival's is 54.67% (2.206) | Ceilings are arithmetic on the rival's closed ledger | GATE-TENFOLD-03, GATE-TENFOLD-09 | mutation; `wrong_arm_ceiling`; N39a-37 (N39a-41) |
| TF-CX-04 | Quote the warm limit as the warm ratio | Warm card: limit 8.617 against R_warm(10) = 2.120 | The warm verdict is taken at N = 10 and never quoted without S and N* | GATE-TENFOLD-03 | mutation; `limit_as_warm`; N39a-17 (N39a-41) |
| TF-CX-05 | Leave out a standing asset | Warm card with S_A = 0: 8.617 at every N, N* = 1; truth N* = 5 | Standing assets are declared with their build cost | GATE-TENFOLD-03 | planted-input; `standing_asset_omitted` (the warm card with S_A = 0 and the asset in use); N39a-17 (N39a-41) |
| TF-CX-06 | Use a normal approximation for the guard | Without a continuity correction it passes 9 of 12 (z = 1.732, tail 0.0416) where the exact tail is 299/4,096 = 0.0730 and 10 of 12 is needed; and 21 of 32 (z = 1.768, tail 0.0385) where the exact tail is 0.0551 and 22 is needed | The guard is exact binomial arithmetic at fixed m | GATE-TENFOLD-03 | mutation; `normal_guard`; N39a-29 (N39a-41) |
| TF-CX-07 | Open a confirmatory cell with four families | 4 of 4 has tail 0.0625 > 0.05: the word is unreachable | A cell that cannot reach its verdicts is refused or opened descriptive | GATE-TENFOLD-03, GATE-TENFOLD-04 | planted-input; `four_family_cell`; N39a-33 (refused by N39a-34) |
| TF-CX-08 | Bootstrap 24 zeros | Zero misses in 24 families: bootstrap width 0; valid bound 1 − 0.05^(1/24) = 0.117346, printed 0.1174 (an upper bound, rounded up) | Rare failures get an interval valid at zero counts | GATE-TENFOLD-03 | mutation; `bootstrap_zero_counts`; N39a-30 |
| TF-CX-09 | Stop the fixed-n study when the monitor stops | Ask for the fixed-n verdict at the monitor's stopping time | The two study types are never mixed (INV-TENFOLD-07) | GATE-TENFOLD-03 | planted-input; `peek_verdict`; N39a-35 (N39a-41; N39b-23) |
| TF-CX-10 | Stop for futility on a running p-value | Seven consecutive families below 3: E = 1.5⁷ = 17.09 < 20; a mutant that stops there has no anytime validity | The monitor stops at E ≥ 20 only (eight: 1.5⁸ = 25.63) | GATE-TENFOLD-03 | mutation; `futility_on_running_p`; N39a-35 |
| TF-CX-11 | Accept a certificate because the builders' checker did | A planted false certificate: accepted by its declared checker, contradicted by direct evaluation at a sealed probe point | One false certificate disqualifies the arm from the scoped claim | GATE-TENFOLD-03 | planted-input; `false_certificate`; N39a-07 (direct evaluation), N39b-01 (disqualification); decided by N39b-03 |
| TF-CX-12 | Treat a sealed value in a transcript as harmless | A canary value planted in a sealed world appears in an arm's transcript | Any leak voids the touched cells and retires the families | GATE-TENFOLD-03 | planted-input; `canary_in_transcript`; N39b-02; decided by N39b-04 |
| TF-CX-13 | Keep a run whose ledger does not close | A ledger with 3% unattributed | Flagged `ledger-open`, kept in the ratio, excluded from ceilings (INV-TENFOLD-04) | GATE-TENFOLD-03, GATE-TENFOLD-02 | planted-input; `ledger_three_percent_open`; N39a-24 (the 2% closure rule; N39b-23) |
| TF-CX-14 | Freeze after seeing the suite | A manifest whose freeze time follows the suite opening | No paired run without all arms frozen and published first (INV-TENFOLD-03) | GATE-TENFOLD-03, GATE-TENFOLD-04 | planted-input; `freeze_after_open`; N39a-18 (N39b-23) |
| TF-CX-15 | Tune against the judge | An arm whose logs show the adjudicating simulator during tuning | Use of the adjudicating simulator in development disqualifies | GATE-TENFOLD-03, GATE-TENFOLD-05 | planted-input; `judge_in_tuning_logs`; N39a-08 (the adapter's access log), N39b-01; N39a-11 reads it |
| TF-CX-16 | Certify a population ratio by resampling | Rare-tail population (true ratio 1.8215): the percentile bootstrap certifies ten-fold in 78.6% of 24-family suites; the contrast test in none | A population claim rests on a valid bound and enough families | GATE-TENFOLD-10 | mutation; `bootstrap_population_ratio`; N39b-35 (N39b-36's 2,000 campaigns) |
| TF-CX-17 | Cite decisions per hour as the time ratio | 25-mission case: R_rate = 9.677 where R_time = 10.081 | The word attaches to R_time only | GATE-TENFOLD-10 | mutation; `rate_as_time_ratio`; N39b-35 (N39b-36) |
| TF-CX-18 | Count missions, not families | 24 missions on one site entered as 24 units | The family is the unit of inference; 24 rows from one site are one replicate | GATE-TENFOLD-10, GATE-TENFOLD-12 | planted-input; `missions_as_units`; N39b-34 (refused by `eval.preanalyze`), N39a-13 |
| TF-CX-19 | Build a kit from a private scene | A kit input carrying the mark `private` | A kit with a private or non-redistributable input is refused | GATE-TENFOLD-08 | planted-input; `private_kit_input`; N39b-18 |
| TF-CX-20 | Retune after the audit | An arm's pack digest changes after sign-off | A changed digest voids the arm | GATE-TENFOLD-11 | planted-input; `pack_digest_changed`; N39b-37 |
| TF-CX-21 | Keep a verdict the truth withdraws | An amendment sequence with one missed withdrawal | An arm keeps its scoped claim only with zero missed withdrawals | GATE-TENFOLD-12 | planted-input; `missed_withdrawal`; N39b-41 (N39b-42) |
| TF-CX-22 | Report an ablation above its bound | An information-only replay reporting 1.6 where unremoved training is 70% (bound 1/0.7 = 1.43) | A ratio above 1/s or 1/(1 − s) voids the stratum as an accounting error | GATE-TENFOLD-13 | planted-input; `ablation_above_bound`; N39b-45 |
| TF-CX-23 | Award the word against the declared incumbent alone | F against B gives 12 while the composed incumbent W is faster than B and F against W gives 4 | The word is governed by F against the stronger of B and W | GATE-TENFOLD-13, GATE-TENFOLD-06 | mutation; `word_against_declared_only`; N39b-43 |
| TF-CX-24 | Patch a frozen arm mid-gate | A defect fixed in arm A after the suite opened | The affected cells are void and re-run at the next gate | GATE-TENFOLD-03 | planted-input; `arm_patched_after_open`; N39a-18 (the freeze manifest), N39b-23 (the auditor) |
| TF-CX-25 | Measure across a pin move | A sibling or incumbent pin advanced during the tournament freeze, between two paired runs of one cell | A ratio of times measured across a pin move is not one measurement (`tenfold.protocol-breach`): with the stack freeze still open, the runs that span the move and their cells are void (`void: stack moved`) and are re-run on fresh families; the tournament is void, and re-run under a new freeze on a rotated suite, only if the freeze is closed as `aborted` or a run cannot be attributed to one receipt | GATE-TENFOLD-07 | planted-input; `pin_moved_in_freeze`; N39bp3-01 |
| TF-CX-26 | Stop a tournament cell on an interim number | A futility stop requested at M7 after eight consecutive families below 3 (E = 25.63) | At the tournament the monitor reports and every cell runs to n; the stop is refused | GATE-TENFOLD-07 | planted-input; `tournament_futility_stop`; N39a-35 (the fixture and the monitor side: `futility-crossing` and no stop), N39bp3-01 (the stop-request side: `frob-tenfold tournament stop-cell` refused) |
| TF-CX-27 | Time an incumbent on a drifted pin | The incumbent's container digest differs from the revision pinned by the rival audit | The baseline is timed only on audited pins: the run stops before any timing, and a figure without its complete disclosure block is refused (FROB-REF-862) | GATE-TENFOLD-01 | planted-input; `incumbent_pin_drift`; N39b-08 (`pin_drift_stops_before_timing`) |

The seven controls §39.17 names (a false certificate, a canary in a transcript, a ledger with 3% unattributed, a
manifest frozen after the opening, a four-family confirmatory cell, a fixed-n verdict at a monitor's stopping time,
an arm that touched the adjudicating simulator: TF-CX-11, -12, -13, -14, -07, -09, -15) are all attached to
GATE-TENFOLD-03.

**Catalogue entries reused.** **CX-003** (declare ten-fold when the point estimate is at least 10 on every brief:
that rule confirmed a true ratio of 10 in at most 18% of simulated campaigns; protects the pooled rule and its
pre-analysis) and **CX-008** (report speed on the missions that finished: with a fraction q of deadlines missed
R ≤ 1/q, so 12% unfinished caps the ratio at 25/3; protects restricted mean time). They are entries of the thesis
chapter (N00-CX), which owns them and their controls; this bead does not restate them. GATE-TENFOLD-03 must catch
both: N00-CX lists it among their `rule_ids` and in the `gate` list of their controls (its test
`cx003_cx008_carry_tenfold_03`), and this bead's registry check fails with `gate-missing`, naming the entry, when
either is found in the catalogue without it.

**The normal-tail control (TF-CX-06).** It is planted as a mutant scorer using the uncorrected normal tail; it must
be caught on every row where its critical count differs from the exact one. At α = 0.05: m = 12 (9 against 10) and
m = 32 (21 against 22), and m = 4, where it passes 4 of 4 (z = 2.0, tail 0.0228) and so makes the unreachable
four-family cell look reachable. At the first Holm step with four cells, α = 0.0125: m = 6 (it passes 6 of 6,
z = 2.449, tail 0.0072, where the exact tail 1/64 = 0.0156 exceeds the level and no count exists) and m = 12 (10
against 11; exact tails 79/4,096 = 0.0193 and 13/4,096 = 0.0032). Where both agree (m = 5, 6, 8, 16, 24 at 0.05;
m = 8, 16, 24, 32 at 0.0125) the mutant is not caught, which the derivation file states. With a continuity
correction the approximation gives the exact critical counts for every m in the guard table at 0.05, so only the
uncorrected form is a usable control.

## Acceptance criteria

- [ ] 27 entries are in `counterexamples.toml` in the schema of N38b-05, each with its plant, its gates in
      `rule_ids` and an expected value computed in `docs/derivations/tenfold/tf-cx.md`; the block request for 27
      numbers is committed; after issue the entries carry the block's numbers in table order with their tags in
      `slug`, and no tag remains as an identifier at M0.
- [ ] After the seeding tool of N38b-05 is re-run, each entry has exactly one control whose `seed` names it, with
      the gates of the table; the second run changes no byte; the join passes; no entry is in the findings file;
      this bead writes no record of `controls.toml`.
- [ ] Each of the seven controls the chapter names (TF-CX-11, -12, -13, -14, -07, -09, -15) is attached to
      GATE-TENFOLD-03.
- [ ] No gate of the ten-fold test (GATE-TENFOLD-01 to -13) has an empty control list after seeding: -01
      (TF-CX-27), -02 (TF-CX-13), -07 (TF-CX-25, -26) and the others as tabulated; `each_gate_has_control`
      enumerates all thirteen.
- [ ] The derivation file shows TF-CX-06 differing from the exact guard on exactly the five rows listed above and
      on no agreeing row, by exact arithmetic.
- [ ] Before its planting bead lands a control is reported `stale` with the bead, and its gate cannot record a
      pass (the runner's rule, N38b-04); the registry check reports this and does not fail this bead for it.
- [ ] `scripts/e2e/registry_n39_cx.sh` passes and rejects each of its planted registry defects.

## Test plan

### Unit tests

`crates/frob-conformance/tests/registry_cx_tenfold.rs`, run as `cargo test -p frob-conformance --test
registry_cx_tenfold` (real registry files and scratch copies, no mocks): `entries_load` (27, by tag before issue
and by number and `slug` after); `block_request_of_27`; `each_gate_has_control` (GATE-TENFOLD-01 to -13 after a
seeding run on a scratch copy); `seven_named_controls_on_gate_03`; `planting_beads_live`; `expect_differs_from_truth`
(each expected value of the derivation file differs from the true value of its case: 6.23 and 7.05 against 2.647;
1.11 against 2.206; 8.617 against 2.120; N* = 1 against 5; 9 of 12 and 21 of 32 against 10 and 22; 0 against
0.117346; 17.09 < 20 and 25.63 ≥ 20; 78.6% against 0%; 9.677 against 10.081; 1.6 against 1.43; 12 against 4: a
record whose `expect` equals the truth is rejected); `normal_tail_rows` (exact binomial tails as rationals and the
uncorrected and corrected normal tails for every m of the guard table at 0.05 and 0.0125: the uncorrected form
differs on exactly m = 4, 12, 32 at 0.05 and m = 6, 12 at 0.0125, and the corrected form on none at 0.05);
`reused_entries_reported` (on a scratch catalogue holding CX-003 and CX-008 with GATE-TENFOLD-03 in `rule_ids`, as
N00-CX enters them, nothing is reported; with the gate removed from CX-008 the check fails with `gate-missing`
naming CX-008); `tag_past_due_fails` (a scratch ledger at M0 with a tag left as an identifier fails
`provisional-tag-past-due`).

### End-to-end script and logging

`scripts/e2e/registry_n39_cx.sh` (header `bead=N39-CX milestone=M0 labels=tenfold,registry
plants=drop-plant,unknown-gate,expect-is-truth,stale-tag`): runs the test file; re-runs the seeding tool (`cargo
test -p frob-conformance --test seed_controls -- --ignored seed`) and the join (`cargo test -p frob-conformance
--test seed_join`) on a scratch copy of `registry/` and asserts by digest that a second seeding run is a no-op;
plants, one at a time on scratch copies, an entry with its plant removed, an entry naming GATE-TENFOLD-99, an entry
whose expected value equals its truth (TF-CX-01 at 2.647), and a tag left as an identifier at M0, and asserts each
is rejected. It calls no `frob` verb and runs no planted scorer: the controls are run by
`scripts/e2e/tenfold_known_answers.sh` (N39b-22) through `e2e_expect_caught` for GATE-TENFOLD-03, by N39a-41's
rehearsal for the planted scorers, and by the scripts of the other gates' beads, each logging one line per control
with `caught: true|false`. This script sources the STD-E2E helper and writes
`target/e2e-logs/registry_n39_cx/<utc-timestamp>.ndjson` with a transcript: every line has `ts`, `script`, `bead`,
`step`, `event`; `cmd` lines with the exact command line, exit code and wall time; the evidence block (claim Known
answer, class Tested, determinism D0); `metric` lines with entries, controls seeded, stale, and controls per gate
for GATE-TENFOLD-01 to -13; the catalogue and scratch-controls digests before and after the second seeding; per
planted defect an `assert` with the expected and observed failing check; on failure the first differing entry and
the scratch path; a `summary` line naming GATE-TENFOLD-03.

### Known answers and negative controls

The table is the content. The registry's own controls: a record whose `expect` equals the true value (a control
that plants nothing) is rejected by `expect_differs_from_truth`; the four planted defects of the script.
Determinism: D0, exact arithmetic.

## Considerations

- A control that has never failed its gate in any recorded run is reviewed at each milestone exit.
- A false certificate or leak found in real use becomes a permanent control with its counterexample, appended
  through a new block.
- Planted scorers exist only under a test feature reachable only under `gauntlet-mutations` and can never be
  linked into the kit's scorer (N39b-19's standalone scorer is built without it).
- The plant identifiers follow N39a-41's step 3 where it names them (`avg_ratios`, `drop_censored`,
  `wrong_arm_ceiling`, `limit_as_warm`, `normal_guard`, `peek_verdict`); TF-CX-05 takes its own planted input,
  because one control per entry needs a plant per entry and leaving a standing asset out is a defect of the input,
  not of the scorer that quotes the limit.

## References

§39.17, §39.16, §39.20, §39.9, §39.14, §39.21, §39.22, §38.11; GATE-TENFOLD-01 to -13; CX-003, CX-008;
INV-TENFOLD-03, -04, -05, -07; FROB-REF-862.
