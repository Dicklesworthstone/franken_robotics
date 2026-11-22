---
key: "VERBS-task"
title: "frob-verbs registry: cluster `task`, eight rows, with parity, e2e and schema-snapshot checks"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "tether", "arena", "frob-verbs", "solid", "registry", "verb", "test"]
parent: "WP-06"
deps: ["BD-022", "NJL-01", "NJL-02", "STD-E2E"]
needs: []
related: ["BD-122", "N24c-01", "BD-123", "BD-195", "N24c-03", "N24c-02", "N24c-22", "N24c-06", "N24a-02", "BD-137", "BD-312", "NJL-03", "NJL-04", "NJL-05"]
gates: ["GATE-TETHER-02"]
plan_refs: ["Appendix J (J.1, J.2, J.3, J.5)", "§24.4", "§24.5", "§24.7", "§24.10", "§24.14", "§24.20", "§32.6", "§32.12.3", "§32.21 (GATE-TETHER-02)"]
---
## Background

FrankenRobotics (a robotics simulation and design system) is reached through the command line `frob`, the daemon
`frobd` and the MCP server `frob-mcp` (the Model Context Protocol, by which an agent harness lists and calls tools).
Every capability is a verb, `cluster.verb`, declared once in `registries/verbs.toml` (crate `frob-verbs`, tier
Solid). The CLI, the daemon protocol, the MCP tools, the schemas, the refusal table and the agent guide are
generated from that file, and gate GATE-TETHER-02 compares the served surfaces with it. The registry holds 243 verbs
in 44 clusters (namespaces).

This bead is the registry of record for the cluster `task`. The cluster covers tasks: creating a task from a
document, showing and listing tasks by content address, the admission check, a verdict on a recorded run, the
settled-pose cache, registered fixtures against their known answers, and the audit of a task before any candidate is
scored. It has eight verbs (4 `read`, 3 `run`, 1 `mutate`); the handlers live in `frob-arena`, `frob-spec`. The bead
carries every row completely, with the three checks that keep the rows true: the generated CLI commands and MCP
tools match the list; every verb has an end-to-end script; the schema printed by `frob <verb> --help --json` is
stable.

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

In: the eight registry entries of cluster `task`; the MCP tool names; the cluster's conformance script; the
help-schema snapshots; each verb's answer before its milestone. Out: behaviour (the implementing beads named below);
the entry schema and lints (NJL-01); the conformance harness (NJL-02); the alias lint (NJL-04); task clusters
(NJL-05, NJL-06); generation of the surfaces (the Tether seed beads).

## Technical approach

**The rows.**

| Verb | Kind | First milestone | Tier | Purpose | Defined in |
|---|---|---|---|---|---|
| `task.check` | read | M1 | S | Units, clocks, windows, reach against horizon, labelled examples, the mutation check | §24.5 |
| `task.fixture` | run | M1 | S | Run a registered fixture's analytic layer or batch against its known answers | §24.10 |
| `task.list` | read | M1 | S | Tasks, by content address | §24.14 |
| `task.new` | mutate | M1 | S | Create a task from a document | §24.4 |
| `task.settle` | run | M1 | S | Build or refresh the settled-pose cache for a model and task | §24.7 |
| `task.show` | read | M1 | S | A task's sections, requirement and examples | §24.4 |
| `task.verdict` | read | M1 | S | Evaluate a task's requirements on a recorded run; returns `frob.verdict/1` | §24.5 |
| `task.audit` | run | M3 | S | Attack a task before any candidate is scored: foil bodies and intent invariants | §24.20 |

**Contract of each verb.** Evidence labels used below (the `evidence` block of the envelope):
- *static*: class Tested, tier `static`, determinism `exact` (class D0); `claims` empty unless a certificate is
  attached; `reality: no-claim`.
- *known answer*: a Known answer where a registered row is recovered within its tolerance; class Tested.
- *pass-through*: passes through the claim product of the stored objects it reads and adds no claim of its own; the
  read itself is `exact`.
- *run*: class Tested at best, by the claim product of the run; the tier of the run, `provisional` on a fast tier
  until promoted to the referee; the determinism class of the placement (D2 unless stated); `reality: no-claim`.

