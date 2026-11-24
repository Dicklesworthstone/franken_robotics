---
key: "N00-CX"
title: "frob-conformance: counterexample catalogue entries CX-001 to CX-015 seeded as negative controls"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "gauntlet", "frob-conformance", "solid", "registry", "test"]
parent: "WP-34"
deps: ["BD-025", "N38b-05", "STD-E2E"]
needs: []
related: ["N00-KA", "N22-CX", "N22c-27", "N38a-10", "N39-CX", "NS-22", "STD-TEST"]
gates: ["GATE-GAUNT-01"]
plan_refs: ["Appendix N.1", "Appendix N.2", "Appendix N.3", "Appendix N.4", "Appendix N.5", "Appendix N.35", "§0.7", "§38.11"]
---
## Background

The program's rule is that a plausible shortcut ships with the smallest hand-checkable case that breaks it, and
that every gate has a negative control it must catch: a gate that nothing can fail validates nothing. The
counterexample catalogue holds 384 such cases at plan time, numbered CX-001 onward, append-only. Each entry seeds
at least one record in the negative-control registry: the shortcut is planted as a mutation or as an input, and
the gates of the rule it protects must catch it. This bead carries the first fifteen entries, those stated by
document control (2), the ten-fold thesis (7), the bets (4) and the claim product (2). The audit chapter and the
North-Star scenarios state none.

Control kinds (DESIGN): `mutation` (a test build in which the shortcut replaces the rule), `planted-input` (a
request that asks for the unsound conclusion and must be refused), `planted-design` (a design the shortcut would
accept or discard wrongly), `planted-world` (a world or world pair separating shortcut from rule), `a-a` or a
scripted adversary (a wrong experimental procedure).

## Scope

In: fifteen catalogue records and the control records derived from them, each attached to the gates of the rule
it protects; for CX-012, the forward reference to its numeric instance and its entry as an obligation, which
the seeding makes runnable when that instance loads. Out: the catalogue's record schema, its loader and the
seeding tool that writes `controls.toml` (N38b-05); the mechanisms that must catch them (named per row); the numeric
instance of the pre-slip aliasing fixture itself, which is the Reflex catalogue entry `reflex-untimely` of
N22-CX (M1; `same_as = "CX-012"`, plant `list_untimely_assumption_as_monitored`, gate GATE-SHIELD-06), whose
case is the fixture of N22c-27 (M4), with `reflex-intersample` (plant `drop_intersample_term`) as a second
instance on the same fixture. This bead cannot depend on either (both are later milestones); it names them, and
the registry closes the obligation when they land.

## Technical approach

Catalogue records go to `crates/frob-conformance/registry/counterexamples.toml` in the catalogue schema of
N38b-05 (`id`, `shortcut`, `case`, `rule`, `rule_ids`, `control_kind`, `plant`, …), where `rule_ids` holds every
gate, bet, kill-gate and refusal identifier of the row's last column. Every entry has `plan_case = "numbers"`
(none is among the catalogue's seventeen word-only entries, which the loader counts by `plan_case = "words"`)
and `numeric = true`, except CX-012 (below). From each entry N38b-05's seeding tool derives one or more
`[[control]]` records in `controls.toml` (this bead writes none by hand) with fields `id` (stable `NC-nnnn`),
`seed` (the CX number), `gate` (every gate that must catch it), `kind`, `target`, `defect`, `expect`,
`last_seen`. CX-003 and CX-008 are also the ten-fold
harness's entries (N39-CX reuses them and does not restate them): GATE-TENFOLD-03 (the harness reproduces every
known answer and catches every negative control, M3, repeated at M7) must catch both, so it is among their
`rule_ids` and in the `gate` list of their controls.

**CX-012, an obligation until its instance loads** (obligation (b) of N38b-05, in the catalogue's own fields;
DESIGN, correcting an earlier draft of this bead that gave the record fields `status = "owed"` and `instance`,
which the schema does not have and its loader refuses as unknown): the entry is written with `numeric = false`
and an empty `case_supplied_by`, and names the plant its instance must use,
`plant = "N22c-27:list_untimely_assumption_as_monitored"`. The seeding tool therefore gives it one control in
state `obligation`: it has its `NC-` identifier and its gates, is not run and makes no gate eligible. When the
Reflex entry `reflex-untimely` (N22-CX; `same_as = "CX-012"`, `numeric = true`) is in the catalogue, the next
seeding fills CX-012's `case_supplied_by` with that entry, and CX-012's control becomes runnable under the same
`NC-` number against its own gates (GATE-SHIELD-06, and those BET-25 resolves to through
`registry/rule_gates.toml`); `reflex-untimely` and `reflex-intersample` keep controls of their own (two rules,
two controls). The switch is the seeding tool's and no entry is edited by hand.

