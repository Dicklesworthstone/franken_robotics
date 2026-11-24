---
key: "VERBS-study"
title: "frob-verbs registry: cluster `study`, five rows, with parity, e2e and schema-snapshot checks"
type: "task"
priority: 1
milestone: "M2"
labels: ["m2", "tether", "arena", "frob-verbs", "solid", "registry", "verb", "test", "review-notes-open"]
parent: "WP-43"
deps: ["BD-022", "NJL-01", "NJL-02", "NJL-04", "NJL-05", "STD-E2E"]
needs: []
related: []
gates: ["GATE-TETHER-02"]
plan_refs: ["Appendix J (J.1, J.2, J.3, J.5)", "§32.6.1", "§24.12", "§28.6", "§32.6", "§32.12.3", "§32.21 (GATE-TETHER-02)"]
---
## Background

FrankenRobotics (a robotics simulation and design system) is reached through the command line `frob`, the daemon
`frobd` and the MCP server `frob-mcp` (the Model Context Protocol, by which an agent harness lists and calls tools).
Every capability is a verb, `cluster.verb`, declared once in `registries/verbs.toml` (crate `frob-verbs`, tier
Solid). The CLI, the daemon protocol, the MCP tools, the schemas, the refusal table and the agent guide are
generated from that file, and gate GATE-TETHER-02 compares the served surfaces with it. The registry holds 244 verbs
in 44 clusters (namespaces): the 243 of the specification and `artifact.import` (VERBS-artifact); the
count of record is NJL-03's.

This bead is the registry of record for the cluster `study`. The cluster covers the two-phase study: binding a
manifest and one study type into a plan, submitting the plan under an idempotency key, progress, cancellation, and
the comparison read from a finished study. It has five verbs (2 `read`, 1 `run`, 1 `mutate`, 1 `admin`); the
handlers live in `frob-arena`, `frob-opt`. The bead carries every row completely, with the three checks that keep
the rows true: the generated CLI commands and MCP tools match the list; every verb has an end-to-end script; the
schema printed by `frob <verb> --help --json` is stable.

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

In: the five registry entries of cluster `study`; the MCP tool names; the cluster's conformance script; the
help-schema snapshots; each verb's answer before its milestone. Out: behaviour (the implementing beads named below);
the entry schema and lints (NJL-01); the conformance harness (NJL-02); the alias lint (NJL-04); task clusters
(NJL-05, NJL-06); generation of the surfaces (the Tether seed beads).

## Technical approach

**The rows.**

| Verb | Kind | First milestone | Tier | Purpose | Defined in |
|---|---|---|---|---|---|
| `study.compare` | read | M3 | S | Per metric: `settled`, `tied` or `inconclusive`, with intervals and exact pair counts | §32.6.1 |
| `study.plan` | mutate | M2 | S | Bind a manifest and one study type; returns the plan address, the cost and the reachable verdicts | §32.6.1 |
| `study.status` | read | M2 | S | Progress of a submitted study | §32.6.1 |
| `study.submit` | run | M2 | S | Submit a planned study under an idempotency key | §32.6.1 |
| `study.cancel` | admin | M2 | S | Cancel a submitted study | §32.6 |

**Contract of each verb.** Evidence labels used below (the `evidence` block of the envelope):
- *study*: a Design comparison of class Tested (Estimated if any input is `assumed`); tier and determinism the
  weakest of the arms; `provisional` until promoted to the referee.
- *static*: class Tested, tier `static`, determinism `exact` (class D0); `claims` empty unless a certificate is
  attached; `reality: no-claim`.
- *operational*: no claim: an operational answer about the session, the store, a job or a device; `evidence.claims`
  is empty.

**`study.compare`** (MCP tool `study_compare`; owner `frob-opt`)
- Inputs (N28a-21, N32a-13): a study or job identifier (positional); optional `--metric M`. An unknown study is
  `FROB-REF-802` with pointer `study`, exit 3, the convention of `job.cancel` for an unknown job (N32a-13).
- Outputs: `frob.comparison/1`: per metric `settled`, `tied` or `inconclusive` (`incomparable` or `infeasible`
  where unscored), with the interval and the e-value or exact interval, failure counts by class per arm, the first
  divergent event on a common address, the five pair counts and `planned` (`int53`), which sum to the planned
  pairs (a sum that fails is the internal fault `FROB-REF-990`, exit 9, of N32a-03, never a printed number:
  N32a-13), and the committed frontier of the answer. Evidence: *study*.
