---
key: "NJL-02"
title: "frob-conformance: verb registry conformance harness: surface parity, e2e coverage lint and help-schema snapshots"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "tether", "gauntlet", "frob-conformance", "frob-verbs", "solid", "test", "e2e", "gate"]
parent: "WP-06"
deps: ["BD-022", "BD-023", "NJL-01", "STD-E2E", "STD-TEST"]
needs: []
related: ["BD-137", "BD-138", "BD-222", "BD-298", "BD-312", "BD-376", "BD-409", "BD-004", "NJL-03", "NJL-05", "NJL-07"]
gates: ["GATE-TETHER-02"]
plan_refs: ["Appendix J (preamble, J.1, J.3)", "§32.2", "§32.5", "§32.12.3", "§32.12.4", "§32.19 (INV-TETHER-01, -02)", "§32.21 (GATE-TETHER-02)", "§40 (GATE-TETHER-02, INT-FMCP-01)"]
---
## Background

Every verb of FrankenRobotics is declared once in `registries/verbs.toml` (crate `frob-verbs`), and the
command line (`frob`), the MCP server (`frob-mcp`, also hosted inside the daemon as `frobd --mcp`), the schemas
and the agent guide are generated from it. Generation makes drift unlikely; it does not prove that the served
surfaces equal the registry. Gate **GATE-TETHER-02** (TARGET; claim Known answer; due at M1 and re-measured at
every later milestone for the verbs that land) demands the proof: the CLI and MCP surfaces are generated from the
registry and their parity diff is empty; every verb has a schema, an example and a refusal list; the served
`tools/list` equals the registry under the name map (`.` and `-` become `_`). The admission gate INT-FMCP-01 (M0)
needs a first form of the same measurement: `frob-mcp` lists the tools generated from the verb registry.

The registry holds 243 verbs in 44 clusters. Each cluster has a registry bead (`VERBS-<cluster>`) that carries
its rows; each of those beads must show three things about its rows: (A) the CLI commands and MCP tools generated
from the registry match the list; (B) every verb has an end-to-end script; (C) the schema printed by `frob
<verb> --help --json` is stable. This bead builds those three checks once, as a harness parameterized by
cluster, so that 44 beads do not write 44 diffs.

Design decisions and rejected alternatives (DESIGN):

- The harness is a tool of the verification crate `frob-conformance` (layer 7), not a `frob` verb and not an
  `xtask` subcommand. Reason: a check driven through the surface it judges can be made to pass by the surface
  (the program's rule against self-certification), and `xtask` holds exactly six checks and runs no binaries.
  Rejected: `frob capabilities --check-parity` (self-judging).
- It compares the **served** surfaces: it launches the real `frob` and the real `frob-mcp`, lists and calls them.
  Rejected: comparing generator outputs in memory (that tests the generator's intentions; ADR-TETHER-01 says
  the gate checks what is served).
- The parity exemptions are a closed list in the harness source, each with its reason. An exemption that is not
  on the list is a failure. Rejected: a tolerance file in the repository (it would grow).

Invariants this harness tests. INV-TETHER-01: every surface is generated from the registry; no verb, flag,
schema or refusal exists on one surface and not the other, except per-surface defaults declared in the registry
and the unknown-verb case. INV-TETHER-02: with `--json`, exactly one envelope per invocation; malformed input
and unknown verbs produce it; exit code 1 is never emitted on purpose.

## Scope

In: the tool `frob-surface-diff` with its three subcommands (`parity`, `e2e-coverage`, `snapshot`); the record
schemas `frob.verbs.parity/1`, `frob.verbs.coverage/1`; the shell library `scripts/e2e/verbs_lib.sh` with the
function every cluster script calls; the snapshot directory `fixtures/golden/verbs/` and its update rule; the
registry-wide script `scripts/e2e/verbs_registry.sh`. Out: the registry rows (the `VERBS-<cluster>` beads); the
entry schema and lints (NJL-01); the generation of CLI and MCP surfaces (BD-022, BD-023, BD-138, BD-298); the
recording of the gate's status (a fresh-context session, per the harness rules of STD-E2E).

## Technical approach

**Tool.** `crates/frob-conformance/src/bin/frob-surface-diff.rs` with modules `src/verbs/{parity,coverage,
snapshot,mcp_client}.rs`. It reads the registry file directly with its own strict TOML reader for the entry
schema (it does not link the generated table, so a generator defect cannot hide itself). It drives `frob` as a
subprocess and `frob-mcp` over stdio; with `--mcp-url http://127.0.0.1:<port>` it talks to `frobd --mcp` over
loopback HTTP with a bearer token from `FROB_SESSION_TOKEN`. MCP listing is done with every task cluster
visible: the tool filters by the tag `cluster:<name>` through `includeTags` and, where the server was built with
static clusters, is launched with `--clusters all`.

**Check A, `frob-surface-diff parity [--cluster <c>] [--milestone Mn]`.** Steps:
1. Load the registry rows R (restricted to the cluster if given).
2. CLI side: for each row, run `frob <cli path> --help --json`; it must exit 0 with one envelope whose
   `result.entry` reproduces the row. Collect C = the set of verbs `frob robot-docs --json` lists.
3. MCP side: `tools/list` (all pages); map each tool name back through the inverse name map. Collect M.
4. Set differences `R∖C`, `R∖M`, `C∖R`, `M∖R` must be empty, apart from the exemptions.
5. Field comparison per verb, registry against each surface: name under the map; description present and at most
   800 characters; input schema equal to the row's schema plus the common properties of its kind (`run`:
   `sync_wait_ms`, `idempotency_key`, `budget`, `estimate_only`, `on_cancel`; `mutate`: `idempotency_key`,
   `expected_parent`); output schema equal to `frob.envelope/1` with `result` specialized; tags `cluster:<c>`,
   `kind:<k>`, `task:<t>` per task cluster, `tier:<S|F|M>`; annotations (`read_only` for kind `read`;
   `idempotent` from the row; `destructive` only for an `admin` verb that deletes; `open_world_hint: false`);
   the refusal list (chapter codes plus the codes appended by kind); at least one example.
