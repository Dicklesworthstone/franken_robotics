---
key: "VERBS-phylum"
title: "frob-verbs registry: cluster `phylum`, seven rows, with parity, e2e and schema-snapshot checks"
type: "task"
priority: 1
milestone: "M2"
labels: ["m2", "tether", "phylum", "frob-verbs", "solid", "registry", "verb", "test"]
parent: "WP-43"
deps: ["BD-022", "NJL-01", "NJL-02", "STD-E2E"]
needs: []
related: ["BD-287", "BD-353", "BD-342", "N17b-18", "BD-171", "BD-285", "BD-286", "N17b-14", "BD-184", "N17b-04", "N17b-12", "BD-222", "BD-312", "BD-376", "NJL-03", "NJL-04", "NJL-05"]
gates: ["GATE-TETHER-02"]
plan_refs: ["Appendix J (J.1, J.2, J.3, J.5)", "§17.5", "§17.11", "§17.12", "§17.13", "§17.15", "§17.16", "§32.6", "§32.12.3", "§32.21 (GATE-TETHER-02)"]
---
## Background

FrankenRobotics (a robotics simulation and design system) is reached through the command line `frob`, the daemon
`frobd` and the MCP server `frob-mcp` (the Model Context Protocol, by which an agent harness lists and calls tools).
Every capability is a verb, `cluster.verb`, declared once in `registries/verbs.toml` (crate `frob-verbs`, tier
Solid). The CLI, the daemon protocol, the MCP tools, the schemas, the refusal table and the agent guide are
generated from that file, and gate GATE-TETHER-02 compares the served surfaces with it. The registry holds 243 verbs
in 44 clusters (namespaces).

This bead is the registry of record for the cluster `phylum`. The cluster covers populations of designs in mixed
batches: planning buckets and memory without running physics, sweeping a population, sampling designs from a
generator, quotienting by symmetry, reuse after an edit, the quality-diversity map and paired design gradients. It
has seven verbs (5 `read`, 2 `run`); the handlers live in `frob-phylum`. The bead carries every row completely, with
the three checks that keep the rows true: the generated CLI commands and MCP tools match the list; every verb has an
end-to-end script; the schema printed by `frob <verb> --help --json` is stable.

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

In: the seven registry entries of cluster `phylum`; the MCP tool names; the cluster's conformance script; the
help-schema snapshots; each verb's answer before its milestone. Out: behaviour (the implementing beads named below);
the entry schema and lints (NJL-01); the conformance harness (NJL-02); the alias lint (NJL-04); task clusters
(NJL-05, NJL-06); generation of the surfaces (the Tether seed beads).

## Technical approach

**The rows.**

| Verb | Kind | First milestone | Tier | Purpose | Defined in |
|---|---|---|---|---|---|
| `phylum.generate` | read | M3 | S | Sample designs from a generator with derivations and log-probabilities | §17.15 |
| `phylum.gradient` | run | M4 | F | Paired finite-difference design gradient with intervals and divergence steps | §17.13 |
| `phylum.map` | read | M4 | S | The quality-diversity map of a campaign or one dispatch | §17.12 |
| `phylum.plan` | read | M2 | S | Buckets, padding overhead, bytes, the machine that fits a population; never runs physics | §17.5 |
| `phylum.quotient` | read | M3 | S | Orbit representatives under Γ_obs and mission-digest classes for a design set | §17.11 |
| `phylum.reuse` | read | M3 | S | For an edit: which cached results survive and the field that forced each invalidation | §17.15 |
| `phylum.sweep` | run | M2 | F | Run a population of designs in mixed batches; a provisional scorecard per design | §17.15 |

**Contract of each verb.** Evidence labels used below (the `evidence` block of the envelope):
- *static*: class Tested, tier `static`, determinism `exact` (class D0); `claims` empty unless a certificate is
  attached; `reality: no-claim`.
- *screening*: class Estimated: a screening or planning signal that may discard under the ladder's rules and may not
  establish a comparison.
- *pass-through*: passes through the claim product of the stored objects it reads and adds no claim of its own; the
  read itself is `exact`.
