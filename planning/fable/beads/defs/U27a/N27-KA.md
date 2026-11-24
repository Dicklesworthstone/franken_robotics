---
key: "N27-KA"
title: "Known-answer registry for Autopsy: KA-670 to KA-679, the related answers, derived values and their controls"
type: "task"
priority: 1
milestone: "M2"
labels: ["m2", "autopsy", "frob-conformance", "solid", "registry", "test"]
parent: "WP-56"
deps: ["BD-025", "N10-KA3", "N29-KA", "N35-KA", "N38a-12", "N38a-40", "STD-E2E", "STD-TEST"]
needs: []
related: ["N27-REF", "N27a-02", "N27a-06", "N27a-07", "N27a-09", "N27a-10", "N27a-11", "N27a-12", "N27a-13", "N27a-14", "N27a-16", "N27a-17", "N27a-18", "N27a-19", "N27b-20", "N38b-33", "SPEC-C-29"]
gates: []
plan_refs: ["§27.16", "§27.1", "§27.7", "§27.8", "§27.9", "§27.10", "§27.19", "Appendix M.26", "Appendix M.37", "§38.6", "§10 C-29", "§29.19"]
---
## Background

A known answer is a case whose right result is fixed by a formula or a construction before any code exists, with
a derivation on file. The program's rule is that known-answer tests pass against values that were never
regenerated from the candidate, and that every gate has at least one negative control: a planted defect the test
must catch, because a gate that nothing can fail validates nothing.

This bead is the registry of the known answers of the counterfactual-debugging chapter (the subsystem Autopsy,
crate `frob-autopsy`, and the fork mechanism in `frob-chronicle` that it specifies). The chapter adds a rule of
its own: a value in the chapter that follows from a stated formula is a derived value and a known answer; every
other number carries its label (TARGET, DESIGN, FACT). The registry therefore has three parts: the ten numbered
rows, the answers stated elsewhere that the same tests must honour, and the derived values. Mechanism beads cite
these numbers and restate the values they need; this bead is where the whole list lives and where its
completeness is checked.

Verification tiers named below: G0 (host arithmetic), G1 (known answer on the stated tier), G5 (determinism).

## Scope

In: the ten numbered rows and the two lane rows as `[[ka]]` records in the suite's one known-answer file,
`crates/frob-conformance/registry/known_answers.toml` (schema and loader: N38a-12; the same file as every
other chapter's rows; a file of this chapter's own, such as `registry/known_answers/autopsy.toml`, is what the
loader refuses as `ka-second-file`); for each row its fixture identity, truth, tolerance, tier and sites,
derivation path, label and due milestone; the controls for the two rows that state none; the request for a
block of two numbers for the lane rows in `crates/frob-conformance/registry/allocations.toml` (N38a-40); the
planning map from each row to the bead that carries it; the chapter's check (every row loads, every related
row is present under its owner, every derived value recomputes).
Out: the tests themselves (the beads named in the last column); the M3 suite that binds each numbered row and
the two lane rows to a runner entry and a control, runs them and proves each control fails as stated
(N27b-21, which writes no registry row: it adds the runner entries and the derivation files this bead's rows
point to); the control records in `controls.toml` with their NC numbers, which the seeding tool of N38b-05
writes from the control texts entered here; the rows of other chapters listed in part 2 (entered by their
owners: the construction's rows KA-145 to KA-148 by the constructions' registry N10-KA3, KA-716 to KA-718 by
the telemetry registry N29-KA, KA-802 by the numerics registry N35-KA, three M1 registries on which this bead
depends because its check reads their rows); the host fixtures (N27b-03, N27b-07); the derivation
files (N27a-16 for the thermal case, N27a-09 for the first-divergence argument, N27b-21 for the rest); the
counterexample registry and the refusal registry of the chapter (N27-CX, N27-REF).

## Technical approach

**Part 1. The numbered known answers (ten rows).**

