---
key: "VERBS-analyze"
title: "frob-verbs registry: cluster `analyze`, twenty-six rows, with parity, e2e and schema-snapshot checks"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "tether", "atelier", "frob-verbs", "solid", "registry", "verb", "test"]
parent: "WP-06"
deps: ["BD-022", "NJL-01", "NJL-02", "NJL-04", "NJL-05", "STD-E2E"]
needs: []
related: []
gates: ["GATE-TETHER-02"]
plan_refs: ["Appendix J (J.1, J.2, J.3, J.5)", "§28.4", "§13.4", "§32.6", "§32.12.3", "§32.21 (GATE-TETHER-02)"]
---
## Background

FrankenRobotics (a robotics simulation and design system) is reached through the command line `frob`, the daemon
`frobd` and the MCP server `frob-mcp` (the Model Context Protocol, by which an agent harness lists and calls tools).
Every capability is a verb, `cluster.verb`, declared once in `registries/verbs.toml` (crate `frob-verbs`, tier
Solid). The CLI, the daemon protocol, the MCP tools, the schemas, the refusal table and the agent guide are
generated from that file, and gate GATE-TETHER-02 compares the served surfaces with it. The registry holds 244 verbs
in 44 clusters (namespaces): the 243 of the specification and `artifact.import` (VERBS-artifact); the
count of record is NJL-03's.

This bead is the registry of record for the cluster `analyze`. The cluster covers the static analyses A-01 to A-26
of the design studio, one read verb per analysis. The analyses answer questions about a body before any batch
simulation is spent (mobility, mass, symmetry, reach, stability, sizing, timing, energy), and two of them read
recorded runs. It has twenty-six verbs (26 `read`); the handlers live in `frob-analyze`. The bead carries every row
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

In: the twenty-six registry entries of cluster `analyze`, entered here as data of record; the MCP tool names; the
cluster's conformance script; the help-schema snapshots; each verb's answer before its milestone. Out: behaviour
(the implementing beads named below); the entry schema and lints (NJL-01); the conformance harness (NJL-02); the
alias lint (NJL-04); task clusters (NJL-05, NJL-06); generation of the surfaces (the Tether seed beads); and the
generator in `frob-analyze` that derives the same twenty-six rows from the `Analysis` registrations, with the
handler bindings, the per-verb schema files, the missing-binding and not-implemented behaviour and the memo
wrapping (N28a-05, which depends on this bead). The rows are entered once, here, at M1 and before any analysis
exists. From the moment N28a-05 lands, the `analyze` block of `registries/verbs.toml` is under
`check-generated`: it must equal the generator's output byte for byte, and a later analysis reaches the
registry by running the generator, never by a second hand entry. This bead depends on no mechanism; N28a-05
depends on it.

## Technical approach

**The rows.**

| Verb | Kind | First milestone | Tier | Purpose | Defined in |
|---|---|---|---|---|---|
| `analyze.mobility` | read | M1 | S/F | A-01: How many degrees of freedom the mechanism really has, and whether any loop is over-constrained | §28.4 |
| `analyze.mass` | read | M1 | S | A-02: Mass, centre of mass and inertia by subtree, with physical consistency | §28.4 |
| `analyze.symmetry` | read | M1 | S/F | Γ_mech and Γ_obs of a design, with the element that breaks each excluded generator (A-03) | §13.4 |
| `analyze.gait` | read | M4 | F | A-04: Which gaits the body can have, with phase lags and stability margins; whether a periodic orbit exists under the baseline controller (a screening signal) | §28.4 |
| `analyze.workspace` | read | M1 | S/F | A-05: Whether the feet or the hand reach what the tasks require (step height, shelf height) | §28.4 |
| `analyze.singularities` | read | M1 | S | A-06: Where the singular configurations are and how close a nominal gait comes | §28.4 |
| `analyze.maneuverability` | read | M1 | S/F | A-07: Growth vector, turning footprint, cost of a sideways shift, door and aisle passability | §28.4 |
| `analyze.stability` | read | M1 | S | A-08: Support margins, tip-over angles on slopes and the contact-wrench-cone margin with payload | §28.4 |
| `analyze.curvature` | read | M3 | F | A-09: Displacement per shape cycle for kinematic locomotors; reorientation capacity in flight | §28.4 |
| `analyze.closure` | read | M1 | S | A-10: Whether the mass spiral of co-design closes, and the least mass and power | §28.4 |
| `analyze.envelope` | read | M1 | S | A-11: Whether the torque-speed points of standing, crouching, stepping up and nominal gaits fit inside each actuator card, with the thermal steady state | §28.4 |
| `analyze.modes` | read | M1 | S | A-12: Natural frequencies and mode shapes at stance, block-diagonalized by symmetry; whether actuator bandwidth suffices | §28.4 |
| `analyze.self-collision` | read | M1 | S | A-13: Whether limbs sweep through each other or the ground over their ranges | §28.4 |
| `analyze.coverage` | read | M3 | S | A-14: Whether the sensor suite sees the footholds and the route ahead | §28.4 |
| `analyze.structure` | read | M4 | F | A-15: Whether a link would yield under the peak loads seen or bounded | §28.4 |
| `analyze.sensitivity` | read | M3 | S/F | A-16: Which parameters matter most for a chosen metric | §28.4 |
| `analyze.observability` | read | M1 | S/F | A-17: Whether the state the task needs can be reconstructed from the sensor suite at the declared rates and delays (a screening signal for the conditioning) | §28.4 |
| `analyze.route` | read | M3 | F | A-18: Whether the footprint family fits the routes of a bound scene: fits, cannot fit or unresolved, with the bottleneck named | §28.4 |
| `analyze.rank-robustness` | read | M4 | S/F | A-19: Whether the ranking of finalists survives a change of contact law, cone, stiffness, step or solver tolerance, and whether any finalist profited from energy the simulator created (reads runs; never joins `check`) | §28.4 |
| `analyze.energy-budget` | read | M1 | S/F | A-20: Whether the mission's mandatory work can be paid from the declared energy and power budget, approach kinetic energy included | §28.4 |
| `analyze.deadlines` | read | M1 | S | A-21: Whether the last informative observation arrives before the decision it must inform, on every admitted schedule | §28.4 |
| `analyze.loop-stability` | read | M1 | S | A-22: Whether each actuator's current and speed loop is stable at the declared control period, feedforward law and reflected inertia | §28.4 |
| `analyze.bus-schedule` | read | M1 | S | A-23: Whether all consumers on a shared electrical bus can meet their demands by their deadlines, jointly (never a sum of peaks) | §28.4 |
| `analyze.wrench` | read | M4 | S | A-24: Whether the body can produce the wrench a skill or task needs inside torque, speed, friction and support limits | §28.4 |
| `analyze.identifiability` | read | M4 | S/F | A-25: Whether a proposed experiment would determine the card or scene parameters the pending decision depends on | §28.4 |
| `analyze.regimes` | read | M4 | F | A-26: In which contact modes, slip and saturation states the design operated, where a gradient is trustworthy and where a ranking changes (reads runs; never joins `check`) | §28.4 |

**Contract of each verb.** Evidence labels used below (the `evidence` block of the envelope):
- *analysis*: tier `static`; class the claim product of the analysis and its inputs: `Tested` for an exact
  computation on exact inputs (determinism `exact`, class D0), and the weakest class of the inputs read otherwise
  (a result computed from actuator, battery or material card values is `Estimated` unless every field read has
  provenance `measured`); determinism D0 for the exact analyses and the class the analysis declares otherwise;
  `claims` empty; `reality: no-claim`. A *screening* analysis (a rung that may discard and may not establish)
  sets `result.screening = true` and class `Estimated`. A-19 and A-26, which read runs, carry the claim product
  of the runs read. (An earlier draft labelled every row Tested and exact; the analysis envelope of N28a-05 sets
  the class per analysis, and governs.)

