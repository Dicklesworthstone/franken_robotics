---
key: "VERBS-sim"
title: "frob-verbs registry: cluster `sim`, three rows, with parity, e2e and schema-snapshot checks"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "tether", "kinesis", "frob-verbs", "solid", "registry", "verb", "test", "review-notes-open"]
parent: "WP-06"
deps: ["BD-022", "NJL-01", "NJL-02", "NJL-04", "STD-E2E"]
needs: []
related: []
gates: ["GATE-TETHER-02"]
plan_refs: ["Appendix J (J.1, J.2, J.3, J.5)", "§16.13", "§29.8", "§32.9", "§32.6", "§32.12.3", "§32.21 (GATE-TETHER-02)", "§35", "§36.10"]
---
## Background

FrankenRobotics (a robotics simulation and design system) is reached through the command line `frob`, the daemon
`frobd` and the MCP server `frob-mcp` (the Model Context Protocol, by which an agent harness lists and calls tools).
Every capability is a verb, `cluster.verb`, declared once in `registries/verbs.toml` (crate `frob-verbs`, tier
Solid). The CLI, the daemon protocol, the MCP tools, the schemas, the refusal table and the agent guide are
generated from that file, and gate GATE-TETHER-02 compares the served surfaces with it. The registry holds 244 verbs
in 44 clusters (namespaces): the 243 of the specification and `artifact.import` (VERBS-artifact); the
count of record is NJL-03's.

This bead is the registry of record for the cluster `sim`. The cluster covers simulation: estimating the cost of a
run without running it, running worlds under a profile and a contact law, and reproducing a world from its address.
It has three verbs (1 `read`, 2 `run`); the handlers live in `frob-chronicle`, `frob-dyn`. The bead carries every
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
inventory); one bead for all the rows (too large to review).

`sim.run` is the verb most scripts of the program call (more than a hundred bead files pass `--seed` to it alone),
so its row is also the place where the spelling of every run option is fixed. An option a mechanism bead needs
and this row lacks is refused by every build as an unknown parameter (`FROB-REF-802`), which is why the row lists
each option with the bead that owns it, and why the spellings the program used in more than one form are settled
here once, with the form that replaces each (KNOWN_DRIFTS 8 and 12).

## Scope

This bead is the only place where the rows of cluster `sim` are written, including what the dynamics chapter adds
to `sim.run` (`--profile`, `--law`, per-world slots through `--designs`, the chapter's refusal codes) and to
`sim.estimate` (the four Kinesis output groups), and every option that a mechanism bead of another unit defines on
these verbs (the input tables below). The dynamics unit's bead N16b-27 reads these rows, records the spellings of
those options and groups, and writes no row.

In: the three registry entries of cluster `sim`, with every input, its owner bead, domain, default, identity effect
and refusals; the worded and proposed refusals the rows will carry once the allocation ledger assigns them; the
spellings ruled out, with the form that replaces each; the MCP tool names; the cluster's conformance script; the
help-schema snapshots; each verb's answer before its milestone. Out: behaviour (the implementing and owner beads
named below); the entry schema and lints (NJL-01); the conformance harness (NJL-02); the alias lint (NJL-04); task
clusters (NJL-05, NJL-06); generation of the surfaces (the Tether seed beads); the check of these rows against the
dynamics chapter's verb table and refusal table (N16b-27, which depends on this bead); the numbers of refusal
codes (the allocation ledger, N35-33 and N35x-02).

## Technical approach

**The rows.**

| Verb | Kind | First milestone | Tier | Purpose | Defined in |
|---|---|---|---|---|---|
| `sim.estimate` | read | M2 | S | Cost, memory, capacities, expected `Degraded` rate and referee coverage; never runs physics | §16.13, §32.9 |
| `sim.replay` | run | M2 | S | Reproduce a world from its address; `--tier` teleports it | §29.8 |
| `sim.run` | run | M0 | S | Run worlds under a profile, a law and per-world designs: referee at M0, all tiers at M2. `--profile` takes a step profile (`lockstep`, `strict`, `referee`); determinism and precision profiles take `--determinism` and `--precision` (§35) | §16.13 |

**Contract of each verb.** Evidence labels used below (the `evidence` block of the envelope):
- *operational*: no claim: an operational answer about the session, the store, a job or a device; `evidence.claims`
  is empty.
- *run*: class Tested at best, by the claim product of the run; the tier of the run, `provisional` on a fast tier
  until promoted to the referee; the determinism class of the placement (D2 unless stated); `reality: no-claim`.

