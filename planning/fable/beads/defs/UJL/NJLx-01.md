---
key: "NJLx-01"
title: "frob-verbs registry: three late analysis verbs (sweep-monotone, constraint-graph, regressor-identity); 246 rows"
type: "task"
priority: 2
milestone: "M3"
labels: ["m3", "tether", "augur", "frob-verbs", "solid", "registry", "verb", "test"]
parent: "WP-26"
deps: ["BD-022", "NJL-01", "NJL-02", "NJL-03", "NJL-05", "VERBS-analyze", "STD-E2E"]
needs: []
related: ["N25e-08", "N25e-27", "N25e-19", "N25e-21", "N25b-08", "BD-312", "BD-376", "NJL-04", "NJL-06"]
gates: ["GATE-TETHER-02"]
plan_refs: ["Appendix J (J.1, J.3, J.4, J.5)", "§32.6", "§32.12.4", "§0.2", "§25.28", "§25.30", "§25.33", "§28.4"]
superseded_by: "NJLx-51"
---
## Background

FrankenRobotics (a robotics simulation and design system) is reached through the command line `frob`, the daemon
`frobd` and the MCP server `frob-mcp` (the Model Context Protocol, by which an agent harness lists and calls
tools). Every capability is a verb, `cluster.verb`, declared once in `registries/verbs.toml` (crate `frob-verbs`,
tier Solid). The CLI, the MCP tools, the schemas, the refusal table and the agent guide are generated from that
file, and gate GATE-TETHER-02 compares the served surfaces with it. The registry is **closed**: it holds the 243
verbs the specification lists, and count tests fail when a row is added or removed without the counts being
changed in the same commit.

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

This bead is the registry of record for the three rows, moves the count known answers from 243 to 246, seats the
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
45-cluster assignment as a whole (NJL-05).

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
parameter, 808 an out-of-range integer). One entry in full:

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

**The count known answers after this bead** (each replaces the value stated where the count is tested).

| Quantity | Before | After |
|---|---|---|
| Verbs in the registry; clusters | 243; 44 | 246; 44 |
| By kind: read, run, mutate, admin, two kinds | 136, 83, 15, 6, 3 | 139, 83, 15, 6, 3 |
| By first milestone | M0 16, M1 72, M2 41, M3 53, M4 46, M5 11, M6 1, M8 3 | M0 16, M1 72, M2 41, M3 55, M4 47, M5 11, M6 1, M8 3 |
| Verbs outside the mechanical inventory of 163 | 80 | 83 |
| Rows of cluster `analyze` | 26 | 29 |
| Verbs seated in a task cluster; base tools | 240; 3 | 243; 3 |
| Task clusters below seven tools | `audit` (5), `analyze-evidence` (5) | `analyze-evidence` (6) |
| MCP tools with every cluster enabled | 243 | 246 |

If the owner registers `fidelity.rank` (an open decision), every total is one higher, as before.

**The rule for additions (DESIGN).** The registry file gains two pieces of data, both checked at build time.

```toml
[counts]                       # the one place the totals are stated; tests read it
total = 246
clusters = 44
by_kind = { read = 139, run = 83, mutate = 15, admin = 6, two_kinds = 3 }

[[addition]]                   # every verb beyond the baseline of 243
name = "analyze.sweep-monotone"
added_by = "NJLx-01"
defined_in = "§25.30"
```

1. `count-mismatch`: the rows do not match `[counts]`.
2. `unrecorded-addition`: the set of registered names minus the baseline list
   `fixtures/golden/verbs/baseline-243.txt` differs from the `[[addition]]` names.
3. `unregistered-command`: a `frob <path>` command line in `scripts/e2e/` or in a registry example whose path is
   not a registered verb (run by the harness's coverage check beside its existing direction, every verb has a
   script).
4. A bead that introduces a verb adds its row, its `[[addition]]` record, its seat and the new counts in one
   change, or does not use the name. A withdrawn name goes to the retired-name table and is never reused.

**Before the milestone.** The rows are entered with this bead; until M3 (M4 for the regressor identity) each
verb answers one envelope with `FROB-REF-724`, exit 5, on both surfaces, and `frob capabilities --json` lists it
as absent.

## Acceptance criteria

- [ ] `registries/verbs.toml` holds the three entries with every field of the entry schema, and `[counts]` and three `[[addition]]` records; the registry builds with all lints.
- [ ] The count table above holds exactly: the registry test prints 246 verbs in 44 clusters with 139 `read`, 83 `run`, 15 `mutate`, 6 `admin` and 3 two-kind rows, and the milestone row sums to 246.
- [ ] `frob-surface-diff parity --cluster analyze` exits 0 in process and through `frobd --mcp`: registry, CLI and MCP each count 29.
- [ ] After `session_focus {cluster: "audit"}` the list holds seven tools plus the base three; `analyze-evidence` holds six plus three; the list budget of 12,000 bytes holds for both.
- [ ] `frob <verb> --help --json` equals `fixtures/golden/verbs/analyze/<verb>.help.json` byte for byte for each of the three, twice, in debug and release, on Linux and macOS.
- [ ] Before its milestone each verb answers `FROB-REF-724`, exit 5; from its milestone its registry example runs with the result its implementing bead specifies; a misspelt parameter is `FROB-REF-802`, exit 3.
- [ ] The three lints fire on their planted defects and name them.
- [ ] `frob robot-docs --json` and `frob capabilities --json` list 246 verbs; the alias lint finds no second spelling of the three.

