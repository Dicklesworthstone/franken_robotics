---
key: "NAB-43"
title: "registries/ideas.toml: the first pass: thirty ideas, the best five and next ten with reasons, carriers for all"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "delivery", "gauntlet", "solid", "registry", "decision-record"]
parent: "EPIC-M0"
deps: ["NAB-42"]
needs: ["§45: the seed bead set with its current numbering, from which carriers are named (the planning text's own bead numbers for these ideas belong to an earlier numbering and are not used)"]
related: ["NAB-44", "NAB-45", "NAB-46", "NAB-41", "NAB-50", "BD-018", "BD-092", "BD-110", "BD-114", "BD-204", "BD-263"]
gates: []
plan_refs: ["Appendix B.1", "§0.8", "§1.1"]
---
## Background

The first idea pass for FrankenRobotics generated thirty ideas against the brief (a robotics simulation and
design system in which an agent designs robots), thought each through (how it works, how an agent would
experience it, how it is built, how it fails), and winnowed them. The ranking criterion was *obviously
accretive and pragmatic for an agent designing a robot*, not mathematical interest. Every survivor is designed
into the system; this bead records where, in the idea-pass register (`registries/ideas.toml`, schema
`frob.ideapass/1`, NAB-42), with records `FP-01` to `FP-30`.

The record matters for two reasons. It carries the *reasons* the best five were ranked first, which are the
shortest statement of what the system is for: results must not be artifacts of the simulator; feedback must
arrive in seconds, before any training; every design needs a controller without a training run; a batched GPU
simulator that cannot be debugged cannot be trusted; and energy per metre is the figure of merit of a mobile
robot. And it gives every idea a carrier, so that "designed into the system" is checkable.

Reading rules (from NAB-42): where a later pass or the hypothesis register narrowed an idea, the narrowed form
is the one stated; every ratio a pass claimed is a HYPOTHESIS unless marked FACT; every threshold quoted from
a gate is a TARGET owned by that gate, and the gate register governs where the two differ. Tier: Solid (a
decision record).

## Scope

In: records `FP-01` to `FP-30` with rank, reason, confidence, what changed since, and carriers. Out: the
schema and lint (NAB-42); the later rounds (NAB-44 to NAB-49).

## Technical approach

**Transcription rule.** One record per idea, numbered as in the list of thirty. Extra fields for this pass:
`rank` (1 to 15 for the ranked ideas; empty for the rest), `why_it_wins`, `confidence`, `since_first_pass`,
`narrowed` (text), all verbatim from the paragraphs below. `proposals = 1` (one pass). `disposition` is
`placed` or `placed-narrowed`. The planning text cited bead numbers for these ideas from an earlier numbering
(for example the trust ladder at two numbers that now belong to the self-collision analysis and to
`explain.run`); those numbers are dropped and carriers are given below by current key.

**The thirty ideas, as listed.** (1) a trust ladder with promotion and exploitation detectors; (2) a check-before-simulate static
analysis suite; (3) a synthesized baseline controller for every design; (4) single-source derived kernels on
three backends; (5) teleporting a GPU world to the CPU by address; (6) an energy ledger read from the port
structure; (7) anytime-valid design comparison on common random numbers; (8) morphology symmetry detection with
equivariant learning; (9) typed edits with lineage diff, blame and bisect; (10) scene splits enforced
mechanically; (11) actuator cards with provenance and a co-design closure check; (12) registered predictions with
per-agent calibration; (13) a quality-diversity archive of designs; (14) deterministic ray-kernel sensors and
stills with pixel inspection; (15) scorecards normalized against a reference robot per class; (16) a structural
margin loop from telemetry loads; (17) a gait atlas from symmetry with max-plus timing; (18) a maneuverability
index from Lie brackets; (19) a connection-curvature design oracle; (20) import loss reports with
physical-consistency repair; (21) a budget estimate before every expensive verb; (22) one verb registry that
generates the CLI, the MCP tools and the documentation; (23) a failure taxonomy with first-cause ordering;
(24) token-budgeted telemetry summaries; (25) self-contained HTML replays; (26) randomization derived from card
uncertainty; (27) certificates bound to kernel hashes; (28) racing optimizers that cancel losers; (29) a
behaviour-cloning warm start from the baseline controller; (30) a negative-control registry for every gate.

**Rank to idea number, and carriers** (`carriers` of each record):

