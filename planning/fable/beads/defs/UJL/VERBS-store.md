---
key: "VERBS-store"
title: "frob-verbs registry: cluster `store`, two rows, with parity, e2e and schema-snapshot checks"
type: "task"
priority: 1
milestone: "M2"
labels: ["m2", "tether", "chronicle", "frob-verbs", "solid", "registry", "verb", "test"]
parent: "WP-43"
deps: ["BD-022", "NJL-01", "NJL-02", "NJL-04", "NJL-05", "STD-E2E"]
needs: []
related: []
gates: ["GATE-TETHER-02"]
plan_refs: ["Appendix J (J.1, J.2, J.3, J.5)", "§29.16", "§32.6", "§32.12.3", "§32.21 (GATE-TETHER-02)"]
---
## Background

FrankenRobotics (a robotics simulation and design system) is reached through the command line `frob`, the daemon
`frobd` and the MCP server `frob-mcp` (the Model Context Protocol, by which an agent harness lists and calls tools).
Every capability is a verb, `cluster.verb`, declared once in `registries/verbs.toml` (crate `frob-verbs`, tier
Solid). The CLI, the daemon protocol, the MCP tools, the schemas, the refusal table and the agent guide are
generated from that file, and gate GATE-TETHER-02 compares the served surfaces with it. The registry holds 244 verbs
in 44 clusters (namespaces): the 243 of the specification and `artifact.import` (VERBS-artifact); the
count of record is NJL-03's.

This bead is the registry of record for the cluster `store`. The cluster covers the store: its size, pins and
retention, and a collection cycle with tombstones. It has two verbs (1 `read`, 1 `admin`); the handlers live in
`frob-chronicle`. The bead carries every row completely, with the three checks that keep the rows true: the
generated CLI commands and MCP tools match the list; every verb has an end-to-end script; the schema printed by
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

In: the two registry entries of cluster `store`; the MCP tool names; the cluster's conformance script; the
help-schema snapshots; each verb's answer before its milestone. Out: behaviour (the implementing beads named below);
the entry schema and lints (NJL-01); the conformance harness (NJL-02); the alias lint (NJL-04); task clusters
(NJL-05, NJL-06); generation of the surfaces (the Tether seed beads).

## Technical approach

**The rows.**

| Verb | Kind | First milestone | Tier | Purpose | Defined in |
|---|---|---|---|---|---|
| `store.status` | read | M2 | S | Store size, pins and retention | §29.16 |
| `store.collect` | admin | M2 | S | A collection cycle with tombstones | §29.16 |

**Contract of each verb.** Evidence labels used below (the `evidence` block of the envelope):
- *operational*: no claim: an operational answer about the session, the store, a job or a device; `evidence.claims`
  is empty.

**`store.status`** (MCP tool `store_status`; owner `frob-chronicle`)
- Inputs: none.
- Outputs (BD-209): per retention class the object count and bytes; the budget and the bytes over or under it;
  pinned bytes by reason kind with the top pin holders (claim and job identifiers); the oldest unpinned sampled
  trace; tombstone counts by reason; the memo store's size against its default; the retention window in force;
  the per-kind memo counters (N28b-38); per memo policy hits, misses, bytes used, pinned bytes, evictions,
  admissions refused by cost and by size, ghost admissions, cost saved by hits in both units and the current
  small share (N33c-19); `unpinned_closure_objects` (the writer-rule audit, which must read 0), the count of
  orphan pins released with reason `holder-gone`, and the operator `hold` pins with their age (N29b-13). The
  line for private holdings (objects, bytes, pins) is printed only to a session holding `private:read`, and the
  output is otherwise byte-identical, with no private byte total (BD-305, N29b-14). Evidence: *operational*.
- Chapter refusals: none of its own.
- Implemented by: BD-209. Extended by: N28b-38 (memo counters), N33c-19 (memo policy statistics), N29b-13
  (audit and orphan lines), BD-305 and N29b-14 (the private line).

