---
key: "NJL-04"
title: "frob-verbs: one name per verb: the alias lint, the retired-name table, fifteen mechanisms with one owner each"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "tether", "frob-verbs", "solid", "registry", "verb", "test"]
parent: "WP-06"
deps: ["BD-022", "BD-023", "NJL-01", "STD-E2E"]
needs: []
related: ["NJLx-51", "STD-TEST", "VERBS-actuator", "VERBS-algebra", "VERBS-analyze", "VERBS-artifact", "VERBS-autopsy", "VERBS-contact", "VERBS-control", "VERBS-cores", "VERBS-design", "VERBS-eval", "VERBS-evidence", "VERBS-experiment", "VERBS-explain", "VERBS-fidelity", "VERBS-fleet", "VERBS-foundry", "VERBS-gait", "VERBS-ground", "VERBS-job", "VERBS-mission", "VERBS-module", "VERBS-nemesis", "VERBS-operator", "VERBS-opt", "VERBS-phylum", "VERBS-policy", "VERBS-predict", "VERBS-prove", "VERBS-report", "VERBS-result", "VERBS-run", "VERBS-scene", "VERBS-sense", "VERBS-session", "VERBS-shield", "VERBS-sim", "VERBS-skill", "VERBS-solve", "VERBS-spark", "VERBS-store", "VERBS-study", "VERBS-task", "VERBS-telemetry", "VERBS-train"]
gates: ["GATE-TETHER-02"]
plan_refs: ["Appendix J (J.2 preamble, J.4)", "§32.6", "§32.5", "§27.14", "§26.12", "§28.10", "§24.12", "§29.8", "§29.12"]
---
## Background

FrankenRobotics (a robotics simulation and design system) exposes every capability as a verb in one registry,
`registries/verbs.toml` (crate `frob-verbs`); the command line `frob`, the MCP server `frob-mcp`, the schemas
and the agent guide are generated from it. The rule of this bead: **the registry holds no alias**. Every verb
is registered under the name its owner chapter gives it, the surface lists the same names, and a lint refuses a
second spelling of a registered verb. Where two chapters once named one mechanism, the owner's name is the
registered one and the other name is not a verb.

Why (DESIGN). An agent chooses tools from a list; two names for one mechanism double the list, split the
documentation and, worse, split the evidence: a result recorded under one name is not found by a search under
the other, and an idempotency key is scoped to (principal, verb, key), so the same request under two names
would be two jobs and two charges. Aliases also defeat the parity gate, because "the served tool list equals
the registry under the name map" has no meaning when the map is many-to-one. Rejected: deprecated aliases kept
"for compatibility" (there is no installed base to be compatible with, and every alias is a second surface to
test); aliases on the CLI only (the two surfaces would then differ, which the first invariant of the surface
forbids).

FACT about how the registry was filled: a mechanical scan of the specification for dotted names found 169. One,
`policy.rs`, is a source file name in a sibling library. Five more were second names for a mechanism that an
owner chapter registers under another name, or had no owner; they are not registered. The other 163, with 80
that the scan missed, are the 243 registered verbs.

## Scope

In: the table of mechanisms that were named twice and the one registered name of each; the table of retired
names; the lint and where it runs; what the surfaces do when a retired or unknown name is typed; the ban on
alias features in the CLI generator and in the MCP registration. Out: the entry schema and the name grammar
(NJL-01); the parity harness (NJL-02); the count reconciliation (NJL-03); the open decision on `fidelity.rank`
(recorded with bead N16b-27; this bead only reads it).

## Technical approach

**The fifteen mechanisms and their one name each.** The owner column is the chapter section whose bead
implements the verb; the other chapters call it.

