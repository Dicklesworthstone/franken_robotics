---
key: "VERBS-predict"
title: "frob-verbs registry: cluster `predict`, two rows, with parity, e2e and schema-snapshot checks"
type: "task"
priority: 2
milestone: "M4"
labels: ["m4", "tether", "atelier", "frob-verbs", "solid", "registry", "verb", "test", "review-notes-open"]
parent: "WP-62"
deps: ["BD-022", "N28cx-01", "N30-REF", "NJL-01", "NJL-02", "NJL-04", "NJL-05", "STD-E2E"]
needs: []
related: []
gates: ["GATE-TETHER-02"]
plan_refs: ["Appendix J (J.1, J.2, J.3, J.5)", "§28.17", "§32.6", "§32.12.3", "§32.21 (GATE-TETHER-02)"]
---
## Background

FrankenRobotics (a robotics simulation and design system) is reached through the command line `frob`, the daemon
`frobd` and the MCP server `frob-mcp` (the Model Context Protocol, by which an agent harness lists and calls tools).
Every capability is a verb, `cluster.verb`, declared once in `registries/verbs.toml` (crate `frob-verbs`, tier
Solid). The CLI, the daemon protocol, the MCP tools, the schemas, the refusal table and the agent guide are
generated from that file, and gate GATE-TETHER-02 compares the served surfaces with it. The registry holds 244 verbs
in 44 clusters (namespaces): the 243 of the specification and `artifact.import` (VERBS-artifact); the
count of record is NJL-03's.

This bead is the registry of record for the cluster `predict`. The cluster covers registered predictions: an agent
states an interval and a confidence before a run, and its calibration is reported. It has two verbs (1 `read`, 1
`mutate`); the handlers live in `frob-opt`. The bead carries every row completely, with the three checks that keep
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
inventory); one bead for all the rows (too large to review).

## Scope

In: the two registry entries of cluster `predict`; the MCP tool names; the cluster's conformance script; the
help-schema snapshots; each verb's answer before its milestone. Out: behaviour (the implementing beads named below);
the entry schema and lints (NJL-01); the conformance harness (NJL-02); the alias lint (NJL-04); task clusters
(NJL-05, NJL-06); generation of the surfaces (the Tether seed beads).

## Technical approach

**The rows.**

| Verb | Kind | First milestone | Tier | Purpose | Defined in |
|---|---|---|---|---|---|
| `predict.register` | mutate | M4 | S | Register a prediction with an interval and a confidence before a run | §28.17 |
| `predict.report` | read | M4 | S | Calibration of an agent's registered predictions | §28.17 |

**Contract of each verb.** Evidence labels used below (the `evidence` block of the envelope):
- *operational*: no claim: an operational answer about the session, the store, a job or a device; `evidence.claims`
  is empty.
- *pass-through*: passes through the claim product of the stored objects it reads and adds no claim of its own; the
  read itself is `exact`.

**`predict.register`** (MCP tool `predict_register`; owner `frob-opt`)
- Inputs: `--metric` (a registered metric whose unit matches); `--interval <lo> <hi>` (lo ≤ hi); `--confidence c`
  (0 < c < 1); `--for <target>` (a planned run or campaign that the caller may read, or `next`: the caller's next
  run or campaign dispatched in this session, whose immutable identifiers are bound into the record's freeze set
  at that dispatch); idempotency key (the same key with the same content returns the same record). An inverted
  interval, a confidence outside (0, 1) or an unknown metric is the appended `FROB-REF-802`.
- Outputs: `prediction_id`, `registered_seq` (the ledger append order, never a clock) and the freeze status of
  the target. The record is hashed and frozen into the target job's freeze set when the job is dispatched, and
  every record later receives exactly one disposition (`scored` with coverage and interval score, `no-outcome:`
  `cancelled`/`failed`/`refused`, or `no-outcome: pending`); a `no-outcome` carries its reason, including
  `next-unbound` (a `next` record never bound by the end of the session) and `target-replanned` (the target's
  immutable identifiers changed before it ran). Evidence: *operational*.
- Chapter refusals: `FROB-REF-883` (a prediction changed after freezing: a registration by the same predictor for
  the same target whose content differs from a record of the freeze set; `pointer: prediction`; the code's second
  clause, merged by N28cx-01).
