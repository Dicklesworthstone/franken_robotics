---
key: "VERBS-skill"
title: "frob-verbs registry: cluster `skill`, four rows, with parity, e2e and schema-snapshot checks"
type: "task"
priority: 2
milestone: "M4"
labels: ["m4", "tether", "reflex", "frob-verbs", "solid", "registry", "verb", "test"]
parent: "WP-62"
deps: ["BD-022", "NJL-01", "NJL-02", "NJL-04", "NJL-05", "STD-E2E"]
needs: []
related: []
gates: ["GATE-TETHER-02"]
plan_refs: ["Appendix J (J.1, J.2, J.3, J.5)", "§22.10", "§32.6", "§32.12.3", "§32.21 (GATE-TETHER-02)"]
---
## Background

FrankenRobotics (a robotics simulation and design system) is reached through the command line `frob`, the daemon
`frobd` and the MCP server `frob-mcp` (the Model Context Protocol, by which an agent harness lists and calls tools).
Every capability is a verb, `cluster.verb`, declared once in `registries/verbs.toml` (crate `frob-verbs`, tier
Solid). The CLI, the daemon protocol, the MCP tools, the schemas, the refusal table and the agent guide are
generated from that file, and gate GATE-TETHER-02 compares the served surfaces with it. The registry holds 244 verbs
in 44 clusters (namespaces): the 243 of the specification and `artifact.import` (VERBS-artifact); the
count of record is NJL-03's.

This bead is the registry of record for the cluster `skill`. The cluster covers skill artifacts: compiling a
`.fskill` and a design to per-mode maps and a program, checking feasibility of a skill over a set of states, showing
a skill, and moving a skill to another body. It has four verbs (1 `read`, 3 `run`); the handlers live in
`frob-control`. The bead carries every row completely, with the three checks that keep the rows true: the generated
CLI commands and MCP tools match the list; every verb has an end-to-end script; the schema printed by `frob <verb>
--help --json` is stable.

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

In: the four registry entries of cluster `skill`; the MCP tool names; the cluster's conformance script; the
help-schema snapshots; each verb's answer before its milestone. Out: behaviour (the implementing beads named below);
the entry schema and lints (NJL-01); the conformance harness (NJL-02); the alias lint (NJL-04); task clusters
(NJL-05, NJL-06); generation of the surfaces (the Tether seed beads).

## Technical approach

**The rows.**

| Verb | Kind | First milestone | Tier | Purpose | Defined in |
|---|---|---|---|---|---|
| `skill.check` | run | M4 | S | A feasibility or infeasibility witness for a skill over a set of states | §22.10 |
| `skill.compile` | run | M4 | S | A `.fskill` and a design to per-mode maps and a program | §22.10 |
| `skill.show` | read | M4 | S | A skill artifact, its phases and assumptions | §22.10 |
| `skill.transport` | run | M8 | M | Move a skill artifact to another body with a learned residual [M]. A certified claim moves through `mission.transport` | §22.10 |

**Contract of each verb.** Evidence labels used below (the `evidence` block of the envelope):
- *certificate*: a Proof claim of class Certified where the checker accepted a certificate, otherwise the class of
  the weakest input (the claim product takes the minimum); tier `static`, determinism `exact`; `reality: no-claim`.
- *static*: class Tested, tier `static`, determinism `exact` (class D0); `claims` empty unless a certificate is
  attached; `reality: no-claim`.
- *pass-through*: passes through the claim product of the stored objects it reads and adds no claim of its own; the
  read itself is `exact`.
- *screening*: class Estimated: a screening or planning signal that may discard under the ladder's rules and may not
  establish a comparison.

