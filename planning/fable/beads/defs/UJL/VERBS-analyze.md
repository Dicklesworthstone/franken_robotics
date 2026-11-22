---
key: "VERBS-analyze"
title: "frob-verbs registry: cluster `analyze`, twenty-six rows, with parity, e2e and schema-snapshot checks"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "tether", "atelier", "frob-verbs", "solid", "registry", "verb", "test"]
parent: "WP-06"
deps: ["BD-022", "NJL-01", "NJL-02", "STD-E2E"]
needs: []
related: ["BD-101", "N28a-05", "BD-100", "BD-102", "BD-332", "BD-105", "BD-106", "BD-243", "BD-103", "BD-107", "BD-104", "BD-401", "BD-120", "BD-233", "BD-346", "BD-108", "BD-348", "BD-110", "N28a-29", "BD-137", "BD-312", "BD-376", "NJL-03", "NJL-04", "NJL-05"]
gates: ["GATE-TETHER-02"]
plan_refs: ["Appendix J (J.1, J.2, J.3, J.5)", "§28.4", "§13.4", "§32.6", "§32.12.3", "§32.21 (GATE-TETHER-02)"]
---
## Background

FrankenRobotics (a robotics simulation and design system) is reached through the command line `frob`, the daemon
`frobd` and the MCP server `frob-mcp` (the Model Context Protocol, by which an agent harness lists and calls tools).
Every capability is a verb, `cluster.verb`, declared once in `registries/verbs.toml` (crate `frob-verbs`, tier
Solid). The CLI, the daemon protocol, the MCP tools, the schemas, the refusal table and the agent guide are
generated from that file, and gate GATE-TETHER-02 compares the served surfaces with it. The registry holds 243 verbs
in 44 clusters (namespaces).

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
inventory); one bead for all 243 rows (too large to review).

## Scope

In: the twenty-six registry entries of cluster `analyze`; the MCP tool names; the cluster's conformance script; the
help-schema snapshots; each verb's answer before its milestone. Out: behaviour (the implementing beads named below);
the entry schema and lints (NJL-01); the conformance harness (NJL-02); the alias lint (NJL-04); task clusters
(NJL-05, NJL-06); generation of the surfaces (the Tether seed beads).

## Technical approach

**The rows.**

| Verb | Kind | First milestone | Tier | Purpose | Defined in |
|---|---|---|---|---|---|
| `analyze.mobility` | read | M1 | S/F | A-01: How many degrees of freedom the mechanism really has, and whether any loop is over-constrained | §28.4 |
| `analyze.mass` | read | M1 | S | A-02: Mass, centre of mass and inertia by subtree, with physical consistency | §28.4 |
| `analyze.symmetry` | read | M1 | S | Γ_mech and Γ_obs of a design, with the element that breaks each excluded generator (A-03) | §13.4 |
| `analyze.gait` | read | M4 | F | A-04: Which gaits the body can have, with phase lags and stability margins; whether a periodic orbit exists under the baseline controller (a screening signal) | §28.4 |
| `analyze.workspace` | read | M1 | S | A-05: Whether the feet or the hand reach what the tasks require (step height, shelf height) | §28.4 |
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
- *static*: class Tested, tier `static`, determinism `exact` (class D0); `claims` empty unless a certificate is
  attached; `reality: no-claim`.

**`analyze.mobility`** (MCP tool `analyze_mobility`; owner `frob-analyze`)
- Inputs: design; optional exact sample points.
- Outputs: mobility count, rank of the constraint Jacobian at exact rational sample points, the over-constrained
  loops, ranked repairs as typed edits. Evidence: *static*.
- Chapter refusals: `FROB-REF-110` (a closure that over-constrains the mechanism (mobility below one)).
- Implemented by: BD-101, N28a-05.

**`analyze.mass`** (MCP tool `analyze_mass`; owner `frob-analyze`)
- Inputs: design; optional subtree path.
- Outputs: mass, centre of mass and inertia per subtree with provenance; the smallest pseudo-inertia eigenvalue when
  inconsistent. Evidence: *static*.
