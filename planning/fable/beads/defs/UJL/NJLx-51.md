---
key: "NJLx-51"
title: "frob-verbs registry: three late analysis verbs (sweep-monotone, constraint-graph, regressor-identity); 247 rows"
type: "task"
priority: 2
milestone: "M3"
labels: ["m3", "tether", "augur", "frob-verbs", "solid", "registry", "verb", "test"]
parent: "WP-26"
deps: ["BD-022", "BD-139", "N32a-14", "NJL-01", "NJL-02", "NJL-03", "NJL-04", "NJL-05", "STD-E2E", "VERBS-analyze"]
needs: []
related: []
gates: ["GATE-TETHER-02"]
plan_refs: ["Appendix J (J.1, J.3, J.4, J.5)", "§32.6", "§32.12.4", "§0.2", "§25.28", "§25.30", "§25.33", "§28.4"]
---
## Background

FrankenRobotics (a robotics simulation and design system) is reached through the command line `frob`, the daemon
`frobd` and the MCP server `frob-mcp` (the Model Context Protocol, by which an agent harness lists and calls
tools). Every capability is a verb, `cluster.verb`, declared once in `registries/verbs.toml` (crate `frob-verbs`,
tier Solid). The CLI, the MCP tools, the schemas, the refusal table and the agent guide are generated from that
file, and gate GATE-TETHER-02 compares the served surfaces with it. The registry is **closed**: it holds the 243
verbs the specification lists and the rows recorded as additions (before this bead one, `artifact.import`,
registered by VERBS-artifact at M1: 244 in all, the count of record being NJL-03's), and count tests fail when a
row is added or removed without the counts being changed in the same commit.

Three mechanisms that the mission subsystem (Augur) gained in late subsections need a verb and have none in the
specification's verb list. Their implementing beads call them by `analyze` names:

- the detector for non-monotone sweep verdicts (a swept scalar whose verdict flips more than once is a warning
  about the model, recorded and routed to the fidelity ledger): `analyze.sweep-monotone`;
- the measurement behind open question OPEN-AUGUR-3 (how sparse the constraint-to-parameter graph of the robot
  classes is, and how strongly each parameter enters each lowered static constraint): `analyze.constraint-graph`;
- the identity test of the inertial-parameter regressor (inverse dynamics is linear in the inertial parameters π,
  ID(q, q̇, q̈; π) = Y(q, q̇, q̈)·π, and the emitted Y must reproduce inverse dynamics): `analyze.regressor-identity`.

A capability counts as implemented only if it is reachable through `frob --json` with a documented schema and
exit code. Without a row each of these would answer `FROB-REF-801` (unknown verb) on the CLI, would not exist over
MCP, and the end-to-end scripts that call them could not run. DESIGN decision: register the three as `read` verbs
of the cluster `analyze`, under the names their beads use. Reasons: the specification puts "one verb per
analysis … and each later analysis" in that cluster; all three are static, read-only computations that return
their own artifact; and the names are already written into three beads' acceptance criteria. Rejected: an option
on an existing verb (`mission.check --constraint-graph` and the like: one verb would then have two output
schemas, and the help-schema snapshot could not pin either); test-only binaries outside the registry (the
mechanisms would be unreachable for agents, and the reachability condition would fail); numbering them A-27 to
A-29 (the numbered analyses are the list that `design.check` draws from; none of the three analyses a design
alone, and numbering belongs to the owner of that list).

This bead is the registry of record for the three rows, moves the count known answers from 244 to 247, seats the
three tools in task clusters, and writes down the rule for any later addition, so that a verb name can no longer
appear in a bead without a row.

Kinds: `read` returns stored or cheaply computed facts and changes nothing. Tiers: S Solid, F Frontier (with a
pilot and a kill gate). A task cluster is the set of at most seven MCP tools an agent sees at a time.

## Scope

In: the three registry entries; their MCP tool names; their task-cluster seats; the new values of every count
known answer that the additions move; the additions record and its lints; the cluster conformance script for the
three; help-schema snapshots; each verb's answer before its milestone. Out: behaviour (N25e-08 the detector,
N25e-27 the graph measurement with the artifact of N25b-08, N25e-19 the regressor, N25e-21 its end-to-end lane);
the entry schema (NJL-01); the conformance harness (NJL-02); the other 26 `analyze` rows (VERBS-analyze); the
45-cluster assignment as a whole (NJL-05); the known-answer registry rows that restate the counts (rows JL-01 to
JL-05 of NJLx-02, whose expected values this bead moves).

