---
key: "NAB-27"
title: "M4 exit: sweep of the decision records after rank agreement, family policies and mission queries are decided"
type: "task"
priority: 2
milestone: "M4"
labels: ["m4", "delivery", "gauntlet", "solid", "exit-review", "decision-record"]
parent: "EPIC-M4"
deps: ["BD-328", "N13-24", "N14b-31", "N17a-14", "N17b-20", "N19b-42", "N20b-38", "N21c-11", "N21c-18", "N22b-16", "N22b-28", "N22c-16", "N22c-17", "N23a-26", "N23a-30", "N23a-33", "N25b-17", "N25c-27", "N25c-40", "N25d-11", "N25d-15", "N25d-20", "N25e-17", "N25e-22", "N25e-26", "N28b-20", "N28b-36", "N28c-17", "N28c-41", "N28c-45", "N40-04", "NAB-22", "NAB-26", "NAB-50", "STD-E2E"]
needs: []
related: ["BD-378", "NAB-32", "NAB-36", "NAB-37", "NAB-42", "NAB-47"]
gates: []
plan_refs: ["Appendix C.1", "Appendix C.2", "Appendix B.5", "§43 (M4)", "§0.8"]
---
## Background

FrankenRobotics keeps its decision records as data (the expected-value matrix of 143 scored families, the
Moonshot queue, the killed-idea register of 67 entries, the idea-pass register, the source register) and
re-reads them at every milestone exit against the evidence that exit produced, under the rule that a verdict
changes only on new evidence of the kind that produced it. NAB-22 defines the procedure; this bead is the sweep
at the exit of **M4**: amortized control, co-design and mission queries at scale, whose aim is to decide many
designs while training few.

M4 is where the Frontier bets of the program are decided. The planning record calls the rank-agreement
experiment "the decisive early experiment": if cheap evaluation rungs do not rank designs as trained
specialists on the referee do, the ladder that lets most designs go untrained cannot discard anything. The
family-policy experiment, policy inheritance along edits, equivariant policies, gait orbits as a screening
rung, runtime shields, synthesis lanes and retrofits all have their kill gates here: twenty-one families of
the matrix and twenty-four kill-gate pairs, the largest sweep.

## Scope

In: the M4 sweep record and report; history entries; proposals. Out: deciding gates; the exit review (BD-378);
the procedure (NAB-22).

Order of work (DESIGN, as at M2 and M3 in NAB-25 and NAB-26). The sweep is made at the exit candidate commit,
before the exit record is written, and the exit review depends on it: the review's runner executes
`decision-sweep check --exit M4` as the attached check of NAB-22 and copies the sweep's section into the exit
record, so the record must exist first. What the sweep reads is produced before the review, not by it: the gate
report at that commit (N40-04); the kill-decision ADRs written by their decision beads (step 1's table names the
gate; the beads are N28b-36, N28c-17, N25d-11, N23a-30, N28b-20, BD-328 for KILL-RANK, N22c-17, N25b-17, N19b-42,
N21c-11, N22b-16, N22b-28, N22c-16, N23a-26, N23a-33, N13-24, N25d-20, N25d-15, N17b-20, N17a-14, N25c-27, N25c-40
and N21c-18); the late kill decisions of step 2's last paragraph (N25e-17, N25e-22, N25e-26, N28c-41, N20b-38,
N14b-31); and the reuse ledger (N28c-45) that KI-33 and KI-34 read. The dependency list names those beads for that
reason. This is the order of NAB-22's design decision 1, which every sweep follows; what the sweep shares with
the review is its independence rule (a session that implemented none of the cited beads).

## Technical approach

`cargo run -p frob-conformance --bin decision-sweep -- skeleton --exit M4` (the tool of NAB-22; below shortened
to `decision-sweep …`), fill, `decision-sweep check --exit M4`, by a session that implemented none of the cited
beads.

**1. Families due by a kill gate decided at M4.**