| Rank | Idea (number in the list of thirty) | Carried by |
|---|---|---|
| 1 | The trust ladder (1) | BD-212, BD-263, BD-267 |
| 2 | Check before you simulate (2) | BD-100 to BD-108, BD-110, BD-244 |
| 3 | A baseline controller for every design (3) | BD-114, BD-116, BD-117, BD-118, BD-247, BD-249 |
| 4 | Teleport, on single-source kernels (4 and 5) | BD-018, BD-019, BD-020, BD-221; BD-203, BD-204 |
| 5 | The energy ledger (6) | BD-076, BD-092, BD-208 |
| 6 | Anytime-valid comparison on common random numbers (7) | BD-124, BD-262, SPEC-C-15 |
| 7 | Symmetry as a computed property (8) | BD-102, BD-241, BD-325, BD-326, SPEC-C-09 |
| 8 | Lineage with diff, blame and bisect (9) | BD-058, BD-113, BD-343 |
| 9 | Scene splits enforced mechanically (10) | BD-235 |
| 10 | Actuator cards and the closure check (11) | BD-069, BD-103 |
| 11 | Registered predictions (12) | BD-344 |
| 12 | A quality-diversity archive (13) | BD-342 |
| 13 | Deterministic ray-kernel sensing and stills (14) | BD-187, BD-191 |
| 14 | Reference-robot scorecards (15) | BD-125, BD-201, BD-254 |
| 15 | A negative-control registry (30) | BD-025, STD-TEST |

**The best five, best first.**

1. **The trust ladder (§16.8, §23.16).** Fast-tier results are provisional until a sample is
   re-run on a higher tier, and policies are re-tested under perturbations that should not matter. *Why it wins:*
   the most damaging failure of a learned-control simulator is a result that is an artifact of the simulator. It
   is also the cheapest to guard against, because the referee already exists in FrankenSim and the sample size is
   set by a sequential test. *Confidence:* high. The mechanism is simple, and FrankenSim already states the same
   pattern for its surrogates. *Since the first pass:* promotion compares task decisions as well as state errors
   (§35.4); finalists are re-ranked under other contact laws, cones, stiffnesses and steps, and a ranking that
   flips is typed `model-conditional` (§16.9, A-19); a paired tier correction reports a referee-level estimate
   (C-25, §29.11); a sampled background referee is [F] (§16.11).
2. **Check before you simulate (§14.11, §28.4).** The analyses A-01 to A-26, of which those
   marked for `design check` run inside eight preflight stages in under two seconds (TARGET, GATE-ATEL-02 and
   GATE-ATEL-05), with repairs as typed edits. *Why it wins:* it moves the first feedback from the minutes of a
   training run (FACT, §1.1: about 410 s for a quadruped policy on one RTX 4090 in an incumbent) to seconds, it
   runs on a CPU with exact arithmetic where possible, it ships at M1 before any GPU work, and it is where the
   Lie theory visibly earns its place. *Confidence:* high for sizing, stability, symmetry and collision; medium
   for how well verdicts predict simulated failure, which BET-05 tests (GATE-ATEL-01, KILL-STATIC).
3. **A baseline controller for every design (§22.3, §22.4).** An estimator and a
   controller synthesized from the model. *Why it wins:* without it every design iteration contains a training
   run, and the loop of NS-06 is impossible. It also supplies demonstrations for warm starts (§23.6, §23.10).
   *Confidence:* medium-high for standard layouts, medium for novel ones; BET-06 and KILL-BASELINE handle the
   miss. *Since the first pass:* where the baseline is not synthesizable, a sampling controller gives a first
   behaviour as a screening rung (§22.8), and a family policy per morphology class is [F] (§23.11).
4. **Teleport, on single-source kernels (§21.6, §21.7, §29.8).** Any GPU world
   can be re-run on the CPU from its address, and the first differing stage is named. *Why it wins:* a batched GPU
   simulator that cannot be debugged cannot be trusted, and counter-based addressing plus a CPU twin make this
   nearly free once designed in. *Confidence:* high for teleport; medium for the speed of derived arithmetic,
   which BET-01 tests and KILL-KIR covers. *Narrowed:* one kernel source lowered to several backends is prior art
   in incumbents (FACT, §1.1), so the differentiator claimed is the recorded cross-backend disagreement per
   kernel; and a teleported world is a bit-exact replay only where class D1 is attained, a tolerance replay
   otherwise (KILL-D1).
