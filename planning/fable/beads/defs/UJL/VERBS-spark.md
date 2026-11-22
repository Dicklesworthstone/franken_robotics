---
key: "VERBS-spark"
title: "frob-verbs registry: cluster `spark`, seven rows, with parity, e2e and schema-snapshot checks"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "tether", "spark", "frob-verbs", "solid", "registry", "verb", "test"]
parent: "WP-06"
deps: ["BD-022", "NJL-01", "NJL-02", "STD-E2E"]
needs: []
related: ["BD-173", "BD-176", "N21b-32", "N21c-04", "BD-029", "BD-178", "BD-017", "BD-222", "NJL-03", "NJL-04", "NJL-05"]
gates: ["GATE-TETHER-02"]
plan_refs: ["Appendix J (J.1, J.2, J.3, J.5)", "§21.8", "§21.10", "§21.12", "§21.13", "§21.15", "§21.16", "§32.6", "§32.12.3", "§32.21 (GATE-TETHER-02)"]
---
## Background

FrankenRobotics (a robotics simulation and design system) is reached through the command line `frob`, the daemon
`frobd` and the MCP server `frob-mcp` (the Model Context Protocol, by which an agent harness lists and calls tools).
Every capability is a verb, `cluster.verb`, declared once in `registries/verbs.toml` (crate `frob-verbs`, tier
Solid). The CLI, the daemon protocol, the MCP tools, the schemas, the refusal table and the agent guide are
generated from that file, and gate GATE-TETHER-02 compares the served surfaces with it. The registry holds 243 verbs
in 44 clusters (namespaces).

This bead is the registry of record for the cluster `spark`. The cluster covers devices and kernels: the devices
with their capabilities and fingerprints, arithmetic probes, the CPU-versus-device check, the hardware fingerprint,
the placement plan, precompilation of a shape class's kernels and the binary kernel cache. It has seven verbs (3
`read`, 4 `run`); the handlers live in `frob-cli`, `frob-kernels`. The bead carries every row completely, with the
three checks that keep the rows true: the generated CLI commands and MCP tools match the list; every verb has an
end-to-end script; the schema printed by `frob <verb> --help --json` is stable.

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

In: the seven registry entries of cluster `spark`; the MCP tool names; the cluster's conformance script; the
help-schema snapshots; each verb's answer before its milestone. Out: behaviour (the implementing beads named below);
the entry schema and lints (NJL-01); the conformance harness (NJL-02); the alias lint (NJL-04); task clusters
(NJL-05, NJL-06); generation of the surfaces (the Tether seed beads).

## Technical approach

**The rows.**

| Verb | Kind | First milestone | Tier | Purpose | Defined in |
|---|---|---|---|---|---|
| `spark.cache` | read, admin | M2 | S | The binary kernel cache: contents, hits, eviction | §21.10 |
| `spark.check` | run | M0 | S | The CPU-versus-device check per kernel, with its currency | §21.13 |
| `spark.devices` | read | M0 | S | Devices with capabilities, ceilings and current fingerprint digests | §21.16 |
| `spark.fingerprint` | run | M0 | S | Measure and write the hardware fingerprint record | §21.16 |
| `spark.place` | read | M2 | S | The placement plan and the residency map, before anything runs | §21.12 |
| `spark.precompile` | run | M2 | S | Compile or load the kernel set of a shape class | §21.10 |
| `spark.probe` | run | M0 | S | Arithmetic probes and the boundary corpus on a device | §21.8, §21.15 |

**Contract of each verb.** Evidence labels used below (the `evidence` block of the envelope):
- *operational*: no claim: an operational answer about the session, the store, a job or a device; `evidence.claims`
  is empty.
- *fidelity*: a Fidelity claim (the faster tier tracks the more trusted tier within a frozen envelope); class
  Tested; directional.
- *known answer*: a Known answer where a registered row is recovered within its tolerance; class Tested.

**`spark.cache`** (MCP tool `spark_cache`; owner `frob-kernels`)
- Inputs: device; action `show` (read) or `evict` (admin).
- Outputs: the binary kernel cache: contents, hits, integrity; eviction receipts. Evidence: *operational*.
- Chapter refusals: `FROB-REF-612` (kernel cache entry corrupt and recompilation unavailable).
- Implemented by: BD-173.

