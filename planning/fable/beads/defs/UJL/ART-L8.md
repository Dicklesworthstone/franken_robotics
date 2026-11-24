---
key: "ART-L8"
title: "golden fixture L.8: the fixed-n study plan `frob.plan/1`, exact pre-analysis, a consistent result, CLI script"
type: "task"
priority: 2
milestone: "M3"
labels: ["m3", "arena", "atelier", "frob-arena", "frob-opt", "frob-conformance", "solid", "test", "e2e"]
parent: "WP-25"
deps: ["ART-L1", "ART-L3", "BD-124", "BD-262", "N24b-06", "N28a-21", "N32a-13", "NJL-07", "NJLx-02", "STD-E2E"]
needs: []
related: ["ART-L9", "BD-211", "BD-219", "BD-263", "GB-30", "N24b-14", "N24b-21", "N33b-19", "NJL-09"]
gates: []
plan_refs: ["Appendix L.8", "§32.6.1", "§24.12", "§24.10", "§28.6", "§32.9", "§35"]
---
## Background

FrankenRobotics (a robotics simulation and design system reached through the command line `frob`) compares
designs with a **two-phase study**. `study.plan` binds a trial manifest to immutable designs, scene, task,
seed protocol, budget, backend and one declared study type (anytime-valid, or fixed-n exact; never both in one
study) and returns a plan whose identifier is the content address of the resolved manifest, a cost estimate
with its interval and, for a fixed-n study, the **pre-analysis**: which verdicts the rule can reach at that n.
Planning runs no physics. `study.submit` takes the plan address and an idempotency key; `study.compare` reads
the finished study. The reason for two phases (DESIGN): the agent sees cost and reachable verdicts before any
budget is spent, and the submitted object cannot drift from the planned one.

This bead turns the eighth worked artifact of the specification into a golden fixture: the plan of a fixed-n
exact study in the nine-artifact story (a quadruped carries a tray through a commercial-kitchen galley). It
compares the design before and after one edit (a larger foot) on the tray-delivery task of artifact L.3. The
artifact is the `result` of `study.plan`. The specification fixes what a plan binds and does not print the
fields of `frob.plan/1`, so the spellings below are proposals confined to those contents. The estimate is a
DESIGN specimen; the pre-analysis and the result arithmetic are exact.

