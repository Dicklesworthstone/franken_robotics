---
key: "NJL-02"
title: "frob-conformance: verb registry conformance harness: surface parity, e2e coverage lint, help-schema snapshots"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "tether", "gauntlet", "frob-conformance", "frob-verbs", "solid", "test", "e2e", "gate", "review-notes-open"]
parent: "WP-06"
deps: ["BD-022", "BD-023", "NJL-01", "STD-E2E", "STD-TEST"]
needs: []
related: ["NJL-04", "NJL-06", "NJL-07"]
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

The complete registry holds 244 verbs in 44 clusters (the 243 of the specification and `artifact.import` of
VERBS-artifact; 247 with the three late analysis verbs NJLx-51 registers at M3; the count of record is NJL-03's).
Rows enter with their cluster beads from M0 to M4, so the harness reads the rows from the registry of the build
and hard-codes no count. Each cluster has a registry bead (`VERBS-<cluster>`) that carries
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
- The differences the parity check may accept are a closed list in the harness source, each with its reason:
  one exemption, the unknown-verb case (the plan's rule: the parity gate lists it as the only exemption), and
  three declared differences that the registry or the surface design declares in advance. A difference that
  is not on the list is a failure. Rejected: a tolerance file in the repository (it would grow).
- One parity implementation serves the registry and the gate. The surface chapter's unit had drafted a second
  harness for the same gate (N32a-11); it is now the gate-grade extension of this tool's `parity` check, from
  M1, and depends on this bead. This bead owns the tool, the record schema, the list of accepted differences
  and the reader of the registry; N32a-11 adds checks and the per-milestone gate report and changes none of
  those.

Invariants this harness tests. INV-TETHER-01: every surface is generated from the registry; no verb, flag,
schema or refusal exists on one surface and not the other, except per-surface defaults declared in the registry
and the unknown-verb case. INV-TETHER-02: with `--json`, exactly one envelope per invocation; malformed input
and unknown verbs produce it; exit code 1 is never emitted on purpose.

## Scope

In: the tool `frob-surface-diff` with its three subcommands (`parity`, `e2e-coverage`, `snapshot`); the record
schemas `frob.verbs.parity/1`, `frob.verbs.coverage/1`; the shell library `scripts/e2e/verbs_lib.sh` with the
function every cluster script calls; the example identities file `fixtures/verbs/identities.toml` and its
substitution rule; the snapshot directory `fixtures/golden/verbs/` and its update rule; the registry-wide script `scripts/e2e/verbs_registry.sh`. Out: the registry rows (the `VERBS-<cluster>` beads); the
entry schema and lints (NJL-01); the generation of CLI and MCP surfaces (BD-022, BD-023, BD-138, BD-298); the
recording of the gate's status (a fresh-context session, per the harness rules of STD-E2E); the gate-grade
additions to check A that GATE-TETHER-02 needs from M1 and the gate's script and report (N32a-11, which
depends on this bead): the probe of the flags the CLI parser really accepts, the cross-check of the schemas
served by `frob schema`, the detection of an undeclared per-surface default, behavioural parity on each verb's
first example, the in-memory MCP transport, the six negative controls registered for the gate, the script
`scripts/e2e/tether_parity.sh --through <milestone>` that the milestone verb-set beads run, and its report
`target/parity/<milestone>.json`.

## Technical approach

**Tool.** `crates/frob-conformance/src/bin/frob-surface-diff.rs` with modules `src/verbs/{parity,coverage,
snapshot,expect,mcp_client}.rs`. It is an operator binary of the verification crate under BD-025's rule: declared
in the crate's manifest as a `[[bin]]` entry with `name = "frob-surface-diff"` and `path =
"src/bin/frob-surface-diff.rs"` (the manifest sets `autobins = false`, so a binary exists only by declaration),
with a row in the binary table of `crates/frob-conformance/CONTRACT.md`: owning bead NJL-02; reads
`registries/verbs.toml`, the served surfaces of `frob`, `frob-mcp` and `frobd --mcp`, the script headers and the
logs under `target/e2e-logs/`, and the snapshots; writes the records `frob.verbs.parity/1`,
`frob.verbs.coverage/1` and (NJL-03's subcommand) `frob.verbs.inventory/1`, and with `snapshot --update` the
files under `fixtures/golden/verbs/`; it runs no tier and writes no gate row. It reads the registry file
directly with its own strict TOML reader for the entry schema (it does not link the generated table, so a
generator defect cannot hide itself). It drives `frob` as a
subprocess and `frob-mcp` over stdio; with `--mcp-url http://127.0.0.1:<port>` it talks to the server at that
loopback address over HTTP: `frob-mcp`'s own HTTP transport (BD-023, from M0) or, from the build that has the
daemon (N32a-14, M2), `frobd --mcp`, with a bearer token from `FROB_SESSION_TOKEN` where the server asks for one
(N32a-27, M3). MCP listing is done with every task cluster
visible: the tool filters by the tag `cluster:<name>` through `includeTags` and, where the server was built with
static clusters, is launched with `--clusters all`.

**Check A, `frob-surface-diff parity [--cluster <c>] [--milestone Mn]`.** Steps:
1. Load the registry rows R (restricted to the cluster if given).
2. CLI side: for each row, run `frob <cli path> --help --json`; it must exit 0 with one envelope whose
   `result.entry` reproduces the row. Collect C = the set of verbs `frob robot-docs --json` lists.
3. MCP side: `tools/list` (all pages); map each tool name back through the inverse name map. Collect M.
4. Set differences `R∖C`, `R∖M`, `C∖R`, `M∖R` must be empty, apart from the exemptions.
5. Field comparison per verb, registry against each surface: name under the map; description present and at most
   800 characters; input schema equal to the row's schema plus the common properties NJL-01 derives for the row,
   in NJL-01's fixed order: those of its kind (`run`: `sync_wait_ms`, `idempotency_key`, `budget`,
   `estimate_only`, `on_cancel`, `envelope_scope`; `mutate`: `idempotency_key`, `expected_parent`) and those of
   every arm of NJL-01's variation table that applies to the row. The table has sixteen arms (pagination,
   `estimable`, `key_optional`, an `int53` input, `actions`, `needs_groups`, the three `params` arms, the loop
   table's `override` and `override_reason`, `from_proposal`, `machine`, `engine`, `memo`'s `no_memo`, a report
   surface or artifact reader, and the `[[admission_check]]` rows' `admission_checks` and
   `diagnostic_override`); an arm applies only once the bead that lands it has added its field or table, so a
   registry without the field expects nothing from it.
   `idempotency_key` is never in the schema's `required` array and carries `x-frob-key: "required"`, or
   `"optional"` on a `key_optional` entry; for a verb with actions, the `action` property and the flat union of
   the properties of each action's kind; output schema equal to `frob.envelope/1` with `result` specialized;
   tags `cluster:<c>`, `kind:<k>` once for each distinct kind among the default action and the actions,
   `task:<t>` per task cluster, `tier:<S|F|M>`; annotations (NJL-01's rule: `read_only` only when every action,
   the default included, is `read`; `idempotent` the row's field, and false when any action is `run` or
   `mutate`; `destructive` for an `admin` entry with `destructive = true` and for an entry with an `admin` action
   whose `destructive` member is true, its default; `open_world_hint: false`); the refusal list (chapter
   codes plus the codes NJL-01 appends by kind and by each applying arm: 806 and 807 for pagination; 808 exactly
   when the generated input schema has an `int53` property at any depth, which every `run` entry has through
   `sync_wait_ms`; 790 for `needs_groups`; 891 to 893, 734 and 828 for `params`; 804 for `from_proposal`; the
   codes of the `machine` and `engine` arms by their owners' constants; 814, 816, 819, 873, 889, 895 and 897 for
   a report surface or an artifact reader; each guarding admission check's `refusals`; never 990); at least one
   example. DESIGN: the harness computes this expectation from the row with its own table-driven implementation
   of NJL-01's two tables (`src/verbs/expect.rs`), which shares no code with the generator's `derive.rs`, for the
   reason its reader shares none with the build script (a defect in the derivation must not hide itself); the
   unit test `expectation_equals_derivation` then requires its result to equal
   `VerbEntry::common_properties()` and `all_refusals()` on every committed row, so a rule added to one
   implementation and not to the other fails by name. Rejected: a hand list of some variations in this bead (an earlier text named
   three and went stale as NJL-01's table grew); calling `common_properties()` as the expectation (the surface
   and its judge would then run the same code).
6. Behavioural spot checks per verb, on both surfaces: a call with a misspelt parameter is refused
   (`FROB-REF-802`, exit 3; on MCP a tool error whose structured content is the same envelope skeleton); a `run`
   or `mutate` call without an idempotency key is refused (`FROB-REF-805`, exit 3), except on a `key_optional`
   entry (`session.focus`, `study.plan`), where the same call succeeds; a verb whose handler is not in the build
   (`frob capabilities --json` lists it absent) answers `FROB-REF-724`, exit 5. Whether a verb answers is decided
   by its handler being in the build, never by comparing the build's milestone with `first_milestone` (NJL-03's
   rule: a reduced form that lands early answers before the row's milestone, with its restriction named, for
   example `form: "cpu"`). The missing-key check needs the admission transaction of N32a-09 (M1), which is what
   raises 805: in an M0 build a missing key is not refused (BD-022), so there the check is logged as a skip
   naming N32a-09, never as a pass, and from M1 it is required.
