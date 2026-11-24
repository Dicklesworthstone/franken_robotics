---
key: "BET-01"
title: "Decide BET-01: the algebra pays for itself (derived kernels within 1.3× of hand-tuned; types remove bug classes)"
type: "task"
priority: 0
milestone: "M2"
labels: ["m2", "spark", "frob-kir", "solid", "bet", "gate"]
parent: "WP-18"
deps: ["BD-217", "BD-221", "N00-28", "STD-E2E"]
needs: []
related: ["BET-02", "BET-11", "N00-11", "N00-15", "N40-31"]
gates: ["GATE-SPARK-03", "KILL-KIR"]
plan_refs: ["§4 (BET-01)", "§40.5", "§0.8 (H-04)", "§1.6 (N1)", "§21.6"]
---
## Background

**The bet.** A Lie-typed, derived formulation gives competitive speed and removes whole bug classes. Hot kernels
are not typed in by hand: each is the image of an algebraic object under a derivation that can be replayed, with
a content hash, emitted for a CPU reference, CUDA and Metal. The bet is that this costs little speed and buys
correctness: frames, signs and units are checked by types and by a dimension checker before a kernel exists.

**What it is not.** It is not a throughput lever. The Lie-algebraic recursions that derivation could specialize
are about 16–20% of an incumbent's step trace, so recursion-only specialization has a ceiling near 1.2×
(hypothesis H-04, REFUTED as a lever; reversal condition: a specializable share of at least one third on our own
profile). The bet is about paying for itself, not about winning speed.

**Why it matters.** If derived kernels were much slower than hand-written ones, the derivation toolchain would
be a tax on every throughput gate, and the differentiator "kernel arithmetic is derived once and emitted for
three backends with a stored disagreement record" would be bought at the cost of parity. Tier: Solid.

## Scope

In: pre-registration of the falsifier, the bet's verdict at M2 from recorded gates, and its decision record.
Out: the derivation toolchain, the kernels and the performance harness (their own beads). Also out: the
kill-gate decision KILL-KIR. The seed BD-221 measures the expressed share and the kernel slowdowns, applies the
trigger and writes `ADR-KILL-KIR` and the gate-register row. This bead depends on it, reads that record (the
per-kernel paired ratios are clause (a)'s data), evaluates the bet's two falsifier clauses and writes the bet
verdict; it applies no kill trigger and writes no kill ADR.

## Technical approach

1. **Falsifier clauses** (thresholds are TARGETs until measured; either clause falsifies):
   a. On the same backend, after one optimization pass, a derived kernel runs more than 1.3× slower than the
      hand-tuned equivalent of the same stage. The statistic is the one BD-221 applies for KILL-KIR, so that
      the bet and its kill gate never read the same timings two ways: whole-kernel time at 8,192 worlds, at
      least ten paired runs in randomized order back to back; with ℓᵢ = ln(t_derived,i / t_hand,i), the one-sided
      95% upper bound of the slowdown is exp(ℓ̄ + t·s/√n), where ℓ̄ and s are the mean and standard deviation of
      the ℓᵢ, n the number of pairs and t the one-sided 95% Student quantile (1.833 for n = 10). The clause
      fires if that bound exceeds 1.3 for any measured kernel on any backend: the bound has to be shown, not
      merely not refuted. "One optimization pass" is one pass of the emitter's exact improvements
      (common-subexpression sharing, the exact rewrite table, live-range scheduling, the derivation-time choice
      of algebraic form) guided by the profile; no reassociation and no fusion.
   b. Frame, sign and unit bugs still appear in conformance (Gauntlet) triage at the rate of an untyped
      baseline. Quantities (DESIGN, because the plan names the comparison and not the statistic): r_typed =
      frame, sign and unit defects found by triage during M2 in derived kernels, divided by the number of
      derived kernels; r_untyped = the same tally for the hand-written skeleton bodies and hand-tuned twins,
      divided by their number (BD-221 records both tallies; the hand-written code is the untyped baseline
      available in this project). The clause fires if r_untyped > 0 and r_typed ≥ r_untyped. If both tallies
      are zero the clause is recorded `untested: no defects in either arm` and the planted-defect control
      below carries the evidence on its own.
2. **Competing arm.** A hand-tuned kernel for the same stage on the same backend, written by an engineer given
   the same profile and the same pass budget. It must agree with the derived kernel inside 4 ulp per output, or
   it is rejected as an arm (they may differ in operation order).
3. **Potency control.** Planted frame defects (a transform applied in the wrong direction), sign defects (a
   negated cross product, a reversed coadjoint) and unit defects (a length added to a velocity), each of which
   the types and the dimension checker (`FROB-REF-250`) or a known answer bound to the kernel must reject before
   emission; a planted slowdown (a derived kernel whose body is executed twice per call, about 2×) that must
   make clause (a) fire; and the A/A null (a derived kernel timed against itself), whose bound must not exceed
   1.02.
4. **Deciding gate.** GATE-SPARK-03 (Performance, M2): the CUDA rows of the throughput table inside their
   tolerances, with the disclosure block and the paired lower bound. The kernel-level ratio of clause (a) is
   measured by the toolchain bead on the stages of the batched step (forward kinematics, twists, composite
   inertia, inverse dynamics, the Cayley step).
5. **Kill gate and fallback.** KILL-KIR, decided at M2: fires if the derivation toolchain cannot express or emit
   more than 20% of the kernel arithmetic, or if emitted kernels miss the 1.3× bound after one optimization
   pass. Fallback: arithmetic is hand-written in Rust and in the kernel dialect and bound by differential tests;
   kernel certificates then fall back as the proof bet's kill gate prescribes (exact-rational randomized tests
   recorded as Tested).
