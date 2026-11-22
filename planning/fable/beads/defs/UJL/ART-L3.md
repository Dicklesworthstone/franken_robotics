---
key: "ART-L3"
title: "golden fixture L.3: the `.ftask` kitchen tray delivery, its eleven labelled traces, the task.check table and CLI script"
type: "task"
priority: 2
milestone: "M3"
labels: ["m3", "arena", "frob-spec", "frob-conformance", "solid", "test", "e2e"]
parent: "WP-24"
deps: ["BD-121", "BD-122", "BD-225", "BD-227", "BD-235", "NJL-07", "STD-E2E"]
needs: ["§24.4: the `.ftask` schema `frob.task/1` with nine sections, canonical JSON and the content-addressed `TaskId` (drafted by unit U24a as N24a-02)", "§24.5: `task.check` with labelled example traces, monitor agreement, the mutation check and vacuous atoms (drafted by unit U24c as N24c-01)"]
related: ["N24c-01", "N24a-02", "N24c-02", "N24c-06", "BD-078", "BD-124", "BD-195", "BD-255", "BD-141", "ART-L6", "ART-L8", "ART-L9"]
gates: []
plan_refs: ["Appendix L.3", "§24.4", "§24.5", "§24.7", "§24.12", "§20.4", "§23.3", "§19.8"]
---
## Background

In FrankenRobotics (a robotics simulation and design system reached through the command line `frob`) a
**task** is data: a `.ftask` file (format FMT-FTASK, schema `frob.task/1`) whose canonical form is JSON and
whose `TaskId` is the BLAKE3 hash of that form; an S-expression view is lossless and optional. A task has nine
sections. The specification fixes the nine section names, their contents and the grammar of the
`requirements` section; the keywords inside the other eight sections are proposals confined to those contents.

This bead turns the third worked artifact of the specification into a golden fixture: the task "kitchen tray
delivery" of the nine-artifact story (a quadruped carries a tray through a commercial-kitchen galley and sets
it down at a pass counter). It is the task that the adversarial search, the paired study, the autopsy and the
qualification of the later artifacts all name, as `t:kitchen-tray@88c1…`. The numbers are DESIGN specimens of
the format; the tick and margin arithmetic is exact.

Why this fixture matters (DESIGN). The requirement language is where an agent's intent becomes a verdict, and
a model translating natural language into temporal logic is wrong about one time in four (FACT cited by the
specification: 77.7% accuracy over 300 samples), so a requirement is admitted only with labelled example
traces and a mutation check. A complete task with signals on several clocks, an until-form verdict and
labelled traces is the regression test for that admission. Rejected: a fixture with the requirement section
alone (the checks that cross sections, such as clock reductions and the reach against the horizon, would not
run).

## Scope

In: the specimen in the S-expression view and its resolved and canonical forms; the eleven synthetic labelled
traces that make the admission check pass; the table of what `task.check` establishes, with the refusal each
check raises when planted; the margin and tick known answers; the script. Out: the requirement language and
its evaluators (BD-121), the task file and its verbs (BD-122, N24c-01), the kitchen archetype and the hazard
registries (BD-225, BD-227), scene splits (BD-235); the layers and the identity map (NJL-07).

## Technical approach

**Files** under `fixtures/golden/appendix-l/L3/`: `kitchen-tray.specimen.ftask.sexpr` (the text below);
`kitchen-tray.resolved.ftask.sexpr` (identities filled, eleven examples); `kitchen-tray.canonical.ftask.json`
(written by the implementation, blessed once); `traces/*.json` (the eleven synthetic traces);
`check.expect.json`; `fields.toml`.

**The artifact** (S-expression view).

