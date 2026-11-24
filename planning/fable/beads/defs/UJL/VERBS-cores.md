---
key: "VERBS-cores"
title: "frob-verbs registry: cluster `cores`, one row (`cores.show`), with parity, e2e and schema-snapshot checks"
type: "task"
priority: 2
milestone: "M3"
labels: ["m3", "tether", "atelier", "frob-verbs", "solid", "registry", "verb", "test"]
parent: "WP-26"
deps: ["BD-022", "NJL-01", "NJL-02", "NJL-04", "NJL-05", "STD-E2E"]
needs: []
related: []
gates: ["GATE-TETHER-02"]
plan_refs: ["Appendix J (J.1, J.2, J.3, J.5)", "§28.19", "§32.6", "§32.12.3", "§32.21 (GATE-TETHER-02)"]
---
## Background

FrankenRobotics (a robotics simulation and design system) is reached through the command line `frob`, the daemon
`frobd` and the MCP server `frob-mcp` (the Model Context Protocol, by which an agent harness lists and calls tools).
Every capability is a verb, `cluster.verb`, declared once in `registries/verbs.toml` (crate `frob-verbs`, tier
Solid). The CLI, the daemon protocol, the MCP tools, the schemas, the refusal table and the agent guide are
generated from that file, and gate GATE-TETHER-02 compares the served surfaces with it. The registry holds 244 verbs
in 44 clusters (namespaces): the 243 of the specification and `artifact.import` (VERBS-artifact); the
count of record is NJL-03's.

This bead is the registry of record for the cluster `cores`. The cluster covers one verb that shows a decision core:
a small fixed table that decides an allocation (promote, stop, certify) with a stated loss model, obligations and
fallback. It has one verb (1 `read`); the handlers live in `frob-opt`. The bead carries every row completely, with
the three checks that keep the rows true: the generated CLI commands and MCP tools match the list; every verb has an
end-to-end script; the schema printed by `frob <verb> --help --json` is stable.

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

In: the one registry entry of cluster `cores`, which is the row of record for `cores.show` and the only place
where that row is entered; the MCP tool names; the cluster's conformance script; the help-schema snapshots; each
verb's answer before its milestone. Out: behaviour (the implementing beads named below: for `cores.show`,
N28c-04 has the content of the two schemas, the flag spellings, the output, the stale-table behaviour and the
decision summary, quotes this row without entering one, and depends on this bead); the entry schema and lints
(NJL-01); the conformance harness (NJL-02); the alias lint (NJL-04); task clusters (NJL-05, NJL-06); generation
of the surfaces (the Tether seed beads).

## Technical approach

**The rows.**

| Verb | Kind | First milestone | Tier | Purpose | Defined in |
|---|---|---|---|---|---|
| `cores.show` | read | M3 | S | A decision core's table, loss model, obligations and fallback | §28.19 |

**Contract of each verb.** Evidence labels used below (the `evidence` block of the envelope):
- *pass-through*: passes through the claim product of the stored objects it reads and adds no claim of its own; the
  read itself is `exact`.

**`cores.show`** (MCP tool `cores_show`; owner `frob-opt`)
- Inputs: exactly one of `core` (`dc-01` to `dc-11`) and `all` (one summary line per core); optional `stratum`
  (the table bound to that ledger stratum; default the committed default table); `rows` (include the table rows,
  paginated with a cursor pinned to the registry commit); `decisions <campaign-or-run>` (a summary of the decisions
  the core made there); `rejected` (the methods considered for this core and not built, with their reopen
  conditions); cursor.
- Outputs: `id`, `decision`, `owner_crate`, `governed` (verbs), `timescale`, `plane`, `family`, `tier`; `policy`
  and adoption `status`, `registered`; `artifacts[]` (kind, version, hash, generator, parameters, `bound_to`,
  `state`: `loaded`, `stale`, `missing` or `not-registry-version`, and the rows with `rows`); `loss` (states,
  actions, constants, each constant `declared` or `open: OPEN-ATEL-1`); `obligations[]` with the status of the
  last discharge; `fallback {rule, triggers, in_force, reason}`; `budget {mode}`; `replay`, `calibration`, `demo`;
  with `decisions`, counts by rule, the fallbacks fired and the shadow agreements; with `rejected`, the entries of
  `registries/cores/rejected.toml` (method, why not, kept as, reopen condition, status). Exit 0 when every table
  is loaded, 2 with a stale or missing table. Evidence: *pass-through*.
