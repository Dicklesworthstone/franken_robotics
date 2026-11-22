---
key: "VERBS-study"
title: "frob-verbs registry: cluster `study`, five rows, with parity, e2e and schema-snapshot checks"
type: "task"
priority: 1
milestone: "M2"
labels: ["m2", "tether", "arena", "frob-verbs", "solid", "registry", "verb", "test"]
parent: "WP-43"
deps: ["BD-022", "NJL-01", "NJL-02", "STD-E2E"]
needs: []
related: ["BD-262", "N28a-21", "BD-124", "N33b-19", "N32a-13", "BD-222", "BD-312", "NJL-03", "NJL-04", "NJL-05"]
gates: ["GATE-TETHER-02"]
plan_refs: ["Appendix J (J.1, J.2, J.3, J.5)", "§32.6.1", "§24.12", "§28.6", "§32.6", "§32.12.3", "§32.21 (GATE-TETHER-02)"]
---
## Background

FrankenRobotics (a robotics simulation and design system) is reached through the command line `frob`, the daemon
`frobd` and the MCP server `frob-mcp` (the Model Context Protocol, by which an agent harness lists and calls tools).
Every capability is a verb, `cluster.verb`, declared once in `registries/verbs.toml` (crate `frob-verbs`, tier
Solid). The CLI, the daemon protocol, the MCP tools, the schemas, the refusal table and the agent guide are
generated from that file, and gate GATE-TETHER-02 compares the served surfaces with it. The registry holds 243 verbs
in 44 clusters (namespaces).

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
inventory); one bead for all 243 rows (too large to review).

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
- Inputs: study.
- Outputs: per metric `settled`, `tied` or `inconclusive` (`incomparable` or `infeasible` where unscored), with
  intervals and the five pair counts, which sum to the planned pairs. Evidence: *study*.
- Chapter refusals: `FROB-REF-553` (a comparison that mixes study types, or re-reads fixed-n data as a sequential
  study).
- Implemented by: BD-262, N28a-21.

**`study.plan`** (MCP tool `study_plan`; owner `frob-arena`)
- Inputs: manifest: designs, scene, task, seed protocol, budget, backend, one study type.
- Outputs: `frob.plan/1`: the plan address, the cost estimate with its interval, and for fixed-n the pre-analysis of
  reachable verdicts; runs no physics. Evidence: *static*.
- Chapter refusals: `FROB-REF-532` (a fixed-n study has no pre-analysis, or names a verdict it cannot reach);
  `FROB-REF-542` (a paired study under an uncontrolled determinism class, or with arms in different classes);
  `FROB-REF-543` (an `anytime` study on a metric with no declared span); `FROB-REF-552` (a request mixes study
  types); `FROB-REF-555` (a fixed-n study whose declared verdict set contains an unreachable verdict);
  `FROB-REF-556` (a pairing keyed by event order or by a training counter).
- Implemented by: BD-124, N33b-19.

**`study.status`** (MCP tool `study_status`; owner `frob-arena`)
- Inputs: study.
- Outputs: progress of the submitted study with a denominator. Evidence: *operational*.
- Chapter refusals: none of its own.
- Implemented by: BD-124.

**`study.submit`** (MCP tool `study_submit`; owner `frob-arena`)
- Inputs: plan address; idempotency key.
- Outputs: a job; on completion the study record. Evidence: *study*.
- Chapter refusals: `FROB-REF-809` (plan not found, or bound to an identity that has since been superseded).
- Implemented by: BD-124.

**`study.cancel`** (MCP tool `study_cancel`; owner `frob-arena`)
- Inputs: study.
- Outputs: a cancellation receipt naming the last committed pair. Evidence: *operational*.
- Chapter refusals: none of its own.
- Implemented by: N32a-13.

**Machine fields of each row.**

