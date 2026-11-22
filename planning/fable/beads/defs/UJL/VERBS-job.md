---
key: "VERBS-job"
title: "frob-verbs registry: cluster `job`, five rows, with parity, e2e and schema-snapshot checks"
type: "task"
priority: 1
milestone: "M2"
labels: ["m2", "tether", "frob-verbs", "solid", "registry", "verb", "test"]
parent: "WP-43"
deps: ["BD-022", "NJL-01", "NJL-02", "STD-E2E"]
needs: []
related: ["BD-210", "N29a-23", "N32a-16", "BD-211", "BD-222", "NJL-03", "NJL-04", "NJL-05"]
gates: ["GATE-TETHER-02"]
plan_refs: ["Appendix J (J.1, J.2, J.3, J.5)", "§32.7", "§32.10", "§32.12.5", "§32.12.6", "§32.6", "§32.12.3", "§32.21 (GATE-TETHER-02)"]
---
## Background

FrankenRobotics (a robotics simulation and design system) is reached through the command line `frob`, the daemon
`frobd` and the MCP server `frob-mcp` (the Model Context Protocol, by which an agent harness lists and calls tools).
Every capability is a verb, `cluster.verb`, declared once in `registries/verbs.toml` (crate `frob-verbs`, tier
Solid). The CLI, the daemon protocol, the MCP tools, the schemas, the refusal table and the agent guide are
generated from that file, and gate GATE-TETHER-02 compares the served surfaces with it. The registry holds 243 verbs
in 44 clusters (namespaces).

This bead is the registry of record for the cluster `job`. The cluster covers the job ledger of the daemon: listing
jobs, their state, a bounded wait, recovery of a job by its idempotency key after a transport failure, and
cancellation that drains. It has five verbs (4 `read`, 1 `admin`); the handlers live in `frob-verbs`. The bead
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

In: the five registry entries of cluster `job`; the MCP tool names; the cluster's conformance script; the
help-schema snapshots; each verb's answer before its milestone. Out: behaviour (the implementing beads named below);
the entry schema and lints (NJL-01); the conformance harness (NJL-02); the alias lint (NJL-04); task clusters
(NJL-05, NJL-06); generation of the surfaces (the Tether seed beads).

## Technical approach

**The rows.**

| Verb | Kind | First milestone | Tier | Purpose | Defined in |
|---|---|---|---|---|---|
| `job.cancel` | admin | M2 | S | Ask the daemon to cancel; it drains at the kernel-batch boundary and names the last committed tick | §32.10 |
| `job.find` | read | M2 | S | Find a job by idempotency key after a transport failure | §32.7 |
| `job.list` | read | M2 | S | Jobs in the caller's namespace with state and progress | §32.10 |
| `job.status` | read | M2 | S | Job state | §32.10, §32.12.5 |
| `job.wait` | read | M2 | S | A bounded wait on a job | §32.10, §32.12.5 |

**Contract of each verb.** Evidence labels used below (the `evidence` block of the envelope):
- *operational*: no claim: an operational answer about the session, the store, a job or a device; `evidence.claims`
  is empty.

**`job.cancel`** (MCP tool `job_cancel`; owner `frob-verbs`)
- Inputs: job.
- Outputs: a cancellation receipt: outstanding device work at the request, the observed latency, the last committed
  tick per world. Evidence: *operational*.
- Chapter refusals: none of its own.
- Implemented by: BD-210, N29a-23.

**`job.find`** (MCP tool `job_find`; owner `frob-verbs`)
- Inputs: verb and idempotency key.
- Outputs: the job bound to that key, or `none`; exactly one job and one charge exist or none do. Evidence:
  *operational*.
- Chapter refusals: none of its own.
- Implemented by: BD-210, N29a-23.

**`job.list`** (MCP tool `job_list`; owner `frob-verbs`)
- Inputs: optional state filter; cursor.
- Outputs: jobs in the caller's namespace with state and progress, paged against a committed frontier. Evidence:
  *operational*.
- Chapter refusals: none of its own.
- Implemented by: BD-210, N29a-23.

**`job.status`** (MCP tool `job_status`; owner `frob-verbs`)
- Inputs: job.
- Outputs: state, progress with a denominator, the estimate of time remaining with its interval, the last committed
  tick. Evidence: *operational*.
- Chapter refusals: none of its own.
- Implemented by: BD-210, N29a-23.

