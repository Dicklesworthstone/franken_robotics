---
key: "VERBS-explain"
title: "frob-verbs registry: cluster `explain`, three rows, with parity, e2e and schema-snapshot checks"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "tether", "atelier", "frob-verbs", "solid", "registry", "verb", "test"]
parent: "WP-06"
deps: ["BD-022", "NJL-01", "NJL-02", "NJL-04", "NJL-05", "STD-E2E"]
needs: []
related: []
gates: ["GATE-TETHER-02"]
plan_refs: ["Appendix J (J.1, J.2, J.3, J.5)", "§28.5", "§32.6", "§32.12.3", "§32.21 (GATE-TETHER-02)"]
---
## Background

FrankenRobotics (a robotics simulation and design system) is reached through the command line `frob`, the daemon
`frobd` and the MCP server `frob-mcp` (the Model Context Protocol, by which an agent harness lists and calls tools).
Every capability is a verb, `cluster.verb`, declared once in `registries/verbs.toml` (crate `frob-verbs`, tier
Solid). The CLI, the daemon protocol, the MCP tools, the schemas, the refusal table and the agent guide are
generated from that file, and gate GATE-TETHER-02 compares the served surfaces with it. The registry holds 244 verbs
in 44 clusters (namespaces): the 243 of the specification and `artifact.import` (VERBS-artifact); the
count of record is NJL-03's.

This bead is the registry of record for the cluster `explain`. The cluster covers token-budgeted explanations that
end in next actions: what limits a design, the first cause of a run's outcome, and why two designs differ on a task.
It has three verbs (3 `read`); the handlers live in `frob-chronicle` (`explain.run`, `explain.design`) and
`frob-opt` (`explain.compare`). The bead carries every row
completely, with the three checks that keep the rows true: the generated CLI commands and MCP tools match the list;
every verb has an end-to-end script; the schema printed by `frob <verb> --help --json` is stable.

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

In: the three registry entries of cluster `explain`; the MCP tool names; the cluster's conformance script; the
help-schema snapshots; each verb's answer before its milestone. Out: behaviour (the implementing beads named below);
the entry schema and lints (NJL-01); the conformance harness (NJL-02); the alias lint (NJL-04); task clusters
(NJL-05, NJL-06); generation of the surfaces (the Tether seed beads).

## Technical approach

**The rows.**

| Verb | Kind | First milestone | Tier | Purpose | Defined in |
|---|---|---|---|---|---|
| `explain.compare` | read | M3 | S | Why two designs differ on a task: first divergent event and binding atoms | §28.5 |
| `explain.design` | read | M1 | S | What limits a design, by analysis, ending with next actions | §28.5 |
| `explain.run` | read | M1 | S | First cause of a run's outcome, token-budgeted, ending with next actions | §28.5 |

**Contract of each verb.** Evidence labels used below (the `evidence` block of the envelope):
- *pass-through*: passes through the claim product of the stored objects it reads and adds no claim of its own; the
  read itself is `exact`.

