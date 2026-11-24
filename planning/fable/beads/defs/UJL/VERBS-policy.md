---
key: "VERBS-policy"
title: "frob-verbs registry: cluster `policy`, one row (`policy.apply`), with parity, e2e and schema-snapshot checks"
type: "task"
priority: 2
milestone: "M4"
labels: ["m4", "tether", "cortex", "frob-verbs", "solid", "registry", "verb", "test"]
parent: "WP-62"
deps: ["BD-022", "N23-REF", "N28-REF", "NJL-01", "NJL-02", "NJL-04", "NJL-05", "STD-E2E"]
needs: []
related: []
gates: ["GATE-TETHER-02"]
plan_refs: ["Appendix J (J.1, J.2, J.3, J.5)", "§23.11", "§32.6", "§32.12.3", "§32.21 (GATE-TETHER-02)"]
---
## Background

FrankenRobotics (a robotics simulation and design system) is reached through the command line `frob`, the daemon
`frobd` and the MCP server `frob-mcp` (the Model Context Protocol, by which an agent harness lists and calls tools).
Every capability is a verb, `cluster.verb`, declared once in `registries/verbs.toml` (crate `frob-verbs`, tier
Solid). The CLI, the daemon protocol, the MCP tools, the schemas, the refusal table and the agent guide are
generated from that file, and gate GATE-TETHER-02 compares the served surfaces with it. The registry holds 244 verbs
in 44 clusters (namespaces): the 243 of the specification and `artifact.import` (VERBS-artifact); the
count of record is NJL-03's.

This bead is the registry of record for the cluster `policy`. The cluster covers one verb that applies a family
policy to a design, with or without fine-tuning. It has one verb (1 `run`); the handlers live in `frob-learn`. The
bead carries every row completely, with the three checks that keep the rows true: the generated CLI commands and MCP
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
inventory); one bead for all the rows (too large to review).

## Scope

In: the one registry entry of cluster `policy`; the MCP tool names; the cluster's conformance script; the
help-schema snapshots; each verb's answer before its milestone. Out: behaviour (the implementing beads named below);
the entry schema and lints (NJL-01); the conformance harness (NJL-02); the alias lint (NJL-04); task clusters
(NJL-05, NJL-06); generation of the surfaces (the Tether seed beads).

## Technical approach

**The rows.**

| Verb | Kind | First milestone | Tier | Purpose | Defined in |
|---|---|---|---|---|---|
| `policy.apply` | run | M4 | F | Apply a family policy to a design, with or without fine-tuning; refused out of distribution | §23.11 |

**Contract of each verb.** Evidence labels used below (the `evidence` block of the envelope):
- *screening*: class Estimated: a screening or planning signal that may discard under the ladder's rules and may not
  establish a comparison.

**`policy.apply`** (MCP tool `policy_apply`; owner `frob-learn`)
- Inputs: the family policy (positional: an identity such as `fam:quad-1`, or `class:<name>` for the class's
  current family policy); `--design`; optional `--task`, `--fine-tune` (`none` | `steps:<n>` | `fraction:<f>` of
  the class's reference specialist steps, 0.05 being the registered rung | `to-target`), `--seed`,
  `--score-for-ladder select|discard` (the use the score is asked for, to select survivors or to discard;
  DESIGN of N23a-24 and N23a-25, because the division of 537 and 545 below depends on that use and a bare
  flag cannot carry it; the flag without a value, or another value, is 802), profile; budget.
- Outputs: a `BoundPolicy` identity (the family weights with the design's token table and gather/scatter maps,
  stored with a manifest whose donor is the family policy) and the report `in_distribution` (inside, margins per
  parameter, `near-support-edge`); with `--task`, a short evaluation on the development stream, labelled
  `unranked` unless `--score-for-ladder` passes its guard; with `--fine-tune`, a training job (exit 8 while
  running) whose result is a specialist checkpoint with its own manifest. Exit 0 or 2 on completion (flags
  `near-support-edge`, `unranked`); 5 when no family policy is admitted for the class (the capability refusal,
  never 534). Evidence: *screening*.
- Chapter refusals: `FROB-REF-531` (a policy's mapping table does not match the design, or cannot be built for an
  edit); `FROB-REF-534` (a family policy applied outside its class or its pretraining distribution);
  `FROB-REF-537` (`--score-for-ladder select` or `discard` with no rank-agreement record for the class, task
  family and policy, or `--score-for-ladder discard` through a record whose mode is `uninformative`; until the
  record lookup of N23a-24 exists no record can exist and the flag always refuses); `FROB-REF-545`
  (`--score-for-ladder select` through a record whose mode is not `selective`; repairs: let calibration
  accumulate through the audit lane, accept "k survivors"; exit 3). The division of 537 and 545 is the
  ladder's mode automaton (N28b-23), which N23a-24 and N23a-25 implement: one request raises one of them.
- Cited, not raised: `FROB-REF-541` (outside generator support). By the division of the learning refusal registry
  (N23-REF), when the generator's derivation test reports a design outside its support the verb raises 534 and
  cites the 541 finding in the refusal's payload; 541 is therefore not on the row (the lint would report it as a
  listed code with no raising site).
