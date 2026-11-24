---
key: "VERBS-experiment"
title: "frob-verbs registry: cluster `experiment`, one row, with parity, e2e and schema-snapshot checks"
type: "task"
priority: 2
milestone: "M3"
labels: ["m3", "tether", "atelier", "frob-verbs", "solid", "registry", "verb", "test", "review-notes-open"]
parent: "WP-62"
deps: ["BD-022", "NJL-01", "NJL-02", "NJL-04", "NJL-05", "STD-E2E"]
needs: []
related: []
gates: ["GATE-TETHER-02"]
plan_refs: ["Appendix J (J.1, J.2, J.3, J.5)", "§28.16", "§25.14", "§25.3.1", "§32.6", "§32.12.3", "§32.21 (GATE-TETHER-02)"]
---
## Background

FrankenRobotics (a robotics simulation and design system) is reached through the command line `frob`, the daemon
`frobd` and the MCP server `frob-mcp` (the Model Context Protocol, by which an agent harness lists and calls tools).
Every capability is a verb, `cluster.verb`, declared once in `registries/verbs.toml` (crate `frob-verbs`, tier
Solid). The CLI, the daemon protocol, the MCP tools, the schemas, the refusal table and the agent guide are
generated from that file, and gate GATE-TETHER-02 compares the served surfaces with it. The registry holds 244 verbs
in 44 clusters (namespaces): the 243 of the specification and `artifact.import` (VERBS-artifact); the
count of record is NJL-03's.

This bead is the registry of record for the cluster `experiment`. The cluster covers one verb that ranks
information-gathering proposals by their value for a pending decision. It has a cluster of its own because it serves
mission queries, design comparison and calibration alike. It has one verb (1 `read`); the handler lives in
`frob-mission`, which calls the ranking of `frob-opt`. The bead carries every row completely, with the three checks that keep the rows true: the generated CLI
commands and MCP tools match the list; every verb has an end-to-end script; the schema printed by `frob <verb>
--help --json` is stable.

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

In: the one registry entry of cluster `experiment`; the MCP tool names; the cluster's conformance script; the
help-schema snapshots; each verb's answer before its milestone. Out: behaviour (the implementing beads named below);
the entry schema and lints (NJL-01); the conformance harness (NJL-02); the alias lint (NJL-04); task clusters
(NJL-05, NJL-06); generation of the surfaces (the Tether seed beads).

## Technical approach

**The rows.**

| Verb | Kind | First milestone | Tier | Purpose | Defined in |
|---|---|---|---|---|---|
| `experiment.suggest` | read | M4 | F | Ranked information-gathering proposals with testable predictions | §28.16, §25.14 |

**Contract of each verb.** Evidence labels used below (the `evidence` block of the envelope):
- *screening*: class Estimated: a screening or planning signal that may discard under the ladder's rules and may not
  establish a comparison.

**`experiment.suggest`** (MCP tool `experiment_suggest`; owner `frob-mission`)
- Inputs: what the decision is about, one of: finalist designs with their stored comparisons (or the study whose
  finalists are compared, `study`), a mission answer with unresolved cells, or the competing causes (`between a b
  [c…]`) with the evidence already held (`evidence`: check reports, runs, verdicts); the remaining budget, spelled
  `--budget <key>=<value>` with the budget keys of N32a-15 (`wall_s`, `gpu_s`, `cpu_core_s`; for example
  `frob experiment suggest a:93f0… --budget gpu_s=600 --json`, N25c-18), and the budget price λ; the allowed
  experiments; `grid expert` (the expert-grid form); `--scope ignore` (N25e-38: evaluates every nuisance parameter
  independently, as if no scope were declared, and marks every bound `scope: undeclared`; the comparison form
  printed beside the shared form, which can only widen bounds; without it the declared scopes apply);
  `tie_break` (`--tie-break largest-gap|smallest-radius`, default `largest-gap`; N25cx-02, M3: the order of
  separating rows of equal cost, the largest gap between the nearest reading sets or the smallest radius, with the
  row id breaking any remaining tie; the command-line form of BD-291's mission field `candidates.tie_break`, which
  the `--between` form has no mission to carry; with a positional answer whose mission sets
  `candidates.tie_break`, a `--tie-break` that differs from it is FROB-REF-802). The budget is an input of this
  `read` verb that sizes the proposals; the verb spends nothing.
