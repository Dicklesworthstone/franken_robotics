---
key: "VERBS-autopsy"
title: "frob-verbs registry: cluster `autopsy`, five rows, with parity, e2e and schema-snapshot checks"
type: "task"
priority: 2
milestone: "M3"
labels: ["m3", "tether", "autopsy", "frob-verbs", "solid", "registry", "verb", "test"]
parent: "WP-26"
deps: ["BD-022", "NJL-01", "NJL-02", "STD-E2E"]
needs: []
related: ["BD-280", "BD-279", "N27b-14", "BD-284", "BD-283", "N27b-16", "N27b-09", "BD-312", "NJL-03", "NJL-04", "NJL-05"]
gates: ["GATE-TETHER-02"]
plan_refs: ["Appendix J (J.1, J.2, J.3, J.5)", "§27.5", "§27.6", "§27.7", "§27.8", "§27.12", "§27.14", "§32.6", "§32.12.3", "§32.21 (GATE-TETHER-02)"]
---
## Background

FrankenRobotics (a robotics simulation and design system) is reached through the command line `frob`, the daemon
`frobd` and the MCP server `frob-mcp` (the Model Context Protocol, by which an agent harness lists and calls tools).
Every capability is a verb, `cluster.verb`, declared once in `registries/verbs.toml` (crate `frob-verbs`, tier
Solid). The CLI, the daemon protocol, the MCP tools, the schemas, the refusal table and the agent guide are
generated from that file, and gate GATE-TETHER-02 compares the served surfaces with it. The registry holds 243 verbs
in 44 clusters (namespaces).

This bead is the registry of record for the cluster `autopsy`. The cluster covers counterfactual debugging:
interventions on a fork of a failed run, placebo forks, interventional contrasts and minimal repair sets. It answers
which change rescues a failing run, as a cause in the model. It has five verbs (2 `read`, 3 `run`); the handlers
live in `frob-autopsy`. The bead carries every row completely, with the three checks that keep the rows true: the
generated CLI commands and MCP tools match the list; every verb has an end-to-end script; the schema printed by
`frob <verb> --help --json` is stable.

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

In: the five registry entries of cluster `autopsy`; the MCP tool names; the cluster's conformance script; the
help-schema snapshots; each verb's answer before its milestone. Out: behaviour (the implementing beads named below);
the entry schema and lints (NJL-01); the conformance harness (NJL-02); the alias lint (NJL-04); task clusters
(NJL-05, NJL-06); generation of the surfaces (the Tether seed beads).

## Technical approach

**The rows.**

| Verb | Kind | First milestone | Tier | Purpose | Defined in |
|---|---|---|---|---|---|
| `autopsy.attribute` | read | M3 | S | Contrasts, flip rates, pairs, labels and an optional Shapley summary for a fork | §27.6, §27.7 |
| `autopsy.intervene` | run | M3 | S | Run interventions, placebo forks, replicates and coalitions from a fork handle made by `run.fork` | §27.5 |
| `autopsy.repair` | run | M3 | S | Minimal repair sets, by shrinking the intervention set | §27.7 |
| `autopsy.rerun` | run | M3 | S | Paired re-run from the episode start for design-parameter and structural edits | §27.8 |
| `autopsy.show` | read | M3 | S | The transcript of an attribution session, token-budgeted | §27.12 |

**Contract of each verb.** Evidence labels used below (the `evidence` block of the envelope):
- *model cause*: class Estimated, with scope "a cause in the model at this tier, among the listed factors"; never an
  established cause and never a cause in the physical robot.
- *run*: class Tested at best, by the claim product of the run; the tier of the run, `provisional` on a fast tier
  until promoted to the referee; the determinism class of the placement (D2 unless stated); `reality: no-claim`.
- *study*: a Design comparison of class Tested (Estimated if any input is `assumed`); tier and determinism the
  weakest of the arms; `provisional` until promoted to the referee.
- *pass-through*: passes through the claim product of the stored objects it reads and adds no claim of its own; the
  read itself is `exact`.

**`autopsy.attribute`** (MCP tool `autopsy_attribute`; owner `frob-autopsy`)
- Inputs: fork handle; optional `--shapley` budget.
- Outputs: `frob.autopsy.attribution/1`: contrasts, flip rates at two perturbation scales, pairs with interaction,
  labels (`alternative`, `necessary`, …), placebo rate, refused interventions; optional Shapley summary. Evidence:
  *model cause*.