**`analyze.mobility`** (MCP tool `analyze_mobility`; owner `frob-analyze`)
- Inputs: design; optional exact sample points.
- Outputs: mobility count, rank of the constraint Jacobian at exact rational sample points, the over-constrained
  loops, ranked repairs as typed edits; the field `lattice` (`not-run` on the default build; from M3, with the
  feature `frontier-lattice-mobility`, `decided(n)` or `undecided(…)` from the subgroup-lattice route of N13-28;
  the schema is unchanged when it fills). Evidence: *analysis*.
- Chapter refusals: `FROB-REF-110` (a closure that over-constrains the mechanism (mobility below one)).
- Implemented by: BD-101, N28a-05; extended by N13-28 (the Frontier lattice route, M3).

**`analyze.mass`** (MCP tool `analyze_mass`; owner `frob-analyze`)
- Inputs: design; optional subtree path.
- Outputs: mass, centre of mass and inertia per subtree with provenance; the smallest pseudo-inertia eigenvalue when
  inconsistent. Evidence: *analysis*.
- Chapter refusals: `FROB-REF-104` (inertia not physically consistent (the smallest eigenvalue of the pseudo-inertia
  is reported)); `FROB-REF-106` (declared mass properties violate the mass-support bound); `FROB-REF-168` (a solid
  body exceeds the mass plausibility bound of its part class and no fill is declared (§11.5)).
- Implemented by: BD-100, N28a-05.

**`analyze.symmetry`** (MCP tool `analyze_symmetry`; owner `frob-analyze`)
- Inputs: design; optional `--task`, sensor profile and scene family; `--near-rank k` (the largest rank of
  near-symmetry reported, default 2, BD-102); from M6 `--symmetry-events <run>` (N25d-02, Frontier, kill gate
  KILL-AUGUR-SYMEVENT: the symmetry-event findings taken on that run).
- Outputs (`frob.analyze.symmetry.out/1`, BD-102): `mech` (order, recognized name, generators, elements) and `obs`
  (the same, with `obs_scope`); `representations` per channel space; `breakers`, per excluded element {element,
  field path, size, repair edit with its predicted effect}; `near_symmetries` (rank ≤ k, each mismatching field
  with both values and the typed edit that equalizes it); `forced_ties` (the fixed subspace of each non-identity
  element of Γ_obs); `route` per element (by construction, by exact field arithmetic, or by tolerance);
  `continuous_stabilizer` where a body of revolution has one; the repair edits as `next_actions`. With
  `--symmetry-events`: per (group element, event) a finding {element, event, image event, status `holds`,
  `broken`, `inconclusive`, `unsupported` or `ambiguous`, residual, error estimate, method, observables,
  derivative label, class} and the transport licences issued, stored in the ledger with their replay address.
  Evidence: *analysis*; routes by construction and by exact field arithmetic D0 and `Certified` when the
  certificate is emitted, the tolerance route `Tested` with its tolerance recorded; a symmetry-event finding has
  the determinism class of the trajectory it was taken on.
- Chapter refusals: `FROB-REF-109` (declared symmetry differs from computed symmetry; raised by stage 5 of
  `design.check` through N14a-11's comparison, the verb attaching the breaker report); `FROB-REF-214` (a
  representation with a constituent of quaternionic type, or a symmetry group outside the supported families);
  `FROB-REF-215` (an exact evaluation over its bit budget: the collect-all descent and the fixed subspaces run in
  exact field arithmetic, BD-102); from M6 `FROB-REF-263` (a derivative requested at grazing, at an ambiguous
  event order, across a change of active set or at frictional degeneracy) and `FROB-REF-852` (a symmetry
  transport lacks a side condition), both N25d-02's.
- Implemented by: BD-102, N28a-05; extended by N25d-02 (`--symmetry-events`, M6, Frontier).

**`analyze.gait`** (MCP tool `analyze_gait`; owner `frob-analyze`)
- Inputs (BD-332's registration of A-04): design; `--task` (the common binding: Γ_obs, and the command class
  from the task's commanded velocity, `forward` without one); `--controller <ControllerId>` (optional; read by
  the orbit part); `--param <name>=<a>..<b>` (optional; the design parameter of the continuation part); the
  analysis verbs' `--time-budget`, which bounds the orbit part (declared default 60 s for that part, BD-333).
  The network is the one registered for the design's leg count; the verb has no `--network` and no `--timing`,
  which are `gait.atlas`'s.
- Outputs: `{ atlas, orbits, continuation }`: gaits as (H, K) pairs with phase lags; orbit existence, multipliers
  and residual; standing `screening`. A part that was not computed reads `not-run` with its reason: `orbits`
  without `--controller` is `unbound`, and its entries beyond the time budget are `budget` (status `Undecided`,
  exit 2, with a `gait.orbit` command per unsolved entry in `next_actions`); a part whose bead is not in the
  build names that bead (BD-333 for `orbits`, BD-335 for `continuation`). The schema is the same whichever
  parts are filled.
  Evidence: *analysis*, screening (the orbit solve on the compliant tier is a screening signal).
- Chapter refusals: `FROB-REF-431` (a gait derivative was requested at grazing or at a fold).
- Implemented by: BD-332, which registers A-04 as an `Analysis` of `frob-analyze` (a thin view that calls the
  atlas function of `frob-gait`; (H, K) pairs and phase lags) and fills `atlas`; BD-333 fills `orbits` (return
  maps and Floquet multipliers, the solver of `gait.orbit`); the continuation is BD-335's (`gait.continue`);
  N28a-05 generates the row from the registration.

**`analyze.workspace`** (MCP tool `analyze_workspace`; owner `frob-analyze`)
- Inputs: design; `--leg <name|all>` (the end effector); targets as points in the body frame, task-level
  heights (`--target-step-height h`, a step height at a stance width) or a task. From M4, under the Frontier
  certified inverse-kinematics mode of N10ax-01: `--chain <base>:<tip>`, `--task pose`, `--targets <file>`, `--ik
  certified|closed-form|multistart|auto` (default `auto`: the closed form where the chain is in the derived leg
  class, the certified solver where it is inside the caps, multistart otherwise), `--box …`, `--fix
  joint=value`, `--max-depth n`, `--max-boxes n`, `--emit-certs <dir>` (writes the `interval-root` bundles),
  `--require-complete`. `--targets` and `--emit-certs` take paths, so the generator's path rule applies (NJL-01).
- Outputs: per target `reachable` with the surviving branches, `unreachable` with the binding reason (inside the
  offset cylinder, beyond full extension, inside the folded radius, or every branch outside a named joint's
  range), or `unresolved` when the deciding quantity lies within its forward error bound of zero; extension,
  fold and cylinder margins in metres; typed `exact` for the leg class; ranked repairs. With `--ik`, per target
  the block `frob.analyze.ik/1` inside the result: `status`, `method`, `solutions[]` (joint values as
  intervals, the box, the certificate hash), `count` or `count_lower_bound`, `undecided[]` (box, smallest
  singular value), `boxes` (certified, empty, undecided, alias), `depth_reached`, `reason` when `unsupported`;
  running out of `--max-boxes` is `Partial`, never a refusal; exit 2 when a required target has no solution or
  the status is not `Complete` under `--require-complete`. Evidence: *analysis* (`Tested`, D0); the certified
  box set D0, its certificates re-checked by the offline auditor.
- Chapter refusals: `FROB-REF-119` (no admissible route or reach in the scene; stage 7 of `design.check` for the
  task's heights); from M4 `FROB-REF-215` (an exact evaluation over its declared 4,096-bit coefficient cap;
  repairs: shrink the box, round the geometry to a declared grid, or `--ik multistart`).
