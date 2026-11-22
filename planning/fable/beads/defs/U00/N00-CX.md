---
key: "N00-CX"
title: "frob-conformance: counterexample catalogue entries CX-001 to CX-015 seeded as negative controls (document control, thesis, bets, claims)"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "gauntlet", "frob-conformance", "solid", "registry", "test"]
parent: "WP-34"
deps: ["BD-025"]
needs: ["§38.11: the negative-control registry `frob-conformance/registry/controls.toml` and its five rules (a gate with no control cannot record PASS, INV-GAUNT-02)"]
related: ["N00-KA", "N00-17", "N00-27", "BD-354", "BD-124", "STD-TEST"]
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
it protects. Out: the mechanisms that must catch them (named per row); the numeric instance of the pre-slip
aliasing fixture, which is owed by the energy-budget tier before CX-012 can be registered as a control.

## Technical approach

Catalogue records go to `crates/frob-conformance/registry/counterexamples.toml`; each produces one or more
`[[control]]` records in `controls.toml` with fields `id` (stable `NC-nnnn`), `seed` (the CX number), `gate`
(every gate that must catch it), `kind`, `target`, `defect`, `expect`, `last_seen`.

**Document control.**

| CX | The tempting shortcut | The smallest case that breaks it | The rule it protects | Control kind; caught by |
|---|---|---|---|---|
| CX-001 | Run a speed comparison first and account for the shared work afterwards | A composed solver took 1.345× the time of a specialized one (1.088× cold). The specialized batch held a residual of 575.7 µs the solver does not touch, so no solver could have shown more than about 1.46× (842.8 ÷ 575.7): the test could not have confirmed the claim it was run to test | The ceiling implied by the work both arms share is computed before a speed comparison is locked; the test is refused if the ceiling is below the claimed ratio | `planted-input`: a registration claiming 10 with this phase breakdown; refused with FROB-REF-866. N00-17, BD-354, GATE-TENFOLD-09 |
| CX-002 | Treat a closed bead as an accepted capability | A sibling project recorded 87 beads closed within 12 seconds with one shared close reason, serving figures from a stand-in | A bead closes with a typed reason that cites its own test, and gates require a live-dependency proof | scripted adversary: 87 synthetic closures sharing one reason inside 12 s; the tracker hygiene check must flag them. N00-08 |

**The ten-fold thesis.**

| CX | The tempting shortcut | The smallest case that breaks it | The rule it protects | Control kind; caught by |
|---|---|---|---|---|
| CX-003 | Declare ten-fold when the point estimate is at least 10 on every brief | A draft rule of that form confirmed a true ratio of 10 in at most 18% of simulated campaigns | The decision rule is on a pooled ratio of restricted mean times; its power against a true ratio of 10 is computed before the sealed suite is opened; a sealed-suite result is a finite-suite result and a population claim needs at least 32 independent mission families | `planted-input`: a decision rule of the draft form submitted for pre-analysis; refused for lack of power. BET-13 |
| CX-004 | Seek the ten-fold in a faster simulator step | At fixed sample count an infinitely fast step speeds training by at most 1/(1 − s): about 7.7× for a rough-terrain humanoid (s = 87%), 6.7× for a hand, 1.9× for a cart-pole, about 1.4× from physics alone on another stack | Engine throughput is a parity requirement, not the source of the ten-fold | `planted-input`: a funding record justifying a step-speed work package by a ten-fold claim; refused by the funding checklist. N00-09, N00-17 |
| CX-005 | Decide one design per agent turn and rely on a fast cycle | With agent latency t in both arms and about 300 s of incumbent training, one design per turn gives at most (300 + t)/t: 3.5–6× for t in 60–120 s, 4.4–7.8× against the 410 s one-card figure, even at zero compute. A 120 s cycle with no training is 3.0–3.5× an incumbent quadruped cycle of 360–420 s | The unit of dispatch is a family: many designs are decided per turn | `planted-input`: a lever record claiming ten from cycle time alone. N00-17, N00-20 |
| CX-006 | Rely on pruning bad designs to cut compute per decision | C ÷ (c + c_T·f) with C = 410, c = 38, c_T = 410 GPU-s: about 10.8× at f = 0; ten needs f ≤ 0.7%; no f reaches ten if the cheap rung costs more than 41 GPU-s. If 16% of candidates fail trivially, pruning them cuts trainings by at most 1.19× | A cheaper cheap rung, many designs per rung or a cheaper training rung is required; static checks are valued for feedback in seconds, not for pruning | `planted-input`: a lever record attributing ten to pruning at f = 5%. N00-17, BET-05 |
| CX-007 | Multiply a pruning ratio by a GPU speedup | Fraction f improved by r with overhead h gives at most 1/(1 − f + f/r + h): about 13.5× at f = 0.97, r = 40, h = 0.02; infinite acceleration of 85% of the time cannot reach ten. Smallest composed case: 100 candidates of 10 s; pruning removes 90; the ten survivors accelerate by 2, not by the 5 measured on the full set; the result is 1,000/50 = 20, not 10 × 5 = 50 | The three terms are measured on the rival's critical path; a pruning ratio is never multiplied by a speedup measured on work the pruning already removed | `mutation`: a ceiling calculator that multiplies the two factors; `planted-input`: the claim-50 record. N00-17, BD-354 |
| CX-008 | Report speed on the missions that finished | If the candidate misses a fraction q of deadlines then R ≤ 1/q: with 12% unfinished the ratio cannot exceed 25/3 ≈ 8.33; one mission in eight unresolved cannot show ten-fold | Restricted mean time charges the deadline H for an unresolved mission; a failure is a failed cell and never an excluded cell | `mutation`: an estimator that drops censored missions. N00-17, N00-19, N00-27 (rule 2) |
| CX-009 | Use a sampling controller as the evaluator because it needs no training | The most sample-efficient published controller costs about 4 × 10³ times a policy rollout per simulated second: a battery of 256 twenty-second episodes takes about 86 minutes of a device against about 1.3 s for a policy | The sampling controller is a screening rung, a fallback and a source of demonstrations; it never stands in for a trained policy as acceptance evidence; its own episodes are accepted only under the delivered-controller lane | `planted-input`: an acceptance request citing sampling-controller episodes for a trained policy, with no frozen planner record; refused (FROB-REF-817). BET-27 |

