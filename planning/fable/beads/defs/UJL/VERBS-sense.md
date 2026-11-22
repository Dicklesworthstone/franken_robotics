---
key: "VERBS-sense"
title: "frob-verbs registry: cluster `sense`, eight rows, with parity, e2e and schema-snapshot checks"
type: "task"
priority: 1
milestone: "M2"
labels: ["m2", "tether", "sensorium", "frob-verbs", "solid", "registry", "verb", "test"]
parent: "WP-43"
deps: ["BD-022", "NJL-01", "NJL-02", "STD-E2E"]
needs: []
related: ["BD-240", "BD-134", "BD-242", "BD-119", "BD-120", "BD-099", "BD-078", "BD-241", "N20b-33", "N20b-17", "BD-222", "BD-312", "NJL-03", "NJL-04", "NJL-05"]
gates: ["GATE-TETHER-02"]
plan_refs: ["Appendix J (J.1, J.2, J.3, J.5)", "§20.4", "§20.11", "§20.12", "§20.13", "§20.14", "§20.15", "§20.18", "§20.24", "§32.6", "§32.12.3", "§32.21 (GATE-TETHER-02)"]
---
## Background

FrankenRobotics (a robotics simulation and design system) is reached through the command line `frob`, the daemon
`frobd` and the MCP server `frob-mcp` (the Model Context Protocol, by which an agent harness lists and calls tools).
Every capability is a verb, `cluster.verb`, declared once in `registries/verbs.toml` (crate `frob-verbs`, tier
Solid). The CLI, the daemon protocol, the MCP tools, the schemas, the refusal table and the agent guide are
generated from that file, and gate GATE-TETHER-02 compares the served surfaces with it. The registry holds 243 verbs
in 44 clusters (namespaces).

This bead is the registry of record for the cluster `sense`. The cluster covers sensing: the sensor profile of a
design and task, the exact boundary schedule of the clock lattice, the causal-policy test, observability along a
route, alias witnesses, the observation-preserving symmetry group, extrinsic calibration and visibility along a
motion. It has eight verbs (3 `read`, 5 `run`); the handlers live in `frob-sense`. The bead carries every row
completely, with the three checks that keep the rows true: the generated CLI commands and MCP tools match the list;
every verb has an end-to-end script; the schema printed by `frob <verb> --help --json` is stable.

Kinds: `read` returns stored or cheaply computed facts and changes nothing; `run` spends compute, takes an
idempotency key and a budget, and after a bounded synchronous wait returns a job handle with exit code 8; `mutate`
changes a named object in the caller's namespace and takes an idempotency key and an expected parent; `admin`
cancels, stops, evicts or deletes and needs a capability scope. Tiers: S Solid, F Frontier (with a pilot and a kill
gate), M Moonshot; `S/F` is a Solid core with Frontier extensions.

DESIGN: a registry bead per cluster exists beside the implementing beads because a row missing from the registry,
spelled twice, or served with different refusal lists on the CLI and over MCP is a surface defect that no mechanism
test catches. Rejected: rows left to the implementing beads alone (nothing checks their union against the
inventory); one bead for all 243 rows (too large to review).

## Scope

In: the eight registry entries of cluster `sense`; the MCP tool names; the cluster's conformance script; the
help-schema snapshots; each verb's answer before its milestone. Out: behaviour (the implementing beads named below);
the entry schema and lints (NJL-01); the conformance harness (NJL-02); the alias lint (NJL-04); task clusters
(NJL-05, NJL-06); generation of the surfaces (the Tether seed beads).

## Technical approach

**The rows.**

| Verb | Kind | First milestone | Tier | Purpose | Defined in |
|---|---|---|---|---|---|
| `sense.alias` | run | M3 | S/F | An alias witness with its class and the priced separating changes, or "none found" | §20.12 |
| `sense.calibrate` | run | M3 | S | Extrinsic corrections with covariance and residual per edge: simulation at M3, imported logs at M5 | §20.14 |
| `sense.causal-test` | run | M2 | S | Pass, or the first decision instant and channel at which causality fails | §20.15 |
| `sense.observability` | run | M3 | S/F | The Gramian spectrum along a route, with named weak directions | §20.11 |
| `sense.profile` | read | M2 | S | The sensor suite, cards, clocks, link models and queue capacities of a design and task | §20.18 |
| `sense.schedule` | read | M2 | S | The exact boundary schedule over one repeat of the clock lattice, with lateness per event | §20.4 |
| `sense.symmetry` | read | M3 | S | Γ_obs, the elements of Γ_mech it excludes and what excludes each | §20.13 |
| `sense.visibility` | run | M3 | S | Visibility of each sensor along a nominal motion, with the robot's own limbs and payload as occluders | §20.24 |