- Chapter refusals: `FROB-REF-104` (inertia not physically consistent (the smallest eigenvalue of the pseudo-inertia
  is reported)); `FROB-REF-106` (declared mass properties violate the mass-support bound); `FROB-REF-168` (a solid
  body exceeds the mass plausibility bound of its part class and no fill is declared (§11.5)).
- Implemented by: BD-100, N28a-05.

**`analyze.symmetry`** (MCP tool `analyze_symmetry`; owner `frob-analyze`)
- Inputs: design; optional sensor profile, task and scene family.
- Outputs: Γ_mech and Γ_obs as group names and generator lists; for each excluded generator the element (sensor,
  payload or task) that breaks it; near-symmetry repairs as typed edits. Evidence: *static*.
- Chapter refusals: `FROB-REF-109` (declared symmetry differs from computed symmetry); `FROB-REF-214` (a
  representation with a constituent of quaternionic type, or a symmetry group outside the supported families).
- Implemented by: BD-102, N28a-05.

**`analyze.gait`** (MCP tool `analyze_gait`; owner `frob-analyze`)
- Inputs: design; controller; optional design parameter for continuation.
- Outputs: gaits as (H, K) pairs with phase lags; orbit existence, multipliers and residual; standing `screening`.
  Evidence: *static*.
- Chapter refusals: `FROB-REF-431` (a gait derivative was requested at grazing or at a fold).
- Implemented by: BD-332, N28a-05.

**`analyze.workspace`** (MCP tool `analyze_workspace`; owner `frob-analyze`)
- Inputs: design; end effector; required heights or a task.
- Outputs: reachable set summary per end effector, the required points reached or not, margins. Evidence: *static*.
- Chapter refusals: `FROB-REF-119` (no admissible route or reach in the scene).
- Implemented by: BD-105, N28a-05.

**`analyze.singularities`** (MCP tool `analyze_singularities`; owner `frob-analyze`)
- Inputs: design; end effector; optional nominal motion.
- Outputs: singular loci, the closest approach of the nominal motion, conditioning along it. Evidence: *static*.
- Chapter refusals: none of its own.
- Implemented by: BD-105, N28a-05.

**`analyze.maneuverability`** (MCP tool `analyze_maneuverability`; owner `frob-analyze`)
- Inputs: design; optional bound scene.
- Outputs: growth vector, controllability rank, turning footprint, passability against door and aisle widths.
  Evidence: *static*.
- Chapter refusals: `FROB-REF-118` (the base cannot produce a motion the task class requires).
- Implemented by: BD-106, N28a-05.

**`analyze.stability`** (MCP tool `analyze_stability`; owner `frob-analyze`)
- Inputs: design; stance; payload; load-sharing model.
- Outputs: margins typed `exact`, `inner` or `outer`, with the load-sharing (authority) model named; ranked repairs.
  Evidence: *static*.
- Chapter refusals: `FROB-REF-124` (a static margin the task requires is not met).
- Implemented by: BD-100, N28a-05.

**`analyze.curvature`** (MCP tool `analyze_curvature`; owner `frob-analyze`)
- Inputs: design; shape cycle.
- Outputs: holonomy per cycle, the curvature form on the shape space, validity of the kinematic reduction. Evidence:
  *static*.
- Chapter refusals: none of its own.
- Implemented by: BD-243, N28a-05.

**`analyze.closure`** (MCP tool `analyze_closure`; owner `frob-analyze`)
- Inputs: design; budgets.
- Outputs: `closed` or not with the loop gain, the least fixed point of mass and power, the binding budget.
  Evidence: *static*.
- Chapter refusals: `FROB-REF-115` (a design budget exceeded: mass, cost, peak power, energy or envelope);
  `FROB-REF-116` (co-design does not close: loop gain at or above one).
- Implemented by: BD-103, N28a-05.

