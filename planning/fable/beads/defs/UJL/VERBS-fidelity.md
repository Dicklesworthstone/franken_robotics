---
key: "VERBS-fidelity"
title: "frob-verbs registry: cluster `fidelity`, five rows, with parity, e2e and schema-snapshot checks"
type: "task"
priority: 1
milestone: "M2"
labels: ["m2", "tether", "kinesis", "frob-verbs", "solid", "registry", "verb", "test", "review-notes-open"]
parent: "WP-43"
deps: ["BD-022", "NJL-01", "NJL-02", "NJL-04", "NJL-05", "STD-E2E"]
needs: []
related: []
gates: ["GATE-TETHER-02"]
plan_refs: ["Appendix J (J.1, J.2, J.3, J.5)", "§16.9", "§16.11", "§16.13", "§32.6", "§32.12.3", "§32.21 (GATE-TETHER-02)"]
---
## Background

FrankenRobotics (a robotics simulation and design system) is reached through the command line `frob`, the daemon
`frobd` and the MCP server `frob-mcp` (the Model Context Protocol, by which an agent harness lists and calls tools).
Every capability is a verb, `cluster.verb`, declared once in `registries/verbs.toml` (crate `frob-verbs`, tier
Solid). The CLI, the daemon protocol, the MCP tools, the schemas, the refusal table and the agent guide are
generated from that file, and gate GATE-TETHER-02 compares the served surfaces with it. The registry holds 244 verbs
in 44 clusters (namespaces): the 243 of the specification and `artifact.import` (VERBS-artifact); the
count of record is NJL-03's.

This bead is the registry of record for the cluster `fidelity`. The cluster covers the trust ladder between tiers:
the fidelity ledger with its frozen envelopes, the audit for energy created by the simulator, the referee's coverage
figures, the model-form rank check, and the sampled background referee. It has five verbs (3 `read`, 1 `run`, 1
`admin`); the handlers live in `frob-dyn`. The bead carries every row completely, with the three checks that keep
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

This bead is the only place where the rows of cluster `fidelity` are written: every field of the five entries
(name, kind, tier, capabilities, schema names, refusal list, first milestone, task clusters, example, owner). The
dynamics unit's bead N16b-27 reads these rows and writes none.

In: the five registry entries of cluster `fidelity`; the MCP tool names; the cluster's conformance script; the
help-schema snapshots; each verb's answer before its milestone. Out: behaviour (the implementing beads named below);
the entry schema and lints (NJL-01); the conformance harness (NJL-02); the alias lint (NJL-04); task clusters
(NJL-05, NJL-06); generation of the surfaces (the Tether seed beads); the owner decision on the name of the
tier-pair rank verb and on the first milestone of `fidelity.shadow`, with its decision record, and the check of
these rows against the dynamics chapter's verb table and refusal table (N16b-27, which depends on this bead; the
defaults in force are stated in the notes below and the rows follow them).

## Technical approach

**The rows.**

| Verb | Kind | First milestone | Tier | Purpose | Defined in |
|---|---|---|---|---|---|
| `fidelity.ledger` | read | M2 | S | Ledger rows and frozen envelopes | §16.13 |
| `fidelity.energy` | read | M2 | S | Created-energy totals | §16.13 |
| `fidelity.coverage` | read | M2 | S | Both coverage figures | §16.13 |
| `fidelity.modelform` | run | M3 | S | `model-robust` or `model-conditional` for a ranking | §16.9, §16.13 |
| `fidelity.shadow` | admin | M3 | S | Start, stop and configure the background referee for a job | §16.13 |

**Contract of each verb.** Evidence labels used below (the `evidence` block of the envelope):
- *pass-through*: passes through the claim product of the stored objects it reads and adds no claim of its own; the
  read itself is `exact`.
- *study*: a Design comparison of class Tested (Estimated if any input is `assumed`); tier and determinism the
  weakest of the arms; `provisional` until promoted to the referee.
- *operational*: no claim: an operational answer about the session, the store, a job or a device; `evidence.claims`
  is empty.

