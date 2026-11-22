RESULT_STATUS: MATERIAL_INCREMENT

# P05-BACKCAST-premortem (PQ5)

Labels: FACT cites the case ledger or a plan section. ARITH is arithmetic on cited numbers; the plan's numbers are targets, so every ARITH result is illustrative. HYP is untested.

## A. Backcast from a confirmed tenfold claim in 2029

Working metric (proposed, because E016 records that none exists): wall-clock and device-seconds from a design brief to a decision that survives a party-neutral adjudicator, at a matched decision-error rate, against the same agent driving the best incumbent stack.

ARITH that frames the rest. Incumbent quadruped training costs about 600 GPU-s (5 min on two RTX 4090, E011). The plan's cheap rung costs about 38 GPU-s per design (§24.8). Its training rung targets 900 GPU-s (GATE-CORTEX-05), 1.5 times worse than the incumbent. Compute ratio = 600 / (38 + 900·f), where f is the share of simulated designs that get trained. Tenfold needs f ≤ 2.4%, under one design in forty. Wall-clock: NS-06 is 120 s per cycle against roughly 360 to 420 s for an incumbent cycle (300 s of training plus an assumed 60 to 120 s of agent and evaluation overhead), so 3 to 4 times. For a humanoid (under 30 min, E011) the same arithmetic allows up to about 15 times.

Capabilities, in the order they had to exist:

1. **A decision benchmark with an incumbent arm and a neutral adjudicator. Absent.** Fixed design briefs; the same agent on the incumbent stack, given the same sequential test so speed is not bought with error; adjudication outside FrankenRobotics' own referee. Lives in §2 (a new bet), a new §21.5, §28.1 (end-to-end rows) and §30.2 (the incumbent harness as an out-of-process oracle, which §4.3 permits). First act at M0: measure the incumbent's phase times (case §4, first unknown).
2. **A rung-fidelity ledger for controllers. Present but timid.** §14.5 ledgers physics tiers only, and BET-06 asks for 5 m of flat ground. Needed: rank agreement between baseline-controller and trained-policy scorecards per class and task family, with a `controller-sensitive` flag. §14.5, §18.6, §22.6.
3. **Designs as the batch axis. Present but unstated.** §14.3 fixes shapes per environment and no sentence lets worlds carry different design parameters. Agent think-time is common to both arms, so designs evaluated per agent turn is the only route to tenfold wall-clock on quadrupeds. §12.3, §14.3, §22.6, §28.1.
4. **An amortized, design-conditioned policy prior. Absent.** §19.3 tags transformers [M] and plans no pretraining. It removes the 900·f term. One policy across many legged embodiments with zero-shot transfer is published (abstract-level reading). §19.3, OPEN-SPARK-2.
5. **Referee coverage of the scenes where decisions are made. Present only as upstream asks.** E004: 21-contact default, no persistent stepper, convex primitives. No gate measures the share of task scenarios the referee can run, or promotion latency. §14.2, FD-FSIM-02 and -03.
6. **An external truth anchor and a replication kit. Present but late.** §30.7 arrives at M5 with default `no-claim`; OPEN-PROG-1 defaults to private. Nobody can independently confirm a private repository.

## B. Pre-mortem: every gate passed, the claim still failed

| Rank | Reason | Leading indicator | Where the plan hides it |
|---|---|---|---|
| 1 | Nothing confirmable existed: no metric, no incumbent arm. GATE-ATEL-03 compares the product with its own ablation. | No incumbent phase measurement at M2 exit | §2 BET-10, §28.1 |
| 2 | Amdahl. Incumbent training is minutes, not the "six hours" of NS-01, and agent latency is common to both arms. | Training under about 90% of a measured incumbent cycle | §5, §18.1 |
| 3 | The cheap rung reorders designs, so most designs get trained after all, on a training rung targeted worse than the incumbent's. Worst on stairs, rubble and bipeds, where the multiplier is largest. | Verdict flips after finalist training; f above 2.4% | §18.1, §18.5, §22.6 |
| 4 | The referee could not run interior or tracked scenarios (E004), so results stayed `provisional`. | Share of standard-battery scenarios the referee completes at M2 | §14.2 |
| 5 | Common-mode error. All tiers share `generic` cards, so accepted decisions failed neutral adjudication (E013). | Share of card fields that are `assumed` or `generic` in benchmark designs | §6 rule 4, §13.2 |
| 6 | Structure helped agents only slightly. BET-10 passes on any win. | Iterations-to-spec ratio in the M4 A/B | §22.9 |

## C. Conservatism audit

**Safe choice where a bolder option with a kill gate is worth far more**