- Outputs: ranked proposals `{kind, target, cost, component, evsi, ratio, prediction, model_class, command}` (a
  measurement, a probe, a higher-tier simulation, a fine-tune, a cell to split), each with a testable prediction per
  mechanism (an 80% interval), cost and decision relevance and the exact command that would run it; the direct
  measurement as the stronger comparator; `stop {price, best_rejected_ratio}` (`STOP` when no proposal can change
  the decision); a posterior summary. No output field is a verdict type. The expert-grid form orders a
  hand-declared list of candidate experiments by cost. Evidence: *screening* (class `Estimated`, `claims` empty;
  the evidence class of the answer read; determinism D0).
- Chapter refusals: `FROB-REF-859` (an experiment or world edit cannot be realized through the declared interfaces,
  or does not declare the causal path it changes); `FROB-REF-883`, on the interim form of the request below, from
  M4: every ranked suggestion registers the predicted outcome interval of its proposals in the prediction ledger
  before anything runs (§28.16, §28.17; BD-344's ledger, M4; the expert-grid form of M3 registers nothing), and a
  registration for a target that already has an outcome record is
  raised as 883 with `detail = "registration-after-outcome"` and `pointer: prediction.target` (N28cx-01, N30-REF).
  DESIGN: the registration is the studio's own prediction record, appended by the ledger, and no object in the
  caller's namespace, so the kind stays `read`.
- Pending, uncoded: `prediction-after-outcome` (N28cx-01's held `8xx` request, exit 3; until a number is released
  it is raised on the interim form above, so the row's list stays 859 and 883 and gains the new code when it is
  assigned, or stays as it is if the owner merges the request into 883).
- Correction (round 3): the row formerly also listed `FROB-REF-872` (a value-of-information output cited as a
  verdict). This verb takes no verdict parameter and cannot raise it: 872 is raised downstream, by the guard on the
  parameters marked `accepts = "verdict"` (N28c-12: `eval.promote`'s `result`, the `--cite` of the report verbs,
  `result.revalidate`), as N25c-18, N25cx-02 and BD-351 state, and N28-REF's two-way check `verbs_lists_agree`
  fails on a row that lists 872 when its verbs column does not name the verb.
- Implemented by: N25cx-02 (M3: the expert-grid form, which separates two causes at the least cost), BD-351 (M4:
  the ranking by decision value per cost, with the stop rule), N25c-18 (M4: the mission chapter's proposal
  builders).
- Extended by: N25e-38 (`--scope ignore`), N28cx-01 and BD-344 (883 on the registration of the outcome
  intervals). The row's `first_milestone` keeps the appendix's M4; from M3 the verb answers its expert-grid form
  and `frob capabilities` lists it as implemented with `form: "expert-grid"`; the ranked form before BD-351 lands
  answers `FROB-REF-724` naming the form (NJL-01, NJL-03). No `forms` field is added to the entry.

**Machine fields of each row.**

| Verb | Capabilities | Output schema | Task clusters | Example |
|---|---|---|---|---|
| `experiment.suggest` | `experiment:read` | `frob.experiment.suggest.out/1` | `compare`, `search` | `frob experiment suggest --study plan:b3:52e8 --json` |

The remaining fields follow by rule: `input_schema` is `frob.<verb name>.in/1`; `refusals` is the chapter list of
the verb's block; `idempotent` is `true` for `read` and `admin` and `false` for `run` and `mutate` (which take an
idempotency key); `cost_model` is `static: …` for `read`, `mutate` and `admin` and the `--estimate` cost model for
`run`; a `run` row carries `wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }`; `owner` is the
crate of the block; `purpose` is the Purpose column.

One entry in full (the others follow field for field):

