---
key: "VERBS-telemetry"
title: "frob-verbs registry: cluster `telemetry`, nine rows, with parity, e2e and schema-snapshot checks"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "tether", "chronicle", "frob-verbs", "solid", "registry", "verb", "test"]
parent: "WP-06"
deps: ["BD-022", "NJL-01", "NJL-02", "STD-E2E"]
needs: []
related: ["BD-206", "BD-094", "N24a-37", "BD-191", "N24a-41", "BD-207", "N29a-15", "N24a-40", "N24a-36", "N24a-35", "N24a-39", "BD-137", "BD-222", "NJL-03", "NJL-04", "NJL-05"]
gates: ["GATE-TETHER-02"]
plan_refs: ["Appendix J (J.1, J.2, J.3, J.5)", "§29.4", "§29.5", "§29.7", "§29.8", "§29.9", "§29.10", "§29.12", "§20.9", "§32.15", "§32.6", "§32.12.3", "§32.21 (GATE-TETHER-02)"]
---
## Background

FrankenRobotics (a robotics simulation and design system) is reached through the command line `frob`, the daemon
`frobd` and the MCP server `frob-mcp` (the Model Context Protocol, by which an agent harness lists and calls tools).
Every capability is a verb, `cluster.verb`, declared once in `registries/verbs.toml` (crate `frob-verbs`, tier
Solid). The CLI, the daemon protocol, the MCP tools, the schemas, the refusal table and the agent guide are
generated from that file, and gate GATE-TETHER-02 compares the served surfaces with it. The registry holds 243 verbs
in 44 clusters (namespaces).

This bead is the registry of record for the cluster `telemetry`. The cluster covers reading recorded runs: a
token-budgeted summary, columns by stream and range, the energy ledger, failures by class, histograms, the first
divergence of two runs, stills with overlays, the quantity under a pixel, and the self-contained HTML replay. It has
nine verbs (9 `read`); the handlers live in `frob-chronicle`. The bead carries every row completely, with the three
checks that keep the rows true: the generated CLI commands and MCP tools match the list; every verb has an
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

In: the nine registry entries of cluster `telemetry`; the MCP tool names; the cluster's conformance script; the
help-schema snapshots; each verb's answer before its milestone. Out: behaviour (the implementing beads named below);
the entry schema and lints (NJL-01); the conformance harness (NJL-02); the alias lint (NJL-04); task clusters
(NJL-05, NJL-06); generation of the surfaces (the Tether seed beads).

## Technical approach

**The rows.**

| Verb | Kind | First milestone | Tier | Purpose | Defined in |
|---|---|---|---|---|---|
| `telemetry.divergence` | read | M2 | S | First divergence between two runs, in exact mode along the digest chain or in tolerance mode | §29.8 |
| `telemetry.energy` | read | M1 | S | The energy ledger and the created-energy audit | §29.9 |
| `telemetry.export-html` | read | M2 | S | A replay as one self-contained HTML file | §29.12; viewer text in §32.15, §32.24 |
| `telemetry.failures` | read | M1 | S | Failures by class, with first offending events | §29.10 |
| `telemetry.histogram` | read | M2 | S | Deterministic integer log-bin histograms and quantiles | §29.7 |
| `telemetry.inspect-pixel` | read | M2 | S | The quantity under a pixel with its unit, channel and world address | §32.15 |
| `telemetry.query` | read | M1 | S | Columns by stream and range, paged against a committed frontier | §29.4, §29.5 |
| `telemetry.stills` | read | M2 | S | A still or a contact sheet with overlays | §20.9 |
| `telemetry.summary` | read | M1 | S | A token-budgeted run summary with handles for drill-down | §29.12 |

**Contract of each verb.** Evidence labels used below (the `evidence` block of the envelope):
- *pass-through*: passes through the claim product of the stored objects it reads and adds no claim of its own; the
  read itself is `exact`.

**`telemetry.divergence`** (MCP tool `telemetry_divergence`; owner `frob-chronicle`)
- Inputs: two runs; mode (`exact` along the digest chain, or `tolerance`).
- Outputs: the first divergent tick, stage and channel. Evidence: *pass-through*.
- Chapter refusals: `FROB-REF-894` (a snapshot restored under another numerical profile and labelled a
  continuation).
- Implemented by: BD-206.

