---
key: "VERBS-task"
title: "frob-verbs registry: cluster `task`, eight rows, with parity, e2e and schema-snapshot checks"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "tether", "arena", "frob-verbs", "solid", "registry", "verb", "test", "review-notes-open"]
parent: "WP-06"
deps: ["BD-022", "NJL-01", "NJL-02", "NJL-04", "NJL-05", "STD-E2E"]
needs: []
related: []
gates: ["GATE-TETHER-02"]
plan_refs: ["Appendix J (J.1, J.2, J.3, J.5)", "§24.4", "§24.5", "§24.7", "§24.10", "§24.14", "§24.20", "§24.22", "§32.6", "§32.12.3", "§32.21 (GATE-TETHER-02)"]
---
## Background

FrankenRobotics (a robotics simulation and design system) is reached through the command line `frob`, the daemon
`frobd` and the MCP server `frob-mcp` (the Model Context Protocol, by which an agent harness lists and calls tools).
Every capability is a verb, `cluster.verb`, declared once in `registries/verbs.toml` (crate `frob-verbs`, tier
Solid). The CLI, the daemon protocol, the MCP tools, the schemas, the refusal table and the agent guide are
generated from that file, and gate GATE-TETHER-02 compares the served surfaces with it. The registry holds 244 verbs
in 44 clusters (namespaces): the 243 of the specification and `artifact.import` (VERBS-artifact); the
count of record is NJL-03's.

This bead is the registry of record for the cluster `task`. The cluster covers tasks: creating a task from a
document, showing and listing tasks by content address, the admission check, a verdict on a recorded run, the
settled-pose cache, registered fixtures against their known answers, and the audit of a task before any candidate is
scored. It has eight verbs (4 `read`, 3 `run`, 1 `mutate`); the handlers live in `frob-arena`, `frob-spec`. The bead
carries every row completely, with the three checks that keep the rows true: the generated CLI commands and MCP
tools match the list; every verb has an end-to-end script; the schema printed by `frob <verb> --help --json` is
stable.

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

In: the eight registry entries of cluster `task`; the MCP tool names; the cluster's conformance script; the
help-schema snapshots; each verb's answer before its milestone. Out: behaviour (the implementing beads named below);
the entry schema and lints (NJL-01); the conformance harness (NJL-02); the alias lint (NJL-04); task clusters
(NJL-05, NJL-06); generation of the surfaces (the Tether seed beads).

## Technical approach

**The rows.**

| Verb | Kind | First milestone | Tier | Purpose | Defined in |
|---|---|---|---|---|---|
| `task.check` | read | M1 | S | Units, clocks, windows, reach against horizon, labelled examples, the mutation check | §24.5 |
| `task.fixture` | run | M1 | S | Run a registered fixture's analytic layer or batch against its known answers | §24.10 |
| `task.list` | read | M1 | S | Tasks, by content address | §24.14 |
| `task.new` | mutate | M1 | S | Create a task from a document | §24.4 |
| `task.settle` | run | M1 | S | Build or refresh the settled-pose cache for a model and task | §24.7 |
| `task.show` | read | M1 | S | A task's sections, requirement and examples | §24.4 |
| `task.verdict` | read | M1 | S | Evaluate a task's requirements on a recorded run; returns `frob.verdict/1` | §24.5 |
| `task.audit` | run | M3 | S | Attack a task before any candidate is scored: foil bodies and intent invariants | §24.20 |

**Contract of each verb.** Evidence labels used below (the `evidence` block of the envelope):
- *static*: class Tested, tier `static`, determinism `exact` (class D0); `claims` empty unless a certificate is
  attached; `reality: no-claim`.
- *known answer*: a Known answer where a registered row is recovered within its tolerance; class Tested.
- *pass-through*: passes through the claim product of the stored objects it reads and adds no claim of its own; the
  read itself is `exact`.
- *run*: class Tested at best, by the claim product of the run; the tier of the run, `provisional` on a fast tier
  until promoted to the referee; the determinism class of the placement (D2 unless stated); `reality: no-claim`.

**`task.check`** (MCP tool `task_check`; owner `frob-spec`)
- Inputs: task (positional: a `.ftask` document in either view, or a `TaskId` or name; BD-122); optional
  `design` (`--design <d>`: resolves the `:from` references of the task's `randomization` section against that
  design's instances, BD-200, M2); optional `overlay` (`--overlay <file>`: a training overlay, format
  `FMT-FOVERLAY`, extension `.foverlay`, schema `frob.task.overlay/1`, checked against the task by N24a-13's
  `overlay::check`, wired in by BD-122 as a stage of the stage runner); optional `element` (`--element <file>`:
  a file of permutations, one group element, format `group-element`, schema `frob.group.element/1`, for which
  the `symmetry` block prints `preserves_task`, N24ax-02). The action `coupling` and its options are below.
