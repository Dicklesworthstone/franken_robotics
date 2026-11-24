---
key: "N28-REF"
title: "Atelier refusal-code registry: FROB-REF-542, -543, -545, -546, -548 to -551, -553, -555 to -557, -870 to -872"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "atelier", "frob-core", "frob-opt", "solid", "registry"]
parent: "WP-41"
deps: ["BD-005", "BD-022", "N35x-02"]
needs: []
related: ["N28a-02", "N28b-12", "N28b-13", "N28b-15", "N28b-22", "N28b-23", "N28b-24", "N28b-27", "N28b-30", "N28b-38", "N28c-01", "N28c-12", "N28c-25", "N28c-29", "N28c-44", "N28cx-01", "N35x-01", "N41-REF", "N16b-04", "N23-REF"]
gates: []
plan_refs: ["§28.20", "Appendix O.5", "Appendix O.8", "§32.4", "§28.6", "§28.7", "§28.9", "§28.10", "§28.13", "§28.16", "§28.19"]
---
## Background

Every refusal in FrankenRobotics has a stable code, a message and ranked repairs. A refusal is not an error: it is
the system declining to produce a result that would be misleading, with the smallest changes that would make the
question answerable. The machine-readable registry is `registries/refusals.toml`; an unknown code is a load-time
error and codes are never reused. With `--json`, a refused verb returns the standard envelope with `ok: false` and
a `refusal` object: `code`, `message`, `pointer` (the field or part at fault), `observed`, `required`, `unit`,
`affected_claims`, `retry` (`never`, `after-change` or `backoff`) and ranked `repairs`. Exit codes: 3 for a refused
input or question (codes `1xx` to `5xx`, and `8xx` other than conflicts); exit code 1 is never emitted on purpose.

The design studio (Atelier) defines fifteen codes. Twelve are in the `5xx` block (learning and evaluation
protocol) and guard the honesty of comparisons, the ladder and the analyses; three are in the `8xx` block
(evidence and protocol) and guard reuse, decision tables and the wall between advisory numbers and verdicts. This
bead is the registry for the chapter: it enters the fifteen rows, with trigger and repairs complete enough that
each mechanism bead can raise its code without consulting anything else, and it tests the registry rows
themselves. The mechanisms raise the codes; the end-to-end refusal battery fires each on a planted violation.

Design reason for refusals instead of warnings here: each of these conditions makes a statistic or a verdict mean
something other than what its reader will assume (a paired difference that pairs nothing; a "selected" design from
a rung that cannot select; an optimum that survived only because nobody rechecked it). A warning would travel
with the number and be dropped at the first copy.

## Scope

In: the fifteen registry rows; message templates; pointer, observed and required conventions; retry class; ranked
repairs; exit code; the verbs each code is listed under; registry tests.
Out: raising each code (the mechanism beads named in the table); the planted-violation battery (N28c-21); codes
this chapter only consumes (for example `FROB-REF-537`, a rung used to select or discard without a rank-agreement
record that meets its threshold, defined by the learning subsystem; `FROB-REF-820`, a non-established label cited
as established; `FROB-REF-869`, a claim whose evidence closure is unavailable).

## Technical approach

Rows to enter in `registries/refusals.toml` (all exit 3; "First due" is the milestone at which the raising
mechanism first exists).

