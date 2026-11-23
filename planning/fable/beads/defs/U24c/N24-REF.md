---
key: "N24-REF"
title: "registry: the 21 Arena refusal codes (502, 520 to 539, 552, 840 to 845) with triggers, repairs, exit codes"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "arena", "frob-arena", "frob-spec", "registry", "test", "solid"]
parent: "WP-41"
deps: ["BD-005"]
needs: []
related: ["BD-122", "BD-124", "BD-259", "BD-261", "N24-CX", "BD-137", "BD-312", "N24a-12", "N24a-16", "N24a-31"]
gates: ["GATE-ARENA-03"]
plan_refs: ["§24.14", "§24.5", "§24.7", "§24.10", "§24.12", "§24.13", "§32.4", "§35.11", "Appendix O"]
---
## Background

Every refusal in this system has a stable code, a message and ranked repairs; codes are never reused; the
machine-readable registry is `registries/refusals.toml`, and an unknown code is a load-time error. Three standing
rules apply: refuse, never truncate; a budget stop publishes nothing partial as a result; a misspelt parameter is
a refusal, not a default. Code families: `1xx` design and model; `2xx` numerics; `3xx` scene; `4xx` control and
estimation; `5xx` learning and evaluation protocol; `6xx` backend and device; `7xx` budget and capability; `8xx`
evidence and protocol; `9xx` internal.

The Arena chapter defines 21 codes: one carried from the scene chapter's split rule (502), fourteen in the `5xx`
family for the requirement language, the reset cache, fixtures and studies, and six in the `8xx` family for the
blinded qualification lane. The chapter's own table gives each code's meaning in one line. This bead is the
registry for those 21: it carries, for each, the meaning, the trigger as the mechanism states it, ranked repairs
and the exit code, so that the entries of `registries/refusals.toml` and the refusal lists of the verbs can be
written from one place. The repairs below are derived from the mechanism text of the chapter (the chapter's
table lists meanings only); they are DESIGN and are ranked cheapest first.

On the wire a refusal is the envelope's `refusal` object: `code`, `message`, `pointer` (the field or part at
fault), `observed`, `required`, `unit`, `affected_claims`, `retry` (`never`, `after-change`, `backoff`), and
ranked `repairs`; the first repair also appears as `next_actions[0]`. Exit code 3 is "refused input or question"
and covers `FROB-REF-1xx` to `5xx` and `8xx` other than conflicts; exit code 7 is a conflict (idempotency key
reuse with different content, stale parent).

## Scope

In: the 21 registry entries; a test that each is registered, raised by its planted trigger, and carries at least
one repair; the list of refusals that the late Arena subsections define without code numbers; the list of codes
owned elsewhere that Arena mechanisms raise.
Out: implementing the mechanisms that raise them (the beads named per row).

## Technical approach

**The 21 codes.**