**`store.collect`** (MCP tool `store_collect`; owner `frob-chronicle`)
- Inputs (BD-209): `--dry-run`; `--budget <size>` (bytes, or with an SI suffix `kB`, `MB`, `GB`: `1MB` is 10⁶
  bytes); `--now <instant>` (tests); `--class <c>` (restrict to one class); `--force` (remove the objects of the
  named class that the window and budget rules would keep; a pinned object is never removed); `idempotency_key`
  (`--key`), declared by this row as its own optional input because the `admin` kind has no common key property:
  absent, the call is a dry run (BD-209's default); present, it is a real collection, and a repeat with the same
  key returns the stored cycle. The withdrawal form (N29b-13): `--withdraw-claim <claim_id> --reason <text>`,
  idempotent by construction (a second withdrawal returns the first record) and taking no key.
- Outputs: the removed (or would-be removed) objects by class with bytes, the tombstones written, and what was kept
  and why for the ten largest kept objects; pinned objects and frontiers held by open cursors are kept; orphan
  pins released before the mark, each with reason `holder-gone` (N29b-13). Before the mark, and before any
  orphan pin is released, a real collection and a dry run run the writer-rule audit
  (`audit::pins_cover_live_claims`); a non-empty result aborts the cycle with the refusal
  `unpinned-claim-closure` (below) and removes nothing, so an aborted cycle changes nothing (N29b-13). For a
  withdrawal: the claim's released pins, the dependent claims made unavailable, and the affected gate parts and
  beads. Evidence: *operational*.
- Chapter refusals: `FROB-REF-813` (a deletion that would remove a pinned object; meeting one under `--force`
  refuses the whole call and names the pins). A withdrawal of a claim that supports a `qualification-receipt`
  without the owner scope is the appended `FROB-REF-720` naming that scope (exit 5). One refusal is a ledger row
  that no bead numbers by hand: `unpinned-claim-closure` (N29-REF's row, raised by N29b-13; family `9xx`, exit 9,
  retry `never`: an integrity fault of the store, not an input the caller got wrong): the pre-mark audit found a
  live claim whose evidence closure holds a stored object the claim does not pin; the block carries the count of
  (claim, object) pairs and the first 20 pairs with a handle for the rest; repairs 1. withdraw each listed claim
  (`--withdraw-claim`, which runs no cycle and is not blocked by the audit) and re-run its evidence, 2. report
  the writer that inserted the claim and leave the store uncollected until it is fixed. Its number comes from
  the allocation ledger (`registries/refusal_allocations.toml`, N35-33's rule; no proposal, no interim code); the
  row's `refusals` receives that number in the change that registers it, and no bead types it.
- Implemented by: BD-209.
- Extended by: N29b-13 (claim withdrawal, `--withdraw-claim`; the pre-mark audit with `unpinned-claim-closure`
  and the orphan release).

**Machine fields of each row.**

| Verb | Capabilities | Output schema | Task clusters | Example |
|---|---|---|---|---|
| `store.status` | `store:read` | `frob.store.status.out/1` | `housekeeping` | `frob store status --json` |
| `store.collect` | `store:admin` | `frob.store.collect.out/1` | `housekeeping` | `frob store collect --dry-run --budget 1MB --json` |

The remaining fields follow by rule: `input_schema` is `frob.<verb name>.in/1`; `refusals` is the chapter list of
the verb's block; `idempotent` is `true` for `read` and `admin` and `false` for `run` and `mutate` (which take an
idempotency key); `cost_model` is `static: …` for `read`, `mutate` and `admin` and the `--estimate` cost model for
`run`; a `run` row carries `wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }`; `owner` is the
crate of the block; `purpose` is the Purpose column.

One entry in full (the others follow field for field):

```toml
[[verb]]
name = "store.status"
cluster = "store"
kind = "read"
tier = "S"
capabilities = ["store:read"]
input_schema = "frob.store.status.in/1"
output_schema = "frob.store.status.out/1"
refusals = []
idempotent = true
cost_model = "static: answered from the store or by a small CPU computation; no job"
first_milestone = "M2"
task_clusters = ["housekeeping"]
examples = ["frob store status --json"]
owner = "frob-chronicle"
purpose = "Store size, pins and retention"
```

**Conventions the rows follow.**

1. The MCP tool name is the verb name with `.` and `-` replaced by `_`. The output on the wire is one envelope
   `frob.envelope/1` with `result` specialized to the output schema; its `evidence` block is a claim product (the
   weakest class, tier and determinism class among the inputs).
2. The generator adds the common input properties of the kind, as NJL-01's kinds table lists them (`run`:
   `sync_wait_ms` from 0 to the row's `wait.mcp_max_ms`, 30,000, with MCP default 5,000; `idempotency_key`, CLI
   `--key`; `budget`; `estimate_only`, CLI `--estimate`; `on_cancel`, `leave` (the default: the job keeps
   running) or `cancel_job`; `envelope_scope`, CLI `--envelope-scope <gate:…|campaign:…|pretrain:…>`, the
   approved compute-envelope scope the job cites (NJL-01; a citation that contradicts the token of the launcher
   the job runs under is 761 with pointer `/envelope_scope`, N35-REF); `mutate`: `idempotency_key` and
   `expected_parent`). `idempotency_key` is never in the JSON-schema `required` array: the annotation
   `x-frob-key: "required"` (`"optional"` on a `key_optional` entry) carries the rule, and the admission
   transaction refuses a missing key with 805 on both surfaces. The surface-wide refusals appended by kind:
   `FROB-REF-720` (scope missing), 723 (token invalid), 724 (not implemented in this build) and 802 (schema
   failure, including an unknown parameter) for every verb; 701 to 704 (budget and admission), 761 (above the
   compute envelope in force), 803 (key reused with different content) and 805 (missing key) for `run`; 725, 803,
   804 (stale parent) and 805 for `mutate`. The properties and codes driven by fields are those of NJL-01's
   variation table: `cursor`, `limit` and `follow` with 806 and 807 for a paginated entry or a paginated action;
   808 (an integer outside ±(2⁵³ − 1)) for an entry whose generated input schema has an `int53` property at any
   depth, whatever its kind (every `run` entry has one, `sync_wait_ms`); `estimate_only` on a non-`run` entry
   with `estimable = true`; no 805 for a `key_optional` entry; the `action` property, with each action's codes,
   for an entry with `actions`; 790 for a non-empty `needs_groups`; 891 for an `artifact-input<F>`,
   `artifact-ref<F>` or `artifact-identity<F>` parameter, and 892 as well for `artifact-identity<F>`; 893 for a
   `foreign-file` parameter; 734 and 828 for a parameter that accepts a path to a file (`artifact-input<F>`,
   `artifact-identity<F>`, `foreign-file`). The later variations of that table (10 to 13) land with their beads
   and are read there, not copied here.