**The bets.**

| CX | The tempting shortcut | The smallest case that breaks it | The rule it protects | Control kind; caught by |
|---|---|---|---|---|
| CX-010 | Midpoint counterexample: admit the midpoint of two feasible designs to an inner set | Two feasible designs whose midpoint is infeasible: (e, W) = (0, 1.9) and (0.9, 1) each need 0.95 N; the midpoint needs 841/800 N from a pair capped at 1 N | Region certificates need a witness for the whole cell | `planted-design`: the midpoint offered as inner; must be excluded from every inner set. BET-20 |
| CX-011 | Coupled-bus fixture: merge two modules because their local summaries are identical | Two modules with identical local summaries share one current-limited bus: on a unit-current bus no one-tick schedule is feasible and two two-tick schedules are | Campaign quotienting makes no false equivalence; one false equivalence falsifies it | `planted-design`: the two modules; a merge is a failure. BET-31 |
| CX-012 | Pre-slip aliasing fixture: enforce a safety budget with a monitor alone | Two friction regimes with identical histories before first slip, where a monitor alone cannot see the breach in time. No numbers are stated with the bet; the numeric instance is owed by the energy-budget tier | The energy-budget claim is enforced by a shield, not only observed by a monitor | `planted-world` pair, registered once its numbers exist. BET-25 |
| CX-013 | Omitted-law fixture: issue a calibration band because every in-family residual check passes | A fixture with an omitted law passes every in-family residual check and reverses the decision: the probe was correct in 675 of 675 in-class cases and in 0 of 225 on the omitted law | The exchangeability alarm or a refusal must fire; a band is issued only within a calibrated family | `planted-world`: the omitted-law cases; a band issued is a failure. BET-28 (KA-017) |

**The claim product.**

| CX | The tempting shortcut | The smallest case that breaks it | The rule it protects | Control kind; caught by |
|---|---|---|---|---|
| CX-014 | Report a result at the class of its strongest input | A design comparison consumes a Certified kernel identity, a friction coefficient from a card field marked `assumed` (Estimated) and a paired anytime-valid study on the GPU tier under class D2 (Tested). The product is a Design comparison of class Estimated, its weakest input is the friction field, its reality scope is `no-claim` | The weakest input wins by construction of the type: no conversion into Proved, no setter for the class, no constructor that takes a class | `mutation`: a `combine` that takes the maximum; three compile-fail crates. BD-005, N00-27 (KA-027, KA-028) |
| CX-015 | Quantifier-order fixture: treat "for every world some controller works" as "one controller works for every world" | A plant in which every build has a command that works and no single command works for all: docking with a late packet, gains 1 and 2, whose required commands differ by 1/2 (the common prefix must satisfy 2·u₁ + u₂ = 1 for gain 1 and = 1/2 for gain 2) | Quantifiers are part of the claim; evidence for the easier order is rejected; the auditor rejects a changed quantifier even when every hash is consistent | `planted-input`: a certificate offered with its quantifiers reordered (FROB-REF-826, FROB-REF-881). BET-19, N00-27 (KA-032) |

## Acceptance criteria

- [ ] Fifteen catalogue records load; every one except CX-012 has at least one `[[control]]` record naming the
      gate or check that must catch it; CX-012 is recorded as `owed: numeric instance` and is not counted.
- [ ] Each control has been shown to fail its gate at least once (`last_seen.caught = true` with a session and a
      date) before that gate records PASS.
- [ ] A catalogue number is never renumbered or reused (load error on either).
- [ ] The five fixtures the thesis requires of the catalogue are present by name: the quantifier-order fixture
      (CX-015), the midpoint counterexample (CX-010), the coupled-bus fixture (CX-011), the pre-slip aliasing
      fixture (CX-012) and the omitted-law fixture (CX-013).

## Test plan

### Unit tests
`crates/frob-conformance/tests/cx_part0_8.rs`: `catalogue_has_cx_001_to_015`; `every_registered_cx_has_a_control`
(fourteen of fifteen; CX-012 reported owed); `midpoint_arithmetic` (0.95 N at both end points; 841/800 = 1.05125
N at the midpoint against a 1 N cap); `late_packet_prefix_is_inconsistent` (2·u₁ + u₂ cannot equal both 1 and
1/2); `pruning_times_speedup_is_20_not_50`; `control_without_gate_is_a_finding` (a control whose rule has no
gate is listed for the next milestone exit and is not silently dropped).

### End-to-end script and logging
`scripts/e2e/cx_part0_8.sh` runs each registered control against its gate's test in a scratch build and asserts
that the gate fails with the planted defect and passes without it. Logged per STD-E2E: per control an `assert`
line with `seed`, `kind`, the gate, `caught` and the first differing value; a `summary` line with controls run,
caught and owed, and gate GATE-GAUNT-01.

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
- CX-001 and the pilots behind CX-003 and CX-013 are FACT from planning pilots; they are re-run in this
  repository before they are registered (N00-06).

## References

Appendix N.1 to N.5, N.35; §0.7; §38.11; GATE-GAUNT-01; INV-GAUNT-02, INV-GAUNT-07; PO-GAUNT-03.