**Document control.**

| CX | The tempting shortcut | The smallest case that breaks it | The rule it protects | Control kind; caught by |
|---|---|---|---|---|
| CX-001 | Run a speed comparison first and account for the shared work afterwards | A composed solver took 1.345× the time of a specialized one (1.088× cold). The specialized batch held a residual of 575.7 µs the solver does not touch, so no solver could have shown more than about 1.46× (842.8 ÷ 575.7): the test could not have confirmed the claim it was run to test | The ceiling implied by the work both arms share is computed before a speed comparison is locked; the test is refused if the ceiling is below the claimed ratio | `planted-input`: a registration claiming 10 with this phase breakdown; refused with FROB-REF-866. N00-17, BD-354, GATE-TENFOLD-09 |
| CX-002 | Treat a closed bead as an accepted capability | A sibling project recorded 87 beads closed within 12 seconds with one shared close reason, serving figures from a stand-in | A bead closes with a typed reason that cites its own test, and gates require a live-dependency proof | scripted adversary: 87 synthetic closures sharing one reason inside 12 s; the tracker hygiene check must flag them. N00-08 |

**The ten-fold thesis.**

| CX | The tempting shortcut | The smallest case that breaks it | The rule it protects | Control kind; caught by |
|---|---|---|---|---|
| CX-003 | Declare ten-fold when the point estimate is at least 10 on every brief | A draft rule of that form confirmed a true ratio of 10 in at most 18% of simulated campaigns | The decision rule is on a pooled ratio of restricted mean times; its power against a true ratio of 10 is computed before the sealed suite is opened; a sealed-suite result is a finite-suite result and a population claim needs at least 32 independent mission families | `planted-input`: a decision rule of the draft form submitted for pre-analysis; refused for lack of power. BET-13, GATE-TENFOLD-03 |
| CX-004 | Seek the ten-fold in a faster simulator step | At fixed sample count an infinitely fast step speeds training by at most 1/(1 − s): about 7.7× for a rough-terrain humanoid (s = 87%), 6.7× for a hand, 1.9× for a cart-pole, about 1.4× from physics alone on another stack | Engine throughput is a parity requirement, not the source of the ten-fold | `planted-input`: a funding record justifying a step-speed work package by a ten-fold claim; refused by the funding checklist. N00-09, N00-17 |
| CX-005 | Decide one design per agent turn and rely on a fast cycle | With agent latency t in both arms and about 300 s of incumbent training, one design per turn gives at most (300 + t)/t: 3.5–6× for t in 60–120 s, 4.4–7.8× against the 410 s one-card figure, even at zero compute. A 120 s cycle with no training is 3.0–3.5× an incumbent quadruped cycle of 360–420 s | The unit of dispatch is a family: many designs are decided per turn | `planted-input`: a lever record claiming ten from cycle time alone. N00-17, N00-20 |
| CX-006 | Rely on pruning bad designs to cut compute per decision | C ÷ (c + c_T·f) with C = 410, c = 38, c_T = 410 GPU-s: about 10.8× at f = 0; ten needs f ≤ 0.7%; no f reaches ten if the cheap rung costs more than 41 GPU-s. If 16% of candidates fail trivially, pruning them cuts trainings by at most 1.19× | A cheaper cheap rung, many designs per rung or a cheaper training rung is required; static checks are valued for feedback in seconds, not for pruning | `planted-input`: a lever record attributing ten to pruning at f = 5%. N00-17, BET-05 |
| CX-007 | Multiply a pruning ratio by a GPU speedup | Fraction f improved by r with overhead h gives at most 1/(1 − f + f/r + h): about 13.5× at f = 0.97, r = 40, h = 0.02; infinite acceleration of 85% of the time cannot reach ten. Smallest composed case: 100 candidates of 10 s; pruning removes 90; the ten survivors accelerate by 2, not by the 5 measured on the full set; the result is 1,000/50 = 20, not 10 × 5 = 50 | The three terms are measured on the rival's critical path; a pruning ratio is never multiplied by a speedup measured on work the pruning already removed | `mutation`: a ceiling calculator that multiplies the two factors; `planted-input`: the claim-50 record. N00-17, BD-354 |
| CX-008 | Report speed on the missions that finished | If the candidate misses a fraction q of deadlines then R ≤ 1/q: with 12% unfinished the ratio cannot exceed 25/3 ≈ 8.33; one mission in eight unresolved cannot show ten-fold | Restricted mean time charges the deadline H for an unresolved mission; a failure is a failed cell and never an excluded cell | `mutation`: an estimator that drops censored missions. N00-17, N00-19, N00-27 (rule 2), GATE-TENFOLD-03 |
| CX-009 | Use a sampling controller as the evaluator because it needs no training | The most sample-efficient published controller costs about 4 × 10³ times a policy rollout per simulated second: a battery of 256 twenty-second episodes takes about 86 minutes of a device against about 1.3 s for a policy | The sampling controller is a screening rung, a fallback and a source of demonstrations; it never stands in for a trained policy as acceptance evidence; its own episodes are accepted only under the delivered-controller lane | `planted-input`: an acceptance request citing sampling-controller episodes for a trained policy, with no frozen planner record; refused (FROB-REF-817). BET-27 |