- Chapter refusals: `FROB-REF-871` (a decision-core table that is missing, stale against its contract, or not the
  registry version).
- Implemented by: BD-309, N28c-04. Extended by: N28c-07 (`rejected`).

**Machine fields of each row.**

| Verb | Capabilities | Output schema | Task clusters | Example |
|---|---|---|---|---|
| `cores.show` | `cores:read` | `frob.cores.show.out/1` | `ladder` | `frob cores show dc-01 --json` |

The remaining fields follow by rule: `input_schema` is `frob.<verb name>.in/1`; `refusals` is the chapter list of
the verb's block; `idempotent` is `true` for `read` and `admin` and `false` for `run` and `mutate` (which take an
idempotency key); `cost_model` is `static: …` for `read`, `mutate` and `admin` and the `--estimate` cost model for
`run`; a `run` row carries `wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }`; `owner` is the
crate of the block; `purpose` is the Purpose column; `cores.show` has `paginated = true` (its table rows).

One entry in full (the others follow field for field):

```toml
[[verb]]
name = "cores.show"
cluster = "cores"
kind = "read"
tier = "S"
capabilities = ["cores:read"]
input_schema = "frob.cores.show.in/1"
output_schema = "frob.cores.show.out/1"
refusals = ["FROB-REF-871"]
idempotent = true
cost_model = "static: answered from the store or by a small CPU computation; no job"
first_milestone = "M3"
task_clusters = ["ladder"]
examples = ["frob cores show dc-01 --json"]
owner = "frob-opt"
purpose = "A decision core's table, loss model, obligations and fallback"
paginated = true
```

**Conventions the rows follow.**

1. The MCP tool name is the verb name with `.` and `-` replaced by `_`. The output on the wire is one envelope
   `frob.envelope/1` with `result` specialized to the output schema; its `evidence` block is a claim product (the
   weakest class, tier and determinism class among the inputs).
2. The generator adds the common input properties of the kind, as NJL-01's kinds table lists them (`run`:
   `sync_wait_ms` from 0 to the row's `wait.mcp_max_ms`, 30,000, with MCP default 5,000; `idempotency_key`, CLI
   `--key`; `budget`; `estimate_only`, CLI `--estimate`; `on_cancel`, `leave` (the default: the job keeps
   running) or `cancel_job`; `envelope_scope`, CLI `--envelope-scope <gate:…|campaign:…|pretrain:…>`, the
   approved compute-envelope scope the job cites (program convention 14, owned by NJL-01); `mutate`:
   `idempotency_key` and `expected_parent`). `idempotency_key` is never in the JSON-schema `required` array: the
   annotation `x-frob-key: "required"` (`"optional"` on a `key_optional` entry) carries the rule, and the
   admission transaction refuses a missing key with 805 on both surfaces. The surface-wide refusals appended by
   kind: `FROB-REF-720` (scope missing), 723 (token invalid), 724 (not implemented in this build) and 802 (schema
   failure, including an unknown parameter) for every verb; 701 to 704 (budget and admission), 761 (above the
   compute envelope in force; N36-20's `admit_run`), 803 (key reused with different content) and 805 (missing
   key) for `run`; 725, 803, 804 (stale parent) and 805 for `mutate`. The properties and codes driven by fields
   are those of NJL-01's variation table: `cursor`, `limit` and `follow` with 806 and 807 for a paginated entry;
   808 (an integer outside ±(2⁵³ − 1)) for an entry whose generated input schema has an `int53` property at any
   depth, whatever its kind (every `run` entry has one, `sync_wait_ms`); `estimate_only` on a non-`run` entry
   with `estimable = true`; no 805 for a `key_optional` entry; the `action` property, with each action's codes,
   for an entry with `actions`; 790 for a non-empty `needs_groups`; 891 for an `artifact-input<F>`,
   `artifact-ref<F>` or `artifact-identity<F>` parameter, and 892 as well for `artifact-identity<F>`; 893 for a
   `foreign-file` parameter; 734 and 828 for a parameter that accepts a path to a file (`artifact-input<F>`,
   `artifact-identity<F>`, `foreign-file`). The later variations of that table (10 to 13) land with their beads
   and are read there, not copied here. `cores.show` takes identifiers only (a core, a stratum, a campaign or
   run), so it gets neither 734 nor 828; it gets 806, 807 and, through the paginated `limit`, 808.