**`fidelity.ledger`** (MCP tool `fidelity_ledger`; owner `frob-dyn`, with the envelope store in `frob-chronicle`;
paginated: `paginated = true`, so the generator adds `cursor`, `limit`, `follow` and appends 806 and 807)
- Inputs: `--class` (required); `--tier-pair` (required except with `--cases`, ordered fast,slow; a pair that names
  one tier twice, for example `--tier-pair batch-cpu,batch-cpu --quantity coefficient-lag`, asks for the intrinsic
  rows of that tier, N16bp1-01); optional `--quantity` (default all), `--scene-class`, `--profile-pair`,
  `--law-pair`, `--kernel-versions`, `--battery` (a battery name or a `BatteryId`; default the frozen battery of
  the class); `cases` (`--cases`, a flag; N16bp1-01 step 10, plan §26.6: list the class's tier-sensitive cases in
  place of strata, newest first, paginated against the same pinned frontier; with it `--tier-pair` is optional and
  filters on the case's tier pair, and `--quantity`, `--profile-pair`, `--law-pair` and `--battery` are
  `FROB-REF-802`).
- Outputs: ledger rows per stratum `{ stratum_key, kind, sub_key, n, unpaired_by_cause, median, p95, p99, max,
  bias, rho { continuous, binary }, decisions { agree, discordant_fast_only, discordant_slow_only }, envelope { id,
  bound, unit, basis, frozen_at, rationale } | null, adjudicators { referee, batch-cpu:strict }, gate { id,
  threshold_median, threshold_p95, verdict } | null }`, paginated against a committed frontier pinned for the whole
  walk (`next_cursor`). `kind` is `pair` for a row between two tiers, `intrinsic` for a row of one tier and `case`
  for a tier-sensitive case (below);
  `sub_key` is `{ coefficient_law, contact_mode }` on an intrinsic `coefficient-lag` row and `null` otherwise, and
  an intrinsic row carries `null` in every pair-only field (N16bp1-01; an additive change of the schema, whose
  golden help file `fixtures/golden/verbs/fidelity/fidelity.ledger.help.json` is regenerated in the same change).
  With `--cases` each row is a tier-sensitive case, `kind: "case"`: a failure that the Nemesis artifact guard
  (BD-272, M3) labels `artifact-candidate`, from a world that is not a battery address, with the record
  `frob.fidelity.case/1` (`case_id`, `source` (`nemesis.guard`), `class`, `run`, `world`, `model_id`,
  `candidate`, `quantity` (`task-decision`), `pair_or_variant`, `step_plan_ids`, `outcomes`, `recorded_at`) and
  `null` in every stratum field; a case enters no stratum, distribution, envelope or promotion. Until BD-272 lands
  the case table is empty and the read returns no rows, exit 0 (an additive change of the input and output
  schemas, with the golden help file regenerated in the same change). An unfrozen stratum prints `envelope: null` and `next_actions: ["freeze"]` with the freeze command line (the
  option `--freeze-envelopes` of `eval.run`, row of VERBS-eval). Exit 2 when a returned stratum misses its gate
  threshold (a finding). Evidence: *pass-through* (claim Fidelity, class Tested, determinism D3 across tiers; the
  read itself is exact).
- Chapter refusals: none of its own (`FROB-REF-821`, envelope not frozen, is raised by the consumers of the ledger,
  promotion first, never by this read).
- Implemented by: N16bp1-01 (the verb, the envelope record and store, the freeze routine, the case table and
  `--cases`), with the measured M2 rows from BD-212 and the cases from BD-272's guard.

**`fidelity.energy`** (MCP tool `fidelity_energy`; owner `frob-dyn`)
- Inputs: run; optional `--world` (one world address), `--by` (`mode` or `world`), `--limit` (n worlds listed).
- Outputs: created-energy totals per world and contact mode, with the worlds that crossed the bound and their worst
  window; exit 2 when any world crossed it (a finding). Evidence: *pass-through*.
- Chapter refusals: `FROB-REF-840` (a run sealed in an open qualification campaign, asked for by a session without
  the qualification administration scope); `FROB-REF-888` (the window buffer or Contact stream needed for the scope
  asked is missing: a metric over missing samples).
- Implemented by: BD-208, N29a-22 (the data contract of the audit it reads is N16b-14).

**`fidelity.coverage`** (MCP tool `fidelity_coverage`; owner `frob-dyn`)
- Inputs: `--run` or `--class` (exactly one of the two); optional `--battery` (a battery name or a `BatteryId`;
  default the frozen standard battery of the class; BD-213).
