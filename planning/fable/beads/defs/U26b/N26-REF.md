---
key: "N26-REF"
title: "registry: the twelve refusal codes of Nemesis (360–362, 560, 563–569, 760) with triggers, repairs and exit codes"
type: "task"
priority: 2
milestone: "M3"
labels: ["m3", "nemesis", "frob-nemesis", "solid", "registry"]
parent: "WP-55"
deps: ["BD-005", "BD-022"]
needs: ["§32.4, §32.18: a ruling by the owner of the refusal registry on the exit code of the `76x` codes (FROB-REF-760 here), which the exit-code table places in neither the `70x` budget row nor the `72x`, `73x` capability row", "§26.3, §26.9: the beads that raise ten of the twelve codes (BD-270, N26a-02, N26a-23, N26a-21, BD-277 and BD-272 with N26a-17, drafted by unit U26a; seed BD-275, drafted by unit U15a)"]
related: ["N26b-13", "N26b-14", "BD-270", "BD-271", "BD-273", "BD-274", "BD-275", "BD-276", "BD-277", "BD-278", "N26b-03", "N26b-05", "N26b-11", "BD-259", "BD-299", "N26-KA", "N26-CX"]
gates: []
plan_refs: ["§26.12", "Appendix O", "§32.4", "§32.5", "§32.18", "§35", "§28.20"]
---
## Background

A refusal is a typed answer, not an error message: the verb was reached and declined, and the envelope says why
and what would make the request admissible. Every refusal has a stable code, a message and ranked repairs. The
machine-readable registry is `registries/refusals.toml`; an unknown code is a load-time error; codes are never
reused; the chapter that defines a code governs its meaning and its repairs. The envelope of a refusal carries
`code`, `message`, `pointer` (the field or part at fault), `observed`, `required`, `unit`, `affected_claims`,
`retry` (`never`, `after-change` or `backoff`) and ranked `repairs`. Every refusal code maps to exactly one exit
code: 3 for a refused input or question (the `1xx` to `5xx` families and most of `8xx`), 4 for budget (refused
at admission or stopped; nothing partial is published as a result), 5 for a missing capability or scope.

Nemesis defines twelve codes, in three families: scene (`3xx`: what may be searched), learning and evaluation
protocol (`5xx`: what data may support which statement) and budget (`7xx`). They are the enforcement points of
the chapter's decisions: search inside a legality set; only worlds a witness shows solvable count; exploration
and certification never share data; everything Nemesis finds is discovery data; screens are audited.

This bead is the registry of those twelve codes: the rows of `registries/refusals.toml`, each with meaning,
trigger, envelope fields, ranked repairs and exit code, and the test that every row is raised somewhere and
tested from the user's side.

## Scope

In: the twelve rows; the mapping to exit codes and retry classes; the cross-references to codes of other
chapters that Nemesis verbs raise or that restate a Nemesis rule; the list of refusal-like situations for which
the plan names no code; the registry tests.
Out: raising the codes (the mechanism beads named per row); the end-to-end exercise of each code (N26b-14).

## Technical approach

**The twelve codes.** Message text is the meaning column, verbatim. All have exit code 3 except FROB-REF-760.