- §28.1 has only parity throughput rows. Add the end-to-end ratio. Kill: under 3 times on compute for quadruped briefs at M3 withdraws the speed claim and the product is positioned on auditability.
- NS-06 (thirty cycles an hour) is itself only 3 to 4 times. Restate it as designs decided per hour (P05-4).
- GATE-REFLEX-02 tests whether a design moves, not whether its ranking holds (P05-3).
- GATE-ATEL-03 needs a magnitude and an incumbent arm (P05-2).
- §19.3 trains per design (P05-5).
- §30.7 could admit published figures for the reference robots at M3 as a first anchor that exposes shared card error (HYP: those figures are comparable enough).

**Already ambitious; do not inflate**

- Raw steps per second. No mechanism gives tenfold per step: BET-01 concedes up to 1.3 times slower than hand-tuned code, and E012 shows what an unearned headline costs.
- Single-source derived kernels on three backends with the IEEE probe.
- The trust ladder and e-process comparison. Right-sized.
- Sixteen static analyses. Add none.
- Gradients through contact at M6. A correct deferral once item 3 of section A supplies finite differences across a population.
- Reject sampling-based predictive control as the universal baseline. ARITH: 2,048 samples × 20 knots × 1,000 control steps is about 4 × 10⁷ env-steps per episode, so six episodes cost one incumbent training run (about 2.4 × 10⁸).

**Over-built for its payoff against this metric**

- Lean capsules and T-12 to T-14. No effect on decision time, and E009 blocks the modules that matter. Instance certificates suffice.
- Seven interior grammars at M3 before one archetype has produced benchmark evidence. All seven stay in scope by owner constraint; sequence one first.
- A-09 and A-15, which Appendix B already names as first cuts.

## Candidate claims

**P05-1.** As written, the plan's targets support roughly 3 to 4 times wall-clock and tenfold compute only if f ≤ 2.4% on quadrupeds. Type: ARITH. Evidence: E011, §24.8, NS-06, GATE-CORTEX-05. Assumptions: the 5-minute figure holds for novel designs; the 38 GPU-s of §24.8 is illustrative; 60 to 120 s of overhead per incumbent cycle. Scope: quadruped velocity tasks. Reverses if measured incumbent cycles exceed 20 minutes.

**P05-2.** No tenfold claim is confirmable without a decision benchmark carrying an incumbent arm, matched error rates and a neutral adjudicator. Type: gap. Evidence: E016, §22.9, §28.1. Assumptions: none. Scope: whole program. Reverses if the owner defines tenfold as an internal ablation.

**P05-3.** Add a controller rung-fidelity gate: Kendall τ ≥ 0.7 between baseline-rung and trained-rung rankings on the held-out corpus, per task family. Below 0.5, baseline verdicts for that class are advisory and the class leaves the speed claim. Type: proposal with kill gate. Evidence: §14.5, §18.5, BET-06. Assumptions: HYP that rankings can diverge. Scope: §18, §22.6. Reverses if P05-5 succeeds so fully that every design is scored by a learned policy at cheap-rung cost.

**P05-4.** Make per-world design parameters explicit. Test: 1,024 parametric variants in one batch reproduce the digests of 1,024 single-design runs at no more than 1.3 times homogeneous step cost. Kill: sequential environments. Type: proposal. Evidence: §14.3, §22.6, §27.4. Assumptions: variation within one topology family. Scope: continuous parameters. Reverses if one-design-per-turn cycles measure under 40 s end to end.

**P05-5.** Add a design-conditioned policy prior. Test: zero-shot on held-out variants reaches 80% of per-design PPO return with τ ≥ 0.7, pretraining counted in compute. Kill: behaviour-cloning warm start, the present plan. Type: proposal. Evidence: https://arxiv.org/abs/2409.06366 , https://arxiv.org/abs/2505.05753 , case §4. Assumptions: an approved envelope; ARITH: 48 GPU-hours pays back after roughly 60 to 190 decisions. Scope: legged families. Reverses if f ≤ 2.4% without it.

**P05-6.** Do not adopt sampling-based predictive control as the universal zero-training baseline. Type: rejection, ARITH. Evidence: E011, https://arxiv.org/abs/2409.15610 . Assumptions: sample count from a secondary source; horizon assumed; each RTX 4090 near the published A100 training rate. Scope: many-world evaluation, not a single-world fallback for `not-synthesizable` designs. Reverses if under 100 samples suffice.

**P05-7.** Do not raise raw throughput targets. Type: rejection. Evidence: E012, E016, BET-01. Assumptions: incumbents sit near the hardware limit. Scope: §28.1. Reverses if incumbent throughput measured on `trj` falls tenfold below the §28.1 targets.