**`task.check`** (MCP tool `task_check`; owner `frob-spec`)
- Inputs: task.
- Outputs: each admission check with its result: units and scales, clocks and reductions, windows, reach against
  horizon, labelled examples, the mutation check. Evidence: *static*.
- Chapter refusals: `FROB-REF-520` (an atom has no scale and is not declared exact); `FROB-REF-523` (a future-time
  operator appears in an online signal); `FROB-REF-525` (a signal is read across clocks without a declared
  reduction, or its channel kind does not admit the reduction); `FROB-REF-526` (the episode horizon is shorter than
  the reach of the verdict); `FROB-REF-528` (an online window exceeds its capacity); `FROB-REF-535` (dimension
  mismatch inside an atom); `FROB-REF-536` (a temporal operator has no finite window); `FROB-REF-538` (a window
  bound is not a multiple of the clock period and no rounding is declared); `FROB-REF-539` (a requirement has no
  labelled examples, or the monitor disagrees with a label).
- Implemented by: BD-122, N24c-01.

**`task.fixture`** (MCP tool `task_fixture`; owner `frob-arena`)
- Inputs: fixture name; layer (`analytic` or `batch`).
- Outputs: every row of the fixture against its known answer, with tolerance and pass or fail. Evidence: *known
  answer*.
- Chapter refusals: `FROB-REF-530` (a run offered as a fixture run does not match the fixture's recorded hash).
- Implemented by: BD-123.

**`task.list`** (MCP tool `task_list`; owner `frob-spec`)
- Inputs: optional filter.
- Outputs: tasks, by content address. Evidence: *pass-through*.
- Chapter refusals: none of its own.
- Implemented by: BD-122.

**`task.new`** (MCP tool `task_new`; owner `frob-spec`)
- Inputs: a `.ftask` document.
- Outputs: `TaskId` (the hash of the canonical form). Evidence: *static*.
- Chapter refusals: `FROB-REF-103` (unknown, misspelt or duplicate field); `FROB-REF-520` (an atom has no scale and
  is not declared exact).
- Implemented by: BD-122.

**`task.settle`** (MCP tool `task_settle`; owner `frob-arena`)
- Inputs: model; task.
- Outputs: the settled-pose cache built or refreshed, with counts and the admission criteria met. Evidence: *run*.
- Chapter refusals: `FROB-REF-529` (no settled-pose cache for this model and task).
- Implemented by: BD-195, N24c-03.

**`task.show`** (MCP tool `task_show`; owner `frob-spec`)
- Inputs: task.
- Outputs: the nine sections, the requirement and its examples. Evidence: *pass-through*.
- Chapter refusals: none of its own.
- Implemented by: BD-122.

**`task.verdict`** (MCP tool `task_verdict`; owner `frob-spec`)
- Inputs: task; a recorded run.
- Outputs: `frob.verdict/1`: the verdict, the binding atom, margins, `sampled` or `interval` labels, `undetermined`
  with a reason where a window is not covered. Evidence: *static*.
- Chapter refusals: `FROB-REF-526` (the episode horizon is shorter than the reach of the verdict); `FROB-REF-888` (a
  metric computed over missing samples).
- Implemented by: BD-122, N24c-02.

**`task.audit`** (MCP tool `task_audit`; owner `frob-arena`)
- Inputs: task; foil bodies; intent invariants.
- Outputs: audit records and findings with proposed clauses, before any candidate is scored. Evidence: *run*.
- Chapter refusals: none of its own.
- Implemented by: N24c-22.

**Machine fields of each row.**

| Verb | Capabilities | Output schema | Task clusters | Example |
|---|---|---|---|---|
| `task.check` | `task:read` | `frob.task.check.out/1` | `tasks` | `frob task check t:kitchen-tray --json` |
| `task.fixture` | `task:run` | `frob.task.fixture.out/1` | `tasks` | `frob task fixture fixture:cart-sill-dock --layer analytic --key fx-1 --json` |
| `task.list` | `task:read` | `frob.task.list.out/1` | `tasks`, `batteries` | `frob task list --json` |
| `task.new` | `task:write` | `frob.task.new.out/1` | `tasks` | `frob task new kitchen-tray.ftask --key tk-1 --json` |
| `task.settle` | `task:run` | `frob.task.settle.out/1` | `tasks` | `frob task settle quad-tray --task t:kitchen-tray --key ts-1 --json` |
| `task.show` | `task:read` | `frob.task.show.out/1` | `tasks` | `frob task show t:kitchen-tray --json` |
| `task.verdict` | `task:read` | `frob.verdict/1` | `tasks` | `frob task verdict t:kitchen-tray --run r:19e2 --json` |
| `task.audit` | `task:run` | `frob.task.audit.out/1` | `audit` | `frob task audit t:kitchen-tray --key ta-1 --json` |

The remaining fields follow by rule: `input_schema` is `frob.<verb name>.in/1`; `refusals` is the chapter list of
the verb's block; `idempotent` is `true` for `read` and `admin` and `false` for `run` and `mutate` (which take an
idempotency key); `cost_model` is `static: …` for `read`, `mutate` and `admin` and the `--estimate` cost model for
`run`; a `run` row carries `wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }`; `owner` is the
crate of the block; `purpose` is the Purpose column.

