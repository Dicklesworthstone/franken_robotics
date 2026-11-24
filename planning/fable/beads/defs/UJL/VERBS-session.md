---
key: "VERBS-session"
title: "frob-verbs registry: cluster `session`, four rows, with parity, e2e and schema-snapshot checks"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "tether", "frob-verbs", "solid", "registry", "verb", "test", "review-notes-open"]
parent: "WP-06"
deps: ["BD-022", "NJL-01", "NJL-02", "NJL-04", "STD-E2E"]
needs: []
related: []
gates: ["GATE-TETHER-02"]
plan_refs: ["Appendix J (J.1, J.2, J.3, J.5)", "§32.12.4", "§32.13", "§32.6", "§32.12.3", "§32.21 (GATE-TETHER-02)"]
---
## Background

FrankenRobotics (a robotics simulation and design system) is reached through the command line `frob`, the daemon
`frobd` and the MCP server `frob-mcp` (the Model Context Protocol, by which an agent harness lists and calls tools).
Every capability is a verb, `cluster.verb`, declared once in `registries/verbs.toml` (crate `frob-verbs`, tier
Solid). The CLI, the daemon protocol, the MCP tools, the schemas, the refusal table and the agent guide are
generated from that file, and gate GATE-TETHER-02 compares the served surfaces with it. The registry holds 244 verbs
in 44 clusters (namespaces): the 243 of the specification and `artifact.import` (VERBS-artifact); the
count of record is NJL-03's.

This bead is the registry of record for the cluster `session`. The cluster covers sessions: opening one with scopes
and budgets, showing scopes, budgets left, namespace and focus, switching the visible task cluster, and closing. It
has four verbs (1 `read`, 3 `mutate`); the handlers live in `frob-verbs`. The bead carries every row completely,
with the three checks that keep the rows true: the generated CLI commands and MCP tools match the list; every verb
has an end-to-end script; the schema printed by `frob <verb> --help --json` is stable.

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

In: the four registry entries of cluster `session`; the MCP tool names; the cluster's conformance script; the
help-schema snapshots; each verb's answer before its milestone. Out: behaviour (the implementing beads named below);
the entry schema and lints (NJL-01); the conformance harness (NJL-02); the alias lint (NJL-04); task clusters
(NJL-05, NJL-06); generation of the surfaces (the Tether seed beads).

## Technical approach

**The rows.**

| Verb | Kind | First milestone | Tier | Purpose | Defined in |
|---|---|---|---|---|---|
| `session.focus` | mutate | M0 | S | Switch the visible task cluster; lists the now-visible tools | §32.12.4 |
| `session.open` | mutate | M0 | S | Open a session with scopes and budgets (tokens and budgets at M2) | §32.13 |
| `session.show` | read | M0 | S | Scopes, budgets left, namespace and focus | §32.13 |
| `session.close` | mutate | M0 | S | Close a session and release its reservations | §32.6 |

**Contract of each verb.** Evidence labels used below (the `evidence` block of the envelope):
- *operational*: no claim: an operational answer about the session, the store, a job or a device; `evidence.claims`
  is empty.