- Implemented by: BD-105, N28a-05; extended by N10ax-01 (certified inverse kinematics, M4, Frontier).

**`analyze.singularities`** (MCP tool `analyze_singularities`; owner `frob-analyze`)
- Inputs: design; `--leg` (the end effector); a pose (`--pose reference`) or a trajectory of joint coordinates
  (a nominal gait's swing and stance); a singular-value threshold (default 1% of a + b per radian, BD-105).
- Outputs: per sample det J, its two factors and the smallest singular value of J in metres per radian; over a
  trajectory the minimum of each with its sample and the responsible factor; samples flagged `singular`; repairs
  (bend the nominal knee angle, change the stance height, change the hip offset). Evidence: *analysis*.
- Chapter refusals: none of its own.
- Implemented by: BD-105, N28a-05.

**`analyze.maneuverability`** (MCP tool `analyze_maneuverability`; owner `frob-analyze`)
- Inputs: design; optional bound scene.
- Outputs: growth vector, controllability rank, turning footprint, passability against door and aisle widths.
  Evidence: *analysis*.
- Chapter refusals: `FROB-REF-118` (the base cannot produce a motion the task class requires).
- Implemented by: BD-106, N28a-05.

**`analyze.stability`** (MCP tool `analyze_stability`; owner `frob-analyze`)
- Inputs: design; `--stance`; payload; the load-sharing model (`--authority`, for example
  `passive-equal-spring`).
- Outputs: margins typed `exact`, `inner` or `outer`, with the load-sharing (authority) model named; for a leg
  with a coupled transmission the limit taken from the joint-torque generators and named as such (N15b-40);
  ranked repairs. Evidence: *analysis*.
- Chapter refusals: `FROB-REF-124` (a static margin the task requires is not met).
- Implemented by: BD-100, N28a-05; extended by N15b-40 (coupled transmissions); its stance battery is N28a-06.

**`analyze.curvature`** (MCP tool `analyze_curvature`; owner `frob-analyze`)
- Inputs: design; the contact stratum, `--stratum <rolling[:<wheel>,…]|flight>` (DESIGN spelling, fixed here
  because BD-243 names the input and gives it none, so no caller could choose a stratum: `rolling` takes every
  wheel the design declares as rolling without lateral slip and is the default for a design that declares
  wheel constraints; `rolling:<wheel>,…` takes only the named wheels, the others giving no constraint row;
  `flight` is zero-momentum free flight, from the inertia blocks, and is the default for a design that declares
  no wheel constraint; the result echoes the stratum used); `--shape j1,j2[,…]` (two or more shape joints);
  `--loop <shape loop>` (BD-243: `rect:<a>,<b>`, a rectangle with sides a and b, or `circle:<ρ>`, in radians, in
  the plane of the first two shape joints, with an optional centre `@<s₁>,<s₂>` in radians, for example
  `rect:0.05,0.05@0.5,-0.5`; without a centre the loop is centred at the design's home shape, the shape joints'
  values in its home pose, which is why the fixture `snake-a` has its home pose at (0.5, −0.5) rad; the loop is
  traversed from the first shape joint towards the second).
- Outputs (BD-243): the curvature component fields D𝔸 on a grid over the joint ranges (default 33 × 33 per pair)
  with their extrema; per named loop the displacement (x, y, yaw for planar systems) by the area formula,
  labelled `leading-order`, and by the ordered product along the loop, with their difference as the error
  indicator; the stratum; the minimum-perturbation coordinate change β; the note `kinematic only`. From M6
  (N25c-43, Frontier, alive while KILL-AUGUR-PHASE keeps the inverse holonomy query) the local connection object
  with its admission record, which `mission.synthesize` consumes. Evidence: *analysis*, class `Estimated`, D0 on
  one platform's deterministic functions; N25c-43's connections `Certified` for Pfaffian connections on exact
  kinematic fixtures and `Tested` for fitted ones.
- Chapter refusals: `FROB-REF-118` (the base cannot produce a motion the task class requires: the code of the
  finding that the required net displacement is unreachable at leading order on every admissible loop, raised by
  stage 4 of `design.check`); and the refusal `connection-outside-stratum` ("a local connection was used outside
  its qualified stratum or scope": the sixth row of the Augur refusal decision N25e-45; family `1xx`, exit 3,
  `retry: after-change`), raised by the verb itself and told apart by `detail`, of which there are five kinds:
  from M3, by BD-243 step 1, `rank-drop` (the stratum's constraint rows do not give rank W_g = dim G at a shape
  or on a loop segment) and `over-determined` (the rows over-determine the group motion for generic shapes; the
  redundant rows are named); from M6, by N25c-43, `stratum-crossing` (a loop across a slip or lift-off
  boundary), `component-removed` (a motion component the constraints remove, such as the yaw of a skid-steer
  base) and `rank-only-displacement` (a finite displacement requested on the strength of accessibility rank
  alone). Its number is the one the allocation run of N35x-02 assigns at M0, before any raising bead lands, so
  no interim code exists for it; every emit site raises it through the constant
  `frob_core::refusal::augur_late::CONNECTION_OUTSIDE_STRATUM`, and this row lists the number the ledger
  (`registries/refusal_allocations.toml`, request `connection-outside-stratum`) holds. This bead writes no
  number.
- Implemented by: BD-243, N28a-05; extended by N25c-43 (the qualified local connection, M6, Frontier).

**`analyze.closure`** (MCP tool `analyze_closure`; owner `frob-analyze`)
- Inputs: design; budgets.
- Outputs: `closed` or not with the loop gain, the least fixed point of mass and power, the binding budget.
  Evidence: *analysis*.
- Chapter refusals: `FROB-REF-115` (a design budget exceeded: mass, cost, peak power, energy or envelope);
  `FROB-REF-116` (co-design does not close: loop gain at or above one).
- Implemented by: BD-103, N28a-05.

**`analyze.envelope`** (MCP tool `analyze_envelope`; owner `frob-analyze`)
- Inputs: design; operating points or a task; `--strict-data`.
- Outputs: per joint: utilization against the card envelope, thermal steady state, findings with repairs (swap
  actuator, regear). Evidence: *analysis*.
- Chapter refusals: `FROB-REF-113` (an actuator card with a required field of class `assumed` when the caller asked
  for `--strict-data`); `FROB-REF-131` (a required operating point outside the torque, speed and voltage envelope);
  `FROB-REF-132` (no stable thermal equilibrium at a required operating point, or an equilibrium above a limit);
  `FROB-REF-133` (peak demand above the bus ceiling E²/(4·R), checked at end-of-discharge voltage and
  cold-temperature resistance, BD-103 step 4).
- Implemented by: BD-103, N28a-05.

**`analyze.modes`** (MCP tool `analyze_modes`; owner `frob-analyze`)
- Inputs: design; `--stance S`.
- Outputs: frequencies and mode shapes by isotypic block; bandwidth margin per actuator. Evidence: *analysis*.
- Chapter refusals: `FROB-REF-214` (a representation with a constituent of quaternionic type, or a symmetry group
  outside the supported families).
- Implemented by: BD-107, N28a-05.

**`analyze.self-collision`** (MCP tool `analyze_self_collision`; owner `frob-analyze`)
- Inputs: design; joint ranges.
- Outputs: colliding pairs with a witness configuration, or clearance margins over the ranges. Evidence: *analysis*.
- Chapter refusals: `FROB-REF-123` (self-collision or interpenetration at the reference pose or over declared
  ranges).
- Implemented by: BD-104, N28a-05.

**`analyze.coverage`** (MCP tool `analyze_coverage`; owner `frob-analyze`)
- Inputs: design (its sensor mounts and cards); optional `--scene` and task.
- Outputs (N33b-17): per target class (the next k footholds per leg, default k = 2, or wheel-path strips; the
  route corridor out to the lookahead distance d; with a scene, each threshold, step edge and hazard region in
  the corridor) the covered count and total; per uncovered target the reason (`outside-field`, `too-near`,
  `too-far`, `grazing-incidence`, `occluded-by-scene`, `occluded-by-link:<name>`) and the sensor that came
  closest; `covers_lookahead` with the nearest and farthest covered corridor distance against d; ranked repairs
  (tilt or raise the named mount by the smallest change, or reduce the nominal speed); findings
  `A-14.<target>.uncovered` and `A-14.corridor.lookahead-short` (exit 2). Evidence: *analysis*, `Tested` for
  the geometry at the reference pose, D0; the result states that it is static at one pose.
- Chapter refusals: none of its own (an uncovered foothold on the only admissible route contributes to the
  stage-7 refusal `FROB-REF-119`, which BD-244 raises in `design.check`).
- Implemented by: N33b-17, N28a-05; BD-243 checks the A-09, A-14 and A-16 registry entries side by side.

**`analyze.structure`** (MCP tool `analyze_structure`; owner `frob-analyze`)
- Inputs: design; load cases or a load campaign, with the spellings and the default of `foundry.size` (BD-401
  step 6): `--loads <campaign id | file>` (the envelope cases of a load campaign, or a declared load set),
  `--loads statics` (the bounded statics set) or `--loads statics:<factor>` (the same set with one load factor
  for every port); with no `--loads`, as in the registry example, the set is `statics` with the class load
  factors of the design's archetype card, and the result is labelled `load set: bounded-statics` with the
  factors used. A file given to `--loads` is a path, so the generator's path rule applies (NJL-01).
- Outputs: structural margin per link at the beam rung, the governing case and location. Evidence: *analysis*.
- Chapter refusals: `FROB-REF-163` (a check whose material card lacks the field or provenance it needs);
  `FROB-REF-167` (a load set containing a case that is not one instant) and `FROB-REF-169` (a symmetric class's
  load set that does not cover every instance, naming the missing limbs), both raised by the load-set reader
  for a campaign set whatever verb calls it, as on `foundry.size` (VERBS-foundry).
- Implemented by: BD-401 (M5): the margin is `foundry.size`'s at the beam rung, and BD-401 registers A-15 as an
  `Analysis` of `frob-analyze`, a view that reaches that margin through the `MemberSizer` trait and adds no
  second implementation; N28a-05 generates the row from the registration. The row's first milestone is M4 and
  BD-401's is M5, so the row is one of NJL-02's `LATE_ROWS` (lands M5).

**`analyze.sensitivity`** (MCP tool `analyze_sensitivity`; owner `frob-analyze`)
- Inputs (N33b-18): a design or a run; `--metric M`; `--params p1,p2,…` (the parameter set); `--mode
  static|morris|smoothed-gradient`; with `morris` an optional `--from-study <study>`; `--strict-data`.
- Outputs: ranked sensitivities with derivative labels and intervals, the sign attached, each lever as a typed
  edit with its predicted change and the analyses to re-run (locked fields listed, not offered); the
  `card-conditional` label naming the card field that can flip the verdict and the value at which it does; the
  method named. The verb is `read` in every mode and never submits episodes, so `morris` is two calls: without
  `--from-study` it returns the Morris plan (the r trajectories as typed parameter settings, the r·(k + 1) points
  each with its prospective `DesignId`, the world-address set every point shares, the cost estimate in batches,
  episodes and device-seconds) and as `next_actions` the commands `design patch` per point, then `study plan`,
  then `study submit`; with `--from-study` it reads the completed study and returns μ*_i, σ_i and their intervals
  (a running study gives the job's progress and no ranking; missing points give `not-separated`, never an
  estimate from fewer trajectories). `smoothed-gradient` is declared and answers `FROB-REF-724` until its pilot
  passes (on its kill gate it returns the finite-difference stencil labelled `finite-difference`). Evidence:
  *analysis*; exact (D0) for automatic differentiation of a static metric; `Estimated`, screening, with the
  determinism of the tier the episodes ran on, for Morris.
- Chapter refusals: `FROB-REF-263` (a derivative is requested where none exists: the static mode on a simulated
  metric, repair `--mode morris`, or on a static metric whose active term changes within rounding, repair a
  one-sided derivative per side); `FROB-REF-113` (under `--strict-data`, a verdict resting on a card field of
  provenance `assumed`). A task parameter in a Morris parameter set is `FROB-REF-802` (appended by kind; OPEN,
  owner N33b-19: refused, never silently dropped, until the study machinery accepts per-arm settings).
- Implemented by: N33b-18, N28a-05; BD-243 checks the A-09, A-14 and A-16 registry entries side by side.

**`analyze.observability`** (MCP tool `analyze_observability`; owner `frob-analyze`)
- Inputs: design; task; nominal motion.
- Outputs: structural rank, Gramian conditioning along the motion, named weak directions; a pointer to an alias
  witness if one stands. Evidence: *analysis*; the structural rank is exact (it joins `check`), the Gramian
  conditioning is a screening signal.
- Chapter refusals: `FROB-REF-120` (observation alias under every admitted probe).
- Implemented by: BD-120, N28a-05; the alias witness it points to is BD-240's (`sense.alias`, M3, Frontier).

**`analyze.route`** (MCP tool `analyze_route`; owner `frob-analyze`)
- Inputs (N19b-34): design; `--scene`; routes (default: the scene's declared routes); `--margin <m>` (clearance
  margin override, metres); `--budget-ms <n>` (the refinement wall-time budget, default 2,000, the stage-7
  target).
- Outputs: `fits` with a witness path, `cannot-fit` with an impossibility cut, or `unresolved` with its band and
  stop rule, and the bottleneck with margins and shortfall; a `cannot-fit` is a finding (exit 2) with a repair
  as a typed edit; an `unresolved` answer names `frob scene fit --design <d> --scene <s> --route <r>` (the `run`
  verb with a job budget) in `next_actions`. Evidence: *analysis*; decided results claim `Proof`, class
  `Certified`, scoped to the compiled scene and its bounds, with the model and complex identities, D0.
- Chapter refusals: `FROB-REF-119` (no admissible route or reach in the scene; raised by stage 7 of
  `design.check`, BD-244, from this result); `FROB-REF-308` (a clearance query is outside the domain of the
  complex: the footprint changes shape along the route without declared postures, a height or step capability is
  missing; raised by this analysis).
- Implemented by: N19b-34 (A-18 on the clearance complex of BD-233), N28a-05.

**`analyze.rank-robustness`** (MCP tool `analyze_rank_robustness`; owner `frob-analyze`)
- Inputs: campaign or finalist set; model-form variants.
- Outputs: per variant the ranking and the flips; `model-robust` or `model-conditional`; created-energy flags.
  Evidence: *analysis* (the claim product of the runs read).
- Chapter refusals: none of its own. The view sets the `model-conditional` condition and raises nothing about
  it: `FROB-REF-557` (a `model-conditional` result offered as a design comparison without its condition) is
  raised by the verbs that present the ranking, `report.design` and `report.campaign`, and `FROB-REF-830` (the
  promotion, build-packet and prototype-budget half) where the result is spent, by the one-code precedence rule
  of the refusal registry N28-REF, whose row for 557 names those verbs and not this one. A-19 never joins
  `design.check`, so no stage raises a code from its findings either; `A-19.model-conditional` is a High
  finding, exit 2.
- Implemented by: BD-346 (cascade stage 5, the rule over the model-form check of BD-266, `fidelity.modelform`:
  one mechanism), which registers A-19 as an `Analysis` of `frob-analyze`, a read view over the stored stage-5
  record that re-runs nothing; N28a-05 generates the row from the registration.

**`analyze.energy-budget`** (MCP tool `analyze_energy_budget`; owner `frob-analyze`)
- Inputs (N28a-07, which fixes the spellings): design; `--task <t>` (the segment: rise h, path length d, optional
  deadline T, the admitted approach-speed interval); `--budget
  stored=<E>J[,peak=<P>W][,injected=<E>J][,elastic=<E>J]` (for example `stored=0.90J`; peak power and the bus
  otherwise from the design's budgets and cards); `--declare <source>[,<source>]` (the energy sources the
  request declares present beyond stored electrical and gravitational potential, which are always enclosed:
  `approach`, `injected`, `elastic`, `external`); `--approach-speed <v_lo>..<v_hi>` (m/s; binds `approach` and
  replaces the task's interval); `--approach-energy <E>J` (binds `approach` by an upper bound on the kinetic
  energy at the start of the segment); `--contact-energy-cap <W>J` (M1); from M4 a bound `.fenergy` absorber
  envelope. `approach` declared and bound neither by an option nor by the task is the unbound source of 551.
- Outputs: the mandatory work W_min = m·g·max(0, h) + D_min and the available energy E_avail with each source's
  contribution (stored, injected, elastic, approach kinetic energy over the admitted speeds); an exclusion typed
  `outer` (`A-20.energy`, and `A-20.power` under a deadline) or `unresolved` with the slack, never `inner`; the
  contact-energy finding `A-20.contact-energy` when the deliverable energy exceeds the cap. Evidence: *analysis*,
  D0, `Tested` or `Estimated` when a card field read is `assumed`.
- Chapter refusals: `FROB-REF-551` (an exclusion from an analysis whose model does not enclose a declared state or
  energy source: a declared source outside the enclosed list, or approach motion declared without its speed
  interval bound); `FROB-REF-115` (a design budget exceeded: the code of `A-20.energy` and `A-20.power` raised by
  stage 6 of `design.check` when a budget or segment is bound). `FROB-REF-451` is not this verb's: it is raised by
  `shield.budget` and `shield.check` (N22-REF), which an earlier draft of this row confused with A-20.
- Implemented by: N28a-07, N28a-05; joined to `design.check` by BD-108.

**`analyze.deadlines`** (MCP tool `analyze_deadlines`; owner `frob-analyze`)
- Inputs: design; task (clocks, delays).
- Outputs: per decision: the arrival instant against the decision instant in exact rational time; the first late
  packet. Evidence: *analysis*.
- Chapter refusals: `FROB-REF-440` (the clock lattice is not admissible: a period is zero or negative, or the common
  denominator exceeds the limit). A mandatory decision that no admitted schedule serves has no code (OPEN, owner
  the surface lead, N28a-08); fail-closed until one is registered: the finding `A-21.late` is High and
  `design.check` exits 2.
- Implemented by: N28a-08, N28a-05; joined to `design.check` by BD-108.

**`analyze.loop-stability`** (MCP tool `analyze_loop_stability`; owner `frob-analyze`)
- Inputs: design; control period; feedforward law; from M2 (N16c-42) `--task <t>` (its decimation), `--tier
  <tier>` and `--decimation <d>` (repeatable).
- Outputs: per loop the spectral radius of the one-tick map with the hold modelled exactly (`rho_exact`,
  `stable_exact`); from M2 per (tier, integrator, decimation) `rho_tier` and `stable_on_tier`, the label
  `stable`, `stable-exact-only` (the label withheld for that tier: the High finding `A-22.<joint>.tier-unstable`,
  exit 2, never a refusal) or `unstable`, and the smallest decimation at which ρ_tier < 1. Evidence: *analysis*;
  D0 where both maps are rational.
- Chapter refusals: none of its own.
- Implemented by: N28a-09, N28a-05; extended by N16c-42 (the per-tier evaluation, M2); joined to `design.check`
  by BD-108.

**`analyze.bus-schedule`** (MCP tool `analyze_bus_schedule`; owner `frob-analyze`)
- Inputs: design; time-resolved demand profiles; `--deadline <n>`.
- Outputs: a joint schedule, or infeasibility (`A-23.bus-infeasible`) with the conflicting consumers and ticks.
  Evidence: *analysis*.
- Chapter refusals: `FROB-REF-133` (peak demand above the bus ceiling E²/(4·R); the stage-6 code of
  `A-23.bus-infeasible` in `design.check`).
- Implemented by: N28a-10, N28a-05; joined to `design.check` by BD-108.

**`analyze.wrench`** (MCP tool `analyze_wrench`; owner `frob-analyze`)
- Inputs (N28ap1-02): design; `--skill S`, or `--wrench Fx,Fy[,Fz,Mx,My,Mz]` with `--frame F`; `--states …`
  (the stance and sampled states; the standalone verb covers 64 states inside 200 ms, TARGET).
- Outputs: `instantaneously-attainable` with a feasible allocation (u, f_c) that replays on the referee, or an
  infeasibility witness (direction c, support value h(c), the gap cᵀ·w* − h(c), the binding actuator or
  contact), typed `outer` for the instantaneous model, never `inner`; the dual certificate re-checked in exact
  rationals and canonicalized; the finding `A-24.unattainable` with ranked repairs. Exit 0 when attainable at
  every state, 2 with a witness, 3 for an invalid request or a missing binding. Evidence: *analysis*,
  `Certified` when the certificate is emitted, else `Tested`.
- Chapter refusals: `FROB-REF-448` (a skill's task wrench lies outside the body's attainable set; the stage-6
  code of `A-24.unattainable` in `design.check` when a skill or task wrench is bound).
- Implemented by: N28ap1-02, N28a-05; BD-348 records its GATE-ATEL-05 row.

**`analyze.identifiability`** (MCP tool `analyze_identifiability`; owner `frob-analyze`)
- Inputs (N28a-13, which fixes the spellings): design; `--params <field>[,<field>]` (card and scene fields, each
  prior read from the field's declared interval or standard deviation); `--excitation <file>` (the proposed
  runs with commanded inputs, conditions and the readings each would record); optionally `--decision <file>`
  (the pending decision as a partition of parameter space: which values imply which design choice). Both files
  are paths, so the generator's path rule applies (NJL-01).
- Outputs: the rank of the scaled sensitivity of predicted readings with each structurally unidentified
  direction (σ_min/σ_max below 10⁻⁶); `zero-sensitivity` parameters; aliased combinations (|cos| ≥ 0.95) with the
  combination that is identified and what separates them (eight seeded structural aliases); regime coverage
  (`always`, `never`, `mixed`) of each switching law per run; the advisory precision bound;
  `identification-not-needed` when every prior value implies the same choice; findings
  `A-25.<param>.unidentified` with the cheapest excitation that enters the regime, and `A-25.<a>~<b>.aliased`.
  Exit 2 when anything is unidentified. Evidence: *analysis*, `Tested` on the closed-form path; the simulated
  path is a screening signal.
- Chapter refusals: none of its own. A-25 never fits and never refuses; `FROB-REF-879` (a fit on a parameter set
  that fails identifiability), listed here by an earlier draft, is raised by `ground.identify` and card-field
  updates (N30-REF, M5) on the verdict this analysis computes.
- Implemented by: N28a-13, N28a-05; BD-348 records its GATE-ATEL-05 row.

**`analyze.regimes`** (MCP tool `analyze_regimes`; owner `frob-analyze`)
- Inputs (N28a-16): one or more runs of one design (`<run…>`); `--compare <id>` (a paired comparison);
  `--metric M` (for the local models); `--from-study <study>` (the completed paired perturbation study to read;
  the spelling `analyze.sensitivity` uses); `--budget N` (a token budget, as the explain verbs take);
  `--estimate` (the episode cost of the local models' paired perturbation runs, without running them: the row
  has `estimable = true`). The verb is `read`: it reads recorded runs and never submits episodes, so on the
  branch that keeps the label the local models take two calls. (a) With `--metric` and without `--from-study`
  it returns the descriptive atlas and the perturbation plan (per (region, lever) the lever's step up and
  down, the world addresses of the region's visits, each perturbed design's typed edits and prospective
  `DesignId`, and the cost `--estimate` prints), with `next_actions` in order: `design patch` per perturbed
  design, `study plan`, `study submit`; every region is `distrust` with reason `too-few-visits` until the
  study is read. (b) With `--from-study` it reads the completed study, fits the local models and assigns the
  labels; a study still running gives the job's progress and no labels, and a missing perturbed run leaves its
  (region, lever) `distrust`, `too-few-visits`. On the fallback branch of KILL-ATLAS there is no local model
  and `--from-study` is `FROB-REF-802` with the pointer at the option.
- Outputs (`frob.analyze.regimes.out/1`): the regions keyed by (active contact sites, stick or slip per site,
  saturated joints by kind, derivative-valid), with visits, worlds, share of operating time, mean dwell and the
  regime graph; `transient` intervals kept and flagged; per region with a failure the shortest replayable
  failure (world address, tick range, replay handle); with `--compare` the ranking-change regions; streams not
  recorded named per stream. On the branch KILL-ATLAS keeps (decision N28a-15): per (region, lever) a local model
  with residual and visits, and the validity label `trust` or `distrust` with its reason; on the fallback the
  fields `validity` and `local_model` are absent from the registered schema. Exit 2 when any region is
  `distrust` or holds a failure. Evidence: *analysis*, `Estimated`, `screening: true`, `claims: []`, the claim
  product of the runs read; D0 over integers and event ticks.
- Chapter refusals: none of its own. Derivative invalidity is a label of a region (`distrust`,
  `derivative-invalid`), never a refusal, and the atlas requests no derivative; `FROB-REF-263`, listed here by an
  earlier draft, is not raised by it.
- Implemented by: N28a-16, N28a-05 (pilot N28a-14, decision N28a-15); BD-348 records its GATE-ATEL-05 row.

**Machine fields of each row.**

| Verb | Capabilities | Output schema | Task clusters | Example |
|---|---|---|---|---|
| `analyze.mobility` | `analyze:read` | `frob.analyze.mobility.out/1` | `analyze-body` | `frob analyze mobility quad-a --json` |
| `analyze.mass` | `analyze:read` | `frob.analyze.mass.out/1` | `analyze-body` | `frob analyze mass quad-a --json` |
| `analyze.symmetry` | `analyze:read` | `frob.analyze.symmetry.out/1` | `analyze-body`, `symmetry` | `frob analyze symmetry quad-a --json` |
| `analyze.gait` | `analyze:read` | `frob.analyze.gait.out/1` | `analyze-motion` | `frob analyze gait quad-a --json` |
| `analyze.workspace` | `analyze:read` | `frob.analyze.workspace.out/1` | `analyze-body` | `frob analyze workspace quad-a --json` |
| `analyze.singularities` | `analyze:read` | `frob.analyze.singularities.out/1` | `analyze-body` | `frob analyze singularities quad-a --json` |
| `analyze.maneuverability` | `analyze:read` | `frob.analyze.maneuverability.out/1` | `analyze-motion` | `frob analyze maneuverability quad-a --json` |
| `analyze.stability` | `analyze:read` | `frob.analyze.stability.out/1` | `analyze-motion` | `frob analyze stability quad-a --json` |
| `analyze.curvature` | `analyze:read` | `frob.analyze.curvature.out/1` | `analyze-motion` | `frob analyze curvature snake-a --shape joint1,joint2 --loop rect:0.05,0.05 --json` |
| `analyze.closure` | `analyze:read` | `frob.analyze.closure.out/1` | `analyze-budget` | `frob analyze closure quad-a --json` |
| `analyze.envelope` | `analyze:read` | `frob.analyze.envelope.out/1` | `analyze-budget`, `actuators` | `frob analyze envelope quad-a --json` |
| `analyze.modes` | `analyze:read` | `frob.analyze.modes.out/1` | `analyze-body` | `frob analyze modes quad-a --json` |
| `analyze.self-collision` | `analyze:read` | `frob.analyze.self-collision.out/1` | `analyze-body` | `frob analyze self-collision quad-a --json` |
| `analyze.coverage` | `analyze:read` | `frob.analyze.coverage.out/1` | `analyze-evidence` | `frob analyze coverage quad-a --json` |
| `analyze.structure` | `analyze:read` | `frob.analyze.structure.out/1` | `analyze-budget`, `structure` | `frob analyze structure quad-a --json` |
| `analyze.sensitivity` | `analyze:read` | `frob.analyze.sensitivity.out/1` | `analyze-budget` | `frob analyze sensitivity quad-a --metric mass.total --mode static --json` |
| `analyze.observability` | `analyze:read` | `frob.analyze.observability.out/1` | `analyze-evidence` | `frob analyze observability quad-a --json` |
| `analyze.route` | `analyze:read` | `frob.analyze.route.out/1` | `analyze-motion` | `frob analyze route carrier-a --scene home-fixture --json` |
| `analyze.rank-robustness` | `analyze:read` | `frob.analyze.rank-robustness.out/1` | `analyze-evidence` | `frob analyze rank-robustness camp:12 --json` |
| `analyze.energy-budget` | `analyze:read` | `frob.analyze.energy-budget.out/1` | `analyze-budget` | `frob analyze energy-budget cart-a --task sill --budget stored=0.90J --json` |
| `analyze.deadlines` | `analyze:read` | `frob.analyze.deadlines.out/1` | `analyze-budget` | `frob analyze deadlines quad-a --json` |
| `analyze.loop-stability` | `analyze:read` | `frob.analyze.loop-stability.out/1` | `analyze-motion` | `frob analyze loop-stability quad-a --json` |
| `analyze.bus-schedule` | `analyze:read` | `frob.analyze.bus-schedule.out/1` | `analyze-budget` | `frob analyze bus-schedule quad-a --json` |
| `analyze.wrench` | `analyze:read` | `frob.analyze.wrench.out/1` | `analyze-motion` | `frob analyze wrench arm-2r --wrench 10,0 --frame tool --json` |
| `analyze.identifiability` | `analyze:read` | `frob.analyze.identifiability.out/1` | `analyze-evidence`, `calibrate` | `frob analyze identifiability cart-b --params mu --excitation two-runs.toml --json` |
| `analyze.regimes` | `analyze:read` | `frob.analyze.regimes.out/1` | `analyze-evidence` | `frob analyze regimes r:19e2 --json` |

The remaining fields follow by rule: `input_schema` is `frob.<verb name>.in/1`; `refusals` is the chapter list of
the verb's block plus, for every verb of this cluster, `FROB-REF-551` (an exclusion from an analysis whose model
does not enclose a declared state or energy source), which any analysis that excludes can raise and which the
blocks above print only where it is the verb's characteristic refusal; `idempotent` is `true` for `read` and `admin` and `false` for `run` and `mutate` (which take an
idempotency key); `cost_model` is `static: …` for `read`, `mutate` and `admin` and the `--estimate` cost model for
`run`; a `run` row carries `wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }`; `owner` is the
crate of the block; `purpose` is the Purpose column. A row printed `S/F` (a Solid core with Frontier extensions)
is registered with `tier = "S"`, because the field takes one of S, F, M; its Frontier extensions are listed by
`frob capabilities` with their pilots and kill gates when their handlers are in the build (N28a-05's generator
emits the same value). An analysis that needs a binding (a scene, a budget, a skill or task wrench, a run or a
campaign) or a required analysis parameter (the metric and mode of A-16; the shape joints and loop of A-09)
names it in its example, so that the example runs verbatim in the cluster script; the fixture `snake-a` (an
instance of the `snake-crawler` template, joints `joint1` and `joint2`) is stored for the A-09 example in the
fixture store the cluster scripts run against by its implementing bead (BD-243; a fixture name is not
substituted by NJL-02's identity table). `analyze.regimes` has `estimable = true` (it gets
`estimate_only`, the `--estimate` form, and no budget property or 70x code); the other twenty-five do not.

