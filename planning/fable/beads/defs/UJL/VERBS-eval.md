---
key: "VERBS-eval"
title: "frob-verbs registry: cluster `eval`, ten rows, with parity, e2e and schema-snapshot checks"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "tether", "arena", "frob-verbs", "solid", "registry", "verb", "test"]
parent: "WP-06"
deps: ["BD-022", "NJL-01", "NJL-02", "STD-E2E"]
needs: []
related: ["BD-262", "BD-124", "BD-267", "N24b-15", "BD-259", "N24b-06", "BD-263", "BD-328", "BD-265", "N24c-04", "BD-125", "BD-254", "N24c-30", "BD-137", "BD-222", "BD-312", "BD-376", "NJL-03", "NJL-04", "NJL-05"]
gates: ["GATE-TETHER-02"]
plan_refs: ["Appendix J (J.1, J.2, J.3, J.5)", "§24.8", "§24.12", "§24.13", "§24.21", "§16.8", "§23.16", "§28.6", "§28.10", "§32.6", "§32.12.3", "§32.21 (GATE-TETHER-02)"]
---
## Background

FrankenRobotics (a robotics simulation and design system) is reached through the command line `frob`, the daemon
`frobd` and the MCP server `frob-mcp` (the Model Context Protocol, by which an agent harness lists and calls tools).
Every capability is a verb, `cluster.verb`, declared once in `registries/verbs.toml` (crate `frob-verbs`, tier
Solid). The CLI, the daemon protocol, the MCP tools, the schemas, the refusal table and the agent guide are
generated from that file, and gate GATE-TETHER-02 compares the served surfaces with it. The registry holds 243 verbs
in 44 clusters (namespaces).

This bead is the registry of record for the cluster `eval`. The cluster covers evaluation: running a battery,
scorecards with outcome columns, the pre-analysis of a fixed-n rule, paired comparison, promotion of a fast-tier
result to the referee, the blinded qualification lane and its ledger, exploitation detectors, rank agreement between
rungs, and the item bank. It has ten verbs (4 `read`, 6 `run`); the handlers live in `frob-arena`, `frob-dyn`,
`frob-learn`, `frob-opt`. The bead carries every row completely, with the three checks that keep the rows true: the
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

In: the ten registry entries of cluster `eval`; the MCP tool names; the cluster's conformance script; the
help-schema snapshots; each verb's answer before its milestone. Out: behaviour (the implementing beads named below);
the entry schema and lints (NJL-01); the conformance harness (NJL-02); the alias lint (NJL-04); task clusters
(NJL-05, NJL-06); generation of the surfaces (the Tether seed beads).

## Technical approach

**The rows.**

| Verb | Kind | First milestone | Tier | Purpose | Defined in |
|---|---|---|---|---|---|
| `eval.compare` | run | M3 | S | Plan and submit a paired study on common world addresses; the reachability pre-analysis is `eval.preanalyze` | §28.6 |
| `eval.detect` | run | M3 | S | The exploitation detectors, on one result | §23.16 |
| `eval.ledger` | read | M3 | S | A qualification campaign's index, levels spent and receipts; nothing sealed | §24.13 |
| `eval.preanalyze` | read | M1 | S | Reachable verdicts and power of a fixed-n rule at a given n and level | §24.12 |
| `eval.promote` | run | M3 | S | Apply the promotion rule and the tier correction to a fast-tier result | §16.8 |
| `eval.qualify` | run | M3 | S | Submit a committed request to the blinded lane; returns a receipt and nothing per world | §24.13 |
| `eval.rank-agreement` | run | M4 | S | Measure rank agreement between two rungs in a stratum and write the ledger record | §28.10 |
| `eval.run` | run | M2 | S | Run a battery and record verdicts | §24.8 |
| `eval.scorecard` | read | M3 | S | Metrics by scene class with the outcome columns and two denominators | §24.8 |
| `eval.items` | read | M3 | S | The item bank of a battery and the sampling weights a close comparison used | §24.21 |

**Contract of each verb.** Evidence labels used below (the `evidence` block of the envelope):
- *study*: a Design comparison of class Tested (Estimated if any input is `assumed`); tier and determinism the
  weakest of the arms; `provisional` until promoted to the referee.
- *screening*: class Estimated: a screening or planning signal that may discard under the ladder's rules and may not
  establish a comparison.