| KA | The case | The truth | Tolerance | Tier | Negative control | Carried by |
|---|---|---|---|---|---|---|
| KA-670 | Planted single causes: 200 failures in five classes (friction drop, torque cap, payload shift, latency, terrain step), ten candidate factors each | The planted factor ranks first with label `but-for`; accuracy at least 90% (the kill gate KILL-AUTOPSY triggers below that) | As stated by GATE-AUTOPSY-03: at least 90% of 200, and the top factor changes in at most 30% of cases under 10⁻⁷ perturbation | G1 · referee | A failure planted through a factor outside the list names no factor | BD-280 with its suite N27b-08 and gate script N27b-09; control: N27a-14, BD-282 |
| KA-671 | Null fork | Identical digests | Exact (digest) | G5 · all tiers | A checkpoint with its packet queue stripped does not replay and is refused | N27a-06, BD-205, N27a-08; N27b-03, BD-279, N27b-04 |
| KA-672 | Chaotic placebo: a doubling map x ← 2x mod 1 with failure defined on its fortieth iterate | A 10⁻¹² placebo is amplified by 2⁴⁰ = 1,099,511,627,776 ≈ 1.1 × 10¹²; about half of placebo forks flip; the label is `chaotic: unattributable` | Exact (label) | G1 · host | The same map over five iterates (amplification 2⁵ = 32) attributes normally | N27a-14, BD-282, N27a-18; N27b-03 (the doubling-map fixture and the placebo counts) |
| KA-673 | Joint and alternative pairs: v of the pair 1 and of each single 0; v equal to 1 for either single and for the pair | Labels `joint` and `alternative`; Shapley values (½, ½) in both; repair sets {1, 2}, and {1} or {2} | Exact (rational) | G1 · host | A summary that reports only Shapley values cannot tell them apart | N27a-11, N27a-12, BD-281, N27a-18 |
| KA-674 | Negative Shapley value: v(∅) = 0, v({1}) = 1, v({2}) = 0, v({1, 2}) = 0 | Values (½, −½); they sum to v(N) − v(∅) = 0 exactly; a single-factor repair exists | Exact (rational) | G1 · host | None stated in the plan. Registered here (DESIGN): an estimator that clips negative values to zero returns (½, 0) and breaks the sum | N27a-11, N27a-12, BD-281, N27a-18 |
| KA-675 | Thermal re-run: one thermal node, time constant 60 s, ambient 25 °C, trip at 115 °C; steady-state rise 192 K at gear ratio 12 and 108 K at 16; 45 s task | Trip at 60·ln(192/102) = 37.95 s; ratio 16 from the start peaks at 25 + 108·(1 − e^(−0.75)) = 82.0 °C; ratio 16 applied at 36.95 s finds the winding at 113.3 °C and trips at 42.4 s | 0.1 s and 0.1 K (GATE-AUTOPSY-06) | G1 · referee | A tool that reports the forced fork's result as the answer to the design question | N27a-16, BD-283, N27a-17; N27b-03 (the host thermal node) |
| KA-676 | Warm-start graft across a design edit | Refused with FROB-REF-582 | Exact (verdict) | G1 · host | None stated in the plan. Registered here (DESIGN): a classifier that allows the graft when the radii differ by less than 10% runs the fork; and the same design edit at tick 0 must be accepted, so the refusal is shown to be specific | N27a-02, N27a-07, BD-205, N27a-08 |
| KA-677 | First divergence: two traces of eight ticks that differ at ticks 1 and 5 | Tick 1, by the chained digest | Exact | G1 · host | The pointwise midpoint search probes tick 3 or 4, sees agreement and reports tick 5 | N27a-09, N27a-13; BD-206 |
| KA-678 | Tolerance mode with a coverage gap | The earliest possible interval and the coverage gap | Exact | G1 · host | A tool that skips the gap reports a later tick | N27a-09, N27a-13; BD-206 |
| KA-679 | Precision: a marginal failure built to flip under a 10⁻⁷ perturbation and not under 10⁻¹², taken from a single-precision run | Labelled `precision-sensitive` | Exact (label) | G1 · referee, lockstep | The same failure with a wide margin attributes normally | N27b-07 (precision routing, under BD-280); BD-282 (the forced precision trigger) |

