---
key: "N29-CX"
title: "frob-conformance: Chronicle counterexample registry (CX-319 to CX-325), negative controls and late cases"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "chronicle", "gauntlet", "frob-conformance", "solid", "registry", "test"]
parent: "WP-13"
deps: ["BD-025", "STD-TEST"]
needs: ["Appendix M: the known-answer registry of the chapter (bead N29-KA, written by unit U29a) should append three rows for the late subsection on the solver's kinetic-energy correction, which states known answers that the appendix does not list (given below)"]
related: ["N29-REF", "N29-KA", "BD-090", "BD-091", "BD-092", "BD-205", "BD-206", "BD-207", "BD-209", "BD-211", "BD-264", "N29a-04", "N29a-06", "N29a-10", "N29a-12", "N29a-13", "N29a-22", "N29b-01", "N29b-03", "N29b-04", "N29b-06", "N29b-08", "N29b-10", "N29b-11", "N29b-13", "N29b-15", "N29b-16", "N29b-23", "BD-223", "N38b-05", "N38b-34"]
gates: []
plan_refs: ["Appendix N.26", "§29.19", "§29.5", "§29.6", "§29.7", "§29.8", "§29.9", "§29.11", "§29.22", "§38.11"]
---
## Background

A counterexample (CX) is the smallest hand-checkable case that breaks a tempting shortcut. The project's rule is
that every plausible shortcut ships with its counterexample, and that the case becomes a permanent negative
control: a planted defect that some gate must catch, because a gate that nothing can fail validates nothing. The
counterexample catalogue is data. `frob-conformance/registry/controls.toml` is the authority; it is seeded from
the catalogue (each entry becomes at least one control, attached to the gates of the rule it protects) and from
every "negative control" cell of the known-answer registry. A CX number is never reused and never renumbered; new
rows append. A control record has the form:

```toml
[[control]]
id        = "NC-…"                 # stable, never reused
seed      = "CX-319"               # the catalogue entry it came from, if any
gate      = ["GATE-CHRON-07"]      # every gate that must catch it
kind      = "mutation"             # mutation | planted-input | planted-design | planted-world | a-a
target    = "frob-chronicle::…"
defect    = "…"
expect    = "…"
last_seen = { caught = true, session = "…", date = "…" }
```

Five rules govern the registry: a gate with no control cannot record PASS; a control is run every time its gate
is run, and a run in which a control was not caught is a FAIL; a control is a defect, not a boundary (the control
is the build or input in which the limit was removed); a control that has never failed its gate in any recorded
run is suspect and is reviewed at each milestone exit; a false certificate, a false promotion or a withdrawn
result found in use becomes a permanent control.

This bead is the registry of the counterexamples that belong to the record subsystem (telemetry, replay,
evidence). It exists so that no counterexample is lost when the mechanisms are split across beads and units: each
row names the rule it protects, the gate that must catch its mutant, and the bead whose tests carry it.

## Scope

In: registry rows for CX-319 to CX-325 (the seven rows the catalogue numbers for this chapter); the fourteen
negative controls of the chapter's known-answer table, each attached to a gate and a carrier; the cases stated in
the chapter's text and in its late subsection that the catalogue has not numbered, entered as appended rows; one
numeric case supplied where the catalogue says none is given; the linkage test.

Out: implementing the mutants and fixtures (the carrier beads do that); the known-answer registry (N29-KA of
unit U29a); counterexamples of other chapters, cited where a Chronicle gate also catches them.

## Technical approach

**Row format (DESIGN).** `id`, `shortcut`, `case` (with its numbers), `protects` (invariant, proof obligation,
refusal or rule), `gates`, `kind` (`mutant`: a planted build defect; `fixture`: an input the system must refuse
or label; `arithmetic`: a host-computed fact a claim must not contradict), `owner_bead`, `expected_observable`.

**The seven numbered rows.**

