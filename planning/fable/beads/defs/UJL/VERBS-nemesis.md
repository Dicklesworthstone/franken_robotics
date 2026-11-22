---
key: "VERBS-nemesis"
title: "frob-verbs registry: cluster `nemesis`, seven rows, with parity, e2e and schema-snapshot checks"
type: "task"
priority: 2
milestone: "M3"
labels: ["m3", "tether", "nemesis", "frob-verbs", "solid", "registry", "verb", "test"]
parent: "WP-26"
deps: ["BD-022", "NJL-01", "NJL-02", "STD-E2E"]
needs: []
related: ["BD-345", "N26b-05", "BD-276", "BD-278", "BD-275", "BD-271", "BD-273", "BD-274", "N26b-13", "BD-312", "BD-376", "NJL-03", "NJL-04", "NJL-05"]
gates: ["GATE-TETHER-02"]
plan_refs: ["Appendix J (J.1, J.2, J.3, J.5)", "§26.4", "§26.5", "§26.7", "§26.8", "§26.9", "§26.10", "§26.11", "§26.12", "§32.6", "§32.12.3", "§32.21 (GATE-TETHER-02)"]
---
## Background

FrankenRobotics (a robotics simulation and design system) is reached through the command line `frob`, the daemon
`frobd` and the MCP server `frob-mcp` (the Model Context Protocol, by which an agent harness lists and calls tools).
Every capability is a verb, `cluster.verb`, declared once in `registries/verbs.toml` (crate `frob-verbs`, tier
Solid). The CLI, the daemon protocol, the MCP tools, the schemas, the refusal table and the agent guide are
generated from that file, and gate GATE-TETHER-02 compares the served surfaces with it. The registry holds 243 verbs
in 44 clusters (namespaces).

This bead is the registry of record for the cluster `nemesis`. The cluster covers adversarial worlds and rare
failures: search inside a declared legality profile, the solvability oracle, shrinking a failure to a minimal
counterexample world, a bound on a failure probability, the audit of a screening stage, covering scene sets and the
hard-world set. It has seven verbs (2 `read`, 5 `run`); the handlers live in `frob-nemesis`. The bead carries every
row completely, with the three checks that keep the rows true: the generated CLI commands and MCP tools match the
list; every verb has an end-to-end script; the schema printed by `frob <verb> --help --json` is stable.

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

In: the seven registry entries of cluster `nemesis`; the MCP tool names; the cluster's conformance script; the
help-schema snapshots; each verb's answer before its milestone. Out: behaviour (the implementing beads named below);
the entry schema and lints (NJL-01); the conformance harness (NJL-02); the alias lint (NJL-04); task clusters
(NJL-05, NJL-06); generation of the surfaces (the Tether seed beads).

## Technical approach

**The rows.**

| Verb | Kind | First milestone | Tier | Purpose | Defined in |
|---|---|---|---|---|---|
| `nemesis.audit` | run | M4 | S | Audit a screening stage with stratified discards and a planted winner; the one screening audit, applied by the cascade of §28.12 | §26.10 |
| `nemesis.bound` | run | M3 | S | Certify, refute or estimate a failure probability against a target | §26.9 |
| `nemesis.cover` | read | M3 | S | Select a covering scene set and return it with its certificate | §26.11 |
| `nemesis.hardset` | read, run | M3 | S | Show, add to, retire from and run the hard-world set of a class | §26.8 |
| `nemesis.oracle` | run | M3 | S | Classify one world for one candidate: a witness, or the failed necessary condition | §26.5 |
| `nemesis.search` | run | M3 | F | Adversarial worlds inside a declared legality profile, with the baseline arm's count at equal budget | §26.4 |
| `nemesis.shrink` | run | M3 | F | Shrink a promoted failure to a minimal counterexample world and write the `.fmcw` | §26.7 |

**Contract of each verb.** Evidence labels used below (the `evidence` block of the envelope):
- *study*: a Design comparison of class Tested (Estimated if any input is `assumed`); tier and determinism the
  weakest of the arms; `provisional` until promoted to the referee.
- *static*: class Tested, tier `static`, determinism `exact` (class D0); `claims` empty unless a certificate is
  attached; `reality: no-claim`.
- *run*: class Tested at best, by the claim product of the run; the tier of the run, `provisional` on a fast tier
  until promoted to the referee; the determinism class of the placement (D2 unless stated); `reality: no-claim`.