- Correction (round 1): the row formerly listed 541 among its refusals and lacked 537, which N23a-25 raises.
- Implemented by: BD-322 (the binding and the mapping table), N23a-25 (the verb), N23a-24 (the rank-agreement
  record lookup that the 537 and 545 guards read).

**Machine fields of each row.**

| Verb | Capabilities | Output schema | Task clusters | Example |
|---|---|---|---|---|
| `policy.apply` | `policy:run` | `frob.policy.apply.out/1` | `inherit` | `frob policy apply fam:quad-1 --design quad-a --key pa-1 --json` |

The remaining fields follow by rule: `input_schema` is `frob.<verb name>.in/1`; `refusals` is the chapter list of
the verb's block; `idempotent` is `true` for `read` and `admin` and `false` for `run` and `mutate` (which take an
idempotency key); `cost_model` is `static: …` for `read`, `mutate` and `admin` and the `--estimate` cost model for
`run`; a `run` row carries `wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }`; `owner` is the
crate of the block; `purpose` is the Purpose column.

One entry in full (the others follow field for field):

```toml
[[verb]]
name = "policy.apply"
cluster = "policy"
kind = "run"
tier = "F"
capabilities = ["policy:run"]
input_schema = "frob.policy.apply.in/1"
output_schema = "frob.policy.apply.out/1"
refusals = ["FROB-REF-531", "FROB-REF-534", "FROB-REF-537", "FROB-REF-545"]
idempotent = false
cost_model = "estimated: `--estimate` returns CPU core-seconds, GPU seconds, memory and disk with a conformal interval"
first_milestone = "M4"
task_clusters = ["inherit"]
examples = ["frob policy apply fam:quad-1 --design quad-a --key pa-1 --json"]
owner = "frob-learn"
purpose = "Apply a family policy to a design, with or without fine-tuning; refused out of distribution"
wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }
```

**Conventions the rows follow.**

1. The MCP tool name is the verb name with `.` and `-` replaced by `_`. The output on the wire is one envelope
   `frob.envelope/1` with `result` specialized to the output schema; its `evidence` block is a claim product (the
   weakest class, tier and determinism class among the inputs).
2. The generator adds the common input properties of the kind, as NJL-01's kinds table lists them. `run`:
   `sync_wait_ms` (from 0 to the row's `wait.mcp_max_ms`, at most 30,000; MCP default 5,000; CLI default to
   completion), `idempotency_key` (CLI `--key`), `budget`, `estimate_only` (CLI `--estimate`), `on_cancel`
   (`leave`, the default, or `cancel_job`) and `envelope_scope` (CLI `--envelope-scope
   <gate:…|campaign:…|pretrain:…>`, the approved compute-envelope scope the run belongs to); `mutate`:
   `idempotency_key` and `expected_parent`; `read` and `admin`: none. `idempotency_key` is in no JSON-schema
   `required` array: the annotation `x-frob-key: "required"` carries the rule, and the admission transaction
   refuses a missing key with 805 on both surfaces. The generator appends the surface-wide refusals by kind:
   `FROB-REF-720` (scope missing), 723 (token invalid), 724 (not implemented in this build) and 802 (schema
   failure, including an unknown parameter) for every verb; 701 to 704 (budget and admission), 761 (above the
   compute envelope in force), 803 (key reused with different content) and 805 (missing key) for `run`; 725,
   803, 804 (stale parent) and 805 for `mutate`. What fields and registry tables add is NJL-01's variation table,
   the one list of it: 806 and 807, with `cursor`, `limit` and `follow`, for a paginated read (variation 1);
   `estimate_only` on a non-`run` entry with `estimable = true` (2); `x-frob-key: "optional"` and no 805 on a
   `key_optional` entry (3); 808 for an entry with an `int53` property at any depth of its input schema, whatever
   its kind, so every `run` row through `sync_wait_ms` (4); `action` for an `actions` list (5); 790 for a
   non-empty `needs_groups` (6); 891 to 893 for artifact `params` (7, 8); 734 and 828 for a parameter that
   accepts a path to a file (`artifact-input<F>`, `artifact-identity<F>`, `foreign-file`) (9). Variations 10 to
   13 are read from NJL-01's table with the beads that land them.
