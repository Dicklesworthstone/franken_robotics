---
key: "VERBS-train"
title: "frob-verbs registry: cluster `train`, seven rows, with parity, e2e and schema-snapshot checks"
type: "task"
priority: 1
milestone: "M2"
labels: ["m2", "tether", "cortex", "frob-verbs", "solid", "registry", "verb", "test"]
parent: "WP-43"
deps: ["BD-022", "NJL-01", "NJL-02", "STD-E2E"]
needs: []
related: ["BD-323", "BD-198", "N23a-13", "BD-320", "BD-322", "N23a-14", "BD-222", "BD-376", "NJL-03", "NJL-04", "NJL-05"]
gates: ["GATE-TETHER-02"]
plan_refs: ["Appendix J (J.1, J.2, J.3, J.5)", "§23.6", "§23.7", "§23.10", "§23.11", "§23.12", "§32.6", "§32.12.3", "§32.21 (GATE-TETHER-02)"]
---
## Background

FrankenRobotics (a robotics simulation and design system) is reached through the command line `frob`, the daemon
`frobd` and the MCP server `frob-mcp` (the Model Context Protocol, by which an agent harness lists and calls tools).
Every capability is a verb, `cluster.verb`, declared once in `registries/verbs.toml` (crate `frob-verbs`, tier
Solid). The CLI, the daemon protocol, the MCP tools, the schemas, the refusal table and the agent guide are
generated from that file, and gate GATE-TETHER-02 compares the served surfaces with it. The registry holds 243 verbs
in 44 clusters (namespaces).

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
inventory); one bead for all 243 rows (too large to review).

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
- Inputs: teacher (baseline, specialist or sampling controller); student network; task.
- Outputs: a student policy with its distillation record. Evidence: *screening*.
- Chapter refusals: `FROB-REF-830` (a `model-conditional` result offered for promotion, build or prototype budget).
- Implemented by: BD-323.

**`train.export`** (MCP tool `train_export`; owner `frob-learn`)
- Inputs: training run or policy.
- Outputs: the policy with its `frob.policy/1` manifest and mapping table. Evidence: *pass-through*.
- Chapter refusals: `FROB-REF-733` (the licence set of the artifact forbids this export); `FROB-REF-828` (a payload
  that carries code, or a weight format outside the allowlist).
- Implemented by: BD-198, N23a-13.

