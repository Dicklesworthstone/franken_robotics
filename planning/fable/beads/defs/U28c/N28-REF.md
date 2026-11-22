---
key: "N28-REF"
title: "Atelier refusal-code registry: FROB-REF-542, -543, -545, -546, -548 to -551, -553, -555 to -557, -870 to -872"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "atelier", "frob-core", "frob-opt", "solid", "registry"]
parent: "WP-41"
deps: ["BD-005", "BD-022"]
needs: ["§32.18: the verb registry's refusal list per verb and the test that every refusal code maps to exactly one exit code (BD-137, BD-299)", "§28.26: a refusal code for a reuse figure printed in a report without its column name; the plan states the refusal and assigns no code (the owner of Appendix O)"]
related: ["N28-CX", "N28c-21", "N28c-25", "BD-124", "BD-262", "BD-309"]
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
| FROB-REF-542 | A paired study under an uncontrolled determinism class, or with arms in different classes | Either arm's determinism class is D4; or the arms' classes differ; or the class is D3 without decision-level equivalence recorded for the task. (Both arms must run at D2 or better on one backend, or at D3 with that record.) | 1. Rerun both arms on a D2 profile. 2. Record decision-level equivalence for the task, then compare at D3 | after-change | The two study types; `eval.compare`, `study.plan`, `study.submit`, `design.bisect`, `opt.start` | M1 |
| FROB-REF-543 | An `anytime` study on a metric with no declared span | Study type `anytime` and the metric has no bound on its paired difference fixed before the first observation (the e-process needs an explicit loss span that must not be estimated from the race data) | 1. Declare the span in the task definition. 2. Use the fixed-n study type | after-change | The two study types; `eval.compare`, `study.plan` | M1 |
| FROB-REF-545 | A rung asked to select in a stratum whose mode is not `selective` | A request for finalists ("the top k") from a rung pair and stratum whose mode in the rank-agreement ledger is `discard-only` or `uninformative` | 1. Run the audit lane (and the rank-agreement measurement) for the stratum. 2. Accept "k survivors" | after-change | The ladder; `opt.cascade`, `opt.start` | M4 |
| FROB-REF-546 | Calibration from survivors, or an audit episode cancelled or re-weighted | A calibration set containing a design not drawn by the keyed audit stream; or an audit-lane episode in state cancelled; or an audit design selected or re-weighted after a score for it existed | 1. Redraw the audit lane by key | after-change | The ladder's audit lane; `opt.cascade`, `eval.rank-agreement` | M4 (pilot M3) |
| FROB-REF-548 | Pooling evidence or calibration across two model identities | Rows offered to one ledger stratum or calibration set whose model identity, kernel versions, effort profile or battery version differ | 1. Restart the ledger rows in the new identity | after-change | Promotion, the rank-agreement ledger, conformal calibration; `eval.promote`, `eval.rank-agreement`, `opt.cascade` | M3 |
| FROB-REF-549 | A rare-failure bound over battery-selected or search-selected episodes | The episode set offered to a certification bound contains an address selected by a battery or by an adversarial search | 1. Draw nominal episodes | after-change | The rare-failure bound; `nemesis.bound` | M3 |
| FROB-REF-550 | `first-change` asked of bisect without a prefix scan or a declared monotone predicate | `design.bisect` asked to label its result `first-change` when not every earlier version on the path has a decided outcome and the caller has not declared the predicate monotone | 1. Scan the prefix. 2. Declare the predicate monotone, which is recorded as an assumption on the result | after-change | Lineage bisect; `design.bisect` | M4 |
| FROB-REF-551 | An exclusion from an analysis whose model does not enclose a declared state or energy source | An analysis is about to return an `outer` (excluding) result while a state or energy source declared in the design, scene or mission is not bound in the analysis model (for example approach kinetic energy in the energy-budget analysis) | 1. Bind the missing source. 2. Accept `unresolved` | after-change | The bound analyses; `design.check`, `analyze.<name>` | M1 |
| FROB-REF-553 | A comparison that mixes study types, or re-reads fixed-n data as a sequential study | A study that declares both types; an e-process asked to consume outcomes collected under a frozen fixed-n rule; a fixed-n rule applied to a stopped anytime study | 1. Declare one type. 2. Open a new study | after-change | The two study types; `eval.compare`, `study.plan`, `study.compare` | M1 |
| FROB-REF-555 | A fixed-n study whose declared verdict set contains an unreachable verdict | The reachability pre-analysis at the declared n and margin shows a declared verdict cannot be returned, and the request does not acknowledge it | 1. Raise n to the first value at which the verdict is reachable (the refusal reports it; for a practical tie at a five-point margin, n = 86). 2. Acknowledge the unreachable verdict in the request | after-change | The fixed-n planner; `study.plan`, `eval.preanalyze`, `eval.compare` | M1 |
| FROB-REF-556 | A pairing keyed by event order or by a training counter | A pairing key that contains an event index ("the k-th friction draw") or a counter that depends on training progress (an epoch), instead of (world, replicate) addresses and space-indexed exogenous fields | 1. Index exogenous fields by physical coordinate. 2. Use (world, replicate) addresses | after-change | The pairing rules; `eval.compare`, `study.plan`, `opt.start` | M1 |
| FROB-REF-557 | A `model-conditional` result offered as a design comparison without its condition, or as input to a build packet | A report, export or claim that presents a `model-conditional` ranking without the variant that flips it; or a build-packet request whose input is `model-conditional` | 1. Run the rank-robustness analysis (A-19). 2. Report the condition | after-change | Reports and the build packet; `report.design`, `report.campaign`, `foundry.export` | M3 |
| FROB-REF-870 | Reuse of a verdict with no licence, or under a rule the registry does not know | A reused verdict without a grade and licence; a grade-3 licence naming a rule absent from the registry; a changed quantifier prefix | 1. Recheck | after-change | Revalidation; `result.revalidate`, `mission.revalidate` | M1 (mission cells), M3 |
| FROB-REF-871 | A decision-core table that is missing, stale against its contract, or not the registry version | Table file absent; content hash or version differs from the core's record; table bound to another identity | 1. Regenerate the table. 2. Continue: the core's fallback is in force meanwhile | after-change | The core-table loader; every verb a core governs; shown as a finding by `cores.show` | M3 |
| FROB-REF-872 | A value-of-information output cited as a verdict | An identifier of result kind `advisory` passed where a verdict or an establishable result is required | 1. Run the comparison | after-change | The advisory type wall; `eval.promote`, `eval.qualify`, `report.*`, `result.revalidate`, `foundry.export` | M3 |

