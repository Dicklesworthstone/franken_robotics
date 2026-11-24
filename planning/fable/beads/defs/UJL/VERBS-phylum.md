---
key: "VERBS-phylum"
title: "frob-verbs registry: cluster `phylum`, seven rows, with parity, e2e and schema-snapshot checks"
type: "task"
priority: 1
milestone: "M2"
labels: ["m2", "tether", "phylum", "frob-verbs", "solid", "registry", "verb", "test", "review-notes-open"]
parent: "WP-43"
deps: ["BD-022", "N17-REF", "N21-REF", "N25-REF", "NJL-01", "NJL-02", "NJL-04", "NJL-05", "STD-E2E"]
needs: []
related: []
gates: ["GATE-TETHER-02"]
plan_refs: ["Appendix J (J.1, J.2, J.3, J.5)", "§17.5", "§17.11", "§17.12", "§17.13", "§17.15", "§17.16", "§32.6", "§32.12.3", "§32.21 (GATE-TETHER-02)", "§16.21", "§21.11", "§25.30"]
---
## Background

FrankenRobotics (a robotics simulation and design system) is reached through the command line `frob`, the daemon
`frobd` and the MCP server `frob-mcp` (the Model Context Protocol, by which an agent harness lists and calls tools).
Every capability is a verb, `cluster.verb`, declared once in `registries/verbs.toml` (crate `frob-verbs`, tier
Solid). The CLI, the daemon protocol, the MCP tools, the schemas, the refusal table and the agent guide are
generated from that file, and gate GATE-TETHER-02 compares the served surfaces with it. The registry holds 244 verbs
in 44 clusters (namespaces): the 243 of the specification and `artifact.import` (VERBS-artifact); the
count of record is NJL-03's.

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
inventory); one bead for all the rows (too large to review).

## Scope

In: the seven registry entries of cluster `phylum`; the options registered with the rows and their gating; the
MCP tool names; the check that the cluster stays at seven verbs; the cluster's conformance script; the
help-schema snapshots; each verb's answer before its milestone. This bead is the one registry bead of the
cluster: the Phylum unit's cluster bead (N17b-12) specified the same seven entries and is folded in here, with
its option table, its seven-verb check and its gated-option tests. Out: behaviour (the implementing beads named
below); the entry schema and lints (NJL-01); the conformance harness (NJL-02); the alias lint (NJL-04); task
clusters (NJL-05, NJL-06); generation of the surfaces (the Tether seed beads); the refusal rows of the chapter
(N17-REF); `opt.archive`, which lives in the `opt` cluster (N17b-18).

Order of work. This is a registry bead: it is data entry that can be done as soon as the registry skeleton
(BD-022) exists, and it depends on no verb's implementation. Every entry exists from M2 with the
not-implemented answer, so the beads that implement the verbs (BD-171, BD-184, N17b-04, BD-287, BD-285, BD-286,
N17b-14, BD-353, BD-342, N17b-18) depend on this bead and fill their entries in; this bead never waits for
them. Its end-to-end script accepts either a result or the not-implemented refusal for each verb, according to
the capability report, and so passes at every stage.

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
- Inputs: `--generator <id or file>` (a generator identity, a specification file, or a committed fixture
  reference `@fixture:<name>`, as for `--split`); `--count <n>`; `--seed <s>`; optional `--start <i>`,
  `--mutate <design>`, `--locate <design>` (support test), `--report`, `--stratify`; `--enumerate` (in place of
  `--seed` and
  `--count`; valid only for a specification marked `enumerable`, one the specification lint has walked
  completely within a cap of 10⁴ derivations; on any other specification it is the appended `FROB-REF-802`,
  exit 3); `--split <spec>` (a split specification, a file or `@fixture:<name>`: generator, split seed, group,
  strata with fractions, seed ranges).
- Outputs: sampled designs with derivations and log-probabilities; rejection rates and the bucket histogram.
  With `--enumerate`: every derivation in canonical derivation order with its exact probability, the distinct
  designs with their summed probabilities, and the exact total, which must be 1/1. With `--split`: per sample
  its orbit key, stratum and side; the split identity, both manifest hashes, counts per side and stratum, the
  excluded candidates with their reasons, and the straddle result over the samples returned (`straddling: []`,
  or the first straddling orbit with both members and the group element that maps one to the other).
  Evidence: *static*.
