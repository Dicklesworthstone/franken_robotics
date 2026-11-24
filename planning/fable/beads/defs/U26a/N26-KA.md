---
key: "N26-KA"
title: "Nemesis known-answer registry: KA-659 to KA-669 with truths, tolerances, tiers, controls and owner beads"
type: "task"
priority: 2
milestone: "M3"
labels: ["m3", "nemesis", "frob-nemesis", "frob-conformance", "solid", "registry", "test", "review-notes-open"]
parent: "WP-55"
deps: ["BD-025", "N38a-12", "N38b-05", "N38b-28", "STD-E2E", "STD-TEST"]
needs: []
related: ["N26-REF", "N26a-01", "N26a-03", "N26a-04", "N26a-05", "N26a-06", "N26a-11", "N26a-13", "N26a-14", "N26a-15", "N26a-16", "N26a-17", "N26a-18", "N26a-19", "N26a-20", "N26a-21", "N26a-22", "N26a-23", "N26a-24", "N26a-25", "N26a-26", "N26a-27", "N26a-28", "N26a-29", "N26a-30", "N26a-31", "N26a-33", "N26a-34", "N26b-02", "N26b-03", "N26b-05", "N26b-06", "N26b-07", "N26b-08", "N26b-09", "N26b-10", "N26b-12", "N26b-14", "N26b-15", "N26b-16", "N26b-18", "N26b-22", "N26b-24", "N38b-56", "N38b-57", "N38b-61"]
gates: []
plan_refs: ["Appendix M M.25", "§26.14", "§26.3", "§26.5", "§26.6", "§26.7", "§26.9", "§26.10", "§26.11", "§38.6", "§10 C-27", "§10 C-28"]
---
## Background

A known answer is a case with a truth a test can compare against: an analytic formula, a hand-derived value or a
planted truth, with the tolerance of the comparison, the tier and site on which it runs, and the negative control
that must make it fail. The known-answer registry of `frob-conformance` is the authority for all of them; numbers
are append-only (a row is never renumbered and a number is never reused). The chapter on adversarial worlds and
rare failures states a rule of its own: a value in the chapter that follows from a stated formula is a derived
value and a known answer; every other number carries its label. This bead enters the chapter's eleven rows,
KA-659 to KA-669, so that none can be lost between the mechanism beads, and names for each the beads whose tests
carry it. It also lists the seven known answers of the two constructions behind the chapter (rare failures;
minimal counterexample worlds), which the chapter's beads cite and the constructions' own registry owns.

Reading the columns. *Tolerance:* `exact` for integers, verdicts, labels and identities; "inside the interval"
where the truth must lie inside the estimator's own confidence set in a stated share of repetitions; a gate's
threshold where the row quotes one. *Tier:* G0 laws and self-tests; G1 known answers; G5 determinism. Sites:
`host` (exact or double-precision arithmetic in a test binary), `referee`, `lockstep` (the throughput tiers).
*Negative control:* the mutation or planted failure the row must catch.

Labels. Planted fixtures and their constants are DESIGN choices. Truths derived from formulas are DESIGN
arithmetic with a derivation file under `docs/derivations/`. Gate thresholds are TARGETs. Nothing here is a
measurement: no row has been run on FrankenRobotics code.

## Scope

In: the eleven registry records; a derivation path per derived truth; the derivation files that no proof
obligation covers (the gambler's-ruin formula, the Good–Turing expectations, the delta-debugging reference
traces, the box-cone and penetration-pump arithmetic); the map from row to owning bead; each row's stated
negative control in its `control` cell, from which the seeding tool of the negative-control registry (N38b-05)
makes the control record (this bead writes no row of `controls.toml`).
Out: the tests themselves (owned by the beads in the last column); the derivation files of the four proof
obligations, which three rows point at and do not restate (N26b-16: 1-minimality and the 78-test bound for
KA-663, the zero-failure count for KA-664, the weight bound for KA-665, the coverage certificate for KA-669);
the runner that executes the eleven rows with their controls (N26b-17), whose registry view names tests and
controls by row number and reads truth, tolerance, tier, site and derivation path from the records here; the
chapter's counterexample catalogue and refusal registry (`N26-CX`, `N26-REF`).

