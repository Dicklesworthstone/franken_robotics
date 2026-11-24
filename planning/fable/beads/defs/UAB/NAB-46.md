---
key: "NAB-46"
title: "registries/ideas.toml: ideas IP-49 to IP-88 placed on a single proposal or audited reasoning pass"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "delivery", "gauntlet", "solid", "registry", "decision-record"]
parent: "EPIC-M0"
deps: ["NAB-42"]
needs: []
related: ["NAB-47", "NAB-48", "NAB-49"]
gates: []
plan_refs: ["Appendix B.4", "Appendix B.2"]
---
## Background

The ideas behind FrankenRobotics were generated in several independent passes and winnowed by
cross-examination. Besides the ideas on which passes converged (NAB-44, NAB-45), forty ideas were placed on the
strength of **a single pass**, or arose in an audited reasoning pass (a pass that checked claims against
sibling source code or by a small executed experiment). They are in the system because the mechanism is sound
at the stated tier, not because several planners agreed. This is the other half of the rule that convergence
is evidence about a problem and not about a mechanism: lack of convergence is not evidence against one.

This bead carries them into the idea-pass register (`registries/ideas.toml`, schema `frob.ideapass/1`,
NAB-42) as records `IP-49` to `IP-88`, continuing the numbering of the converged table. Each row gives the
idea, the mechanism in one line, where it lives, its tier ([S] Solid, [F] Frontier, [M] Moonshot), its gates,
kill gates and bets, and the beads that carry it. Thresholds are TARGETs owned by their gates. Many of these
are small and Solid (twenty rows have no kill gate because they have no optional part); a few are the most
distinctive mechanisms of the system (the mission-observable quotient; discriminating-instrument design; the
blinded qualification lane; the ceiling-before-speed rule). Tier of the bead: Solid.

## Scope

In: records `IP-49` to `IP-88` with carriers. Out: the schema and lint (NAB-42); rows 1 to 48 (NAB-44,
NAB-45); the contested records (NAB-47); the not-yet-placed list (NAB-49).

## Technical approach

**Transcription rule.** As in NAB-44, with `proposals = 1` (or 0 for an idea that arose in an audited
reasoning pass; the planning table does not distinguish, so 1 is stored and `proposals_note = "single pass or
audited reasoning pass"`). `contested` links: IP-70 (joint paired-outcome certification) and IP-74 with IP-75
(catalog co-design by monotone bounds). `ungated` is filled for the eight rows that name no gate or, at an
upper tier, no kill gate, from their gates column and the notes below: IP-50 (the [F] screens are never trusted
without a passed screening audit, GATE-ATEL-09 and GATE-NEMESIS-07, and each cascade stage has its own kill
gate), IP-60 (a rule checked at compile time; its claim as a throughput lever is KI-08), IP-69 (the pilot
falsifier of §18.17), IP-73 (the hand-computed eigenvalue test of C-10), IP-79 (the known answer of §25.21),
IP-80 (static half only; dynamic recovery stays `Tested`), IP-81 and IP-83 (gated by a convergence or
known-answer gate, GATE-GAIT-01 and GATE-PHYLUM-08).

**Notes the one-line rows compress.**

- IP-49: FACT by enumeration on the ballast fixture: 735,471 designs fall into 24,546 classes (a ratio of
  29.96 that is a property of that fixture; in general the factor is 1).
- IP-51: the invariant is that an edit recompiles only the subterms it touches and the result equals full
  compilation bit for bit. The scene-side half (incremental geometry rebuilds) is not placed (NAB-49).
- IP-53: the limb's articulated inertia at its port is positive semidefinite, not definite, and the summary is
  exact only without base-coupled contact.
- IP-60: checks at compile time and none at run time is a rule; its claim as a throughput lever is a killed
  idea (KI-08).
- IP-61: the [F] advection half has its own kill gate, KILL-ADVECT (§19.10.7; BET-36 names KILL-FILM,
  KILL-ADVECT and KILL-LIVING), which the row's gates column omits; `kill_gates` adds it, and `gate_text` stays
  verbatim.
- IP-65: whether idle fixtures sleep is an open question (OPEN-KIN-2).
- IP-69: tactile micro-patches are a Moonshot whose gate is the falsifier of its pilot.
- IP-73: the test is that the max-plus eigenvalue of a trot schedule equals the stride period computed by
  hand; the exact-reachability half is a killed idea (KI-54).