- Chapter refusals: `FROB-REF-553` (a comparison that mixes study types, or re-reads fixed-n data as a sequential
  study); `FROB-REF-840` (a sealed confirmation study read by a session without `qual:admin` before its campaign
  closes; exit 5, N24-REF; N28a-21's firewall, INV-ATEL-11); `FROB-REF-825` (a stored study whose arm results
  include a result of class D4 or of unrecorded class, refused by the evidence boundary on the paired-statistics
  path, one of its five consumers: §37.11, INV-SAFE-07; N37-28 with BD-269's consumer list, M3; the same result
  named as an arm of `eval.compare` is refused earlier there with 542, N37-REF's division); `FROB-REF-870` (a
  verdict the record cites across an edit without a grade and a licence: BD-347's consumer check
  `revalidate::require_licence`, M4, N28-REF); `FROB-REF-890` (`stack-mismatch`: a study whose arms' evidence was
  recorded under different stack receipts, offered as same-stack pairs; N33c-31, from M1, N33-REF).
- Implemented by: BD-262, N28a-21, N32a-13. Extended by: N33c-31 (890), N37-28 (825), BD-347 (870).

**`study.plan`** (MCP tool `study_plan`; owner `frob-arena`)
- Inputs (N33b-19): a manifest (positional, a file read under the session's roots, of the registered format
  `frob.study.manifest/1`, identifier `FMT-FSTUDY`, extension `.fstudy`, owned by `frob-arena`; a `.json` file
  whose top-level `schema` member names that schema is routed as the same format, N34-24's rule; the row's
  `params` entry, below): `designs`; `scene`; `task`;
  `controller_arm` (`shared`, `retuned-equal-resources` with the resources per arm, or `inherits-weights` with
  its warm start and transfer baseline); `seed_protocol` (pairing by (world, replicate); unit `world` or
  `cluster`, with the cluster map `all` or `mean { span }`); `budget`; `backend` (at M1 `referee`); `arms`
  (optional), a list parallel to `designs` with one table of options per arm, whose members are `capacity =
  { contacts = <C>, rows = <R> }` (`rows` optional; from M1, N33b-19: the manifest form of the step-plan
  capacity override that `sim.run` spells `--capacity contacts=<C>[,rows=<R>]`, N16a-14; it enters that arm's
  `StepPlanId` and is printed per arm in the plan; on an arm whose backend is the referee it is `FROB-REF-802`
  with pointer `/arms/<i>/capacity`, as `sim.run --tier referee --capacity` is, so at M1, where the only backend
  is the referee, it is parsed, validated for shape (positive integers) and always refused; from M2 N16a-14's
  capacity registry validates the values on the batched tiers, and N18b-22's fixture `fixture:capacity-bias`
  runs both arms at a cap of 8 through it) and `solve_path` (`dense` or `ancestor`, N16c-38, M2: a study that
  states none gets one path for every arm, `ancestor` only if every arm's shape class would get it, with the
  reason `study-uniform` in the output); an unknown member of an arm's table is `FROB-REF-802`; `registration`
  (optional; BD-145: the registration document itself as an object, validated against `frob.registration/1`,
  kept in canonical form in the resolved manifest, so the plan address covers every threshold, and reported as
  `bound.registration`; a manifest without it plans exactly as before); and exactly one study type, `anytime { null, margin, span, betting_rule, level, planning_effect? }` (the optional
  standardized effect δ > 0 for sizing only) or `fixed_n { n, rule, level, margin, declared_verdicts }` (on the
  command line `--fixed-n <n> --rule … --level … --margin … --declared-verdicts <tokens>`, the flags of
  `eval.preanalyze`). The arm constructor `--kind step-refinement --base <run or plan> --steps <list>` (for
  example `--steps 2ms,1ms`; N16c-43) builds the arms by `refine` from one base plan, every clock held in
  seconds; it is not a third study type, and the manifest still carries exactly one of `anytime` and `fixed_n`.
  A manifest names immutable `DesignId`s and one task and carries no per-arm lever, card-field or task-parameter
  settings (DESIGN of N33b-19: planning is pure and writes no design version, and arms with different task
  parameters would meet different tasks, while a pair compares designs on one task instance); such a member is
  `FROB-REF-802`. The idempotency key is accepted but not required (`key_optional = true`, lint `RL-21` of N32a-02: the plan
  identifier is the content address of the resolved request, so a repeat returns the same plan; N32a-13), and
  the appended list omits `FROB-REF-805`.