**`telemetry.energy`** (MCP tool `telemetry_energy`; owner `frob-chronicle`)
- Inputs: run; optional world.
- Outputs: the energy ledger by line, joint, limb and phase, the numerical residual, the created-energy audit flags.
  Evidence: *pass-through*.
- Chapter refusals: none of its own.
- Implemented by: BD-094, N24a-37.

**`telemetry.export-html`** (MCP tool `telemetry_export_html`; owner `frob-chronicle`)
- Inputs: run; worlds; overlays.
- Outputs: one self-contained HTML replay file (at most 15 MB, no network fetch), its size computed before writing.
  Evidence: *pass-through*.
- Chapter refusals: `FROB-REF-731` (a private object, or anything derived from one, without the private scope and
  flag); `FROB-REF-733` (the licence set of the artifact forbids this export); `FROB-REF-816` (an artifact whose
  authorization flags are missing or not `false`).
- Implemented by: BD-191, N24a-41.

**`telemetry.failures`** (MCP tool `telemetry_failures`; owner `frob-chronicle`)
- Inputs: run.
- Outputs: failures by class with first offending events. Evidence: *pass-through*.
- Chapter refusals: none of its own.
- Implemented by: BD-094.

**`telemetry.histogram`** (MCP tool `telemetry_histogram`; owner `frob-chronicle`)
- Inputs: run; channel; bins.
- Outputs: deterministic integer log-bin histograms and quantiles. Evidence: *pass-through*.
- Chapter refusals: `FROB-REF-898` (an order-dependent aggregate requested on the evidence path).
- Implemented by: BD-207, N29a-15.

**`telemetry.inspect-pixel`** (MCP tool `telemetry_inspect_pixel`; owner `frob-chronicle`)
- Inputs: still (run, world, instant); pixel.
- Outputs: the quantity under the pixel with its unit, channel and world address. Evidence: *pass-through*.
- Chapter refusals: none of its own.
- Implemented by: BD-191, N24a-40.

**`telemetry.query`** (MCP tool `telemetry_query`; owner `frob-chronicle`)
- Inputs: run; stream; range; channels; cursor.
- Outputs: columns by stream and range, downsampled by channel kind, paged against a committed frontier. Evidence:
  *pass-through*.
- Chapter refusals: `FROB-REF-888` (a metric computed over missing samples); `FROB-REF-896` (a pagination cursor
  used with a different pin, filter or order).
- Implemented by: BD-094, N24a-36.

**`telemetry.stills`** (MCP tool `telemetry_stills`; owner `frob-chronicle`)
- Inputs: run; world; instant; `--overlay` list.
- Outputs: a still or a contact sheet (PNG) with overlays. Evidence: *pass-through*.
- Chapter refusals: `FROB-REF-731` (a private object, or anything derived from one, without the private scope and
  flag).
- Implemented by: BD-191.

**`telemetry.summary`** (MCP tool `telemetry_summary`; owner `frob-chronicle`)
- Inputs: run; token budget.
- Outputs: a token-budgeted hierarchical summary with denominators, the frontier and handles for drill-down.
  Evidence: *pass-through*.
- Chapter refusals: none of its own.
- Implemented by: BD-094, N24a-35.

**Machine fields of each row.**

| Verb | Capabilities | Output schema | Task clusters | Example |
|---|---|---|---|---|
| `telemetry.divergence` | `telemetry:read` | `frob.telemetry.divergence.out/1` | `runs`, `repair` | `frob telemetry divergence r:19e2 r:5c11 --json` |
| `telemetry.energy` | `telemetry:read` | `frob.telemetry.energy.out/1` | `diagnose` | `frob telemetry energy r:19e2 --json` |
| `telemetry.export-html` | `telemetry:read`, `export:report` | `frob.telemetry.export-html.out/1` | `runs` | `frob telemetry export-html r:19e2 --world 311 --out replay.html --json` |
| `telemetry.failures` | `telemetry:read` | `frob.telemetry.failures.out/1` | `simulate` | `frob telemetry failures r:19e2 --json` |
| `telemetry.histogram` | `telemetry:read` | `frob.telemetry.histogram.out/1` | `runs` | `frob telemetry histogram r:19e2 --channel tilt --json` |
| `telemetry.inspect-pixel` | `telemetry:read` | `frob.telemetry.inspect-pixel.out/1` | `diagnose` | `frob telemetry inspect-pixel r:19e2 --world 311 --at failure --pixel 412,230 --json` |
| `telemetry.query` | `telemetry:read` | `frob.telemetry.query.out/1` | `diagnose` | `frob telemetry query r:19e2 --stream contact --from 10s --to 12s --json` |
| `telemetry.stills` | `telemetry:read` | `frob.telemetry.stills.out/1` | `simulate`, `places` | `frob telemetry stills r:19e2 --world 311 --at failure --overlay contact-cones,film --json` |
| `telemetry.summary` | `telemetry:read` | `frob.telemetry.summary.out/1` | `simulate`, `learn` | `frob telemetry summary r:19e2 --budget 600 --json` |