6. Behavioural spot checks per verb, on both surfaces: a call with a misspelt parameter is refused
   (`FROB-REF-802`, exit 3; on MCP a tool error whose structured content is the same envelope skeleton); a `run`
   or `mutate` call without an idempotency key is refused (`FROB-REF-805`, exit 3); a verb whose milestone has
   not come answers `FROB-REF-724`, exit 5.
7. Write the record and exit 0 only if every list in it is empty.

```json
{ "schema": "frob.verbs.parity/1", "cluster": "design", "registry": 12, "cli": 12, "mcp": 12,
  "missing_cli": [], "missing_mcp": [], "extra_cli": [], "extra_mcp": [],
  "field_diffs": [ { "verb": "design.check", "field": "refusals", "registry": ["…"], "cli": ["…"], "mcp": ["…"] } ],
  "behaviour_diffs": [], "exemptions_used": ["unknown-verb", "wait-default"] }
```

The closed list of exemptions:

| Exemption | What differs | Why it is allowed |
|---|---|---|
| `unknown-verb` | CLI: an envelope with `FROB-REF-801`, exit 3. MCP: a JSON-RPC error (−32602, "Unknown tool") from the library before any generated code runs | The library answers before a handler exists, so no envelope can be produced |
| `wait-default` | A `run` verb waits to completion on the CLI and 5,000 ms on MCP | Declared per surface in the row's `wait` field |
| `verb-invoke` | The MCP-only compatibility tool `verb_invoke {verb, arguments}` | For clients that never re-list tools; it is not a registry verb and must not appear in the registry |
| `session-focus-mcp` | The MCP handler of `session.focus` also edits the session's visible tool set | The one hand-written per-verb code path (ADR-TETHER-01) |