- Pending, uncoded: `prediction-after-outcome` (a registration when an outcome for the target already exists in
  the ledger, or the target's job has committed its first chunk; `pointer: prediction.target`). N28cx-01 files
  it in the `8xx` family with status `held`; until a number is released it is raised on its interim form, 883
  with `detail = "registration-after-outcome"`, so the row's list stays 883 and gains the new code when it is
  assigned (or keeps 883 alone if the owner merges it).
- Implemented by: BD-344. Codes decided by: N28cx-01.

**`predict.report`** (MCP tool `predict_report`; owner `frob-opt`)
- Inputs: optional `--predictor <id>` (default the caller; an agent sees its own predictions and the studio's, an
  operator scope sees all), `--metric <m>`, `--since <frontier>`.
- Outputs: per predictor and version, per confidence level: counts of registered, scored, covered, missed,
  unresolved and pending; empirical coverage with its exact binomial interval, and coverage counting unresolved
  as misses beside it; the mean interval score; the most recent records with dispositions. The accounting
  identity registered = scored + unresolved + pending holds in every output. Evidence: *pass-through* (counts
  over ledger records: class Tested, determinism D0 at a pinned ledger frontier).
- Chapter refusals: none of its own.
- Implemented by: BD-344.

**Machine fields of each row.**

| Verb | Capabilities | Output schema | Task clusters | Example |
|---|---|---|---|---|
| `predict.register` | `predict:write` | `frob.predict.register.out/1` | `compare` | `frob predict register --metric success --interval 0.6 0.9 --confidence 0.8 --for next --key pr-1 --json` |
| `predict.report` | `predict:read` | `frob.predict.report.out/1` | `reality`, `ladder` | `frob predict report --json` |

The remaining fields follow by rule: `input_schema` is `frob.<verb name>.in/1`; `refusals` is the chapter list of
the verb's block; `idempotent` is `true` for `read` and `admin` and `false` for `run` and `mutate` (which take an
idempotency key); `cost_model` is `static: …` for `read`, `mutate` and `admin` and the `--estimate` cost model for
`run`; a `run` row carries `wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }`; `owner` is the
crate of the block; `purpose` is the Purpose column.

One entry in full (the others follow field for field):

