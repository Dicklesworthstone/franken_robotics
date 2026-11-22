---
key: "BET-05"
title: "Decide BET-05: static analysis prunes bad designs before simulation and does not throw away a winner"
type: "task"
priority: 1
milestone: "M3"
labels: ["m3", "atelier", "frob-analyze", "solid", "bet", "gate"]
parent: "WP-25"
deps: ["N00-28", "BD-248", "BD-250", "STD-E2E"]
needs: ["§28.12 and §26.10: the blinded audit of discarded strata and the planted winner (the screening audit applied by the cascade), needed for the second and third falsifier clauses"]
related: ["BD-110", "BD-345", "BET-17", "NS-01", "N00-20", "N00-15"]
gates: ["GATE-ATEL-01", "KILL-STATIC"]
plan_refs: ["§4 (BET-05)", "§40.5", "§2.2", "§2.5", "§28.23", "§1.6 (N2)"]
---
## Background

**The bet.** Analyses that need no simulation (`design check`: mass closure, actuator envelopes, symmetry,
mobility, static stability, self-collision and the rest) identify designs that will fail, in seconds, and do
not discard a design that would have won.

**What it is worth, honestly.** Its measured ceiling as a pruning lever is modest: in one published loop under
2% of morphology proposals fail validation before training and 15.8% of trained candidates are degenerate after
a full training run, so pruning trivial failures cuts trainings by at most 1/(1 − 0.16) ≈ 1.19×. The main
value is feedback in seconds with typed repairs, which serves the levers "decide many designs per turn" and
"explain by intervention". The bet is that the verdicts are *predictive enough to act on* and *safe*: a screen
that silently removes the best design is worse than no screen. Tier: Solid.

## Scope

In: pre-registration, the M3 reading on the design corpus, verdict and decision record. Out: the analyses, the
corpus, the heuristic arm and the audit lane.

## Technical approach

1. **Falsifier clauses** (any falsifies), on the design corpus (about 200 designs with frozen labels, planted
   defects, planted winners and planted single causes):
   a. `design check` verdicts predict simulated failure no better than a mass-and-torque heuristic: area under
      the ROC curve below 0.8, or less than 0.05 above the heuristic.
   b. The blinded audit of discarded strata finds a discarded design in the true top-k.
   c. The planted winner is discarded.
2. **Competing arm.** A mass-and-torque heuristic (total mass against summed peak actuator torque at the joints
   that carry weight), given the same designs. **Potency controls.** A planted winner that the screen represents
   poorly; sampled discarded strata; planted defective designs that must be flagged and a planted sound design
   that must pass.
3. **Deciding gate (TARGET).** GATE-ATEL-01 (Design comparison, M3): on the corpus, area under the ROC curve at
   least 0.8 and at least 0.05 above the heuristic baseline at a 95% paired lower bound; planted defective
   designs flagged and a planted sound design passed.
4. **Kill gate and fallback.** KILL-STATIC (M3): GATE-ATEL-01 fails. Fallback: `design check` verdicts are
   downgraded to advisory lints and the pruning claim is withdrawn.
5. **Downstream if falsified.** The first rung of the evaluation ladder stops eliminating: every design proceeds
   to simulation and static findings are shown beside results. The differentiator "the first shipped suite of
   pre-simulation design analyses callable by an agent" stays (it is a statement about a function, not about
   pruning). The ten-fold arithmetic for wheeled and tracked briefs, which leans on clearance and static
   analysis removing candidates, is recomputed.
6. **Evidence so far.** Published degenerate-candidate rates (FACT about one loop). A plan-time pilot in which a
   support-polygon-plus-capacity screen wrongly accepted 1,596 bodies that an exact classifier rejects: screens
   err in both directions, which is why clauses (b) and (c) exist.
7. **Procedure.** Register thresholds and the corpus digest before labels are computed; read at M3 exit; write
   `docs/adr/bet-05-m3.md`.

## Acceptance criteria

- [ ] Pre-registered with the corpus digest; labels frozen before `design check` is run on the corpus.
- [ ] Measured area under the curve for both arms with the paired 95% lower bound of their difference.
- [ ] The audit of discarded strata and the planted-winner result are recorded.
- [ ] Verdict and the KILL-STATIC decision recorded as an ADR at M3 exit.

## Test plan

### Unit tests
`bets.rs`: `bet05_auc_0_86_vs_0_78_holds` (difference lower bound 0.055); `bet05_auc_0_79_falsifies_a`;
`bet05_auc_gap_0_04_falsifies_a`; `bet05_discarded_design_in_top_k_falsifies_b`;
`bet05_planted_winner_discarded_falsifies_c`; `bet05_kill_static_downgrades_to_lint`.

### End-to-end script and logging
`scripts/e2e/bet_05_decide.sh` recomputes the verdict from the gate row and audit records, and under a fired
kill gate checks that `frob design check --json` returns findings with severity `advisory` and that no ladder
record lists an elimination by static verdict. Logged per STD-E2E: both curves' areas, the paired bound, audit
sample sizes and findings, the planted cases' outcomes; `summary` naming GATE-ATEL-01 and KILL-STATIC.

### Known answers and negative controls
KA-015 (1/(1 − 0.16) = 25/21 ≈ 1.19). CX-006. Negative controls: the planted winner; the planted defective and
sound designs; labels recomputed after seeing the screen's output (rejected by the freeze digest).

## Considerations

- The curve is computed against simulated failure under the baseline controller; a design that fails only
  because its controller is poor is a controller finding, and the label set records which is which.
- Top-k and the audit sample size are fixed at registration; thirty clean audited discards bound the discard
  error rate at 9.5%.
- A screen can be predictive and still unsafe; clauses (b) and (c) are independent of (a).

## References

§4 (BET-05), §40.5, §2.2, §2.5, §28.23; GATE-ATEL-01; KILL-STATIC; KA-015; CX-006.