**Check B, `frob-surface-diff e2e-coverage [--milestone Mn]`.** A verb *has an end-to-end script* at two levels.
Surface level: some script under `scripts/e2e/` carries the header label `verb:<name>` and, in its newest log,
a `cmd` line whose `envelope.verb` equals the name (the cluster's `verbs_<cluster>.sh` satisfies this). Capability
level: a script owned by a bead other than a registry bead (a header `bead=` that is not `VERBS-…` or `NJL-…`)
carries the label and has a `cmd` line for the verb with `ok: true`, or an expected refusal. Rule: the surface
level is required from the milestone the row is registered; the capability level is required from the verb's
`first_milestone`. The record `frob.verbs.coverage/1` lists, per verb, both levels with the script names, and the
tool exits non-zero when a required level is missing. Input: the output of `frob-e2e list` (script headers) and
the logs under `target/e2e-logs/`.

**Check C, `frob-surface-diff snapshot --check | --update [--cluster <c>]`.** For each verb, `frob <cli path>
--help --json` returns `result = { entry, input_schema, output_schema, refusals[{code, meaning, exit, repairs}],
examples }`. The canonical JSON of that `result` (keys sorted by code point, no insignificant white space) is
the snapshot, stored at `fixtures/golden/verbs/<cluster>/<verb>.help.json`. `--check` fails on any byte
difference and classifies it:
- `additive`: a new optional output property, a new appended refusal code, a new example, a longer description.
  Allowed without a version change; the snapshot is updated in the same change by `--update`.
- `breaking`: a removed or renamed property, a changed type, a new required input, a removed refusal code, a
  changed kind or name. Allowed only when the major version in the schema identifier is raised (`…/1` → `…/2`);
  the old snapshot is kept beside the new one as `<verb>.help.v1.json` until the old version is retired.
A snapshot is never regenerated silently: `--update` refuses to write when the classification is `breaking`
and the major version is unchanged.

**Shell library `scripts/e2e/verbs_lib.sh`** (sourced after the shared helper `lib.sh`). One function,
`verbs_cluster_conformance <cluster>`, which every `scripts/e2e/verbs_<cluster>.sh` calls. It runs, as logged
steps: (1) `parity --cluster`, asserting each list of the record empty; (2) `snapshot --check --cluster`;
(3) for each verb of the cluster, its first registry example verbatim, expecting a result (exit 0 or 2), a job
handle (exit 8, then `frob job wait`), or `FROB-REF-724` exactly when `frob capabilities --json` says the verb
is absent; (4) for each implemented verb, the same call over MCP in process and through `frobd --mcp`, asserting
that the two envelopes agree on `verb`, `ok`, `exit`, `result`, `refusal`, `evidence`, `bound`, `warnings` and
`next_actions` (only `cost`, `budget` and job timing may differ); (5) the three behavioural refusals of step 6
above through `e2e_expect_refusal`; (6) exactly one envelope on standard output for every call, and no exit
code 1.

**Registry-wide script `scripts/e2e/verbs_registry.sh`.** Runs `parity`, `e2e-coverage` and `snapshot --check`
over the whole registry for a milestone and logs the three records as metrics against zero.

## Acceptance criteria

- [ ] `frob-surface-diff parity` exits 0 on the M0 registry (the M0 verbs of the `operator`, `session`, `sim`,
      `spark` and `prove` clusters) over stdio and over loopback HTTP, and its record shows `registry = cli =
      mcp` with empty difference lists (the INT-FMCP-01 measurement that the tools listed are the registry's).
- [ ] GATE-TETHER-02 (TARGET): at M1 and at each later milestone the parity record over all verbs due is empty
      apart from the four exemptions; every verb has a schema, an example and a refusal list; the served
      `tools/list` equals the registry under the name map. The measured counts are logged against the registry
      count.
- [ ] Negative controls, each caught: a tool registered by hand in `frob-mcp` outside the registry (`extra_mcp`);
      a registry row whose CLI command is suppressed (`missing_cli`); a row whose MCP description omits its
      refusal codes (`field_diffs`); a `run` tool served without `sync_wait_ms`; an exemption name not on the
      list.
- [ ] `e2e-coverage` reports a planted verb with no labelled script as missing at the surface level, and a verb
      past its first milestone with only the cluster script as missing at the capability level.
- [ ] `snapshot --check` passes twice in a row; a planted renamed output property is classified `breaking`
      and `--update` refuses it; a planted new optional property is classified `additive`.
- [ ] Snapshots are byte-identical across two runs, debug and release builds, Linux and macOS.
- [ ] `verbs_lib.sh` is used unchanged by the 44 cluster scripts; a cluster script is at most 15 lines.
- [ ] The tool's reader of the registry shares no code with the `frob-verbs` build script (a source scan).

## Test plan

### Unit tests

`cargo test -p frob-conformance --test verbs_harness` (real binaries where a binary is needed; fixture
registries labelled `synthetic`):
- `inverse_name_map`: `telemetry_inspect_pixel` resolves to `telemetry.inspect-pixel` only through the
  registered set; an unmapped tool name is reported as `extra_mcp`.
- `parity_detects_missing_and_extra`: four planted surfaces → the four difference lists, one entry each.
- `parity_field_diff_refusals`: a row with refusals {101, 104} served with {101} → one `field_diffs` entry
  naming `refusals`.
- `exemptions_closed`: a difference tagged with an unknown exemption fails; each of the four known ones passes
  only for its own case.
- `coverage_two_levels`: a header set with only `verbs_design.sh` labelled `verb:design.check` → surface level
  present, capability level missing.
- `snapshot_classify_additive`, `snapshot_classify_breaking`, `snapshot_update_refuses_breaking`.
- `canonical_json_stable`: key order and white space of a snapshot do not depend on the order of emission.
- `one_envelope_per_call`: a planted binary that prints two JSON objects fails the check (INV-TETHER-02).

### End-to-end script and logging

`scripts/e2e/verbs_registry.sh` (header: `bead=NJL-02 milestone=M0 labels=tether,verbs,registry
gates=GATE-TETHER-02 seed=1 needs=cpu,mcp,frobd plants=extra-tool,missing-cli,renamed-field`). It starts `frobd
--mcp` on a loopback port inside the scratch root, runs the three subcommands for `--milestone` of the build,
and repeats `parity` over stdio and over HTTP. Logged through the shared helper to
`target/e2e-logs/verbs_registry/<utc>.ndjson`: each command with exit code and wall time; metrics `registry_verbs`,
`cli_verbs`, `mcp_tools`, `missing_cli`, `missing_mcp`, `extra_cli`, `extra_mcp`, `field_diffs`,
`snapshot_diffs`, `verbs_without_script` each against its threshold (`== registry_verbs` or `== 0`) and tagged
`--gate GATE-TETHER-02`; on failure the first differing verb and field and the path of the record. With a plant
selected, the script must exit 1 with a failed assertion that names the plant. In small mode (`E2E_SMALL=1`) it
checks one cluster and the log says the gate is not claimable.

### Known answers and negative controls

- KA-755 (the exit-code table on malformed, unknown, over-budget, unscoped and conflicting inputs: each gives
  the exit and refusal code of the surface's tables; exact) is exercised per verb by the behavioural spot checks:
  misspelt parameter → 802 and exit 3; missing key → 805 and exit 3; unimplemented → 724 and exit 5; unknown
  verb on the CLI → 801 and exit 3.
- Known answer of the harness: on a registry of n rows with nothing planted, `registry = cli = mcp = n` and all
  lists are empty.
- Negative controls: the five planted surfaces of the acceptance criteria; the planted snapshot changes; the
  planted two-envelope binary.
- Metamorphic relation: restricting `parity` to each cluster in turn and summing the counts equals the
  registry-wide run.
- Determinism lanes: records and snapshots identical twice on one machine, across debug and release, and across
  Linux and macOS (class D0).

## Considerations

- OPEN-TETHER-2 (owner: Tether lead; blocks GATE-TETHER-12): whether the agent harnesses in use act on
  catalog-changed notifications. It does not affect this bead: the harness lists by tag or with all clusters
  enabled and never depends on a notification.
- If kill gate KILL-MCP fires and `frob-mcp` becomes its own workspace talking to `frobd` over a socket, the
  harness is unchanged (it drives served surfaces); if the second fallback fires (CLI only), the MCP half of
  each check is recorded `not-run` with the ADR cited, and GATE-TETHER-02 is measured on the CLI alone.
- The cost of a registry-wide run is 243 `--help` calls and one listing per surface: seconds. The registry-wide
  script calls `run` verbs with `estimate_only`, so it spends no compute; the cluster scripts run each registry
  example for real, which is why an example must name the smallest fixture that exercises the verb.
- Snapshots are golden files, so a careless `--update` could bless a regression. The classifier and the refusal
  to update a breaking change are the guard; a reviewer still reads the diff.
- The tool needs an MCP client; it uses `fastmcp-client`, the client crate of the MCP library the server is built on
  (`fastmcp_rust`), which is acceptable in the verification crate but means a library defect could hide on both sides. The stdio
  transcript is therefore also checked at the JSON-RPC level for the two framing rules that matter here (one
  response per request; `tools/list` pagination followed to the end).

## References

Appendix J (preamble, J.1, J.3), §32.2, §32.5, §32.12.3, §32.12.4, §32.19, §32.21, §40; GATE-TETHER-02;
INT-FMCP-01; INV-TETHER-01, -02; ADR-TETHER-01; KA-755; KILL-MCP; OPEN-TETHER-2.