```lisp
(task "kitchen-tray" :schema "frob.task/1"
  (interface
    (ports (mech base) (elec bus) (payload tray))
    (observation proprio-and-scan :frame base :rate 50 Hz
      (joint-pos 12 rad) (joint-vel 12 rad/s) (base-gyro 3 rad/s) (gravity-dir 3 1)
      (scan 64 m) (goal-vec 2 m) (sample-age 2 s))
    (action joint-targets 12 rad :rate 50 Hz :law latent-then-limit))
  (clocks
    (physics 1/500 s :phase 0 s) (imu 1/500 s :phase 0 s) (policy 1/50 s :phase 0 s)
    (lidar 1/10 s :phase 1/20 s) (req 1/50 s :phase 0 s))
  (terms
    (obs joint-pos    :unit rad   :noise encoder-14b :delay 1 ms :rep trivial)
    (obs base-gyro    :unit rad/s :noise imu-generic :delay 2 ms :rep pseudovector)
    (obs scan         :unit m     :noise lidar-2d-generic :delay 30 ms :rep permutation)
    (reward progress  :unit 1/s   :rep invariant)        ; integrated over simulated time
    (reward tray-calm :unit 1/s   :rep invariant)
    (terminate success :on (decided delivered satisfied))
    (terminate failure :on (decided delivered violated)))
  (signals
    (dist_to_pass     m     :kind state            :clock physics :reduce last)
    (base_speed       m/s   :kind state            :clock physics :reduce max)
    (tilt             deg   :kind interval-extremum :clock physics :reduce max)
    (tray_tilt        deg   :kind interval-extremum :clock physics :reduce max)
    (tray_acc         m/s^2 :kind interval-extremum :clock physics :reduce max)
    (clearance_min    m     :kind interval-extremum :clock physics :reduce min)
    (winding_temp_max K     :kind state            :clock physics :reduce max)
    (spill_events     1     :kind count            :clock physics :reduce last))
  (requirements
    (atom at-pass    (>= (- 0.15 m dist_to_pass) 0 m)        :scale 0.15 m)
    (atom settled    (>= (- 0.05 m/s base_speed) 0 m/s)      :scale 0.05 m/s)
    (atom upright    (>= (- 12 deg tilt) 0 deg)              :scale 12 deg)
    (atom tray-level (>= (- 8 deg tray_tilt) 0 deg)          :scale 8 deg)
    (atom gentle     (>= (- 3 m/s^2 tray_acc) 0 m/s^2)       :scale 3 m/s^2)
    (atom clear      (>  clearance_min 0 m)                  :scale 0.05 m)
    (atom cool       (>= (- 383 K winding_temp_max) 0 K)     :scale 20 K)
    (atom no-spill   (=  spill_events 0)                     :exact)
    (online placed   (H (0 s 1 s) (and at-pass settled tray-level)))
    (online carrying (and upright tray-level gentle))
    (verdict delivered (U (0 s 20 s) (and upright tray-level gentle clear cool no-spill) placed))
    (example trace:b3:1c0a… satisfied) (example trace:b3:77e2… satisfied)
    (example trace:b3:a04f… violated)  (example trace:b3:5d19… violated))
  (randomization
    (draw floor-friction :from card:rubber-60a/tile:uncertainty :stream world.material)
    (draw film           :from registry:hazards/commercial-kitchen:wet-floor :stream world.film)
    (draw closer-torque  :from registry:hazards/commercial-kitchen:swing-door :stream world.fixture)
    (draw tray-fill      :from registry:hazards/commercial-kitchen:hot-liquid :stream world.payload)
    (draw imu-bias       :from card:imu-generic:uncertainty :stream sensor.imu))
  (reset
    (settled-pose :count 64 :speed-below 1e-3 :for 0.2 s :penetration-max 0.5 mm :first-impulse-within 5 %))
  (world
    (scenes :family kitchen-commercial/galley :split split:b3:60d3… :group-by building)
    (horizon 22 s))
  (study
    (default :type fixed-n :n 64 :margin 0.05 :rule paired-clopper-pearson/1 :level 0.05)
    (budget :gpu-s 600)))
```

**The nine sections and what this file puts in them.** `interface`: required ports, an observation layout of
98 scalars at 50 Hz in the base frame (12 joint positions, 12 joint velocities, 3 gyro components, 3
gravity-direction components, 64 scan ranges, a 2-vector to the goal, 2 sample ages) and an action of 12 joint
targets at 50 Hz. `clocks`: physics and IMU at 1/500 s, policy and requirement at 1/50 s, lidar at 1/10 s with
phase 1/20 s. `terms`: observation terms with unit, noise card, delay and the representation under which each
transforms; two reward terms integrated over simulated time; termination on a decided verdict. `signals`: the
eight telemetry channels the requirements read, each with unit, channel kind, clock and reduction.
`requirements`: eight atoms, two online signals, one verdict and labelled examples. `randomization`: five
draws, each from an uncertainty on a card or a hazard registry and bound to a named random stream. `reset`:
the settled-pose cache specification (64 poses; speed below 10⁻³ for 0.2 s; penetration at most 0.5 mm; first
impulse within 5%). `world`: the scene family with its split, grouped by building, and a horizon of 22 s.
`study`: the default study (fixed-n, n = 64, margin 0.05, rule `paired-clopper-pearson/1`, level 0.05) and a
budget of 600 GPU-seconds.