One entry in full (the others follow field for field):

```toml
[[verb]]
name = "analyze.mobility"
cluster = "analyze"
kind = "read"
tier = "S"
capabilities = ["analyze:read"]
input_schema = "frob.analyze.mobility.in/1"
output_schema = "frob.analyze.mobility.out/1"
refusals = ["FROB-REF-110", "FROB-REF-551"]
idempotent = true
cost_model = "static: answered from the store or by a small CPU computation; no job"
first_milestone = "M1"
task_clusters = ["analyze-body"]
examples = ["frob analyze mobility quad-a --json"]
owner = "frob-analyze"
purpose = "A-01: How many degrees of freedom the mechanism really has, and whether any loop is over-constrained"
```

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
4. Rows are entered when the cluster is first registered (M1); each row's tool is added to the task-cluster
   tables its `task_clusters` names in the same change (NJL-05's staging rule). A verb whose handler is not in
   the build answers one envelope with `FROB-REF-724`, exit 5, on both surfaces, and `frob capabilities --json`
   lists it as absent; a reduced form that lands before the row's `first_milestone` answers and is listed as
   implemented with its form named (NJL-01, NJL-03: the field keeps the appendix's milestone).

**Notes for this cluster.**

- Only `analyze.symmetry` (A-03) is spelled by the inventory. The other 25 names are DESIGN defaults fixed by the
  bead that generates the cluster from the analysis registry of `frob-analyze` (N28a-05); this bead adopts them so
  that one spelling exists.
- Three machine fields were drafted differently in the generator's bead and are settled as follows (DESIGN, with
  the reason; the same text is in N28a-05). `capabilities` is `["analyze:read"]` for all twenty-six, A-19 and
  A-26 included: the scope catalogue has one read scope per cluster, granting that cluster's `read` verbs (the
  generator's draft had `design:read`, with `run:read` for the two that read runs; what a session may see of
  another namespace or of a private scene is decided by the namespace and private-scope rules). `output_schema`
  is one name per verb, `frob.analyze.<name>.out/1`, so that snapshots and the breaking-change classification
  are per verb (the generator's draft had the single name `frob.analyze.out/1`); the shape is common, each
  schema being the generator's `AnalysisResult` template (`output`, `findings`, `bound`, `authority`,
  `screening`, `status`) with `output` specialized. `refusals` carries `FROB-REF-551` on every row, as the
  generator's draft had it (this bead had it on `analyze.energy-budget` only).
