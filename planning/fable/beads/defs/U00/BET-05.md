---
key: "BET-05"
title: "Decide BET-05: static analysis prunes bad designs before simulation and does not throw away a winner"
type: "task"
priority: 1
milestone: "M3"
labels: ["m3", "atelier", "frob-analyze", "solid", "bet", "gate"]
parent: "WP-25"
deps: ["BD-248", "BD-250", "N00-28", "N28a-22", "STD-E2E"]
needs: []
related: ["BET-17", "N00-15", "N00-20", "N26b-03", "N26b-05", "N40-31", "NS-01"]
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

In: pre-registration, the M3 reading on the design corpus, verdict and decision record. Out, with the beads that
have them: the design corpus and its frozen labels (BD-248); the validation of the analyses against the
heuristic arm, the GATE-ATEL-01 row and the discard audit on the corpus (BD-250); the KILL-STATIC decision
(N28a-22), which this bead reads and cites and never re-decides; the campaign-time screening audit (BD-345) and
the stage audit verb (N26b-05, M4).

## Technical approach

1. **Falsifier clauses** (any falsifies), on the design corpus (about 200 designs with frozen labels, planted
   defects, planted winners and planted single causes):
   a. `design check` verdicts predict simulated failure no better than a mass-and-torque heuristic: area under
      the ROC curve below 0.8, or less than 0.05 above the heuristic.
   b. The blinded audit of discarded strata finds a discarded design in the true top-k.
   c. The planted winner is discarded.
2. **Competing arm.** The mass-and-torque heuristic as BD-250 fixes it, given the same designs:
   s_heur = min over actuated joints j of τ_cont,j·N_j·η_j/(m·g·r_j), with τ_cont the card's continuous torque,
   N the gear ratio, η the forward efficiency, m the total mass, g gravity and r_j the horizontal lever from the
   joint to its limb's contact at the reference stance (the wheel radius for a driven wheel); one definition,
   owned by BD-250. **Potency controls.** A planted winner that the screen represents
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
- [ ] The verdict is recorded as `docs/adr/bet-05-m3.md` at M3 exit by a session that did not implement the
      analyses, and it cites the KILL-STATIC decision `docs/adr/ADR-KILL-STATIC-M3.md` (N28a-22) by identifier and
      hash, whatever its outcome; this bead writes no kill ADR, and with no decision on record the verdict is not
      filed.

## Test plan

### Unit tests
`crates/frob-conformance/tests/bets.rs` (N00-28's verdict function on `synthetic` gate records):
`bet05_auc_0_86_vs_0_78_holds` (difference lower bound 0.055); `bet05_auc_0_79_falsifies_a`;
`bet05_auc_gap_0_04_falsifies_a`; `bet05_discarded_design_in_top_k_falsifies_b`;
`bet05_planted_winner_discarded_falsifies_c`; `bet05_reads_kill_static_record` (N28a-22's record with outcome
`fired` → the bet's record carries that outcome and its hash, and the ADR names the fallback "advisory lints, no
pruning claim"; with no record on file the verdict is not filed).

### End-to-end script and logging
`scripts/e2e/bet_05_decide.sh` reads the gate row and the audit records with `cargo run -p frob-conformance
--example registry_report -- gates --chapter 28 --milestone M3 --json`, recomputes the counts, reads the verdict
of BET-05 from the bet register's report (`cargo run -p frob-conformance --example bets -- status --json`,
example `bets` of N00-28, the one verdict rule), checks that
`docs/adr/bet-05-m3.md` cites the same values and the KILL-STATIC outcome of N28a-22, and under a fired kill gate
checks that `frob design check <the corpus's planted top-heavy design> --json` exits 2 (not 3) with the
formerly refusing finding reported at severity High (the advisory form of N28a-22), and that no ladder record
lists an elimination by static verdict. Logged per STD-E2E (`target/e2e-logs/bet_05_decide/<utc>.ndjson`: `ts`,
`script`, `bead`, `step`, `event`, the exact command line, exit code, wall time, the envelope's `evidence`
block): both curves' areas, the paired bound against 0.05, AUC_check against 0.8, audit sample sizes and
findings, the planted cases' outcomes, the receipt hashes; on failure the first value that differs between the
ADR and the gate row; `summary` naming GATE-ATEL-01 and KILL-STATIC.

### Known answers and negative controls
KA-015 (1/(1 − 0.16) = 25/21 ≈ 1.19). CX-006. Negative controls: the planted winner; the planted defective and
sound designs; labels recomputed after seeing the screen's output (rejected by the freeze digest).

## Considerations

- The curve is computed against simulated failure under the baseline controller; a design that fails only
  because its controller is poor is a controller finding, and the label set records which is which.
- Top-k and the audit sample size are fixed at registration: k = 10 per archetype family (BD-250), and with no
  top-k design among m audited discards the discarded-winner rate is at most 1 − δ^(1/m) at confidence 1 − δ, so
  thirty clean audited discards bound it at 1 − 0.05^(1/30) = 9.5%.
- A screen can be predictive and still unsafe; clauses (b) and (c) are independent of (a).

## References

§4 (BET-05), §40.5, §2.2, §2.5, §28.23; GATE-ATEL-01; KILL-STATIC; KA-015; CX-006.