| Code | Meaning | Trigger | Ranked repairs | Exit | Raised by |
|---|---|---|---|---|---|
| FROB-REF-502 | An evaluation reuses a scene from the policy's training manifest (carried from the scene-split rule) | At scoring time a battery scene's split group intersects the training manifest of the candidate's lineage, or the lineage has a node with no manifest | Evaluate on the held-out side of the split; publish a battery version without the scene (a recorded event that changes the battery identity); report the score as in-distribution, not held-out | 3 | N24a-31 |
| FROB-REF-520 | An atom has no scale and is not declared exact | `(atom …)` without `:scale <quantity>` or `:exact`; `:exact` on a non-integer signal | Add a scale of the expression's dimension; declare `:exact` if the signal is integer-valued | 3 | BD-122 |
| FROB-REF-523 | A future-time operator appears in an online signal | `G`, `F` or `U` inside `(online …)` | Move the formula into the verdict; rewrite as past time (G over [0, T] at 0 equals H over [0, T] at T) | 3 | BD-122 |
| FROB-REF-525 | A signal is read across clocks without a declared reduction, or its channel kind does not admit the reduction | A faster-clock signal with no `last`, `min` or `max`; an interval-extremum channel reduced by another reduction | Declare the reduction; use the extremum matching the channel kind | 3 | BD-122 |
| FROB-REF-526 | The episode horizon is shorter than the reach of the verdict | Reach (the sum of nested window ends) exceeds the task's horizon | Lengthen the horizon; shorten a window | 3 | BD-122 |
| FROB-REF-528 | An online window exceeds its capacity | An online window longer than 4,096 requirement ticks (TARGET) | Shorten the window; move the formula to the verdict (offline only); use a slower requirement clock | 3 | BD-122 |
| FROB-REF-529 | No settled-pose cache for this model and task | A run is started for a (model, spawn specification, contact law, kernel versions) key with no cache; a design edit changed the key | Run `task.settle` for this model and task; settle the edited model | 3 | N24a-16 |
| FROB-REF-530 | A run offered as a fixture run does not match the fixture's recorded hash | The recomputed hash of constants, seeds, input table and rule differs from the hash written to the ledger before the fixture's first run | Re-run from the registered fixture identity; register the changed fixture under a new identity; cite the run as an ordinary run | 3 | The cart fixture bead |
| FROB-REF-532 | A fixed-n study has no pre-analysis, or names a verdict it cannot reach | A `fixed-n` plan without the reachability pre-analysis, or whose declared verdict set contains a verdict unreachable at the declared n and level (a practical tie at n = 64 with a five-point margin) | Run `eval.preanalyze`; raise n to the first n at which the verdict is reachable (86 for the tie); remove the verdict from the declared set | 3 | The study-types bead |
| FROB-REF-533 | Paired statistics requested under an uncontrolled determinism class | A paired study with an arm that is not class D2 on one backend, nor D3 with decision-level equivalence recorded for the task; any D4 arm | Run both arms under a replayable profile on one backend; record decision-level equivalence; compare unpaired by declaration | 3 | The study-types bead |
| FROB-REF-535 | Dimension mismatch inside an atom | The two sides of a comparison, or the terms of a sum, have different dimensions | Correct the unit of the offending term | 3 | BD-122 |
| FROB-REF-536 | A temporal operator has no finite window | A missing or unbounded window on H, O, S, G, F or U | Add a finite window | 3 | BD-122 |
| FROB-REF-538 | A window bound is not a multiple of the clock period and no rounding is declared | A bound that is not an integer multiple of the requirement period, compared in exact rationals | Declare `:round inward` or `:round outward`; change the bound to a multiple; change the requirement clock | 3 | BD-122 |
| FROB-REF-539 | A requirement has no labelled examples, or the monitor disagrees with a label | Fewer than two satisfied and two violated examples; an example whose monitor verdict differs from its label; a vacuous hard-limit atom | Correct the formula at the binding atom reported; correct the label; add a near-boundary example | 3 | BD-122 (absence), N24a-12 (disagreement) |
| FROB-REF-552 | A request mixes study types | A fixed-n interval asked of an anytime study at its stopping time; a fixed-n study stopped on a running p-value; one study declared with both types | Declare one type before the first episode; open a second study for the other inference | 3 | The study-types bead |
| FROB-REF-840 | A session without `qual:admin` asks for sealed confirmation data before close | Any verb asked for a sealed world address, per-unit outcome, trace or still of an open campaign; the attempt is logged | Read `eval.ledger` (index, levels, receipts); wait for the campaign to close | 3 | The firewall bead |
| FROB-REF-841 | A confirmation request was not committed first, or differs from its commitment | A run step with no prior commitment record, or whose request hash differs from the committed one | Commit the request, then submit it unchanged; submit a new request (which consumes the next index) | 3; 7 when the same idempotency key carries different content | The qualification-lane bead |
| FROB-REF-842 | The units the request needs at its level exceed its budget; the reachable verdicts are attached | At level α_i the study rule needs more clusters than the request's budget allows for the claimed verdict | Raise the budget; claim a verdict reachable at this n (the attachment lists them); accept that later requests cost more (ln(1/α_i) growth) | 3 | The qualification-lane bead |
| FROB-REF-843 | The candidate's lineage contains data from the sealed family | A candidate, controller or policy whose training or tuning lineage includes any world of the sealed family | Retrain without the data; the family is retired if it informed selection | 3 | The qualification-lane bead |
| FROB-REF-844 | A cluster would be used twice | A cluster derivation that collides with a cluster already served to any request, or that appears in a discovery dataset | None for the caller (the service derives another cluster); a persistent collision is an integrity fault to report | 3 | The qualification-lane bead |
| FROB-REF-845 | A result is cited as qualified without a promoting receipt | A report, scorecard or claim that uses the wording "qualified" with no receipt whose decision is `promoted` | Cite the result as a discovery-lane comparison; obtain a receipt through `eval.qualify` | 3 | The qualification-lane bead; report verbs |

**Refusals the late Arena subsections define with no code number.** The plan gives each a rule and no number.
They are registered under these symbolic names in `registries/refusals.toml` and numbered by the registry owner
at implementation (`5xx` for evaluation protocol, `8xx` for evidence and protocol); until then tests match the
name. Numbers are never reused.

| Symbolic name | Rule | Bead |
|---|---|---|
| `metric-moment-free-under-moment-command` | A scorecard ranks contact geometries by moment-free capacity when the task commands a moment | N24c-18 |
| `foil-as-candidate` | A foil body offered to an optimisation verb, a comparison or the qualification lane | N24c-23 |
| `task-audit-missing`, `task-audit-unresolved` | `eval.qualify` on a task with no audit record for its `TaskId`, or with an unresolved finding (idleness findings included) | N24c-22 |
| `wins-outside-intent-unresolved` | Promotion or qualification of a design whose winning episodes violate an adopted intent invariant | N24c-21 |
| `item-bank-in-fixed-n`, `item-bank-in-qualification`, `item-bank-with-tier-correction`, `item-bank-needs-variance-adaptive` | Item-bank sampling in a fixed-n study, in the blinded lane, combined with the tier-correction estimator, or with a range-driven sequence | N24c-31 |
| `stratified-sampling-not-admitted` | Stratified sampling outside anytime discovery studies (fallback branch only) | N24c-35 |
| `exposure-domain-missing`, `exposure-event-undeclared` | A randomisation draw with no domain; an `event` draw on an undeclared mission event | N24c-36 |
| `coupling-test-failed`, `exposure-table-untested` | A coupling that fails one of the four checks; a paired study on a task whose exposure table has not passed | N24c-37 |
| `coupling-choice-after-draw` | Confirmation addresses derived before the coupling lock, or the pilot seed reused | N24c-40 |