- Chapter refusals: `FROB-REF-562` (attribution or a counterfactual fork was requested below class D2);
  `FROB-REF-584` (attribution requested for a failure that neither lane admits, or under an uncontrolled determinism
  class); `FROB-REF-880` (an attribution cited as an established cause, or as a cause in the physical robot).
- Implemented by: BD-280.

**`autopsy.intervene`** (MCP tool `autopsy_intervene`; owner `frob-autopsy`)
- Inputs: fork handle from `run.fork`; `do` list (one declared mechanism each); placebo count; replicates;
  coalitions.
- Outputs: forks run, by kind; refused interventions with the earliest valid tick; the null-replay result. Evidence:
  *run*.
- Chapter refusals: `FROB-REF-562` (attribution or a counterfactual fork was requested below class D2);
  `FROB-REF-580` (a continuation whose physical, clock, sensor or policy contract does not match the checkpoint);
  `FROB-REF-581` (a structural or design-parameter intervention requested at a mid-episode tick; the earliest valid
  tick and the forcing fields are returned); `FROB-REF-582` (state from one model grafted onto another);
  `FROB-REF-583` (the null fork does not reproduce its parent); `FROB-REF-586` (an intervention changes more than
  one declared mechanism).
- Implemented by: BD-279.

**`autopsy.repair`** (MCP tool `autopsy_repair`; owner `frob-autopsy`)
- Inputs: fork handle; candidate intervention set.
- Outputs: 1-minimal repair sets, each with rescue counts; diagnostic factors kept apart from repairs. Evidence:
  *model cause*.
- Chapter refusals: `FROB-REF-584` (attribution requested for a failure that neither lane admits, or under an
  uncontrolled determinism class); `FROB-REF-586` (an intervention changes more than one declared mechanism).
- Implemented by: N27b-14, BD-284.

**`autopsy.rerun`** (MCP tool `autopsy_rerun`; owner `frob-autopsy`)
- Inputs: run; typed edit (design parameter or structural); worlds (`failing`, `sample:n`).
- Outputs: paired outcomes from the episode start for the edited and original design, on common world addresses.
  Evidence: *study*.
- Chapter refusals: `FROB-REF-581` (a structural or design-parameter intervention requested at a mid-episode tick;
  the earliest valid tick and the forcing fields are returned); `FROB-REF-587` (a design-parameter re-run with a
  policy that cannot act on the edited body without retraining).
- Implemented by: BD-283.

**`autopsy.show`** (MCP tool `autopsy_show`; owner `frob-autopsy`)
- Inputs: fork or session handle; token budget.
- Outputs: the transcript of the attribution session with handles for drill-down. Evidence: *pass-through*.
- Chapter refusals: none of its own.
- Implemented by: BD-284.

**Machine fields of each row.**

| Verb | Capabilities | Output schema | Task clusters | Example |
|---|---|---|---|---|
| `autopsy.attribute` | `autopsy:read` | `frob.autopsy.attribution/1` | `diagnose` | `frob autopsy attribute f:a41c --json` |
| `autopsy.intervene` | `autopsy:run` | `frob.autopsy.intervene.out/1` | `diagnose` | `frob autopsy intervene f:a41c --do 'scene./floor/patch[galley-2]/film := dry' --placebo 64 --replicates 8 --key aut-1 --json` |
| `autopsy.repair` | `autopsy:run` | `frob.autopsy.repair.out/1` | `repair` | `frob autopsy repair f:a41c --key rep-1 --json` |
| `autopsy.rerun` | `autopsy:run` | `frob.autopsy.rerun.out/1` | `repair` | `frob autopsy rerun r:19e2 --edit 'set-param foot.radius 0.030 m' --worlds failing,sample:64 --key rr-1 --json` |
| `autopsy.show` | `autopsy:read` | `frob.autopsy.show.out/1` | `diagnose`, `repair` | `frob autopsy show f:a41c --budget 600 --json` |

