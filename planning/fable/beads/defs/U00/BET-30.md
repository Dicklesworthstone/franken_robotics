---
key: "BET-30"
title: "Decide BET-30: equivariance under the observation-preserving subgroup (Γ_obs ≤ Γ_mech is the right group)"
type: "task"
priority: 1
milestone: "M4"
labels: ["m4", "cortex", "marrow", "sensorium", "frob-learn", "frontier", "bet", "gate"]
parent: "WP-29"
deps: ["BD-241", "BD-325", "BD-326", "BET-07", "N00-28", "N13-22", "N13-24", "N23a-33", "N38a-09", "STD-E2E"]
needs: []
related: ["N00-12", "N23b-02", "N40-31", "NS-05"]
gates: ["GATE-MARROW-03", "GATE-MARROW-04", "GATE-CORTEX-03", "KILL-EQUIV", "KILL-MARROW-BLOCKS"]
plan_refs: ["§4 (BET-30)", "§40.5", "§0.8 (H-24)", "§13.10", "§23.23"]
---
## Background

**The bet.** A robot's mechanical symmetry group Γ_mech (for example C₂ × C₂ for a quadruped that is symmetric
left to right and front to back) is not always the group a policy may be constrained to. A front-only depth
camera, a one-sided payload or a task with a preferred direction breaks part of it. The subgroup that also
preserves sensors, payload and task is Γ_obs ≤ Γ_mech (the observation-preserving subgroup). The bet is that
Γ_obs, not Γ_mech, is the right group for equivariant policies: constraining to the larger group on a robot with
asymmetric sensing costs return. Tier: Frontier.

**The Moonshot rider.** The same bet row carries the falsifier of a geometric-algebra tree policy [M]:
invariance to heading and to description conventions is necessary, augmentation recovers most of it, and there
is no locomotion evidence for the geometric-algebra specifics (expected gain 0–1.3×). That half is read at M8
by its own gate and kill gate and does not block this bead. Its reading and its record `docs/adr/bet-30-m8.md`
are the bead N00p1-11, which depends on this one: a bead closed at M4 cannot carry a reading due at M8.

## Scope

In: pre-registration; the M4 reading of the one question this bet adds to BET-07, namely whether Γ_obs and not
the larger Γ_mech is the group a policy must be tied under. It is read on the diagnostic arm of the three-arm
experiment (a robot with asymmetric sensing, clause (a)) and on the gates of the symmetry analysis and of the
exact block construction (GATE-MARROW-03, GATE-MARROW-04). Also in: this bet's M4 verdict and its decision
record `docs/adr/bet-30-m4.md`, with the Moonshot half recorded `pending M8`.

Out, and where it is:

- Whether an equivariant policy is worth having at all on a symmetric robot (more than 0.7× the steps of the
  unconstrained arm, or no better than augmentation): BET-07. Both bets read one record of GATE-CORTEX-03 and
  one decision KILL-EQUIV. Clause (b) here takes BET-07's verdict as its input and does not decide it again,
  which is why this bead depends on BET-07.
- The symmetry analysis (BD-102, BD-241, BD-246); the equivariant maps (BD-325); the experiment with its
  diagnostic arm (BD-326); the tree policy at M8 (BD-439).
- Applying the triggers of KILL-EQUIV (N23a-33) and KILL-MARROW-BLOCKS (N13-24) and writing their ADRs. This
  bead reads those decisions.
- The M8 reading of the Moonshot half: GATE-CORTEX-17 and the final reading of KILL-GATREE are measured and
  decided by N23bx-02 (first reading N23b-02); the bet's M8 verdict on that half and `docs/adr/bet-30-m8.md` are
  N00p1-11.

## Technical approach