| Mechanism | Registered verb | Owner |
|---|---|---|
| The separating pair of worlds | `mission.separate`; its search machinery is §26's | §25.3.1 |
| Forks and their classification | `run.fork`; the classes are those of §27.4 | §29.8 |
| First divergence of two runs | `telemetry.divergence` | §29.8 |
| Interventions and attribution | `autopsy.intervene`, then `autopsy.attribute` | §27.14 |
| The screening audit | `nemesis.audit`; the cascade applies it | §26.10 |
| A bound on a failure rate; the hard-world set | `nemesis.bound`; `nemesis.hardset` | §26.12 |
| The model-form rank check (A-19) | `fidelity.modelform` | §16.9 |
| The rank-agreement record, for rungs and for tier pairs | `eval.rank-agreement` | §28.10 |
| The pre-analysis of a fixed-n study | `eval.preanalyze`; `study.plan` calls it | §24.12 |
| Certificate bundles | `prove.bundle`, `prove.verify`, `prove.list` | §12 |
| Calibration | `ground.ingest`, `ground.identify`, `ground.score` | §30.16 |
| Structure and the build packet | `foundry.size`, `foundry.optimize`, `foundry.check`, `foundry.export` (`--with-firmware` under its feature) | §31.15 |
| The co-design cascade and its one-call form | `opt.cascade`; `solve.run` is the agent-facing orchestration verb that calls it | §28.8, §32.6.2 |
| Transport | `mission.transport` for a certified claim through an interface map [F]; `skill.transport` for a skill artifact with a learned residual [M] | §25.3.1, §22.10 |
| The self-contained replay | `telemetry.export-html`, in the telemetry cluster; the viewer text is §32.15 | §29.12 |

Readings that follow from the table and bind implementers:
1. Autopsy registers no fork verb and no divergence verb: a fork point is made with `run.fork`, whose classes
   are the five fork classes of the counterfactual chapter, and the first divergence of two runs is read with
   `telemetry.divergence`.
2. `nemesis.audit` is the one screening audit; the co-design cascade applies it and has no audit verb of its own.
3. `eval.preanalyze` is the one pre-analysis of a fixed-n study; `study.plan` calls it and attaches its hash.
4. `opt.cascade` is the mechanism and `solve.run` the agent-facing orchestration verb that calls it. They are
   two verbs because their contracts differ (the second adds no evidence of its own); neither is an alias.
5. `mission.transport` (a certified claim through an interface map, Frontier) and `skill.transport` (a skill
   artifact with a learned residual, Moonshot) are two verbs with different outputs; neither is an alias.
6. `result.revalidate` and `mission.revalidate` are two verbs over one engine (decision bead N28c-29): the
   second takes a mission answer and applies the mission polarity rules; neither is an alias.
7. An `analyze.<name>` verb that shares its mathematics with a verb of another cluster (`analyze.observability`
   and `sense.observability`, for example) returns the studio's finding-shaped view and calls the other
   computation; the output schemas differ, so the pair is not an alias.

