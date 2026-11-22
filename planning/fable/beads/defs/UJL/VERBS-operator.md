---
key: "VERBS-operator"
title: "frob-verbs registry: cluster `operator`, four rows, with parity, e2e and schema-snapshot checks"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "tether", "frob-verbs", "solid", "registry", "verb", "test"]
parent: "WP-06"
deps: ["BD-022", "NJL-01", "NJL-02", "STD-E2E"]
needs: []
related: ["BD-022", "BD-313", "N32a-26", "N00-02", "NJL-03", "NJL-04", "NJL-05"]
gates: ["GATE-TETHER-02"]
plan_refs: ["Appendix J (J.1, J.2, J.3, J.5)", "§32.6", "§32.17", "§32.12.3", "§32.21 (GATE-TETHER-02)"]
---
## Background

FrankenRobotics (a robotics simulation and design system) is reached through the command line `frob`, the daemon
`frobd` and the MCP server `frob-mcp` (the Model Context Protocol, by which an agent harness lists and calls tools).
Every capability is a verb, `cluster.verb`, declared once in `registries/verbs.toml` (crate `frob-verbs`, tier
Solid). The CLI, the daemon protocol, the MCP tools, the schemas, the refusal table and the agent guide are
generated from that file, and gate GATE-TETHER-02 compares the served surfaces with it. The registry holds 243 verbs
in 44 clusters (namespaces).

This bead is the registry of record for the cluster `operator`. The cluster covers the four operator verbs, which
have no cluster prefix: `doctor`, `capabilities`, `robot-docs` and `schema`. They tell an agent what exists and what
works before it spends anything. It has four verbs (4 `read`); the handlers live in `frob-verbs`. The bead carries
every row completely, with the three checks that keep the rows true: the generated CLI commands and MCP tools match
the list; every verb has an end-to-end script; the schema printed by `frob <verb> --help --json` is stable.

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

In: the four registry entries of cluster `operator`; the MCP tool names; the cluster's conformance script; the
help-schema snapshots; each verb's answer before its milestone. Out: behaviour (the implementing beads named below);
the entry schema and lints (NJL-01); the conformance harness (NJL-02); the alias lint (NJL-04); task clusters
(NJL-05, NJL-06); generation of the surfaces (the Tether seed beads).

## Technical approach

**The rows.**

| Verb | Kind | First milestone | Tier | Purpose | Defined in |
|---|---|---|---|---|---|
| `doctor` | read | M0 | S | Operator verb: check toolchains, foundation pins, devices, kernel caches and probe status | §32.6, §32.17 |
| `capabilities` | read | M0 | S | Operator verb: what is implemented and what is not, with resolved modes and ceilings per device | §32.6, §32.17 |
| `robot-docs` | read | M0 | S | Operator verb: the agent guide; `--cluster` prints one task cluster | §32.6, §32.17 |
| `schema` | read | M0 | S | Operator verb: a schema with a minimal valid example | §32.6, §32.17 |

**Contract of each verb.** Evidence labels used below (the `evidence` block of the envelope):
- *operational*: no claim: an operational answer about the session, the store, a job or a device; `evidence.claims`
  is empty.

**`doctor`** (MCP tool `doctor`; owner `frob-verbs`)
- Inputs: none.
- Outputs: toolchains, foundation pins, devices, kernel caches and probe status, each with pass, fail or absent.
  Evidence: *operational*.
- Chapter refusals: none of its own.
- Implemented by: BD-022.

**`capabilities`** (MCP tool `capabilities`; owner `frob-verbs`)
- Inputs: none.
- Outputs: every verb with whether it is implemented in the eight-condition sense, resolved modes and ceilings per
  device. Evidence: *operational*.
- Chapter refusals: none of its own.
- Implemented by: BD-022.

**`robot-docs`** (MCP tool `robot_docs`; owner `frob-verbs`)
- Inputs: optional `--cluster`.
- Outputs: the agent guide: verbs, schemas and worked examples, or one task cluster. Evidence: *operational*.
- Chapter refusals: none of its own.
- Implemented by: BD-022, BD-313, N32a-26.