**The bets.**

| CX | The tempting shortcut | The smallest case that breaks it | The rule it protects | Control kind; caught by |
|---|---|---|---|---|
| CX-010 | Midpoint counterexample: admit the midpoint of two feasible designs to an inner set | Two feasible designs whose midpoint is infeasible: (e, W) = (0, 1.9) and (0.9, 1) each need 0.95 N; the midpoint needs 841/800 N from a pair capped at 1 N | Region certificates need a witness for the whole cell | `planted-design`: the midpoint offered as inner; must be excluded from every inner set. BET-20 |
| CX-011 | Coupled-bus fixture: merge two modules because their local summaries are identical | Two modules with identical local summaries share one current-limited bus: on a unit-current bus no one-tick schedule is feasible and two two-tick schedules are | Campaign quotienting makes no false equivalence; one false equivalence falsifies it | `planted-design`: the two modules; a merge is a failure. BET-31 |
| CX-012 | Pre-slip aliasing fixture: enforce a safety budget with a monitor alone | Two friction regimes with identical histories before first slip, where a monitor alone cannot see the breach in time. No numbers are stated with the bet; the numeric instance is the Reflex entry `reflex-untimely` on the fixture of N22c-27: a cart at 2.0 m/s braking at 5.0 m/s² from s ≤ 0.45 m, worlds μ = 0.8 and μ = 0.3 with identical transcripts before braking; at μ = 0.3 the cart crosses the line at 1.162 m/s and stops 0.2296 m beyond it | The energy-budget claim is enforced by a shield, not only observed by a monitor | `planted-world` pair with the plant `N22c-27:list_untimely_assumption_as_monitored` (an untimely assumption listed as monitored instead of placed inside the quantifier); written at M0 with `numeric = false`, an obligation, and runnable when the entry `reflex-untimely` loads. BET-25, GATE-SHIELD-06 |
| CX-013 | Omitted-law fixture: issue a calibration band because every in-family residual check passes | A fixture with an omitted law passes every in-family residual check and reverses the decision: the probe was correct in 675 of 675 in-class cases and in 0 of 225 on the omitted law | The exchangeability alarm or a refusal must fire; a band is issued only within a calibrated family | `planted-world`: the omitted-law cases; a band issued is a failure. BET-28 (KA-017) |

**The claim product.**

| CX | The tempting shortcut | The smallest case that breaks it | The rule it protects | Control kind; caught by |
|---|---|---|---|---|
| CX-014 | Report a result at the class of its strongest input | A design comparison consumes a Certified kernel identity, a friction coefficient from a card field marked `assumed` (Estimated) and a paired anytime-valid study on the GPU tier under class D2 (Tested). The product is a Design comparison of class Estimated, its weakest input is the friction field, its reality scope is `no-claim` | The weakest input wins by construction of the type: no conversion into Proved, no setter for the class, no constructor that takes a class | `mutation`: a `combine` that takes the maximum; three compile-fail crates. BD-005, N00-27 (KA-027, KA-028) |
| CX-015 | Quantifier-order fixture: treat "for every world some controller works" as "one controller works for every world" | A plant in which every build has a command that works and no single command works for all: docking with a late packet, gains 1 and 2, whose required commands differ by 1/2 (the common prefix must satisfy 2·u₁ + u₂ = 1 for gain 1 and = 1/2 for gain 2) | Quantifiers are part of the claim; evidence for the easier order is rejected; the auditor rejects a changed quantifier even when every hash is consistent | `planted-input`: a certificate offered with its quantifiers reordered (FROB-REF-826, FROB-REF-881). BET-19, N00-27 (KA-032) |

## Acceptance criteria