**Contract of each verb.** Evidence labels used below (the `evidence` block of the envelope):
- *certificate*: a Proof claim of class Certified where the checker accepted a certificate, otherwise the class of
  the weakest input (the claim product takes the minimum); tier `static`, determinism `exact`; `reality: no-claim`.
- *screening*: class Estimated: a screening or planning signal that may discard under the ladder's rules and may not
  establish a comparison.
- *static*: class Tested, tier `static`, determinism `exact` (class D0); `claims` empty unless a certificate is
  attached; `reality: no-claim`.
- *pass-through*: passes through the claim product of the stored objects it reads and adds no claim of its own; the
  read itself is `exact`.

**`sense.alias`** (MCP tool `sense_alias`; owner `frob-sense`)
- Inputs: design; task or mission; world family.
- Outputs: an alias witness (two worlds with one observation history and different required actions) with its class
  and the priced separating changes, or `none found`. Evidence: *certificate*.
- Chapter refusals: none of its own.
- Implemented by: BD-240, BD-134.

**`sense.calibrate`** (MCP tool `sense_calibrate`; owner `frob-sense`)
- Inputs: design; calibration data (simulation at M3, imported logs at M5).
- Outputs: extrinsic corrections with covariance, the consistency residual per edge, a fault attribution where the
  graph supports one. Evidence: *screening*.
- Chapter refusals: `FROB-REF-446` (a calibration request exceeds what the graph supports: no anchor, or fault
  localization on a graph without the needed cycles).
- Implemented by: BD-242.

**`sense.causal-test`** (MCP tool `sense_causal_test`; owner `frob-sense`)
- Inputs: design; task; controller or policy.
- Outputs: `pass`, or the first decision instant and channel at which causality fails. Evidence: *static*.
- Chapter refusals: `FROB-REF-442` (an observation is requested from presentation state, or an actor schema names a
  privileged channel).
- Implemented by: BD-119.

**`sense.observability`** (MCP tool `sense_observability`; owner `frob-sense`)
- Inputs: design; task; route.
- Outputs: the Gramian spectrum along the route modulo gauge, with named weak directions. Evidence: *screening*.
- Chapter refusals: none of its own.
- Implemented by: BD-120.

**`sense.profile`** (MCP tool `sense_profile`; owner `frob-sense`)
- Inputs: design; task.
- Outputs: the sensor suite, cards with provenance, clocks, link models and queue capacities. Evidence:
  *pass-through*.
- Chapter refusals: `FROB-REF-126` (a sensor without a parent frame, a rate or a clock); `FROB-REF-440` (the clock
  lattice is not admissible: a period is zero or negative, or the common denominator exceeds the limit).
- Implemented by: BD-099.

**`sense.schedule`** (MCP tool `sense_schedule`; owner `frob-sense`)
- Inputs: design; task.
- Outputs: the exact boundary schedule over one repeat of the clock lattice, with lateness per event. Evidence:
  *static*.
- Chapter refusals: `FROB-REF-440` (the clock lattice is not admissible: a period is zero or negative, or the common
  denominator exceeds the limit).
- Implemented by: BD-078.

**`sense.symmetry`** (MCP tool `sense_symmetry`; owner `frob-sense`)
- Inputs: design; sensor profile; task.
- Outputs: Γ_obs, the elements of Γ_mech it excludes, and the sensor or task that excludes each. Evidence: *static*.
- Chapter refusals: `FROB-REF-443` (a symmetry consumer names a group element outside Γ_obs).
- Implemented by: BD-241.

**`sense.visibility`** (MCP tool `sense_visibility`; owner `frob-sense`)
- Inputs: design; nominal motion; payload.
- Outputs: lead and blind-interval profiles per sensor, with the robot's own limbs and payload as occluders.
  Evidence: *static*.
- Chapter refusals: none of its own.
- Implemented by: N20b-33.

**Machine fields of each row.**

| Verb | Capabilities | Output schema | Task clusters | Example |
|---|---|---|---|---|
| `sense.alias` | `sense:run` | `frob.sense.alias.out/1` | `sensing` | `frob sense alias quad-a --task dock-3tick --key al-1 --json` |
| `sense.calibrate` | `sense:run` | `frob.sense.calibrate.out/1` | `sensing`, `calibrate` | `frob sense calibrate quad-a --data cal-run:3 --key sca-1 --json` |
| `sense.causal-test` | `sense:run` | `frob.sense.causal-test.out/1` | `sensing` | `frob sense causal-test quad-a --task stairs-0.4 --key ct-1 --json` |
| `sense.observability` | `sense:run` | `frob.sense.observability.out/1` | `sensing` | `frob sense observability quad-a --route corridor-1 --key ob-1 --json` |
| `sense.profile` | `sense:read` | `frob.sense.profile.out/1` | `sensing` | `frob sense profile quad-a --task stairs-0.4 --json` |
| `sense.schedule` | `sense:read` | `frob.sense.schedule.out/1` | `sensing` | `frob sense schedule quad-a --task stairs-0.4 --json` |
| `sense.symmetry` | `sense:read` | `frob.sense.symmetry.out/1` | `sensing`, `symmetry` | `frob sense symmetry quad-a --json` |
| `sense.visibility` | `sense:run` | `frob.sense.visibility.out/1` | `audit` | `frob sense visibility quad-tray --motion trot-forward --key vis-1 --json` |