## Technical approach

**Registry records.** Eleven `[[ka]]` rows appended to the suite's one physical file,
`crates/frob-conformance/registry/known_answers.toml`, in the row schema of N38a-12 and with no field of the
chapter's own (a chapter file such as `registry/known_answers/nemesis.toml`, which an earlier form of this bead
named, is a second copy and fails that loader with `ka-second-file`). The table's columns are the schema's
fields: "Stated in" is `section` (the subsection the column names: §26.3 for the search space, §26.5 the
oracle, §26.6 the guard, §26.7 shrinking, §26.9 rare failures, §26.10 the screening audit, §26.11 coverage
selection; every row is also listed in §26.14), with `owner` naming the gate where the row quotes one; "The
case" is `case`, and `fixture` names the physical case (rows share it only through `same_as`, which is empty
for all eleven); "The truth" is `truth`; "Tier · site" is `tier` and `sites`; "Negative control" is `control.text`
(`control.nc` is filled by the seeding tool of N38b-05, never by hand); `derivation` is the path under
`docs/derivations/nemesis/`; `label` is `DESIGN` for all eleven; `due` is `M3`, except KA-668, which is `M4`
(its owners are the screening-audit beads); `printed` and `slug` are empty (the rows are plan-time rows).
`tolerance.kind` is `exact` where the column says exact; `gate` naming GATE-NEMESIS-01 for KA-661;
`printed-digits` for the two bounds of KA-668; and, DESIGN (the schema has no interval-membership kind, and its
rule for real values is "the tolerance of the owning gate"), `gate` for the three rows whose truth must lie
inside the estimator's own confidence set: GATE-NEMESIS-04 for KA-665, GATE-NEMESIS-05 for KA-666 and KA-667
(the exploration gate, whose stopping signal the monitor is), with the membership rule and its share of seeded
repetitions written in the derivation file as the row's error budget.

**Which test carries a row.** The last column names the beads whose tests carry each row. The binding is the
test kit's and needs no field: a bound test is named `ka_<nnn>_…` (STD-TEST), and the runner view of N26b-17
names the rows it executes; the suite's lint fails a row due at the current milestone that has neither. The
check `owner_paths_exist` (provided here) reports per row `resolved` (a bound test or a runner entry exists) or
`planned`; the status is computed on each run and is not stored in the registry.

