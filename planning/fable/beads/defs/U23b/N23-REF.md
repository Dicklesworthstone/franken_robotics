---
key: "N23-REF"
title: "Cortex refusal registry: twelve codes (521 to 554, 830, 867) and eight late refusal requests"
type: "task"
priority: 1
milestone: "M2"
labels: ["m2", "cortex", "frob-learn", "frob-core", "solid", "registry", "test", "e2e", "review-notes-open"]
parent: "WP-48"
deps: ["BD-005", "BD-022", "N35-33", "STD-E2E"]
needs: []
related: ["BD-323", "BD-328", "N23a-03", "N23a-05", "N23a-08", "N23a-13", "N23a-18", "N23a-24", "N23a-25", "N23a-27", "N23a-31", "N23a-32", "N23b-03", "N23b-04", "N23b-08", "N23b-12", "N23b-15", "N23bx-03", "N28-REF", "N28b-23", "N31ax-01", "N33-REF", "N35-REF", "N41-REF"]
gates: []
plan_refs: ["§23.19", "Appendix O (O.5, O.8)", "§32.4", "§35.11", "§23.5", "§23.10", "§23.13", "§23.15", "§23.17", "§23.25", "§23.26", "§23.28", "§28.20"]
---
## Background

Every way in which FrankenRobotics declines to answer is a typed refusal: a stable code, a message, the field at
fault, observed and required values with units, and repairs ranked by how likely they are to help. The standing
rules are: refuse, never truncate; a misspelt or unsupported request is a refusal and not a default; a refusal that
exists only as a row of a table and is never raised is worse than none, because agents plan around repairs the
system never offers. Codes live in one machine-readable registry, `registries/refusals.toml`, loaded by
`frob-core`; an unknown code is a load-time error and a code is never reused. Families: `1xx` design and model,
`2xx` numerics, `3xx` scene, `4xx` control and estimation, `5xx` learning and evaluation protocol, `6xx` backend
and device, `7xx` budget and capability, `8xx` evidence and protocol.

Cortex (the learning subsystem: `frob-learn` and the annex `bindings/frob-gymnasium`) uses the `5xx` family and
two codes of the evidence family. This bead is the registry of the twelve codes Cortex defines, plus eight
refusals that have no code: four that later sections state in words (three in §23.25, §23.26 and §23.28, and the
rule of §23.15 that randomization touches physical parameters only), the refusal of a task that lacks a
declaration the term contract of §23.5 requires, which the plan states as a requirement and never numbers, and
two evaluation rules the plan states as prohibitions: a distilled student is never evaluated on teacher states
(§23.10), and no adaptive randomization chooses evaluation episodes (§23.17); and the refusal of a weight tying
or an augmentation requested over a trivial symmetry group, which the two tying mechanisms of §23.13 state
(DESIGN of N23a-31 and N23a-32: the plan names the groups and no refusal). It fixes the content and the exit code of each row
and owns the test that each code is raised by its trigger. The mechanisms that raise them are other beads, named
per row, so that no code is left without a raising site.

## Scope

In: the twelve registry rows with meaning, trigger, envelope fields, ranked repairs, retry class and exit code;
the eight late refusals as allocation requests (four with proposed codes); the disambiguation against nine neighbouring codes of other
chapters; the registry tests; one end-to-end script that provokes each code from the command line.
Out: the detection logic of each condition (the raising beads); the allocation ledger and its rule (N35-33); the
row for randomization on a card field with no interval (N23bx-03).

## Technical approach

**Envelope shape (FACT about the surface contract, schema `frob.envelope/1`).** When `ok` is false the envelope
carries `refusal = { code, message, pointer, observed, required, unit, affected_claims, retry, repairs[] }`;
`retry` is `never`, `after-change` or `backoff`; each repair is a typed suggestion (a verb with arguments or a
typed design edit) with a one-line reason. Exit 3 means refused input or question and covers codes `1xx` to `5xx`
and `8xx` other than conflicts; exit 1 is never emitted on purpose. All twenty rows below exit 3.

**The twelve rows.**