**`nemesis.audit`** (MCP tool `nemesis_audit`; owner `frob-nemesis`)
- Inputs: screening stage of a campaign; strata; planted winner.
- Outputs: the audit record: sampled discards per stratum with an exact bound, and the rank of the planted winner.
  Evidence: *study*.
- Chapter refusals: `FROB-REF-546` (calibration from survivors, or an audit episode cancelled or re-weighted);
  `FROB-REF-566` (a planted winner's rank is not on record from a full evaluation).
- Implemented by: BD-345, N26b-05.

**`nemesis.bound`** (MCP tool `nemesis_bound`; owner `frob-nemesis`)
- Inputs: candidate; failure predicate; target; mode (`certify`, `refute`, `estimate`); budget.
- Outputs: a bound on the failure probability against the target, with the method and effective sample size, or the
  bound reachable within budget. Evidence: *study*.
- Chapter refusals: `FROB-REF-549` (a rare-failure bound over battery-selected or search-selected episodes);
  `FROB-REF-560` (a bound was offered episodes that were not drawn nominally or with recorded bounded weights);
  `FROB-REF-563` (importance sampling outside its admitted law, or with a proposal fitted on the run it is applied
  to); `FROB-REF-568` (the failure predicate changed after sampling began); `FROB-REF-569` (splitting requested
  where no randomness enters after the fork); `FROB-REF-760` (the episodes a claim needs at its level exceed the
  budget; the bound reachable within budget is attached).
- Implemented by: BD-276.

**`nemesis.cover`** (MCP tool `nemesis_cover`; owner `frob-nemesis`)
- Inputs: scene class; incidence of hazards; size.
- Outputs: a covering scene set with its coverage certificate. Evidence: *static*.
- Chapter refusals: `FROB-REF-565` (coverage selection on a stale incidence).
- Implemented by: BD-278.

**`nemesis.hardset`** (MCP tool `nemesis_hardset`; owner `frob-nemesis`)
- Inputs: class; action `show` (read), or `add`, `retire`, `run` (run).
- Outputs: `frob.hardset/1`: the hard-world set of the class; for `run`, verdicts on it labelled as regression
  results. Evidence: *run*.
- Chapter refusals: `FROB-REF-567` (a hard-world or search-selected world offered as a confirmation sample).
- Implemented by: BD-275.

**`nemesis.oracle`** (MCP tool `nemesis_oracle`; owner `frob-nemesis`)
- Inputs: world; candidate.
- Outputs: class `solvable-causal` or `plant-feasible` with a witness controller and margin, or the failed necessary
  condition. Evidence: *run*.
- Chapter refusals: `FROB-REF-360` (a world or edit lies outside the legality profile; the violated bound is named).
- Implemented by: BD-271.

**`nemesis.search`** (MCP tool `nemesis_search`; owner `frob-nemesis`)
- Inputs: candidate; task; legality profile; method; budget.
- Outputs: worlds searched, classified candidate failures, the worst world, the baseline arm's failure count at
  equal budget, the artifact guard's result. Evidence: *run*.
- Chapter refusals: `FROB-REF-360` (a world or edit lies outside the legality profile; the violated bound is named);
  `FROB-REF-361` (the request demands `cited` bounds and the profile has an `assumed` bound on a touched quantity);
  `FROB-REF-362` (an edit does not declare the causal paths it changes).
- Implemented by: BD-273.

**`nemesis.shrink`** (MCP tool `nemesis_shrink`; owner `frob-nemesis`)
- Inputs: a promoted failing world.
- Outputs: a `.fmcw` (`frob.mcw/1`): the minimal edits, minimality kind, tests run, seeds failing, passing
  neighbours, the oracle's class. Evidence: *run*.
- Chapter refusals: `FROB-REF-360` (a world or edit lies outside the legality profile; the violated bound is named);
  `FROB-REF-564` (shrinking requested for a failure that does not replay, or that the guard labelled an artifact
  candidate).
- Implemented by: BD-274.

**Machine fields of each row.**

| Verb | Capabilities | Output schema | Task clusters | Example |
|---|---|---|---|---|
| `nemesis.audit` | `nemesis:run` | `frob.nemesis.audit.out/1` | `ladder` | `frob nemesis audit camp:12 --stage 3 --key aud-1 --json` |
| `nemesis.bound` | `nemesis:run` | `frob.nemesis.bound.out/1` | `attack` | `frob nemesis bound quad-tray --task t:kitchen-tray --target 1e-3 --mode certify --key nb-1 --json` |
| `nemesis.cover` | `nemesis:read` | `frob.nemesis.cover.out/1` | `batteries` | `frob nemesis cover --class kitchen-commercial --size 32 --json` |
| `nemesis.hardset` | `nemesis:read` | `frob.hardset/1` | `attack`, `batteries` | `frob nemesis hardset --class kitchen-commercial --json` |
| `nemesis.oracle` | `nemesis:run` | `frob.nemesis.oracle.out/1` | `attack` | `frob nemesis oracle w:3fa9 --candidate quad-tray --key or-1 --json` |
| `nemesis.search` | `nemesis:run` | `frob.nemesis.search.out/1` | `attack` | `frob nemesis search quad-tray --task t:kitchen-tray --legality kitchen-commercial/default --budget gpu_s=240 --key nem-1 --json` |
| `nemesis.shrink` | `nemesis:run` | `frob.nemesis.shrink.out/1` | `attack` | `frob nemesis shrink w:9d04 --key shr-1 --json` |

The remaining fields follow by rule: `input_schema` is `frob.<verb name>.in/1`; `refusals` is the chapter list of
the verb's block; `idempotent` is `true` for `read` and `admin` and `false` for `run` and `mutate` (which take an
idempotency key); `cost_model` is `static: …` for `read`, `mutate` and `admin` and the `--estimate` cost model for
`run`; a `run` row carries `wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }`; `owner` is the
crate of the block; `purpose` is the Purpose column. A verb printed with two kinds has `kind` equal to its default
action and an `actions` list for the others, each with its own kind and scope: `nemesis.hardset`: `add` (run),
`retire` (run), `run` (run).

