---
key: "BET-14"
title: "Decide BET-14: the mechanisms need this engine (the composed-incumbent tie rule and the recovery ratio)"
type: "task"
priority: 1
milestone: "M7"
labels: ["m7", "tenfold", "frob-conformance", "solid", "bet", "gate", "owner-decision"]
parent: "WP-77"
deps: ["N00-28", "N00-19", "BD-315", "BD-317", "BD-430", "BD-432", "BD-433", "STD-E2E"]
needs: ["§39.22: three arms (ours; the composed incumbent; our layer over the incumbent engine), the recovery ratios r_T and r_C with 95% intervals, the three single-change phase ablations and the one-receipt pipeline for the layer arm (GATE-TENFOLD-13)", "§25.27: KILL-AUGUR-TIE, the tie rule read on the family strata"]
related: ["BET-13", "BET-19", "N00-14", "N00-12", "N00-15", "N00-20", "NS-24", "BD-301"]
gates: ["GATE-TENFOLD-06", "GATE-TENFOLD-07", "GATE-TENFOLD-13", "KILL-TENFOLD-TIE", "KILL-AUGUR-TIE"]
plan_refs: ["§4 (BET-14)", "§40.5", "§1.4", "§2.6", "§0.8 (H-19)", "§6.3", "§39.22", "§43.12"]
---
## Background

**The bet.** Almost every mechanism in this design can in principle be hosted by an incumbent engine with a
layer on top: static analyses, a ladder of rungs, paired statistics, adversarial search, mission queries with
conventional solvers. The bet is that this is not enough: a short robust program over an incumbent engine
cannot reach the same accepted answers, with the same checking scope, in comparable time. If it can, the honest
conclusion is that the composition was worth building as a layer and not as an engine. This is the second of
the three bets on which the thesis turns, and it is decided by measurement, not by a count of gaps.

**What is known about engine-internal gaps.** Two are documented in incumbents' own documentation: a
mixed-topology batch with run-to-run replay, and a Metal tier. Two are unconfirmed and must be checked on the
installed incumbents before anything relies on them: replay of one world outside its batch, and solver-internal
certificates. In the documents read, run-to-run determinism, heterogeneous worlds and the fastest solver sit on
different solver paths, so each is available and their conjunction is not shown. The hypothesis "a thin layer
over an incumbent would do" is ACTIVE: unrefuted and untested.