**`session.focus`** (MCP tool `session_focus`; owner `frob-verbs`)
- Inputs: `cluster` (the task cluster to focus); the idempotency key is accepted but not required (`key_optional =
  true`, lint `RL-21` of N32a-02: the mutation sets the caller's focus to the requested cluster, so a repeat
  changes nothing; the plan's worked session calls `session_focus {cluster}` with no key), so its appended list
  omits `FROB-REF-805`.
- Outputs: the new focus and the names and one-line purposes of the now-visible tools. Evidence: *operational*.
- Chapter refusals: none of its own.
- Implemented by: BD-138.

**`session.open`** (MCP tool `session_open`; owner `frob-verbs`)
- Inputs (N32b-04): `scopes` (catalogue scopes, non-budget); `budget` (`gpu_s`, `cpu_core_s`, `disk_bytes`, each an
  `int53` count, and optional caps `wall_s`, `sim_s`, `device_mem_bytes`, `trace_bytes`, `episodes`; stored as
  the budget scopes `gpu:seconds=N`, `cpu:core_seconds=N`, `disk:bytes=N`); `namespace` (optional; root sessions
  only); `principal` (optional; root sessions only: the agent principal whose `principal.<id>` grant in the
  owner-held safety configuration, N37-02, the root token is minted from; an unlisted principal is the appended
  `FROB-REF-723`, nothing to mint from; given with `derive` it is `FROB-REF-802`, because a derived session inherits
  its parent's principal, N32b-02; CLI `--principal agent-17`, as N37-15's script opens `--principal agent-a` and
  `--principal agent-b`); `derive` (Boolean, CLI `--derive`, as BD-258 item 4 and N32b-04 spell it: derive from
  the caller's own token, which travels in `FROB_SESSION_TOKEN` or `--session-token-file` and never as an argument,
  N32b-01 item 5; in the schema from M2, refused with `FROB-REF-724`, reason `not-linked`, until N32b-02 lands at
  M3); `ttl_s` (optional, at most the daemon's maximum); idempotency key (required).
- Defaults and refusals of a root call (N32b-04, the verb's statement of N32b-01's minting rule): without
  `principal` the session is minted for the one principal the configuration grants when it grants exactly one
  (so the plan's worked call without a principal stays valid on a single-agent host); when it grants several, the
  call is `FROB-REF-802` with `pointer: /principal` and the granted identifiers in the repair; an unlisted
  principal, or any root call when none is granted, is `FROB-REF-723`. Omitted `scopes` grant no non-budget scope
  (the session can call `public` verbs and read itself; authority is always named, N32bx-02); an omitted budget
  unit takes the grant's amount of that unit; the output echoes the granted set and table. A request beyond the
  grant (a scope, an amount, a lifetime above `max_lifetime_s`, a namespace other than the grant's) is
  `FROB-REF-723` naming the widening element. A root call that presents a valid token is `FROB-REF-723` (a holder
  derives, N32b-01), so over MCP, where every call carries a token, `session_open` is derive-only.
- Outputs: session identifier, namespace, principal, the scopes (sorted) and the budget table recorded, expiry,
  parent (or null), the focus (`orient`) and the capability token (schema-marked `x-frob-secret`: printed once,
  redacted by every logger; tokens and budgets from M2). Evidence: *operational*.
- Chapter refusals: none of its own (an agent's request to mint an `owner_only` scope is the appended
  `FROB-REF-723`).
- Implemented by: BD-138, N32b-04, BD-258. Extended by: N32b-02 (`derive`, M3).

**`session.show`** (MCP tool `session_show`; owner `frob-verbs`)
- Inputs: none for the caller's own session, which is the session of the caller's token (the process option
  `--session` carries that identifier, and N32b-01 refuses a different one with `FROB-REF-802`, so the row has no
  input named `session`: NJL-01's lint `common-property-shadowed` would fail it); `all` (CLI `--all`: every
  session's budget table; option capability `session:read`, N32b-04); `transcript` (CLI `--transcript`: the call
  transcript `frob.session-transcript/1`, N32bx-01: a session reads its own and those of sessions derived from it)
  with optional `of_session` (CLI `--of-session <id>`, N32bx-01's name for the session whose transcript is read;
  another session's is the `audit` action below); `phases` (CLI `--phases`, M1, N28a-03: the phase intervals of
  the caller's session). Paginated (`paginated = true`: the derived children, the `all` listing, the transcript
  and the phase intervals are paged with the authenticated cursor), so the row has no `limit`, `cursor` or
  `follow` of its own: the generator adds them, and the appended list adds 806 and 807.
- Outputs: scopes, budgets left, namespace and focus, visible tools; from M2 the daemon's idle reserve line; from
  M3 the open and closed derived sessions with each child's caps, spent and left (N32b-02); with `transcript`
  the rows, cursor and frontier; with `phases` the block `phases` (N28a-03): the session clock (`opened_ms`, and
  `now_ms` or `closed_ms`), the interval records of the session's verb calls with their phase and resource costs,
  each also in the raw-record form of the ten-fold ledger (N39a-23: `kind = "work"`, `phase` its ledger phase,
  `start_ms` and `end_ms` in integer milliseconds on the daemon's monotonic clock, `resource`, `proc` the job or
  call identifier, `tag_rule = "verb-phase:<verb>"`), and per ledger phase the summed (non-exclusive) seconds.
  Evidence: *operational*.
- Actions: the default action is `read` with no capability; `audit` (kind `read`, capability `session:audit`, an
  owner-held named scope of N32bx-01) is selected in dispatch when `transcript` with `of_session` names a session
  that is neither the caller's nor one of its descendants: `actions = [{ name = "audit", kind = "read",
  capabilities = ["session:audit"] }]` (N32a-02's two-kind form; requested by U32b). Without the scope the call
  is `FROB-REF-720` with `required: "session:audit"`.
- Chapter refusals: none of its own.
- Implemented by: BD-138, N32b-04. Extended by: N28a-03 (`phases`, M1), N32b-02 (derived children, M3), N32bx-01
  (`transcript`, `of_session` and the `audit` action, M3; they answer `FROB-REF-724` before it lands).
- Which test compares what: N32b-04's `registry_rows_match` compares the row's entry fields field for field and
  excepts only N32bx-01's (`transcript`, `of_session`, `actions`), compared from the build that carries them; the
  input `phases` is compared by N28a-03's own test `session_show_phases_raw_records` and by this bead's
  help-schema snapshot, and N28a-29's script calls `frob session show --phases --json`.

**`session.close`** (MCP tool `session_close`; owner `frob-verbs`)
- Inputs: the idempotency key; the session closed is the caller's own, identified by its token (BD-138).
- Outputs: the reservations released and the final spend; from M3 closing a session closes its open descendants
  in the same transaction, deepest first, each returning its unspent budget upward (N32b-02). Evidence:
  *operational*.
- Chapter refusals: none of its own.
- Implemented by: BD-138. Extended by: N32b-02 (return of unspent budget and cascade, M3).

**Machine fields of each row.**

| Verb | Capabilities | Output schema | Task clusters | Example |
|---|---|---|---|---|
| `session.focus` | none (`[]`, `public = true`; `key_optional = true`) | `frob.session.focus.out/1` | `base` | `frob session focus mission --json` |
| `session.open` | none (`[]`, `public = true`; `option_capabilities = {}`) | `frob.session.open.out/1` | `housekeeping` | `frob session open --scopes design:write,sim:run --budget gpu_s=600,cpu_core_s=7200 --key s-1 --json` |
| `session.show` | none (`[]`, `public = true`); `option_capabilities = { all = ["session:read"] }`; action `audit`: `session:audit`; `paginated = true` | `frob.session.show.out/1` | `orient` | `frob session show --json` |
| `session.close` | none (`[]`, `public = true`) | `frob.session.close.out/1` | `housekeeping` | `frob session close --key sc-1 --json` |

Capabilities (DESIGN, corrected in round 1 from `session:write` and `session:read`, after N32b-01 and N32b-04):
reading, focusing or closing one's own session needs no scope, and a root `session.open` has no prior token to
hold one (`derive` is checked by the attenuation order of N32b-02, not by a scope). The plan's worked MCP session
holds six scopes, none of them `session:*`, and calls `session_focus` and `session_show`; a row citing
`session:write` would refuse that session with 720, and N32b-01's catalogue has no `session:write` row (its write
scopes are for `mutate` verbs that create or change named objects). The scopes the cluster uses are option and
action scopes: `session:read` for `session.show --all` (also `job.list`'s `all_agents`, N32b-03) and
`session:audit` for the `audit` action.

The remaining fields follow by rule: `input_schema` is `frob.<verb name>.in/1`; `refusals` is the chapter list of
the verb's block; `idempotent` is `true` for `read` and `admin` and `false` for `run` and `mutate` (which take an
idempotency key); `cost_model` is `static: …` for `read`, `mutate` and `admin` and the `--estimate` cost model for
`run`; a `run` row carries `wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }`; `owner` is the
crate of the block; `purpose` is the Purpose column.

One entry in full (the others follow field for field):

```toml
[[verb]]
name = "session.focus"
cluster = "session"
kind = "mutate"
tier = "S"
capabilities = []
public = true
key_optional = true
input_schema = "frob.session.focus.in/1"
output_schema = "frob.session.focus.out/1"
refusals = []
idempotent = false
cost_model = "static: one ledger transaction"
first_milestone = "M0"
task_clusters = ["base"]
examples = ["frob session focus mission --json"]
owner = "frob-verbs"
purpose = "Switch the visible task cluster; lists the now-visible tools"
```

**Conventions the rows follow.**

1. The MCP tool name is the verb name with `.` and `-` replaced by `_`. The output on the wire is one envelope
   `frob.envelope/1` with `result` specialized to the output schema; its `evidence` block is a claim product (the
   weakest class, tier and determinism class among the inputs).
2. The generator adds the common input properties of the kind (`run`: `sync_wait_ms` from 0 to 30,000 with MCP
   default 5,000, `idempotency_key`, `budget`, `estimate_only`, `on_cancel` (`leave`, the default, or
   `cancel_job`) and `envelope_scope` (`--envelope-scope <gate:…|campaign:…|pretrain:…>`: the approved
   compute-envelope scope the job cites; N36-20 takes a job's scope from a launcher's token, then from this
   input, then from the verb's own inputs, and otherwise `adhoc`, whose class in force is E0, and a citation that
   contradicts the launcher's token is refused); `mutate`: `idempotency_key`, `expected_parent`).
   `idempotency_key` is in no JSON-schema `required` array: the annotation `x-frob-key` (`"required"`, or
   `"optional"` on a `key_optional` entry) carries the rule, and the admission transaction (N32a-09, from M1)
   refuses a missing key with 805 on both surfaces. The generator appends the surface-wide refusals:
   `FROB-REF-720` (scope missing), 723 (token invalid), 724 (not implemented in this build) and 802 (schema
   failure, including an unknown parameter) for every verb; 701 to 704 (budget and admission), 761 (above the
   compute envelope in force), 803 (key reused with different content) and 805 (missing key) for `run`; 725, 803,
   804 (stale parent) and 805 for `mutate`. What a field or a registry table adds follows NJL-01's variation
   table, which is the one list of what the generator adds: 806 and 807 for `paginated = true`; 808 for an entry
   with an `int53` input at any depth, whatever its kind (every `run` entry has one, `sync_wait_ms`); a
   `key_optional` entry is not given 805; a non-`run` entry with `estimable = true` gets `estimate_only`; 790 for
   a non-empty `needs_groups`; 891 to 893 for artifact `params`; 734 and 828 for a parameter that accepts a path
   to a file (`artifact-input<F>`, `artifact-identity<F>`, `foreign-file`). Its later variations, 10 to 16
   (`override` with `override_reason`, `from_proposal`, `machine`, `engine`, `no_memo`, the reality guard's seven
   codes, and `diagnostic_override` with the admission checks' codes), are read from that table and not copied
   here.
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

- `session.focus` is a base tool: always listed, never counted against a task cluster's seven. Focus changes what is
  listed, never what is permitted; authority is the capability token.
- The MCP half of `session.focus` is the one hand-written per-verb code path: it enables the tools of the target
  cluster and disables those of the previous one, and its result lists the names and one-line purposes of the
  now-visible tools for clients that do not act on the catalog-changed notification.
- The inventory gives the cluster's first milestone as M0 (tokens and budgets at M2); the seed bead that names the
  four verbs (BD-138) is at M1. At M0 the rows are registered (BD-022 declares them) and all four answer
  `FROB-REF-724` until BD-138 lands at M1, which NJL-02's capability-level check accepts as an expected refusal
  because the four are declared late rows of its closed list `LATE_ROWS` (lands M1; an M1 build without
  BD-138's handlers fails the check);
  the M2 forms with scopes, budgets and the token are N32b-04's and BD-258's. (An earlier sentence said
  `session.show` answers at M0 for an implicit single-user session; no bead builds that, and BD-022 says the four
  answer 724 until BD-138, so it is withdrawn.)

## Acceptance criteria

- [ ] `registries/verbs.toml` holds exactly four entries with `cluster = "session"` (1 `read`, 3 `mutate`), each
      with every field of the entry schema, an example and a task-cluster list; the alias lint finds no second spelling.
- [ ] `frob-surface-diff parity --cluster session` exits 0 in process (`frob-mcp` over stdio) and through `frobd
      --mcp`: registry, CLI and MCP each count 4, and every difference list is empty apart from the harness's closed
      exemptions.
- [ ] Every verb has an end-to-end script: `scripts/e2e/verbs_session.sh` runs each verb of the cluster (four), and
      from a verb's first milestone a capability script of its implementing bead exists (`frob-surface-diff
      e2e-coverage`).
- [ ] `frob <verb> --help --json` equals `fixtures/golden/verbs/session/<verb>.help.json` byte for byte for each of
      the four, on two runs, debug and release, Linux and macOS; a planted renamed output property is classified
      `breaking` and refused.
- [ ] A verb not yet implemented answers `FROB-REF-724`, exit 5, with one envelope on both surfaces; an implemented
      verb runs its registry example with the result its implementing bead specifies; a misspelt parameter is
      `FROB-REF-802`, exit 3; a `mutate` call without a key is `FROB-REF-805`, exit 3, except `session.focus`
      (`key_optional`), which succeeds without one.
- [ ] A token holding no `session:*` scope can call `session.focus`, `session.show` (own session) and
      `session.close`; `session.show --all` without `session:read` is `FROB-REF-720`; once N32bx-01 has landed,
      `session.show --transcript --of-session <foreign>` without `session:audit` is `FROB-REF-720` with `required:
      "session:audit"`, and with a derived session's identifier it answers.
- [ ] The generated refusal table lists for each verb exactly its chapter codes plus the codes appended for its
      kind and fields (NJL-01), and every code exists in `registries/refusals.toml`.
- [ ] The gate row is recorded with the measured counts by a session that did not write the rows.

## Test plan

### Unit tests

`cargo test -p frob-verbs --test cluster_session` (the real generated table; no mocks; planted registries are TOML
strings labelled `synthetic`):
- `rows_present`: exactly 4 rows in the cluster, with the names of the rows table; a planted extra row or a removed
  row fails.
- `kinds_milestones_tiers`: each row's kind, first milestone and tier equal the rows table (for example
  `session.close` is `mutate` at M0).
- `schemas_and_scopes`: input schemas are `frob.<name>.in/1`; output schemas and capabilities equal the
  machine-fields table.
- `refusal_lists`: each row's chapter codes equal its block (`session.focus`: none; `session.open`: none;
  `session.show`: none; `session.close`: none); the appended codes equal the rule for its kind and fields (`session.focus`
  without 805; `session.show` paginated, with 806 and 807).
- `scopes_by_rule`: every row has `capabilities = []` with `public = true`; `session.show` has `option_capabilities
  = { all = ["session:read"] }` and the action `audit` with `["session:audit"]`; `session.open` has
  `option_capabilities = {}`; a planted `session:write` on any row fails against the scope catalogue.
- `focus_key_optional`: `session.focus` has `key_optional = true` and succeeds without a key; the other two
  `mutate` rows refuse a missing key with 805.
- `open_inputs`: `session.open`'s input schema holds `scopes`, `budget`, `namespace`, `principal`, `derive` and
  `ttl_s`; `principal` with `derive` is 802; a planted `derive_from` property fails (the token never travels as an
  argument).
- `show_inputs`: `session.show`'s input schema holds `all`, `transcript`, `of_session` and `phases` and the
  generated `cursor`, `limit` and `follow` once each; a planted own `session` property, or an own `limit`, fails
  the build with `common-property-shadowed`.
- `mcp_names_one_to_one`: the tool names are the mapped names (`session_focus`, `session_open`, `session_show`,
  `session_close`), with no collision against any other cluster.
- `task_clusters_and_examples`: every row sits in a task cluster of at most seven tools or is a base tool; each
  example parses against the generated CLI and its command path is the row's own.
- `kind_behaviour`: a `mutate` row has `idempotency_key` and `expected_parent` and, unless `key_optional`, refuses a
  call without the key with code 805; a verb whose capability is absent returns code 724, exit 5, one envelope.

### End-to-end script and logging

`scripts/e2e/verbs_session.sh` (header: `bead=VERBS-session milestone=M0 labels=tether,verbs,verb:session.*
gates=GATE-TETHER-02 seed=1 needs=cpu,mcp,frobd plants=alias,missing-row,renamed-field`) sources the shared helper
and `scripts/e2e/verbs_lib.sh` and calls `verbs_cluster_conformance session`. Steps: (1) `frob capabilities --json`
and `frob robot-docs --json` list the four verbs with kind and milestone; (2) `frob-surface-diff parity --cluster
session` over stdio and through `frobd --mcp`, every difference list empty; (3) `frob-surface-diff snapshot --check
--cluster session`; (4) each verb's registry example verbatim, expecting a result (exit 0 or 2), a job handle (exit
8, then `frob job wait`) or `FROB-REF-724` exactly when the capability report says the verb is absent; (5) for each
implemented verb the same call as an MCP tool, the two envelopes agreeing on `verb`, `ok`, `exit`, `result`,
`refusal`, `evidence`, `bound`, `warnings` and `next_actions`; (6) per verb one call with a misspelt parameter
(`e2e_expect_refusal FROB-REF-802 3`) and, for `run` and `mutate` verbs other than `session.focus`, one without a
key (`e2e_expect_refusal FROB-REF-805 3`), while `session.focus` without a key is expected to succeed; (7) one
envelope per call and no exit code 1; (8) from the build whose capability report lists BD-138's handlers, with a
token holding no `session:*` scope: `session focus`, `session show` and `session close` succeed, `session show
--all` is `e2e_expect_refusal FROB-REF-720 5`, and from N32bx-01 a foreign `--transcript --of-session <id>` is the
same refusal with `required: "session:audit"` logged.

Log: `target/e2e-logs/verbs_session/<utc>.ndjson` with a transcript beside it. Each `cmd` line has the exact
command, exit code, wall time and the envelope's `evidence` block; each `assert` has observed and expected values
and the tag `--gate GATE-TETHER-02`; the metrics `registry_verbs`, `cli_verbs` and `mcp_tools` are logged against 4
and the difference counts against 0; refusals are logged with code and first repair; a failure logs the first
differing verb and field; the `summary` line gives assertions passed and failed and the gate touched.

### Known answers and negative controls

- Known answers: the cluster has 4 verbs (1 `read`, 3 `mutate`); the tables above. KA-755 (the exit-code table on
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

- OPEN-TETHER-2 (owner: Tether lead): whether agent harnesses act on catalog-changed notifications. Fail-closed
  default: static clusters by launch flag plus the compatibility tool `verb_invoke`.
- Closing rule: the bead closes at M0, when the rows are registered, the surfaces match and every verb passes its
  example or answers the not-implemented refusal. The per-milestone surface beads (BD-022 at M0; BD-137 at M1,
  when BD-138's handlers land and the four rows leave `LATE_ROWS`) re-run this script as verbs land; that is how
  GATE-TETHER-02 is re-measured.
- Scope names follow the naming rule decided in BD-258 (no longer proposals): a row cites scopes of its own
  cluster by kind (`<cluster>:read`, `<cluster>:run`, `<cluster>:write`, `<cluster>:admin`) plus the named scopes
  of the surface (here `session:read` for `--all` and the owner-held `session:audit` of N32bx-01), never a scope
  named after the object read; N32b-01's catalogue generates the forms, and a `run` or `write` scope of a cluster
  implies its `read` scope (`Scopes::close_implied`). The cluster's rows need no scope at all (`public = true`).
  Inputs and outputs above name contents, not flag spellings, except where a flag is quoted; the implementing bead
  fixes the spellings and the snapshot then pins them (additive changes keep the schema's major version; anything
  else raises it).

## References

Appendix J (J.1 entry form; J.2 inventory rows and J.3 added rows; J.5 task clusters), §32.12.4, §32.13, §32.4,
§32.6, §32.12.3, §32.18, §32.21; GATE-TETHER-02; INV-TETHER-01, -02; KA-755.
