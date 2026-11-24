---
key: "N24-REF"
title: "registry: the 21 Arena refusal codes (502, 520 to 539, 552, 840 to 845) with triggers, repairs, exit codes"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "arena", "frob-arena", "frob-spec", "registry", "test", "solid", "review-notes-open"]
parent: "WP-41"
deps: ["BD-005", "N35-33"]
needs: []
related: ["BD-259", "N24a-12", "N24a-16", "N24a-31", "N24b-15", "N24b-16", "N24c-13", "N26-REF", "N30-REF", "N35x-01", "N39-REF"]
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
and covers `FROB-REF-1xx` to `5xx` and `8xx` other than conflicts; exit code 5 is a capability not implemented or
not admitted, or a request outside the session token's scopes; exit code 7 is a conflict (idempotency key reuse
with different content, stale parent, or a conflict with stored state). FROB-REF-840 is the one Arena code with
exit 5: it refuses a session that lacks the `qual:admin` scope, and the safety chapter's refusal table, which
lists 840 under its owner (this chapter) and retires FROB-REF-730 in its favour, gives it exit 5. The firewall
(BD-261), the safety registry (N37-REF) and the verb `eval.qualify` (BD-259) carry the same exit.

## Scope

In: the 21 registry entries; a test that each is registered, raised by its planted trigger, and carries at least
one repair; the list of refusals that the late Arena subsections define without code numbers; the list of codes
owned elsewhere that Arena mechanisms raise.
Out: implementing the mechanisms that raise them (the beads named per row).

## Technical approach