| Code | Meaning | Trigger (where it is raised) | `pointer`; `observed`; `required` | Ranked repairs | Retry | Exit |
|---|---|---|---|---|---|---|
| FROB-REF-360 | A world or edit lies outside the legality profile; the violated bound is named | The legality check, on any single edit and again on the final world (two individually legal obstacles can narrow a passage below its minimum width); an edit to a path the profile does not list. Raised before any episode runs, by every Nemesis verb that takes a world | the edit's index and path, or the final-world quantity; the value; the bound with its provenance class | (1) move the value inside the named bound (the nearest legal value is attached); (2) drop the named edit; (3) use a profile that declares the wider bound, with its provenance | after-change | 3 |
| FROB-REF-361 | The request demands `cited` bounds and the profile has an `assumed` bound on a touched quantity | `--require-cited`, or a report that claims a counterexample is realistic, with any touched bound of provenance class `assumed` | the touched bound; `assumed`; `cited` or `card` | (1) rerun without the demand and accept the label; (2) record a source for the bound in the source register; (3) restrict the search to quantities with `cited` or `card` bounds | after-change | 3 |
| FROB-REF-362 | An edit does not declare the causal paths it changes | The declaration check of a typed edit: `changes` empty or with an unknown tag; the edit class's default tag missing; an aspect of the compiled scene whose digest changed and whose tag is not declared; several changes bundled in one edit | the edit's index and path; the declared tags; the tags found by the per-aspect digests | (1) add the listed tags; (2) split the edit into one edit per mechanism; (3) choose a different quantity | after-change | 3 |
| FROB-REF-560 | A bound was offered episodes that were not drawn nominally or with recorded bounded weights | The bound's admission rule: an episode whose proposal law is a search, a tilt, a fork, a given world, a battery selection or a hard-world set; a defensive-mixture episode with a missing weight or a weight above 1/ε; a subset chosen after outcomes | the first offending episode; its proposal law; `nominal` or `defensive-mixture` with a recorded weight | (1) draw fresh nominal episodes under a new commitment; (2) remove the offered set and let the verb sample; (3) use the offered episodes for exploration only | after-change | 3 |
| FROB-REF-563 | Importance sampling outside its admitted law, or with a proposal fitted on the run it is applied to | A randomized quantity with no declared monotone transform to normal germs; a law that is neither independent nor jointly Gaussian in germs; a proposal whose fitting campaign overlaps the committed range | the quantity or the proposal; its law or its `fitted_on`; an admitted law, or a separate completed pilot | (1) declare the transform to normal germs; (2) fit the proposal on a separate completed pilot and sample with a fresh key; (3) fall back to crude nominal sampling | after-change | 3 |
| FROB-REF-564 | Shrinking requested for a failure that does not replay, or that the guard labelled an artifact candidate | `nemesis.shrink` (and the shrink test relation): seed 0 of the full edit set does not reproduce its recorded state digests on the originating backend; or the artifact guard's label is `artifact-candidate` | the run; the first differing step, or the variant that removed the failure; identical digests and a passed guard | (1) rerun the guard after the simulator defect is fixed; (2) replay on the originating backend and numerical profile; (3) shrink a different failure | after-change | 3 |
| FROB-REF-565 | Coverage selection on a stale incidence | `nemesis.cover`: an identity the incidence was built under (model class, tier, kernel versions, failure taxonomy; also severity table and task version) differs from the identity in force | the incidence; the built identity; the identity in force | (1) rebuild the incidence under the identities in force; (2) use the standard battery, the fallback of the covering selection; (3) pin the request to the built identities to reproduce an earlier selection as a historical record only | after-change | 3 |
| FROB-REF-566 | A planted winner's rank is not on record from a full evaluation | `nemesis.audit prepare`: the proposed plant has no ledger record of a final-stage (full) evaluation, or its rank is outside the top k, or the record's task family, robot class or tier is not the campaign's | the design; the best record found (none, an earlier stage, a rank, a mismatched identity); a final-stage rank within the top k | (1) run the full evaluation of the design first, then plant it; (2) choose a planted winner of the design corpus that has a record; (3) run the audit without a plant: the control is reported `not-run` and finalists are described as "k survivors" | after-change | 3 |
| FROB-REF-567 | A hard-world or search-selected world offered as a confirmation sample | The qualification lane, at commit and at citation, and the hard-world set's own guard: a request, a confirmation family or a cited result contains a world found in the discovery record (hard-set members including retired ones, counterexample files, search-selected addresses, items of the `hard` and `covering` batteries); a hard-set run cited as confirmation | the world; `discovery (origin nemesis)`; a world of a sealed family | (1) draw confirmation worlds from a sealed world family through the qualification verb; (2) cite the result as a regression or discovery result; (3) remove the world from the request | never (for that world) | 3 |
| FROB-REF-568 | The failure predicate changed after sampling began | The bound's admission rule: an episode's predicate identity (the hash of task identity, requirement name and failure predicate) differs from the predicate committed when the bound was opened | the episode or the request; its predicate identity; the committed one | (1) start a new bound under the new predicate with fresh episodes; (2) restore the committed task identity | after-change | 3 |
| FROB-REF-569 | Splitting requested where no randomness enters after the fork | The splitting explorer's admission: the task has no registered per-step random stream that is active after the earliest possible fork tick and not held fixed by the request (a fork would replay its parent identically) | the task's stream registry; the streams active after the fork; at least one | (1) use tilting or the defensive mixture, which act on static parameters; (2) enable a per-step disturbance the task declares; (3) use crude sampling | after-change | 3 |
| FROB-REF-760 | The episodes a claim needs at its level exceed the budget; the bound reachable within budget is attached | `nemesis.bound --mode certify` at admission: the required n = ⌈ln δ / ln(1 − p⋆)⌉ at the request's level exceeds the n_b the budget affords | the budget; n_b; n; attachment `reachable_bound = 1 − δ^(1/n_b)` | (1) raise the budget to the stated amount; (2) ask for the reachable bound instead; (3) certify a decade higher and report the target as `Estimated` | after-change | 4 |

Worked attachment for FROB-REF-760: a target of 10⁻⁵ at δ = 0.05 needs 299,572 failure-free nominal episodes; a
budget that affords 100,000 yields `reachable_bound` = 1 − 0.05^(1/100000) = 2.996 × 10⁻⁵.

**Exit code of FROB-REF-760 (DESIGN, pending a ruling).** The exit table names `70x` under exit 4 (budget) and
`72x`, `73x` under exit 5 (capability); `76x` is in neither. The code is a budget refusal at admission with the
affordable alternative attached, so it is registered with exit 4, beside its neighbour FROB-REF-761 (a job
exceeds the compute envelope in force). The parity gate requires every code to map to exactly one exit code;
the row is flagged until the owner of the registry confirms.