| Code | Refused | Trigger (the condition the check tests) | Ranked repairs | Retry | Raised by; verbs | First due |
|---|---|---|---|---|---|---|
| FROB-REF-542 | A paired study under an uncontrolled determinism class, or with arms in different classes | Either arm's determinism class is D4; or the arms' classes differ; or the class is D3 without decision-level equivalence recorded for the task. (Both arms must run at D2 or better on one backend, or at D3 with that record.) | 1. Rerun both arms on a D2 profile. 2. Record decision-level equivalence for the task, then compare at D3 | after-change | The comparison path (`pairing::admit`, N28a-18); `eval.compare`, `design.bisect`, `opt.start` (not the Arena study verbs, which answer 533: layer rule below) | M1 |
| FROB-REF-543 | An `anytime` study on a metric with no declared span | Study type `anytime` and the metric has no bound on its paired difference fixed before the first observation (the e-process needs an explicit loss span that must not be estimated from the race data) | 1. Declare the span in the task definition. 2. Use the fixed-n study type | after-change | The two study types; `eval.compare`, `study.plan`, `opt.start` (BD-341: an e-process race on a metric with no declared span), `design.bisect` (BD-343: a probe predicate `metric M worse-by m` on a metric with no declared span, refused before any probe) | M1 (`opt.start`, `design.bisect` M4) |
| FROB-REF-545 | A rung asked to select in a stratum whose mode is not `selective` | A request for finalists ("the top k") from a rung pair and stratum whose rank-agreement record exists and whose mode by that record is `discard-only` or `uninformative` (a stratum with no record at all also fails closed, under the learning chapter's FROB-REF-537 and not under this code: `related` below); `opt.start --method mf-bo` whose adapter's fidelity set holds a rung not `selective` in the campaign's stratum (N28b-07, refused at admission before any world is created); a tier-pair record's consumer asking the fast tier to select in such a stratum (BD-265); a family-policy rung score requested to select (`policy.apply --score-for-ladder select`) through a record whose mode is not `selective` (N23a-24, N23a-25) | 1. Run the audit lane (and the rank-agreement measurement) for the stratum. 2. Accept "k survivors" (`opt.cascade`, the ladder); for `opt.start --method mf-bo`, use a single-fidelity method (`tr-bo`) | after-change | The ladder (N28b-23), the multi-fidelity rule (N28b-07) and the rung-scoring entry points of `frob-learn` (N23a-24, N23a-25); `opt.cascade` (also `--finalists-from <rung>`), `opt.start`, `eval.rank-agreement`, `policy.apply`. `related`: FROB-REF-537 (the learning chapter's code, N23-REF: a rung asked to select or discard with no record, or to discard where the record makes the stratum `uninformative`): 537 when the record is absent or forbids the discard, 545 when a record exists and only selection is forbidden; the division is BD-328's, which owns the record, and N23-REF carries the same sentence on its 537 row. One site lies outside the division (DESIGN, N28b-07's present rule): `opt.start --method mf-bo` reads only the mode, in which an unmeasured stratum counts as `uninformative`, and answers 545 with or without a record, because the repair that helps there is this row's single-fidelity method and 537's "use the rung as advisory" does not apply to a fidelity set | M3 (tier pairs, BD-265), M4 (the ladder, `opt.start`, `policy.apply`) |
| FROB-REF-546 | Calibration from survivors, or an audit episode cancelled or re-weighted | A calibration set containing a design not drawn by the keyed audit stream; or a request that cancels an audit-lane episode short of cancelling the whole campaign (a job cancellation aimed at one audit episode); or an audit design selected or re-weighted after a score for it existed; or a design outside the committed draw offered as audited | 1. Redraw the audit lane by key (a new generation's draw, never a re-draw of the same one) | after-change | The ladder's audit lane (N28b-13), the screening audit (N26b-05, BD-345) and the rank-agreement record's calibration (BD-328: a design set holding a design the keyed stream did not draw); `opt.cascade`, `job.cancel` (an audit episode's job), `nemesis.audit`, `eval.rank-agreement` | M3 (the audit lane, through `job.cancel`), M4 (the cascade, `nemesis.audit`, `eval.rank-agreement`) |
| FROB-REF-548 | Pooling evidence or calibration across two model identities | Rows offered to one ledger stratum or calibration set whose model identity, kernel versions, effort profile or battery version differ | 1. Restart the ledger rows in the new identity | after-change | Promotion, the rank-agreement ledger, conformal calibration; `eval.promote`, `eval.rank-agreement`, `opt.cascade` | M3 |
| FROB-REF-549 | A rare-failure bound over battery-selected or search-selected episodes | The episode set that the studio (the co-design cascade, or any `frob-opt` caller) offers to a certification bound contains an address selected by a battery or by an adversarial search. The same rule is enforced one layer down by the bound's own admission rule, whose code is the adversarial chapter's `FROB-REF-560`: a direct call of `nemesis.bound` raises 560, a call made through the studio surfaces 549 | 1. Draw nominal episodes | after-change | The studio's bound request; `opt.cascade`, and `nemesis.bound` when called by the cascade | M3 |
| FROB-REF-550 | `first-change` asked of bisect without a prefix scan or a declared monotone predicate | `design.bisect` asked to label its result `first-change` when not every earlier version on the path has a decided outcome and the caller has not declared the predicate monotone | 1. Scan the prefix. 2. Declare the predicate monotone, which is recorded as an assumption on the result | after-change | Lineage bisect; `design.bisect` | M4 |
| FROB-REF-551 | An exclusion from an analysis whose model does not enclose a declared state or energy source | An analysis is about to return an `outer` (excluding) result while a state or energy source declared in the design, scene or mission is not bound in the analysis model (for example approach kinetic energy in the energy-budget analysis) | 1. Bind the missing source. 2. Accept `unresolved` | after-change | The bound analyses; `design.check`, `analyze.<name>`, `eval.compare` (N25e-15: with `--bracket`, a stop requested on a relaxation that does not enclose a declared state or energy source) | M1 (`eval.compare --bracket` M4) |
| FROB-REF-553 | A comparison that mixes study types, or re-reads fixed-n data as a sequential study | A study that declares both types; an e-process asked to consume outcomes collected under a frozen fixed-n rule; a fixed-n rule applied to a stopped anytime study. Autopsy's two forms, both before any fork is bought: an `autopsy.intervene` request any of whose interventions, by canonical string, was already run on the fork under the other test type, in both directions (N27bp2-01; `pointer` `test`, `observed` the stored type and its ledger); a pooled `autopsy.attribute --forks` read over a `fixed` set and a `sequential` set (BD-280; `pointer` the first fork of the other type) | 1. Declare one type. 2. Open a new study. For `autopsy.intervene`: 1. read the stored set (`frob autopsy attribute <fork> --ledger <id>`); 2. repeat the request with the stored type; 3. run the other type from a fork point opened at another lead (`frob run fork <run> --world <w> --before failure --dt 0.5s`). For `autopsy.attribute`: read the forks of each type in separate calls | after-change | The comparison path (`pairing::admit`, N28a-18) and the study comparison; `eval.compare`, `study.compare` (not `study.plan`, which answers 552); Autopsy's test-type check (N27bp2-01, BD-280); `autopsy.intervene`, `autopsy.attribute` | M1 (Autopsy's forms M3) |
| FROB-REF-555 | A fixed-n study whose declared verdict set contains an unreachable verdict | The reachability pre-analysis at the declared n and margin shows a declared verdict cannot be returned, and the request does not acknowledge it | 1. Raise n to the first value at which the verdict is reachable (the refusal reports it; for a practical tie at a five-point margin, n = 86). 2. Acknowledge the unreachable verdict in the request | after-change | The comparison path (`pairing::admit`, N28a-18); `eval.compare` (not `study.plan` or `eval.preanalyze`, which answer 532) | M1 |
| FROB-REF-556 | A pairing keyed by event order or by a training counter | A pairing key that contains an event index ("the k-th friction draw") or a counter that depends on training progress (an epoch), instead of (world, replicate) addresses and space-indexed exogenous fields | 1. Index exogenous fields by physical coordinate. 2. Use (world, replicate) addresses | after-change | The pairing rules; `eval.compare`, `study.plan`, `opt.start`, `task.check` (under `--coupling`, N24c-37: the coupling checks of a task's exposure table) | M1 (`task.check --coupling` M2) |
| FROB-REF-557 | A `model-conditional` result offered as a design comparison without its condition (the build-packet half of the plan's row is raised as `FROB-REF-830`, see the precedence rule below) | A report, export or claim that presents a `model-conditional` ranking as a design comparison without the variant that flips it (for example a compact report form that drops the condition columns) | 1. Run the rank-robustness analysis (A-19). 2. Report the condition | after-change | Reports and claims; `report.design`, `report.campaign` (the verbs that present the ranking). Not `analyze.rank-robustness`, `fidelity.modelform` or `opt.cascade`, which set the `model-conditional` condition and raise neither 557 nor 830 (BD-346): a row lists what its handler can return | M3 |
| FROB-REF-870 | Reuse of a verdict with no licence, or under a rule the registry does not know | A reused verdict without a grade and licence; a grade-3 licence naming a rule absent from the registry; a changed quantifier prefix | 1. Recheck (`frob result revalidate <id>`) | after-change | Revalidation (N28b-32, BD-292; the one engine of both verbs, N28c-29); `result.revalidate`, `mission.revalidate`. From M4 the consumer check of BD-347 (`revalidate::require_licence`) also raises it wherever a consumer presents a verdict carried across an edit without a grade and a licence: `report.design`, `report.mission` and `study.compare` (wired by BD-347), and `opt.cascade` (its reuse stage, stage 2) and `report.campaign` (wired by BD-352) | M1 (`result.revalidate`), M3 (`mission.revalidate`), M4 (consumers) |
| FROB-REF-871 | A decision-core table that is missing, stale against its contract, or not the registry version | Table file absent; content hash or version differs from the core's record; table bound to another identity. On a normal call of a governed verb this is not a refusal: the verb completes on the core's fallback with the code in `warnings` and in its decision record; it is a refusal (exit 3) when the request pins the rule (`--require-rule table`) | 1. Regenerate the table (`cargo xtask check-generated` regenerates and compares). 2. Continue: the core's fallback is in force meanwhile | after-change | The core-table loader (N28c-02); the verbs named in the `governed` list of a core record (N28c-01): `eval.promote` (DC-01, BD-263), `nemesis.cover` (DC-10, BD-309), `opt.cascade` (DC-02, DC-03) and the others their records (BD-349, N28a-27, N28a-28) name; this column is derived from those lists by `xtask check-generated`, not typed; shown as a finding (exit 2, never a refusal) by `cores.show` | M3 |
| FROB-REF-872 | A value-of-information output cited as a verdict | An identifier of result kind `advisory` passed where a verdict or an establishable result is required | 1. Run the comparison (`frob eval compare A B --task T`, pre-filled from the advisory record) | after-change | The advisory type wall (N28c-12, `frob_verbs::require_establishable`): every parameter the registry marks `accepts = "verdict"`, which are, as the rows stand: the `result` of `eval.promote`; `--cite` of `report.design`, `report.run`, `report.campaign` and `report.mission` (and their statements, built from cited identifiers); the target of `result.revalidate`. Not `eval.qualify`, `evidence.trace` or `foundry.export`: no parameter of theirs takes a verdict or result identifier, so an advisory identifier given to them fails the schema with `FROB-REF-802`. Not `experiment.suggest`, which produces advisory outputs and accepts none as a verdict | M3 |

**Message templates and fields (DESIGN).** One line each, stating what was refused and the observed and required
values, for example for `FROB-REF-555`: message "A declared verdict cannot be returned at this n"; `pointer`
"study.verdicts[practical-tie]"; `observed` "n = 64, margin = 0.05"; `required` "n ≥ 86"; `unit` "pairs". For
`FROB-REF-542`: `observed` "arm A: D2 (cuda), arm B: D4"; `required` "both D2 on one backend, or D3 with
decision-level equivalence". For `FROB-REF-548`: `observed` the two identities that differ, field by field.
`affected_claims` lists the claim identifiers that would have rested on the refused result.

**Precedence among the codes for `model-conditional` results (DESIGN).** The plan's row for 557 names two cases
(a comparison without its condition, and input to a build packet), and the learning chapter's `FROB-REF-830`
names "a `model-conditional` result offered for promotion, build or prototype budget"; the build packet is in
both. One request raises exactly one code, by the rule the fidelity bead N16b-04 states for the whole family: a
`model-conditional` result offered for promotion, a build packet (`foundry.export`) or prototype budget raises
`FROB-REF-830` (the more specific spend code, which GATE-CORTEX-15 requires to fire); a `model-conditional`
ranking presented as a design comparison without the variant that flips it raises `FROB-REF-557`; a
`capacity-biased` comparison cited as a verdict raises `FROB-REF-822`; every other citation of a non-established
result raises `FROB-REF-820`. Reason: a build-packet request is a spend of budget, whose code and gate belong to
the spend layer; the studio's code guards presentation. The protection the plan requires is kept whole: a
`model-conditional` result still cannot enter a build packet; only the code that says so is fixed.

**Registry mechanics.** Each row carries: `code`, `block` (`5xx` or `8xx`), `message`, `retry`, `repairs`
(ordered), `exit = 3`, `defined_in` (the studio), `raised_by` (crate and module), and `verbs` (must agree with the
`refusals` lists in `registries/verbs.toml`; the generated-artifact check compares the two).

## Acceptance criteria

- [ ] The fifteen rows are present in `registries/refusals.toml`, load without error, and each has a message, a
      retry class, at least one ranked repair and exit 3.
- [ ] For each code, every verb in its `verbs` column lists the code in `registries/verbs.toml`, and no verb lists
      one of these codes without the row naming it (checked by `xtask check-generated`). Two columns are derived,
      not typed: 871's from the `governed` lists of the core records, 872's from the parameters marked
      `accepts = "verdict"`; a governed verb or a marked parameter whose verb lacks the code fails the check.
- [ ] No two rows share a code; none of the fifteen collides with a code defined elsewhere; a deleted row fails
      the load of any verb that lists it.
- [ ] Each code maps to exactly one exit code (3).
- [ ] The refusal envelope for each code validates against the envelope schema with `pointer`, `observed`,
      `required` and `repairs` populated (checked with a fixture refusal per code).
- [ ] The documentation table generated into the agent-facing reference lists all fifteen with repairs.
- [ ] The precedence rule for `model-conditional` results is in the registry's documentation, `foundry.export`
      lists 830 and not 557, and `precedence_one_code_per_request` passes.
- [ ] The layer rule holds on every verb row: the Arena study verbs list 552, 532 and 533, `eval.compare` and the
      Atelier verbs list 553, 555 and 542, and no verb lists both codes of one pair (`layer_rule_one_code_per_pair`).
- [ ] `scripts/e2e/atelier_refusal_registry.sh` passes, and its four scratch-registry controls each fail as stated.

## Test plan

### Unit tests

`crates/frob-core/tests/refusals_atelier.rs`:
- `fifteen_codes_present` (the exact set: 542, 543, 545, 546, 548, 549, 550, 551, 553, 555, 556, 557, 870, 871,
  872);
- `all_exit_3`; `all_have_repairs`; `retry_class_after_change`;
- `verbs_lists_agree` (registry cross-check in both directions; for 871 against the union of the core records'
  `governed` lists, for 872 against the verbs of the `accepts = "verdict"` parameters; a scratch registry in which
  `job.cancel` drops 546, or `experiment.suggest` lists 872, fails naming the verb and the code);
- `fixture_envelopes_validate` (one synthetic refusal per code built with the constructor the mechanisms use);
- `repair_order_stable` (the first repair of 555 is "raise n", of 871 "regenerate", of 872 "run the comparison");
- `unknown_code_is_load_error` (a scratch copy of the registry with the `FROB-REF-550` row deleted, while
  `design.bisect` still lists `FROB-REF-550`, fails to load and names the code; the test uses one of this
  chapter's own codes on a scratch copy, so it never depends on which numbers other chapters have allocated);
- `precedence_one_code_per_request` (a `model-conditional` ranking printed without its condition maps to 557; the
  same result offered to a build-packet request maps to 830 and not to 557);
- `layer_rule_one_code_per_pair` (no verb row of `registries/verbs.toml` lists both codes of a pair: 552 and 553,
  532 and 555, 533 and 542; `study.plan` lists 532, 533, 552, 543, 556 and none of 553, 555, 542; `eval.compare`
  lists 553, 555, 542, 543, 556 and none of 552, 532, 533; a scratch registry in which `study.plan` lists 542
  fails, naming the verb and the pair).

### End-to-end script and logging

`scripts/e2e/atelier_refusal_registry.sh`: the registry seen from the user's side, needing no Atelier mechanism.
It runs `frob schema --refusals --json` and asserts that the fifteen rows are present with message template,
pointer kind, `retry: after-change`, ranked repairs (first repair equal to the table's) and exit 3; validates one
fixture refusal envelope per code against `frob schema frob.refusal/1 --json`; reads the refusal lists of
`study.plan` and `eval.compare` from `frob schema <verb> --json` and logs, per pair (552/553, 532/555, 533/542),
the code each verb lists, asserting one per verb (step `layer_rule`); checks that the generated
agent-facing documentation lists each code with its repairs; and asserts the negative controls below from the
command line on scratch copies of the registry under `target/e2e-scratch/atelier_refusal_registry/` (never the
committed file): the 871 row removed, a row with exit 4, a verb raising 553 without listing it, the 550 row
deleted while `design.bisect` lists it. Each must make `frob schema --refusals` or `xtask check-generated` fail
with the code named. It sources the STD-E2E helper and writes
`target/e2e-logs/atelier_refusal_registry/<utc-timestamp>.ndjson` with a transcript; every line has `ts`,
`script`, `bead` (`N28-REF`), `step`, `event` (`start`, `cmd`, `assert`, `refusal`, `end`), the exact command line,
exit code, wall time and the envelope's `evidence` block (claim Known answer, class Tested, determinism D0); per
code an `assert` line with code, exit code, retry class and first repair against the table; on failure the first
missing or differing field and the scratch registry path. A final `summary` line counts assertions passed and
failed and names GATE-ATEL-06 (the five M1 rows) and GATE-TETHER-04.

The battery that fires each of the fifteen on a planted violation through the real verbs is
`scripts/e2e/atelier_refusals.sh` (N28c-21); it logs, per code, a `refusal` line with code, exit code, pointer,
observed, required and the ranked repairs, compared with this registry. This bead does not write a second script
of that name.

### Known answers and negative controls

- Known answers that appear in refusal payloads: the first n at which a practical tie is reachable at a
  five-point margin is 86 (at n = 64 there are 2,145 count pairs, of which 521 promote, 521 are inferior, 1,103
  are inconclusive and none is a tie).
- Negative controls: a registry with `FROB-REF-871` removed (verbs governed by cores fail to load); a row with
  exit 4; a verb that raises `FROB-REF-553` without listing it (generated-artifact check fails); the 550 row
  deleted while `design.bisect` lists it (load fails naming the code); a build-packet request with a
  `model-conditional` input that raises 557 instead of 830 (the precedence test fails).
- Gate GATE-ATEL-06 requires `FROB-REF-553`, `-555`, `-542`, `-543` and `-556` each to fire on their planted
  violation, 100 of 100; those five rows are due at M1.

## Considerations

- Numbering has gaps (544, 547, 552, 554 belong to other subsystems); the registry loader does not assume
  contiguity.
- The plan states one refusal with no code: a reuse figure printed without its column name is refused in a
  report (raised by N28c-44). It is the ledger request `reuse-figure-without-column`, filed in the second
  allocation round (N35x-02) in family `8xx` and held with that family under the owner decision N35x-01. While
  it is held the report verb refuses with exit 3 as a named generation error with the refusal envelope's shape
  (message, `pointer` at the offending field, the repairs "print the three columns" and "name the column and
  regime"), as N28c-44 specifies; this registry enters a numbered row only from the ledger.
- Three more conditions of the chapter have no code, and the chapter's beads agree on what happens instead;
  this registry adds no row for any of them. (a) A loop stage asked to run on a design the previous stage
  refused (INV-ATEL-01): the loop-order rule re-issues the upstream stage's own code, with the override flag
  recorded on downstream results when the caller overrides; the invariant battery accepts that code. (b) An
  A-21 deadline that no admitted schedule serves, and (c) an A-22 loop that is unstable under its sampling:
  both are High findings of `design.check` with exit 2 and do not refuse.
- The same defect is refused under different codes at different layers, and the pairs are deliberate: a
  paired statistic under an uncontrolled determinism class is 542 here (raised by `eval.compare`), 533 in the
  task chapter and 561 in the numerics chapter; mixing study types is 553 here and 552 there; an unreachable
  verdict is 555 here and 532 there; a rare-failure bound over selected episodes is 549 here and 560 in the
  adversarial chapter, which raises it inside `nemesis.bound`. Each verb lists the code of its own layer.
  **Layer rule (DESIGN, the rule of the Arena registry N24-REF, kept here so that both registries state it):**
  both codes of each pair stay, because the plan registers both (§24.14 and §28.20) and merging them would renumber
  a published code; one situation raises one code on one verb. The Arena study verbs (`study.plan`,
  `study.submit`, `study.status`, `eval.preanalyze`) admit through `study::admit` (N24b-14), which runs the Arena
  checks first and so answers 552, 532 and 533; `eval.compare`, `study.compare` and the Atelier verbs admit through
  `pairing::admit` first and answer 553, 555 and 542. The codes without an Arena twin (543, 556) are listed by both
  paths. The `verbs` column above follows that rule, and the test `layer_rule_one_code_per_pair` holds every verb
  row to it: no verb lists both codes of one pair (552 and 553; 532 and 555; 533 and 542). The verb rows hold to
  it (VERBS-study, VERBS-eval): `study.plan` lists 532, 533 and 552, `eval.compare` lists 542, 553 and 555, and
  `study.submit` lists none of the six.
- OPEN-ATEL-4 (whether `result.revalidate` and `mission.revalidate` stay two verbs) changes only the `verbs`
  column of `FROB-REF-870`.
- Repairs are advice to an agent; each must be an action the agent can take with a verb, and the first repair
  should be the cheapest one that makes the original question answerable.

## References

§28.20; Appendix O (5xx and 8xx); §32.4 (envelope and exit codes); GATE-ATEL-06; GATE-TETHER-04; OPEN-ATEL-4.