**The 21 codes.** Each entry carries `defined_in = "§24.14"` (the chapter's refusal table) and `raised_by`, the
bead keys of the last column (the fields N35-33's registry-wide check reads).

| Code | Meaning | Trigger | Ranked repairs | Exit | Raised by |
|---|---|---|---|---|---|
| FROB-REF-502 | An evaluation reuses a scene from the policy's training manifest (carried from the scene-split rule) | At scoring time a battery scene's split group intersects the training manifest of the candidate's lineage, or the lineage has a node with no manifest | Evaluate on the held-out side of the split; publish a battery version without the scene (a recorded event that changes the battery identity); report the score as in-distribution, not held-out | 3 | N24a-31 |
| FROB-REF-520 | An atom has no scale and is not declared exact | `(atom …)` without `:scale <quantity>` or `:exact`; `:exact` on a non-integer signal | Add a scale of the expression's dimension; declare `:exact` if the signal is integer-valued | 3 | BD-122 |
| FROB-REF-523 | A future-time operator appears in an online signal | `G`, `F` or `U` inside `(online …)` | Move the formula into the verdict; rewrite as past time (G over [0, T] at 0 equals H over [0, T] at T) | 3 | BD-122 |
| FROB-REF-525 | A signal is read across clocks without a declared reduction, or its channel kind does not admit the reduction | A faster-clock signal with no `last`, `min` or `max`; an interval-extremum channel reduced by another reduction | Declare the reduction; use the extremum matching the channel kind | 3 | BD-122 |
| FROB-REF-526 | The episode horizon is shorter than the reach of the verdict | Reach (the sum of nested window ends) exceeds the task's horizon | Lengthen the horizon; shorten a window | 3 | BD-122 |
| FROB-REF-528 | An online window exceeds its capacity | An online window longer than 4,096 requirement ticks (TARGET) | Shorten the window; move the formula to the verdict (offline only); use a slower requirement clock | 3 | BD-122 |
| FROB-REF-529 | No settled-pose cache for this model and task | A run is started for a (model, spawn specification, contact law, kernel versions) key with no cache; a design edit changed the key | Run `task.settle` for this model and task; settle the edited model | 3 | N24a-16 |
| FROB-REF-530 | A run offered as a fixture run does not match the fixture's recorded hash | The recomputed hash of constants, seeds, input table and rule differs from the hash written to the ledger before the fixture's first run | Re-run from the registered fixture identity; register the changed fixture under a new identity; cite the run as an ordinary run | 3 | N24b-03 (the fixture registry; the cart batch N24b-07 and the fixtures' runs raise it through it) |
| FROB-REF-532 | A fixed-n study has no pre-analysis, or names a verdict it cannot reach | A `fixed-n` plan without the reachability pre-analysis, or whose declared verdict set contains a verdict unreachable at the declared n and level (a practical tie at n = 64 with a five-point margin) | Run `eval.preanalyze`; raise n to the first n at which the verdict is reachable (86 for the tie); remove the verdict from the declared set | 3 | BD-124 (called by `study::admit`, N24b-14) |
| FROB-REF-533 | Paired statistics requested under an uncontrolled determinism class | A paired study with an arm that is not class D2 on one backend, nor D3 with decision-level equivalence recorded for the task; any D4 arm | Run both arms under a replayable profile on one backend; record decision-level equivalence; compare unpaired by declaration | 3 | BD-124 (called by `study::admit`, N24b-14) |
| FROB-REF-535 | Dimension mismatch inside an atom | The two sides of a comparison, or the terms of a sum, have different dimensions | Correct the unit of the offending term | 3 | BD-122 |
| FROB-REF-536 | A temporal operator has no finite window | A missing or unbounded window on H, O, S, G, F or U | Add a finite window | 3 | BD-122 |
| FROB-REF-538 | A window bound is not a multiple of the clock period and no rounding is declared | A bound that is not an integer multiple of the requirement period, compared in exact rationals | Declare `:round inward` or `:round outward`; change the bound to a multiple; change the requirement clock | 3 | BD-122 |
| FROB-REF-539 | A requirement has no labelled examples, or the monitor disagrees with a label | Fewer than two satisfied and two violated examples; an example whose monitor verdict differs from its label; a vacuous hard-limit atom | Correct the formula at the binding atom reported; correct the label; add a near-boundary example | 3 | BD-122 (absence), N24a-12 (disagreement) |
| FROB-REF-552 | A request mixes study types | A fixed-n interval asked of an anytime study at its stopping time; a fixed-n study stopped on a running p-value; one study declared with both types | Declare one type before the first episode; open a second study for the other inference | 3 | BD-124 (called by `study::admit`, N24b-14) |
| FROB-REF-840 | A session without `qual:admin` asks for sealed confirmation data before close | Any verb asked for a sealed world address, per-unit outcome, trace or still of an open campaign; the attempt is logged | Read `eval.ledger` (index, levels, receipts); wait for the campaign to close | 5 (outside the session's scopes) | BD-261 (the confirmation firewall) |
| FROB-REF-841 | A confirmation request was not committed first, or differs from its commitment | A run step with no prior commitment record, or whose request hash differs from the committed one | Commit the request, then submit it unchanged; submit a new request (which consumes the next index) | 3 | N24b-16, through `eval.qualify` (BD-259) |
| FROB-REF-842 | The units the request needs at its level exceed its budget; the reachable verdicts are attached | At level α_i the study rule needs more clusters than the request's budget allows for the claimed verdict | Raise the budget; claim a verdict reachable at this n (the attachment lists them); accept that later requests cost more (ln(1/α_i) growth) | 3 | N24b-16, through `eval.qualify` (BD-259) |
| FROB-REF-843 | The candidate's lineage contains data from the sealed family | A candidate, controller or policy whose training or tuning lineage includes any world of the sealed family. The same rule at two further sites: an item-bank refit inside `eval.compare` whose row set holds a row of a sealed confirmation family (N24c-30); and, from M8, a paired run of `mission.transport --coupling-source paired-runs` whose world address appears in either controller's training or tuning lineage (N25e-04) | Retrain without the data; the family is retired if it informed selection. For the item-bank refit: drop the rows of the sealed family, or wait for its retirement into discovery | 3 | N24b-16 (reading N24b-17's lineage mark); N24c-30 (the item-bank refit of `eval.compare`, M3); N25e-04 (`mission.transport`, M8) |
| FROB-REF-844 | A cluster would be used twice | A cluster derivation that collides with a cluster already served to any request, or that appears in a discovery dataset | None for the caller (the service derives another cluster); a persistent collision is an integrity fault to report | 3 | N24b-17 (sealed cluster derivation) |
| FROB-REF-845 | A result is cited as qualified without a promoting receipt | A report, scorecard or claim that uses the wording "qualified" with no receipt whose decision is `promoted` | Cite the result as a discovery-lane comparison; obtain a receipt through `eval.qualify` | 3 | N24b-19 (the standing `qualified`); report verbs |

**Refusals the Arena text defines with no code number** (the late subsections, two rules of §24.5 that the
verdict verb enforces, the structural rule of the task file, which §24.4 implies and N24a-02 states, the
battery membership rule of §24.8, which N24b-01's loader enforces, and the load rule of the disturbance-law
registry, which N24cp3-02 states). The plan gives each a rule and no number.
Each is a row of the refusal allocation ledger `registries/refusal_allocations.toml` under its symbolic name as
the slug (N35-33 owns the ledger and its rule): the fifteen on file when the first round ran were filed there by
N35-33's first round ("Arena (N24-REF): fifteen refusals registered by symbolic name"), and every later one is
appended by this bead (`requested_by = "N24-REF"`; the ledger is append-only and later requests follow the same
rule). The family is decided by what is refused, not by this chapter (`5xx` for evaluation protocol, `8xx` for
evidence and protocol); an `8xx` row is held while the owner decision N35x-01 holds that family. Until a number
is assigned the raising bead emits the ledger's interim form with the slug behind one named constant, and tests
match the slug. Numbers are never reused.

| Symbolic name | Rule | Bead |
|---|---|---|
| `task-structure-invalid` | `5xx`, exit 3: a task file whose structure fails at load: a missing section or required field, a repeated name within a section, a reference that does not resolve (clock, signal, atom, verdict, random stream), not exactly one verdict, no atom, no `req` clock, or `thermal_start` weights that are not exact rationals summing to one; `observed` names the kind (`missing`, `duplicate-name`, `dangling`, `count`, `weights`). Unknown or misspelt members and duplicate keys are FROB-REF-103, an unknown major version FROB-REF-891. Repairs: declare the referenced name; remove the duplicate; correct the count or weights | N24a-02, N15a-29 |
| `battery-manifest-invalid` | `5xx`, exit 3: a battery manifest refused at load (§24.8 gives the rule that a removal is a recorded event with a reason and changes the identity, and no code); `observed` names the kind: `no-event` (the manifest's `BatteryId` is not reached from the last registered identity by the declared events: an item added, removed or edited by hand), `event-mismatch` (an event's `from` or `to` is not the identity of its item set), `empty-reason`, `record-edited` (a recorded event edited or deleted in the events file), `schema` (a missing field, a lane other than `discovery`, an item whose task resolves to no task file). Unknown, misspelt and duplicate keys are FROB-REF-103. Repairs: append the event with the item and a reason; restore the registered item set; correct the field named by the pointer | N24b-01 (returned through `eval.run`, N24c-04) |
| `verdict-task-mismatch` | `task.verdict` asked to evaluate a run under a task other than the one it recorded, without `--re-evaluate` (DESIGN of the verb; the plan names no code) | N24c-02 |
| `stage-concatenation` | A mission verdict assembled from separately recorded stage episodes (CX-269; the plan states the rule and no code) | N24c-02 |
| `metric-moment-free-under-moment-command` | A scorecard ranks contact geometries by moment-free capacity when the task commands a moment | N24c-18 |
| `wrench-command-missing` | A task reads a traction metric and declares neither a `wrench_command` nor `moments: free` (no default) | N24c-18 |
| `foil-as-candidate` | A foil body offered to an optimisation verb, a comparison or the qualification lane (retired, never reused, if KILL-FOIL falls back) | N24c-23 |
| `audit-arm-not-built` | `task.audit --arms` names an arm this build does not contain (the foil arm after KILL-FOIL's fallback, or the actual-design arm under the pilot's falsifier) | N24cp1-01 |
| `task-audit-missing`, `task-audit-unresolved` | `eval.qualify` on a task with no audit record for its `TaskId`, or with an unresolved finding (idleness findings included) | N24c-22 |
| `wins-outside-intent-unresolved` | Promotion or qualification of a design whose winning episodes violate an adopted intent invariant | N24c-21 |
| `intent-signal-misused` | A reward term or termination rule references an `:intent` signal (monitored, not required) | N24c-21 |
| `item-bank-in-fixed-n`, `item-bank-in-qualification`, `item-bank-with-tier-correction`, `item-bank-needs-variance-adaptive` | Item-bank sampling in a fixed-n study, in the blinded lane, combined with the tier-correction estimator, or with a range-driven sequence | N24c-31 |
| `stratified-sampling-not-admitted` | Stratified sampling outside anytime discovery studies (fallback branch only) | N24c-35 |
| `sampling-mode-not-built` | `eval.compare --sampling item-bank` after KILL-ITEMBANK's fallback withdrew the mode (the four item-bank refusals above are then retired, never reused) | N24c-35 |
| `exposure-domain-missing`, `exposure-event-undeclared` | A randomisation draw with no domain; an `event` draw on an undeclared mission event | N24c-36 |
| `coupling-test-failed`, `exposure-table-untested` | A coupling that fails one of the four checks; a paired study on a task whose exposure table has not passed | N24c-37 |
| `coupling-choice-after-draw` | Confirmation addresses derived before the coupling lock, or the pilot seed reused | N24c-40 |
| `coupling-family-too-large` | `5xx`, exit 3: `task.check --coupling --choose` on a task whose candidate coupling tables (the product of the draws' alternatives) number more than 16 (DESIGN cap of N24c-40); refused before any world runs; `pointer` the draws that carry alternatives, `detail` the product count. Repairs: 1. declare a single key for the draws whose choice is settled; 2. split the family into tasks of at most 16 candidates | N24c-40 |
| `stream-adapter-missing`, `coupling-choice-not-built` | After KILL-EXPODOMAIN's fallback: a draw whose stream has no explicit adapter; `task.check --coupling --choose` | N24cp1-02 |
| `disturbance-registry-invalid` | `5xx`, exit 3: a verb that needs the disturbance-law registry `registries/disturbances.toml` (`task.new` and `task.check` on a task that draws from `registry:disturbances`; `sim.run --task` at environment admission) finds that the registry does not load: an entry with both `range` and `value` or with neither, a `range` with no `distribution`, a lower bound above its upper bound, a unit that does not parse or does not fit the entry, an unknown field, a duplicate slug, an unknown `kind` or `applies_to` form. No partial registry is served; `pointer` the file, slug and field. Repairs: 1. correct the row; 2. restore the registry from the repository | N24cp3-02 |
| `campaign-already-open` | `8xx`, exit 7 (a conflict with the ledger's state, DESIGN): a second campaign opened on a claim family that has one open, or more open campaigns than `families_per_task_family` across one task family; the refusal names the open campaign and leaves every index sequence untouched. Repairs: submit to the open campaign; ask the holder of `qual:admin` to close it first | N24b-15 |
| `campaign-request-cap` | `8xx`, exit 3: request `max_requests + 1` of a campaign that carries the optional hard cap (the BET-21 fallback as a setting); refused before commitment, with no index assigned and no level spent. Repairs: ask the holder of `qual:admin` to open a campaign on a fresh sealed family | N24b-15 |
| `campaign-not-open`, `claim-family-mismatch` | `8xx`, exit 3: a qualification request to a campaign that is not open (never opened, or closed), or whose task or baseline class is not the campaign's claim family; refused before commitment, nothing spent. Repairs: ask the holder of `qual:admin` to open a campaign for this claim family; submit to the campaign whose claim family matches | N24b-16 |
| `qualification-undecided` | `8xx`, exit 3: `eval.ledger --open` on a real (non-`synthetic`) claim family while `config/qualification.toml` has `decided: false` (a missing or malformed file loads as `decided: false`): a real claim family has no sealed family until the decision exists, the fail-closed posture of N24c-13. A battery campaign needs no sealed world family and is not refused. Repairs: 1. record the OPEN-ARENA-3 decision (N24c-13); 2. use a synthetic claim family for tests | N24b-15 |

**Codes owned by other chapters that Arena mechanisms raise.** FROB-REF-311 (a scene group straddles a split);
FROB-REF-542, -543, -553, -555, -556 (paired study under an uncontrolled class; an anytime study on a metric with
no declared span; a comparison that mixes study types; a fixed-n study with an unreachable verdict; a pairing
keyed by event order or a training counter), which the comparison chapter defines for the same situations seen
from `eval.compare`; FROB-REF-549 and -560 (rare-failure bounds over selected or non-nominal episodes);
FROB-REF-554 (a future-time requirement term offered as an online reward); FROB-REF-561 (a paired statistic with
a D4 arm); FROB-REF-817 (sampling-controller episodes offered as a scoring source); FROB-REF-888 (a metric whose
samples are missing); FROB-REF-891 to -893 (format version, view offered as identity, owner bypass);
FROB-REF-103 (an unknown, misspelt or duplicate field, defined by the design chapter and raised by the task-file
reader for the same defect, N24a-02);
FROB-REF-701 (exit 4: a qualification request whose cost the session's budget cannot pay, refused before
commitment, N24b-16); FROB-REF-720 (exit 5: a session without `qual:admin` opening or closing a campaign, or
without `qual:run` submitting a request, N24b-15 and N24b-16); FROB-REF-567 (any world set the caller names as the
confirmation sample: a search-selected or hard world, a battery such as `hard` or `covering`, or development
addresses; one code for one defect, since every named world is a world the lane did not draw; `observed` names
which; N24b-16, whose `clusters` is a count and whose lane draws its own worlds) and FROB-REF-825 (a D4 result
offered as evidence, N24b-16); FROB-REF-803 (exit 7: an idempotency key reused with different resolved content,
the surface's idempotency rule, KA-763: at the `eval.qualify` commit, refused with no index assigned, N24b-16
step 3 and BD-259, and at `eval.ledger --open` and `--close`, N24b-15) and FROB-REF-805 (exit 3: a `run` request
with no key, BD-259);
FROB-REF-886 (reuse of a spent holdout, defined by the grounding chapter: in Arena it is also the refusal of a
retired confirmation family, whether closed and revealed or retired early, offered to a new campaign, a draw or a
score, N24b-17, exit 3).

FROB-REF-802 (arguments fail the schema, exit 3) is the code of study admission checks 5 to 7 (N24b-14): a study
over clustered worlds declaring `unit: world` (pointer `/seed_protocol/unit`), a pairwise study of an N-way race
carrying a level other than α/(N − 1) (pointer to that `level`, `required` the exact rational), and a design
comparison with no controller arm or with arms that contradict it (pointer `/controller_arm`). DESIGN: each
defect is a field of the request whose admissible value is
fixed by a rule over the request and the records it names, so the pointer and `required` tell the caller exactly
what to change and a dedicated code would carry no further information. If a caller is found that must tell
these apart from a malformed argument by code alone, a `5xx` code is assigned then and recorded here.

## Acceptance criteria

- [ ] `registries/refusals.toml` holds the 21 codes with message, family, defining section, `retry` class and at least one repair each; the loader rejects an unknown or duplicated code.
- [ ] For each of the 21 codes a planted trigger raises exactly that code through `frob --json`, with exit code 3 (5 for 840), a `pointer`, and `repairs[0]` also present as `next_actions[0]`. A reused idempotency key with different content at the qualification commit is FROB-REF-803 with exit 7, not 841. This registry depends on none of its raisers: a code whose raising bead has not landed is reported `pending: <raising bead>`, and a pending code fails the check from its raiser's milestone exit on (M1 for the language codes, 529, 530, 532, 533 and 552; M3 for 502 and 840 to 845, raised by N24a-31, BD-261, N24b-16, N24b-17, N24b-19 and BD-259).
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
sealed world), 844 (a forced cluster collision), 845 (a report saying "qualified" with no promoting receipt),
each reported `pending: <raising bead>` while its raiser has not landed, as the acceptance criteria state;
`every_refusal_has_repair`; `first_repair_is_next_action`; `exit_code_by_family` (every code 3 except 840 → 5;
the symbolic `campaign-already-open` → 7, the other symbolic names → 3); `conflicting_key_is_803_not_841` (the
same key with altered content at the commit → FROB-REF-803, exit 7, no index assigned); `undecided_refuses_real_family`
(with `decided: false`, opening a real claim family → `qualification-undecided`, exit 3; a synthetic claim family
and a battery campaign are admitted); `named_worlds_are_567` (`clusters` naming the `hard` battery, development
addresses, or a search-selected world → 567 with `observed` naming the kind); the last three need the
qualification lane (BD-259, N24b-15, N24b-16, M3) and are reported `pending` the same way until it lands.

### End-to-end script and logging
The codes are exercised from the user's side by the capability scripts: `scripts/e2e/arena_task_file.sh` (the
nine language codes), `scripts/e2e/arena_m1_families.sh` (529), `scripts/e2e/gate_arena_02_scene_split.sh` (502), and
`scripts/e2e/arena_qualification_lane.sh` (N24b-24: 840 with exit 5, 841 to 845, and the symbolic campaign
refusals). Each logs `refusal` events with `code_expected`, `code_observed`,
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