| Code | Refused | Raised when (trigger) | `pointer`, `observed`, `required` | Ranked repairs | `retry` | First milestone | Raising bead |
|---|---|---|---|---|---|---|---|
| FROB-REF-521 | `step` on a world that needs reset, in explicit mode | `VecEnv::step` receives an action for a world whose `needs_reset` (terminated, truncated or invalid) is set and the run's reset mode is `explicit` (environment rule E-1). Checked before any state changes | The world's logical id and the flag that ended it; `observed` = `needs_reset: true`; `required` = a reset of that world | 1. Reset the world (`reset(mask)`). 2. Run in the labelled mode `autoreset: masked`, where the world is reset inside the step and the final observation travels in `final_obs` | `after-change` | M2 | BD-192 |
| FROB-REF-522 | A non-finite action, before any state changes | Any element of the action tensor is NaN or infinite (rule E-5); the check runs before the limiter and before any world is mutated, so the whole batch is unchanged | World id and channel id of the first offending element; `observed` = the value's class (`nan`, `+inf`, `-inf`); `required` = finite | 1. Fix the policy output (the world and channel are named). No second repair: the environment does not sanitize | `after-change` | M2 | BD-192 |
| FROB-REF-524 | The actor schema reads a `critic-only` or `evaluation-only` channel | A learner component asks to read, as the actor, a channel that the task classifies `critic-only` or `evaluation-only` (rule E-8): a network specification names it as an actor input; a checkpoint's actor schema names it when checked against the current task; or a host caller requests an actor view that includes it. The actor is always run with the other channels physically absent | The channel id and its access class; `required` = `actor` | 1. Move the channel to the critic (asymmetric actor-critic). 2. Declare a privileged teacher and distill a student whose schema omits the channel | `after-change` | M2 | N23a-03 (raise; integrated in BD-192); extended to rewards, flags, masks and `info` by N23b-31 |
| FROB-REF-527 | The action law at learning differs from the law at collection | The registered action law of a rollout (`latent-then-limit` or `clipped-with-endpoint-mass`) differs from the law the update evaluates, or a batch mixes the two (rule P-3, INV-CORTEX-03) | The rollout id; `observed` = the law at collection; `required` = the law at learning | 1. Recollect under the learner's law. 2. Relabel the stored samples under the registered law (possible only where the stored variable allows it: a stored latent can be relabelled, a stored clipped action cannot recover its latent) | `after-change` | M2 | N23a-08 (raise), BD-197 (battery), N23a-14 (a resume under another law) |
| FROB-REF-531 | A policy's mapping table does not match the design, or cannot be built for an edit | Loading or applying a checkpoint whose `frob.policy/1` channels (roles, dimensions, signs, units, rates) do not match the target design by role; or an inheritance request whose typed edit declares no usable correspondence (INV-CORTEX-04) | The first unmatched role with the differing attribute; `observed` and `required` = the two attribute values with units | 1. Supply the correspondence in the typed edit. 2. Train fresh | `after-change` | M2 (load), M4 (edits) | N23a-13 (role-matched loading), N23a-27 (an inheritance map that cannot be built), BD-320 (the family mapping table), N23a-25 (binding) |
| FROB-REF-534 | A family policy applied outside its class or its pretraining distribution | `policy.apply` or a family-policy score is requested for a design that fails the membership rule: outside the parameter box of the pretraining distribution, or a topology signature outside the class | The design id; the failing parameter with its value and the box, or the topology signature and the class | 1. Screen with the baseline controller or the sampling controller. 2. Train a specialist | `after-change` | M4 | BD-320 (membership at every scoring path), N23a-25 (`policy.apply`) |
| FROB-REF-537 | A rung used to select or discard without a rank-agreement record that meets its threshold | A ladder allocation asks a rung to select or discard in a stratum that has no rank-agreement record (an unmeasured stratum fails closed), or asks it to discard in a stratum whose record makes it `uninformative`: the lower 95% bound on top-band Kendall τ is below 0.5, or the planted winner was lost in this stratum (INV-CORTEX-06). A request to *select* where a record exists and the stratum's mode is not `selective` (`discard-only`, or `uninformative` by its record) is the ladder's FROB-REF-545 (§28.20, N28b-23), not 537: the division of BD-328, which owns the record (DESIGN, the plan states the two codes in two chapters and does not divide them) | Class, task family, rung pair and stratum; `observed` = the record's τ bound, `winner-lost` or `absent`; `required` = a record, with a τ bound of at least 0.5 to discard (0.7 to select) | 1. Run the rank-agreement experiment. 2. Use the rung as advisory (its scores are reported and decide nothing) | `after-change` | M3 (pilot record), M4 | N23a-24 (the rung-scoring entry points of `frob-learn`), BD-328 (the record and the verb); the ladder's allocation site |
| FROB-REF-544 | A training gate record without its negative controls | A gate record of a learning gate lacks any of: a zero-weight policy and a random policy that both fail the same threshold; a shuffled-label student where distillation is involved; a replay of the exported checkpoint that reproduces the behaviour (INV-CORTEX-05) | The gate id and the missing control | 1. Add the zero-weight, random and shuffled-label controls and the checkpoint replay | `after-change` | M2 | N23b-15 |
| FROB-REF-547 | A tensor view the receiver cannot represent, under a zero-copy contract | A `TensorView` crosses a boundary whose receiver cannot express its origin or strides (for example a negative stride into an unsigned-stride library) and the caller declared `zero-copy` | The view's extents, origin and strides; the receiver's convention | 1. Allow one charged copy (the copy is recorded on the receipt). 2. Change the layout | `after-change` | M2 | BD-193; the adapter's shared-device transport N23b-12 |
| FROB-REF-554 | A future-time requirement term offered as an online reward | A requirement whose temporal operator looks forward (`always`, `eventually`, `until` over a future window) is declared as a training reward or an online monitor; it is refused, never truncated: a 600 s window at 50 Hz is 30,000 samples | The term id and operator; `observed` = the future window; `required` = a bounded past window | 1. Use the past-time form online (`historically`, `once`, `since`). 2. Keep the future-time verdict offline on the completed trace | `after-change` | M2 | N23a-05 (task admission by the learner), BD-194, BD-255 |
| FROB-REF-830 | A `model-conditional` result offered for promotion, build or prototype budget | A ranking or result typed `model-conditional` by the rank-robustness check is offered to promotion to `established`, to a build packet, or to prototype spend | The result id and the variant that flips the ranking | 1. Resolve the variant that flips the ranking (the cheapest settling experiment is named in `next_actions`). 2. Report both rankings | `after-change` | M3 | N23b-08 (the check `check_spend_or_promote` and its call at promotion); N31ax-01 (the build-packet request of `foundry.export`, which is the prototype spend) |
| FROB-REF-867 | A D4 training result cited without re-evaluation on a stricter profile | A policy trained under the labelled nondeterministic training profile (class D4) is cited, scored into a ladder or attached to a gate by a `frob-learn` verb or record builder before a re-evaluation under a D2 or D3 profile exists (at every other citation point the same condition is the numerics chapter's 861; see the neighbouring codes) | The policy id; `observed` = `trained_under: D4`, re-evaluation absent | 1. Re-evaluate on D2 or D3 | `after-change` | M3 | N23a-18 (the citation check in `frob-learn`), BD-269 (the profile); consumer N23b-38 |

**Eight late refusals stated without a code (DESIGN: requests of the allocation ledger).** Each
is a request row of the allocation ledger `registries/refusal_allocations.toml` (schema `frob.refalloc/1`, owned
by N35-33: `request` slug, `requested_by`, `defined_in`, `condition`, `family = "5xx"`, `proposed`, `interim`,
`status`). Until a number is assigned, the raising bead raises the refusal behind one named constant
(`frob_learn::refusal::<SLUG>`), so assignment is a one-line change; the refusal is enforced from its first day.
All eight exit 3 (family `5xx`) and have `retry = after-change`. The proposals follow the ledger's rule (the
lowest number above Cortex's highest `5xx` code that is free: 555 to 557 belong to §28.20 and 560 to 569 to
§26.12); a proposal is a preference, and the ledger may assign another number. The fifth to eighth requests
carry no proposal: a bead never invents a number, and the ledger's rule assigns one. They are appended to the
ledger by this bead (`requested_by = "N23-REF"`; the ledger is append-only and later requests follow the same
rule).