7. Write the record and exit 0 only if every list in it is empty.

```json
{ "schema": "frob.verbs.parity/1", "cluster": "design", "milestone": "M1", "registry": 12, "cli": 12, "mcp": 12,
  "registry_digest": "…", "transports": ["stdio", "http"], "configuration": "in-process",
  "missing_cli": [], "missing_mcp": [], "extra_cli": [], "extra_mcp": [],
  "field_diffs": [ { "verb": "design.check", "field": "refusals", "registry": ["…"], "cli": ["…"], "mcp": ["…"] } ],
  "behaviour_diffs": [], "exemptions_used": ["unknown-verb"], "declared_differences_used": ["wait-default"] }
```

This is the one record schema of surface parity. The gate report of N32a-11 (`target/parity/<milestone>.json`)
is this record for `--milestone Mn` over the whole registry, with the lists its added checks fill
(`flag_diffs`, `default_diffs`, `example_diffs`); the three fields `registry_digest`, `transports` and
`configuration` (`in-process`, or `socket-fallback` if KILL-MCP fired) are written by every run.

The closed list of accepted differences. The parity gate has exactly one exemption (DESIGN reading of the
plan's sentence "the parity gate lists it as the only exemption"; an earlier draft of this bead called all
four rows exemptions, and the gate bead N32a-11 asserts that the exemption list has one entry). The other
three rows are declared differences: each is announced by the registry or by the surface's design before any
surface is served, which is what invariant INV-TETHER-01 allows ("except per-surface defaults declared in the
registry and the unknown-verb case").

| Name | Class | What differs | Why it is allowed |
|---|---|---|---|
| `unknown-verb` | exemption | CLI: an envelope with `FROB-REF-801`, exit 3. MCP: a JSON-RPC error (−32602, "Unknown tool") from the library before any generated code runs | The library answers before a handler exists, so no envelope can be produced |
| `wait-default` | declared difference | A `run` verb waits to completion on the CLI and 5,000 ms on MCP | Declared per surface in the row's `wait` field |
| `verb-invoke` | declared difference | The MCP-only compatibility tool `verb_invoke {verb, arguments}` | For clients that never re-list tools; it is not a registry verb and must not appear in the registry. It is the only tool the served list may hold that maps to no registry verb |
| `session-focus-mcp` | declared difference | The MCP handler of `session.focus` also edits the session's visible tool set | The one hand-written per-verb code path (ADR-TETHER-01) |

The three base tools `session_focus`, `job_wait` and `job_cancel` are registry verbs and are compared like
any other; they are listed whatever the focus, so in a listing focused on one task cluster the only names
beyond the cluster are those three and `verb_invoke` (four names; a fifth is `extra_mcp`).

**Check B, `frob-surface-diff e2e-coverage [--milestone Mn]`.** A verb *has an end-to-end script* at two levels.
Surface level: some script under `scripts/e2e/` carries the header label `verb:<name>` and, in its newest log,
a `cmd` line whose `envelope.verb` equals the name (the cluster's `verbs_<cluster>.sh` satisfies this). Capability
level: a script owned by a bead other than a registry bead (a header `bead=` that is not `VERBS-…` or `NJL-…`)
carries the label and has a `cmd` line for the verb with `ok: true`, or an expected refusal. Rule: the surface
level is required from the milestone the row is registered; the capability level is required from the verb's
`first_milestone`, and earlier from the first build whose capability report lists the verb as implemented (a
reduced form that lands early).

*Declared late rows (DESIGN).* The registry keeps the appendix's `first_milestone` even when the bead that
implements a verb lands later (BD-137's rule), so such a row is past its milestone with no handler for a while,
and the rule above alone would fail GATE-TETHER-02 for it, while accepting every `FROB-REF-724` would let a verb
whose handler was simply never written pass. Rejected: a registry field naming the handler's milestone (it
would be a second copy of the delivery plan in the product's registry). A second closed list in the harness source, `LATE_ROWS`,
holds each such row with its implementing bead, that bead's milestone (`lands`) and the reason. At the time of
writing, read from the cluster beads' "Implemented by" lines (thirteen rows): `session.open`, `session.show`,
`session.focus`, `session.close` (M0; BD-138, M1); `ground.ingest` (M1; BD-387, M5); `study.cancel` (M2;
N32a-13, M3); `scene.retrofit` (M3; BD-369 and N19b-40, M4); `shield.budget`
(M3; BD-337, M4); `shield.check` (M3; BD-338, M4); `analyze.structure` and `foundry.size` (M4; BD-401, M5);
`foundry.loads` (M4; BD-400, M5); `ground.excite` (M4; BD-390, M5). (`contact.laws`, `contact.probe` and
`task.settle`, which an earlier list named, have M1 handlers in N18b-28, N18b-29 and N24a-16 and are not late.
`scene.import`, which an earlier list also named for BD-398's `place` form at M5, is not late either: N19a-46
ships its `asset` form at M3, so from the row's own milestone the build lists it implemented with its form
named and the ordinary rule applies; the entry could never take effect and would have failed the self-check as
stale at M5. That `--kind place` answers `FROB-REF-724` until BD-398 is an expected refusal of one option value,
asserted by the cluster script of VERBS-scene. The list is about rows, not about forms of a row.)
An omission is not silent: the row then fails the capability level by name, and is resolved by its handler or
by a reviewed addition with its reason. For a late row whose build lists it absent, from `first_milestone` to the
milestone before `lands`, the capability level is satisfied by the cluster script's `cmd` line answering
`FROB-REF-724`, exit 5 (the expected refusal), and the record marks the level `late-724` with the bead; from
`lands` the ordinary rule applies, so a late row still without a handler at its bead's milestone is missing. A
late row that the build lists implemented is checked by the ordinary rule. The list is closed like the
exemptions: an entry whose verb is not in the registry, whose `lands` is not later than the row's
`first_milestone`, or whose verb has been listed implemented in a build at or past `lands` (a stale entry) fails
the tool's self-check, so the list only shrinks unless a reviewed change adds a row with its reason.

The record `frob.verbs.coverage/1` lists, per verb, both levels with the script names (and `late-724` with the
implementing bead where it applies), and the tool exits non-zero when a required level is missing. Input: the
output of `frob-e2e list` (script headers) and the logs under `target/e2e-logs/`.

**Check C, `frob-surface-diff snapshot --check | --update [--cluster <c>]`.** For each verb, `frob <cli path>
--help --json` returns `result = { entry, input_schema, output_schema, refusals[{code, meaning, exit, repairs}],
examples }`. The canonical JSON of that `result` (keys sorted by code point, no insignificant white space) is
the snapshot, stored at `fixtures/golden/verbs/<cluster>/<verb>.help.json`. `--check` fails on any byte
difference and classifies it:
- `additive`: a new optional output property, a new optional input property (as each of NJL-01's later
  variations adds to many verbs at once: `envelope_scope`, `no_memo`, `diagnostic_override`, `override` and
  `override_reason`, `from_proposal`, `machine`, `engine`; and as an option a cluster bead lists with a later milestone), a new appended refusal code, a new
  example, a longer description. Allowed without a version change; the snapshot is updated in the same change by
  `--update`.
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
is absent; (4) for each implemented verb, the same call over MCP in process (`frob-mcp` over stdio) and, from the
build that has the daemon (N32a-14, M2), through `frobd --mcp` (before it, a logged skip naming N32a-14),
asserting that the CLI and MCP envelopes agree on `verb`, `ok`, `exit`, `result`, `refusal`, `evidence`, `bound`, `warnings` and
`next_actions` (only `cost`, `budget` and job timing may differ); (5) the three behavioural refusals of step 6
above through `e2e_expect_refusal` (the missing-key one through `e2e_skip` naming N32a-09 in an M0 build); (6) exactly one envelope on standard output for every call, and no exit
code 1.

**Example identities (DESIGN).** A registry example names its objects as the plan prints them: a fixture by
name (`quad-a`, `home-fixture`) or a stored object by an abbreviated identity (`r:19e2`, `b3:61f0`, `d:7a3e`,
`j:77c0`; most are the Appendix L abbreviations). About sixty examples across 26 clusters do so. An
abbreviation does not satisfy an input schema (an identity is a full digest), so the examples cannot be run
as written, and step (3) would see `FROB-REF-802` where it expects a result. Rule: "verbatim" means verbatim
after one substitution, and nothing else of the line changes. The function reads
`fixtures/verbs/identities.toml` (one row per abbreviation: the full identity, the object's kind, the fixture
or script that stores it in the fixture store the cluster scripts run against, and the milestone from which it
is there), replaces each abbreviation of the example by its full identity, and logs both lines (`example` and
`resolved`). A `j:` abbreviation names no stored object: it resolves to a fresh job that the function submits
first (the first implemented `run` example of the registry, with `sync_wait_ms = 0`). An abbreviation with no
row, or whose row's milestone is later than the build's, is replaced, for a verb the capability report lists
as absent, by a well-formed identity of its kind that names no object (its prefix and an all-zero digest), so
that the call passes the schema and reaches the expected `FROB-REF-724`; for an implemented verb it fails step
(3) as `example-unresolved`, naming the abbreviation and the verb. A row is added by the bead that makes the
example run (the implementing bead, or the milestone verb-set bead BD-137 and its successors). From M1, when
NJL-07 lands, a row whose abbreviation NJL-07's `fixtures/golden/appendix-l/IDENTITIES.toml` also lists must
carry the same full identity (`example_identities_agree`), so one abbreviation names one object everywhere.
Rejected: full digests in the registry (unreadable in the agent guide, and unknown until the fixture is
blessed); prefix lookup in the CLI (a product feature the plan does not have, and ambiguous as a store grows).
N32a-11's behavioural parity on each verb's first example and BD-137's `examples_run` use the same file.
Examples that name a file (`patch.json`, `arm.urdf`, `tray-stance.fmission`, `regime.toml`) are run with
`fixtures/verbs/<cluster>/` as the working directory, which holds every file an example of the cluster names; a
missing file is a failure of step (3) naming the file, not a refusal to be accepted.

**Registry-wide script `scripts/e2e/verbs_registry.sh`.** Runs `parity`, `e2e-coverage` and `snapshot --check`
over the whole registry for a milestone and logs the three records as metrics against zero. It also runs, as one
logged stage, stages 1 to 3, 5 and 6 of NJL-01's script `scripts/e2e/verbs_registry_schema.sh` against the
build's binaries, the stages NJL-01 assigns to this bead because they need `frob`: the entry schema with its
example, the capability listing, the common properties and the full refusal list of one registered verb of each
kind, the `FROB-REF-724` answer of a verb absent from the build, and the two `FROB-REF-802` bounds
(`--sync-wait-ms 30001`, `--on-cancel leave_running`) on the test-lane binary built with `synthetic-verbs`. A
stage whose prerequisite is absent is a logged skip naming it (a kind with no registered verb, such as `admin`
at M0); a failed stage fails this script. Stage 4 is BD-023's and stage 7 BD-138's.

## Acceptance criteria

- [ ] `frob-surface-diff parity` exits 0 on the M0 registry (the M0 verbs of the `operator`, `session`, `sim`,
      `spark` and `prove` clusters) over stdio and over loopback HTTP, and its record shows `registry = cli =
      mcp` with empty difference lists (the INT-FMCP-01 measurement that the tools listed are the registry's).
- [ ] GATE-TETHER-02 (TARGET): at M1 and at each later milestone the parity record over all verbs due is empty
      apart from the one exemption and the three declared differences; every verb has a schema, an example
      and a refusal list; the served `tools/list` equals the registry under the name map. The measured counts
      are logged against the registry count. The gate's own script, report and added checks from M1 are
      N32a-11's, run on this tool; this bead's part of the gate is that the tool's record is the one they use.
- [ ] Negative controls, each caught: a tool registered by hand in `frob-mcp` outside the registry (`extra_mcp`);
      a registry row whose CLI command is suppressed (`missing_cli`); a row whose MCP description omits its
      refusal codes (`field_diffs`); a `run` tool served without `sync_wait_ms`; an exemption name not on the
      list.
- [ ] `e2e-coverage` reports a planted verb with no labelled script as missing at the surface level, and a verb
      past its first milestone with only the cluster script as missing at the capability level.
- [ ] On the M0 registry the four `session` verbs, and on the M1 registry `ground.ingest` (declared late rows,
      absent from the build), are reported `late-724` with their implementing beads and do not fail the check;
      the same rows without a handler in a build at their bead's milestone are reported missing; a planted
      `LATE_ROWS` entry for a verb not in the registry fails the self-check.
- [ ] An example naming `r:19e2` is run with the full identity of its row, and the log carries both lines; for
      an absent verb an abbreviation with no row still yields `FROB-REF-724`, exit 5; for an implemented verb it
      fails step (3) as `example-unresolved` naming the abbreviation; a `j:` abbreviation runs against a job the
      function submitted.
- [ ] `snapshot --check` passes twice in a row; a planted renamed output property is classified `breaking`
      and `--update` refuses it; a planted new optional property is classified `additive`.
- [ ] Snapshots are byte-identical across two runs, debug and release builds, Linux and macOS.
- [ ] `verbs_lib.sh` is used unchanged by the 44 cluster scripts; a cluster script is at most 15 lines.
- [ ] The tool's reader of the registry shares no code with the `frob-verbs` build script, and `expect.rs` none
      with `derive.rs` (a source scan); `expectation_equals_derivation` passes on the committed registry.
- [ ] `frob-surface-diff` is a declared `[[bin]]` with explicit `name` and `path` and has its row in the binary
      table of `CONTRACT.md`; BD-025's test `binaries_declared` passes with it.
- [ ] `scripts/e2e/verbs_registry.sh` runs stages 1 to 3, 5 and 6 of `verbs_registry_schema.sh` green on the M0
      binaries, with the `admin` kind logged as a skip at M0; NJL-01's plant `omit-805` is caught by stage 3,
      and its plant `unimplemented-as-unknown` by stage 5 where the dispatch mutant of N00-02 is built (a logged
      skip naming N00-02 otherwise, since this bead does not depend on it).
- [ ] In an M0 build the missing-key spot check is logged as a skip naming N32a-09 and counted apart from the
      passes; in an M1 build the same call is refused with `FROB-REF-805`, exit 3.

## Test plan

### Unit tests

`cargo test -p frob-conformance --test verbs_harness` (real binaries where a binary is needed; fixture
registries labelled `synthetic`):
- `inverse_name_map`: `telemetry_inspect_pixel` resolves to `telemetry.inspect-pixel` only through the
  registered set; an unmapped tool name is reported as `extra_mcp`.
- `parity_detects_missing_and_extra`: four planted surfaces → the four difference lists, one entry each.
- `parity_field_diff_refusals`: a row with refusals {101, 104} served with {101} → one `field_diffs` entry
  naming `refusals`.
- `exemptions_closed`: a difference tagged with an unknown name fails; each of the four known names (one
  exemption, three declared differences) passes only for its own case; `exemptions_used` can hold nothing
  but `unknown-verb`.
- `coverage_two_levels`: a header set with only `verbs_design.sh` labelled `verb:design.check` → surface level
  present, capability level missing.
- `coverage_early_form`: a `synthetic` row with `first_milestone = "M4"` whose capability report says
  implemented at M3 needs the capability level at M3; with only the cluster script it is reported missing.
- `coverage_late_row`: a `synthetic` row with `first_milestone = "M1"`, absent from the build and declared late
  with `lands = "M2"`: in an M1 build whose cluster script logged `FROB-REF-724`, exit 5, the capability level is
  `late-724`; in an M2 build in the same state it is missing; the same row not declared late is missing at M1.
- `late_rows_closed`: a `LATE_ROWS` entry whose verb is unregistered, or whose `lands` equals its
  `first_milestone`, fails the self-check; the thirteen entries of the shipped list pass it against the
  committed registry; a planted entry for `scene.import` (listed implemented from M3 with its `asset` form)
  fails as stale in a build at its `lands`.
- `expectation_equals_derivation`: for every committed row, and for the `synthetic` matrix of NJL-01's
  `prop_derivation_closed`, the properties (names, order, CLI flags, `x-frob-key`) and the full refusal list
  that `expect.rs` computes equal `VerbEntry::common_properties()` and `all_refusals()`; a `synthetic` row with
  an `int53` input nested in an object expects 808 and one with strings only does not; `idempotency_key` in a
  served `required` array is a `field_diffs` entry.
- `missing_key_skipped_before_admission`: with `--milestone M0` the 805 spot check is recorded `skipped:
  N32a-09` and is not in `behaviour_diffs`; with `--milestone M1` a keyless `design.edit` that succeeds is one.
- `spot_checks_follow_fields`: the spot check expects 805 for `design.edit` without a key and success for
  `session.focus` and `study.plan` without one; a served schema of a paginated `read` without `follow` is a
  `field_diffs` entry; a served `scene.bank` with `read_only: true` is a `field_diffs` entry.
- `answers_by_handler_presence`: an implemented-early verb that answers before its `first_milestone` is not a
  `behaviour_diffs` entry; a verb listed implemented that answers 724 is one.
- `example_identities_substituted`: a `synthetic` example `frob run show r:19e2 --json` with a row for `r:19e2`
  becomes the same line with the full identity and no other byte changed; an abbreviation occurring twice is
  replaced twice; a fixture name (`quad-a`) is left alone.
- `example_unresolved`: with no row, an absent verb's call carries the all-zero identity of its kind and an
  implemented verb's is `example-unresolved`; a row whose milestone is later than the build's counts as no row.
- `example_identities_agree`: a planted row giving `b3:61f0` a full identity other than the one in NJL-07's
  `IDENTITIES.toml` fails (skipped, and logged as skipped, before that file exists).
- `snapshot_classify_additive` (a new optional output property; a new optional input property such as
  `from_proposal` added to every `run` verb), `snapshot_classify_breaking` (a renamed property; a new required
  input), `snapshot_update_refuses_breaking`.
- `canonical_json_stable`: key order and white space of a snapshot do not depend on the order of emission.
- `one_envelope_per_call`: a planted binary that prints two JSON objects fails the check (INV-TETHER-02).

### End-to-end script and logging

`scripts/e2e/verbs_registry.sh` (header: `bead=NJL-02 milestone=M0 labels=tether,verbs,registry
gates=GATE-TETHER-02 seed=1 needs=cpu,mcp plants=extra-tool,missing-cli,renamed-field`). It starts `frob-mcp`
on a loopback HTTP port inside the scratch root (BD-023's transport; from a build that has the daemon, N32a-14,
also `frobd --mcp`, whose parity run is then logged as a third transport line), runs the three subcommands for `--milestone` of the build,
repeats `parity` over stdio and over HTTP, and runs the five stages of `verbs_registry_schema.sh` named above
(each logged with its stage number, the command, the exit code and, for a skip, its reason). Logged through the shared helper to
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
- The cost of a registry-wide run is one `--help` call per row (244 when the registry is complete, then 247)
  and one listing per surface: seconds. The registry-wide
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