**`skill.check`** (MCP tool `skill_check`; owner `frob-control`)
- Inputs (BD-336): a compiled skill (a `CompiledSkillId`), or a skill (a `.fskill` path or `SkillId`) with
  `--design <design>`, in which case the verb compiles first or reuses the memoized compilation (the form of the
  row's example; both forms give the same `CompiledSkillId` and outcome); the set of states, as (a) the samples of
  each phase's reference at the skill's clock rate (the default), (b) a recorded run (`--run <run | file.frun>`,
  the `run` parameter below), or (c) a grid over the assumption box (`--grid <N>`: N states over friction from
  `mu_min` upward, the payload range and slope up to `slope_max`); `--run` with `--grid` is `FROB-REF-802`; `--refinement` (adds
  the refinement check); `--replay` (re-applies each feasible witness on the referee for one step and compares the
  realized task quantity); idempotency key; budget.
- Outputs: per state and mode `feasible` or `infeasible` with a witness, or `unknown`; counts of the three; the
  first infeasible state with its failure report; the refinement verdict; `sufficiency: "necessary-only"`;
  certificate bundle references. Evidence: *certificate* (per-state witnesses Certified about the declared model;
  a verdict over sampled states `Tested`).
- Chapter refusals: `FROB-REF-444` (a refinement counterexample under `--refinement`: the uncovered successor is
  named); `FROB-REF-448` (a skill's task wrench lies outside the body's attainable set). Both as N22-REF's
  raised-by column states.
- Implemented by: BD-336.

**`skill.compile`** (MCP tool `skill_compile`; owner `frob-control`)
- Inputs (BD-336): a skill (path or `SkillId`); a design; optional mode list; optional state-sampling
  specification; idempotency key; budget.
- Outputs: `CompiledSkillId`; per phase and mode the map digest, program identity, condition notes and scope
  findings (content-addressed and memoized on skill and model). Evidence: *static*.
- Chapter refusals: `FROB-REF-448` (a skill's task wrench lies outside the body's attainable set).
- Implemented by: BD-336.

**`skill.show`** (MCP tool `skill_show`; owner `frob-control`)
- Inputs: a skill or a compiled skill.
- Outputs: the skill artifact, its phases and assumptions, the five limit texts (N22b-01), the evidence block and
  the flags; for a compiled skill also the modes and program identities. Evidence: *pass-through*.
- Chapter refusals: none of its own.
- Implemented by: BD-336.

**`skill.transport`** (MCP tool `skill_transport`; owner `frob-control`)
- Inputs (N22b-07): a `.fskill` with a non-null `residual`; the target design (`--design`); the task family
  (`--task`); budget; idempotency key. The verb exists behind the Moonshot feature (a build without it answers
  `FROB-REF-724`).
- Outputs: the skill moved to another body with a learned residual: success with interval, the comparison with
  the tuned baseline where one exists, the residual's saturation fraction and both cost denominators; a target
  outside the residual's training envelope gives the finding `outside-training-envelope`, exit 2, with no score.
  It carries no certified claim unless `mission.transport` issued the interface certificate. Evidence:
  *screening* (class at most `Tested`, standing `screening`).
- Chapter refusals: `FROB-REF-444` (a transported skill does not refine its specification); `FROB-REF-448` (the
  exact layer is infeasible on the target, refused before any learned component runs: N22b-07 step 1).
- Implemented by: BD-436, N22b-07.

**Parameters that accept a path to a file** (NJL-01's `params` rule: an `artifact-input<F>` parameter appends
891, 734 and 828; the formats are rows of `registries/formats.toml`, BD-002: FMT-FSKILL, FMT-FBOT, FMT-FRUN). Each
parameter also accepts the identifier or name, as `task.verdict`'s `run` does:

| Row | `params` |
|---|---|
| `skill.check` | `{ skill = "artifact-input<skill>", design = "artifact-input<design>", run = "artifact-input<run>" }` (`run` is the recorded-run state source) |
| `skill.compile` | `{ skill = "artifact-input<skill>", design = "artifact-input<design>" }` |
| `skill.show` | `{ skill = "artifact-input<skill>" }` |
| `skill.transport` | `{ skill = "artifact-input<skill>", design = "artifact-input<design>" }` |

All four rows carry `needs_groups = []`: no handler of the cluster raises `FROB-REF-790` (BD-336, N22b-07).

**Machine fields of each row.**

| Verb | Capabilities | Output schema | Task clusters | Example |
|---|---|---|---|---|
| `skill.check` | `skill:run` | `frob.skill.check.out/1` | `shield`, `transfer` | `frob skill check carry.fskill --design quad-tray --key skc-1 --json` |
| `skill.compile` | `skill:run` | `frob.skill.compile.out/1` | `shield` | `frob skill compile carry.fskill --design quad-tray --key sk-1 --json` |
| `skill.show` | `skill:read` | `frob.skill.show.out/1` | `shield`, `transfer` | `frob skill show carry.fskill --json` |
| `skill.transport` | `skill:run` | `frob.skill.transport.out/1` | `transfer` | `frob skill transport carry.fskill --design carrier-b --task carry-level --key skt-1 --json` |

The remaining fields follow by rule: `input_schema` is `frob.<verb name>.in/1`; `refusals` is the chapter list of
the verb's block; `idempotent` is `true` for `read` and `admin` and `false` for `run` and `mutate` (which take an
idempotency key); `cost_model` is `static: …` for `read`, `mutate` and `admin` and the `--estimate` cost model for
`run`; a `run` row carries `wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }`; `owner` is the
crate of the block; `purpose` is the Purpose column.