- IP-74: the kill trigger is one false cut on the frozen catalog, or under 70% of evaluation cost saved.
- IP-78: the refusal that returns a minimal conflicting requirement set is `FROB-REF-877`, with a dual
  certificate where the conflict is linear.
- IP-79: its known answer is realizing a target pseudo-inertia by nonnegative masses on admissible sites, or
  returning an exclusion.
- IP-80: the static half only; dynamic recovery after an actuator loss stays evidence class Tested.
- IP-82: the i-th confirmation request runs on fresh sealed clusters at level αᵢ = α/[i·(i + 1)].
- IP-88: compute the ratio a test could show from its shared fixed work and refuse the test if that is below
  the claim; enumerate the verdicts a frozen rule can reach at its sample size.

**The rows.**

| Id | Idea | Mechanism in one line | Where it lives | Tier | Gates, kill gates, bets | Carried by |
|---|---|---|---|---|---|---|
| IP-49 | Mission-observable quotient | Project a design onto exactly what a mission experiment can observe; designs with equal projections and equal reduced inputs share one run | Phylum §17.11 (the digest); C-22; Atelier §28.15 | [F] | GATE-PHYLUM-01, on the ballast fixture (FACT by enumeration, §17.18: 735,471 designs fall into 24,546 classes); KILL-QUOTIENT | BD-285, N10b-12, N17a-25 |
| IP-50 | Algebra applied at campaign time | Use algebra to remove simulations from a campaign (orbits, enumeration, screens with audits), not to shave the step | §2.4; Phylum §17.11; Atelier §28.11, §28.12; C-22 | [S] audits; [F] screens | GATE-ATEL-09 (planted winner); GATE-NEMESIS-07; BET-31 | BD-049, BD-285, BD-345, BD-352 |
| IP-51 | Incremental compilation keyed to typed edits | An edit recompiles only the subterms it touches, and the result equals full compilation bit for bit | Skeleton §14.7 to §14.9 | [S] | INV-SKEL-02; GATE-SKEL-02. The scene-side half is in the not-yet-placed list (NAB-49) | BD-058, BD-063 |
| IP-52 | Trajectory optimization with design variables | Direct transcription for one motion and a fixed contact schedule, with continuous design parameters among the unknowns; the output is a labelled candidate and never a score | Augur §25.21.2 | [F] | GATE-AUGUR-18; KILL-SYNTH-TRAJOPT | BD-362; N25d-15 (KILL-SYNTH-TRAJOPT) |
| IP-53 | Limb port interface | A limb acts on its parent through its articulated inertia and bias wrench at the port; the inertia is positive semidefinite, not definite | Skeleton §14.6; Kinesis §16.6 (tree-operator route) | [F] | PO-SKEL-04; KILL-SKEL-PORT | BD-385, N14a-22, N14a-23; N14a-24 (KILL-SKEL-PORT) |
| IP-54 | Contact compaction with a capacity rule | Count, scan and write compaction at fixed capacity per world; an overflow is flagged and never silently clamped | Kinesis §16.6 (`K-compact`); Traction §18.11 | [S] | GATE-KIN-07 | BD-152, BD-168, N21a-08, N18b-22 |
| IP-55 | Factor-once response batches | At a converged step, reuse the factor to solve many impulse or wrench interventions on a fixed active set, each column confirmed against residual and mode predicates | Traction §18.5 (`contact.response`) | [F] | KILL-RESPONSE | BD-367, N18b-40; N18a-26 (KILL-RESPONSE) |
| IP-56 | Tree-structured factorization | Let limbs eliminate independently and couple through the base, with a dense fallback where contact couples them | Kinesis §16.6 (tree-operator route) | [F] | KILL-TREEOP | BD-385, N16a-24; N16a-23 (KILL-TREEOP) |
| IP-57 | Specialized kernels for survivors | Fold one design's constants into the generic kernel only for tournament survivors, bit-identical to the generic kernel | Phylum §17.9; Spark §21.18 | [F] | GATE-SPARK-14; KILL-SPECIALIZE; NONGOAL-13 | BD-371, N21c-16, N21c-17 |
| IP-58 | Determinism engineered instead of asserted | Stable-key sort and segmented reduction, per-tick digests, a determinism class on every receipt, and refusal of paired statistics under an uncontrolled class | §35.1 to §35.7; Spark §21.8, §21.15 | [S] | GATE-SPARK-08, -15, -16, -17; KILL-D1 | BD-152, BD-176, BD-177, BD-181, BD-203, BD-269 |
| IP-59 | Multirate thermal state at power ports | Thermal nodes advance on a slower rational clock and receive the heat accumulated since their last tick | Sinew §15.5, §15.7 | [S] | GATE-SINEW-05 | BD-071, BD-072, N15a-06 |
| IP-60 | Checks at compile time, none at run time | Frames, units and proofs are checked when the model compiles; batched kernels carry no tags and no proof objects | C-02, C-14; Spark §21.7 | [S] | Its claim as a throughput lever is in the killed-idea register | BD-018, BD-047, SPEC-C-02, SPEC-C-14 |
| IP-61 | Film-friction law and robot-advected films | Friction on a wetted region depends on dwell time, load and pad size; a conservative film field is carried and spread by the robot | Traction §18.8; Habitat §19.10 | [S] law; [F] advection | GATE-TRAC-05; GATE-HAB-06, -07; KILL-FILM; BET-36 | BD-230, BD-413; N18a-33, N18a-34 (KILL-FILM), N19b-11 (KILL-ADVECT) |
| IP-62 | Slosh, cords and rugs by reduced models, with coupled islands for finalists | Sprung masses for slosh, chains and hinged plates for cords and rugs; resolved fluid, cloth and rods only beside one world on the CPU referee | Habitat §19.10 | [F] proxies and slosh; [M] islands | GATE-HAB-08, -09; KILL-SLOSH, KILL-PROXY, KILL-LIVING | BD-231, BD-414, BD-415, BD-416; N19b-15, N19b-19, N19b-20 (kill decisions) |
| IP-63 | Robot and building retrofit co-design | A priced catalog of scene amendments; find the smallest kit that flips a verdict, checked by the same solvability oracle | Habitat §19.12; NS-18 | [F] | GATE-HAB-11; KILL-RETROFIT; BET-37 | BD-369; N19b-42 (KILL-RETROFIT) |
| IP-64 | Sparse brick fields with analytic sharp-edge overlays | A shared pool of distance bricks under a dense directory per scene, with exact overlays for nosings, kerbs and thresholds | Habitat §19.3, §19.4, §19.15 | [S] | GATE-HAB-01, -13; KILL-POOL | BD-162, BD-163, BD-164, BD-166 |
| IP-65 | Articulated interior fixtures | Doors, drawers, gates, lifts and carts as articulated models in the scene | Habitat §19.3, §19.7 | [S] | GATE-HAB-02. Whether idle fixtures sleep is OPEN-KIN-2 | BD-225, BD-229 |
| IP-66 | Sustained-duty thermal limits | Winding temperature, resistance rise and the runaway condition decide whether a duty holds for a shift | Sinew §15.7; A-11 | [S] | GATE-SINEW-05; GATE-AUTOPSY-06 | BD-072, BD-103 (A-11), BD-283 |
| IP-67 | Discriminating-instrument design | Return the cheapest realizable experiment that separates design decisions, or two indistinguishable worlds that demand different designs; every verdict is conditional on the stated law family | Augur §25.14; C-39 | [F] | GATE-AUGUR-11; KILL-AUGUR-PROBE | BD-291; N25c-14 to N25c-17 |
| IP-68 | Calibration diagnosis on the sensor graph | A least-squares pose graph over overlapping sensor views; the residual pattern localizes a bad node where the graph permits | Sensorium §20.14 | [S] | GATE-SENS-11 | BD-242 |
| IP-69 | Tactile micro-patches | A fingertip pad as a small array of four-dimensional contacts feeding the sensor models | C-37; Traction §18.17 (pilot table) | [M] | The pilot falsifier of §18.17 | BD-418, N18b-35 |
| IP-70 | Joint paired-outcome certification | Compile a fixed-controller comparison of two designs into one product system with shared, space-indexed inputs, and close parameter boxes where every compatible branch pair has the same joint outcome | Augur §25.16; C-40 | [F] | GATE-AUGUR-16; KILL-AUGUR-DOMINANCE. Contested, the contested-ideas record (NAB-47) | BD-359, SPEC-C-40; N25c-27 (KILL-AUGUR-DOMINANCE) |
| IP-71 | Schedule and timing-race check | Enumerate orderings of sensor capture, packet arrival, estimator update, watchdog and actuator commit up to commutation, and test the daemon under seeded schedules | Sensorium §20.4.7; Chronicle §29.15 | [F] | GATE-CHRON-11; KILL-SCHED | BD-096, BD-189; N20a-09 to N20a-11 (KILL-SCHED) |
| IP-72 | Kernel reflection for proof obligations | Checkers proved sound inside the proof kernel discharge instance certificates by evaluation | Lemma §12.1, §12.4 (`prove.reflect`) | [F] | GATE-LEMMA-05; KILL-REFLECT | BD-382; N12-32 (KILL-REFLECT) |
| IP-73 | Max-plus timing | A footfall or task schedule as a max-plus linear system whose eigenvalue is the cycle time | C-10; Marrow §13.1; A-04 | [F] | The test of C-10: the eigenvalue of a trot schedule equals the stride period computed by hand. The exact-reachability half is in the killed-idea register | BD-332, SPEC-C-10 |
| IP-74 | Requirements-to-morphology search with checked cuts | Fill typed holes in a design term by branch and bound, pruning only with a bound that holds for every completion of a node | Augur §25.7 | [F] | KILL-AUGUR-CUTS: one false cut on the frozen catalog, or under 70% of evaluation cost saved | BD-357, N25b-13; N25b-16, N25b-17 (KILL-AUGUR-CUTS) |
| IP-75 | Monotone co-design over part catalogs | Component selection as a least fixed point over antichains of resources | Augur §25.21.1; C-12; A-10 | [S] catalog closure; [F] antichains | GATE-AUGUR-17; KILL-SYNTH-CODESIGN. Contested, the contested-ideas record (NAB-47) | BD-052, BD-103 (A-10), BD-360, N25d-09 (pilot); N25d-11 (KILL-SYNTH-CODESIGN) |
| IP-76 | Numerical continuation with certified roots | Solve one generic instance, track parameter homotopies, certify each root, and label the result `Complete` only when a trace test agrees | Augur §25.21.4; C-34 | [M], on the CPU | GATE-AUGUR-22; KILL-SYNTH-CONT | BD-437; N25d-23 to N25d-25 |
| IP-77 | Inverse static-stance family | Return the whole family of allowable stance dimensions and foot ratings for a load hull and a declared actuator authority, typed exact, inner or outer | Augur §25.13; A-08 | [S] | GATE-AUGUR-02 | BD-100 (A-08), BD-130, N25c-11 |
| IP-78 | Minimal conflicting requirement set | When the predicates conflict with the world family for every design, return a minimal conflicting subset, with its dual certificate where linear | Augur §25.24 (FROB-REF-877) | [S] | GATE-AUGUR-01 | BD-126, N25b-07 |
| IP-79 | Mass placement inside a chassis volume | Realize a target pseudo-inertia by nonnegative masses on admissible sites, or return an exclusion | Augur §25.21; §11.5 | [F] | The known answer of §25.21 | N25d-07 (pilot), N25d-08 |
| IP-80 | Single-actuator-loss robustness | A rank screen on the remaining wrench map, then a bounded allocation program, for each catalogued removal | Augur §25.21 | [F] | Static half only; dynamic recovery stays `Tested` | N25d-07 (pilot), N25d-08 |
| IP-81 | Symmetry-breaking bifurcations | During continuation, a multiplier that leaves the unit circle in a non-trivial symmetry block is labelled with the broken generator, and the lower-symmetry branch is offered as a seed | Reflex §22.9 | [F] | GATE-GAIT-01 | BD-335, N22a-32 |
| IP-82 | Blinded qualification with an error budget across retries | Discovery cannot read confirmation worlds; the i-th confirmation request runs on fresh sealed clusters at level αᵢ = α/[i·(i + 1)] | Arena §24.13; §37.5 | [S] | GATE-QUAL-01, -02, -03; GATE-SAFE-03; BET-21 | BD-259, BD-260, BD-261 |
| IP-83 | Quality-diversity maps per dispatch | One generation of typed edits, limb additions included, is one mixed batch under the class's family policy | Phylum §17.12; Atelier §28.8 | [F] | GATE-PHYLUM-08 | BD-342, N28b-09 |
| IP-84 | Regime-wise comparison report | Cluster paired-episode differences by contact signature and scene region, and name the regimes where a candidate gained or regressed | Atelier §28.5 (`explain.compare`); A-26 | [S] report; [F] atlas | GATE-ATEL-16; KILL-ATLAS | BD-262, BD-348 (A-26), N28a-17; N28a-15 (KILL-ATLAS) |
| IP-85 | Gait report by symmetry mode | Project joint trajectories onto the isotypic components of the observation-preserving subgroup and report energy per component | Atelier §28.5 | [S] | GATE-ATEL-16 | BD-112, N28a-17 |
| IP-86 | Correlated tolerance stacks | Low-dimensional correlated tolerances on datums and mounts, propagated to clearance and backlash | Foundry §31.11 | [F] | GATE-FOUNDRY-08; KILL-TOLERANCE | BD-406, N10c-07 |
| IP-87 | Forecast feedback | The calibration of registered predictions decides when an agent is told to buy evidence before its next edit | Atelier §28.17 | [S] | GATE-ATEL-16 | BD-344, N28c-15 |
| IP-88 | Ceiling before a speed test; reachability before a fixed-n study | Compute the ratio a test could show from its shared fixed work and refuse the test if that is below the claim; enumerate the verdicts a frozen rule can reach at its n | §36.5; Arena §24.12 (`eval.preanalyze`); Tether §32.6.1 | [S] | GATE-AUGUR-14; GATE-ARENA-05; GATE-ATEL-06 | BD-124, BD-354, N40-18 |

