---
key: "N26-REF"
title: "registry: the twelve refusal codes of Nemesis (360–362, 560, 563–569, 760) with triggers, repairs and exit codes"
type: "task"
priority: 2
milestone: "M3"
labels: ["m3", "nemesis", "frob-nemesis", "solid", "registry"]
parent: "WP-55"
deps: ["BD-005", "BD-022", "N35-33", "N35x-02"]
needs: []
related: ["BD-275", "N24-REF", "N24b-16", "N26a-01", "N26a-02", "N26a-05", "N26a-16", "N26a-17", "N26a-21", "N26a-23", "N26a-26", "N26a-30", "N26a-31", "N26a-34", "N26b-01", "N26b-03", "N26b-04", "N26b-07", "N26b-08", "N26b-11", "N26b-12", "N26b-17", "N26b-19", "N26b-21", "N26b-22", "N26b-23", "VERBS-nemesis"]
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
at admission or stopped; nothing partial is published as a result), 5 for a missing capability or scope. The
mapping is data: the `[exit_rule]` table of the one refusal registry (N35-33) gives one exit per sub-family
(`1xx` to `5xx` → 3; `70x` → 4; `72x`, `73x` → 5; `74x` → 4; `76x` → 4; `79x` → 5), and a row whose exit
differs from its sub-family's fails the load.

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
| FROB-REF-560 | A bound was offered episodes that were not drawn nominally or with recorded bounded weights | The bound's admission rule: an episode whose proposal law is a search, a tilt, a fork, a given world, a battery selection or a hard-world set; a defensive-mixture episode with a missing weight or a weight above 1/ε; a subset chosen after outcomes; an episode whose address was admitted before, lies outside the committed range, or appears in the discovery record (N26a-23 rule 2); an episode run on a tier or numerical profile other than the committed ones (N26a-23 rule 4) | the first offending episode; its proposal law, or its address, or its tier and numerical profile; `nominal` or `defensive-mixture` with a recorded weight, a fresh address in the committed range, or the committed tier and profile | (1) draw fresh nominal episodes under a new commitment; (2) remove the offered set and let the verb sample; (3) use the offered episodes for exploration only | after-change | 3 |
| FROB-REF-563 | Importance sampling outside its admitted law, or with a proposal fitted on the run it is applied to | A randomized quantity with no declared monotone transform to normal germs; a law that is neither independent nor jointly Gaussian in germs; a proposal whose fitting campaign overlaps the committed range | the quantity or the proposal; its law or its `fitted_on`; an admitted law, or a separate completed pilot | (1) declare the transform to normal germs; (2) fit the proposal on a separate completed pilot and sample with a fresh key; (3) fall back to crude nominal sampling | after-change | 3 |
| FROB-REF-564 | Shrinking requested for a failure that does not replay, or that the guard labelled an artifact candidate | `nemesis.shrink` (and the shrink test relation): seed 0 of the full edit set does not reproduce its recorded state digests on the originating backend; or the artifact guard's label is `artifact-candidate` | the run; the first differing step, or the variant that removed the failure; identical digests and a passed guard | (1) rerun the guard after the simulator defect is fixed; (2) replay on the originating backend and numerical profile; (3) shrink a different failure | after-change | 3 |
| FROB-REF-565 | Coverage selection on a stale incidence | `nemesis.cover`: an identity the incidence was built under (model class, tier, kernel versions, failure taxonomy; also severity table and task version) differs from the identity in force | the incidence; the built identity; the identity in force | (1) rebuild the incidence under the identities in force; (2) use the standard battery, the fallback of the covering selection; (3) pin the request to the built identities to reproduce an earlier selection as a historical record only | after-change | 3 |
| FROB-REF-566 | A planted winner's rank is not on record from a full evaluation | `nemesis.audit prepare`: the proposed plant has no ledger record of a final-stage (full) evaluation, or its rank is outside the top k, or the record's task family, robot class or tier is not the campaign's | the design; the best record found (none, an earlier stage, a rank, a mismatched identity); a final-stage rank within the top k | (1) run the full evaluation of the design first, then plant it; (2) choose a planted winner of the design corpus that has a record; (3) run the audit without a plant: the control is reported `not-run` and finalists are described as "k survivors" | after-change | 3 |
| FROB-REF-567 | A hard-world or search-selected world offered as a confirmation sample | The qualification lane, at commit and at citation (N24b-16), and the hard-world set's own guard (BD-275): a request, a confirmation family or a cited result contains a world found in the discovery record (hard-set members including retired ones, counterexample files, search-selected addresses, items of the `hard` and `covering` batteries); at the qualification commit, any world set the caller names as the confirmation sample (the lane draws its own worlds, so a named hard or search-selected world, a `hard` or `covering` battery, or development addresses are each this code; `observed` names which, as in N24-REF); a hard-set run cited as confirmation | the world; `discovery (origin nemesis)`; a world of a sealed family | (1) draw confirmation worlds from a sealed world family through the qualification verb; (2) cite the result as a regression or discovery result; (3) remove the world from the request | never (for that world) | 3 |
| FROB-REF-568 | The failure predicate changed after sampling began | The bound's admission rule: an episode's predicate identity (the hash of task identity, requirement name and failure predicate) differs from the predicate committed when the bound was opened | the episode or the request; its predicate identity; the committed one | (1) start a new bound under the new predicate with fresh episodes; (2) restore the committed task identity | after-change | 3 |
| FROB-REF-569 | Splitting requested where no randomness enters after the fork | The splitting explorer's admission: the task has no registered per-step random stream that is active after the earliest possible fork tick and not held fixed by the request (a fork would replay its parent identically) | the task's stream registry; the streams active after the fork; at least one | (1) use tilting or the defensive mixture, which act on static parameters; (2) enable a per-step disturbance the task declares; (3) use crude sampling | after-change | 3 |
| FROB-REF-760 | The episodes a claim needs at its level exceed the budget; the bound reachable within budget is attached | `nemesis.bound --mode certify` at admission: the required n = ⌈ln δ / ln(1 − p⋆)⌉ at the request's level exceeds the n_b the budget affords | the budget; n_b; n; attachment `reachable_bound = 1 − δ^(1/n_b)` | (1) raise the budget to the stated amount; (2) ask for the reachable bound instead; (3) certify a decade higher and report the target as `Estimated` | after-change | 4 |

