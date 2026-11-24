---
key: "VERBS-sense"
title: "frob-verbs registry: cluster `sense`, eight rows, with parity, e2e and schema-snapshot checks"
type: "task"
priority: 1
milestone: "M2"
labels: ["m2", "tether", "sensorium", "frob-verbs", "solid", "registry", "verb", "test", "review-notes-open"]
parent: "WP-43"
deps: ["BD-022", "NJL-01", "NJL-02", "NJL-04", "NJL-05", "STD-E2E"]
needs: []
related: []
gates: ["GATE-TETHER-02"]
plan_refs: ["Appendix J (J.1, J.2, J.3, J.5)", "§32.4", "§20.4", "§20.11", "§20.12", "§20.13", "§20.14", "§20.15", "§20.18", "§20.24", "§32.6", "§32.12.3", "§32.21 (GATE-TETHER-02)"]
---
## Background

FrankenRobotics (a robotics simulation and design system) is reached through the command line `frob`, the daemon
`frobd` and the MCP server `frob-mcp` (the Model Context Protocol, by which an agent harness lists and calls tools).
Every capability is a verb, `cluster.verb`, declared once in `registries/verbs.toml` (crate `frob-verbs`, tier
Solid). The CLI, the daemon protocol, the MCP tools, the schemas, the refusal table and the agent guide are
generated from that file, and gate GATE-TETHER-02 compares the served surfaces with it. The registry holds 244 verbs
in 44 clusters (namespaces): the 243 of the specification and `artifact.import` (VERBS-artifact); the
count of record is NJL-03's.

This bead is the registry of record for the cluster `sense`. The cluster covers sensing: the sensor profile of a
design and task, the exact boundary schedule of the clock lattice, the causal-policy test, observability along a
route, alias witnesses, the observation-preserving symmetry group, extrinsic calibration and visibility along a
motion. It has eight verbs (3 `read`, 5 `run`); the handlers live in `frob-sense`. The bead carries every row
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
inventory); one bead for all the rows (too large to review).

## Scope

This bead is the one registry bead of the cluster. The sensing unit drafted a second one for the same eight rows
(N20b-17, now superseded by this bead); everything it held that this bead lacked is folded in below: the refusal
codes its table added, the reduced forms that answer at M1, the wait behaviour of the `run` verbs, its negative
controls, the rule on the two telemetry verbs and the two refusals that still lack a code.

In: the eight registry entries of cluster `sense`; the MCP tool names; the cluster's conformance script; the
help-schema snapshots; each verb's answer before its milestone, including the reduced forms that land early; the
sensing side of the rule for the two telemetry verbs. Out: behaviour (the implementing beads named below);
the entry schema and lints (NJL-01); the conformance harness (NJL-02); the alias lint (NJL-04); task clusters
(NJL-05, NJL-06); generation of the surfaces (the Tether seed beads).

## Technical approach

**The rows.**

| Verb | Kind | First milestone | Tier | Purpose | Defined in |
|---|---|---|---|---|---|
| `sense.alias` | run | M3 | S/F | An alias witness with its class and the priced separating changes, or "none found" | §20.12 |
| `sense.calibrate` | run | M3 | S | Extrinsic corrections with covariance and residual per edge: simulation at M3, imported logs at M5 | §20.14 |
| `sense.causal-test` | run | M2 | S | Pass, or the first decision instant and channel at which causality fails | §20.15 |
| `sense.observability` | run | M3 | S/F | The Gramian spectrum along a route, with named weak directions | §20.11 |
| `sense.profile` | read | M2 | S | The sensor suite, cards, clocks, link models and queue capacities of a design and task | §20.18 |
| `sense.schedule` | read | M2 | S | The exact boundary schedule over one repeat of the clock lattice, with lateness per event | §20.4 |
| `sense.symmetry` | read | M3 | S | Γ_obs, the elements of Γ_mech it excludes and what excludes each | §20.13 |
| `sense.visibility` | run | M3 | S | Visibility of each sensor along a nominal motion, with the robot's own limbs and payload as occluders | §20.24 |

**Contract of each verb.** Evidence labels used below (the `evidence` block of the envelope):
- *certificate*: a Proof claim of class Certified where the checker accepted a certificate, otherwise the class of
  the weakest input (the claim product takes the minimum); tier `static`, determinism `exact`; `reality: no-claim`.
- *screening*: class Estimated: a screening or planning signal that may discard under the ladder's rules and may not
  establish a comparison.
- *static*: class Tested, tier `static`, determinism `exact` (class D0); `claims` empty unless a certificate is
  attached; `reality: no-claim`.
- *pass-through*: passes through the claim product of the stored objects it reads and adds no claim of its own; the
  read itself is `exact`.
- *run*: class Tested at best, by the claim product of the run; the tier of the run, `provisional` on a fast tier
  until promoted to the referee; the determinism class of the placement (D2 unless stated); `reality: no-claim`
  (for `sense.calibrate` with `--claim reality`, the Reality claim the guard of N20b-18 admits).

Evidence words reconciled with the implementing beads (round 1, requested by U20b): `sense.causal-test` is
*run* (BD-119: the verdict is measured on run twins), `sense.calibrate` is *run* (BD-242: class Tested,
estimates from measured edges; the simulated front end is labelled `synthetic`), `sense.visibility` is *run* with `screening: true` with respect to
estimator performance (N20b-33), and `sense.profile` stays *pass-through* (BD-099, N20b-16). The earlier words
(*static*, *screening*, *static*) are replaced; *run* is the label the run-based registries (sim, eval, task) already
define, so no new word is introduced.

