---
key: "N27-CX"
title: "Autopsy counterexample registry: CX-296 to CX-301, their construction-level equivalents and two late cases"
type: "task"
priority: 2
milestone: "M3"
labels: ["m3", "autopsy", "frob-conformance", "frob-autopsy", "solid", "registry", "test"]
parent: "WP-56"
deps: ["BD-025", "N38a-40", "N38b-05", "STD-TEST"]
needs: []
related: ["N27-KA", "N27-REF", "N27a-02", "N27a-07", "N27a-09", "N27a-11", "N27a-12", "N27a-13", "N27a-14", "N27a-16", "N27a-18", "N27b-03", "N27b-05", "N27b-06", "N27b-08", "N27b-09", "N27b-14", "N27b-15", "N27b-20", "N27b-21", "N27b-23", "SPEC-C-29"]
gates: []
plan_refs: ["Appendix N N.24", "§27.5", "§27.7", "§27.8", "§27.10", "§27.13", "§27.19", "§10 C-29", "§0.7", "§38.11"]
---
## Background

Counterexamples are first-class in this program: where a plausible shortcut is wrong, the smallest
hand-checkable case that breaks it is written down, enters the counterexample catalogue, and seeds a negative
control: a test in which the shortcut is planted and must be caught. A rule that ships without such a case has
not been thought through, and a gate that nothing can fail validates nothing.

Autopsy (crate `frob-autopsy`: counterfactual debugging by forks, interventions and a placebo) invites a
particular family of shortcuts, all of which make an attribution say more than its experiments support: reading
a summary value as a cause, treating any flip as an effect, testing a design change in the middle of an episode,
bisecting on a predicate that is not monotone, calling a contrast a proof. The catalogue gives Autopsy six
numbered entries, CX-296 to CX-301. Three of them restate, with the Autopsy specification's numbers, cases first written for
the construction "a deterministic forkable simulator is a causal model" (CX-069, CX-070, CX-071); the
equivalence is recorded so that neither copy drifts. The single-precision lane, added to the Autopsy specification later,
states one further counterexample that has no catalogue number, and drafting found one more case worth pinning;
both are registered under provisional tags so that neither is lost.

Tier Solid. Mechanism beads cite the numbers and state the values inline; this bead holds the rows as data and
binds each to the control test that realizes it.

## Scope

In: the six Autopsy entries and the two late entries in the suite's one catalogue,
`crates/frob-conformance/registry/counterexamples.toml`, in the catalogue schema of N38b-05 (a per-chapter file
such as `registries/counterexamples/autopsy.toml` would be a second copy, which the catalogue lint refuses); the
request for a block of two `cx` numbers for the late entries in the allocation ledger of N38a-40; the
construction-level links; the naming of each entry's catching bead and control test, with its plant; the
registry check.
Out: the control tests' implementations (in the beads named); the `controls.toml` records and their NC numbers,
which N38b-05's seeding tool writes from these entries (this bead writes entries, never control records);
counterexamples of other subsystems that Autopsy's tests also use (listed in the third table for reference
only).

## Technical approach

Entry fields, exactly N38b-05's catalogue record (the loader rejects any other key): `id`, `shortcut`, `case`
(with its numbers), `numeric`, `rule`, `rule_ids`, `defined_in`, `same_as`, `label`, `control_kind`, `plant`,
and `slug` for the late entries. The bead-and-test binding of the tables below is carried by `plant` (the build
flag or fixture request that puts the shortcut into the real component) and by tagging each control test with the
entry's identifier (`#[cx("CX-298")]`), which is how the negative-control registry finds the test that must turn
red.

Field values per entry (DESIGN where the catalogue does not state them):

