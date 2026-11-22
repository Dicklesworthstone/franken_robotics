---
key: "VERBS-ground"
title: "frob-verbs registry: cluster `ground`, seven rows, with parity, e2e and schema-snapshot checks"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "tether", "ground", "frob-verbs", "solid", "registry", "verb", "test"]
parent: "WP-06"
deps: ["BD-022", "NJL-01", "NJL-02", "STD-E2E"]
needs: []
related: ["BD-393", "BD-390", "BD-391", "BD-387", "BD-396", "BD-392", "N30b-06", "BD-137", "BD-376", "BD-409", "NJL-03", "NJL-04", "NJL-05"]
gates: ["GATE-TETHER-02"]
plan_refs: ["Appendix J (J.1, J.2, J.3, J.5)", "§30.5", "§30.9", "§30.11", "§30.16", "§30.19", "§32.6", "§32.12.3", "§32.21 (GATE-TETHER-02)"]
---
## Background

FrankenRobotics (a robotics simulation and design system) is reached through the command line `frob`, the daemon
`frobd` and the MCP server `frob-mcp` (the Model Context Protocol, by which an agent harness lists and calls tools).
Every capability is a verb, `cluster.verb`, declared once in `registries/verbs.toml` (crate `frob-verbs`, tier
Solid). The CLI, the daemon protocol, the MCP tools, the schemas, the refusal table and the agent guide are
generated from that file, and gate GATE-TETHER-02 compares the served surfaces with it. The registry holds 243 verbs
in 44 clusters (namespaces).

This bead is the registry of record for the cluster `ground`. The cluster covers calibration against reality:
admitting logs and measurements as data, identifiability and excitation design, staged identification, frozen
predictions scored once on sealed data, conformal reality bands per calibrated cell, and the replication kit. It has
seven verbs (2 `read`, 3 `run`, 2 `mutate`); the handlers live in `frob-ground`. The bead carries every row
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

In: the seven registry entries of cluster `ground`; the MCP tool names; the cluster's conformance script; the
help-schema snapshots; each verb's answer before its milestone. Out: behaviour (the implementing beads named below);
the entry schema and lints (NJL-01); the conformance harness (NJL-02); the alias lint (NJL-04); task clusters
(NJL-05, NJL-06); generation of the surfaces (the Tether seed beads).

## Technical approach

**The rows.**

| Verb | Kind | First milestone | Tier | Purpose | Defined in |
|---|---|---|---|---|---|
| `ground.band` | read | M5 | F | Issue or refuse a reality band; cells, coverage accounting, monitor states; `--retire` needs a capability | §30.16 |
| `ground.excite` | read | M4 | F | Identifiability report and excitation plan for a design, a card set and a rig | §30.16 |
| `ground.identify` | run | M5 | F | Staged identification; writes candidate card versions and the calibration record | §30.9 |
| `ground.ingest` | mutate | M1 | F | Admit a log, a bench measurement or a scan; fit clocks; seal splits | §30.5 |
| `ground.kit` | run | M5 | F | Build or check a replication kit | §30.16 |
| `ground.predict` | mutate | M5 | F | Freeze a prediction for a planned real trial | §30.11 |
| `ground.score` | run | M5 | F | Open sealed data once, score, retire the holdout, admit or reject the posterior | §30.11 |

**Contract of each verb.** Evidence labels used below (the `evidence` block of the envelope):
- *reality*: a Reality claim only for a calibrated cell with a frozen prediction scored on sealed data; otherwise
  `reality: no-claim`; class Tested or Estimated by the weakest input.
- *static*: class Tested, tier `static`, determinism `exact` (class D0); `claims` empty unless a certificate is
  attached; `reality: no-claim`.
- *screening*: class Estimated: a screening or planning signal that may discard under the ladder's rules and may not
  establish a comparison.
- *pass-through*: passes through the claim product of the stored objects it reads and adds no claim of its own; the
  read itself is `exact`.