| Family | Now | Kill gate | Decided | Trigger (thresholds are TARGETs) | Fallback named in advance |
|---|---|---|---|---|---|
| EV-S26 Memo graph past model identity, with dependency-directed revalidation of verdicts | 8.0 | KILL-REUSE | M4 | Any unsafe reuse in GATE-ATEL-11, or grade 3 saves less than a fifth of the recheck cost grade 2 leaves | Grades 1 and 2 only |
| EV-S41 Value of an experiment in closed form, used as a stop rule | 4.5 | KILL-VOI | M4 | GATE-ATEL-15 fails | Value of information is a stop rule only; `experiment.suggest` lists proposals unranked by value |
| EV-F03 Monotone co-design: closure analysis and catalog lane | 8.0 | KILL-SYNTH-CODESIGN | M4 | The lane mis-ranks more than 20% of top designs against the referee, or prunes one independently feasible design | Restrict to the closure analysis A-10 for actuator, battery and compute selection |
| EV-F05 Inheritance of a policy along design edits | 6.0 | KILL-INHERIT | M4 | Median parameter-edit cost above 60% of from-scratch steps, or the inherited arm more than 5% worse in over 20% of edits | Every child trains from the class prior or from scratch; the mapping table is kept for loading policies |
| EV-F08 Rank agreement between rungs, the ladder allocation and conformal survivor sets | 5.0 | KILL-LADDER | M4 | GATE-ATEL-08 fails after one repair cycle | The fixed halving table is the allocation rule; the audit lane is kept as measurement |
| EV-F08 Rank agreement between rungs, the ladder allocation and conformal survivor sets | 5.0 | KILL-RANK | M4 | GATE-ATEL-07 fails for every morphology class tested | R1 is `uninformative` everywhere and every candidate that survives static analysis is scored on the higher rung; the ladder's claim shrinks to stopping and elimination among trained candidates; §39 records the lost factor |
| EV-F09 Gait atlas from symmetry | 4.5 | KILL-GAIT | M4 | GATE-GAIT-03 fails | Orbits serve stability diagnostics and warm starts only; the atlas keeps supplying clocks and reward references; no design is ranked by an orbit |
| EV-F11 Campaign quotienting by symmetry orbits and mission observables | 4.5 | KILL-AUGUR-CUTS | M4 | One false cut on the frozen catalog, or less than 70% of evaluation cost saved; or one quotient merge with different replayed traces | Cuts off: exhaustive evaluation with the static screen as an ordering heuristic. Quotient off: memoize on full model identity only |
| EV-F12 Retrofit catalog: priced edits of the world as repairs | 4.5 | KILL-RETROFIT | M4 | On the home battery kits flip fewer than 30% of the failures attributed to a building feature, or more than 10% of kit verdicts reverse on the referee | The catalog remains the edit vocabulary of Nemesis; the search verb is withdrawn |
| EV-F13 Division of labour between the two machines, and the fleet scheduler | 4.5 | KILL-FLEET | M4 | The pilot shows g < 1/3, or after one repair cycle the paired lower bound of campaign speedup at GATE-SPARK-13 is below 1.3 against the better single machine | The machines run standalone; jobs are assigned by the role table; the seeds-only rule stays |
| EV-F16 Energy-budget safety tier and runtime shields | 4.0 | KILL-ENERGY | M4 | An exceedance under the premises survives one repair cycle at GATE-SHIELD-01 | The claim is withdrawn; the monitor remains as an alarm of class `Estimated` |
| EV-F16 Energy-budget safety tier and runtime shields | 4.0 | KILL-SHIELD | M4 | GATE-SHIELD-03 fails after one repair cycle | The energy tier stays; barrier rows become reward penalties and alarms; nothing is called a shield |
| EV-F20 Sampling controller as a fallback and a source of demonstrations | 4.0 | KILL-MPC | M4 | GATE-REFLEX-06 fails after one repair cycle | The sampling controller is kept as a single-world CPU debugging aid; demonstrations come from the baseline and from specialists |
| EV-F22 Family policies per morphology class | 3.8 | KILL-FAMILY | M4 | At GATE-CORTEX-11, return with no adaptation below 70% of specialists on more than half of the held-out set and fine-tuning above 20% of specialist steps; or the pilot's falsifier holds | No family policy: specialists with inheritance from the nearest parent, and the baseline screens |
| EV-F25 Equivariant networks under the observation-preserving subgroup | 3.0 | KILL-EQUIV | M4 | GATE-CORTEX-03 fails (BET-07) | Symmetry is used for augmentation and metamorphic testing only |
| EV-F25 Equivariant networks under the observation-preserving subgroup | 3.0 | KILL-MARROW-BLOCKS | M4 | GATE-MARROW-04 fails after one repair cycle, or exact field arithmetic for n ≤ 12 exceeds its bit budget | Equivariant layers by Reynolds projection in `f64` with an idempotence test, classed `Tested`; the free-parameter count is read from the projection's rank |
| EV-F28 Hybrid gait orbits: saltation, return maps, labeled derivatives, continuation in design parameters | 3.0 | KILL-GAIT | M4 | GATE-GAIT-03 fails | Orbits serve stability diagnostics and warm starts only; the atlas keeps supplying clocks and reward references; no design is ranked by an orbit |
| EV-F30 Bounded exact linkage synthesis | 3.0 | KILL-SYNTH-LINKAGE | M4 | A certified linkage binds or changes circuit on the referee, or more than 25% of template instances exceed the caps | Multi-start numerical optimization of linkage dimensions with the same circuit and clearance check, classed `Tested` |
| EV-F31 Trajectory optimization with design variables, as local refinement | 3.0 | KILL-SYNTH-TRAJOPT | M4 | GATE-AUGUR-18 fails, or solves exceed the budget at 12 degrees of freedom | Continuous optimization of §28 with the baseline controller in the inner loop |
| EV-F33 Paired finite-difference design gradients over short horizons | 3.0 | KILL-FDGRAD | M4; the pilot runs before funding | The pilot's falsifier fires (variance ratio below 2 at every horizon of 50 steps or more, or sign agreement below 70%), or no end point is confirmed by a specialist | Rank-based search (CMA-ES, racing); the stencil is kept as a sensitivity report for A-16 |
| EV-F34 Design-family lanes under the bound 1/(1 − f + f/r + h) | 3.0 | KILL-FAMILYLANE | M4 | Measured whole-step gain below 1.15 on the mixed workload, or overhead h ≥ f·(1 − 1/r) | The general masked lane only, with shared structure tables kept |
| EV-F41 Dominance as unreachability in a product system | 2.0 | KILL-AUGUR-DOMINANCE | M4 | One mislabelled joint outcome, or a conventional reachability tool closes the same cells at similar cost | Paired comparison by replay under the studies of §24; the product lowering is kept only as the rule for space-indexed shared inputs |
| EV-F56 Label minimization from contact for a shared controller | 1.0 | KILL-AUGUR-LABELS | M4 | Deriving command sets costs more than labelling every part, or an inner set contains a failing command | The query is kept for supplied sets through a conventional solver; extraction from contact is parked |
| EV-P18 Specialized kernels for tournament survivors | 1.3 | KILL-SPECIALIZE | M4 | In the pilot the specialized kernel is below 1.2 times the data-driven one on the whole step, or takes more than 60 s to compile; or the M2 profile shows a specializable share under one third | Data-driven kernels only; derivation stays for correctness, certificates and the derivative kernels of CPU solvers |

