---
key: "BET-34"
title: "Decide BET-34: referee coverage (the referee completes decision scenes; the sampled referee covers enough)"
type: "task"
priority: 1
milestone: "M3"
labels: ["m3", "kinesis", "frob-dyn", "solid", "bet", "gate"]
parent: "WP-50"
deps: ["BD-086", "BD-093", "BD-213", "BD-214", "BD-224", "BD-268", "N00-28", "N16b-19", "STD-E2E"]
needs: []
related: ["FD-FSIM-12", "FD-FSIM-13", "GB-16", "N00-11", "N00-16", "N00-35", "N40-31", "N40-32"]
gates: ["GATE-FID-03", "GATE-FID-07", "KILL-SHADOW"]
plan_refs: ["§4 (BET-34)", "§40.5", "§0.8 (H-11)", "§1.7", "§1.8", "§16.17", "§43.5", "§43.12"]
---
## Background

**The bet.** The trust ladder says GPUs propose and the referee disposes: a fast-tier result is provisional
until it has been re-evaluated on the referee tier. That only works if the referee can actually run the scenes
on which decisions are made, within a time budget, and if enough fast-tier work is spot-checked. The referee is
built on the sibling's articulated contact step under a transactional, event-located step (a step is committed
only when all its checks pass; a failed step is restored and retried with different numerical parameters;
contact events are located on rational sub-intervals). Beside promotion, a *sampled background referee* re-runs
a small, address-hashed sample of fast-tier snapshots to detect tier-sensitive behaviour early. The bet is that
both have enough coverage. Tier: Solid.

**Why it could fail (FACT with path in the foundation fact sheet).** The sibling's articulated contact step
refuses more than 21 contacts and has no complete persistent contact time stepper. A humanoid on rubble or a
tracked robot can exceed that. And a CPU referee at 0.2–1.0× real time per core on 24 cores can shadow only
about 0.1–2.4% of GPU world-seconds; the figure scales with referee speed and is unmeasured.

## Scope

In: pre-registration, readings at M2 (coverage report; sampler) and M3 (the sampler at its gate), per-class
verdict and decision record. Out, with the beads that have them: the thin-barrier and sharp-sill fixtures
(BD-086) and witness migration in the transactional step (BD-093); the coverage report `fidelity.coverage` and
GATE-FID-03 (BD-213); the sampled background referee `fidelity.shadow` and its M2 record of GATE-FID-07
(BD-214); GATE-FID-07 repeated at M3 with the tier-correction ledger (BD-268); the decision KILL-SHADOW, its
ADR and fallback (N16b-19), which this bead reads and cites and never re-decides; the strict CPU profile (its
Kinesis beads); the upstream work items that raise the contact capacity and complete the stepper (FD-FSIM-12,
FD-FSIM-13).

## Technical approach

1. **Falsifier clauses** (any falsifies, per robot class):
   a. At M2 exit the referee completes fewer than 90% of standard-battery scenarios.
   b. A promotion exceeds its time budget.
   c. Sampled coverage is below 0.1% of GPU world-seconds, with coverage computed from the referee's measured
      world-seconds per second and no band assumed.
2. **Potency control.** A thin-barrier fixture and a witness-migration fixture that a fixed-step tier gets
   wrong (a body tunnels through a thin wall; a contact witness must move when rolling around a sharp lip): the
   referee must get them right, showing it is doing something the fast tier is not. For the sampler: a planted
   single-kernel defect and a planted tier-sensitive policy.
3. **Deciding gates (TARGETs).**
   - GATE-FID-03 (Fidelity, M2): the referee's coverage reported per robot class at M2 exit, with the breakdown
     by refusal cause, the strict-profile share and promotion wall-clock at the 50th and 95th percentiles; an
     adjudicator named wherever coverage is below 90%; coverage below 90% in a reference class falsifies this
     bet for that class.
   - GATE-FID-07 (Fidelity, M2; the sampler at its gate at M3): coverage at least 0.1% of GPU world-seconds,
     reported; GPU throughput loss at most 5%; a planted single-kernel defect localized to its stage from
     one-step defects; a planted tier-sensitive policy detected within twice the predicted time. The
     prediction: a gap of δ in success rate p between tiers needs about 16·p·(1 − p)/δ² episodes per tier (576
     at p = 0.9 and 1,600 at p = 0.5 for δ = 0.05; 11,520 to 32,000 world-seconds at 20 s per episode), divided
     by the measured referee rate; one-step defects from a wrong kernel show in seconds.
4. **Kill gate and fallback.** KILL-SHADOW (M3): GATE-FID-07 fails on cost or on coverage after one repair
   cycle, or no planted defect is detected; the referee runs at promotion time only, plus an offline audit of
   sampled snapshots after each run. For clause (a): upstream work items FD-FSIM-12 and FD-FSIM-13 raise the
   contact capacity and complete the stepper; until they land, the strict CPU batch profile (our own kernels, transactional and
   event-located, double precision) stands as the highest tier for scenes the referee cannot complete, and
   results there stay provisional and say so.