3. Exit codes: 0 done; 2 done with findings; 3 refused input (1xx to 5xx, and 8xx other than conflicts); 4 budget
   (70x and 761); 5 capability (72x, 73x); 6 device (6xx); 7 conflict (803, 804, 809); 8 accepted and still running; 9
   indeterminate; never 1 on purpose.
4. Rows are entered when the cluster is first registered (M4); each row's tool is added to the task-cluster
   tables its `task_clusters` names in the same change (NJL-05's staging rule). A verb whose handler is not in
   the build answers one envelope with `FROB-REF-724`, exit 5, on both surfaces, and `frob capabilities --json`
   lists it as absent; a reduced form that lands before the row's `first_milestone` answers and is listed as
   implemented with its form named (NJL-01, NJL-03: the field keeps the appendix's milestone).

**Notes for this cluster.**

- Applying a family policy outside its morphology class or its pretraining distribution is refused (`FROB-REF-534`),
  and a mapping table that does not match the design is refused (`FROB-REF-531`).
- A score from `policy.apply` enters a ladder only through `--score-for-ladder` and the rank-agreement guard
  (`FROB-REF-537`, `FROB-REF-545`); otherwise it is labelled `unranked` (advisory use is always allowed).
- Agreement with the studio's refusal registry: 545 is raised on this row by N23a-24 and N23a-25
  (`--score-for-ladder select` through a record whose mode is not `selective`, the division of N28b-23), so
  N28-REF's row for 545 names `policy.apply` in its verbs column beside `opt.cascade`, `opt.start` and
  `eval.rank-agreement`, and the bidirectional cross-check of the two registries holds for it.

## Acceptance criteria

- [ ] `registries/verbs.toml` holds exactly one entry with `cluster = "policy"` (1 `run`), each with every field of
      the entry schema, an example and a task-cluster list; the alias lint finds no second spelling.
- [ ] `frob-surface-diff parity --cluster policy` exits 0 in process (`frob-mcp` over stdio) and through `frobd
      --mcp`: registry, CLI and MCP each count 1, and every difference list is empty apart from the harness's closed
      exemptions.
- [ ] Every verb has an end-to-end script: `scripts/e2e/verbs_policy.sh` runs each verb of the cluster (one), and
      from a verb's first milestone a capability script of its implementing bead exists (`frob-surface-diff
      e2e-coverage`).
- [ ] `frob <verb> --help --json` equals `fixtures/golden/verbs/policy/<verb>.help.json` byte for byte for each of
      the one, on two runs, debug and release, Linux and macOS; a planted renamed output property is classified
      `breaking` and refused.
- [ ] A verb not yet implemented answers `FROB-REF-724`, exit 5, with one envelope on both surfaces; an implemented
      verb runs its registry example with the result its implementing bead specifies; a misspelt parameter is
      `FROB-REF-802`, exit 3; a `run` or `mutate` call without a key is `FROB-REF-805`, exit 3.
- [ ] The generated refusal table lists for each verb exactly its chapter codes plus the codes appended for its
      kind and fields (NJL-01), and every code exists in `registries/refusals.toml`.
- [ ] The script's run leaves a receipt with the measured counts (registry, CLI and MCP each 1; every difference
      list 0) and the controls caught. This bead records no gate value: the M4 surface bead BD-376 records
      GATE-TETHER-02's M4 part from that receipt with `cargo run -p frob-conformance --example record_gate --
      --gate GATE-TETHER-02 --receipt <hash> --part M4`, in a session that did not write the rows (N38b-19).

## Test plan

### Unit tests

`cargo test -p frob-verbs --test cluster_policy` (the real generated table; no mocks; planted registries are TOML
strings labelled `synthetic`):
- `rows_present`: exactly 1 row in the cluster, with the names of the rows table; a planted extra row or a removed
  row fails.
- `kinds_milestones_tiers`: each row's kind, first milestone and tier equal the rows table (for example
  `policy.apply` is `run` at M4).
- `schemas_and_scopes`: input schemas are `frob.<name>.in/1`; output schemas and capabilities equal the
  machine-fields table.
- `refusal_lists`: each row's chapter codes equal its block (`policy.apply`: 531, 534, 537, 545); the appended codes
  equal the rule for its kind and fields; a planted 541 on the row is reported by the lint as a code with no raising site
  (N23-REF: the verb cites 541, it raises 534).
- `inputs_by_row`: `policy.apply` accepts a positional `fam:…` or `class:<name>`, `--design`, `--task`,
  `--fine-tune none|steps:<n>|fraction:<f>|to-target`, `--seed`, `--score-for-ladder select` and
  `--score-for-ladder discard`; `--score-for-ladder` without a value and `--score-for-ladder rank` are 802.
- `mcp_names_one_to_one`: the tool names are the mapped names (`policy_apply`), with no collision against any other
  cluster.
- `task_clusters_and_examples`: every row sits in a task cluster of at most seven tools or is a base tool; each
  example parses against the generated CLI and its command path is the row's own.
- `kind_behaviour`: a `run` row has the six common properties of conventions item 2, `idempotency_key` in no
  `required` array and annotated `x-frob-key: "required"`, `on_cancel` admitting exactly `leave` and
  `cancel_job`, and refuses `sync_wait_ms = 30001` with code 802; a
  verb whose capability is absent returns code 724, exit 5, one envelope.

### End-to-end script and logging

`scripts/e2e/verbs_policy.sh` (header: `bead=VERBS-policy milestone=M4 labels=tether,verbs,verb:policy.apply
gates=GATE-TETHER-02 seed=1 needs=cpu,mcp,frobd plants=alias,missing-row,renamed-field`) sources the shared helper
and `scripts/e2e/verbs_lib.sh` and calls `verbs_cluster_conformance policy`. Steps: (1) `frob capabilities --json`
and `frob robot-docs --json` list the one verb with kind and milestone; (2) `frob-surface-diff parity --cluster
policy` over stdio and through `frobd --mcp`, every difference list empty; (3) `frob-surface-diff snapshot --check
--cluster policy`; (4) each verb's registry example verbatim, expecting a result (exit 0 or 2), a job handle (exit
8, then `frob job wait`) or `FROB-REF-724` exactly when the capability report says the verb is absent; (5) for each
implemented verb the same call as an MCP tool, the two envelopes agreeing on `verb`, `ok`, `exit`, `result`,
`refusal`, `evidence`, `bound`, `warnings` and `next_actions`; (6) per verb one call with a misspelt parameter
(`e2e_expect_refusal FROB-REF-802 3`) and, for `run` and `mutate` verbs, one without a key (`e2e_expect_refusal
FROB-REF-805 3`); (7) one envelope per call and no exit code 1.

Log: `target/e2e-logs/verbs_policy/<utc>.ndjson` with a transcript beside it. Each `cmd` line has the exact command,
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

- Frontier: pretraining of a family policy needs an approved compute envelope (OPEN-CORTEX-1); without it, or if
  kill gate KILL-FAMILY fires, the milestone ships the variant with no family policy and this row answers the
  not-implemented refusal.
- Closing rule: the bead closes at M4, when the rows are registered, the surfaces match and every verb passes its
  example or answers the not-implemented refusal. The per-milestone surface beads (BD-376) re-run this script as
  verbs land; that is how GATE-TETHER-02 is re-measured.
- Scope names follow the naming rule decided in BD-258: a row cites scopes of its own cluster by kind (`read` →
  `<cluster>:read`; `run` → `<cluster>:run` or a named spending scope; `mutate` → `<cluster>:write` or a named write
  scope; `admin` → `<cluster>:admin`, `job:cancel` or a named admin scope), plus `export:*` when the row exports and
  `private:*` where privacy applies, never a scope named after the object read or the engine spent on; N32b-01's
  catalogue generates the forms. `<cluster>:run` and `<cluster>:write` imply `<cluster>:read`, closed when a set is
  stored (at minting and at derivation, `Scopes::close_implied`), so checks stay plain set inclusion. Inputs and
  outputs above name contents, not flag spellings, except where a flag is quoted; the implementing bead fixes the
  spellings and the snapshot then pins them (additive changes keep the schema's major version; anything else raises
  it).

## References

Appendix J (J.1 entry form; J.2 inventory rows; J.5 task clusters), §23.11, §32.4, §32.6, §32.12.3, §32.18, §32.21;
GATE-TETHER-02; INV-TETHER-01, -02; KA-755.