Family and kill-gate pairs due at this exit: 24.

**2. Killed-idea records whose reversal condition names an M4 gate.**

| Record | Condition read at M4 | If met |
|---|---|---|
| KI-40 rank fidelity assumed between rungs | On our engine, per morphology class and task family: τ ≥ 0.7 near the top with the winner check passed (select); 0.5 to 0.7 (discard only); below 0.5 or no record (neither). A control variate needs ρ ≥ 0.89 for a fifth of the samples (1/(1 − ρ²) = 5) | The ladder's calibrated layer is licensed per class; otherwise the fixed halving table stays |
| KI-27 one universal policy | Within a class: return with no adaptation at 70% of specialists or better on at least half of the held-out set, or fine-tuning at 20% of specialist steps or less (the kill gate fires only when both fail) | Family policies per class stay; a universal policy stays refuted at an affordable budget |
| KI-28 policy inheritance for any edit | Median parameter-edit cost at most 60% of from-scratch steps, and the inherited arm not more than 5% worse in more than 20% of edits | Inheritance is the default for parameter edits |
| KI-30 gait continuation at one percent | Orbits rank designs: the orbit-screening gate | Orbits are a screening rung; otherwise diagnostics and warm starts |
| KI-31 proof-guided control for full-order robots | The energy-budget tier: no exceedance under the premises survives a repair cycle; the shield's measured margin with independent breach labels | Class Tested; the word "deployable" stays unused |
| KI-26 sampling control as a stand-in evaluator | The sampling controller at its gate; cost per episode against the break-even | A screening rung of three episodes per design by default |
| KI-39 value of information as a selection lever | Experiment suggestion at its gate; an A/B against capable agents | A stop rule and budget allocator at the finalist stage |
| KI-33, KI-34 exact reuse; reuse by sensitivity margin | The reuse ledger's exact-reuse column (the refutation predicts at most about 10% of compute; the hypothesis's own falsifier was 30%); any unsafe reuse; grade 3 saving at least a fifth of the recheck cost grade 2 leaves | Reuse by implication stays only if no unsafe reuse occurred |
| KI-41, KI-23 a family per dispatch; gradients through everything | The paired finite-difference pilot: variance ratio of at least 2 at some horizon of 50 steps or more, sign agreement of at least 70%, an end point confirmed | Short-horizon paired differences stay as a sensitivity report |
| KI-03 supercompiled per-design kernels | The survivor-specialization pilot: at least 1.2 times on the whole step within 60 s of compilation, given the M2 share | Specialization for survivors only |
| KI-49 Reynolds averaging; KI-48 quaternionic blocks | The equivariant blocks against the Reynolds projector as oracle; exact field arithmetic for n ≤ 12 within its bit budget | Blocks stay as a parameterization |
| KI-52 from mission to design by solving | The three synthesis lanes at their gates (bounded exact linkages; trajectory optimization at about 12 degrees of freedom within budget; catalog co-design mis-ranking at most 20% of top designs and pruning no feasible design) | Each lane stands or falls alone; every synthesized candidate still passes the ordinary ladder |
| KI-35 joint paving | The weak-variable enclosure measurement and per-constraint certificates on sub-boxes of about six parameters | Region certificates stay per constraint |