5. **Downstream if falsified.** For a class under 90%: a named adjudicator (the strict profile) and a statement
   on every result of that class; "qualified on the referee" is not used for it. The definition of done prints
   80% as the level below which an adjudicator is named; the gate register, this bet and the dynamics chapter say
   90%, the register governs, and the adjudicator is named wherever coverage is below 90% (the discrepancy is
   BD-213's to list). If the sampler is killed, tier-sensitivity is caught only at promotion, later and at more
   cost.
6. **Procedure.** Register the battery, the promotion time budgets per class and the sampling rate before the
   first coverage run; read at M2 and M3 exits; `docs/adr/bet-34-m2.md`, `-m3.md`. The M2 reading (GATE-FID-03
   and the M2 record of GATE-FID-07) is taken at the M2 exit by the exit review (BD-224: "read here and decided
   at M3"), from the gate rows through the bet register's verdict function (N00-28) and N40-31's exit state; a
   class below 90% has its adjudicator named by then (BD-213). This bead closes at M3 and writes `bet-34-m2.md`
   from that recorded M2 reading and `bet-34-m3.md` from BD-268's record and the KILL-SHADOW decision.

## Acceptance criteria

- [ ] Pre-registered with per-class promotion budgets.
- [ ] Per class: scenarios completed out of attempted, refusals by cause (the causes the coverage report uses:
      contact cap above 21 contacts, fixed-witness gate, missing joint class, kinematic loop), the share the
      strict profile completes instead, promotion wall-clock at the 50th and 95th percentiles, adjudicator if
      below 90%.
- [ ] Sampler: measured coverage as a share of GPU world-seconds (from measured referee speed), throughput loss,
      detection of both planted defects.
- [ ] The thin-barrier and witness-migration fixtures are right on the referee and wrong on the fixed-step tier.
- [ ] Verdict per class recorded in `docs/adr/bet-34-m2.md` and `bet-34-m3.md`; the M3 record cites the
      KILL-SHADOW decision on record by N16b-19 (by identifier and hash; with it missing the verdict is not
      filed).

## Test plan

### Unit tests
`bets.rs`: `bet34_coverage_94_percent_holds`; `bet34_humanoid_class_81_percent_falsified_for_class` (verdict
`HeldInPart`, adjudicator required); `bet34_promotion_over_budget_falsifies_b`;
`bet34_sampled_coverage_0_06_percent_falsifies_c`; `bet34_coverage_from_assumed_band_rejected` (the record must
cite measured referee world-seconds per second); `bet34_reads_kill_shadow_record` (N16b-19's record fired
because no planted defect was detected → the sampler half is `Falsified` through its kill gate, the record's
hash carried; the trigger is applied there); `bet34_detection_prediction_576_and_1600` (16·p·(1 − p)/δ² at
δ = 0.05).

### End-to-end script and logging
`scripts/e2e/bet_34_decide.sh` runs `frob fidelity coverage --class <c> --json` for each reference class and
`frob fidelity coverage --run <id> --json` on the recorded M2 and M3 gate runs (`frob fidelity shadow status
--job <id> --json` where the job is still live), re-runs the thin-barrier fixture on the referee and on the
lockstep tier (`frob sim run <thin-barrier fixture> --tier referee --key b34-ref --json` and `--tier batch-cpu
--profile lockstep --key b34-lock --json`), recomputes coverage shares and reads the per-class verdict of BET-34 from the bet register's report (`cargo run
-p frob-conformance --example bets -- status --json`, example `bets` of N00-28), checking both ADRs against it. Logged per STD-E2E: class, completed and attempted, refusal causes, percentiles against
budgets, referee world-seconds per second, GPU world-seconds per second, coverage share against 0.1%,
throughput loss against 5%; `summary` naming GATE-FID-03, GATE-FID-07 and KILL-SHADOW.

### Known answers and negative controls
Coverage arithmetic (GPU world-seconds per second are T·h): a humanoid batch at 1.7 M world-steps/s with
h = 2 ms produces 3,400, a quadruped batch at 2 M produces 4,000; a referee at 0.2–1.0× real time per core on 24
cores produces 4.8–24. Shares: 4.8/4,000 = 0.12% to 24/4,000 = 0.6% (quadruped), 4.8/3,400 = 0.14% to
24/3,400 = 0.71% (humanoid), and 24/1,000 = 2.4% where a device produces only 1,000 world-seconds per second:
the "about 0.1–2.4%" of the background. KA-008 (witness migration). Negative controls: the two fixtures; the planted
kernel defect; the planted tier-sensitive policy; a coverage report that counts refused scenarios as completed.

## Considerations

- The sampled referee is a correctness guard that saves no compute; its samples, drawn with recorded inclusion
  probabilities, also feed the paired tier correction.
- Refusal is the referee's honest answer above its capacity; a clamp of the contact count would be a defect.

## References

§4 (BET-34), §40.5, §0.8 (H-11), §1.7, §1.8, §16.17, §43.5, §43.12; GATE-FID-03, -07; KILL-SHADOW; KA-008.