- What a chapter code on an analysis row means (N28a-05 item 4): a verb never refuses because of a finding.
  Most of the codes in the blocks are the codes an analysis's findings carry as `Refusing(code)` severities,
  which `design.check` raises at the analysis's stage; called as its own verb, the analysis reports the same
  finding with exit 2. The verb itself refuses (exit 3) only for bad input, a missing binding, 551, or a
  refusal the block says it raises itself (215 on `analyze.symmetry` and `analyze.workspace`, 308 on
  `analyze.route`, 263 and 113 on `analyze.sensitivity`, 440 on `analyze.deadlines`, 167 and 169 on
  `analyze.structure`, 431 on `analyze.gait`, and the stratum refusal `connection-outside-stratum` on
  `analyze.curvature`). The list is what the generator emits from each analysis's declared codes, so the two
  meanings share one field.
- Every analysis verb takes, beside the inputs of its block, the common inputs of the generator (N28a-05): the
  design as a name or a `DesignId`, resolved to immutable identities echoed in `bound`, and `--time-budget <ms>`,
  the wall-time bound of the call (DESIGN: milliseconds, the unit `design.check` gives the option of the same
  name, so that one spelling has one unit). An analysis whose registration declares a budgeted part (the orbit
  part of A-04: 60,000 ms) solves until the budget ends and reports the rest `not-run` with reason `budget`,
  status `Undecided`, exit 2.