**Parameters that accept a path to a file** (NJL-01's `params` rule appends `FROB-REF-734` and `FROB-REF-828` to
each, and 891 for an artifact kind or 893 for `foreign-file`; N34-24 fixes the kind). Each is declared in the row's
`params` field. DESIGN where the implementing bead named no kind: a project document with a registered format is
`artifact-input<F>` of that format (N34-24's kind for "a design to check, a task to run"); one without a registered
schema is `foreign-file` until its owner registers the format (the rule VERBS-scene uses).

| Row | Parameter | Kind |
|---|---|---|
| `sense.alias` | the path forms of `<design>`, `--mission` and the card of `--card` (N25e-36 passes `.fbot`-format, `.fmission` and `.fcard` files) | `artifact-input<fbot>`; `artifact-input<fmission>` (as NJLx-51 declares it for `analyze.constraint-graph`); `artifact-input<fcard>` |
| `sense.calibrate` | `edges` | `artifact-input<frob.sense.calib.edges/1>` |
| `sense.causal-test` | `actor_schema`; `twin_fixtures` | `foreign-file` each (no registered format) |
| `sense.visibility` | `trajectory` (a run identifier or a `.frun` path, N20b-33) | `artifact-input<run>` (as VERBS-task declares `task.verdict`'s run) |

The design argument given as a name or an identity rather than a path is the same parameter: N34-24's rule "one
kind per parameter" declares a parameter that accepts a name or an identity as well as a path
`artifact-input<F>`, never two kinds and never `artifact-ref<F>` (which is for a parameter that cannot take a
path), and the forms are told apart by the value's text. The `artifact-input<fbot>` declaration above therefore
covers the name and identity forms of `<design>`, and no second declaration exists.

**`sense.alias`** (MCP tool `sense_alias`; owner `frob-sense`)
- Inputs (BD-134, N20a-41 and the lane beads fix the spellings): `<design>` (positional; omitted for a mission on a
  declared abstraction, such as BD-134's late-packet docking and N20a-37's abstract T-junction); exactly one of
  `--task <task>` and `--mission <mission>` (`FROB-REF-802` for both); `--worlds <w1>,<w2>` (BD-134's
  supplied-worlds form: a declared pair sent to the checker); `--source <lane>` (repeatable: `declared`,
  `structural` (N20a-38), `symmetry` (N20a-39), `enumerated`, `stochastic` (N20a-40), `searched` (BD-240, the
  Frontier lane); without it, the default lane set of N20a-41 item 2; after KILL-ALIAS fires, N20a-43, the searched
  lane is withdrawn and `--source searched` answers `FROB-REF-724`, exit 5, as a withdrawn Frontier lane does);
  `--card <sensor>=<card>` (repeatable: binds a card to a sensor, a card name or a `.fcard` file; N25e-34's card
  whose `error_dependence` governs the probe's reads is passed this way; DESIGN: BD-134 writes `<sensor>=<card>`
  and N25e-34 a bare card, and one grammar serves both); `--reads <n>` (the number of reads that fit before
  commitment, N20a-40 and N25e-34); `--deadline-reads <n>` (the sequential read policy's deadline, N20a-40);
  `--claim-redundancy` (N25e-34: asks for a guaranteed value above the single-read accuracy); idempotency key;
  budget. The fixture shorthands that BD-134, N13-36, N20a-37 and N20a-48 pass (`--link late|on-time`,
  `--link-latency`, `--encoder single-turn|multi-turn`, `--battery-m`) are not options of this row: they select a
  mission variant or a card, which `--mission` and `--card` express (BD-134 itself calls `--encoder` "the
  fixture's shorthand for the encoder card's `kind`"), and those beads call the registered forms (`--link late`
  is `FROB-REF-802`, BD-134).
- Outputs: an alias witness (two worlds with one observation history and different required actions) with its
  source, class, probe scope and bound, and the separating changes, each a typed edit or mission amendment
  `{kind, edit, verified: true, price | unpriced}` (N20a-41 item 4); or `none found` with per-lane coverage, or
  `none exists` only where a structural or enumerated argument supports it, with its scope and the separating
  policy; with `--card`/`--reads`, the block `redundancy { law, single_read, guaranteed, decoder, histories,
  law_marginals }` (N25e-34). Evidence: *certificate*.
- Chapter refusals: none of its own ("none found" is an answer; 445 is raised by the admitting verbs). Listed
  because they reach the caller (N20a-41 item 1, round 3): `FROB-REF-440` (the clock lattice is not admissible) and
  `FROB-REF-441` (history buffers cannot cover the declared latency and exposure window), raised by the worlds the
  checker replays, as on `sense.observability` and `sense.causal-test`; `FROB-REF-846` (marginal rates combined
  without a dependence assumption) with `detail: joint-error-law-missing`, raised under `--claim-redundancy` on a
  card whose law is `pairwise` or `unknown`, through the constant of `frob_core::refusal::augur_late`, until the
  ledger assigns the row N25e-45 specifies (the row then lists the assigned number in place of 846).
- Implemented by: BD-134 (M1 form), BD-240 (the search, M3), N20a-41 (the verb at M3). Extended by: N20a-37 (the
  witness object and its checker), N20a-38, N20a-39, N20a-40 (lanes), N20a-43 (the searched lane's kill rule),
  N25e-34 (`--card` error laws, `--reads`, `--claim-redundancy`, the `redundancy` block).

**`sense.calibrate`** (MCP tool `sense_calibrate`; owner `frob-sense`)
- Inputs: design; a source of edges, one of `--episode <id>` (a simulated episode; simulation at M3) or `--edges
  <file>` (`frob.sense.calib.edges/1`, explicit edges, which is what imported logs supply at M5); `--window
  <t0>..<t1>` (with `--episode` only: the measurements whose sample instants lie in [t0, t1)); `--anchor <node>`
  (default the base); `--reference <calibration>` (the bump test per node); `--localize` (bad-edge and bump
  attribution); `--retraction exp|cay`; `--claim reality` (a Reality claim, routed through the guard of N20b-18); idempotency key; budget.
- Outputs: extrinsic corrections with covariance, the consistency residual per edge, a fault attribution where the
  graph supports one. Evidence: *run*.
- Chapter refusals: `FROB-REF-446` (a calibration request exceeds what the graph supports: no anchor, or fault
  localization on a graph without the needed cycles); `FROB-REF-447` (a Reality claim rests on a sensor or optical
  card field of class `assumed`: raised when the caller asks for a Reality claim on a calibration whose edge
  weights or card fields are `assumed`).
- Implemented by: BD-242.
- Extended by: N20b-08 (imported logs).

**`sense.causal-test`** (MCP tool `sense_causal_test`; owner `frob-sense`)
- Inputs: design (positional); `--task`; the actor, `--controller <id>` or `--policy <id>` (default the design's
  baseline controller for the task); optional `--actor-schema <path>` (an actor schema frozen in place of the
  actor's own); the twin set, `--twins all` (default) or a list of kinds, or `--twin-fixtures <path>`; seeds
  (`--seeds <n>`, as N23a-03 calls it);
  `--tier`; `--min-compared`; `--no-record` (diagnostic: physics and delivered-observation digests only, no
  verdict and no receipt); `--control <leak id>` (accepted only by a build with the `conformance-leaks` feature;
  on any other build it is an unknown parameter, `FROB-REF-802`); idempotency key.
- Outputs: `pass`, or the first decision instant and channel at which causality fails. Evidence: *run*.
- Chapter refusals: `FROB-REF-442` (an observation is requested from presentation state, or an actor schema names a
  privileged channel); and, for the worlds it runs, `FROB-REF-440` (the clock lattice is not admissible) and
  `FROB-REF-441` (history buffers cannot cover the declared latency and exposure window), as on
  `sense.observability`.
- Implemented by: BD-119.

**`sense.observability`** (MCP tool `sense_observability`; owner `frob-sense`)
- Inputs (N20a-34, which fixes the spellings): design; `--task` (which fixes sensors, clocks and links); `--scene`
  and `--route` (or a recorded run to analyse in place); `--window <N>` (the window length); `--spacing <metres>`
  (the station spacing along the route, default 0.5 m); `--form linearized|empirical|both` (the reduced form of
  M1 is linearized on the referee); `--tier`; scaling and gauge overrides (the gauge defaults to the one the
  estimator declares for the suite); `--lambda-warn <value>` (the threshold λ_warn below which a stretch is
  reported weak; default proposed from the estimator's acceptable drift over the window and recorded with each
  result); a non-positive `--spacing` or `--lambda-warn` is `FROB-REF-802`; idempotency key.
- Outputs: the Gramian spectrum along the route modulo gauge, with named weak directions; per station the null
  dimension, λ_min and the weak direction; stretches below λ_warn; with `--form both` the per-station
  disagreement and a `forms-disagree` flag above 5%. Evidence: *screening*.
- Chapter refusals: none of its own; it lists the two admission refusals of the worlds it runs, `FROB-REF-440` (the
  clock lattice is not admissible) and `FROB-REF-441` (history buffers cannot cover the declared latency and
  exposure window), because a verb that raises an unlisted code fails the conformance test.
- Placement (N20a-36, Frontier, M4, built only if its pilot N20a-35 returned `build`): `--recommend-mount <sensor>`
  scores candidate mounts of that sensor by the Gramian (J_worst by default, J_mean reported) over the declared
  admissible set and returns the best mount as a typed edit `move-sensor` with predicted J_worst before and after,
  the stretch it repairs, side effects, and the label `screening: recommendation, not a comparison`. Before N20a-36
  lands, when its pilot did not fund it, or after its kill rule reads `withdrawn`, the option answers
  `FROB-REF-724`, exit 5, and the verb remains a report.
- Implemented by: BD-120 (the linearized form on the CPU), BD-239 (the empirical form on every tier), N20a-34.
  Extended by: N20a-36 (`--recommend-mount`).

**`sense.profile`** (MCP tool `sense_profile`; owner `frob-sense`)
- Inputs (N20b-16): design (positional); optional `--task`; optional `--scene <id>` (optical context); optional
  `--tiers <list>` (the tiers on which section 7's force-history grades are reported; a tier not in the list is
  omitted, a tier not built is `not available`).
- Outputs: the sensor suite, cards with provenance, clocks, link models and queue capacities (N20b-16's sections,
  additive over BD-099's version 0); from M6 the block `frontier_models`: per sensor the Frontier models it
  requests, each with `status` (from `registries/frontier_sensors.toml`), `pilot`, `fallback` and, where the
  fallback is in force, its finding (N20ap3-01; an empty list when no model is requested; `not due` while no task
  is registered). Evidence: *pass-through*.
- Chapter refusals: `FROB-REF-126` (a sensor without a parent frame, a rate or a clock); `FROB-REF-440` (the clock
  lattice is not admissible: a period is zero or negative, or the common denominator exceeds the limit);
  `FROB-REF-441` (history buffers cannot cover the declared latency and exposure window; raised by the packet
  queues at admission and surfaced by this verb).
- Implemented by: BD-099 (version 0: proprioception on the referee), N20b-16 (the full interface). Extended by:
  N20ap3-01 (`frontier_models`, M6).

**`sense.schedule`** (MCP tool `sense_schedule`; owner `frob-sense`)
- Inputs (BD-078 item 9): design (positional) and `--task`, or in their place an explicit clock list `--clocks
  <id>=<rate>,…` (as GB-23 and N20a-44 call it, for example `physics=400Hz,imu=100Hz,policy=30Hz`; with a design,
  `FROB-REF-802`); `--tier <tier or profile>` (fixes each clock's intra-step policy); an optional window `--window
  <t0>..<t1>` (BD-078: rational seconds, half-open [t0, t1), the form `sense.calibrate` already uses; default one
  repeat, [max phase, max phase + repeat); t1 ≤ t0 is `FROB-REF-802`). Paginated (`paginated = true`): `events[]` is
  returned with a cursor, so the
  row gains `cursor`, `limit` and `follow` and the appended codes 806 and 807 (NJL-01).
- Outputs: `q`, `step_quanta`, `repeat_quanta`, per clock `{id, kind, period_q, phase_q, rate_error, policy,
  long_run_rate}` (rates as exact rationals), and `events[]` with lateness per event, over one repeat of the clock
  lattice, in the total order (realized time, stage, clock identifier, k). Evidence: *static*.
- Chapter refusals: `FROB-REF-440` (the clock lattice is not admissible: a period is zero or negative, or the common
  denominator exceeds the limit).
- Implemented by: BD-078.

**`sense.symmetry`** (MCP tool `sense_symmetry`; owner `frob-sense`)
- Inputs: design (positional); optional `--task`, `--scene-family`, `--scan-timing per-beam|instantaneous`;
  `--action test` (the equivariance test, BD-241: it runs worlds, so it is an action of kind `run`, capability
  `sense:run`, carried as `actions = [{ name = "test", kind = "run", capabilities = ["sense:run"] }]`
  (N32a-02's two-kind form, served by NJL-01's `action` property, selected only with `--action test` by NJL-01's
  action rule 1; there is no `--test` flag, and `--test` is `FROB-REF-802`): it takes an
  idempotency key and the budget, accepts `--estimate`, waits boundedly and otherwise returns a job handle with
  exit 8, and appends the `run` codes; without a key it is `FROB-REF-805`; the default action stays `read`); `--license-check
  <use> --element <id>` (a dry run of the consumer guard of N20b-02 for one use, `tie`, `augment`, `dedup` or
  `quotient`, and one group element: `licensed` or 443 exactly as the guard would answer).
- Outputs: Γ_obs, the elements of Γ_mech it excludes, and the sensor or task that excludes each. Evidence: *static*
  (class Certified or Tested by route for the group computation; with `--action test`, *run*).
- Chapter refusals: `FROB-REF-443` (a symmetry consumer names a group element outside Γ_obs; raised by the
  consumers of the stored subgroup, and by this verb under `--license-check`); `FROB-REF-214` (a symmetry group
  outside the supported families, or a representation with a constituent of quaternionic type, surfaced when the
  mechanical group of the design is one).
- Implemented by: BD-241.

**`sense.visibility`** (MCP tool `sense_visibility`; owner `frob-sense`)
- Inputs (N20b-33, which fixes the spellings): design (positional); the nominal motion, exactly one of three
  forms: `--motion <id>` (a committed motion record that binds scene, route and gait; `--scene` overrides its
  scene), the explicit form `--scene <s> --route <r> --gait <g>`, or `--trajectory <run>` (a recorded nominal
  trajectory: a run identifier or a `.frun` path, as `task.verdict` takes its run; the run's base-pose and joint
  streams recorded at telemetry level T2 or above give the pose trajectory, its contact events the planned contact
  sites and touch instants, and its scene is the motion's scene; a run without those streams is `FROB-REF-802`
  with pointer `trajectory`, and two motion forms together are `FROB-REF-802`); optional `--payload <id>|none`
  (default the payload the design declares); `--tier <tier>` (as N20b-34 calls it); the optional target filter
  `--target-filter <kind>,…` (kinds `foothold`, `wheel-path`, `threshold`, `step`, `hazard`; default all) and the
  age bound of the blind dwell `--age-bound <s>` (seconds, a rational; default the task's declared bound), both
  spellings confirmed by N20b-33; idempotency key; budget.
- Outputs: per target the lead, blind interval, required lead, margins and the visibility timeline with the cause
  of each gap; the route profile; per sensor its sole-visibility fraction and the fraction of rays intercepted by
  the robot's own links and payload; findings with ranked repairs. Evidence: *run* with `screening: true`.
- Chapter refusals: none of its own; it lists `FROB-REF-441` for the worlds it runs. The chapter also states a
  refusal without a code, raised at stage 7 of `design check`, which calls this computation: a target whose lead
  is shorter than the declared latency chain (reserved registry slot `sens-lead-too-short`). It is raised by
  `design.check` at stage 7, not by this verb; the number is assigned through the allocation ledger by the
  decision bead N20b-45, after which N20-REF names it (until then it is raised under its slot name, exit 3).
- Implemented by: N20b-33.

**Machine fields of each row.**

| Verb | Capabilities | Output schema | Task clusters | Example |
|---|---|---|---|---|
| `sense.alias` | `sense:run` | `frob.sense.alias.out/1` | `sensing` | `frob sense alias quad-a --task dock-3tick --key al-1 --json` |
| `sense.calibrate` | `sense:run` | `frob.sense.calibrate.out/1` | `sensing`, `calibrate` | `frob sense calibrate rig-a --episode ep:4c1 --anchor base --localize --key sca-1 --json` |
| `sense.causal-test` | `sense:run` | `frob.sense.causal-test.out/1` | `sensing` | `frob sense causal-test quad-a --task stairs-0.4 --key ct-1 --json` |
| `sense.observability` | `sense:run` | `frob.sense.observability.out/1` | `sensing` | `frob sense observability quad-a --task aisle-delivery --route r3 --key ob-1 --json` |
| `sense.profile` | `sense:read` | `frob.sense.profile.out/1` | `sensing` | `frob sense profile quad-a --task stairs-0.4 --json` |
| `sense.schedule` | `sense:read` | `frob.sense.schedule.out/1` | `sensing` | `frob sense schedule quad-a --task stairs-0.4 --json` |
| `sense.symmetry` | `sense:read` | `frob.sense.symmetry.out/1` | `sensing`, `symmetry` | `frob sense symmetry quad-a --json` |
| `sense.visibility` | `sense:run` | `frob.sense.visibility.out/1` | `audit` | `frob sense visibility quad-tray --motion trot-forward --key vis-1 --json` |

The remaining fields follow by rule: `input_schema` is `frob.<verb name>.in/1`; `refusals` is the chapter list of
the verb's block; `idempotent` is `true` for `read` and `admin` and `false` for `run` and `mutate` (which take an
idempotency key); `cost_model` is `static: …` for `read`, `mutate` and `admin` and the `--estimate` cost model for
`run`; a `run` row carries `wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }`; `owner` is the
crate of the block; `purpose` is the Purpose column; `params` lists the parameters of the paths table with their
kinds; `sense.schedule` sets `paginated = true` (BD-078 returns its events with a cursor).

One entry in full (the others follow field for field):

```toml
[[verb]]
name = "sense.alias"
cluster = "sense"
kind = "run"
tier = "S"
capabilities = ["sense:run"]
input_schema = "frob.sense.alias.in/1"
output_schema = "frob.sense.alias.out/1"
refusals = ["FROB-REF-440", "FROB-REF-441", "FROB-REF-846"]
params = { design = "artifact-input<fbot>", mission = "artifact-input<fmission>", card = "artifact-input<fcard>" }
idempotent = false
cost_model = "estimated: `--estimate` returns CPU core-seconds, GPU seconds, memory and disk with a conformal interval"
first_milestone = "M3"
task_clusters = ["sensing"]
examples = ["frob sense alias quad-a --task dock-3tick --key al-1 --json"]
owner = "frob-sense"
purpose = "An alias witness with its class and the priced separating changes, or \"none found\""
wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }
```

**Conventions the rows follow.**

1. The MCP tool name is the verb name with `.` and `-` replaced by `_` (`sense.causal-test` →
   `sense_causal_test`); no registry name contains an underscore, so the map is one-to-one. Every refusal in a
   verb's list is one of the registry's codes, and a verb that raises a code its list lacks fails the conformance
   test; all `4xx` codes exit 3. The output on the wire is one envelope
   `frob.envelope/1` with `result` specialized to the output schema; its `evidence` block is a claim product (the
   weakest class, tier and determinism class among the inputs).
2. The generator adds the common input properties of the kind (`run`: `sync_wait_ms` from 0 to 30,000 with MCP
   default 5,000, `idempotency_key`, `budget`, `estimate_only`, `on_cancel` (`leave`, the default, or
   `cancel_job`) and `envelope_scope` (`--envelope-scope <gate:…|campaign:…|pretrain:…>`: the approved
   compute-envelope scope the job cites; N36-20 takes a job's scope from a launcher's token, then from this
   input, then from the verb's own inputs, and otherwise `adhoc`, whose class in force is E0, and a citation that
   contradicts the launcher's token is refused); `mutate`: `idempotency_key`, `expected_parent`).
   `idempotency_key` is in no JSON-schema `required` array: the annotation `x-frob-key` (`"required"`, or
   `"optional"` on a `key_optional` entry) carries the rule, and the admission transaction (N32a-09, from M1)
   refuses a missing key with 805 on both surfaces. The generator appends the surface-wide refusals:
   `FROB-REF-720` (scope missing), 723 (token invalid), 724 (not implemented in this build) and 802 (schema
   failure, including an unknown parameter) for every verb; 701 to 704 (budget and admission), 761 (above the
   compute envelope in force), 803 (key reused with different content) and 805 (missing key) for `run`; 725, 803,
   804 (stale parent) and 805 for `mutate`. What a field or a registry table adds follows NJL-01's variation
   table, which is the one list of what the generator adds: 806 and 807 for `paginated = true`; 808 for an entry
   with an `int53` input at any depth, whatever its kind (every `run` entry has one, `sync_wait_ms`); a
   `key_optional` entry is not given 805; a non-`run` entry with `estimable = true` gets `estimate_only`; 790 for
   a non-empty `needs_groups`; 891 to 893 for artifact `params`; 734 and 828 for a parameter that accepts a path
   to a file (`artifact-input<F>`, `artifact-identity<F>`, `foreign-file`). Its later variations, 10 to 16
   (`override` with `override_reason`, `from_proposal`, `machine`, `engine`, `no_memo`, the reality guard's seven
   codes, and `diagnostic_override` with the admission checks' codes), are read from that table and not copied
   here.
3. Exit codes: 0 done; 2 done with findings; 3 refused input (1xx to 5xx, and 8xx other than conflicts); 4 budget
   (70x and 761); 5 capability (72x, 73x); 6 device (6xx); 7 conflict (803, 804, 809); 8 accepted and still running; 9
   indeterminate; never 1 on purpose.
4. `first_milestone` is the value the verb appendix prints, for every row (the rule the M1 registration bead
   BD-137 and the registry beads share): the registry's count known answers are pinned to the appendix, and a
   field that moved when a seed bead built a reduced form early would make them depend on delivery order. What a
   build serves is decided by the handler: a verb whose computation has not landed answers one envelope with
   `FROB-REF-724`, exit 5, on both surfaces, and `frob capabilities --json` lists it as absent, whatever its
   `first_milestone`; a reduced form that lands before the row's milestone answers from then on and is listed as
   implemented with its restriction named.

**Reduced forms that answer before the row's milestone.** Five verbs of this cluster are built in a reduced form at
M1, because the first mission slice needs them. Their rows are entered at M1 by BD-137 with the appendix's
milestone and are completed here; an earlier draft that wrote `first_milestone = "M1"` with a coverage note for
these five is withdrawn in favour of the shared rule.

| Verb | Row's milestone | Answers from | Restriction named by `frob capabilities` | Complete at |
|---|---|---|---|---|
| `sense.profile` | M2 | M1 (BD-099) | `form: "referee-proprioception"` | M2 (N20b-16) |
| `sense.schedule` | M2 | M1 (BD-078) | `form: "cpu"` | M2, every tier |
| `sense.causal-test` | M2 | M1 (BD-119) | `form: "referee"` | M2, every tier (N20b-12) |
| `sense.observability` | M3 | M1 (BD-120) | `form: "linearized-cpu"` | M3, empirical (BD-239) |
| `sense.alias` | M3 | M1 (BD-134) | `form: "enumerated-worlds"` | M3, search (BD-240) |
| `sense.symmetry` | M3 | M3 (BD-241) | none | M3 |
| `sense.calibrate` | M3 | M3 in simulation (BD-242) | `form: "simulation"` | M5, imported logs (N20b-08) |
| `sense.visibility` | M3 | M3 (N20b-33) | none | M3 |

**Wait behaviour of the five `run` verbs.** A `run` verb waits to completion on the command line; on MCP it waits
5,000 ms by default and at most 30,000 ms; when the wait elapses it returns a job handle with exit code 8, and
the envelope obtained on completion is the same as the one a waiting call returns.

**Notes for this cluster.**

- Eight verbs. `telemetry.stills` and `telemetry.inspect-pixel` (read, M2, implemented by BD-191: contact sheets
  drawn with the ray kernels plus an identifier buffer; a pixel mapped to a link, fixture, region or contact) are
  specified with sensing but belong to the `telemetry` cluster, whose registry bead carries their rows.
  `telemetry.export-html` is the chronicle chapter's, and the viewer it packages the surface chapter's. The
  sensing rule those rows must respect is stated here: a still is not an observation, so neither verb is a
  source for the observation service, and a request to use one as an observation is `FROB-REF-442` at the
  observation service, never a result of these verbs.
- `sense.visibility` was added by a late subsection and is absent from the chapter's original verb table; the
  cluster test asserts that it is present.
- `sense.alias` returns a witness or "none found", never "none exists" unless a structural or an enumerated argument
  supports it; it raises no refusal of its own for "no witness". A standing alias witness for a mission's interface
  makes training and optimization on it refuse (`FROB-REF-445`), and that code is raised by those verbs.
- Three late refusals of the chapter get their numbers through the decision bead N20b-45 (the allocation ledger's
  second round): a peak-load claim without a resolved or modelled force history (slot `sens-peak-load-ungraded`,
  raised by N20b-29), the stage-7 lead refusal (slot `sens-lead-too-short`, raised by `design.check` through
  N20b-33's computation) and the evaluation of a policy with no passing causal-policy receipt (slot
  `causal-receipt-missing`). A row lists such a code once N20b-45 has applied the number; until then the refusal
  is raised under its slot name with exit 3 and is on no row's list.
- `sense.calibrate` first answers at M3 in simulation and at M5 on imported logs.

## Acceptance criteria

- [ ] `registries/verbs.toml` holds exactly eight entries with `cluster = "sense"` (3 `read`, 5 `run`), each with
      every field of the entry schema, an example and a task-cluster list; the alias lint finds no second spelling.
- [ ] `frob-surface-diff parity --cluster sense` exits 0 in process (`frob-mcp` over stdio) and through `frobd
      --mcp`: registry, CLI and MCP each count 8, and every difference list is empty apart from the harness's closed
      exemptions.
- [ ] Every verb has an end-to-end script: `scripts/e2e/verbs_sense.sh` runs each verb of the cluster (eight), and
      from a verb's first milestone a capability script of its implementing bead exists (`frob-surface-diff
      e2e-coverage`).
- [ ] `frob <verb> --help --json` equals `fixtures/golden/verbs/sense/<verb>.help.json` byte for byte for each of
      the eight, on two runs, debug and release, Linux and macOS; a planted renamed output property is classified
      `breaking` and refused.
- [ ] A verb not yet implemented answers `FROB-REF-724`, exit 5, with one envelope on both surfaces; an implemented
      verb runs its registry example with the result its implementing bead specifies; a misspelt parameter is
      `FROB-REF-802`, exit 3; a `run` or `mutate` call without a key is `FROB-REF-805`, exit 3.
- [ ] The generated refusal table lists for each verb exactly its chapter codes plus the codes appended for its
      kind and fields (NJL-01), and every code exists in `registries/refusals.toml`.
- [ ] Each `run` verb returns exit 8 with a job handle when its wait elapses, and the same envelope on completion;
      each verb's listed refusals exit 3.
- [ ] `frob schema sense.<verb>` prints each of the eight schemas and `frob robot-docs` lists the cluster; the
      served MCP tool list contains `sense_profile`, `sense_schedule`, `sense_causal_test`, `sense_observability`,
      `sense_alias`, `sense_symmetry`, `sense_calibrate` and `sense_visibility` once each answers.
- [ ] The five reduced forms answer at M1 with their restriction named by `frob capabilities --json`, and every
      row's `first_milestone` equals the verb appendix's value.
- [ ] `sense.symmetry` carries the action `test` of kind `run` (capability `sense:run`), selected only with
      `--action test`: `--action test` without a key is `FROB-REF-805`, with a key it completes or returns a job
      handle (exit 8) on MCP; `--test` is `FROB-REF-802`; its default action is `read`.
      Each row's inputs and evidence label equal its block, which equals its implementing bead's.
- [ ] Removing a refusal from a row's list, or adding an underscore to a verb name, fails the cluster test
      (negative controls).
- [ ] The gate row is recorded with the measured counts by a session that did not write the rows.

## Test plan

### Unit tests

`cargo test -p frob-verbs --test cluster_sense` (the real generated table; no mocks; planted registries are TOML
strings labelled `synthetic`):
- `rows_present`: exactly 8 rows in the cluster, with the names of the rows table; a planted extra row or a removed
  row fails.
- `kinds_milestones_tiers`: each row's kind, first milestone and tier equal the rows table (for example
  `sense.visibility` is `run` at M3).
- `schemas_and_scopes`: input schemas are `frob.<name>.in/1`; output schemas and capabilities equal the
  machine-fields table.
- `refusal_lists`: each row's chapter codes equal its block (`sense.alias`: 440, 441, 846; `sense.calibrate`: 446, 447;
  `sense.causal-test`: 440, 441, 442; `sense.observability`: 440, 441; `sense.profile`: 126, 440, 441; `sense.schedule`:
  440; `sense.symmetry`: 443, 214; `sense.visibility`: 441); each is a subset of the registered codes; the
  appended codes equal the rule for its kind and fields (NJL-01).
- `schemas_exist_and_examples_validate`: each schema file exists and validates the row's examples.
- `run_verbs_have_wait_defaults`: the five `run` rows carry `wait = { cli = "to-completion", mcp_ms = 5000,
  mcp_max_ms = 30000 }`.
- `visibility_row_present`: `sense.visibility` is in the cluster although the chapter's first table omits it.
- `early_forms_keep_appendix_milestone`: the five rows served at M1 have `first_milestone` M2 or M3 as printed, and
  a capability report built at M1 names their `form`.
- `stills_are_not_observations`: the observation service refuses a source of kind still or presentation state with
  code 442; the rows of `telemetry.stills` and `telemetry.inspect-pixel` are not in this cluster.
- `control_unlisted_refusal_fails`, `control_underscore_name_fails`: a planted row whose handler raises a code
  its list lacks, and a planted name with an underscore, each fail.
- `inputs_by_row`: `sense.calibrate` accepts exactly one of `--episode` and `--edges` and refuses `--window` with
  `--edges` (802); `sense.causal-test` refuses `--control` on a build without `conformance-leaks` (802);
  `sense.symmetry --action test` without a key is 805 and its appended codes are the `run` set (the default
  action's are the `read` set), while `sense.symmetry quad-a --test` is 802 (no per-action flag);
  `sense.visibility` accepts the `--motion` form, the explicit `--scene --route --gait` form and the
  `--trajectory` form, refuses `--motion` with `--trajectory` (802), and accepts `--target-filter foothold` and
  `--age-bound`; `sense.observability` accepts `--spacing 0.25` and `--lambda-warn` and refuses a non-positive
  value of either (802); `sense.alias` refuses `--task` with `--mission`
  (802), accepts `--card
  probe=<card>` and refuses `--link` (802: not an option of the row); `sense.schedule` refuses `--clocks` with a
  design (802) and is paginated (cursor, limit, follow; 806 and 807 appended); `sense.observability
  --recommend-mount <sensor>` is 724 in a build without N20a-36's mechanism; `sense.profile` accepts `--scene` and
  `--tiers`.
- `path_params`: each parameter of the paths table is declared in `params` with its kind, and its row's full list
  holds 734, 828 and 891 (or 893 for `foreign-file`), once each.
- `evidence_words`: the evidence label of each row equals the reconciled table (causal-test, calibrate and
  visibility *run*, visibility with `screening: true`; profile *pass-through*).
- `mcp_names_one_to_one`: the tool names are the mapped names (`sense_alias`, `sense_calibrate`,
  `sense_causal_test`, `sense_observability`, `sense_profile`, `sense_schedule`, `sense_symmetry`,
  `sense_visibility`), with no collision against any other cluster.
- `task_clusters_and_examples`: every row sits in a task cluster of at most seven tools or is a base tool; each
  example parses against the generated CLI and its command path is the row's own.
- `kind_behaviour`: a `run` row has the six common properties of convention 2 and refuses `sync_wait_ms = 30001`
  with code 802; a verb whose capability is absent returns code 724, exit 5, one envelope.

### End-to-end script and logging

`scripts/e2e/verbs_sense.sh` (header: `bead=VERBS-sense milestone=M2 labels=tether,verbs,verb:sense.*
gates=GATE-TETHER-02 seed=1 needs=cpu,mcp,frobd plants=alias,missing-row,renamed-field`) sources the shared helper
and `scripts/e2e/verbs_lib.sh` and calls `verbs_cluster_conformance sense`. Steps: (1) `frob capabilities --json`
and `frob robot-docs --json` list the eight verbs with kind and milestone; (2) `frob-surface-diff parity --cluster
sense` over stdio and through `frobd --mcp`, every difference list empty; (3) `frob-surface-diff snapshot --check
--cluster sense`; (4) each verb's registry example verbatim, expecting a result (exit 0 or 2), a job handle (exit 8,
then `frob job wait`) or `FROB-REF-724` exactly when the capability report says the verb is absent; (5) for each
implemented verb the same call as an MCP tool, the two envelopes agreeing on `verb`, `ok`, `exit`, `result`,
`refusal`, `evidence`, `bound`, `warnings` and `next_actions`; (6) per verb one call with a misspelt parameter
(`e2e_expect_refusal FROB-REF-802 3`) and, for `run` and `mutate` verbs and the `test` action of `sense.symmetry`,
one without a key (`e2e_expect_refusal FROB-REF-805 3`); (7) one envelope per call and no exit code 1; (8) each implemented `run` verb once with a zero
wait on each surface, expecting a job handle with exit 8 and, after `frob job wait`, structured content equal to
the waiting call's.

Log: `target/e2e-logs/verbs_sense/<utc>.ndjson` with a transcript beside it. Each `cmd` line has the exact command,
exit code, wall time and the envelope's `evidence` block; each `assert` has observed and expected values and the tag
`--gate GATE-TETHER-02`; the metrics `registry_verbs`, `cli_verbs` and `mcp_tools` are logged against 8 and the
difference counts against 0; refusals are logged with code and first repair; a failure logs the first differing verb
and field; the `summary` line gives assertions passed and failed and the gate touched.

### Known answers and negative controls

- Known answers: the cluster has 8 verbs (3 `read`, 5 `run`); the tables above. KA-755 (the exit-code table on
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

- Closing rule: the bead closes at M2, when the rows are registered, the surfaces match and every verb passes its
  example or answers the not-implemented refusal. The rows of this cluster carry the milestones M2 and M3; five
  reduced forms answer from M1 (the table above). The per-milestone
  surface beads (BD-222, BD-312) re-run this script as verbs land; that is how GATE-TETHER-02 is re-measured.
- Scope names follow the naming rule decided in BD-258 (no longer proposals): a row cites scopes of its own
  cluster by kind (`read` → `<cluster>:read`; `run` → `<cluster>:run` or a named spending scope; `mutate` →
  `<cluster>:write` or a named write scope; `admin` → `<cluster>:admin`, `job:cancel` or a named admin scope),
  plus `export:*` when the row exports and `private:*` where privacy applies, never a scope named after the
  object read or the engine spent on; N32b-01's catalogue generates the forms. `<cluster>:run` and
  `<cluster>:write` imply `<cluster>:read`, closed when a set is stored (at minting and at derivation,
  `Scopes::close_implied`), so checks stay plain set inclusion.
  The sensing unit's draft of these rows scoped the three reads by the object read (`design:read`) and the five
  runs by `sim:run`; BD-258 step 2 has rejected that draft by name, and the cluster rule (`sense:read`,
  `sense:run`) is decided, as in every other cluster's registry bead. The implementing beads BD-241, BD-242,
  N20a-34, N20a-41 and N20b-33 name the cluster scopes, as BD-078 (`sense:read` for `sense.schedule`) and N20b-16
  (`sense:read` for `sense.profile`) do.
- Task clusters are those of the assignment bead NJL-05 (`sensing`, with `sense.calibrate` also in `calibrate`,
  `sense.symmetry` also in `symmetry`, and `sense.visibility` in `audit`). The sensing unit's draft proposed one
  cluster `perceive` for seven of the verbs; it is not used.
- A registry bead depends only on the entry schema and the conformance harness, never on the verbs' implementing
  beads; those are listed under `related`, and the per-milestone surface beads re-run this bead's script as they
  land.
  Inputs above give the spellings the implementing beads fix (KNOWN_DRIFTS 8), each input a script calls with its
  flag; where a spelling is this registry's DESIGN, the row says so. The snapshot then pins them (additive changes
  keep the schema's major version; anything else raises it).
- Scripts that call a `run` verb of this cluster carry `--key` (KNOWN_DRIFTS 5), distinct keys where a test
  compares two calls; the `sense.alias` stages of BD-134, N13-36, N20a-37, N20a-39, N20a-40 and N20a-48 do (a call
  without one is `FROB-REF-805`).

## References

Appendix J (J.1 entry form; J.2 inventory rows and J.3 added rows; J.5 task clusters), §20.4, §20.11, §20.12,
§20.13, §20.14, §20.15, §20.18, §20.24, §32.4, §32.6, §32.12.3, §32.18, §32.21; GATE-TETHER-02; INV-TETHER-01, -02;
KA-755.
