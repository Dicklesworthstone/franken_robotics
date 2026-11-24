---
key: "BET-08"
title: "Decide BET-08: single precision is adequate on the GPU (anchoring, group updates, scaling, guarded predicates)"
type: "task"
priority: 0
milestone: "M2"
labels: ["m2", "spark", "frob-kernels", "solid", "bet", "gate"]
parent: "WP-22"
deps: ["BD-030", "BD-179", "BD-182", "N00-28", "N21bp2-01", "STD-E2E"]
needs: []
related: ["GB-04", "GB-22", "N00-12", "N00-35", "N00-KA", "N40-31", "N40-32"]
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

In: pre-registration, the M2 reading, verdict and decision record. Out, with the beads that have them: the exact
and guarded predicate kernels and the boundary corpus (BD-030); anchored positions, the precision battery and
the conditioning certificate, which record GATE-SPARK-18 and measure every clause of GATE-SPARK-05 but the energy
residual (BD-179); that clause and the GATE-SPARK-05 record (N21bp2-01); the guarded predicates on devices and
the contact-battery fallback rate that KILL-MARROW-GUARD reads (BD-182).

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
   guard's own kill gate KILL-MARROW-GUARD applies (M2; trigger: a fallback rate above 10⁻² on the contact
   battery on either GPU backend, or an exact path that cannot be made bit-identical across backends; fallback:
   strict predicates evaluated exactly only on the `strict` CPU profile and the referee, GPU tiers mark worlds
   that came within the error bound E of a boundary, and the sampled referee re-runs them). That kill gate is
   not this bet's: BET-08 has none, and its verdict only cites the guard's decision where it applies.
6. **Evidence so far (FACT about plan-time checks).** Humanoid contact Hessians: κ₂ of 3 × 10⁴–1 × 10⁵ raw and
   2 × 10³–9 × 10³ scaled at default impedance; 2–3 × 10⁶ raw and 2–5 × 10⁵ scaled near-rigid. The boundary
   screen ran identically on an M5 Ultra and two RTX 4090 for the integer and guarded kernels. Both are to be
   re-run in this repository.
7. **Procedure.** Register the envelopes of clause (a) and the screen before the battery's `f32` arms run: 4 ulp
   for group-membership error; 10⁻⁴ relative for the energy-ledger residual; for closure and contact violation
   the values E of BD-179's `battery/precision/envelopes.toml` (E = max(2·M₆₄, F), fixed from the `f64` arm;
   the file's hash goes into the pre-registration record). Read at M2 exit; `docs/adr/bet-08-m2.md`.

## Acceptance criteria

- [ ] Pre-registered with the envelopes of clause (a) as numbers.
- [ ] GATE-SPARK-05 and GATE-SPARK-18 recorded per backend with measured values.
- [ ] The boundary screen shows 452 and 23,438 errors unguarded and zero errors guarded, with identical bytes
      across backends.
- [ ] Verdict recorded; if falsified, the list of quantities moved to double-float accumulators or to
      `batch-cpu`, with the throughput cost disclosed.

## Test plan

### Unit tests
`bets.rs`, each on a `synthetic` gate registry: `bet08_within_envelopes_holds` (group error 3 ulp, energy
residual 4·10⁻⁵, violations inside E, screen 452 and 23,438 unguarded and 0 guarded → `Held`);
`bet08_group_error_6_ulp_falsifies_a` (6 ulp against 4 → `Falsified`, clause a);
`bet08_one_predicate_disagreement_falsifies_b` (one guarded decision differing from the exact integer one →
clause b); `bet08_energy_residual_2e_4_falsifies_a` (2·10⁻⁴ against 10⁻⁴);
`bet08_screen_without_unguarded_errors_is_untested` (unguarded counts 0 and 0: the screen has no power →
`Untested`); `bet08_fallback_lists_quantities` (a falsified record without the list of quantities moved to
double-float accumulators or to `batch-cpu` is rejected); `bet08_energy_clause_not_run_is_untested`
(GATE-SPARK-05 with its energy clause `not run` → `Untested`, never `Held`).

### End-to-end script and logging
`scripts/e2e/bet_08_decide.sh` runs `cargo run -p frob-conformance --example bets -- status --json` (the report
of the bet register, example `bets` of N00-28), recomputes the verdict of BET-08 from the two gate rows and the screen's record, and checks
that `docs/adr/bet-08-m2.md` states the same verdict with each clause's measured value. Logged per STD-E2E: per backend the maximum group error in ulp against 4, the energy residual against
10⁻⁴, the Newton-step error against 10·κ₂·u, the screen's four counts; `summary` naming GATE-SPARK-05 and
GATE-SPARK-18.

### Known answers and negative controls
KA-018 (the boundary screen counts). Arithmetic context: at κ₂ = 3 × 10⁶ and u ≈ 6 × 10⁻⁸, κ₂·u ≈ 0.18 raw;
at 5 × 10⁵ scaled, 0.03. Negative controls: the unguarded comparison; a battery run without anchoring at 1 km
(must exceed the group-error envelope); scaling skipped on near-rigid states (must violate GATE-SPARK-18).

## Considerations

- "Adequate" is defined by the envelopes, which are fixed before the run; a failed quantity is moved, not
  re-enveloped.
- Guards cost a small fallback rate; if that rate exceeds 10⁻² on the contact battery KILL-MARROW-GUARD
  decides (above).

## References

§4 (BET-08), §40.5, §35.8, §35.10, §0.8 (H-15); GATE-SPARK-05, GATE-SPARK-18; KA-018.