**`ground.band`** (MCP tool `ground_band`; owner `frob-ground`)
- Inputs: predicted metric; calibrated cell; `--retire` (needs a capability).
- Outputs: a conformal reality band for the cell, or a refusal; cells, coverage accounting, monitor states.
  Evidence: *reality*.
- Chapter refusals: `FROB-REF-889` (a band for an uncalibrated cell, or with too few calibration points);
  `FROB-REF-895` (a band for a design that was never built); `FROB-REF-897` (use of a retired calibration: The
  retirement reason; recalibrate).
- Implemented by: BD-393.

**`ground.excite`** (MCP tool `ground_excite`; owner `frob-ground`)
- Inputs: design; card set; rig description.
- Outputs: identifiability report and an excitation plan. Evidence: *static*.
- Chapter refusals: `FROB-REF-879` (a fit on a parameter set that fails identifiability).
- Implemented by: BD-390.

**`ground.identify`** (MCP tool `ground_identify`; owner `frob-ground`)
- Inputs: dataset; parameter set; stages (actuators first).
- Outputs: candidate card versions and the calibration record. Evidence: *screening*.
- Chapter refusals: `FROB-REF-814` (pooling across cells, across `ModelId`s, or of synthetic with real data);
  `FROB-REF-879` (a fit on a parameter set that fails identifiability).
- Implemented by: BD-391.

**`ground.ingest`** (MCP tool `ground_ingest`; owner `frob-ground`)
- Inputs: a log, a bench measurement or a scan, as data, with its manifest (licence, provenance, rights,
  uncertainty).
- Outputs: dataset identity (`frob.ground.dataset/1`), fitted clocks, sealed validation and holdout splits.
  Evidence: *pass-through*.
- Chapter refusals: `FROB-REF-812` (an as-built registration that is ill-posed or below its noise screen);
  `FROB-REF-873` (a dataset lacking licence, provenance, rights or measurement uncertainty); `FROB-REF-876` (clock
  alignment under-determined, or a scored instant outside the bracket).
- Implemented by: BD-387.

**`ground.kit`** (MCP tool `ground_kit`; owner `frob-ground`)
- Inputs: calibration record; output root.
- Outputs: a replication kit, or the check of one. Evidence: *pass-through*.
- Chapter refusals: `FROB-REF-899` (a private or non-redistributable input to a kit).
- Implemented by: BD-396.

**`ground.predict`** (MCP tool `ground_predict`; owner `frob-ground`)
- Inputs: planned real trial; model and card versions.
- Outputs: a frozen prediction record with its hash. Evidence: *pass-through*.
- Chapter refusals: `FROB-REF-883` (a read of sealed data with no frozen prediction, or a prediction changed after
  freezing).
- Implemented by: BD-392.

**`ground.score`** (MCP tool `ground_score`; owner `frob-ground`)
- Inputs: frozen prediction; sealed data.
- Outputs: the score, the holdout retired, the posterior admitted or rejected. Evidence: *reality*.
- Chapter refusals: `FROB-REF-883` (a read of sealed data with no frozen prediction, or a prediction changed after
  freezing); `FROB-REF-886` (reuse of a spent holdout: Acquire fresh sealed data).
- Implemented by: BD-392, N30b-06.

**Machine fields of each row.**

| Verb | Capabilities | Output schema | Task clusters | Example |
|---|---|---|---|---|
| `ground.band` | `ground:read` | `frob.ground.band.out/1` | `reality`, `calibrate` | `frob ground band --metric crossing_time --cell cart/sill-12mm --json` |
| `ground.excite` | `ground:read` | `frob.ground.excite.out/1` | `calibrate` | `frob ground excite cart-a --cards drive-60 --rig stand-A --json` |
| `ground.identify` | `ground:run` | `frob.ground.identify.out/1` | `reality`, `calibrate` | `frob ground identify ds:stand-A-01 --key gi-1 --json` |
| `ground.ingest` | `ground:import` | `frob.ground.ingest.out/1` | `reality`, `calibrate` | `frob ground ingest stand-A-01/ --key gin-1 --json` |
| `ground.kit` | `ground:run`, `export:packet` | `frob.ground.kit.out/1` | `calibrate` | `frob ground kit cal:7 --out kit/ --key gk-1 --json` |
| `ground.predict` | `ground:write` | `frob.ground.predict.out/1` | `reality` | `frob ground predict --trial sill-day-3 --key gp-1 --json` |
| `ground.score` | `ground:run` | `frob.ground.score.out/1` | `reality` | `frob ground score pred:b3:11a0 --key gs-1 --json` |