## Test plan

### Unit tests

`cargo test -p frob-verbs --test cluster_analyze_late` and `--test registry_counts` (the real generated table; no
mocks; planted registries are TOML strings labelled `synthetic`):
- `rows_present`: the three names exist in cluster `analyze`; the cluster has 29 rows.
- `kinds_milestones_tiers`: all three `read`; M3, M3, M4; S, F, F.
- `schemas_scopes_owners`: the machine-fields table, field for field.
- `refusal_lists`: chapter codes none, 868, none; appended codes {720, 723, 724, 802, 808}.
- `mcp_names_one_to_one`: `analyze_sweep_monotone`, `analyze_constraint_graph`, `analyze_regressor_identity`, with no collision in the whole registry.
- `seats`: `audit` has seven tools, `analyze-evidence` six, no cluster above seven, 243 seated verbs.
- `registry_total_246`: the count table; the per-kind and per-milestone sums both equal 246.
- `additions_equal_registry_minus_baseline`: exactly the three names.
- `lint_count_mismatch` (a fourth row without a count change), `lint_unrecorded_addition`, `lint_unregistered_command` (a script line `frob analyze constraint-graf …`).
- `unimplemented_answers_724`: with the build milestone below M3 each verb returns code 724, exit 5, one envelope.
- `integer_types`: `--states` is `int53` and `--seed` is `u64str` in the generated input schema.

### End-to-end script and logging

`scripts/e2e/verbs_analyze_late.sh` (header: `bead=NJLx-01 milestone=M3 labels=tether,verbs,verb:analyze.*
gates=GATE-TETHER-02 seed=1 needs=cpu,mcp,frobd plants=missing-row,count-stale,unrecorded`). It sources the
shared helper and `scripts/e2e/verbs_lib.sh`. Steps: (1) `frob capabilities --json` and `frob robot-docs --json`:
246 verbs, the three present with kind and milestone; (2) `frob-surface-diff parity --cluster analyze` over stdio
and through `frobd --mcp`, counts 29, every difference list empty; (3) `frob-surface-diff snapshot --check` for
the three; (4) each registry example verbatim, expecting a result (exit 0 or 2) or `FROB-REF-724` exactly when
the capability report says the verb is absent; (5) the same calls as MCP tools after `session_focus` on `audit`
and on `analyze-evidence`, the envelopes agreeing with the CLI on `verb`, `ok`, `exit`, `result`, `refusal` and
`evidence`; (6) one call per verb with a misspelt parameter → `e2e_expect_refusal FROB-REF-802 3`; (7) the three
plants: a registry without `analyze.constraint-graph` (the CLI answers `FROB-REF-801` and the coverage check
names the script line), `[counts] total = 243` (the build fails with `count-mismatch`), a row with no
`[[addition]]` record. Log: `target/e2e-logs/verbs_analyze_late/<utc>.ndjson` with a transcript: each `cmd` with
exit code, wall time and the envelope's `evidence` block; `metric` lines `registry_verbs` (== 246),
`analyze_rows` (== 29), `mcp_tools` (== 246), `tools_listed` per focused cluster (<= 10) and `list_bytes`
(<= 12000), tagged with the gate; refusals with code and first repair; the first differing verb and field on
failure; the `summary`.

### Known answers and negative controls

- Known answers (exact): the count table; the three tool names; 16 + 72 + 41 + 55 + 47 + 11 + 1 + 3 = 246 and
  139 + 83 + 15 + 6 + 3 = 246.
- Negative controls: the three plants above; a served MCP list missing one of the three tools (`missing_mcp` not
  empty); an output property renamed without a version change (the snapshot check reports `breaking`).
- Invariants carried: INV-TETHER-01 (every surface is generated from the registry; nothing exists on one surface
  and not the other) as step (2); INV-TETHER-02 (exactly one envelope per invocation) on every call.
- Determinism lanes: entries, tool definitions and snapshots byte-identical twice on one machine, across debug
  and release and across Linux and macOS (class D0); the verbs' own lanes belong to their implementing beads.

## Considerations

- The count constants live in three older tests (the reconciliation, the task-cluster assignment and the
  `analyze` cluster). This bead changes them in the commit that adds the rows; `[counts]` then makes the registry
  file the one place a later addition edits.
- The registry is frozen at the M3 exit. These rows must land before the freeze; the regressor identity is
  registered then and answers `FROB-REF-724` until M4.
- `analyze.constraint-graph` serves an experiment that settles an open question. If the owner's decision makes
  the measurement a one-off, the verb stays registered as the way to re-measure a new robot class; if it is
  withdrawn, its name is retired, the counts go to 245 and `audit` to six.
- If the placement lane falls at its kill gate (KILL-SYNTH-REGRESSOR), `analyze.regressor-identity` stays: the
  calibration subsystem uses the same regressor.
- The two seats in `audit` fill that cluster. The next late verb needs a new task cluster or a regrouping; the
  lints, not this table, are the contract.

## References

Appendix J (J.1 entry form; J.3; J.4 one name per verb; J.5 task clusters), §32.6, §32.12.4, §0.2 (reachability),
§25.28, §25.30, §25.33, §28.4; GATE-TETHER-02; INV-TETHER-01, -02; OPEN-AUGUR-3.