- *run*: class Tested at best, by the claim product of the run; the tier of the run, `provisional` on a fast tier
  until promoted to the referee; the determinism class of the placement (D2 unless stated); `reality: no-claim`.

**`phylum.generate`** (MCP tool `phylum_generate`; owner `frob-phylum`)
- Inputs: generator; count; seed; `--locate` (support test); `--report`.
- Outputs: sampled designs with derivations and log-probabilities; rejection rates and the bucket histogram.
  Evidence: *static*.
- Chapter refusals: `FROB-REF-541` (outside generator support: A family-policy score is requested for a design
  outside its training generator's support).
- Implemented by: BD-287.

**`phylum.gradient`** (MCP tool `phylum_gradient`; owner `frob-phylum`)
- Inputs: design; levers; task; pairs; `--method adjoint` (Moonshot, refused until its gate).
- Outputs: paired finite-difference design gradients with intervals, per-pair divergence steps, the
  `truncated-horizon` label where it applies. Evidence: *screening*.
- Chapter refusals: `FROB-REF-540` (gradient unreliable: Non-smooth metric, horizon above the admitted cap, pairs
  diverged early, or an uncontrolled determinism class).
- Implemented by: BD-353.

**`phylum.map`** (MCP tool `phylum_map`; owner `frob-phylum`)
- Inputs: campaign or one dispatch; `--merge`.
- Outputs: the quality-diversity map; the merge of maps of one specification. Evidence: *pass-through*.
- Chapter refusals: none of its own.
- Implemented by: BD-342, N17b-18.

**`phylum.plan`** (MCP tool `phylum_plan`; owner `frob-phylum`)
- Inputs: a population of designs; target machine.
- Outputs: buckets, padding overhead per stage exponent, bytes per world and in total, the machine that fits it;
  never runs physics. Evidence: *static*.
- Chapter refusals: `FROB-REF-160` (no shape class: A design exceeds the largest admitted bucket and the
  tree-operator route is off); `FROB-REF-740` (population exceeds device: The population's bytes exceed every
  admitted placement); `FROB-REF-741` (bucket fragmentation: More than eight buckets are needed to hold padding
  waste at or below the limit).
- Implemented by: BD-171.

**`phylum.quotient`** (MCP tool `phylum_quotient`; owner `frob-phylum`)
- Inputs: a design set.
- Outputs: orbit representatives under Γ_obs with the orbit count, and mission-digest classes with the fields that
  split them. Evidence: *static*.
- Chapter refusals: `FROB-REF-161` (group not observation-preserving: A quotient or a weight tie is requested under
  a group that moves a sensor, the payload or the task).
- Implemented by: BD-285.

**`phylum.reuse`** (MCP tool `phylum_reuse`; owner `frob-phylum`)
- Inputs: an edit (design and parent).
- Outputs: which cached results survive, which are invalidated, and the field that forced each invalidation.
  Evidence: *pass-through*.
- Chapter refusals: `FROB-REF-162` (reuse without a dependency digest: A cached result has no recorded dependency
  digest, or the shadow recompute disagreed).
- Implemented by: BD-286, N17b-14.

**`phylum.sweep`** (MCP tool `phylum_sweep`; owner `frob-phylum`)
- Inputs: `--designs`, `--worlds-per-design`, `--task`, `--tier`; budget.
- Outputs: one envelope with a provisional scorecard per design, cohort count, padding overhead, the map delta,
  referee coverage. Evidence: *run*.
- Chapter refusals: `FROB-REF-160` (no shape class: A design exceeds the largest admitted bucket and the
  tree-operator route is off); `FROB-REF-740` (population exceeds device: The population's bytes exceed every
  admitted placement); `FROB-REF-741` (bucket fragmentation: More than eight buckets are needed to hold padding
  waste at or below the limit).
- Implemented by: BD-184, N17b-04.

**Machine fields of each row.**

| Verb | Capabilities | Output schema | Task clusters | Example |
|---|---|---|---|---|
| `phylum.generate` | `phylum:read` | `frob.phylum.generate.out/1` | `families` | `frob phylum generate gen:quad-1 --n 64 --seed 7 --json` |
| `phylum.gradient` | `phylum:run` | `frob.phylum.gradient.out/1` | `families` | `frob phylum gradient quad-a --levers thigh --task stairs-0.4 --key pg-1 --json` |
| `phylum.map` | `phylum:read` | `frob.phylum.map.out/1` | `families` | `frob phylum map camp:12 --json` |
| `phylum.plan` | `phylum:read` | `frob.phylum.plan.out/1` | `families`, `fleet` | `frob phylum plan --designs @archive:gen-14 --json` |
| `phylum.quotient` | `phylum:read` | `frob.phylum.quotient.out/1` | `families`, `symmetry` | `frob phylum quotient --designs @archive:gen-14 --json` |
| `phylum.reuse` | `phylum:read` | `frob.phylum.reuse.out/1` | `families` | `frob phylum reuse quad-a --since d:41aa --json` |
| `phylum.sweep` | `phylum:run` | `frob.phylum.sweep.out/1` | `families` | `frob phylum sweep --designs @archive:gen-14 --task stairs-0.15 --worlds-per-design 16 --tier gpu-cuda --key sw-1 --json` |

The remaining fields follow by rule: `input_schema` is `frob.<verb name>.in/1`; `refusals` is the chapter list of
the verb's block; `idempotent` is `true` for `read` and `admin` and `false` for `run` and `mutate` (which take an
idempotency key); `cost_model` is `static: …` for `read`, `mutate` and `admin` and the `--estimate` cost model for
`run`; a `run` row carries `wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }`; `owner` is the
crate of the block; `purpose` is the Purpose column.

