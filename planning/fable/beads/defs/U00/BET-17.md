---
key: "BET-17"
title: "Decide BET-17: rank agreement between rungs (cheap rungs order designs as the trained, promoted rung does)"
type: "task"
priority: 1
milestone: "M4"
labels: ["m4", "atelier", "cortex", "reflex", "frob-opt", "frontier", "bet", "gate", "owner-decision"]
parent: "WP-62"
deps: ["N00-28", "N00-07", "BD-145", "BD-310", "BD-328", "STD-E2E"]
needs: ["§42 (OPEN-PROG-4): the owner's approval of the compute for about forty per-design trainings (about 5 GPU-hours where specialists are reused, about 45 otherwise), which exceeds the default envelope", "§23.28: the winner check for a selecting rung (GATE-CORTEX-21)", "§28.10: the rank-agreement ledger record and the three modes `selective`, `discard-only`, `uninformative`"]
related: ["BET-13", "BET-15", "BET-05", "BET-06", "BD-265", "BD-329", "BD-345", "N00-12", "N00-09"]
gates: ["GATE-REFLEX-04", "GATE-CORTEX-12", "GATE-ATEL-07", "KILL-RANK"]
plan_refs: ["§4 (BET-17)", "§40.5", "§0.8 (H-22a)", "§2.4", "§2.7", "§28.23", "§43.7"]
---
## Background

**The bet.** The evaluation ladder has cheap rungs (a static check, a synthesized baseline controller, a family
policy, a short horizon) and an expensive one (a trained policy promoted on the referee). Cheap rungs are only
useful for *eliminating* designs if they rank designs the way the expensive rung does. The bet is that they do,
well enough. It is the third bet on which the thesis turns, and the decisive early experiment of the program:
it decides the "train few" lever and half of "spend where it matters".

**What the evidence supports.** Nothing direct. No published rank agreement between a shared policy and
per-design specialists was found; the nearest figure, 0.40, compares a critic with its own policy. And a high
rank correlation does not protect the winner: with 40 designs and the true best at cheap rank 3, Kendall's τ on
the top ten is 0.91 and a cut to two discards it. The experiment therefore reports containment and regret
beside τ. Tier: Frontier.

**Kendall's τ** between two rankings of n designs is (concordant pairs − discordant pairs) ÷ (n·(n − 1)/2).

## Scope

In: the pre-registration check (thresholds fixed before data), the M3 reading (pilot on incumbent tools; legged
and wheeled classes on the baseline rung), the M4 reading (our tiers; tracked and hybrid), the per-class mode
table, verdict and decision record. Out: the experiment, the ledger, the ladder.

## Technical approach

1. **Falsifier clauses** (any falsifies, per morphology class and task family; thresholds fixed before data are
   collected):
   a. Kendall τ between the baseline-controller rung and the trained-policy rung on the held-out corpus is
      below 0.7. Below 0.5 the cheap rung's verdicts are advisory for that class and the class leaves the speed
      claim.
   b. Top-k recall, measured by the audit lane, is below its declared floor.
   c. A cut sized from τ alone discards the higher rung's best design in the calibration set (the winner check).
2. **Competing arm.** Successive halving on a fixed schedule. **Potency controls.** A planted winner that the
   cheap rung represents poorly; sampled discarded strata; classes expected to be hard (stairs, rubble, bipeds);
   planted populations with known rank correlation on each side of each threshold.
3. **Deciding gates (TARGETs).**
   - GATE-REFLEX-04 (M3 legged and wheeled; M4 tracked and hybrid): rank agreement of the baseline rung with
     trained specialists on 30 to 40 designs per class and task family: a τ lower 95% bound of at least 0.7 lets
     the rung select; at least 0.5 lets it discard the bottom half; below 0.5 it removes outright failures only
     (`baseline-only: unranked`).
   - GATE-CORTEX-12 (M3 as a pilot on incumbent tools; M4 on ours): the experiment is run and its consequence
     applied: τ, top-10 recall and intervals recorded per class and task family, and a rung without a record is
     refused. Passes when the record exists and the rule is enforced, whatever τ is.
   - GATE-ATEL-07 (M3 pilot; M4): per class, τ on the top band at least 0.7 for `selective` and at least 0.5
     for `discard-only`, and per-episode correlation ρ_ep at least 0.7, all at the lower bootstrap bound, on 40
     designs. A class below 0.5 is `uninformative`.