**The recovery ratio.** With arm A (this project's engine and layer), arm C (the strongest composed incumbent)
and arm W (this project's layer driving the incumbent engine through the same verbs, where the verbs allow it),
the recovery ratio says how much of A's gain over the commonly run incumbent the composed incumbent recovers,
on elapsed time (r_T) and on compute (r_C), each with a 95% interval. Tier: Solid as a measurement.

## Scope

In: pre-registration, the readings at M3 and M7 per stratum, the tie statement, the consequences and the
decision records. Out: the arms, the adapter through which rival stacks drive this environment, the ablations,
the estimator.

## Technical approach

1. **Falsifier clauses** (either falsifies, per stratum):
   a. The composed-incumbent arm reaches the same accepted scope with a restricted mean time within a factor of
      1.5 of ours.
   b. The composed incumbent's recovery ratio is at least 0.8 on elapsed time and on compute.
2. **Competing arms.** The composed incumbent; and our own layer over the incumbent engine through the same
   verbs, where the verbs allow it. **Potency controls.** The rival audit before sealing (each arm's frozen
   configuration reproduces its development-fixture figure within 10%); an easy win for conventional methods in
   every stratum, so that a harness in which the rival never wins is detected.
3. **Deciding gates.** Arm C of GATE-TENFOLD-06 (M3) and of GATE-TENFOLD-07 (M7); GATE-TENFOLD-13 (M3; M7 for
   the sealed suite): the recovery ratios with 95% intervals for every stratum that has a ratio; the three
   phase ablations on both briefs, each inside its bound; the one-receipt pipeline for arm W built first.
4. **Kill gates.** KILL-TENFOLD-TIE (M3; M7): the ablation arm reaches the same accepted scope within a factor
   of 1.5, or the recovery ratio is at least 0.8 on elapsed time and on compute. KILL-AUGUR-TIE on the family
   strata (first read on the docking slice at M1, decided at M3): a short robust program over a conventional
   solver reaches the same checked scope at comparable turnaround.
5. **What happens on a tie (the fallback, fixed now).** Say so in the report. Ship the layer over that engine
   for that stratum, openly and out of process, never as a hidden dependency of the core. Keep engine work only
   for the gaps internal to an engine (the two documented ones, and the two unconfirmed ones if the M0 re-audit
   confirms them). On the family strata, ship the conventional program behind the same verbs, schema and
   checker, and retire any claim of exclusive expressivity. Spend the research budget on the first mission the
   rival cannot resolve. If GATE-TENFOLD-13 is not met at M3, no engine-internal work package beyond M2 is
   funded.
6. **Downstream.** The composition differentiator (N13) is rendered with the tie statement for the stratum; the
   funding check refuses engine-internal speed levers for a tied stratum; the suite of M7 is re-stratified if
   whole classes tie.
7. **Evidence so far.** Every plan-time pilot's strong baseline tied or won (a lifted linear program tied the
   projection; a scenario tree tied the docking family; a dependency graph tied the semantic receipt;
   quantifier elimination produced the pusher answers). That is evidence for the tie on small plants and is the
   reason this bet exists.
8. **Procedure.** Register clauses, arms and the tie statement template before the M3 suite is sealed; read per
   stratum at M3 and M7; `docs/adr/bet-14-m3.md`, `bet-14-m7.md`.

## Acceptance criteria

- [ ] Pre-registered before sealing; arm W's one-receipt pipeline exists before arm A is timed.
- [ ] Per stratum: restricted mean times of arms A, C and W, the A-to-C ratio and interval, r_T and r_C with
      95% intervals, and equality of accepted scope stated explicitly.
- [ ] Each stratum is labelled `engine needed`, `tie`, or `not run` with the reason, and the consequence of a
      tie is applied (report text, shipping form, funding).
- [ ] The unconfirmed engine-internal gaps carry their M0 re-audit records before any text relies on them.

## Test plan

### Unit tests
`bets.rs`: `bet14_ratio_2_3_and_recovery_0_4_holds`; `bet14_ratio_1_4_falsifies_a` (within a factor of 1.5);
`bet14_recovery_0_85_on_both_falsifies_b`; `bet14_recovery_0_85_time_0_6_compute_holds` (clause b needs both);
`bet14_different_accepted_scope_is_not_a_tie`; `bet14_tie_refuses_engine_speed_funding_for_stratum`.

### End-to-end script and logging
`scripts/e2e/bet_14_decide.sh` recomputes the per-stratum labels from the gate records, checks the report
contains the tie statement for every tied stratum, and checks that the claims-wording registry renders N13
accordingly. Logged per STD-E2E: stratum, the three arms' restricted mean times, ratio and interval, r_T, r_C,
scope equality, label; `summary` naming the gates and both kill gates.

### Known answers and negative controls
Boundary arithmetic: a ratio of exactly 1.5 is "within a factor of 1.5" (tie); recovery exactly 0.8 on both is
a tie. Negative controls: a rival arm changed after its freeze (the pack digest changes and voids the arm); a
harness in which the easy-win missions are missing; a report that omits a tie.

## Considerations

- A tie is a result worth publishing, and the layer is a product; the bet's loss is not the program's failure.
- The comparison is only as fair as the rival's tuning; the rival audit and its sign-off are part of the record.
- Where the verbs cannot drive the incumbent engine, arm W is `not run` for that stratum with the reason.

## References

§4 (BET-14), §40.5, §1.4, §2.6, §0.8 (H-19), §39.22, §43.12; GATE-TENFOLD-06, -07, -13; KILL-TENFOLD-TIE;
KILL-AUGUR-TIE.
