---
key: "VERBS-solve"
title: "frob-verbs registry: cluster `solve`, one row (`solve.run`), with parity, e2e and schema-snapshot checks"
type: "task"
priority: 2
milestone: "M4"
labels: ["m4", "tether", "frob-verbs", "solid", "registry", "verb", "test"]
parent: "WP-62"
deps: ["BD-022", "NJL-01", "NJL-02", "NJL-04", "NJL-05", "STD-E2E"]
needs: []
related: []
gates: ["GATE-TETHER-02"]
plan_refs: ["Appendix J (J.1, J.2, J.3, J.5)", "§32.6.2", "§32.6", "§32.12.3", "§32.21 (GATE-TETHER-02)"]
---
## Background

FrankenRobotics (a robotics simulation and design system) is reached through the command line `frob`, the daemon
`frobd` and the MCP server `frob-mcp` (the Model Context Protocol, by which an agent harness lists and calls tools).
Every capability is a verb, `cluster.verb`, declared once in `registries/verbs.toml` (crate `frob-verbs`, tier
Solid). The CLI, the daemon protocol, the MCP tools, the schemas, the refusal table and the agent guide are
generated from that file, and gate GATE-TETHER-02 compares the served surfaces with it. The registry holds 244 verbs
in 44 clusters (namespaces): the 243 of the specification and `artifact.import` (VERBS-artifact); the
count of record is NJL-03's.