**How an input enters a row (DESIGN).** An input enters the generated input schema in the change that lands its
owner bead (an additive change that keeps the schema's major version, classified by NJL-02), together with the
refusal codes it brings; before that change a call that names it is an unknown parameter, `FROB-REF-802`, exit 3.
A value or input whose mechanism is built behind a feature or a kill gate is in the schema and answers
`FROB-REF-724`, exit 5, while that feature is off or killed. A conformance-build-only input (its own table below)
is in the schema of a binary built with the feature `conformance` only, so a release build answers it with
`FROB-REF-802`; a chaos-build-only input likewise with the test feature `chaos`. The milestone given for an input is
its owner bead's. Reason: an input that appears in the schema before its handler would pass validation and do
nothing, and a script would then measure a run it did not ask for.

**`sim.estimate`** (MCP tool `sim_estimate`; owner `frob-dyn`)
- Inputs (BD-219, N16b-26): a design (positional) or a design list (`--designs`, as on `sim.run`); scene; task;
  worlds; seconds; `--tier`; `--profile`; `--law`; `--workload <id>[:<ground>]` (N36-33's resolver, called by
  BD-219: a reference workload resolves to the request it names, and the verb prints the same breakdown as for the
  request typed out in full; an unknown workload is 802); `--purpose` (the input of `sim.run` below, with the same
  domain and default, from N19b-03 at M3; N19b-21 passes `oracle-check`, and N19b-19, M6, refuses a `training`
  estimate that needs the articulated proxy tier once KILL-PROXY has fired on cost); `--require-living-tier F|M`
  (the input of `sim.run` below, with the same domain, from N19b-03 at M3: the estimate applies the run's
  admission to the request it prices, N19b-03 step 7, and answers with the run's code); `--fork-batch <run> --forks
  <n>` (N19c-35, M6: the bytes of the fork-window store before a fork batch is admitted); `--gradient adjoint
  --horizon <K>` (N16c-26, M6: K control intervals, at most 32, any other value 802; prints the adjoint method's
  step store; `--horizon` without `--gradient` is 802; the admission refusal of an over-budget store belongs to
  `phylum.gradient`, not to this verb). No idempotency key (a `read`).
- Outputs: CPU core-seconds, GPU seconds, memory and disk with a 90% interval; capacities, bytes per world, the
  expected `Degraded` rate and referee coverage (the four Kinesis output groups of N16b-26); the fit of the whole
  request against the device budget (`memory.fits`, `memory.largest_worlds_that_fit`, BD-219); the envelope class
  the job needs; the store bytes of the fork and adjoint forms; the group `solve_queues` (N16c-15: whether queues
  are qualified for the class on the backend, the bound with its measured-or-HYPOTHESIS marking, and the
  general-lane share or `unmeasured`); the top-level fields `role` and `preferred_machines` beside `envelope`
  (declared optional by BD-219 and filled by N21c-07 through `role_of`; absent, never empty or guessed, in a
  build without N21c-07); never runs physics. Evidence: *operational*.
- Chapter refusals: `FROB-REF-160` (no shape class: A design exceeds the largest admitted bucket and the
  tree-operator route is off; raised by the lowering of BD-170 that the estimate calls); `FROB-REF-602` (the memory
  estimate exceeds the device budget at admission). The verb answers a request that does not fit with `fits:
  false`, the 602 payload (observed, required, the ranked repairs and the largest world count that fits) and exit
  2, allocating nothing; the refusal with exit 6 is raised by the run verbs at admission, from the same fit test
  (BD-219, N16b-26; the reading of §36.10's "refuses a job that does not fit"). The row keeps 602 so that the
  code its finding carries is the code the run verbs raise. `FROB-REF-726` (`mechanism-not-admitted`, N32-REF;
  exit 5): the run's admission applied to the priced request, with the reasons `island-in-training` and
  `tier-not-eligible` (N19b-03, M3) and `purpose-not-eligible` (N19b-19, M6: a `--purpose training` estimate with
  `--require-living-tier F` after KILL-PROXY's decision in force is `fired` on cost, the block naming the
  decision's ADR).
- Implemented by: BD-219, N16b-26. Extended by: N36-33 (`--workload`), N19b-03 and N19b-19 (`--purpose`,
  `--require-living-tier` and 726, M3 and M6), N19c-35 (`--fork-batch`, `--forks`, M6), N16c-26 (`--gradient
  adjoint`, `--horizon`, M6), N16c-15 (the `solve_queues` group, M2), N21c-07 (`role`, `preferred_machines`,
  M2).

**`sim.replay`** (MCP tool `sim_replay`; owner `frob-chronicle`)
- Inputs (BD-203): an episode (positional), or `--run <r> --world <w> [--episode <e>]`; the positional also accepts
  a `.fmcw` file (the reader is N26a-19's; `FROB-REF-724` until it is linked) and a world identity `w:<hash>` as
  `neighbours_passing` lists it (NS-15 replays those neighbours; N26a-19's resolver turns the identity into run,
  world and episode from the discovery record first, then from the stored `.fmcw` files; an identity found in
  neither is the surface's missing identity, 802); the window `[--from <tick> | --from-snapshot <hash>] [--to
  <tick>]` (the replay address that `foundry.loads` returns, world address, episode, snapshot identity and tick,
  maps onto these fields, so N31b-35's re-extraction calls this verb and needs no other); co-tenancy `--alone |
  --batch-size <n> | --co-tenants <seed>`; `--capture digests|t2|snapshots|failure` (`failure` from N29ap1-03, with
  `--markers` to capture every pending marker of the run); `--tier <t>` (teleport, BD-204) with `--precision
  f32|f64` and `--comparison`; `--require bit-equal` (N35-07, M2: a teleport across device families without class
  D1 is labelled `tolerance-replay`, and with this input refused with 247 instead); idempotency key. Without
  `--tier` the replay runs on the recorded backend and profile, resolving the producer worker (N28b-05, M3: in
  process for an equal build, the cached worker of the producer build otherwise).
- Conformance-build-only input: `--conformance-plant <file>` (N29a-25; schema `frob.conformance-plant/1`; the same
  input as on `sim.run`).
- Outputs: the world reproduced from its address with its digests (exit 0 for `Bitwise`, 2 for a mismatch with the
  first differing chunk and tick and the divergence-ledger row); under `--tier`, the step and stage at which tiers
  first differ, labelled `exact` or `tolerance-replay`. A world identity that resolves from a file with no
  recorded episode (a hand-written file, or a world never run on its own) is run on the file's originating backend
  and profile as a first run, with its digests and outcome and the comparison `none (first run)`, exit 0, never
  `Bitwise` (N26a-19). Evidence: *run*.
- Chapter refusals: `FROB-REF-222` (teleport to a tier that lacks the step plan's profile, BD-204); `FROB-REF-247`
  (`--require bit-equal` across device families without D1, N35-07); `FROB-REF-601` (the recorded backend is
  unavailable; the repair names `--tier batch-cpu`); `FROB-REF-606` (fingerprint missing, or changed since
  admission); `FROB-REF-791` (`referee-worker-mismatch`: the producer worker is absent, or was built under another
  build; the repair is its build command, N28b-05; in mode `remote`, the referee worker absent or not the build
  the job names under `--tier referee`, N33c-13); `FROB-REF-792` (`referee-upstream-refusal`: the referee
  refused for an upstream reason with no dedicated code, raised by the airlock's refusal conversion N33a-13);
  `FROB-REF-816` (a `.fmcw` positional whose no-authorization flags are missing or not `false`, mapped from the
  `frob.mcw/1` reader's rejection by N26a-19; exit 3); `FROB-REF-840` (a sealed confirmation run); `FROB-REF-894` (a
  snapshot restored under another numerical profile and labelled a continuation; `--precision f64` without
  `--comparison`). A missing identity or an invalid argument is the surface's 802.
- Implemented by: BD-203, BD-204. Extended by: N29ap1-03 (`--capture failure`, `--markers`; it raises 601, 606,
  840 and 894 under the same rules), N35-07 (`--require bit-equal`, 247), N28b-05 (producer resolution, 791, M3),
  N26a-19 (`.fmcw` and `w:` positionals, 816, M3), N29a-25 (`--conformance-plant`), N33c-13 and N33a-13 (791 and
  792 on the referee, N33-REF).

**`sim.run`** (MCP tool `sim_run`; owner `frob-dyn`)

Every input below names its owner (the bead that defines its domain and behaviour, with the first milestone),
domain and default, identity effect and refusals. Every input except the operator-only ones, `--profile-kernels`
and `--no-memo` is written into the run manifest and so enters `RunId`; the identity column names the narrower
identity an input also enters (`StepPlanId`, `ModelId`, `ControllerId`) where its owner says so, and
`exec_options_digest` for an execution option of the step plan, which is recorded in the receipt outside
`StepPlanId` (N16a-03 item 3). Code numbers are cited bare (`802` for `FROB-REF-802`).

*Subject, length and seed.*

| Input | Owner (milestone) | Domain; default | Identity | Refusals and rules |
|---|---|---|---|---|
| `design` (positional) | BD-024 (M0: `g1` only), N29ax-01 (M1: stored designs on the referee, recorded by `DesignId` as `design:b3:<hex>`, never by name), N16b-25 (M2) | A design path, `DesignId`, design name or catalog model (`g1`) | `ModelId` | Any design other than `g1` at M0, and designs other than the catalog model before M1, are 724 (BD-024). DESIGN: the property is named `design`, not `fixture`, which would collide with `--fixture`. Exclusive with `--designs` and `--fixture` (802) |
| `--designs <list or file>` | BD-169, BD-170 (M2) | Several designs, one per world slot; a design list or a designs file | `ModelId` per slot | 160 at lowering or at variant-slot admission on the device tiers (N21b-16). A designs file is a `params` entry (below) |
| `--worlds-per-design <W>` | BD-170 (M2; the `phylum.sweep` spelling) | Positive integer; worlds of each design of `--designs` | manifest | Only with `--designs`; exclusive with `--worlds` (802) |
| `--select <i>` | N16a-19 (M2) | Index of one variant of the designs file | manifest | Only with `--designs`; an index outside the file is 802 |
| `--fixture <name>` | N24b-03 (M1), N16c-41 | A registered fixture supplying design, scene, task, controller and input table: `<name>` (`assets/fixtures/<name>/fixture.toml`), or a conformance fixture `<space>/<name>` under `crates/frob-conformance/fixtures/<space>/<name>`, registered in the same registry by its owner (`traction/<name>`: BD-080's referee fixtures, used at M1 by BD-085, N18b-32 and N33ap1-01's `rest<n>` and `tip<n>`; `kin/<name>`: BD-083) | fixture identity, hence `ModelId` and `ControllerId` | 530 when the run does not match the fixture's recorded hash. Exclusive with a positional design and `--designs` (802); `--scene`, `--task` and `--controller` are 802 when the fixture declares them, and allowed when it does not (N22c-29 and N22c-46 choose the controller of a fixture design). DESIGN: the one spelling of a fixture; the positional `fixture:<name>` is ruled out (below) |
| `--scene <scene>` | N16b-25 (M2); fleet scenes N25cx-01 (M8) | A scene identity or name, or the path of a scene file (`.fscene`, the `artifact-input<scene>` parameter of the path table below, with its 891, 734 and 828); from M8 a fleet scene `frob.scene.fleet/1` | manifest | With a fleet scene a positional design and `--policy` conflict (802; the scene names its robots), and a lockstep tier is 160 |
| `--scenes <set>` with `--schedule fixed\|cycle\|draw` | N19c-22 (M3) | A scene set and the addressed schedule of scenes over episodes | manifest | Exclusive with `--scene` (802); `scene-variant-not-resident` (worded, below) at job admission and at a reset |
| `--task <task>` | N16ap4-01 (M1: the referee, registered and admitted tasks such as BD-125's `stand-10s` and `traverse-5m-flat`), N16b-25 (M2: every tier); randomization BD-200 (M2); disturbance registry N24cp3-02 (M2) | The task whose environment the run creates (`frob sim run <design> --task <t>`): a registered task name, a `TaskId` or a task file; with no length input each episode runs to the task's horizon (N16ap4-01) | manifest (the `TaskId`) | At admission, before any world exists: 529 when the model has no settled-pose cache for the task (N24a-16's check, which every verb that starts episodes applies). Its randomization plan is compiled on the environment-admission path before any world exists, and the output gains the `randomization` block (plan identifier, provenance shares and per world address the digest of its resolved list, BD-200). At admission, with zero worlds created: 103 (an entry naming a field with no parameter row); `randomize-numerical-parameter` and `randomize-no-interval` (worded, below); `disturbance-registry-invalid` (worded, below) when the task draws from `registries/disturbances.toml` and the registry does not load (N24cp3-02) |
| `--worlds <n>` | BD-024 (M0: 1), BD-090 (M1: any count on the referee, each world at its own address and recorded at the run's telemetry level), N16b-25 (M2) | Positive integer; default 1 | manifest | Any other count at M0 is 724 (BD-024) |
| `--seconds <s>` | BD-024 (M0) | An exact rational > 0 whose product with the step rate is whole; default 2 when no length is given | manifest | Never rounded; otherwise 802 with `pointer` at `/seconds` (BD-024) |
| `--steps <n>` | N16b-25 (M2) | Physics steps, positive integer | manifest | DESIGN: exactly one of `--seconds`, `--steps`, `--intervals`; two or more are 802. The manifest records the resolved step count |
| `--intervals <n>` | N16b-25 (M2) | Control intervals, positive integer | manifest | As `--steps`. The Spark scripts use this form (for example BD-178, N21b-06) |
| `--episodes <n>` | N16b-25 (M2), N19c-22 (M3: with the scene schedule) | Episodes per world; default 1 | manifest | Users at M2: N29a-16, N32a-18 |
| `--seed <u64>` | BD-024 (M0) | A u64, a string on the wire; default 0 | `RunId`; the `run_seed` of `WorldAddr = (run_seed, world, episode)` | Every random draw, reset and task assignment is a function of the world address, so a re-run with the same seed reproduces the same world addresses (BD-179, N21bp2-01) |
| `--world-addr <w>` | N16a-12 (M2) | One world index of the run's seed | manifest | Runs the one world at that address, so a world of a large run can be re-run alone |
| `--family-world <i>` | N14b-17 (M3; users BD-244, N14b-13) | Index of one world of the task's declared finite alias family (`Task::alias_family()`, worlds in index order with their names and parameter values); the run sets the task's parameters to that world's values | manifest | Only with `--task` (802 otherwise); an index outside the family is 802; a task whose family is not finite is 802 naming the parameters |
| `--init <name>[=<value>]` | N16ap4-01 (M1: the referee), N16b-25 (M2: every tier); users N13-36 at M1, N13-37 at M2, BD-384, N16a-18, N18b-24 | A named initial condition the design or fixture declares (`A`, `B`, `free-flight`, `drop=0.1m`, `flippers-down`) | `ModelId` | An undeclared name, or a value outside the declaration's domain, is 802 naming the declared conditions |
| `--param <name>=<value>` (repeatable) | N22c-29 (M4; also N22c-46) | A parameter the fixture design or the controller declares as free (N22c-29's draws `mu`, `v0`, `s0`; N22c-46's `m`, `v0`, `kd`, `h`): `<name>` matches `[a-z][a-z0-9_]*`, `<value>` an exact decimal with an optional unit token, a bare number read in the coherent SI unit of the declared dimension | `ModelId` (a design parameter) or `ControllerId` (a controller parameter); unset parameters take their declared defaults, which the manifest records | DESIGN. An undeclared name, one the fixture fixes in its `constants`, a repeated name, or a value outside a controller parameter's declared domain is 802 (`pointer` `/param/<name>`); a unit of the wrong dimension is 102. With `--fixture`, N24b-03's rule: values inside the declared domain match the fixture's recorded hash and are recorded, and a value outside it is 530 (`pointer` `free.<name>.domain`) |
| `--count <n>` | BD-086 (M1) | Draws of a fixture's randomized family to run (BD-086: 10⁶ valid thin-barrier crossings, discarded draws counted) | manifest | Fixture-scoped: only with a fixture that declares a randomized family (802 otherwise) |
| `--thermal-start cold\|soaked:<T>\|steady` | N15a-29 (M1; users N15a-27 at M1, N15a-30 on the batched path) | Overrides the task's declared thermal start for every world (N15a-15's three starts); needs `--task` | a derived task: its canonical bytes carry the override and its `TaskId` is new, and the envelope's `bound` lists both identities (hence `RunId`) | A malformed value is 802; a soaked temperature outside [ambient, lowest `t_max` of the model], or a `steady` start the actuators cannot sustain, is 132 (N15a-15) |
| `--fault <spec>` (repeatable) | N20a-31 (M3; users N20b-06, N20b-07) | A scheduled sensor fault `<kind>:<sensor>[:<param>(,<param>)*]@<trigger>` added to every world's definition: `<kind>` one of `stuck`, `dead`, `extrinsic-bump`, `link`; `<trigger>` `t=<seconds>` or `impact:<body>:<impulse>Ns` (the first committed impact on the body above the impulse); for example `extrinsic-bump:lidar:rz=2deg,x=15mm@t=3s`, `stuck:enc_knee_fl:annunciated=true@impact:base:5Ns` | manifest (the fault is part of the world, so it enters the world address and the identity) | All 802 with `pointer` `/fault/<i>`: an unknown kind, sensor, body or parameter; a malformed value or unit; a parameter of another kind; a `t=` that is not a whole number of quanta of the run's clock lattice; two specs for one sensor with overlapping active intervals |

*Placement, profiles and model level.*

| Input | Owner (milestone) | Domain; default | Identity | Refusals and rules |
|---|---|---|---|---|
| `--tier <t>` | BD-024 (M0: `referee`), N16b-25 (M2); modes BD-178, N21b-29 (M2); one card N36-31 (M2) | `referee`, `batch-cpu`, `gpu-cuda`, `gpu-metal`; a device card as `gpu-cuda:<n>`; a planner mode prefix `strict:`, `prefer:` or `mixed-allowed:` before a device backend | placement in the manifest | At M0 a tier other than `referee` is 724; a value outside the grammar is 802 (`pointer` at `/tier`). Planner refusals 604 (strict: a stage has no kernel on the backend), 605, 616; 601 |
| `--require-current-check` | N21b-29 (M2) | Flag | manifest | 605 when the CPU-versus-device check is not current for a kernel of the plan |
| `--profile lockstep\|strict\|referee` | N16a-02 (M1), N16b-25 | Step profile; default by tier | `StepPlanId` | 222 for `strict` or `referee` on a GPU tier |
| `--law <law>` | N18b-28 (M1), N16b-25 (M2: admitted on every tier) | A row of the law table, spelled as the table spells it: `CL-1`, `CL-2`, `CL-H`, `CL-M` (parity only), `CL-1R`, and the film, soil, belt and tyre rows | `StepPlanId` (the law is part of the step plan, plan §16.6; a change of law changes `StepPlanId` and no other identity) | 246 for a law whose gate has not passed on that tier or profile. A lowercase spelling (`cl-h`) is 802 |
| `--determinism reference-replay\|device-replay\|cross-backend\|fast-training` | N35-04 (M2) | Default `reference-replay` on the CPU tiers, `device-replay` on the device tiers | manifest | 247 when the class is not attainable on the placement; `fast-training` belongs to the training verbs and is 247 here |
| `--precision default\|stiff\|f32\|f64` | N35-04 (M2) | Precision profile; default `default` | `StepPlanId` | Half-precision spellings are 245 (N18b-27); a width the tier cannot step is 604 |
| `--purpose training\|throughput\|evaluation\|oracle-check\|counterexample-verification` | N15a-30 (M2: `training`, `evaluation`), N19b-03 (M3: the rest and the admission rule) | Default `evaluation` | manifest | DESIGN: one input on `sim.run` and `sim.estimate` serves both requests; it replaces N15a-30's `--mode train\|eval` (a `training` run draws the thermal start of N15a-30). A training or throughput run that names a coupled island is 726 (`island-in-training`) on every build |
| `--require-living-tier F\|M` | N19b-03 (M3) | The living-world tier the run must use | manifest | 724 when its feature is not built or killed; 726 with reason `tier-not-eligible` when the tier's admission excludes the run, and with reason `purpose-not-eligible` when the run's purpose is one a kill decision's fallback form excludes (KILL-PROXY's decision in force `fired` on cost: articulated proxies in evaluation batches only, so `training` and `throughput` are refused; N19b-19, M6) |
| `--actuator-level L0\|L1\|L2\|L3` | N15a-04 (M1, the level type), BD-070 (L0, L1), BD-071 (L2); the axis domain of N24bx-01 | Default L1 (training and sweeps, N15a-04) | `ModelId` | L0 carries the flag `ideal-actuators` (not citable for a design comparison). L3 compliance and the rotor port arrive only after their pilots and kill-gate decisions (724 until then); `L4` (hydraulic, N15b-20) is 724 until its epic lands; a value outside L0 to L4 is 802; 136 when the tier does not carry an option of the level (N15a-04's capability table); 726 (`card-provenance`, M6) for series elasticity on a card field of provenance `generic` or `assumed` (N15b-18) |
| `--state estimated\|privileged` | N22a-03 (M1) | DESIGN default `estimated` for a run whose controller reads state (a result on privileged state alone is not a result, §22); a run without a controller ignores it | manifest | 464 when an estimated-state run lacks a sensor the filter needs (`pointer` at the role) |

*Controller and policy.*

| Input | Owner (milestone) | Domain; default | Identity | Refusals and rules |
|---|---|---|---|---|
| `--controller <c>` | The input, `baseline` and a `ControllerId`: N16ap4-01 (M1: the referee), N16b-25 (M2: every tier); `fixture:<name>` N14b-06 (M1); `open-loop:<schedule>` N15ap3-01 (M1; users N15a-12, N15a-24, N19ax-01, N19ax-02); `baseline+residual:<policy>\|zero` N22a-16 (M3); `backup[:<law>]` N22bx-02 (M3) | A `ControllerId` (a controller record of `control.synth`, BD-118) or registered controller name (N22c-29's `arm1-nominal-monitor`, N22c-46's `sampled-damper`), or one of the prefixed forms; default `baseline` (the design's baseline controller for the task) | `ControllerId` | Exclusive with `--policy` (802); an unknown `ControllerId` or name is 802. A fixture controller is a registered file under `fixtures/controllers/` (DESIGN: present in every build, since N14b-06's archetype script runs it on debug and release builds). An open-loop schedule is the file `fixtures/controllers/open-loop/<schedule>.toml` of schema `frob.openloop/1`, hashed into `ControllerId`; every refusal of the form is 802 (`pointer` `/controller`): no file of that name, a file that fails the schema, a channel naming a joint the design lacks, has no actuator for or names twice, and `position` on an actuator at level L0 (N15ap3-01). An unknown backup law is 802 |
| `--policy <checkpoint>\|zero\|@fixture:<name>` | N23a-13 (M2, the role-matched loader); users N23a-02, N23a-34, N23-REF, BD-301, N20b-11 | A checkpoint file (safetensors with its `frob.policy/1` manifest and mapping table; "`--policy` names that file", N23bp1-01), the zero policy, or a fixture policy | the loaded policy's digest, part of the job identity | An ordinary input on every build (N23bp1-01). 531 when the checkpoint's mapping table does not match the design's roles; a payload that carries code or a layout outside the allowlist is refused before any world is built (the appended 828, `params` below) |
| `--reset-mode explicit\|autoreset-masked` | BD-192 (M2) | Default `explicit`; with `--task` and a policy | manifest | 522 when a policy emits a non-finite action (the world and channel are named) |
| `--shield <ShieldId>` | BD-338 (M4) | A fitted shield: its action-filter stage is attached | `RunId` | 457 when the shield's margin is used outside its scope key (checked at start) |
| `--shield-budget <EnergyBudgetId>` | BD-337 (M4) | An admitted energy budget: the specialized energy monitor is attached | `RunId` | An unknown or unadmitted budget is 802 |
| `--verify-reduction` | BD-327, N22b-38 (M4) | Flag: the reduced controller's command against the full twin's at every tick, on both sides of each mode switch | manifest | 461 with the tick and the mode on a disagreement (the reduction is disabled for that context and the full model continues); a run without a reduced controller ignores the flag and says so |
| `--intra-step hold\|interpolate` | N20a-12 (M2) | The policy of every clock its task does not declare, on the lockstep profile of `batch-cpu` and on `gpu-cuda` and `gpu-metal`: `hold` holds each such clock; `interpolate` interpolates sensor clocks and holds consumer clocks (controllers, estimators, actuator channels), listed in the output as `hold (consumer)`. Precedence: a clock the task declares, then this input, then the tier profile's default, which is the decision of OPEN-SENS-1 (`hold` until its ADR exists) | `RunId` through the resolved policy of each clock, not the raw input: an input that changes no clock changes no identity | On the referee and the strict profile every clock is `split`; the input changes nothing there and the output says so. The output records per clock its policy and its source (`task`, `run` or `default`). An unknown value, `split` included, is 802 |

*Step plan.*

| Input | Owner (milestone) | Domain; default | Identity | Refusals and rules |
|---|---|---|---|---|
| `--step <duration>`, `--newton-iterations <n>`, `--warm-start on\|off` | N16a-03, N16a-04 (M1); spellings N16bp2-01 | The numerical plan; defaults from the profile | `StepPlanId` | Outside the plan's domain 802; 212 is avoided by substepping |
| `--substeps <d>`, `--integration exact-hold` | N16c-41 (M2) | Fixture-scoped | `StepPlanId` | 802 without a fixture that declares them |
| `--txn l-max=<L>,e-max=<E>,retries=<N>` | N16a-09 (M1; its DESIGN spelling) | Event-refinement limits and retry-ladder length of the transactional step | `StepPlanId` | DESIGN: transactional profiles (`referee`, `strict`) only, 802 under `lockstep`; 220, 221 as the limits bind |
| `--adjudicate referee-only\|patch-reduction\|strict-fallback` | N16a-05 (M1) | Single-run routing; default `strict-fallback` for evaluation | manifest: the policy selects which step answers and shapes none, so each result carries the `StepPlanId` of the step that answered it | The spelling `strict` is 802 |
| `--patch-reduction <name>` (repeatable) | N18b-21 (M2) | A qualified row of `registries/patch_reductions.toml` | `ModelId` | `patch-reduction-unqualified` (worded, below) |
| `--closures auto\|rows` | N16c-32 (M2) | Default `auto` | `ModelId` | A bad value is 802 |
| `--joint-friction rows\|smooth` | BD-155 (M2) | Default `rows` | `StepPlanId` | `smooth` on a lockstep tier only, 136 otherwise; result condition `smooth-joint-friction` |
| `--capacity contacts=<C>[,rows=<R>][,rut-tiles=<T>]`, or `--capacity rows=<R>` alone | N16a-14 (M2, the capacity registry and defaults); member `rut-tiles` N38a-29 (M6, Moonshot, feature `moonshot-ruts`); users N16a-12, N16a-18, N16b-03 | Contact and row capacities per world on `batch-cpu` and the device tiers: C a positive integer, R a positive multiple of 8; an omitted C keeps the registry's, an omitted R is the registry's when C equals the workload's registered C and `default_rows(C, actuated)` otherwise. `rut-tiles=<T>`: T_rut, a positive integer, default the fixture's declared value, otherwise 1,024 | `StepPlanId` (a new shape class; the disclosure prints `capacity_source: registry\|override`) | DESIGN spelling; any other value is 802, and on `--tier referee` the input is 802 (the referee's cap is a limit-register entry, N16a-05); 721, 722, 610 as the capacities bind. `rut-tiles` is 724 on a build without `moonshot-ruts` or on a backend whose KILL-RUT decision in force is `fired` (naming `not-in-build` or `killed`); zero or a non-integer is 802 |
| `--jacobian dense\|path-table` | N16a-13 (M2) | Default `dense` | `StepPlanId` | `path-table` is opt-in and labels the result `experimental-execution` |
| `--refine-step <factor>` | N16c-43 (M2) | A step-refinement factor | (submits a study) | Builds and submits the two-arm step-refinement study |
| `--integrator default\|variational` | BD-384 (M5), feature `frontier-variational` | Default `default` | `StepPlanId` | 724 when the feature is off or killed (N16a-22); 222 on a GPU tier; 802 with `--profile lockstep` |
| `--solve-path dense\|ancestor\|tree` | N16c-38 (M2); `tree` N16a-24 (M6, behind its Frontier feature), which is the option's owner after a point-A kill of KILL-ANCESTOR (N16cp3-03) | Default `dense` until GATE-KIN-12 passes and the kill decision records `keep`, then chosen per shape class by the planner and always written into the step plan | `StepPlanId` | `tree` is 724 without its feature. After a point-A kill (`docs/adr/ADR-KILL-ANCESTOR-M2.md`) the value `ancestor` is never built and is 724, exit 5, the reason naming that ADR; after a point-B kill (N16c-40), `ancestor` on a model of 18 to about 60 coordinates is the worded `ancestor-path-after-kill` (below), and it stays admitted above that range |
| `--solve-queues off\|auto\|<q0,q1,q2>` | N16c-15 (M2) | Default `off`; explicit capacities are multiples of 32 | `exec_options_digest`: queue membership and capacities are execution options of the step plan and change no digest (§16.19, N16a-03 item 3), so they are outside `StepPlanId` | 802 under `--profile strict` or `--tier referee` (`pointer` `execution.solve_queues`); a queued run on an unqualified backend is labelled `experimental-execution` |
| `--family-lanes` | BD-356 (M4, Frontier, crate flag `family-lanes`; the same input on `phylum.sweep`, VERBS-phylum) | Flag: worlds that share tree, sensor graph and contact mode run in design-family lanes, bit-identical to the general masked lane | `exec_options_digest`: lanes are an execution option of the step plan (BD-356 step 5), outside `StepPlanId` | 724 while the `family-lanes` flag is off in the build or KILL-FAMILYLANE's decision in force is `fired`; until the digest comparison has passed on a backend, a run with lanes is labelled `experimental-execution` and refused as promotion evidence |
| `--decouple [reference]` | N16c-20 (M8) | Certified islands stepped as single-robot worlds; `reference` runs the reference arm | `StepPlanId` | 721, 722, 610 as for the coupled run |
| `--no-field-memo`, `--field-memo-capacity <M>` | N19ax-03 (M2) | Capacity a power of two from 1 to 4,096; default 256 | step plan (digests unchanged) | Outside the domain 802 |
| `--effort <name>` | N28a-34 (M4) | A registered solver effort profile (rule `pinned`); default the table's selection | `StepPlanId` (the `EffortProfileId` is folded in) | An unregistered name is 802 |
| `--kernel-set generic\|specialized` | N21c-19 (M4) | Absent: the planner's choice (generic unless N21c-20's survivor switch-over proposes a specialized kernel, which the fence checks without refusing the caller) | `StepPlanId` | An explicit `specialized` is refused with 613 when the specialized kernel is not bit-identical to its generic kernel, zero worlds stepped (repair 1: `--kernel-set generic`); 603 when it does not compile |
| `--step-plan staged\|fused` | N21c-23 (M8) | Default `staged` | `StepPlanId` | 604 when the shape class has no fused kernel, zero worlds stepped. The value `fused-pilot` exists only in a development build with the feature `pilot-fused-step` (N21c-21); any other build answers it with 724 |

*Failure policies.*

| Input | Owner (milestone) | Domain; default | Identity | Refusals and rules |
|---|---|---|---|---|
| `--on-overflow refuse\|reset\|continue-flagged` | N16b-25 (M2); device tiers N21b-23 | Default `continue-flagged` | manifest | Under `refuse`: 722 on `batch-cpu`, 610 on `gpu-cuda` and `gpu-metal` (the split below) |
| `--on-degraded refuse\|continue-flagged` | N35-04 (M2) | Default `refuse` | manifest | 248 for an ill-conditioned world under `refuse` |
| `--max-faulted <count or fraction>` | N21b-23, BD-175 (M2) | Default 0 for evaluation | manifest | 608 when the worlds marked `Faulted` exceed it |
| `--fail-fast` | N16b-25 (M2) | Flag | manifest | A world-level refusal (220, 221, 224, 721 under the continue policies) fails the whole run instead of being reported per world |

*Recording and output.*

| Input | Owner (milestone) | Domain; default | Identity | Refusals and rules |
|---|---|---|---|---|
| `--telemetry T0\|T1\|T2\|T3\|task-all` | BD-090 (M1: the input, T0 to T2 on the referee), BD-207 (M2: the batched tiers), N29ap1-02 (T3 snapshots), N24cx-01 (M2: the selection `task-all`) | DESIGN: the highest level recorded; default `T1` (T0 and T1 for every world, as BD-207's default scopes) | manifest (physics digests unchanged) | Lowercase levels (`t2`) and `packets` are 802 |
| `--trace-rate <a/b>` \| `--trace-size <k>` | N29a-12 (M2) | Mutually exclusive; default rate 1/100 | manifest (`trace.rate`, `trace.size`) | Both, or an invalid value, are 802 |
| `--snapshot-every <chunks>` | N29ap1-02 (M2) | Sets the manifest field `t3.every_chunks`, a positive integer; default 1: a traced world has a stored full snapshot at every k-th logical chunk boundary | manifest | DESIGN spelling for the existing field (N20a-45, N21b-26 and BD-172 use it). Zero, a negative or a non-integer value is 802 (`pointer` `/snapshot_every`); the input without `--telemetry T3` is 802 naming `--telemetry T3` |
| `--chunk-len <L>` | N29b-18 (M1) | A power of two from 64 to 1,024 control ticks; default the decided L (256) | `RunId` | Anything else is 802 |
| `--peak-capture <scalar>[=<entity ids>]` | N29ax-02 (M3) | One time-coherent T1 sample per entity per episode at the scalar's running maximum | manifest | Not `--capture`, which `sim.replay` uses |
| `--present` | N16b-24 (M3) | One world paced at presentation rate | physics unchanged | |
| `--resolved-out <file>` | BD-200 (M2); the export door N37-18 (M3) | JSON lines of each world's resolved randomization list (address, draw, instance, value with unit, support, law), first line a `frob.diag-header/1` header; export kind `diagnostic` | outside every identity | Under the session's export root (734 for a path that leaves it, N37-REF's list rule). From N37-18 the output takes an `ExportGrant`: the row carries `exports = true`, `option_capabilities = { resolved_out = ["export:report"] }`, and 731, 733 and 816 |
| `--profile-kernels` | N21b-06 (M2, `gpu-metal`); N21a-27 (CUDA and `batch-cpu`, the profile record) | Flag: the profiling mode, one batch per kernel | outside every identity; results carry `profiling: true` and change no digest | 802 on a tier that does not yet support it |

*Attachments and workload.*

| Input | Owner (milestone) | Domain; default | Identity | Refusals and rules |
|---|---|---|---|---|
| `--workload <id>[:<ground>]` | N36-33 (M2) | A reference workload resolved to its request | as the resolved request | An unknown workload is 802 |
| `--battery-age f_q=<ratio>,f_r=<ratio>` | N15b-14 (M6) | Constant ageing; an omitted member is 0 | `ModelId` | |
| `--battery-age evolved` with `--age-record <name>`, `--duty-multiplier <w>`, `--idle-time <duration>` | N15b-17 (M6) | Duty multiplier a positive rational, default 1; idle time default 0 s | `ModelId` | 724 without the evolved law linked (N15b-16) |
| `--co-locate` | N19c-20 (M3) | Flag: all of a job's variants in one pool segment | manifest | |
| `--migration delta-w=<metres>,max-rotation-deg=<degrees>` | N18b-38 (M1) | Witness-migration tolerances | step plan; printed in the envelope | Only with `--fixture` under a transactional profile, 802 otherwise |
| `--no-migration` | BD-093 (M1) | Flag: turns off the witness-change guard and T4.7 | manifest; `diagnostic: true` | Only with `--tier referee` (802 otherwise); a run made with it enters no claim, gate record or comparison |

*Operator-only inputs* (marked operator-only in the input schema, outside every identity: neither enters
`StepPlanId`, a state digest or a result identity, and the receipt echoes them): `workers` (`--workers`, an
integer 1 to 512; default from the pool configuration) and `tile_worlds` (`--tile-worlds`, a power of two from 1
to 1,024; default 32), both for `--tier batch-cpu` only; on another tier, or outside its domain (a `tile_worlds`
that is not a power of two included), `FROB-REF-802`, exit 3 (BD-160, M2).

*Conformance-build-only inputs* (feature `conformance`, absent from a release build's schema, so a release build
answers 802; each is written into the manifest and, where it changes stepping, into `StepPlanId`, so a planted run
never shares an identity with an ordinary one; the precedent is N21b-07's):

| Input | Owner | Rule and refusal |
|---|---|---|
| `--inject-faults seed=<S>,count=<N> \| consecutive=<K>[,fence=off]` | N21b-07 | 607 after bounded retries |
| `--inject-stall chunk=<k>` | N21b-24 | 615 (watchdog) |
| `--withhold-kernel <name>` | N21b-29 | 604 under a strict mode |
| `--conformance-host-read <slab>` | N21b-11 (N21b-26 uses it) | 614 (host access outside the lease) |
| `--admit-at <chunk>` with `--design <id>` | N21b-16, N21b-25 (NS-14 uses it) | 611 when the admission would change the running environment's shape; `--design` is accepted only with `--admit-at` |
| `--mutant <name>[:<key>]` | N21b-42 (the one loader of the input), N35-15 (its mutants, registered with that loader) | A planted mutant of the named check |
| `--chunk-intervals <N>` | N21b-22 (users N21b-39, N35-03, N21c-39, N21c-43) | Not `--chunk-len` (the release option of N29b-18) |
| `--group-multiple <N>`, `--cancel-at seed=<S>,count=<N>`, `--cache-dir <path>`, `--host-workers <N>` | N21b-39 | `--cache-dir` names a directory under the session's roots (734 for one outside them). `--host-workers`: the host threads that prepare uploads and reset writes and assemble ring and snapshot readback on a device tier (the device form of the `workers` lane, since `--workers` is `batch-cpu` only), 1 to 64, otherwise 802; written into the manifest and echoed in the receipt, outside `StepPlanId` |
| `--inject-device-loss open\|chunk=<k>` | N21b-23 (used by N21b-25's `device_lost`) | The device open fails, or the device reports a context loss at chunk k: 601 (before any world is stepped for `open`, after chunk k − 1's commit for `chunk=<k>`) |
| `--capacity-step 1\|max` | BD-170 (M2; N17a-10's test option of the same name on `phylum.sweep`) | `1` places every design in the next larger registered bucket than its native one, `max` in the largest bucket that admits it; a design with no larger bucket runs at raised contact and row capacities of its own bucket, in N17a-10's sequence. Enters the manifest and `StepPlanId` |
| `--conformance-draw-tap` | N35-16 | |
| `--conformance-plant <file>` | N29a-25; plant kinds of `registries/plant-kinds.toml` registered by N16b-15 (`mutation`: `penetration-recovery`, `explicit-penalty`, `pyramid4-base`, one per file) and N21b-23 (`nan`, `overflow`) | Schema `frob.conformance-plant/1`; the file's canonical bytes enter `StepPlanId` (N29a-25's rule); also on `sim.replay` |
| `--perturb-ulp q0` | BD-024 (M0) | A manifest field, so a perturbed run has its own `RunId` |
| `--kernel-arm a\|b\|c\|d` | N17b-24 | The pilot's arm selector |

*Chaos-build-only inputs* (test feature `chaos`, M1; absent from release and conformance builds, where each is
802; each is written into the manifest and `StepPlanId`, and the envelope carries `chaos: true`):

| Input | Owner | Rule and refusal |
|---|---|---|
| `--chaos faults=<n>,seed=<s>,point=<p>` | BD-081 (user N16a-10) | `n` a positive integer, the faults injected over the run; `s` a u64, default the run's `--seed`; `p` one of `t4`, `t5-t6`, `checkpoint`, `restore`, `any` (default `any`); the output gains the block `chaos { faults, reads, uncommitted_reads, by_point, by_kind }`; `faults=0` or another point is 802 |
| `--chaos-plan <file>` | BD-083 (users N16c-02, N15a-12, N18a-37) | Schema `frob.chaos.retry/1`: entries `{ world, base_interval, fail_attempts_below = k }`, 1 ≤ k ≤ N_retry, each one forced retry; an entry naming a world or interval the run does not have, or k outside that range, is 802 (`pointer` at the entry) |
| `--numerical-plan-floor <k>` | BD-083 (user N16c-02) | Only with `--chaos-plan` (802 otherwise): the listed intervals start the retry ladder at rung k and no check is forced to fail, the clean plan-k run a forced run is compared with |

*Conditional inputs* (N23bp1-01, the whole-rollout fallback): `--horizon <H control intervals>` and `--record
trajectories`, which return a `frob.rollout/1` array artifact through the job interface. They are admitted only
while KILL-GYMADAPT's decision in force is `fired` (GATE-CORTEX-16 `killed`; a kill row's state is its outcome,
N40-03); on every other build they are refused as a capability not in the build (`FROB-REF-724`, exit 5).
`--policy` is not conditional (above).

*Common inputs* (generated for every `run` verb by NJL-01, not listed per row): `--key`, `--budget`,
`--sync-wait-ms`, `--estimate`, `--on-cancel leave|cancel_job`, `--envelope-scope <gate:…|campaign:…|pretrain:…>`
(the approved compute envelope scope the job cites, KNOWN_DRIFTS 14); `--json`, `--help`, and the CLI's raw
argument forms `--args '<json>'` and `--args-file <path>` (N34-20). Under `--estimate` the estimate block carries
`role` and `preferred_machines` as `sim.estimate` does (N21c-07). BD-024's M0 handler accepts a missing key on the
CLI (805 is raised from M1, N32-REF); every script and example passes `--key` (KNOWN_DRIFTS 5). NJL-01's
variations add the later common inputs of every `run` entry (`--from-proposal`, N28c-14, and `--machine
mac|trj`, N41-12, both from M4); they are not inputs of this row. The entry sets `memo = true` (BD-111, M1, whose
wrapper fronts this verb's executor with the memo graph), so NJL-01's variation 14 generates `no_memo` (boolean,
default false; CLI `--no-memo`): nothing is served from the memo graph for this call in any store mode, every node
is computed and still stored, and the envelope's memo disposition is `NoMemo` (BD-111, N28b-38; user N28b-42, its
episodes step); it is outside every identity, since a served product is bit-identical to a computed one. The row
declares no `no_memo` of its own, which would fail NJL-01's lint `common-property-shadowed`.

- Outputs: a run with its manifest, outcome summary, digests and standing (`provisional` on a fast tier); with
  `--task`, the verdict record `frob.verdict/1` of each episode (N16ap4-01, by N24c-02's evaluator) and the
  `randomization` block (BD-200). Evidence: *run*.
- Chapter refusals (numbered, each in `registries/refusals.toml`; raised as stated above or below):
  `FROB-REF-102` (a `--param` value whose unit has the wrong dimension, N22c-29, M4); `FROB-REF-103` (a
  randomization entry naming a field with no parameter row, BD-200); `FROB-REF-132` (a `--thermal-start` the
  model cannot take: a soaked temperature outside [ambient, lowest `t_max`], or a steady start the actuators
  cannot sustain, N15a-29 through N15a-15, M1); `FROB-REF-136` (a row option
  the requested tier does not carry: N15a-04, BD-155); `FROB-REF-160` (no shape class: a fleet scene on a
  lockstep tier, N25cx-01, and variant-slot admission, N21b-16); `FROB-REF-212` (Cayley bound: ‖ω‖·h above 0.2 in
  a substep; an existing code of the algebra subsystem that the dynamics chapter's refusal table lists for this
  verb; the step planner avoids it by substepping); `FROB-REF-220` (event refinement limit: An event cannot be
  bracketed within L_max, or more than E_max events fall in one physics interval); `FROB-REF-221` (transaction
  exhausted: Every numerical plan of the retry ladder failed its checks); `FROB-REF-222` (profile not available on
  tier: `strict` or `referee` requested on a GPU tier, and the variational integrator there); `FROB-REF-223` (no
  refinement law: A stochastic driver without a registered refinement law under a transactional profile);
  `FROB-REF-224` (indeterminate contact: No solution, or several distinct post-event motions); `FROB-REF-245`
  (half precision, N18b-27); `FROB-REF-246` (law not admitted: A law requested on a tier or profile where its gate
  has not passed); `FROB-REF-247` (the determinism class a verb requires is not attainable on the requested
  placement); `FROB-REF-248` (an ill-conditioned solve under `--on-degraded refuse`, N35-04); `FROB-REF-457` (a
  shield margin used outside its scope key, BD-338); `FROB-REF-461` (a reduced controller disagreeing with its
  full twin under `--verify-reduction`, N22b-38); `FROB-REF-464` (an estimated-state run lacking a sensor the
  filter needs, N22a-03); `FROB-REF-522` (a non-finite action from a policy, BD-192); `FROB-REF-529` (no
  settled-pose cache for the model and the task of `--task`: N24a-16's admission check, applied by N16ap4-01 on
  the referee at M1 and by N16b-25 on every tier); `FROB-REF-530` (a fixture run
  not matching the fixture's recorded hash, N24b-03); `FROB-REF-531` (a policy whose mapping table does not match
  the roles, N23a-13); `FROB-REF-601` (device unavailable or lost: Rerun on another device, or on `batch-cpu` under
  `mixed-allowed`); `FROB-REF-602` (the memory estimate exceeds the device budget at admission, BD-219);
  `FROB-REF-603` (kernel compile failure: BD-172, BD-173, BD-174, N21c-19); `FROB-REF-604` (a stage has no kernel
  on the strict backend: BD-178, N21b-29, N21c-23; a precision width the tier cannot step, N35-04); `FROB-REF-605`
  (the CPU-versus-device check is not current: BD-178, BD-176); `FROB-REF-606` (fingerprint missing or changed
  since admission, N21c-05); `FROB-REF-607` (a launch failed after bounded retries: BD-172, BD-174, BD-175);
  `FROB-REF-608` (worlds marked `Faulted` above the threshold, BD-175); `FROB-REF-609` (a double-float canary
  failed: N21b-36, BD-179); `FROB-REF-610` (capacity overflow under the `refuse` policy on a device tier: N21b-23,
  BD-175); `FROB-REF-611` (a shape change inside a running environment, N21b-16); `FROB-REF-612` (a corrupt kernel
  cache entry and no recompilation, BD-173); `FROB-REF-613` (a specialized kernel not bit-identical, N21c-19);
  `FROB-REF-614` (host access outside a lease, BD-173); `FROB-REF-615` (device stalled, BD-174, BD-175);
  `FROB-REF-616` (the device lacks an arithmetic feature the kernel declares, N21c-42); `FROB-REF-721` (referee
  capacity: An island exceeds the referee's contact cap, or FrankenSim's fixed-witness gate refuses);
  `FROB-REF-722` (capacity overflow, refused: A world exceeds its contact or row capacity and the caller chose
  refuse; raised on `batch-cpu`); `FROB-REF-726` (`mechanism-not-admitted`, N32-REF, exit 5: reasons
  `island-in-training` and `tier-not-eligible` from N19b-03 at M3, `purpose-not-eligible` from N19b-19 at M6, and
  `card-provenance` from N15b-18 at M6); `FROB-REF-731`, `FROB-REF-733` and `FROB-REF-816` (the export door of
  `--resolved-out`, from N37-18); `FROB-REF-734` (a `--resolved-out` path outside the session's export root,
  BD-200); `FROB-REF-791` (`referee-worker-mismatch`: in mode `remote`, the referee worker of a `--tier referee`
  run is absent or not the build the job names, N33c-13); `FROB-REF-792` (`referee-upstream-refusal`: the
  referee refused for an upstream reason with no dedicated code, raised by the airlock's refusal conversion
  N33a-13).
- Overflow under `--on-overflow refuse` is split by tier, and the split is settled (N16-REF, N21-REF, KNOWN_DRIFTS
  6): `FROB-REF-722` (exit 5) on `batch-cpu`, raised by the batched step BD-159 (N16b-25's handler); `FROB-REF-610`
  (exit 6) on `gpu-cuda` and `gpu-metal`, raised by N21b-23 through the same function, with BD-175 the device
  capstone that tests it. The row lists both; neither is retired.
- Airlock requirement: `needs_groups = ["referee", { groups = ["multiphysics"], when = "scene-has-coupled-island"
  }, { groups = ["actuation"], when = "design-has-port-actuators" }]`. BD-024's M0 handler raises `FROB-REF-790`
  (`foundation-group-absent`) when the binary lacks the `referee` group, and N33c-32 names this row's
  coupled-island predicate as its example; NJL-01's generator appends 790 from the field. The predicate
  `design-has-port-actuators` (DESIGN, registered in `frob_verbs::admission::predicates` by N15a-05, M1) holds
  when the resolved request's design, or a design of `--designs`, has an actuator whose level builds N15a-05's
  port-Hamiltonian elements on the referee, which reach `fs-phs` through the airlock's `actuation` group (BD-068);
  a binary without that group answers such a run with 790 naming `actuation`, and a run whose designs have no such
  actuator does not require the group. `sim.estimate` and `sim.replay` carry `needs_groups = []`.
- Implemented by: BD-024 (M0), N16ap4-01 (M1: `--task`, `--controller` and `--init` on the referee), N16b-25 (M2).
- Extended by (each owner of the tables above; the milestone is the owner's): N16a-02, N16a-03, N16a-04, N16a-05,
  N16a-09, N16a-12, N16a-13, N16a-14, N16a-19, N16a-24, N16bp2-01, N16b-24, N16c-15, N16c-20, N16c-32, N16c-38,
  N16c-41, N16c-43, N15a-04, N15a-05, N15ap3-01, N15a-29, N15a-30, N15b-14, N15b-17, N15b-18, N15b-20, BD-070,
  BD-071, BD-086, BD-093, BD-155, BD-160, BD-169, BD-170, BD-175, BD-178, BD-192, BD-200, BD-207, BD-090, BD-327,
  BD-337, BD-338, BD-356, BD-384, N14b-06, N14b-17, N18b-21, N18b-28, N18b-38, N19ax-03, N19b-03, N19b-19,
  N19c-20, N19c-22, N20a-12, N20a-31, N21b-06, N21b-23, N21b-29, N21c-19, N21c-23, N22a-03, N22a-16, N22bx-02,
  N22b-38, N22c-29, N23a-13, N23bp1-01, N24b-03, N24cp3-02, N24cx-01, N25cx-01, N28a-34, N29a-12,
  N29ap1-02, N29ax-01, N29ax-02, N29b-18, N33a-13, N33c-13, N35-04, N36-31, N36-33, N37-18, N38a-29; the
  conformance-build inputs' owners N21b-07, N21b-11, N21b-16, N21b-22, N21b-24, N21b-25, N21b-39, N21b-42, N35-15,
  N35-16, N29a-25, N16b-15, N17b-24, BD-170; the chaos-build owners BD-081 and BD-083.

**Worded and proposed refusals.** These codes have no assigned number yet: the allocation ledger
(`registries/refusal_allocations.toml`, N35-33, N35x-02) assigns it, and no bead types one. Each is raised
through its constant once its registry row exists, and the row's `refusals` field receives the assigned number in
the change that registers it; until then the code does not compile, so no handler raises it and the rule "a verb
raises only codes of its list" holds at every step (the convention of VERBS-scene).

| Slug (proposal) | Registry row | Family, exit | Raised by, on this cluster |
|---|---|---|---|
| `randomize-numerical-parameter` (proposed 572) | N23-REF | `5xx`, 3 | BD-200, `sim.run --task`: a numerical parameter such as the time step in `randomization`; zero worlds created |
| `randomize-no-interval` (proposed 571) | N23bx-03 | `5xx`, 3 | BD-200, `sim.run --task`: a field whose uncertainty is `exact` or absent; zero worlds created |
| `scene-variant-not-resident` (proposed 319; row 2 of N19c-42) | N19c-42 | `3xx`, 3 | N19c-22, `sim.run --scenes`: at job admission and at a reset |
| `patch-reduction-unqualified` | N18-REF | `2xx`, 3 | N18b-21, `sim.run --patch-reduction`: no passing qualification receipt for the pair |
| `device-count-wrap` (proposed 619) | N21-REF | `6xx`, 6 | N21c-39, `sim.run` on a device tier: the no-wrap check in `StepPlan::build` at admission (M2) |
| `disturbance-registry-invalid` | N24-REF | `5xx`, 3 | N24cp3-02, `sim.run --task` at environment admission (M2): `registries/disturbances.toml` does not load (`pointer` the file, slug and field); no partial registry is served |
| `ancestor-path-after-kill` | N16-REF | `8xx` (held for the owner decision, N35-33 rule 8), 3 | N16c-38 after the point-B kill of KILL-ANCESTOR (N16c-40), `sim.run --solve-path ancestor` on a model of 18 to about 60 coordinates; raised meanwhile as 802 with `detail` the slug (N16-REF's interim) |

**Spellings ruled out.** Each is 802 against these rows; the form on the right replaces it, and a bead that
spells an input the left way changes to the right (KNOWN_DRIFTS 12):

| Used | Use instead | Reason |
|---|---|---|
| `--seeds 1..100` | a loop over `--seed <s>` | One run has one root seed; a range would make one `RunId` stand for many |
| `--design <id>` (outside `--admit-at`) | the positional design | The design is positional (N16b-25, N22b-32, N23bp1-01); one spelling per input |
| `--level L2` | `--actuator-level L2` | N24bx-01's axis binds `--actuator-level` |
| `--mode train\|eval` | `--purpose training\|evaluation` | One purpose input on both rows (above) |
| `--law cl-h` and other lowercase laws | `--law CL-H` | The law table's spelling |
| `--telemetry t1`, `t2`, `packets` | `--telemetry T1`, `T2` | Levels are T0 to T3 and the selection `task-all`; packets are recorded at T2 |
| `--adjudicate strict` | `--adjudicate strict-fallback` | N16a-05's value set |
| `--device gpu-cuda:<n>` | `--tier gpu-cuda:<n>` | One card is named in the tier (N36-31) |
| `--workers` as a conformance knob | the operator-only `--workers` (batch-cpu only) | BD-160 owns it, outside every identity |
| `--battery <battery>` | `eval.run --battery` | A battery is `eval.run`'s input |
| `--co-tenants @random:<i>` on `sim.run` | `sim.replay --co-tenants <seed>` | Co-tenancy of a reproduced world is the replay's input |
| `--gait <g> --speed <v>` | `--controller <ControllerId>` from `control.synth --gait <g>`, and the commanded speed in the task | The gait is a synthesis choice, the speed a task parameter |
| `--regen to-battery\|shunt\|clip` | three fixture designs, or `--param regen_policy=<p>` where the fixture declares it | `regen_policy` is a card field of the driver group (N15b-04) |
| `--variants parameter\|none` | `--designs <variants file>` against a homogeneous design | N16a-19's form of parameter variants |
| `--rows exact` | `--capacity rows=<R>`, or a pilot-only input that N17a-11 defines | No bead defines `--rows` |
| `fixture:<name>` as the positional design | `--fixture <name>` | One spelling of a fixture (21 bead files use `--fixture`, two the positional form) |

**Parameters that accept a path to a file** (NJL-01's `params` rule: `artifact-input<F>` appends 891, 734 and
828; `foreign-file` appends 893, 734 and 828; the formats are the rows of `registries/formats.toml`, BD-002). DESIGN
where the implementing bead names no kind, by VERBS-scene's rule (a plain list is `foreign-file`; a document of a
registered format is `artifact-input` of it); an identifier or a name is accepted by the same parameter, as
`task.verdict`'s `run` accepts an identifier or a `.frun` path:

| Row | `params` |
|---|---|
| `sim.estimate` | `{ design = "artifact-input<design>", scene = "artifact-input<scene>", task = "artifact-input<task>", designs = "foreign-file" }` |
| `sim.replay` | `{ episode = "artifact-input<mcw>" }` (the positional: an episode identifier, a world identity `w:<hash>`, or a `.fmcw` path) |
| `sim.run` | `{ design = "artifact-input<design>", scene = "artifact-input<scene>", task = "artifact-input<task>", designs = "foreign-file", policy = "foreign-file" }` |

`policy` is a safetensors file with its `frob.policy/1` manifest and mapping table, read only through N23a-13's
loader (N23a-13 rejected a bespoke policy format, so it is not a row of `formats.toml`); the same parameter takes
`zero` and `@fixture:<name>`. `--fixture` takes a registered fixture name, and the conformance-build and
chaos-build files (`--conformance-plant`, `--chaos-plan`) are declared by their owners in the builds that carry
them; `--resolved-out` is an export path, whose codes (731, 733, 734, 816) are in the chapter list.

**Machine fields of each row.**

| Verb | Capabilities | Output schema | Task clusters | Example |
|---|---|---|---|---|
| `sim.estimate` | `sim:read` | `frob.sim.estimate.out/1` | `simulate`, `learn`, `fleet` | `frob sim estimate quad-a --scene terrain:stairs@0.4 --worlds 2048 --seconds 20 --tier gpu-cuda --json` |
| `sim.replay` | `sim:run` | `frob.sim.replay.out/1` | `simulate`, `runs` | `frob sim replay --run r:19e2 --world 311 --key rp-1 --json` |
| `sim.run` | `sim:run` | `frob.sim.run.out/1` | `simulate` | `frob sim run quad-a --scene terrain:stairs@0.4 --worlds 2048 --seconds 20 --tier gpu-cuda --key run-1 --json`; the M0 example `frob sim run g1 --tier referee --seconds 2 --key run-m0 --json` (BD-024) |

The remaining fields follow by rule: `input_schema` is `frob.<verb name>.in/1`; `refusals` is the chapter list of
the verb's block; `idempotent` is `true` for `read` and `admin` and `false` for `run` and `mutate` (which take an
idempotency key); `cost_model` is `static: …` for `read`, `mutate` and `admin` and the `--estimate` cost model for
`run`; a `run` row carries `wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }`; `owner` is the
crate of the block; `purpose` is the Purpose column.

One entry in full (the others follow field for field):

```toml
[[verb]]
name = "sim.replay"
cluster = "sim"
kind = "run"
tier = "S"
capabilities = ["sim:run"]
input_schema = "frob.sim.replay.in/1"
output_schema = "frob.sim.replay.out/1"
refusals = ["FROB-REF-222", "FROB-REF-247", "FROB-REF-601", "FROB-REF-606", "FROB-REF-791", "FROB-REF-792", "FROB-REF-816", "FROB-REF-840", "FROB-REF-894"]
idempotent = false
cost_model = "estimated: `--estimate` returns CPU core-seconds, GPU seconds, memory and disk with a conformal interval"
first_milestone = "M2"
task_clusters = ["simulate", "runs"]
examples = ["frob sim replay --run r:19e2 --world 311 --key rp-1 --json"]
owner = "frob-chronicle"
purpose = "Reproduce a world from its address; `--tier` teleports it"
wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }
needs_groups = []
params = { episode = "artifact-input<mcw>" }
```

The field-driven lines of `sim.run` (the rest as above):

```toml
exports = true
option_capabilities = { resolved_out = ["export:report"] }
memo = true
needs_groups = ["referee", { groups = ["multiphysics"], when = "scene-has-coupled-island" }, { groups = ["actuation"], when = "design-has-port-actuators" }]
params = { design = "artifact-input<design>", scene = "artifact-input<scene>", task = "artifact-input<task>", designs = "foreign-file", policy = "foreign-file" }
examples = ["frob sim run quad-a --scene terrain:stairs@0.4 --worlds 2048 --seconds 20 --tier gpu-cuda --key run-1 --json", "frob sim run g1 --tier referee --seconds 2 --key run-m0 --json"]
```

`exports` and `option_capabilities` enter with N37-18 (M3), each field in the change that brings it; RL-22 holds
because `resolved_out` is an input property of the row. The `actuation` member of `needs_groups` enters with
N15a-05 (M1), in the change that registers its predicate, so the lint `needs-groups-known` holds at every step.
`memo = true` enters with BD-111 (M1), in the change that adds the field to N32a-02's later-field table.

**Conventions the rows follow.**

1. The MCP tool name is the verb name with `.` and `-` replaced by `_`. The output on the wire is one envelope
   `frob.envelope/1` with `result` specialized to the output schema; its `evidence` block is a claim product (the
   weakest class, tier and determinism class among the inputs).
2. The generator adds the common input properties of the kind (`run`: `sync_wait_ms` from 0 to 30,000 with MCP
   default 5,000, `idempotency_key`, `budget`, `estimate_only`, `on_cancel` (`leave`, the default, or
   `cancel_job`) and `envelope_scope` (`--envelope-scope <gate:…|campaign:…|pretrain:…>`: the approved compute
   envelope scope the job cites; its schema, its place in the scope order and its refusals are NJL-01's);
   `mutate`: `idempotency_key`, `expected_parent`). `idempotency_key` is in no JSON-schema `required` array: the
   annotation `x-frob-key` (`"required"`, or `"optional"` on a `key_optional` entry) carries the rule, and a
   missing key is refused with 805 by the admission transaction on both surfaces. The generator appends the
   surface-wide refusals: `FROB-REF-720` (scope missing), 723 (token invalid), 724 (not implemented in this
   build) and 802 (schema failure, including an unknown parameter) for every verb; 701 to 704 (budget and
   admission), 761 (above the compute envelope in force), 803 (key reused with different content) and 805
   (missing key) for `run`; 725, 803, 804 (stale parent) and 805 for `mutate`. The other properties and codes are
   added by field, as the variation table of NJL-01 lists them (that table is the one list; its variations 10
   to 16, which later beads land, are read there and not copied here): 806 and 807 for paginated reads; 808 (an
   integer outside ±(2⁵³ − 1)) for an entry whose generated input schema has an `int53` property and for no
   entry by kind (every `run` entry has one in `sync_wait_ms`, every paginated entry in `limit`); a
   `key_optional` entry is not given 805; a non-`run` entry with `estimable = true` gets `estimate_only`; 790
   for a non-empty `needs_groups`; 891 to 893 for artifact `params`; 734 and 828 for a parameter that accepts a
   path to a file (`artifact-input<F>`, `artifact-identity<F>`, `foreign-file`).
3. Exit codes: 0 done; 2 done with findings; 3 refused input (1xx to 5xx, and 8xx other than conflicts); 4 budget
   (70x and 761); 5 capability (72x, 73x); 6 device (6xx); 7 conflict (803, 804, 809); 8 accepted and still running; 9
   indeterminate; never 1 on purpose.
4. Rows are entered when the cluster is first registered (M0), each with the `task_clusters` of the
   machine-fields table, which is NJL-05's target assignment. No `[[task_cluster]]` table exists at M0: NJL-05
   declares the tables at M1 and seats these rows in them (its staging rule, step 1), and from then
   `task-cluster-mismatch` holds for them. A verb whose handler is not in the build answers one envelope with `FROB-REF-724`, exit 5, on both surfaces, and `frob capabilities --json`
   lists it as absent; a reduced form that lands before the row's `first_milestone` answers and is listed as
   implemented with its form named (NJL-01, NJL-03: the field keeps the appendix's milestone). Inputs enter a row
   by the rule "How an input enters a row" above.

**Notes for this cluster.**

- `sim.run` first answers at M0 for the referee tier on the built-in fixture and at M2 on all tiers. `--profile`
  takes a step profile (`lockstep`, `strict`, `referee`); determinism and precision profiles take `--determinism`
  and `--precision`. Fast-tier results are `provisional` until promoted.
- `sim.estimate` never runs physics; its interval is conformal per stratum and admission uses the upper end.
- `sim.replay --tier` teleports a world to another tier and localizes the first step and stage at which the tiers
  differ.

## Acceptance criteria

- [ ] `registries/verbs.toml` holds exactly three entries with `cluster = "sim"` (1 `read`, 2 `run`), each with
      every field of the entry schema, an example and a task-cluster list; the alias lint finds no second spelling.
- [ ] `frob-surface-diff parity --cluster sim` exits 0 in process (`frob-mcp` over stdio) and through `frobd --mcp`:
      registry, CLI and MCP each count 3, and every difference list is empty apart from the harness's closed exemptions.
- [ ] Every verb has an end-to-end script: `scripts/e2e/verbs_sim.sh` runs each verb of the cluster (three), and
      from a verb's first milestone a capability script of its implementing bead exists (`frob-surface-diff
      e2e-coverage`).
- [ ] `frob <verb> --help --json` equals `fixtures/golden/verbs/sim/<verb>.help.json` byte for byte for each of the
      three, on two runs, debug and release, Linux and macOS; a planted renamed output property is classified `breaking`
      and refused.
- [ ] A verb not yet implemented answers `FROB-REF-724`, exit 5, with one envelope on both surfaces; an implemented
      verb runs its registry example with the result its implementing bead specifies; a misspelt parameter is
      `FROB-REF-802`, exit 3; a `run` or `mutate` call without a key is `FROB-REF-805`, exit 3.
- [ ] The generated refusal table lists for each verb exactly its chapter codes plus the codes appended for its
      kind and fields (NJL-01), and every code exists in `registries/refusals.toml`; no code of the worded and
      proposed table is in a `refusals` field before its registry row exists.
- [ ] The generated `sim.run` input schema of a build holds exactly the inputs of the tables whose owners that
      build carries, with the domains and defaults stated; a release build's schema has none of the
      conformance-build-only or chaos-build-only inputs; every spelling of "Spellings ruled out" is 802.
- [ ] The gate row is recorded with the measured counts by a session that did not write the rows.

## Test plan

### Unit tests

`cargo test -p frob-verbs --test cluster_sim` (the real generated table; no mocks; planted registries are TOML
strings labelled `synthetic`):
- `rows_present`: exactly 3 rows in the cluster, with the names of the rows table; a planted extra row or a removed
  row fails.
- `kinds_milestones_tiers`: each row's kind, first milestone and tier equal the rows table (for example `sim.run` is
  `run` at M0).
- `schemas_and_scopes`: input schemas are `frob.<name>.in/1`; output schemas and capabilities equal the
  machine-fields table.
- `refusal_lists`: each row's chapter codes equal its block (`sim.estimate`: 160, 602, 726; `sim.replay`: 222,
  247, 601, 606, 791, 792, 816, 840, 894; `sim.run`: 102, 103, 132, 136, 160, 212, 220, 221, 222, 223, 224, 245,
  246, 247, 248, 457, 461, 464, 522, 529, 530, 531, 601 to 616, 721, 722, 726, 731, 733, 734, 791, 792, 816); the
  appended codes equal the rule for its kind and fields (790 on `sim.run` from `needs_groups`; 891, 734 and 828
  on all three from `params`, and 893 on `sim.run` and `sim.estimate` from `designs`; 806 and 807 on none).
- `worded_refusals_by_ledger`: each refusal of the worded table is in a row's `refusals` exactly under the number
  `registries/refusal_allocations.toml` records as `assigned` for its request (`randomize-numerical-parameter`
  and `randomize-no-interval` are assigned in N35-33's first round, so the M2 registry lists them on `sim.run`),
  and in no list while the request has no `assigned` value; no slug and no unassigned proposal (572, 571, 319,
  619) is in any list; a planted ledger (`synthetic`) that assigns `randomize-numerical-parameter` a number other
  than the one in `sim.run`'s list fails the test, naming the request and both numbers.
- `field_driven_lines`: `sim.run` has `exports = true`, `option_capabilities` naming `resolved_out`, which is an
  input property (RL-22), and the `needs_groups` of the block; a planted `option_capabilities = { out = [...] }`
  fails RL-22. From the build carrying BD-111, `sim.run` has `memo = true` and its generated input schema has
  `no_memo` (boolean, default false, outside every identity) from variation 14; a planted row that declares its own
  `no_memo` fails `common-property-shadowed`.
- `operator_inputs`: the generated `sim.run` input schema has `task`, `resolved_out`, and `workers` (1 to 512) and
  `tile_worlds` (power of two, 1 to 1,024, default 32) marked operator-only; `--tile-worlds 24`, `--workers 0` and
  either field with `--tier gpu-cuda` are `FROB-REF-802`, exit 3; `workers`, `tile_worlds`, `resolved_out` and
  `profile-kernels` stay out of every identity (two calls differing only in `workers` have one `StepPlanId`), while
  `task` is part of the request.
- `run_inputs`: `sim.run` accepts `--on-overflow` with exactly the three policies and `--peak-capture` (from the
  build carrying N29ax-02); `--seconds` with `--steps`, `--worlds` with `--worlds-per-design`, `--trace-rate` with
  `--trace-size`, `--controller` with `--policy`, and `--fixture` with a positional design are each 802; `--seed
  18446744073709551615` is accepted and `--seed -1` is 802; `--actuator-level L4` is 724 and `L5` is 802;
  `--step-plan fused-pilot` is 724 on a release build; `--policy zero` is accepted on every build, while
  `--record trajectories` and `--horizon` are `FROB-REF-724`, exit 5, on a build where KILL-GYMADAPT's decision
  in force is not `fired`; a release build answers `--inject-faults seed=1,count=1` and `--chaos-plan p.toml` with 802;
  `sim.replay` accepts the positional episode and the `--run --world` form, rejects `--from` with
  `--from-snapshot` (802), and `--capture failure` answers 724 until N29ap1-03 lands.
- `ruled_out_spellings`: each spelling of the "Spellings ruled out" table is 802 with the replacement as the
  first repair's nearest valid parameter where one exists (`--level` → `--actuator-level`, `--mode` →
  `--purpose`, `--seeds` → `--seed`).
- `identity_inputs`: two `sim.run` requests differing only in one input whose identity column is `StepPlanId`
  (`--joint-friction`, `--capacity`, `--solve-path`) have different `StepPlanId`s; differing only in
  `--telemetry` or `--trace-rate` they have one `StepPlanId` and different `RunId`s.
- `mcp_names_one_to_one`: the tool names are the mapped names (`sim_estimate`, `sim_replay`, `sim_run`), with no
  collision against any other cluster.
- `task_clusters_and_examples`: every row sits in a task cluster of at most seven tools or is a base tool; each
  example parses against the generated CLI and its command path is the row's own; every `run` example carries
  `--key`.
- `kind_behaviour`: a `run` row has the six common properties (`sync_wait_ms`, `idempotency_key`, `budget`,
  `estimate_only`, `on_cancel`, `envelope_scope`), with `idempotency_key` outside the schema's `required` array
  and annotated `x-frob-key: "required"`, and refuses `sync_wait_ms = 30001` with code 802; a verb whose
  capability is absent returns code 724, exit 5, one envelope.

### End-to-end script and logging

`scripts/e2e/verbs_sim.sh` (header: `bead=VERBS-sim milestone=M0
labels=tether,verbs,verb:sim.estimate,verb:sim.replay,verb:sim.run gates=GATE-TETHER-02 seed=1 needs=cpu,mcp,frobd
plants=alias,missing-row,renamed-field,unlisted-input`) sources the shared helper and `scripts/e2e/verbs_lib.sh`
and calls `verbs_cluster_conformance sim`. Steps: (1) `frob capabilities --json` and `frob robot-docs --json` list
the three verbs with kind and milestone; (2) `frob-surface-diff parity --cluster sim` over stdio and through
`frobd --mcp`, every difference list empty; (3) `frob-surface-diff snapshot --check --cluster sim`; (4) each
verb's registry example verbatim, expecting a result (exit 0 or 2), a job handle (exit 8, then `frob job wait`) or
`FROB-REF-724` exactly when the capability report says the verb is absent; (5) for each implemented verb the same
call as an MCP tool, the two envelopes agreeing on `verb`, `ok`, `exit`, `result`, `refusal`, `evidence`, `bound`,
`warnings` and `next_actions`; (6) per verb one call with a misspelt parameter (`e2e_expect_refusal FROB-REF-802
3`) and, for `run` and `mutate` verbs, one without a key (`e2e_expect_refusal FROB-REF-805 3`, from M1); (7)
`frob sim run --help --json` lists exactly the inputs of the tables present in the build (logged as the set
difference against the tables, expected empty), and on a release build one call per conformance-build-only input
answers 802; (8) one envelope per call and no exit code 1.

Log: `target/e2e-logs/verbs_sim/<utc>.ndjson` with a transcript beside it. Each `cmd` line has the exact command,
exit code, wall time and the envelope's `evidence` block; each `assert` has observed and expected values and the tag
`--gate GATE-TETHER-02`; the metrics `registry_verbs`, `cli_verbs` and `mcp_tools` are logged against 3 and the
difference counts against 0; `sim_run_inputs` is logged against the count the build's tables give; refusals are
logged with code and first repair; a failure logs the first differing verb, field or input; the `summary` line gives
assertions passed and failed and the gate touched.

### Known answers and negative controls

- Known answers: the cluster has 3 verbs (1 `read`, 2 `run`); the tables above. KA-755 (the exit-code table on
  malformed, unknown, over-budget, unscoped and conflicting inputs gives the exit and refusal code of the surface's
  tables; exact) is exercised for every verb by steps (6) and (8).
- Negative controls the script must catch when planted: `alias` (a second spelling of one verb: the build fails with
  the alias lint); `missing-row` (a row removed from the served MCP list: `missing_mcp` is not empty);
  `renamed-field` (an output property renamed without a version change: the snapshot check reports `breaking`);
  `unlisted-input` (an input property served by `sim.run` that no table lists: step (7) reports it). A verb
  answering with two envelopes or exit code 1 fails step (8).
- Invariants carried: INV-TETHER-01 (every surface is generated from the registry; nothing exists on one surface and
  not the other, apart from declared per-surface defaults and the unknown-verb case) as step (2); INV-TETHER-02
  (exactly one envelope per invocation with `--json`) as step (8). Determinism lanes: entries, tool definitions and
  snapshots byte-identical twice on one machine, across debug and release and across Linux and macOS (class D0); the
  verbs' own lanes belong to their implementing beads.

## Considerations

- Closing rule: the bead closes at M0, when the rows are registered, the surfaces match and every verb passes its
  example or answers the not-implemented refusal. Verbs of this cluster first answer at M0, M2. The per-milestone
  surface beads (BD-022, BD-222) re-run this script as verbs land; that is how GATE-TETHER-02 is re-measured, and
  each later input enters by the rule "How an input enters a row".
- Scope names are proposals (`<cluster>:<read|run|write|admin>` plus the named scopes of the surface) until the
  capability bead BD-258 fixes the scope registry; a `run` or `write` scope of a cluster implies its `read` scope.
  For `sim.run` and `sim.replay` the input tables fix the flag spellings, and a mechanism bead that spells an input
  otherwise changes (KNOWN_DRIFTS 12); elsewhere inputs and outputs name contents, and the implementing bead fixes
  the spellings. The snapshot then pins them (additive changes keep the schema's major version; anything else
  raises it).
- DESIGN choices made here where the plan is silent, each with its reason: the positional `design` (one
  spelling, which BD-024 uses); exactly one of `--seconds`, `--steps`, `--intervals` (all three are used, and a
  run has one length); one `--purpose` input on both rows with default `evaluation` (two mechanisms need the same
  input; `evaluation` is the purpose of a run nobody declared, and N19b-03's admission substitutes for it rather
  than refusing); `--telemetry` default `T1` (BD-207's default scopes); `--state` default `estimated` (§22's rule
  that a privileged-only result is not a result); fixture controllers in every build (N14b-06 runs them on
  release builds, and N15bp2-01's fixture controller is present in every build); `--snapshot-every` as the
  spelling of the manifest's `t3.every_chunks`; `--txn` only under transactional profiles; the predicate
  `design-has-port-actuators` for the `actuation` group (only a run whose design composes actuators through
  `fs-phs` needs it). Each is a choice a later owner may revise through this row, not a fact of the plan.
- The row is long because `sim.run` is the program's common run surface. Rejected: splitting it into several
  verbs by option family (every script would then need two verbs for one run, and the identity rules would be
  stated twice); leaving options to the mechanism beads without a row (the round-2 review found more than forty
  options in scripts that no row listed, each one an 802 on the real binary).
- Risks: an owner bead may later specify an option's behaviour differently from its row; the owner's text then
  wins for behaviour and this row is updated in the same change, keeping the spelling unless the plan gives
  another.

## References

Appendix J (J.1 entry form; J.2 inventory rows; J.5 task clusters), §16.13, §22, §29.8, §32.9, §32.4, §32.6,
§32.12.3, §32.18, §32.21, §35, §36.10; GATE-TETHER-02; INV-TETHER-01, -02; KA-755; KNOWN_DRIFTS 5, 6, 8 and 12.