| Request slug (proposed code) | Refused | Trigger | `pointer`, `observed`, `required` | Ranked repairs | First milestone | Raising bead |
|---|---|---|---|---|---|---|
| `sampler-unqualified` (FROB-REF-558) | Action sampler not qualified against its likelihood | `train.start` is asked to use a sampler that has no qualification receipt for the pinned kernel identity (rule P-12) | The sampler's kernel identity; `observed` = no receipt, or a receipt for another kernel digest; `required` = a passing joint-law receipt for this digest | 1. Run the joint-law battery for the kernel. 2. Select a qualified sampler | M2 | N23b-29 (raise), N23b-30 (receipt) |
| `actor-read-undeclared` (FROB-REF-559) | Actor read a field outside its information contract | A run whose policy consumed a reward, flag, mask or `info` field that the task does not declare as an `actor` channel is offered for a mission answer (rule E-10) | The run id and the first undeclared field with its access class and delivery instant; `required` = a declared `actor` channel | 1. Declare the field as an `actor` channel with its delivery instant (the task identity changes). 2. Remove the read | M2 (adapter sites at the adapter's release, M3) | N23b-31, N23b-32; the adapter BD-301 |
| `selection-below-containment` (FROB-REF-570) | Selection retains fewer designs than the containment rank | A rung admitted to selection is asked to advance fewer than k_c(m) designs, k_c(m) being the smallest k such that the higher rung's top m lay inside the lower rung's top k in the calibration and every audit draw | Class, task family, rung pair and m; `observed` = the requested retention k; `required` = k_c(m) | 1. Retain k_c(m) designs (the value is returned). 2. Discard by the fixed table without selecting | M3 | N23b-35 |
| `randomize-numerical-parameter` (FROB-REF-572) | Randomization requested on a numerical parameter | A task's `randomization` section names a parameter of class `numerical` (time step, substeps, solver tolerance or iterations, a contact law's penalty stiffness or regularization) at task admission; randomization touches physical parameters only, and numerical ones are varied by the exploitation detectors | The entry's path in the task; `observed` = the parameter and its class `numerical`; `required` = a parameter of class `physical` traced to a card | 1. Remove the entry and vary the parameter through `eval.detect` (its `dt-sensitive`, `tolerance-sensitive` and `stiffness-sensitive` detectors). 2. Where the quantity is a fitted physical property (a compliant pad's stiffness), declare it `physical` on its card with an interval | M2 | BD-200 (plan compilation at environment admission), N23b-03 (task admission) |
| `term-contract-unmet` (no proposal) | A task that lacks a declaration the term contract requires | The learner's admission pass over a `TaskProgram` (`admit`, before any world exists) reports one or more of its findings: `observation-unclassified`, `rate-not-rational`, `age-not-exposed`, `action-unbounded`, `action-law-missing`, `action-law-conflict`, `mapping-entry-missing`, `reward-kind-missing`, a `rate` term whose unit is not per second, `end-condition-unclassified`, `range-without-card`, `curriculum-touches-evaluation`, `curriculum-changes-atom`. One refusal carries all findings of the task. A future-time online reward keeps 554 and a missing settled-pose cache keeps the cache's 529 | The path of the first failing term in the task; `observed` = every finding with its term; `required` = the declaration each needs | 1. Add the missing declaration to the task file (the edit is named per finding; the `TaskId` changes). 2. For the two action-law findings only: name the law for the run, or omit the run's argument where the task already declares one | M2 | N23a-05 |
| `distilled-student-on-teacher-states` (no proposal; `defined_in = "§23.10"`) | A distilled student evaluated on teacher states | An evaluation of a distilled student is requested with any state source other than the student's own rollouts under its own observation schema (a teacher's visited states, or a teacher's observation schema). Refused at admission, before any episode runs; the evaluation environment is built from the student's manifest only | The state source in the request; `observed` = the teacher-state source with the teacher's identity; `required` = the student's own rollouts and manifest schema | 1. Evaluate the student on its own rollouts (drop the teacher-state source). `fewer_repairs_reason`: "a student is never scored on teacher states; there is no admitted alternative" | M4 | BD-323 (the evaluation step of `train.distill` and every evaluation of a distilled policy) |
| `evaluation-under-adaptive-law` (no proposal; `defined_in = "§23.17"`) | An evaluation drawn from the adaptive randomization law | `eval.run` (or any evaluation verb) is asked to draw its episodes under `--randomization adaptive`: evaluation episodes come from the fixed nominal law and the fixed scene splits, and no curriculum, adaptive randomization or search chooses them | `pointer` = `randomization`; `observed` = `adaptive`; `required` = `fixed` | 1. Evaluate under the fixed nominal law (omit the option or pass `fixed`). 2. Run the same request with `--randomization training-distribution` (N23b-04); its result is labelled `training-distribution` and is never an evaluation | M4 | N23b-04 |
| `tying-group-trivial` (no proposal; `defined_in = "§23.13"`) | A symmetry tying requested over a trivial group | `train.start --tying equivariant` for a design and task whose observation-preserving group Γ_obs (the stored record of BD-241) is trivial: there is nothing to tie (N23a-31); or `train.start --tying augmentation` where Γ_aug, the elements of Γ_obs under which the task's command distribution is also invariant in law, is trivial (N23a-32). Refused at admission, before any world exists, and on a dry run | `pointer` = `tying`; `observed` = the group the request ties under (`obs` for `equivariant`, `aug` for `augmentation`) with its order 1; `required` = a group of order at least 2 | 1. Train with `--tying none`. 2. Where a symmetry was expected, read which element breaks each excluded generator (`frob analyze symmetry <design> --task <task> --json`), remove that asymmetry in the design or the task (the `DesignId` or `TaskId` changes) and repeat the request. `--tying augmentation` is not a repair of a trivial Γ_obs: Γ_aug is a subgroup of Γ_obs, so that request is refused under this same slug | M4 | N23a-31 (`--tying equivariant`), N23a-32 (`--tying augmentation`) |