- Outputs: each admission check with its result: units and scales, clocks and reductions, windows, reach against
  horizon, labelled examples, the mutation check; for the verdict its reach and whether each atom's channel kind
  makes it `sampled` or `interval` (BD-122), with a warning for every hard-limit atom labelled `sampled`
  (N24a-06); the `symmetry` block (counts of declared, `fixed` and undeclared terms, each inferred kind, every
  mismatch, and with `element` the answer `preserves_task`; N24ax-02); with `design`, the compiled randomization
  plan's summary (plan identifier, provenance shares), running nothing; with `overlay`, the overlay stage's
  report (each addressed path with its kind, `distribution` or `reward-weight`, and the card support it stays
  inside; the unchanged `TaskId`; the overlay's digest; N24a-13); the `:intent` signals the task adopts
  (N24c-21, M3); and, if OPEN-SKEL-3 is decided for its option B, the `alias_family` block (N14b-17, M3).
  Evidence: *static*.
- Chapter refusals: `FROB-REF-103` (unknown, misspelt or duplicate field, from the reader; with `overlay`, an
  overlay that names a requirement, signal, clock or split field, N24a-13; with `design`, a randomization entry
  naming a field with no parameter row, BD-200); `FROB-REF-442` (an observation channel bound to presentation
  state, first clause, raised at task registration, N20a-01; N20b-15 asserts it here); `FROB-REF-520` (an atom
  has no scale and is not declared exact); `FROB-REF-523` (a future-time operator appears in an online signal);
  `FROB-REF-525` (a signal is read across clocks without a declared reduction, or its channel kind does not admit
  the reduction); `FROB-REF-526` (the episode horizon is shorter than the reach of the verdict); `FROB-REF-528`
  (an online window exceeds its capacity); `FROB-REF-535` (dimension mismatch inside an atom; with `detail:
  representation-mismatch`, a term declared with one representation whose expression infers another, N24ax-02);
  `FROB-REF-536` (a temporal operator has no finite window); `FROB-REF-538` (a window bound is not a multiple of
  the clock period and no rounding is declared); `FROB-REF-539` (a requirement has no labelled examples, or the
  monitor disagrees with a label); `FROB-REF-554` (a future-time requirement term offered as an online reward,
  refused at compile time with the window in samples, BD-194, M2); `FROB-REF-556` (a pairing keyed by event
  order or by a training counter: refused when the exposure table is compiled, N24c-36, so before any world of
  `coupling` runs, N24c-37); `FROB-REF-891` (an unknown major version or required field, BD-122; an unknown
  representation kind, N24ax-02).
- Worded refusals (registered without a number in N24-REF or N23-REF; each enters this row's `refusals` when its
  number is allocated): `task-structure-invalid` (a structure failure at load, N24a-02);
  `randomize-numerical-parameter` (a numerical parameter such as the time step named in `randomization`, exit 3;
  proposed 572, N23-REF's row; raised at task admission, N23b-03 step 4 without `design`, and by the
  randomization compiler that `sim.run --task` shares, BD-200); `randomize-no-interval`
  (a field whose uncertainty is `exact` or absent, exit 3; proposed `FROB-REF-571`, N23bx-03's row);
  `exposure-domain-missing` and `exposure-event-undeclared` (N24c-36, M2); `wrench-command-missing` and
  `metric-moment-free-under-moment-command` (N24c-18, M2); `intent-signal-misused` (N24c-21, M3);
  `coupling-test-failed` (action `coupling`, N24c-37); `coupling-family-too-large` (action `coupling` with
  `--choose`: more than 16 candidate coupling tables, refused before any world runs; `pointer` the draws that
  carry alternatives, `detail` the product count; N24c-40, M3); `coupling-choice-not-built` (`--choose` after
  KILL-EXPODOMAIN's fallback, N24cp1-02); `stream-adapter-missing` (after KILL-EXPODOMAIN's fallback, a draw
  whose stream has no explicit adapter; exit 3, repair: write the adapter; N24cp1-02, M3);
  `disturbance-registry-invalid` (a task drawing from `registry:disturbances` while `registries/disturbances.toml`
  does not load; exit 3, `pointer` the file, slug and field; N24cp3-02, M2).
- Action `coupling` (selected only with `--action coupling`, which as a `run` action carries `--key`; there is no
  `--coupling` flag, which is the unknown-parameter 802; kind `run`, capability `task:run`, from M2; N32a-02's
  two-kind form, served by NJL-01's `action` property; DESIGN, below). Inputs (N24c-37): the pair of arms
  (`--arms A,B`: two designs with controllers, or one design under two controllers), the tier the checks run
  on (`--tier <tier>`), `--worlds N` (default 4,096); idempotency key; budget. It runs the four checks (marginal, insertion, rate and retiming invariance) and returns the record
  `frob.task.coupling/1` (each check's statistic, the worlds used, the first differing value on a failure, its
  hash), and sets the exposure table's `coupling` status to `passed` or `failed` for the arms' morphology
  class. With `--choose` (N24c-40, M3; Frontier, behind KILL-EXPODOMAIN) and `--pilot-worlds n` (default 1,024)
  on a coupling family: the candidate tables (at most 16), each one's test result and per-arm marginals, the
  paired correlation ρ̂ and Var(D) with a bootstrap interval on the pilot addresses, the choice, the pilot seed
  (recorded as consumed) and the ledger sequence of the lock record it appends; or the finding
  `alternatives-change-marginals`, which stops the procedure. Evidence: *run*.
- Implemented by: BD-122, N24a-12.
- Extended by: N24a-13 (`overlay`), N24ax-02 (`element` and the `symmetry` block), N24a-06 (the `sampled`
  warning), BD-200 (M2: `design` and the randomization refusals), N23b-03 (M2: `randomize-numerical-parameter`
  at admission), BD-194 (M2:
  554), N24c-36 (M2: 556 and the exposure refusals), N24c-18 (M2), N24c-37 (M2: the action `coupling`),
  N24c-21 (M3), N24c-40 (M3: `--choose`, `--pilot-worlds`, the lock and `coupling-family-too-large`),
  N24cp1-02 (M3, on KILL-EXPODOMAIN's fallback), N24cp3-02 (M2: `disturbance-registry-invalid`), N14b-17 (M3,
  on option B).

**`task.fixture`** (MCP tool `task_fixture`; owner `frob-arena`)
- Inputs (N24b-03): `fixture` (name: `fixture:<name>` or the bare name, which selects the same fixture); `layer`
  (`analytic`, `rigs`, `batch` or `all`; `--analytic` is accepted as a spelling of `--layer analytic`); `rows`
  (optional subset); `tier` (`host`, `referee`, `batch-cpu`, `cuda`, `metal`); `designs` (optional subset for a
  batch); `control` (a registered negative control, accepted only in a build with the cargo feature
  `gauntlet-mutations`, the one name of the feature that builds planted defects; a release build answers
  `FROB-REF-724`, exit 5); idempotency key; budget. The run axes and row selectors of N24bx-01, each
  a value the fixture's manifest declares: `--actuator-level`, `--precision`, `--seed`, `--profile`, `--variant`,
  `--set`, `--reflect`, `--law`; `--measure <slug>` and `--report a,b` (intersected with `rows`). `--battery
  fixtures` runs every registered row (N24c-10). The registered fixtures and their declared axes are listed by
  `task.list` with `kind = "fixture"`, not by an option of this verb (DESIGN, below).
- Outputs: every row of the fixture against its known answer, with tolerance and pass or fail; the `selection`
  block, `rows_not_selected`, and per row `measure` and `group` (N24bx-01). Exit 0 all selected rows pass, 2 at
  least one out of tolerance. Evidence: *known answer*.
- Chapter refusals: `FROB-REF-136` (a row option or actuator level the requested tier does not carry, or a
  precision the tier does not offer, N24bx-01); `FROB-REF-222` (a profile the tier does not register, N24bx-01);
  `FROB-REF-529` (no settled-pose cache for this model and task: a fixture whose rig needs a settled start state,
  N24b-03); `FROB-REF-530` (a run offered as a fixture run does not match the fixture's recorded hash, including
  an axis value the manifest does not declare, or a selection that leaves no row); `FROB-REF-726`
  (`mechanism-not-admitted`, reason `card-provenance`, N32-REF: a fixture run that admits an actuator model with
  the series-elasticity option enabled on a card whose `series_stiffness` has provenance `generic` or
  `assumed`; block fields `mechanism`, `reason`, `field`, `provenance_found` and `provenances_admitted`; N15b-18,
  M6).
- Implemented by: N24b-03 (the verb), BD-123 (fixture 1). Extended by: N24bx-01 (run axes and selectors, 136 and
  222), N24c-10 (`--battery fixtures`), N15b-18 (M6: 726, `card-provenance`).

**`task.list`** (MCP tool `task_list`; owner `frob-spec`)
- Inputs: namespace; optional `kind` (absent: tasks; `fixture`: the registered fixtures, N24bx-01); cursor and
  limit (paginated).
- Outputs: tasks by content address with name, family and first milestone, paged against a committed frontier
  (BD-122); with `kind = "fixture"`, per fixture its name, version, kind (`synthetic` or `measured`),
  `fixture_id`, whether it is registered, its layers, and the declared axes with their values and defaults
  (N24bx-01), under the same pagination; tasks grouped by family, per family its computed status (`planned`,
  `partial` with the missing items named, or `available`), its tasks with their `TaskId`s, the applicable robot
  classes and the baselines with scorecard handles (N24a-23, from `registries/task-families.toml`). Evidence:
  *pass-through*.
- Chapter refusals: none of its own (806 and 807 are appended because the row is paginated).
- Implemented by: BD-122. Extended by: N24bx-01 (`kind = "fixture"`), N24a-23 (grouping by family with each
  family's computed status).

**`task.new`** (MCP tool `task_new`; owner `frob-spec`)
- Inputs: a `.ftask` document in either view (positional, a file or inline); a name; idempotency key (BD-122).
- Outputs: `TaskId` (the hash of the canonical form), the canonical bytes' digest and the check report.
  Evidence: *static*.
- Chapter refusals: `FROB-REF-103` (unknown, misspelt or duplicate field); `FROB-REF-891` (an unknown major version
  or an unknown required field; N24a-02 gives `task.new` both reader codes); `FROB-REF-442` (an observation
  channel bound to presentation state, refused at task registration, N20a-01); and the static-admission codes,
  which `task.new` runs as `task.check` does and in the same order (BD-122): `FROB-REF-520`, `FROB-REF-523`,
  `FROB-REF-525`, `FROB-REF-526`, `FROB-REF-528`, `FROB-REF-535`, `FROB-REF-536`, `FROB-REF-538`,
  `FROB-REF-539`, with the meanings given under `task.check`. Worded: `task-structure-invalid` (N24a-02);
  `stream-adapter-missing` (N24cp1-02, M3, on KILL-EXPODOMAIN's fallback) and `disturbance-registry-invalid`
  (N24cp3-02, M2), with the meanings given under `task.check`.
- Implemented by: BD-122. Extended by: N24cp3-02 (M2), N24cp1-02 (M3, on KILL-EXPODOMAIN's fallback).

**`task.settle`** (MCP tool `task_settle`; owner `frob-arena`)
- Inputs: task (positional); `--model <id>`, or from M2 `--cohort <cohort>` (BD-195: the cohort's own cache;
  `--model` and `--cohort` together are `FROB-REF-802`); from M3 `--tier <batched tier>` (BD-255: BD-195's load
  path and per-tier check); `--refresh` (N24a-16: rebuild a complete cache under the call's key instead of
  returning the stored receipt; on the same build the rebuilt state bytes are identical, class D2 on the
  referee); idempotency key.
- Outputs: the settled-pose cache built or refreshed, with counts and the admission criteria met (64 accepted
  states per model; N24a-16); with `--tier`, the spawn specification, contact law and kernel versions, the first
  contact impulse after loading against the static support force per tier, and the count of worlds marked
  `Degraded: unsettled` (BD-255). Evidence: *run*.
- Chapter refusals: `FROB-REF-529` (no settled-pose cache for this model and task).
- Implemented by: N24a-16 (the verb on the CPU), BD-195 (the batched-tier caches). Extended by: BD-195 (M2:
  `--cohort`), BD-255 (M3: `--tier`).

**`task.show`** (MCP tool `task_show`; owner `frob-spec`)
- Inputs: task (`TaskId` or name); `--as json` or `sexpr`; `--section <name>` (BD-122), including `symmetry`
  (N24ax-02) and, with `--design <d>`, `reads` (N24ax-03, M2).
- Outputs: the nine sections, the requirement and its examples, bound identifiers; `--section symmetry` prints
  the `symmetry` block of `task.check`; `--design <d> --section reads` returns `bound` and `reads` (paths only),
  the block of schema `frob.task.reads/1` (N24ax-03). Evidence: *pass-through* (the `reads` block: class
  `Tested`, tier `static`, determinism exact).
- Chapter refusals: `FROB-REF-103` (with `--design`: the task names a channel or constant the design does not
  declare, `detail: no-declared-source-set`, N24ax-03).
- Implemented by: BD-122. Extended by: N24ax-02 (`--section symmetry`), N24ax-03 (M2: `--design`, `--section
  reads`).

**`task.verdict`** (MCP tool `task_verdict`; owner `frob-spec`)
- Inputs (N24c-02): a recorded run (positional: a run identifier or a `.frun` path); `task` (`--task <id>`, a
  `TaskId` or name; optional: default the `TaskId` the run recorded; required for a trace that records none, a
  labelled example trace or a `.frun` with `trace_kind: synthetic` and no `task`, where its absence is
  `FROB-REF-802` with pointer `/task`); optional `requirement` (`--requirement <id>`, the verdict's identifier;
  default: the task's verdict); optional `worlds` (default: every episode of the run);
  `--re-evaluate` (evaluate a task other than the one the run recorded, when every signal it reads was recorded);
  from M3 `--online` (BD-255: on a run of a batched tier, per past-time requirement the streamed margin's final
  value and its equality with the offline recomputation).
- Outputs: `frob.verdict/1`: the verdict, the binding atom, margins, `sampled` or `interval` labels, `undetermined`
  with a reason where a window is not covered. Exit 0 when every episode is `satisfied`, 2 when any is
  `violated`, `undetermined` or `invalid`. Evidence: *static*.
- Chapter refusals: `FROB-REF-526` (the episode horizon is shorter than the reach of the verdict); `FROB-REF-888` (a
  metric computed over missing samples: a signal the requirement reads was not recorded). Worded (N24-REF):
  `verdict-task-mismatch` (the run recorded another task and `--re-evaluate` is absent) and
  `stage-concatenation` (verdicts of separately recorded stage episodes combined into one mission verdict).
- Implemented by: BD-122, N24c-02. Extended by: BD-255 (M3: `--online`).

**`task.audit`** (MCP tool `task_audit`; owner `frob-arena`)
- Inputs (N24c-22): `task`; optional `design` and `controller` (for the idle and actual-design arms); `arms`
  (default: every arm in the build: `foil`, `actual`, `idle`, `in-campaign`); `budget` (default 5% of the task's
  reference campaign in simulated steps); `--in-campaign <result>`; idempotency key; the resolution mode
  `--resolve <finding> --accept` or `--intended`, with an optional `--clause` overriding the proposal;
  `--list-bodies` (with `--arms foil`: instantiates every foil kind for the task's class and returns each with its
  `DesignId` and `foil: true`, running no episode; N24c-23).
- Outputs: audit records (`frob.task.audit/1`) and findings ranked by score, with proposed clauses, before any
  candidate is scored; `next_actions` with the command that accepts the first finding's clause. Exit 2 with
  findings open. Evidence: *run*.
- Chapter refusals: none numbered. Worded (N24-REF): `audit-arm-not-built` (`--arms` names an arm this build does
  not contain: `foil` after KILL-FOIL's fallback, or `actual` on the pilot-falsified branch; N24cp1-01).
- Implemented by: N24c-22. Extended by: N24c-23 (`--list-bodies`), N24cp1-01 (the arms on KILL-FOIL's fallback).

**Machine fields of each row.**

| Verb | Capabilities | Output schema | Task clusters | Example |
|---|---|---|---|---|
| `task.check` | `task:read`; `task:run` for action `coupling` | `frob.task.check.out/1` | `tasks` | `frob task check t:kitchen-tray --json` |
| `task.fixture` | `task:run` | `frob.task.fixture.out/1` | `tasks` | `frob task fixture fixture:cart-sill-dock --layer analytic --key fx-1 --json` |
| `task.list` | `task:read` | `frob.task.list.out/1` | `tasks`, `batteries` | `frob task list --json` |
| `task.new` | `task:write` | `frob.task.new.out/1` | `tasks` | `frob task new kitchen-tray.ftask --key tk-1 --json` |
| `task.settle` | `task:run` | `frob.task.settle.out/1` | `tasks` | `frob task settle t:kitchen-tray --model quad-tray --key ts-1 --json` |
| `task.show` | `task:read` | `frob.task.show.out/1` | `tasks` | `frob task show t:kitchen-tray --json` |
| `task.verdict` | `task:read` | `frob.verdict/1` | `tasks` | `frob task verdict r:19e2 --task t:kitchen-tray --json` |
| `task.audit` | `task:run` | `frob.task.audit.out/1` | `audit` | `frob task audit t:kitchen-tray --key ta-1 --json` |

The remaining fields follow by rule: `input_schema` is `frob.<verb name>.in/1`; `refusals` is the chapter list of
the verb's block; `idempotent` is `true` for `read` and `admin` and `false` for `run` and `mutate` (which take an
idempotency key); `cost_model` is `static: …` for `read`, `mutate` and `admin` and the `--estimate` cost model for
`run`; a `run` row carries `wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }`; `owner` is the
crate of the block; `purpose` is the Purpose column. `task.list` has `paginated = true` (BD-122: paged against a
committed frontier). `task.check` has one kind, `read`, the kind of its default action, and `actions = [{ name =
"coupling", kind = "run", capabilities = ["task:run"] }]` (the `run` action carries the `run` row's `wait`).
`params` (N34-24's kinds; the formats are those of `registries/formats.toml`): `task.new`: `{ document =
"artifact-input<task>" }`; `task.check`: `{ task = "artifact-input<task>", design = "artifact-input<design>",
overlay = "artifact-input<task-overlay>", element = "artifact-input<group-element>" }` (the overlay format of
N24a-13 and the group-element format of N24ax-02);
`task.show`: `{ design = "artifact-input<design>" }`; `task.verdict`: `{ run = "artifact-input<run>" }` (an
identifier or a `.frun` path). `task.fixture`, `task.list`, `task.settle` and `task.audit` take identifiers and
names only.

One entry in full (the others follow field for field):

```toml
[[verb]]
name = "task.fixture"
cluster = "task"
kind = "run"
tier = "S"
capabilities = ["task:run"]
input_schema = "frob.task.fixture.in/1"
output_schema = "frob.task.fixture.out/1"
refusals = ["FROB-REF-136", "FROB-REF-222", "FROB-REF-529", "FROB-REF-530", "FROB-REF-726"]
idempotent = false
cost_model = "estimated: `--estimate` returns CPU core-seconds, GPU seconds, memory and disk with a conformal interval"
first_milestone = "M1"
task_clusters = ["tasks"]
examples = ["frob task fixture fixture:cart-sill-dock --layer analytic --key fx-1 --json"]
owner = "frob-arena"
purpose = "Run a registered fixture's analytic layer or batch against its known answers"
wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }
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
   In this cluster the inputs that accept a path to a file are the `params` entries listed above, so `task.new`,
   `task.check`, `task.show` and `task.verdict` get 891, 734 and 828 (a code the chapter list already names is
   listed once). `task.check`'s `--overlay` and `--element` files are such entries, of the registered formats
   `task-overlay` (N24a-13) and `group-element` (N24ax-02): the input stage routes each by its extension and its
   top-level `schema` member (N34-24), and the path jail's 734 and 828 and the reader codes apply to them as to
   every file input. The action `coupling` of `task.check` gets the `run` codes (701 to 704, 761, 803, 805, and
   808 for its `int53` properties) when it is invoked, and its scope `task:run`.
3. Exit codes: 0 done; 2 done with findings; 3 refused input (1xx to 5xx, and 8xx other than conflicts); 4 budget
   (70x and 761); 5 capability (72x, 73x); 6 device (6xx); 7 conflict (803, 804, 809); 8 accepted and still running; 9
   indeterminate; never 1 on purpose.
4. Rows are entered when the cluster is first registered (M1); each row's tool is added to the task-cluster
   tables its `task_clusters` names in the same change (NJL-05's staging rule). A verb whose handler is not in
   the build answers one envelope with `FROB-REF-724`, exit 5, on both surfaces, and `frob capabilities --json`
   lists it as absent; a reduced form that lands before the row's `first_milestone` answers and is listed as
   implemented with its form named (NJL-01, NJL-03: the field keeps the appendix's milestone).
5. Options (DESIGN). An option or action a row lists whose implementing bead is not in the build
   (`task.check`'s `coupling` before N24c-37, `--choose` before N24c-40, `task.settle --cohort` and `--tier`,
   `task.verdict --online`, `task.audit --list-bodies`) is in the input schema and answers one envelope
   `FROB-REF-724`, exit 5, naming the option, never the schema refusal 802; the same holds for `task.fixture`'s
   `control` in a build without the `gauntlet-mutations` feature (N24b-03). A withdrawn option
   answers its worded refusal instead (`coupling-choice-not-built`, `audit-arm-not-built`).

**Notes for this cluster.**

- Eight verbs. A task is data and is content-addressed: `TaskId` is the hash of its canonical form.
- One two-kind verb (DESIGN, round 3). `task.check --action coupling` runs worlds: the four checks on N worlds per
  arm (default 4,096, N24c-37) and, with `--choose`, both arms under each candidate on the pilot addresses
  (N24c-40); N24c-38 waits on its job. It also writes: the table's `coupling` status and, with `--choose`, a lock
  record in the ledger. So it is the action `coupling` of kind `run` (scope `task:run`; idempotency key, budget,
  bounded wait, job handle with exit 8), as `sense.symmetry`'s action `test` is; a repeated key returns the same
  record and lock, and a re-choice is a new family version, so no expected parent is needed. The plan names the verb
  `task.check --coupling` (§24.22), so the mode stays on `task.check`, as its action `coupling`, which the command
  line selects with `--action coupling` like every action (NJL-01's action rule: the plan's `--coupling` would be a
  per-action flag, a second spelling on the CLI only). Rejected: `mutate` for `--choose` (a `mutate` has no budget
  or wait, and the choice spends worlds); a separate verb (outside the plan's inventory of 243, and the plan's own
  spelling is `task.check --coupling`).
- The fixture listing is `task.list` with `kind = "fixture"` (N24bx-01). A CLI spelling that MCP does not have
  is the CLI-only alias that NJL-04 rejects (`surface-alias`) and INV-TETHER-01 forbids, so the row has no
  `--list`, `frob task fixture --list` is the unknown-option refusal 802, and the listing is spelled `frob task
  list --kind fixture`, as N24bx-01 and N16c-41 spell it (DESIGN, round 3).
- `task.check` refuses a task whose atom has no scale, whose signal crosses clocks without a reduction, whose window
  is not a multiple of the requirement period, whose online signal uses a future-time operator, whose horizon is
  shorter than the reach of its verdict, or whose labelled examples are missing or disagree with the monitor.

## Acceptance criteria

- [ ] `registries/verbs.toml` holds exactly eight entries with `cluster = "task"` (4 `read`, 3 `run`, 1 `mutate`),
      each with every field of the entry schema, an example and a task-cluster list; the alias lint finds no second
      spelling.
- [ ] `frob-surface-diff parity --cluster task` exits 0 in process (`frob-mcp` over stdio) and through `frobd
      --mcp`: registry, CLI and MCP each count 8, and every difference list is empty apart from the harness's closed
      exemptions.
- [ ] Every verb has an end-to-end script: `scripts/e2e/verbs_task.sh` runs each verb of the cluster (eight), and
      from a verb's first milestone a capability script of its implementing bead exists (`frob-surface-diff
      e2e-coverage`).
- [ ] `frob <verb> --help --json` equals `fixtures/golden/verbs/task/<verb>.help.json` byte for byte for each of the
      eight, on two runs, debug and release, Linux and macOS; a planted renamed output property is classified `breaking`
      and refused.
- [ ] A verb not yet implemented answers `FROB-REF-724`, exit 5, with one envelope on both surfaces; an implemented
      verb runs its registry example with the result its implementing bead specifies; a misspelt parameter is
      `FROB-REF-802`, exit 3; a `run` or `mutate` call without a key is `FROB-REF-805`, exit 3.
- [ ] The generated refusal table lists for each verb exactly its chapter codes plus the codes appended for its
      kind and fields (NJL-01), and every code exists in `registries/refusals.toml`; a worded refusal named in a
      block enters `refusals` only with its allocated number.
- [ ] `task.check` exposes `action`: `action = "coupling"` (CLI `--action coupling`) resolves to kind `run`,
      requires `task:run`, an idempotency key (without it `FROB-REF-805`, exit 3) and the budget, and returns a job
      handle with exit 8 after the bounded wait; a token holding only `task:read` is refused `FROB-REF-720`, exit 5;
      the default action stays `read` and needs no key; `frob task check <task> --coupling --json` is
      `FROB-REF-802`, exit 3 (no per-action flag).
- [ ] `task.list` is paginated (cursor, limit, follow; 806 and 807 appended) and accepts `kind = "fixture"`;
      `task.fixture` has no `--list` property on either surface.
- [ ] The gate row is recorded with the measured counts by a session that did not write the rows.

## Test plan

### Unit tests

`cargo test -p frob-verbs --test cluster_task` (the real generated table; no mocks; planted registries are TOML
strings labelled `synthetic`):
- `rows_present`: exactly 8 rows in the cluster, with the names of the rows table; a planted extra row or a removed
  row fails.
- `kinds_milestones_tiers`: each row's kind, first milestone and tier equal the rows table (for example `task.audit`
  is `run` at M3).
- `schemas_and_scopes`: input schemas are `frob.<name>.in/1`; output schemas and capabilities equal the
  machine-fields table (`task.verdict` returns `frob.verdict/1`).
- `refusal_lists`: each row's chapter codes equal its block (`task.check`: 103, 442, 520, 523, 525, 526, 528, 535,
  536, 538, 539, 554, 556, 891, and the number the ledger assigned to `randomize-numerical-parameter`, read from
  `registries/refusal_allocations.toml` by slug, in a registry where that request is `assigned` (none is typed in
  the test; while it is unassigned no number for it is in the list; no slug and no unassigned proposal, 571 or
  572, is in any list); the generated `task.check` input schema has the optional `design`, `overlay` and
  `element`; `task.fixture`: 136, 222, 529, 530, 726; `task.list`: none; `task.new`: 103, 442, 520, 523,
  525, 526, 528, 535, 536, 538, 539, 891; `task.settle`: 529; `task.show`: 103; `task.verdict`: 526, 888;
  `task.audit`: none); the appended codes equal the rule for its kind and fields (806 and 807 on
  `task.list`; 891, 734 and 828 on `task.new`, `task.check`, `task.show` and `task.verdict`, each code once, on
  `task.check` from the four `params` entries `task`, `design`, `overlay` and `element`; 808 on every row whose
  generated schema has an `int53` property, every `run` row among them).
- `fixture_inputs`: `task.fixture` accepts the four layers, the eight axis flags and the selectors, and the
  fixture as `fixture:<name>` or the bare name; an axis value outside the axis's domain (`--actuator-level L7`)
  is 802; `control` on a release build is 724; `--list` is an unknown parameter (802); `task.list` accepts `kind
  = "fixture"` and refuses another value with 802.
- `option_rows`: the inputs of the blocks are properties of the generated schemas (`task.settle`: `cohort`,
  `tier`, `refresh`; `task.verdict`: `task` (not required), `requirement` (CLI `--requirement`), `worlds`,
  `re_evaluate`, `online`; `task.show`: `as`, `section`, `design`; `task.audit`: `list_bodies`); `task.settle`
  with both `model` and `cohort` is 802; an option whose implementing bead is absent answers 724 naming it
  (convention 5).
- `two_kind_check`: `task.check` with `action = "coupling"` resolves to kind `run` with scope `task:run`, carries
  the six common `run` properties and the `run` row's `wait`; `arms`, `tier`, `worlds`, `choose` and
  `pilot_worlds` are properties of that action; the default action is `read` with `idempotent = true`; the
  generated CLI selects the action only with `--action coupling` and has no `--coupling` flag (802).
- `mcp_names_one_to_one`: the tool names are the mapped names (`task_check`, `task_fixture`, `task_list`,
  `task_new`, `task_settle`, `task_show`, `task_verdict`, `task_audit`), with no collision against any other
  cluster.
- `task_clusters_and_examples`: every row sits in a task cluster of at most seven tools or is a base tool; each
  example parses against the generated CLI and its command path is the row's own.
- `kind_behaviour`: a `run` row has the six common properties of its kind (`sync_wait_ms`, `idempotency_key`
  with `x-frob-key: "required"` and absent from the schema's `required` array, `budget`, `estimate_only`,
  `on_cancel` with the values `leave` and `cancel_job`, `envelope_scope`) and refuses `sync_wait_ms = 30001`
  with code 802; a `mutate` row has `idempotency_key` and `expected_parent` and refuses a call without the key
  with code 805; a verb
  whose capability is absent returns code 724, exit 5, one envelope.

### End-to-end script and logging

`scripts/e2e/verbs_task.sh` (header: `bead=VERBS-task milestone=M1 labels=tether,verbs,verb:task.*
gates=GATE-TETHER-02 seed=1 needs=cpu,mcp,frobd plants=alias,missing-row,renamed-field`) sources the shared helper
and `scripts/e2e/verbs_lib.sh` and calls `verbs_cluster_conformance task`. Steps: (1) `frob capabilities --json` and
`frob robot-docs --json` list the eight verbs with kind and milestone; (2) `frob-surface-diff parity --cluster task`
over stdio and through `frobd --mcp`, every difference list empty; (3) `frob-surface-diff snapshot --check --cluster
task`; (4) each verb's registry example verbatim, expecting a result (exit 0 or 2), a job handle (exit 8, then `frob
job wait`) or `FROB-REF-724` exactly when the capability report says the verb is absent; (5) for each implemented
verb the same call as an MCP tool, the two envelopes agreeing on `verb`, `ok`, `exit`, `result`, `refusal`,
`evidence`, `bound`, `warnings` and `next_actions`; (6) per verb one call with a misspelt parameter
(`e2e_expect_refusal FROB-REF-802 3`) and, for `run` and `mutate` verbs and for `task check --action
coupling`, one without a key (`e2e_expect_refusal FROB-REF-805 3`; before M2 the coupling call expects
`FROB-REF-724`, exit 5); `frob task list --kind fixture --json` pages the fixtures, and `frob task fixture
--list` and `frob task check t:kitchen-tray --coupling --json` are `e2e_expect_refusal FROB-REF-802 3`; (7) one
envelope per call and no exit code 1.

Log: `target/e2e-logs/verbs_task/<utc>.ndjson` with a transcript beside it. Each `cmd` line has the exact command,
exit code, wall time and the envelope's `evidence` block; each `assert` has observed and expected values and the tag
`--gate GATE-TETHER-02`; the metrics `registry_verbs`, `cli_verbs` and `mcp_tools` are logged against 8 and the
difference counts against 0; refusals are logged with code and first repair; a failure logs the first differing verb
and field; the `summary` line gives assertions passed and failed and the gate touched.

### Known answers and negative controls

- Known answers: the cluster has 8 verbs (4 `read`, 3 `run`, 1 `mutate`); the tables above. KA-755 (the exit-code
  table on malformed, unknown, over-budget, unscoped and conflicting inputs gives the exit and refusal code of the
  surface's tables; exact) is exercised for every verb by steps (6) and (7).
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

- Closing rule: the bead closes at M1, when the rows are registered, the surfaces match and every verb passes its
  example or answers the not-implemented refusal. Verbs of this cluster first answer at M1, M3. The per-milestone
  surface beads (BD-137, BD-312) re-run this script as verbs land; that is how GATE-TETHER-02 is re-measured.
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
- Round 3 (settled requests). Applied: S30 (`overlay`, `element` and the `symmetry` block, `task.show --section
  symmetry` and `--design --section reads`, the bare fixture name), S31 (136 and 222; `kind = "fixture"`), S51
  (`--cohort`), S53 (`--online`, `--tier`, `--list-bodies`; `--choose`, `--arms`, `--pilot-worlds`), S58 (554).
  Applied in corrected form: S53's question of kind (the action `coupling` of kind `run`, selected with `--action
  coupling`, not `mutate` and not a new verb); S31's listing (no `task.fixture --list`, see the notes).
  Found in the review and applied: `task.new` carries the static-admission codes BD-122 gives it; 442 (N20a-01,
  N20b-15); `randomize-numerical-parameter` at admission (N23b-03); 891 on `task.check`; `task.list` paginated;
  the `params` entries; the worded refusals of N24-REF on the rows that raise them. Not added:
  `term-contract-unmet`, which N23a-05's admission pass raises at `train.start`, where N23a-34 step 14 asserts
  it; the `task.check` row does not raise it.

## References

Appendix J (J.1 entry form; J.2 inventory rows and J.3 added rows; J.5 task clusters), §24.4, §24.5, §24.7, §24.10,
§24.14, §24.20, §24.22, §32.4, §32.6, §32.12.3, §32.18, §32.21; GATE-TETHER-02; INV-TETHER-01, -02; KA-755.