3. Exit codes: 0 done; 2 done with findings; 3 refused input (1xx to 5xx, and 8xx other than conflicts); 4 budget
   (70x and 761); 5 capability (72x, 73x); 6 device (6xx); 7 conflict (803, 804, 809); 8 accepted and still running; 9
   indeterminate; never 1 on purpose.
4. Rows are entered when the cluster is first registered (M3); each row's tool is added to the task-cluster
   tables its `task_clusters` names in the same change (NJL-05's staging rule). A verb whose handler is not in
   the build answers one envelope with `FROB-REF-724`, exit 5, on both surfaces, and `frob capabilities --json`
   lists it as absent; a reduced form that lands before the row's `first_milestone` answers and is listed as
   implemented with its form named (NJL-01, NJL-03: the field keeps the appendix's milestone).

**Notes for this cluster.**

- `FROB-REF-871` (a decision-core table that is missing, stale against its contract, or not the registry
  version) is in this row's list because `cores.show` can emit it, but this verb does not refuse on it (DESIGN,
  the implementing bead N28c-04: inspection must work exactly when something is wrong). It completes with exit
  2, marks the table `stale` or `missing`, shows the fallback in force and carries the code in a finding with its
  repair ("regenerate; the fallback is in force meanwhile"). The verbs a core governs are where the code is a
  refusal or a recorded fallback.
- The core identifier in the example is lower case (`dc-01`), the spelling the implementing bead fixes for the
  command line and the input schema; an identifier outside `dc-01` to `dc-11` is a schema refusal, exit 3.
- The evidence label *pass-through* gives, for a core's record and tables alone, class `Tested`, tier `static`
  and determinism `exact` with no claims; with a decision summary it is the weakest among the decision records
  read (N28c-04 states both).

## Acceptance criteria

- [ ] `registries/verbs.toml` holds exactly one entry with `cluster = "cores"` (1 `read`), each with every field of
      the entry schema, an example and a task-cluster list; the alias lint finds no second spelling.
- [ ] `frob-surface-diff parity --cluster cores` exits 0 in process (`frob-mcp` over stdio) and through `frobd
      --mcp`: registry, CLI and MCP each count 1, and every difference list is empty apart from the harness's closed
      exemptions.
- [ ] Every verb has an end-to-end script: `scripts/e2e/verbs_cores.sh` runs each verb of the cluster (one), and
      from a verb's first milestone a capability script of its implementing bead exists (`frob-surface-diff
      e2e-coverage`).
- [ ] `frob <verb> --help --json` equals `fixtures/golden/verbs/cores/<verb>.help.json` byte for byte for each of
      the one, on two runs, debug and release, Linux and macOS; a planted renamed output property is classified
      `breaking` and refused.
- [ ] A verb not yet implemented answers `FROB-REF-724`, exit 5, with one envelope on both surfaces; an implemented
      verb runs its registry example with the result its implementing bead specifies; a misspelt parameter is
      `FROB-REF-802`, exit 3.
- [ ] The generated refusal table lists for each verb exactly its chapter codes plus the codes appended for its
      kind and fields (NJL-01), and every code exists in `registries/refusals.toml`.
- [ ] The gate row is recorded with the measured counts by a session that did not write the rows.

## Test plan

### Unit tests

`cargo test -p frob-verbs --test cluster_cores` (the real generated table; no mocks; planted registries are TOML
strings labelled `synthetic`):
- `rows_present`: exactly 1 row in the cluster, with the names of the rows table; a planted extra row or a removed
  row fails.
- `kinds_milestones_tiers`: each row's kind, first milestone and tier equal the rows table (for example `cores.show`
  is `read` at M3).
- `schemas_and_scopes`: input schemas are `frob.<name>.in/1`; output schemas and capabilities equal the
  machine-fields table.
- `refusal_lists`: each row's chapter codes equal its block (`cores.show`: 871); the appended codes equal the rule
  for its kind and fields (NJL-01): {720, 723, 724, 802} by kind, 806 and 807 (paginated) and 808 (through the
  `int53` property `limit`), and no 734 or 828.
- `input_shape`: the generated input schema requires exactly one of `core` and `all`, accepts `dc-01` to `dc-11`
  only (`dc-12` is 802, exit 3), and has `stratum`, `rows`, `decisions`, `rejected` and `cursor`.
- `mcp_names_one_to_one`: the tool names are the mapped names (`cores_show`), with no collision against any other
  cluster.
- `task_clusters_and_examples`: every row sits in a task cluster of at most seven tools or is a base tool; each
  example parses against the generated CLI and its command path is the row's own.
- `kind_behaviour`: a verb whose capability is absent returns code 724, exit 5, one envelope.

### End-to-end script and logging

`scripts/e2e/verbs_cores.sh` (header: `bead=VERBS-cores milestone=M3 labels=tether,verbs,verb:cores.show
gates=GATE-TETHER-02 seed=1 needs=cpu,mcp,frobd plants=alias,missing-row,renamed-field`) sources the shared helper
and `scripts/e2e/verbs_lib.sh` and calls `verbs_cluster_conformance cores`. Steps: (1) `frob capabilities --json`
and `frob robot-docs --json` list the one verb with kind and milestone; (2) `frob-surface-diff parity --cluster
cores` over stdio and through `frobd --mcp`, every difference list empty; (3) `frob-surface-diff snapshot --check
--cluster cores`; (4) each verb's registry example verbatim, expecting a result (exit 0 or 2), a job handle (exit 8,
then `frob job wait`) or `FROB-REF-724` exactly when the capability report says the verb is absent; (5) for each
implemented verb the same call as an MCP tool, the two envelopes agreeing on `verb`, `ok`, `exit`, `result`,
`refusal`, `evidence`, `bound`, `warnings` and `next_actions`; (6) per verb one call with a misspelt parameter
(`e2e_expect_refusal FROB-REF-802 3`); (7) one envelope per call and no exit code 1.

Log: `target/e2e-logs/verbs_cores/<utc>.ndjson` with a transcript beside it. Each `cmd` line has the exact command,
exit code, wall time and the envelope's `evidence` block; each `assert` has observed and expected values and the tag
`--gate GATE-TETHER-02`; the metrics `registry_verbs`, `cli_verbs` and `mcp_tools` are logged against 1 and the
difference counts against 0; refusals are logged with code and first repair; a failure logs the first differing verb
and field; the `summary` line gives assertions passed and failed and the gate touched.

### Known answers and negative controls

- Known answers: the cluster has 1 verb (1 `read`); the tables above. KA-755 (the exit-code table on malformed,
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

- Closing rule: the bead closes at M3, when the rows are registered, the surfaces match and every verb passes its
  example or answers the not-implemented refusal. The per-milestone surface beads (BD-312) re-run this script as
  verbs land; that is how GATE-TETHER-02 is re-measured.
- Scope names are proposals (`<cluster>:<read|run|write|admin>` plus the named scopes of the surface) until the
  capability bead BD-258 fixes the scope registry; a `run` or `write` scope of a cluster implies its `read` scope.
  Inputs and outputs above name contents, not flag spellings, except where a flag is quoted; the implementing bead
  fixes the spellings and the snapshot then pins them (additive changes keep the schema's major version; anything
  else raises it).

## References

Appendix J (J.1 entry form; J.3 added rows; J.5 task clusters), §28.19, §32.4, §32.6, §32.12.3, §32.18, §32.21;
GATE-TETHER-02; INV-TETHER-01, -02; KA-755.
