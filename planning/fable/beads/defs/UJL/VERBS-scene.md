---
key: "VERBS-scene"
title: "frob-verbs registry: cluster `scene`, thirteen rows, with parity, e2e and schema-snapshot checks"
type: "task"
priority: 1
milestone: "M2"
labels: ["m2", "tether", "habitat", "frob-verbs", "solid", "registry", "verb", "test"]
parent: "WP-43"
deps: ["BD-022", "NJL-01", "NJL-02", "STD-E2E"]
needs: []
related: ["BD-228", "BD-236", "BD-163", "N19b-32", "BD-398", "BD-165", "BD-227", "BD-291", "N19c-05", "BD-305", "N37-19", "BD-369", "BD-235", "N19a-02", "N19c-41", "BD-222", "BD-312", "BD-409", "NJL-03", "NJL-04", "NJL-05"]
gates: ["GATE-TETHER-02"]
plan_refs: ["Appendix J (J.1, J.2, J.3, J.5)", "§19.3", "§19.4", "§19.5", "§19.6", "§19.7", "§19.8", "§19.11", "§19.12", "§19.13", "§19.14", "§19.15", "§37.6", "§32.6", "§32.12.3", "§32.21 (GATE-TETHER-02)"]
---
## Background

FrankenRobotics (a robotics simulation and design system) is reached through the command line `frob`, the daemon
`frobd` and the MCP server `frob-mcp` (the Model Context Protocol, by which an agent harness lists and calls tools).
Every capability is a verb, `cluster.verb`, declared once in `registries/verbs.toml` (crate `frob-verbs`, tier
Solid). The CLI, the daemon protocol, the MCP tools, the schemas, the refusal table and the agent guide are
generated from that file, and gate GATE-TETHER-02 compares the served surfaces with it. The registry holds 243 verbs
in 44 clusters (namespaces).

This bead is the registry of record for the cluster `scene`. The cluster covers scenes: the catalog, compiling a
pack from source geometry, generating terrain and interiors, importing assets and real places, acceptance, hazards,
clearance fits, retrofits, splits for train and test hygiene, the world bank on a device, probe requests and purging
a private pack. It has thirteen verbs (5 `read`, 6 `run`, 1 `mutate`, 1 `admin`); the handlers live in `frob-scene`.
The bead carries every row completely, with the three checks that keep the rows true: the generated CLI commands and
MCP tools match the list; every verb has an end-to-end script; the schema printed by `frob <verb> --help --json` is
stable.

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

In: the thirteen registry entries of cluster `scene`; the MCP tool names; the cluster's conformance script; the
help-schema snapshots; each verb's answer before its milestone. Out: behaviour (the implementing beads named below);
the entry schema and lints (NJL-01); the conformance harness (NJL-02); the alias lint (NJL-04); task clusters
(NJL-05, NJL-06); generation of the surfaces (the Tether seed beads).

## Technical approach

**The rows.**

| Verb | Kind | First milestone | Tier | Purpose | Defined in |
|---|---|---|---|---|---|
| `scene.accept` | run | M3 | S | The acceptance report of a pack | §19.5 |
| `scene.bank` | read, admin | M3 | S | The resident set on a device, an admission estimate, eviction | §19.15 |
| `scene.compile` | run | M2 | S | A pack from source geometry, with compile receipts | §19.4 |
| `scene.fit` | run | M3 | F | A feasibility witness, an impossibility cut or `unresolved` for a footprint family and a route | §19.11 |
| `scene.gen` | run | M2 | S | A pack from (family, parameters, seed): terrain at M2, interiors at M3 | §19.6, §19.7 |
| `scene.hazards` | read | M3 | S | The hazard registry of an archetype with the provenance of every dimension | §19.8 |
| `scene.import` | run | M3 | F | A pack with a loss report, a scale label and an unknown mask: assets at M3, real places at M5 | §19.13 |
| `scene.list` | read | M2 | S | The scene catalog | §19.3 |
| `scene.probe-request` | read | M5 | F | The measurements that would settle an unresolved answer | §19.13 |
| `scene.purge` | admin | M3 | S | Remove a private scene pack and the lineage closure of everything derived from it | §37.6 |
| `scene.retrofit` | run | M3 | F | The cheapest building amendment that flips a failing verdict, beside the cheapest redesign | §19.12 |
| `scene.show` | read | M2 | S | A manifest, the hazards drawn and the feasibility tag | §19.3 |
| `scene.split` | mutate | M3 | S | A split over scene groups, with feasibility counts | §19.14 |