Why this fixture matters (DESIGN). The pre-analysis is the reason to plan before spending: at 64 pairs and a
five-point margin, a practical tie cannot be established by any outcome, and a study that needed that verdict
would spend its whole budget for nothing. The counts are exact and are a known answer for the enumerator, and
the result (b = 19, a = 2) is a known answer for the rule. Rejected: testing the planner only against its own
enumerator (the fixture's numbers were derived independently and recomputed for this bead).

## Scope

In: the specimen and resolved plan; the rule `paired-clopper-pearson/1`; the pre-analysis counts; how the plan
is read; the estimate against the budget; a result consistent with the plan, with its interval and verdict;
refusals on planted mutations; the script. Out: the two study types and the plan verb (BD-124), the rule's
enumerator (N24b-06), paired comparison on devices and the comparison record (BD-262, N28a-21), promotion to
the referee (BD-263); the submission checks (`FROB-REF-809`) and the pair-accounting constructor (N32a-13); the
layers and the identity map (NJL-07).

## Technical approach

**Files** under `fixtures/golden/appendix-l/L8/`: `plan.specimen.json` (the text below, with its `preanalysis`
block in the owner's form, as the paragraph after the text says); `plan.resolved.json`;
`manifest.json` (the input manifest that yields the plan); `preanalysis.expect.json`; `compare.expect.json`;
`fields.toml` (the `estimate` block is `illustrative`; `plan`, the design and task identities and
`preanalysis.hash` are `identity`; everything else is `exact`).

**The artifact.**

```json
{ "schema": "frob.plan/1", "plan": "plan:b3:52e8…",
  "question": "B delivers the tray more often than A by more than 0.05",
  "designs": { "A": "d:7a3e…", "B": "d:c05b…" },
  "controller_arm": "shared: baseline controller re-synthesized per design, equal resources",
  "task": "t:kitchen-tray@88c1…", "requirement": "delivered",
  "scenes": { "family": "kitchen-commercial/galley", "split": "split:b3:60d3…", "part": "development" },
  "seed_protocol": { "pair_by": ["world", "replicate"], "unit": "building", "addresses": "philox:plan-key" },
  "backend": "strict:gpu-cuda", "determinism": "D2", "promote_to": "referee",
  "study": { "type": "fixed-n", "n": 64, "margin": "0.05", "level": "0.05",
             "rule": "paired-clopper-pearson/1" },
  "preanalysis": { "outcomes": 2145,
                   "reachable": { "promote": 521, "inferior": 521, "inconclusive": 1103 },
                   "unreachable": ["practical-tie"],
                   "note": "perfect concordance gives [-0.0662, +0.0662]; a tie first becomes reachable at n = 86",
                   "least_b_at_a_0": 15, "hash": "b3:5d10…" },
  "estimate": { "gpu_s": 96, "interval_90": [78, 121], "referee_cpu_core_s": 410 },
  "budget": { "gpu_s": 150, "cpu_core_s": 600 } }
```

**The `preanalysis` block is entered in the owner's form (DESIGN, following the owner of the format).** The
specification prints that block under names of its own, as shown above (`reachable` as a map of counts,
`unreachable`, `note`, `least_b_at_a_0`), and calls its spellings proposals. The owner has fixed the block:
`study.plan` embeds the record `frob.preanalysis/1` of `eval.preanalyze` unchanged (N33b-19; the record is
N24b-06's, with the members `rule`, `n`, `margin`, `level`, `interval_level`, `outcomes`, `counts` per verdict,
`reachable`, `first_reachable`, `min_b_for_prefer`, `concordance_interval`, `power` and `hash`), so that
`preanalysis.hash` in a plan equals the hash the verb prints. A plan carrying the printed block would not be the
owner's `frob.plan/1`, and a specimen that the format's own reader rejected could not be resolved and validated.
`plan.specimen.json` therefore carries, in place of the printed block, the record that `frob eval preanalyze
--rule paired-clopper-pearson/1 --n 64 --margin 0.05 --level 0.05 --json` returns, with its `hash` abbreviated
to `b3:5d10…`. This is the one part of the specimen taken from the owner's writer and not typed from this bead:
the record's byte form is N24b-06's, and typing it here would state that schema a second time. Its contents are
the specification's numbers under the owner's names, pinned member by member in `preanalysis.expect.json`:
`outcomes` 2,145; `counts` 521 promote, 521 inferior, 0 practical-tie and 1,103 inconclusive (the printed
`reachable` map, with the tie's zero); `reachable` without `practical-tie` (the printed `unreachable`);
`concordance_interval` [−0.0662, +0.0662] to four decimals and `first_reachable` 86 for `practical-tie` (the
printed `note`); `min_b_for_prefer` 15, 17, 19, 21, 23, whose first entry is the printed `least_b_at_a_0`;
`level` 1/20 and `interval_level` 39/40.

**What the plan binds.** `question`: the claim in words. `designs`: arms A and B by immutable identity (A is
the design of artifact L.1; B is the same design after the edit that sets the foot radius to 0.030 m).
`controller_arm`: how controllers are treated; here a shared rule, the baseline controller re-synthesized per
design with equal resources. `task`, `requirement`: the task of artifact L.3 and its verdict `delivered`.
`scenes`: the family, the split and the part of the split (`development`). `seed_protocol`: units are paired
by world and replicate; the unit of inference is the building (episodes that share a building share a
generator draw); world addresses are derived from the plan key. `backend`, `determinism`, `promote_to`: a
strict GPU backend at class D2, with promotion to the referee. `study`: one type with its n, margin, level
and rule. `preanalysis`: the reachable verdicts at this n. `estimate`, `budget`: the cost estimate with its
90% interval against the budget.

**The rule `paired-clopper-pearson/1`.** Let n be the number of complete pairs, b the pairs in which only B
succeeds and a those in which only A succeeds. Form two-sided 97.5% exact Clopper–Pearson intervals
[L_b, U_b] and [L_a, U_a] for the two discordance probabilities; the interval [L_b − U_a, U_b − L_a] covers
their difference with probability at least 95%. With margin m = 0.05: *promote* (prefer B) when
L_b − U_a > +m and B violates no hard limit in a condition where A does not; *inferior* when U_b − L_a < −m;
*practical tie* when the interval lies inside [−m, +m]; *inconclusive* otherwise.

**The pre-analysis at n = 64** (every count pair (a, b) with a + b ≤ 64 is classified before any pair runs):

| Quantity | Value | Why |
|---|---|---|
| Outcomes | 2,145 | C(66, 2) count pairs |
| Promote; inferior; inconclusive | 521; 521; 1,103 | Enumeration of the rule; the three sum to 2,145 |
| Practical tie | unreachable | Perfect concordance (a, b) = (0, 0) gives [−0.0662, +0.0662], because 1 − 0.0125^(1/64) = 0.0662 > 0.05 |
| First n at which a tie is reachable | 86, and only for (0, 0) among 3,828 outcomes | 1 − 0.0125^(1/86) = 0.0497 < 0.05 |
| Least b for promotion with a = 0 | 15 | With one to four A-only pairs: 17, 19, 21, 23 |

**How the plan is read and used** (rules, not illustrations).
1. The type is declared and kept. A fixed-n study computes its verdict once, at 64 complete pairs. No
   e-process may stop it; it may stop early only when every completion of the unresolved pairs gives the same
   verdict.
2. The pre-analysis is the reason to plan before spending. A request that named "tie" among its verdicts is
   refused (`FROB-REF-532`).
3. The unit is the building: pairs are formed per building, and the 64 units are 64 buildings.
4. Pairing needs a controlled determinism class: both arms run at D2 on one backend; under an uncontrolled
   class the paired statistic is refused.
5. The submitted object cannot drift: `study.submit` takes the plan address and an idempotency key.

**The estimate against the budget.** The estimate is 96 GPU-seconds with a 90% interval of [78, 121].
Admission uses the upper end of the interval: 121 ≤ 150, so the plan is admissible under its budget; under a
budget of 100 GPU-seconds the same plan is refused with the estimate attached (`FROB-REF-701`), although the
point estimate would fit.

**A result consistent with the plan** (`compare.expect.json`; from `study.compare`). b = 19 pairs where only B
delivers, a = 2 where only A does, 40 where both do, 3 where neither does, none unscored: 19 + 2 + 40 + 3 + 0
= 64. The intervals are [L_b, U_b] = [0.1764, 0.4418] and [L_a, U_a] = [0.0026, 0.1210], so the interval on
the difference is [0.0554, 0.4392]; its lower end exceeds 0.05, so the rule promotes B and `study.compare`
reports the metric as `settled`. The result stays `provisional` until it is promoted to the referee. It is a
development result. It is not a qualification.

## Acceptance criteria

- [ ] The specimen equals the text above byte for byte outside its `preanalysis` block, which is the
      `frob.preanalysis/1` record described above (the manifest SHA-256 covers the whole file); the resolved plan
      parses strictly, validates against `frob.plan/1` and round-trips with zero byte differences.
- [ ] `frob study plan manifest.json` returns a plan equal to the resolved fixture on every `exact` field; the
      call runs no physics (no run is created; `cost.gpu_s` is 0) and repeating it returns the same plan
      address (idempotent by construction).
- [ ] The pre-analysis equals the table: 2,145 outcomes; 521, 521 and 1,103; tie unreachable with the
      concordance interval [−0.0662, +0.0662]; first tie at n = 86; least b at a = 0 equal to 15.
- [ ] A manifest that names "tie" among its verdicts is refused with `FROB-REF-532`, exit 3; one that declares
      both study types with `FROB-REF-552`; one whose arms have different determinism classes, or an
      uncontrolled class, with `FROB-REF-533` (the paired-study refusal of `study.plan`: by the layer rule of
      N24-REF and N28-REF the Arena study verbs answer 552, 532 and 533 and never their Atelier twins 553, 555 and
      542); a submit of an unknown plan address, or of a plan bound to a scene pack retired after planning, with
      `FROB-REF-809`, exit 7, nothing charged; a submit without a key with `FROB-REF-805`.
- [ ] Under a budget of 100 GPU-seconds an estimate with upper end 121 is refused with `FROB-REF-701`, exit 4,
      the estimate attached; under 150 it is admitted. (The check uses the fixture's interval injected through
      the calibration fixture; real estimates are `illustrative`.)
- [ ] The rule evaluated at (a, b, n) = (2, 19, 64) gives the interval [0.0554, 0.4392] to four decimals and
      the verdict promote; at (0, 14, 64) inconclusive and at (0, 15, 64) promote.
- [ ] A submitted study's comparison satisfies a_only + b_only + both + neither + unscored = 64, reports each
      metric as `settled`, `tied` or `inconclusive`, and is labelled `provisional` until promoted.
- [ ] The plan address and the pre-analysis hash are identical on two runs, in debug and release, on Linux and
      macOS (class D0); once blessed they equal the map's entries for `plan:b3:52e8…` and `b3:5d10…`.

## Test plan

### Unit tests

`cargo test -p frob-arena --test golden_l8` and `cargo test -p frob-opt --test golden_l8` (the real planner,
enumerator and rule; exact integer counts; interval endpoints rounded outward):
- `l8_specimen_parses_and_validates`; `l8_manifest_hash`; `l8_roundtrip_value`.
- `l8_outcomes_2145`: the number of (a, b) with a + b ≤ 64 is C(66, 2) = 2,145.
- `l8_reachability_64`: 521 promote, 521 inferior, 1,103 inconclusive, 0 tie.
- `l8_reachability_86`: 3,828 outcomes; 1,031, 1,031, 1 and 1,765; the tie is (0, 0).
- `l8_concordance_interval`: (0, 0) at n = 64 gives an upper end of 0.0662 (1 − 0.0125^(1/64)); at n = 86,
  0.0497.
- `l8_least_b`: 15, 17, 19, 21, 23 for a = 0 to 4.
- `l8_result_19_2`: [L_b, U_b] = [0.1764, 0.4418], [L_a, U_a] = [0.0026, 0.1210]; difference
  [0.0554, 0.4392]; verdict promote.
- `l8_pair_accounting`: 19 + 2 + 40 + 3 + 0 = 64; a comparison record whose five counts do not sum to the
  planned pairs is rejected by the `PairCounts` constructor of N32a-13 as an integrity fault (`FROB-REF-990`,
  exit 9), never printed.
- `l8_admission_uses_upper_end`: interval [78, 121] against budgets 150 (admit) and 100 (refuse, code 701).
- `l8_plan_address_is_content_address`: reordering manifest keys keeps the address; changing n to 96 changes
  it.
- `l8_refusal_<code>` for 532, 533, 552, 805, 809; `l8_no_atelier_twin`: none of the planted plan requests is
  answered 542, 553 or 555 (the codes of the comparison layer).
- `l8_no_early_stop_on_evalue`: a fixed-n study offered an e-process stop is refused; it stops early only when
  every completion of the unresolved pairs gives the same verdict (for example b = 60, a = 0 after 60 pairs:
  the worst completion, four A-only pairs, still promotes).

### End-to-end script and logging

`scripts/e2e/art_l8_plan.sh` (header: `bead=ART-L8 milestone=M3 labels=golden,appendix-l,arena,
verb:study.plan,verb:study.submit,verb:study.status,verb:study.compare,verb:eval.preanalyze seed=1
needs=cpu,cuda,frobd plants=tie-requested,counts-off`). Steps: (1) create design A from artifact L.1's fixture
and design B by `frob design edit quad-tray set-param /limbs/of/of/chain/3/foot/region/sphere 0.030m
--expected-parent <A's DesignId> --key l8-edit --json` (a `mutate` verb: key and expected parent; the pointer is
the foot-radius lever of artifact L.1, and 0.030 m lies in its budget [0.018, 0.032] m); create the task
from artifact L.3's fixture; (2) `frob eval preanalyze --rule paired-clopper-pearson/1 --n 64 --margin 0.05
--level 0.05 --json`, assert the table with `e2e_assert_num` (exact); (3) `frob study plan manifest.json --key
l8-plan --json`, compare with the resolved fixture field by field and assert that no run was created;
(4) repeat step (3) and assert `idempotent_replay` or an identical address; (5) the refusals through
`e2e_expect_refusal` (532 for the tie request, 533 for arms in different determinism classes, 552 for both
study types, 805 for a keyless submit, 809 for a submit of an unknown plan address); (6) `frob study submit <plan> --key l8-sub
--sync-wait-ms 5000 --json`, follow the job with `frob job wait`, then `frob study status <study> --json` (assert complete, with the
planned pair count); (7) `frob study compare <study> --json`,
assert the pair accounting, that the verdict equals the rule applied to the reported counts (recomputed by the
script's helper), and the label `provisional`; (8) `frob-golden check --artifact L8`. Plants: `tie-requested`
must be refused; `counts-off` alters one count in the comparison record and must fail the accounting
assertion. The log (`target/e2e-logs/art_l8_plan/<utc>.ndjson`) records every command with exit code and wall
time, the evidence block (Design comparison, class, tier, determinism D2, identities, seed), the pre-analysis
counts and the pair counts as metrics with expected values, the interval endpoints, refusals with code and
first repair, and the `summary`. In small mode the study is submitted with 8 pairs on `batch-cpu`, the
pre-analysis and plan assertions are unchanged, and the log says that nothing is claimable.

### Known answers and negative controls

- KA-612 and KA-693 (the fixed-n rule at n = 64: 2,145 count pairs, 521 prefer, 521 inferior, 0 ties, 1,103
  inconclusive; a tie first reachable at n = 86 and only for (0, 0); with no A-only pair a preference needs 15
  B-only pairs of 64, and 17, 19, 21, 23 with one to four A-only pairs; exact; control: a request for a tie
  verdict at n = 64 is refused). KA-090 (perfect concordance at n = 64 gives [−0.0662, +0.0662]). KA-757 (pair
  accounting: a_only + b_only + both + neither + unscored = planned).
- Known answer of this bead (recomputed for it): the interval [0.0554, 0.4392] at (a, b) = (2, 19). It is row
  JL-17 of NJLx-02's known-answer block (a dependency), which this bead's tests cite by number once the block is
  issued, with the row's controls (labels A and B swapped without negating the interval; a one-sided bound
  printed as two-sided).
- Counterexample kept alive (CX-266, BD-124): stopping a fixed-n study when its running test first looks good. An exact
  one-sided sign test at nominal level 5%, looked at after every pair, falsely rejects with probability
  13/256 = 5.08% after 8 looks, 8.76% after 16, 12.9% after 32 and 17.4% after 64.
- Negative controls: the two plants; the five refusals; a planner that omits the pre-analysis must be refused.
- Metamorphic relation: swapping the labels A and B swaps promote and inferior counts and negates the
  interval.
- Determinism lanes: the plan and the pre-analysis twice on one machine, in debug and release, on Linux and
  macOS (class D0); the study at class D2 on one backend, twice.

## Considerations

- Two refusal codes describe an unreachable verdict: `FROB-REF-532` (a fixed-n study has no pre-analysis, or
  names a verdict it cannot reach) in the evaluation chapter and `FROB-REF-555` (a declared verdict set that
  contains an unreachable verdict) in the studio chapter. The layer rule of N24-REF and N28-REF settles which
  verb answers which: `study.plan`, `study.submit` and `eval.preanalyze` admit through `study::admit` and answer
  532 (and 552, 533); `eval.compare` and `study.compare` admit through `pairing::admit` and answer 555 (and 553,
  542). The artifact names 532, and the script asserts 532 and 533 on `study.plan`.
- Outside the `preanalysis` block the field spellings of `frob.plan/1` are still the specification's proposals
  (`plan`, `designs` as a map of A and B, `controller_arm` as a sentence, `scenes`, `seed_protocol` with the
  unit `building`, `study`). The manifest that `study.plan` reads is not among them: it is the owner's format
  `frob.study.manifest/1` (N33b-19: `controller_arm` `shared`, `seed_protocol` with `unit: cluster` and its
  cluster map, the building being the cluster, and a `fixed_n` block), and `manifest.json` is written in it.
  When the plan beads (BD-124, N33b-19) fix the members of the plan itself otherwise, the specimen is
  re-entered once from the amended artifact bead.
- The study itself costs GPU time (about 96 GPU-seconds in the story) and needs the kitchen scene family and
  its split; the plan and pre-analysis lanes need neither and run on any host.
- The real study will not return 19 and 2; `compare.expect.json` holds the rule's output for those counts as a
  unit-level known answer, and the script asserts the rule on whatever counts the study reports.

## References

Appendix L.8; §32.6.1 (the two-phase study), §24.12 (study types and the pre-analysis), §24.10 (the rule and
its enumeration), §28.6, §32.9 (admission uses the upper end of the estimate); INV-ARENA-05 (a study has one
type for its whole life); INV-TETHER-04 (the five pair counts sum to the planned pairs); PO-ARENA-03 (the
enumerator is exact); GATE-ARENA-05 and GATE-ATEL-06 are measured by the study beads; KA-090, KA-612, KA-693,
KA-757.