| KA | Stated in | The case | The truth (formula or value) | Tolerance | Tier · site | Negative control | Carried by |
|---|---|---|---|---|---|---|---|
| KA-659 | Search space; known answers | Planted illegal edit | Refused with FROB-REF-360 and the violated bound named; 100 of 100 planted illegal edits refused (GATE-NEMESIS-01) | exact | G1 · host | A profile with the bound removed admits it | BD-270, N26a-04, N26a-05 |
| KA-660 | Solvability oracle | Planted impossible trap: a doorway narrower than the body (body 0.75 m, door 0.70 m clear) | `infeasible-for-candidate` with the bottleneck named; never a counterexample; 0 of 100 planted impossible traps counted (GATE-NEMESIS-01) | exact | G1 · host, referee | An oracle that checks a point robot calls it solvable | N26a-13, N26a-04, N26a-33; BD-271 |
| KA-661 | Solvability oracle | Planted solvable trap of known nominal probability (trigger box z₁ > 2.3263 and z₂ > 2.7478: 0.01 × 0.003 = 3 × 10⁻⁵) | Found, with a causal witness attached; at least 95 of 100 planted solvable traps receive a witness (GATE-NEMESIS-01) | the gate's threshold | G1 · referee | Card-uniform randomization at the same budget, as the baseline | N26a-14, N26a-04, N26a-33, BD-273, N26a-07, N26a-11; BD-271 |
| KA-662 | Artifact guard | Planted artifacts: box-cone traction (a box-linearized friction cone grants √2·μ along its diagonals: 49.05 N against 69.37 N for 10 kg at μ = 0.5); energy-pumping penetration recovery (1 kg, 1 mm removed in a 2 ms step: 0.5 m/s, 0.125 J per event; the compliant law at 10⁵ N/m stores 0.05 J) | Each labelled `artifact-candidate`, with the variant that removed it | exact (label) | G1 · lockstep, referee | A genuine planted failure survives every variant | BD-272, N26a-15, N26a-16, N26a-14 |
| KA-663 | Shrinking | Planted causes of size 1, 2 and 3 among 8 to 32 edits | The planted set exactly. Reference traces: n = 8, 16, 32 with planted {2}, {2, n − 1}, {1, n/2, n} take 4, 16, 26; 5, 23, 41; 6, 30, 56 distinct tests | exact | G1 · referee | The order-dependence example: with edits listed (a, c, d, e, b) and failure when {a, b} or {c, d, e} is present, delta debugging ends at {c, d, e} before the pair check (15 distinct tests) and at {a, b} after it (20 in all; at most 78 tests of one- and two-edit subsets for 12 edits) | N26a-18, N26a-17, N26a-34; BD-274 |
| KA-664 | Rare failures | Zero-failure rule at δ = 0.05 for p⋆ = 10⁻⁴, 10⁻⁵, 10⁻⁶: n = ⌈ln δ/ln(1 − p⋆)⌉ | 29,956, 299,572 and 2,995,731 nominal episodes with no failure; the tenth qualification request (δ = 0.05/110) needs 76,959 for 10⁻⁴ | exact (count) | G0 · host | An episode count one short of the exact value, ⌊ln δ/ln(1 − p⋆)⌋ (29,955; 299,571; 2,995,730; 76,958 for the tenth request), for which (1 − p⋆)ⁿ > δ (0.0500041 at 29,955), so the bound is claimed without being established; and the tenth request tested at the δ = 0.05 count (29,956 against 76,959). The source's control, "ln(1/δ)/p⋆ rounded down" (29,957; 299,573; 2,995,732), errs on the safe side, cannot fail the row and is retired as `cannot-fail` (N38b-57, NEG-04, CX-400) | N26a-21, N26a-30, N26b-16 (PO-NEMESIS-04), N38b-57 (the replacement controls) |
| KA-665 | Rare failures | Gaussian tail by defensive mixture q = ε·p + (1 − ε)·p_shifted, shift 4, ε = 0.1; w = 1/(ε + (1 − ε)·exp(4z − 8)) | P(z > 4) = 3.167 × 10⁻⁵ recovered inside the confidence sequence; every weight at most 1/ε = 10 (9.970 at z = 0; 1 at z = 2; 3.727 × 10⁻⁴ at z = 4) | inside the confidence sequence | G0 · host | A mixture without its nominal share on a mis-centred proposal (failure when \|z\| > 4, truth 6.334 × 10⁻⁵, proposal shifted to +4 only) | BD-276, N26a-24, N26a-25, N26a-30 |
| KA-666 | Rare failures | Splitting on a biased random walk: up-probability 0.4, start at level 1; P(reach b before 0 from a) = (1 − 1.5ᵃ)/(1 − 1.5ᵇ) | Probability of reaching level 20 before level 0 is 0.5/(1.5²⁰ − 1) = 1.504 × 10⁻⁴ (level-to-level factors for levels 2, 4, 8, 16, 20: 0.4, 0.3077, 0.1649, 0.03755, 0.1973) | inside the estimator's interval | G0 · host | A walk with no noise after the fork returns identical children and is refused | BD-277, N26a-28, N26a-29, N26a-31 |
| KA-667 | Rare failures | Good–Turing monitor on a synthetic catalogue with known mode probabilities (1/2, 1/4, 1/8, 1/16, 1/32, 1/32) | Unseen mass inside its interval; exact expectations at N = 20: E[N₁/N] = Σ pᵢ(1 − pᵢ)¹⁹ = 0.0635, realized unseen mass Σ pᵢ(1 − pᵢ)²⁰ = 0.0598 | inside the interval | G0 · host | A monitor that counts distinct modes instead of modes seen once (0.230 at N = 20) | N26a-27 |
| KA-668 | Screening audit | A deliberately biased screen with a planted winner; exact bound 1 − δ^(1/m) | The planted winner's removal is reported; no top-k design among 30 audited discards bounds the false-discard rate at 9.5% with 95% confidence, and 59 bound it at 5.0% (the approximation ln(1/δ)/m gives 10.0% at m = 30 and is not used) | exact (verdict); printed digits for the bounds | G0 · host | A screen with no bias passes | N26b-02 (bound), N26b-06 (planted screens), N26b-07 (script); applied to the cascade by BD-345 |
| KA-669 | Coverage selection | Worked instance: 40 modes of total weight 100; greedy gains 31, 22, 14, 9, 6, 4; the six largest remaining gains sum to 12 | 86 covered; OPT₆ ≤ 86 + 12 = 98; the selection is within 86/98 = 88% of the best six-scene battery, against a worst-case guarantee of 1 − 1/e = 63% | exact | G1 · host | A selection that double-counts a mode | N26b-10 (certificate and worked instance), BD-278, N26b-12 (script) |