**Contract of each verb.** Evidence labels used below (the `evidence` block of the envelope):
- *static*: class Tested, tier `static`, determinism `exact` (class D0); `claims` empty unless a certificate is
  attached; `reality: no-claim`.
- *operational*: no claim: an operational answer about the session, the store, a job or a device; `evidence.claims`
  is empty.
- *certificate*: a Proof claim of class Certified where the checker accepted a certificate, otherwise the class of
  the weakest input (the claim product takes the minimum); tier `static`, determinism `exact`; `reality: no-claim`.
- *pass-through*: passes through the claim product of the stored objects it reads and adds no claim of its own; the
  read itself is `exact`.
- *screening*: class Estimated: a screening or planning signal that may discard under the ladder's rules and may not
  establish a comparison.
- *study*: a Design comparison of class Tested (Estimated if any input is `assumed`); tier and determinism the
  weakest of the arms; `provisional` until promoted to the referee.

**`scene.accept`** (MCP tool `scene_accept`; owner `frob-scene`)
- Inputs: scene pack.
- Outputs: the acceptance report: every check with its value and threshold, completeness. Evidence: *static*.
- Chapter refusals: `FROB-REF-312` (a realism claim cites a scene with `assumed` dimensions); `FROB-REF-317`
  (collision-to-visual deviation exceeds the pack threshold with no waiver).
- Implemented by: BD-228.

**`scene.bank`** (MCP tool `scene_bank`; owner `frob-scene`)
- Inputs: device; action `show` or `estimate` (read), `evict` (admin).
- Outputs: the resident set on the device, an admission estimate, the admission ledger; eviction receipts. Evidence:
  *operational*.
- Chapter refusals: `FROB-REF-314` (world-bank admission would exceed the device cap).
- Implemented by: BD-236.

**`scene.compile`** (MCP tool `scene_compile`; owner `frob-scene`)
- Inputs: source geometry; compile options.
- Outputs: a scene pack with eleven stage receipts. Evidence: *static*.
- Chapter refusals: `FROB-REF-301` (source geometry is not watertight or not consistently oriented); `FROB-REF-302`
  (the transform class of an import is not established: a reflection or affine map is not excluded, or winding
  disagrees with the determinant); `FROB-REF-303` (a decode limit is exceeded or a reference does not resolve);
  `FROB-REF-306` (a feature below voxel scale has no overlay, or an overlay index cell is over capacity).
- Implemented by: BD-163.

**`scene.fit`** (MCP tool `scene_fit`; owner `frob-scene`)
- Inputs: footprint family; scene; route.
- Outputs: a feasibility witness path, an impossibility cut, or `unresolved` with what to refine. Evidence:
  *certificate*.
- Chapter refusals: `FROB-REF-305` (a required route, spawn or contact lies in an unknown region); `FROB-REF-308` (a
  clearance query is outside the domain of the complex: the footprint changes shape along the route, a height or
  step capability is missing).
- Implemented by: N19b-32, BD-398.

**`scene.gen`** (MCP tool `scene_gen`; owner `frob-scene`)
- Inputs: family; parameters; seed.
- Outputs: a scene pack identified by (family, parameters, seed): terrain at M2, interiors at M3. Evidence:
  *static*.
- Chapter refusals: `FROB-REF-315` (a movable object has no declared behaviour); `FROB-REF-316` (a surface region
  has no optical card).
- Implemented by: BD-165.

**`scene.hazards`** (MCP tool `scene_hazards`; owner `frob-scene`)
- Inputs: archetype.
- Outputs: the hazard registry of the archetype with the provenance (`cited`, `assumed`) of every dimension.
  Evidence: *pass-through*.
- Chapter refusals: none of its own.
- Implemented by: BD-227.