The remaining fields follow by rule: `input_schema` is `frob.<verb name>.in/1`; `refusals` is the chapter list of
the verb's block; `idempotent` is `true` for `read` and `admin` and `false` for `run` and `mutate` (which take an
idempotency key); `cost_model` is `static: …` for `read`, `mutate` and `admin` and the `--estimate` cost model for
`run`; a `run` row carries `wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }`; `owner` is the
crate of the block; `purpose` is the Purpose column.

One entry in full (the others follow field for field):

```toml
[[verb]]
name = "ground.identify"
cluster = "ground"
kind = "run"
tier = "F"
capabilities = ["ground:run"]
input_schema = "frob.ground.identify.in/1"
output_schema = "frob.ground.identify.out/1"
refusals = ["FROB-REF-814", "FROB-REF-879"]
idempotent = false
cost_model = "estimated: `--estimate` returns CPU core-seconds, GPU seconds, memory and disk with a conformal interval"
first_milestone = "M5"
task_clusters = ["reality", "calibrate"]
examples = ["frob ground identify ds:stand-A-01 --key gi-1 --json"]
owner = "frob-ground"
purpose = "Staged identification; writes candidate card versions and the calibration record"
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

- Seven verbs, the limit for one registry cluster of this chapter.
- Refusal codes. The chapter's verb table prints chapter-local numbers that predate the refusal registry (840 to
  850), which collide with the qualification codes. The registry codes used here are those of the chapter's refusal
  table, matched by meaning: 840 → `FROB-REF-873`, 841 → 876, 842 → 879, 843 → 883, 844 → 886, 845 → 889, 846 → 895,
  847 → 897, 848 → 899, 849 → 812, 850 → 814.
- `ground.ingest` first answers at M1 for admission and for the `reality:` line that every report prints, and at M5
  for scans. Its seed bead (BD-387) is at M5; the M1 form is the admission record and the `reality: no-claim` line.
- Imported text is data and is never interpreted as instructions; every echo of it carries the untrusted mark.

## Acceptance criteria

- [ ] `registries/verbs.toml` holds exactly seven entries with `cluster = "ground"` (2 `read`, 3 `run`, 2 `mutate`),
      each with every field of the entry schema, an example and a task-cluster list; the alias lint finds no second
      spelling.
- [ ] `frob-surface-diff parity --cluster ground` exits 0 in process (`frob-mcp` over stdio) and through `frobd
      --mcp`: registry, CLI and MCP each count 7, and every difference list is empty apart from the harness's closed
      exemptions.
- [ ] Every verb has an end-to-end script: `scripts/e2e/verbs_ground.sh` runs each verb of the cluster (seven), and
      from a verb's first milestone a capability script of its implementing bead exists (`frob-surface-diff
      e2e-coverage`).
- [ ] `frob <verb> --help --json` equals `fixtures/golden/verbs/ground/<verb>.help.json` byte for byte for each of
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

`cargo test -p frob-verbs --test cluster_ground` (the real generated table; no mocks; planted registries are TOML
strings labelled `synthetic`):
- `rows_present`: exactly 7 rows in the cluster, with the names of the rows table; a planted extra row or a removed
  row fails.
- `kinds_milestones_tiers`: each row's kind, first milestone and tier equal the rows table (for example
  `ground.score` is `run` at M5).
- `schemas_and_scopes`: input schemas are `frob.<name>.in/1`; output schemas and capabilities equal the
  machine-fields table.
- `refusal_lists`: each row's chapter codes equal its block (`ground.band`: 889, 895, 897; `ground.excite`: 879;
  `ground.identify`: 814, 879); the appended codes equal the rule for its kind.
- `mcp_names_one_to_one`: the tool names are the mapped names (`ground_band`, `ground_excite`, `ground_identify`),
  with no collision against any other cluster.
- `task_clusters_and_examples`: every row sits in a task cluster of at most seven tools or is a base tool; each
  example parses against the generated CLI and its command path is the row's own.
- `kind_behaviour`: a `run` row has the five common properties and refuses `sync_wait_ms = 30001` with code 802; a
  `mutate` row has `idempotency_key` and `expected_parent` and refuses a call without the key with code 805; a verb
  whose capability is absent returns code 724, exit 5, one envelope.

### End-to-end script and logging

`scripts/e2e/verbs_ground.sh` (header: `bead=VERBS-ground milestone=M1 labels=tether,verbs,verb:ground.*
gates=GATE-TETHER-02 seed=1 needs=cpu,mcp,frobd plants=alias,missing-row,renamed-field`) sources the shared helper
and `scripts/e2e/verbs_lib.sh` and calls `verbs_cluster_conformance ground`. Steps: (1) `frob capabilities --json`
and `frob robot-docs --json` list the seven verbs with kind and milestone; (2) `frob-surface-diff parity --cluster
ground` over stdio and through `frobd --mcp`, every difference list empty; (3) `frob-surface-diff snapshot --check
--cluster ground`; (4) each verb's registry example verbatim, expecting a result (exit 0 or 2), a job handle (exit
8, then `frob job wait`) or `FROB-REF-724` exactly when the capability report says the verb is absent; (5) for each
implemented verb the same call as an MCP tool, the two envelopes agreeing on `verb`, `ok`, `exit`, `result`,
`refusal`, `evidence`, `bound`, `warnings` and `next_actions`; (6) per verb one call with a misspelt parameter
(`e2e_expect_refusal FROB-REF-802 3`) and, for `run` and `mutate` verbs, one without a key (`e2e_expect_refusal
FROB-REF-805 3`); (7) one envelope per call and no exit code 1.

Log: `target/e2e-logs/verbs_ground/<utc>.ndjson` with a transcript beside it. Each `cmd` line has the exact command,
exit code, wall time and the envelope's `evidence` block; each `assert` has observed and expected values and the tag
`--gate GATE-TETHER-02`; the metrics `registry_verbs`, `cli_verbs` and `mcp_tools` are logged against 7 and the
difference counts against 0; refusals are logged with code and first repair; a failure logs the first differing verb
and field; the `summary` line gives assertions passed and failed and the gate touched.

### Known answers and negative controls

- Known answers: the cluster has 7 verbs (2 `read`, 3 `run`, 2 `mutate`); the tables above. KA-755 (the exit-code
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

- Without imported data every verb of the cluster still answers: `ground.band` returns `reality: no-claim` with the
  simulation's own interval. Nothing in the program waits for a bench.
- `ground.band --retire` needs a capability scope and is the only mutating action of a `read` verb in this cluster;
  it is modelled as an action of kind `admin`.
- Closing rule: the bead closes at M1, when the rows are registered, the surfaces match and every verb passes its
  example or answers the not-implemented refusal. Verbs of this cluster first answer at M1, M4, M5. The
  per-milestone surface beads (BD-137, BD-376, BD-409) re-run this script as verbs land; that is how GATE-TETHER-02
  is re-measured.
- Scope names are proposals (`<cluster>:<read|run|write|admin>` plus the named scopes of the surface) until the
  capability bead BD-258 fixes the scope registry; a `run` or `write` scope of a cluster implies its `read` scope.
  Inputs and outputs above name contents, not flag spellings, except where a flag is quoted; the implementing bead
  fixes the spellings and the snapshot then pins them (additive changes keep the schema's major version; anything
  else raises it).

## References

Appendix J (J.1 entry form; J.2 inventory rows; J.5 task clusters), §30.5, §30.9, §30.11, §30.16, §30.19, §32.4,
§32.6, §32.12.3, §32.18, §32.21; GATE-TETHER-02; INV-TETHER-01, -02; KA-755.