| CX | Tempting shortcut | Smallest case that breaks it | Rule it protects | Gate; carrier |
|---|---|---|---|---|
| CX-319 | A mergeable quantile sketch is fine on the evidence path | A compactor of capacity two that keeps the lower of each sorted pair summarizes the stream 1, 2, 3 as {1 with weight 2, 3} and the stream 3, 2, 1 as {2 with weight 2, 1}: medians 1 and 2. Integer bin counts are 1, 1, 1 in both orders | Deterministic integer histograms; decision ADR-CHRON-03 (evidence aggregates are integers); INV-CHRON-06; FROB-REF-898 | GATE-CHRON-07; N29a-13, N29a-16, BD-207, N29b-15, N29b-16 |
| CX-320 | Bisect on the pointwise predicate "the states differ at tick t" | Two traces differ at ticks 1 and 5 and agree at ticks 2 to 4: a midpoint search probes tick 3, sees agreement, goes right and reports tick 5; the chained digest, whose value at each chunk covers the whole prefix, reports tick 1 | Never bisect on an undecided or pointwise predicate; PO-CHRON-05 | GATE-CHRON-06; BD-206, N29a-21, N29b-16 |
| CX-321 | Graft a warm start across a design edit in a fork | The contact warm start of a 60 mm wheel grafted onto a 75 mm design at a threshold lip is refused (FROB-REF-582), naming the contact memory as the field that forces a restart. A prefix is reused only when it is identical by construction, never because it is "close" | The five fork classes; design parameters re-run from the authored initial condition | GATE-CHRON-05; BD-205 |
| CX-322 | Downsample an impulse channel by averaging | An impulse train averaged over a window loses the momentum transfer that the sum conserves. The catalogue gives no numbers; numeric case supplied here: ten impulses of 0.5 N·s in one second on a 1 kHz impulse-per-tick channel. Sum over the window: 5 N·s (the momentum transferred). Mean over the window: 0.005 N·s, which a consumer reads as the window's impulse and is wrong by a factor of 1,000. Sampling every 100th tick sees an impulse only by coincidence | Channel kinds declare their downsampling rule (impulse: sum over the window; interval mean: duration-weighted mean; cumulative counter: last value; event: all, or a count; state: a sample or an envelope) | The round-trip and downsampling tests of BD-090; N29b-01 for the interval-mean and counter cases |
| CX-323 | Order paginated telemetry by timestamp | During concurrent appends a query ordered by timestamp alone repeats or skips equal-time samples; pinned to a committed frontier, with the total order (tick, world, episode, stream, sample identifier), each of 10⁴ rows appears exactly once | Pagination pinned to a committed frontier; INV-CHRON-04; FROB-REF-896 | GATE-CHRON-04; N29a-04, N29a-06, BD-211, N29b-11 |
| CX-324 | Choose which fast-tier episodes to audit after seeing them | Audit bits chosen after seeing fast-tier failures, without recorded probabilities, bias the tier correction; with pᵢ recorded before the draw the estimator is unbiased even when the prediction is off by a constant. Numeric case added by the carrier: four addresses with referee outcomes (1, 1, 0, 0), prediction 0.5, claimed p = 0.5; auditing exactly the two failures gives Z = (0.5, 0.5, −0.5, −0.5) and an estimate of 0 against the truth 0.5 | The tier-correction ledger; INV-CHRON-13; PO-CHRON-02; FROB-REF-811 | GATE-CHRON-09; BD-264, N29b-08 |
| CX-325 | Sample traced worlds by arrival order | Arrival-order reservoir sampling gives different traced sets under different slot assignments and shard counts; sampling keyed by world address (the k smallest address hashes) gives the same set under 100 of each | Reservoir by address; decision ADR-CHRON-04; INV-CHRON-05; PO-CHRON-03 | GATE-CHRON-07; N29a-12, N29a-16, BD-207, N29b-16 |

**The chapter's negative controls (one per known-answer subject), with carriers.**

