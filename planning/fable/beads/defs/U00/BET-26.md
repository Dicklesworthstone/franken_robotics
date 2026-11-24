---
key: "BET-26"
title: "Decide BET-26: gait orbits as screening (periodic orbits continued in design parameters, on regular ground)"
type: "task"
priority: 1
milestone: "M4"
labels: ["m4", "reflex", "frob-gait", "frontier", "bet", "gate"]
parent: "WP-64"
deps: ["BD-328", "BD-333", "BD-335", "N00-28", "N22a-38", "N22c-17", "N38a-09", "STD-E2E"]
needs: []
related: ["GB-27", "N00-11", "N40-31"]
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

In: pre-registration of the bet, the M4 reading, the bet's verdict and its decision record
`docs/adr/bet-26-m4.md`. Out: the orbit solver, continuation, the catalogue; the measurement of GATE-GAIT-03
(BD-335); the decision KILL-GAIT itself, its ADR, its registry row and the application of its fallback
(N22c-17). This bead depends on both: it reads BD-335's recorded numbers and N22c-17's decision record,
evaluates the bet's four falsifier clauses on them and writes the bet's verdict. It decides no kill gate.

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

- [ ] Pre-registered with the catalogue digest and the calibration set: the bet's row in N00-28's register and
      BD-335's protocol record carry both, and the record's commit predates the first gate solve (checked here,
      since the registration itself is made by those beads, which this one follows).
- [ ] Median and distribution of solve times against 10 minutes, beside the published 4–260 s; the continuation
      success share against 50%; τ against 0.5 with its interval.
- [ ] Every planted grazing and unstable control was caught with a named reason (a grazing child refused with
      `FROB-REF-431` or marked at grazing, an unstable parent given the verdict `unstable`) and none carries a
      score in a screening table; one uncaught control falsifies clause c.
- [ ] Verdict recorded in `docs/adr/bet-26-m4.md`, which cites the KILL-GAIT decision on record in N22c-17's
      ADR and registry row (that decision is made there, not here); every orbit result carries the standing
      `screening`.

## Test plan

### Unit tests
`crates/frob-conformance/tests/bets.rs` (the bet register's rule of N00-28 on recorded values):
`bet26_median_3_min_success_0_62_tau_0_58_holds`; `bet26_median_12_min_falsifies_a`;
`bet26_continuation_success_0_45_falsifies_b`; `bet26_grazing_orbit_ranked_falsifies_c`;
`bet26_tau_0_38_falsifies_d`; `bet26_reads_kill_gait_record` (N22c-17's record: fired → `Falsified`, the record's
hash carried and the ADR naming the fallback "no design is ranked by an orbit"; the rung's removal is applied
there, not here).

### End-to-end script and logging
`scripts/e2e/bet_26_decide.sh` re-solves the rimless-wheel and compass-gait fixtures with N22a-38's spellings
(`frob gait orbit fixture:rimless-wheel --set spokes=8,l=0.5,slope_deg=4 --section post-impact --key
bet26-rw-<run-id> --json`; `frob gait orbit fixture:compass-gait --section auto --key bet26-cg-<run-id> --json`;
fresh run-scoped keys, so that each is a new solve whose time is measured and not the stored receipt of the gate's
run), runs one continuation (`frob gait continue --from <the reference trot's orbit> --param
thigh_len=0.20..0.22 --key bet26-gc-<run-id> --json`), submits one planted grazing child of BD-335's catalogue
(`frob gait orbit <child> --gait trot --key bet26-graze-<run-id> --json`; expects `FROB-REF-431`, exit 3, or an
orbit marked at grazing that carries no score), recomputes the timing and success statistics and τ from BD-335's
logged per-child records and the rank-agreement ledger, reads N22c-17's decision record and its hash, reads the
verdict of BET-26 from the bet register's report (`cargo run -p frob-conformance --example bets -- status
--json`, example `bets` of N00-28, the one verdict rule) and checks `docs/adr/bet-26-m4.md` against it (the
script writes no ADR). Logged per STD-E2E: fixture, residual, multipliers against their known values, solve time,
continuation steps, refusal code; `summary` naming GATE-GAIT-01, GATE-GAIT-03 and KILL-GAIT.

### Known answers and negative controls
KA-093, the rimless wheel with spoke half-angle α and leg length ℓ on a slope γ: the return map on the
post-impact rate is ω₊² ↦ c²·(ω₊² + b) with c = cos 2α and b = (4g/ℓ)·sin α·sin γ, so the fixed point is
ω*² = c²·b/(1 − c²) and the multiplier is c², exactly ½ for eight spokes (α = π/8); an uphill slope (b < 0) has no
fixed point and must return a typed outcome without an orbit (exit 2, no `GaitOrbit` record; N22a-38 step 3),
never a solved orbit. The test compares against these within 10⁻⁶ (GATE-GAIT-01, with proof obligation
PO-GAIT-01; KA-560 is the chapter's row of the same wheel). Share of a
training run: 4/410 = 1%, 260/410 = 63%. Negative controls: the planted grazing and unstable orbits; the
planted transverse instability; a solve reported as converged with its residual above tolerance.

## Considerations

- A loop with no clock has a free period and a trivial unit multiplier that must not be read as marginal
  stability; that case has its own gate (GATE-GAIT-04: the rimless wheel and compass gait pass through the
  autonomous path with the trivial multiplier removed and listed).
- Orbit screening applies to periodic gaits on regular ground; on rubble or stairs it says nothing and must say
  so.

## References

§4 (BET-26), §40.5, §0.8 (H-05), §22.18; GATE-GAIT-01, -03; KILL-GAIT.