**What counts as an alias (the lint's four tests).** Semantics cannot be linted, so the rule is made mechanical:
1. `retired-name`: no registry row may carry a name listed in the retired-name table (below).
2. `same-handler`: no two registry rows may resolve to the same handler symbol in the dispatch table that binds
   handlers to rows.
3. `same-schemas`: no two rows may declare the same pair of input and output schema identifiers.
4. `surface-alias`: the CLI generator and the MCP registration may not attach a second name to a command or
   tool. A source scan of `frob-cli` and `surface/frob-mcp` fails on any `alias`, `visible_alias` or
   `aliases` attribute and on any second registration of one handler; the MCP server is built with duplicate
   names as an error.
A fifth test covers text an agent copies: `next-actions-registered`, over the golden envelopes of the
conformance suite, asserts that every `next_actions.command` and every registry `examples` entry starts with
the CLI path of a registered verb.

**The retired-name table (the `[[retired_name]]` array of `registries/verbs.toml`).** DESIGN, settling a
difference between two drafts: this bead had a separate file `registries/verbs-retired.toml` with `[[retired]]`
rows; the registry bead N32a-02, whose build script runs the check (its lint `RL-03`), reads a `[[retired_name]]`
table in the registry file itself. The table lives in the registry file under N32a-02's name, because the
strict reader of that file is the one that must see it and a second file would be a second build input. Rows:
`name`, `use` (a list of registered verbs, empty for a file name), `reason` (one sentence; N32a-02's strict
reader admits exactly these three fields and rejects a row without `reason`). FACT (earlier drafts of the chapters, before
the one-name pass): these second spellings appeared in drafts and were removed. The mapping to
the registered verb follows the mechanisms table; where it is this bead's reading it is marked DESIGN.

| Retired name | Use instead | Basis |
|---|---|---|
| `autopsy.fork` | `run.fork` | Forks and their classification |
| `autopsy.diverge` | `telemetry.divergence` | First divergence of two runs |
| `autopsy.interventions` | `autopsy.intervene`, then `autopsy.attribute` | Interventions and attribution |
| `eval.audit` | `nemesis.audit` | The screening audit |
| `eval.rankcheck` | `fidelity.modelform` for the model-form check; `eval.rank-agreement` for the record between rungs or tiers (DESIGN reading) | The model-form rank check; the rank-agreement record |
| `fidelity.rank` | `eval.rank-agreement` with a tier pair (default of the open owner decision; the row leaves this table if the owner registers the name) | The rank-agreement record |
| `nemesis.show` | `nemesis.hardset` with no `--action`: its default read action, `show`, has no name to pass, and `--action show` is `FROB-REF-802` (VERBS-nemesis), so the repair's skeleton is `frob nemesis hardset --class <class> --json` (DESIGN reading) | The hard-world set |
| `cert.bundle` | `prove.bundle` | Certificate bundles |
| `ground.import` | `ground.ingest` | Calibration |
| `foundry.structure` | `foundry.size`, `foundry.optimize`, `foundry.check` | Structure and the build packet |
| `policy.rs`, `contact.rs` | none: source file names, not verbs | The scan's false positives |

```toml
[[retired_name]]
name = "autopsy.fork"
use = ["run.fork"]
reason = "forks and their classification are owned by the replay chapter"
```

**What a caller sees.** Typing a retired or unknown name on the CLI gives one envelope with `ok: false`,
`FROB-REF-801` (unknown verb), exit 3, and nothing is charged. For a retired name, `refusal.repairs[0]` names
the registered verb with a complete command skeleton. That is a repair, not an alias: nothing executes. On MCP
an unknown tool name is answered by the MCP library before any generated code runs (a JSON-RPC error, −32602,
"Unknown tool"), so no envelope exists there; this is the one sanctioned difference between the surfaces for
unknown names.

**Where it runs.** Tests 1 and 3 are N32a-02's lint `RL-03` in the build script of `frob-verbs` (a registry with
an alias does not build); this bead supplies the table's rows and the planted registries. Test 2 cannot run in
the build script, which sees no handler: it is a unit test of `frob-verbs` over the dispatch table that binds a
handler to each row (the handler bindings of the per-milestone surface beads, BD-137 from M1), and it runs in
every build of `frob`. Test 4 is a unit test in each surface crate (`frob-cli`, and `surface/frob-mcp`, which is
BD-023's, a dependency: the MCP half of this bead's tests and of its script needs that crate). Test 5 is in the
conformance crate. No new `xtask` subcommand is added (the workspace keeps exactly six).