5. **The energy ledger (§15.11, §29.9).** Every joule attributed to a loss or a store, with the
   numerical residual on its own line. *Why it wins:* energy per metre is the figure of merit for a mobile robot,
   and knowing where the energy went tells an agent what to change. It falls out of the port structure instead of
   being bolted on. *Confidence:* high. *Since the first pass:* the same ledger drives the audit for energy
   created by the simulator (§16.10).

**The next ten.**

6. **Anytime-valid comparison on common random numbers** (§28.6; C-15; GB-06): an agent can stop as soon
   as the answer is clear, and pairing by world address does more for sample size than the test itself. It is now
   one of two declared study types; the other is a fixed-n exact study with a pre-analysis of which verdicts are
   reachable at that n (§24.12).
7. **Symmetry as a computed property** (C-09; A-03; §13.4): the group is found, not declared; it
   gives augmentation, equivariant policies and a metamorphic test of the whole simulator per group element.
   *Narrowed:* a morphology has two groups. Closed-loop consumers (augmentation, equivariant policies of §23.13,
   symmetric controllers of §22.13) use only the observation-preserving subgroup, intersected with the
   stabilizers of the current payload, terrain and task (D-14). KILL-EQUIV keeps symmetry for augmentation and
   metamorphic tests if GATE-CORTEX-03 fails.
8. **Lineage with diff, blame and bisect** (§28.7): version control for robots, so "which edit broke
   stairs" has an answer.
9. **Scene splits enforced mechanically** (§19.14; GATE-ARENA-02, GATE-HAB-14): a policy cannot be scored on a
   scene it trained on, by construction; a mirrored layout or a retrofit variant planted across a split is
   refused.
10. **Actuator cards and the closure check** (§15.3, A-10, A-11; GB-03): the mass spiral is
    decided before simulation, and no actuator constant is invented.
11. **Registered predictions** (§28.17): a calibration record per agent, at almost no cost.
12. **A quality-diversity archive** (§28.8, §17.12): a map of different good designs instead of one
    optimum.
13. **Deterministic ray-kernel sensing and stills** (§20.6, §20.9): depth, lidar and height
    scans share the physics determinism class, and a vision-capable agent can look and then ask about a pixel.
14. **Reference-robot scorecards** (§24.8): every number has a familiar baseline beside it.
15. **A negative-control registry** (§38; GATE-GAUNT-01): a gate that nothing can fail is not a gate.

**Ideas 16 to 29** are also in the system, at lower priority or as Frontier items.

| # in the list | Idea | Where it lives now | State | Carried by |
|---|---|---|---|---|
| 16 | Structural margin loop from telemetry loads | A-15 (§28.4); Foundry §31.5 to §31.9 | [F]. It grew into the structure loop of §31, with KILL-LOADS and KILL-TOPOPT | BD-400, BD-401, BD-402, BD-403, BD-404; kill decisions at BD-412 |
| 17 | Gait atlas from symmetry with max-plus timing | C-10; A-04; Reflex §22.9 | [F]. The atlas supplies clocks and seeds; an orbit ranks designs only if GATE-GAIT-03 passes (KILL-GAIT) | BD-332, SPEC-C-10; orbit ranking decided at BD-335 (GATE-GAIT-03) and N22c-17 (KILL-GAIT) |
| 18 | Maneuverability index from Lie brackets | C-07; A-07 (§28.4); GB-05 | [S] | BD-106, SPEC-C-07 |
| 19 | Connection-curvature design oracle | C-08; A-09; Augur §25.19 | [F], kinematic locomotors only. Its inverse use is [F] on kinematic fixtures and [M] as an atlas, under KILL-AUGUR-PHASE. Its use as a roller-misalignment test is withdrawn (the killed-idea register) | BD-243 (A-09), BD-425, SPEC-C-08; N25c-46 (KILL-AUGUR-PHASE) |
| 20 | Import loss reports with physical-consistency repair | Skeleton §14.12; §11.5; NS-07 | [S]; GATE-SKEL-01 | BD-065, BD-066, BD-051 |
| 21 | A budget estimate before every expensive verb | Tether §32.9; `sim.estimate` (§16.13) | [S]; GATE-TETHER-11 | BD-219, BD-297, N16b-26 |
| 22 | One verb registry that generates CLI, MCP tools and documentation | Tether §32.6, §32.12; the verb registry | [S]; GATE-TETHER-02 | BD-022, BD-137, BD-138 |
| 23 | Failure taxonomy with first-cause ordering | Chronicle §29.10 | [S]. The ordering is `Estimated`; a causal statement comes only from a fork (§27) | BD-094 |
| 24 | Token-budgeted telemetry summaries | Chronicle §29.12 | [S] | BD-094, N24a-35 |
| 25 | Self-contained HTML replays | Sensorium §20.10; Tether §32.15 (`telemetry.export-html`) | [S]; GATE-TETHER-14 | BD-191, N24a-41, BD-300 |
| 26 | Randomization derived from card uncertainty | Cortex §23.15 | [S] | BD-200, N15b-06 |
| 27 | Certificates bound to kernel hashes | C-14; Lemma §12.4; Spark §21.5 | [F]; KILL-LEMMA | BD-021, BD-027, BD-379; N12-31 (KILL-LEMMA) |
| 28 | Racing optimizers that cancel losers | Atelier §28.8; C-15 | [S]. Expected saving: two to five times fewer evaluations when candidates are separated and none otherwise (HYPOTHESIS, §28.8) | BD-341, N33a-18 |
| 29 | Behaviour-cloning warm start from the baseline controller | Cortex §23.6, §23.10 | [F]; KILL-DISTILL, KILL-RESIDUAL | BD-323; N23a-21 (KILL-DISTILL), N22c-15 (KILL-RESIDUAL) |