```toml
[[verb]]
name = "experiment.suggest"
cluster = "experiment"
kind = "read"
tier = "F"
capabilities = ["experiment:read"]
input_schema = "frob.experiment.suggest.in/1"
output_schema = "frob.experiment.suggest.out/1"
refusals = ["FROB-REF-859", "FROB-REF-883"]
idempotent = true
cost_model = "static: answered from the store or by a small CPU computation; no job"
first_milestone = "M4"
task_clusters = ["compare", "search"]
examples = ["frob experiment suggest --study plan:b3:52e8 --json"]
owner = "frob-mission"
purpose = "Ranked information-gathering proposals with testable predictions"
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
4. Rows are entered when the cluster is first registered (M3, for the expert-grid form); each row's tool is added to the task-cluster
   tables its `task_clusters` names in the same change (NJL-05's staging rule). A verb whose handler is not in
   the build answers one envelope with `FROB-REF-724`, exit 5, on both surfaces, and `frob capabilities --json`
   lists it as absent; a reduced form that lands before the row's `first_milestone` answers and is listed as
   implemented with its form named (NJL-01, NJL-03: the field keeps the appendix's milestone).

**Notes for this cluster.**

- A proposal is a measurement, a probe, a simulation at a higher tier or a cell to split, each with a testable
  prediction per mechanism. The output is a planning signal: citing a value-of-information figure as a verdict is
  refused downstream (`FROB-REF-872`, by the verbs whose verdict parameters carry the mark, N28c-12).
- Owner (DESIGN, with the reason): the handler is in `frob-mission`. It reaches the mission chapter's proposal
  builders (N25c-18) and the expert grid (N25cx-02, `crates/frob-mission/src/suggest/`), both in `frob-mission`,
  and hands the list to `frob-opt`'s admission check and ranking (N28c-13, BD-351), a lower layer; a handler in
  `frob-opt` could not call the higher layer. The proposal record `frob.proposal/1` stays defined once, in
  `frob-opt`. An earlier draft named `frob-opt`.
- Milestone (DESIGN): the row is registered at M3, the milestone of its first form, because the expert grid is
  named by the M3 milestone list and needed by GATE-TETHER-15 and by KILL-AUGUR-PROBE's fallback at M3; a row
  registered only at M4 would leave `frob experiment suggest` an unknown verb at M3. The field `first_milestone`
  stays M4, so the milestone histogram of the registry (NJL-03) is unchanged.

## Acceptance criteria

- [ ] `registries/verbs.toml` holds exactly one entry with `cluster = "experiment"` (1 `read`), each with every
      field of the entry schema, an example and a task-cluster list; the alias lint finds no second spelling.
- [ ] `frob-surface-diff parity --cluster experiment` exits 0 in process (`frob-mcp` over stdio) and through `frobd
      --mcp`: registry, CLI and MCP each count 1, and every difference list is empty apart from the harness's closed
      exemptions.
- [ ] Every verb has an end-to-end script: `scripts/e2e/verbs_experiment.sh` runs each verb of the cluster (one),
      and from a verb's first milestone a capability script of its implementing bead exists (`frob-surface-diff
      e2e-coverage`).
- [ ] `frob <verb> --help --json` equals `fixtures/golden/verbs/experiment/<verb>.help.json` byte for byte for each
      of the one, on two runs, debug and release, Linux and macOS; a planted renamed output property is classified
      `breaking` and refused.
- [ ] A verb not yet implemented answers `FROB-REF-724`, exit 5, with one envelope on both surfaces; an implemented
      verb runs its registry example with the result its implementing bead specifies; a misspelt parameter is
      `FROB-REF-802`, exit 3.
- [ ] The generated refusal table lists for each verb exactly its chapter codes plus the codes appended for its
      kind and fields (NJL-01), and every code exists in `registries/refusals.toml`.
- [ ] The gate row is recorded with the measured counts by a session that did not write the rows.

## Test plan

### Unit tests

`cargo test -p frob-verbs --test cluster_experiment` (the real generated table; no mocks; planted registries are
TOML strings labelled `synthetic`):
- `rows_present`: exactly 1 row in the cluster, with the names of the rows table; a planted extra row or a removed
  row fails.
- `kinds_milestones_tiers`: each row's kind, first milestone and tier equal the rows table (for example
  `experiment.suggest` is `read` with `first_milestone` M4, registered at M3).
- `schemas_and_scopes`: input schemas are `frob.<name>.in/1`; output schemas and capabilities equal the
  machine-fields table.
- `refusal_lists`: each row's chapter codes equal its block (`experiment.suggest`: 859, 883); the appended codes
  equal the rule for its kind and fields; 872 is not listed (N28-REF's `verbs_lists_agree`); the pending name
  `prediction-after-outcome` is on no row's list, and a planted slug on the list fails.
- `inputs_present`: the generated input schema of `experiment.suggest` has `budget` with the keys `wall_s`, `gpu_s`
  and `cpu_core_s` (an unknown key is 802), `scope` with the value `ignore`, and `tie_break` with the values
  `largest-gap` (the default) and `smallest-radius` (`--tie-break nearest` is 802).
- `early_form`: with the expert grid in the build and BD-351 absent, `frob capabilities --json` lists
  `experiment.suggest` as implemented with `form: "expert-grid"`, the grid call of N25cx-02 answers, and a ranked
  request answers `FROB-REF-724` naming the form; the row's owner is `frob-mission`.
- `mcp_names_one_to_one`: the tool names are the mapped names (`experiment_suggest`), with no collision against any
  other cluster.
- `task_clusters_and_examples`: every row sits in a task cluster of at most seven tools or is a base tool; each
  example parses against the generated CLI and its command path is the row's own.
- `kind_behaviour`: a verb whose capability is absent returns code 724, exit 5, one envelope.

### End-to-end script and logging

`scripts/e2e/verbs_experiment.sh` (header: `bead=VERBS-experiment milestone=M3
labels=tether,verbs,verb:experiment.suggest gates=GATE-TETHER-02 seed=1 needs=cpu,mcp,frobd
plants=alias,missing-row,renamed-field`) sources the shared helper and `scripts/e2e/verbs_lib.sh` and calls
`verbs_cluster_conformance experiment`. Steps: (1) `frob capabilities --json` and `frob robot-docs --json` list the
one verb with kind and milestone; (2) `frob-surface-diff parity --cluster experiment` over stdio and through `frobd
--mcp`, every difference list empty; (3) `frob-surface-diff snapshot --check --cluster experiment`; (4) each verb's
registry example verbatim, expecting a result (exit 0 or 2), a job handle (exit 8, then `frob job wait`) or
`FROB-REF-724` exactly when the capability report says the verb is absent; (5) for each implemented verb the same
call as an MCP tool, the two envelopes agreeing on `verb`, `ok`, `exit`, `result`, `refusal`, `evidence`, `bound`,
`warnings` and `next_actions`; (6) per verb one call with a misspelt parameter (`e2e_expect_refusal FROB-REF-802
3`); (7) one envelope per call and no exit code 1.

Log: `target/e2e-logs/verbs_experiment/<utc>.ndjson` with a transcript beside it. Each `cmd` line has the exact
command, exit code, wall time and the envelope's `evidence` block; each `assert` has observed and expected values
and the tag `--gate GATE-TETHER-02`; the metrics `registry_verbs`, `cli_verbs` and `mcp_tools` are logged against 1
and the difference counts against 0; refusals are logged with code and first repair; a failure logs the first
differing verb and field; the `summary` line gives assertions passed and failed and the gate touched.

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

- Frontier: the mechanism carries its pilot and gate in its owner beads; the row stays registered if the method is
  cut, returning the comparators (direct measurement) only.
- Closing rule: the bead closes at M3, when the row is registered, the surfaces match and the verb passes its
  example (in its expert-grid form, or the not-implemented refusal naming the ranked form). The per-milestone
  surface beads (BD-312 at M3, BD-376 at M4) re-run this script as the forms land; that is how GATE-TETHER-02 is
  re-measured.
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

Appendix J (J.1 entry form; J.2 inventory rows; J.5 task clusters), §28.16, §25.14, §25.3.1, §32.4, §32.6, §32.12.3,
§32.18, §32.21; GATE-TETHER-02; INV-TETHER-01, -02; KA-755.