**Staging (the rule NJL-03 states for the counts, applied here by cluster).** Rows enter the registry with their
cluster's `VERBS-<cluster>` bead, from M0 to M4, and those beads depend on this one, so when this bead closes at
M0 most verbs of the two tables are not registered yet. The checks that say a verb exists are therefore staged:
a verb of the mechanisms table, or a `use` target, whose cluster (the prefix of its name) has no row yet is
reported `not-due` naming the cluster, never as a pass and never as a failure; from the first build in which
its cluster has a row it must be a registered row. The checks that say a name does not exist (no retired name
is a row, no alias attribute, the lint's tests) hold at every milestone from M0. The refusal of a retired name
and its first repair are generated from the retired-name table alone, so they hold at M0 whether or not the
repair's verb is registered yet.

## Acceptance criteria

- [ ] The `[[retired_name]]` table of `registries/verbs.toml` holds the rows of the table above (twelve names);
      every `use` target whose cluster is registered is a registered verb, and the others are reported `not-due`
      with their clusters (the staging rule).
- [ ] Each of the fifteen mechanisms is reachable under its registered name and under no other: no retired name
      of a mechanism is ever a row, and from the build in which a listed verb's cluster is registered the
      registry has exactly the verbs of the middle column for it (`not-due` before that, never a pass).
- [ ] Each of the four lint tests and the fifth check fires on its planted defect and names it (negative
      controls, 5 of 5); the clean tree passes.
- [ ] `frob autopsy fork …` (a retired name) exits 3 with `FROB-REF-801`, and the first repair is a `frob run
      fork …` skeleton; `frob nosuch verb` exits 3 with `FROB-REF-801` and no repair naming a verb.
- [ ] Over MCP, calling `autopsy_fork` returns the library's unknown-tool error, and `tools/list` never
      contains a mapped retired name.
- [ ] `frob robot-docs --json` contains no retired name anywhere in its text.
- [ ] If the owner registers `fidelity.rank`, removing its row from the retired table is the only change this
      bead needs: the name is either retired or registered, never both, and the count tests follow.

## Test plan

### Unit tests

`cargo test -p frob-verbs --test one_name` (planted registries are TOML strings labelled `synthetic`):
- `mechanisms_have_one_name`: for each row of the mechanisms table, the listed verbs exist and no retired name
  of that mechanism does; a listed verb whose cluster has no row yet is `not-due` with the cluster named, and a
  `synthetic` registry that holds the cluster `run` without `run.fork` fails.
- `lint_retired_name`: a registry with a row `autopsy.fork` fails with `retired-name` and names `run.fork`.
- `lint_same_handler`: two rows bound to one handler symbol fail with `same-handler`.
- `lint_same_schemas`: two rows with identical schema identifier pairs fail with `same-schemas`.
- `retired_targets_exist`: every `use` entry resolves to a registered row, or is `not-due` because its cluster
  has no row yet; a `synthetic` registry whose cluster is registered without the target fails.
- `two_verbs_one_engine_allowed`: `result.revalidate` and `mission.revalidate`, and `opt.cascade` and
  `solve.run`, pass (different handlers and schemas): on `synthetic` rows of those shapes at M0, and on the
  registered rows from the builds in which their clusters land.
`cargo test -p frob-cli --test no_aliases` and `cargo test -p frob-mcp --test no_aliases`:
- `no_alias_attributes`: the source scan finds no alias attribute; a planted `visible_alias` fails.
- `duplicate_tool_is_build_error`: registering one handler under two tool names fails server construction.
`cargo test -p frob-conformance --test next_actions_registered`: every command in the golden envelopes and in
the registry examples begins with a registered CLI path; a planted `frob cert bundle …` fails.

### End-to-end script and logging

`scripts/e2e/verbs_one_name.sh` (header: `bead=NJL-04 milestone=M0 labels=tether,verbs,registry
gates=GATE-TETHER-02 seed=1 needs=cpu,mcp plants=alias-row,cli-alias`). Steps: (1) for each retired name with a
`use` target, run it on the CLI and `e2e_expect_refusal FROB-REF-801 3`, then assert with
`e2e_assert_repairs_order` that the first repair names the registered verb; (2) call each mapped retired name
as an MCP tool and assert the unknown-tool error; (3) `frob robot-docs --json` piped through a search for each
retired name, asserting zero hits; (4) the plants: a registry with an alias row must fail to build with
`retired-name`; a CLI generator patched to add an alias must fail `no_alias_attributes`. Logged through the
shared helper to `target/e2e-logs/verbs_one_name/<utc>.ndjson`: every command with exit code and wall time,
each refusal with its code and first repair, each assertion with observed and expected values, the controls
with `caught`, and the `summary`.

### Known answers and negative controls

- Known answer: the mechanisms table (fifteen rows) and the retired table are exact; KA-755 (unknown verb →
  `FROB-REF-801`, exit 3) covers step (1).
- Negative controls: the five planted defects of the lint; the two script plants.
- Invariant carried: INV-TETHER-01 (no verb exists on one surface and not the other, except per-surface
  defaults and the unknown-verb case), here in the form "no name exists on a surface that the registry lacks".
- Determinism: the lint result and the generated tables are identical across runs and platforms (class D0).

## Considerations

- The specification says five scanned names were not registered and does not print them; earlier drafts of
  the chapters show nine retired spellings and two file names. The table above carries all of them, which is the safe
  superset: a retired name that was never used costs nothing to deny.
- `same-schemas` would fire on two verbs that share a named artifact schema as output and have equal inputs.
  None exists today; if one appears, the lint compares the pair (input identifier, output identifier), and
  input identifiers are per verb, so a true positive needs a deliberate copy.
- The repair on a retired name is generated from the retired table, never from text an import supplied, so the
  rule that no `next_actions.command` contains imported text is kept.
- Risk: a future chapter introduces a convenience name ("just an alias on the CLI"). The source scan is the
  guard; an exception needs an ADR and a change to this bead.

## References

Appendix J (J.2 preamble, J.4), §32.6, §32.5, §27.14, §26.12, §28.10, §24.12, §29.8, §29.12; GATE-TETHER-02;
INV-TETHER-01; KA-755.