**`job.wait`** (MCP tool `job_wait`; owner `frob-verbs`)
- Inputs: job; `sync_wait_ms` (0 to 30,000).
- Outputs: the job's final envelope if it finishes inside the wait, otherwise the handle again with exit 8.
  Evidence: *operational*.
- Chapter refusals: none of its own.
- Implemented by: BD-210, N29a-23, N32a-16.

**Machine fields of each row.**

| Verb | Capabilities | Output schema | Task clusters | Example |
|---|---|---|---|---|
| `job.cancel` | `job:cancel` | `frob.job.cancel.out/1` | `base` | `frob job cancel j:77c0 --json` |
| `job.find` | `job:read` | `frob.job.find.out/1` | `operate` | `frob job find --verb nemesis.search --key nem-1 --json` |
| `job.list` | `job:read` | `frob.job.list.out/1` | `operate`, `fleet` | `frob job list --json` |
| `job.status` | `job:read` | `frob.job.status.out/1` | `operate` | `frob job status j:77c0 --json` |
| `job.wait` | `job:read` | `frob.job.wait.out/1` | `base` | `frob job wait j:77c0 --sync-wait-ms 30000 --json` |

The remaining fields follow by rule: `input_schema` is `frob.<verb name>.in/1`; `refusals` is the chapter list of
the verb's block; `idempotent` is `true` for `read` and `admin` and `false` for `run` and `mutate` (which take an
idempotency key); `cost_model` is `static: …` for `read`, `mutate` and `admin` and the `--estimate` cost model for
`run`; a `run` row carries `wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }`; `owner` is the
crate of the block; `purpose` is the Purpose column.

One entry in full (the others follow field for field):