```toml
[[verb]]
name = "predict.register"
cluster = "predict"
kind = "mutate"
tier = "S"
capabilities = ["predict:write"]
input_schema = "frob.predict.register.in/1"
output_schema = "frob.predict.register.out/1"
refusals = ["FROB-REF-883"]
idempotent = false
cost_model = "static: one ledger transaction"
first_milestone = "M4"
task_clusters = ["compare"]
examples = ["frob predict register --metric success --interval 0.6 0.9 --confidence 0.8 --for next --key pr-1 --json"]
owner = "frob-opt"
purpose = "Register a prediction with an interval and a confidence before a run"
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

- A prediction is frozen before the run it is for; registering after the outcome is known is refused by the ledger
  order (interim form 883 with `detail = "registration-after-outcome"` until N28cx-01's held number is
  released); altering a frozen prediction is 883.
- A refit after scoring opens a new prediction version; the old one stays in the ledger with its scores, and
  calibration windows are per version.

## Acceptance criteria

- [ ] `registries/verbs.toml` holds exactly two entries with `cluster = "predict"` (1 `read`, 1 `mutate`), each with
      every field of the entry schema, an example and a task-cluster list; the alias lint finds no second spelling.
- [ ] `frob-surface-diff parity --cluster predict` exits 0 in process (`frob-mcp` over stdio) and through `frobd
      --mcp`: registry, CLI and MCP each count 2, and every difference list is empty apart from the harness's closed
      exemptions.
- [ ] Every verb has an end-to-end script: `scripts/e2e/verbs_predict.sh` runs each verb of the cluster (two), and
      from a verb's first milestone a capability script of its implementing bead exists (`frob-surface-diff
      e2e-coverage`).
- [ ] `frob <verb> --help --json` equals `fixtures/golden/verbs/predict/<verb>.help.json` byte for byte for each of
      the two, on two runs, debug and release, Linux and macOS; a planted renamed output property is classified
      `breaking` and refused.
- [ ] A verb not yet implemented answers `FROB-REF-724`, exit 5, with one envelope on both surfaces; an implemented
      verb runs its registry example with the result its implementing bead specifies; a misspelt parameter is
      `FROB-REF-802`, exit 3; a `run` or `mutate` call without a key is `FROB-REF-805`, exit 3.
- [ ] The generated refusal table lists for each verb exactly its chapter codes plus the codes appended for its
      kind and fields (NJL-01), and every code exists in `registries/refusals.toml`.
- [ ] The script's run leaves a receipt with the measured counts (registry, CLI and MCP each 2; every difference
      list 0) and the controls caught. This bead records no gate value: the M4 surface bead BD-376 records
      GATE-TETHER-02's M4 part from that receipt with `cargo run -p frob-conformance --example record_gate --
      --gate GATE-TETHER-02 --receipt <hash> --part M4`, in a session that did not write the rows (N38b-19).

## Test plan

### Unit tests

`cargo test -p frob-verbs --test cluster_predict` (the real generated table; no mocks; planted registries are TOML
strings labelled `synthetic`):
- `rows_present`: exactly 2 rows in the cluster, with the names of the rows table; a planted extra row or a removed
  row fails.
- `kinds_milestones_tiers`: each row's kind, first milestone and tier equal the rows table (for example
  `predict.report` is `read` at M4).
- `schemas_and_scopes`: input schemas are `frob.<name>.in/1`; output schemas and capabilities equal the
  machine-fields table.
- `refusal_lists`: each row's chapter codes equal its block (`predict.register`: 883; `predict.report`: none); the
  appended codes equal the rule for its kind and fields; the pending name `prediction-after-outcome` is on no row's list
  until it has a number.
- `mcp_names_one_to_one`: the tool names are the mapped names (`predict_register`, `predict_report`), with no
  collision against any other cluster.
- `task_clusters_and_examples`: every row sits in a task cluster of at most seven tools or is a base tool; each
  example parses against the generated CLI and its command path is the row's own.
- `kind_behaviour`: a `mutate` row has `idempotency_key` (in no `required` array, annotated `x-frob-key:
  "required"`) and `expected_parent` and refuses a call without the key with code 805; a verb whose capability is
  absent returns code 724, exit 5, one envelope.

### End-to-end script and logging

`scripts/e2e/verbs_predict.sh` (header: `bead=VERBS-predict milestone=M4
labels=tether,verbs,verb:predict.register,verb:predict.report gates=GATE-TETHER-02 seed=1 needs=cpu,mcp,frobd
plants=alias,missing-row,renamed-field`) sources the shared helper and `scripts/e2e/verbs_lib.sh` and calls
`verbs_cluster_conformance predict`. Steps: (1) `frob capabilities --json` and `frob robot-docs --json` list the two
verbs with kind and milestone; (2) `frob-surface-diff parity --cluster predict` over stdio and through `frobd
--mcp`, every difference list empty; (3) `frob-surface-diff snapshot --check --cluster predict`; (4) each verb's
registry example verbatim, expecting a result (exit 0 or 2), a job handle (exit 8, then `frob job wait`) or
`FROB-REF-724` exactly when the capability report says the verb is absent; (5) for each implemented verb the same
call as an MCP tool, the two envelopes agreeing on `verb`, `ok`, `exit`, `result`, `refusal`, `evidence`, `bound`,
`warnings` and `next_actions`; (6) per verb one call with a misspelt parameter (`e2e_expect_refusal FROB-REF-802 3`)
and, for `run` and `mutate` verbs, one without a key (`e2e_expect_refusal FROB-REF-805 3`); (7) one envelope per
call and no exit code 1.

Log: `target/e2e-logs/verbs_predict/<utc>.ndjson` with a transcript beside it. Each `cmd` line has the exact
command, exit code, wall time and the envelope's `evidence` block; each `assert` has observed and expected values
and the tag `--gate GATE-TETHER-02`; the metrics `registry_verbs`, `cli_verbs` and `mcp_tools` are logged against 2
and the difference counts against 0; refusals are logged with code and first repair; a failure logs the first
differing verb and field; the `summary` line gives assertions passed and failed and the gate touched.

### Known answers and negative controls

- Known answers: the cluster has 2 verbs (1 `read`, 1 `mutate`); the tables above. KA-755 (the exit-code table on
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

Appendix J (J.1 entry form; J.2 inventory rows; J.5 task clusters), §28.17, §32.4, §32.6, §32.12.3, §32.18, §32.21;
GATE-TETHER-02; INV-TETHER-01, -02; KA-755.