- Outputs: both coverage figures of the referee: `shadow` (for a run, its `referee_coverage = { gpu_world_seconds,
  shadowed_world_seconds, share, one_step_defects, outcome_pairs, adjudicator }`, `share: 0` when no background
  referee ran; for a class, the latest recorded shares of its runs) and `referee_own` (the referee's own coverage
  per robot class with its causes and adjudicator, or `"not measured"` with the next action that would measure it,
  the `eval.run --adjudicate … --record coverage` campaign of BD-213). It never runs physics. Evidence:
  *pass-through* (claim Fidelity, class `Tested`, tier `referee`; the battery identity and stack-lock digest among
  the identities).
- Chapter refusals: none of its own (an unknown run or class is a schema failure, the appended 802).
- Implemented by: BD-213 (the referee's own coverage), BD-214 (the background share).

**`fidelity.modelform`** (MCP tool `fidelity_modelform`; owner `frob-dyn`)
- Inputs: `--ranking` (a stored final-round ranking) or `--study` (the ranking of that study's final round); `--k`
  (the top k checked, DESIGN default 8; the runners-up are ranks k + 1 to 2k; a ranking of k designs or fewer is
  `FROB-REF-802`); `--budget-share` (the share of the campaign the check may spend, default 0.10);
  `--on-overflow refuse|continue-flagged` (the policy for a variant world that overflows its contact or row
  capacity, BD-266 step 3; DESIGN default `refuse`, the default for evidence-bearing evaluation of N21b-23; `reset`
  is not offered, because a reset episode has no score to rank); the model-form variant families come from the
  step plan's registered variants.
- Outputs: `model-robust` or `model-conditional`, with the variant that flipped it (`flipped_by`); the model-form
  record `frob.modelform/1`: `{ ranking, k, variants: [{ family, arm, base_step_plan, step_plan, model_id,
  param_digest_base, param_digest, overlay, admitted, reason, top_k_overlap, order }], energy_audit: { clean,
  flagged }, verdict, flipped_by, cost: { share, episodes_per_arm }, next_actions }`. Each variant entry carries
  every field of N16b-13's `ModelFormVariant` record, so that a reader can audit from the verb's own output that
  each variant changed one thing (INV-KIN-07's digests); under `continue-flagged` a variant's overlap is reported
  with its count of degraded episodes. The help-schema snapshot is updated with these fields in the same change
  (additive). Exit 0 for `model-robust`, 2 for `model-conditional` or any `not-run` arm, 8 while running.
  Evidence: *study*.
- Chapter refusals: `FROB-REF-722` (capacity overflow, refused: a world of a variant run exceeds its contact or row
  capacity under `--on-overflow refuse`, on `batch-cpu`), raised by the verb itself; `FROB-REF-610` (the same on a
  GPU tier, KNOWN_DRIFTS 6: the two codes are not merged), when the ranking's variants run on a device tier.
- Correction (round 3): the row formerly also listed `FROB-REF-557` and `FROB-REF-830` as codes of the verbs that
  consume this verb's result (DESIGN shared with BD-266). The chapter registry N28-REF states that
  `fidelity.modelform` sets the `model-conditional` condition and raises neither 557 nor 830 ("a row lists what its
  handler can return"), and its two-way check `verbs_lists_agree` fails on a row that lists a chapter-28 code its
  verbs column does not name. The two codes stay on their raising verbs: 557 on `report.design` and
  `report.campaign`, 830 on `eval.promote` and the build-packet sites. KNOWN_DRIFTS 12: the registry is the
  authority.
- Implemented by: BD-266.

**`fidelity.shadow`** (MCP tool `fidelity_shadow`; owner `frob-dyn`)
- Inputs: the input property `action`, one of `start`, `stop`, `configure`, `status` (BD-214), and `audit` on the
  KILL-SHADOW branch only (N16b-20; the value joins the enumeration in the same change, an additive schema
  change); `--job` (the device job shadowed; `--run` for `audit`); for `start` and `configure`: `--p` (the sampling
  probability of a world's control interval), `--cpu-cores` (the CPU workers given to the referee), `--tier` (the
  higher tier, default `referee`). Every action requires `fidelity:admin`.
- Outputs: the configuration in force, the running `referee_coverage`, `dropped_snapshots`, the predicted share
  beside the measured share, and the cost (CPU core-seconds used, measured throughput loss when a paired
  measurement exists); for `audit`, the audit receipt (defects by stage, suspect stages, audited share), all in
  `frob.fidelity.shadow.out/1`. Evidence: *operational*.
- Chapter refusals (the referee forms, N33-REF): `FROB-REF-791` (`referee-worker-mismatch`: with the referee in
  mode `remote`, the worker is absent, gives no handshake within the start timeout, speaks another protocol major
  version, carries a stack receipt other than the job's, or is a development build offered for gate-grade
  evidence; raised before any job frame is sent; exit 5, its registry row's); `FROB-REF-792`
  (`referee-upstream-refusal`: the referee refused for a reason that has no dedicated code, raised by the airlock
  adapter's refusal conversion, N33a-13; exit 3). A build without the feature `frontier-shadow` answers
  `FROB-REF-724`.
- Implemented by: BD-214 (`start`, `stop`, `configure`, `status`); N16b-20 (`audit`, KILL-SHADOW branch).

**Machine fields of each row.**

| Verb | Capabilities | Output schema | Task clusters | Example |
|---|---|---|---|---|
| `fidelity.ledger` | `fidelity:read` | `frob.fidelity.ledger.out/1` | `fidelity` | `frob fidelity ledger --class quadruped --tier-pair batch-cpu,referee --quantity energy-per-metre --json` |
| `fidelity.energy` | `fidelity:read` | `frob.fidelity.energy.out/1` | `fidelity` | `frob fidelity energy r:19e2 --json` |
| `fidelity.coverage` | `fidelity:read` | `frob.fidelity.coverage.out/1` | `fidelity` | `frob fidelity coverage --class humanoid --json` |
| `fidelity.modelform` | `fidelity:run` | `frob.fidelity.modelform.out/1` | `fidelity`, `inherit` | `frob fidelity modelform --study st:9a1 --key mf-1 --json` |
| `fidelity.shadow` | `fidelity:admin` | `frob.fidelity.shadow.out/1` | `fidelity` | `frob fidelity shadow start --job j:77c0 --p 0.002 --cpu-cores 24 --tier referee --json` |

The remaining fields follow by rule: `input_schema` is `frob.<verb name>.in/1`; `refusals` is the chapter list of
the verb's block; `idempotent` is `true` for `read` and `admin` and `false` for `run` and `mutate` (which take an
idempotency key); `cost_model` is `static: …` for `read`, `mutate` and `admin` and the `--estimate` cost model for
`run`; a `run` row carries `wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }`; `owner` is the
crate of the block; `purpose` is the Purpose column.

One entry in full (the others follow field for field):

```toml
[[verb]]
name = "fidelity.modelform"
cluster = "fidelity"
kind = "run"
tier = "S"
capabilities = ["fidelity:run"]
input_schema = "frob.fidelity.modelform.in/1"
output_schema = "frob.fidelity.modelform.out/1"
refusals = ["FROB-REF-610", "FROB-REF-722"]
idempotent = false
cost_model = "estimated: `--estimate` returns CPU core-seconds, GPU seconds, memory and disk with a conformal interval"
first_milestone = "M3"
task_clusters = ["fidelity", "inherit"]
examples = ["frob fidelity modelform --study st:9a1 --key mf-1 --json"]
owner = "frob-dyn"
purpose = "`model-robust` or `model-conditional` for a ranking"
wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }
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
   indeterminate; never 1 on purpose. A code whose registry row states another exit keeps its row's exit
   (`FROB-REF-791`, exit 5).
4. Rows are entered when the cluster is first registered (M2); each row's tool is added to the task-cluster
   tables its `task_clusters` names in the same change (NJL-05's staging rule). A verb whose handler is not in
   the build answers one envelope with `FROB-REF-724`, exit 5, on both surfaces, and `frob capabilities --json`
   lists it as absent; a reduced form that lands before the row's `first_milestone` answers and is listed as
   implemented with its form named (NJL-01, NJL-03: the field keeps the appendix's milestone).

**Notes for this cluster.**

- Five registered verbs. The Kinesis chapter's verb table prints a sixth, `fidelity.rank` (the rank-agreement record
  for a design set and a tier pair), and says the cluster has six. The registry appendix registers that record under
  `eval.rank-agreement` "for rungs and for tier pairs", counts 55 added verbs (which the appendix's own table
  reaches only without `fidelity.rank`), and assigns "its five verbs" to the task cluster `fidelity`. Both names
  cannot be registered for one mechanism. Fail-closed default until the owner decides (the decision is recorded with
  bead N16b-27): `fidelity.rank` is not a verb; `eval.rank-agreement` accepts a tier pair. If the owner chooses
  `fidelity.rank`, it becomes a sixth row of this bead (kind `run`, M3, executor BD-265) and the registry holds 245
  verbs (NJL-03's count with `artifact.import`).
- `fidelity.shadow` is of kind `admin`. The inventory gives it M3, in both verb tables of the plan; its seed bead
  (BD-214) is at M2 and GATE-FID-07 is recorded at the M2 exit. DESIGN, the same statement in N16b-27, where the
  question is put to the owner: the row's `first_milestone` is M3 as printed; at M2 the verb is delivered behind
  the feature `frontier-shadow` and a build without that feature answers `FROB-REF-724`; if the owner moves the
  row to M2 only this field and the test value below change.
- The task cluster `fidelity` (declared by NJL-05) holds these five tools with `eval_promote` and
  `eval_rank_agreement`: seven. `fidelity_modelform` also sits in `inherit`.
- What the dynamics chapter adds to rows of other clusters (`sim.run`, `sim.estimate`, `eval.promote`) is written
  in those clusters' registry beads (VERBS-sim, VERBS-eval) and checked against the chapter by N16b-27.

## Acceptance criteria

- [ ] `registries/verbs.toml` holds exactly five entries with `cluster = "fidelity"` (3 `read`, 1 `run`, 1 `admin`),
      each with every field of the entry schema, an example and a task-cluster list; the alias lint finds no second
      spelling.
- [ ] `frob-surface-diff parity --cluster fidelity` exits 0 in process (`frob-mcp` over stdio) and through `frobd
      --mcp`: registry, CLI and MCP each count 5, and every difference list is empty apart from the harness's closed
      exemptions.
- [ ] Every verb has an end-to-end script: `scripts/e2e/verbs_fidelity.sh` runs each verb of the cluster (five), and
      from a verb's first milestone a capability script of its implementing bead exists (`frob-surface-diff
      e2e-coverage`).
- [ ] `frob <verb> --help --json` equals `fixtures/golden/verbs/fidelity/<verb>.help.json` byte for byte for each of
      the five, on two runs, debug and release, Linux and macOS; a planted renamed output property is classified
      `breaking` and refused.
- [ ] A verb not yet implemented answers `FROB-REF-724`, exit 5, with one envelope on both surfaces; an implemented
      verb runs its registry example with the result its implementing bead specifies; a misspelt parameter is
      `FROB-REF-802`, exit 3; a `run` or `mutate` call without a key is `FROB-REF-805`, exit 3.
- [ ] The generated refusal table lists for each verb exactly its chapter codes plus the codes appended for its
      kind and fields (NJL-01), and every code exists in `registries/refusals.toml`.
- [ ] The gate row is recorded with the measured counts by a session that did not write the rows.

## Test plan

### Unit tests

`cargo test -p frob-verbs --test cluster_fidelity` (the real generated table; no mocks; planted registries are TOML
strings labelled `synthetic`):
- `rows_present`: exactly 5 rows in the cluster, with the names of the rows table; a planted extra row or a removed
  row fails.
- `kinds_milestones_tiers`: each row's kind, first milestone and tier equal the rows table (for example
  `fidelity.shadow` is `admin` at M3).
- `schemas_and_scopes`: input schemas are `frob.<name>.in/1`; output schemas and capabilities equal the
  machine-fields table.
- `refusal_lists`: each row's chapter codes equal its block (`fidelity.ledger`: none; `fidelity.energy`: 840 and
  888; `fidelity.coverage`: none; `fidelity.modelform`: 610 and 722; `fidelity.shadow`: 791 and 792); the
  appended codes equal the rule for its kind and fields (NJL-01) (`fidelity.ledger` is paginated and also gets 806,
  807 and, through its `int53` property `limit`, 808; `fidelity.modelform` gets 808 through `sync_wait_ms`); no
  row lists 557 or 830 (N28-REF's `verbs_lists_agree`).
- `ledger_rows_typed`: the generated output schema of `fidelity.ledger` has `kind` (`pair`, `intrinsic` or
  `case`) and `sub_key`; a request with `--tier-pair batch-cpu,batch-cpu --quantity coefficient-lag` parses, and
  its rows are `intrinsic` with `sub_key { coefficient_law, contact_mode }` (on N16bp1-01's `synthetic` fixture);
  `--class quadruped --cases` parses without `--tier-pair` and returns rows of `kind: "case"` (none before BD-272,
  one on N16bp1-01's `synthetic` case), and `--cases --quantity energy-per-metre` is `FROB-REF-802`.
- `modelform_record_fields`: the output schema of `fidelity.modelform` carries every variant field of
  `frob.modelform/1` listed in the block (`base_step_plan` to `order`); `--on-overflow` accepts `refuse` and
  `continue-flagged` and refuses `reset` with 802; `fidelity.coverage` accepts `--battery`.
- `shadow_actions_enumerated`: the input schema of `fidelity.shadow` enumerates `start`, `stop`, `configure`,
  `status` (and `audit` once N16b-20 lands); a planted `pause` is `FROB-REF-802`, exit 3.
- `ledger_paginated`: `fidelity.ledger` has `paginated = true` and its input schema has `cursor`, `limit`,
  `follow`; the other read rows have none of them.
- `mcp_names_one_to_one`: the tool names are the mapped names (`fidelity_ledger`, `fidelity_energy`,
  `fidelity_coverage`, `fidelity_modelform`, `fidelity_shadow`), with no collision against any other cluster.
- `task_clusters_and_examples`: every row sits in a task cluster of at most seven tools or is a base tool; each
  example parses against the generated CLI and its command path is the row's own.
- `kind_behaviour`: a `run` row has the six common properties (`sync_wait_ms`, `idempotency_key`, `budget`,
  `estimate_only`, `on_cancel`, `envelope_scope`), with `idempotency_key` outside the schema's `required` array and
  annotated `x-frob-key: "required"`, and refuses `sync_wait_ms = 30001` with code 802; an `admin` row or action
  without its scope is refused with code 720; a verb whose capability is absent returns code 724, exit 5, one
  envelope.

### End-to-end script and logging

`scripts/e2e/verbs_fidelity.sh` (header: `bead=VERBS-fidelity milestone=M2 labels=tether,verbs,verb:fidelity.*
gates=GATE-TETHER-02 seed=1 needs=cpu,mcp,frobd plants=alias,missing-row,renamed-field`) sources the shared helper
and `scripts/e2e/verbs_lib.sh` and calls `verbs_cluster_conformance fidelity`. Steps: (1) `frob capabilities --json`
and `frob robot-docs --json` list the five verbs with kind and milestone; (2) `frob-surface-diff parity --cluster
fidelity` over stdio and through `frobd --mcp`, every difference list empty; (3) `frob-surface-diff snapshot --check
--cluster fidelity`; (4) each verb's registry example verbatim, expecting a result (exit 0 or 2), a job handle (exit
8, then `frob job wait`) or `FROB-REF-724` exactly when the capability report says the verb is absent; (5) for each
implemented verb the same call as an MCP tool, the two envelopes agreeing on `verb`, `ok`, `exit`, `result`,
`refusal`, `evidence`, `bound`, `warnings` and `next_actions`; (6) per verb one call with a misspelt parameter
(`e2e_expect_refusal FROB-REF-802 3`) and, for `run` and `mutate` verbs, one without a key (`e2e_expect_refusal
FROB-REF-805 3`); (7) one envelope per call and no exit code 1.

Log: `target/e2e-logs/verbs_fidelity/<utc>.ndjson` with a transcript beside it. Each `cmd` line has the exact
command, exit code, wall time and the envelope's `evidence` block; each `assert` has observed and expected values
and the tag `--gate GATE-TETHER-02`; the metrics `registry_verbs`, `cli_verbs` and `mcp_tools` are logged against 5
and the difference counts against 0; refusals are logged with code and first repair; a failure logs the first
differing verb and field; the `summary` line gives assertions passed and failed and the gate touched.

### Known answers and negative controls

- Known answers: the cluster has 5 verbs (3 `read`, 1 `run`, 1 `admin`); the tables above. KA-755 (the exit-code
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

Appendix J (J.1 entry form; J.3 added rows; J.5 task clusters), §16.9, §16.11, §16.13, §32.4, §32.6, §32.12.3,
§32.18, §32.21; GATE-TETHER-02; INV-TETHER-01, -02; KA-755.