Worked attachment for FROB-REF-760: a target of 10⁻⁵ at δ = 0.05 needs 299,572 failure-free nominal episodes; a
budget that affords 100,000 yields `reachable_bound` = 1 − 0.05^(1/100000) = 2.996 × 10⁻⁵.

**Exit code of FROB-REF-760 (settled by the registry's exit rule).** The plan's exit table names `70x` under
exit 4 (budget) and `72x`, `73x` under exit 5 (capability); `76x` is in neither. The code is a budget refusal at
admission with the affordable alternative attached, so it is registered with exit 4, beside its neighbour
FROB-REF-761 (a job exceeds the compute envelope in force). The one refusal registry (N35-33) closes the gap with
a row of its `[exit_rule]` table, `76x` → 4 (a budget or a capacity refused at admission), and fails the load
of a `76x` row with any other exit; this row conforms to it, so the exit is fixed by the loader, not by a flag.

**Codes of other chapters at Nemesis boundaries (not owned here; named so a reader is not surprised).**
FROB-REF-840 (a session without the qualification administration scope asks for sealed confirmation data:
raised by every Nemesis verb for a world address in the confirmation namespace; exit 5, a missing scope, as the
qualification chapter's registry states). FROB-REF-549 (a rare-failure bound over battery-selected or
search-selected episodes: the cascade's code for the rule that FROB-REF-560 enforces at the bound). FROB-REF-546
(calibration from survivors, or an audit episode cancelled or re-weighted: the cascade's code beside the
screening audit; `nemesis.audit` also raises it, exit 3, for a stage record that offers a design outside the
committed lane draw as audited). FROB-REF-312 (a realism claim cites a scene with `assumed`
dimensions: the scene chapter's sibling of FROB-REF-361). FROB-REF-724 (the verb is not implemented in this
build: `nemesis.audit` before M4).

**Situations the plan gives no code (each a completed result or a refusal known by its slug; flagged).**

| Situation | Slug | Present behaviour | Where |
|---|---|---|---|
| A hard-world set `add` whose admission check fails; a battery whose only content is hard worlds | `hard-world-set-inadmissible` (`3xx`) | Refusal with exit 3, asserted by pointer and failed check | the hard-world-set bead (BD-275) |
| A rare-failure bound requested for a design that is not a finalist | `rare-bound-not-finalist` (`5xx`) | Refusal with exit 3 | the decision-policy bead for rare failures (N26a-26) |
| `nemesis.bound --mode certify` with `--proposal`, `--shift` or `--epsilon` (importance sampling never certifies absence) | `certify-by-importance-sampling` | Refusal at admission with exit 3, zero episodes drawn; repairs "use `--mode refute` or `--mode estimate`; drop the proposal and certify by crude nominal sampling" | BD-276 |
| A bounded-weight (importance) run planned above the 10⁶ evaluations of the pinned sibling's importance execution (DESIGN: exit 4, like FROB-REF-760 a count refused at admission with the reachable alternative attached; nothing runs and nothing is truncated) | `importance-over-sibling-cap` | Refusal at admission with exit 4; `observed` the planned evaluations, `required` at most the cap in force, the bound reachable at the cap attached; repairs "split the estimate into independent stages under separate commitments; use crude nominal sampling" | BD-276 (`cap_respected`), pinned by N26b-22; the cap becomes the configured one when FD-FSIM-18 lands (N26b-24) |
| A screening audit for a stage whose plan was not committed before its scores | none (completed result) | Completed result `not-run` with the reason, exit 2 | N26b-05 |
| Shrinking a failure whose world has no solvability witness | none (completed result) | Completed result `not-shrunk`, exit 2 | BD-274 |
| A coverage selection whose certificate fails its own check | none (completed result) | The fallback battery with a flag, exit 2, and an integrity finding | BD-278 |

All four slugs are filed in the second allocation round of the refusal registry (N35x-02), whose ledger assigns
their numbers: `hard-world-set-inadmissible` in the `3xx` family (363 to 369 were free when filed);
`rare-bound-not-finalist` and `certify-by-importance-sampling` in `5xx` (570 to 579 were free when filed, and the
first round's proposals for that decade come first by the ledger's rule, so no number is assumed here);
`importance-over-sibling-cap` in the sub-family `76x`, beside 760 and 761, with exit 4 (the lowest free number
above 761: a family-wide search would hand it a `72x` number, whose exit is 5). The three completed results are
recorded there as not requests. This bead enters a number only from the ledger; until a slug has an assigned
number, tests assert slug, exit code, pointer and failed check, and once it has one they read the number from
the ledger, never from a literal.

**FROB-REF-569 on `nemesis.bound`.** The bound verb lists 569 among its refusals because its decision policy may
choose splitting for exploration. There 569 is not the verb's exit: a splitting step that would be refused for want
of randomness after the fork is the policy's fallback trigger (b) (forks carry no fresh randomness; N26a-26),
recorded in the result's `fallback` block with the code, and the policy skips exploration and runs crude nominal
sampling with the full budget. Only `nemesis.search --method splitting` returns 569 as its own refusal (exit 3).

**Registry file.** Twelve `[[refusal]]` rows in `registries/refusals.toml`: `code`, `family`, `message`,
`defined_in = "§26.12"`, `exit`, `retry`, `repairs = [...]` in rank order, `raised_by` (the keys of the beads
named in the trigger column, the field N35-33's registry-wide check reads), `raised_at` (crate and function),
`tested_by` (unit test names and the end-to-end step). The loader rejects a row without repairs.

## Acceptance criteria

- [ ] `registries/refusals.toml` contains the twelve rows with the meanings above verbatim, at least two ranked
      repairs each, a retry class and an exit code; the loader accepts the file.
- [ ] Every row names `raised_by` bead keys that are live in the tracker index, a `raised_at` function and a `tested_by` unit test (checked by the
      registry test). This registry depends on none of its raisers, and several depend on it (BD-275, N26b-05,
      N26b-22), so the check reports a row whose function or test is not yet in the crate as
      `pending: <raising bead>`, and a pending row fails it from the M3 milestone exit on (M4 for FROB-REF-566,
      raised through `nemesis.audit`, N26b-03 and N26b-05). Every row is exercised from the user's side by the
      refusal script of N26b-14 or, for FROB-REF-566, by the audit script of N26b-07; both depend on this bead
      and carry that check.
- [ ] Each code maps to exactly one exit code: 3 for eleven of them, 4 for FROB-REF-760; every row's exit equals
      the exit its sub-family has in the registry's `[exit_rule]` table (N35-33), checked by the loader.
- [ ] No Nemesis code path emits a refusal code outside this table and the cross-referenced list; a refusal of
      the uncoded table is emitted with its slug and, once the allocation ledger assigns one, that number.
- [ ] The seven uncoded situations are listed in the crate's `CONTRACT.md` under known gaps, with slug, exit code
      and owning bead.
- [ ] The crate's `CONTRACT.md` states beside `nemesis.bound` that FROB-REF-569 is recorded there as fallback
      trigger (b) and is not the verb's exit; the user-side check is a step of N26b-14.

## Test plan

### Unit tests
`crates/frob-nemesis/tests/refusal_registry.rs`:
- `twelve_rows_present_and_verbatim` (the meaning strings compared byte for byte).
- `repairs_ranked_and_nonempty`; `exit_codes` (eleven 3s, one 4); `retry_classes`.
- `raised_by_and_tested_by_resolve` (`raised_by` keys found in the tracker index; `raised_at` function paths and test names found in the crate, a row whose raiser has not landed reported `pending: <raising bead>`; a scratch copy with the milestone-exit flag set and one pending row fails).
- `ref_560_payload_by_rule`: the constructed 560 refusal for a tilted episode carries the proposal law; for a reused address, an address outside the committed range and an address in the discovery record it carries the address (N26a-23 rule 2); for an episode on another tier it carries the episode's tier and numerical profile against the committed ones (rule 4).
- `no_unregistered_code_emitted`: every `Refusal::new` call site in the crate uses a code from the allowed set.
- `envelope_shape_per_code`: for each code a constructed refusal has `pointer`, `observed`, `required` and
  `repairs` filled as in the table (twelve cases).
- `ref_760_attachment`: budget 100,000 episodes, p⋆ = 10⁻⁵, δ = 0.05 → `required` 299,572, `observed` 100,000,
  `reachable_bound` 2.996 × 10⁻⁵ (to four significant digits, rounded upward).
- `exit_rule_conformance`: each of the twelve rows has the exit of its sub-family in `[exit_rule]`; a copy of the
  file with 760 at exit 3 fails the load.
- `uncoded_slugs_listed`: the seven situations are in `CONTRACT.md` with slug and exit; for each slug that the
  allocation ledger has assigned, the registry carries a row with the ledger's number and the stated exit, and
  for each slug it has not, no Nemesis row claims a number in 363 to 369, 570 to 579 or 762 to 769.
The user-side check that 569 on `nemesis.bound` is a fallback and not an exit is a step of the refusal script
(N26b-14), since it needs the mechanisms; this registry depends on none of them.

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