```toml
[[verb]]
name = "job.cancel"
cluster = "job"
kind = "admin"
tier = "S"
capabilities = ["job:cancel"]
input_schema = "frob.job.cancel.in/1"
output_schema = "frob.job.cancel.out/1"
refusals = []
idempotent = true
cost_model = "static: one daemon request"
first_milestone = "M2"
task_clusters = ["base"]
examples = ["frob job cancel j:77c0 --json"]
owner = "frob-verbs"
purpose = "Ask the daemon to cancel; it drains at the kernel-batch boundary and names the last committed tick"
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

- `job.wait` and `job.cancel` are two of the three base tools that every MCP tool list carries (the third is
  `session.focus`); they are not counted against the seven tools of a task cluster.
- `job.wait` takes `sync_wait_ms` (0 to 30,000) like every `run` tool, so an agent can hold a job with a sequence of
  bounded waits; it is itself of kind `read`.
- `job.cancel` asks the daemon to cancel; the daemon drains at the kernel-batch boundary and the receipt names the
  last committed tick. A wire-cancelled wait is not a cancelled job (INV-TETHER-11).
- `job.find` is the recovery path of the transport failure class: after a dropped connection the caller never
  resubmits blindly; one job and one budget charge exist for (principal, verb, key), or none do (INV-TETHER-05).

## Acceptance criteria

- [ ] `registries/verbs.toml` holds exactly five entries with `cluster = "job"` (4 `read`, 1 `admin`), each with
      every field of the entry schema, an example and a task-cluster list; the alias lint finds no second spelling.
- [ ] `frob-surface-diff parity --cluster job` exits 0 in process (`frob-mcp` over stdio) and through `frobd --mcp`:
      registry, CLI and MCP each count 5, and every difference list is empty apart from the harness's closed exemptions.
- [ ] Every verb has an end-to-end script: `scripts/e2e/verbs_job.sh` runs each verb of the cluster (five), and from
      a verb's first milestone a capability script of its implementing bead exists (`frob-surface-diff e2e-coverage`).
- [ ] `frob <verb> --help --json` equals `fixtures/golden/verbs/job/<verb>.help.json` byte for byte for each of the
      five, on two runs, debug and release, Linux and macOS; a planted renamed output property is classified `breaking`
      and refused.
- [ ] A verb not yet implemented answers `FROB-REF-724`, exit 5, with one envelope on both surfaces; an implemented
      verb runs its registry example with the result its implementing bead specifies; a misspelt parameter is
      `FROB-REF-802`, exit 3.
- [ ] The generated refusal table lists for each verb exactly its chapter codes plus the codes appended for its
      kind, and every code exists in `registries/refusals.toml`.
- [ ] The gate row is recorded with the measured counts by a session that did not write the rows.

## Test plan

### Unit tests

`cargo test -p frob-verbs --test cluster_job` (the real generated table; no mocks; planted registries are TOML
strings labelled `synthetic`):
- `rows_present`: exactly 5 rows in the cluster, with the names of the rows table; a planted extra row or a removed
  row fails.
- `kinds_milestones_tiers`: each row's kind, first milestone and tier equal the rows table (for example `job.wait`
  is `read` at M2).
- `schemas_and_scopes`: input schemas are `frob.<name>.in/1`; output schemas and capabilities equal the
  machine-fields table.
- `refusal_lists`: each row's chapter codes equal its block (`job.cancel`: none; `job.find`: none; `job.list`:
  none); the appended codes equal the rule for its kind.
- `mcp_names_one_to_one`: the tool names are the mapped names (`job_cancel`, `job_find`, `job_list`), with no
  collision against any other cluster.
- `task_clusters_and_examples`: every row sits in a task cluster of at most seven tools or is a base tool; each
  example parses against the generated CLI and its command path is the row's own.
- `kind_behaviour`: an `admin` row or action without its scope is refused with code 720; a verb whose capability is
  absent returns code 724, exit 5, one envelope.

### End-to-end script and logging

`scripts/e2e/verbs_job.sh` (header: `bead=VERBS-job milestone=M2 labels=tether,verbs,verb:job.* gates=GATE-TETHER-02
seed=1 needs=cpu,mcp,frobd plants=alias,missing-row,renamed-field`) sources the shared helper and
`scripts/e2e/verbs_lib.sh` and calls `verbs_cluster_conformance job`. Steps: (1) `frob capabilities --json` and
`frob robot-docs --json` list the five verbs with kind and milestone; (2) `frob-surface-diff parity --cluster job`
over stdio and through `frobd --mcp`, every difference list empty; (3) `frob-surface-diff snapshot --check --cluster
job`; (4) each verb's registry example verbatim, expecting a result (exit 0 or 2), a job handle (exit 8, then `frob
job wait`) or `FROB-REF-724` exactly when the capability report says the verb is absent; (5) for each implemented
verb the same call as an MCP tool, the two envelopes agreeing on `verb`, `ok`, `exit`, `result`, `refusal`,
`evidence`, `bound`, `warnings` and `next_actions`; (6) per verb one call with a misspelt parameter
(`e2e_expect_refusal FROB-REF-802 3`); (7) one envelope per call and no exit code 1.

Log: `target/e2e-logs/verbs_job/<utc>.ndjson` with a transcript beside it. Each `cmd` line has the exact command,
exit code, wall time and the envelope's `evidence` block; each `assert` has observed and expected values and the tag
`--gate GATE-TETHER-02`; the metrics `registry_verbs`, `cli_verbs` and `mcp_tools` are logged against 5 and the
difference counts against 0; refusals are logged with code and first repair; a failure logs the first differing verb
and field; the `summary` line gives assertions passed and failed and the gate touched.

### Known answers and negative controls

- Known answers: the cluster has 5 verbs (4 `read`, 1 `admin`); the tables above. KA-755 (the exit-code table on
  malformed, unknown, over-budget, unscoped and conflicting inputs gives the exit and refusal code of the surface's
  tables; exact) is exercised for every verb by steps (6) and (7).
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

- The cluster first answers at M2 with the job engine. Before the daemon exists, CPU-tier verbs run in process and
  never return a handle, so the rows answer the not-implemented refusal.
- Closing rule: the bead closes at M2, when the rows are registered, the surfaces match and every verb passes its
  example or answers the not-implemented refusal. The per-milestone surface beads (BD-222) re-run this script as
  verbs land; that is how GATE-TETHER-02 is re-measured.
- Scope names are proposals (`<cluster>:<read|run|write|admin>` plus the named scopes of the surface) until the
  capability bead BD-258 fixes the scope registry; a `run` or `write` scope of a cluster implies its `read` scope.
  Inputs and outputs above name contents, not flag spellings, except where a flag is quoted; the implementing bead
  fixes the spellings and the snapshot then pins them (additive changes keep the schema's major version; anything
  else raises it).

## References

Appendix J (J.1 entry form; J.2 inventory rows and J.3 added rows; J.5 task clusters), §32.7, §32.10, §32.12.5,
§32.12.6, §32.4, §32.6, §32.12.3, §32.18, §32.21; GATE-TETHER-02; INV-TETHER-01, -02; KA-755.