**Known answers of the two constructions, cited by the chapter's beads** (registered with the constructions'
specification beads; listed here so a reader of one chapter finds them):

| KA | The case | The truth | Cited by |
|---|---|---|---|
| KA-138 | Zero-failure bound | p ≤ 1 − δ^(1/n) ≤ ln(1/δ)/n at confidence 1 − δ; about 3 × 10⁴ episodes for p⋆ = 10⁻⁴ at δ = 0.05 | N26a-21 |
| KA-139 | Defensive mixture with zero observed failures | Weights at most 1/ε; the bound is only p ≤ ln(1/δ)/(ε·n), never better than crude sampling on the nominal share (ε = 0.1, n = 29,956: about 10⁻³) | N26a-25, BD-276 |
| KA-140 | Splitting with L = 6, p₀ = 0.1, N = 10⁴ | cv² near L·(1 − p₀)/(p₀·N); cv = √(6 × 0.9/1,000) = 7.3% | BD-277, N26a-29 |
| KA-141 | A random walk with a closed-form hitting probability | Crude and bounded-weight bounds bracket the truth in at least 1 − δ of 10⁴ repetitions; fork control: children sharing the parent's streams are detected as duplicates | N26a-22, N26a-30, BD-277 |
| KA-142 | Eight edits, failure exactly when edits 3 and 6 are present | The search returns {3, 6} (16 distinct tests); non-monotone control: a grease patch (edit 1) fails and a rug over it (edit 2) masks it, so {1} fails and {1, 2} passes; the tool returns {1} and claims no distance | N26a-18, N26a-17 |
| KA-143 | Impossible-world control: a doorway narrower than the body | Rejected as a counterexample for lack of a solvability witness | N26a-13 |
| KA-144 | Replay control of a stored counterexample file | The stored world reproduces its state hash | N26a-19 |

**Derivations on file** (`docs/derivations/nemesis/`). Written by this bead: the gambler's-ruin formula for the
walk; the Good–Turing expectations; the delta-debugging reference traces (generated by a 60-line reference script
kept with the derivation, independent of the product code); the box-cone and penetration-pump arithmetic.
Written by the derivations bead N26b-16 and referenced from the records here by path: the zero-failure count and
its two approximations (`po-nemesis-04-zero-failure.md`); the weight bound and unbiasedness of the defensive
mixture (`po-nemesis-02-weight-bound.md`); 1-minimality and the pair-check count
(`po-nemesis-01-one-minimal.md`); the coverage certificate (`po-nemesis-03-coverage-certificate.md`).