## Acceptance criteria

- [ ] Records `IP-49` … `IP-88` exist with the columns verbatim and carriers as listed; with NAB-44 and NAB-45
      the register holds 88 `IP` records.
- [ ] Every carrier key exists as a bead; every gate and kill gate named exists in the gate register (23
      distinct kill gates in these records: 22 in the gates column, plus KILL-ADVECT added to IP-61).
- [ ] The twenty rows with no kill gate are Solid, or state the fallback or the limit in `ungated` (IP-50: the
      screening audit; IP-60: a rule, its lever claim killed; IP-69: the pilot's falsifier; IP-73: a
      hand-computed test; IP-79, IP-80, IP-81, IP-83: gated by a known answer or a convergence gate).
- [ ] `FROB-REF-877` exists in the refusal registry.
- [ ] The traceability lint of NAB-42 passes on these records.

## Test plan

### Unit tests

`crates/frob-conformance/tests/idea_register.rs`, module `single_proposal`:

- `forty_records`; `ids_49_to_88`; `total_ip_records_is_88`.
- `kill_gates_are_23` (22 from the gates column and KILL-ADVECT on IP-61); `rows_without_kill_gate_are_20`;
  `ungated_rows_are_8` (IP-50, IP-60, IP-69, IP-73, IP-79, IP-80, IP-81, IP-83).