| Verb | Capabilities | Output schema | Task clusters | Example |
|---|---|---|---|---|
| `study.compare` | `study:read` | `frob.comparison/1` | `compare`, `studies` | `frob study compare st:9a1 --json` |
| `study.plan` | `study:write` | `frob.plan/1` | `compare`, `studies` | `frob study plan manifest.json --key pl-1 --json` |
| `study.status` | `study:read` | `frob.study.status.out/1` | `compare`, `studies` | `frob study status st:9a1 --json` |
| `study.submit` | `study:submit` | `frob.study.submit.out/1` | `compare`, `studies` | `frob study submit plan:b3:52e8 --key sub-1 --json` |
| `study.cancel` | `job:cancel` | `frob.study.cancel.out/1` | `studies` | `frob study cancel st:9a1 --json` |

The remaining fields follow by rule: `input_schema` is `frob.<verb name>.in/1`; `refusals` is the chapter list of
the verb's block; `idempotent` is `true` for `read` and `admin` and `false` for `run` and `mutate` (which take an
idempotency key); `cost_model` is `static: …` for `read`, `mutate` and `admin` and the `--estimate` cost model for
`run`; a `run` row carries `wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }`; `owner` is the
crate of the block; `purpose` is the Purpose column.

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
refusals = ["FROB-REF-809"]
idempotent = false
cost_model = "estimated: `--estimate` returns CPU core-seconds, GPU seconds, memory and disk with a conformal interval"
first_milestone = "M2"
task_clusters = ["compare", "studies"]
examples = ["frob study submit plan:b3:52e8 --key sub-1 --json"]
owner = "frob-arena"
purpose = "Submit a planned study under an idempotency key"
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

- `study.plan` binds a manifest to immutable designs, scene, task, seed protocol, budget, backend and one declared
  study type (anytime-valid or fixed-n exact, never both). It returns the plan address (the content address of the
  resolved manifest), the cost estimate with its interval and, for fixed-n, the pre-analysis of reachable verdicts.
  Planning runs no physics and is idempotent by construction.
- `study.submit` takes the plan address and an idempotency key, so the submitted object cannot drift from the
  planned one; a plan bound to a superseded identity is refused (`FROB-REF-809`).
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
      `FROB-REF-802`, exit 3; a `run` or `mutate` call without a key is `FROB-REF-805`, exit 3.
- [ ] The generated refusal table lists for each verb exactly its chapter codes plus the codes appended for its
      kind, and every code exists in `registries/refusals.toml`.
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
- `refusal_lists`: each row's chapter codes equal its block (`study.compare`: 553; `study.plan`: 532, 542, 543, 552,
  555, 556; `study.status`: none); the appended codes equal the rule for its kind.
- `mcp_names_one_to_one`: the tool names are the mapped names (`study_compare`, `study_plan`, `study_status`), with
  no collision against any other cluster.
- `task_clusters_and_examples`: every row sits in a task cluster of at most seven tools or is a base tool; each
  example parses against the generated CLI and its command path is the row's own.
- `kind_behaviour`: a `run` row has the five common properties and refuses `sync_wait_ms = 30001` with code 802; a
  `mutate` row has `idempotency_key` and `expected_parent` and refuses a call without the key with code 805; an
  `admin` row or action without its scope is refused with code 720; a verb whose capability is absent returns code
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
(`e2e_expect_refusal FROB-REF-802 3`) and, for `run` and `mutate` verbs, one without a key (`e2e_expect_refusal
FROB-REF-805 3`); (7) one envelope per call and no exit code 1.

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
- Closing rule: the bead closes at M2, when the rows are registered, the surfaces match and every verb passes its
  example or answers the not-implemented refusal. Verbs of this cluster first answer at M2, M3. The per-milestone
  surface beads (BD-222, BD-312) re-run this script as verbs land; that is how GATE-TETHER-02 is re-measured.
- Scope names are proposals (`<cluster>:<read|run|write|admin>` plus the named scopes of the surface) until the
  capability bead BD-258 fixes the scope registry; a `run` or `write` scope of a cluster implies its `read` scope.
  Inputs and outputs above name contents, not flag spellings, except where a flag is quoted; the implementing bead
  fixes the spellings and the snapshot then pins them (additive changes keep the schema's major version; anything
  else raises it).

## References

Appendix J (J.1 entry form; J.2 inventory rows and J.3 added rows; J.5 task clusters), §32.6.1, §24.12, §28.6,
§32.4, §32.6, §32.12.3, §32.18, §32.21; GATE-TETHER-02; INV-TETHER-01, -02; KA-755.