4. **Kill gate and fallback.** KILL-RANK (M4): GATE-ATEL-07 fails for every class tested. Fallback: the cheap
   rung is `uninformative` everywhere and every candidate that survives static analysis is scored on the higher
   rung; the ladder's claim shrinks to stopping and elimination among trained candidates; the ten-fold report
   records the lost factor. Per class short of the kill: the rung is demoted to advisory, the elimination
   margin widens, and more designs are trained.
5. **Downstream if falsified.** The compute-per-decision ceiling needs under about 0.7% of designs trained to
   reach ten; without elimination by cheap rungs that fraction is not reachable, so the affected classes leave
   the speed claim and the sealed suite is re-stratified. If KILL-RANK and KILL-FAMILY both fire, the "train
   few" lever is gone.
6. **Procedure.** Pre-registration (design set, strata, the τ thresholds 0.7 and 0.5, the compute request) is
   on record before any data; the owner approves the compute; read at M3 exit and M4 exit;
   `docs/adr/bet-17-m3.md`, `bet-17-m4.md` with the per-class mode table.

## Acceptance criteria

- [ ] The pre-registration predates the first training run of the experiment (digest and date in the ledger).
- [ ] Per class and task family: τ with its lower bound, top-10 recall, containment of the best design, regret,
      ρ_ep, and the assigned mode (`selective`, `discard-only`, `uninformative`).
- [ ] The planted winner and the planted-correlation populations were handled as prescribed.
- [ ] A rung with no record is refused by the ladder; verdict and the KILL-RANK decision recorded as ADRs.

## Test plan

### Unit tests
`bets.rs`: `bet17_tau_lb_0_74_selective`; `bet17_tau_lb_0_58_discard_only`; `bet17_tau_lb_0_41_uninformative`;
`bet17_winner_lost_despite_tau_0_91` (the 40-design example: τ on the top ten = 1 − 2·2/45 = 0.911 with the
best at cheap rank 3; a cut to two fails containment → clause c); `bet17_all_classes_fail_fires_kill_rank`;
`bet17_threshold_set_after_data_rejected`.

### End-to-end script and logging
`scripts/e2e/bet_17_decide.sh` recomputes τ, recall and containment from the ledger's rank records, the mode per
class and the verdict, then asks the ladder (`frob eval rank-agreement --show`) for the mode in force and checks
it matches. Logged per STD-E2E: class, task family, n, τ and bounds against 0.7 and 0.5, recall, containment,
regret, mode; `summary` naming the three gates and KILL-RANK.

### Known answers and negative controls
The winner-lost example above. Negative controls: the planted winner; planted populations at τ = 0.75 and 0.45
(must be assigned `selective` and `uninformative`); thresholds edited after data.

## Considerations

- A pilot on incumbent tools or on the CPU batch tier informs funding and supports no gate of ours; it is
  re-run on our tiers at M4.
- τ is top-weighted in the ledger because only the top of the ranking is ever acted on.
- An unmeasured stratum never selects. The source texts differ on whether it may discard (one passage says
  `discard-only`, another `uninformative` until a record exists, and the gate says a rung without a record is
  refused); the fail-closed reading, `uninformative`, is used here and the difference is flagged for the lead.

## References

§4 (BET-17), §40.5, §0.8 (H-22a), §2.4, §2.7, §28.23, §43.7; GATE-REFLEX-04, GATE-CORTEX-12, GATE-ATEL-07;
KILL-RANK.
