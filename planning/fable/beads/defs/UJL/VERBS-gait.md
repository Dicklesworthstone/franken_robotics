---
key: "VERBS-gait"
title: "frob-verbs registry: cluster `gait`, four rows, with parity, e2e and schema-snapshot checks"
type: "task"
priority: 2
milestone: "M4"
labels: ["m4", "tether", "reflex", "frob-verbs", "solid", "registry", "verb", "test"]
parent: "WP-62"
deps: ["BD-022", "NJL-01", "NJL-02", "NJL-04", "NJL-05", "STD-E2E"]
needs: []
related: ["N20-REF", "N22-REF", "N25-REF", "N25d-02", "N33c-32"]
gates: ["GATE-TETHER-02"]
plan_refs: ["Appendix J (J.1, J.2, J.3, J.5)", "§22.9", "§22.9.5", "§22.9.6", "§32.6", "§32.12.3", "§32.21 (GATE-TETHER-02)"]
---
## Background

FrankenRobotics (a robotics simulation and design system) is reached through the command line `frob`, the daemon
`frobd` and the MCP server `frob-mcp` (the Model Context Protocol, by which an agent harness lists and calls tools).
Every capability is a verb, `cluster.verb`, declared once in `registries/verbs.toml` (crate `frob-verbs`, tier
Solid). The CLI, the daemon protocol, the MCP tools, the schemas, the refusal table and the agent guide are
generated from that file, and gate GATE-TETHER-02 compares the served surfaces with it. The registry holds 244 verbs
in 44 clusters (namespaces): the 243 of the specification and `artifact.import` (VERBS-artifact); the
count of record is NJL-03's.