1. **Falsifier clauses** (either falsifies the Frontier half):
   a. On a robot with asymmetric sensing, a policy constrained to Γ_mech does not lose return against one
      constrained to Γ_obs (then the distinction is unnecessary). Read on the diagnostic arm M of the three-arm
      experiment (BD-326): on the front-only-lidar variant of the reference quadruped (Γ_mech = C₂ × C₂,
      Γ_obs = C₂), arm M tied under Γ_mech through BD-326's labelled test-only override of the type barrier,
      against arm E tied under Γ_obs, ten paired seeds, final return at the common step cap. With
      dᵢ = return(M)ᵢ − return(E)ᵢ and τ the registered tolerance (2% of arm E's mean return, the tolerance BD-326
      uses for "no loss of final return"): the clause is **true** (no loss, bet falsified) when the paired
      one-sided 95% lower bound of d̄ is above −τ; it is **false** (loss shown) when the paired 95% upper bound
      of d̄ is below −τ; otherwise it is unmeasured and the bet is `Untested` on (a), with the interval printed.
      DESIGN: the same tolerance and pairing as BD-326's metric 2, so "loses return" means one thing in both
      bets.
   b. The Γ_obs policy fails the threshold of the symmetry bet (it does not reach the return in at most 0.7× the
      steps of the unconstrained arm).
   Moonshot half (read at M8 by N00p1-11, never here): the tree policy is falsified if it does not reach the
   threshold in 1/1.3 of the steps of a plain morphology transformer.
2. **Competing arms.** A Γ_mech-equivariant policy; augmentation only; a plain morphology transformer (for the
   Moonshot half). **Potency control.** Planted asymmetries (a moved sensor, a one-sided payload, a changed
   sole material, a reversed gear) for which the analysis must name the correct breaker.
3. **Deciding gates (TARGETs).**
   - GATE-MARROW-03 (Known answer, M3): Γ_mech and Γ_obs equal the groups on file for all eighteen archetypes
     and for 24 planted asymmetries, with the correct breaker named in every case; zero disagreements. Its
     complete record is written by N13-22, which this bead reads.
   - GATE-MARROW-04 (Known answer, M4): the equivariant-map construction holds exactly for every group of the
     character-table gate on 100 random pairs of channel spaces each; all eight registered mutations are caught
     by at least three fixtures each.
   - GATE-CORTEX-03 (Design comparison, M4): the three-arm experiment, ten paired seeds, 0.7× step ratio.
   - GATE-CORTEX-17 (M8, Moonshot; measured by N23bx-02, read for the bet by N00p1-11): one tree-policy arm is at
     least 1.3 times faster than the plain arm to the return threshold at equal wall-clock, or better with no
     adaptation beyond seed spread.
4. **Kill gates and fallbacks.** KILL-EQUIV (M4): the equivariance gate fails; symmetry is used for augmentation
   and metamorphic testing only. KILL-MARROW-BLOCKS (M4): the exact block construction fails after one repair
   cycle or exceeds its bit budget for n ≤ 12; equivariant layers by Reynolds projection in double precision
   with an idempotence test, classed Tested, the free-parameter count read from the projection's rank.
   KILL-GATREE (M8; N23b-02 and N23bx-02): the plain morphology transformer remains. The
   bet's own fallback: augmentation and metamorphic tests under Γ_obs.
5. **Downstream if falsified.** Under (a): policies may be constrained to Γ_mech where it is larger, and the
   Γ_obs computation remains an analysis (it is still needed for campaign quotienting and for sensing
   feasibility). Under (b): no equivariant layers ship; Γ_obs is used for augmentation and tests.
6. **Procedure.** Register the asymmetric-sensing robot, the arms, seeds and return target before any run; read
   at M4 exit; `docs/adr/bet-30-m4.md`. The M8 result is a second record, `docs/adr/bet-30-m8.md`, written by
   N00p1-11; this record is not edited after M4.

## Acceptance criteria

- [ ] Pre-registered with the robot, its Γ_mech and Γ_obs on file, and the planted asymmetries.
- [ ] The symmetry analysis names the correct breaker in 24 of 24 planted cases.
- [ ] Final return of the Γ_mech-constrained and Γ_obs-constrained arms with the paired interval of d̄ against
      −τ and the clause-(a) reading (true, false or unmeasured); the Γ_obs arm's step ratio against 0.7.
