---
key: "VERBS-control"
title: "frob-verbs registry: cluster `control`, seven rows, with parity, e2e and schema-snapshot checks"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "tether", "reflex", "frob-verbs", "solid", "registry", "verb", "test", "review-notes-open"]
parent: "WP-06"
deps: ["BD-022", "NJL-01", "NJL-02", "NJL-04", "NJL-05", "STD-E2E"]
needs: []
related: []
gates: ["GATE-TETHER-02"]
plan_refs: ["Appendix J (J.1, J.2, J.3, J.5)", "§22.3", "§22.4", "§22.7", "§22.8", "§22.13", "§22.20", "§32.6", "§32.12.3", "§32.21 (GATE-TETHER-02)"]
---
## Background

FrankenRobotics (a robotics simulation and design system) is reached through the command line `frob`, the daemon
`frobd` and the MCP server `frob-mcp` (the Model Context Protocol, by which an agent harness lists and calls tools).
Every capability is a verb, `cluster.verb`, declared once in `registries/verbs.toml` (crate `frob-verbs`, tier
Solid). The CLI, the daemon protocol, the MCP tools, the schemas, the refusal table and the agent guide are
generated from that file, and gate GATE-TETHER-02 compares the served surfaces with it. The registry holds 244 verbs
in 44 clusters (namespaces): the 243 of the specification and `artifact.import` (VERBS-artifact); the
count of record is NJL-03's.

This bead is the registry of record for the cluster `control`. The cluster covers the estimator, the baseline
controller and the control rungs above it: synthesis by morphology class with three outcomes, the invariant filter
replayed on a run, the sampling-based predictive controller as a screening rung, symmetric controllers, and a lifted
planning model for soft soil. It has seven verbs (4 `read`, 3 `run`); the handlers live in `frob-control`. The bead
carries every row completely, with the three checks that keep the rows true: the generated CLI commands and MCP
tools match the list; every verb has an end-to-end script; the schema printed by `frob <verb> --help --json` is
stable.

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

In: the seven registry entries of cluster `control`; the MCP tool names; the cluster's conformance script; the
help-schema snapshots; each verb's answer before its milestone. Out: behaviour (the implementing beads named below);
the entry schema and lints (NJL-01); the conformance harness (NJL-02); the alias lint (NJL-04); task clusters
(NJL-05, NJL-06); generation of the surfaces (the Tether seed beads).

## Technical approach

**The rows.**

| Verb | Kind | First milestone | Tier | Purpose | Defined in |
|---|---|---|---|---|---|
| `control.estimate` | read | M1 | S | Replay a run's packets through the invariant filter and its comparator | §22.3 |
| `control.ladder` | read | M3 | S/F | The control rung a design is on, the reason, and the next experiment with its cost | §22.7 |
| `control.mpc` | run | M3 | F | The sampling-based predictive controller, as a screening rung; three episodes per design by default, and up to the break-even count for a paired screening comparison | §22.8 |
| `control.symmetry` | read | M4 | S | Γ_mech, Γ_obs, the stabilizers in force and the group a controller may use | §22.13 |
| `control.synth` | run | M1 | S | Synthesize the baseline controller for a morphology class, with three outcomes | §22.4 |
| `control.show` | read | M1 | S | Show a synthesized controller | §22.4 |
| `control.lift` | run | M6 | M | Fit and calibrate a lifted linear planning model for one soft-soil regime | §22.20 |

**Contract of each verb.** Evidence labels used below (the `evidence` block of the envelope):
- *static*: class Tested, tier `static`, determinism `exact` (class D0); `claims` empty unless a certificate is
  attached; `reality: no-claim`.
- *pass-through*: passes through the claim product of the stored objects it reads and adds no claim of its own; the
  read itself is `exact`.
