---
key: "BET-08"
title: "Decide BET-08: single precision is adequate on the GPU (anchoring, group updates, scaling, guarded predicates)"
type: "task"
priority: 0
milestone: "M2"
labels: ["m2", "spark", "frob-kernels", "solid", "bet", "gate"]
parent: "WP-22"
deps: ["N00-28", "BD-030", "BD-179", "BD-182", "STD-E2E"]
needs: ["§35.10: the double-float accumulator fallback and the list of quantities it may be applied to"]
related: ["BD-053", "BET-02", "N00-35", "N00-12", "N00-KA"]
gates: ["GATE-SPARK-05", "GATE-SPARK-18"]
plan_refs: ["§4 (BET-08)", "§40.5", "§35.8", "§35.10", "§0.8 (H-15)"]
---
## Background

**The bet.** GPUs are fastest in single precision, and the bet is that 32-bit floats are adequate for the batched
physics if four measures are taken: positions are stored as an integer cell plus a small offset (anchored
coordinates: cell side 8 m, offset in [−4, 4) m), so precision does not fall with distance from the origin;
rotations and poses are advanced by maps that stay on the group; the contact system is diagonally scaled before
it is solved; and every strict yes-or-no predicate is decided by a guarded comparison with an exact fallback.
Half precision is not used at all: the planning evidence refuted it (the condition number times half-precision
round-off is at or above one in 37–64% of sampled contact states, and refinement alone costs more than the
single-precision factorization).

**Why it matters.** Without it the GPU tier would run in double precision at a large throughput cost on consumer
cards, or produce long-horizon drift and wrong boundary decisions. A wrong strict predicate is not a small
error: it flips a verdict. Tier: Solid.

## Scope

In: pre-registration, the M2 reading, verdict and decision record. Out: the anchoring, scaling, guarded kernels
and the precision battery.

## Technical approach

1. **Falsifier clauses** (either falsifies):
   a. The precision battery fails at long horizons: constraint violation, group-membership error or energy error
      exceed their envelopes.
   b. A strict predicate decided in guarded single precision disagrees with exact integer evaluation.
2. **Competing arm.** `batch-cpu` in double precision on the same worlds. **Potency control.** A boundary screen
   on which unguarded single-precision comparison is known to err: 65,536 boundary cases with 452 false
   admissions and 23,438 false exclusions when unguarded; under the guard with the exact fallback it must end
   with zero errors (and the unguarded variant must show the errors, or the screen has no power).
3. **Deciding gates (TARGETs).**
   - GATE-SPARK-05 (Known answer, M2): over 10⁶ steps, group-membership error within 4 ulp, closure and contact
     violations inside their envelopes, energy-ledger residual under 10⁻⁴ relative, at distances up to 1 km from
     the origin.
   - GATE-SPARK-18 (Known answer, M2): on 10⁴ contact states per reference robot the relative error of the
     single-precision Newton step against double precision is at most 10·κ₂·u (κ₂ the 2-norm condition number of
     the scaled contact Hessian, u the single-precision unit round-off), and at most 10⁻³ on near-rigid states
     under the `stiff` profile; the estimated condition number never exceeds the κ₂ computed in double
     precision.
4. **No kill gate; the fallback is written.** Double-float accumulators (a pair of single-precision values
   carrying an error term) on the failing quantities; quantities that still fail are evaluated on `batch-cpu`
   and labelled as such on every result that used them.
5. **Downstream if falsified.** The affected quantities' kernels are replaced and the throughput parity rows are
   re-measured and disclosed; if strict predicates cannot be guarded bit-identically across backends, the
   guard's own kill gate applies (strict predicates evaluated exactly only on the CPU tiers, GPU tiers mark
   worlds that came within the error bound of a boundary, and the sampled referee re-runs them).
6. **Evidence so far (FACT about plan-time checks).** Humanoid contact Hessians: κ₂ of 3 × 10⁴–1 × 10⁵ raw and
   2 × 10³–9 × 10³ scaled at default impedance; 2–3 × 10⁶ raw and 2–5 × 10⁵ scaled near-rigid. The boundary
   screen ran identically on an M5 Ultra and two RTX 4090 for the integer and guarded kernels. Both are to be
   re-run in this repository.
7. **Procedure.** Register envelopes and the screen before the battery runs; read at M2 exit;
   `docs/adr/bet-08-m2.md`.

## Acceptance criteria

- [ ] Pre-registered with the envelopes of clause (a) as numbers.
- [ ] GATE-SPARK-05 and GATE-SPARK-18 recorded per backend with measured values.
- [ ] The boundary screen shows 452 and 23,438 errors unguarded and zero errors guarded, with identical bytes
      across backends.
- [ ] Verdict recorded; if falsified, the list of quantities moved to double-float accumulators or to
      `batch-cpu`, with the throughput cost disclosed.

## Test plan

### Unit tests
`bets.rs`: `bet08_within_envelopes_holds`; `bet08_group_error_6_ulp_falsifies_a`;
`bet08_one_predicate_disagreement_falsifies_b`; `bet08_energy_residual_2e_4_falsifies_a`;
`bet08_screen_without_unguarded_errors_is_untested`; `bet08_fallback_lists_quantities`.

### End-to-end script and logging
`scripts/e2e/bet_08_decide.sh` recomputes the verdict from the two gate rows and the screen's record and checks
the ADR. Logged per STD-E2E: per backend the maximum group error in ulp against 4, the energy residual against
10⁻⁴, the Newton-step error against 10·κ₂·u, the screen's four counts; `summary` naming GATE-SPARK-05 and
GATE-SPARK-18.

### Known answers and negative controls
KA-018 (the boundary screen counts). Arithmetic context: at κ₂ = 3 × 10⁶ and u ≈ 6 × 10⁻⁸, κ₂·u ≈ 0.18 raw;
at 5 × 10⁵ scaled, 0.03. Negative controls: the unguarded comparison; a battery run without anchoring at 1 km
(must exceed the group-error envelope); scaling skipped on near-rigid states (must violate GATE-SPARK-18).

## Considerations

- "Adequate" is defined by the envelopes, which are fixed before the run; a failed quantity is moved, not
  re-enveloped.
- Guards cost a small fallback rate; if that rate exceeds 10⁻² on the contact battery the guard's own kill gate
  decides.

## References

§4 (BET-08), §40.5, §35.8, §35.10, §0.8 (H-15); GATE-SPARK-05, GATE-SPARK-18; KA-018.