One entry in full (the others follow field for field):

```toml
[[verb]]
name = "nemesis.audit"
cluster = "nemesis"
kind = "run"
tier = "S"
capabilities = ["nemesis:run"]
input_schema = "frob.nemesis.audit.in/1"
output_schema = "frob.nemesis.audit.out/1"
refusals = ["FROB-REF-546", "FROB-REF-566"]
idempotent = false
cost_model = "estimated: `--estimate` returns CPU core-seconds, GPU seconds, memory and disk with a conformal interval"
first_milestone = "M4"
task_clusters = ["ladder"]
examples = ["frob nemesis audit camp:12 --stage 3 --key aud-1 --json"]
owner = "frob-nemesis"
purpose = "Audit a screening stage with stratified discards and a planted winner; the one screening audit, applied by the cascade of §28.12"
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

- Seven verbs. `nemesis.hardset` has two kinds: `show` is a `read`; `add`, `retire` and `run` are `run` actions.
- Only solvable worlds count: no world without a `solvable-causal` or `plant-feasible` witness enters the hard-world
  set or a counterexample count (INV-NEMESIS-01). A failure and its witness count only after the artifact guard.
- "None found" is a search result and not a certificate; the envelope of an empty search says so in its result.
- `nemesis.audit` is the one screening audit; the co-design cascade applies it and registers no audit verb of its
  own.

## Acceptance criteria

- [ ] `registries/verbs.toml` holds exactly seven entries with `cluster = "nemesis"` (2 `read`, 5 `run`), each with
      every field of the entry schema, an example and a task-cluster list; the alias lint finds no second spelling.
- [ ] `frob-surface-diff parity --cluster nemesis` exits 0 in process (`frob-mcp` over stdio) and through `frobd
      --mcp`: registry, CLI and MCP each count 7, and every difference list is empty apart from the harness's closed
      exemptions.
- [ ] Every verb has an end-to-end script: `scripts/e2e/verbs_nemesis.sh` runs each verb of the cluster (seven), and
      from a verb's first milestone a capability script of its implementing bead exists (`frob-surface-diff
      e2e-coverage`).
- [ ] `frob <verb> --help --json` equals `fixtures/golden/verbs/nemesis/<verb>.help.json` byte for byte for each of
      the seven, on two runs, debug and release, Linux and macOS; a planted renamed output property is classified
      `breaking` and refused.
- [ ] A verb not yet implemented answers `FROB-REF-724`, exit 5, with one envelope on both surfaces; an implemented
      verb runs its registry example with the result its implementing bead specifies; a misspelt parameter is
      `FROB-REF-802`, exit 3; a `run` or `mutate` call without a key is `FROB-REF-805`, exit 3.
- [ ] The generated refusal table lists for each verb exactly its chapter codes plus the codes appended for its
      kind, and every code exists in `registries/refusals.toml`.
- [ ] The two-kind verb exposes `action`; an `admin` or `run` action without its scope is `FROB-REF-720`, exit 5,
      and the `read` action still answers.
- [ ] The gate row is recorded with the measured counts by a session that did not write the rows.

## Test plan

### Unit tests

`cargo test -p frob-verbs --test cluster_nemesis` (the real generated table; no mocks; planted registries are TOML
strings labelled `synthetic`):
- `rows_present`: exactly 7 rows in the cluster, with the names of the rows table; a planted extra row or a removed
  row fails.
- `kinds_milestones_tiers`: each row's kind, first milestone and tier equal the rows table (for example
  `nemesis.shrink` is `run` at M3).
- `schemas_and_scopes`: input schemas are `frob.<name>.in/1`; output schemas and capabilities equal the
  machine-fields table (`nemesis.hardset` returns `frob.hardset/1`).
- `refusal_lists`: each row's chapter codes equal its block (`nemesis.audit`: 546, 566; `nemesis.bound`: 549, 560,
  563, 568, 569, 760; `nemesis.cover`: 565); the appended codes equal the rule for its kind.
- `mcp_names_one_to_one`: the tool names are the mapped names (`nemesis_audit`, `nemesis_bound`, `nemesis_cover`),
  with no collision against any other cluster.
- `task_clusters_and_examples`: every row sits in a task cluster of at most seven tools or is a base tool; each
  example parses against the generated CLI and its command path is the row's own.
- `kind_behaviour`: a `run` row has the five common properties and refuses `sync_wait_ms = 30001` with code 802; an
  `admin` row or action without its scope is refused with code 720; a verb whose capability is absent returns code
  724, exit 5, one envelope.

### End-to-end script and logging

`scripts/e2e/verbs_nemesis.sh` (header: `bead=VERBS-nemesis milestone=M3 labels=tether,verbs,verb:nemesis.*
gates=GATE-TETHER-02 seed=1 needs=cpu,mcp,frobd plants=alias,missing-row,renamed-field`) sources the shared helper
and `scripts/e2e/verbs_lib.sh` and calls `verbs_cluster_conformance nemesis`. Steps: (1) `frob capabilities --json`
and `frob robot-docs --json` list the seven verbs with kind and milestone; (2) `frob-surface-diff parity --cluster
nemesis` over stdio and through `frobd --mcp`, every difference list empty; (3) `frob-surface-diff snapshot --check
--cluster nemesis`; (4) each verb's registry example verbatim, expecting a result (exit 0 or 2), a job handle (exit
8, then `frob job wait`) or `FROB-REF-724` exactly when the capability report says the verb is absent; (5) for each
implemented verb the same call as an MCP tool, the two envelopes agreeing on `verb`, `ok`, `exit`, `result`,
`refusal`, `evidence`, `bound`, `warnings` and `next_actions`; (6) per verb one call with a misspelt parameter
(`e2e_expect_refusal FROB-REF-802 3`) and, for `run` and `mutate` verbs, one without a key (`e2e_expect_refusal
FROB-REF-805 3`); (7) one envelope per call and no exit code 1.

Log: `target/e2e-logs/verbs_nemesis/<utc>.ndjson` with a transcript beside it. Each `cmd` line has the exact
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

- Hard-world and search-selected worlds are discovery data; offering one as a confirmation sample is refused
  (`FROB-REF-567`).
- A certified failure rate of 10⁻⁶ needs a compute envelope above the default (OPEN-NEMESIS-3); without approval the
  verb certifies to 10⁻⁵ and reports 10⁻⁶ as Estimated.
- Closing rule: the bead closes at M3, when the rows are registered, the surfaces match and every verb passes its
  example or answers the not-implemented refusal. Verbs of this cluster first answer at M3, M4. The per-milestone
  surface beads (BD-312, BD-376) re-run this script as verbs land; that is how GATE-TETHER-02 is re-measured.
- Scope names are proposals (`<cluster>:<read|run|write|admin>` plus the named scopes of the surface) until the
  capability bead BD-258 fixes the scope registry; a `run` or `write` scope of a cluster implies its `read` scope.
  Inputs and outputs above name contents, not flag spellings, except where a flag is quoted; the implementing bead
  fixes the spellings and the snapshot then pins them (additive changes keep the schema's major version; anything
  else raises it).

## References

Appendix J (J.1 entry form; J.2 inventory rows; J.5 task clusters), §26.4, §26.5, §26.7, §26.8, §26.9, §26.10,
§26.11, §26.12, §32.4, §32.6, §32.12.3, §32.18, §32.21; GATE-TETHER-02; INV-TETHER-01, -02; KA-755.
