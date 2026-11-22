---
key: "BET-26"
title: "Decide BET-26: gait orbits as screening (periodic orbits continued in design parameters, on regular ground)"
type: "task"
priority: 1
milestone: "M4"
labels: ["m4", "reflex", "frob-gait", "frontier", "bet", "gate"]
parent: "WP-64"
deps: ["N00-28", "N00-07", "BD-333", "BD-335", "BD-328", "STD-E2E"]
needs: ["§22.9: the gait catalogue with planted grazing and unstable orbits, and the calibration set of designs with specialist results"]
related: ["BD-332", "BD-334", "BET-17", "N00-11"]
gates: ["GATE-GAIT-01", "GATE-GAIT-03", "KILL-GAIT"]
plan_refs: ["§4 (BET-26)", "§40.5", "§0.8 (H-05)", "§22.18"]
---
## Background

**The bet.** A steady gait is a periodic orbit of a hybrid system (continuous motion punctuated by contact
events). Instead of simulating many episodes, one can solve for the orbit directly by shooting, read its
stability from the Floquet multipliers of its return map, and then follow the orbit as a design parameter
changes (pseudo-arclength continuation), obtaining cost of transport and stability as functions of the design.
The bet is that this is a cheap *screening* signal for periodic gaits on regular ground. It is never a
referee-ready controller and never acceptance evidence. Tier: Frontier.

**What the evidence supports.** Published full-order orbit solves take about 4 to 260 s of CPU at 20 to 68
states, which is about 1% to 63% of a 410 s training run: cheap at the low end, not at the high end. Orbits can
graze (a foot barely touching, where the return map is not differentiable) or be unstable; both must be
recognized and refused, not ranked.

## Scope

In: pre-registration, the M4 reading, verdict and decision record. Out: the orbit solver, continuation, the
catalogue.

## Technical approach

1. **Falsifier clauses** (any falsifies):
   a. The median orbit solve takes more than 10 minutes of CPU.
   b. Continuation fails on half the catalogue.
   c. A grazing or unstable control case is not refused.
   d. The orbit rung fails the rank-agreement bet for the class (its ranking of designs does not agree with
      specialists well enough to screen).
2. **Competing arm.** Baseline-controller rollouts without orbit analysis. **Potency control.** Grazing and
   unstable orbits planted in the catalogue; a planted transverse instability that a restricted computation
   would miss.
3. **Deciding gates (TARGETs).**
   - GATE-GAIT-01 (Known answer, M4): the rimless-wheel fixed point and multiplier within 10⁻⁶; compass-gait
     multipliers within 10⁻⁶ of an independent high-precision computation; the planted transverse instability
     is found and the restricted computation is marked incomplete.
   - GATE-GAIT-03 (Design comparison, M4): continuation succeeds on at least 50% of 10% design perturbations;
     median solve at most 10 minutes of CPU; Kendall τ of at least 0.5 between orbit cost of transport and
     specialists on the calibration set; the measured solve cost is recorded beside the published range.
4. **Kill gate and fallback.** KILL-GAIT (M4): GATE-GAIT-03 fails. Fallback: orbits serve stability diagnostics
   and warm starts only; the gait table and max-plus timing analysis keep supplying clocks and reward
   references; no design is ranked by an orbit.
5. **Downstream if falsified.** The orbit rung leaves the evaluation ladder; the gait atlas (which gaits a
   symmetry group permits, with phase lags) is unaffected; the design-gradient-through-orbit verb is offered
   with labels only for diagnostics.
6. **Procedure.** Register the catalogue, the perturbation size (10%), the calibration set and the CPU-time
   accounting (one core, wall time, the machine fingerprint) before any solve; read at M4 exit;
   `docs/adr/bet-26-m4.md`.

## Acceptance criteria

- [ ] Pre-registered with the catalogue digest and the calibration set.
- [ ] Median and distribution of solve times against 10 minutes, beside the published 4–260 s; the continuation
      success share against 50%; τ against 0.5 with its interval.
- [ ] Every planted grazing and unstable orbit was refused with a named reason.
- [ ] Verdict and the KILL-GAIT decision recorded as an ADR; every orbit result carries the standing
      `screening`.

## Test plan

### Unit tests
`bets.rs`: `bet26_median_3_min_success_0_62_tau_0_58_holds`; `bet26_median_12_min_falsifies_a`;
`bet26_continuation_success_0_45_falsifies_b`; `bet26_grazing_orbit_ranked_falsifies_c`;
`bet26_tau_0_38_falsifies_d`; `bet26_kill_gait_removes_rung`.

### End-to-end script and logging
`scripts/e2e/bet_26_decide.sh` re-solves the rimless-wheel and compass-gait fixtures with `frob gait orbit`,
runs one continuation with `frob gait continue`, submits the planted grazing orbit (expects a refusal),
recomputes the timing and success statistics and τ from the ledger, and the verdict. Logged per STD-E2E:
fixture, residual, multipliers against their known values, solve time, continuation steps, refusal code;
`summary` naming GATE-GAIT-01, GATE-GAIT-03 and KILL-GAIT.

### Known answers and negative controls
The rimless wheel on a slope has a closed-form fixed point and a single non-trivial multiplier (the squared
cosine of the inter-spoke angle 2α governs the speed lost at each impact: the post-impact angular speed is
cos 2α times the pre-impact one); the test compares against the derivation file within 10⁻⁶. Share of a
training run: 4/410 = 1%, 260/410 = 63%. Negative controls: the planted grazing and unstable orbits; the
planted transverse instability; a solve reported as converged with its residual above tolerance.

## Considerations

- A loop with no clock has a free period and a trivial unit multiplier that must not be read as marginal
  stability; that case has its own gate.
- Orbit screening applies to periodic gaits on regular ground; on rubble or stairs it says nothing and must say
  so.

## References

§4 (BET-26), §40.5, §0.8 (H-05), §22.18; GATE-GAIT-01, -03; KILL-GAIT.