**Grammar of the requirements section** (fixed; the JSON form is the same tree). An atom is `(atom name
(cmp expr expr) :scale quantity)` or `… :exact)`, with `cmp` one of `>`, `>=`, `<`, `<=`, `=`; expressions
are signals, quantities, `+ - * min max abs` and `(delay quantity signal)`. Past-time formulas use `and`,
`or`, `not`, `H` (historically), `O` (once), `S` (since); future-time formulas add `G`, `F`, `U`. A window is
`(a b)`, a closed interval of simulated time, optionally `:round inward|outward`. `(online name past)`
declares an online signal; `(verdict name future)` the verdict; `(example trace satisfied|violated)` a
labelled example.

**Semantics used by the known answers.** Every formula has a truth value and a dimensionless margin ρ. For an
atom with expression e ≥ 0 or e > 0 and scale s, ρ = e/s; an `:exact` atom has margin +1 or −1; `and` takes
the minimum and `or` the maximum; `H` over [a, b] takes the minimum over ticks in [t − b, t − a]; `φ U ψ` over
[a, b] takes the maximum over t′ in [t + a, t + b] of min(ρ(ψ, t′), min of ρ(φ, t″) for t″ in [t, t′)). A
window [a, b] on the requirement clock of period T denotes the ticks j with a ≤ j·T ≤ b, compared in exact
rational arithmetic.

**What the file says.** The until-form verdict `delivered` says that the six carrying conditions (upright,
tray level, gentle, clear, cool, no spill) hold at every requirement tick before the tray is placed, and that
placement completes within 20 s. Placement cannot latch: `H` over 1 s is false the tick any of its three
conditions (at the pass, settled, tray level) lapses.

**What `task.check` establishes for this file**, each a refusal if it fails:

| Check | Here |
|---|---|
| Every atom has a scale of its own dimension or is `:exact` | Seven scaled atoms; `no-spill` is `:exact` on a count |
| Signals crossing clocks declare a reduction | Every signal is reduced from the 1/500 s physics clock to the 1/50 s requirement clock |
| Window bounds are multiples of the requirement period | 1 s and 20 s are 50 and 1,000 ticks of 1/50 s |
| Online signals are past-time | `placed` uses `H`; `carrying` is a conjunction of atoms |
| The horizon covers the reach of the verdict | Reach 20 s, horizon 22 s |
| Labelled examples exist and the monitors agree with them | Two satisfied and two violated traces |
| The mutation check | Reversing a comparator or deleting an atom changes at least one example's label |

| Check | Refusal when planted |
|---|---|
| An atom has no scale and is not `:exact` | `FROB-REF-520` |
| Dimension mismatch inside an atom (for example `(- 12 deg base_speed)`) | `FROB-REF-535` |
| A signal read across clocks without a reduction (drop `:reduce` from `tilt`) | `FROB-REF-525` |
| A window bound that is not a multiple of the period with no rounding (`(0 s 1.01 s)`) | `FROB-REF-538` |
| A future-time operator in an online signal (`(online placed (F …))`) | `FROB-REF-523` |
| A temporal operator with no finite window | `FROB-REF-536` |
| A horizon shorter than the reach of the verdict (`(horizon 19 s)`) | `FROB-REF-526` |
| No labelled examples, or a label the monitor contradicts | `FROB-REF-539` |
| An online window above 4,096 requirement ticks | `FROB-REF-528` |

**Known answers of the arithmetic.**
1. Windows: 1 s and 20 s are 50 and 1,000 ticks of 1/50 s; the `H` window (0 s, 1 s) holds 51 ticks (0 to 50).
2. Reach: the verdict's reach is 20 s (the past-time `placed` adds none); the horizon is 22 s.
3. Margin: a tilt of 17.04° gives (12 − 17.04)/12 = −0.42 for `upright`.
4. Clock lattice: the common repeat is 1/10 s = 50 physics ticks; lidar events fall at 1/20 + k/10 s, that is
   on physics tick 25 + 50·k, with zero lateness; no special case is needed for the phase.
5. Reductions: every signal is reduced from the 1/500 s physics clock to the 1/50 s requirement clock, ten
   physics ticks per requirement tick, by `last`, `max` or `min` as declared.