- *pass-through*: passes through the claim product of the stored objects it reads and adds no claim of its own; the
  read itself is `exact`.
- *static*: class Tested, tier `static`, determinism `exact` (class D0); `claims` empty unless a certificate is
  attached; `reality: no-claim`.
- *fidelity*: a Fidelity claim (the faster tier tracks the more trusted tier within a frozen envelope); class
  Tested; directional.
- *qualification*: a Design comparison of class Tested at the referee tier, `qualified` only by a receipt whose
  decision is `promoted`; the receipt carries the three no-authorization flags, all false.
- *run*: class Tested at best, by the claim product of the run; the tier of the run, `provisional` on a fast tier
  until promoted to the referee; the determinism class of the placement (D2 unless stated); `reality: no-claim`.

**`eval.compare`** (MCP tool `eval_compare`; owner `frob-opt`)
- Inputs: two designs; task; scenes; level; budget (plans and submits an anytime-valid paired study with defaults).
- Outputs: a job; on completion the e-value, the confidence sequence, pairs used, the verdict per metric. Evidence:
  *study*.
- Chapter refusals: `FROB-REF-533` (paired statistics requested under an uncontrolled determinism class);
  `FROB-REF-542` (a paired study under an uncontrolled determinism class, or with arms in different classes);
  `FROB-REF-543` (an `anytime` study on a metric with no declared span); `FROB-REF-553` (a comparison that mixes
  study types, or re-reads fixed-n data as a sequential study); `FROB-REF-556` (a pairing keyed by event order or by
  a training counter).
- Implemented by: BD-262, BD-124.

**`eval.detect`** (MCP tool `eval_detect`; owner `frob-learn`)
- Inputs: one result (run or study).
- Outputs: the exploitation detectors that fired, with the evidence for each. Evidence: *screening*.
- Chapter refusals: none of its own.
- Implemented by: BD-267.

**`eval.ledger`** (MCP tool `eval_ledger`; owner `frob-arena`)
- Inputs: campaign.
- Outputs: the campaign's index, the levels α_i spent, the receipts; nothing sealed. Evidence: *pass-through*.
- Chapter refusals: `FROB-REF-840` (a session without `qual:admin` asks for sealed confirmation data before close).
- Implemented by: N24b-15, BD-259.

**`eval.preanalyze`** (MCP tool `eval_preanalyze`; owner `frob-arena`)
- Inputs: rule, n, margin, level; optional alternatives.
- Outputs: counts of outcomes by reachable verdict, unreachable verdicts, power at the declared alternatives, the
  pre-analysis hash. Evidence: *static*.
- Chapter refusals: `FROB-REF-532` (a fixed-n study has no pre-analysis, or names a verdict it cannot reach).
- Implemented by: N24b-06, BD-124.

**`eval.promote`** (MCP tool `eval_promote`; owner `frob-dyn`)
- Inputs: a fast-tier result; target tier; `--method` (`paired`, `dr`, `multilevel`, `segment`).
- Outputs: the promoted result with standing `established` or the reason it is not; ρ and the higher-tier-only count
  it replaced. Evidence: *fidelity*.
- Chapter refusals: `FROB-REF-811` (a tier-corrected estimate across identities, with no audited address, or below
  the effective-sample floor); `FROB-REF-820` (label misuse: A result that is not `established` is cited as
  established, or a `model-conditional` ranking is used to authorize spend); `FROB-REF-821` (envelope not frozen:
  Promotion requested for a quantity whose envelope is not hashed in the ledger); `FROB-REF-830` (a
  `model-conditional` result offered for promotion, build or prototype budget).
- Implemented by: BD-263.

**`eval.qualify`** (MCP tool `eval_qualify`; owner `frob-arena`)
- Inputs: design; task; claim; `--dry` for the reachability pre-analysis.
- Outputs: `frob.qualify.receipt/1`: decision, index, level spent, units used, receipt hash; nothing per world.
  Evidence: *qualification*.