**`analyze.envelope`** (MCP tool `analyze_envelope`; owner `frob-analyze`)
- Inputs: design; operating points or a task; `--strict-data`.
- Outputs: per joint: utilization against the card envelope, thermal steady state, findings with repairs (swap
  actuator, regear). Evidence: *static*.
- Chapter refusals: `FROB-REF-113` (an actuator card with a required field of class `assumed` when the caller asked
  for `--strict-data`); `FROB-REF-131` (a required operating point outside the torque, speed and voltage envelope);
  `FROB-REF-132` (no stable thermal equilibrium at a required operating point, or an equilibrium above a limit).
- Implemented by: BD-103, N28a-05.

**`analyze.modes`** (MCP tool `analyze_modes`; owner `frob-analyze`)
- Inputs: design; stance.
- Outputs: frequencies and mode shapes by isotypic block; bandwidth margin per actuator. Evidence: *static*.
- Chapter refusals: `FROB-REF-214` (a representation with a constituent of quaternionic type, or a symmetry group
  outside the supported families).
- Implemented by: BD-107, N28a-05.

**`analyze.self-collision`** (MCP tool `analyze_self_collision`; owner `frob-analyze`)
- Inputs: design; joint ranges.
- Outputs: colliding pairs with a witness configuration, or clearance margins over the ranges. Evidence: *static*.
- Chapter refusals: `FROB-REF-123` (self-collision or interpenetration at the reference pose or over declared
  ranges).
- Implemented by: BD-104, N28a-05.

**`analyze.coverage`** (MCP tool `analyze_coverage`; owner `frob-analyze`)
- Inputs: design; sensor profile; route or footholds.
- Outputs: coverage fraction per sensor, blind regions, the first uncovered foothold. Evidence: *static*.
- Chapter refusals: none of its own.
- Implemented by: BD-243, N28a-05.

**`analyze.structure`** (MCP tool `analyze_structure`; owner `frob-analyze`)
- Inputs: design; load cases or a load campaign.
- Outputs: structural margin per link at the beam rung, the governing case and location. Evidence: *static*.
- Chapter refusals: `FROB-REF-163` (a check whose material card lacks the field or provenance it needs).
- Implemented by: BD-401, N28a-05.

**`analyze.sensitivity`** (MCP tool `analyze_sensitivity`; owner `frob-analyze`)
- Inputs: design; metric; parameter set.
- Outputs: ranked sensitivities with derivative labels and intervals; method named (automatic differentiation,
  Morris screening, smoothed gradient). Evidence: *static*.
- Chapter refusals: `FROB-REF-263` (a derivative is requested near grazing, at an ambiguous event order).
- Implemented by: BD-243, N28a-05.

**`analyze.observability`** (MCP tool `analyze_observability`; owner `frob-analyze`)
- Inputs: design; task; nominal motion.
- Outputs: structural rank, Gramian conditioning along the motion, named weak directions; a pointer to an alias
  witness if one stands. Evidence: *static*.
- Chapter refusals: `FROB-REF-120` (observation alias under every admitted probe).
- Implemented by: BD-120, N28a-05.

**`analyze.route`** (MCP tool `analyze_route`; owner `frob-analyze`)
- Inputs: design; scene; route.
- Outputs: `fits` with a witness path, `cannot-fit` with an impossibility cut, or `unresolved`, and the bottleneck.
  Evidence: *static*.
- Chapter refusals: `FROB-REF-119` (no admissible route or reach in the scene); `FROB-REF-308` (a clearance query is
  outside the domain of the complex: the footprint changes shape along the route, a height or step capability is
  missing).
- Implemented by: BD-233, N28a-05.

**`analyze.rank-robustness`** (MCP tool `analyze_rank_robustness`; owner `frob-analyze`)
- Inputs: campaign or finalist set; model-form variants.
- Outputs: per variant the ranking and the flips; `model-robust` or `model-conditional`; created-energy flags.
  Evidence: *static*.
