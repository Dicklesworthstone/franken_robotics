---
key: "VERBS-foundry"
title: "frob-verbs registry: cluster `foundry`, seven rows, with parity, e2e and schema-snapshot checks"
type: "task"
priority: 2
milestone: "M4"
labels: ["m4", "tether", "foundry", "frob-verbs", "solid", "registry", "verb", "test"]
parent: "WP-62"
deps: ["BD-022", "NJL-01", "NJL-02", "STD-E2E"]
needs: []
related: ["BD-405", "BD-408", "BD-423", "N31a-50", "BD-400", "BD-403", "BD-401", "BD-406", "BD-376", "BD-409", "NJL-03", "NJL-04", "NJL-05"]
gates: ["GATE-TETHER-02"]
plan_refs: ["Appendix J (J.1, J.2, J.3, J.5)", "§31.5", "§31.11", "§31.12", "§31.15", "§31.16", "§32.6", "§32.12.3", "§32.21 (GATE-TETHER-02)"]
---
## Background

FrankenRobotics (a robotics simulation and design system) is reached through the command line `frob`, the daemon
`frobd` and the MCP server `frob-mcp` (the Model Context Protocol, by which an agent harness lists and calls tools).
Every capability is a verb, `cluster.verb`, declared once in `registries/verbs.toml` (crate `frob-verbs`, tier
Solid). The CLI, the daemon protocol, the MCP tools, the schemas, the refusal table and the agent guide are
generated from that file, and gate GATE-TETHER-02 compares the served surfaces with it. The registry holds 243 verbs
in 44 clusters (namespaces).

This bead is the registry of record for the cluster `foundry`. The cluster covers structure and manufacture: load
cases from telemetry, beam-rung sizing, topology optimization with the loads-mass loop, the independent check,
tolerance allocation, the build packet and, as a Moonshot, lifecycle runs. It has seven verbs (2 `read`, 5 `run`);
the handlers live in `frob-foundry`, `frob-lifecycle`. The bead carries every row completely, with the three checks
that keep the rows true: the generated CLI commands and MCP tools match the list; every verb has an end-to-end
script; the schema printed by `frob <verb> --help --json` is stable.

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

In: the seven registry entries of cluster `foundry`; the MCP tool names; the cluster's conformance script; the
help-schema snapshots; each verb's answer before its milestone. Out: behaviour (the implementing beads named below);
the entry schema and lints (NJL-01); the conformance harness (NJL-02); the alias lint (NJL-04); task clusters
(NJL-05, NJL-06); generation of the surfaces (the Tether seed beads).

## Technical approach

**The rows.**

| Verb | Kind | First milestone | Tier | Purpose | Defined in |
|---|---|---|---|---|---|
| `foundry.check` | run | M5 | F | Independent stress, buckling, modal, fatigue and manufacturability check of a geometry | §31.15 |
| `foundry.export` | run | M5 | F | Write geometry and the build packet; `--with-firmware` under its feature | §31.12 |
| `foundry.lifecycle` | run | M8 | M | A lifecycle run under the feature `lifecycle` | §31.15 |
| `foundry.loads` | run | M4 | F | Run or read a load campaign; envelope cases, spectra, the free-body residual | §31.15 |
| `foundry.optimize` | run | M5 | F | Topology optimization of link classes with the loads–mass loop | §31.5 |
| `foundry.size` | read | M4 | F | Beam-rung sizing and the structural margin of A-15 | §31.15 |
| `foundry.tolerance` | read | M5 | F | Sensitivity-driven tolerance allocation, stacks and the band-extreme check | §31.11 |

**Contract of each verb.** Evidence labels used below (the `evidence` block of the envelope):
- *static*: class Tested, tier `static`, determinism `exact` (class D0); `claims` empty unless a certificate is
  attached; `reality: no-claim`.
- *pass-through*: passes through the claim product of the stored objects it reads and adds no claim of its own; the
  read itself is `exact`.
- *screening*: class Estimated: a screening or planning signal that may discard under the ladder's rules and may not
  establish a comparison.
- *run*: class Tested at best, by the claim product of the run; the tier of the run, `provisional` on a fast tier
  until promoted to the referee; the determinism class of the placement (D2 unless stated); `reality: no-claim`.

**`foundry.check`** (MCP tool `foundry_check`; owner `frob-foundry`)
- Inputs: geometry (a link or a build); load cases; material cards.
- Outputs: stress, buckling, modal, fatigue and manufacturability results on an independent mesh; the governing case
  and location. Evidence: *static*.