- Outputs: `frob.plan/1`: the plan address, the `bound` block of resolved identities, the cost estimate with its
  interval (for an anytime plan with `planning_effect` δ also `expected_pairs` = ⌈2·ln(1/α)/δ²⌉, labelled
  `approximation: gaussian-growth`; absent without δ, never a default), and for fixed-n the embedded
  `frob.preanalysis/1` record of reachable verdicts (its `hash` equal to `eval.preanalyze`'s for the same
  arguments), for anytime the stopping thresholds; each arm's solve path; under `--kind step-refinement` per arm
  h, the derived decimation, the clock signature digest, the physical-parameter digest and any quantization.
  Runs no physics. Evidence: *static*.
- Chapter refusals, under the layer rule (convention below): `FROB-REF-532` (a fixed-n study has no
  pre-analysis, or names a verdict it cannot reach); `FROB-REF-533` (paired statistics requested under an
  uncontrolled determinism class: the code `study::admit` of N24b-14 raises at the determinism check, BD-124,
  N33b-19); `FROB-REF-543` (an `anytime` study on a metric with no declared span); `FROB-REF-552` (a request
  mixes study types); `FROB-REF-556` (a pairing keyed by event order or by a training counter). The manifest
  file's 891, 734 and 828 are appended from the row's `params` entry (convention 2), not listed here. Two
  refusals are N16-REF ledger rows that no bead numbers by hand, both `5xx`, exit 3, retry `after-change`:
  `solve-path-mixed-study` (N16c-38: arms whose `solve_path` values differ; message "study mixes solve paths",
  `observed` the two step-plan identities and their paths, `pointer` the arms; repairs 1. re-run one arm on the
  other's path, 2. declare the comparison unpaired) and `refinement-study-changes-clock` (N16c-43: arms of a
  step-refinement study whose clock signatures or physical-parameter digests differ; `observed` names what
  differs, in seconds; repairs 1. hold every clock fixed in seconds and change only h, 2. submit it as a
  comparison of two controllers). N35-33's first-round run numbers both at M0, before N16c-38 and N16c-43
  land; the row's `refusals` holds the numbers the ledger assigned, entered in the change that registers them,
  and no bead types them. Two more are N24-REF ledger rows, raised by the study-admission check that
  `study.plan` shares with `eval.compare` (N24c-37, N24c-40), both exit 3, retry `after-change`, entered in the
  row's `refusals` under the numbers the ledger assigns and typed by no bead: `exposure-table-untested`
  (N24c-37, M2: a paired study on a task whose exposure table's coupling-test status is not `passed` for the
  arms' morphology class and tier family; repair: run the coupling test on the task and the pair, `task.check`'s
  action `coupling`, the command given in `next_actions`) and `coupling-choice-after-draw` (N24c-40, M3: a study
  on a task with a locked coupling choice whose address list was derived before the task's coupling lock record,
  or that reuses the pilot seed the choice recorded as consumed; repair: draw fresh addresses).
- Implemented by: BD-124, N33b-19. Extended by: N16c-38 (the `arms` member `solve_path`,
  `solve-path-mixed-study`), N16c-43 (`--kind step-refinement`, `refinement-study-changes-clock`), BD-145 (the
  manifest member `registration`), N24c-37 (`exposure-table-untested`), N24c-40 (`coupling-choice-after-draw`).

**`study.status`** (MCP tool `study_status`; owner `frob-arena`)
- Inputs: a study or job identifier.
- Outputs: progress of the submitted study with a denominator: state, pair counts so far with the accounting
  identity; for anytime the current e-value and confidence sequence; for fixed-n progress only (counts and whether
  exact curtailment has fired; no interim estimate); the stopping reason once terminal. Evidence: *operational*.
- Chapter refusals: none of its own.
- Implemented by: BD-124, N33b-19.