**`spark.check`** (MCP tool `spark_check`; owner `frob-kernels`)
- Inputs: kernel(s); devices.
- Outputs: the CPU-versus-device check per kernel: ulp histograms, worst case, class attained, and its currency.
  Evidence: *fidelity*.
- Chapter refusals: `FROB-REF-603` (kernel compile failure; compiler diagnostics verbatim); `FROB-REF-605` (the
  CPU-versus-device check is not current for a kernel in the plan).
- Implemented by: BD-176, N21b-32.

**`spark.devices`** (MCP tool `spark_devices`; owner `frob-cli`)
- Inputs: none.
- Outputs: devices with capabilities, ceilings, state and current fingerprint digests. Evidence: *operational*.
- Chapter refusals: none of its own.
- Implemented by: N21c-04.

**`spark.fingerprint`** (MCP tool `spark_fingerprint`; owner `frob-kernels`)
- Inputs: device.
- Outputs: the hardware fingerprint record `frob.fingerprint/1`. Evidence: *operational*.
- Chapter refusals: `FROB-REF-606` (fingerprint missing, or changed since admission).
- Implemented by: BD-029.

**`spark.place`** (MCP tool `spark_place`; owner `frob-kernels`)
- Inputs: step plan or shape class; request mode (`strict`, `prefer`, `mixed-allowed`).
- Outputs: `frob.placement/1`: the placement plan and the residency map, before anything runs. Evidence:
  *operational*.
- Chapter refusals: `FROB-REF-602` (the memory estimate exceeds the device budget at admission); `FROB-REF-604`
  (strict backend: a stage has no kernel on the requested backend); `FROB-REF-605` (the CPU-versus-device check is
  not current for a kernel in the plan); `FROB-REF-616` (the device lacks an arithmetic feature the kernel declares
  (for example 64-bit integers)).
- Implemented by: BD-178.

**`spark.precompile`** (MCP tool `spark_precompile`; owner `frob-kernels`)
- Inputs: shape class; devices.
- Outputs: the kernel set compiled or loaded, with cache receipts. Evidence: *operational*.
- Chapter refusals: `FROB-REF-603` (kernel compile failure; compiler diagnostics verbatim).
- Implemented by: BD-173.

**`spark.probe`** (MCP tool `spark_probe`; owner `frob-kernels`)
- Inputs: device.
- Outputs: arithmetic probe results and the boundary corpus on the device; the attainable determinism class.
  Evidence: *known answer*.
- Chapter refusals: `FROB-REF-609` (a double-float canary failed: Run without double-float kernels, labeled; file
  the compiler evidence); `FROB-REF-616` (the device lacks an arithmetic feature the kernel declares (for example
  64-bit integers)).
- Implemented by: BD-017.

**Machine fields of each row.**

| Verb | Capabilities | Output schema | Task clusters | Example |
|---|---|---|---|---|
| `spark.cache` | `spark:read` | `frob.spark.cache.out/1` | `devices`, `housekeeping` | `frob spark cache --device cuda:0 --json` |
| `spark.check` | `spark:run` | `frob.spark.check.out/1` | `devices` | `frob spark check K-cay-step --devices cuda:0,metal --key ck-1 --json` |
| `spark.devices` | `spark:read` | `frob.spark.devices.out/1` | `operate`, `devices`, `fleet` | `frob spark devices --json` |
| `spark.fingerprint` | `spark:run` | `frob.fingerprint/1` | `devices` | `frob spark fingerprint --device metal --key fp-1 --json` |
| `spark.place` | `spark:read` | `frob.placement/1` | `devices`, `fleet` | `frob spark place --shape quad-12 --mode strict --json` |
| `spark.precompile` | `spark:run` | `frob.spark.precompile.out/1` | `devices` | `frob spark precompile --shape quad-12 --devices cuda:0 --key pc-1 --json` |
| `spark.probe` | `spark:run` | `frob.spark.probe.out/1` | `devices` | `frob spark probe --device cuda:0 --key pb-1 --json` |

