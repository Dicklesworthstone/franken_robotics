---
key: "VERBS-train"
title: "frob-verbs registry: cluster `train`, seven rows, with parity, e2e and schema-snapshot checks"
type: "task"
priority: 1
milestone: "M2"
labels: ["m2", "tether", "cortex", "frob-verbs", "solid", "registry", "verb", "test", "review-notes-open"]
parent: "WP-43"
deps: ["BD-022", "N14-REF", "N17-REF", "N20-REF", "N21-REF", "N22-REF", "N23-REF", "N24-REF", "N32-REF", "N35-REF", "N37-REF", "NJL-01", "NJL-02", "NJL-04", "NJL-05", "STD-E2E"]
needs: []
related: []
gates: ["GATE-TETHER-02"]
plan_refs: ["Appendix J (J.1, J.2, J.3, J.5)", "§23.6", "§23.7", "§23.10", "§23.11", "§23.12", "§23.13", "§32.6", "§32.12.3", "§32.21 (GATE-TETHER-02)"]
---
## Background

FrankenRobotics (a robotics simulation and design system) is reached through the command line `frob`, the daemon
`frobd` and the MCP server `frob-mcp` (the Model Context Protocol, by which an agent harness lists and calls tools).
Every capability is a verb, `cluster.verb`, declared once in `registries/verbs.toml` (crate `frob-verbs`, tier
Solid). The CLI, the daemon protocol, the MCP tools, the schemas, the refusal table and the agent guide are
generated from that file, and gate GATE-TETHER-02 compares the served surfaces with it. The registry holds 244 verbs
in 44 clusters (namespaces): the 243 of the specification and `artifact.import` (VERBS-artifact); the
count of record is NJL-03's.

This bead is the registry of record for the cluster `train`. The cluster covers training: starting, watching and
stopping a training run, exporting a policy with its manifest and mapping table, pretraining a family policy,
inheriting a policy along a design edit, and distillation. It has seven verbs (2 `read`, 4 `run`, 1 `admin`); the
handlers live in `frob-learn`. The bead carries every row completely, with the three checks that keep the rows true:
the generated CLI commands and MCP tools match the list; every verb has an end-to-end script; the schema printed by
`frob <verb> --help --json` is stable.

Kinds: `read` returns stored or cheaply computed facts and changes nothing; `run` spends compute, takes an
idempotency key and a budget, and after a bounded synchronous wait returns a job handle with exit code 8; `mutate`
changes a named object in the caller's namespace and takes an idempotency key and an expected parent; `admin`
cancels, stops, evicts or deletes and needs a capability scope. Tiers: S Solid, F Frontier (with a pilot and a kill
gate), M Moonshot; `S/F` is a Solid core with Frontier extensions.

DESIGN: a registry bead per cluster exists beside the implementing beads because a row missing from the registry,
spelled twice, or served with different refusal lists on the CLI and over MCP is a surface defect that no mechanism
test catches. Rejected: rows left to the implementing beads alone (nothing checks their union against the
inventory); one bead for all the rows (too large to review).

## Scope

In: the seven registry entries of cluster `train`; the MCP tool names; the cluster's conformance script; the
help-schema snapshots; each verb's answer before its milestone. Out: behaviour (the implementing beads named below);
the entry schema and lints (NJL-01); the conformance harness (NJL-02); the alias lint (NJL-04); task clusters
(NJL-05, NJL-06); generation of the surfaces (the Tether seed beads).

## Technical approach

**The rows.**

| Verb | Kind | First milestone | Tier | Purpose | Defined in |
|---|---|---|---|---|---|
| `train.distill` | run | M4 | F | Distill a teacher into a student policy | §23.10 |
| `train.export` | read | M2 | S | Export a policy with its manifest and mapping table | §23.7 |
| `train.family` | run | M4 | F | Pretrain a family policy for a morphology class, after the envelope check | §23.11 |
| `train.inherit` | run | M4 | F | Start a child's policy from a donor through the typed edit that joins them | §23.12 |
| `train.start` | run | M2 | S | Start training; algorithm, network, devices, tying and profile are arguments | §23.6 |
| `train.status` | read | M2 | S | Return, steps, learner share and flags of a training run | §23.6 |
| `train.stop` | admin | M2 | S | Stop a training run at a commit boundary | §23.6 |

**Contract of each verb.** Evidence labels used below (the `evidence` block of the envelope):
- *screening*: class Estimated: a screening or planning signal that may discard under the ladder's rules and may not
  establish a comparison.
- *pass-through*: passes through the claim product of the stored objects it reads and adds no claim of its own; the
  read itself is `exact`.
- *run*: class Tested at best, by the claim product of the run; the tier of the run, `provisional` on a fast tier
  until promoted to the referee; the determinism class of the placement (D2 unless stated); `reality: no-claim`.
- *operational*: no claim: an operational answer about the session, the store, a job or a device; `evidence.claims`
  is empty.

**`train.distill`** (MCP tool `train_distill`; owner `frob-learn`)
- Inputs (BD-323): `teacher` (one or more identities, each of kind `baseline`, `sampling`, `specialist` or
  `privileged`); `student` (the student network, as `train.start`'s `--network`); `task`; `design` (or a design
  set, for pooled distillation into one student); the mixing schedule; the dataset capacity; budget; seed; the
  determinism profile (`--determinism`, see the notes); idempotency key; `--control shuffled-labels` (the
  shuffled-label control run, labelled as such and never exported as a policy); `--relabel-budget <GPU-seconds>`
  (the sampling teacher's relabelling budget); `--dry-run` (trains nothing and spends no device time: resolves
  the teachers, applies the admission checks, including the restriction a fired KILL-DISTILL imposes, binds the
  channels by role and reports the plan).
- Outputs: the job handle; on completion the student's `PolicyId` with its distillation record (the manifest's
  teacher list and standing), the gap report, the control results. Evidence: *screening*.