| Subject | Negative control that must fail | Gate | Carrier |
|---|---|---|---|
| Energy ledger | A sign error seeded at one port is localized to that port | GATE-CHRON-03 | BD-092 |
| Energy per task | A report that prints only the conditional mean (10 J and 30 J against 12 J and 12 J) | GATE-CHRON-03 (recorded check) | BD-092, N29b-01 |
| Created energy | The audit switched off: the bouncing policy wins the comparison | GATE-CHRON-08 | N29a-22, N29b-04, N29b-06 |
| Cone excess | A circular-cone run flagged | GATE-CHRON-08 | N29a-22, N29b-03 |
| Chunk protocol | A late lease that writes; overlapping ranges that commit; a reset at tick zero that collides | GATE-CHRON-04 | BD-091, N29a-06, BD-210, N29a-24, N29b-11 |
| Pagination | A query ordered by timestamp alone repeats or skips equal-time samples | GATE-CHRON-04 | N29a-04, N29a-06, BD-211, N29b-11 |
| Sampling | Arrival-order reservoir sampling gives different sets | GATE-CHRON-07 | N29a-12, N29a-16 |
| Histogram | An order-dependent sketch on the evidence path | GATE-CHRON-07 | N29a-13, N29a-16 |
| Downsampling | The same impulse train averaged | (format tests) | BD-090 |
| Fork | The 60 mm warm start grafted onto the 75 mm wheel | GATE-CHRON-05 | BD-205 |
| First divergence | Midpoint search on a pointwise predicate reports tick 5 | GATE-CHRON-06 | BD-206, N29a-21 |
| Teleport | None stated by the plan; proposed here: a comparison of whole-step digests only, which names the step and not the stage | GATE-CHRON-02 | BD-204 |
| Tier correction | Audit bits chosen after seeing fast-tier failures, without recorded probabilities | GATE-CHRON-09 | BD-264, N29b-08 |
| Artifact graph | A claim printed after its evidence was collected | GATE-CHRON-10 | BD-095, N29a-10, N29b-10 |

**Rows appended from the chapter's text and its late subsection.** Labels C1 to C6 are provisional; the registry
assigns the next free CX numbers on append.

| Label | Tempting shortcut | Smallest case that breaks it | Rule it protects | Gate; carrier |
|---|---|---|---|---|
| C1 | The kinetic energy a contact solve removes is contact dissipation | A 1 kg block rests on a frictionless stationary support under g = 8 m/s². A step that applies forces, then contact, then drift gives the block a free velocity g·h and removes it with the support impulse: ½·m·(g·h)² per step, 1 J/s at 32 Hz and 0.5 J/s at 64 Hz, with zero displacement and no heat. Booked as dissipation it warms the tyres of a parked robot at a rate set by the step size | The line `algorithmic exchange`; only constitutive dissipation feeds a thermal or wear state | GATE-CHRON-14; N29b-20 to N29b-24 |
| C2 | Energy per task is the mean over successful episodes | A design that succeeds once for 10 J and fails once for 20 J reports 10 J; a rival with two 12 J successes reports 12 J; the first figure ranks them wrongly. Total over all attempts divided by completions: 30 J against 12 J | Both figures are always printed | BD-092, N29b-01 |
| C3 | A stiff penalty contact is fine if the step is under the stability limit | A body bouncing on a stiff penalty contact at a step near the stability limit gains height. One-step arithmetic: with ω = √(k/m) and ω·h = 2 (a 1 kg body, k = 10⁶ N/m, h = 2 ms), a step that starts with penetration v·h at approach speed v ends at −v + (ω·h)²·v = 3·v upward | The flags `energy-created` and `contact-active` | GATE-CHRON-08; N29a-22, N29b-02, N29b-03 |
| C4 | Net bus energy is enough | +10 W for 0.5 s then −10 W for 0.5 s: draw 5 J, regeneration 5 J, net 0; a split computed from the one-second mean reports 0 and 0 | Bus power integrated with its positive and negative parts separately; regeneration never netted silently | BD-092, N29b-01 |
| C5 | A numerical fault can be counted as a failed episode, or dropped | A candidate with 2 successes in 2 valid trials and 98 unresolved has a complete-case rate of 100% and a true rate anywhere in [2%, 100%] (case shared with the dynamics chapter's CX-150) | A numerical outcome leaves the verdict unknown; it is not a physical failure and is not dropped from the denominator | GATE-CHRON-08; BD-094, N29b-05 |
| C6 | Write the verdict, then pin its evidence | A collection cycle that runs between the two steps removes a chunk the verdict was computed from; the verdict cites a tombstone from the moment it exists | Pins are taken in the same transaction that writes the verdict; INV-CHRON-12 | GATE-CHRON-12; BD-209, N29b-13, N29b-11 |

Cases of other chapters that Chronicle's gates also catch are cited, not re-registered: CX-147 (position-level
penetration recovery: 0.5 m/s and 0.125 J per event at m = 1 kg, d = 1 mm, h = 2 ms) and CX-148 (a pyramid
grants √2·μ along its diagonals) are controls of GATE-CHRON-08 through the dynamics unit's plants; CX-346
(pagination by offset: rows at t = 1, 2, 3, 4 with page size 2; a late commit at 1.5 makes offset 2 return
{2, 3}) and KA-765's partition case (ranges [0, 100) and [50, 150) after a shard split) are controls of
GATE-CHRON-04.