- Chapter refusals: `FROB-REF-557` (a `model-conditional` result offered as a design comparison without its
  condition, or as input to a build packet).
- Implemented by: BD-346, N28a-05.

**`analyze.energy-budget`** (MCP tool `analyze_energy_budget`; owner `frob-analyze`)
- Inputs: design; task or mission segment; energy budget.
- Outputs: lower bound on mandatory work against stored, injected and approach kinetic energy; result typed `outer`
  only for the enclosed model. Evidence: *static*.
- Chapter refusals: `FROB-REF-451` (an energy budget omits a source in the port graph, or a class above `Estimated`
  is requested on an `assumed` absorber envelope); `FROB-REF-551` (an exclusion from an analysis whose model does
  not enclose a declared state or energy source).
- Implemented by: BD-108, N28a-05.

**`analyze.deadlines`** (MCP tool `analyze_deadlines`; owner `frob-analyze`)
- Inputs: design; task (clocks, delays).
- Outputs: per decision: the arrival instant against the decision instant in exact rational time; the first late
  packet. Evidence: *static*.
- Chapter refusals: `FROB-REF-440` (the clock lattice is not admissible: a period is zero or negative, or the common
  denominator exceeds the limit).
- Implemented by: BD-108, N28a-05.

**`analyze.loop-stability`** (MCP tool `analyze_loop_stability`; owner `frob-analyze`)
- Inputs: design; control period; feedforward law.
- Outputs: spectral radius of the one-tick map per loop, with the hold modelled exactly. Evidence: *static*.
- Chapter refusals: none of its own.
- Implemented by: BD-108, N28a-05.

**`analyze.bus-schedule`** (MCP tool `analyze_bus_schedule`; owner `frob-analyze`)
- Inputs: design; time-resolved demand profiles.
- Outputs: a joint schedule, or infeasibility with the conflicting consumers and ticks. Evidence: *static*.
- Chapter refusals: `FROB-REF-133` (peak demand above the bus ceiling E²/(4·R)).
- Implemented by: BD-108, N28a-05.

**`analyze.wrench`** (MCP tool `analyze_wrench`; owner `frob-analyze`)
- Inputs: design; skill or task wrench; stance.
- Outputs: `attainable` with an allocation, or a separating dual vector. Evidence: *static*.
- Chapter refusals: `FROB-REF-448` (a skill's task wrench lies outside the body's attainable set).
- Implemented by: BD-348, N28a-05.

**`analyze.identifiability`** (MCP tool `analyze_identifiability`; owner `frob-analyze`)
- Inputs: design; card set; proposed excitation.
- Outputs: rank of the sensitivity of predicted readings; the aliased parameter combinations; regime coverage of
  each switching law. Evidence: *static*.
- Chapter refusals: `FROB-REF-879` (a fit on a parameter set that fails identifiability).
- Implemented by: BD-348, N28a-05.

**`analyze.regimes`** (MCP tool `analyze_regimes`; owner `frob-analyze`)
- Inputs: run or campaign.
- Outputs: a partition of recorded operation by mode, slip, saturation and derivative validity; per region a local
  model, an error estimate and the shortest replayable failure. Evidence: *static*.