**Codes of other chapters at Nemesis boundaries (not owned here; named so a reader is not surprised).**
FROB-REF-840 (a session without the qualification administration scope asks for sealed confirmation data:
raised by every Nemesis verb for a world address in the confirmation namespace). FROB-REF-549 (a rare-failure
bound over battery-selected or search-selected episodes: the cascade's code for the rule that FROB-REF-560
enforces at the bound). FROB-REF-546 (calibration from survivors, or an audit episode cancelled or re-weighted:
the cascade's code beside the screening audit). FROB-REF-312 (a realism claim cites a scene with `assumed`
dimensions: the scene chapter's sibling of FROB-REF-361). FROB-REF-724 (the verb is not implemented in this
build: `nemesis.audit` before M4).

**Situations the plan gives no code (each a completed result or an unnumbered refusal today; flagged).**

| Situation | Present behaviour | Where |
|---|---|---|
| A hard-world set `add` whose admission check fails; a battery whose only content is hard worlds | Refusal with exit 3, asserted by pointer and failed check | the hard-world-set bead (BD-275) |
| A rare-failure bound requested for a design that is not a finalist | Refusal with exit 3 | the decision-policy bead for rare failures |
| A screening audit for a stage whose plan was not committed before its scores | Completed result `not-run` with the reason, exit 2 | N26b-05 |
| Shrinking a failure whose world has no solvability witness | Completed result `not-shrunk`, exit 2 | BD-274 |
| A coverage selection whose certificate fails its own check | The fallback battery with a flag, exit 2, and an integrity finding | BD-278 |

Free numbers exist in the chapter's blocks (363 to 369; 570 to 579). Assigning any is the registry owner's act;
until then tests assert exit code, pointer and failed check, never a number.

**Registry file.** Twelve `[[refusal]]` rows in `registries/refusals.toml`: `code`, `family`, `message`,
`defined_in = "§26.12"`, `exit`, `retry`, `repairs = [...]` in rank order, `raised_by` (crate and function),
`tested_by` (unit test names and the end-to-end step). The loader rejects a row without repairs.

## Acceptance criteria

- [ ] `registries/refusals.toml` contains the twelve rows with the meanings above verbatim, at least two ranked
      repairs each, a retry class and an exit code; the loader accepts the file.
- [ ] Every row names a `raised_by` function that exists and a `tested_by` unit test that exists (checked by the
      registry test); every row is exercised from the user's side by the refusal script (N26b-14) or, for
      FROB-REF-566, by the audit script (N26b-07).
- [ ] Each code maps to exactly one exit code: 3 for eleven of them, 4 for FROB-REF-760; the flag on 760's exit
      code is cleared only by a recorded ruling.
- [ ] No Nemesis code path emits a refusal code outside this table and the cross-referenced list.
- [ ] The five uncoded situations are listed in the crate's `CONTRACT.md` under known gaps.

## Test plan

### Unit tests
`crates/frob-nemesis/tests/refusal_registry.rs`:
- `twelve_rows_present_and_verbatim` (the meaning strings compared byte for byte).
- `repairs_ranked_and_nonempty`; `exit_codes` (eleven 3s, one 4); `retry_classes`.
- `raised_by_and_tested_by_resolve` (function paths and test names found in the crate).
- `no_unregistered_code_emitted`: every `Refusal::new` call site in the crate uses a code from the allowed set.
- `envelope_shape_per_code`: for each code a constructed refusal has `pointer`, `observed`, `required` and
  `repairs` filled as in the table (twelve cases).
- `ref_760_attachment`: budget 100,000 episodes, p⋆ = 10⁻⁵, δ = 0.05 → `required` 299,572, `observed` 100,000,
  `reachable_bound` 2.996 × 10⁻⁵ (to four significant digits, rounded upward).

### End-to-end script and logging
`scripts/e2e/nemesis_refusals.sh` (N26b-14) triggers each code with a planted request and its admissible twin,
and logs for each the envelope's `refusal` block, the exit code and the first repair.

### Known answers and negative controls
KA-659 (a planted illegal edit is refused with FROB-REF-360 and the bound named; control: a profile whose bound
is widened admits it). The chapter's verification items "a tilted episode offered to the bound is refused"
(FROB-REF-560) and "a walk with no noise after the fork is refused" (FROB-REF-569). Negative controls: for each
code, the admissible twin of the planted request must not be refused (a refusal that fires on everything
protects nothing); a registry row with an empty repair list must fail the loader.

## Considerations

- Two codes of two chapters guard one rule in two places (FROB-REF-560 at the bound, FROB-REF-549 at the
  cascade). They are kept distinct because the caller and the repair differ; the cross-reference prevents a
  reader from thinking one is redundant.
- FROB-REF-567's retry class is `never` for the offending world: a world that has been discovery data cannot
  become a confirmation sample by any change to the request.
- A refusal's repairs are part of its contract. Changing a repair list is a registry change with a recorded
  reason; adding a code never renumbers another.

## References

Plan §26.12 (the refusal table), Appendix O (the registry: `3xx` scene, `5xx` learning and evaluation protocol,
`7xx` budget and capability), §32.4 (envelope; exit codes), §32.5 (failure classes), §32.18, §35, §28.20
(FROB-REF-546, -549). KA-659.