**Message templates and fields (DESIGN).** One line each, stating what was refused and the observed and required
values, for example for `FROB-REF-555`: message "A declared verdict cannot be returned at this n"; `pointer`
"study.verdicts[practical-tie]"; `observed` "n = 64, margin = 0.05"; `required` "n ≥ 86"; `unit` "pairs". For
`FROB-REF-542`: `observed` "arm A: D2 (cuda), arm B: D4"; `required` "both D2 on one backend, or D3 with
decision-level equivalence". For `FROB-REF-548`: `observed` the two identities that differ, field by field.
`affected_claims` lists the claim identifiers that would have rested on the refused result.

**Registry mechanics.** Each row carries: `code`, `block` (`5xx` or `8xx`), `message`, `retry`, `repairs`
(ordered), `exit = 3`, `defined_in` (the studio), `raised_by` (crate and module), and `verbs` (must agree with the
`refusals` lists in `registries/verbs.toml`; the generated-artifact check compares the two).

## Acceptance criteria

- [ ] The fifteen rows are present in `registries/refusals.toml`, load without error, and each has a message, a
      retry class, at least one ranked repair and exit 3.
- [ ] For each code, every verb in its `verbs` column lists the code in `registries/verbs.toml`, and no verb lists
      one of these codes without the row naming it (checked by `xtask check-generated`).
- [ ] No two rows share a code; none of the fifteen collides with a code defined elsewhere; a deleted row fails
      the load of any verb that lists it.
- [ ] Each code maps to exactly one exit code (3).
- [ ] The refusal envelope for each code validates against the envelope schema with `pointer`, `observed`,
      `required` and `repairs` populated (checked with a fixture refusal per code).
- [ ] The documentation table generated into the agent-facing reference lists all fifteen with repairs.

## Test plan

### Unit tests

`crates/frob-core/tests/refusals_atelier.rs`:
- `fifteen_codes_present` (the exact set: 542, 543, 545, 546, 548, 549, 550, 551, 553, 555, 556, 557, 870, 871,
  872);
- `all_exit_3`; `all_have_repairs`; `retry_class_after_change`;
- `verbs_lists_agree` (registry cross-check in both directions);
- `fixture_envelopes_validate` (one synthetic refusal per code built with the constructor the mechanisms use);
- `repair_order_stable` (the first repair of 555 is "raise n", of 871 "regenerate", of 872 "run the comparison");
- `unknown_code_is_load_error` (a verb listing `FROB-REF-558` fails to load).

### End-to-end script and logging

`scripts/e2e/atelier_refusals.sh` (N28c-21) fires each of the fifteen on a planted violation through the real
verbs and logs, per code, a `refusal` line with code, exit code, pointer, observed, required and the ranked
repairs, compared with this registry.

### Known answers and negative controls

- Known answers that appear in refusal payloads: the first n at which a practical tie is reachable at a
  five-point margin is 86 (at n = 64 there are 2,145 count pairs, of which 521 promote, 521 are inferior, 1,103
  are inconclusive and none is a tie).
- Negative controls: a registry with `FROB-REF-871` removed (verbs governed by cores fail to load); a row with
  exit 4; a verb that raises `FROB-REF-553` without listing it (generated-artifact check fails).
- Gate GATE-ATEL-06 requires `FROB-REF-553`, `-555`, `-542`, `-543` and `-556` each to fire on their planted
  violation, 100 of 100; those five rows are due at M1.

## Considerations

- Numbering has gaps (544, 547, 552, 554 belong to other subsystems); the registry loader does not assume
  contiguity.
- The plan states one refusal with no code: a reuse figure printed without its column name is refused in a
  report. Until the registry owner assigns a code, the report lint raises it as a generation failure of the
  report verb with a clear message; this is recorded as a need and in the coverage notes.
- OPEN-ATEL-4 (whether `result.revalidate` and `mission.revalidate` stay two verbs) changes only the `verbs`
  column of `FROB-REF-870`.
- Repairs are advice to an agent; each must be an action the agent can take with a verb, and the first repair
  should be the cheapest one that makes the original question answerable.

## References

§28.20; Appendix O (5xx and 8xx); §32.4 (envelope and exit codes); GATE-ATEL-06; GATE-TETHER-04; OPEN-ATEL-4.