- [ ] Verdict recorded in `docs/adr/bet-30-m4.md`, citing the KILL-EQUIV (N23a-33) and KILL-MARROW-BLOCKS (N13-24)
      decisions on record by identifier and hash, and BET-07's verdict for clause (b); the Moonshot half marked
      `pending M8` with N00p1-11 named as its reader.

## Test plan

### Unit tests
`bets.rs` (with arm E's mean return 100, so τ = 2): `bet30_gmech_loses_return_and_gobs_ratio_0_64_holds`
(d̄ = −5.0 with paired 95% upper bound −3.1 < −2: loss shown; step ratio 0.64 ≤ 0.7);
`bet30_gmech_no_loss_falsifies_a` (d̄ = −0.4 with lower bound −1.5 > −2); `bet30_straddle_is_untested_a` (d̄ = −2.2,
interval [−4.0, −0.4]); `bet30_gobs_ratio_0_78_falsifies_b` (BET-07's verdict `Falsified` on the 0.7 clause is the
input); `bet30_wrong_breaker_named_fails_marrow03`; `bet30_moonshot_half_pending_does_not_block`;
`bet30_reads_kill_equiv_record` (N23a-33's record fired → `Falsified` through its kill gate, the record's hash
carried; the trigger is applied there).

### End-to-end script and logging
`scripts/e2e/bet_30_decide.sh` runs `frob analyze symmetry <variant> --json` and `frob sense symmetry <variant>
--json` on the front-only-lidar variant registered for the diagnostic arm (expects Γ_mech = C₂ × C₂ and Γ_obs =
C₂ with the lidar mount named as breaker); requests a reduction under Γ_mech on it through the verb whose row
lists the refusal, `frob phylum quotient --designs <set holding the variant> --task <the registered task> --group
mech --json` (expects FROB-REF-161, exit 3, naming the generator that moves the lidar), then asks for weights tied
under Γ_mech at the weight-tie site of the same code (N23a-31), `frob train start <variant> --task <the registered
task> --tying equivariant --group mech --dry-run --key b30-tie-mech --json` (expects FROB-REF-161, exit 3,
`observed` Γ_mech of order 4, `required` Γ_obs of order 2, the lidar generator named; with KILL-EQUIV fired,
`equivariant` is not admitted and the call answers FROB-REF-724, exit 5, logged as that branch); recomputes the
arm comparison from the ledger; reads the verdict of BET-30 from the bet register's report
(`cargo run -p frob-conformance --example bets -- status --json`, example `bets` of N00-28, the one verdict rule)
and checks `docs/adr/bet-30-m4.md` against it. Logged per STD-E2E: design, groups as generator lists,
breakers, per-seed returns and steps per arm, ratios; `summary` naming the gates and kill gates.

### Known answers and negative controls
The quadruped with a front-only depth camera or lidar: Γ_mech = C₂ × C₂ (order 4), Γ_obs = C₂ (order 2),
breaker = the sensor's mount (CX-118). Negative controls: the 24 planted asymmetries; a declared symmetry that
differs from the computed one (`quad-mammal` with a forward lidar and `assert_obs: C2xC2` is refused,
FROB-REF-109, with the lidar mount as breaker: KA-276, CX-133); a weight tie requested under Γ_mech on a
symmetry-breaking robot or task outside the labelled override, which must be refused (FROB-REF-161, group not
observation-preserving).

## Considerations

- Γ_obs depends on the task as well as the body; it is computed per robot and task, never cached per robot
  alone.
- At most four Moonshot items are active at once; the tree-policy half waits its turn.

## References

§4 (BET-30), §40.5, §0.8 (H-24), §13.10, §23.23; GATE-MARROW-03, -04; GATE-CORTEX-03, -17; KILL-EQUIV;
KILL-MARROW-BLOCKS; KILL-GATREE.
