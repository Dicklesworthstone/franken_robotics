---
key: "BET-02"
title: "Decide BET-02: one kernel source, three backends, inside declared determinism classes; worlds independent of batch"
type: "task"
priority: 0
milestone: "M2"
labels: ["m2", "spark", "frob-kernels", "solid", "bet", "gate"]
parent: "WP-46"
deps: ["N00-28", "BD-017", "BD-176", "BD-177", "BD-181", "STD-E2E"]
needs: ["§35.14: the two negative controls of the invariance test (a planted fused multiply-add contraction in one backend; a planted read of another world's rows)"]
related: ["BD-020", "BD-203", "BD-204", "BET-08", "BET-09", "N00-15", "N00-14", "NS-02", "NS-10"]
gates: ["GATE-SPARK-01", "GATE-SPARK-02", "GATE-SPARK-15", "GATE-SPARK-16", "KILL-D1"]
plan_refs: ["§4 (BET-02)", "§40.5", "§35.1", "§35.5", "§1.2", "§1.6 (N6)"]
---
## Background

**The bet.** Derived arithmetic plus a shared kernel dialect keeps the CPU reference, CUDA and Metal inside
their declared determinism classes; the measured disagreement is stored per kernel; and a world's trajectory
does not depend on the batch it ran in. Compiling one source for several backends is prior art; the classes and
the stored record are the bet.

**Determinism classes** (every result records one). D0 exact: integer, rational or symbolic results. D1 bitwise
across backends: the same bits on `batch-cpu` in single precision, on CUDA and on Metal. D2 bitwise per backend:
the same bits run to run and across worker counts on one device family, and across batch size, slot, co-tenant
worlds, bucket capacity and shard count. D3 tolerance: agreement inside a recorded envelope plus agreement of
task decisions. D4 unspecified: never cited as evidence. The weakest input wins, and a class is never raised by
relabelling, only by running again under a stricter profile.

**Why it matters.** Incumbents document opt-in bitwise run-to-run determinism for the same batch on the same
GPU, and state that CPU and GPU results usually differ; one solver's own tests hold a world equal to its
isolated run only to 10⁻⁵–10⁻⁴. The two properties this project would add are bit equality between a device and
the CPU reference (D1) and independence of a world from its batch. On them rest replay of any world alone on the
CPU, paired comparison on common world addresses, counterfactual forks and attribution. Tier: Solid.

## Scope

In: pre-registration, the readings at M0 (the arithmetic probe) and M2 (kernels, the full step, the invariance
test, decision-level equivalence), the per-device-family verdict and the decision record. Out: the probe,
kernels, digest machinery and tests themselves.

## Technical approach

1. **Falsifier clauses** (either falsifies, per device family):
   a. Unexplained cross-backend disagreement above the class envelope survives two repair cycles.
   b. The batch-composition invariance test fails.
2. **Potency controls.** A planted contraction (a fused multiply-add) in one backend, and a planted read of
   another world's rows, must both be detected. A planted bias of one decision in fifty must fail the
   decision-level test.
3. **Deciding gates (TARGETs).**
   - GATE-SPARK-01 (M0): the IEEE probe per device family: exhaustive bit comparison of single-precision square
     root and reciprocal over all 2³² inputs; 10⁹ sampled operand pairs for +, −, ×, ÷ with subnormal and
     overflow edges; the contraction detector; subnormal and NaN behaviour recorded. The outcome sets each
     family's best attainable class.
   - GATE-SPARK-02 (M2): every M2 kernel and the full step agree between `batch-cpu` and each device inside the
     declared class: bit-equal for D1; for D3 at most 4 ulp per kernel output, statistical equivalence of
     1,000-step rollouts, and the decision-level test.
   - GATE-SPARK-15 (M2): 1,000 episodes per reference workload: per-world digests identical alone, in 100 random
     co-tenancies, under slot permutation, at two bucket capacities and on one and two shards, on `batch-cpu`
     and each device family; draws depend on address alone; both negative controls caught.
   - GATE-SPARK-16 (M2): every D3 placement pair used as evidence, on at least 4,096 frozen worlds per reference
     workload: upper bound on discordant decisions at most 1%; net one-sided discordance at most 0.5%;
     continuous outcomes equivalent inside ledger margins; first divergences recorded.
4. **Kill gate and fallback.** KILL-D1 (M2): if GATE-SPARK-01 or -02 shows a device family cannot hold bit
   equality with `batch-cpu` for the floating kernels of the step, that family is D2 for itself and D3 against
   others, with the decision-level test; a teleported world is a tolerance replay; cross-family comparisons are
   unpaired; the replay wording is withdrawn for that family.
5. **Downstream if falsified.** Clause (a) alone: KILL-D1 as above, and differentiator N6's bit-for-bit sentence
   is withdrawn for the family. Clause (b): replay of a world taken out of its batch is a tolerance replay on
   that family; paired statistics are refused or labelled `paired-by-address` with the realized correlation;
   attribution and forks are refused on it; if no family holds invariance, M2 falls back to its no-GPU variant.
6. **Evidence so far.** None on this system. A plan-time check ran one integer decision kernel with identical
   labels on an M5 and two RTX 4090 (exact-integer kernels only; says nothing about floating kernels).
7. **Procedure.** Register before the probe runs; first reading at M0 exit (probe only: sets attainable
   classes), final reading at M2 exit; decision record `docs/adr/bet-02-m2.md` per device family.

## Acceptance criteria

- [ ] Pre-registered before GATE-SPARK-01 is measured.
- [ ] A verdict per device family (CUDA, Metal): the class attained against `batch-cpu`, the stored
      per-kernel disagreement record, and the invariance result.
- [ ] Both planted controls and the one-in-fifty bias were caught in the recorded runs.
- [ ] KILL-D1 is recorded as fired or not fired per family, with the wording consequence applied through the
      claims-wording registry.

## Test plan

### Unit tests
`bets.rs`: `bet02_all_gates_pass_cuda_holds`; `bet02_metal_d3_only_is_held_in_part_with_kill_d1` (GATE-SPARK-02
reports 3 ulp on Metal: family verdict D3, KILL-D1 fired for Metal, bet `HeldInPart`);
`bet02_invariance_failure_falsifies_b`; `bet02_discordance_1_2_percent_fails_spark16`;
`bet02_control_not_caught_is_untested`.

### End-to-end script and logging
`scripts/e2e/bet_02_decide.sh` reads the four gate rows per family, recomputes the verdict and checks the ADR
and the rendered N6 wording. Logged per STD-E2E: per family the attained class, kernels compared, maximum ulp,
digest-equality counts across co-tenancies, discordance bounds against 1% and 0.5%, controls caught; `summary`
naming the four gates and KILL-D1.

### Known answers and negative controls
Divergence growth context: one unit in the last place grows like e^(λt) and reaches order one after about
ln(2²⁴)/λ ≈ 16.6/λ seconds. Negative controls: the planted contraction; the planted cross-world read; the
planted decision bias; a relabelled class with no re-run (rejected).

## Considerations

- A family may hold D1 for algebraic kernels and not for elementary functions; the verdict is stated per kernel
  class, and the weakest governs a step that mixes them.
- "Two repair cycles" is counted in the decision record so the bet cannot stay open indefinitely.
- Driver and compiler updates can change a verdict; it is re-read whenever the hardware fingerprint changes.

## References

§4 (BET-02), §40.5, §35.1, §35.5, §1.2, §1.6 (N6); GATE-SPARK-01, -02, -15, -16; KILL-D1.