**`explain.compare`** (MCP tool `explain_compare`; owner `frob-opt`)
- Inputs: a stored paired comparison (the record written by `eval.compare` or `study.compare`, which fixes the two
  designs, the task and the world addresses); optional `--metric` (default: the comparison's primary metric);
  `--budget` (token budget, default 600); `--regimes` (k, the most regimes named, default 3).
- Outputs: per regime (a contact signature and a scene region at the first divergent task event of a traced pair):
  pairs, candidate wins, losses and ties, mean paired difference with a seeded bootstrap interval, the losing arm's
  dominant failure class and binding requirement atom, one representative address with drill-down handles; the
  counts `traced_pairs`, `untraced_pairs`, `planned`; omitted regimes by name; next actions (the sensitivity
  analysis on the losing design, the `autopsy.intervene` fork that would test the described mechanism). Evidence:
  *pass-through*, labelled `Estimated`, `claims: []`; the comparison's standing and `provisional` flag echoed.
- Chapter refusals: `FROB-REF-840` (sealed confirmation data asked for by a session without the qualification
  administration scope; every `explain.*` verb refuses a confirmation run or study).
- Implemented by: N28a-17 (the verb), BD-262 (the comparison it reads).
- Extended by: N28c-38 (the "Controller budget" section: both budget curves).
- Correction (DESIGN, round 1): the row formerly listed `FROB-REF-548` (pooling evidence or calibration across two
  model identities). The verb writes no ledger stratum or calibration set, and the chapter refusal registry
  (N28-REF) assigns 548 to `eval.promote`, `eval.rank-agreement` and `opt.cascade` only; listing it here would make
  the bidirectional refusal cross-check fail. The input is a comparison, as the chapter's call
  `frob explain compare <comparison>` states; the two designs and the task are read from it.

**`explain.design`** (MCP tool `explain_design`; owner `frob-chronicle`)
- Inputs: design; optional `--task` (limits evaluated at that task's points); `--budget` (token budget, default 600).
- Outputs: what limits the design, analysis by analysis (binding constraints by normalized margin, each with its
  evidence class, the card fields it rests on and their provenance class), ending with next actions (ranked repairs
  as typed edits with predicted effects). Evidence: *pass-through*.
- Chapter refusals: none of its own.
- Implemented by: BD-112. DESIGN (BD-112): owned by `frob-chronicle`, not `frob-opt`: both explain verbs of M1 share
  one token estimator, one section assembler with drill-down handles and one `next_actions` ranker, and only the
  layer-6 crate reads both the analysis findings (layer 4) and the telemetry and ledger (layer 6).

**`explain.run`** (MCP tool `explain_run`; owner `frob-chronicle`)
- Inputs: run; optional `--world` (one world address); `--budget` (token budget, default 600).
- Outputs: the first cause of the outcome with its evidence (labelled `Estimated`, a description), ending with next
  actions. Evidence: *pass-through*.
- Chapter refusals: `FROB-REF-840` (a run sealed in an open qualification campaign, asked for by a session without
  the qualification administration scope); `FROB-REF-888` (a metric computed over missing samples, when the caller
  requires it).
- Implemented by: BD-112.
- Extended by: N28a-17 (at M3: next actions ranked from the sensitivity analysis, the first-divergence lead after an
  edit, the intervention offer).

**Machine fields of each row.**

| Verb | Capabilities | Output schema | Task clusters | Example |
|---|---|---|---|---|
| `explain.compare` | `explain:read` | `frob.explain.compare.out/1` | `ladder`, `explain` | `frob explain compare st:9a1 --budget 600 --json` |
| `explain.design` | `explain:read` | `frob.explain.design.out/1` | `explain` | `frob explain design quad-a --budget 600 --json` |
| `explain.run` | `explain:read` | `frob.explain.run.out/1` | `diagnose`, `explain` | `frob explain run r:19e2 --budget 600 --json` |

The remaining fields follow by rule: `input_schema` is `frob.<verb name>.in/1`; `refusals` is the chapter list of
the verb's block; `idempotent` is `true` for `read` and `admin` and `false` for `run` and `mutate` (which take an
idempotency key); `cost_model` is `static: …` for `read`, `mutate` and `admin` and the `--estimate` cost model for
`run`; a `run` row carries `wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }`; `owner` is the
crate of the block; `purpose` is the Purpose column.

One entry in full (the others follow field for field):

```toml
[[verb]]
name = "explain.compare"
cluster = "explain"
kind = "read"
tier = "S"
capabilities = ["explain:read"]
input_schema = "frob.explain.compare.in/1"
output_schema = "frob.explain.compare.out/1"
refusals = ["FROB-REF-840"]
idempotent = true
cost_model = "static: answered from the store or by a small CPU computation; no job"
first_milestone = "M3"
task_clusters = ["ladder", "explain"]
examples = ["frob explain compare st:9a1 --budget 600 --json"]
owner = "frob-opt"
purpose = "Why two designs differ on a task: first divergent event and binding atoms"
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
   indeterminate; never 1 on purpose.
4. Rows are entered when the cluster is first registered (M1); each row's tool is added to the task-cluster
   tables its `task_clusters` names in the same change (NJL-05's staging rule). A verb whose handler is not in
   the build answers one envelope with `FROB-REF-724`, exit 5, on both surfaces, and `frob capabilities --json`
   lists it as absent; a reduced form that lands before the row's `first_milestone` answers and is listed as
   implemented with its form named (NJL-01, NJL-03: the field keeps the appendix's milestone).

**Notes for this cluster.**

- Every explanation is sized to a token budget, carries handles for drill-down, and ends with `next_actions` whose
  commands are complete command lines. No explanation contains text that originated in an import (INV-TETHER-13).

## Acceptance criteria

- [ ] `registries/verbs.toml` holds exactly three entries with `cluster = "explain"` (3 `read`), each with every
      field of the entry schema, an example and a task-cluster list; the alias lint finds no second spelling.
- [ ] `frob-surface-diff parity --cluster explain` exits 0 in process (`frob-mcp` over stdio) and through `frobd
      --mcp`: registry, CLI and MCP each count 3, and every difference list is empty apart from the harness's closed
      exemptions.
- [ ] Every verb has an end-to-end script: `scripts/e2e/verbs_explain.sh` runs each verb of the cluster (three), and
      from a verb's first milestone a capability script of its implementing bead exists (`frob-surface-diff
      e2e-coverage`).
- [ ] `frob <verb> --help --json` equals `fixtures/golden/verbs/explain/<verb>.help.json` byte for byte for each of
      the three, on two runs, debug and release, Linux and macOS; a planted renamed output property is classified
      `breaking` and refused.
- [ ] A verb not yet implemented answers `FROB-REF-724`, exit 5, with one envelope on both surfaces; an implemented
      verb runs its registry example with the result its implementing bead specifies; a misspelt parameter is
      `FROB-REF-802`, exit 3.
- [ ] The generated refusal table lists for each verb exactly its chapter codes plus the codes appended for its
      kind and fields (NJL-01), and every code exists in `registries/refusals.toml`.
- [ ] The gate row is recorded with the measured counts by a session that did not write the rows.

## Test plan

### Unit tests

`cargo test -p frob-verbs --test cluster_explain` (the real generated table; no mocks; planted registries are TOML
strings labelled `synthetic`):
- `rows_present`: exactly 3 rows in the cluster, with the names of the rows table; a planted extra row or a removed
  row fails.
- `kinds_milestones_tiers`: each row's kind, first milestone and tier equal the rows table (for example
  `explain.run` is `read` at M1).
- `schemas_and_scopes`: input schemas are `frob.<name>.in/1`; output schemas and capabilities equal the
  machine-fields table.
- `refusal_lists`: each row's chapter codes equal its block (`explain.compare`: 840; `explain.design`: none;
  `explain.run`: 840, 888); the appended codes equal the rule for its kind and fields; a planted 548 on `explain.compare`
  fails against the chapter registry's verbs column.
- `owners`: `explain.run` and `explain.design` are owned by `frob-chronicle`, `explain.compare` by `frob-opt`.
- `mcp_names_one_to_one`: the tool names are the mapped names (`explain_compare`, `explain_design`, `explain_run`),
  with no collision against any other cluster.
- `task_clusters_and_examples`: every row sits in a task cluster of at most seven tools or is a base tool; each
  example parses against the generated CLI and its command path is the row's own.
- `kind_behaviour`: a verb whose capability is absent returns code 724, exit 5, one envelope.

### End-to-end script and logging

`scripts/e2e/verbs_explain.sh` (header: `bead=VERBS-explain milestone=M1
labels=tether,verbs,verb:explain.compare,verb:explain.design,verb:explain.run gates=GATE-TETHER-02 seed=1
needs=cpu,mcp,frobd plants=alias,missing-row,renamed-field`) sources the shared helper and
`scripts/e2e/verbs_lib.sh` and calls `verbs_cluster_conformance explain`. Steps: (1) `frob capabilities --json` and
`frob robot-docs --json` list the three verbs with kind and milestone; (2) `frob-surface-diff parity --cluster
explain` over stdio and through `frobd --mcp`, every difference list empty; (3) `frob-surface-diff snapshot --check
--cluster explain`; (4) each verb's registry example verbatim, expecting a result (exit 0 or 2), a job handle (exit
8, then `frob job wait`) or `FROB-REF-724` exactly when the capability report says the verb is absent; (5) for each
implemented verb the same call as an MCP tool, the two envelopes agreeing on `verb`, `ok`, `exit`, `result`,
`refusal`, `evidence`, `bound`, `warnings` and `next_actions`; (6) per verb one call with a misspelt parameter
(`e2e_expect_refusal FROB-REF-802 3`); (7) one envelope per call and no exit code 1.

Log: `target/e2e-logs/verbs_explain/<utc>.ndjson` with a transcript beside it. Each `cmd` line has the exact
command, exit code, wall time and the envelope's `evidence` block; each `assert` has observed and expected values
and the tag `--gate GATE-TETHER-02`; the metrics `registry_verbs`, `cli_verbs` and `mcp_tools` are logged against 3
and the difference counts against 0; refusals are logged with code and first repair; a failure logs the first
differing verb and field; the `summary` line gives assertions passed and failed and the gate touched.

### Known answers and negative controls

- Known answers: the cluster has 3 verbs (3 `read`); the tables above. KA-755 (the exit-code table on malformed,
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

- Closing rule: the bead closes at M1, when the rows are registered, the surfaces match and every verb passes its
  example or answers the not-implemented refusal. Verbs of this cluster first answer at M1, M3. The per-milestone
  surface beads (BD-137, BD-312) re-run this script as verbs land; that is how GATE-TETHER-02 is re-measured.
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

Appendix J (J.1 entry form; J.2 inventory rows; J.5 task clusters), §28.5, §32.4, §32.6, §32.12.3, §32.18, §32.21;
GATE-TETHER-02; INV-TETHER-01, -02; KA-755.