**KA-664 is a counting row.** Its truth is an episode count whose safe direction is `higher` (more episodes is
never wrong), declared in the row's `safe` field as N38b-28's rule 3 requires; a control must lie strictly on
the unsafe side, which is why the cell above carries the undercounts of N38b-57 and not the source's
rounded-down approximation (Appendix M.38, row 8; record REC-08). The printed control is kept in the row's
history as `retired: cannot-fail`, never as a live control.

**Negative-control seeding.** Each row's `control.text` is the table's cell. Where the cell reads as a refusal
(KA-666: identical children are refused), the text also names the plant behind it (children keyed by the
parent's address, the build feature `nemesis-fork-same-key` of CX-292), because the registry wants the build or
input in which the rule was removed and flags a restated refusal `boundary-only`. The seeding tool of N38b-05
then makes one control per plant (`NC-nnnn` in `controls.toml`, reusing the control of a counterexample entry of
N26-CX that names the same plant) and appends its number to `control.nc`, so the program-wide check "every gate
has a registered negative control that it catches" finds one for every Nemesis gate.

## Acceptance criteria

- [ ] `crates/frob-conformance/registry/known_answers.toml` holds the eleven rows KA-659 to KA-669, each with
      every field of N38a-12's row schema (truth, tolerance, tier, sites, control text, derivation path, `due`);
      N38a-12's loader accepts the file and rejects a duplicate or renumbered row; no other file in the
      repository holds a Nemesis known-answer row (a planted `registry/known_answers/nemesis.toml` fails the
      load with `ka-second-file`).
- [ ] Every derived value in the table is reproduced by a test from its formula: 29,956; 299,572; 2,995,731;
      76,959; 3.167 × 10⁻⁵; 6.334 × 10⁻⁵; 1.504 × 10⁻⁴; 0.0635 and 0.0598; 9.5% and 5.0%; 86, 98 and 88%; 78; the
      delta-debugging test counts.
- [ ] Each of the eleven rows states its control, none is flagged `boundary-only`, and after the seeding tool
      of N38b-05 has run every row's `control.nc` names at least one control of `controls.toml`. Demonstrating
      that each owning test does fail under its control needs the owning beads, which come later; it is done by
      the runner N26b-17 (which depends on this bead and on them) and recorded there.
- [ ] KA-664 declares `safe = "higher"`; its control cell names the two undercounts of N38b-57 (29,955 and
      the other one-short counts; the tenth request at 29,956), each strictly on the unsafe side; the source's
      safe-side count 29,957 offered as a live control is refused by the loader with `cannot-fail`.
- [ ] The check `owner_paths_exist` (provided here) reports a row `resolved` when a test named `ka_<nnn>_…` or
      a runner entry for it exists, and `planned` otherwise; nothing about the status is stored in the row.
      N26b-17 requires every row `resolved` at the M3 exit except KA-668 (`due = "M4"`), whose owners (N26b-02,
      N26b-07) land by the M4 exit.
- [ ] The reference script for the delta-debugging traces is independent of `frob-nemesis` and its output is
      pinned.

## Test plan

### Unit tests
`crates/frob-conformance/tests/ka_nemesis.rs`:
- `registry_loads_eleven_rows` (through N38a-12's loader, on the one file); `numbers_are_append_only` (a
  renumbered fixture file is rejected); `no_chapter_file` (a scratch tree with
  `registry/known_answers/nemesis.toml` fails the load with `ka-second-file`); `due_milestones` (KA-668 at M4,
  the other ten at M3).
- `derived_values`: one assertion per value in the acceptance list, computed in exact or interval arithmetic
  (the zero-failure counts by certified enclosure at n and n − 1; the Gaussian tails from the complementary error
  function to 4 significant figures; the walk by the closed form; the Good–Turing sums; 1 − 0.05^(1/30) = 0.0950
  and 1 − 0.05^(1/59) = 0.0495; 12 + 66 = 78).
- `ka_664_controls_unsafe_side`: for each of the four counts the control count is the exact count minus one
  (29,955; 299,571; 2,995,730; 76,958) and (1 − p⋆)^(n − 1) > δ ≥ (1 − p⋆)ⁿ holds in interval arithmetic;
  (1 − 10⁻⁴)^29,955 = 0.0500041; a scratch row offering 29,957 is refused `cannot-fail` with both numbers.
- `controls_stated_and_seeded`: no row's `control.text` is empty or `boundary-only`; on a scratch copy of the
  registries the seeding tool of N38b-05 fills `control.nc` for all eleven, and a row that names the plant of a
  counterexample entry (KA-660 and CX-289, the point-robot oracle; KA-666 and CX-292, children keyed by the
  parent's address) receives that entry's control and no second one.
- `owner_paths_exist`: on a fixture workspace, a row with a test named `ka_<nnn>_…` is reported `resolved` and a
  row with none is reported `planned` and is listed; on the real workspace the check is reported, not
  asserted, until N26b-17 asserts it.

### End-to-end script and logging
`scripts/e2e/nemesis_ka_registry.sh` (header `bead=N26-KA milestone=M3 labels=nemesis,registry gates=`; sources
the STD-E2E helper; NDJSON to `target/e2e-logs/nemesis_ka_registry/<utc>.ndjson` with a transcript) checks the
registry only, not the mechanisms: it runs `cargo test -p frob-conformance --test registry_check` and `cargo
test -p frob-conformance --test ka_nemesis`, logs as `metric` lines the row count (11), the rows per computed
eligibility state, the rows `resolved` and `planned` from `owner_paths_exist`, the `control.nc` count per row
after seeding, KA-664's `safe` declaration with its two live controls, and the digest of `known_answers.toml`;
then, on scratch copies of the registry, it plants a deleted KA-666 (completeness fails), a duplicated KA-660
(the loader refuses it) and the retired safe-side control 29,957 offered as KA-664's live control (refused
`cannot-fail`), each with `e2e_expect_caught` or `e2e_expect_refusal`. Every line carries `ts`, `script`,
`bead`, `step`, `event`, command line, exit code and wall time; the `summary` gives assertions passed and failed
and `gates: []`. The rows themselves are executed with their controls by N26b-17's runner (which depends on this
bead), which logs per row the truth, the tolerance, the measured value, the control's outcome and the derivation
path from these records.
`scripts/e2e/nemesis_*.sh` name the KA numbers they exercise in their `assert` events (field `ka`), so the
conformance report can show, per row, the last script run that touched it and its outcome.

### Known answers and negative controls
The table is the content. Negative control of the registry itself: a row whose truth was regenerated from the
candidate implementation instead of derived (the derivation file is then missing, or does not list the row;
N38a-12's loader marks such a row `ineligible: no derivation`, and an ineligible row supports no gate).

## Considerations

- Three formulas for the zero-failure count appear in the plan: the exact ⌈ln δ/ln(1 − p⋆)⌉ (29,956), the
  construction's ⌈ln 20/p⋆⌉ (29,958) and the control's rounded-down value (29,957). The row pins the exact
  count; the others are sufficient and are not the known answer.
- Rows whose tolerance is "inside the interval" are statistical: their tests fix seeds and repetition counts so
  that a correct implementation passes with probability above 0.999, and say so in the test.
- The corpus of planted fixtures behind several rows is discovery data and is frozen before any mechanism is
  tuned on it.

## References

Plan Appendix M (M.25: KA-659 to KA-669; KA-138 to KA-144), §26.14, §38.6, §10 C-27 and C-28.