**`train.family`** (MCP tool `train_family`; owner `frob-learn`)
- Inputs: morphology class; generator and split; envelope.
- Outputs: a pretrained family policy, after the compute-envelope check. Evidence: *screening*.
- Chapter refusals: `FROB-REF-534` (a family policy applied outside its class or its pretraining distribution);
  `FROB-REF-541` (outside generator support: A family-policy score is requested for a design outside its training
  generator's support); `FROB-REF-704` (a campaign-class job without an approved compute envelope).
- Implemented by: BD-320.

**`train.inherit`** (MCP tool `train_inherit`; owner `frob-learn`)
- Inputs: donor policy; child design; the typed edit that joins them.
- Outputs: the child's policy started from the donor, with the matched transfer baseline declared. Evidence:
  *screening*.
- Chapter refusals: `FROB-REF-531` (a policy's mapping table does not match the design, or cannot be built for an
  edit).
- Implemented by: BD-322.

**`train.start`** (MCP tool `train_start`; owner `frob-learn`)
- Inputs: design; task; algorithm; network; devices; tying; profile.
- Outputs: a training job. Evidence: *run*.
- Chapter refusals: `FROB-REF-445` (a training or optimization run is requested on a mission with a standing alias
  witness for its interface); `FROB-REF-524` (the actor schema reads a `critic-only` or `evaluation-only` channel);
  `FROB-REF-527` (the action law at learning differs from the law at collection); `FROB-REF-547` (a tensor view the
  receiver cannot represent, under a zero-copy contract).
- Implemented by: BD-198, N23a-14.

**`train.status`** (MCP tool `train_status`; owner `frob-learn`)
- Inputs: training run.
- Outputs: return, steps, learner share and flags; `crossing` distinguished from `attained`. Evidence:
  *operational*.
- Chapter refusals: none of its own.
- Implemented by: BD-198.

**`train.stop`** (MCP tool `train_stop`; owner `frob-learn`)
- Inputs: training run.
- Outputs: a stop receipt at a commit boundary. Evidence: *operational*.
- Chapter refusals: none of its own.
- Implemented by: BD-198.

**Machine fields of each row.**

| Verb | Capabilities | Output schema | Task clusters | Example |
|---|---|---|---|---|
| `train.distill` | `train:run` | `frob.train.distill.out/1` | `inherit` | `frob train distill --teacher c:2be8 --task stairs-0.4 --key td-1 --json` |
| `train.export` | `train:read`, `export:policy` | `frob.train.export.out/1` | `learn` | `frob train export tr:41 --out policies/ --json` |
| `train.family` | `train:run` | `frob.train.family.out/1` | `inherit` | `frob train family --generator gen:quad-1 --split dev --key tf-1 --json` |
| `train.inherit` | `train:run` | `frob.train.inherit.out/1` | `inherit` | `frob train inherit --donor p:6f1a --design quad-a --key ti-1 --json` |
| `train.start` | `train:run` | `frob.train.start.out/1` | `learn` | `frob train start quad-a --task flat-velocity --algo ppo --devices cuda:0 --key tr-1 --json` |
| `train.status` | `train:read` | `frob.train.status.out/1` | `learn` | `frob train status tr:41 --json` |
| `train.stop` | `train:admin` | `frob.train.stop.out/1` | `learn` | `frob train stop tr:41 --json` |

The remaining fields follow by rule: `input_schema` is `frob.<verb name>.in/1`; `refusals` is the chapter list of
the verb's block; `idempotent` is `true` for `read` and `admin` and `false` for `run` and `mutate` (which take an
idempotency key); `cost_model` is `static: …` for `read`, `mutate` and `admin` and the `--estimate` cost model for
`run`; a `run` row carries `wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }`; `owner` is the
crate of the block; `purpose` is the Purpose column.

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
refusals = ["FROB-REF-830"]
idempotent = false
cost_model = "estimated: `--estimate` returns CPU core-seconds, GPU seconds, memory and disk with a conformal interval"
first_milestone = "M4"
task_clusters = ["inherit"]
examples = ["frob train distill --teacher c:2be8 --task stairs-0.4 --key td-1 --json"]
owner = "frob-learn"
purpose = "Distill a teacher into a student policy"
wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }
```

**Conventions the rows follow.**

1. The MCP tool name is the verb name with `.` and `-` replaced by `_`. The output on the wire is one envelope
   `frob.envelope/1` with `result` specialized to the output schema; its `evidence` block is a claim product (the
   weakest class, tier and determinism class among the inputs).
2. The generator adds the common input properties of the kind (`run`: `sync_wait_ms` from 0 to 30,000 with MCP
   default 5,000, `idempotency_key`, `budget`, `estimate_only`, `on_cancel`; `mutate`: `idempotency_key`,
   `expected_parent`) and appends the surface-wide refusals: `FROB-REF-720` (scope missing), 723 (token invalid),
   724 (not implemented in this build) and 802 (schema failure, including an unknown parameter) for every verb; 701
   to 704 (budget and admission), 803 (key reused with different content) and 805 (missing key) for `run`; 725, 803,
   804 (stale parent) and 805 for `mutate`; 806 and 807 for paginated reads; 808 for an out-of-range integer.
3. Exit codes: 0 done; 2 done with findings; 3 refused input (1xx to 5xx, and 8xx other than conflicts); 4 budget
   (70x); 5 capability (72x, 73x); 6 device (6xx); 7 conflict (803, 804, 809); 8 accepted and still running; 9
   indeterminate; never 1 on purpose.
4. Rows are entered when the cluster is first registered (M2). Until its own milestone a verb answers one envelope
   with `FROB-REF-724`, exit 5, on both surfaces, and `frob capabilities --json` lists it as absent.

**Notes for this cluster.**

- `train.stop` stops at a commit boundary; it is an `admin` verb.
- A number from a fast-training (D4) run may not be cited as evidence without re-evaluation on a stricter profile.

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
      kind, and every code exists in `registries/refusals.toml`.
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
- `refusal_lists`: each row's chapter codes equal its block (`train.distill`: 830; `train.export`: 733, 828;
  `train.family`: 534, 541, 704); the appended codes equal the rule for its kind.
- `mcp_names_one_to_one`: the tool names are the mapped names (`train_distill`, `train_export`, `train_family`),
  with no collision against any other cluster.
- `task_clusters_and_examples`: every row sits in a task cluster of at most seven tools or is a base tool; each
  example parses against the generated CLI and its command path is the row's own.
- `kind_behaviour`: a `run` row has the five common properties and refuses `sync_wait_ms = 30001` with code 802; an
  `admin` row or action without its scope is refused with code 720; a verb whose capability is absent returns code
  724, exit 5, one envelope.

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
- Scope names are proposals (`<cluster>:<read|run|write|admin>` plus the named scopes of the surface) until the
  capability bead BD-258 fixes the scope registry; a `run` or `write` scope of a cluster implies its `read` scope.
  Inputs and outputs above name contents, not flag spellings, except where a flag is quoted; the implementing bead
  fixes the spellings and the snapshot then pins them (additive changes keep the schema's major version; anything
  else raises it).

## References

Appendix J (J.1 entry form; J.2 inventory rows; J.5 task clusters), §23.6, §23.7, §23.10, §23.11, §23.12, §32.4,
§32.6, §32.12.3, §32.18, §32.21; GATE-TETHER-02; INV-TETHER-01, -02; KA-755.
