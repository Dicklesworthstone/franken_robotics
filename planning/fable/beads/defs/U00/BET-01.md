---
key: "BET-01"
title: "Decide BET-01: the algebra pays for itself (derived kernels within 1.3× of hand-tuned; types remove bug classes)"
type: "task"
priority: 0
milestone: "M2"
labels: ["m2", "spark", "frob-kir", "solid", "bet", "gate"]
parent: "WP-18"
deps: ["N00-28", "BD-217", "BD-221", "STD-E2E"]
needs: ["§21.6: the hand-tuned comparison kernels for the stages named in the falsifier, and the planted frame, sign and unit defects of the dimension checker", "§38: the Gauntlet triage log from which frame and unit bug rates are counted"]
related: ["BD-018", "BD-019", "BD-149", "BD-150", "BET-02", "BET-11", "N00-11"]
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

In: pre-registration of the falsifier, the verdict at M2 from recorded gates, and the decision record. Out:
the derivation toolchain, the kernels and the performance harness (their own beads).

## Technical approach

1. **Falsifier clauses** (thresholds are TARGETs until measured; either clause falsifies):
   a. On the same backend, after one optimization pass, a derived kernel runs more than 1.3× slower than the
      hand-tuned equivalent of the same stage (paired lower bound of the time ratio above 1.3).
   b. Frame, sign and unit bugs still appear in verification triage at the rate of an untyped baseline.
2. **Competing arm.** A hand-tuned kernel for the same stage on the same backend, written by an engineer given
   the same profile and the same pass budget.
3. **Potency control.** Planted frame, sign and unit defects that the types and the dimension checker must
   reject before emission; a planted slowdown in a derived kernel that the paired comparison must detect.
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
- [ ] Both potency controls were caught in the same runs.
- [ ] The verdict (`Held`, `Falsified` with clauses, or `Untested`) and, if falsified, the KILL-KIR decision are
      recorded as an ADR at M2 exit by a session that did not implement the toolchain.

## Test plan

### Unit tests
`crates/frob-conformance/tests/bets.rs`: `bet01_ratio_1_25_holds` (all stage ratios ≤ 1.3 with controls caught →
`Held`); `bet01_ratio_1_34_falsifies` (one stage with paired lower bound 1.34 → `Falsified{a}`);
`bet01_emit_share_18_percent_fires_kill_kir` (expressible share below 20% → kill gate);
`bet01_pass_without_planted_slowdown_caught_is_untested`.

### End-to-end script and logging
`scripts/e2e/bet_01_decide.sh` reads the recorded gate rows and the toolchain's kernel-ratio records, recomputes
the verdict with `cargo run -p frob-conformance -- bets status --bet BET-01 --json`, and checks the ADR exists
and matches. Logged per STD-E2E: per stage and backend the ratio, its paired lower bound and the 1.3 threshold;
triage counts; controls caught; the verdict; `summary` naming GATE-SPARK-03 and KILL-KIR.

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