One entry in full (the others follow field for field):

```toml
[[verb]]
name = "skill.check"
cluster = "skill"
kind = "run"
tier = "S"
capabilities = ["skill:run"]
input_schema = "frob.skill.check.in/1"
output_schema = "frob.skill.check.out/1"
refusals = ["FROB-REF-444", "FROB-REF-448"]
idempotent = false
cost_model = "estimated: `--estimate` returns CPU core-seconds, GPU seconds, memory and disk with a conformal interval"
first_milestone = "M4"
task_clusters = ["shield", "transfer"]
examples = ["frob skill check carry.fskill --design quad-tray --key skc-1 --json"]
owner = "frob-control"
purpose = "A feasibility or infeasibility witness for a skill over a set of states"
wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }
needs_groups = []
params = { skill = "artifact-input<skill>", design = "artifact-input<design>", run = "artifact-input<run>" }
```

**Conventions the rows follow.**

1. The MCP tool name is the verb name with `.` and `-` replaced by `_`. The output on the wire is one envelope
   `frob.envelope/1` with `result` specialized to the output schema; its `evidence` block is a claim product (the
   weakest class, tier and determinism class among the inputs).
2. The generator adds the common input properties of the kind (`run`: `sync_wait_ms` from 0 to 30,000 with MCP
   default 5,000, `idempotency_key`, `budget`, `estimate_only`, `on_cancel` (`leave`, the default, or
   `cancel_job`) and `envelope_scope` (`--envelope-scope <gate:…|campaign:…|pretrain:…>`: the approved compute
   envelope scope the job cites; its schema, its place in the scope order and its refusals are NJL-01's);
   `mutate`: `idempotency_key`, `expected_parent`). `idempotency_key` is in no JSON-schema `required` array: the
   annotation `x-frob-key` (`"required"`, or `"optional"` on a `key_optional` entry) carries the rule, and a
   missing key is refused with 805 by the admission transaction on both surfaces. The generator appends the
   surface-wide refusals: `FROB-REF-720` (scope missing), 723 (token invalid), 724 (not implemented in this
   build) and 802 (schema failure, including an unknown parameter) for every verb; 701 to 704 (budget and
   admission), 761 (above the compute envelope in force), 803 (key reused with different content) and 805
   (missing key) for `run`; 725, 803, 804 (stale parent) and 805 for `mutate`. The other properties and codes are
   added by field, as the variation table of NJL-01 lists them (that table is the one list; its variations 10
   to 16, which later beads land, are read there and not copied here): 806 and 807 for paginated reads; 808 (an
   integer outside ±(2⁵³ − 1)) for an entry whose generated input schema has an `int53` property and for no
   entry by kind (every `run` entry has one in `sync_wait_ms`, every paginated entry in `limit`); a
   `key_optional` entry is not given 805; a non-`run` entry with `estimable = true` gets `estimate_only`; 790
   for a non-empty `needs_groups`; 891 to 893 for artifact `params`; 734 and 828 for a parameter that accepts a
   path to a file (`artifact-input<F>`, `artifact-identity<F>`, `foreign-file`).
3. Exit codes: 0 done; 2 done with findings; 3 refused input (1xx to 5xx, and 8xx other than conflicts); 4 budget
   (70x and 761); 5 capability (72x, 73x); 6 device (6xx); 7 conflict (803, 804, 809); 8 accepted and still running; 9
   indeterminate; never 1 on purpose.
4. Rows are entered when the cluster is first registered (M4); each row's tool is added to the task-cluster
   tables its `task_clusters` names in the same change (NJL-05's staging rule). A verb whose handler is not in
   the build answers one envelope with `FROB-REF-724`, exit 5, on both surfaces, and `frob capabilities --json`
   lists it as absent; a reduced form that lands before the row's `first_milestone` answers and is listed as
   implemented with its form named (NJL-01, NJL-03: the field keeps the appendix's milestone).

**Notes for this cluster.**

- `skill.transport` is a Moonshot (M8): it moves a skill artifact with a learned residual and carries no certified
  claim across unless `mission.transport` has issued the interface certificate for the same pair of bodies.

## Acceptance criteria

- [ ] `registries/verbs.toml` holds exactly four entries with `cluster = "skill"` (1 `read`, 3 `run`), each with
      every field of the entry schema, an example and a task-cluster list; the alias lint finds no second spelling.
- [ ] `frob-surface-diff parity --cluster skill` exits 0 in process (`frob-mcp` over stdio) and through `frobd
      --mcp`: registry, CLI and MCP each count 4, and every difference list is empty apart from the harness's closed
      exemptions.
- [ ] Every verb has an end-to-end script: `scripts/e2e/verbs_skill.sh` runs each verb of the cluster (four), and
      from a verb's first milestone a capability script of its implementing bead exists (`frob-surface-diff
      e2e-coverage`).
- [ ] `frob <verb> --help --json` equals `fixtures/golden/verbs/skill/<verb>.help.json` byte for byte for each of
      the four, on two runs, debug and release, Linux and macOS; a planted renamed output property is classified
      `breaking` and refused.
- [ ] A verb not yet implemented answers `FROB-REF-724`, exit 5, with one envelope on both surfaces; an implemented
      verb runs its registry example with the result its implementing bead specifies; a misspelt parameter is
      `FROB-REF-802`, exit 3; a `run` or `mutate` call without a key is `FROB-REF-805`, exit 3.
- [ ] The generated refusal table lists for each verb exactly its chapter codes plus the codes appended for its
      kind and fields (NJL-01), and every code exists in `registries/refusals.toml`.
- [ ] The gate row is recorded with the measured counts by a session that did not write the rows.

## Test plan

### Unit tests

`cargo test -p frob-verbs --test cluster_skill` (the real generated table; no mocks; planted registries are TOML
strings labelled `synthetic`):
- `rows_present`: exactly 4 rows in the cluster, with the names of the rows table; a planted extra row or a removed
  row fails.
- `kinds_milestones_tiers`: each row's kind, first milestone and tier equal the rows table (for example
  `skill.transport` is `run` at M8).
- `schemas_and_scopes`: input schemas are `frob.<name>.in/1`; output schemas and capabilities equal the
  machine-fields table.
- `refusal_lists`: each row's chapter codes equal its block (`skill.check`: 444, 448; `skill.compile`: 448;
  `skill.show`: none; `skill.transport`: 444, 448); the appended codes equal the rule for its kind and fields
  (891, 734 and 828 on all four from `params`; no 790).
- `inputs_by_row`: `skill.check` accepts a `CompiledSkillId` and the `.fskill --design` form, which give the same
  `CompiledSkillId` and outcome (BD-336's `check_accepts_skill_and_design`), the three state sources (the
  default reference samples, `--run <run | file.frun>`, `--grid <N>`; `--run` with `--grid` is 802) and
  `--refinement` and `--replay`;
  `skill.transport` takes `--design` and `--task` (a planted `--to` is 802).
- `mcp_names_one_to_one`: the tool names are the mapped names (`skill_check`, `skill_compile`, `skill_show`,
  `skill_transport`), with no collision against any other cluster.
- `task_clusters_and_examples`: every row sits in a task cluster of at most seven tools or is a base tool; each
  example parses against the generated CLI and its command path is the row's own.
- `kind_behaviour`: a `run` row has the six common properties (`sync_wait_ms`, `idempotency_key`, `budget`,
  `estimate_only`, `on_cancel`, `envelope_scope`), with `idempotency_key` outside the schema's `required` array
  and annotated `x-frob-key: "required"`, and refuses `sync_wait_ms = 30001` with code 802; a verb whose
  capability is absent returns code 724, exit 5, one envelope.

### End-to-end script and logging

`scripts/e2e/verbs_skill.sh` (header: `bead=VERBS-skill milestone=M4 labels=tether,verbs,verb:skill.*
gates=GATE-TETHER-02 seed=1 needs=cpu,mcp,frobd plants=alias,missing-row,renamed-field`) sources the shared helper
and `scripts/e2e/verbs_lib.sh` and calls `verbs_cluster_conformance skill`. Steps: (1) `frob capabilities --json`
and `frob robot-docs --json` list the four verbs with kind and milestone; (2) `frob-surface-diff parity --cluster
skill` over stdio and through `frobd --mcp`, every difference list empty; (3) `frob-surface-diff snapshot --check
--cluster skill`; (4) each verb's registry example verbatim, expecting a result (exit 0 or 2), a job handle (exit 8,
then `frob job wait`) or `FROB-REF-724` exactly when the capability report says the verb is absent; (5) for each
implemented verb the same call as an MCP tool, the two envelopes agreeing on `verb`, `ok`, `exit`, `result`,
`refusal`, `evidence`, `bound`, `warnings` and `next_actions`; (6) per verb one call with a misspelt parameter
(`e2e_expect_refusal FROB-REF-802 3`) and, for `run` and `mutate` verbs, one without a key (`e2e_expect_refusal
FROB-REF-805 3`); (7) one envelope per call and no exit code 1.

Log: `target/e2e-logs/verbs_skill/<utc>.ndjson` with a transcript beside it. Each `cmd` line has the exact command,
exit code, wall time and the envelope's `evidence` block; each `assert` has observed and expected values and the tag
`--gate GATE-TETHER-02`; the metrics `registry_verbs`, `cli_verbs` and `mcp_tools` are logged against 4 and the
difference counts against 0; refusals are logged with code and first repair; a failure logs the first differing verb
and field; the `summary` line gives assertions passed and failed and the gate touched.

### Known answers and negative controls

- Known answers: the cluster has 4 verbs (1 `read`, 3 `run`); the tables above. KA-755 (the exit-code table on
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
  example or answers the not-implemented refusal. Verbs of this cluster first answer at M4, M8. The per-milestone
  surface beads (BD-376) re-run this script as verbs land; that is how GATE-TETHER-02 is re-measured.
- Scope names are proposals (`<cluster>:<read|run|write|admin>` plus the named scopes of the surface) until the
  capability bead BD-258 fixes the scope registry; a `run` or `write` scope of a cluster implies its `read` scope.
  Inputs and outputs above name contents, not flag spellings, except where a flag is quoted; the implementing bead
  fixes the spellings and the snapshot then pins them (additive changes keep the schema's major version; anything
  else raises it).

## References

Appendix J (J.1 entry form; J.2 inventory rows; J.5 task clusters), §22.10, §32.4, §32.6, §32.12.3, §32.18, §32.21;
GATE-TETHER-02; INV-TETHER-01, -02; KA-755.