- Chapter refusals: `FROB-REF-541` (outside generator support: A family-policy score is requested for a design
  outside its training generator's support).
- Implemented by: BD-287 (sampling, `--start`, `--mutate`, `--enumerate`). Extended by: N17a-18 (`--report`,
  `--stratify`), N17a-19 (`--split`), N17a-20 (`--locate`).

**`phylum.gradient`** (MCP tool `phylum_gradient`; owner `frob-phylum`)
- Inputs: `--design`; `--levers` (default: every unlocked continuous lever); `--metric`; `--task`; `--horizon` (in
  control intervals; default the smaller of the task's horizon and the admitted horizon); `--seeds` (default 64);
  `--delta` per lever (default 3% of the bounds range); `--controller` (one policy for the whole stencil);
  `--tier`; `--measure-variance-ratio` (adds the independent arm); `--stencil-only`; `--method paired-fd` (the
  reference) or `adjoint` (Moonshot, M6; refused with `FROB-REF-724`, exit 5, zero world-steps, until N16c-26's
  reverse kernels are built on the tier, and again if KILL-STEPADJOINT fires); with `adjoint`, `--horizon` is at
  most 32 control intervals and `--seeds` is the world count (one world per seed); `--both-sides` (with
  `adjoint`: both one-sided values of a gradient labelled `one-sided`, N16c-26).
- Outputs: paired finite-difference design gradients with intervals, per-pair divergence steps, the
  `truncated-horizon` label where it applies. With `--method adjoint` the output gains `method`, `horizon_used`,
  `gradient` (per parameter, in its tangent coordinates, in metric units per parameter unit), `label` (the
  weakest on the path), `steps_by_label`, `worlds_dropped_nonconverged`, `uninformative`, `agreement`
  (`directions`, `seeds`, `cosine`, `fd_directional`, `adjoint_directional`) and `store_bytes`; a request no
  world could serve returns no gradient with the count, never a zero vector. Evidence: *screening* (the adjoint
  form: class Estimated).
- Chapter refusals: `FROB-REF-540` (gradient unreliable: Non-smooth metric, horizon above the admitted cap, pairs
  diverged early, or an uncontrolled determinism class); `FROB-REF-602` (with `--method adjoint`: the reverse
  sweep's store, K × world-steps per control interval × bytes per world × worlds, exceeds the device budget at
  admission, before any world steps; repairs: fewer worlds, a shorter horizon, the stencil; N16c-26). The
  capability refusal 724 of the adjoint is the appended code every verb carries (NJL-01) and is not repeated in
  the chapter list.
- Implemented by: BD-353.
- Extended by: N16c-26 (`--method adjoint`, `--both-sides`, 602).

**`phylum.map`** (MCP tool `phylum_map`; owner `frob-phylum`)
- Inputs: one of `--campaign <id>`, `--dispatch <id>`, `--map <MapId>`, or `--merge <ref> <ref>…`; optional
  `--cells` (a stratum or an index range), `--top` (elites per stratum); pagination (`paginated = true`: a long
  cell list is paged against a committed frontier, and the generator appends 806 and 807).
- Outputs: the quality-diversity map (`map`, `spec`, `coverage`, `quality`, cells with elites, strata, sources);
  with `--merge`, the merged map written as a content-addressed artifact (the same inputs give the same
  address, in any argument order) and no campaign head moved. A merge of maps with different specifications is
  the appended `FROB-REF-802` with `pointer` the first differing `merge[i]`. Evidence: *pass-through*.
- Chapter refusals: none of its own.
- Implemented by: BD-342, N17b-18 (the store and the campaign head are N17b-29's).

**`phylum.plan`** (MCP tool `phylum_plan`; owner `frob-phylum`)
- Inputs: `--designs <set>` (a design set: design keys, a list file or an archive selector; a workload
  reference `@workload:<name>`, resolved by N17b-03's `frob_phylum::workload::resolve` to its `WorkloadId` and
  design list, as on `phylum.sweep`; a corpus reference `@corpus:<id>`, a frozen corpus or corpus set by name or
  identity, resolved by the loader of the bead that froze it (N17a-30 for the campaign corpus sets); each
  reference echoed with its resolution in the envelope's `bound` block; or a population
  descriptor, `@fixture:<name>` or a path: a file labelled `synthetic` that lists sizes with their link, contact
  and row needs and world weights summing to one, with a total world or design count, so that a hypothetical
  population is planned without compiling it); `--worlds-per-design <W>`; optional `--tier`, `--buckets
  registered|population` (`exact` is accepted as a synonym of `population`, and the envelope prints
  `population`), `--waste-limit` (default the registered 0.25), `--max-buckets <k>` (lowers the cap of eight for
  this call, 1 ≤ k ≤ 8; above 8 is the appended 802, because the cap is not raisable), `--tau <duration>`,
  `--launch <duration>` and `--shares <a₁,a₂,a₃>` (τ, c_L and the stage shares given explicitly, each labelled
  `given` in the envelope's `inputs` in place of HYPOTHESIS or measured; shares that do not sum to one are 802),
  `--bucket-set <c₁,…,c_k>` (with `--buckets registered` only: a candidate set of distinct ascending positive
  degree-of-freedom sizes stands in for the registered set for this call, labelled `given`, every bucket row
  marked `candidate`, nothing registered; with `--buckets population` or malformed, 802), `--training`,
  `--assume <device>` (the target machine).
- Outputs: buckets, padding overhead per stage exponent, bytes per world and in total, the machine that fits it,
  the break-even world count of every bucket boundary, the KILL-BUCKET report (`kill_bucket: { buckets_needed,
  limit: 8, trigger }`; exit 2 with a finding when more than eight buckets are needed to hold Ω₃ ≤ 2); never
  runs physics and opens no backend. Evidence: *static* (throughput and latency fields Estimated).
- Chapter refusals: `FROB-REF-160` (no shape class: A design exceeds the largest admitted bucket and the
  tree-operator route is off); `FROB-REF-740` (population exceeds device: The population's bytes exceed every
  admitted placement); `FROB-REF-741` (bucket fragmentation: More than eight buckets are needed to hold padding
  waste at or below the limit). 160 exits 3; 740 and 741 exit 4 (the `74x` decade, N17-REF and N35-33).
- Implemented by: BD-171.

**`phylum.quotient`** (MCP tool `phylum_quotient`; owner `frob-phylum`)
- Inputs: one of two forms. Design-set form: `--designs <set>` (a design set as on `phylum.plan`, the
  `@workload:` and `@corpus:` references included; N17a-28 passes `@corpus:<set>`), `--task`, `--scenes`,
  optional `--policy`, `--tier`, `--profile`, `--explain <a>,<b>` (the fields that split two classes), `--group
  <obs|mech|file>`.
  Enumeration form: `--sweep <chassis> --slots <slot set> --types <t₁,…,t_k>`, optional `--task`, `--group`;
  when `<chassis>` is a sweep fixture (`@fixture:<name>`) that records its slot set and type list, `--slots` and
  `--types` may be omitted (BD-285), and a fixture that records neither leaves them required (802).
  Paginated (`paginated = true`; a long answer says so and is never cut).
- Outputs: orbit representatives under Γ_obs with the orbit count, and mission-digest classes with the fields that
  split them; similarity classes and the `SavedReport`; for the enumeration form the group order, cycle types,
  raw count kⁿ, orbit count and reduction. Evidence: *static*.
- Chapter refusals: `FROB-REF-161` (group not observation-preserving: A quotient or a weight tie is requested under
  a group that moves a sensor, the payload or the task); `FROB-REF-162` (on the memoized path: a cached
  mission-digest or quotient result with no recorded dependency digest, or whose shadow recompute disagreed;
  BD-285, with N17a-26's memo entries of the mission digest).
- Implemented by: BD-285.

**`phylum.reuse`** (MCP tool `phylum_reuse`; owner `frob-phylum`)
- Inputs: one of two forms. Edit form: `--design <d>` and `--edit <kind | edit ref | patch>` with the edit's own
  arguments (for `swap-module`: `--path`, `--to <module>`). Lineage form: `--from <d:…> --to <d:…>` (two
  versions of one lineage). Optional `--kinds` (node kinds: analysis, synthesis, step-plan, kernel, scene,
  episode, comparison), `--task`; paginated (`paginated = true`).
- Outputs: which cached results survive, which are invalidated, and the field that forced each invalidation.
  Evidence: *pass-through*.
- Chapter refusals: `FROB-REF-162` (reuse without a dependency digest: A cached result has no recorded dependency
  digest, or the shadow recompute disagreed).
- Implemented by: BD-286, N17b-14.

**`phylum.sweep`** (MCP tool `phylum_sweep`; owner `frob-phylum`)
- Inputs: `--designs` (identities, names at a version, an archive reference such as `@archive:gen-14`, a workload
  reference `@workload:<name>` (N17b-03), a corpus reference `@corpus:<id>` as on `phylum.plan`);
  `--worlds-per-design`; `--task`; `--tier`; `--controller` (one shared baseline controller or
  policy); `--seed` (the root of the world addresses); `--profile`; `--waste-limit` (default the registered
  padding-waste limit, 25%; repair 2 of `FROB-REF-741` is applied here; a value above the registered limit is
  accepted only explicitly and echoed in `execution` and `bound`); `--campaign <id>` (the campaign whose map this
  dispatch joins, N17b-29); `--map-spec <MapSpecId>` (must equal the campaign's specification, else the appended
  `FROB-REF-802` before any world runs, nothing charged); `--fill licensed` (default off; N25e-07, M3: licensed
  monotone fills are attempted and each fill appears as a `proved` point with its licence and proof reference;
  without it every design neither evaluated nor covered by a proof is `unresolved`, never interpolated); the
  gated options of the table below; budget.
- Outputs: one envelope with a provisional scorecard per design, cohort count, padding overhead, the map delta,
  referee coverage; unevaluated designs marked `unresolved`. Exit 0 when every design is scored with nothing
  flagged, 2 with unresolved designs or flagged worlds. Evidence: *run*.
- Chapter refusals: `FROB-REF-160` (no shape class: A design exceeds the largest admitted bucket and the
  tree-operator route is off; per design, inside `results`); `FROB-REF-740` (population exceeds device: The
  population's bytes exceed every admitted placement); `FROB-REF-741` (bucket fragmentation: More than eight
  buckets are needed to hold padding waste at or below the limit); `FROB-REF-852` (with `--fill licensed` on a
  sweep whose verdicts come from dynamic rollouts: a monotone transport that lacks a side condition, since a
  rollout verdict has no monotonicity licence by default; pointer at the swept parameter; repairs: evaluate the
  missing values, supply the monotonicity proof and register a licence, accept `unresolved`; exit 3; N25e-07).
  740 and 741 exit 4 at admission.
- Implemented by: BD-184, N17b-04, N17b-29 (`--campaign`, `--map-spec`, the dispatch map and its delta).
  Extended by: N25e-07 (`--fill licensed`, 852), N16c-15 (`--solve-queues`), N21b-17 (`--tables`), BD-285
  (`--quotient`), BD-356 (`--family-lanes`), N17a-16 (`--specialize-survivors`), N17a-10 (`--capacity-step`).

**Machine fields of each row.**

| Verb | Capabilities | Output schema | Task clusters | Example |
|---|---|---|---|---|
| `phylum.generate` | `phylum:read` | `frob.phylum.generate.out/1` | `families` | `frob phylum generate --generator gen:quad-1 --count 64 --seed 7 --json` |
| `phylum.gradient` | `phylum:run` | `frob.phylum.gradient.out/1` | `families` | `frob phylum gradient --design quad-a --levers thigh --metric tracking-error --task stairs-0.4 --key pg-1 --json` |
| `phylum.map` | `phylum:read` | `frob.phylum.map.out/1` | `families` | `frob phylum map --campaign camp:12 --top 3 --json` |
| `phylum.plan` | `phylum:read` | `frob.phylum.plan.out/1` | `families`, `fleet` | `frob phylum plan --designs @archive:gen-14 --worlds-per-design 16 --json` |
| `phylum.quotient` | `phylum:read` | `frob.phylum.quotient.out/1` | `families`, `symmetry` | `frob phylum quotient --designs @archive:gen-14 --task stairs-0.15 --json` |
| `phylum.reuse` | `phylum:read` | `frob.phylum.reuse.out/1` | `families` | `frob phylum reuse --design quad-a --edit swap-module --path /limbs/of/of --to std/leg-3r-qdd-l.fmodule --json` |
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
examples = ["frob phylum gradient --design quad-a --levers thigh --metric tracking-error --task stairs-0.4 --key pg-1 --json"]
owner = "frob-phylum"
purpose = "Paired finite-difference design gradient with intervals and divergence steps"
wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }
```

**Conventions the rows follow.**

1. The MCP tool name is the verb name with `.` and `-` replaced by `_`. The output on the wire is one envelope
   `frob.envelope/1` with `result` specialized to the output schema; its `evidence` block is a claim product (the
   weakest class, tier and determinism class among the inputs).
2. The generator adds the common input properties of the kind, as NJL-01's kinds table lists them. `run`:
   `sync_wait_ms` (from 0 to the row's `wait.mcp_max_ms`, at most 30,000; MCP default 5,000; CLI default to
   completion), `idempotency_key` (CLI `--key`), `budget`, `estimate_only` (CLI `--estimate`), `on_cancel`
   (`leave`, the default, or `cancel_job`) and `envelope_scope` (CLI `--envelope-scope
   <gate:…|campaign:…|pretrain:…>`, the approved compute-envelope scope the run belongs to); `mutate`:
   `idempotency_key` and `expected_parent`; `read` and `admin`: none. `idempotency_key` is in no JSON-schema
   `required` array: the annotation `x-frob-key: "required"` carries the rule, and the admission transaction
   refuses a missing key with 805 on both surfaces. The generator appends the surface-wide refusals by kind:
   `FROB-REF-720` (scope missing), 723 (token invalid), 724 (not implemented in this build) and 802 (schema
   failure, including an unknown parameter) for every verb; 701 to 704 (budget and admission), 761 (above the
   compute envelope in force), 803 (key reused with different content) and 805 (missing key) for `run`; 725,
   803, 804 (stale parent) and 805 for `mutate`. What fields and registry tables add is NJL-01's variation table,
   the one list of it: 806 and 807, with `cursor`, `limit` and `follow`, for a paginated read (variation 1);
   `estimate_only` on a non-`run` entry with `estimable = true` (2); `x-frob-key: "optional"` and no 805 on a
   `key_optional` entry (3); 808 for an entry with an `int53` property at any depth of its input schema, whatever
   its kind, so every `run` row through `sync_wait_ms` (4); `action` for an `actions` list (5); 790 for a
   non-empty `needs_groups` (6); 891 to 893 for artifact `params` (7, 8); 734 and 828 for a parameter that
   accepts a path to a file (`artifact-input<F>`, `artifact-identity<F>`, `foreign-file`) (9). Variations 10 to
   16 are read from NJL-01's table with the beads that land them.
3. Exit codes: 0 done; 2 done with findings; 3 refused input (1xx to 5xx, and 8xx other than conflicts); 4 budget
   and capacity at admission (70x, 74x and 76x, so this cluster's 740 and 741 and the appended 761); 5 capability
   (72x, 73x, 79x); 6 device (6xx); 7 conflict (803, 804, 809); 8 accepted and still running; 9 indeterminate;
   never 1 on purpose (the sub-family rule of N35-33).
4. Rows are entered when the cluster is first registered (M2); each row's tool is added to the task-cluster
   tables its `task_clusters` names in the same change (NJL-05's staging rule). A verb whose handler is not in
   the build answers one envelope with `FROB-REF-724`, exit 5, on both surfaces, and `frob capabilities --json`
   lists it as absent; a reduced form that lands before the row's `first_milestone` answers and is listed as
   implemented with its form named (NJL-01, NJL-03: the field keeps the appendix's milestone).

**Notes for this cluster.**

- Exactly seven verbs, the limit this chapter sets for its cluster: specialization of survivors is an option of
  `phylum.sweep`, not a verb, and stays refused until the gate that funds it passes. How the limit is held
  (DESIGN): the surface's limit of seven is on task clusters, the tools an agent lists at a time, and the seven
  `phylum` verbs are exactly the task cluster `families`, which is therefore full. An eighth `phylum` verb would
  overflow it, so this bead's count test fixes the cluster at seven rows, and a later mechanism that needs a
  new population verb has to become an option of an existing verb or go to another cluster.
- The task cluster of the seven rows is `families`, as the registry appendix assigns it (with `fleet` for
  `phylum.plan` and `symmetry` for `phylum.quotient` in addition); no row uses another task-cluster name.
- Capability scopes are `phylum:read` and `phylum:run`; a row lists no scope of another cluster.
- Options registered with rows and gated: `--limb-kernels`, `--solve-queues`, `--specialize-survivors`,
  `--quotient`, `--family-lanes`, `--tables shared` on `phylum.sweep`; `--method adjoint` and `--both-sides` on
  `phylum.gradient` (Moonshot). Each answers the not-implemented refusal until its gate is recorded as passed.
  The full table of options:

| Option | On | Meaning | Until its gate passes |
|---|---|---|---|
| `--limb-kernels` | `phylum.sweep` | Run kinematics and dynamics terms through limb-archetype kernels, bit-identical to the table-driven kernel | Refused with the capability refusal `FROB-REF-724` |
| `--solve-queues off\|auto\|<q0,q1,q2>` | `phylum.sweep` (and `sim.run`) | Regroup worlds by solver work in row-class queues, bit-identical to the masked path; explicit capacities are multiples of 32, each at most the world count; admitted only with `--profile lockstep` (on `referee` or under `strict` it is 802, pointer `execution.solve_queues`) (N16c-15) | `auto` and explicit capacities refused the same way; `off` is the default |
| `--tables shared\|plain` | `phylum.sweep` | The shape class with or without the design-family constant tables shared by content hash, per-design parameter columns; equal per-world digests either way (N21b-17, M4) | `shared` refused with `FROB-REF-724` until the mechanism is funded and built (the pilot's verdict, KILL-BUCKET not fired); `plain`, the default, always accepted |
| `--fill licensed` | `phylum.sweep` | Attempt licensed monotone fills; each appears as a `proved` point with its licence (N25e-07, M3) | Off by default; `FROB-REF-724` until N25e-07's handler is in the build; on a rollout sweep `FROB-REF-852` |
| `--specialize-survivors` | `phylum.sweep` | Per-design specialized kernels for long-running survivors (the promotion mechanism is N17a-16) | Off and refused until the gate that funds specialization passes |
| `--quotient` | `phylum.sweep` | Dispatch one representative per orbit and per mission-digest class and expand the results afterwards (wired by BD-285, M3) | Refused with `FROB-REF-724` until BD-285 lands; under the fallback of the quotient kill gate it dispatches orbit representatives only |
| `--family-lanes` | `phylum.sweep` (and `sim.run`) | Run worlds that share tree, sensor graph and contact mode in design-family lanes, bit-identical to the general masked lane (BD-356, M4) | Refused with `FROB-REF-724` until the lanes land; rejected again if the family-lane kill gate fires |
| `--capacity-step 1\|max` | `phylum.sweep` | Test option of the digest gate: `1` places every design in the next larger registered bucket than its native one, `max` in the largest bucket that admits it, raising contact and row capacities where no larger bucket exists (N17a-10); written into the manifest and the `StepPlanId`, so such a sweep never shares an identity with an ordinary one | Conformance-build-only (feature `conformance`), as the same input of `sim.run` (BD-170): absent from a release build's schema, where it is the unknown-parameter refusal `FROB-REF-802`, exit 3 |
| `--method adjoint` | `phylum.gradient` | A Moonshot alternative through the converged contact solve; the paired finite-difference method stays the reference (N16c-26, M6) | Refused with `FROB-REF-724` on every tier without the reverse kernels |
| `--both-sides` | `phylum.gradient` | With `adjoint`: both one-sided values of a `one-sided` gradient (N16c-26) | Refused with `adjoint` |
| `--locate`, `--report` | `phylum.generate` | Support test (N17a-20); rejection rates and bucket histogram (N17a-18) | — |
| `--stratify` | `phylum.generate` | Stratified sampling by shape class with quotas (N17a-18) | — |
| `--enumerate` | `phylum.generate` | Every derivation of an `enumerable` specification with exact probabilities and their total (BD-287) | — (802 on a specification not so marked) |
| `--split <spec>` | `phylum.generate` | Held-out designs by seed and stratum, whole Γ_obs orbits on one side, with the straddle result (N17a-19) | — |
| `--max-buckets`, `--tau`, `--launch`, `--shares`, `--bucket-set` | `phylum.plan` | A lowered bucket cap, given unit costs and shares, a candidate bucket set (BD-171) | — |
| `--merge` | `phylum.map` | Merge two or more maps of one specification | — |

- No option exists as a separate verb. An unknown option is the schema refusal `FROB-REF-802`, exit 3 (a misspelt
  parameter is a refusal, not a default).
- `phylum.plan` additionally prints, for the population in hand, the memory table by machine, the bound of the
  row-class solve queues and the limb-archetype coverage; those fields are added by the beads that own the
  mechanisms and are optional in the schema until then. Its option `--buckets registered|population`, the
  waste limit, `--training` and `--assume` are specified with the verb (BD-171).
- `phylum.sweep` carries its cost estimate from `sim.estimate` on the bucket plan; that is the `--estimate` cost
  model of the row.
- Spellings settled with the implementing beads (round 1): `phylum.generate` takes `--generator` and `--count`
  (BD-287; the former example's positional generator and `--n` were no input); `phylum.gradient` takes
  `--design` and requires `--metric` (BD-353); `phylum.map` and `phylum.sweep` take `--campaign` (N17b-18,
  N17b-29); `phylum.reuse` takes `--design --edit` or `--from --to` (N17b-14; the former `--since` was no input).
  In the reuse edit form `--to` is the swap-module edit's target module; in the lineage form it is the later
  version; the presence of `--edit` or `--from` tells the forms apart. The three paginated reads page with the
  common properties of NJL-01, `--cursor`, `--limit` and `--follow`; no row has a `--page-size`, which is the
  schema refusal 802 (N17b-18's script pages `phylum.map` with `--limit 50`).

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
      kind and fields (NJL-01), and every code exists in `registries/refusals.toml`.
- [ ] The script's run leaves a receipt with the measured counts (registry, CLI and MCP each 7; every difference
      list 0) and the controls caught. This bead records no gate value: the per-milestone surface beads (BD-222
      at M2, then BD-312 and BD-376) record GATE-TETHER-02's part for their milestone from such a receipt with
      `cargo run -p frob-conformance --example record_gate -- --gate GATE-TETHER-02 --receipt <hash> --part <M>`,
      in a session that did not write the rows (N38b-19).
- [ ] The build fails if an eighth verb is added to the cluster or if a `phylum` verb gains a second spelling.
- [ ] `frob schema phylum.<verb>` prints both schemas for all seven; the generated refusal table lists the
      chapter's seven codes (160, 161, 162, 540, 541, 740, 741) on the verbs of the rows table, and the two codes of
      other chapters the rows carry: 602 (Spark, on `phylum.gradient`) and 852 (Augur, on `phylum.sweep`).
- [ ] Every gated option answers `FROB-REF-724` until its gate is recorded PASS, and no option exists as a
      separate verb.
- [ ] The agent documentation generated from the registry contains the table of the seven verbs with kinds and
      milestones.
- [ ] Each implemented verb passes its own bead's tests through the generated surface.

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
- `refusal_lists`: each row's chapter codes equal its block (`phylum.plan`: 160, 740, 741; `phylum.sweep`: 160,
  740, 741, 852; `phylum.generate`: 541; `phylum.quotient`: 161, 162; `phylum.reuse`: 162; `phylum.gradient`:
  540, 602; `phylum.map`: none); the appended codes equal the rule for its kind and fields (NJL-01), and 724 is
  in no chapter list.
- `cluster_has_exactly_seven`: the registry loader counts seven; a fixture registry with eight (a planted
  `phylum.specialize`) fails with the cluster-limit error.
- `kinds_and_milestones`: plan, generate, quotient, reuse, map are `read`; sweep and gradient are `run`;
  milestones M2 (plan), M2 (sweep), M3 (generate), M3 (quotient), M3 (reuse), M4 (gradient), M4 (map).
- `unimplemented_answers_724`: with the gradient capability absent, `phylum.gradient` returns `ok: false`, code
  `FROB-REF-724`, exit 5.
- `options_gated`: `--limb-kernels`, `--solve-queues auto`, `--specialize-survivors`, `--quotient`,
  `--family-lanes` and `--tables shared` on `phylum.sweep` and `--method adjoint` (with or without
  `--both-sides`) on `phylum.gradient` → `FROB-REF-724`, exit 5, while their mechanisms are absent; `--tables
  plain` and `--solve-queues off` are accepted; an unknown option → the schema refusal, exit 3.
- `inputs_by_row`: `phylum.sweep` accepts `--controller`, `--seed`, `--profile`, `--waste-limit`, `--campaign`,
  `--map-spec`, `--fill licensed`, `--tables plain|shared`, and `--solve-queues` only with `--profile lockstep`
  (802 otherwise); `--capacity-step 1` and `max` are in the input schema of a build with the feature
  `conformance` and absent from a release build's, where each is 802; `phylum.sweep` and `phylum.plan` accept
  `--designs @workload:h2h-512` and `--designs @corpus:<id>`, and `phylum.quotient` accepts
  `--designs @corpus:<id>`; `phylum.map` accepts exactly one of `--campaign`, `--dispatch`, `--map`, `--merge`
  (two given together is 802); `phylum.reuse` accepts the edit form or the lineage form and refuses both together (802);
  `phylum.quotient` accepts the design-set form or the `--sweep` form, and `--sweep @fixture:<name>` without
  `--slots` and `--types`; `phylum.generate` accepts `--generator @fixture:<name>`, `--enumerate` without
  `--seed` and `--count`, `--split @fixture:<name>` and `--locate <design>`; `phylum.plan` accepts a
  population descriptor for `--designs`, `--buckets exact` (printed `population`), `--max-buckets 1` to 8, `--tau`,
  `--launch`, `--shares` and
  `--bucket-set 16,32,64`, and refuses with 802 `--max-buckets 9`, `--shares 0.2,0.3,0.4`, `--bucket-set
  32,16,64` and `--buckets population --bucket-set 16,32,64`; `phylum.gradient` accepts `--both-sides`; a
  planted `--since` on `phylum.reuse` or `--n` on `phylum.generate` is 802.
- `paginated_rows`: `phylum.map`, `phylum.quotient` and `phylum.reuse` carry `paginated = true` and 806, 807.
- `mcp_names_one_to_one`: the tool names are the mapped names (`phylum_generate`, `phylum_gradient`, `phylum_map`,
  `phylum_plan`, `phylum_quotient`, `phylum_reuse`, `phylum_sweep`), with no collision against any other cluster.
- `task_clusters_and_examples`: every row sits in a task cluster of at most seven tools or is a base tool; each
  example parses against the generated CLI and its command path is the row's own.
- `kind_behaviour`: a `run` row has the six common properties of conventions item 2, `idempotency_key` in no
  `required` array and annotated `x-frob-key: "required"`, `on_cancel` admitting exactly `leave` and
  `cancel_job`, and refuses `sync_wait_ms = 30001` with code 802; a
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
FROB-REF-805 3`); (7) one envelope per call and no exit code 1; (8) `frob schema phylum.<verb> --json` for the
seven verbs, both schemas printed; (9) each gated option once on its verb, expecting `FROB-REF-724` until the
capability report says its gate has passed, and, on a release build, `phylum.sweep` with `--capacity-step 1`
expecting `FROB-REF-802`, exit 3 (the conformance-build-only input); (10) each verb once with a minimal valid
input on tier `batch-cpu`, and, where the MCP server is present, `phylum_plan` called as a tool with the CLI and MCP
envelopes equal. This is the one end-to-end script of the cluster; no `phylum_verb_cluster.sh` exists beside it.

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
  verb answering with two envelopes or exit code 1 fails step (7). Also: a registry with a `phylum.specialize`
  verb must fail the limit check; an option accepted silently before its gate must fail `options_gated`.
- Invariants carried: INV-TETHER-01 (every surface is generated from the registry; nothing exists on one surface and
  not the other, apart from declared per-surface defaults and the unknown-verb case) as step (2); INV-TETHER-02
  (exactly one envelope per invocation with `--json`) as step (7). Determinism lanes: entries, tool definitions and
  snapshots byte-identical twice on one machine, across debug and release and across Linux and macOS (class D0); the
  verbs' own lanes belong to their implementing beads.

## Considerations

- If kill gate KILL-PHYLUM (mixed batches lose the head-to-head) fires, the verbs and schemas stay; `phylum.plan`
  reports template coverage and `phylum.sweep` forms cohorts by exact tree.
- If the kill gate on paired gradients fires, `phylum.gradient` remains registered and returns the stencil as a
  sensitivity report, with its claim reduced accordingly.
- Closing rule: the bead closes at M2, when the rows are registered, the surfaces match and every verb passes its
  example or answers the not-implemented refusal. Verbs of this cluster first answer at M2, M3, M4. The
  per-milestone surface beads (BD-222, BD-312, BD-376) re-run this script as verbs land; that is how GATE-TETHER-02
  is re-measured.
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

## References

Appendix J (J.1 entry form; J.2 inventory rows; J.5 task clusters), §17.5, §17.11, §17.12, §17.13, §17.15, §17.16,
§32.4, §32.6, §32.12.3, §32.18, §32.21; §16.21 (adjoint gradient), §21.11 (bucket count, KILL-BUCKET), §25.30
(licensed fills); GATE-TETHER-02; INV-TETHER-01, -02; KA-755.