- `tiers`: 19 rows name [S], 23 name [F], 3 name [M] (IP-62, IP-69, IP-76).
- `ballast_fixture_ratio`: 735,471 / 24,546 = 29.963.
- `qualification_levels`: αᵢ = α/[i·(i + 1)] at α = 0.05 gives 0.025, 0.00833, 0.004167 for i = 1, 2, 3, and
  the partial sums 1 − 1/(n + 1) times α never exceed α.
- `carriers_resolve`; `gates_resolve`; `contested_links_resolve`.
- `rows_frozen` against `tests/fixtures/ideas/single_proposal.tsv`.

### End-to-end script and logging

`scripts/e2e/decision_records.sh` (NAB-50) asserts 40 records (88 in total) and zero unresolved carriers and
gates, logged per STD-E2E.

### Known answers and negative controls

Known answers: 40 rows; 23 distinct kill gates; 20 rows without one; Σᵢ 1/[i·(i + 1)] = 1, so the levels sum to
α. Negative controls: IP-49's ratio cited as a general saving (`idea.ratio-as-result`); a carrier removed from
IP-82; IP-60 placed with a speed claim.

## Considerations

- Several of these ideas were added by examiners rather than proposers (a constrained allocator that reports
  refusals under any learned policy; a shield fixture with a verified backup). They are recorded with the
  blind spots (NAB-48) as well.
- IP-79 and IP-80 share one pilot and one mechanism bead; the register keeps two records because they answer
  two different questions.

## References

Appendix B.4; Appendix B.2.