This bead is the registry of record for the cluster `gait`. The cluster covers gaits as hybrid periodic orbits: the
atlas of gaits a body can have, an orbit solved by shooting with its Floquet multipliers, continuation of an orbit
in a design parameter, and design gradients of an orbit. It has four verbs (1 `read`, 3 `run`); the handlers live in
`frob-gait`. The bead carries every row completely, with the three checks that keep the rows true: the generated CLI
commands and MCP tools match the list; every verb has an end-to-end script; the schema printed by `frob <verb>
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

In: the four registry entries of cluster `gait`; the MCP tool names; the cluster's conformance script; the
help-schema snapshots; each verb's answer before its milestone. Out: behaviour (the implementing beads named below);
the entry schema and lints (NJL-01); the conformance harness (NJL-02); the alias lint (NJL-04); task clusters
(NJL-05, NJL-06); generation of the surfaces (the Tether seed beads).

## Technical approach

**The rows.**

| Verb | Kind | First milestone | Tier | Purpose | Defined in |
|---|---|---|---|---|---|
| `gait.atlas` | read | M4 | F | Enumeration of gaits as (H, K) pairs with phase lags (analysis A-04) | §22.9 |
| `gait.continue` | run | M4 | F | Pseudo-arclength continuation of an orbit in a design parameter | §22.9.5 |
| `gait.gradient` | run | M4 | F | Design gradient of an orbit and its metrics, with derivative labels | §22.9.6 |
| `gait.orbit` | run | M4 | F | Solve a periodic orbit by shooting; multipliers, residual, standing `screening` | §22.9 |

**Contract of each verb.** Evidence labels used below (the `evidence` block of the envelope):
- *static*: class Tested, tier `static`, determinism `exact` (class D0); `claims` empty unless a certificate is
  attached; `reality: no-claim`.
- *screening*: class Estimated: a screening or planning signal that may discard under the ladder's rules and may not
  establish a comparison.

**`gait.atlas`** (MCP tool `gait_atlas`; owner `frob-gait`)
- Inputs: design; optional `--task`, `--command` (default `forward`), `--network` (an enumeration generated from
  `registries/gait-networks.toml`; an unknown name fails the schema, `FROB-REF-802`), `--timing` (the max-plus
  timing constants).
- Outputs: gaits as (H, K) subgroup pairs with phase lags and max-plus timing (analysis A-04); exit 2 when the body
  group is `unbound`. Evidence: *static* (class `Certified` for the enumeration when its theorem-card certificate is
  attached and accepted, `Tested` otherwise).
- Chapter refusals: `FROB-REF-214` (a representation with a constituent of quaternionic type, or a symmetry group
  outside the supported families; also a network whose legs do not match the design's leg ports).
- Implemented by: BD-332.

**`gait.continue`** (MCP tool `gait_continue`; owner `frob-gait`)
- Inputs: `--from` a stored orbit; one target: `--to <design>` (joined to the orbit's design by parameter-only
  typed edits), `--param name=a..b` (a design parameter and its range), or `--children <design set>` (orbit
  screening of a set of child designs); optional `--through` (record stability-changing points and continue),
  `--spectrum-every k`, `--form stroboscopic|section|bordered` (default the form of the `--from` orbit; `section`
  and `bordered` are served by N22c-36 and N22c-37); the operator-only `--workers <n>` of BD-333's grammar
  (`batch-cpu` only, an integer 1 to 512, outside every identity; on another tier `FROB-REF-802`; BD-335); the
  idempotency key `--key`.
- Outputs: per target one typed outcome of the continuation (`arrived`, `fold`, `period-doubling`, `torus`,
  `symmetry-breaking`, `mode-change`, `diverged`, `budget`, `not-continuable: discrete-edit`) with its payload
  (parameter value of the event, broken generator, old and new contact sequences, offered seed), the path record's
  identity, multipliers along the path, and, only when `arrived`, the target's `GaitOrbit` record with
  `parent_orbit` set; with `--children`, the screening table (one row per child, with the rank-agreement mode in
  force for the rung pair in its header). Exit 0 for `arrived`, 2 for any other typed outcome (a fold is an answer,
  not an error). Evidence: *screening* (standing `screening: gait-orbit`).
- Chapter refusals: `FROB-REF-431` (a gait derivative was requested at grazing or at a fold: the orbit sits at
  grazing and no selection can be followed).
- Pending, uncoded (enters the list with the number the allocation ledger assigns to its slug, the row of N35x-02
  in family `4xx` numbered by N35-33's rule, once N22-REF has entered that number in `registries/refusals.toml`;
  emitted meanwhile under its name, exit 3): `fixed-period-shooting-on-free-period` (fixed-period continuation of
  a loop whose controller declares phase source `event-reset` or `none`; first repair: the section or bordered
  form).
- Implemented by: BD-335 (with the continuation algorithm N22a-37). Extended by: N22c-36 (section form), N22c-37
  (bordered form).

**`gait.gradient`** (MCP tool `gait_gradient`; owner `frob-gait`)
- Inputs (BD-334): an orbit identity, or a design, gait and tier from which one is solved; `--params` (design
  parameters, comma-separated; the gate's cosine needs at least three) or `--edit <typed edit>` (the parameters
  the edit changes); `--metrics` (metrics of the orbit, for example `cot`, `torque_margin`); optional `--wrt
  <fixture parameter>` (a continuous key of a registered fixture's `--set` grammar, such as `slope`, `l` or `g`
  of the rimless wheel; a discrete key such as `spokes` is `FROB-REF-802`), `--multipliers` (multiplier
  gradients, from a continuation path through the orbit when one exists), `--state-sensitivity` (the
  fixed-point sensitivities), `--check fd` (re-solve the orbit at θ ± δ and report the cosine), `--tier`, the
  operator-only `--workers <n>` of BD-333's grammar; the idempotency key `--key`.
- Outputs: per metric and parameter the value with units, its derivative label (one of the nine of construction
  C-16: `smooth`, `saltation`, `discrete-step`, `fixed-mode`, `one-sided`, `generalized`, `smoothed`,
  `finite-difference`, `refused`; the label of a gradient is the weakest of its inputs', and a `generalized`
  monodromy gives one gradient per selection), method, step sizes and selection; the check's cosine when run;
  the fold margin; on a refusal at a fold, the one-sided estimates; from M6 (N25d-02) a `symmetry_events` block
  naming every symmetry–event licence the gradient used. Exit 0; 2 when some entries are refused or
  `generalized`. Evidence: *screening*.
- Chapter refusals, divided by condition (BD-334): `FROB-REF-431` (a gait derivative requested at grazing or at a
  fold, wrapping the kernel's 263 when that fired underneath); `FROB-REF-263` (the other conditions: an ambiguous
  event order, frictional degeneracy at an event, with the offending state and directions); and, added to the
  plan's list (DESIGN, raised from M6 by N25d-02), `FROB-REF-852` (a symmetry transport across an event with no
  licence; Augur's code, N25-REF; first repair: the missing finding).
- Implemented by: BD-334. Extended by: N22c-37 (the stride-time derivative through the bordered form), N25d-02
  (M6: `symmetry_events` and 852).
- Correction (round 3): the label list formerly read `ordinary`, `one-sided`, `generalized`; `ordinary` is no
  label of C-16, and the row now lists the nine.

**`gait.orbit`** (MCP tool `gait_orbit`; owner `frob-gait`)
- Inputs: a subject (a design or family identifier, or `fixture:<name>` for a registered fixture); `--set k=v[,…]`
  (fixture or family parameters; there is no `--param` and no per-parameter flag, which fail the schema with
  `FROB-REF-802`); `--gait <atlas id>`, or `--group <H>,<K>,<σ>` with `--stride <N>`; `--controller`; `--tier`;
  tolerances (`--tol`); `--seed`; `--seed-orbit <ref>` (an initial guess from a seed offered by `gait.continue`;
  BD-333); the operator-only `--workers <n>` (`batch-cpu` only, an integer 1 to 512, default from the pool
  configuration, outside every identity and echoed by the receipt, as `sim.run` has it; on another tier or
  outside its domain `FROB-REF-802`; BD-333); `--section <event | auto | post-impact>` (N22c-36); `--form
  stroboscopic|section|bordered` (N22c-36, N22c-37) with `--period <T>` for `stroboscopic` (on a clock-forced loop
  it must equal the clock's stride period N·Δt_c, otherwise `FROB-REF-802`; N22c-36); `--modes <mode set>` with
  `--cycle-length` (stability over admitted stride sequences, N22c-41, N22c-42) and `--task <task>` (a
  terrain-traversal task that supplies the mode worlds; without it the modes come from a registered fixture;
  N22c-41); diagnostics `--check-direct`, `--spectrum-scope restricted`, `--impact-report`; the idempotency key
  `--key`.
- Outputs: a periodic orbit (`GaitOrbit`) with Floquet multipliers and residual, or a typed outcome (`diverged`,
  `budget`); standing `screening: gait-orbit`. Evidence: *screening*.
- Chapter refusals: `FROB-REF-431` (a monodromy assembly at grazing or at a fold); `FROB-REF-461` (a reduced orbit
  solve under a group not contained in Γ_use); `FROB-REF-443` (a requested group element outside Γ_obs, the
  observation-preserving subgroup; checked first, so such a request never reaches 461; Sensorium's code, N20-REF;
  BD-333, N22a-31). The airlock requirement `needs_groups = ["spectral"]` (the
  monodromy spectrum goes to the simulator's Floquet analysis, gate INT-FSIM-13) makes the generator append
  `FROB-REF-790` (a required foundation group absent from this build, exit 5).
- Pending, uncoded (enters the list as on `gait.continue`, with the ledger's number once N22-REF has entered it):
  `fixed-period-shooting-on-free-period` (a stroboscopic solve on an autonomous loop; N22c-36).
- Implemented by: BD-333 (with the symmetric shooting and group admission of N22a-31). Extended by: N22c-36,
  N22c-37, N22c-41 (`--modes`, `--cycle-length`, `--task`); the gate script of the mode set is N22c-42's.
- Correction (round 1): the row formerly listed `FROB-REF-224` (indeterminate contact). BD-333, which registers the
  verb, and the chapter refusal registry N22-REF list 431 (and 461 on `gait.orbit`) and 790; an indeterminate
  contact inside a shooting world surfaces as the typed outcome `diverged` with the world's per-world code, not as
  a refusal of the verb.

**Machine fields of each row.**

| Verb | Capabilities | Output schema | Task clusters | Example |
|---|---|---|---|---|
| `gait.atlas` | `gait:read` | `frob.gait.atlas.out/1` | `gaits`, `symmetry` | `frob gait atlas quad-a --json` |
| `gait.continue` | `gait:run` | `frob.gait.continue.out/1` | `gaits` | `frob gait continue --from orb:3c1 --param thigh_len=0.20..0.22 --key gc-1 --json` |
| `gait.gradient` | `gait:run` | `frob.gait.gradient.out/1` | `gaits` | `frob gait gradient orb:3c1 --params thigh_len,shank_len,body_mass --metrics cot,torque_margin --key gg-1 --json` |
| `gait.orbit` | `gait:run` | `frob.gait.orbit.out/1` | `gaits` | `frob gait orbit quad-a --gait trot --key go-1 --json` |

The remaining fields follow by rule: `input_schema` is `frob.<verb name>.in/1`; `refusals` is the chapter list of
the verb's block; `idempotent` is `true` for `read` and `admin` and `false` for `run` and `mutate` (which take an
idempotency key); `cost_model` is `static: …` for `read`, `mutate` and `admin` and the `--estimate` cost model for
`run`; a `run` row carries `wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }`; `owner` is the
crate of the block; `purpose` is the Purpose column.

One entry in full (the others follow field for field):

```toml
[[verb]]
name = "gait.continue"
cluster = "gait"
kind = "run"
tier = "F"
capabilities = ["gait:run"]
input_schema = "frob.gait.continue.in/1"
output_schema = "frob.gait.continue.out/1"
refusals = ["FROB-REF-431"]
idempotent = false
cost_model = "estimated: `--estimate` returns CPU core-seconds, GPU seconds, memory and disk with a conformal interval; per accepted point (4·n_K + 5) worlds over N/m periods, plus a monodromy when the spectrum is requested at that point"
first_milestone = "M4"
task_clusters = ["gaits"]
examples = ["frob gait continue --from orb:3c1 --param thigh_len=0.20..0.22 --key gc-1 --json", "frob gait continue --from orb:3c1 --to dsn:child --key gc-2 --json", "frob gait continue --from orb:3c1 --children set:edits-17 --key gc-3 --json"]
owner = "frob-gait"
purpose = "Pseudo-arclength continuation of an orbit in a design parameter"
wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }
```

**Conventions the rows follow.**

1. The MCP tool name is the verb name with `.` and `-` replaced by `_`. The output on the wire is one envelope
   `frob.envelope/1` with `result` specialized to the output schema; its `evidence` block is a claim product (the
   weakest class, tier and determinism class among the inputs).
2. The generator (NJL-01) adds the common input properties of the kind. `run`: `sync_wait_ms` (0 to the row's
   `wait.mcp_max_ms`, at most 30,000; MCP default 5,000; the CLI waits to completion), `idempotency_key` (CLI
   `--key`), `budget`, `estimate_only` (CLI `--estimate`), `on_cancel` (`leave`, the default, or `cancel_job`;
   `leave_running` is 802) and `envelope_scope` (CLI `--envelope-scope <gate:…|campaign:…|pretrain:…>`: the
   approved compute-envelope scope the run belongs to; a launcher's token comes first and a citation that
   contradicts it is refused; with neither, nor a scope the verb derives from its own inputs, the run is `adhoc`,
   class E0; N36-20). `mutate`: `idempotency_key` and `expected_parent`. `idempotency_key` is in no JSON-schema
   `required` array: the annotation `x-frob-key: "required"` (`"optional"` on a `key_optional` entry) carries the
   rule, and the admission transaction refuses a missing key with 805 on both surfaces. The generator appends the
   surface-wide refusals: `FROB-REF-720` (scope missing), 723 (token invalid), 724 (not implemented in this build)
   and 802 (schema failure, including an unknown parameter) for every verb; 701 to 704 (budget and admission), 761
   (above the compute envelope in force), 803 (key reused with different content) and 805 (missing key) for `run`;
   725, 803, 804 (stale parent) and 805 for `mutate`. The rest is NJL-01's table of variations driven by fields
   and tables, cited here and not copied: among them 806 and 807 (with `cursor`, `limit`, `follow`) for a
   paginated read; 808 (an integer outside ±(2⁵³ − 1)) for every entry whose input has an `int53` field, whatever
   its kind (every `run` entry has one, `sync_wait_ms`); no 805 for a `key_optional` entry; `estimate_only` on a
   non-`run` entry with `estimable = true`; the property `action` for an entry with `actions`; 790 for a non-empty
   `needs_groups`; 891 to 893 for artifact `params`; 734 and 828 for a parameter that accepts a path to a file
   (`artifact-input<F>`, `artifact-identity<F>`, `foreign-file`); and the later variations 10 to 16 (`override`
   with `override_reason`, `from_proposal`, `machine`, `engine`, `no_memo` for `memo = true`, the reality guard's
   refusals for `reports = true` or an artifact reader, `diagnostic_override` with the codes of the
   `[[admission_check]]` rows), each landing with the bead NJL-01's table names.
3. Exit codes: 0 done; 2 done with findings; 3 refused input (1xx to 5xx, and 8xx other than conflicts); 4 budget
   (70x and 761); 5 capability (72x, 73x); 6 device (6xx); 7 conflict (803, 804, 809); 8 accepted and still running; 9
   indeterminate; never 1 on purpose.
4. Rows are entered when the cluster is first registered (M4); each row's tool is added to the task-cluster
   tables its `task_clusters` names in the same change (NJL-05's staging rule). A verb whose handler is not in
   the build answers one envelope with `FROB-REF-724`, exit 5, on both surfaces, and `frob capabilities --json`
   lists it as absent; a reduced form that lands before the row's `first_milestone` answers and is listed as
   implemented with its form named (NJL-01, NJL-03: the field keeps the appendix's milestone).

**Notes for this cluster.**

- All four verbs are Frontier and their results are screening signals (standing `screening`): they may discard under
  the ladder's rules and may not establish a comparison.
- A derivative requested at grazing or at a fold is refused (`FROB-REF-431`), and one near an ambiguous event order
  or a change of active set is refused with the offending state and directions (`FROB-REF-263`); from M6 a
  gradient that would transport a derivative across a symmetry event with no licence is refused
  (`FROB-REF-852`).
- Codes of other chapters on these rows: 263 (dynamics), 443 (Sensorium, N20-REF) and 852 (Augur, N25-REF); each
  is listed because the implementing bead raises it and a verb must not return a code its row does not list. The
  `--workers` option is operator-only on all three `run` rows, as in `sim.run`: it never enters an identity, so
  the determinism lane across worker counts runs through the binary.

## Acceptance criteria

- [ ] `registries/verbs.toml` holds exactly four entries with `cluster = "gait"` (1 `read`, 3 `run`), each with
      every field of the entry schema, an example and a task-cluster list; the alias lint finds no second spelling.
- [ ] `frob-surface-diff parity --cluster gait` exits 0 in process (`frob-mcp` over stdio) and through `frobd
      --mcp`: registry, CLI and MCP each count 4, and every difference list is empty apart from the harness's closed
      exemptions.
- [ ] Every verb has an end-to-end script: `scripts/e2e/verbs_gait.sh` runs each verb of the cluster (four), and
      from a verb's first milestone a capability script of its implementing bead exists (`frob-surface-diff
      e2e-coverage`).
- [ ] `frob <verb> --help --json` equals `fixtures/golden/verbs/gait/<verb>.help.json` byte for byte for each of the
      four, on two runs, debug and release, Linux and macOS; a planted renamed output property is classified `breaking`
      and refused.
- [ ] A verb not yet implemented answers `FROB-REF-724`, exit 5, with one envelope on both surfaces; an implemented
      verb runs its registry example with the result its implementing bead specifies; a misspelt parameter is
      `FROB-REF-802`, exit 3; a `run` or `mutate` call without a key is `FROB-REF-805`, exit 3.
- [ ] The generated refusal table lists for each verb exactly its chapter codes plus the codes appended for its
      kind and fields (NJL-01), and every code exists in `registries/refusals.toml`.
- [ ] The gate row is recorded with the measured counts by a session that did not write the rows.

## Test plan

### Unit tests

`cargo test -p frob-verbs --test cluster_gait` (the real generated table; no mocks; planted registries are TOML
strings labelled `synthetic`):
- `rows_present`: exactly 4 rows in the cluster, with the names of the rows table; a planted extra row or a removed
  row fails.
- `kinds_milestones_tiers`: each row's kind, first milestone and tier equal the rows table (for example `gait.orbit`
  is `run` at M4).
- `schemas_and_scopes`: input schemas are `frob.<name>.in/1`; output schemas and capabilities equal the
  machine-fields table.
- `refusal_lists`: each row's chapter codes equal its block (`gait.atlas`: 214; `gait.continue`: 431;
  `gait.gradient`: 263, 431, 852; `gait.orbit`: 431, 443, 461); the appended codes equal the rule for its kind and
  fields; a planted 224 on `gait.orbit` fails against N22-REF's raised-by column; each code of another chapter
  (263 of the dynamics chapter, 443 of N20-REF, 852 of N25-REF) is checked against its own registry's row.
- `needs_groups_orbit`: `gait.orbit` has `needs_groups = ["spectral"]` and its generated refusal list contains 790;
  the other three rows have none and never list 790.
- `examples_have_keys`: every `run` example carries `--key`, and none contains an ellipsis.
- `options_by_row`: the generated input schemas accept the options of each block (`gait.orbit`: `--set`, `--gait`,
  `--group`, `--stride`, `--controller`, `--tier`, `--tol`, `--seed`, `--seed-orbit`, `--workers`, `--section`,
  `--form`, `--period`, `--modes`, `--cycle-length`, `--task`, `--check-direct`, `--spectrum-scope`,
  `--impact-report`; `gait.gradient`: `--params`, `--edit`, `--metrics`, `--wrt`, `--multipliers`,
  `--state-sensitivity`, `--check`, `--tier`, `--workers`; `gait.continue`: `--from`, `--to`, `--param`,
  `--children`, `--through`, `--spectrum-every`, `--form`, `--workers`; `gait.atlas`: `--task`, `--command`,
  `--network`, `--timing`); `--param` on `gait.orbit` and `--workers 4 --tier gpu-cuda` are 802.
- `gradient_labels_are_c16`: the label enum of `frob.gait.gradient.out/1` is exactly the nine labels of C-16; a
  planted `ordinary` fails.
- `mcp_names_one_to_one`: the tool names are the mapped names (`gait_atlas`, `gait_continue`, `gait_gradient`,
  `gait_orbit`), with no collision against any other cluster.
- `task_clusters_and_examples`: every row sits in a task cluster of at most seven tools or is a base tool; each
  example parses against the generated CLI and its command path is the row's own.
- `kind_behaviour`: a `run` row has the six common properties of its kind (`sync_wait_ms`, `idempotency_key`,
  `budget`, `estimate_only`, `on_cancel`, `envelope_scope`) and refuses `sync_wait_ms = 30001` with code 802; a
  verb whose capability is absent returns code 724, exit 5, one envelope.

### End-to-end script and logging

`scripts/e2e/verbs_gait.sh` (header: `bead=VERBS-gait milestone=M4 labels=tether,verbs,verb:gait.*
gates=GATE-TETHER-02 seed=1 needs=cpu,mcp,frobd plants=alias,missing-row,renamed-field`) sources the shared helper
and `scripts/e2e/verbs_lib.sh` and calls `verbs_cluster_conformance gait`. Steps: (1) `frob capabilities --json` and
`frob robot-docs --json` list the four verbs with kind and milestone; (2) `frob-surface-diff parity --cluster gait`
over stdio and through `frobd --mcp`, every difference list empty; (3) `frob-surface-diff snapshot --check --cluster
gait`; (4) each verb's registry example verbatim, expecting a result (exit 0 or 2), a job handle (exit 8, then `frob
job wait`) or `FROB-REF-724` exactly when the capability report says the verb is absent; (5) for each implemented
verb the same call as an MCP tool, the two envelopes agreeing on `verb`, `ok`, `exit`, `result`, `refusal`,
`evidence`, `bound`, `warnings` and `next_actions`; (6) per verb one call with a misspelt parameter
(`e2e_expect_refusal FROB-REF-802 3`) and, for `run` and `mutate` verbs, one without a key (`e2e_expect_refusal
FROB-REF-805 3`); (7) one envelope per call and no exit code 1.

Log: `target/e2e-logs/verbs_gait/<utc>.ndjson` with a transcript beside it. Each `cmd` line has the exact command,
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

- Kill gate KILL-GAIT (the gait gate on orbit screening fails): fallback, orbits serve stability diagnostics and
  warm starts only and no design is ranked by an orbit; the rows stay registered.
- Closing rule: the bead closes at M4, when the rows are registered, the surfaces match and every verb passes its
  example or answers the not-implemented refusal. The per-milestone surface beads (BD-376) re-run this script as
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

Appendix J (J.1 entry form; J.2 inventory rows; J.5 task clusters), §22.9, §22.9.5, §22.9.6, §32.4, §32.6, §32.12.3,
§32.18, §32.21; GATE-TETHER-02; INV-TETHER-01, -02; KA-755.