The remaining fields follow by rule: `input_schema` is `frob.<verb name>.in/1`; `refusals` is the chapter list of
the verb's block; `idempotent` is `true` for `read` and `admin` and `false` for `run` and `mutate` (which take an
idempotency key); `cost_model` is `static: …` for `read`, `mutate` and `admin` and the `--estimate` cost model for
`run`; a `run` row carries `wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }`; `owner` is the
crate of the block; `purpose` is the Purpose column.

One entry in full (the others follow field for field):

```toml
[[verb]]
name = "telemetry.divergence"
cluster = "telemetry"
kind = "read"
tier = "S"
capabilities = ["telemetry:read"]
input_schema = "frob.telemetry.divergence.in/1"
output_schema = "frob.telemetry.divergence.out/1"
refusals = ["FROB-REF-894"]
idempotent = true
cost_model = "static: answered from the store or by a small CPU computation; no job"
first_milestone = "M2"
task_clusters = ["runs", "repair"]
examples = ["frob telemetry divergence r:19e2 r:5c11 --json"]
owner = "frob-chronicle"
purpose = "First divergence between two runs, in exact mode along the digest chain or in tolerance mode"
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

- Nine verbs, all `read`. Paginated answers are pinned to a committed frontier; a cursor used with changed filters
  is refused (`FROB-REF-806`) and one whose frontier was collected is refused (`FROB-REF-807`).
- `telemetry.export-html` belongs to this cluster; the viewer it packages is specified with the surface. One HTML
  file of at most 15 MB (TARGET), no network fetch.
- `telemetry.inspect-pixel` exists so that an agent that cannot see still reads every overlay: it returns the
  quantity under a pixel with its unit, channel and world address.

## Acceptance criteria

- [ ] `registries/verbs.toml` holds exactly nine entries with `cluster = "telemetry"` (9 `read`), each with every
      field of the entry schema, an example and a task-cluster list; the alias lint finds no second spelling.
- [ ] `frob-surface-diff parity --cluster telemetry` exits 0 in process (`frob-mcp` over stdio) and through `frobd
      --mcp`: registry, CLI and MCP each count 9, and every difference list is empty apart from the harness's closed
      exemptions.
- [ ] Every verb has an end-to-end script: `scripts/e2e/verbs_telemetry.sh` runs each verb of the cluster (nine),
      and from a verb's first milestone a capability script of its implementing bead exists (`frob-surface-diff
      e2e-coverage`).
- [ ] `frob <verb> --help --json` equals `fixtures/golden/verbs/telemetry/<verb>.help.json` byte for byte for each
      of the nine, on two runs, debug and release, Linux and macOS; a planted renamed output property is classified
      `breaking` and refused.
- [ ] A verb not yet implemented answers `FROB-REF-724`, exit 5, with one envelope on both surfaces; an implemented
      verb runs its registry example with the result its implementing bead specifies; a misspelt parameter is
      `FROB-REF-802`, exit 3.
- [ ] The generated refusal table lists for each verb exactly its chapter codes plus the codes appended for its
      kind, and every code exists in `registries/refusals.toml`.
- [ ] The gate row is recorded with the measured counts by a session that did not write the rows.

## Test plan

### Unit tests

`cargo test -p frob-verbs --test cluster_telemetry` (the real generated table; no mocks; planted registries are TOML
strings labelled `synthetic`):
- `rows_present`: exactly 9 rows in the cluster, with the names of the rows table; a planted extra row or a removed
  row fails.
- `kinds_milestones_tiers`: each row's kind, first milestone and tier equal the rows table (for example
  `telemetry.summary` is `read` at M1).
- `schemas_and_scopes`: input schemas are `frob.<name>.in/1`; output schemas and capabilities equal the
  machine-fields table.
- `refusal_lists`: each row's chapter codes equal its block (`telemetry.divergence`: 894; `telemetry.energy`: none;
  `telemetry.export-html`: 731, 733, 816); the appended codes equal the rule for its kind.
- `mcp_names_one_to_one`: the tool names are the mapped names (`telemetry_divergence`, `telemetry_energy`,
  `telemetry_export_html`), with no collision against any other cluster.
- `task_clusters_and_examples`: every row sits in a task cluster of at most seven tools or is a base tool; each
  example parses against the generated CLI and its command path is the row's own.
- `kind_behaviour`: a verb whose capability is absent returns code 724, exit 5, one envelope.

### End-to-end script and logging

`scripts/e2e/verbs_telemetry.sh` (header: `bead=VERBS-telemetry milestone=M1 labels=tether,verbs,verb:telemetry.*
gates=GATE-TETHER-02 seed=1 needs=cpu,mcp,frobd plants=alias,missing-row,renamed-field`) sources the shared helper
and `scripts/e2e/verbs_lib.sh` and calls `verbs_cluster_conformance telemetry`. Steps: (1) `frob capabilities
--json` and `frob robot-docs --json` list the nine verbs with kind and milestone; (2) `frob-surface-diff parity
--cluster telemetry` over stdio and through `frobd --mcp`, every difference list empty; (3) `frob-surface-diff
snapshot --check --cluster telemetry`; (4) each verb's registry example verbatim, expecting a result (exit 0 or 2),
a job handle (exit 8, then `frob job wait`) or `FROB-REF-724` exactly when the capability report says the verb is
absent; (5) for each implemented verb the same call as an MCP tool, the two envelopes agreeing on `verb`, `ok`,
`exit`, `result`, `refusal`, `evidence`, `bound`, `warnings` and `next_actions`; (6) per verb one call with a
misspelt parameter (`e2e_expect_refusal FROB-REF-802 3`); (7) one envelope per call and no exit code 1.

Log: `target/e2e-logs/verbs_telemetry/<utc>.ndjson` with a transcript beside it. Each `cmd` line has the exact
command, exit code, wall time and the envelope's `evidence` block; each `assert` has observed and expected values
and the tag `--gate GATE-TETHER-02`; the metrics `registry_verbs`, `cli_verbs` and `mcp_tools` are logged against 9
and the difference counts against 0; refusals are logged with code and first repair; a failure logs the first
differing verb and field; the `summary` line gives assertions passed and failed and the gate touched.

### Known answers and negative controls

- Known answers: the cluster has 9 verbs (9 `read`); the tables above. KA-755 (the exit-code table on malformed,
  unknown, over-budget, unscoped and conflicting inputs gives the exit and refusal code of the surface's tables;
  exact) is exercised for every verb by steps (6) and (7).
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

- Kill gate KILL-REPLAYPACK: fallback, `run.export` to the episode file, stills with overlays and the live viewer;
  `telemetry.export-html` then answers the not-implemented refusal.
- Closing rule: the bead closes at M1, when the rows are registered, the surfaces match and every verb passes its
  example or answers the not-implemented refusal. Verbs of this cluster first answer at M1, M2. The per-milestone
  surface beads (BD-137, BD-222) re-run this script as verbs land; that is how GATE-TETHER-02 is re-measured.
- Scope names are proposals (`<cluster>:<read|run|write|admin>` plus the named scopes of the surface) until the
  capability bead BD-258 fixes the scope registry; a `run` or `write` scope of a cluster implies its `read` scope.
  Inputs and outputs above name contents, not flag spellings, except where a flag is quoted; the implementing bead
  fixes the spellings and the snapshot then pins them (additive changes keep the schema's major version; anything
  else raises it).

## References

Appendix J (J.1 entry form; J.2 inventory rows; J.5 task clusters), §29.4, §29.5, §29.7, §29.8, §29.9, §29.10,
§29.12, §20.9, §32.15, §32.4, §32.6, §32.12.3, §32.18, §32.21; GATE-TETHER-02; INV-TETHER-01, -02; KA-755.