- Extensions after a row's first milestone (A-01's lattice at M3, A-22's per-tier rows at M2, A-05's certified
  mode at M4, A-03's symmetry events and A-09's connection at M6) are listed in the blocks with their
  milestone. Their options enter the generated input schema when the extending bead lands (an additive change
  that keeps the major version); before that a call that passes one is an unknown parameter, `FROB-REF-802`.
- There is no listing command of the cluster on the CLI: what exists at a build is read from
  `frob capabilities --json`. A form `frob analyze --list`, drafted with the generator, would be a CLI command
  with no row and no MCP tool and would fail this bead's parity check; it is not built.
- Seven analyses share their mathematics with a verb of another cluster: `analyze.observability` with
  `sense.observability`, `analyze.route` with `scene.fit` and `contact.clearance`, `analyze.gait` with `gait.atlas`
  and `gait.orbit`, `analyze.structure` with `foundry.size`, `analyze.wrench` with `skill.check`,
  `analyze.identifiability` with `ground.excite`, `analyze.rank-robustness` with `fidelity.modelform`. The `analyze` verb is the studio's finding-shaped view (findings
  with ranked repairs as typed edits) and calls the other computation; it adds no second implementation. This is not
  an alias in the sense of the one-name rule: the two verbs have different output schemas.