**`scene.import`** (MCP tool `scene_import`; owner `frob-scene`)
- Inputs: an asset or a scan of a real place.
- Outputs: a scene pack with a loss report, a scale label and an unknown mask. Evidence: *static*.
- Chapter refusals: `FROB-REF-302` (the transform class of an import is not established: a reflection or affine map
  is not excluded, or winding disagrees with the determinant); `FROB-REF-303` (a decode limit is exceeded or a
  reference does not resolve); `FROB-REF-304` (scale authority is below what the task requires); `FROB-REF-832` (an
  import over its size, depth or polygon limit).
- Implemented by: BD-398.

**`scene.list`** (MCP tool `scene_list`; owner `frob-scene`)
- Inputs: optional filters; cursor.
- Outputs: the scene catalog; private packs are absent without the private scope. Evidence: *pass-through*.
- Chapter refusals: none of its own.
- Implemented by: BD-163.

**`scene.probe-request`** (MCP tool `scene_probe_request`; owner `frob-scene`)
- Inputs: an unresolved answer (mission cell or fit).
- Outputs: the measurements that would settle it: box, quantity, accuracy. Evidence: *screening*.
- Chapter refusals: none of its own.
- Implemented by: BD-291, N19c-05.

**`scene.purge`** (MCP tool `scene_purge`; owner `frob-scene`)
- Inputs: a private scene pack.
- Outputs: the pack and the lineage closure of everything derived from it removed; tombstones. Evidence:
  *operational*.
- Chapter refusals: `FROB-REF-813` (a deletion that would remove a pinned object).
- Implemented by: BD-305, N37-19.

**`scene.retrofit`** (MCP tool `scene_retrofit`; owner `frob-scene`)
- Inputs: a failing verdict or a `.fmcw`; the retrofit catalog.
- Outputs: the cheapest building amendment that flips the verdict with the policy frozen, beside the cheapest
  redesign. Evidence: *study*.
- Chapter refusals: `FROB-REF-313` (a retrofit violates a constraint or an install class the mission does not
  permit).
- Implemented by: BD-369.

**`scene.show`** (MCP tool `scene_show`; owner `frob-scene`)
- Inputs: scene pack.
- Outputs: the manifest, the hazards drawn and the feasibility tag. Evidence: *pass-through*.
- Chapter refusals: `FROB-REF-731` (a private object, or anything derived from one, without the private scope and
  flag).
- Implemented by: BD-163.

**`scene.split`** (MCP tool `scene_split`; owner `frob-scene`)
- Inputs: scene groups; split specification.
- Outputs: a `SceneSplit` over scene groups with feasibility counts; building-level holdouts. Evidence: *static*.
- Chapter refusals: `FROB-REF-311` (a scene group straddles a split: Move the whole group).
- Implemented by: BD-235.

**Machine fields of each row.**

| Verb | Capabilities | Output schema | Task clusters | Example |
|---|---|---|---|---|
| `scene.accept` | `scene:run` | `frob.scene.accept.out/1` | `places-check` | `frob scene accept s:kitchen-17 --key acc-1 --json` |
| `scene.bank` | `scene:read` | `frob.scene.bank.out/1` | `housekeeping` | `frob scene bank --device cuda:0 --json` |
| `scene.compile` | `scene:run` | `frob.scene.compile.out/1` | `places` | `frob scene compile stairs.glb --name stairs-a --key sc-1 --json` |
| `scene.fit` | `scene:run` | `frob.scene.fit.out/1` | `places-check` | `frob scene fit --design quad-a --scene s:kitchen-17 --route galley --key fit-1 --json` |
| `scene.gen` | `scene:run` | `frob.scene.gen.out/1` | `places` | `frob scene gen terrain:stairs --difficulty 0.4 --seed 11 --key sg-1 --json` |
| `scene.hazards` | `scene:read` | `frob.scene.hazards.out/1` | `places-check` | `frob scene hazards kitchen-commercial --json` |
| `scene.import` | `scene:import` | `frob.scene.import.out/1` | `places` | `frob scene import galley-scan/ --name galley-real --key si-1 --json` |
| `scene.list` | `scene:read` | `frob.scene.list.out/1` | `orient`, `places`, `batteries` | `frob scene list --json` |
| `scene.probe-request` | `scene:read` | `frob.scene.probe-request.out/1` | `places-check` | `frob scene probe-request a:0e51 --cell c:0204 --json` |
| `scene.purge` | `scene:admin`, `private:read` | `frob.scene.purge.out/1` | `housekeeping` | `frob scene purge s:galley-real --json` |
| `scene.retrofit` | `scene:run` | `frob.scene.retrofit.out/1` | `places-check` | `frob scene retrofit w:3fa9 --key rt-1 --json` |
| `scene.show` | `scene:read` | `frob.scene.show.out/1` | `attack`, `places` | `frob scene show s:kitchen-17 --json` |
| `scene.split` | `scene:write` | `frob.scene.split.out/1` | `places-check` | `frob scene split --family kitchen-commercial/galley --group-by building --key sp-1 --json` |