The remaining fields follow by rule: `input_schema` is `frob.<verb name>.in/1`; `refusals` is the chapter list of
the verb's block; `idempotent` is `true` for `read` and `admin` and `false` for `run` and `mutate` (which take an
idempotency key); `cost_model` is `static: …` for `read`, `mutate` and `admin` and the `--estimate` cost model for
`run`; a `run` row carries `wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }`; `owner` is the
crate of the block; `purpose` is the Purpose column. A verb printed with two kinds has `kind` equal to its default
action and an `actions` list for the others, each with its own kind and scope: `spark.cache`: `evict` (admin).

One entry in full (the others follow field for field):

```toml
[[verb]]
name = "spark.check"
cluster = "spark"
kind = "run"
tier = "S"
capabilities = ["spark:run"]
input_schema = "frob.spark.check.in/1"
output_schema = "frob.spark.check.out/1"
refusals = ["FROB-REF-603", "FROB-REF-605"]
idempotent = false
cost_model = "estimated: `--estimate` returns CPU core-seconds, GPU seconds, memory and disk with a conformal interval"
first_milestone = "M0"
task_clusters = ["devices"]
examples = ["frob spark check K-cay-step --devices cuda:0,metal --key ck-1 --json"]
owner = "frob-kernels"
purpose = "The CPU-versus-device check per kernel, with its currency"
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
4. Rows are entered when the cluster is first registered (M0). Until its own milestone a verb answers one envelope
   with `FROB-REF-724`, exit 5, on both surfaces, and `frob capabilities --json` lists it as absent.

**Notes for this cluster.**

- Seven verbs. `spark.cache` has two kinds: contents and hits are `read`; eviction is an `admin` action.
- Four verbs first answer at M0 (`devices`, `probe`, `check`, `fingerprint`), the rest at M2.
- `spark.place` answers before anything runs: the placement plan and the residency map, with requested-versus-actual
  receipts per stage.

## Acceptance criteria

- [ ] `registries/verbs.toml` holds exactly seven entries with `cluster = "spark"` (3 `read`, 4 `run`), each with
      every field of the entry schema, an example and a task-cluster list; the alias lint finds no second spelling.
- [ ] `frob-surface-diff parity --cluster spark` exits 0 in process (`frob-mcp` over stdio) and through `frobd
      --mcp`: registry, CLI and MCP each count 7, and every difference list is empty apart from the harness's closed
      exemptions.
- [ ] Every verb has an end-to-end script: `scripts/e2e/verbs_spark.sh` runs each verb of the cluster (seven), and
      from a verb's first milestone a capability script of its implementing bead exists (`frob-surface-diff
      e2e-coverage`).
- [ ] `frob <verb> --help --json` equals `fixtures/golden/verbs/spark/<verb>.help.json` byte for byte for each of
      the seven, on two runs, debug and release, Linux and macOS; a planted renamed output property is classified
      `breaking` and refused.
- [ ] A verb not yet implemented answers `FROB-REF-724`, exit 5, with one envelope on both surfaces; an implemented
      verb runs its registry example with the result its implementing bead specifies; a misspelt parameter is
      `FROB-REF-802`, exit 3; a `run` or `mutate` call without a key is `FROB-REF-805`, exit 3.
- [ ] The generated refusal table lists for each verb exactly its chapter codes plus the codes appended for its
      kind, and every code exists in `registries/refusals.toml`.
- [ ] The two-kind verb exposes `action`; an `admin` or `run` action without its scope is `FROB-REF-720`, exit 5,
      and the `read` action still answers.
- [ ] The gate row is recorded with the measured counts by a session that did not write the rows.

## Test plan

### Unit tests

`cargo test -p frob-verbs --test cluster_spark` (the real generated table; no mocks; planted registries are TOML
strings labelled `synthetic`):
- `rows_present`: exactly 7 rows in the cluster, with the names of the rows table; a planted extra row or a removed
  row fails.
- `kinds_milestones_tiers`: each row's kind, first milestone and tier equal the rows table (for example
  `spark.probe` is `run` at M0).
- `schemas_and_scopes`: input schemas are `frob.<name>.in/1`; output schemas and capabilities equal the
  machine-fields table (`spark.fingerprint` returns `frob.fingerprint/1`) (`spark.place` returns
  `frob.placement/1`).
- `refusal_lists`: each row's chapter codes equal its block (`spark.cache`: 612; `spark.check`: 603, 605;
  `spark.devices`: none); the appended codes equal the rule for its kind.
- `mcp_names_one_to_one`: the tool names are the mapped names (`spark_cache`, `spark_check`, `spark_devices`), with
  no collision against any other cluster.
- `task_clusters_and_examples`: every row sits in a task cluster of at most seven tools or is a base tool; each
  example parses against the generated CLI and its command path is the row's own.
- `kind_behaviour`: a `run` row has the five common properties and refuses `sync_wait_ms = 30001` with code 802; an
  `admin` row or action without its scope is refused with code 720; a verb whose capability is absent returns code
  724, exit 5, one envelope.

### End-to-end script and logging

`scripts/e2e/verbs_spark.sh` (header: `bead=VERBS-spark milestone=M0 labels=tether,verbs,verb:spark.*
gates=GATE-TETHER-02 seed=1 needs=cpu,mcp,frobd plants=alias,missing-row,renamed-field`) sources the shared helper
and `scripts/e2e/verbs_lib.sh` and calls `verbs_cluster_conformance spark`. Steps: (1) `frob capabilities --json`
and `frob robot-docs --json` list the seven verbs with kind and milestone; (2) `frob-surface-diff parity --cluster
spark` over stdio and through `frobd --mcp`, every difference list empty; (3) `frob-surface-diff snapshot --check
--cluster spark`; (4) each verb's registry example verbatim, expecting a result (exit 0 or 2), a job handle (exit 8,
then `frob job wait`) or `FROB-REF-724` exactly when the capability report says the verb is absent; (5) for each
implemented verb the same call as an MCP tool, the two envelopes agreeing on `verb`, `ok`, `exit`, `result`,
`refusal`, `evidence`, `bound`, `warnings` and `next_actions`; (6) per verb one call with a misspelt parameter
(`e2e_expect_refusal FROB-REF-802 3`) and, for `run` and `mutate` verbs, one without a key (`e2e_expect_refusal
FROB-REF-805 3`); (7) one envelope per call and no exit code 1.

Log: `target/e2e-logs/verbs_spark/<utc>.ndjson` with a transcript beside it. Each `cmd` line has the exact command,
exit code, wall time and the envelope's `evidence` block; each `assert` has observed and expected values and the tag
`--gate GATE-TETHER-02`; the metrics `registry_verbs`, `cli_verbs` and `mcp_tools` are logged against 7 and the
difference counts against 0; refusals are logged with code and first repair; a failure logs the first differing verb
and field; the `summary` line gives assertions passed and failed and the gate touched.

### Known answers and negative controls

- Known answers: the cluster has 7 verbs (3 `read`, 4 `run`); the tables above. KA-755 (the exit-code table on
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

- Closing rule: the bead closes at M0, when the rows are registered, the surfaces match and every verb passes its
  example or answers the not-implemented refusal. Verbs of this cluster first answer at M0, M2. The per-milestone
  surface beads (BD-022, BD-222) re-run this script as verbs land; that is how GATE-TETHER-02 is re-measured.
- Scope names are proposals (`<cluster>:<read|run|write|admin>` plus the named scopes of the surface) until the
  capability bead BD-258 fixes the scope registry; a `run` or `write` scope of a cluster implies its `read` scope.
  Inputs and outputs above name contents, not flag spellings, except where a flag is quoted; the implementing bead
  fixes the spellings and the snapshot then pins them (additive changes keep the schema's major version; anything
  else raises it).

## References

Appendix J (J.1 entry form; J.2 inventory rows; J.5 task clusters), §21.8, §21.10, §21.12, §21.13, §21.15, §21.16,
§32.4, §32.6, §32.12.3, §32.18, §32.21; GATE-TETHER-02; INV-TETHER-01, -02; KA-755.
