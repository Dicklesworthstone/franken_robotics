---
key: "VERBS-fidelity"
title: "frob-verbs registry: cluster `fidelity`, five rows, with parity, e2e and schema-snapshot checks"
type: "task"
priority: 1
milestone: "M2"
labels: ["m2", "tether", "kinesis", "frob-verbs", "solid", "registry", "verb", "test"]
parent: "WP-43"
deps: ["BD-022", "NJL-01", "NJL-02", "STD-E2E"]
needs: []
related: ["BD-212", "BD-208", "N29a-22", "BD-213", "BD-266", "BD-214", "N16b-27", "BD-222", "BD-312", "NJL-03", "NJL-04", "NJL-05"]
gates: ["GATE-TETHER-02"]
plan_refs: ["Appendix J (J.1, J.2, J.3, J.5)", "§16.9", "§16.11", "§16.13", "§32.6", "§32.12.3", "§32.21 (GATE-TETHER-02)"]
---
## Background

FrankenRobotics (a robotics simulation and design system) is reached through the command line `frob`, the daemon
`frobd` and the MCP server `frob-mcp` (the Model Context Protocol, by which an agent harness lists and calls tools).
Every capability is a verb, `cluster.verb`, declared once in `registries/verbs.toml` (crate `frob-verbs`, tier
Solid). The CLI, the daemon protocol, the MCP tools, the schemas, the refusal table and the agent guide are
generated from that file, and gate GATE-TETHER-02 compares the served surfaces with it. The registry holds 243 verbs
in 44 clusters (namespaces).

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
inventory); one bead for all 243 rows (too large to review).

## Scope

In: the five registry entries of cluster `fidelity`; the MCP tool names; the cluster's conformance script; the
help-schema snapshots; each verb's answer before its milestone. Out: behaviour (the implementing beads named below);
the entry schema and lints (NJL-01); the conformance harness (NJL-02); the alias lint (NJL-04); task clusters
(NJL-05, NJL-06); generation of the surfaces (the Tether seed beads).

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

**`fidelity.ledger`** (MCP tool `fidelity_ledger`; owner `frob-dyn`)
- Inputs: robot class; quantity; tier pair.
- Outputs: ledger rows with frozen envelopes and their freeze date. Evidence: *pass-through*.
- Chapter refusals: none of its own.
- Implemented by: BD-212.

**`fidelity.energy`** (MCP tool `fidelity_energy`; owner `frob-dyn`)
- Inputs: run.
- Outputs: created-energy totals per world and contact mode, with the worlds that crossed the bound. Evidence:
  *pass-through*.
- Chapter refusals: none of its own.
- Implemented by: BD-208, N29a-22.

**`fidelity.coverage`** (MCP tool `fidelity_coverage`; owner `frob-dyn`)
- Inputs: run or class.
- Outputs: both coverage figures of the referee. Evidence: *pass-through*.
- Chapter refusals: none of its own.
- Implemented by: BD-213.

**`fidelity.modelform`** (MCP tool `fidelity_modelform`; owner `frob-dyn`)
- Inputs: a ranking; model-form variants.
- Outputs: `model-robust` or `model-conditional`, with the variant that flipped it. Evidence: *study*.
- Chapter refusals: `FROB-REF-557` (a `model-conditional` result offered as a design comparison without its
  condition, or as input to a build packet).
- Implemented by: BD-266.

**`fidelity.shadow`** (MCP tool `fidelity_shadow`; owner `frob-dyn`)
- Inputs: job; action (`start`, `stop`, `configure`).
- Outputs: the background referee's configuration and its cost. Evidence: *operational*.
- Chapter refusals: none of its own.
- Implemented by: BD-214.

**Machine fields of each row.**

| Verb | Capabilities | Output schema | Task clusters | Example |
|---|---|---|---|---|
| `fidelity.ledger` | `fidelity:read` | `frob.fidelity.ledger.out/1` | `fidelity` | `frob fidelity ledger --class quadruped --json` |
| `fidelity.energy` | `fidelity:read` | `frob.fidelity.energy.out/1` | `fidelity` | `frob fidelity energy r:19e2 --json` |
| `fidelity.coverage` | `fidelity:read` | `frob.fidelity.coverage.out/1` | `fidelity` | `frob fidelity coverage r:19e2 --json` |
| `fidelity.modelform` | `fidelity:run` | `frob.fidelity.modelform.out/1` | `fidelity`, `inherit` | `frob fidelity modelform --study st:9a1 --key mf-1 --json` |
| `fidelity.shadow` | `fidelity:admin` | `frob.fidelity.shadow.out/1` | `fidelity` | `frob fidelity shadow j:77c0 --start --rate 1/64 --json` |

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
refusals = ["FROB-REF-557"]
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

- Five registered verbs. The Kinesis chapter's verb table prints a sixth, `fidelity.rank` (the rank-agreement record
  for a design set and a tier pair), and says the cluster has six. The registry appendix registers that record under
  `eval.rank-agreement` "for rungs and for tier pairs", counts 55 added verbs (which the appendix's own table
  reaches only without `fidelity.rank`), and assigns "its five verbs" to the task cluster `fidelity`. Both names
  cannot be registered for one mechanism. Fail-closed default until the owner decides (the decision is recorded with
  bead N16b-27): `fidelity.rank` is not a verb; `eval.rank-agreement` accepts a tier pair. If the owner chooses
  `fidelity.rank`, it becomes a sixth row of this bead (kind `run`, M3, executor BD-265) and the registry holds 244
  verbs.
- `fidelity.shadow` is of kind `admin`. The inventory gives it M3; its seed bead (BD-214) is at M2.

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
      kind, and every code exists in `registries/refusals.toml`.
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
- `refusal_lists`: each row's chapter codes equal its block (`fidelity.ledger`: none; `fidelity.energy`: none;
  `fidelity.coverage`: none); the appended codes equal the rule for its kind.
- `mcp_names_one_to_one`: the tool names are the mapped names (`fidelity_ledger`, `fidelity_energy`,
  `fidelity_coverage`), with no collision against any other cluster.
- `task_clusters_and_examples`: every row sits in a task cluster of at most seven tools or is a base tool; each
  example parses against the generated CLI and its command path is the row's own.
- `kind_behaviour`: a `run` row has the five common properties and refuses `sync_wait_ms = 30001` with code 802; an
  `admin` row or action without its scope is refused with code 720; a verb whose capability is absent returns code
  724, exit 5, one envelope.

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
- Scope names are proposals (`<cluster>:<read|run|write|admin>` plus the named scopes of the surface) until the
  capability bead BD-258 fixes the scope registry; a `run` or `write` scope of a cluster implies its `read` scope.
  Inputs and outputs above name contents, not flag spellings, except where a flag is quoted; the implementing bead
  fixes the spellings and the snapshot then pins them (additive changes keep the schema's major version; anything
  else raises it).

## References

Appendix J (J.1 entry form; J.3 added rows; J.5 task clusters), §16.9, §16.11, §16.13, §32.4, §32.6, §32.12.3,
§32.18, §32.21; GATE-TETHER-02; INV-TETHER-01, -02; KA-755.