| Entry | `numeric` | `rule_ids` | `control_kind` | `plant` |
|---|---|---|---|---|
| CX-296 | true | GATE-AUTOPSY-04, ADR-AUTOPSY-05, C-29 | `mutation` | build flag `autopsy-label-from-shapley` (labels taken from the largest Shapley value) |
| CX-297 | true | GATE-AUTOPSY-04, PO-AUTOPSY-01, C-29 | `mutation` | build flag `autopsy-repair-monotone` (a repair search that assumes adding an intervention never loses a rescue) |
| CX-298 | true | GATE-AUTOPSY-06, INV-AUTOPSY-03, FROB-REF-581, C-29 | `planted-input` | two plant records: the thermal fixture's request `--do 'model.joint[m].gear_ratio := 16'` at the fork 1 s before the trip, which must be refused with FROB-REF-581, with `removed_check` = "the structural and design-parameter check of the fork classifier" (BD-025's rule 5: a planted input whose expectation is the product's own refusal names the check disabled in its build); and the build flag `autopsy-forced-fork-answers` for the mutant (kind `mutation`) |
| CX-299 | true | GATE-AUTOPSY-07, PO-AUTOPSY-02 | `mutation` | build flag `divergence-pointwise-midpoint` |
| CX-300 | true | GATE-AUTOPSY-05, INV-AUTOPSY-05, C-29 | `planted-world` | the doubling-map fixture at forty iterates, with the build flag `autopsy-no-placebo` for the mutant |
| CX-301 | false | FROB-REF-880, GATE-SAFE-08 | `planted-input` | the request "cite this attribution as established", which must be refused with FROB-REF-880, with `removed_check` = "the citation guard's class check (N27b-15's `offer`)" (BD-025's rule 5, as for CX-298) (the concrete planted defect N38b-05 asks the owning chapter to supply for this word-only entry) |
| late-01 | true | GATE-AUTOPSY-09 | `planted-world` | the pendulum fixture released 5 × 10⁻⁸ rad from upright, with the build flag `lane-admit-reads-1e12` |
| late-02 | true | GATE-AUTOPSY-05 | `mutation` | build flag `perturb-fixed-size` |

**Planting beads.** Each plant's `id` has N38b-05's form `<planting bead>:<name>`. The planting bead declares
the build flag as a Cargo feature of the crate that holds the component (a per-control feature, never part of
`gauntlet-mutations`, BD-025), or writes the planted request as a fixture with its `synthetic` header:
`N27b-06:autopsy-label-from-shapley` (`frob-autopsy`); `N27b-14:autopsy-repair-monotone` (`frob-autopsy`);
`BD-279:cx-298-gear-change` (the planted request) and `N27a-17:autopsy-forced-fork-answers` (`frob-autopsy`,
the same feature as the plant of KA-675's control); `N27a-09:divergence-pointwise-midpoint` (`frob-chronicle`,
the same feature as the plant of KA-677's control); `BD-282:autopsy-no-placebo` (`frob-autopsy`, the same
feature as GATE-AUTOPSY-05's first control); `N27b-15:cx-301-cite-established` (the planted request);
`N27b-23:lane-admit-reads-1e12` (`frob-autopsy`); `N27b-03:perturb-fixed-size` (`frob-autopsy`). A feature
shared with a known-answer control builds one defect for two controls, one per protected rule.

`defined_in` is §27.7, §27.7, §27.8, §27.10, §27.5, §27.13, §27.19 and §27.5 in table order; `label` is DESIGN for
every entry (each case is derived in this program, none is imported); `same_as` is empty for all eight (below).

**Construction-level links.** CX-296, CX-298 and CX-300 restate, with Autopsy's numbers, the construction's
entries CX-069, CX-071 and CX-070. DESIGN: they are not linked by `same_as`. The catalogue marks thirteen
`same_as` pairs and N38b-05's loader asserts that count; these three pairs are stated in different words and with
different detail (CX-071 has no numbers), protect different rules, and so are separate entries with separate
controls, as the catalogue's rule "one control per protected rule" requires. The link is kept by `rule_ids`
naming C-29 and by this bead's test `construction_forms_agree`, which checks that the shared numbers of each pair
are equal (the (½, ½) values of CX-296 and CX-069).

**The six catalogue rows.**

| CX | The tempting shortcut | The smallest case that breaks it | The rule it protects | Control (bead: test) and planted mutant |
|---|---|---|---|---|
| CX-296 | A Shapley summary tells joint causes from alternative causes | Two factors that rescue only together (v of the pair 1, of each single 0) and two that each rescue alone (v = 1 for either single and for the pair) both have Shapley values (½, ½); the repair sets are {1, 2}, and {1} or {2} | The coalition table is the result; the summary is a summary | N27b-06: `joint_pair`, `alternative_pair`; N27b-14: `joint_pair_repair_set`, `alternative_pair_repair_sets`; N27b-20: `shapley_hand_cases`. Mutant: labels taken from the largest Shapley value. Equivalent to CX-069 |
| CX-297 | A positive Shapley value shows a factor is necessary, and values are nonnegative | v(∅) = 0, v({1}) = 1, v({2}) = 0, v({1, 2}) = 0 gives (½, −½): the values sum to v(N) − v(∅) = 0, a single-factor repair exists, and factor 2 is negative because applying it cancels the rescue | Labels `but-for`, `joint`, `alternative` come from the table; a Shapley value never establishes necessity | N27b-14: `cancelling_factor`; N27b-20: `shapley_hand_cases`, `shapley_efficiency_dummy_symmetry`. Mutant: a repair search that assumes adding interventions never loses a rescue |
| CX-298 | Apply a design edit at a mid-episode fork to see whether it prevents the failure | One thermal node, time constant 60 s, ambient 25 °C, trip at 115 °C. At gear ratio 12 the steady-state rise is 192 K and the motor trips at 60·ln(192/102) = 37.95 s of a 45 s task. Heating scales with the inverse square of the ratio, so at 16 the rise is 108 K. Ratio 16 from the start peaks at 25 + 108·(1 − e^(−0.75)) = 82.0 °C and passes with 33 K in hand; ratio 16 applied at 36.95 s finds the winding at 113.3 °C, still heading for 133 °C, and trips at 42.4 s. The fork is not wrong about the experiment it ran; it ran the wrong experiment | Design parameters re-run from the episode start; a structural or design-parameter intervention at a mid-episode tick is refused (FROB-REF-581) except in the labelled diagnostic mode; GATE-AUTOPSY-06 | BD-279: `gear_change_refused_581`, `diagnostic_force_is_labelled`; N27b-21: `thermal_closed_form`; the re-run bead's thermal tests. Mutant: a tool that reports the forced fork's result (a trip at 42.4 s) as the answer to the design question. Equivalent to CX-071 |
| CX-299 | Bisect for the first tick at which two traces differ | Two traces of eight ticks differ at ticks 1 and 5: a midpoint search on the pointwise predicate probes tick 3 or 4, sees agreement and reports tick 5; the chained digest reports tick 1 | Bisection only on a monotone, decided predicate; the same rule for `design.bisect` over a lineage, where the predicate must be "the outcome has changed by this edit" and each step must be decided by a study | N27b-20: `eight_tick_trap`, `pointwise_midpoint_mutant_reports_5`, `first_divergence_exact_vs_scan`; the first-divergence bead's own tests. Mutant: the pointwise midpoint search |
| CX-300 | Any flip under an intervention is attribution | Doubling map x ← 2x mod 1 with failure on the fortieth iterate: a 10⁻¹² placebo is amplified by 2⁴⁰ ≈ 1.1 × 10¹² and about half of placebo forks flip | Placebo forks and the label `chaotic: unattributable` (no factor is named when the placebo flip rate is above 0.2) | N27b-03: `doubling_map_chaotic`, `doubling_map_five_iterates`; N27b-06: `chaotic_overrides_everything`; N27b-05: `chaos_threshold_integer`. Mutant: an attribution that subtracts no placebo. Equivalent to CX-070 |
| CX-301 | A contrast between forks is a proof of cause | A contrast is a replayable experiment of class `Tested` on its tier (the catalogue gives no numeric case); an attribution drawn from contrasts is `Estimated`, scoped to the model, the tier and the listed factors | The non-claims of the Autopsy specification; FROB-REF-880 (an attribution cited as an established cause, or as a cause in the physical robot) | BD-280: `evidence_is_estimated`; N27b-15: `offer_established_880`, `offer_reality_880`, `no_upward_conversion`. Mutant: a citation rule that allows `established` when the contrast is 1.0 with zero flips |

**Late cases, registered under provisional tags.**

| Tag | The tempting shortcut | The smallest case that breaks it | The rule it protects | Control and mutant |
|---|---|---|---|---|
| `late-01` (stated by the single-precision lane) | A clean placebo series at 10⁻¹² shows a verdict is stable | A pendulum released 5 × 10⁻⁸ rad from upright with no control, verdict "falls to the positive side". A placebo of 10⁻⁷ rad flips the side whenever its sign is negative, so ρ₀ is about one half; at 10⁻¹² the side is stable, so the 10⁻¹² series alone would have hidden the problem | Admission to the single-precision lane is measured at 10⁻⁷ on the originating tier (ρ₀ ≤ 0.1 and G(W)·u ≤ 10⁻⁴); the label is `marginal`; every attribution reports its flip rates at both scales | N27b-23: `pendulum_marginal`, `pendulum_stable_at_1e12`; N27b-21: `pendulum_sign_decides`. Mutant: an admission that reads the 10⁻¹² series |
| `late-02` (found while drafting the perturbation rule) | One fixed perturbation size is enough for a placebo | On the doubling map a fixed 10⁻¹² shifts the fortieth iterate by 10⁻¹²·2⁴⁰ mod 1 = 0.0995 in every fork, so only about 20% of placebo forks flip (a failing world flips only if its fortieth iterate lies within 0.0995 of a boundary), which sits on the 0.2 threshold; with sizes drawn on [1, 2)·10⁻¹² and a drawn sign the shift wraps the interval and 0.45 to 0.51 flip | Perturbation sizes are drawn on [1, 2)·ε with a drawn sign, by address | N27b-03: `doubling_map_chaotic` (band 0.3 to 0.7). Mutant: a generator with a fixed size |

**Entries of other subsystems that Autopsy's tests use (not owned here).**

| CX | Case, in brief | Used by |
|---|---|---|
| CX-069 | Either-alone and only-together both give φ = (½, ½); the first has two but-for causes with PN = 1 and the second has none | The construction's form of CX-296 |
| CX-070 | On a fixture where every last-place nudge of an unrelated state flips the outcome, any intervention appears to prevent the failure | The construction's form of CX-300 |
| CX-071 | For a gear ratio or a mass, the thermal state, the estimator state and the policy's history at the snapshot would all differ under the changed design | The construction's form of CX-298 |
| CX-288 | Delta debugging is 1-minimal only: with edits (a, c, d, e, b) and failure when {a, b} or {c, d, e} is present, it ends at {c, d, e}; the one- and two-edit subset check (78 tests at most for 12 edits) finds {a, b} | N27b-14: `order_dependence_then_pair_check` |
| CX-321 | The 60 mm wheel's warm start grafted onto the 75 mm wheel is refused (FROB-REF-582) | BD-279: `warm_start_graft_refused_582` |

**Late entries and their numbers.** This bead files one request in
`crates/frob-conformance/registry/allocations.toml` (`registry = "cx"`, `count = 2`, `owner = "N27-CX"`,
`section = "§27.19, §27.5"`); nobody types `first`. Until the block is issued the entries carry `slug =
"late-01"` and `"late-02"`; when it is issued, the tags are replaced by the two numbers in table order in one
commit, in the catalogue, the test tags and the derivation files, and a tag still present at this bead's
milestone exit fails N38a-40's check `provisional-tag-past-due`.

**When an entry is runnable.** An entry whose catching bead (the bead that owns its control test) is still open
is loaded as an obligation, with the catching bead named, and makes no gate eligible; it becomes runnable when
that bead closes with its tagged test. Every catching bead here is an M3 bead of the Autopsy work package, so all
eight entries must be runnable by the Autopsy exit review (N27b-30). CX-301 stays an obligation under N38b-05's
rule for entries with `numeric = false` until its plant above is accepted by that bead's seeding tool.

The registry check (`crates/frob-conformance/tests/counterexamples_autopsy.rs`) loads the catalogue with
N38b-05's loader and fails when an Autopsy entry is missing or carries a key outside the schema, when a runnable
entry has no tagged control test or its test did not run in the suite, when an entry's numbers differ from the
constants of its derivation file, or when a late entry still has its tag at the milestone exit.

## Acceptance criteria

- [ ] The six numbered entries and the two late entries are in `crates/frob-conformance/registry/counterexamples.toml`
      with every field of N38b-05's schema; N38b-05's loader accepts the file and counts 6 for the Autopsy
      chapter; no file `registries/counterexamples/autopsy.toml` exists.
- [ ] The request for two `cx` numbers is in the allocation ledger; once the block is issued, no `late-0n` tag
      remains in the catalogue, a test name or a derivation file.
- [ ] Every entry whose catching bead is closed has a control test tagged with its identifier that exists, ran,
      and fails when its plant is enabled (each plant is enabled once in the registry's own test and must turn
      the control red); every other entry is an obligation naming its catching bead.
- [ ] The numbers in the entries are reproduced by the tests: (½, ½) twice; (½, −½) summing to 0; 37.95 s,
      113.3 °C, 42.4 s, 82.0 °C; tick 1 against tick 5; a placebo flip rate of about one half on the doubling
      map; ρ₀ of about one half for the pendulum at 10⁻⁷ and zero at 10⁻¹².
- [ ] `construction_forms_agree` passes for CX-296/CX-069, CX-298/CX-071 and CX-300/CX-070.
- [ ] The registry check fails when a test tag is removed from a runnable entry's control (shown once).

## Test plan

### Unit tests
`crates/frob-conformance/tests/counterexamples_autopsy.rs`: `entries_load_with_n38b05_schema` (and a scratch
entry carrying `equivalent_to` or `control` is rejected); `autopsy_chapter_counts_6`; `controls_exist_and_ran`
(runnable entries); `obligation_until_catcher_closes` (a scratch entry whose catching bead is open loads as an
obligation, and the same entry marked runnable with no tagged test fails); `numbers_match_derivations`;
`construction_forms_agree`; `late_block_requested` (`owner = "N27-CX"`, `count = 2`); `tag_past_due_fails`
(a scratch copy at the milestone with `late-01` left in a test name); `plants_turn_controls_red` (one
sub-test per runnable entry, each enabling its plant through the test-only feature of the owning crate and
asserting the tagged control fails); `removing_a_tag_fails_the_check`.

### End-to-end script and logging
No script of its own. The end-to-end scripts tag their `assert` lines with the counterexample they exercise
(`cx: "CX-298"` on the gear-change refusal, `cx: "CX-300"` where a chaotic placebo is asserted, `cx: "CX-301"`
on the citation refusal), and the registry's test run writes one NDJSON line per entry with `ts`, `bead`
(`N27-CX`), `id` (or slug), `state` (runnable or obligation, with the catching bead), the control test, the
plant, and the outcome of the control with and without the plant, then a `summary` line with counts of
runnable entries, obligations and controls caught.

### Known answers and negative controls
Each row is itself a negative control. Known answers that share cases with the rows: KA-672 (CX-300), KA-673
(CX-296), KA-674 (CX-297), KA-675 (CX-298), KA-677 (CX-299), KA-146 and KA-147 (the construction's forms).
Determinism: the cases are exact arithmetic or closed forms; the controls give the same verdict on every
platform.

## Considerations

- CX-301 has no numeric case in the catalogue. Its control is a type-level and refusal-level test; if a numeric
  case is wanted later, the natural one is a contrast of 1.0 with zero flips on a model whose friction card is
  `assumed`, which the claim product already reduces to `Estimated`.
- `late-02` is a property of the fixture and the generator, not of physics; it is registered because a future
  simplification of the generator to a fixed size would silently weaken the chaos test.
- If KILL-AUTOPSY removes coalitions and Shapley summaries, CX-296 and CX-297 keep their entries (the shortcut
  remains tempting) and their controls reduce to the label and repair tests that survive. If KILL-ENSEMBLE
  fires, late-01 keeps its entry (its rule, "measure stability at 10⁻⁷", still governs the 10⁻⁷ series of every
  attribution) and its control moves to N27b-03's series tests.
- Numbers are never reused: an entry withdrawn because its mechanism's kill gate fired is marked void in its
  block (N38a-40), not deleted.

## References

Plan Appendix N (N.24: CX-296 to CX-301), §27.5, §27.7, §27.8, §27.10, §27.13, §27.19, §10 (C-29: CX-069 to
CX-071), §0.7, §38.11. KA-146, KA-147, KA-672 to KA-677; FROB-REF-581, -582, -880. Catalogue schema N38b-05;
number blocks N38a-40.