**The labelled traces (`synthetic`).** The specimen names four examples (two satisfied, two violated). Under
the admission rule an atom that no example can distinguish is vacuous, and a vacuous hard-limit atom is
refused; with six hard-limit atoms, two violated examples cannot distinguish them all. The resolved layer
therefore carries eleven traces, of which the specimen's four are S1, S2, V1 and V9. Each trace gives the
eight signals at the requirement clock for 1,101 ticks (0 to 22 s). The nominal trace N has tilt 3°, tray tilt
2°, tray acceleration 1 m/s², clearance 0.10 m, winding temperature 350 K, no spill, speed 0.6 m/s, and the
distance to the pass falling linearly to 0.10 m at the arrival tick, after which speed is 0.02 m/s.

| Trace | Difference from N | Label | Decided by |
|---|---|---|---|
| S1 | arrival at tick 500 (10 s) | satisfied | `placed` first true at tick 550 (11 s) |
| S2 | arrival at tick 950 (19 s) | satisfied | `placed` first true at tick 1,000 (20 s), the closed end of the window |
| V1 | arrival at tick 600; tilt 17.04° at tick 565 (11.3 s) | violated | `upright`, margin −0.42 |
| V2 | arrival at tick 600; tray tilt 8.2° at tick 300 | violated | `tray-level`, margin −0.025 |
| V3 | arrival at tick 600; tray acceleration 3.1 m/s² at tick 300 | violated | `gentle` |
| V4 | arrival at tick 600; clearance 0 m at tick 300 | violated | `clear` (strict comparison) |
| V5 | arrival at tick 600; winding temperature 383.5 K at tick 300 | violated | `cool`, margin −0.025 |
| V6 | arrival at tick 600; one spill event at tick 300 | violated | `no-spill`, margin −1 |
| V7 | never closer than 0.151 m | violated | `at-pass` never holds; no placement |
| V8 | arrives; speed never below 0.051 m/s | violated | `settled` never holds; no placement |
| V9 | arrival at tick 951 (19.02 s) | violated | `placed` first true at tick 1,001, one tick late |

The mutation check on this set: reversing any comparator changes the label of S1; deleting an atom changes
the label of the violated trace that atom alone decides (V1 to V8). Every atom is distinguished.

## Acceptance criteria

- [ ] The specimen equals the text above byte for byte (manifest SHA-256). The resolved file parses, converts
      to canonical JSON and back to the S-expression view as the identity, with one `TaskId`.
- [ ] M1 lane (library level, no registry resolution): the requirement section parses to the fixed grammar;
      the five arithmetic known answers hold; the evaluators agree with all eleven labels; the mutation check
      distinguishes all eight atoms.
- [ ] M3 lane: `frob task new` on the resolved file succeeds with the scene family, the split and the three
      hazard-registry draws resolved and echoed in `bound`; `frob task check` reports every row of the check
      table as passed, with evidence class Tested, tier `static`, determinism `exact`.
- [ ] With only the specimen's four examples, `task.check` reports five hard-limit atoms as vacuous
      (`tray-level`, `gentle`, `clear`, `cool`, `no-spill`) and refuses (`FROB-REF-539`, detail
      `vacuous-hard-limit`): the control that shows why eleven are needed.
- [ ] Each of the nine planted defects of the refusal table is refused with its code, exit 3, one envelope.
- [ ] `frob task verdict` on V1 returns `frob.verdict/1` with verdict `violated`, binding atom `upright`,
      margin −0.42 and time 11.3 s; on S2 `satisfied` decided at 20 s; on V9 `violated`.
- [ ] `TaskId` and canonical bytes are identical on two runs, in debug and release, on Linux and macOS (class
      D0); once blessed, the identity equals the map's entry for `t:kitchen-tray@88c1…`.

## Test plan

### Unit tests

`cargo test -p frob-spec --test golden_l3` (the real parser, unit checker and evaluators; traces labelled
`synthetic`):
- `l3_specimen_parses`; `l3_manifest_hash`; `l3_view_roundtrip_same_id`.
- `l3_nine_sections_present`; `l3_observation_width_98`; `l3_action_width_12`.
- `l3_window_ticks`: (0 s, 1 s) → 51 ticks; (0 s, 20 s) → 1,001 ticks; both bounds are multiples of 1/50 s.
- `l3_reach_and_horizon`: reach 20 s ≤ horizon 22 s; with horizon 19 s the reach check fails.
- `l3_margin_upright`: tilt 17.04° → −0.42 exactly as a decimal; tilt 12° → 0.
- `l3_lidar_on_lattice`: lidar event k is at physics tick 25 + 50·k for k = 0…9.
- `l3_reductions_declared`: eight signals, each with a reduction that its channel kind admits.
- `l3_examples_agree`: the three evaluators (the online block kernel, the deque monitor and the definitional
  evaluator) return the labels of the table for all eleven traces and agree bit for bit.