**`schema`** (MCP tool `schema`; owner `frob-verbs`)
- Inputs: schema identifier or verb name.
- Outputs: the schema with a minimal valid example. Evidence: *operational*.
- Chapter refusals: `FROB-REF-891` (`format-version-unsupported`: Unknown major version, or an unknown required
  field).
- Implemented by: BD-022.

**Machine fields of each row.**

| Verb | Capabilities | Output schema | Task clusters | Example |
|---|---|---|---|---|
| `doctor` | `operator:read` | `frob.doctor.out/1` | `orient` | `frob doctor --json` |
| `capabilities` | `operator:read` | `frob.capabilities.out/1` | `orient` | `frob capabilities --json` |
| `robot-docs` | `operator:read` | `frob.robot-docs.out/1` | `orient` | `frob robot-docs --cluster author --json` |
| `schema` | `operator:read` | `frob.schema.out/1` | `orient` | `frob schema frob.design/1 --json` |

The remaining fields follow by rule: `input_schema` is `frob.<verb name>.in/1`; `refusals` is the chapter list of
the verb's block; `idempotent` is `true` for `read` and `admin` and `false` for `run` and `mutate` (which take an
idempotency key); `cost_model` is `static: …` for `read`, `mutate` and `admin` and the `--estimate` cost model for
`run`; a `run` row carries `wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }`; `owner` is the
crate of the block; `purpose` is the Purpose column.

One entry in full (the others follow field for field):

```toml
[[verb]]
name = "doctor"
cluster = "operator"
kind = "read"
tier = "S"
capabilities = ["operator:read"]
input_schema = "frob.doctor.in/1"
output_schema = "frob.doctor.out/1"
refusals = []
idempotent = true
cost_model = "static: answered from the store or by a small CPU computation; no job"
first_milestone = "M0"
task_clusters = ["orient"]
examples = ["frob doctor --json"]
owner = "frob-verbs"
purpose = "Operator verb: check toolchains, foundation pins, devices, kernel caches and probe status"
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
4. Rows are entered when the cluster is first registered (M0). Until its own milestone a verb answers one envelope
   with `FROB-REF-724`, exit 5, on both surfaces, and `frob capabilities --json` lists it as absent.

**Notes for this cluster.**

- These four are the only registry names without a dot; their registry cluster is `operator` and their CLI path is
  the bare name (`frob doctor`).
- `frob capabilities --json` lists what is implemented in the eight-condition sense (code in its crate, a contract,
  known-answer tests against a pinned oracle, evidence on outputs, a CPU reference path, reachability through `frob
  --json`, cancellation tested, the gate recorded with measured values) and what is not.
- `robot-docs`, the MCP resources under `frob://docs/` and the MCP prompts are generated from the same source, so
  the three cannot drift.

## Acceptance criteria

- [ ] `registries/verbs.toml` holds exactly four entries with `cluster = "operator"` (4 `read`), each with every
      field of the entry schema, an example and a task-cluster list; the alias lint finds no second spelling.
- [ ] `frob-surface-diff parity --cluster operator` exits 0 in process (`frob-mcp` over stdio) and through `frobd
      --mcp`: registry, CLI and MCP each count 4, and every difference list is empty apart from the harness's closed
      exemptions.
