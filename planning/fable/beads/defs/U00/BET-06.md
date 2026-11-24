---
key: "BET-06"
title: "Decide BET-06: a baseline controller synthesized from the model makes most well-formed designs move"
type: "task"
priority: 1
milestone: "M3"
labels: ["m3", "reflex", "frob-control", "solid", "bet", "gate"]
parent: "WP-27"
deps: ["BD-248", "BD-249", "BD-252", "N00-28", "N22c-14", "STD-E2E"]
needs: []
related: ["BET-17", "BET-27", "N00-11", "N00-15", "N40-31", "N40-32", "NS-08"]
gates: ["GATE-REFLEX-02", "KILL-BASELINE"]
plan_refs: ["§4 (BET-06)", "§40.5", "§1.1", "§1.6 (N7)", "§22.18"]
---
## Background

**The bet.** For each morphology class, a baseline estimator and controller can be synthesized from the robot
model in seconds, with no per-robot cost tuning, and it makes most well-formed designs move. This is what lets
"every design stand on day one": a new design gets closed-loop feedback before any policy is trained.

**What is and is not claimed.** Published sampling controllers run real quadrupeds with no offline training,
but each uses cost weights chosen per task and robot. The claim here is narrower and different: synthesis per
morphology class from the model. The words "no training needed" are not used anywhere. The permitted wording of
the differentiator ("a controller is synthesized from the model in seconds for each morphology class, with no
per-robot cost tuning") carries the bracket "conditional on BET-06" until this bet is decided. Tier: Solid.

**Why it matters.** The baseline controller is the cheap rung of the evaluation ladder. If it cannot make
designs move, the ladder has nothing between a static check and a full training run, and the "train few" lever
has no screen.

## Scope

In: pre-registration of the bet, the M3 reading per morphology class, the bet's verdict and its decision record
`docs/adr/bet-06-m3.md`. Out: the synthesis, the corpus, the sampling controller; the corpus measurement of
GATE-REFLEX-02, its statistic, its per-class table and the evaluation of the kill gate's trigger (BD-249); the
decision KILL-BASELINE itself, its ADR `docs/adr/ADR-KILL-BASELINE-M3.md`, its registry row and the application of its
fallback (N22c-14). This bead depends on both: it reads BD-249's recorded numbers and N22c-14's decision
record, evaluates the bet's falsifier clauses on them and writes the bet's verdict. It measures nothing and
decides no kill gate.

## Technical approach

1. **Falsifier.** Fewer than 80% of corpus designs that pass `design check` traverse 5 m of flat ground under
   the baseline controller.
2. **Competing arm.** A sampling controller with class-default costs on the same designs. **Potency control.**
   Every corpus design with a planted control-relevant defect (for example an actuator too weak to hold the
   stance, a reversed joint axis, a missing foot contact law) must fail the traverse: a test on which nothing
   fails shows nothing.
3. **Deciding gate (TARGET).** GATE-REFLEX-02 (Design comparison, M3): at least 80% of corpus designs that pass
   `design check` traverse 5 m, as a paired 95% lower bound; every planted control-relevant defect fails.
4. **Per-class reading.** The verdict is a map over morphology classes (quadruped, hexapod, biped, humanoid,
   differential, Ackermann, omnidirectional, tracked, hybrid, arm on a base); tracked and hybrid classes are
   read at M4 with their contact laws. A class is read by the class rule registered with BD-249's protocol and
   applied by N22c-14, and by no second rule: with n_c corpus designs of the class passing `design check` and
   k_c of them traversing, the class passes (the bet is held for it) when n_c ≥ 10 and k_c/n_c ≥ 0.80; with
   fewer than ten designs it is `undecided`: it keeps synthesis only if every one of its designs traverses,
   is otherwise treated as not passing, and is listed in the verdict as `undecided` with the designs the
   corpus would need to decide it. The lower bound is the corpus statistic; per class it is reported and does
   not decide, because with 20 designs 19 passes give a 95% lower bound of 0.78.
5. **Kill gate and fallback.** KILL-BASELINE (M3): GATE-REFLEX-02 fails. Fallback: baseline synthesis is limited
   to the morphology classes that pass; the others report `not-synthesizable` and route to the sampling
   fallback or to training.
6. **Downstream if falsified.** For a failing class, humanoid and biped briefs in particular, the expected
   ten-fold range collapses toward parity (the range of 5 to 20 for humanoid briefs requires a baseline or
   family policy to stand in for training when designs are ranked); the differentiator N7 is rendered per
   class; rank agreement (BET-17) cannot be measured for a class with no baseline rung.
7. **Evidence so far.** Published model-based and sampling controllers on legged robots (FACT about sources);
   nothing on a varied corpus, nothing for tracked robots. No measurement on this system.
8. **Procedure.** Register the threshold, the corpus digest and the class list before synthesis is run on the
   corpus; read at M3 exit (M4 for tracked and hybrid); `docs/adr/bet-06-m3.md`.

## Acceptance criteria

- [ ] Pre-registered with corpus digest and class list.
- [ ] Per class: designs passing `design check`, designs traversing 5 m, the proportion with its paired 95%
      lower bound, and the sampling-controller arm's proportion on the same designs.
- [ ] All planted control-relevant defects failed.
- [ ] Verdict per class recorded in `docs/adr/bet-06-m3.md`, which cites the KILL-BASELINE decision on record
      in N22c-14's ADR and registry row (that decision is made there, not here); N7's rendering updated.

## Test plan

### Unit tests
`bets.rs`: `bet06_lower_bound_0_83_holds`; `bet06_point_0_82_lower_bound_0_77_falsifies` (the threshold is on
the lower bound, not the point estimate); `bet06_per_class_map` (quadruped 0.91 held, biped 0.55 falsified →
`HeldInPart`); `bet06_planted_defect_that_traverses_is_untested`; `bet06_kill_baseline_routes_class`.

### End-to-end script and logging
`scripts/e2e/bet_06_decide.sh` recomputes the per-class verdict from BD-249's recorded rows with the bet
register's report (`cargo run -p frob-conformance --example bets -- status --json`, example `bets` of N00-28; the
script reads the row of BET-06, whose `HeldInPart` map gives one verdict per class), checks the ADR against it
field by field, and under a fired kill gate
checks that `frob control synth <design> --key b06-withdrawn --json` on a design of a failing class returns the outcome `not-synthesizable`
with the route to the sampling fallback as its first next action. Logged per STD-E2E: per class the counts,
proportion, lower bound against 0.80, the arm's proportion, planted-defect outcomes; `summary` naming
GATE-REFLEX-02 and KILL-BASELINE.

### Known answers and negative controls
No closed-form known answer applies; the bound is binomial. Negative controls: the planted control-relevant
defects; a traverse counted after a manual cost adjustment for one robot (the run manifest records that no
per-robot parameter was set; a manifest with one fails the gate).

## Considerations

- "Well-formed" is defined by `design check` passing; if BET-05 is falsified the population changes and this
  bet is re-read on designs that pass the mechanical lints only.
- Flat ground for 5 m is a low bar on purpose: the baseline is a screen, never acceptance evidence.
- The referee refuses more than 21 contacts; classes that exceed it are read on the CPU batch tier.

## References

§4 (BET-06), §40.5, §1.1, §1.6 (N7), §22.18; GATE-REFLEX-02; KILL-BASELINE.