The remaining fields follow by rule: `input_schema` is `frob.<verb name>.in/1`; `refusals` is the chapter list of
the verb's block; `idempotent` is `true` for `read` and `admin` and `false` for `run` and `mutate` (which take an
idempotency key); `cost_model` is `static: …` for `read`, `mutate` and `admin` and the `--estimate` cost model for
`run`; a `run` row carries `wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }`; `owner` is the
crate of the block; `purpose` is the Purpose column.

One entry in full (the others follow field for field):

```toml
[[verb]]
name = "sense.alias"
cluster = "sense"
kind = "run"
tier = "S"
capabilities = ["sense:run"]
input_schema = "frob.sense.alias.in/1"
output_schema = "frob.sense.alias.out/1"
refusals = []
idempotent = false
cost_model = "estimated: `--estimate` returns CPU core-seconds, GPU seconds, memory and disk with a conformal interval"
first_milestone = "M3"
task_clusters = ["sensing"]
examples = ["frob sense alias quad-a --task dock-3tick --key al-1 --json"]
owner = "frob-sense"
purpose = "An alias witness with its class and the priced separating changes, or \"none found\""
wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }
```

**Conventions the rows follow.**

1. The MCP tool name is the verb name with `.` and `-` replaced by `_`. The output on the wire is one envelope
   `frob.envelope/1` with `result` specialized to the output schema; its `evidence` block is a claim product (the
   weakest class, tier and determinism class among the inputs).
2. The generator adds the common input properties of the kind (`run`: `sync_wait_ms` from 0 to 30,000 with MCP
   default 5,000, `idempotency_key`, `budget`, `estimate_only`, `on_cancel`; `mutate`: `idempotency_key`,
   `expected_parent`) and appends the surface-wide refusals: `FROB-REF-720` (scope missing), 723 (token invalid),
   724 (not implemented in this build) and 802 (schema failure, including an unknown parameter) for every verb; 701
   to 704 (budget and admission), 803 (key reused with different content) and 805 (missing key) for `run`; 725, 803,
   804 (stale parent) and 805 for `mutate`; 806 and 807 for paginated reads; 808 for an out-of-range integer.
3. Exit codes: 0 done; 2 done with findings; 3 refused input (1xx to 5xx, and 8xx other than conflicts); 4 budget
   (70x); 5 capability (72x, 73x); 6 device (6xx); 7 conflict (803, 804, 809); 8 accepted and still running; 9
   indeterminate; never 1 on purpose.
4. Rows are entered when the cluster is first registered (M2). Until its own milestone a verb answers one envelope
   with `FROB-REF-724`, exit 5, on both surfaces, and `frob capabilities --json` lists it as absent.

**Notes for this cluster.**

- Eight verbs. `telemetry.stills` and `telemetry.inspect-pixel` are specified with sensing but belong to the
  `telemetry` cluster.
- `sense.alias` returns a witness or "none found"; a standing alias witness for a mission's interface makes training
  and optimization on it refuse (`FROB-REF-445`).
- `sense.calibrate` first answers at M3 in simulation and at M5 on imported logs.

## Acceptance criteria

- [ ] `registries/verbs.toml` holds exactly eight entries with `cluster = "sense"` (3 `read`, 5 `run`), each with
      every field of the entry schema, an example and a task-cluster list; the alias lint finds no second spelling.
- [ ] `frob-surface-diff parity --cluster sense` exits 0 in process (`frob-mcp` over stdio) and through `frobd
      --mcp`: registry, CLI and MCP each count 8, and every difference list is empty apart from the harness's closed
      exemptions.