6. **Downstream if falsified.** The differentiator wording N1 is re-rendered without "derived"; the kernel
   certificate bet (BET-11) loses its subject for the affected kernels; derivation stays only where it supplies
   derivative kernels for CPU solvers and test oracles.
7. **Evidence so far (FACT about sources).** No measurement on this system. Incumbent traces put recursions at
   16–20% of the step. Sibling code proves the single-source GPU pattern outside this project.
8. **Procedure.** Register the clauses, arm and controls in the bet register before the first kernel timing;
   read the gates at M2 exit; write `docs/adr/bet-01-m2.md` with each clause's measured value.

## Acceptance criteria

- [ ] The bet is pre-registered (clauses with numeric thresholds, arm, controls, fallback) before any kernel
      timing is recorded.
- [ ] Clause (a) has a measured paired ratio per stage and backend, with the hand-tuned arm's source and pass
      budget recorded; clause (b) has the triage counts for typed and untyped baselines.
- [ ] Every potency control was caught in the same runs: every planted frame, sign and unit defect rejected
      before emission, the planted 2× slowdown made clause (a) fire, and the A/A bound was at most 1.02.
- [ ] The verdict (`Held`, `Falsified` with clauses, or `Untested`) is recorded as an ADR at M2 exit by a
      session that did not implement the toolchain, and it cites the KILL-KIR decision as recorded by BD-221
      (`ADR-KILL-KIR`, by identifier and hash), whatever that decision's outcome. If the decision is not on
      record, the verdict is not filed.

## Test plan

### Unit tests
`crates/frob-conformance/tests/bets.rs`: `bet01_bound_1_294_holds` (ten pairs, ln-mean 0.2, ln-standard
deviation 0.1: bound exp(0.2 + 1.833·0.1/√10) = 1.294 ≤ 1.3 on every stage, controls caught → `Held`);
`bet01_bound_1_320_falsifies` (one stage with ln-mean 0.22, same spread: bound 1.320 → `Falsified{a}`, although
the point ratio e^0.22 = 1.246 is under 1.3); `bet01_clause_b_rates` (typed 1 defect in 40 kernels against
untyped 4 in 40 → not fired; typed 3 in 40 against untyped 3 in 40 → `Falsified{b}`; 0 and 0 →
`untested: no defects in either arm`);
`bet01_reads_kill_kir_record` (given a KILL-KIR record with deciding share 18%, not more than 20%, and outcome
`fired`, the bet's record carries that outcome and the record's hash; the trigger itself is tested where it is
applied, in BD-221's `share_threshold`);
`bet01_pass_without_planted_slowdown_caught_is_untested`.

### End-to-end script and logging
`scripts/e2e/bet_01_decide.sh` reads the recorded gate rows and the toolchain's kernel-ratio records, recomputes
the verdict with the bet register's report (`cargo run -p frob-conformance --example bets -- status --json`, the
example target `bets` that the register bead N00-28 declares; the script reads the row of BET-01 from its output, since the report has no
per-bet option), and checks that the ADR `docs/adr/bet-01-m2.md` exists and matches. Logged per STD-E2E: per stage and backend the point ratio, the number of pairs, its one-sided 95%
upper bound and the 1.3 threshold; the A/A bound against 1.02; triage counts and the two rates; planted defects
and how many were rejected; controls caught; the verdict; on a mismatch between the recomputed verdict and the
ADR, the first differing field and the paths of the gate rows and the ADR; `summary` naming GATE-SPARK-03 and
KILL-KIR.

### Known answers and negative controls
The ceiling 1/(1 − 0.16) = 1.19 to 1/(1 − 0.20) = 1.25 for recursion-only specialization (context, not a
threshold of this bet). Negative controls: the planted frame, sign and unit defects; the planted slowdown; a
decision record written before the gate rows exist (rejected).

## Considerations

- A hand-tuned arm that is not competent makes the bet trivially true; the arm's author and pass budget are
  recorded, and the exit review may ask for a second arm.
- Compile-time cost of derivation is not in the falsifier but is disclosed with the result.
- If our own M2 profile shows a specializable share of at least one third, the specialization hypothesis is
  reopened by its own rule; that does not change this bet.

## References

§4 (BET-01), §40.5, §0.8 (H-04), §1.6 (N1); GATE-SPARK-03; KILL-KIR.