- Chapter refusals: `FROB-REF-163` (a check whose material card lacks the field or provenance it needs);
  `FROB-REF-261` (an optimized part that fails the independent check).
- Implemented by: BD-405.

**`foundry.export`** (MCP tool `foundry_export`; owner `frob-foundry`)
- Inputs: design; output root; `--with-firmware` (its own feature, Moonshot).
- Outputs: geometry files and the build packet `.fbuild` with its four flags, all false. Evidence: *pass-through*.
- Chapter refusals: `FROB-REF-166` (optimization or packet export for a design with no qualification verdict);
  `FROB-REF-818` (a priced parts list without a pinned, licensed catalogue); `FROB-REF-824` (a STEP export above its
  bounds or failing bit-exact replay); `FROB-REF-827` (a fixed-point obligation refuted or undecided within budget);
  `FROB-REF-860` (an export without the flags, or any attempt to produce a true flag).
- Implemented by: BD-408.

**`foundry.lifecycle`** (MCP tool `foundry_lifecycle`; owner `frob-lifecycle`)
- Inputs: design; duty cycle; lifecycle laws.
- Outputs: a lifecycle run under the feature `lifecycle`; class at most Estimated. Evidence: *screening*.
- Chapter refusals: `FROB-REF-831` (a lifecycle law outside its validity domain, or cited above class Estimated).
- Implemented by: BD-423, N31a-50.

**`foundry.loads`** (MCP tool `foundry_loads`; owner `frob-foundry`)
- Inputs: design; runs or a campaign at telemetry level T2.
- Outputs: envelope load cases (one instant each), load spectra, the free-body residual. Evidence: *run*.
- Chapter refusals: `FROB-REF-167` (a load case that is not one instant, or a campaign failing the free-body check);
  `FROB-REF-169` (telemetry without the traces or samples a load metric needs); `FROB-REF-262` (quasi-static
  superposition where the frequency separation fails).
- Implemented by: BD-400.

**`foundry.optimize`** (MCP tool `foundry_optimize`; owner `frob-foundry`)
- Inputs: design; link classes; load cases; a qualification verdict or `--exploratory`.
- Outputs: optimized link geometry with the loads-mass loop record and convergence figure. Evidence: *screening*.
- Chapter refusals: `FROB-REF-164` (an empty design region, or ports with no load path inside the envelope);
  `FROB-REF-166` (optimization or packet export for a design with no qualification verdict); `FROB-REF-260` (a
  loads–mass loop that diverges, cycles or exceeds its iteration budget).
- Implemented by: BD-403.

**`foundry.size`** (MCP tool `foundry_size`; owner `frob-foundry`)
- Inputs: design; load cases.
- Outputs: beam-rung sizing per link and the structural margin of analysis A-15. Evidence: *static*.
- Chapter refusals: `FROB-REF-163` (a check whose material card lacks the field or provenance it needs).
- Implemented by: BD-401.

**`foundry.tolerance`** (MCP tool `foundry_tolerance`; owner `frob-foundry`)
- Inputs: design; target metric bands.
- Outputs: sensitivity-driven tolerance allocation, stacks, the band-extreme check. Evidence: *static*.
- Chapter refusals: `FROB-REF-165` (an allocation with a zero or unknown sensitivity, or a failed band-extreme
  check).
- Implemented by: BD-406.

**Machine fields of each row.**

| Verb | Capabilities | Output schema | Task clusters | Example |
|---|---|---|---|---|
| `foundry.check` | `foundry:run` | `frob.foundry.check.out/1` | `make`, `structure` | `frob foundry check quad-a --link thigh --key fc-1 --json` |
| `foundry.export` | `foundry:run`, `export:packet` | `frob.foundry.export.out/1` | `make` | `frob foundry export quad-a --out build/ --key fe-1 --json` |
| `foundry.lifecycle` | `foundry:run` | `frob.foundry.lifecycle.out/1` | `structure` | `frob foundry lifecycle quad-a --duty kitchen-shift --key lc-1 --json` |
| `foundry.loads` | `foundry:run` | `frob.foundry.loads.out/1` | `structure` | `frob foundry loads quad-a --campaign c:14 --key fl-1 --json` |
| `foundry.optimize` | `foundry:run` | `frob.foundry.optimize.out/1` | `make` | `frob foundry optimize quad-a --links thigh,shank --key fo-1 --json` |
| `foundry.size` | `foundry:read` | `frob.foundry.size.out/1` | `make`, `structure` | `frob foundry size quad-a --json` |
| `foundry.tolerance` | `foundry:read` | `frob.foundry.tolerance.out/1` | `make`, `structure` | `frob foundry tolerance quad-a --json` |

