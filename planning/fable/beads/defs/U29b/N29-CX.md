---
key: "N29-CX"
title: "frob-conformance: Chronicle counterexample registry (CX-319 to CX-325), negative controls and late cases"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "chronicle", "gauntlet", "frob-conformance", "solid", "registry", "test"]
parent: "WP-13"
deps: ["BD-025", "N38a-40", "N38b-05", "STD-E2E", "STD-TEST"]
needs: []
related: ["N29-KA", "N29-REF", "N29a-04", "N29a-06", "N29a-10", "N29a-12", "N29a-13", "N29a-17", "N29a-19", "N29a-22", "N29b-01", "N29b-02", "N29b-03", "N29b-04", "N29b-06", "N29b-08", "N29b-10", "N29b-11", "N29b-13", "N29b-15", "N29b-16", "N29b-23", "N38b-04", "N38b-34", "BD-096", "BD-203"]
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

Division of labour (the same for every chapter, fixed by N38b-05): this bead writes the chapter's entries of the
counterexample catalogue `crates/frob-conformance/registry/counterexamples.toml` in the catalogue's record schema,
and never a row of `controls.toml`; the seeding tool of N38b-05 derives the controls from the catalogue entries
and from the stated control cells of the known-answer registry, allocates their `NC-` identifiers once and never
rewrites them. The control record above is therefore what the seeding produces, not what this bead types.

## Scope