Kill gates of later-added mechanisms decided at M4 are listed by the tool: utility brackets and the dominance
stop, placement and sizing along a prescribed motion, the weak-variable split set, the controller-budget curve
(its oracle experiment), sensor mounts as design variables, the composed screen against batched evaluation.

**3. Contested ideas with their measurement at M4.** Three of the four standing disagreements of the planning
record are measured here: whether closed-loop design gradients can work (the paired finite-difference pilot);
whether randomized audits have a regime in which they beat coupled estimation (the tier-correction estimator:
a fitted, frozen control-variate coefficient with a doubly robust audit correction on logged propensities,
falling back to plain paired promotion when ρ is below 0.7 on 200 addresses); and whether orbit sharing saves
real work (quotient merges with identical replayed traces; at least 5% of dispatches saved). The sweep records
each as settled with its numbers, or as still open with the reason.

**4. Speed comparisons.** Every registered speed comparison of M4 must have recorded its ceiling
1/(1 − f + f/r + h) before the comparison ran, and a comparison whose ceiling is below its claim is refused.
The sweep lists the comparisons and their recorded ceilings; a family whose only evidence is a comparison with
no prior ceiling gets no change of score.

**5. Moonshot queue.** MQ-11 (label minimization from contact for a shared controller) becomes eligible after
M4, and only if its Frontier part (label queries on supplied command sets) passed its own gate. The sweep
calls `next_entries` and records the result.

**6. The no-family-policy variant.** If M4 exited in its variant with no family policy (compute envelope not
approved, or the kill gate fired), the family-policy and inheritance families are recorded `fell-back`
(specialists with inheritance from the nearest parent, and the baseline screens), and every family whose
adoption assumed a family policy is listed for re-scoring of Confidence with that reason.

**7. Idea register and sources.** Idea records take the state of their carriers' gates. Reading tasks for M4
gates are closed: the published transfer and inheritance figures (37% on held-out robots; about 10% with no
adaptation on one test morphology; 2 to 25% of a from-scratch run; the one parent-to-child figure of about 50%)
are re-read before the family-policy and inheritance gates cite them.

## Acceptance criteria

- [ ] `docs/delivery/exits/M4/decision-records.json` exists and `decision-sweep check --exit M4` passes.
- [ ] Each of the 24 family-and-kill-gate pairs of step 1 has an outcome citing its decision ADR.
- [ ] Each row of step 2 has measured values and `condition_met`.
- [ ] The three disagreements of step 3 each have a recorded state with numbers.
- [ ] Every speed comparison cited as evidence has a ceiling recorded before it ran.
- [ ] In the no-family-policy variant the two families read `fell-back` and their dependants are listed.
- [ ] The sweep record exists at the exit candidate commit before the M4 exit record is written, and the exit
      runner's attached check reads it; the sweep's session implemented none of the beads it cites.