3. Exit codes: 0 done; 2 done with findings; 3 refused input (1xx to 5xx, and 8xx other than conflicts); 4 budget
   (70x and 761); 5 capability (72x, 73x); 6 device (6xx); 7 conflict (803, 804, 809); 8 accepted and still running; 9
   indeterminate; never 1 on purpose.
4. Rows are entered when the cluster is first registered (M2); each row's tool is added to the task-cluster
   tables its `task_clusters` names in the same change (NJL-05's staging rule). A verb whose handler is not in
   the build answers one envelope with `FROB-REF-724`, exit 5, on both surfaces, and `frob capabilities --json`
   lists it as absent; a reduced form that lands before the row's `first_milestone` answers and is listed as
   implemented with its form named (NJL-01, NJL-03: the field keeps the appendix's milestone).

**Notes for this cluster.**

- `store.collect` is of kind `admin`. Collection is bounded and respects retained checkpoints, pinned objects and
  the frontiers pinned by open cursors; a deletion that would remove a pinned object is refused (`FROB-REF-813`).
- Key on an `admin` verb (DESIGN, round 1): BD-209 requires an idempotency key for a real collection and makes a
  keyless call a dry run, while N29b-13 says an `admin` verb takes no key. Both hold with the key as this row's own
  optional input, which selects a real collection and makes it replayable; no `FROB-REF-805` is appended (the
  `admin` rule), because a keyless call is a valid dry run, not a refusal.

## Acceptance criteria

- [ ] `registries/verbs.toml` holds exactly two entries with `cluster = "store"` (1 `read`, 1 `admin`), each with
      every field of the entry schema, an example and a task-cluster list; the alias lint finds no second spelling.
- [ ] `frob-surface-diff parity --cluster store` exits 0 in process (`frob-mcp` over stdio) and through `frobd
      --mcp`: registry, CLI and MCP each count 2, and every difference list is empty apart from the harness's closed
      exemptions.
- [ ] Every verb has an end-to-end script: `scripts/e2e/verbs_store.sh` runs each verb of the cluster (two), and
      from a verb's first milestone a capability script of its implementing bead exists (`frob-surface-diff
      e2e-coverage`).
- [ ] `frob <verb> --help --json` equals `fixtures/golden/verbs/store/<verb>.help.json` byte for byte for each of
      the two, on two runs, debug and release, Linux and macOS; a planted renamed output property is classified
      `breaking` and refused.
- [ ] A verb not yet implemented answers `FROB-REF-724`, exit 5, with one envelope on both surfaces; an implemented
      verb runs its registry example with the result its implementing bead specifies; a misspelt parameter is
      `FROB-REF-802`, exit 3.
- [ ] The generated refusal table lists for each verb exactly its chapter codes plus the codes appended for its
      kind and fields (NJL-01), and every code exists in `registries/refusals.toml`.
- [ ] The gate row is recorded with the measured counts by a session that did not write the rows.

## Test plan

### Unit tests

`cargo test -p frob-verbs --test cluster_store` (the real generated table; no mocks; planted registries are TOML
strings labelled `synthetic`):
- `rows_present`: exactly 2 rows in the cluster, with the names of the rows table; a planted extra row or a removed
  row fails.
- `kinds_milestones_tiers`: each row's kind, first milestone and tier equal the rows table (for example
  `store.collect` is `admin` at M2).
- `schemas_and_scopes`: input schemas are `frob.<name>.in/1`; output schemas and capabilities equal the
  machine-fields table.
- `refusal_lists`: each row's chapter codes equal its block (`store.status`: none; `store.collect`: 813, and the
  number the ledger assigned to `unpinned-claim-closure`, read from `registries/refusal_allocations.toml` by
  slug, in a registry where that row is entered; none is typed in the test); the appended codes equal the rule
  for its kind and fields (NJL-01) (no 805 on `store.collect`).
- `collect_key_selects_mode`: `store.collect` without a key returns a dry run and removes nothing; with a key it
  collects, and the same key again returns the stored cycle; `--withdraw-claim` twice returns the first record.
- `mcp_names_one_to_one`: the tool names are the mapped names (`store_status`, `store_collect`), with no collision
  against any other cluster.
- `task_clusters_and_examples`: every row sits in a task cluster of at most seven tools or is a base tool; each
  example parses against the generated CLI and its command path is the row's own.
- `kind_behaviour`: an `admin` row or action without its scope is refused with code 720; a verb whose capability is
  absent returns code 724, exit 5, one envelope.

### End-to-end script and logging

`scripts/e2e/verbs_store.sh` (header: `bead=VERBS-store milestone=M2
labels=tether,verbs,verb:store.status,verb:store.collect gates=GATE-TETHER-02 seed=1 needs=cpu,mcp,frobd
plants=alias,missing-row,renamed-field`) sources the shared helper and `scripts/e2e/verbs_lib.sh` and calls
`verbs_cluster_conformance store`. Steps: (1) `frob capabilities --json` and `frob robot-docs --json` list the two
verbs with kind and milestone; (2) `frob-surface-diff parity --cluster store` over stdio and through `frobd --mcp`,
every difference list empty; (3) `frob-surface-diff snapshot --check --cluster store`; (4) each verb's registry
example verbatim, expecting a result (exit 0 or 2), a job handle (exit 8, then `frob job wait`) or `FROB-REF-724`
exactly when the capability report says the verb is absent; (5) for each implemented verb the same call as an MCP
tool, the two envelopes agreeing on `verb`, `ok`, `exit`, `result`, `refusal`, `evidence`, `bound`, `warnings` and
`next_actions`; (6) per verb one call with a misspelt parameter (`e2e_expect_refusal FROB-REF-802 3`); (7) one
envelope per call and no exit code 1.

Log: `target/e2e-logs/verbs_store/<utc>.ndjson` with a transcript beside it. Each `cmd` line has the exact command,
exit code, wall time and the envelope's `evidence` block; each `assert` has observed and expected values and the tag
`--gate GATE-TETHER-02`; the metrics `registry_verbs`, `cli_verbs` and `mcp_tools` are logged against 2 and the
difference counts against 0; refusals are logged with code and first repair; a failure logs the first differing verb
and field; the `summary` line gives assertions passed and failed and the gate touched.

### Known answers and negative controls

- Known answers: the cluster has 2 verbs (1 `read`, 1 `admin`); the tables above. KA-755 (the exit-code table on
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

- Closing rule: the bead closes at M2, when the rows are registered, the surfaces match and every verb passes its
  example or answers the not-implemented refusal. The per-milestone surface beads (BD-222) re-run this script as
  verbs land; that is how GATE-TETHER-02 is re-measured.
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

Appendix J (J.1 entry form; J.3 added rows; J.5 task clusters), §29.16, §32.4, §32.6, §32.12.3, §32.18, §32.21;
GATE-TETHER-02; INV-TETHER-01, -02; KA-755.