- `frob design check` runs the analyses marked for it (A-01, A-02, A-03, A-08, A-10, A-11, A-13, A-17 structural
  rank, A-21, A-22, A-23 always; A-07 for wheeled bases; A-18, A-20, A-24 when a scene, a budget or a skill is
  bound) and returns in under 2 s for a template-sized design (TARGET). A-19 and A-26 read runs and never join
  `check`.
- Four rules every analysis verb obeys: a feasibility result is typed `exact`, `inner` or `outer` and names the
  authority model it assumed; an exclusion encloses every state and every energy source (A-20 always includes
  approach kinetic energy); an analysis is a memo node keyed by the model identity restricted to the fields it reads
  (determinism class D0 for the exact analyses); a screening analysis (A-04 orbits, A-17 conditioning, A-26 regions)
  is labelled as one and may not establish a comparison.
- Milestones: the appendix gives the family M1 to M4. `analyze.structure` (A-15) is entered at M4 with
  `foundry.size`, whose seed bead (BD-401) is at M5; until that bead lands the verb answers the not-implemented
  refusal, which NJL-02's coverage check accepts because the row is one of its declared late rows (`LATE_ROWS`,
  lands M5).

## Acceptance criteria

- [ ] `registries/verbs.toml` holds exactly twenty-six entries with `cluster = "analyze"` (26 `read`), each with
      every field of the entry schema, an example and a task-cluster list; the alias lint finds no second spelling.
- [ ] `frob-surface-diff parity --cluster analyze` exits 0 in process (`frob-mcp` over stdio) and through `frobd
      --mcp`: registry, CLI and MCP each count 26, and every difference list is empty apart from the harness's closed
      exemptions.
- [ ] Every verb has an end-to-end script: `scripts/e2e/verbs_analyze.sh` runs each verb of the cluster
      (twenty-six), and from a verb's first milestone a capability script of its implementing bead exists
      (`frob-surface-diff e2e-coverage`).