- Chapter refusals: `FROB-REF-263` (a derivative is requested near grazing, at an ambiguous event order).
- Implemented by: BD-348, N28a-05.

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
| `analyze.curvature` | `analyze:read` | `frob.analyze.curvature.out/1` | `analyze-motion` | `frob analyze curvature quad-a --json` |
| `analyze.closure` | `analyze:read` | `frob.analyze.closure.out/1` | `analyze-budget` | `frob analyze closure quad-a --json` |
| `analyze.envelope` | `analyze:read` | `frob.analyze.envelope.out/1` | `analyze-budget`, `actuators` | `frob analyze envelope quad-a --json` |
| `analyze.modes` | `analyze:read` | `frob.analyze.modes.out/1` | `analyze-body` | `frob analyze modes quad-a --json` |
| `analyze.self-collision` | `analyze:read` | `frob.analyze.self-collision.out/1` | `analyze-body` | `frob analyze self-collision quad-a --json` |
| `analyze.coverage` | `analyze:read` | `frob.analyze.coverage.out/1` | `analyze-evidence` | `frob analyze coverage quad-a --json` |
| `analyze.structure` | `analyze:read` | `frob.analyze.structure.out/1` | `analyze-budget`, `structure` | `frob analyze structure quad-a --json` |
| `analyze.sensitivity` | `analyze:read` | `frob.analyze.sensitivity.out/1` | `analyze-budget` | `frob analyze sensitivity quad-a --json` |
| `analyze.observability` | `analyze:read` | `frob.analyze.observability.out/1` | `analyze-evidence` | `frob analyze observability quad-a --json` |
| `analyze.route` | `analyze:read` | `frob.analyze.route.out/1` | `analyze-motion` | `frob analyze route quad-a --json` |
| `analyze.rank-robustness` | `analyze:read` | `frob.analyze.rank-robustness.out/1` | `analyze-evidence` | `frob analyze rank-robustness quad-a --json` |
| `analyze.energy-budget` | `analyze:read` | `frob.analyze.energy-budget.out/1` | `analyze-budget` | `frob analyze energy-budget quad-a --json` |
| `analyze.deadlines` | `analyze:read` | `frob.analyze.deadlines.out/1` | `analyze-budget` | `frob analyze deadlines quad-a --json` |
| `analyze.loop-stability` | `analyze:read` | `frob.analyze.loop-stability.out/1` | `analyze-motion` | `frob analyze loop-stability quad-a --json` |
| `analyze.bus-schedule` | `analyze:read` | `frob.analyze.bus-schedule.out/1` | `analyze-budget` | `frob analyze bus-schedule quad-a --json` |
| `analyze.wrench` | `analyze:read` | `frob.analyze.wrench.out/1` | `analyze-motion` | `frob analyze wrench quad-a --json` |
| `analyze.identifiability` | `analyze:read` | `frob.analyze.identifiability.out/1` | `analyze-evidence`, `calibrate` | `frob analyze identifiability quad-a --json` |
| `analyze.regimes` | `analyze:read` | `frob.analyze.regimes.out/1` | `analyze-evidence` | `frob analyze regimes quad-a --json` |

The remaining fields follow by rule: `input_schema` is `frob.<verb name>.in/1`; `refusals` is the chapter list of
the verb's block; `idempotent` is `true` for `read` and `admin` and `false` for `run` and `mutate` (which take an
idempotency key); `cost_model` is `static: …` for `read`, `mutate` and `admin` and the `--estimate` cost model for
`run`; a `run` row carries `wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }`; `owner` is the
crate of the block; `purpose` is the Purpose column.

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
refusals = ["FROB-REF-110"]
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

- Only `analyze.symmetry` (A-03) is spelled by the inventory. The other 25 names are DESIGN defaults fixed by the
  bead that generates the cluster from the analysis registry of `frob-analyze` (N28a-05); this bead adopts them so
  that one spelling exists.
- Six analyses share their mathematics with a verb of another cluster: `analyze.observability` with
  `sense.observability`, `analyze.route` with `scene.fit` and `contact.clearance`, `analyze.gait` with `gait.atlas`
  and `gait.orbit`, `analyze.structure` with `foundry.size`, `analyze.wrench` with `skill.check`,
  `analyze.identifiability` with `ground.excite`. The `analyze` verb is the studio's finding-shaped view (findings
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
  refusal.

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
      kind, and every code exists in `registries/refusals.toml`.
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
- `refusal_lists`: each row's chapter codes equal its block (`analyze.mobility`: 110; `analyze.mass`: 104, 106, 168;
  `analyze.symmetry`: 109, 214); the appended codes equal the rule for its kind.
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