## Technical approach

**The rows.**

| Verb | Kind | First milestone | Tier | Purpose | Defined in |
|---|---|---|---|---|---|
| `analyze.sweep-monotone` | read | M3 | S | Classify the verdict sequence of a recorded scalar sweep as `constant`, `single-threshold` or `non-monotone`, and return the record of a non-monotone one | §25.30 |
| `analyze.constraint-graph` | read | M3 | F | The constraint-to-parameter graph of a mission family: incidence, components, and the strength of each parameter in each lowered static constraint | §25.28 (OPEN-AUGUR-3), §25.6 |
| `analyze.regressor-identity` | read | M4 | F | Check the emitted regressor against inverse dynamics on sampled states of a reference robot and report the largest relative error | §25.33 |

**Contract of each verb.**

**`analyze.sweep-monotone`** (MCP tool `analyze_sweep_monotone`; owner `frob-analyze`)
- Inputs: the identifier of a stored result that holds a swept scalar (an answer of `mission.synthesize`,
  `nemesis.search` or `phylum.sweep`).
- Outputs: the classification; for `single-threshold` the bracket of the change; unresolved points as gaps; for
  `non-monotone` the record `frob.sweep.nonmonotone/1` (parameter and unit, grid, verdict sequence, indices of
  the changes, affected interval, physics step, requirement clock, contact parameters, tier, fixture, seed,
  replay addresses of the points beside each change). Evidence: the claim product of the sweep's inputs;
  determinism exact; no claim of its own. Exit 0 for `constant` and `single-threshold`, 2 for `non-monotone`.
- Chapter refusals: none of its own. Cost: linear in the number of points; no physics.
- Implemented by: N25e-08.

**`analyze.constraint-graph`** (MCP tool `analyze_constraint_graph`; owner `frob-mission`)
- Inputs: a mission family (a `.fmission` file or a stored query), as an artifact input read by the format's owner.
- Outputs: the artifact `frob.constraint-graph/1`: for each lowered constraint g_k the parameter set S_k and
  |S_k|; the connected components of the bipartite graph; on the root box and 64 sub-boxes the affine-form
  coefficients |aᵢ| per parameter, the remainder r, the ranked order and the effective split size s_k; the
  summary shares (|S_k| ≤ 6; |S_k| > 6 with s_k ≤ 6; s_k > 6). Evidence: incidence is exact; strengths are class
  `Tested` as a description of the stated boxes; nothing returned is a certificate. Exit 0.
- Chapter refusals: `FROB-REF-868` (the `.fmission` file is not valid), raised by the mission reader.
- Implemented by: N25e-27, on the artifact of N25b-08.

**`analyze.regressor-identity`** (MCP tool `analyze_regressor_identity`; owner `frob-synth`)
- Inputs: `--robot <reference robot>`, `--states <n>` (an `int53` count, default 10,000), `--seed` (a `u64str`).
- Outputs: per robot the largest relative error of Y·π against inverse dynamics over the sampled states, the
  state that attains it, and the threshold 10⁻¹² (TARGET) it is compared with. Evidence: a Known answer, class
  Tested. Exit 0 when the error is within the threshold, 2 otherwise.
- Chapter refusals: none of its own. Implemented by: N25e-19; exercised end to end by N25e-21.

**Machine fields.**

