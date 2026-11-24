---
key: "VERBS-result"
title: "frob-verbs registry: cluster `result`, two rows, with parity, e2e and schema-snapshot checks"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "tether", "atelier", "frob-verbs", "solid", "registry", "verb", "test"]
parent: "WP-06"
deps: ["BD-022", "N12-REF", "N24-REF", "N25-REF", "N28-REF", "N29-REF", "NJL-01", "NJL-02", "NJL-04", "NJL-05", "STD-E2E"]
needs: []
related: []
gates: ["GATE-TETHER-02"]
plan_refs: ["Appendix J (J.1, J.2, J.3, J.5)", "§28.13", "§32.6", "§32.12.3", "§32.21 (GATE-TETHER-02)"]
---
## Background

FrankenRobotics (a robotics simulation and design system) is reached through the command line `frob`, the daemon
`frobd` and the MCP server `frob-mcp` (the Model Context Protocol, by which an agent harness lists and calls tools).
Every capability is a verb, `cluster.verb`, declared once in `registries/verbs.toml` (crate `frob-verbs`, tier
Solid). The CLI, the daemon protocol, the MCP tools, the schemas, the refusal table and the agent guide are
generated from that file, and gate GATE-TETHER-02 compares the served surfaces with it. The registry holds 244 verbs
in 44 clusters (namespaces): the 243 of the specification and `artifact.import` (VERBS-artifact); the
count of record is NJL-03's.

This bead is the registry of record for the cluster `result`. The cluster covers stored results: showing a result
with its claim product and dependencies, and revalidation after a change. It has two verbs (1 `read`, 1 `run`); the
handlers live in `frob-verbs`. The bead carries every row completely, with the three checks that keep the rows true:
the generated CLI commands and MCP tools match the list; every verb has an end-to-end script; the schema printed by
`frob <verb> --help --json` is stable.

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

In: the two registry entries of cluster `result`; the MCP tool names; the cluster's conformance script; the
help-schema snapshots; each verb's answer before its milestone. Out: behaviour (the implementing beads named below);
the entry schema and lints (NJL-01); the conformance harness (NJL-02); the alias lint (NJL-04); task clusters
(NJL-05, NJL-06); generation of the surfaces (the Tether seed beads).

## Technical approach

**The rows.**

| Verb | Kind | First milestone | Tier | Purpose | Defined in |
|---|---|---|---|---|---|
| `result.revalidate` | run | M4 | S | After a change: which results and certificates survive, which are invalidated, what reruns cost | §28.13 |
| `result.show` | read | M4 | S | A stored result with its claim product and dependencies | §28.13, §32.6 |

**Contract of each verb.** Evidence labels used below (the `evidence` block of the envelope):
- *pass-through*: passes through the claim product of the stored objects it reads and adds no claim of its own; the
  read itself is `exact`.

**`result.revalidate`** (MCP tool `result_revalidate`; owner `frob-verbs`)
- Inputs: the target (positional): a `VerdictId`, a `ClaimId`, a mission answer, or a design by name or version
  (every stored verdict whose evidence closure cites a version of it); the change, exactly one of `--after-edit
  <edit>`, `--since <identity>` (the edits from it to the pinned head), `--between <a> <b>` (two versions of a
  design, card, scene, task, clock or mission), `--amendment field=value,…` (a set of field changes, the
  generated form of the object property), or `--against q:…` (an amended query, for a mission-answer target
  only: the answer is revalidated against that query, the form of N25a-13's engine and of `mission.revalidate`;
  with any other target it is 802 with `pointer: /change/against`); two of them, or none, is the appended 802
  with `pointer: /change` (N28b-32); `--grades <n>` (the highest
  grade allowed, default 2; 3 only with the session flag `frontier-revalidate-implication`); `--run` (submit the
  planned reruns as jobs, N28b-33; until it lands the reruns are listed with reason `no-planner`); `--estimate`
  (the CLI spelling of the common property `estimate_only`); budget; idempotency key. All fields exist from M1,
  so no later engine changes the schema. Each rerun's `command` is printed in the surface's own spelling (`--key`
  for the idempotency key).
- Outputs: per obligation its role (feasibility or optimality, never merged), grade, status (`preserved` or
  `revoked`; never `preserved` without a licence) and licence; `verdict_after`; `invalidated_regions` for a
  mission answer; the reruns with their cost and command; counts by grade. For a design target, one such answer
  per verdict under `verdicts` and the walkthrough `summary`: `inherited` (with the reason, by read set),
  `invalidated` (with the touching field) and `reruns` (count and interval estimate). Exit 0 when every obligation
  is preserved, 2 when one is revoked or to be rechecked. Evidence: *pass-through* (the original class kept for
  preserved obligations only, with `revalidated: { grade_max, licence_ids }`; determinism D0).