- `l3_boundary_pair`: S2 satisfied at tick 1,000; V9 violated with placement at tick 1,001.
- `l3_no_latch`: in S1, raising speed to 0.06 m/s at tick 560 makes `placed` false at tick 560.
- `l3_mutation_check`: sixteen mutants (eight reversals, eight deletions); every atom distinguished.
- `l3_four_examples_vacuous`: with S1, S2, V1, V9 only, `tray-level`, `gentle`, `clear`, `cool` and `no-spill`
  are reported vacuous under deletion.
- `l3_refusal_<code>` for the nine planted defects.

### End-to-end script and logging

`scripts/e2e/art_l3_ftask.sh` (header: `bead=ART-L3 milestone=M3 labels=golden,appendix-l,arena,
verb:task.new,verb:task.show,verb:task.check,verb:task.verdict seed=1 needs=cpu
plants=four-examples,late-label`). Steps: (1) `frob task new …/kitchen-tray.resolved.ftask.sexpr --key l3-new
--json`, exit 0, capture the `TaskId` and the `bound` block; (2) `frob task show` and assert the nine
sections; (3) `frob task check --json`, assert each row of the check table and the mutation table with
`e2e_assert_json`; (4) `frob task verdict` on V1, S2 and V9 from the trace files, assert verdict, binding atom,
margin and time (`e2e_assert_num` on −0.42 and on 11.3 s); (5) the nine refusals through `e2e_expect_refusal
<code> 3`; (6) the four-example control through `e2e_expect_refusal FROB-REF-539 3`; (7) `frob-golden check
--artifact L3`. Plants: `four-examples` removes seven traces and must fail step (3); `late-label` labels V9 as
satisfied and must be refused with 539. The log (`target/e2e-logs/art_l3_ftask/<utc>.ndjson`) records every
command with exit code and wall time, the evidence block, each assertion with observed and expected values and
units, each refusal with code and first repair, and the `summary`.

### Known answers and negative controls

- Known answers (exact): 50 and 1,000 ticks; 51 ticks in the closed one-second window; reach 20 s; margin
  −0.42; lidar on tick 25 + 50·k; the eleven labels. They have no number in the known-answer suite; the
  unit's coverage file lists them for registration.
- Counterexample kept alive: composing margins without scales. A clearance margin of 0.02 m and a thermal
  margin of 3 K compose to a raw minimum of 0.02, so clearance binds; in millimetres the raw minimum is 3, so
  the thermal margin binds; with scales the binding atom does not depend on the unit system.
- Negative controls: the nine planted defects; the two plants; an evaluator that reads the verdict from the
  first 20 s only must fail on S2 and V9.
- Metamorphic relation: rewriting every quantity in other units (degrees to turns, kelvin offsets kept) leaves
  every label, margin and binding atom unchanged.
- Determinism lanes: twice on one machine; debug and release; Linux and macOS; class D0.

## Considerations

- The specimen's check table says the mutation check passes with four examples; under the admission rule it
  cannot. The fixture resolves this by carrying eleven traces and keeping the four-example case as a control.
  The contradiction is recorded in the unit's coverage file.
- The keywords of eight sections are proposals. When the task-file bead fixes a spelling that differs, the
  specimen is re-entered once from the amended artifact bead (a reviewed change of the manifest hash).
- The traces are synthetic signal traces, not simulated runs. Recorded runs on the kitchen scene replace none
  of them: boundary cases such as S2 and V9 cannot be obtained from a simulator on demand.
- The hazard-registry draws (`wet-floor`, `swing-door`, `hot-liquid`) name dimensions whose ranges may be of
  provenance `assumed`; a report that cites this task for a realism claim must say so.

## References

Appendix L.3; §24.4 (the nine sections and the grammar), §24.5 (margins, clocks, online and offline
evaluation, the admission check), §24.7, §24.12, §20.4; INV-ARENA-01 (a verdict is a pure function of the
trace and the `TaskId`), INV-ARENA-02 (no requirement value from a truncated window); PO-ARENA-01, PO-ARENA-04;
GATE-ARENA-03 is measured by the requirement-language beads.
