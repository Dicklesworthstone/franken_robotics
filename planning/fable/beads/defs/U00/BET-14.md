---
key: "BET-14"
title: "Decide BET-14: the mechanisms need this engine (the composed-incumbent tie rule and the recovery ratio)"
type: "task"
priority: 1
milestone: "M7"
labels: ["m7", "tenfold", "frob-conformance", "solid", "bet", "gate", "owner-decision"]
parent: "WP-77"
deps: ["BD-315", "BD-317", "N00-19", "N00-28", "N25a-18", "N39b-07", "N39b-13", "N39b-43", "N39b-44", "N39b-45", "N40-15", "N40-16", "N40-22", "STD-E2E"]
needs: []
related: ["BET-19", "N00-10", "N00-12", "N00-14", "N00-15", "N00-20", "N32ax-02", "N39b-05", "N39b-11", "N39b-46", "N39b-47", "N40-31", "N40-32", "N41-15", "N41-RISK2", "NS-24"]
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

**The arms and the recovery ratio.** The names are those of the ten-fold test and are the same in the kill-gate
bead N39b-07, which this bead reads. Arm A is this project's engine and layer (written F in the recovery
ratio). Arm B is the declared incumbent as it is commonly run: the strongest composed rival, a named incumbent
engine with per-design reinforcement learning and a standard outer-loop optimizer. Arm C is the ablation arm: a
short robust program (at most 2,000 lines) over the engine of arm B. Arm C is the first form of the composed
incumbent, written W in the recovery ratio: the same engine with every portable method layered on by a
competent baseline engineer, built as one pipeline before arm A exists and not held to 2,000 lines where a
portable method needs more. The tie test and the recovery ratio are both computed on that arm. Arm D, optional,
is this project's own layer driving the incumbent engine through the same verbs, where the verbs allow it (the
"second arm" of the bet's text; open question OPEN-TENFOLD-9, default: not built unless arm C ties on a
stratum; decided in the ADR of N39b-07). With T_X the restricted mean time of arm X on a stratum and C_X its
GPU-hours, r_T = (T_B − T_W)/(T_B − T_F) and r_C = (C_B − C_W)/(C_B − C_F): how much of F's gain over the
commonly run incumbent B the composed incumbent W recovers, on elapsed time and on compute, each with a 95%
interval, defined when F beats B on that measure. Tier: Solid as a measurement.

## Scope

In: pre-registration; the readings at M3 and M7, per stratum, of one question: does the composed incumbent, or
this project's layer over the incumbent engine, reach the same accepted scope within a factor of 1.5, or
recover at least 0.8 of the gain on elapsed time and on compute. Also in: the tie statement, the consequences,
and the decision records `docs/adr/bet-14-m3.md` and `docs/adr/bet-14-m7.md`.

Out, and where it is:

- The size of the ratio against the rival and where the word "ten-fold" may be used: BET-13, which reads the
  same records of GATE-TENFOLD-06 and GATE-TENFOLD-07 for a different question.
- What a tie on the family strata means for mission queries themselves: BET-19. Both bets read the one decision
  KILL-AUGUR-TIE; this bead takes from it only whether the stratum counts as a tie for the engine question.
- The tournament seen from the user's side (NS-24).
- The arms and the adapter through which rival stacks drive this environment (BD-315); the three-arm
  statistics (N39b-43); the one-receipt pipeline of arm W (N39b-44); the ablations (N39b-45); the record of
  GATE-TENFOLD-13 (N40-22).
- Applying the triggers of KILL-TENFOLD-TIE (N39b-07) and KILL-AUGUR-TIE (N25a-18) and writing their ADRs. This
  bead reads those decisions and depends on both beads: "the kill decision recorded" in this bead's records
  means on record by those beads, cited by ADR identifier, never re-decided here.

## Technical approach

1. **Falsifier clauses** (either falsifies, per stratum):
   a. The composed-incumbent arm reaches the same accepted scope with a restricted mean time within a factor of
      1.5 of ours.
   b. The composed incumbent's recovery ratio is at least 0.8 on elapsed time and on compute.
2. **Competing arms.** The composed incumbent (arm C, W in the recovery ratio); and our own layer over the
   incumbent engine through the same verbs, where the verbs allow it (arm D, reported when run). **Potency controls.** The rival audit before sealing (each arm's frozen
   configuration reproduces its development-fixture figure within 10%); an easy win for conventional methods in
   every stratum, so that a harness in which the rival never wins is detected.