The remaining fields follow by rule: `input_schema` is `frob.<verb name>.in/1`; `refusals` is the chapter list of
the verb's block; `idempotent` is `true` for `read` and `admin` and `false` for `run` and `mutate` (which take an
idempotency key); `cost_model` is `static: …` for `read`, `mutate` and `admin` and the `--estimate` cost model for
`run`; a `run` row carries `wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }`; `owner` is the
crate of the block; `purpose` is the Purpose column.

One entry in full (the others follow field for field):

```toml
[[verb]]
name = "foundry.check"
cluster = "foundry"
kind = "run"
tier = "F"
capabilities = ["foundry:run"]
input_schema = "frob.foundry.check.in/1"
output_schema = "frob.foundry.check.out/1"
refusals = ["FROB-REF-163", "FROB-REF-261"]
idempotent = false
cost_model = "estimated: `--estimate` returns CPU core-seconds, GPU seconds, memory and disk with a conformal interval"
first_milestone = "M5"
task_clusters = ["make", "structure"]
examples = ["frob foundry check quad-a --link thigh --key fc-1 --json"]
owner = "frob-foundry"
purpose = "Independent stress, buckling, modal, fatigue and manufacturability check of a geometry"
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
4. Rows are entered when the cluster is first registered (M4). Until its own milestone a verb answers one envelope
   with `FROB-REF-724`, exit 5, on both surfaces, and `frob capabilities --json` lists it as absent.

**Notes for this cluster.**

- Seven verbs, the limit for one registry cluster of this chapter.
- Refusal codes. The chapter's verb table prints chapter-local numbers that predate the refusal registry (160 to
  165, 260 to 262, 860 to 864). The registry codes used here are those of the chapter's refusal table, matched by
  meaning: 160 → `FROB-REF-166`, 161 → 167, 162 → 169, 163 → 163, 164 → 164, 165 → 165, 260 → 260, 261 → 261, 262 →
  262, 860 → 860, 861 → 818, 862 → 824, 863 → 827, 864 → 831.
- Every exported artifact carries the no-authorization flags, all false, and a build packet adds a fourth,
  `fabrication_released: false`; an export without the flags, or any attempt to produce a true flag, is refused
  (`FROB-REF-860`).
- Milestones as the inventory prints them: `foundry.loads` and `foundry.size` M4, the rest M5, `foundry.lifecycle`
  M8. The seed beads place `foundry.size` and `foundry.loads` at M5 (BD-401, BD-400) and the lifecycle epic at M6
  (BD-423); until a verb's bead lands it answers the not-implemented refusal.

## Acceptance criteria

- [ ] `registries/verbs.toml` holds exactly seven entries with `cluster = "foundry"` (2 `read`, 5 `run`), each with
      every field of the entry schema, an example and a task-cluster list; the alias lint finds no second spelling.
- [ ] `frob-surface-diff parity --cluster foundry` exits 0 in process (`frob-mcp` over stdio) and through `frobd
      --mcp`: registry, CLI and MCP each count 7, and every difference list is empty apart from the harness's closed
      exemptions.
- [ ] Every verb has an end-to-end script: `scripts/e2e/verbs_foundry.sh` runs each verb of the cluster (seven), and
      from a verb's first milestone a capability script of its implementing bead exists (`frob-surface-diff
      e2e-coverage`).
- [ ] `frob <verb> --help --json` equals `fixtures/golden/verbs/foundry/<verb>.help.json` byte for byte for each of
      the seven, on two runs, debug and release, Linux and macOS; a planted renamed output property is classified
      `breaking` and refused.
- [ ] A verb not yet implemented answers `FROB-REF-724`, exit 5, with one envelope on both surfaces; an implemented
      verb runs its registry example with the result its implementing bead specifies; a misspelt parameter is
      `FROB-REF-802`, exit 3; a `run` or `mutate` call without a key is `FROB-REF-805`, exit 3.
- [ ] The generated refusal table lists for each verb exactly its chapter codes plus the codes appended for its
      kind, and every code exists in `registries/refusals.toml`.
- [ ] The gate row is recorded with the measured counts by a session that did not write the rows.

## Test plan

### Unit tests

`cargo test -p frob-verbs --test cluster_foundry` (the real generated table; no mocks; planted registries are TOML
strings labelled `synthetic`):
- `rows_present`: exactly 7 rows in the cluster, with the names of the rows table; a planted extra row or a removed
  row fails.
- `kinds_milestones_tiers`: each row's kind, first milestone and tier equal the rows table (for example
  `foundry.tolerance` is `read` at M5).
- `schemas_and_scopes`: input schemas are `frob.<name>.in/1`; output schemas and capabilities equal the
  machine-fields table.
- `refusal_lists`: each row's chapter codes equal its block (`foundry.check`: 163, 261; `foundry.export`: 166, 818,
  824, 827, 860; `foundry.lifecycle`: 831); the appended codes equal the rule for its kind.
- `mcp_names_one_to_one`: the tool names are the mapped names (`foundry_check`, `foundry_export`,
  `foundry_lifecycle`), with no collision against any other cluster.
- `task_clusters_and_examples`: every row sits in a task cluster of at most seven tools or is a base tool; each
  example parses against the generated CLI and its command path is the row's own.
- `kind_behaviour`: a `run` row has the five common properties and refuses `sync_wait_ms = 30001` with code 802; a
  verb whose capability is absent returns code 724, exit 5, one envelope.

### End-to-end script and logging

`scripts/e2e/verbs_foundry.sh` (header: `bead=VERBS-foundry milestone=M4 labels=tether,verbs,verb:foundry.*
gates=GATE-TETHER-02 seed=1 needs=cpu,mcp,frobd plants=alias,missing-row,renamed-field`) sources the shared helper
and `scripts/e2e/verbs_lib.sh` and calls `verbs_cluster_conformance foundry`. Steps: (1) `frob capabilities --json`
and `frob robot-docs --json` list the seven verbs with kind and milestone; (2) `frob-surface-diff parity --cluster
foundry` over stdio and through `frobd --mcp`, every difference list empty; (3) `frob-surface-diff snapshot --check
--cluster foundry`; (4) each verb's registry example verbatim, expecting a result (exit 0 or 2), a job handle (exit
8, then `frob job wait`) or `FROB-REF-724` exactly when the capability report says the verb is absent; (5) for each
implemented verb the same call as an MCP tool, the two envelopes agreeing on `verb`, `ok`, `exit`, `result`,
`refusal`, `evidence`, `bound`, `warnings` and `next_actions`; (6) per verb one call with a misspelt parameter
(`e2e_expect_refusal FROB-REF-802 3`) and, for `run` and `mutate` verbs, one without a key (`e2e_expect_refusal
FROB-REF-805 3`); (7) one envelope per call and no exit code 1.

Log: `target/e2e-logs/verbs_foundry/<utc>.ndjson` with a transcript beside it. Each `cmd` line has the exact
command, exit code, wall time and the envelope's `evidence` block; each `assert` has observed and expected values
and the tag `--gate GATE-TETHER-02`; the metrics `registry_verbs`, `cli_verbs` and `mcp_tools` are logged against 7
and the difference counts against 0; refusals are logged with code and first repair; a failure logs the first
differing verb and field; the `summary` line gives assertions passed and failed and the gate touched.

### Known answers and negative controls

- Known answers: the cluster has 7 verbs (2 `read`, 5 `run`); the tables above. KA-755 (the exit-code table on
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

- Optimization or packet export for a design with no qualification verdict is refused (`FROB-REF-166`) unless
  `--exploratory` is given, and an exploratory result cannot enter a packet.
- `foundry.export --with-firmware` is a Moonshot option under its own feature; it is an option of the verb, not a
  separate row.
- Closing rule: the bead closes at M4, when the rows are registered, the surfaces match and every verb passes its
  example or answers the not-implemented refusal. Verbs of this cluster first answer at M4, M5, M8. The
  per-milestone surface beads (BD-376, BD-409) re-run this script as verbs land; that is how GATE-TETHER-02 is
  re-measured.
- Scope names are proposals (`<cluster>:<read|run|write|admin>` plus the named scopes of the surface) until the
  capability bead BD-258 fixes the scope registry; a `run` or `write` scope of a cluster implies its `read` scope.
  Inputs and outputs above name contents, not flag spellings, except where a flag is quoted; the implementing bead
  fixes the spellings and the snapshot then pins them (additive changes keep the schema's major version; anything
  else raises it).

## References

Appendix J (J.1 entry form; J.2 inventory rows; J.5 task clusters), §31.5, §31.11, §31.12, §31.15, §31.16, §32.4,
§32.6, §32.12.3, §32.18, §32.21; GATE-TETHER-02; INV-TETHER-01, -02; KA-755.