- [ ] Every verb has an end-to-end script: `scripts/e2e/verbs_operator.sh` runs each verb of the cluster (four), and
      from a verb's first milestone a capability script of its implementing bead exists (`frob-surface-diff
      e2e-coverage`).
- [ ] `frob <verb> --help --json` equals `fixtures/golden/verbs/operator/<verb>.help.json` byte for byte for each of
      the four, on two runs, debug and release, Linux and macOS; a planted renamed output property is classified
      `breaking` and refused.
- [ ] A verb not yet implemented answers `FROB-REF-724`, exit 5, with one envelope on both surfaces; an implemented
      verb runs its registry example with the result its implementing bead specifies; a misspelt parameter is
      `FROB-REF-802`, exit 3.
- [ ] The generated refusal table lists for each verb exactly its chapter codes plus the codes appended for its
      kind, and every code exists in `registries/refusals.toml`.
- [ ] The gate row is recorded with the measured counts by a session that did not write the rows.

## Test plan

### Unit tests

`cargo test -p frob-verbs --test cluster_operator` (the real generated table; no mocks; planted registries are TOML
strings labelled `synthetic`):
- `rows_present`: exactly 4 rows in the cluster, with the names of the rows table; a planted extra row or a removed
  row fails.
- `kinds_milestones_tiers`: each row's kind, first milestone and tier equal the rows table (for example `schema` is
  `read` at M0).
- `schemas_and_scopes`: input schemas are `frob.<name>.in/1`; output schemas and capabilities equal the
  machine-fields table.
- `refusal_lists`: each row's chapter codes equal its block (`doctor`: none; `capabilities`: none; `robot-docs`:
  none); the appended codes equal the rule for its kind.
- `mcp_names_one_to_one`: the tool names are the mapped names (`doctor`, `capabilities`, `robot_docs`), with no
  collision against any other cluster.
- `task_clusters_and_examples`: every row sits in a task cluster of at most seven tools or is a base tool; each
  example parses against the generated CLI and its command path is the row's own.
- `kind_behaviour`: a verb whose capability is absent returns code 724, exit 5, one envelope.

### End-to-end script and logging

`scripts/e2e/verbs_operator.sh` (header: `bead=VERBS-operator milestone=M0 labels=tether,verbs,verb:operator.*
gates=GATE-TETHER-02 seed=1 needs=cpu,mcp,frobd plants=alias,missing-row,renamed-field`) sources the shared helper
and `scripts/e2e/verbs_lib.sh` and calls `verbs_cluster_conformance operator`. Steps: (1) `frob capabilities --json`
and `frob robot-docs --json` list the four verbs with kind and milestone; (2) `frob-surface-diff parity --cluster
operator` over stdio and through `frobd --mcp`, every difference list empty; (3) `frob-surface-diff snapshot --check
--cluster operator`; (4) each verb's registry example verbatim, expecting a result (exit 0 or 2), a job handle (exit
8, then `frob job wait`) or `FROB-REF-724` exactly when the capability report says the verb is absent; (5) for each
implemented verb the same call as an MCP tool, the two envelopes agreeing on `verb`, `ok`, `exit`, `result`,
`refusal`, `evidence`, `bound`, `warnings` and `next_actions`; (6) per verb one call with a misspelt parameter
(`e2e_expect_refusal FROB-REF-802 3`); (7) one envelope per call and no exit code 1.

Log: `target/e2e-logs/verbs_operator/<utc>.ndjson` with a transcript beside it. Each `cmd` line has the exact
command, exit code, wall time and the envelope's `evidence` block; each `assert` has observed and expected values
and the tag `--gate GATE-TETHER-02`; the metrics `registry_verbs`, `cli_verbs` and `mcp_tools` are logged against 4
and the difference counts against 0; refusals are logged with code and first repair; a failure logs the first
differing verb and field; the `summary` line gives assertions passed and failed and the gate touched.

### Known answers and negative controls

- Known answers: the cluster has 4 verbs (4 `read`); the tables above. KA-755 (the exit-code table on malformed,
  unknown, over-budget, unscoped and conflicting inputs gives the exit and refusal code of the surface's tables;
  exact) is exercised for every verb by steps (6) and (7).
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

- Closing rule: the bead closes at M0, when the rows are registered, the surfaces match and every verb passes its
  example or answers the not-implemented refusal. The per-milestone surface beads (BD-022) re-run this script as
  verbs land; that is how GATE-TETHER-02 is re-measured.
- Scope names are proposals (`<cluster>:<read|run|write|admin>` plus the named scopes of the surface) until the
  capability bead BD-258 fixes the scope registry; a `run` or `write` scope of a cluster implies its `read` scope.
  Inputs and outputs above name contents, not flag spellings, except where a flag is quoted; the implementing bead
  fixes the spellings and the snapshot then pins them (additive changes keep the schema's major version; anything
  else raises it).

## References

Appendix J (J.1 entry form; J.3 added rows; J.5 task clusters), §32.17, §32.4, §32.6, §32.12.3, §32.18, §32.21;
GATE-TETHER-02; INV-TETHER-01, -02; KA-755.