- [ ] A family whose kill gate fired at M4 names its fallback in every registry of the decision records the same
      day (no family reads "adopted" under a fired gate).

## Test plan

### Unit tests

`crates/frob-conformance/tests/decision_sweep.rs`: `due_m4_families` (twenty-one ids: `EV-S26`, `EV-S41`,
`EV-F03`, `EV-F05`, `EV-F08`, `EV-F09`, `EV-F11`, `EV-F12`, `EV-F13`, `EV-F16`, `EV-F20`, `EV-F22`, `EV-F25`,
`EV-F28`, `EV-F30`, `EV-F31`, `EV-F33`, `EV-F34`, `EV-F41`, `EV-F56`, `EV-P18`; 24 pairs, because EV-F08, EV-F16
and EV-F25 each have two kill gates); `speed_evidence_needs_prior_ceiling` (a family re-scored on a comparison whose
ceiling was recorded after it ran → `check` fails `sweep.change-without-evidence`); `no_family_policy_variant_marks_fell_back`;
`rung_select_at_tau_0_6_is_discard_only` (a KI-40 entry licensing selection at τ = 0.6 → `check` fails; at 0.6 the
entry must read "discard only"; `sweep.change-without-evidence`); `ki28_needs_parameter_edits` (KI-28 marked met
from a topology-edit result → `check` fails `sweep.change-without-evidence`); `fired_gate_family_renamed`
(KILL-FAMILY fired, EV-F22 still reading `adopted` → `check` fails `sweep.kill-not-reflected`).

### End-to-end script and logging

`scripts/e2e/decision_records.sh --exit M4` (NAB-50): prints the due list, runs `decision-sweep check --exit M4`,
and asserts the 24 pairs, the conditions of step 2, the three disagreements of step 3 and the ceilings of step 4;
then replays the three negative controls below (fixtures under
`tests/fixtures/sweep/M4/controls/` written by this bead, the hook of NAB-50) on scratch copies of the sweep record and asserts that `check`
fails on each with its reason. Logged through the shared helper of STD-E2E to
`target/e2e-logs/decision_records/<utc>.ndjson` with a transcript, every line with `ts`, `script`, `bead`
(`NAB-27`), `step`, `event`: `cmd` lines with the exact command line, exit code and wall time; one `assert` per pair
with the kill gate, the ADR identifier and the outcome; one `metric` per condition of step 2 with the measured
number, its unit, the threshold and `condition_met`; one `metric` per speed comparison with its recorded ceiling
and claimed ratio; the digests of the gate report and of the sweep record; `assert` lines of kind
`negative-control` with `caught`; on failure the first due item without an outcome, or the first condition whose
number does not resolve to a receipt, with the path of the sweep record; a `summary` with assertions passed and
failed and the kill gates read.

### Known answers and negative controls

Known answers: twenty-one families, 24 pairs; 1/(1 − ρ²) = 1.96 at ρ = 0.7, 5.26 at 0.9 and 10.3 at 0.95, so a
fifth of the samples needs ρ ≥ √0.8 = 0.894; the design-family lane's ceiling is 1.10 to 1.32 and its kill
trigger is a measured whole-step gain below 1.15 or overhead h ≥ f·(1 − 1/r). Negative controls (each fails `sweep.change-without-evidence` by NAB-22 check (c)): a rung
licensed to select at τ = 0.6 (KI-40's selection threshold is `tau >= 0.7`; at 0.6 the entry must read "discard
only"); a family re-scored on a speed comparison with no prior ceiling (the pilot record does not resolve as
evidence); KI-28 marked met from a topology-edit result (its thresholds name parameter edits, so a measured value
under another name leaves them unmet).

## Considerations

- The rank-agreement result is per morphology class. The matrix has one row for it; the sweep's history entry
  carries the per-class table, and the row's Confidence reflects the worst class that the ladder serves.
- A failed Frontier bet here is the design working as intended: the fallback is already built. The sweep's job
  is to make every record say so in plain words.

## References

Appendix C.1, C.2 (entries on control, learning, search and statistics); Appendix B.5; the M4 milestone and its
exit review.
