---
key: "VERBS-operator"
title: "frob-verbs registry: cluster `operator`, four rows, with parity, e2e and schema-snapshot checks"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "tether", "frob-verbs", "solid", "registry", "verb", "test", "review-notes-open"]
parent: "WP-06"
deps: ["BD-022", "NJL-01", "NJL-02", "NJL-04", "STD-E2E"]
needs: []
related: ["BD-304", "N00-02", "N32a-26", "N32b-17", "N32b-19", "N32b-20", "N33c-32", "N35-33", "N37-23"]
gates: ["GATE-TETHER-02"]
plan_refs: ["Appendix J (J.1, J.2, J.3, J.5)", "§32.6", "§32.17", "§32.12.3", "§32.21 (GATE-TETHER-02)"]
---
## Background

FrankenRobotics (a robotics simulation and design system) is reached through the command line `frob`, the daemon
`frobd` and the MCP server `frob-mcp` (the Model Context Protocol, by which an agent harness lists and calls tools).
Every capability is a verb, `cluster.verb`, declared once in `registries/verbs.toml` (crate `frob-verbs`, tier
Solid). The CLI, the daemon protocol, the MCP tools, the schemas, the refusal table and the agent guide are
generated from that file, and gate GATE-TETHER-02 compares the served surfaces with it. The registry holds 244 verbs
in 44 clusters (namespaces): the 243 of the specification and `artifact.import` (VERBS-artifact); the
count of record is NJL-03's.

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
inventory); one bead for all the rows (too large to review).

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
- Inputs: optional `--section <name>`, repeatable (none: every section). Sections, one static table sorted by
  name, each contributed through the section interface of BD-022 by the bead that owns its crate: `toolchain`,
  `stack` (one row per admitted foundation: pin, on-pin, clean, admission gate; the foundation-pin checks),
  `stack_receipt`, `upstream`, `devices`, `kernel_cache`, `probes`, `daemon` (BD-022 at M0; completed by N32b-20
  at M2, which also adds `store`); `kernels` (N21b-01); `ledger` (N34-26); `registries` (N32ax-03; the flag
  `--registries` is its shorthand); `referee` (N33a-14). Optional `--notices` (N37-18, M3) prints the
  `THIRD_PARTY_NOTICES` file embedded in the binary instead of running sections. An unknown section name is
  `FROB-REF-802` with `pointer: /section`.
- Outputs: `frob.doctor.out/1`: one block per section with `status` (`ok`, `findings`, `skipped` with a reason)
  and `findings[]` (`code` a stable slug, `severity` `info`/`low`/`high`, `message`, `observed`, `required`,
  ranked `repairs` as commands). It changes nothing: no cache rebuilt, no pin moved, no probe re-run; a section
  never takes longer than 2 s (TARGET) and one that times out is a finding. Exit 0 when no finding is above
  `info`; exit 2 with the findings otherwise. Evidence: *operational*.
- Chapter refusals: none of its own.
- Implemented by: BD-022 (the M0 form and the section interface), N32b-20 (the complete form at M2).
- Extended by: N21b-01 (`kernels`), N34-26 (the read-only `ledger` block read at M0), N32ax-03 (`registries`),
  N33a-14 (`referee`), N37-18 (`--notices`, M3).
- Spelling (DESIGN): the selector is `--section <name>` (repeatable), as BD-022, the section beads and N32b-20
  write it, with BD-022's section names (`stack` holds the pin checks; `kernel_cache` with an underscore) and
  `store` as N32b-20's section; `--check <group>` is no input of the row (802).