This bead is the registry of record for the cluster `solve`. The cluster covers the one-call pipeline: from a task
specification and a budget to a ranked set of designs with verdicts. It has one verb (1 `run`); the handlers live in
`frob-verbs`. The bead carries every row completely, with the three checks that keep the rows true: the generated
CLI commands and MCP tools match the list; every verb has an end-to-end script; the schema printed by `frob <verb>
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

In: the one registry entry of cluster `solve`; the MCP tool names; the cluster's conformance script; the help-schema
snapshots; each verb's answer before its milestone. Out: behaviour (the implementing beads named below); the entry
schema and lints (NJL-01); the conformance harness (NJL-02); the alias lint (NJL-04); task clusters (NJL-05,
NJL-06); generation of the surfaces (the Tether seed beads).

## Technical approach

**The rows.**

| Verb | Kind | First milestone | Tier | Purpose | Defined in |
|---|---|---|---|---|---|
| `solve.run` | run | M4 | F | One call from a task and a budget to a ranked set with verdicts; the agent-facing orchestration verb, which calls `opt.cascade` [F] | §32.6.2 |

**Contract of each verb.** Evidence labels used below (the `evidence` block of the envelope):
- *study*: a Design comparison of class Tested (Estimated if any input is `assumed`); tier and determinism the
  weakest of the arms; `provisional` until promoted to the referee.

**`solve.run`** (MCP tool `solve_run`; owner `frob-verbs`)
- Inputs (BD-372): `task` (a task or mission identifier); `candidates` (a list of design identifiers, a generator
  identifier with a count, or a design family); `constraints` (requirement overrides as expressions of the
  requirement language: data, never code); `budget` (required; named units); `stop_at` (`check`, `sweep`, `race`,
  `promote`, `qualify`; default `promote`); `top_k` (default 3); `--estimate` (also lists the scopes of the
  verbs it will call); idempotency key.
- Outputs: a ranked set of designs with verdicts, bounded by `top_k` (default 3) and returned whole in the job's
  result (the row carries no `paginated` field: NJL-01's variation 1 applies to the `read` kind only), with
  provenance per number; orchestration that calls `opt.cascade` and adds no evidence of its own.
  Evidence: *study*.
- Chapter refusals: `FROB-REF-701` (the estimate's upper end exceeds the remaining budget; the estimate is
  attached); `FROB-REF-704` (a campaign-class job without an approved compute envelope). Both are also in the
  `run` kind's appended set; the chapter table cites them for this verb, and the generated list holds each code
  once.
- Scopes of the called verbs: `solve.run` acts under the caller's token and never widens it. Before any job is
  submitted it checks that the token also holds the scopes of the verbs its manifest will call (`opt:run`,
  `sim:run`, `study:submit`, and `qual:run` when `stop_at = qualify`) and otherwise refuses with the appended
  `FROB-REF-720` naming the first missing scope. The row declares only `solve:run`.
- Implemented by: BD-372. Decided by: N32ap1-01 (KILL-SOLVE clause 1, the paper ceiling, read before the verb is
  funded) and N32a-32 (clause 2, the A/B, and the final status).
- Removal rule: if clause 1 fires (`fires(ceiling)`), the verb is never built and this row is removed from the
  registry before the M4 verb set is frozen, in the same change as the decision (N32ap1-01's decision record);
  `solve_run` leaves the `search` task cluster, and `frob capabilities` reports the capability `killed` with
  the fallback "prompts". If clause 2 fires, the entry is removed at the next registry freeze (N32a-32's default)
  and the same report follows. Either way the cluster's counts in this bead's tests become 0 and its script
  asserts the killed report instead of the example. The withdrawal is whole (NJL-05's ruling; BD-376 records
  it in the freeze file): the row does not stay registered answering `FROB-REF-724`, because a registered row
  with no seat in a task cluster fails the lint `verb-without-task-cluster`. In the same change the name
  `solve.run` enters the retired-name table of NJL-04 with `use = ["opt.cascade"]`, so `frob solve run …` and
  the MCP call answer `FROB-REF-801` (unknown verb), exit 3, with the first repair naming `opt.cascade`; and
  `opt_cascade` takes the vacated seat in `search`, which keeps seven tools (VERBS-opt's row gains the cluster).

**Machine fields of each row.**

| Verb | Capabilities | Output schema | Task clusters | Example |
|---|---|---|---|---|
| `solve.run` | `solve:run` | `frob.solve.run.out/1` | `search` | `frob solve run --task stairs-0.4 --budget gpu_s=3600 --key sol-1 --json` |

The remaining fields follow by rule: `input_schema` is `frob.<verb name>.in/1`; `refusals` is the chapter list of
the verb's block; `idempotent` is `true` for `read` and `admin` and `false` for `run` and `mutate` (which take an
idempotency key); `cost_model` is `static: …` for `read`, `mutate` and `admin` and the `--estimate` cost model for
`run`; a `run` row carries `wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }`; `owner` is the
crate of the block; `purpose` is the Purpose column.

One entry in full (the others follow field for field):

```toml
[[verb]]
name = "solve.run"
cluster = "solve"
kind = "run"
tier = "F"
capabilities = ["solve:run"]
input_schema = "frob.solve.run.in/1"
output_schema = "frob.solve.run.out/1"
refusals = ["FROB-REF-701", "FROB-REF-704"]
idempotent = false
cost_model = "estimated: `--estimate` returns CPU core-seconds, GPU seconds, memory and disk with a conformal interval"
first_milestone = "M4"
task_clusters = ["search"]
examples = ["frob solve run --task stairs-0.4 --budget gpu_s=3600 --key sol-1 --json"]
owner = "frob-verbs"
purpose = "One call from a task and a budget to a ranked set with verdicts; the agent-facing orchestration verb, which calls `opt.cascade` [F]"
wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }
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
4. Rows are entered when the cluster is first registered (M4); each row's tool is added to the task-cluster
   tables its `task_clusters` names in the same change (NJL-05's staging rule). A verb whose handler is not in
   the build answers one envelope with `FROB-REF-724`, exit 5, on both surfaces, and `frob capabilities --json`
   lists it as absent; a reduced form that lands before the row's `first_milestone` answers and is listed as
   implemented with its form named (NJL-01, NJL-03: the field keeps the appendix's milestone).

**Notes for this cluster.**

- `solve.run` is orchestration, not a mechanism: every number in its answer comes from the verbs it calls
  (`opt.cascade` and the rest), and it adds no evidence of its own.
- HYPOTHESIS: about five-fold fewer serial agent turns per accepted design; the test is the second arm of
  GATE-TETHER-13. Ceiling, stated before any test: the fraction of turns in recorded sessions that carry no
  decision; below 30% the ceiling is under 1.4-fold and the work is not funded.

## Acceptance criteria

- [ ] `registries/verbs.toml` holds exactly one entry with `cluster = "solve"` (1 `run`), each with every field of
      the entry schema, an example and a task-cluster list; the alias lint finds no second spelling.
- [ ] `frob-surface-diff parity --cluster solve` exits 0 in process (`frob-mcp` over stdio) and through `frobd
      --mcp`: registry, CLI and MCP each count 1, and every difference list is empty apart from the harness's closed
      exemptions.
- [ ] Every verb has an end-to-end script: `scripts/e2e/verbs_solve.sh` runs each verb of the cluster (one), and
      from a verb's first milestone a capability script of its implementing bead exists (`frob-surface-diff
      e2e-coverage`).
- [ ] `frob <verb> --help --json` equals `fixtures/golden/verbs/solve/<verb>.help.json` byte for byte for each of
      the one, on two runs, debug and release, Linux and macOS; a planted renamed output property is classified
      `breaking` and refused.
- [ ] A verb not yet implemented answers `FROB-REF-724`, exit 5, with one envelope on both surfaces; an implemented
      verb runs its registry example with the result its implementing bead specifies; a misspelt parameter is
      `FROB-REF-802`, exit 3; a `run` or `mutate` call without a key is `FROB-REF-805`, exit 3.
- [ ] The generated refusal table lists for each verb exactly its chapter codes plus the codes appended for its
      kind and fields (NJL-01), and every code exists in `registries/refusals.toml`.
- [ ] The gate row is recorded with the measured counts by a session that did not write the rows.

## Test plan

### Unit tests

`cargo test -p frob-verbs --test cluster_solve` (the real generated table; no mocks; planted registries are TOML
strings labelled `synthetic`):
- `rows_present`: exactly 1 row in the cluster, with the names of the rows table; a planted extra row or a removed
  row fails.
- `kinds_milestones_tiers`: each row's kind, first milestone and tier equal the rows table (for example `solve.run`
  is `run` at M4).
- `schemas_and_scopes`: input schemas are `frob.<name>.in/1`; output schemas and capabilities equal the
  machine-fields table.
- `refusal_lists`: each row's chapter codes equal its block (`solve.run`: 701, 704); the appended codes equal the
  rule for its kind and fields, with no 806 or 807 (a `run` row is not paginated), and the generated list names
  each code once.
- `called_scopes_checked`: a token holding `solve:run` but not `sim:run` is refused with 720 naming `sim:run`
  before any job is submitted; `--estimate` lists the four called scopes.
- `branch_consistency`: with KILL-SOLVE recorded as fired in the register, a registry that still lists
  `solve.run` fails (N32a-32's `registry_matches_branch`, run here against the cluster); on the fired branch
  the retired-name table holds `solve.run` with `use = ["opt.cascade"]`, a call of `frob solve run` answers
  `FROB-REF-801`, exit 3, with `opt.cascade` in its first repair, and the `search` cluster has seven tools
  with `opt_cascade` among them.
- `mcp_names_one_to_one`: the tool names are the mapped names (`solve_run`), with no collision against any other
  cluster.
- `task_clusters_and_examples`: every row sits in a task cluster of at most seven tools or is a base tool; each
  example parses against the generated CLI and its command path is the row's own.
- `kind_behaviour`: a `run` row has the six common properties (`sync_wait_ms`, `idempotency_key`, `budget`,
  `estimate_only`, `on_cancel`, `envelope_scope`), with `idempotency_key` outside the schema's `required` array
  and annotated `x-frob-key: "required"`, and refuses `sync_wait_ms = 30001` with code 802; a verb whose
  capability is absent returns code 724, exit 5, one envelope.

### End-to-end script and logging

`scripts/e2e/verbs_solve.sh` (header: `bead=VERBS-solve milestone=M4 labels=tether,verbs,verb:solve.run
gates=GATE-TETHER-02 seed=1 needs=cpu,mcp,frobd plants=alias,missing-row,renamed-field`) sources the shared helper
and `scripts/e2e/verbs_lib.sh` and calls `verbs_cluster_conformance solve`. Steps: (1) `frob capabilities --json`
and `frob robot-docs --json` list the one verb with kind and milestone; (2) `frob-surface-diff parity --cluster
solve` over stdio and through `frobd --mcp`, every difference list empty; (3) `frob-surface-diff snapshot --check
--cluster solve`; (4) each verb's registry example verbatim, expecting a result (exit 0 or 2), a job handle (exit 8,
then `frob job wait`) or `FROB-REF-724` exactly when the capability report says the verb is absent; (5) for each
implemented verb the same call as an MCP tool, the two envelopes agreeing on `verb`, `ok`, `exit`, `result`,
`refusal`, `evidence`, `bound`, `warnings` and `next_actions`; (6) per verb one call with a misspelt parameter
(`e2e_expect_refusal FROB-REF-802 3`) and, for `run` and `mutate` verbs, one without a key (`e2e_expect_refusal
FROB-REF-805 3`); (7) one envelope per call and no exit code 1.

Log: `target/e2e-logs/verbs_solve/<utc>.ndjson` with a transcript beside it. Each `cmd` line has the exact command,
exit code, wall time and the envelope's `evidence` block; each `assert` has observed and expected values and the tag
`--gate GATE-TETHER-02`; the metrics `registry_verbs`, `cli_verbs` and `mcp_tools` are logged against 1 and the
difference counts against 0; refusals are logged with code and first repair; a failure logs the first differing verb
and field; the `summary` line gives assertions passed and failed and the gate touched.

### Known answers and negative controls

- Known answers: the cluster has 1 verb (1 `run`); the tables above. KA-755 (the exit-code table on malformed,
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

- Kill gate KILL-SOLVE (the ceiling is under 1.4-fold, or agents with `solve.run` do no better than agents driving
  the verbs): fallback, the MCP prompts; the row is then withdrawn from the registry by the removal rule above
  (retired name with `use = ["opt.cascade"]`, `opt_cascade` seated in `search`). It does not remain as a row
  that answers the not-implemented refusal: an earlier wording of this bullet said so, and a registered row
  without a task-cluster seat fails NJL-05's lint.
- Closing rule: the bead closes at M4, when the rows are registered, the surfaces match and every verb passes its
  example or answers the not-implemented refusal. The per-milestone surface beads (BD-376) re-run this script as
  verbs land; that is how GATE-TETHER-02 is re-measured.
- Scope names are proposals (`<cluster>:<read|run|write|admin>` plus the named scopes of the surface) until the
  capability bead BD-258 fixes the scope registry; a `run` or `write` scope of a cluster implies its `read` scope.
  Inputs and outputs above name contents, not flag spellings, except where a flag is quoted; the implementing bead
  fixes the spellings and the snapshot then pins them (additive changes keep the schema's major version; anything
  else raises it).

## References

Appendix J (J.1 entry form; J.2 inventory rows; J.5 task clusters), §32.6.2, §32.4, §32.6, §32.12.3, §32.18, §32.21;
GATE-TETHER-02; INV-TETHER-01, -02; KA-755.