A further refusal of §23.15, randomization on a card field whose uncertainty is `exact` or absent
(`randomize-no-interval`, proposed FROB-REF-571), is decided in its own bead, N23bx-03, which depends on this one;
the two never merge, because their repairs differ (572 is about the parameter's class, 571 about a physical
field's missing interval).

**Neighbouring codes and which one fires (DESIGN).** One condition must map
to one code. 524 against 442 (sensing: an observation requested from presentation state, or an actor schema
names a privileged channel): the first clause of 442 fires in the observation service and never meets 524. For
the second clause the division is by raising site. They are divided by raising site, so that one event has one code. `FROB-REF-524` is the learner's: `frob-learn` raises it when one of its components asks to read, as the actor, a channel that the task classifies `critic-only` or `evaluation-only` (a network specification; a checkpoint's schema checked against the current task; a host caller's request for an actor view). That check runs before any schema is frozen, so such a request never reaches the freeze. `FROB-REF-442`, second clause, is the freeze's: `freeze_actor_schema` in `frob-sense` raises it, for whatever caller, when a channel offered for an actor schema is privileged by source (the truth namespace; a belief or measurement channel whose producer had privileged inputs; a task or curriculum parameter; a reward or termination signal) or is not of actor access. A learner's request that passed the class check and is still privileged by source (a task that classifies a truth channel as `actor`) is therefore 442. The same text stands in N20a-03, N23a-03 and the learning registry N23-REF. 554 (a future-time term as an online reward, at task admission by the learner) against 523 (a future-time
operator in any online signal, at requirement compilation): 523 fires first where the compiler sees it; 554 is the
learner's check on a compiled task. 534 (outside class or pretraining distribution, by the policy's membership
rule) against 541 (outside the generator's support, by the generator's derivation test): both may apply; the
policy raises 534 and cites the 541 finding when the generator reports it. 830 (promotion, build or prototype
budget) against 820 (a non-established label cited as established) and 557 (a model-conditional result offered
as a design comparison without its condition): 830 at the three spend-and-promote sites, 557 at comparison
reports, 820 at citation. 867 (a D4-trained policy cited without re-evaluation) against 861 (the numerics
chapter's code: a number from a fast-training run cited as evidence, or a policy trained under it that lacks a
stricter re-evaluation) and 825 (the safety chapter's code: a result of class D4 offered as evidence): the
consumer decides, one code per raising site. A D4-trained policy without a re-evaluation is 867 where a
`frob-learn` verb or record builder is the consumer (`train.export` summaries, the training-gate record
builder, a ladder score) and 861 at every other citation point. A number observed during the run (a learning
curve, a training return) is 861 wherever it is cited; the plan's known answers KA-801 and KA-802 name 861 for
a cited training return. 825 is the evidence boundary's code for a stored result of class D4 or unrecorded
offered to promotion, qualification, a certificate, a build packet or paired statistics computed from stored
results. The plan defines the three codes in three chapters and does not divide them; the numerics registry
N35-REF (owner of 861) and the safety registry N37-REF (owner of 825) state the same division. 547 (a tensor view the receiver
cannot represent under a zero-copy contract, raised in `frob-learn` and at the adapter) against 617 (the code the
device chapter's kernel-manifest text gives for the same rule): 547 is raised by the learner and the adapter,
617 by the device layer; BD-193 raises 547 and names 617 as the device-layer code. 537 (a rung asked to select or
discard with no record, or to discard where the record makes the stratum `uninformative`) against 545 (the
ladder's code: a rung asked to select in a stratum whose record exists and whose mode is not `selective`): 537
when the record is absent or forbids the discard, 545 when a record exists and only selection is forbidden; the
division is BD-328's, and the ladder registry N28-REF carries the same sentence on its 545 row. One site lies
outside the division: `opt.start --method mf-bo` answers 545 for a fidelity-set rung that is not `selective`,
an unmeasured stratum included (N28b-07; N28-REF's 545 row gives the reason).

**Registry row format.** One TOML table per code, with the fields the registry-wide checks of N35-33 require:
`code`, `name`, `family`, `message_template`, `exit = 3`, `retry`, `repairs` (ordered; at least two, or
`fewer_repairs_reason`, which 522 carries: "the environment does not sanitize actions", and among the late
requests `distilled-student-on-teacher-states`), `defined_in` (the plan
subsection that states the refusal: `§23.19` for the twelve, `§23.25`, `§23.26`, `§23.28`, `§23.15`, `§23.5`,
`§23.10`, `§23.17` and `§23.13` for the late eight), `raised_by` (the bead keys of the table's last column), `related` (the neighbouring codes below, each
with the one sentence that says which raising site uses which code; the pair must be symmetric with the
neighbour's row), and `first_milestone`. A row may exist before its mechanism; the verb that would raise it then
answers the capability refusal (not implemented in this build), never `ok`.

## Acceptance criteria

- [ ] `registries/refusals.toml` holds the twelve rows, schema-valid, with ordered repairs and exit 3 each.
- [ ] `cargo test -p frob-core` loads the registry; a duplicated code, an unknown code and a row with two exit codes are load-time errors.
- [ ] The eight late refusals are request rows of `registries/refusal_allocations.toml` and are registered under the codes the ledger assigns (or the proposals, if confirmed); until then each is raised behind its named constant, and no bead raises them as an untyped error.
- [ ] Every row carries `defined_in` (a plan subsection), `raised_by`, `retry`, `related` and at least two ranked repairs or its `fewer_repairs_reason`; N35-33's cross-chapter checks pass on the Cortex rows.
- [ ] Each of the twenty codes has a raising test in its raising bead asserting code, `pointer`, `observed`, `required`, `retry` and repair order.
- [ ] `CONTRACT.md` of `frob-learn` lists the codes in its refusals section with the same wording; `frob schema` and the served MCP tool list print the same refusal lists per verb.
- [ ] The end-to-end script provokes every code whose mechanism is present and gets exit 3 and the registered code; an absent mechanism is logged `skipped: capability absent`.

## Test plan

### Unit tests

`cargo test -p frob-core refusals_cortex`:
- `rows_present`: the twelve codes with names; ten in family `5xx`, two in `8xx`.
- `exit_codes`: all twelve map to exit 3; none maps to two exits.
- `repairs_ordered`: first repair of 521 is "reset the world"; first of 537 is "run the rank-agreement experiment"; first of 547 is "allow one charged copy".
- `no_sanitizing_repair_for_522`: the row has exactly one repair.
- `late_codes_unique`: the eight late requests do not collide with any registered code, with each other, with N23bx-03's 571 or with `retired`; the four requests without a proposal have ledger rows with `proposed` absent and are raised behind their constants.
- `row_fields_complete`: every Cortex row has `defined_in` matching `^§23\.[0-9]+$`, a non-empty `raised_by`, one `retry`, and two repairs or a `fewer_repairs_reason` (only 522 among the twelve, and `distilled-student-on-teacher-states` among the late eight); a planted row with `defined_in = "cortex"` fails.
- `neighbours_symmetric`: 524↔442, 554↔523, 534↔541, 830↔820, 830↔557, 867↔861, 867↔825, 547↔617, 537↔545 each appear in both rows' `related`, with two different raising sites.
- `division_537_545` (with BD-328 and N28b-23, M4): a select request in a stratum with no record → 537 with `observed = absent`; a select request in a stratum whose record has a τ bound between 0.5 and 0.7 (`discard-only`) → 545, not 537; a discard request in a stratum whose record has a τ bound below 0.5 → 537.
- `row_before_mechanism`: with the family-policy capability absent (before M4, or after its kill gate has fired), `policy.apply` answers the capability refusal, exit 5, and not 534.

### End-to-end script and logging

`scripts/e2e/cortex_refusals.sh` sources the helper of `STD-E2E` and logs newline-delimited JSON to
`target/e2e-logs/cortex_refusals/<utc>.ndjson` with a transcript. Each step is a `cmd` event (exact command, exit
code, wall time, the envelope's `evidence` block) followed by `assert` events on code, pointer and repair order.
Every call of a run-kind verb carries `--key`. A step whose verb or option the build does not serve yet (several
rows are first raised at M3 or M4) is logged `skipped: capability absent` with the missing verb or option named,
never as passed:

1. 521 has no raising site in a verb (no run loop of the product steps an ended world; the raising sites are host callers of `VecEnv::step` and, from M3, the adapter). The step runs the Gauntlet entry that holds its fixture, `FROB_TEST_SEED=<seed> cargo test -p frob-conformance --test g1_known_answers -- GATE-CORTEX-06`, and asserts from the receipt that the entry of rule E-1 observed 521 with the world ids in `pointer` and an unchanged state digest.
2. 522, from the same receipt: the entry of rule E-5 observed 522 naming world and channel, and the digest of every world before and after is equal. (From a verb the code appears only when a learned policy emits a non-finite action during a job; no fixture policy can be injected into `sim.run` on a standard build.)
3. `frob train start quad-ref --task quad-flat-velocity --actor-channels +friction_true --dry-run --key ref-524 --json` (an actor specification that names a `critic-only` channel, N23a-03) → 524.
4. `frob train start quad-ref --task quad-flat-velocity --resume @fixture:checkpoint-clipped-law --action-law latent-then-limit --key ref-527 --json` → 527.
5. `frob eval run @fixture:hexapod --policy @fixture:quad-specialist --battery smoke --key ref-531 --json` (a policy whose mapping table does not match the design; the form N23a-13 uses for role-matched loading, with the learned policy named as the candidate's controller) → 531.
6. `frob policy apply @fixture:quad-family --design @fixture:quad-out-of-box --key ref-534 --json` → 534.
7. `frob opt cascade --spec @fixture:unmeasured-stratum-task --population @fixture:unmeasured-stratum-set --key ref-537 --json` (a rung asked to discard in a stratum that has no rank-agreement record) → 537.
8. A training gate record without its controls offered to the record builder of N23b-15 (a library call, made through the raising test of N23b-15, since the record builder has no verb of its own) → 544.
9. A reversed view under the zero-copy contract: `frob train start quad-ref --task @fixture:reversed-channel --trainer frankentorch --zero-copy --key ref-547 --json` (BD-193) → 547.
10. `frob task check @fixture:future-time-reward --json` and `frob train start quad-ref --task @fixture:future-time-reward --dry-run --key ref-554 --json` → 554 at both.
11. `frob eval promote @fixture:model-conditional --key ref-830 --json` → 830.
12. A D4-trained policy offered to `frob eval run` as the cited policy of a ladder score → 867.
13. to 20. The eight late refusals. `sampler-unqualified` (558): the plant of N33c-37 cannot be selected from the `frob` binary, so the step reads the receipt of `FROB_TEST_SEED=<seed> cargo test -p frob-conformance --features gauntlet-mutations --test g1_known_answers -- GATE-CORTEX-18`, in which the admission check of N23b-29 refuses the plant's identity; `frob mission synthesize @fixture:dock-cites-undeclared-read --key ref-559 --json`, a query citing a run whose policy read the delivered reward (559); `frob opt cascade --spec @fixture:forty-designs-task --population @fixture:forty-designs --key ref-570 --json` with the retention of two designs at the selecting rung that N23b-35's script requests, on the forty-design fixture whose containment rank is 3 (570); `frob train start quad-ref --task @fixture:randomize-time-step --dry-run --key ref-572 --json` (572); `frob train start quad-ref --task @fixture:observation-unclassified --dry-run --key ref-tc --json` (`term-contract-unmet`, with the finding in `observed`); `distilled-student-on-teacher-states`: no verb option supplies teacher states (the evaluation environment is built from the student's manifest), so the step reads the receipt of BD-323's raising test, which offers teacher states to the evaluation builder and asserts the refusal before any episode; `frob eval run quad-ref --policy <adaptive policy> --task quad-flat-velocity --split held-out --randomization adaptive --key ref-adapt --json` (`evaluation-under-adaptive-law`, the form of N23b-04's script; the second repair is asserted to name `--randomization training-distribution`); `frob train start @fixture:trivial-gamma-obs --task quad-flat-velocity --tying equivariant --dry-run --key ref-tie --json` (`tying-group-trivial`: the robot-task fixture with trivial Γ_obs of N23a-31's script; `pointer` = `tying`, `observed` the group `obs` of order 1, first repair `--tying none`).

The `summary` line gives assertions passed and failed, steps skipped with the reason and `gates: []`.

### Known answers and negative controls

- Known values used by the fixtures: the stride case of step 9 is byte origin 4 with strides (8, −4) on a 2 × 2 `f32` view; step 2 asserts equal state digests (no state changed); step 7 uses a stratum with no record; step 15 uses the forty-design fixture with k = 2 (containment rank 3).
- Negative controls: a planted raising site that returns `ok: true` with a warning in place of 522 must fail step 2; a planted registry row without repairs must fail the registry lint; a planted second exit code must fail `exit_codes`; a planted build in which the action check runs after the limiter (so a NaN reaches state) must fail the digest assertion of step 2.
- Counterexamples behind the codes: CX-249 (527), CX-255 (547), CX-256 and KA-593 (554), CX-257 (544), CX-258 (830).
- Determinism: the script runs twice on one machine; the two logs agree on every asserted field.

## Considerations

- The overlaps listed above are real: three codes speak about `model-conditional` results and three about D4. The one-condition-one-code rule is this bead's reading, and for the D4 codes it is the same reading as in N35-REF and N37-REF (a change to one is a change to all three); if the registry owner merges codes, the raising beads change one constant each.
- 534 and 537 are registered at M2 and first raised at M4 and M3. One event has one code: 534 is the membership refusal of a family policy that exists (a design outside its class or its pretraining distribution). Where no family policy exists (before M4, or because the compute envelope was not approved, or because KILL-FAMILY fired) `policy.apply` answers the capability refusal, exit 5, naming the decision in force and the fallback rungs, as N23a-25 and the decision bead N23a-26 state. After such a kill 534 stays in the registry, since a code is never reused, and has no raising site.
- Repair 2 of 527 is conditional; the envelope omits it when the stored variable cannot be relabelled, and never prints unavailable advice.

## References

§23.19; Appendix O sections O.5 and O.8; §32.4 (exit codes); §35.11; §23.5, §23.10, §23.13, §23.15, §23.17, §23.25, §23.26, §23.28 (late refusals); §28.20 (545); CX-249, CX-255, CX-256, CX-257, CX-258.