**`capabilities`** (MCP tool `capabilities`; owner `frob-verbs`; `paginated = true`: paged with a cursor when
`--limit` is given, and the generator appends 806 and 807)
- Inputs, by the bead and milestone that bring each: `--verb <name>` (BD-022, M0); `--missing` (the same as
  `--status partial,not-in-build`; N00-02, M0); `--require-group <group>` (N33c-32, M0; repeatable; the values
  are the known airlock feature-group names, and another value is 802); `--device <id>`, `--status <list>`,
  `--cluster`, `--milestone`, and `--limit` with the cursor (N32b-19, M1). What each filter selects (N32b-19,
  DESIGN): `--verb`, `--status`, `--cluster <registry cluster>` and `--milestone <Mn>` keep the rows of
  `capabilities[]` that list that verb, have one of those statuses, have a verb in that cluster, or have that
  `first_milestone`, and `summary` counts the rows kept; `--device <id>` keeps that one row of `devices[]`, and
  an identifier that names no device of the host is `FROB-REF-802` with `pointer: /device` and the present
  identifiers in the repair; `build` and `gates[]` are never filtered. `--require-group` answers exit 0 with the
  same listing when every named group is compiled and admitted, and otherwise refuses with `FROB-REF-790` for
  the first missing one (pointer `foundation/<group>/compiled` or `foundation/<group>/admitted`, exit 5): the
  row carries `needs_groups = [{ from_argument = "require_group" }]`, so the generator appends 790 with source
  `field:needs_groups` (convention 2); N33c-32 writes the input and the field into this row in the change that
  adds `needs_groups` to the entry schema (N32a-02's later-field table), since no row may carry the field
  before that change. A cursor reused with a changed filter is `FROB-REF-806`; a page asked for after a gate
  was recorded is `FROB-REF-807` (the cursor's frontier is the pair of registry and gate-register digests,
  N32b-19).
- Outputs: `frob.capabilities.out/1` (schema owned by N00-02): from M0 the registry digest, `capabilities[]`
  (every verb with its status, `implemented` in the eight-condition sense, `partial`, `not-in-build` or
  `killed`, the missing condition numbers, `gates`, `first_milestone` and `fallback`) and `summary` (counts by
  status), computed by N00-02's status function from the capability map `registries/capabilities.toml` and read
  by BD-022's handler through its interface `StatusSource` (BD-022 ships only the fail-closed source: nothing
  is ever `implemented` until N00-02 installs the registry-driven one); from M1 (N32b-19) `build` (commit,
  toolchain, stack receipt digest, enabled feature groups), `gates[]` (the gate register as rows) and
  `devices[]` (per device: backend, availability, hardware fingerprint, resolved modes with the reason,
  attainable determinism classes, arithmetic features and ceilings, an unmeasured ceiling reported as
  `unknown`). Evidence: *operational* (class Tested, tier `static`, determinism `exact`).
- Chapter refusals: none of its own (790 is appended by field, above).
- Implemented by: BD-022 (the M0 handler and `--verb`), N00-02 (M0: the map, the status function, `--missing`
  and the dispatch rule of `FROB-REF-724`), N32b-19 (M1: the complete form; from M3 the resource
  `frob://capabilities`, N32a-25). Extended by: N33c-32 (M0: `--require-group`, `needs_groups` and the group
  listing `compiled`, `admitted` and the gate per group).
- Correction (round 3): this block formerly gave `--device` to BD-022 and dated `build`, `gates[]` and
  `devices[]` at M2; BD-022's own table gives the M0 handler `--verb` only and assigns `--device` and the three
  sections to N32b-19, whose milestone is M1.

**`robot-docs`** (MCP tool `robot_docs`; owner `frob-verbs`)
- Inputs: optional `--cluster <task cluster>`, `--verb <name>`, `--budget <tokens>` (the guide trimmed to a token
  budget), `--encoding plain|compact` (N32b-17); `--workflow <name>` (N32a-26, M3: one of the seven workflow
  names `studio_loop`, `mission_query`, `diagnose_failure`, `qualify_design`, `compare_designs`,
  `import_robot`, `calibrate_to_reality`; any other name is `FROB-REF-802`).
- Outputs: `frob.robot-docs.out/1`: the agent guide: per verb its purpose, when to use it, kind, cost class,
  refusal codes and one example; the envelope and exit tables; worked sessions and contributed pages; or one task
  cluster or verb; with `--workflow`, the optional member `workflow` (name, variables, the ordered steps of
  `registries/examples/workflows/<name>.toml`), the same steps the MCP prompt of that workflow renders.
  Evidence: *operational*.
- Chapter refusals: none of its own.
- Implemented by: BD-022 (the v0 reference), N32b-17 (M1, the layered guide), BD-313 (M3, the full guide, skill
  file and worked sessions). Extended by: N32a-26 (M3, `--workflow`).