| Verb | Capabilities | Output schema | Task clusters | Example |
|---|---|---|---|---|
| `analyze.sweep-monotone` | `analyze:read` | `frob.analyze.sweep-monotone.out/1` | `analyze-evidence` | `frob analyze sweep-monotone a:8b27 --json` |
| `analyze.constraint-graph` | `analyze:read` | `frob.analyze.constraint-graph.out/1` | `audit` | `frob analyze constraint-graph quad-12.fmission --json` |
| `analyze.regressor-identity` | `analyze:read` | `frob.analyze.regressor-identity.out/1` | `audit` | `frob analyze regressor-identity --robot iiwa7 --states 10000 --seed 7 --json` |

The remaining fields follow the rule of the cluster: `input_schema` is `frob.<verb name>.in/1`; `idempotent =
true`; `cost_model = "static: …"` with the bound of the implementing bead; `purpose` is the Purpose column;
`refusals` is the chapter list above, and the generator appends the surface-wide codes of a `read` verb (720
scope missing, 723 token invalid, 724 not implemented in this build, 802 schema failure including an unknown
parameter) and the codes its fields imply (NJL-01's variation table): 808 (an out-of-range integer) only on
`analyze.regressor-identity`, whose `--states` is an `int53` input (arm 4; the other two have no integer input);
and the mission family of `analyze.constraint-graph` is a `params` entry of kind `artifact-input<fmission>` (it
accepts a path to a `.fmission` file, read by the format's owner), so that row also gets 891
(`format-version-unsupported`, arm 7), 734 (`path-outside-roots`, exit 5) and 828
(`code-payload-or-weight-format`, exit 3) (arm 9), and, as an artifact reader, the reality guard's 814, 816,
819, 873, 889, 895 and 897 (arm 15, BD-139); the other two rows take an identifier and a robot name, no path
and no artifact input, and get none of these. One entry in full:

```toml
[[verb]]
name = "analyze.sweep-monotone"
cluster = "analyze"
kind = "read"
tier = "S"
capabilities = ["analyze:read"]
input_schema = "frob.analyze.sweep-monotone.in/1"
output_schema = "frob.analyze.sweep-monotone.out/1"
refusals = []
idempotent = true
cost_model = "static: linear in the number of sweep points; no physics"
first_milestone = "M3"
task_clusters = ["analyze-evidence"]
examples = ["frob analyze sweep-monotone a:8b27 --json"]
owner = "frob-analyze"
purpose = "Classify a recorded scalar sweep as constant, single-threshold or non-monotone"
```

**Task-cluster seats.** `analyze-evidence` (five tools: the analyses of sensing and of recorded runs) gains
`analyze_sweep_monotone` and has six. `audit` (five tools: the verbs that later subsections define) gains
`analyze_constraint_graph` and `analyze_regressor_identity` and has seven. No cluster exceeds seven; there are
still 45 task clusters.

**The count known answers after this bead** (each replaces the value stated where the count is tested, and the
expected value of the matching known-answer registry row).

| Quantity | Before | After |
|---|---|---|
| Verbs in the registry; clusters | 244; 44 | 247; 44 |
| By kind: read, run, mutate, admin, two kinds | 136, 84, 15, 6, 3 | 139, 84, 15, 6, 3 |
| By first milestone | M0 16, M1 72, M2 42, M3 53, M4 46, M5 11, M6 1, M8 3 | M0 16, M1 72, M2 42, M3 55, M4 47, M5 11, M6 1, M8 3 |
| Verbs outside the mechanical inventory of 163 | 81 | 84 |
| Rows of cluster `analyze` | 26 | 29 |
| Verbs seated in a task cluster; base tools | 241; 3 | 244; 3 |
| Task clusters below seven tools | `audit` (5), `analyze-evidence` (5) | `analyze-evidence` (6) |
| MCP tools with every cluster enabled | 244 | 247 |

The "Before" column is NJL-03's complete registry: the specification's 243 and `artifact.import`. If the owner
registers `fidelity.rank` (an open decision), every total is one higher, as before. The kind row counts kinds as
the specification prints them and as this bead's rows declare them (NJL-03's convention).

**Staging.** The table describes the complete registry, as NJL-03's count rows do: rows enter with their
cluster's bead from M0 to M4, so at this bead's milestone the file does not yet hold the rows of the clusters
whose registry beads land at M4 (`fleet`, `foundry`, `gait`, `opt`, `policy`, `predict`, `skill`, `solve`). From
M3 the table is asserted on NJL-03's baseline list plus the `[[addition]]` records, and on the registry itself
from the first build in which every cluster has a row (NJL-03's `not-due` rule before that). `[counts]`, below,
states the totals of the rows the file holds at each commit, which is what `count-mismatch` compares: this bead
writes it with the totals of its own commit, and every later change that adds a row raises it in the same
change (rule 4), so it reaches the table's values with the last M4 cluster.

**The rule for additions (DESIGN).** The registry file gains two pieces of data, both checked at build time.

```toml
[counts]                       # the totals of the rows this file holds; tests read it (values of the complete registry shown)
total = 247
clusters = 44
by_kind = { read = 139, run = 84, mutate = 15, admin = 6, two_kinds = 3 }

[[addition]]                   # every verb beyond the baseline of 243
name = "artifact.import"       # written with the row by VERBS-artifact at M1 (NJL-03)
added_by = "VERBS-artifact"
defined_in = "§21.17"

[[addition]]
name = "analyze.sweep-monotone"
added_by = "NJLx-51"
defined_in = "§25.30"
```

`by_kind` counts a row under `two_kinds` when it has an `actions` list and under its `kind` otherwise, so the
values the file holds also reflect the rows to which cluster beads have added actions (rule 4; NJL-01 lists
them); the block shows the kinds as the specification prints them.

1. `count-mismatch`: the rows do not match `[counts]`.
2. `unrecorded-addition`: the set of registered names minus the baseline list
   `fixtures/golden/verbs/baseline-243.txt` differs from the `[[addition]]` names; and, in the other direction,
   a baseline name absent from the registry that is not a row of the retired-name table (a withdrawal, such as
   `solve.run` if KILL-SOLVE fires, is recorded there and lowers `[counts]` in the same change). This direction
   is applied to the clusters that have a row in the file: a cluster whose registry bead has not landed yet (the
   staging above) has no row and is skipped, and its names are checked from the change that enters them.
3. `unregistered-command`: a `frob <path>` command line in `scripts/e2e/` or in a registry example whose path is
   not a registered verb (run by the harness's coverage check beside its existing direction, every verb has a
   script).
4. A bead that introduces a verb adds its row, its `[[addition]]` record, its seat and the new counts in one
   change, or does not use the name. A withdrawn name goes to the retired-name table and is never reused. A bead
   that gives an existing row a second kind (an `actions` entry) updates `by_kind` in the same change: N28c-44's
   `run-control` action makes `design.diff` the fourth row with two kinds at M3 (VERBS-design), so `by_kind`
   moves one from `read` to `two_kinds` once both changes are in, whichever lands second recording the combined
   figure.

**Before the handler.** The rows are entered with this bead; until its implementing bead lands (N25e-08 and
N25e-27 at M3, N25e-19 at M4) each verb answers one envelope with `FROB-REF-724`, exit 5, on both surfaces, and
`frob capabilities --json` lists it as absent. Whether a verb answers is decided by its handler being in the
build, never by comparing the build's milestone with `first_milestone` (NJL-03's rule).

## Acceptance criteria

- [ ] `registries/verbs.toml` holds the three entries with every field of the entry schema, `[counts]` equal to the rows the file holds, and four `[[addition]]` records (`artifact.import`, written at M1, and the three of this bead); the registry builds with all lints.
- [ ] The count table above holds exactly on NJL-03's baseline list plus the additions: 247 verbs in 44 clusters with 139 `read`, 84 `run`, 15 `mutate`, 6 `admin` and 3 two-kind rows, and the milestone row sums to 247 (in the file's `[counts]` a row with an `actions` list counts under `two_kinds`, so N28c-44's `run-control` action on `design.diff` moves one from `read` to `two_kinds` at M3, rule 4); it holds on the registry itself from the first build in which every cluster has a row, and is reported `not-due` with the unregistered clusters before that.
- [ ] `frob-surface-diff parity --cluster analyze` exits 0 in process and through `frobd --mcp`: registry, CLI and MCP each count 29.
- [ ] After `session_focus {cluster: "audit"}` the list holds seven tools plus the base three; `analyze-evidence` holds six plus three; the list budget of 12,000 bytes holds for both.
- [ ] `frob <verb> --help --json` equals `fixtures/golden/verbs/analyze/<verb>.help.json` byte for byte for each of the three, twice, in debug and release, on Linux and macOS.
- [ ] While its handler is not in the build each verb answers `FROB-REF-724`, exit 5; once its implementing bead has landed its registry example runs with the result that bead specifies; a misspelt parameter is `FROB-REF-802`, exit 3.
- [ ] The three lints fire on their planted defects and name them.
- [ ] `frob robot-docs --json` and `frob capabilities --json` list exactly the rows the file holds, `[counts].total` of them (247 once the registry is complete); the alias lint finds no second spelling of the three.
- [ ] Every older statement of a moved count (the reconciliation test, the task-cluster test, the `analyze` cluster test, the known-answer rows of the registry) reads the new value in the same commit.

## Test plan

### Unit tests

`cargo test -p frob-verbs --test cluster_analyze_late` and `--test registry_counts` (the real generated table; no
mocks; planted registries are TOML strings labelled `synthetic`):
- `rows_present`: the three names exist in cluster `analyze`; the cluster has 29 rows.
- `kinds_milestones_tiers`: all three `read`; M3, M3, M4; S, F, F.
- `schemas_scopes_owners`: the machine-fields table, field for field.
- `refusal_lists`: chapter codes none, 868, none; appended for its kind and fields (NJL-01): {720, 723, 724,
  802} for each, plus 808 for `analyze.regressor-identity` (its `int53` input `--states`), plus {891, 734, 828,
  814, 816, 819, 873, 889, 895, 897} for `analyze.constraint-graph` (its `artifact-input<fmission>` parameter);
  the full lists are {720, 723, 724, 802} for `analyze.sweep-monotone`, {868, 720, 723, 724, 802, 891, 734,
  828, 814, 816, 819, 873, 889, 895, 897} for `analyze.constraint-graph` and {720, 723, 724, 802, 808} for
  `analyze.regressor-identity`.
- `mcp_names_one_to_one`: `analyze_sweep_monotone`, `analyze_constraint_graph`, `analyze_regressor_identity`, with no collision in the whole registry.
- `seats` (on NJL-05's target file): `audit` has seven tools, `analyze-evidence` six, no cluster above seven, 244 seated verbs.
- `registry_total_247` (on the baseline list plus the additions, and on the registry once it is complete): the count table; the per-kind and per-milestone sums both equal 247.
- `counts_equal_rows`: `[counts]` equals the totals of the rows the file holds at this commit.
- `additions_equal_registry_minus_baseline`: exactly the four names, `artifact.import` and the three of this bead.
- `lint_count_mismatch` (a fourth row without a count change), `lint_unrecorded_addition`, `lint_unregistered_command` (a script line `frob analyze constraint-graf …`).
- `unimplemented_answers_724`: in a build without the verb's handler (the capability report lists it absent) each verb returns code 724, exit 5, one envelope.
- `integer_types`: `--states` is `int53` and `--seed` is `u64str` in the generated input schema.

### End-to-end script and logging

`scripts/e2e/verbs_analyze_late.sh` (header: `bead=NJLx-51 milestone=M3 labels=tether,verbs,verb:analyze.*
gates=GATE-TETHER-02 seed=1 needs=cpu,mcp,frobd plants=missing-row,count-stale,unrecorded`). It sources the
shared helper and `scripts/e2e/verbs_lib.sh`. Steps: (1) `frob capabilities --json` and `frob robot-docs --json`:
as many verbs as `[counts].total` of the build's registry (247 once it is complete), the three present with kind
and milestone; (2) `frob-surface-diff parity --cluster analyze` over stdio
and through `frobd --mcp`, counts 29, every difference list empty; (3) `frob-surface-diff snapshot --check` for
the three; (4) each registry example verbatim, expecting a result (exit 0 or 2) or `FROB-REF-724` exactly when
the capability report says the verb is absent; (5) the same calls as MCP tools after `session_focus` on `audit`
and on `analyze-evidence`, the envelopes agreeing with the CLI on `verb`, `ok`, `exit`, `result`, `refusal` and
`evidence`; (6) one call per verb with a misspelt parameter → `e2e_expect_refusal FROB-REF-802 3`, and
`frob analyze constraint-graph /etc/hosts --json` (a path outside the roots) → `e2e_expect_refusal FROB-REF-734
5`; (7) the three
plants: a registry without `analyze.constraint-graph` (the CLI answers `FROB-REF-801` and the coverage check
names the script line), `[counts] total = 243` (the build fails with `count-mismatch`), a row with no
`[[addition]]` record. Log: `target/e2e-logs/verbs_analyze_late/<utc>.ndjson` with a transcript: each `cmd` with
exit code, wall time and the envelope's `evidence` block; `metric` lines `registry_verbs` (== `[counts].total`),
`analyze_rows` (== 29), `mcp_tools` (== `[counts].total`), `tools_listed` per focused cluster (<= 10) and `list_bytes`
(<= 12000), tagged with the gate; refusals with code and first repair; the first differing verb and field on
failure; the `summary`.

### Known answers and negative controls

- Known answers (exact): the count table; the three tool names; 16 + 72 + 42 + 55 + 47 + 11 + 1 + 3 = 247 and
  139 + 84 + 15 + 6 + 3 = 247; 163 + 84 = 247; 244 seated + 3 base = 247.
- Negative controls: the three plants above; a served MCP list missing one of the three tools (`missing_mcp` not
  empty); an output property renamed without a version change (the snapshot check reports `breaking`).
- Invariants carried: INV-TETHER-01 (every surface is generated from the registry; nothing exists on one surface
  and not the other) as step (2); INV-TETHER-02 (exactly one envelope per invocation) on every call.
- Determinism lanes: entries, tool definitions and snapshots byte-identical twice on one machine, across debug
  and release and across Linux and macOS (class D0); the verbs' own lanes belong to their implementing beads.

## Considerations

- The count constants live in three older tests (the reconciliation, the task-cluster assignment and the
  `analyze` cluster) and in the known-answer registry rows for the registry counts. This bead changes them in
  the commit that adds the rows; `[counts]` then makes the registry file the one place a later addition edits.
- The registry is frozen at the M3 exit. These rows must land before the freeze; the regressor identity is
  registered then and answers `FROB-REF-724` until M4.
- `analyze.constraint-graph` serves an experiment that settles an open question. If the owner's decision makes
  the measurement a one-off, the verb stays registered as the way to re-measure a new robot class; if it is
  withdrawn, its name is retired, the counts go to 246 and `audit` to six.
- If the placement lane falls at its kill gate (KILL-SYNTH-REGRESSOR), `analyze.regressor-identity` stays: the
  calibration subsystem uses the same regressor.
- The two seats in `audit` fill that cluster. The next late verb needs a new task cluster or a regrouping; the
  lints, not this table, are the contract. The seats are written into NJL-05's target file
  (`fixtures/golden/verbs/task_clusters_target.toml`) and into the two `[[task_cluster]]` tables in the same
  change, so `assignment_is_target_restricted` holds.
- The `[counts]` and `[[addition]]` tables are top-level tables of the registry file, whose strict reader is
  N32a-02's build script (an unknown key fails the build); N32a-02's closed list of top-level tables admits both,
  and this bead adds the lints `count-mismatch` and `unrecorded-addition` on them.

## References

Appendix J (J.1 entry form; J.3; J.4 one name per verb; J.5 task clusters), §32.6, §32.12.4, §0.2 (reachability),
§25.28, §25.30, §25.33, §28.4; GATE-TETHER-02; INV-TETHER-01, -02; OPEN-AUGUR-3.