One entry in full (the others follow field for field):

```toml
[[verb]]
name = "task.fixture"
cluster = "task"
kind = "run"
tier = "S"
capabilities = ["task:run"]
input_schema = "frob.task.fixture.in/1"
output_schema = "frob.task.fixture.out/1"
refusals = ["FROB-REF-530"]
idempotent = false
cost_model = "estimated: `--estimate` returns CPU core-seconds, GPU seconds, memory and disk with a conformal interval"
first_milestone = "M1"
task_clusters = ["tasks"]
examples = ["frob task fixture fixture:cart-sill-dock --layer analytic --key fx-1 --json"]
owner = "frob-arena"
purpose = "Run a registered fixture's analytic layer or batch against its known answers"
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
4. Rows are entered when the cluster is first registered (M1). Until its own milestone a verb answers one envelope
   with `FROB-REF-724`, exit 5, on both surfaces, and `frob capabilities --json` lists it as absent.

**Notes for this cluster.**

- Eight verbs. A task is data and is content-addressed: `TaskId` is the hash of its canonical form.
- `task.check` refuses a task whose atom has no scale, whose signal crosses clocks without a reduction, whose window
  is not a multiple of the requirement period, whose online signal uses a future-time operator, whose horizon is
  shorter than the reach of its verdict, or whose labelled examples are missing or disagree with the monitor.

## Acceptance criteria

- [ ] `registries/verbs.toml` holds exactly eight entries with `cluster = "task"` (4 `read`, 3 `run`, 1 `mutate`),
      each with every field of the entry schema, an example and a task-cluster list; the alias lint finds no second
      spelling.
- [ ] `frob-surface-diff parity --cluster task` exits 0 in process (`frob-mcp` over stdio) and through `frobd
      --mcp`: registry, CLI and MCP each count 8, and every difference list is empty apart from the harness's closed
      exemptions.
- [ ] Every verb has an end-to-end script: `scripts/e2e/verbs_task.sh` runs each verb of the cluster (eight), and
      from a verb's first milestone a capability script of its implementing bead exists (`frob-surface-diff
      e2e-coverage`).
- [ ] `frob <verb> --help --json` equals `fixtures/golden/verbs/task/<verb>.help.json` byte for byte for each of the
      eight, on two runs, debug and release, Linux and macOS; a planted renamed output property is classified `breaking`
      and refused.
- [ ] A verb not yet implemented answers `FROB-REF-724`, exit 5, with one envelope on both surfaces; an implemented
      verb runs its registry example with the result its implementing bead specifies; a misspelt parameter is
      `FROB-REF-802`, exit 3; a `run` or `mutate` call without a key is `FROB-REF-805`, exit 3.
- [ ] The generated refusal table lists for each verb exactly its chapter codes plus the codes appended for its
      kind, and every code exists in `registries/refusals.toml`.
- [ ] The gate row is recorded with the measured counts by a session that did not write the rows.

## Test plan

### Unit tests

`cargo test -p frob-verbs --test cluster_task` (the real generated table; no mocks; planted registries are TOML
strings labelled `synthetic`):
- `rows_present`: exactly 8 rows in the cluster, with the names of the rows table; a planted extra row or a removed
  row fails.
- `kinds_milestones_tiers`: each row's kind, first milestone and tier equal the rows table (for example `task.audit`
  is `run` at M3).
- `schemas_and_scopes`: input schemas are `frob.<name>.in/1`; output schemas and capabilities equal the
  machine-fields table (`task.verdict` returns `frob.verdict/1`).
- `refusal_lists`: each row's chapter codes equal its block (`task.check`: 520, 523, 525, 526, 528, 535, 536, 538,
  539; `task.fixture`: 530; `task.list`: none); the appended codes equal the rule for its kind.
- `mcp_names_one_to_one`: the tool names are the mapped names (`task_check`, `task_fixture`, `task_list`), with no
  collision against any other cluster.
- `task_clusters_and_examples`: every row sits in a task cluster of at most seven tools or is a base tool; each
  example parses against the generated CLI and its command path is the row's own.
- `kind_behaviour`: a `run` row has the five common properties and refuses `sync_wait_ms = 30001` with code 802; a
  `mutate` row has `idempotency_key` and `expected_parent` and refuses a call without the key with code 805; a verb
  whose capability is absent returns code 724, exit 5, one envelope.

### End-to-end script and logging

`scripts/e2e/verbs_task.sh` (header: `bead=VERBS-task milestone=M1 labels=tether,verbs,verb:task.*
gates=GATE-TETHER-02 seed=1 needs=cpu,mcp,frobd plants=alias,missing-row,renamed-field`) sources the shared helper
and `scripts/e2e/verbs_lib.sh` and calls `verbs_cluster_conformance task`. Steps: (1) `frob capabilities --json` and
`frob robot-docs --json` list the eight verbs with kind and milestone; (2) `frob-surface-diff parity --cluster task`
over stdio and through `frobd --mcp`, every difference list empty; (3) `frob-surface-diff snapshot --check --cluster
task`; (4) each verb's registry example verbatim, expecting a result (exit 0 or 2), a job handle (exit 8, then `frob
job wait`) or `FROB-REF-724` exactly when the capability report says the verb is absent; (5) for each implemented
verb the same call as an MCP tool, the two envelopes agreeing on `verb`, `ok`, `exit`, `result`, `refusal`,
`evidence`, `bound`, `warnings` and `next_actions`; (6) per verb one call with a misspelt parameter
(`e2e_expect_refusal FROB-REF-802 3`) and, for `run` and `mutate` verbs, one without a key (`e2e_expect_refusal
FROB-REF-805 3`); (7) one envelope per call and no exit code 1.

Log: `target/e2e-logs/verbs_task/<utc>.ndjson` with a transcript beside it. Each `cmd` line has the exact command,
exit code, wall time and the envelope's `evidence` block; each `assert` has observed and expected values and the tag
`--gate GATE-TETHER-02`; the metrics `registry_verbs`, `cli_verbs` and `mcp_tools` are logged against 8 and the
difference counts against 0; refusals are logged with code and first repair; a failure logs the first differing verb
and field; the `summary` line gives assertions passed and failed and the gate touched.

### Known answers and negative controls

- Known answers: the cluster has 8 verbs (4 `read`, 3 `run`, 1 `mutate`); the tables above. KA-755 (the exit-code
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

- Closing rule: the bead closes at M1, when the rows are registered, the surfaces match and every verb passes its
  example or answers the not-implemented refusal. Verbs of this cluster first answer at M1, M3. The per-milestone
  surface beads (BD-137, BD-312) re-run this script as verbs land; that is how GATE-TETHER-02 is re-measured.
- Scope names are proposals (`<cluster>:<read|run|write|admin>` plus the named scopes of the surface) until the
  capability bead BD-258 fixes the scope registry; a `run` or `write` scope of a cluster implies its `read` scope.
  Inputs and outputs above name contents, not flag spellings, except where a flag is quoted; the implementing bead
  fixes the spellings and the snapshot then pins them (additive changes keep the schema's major version; anything
  else raises it).

## References

Appendix J (J.1 entry form; J.2 inventory rows and J.3 added rows; J.5 task clusters), §24.4, §24.5, §24.7, §24.10,
§24.14, §24.20, §32.4, §32.6, §32.12.3, §32.18, §32.21; GATE-TETHER-02; INV-TETHER-01, -02; KA-755.
