---
key: "N23-REF"
title: "Cortex refusal registry: twelve codes (521 to 554, 830, 867) and three late refusals"
type: "task"
priority: 1
milestone: "M2"
labels: ["m2", "cortex", "frob-learn", "frob-core", "solid", "registry", "test", "e2e"]
parent: "WP-48"
deps: ["BD-005", "BD-022", "STD-E2E"]
needs: ["§32.4 and Appendix O: assignment by the refusal-registry owner of three new codes for the late refusals of §23.25, §23.26 and §23.28 (candidates 558, 559 and 570, all unused); until assigned, the raising beads use the candidates behind one constant each", "§23.3, §23.4, §23.6, §23.7: the raising sites of 521, 522, 524 (BD-192), 547 (BD-193), 527 (BD-197) and 531 (BD-198, BD-322), owned by the unit that drafts §23.1 to §23.13", "§35.2: the raising site of 867 at the labelled D4 training profile (BD-269)", "§16.8 and §31: the promotion and build-packet sites that must raise 830 (BD-263 `eval.promote`, BD-408 `foundry.export`)"]
related: ["BD-192", "BD-193", "BD-194", "BD-197", "BD-198", "BD-255", "BD-263", "BD-269", "BD-320", "BD-322", "BD-328", "N23b-08", "N23b-12", "N23b-15", "N23b-29", "N23b-31", "N23b-35", "N23-CX", "N23a-03", "N23a-05", "N23a-08", "N23a-13", "N23a-18", "N23a-24", "N23a-25", "N23a-27"]
gates: []
plan_refs: ["§23.19", "Appendix O (O.5, O.8)", "§32.4", "§35.11", "§23.25", "§23.26", "§23.28"]
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
two codes of the evidence family. This bead is the registry of the twelve codes Cortex defines, plus three
refusals that later sections state in words without a code. It fixes the content and the exit code of each row
and owns the test that each code is raised by its trigger. The mechanisms that raise them are other beads, named
per row, so that no code is left without a raising site.

## Scope

In: the twelve registry rows with meaning, trigger, envelope fields, ranked repairs, retry class and exit code;
the three late refusals with candidate codes; the disambiguation against seven neighbouring codes of other
chapters; the registry tests; one end-to-end script that provokes each code from the command line.
Out: the detection logic of each condition (the raising beads).

## Technical approach

**Envelope shape (FACT about the surface contract, schema `frob.envelope/1`).** When `ok` is false the envelope
carries `refusal = { code, message, pointer, observed, required, unit, affected_claims, retry, repairs[] }`;
`retry` is `never`, `after-change` or `backoff`; each repair is a typed suggestion (a verb with arguments or a
typed design edit) with a one-line reason. Exit 3 means refused input or question and covers codes `1xx` to `5xx`
and `8xx` other than conflicts; exit 1 is never emitted on purpose. All fifteen rows below exit 3.

**The twelve rows.**