- [ ] `frob <verb> --help --json` equals `fixtures/golden/verbs/analyze/<verb>.help.json` byte for byte for each of
      the twenty-six, on two runs, debug and release, Linux and macOS; a planted renamed output property is classified
      `breaking` and refused.
- [ ] A verb not yet implemented answers `FROB-REF-724`, exit 5, with one envelope on both surfaces; an implemented
      verb runs its registry example with the result its implementing bead specifies; a misspelt parameter is
      `FROB-REF-802`, exit 3.
- [ ] The generated refusal table lists for each verb exactly its chapter codes plus the codes appended for its
      kind and fields (NJL-01), and every code exists in `registries/refusals.toml`.
- [ ] The gate row is recorded with the measured counts by a session that did not write the rows.

## Test plan

### Unit tests

`cargo test -p frob-verbs --test cluster_analyze` (the real generated table; no mocks; planted registries are TOML
strings labelled `synthetic`):
- `rows_present`: exactly 26 rows in the cluster, with the names of the rows table; a planted extra row or a removed
  row fails.
- `kinds_milestones_tiers`: each row's kind, first milestone and tier equal the rows table (for example
  `analyze.regimes` is `read` at M4).
- `schemas_and_scopes`: input schemas are `frob.<name>.in/1`; output schemas and capabilities equal the
  machine-fields table.
- `refusal_lists`: each row's chapter codes equal its block plus 551 (`analyze.mobility`: 110, 551;
  `analyze.mass`: 104, 106, 168, 551; `analyze.symmetry`: 109, 214, 215, 263, 551, 852; `analyze.workspace`:
  119, 215, 551; `analyze.envelope`: 113, 131, 132, 133, 551; `analyze.energy-budget`: 115, 551, and not 451;
  `analyze.sensitivity`: 113, 263, 551; `analyze.singularities`, `analyze.coverage`, `analyze.identifiability`
  and `analyze.regimes`, whose blocks have none of their own: 551, and neither 879 nor 263;
  `analyze.rank-robustness`: 551 alone, and neither 557 nor 830; `analyze.structure`: 163, 167, 169, 551;
  `analyze.gait`: 431, 551; `analyze.curvature`: 118, 551 and the code of `connection-outside-stratum`, which
  the test reads from the allocation ledger `registries/refusal_allocations.toml` and compares with the row, no
  number being written in the test); the appended codes equal the rule for its kind and fields
  (`analyze.regimes` also `estimate_only`, the only `estimable` row).
- `tiers_single_valued`: every row's `tier` is one of S, F, M; the nine rows printed `S/F` are registered `S`.
- `examples_need_no_missing_parameter`: the examples of `analyze.sensitivity` (metric and mode) and
  `analyze.curvature` (shape joints and loop) run against the real binary with exit 0 or 2; the same lines with
  `--metric` or `--loop` removed answer exit 3 with the missing parameter named. Both verbs land at M3: on an
  earlier build both lines answer `FROB-REF-724`, exit 5, and the test records that and passes.
- `examples_name_bindings`: the examples of `analyze.route`, `analyze.energy-budget`, `analyze.wrench`,
  `analyze.identifiability`, `analyze.rank-robustness` and `analyze.regimes` name the scene, budget, wrench,
  experiment, campaign or run they need; run against the real binary with the binding removed, each answers exit 3
  with the missing binding named as the repair (N28a-05's rule), which the cluster script must therefore never
  see.
  (The equality of the rows on file with the generator's output is tested once, by N28a-05's
  `generated_block_equals_registry`; this bead's tests read the rows on file.)
- `mcp_names_one_to_one`: the tool names are the mapped names (`analyze_mobility`, `analyze_mass`,
  `analyze_symmetry`), with no collision against any other cluster.
- `task_clusters_and_examples`: every row sits in a task cluster of at most seven tools or is a base tool; each
  example parses against the generated CLI and its command path is the row's own.
- `kind_behaviour`: a verb whose capability is absent returns code 724, exit 5, one envelope.

### End-to-end script and logging

`scripts/e2e/verbs_analyze.sh` (header: `bead=VERBS-analyze milestone=M1 labels=tether,verbs,verb:analyze.*
gates=GATE-TETHER-02 seed=1 needs=cpu,mcp,frobd plants=alias,missing-row,renamed-field`) sources the shared helper
and `scripts/e2e/verbs_lib.sh` and calls `verbs_cluster_conformance analyze`. Steps: (1) `frob capabilities --json`
and `frob robot-docs --json` list the twenty-six verbs with kind and milestone; (2) `frob-surface-diff parity
--cluster analyze` over stdio and through `frobd --mcp`, every difference list empty; (3) `frob-surface-diff
snapshot --check --cluster analyze`; (4) each verb's registry example verbatim, expecting a result (exit 0 or 2), a
job handle (exit 8, then `frob job wait`) or `FROB-REF-724` exactly when the capability report says the verb is
absent; (5) for each implemented verb the same call as an MCP tool, the two envelopes agreeing on `verb`, `ok`,
`exit`, `result`, `refusal`, `evidence`, `bound`, `warnings` and `next_actions`; (6) per verb one call with a
misspelt parameter (`e2e_expect_refusal FROB-REF-802 3`); (7) one envelope per call and no exit code 1.

Log: `target/e2e-logs/verbs_analyze/<utc>.ndjson` with a transcript beside it. Each `cmd` line has the exact
command, exit code, wall time and the envelope's `evidence` block; each `assert` has observed and expected values
and the tag `--gate GATE-TETHER-02`; the metrics `registry_verbs`, `cli_verbs` and `mcp_tools` are logged against 26
and the difference counts against 0; refusals are logged with code and first repair; a failure logs the first
differing verb and field; the `summary` line gives assertions passed and failed and the gate touched.

### Known answers and negative controls

- Known answers: the cluster has 26 verbs (26 `read`); the tables above. KA-755 (the exit-code table on malformed,
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

- Twenty-six analyses do not fit the three analysis task clusters of seven that the surface chapter names (21
  seats). This unit adds a fourth, `analyze-evidence`, for the five analyses of sensing and of recorded runs (bead
  NJL-05).
- From M3 the three late analysis verbs of NJLx-51 (`analyze.sweep-monotone`, `analyze.constraint-graph`,
  `analyze.regressor-identity`) join the cluster, which then has 29 rows; NJLx-51 edits `rows_present` and the
  metric thresholds of this bead's script in the same commit. They are carried by that bead, not here.
- Kill gate KILL-ATLAS (A-26's validity label predicts no better than "trust nowhere" at a 95% lower bound):
  fallback, `analyze.regimes` stays registered as a descriptive report with no validity label; the row and schema
  keep their names.
- Kill gate KILL-STATIC (decided by its own bead): if static refusals are downgraded to advisory lints, the verbs
  still return the same findings; only the exit code of `design.check` changes.
- Closing rule: the bead closes at M1, when the rows are registered, the surfaces match and every verb passes its
  example or answers the not-implemented refusal. Verbs of this cluster first answer at M1, M3, M4. The
  per-milestone surface beads (BD-137, BD-312, BD-376) re-run this script as verbs land; that is how GATE-TETHER-02
  is re-measured.
- Scope names are proposals (`<cluster>:<read|run|write|admin>` plus the named scopes of the surface) until the
  capability bead BD-258 fixes the scope registry; a `run` or `write` scope of a cluster implies its `read` scope.
  Inputs and outputs above name contents, not flag spellings, except where a flag is quoted; the implementing bead
  fixes the spellings and the snapshot then pins them (additive changes keep the schema's major version; anything
  else raises it).

## References

Appendix J (J.1 entry form; J.2 inventory rows and J.3 added rows; J.5 task clusters), §28.4, §13.4, §32.4, §32.6,
§32.12.3, §32.18, §32.21; GATE-TETHER-02; INV-TETHER-01, -02; KA-755.