**`schema`** (MCP tool `schema`; owner `frob-verbs`)
- Inputs: at most one of:
  - a positional argument: a schema identifier (it always carries `/<version>`, for example `frob.design/1`) or a
    verb name (it never does, for example `sim.run`), which is the same request as `--verb <name>` (DESIGN: about
    a dozen scripts call `frob schema <verb>` positionally, N16b-27, N16c-15, N28c-21, N30b-37, N33b-19, N36-33
    among them, and the two kinds of name cannot collide, so admitting both costs nothing and breaks no caller);
  - `--verb <name>` (both schemas of the verb and, for `run` and `mutate` verbs, the common fields added by kind),
    optionally narrowed by `--in` or `--out` (BD-022's form; `--in` and `--out` also narrow a positional verb name);
  - `--list` (every schema identifier with kind, owning crate and version; paged, so `paginated = true` and the
    generator appends 806 and 807);
  - `--refusals`, or no argument at all (the same request; DESIGN: N35-33, N35x-01, N35x-02, N31b-45 and N19-REF
    read the refusal table with no argument, N31-REF, N25-REF, N28-REF and N20-REF with `--refusals`): the refusal
    table served from `registries/refusals.toml`, every row with code, name, message, exit, retry, ranked repairs,
    `defined_in` and `raised_by`, together with the interim list of the allocation ledger
    `registries/refusal_allocations.toml` (each held request with its slug, its interim code and the `detail` or
    `pointer` value that tells it apart, the bead that owns the interim use, and its ledger status; a held request
    with no fitting interim listed with none; N35-33);
  - `--verbs` (every verb with kind, required scopes and its full refusal list, chapter codes and appended codes
    shown apart, each appended code with its source, `kind:<kind>` or `field:<field>`, a held refusal in its
    interim form; read by N28c-29, N28cx-01 and N37-15);
  - `--path-args` (every verb argument that accepts a path, with its root, access and parameter kind; read by
    N37-12); `--exports` (every row with `exports = true`, with the kind of artifact, the carrier and pointer of
    its flags, their spelling and count; read by N37-24); `--echoing` (every verb that echoes imported text, with
    its marked fields; BD-304's list, read by N37-10).
  The list forms other than `--list` take no further argument; two forms together are `FROB-REF-802`.
- Outputs: `frob.schema.out/1`, with exactly one member, named by the form (DESIGN: one output schema, so the row
  keeps one `output_schema`; members are added, never changed): for an identifier or a verb, `{ schema, example,
  example_digest, notes }`: the JSON Schema and its minimal valid example (every required field present, every
  optional field absent, validated at build time and accepted by the real parser), with `notes` giving each
  field's unit and integer type (`int53`, `u64str`, `i64str`); `list`; `refusals` (with `interims`); `verbs`;
  `path_args`; `exports`; `echoing`. Every list member is generated from the registries the binary loads, so it
  cannot fall behind them. Evidence: *operational*.
- Chapter refusals: none of its own. An unknown identifier or verb name is the appended `FROB-REF-802` with
  `pointer` (`/id` or `/verb`) and the three nearest names of either kind; an identifier at a major version this
  build does not hold (`frob.design/2` where the build has `frob.design/1`) is an unknown identifier, so its
  nearest names include the version held (BD-022's rule, which N32b-20 keeps). The verb reads no artifact, so
  the version gate's `FROB-REF-891` has no raising site on this row.
- Implemented by: BD-022 (M0: identifiers and the positional verb name, `--verb`, `--in`, `--out`, `--refusals`
  and the bare form with N35-33's interim list, `--verbs`, `--path-args`, and the listing interface
  `SchemaListing` through which a later bead adds a list form), N32b-20 (M2: the schema catalogue with
  validated examples and `--list`; from M3 the resource `frob://schema/{id}`, N32a-25), N37-23 (M1: `--exports`,
  the listing of its locator registry `registries/exports.toml`, contributed through BD-022's listing
  interface), BD-304 (M3: `--echoing`). A list form whose bead is not in the build is an unknown parameter, 802.

**Machine fields of each row.**

| Verb | Capabilities | Output schema | Task clusters | Example |
|---|---|---|---|---|
| `doctor` | none (`[]`, `public = true`) | `frob.doctor.out/1` | `orient` | `frob doctor --json` |
| `capabilities` | none (`[]`, `public = true`) | `frob.capabilities.out/1` | `orient` | `frob capabilities --json` |
| `robot-docs` | none (`[]`, `public = true`) | `frob.robot-docs.out/1` | `orient` | `frob robot-docs --cluster author --json` |
| `schema` | none (`[]`, `public = true`) | `frob.schema.out/1` | `orient` | `frob schema frob.design/1 --json` |

Capabilities (DESIGN, requested by the implementing beads N32b-17, N32b-19 and N32b-20): the four orientation
verbs need no scope. An agent cannot learn which scopes to ask for, or what the build can do, without reading
them, so they must answer any session, including one whose token carries no scope. The rows write
`capabilities = []` explicitly with `public = true` (lint RL-10 of N32a-02); the scope `operator:read` of the
earlier rows is withdrawn. The surface-wide 720 stays in the appended list by rule but has no raising site on
these rows: a session is never refused one of them for a missing scope.

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
capabilities = []
public = true
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
2. The generator adds the common input properties of the kind and appends the surface-wide refusals. This item
   mirrors the kinds table of NJL-01, which governs wherever the two differ. Common input properties: `run`:
   `sync_wait_ms` (0 to 30,000; MCP default 5,000; the command line waits to completion), `idempotency_key`
   (`--key`), `budget`, `estimate_only` (`--estimate`), `on_cancel` (`leave`, the default, or `cancel_job`) and
   `envelope_scope` (`--envelope-scope <gate:…|campaign:…|pretrain:…>`: the approved compute-envelope scope the
   job cites; a job's scope is the token of a launcher it runs under, then a cited `envelope_scope`, then a
   scope the verb derives from its own inputs, otherwise `adhoc`, whose class in force is E0); `mutate`:
   `idempotency_key` and `expected_parent`; `read` and `admin`: none. `idempotency_key` is never in the
   JSON-schema `required` array: the annotation `x-frob-key` (`"required"`, or `"optional"` on a `key_optional`
   entry) carries the rule, and a call without a key is refused with 805 by the admission transaction (N32a-09)
   on both surfaces, never as a schema failure. Appended by kind: `FROB-REF-720` (scope missing), 723 (token
   invalid), 724 (not implemented in this build) and 802 (schema failure, including an unknown parameter) for
   every verb; 701 to 704 (budget and admission), 761 (above the compute envelope in force), 803 (key reused
   with different content) and 805 (missing key) for `run`; 725, 803, 804 (stale parent) and 805 for `mutate`.
   Appended by field, whatever the kind: 806 and 807, with the inputs `cursor`, `limit` and `follow`, for
   `paginated = true`; 808 (an integer outside ±(2⁵³ − 1)) for an entry with an `int53` input property, which
   every `run` entry has in `sync_wait_ms` and an entry without one never gets; no 805 for a `key_optional`
   entry; `estimate_only` for a non-`run` entry with `estimable = true`; 790 for a non-empty `needs_groups`; 891
   to 893 for artifact `params`; 734 and 828 for a parameter that accepts a path to a file
   (`artifact-input<F>`, `artifact-identity<F>`, `foreign-file`). The later variations of NJL-01's table (10
   onward: the inputs `override` and `override_reason`, `from_proposal`, `machine` and `engine`, which the beads
   named there add to some or all `run` entries) are not copied here: that table is the one list.
3. Exit codes: 0 done; 2 done with findings; 3 refused input (1xx to 5xx, and 8xx other than conflicts); 4 budget
   (70x and 761); 5 capability (72x, 73x); 6 device (6xx); 7 conflict (803, 804, 809); 8 accepted and still running; 9
   indeterminate; never 1 on purpose.
4. Rows are entered when the cluster is first registered (M0), each with the `task_clusters` of the
   machine-fields table, which is NJL-05's target assignment. No `[[task_cluster]]` table exists at M0: NJL-05
   declares the tables at M1 and seats these rows in them (its staging rule, step 1), and from then
   `task-cluster-mismatch` holds for them. A verb whose handler is not in the build answers one envelope with `FROB-REF-724`, exit 5, on both surfaces, and `frob capabilities --json`
   lists it as absent; a reduced form that lands before the row's `first_milestone` answers and is listed as
   implemented with its form named (NJL-01, NJL-03: the field keeps the appendix's milestone).

**Notes for this cluster.**

- These four are the only registry names without a dot; their registry cluster is `operator` and their CLI path is
  the bare name (`frob doctor`).
- `doctor` and `capabilities` exist in a first form at M0 (BD-022) and in their complete form at M2 (N32b-20,
  N32b-19); the complete forms add sections and output blocks and change nothing in the M0 ones (additive, same
  major version).
- `frob capabilities --json` lists what is implemented in the eight-condition sense (code in its crate, a contract,
  known-answer tests against a pinned oracle, evidence on outputs, a CPU reference path, reachability through `frob
  --json`, cancellation tested, the gate recorded with measured values) and what is not.
- `robot-docs`, the MCP resources under `frob://docs/` and the MCP prompts are generated from the same source, so
  the three cannot drift.
- `frob <command path> --help --json` is a built-in of BD-022's parser, not a verb: it runs no handler, exits 0
  and returns one envelope whose `result` is `{ entry, input_schema, output_schema, refusals, examples }`, each
  refusal with `code`, `meaning`, `exit`, `repairs` and `source` (`chapter`, `kind:<kind>` or `field:<field>`,
  with the action's name where the entry has actions); `robot-docs` and `schema --verbs` print the same sources.
  The help snapshots `fixtures/golden/verbs/<cluster>/<verb>.help.json` of every cluster are taken in this form.

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
      kind and fields (NJL-01), and every code exists in `registries/refusals.toml`.
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
  none; `schema`: none, and a planted 891 on `schema` fails the comparison); the
  appended codes equal the rule for its kind and fields (`capabilities` and `schema` are paginated and also get
  806 and 807; once N33c-32's change is in, `capabilities` also gets 790 with source `field:needs_groups`,
  which N33c-32's `capabilities_require_group` asserts).
- `public_rows`: all four rows have `capabilities = []` and `public = true`; a planted `operator:read` on one of
  them fails; a session whose token has no scope calls each verb and gets exit 0 or 2, never 720.
- `doctor_sections`: the section table holds the names of the `doctor` block (`toolchain`, `stack`,
  `stack_receipt`, `upstream`, `devices`, `kernel_cache`, `probes`, `daemon`, and from their beads `store`,
  `kernels`, `ledger`, `registries`, `referee`); `--section kernelz` is 802 with `pointer: /section`;
  `--section` is repeatable.
- `inputs_by_row`: `capabilities` accepts `--verb`, `--device`, `--status`, `--cluster`, `--milestone`,
  `--missing`, `--limit`; `robot-docs` accepts `--cluster`, `--verb`, `--budget`, `--encoding`, `--workflow`;
  `schema` accepts a positional identifier or verb name, `--verb` with `--in` or `--out`, `--list`,
  `--refusals`, `--verbs`, `--path-args`, `--exports`, `--echoing`, and no argument.
- `schema_positional_verb_equals_flag`: `frob schema sim.run --json` and `frob schema --verb sim.run --json`
  return byte-identical results; `frob schema frob.design/1 --json` resolves as an identifier; `frob schema
  sim.rnu --json` is 802 with nearest names that include `sim.run`; `frob schema frob.design/2 --json` is 802,
  `pointer: /id`, with `frob.design/1` among the nearest names.
- `schema_bare_equals_refusals`: the bare form and `--refusals` return byte-identical results; the `refusals`
  member holds one row per code of `registries/refusals.toml` and the `interims` list equals the ledger's held
  requests (each with interim code and `detail`, or `none`); a planted held request with no interim in a
  scratch ledger appears with `none`, never omitted.
- `schema_list_forms_generated`: `--verbs` has one entry per registry row with chapter and appended codes apart;
  `--path-args` lists exactly the parameters whose kind accepts a path; `--exports` lists exactly the rows with
  `exports = true`; a scratch registry with one more such row adds one entry to each (the lists are read, not
  typed); two forms together (`--list --refusals`) are 802.
- `capabilities_milestone_forms`: on an M0 build the output has `capabilities[]` and `summary` and no `build`,
  `gates[]` or `devices[]` member, and `--device` is 802 (the option arrives with N32b-19 at M1); with BD-022's
  fail-closed `StatusSource` no verb is `implemented`.
- `robot_docs_workflow`: `--workflow studio_loop` returns the `workflow` member with eight steps; `--workflow
  nope` is 802 (once N32a-26 is in the build; before that the option is 802 as an unknown parameter).
- `mcp_names_one_to_one`: the tool names are the mapped names (`doctor`, `capabilities`, `robot_docs`, `schema`),
  with no collision against any other cluster.
- `task_clusters_and_examples`: every row sits in a task cluster of at most seven tools or is a base tool; each
  example parses against the generated CLI and its command path is the row's own.
- `kind_behaviour`: a verb whose capability is absent returns code 724, exit 5, one envelope.

### End-to-end script and logging

`scripts/e2e/verbs_operator.sh` (header: `bead=VERBS-operator milestone=M0 labels=tether,verbs,verb:operator.*
gates=GATE-TETHER-02 seed=1 needs=cpu,mcp,frobd plants=alias,missing-row,renamed-field,interim-dropped`) sources the shared helper
and `scripts/e2e/verbs_lib.sh` and calls `verbs_cluster_conformance operator`. Steps: (1) `frob capabilities --json`
and `frob robot-docs --json` list the four verbs with kind and milestone; (2) `frob-surface-diff parity --cluster
operator` over stdio and through `frobd --mcp`, every difference list empty; (3) `frob-surface-diff snapshot --check
--cluster operator`; (4) each verb's registry example verbatim, expecting a result (exit 0 or 2), a job handle (exit
8, then `frob job wait`) or `FROB-REF-724` exactly when the capability report says the verb is absent; (5) for each
implemented verb the same call as an MCP tool, the two envelopes agreeing on `verb`, `ok`, `exit`, `result`,
`refusal`, `evidence`, `bound`, `warnings` and `next_actions`; (6) per verb one call with a misspelt parameter
(`e2e_expect_refusal FROB-REF-802 3`); (7) one envelope per call and no exit code 1; (8) each of the four verbs
called over MCP with a token that carries no scope answers (no 720); (9) `frob doctor --section <name> --json`
for each section of the table and `frob doctor --section kernelz --json` (`e2e_expect_refusal FROB-REF-802 3`,
pointer `/section` logged); (10) `frob capabilities --limit 10 --json` paged to the end, then the cursor reused
with a changed filter (`e2e_expect_refusal FROB-REF-806 3`); (11) `frob schema --verb doctor --out --json` and
`frob schema --list --limit 20 --json`; (12) `frob schema doctor --out --json` compared byte for byte with step
(11)'s first call; `frob --json schema` and `frob schema --refusals --json` compared byte for byte, with the row
count logged against `registries/refusals.toml` and the interim count against the ledger's held requests; (13)
`frob schema --verbs --json`, `frob schema --path-args --json` and `frob schema --exports --json`, each count
logged against the registry rows it is generated from (a form whose implementing bead is not yet in the build
answers 802 and is logged `skipped: form absent`, never as passed); (14) from M3, `frob robot-docs --workflow
studio_loop --json` (eight steps) and `frob robot-docs --workflow nope --json` (`e2e_expect_refusal
FROB-REF-802 3`).

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
  `renamed-field` (an output property renamed without a version change: the snapshot check reports `breaking`);
  `interim-dropped` (a held request removed from the served interim list while the ledger still holds it: step
  (12)'s count differs). A verb answering with two envelopes or exit code 1 fails step (7).
- Invariants carried: INV-TETHER-01 (every surface is generated from the registry; nothing exists on one surface and
  not the other, apart from declared per-surface defaults and the unknown-verb case) as step (2); INV-TETHER-02
  (exactly one envelope per invocation with `--json`) as step (7). Determinism lanes: entries, tool definitions and
  snapshots byte-identical twice on one machine, across debug and release and across Linux and macOS (class D0); the
  verbs' own lanes belong to their implementing beads.

## Considerations

- Closing rule: the bead closes at M0, when the rows are registered, the surfaces match and every verb passes its
  example or answers the not-implemented refusal. The per-milestone surface beads (BD-022) re-run this script as
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

Appendix J (J.1 entry form; J.3 added rows; J.5 task clusters), §32.17, §32.4, §32.6, §32.12.3, §32.18, §32.21;
GATE-TETHER-02; INV-TETHER-01, -02; KA-755.