The remaining fields follow by rule: `input_schema` is `frob.<verb name>.in/1`; `refusals` is the chapter list of
the verb's block; `idempotent` is `true` for `read` and `admin` and `false` for `run` and `mutate` (which take an
idempotency key); `cost_model` is `static: …` for `read`, `mutate` and `admin` and the `--estimate` cost model for
`run`; a `run` row carries `wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }`; `owner` is the
crate of the block; `purpose` is the Purpose column.

One entry in full (the others follow field for field):

```toml
[[verb]]
name = "autopsy.intervene"
cluster = "autopsy"
kind = "run"
tier = "S"
capabilities = ["autopsy:run"]
input_schema = "frob.autopsy.intervene.in/1"
output_schema = "frob.autopsy.intervene.out/1"
refusals = ["FROB-REF-562", "FROB-REF-580", "FROB-REF-581", "FROB-REF-582", "FROB-REF-583", "FROB-REF-586"]
idempotent = false
cost_model = "estimated: `--estimate` returns CPU core-seconds, GPU seconds, memory and disk with a conformal interval"
first_milestone = "M3"
task_clusters = ["diagnose"]
examples = ["frob autopsy intervene f:a41c --do 'scene./floor/patch[galley-2]/film := dry' --placebo 64 --replicates 8 --key aut-1 --json"]
owner = "frob-autopsy"
purpose = "Run interventions, placebo forks, replicates and coalitions from a fork handle made by `run.fork`"
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
4. Rows are entered when the cluster is first registered (M3). Until its own milestone a verb answers one envelope
   with `FROB-REF-724`, exit 5, on both surfaces, and `frob capabilities --json` lists it as absent.

**Notes for this cluster.**

- Autopsy registers no fork verb and no divergence verb. A fork point is made with `run.fork`, which returns the
  class of a proposed change set (one of five fork classes), the earliest valid tick and the null-replay result; the
  first divergence of two runs is read with `telemetry.divergence`.
- An attribution is class Estimated and is a cause in the model at the tier it ran on. Citing it as an established
  cause, or as a cause in the physical robot, is refused (`FROB-REF-880`).
- Nothing is attributed unless the empty intervention replays its parent bit for bit (INV-AUTOPSY-01; refusal
  `FROB-REF-583`), and no factor is named when the placebo flip rate exceeds 0.2.

## Acceptance criteria

- [ ] `registries/verbs.toml` holds exactly five entries with `cluster = "autopsy"` (2 `read`, 3 `run`), each with
      every field of the entry schema, an example and a task-cluster list; the alias lint finds no second spelling.
- [ ] `frob-surface-diff parity --cluster autopsy` exits 0 in process (`frob-mcp` over stdio) and through `frobd
      --mcp`: registry, CLI and MCP each count 5, and every difference list is empty apart from the harness's closed
      exemptions.
- [ ] Every verb has an end-to-end script: `scripts/e2e/verbs_autopsy.sh` runs each verb of the cluster (five), and
      from a verb's first milestone a capability script of its implementing bead exists (`frob-surface-diff
      e2e-coverage`).
- [ ] `frob <verb> --help --json` equals `fixtures/golden/verbs/autopsy/<verb>.help.json` byte for byte for each of
      the five, on two runs, debug and release, Linux and macOS; a planted renamed output property is classified
      `breaking` and refused.
- [ ] A verb not yet implemented answers `FROB-REF-724`, exit 5, with one envelope on both surfaces; an implemented
      verb runs its registry example with the result its implementing bead specifies; a misspelt parameter is
      `FROB-REF-802`, exit 3; a `run` or `mutate` call without a key is `FROB-REF-805`, exit 3.
- [ ] The generated refusal table lists for each verb exactly its chapter codes plus the codes appended for its
      kind, and every code exists in `registries/refusals.toml`.
- [ ] The gate row is recorded with the measured counts by a session that did not write the rows.

## Test plan

### Unit tests

`cargo test -p frob-verbs --test cluster_autopsy` (the real generated table; no mocks; planted registries are TOML
strings labelled `synthetic`):
- `rows_present`: exactly 5 rows in the cluster, with the names of the rows table; a planted extra row or a removed
  row fails.
- `kinds_milestones_tiers`: each row's kind, first milestone and tier equal the rows table (for example
  `autopsy.show` is `read` at M3).
- `schemas_and_scopes`: input schemas are `frob.<name>.in/1`; output schemas and capabilities equal the
  machine-fields table (`autopsy.attribute` returns `frob.autopsy.attribution/1`).
- `refusal_lists`: each row's chapter codes equal its block (`autopsy.attribute`: 562, 584, 880;
  `autopsy.intervene`: 562, 580, 581, 582, 583, 586; `autopsy.repair`: 584, 586); the appended codes equal the rule
  for its kind.
- `mcp_names_one_to_one`: the tool names are the mapped names (`autopsy_attribute`, `autopsy_intervene`,
  `autopsy_repair`), with no collision against any other cluster.
- `task_clusters_and_examples`: every row sits in a task cluster of at most seven tools or is a base tool; each
  example parses against the generated CLI and its command path is the row's own.
- `kind_behaviour`: a `run` row has the five common properties and refuses `sync_wait_ms = 30001` with code 802; a
  verb whose capability is absent returns code 724, exit 5, one envelope.

### End-to-end script and logging

`scripts/e2e/verbs_autopsy.sh` (header: `bead=VERBS-autopsy milestone=M3 labels=tether,verbs,verb:autopsy.*
gates=GATE-TETHER-02 seed=1 needs=cpu,mcp,frobd plants=alias,missing-row,renamed-field`) sources the shared helper
and `scripts/e2e/verbs_lib.sh` and calls `verbs_cluster_conformance autopsy`. Steps: (1) `frob capabilities --json`
and `frob robot-docs --json` list the five verbs with kind and milestone; (2) `frob-surface-diff parity --cluster
autopsy` over stdio and through `frobd --mcp`, every difference list empty; (3) `frob-surface-diff snapshot --check
--cluster autopsy`; (4) each verb's registry example verbatim, expecting a result (exit 0 or 2), a job handle (exit
8, then `frob job wait`) or `FROB-REF-724` exactly when the capability report says the verb is absent; (5) for each
implemented verb the same call as an MCP tool, the two envelopes agreeing on `verb`, `ok`, `exit`, `result`,
`refusal`, `evidence`, `bound`, `warnings` and `next_actions`; (6) per verb one call with a misspelt parameter
(`e2e_expect_refusal FROB-REF-802 3`) and, for `run` and `mutate` verbs, one without a key (`e2e_expect_refusal
FROB-REF-805 3`); (7) one envelope per call and no exit code 1.

Log: `target/e2e-logs/verbs_autopsy/<utc>.ndjson` with a transcript beside it. Each `cmd` line has the exact
command, exit code, wall time and the envelope's `evidence` block; each `assert` has observed and expected values
and the tag `--gate GATE-TETHER-02`; the metrics `registry_verbs`, `cli_verbs` and `mcp_tools` are logged against 5
and the difference counts against 0; refusals are logged with code and first repair; a failure logs the first
differing verb and field; the `summary` line gives assertions passed and failed and the gate touched.

### Known answers and negative controls

- Known answers: the cluster has 5 verbs (2 `read`, 3 `run`); the tables above. KA-755 (the exit-code table on
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

- Attribution needs determinism class D2 or better (`FROB-REF-562`); on a placement that attains only D3 the
  cluster's `run` verbs refuse and name the placement that would do.
- Closing rule: the bead closes at M3, when the rows are registered, the surfaces match and every verb passes its
  example or answers the not-implemented refusal. The per-milestone surface beads (BD-312) re-run this script as
  verbs land; that is how GATE-TETHER-02 is re-measured.
- Scope names are proposals (`<cluster>:<read|run|write|admin>` plus the named scopes of the surface) until the
  capability bead BD-258 fixes the scope registry; a `run` or `write` scope of a cluster implies its `read` scope.
  Inputs and outputs above name contents, not flag spellings, except where a flag is quoted; the implementing bead
  fixes the spellings and the snapshot then pins them (additive changes keep the schema's major version; anything
  else raises it).

## References

Appendix J (J.1 entry form; J.2 inventory rows; J.5 task clusters), §27.5, §27.6, §27.7, §27.8, §27.12, §27.14,
§32.4, §32.6, §32.12.3, §32.18, §32.21; GATE-TETHER-02; INV-TETHER-01, -02; KA-755.