| Code | Refused | Raised when (trigger) | `pointer`, `observed`, `required` | Ranked repairs | `retry` | First milestone | Raising bead |
|---|---|---|---|---|---|---|---|
| FROB-REF-521 | `step` on a world that needs reset, in explicit mode | `VecEnv::step` receives an action for a world whose `needs_reset` (terminated, truncated or invalid) is set and the run's reset mode is `explicit` (environment rule E-1). Checked before any state changes | The world's logical id and the flag that ended it; `observed` = `needs_reset: true`; `required` = a reset of that world | 1. Reset the world (`reset(mask)`). 2. Run in the labelled mode `autoreset: masked`, where the world is reset inside the step and the final observation travels in `final_obs` | `after-change` | M2 | BD-192 |
| FROB-REF-522 | A non-finite action, before any state changes | Any element of the action tensor is NaN or infinite (rule E-5); the check runs before the limiter and before any world is mutated, so the whole batch is unchanged | World id and channel id of the first offending element; `observed` = the value's class (`nan`, `+inf`, `-inf`); `required` = finite | 1. Fix the policy output (the world and channel are named). No second repair: the environment does not sanitize | `after-change` | M2 | BD-192 |
| FROB-REF-524 | The actor schema reads a `critic-only` or `evaluation-only` channel | A frozen actor schema names a channel whose access class is not `actor`, or an inference run requests such a channel (rule E-8); the actor is always run with the other channels physically absent | The channel id and its access class; `required` = `actor` | 1. Move the channel to the critic (asymmetric actor-critic). 2. Declare a privileged teacher and distill a student whose schema omits the channel | `after-change` | M2 | N23a-03 (raise; integrated in BD-192); extended to rewards, flags, masks and `info` by N23b-31 |
| FROB-REF-527 | The action law at learning differs from the law at collection | The registered action law of a rollout (`latent-then-limit` or `clipped-with-endpoint-mass`) differs from the law the update evaluates, or a batch mixes the two (rule P-3, INV-CORTEX-03) | The rollout id; `observed` = the law at collection; `required` = the law at learning | 1. Recollect under the learner's law. 2. Relabel the stored samples under the registered law (possible only where the stored variable allows it: a stored latent can be relabelled, a stored clipped action cannot recover its latent) | `after-change` | M2 | N23a-08 (raise), BD-197 (battery), N23a-14 (a resume under another law) |
| FROB-REF-531 | A policy's mapping table does not match the design, or cannot be built for an edit | Loading or applying a checkpoint whose `frob.policy/1` channels (roles, dimensions, signs, units, rates) do not match the target design by role; or an inheritance request whose typed edit declares no usable correspondence (INV-CORTEX-04) | The first unmatched role with the differing attribute; `observed` and `required` = the two attribute values with units | 1. Supply the correspondence in the typed edit. 2. Train fresh | `after-change` | M2 (load), M4 (edits) | N23a-13 (role-matched loading), N23a-27 (an inheritance map that cannot be built), BD-320 (the family mapping table), N23a-25 (binding) |
| FROB-REF-534 | A family policy applied outside its class or its pretraining distribution | `policy.apply` or a family-policy score is requested for a design that fails the membership rule: outside the parameter box of the pretraining distribution, or a topology signature outside the class | The design id; the failing parameter with its value and the box, or the topology signature and the class | 1. Screen with the baseline controller or the sampling controller. 2. Train a specialist | `after-change` | M4 | BD-320 (membership at every scoring path), N23a-25 (`policy.apply`) |
| FROB-REF-537 | A rung used to select or discard without a rank-agreement record that meets its threshold | A ladder allocation asks a rung to select or discard in a stratum that has no rank-agreement record, or whose record's lower 95% bound on top-band Kendall τ is below 0.5 (discard) or below 0.7 (select) (INV-CORTEX-06) | Class, task family, rung pair and stratum; `observed` = the record's τ bound or `absent`; `required` = 0.5 or 0.7 | 1. Run the rank-agreement experiment. 2. Use the rung as advisory (its scores are reported and decide nothing) | `after-change` | M3 (pilot record), M4 | N23a-24 (the rung-scoring entry points of `frob-learn`), BD-328 (the record and the verb); the ladder's allocation site |
| FROB-REF-544 | A training gate record without its negative controls | A gate record of a learning gate lacks any of: a zero-weight policy and a random policy that both fail the same threshold; a shuffled-label student where distillation is involved; a replay of the exported checkpoint that reproduces the behaviour (INV-CORTEX-05) | The gate id and the missing control | 1. Add the zero-weight, random and shuffled-label controls and the checkpoint replay | `after-change` | M2 | N23b-15 |
| FROB-REF-547 | A tensor view the receiver cannot represent, under a zero-copy contract | A `TensorView` crosses a boundary whose receiver cannot express its origin or strides (for example a negative stride into an unsigned-stride library) and the caller declared `zero-copy` | The view's extents, origin and strides; the receiver's convention | 1. Allow one charged copy (the copy is recorded on the receipt). 2. Change the layout | `after-change` | M2 | BD-193; the adapter's shared-device transport N23b-12 |
| FROB-REF-554 | A future-time requirement term offered as an online reward | A requirement whose temporal operator looks forward (`always`, `eventually`, `until` over a future window) is declared as a training reward or an online monitor; it is refused, never truncated: a 600 s window at 50 Hz is 30,000 samples | The term id and operator; `observed` = the future window; `required` = a bounded past window | 1. Use the past-time form online (`historically`, `once`, `since`). 2. Keep the future-time verdict offline on the completed trace | `after-change` | M2 | N23a-05 (task admission by the learner), BD-194, BD-255 |
| FROB-REF-830 | A `model-conditional` result offered for promotion, build or prototype budget | A ranking or result typed `model-conditional` by the rank-robustness check is offered to promotion to `established`, to a build packet, or to prototype spend | The result id and the variant that flips the ranking | 1. Resolve the variant that flips the ranking (the cheapest settling experiment is named in `next_actions`). 2. Report both rankings | `after-change` | M3 | N23b-08; promotion and build-packet sites in `needs` |
| FROB-REF-867 | A D4 training result cited without re-evaluation on a stricter profile | A policy trained under the labelled nondeterministic training profile (class D4) is cited, scored into a ladder or attached to a gate before a re-evaluation under a D2 or D3 profile exists | The policy id; `observed` = `trained_under: D4`, re-evaluation absent | 1. Re-evaluate on D2 or D3 | `after-change` | M3 | N23a-18 (the citation check in `frob-learn`), BD-269 (the profile); consumer N23b-38 |