**Codes owned by other chapters that Arena mechanisms raise.** FROB-REF-311 (a scene group straddles a split);
FROB-REF-542, -543, -553, -555, -556 (paired study under an uncontrolled class; an anytime study on a metric with
no declared span; a comparison that mixes study types; a fixed-n study with an unreachable verdict; a pairing
keyed by event order or a training counter), which the comparison chapter defines for the same situations seen
from `eval.compare`; FROB-REF-549 and -560 (rare-failure bounds over selected or non-nominal episodes);
FROB-REF-554 (a future-time requirement term offered as an online reward); FROB-REF-561 (a paired statistic with
a D4 arm); FROB-REF-817 (sampling-controller episodes offered as a scoring source); FROB-REF-888 (a metric whose
samples are missing); FROB-REF-891 to -893 (format version, view offered as identity, owner bypass).

## Acceptance criteria

- [ ] `registries/refusals.toml` holds the 21 codes with message, family, defining section, `retry` class and at least one repair each; the loader rejects an unknown or duplicated code.
- [ ] For each of the 21 codes a planted trigger raises exactly that code through `frob --json`, with exit code 3 (7 for the conflicting-key case of 841), a `pointer`, and `repairs[0]` also present as `next_actions[0]`.
- [ ] The nine requirement-language codes (520, 535, 536, 523, 538, 525, 526, 539, 528) are raised by their planted task files (the refusal rows of GATE-ARENA-03).
- [ ] Every verb of the Arena table lists in the verb registry the codes it can raise, and no verb raises an unlisted code in the test suite.
- [ ] The symbolic refusals are registered and each has a test in its mechanism bead; none has silently borrowed an existing number.
- [ ] Refusal messages contain no private-scene data and no sealed data.

## Test plan

### Unit tests
`cargo test -p frob-arena refusals`: `registry_has_21_arena_codes`; `no_code_reused` (against the whole
registry); one `raises_<code>` test per code with its planted trigger: 502 (a battery containing the mirror of a
training scene), 520 to 539 (the planted task files), 529 (a run on an edited design), 530 (a fixture run with
one changed constant), 532 (a tie verdict requested at n = 64), 533 (a paired study with a D4 arm), 552 (a
fixed-n interval asked of an anytime study), 840 (a discovery session asking for a sealed outcome), 841 (a run
without commitment), 842 (256 clusters budgeted where the level needs more), 843 (a candidate trained on a
sealed world), 844 (a forced cluster collision), 845 (a report saying "qualified" with no promoting receipt);
`every_refusal_has_repair`; `first_repair_is_next_action`; `exit_code_by_family`.

### End-to-end script and logging
The codes are exercised from the user's side by the capability scripts: `scripts/e2e/arena_task_file.sh` (the
nine language codes), `scripts/e2e/arena_m1_families.sh` (529), `scripts/e2e/gate_arena_02_scene_split.sh` (502), and the
qualification lane's script (840 to 845). Each logs `refusal` events with `code_expected`, `code_observed`,
`pointer` and the first repair. A registry step in `arena_task_file.sh` compares `frob capabilities --json` with
this table.

### Known answers and negative controls
KA-597 (the planted task files trigger their refusals; exact). KA-612's control (a request for a tie verdict at
n = 64 is refused with 532). Negative controls: a verb that returns a truncated answer where it should refuse (a
verdict from a short trace in place of `undetermined`; a scorecard on a training scene) is caught by its
mechanism's test; a registry with a duplicated number fails to load.

## Considerations

- Several Arena situations have two codes, one from this chapter and one from the comparison chapter (study-type mixing: 552 and 553; unreachable verdict: 532 and 555; uncontrolled class: 533 and 542). The registry keeps both; the rule of thumb is that the Arena code is raised by study planning and the comparison code by `eval.compare`, and a test asserts that one situation raises one code on one verb.
- Refusal 842 is a budget-shaped refusal with an `8xx` number; by the family rule its exit code is 3, not 4, and the reachable verdicts are attached so the caller can act.
- Repairs are advice ranked by cost; they never change the caller's request silently.
- The seed text for the task-file bead speaks of "refusals 520 to 528"; the nine language codes are the ones listed here (four of them now carry numbers above 528).

## References

§24.14, §24.5, §24.7, §24.10, §24.12, §24.13, §32.4, §35.11, Appendix O; GATE-ARENA-03; KA-597, KA-612.
