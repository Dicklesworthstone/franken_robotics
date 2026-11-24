---
key: "VERBS-spark"
title: "frob-verbs registry: cluster `spark`, seven rows, with parity, e2e and schema-snapshot checks"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "tether", "spark", "frob-verbs", "solid", "registry", "verb", "test", "review-notes-open"]
parent: "WP-06"
deps: ["BD-022", "NJL-01", "NJL-02", "NJL-04", "STD-E2E"]
needs: []
related: []
gates: ["GATE-TETHER-02"]
plan_refs: ["Appendix J (J.1, J.2, J.3, J.5)", "§21.8", "§21.10", "§21.12", "§21.13", "§21.15", "§21.16", "§32.6", "§32.12.3", "§32.21 (GATE-TETHER-02)"]
---
## Background

FrankenRobotics (a robotics simulation and design system) is reached through the command line `frob`, the daemon
`frobd` and the MCP server `frob-mcp` (the Model Context Protocol, by which an agent harness lists and calls tools).
Every capability is a verb, `cluster.verb`, declared once in `registries/verbs.toml` (crate `frob-verbs`, tier
Solid). The CLI, the daemon protocol, the MCP tools, the schemas, the refusal table and the agent guide are
generated from that file, and gate GATE-TETHER-02 compares the served surfaces with it. The registry holds 244 verbs
in 44 clusters (namespaces): the 243 of the specification and `artifact.import` (VERBS-artifact); the
count of record is NJL-03's.

This bead is the registry of record for the cluster `spark`. The cluster covers devices and kernels: the devices
with their capabilities and fingerprints, arithmetic probes, the CPU-versus-device check, the hardware fingerprint,
the placement plan, precompilation of a shape class's kernels and the binary kernel cache. It has seven verbs (3
`read`, 4 `run`); the handlers live in `frob-cli`, `frob-kernels`. The bead carries every row completely, with the
three checks that keep the rows true: the generated CLI commands and MCP tools match the list; every verb has an
end-to-end script; the schema printed by `frob <verb> --help --json` is stable.

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

In: the seven registry entries of cluster `spark`; the MCP tool names; the cluster's conformance script; the
help-schema snapshots; each verb's answer before its milestone. Out: behaviour (the implementing beads named below);
the entry schema and lints (NJL-01); the conformance harness (NJL-02); the alias lint (NJL-04); task clusters
(NJL-05, NJL-06); generation of the surfaces (the Tether seed beads).

## Technical approach

**The rows.**

| Verb | Kind | First milestone | Tier | Purpose | Defined in |
|---|---|---|---|---|---|
| `spark.cache` | read, admin | M2 | S | The binary kernel cache: contents, hits, eviction | §21.10 |
| `spark.check` | run | M0 | S | The CPU-versus-device check per kernel, with its currency | §21.13 |
| `spark.devices` | read | M0 | S | Devices with capabilities, ceilings and current fingerprint digests | §21.16 |
| `spark.fingerprint` | run | M0 | S | Measure and write the hardware fingerprint record | §21.16 |
| `spark.place` | read | M2 | S | The placement plan and the residency map, before anything runs | §21.12 |
| `spark.precompile` | run | M2 | S | Compile or load the kernel set of a shape class | §21.10 |
| `spark.probe` | run | M0 | S | Arithmetic probes and the boundary corpus on a device | §21.8, §21.15 |

**Contract of each verb.** Evidence labels used below (the `evidence` block of the envelope):
- *operational*: no claim: an operational answer about the session, the store, a job or a device; `evidence.claims`
  is empty.
- *fidelity*: a Fidelity claim (the faster tier tracks the more trusted tier within a frozen envelope); class
  Tested; directional.
- *known answer*: a Known answer where a registered row is recovered within its tolerance; class Tested.

**`spark.cache`** (MCP tool `spark_cache`; owner `frob-kernels`)
- Inputs (BD-173, N21b-13): the default action `read`: a summary, or `--list [--backend cuda|metal] [--class
  <ShapeClassId>]`, or `--verify` (recompute every entry's digest; a corrupt entry is moved aside and reported,
  exit 2); the action `evict` (admin, scope `spark:admin`), selected only with `--action evict` (NJL-01's action
  rule: no per-action flag), with exactly one target, each an input property of that action: `--entry <cache
  key>` (one entry), `--to-bytes <bytes>` (a target size: cheapest to recompile first) or `--all` (every entry);
  under every target a pinned entry is skipped and listed `skipped: in use`. No target, two targets, or a target
  given without `--action evict` is 802 with `pointer` at the property (NJL-01 action rule 3). DESIGN name: the
  one entry's cache key is `entry`, never `key`, whose flag `--key` is the idempotency key's. Paginated
  (`paginated = true`: the `--list` form is a catalogue, and BD-173 pages long listings; the appended list adds
  806 and 807). Conformance build only (absent from a release build's input schema, where it is the
  unknown-parameter refusal 802), for both actions: `--cache-dir <path>` (N21b-13, with the semantics of
  `sim.run`'s `--cache-dir`: the call reads and evicts in that cache directory instead of the default one,
  created empty if absent, so that an end-to-end script works in a scratch directory and never reads or damages
  the user's cache, as N21b-14 and N21b-25 do; a directory outside the session's roots is `FROB-REF-734`).