**Three late refusals stated without a code (DESIGN: candidates, flagged for the registry owner).**

| Candidate | Refused | Trigger | Ranked repairs | Raising bead |
|---|---|---|---|---|
| FROB-REF-558 | Action sampler not qualified against its likelihood | `train.start` is asked to use a sampler that has no qualification receipt for the pinned kernel identity | 1. Run the joint-law battery for the kernel. 2. Select a qualified sampler | N23b-29 (raise), N23b-30 (receipt) |
| FROB-REF-559 | Actor read a field outside its information contract | A run whose policy consumed a reward, flag, mask or `info` field that the task does not declare as an `actor` channel is offered for a mission answer | 1. Declare the field as an `actor` channel with its delivery instant (the task identity changes). 2. Remove the read | N23b-31, N23b-32; the adapter BD-301 |
| FROB-REF-570 | Selection retains fewer designs than the containment rank | A rung admitted to selection is asked to advance fewer than k_c(m) designs, k_c(m) being the smallest k such that the higher rung's top m lay inside the lower rung's top k in the calibration and every audit draw | 1. Retain k_c(m) designs (the value is returned). 2. Discard by the fixed table without selecting | N23b-35 |

**Neighbouring codes and which one fires (DESIGN, to be confirmed by the registry owner).** One condition must map
to one code. 524 (Cortex: the actor schema reads a non-actor channel, at schema freeze or inference) against 442
(sensing: an observation requested from presentation state): 442 fires in the observation service, 524 in the
learner. 554 (a future-time term as an online reward, at task admission by the learner) against 523 (a future-time
operator in any online signal, at requirement compilation): 523 fires first where the compiler sees it; 554 is the
learner's check on a compiled task. 534 (outside class or pretraining distribution, by the policy's membership
rule) against 541 (outside the generator's support, by the generator's derivation test): both may apply; the
policy raises 534 and cites the 541 finding when the generator reports it. 830 (promotion, build or prototype
budget) against 820 (a non-established label cited as established) and 557 (a model-conditional result offered
as a design comparison without its condition): 830 at the three spend-and-promote sites, 557 at comparison
reports, 820 at citation. 867 (a D4-trained policy cited without re-evaluation) against 861 (a number observed
during a fast-training run cited as evidence) and 825 (a D4 result offered as evidence at the boundary): 867 is
about the policy, 861 about run-time numbers, 825 about the export boundary. 547 (a tensor view the receiver
cannot represent under a zero-copy contract, raised in `frob-learn` and at the adapter) against 617 (the code the
device chapter's kernel-manifest text gives for the same rule): 547 is raised by the learner and the adapter,
617 by the device layer; BD-193 raises 547 and names 617 as the device-layer code.

**Registry row format.** One TOML table per code: `code`, `name`, `family`, `message_template`, `exit = 3`,
`retry`, `repairs` (ordered), `defined_in = "cortex"`, `first_milestone`. A row may exist before its mechanism;
the verb that would raise it then answers the capability refusal (not implemented in this build), never `ok`.

## Acceptance criteria

- [ ] `registries/refusals.toml` holds the twelve rows, schema-valid, with ordered repairs and exit 3 each.
- [ ] `cargo test -p frob-core` loads the registry; a duplicated code, an unknown code and a row with two exit codes are load-time errors.
- [ ] The three late refusals are registered under codes assigned by the registry owner (or the candidates, if confirmed), and no bead raises them as an untyped error.
- [ ] Each of the fifteen codes has a raising test in its raising bead asserting code, `pointer`, `observed`, `required`, `retry` and repair order.
- [ ] `CONTRACT.md` of `frob-learn` lists the codes in its refusals section with the same wording; `frob schema` and the served MCP tool list print the same refusal lists per verb.
- [ ] The end-to-end script provokes every code whose mechanism is present and gets exit 3 and the registered code; an absent mechanism is logged `skipped: capability absent`.

## Test plan

### Unit tests

`cargo test -p frob-core refusals_cortex`:
- `rows_present`: the twelve codes with names; ten in family `5xx`, two in `8xx`.
- `exit_codes`: all twelve map to exit 3; none maps to two exits.
- `repairs_ordered`: first repair of 521 is "reset the world"; first of 537 is "run the rank-agreement experiment"; first of 547 is "allow one charged copy".
- `no_sanitizing_repair_for_522`: the row has exactly one repair.
- `late_codes_unique`: the three late codes do not collide with any registered code.
- `row_before_mechanism`: with the family-policy capability absent, `policy.apply` answers the capability refusal, exit 5, and not 534.

### End-to-end script and logging

`scripts/e2e/cortex_refusals.sh` sources the helper of `STD-E2E` and logs newline-delimited JSON to
`target/e2e-logs/cortex_refusals/<utc>.ndjson` with a transcript. Each step is a `cmd` event (exact command, exit
code, wall time, the envelope's `evidence` block) followed by `assert` events on code, pointer and repair order:

1. `frob sim run --task @fixture:two-world-ended --policy @fixture:steps-ended-world --json` → 521.
2. `frob sim run --task quad-flat-velocity --policy @fixture:nan-action --json` → 522; a digest of every world before and after is equal.
3. `frob train start --task @fixture:actor-reads-critic-channel --json` → 524.
4. `frob train start --task quad-flat-velocity --resume @fixture:rollout-clipped-law --action-law latent-then-limit --json` → 527.
5. `frob sim run --design @fixture:hexapod --policy @fixture:quad-specialist --json` → 531.
6. `frob policy apply --policy @fixture:quad-family --design @fixture:quad-out-of-box --json` → 534.
7. `frob opt cascade --campaign @fixture:unmeasured-stratum --json` → 537.
8. A gate record without controls offered to the gate recorder → 544.
9. A negative-stride view under `--zero-copy` → 547.
10. `frob train start --task @fixture:future-time-reward --json` → 554.
11. `frob eval promote --result @fixture:model-conditional --json` → 830.
12. A D4-trained policy offered to `frob eval run` as the cited policy of a ladder score → 867.
13. to 15. The three late refusals from their fixtures.

The `summary` line gives assertions passed and failed, steps skipped with the reason and `gates: []`.

### Known answers and negative controls

- Known values used by the fixtures: the stride case of step 9 is byte origin 4 with strides (8, −4) on a 2 × 2 `f32` view; step 2 asserts equal state digests (no state changed); step 7 uses a stratum with no record; step 15 uses the forty-design fixture with k = 2 (containment rank 3).
- Negative controls: a planted raising site that returns `ok: true` with a warning in place of 522 must fail step 2; a planted registry row without repairs must fail the registry lint; a planted second exit code must fail `exit_codes`; a planted build in which the action check runs after the limiter (so a NaN reaches state) must fail the digest assertion of step 2.
- Counterexamples behind the codes: CX-249 (527), CX-255 (547), CX-256 and KA-593 (554), CX-257 (544), CX-258 (830).
- Determinism: the script runs twice on one machine; the two logs agree on every asserted field.

## Considerations

- The overlaps listed above are real: three codes speak about `model-conditional` results and three about D4. The one-condition-one-code rule is this bead's reading; if the registry owner merges codes, the raising beads change one constant each.
- 534 and 537 are registered at M2 and first raised at M4 and M3. If the family-policy work falls back at its kill gate, 534 stays registered and is raised by every `policy.apply` (there is then no family policy to apply).
- Repair 2 of 527 is conditional; the envelope omits it when the stored variable cannot be relabelled, and never prints unavailable advice.

## References

§23.19; Appendix O sections O.5 and O.8; §32.4 (exit codes); §35.11; §23.25, §23.26, §23.28 (late refusals); CX-249, CX-255, CX-256, CX-257, CX-258.