One entry in full (the others follow field for field):

```toml
[[verb]]
name = "phylum.gradient"
cluster = "phylum"
kind = "run"
tier = "F"
capabilities = ["phylum:run"]
input_schema = "frob.phylum.gradient.in/1"
output_schema = "frob.phylum.gradient.out/1"
refusals = ["FROB-REF-540"]
idempotent = false
cost_model = "estimated: `--estimate` returns CPU core-seconds, GPU seconds, memory and disk with a conformal interval"
first_milestone = "M4"
task_clusters = ["families"]
examples = ["frob phylum gradient quad-a --levers thigh --task stairs-0.4 --key pg-1 --json"]
owner = "frob-phylum"
purpose = "Paired finite-difference design gradient with intervals and divergence steps"
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

- Exactly seven verbs, the limit this chapter sets for its cluster: specialization of survivors is an option of
  `phylum.sweep`, not a verb, and stays refused until the gate that funds it passes.
- Options registered with rows and gated: `--limb-kernels`, `--solve-queues`, `--specialize-survivors` on
  `phylum.sweep`; `--method adjoint` on `phylum.gradient` (Moonshot). Each answers the not-implemented refusal until
  its gate is recorded as passed.

## Acceptance criteria

- [ ] `registries/verbs.toml` holds exactly seven entries with `cluster = "phylum"` (5 `read`, 2 `run`), each with
      every field of the entry schema, an example and a task-cluster list; the alias lint finds no second spelling.
- [ ] `frob-surface-diff parity --cluster phylum` exits 0 in process (`frob-mcp` over stdio) and through `frobd
      --mcp`: registry, CLI and MCP each count 7, and every difference list is empty apart from the harness's closed
      exemptions.
- [ ] Every verb has an end-to-end script: `scripts/e2e/verbs_phylum.sh` runs each verb of the cluster (seven), and
      from a verb's first milestone a capability script of its implementing bead exists (`frob-surface-diff
      e2e-coverage`).
- [ ] `frob <verb> --help --json` equals `fixtures/golden/verbs/phylum/<verb>.help.json` byte for byte for each of
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

`cargo test -p frob-verbs --test cluster_phylum` (the real generated table; no mocks; planted registries are TOML
strings labelled `synthetic`):
- `rows_present`: exactly 7 rows in the cluster, with the names of the rows table; a planted extra row or a removed
  row fails.
- `kinds_milestones_tiers`: each row's kind, first milestone and tier equal the rows table (for example
  `phylum.sweep` is `run` at M2).
- `schemas_and_scopes`: input schemas are `frob.<name>.in/1`; output schemas and capabilities equal the
  machine-fields table.
- `refusal_lists`: each row's chapter codes equal its block (`phylum.generate`: 541; `phylum.gradient`: 540;
  `phylum.map`: none); the appended codes equal the rule for its kind.
- `mcp_names_one_to_one`: the tool names are the mapped names (`phylum_generate`, `phylum_gradient`, `phylum_map`),
  with no collision against any other cluster.
- `task_clusters_and_examples`: every row sits in a task cluster of at most seven tools or is a base tool; each
  example parses against the generated CLI and its command path is the row's own.
- `kind_behaviour`: a `run` row has the five common properties and refuses `sync_wait_ms = 30001` with code 802; a
  verb whose capability is absent returns code 724, exit 5, one envelope.

### End-to-end script and logging

`scripts/e2e/verbs_phylum.sh` (header: `bead=VERBS-phylum milestone=M2 labels=tether,verbs,verb:phylum.*
gates=GATE-TETHER-02 seed=1 needs=cpu,mcp,frobd plants=alias,missing-row,renamed-field`) sources the shared helper
and `scripts/e2e/verbs_lib.sh` and calls `verbs_cluster_conformance phylum`. Steps: (1) `frob capabilities --json`
and `frob robot-docs --json` list the seven verbs with kind and milestone; (2) `frob-surface-diff parity --cluster
phylum` over stdio and through `frobd --mcp`, every difference list empty; (3) `frob-surface-diff snapshot --check
--cluster phylum`; (4) each verb's registry example verbatim, expecting a result (exit 0 or 2), a job handle (exit
8, then `frob job wait`) or `FROB-REF-724` exactly when the capability report says the verb is absent; (5) for each
implemented verb the same call as an MCP tool, the two envelopes agreeing on `verb`, `ok`, `exit`, `result`,
`refusal`, `evidence`, `bound`, `warnings` and `next_actions`; (6) per verb one call with a misspelt parameter
(`e2e_expect_refusal FROB-REF-802 3`) and, for `run` and `mutate` verbs, one without a key (`e2e_expect_refusal
FROB-REF-805 3`); (7) one envelope per call and no exit code 1.

Log: `target/e2e-logs/verbs_phylum/<utc>.ndjson` with a transcript beside it. Each `cmd` line has the exact command,
exit code, wall time and the envelope's `evidence` block; each `assert` has observed and expected values and the tag
`--gate GATE-TETHER-02`; the metrics `registry_verbs`, `cli_verbs` and `mcp_tools` are logged against 7 and the
difference counts against 0; refusals are logged with code and first repair; a failure logs the first differing verb
and field; the `summary` line gives assertions passed and failed and the gate touched.

### Known answers and negative controls

- Known answers: the cluster has 7 verbs (5 `read`, 2 `run`); the tables above. KA-755 (the exit-code table on
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

- If kill gate KILL-PHYLUM (mixed batches lose the head-to-head) fires, the verbs and schemas stay; `phylum.plan`
  reports template coverage and `phylum.sweep` forms cohorts by exact tree.
- Closing rule: the bead closes at M2, when the rows are registered, the surfaces match and every verb passes its
  example or answers the not-implemented refusal. Verbs of this cluster first answer at M2, M3, M4. The
  per-milestone surface beads (BD-222, BD-312, BD-376) re-run this script as verbs land; that is how GATE-TETHER-02
  is re-measured.
- Scope names are proposals (`<cluster>:<read|run|write|admin>` plus the named scopes of the surface) until the
  capability bead BD-258 fixes the scope registry; a `run` or `write` scope of a cluster implies its `read` scope.
  Inputs and outputs above name contents, not flag spellings, except where a flag is quoted; the implementing bead
  fixes the spellings and the snapshot then pins them (additive changes keep the schema's major version; anything
  else raises it).

## References

Appendix J (J.1 entry form; J.2 inventory rows; J.5 task clusters), §17.5, §17.11, §17.12, §17.13, §17.15, §17.16,
§32.4, §32.6, §32.12.3, §32.18, §32.21; GATE-TETHER-02; INV-TETHER-01, -02; KA-755.