- Chapter refusals: `FROB-REF-524` (a student schema that includes a privileged channel); `FROB-REF-531` (a teacher
  whose action channels cannot be bound to the design by role). A sampling teacher whose relabelling budget exceeds
  the session's is the appended budget refusal. `FROB-REF-830` is not on this row: its raising sites are promotion
  (N23b-08) and `foundry.export` (N31ax-01), N23-REF's row, and BD-323 raises it nowhere (a distilled policy takes
  the weakest standing among its teachers, §23.10). Worded (N23-REF's request, raised through its constant, and
  entered in the row's `refusals` under the number the allocation ledger assigns, as for the worded refusals of
  `train.start`): `distilled-student-on-teacher-states` (the student evaluated with any state source other than its
  own rollouts under its own observation schema, refused before any episode runs).
- Implemented by: BD-323.

**`train.export`** (MCP tool `train_export`; owner `frob-learn`)
- Inputs (N23a-13): a training run or policy identifier; `--at <checkpoint|latest|best-on-dev>`; `--out <path>`,
  the directory the export is written to (inside the session's path jail), or `--to store` (the policy stays in
  the store and is addressed by its `policy_id`).
- Outputs: `policy_id`, the `frob.policy/1` manifest inline with the mapping table, digests, byte size, the
  `trained_on` block (with `trained_under: D4` and the re-evaluation note for a policy of the labelled
  nondeterministic profile), and the `authorization` block with the three no-authorization flags, all `false`.
  Evidence: *pass-through*.
- The row writes outside the store (`exports = true`), so it goes through the one export door of N37-18
  (`export::admit`), whose codes stay in the rows of the exporting verbs (NJL-01): flags, privacy, licence, scope,
  path, in that order.
- Chapter refusals: `FROB-REF-731` (a private object, or a policy trained in a private scene, without the private
  scope and flag); `FROB-REF-733` (the licence set of the artifact forbids this export); `FROB-REF-734` (an `out`
  path outside the session's roots); `FROB-REF-816` (the policy's manifest lacks the no-authorization flags or one
  is not `false`); `FROB-REF-828` (a payload that carries code, or a weight format outside the allowlist).
  N23a-13 also refuses an unknown run and a run with no completed update, each with the appended 802 (`pointer` at
  the identifier, the convention for an unknown identifier; `pointer: /at` with `observed` 0, respectively), and
  writes nothing.
- Implemented by: BD-198, N23a-13.

**`train.family`** (MCP tool `train_family`; owner `frob-learn`)
- Inputs (BD-320): `class` (morphology class); `generator`; `split`; `task_family` (`--task-family`, N23bx-02);
  `designs` (count); `budget`; `devices`; the determinism profile (`--determinism`, see the notes); `approval`
  (the compute-envelope approval token; without it the call is refused at admission, exit 4); `features`
  (`--features canonical|si|si-norm`: the token feature map N23b-40 builds; default read from PILOT-C-21's record
  at admission, `canonical` on `adopt` and `si-norm` otherwise; recorded in `trained_on`); `seed` (`--seed <s>`,
  spelled as on `train.start`: the root seed of the pretraining run, recorded in `trained_on.seed`; it is part of
  the request's content, so two calls that differ only in their seed are two runs, which is how BD-321 and the M4
  pilot N10b-11 train three seeds per arm); idempotency key. The scope of the job's compute demand is
  `pretrain:<run>`, which the verb derives from `approval` (N36-20), unless a launcher's token or a cited
  `envelope_scope` names one first (the scope order of the common input `envelope_scope`, NJL-01).
  `--dry-run` trains nothing and needs no approval token: it reports the bucket plan and the parameter count; with
  `--emit-tokens` it also reports the static and dynamic token digests per design (N23b-40's tokenizer
  diagnostic). From M8, Moonshot (N23bx-01): `--arch plain|ga-encoder|ga-tree` (default `plain`, arm A; the other
  two need the feature `moonshot-ga-policy` and answer `FROB-REF-724` without it), with `--worlds`, `--backend`
  and `--steps` spelled as on `train.start` (N23bx-01's smoke run; N23bx-02 passes BD-320's `--seed` per arm).
- Outputs: a pretrained family policy (`FamilyPolicy` identity), after the compute-envelope check: counts per
  stratum, rejection rate, bucket histogram, steps, device-hours; `trained_on` with `generator`, `split`,
  `designs`, `strata`, the feature map and the approval token's hash. Evidence: *screening*.