- [ ] Every verb has an end-to-end script: `scripts/e2e/verbs_sense.sh` runs each verb of the cluster (eight), and
      from a verb's first milestone a capability script of its implementing bead exists (`frob-surface-diff
      e2e-coverage`).
- [ ] `frob <verb> --help --json` equals `fixtures/golden/verbs/sense/<verb>.help.json` byte for byte for each of
      the eight, on two runs, debug and release, Linux and macOS; a planted renamed output property is classified
      `breaking` and refused.
- [ ] A verb not yet implemented answers `FROB-REF-724`, exit 5, with one envelope on both surfaces; an implemented
      verb runs its registry example with the result its implementing bead specifies; a misspelt parameter is
      `FROB-REF-802`, exit 3; a `run` or `mutate` call without a key is `FROB-REF-805`, exit 3.
- [ ] The generated refusal table lists for each verb exactly its chapter codes plus the codes appended for its
      kind, and every code exists in `registries/refusals.toml`.
- [ ] The gate row is recorded with the measured counts by a session that did not write the rows.

## Test plan

### Unit tests

`cargo test -p frob-verbs --test cluster_sense` (the real generated table; no mocks; planted registries are TOML
strings labelled `synthetic`):
- `rows_present`: exactly 8 rows in the cluster, with the names of the rows table; a planted extra row or a removed
  row fails.
- `kinds_milestones_tiers`: each row's kind, first milestone and tier equal the rows table (for example
  `sense.visibility` is `run` at M3).
- `schemas_and_scopes`: input schemas are `frob.<name>.in/1`; output schemas and capabilities equal the
  machine-fields table.
- `refusal_lists`: each row's chapter codes equal its block (`sense.alias`: none; `sense.calibrate`: 446;
  `sense.causal-test`: 442); the appended codes equal the rule for its kind.
- `mcp_names_one_to_one`: the tool names are the mapped names (`sense_alias`, `sense_calibrate`,
  `sense_causal_test`), with no collision against any other cluster.
- `task_clusters_and_examples`: every row sits in a task cluster of at most seven tools or is a base tool; each
  example parses against the generated CLI and its command path is the row's own.
- `kind_behaviour`: a `run` row has the five common properties and refuses `sync_wait_ms = 30001` with code 802; a
  verb whose capability is absent returns code 724, exit 5, one envelope.

### End-to-end script and logging

`scripts/e2e/verbs_sense.sh` (header: `bead=VERBS-sense milestone=M2 labels=tether,verbs,verb:sense.*
gates=GATE-TETHER-02 seed=1 needs=cpu,mcp,frobd plants=alias,missing-row,renamed-field`) sources the shared helper
and `scripts/e2e/verbs_lib.sh` and calls `verbs_cluster_conformance sense`. Steps: (1) `frob capabilities --json`
and `frob robot-docs --json` list the eight verbs with kind and milestone; (2) `frob-surface-diff parity --cluster
sense` over stdio and through `frobd --mcp`, every difference list empty; (3) `frob-surface-diff snapshot --check
--cluster sense`; (4) each verb's registry example verbatim, expecting a result (exit 0 or 2), a job handle (exit 8,
then `frob job wait`) or `FROB-REF-724` exactly when the capability report says the verb is absent; (5) for each
implemented verb the same call as an MCP tool, the two envelopes agreeing on `verb`, `ok`, `exit`, `result`,
`refusal`, `evidence`, `bound`, `warnings` and `next_actions`; (6) per verb one call with a misspelt parameter
(`e2e_expect_refusal FROB-REF-802 3`) and, for `run` and `mutate` verbs, one without a key (`e2e_expect_refusal
FROB-REF-805 3`); (7) one envelope per call and no exit code 1.

Log: `target/e2e-logs/verbs_sense/<utc>.ndjson` with a transcript beside it. Each `cmd` line has the exact command,
exit code, wall time and the envelope's `evidence` block; each `assert` has observed and expected values and the tag
`--gate GATE-TETHER-02`; the metrics `registry_verbs`, `cli_verbs` and `mcp_tools` are logged against 8 and the
difference counts against 0; refusals are logged with code and first repair; a failure logs the first differing verb
and field; the `summary` line gives assertions passed and failed and the gate touched.

### Known answers and negative controls

- Known answers: the cluster has 8 verbs (3 `read`, 5 `run`); the tables above. KA-755 (the exit-code table on
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

- Closing rule: the bead closes at M2, when the rows are registered, the surfaces match and every verb passes its
  example or answers the not-implemented refusal. Verbs of this cluster first answer at M2, M3. The per-milestone
  surface beads (BD-222, BD-312) re-run this script as verbs land; that is how GATE-TETHER-02 is re-measured.
- Scope names are proposals (`<cluster>:<read|run|write|admin>` plus the named scopes of the surface) until the
  capability bead BD-258 fixes the scope registry; a `run` or `write` scope of a cluster implies its `read` scope.
  Inputs and outputs above name contents, not flag spellings, except where a flag is quoted; the implementing bead
  fixes the spellings and the snapshot then pins them (additive changes keep the schema's major version; anything
  else raises it).

## References

Appendix J (J.1 entry form; J.2 inventory rows and J.3 added rows; J.5 task clusters), §20.4, §20.11, §20.12,
§20.13, §20.14, §20.15, §20.18, §20.24, §32.4, §32.6, §32.12.3, §32.18, §32.21; GATE-TETHER-02; INV-TETHER-01, -02;
KA-755.