**`study.submit`** (MCP tool `study_submit`; owner `frob-arena`)
- Inputs: plan address; idempotency key (a retry with the same key and plan returns the stored receipt with
  `idempotent_replay: true`; the same key with another plan is the appended conflict, exit 7); `no_memo` (CLI
  `--no-memo`, which BD-220's script passes), not declared by the row but generated by NJL-01's variation 14 from
  the row's `memo = true`, which BD-111 sets in its change (M1): nothing is served from the memo graph for this
  call, in any store mode, every node is computed and still stored, and the envelope's memo disposition is
  `NoMemo`. The row declares no `no_memo` of its own (beside `memo = true` that is `common-property-shadowed`).
- Outputs: a job (within the bounded wait, or exit 8 with a job handle: `id`, `state`, `progress {done, total,
  unit}`, `eta_s`, `poll_after_ms`, `last_committed`); on completion the study record. Evidence: *study*.
- Chapter refusals: `FROB-REF-809` (plan not found, exit 7, N33b-19 from M1; or bound to an identity that has
  since been superseded, such as a removed scene pack or a backend whose fingerprint changed its determinism
  class, naming the stale identity and the current one, N32a-13 from M3). The class of the backend is checked
  at plan time as 533 by `study::admit`; a backend whose class record changed after planning is a superseded
  identity, so this verb raises 809 and lists none of 533 and 542 (the layer rule). Also `FROB-REF-726`
  (`mechanism-not-admitted`, N32-REF, M3: a living-world mechanism the plan's episodes would run that is in the
  build but not admitted for the study's purpose or tier, with the reason and the block fields of N32-REF's row;
  N19b-03's living-world admission, applied when the job is admitted). And the N24-REF ledger row
  `coupling-choice-after-draw` of the shared study-admission check (N24c-40, M3, exit 3, numbered by the ledger
  as on `study.plan`): a plan whose address list was written to the ledger before the task's coupling lock
  record, that is a study planned before the lock and submitted after it; repair: draw fresh addresses. The
  coupling-test status (`exposure-table-untested`) is checked when the plan is made and is not listed here, as
  533 is not: a pass record is never withdrawn and the plan binds the task, the arms and the backend, so the
  status cannot change between plan and submit.
- Implemented by: BD-124, N33b-19, N32a-13 (the superseded-identity check). Extended by: N19b-03 (726, M3),
  BD-111 (`memo = true`, so `no_memo`; BD-220 calls it), N24c-40 (`coupling-choice-after-draw`, M3).

**`study.cancel`** (MCP tool `study_cancel`; owner `frob-arena`)
- Inputs: a study or job identifier (N32a-13).
- Outputs: a cancellation receipt naming the last committed pair, and the pair counts at the frontier (the
  remainder in `unscored`, which `study.compare` then reports); a study that is already terminal returns its
  terminal receipt with a warning, not a refusal. Evidence: *operational*.
- Chapter refusals: none of its own (an unknown study is 802 with pointer `study`, exit 3, as for `job.cancel`;
  N32a-13).
- Implemented by: N32a-13 (M3; a declared late row of NJL-02, see Considerations).

**Machine fields of each row.**

| Verb | Capabilities | Output schema | Task clusters | Example |
|---|---|---|---|---|
| `study.compare` | `study:read` | `frob.comparison/1` | `compare`, `studies` | `frob study compare st:9a1 --json` |
| `study.plan` | `study:write` (`key_optional = true`) | `frob.plan/1` (declares `x-frob-identity: "request-digest"`) | `compare`, `studies` | `frob study plan manifest.json --json` |
| `study.status` | `study:read` | `frob.study.status.out/1` | `compare`, `studies` | `frob study status st:9a1 --json` |
| `study.submit` | `study:submit` | `frob.study.submit.out/1` | `compare`, `studies` | `frob study submit plan:b3:52e8 --key sub-1 --json` |
| `study.cancel` | `job:cancel` | `frob.study.cancel.out/1` | `studies` | `frob study cancel st:9a1 --json` |

The remaining fields follow by rule: `input_schema` is `frob.<verb name>.in/1`; `refusals` is the chapter list of
the verb's block; `idempotent` is `true` for `read` and `admin` and `false` for `run` and `mutate` (which take an
idempotency key); `cost_model` is `static: …` for `read`, `mutate` and `admin` and the `--estimate` cost model for
`run`; a `run` row carries `wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }`; `owner` is the
crate of the block; `purpose` is the Purpose column. `params` on `study.plan`: `{ manifest =
"artifact-input<FMT-FSTUDY>" }` (N33b-19 enters the format's row in `registries/formats.toml` through N34-19's
format registry). `memo = true` on `study.submit` (the field BD-111 adds to N32a-02's later-field table and
sets on this row in its change, M1, because the memo graph fronts the study executor); the other four rows have
no `memo` field.

One entry in full (the others follow field for field):

```toml
[[verb]]
name = "study.submit"
cluster = "study"
kind = "run"
tier = "S"
capabilities = ["study:submit"]
input_schema = "frob.study.submit.in/1"
output_schema = "frob.study.submit.out/1"
refusals = ["FROB-REF-726", "FROB-REF-809"]
idempotent = false
cost_model = "estimated: `--estimate` returns CPU core-seconds, GPU seconds, memory and disk with a conformal interval"
first_milestone = "M2"
task_clusters = ["compare", "studies"]
examples = ["frob study submit plan:b3:52e8 --key sub-1 --json"]
owner = "frob-arena"
purpose = "Submit a planned study under an idempotency key"
wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }
memo = true
```

**Conventions the rows follow.**

1. The MCP tool name is the verb name with `.` and `-` replaced by `_`. The output on the wire is one envelope
   `frob.envelope/1` with `result` specialized to the output schema; its `evidence` block is a claim product (the
   weakest class, tier and determinism class among the inputs).
2. The generator adds the common input properties of the kind, as NJL-01's kinds table lists them (`run`:
   `sync_wait_ms` from 0 to the row's `wait.mcp_max_ms`, 30,000, with MCP default 5,000; `idempotency_key`, CLI
   `--key`; `budget`; `estimate_only`, CLI `--estimate`; `on_cancel`, `leave` (the default: the job keeps
   running) or `cancel_job`; `envelope_scope`, CLI `--envelope-scope <gate:…|campaign:…|pretrain:…>`, the
   approved compute-envelope scope the job cites (NJL-01; a citation that contradicts the token of the launcher
   the job runs under is 761 with pointer `/envelope_scope`, N35-REF); `mutate`: `idempotency_key` and
   `expected_parent`). `idempotency_key` is never in the JSON-schema `required` array: the annotation
   `x-frob-key: "required"` (`"optional"` on a `key_optional` entry) carries the rule, and the admission
   transaction refuses a missing key with 805 on both surfaces. The surface-wide refusals appended by kind:
   `FROB-REF-720` (scope missing), 723 (token invalid), 724 (not implemented in this build) and 802 (schema
   failure, including an unknown parameter) for every verb; 701 to 704 (budget and admission), 761 (above the
   compute envelope in force), 803 (key reused with different content) and 805 (missing key) for `run`; 725, 803,
   804 (stale parent) and 805 for `mutate`. The properties and codes driven by fields are those of NJL-01's
   variation table: `cursor`, `limit` and `follow` with 806 and 807 for a paginated entry or a paginated action;
   808 (an integer outside ±(2⁵³ − 1)) for an entry whose generated input schema has an `int53` property at any
   depth, whatever its kind (every `run` entry has one, `sync_wait_ms`); `estimate_only` on a non-`run` entry
   with `estimable = true`; no 805 for a `key_optional` entry; the `action` property, with each action's codes,
   for an entry with `actions`; 790 for a non-empty `needs_groups`; 891 for an `artifact-input<F>`,
   `artifact-ref<F>` or `artifact-identity<F>` parameter, and 892 as well for `artifact-identity<F>`; 893 for a
   `foreign-file` parameter; 734 and 828 for a parameter that accepts a path to a file (`artifact-input<F>`,
   `artifact-identity<F>`, `foreign-file`). The later variations of that table (10 to 16) land with their beads
   and are read there, not copied here; the one this cluster's rows use is 14, `no_memo` from `memo = true` on
   `study.submit` (BD-111).
   In this cluster the one input that accepts a path to a file is `study.plan`'s positional manifest, a `params`
   entry of kind `artifact-input<FMT-FSTUDY>` (N33b-19), so that row gets 891 (a manifest of an unsupported
   schema version), 734 (the path jail) and 828 (the dispatcher's content sniff of N37-03) by rule; a file of
   another registered format given as the manifest is 802 naming both formats (N34-24), and N37-REF's list test,
   which requires 734 and 828 of every verb with a file argument, reads the appended list. The other inputs are
   identifiers.
3. Exit codes: 0 done; 2 done with findings; 3 refused input (1xx to 5xx, and 8xx other than conflicts); 4 budget
   (70x and 761); 5 capability (72x, 73x); 6 device (6xx); 7 conflict (803, 804, 809); 8 accepted and still running; 9
   indeterminate; never 1 on purpose.
4. Rows are entered when the cluster is first registered (M2); each row's tool is added to the task-cluster
   tables its `task_clusters` names in the same change (NJL-05's staging rule). A verb whose handler is not in
   the build answers one envelope with `FROB-REF-724`, exit 5, on both surfaces, and `frob capabilities --json`
   lists it as absent; a reduced form that lands before the row's `first_milestone` answers and is listed as
   implemented with its form named (NJL-01, NJL-03: the field keeps the appendix's milestone).

**Notes for this cluster.**

- `study.plan` binds a manifest to immutable designs, scene, task, seed protocol, budget, backend and one declared
  study type (anytime-valid or fixed-n exact, never both). It returns the plan address (the content address of the
  resolved manifest), the cost estimate with its interval and, for fixed-n, the pre-analysis of reachable verdicts.
  Planning runs no physics and is idempotent by construction.
- `study.submit` takes the plan address and an idempotency key, so the submitted object cannot drift from the
  planned one; a plan bound to a superseded identity is refused (`FROB-REF-809`).
- Forms that answer before the row's milestone (BD-137's M1 set): `study.plan`, `study.submit` and `study.status`
  answer at M1 on the CPU (N33b-19 on BD-124's engines; backend `referee`), and `frob capabilities --json` lists
  them implemented with `form: "cpu"`; their rows keep the appendix's M2 (BD-137's
  `early_reduced_form_is_implemented_not_remilestoned` test reads `study.plan`). BD-137 enters the rows at M1 and
  this bead completes them, as for the sensing cluster.
- The layer rule (settled in N24-REF and N28-REF; both codes of each pair stay registered, one situation raises
  one code on one verb): the Arena study verbs (`study.plan`, `study.submit`, `study.status`) admit through
  `study::admit` (N24b-14), which runs the Arena checks first and answers 552 (study types mixed), 532
  (unreachable or missing pre-analysis) and 533 (uncontrolled determinism class); `study.compare`, `eval.compare`
  and the Atelier verbs admit through `pairing::admit` and answer the twins 553, 555 and 542. 543 and 556 have no
  Arena twin and are listed by both paths. So `study.plan` lists 532, 533, 543, 552, 556 and none of 553, 555,
  542; `study.submit` lists none of the six (809, 726 and the ledger row `coupling-choice-after-draw`);
  `study.compare` lists 553 and none of 552, 532, 533.
  N28-REF's test `layer_rule_one_code_per_pair` fails a verb row that lists both codes of a pair (552/553,
  532/555, 533/542). `solve-path-mixed-study` on `study.plan` (solve paths) is not one of those pairs, and a D4
  arm inside a stored study read by `study.compare` is the evidence boundary's 825 (N37-28), not a pair code.
- `study.compare` returns per metric `settled`, `tied` or `inconclusive` (`incomparable` or `infeasible` where
  unscored), with pair counts that satisfy a_only + b_only + both + neither + unscored = planned (INV-TETHER-04).

## Acceptance criteria

- [ ] `registries/verbs.toml` holds exactly five entries with `cluster = "study"` (2 `read`, 1 `run`, 1 `mutate`, 1
      `admin`), each with every field of the entry schema, an example and a task-cluster list; the alias lint finds no
      second spelling.
- [ ] `frob-surface-diff parity --cluster study` exits 0 in process (`frob-mcp` over stdio) and through `frobd
      --mcp`: registry, CLI and MCP each count 5, and every difference list is empty apart from the harness's closed
      exemptions.
- [ ] Every verb has an end-to-end script: `scripts/e2e/verbs_study.sh` runs each verb of the cluster (five), and
      from a verb's first milestone a capability script of its implementing bead exists (`frob-surface-diff
      e2e-coverage`).
- [ ] `frob <verb> --help --json` equals `fixtures/golden/verbs/study/<verb>.help.json` byte for byte for each of
      the five, on two runs, debug and release, Linux and macOS; a planted renamed output property is classified
      `breaking` and refused.
- [ ] A verb not yet implemented answers `FROB-REF-724`, exit 5, with one envelope on both surfaces; an implemented
      verb runs its registry example with the result its implementing bead specifies; a misspelt parameter is
      `FROB-REF-802`, exit 3; a `run` or `mutate` call without a key is `FROB-REF-805`, exit 3, except `study.plan`
      (`key_optional`), whose keyless call returns the content-addressed plan, and a repeat returns the same one.
- [ ] In the M1 build `study.plan`, `study.submit` and `study.status` answer with `form: "cpu"` in the capability
      report while their rows read M2.
- [ ] The generated refusal table lists for each verb exactly its chapter codes plus the codes appended for its
      kind and fields (NJL-01), and every code exists in `registries/refusals.toml`.
- [ ] The gate row is recorded with the measured counts by a session that did not write the rows.

## Test plan

### Unit tests

`cargo test -p frob-verbs --test cluster_study` (the real generated table; no mocks; planted registries are TOML
strings labelled `synthetic`):
- `rows_present`: exactly 5 rows in the cluster, with the names of the rows table; a planted extra row or a removed
  row fails.
- `kinds_milestones_tiers`: each row's kind, first milestone and tier equal the rows table (for example
  `study.cancel` is `admin` at M2).
- `schemas_and_scopes`: input schemas are `frob.<name>.in/1`; output schemas and capabilities equal the
  machine-fields table (`study.compare` returns `frob.comparison/1`) (`study.plan` returns `frob.plan/1`).
- `refusal_lists`: each row's chapter codes equal its block (`study.compare`: 553, 825, 840, 870, 890;
  `study.plan`: 532, 533, 543, 552, 556, and the numbers the ledger assigned to `solve-path-mixed-study` and
  `refinement-study-changes-clock`, read from `registries/refusal_allocations.toml` by slug, none typed in the
  test, and, in a registry where the ledger has assigned them, the numbers of `exposure-table-untested` and
  `coupling-choice-after-draw`, read the same way; `study.status`: none; `study.submit`: 726, 809, and the
  number assigned to `coupling-choice-after-draw` read the same way, with no number for
  `exposure-table-untested`; `study.cancel`: none; no slug and no unassigned number is in any list); the
  appended codes equal the rule for its kind and fields (NJL-01) (`study.plan` without 805, and with 891, 734
  and 828 from its `params` entry; 808 on `study.submit`, whose `sync_wait_ms` is an `int53` property).
- `layer_rule`: no row of the cluster lists both codes of 552/553, 532/555 or 533/542; a synthetic registry in
  which `study.plan` lists 542 fails with the pair named (the same check as N28-REF's
  `layer_rule_one_code_per_pair`, run on this cluster's rows).
- `plan_inputs`: `study.plan` accepts a manifest with an `arms` list whose tables carry `capacity = { contacts,
  rows }` and `solve_path`, an optional `registration` object, `planning_effect` in an anytime block, the fixed-n
  flags `--fixed-n --rule --level --margin --declared-verdicts`, and `--kind step-refinement --base r:1 --steps
  2ms,1ms`; `--kind` with any other value is 802; an unknown member of an arm's table (a lever or task-parameter
  setting) is 802; the row's `params` entry is `manifest = "artifact-input<FMT-FSTUDY>"`.
- `plan_key_optional`: `study.plan` has `key_optional = true`, its output schema declares `x-frob-identity:
  "request-digest"`, and two keyless calls on one manifest return one plan identifier.
- `mcp_names_one_to_one`: the tool names are the mapped names (`study_compare`, `study_plan`, `study_status`,
  `study_submit`, `study_cancel`), with no collision against any other cluster.
- `task_clusters_and_examples`: every row sits in a task cluster of at most seven tools or is a base tool; each
  example parses against the generated CLI and its command path is the row's own.
- `kind_behaviour`: a `run` row has the six common properties of its kind (`sync_wait_ms`, `idempotency_key`
  with `x-frob-key: "required"` and absent from the schema's `required` array, `budget`, `estimate_only`,
  `on_cancel` with the values `leave` and `cancel_job`, `envelope_scope`) and refuses `sync_wait_ms = 30001`
  with code 802; `study.submit` has `memo = true` and its generated input schema has `no_memo` (boolean,
  default false, CLI `--no-memo`) from variation 14 and not as a row property (a `synthetic` copy of the row that
  declares its own `no_memo` beside `memo = true` fails `common-property-shadowed`), and no other row of the
  cluster has `no_memo`; a `mutate` row has `idempotency_key` and `expected_parent` and, unless `key_optional` (then
  `x-frob-key: "optional"`), refuses a call without the key with code 805; an `admin` row or action without its
  scope is refused with code 720; a verb whose capability is absent returns code
  724, exit 5, one envelope.

### End-to-end script and logging

`scripts/e2e/verbs_study.sh` (header: `bead=VERBS-study milestone=M2 labels=tether,verbs,verb:study.*
gates=GATE-TETHER-02 seed=1 needs=cpu,mcp,frobd plants=alias,missing-row,renamed-field`) sources the shared helper
and `scripts/e2e/verbs_lib.sh` and calls `verbs_cluster_conformance study`. Steps: (1) `frob capabilities --json`
and `frob robot-docs --json` list the five verbs with kind and milestone; (2) `frob-surface-diff parity --cluster
study` over stdio and through `frobd --mcp`, every difference list empty; (3) `frob-surface-diff snapshot --check
--cluster study`; (4) each verb's registry example verbatim, expecting a result (exit 0 or 2), a job handle (exit 8,
then `frob job wait`) or `FROB-REF-724` exactly when the capability report says the verb is absent; (5) for each
implemented verb the same call as an MCP tool, the two envelopes agreeing on `verb`, `ok`, `exit`, `result`,
`refusal`, `evidence`, `bound`, `warnings` and `next_actions`; (6) per verb one call with a misspelt parameter
(`e2e_expect_refusal FROB-REF-802 3`) and, for `run` and `mutate` verbs other than `study.plan`, one without a key
(`e2e_expect_refusal FROB-REF-805 3`), while `study.plan` is called twice without a key and the two plan
identifiers are asserted equal; (7) one envelope per call and no exit code 1.

Log: `target/e2e-logs/verbs_study/<utc>.ndjson` with a transcript beside it. Each `cmd` line has the exact command,
exit code, wall time and the envelope's `evidence` block; each `assert` has observed and expected values and the tag
`--gate GATE-TETHER-02`; the metrics `registry_verbs`, `cli_verbs` and `mcp_tools` are logged against 5 and the
difference counts against 0; refusals are logged with code and first repair; a failure logs the first differing verb
and field; the `summary` line gives assertions passed and failed and the gate touched.

### Known answers and negative controls

- Known answers: the cluster has 5 verbs (2 `read`, 1 `run`, 1 `mutate`, 1 `admin`); the tables above. KA-755 (the
  exit-code table on malformed, unknown, over-budget, unscoped and conflicting inputs gives the exit and refusal
  code of the surface's tables; exact) is exercised for every verb by steps (6) and (7).
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

- `study.cancel` is named by no seed bead; a new bead of the surface unit (N32a-13) carries it, beside `job.cancel`.
  N32a-13 is an M3 bead while the row keeps the appendix's `first_milestone = "M2"`, so in M2 builds the verb
  answers `FROB-REF-724`, exit 5, and is listed absent; the row is one of the declared late rows of NJL-02's closed
  list `LATE_ROWS` (N32a-13, lands M3), so the capability level of the coverage check accepts that answer at M2 and
  not from M3.
- Round 3 (DESIGN, decided by the plan and the implementing bead): `FROB-REF-809` is the plan's "plan not found,
  or bound to an identity that has since been superseded" (§32.18), a `study.submit` code; an unknown *study* is
  not a plan, and N32a-13, which implements `study.compare` and `study.cancel`, answers it with 802 and pointer
  `study`, as `job.cancel` does for an unknown job, and so does N28a-21. So `study.compare`'s row has no 809; it
  has 840, with exit 5 as N24-REF registers it and N28a-21 states.
- Closing rule: the bead closes at M2, when the rows are registered, the surfaces match and every verb passes its
  example or answers the not-implemented refusal. Verbs of this cluster first answer at M2, M3. The per-milestone
  surface beads (BD-222, BD-312) re-run this script as verbs land; that is how GATE-TETHER-02 is re-measured.
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

## References

Appendix J (J.1 entry form; J.2 inventory rows and J.3 added rows; J.5 task clusters), §32.6.1, §24.12, §28.6,
§32.4, §32.6, §32.12.3, §32.18, §32.21; GATE-TETHER-02; INV-TETHER-01, -02; KA-755.