3. **Deciding gates.** Arm C of GATE-TENFOLD-06 (M3) and of GATE-TENFOLD-07 (M7); GATE-TENFOLD-13 (M3; M7 for
   the sealed suite): the recovery ratios with 95% intervals for every stratum that has a ratio; the three
   phase ablations on both briefs, each inside its bound; the one-receipt pipeline for arm W built first.
   The recovery-ratio reading, registered before the sealed suite opens (TARGETs): r_T ≥ 0.8 and r_C ≥ 0.8 on a
   stratum → `tie` (a thin layer over an incumbent is enough there; clause (b)); both below 0.5 → `engine
   needed` on that measure; otherwise `mixed`, reported with the phase the layer could not recover, named from
   the phase ledger. Three cases are fixed in advance: T_F ≥ T_B (or C_F ≥ C_B) → `no saving to recover`, r
   undefined, and the stratum is not a tie by clause (b); T_W < T_F → r above 1, the stratum is a loss to the
   composed incumbent and is labelled `tie` a fortiori; an unresolved mission costs H in every arm. An ablation
   whose ratio exceeds its bound (1/s for an intervention that leaves a share s of elapsed time untouched, 1.43
   at s = 0.7; 1/(1 − s) for one that removes it, 3.33) voids the stratum as an accounting error, label
   `void: ablation bound`. The word "ten-fold" is governed by F against the stronger of B and W, never by F
   against B alone (that wording is BET-13's).
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
- [ ] Per stratum: restricted mean times of arms A (F), B and C (W), and of arm D where it was run; the ratio
      RMT_C ÷ RMT_A and its interval; r_T and r_C with 95% intervals; and equality of accepted scope stated
      explicitly.
- [ ] Each stratum is labelled `engine needed`, `tie`, `mixed` (with the unrecovered phase named), `no saving
      to recover`, `void: ablation bound`, or `not run` with the reason, and the consequence of a tie is applied
      (report text, shipping form, funding).
- [ ] The unconfirmed engine-internal gaps carry their M0 re-audit records before any text relies on them.

## Test plan

### Unit tests
`bets.rs`: `bet14_ratio_2_3_and_recovery_0_4_holds`; `bet14_ratio_1_4_falsifies_a` (within a factor of 1.5);
`bet14_recovery_0_85_on_both_falsifies_b`; `bet14_recovery_0_85_time_0_6_compute_holds` (clause b needs both);
`bet14_different_accepted_scope_is_not_a_tie`; `bet14_tie_refuses_engine_speed_funding_for_stratum`;
`bet14_recovery_0_6_and_0_3_is_mixed`; `bet14_f_not_faster_than_b_is_no_saving` (T_F ≥ T_B → r undefined,
label `no saving to recover`); `bet14_w_faster_than_f_is_tie` (T_W < T_F → r > 1); `bet14_known_answer_0_28`
(T_F = 0.10·T_B: T_W = 0.28·T_B gives r_T = 0.8 exactly, a tie on time; T_W = 0.29·T_B gives 0.789, not);
`bet14_ablation_above_bound_voids_stratum` (s = 0.7, information-only ratio 1.6 > 1.43).

### End-to-end script and logging
`scripts/e2e/bet_14_decide.sh` recomputes the per-stratum labels from the gate records, checks the report
contains the tie statement for every tied stratum, and checks that the claims-wording registry renders N13
accordingly. Logged per STD-E2E: stratum, the three arms' restricted mean times, ratio and interval, r_T, r_C,
scope equality, label; `summary` naming the gates and both kill gates. The three logged arms are A (F), B and C (W); arm D is a fourth column
when run.

### Known answers and negative controls
Boundary arithmetic: a ratio of exactly 1.5 is "within a factor of 1.5" (tie); recovery exactly 0.8 on both is
a tie. Negative controls: a rival arm changed after its freeze (the pack digest changes and voids the arm); a
harness in which the easy-win missions are missing; a report that omits a tie.

## Considerations

- A tie is a result worth publishing, and the layer is a product; the bet's loss is not the program's failure.
- The comparison is only as fair as the rival's tuning; the rival audit and its sign-off are part of the record.
- Where the verbs cannot drive the incumbent engine, or arm D was not opened (its default), arm D is `not run`
  for that stratum with the reason. The composed incumbent W is never optional: its one-receipt pipeline is
  built first.

## References

§4 (BET-14), §40.5, §1.4, §2.6, §0.8 (H-19), §39.22, §43.12; GATE-TENFOLD-06, -07, -13; KILL-TENFOLD-TIE;
KILL-AUGUR-TIE.