Of these, the connection-curvature design oracle (19) remains the most likely to be cut. The structural margin
loop (16), which the first pass also expected to cut, was instead taken up by four later passes and is now the
structure-and-manufacture subsystem.

## Acceptance criteria

- [ ] Records `FP-01` … `FP-30` exist; fifteen have a `rank`; the five best carry `why_it_wins`, `confidence`
      and `since_first_pass` verbatim.
- [ ] Every record has at least one carrier that exists as a bead; no record cites a bead number of the
      earlier numbering.
- [ ] The FACT quoted in the reasons (a quadruped policy trains in about 410 s on one RTX 4090 in an
      incumbent) resolves to a source record, flagged as read by eye with ± 20 s.
- [ ] The TARGET quoted (preflight in under two seconds at the 95th percentile) is recorded as owned by its
      gates and not restated as a result.
- [ ] The traceability lint of NAB-42 passes on these records.

## Test plan

### Unit tests

`crates/frob-conformance/tests/idea_register.rs`, module `first_pass`:

- `thirty_records`; `ranks_are_1_to_15_once_each`; `rank_4_covers_ideas_4_and_5`; `rank_15_is_idea_30`.
- `partition`: the ranked records cover ideas 1 to 15 and 30; the table covers 16 to 29; together 30.
- `carriers_resolve` (against the tracker export).
- `stale_numbers_absent`: no record's text contains the pairs that the earlier numbering used for the best
  five.
- `tier_states_of_16_to_29`: ideas 16, 17, 19, 27 and 29 are Frontier; 18, 20 to 26 and 28 are Solid.
- `text_frozen` against `tests/fixtures/ideas/first_pass.md` (this bead's text).

### End-to-end script and logging

`scripts/e2e/decision_records.sh` (NAB-50) asserts 30 records, 15 ranks and zero unresolved carriers for this
pass, logged per STD-E2E.

### Known answers and negative controls

Known answers: 30 ideas; 6 + 10 + 14 = 30 by section; Frontier among 16 to 29: five (16, 17, 19, 27, 29).
The expected saving of racing optimizers, "two to five times fewer evaluations when candidates are separated
and none otherwise", is a HYPOTHESIS and is stored with that label. Negative controls: a record with an empty
carrier list; a record whose carrier is a stale number; a reason edited to state a ratio as a result.

## Considerations

- The reasons are quoted, not re-argued. If a best-five idea's gate fails, the record keeps the reason and
  gains the outcome; that is more useful to a later reader than a rewritten rationale.
- Idea 19 (the connection-curvature oracle) is flagged as the most likely cut; its use as a roller-misalignment
  test is already withdrawn (KI-21).

## References

Appendix B.1; §0.8 (narrowings); §1.1 (the 410 s figure).