- [ ] Fifteen catalogue records load under N38b-05's loader; after seeding every one has at least one
      `[[control]]` record naming the gate or check that must catch it. CX-012's control is an `obligation`
      (`numeric = false`; it makes no gate eligible) until the entry `reflex-untimely` of N22-CX
      (`same_as = "CX-012"`, `numeric = true`) is in the catalogue, when the next seeding fills its
      `case_supplied_by` and makes the control runnable under the same `NC-` number, with the plant
      `N22c-27:list_untimely_assumption_as_monitored` and GATE-SHIELD-06 among its gates.
- [ ] Each control has been shown to fail its gate at least once (`last_seen.caught = true` with a session and a
      date) before that gate records PASS.
- [ ] A catalogue number is never renumbered or reused (load error on either).
- [ ] The five fixtures the thesis requires of the catalogue are present by name: the quantifier-order fixture
      (CX-015), the midpoint counterexample (CX-010), the coupled-bus fixture (CX-011), the pre-slip aliasing
      fixture (CX-012) and the omitted-law fixture (CX-013).

## Test plan

### Unit tests
`crates/frob-conformance/tests/cx_part0_8.rs`: `catalogue_has_cx_001_to_015`; `every_registered_cx_has_a_control`
(fifteen after seeding; CX-012's is in state `obligation`); `cx012_resolves_when_instance_loads` (on a
`synthetic` catalogue with an entry `reflex-untimely`, `same_as = "CX-012"`, `numeric = true` and plant
`N22c-27:list_untimely_assumption_as_monitored`, the seeding fills CX-012's `case_supplied_by` and makes its
control runnable under the NC number it had as an obligation, with GATE-SHIELD-06 among its gates, and
`reflex-untimely` keeps a control of its own; without the entry CX-012's control stays an `obligation`; a
further entry with `same_as = "CX-012"`, `reflex-intersample` with plant `N22c-27:drop_intersample_term`, has
its own control and does not by itself supply CX-012's case); `cx012_plant_matches_instance` (when
`reflex-untimely` is present its plant equals CX-012's `plant`; an entry `reflex-untimely` carrying a different
plant fails this test, naming both); `cx012_fixture_arithmetic` (v² = 2.0² − 2·(0.3·9.81)·0.45 gives 1.162 m/s at the line and a stop
0.2296 m beyond it; at μ = 0.8 the commanded 5.0 m/s² stops in 0.40 m, inside the 0.45 m); `midpoint_arithmetic` (0.95 N at both end points; 841/800 = 1.05125
N at the midpoint against a 1 N cap); `late_packet_prefix_is_inconsistent` (2·u₁ + u₂ cannot equal both 1 and
1/2); `pruning_times_speedup_is_20_not_50`; `cx003_cx008_carry_tenfold_03` (both entries list GATE-TENFOLD-03
in `rule_ids` and in their controls' `gate`, so N39-CX's `reused_entries_reported` finds no `gate-missing`); `control_without_gate_is_a_finding` (a control whose rule has no
gate is listed for the next milestone exit and is not silently dropped).

### End-to-end script and logging
`scripts/e2e/cx_part0_8.sh` runs each registered control against its gate's test in a scratch build and asserts
that the gate fails with the planted defect and passes without it. A control whose catching mechanism is not
yet built at the current milestone is logged `not yet` with the milestone of its gate and is neither run nor
counted as caught. Logged per STD-E2E: per control an `assert` line with `seed`, `kind`, the gate, the
mechanism bead, `caught` and the first differing value (planted against clean); one `obligation` line per
control in state `obligation`, with the entry that will supply its case and the owing bead; a `summary` line
with controls run, caught, not yet runnable and obligations, and gate GATE-GAUNT-01.

### Known answers and negative controls
This bead is the negative-control seed. Its own control: a registry in which CX-007's control is attached to no
gate must be reported as a finding (PO-GAUNT-03: the rule gains a gate or the entry is withdrawn with a recorded
reason).

## Considerations

- A control is a defect, not a boundary: "the request is refused at the limit" tests the limit; the control is
  the build or input in which the limit was removed.
- A control that has never failed its gate in any recorded run is itself suspect and is reviewed at each
  milestone exit.
- A counterexample found in use (a false certificate, a false promotion, a withdrawn result) is appended to the
  catalogue and becomes a permanent control.
- CX-001 and the pilots behind CX-003 and CX-013 are FACT from planning pilots (`label` FACT with its source and
  `rerun = false`, so that their controls are seeded as obligations): they are re-run in this repository before
  they are registered, by BD-043 (CX-001, the pusher audit; CX-013, the material probe) and N38a-11 (CX-003,
  the draft rule), each recorded in the plan-time pilot ledger of N38a-10.

## References

Appendix N.1 to N.5, N.35; §0.7; §38.11; GATE-GAUNT-01; INV-GAUNT-02, INV-GAUNT-07; PO-GAUNT-03.