- Chapter refusals: `FROB-REF-835` (a verdict is reused after a change without a recorded implication (D-11));
  `FROB-REF-854` (a classifier, screen or certificate reused outside its eligibility); `FROB-REF-869` (the
  target's evidence closure is dangling or tombstoned); `FROB-REF-870` (reuse of a verdict with no licence, or
  under a rule the registry does not know); `FROB-REF-872` (a value-of-information output given where a verdict
  is expected; the target parameter is marked `accepts = "verdict"`, so the advisory type wall of N28c-12
  applies from this verb's first milestone).
- Implemented by: N28b-32 (the surface, M1, mission cells, grades 1 and 2), BD-292 (the engine, M3), BD-347
  (grades 3 and 4, M4); N28b-33 (`--run`, the rerun planner). Extended by: N28c-29 (`same_engine_as =
  "mission.revalidate"`, M3).

**`result.show`** (MCP tool `result_show`; owner `frob-verbs`)
- Inputs: result identifier (a run summary, a study answer, a mission answer, a promotion, a campaign result);
  optional `--depth` (default 1) for the dependency listing; pagination (`paginated = true`: `inputs` is paged,
  and the generator appends 806 and 807).
- Outputs: the result as stored (the same bytes its producing verb returned; it never recomputes and creates no
  job); its claim product in full (kind, class, the weakest input that set the class, tier, determinism, reality
  scope, conditions); `inputs`, every consumed artifact with its own claim product; `standing` (`current`, or
  `invalidated` with the change and the rerun cost). Evidence: *pass-through*.
- Chapter refusals: `FROB-REF-840` (a result of the blinded qualification lane read beyond its verdict and
  receipt); `FROB-REF-869` (citing a claim whose evidence closure is dangling or tombstoned). The scope checked
  is `<cluster>:read` of the result's producing cluster (720 otherwise).
- Implemented by: BD-376.

**Machine fields of each row.**

| Verb | Capabilities | Output schema | Task clusters | Example |
|---|---|---|---|---|
| `result.revalidate` | `result:run` | `frob.result.revalidate.out/1` | `mission`, `repair`, `transfer` | `frob result revalidate quad-tray --since d:7a3e --key rev-1 --json` |
| `result.show` | `result:read` | `frob.result.show.out/1` | `proofs` | `frob result show a:8b27 --json` |

The remaining fields follow by rule: `input_schema` is `frob.<verb name>.in/1`; `refusals` is the chapter list of
the verb's block; `idempotent` is `true` for `read` and `admin` and `false` for `run` and `mutate` (which take an
idempotency key); `cost_model` is `static: …` for `read`, `mutate` and `admin` and the `--estimate` cost model for
`run`; a `run` row carries `wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }`; `owner` is the
crate of the block; `purpose` is the Purpose column.

One entry in full (the others follow field for field):

```toml
[[verb]]
name = "result.revalidate"
cluster = "result"
kind = "run"
tier = "S"
capabilities = ["result:run"]
input_schema = "frob.result.revalidate.in/1"
output_schema = "frob.result.revalidate.out/1"
refusals = ["FROB-REF-835", "FROB-REF-854", "FROB-REF-869", "FROB-REF-870", "FROB-REF-872"]
idempotent = false
cost_model = "estimated: ledger reads only for grades 1 and 2; a rule check per touched obligation for grade 3; reruns are separate jobs; `--estimate` returns CPU core-seconds, GPU seconds, memory and disk with a conformal interval"
first_milestone = "M4"
task_clusters = ["mission", "repair", "transfer"]
examples = ["frob result revalidate quad-tray --since d:7a3e --key rev-1 --json"]
owner = "frob-verbs"
purpose = "After a change: which results and certificates survive, which are invalidated, what reruns cost"
wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }
same_engine_as = "mission.revalidate"
```

`same_engine_as` enters with N28c-29 (M3), in the change that adds the field's row to N32a-02's later-field table.

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
   16 are read from NJL-01's table with the beads that land them.
3. Exit codes: 0 done; 2 done with findings; 3 refused input (1xx to 5xx, and 8xx other than conflicts); 4 budget
   (70x and 761); 5 capability (72x, 73x); 6 device (6xx); 7 conflict (803, 804, 809); 8 accepted and still running; 9
   indeterminate; never 1 on purpose.