- Outputs: `frob.spark.cache.out/1`: the binary kernel cache: directory, size against the cap (4 GiB default),
  entries per backend, hits, misses, corrupt entries, evictions, compiler invocations and compile seconds; per
  entry with `--list`; `ok | corrupt` per entry with `--verify`; eviction receipts (keys removed, bytes freed).
  Evidence: *operational*.
- Chapter refusals: `FROB-REF-612` (kernel cache entry corrupt and recompilation unavailable); `FROB-REF-734`
  (`--cache-dir` outside the session's roots, conformance build; N37-REF's list test requires it of every verb
  with a path argument).
- Implemented by: BD-173, N21b-13 (with `--cache-dir`, conformance build).

**`spark.check`** (MCP tool `spark_check`; owner `frob-kernels`)
- Inputs (N21b-32): `--kernel <name>`; `--backend <batch-cpu|gpu-cuda[:n]|gpu-metal>`; `--reference batch-cpu`;
  `--seed N`; `--lanes N`; `--workers N` (the `batch-cpu` worker count, recorded in the receipt; results must not
  depend on it, class D2); idempotency key (`--key`). Options other beads add, listed here so the surface has one
  owner:
  - `--all`, `--step`, `--rollout <steps>`, `--workload Q|H` and `--worlds N` (the size of the frozen ensemble of
    the step and rollout levels) (BD-176);
  - `--sweeps all|batch|slot|capacity|cotenant|shard|workers` (one axis, a comma-separated list, or `all`; a
    misspelt axis is 802) and `--localize` (N21b-33);
  - `--exhaustive` (the kernel over the finite input domain its manifest declares; a kernel with no declared
    domain is 802; N21b-37);
  - `--footprint-audit` (the dynamic footprint cross-check; `batch-cpu` only, another backend is 802; N21a-06);
  - `--decisions --pair <A>:<B> [--ensemble <hash>]` (BD-177: the decision-level equivalence test of a pair of
    placements registered in `registries/placement_pairs.toml`, with `--workload`); the pair forms
    `solve-path:dense,ancestor` (N16c-38, M2) and `solve-path:dense,tree` (N16a-24, M6), each naming two step
    plans on the one backend given by `--backend` that differ only in `solve_path`; and `--battery standard`
    (N16c-38: the standard battery's frozen addresses for the workload, with its frozen baseline controllers, as
    the ensemble in place of `--ensemble`);
  - `--invariance [--axes all|…] [--episodes N] [--cotenancies N]` and `--invariance --report` (the stored
    record read without running, still under a key) (BD-181; defaults 1,000 episodes and 100 co-tenancies).
    `--axes` takes one axis, a comma-separated list, or `all` (every registered axis that applies to the
    backend); the axes are BD-181's `alone`, `cotenancy`, `slot`, `capacity`, `shard` and `jacobian`, and
    `solve-queues` (N16c-15, M2: each co-tenancy repeated with the row-class solve queues on, `--profile
    lockstep` only, at queue capacities drawn per co-tenancy by an address-keyed stream, every world's digest
    chain compared with its masked `alone` run; GATE-KIN-08's digest clause runs it with `cotenancy`, as
    `--axes cotenancy,solve-queues`). A misspelt axis is 802; `solve-queues` in a build without N16c-15 is
    absent from `all` and named alone answers 724 naming it (convention 5);
  - `--conditions [--plan <id>]` (the eight batch-independence conditions, N35-14);
  - `--specialized <KernelId>` (M4, N21c-19: the bit-identity fence of a specialized kernel against its generic
    kernel on 10⁶ states; `FROB-REF-724`, exit 5, while N21c-19 is not in the build or after KILL-SPECIALIZE has
    fired);
  - `--emit-receipts <dir>` (M0 only, until `artifact.read` exists, N21a-02: writes each full device receipt as
    a file; the directory is resolved under the session's roots by the path jail, `FROB-REF-734`);
  - conformance build only (absent from a release build's input schema, where it is the unknown-parameter
    refusal 802): `--mutant <name>[:<key>]` (N21b-42).
  The action `list` (kind `read`, scope `spark:read`, no idempotency key, no budget; N21a-04), selected only with
  `--action list` (NJL-01's action rule; the verb has no `--list` flag, which is the unknown-parameter 802),
  prints the kernel library index (name, family, class, tier, status, check currency per device family), paged
  and saying so; `--action list` beside `--kernel` or any run option is 802. DESIGN (round 3): the listing
  takes no key, but `key_optional` is lint `RL-21`'s exemption for a content-addressed `mutate` and
  `session.focus` and fails on a `run` entry; a read action of a run verb (N32a-02's two-kind form, as
  `sense.symmetry` and `design.diff` use) gives the listing no key, no budget and a read scope, which is what
  N21a-04 specifies.
- Outputs: `frob.spark.check.out/1`: the CPU-versus-device check per kernel: claimed and attained class, ulp
  histograms, worst case, the row id, and its currency; with `--decisions` the verdict record (`equivalent`,
  `not-equivalent` with the discordant worlds listed, exit 2, or `insufficient-ensemble`); with `--invariance`
  the per-axis mismatches (exit 2 on any). Evidence: *fidelity*.
- Chapter refusals: `FROB-REF-601` (device unavailable or lost); `FROB-REF-603` (kernel compile failure; compiler
  diagnostics verbatim); `FROB-REF-605` (the CPU-versus-device check is not current for a kernel in the plan);
  `FROB-REF-606` (fingerprint missing or changed since admission, once fingerprints exist); `FROB-REF-613` (under
  `--specialized`: the specialized kernel is not bit-identical to its generic kernel, exit 6 with the first
  differing state; N21c-19, M4); `FROB-REF-734` (`--emit-receipts` outside the session's roots; N37-REF's list
  test requires it of every verb with a path argument).
- Implemented by: BD-176, N21b-32.
- Extended by: BD-176 (`--all`, `--step`, `--rollout`, `--workload`, `--worlds`), BD-177 (`--decisions`,
  `--pair`, `--ensemble`), BD-181 (`--invariance`, `--axes`, `--episodes`, `--cotenancies`, `--report`), N16c-38
  (the pair `solve-path:dense,ancestor`, `--battery standard`), N16a-24 (the pair `solve-path:dense,tree`),
  N16c-15 (the invariance axis `solve-queues`), N21b-33 (the sweeps), N21b-37 (`--exhaustive`), N21a-04 (the action `list`), N21a-06 (`--footprint-audit`),
  N21a-02 (`--emit-receipts`), N35-14 (`--conditions`), N21c-19 (`--specialized`, 613), N21b-42 (`--mutant`,
  conformance build).

**`spark.devices`** (MCP tool `spark_devices`; owner `frob-cli`)
- Inputs (N21c-04): optional `device` (an identifier such as `cuda:0`, `metal:0`, `batch-cpu`); optional `recheck`
  (default true: re-read the cheap identity fields from the live machine before reporting currency).
- Outputs: devices with capabilities, ceilings, state and current fingerprint digests. Evidence: *operational*.
- Chapter refusals: none of its own.
- Implemented by: N21c-04.

**`spark.fingerprint`** (MCP tool `spark_fingerprint`; owner `frob-kernels`)
- Inputs (BD-029): `--device <id>` or `--device all`; `--soak 20m` or `--no-soak`; idempotency key.
- Outputs: the hardware fingerprint record `frob.fingerprint/1`. Evidence: *operational*.
- Chapter refusals: `FROB-REF-601` (a named device cannot be opened); `FROB-REF-606` (fingerprint missing, or
  changed since admission; raised by the consumers of the record, listed here with the verb that repairs it).
- Implemented by: BD-029.

**`spark.place`** (MCP tool `spark_place`; owner `frob-kernels`)
- Inputs (BD-178, N21b-29): `--tier <mode:backend>` (mode `strict`, `prefer` or `mixed-allowed`); the workload,
  `--workload Q|H` or `--design <id> --task <id>`, with `--worlds N`; optional `--require-current-check`,
  `--need-class D0|D1|D2|D3`, `--env <id>`; or the utility-job form `--job <kernel-job> --rows N
  [--inputs-resident]`, which plans one job, prints the cost model's T_dev and T_host intervals and crossover
  (N21b-30) and has no residency map; the two forms together are 802. A misspelt mode or backend is a refusal,
  not a default. Conformance build only (absent from a release build's input schema, where each is the
  unknown-parameter refusal 802): `--withhold-kernel <k>` (the registry overlay that hides one kernel, N21b-29)
  and `--chunk-intervals N` (the arrangement knob that pins the chunk length and overrides the chunk-length rule
  of N21b-22, which owns it; the length in force is a receipt field; N21c-39 uses it to reach
  `device-count-wrap`). Paginated
  (`paginated = true`: BD-178 pages long residency maps; the appended list adds 806 and 807).
- Outputs: `frob.spark.place.out/1`: the plan (a `frob.placement/1` record in plan form, with `chunk_intervals`
  and, when the no-wrap bound shortened it, `chunk_shortened_from` and the binding counter), the residency map,
  the cost model's predictions and `next_actions`. Exit 2 for a plan with findings (a fallback under
  `mixed-allowed`, a moved plan under `prefer`, a check that is not current). Evidence: *operational*.
- Chapter refusals: `FROB-REF-247` (the determinism class required by `--need-class` is not attainable on the
  placement; exit 3); `FROB-REF-601` (device unavailable); `FROB-REF-602` (the memory estimate exceeds the device
  budget at admission); `FROB-REF-604` (strict backend: a stage has no kernel on the requested backend);
  `FROB-REF-605` (the CPU-versus-device check is not current for a kernel in the plan); `FROB-REF-606`
  (fingerprint missing or changed); `FROB-REF-616` (the device lacks an arithmetic feature the kernel declares
  (for example 64-bit integers)). One refusal is a ledger row that no bead numbers by hand: `device-count-wrap`
  (N21-REF's row, family `6xx`, exit 6, retry `after-change`; a device count could wrap within a chunk:
  N21c-39's wide-count rule, refused before any world is stepped, `pointer` the counter, repairs a shorter
  chunk, fewer samples per interval, fewer worlds per bucket). N35-33's allocation run gives it its number
  (619 is its proposal); the row's `refusals` holds the number the ledger assigned, entered in the change that
  registers it, and no bead types it, as `sim.run`'s row does for the same slug (VERBS-sim).
- Implemented by: BD-178, N21b-29. Extended by: N21b-30 (the `--job` form's cost model), N21c-39
  (`device-count-wrap`), N21b-22 (`--chunk-intervals`, conformance build).

**`spark.precompile`** (MCP tool `spark_precompile`; owner `frob-kernels`)
- Inputs (BD-173, N21b-13): a target (positional: a device id of this host such as `cuda:0` or `metal:0`, a
  backend family, or an architecture name such as `sm_89`); `--class <ShapeClassId>`, `--workload Q|H` or
  `--all-registered`; `--offline`; `--target-fingerprint <file>` (a `frob.fingerprint/1` record of the host
  that will load the cache, giving the driver and compiler identity that enter the cache key; required exactly
  when no device of the named architecture is visible, and 802 beside a device id; N21b-12, N21b-13; a `params`
  entry of kind `artifact-input<frob.fingerprint/1>`, see convention 2); budget; idempotency key. Options other
  beads add: `--unit <path>` (N21a-10: compile one dialect unit file for the target through the lint, the
  compiler and the cache; the file is read under the session's roots); `--specialize <ModelId>` (M4, BD-371:
  the specialized kernel set of one survivor design; `FROB-REF-724`, exit 5, while BD-371 is not in the build or
  after KILL-SPECIALIZE has fired, never 613 and never a compile: N21-REF's `refusal_row_before_mechanism`,
  N21c-18). Conformance build only (absent from a release build's input schema, where each is 802):
  `--kernel-set <name>` (N21c-17: a kernel set registered in the pilot registry for the stages it covers,
  recorded in the receipt and the disclosure block); `--cache-dir <path>` (N21b-13, as on `spark.cache`: the
  call compiles into and loads from that cache directory, created empty if absent, and leaves the default
  directory unchanged; a directory outside the session's roots is `FROB-REF-734`, which the row already gets
  from its `params` entry).
- Outputs: `frob.spark.precompile.out/1`: the kernel set compiled or loaded, with cache receipts: per kernel key,
  `hit`, `compiled` or `failed`, compile seconds and entry size; totals; the compiler identity and options used;
  the cache size after; progress with a denominator (kernels done of kernels total). Evidence: *operational*.
- Chapter refusals: `FROB-REF-251` (`--unit`: a lint finding in the unit, exit 3; N21a-10); `FROB-REF-601` (a
  named device is unavailable); `FROB-REF-603` (kernel compile failure; compiler diagnostics verbatim);
  `FROB-REF-606` (the target-fingerprint cases: no record and no visible device of the architecture, a record
  of another architecture, or a compiler identity that differs from the loaded compiler's); `FROB-REF-612`
  (kernel cache entry corrupt and recompilation unavailable, under `--offline`); `FROB-REF-616` (the target lacks
  an arithmetic feature a kernel declares).
- Implemented by: BD-173, N21b-13.
- Extended by: N21a-10 (`--unit`, 251), BD-371 (`--specialize`, M4), N21c-17 (`--kernel-set`, conformance build),
  N21b-13 (`--cache-dir`, conformance build).

**`spark.probe`** (MCP tool `spark_probe`; owner `frob-kernels`)
- Inputs (BD-017): `--device <id|family>`; `--suite ieee|boundary|canary|exact-ops|digest|all` (`exact-ops`:
  every exact-integer operator and multi-limb subprogram on a seeded slab, output digest per device, N21a-17,
  M1; `digest`: `K-x-digest` and the run-digest reduction in slot order and with the slots permuted, N21a-23,
  M2; `all` includes each suite whose bead is in the build); `--seed N`; `--quick` (2²⁴ unary inputs and 10⁶
  pairs per stratum, `partial: true`, never sets `attainable_class`); `--emit-receipts <dir>` (M0 only,
  N21a-02, as for `spark.check`); idempotency key. A suite whose bead is not in the build answers
  `FROB-REF-724`, exit 5. Conformance build only (absent from a release build's input schema, where it is the
  unknown-parameter refusal 802): `--library conformance:folded-sum|folded-prod` (N21b-36: runs the canary
  suite against one of the two folded mutant libraries, `two_sum_folded` or `two_prod_folded`, in place of the
  clean primitives; the selector N21c-30's row `spark.folded-eft` uses; the report carries `library:
  conformance:<name>` and is never citable). The fast-math, contraction-on and reassociate variants are not
  inputs: they are the annexes' `mutant-*` cargo features, built only by the conformance job (N21c-30).
- Outputs: arithmetic probe results and the boundary corpus on the device; the attainable determinism class.
  Exit 2 on any mismatch or non-zero detector. Evidence: *known answer*.
- Chapter refusals: `FROB-REF-601` (device unavailable or lost); `FROB-REF-603` (kernel compile failure,
  diagnostics verbatim); `FROB-REF-606` (the machine's identity changed during the run, N21c-05's emission
  check; the probe is exempt from the admission check, because the fingerprint record cites its report);
  `FROB-REF-609` (a double-float canary failed: run without double-float kernels, labelled; file the compiler
  evidence); `FROB-REF-616` (a probe kernel declares an arithmetic feature the device lacks (for example 64-bit
  integers); raised before launch); `FROB-REF-734` (`--emit-receipts` outside the session's roots).
- Implemented by: BD-017. Extended by: N21a-17 (`exact-ops`), N21a-23 (`digest`), N21a-02 (`--emit-receipts`),
  N21b-36 (`--library`, conformance build). The two-limb reduce of N21c-41 is not a form of this verb (its bead
  uses the annex test harness and `spark.check --kernel K-seg-reduce`, BD-017), so the row has no `--op` or
  `--pairs` option.

**Machine fields of each row.**

| Verb | Capabilities | Output schema | Task clusters | Example |
|---|---|---|---|---|
| `spark.cache` | `spark:read`; `spark:admin` for the action `evict` | `frob.spark.cache.out/1` | `devices`, `housekeeping` | `frob spark cache --list --backend cuda --json` |
| `spark.check` | `spark:run`; `spark:read` for the action `list` | `frob.spark.check.out/1` | `devices` | `frob spark check --kernel K-cay-step --backend gpu-cuda:0 --key ck-1 --json` |
| `spark.devices` | `spark:read` | `frob.spark.devices.out/1` | `devices`, `fleet` | `frob spark devices --json` |
| `spark.fingerprint` | `spark:run` | `frob.fingerprint/1` | `devices` | `frob spark fingerprint --device metal:0 --key fp-1 --json` |
| `spark.place` | `spark:read` | `frob.spark.place.out/1` | `devices`, `fleet` | `frob spark place --tier strict:gpu-cuda --workload Q --worlds 1024 --json` |
| `spark.precompile` | `spark:run` | `frob.spark.precompile.out/1` | `devices` | `frob spark precompile sm_89 --workload Q --key pc-1 --json` |
| `spark.probe` | `spark:run` | `frob.spark.probe.out/1` | `devices` | `frob spark probe --device cuda:0 --key pb-1 --json` |

Output schema names (settled in round 1): a verb's output schema is `frob.<verb name>.out/1` unless its result is a
stored record whose own schema is the answer. `spark.place` answers with `frob.spark.place.out/1`, which carries the
plan as a `frob.placement/1` record beside the residency map and the predictions (N21b-29), so its row names the
verb schema; `spark.fingerprint` answers with the record itself, `frob.fingerprint/1` (BD-029), and keeps it. Both
record names are owned by the format registry (`registries/formats.toml`).

The remaining fields follow by rule: `input_schema` is `frob.<verb name>.in/1`; `refusals` is the chapter list of
the verb's block; `idempotent` is `true` for `read` and `admin` and `false` for `run` and `mutate` (which take an
idempotency key); `cost_model` is `static: …` for `read`, `mutate` and `admin` and the `--estimate` cost model for
`run`; a `run` row carries `wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }`; `owner` is the
crate of the block; `purpose` is the Purpose column. A verb printed with two kinds has `kind` equal to its default
action and an `actions` list for the others, each with its own kind and scope: `spark.cache`: `actions = [{ name
= "evict", kind = "admin", capabilities = ["spark:admin"] }]`; `spark.check`: `actions = [{ name = "list", kind =
"read", capabilities = ["spark:read"], paginated = true }]` (N21a-04 pages the index; N32a-02's action member
`paginated`, lint `RL-23`, gives that action `cursor`, `limit` and `follow` and appends 806 and 807 to it).
`paginated = true` on `spark.cache` and `spark.place`. `params` on `spark.precompile`: `{ target_fingerprint =
"artifact-input<frob.fingerprint/1>" }`. `conformance_only` (N32a-02's field, lint `RL-24`: the input
properties a build without the `conformance` feature removes from the embedded schema): `spark.cache`:
`["cache_dir"]`; `spark.check`: `["mutant"]`; `spark.place`: `["chunk_intervals", "withhold_kernel"]`;
`spark.precompile`: `["cache_dir", "kernel_set"]`; `spark.probe`: `["library"]`; empty on the other two rows.

One entry in full (the others follow field for field):

```toml
[[verb]]
name = "spark.check"
cluster = "spark"
kind = "run"
tier = "S"
capabilities = ["spark:run"]
input_schema = "frob.spark.check.in/1"
output_schema = "frob.spark.check.out/1"
refusals = ["FROB-REF-601", "FROB-REF-603", "FROB-REF-605", "FROB-REF-606", "FROB-REF-613", "FROB-REF-734"]
idempotent = false
cost_model = "estimated: `--estimate` returns CPU core-seconds, GPU seconds, memory and disk with a conformal interval"
first_milestone = "M0"
task_clusters = ["devices"]
examples = ["frob spark check --kernel K-cay-step --backend gpu-cuda:0 --key ck-1 --json"]
owner = "frob-kernels"
purpose = "The CPU-versus-device check per kernel, with its currency"
wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }
actions = [{ name = "list", kind = "read", capabilities = ["spark:read"], paginated = true }]
conformance_only = ["mutant"]
```

**Conventions the rows follow.**

1. The MCP tool name is the verb name with `.` and `-` replaced by `_`. The output on the wire is one envelope
   `frob.envelope/1` with `result` specialized to the output schema; its `evidence` block is a claim product (the
   weakest class, tier and determinism class among the inputs).
2. The generator adds the common input properties of the kind, as NJL-01's kinds table lists them (`run`:
   `sync_wait_ms` from 0 to the row's `wait.mcp_max_ms`, 30,000, with MCP default 5,000; `idempotency_key`, CLI
   `--key`; `budget`; `estimate_only`, CLI `--estimate`; `on_cancel`, `leave` (the default: the job keeps
   running) or `cancel_job`; `envelope_scope`, CLI `--envelope-scope <gate:…|campaign:…|pretrain:…>`, the
   approved compute-envelope scope the job cites (NJL-01; a citation that contradicts the token of the launcher
   the job runs under is 761 with pointer `/envelope_scope`, N35-REF); `mutate`: `idempotency_key` and
   `expected_parent`). `idempotency_key` is never in the JSON-schema `required` array: the annotation
   `x-frob-key: "required"` (`"optional"` on a `key_optional` entry) carries the rule, and the admission
   transaction refuses a missing key with 805 on both surfaces. The surface-wide refusals appended by kind:
   `FROB-REF-720` (scope missing), 723 (token invalid), 724 (not implemented in this build) and 802 (schema
   failure, including an unknown parameter) for every verb; 701 to 704 (budget and admission), 761 (above the
   compute envelope in force), 803 (key reused with different content) and 805 (missing key) for `run`; 725, 803,
   804 (stale parent) and 805 for `mutate`. The properties and codes driven by fields are those of NJL-01's
   variation table: `cursor`, `limit` and `follow` with 806 and 807 for a paginated entry or a paginated action;
   808 (an integer outside ±(2⁵³ − 1)) for an entry whose generated input schema has an `int53` property at any
   depth, whatever its kind (every `run` entry has one, `sync_wait_ms`); `estimate_only` on a non-`run` entry
   with `estimable = true`; no 805 for a `key_optional` entry; the `action` property, with each action's codes,
   for an entry with `actions`; 790 for a non-empty `needs_groups`; 891 for an `artifact-input<F>`,
   `artifact-ref<F>` or `artifact-identity<F>` parameter, and 892 as well for `artifact-identity<F>`; 893 for a
   `foreign-file` parameter; 734 and 828 for a parameter that accepts a path to a file (`artifact-input<F>`,
   `artifact-identity<F>`, `foreign-file`). The later variations of that table (10 to 13) land with their beads
   and are read there, not copied here.
   In this cluster the one input that accepts a path to a record file is `spark.precompile`'s
   `--target-fingerprint`, a `params` entry of kind `artifact-input<frob.fingerprint/1>`, so that row gets 891,
   734 and 828 (its `--unit` file and its conformance-build `--cache-dir` pass the same jail); the output
   directory of `--emit-receipts` on `spark.check` and `spark.probe` and the conformance-build `--cache-dir` of
   `spark.cache` are path arguments whose 734 those rows list in their chapter lists, as N37-REF requires; the
   other inputs are identifiers.
3. Exit codes: 0 done; 2 done with findings; 3 refused input (1xx to 5xx, and 8xx other than conflicts); 4 budget
   (70x and 761); 5 capability (72x, 73x); 6 device (6xx); 7 conflict (803, 804, 809); 8 accepted and still running; 9
   indeterminate; never 1 on purpose.
4. Rows are entered when the cluster is first registered (M0), each with the `task_clusters` of the
   machine-fields table, which is NJL-05's target assignment. No `[[task_cluster]]` table exists at M0: NJL-05
   declares the tables at M1 and seats these rows in them (its staging rule, step 1), and from then
   `task-cluster-mismatch` holds for them. A verb whose handler is not in the build answers one envelope with `FROB-REF-724`, exit 5, on both surfaces, and `frob capabilities --json`
   lists it as absent; a reduced form that lands before the row's `first_milestone` answers and is listed as
   implemented with its form named (NJL-01, NJL-03: the field keeps the appendix's milestone).
5. Options (DESIGN). An option or option value a row lists whose implementing bead is not in the build
   (`--specialize`, `--specialized`, a probe suite of a later bead, the invariance axis `solve-queues` before
   N16c-15) is in the input schema and answers one envelope `FROB-REF-724`, exit 5, naming the option, never the
   schema refusal 802 and never a partial run: N21-REF's `refusal_row_before_mechanism` asserts exactly this for
   `--specialize`. An option marked "conformance build only" (`--mutant`, `--withhold-kernel`,
   `--chunk-intervals`, `--kernel-set`, `--library`, `--cache-dir`; owners N21b-42, N21b-29, N21b-22, N21c-17,
   N21b-36, N21b-13) is named in the entry's `conformance_only` field (N32a-02): every build other than a
   conformance build removes it from the embedded input schema before anything is generated, so a release build
   has no such flag and answers it with the unknown-parameter refusal 802; the help snapshots are of the release
   build.

**Notes for this cluster.**

- Seven verbs. Two have two kinds: `spark.cache` (contents and hits are `read`; eviction is an `admin` action)
  and `spark.check` (the check is `run`; the kernel index, `--action list`, is a `read` action).
- Four verbs first answer at M0 (`devices`, `probe`, `check`, `fingerprint`), the rest at M2.
- `spark.place` answers before anything runs: the placement plan and the residency map, with requested-versus-actual
  receipts per stage.

## Acceptance criteria

- [ ] `registries/verbs.toml` holds exactly seven entries with `cluster = "spark"` (3 `read`, 4 `run`), each with
      every field of the entry schema, an example and a task-cluster list; the alias lint finds no second spelling.
- [ ] `frob-surface-diff parity --cluster spark` exits 0 in process (`frob-mcp` over stdio) and through `frobd
      --mcp`: registry, CLI and MCP each count 7, and every difference list is empty apart from the harness's closed
      exemptions.
- [ ] Every verb has an end-to-end script: `scripts/e2e/verbs_spark.sh` runs each verb of the cluster (seven), and
      from a verb's first milestone a capability script of its implementing bead exists (`frob-surface-diff
      e2e-coverage`).
- [ ] `frob <verb> --help --json` equals `fixtures/golden/verbs/spark/<verb>.help.json` byte for byte for each of
      the seven, on two runs, debug and release, Linux and macOS; a planted renamed output property is classified
      `breaking` and refused.
- [ ] A verb not yet implemented answers `FROB-REF-724`, exit 5, with one envelope on both surfaces; an implemented
      verb runs its registry example with the result its implementing bead specifies; a misspelt parameter is
      `FROB-REF-802`, exit 3; a `run` or `mutate` call without a key is `FROB-REF-805`, exit 3.
- [ ] The generated refusal table lists for each verb exactly its chapter codes plus the codes appended for its
      kind and fields (NJL-01), and every code exists in `registries/refusals.toml`.
- [ ] The two two-kind verbs expose `action`; an `admin` or `run` action without its scope is `FROB-REF-720`,
      exit 5, and the `read` action still answers: with only `spark:read`, `frob spark check --action list
      --json` answers without a key (exit 0, a cursor when the page is full) while `frob spark check --kernel
      K-selftest --backend batch-cpu --key ck-s --json` is 720, and `frob spark cache --action evict --all --json`
      is 720 and evicts nothing. An action is selected only by `--action`: `frob spark check --list --json` and
      `frob spark cache --evict-all --json` are 802.
- [ ] Every option listed in the rows above is a property of the generated input schema (`frob schema <verb>
      --json`) of a release build, except the conformance-build-only ones, which appear exactly in a conformance
      build's schema; a release build refuses each of those with 802. The answer `FROB-REF-724` for an option
      whose bead is absent (convention 5) is asserted from the milestone at which the verb's own handler exists,
      by the per-milestone surface beads that re-run this script (at M0 `spark.precompile` as a whole answers
      724).
- [ ] The gate row is recorded with the measured counts by a session that did not write the rows.

## Test plan

### Unit tests

`cargo test -p frob-verbs --test cluster_spark` (the real generated table; no mocks; planted registries are TOML
strings labelled `synthetic`):
- `rows_present`: exactly 7 rows in the cluster, with the names of the rows table; a planted extra row or a removed
  row fails.
- `kinds_milestones_tiers`: each row's kind, first milestone and tier equal the rows table (for example
  `spark.probe` is `run` at M0).
- `schemas_and_scopes`: input schemas are `frob.<name>.in/1`; output schemas and capabilities equal the
  machine-fields table (`spark.fingerprint` returns `frob.fingerprint/1`; `spark.place` returns
  `frob.spark.place.out/1`, whose `plan` validates against `frob.placement/1`).
- `refusal_lists`: each row's chapter codes equal its block (`spark.cache`: 612, 734; `spark.check`: 601, 603,
  605, 606, 613, 734; `spark.devices`: none; `spark.fingerprint`: 601, 606; `spark.place`: 247, 601, 602, 604,
  605, 606, 616, and the number the ledger assigned to `device-count-wrap`, read from
  `registries/refusal_allocations.toml` by slug, in a registry where that row is entered (none is typed in the
  test); `spark.precompile`: 251, 601, 603, 606, 612, 616; `spark.probe`: 601, 603, 606, 609, 616, 734);
  the appended codes equal the rule for its kind and fields (`spark.cache`'s `evict` action takes the `admin`
  set and its `read` default the paginated set with 806 and 807; `spark.place` paginated, 806 and 807;
  `spark.precompile` from its `params` entry 891, 734 and 828; `spark.check`'s `list` action the paginated
  `read` set, without 805; 808 on every row whose generated schema has an `int53` property, every `run` row
  among them).
- `inputs_by_row`: `spark.check` accepts `--kernel`, `--backend`, `--workers`, `--worlds`, `--exhaustive`,
  `--decisions --pair solve-path:dense,ancestor --battery standard`, `--invariance --axes capacity,cotenancy
  --cotenancies 16`, `--invariance --axes cotenancy,solve-queues`, `--conditions --plan p:1`, `--specialized`
  and the other listed options, and refuses a misspelt sweep axis (`--sweeps slots`) and a misspelt invariance
  axis (`--axes cotenant`) with 802; `spark.check --action list` without a key is accepted, `--action list
  --kernel K-sort` is 802 and `--list` is an unknown parameter (802); `spark.cache --action evict` accepts
  exactly one of `--entry <cache key>`, `--to-bytes <bytes>` and `--all`, while no target, two targets, `--entry`
  without `--action evict`, and the per-action flags `--evict`, `--evict-to` and `--evict-all` are 802;
  `spark.place` accepts the workload form and the `--job --rows [--inputs-resident]`
  form and refuses both together (802); `spark.precompile` accepts `--target-fingerprint`, `--unit` and
  `--specialize`, and `--target-fingerprint` beside the device id `cuda:0` is 802; `spark.probe` accepts the six
  suite values and refuses `--suite digests` (802); `spark.devices` accepts `device` and `recheck`.
- `conformance_options`: in a release build `--mutant` (`spark.check`), `--withhold-kernel` and
  `--chunk-intervals` (`spark.place`), `--kernel-set` and `--cache-dir` (`spark.precompile`), `--cache-dir`
  (`spark.cache`) and `--library` (`spark.probe`) are absent from the input schemas and refused with 802; built
  with the `conformance` feature, each is present, `--library conformance:folded-sum` and `:folded-prod` are
  accepted and `--library conformance:fast-math` is 802; each row's `conformance_only` list equals the
  machine-fields paragraph and passes `RL-24`; the golden help snapshots are the release build's.
- `mcp_names_one_to_one`: the tool names are the mapped names (`spark_cache`, `spark_check`, `spark_devices`,
  `spark_fingerprint`, `spark_place`, `spark_precompile`, `spark_probe`), with no collision against any other
  cluster.
- `task_clusters_and_examples`: every row sits in a task cluster of at most seven tools or is a base tool; each
  example parses against the generated CLI and its command path is the row's own.
- `kind_behaviour`: a `run` row has the six common properties of its kind (`sync_wait_ms`, `idempotency_key`
  with `x-frob-key: "required"` and absent from the schema's `required` array, `budget`, `estimate_only`,
  `on_cancel` with the values `leave` and `cancel_job`, `envelope_scope`) and refuses `sync_wait_ms = 30001`
  with code 802; an `admin` row or action without its scope is refused with code 720; `spark.check`'s `list` action needs only
  `spark:read` and no key, and its MCP annotation `read_only` is false (one action is `run`); a verb whose
  capability is absent returns code 724, exit 5, one envelope.

### End-to-end script and logging

`scripts/e2e/verbs_spark.sh` (header: `bead=VERBS-spark milestone=M0 labels=tether,verbs,verb:spark.*
gates=GATE-TETHER-02 seed=1 needs=cpu,mcp,frobd plants=alias,missing-row,renamed-field`) sources the shared helper
and `scripts/e2e/verbs_lib.sh` and calls `verbs_cluster_conformance spark`. Steps: (1) `frob capabilities --json`
and `frob robot-docs --json` list the seven verbs with kind and milestone; (2) `frob-surface-diff parity --cluster
spark` over stdio and through `frobd --mcp`, every difference list empty; (3) `frob-surface-diff snapshot --check
--cluster spark`; (4) each verb's registry example verbatim, expecting a result (exit 0 or 2), a job handle (exit 8,
then `frob job wait`) or `FROB-REF-724` exactly when the capability report says the verb is absent; (5) for each
implemented verb the same call as an MCP tool, the two envelopes agreeing on `verb`, `ok`, `exit`, `result`,
`refusal`, `evidence`, `bound`, `warnings` and `next_actions`; (6) per verb one call with a misspelt parameter
(`e2e_expect_refusal FROB-REF-802 3`) and, for `run` and `mutate` verbs, one without a key (`e2e_expect_refusal
FROB-REF-805 3`), while `frob spark check --action list --json` (the `read` action) is called without a key and
is expected to answer (exit 0, or `FROB-REF-724` exactly when the capability report lists the action absent) and
`frob spark check --action list --kernel K-sort --json` is expected to be 802; (7) one envelope per call and no
exit code 1; (8) with a token holding only `spark:read`, `frob spark check --action list --json` answers and the
`run` and `admin` actions are `e2e_expect_refusal FROB-REF-720 5` (in a build whose session tokens do not yet carry
scopes, N32b-01, the step is logged `not-run` with that reason).

Log: `target/e2e-logs/verbs_spark/<utc>.ndjson` with a transcript beside it. Each `cmd` line has the exact command,
exit code, wall time and the envelope's `evidence` block; each `assert` has observed and expected values and the tag
`--gate GATE-TETHER-02`; the metrics `registry_verbs`, `cli_verbs` and `mcp_tools` are logged against 7 and the
difference counts against 0; refusals are logged with code and first repair; a failure logs the first differing verb
and field; the `summary` line gives assertions passed and failed and the gate touched.

### Known answers and negative controls

- Known answers: the cluster has 7 verbs (3 `read`, 4 `run`); the tables above. KA-755 (the exit-code table on
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

- Closing rule: the bead closes at M0, when the rows are registered, the surfaces match and every verb passes its
  example or answers the not-implemented refusal. Verbs of this cluster first answer at M0, M2. The per-milestone
  surface beads (BD-022, BD-222) re-run this script as verbs land; that is how GATE-TETHER-02 is re-measured.
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
- Round 3 requests settled here: N21a-04's keyless `--list` became the `read` action `list` (DESIGN above, in
  place of `key_optional`, which RL-21 refuses on a `run` entry); the options of BD-176, BD-177, BD-181,
  N16c-38, N16a-24, N21a-10, N21b-13, BD-371, N21c-17, N21c-19, N21c-39, N21a-17 and N21a-23 are in their rows;
  `spark.precompile` lists 251, 601, 606 and 616 beside 603 and 612.

## References

Appendix J (J.1 entry form; J.2 inventory rows; J.5 task clusters), §21.8, §21.10, §21.12, §21.13, §21.15, §21.16,
§32.4, §32.6, §32.12.3, §32.18, §32.21; GATE-TETHER-02; INV-TETHER-01, -02; KA-755.