- Chapter refusals: `FROB-REF-534` (a family policy applied outside its class or its pretraining distribution);
  `FROB-REF-541` (outside generator support: A family-policy score is requested for a design outside its training
  generator's support); `FROB-REF-704` (a campaign-class job without an approved compute envelope).
- Implemented by: BD-320 (with `features` and `seed`). Extended by: N23bx-01 (M8: `--arch` and the smoke-run
  options).

**`train.inherit`** (MCP tool `train_inherit`; owner `frob-learn`)
- Inputs (BD-322): `child` (design identity; `--child`); `donor` (a policy identity; default the policy of the
  child's parent in the lineage graph); `edit_path` (default the lineage path from the donor's design to the
  child); `task`; `budget` or `to: parent-verdict | target`; `compare: none | shared-controller | equal-resource |
  both`; `seed`; the determinism profile (`--determinism`); idempotency key; `--force-inherit` (inherit although
  the reset rule says not to; only with the scope `train:admin`, and the result is labelled `reset-overridden`);
  `--reset-depth <k>` and `--audit-rate <m>` (overrides of N23a-29's registry defaults for a test lineage,
  recorded in the manifest; the result is labelled `schedule-override` and cannot feed the gate); `--dry-run`
  (trains nothing and needs no budget: the `InheritanceMap` with its digest, the reset values in force with their
  decision status, the reset decision the rule would take and the admission state, including the refusal a real
  call would meet).
- Outputs: the child's policy started from the donor, with the matched transfer baseline declared:
  `steps_to_parent_verdict`, `adaptation_fraction`, final return and requirement pass rate, `depth`, audit-twin
  status, `origin: inherited(depth)`, the donor and map digests; the labels `experimental`, `reset-overridden` or
  `schedule-override` where they apply. Evidence: *screening*.
- Chapter refusals: `FROB-REF-531` (a policy's mapping table does not match the design, or cannot be built for an
  edit). `--force-inherit` without `train:admin` is the appended 720 (`option_capabilities`).
- Implemented by: BD-322.

**`train.start`** (MCP tool `train_start`; owner `frob-learn`)
- Inputs, as N23a-14 defines them (schema `frob.train.start.in/1`): design (positional; or a model) with `--task`
  and `--scenes <set>` (scene identities, resolved and echoed in `bound`); `--algo` (`ppo`, the only algorithm
  admitted at M2; `sac`, `td3` and `es` from M4 behind their rows of KILL-ALTALGO; `short-horizon-ac` from M6,
  Moonshot, N23b-38; an algorithm whose row in N23a-14's `registries/algorithms.toml` is `pilot-pending` or
  `killed` is refused with `FROB-REF-724`, exit 5, `reason: pilot-pending` or `reason: killed`, the envelope naming
  its pilot, the pilot's falsifier and its kill gate);
  `--network` (`mlp:<widths>`, `gru:<hidden>`, `morph-transformer:<d>,<layers>,<heads>` from N23b-41, for example
  `morph-transformer:192,4,4`, and `residual:<inner>` from N23a-20 with `--base-controller <ControllerId>`; a
  malformed string of any kind is 802); `--devices` and `--backend` (`batch-cpu`, `gpu-cuda[:0,1]`,
  `gpu-metal`; strict or planner-chosen); `--shards <n>` (the layout field of N23a-19's multi-shard update: one
  shard per device by default, on `batch-cpu` a value above 1 splits the batch on the one host, two cards with
  `--backend gpu-cuda:0,1 --shards 2` under N36-08's per-shard admission; 0 is 802); `--trainer device|frankentorch`
  (default from `registries/defaults.toml`; `cpu-readback` added by N23a-17 on KILL-LEARNKERN's fired branch,
  `frankentorch-device` by FD-FT-02 on adoption; each 724 before its bead lands); `--tying
  none|augmentation|equivariant` (default `none`; `augmentation` from N23a-32 and `equivariant` from N23a-31, both
  M4; after a fired KILL-EQUIV, `equivariant` is not admitted, N23a-33) with `--group obs|mech` (N23a-31, M4;
  DESIGN spelling, the `--group` of `phylum.quotient`: the group the weights are tied under, default `obs`, the
  observation-preserving group Γ_obs of the design and task; accepted only with `--tying equivariant`, otherwise
  802; a named group larger than Γ_obs, as `mech` is wherever a sensor, the payload or the task's goal breaks a
  mechanical symmetry, is 161); `--determinism` (the row's "profile": `device-replay`, class D2, the default;
  `fast-training`, class D4, from M3 by N35-06; `--profile` names a step profile and never a determinism
  profile) with `--precision` and `--on-degraded` (N35-04's shared parser and admission) and `--knob k=v`
  (repeatable; only with `fast-training`, N35-06's five knobs: an unknown knob is 802, an unfunded one 724, a
  D4-making knob under another profile 247); `--action-law latent-then-limit|clipped-with-endpoint-mass`
  (default the registry's, labelled in the envelope); `--worlds`, `--steps-per-collection` and the budget
  (environment steps, wall seconds or GPU seconds; `--steps <n>` is shorthand for `--budget env_steps=<n>`, both
  together 802); `--seed`; `--key`; `--eval-cadence` and `--target` (the development-stream cadence and the target
  return or success rate); `--resume <checkpoint>`; `--reset-mode explicit|autoreset-masked` (BD-192; default
  `explicit`, recorded in the manifest and in `trained_on.reset_mode`); `--zero-copy` (BD-193: refuse rather than
  materialize a view the receiver cannot represent); `--actor-channels <list | +id | -id>` (N23a-03: the actor's
  inputs, by default every `actor` channel of the task); `--init raw:<policy>` (N23a-14, M2: a plain warm start,
  the weights of a stored policy, a `PolicyId` or a checkpoint in the store, copied by position into the run's
  network, which is otherwise a new run with its own optimizer state, counters and looks; recorded as
  `init: raw:<policy>` in the manifest; a policy whose network shape differs from the run's is 802 with
  `pointer: /init`, DESIGN: the raw form ignores roles on purpose, so the mapping-table code 531, whose repairs
  are about correspondences, would misdescribe it; it is not inheritance, which is `train.inherit`; BD-322's
  raw-checkpoint arm and the arms T and R of the pilot N10b-15 use it);
  `--warm-start bc:<controller>` (BD-323: iteration zero of distillation, fitting the actor on the controller's
  own rollouts; recorded as `init: bc:<controller>`); `--dry-run` (N23a-14, M2: admission and the parameter count
  only; no job is created and no device memory allocated; exit 0 with the admission report, the resolved
  arguments with defaults labelled and the parameter count; a failing admission returns the same refusal as a
  real start; N23b-41 adds the `morph-transformer` count, N23a-31 the group and the free-parameter count under
  `--tying equivariant`).
- Inputs added from M4: `--rung R2|R3` and `--rho <fraction>` (BD-330: the rung runners, default authority 0.3
  from the controller record; a rung the ladder does not plan for the design and task starts no job and answers
  the `control.ladder` result with its reason, exit 2; `--rung R1` is 802); `--randomization fixed|adaptive`
  (default `fixed`) with `--adapt-bins`, `--adapt-every`, `--adapt-eta`, `--adapt-weighted true|false` (N23b-04;
  defaults 8, 50, 0.2, weighted); the algorithm arguments of `--algo sac` (`replay_capacity`, `utd`, `batch`,
  `tau`, `target_entropy`, `warmup_steps`; N23a-37), `--algo td3` (`explore_sigma`, `smooth_sigma`,
  `smooth_clip`, `policy_delay`, `tau` and the replay arguments; N23a-38) and `--algo es` (`population`,
  `episodes_per_member`, `sigma`, `step`, `optimizer` `airlock:cma-es|local:antithetic`, and `params`
  `policy|controller:<id>`; N23a-39), each accepted only with its algorithm (otherwise 802), MLP networks only
  for the three; `--shield <ShieldId>` (BD-338: the `ActionFilter` stage enters the environment and the run's
  identity, N22b-31); `--lag-bound <L>` (N36-12: the version-lag bound of asynchronous two-card collection,
  accepted only with `--determinism fast-training` and `--shards 2`, otherwise 802). Conformance build only, from
  M4: `--specialize-force-switch` (N21c-20: issues the switch of the job's survivor cohort to its specialized
  kernel set without the promotion decision of N17a-16, so that the switch mechanism can be tested alone; the
  switch takes effect at the next acknowledged chunk boundary). From M6: `--design-params
  <levers>`, `--window 16|32` and `--design-lr <rate>` with `--algo short-horizon-ac` (N23b-38; the algorithm
  answers 724, reason `not-linked`, until N23b-37 records `not-fired`). From M3: `--from-scratch`, `--no-carry`
  and `--match-return <run>` with `--seeds` (N28c-43: the no-reuse decision control, which also passes
  `--no-memo`); `no_memo` itself (CLI `--no-memo`) is not declared by the row but generated by NJL-01's
  variation 14 from the row's `memo = true` (BD-111's field, M1, so the property exists wherever the field is
  set, not only from M3): nothing is served from the memo graph for this call, in any store mode, every node is
  computed and still stored, and the envelope's memo disposition is `NoMemo` (the row declares no `no_memo` of
  its own; beside `memo = true` that is `common-property-shadowed`); `--schedule fixed|cycle|draw` (N19c-22: the
  addressed rule that gives each world its scene of the `--scenes` set per episode, as on `sim.run`: `fixed`, one
  scene per world for the whole job; `cycle`, entry (w + e) mod n for episode e of world w over n scenes; `draw`, a
  counter-based draw keyed by the world address and the episode index; the scene changes at a masked reset with no
  recompilation and is recorded in each episode's record);
  `--override-alias diagnosis` (N20a-41, the third repair of 445: the run is admitted on a mission with a standing
  alias witness for its interface, every result is labelled `diagnostic: standing alias witness` and is barred
  from promotion and from being cited as evidence that the mission is solvable). Conformance build only, from M3:
  `--conformance-lag <ℓ>` (N35-05: an integer 0 to 8, outside it 802 with `pointer: /conformance_lag`; accepted
  only on the CPU trainer, another tier is 802; iteration i, counting from 1, collects its rollouts with the
  weights after max(0, i − 1 − ℓ) updates, from a ring of the last ℓ + 1 versions, and nothing else changes; the
  value enters the run's identity and receipt, the envelope carries `conformance_lag: ℓ`, and the run is stored
  as non-citable; a seam of the collector for N35-05's lag study, distinct from the asynchronous mechanism's
  `--lag-bound`).
- Outputs: a training job (exit 8 with the handle after the bounded wait; the same key and content return the same
  job). The envelope carries the `randomization` block (plan identifier, provenance shares, per-world digests;
  BD-200), the action law with its default label, and, where they apply, the rung with both `ControllerId`s and
  the cap (BD-330) and `window_store_bytes` with the window in force (N23b-38). Evidence: *run* (D4-tagged under
  `fast-training`, with the citation rule).
- Chapter refusals: `FROB-REF-103` (a randomization entry naming a field with no parameter row, BD-200);
  `FROB-REF-161` (N23a-31, M4, the weight-tie raising site of N17-REF's row: `--tying equivariant` under a group
  larger than Γ_obs, one that moves a sensor pose or field of view, the payload or the task's goal; the payload
  names the generator of the requested group that fails and the element that breaks it, `observed` the requested
  group with its order, `required` Γ_obs with its order; exit 3; refused at admission and on a dry run);
  `FROB-REF-247` (a determinism profile or knob the licence check does not admit for the job, N35-04, N35-06);
  `FROB-REF-442` (the actor-schema freeze finds a truth-sourced channel classified `actor`, N23a-03; surfaced
  unchanged, N23a-14); `FROB-REF-445` (a training or optimization run is requested on a mission with a standing alias
  witness for its interface; N20a-41, M3, through `alias::standing_witness` at admission); `FROB-REF-457` (a
  shield margin used outside its scope key, under `--shield`, BD-338); `FROB-REF-522` (a non-finite action,
  raised during the job when a learned policy emits one; BD-192, N23-REF); `FROB-REF-524` (the actor schema reads
  a `critic-only` or `evaluation-only` channel, including one named by `--actor-channels`); `FROB-REF-527` (the
  action law at learning differs from the law at collection, including a `--resume` collected under another law);
  `FROB-REF-529` (no settled-pose cache for this model and task, at admission); `FROB-REF-547` (a tensor view the
  receiver cannot represent, under `--zero-copy`); `FROB-REF-554` (a future-time requirement term offered as an
  online reward, at admission, N23a-05); `FROB-REF-601` (a device lost during the job, N36-08); `FROB-REF-602` (a
  per-card memory estimate over the device budget, N36-08, or a short-horizon window store that does not fit,
  N23b-38); `FROB-REF-726` (`mechanism-not-admitted`, reason `island-in-training`, N19b-03's living-world
  admission, M3: a scene of the job declares a coupled living-world island; the purpose is checked before the
  build, so this is the answer on every build, with or without `moonshot-islands`; exit 5; N32-REF's row).
- Worded and proposed refusals of `train.start`. Each is raised through its constant, which holds the number the
  allocation ledger (N35-33, N35x-02) assigns to the request; the row's `refusals` field receives that number in
  the change that enters the registry row, and no slug or proposed number is ever written there as a code (N35-33
  rule 7): `sampler-unqualified` (proposed 558, N23-REF's request: the action sampler has no qualification receipt
  for the pinned kernel; N23b-29's `check_qualified` at admission, before any world exists);
  `randomize-numerical-parameter` (proposed 572, N23-REF's request: the task's `randomization` names a numerical
  parameter; BD-200 at environment admission); `randomize-no-interval` (proposed 571, N23bx-03's request: a card
  field whose uncertainty is `exact` or absent; BD-200); `term-contract-unmet` (N23-REF's request: the learner's
  admission pass over the `TaskProgram`, N23a-05: every finding in one refusal); `tying-group-trivial` (N23-REF's
  request, M4: `--tying equivariant` where Γ_obs is trivial, N23a-31, or `--tying augmentation` where Γ_aug is
  trivial, N23a-32; at admission and on a dry run; `pointer` `tying`); `scene-pack-not-accepted` (proposed 318, row
  1 of N19c-42, M3: a scene pack the job binds has an incomplete acceptance report or an unwaived failed fixture;
  BD-228's acceptance check at admission, which no flag skips); `scene-variant-not-resident` (proposed 319, row 2
  of N19c-42, M3: a variant of the job's scene schedule is in no pool segment bound to the job and cannot be
  admitted under the bank's cap; N19c-22, at job admission and at a reset after an administrator's evict).
- Airlock requirement (N23a-39, M4): `needs_groups = [{ groups = ["optimize"], when = "train-es-airlock-optimizer"
  }]` (N33c-32's predicate form; DESIGN predicate name, registered in `frob_verbs::admission::predicates` by
  N23a-39): `--optimizer airlock:cma-es` in a binary without the `optimize` group is the appended `FROB-REF-790`
  and never falls back silently. The entry enters the row in N23a-39's change, together with the predicate it
  names, because N33c-32's lint `needs-groups-known` refuses a row that names an unregistered predicate; the M2
  row carries `needs_groups = []`.
- Implemented by: BD-198, N23a-14.
- Extended by: N23a-03 (`--actor-channels`), N23a-05 (admission: `term-contract-unmet`, 554), N23a-17
  (`cpu-readback`), N23a-19 (`--shards`), N23b-29 (`sampler-unqualified`), BD-192 (`--reset-mode`), BD-193
  (`--zero-copy`), BD-200 (the `randomization` block, 103, `randomize-numerical-parameter`,
  `randomize-no-interval`), N23b-41 (M4: `morph-transformer` and its `--dry-run` count), N35-04 (`--determinism`,
  `--precision`, `--on-degraded`), N36-08 (two-card shards, 601, 602), N36-31, FD-FT-02 (`frankentorch-device`),
  BD-111 (`memo = true`, so `no_memo`), N28c-43 (M3), N35-06 (M3: `fast-training`, `--knob`), N19c-22 (M3:
  `--schedule`, `scene-variant-not-resident`), BD-228 (M3: `scene-pack-not-accepted`), N19b-03 (M3: 726), N20a-41
  (M3: `--override-alias`, 445), N35-05 (M3, conformance build: `--conformance-lag`), N23a-20 (M4:
  `residual:<inner>`, `--base-controller`), BD-323 (M4: `--warm-start`), BD-330 (M4: `--rung`, `--rho`), N23b-04
  (M4: `--randomization`, `--adapt-*`), N23a-37, N23a-38, N23a-39 (M4: `sac`, `td3`, `es`; N23a-39 also the
  `needs_groups` entry), BD-338 (M4: `--shield`, 457), N36-12 (M4: `--lag-bound`), N23a-31 (M4: `--tying
  equivariant`, `--group`, 161, `tying-group-trivial`), N23a-32 (M4: `--tying augmentation`, `tying-group-trivial`),
  N23a-33 (M4: `equivariant` not admitted after a fired KILL-EQUIV), N21c-20 (M4, conformance build:
  `--specialize-force-switch`), N23b-38 (M6: `short-horizon-ac`).

**`train.status`** (MCP tool `train_status`; owner `frob-learn`)
- Inputs: training run.
- Outputs (N23a-14): `state`; `progress {done, total, unit}` in environment steps, always with a denominator;
  `eta_s` with its interval; throughput in both units, `env_steps_per_s` and `world_steps_per_s`;
  `learner_share`; mean return and episode length on the training stream (descriptive, never evidence under the
  D4 profile); per-look development-stream scores with `crossing` distinguished from `attained` (N23b-33); the
  failure counts and flags. Algorithm fields: `replay_fill`, `utd`, `alpha`, the two critics' mean values and the
  target-critic lag (`sac`, N23a-37); the saturation share and the critics' mean values (`td3`, N23a-38); the
  generation index, best and mean fitness, the optimizer's step-size state and the faulted-candidate count
  (`es`, N23a-39); the current bin probabilities, the largest weight and the effective sample size (adaptive
  randomization, N23b-04); the same per window (`short-horizon-ac`, N23b-38). Evidence: *operational*.
- Chapter refusals: none of its own.
- Implemented by: BD-198, N23a-14. Extended by: N23b-33, N23a-37, N23a-38, N23a-39, N23b-04, N23b-38.

**`train.stop`** (MCP tool `train_stop`; owner `frob-learn`)
- Inputs: training run; `--now` (cancel at the next commit boundary inside the iteration and discard the partial
  iteration; N23a-14).
- Outputs: a stop receipt at a commit boundary: the current iteration finishes (collection and update complete,
  weights published), a checkpoint is written and the run ends in state `stopped`; nothing partial is ever
  published. Evidence: *operational*.
- Chapter refusals: none of its own.
- Implemented by: BD-198, N23a-14.

**Machine fields of each row.**

| Verb | Capabilities | Output schema | Task clusters | Example |
|---|---|---|---|---|
| `train.distill` | `train:run` | `frob.train.distill.out/1` | `inherit` | `frob train distill --teacher c:2be8 --task stairs-0.4 --key td-1 --json` |
| `train.export` | `train:read`, `export:policy` | `frob.train.export.out/1` | `learn` | `frob train export tr:41 --out policies/ --json` |
| `train.family` | `train:run` | `frob.train.family.out/1` | `inherit` | `frob train family --class quadruped --generator gen:quad-1 --split dev --key tf-1 --json` |
| `train.inherit` | `train:run` | `frob.train.inherit.out/1` | `inherit` | `frob train inherit --child quad-a --donor p:6f1a --compare both --key ti-1 --json` |
| `train.start` | `train:run` | `frob.train.start.out/1` | `learn` | `frob train start quad-a --task flat-velocity --algo ppo --devices cuda:0 --key tr-1 --json` |
| `train.status` | `train:read` | `frob.train.status.out/1` | `learn` | `frob train status tr:41 --json` |
| `train.stop` | `train:admin` | `frob.train.stop.out/1` | `learn` | `frob train stop tr:41 --json` |

The remaining fields follow by rule: `input_schema` is `frob.<verb name>.in/1`; `refusals` is the chapter list of
the verb's block; `idempotent` is `true` for `read` and `admin` and `false` for `run` and `mutate` (which take an
idempotency key); `cost_model` is `static: …` for `read`, `mutate` and `admin` and the `--estimate` cost model for
`run`; a `run` row carries `wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }`; `owner` is the
crate of the block; `purpose` is the Purpose column. `exports = true` on `train.export` only. `train.inherit` has
`option_capabilities = { force_inherit = ["train:admin"] }` (N32a-02's RL-22). `needs_groups` is empty on all
seven rows at M2; `train.start` gains the predicate form above in N23a-39's change (M4), with the predicate, so the
lint `needs-groups-known` holds at every step. No row takes a file path other than
`train.export`'s `--out`, an export path whose 734 its chapter list carries, so no row has `params`. `memo =
true` on `train.start` only (the field BD-111 adds to N32a-02's later-field table at M1, because the memo graph
fronts the training executor; whichever change lands second, this row's registration or BD-111's, enters it,
as NJL-01's staging rule does for codes).

One entry in full (the others follow field for field):

```toml
[[verb]]
name = "train.distill"
cluster = "train"
kind = "run"
tier = "F"
capabilities = ["train:run"]
input_schema = "frob.train.distill.in/1"
output_schema = "frob.train.distill.out/1"
refusals = ["FROB-REF-524", "FROB-REF-531"]
idempotent = false
cost_model = "estimated: `--estimate` returns CPU core-seconds, GPU seconds, memory and disk with a conformal interval"
first_milestone = "M4"
task_clusters = ["inherit"]
examples = ["frob train distill --teacher c:2be8 --task stairs-0.4 --key td-1 --json"]
owner = "frob-learn"
purpose = "Distill a teacher into a student policy"
wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }
needs_groups = []
```

**Conventions the rows follow.**

1. The MCP tool name is the verb name with `.` and `-` replaced by `_`. The output on the wire is one envelope
   `frob.envelope/1` with `result` specialized to the output schema; its `evidence` block is a claim product (the
   weakest class, tier and determinism class among the inputs).
2. The generator adds the common input properties of the kind (`run`: `sync_wait_ms` from 0 to 30,000 with MCP
   default 5,000, `idempotency_key`, `budget`, `estimate_only`, `on_cancel` (`leave`, the default, or
   `cancel_job`) and `envelope_scope` (`--envelope-scope <gate:…|campaign:…|pretrain:…>`: the approved compute
   envelope scope the job cites; its schema, its place in the scope order and its refusals are NJL-01's);
   `mutate`: `idempotency_key`, `expected_parent`). `idempotency_key` is in no JSON-schema `required` array: the
   annotation `x-frob-key` (`"required"`, or `"optional"` on a `key_optional` entry) carries the rule, and a
   missing key is refused with 805 by the admission transaction on both surfaces. The generator appends the
   surface-wide refusals: `FROB-REF-720` (scope missing), 723 (token invalid), 724 (not implemented in this
   build) and 802 (schema failure, including an unknown parameter) for every verb; 701 to 704 (budget and
   admission), 761 (above the compute envelope in force), 803 (key reused with different content) and 805
   (missing key) for `run`; 725, 803, 804 (stale parent) and 805 for `mutate`. The other properties and codes are
   added by field, as the variation table of NJL-01 lists them (that table is the one list; its variations 10
   to 16, which later beads land, are read there and not copied here): 806 and 807 for paginated reads; 808 (an
   integer outside ±(2⁵³ − 1)) for an entry whose generated input schema has an `int53` property and for no
   entry by kind (every `run` entry has one in `sync_wait_ms`, every paginated entry in `limit`); a
   `key_optional` entry is not given 805; a non-`run` entry with `estimable = true` gets `estimate_only`; 790
   for a non-empty `needs_groups`; 891 to 893 for artifact `params`; 734 and 828 for a parameter that accepts a
   path to a file (`artifact-input<F>`, `artifact-identity<F>`, `foreign-file`).
3. Exit codes: 0 done; 2 done with findings; 3 refused input (1xx to 5xx, and 8xx other than conflicts); 4 budget
   (70x and 761); 5 capability (72x, 73x); 6 device (6xx); 7 conflict (803, 804, 809); 8 accepted and still running; 9
   indeterminate; never 1 on purpose.
4. Rows are entered when the cluster is first registered (M2); each row's tool is added to the task-cluster
   tables its `task_clusters` names in the same change (NJL-05's staging rule). A verb whose handler is not in
   the build answers one envelope with `FROB-REF-724`, exit 5, on both surfaces, and `frob capabilities --json`
   lists it as absent; a reduced form that lands before the row's `first_milestone` answers and is listed as
   implemented with its form named (NJL-01, NJL-03: the field keeps the appendix's milestone).
5. Options (DESIGN, the rule of this cluster's rows: an agent told 724 learns that the option exists and is
   absent from this build or not admitted, where 802 would read as a misspelling, and the help snapshot of each
   row stays the same across milestones). An option or option value a row lists whose implementing bead is not in
   the build or not admitted (`--trainer cpu-readback` and `frankentorch-device`, `--determinism fast-training`
   before N35-06, `--network morph-transformer` before N23b-41, `--tying augmentation` and `equivariant` before
   N23a-32 and N23a-31, `--tying equivariant` after a fired KILL-EQUIV, `--arch ga-encoder` and `ga-tree` without
   `moonshot-ga-policy`, `--algo short-horizon-ac` before N23b-37's verdict) is in the input schema and answers one
   envelope `FROB-REF-724`, exit 5, naming it, never 802; a value outside the option's domain, or an option given
   without the option it requires (`--lag-bound` without `fast-training` and two shards, an algorithm argument
   without its algorithm, `--base-controller` without a `residual` network, `--group` without `--tying
   equivariant`), is 802. A conformance-build-only input (`--conformance-lag`, N35-05; `--specialize-force-switch`,
   N21c-20) is in the input schema of a binary built with the feature `conformance` only, so a release build
   answers it 802 as an unknown parameter. `--conformance-lag` enters the run's identity and receipt (N35-05), so
   a lag arm never shares an identity with an ordinary run; a forced switch is recorded in the run's placement
   receipt with the switch epoch and the state-map digest (N21c-20).

**Notes for this cluster.**

- `train.stop` stops at a commit boundary; it is an `admin` verb.
- A number from a fast-training (D4) run may not be cited as evidence without re-evaluation on a stricter profile.
- The determinism profile is spelled `--determinism` on every verb that takes one (N35-06's rule; `--profile`
  names a step profile). The Appendix J purpose of `train.start` says "profile"; N23a-14 spells it
  `--determinism`. The rows of `train.family`, `train.inherit` and `train.distill` take the same spelling (BD-320,
  BD-322 and BD-323); `fast-training` is enabled only on `train.start` (N35-06), and on the other three it is a
  value outside the option's domain, 802 (BD-320's test `determinism_option`).

## Acceptance criteria

- [ ] `registries/verbs.toml` holds exactly seven entries with `cluster = "train"` (2 `read`, 4 `run`, 1 `admin`),
      each with every field of the entry schema, an example and a task-cluster list; the alias lint finds no second
      spelling.
- [ ] `frob-surface-diff parity --cluster train` exits 0 in process (`frob-mcp` over stdio) and through `frobd
      --mcp`: registry, CLI and MCP each count 7, and every difference list is empty apart from the harness's closed
      exemptions.
- [ ] Every verb has an end-to-end script: `scripts/e2e/verbs_train.sh` runs each verb of the cluster (seven), and
      from a verb's first milestone a capability script of its implementing bead exists (`frob-surface-diff
      e2e-coverage`).
- [ ] `frob <verb> --help --json` equals `fixtures/golden/verbs/train/<verb>.help.json` byte for byte for each of
      the seven, on two runs, debug and release, Linux and macOS; a planted renamed output property is classified
      `breaking` and refused.
- [ ] A verb not yet implemented answers `FROB-REF-724`, exit 5, with one envelope on both surfaces; an implemented
      verb runs its registry example with the result its implementing bead specifies; a misspelt parameter is
      `FROB-REF-802`, exit 3; a `run` or `mutate` call without a key is `FROB-REF-805`, exit 3.
- [ ] The generated refusal table lists for each verb exactly its chapter codes plus the codes appended for its
      kind and fields (NJL-01), and every code exists in `registries/refusals.toml`.
- [ ] The gate row is recorded with the measured counts by a session that did not write the rows.

## Test plan

### Unit tests

`cargo test -p frob-verbs --test cluster_train` (the real generated table; no mocks; planted registries are TOML
strings labelled `synthetic`):
- `rows_present`: exactly 7 rows in the cluster, with the names of the rows table; a planted extra row or a removed
  row fails.
- `kinds_milestones_tiers`: each row's kind, first milestone and tier equal the rows table (for example `train.stop`
  is `admin` at M2).
- `schemas_and_scopes`: input schemas are `frob.<name>.in/1`; output schemas and capabilities equal the
  machine-fields table.
- `refusal_lists`: each row's chapter codes equal its block (`train.distill`: 524, 531, and no 830;
  `train.export`: 731, 733, 734, 816, 828; `train.family`: 534, 541, 704; `train.inherit`: 531; `train.start`:
  103, 161, 247, 442, 445, 457, 522, 524, 527, 529, 547, 554, 601, 602, 726; `train.status`: none; `train.stop`:
  none); the appended codes equal the rule for its kind and fields (NJL-01; no 790 on the M2 registry); every
  chapter code is a row of `registries/refusals.toml` (RL-06).
- `worded_refusals_by_ledger`: each worded refusal of the cluster (`distilled-student-on-teacher-states` on
  `train.distill`; on `train.start` `sampler-unqualified`, `randomize-numerical-parameter`,
  `randomize-no-interval`, `term-contract-unmet`, `tying-group-trivial`, `scene-pack-not-accepted`,
  `scene-variant-not-resident`) is in a row's `refusals` exactly under the number
  `registries/refusal_allocations.toml` records as `assigned` for its request, and in no list while the request is
  unassigned; no slug and no unassigned proposal (558, 571, 572, 318, 319) is in any list; a planted ledger
  (`synthetic`) that assigns `sampler-unqualified` a number other than the one in `train.start`'s list fails the
  test, naming the request and both numbers.
- `export_row_through_door`: `train.export` is the one row of the cluster with `exports = true`, and the registry
  lint of N37-18 finds it routed through `export::admit`; its envelope schema carries the `authorization` block; once
  the handler is in the build, `--out ../escape` is 734, exit 5, and a fixture policy whose manifest lacks a flag is
  816, exit 3, with nothing written.
- `inputs_by_row`: `train.inherit` takes `--child` (a planted `--design` is 802); `train.family --dry-run
  --emit-tokens` needs no approval token; `train.start --network morph-transformer:192,4,4` parses and
  `morph-transformer:192,4` is 802; `--network residual:mlp:128,128 --base-controller <id>` parses.
- `start_options`: the generated `frob.train.start.in/1` has every input of the block, with `determinism` (no
  `profile` property), `trainer`, `shards` (minimum 1), `action_law`, `eval_cadence`, `target`, `resume`,
  `reset_mode`, `zero_copy`, `actor_channels`, `knob` (an array), `rung` (`R2`, `R3`), `rho`, `warm_start`,
  `randomization` with the four `adapt_*`, `shield`, `lag_bound`, `design_params`, `window` (16 or 32),
  `design_lr`, `tying` with `group` (`obs`, `mech`), `scenes` with `schedule` (`fixed`, `cycle`, `draw`), `init`,
  `dry_run`, `override_alias` (`diagnosis`) and the algorithm arguments, and `no_memo` (boolean, default false,
  CLI `--no-memo`) generated by variation 14 from the row's `memo = true`, not declared by the row (a
  `synthetic` copy of the row that declares its own `no_memo` fails `common-property-shadowed`; no other row of
  the cluster has `no_memo`); `--steps` with `--budget env_steps=…` is
  802; `--lag-bound` without `--determinism fast-training --shards 2` is 802; `--rung R1` is 802; `--group mech`
  with `--tying none` is 802; the schema of a release build has neither `conformance_lag` nor
  `specialize_force_switch` (each is 802 there), and that of a conformance build has both, `conformance_lag` an
  integer from 0 to 8 (9 is 802 with `pointer: /conformance_lag`).
- `other_options`: `train.export` has `at` and `to`; `train.stop` has `now`; `train.family` has `features`
  (`canonical`, `si`, `si-norm`), `arch`, `worlds`, `backend`, `steps`, `seed`; `train.inherit` has
  `force_inherit` (with `option_capabilities` requiring `train:admin`: a token without it is 720), `reset_depth`,
  `audit_rate`, `dry_run`; `train.distill` has `design`, the teacher kinds including `privileged`, `control`
  (`shuffled-labels`), `relabel_budget`, `dry_run`.
- `needs_groups_es`: on the M2 registry every row of the cluster has `needs_groups = []` and no generated list
  contains 790. N23a-39's change, which registers the predicate `train-es-airlock-optimizer` and enters the entry,
  sets this test's expectation for `train.start` to `[{ groups = ["optimize"], when = "train-es-airlock-optimizer"
  }]`, passing `needs-groups-known`, with 790 in its generated list (N23a-39's acceptance runs it).
- `mcp_names_one_to_one`: the tool names are the mapped names (`train_distill`, `train_export`, `train_family`,
  `train_inherit`, `train_start`, `train_status`, `train_stop`), with no collision against any other cluster.
- `task_clusters_and_examples`: every row sits in a task cluster of at most seven tools or is a base tool; each
  example parses against the generated CLI and its command path is the row's own.
- `kind_behaviour`: a `run` row has the six common properties (`sync_wait_ms`, `idempotency_key`, `budget`,
  `estimate_only`, `on_cancel`, `envelope_scope`), with `idempotency_key` outside the schema's `required` array
  and annotated `x-frob-key: "required"`, and refuses `sync_wait_ms = 30001` with code 802; an `admin` row or
  action without its scope is refused with code 720; a verb whose capability is absent returns code 724, exit 5,
  one envelope.

### End-to-end script and logging

`scripts/e2e/verbs_train.sh` (header: `bead=VERBS-train milestone=M2 labels=tether,verbs,verb:train.*
gates=GATE-TETHER-02 seed=1 needs=cpu,mcp,frobd plants=alias,missing-row,renamed-field`) sources the shared helper
and `scripts/e2e/verbs_lib.sh` and calls `verbs_cluster_conformance train`. Steps: (1) `frob capabilities --json`
and `frob robot-docs --json` list the seven verbs with kind and milestone; (2) `frob-surface-diff parity --cluster
train` over stdio and through `frobd --mcp`, every difference list empty; (3) `frob-surface-diff snapshot --check
--cluster train`; (4) each verb's registry example verbatim, expecting a result (exit 0 or 2), a job handle (exit 8,
then `frob job wait`) or `FROB-REF-724` exactly when the capability report says the verb is absent; (5) for each
implemented verb the same call as an MCP tool, the two envelopes agreeing on `verb`, `ok`, `exit`, `result`,
`refusal`, `evidence`, `bound`, `warnings` and `next_actions`; (6) per verb one call with a misspelt parameter
(`e2e_expect_refusal FROB-REF-802 3`) and, for `run` and `mutate` verbs, one without a key (`e2e_expect_refusal
FROB-REF-805 3`); (7) one envelope per call and no exit code 1.

Log: `target/e2e-logs/verbs_train/<utc>.ndjson` with a transcript beside it. Each `cmd` line has the exact command,
exit code, wall time and the envelope's `evidence` block; each `assert` has observed and expected values and the tag
`--gate GATE-TETHER-02`; the metrics `registry_verbs`, `cli_verbs` and `mcp_tools` are logged against 7 and the
difference counts against 0; refusals are logged with code and first repair; a failure logs the first differing verb
and field; the `summary` line gives assertions passed and failed and the gate touched.

### Known answers and negative controls

- Known answers: the cluster has 7 verbs (2 `read`, 4 `run`, 1 `admin`); the tables above. KA-755 (the exit-code
  table on malformed, unknown, over-budget, unscoped and conflicting inputs gives the exit and refusal code of the
  surface's tables; exact) is exercised for every verb by steps (6) and (7).
- Negative controls the script must catch when planted: `alias` (a second spelling of one verb: the build fails with
  the alias lint); `missing-row` (a row removed from the served MCP list: `missing_mcp` is not empty);
  `renamed-field` (an output property renamed without a version change: the snapshot check reports `breaking`). A
  verb answering with two envelopes or exit code 1 fails step (7).
- Invariants carried: INV-TETHER-01 (every surface is generated from the registry; nothing exists on one surface and
  not the other, apart from declared per-surface defaults and the unknown-verb case) as step (2); INV-TETHER-02
  (exactly one envelope per invocation with `--json`) as step (7). Determinism lanes: entries, tool definitions and
  snapshots byte-identical twice on one machine, across debug and release and across Linux and macOS (class D0); the
  verbs' own lanes belong to their implementing beads.

## Considerations

- Closing rule: the bead closes at M2, when the rows are registered, the surfaces match and every verb passes its
  example or answers the not-implemented refusal. Verbs of this cluster first answer at M2, M4. The per-milestone
  surface beads (BD-222, BD-376) re-run this script as verbs land; that is how GATE-TETHER-02 is re-measured.
- Scope names follow the naming rule decided in BD-258 (no longer proposals): a row cites scopes of its own
  cluster by kind (`read` → `<cluster>:read`; `run` → `<cluster>:run` or a named spending scope; `mutate` →
  `<cluster>:write` or a named write scope; `admin` → `<cluster>:admin`, `job:cancel` or a named admin scope),
  plus `export:*` when the row exports and `private:*` where privacy applies, never a scope named after the
  object read or the engine spent on; N32b-01's catalogue generates the forms. `<cluster>:run` and
  `<cluster>:write` imply `<cluster>:read`, closed when a set is stored (at minting and at derivation,
  `Scopes::close_implied`), so checks stay plain set inclusion.
  Inputs and outputs above name contents, not flag spellings, except where a flag is quoted; the implementing bead
  fixes the spellings and the snapshot then pins them (additive changes keep the schema's major version; anything
  else raises it).
- Dependencies: the registry beads whose rows the chapter lists cite are dependencies, so that every listed code is
  a row of `registries/refusals.toml` when the cluster is registered (RL-06 makes any other a load-time error):
  N14-REF (103), N17-REF (161, 541), N20-REF (442, 445), N21-REF (601, 602), N22-REF (457), N23-REF (522, 524,
  527, 531, 534, 547, 554), N24-REF (529), N32-REF (704, 726), N35-REF (247), N37-REF (731, 733, 734, 816, 828);
  all are at M2 or earlier.

## References

Appendix J (J.1 entry form; J.2 inventory rows; J.5 task clusters), §23.6, §23.7, §23.10, §23.11, §23.12, §23.13, §32.4,
§32.6, §32.12.3, §32.18, §32.21; GATE-TETHER-02; INV-TETHER-01, -02; KA-755.