- *screening*: a screening or planning signal that may discard under the ladder's rules and may not establish a
  comparison; labelled `screening: sampling-control`; class as stated in the block (a sampling-controller episode
  is `Tested` on its tier as a screening result, a surrogate planner's result `Estimated`).
- *replay*: the claim product of the run replayed (its tier and determinism class), class `Tested`; claim kind
  Known answer when the gate fixture is the input and empty otherwise; `reality: no-claim`.

**`control.estimate`** (MCP tool `control_estimate`; owner `frob-control`)
- Inputs: `run` (a run or episode identifier); optional `world` (default the first traced world); optional
  `window` (two instants of the run's clock); `comparator` (default true); optional `filter_manifest` (replay
  under another manifest than the recorded one); `align` (only `initial`; anything else is 464); cursor
  (paginated: long event lists are paged, never cut).
- Outputs: for each of `invariant` and `comparator`: aligned drift and drift fraction, maximum aligned error, tilt
  error, normalized innovation and estimation error with their χ² bounds, covariance coverage, flags
  `overconfident` and `drifting`, `unobservable_dim`, the log-linearity departure, bias estimates; common fields:
  packet counts by sensor, touchdown and liftoff counts, `contact-downweighted` and out-of-sequence events, the
  filter and scoring manifest hashes, the window. A run without recorded truth gets innovation statistics only,
  with the truth-based columns `not available`. Exit 2 when the invariant filter is flagged. Evidence: *replay*.
- Chapter refusals: `FROB-REF-464` (an estimator was scored with a fitted scale, or an estimated-state run lacks a
  sensor the filter needs); `FROB-REF-888` (a metric computed over missing samples: no packet stream for the
  world, or a recorded gap in a channel the filter needs inside the window; the trace chapter's code, owned by
  BD-090, raised by this verb).
- Implemented by: BD-114, N22a-04. Cost: static, linear in packets; under 1 s for a 20 s run of a 12-joint robot.

**`control.ladder`** (MCP tool `control_ladder`; owner `frob-control`)
- Inputs: `design`; optional `task`; optional `campaign` (whether the design is a finalist, which R4 requires).
- Outputs: `class`; the control rung (R0 to R4) with its status and the result it rests on; the recorded reason
  with the rule that classified it; `escalation` (`next`, `none` for a design refusal, with the witness and the
  typed edits the design check proposes, or `undetermined` for a numerical or budget failure); the next experiment
  with rung, description, cost (TARGET), `available` and its evidence items, each `present` (with its run) or
  `missing` (with the task that produces it). Exit 2 for a design refusal. Evidence: *pass-through*.
- Chapter refusals: none of its own. `FROB-REF-537` (a rung used to select or discard without a rank-agreement
  record that meets its threshold) is not raised here: this verb selects and discards nothing; 537 fires where a
  baseline score is consumed, in N22a-17's `permits` check on the campaign and evaluation-ladder verbs, whose rows
  list it.
- Implemented by: N22a-18 (the verb), BD-330 (the R2 and R3 rung results it reports). Cost: static, under 100 ms;
  reads the ledger.
- Tier `S/F`, registered `S` (N22a-18, DESIGN with its reason): the verb, its registry data and the rules of rungs
  R0 and R1 are Solid, are built at M3, wait for no pilot and stay if KILL-RESIDUAL fires; only the R2 and R3 rows
  are Frontier, and they appear as `available: false` until BD-330 and its pilot have run.

**`control.mpc`** (MCP tool `control_mpc`; owner `frob-control`)
- Inputs: design; task; optional planner setting (default the class's registered setting); tier; seeds;
  `episodes` (capped at three per design unless `demonstrations` or `compare` is present); `demonstrations <job>`;
  `compare <id>` (a paired screening comparison, up to the class's break-even count); `freeze`; `planner`;
  `planning_model`; `state`; `latency` (`period`, the default, `measured:<computer>` or `zero`, labelled
  `zero-delay`); `protocol <id>` (an experiment admitted in the method-experiment register, N38a-07, or a pilot
  whose protocol is frozen in `crates/frob-conformance/registry/pilots.toml`, N38a-09, declaring the sampling
  controller's episode count per design; N22a-24, BD-252); the diagnostic input `diag_replay <run>` with
  `forks <n>` (marked diagnostic in the entry); from N22c-31 `measure_latency` with `computer`; from N28c-33
  `ladder <id>` with `level <i>` or `levels all`; from M6 (N22c-24) `planner lifted:<id>`.
- The cap (N22a-24): at most three screening episodes per design and task, counted across requests under the
  scope key (the campaign, or outside a campaign the caller's namespace; `DesignId`; `TaskId`), whatever the
  planner setting; a repeat under the same idempotency key, `sim.replay` and `diag_replay` are not counted. Four
  named exceptions, each recorded on the result, each with its own count under the same key and none drawing on
  the default three: `demonstrations` (a distillation job), `compare` (up to the class's break-even count),
  `freeze` (no fixed count; bounded by its budget) and `protocol` (up to the count the protocol declares; an
  unregistered identifier, or a protocol declaring no count, leaves the default three in force).
- Outputs: per episode the requirement verdict as a screening result, the run identity, the labels
  (`screening: sampling-control`, `model-privileged` or not, planning step, latency), the pricing record with
  predicted and measured cost, planner diagnostics; the demonstration-set identity for `demonstrations`; the
  frozen record's digest for `freeze`; with `protocol`, `protocol: <id>` and the protocol hash on every result; a
  request beyond its cap is refused at admission with the arithmetic. Evidence: *screening*, class `Tested`
  (`Estimated` with a lifted planner).
- Chapter refusals: `FROB-REF-817` (sampling-controller episodes offered as the scoring source of acceptance,
  promotion or qualification; raised by this verb when `freeze` is asked for a configuration that could never be
  admitted to the delivered-controller lane: `model-privileged`, `latency-unmeasured`,
  `planning-model-not-compiled`, or a lifted planner (N22c-31, N22c-24); the payload names the missing condition).
  Two worded refusals have no number yet and join the list in the change that assigns each (N35-33's rule): the
  episode cap, slug `mpc-episode-cap` (family `4xx`, exit 3, nothing admitted; payload `requested`, `cap`, the
  exception and its count, the episodes on record with their runs, steps per episode, the cost against training
  once, the class's break-even count; N22a-24, registered by slug in N22-REF, ledger row in N35x-02 (Control,
  §22.8)); and from M6 with `planner lifted:<id>`, slug `lifted-planner-out-of-regime` (family `4xx`, N22c-23,
  ledger row in N35x-02).
- Implemented by: BD-252; the caps, labels and pricing record by N22a-24.
- Extended by: N22c-24 (`--planner lifted:<id>`), N22c-31 (`--freeze`, `--measure-latency --computer <name>`),
  N28c-33 (`--ladder <id>`, `--level <i>` | `--levels all`).

**`control.symmetry`** (MCP tool `control_symmetry`; owner `frob-control`)
- Inputs (BD-327's spellings): design (positional); task (`--task`); optional `controller` (`--controller
  <ControllerId>`: the verb checks the group recorded per mode in its manifest against the context); optional
  context: gravity direction in the body frame, `--gravity "<x y z>"`, or a slope, `--slope <deg>`; a world patch
  or scene, `--scene <scene>`; a contact mode, `--mode "<feet in stance>"`; a commanded twist, whose linear and
  angular parts are passed by name so that no ordering convention is assumed, `--twist-linear "<vx vy vz>"` and
  `--twist-angular "<wx wy wz>"` (default level ground, four-foot or all-wheel stance, zero twist); optional
  `events <class>` (`--events`, the event-identity check, which may simulate a landing and so takes a budget).
- Outputs: blocks `mech` (Γ_mech: order, generators, each element's isometry with its exact matrix and
  determinant, body permutation, signed joint permutation), `obs` (Γ_obs, and for each element outside it the
  breaker and the typed edit that would restore it), `stabilizers` (for gravity, boundary conditions, contact
  mode and commanded twist the subgroup and the elements of the running intersection it removed, with reason),
  `use` (Γ_use for the context, its identity, and the controller manifest's equivariance mode, pathwise or in
  law, or `none declared`), `events` on request (per generator and event class the identity residual against
  tolerance, or a grazing flag). The verb changes nothing. Evidence: *static* for exact contexts (with a
  certificate reference where emitted); `Tested` with the tolerance for toleranced contexts.
- Chapter refusals: `FROB-REF-443` (a symmetry consumer names a group element outside Γ_obs; raised through the
  consumer guard of N20b-02); `FROB-REF-461` (a symmetric reduction was requested under a group not contained in
  Γ_use). Precedence 443 before 461: an element outside Γ_obs is the coarser error and is named first.
- Implemented by: BD-327.

**`control.synth`** (MCP tool `control_synth`; owner `frob-control`)
- Inputs: design; optional gait; optional `task` (`--task <task>`, the task class; from M4 it is also the
  context of the symmetry reduction, BD-327: the task's scene supplies gravity in the body frame and the boundary
  conditions, its command the twist, because `control.synth` has no `--scene` input); `require` (the caller
  needs a controller: 421 can fire); tier;
  `seed` (the seed of the standing smoke episodes, which the gate scripts N22c-01, N22c-47 and N22c-48 pass);
  `rung` (`R0`, the default, or `R1`, which requires `task`; `R2` and above are `FROB-REF-802` naming BD-330's
  runner; N22a-19); `class` (a row of the class table admissible for the detected class, for example
  `legged-predictive` for `legged`; any other pairing is 421 with the admissible rows listed; N22a-21); from M4
  (BD-327) `symmetry` (`use`, the default: reduce under Γ_use where it is non-trivial; `mech`: request Γ_mech,
  refused wherever Γ_use is smaller; `none`: no reduction) and `verify_reduction` (run the full twin and compare
  at synthesis, N22b-38). There is no `state` input (DESIGN, from BD-118 step 4): the standing smoke always runs
  under estimated and under privileged state and reports both and their difference, because a controller that
  works only on privileged state is not a result; an input choosing one would let a caller report the privileged
  half alone. The synthesis has no precision input either; the two widths are compared through `sim.run
  --precision` (N22c-47).
- Outputs: always `controller_id`, class, tier and the note "a failure under the baseline is not a verdict on the
  design"; one of three outcomes (§22.6): `ok` (saturation report, fraction of ticks per layer, standing
  `screening` until GATE-REFLEX-04 has passed for the class and task family, estimated-versus-privileged
  difference), `synthesized-failed` (failure classes, outcome kind `physical`, `numerical`, `budget` or
  `protocol`, the fallback log, the first layer that failed, the next rung with its required evidence) or
  `not-synthesizable` (reason, class, a rational witness when the reason is a program, the admitted fallbacks);
  with `rung R1`, the R1 report as a variant beside the three. Exit codes (N22a-12's report type, which BD-118
  cites): 0 for an `ok` that raises no flag; 2 for an `ok` whose report carries a saturation or clamp flag (any
  `stance-saturated` or `footstep-clamped` tick, or limiter clipping in the saturation report), and for
  `synthesized-failed` and `not-synthesizable`; 3 for 421 and 424. Evidence: class `Tested` on the stated tier
  (`Certified` for the stated program when a `not-synthesizable` carries a checked certificate, capped by its
  weakest input; N22a-12), determinism D2 on the batched tiers.
- Chapter refusals: `FROB-REF-421` (no synthesis route exists for the detected morphology class and no fallback is
  admitted); `FROB-REF-424` (a control program is infeasible and the controller has no admissible fallback); from
  M4 `FROB-REF-443` and `FROB-REF-461` (a reduction requested under a group with an element outside Γ_obs, or
  outside Γ_use; precedence 443 before 461; BD-327; N22-REF lists `control.synth` among the raisers of 461).
- Implemented by: BD-118. Extended by: N22a-19 (`rung`), N22a-21 (`class`), BD-327 (`symmetry`,
  `verify_reduction`, 443 and 461, M4).

**`control.show`** (MCP tool `control_show`; owner `frob-control`)
- Inputs: controller or design.
- Outputs: the class, the gait and phase table chosen, each derived gain with its derivation (natural frequency,
  reflected inertia), the programs with iteration counts, the latency model, the group used, the identity, the
  standing and the last synthesis outcome; from N28c-33, `family` and `budget {ladder, level, effort}`. It runs
  nothing. Evidence: *pass-through*.
- Chapter refusals: none of its own.
- Implemented by: BD-118. Extended by: N28c-33 (family and budget fields).

**`control.lift`** (MCP tool `control_lift`; owner `frob-control`)
- Inputs (N22c-23): design; `soil` (a soil card); `regime` (a regime file: soil card ranges, speed or yaw box,
  slope range; a path parameter, so the field rule applies); exactly one mode, `fit`, `calibrate` with `runs <run
  id>` (the calibration runs), or `report`.
- Outputs: the `LiftId`, the regime, n_fit and n_cal, λ, the band per horizon step, the mode-change share, the
  admission verdict with the failing clause if any, fit and calibration cost. Evidence: *screening*, class
  `Estimated`, labels `screening: sampling-control` and `surrogate`.
- Chapter refusals: `FROB-REF-243` (soil or tyre law out of range: Pressure, slip or sinkage outside the card's
  validity range; surfaced from the contact layer, BD-366, when data generation drives the soil law outside the
  card's range). The worded refusal `lifted-planner-out-of-regime` (a query outside the calibrated regime; family
  `4xx`, exit 3; ledger row in N35x-02, no number yet) joins this list and `control.mpc`'s in the change that
  assigns its number. `FROB-REF-421`, listed by an earlier draft, is raised by no step of N22c-23 (the verb fits
  and calibrates a planning model and synthesizes no controller), so it is not listed.
- Tier `M`: the plan marks §22.20 [M] and N22c-23 is a Moonshot, off the critical path; an earlier draft had F.
- Implemented by: N22c-23.

**Machine fields of each row.**

| Verb | Capabilities | Output schema | Task clusters | Example |
|---|---|---|---|---|
| `control.estimate` | `control:read` | `frob.control.estimate.out/1` | `control` | `frob control estimate r:19e2 --json` |
| `control.ladder` | `control:read` | `frob.control.ladder.out/1` | `gaits`, `control` | `frob control ladder quad-a --task stairs-0.4 --json` |
| `control.mpc` | `control:run` | `frob.control.mpc.out/1` | `gaits`, `control` | `frob control mpc quad-a --task stairs-0.4 --episodes 3 --key mpc-1 --json` |
| `control.symmetry` | `control:read` | `frob.control.symmetry.out/1` | `gaits`, `control`, `symmetry` | `frob control symmetry quad-a --json` |
| `control.synth` | `control:run` | `frob.control.synth.out/1` | `author`, `control` | `frob control synth quad-a --key ctl-1 --json` |
| `control.show` | `control:read` | `frob.control.show.out/1` | `control` | `frob control show c:2be8 --json` |
| `control.lift` | `control:run` | `frob.control.lift.out/1` | `audit`, `control` | `frob control lift tracked-a --soil sand-dry --regime regime.toml --fit --key lift-1 --json` |

The remaining fields follow by rule: `input_schema` is `frob.<verb name>.in/1`; `refusals` is the chapter list of
the verb's block; `idempotent` is `true` for `read` and `admin` and `false` for `run` and `mutate` (which take an
idempotency key); `cost_model` is `static: …` for `read`, `mutate` and `admin` and the `--estimate` cost model for
`run`; a `run` row carries `wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }`; `owner` is the
crate of the block; `purpose` is the Purpose column. Stated bounds: `control.synth`: static: under 5 s for a
template-sized design (TARGET); a job above that; `control.estimate` and `control.ladder` as their blocks state.
`control.estimate` has `paginated = true`. The input `diag_replay` of `control.mpc` is marked diagnostic in its
input schema, so that agents do not take it for a control.

One entry in full (the others follow field for field):

```toml
[[verb]]
name = "control.mpc"
cluster = "control"
kind = "run"
tier = "F"
capabilities = ["control:run"]
input_schema = "frob.control.mpc.in/1"
output_schema = "frob.control.mpc.out/1"
refusals = ["FROB-REF-817"]
idempotent = false
cost_model = "estimated: `--estimate` returns CPU core-seconds, GPU seconds, memory and disk with a conformal interval"
first_milestone = "M3"
task_clusters = ["gaits", "control"]
examples = ["frob control mpc quad-a --task stairs-0.4 --episodes 3 --key mpc-1 --json"]
owner = "frob-control"
purpose = "The sampling-based predictive controller, as a screening rung; three episodes per design by default, and up to the break-even count for a paired screening comparison"
wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }
```

**Conventions the rows follow.**

1. The MCP tool name is the verb name with `.` and `-` replaced by `_`. The output on the wire is one envelope
   `frob.envelope/1` with `result` specialized to the output schema; its `evidence` block is a claim product (the
   weakest class, tier and determinism class among the inputs).
2. The generator adds the common input properties of the kind, as NJL-01's kinds table lists them (`run`:
   `sync_wait_ms` from 0 to the row's `wait.mcp_max_ms`, 30,000, with MCP default 5,000; `idempotency_key`, CLI
   `--key`; `budget`; `estimate_only`, CLI `--estimate`; `on_cancel`, `leave` (the default: the job keeps
   running) or `cancel_job`; `envelope_scope`, CLI `--envelope-scope <gate:…|campaign:…|pretrain:…>`, the
   approved compute-envelope scope the job cites (program convention 14, owned by NJL-01); `mutate`:
   `idempotency_key` and `expected_parent`). `idempotency_key` is never in the JSON-schema `required` array: the
   annotation `x-frob-key: "required"` (`"optional"` on a `key_optional` entry) carries the rule, and the
   admission transaction refuses a missing key with 805 on both surfaces. The surface-wide refusals appended by
   kind: `FROB-REF-720` (scope missing), 723 (token invalid), 724 (not implemented in this build) and 802 (schema
   failure, including an unknown parameter) for every verb; 701 to 704 (budget and admission), 761 (above the
   compute envelope in force), 803 (key reused with different content) and 805 (missing key) for `run`; 725, 803,
   804 (stale parent) and 805 for `mutate`. The properties and codes driven by fields are those of NJL-01's
   variation table: `cursor`, `limit` and `follow` with 806 and 807 for a paginated entry; 808 (an integer outside
   ±(2⁵³ − 1)) for an entry whose generated input schema has an `int53` property at any depth, whatever its kind
   (every `run` entry has one, `sync_wait_ms`); `estimate_only` on a non-`run` entry with `estimable = true`; no
   805 for a `key_optional` entry; the `action` property, with each action's codes, for an entry with `actions`;
   790 for a non-empty `needs_groups`; 891 for an `artifact-input<F>`, `artifact-ref<F>` or
   `artifact-identity<F>` parameter, and 892 as well for `artifact-identity<F>`; 893 for a `foreign-file`
   parameter; 734 and 828 for a parameter that accepts a path to a file (`artifact-input<F>`,
   `artifact-identity<F>`, `foreign-file`). The later variations of that table (10 to 13) land with their beads
   and are read there, not copied here.
3. Exit codes: 0 done; 2 done with findings; 3 refused input (1xx to 5xx, and 8xx other than conflicts); 4 budget
   (70x and 761); 5 capability (72x, 73x); 6 device (6xx); 7 conflict (803, 804, 809); 8 accepted and still running; 9
   indeterminate; never 1 on purpose.
4. Rows are entered when the cluster is first registered (M1); each row's tool is added to the task-cluster
   tables its `task_clusters` names in the same change (NJL-05's staging rule). A verb whose handler is not in
   the build answers one envelope with `FROB-REF-724`, exit 5, on both surfaces, and `frob capabilities --json`
   lists it as absent; a reduced form that lands before the row's `first_milestone` answers and is listed as
   implemented with its form named (NJL-01, NJL-03: the field keeps the appendix's milestone).

**Notes for this cluster.**

- `control.synth` never returns a bare failure: its three outcomes are `ok`, `synthesized-failed` and
  `not-synthesizable` (§22.6, BD-118). An earlier draft named them "synthesized, synthesized with stated limits,
  refused"; the outcome names are the plan's. `not-synthesizable` is a result (exit 2), distinct from
  `FROB-REF-421`, which fires only when the caller needs a controller (`require`) and has admitted no fallback.
- Two worded refusals of this cluster are raised by slug until numbered: `lifted-planner-out-of-regime`
  (registered by slug in N22-REF, ledger row in N35x-02, family `4xx`), which enters the `refusals` of both
  `control.lift` and `control.mpc` (its `planner lifted:<id>` form) when it receives its code; and
  `mpc-episode-cap` (N22-REF, N22a-24, family `4xx`), which enters `control.mpc`'s. Both are rows of N35x-02's
  ledger; their numbers come from the ledger's run, read by N22-REF from the generated hand-off page
  `docs/generated/refusal_round2.md`, and no number is typed here.
- Options added after a row's first milestone (`symmetry` and `verify_reduction` of `control.synth` at M4; the
  lifted planner of `control.mpc` at M6) are listed in the blocks with their milestone. Each enters the generated
  input schema when its extending bead lands (an additive change that keeps the major version); before that a
  call that passes one is an unknown parameter, `FROB-REF-802`. A listed code that is not yet raised is legal.
- `control.ladder` is printed `S/F` and registered `S` (RL-12 admits only S, F and M); `control.lift` is `M`.
- Path parameters: `regime` of `control.lift` accepts a path to a file, so the generator appends 734 and 828 to
  that row (NJL-01's field rule) and to no other row of the cluster.
- `control.mpc` no longer lists `FROB-REF-424`: the refusal registry N22-REF names `sim.run` and `control.synth` as
  the verbs that surface it (a conic program of the baseline infeasible at a tick with no admissible fallback),
  and the sampling controller's handler (BD-252) has no path that raises it.
- `control.mpc` is a screening rung: its episodes may discard and may not score acceptance, promotion or
  qualification (`FROB-REF-817`). Three episodes per design by default, and up to the break-even count for a paired
  screening comparison.
- `control.lift` first answers at M6; its row is registered with the cluster and answers the not-implemented refusal
  until then.

## Acceptance criteria

- [ ] `registries/verbs.toml` holds exactly seven entries with `cluster = "control"` (4 `read`, 3 `run`), each with
      every field of the entry schema, an example and a task-cluster list; the alias lint finds no second spelling.
- [ ] `frob-surface-diff parity --cluster control` exits 0 in process (`frob-mcp` over stdio) and through `frobd
      --mcp`: registry, CLI and MCP each count 7, and every difference list is empty apart from the harness's closed
      exemptions.
- [ ] Every verb has an end-to-end script: `scripts/e2e/verbs_control.sh` runs each verb of the cluster (seven), and
      from a verb's first milestone a capability script of its implementing bead exists (`frob-surface-diff
      e2e-coverage`).
- [ ] `frob <verb> --help --json` equals `fixtures/golden/verbs/control/<verb>.help.json` byte for byte for each of
      the seven, on two runs, debug and release, Linux and macOS; a planted renamed output property is classified
      `breaking` and refused.
- [ ] A verb not yet implemented answers `FROB-REF-724`, exit 5, with one envelope on both surfaces; an implemented
      verb runs its registry example with the result its implementing bead specifies; a misspelt parameter is
      `FROB-REF-802`, exit 3; a `run` or `mutate` call without a key is `FROB-REF-805`, exit 3.
- [ ] The generated refusal table lists for each verb exactly its chapter codes plus the codes appended for its
      kind and fields (NJL-01), and every code exists in `registries/refusals.toml`.
- [ ] The gate row is recorded with the measured counts by a session that did not write the rows.

## Test plan

### Unit tests

`cargo test -p frob-verbs --test cluster_control` (the real generated table; no mocks; planted registries are TOML
strings labelled `synthetic`):
- `rows_present`: exactly 7 rows in the cluster, with the names of the rows table; a planted extra row or a removed
  row fails.
- `kinds_milestones_tiers`: each row's kind, first milestone and tier equal the rows table (for example
  `control.lift` is `run` at M6 with tier `M`); `control.ladder`, printed `S/F`, is registered `S`.
- `schemas_and_scopes`: input schemas are `frob.<name>.in/1`; output schemas and capabilities equal the
  machine-fields table.
- `refusal_lists`: each row's chapter codes equal its block (`control.estimate`: 464, 888; `control.ladder`: none;
  `control.mpc`: 817, plus the two worded refusals once numbered; `control.symmetry`: 443, 461; `control.synth`:
  421, 424, 443, 461; `control.show`: none; `control.lift`: 243, and not 421); the appended codes equal the rule
  for its kind and fields, with 806 and 807 on `control.estimate` only, 808 on every row whose generated input
  schema has an `int53` property (the three `run` rows through `sync_wait_ms`, `control.estimate` through
  `limit`), and 734 and 828 on `control.lift` only.
- `mcp_names_one_to_one`: the tool names are the mapped names (`control_estimate`, `control_ladder`, `control_mpc`,
  `control_symmetry`, `control_synth`, `control_show`, `control_lift`), with no collision against any other
  cluster.
- `extension_inputs`: the generated `control.synth` input schema has `rung` (`R0` default, `R1`), `class` and
  `seed` and no `state` property (a call with `--state` is 802); from BD-327's landing it gains `symmetry`
  (`use` default, `mech`, `none`) and `verify_reduction`; the generated `control.mpc` input schema has `freeze`,
  `protocol`, `ladder`, `level` and `levels`, and `diag_replay` marked diagnostic; the generated
  `control.symmetry` input schema has `task`, `controller`, the context fields `gravity`, `slope`, `scene`,
  `mode`, `twist_linear` and `twist_angular` (flags `--gravity`, `--slope`, `--scene`, `--mode`,
  `--twist-linear`, `--twist-angular`) and `events`; the generated
  `control.lift` input schema has exactly one of `fit`, `calibrate` (with `runs`) and `report`; each extension is
  additive (the schema's major version stays 1).
- `task_clusters_and_examples`: every row sits in a task cluster of at most seven tools or is a base tool; each
  example parses against the generated CLI and its command path is the row's own.
- `kind_behaviour`: a `run` row has the six common properties of its kind (`sync_wait_ms`, `idempotency_key`
  with `x-frob-key: "required"` and absent from the schema's `required` array, `budget`, `estimate_only`,
  `on_cancel` with the values `leave` and `cancel_job`, `envelope_scope`) and refuses `sync_wait_ms = 30001`
  with code 802; a verb whose capability is absent returns code 724, exit 5, one envelope.

### End-to-end script and logging

`scripts/e2e/verbs_control.sh` (header: `bead=VERBS-control milestone=M1 labels=tether,verbs,verb:control.*
gates=GATE-TETHER-02 seed=1 needs=cpu,mcp,frobd plants=alias,missing-row,renamed-field`) sources the shared helper
and `scripts/e2e/verbs_lib.sh` and calls `verbs_cluster_conformance control`. Steps: (1) `frob capabilities --json`
and `frob robot-docs --json` list the seven verbs with kind and milestone; (2) `frob-surface-diff parity --cluster
control` over stdio and through `frobd --mcp`, every difference list empty; (3) `frob-surface-diff snapshot --check
--cluster control`; (4) each verb's registry example verbatim, expecting a result (exit 0 or 2), a job handle (exit
8, then `frob job wait`) or `FROB-REF-724` exactly when the capability report says the verb is absent; (5) for each
implemented verb the same call as an MCP tool, the two envelopes agreeing on `verb`, `ok`, `exit`, `result`,
`refusal`, `evidence`, `bound`, `warnings` and `next_actions`; (6) per verb one call with a misspelt parameter
(`e2e_expect_refusal FROB-REF-802 3`) and, for `run` and `mutate` verbs, one without a key (`e2e_expect_refusal
FROB-REF-805 3`); (7) one envelope per call and no exit code 1. Examples that name a file (`regime.toml` of
`control.lift`) run in `fixtures/verbs/control/`, which holds those files.

Log: `target/e2e-logs/verbs_control/<utc>.ndjson` with a transcript beside it. Each `cmd` line has the exact
command, exit code, wall time and the envelope's `evidence` block; each `assert` has observed and expected values
and the tag `--gate GATE-TETHER-02`; the metrics `registry_verbs`, `cli_verbs` and `mcp_tools` are logged against 7
and the difference counts against 0; refusals are logged with code and first repair; a failure logs the first
differing verb and field; the `summary` line gives assertions passed and failed and the gate touched.

### Known answers and negative controls

- Known answers: the cluster has 7 verbs (4 `read`, 3 `run`); the tables above. KA-755 (the exit-code table on
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

- Gaits, skills and shields are separate clusters (`gait`, `skill`, `shield`) although the same chapter owns them; a
  verb is never registered in two clusters.
- Closing rule: the bead closes at M1, when the rows are registered, the surfaces match and every verb passes its
  example or answers the not-implemented refusal. Verbs of this cluster first answer at M1, M3, M4, M6. The
  per-milestone surface beads (BD-137, BD-312, BD-376) re-run this script as verbs land; that is how GATE-TETHER-02
  is re-measured.
- Scope names are proposals (`<cluster>:<read|run|write|admin>` plus the named scopes of the surface) until the
  capability bead BD-258 fixes the scope registry; a `run` or `write` scope of a cluster implies its `read` scope.
  Inputs and outputs above name contents, not flag spellings, except where a flag is quoted; the implementing bead
  fixes the spellings and the snapshot then pins them (additive changes keep the schema's major version; anything
  else raises it).

## References

Appendix J (J.1 entry form; J.2 inventory rows and J.3 added rows; J.5 task clusters), §22.3, §22.4, §22.7, §22.8,
§22.13, §22.20, §32.4, §32.6, §32.12.3, §32.18, §32.21; GATE-TETHER-02; INV-TETHER-01, -02; KA-755.