**Known answers of the late subsection, for the known-answer registry to append** (the appendix has no rows):
resting block: algorithmic exchange 1 J/s at 32 Hz and 0.5 J/s at 64 Hz, constitutive dissipation 0, heat 0
(exact); the same block sliding at 1 m/s on μ = 0.5: 4 W of friction heat at both rates (0.1%); a plastic drop
arriving at 2 m/s: 2 J of impact dissipation, booked once at the located event (10⁻⁹ J). Negative control for
all three: a mutant that feeds algorithmic exchange into the film temperature, whose temperature rise differs
between 32 and 64 Hz by a factor of 2. Tier: G1 on the referee and on `strict`.

**Linkage test.** `registry_chronicle_complete`: every row above has at least one control record; every control
names an existing gate and an owner bead; every Chronicle gate (GATE-CHRON-01 to -14) has at least one control;
no control's `expect` is empty. A control whose carrier bead is closed and whose `last_seen.caught` is false
fails the test.

## Acceptance criteria

- [ ] Seven numbered rows, fourteen subject controls and six appended rows are present in the control registry
      with gate and owner; the appended rows have received CX numbers from the registry's owner or carry their
      provisional labels with a filed request.
- [ ] `registry_chronicle_complete` passes, and fails when a row is planted without a gate or a gate without a
      control.
- [ ] Each of GATE-CHRON-01 to -14 lists at least one control; GATE-CHRON-02 has the proposed control or a
      recorded reason why none can be planted.
- [ ] The numeric case for CX-322 is filed as an erratum to the catalogue (which says no numeric case is given).
- [ ] The three known answers of the late subsection are filed with the known-answer registry's bead.
- [ ] At each milestone exit, the list of Chronicle controls that have never been caught in a recorded run is
      empty or explained.

## Test plan

### Unit tests

- `registry_chronicle_complete` and its two planted failures.
- Arithmetic checks, on the host, of every number in the tables: `cx319_compactor` (medians 1 and 2; counts 1,
  1, 1); `cx320_midpoint_reports_5`; `cx322_sum_5_mean_0_005`; `cx324_chosen_audit_bias` (0 against 0.5);
  `c1_resting_block_rates` (1 and 0.5 J/s); `c2_two_figures` (10, 30, 12, 12); `c3_one_step_gain` (3);
  `c4_split` (5, 5, 0); `c5_bounds` ([0.02, 1.00]).

### End-to-end script and logging

`scripts/e2e/chronicle_controls.sh` iterates over the registry's Chronicle controls whose carriers exist at the
current milestone, runs each control's gate script in its control mode (the carrier's script exposes
`--control <id>`), and asserts `caught`. NDJSON lines per the shared helper: control identifier, seed row, gate,
command, exit code, wall time, `caught` as a boolean with the observable that showed it; controls whose carrier
is not built are `not-run` with the milestone due; a `summary` line with caught of run and the gates covered.

### Known answers and negative controls

The registry is the list of negative controls; its own negative controls are the two planted registry defects of
the linkage test and a control whose mutant was silently removed from the build (the script must then report
`not caught`, which fails the gate, rather than `not-run`).

## Considerations

- Carriers are spread over several drafting units (the first half of the chapter, the dynamics, surface and
  counterfactual-debugging chapters); this registry is the one place that lists all of them for Chronicle's gates.
- The plan states no negative control for teleport; a gate without one cannot record PASS, so the proposed
  control (or a better one from the carrier) is required before GATE-CHRON-02 is recorded.
- Appended rows keep their provisional labels until numbered; nothing may cite a provisional label outside this
  unit's beads.
- A control is a defect, not a boundary: for each refusal code of the chapter the control is the build in which
  the refusal was removed (listed in N29-REF), not the request that triggers the refusal.

## References

Plan Appendix N.26 (CX-319 to CX-325), §29.19 (known answers and negative controls), §29.5 to §29.9, §29.11,
§29.22, §38.11 (the control registry and its rules); INV-CHRON-04, -05, -06, -12, -13; PO-CHRON-02, -03, -05.