The remaining fields follow by rule: `input_schema` is `frob.<verb name>.in/1`; `refusals` is the chapter list of
the verb's block; `idempotent` is `true` for `read` and `admin` and `false` for `run` and `mutate` (which take an
idempotency key); `cost_model` is `static: …` for `read`, `mutate` and `admin` and the `--estimate` cost model for
`run`; a `run` row carries `wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }`; `owner` is the
crate of the block; `purpose` is the Purpose column. A verb printed with two kinds has `kind` equal to its default
action and an `actions` list for the others, each with its own kind and scope: `scene.bank`: `evict` (admin).

One entry in full (the others follow field for field):

```toml
[[verb]]
name = "scene.accept"
cluster = "scene"
kind = "run"
tier = "S"
capabilities = ["scene:run"]
input_schema = "frob.scene.accept.in/1"
output_schema = "frob.scene.accept.out/1"
refusals = ["FROB-REF-312", "FROB-REF-317"]
idempotent = false
cost_model = "estimated: `--estimate` returns CPU core-seconds, GPU seconds, memory and disk with a conformal interval"
first_milestone = "M3"
task_clusters = ["places-check"]
examples = ["frob scene accept s:kitchen-17 --key acc-1 --json"]
owner = "frob-scene"
purpose = "The acceptance report of a pack"
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

- Thirteen verbs, the largest registry cluster; agents see them through the task clusters `places`, `places-check`,
  `housekeeping` and `batteries`.
- `scene.bank` has two kinds: the resident set and an admission estimate are `read`; eviction is an `admin` action.
- `scene.purge` removes a private scene pack and the lineage closure of everything derived from it. An object
  derived from a private scene is not listed and is refused without the private scope (`FROB-REF-731`).
- `scene.gen` first answers at M2 for terrain and at M3 for interiors; `scene.import` at M3 for assets and at M5 for
  real places; `scene.probe-request` at M5.

## Acceptance criteria

- [ ] `registries/verbs.toml` holds exactly thirteen entries with `cluster = "scene"` (5 `read`, 6 `run`, 1
      `mutate`, 1 `admin`), each with every field of the entry schema, an example and a task-cluster list; the alias
      lint finds no second spelling.
- [ ] `frob-surface-diff parity --cluster scene` exits 0 in process (`frob-mcp` over stdio) and through `frobd
      --mcp`: registry, CLI and MCP each count 13, and every difference list is empty apart from the harness's closed
      exemptions.
- [ ] Every verb has an end-to-end script: `scripts/e2e/verbs_scene.sh` runs each verb of the cluster (thirteen),
      and from a verb's first milestone a capability script of its implementing bead exists (`frob-surface-diff
      e2e-coverage`).
- [ ] `frob <verb> --help --json` equals `fixtures/golden/verbs/scene/<verb>.help.json` byte for byte for each of
      the thirteen, on two runs, debug and release, Linux and macOS; a planted renamed output property is classified
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

`cargo test -p frob-verbs --test cluster_scene` (the real generated table; no mocks; planted registries are TOML
strings labelled `synthetic`):
- `rows_present`: exactly 13 rows in the cluster, with the names of the rows table; a planted extra row or a removed
  row fails.
- `kinds_milestones_tiers`: each row's kind, first milestone and tier equal the rows table (for example
  `scene.split` is `mutate` at M3).
- `schemas_and_scopes`: input schemas are `frob.<name>.in/1`; output schemas and capabilities equal the
  machine-fields table.
- `refusal_lists`: each row's chapter codes equal its block (`scene.accept`: 312, 317; `scene.bank`: 314;
  `scene.compile`: 301, 302, 303, 306); the appended codes equal the rule for its kind.
- `mcp_names_one_to_one`: the tool names are the mapped names (`scene_accept`, `scene_bank`, `scene_compile`), with
  no collision against any other cluster.
- `task_clusters_and_examples`: every row sits in a task cluster of at most seven tools or is a base tool; each
  example parses against the generated CLI and its command path is the row's own.
- `kind_behaviour`: a `run` row has the five common properties and refuses `sync_wait_ms = 30001` with code 802; a
  `mutate` row has `idempotency_key` and `expected_parent` and refuses a call without the key with code 805; an
  `admin` row or action without its scope is refused with code 720; a verb whose capability is absent returns code
  724, exit 5, one envelope.

### End-to-end script and logging

`scripts/e2e/verbs_scene.sh` (header: `bead=VERBS-scene milestone=M2 labels=tether,verbs,verb:scene.*
gates=GATE-TETHER-02 seed=1 needs=cpu,mcp,frobd plants=alias,missing-row,renamed-field`) sources the shared helper
and `scripts/e2e/verbs_lib.sh` and calls `verbs_cluster_conformance scene`. Steps: (1) `frob capabilities --json`
and `frob robot-docs --json` list the thirteen verbs with kind and milestone; (2) `frob-surface-diff parity
--cluster scene` over stdio and through `frobd --mcp`, every difference list empty; (3) `frob-surface-diff snapshot
--check --cluster scene`; (4) each verb's registry example verbatim, expecting a result (exit 0 or 2), a job handle
(exit 8, then `frob job wait`) or `FROB-REF-724` exactly when the capability report says the verb is absent; (5) for
each implemented verb the same call as an MCP tool, the two envelopes agreeing on `verb`, `ok`, `exit`, `result`,
`refusal`, `evidence`, `bound`, `warnings` and `next_actions`; (6) per verb one call with a misspelt parameter
(`e2e_expect_refusal FROB-REF-802 3`) and, for `run` and `mutate` verbs, one without a key (`e2e_expect_refusal
FROB-REF-805 3`); (7) one envelope per call and no exit code 1.

Log: `target/e2e-logs/verbs_scene/<utc>.ndjson` with a transcript beside it. Each `cmd` line has the exact command,
exit code, wall time and the envelope's `evidence` block; each `assert` has observed and expected values and the tag
`--gate GATE-TETHER-02`; the metrics `registry_verbs`, `cli_verbs` and `mcp_tools` are logged against 13 and the
difference counts against 0; refusals are logged with code and first repair; a failure logs the first differing verb
and field; the `summary` line gives assertions passed and failed and the gate touched.

### Known answers and negative controls

- Known answers: the cluster has 13 verbs (5 `read`, 6 `run`, 1 `mutate`, 1 `admin`); the tables above. KA-755 (the
  exit-code table on malformed, unknown, over-budget, unscoped and conflicting inputs gives the exit and refusal
  code of the surface's tables; exact) is exercised for every verb by steps (6) and (7).
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

- Kill gate KILL-INGEST (real-place ingestion cannot reach a scale label of `witnessed` or better): fallback, a real
  place is authored in the interior grammar from a tape-measure survey and the pack says it is a proxy;
  `scene.import` keeps its asset form.
- Closing rule: the bead closes at M2, when the rows are registered, the surfaces match and every verb passes its
  example or answers the not-implemented refusal. Verbs of this cluster first answer at M2, M3, M5. The
  per-milestone surface beads (BD-222, BD-312, BD-409) re-run this script as verbs land; that is how GATE-TETHER-02
  is re-measured.
- Scope names are proposals (`<cluster>:<read|run|write|admin>` plus the named scopes of the surface) until the
  capability bead BD-258 fixes the scope registry; a `run` or `write` scope of a cluster implies its `read` scope.
  Inputs and outputs above name contents, not flag spellings, except where a flag is quoted; the implementing bead
  fixes the spellings and the snapshot then pins them (additive changes keep the schema's major version; anything
  else raises it).

## References

Appendix J (J.1 entry form; J.2 inventory rows; J.5 task clusters), §19.3, §19.4, §19.5, §19.6, §19.7, §19.8,
§19.11, §19.12, §19.13, §19.14, §19.15, §37.6, §32.4, §32.6, §32.12.3, §32.18, §32.21; GATE-TETHER-02;
INV-TETHER-01, -02; KA-755.