- Chapter refusals: `FROB-REF-567` (a hard-world or search-selected world offered as a confirmation sample);
  `FROB-REF-840` (a session without `qual:admin` asks for sealed confirmation data before close); `FROB-REF-841` (a
  confirmation request was not committed first, or differs from its commitment); `FROB-REF-842` (the units the
  request needs at its level exceed its budget; the reachable verdicts are attached); `FROB-REF-843` (the
  candidate's lineage contains data from the sealed family); `FROB-REF-844` (a cluster would be used twice).
- Implemented by: BD-259.

**`eval.rank-agreement`** (MCP tool `eval_rank_agreement`; owner `frob-opt`)
- Inputs: design set; two rungs or a tier pair; stratum.
- Outputs: the rank-agreement record (τ with interval, strata, permitted use per stratum) written to the ledger.
  Evidence: *study*.
- Chapter refusals: `FROB-REF-537` (a rung used to select or discard without a rank-agreement record that meets its
  threshold); `FROB-REF-545` (a rung asked to select in a stratum whose mode is not `selective`); `FROB-REF-548`
  (pooling evidence or calibration across two model identities).
- Implemented by: BD-328, BD-265.

**`eval.run`** (MCP tool `eval_run`; owner `frob-arena`)
- Inputs: design; battery; tier.
- Outputs: verdicts per scenario recorded in the ledger; a scorecard handle. Evidence: *run*.
- Chapter refusals: `FROB-REF-502` (an evaluation reuses a scene from the policy's training manifest (carried from
  §19)); `FROB-REF-529` (no settled-pose cache for this model and task).
- Implemented by: N24c-04, BD-125.

**`eval.scorecard`** (MCP tool `eval_scorecard`; owner `frob-arena`)
- Inputs: design or evaluation.
- Outputs: metrics by scene class with the outcome columns and both denominators. Evidence: *pass-through*.
- Chapter refusals: `FROB-REF-845` (a result is cited as qualified without a promoting receipt); `FROB-REF-865` (a
  summary statistic that omits what it excluded).
- Implemented by: BD-254, N24c-04.

**`eval.items`** (MCP tool `eval_items`; owner `frob-arena`)
- Inputs: battery; optional comparison.
- Outputs: the item bank of the battery and the sampling weights a close comparison used. Evidence: *pass-through*.
- Chapter refusals: none of its own.
- Implemented by: N24c-30.

**Machine fields of each row.**

| Verb | Capabilities | Output schema | Task clusters | Example |
|---|---|---|---|---|
| `eval.compare` | `eval:run` | `frob.eval.compare.out/1` | `studies` | `frob eval compare quad-a@prev quad-a --task stairs-0.4 --key cmp-1 --json` |
| `eval.detect` | `eval:run` | `frob.eval.detect.out/1` | `inherit` | `frob eval detect r:19e2 --key det-1 --json` |
| `eval.ledger` | `eval:read` | `frob.eval.ledger.out/1` | `qualify` | `frob eval ledger q:kitchen-tray/1 --json` |
| `eval.preanalyze` | `eval:read` | `frob.eval.preanalyze.out/1` | `studies` | `frob eval preanalyze --rule paired-clopper-pearson/1 --n 64 --margin 0.05 --level 0.05 --json` |
| `eval.promote` | `eval:run` | `frob.eval.promote.out/1` | `compare`, `fidelity` | `frob eval promote r:19e2 --tier referee --key pro-1 --json` |
| `eval.qualify` | `qual:run` | `frob.qualify.receipt/1` | `qualify` | `frob eval qualify quad-tray --task t:kitchen-tray --claim 'failure probability of delivered at most 0.05' --key qual-1 --json` |
| `eval.rank-agreement` | `eval:run` | `frob.eval.rank-agreement.out/1` | `fidelity`, `inherit`, `ladder` | `frob eval rank-agreement --designs @set:40 --pair tier:gpu,referee --key ra-1 --json` |
| `eval.run` | `eval:run` | `frob.eval.run.out/1` | `learn`, `batteries` | `frob eval run quad-a --battery standard --key ev-1 --json` |
| `eval.scorecard` | `eval:read` | `frob.eval.scorecard.out/1` | `qualify`, `batteries` | `frob eval scorecard quad-a --json` |
| `eval.items` | `eval:read` | `frob.eval.items.out/1` | `audit`, `batteries` | `frob eval items --battery standard --json` |