In: catalogue entries for CX-319 to CX-325 (the seven entries the catalogue numbers for this chapter), each with
its control kind, plant and rule identifiers; the map of the fourteen negative controls of the chapter's
known-answer table to their gates and carriers (the controls themselves are seeded from N29-KA's control cells);
the cases stated in the chapter's text and in its late subsection that the catalogue has not numbered, entered as
appended entries with numbers from an allocated block; one numeric case supplied where the catalogue says none is
given; the list of controls other beads register for the Chronicle gates no entry here reaches; the linkage test
and the chapter's control script.

Out: implementing the mutants and fixtures (the carrier beads do that); the catalogue schema, the seeding tool,
the seed map and the join (N38b-05); the number allocator (N38a-40); the control runner and the five registry
rules (N38b-04); the known-answer registry (N29-KA of unit U29a); counterexamples of other chapters, cited where a
Chronicle gate also catches them.

## Technical approach

**Entry format (the catalogue record schema of N38b-05).** `id` (CX number), `shortcut`, `case` (with its
numbers), `numeric` (true, or false for "no numeric case is given"), `rule` (text), `rule_ids` (the gates,
invariants, proof obligations, decisions and refusal codes of the "Rule it protects" and "Gate" columns; the gates
among them are the gates the control must fail), `defined_in`, `same_as`, `label` (DESIGN, or FACT with source),
`control_kind` (`mutation`: a test build in which the shortcut replaces the rule; `planted-input`: a request that
asks for the unsound conclusion and must be refused; `planted-design`; `planted-world`: a world that separates the
shortcut from the rule; `a-a`: a scripted wrong procedure), `plant` (for a mutation, the build flag that puts the
shortcut into the real component, never a mock; for the other kinds, the fixture request or world) and `slug`
(the provisional tag of an appended entry). The carrier bead (the bead whose tests implement the plant) is kept in
the entry's `rule` text and in this bead's tables; a mutation flag is test-only and cannot be enabled in a release
build (compile-time feature, checked by the release rubric).

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

Control kind and plant of each numbered entry (DESIGN; the plant names are the carriers' flags, which the carrier
bead may rename once, in the same commit as this entry):

| CX | `numeric` | `control_kind` | `plant` |
|---|---|---|---|
| CX-319 | true | mutation | `mutant-chron-order-sketch`: the run-level quantile computed by the capacity-two compactor instead of the integer histogram |
| CX-320 | true | mutation | `mutant-chron-midpoint-pointwise`: the divergence localizer bisects on "tick k differs" |
| CX-321 | true | planted-input | `run.fork` request grafting the 60 mm wheel's contact warm start onto the 75 mm design at the threshold lip; refused with FROB-REF-582 naming the contact memory |
| CX-322 | true (numeric case supplied below; was false in the catalogue) | mutation | `mutant-chron-impulse-mean`: the impulse channel downsampled by the window mean |
| CX-323 | true | mutation | `mutant-chron-page-by-time`: pagination ordered by timestamp alone, no pin |
| CX-324 | true | a-a | scripted auditor that chooses audit bits after seeing fast-tier failures and records no probabilities; must be refused (FROB-REF-811) or caught by the bias check |
| CX-325 | true | mutation | `mutant-chron-arrival-reservoir`: classic reservoir sampling by arrival order |

Each `plant` is written as one plant record of N38b-05's form, whose `id` is `<planting bead>:<name>`; the
seeding derives a mutation's `feature` (and another kind's `fixture`) from the name after the colon, so the name
is the flag above and the control runner builds `--features gauntlet-mutations,frob-chronicle/<flag>`. The
planting bead is the carrier that holds the rule: CX-319 `N29a-13:mutant-chron-order-sketch`; CX-320
`BD-206:mutant-chron-midpoint-pointwise`; CX-321 `BD-205:cx-321` (`planted-input`, fixture `cx-321`); CX-322
`BD-090:mutant-chron-impulse-mean`; CX-323 `N29a-04:mutant-chron-page-by-time`; CX-324 `BD-264:cx-324` (`a-a`,
the scripted auditor's fixture `cx-324`); CX-325 `N29a-12:mutant-chron-arrival-reservoir`. Each carrier declares
its feature in `frob-chronicle` as implying `conformance` (N38b-04's mutation guard).

**The chapter's negative controls (one per known-answer subject), with carriers.** These are the stated control
cells of KA-707 to KA-720 (N29-KA). The seeding tool turns each into a control (step 3 of N38b-05: it reuses the
`NC-` of a CX-seeded control with the same target, else allocates one with `seed = "KA-nnn"`). This table is the
gate and carrier map the seeding and the linkage test check against; this bead types none of these controls.

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
| Teleport | None stated by the plan. The control registered with KA-718 and carried by BD-204: a localizer that compares whole-step digests only and so cannot see the stage; it names the last state-writing stage (stage 9) and must be wrong on every trial planted in another stage | GATE-CHRON-02 | BD-204 |
| Tier correction | Audit bits chosen after seeing fast-tier failures, without recorded probabilities | GATE-CHRON-09 | BD-264, N29b-08 |
| Artifact graph | A claim printed after its evidence was collected | GATE-CHRON-10 | BD-095, N29a-10, N29b-10 |

**Entries appended from the chapter's text and its late subsection.** Their numbers come from the allocation
ledger `crates/frob-conformance/registry/allocations.toml` (schema `frob.gauntlet.alloc/1`, N38a-40), never from
"the next free number" at commit time: this bead files one `[[block]]` request with `registry = "cx"`,
`count = 6`, `owner = "N29-CX"`, `section = "§29.9, §29.10, §29.16, §29.22"`; the allocator fills `first` and
`last`, and the six tags below are mapped to the block's numbers in table order in one commit. Until then the
tag is the entry's name; afterwards it stays in the entry's `slug`. A tag still used in a registry, a test name or
a derivation file at this bead's milestone (M1) fails the ledger check `provisional-tag-past-due`. The labels C1
to C6 are this bead's short names for the tags and are not used outside this unit's beads.

| Label (tag) | Tempting shortcut | Smallest case that breaks it | Rule it protects | Gate; carrier |
|---|---|---|---|---|
| C1 (`chron-exchange-heat`) | The kinetic energy a contact solve removes is contact dissipation | A 1 kg block rests on a frictionless stationary support under g = 8 m/s². A step that applies forces, then contact, then drift gives the block a free velocity g·h and removes it with the support impulse: ½·m·(g·h)² per step, 1 J/s at 32 Hz and 0.5 J/s at 64 Hz, with zero displacement and no heat. Booked as dissipation it warms the tyres of a parked robot at a rate set by the step size | The line `algorithmic exchange`; only constitutive dissipation feeds a thermal or wear state | GATE-CHRON-14; N29b-20 to N29b-24 |
| C2 (`chron-task-energy`) | Energy per task is the mean over successful episodes | A design that succeeds once for 10 J and fails once for 20 J reports 10 J; a rival with two 12 J successes reports 12 J; the first figure ranks them wrongly. Total over all attempts divided by completions: 30 J against 12 J | Both figures are always printed | GATE-CHRON-03 (recorded check); BD-092, N29b-01 |
| C3 (`chron-penalty-bounce`) | A stiff penalty contact is fine if the step is under the stability limit | A body bouncing on a stiff penalty contact at a step near the stability limit gains height. One-step arithmetic: with ω = √(k/m) and ω·h = 2 (a 1 kg body, k = 10⁶ N/m, h = 2 ms), a step that starts with penetration v·h at approach speed v ends at −v + (ω·h)²·v = 3·v upward | The flags `energy-created` and `contact-active` | GATE-CHRON-08; N29a-22, N29b-02, N29b-03 |
| C4 (`chron-net-bus`) | Net bus energy is enough | +10 W for 0.5 s then −10 W for 0.5 s: draw 5 J, regeneration 5 J, net 0; a split computed from the one-second mean reports 0 and 0 | Bus power integrated with its positive and negative parts separately; regeneration never netted silently | GATE-CHRON-03 (recorded check); BD-092, N29b-01 |
| C5 (`chron-numerical-failure`) | A numerical fault can be counted as a failed episode, or dropped | A candidate with 2 successes in 2 valid trials and 98 unresolved has a complete-case rate of 100% and a true rate anywhere in [2%, 100%] (case shared with the dynamics chapter's CX-150) | A numerical outcome leaves the verdict unknown; it is not a physical failure and is not dropped from the denominator | GATE-CHRON-08; BD-094, N29b-05 |
| C6 (`chron-pin-after-verdict`) | Write the verdict, then pin its evidence | A collection cycle that runs between the two steps removes a chunk the verdict was computed from; the verdict cites a tombstone from the moment it exists | Pins are taken in the same transaction that writes the verdict; INV-CHRON-12 | GATE-CHRON-12; BD-209, N29b-13, N29b-11 |

Control kind and plant of each appended entry (DESIGN): C1 `mutation`, `mutant-chron-exchange-heat` (the
algorithmic-exchange line bound into the thermal test node standing in for the film temperature; N29b-22,
N29b-23); C2 `mutation`, `mutant-chron-conditional-mean-only` (the per-task report prints the mean over
completions alone); C3 `planted-world`, the stiff-penalty bounce world at ω·h = 2 that must raise both flags
(N29a-22's fixture); C4 `mutation`, `mutant-chron-net-bus` (bus power integrated as one signed sum); C5
`mutation`, `mutant-chron-numerical-as-failure` (a numerical outcome scored as a physical failure, and a second
flag that drops it from the denominator), `same_as = "CX-150"` (one shortcut, two rules, two controls);
C6 `mutation`, `mutant-chron-pin-after-verdict` (the verdict and its pins written in two transactions, with the
collection cycle scheduled between them by the simulation harness). Plant `id`s, by the same rule: C1
`N29b-23:mutant-chron-exchange-heat`; C2 `BD-092:mutant-chron-conditional-mean-only`; C3
`N29a-22:chron-penalty-bounce` (fixture); C4 `BD-092:mutant-chron-net-bus`; C5 two plants,
`BD-094:mutant-chron-numerical-as-failure` and `BD-094:mutant-chron-numerical-dropped` (the second flag); C6
`BD-209:mutant-chron-pin-after-verdict`.

**Controls of the Chronicle gates that no entry of this bead reaches** (registered by the bead named; listed
here so the linkage test can check them, not re-registered): GATE-CHRON-01: a digest that folds padding lanes,
a chain that omits x_{t−1}, a snapshot copy that omits warm starts or packet queues, an address that includes the
slot, a batch-wide normalizer (BD-203, N29a-19). GATE-CHRON-11: the planted lost-chunk bug, a two-statement
idempotency binding, range-keyed chunks, a fake device whose outcome depends on arrival order (BD-096; device rows
N29a-24). GATE-CHRON-13: a missing disclosure field, the ratio of means, the enqueue-only timer, a "bare"
configuration that disables the commit fence (N29a-17). GATE-CHRON-12 has C6 here and the controls of BD-209.

Cases of other chapters that Chronicle's gates also catch are cited, not re-registered: CX-147 (position-level
penetration recovery: 0.5 m/s and 0.125 J per event at m = 1 kg, d = 1 mm, h = 2 ms) and CX-148 (a pyramid
grants √2·μ along its diagonals) are controls of GATE-CHRON-08 through the dynamics unit's plants; CX-346
(pagination by offset: rows at t = 1, 2, 3, 4 with page size 2; a late commit at 1.5 makes offset 2 return
{2, 3}) and KA-765's partition case (ranges [0, 100) and [50, 150) after a shard split) are controls of
GATE-CHRON-04.

**Known answers of the late subsection** (the appendix has no rows; they are N29-KA's rows K1 to K3, which
C1's `rule` text cites, and are not entered here): resting block: algorithmic exchange 1 J/s at 32 Hz and 0.5 J/s at 64 Hz, constitutive dissipation 0, heat 0
(exact); the same block sliding at 1 m/s on μ = 0.5: 4 W of friction heat at both rates (0.1%); a plastic drop
arriving at 2 m/s: 2 J of impact dissipation, booked once at the located event (10⁻⁹ J). Negative control for
all three: a mutant that feeds algorithmic exchange into the film temperature, whose temperature rise differs
between 32 and 64 Hz by a factor of 2. Tier: G1 on the referee and on `strict`.

**Linkage test.** `registry_chronicle_complete` (in `crates/frob-conformance/tests/chronicle_counterexamples.rs`,
run after the seeding tool): every plant of every entry of this bead has exactly one control in `controls.toml`
whose `seed` names the entry (one per plant, N38b-05: fourteen for the thirteen entries, C5 having two); every stated control cell of KA-707 to KA-720 has a control whose gates include the gate of the
subject table above; every control names an existing gate and an owner bead; every Chronicle gate (GATE-CHRON-01
to -14) has at least one control, except a gate whose only controls are those a bead registers itself (the list
"Controls of the Chronicle gates that no entry of this bead reaches" above) while that bead's milestone is later
than the build's `FROB_MILESTONE`: the test prints it as `pending: <bead> (<milestone>)` (GATE-CHRON-01 and -13
at M1, whose controls BD-203, N29a-19 and N29a-17 register at M2) and fails once that milestone is reached; no
control's `expect` is empty; no entry of this bead appears in the seeding
tool's findings file (a rule that resolves to no gate). A control whose carrier bead is closed and whose
`last_seen.caught` is false fails the test.

## Acceptance criteria

- [ ] Seven numbered entries (CX-319 to CX-325) and six appended entries exist in `counterexamples.toml` in the
      catalogue record schema, each with `control_kind`, `plant` and the gates it must fail among its
      `rule_ids`; `registry::alloc::validate` passes.
- [ ] The six appended entries carry numbers from a block issued to N29-CX in `allocations.toml`, in table
      order, with their tags in `slug`; no tag remains outside `slug` at M1 (`provisional-tag-past-due` clean).
- [ ] After the seeding tool of N38b-05 is re-run, each plant of the thirteen entries has exactly one control
      whose `seed` names its entry (fourteen controls; C5 has two plants), each of the fourteen stated KA control cells of the chapter has a control on the gate of
      the subject table, the tool's second run changes no byte, and the join of N38b-05 passes.
- [ ] `registry_chronicle_complete` passes, and fails when an entry is planted without a gate or a gate without a
      control.
- [ ] Each of GATE-CHRON-01 to -14 lists at least one control, or is `pending` with the bead that registers its
      controls and that bead's milestone, later than the build's; GATE-CHRON-02 has the stage-9 localizer
      control (KA-718, BD-204) or a recorded reason why none can be planted.
- [ ] CX-322 is entered with `numeric = true` and the numeric case below, so it leaves the seeding tool's list of
      obligations (it was printed with no numeric case).
- [ ] Each entry's plant is a plant record whose `id` is `<planting bead>:<name>` as listed above, and the
      seeded control's `feature` (or `fixture`) equals the name after the colon.
- [ ] For every entry whose carrier exists at the time, the gate run with the plant enabled fails and with it
      disabled passes; an entry whose carrier does not exist yet is `pending` with the bead and milestone due.
- [ ] At each milestone exit, the list of Chronicle controls that have never been caught in a recorded run is
      empty or explained.

## Test plan

### Unit tests

- `registry_chronicle_complete` and its two planted failures (an entry without a gate; a gate without a control).
- `chronicle_entries_schema`: the thirteen entries load with the catalogue schema; `control_kind` is one of the
  five kinds; `plant` is non-empty; C5's `same_as` is CX-150; a planted entry with `control_kind = "mutant"` (the
  retired spelling) is rejected.
- `chronicle_tags_mapped`: with a synthetic block issued to N29-CX the six tags map to its numbers in table order;
  a tag left in a test name fails `provisional-tag-past-due`.
- Arithmetic checks, on the host, of every number in the tables: `cx319_compactor` (medians 1 and 2; counts 1,
  1, 1); `cx320_midpoint_reports_5`; `cx322_sum_5_mean_0_005` (5 N·s and 0.005 N·s, ratio 1,000);
  `cx324_chosen_audit_bias` (Z = (0.5, 0.5, −0.5, −0.5), estimate 0 against 0.5); `c1_resting_block_rates`
  (½·1·(8/32)²·32 = 1 and ½·1·(8/64)²·64 = 0.5 J/s); `c2_two_figures` (10, 30, 12, 12); `c3_one_step_gain`
  (ω·h = 1000·0.002 = 2; −1 + 4 = 3); `c4_split` (5, 5, 0); `c5_bounds` ([0.02, 1.00]); `cx147_cited`
  (0.001/0.002 = 0.5 m/s; ½·1·0.25 = 0.125 J); `cx346_cited` (offset 2 after the late row returns {2, 3}).

### End-to-end script and logging

`scripts/e2e/chronicle_controls.sh` (helper of STD-E2E) first re-runs the seeding tool and the join for this
chapter's entries (`FROB_TEST_SEED=0 cargo test -p frob-conformance --test seed_controls -- --ignored seed`, then
`cargo test -p frob-conformance --test seed_join`) and asserts a byte-identical second run; then, for each
Chronicle gate with a part due at the build's milestone, runs its tier with the controls (`FROB_MILESTONE=<Mn>
FROB_TEST_SEED=<seed> cargo test -p frob-conformance --test <tier> --features gauntlet-mutations -- <gate>`, the
tier being the one of the gate's entries): the gate's entries pass with no plant, and the control runner of
N38b-04 rebuilds once per mutation control with `<crate>/<feature>` and applies the other kinds, writing one line
per control into the receipt (`caught`, `missed`, `stale` or `not-yet-due`). The script reads those lines from
the receipt and asserts `caught` for every control that is due. It records no gate. NDJSON lines per the shared
helper (`ts`, `script`, `bead`, `step`, `event`, the exact command line, exit code, wall time, the envelope's
`evidence` block): the CX number or KA seed, the control's `NC-` identifier, the plant, the gate, the outcome,
and for `caught` the measured value that crossed the threshold under the plant (value, unit, threshold);
`not-yet-due` and `pending` controls with the bead and milestone due; on failure, the first differing value and
the paths to reproduce. A `summary` line gives controls caught of run, pending, failed and the gates covered.

### Known answers and negative controls

The registry is the list of negative controls; its own negative controls are the two planted registry defects of
the linkage test and a control whose mutant branch was silently removed from its feature (the mutant build then
passes every entry, the control runner reports `missed` and the script fails, rather than passing it over).

## Considerations

- Carriers are spread over several drafting units (the first half of the chapter, the dynamics, surface and
  counterfactual-debugging chapters); this registry is the one place that lists all of them for Chronicle's gates.
- The plan states no negative control for teleport; a gate without one cannot record PASS, so the proposed
  control (or a better one from the carrier) is required before GATE-CHRON-02 is recorded.
- Appended entries keep their tags until the block is issued; nothing may cite a tag outside this unit's beads,
  and after mapping a tag lives only in `slug`.
- Risk: a carrier renames its mutation flag. The flag name is the `plant` field; a rename is one commit that
  changes the carrier's flag and this entry together, and the seeding tool keeps the `NC-` number (it is keyed
  by the CX number, not by the plant).
- A control is a defect, not a boundary: for each refusal code of the chapter the control is the build in which
  the refusal was removed (listed in N29-REF), not the request that triggers the refusal.

## References

Plan Appendix N.26 (CX-319 to CX-325), §29.19 (known answers and negative controls), §29.5 to §29.9, §29.11,
§29.22, §38.11 (the control registry and its rules); INV-CHRON-04, -05, -06, -12, -13; PO-CHRON-02, -03, -05.