**Part 2. Answers stated elsewhere that bind the same tests.**

| Identifier | The case | The truth | Carried by |
|---|---|---|---|
| KA-145 | Null intervention (stated with the construction "a deterministic forkable simulator is a causal model") | The empty intervention replays the recorded failure bit for bit | N27a-06, BD-205 |
| KA-146 | Shapley summary with two interventions in two cases: either alone prevents the failure, v({A}) = v({B}) = v({A, B}) = 1; only both together prevent it, v({A}) = v({B}) = 0 and v({A, B}) = 1 | Both give φ = (½, ½), although the first has two causes with probability of necessity 1 and the second has none. (The construction's row words the first case's label `but-for`; the chapter's label for several single rescuers is `alternative`, and the chapter governs.) | N27a-11, BD-281 |
| KA-147 | Placebo fork on a fixture where every nudge flips the outcome | The report is `chaotic: unattributable` | N27a-14 (the knife-edge fixture), BD-282 |
| KA-148 | Planted cause | Recovered as first-ranked in at least 90% of planted cases (TARGET) | BD-280 |
| KA-716 | Fork (stated with the telemetry chapter) | The empty intervention replays bit for bit; a setpoint change at tick t reuses the prefix to the snapshot before t; control: the 60 mm warm start grafted onto the 75 mm wheel | N27a-04, N27a-05, N27a-07, BD-205 |
| KA-717 | First divergence: traces that differ at ticks 1 and 5 | Tick 1; control: midpoint search on a pointwise predicate reports tick 5 | N27a-09 |
| KA-718 | Teleport | A planted single-kernel divergence is named by step and stage | N27a-09 (the planted tick-and-stage battery) |
| KA-802 (part) | Licence refusals on planted cases | Attribution or a counterfactual fork below determinism class D2 is refused (FROB-REF-562) | N27a-08, BD-283 |
| `lane-1` (no number yet) | The single-precision lane's linear map x ← 1.1735·x (exponent ln 1.1735 = 0.16 per step) | After 40 steps a perturbation has grown by 602; the measured growth must match within 1% | N27b-03 (`linear_map_growth`), N27b-23 (stated in the lane's subsection; listed here so the chapter's list is whole) |
| `lane-2` (no number yet) | The lane's negative control: a pendulum released 5 × 10⁻⁸ rad from upright with no control, verdict "falls to the positive side" | A placebo of 10⁻⁷ rad flips the side whenever its sign is negative, so ρ₀ is about one half; the lane must not admit the case; label `marginal`. At a placebo of 10⁻¹² the side is stable | N27b-23 (`pendulum_marginal`, `pendulum_stable_at_1e12`) |

**Part 3. Derived values of the chapter** (each follows from a stated formula; each has a test).

| Value | Formula | Carried by |
|---|---|---|
| 78 coalitions | 12 singles + C(12, 2) = 66 pairs | BD-281, N27a-10 |
| 689 forks | 78 × 8 replicates + 64 placebo forks + 1 null fork; the 10⁻⁷ series adds 8 per coalition that rescues (counted as the run layer schedules it: each coalition that changed the outcome in the first phase, BD-279) | BD-281, N27a-10, N27a-18 |
| 32 and 1,024 coalitions | 2⁵ for an exact Shapley summary at m = 5; 2¹⁰ for ten factors | N27a-11, N27a-10 |
| Contrasts 0.76 and 0.13 | Of 200 failing worlds: 164, 38 and 12 prevented by torque limit off, nominal friction and a placebo: rescue rates 0.82, 0.19, 0.06; contrasts net of the placebo | N27b-05 (the statistics, under BD-280); cited in BD-281, BD-282 |
| Interaction −11/32 | Rescue rates 8/8 and 3/8, pair 8/8, placebo 2/64: 1 − 1 − 3/8 + 1/32 | BD-281, N27a-10 |
| 137 and 113 forks | Three interventions, singles and pairs: 1 + 64 + 6 × 8 = 113; plus 3 × 8 for three coalitions that changed the outcome = 137 | BD-281 |
| Growth 3.6 × 10⁵ | e^(0.16 × 80) for an exponent of 0.16 per control step over 80 steps (2 s at a 25 ms control period); single-precision round-off 6 × 10⁻⁸ then reaches about 2% of state scale; double-precision round-off 10⁻¹⁶ stays near 4 × 10⁻¹¹ | N27b-07 and N27b-03 (the stability report, under BD-280); cited in N27a-03 |
| Several gigabytes | 20 snapshots × 8,192 worlds × about 35 kB ≈ 5.7 GB | N27a-03 |
| Thermal constants | 192 × (12/16)² = 108 K; 115 − 82.0 = 33 K in hand; 25 + 108 = 133 °C asymptote of the forced fork | N27a-16 |
| Lip lever | √(2rh − h²) for h = 12 mm: 36.000 mm at r = 60 mm, 40.694 mm at r = 75 mm (why a warm start cannot be grafted) | N27a-07 |

**Registry mechanics.** Row format: the `[[ka]]` record of N38a-12, field for field, and no other key (the
loader rejects one): `id`; `section` (the "Where stated" of Appendix M.26, each with §27.16, the chapter's
list: §27.6 for KA-670, §27.3 for KA-671, §27.5 for KA-672, §27.7 for KA-673 and KA-674, §27.8 for KA-675 and
KA-676, §27.10 for KA-677 and KA-678, §27.9 for KA-679; §27.19 for the lane rows; and `owner` naming the gate
the row serves: GATE-AUTOPSY-03 for KA-670 and KA-679, -01 for KA-671, -05 for KA-672, -04 for KA-673 and KA-674, -06
for KA-675, -02 for KA-676, -07 for KA-677 and KA-678, -09 for the lane rows); `fixture` (one identity per
physical case: `autopsy-planted-single`, `null-fork`, `doubling-map`, `shapley-two-factor` shared by KA-673
and KA-674 through `same_as`, `thermal-regear`, `warm-start-graft`, `eight-tick-trace`, `tolerance-gap`,
`threshold-block`, `linear-map`, `upright-pendulum`); `case`; `truth` with its `kind`; `tolerance` (`exact`,
`abs` for the thermal row, `gate` for KA-670, `rel` 1% for the linear map); `tier` with `sites` (the table's
"G1 · referee" is tier G1 with sites `["referee"]`; "all tiers" is the four sites `referee`, `batch-cpu`,
`cuda`, `metal`; "referee, lockstep" is `referee` and the throughput sites); `control` (`text` as in the table
and the `nc` identifiers once the seeding tool has issued them); `derivation` (a path under
`docs/derivations/`: `thermal-regear.md` for the thermal case, `first-divergence-monotonicity.md` for the
first-divergence rows, the `autopsy-*.md` files of N27b-21 for the rest; each file's front matter lists the KA
identifiers it serves); `label` (`DESIGN`: every value is arithmetic or follows from the design by argument);
`due` (M2 for KA-671, KA-676, KA-677 and KA-678, whose first carriers are M2 beads of the fork service; M3 for
the others); `printed` (empty). The "Carried by" column and the script list below are planning provenance,
not fields: a row is bound to code by its runner entry `ka:<id>@<site>` in the tier target (N38a-13; entered
for these rows by N27b-21) and by the tests that cite its identifier.
**The lane rows and their numbers.** The two answers of the single-precision lane were stated after the
numbered list was compiled. This bead files one request in `allocations.toml` (`registry = "ka"`, `count = 2`,
`owner = "N27-KA"`, `section = "§27.19"`); nobody types `first`. Until the block is issued the rows carry
`slug = "lane-1"` and `slug = "lane-2"` and are cited by those tags; when it is issued, the two numbers
replace the tags in table order (linear map, then pendulum) in one commit, in the registry, the test names and
the derivation files, and a tag still present at the rows' due milestone (M3) fails N38a-40's check
`provisional-tag-past-due`. If KILL-ENSEMBLE withdraws the lane, the two numbers are recorded `void` in the
block and are never reused.
**Related rows and derived values.** The rows of part 2 that carry a number are not entered a second time
(a duplicate identifier is a load error): this bead's check asserts that each is present under its owner with
the truth stated above, so that a change there is seen here. The derived values of part 3 are not registry
rows (they have no numbers): each is recomputed from its formula by the test below, in exact rationals where
they are rational and from decimal strings with their formula otherwise; no value is copied from program
output.

## Acceptance criteria

- [ ] `crates/frob-conformance/registry/known_answers.toml` holds the ten numbered rows KA-670 to KA-679 and
      the two lane rows (under their slugs until the block is issued, then under its numbers), each a `[[ka]]`
      record with every field of N38a-12; the loader of N38a-12 validates the file; no file exists under a
      directory named `known_answers`.
- [ ] The block request for two `ka` numbers (owner N27-KA) is in `allocations.toml`; once it is issued no
      `lane-1` or `lane-2` tag remains in the registry, a test name or a derivation file.
- [ ] The eight numbered rows of part 2 are present under their owners with the truths stated here (the check
      names a row that is missing or differs); none is entered twice.
- [ ] KA-674 and KA-676, which the plan lists with no stated control, have the control texts registered here,
      so that the seeding tool can issue their control records.
- [ ] The count matches the plan's own count for the chapter: 10 known-answer rows, 2 with no stated control,
      0 with a tolerance still to set.
- [ ] A recomputation test evaluates every derived value of part 3 from its formula.
- [ ] At this bead's close (M2) the rows load and this bead binds none of them: a registry never waits on its
      mechanisms. The runner entries come with the carrying beads and with N27b-21, which checks the binding
      of every row hard, as does the exit review N27b-30.

## Test plan

### Unit tests
`crates/frob-conformance/tests/ka_autopsy_registry.rs`:
- `ten_numbered_rows`: identifiers KA-670 to KA-679 present in the one file through the test kit's loader
  (STD-TEST), each with every field of N38a-12, a tolerance other than `to-set` and a control text.
- `no_private_file_or_key`: a scratch copy with a row moved to `registry/known_answers/autopsy.toml`, or with
  a key `test` or `carried_by` added to a row, is refused by the loader (`ka-second-file`; unknown key). The key
  `owner` is a field of the schema (the gate the row serves) and is accepted.
- `lane_block_requested` (`owner = "N27-KA"`, `count = 2`); `lane_tag_past_due_fails` (a scratch copy at M3
  with `lane-1` left in a test name).
- `related_rows_present_under_owners`: KA-145 to KA-148, KA-716 to KA-718 and KA-802 load with the truths of
  part 2; a scratch registry that enters KA-146 a second time fails to load.
- `derived_values_recompute`: 12 + 66 = 78; 78 × 8 + 64 + 1 = 689; 2⁵ = 32; 2¹⁰ = 1,024; 164/200 − 12/200 =
  0.76; 38/200 − 12/200 = 0.13; 1 − 1 − 3/8 + 1/32 = −11/32; 113 and 137; e^(12.8) = 3.62 × 10⁵ (within 1%);
  6 × 10⁻⁸ × 3.62 × 10⁵ = 0.0217; 10⁻¹⁶ × 3.62 × 10⁵ = 3.6 × 10⁻¹¹; 20 × 8,192 × 35,000 = 5.73 × 10⁹ bytes;
  60·ln(192/102) = 37.951; 25 + 108·(1 − e^(−0.75)) = 81.984; √(2 × 60 × 12 − 144) = 36.000 and
  √(2 × 75 × 12 − 144) = 40.694.
- `due_milestones`: KA-671, KA-676, KA-677 and KA-678 are due at M2 and the other rows at M3.
- `controls_present`: no row's control text is empty or `none stated`.
- `shapley_rows_exact`: (1/2, 1/2), (1/2, 1/2), (1/2, −1/2) as rationals.

### End-to-end script and logging
`scripts/e2e/autopsy_ka_registry.sh` (header `bead=N27-KA milestone=M2 labels=autopsy,registry gates=`; sources
the STD-E2E helper; NDJSON to `target/e2e-logs/autopsy_ka_registry/<utc>.ndjson` with a transcript) checks the
registry and nothing that a later bead delivers: it runs `cargo test -p frob-conformance --test registry_check`
and `cargo test -p frob-conformance --test ka_autopsy_registry`, logs as `metric` lines the counts (10 numbered
rows, 2 lane rows by tag or number with the block's `first`, `last` and `status`, 0 rows `to-set`, the 8 related
rows found under their owners), each row's computed eligibility and `due`, and the digest of
`known_answers.toml`; on scratch copies it plants KA-674's control text deleted, a lane row moved into
`registry/known_answers/autopsy.toml`, and KA-146 entered a second time, and requires each to be caught. Every line
carries `ts`, `script`, `bead`, `step`, `event`, command line, exit code and wall time; on a failure the first
differing value and the scratch path; the `summary` gives assertions passed and failed and `gates: []`.

The rows themselves are discharged by the scripts of their carrying beads. The scripts that carry rows from the user's side log the KA identifier beside every assertion
that discharges one: `run_fork.sh` (KA-671, KA-676), `gate_autopsy_01.sh` (KA-671), `gate_autopsy_07.sh` (KA-677,
KA-678), `autopsy_rerun.sh` (KA-675), `autopsy_coalitions_abstention.sh` (KA-672, KA-673, KA-674, KA-147 and
KA-670's control), `autopsy_attribute_planted.sh` (N27b-09: KA-670, and the precision route of each case for
KA-679), `autopsy_refusals.sh` (N27b-19: KA-679's FROB-REF-584 on the marginal fixture). Every numbered row is
thus discharged by at least one script from the user's side.

### Known answers and negative controls
This bead is the list. Its own negative controls are on the registry check: a row with its control deleted, a
row moved into a chapter file, a related row entered twice, and a derived value edited by one unit in the last
printed digit must each fail the check. Determinism lanes: the registry and its recomputation are exact or use fixed decimal
strings; identical on every platform.

## Considerations

- The related rows carry identifiers owned by other chapters' registries (the constructions, the telemetry
  chapter, the numerics chapter). They are repeated here by value so that the Autopsy tests can be read alone;
  the owning registry remains the source of the identifier.
- The two unnumbered answers of the single-precision lane were added to the chapter after the numbered list was
  compiled. Their numbers come from the allocation ledger and from nowhere else: this bead requests the block
  and never types a number.
- KA-679 and the lane's pendulum control describe neighbouring cases with different labels
  (`precision-sensitive` and `marginal`); the difference is whether double precision reproduces the failure and
  whether the lane's admission rule was tried. Both are kept, and the tests that carry them state which path
  each fixture takes.

## References

Plan §27.16, §27.1, §27.7 to §27.10, §27.19, Appendix M.26 (KA-670 to KA-679), Appendix M.37 (counts), §38.6,
§10 C-29 (KA-145 to KA-148), §29.19 (KA-716 to KA-718), §35.15 (KA-802).