The remaining fields follow by rule: `input_schema` is `frob.<verb name>.in/1`; `refusals` is the chapter list of
the verb's block; `idempotent` is `true` for `read` and `admin` and `false` for `run` and `mutate` (which take an
idempotency key); `cost_model` is `static: …` for `read`, `mutate` and `admin` and the `--estimate` cost model for
`run`; a `run` row carries `wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }`; `owner` is the
crate of the block; `purpose` is the Purpose column.

One entry in full (the others follow field for field):

```toml
[[verb]]
name = "eval.compare"
cluster = "eval"
kind = "run"
tier = "S"
capabilities = ["eval:run"]
input_schema = "frob.eval.compare.in/1"
output_schema = "frob.eval.compare.out/1"
refusals = ["FROB-REF-533", "FROB-REF-542", "FROB-REF-543", "FROB-REF-553", "FROB-REF-556"]
idempotent = false
cost_model = "estimated: `--estimate` returns CPU core-seconds, GPU seconds, memory and disk with a conformal interval"
first_milestone = "M3"
task_clusters = ["studies"]
examples = ["frob eval compare quad-a@prev quad-a --task stairs-0.4 --key cmp-1 --json"]
owner = "frob-opt"
purpose = "Plan and submit a paired study on common world addresses; the reachability pre-analysis is `eval.preanalyze`"
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
4. Rows are entered when the cluster is first registered (M1). Until its own milestone a verb answers one envelope
   with `FROB-REF-724`, exit 5, on both surfaces, and `frob capabilities --json` lists it as absent.

**Notes for this cluster.**

- Handlers live in four crates: `frob-arena` (run, scorecard, preanalyze, qualify, ledger, items), `frob-opt`
  (compare, rank-agreement), `frob-dyn` (promote) and `frob-learn` (detect).
- `eval.compare` is the one-call form of the two-phase study: it plans and submits an anytime-valid paired study
  with defaults. `eval.preanalyze` is called by `study.plan` for a fixed-n study.
- `eval.qualify` returns a receipt and nothing per world; no scope in a discovery token can read sealed clusters
  before the campaign closes (`FROB-REF-840`). Its `--dry` form commits nothing and spends no level.
- `eval.rank-agreement` is the one registered name for the rank-agreement record, between controller rungs and
  between tier pairs.

## Acceptance criteria

- [ ] `registries/verbs.toml` holds exactly ten entries with `cluster = "eval"` (4 `read`, 6 `run`), each with every
      field of the entry schema, an example and a task-cluster list; the alias lint finds no second spelling.
- [ ] `frob-surface-diff parity --cluster eval` exits 0 in process (`frob-mcp` over stdio) and through `frobd
      --mcp`: registry, CLI and MCP each count 10, and every difference list is empty apart from the harness's closed
      exemptions.
- [ ] Every verb has an end-to-end script: `scripts/e2e/verbs_eval.sh` runs each verb of the cluster (ten), and from
      a verb's first milestone a capability script of its implementing bead exists (`frob-surface-diff e2e-coverage`).
- [ ] `frob <verb> --help --json` equals `fixtures/golden/verbs/eval/<verb>.help.json` byte for byte for each of the
      ten, on two runs, debug and release, Linux and macOS; a planted renamed output property is classified `breaking`
      and refused.
- [ ] A verb not yet implemented answers `FROB-REF-724`, exit 5, with one envelope on both surfaces; an implemented
      verb runs its registry example with the result its implementing bead specifies; a misspelt parameter is
      `FROB-REF-802`, exit 3; a `run` or `mutate` call without a key is `FROB-REF-805`, exit 3.
- [ ] The generated refusal table lists for each verb exactly its chapter codes plus the codes appended for its
      kind, and every code exists in `registries/refusals.toml`.
- [ ] The gate row is recorded with the measured counts by a session that did not write the rows.

## Test plan

### Unit tests

`cargo test -p frob-verbs --test cluster_eval` (the real generated table; no mocks; planted registries are TOML
strings labelled `synthetic`):
- `rows_present`: exactly 10 rows in the cluster, with the names of the rows table; a planted extra row or a removed
  row fails.
- `kinds_milestones_tiers`: each row's kind, first milestone and tier equal the rows table (for example `eval.items`
  is `read` at M3).
- `schemas_and_scopes`: input schemas are `frob.<name>.in/1`; output schemas and capabilities equal the
  machine-fields table (`eval.qualify` returns `frob.qualify.receipt/1`).
- `refusal_lists`: each row's chapter codes equal its block (`eval.compare`: 533, 542, 543, 553, 556; `eval.detect`:
  none; `eval.ledger`: 840); the appended codes equal the rule for its kind.
- `mcp_names_one_to_one`: the tool names are the mapped names (`eval_compare`, `eval_detect`, `eval_ledger`), with
  no collision against any other cluster.
- `task_clusters_and_examples`: every row sits in a task cluster of at most seven tools or is a base tool; each
  example parses against the generated CLI and its command path is the row's own.
- `kind_behaviour`: a `run` row has the five common properties and refuses `sync_wait_ms = 30001` with code 802; a
  verb whose capability is absent returns code 724, exit 5, one envelope.

### End-to-end script and logging

`scripts/e2e/verbs_eval.sh` (header: `bead=VERBS-eval milestone=M1 labels=tether,verbs,verb:eval.*
gates=GATE-TETHER-02 seed=1 needs=cpu,mcp,frobd plants=alias,missing-row,renamed-field`) sources the shared helper
and `scripts/e2e/verbs_lib.sh` and calls `verbs_cluster_conformance eval`. Steps: (1) `frob capabilities --json` and
`frob robot-docs --json` list the ten verbs with kind and milestone; (2) `frob-surface-diff parity --cluster eval`
over stdio and through `frobd --mcp`, every difference list empty; (3) `frob-surface-diff snapshot --check --cluster
eval`; (4) each verb's registry example verbatim, expecting a result (exit 0 or 2), a job handle (exit 8, then `frob
job wait`) or `FROB-REF-724` exactly when the capability report says the verb is absent; (5) for each implemented
verb the same call as an MCP tool, the two envelopes agreeing on `verb`, `ok`, `exit`, `result`, `refusal`,
`evidence`, `bound`, `warnings` and `next_actions`; (6) per verb one call with a misspelt parameter
(`e2e_expect_refusal FROB-REF-802 3`) and, for `run` and `mutate` verbs, one without a key (`e2e_expect_refusal
FROB-REF-805 3`); (7) one envelope per call and no exit code 1.

Log: `target/e2e-logs/verbs_eval/<utc>.ndjson` with a transcript beside it. Each `cmd` line has the exact command,
exit code, wall time and the envelope's `evidence` block; each `assert` has observed and expected values and the tag
`--gate GATE-TETHER-02`; the metrics `registry_verbs`, `cli_verbs` and `mcp_tools` are logged against 10 and the
difference counts against 0; refusals are logged with code and first repair; a failure logs the first differing verb
and field; the `summary` line gives assertions passed and failed and the gate touched.

### Known answers and negative controls

- Known answers: the cluster has 10 verbs (4 `read`, 6 `run`); the tables above. KA-755 (the exit-code table on
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

- Open naming decision (owner decision recorded with bead N16b-27): the Kinesis chapter's verb table also prints
  `fidelity.rank` for the tier-pair measurement. Fail-closed default: only `eval.rank-agreement` is registered,
  accepting a rung pair or a tier pair.
- Every `run` verb of this cluster belongs to a declared study (anytime-valid or fixed-n, never both); a request
  that mixes types is refused (`FROB-REF-552`).
- Closing rule: the bead closes at M1, when the rows are registered, the surfaces match and every verb passes its
  example or answers the not-implemented refusal. Verbs of this cluster first answer at M1, M2, M3, M4. The
  per-milestone surface beads (BD-137, BD-222, BD-312, BD-376) re-run this script as verbs land; that is how
  GATE-TETHER-02 is re-measured.
- Scope names are proposals (`<cluster>:<read|run|write|admin>` plus the named scopes of the surface) until the
  capability bead BD-258 fixes the scope registry; a `run` or `write` scope of a cluster implies its `read` scope.
  Inputs and outputs above name contents, not flag spellings, except where a flag is quoted; the implementing bead
  fixes the spellings and the snapshot then pins them (additive changes keep the schema's major version; anything
  else raises it).

## References

Appendix J (J.1 entry form; J.2 inventory rows and J.3 added rows; J.5 task clusters), §24.8, §24.12, §24.13,
§24.21, §16.8, §23.16, §28.6, §28.10, §32.4, §32.6, §32.12.3, §32.18, §32.21; GATE-TETHER-02; INV-TETHER-01, -02;
KA-755.