4. Rows are entered when the cluster is first registered (M1); each row's tool is added to the task-cluster
   tables its `task_clusters` names in the same change (NJL-05's staging rule). A verb whose handler is not in
   the build answers one envelope with `FROB-REF-724`, exit 5, on both surfaces, and `frob capabilities --json`
   lists it as absent; a reduced form that lands before the row's `first_milestone` answers and is listed as
   implemented with its form named (NJL-01, NJL-03: the field keeps the appendix's milestone).

**Notes for this cluster.**

- `result.revalidate` reports, by read set, which results and certificates are inherited, which are invalidated and
  by which field, and what the reruns cost. A verdict reused after a change without a recorded implication is
  refused (`FROB-REF-835`).
- `result.revalidate` and `mission.revalidate` are two verbs over one engine (decision bead N28c-29). The
  relation is an entry field: `same_engine_as = "mission.revalidate"` on this row and the converse on
  `mission.revalidate` (VERBS-mission), the field N28c-29 adds to both entries at M3 by the entry schema's rule for
  a field a later bead adds (its row in N32a-02's later-field table, with its lint); the generated documentation
  prints the relation from it.
- `result.revalidate` first answers at M1 for the mission slice (dependency grades 1 and 2 on mission cells) and in
  full at M4. Its row keeps the appendix's `first_milestone = "M4"` (the rule BD-137, NJL-01 and every registry
  bead share: the field is the appendix's value and the handler decides what a build serves), and the early forms
  answer from the milestone their beads land:

  | Answers from | Restriction named by `frob capabilities` (the handler words it) | Bead |
  |---|---|---|
  | M1 | targets: mission cells; grades 1 and 2 | N28b-32 (GATE-ATEL-11's M1 row) |
  | M3 | every verdict kind; grades 1 and 2 | BD-292 |
  | M4 | none (grades 3 and 4) | BD-347 |

  Placement (correction, round 1): a form that answers at M1 needs its row at M1, and rows are entered when their
  cluster is first registered, so this bead (which registers the cluster `result`) moves from M4 to M1 (and,
  with that, to the package that owns the verb registry at M1, WP-06, with the priority of an M1 bead, like the
  sixteen other cluster beads registered at M1);
  `result.show` keeps M4 and answers `FROB-REF-724` before BD-376 lands. The entry N28b-32 drafts in its own text (capabilities
  `result:read`, `result:revalidate`; output schema `frob.revalidate.out/1`; task clusters `revalidate`,
  `missions`; `idempotent = true`) is not the row of record: this bead is, with the surface's naming rules
  (`frob.result.revalidate.out/1`; scope `result:run`, which implies `result:read`; the task clusters NJL-05
  assigns; `idempotent = false` for a `run` verb, idempotent by key).

## Acceptance criteria

- [ ] `registries/verbs.toml` holds exactly two entries with `cluster = "result"` (1 `read`, 1 `run`), each with
      every field of the entry schema, an example and a task-cluster list; the alias lint finds no second spelling.
- [ ] `frob-surface-diff parity --cluster result` exits 0 in process (`frob-mcp` over stdio) and through `frobd
      --mcp`: registry, CLI and MCP each count 2, and every difference list is empty apart from the harness's closed
      exemptions.
- [ ] Every verb has an end-to-end script: `scripts/e2e/verbs_result.sh` runs each verb of the cluster (two), and
      from a verb's first milestone a capability script of its implementing bead exists (`frob-surface-diff
      e2e-coverage`).
- [ ] `frob <verb> --help --json` equals `fixtures/golden/verbs/result/<verb>.help.json` byte for byte for each of
      the two, on two runs, debug and release, Linux and macOS; a planted renamed output property is classified
      `breaking` and refused.
- [ ] A verb not yet implemented answers `FROB-REF-724`, exit 5, with one envelope on both surfaces; an implemented
      verb runs its registry example with the result its implementing bead specifies; a misspelt parameter is
      `FROB-REF-802`, exit 3; a `run` or `mutate` call without a key is `FROB-REF-805`, exit 3.
- [ ] The generated refusal table lists for each verb exactly its chapter codes plus the codes appended for its
      kind and fields (NJL-01), and every code exists in `registries/refusals.toml`.
- [ ] The script's run leaves a receipt with the measured counts (registry, CLI and MCP each 2; every difference
      list 0) and the controls caught. This bead records no gate value: the per-milestone surface beads (BD-137 at
      M1, then BD-376) record GATE-TETHER-02's part for their milestone from such a receipt with `cargo run -p
      frob-conformance --example record_gate -- --gate GATE-TETHER-02 --receipt <hash> --part <M>`, in a session
      that did not write the rows (N38b-19).

## Test plan

### Unit tests

`cargo test -p frob-verbs --test cluster_result` (the real generated table; no mocks; planted registries are TOML
strings labelled `synthetic`):
- `rows_present`: exactly 2 rows in the cluster, with the names of the rows table; a planted extra row or a removed
  row fails.
- `kinds_milestones_tiers`: each row's kind, first milestone and tier equal the rows table (for example
  `result.show` is `read` at M4, `result.revalidate` is `run` at M4; in an M1 build `result.revalidate` answers
  and `capabilities` lists it implemented with its restriction named, its row still reading M4).
- `schemas_and_scopes`: input schemas are `frob.<name>.in/1`; output schemas and capabilities equal the
  machine-fields table.
- `refusal_lists`: each row's chapter codes equal its block (`result.revalidate`: 835, 854, 869, 870, 872;
  `result.show`: 840, 869); the appended codes equal the rule for its kind and fields (`result.show` is paginated and also
  gets 806 and 807).
- `inputs_by_row`: `result.revalidate` accepts exactly one of `--after-edit`, `--since`, `--between`,
  `--amendment`, `--against` (two given together, or none, is 802 with `pointer: /change`; `--against q:…` on a
  target that is not a mission answer is 802), and `--grades`, `--run`, `--estimate`; `frob result revalidate
  a:… --against q:… --json`, the ready command of FROB-REF-854's first repair, parses and is admitted;
  `result.show` accepts `--depth`.
- `mcp_names_one_to_one`: the tool names are the mapped names (`result_revalidate`, `result_show`), with no
  collision against any other cluster.
- `task_clusters_and_examples`: every row sits in a task cluster of at most seven tools or is a base tool; each
  example parses against the generated CLI and its command path is the row's own.
- `kind_behaviour`: a `run` row has the six common properties of conventions item 2, `idempotency_key` in no
  `required` array and annotated `x-frob-key: "required"`, `on_cancel` admitting exactly `leave` and
  `cancel_job`, and refuses `sync_wait_ms = 30001` with code 802; a
  verb whose capability is absent returns code 724, exit 5, one envelope.

### End-to-end script and logging

`scripts/e2e/verbs_result.sh` (header: `bead=VERBS-result milestone=M1
labels=tether,verbs,verb:result.revalidate,verb:result.show gates=GATE-TETHER-02 seed=1 needs=cpu,mcp,frobd
plants=alias,missing-row,renamed-field`) sources the shared helper and `scripts/e2e/verbs_lib.sh` and calls
`verbs_cluster_conformance result`. Steps: (1) `frob capabilities --json` and `frob robot-docs --json` list the two
verbs with kind and milestone; (2) `frob-surface-diff parity --cluster result` over stdio and through `frobd --mcp`,
every difference list empty; (3) `frob-surface-diff snapshot --check --cluster result`; (4) each verb's registry
example verbatim, expecting a result (exit 0 or 2), a job handle (exit 8, then `frob job wait`) or `FROB-REF-724`
exactly when the capability report says the verb is absent; (5) for each implemented verb the same call as an MCP
tool, the two envelopes agreeing on `verb`, `ok`, `exit`, `result`, `refusal`, `evidence`, `bound`, `warnings` and
`next_actions`; (6) per verb one call with a misspelt parameter (`e2e_expect_refusal FROB-REF-802 3`) and, for `run`
and `mutate` verbs, one without a key (`e2e_expect_refusal FROB-REF-805 3`); (7) one envelope per call and no exit
code 1.

Log: `target/e2e-logs/verbs_result/<utc>.ndjson` with a transcript beside it. Each `cmd` line has the exact command,
exit code, wall time and the envelope's `evidence` block; each `assert` has observed and expected values and the tag
`--gate GATE-TETHER-02`; the metrics `registry_verbs`, `cli_verbs` and `mcp_tools` are logged against 2 and the
difference counts against 0; refusals are logged with code and first repair; a failure logs the first differing verb
and field; the `summary` line gives assertions passed and failed and the gate touched.

### Known answers and negative controls

- Known answers: the cluster has 2 verbs (1 `read`, 1 `run`); the tables above. KA-755 (the exit-code table on
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

- `result.show` is named by no seed bead of its own; the surface bead for the M4 verbs (BD-376) carries it.
- Closing rule: the bead closes at M1, when the rows are registered, the surfaces match and every verb passes its
  example or answers the not-implemented refusal (`result.show` answers 724 until M4). The per-milestone surface
  beads (BD-137, BD-376) re-run this script as verbs land; that is how GATE-TETHER-02 is re-measured.
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

Appendix J (J.1 entry form; J.2 inventory rows and J.3 added rows; J.5 task clusters), §28.13, §32.4, §32.6,
§32.12.3, §32.18, §32.21; GATE-TETHER-02; INV-TETHER-01, -02; KA-755.
