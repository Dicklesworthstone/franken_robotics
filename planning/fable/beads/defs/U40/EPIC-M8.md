---
key: "EPIC-M8"
title: "Milestone M8, frontier delivery: Moonshot epics behind flags, each with a pilot and a stop rule"
type: "epic"
priority: 4
milestone: "M8"
labels: ["m8", "delivery", "moonshot", "gate"]
parent: "EPIC-PROGRAM"
deps: []
needs: []
related: ["EPIC-PROGRAM", "N40-02", "N40-04", "N40-05", "N40-06", "N40-24", "N40-26", "N40-27", "N40-29", "N40-35"]
gates: ["GATE-AUGUR-22", "GATE-AUGUR-23", "GATE-AUGUR-24", "GATE-REFLEX-09", "GATE-CORTEX-17", "GATE-SHIELD-05", "GATE-FOUNDRY-11", "KILL-MEGA", "KILL-SKILL", "KILL-TUBE", "KILL-GATREE", "KILL-AUGUR-FACILITY", "KILL-AUGUR-TRANSPORT", "KILL-SYNTH-CONT", "KILL-FIRMWARE", "KILL-INTERACT", "KILL-COUPLING"]
plan_refs: ["§43.11", "§43.1", "§43.2", "§40.6", "§40.3", "§40.4", "§9 (D-6)"]
---
## Background

**Purpose.** M8 delivers the Moonshot items that would make the product unlike anything else, each behind a flag
and each with a stop rule. The plan works without every one of them.

**Why it is last and optional.** A Moonshot [M] is a mechanism with a large possible payoff and a low prior. The
program admits such items only under a cap (at most four active at once; an item is active from the funding of its
pilot until its expiry gate; the others wait in the killed-idea register), only after a paper-scale pilot with a
falsifier, and never on the critical path. M8 collects the delivery of those that survived. It comes after the
tournament because nothing here may be needed for the measured claim, and because the tournament's phase ledgers
say which of these could matter. The exit line is *per-epic plans and stop rules*. Planning estimate: per epic
(DESIGN). Critical path: no. Priority: backlog, as for every Moonshot.

**Demonstrates.** No North-Star scenario is tagged M8.

## Scope

In (the epics): facility co-design and multi-robot handoff (the fifth mission slice); skill transport with learned
residuals; continuation-based synthesis on the CPU; firmware export; geometric-algebra tree policies at scale;
reachable-set tables for shields; the fused per-world step kernel.

Explicitly not in M8: anything a Solid or Frontier capability depends on. A build with every M8 flag off is the
released product.

## Technical approach

**1. The artifact: one record per epic**, as at M6: a plan, a pilot with a falsifier, a computed ceiling, a
fallback and a result, and the application of its stop rule. An epic that has not passed its pilot within its
planned weeks returns to the killed-idea register and frees its slot under the cap.

**2. Gates with a part due at this exit (compiled register tables: no admission, 7 convergence).**

| Gate | Claim | Part due | Pass criterion in brief (TARGET) | Defined in |
|---|---|---|---|---|
| GATE-AUGUR-22 | Known answer | M8 | Continuation: 40 certified Stewart–Gough roots on 10 generic parameter sets; `Complete` issued only when the trace test agrees; lost paths at most 0.1% and every one flagged. | §25.27 |
| GATE-AUGUR-23 | Design comparison | M8 | Skill transport: the three-treatment experiment on the tray interface, pre-registered; the planted ramp-crest loss of authority is reported with its mode and a replayable counterexample. | §25.27 |
| GATE-AUGUR-24 | Design comparison | M8 | Facility fixture: at least two feasible engineering strategies discriminated; each of the four seeded failures recovered with a withdrawal and a minimized scenario; the optimality gap reported for every unresolved continuous subproblem. | §25.27 |
| GATE-CORTEX-17 | Design comparison | M8 | Geometric-algebra tree policy [M]: arm B or C is at least 1.3 times faster than arm A to the return threshold at equal wall-clock, or better with no adaptation beyond seed spread. | §23.23 |
| GATE-FOUNDRY-11 | Known answer | M8 | [M] The fixed-point cases of §31.18; 10⁶ vectors bit-equal. | §31.19 |
| GATE-REFLEX-09 | Design comparison | M8 | Learned residual transport [M]: at least 18 of 20 feasible unseen bodies within 3 percentage points of tuned baselines with no adaptation, both cost denominators reported. | §22.18 |
| GATE-SHIELD-05 | Known answer | M8 | Reachable-set table [M]: zero false admissions in 5,000 in-envelope episodes on the braking fixture; `uncovered` outside the table. | §22.18 |

**3. Gates of later subsections due at this exit.**

| Gate | Part due at this exit | Statement (from the defining subsection) | Defined in |
|---|---|---|---|
| GATE-FID-09 | row 4, with the continuation lane | Known answer. Every row is reproduced at its tolerance (times to 0.1 s; sinkage to 10⁻⁵ m in double precision; counts exact; admissions exact) and every negative control fails. Rows 1, 3 and 5 at M1; row 2 at M4; row 4 with the continuation lane. | §38.17 |
| GATE-AUGUR-25 | the tray fixture | Known answer. Zero violated bounds over the 200 finite fixtures; the two-outcome reversal is refused; on the tray fixture the 95% upper bound of the measured budget is below the claim's slack for at least one skill, or the kill gate fires. Finite part M4; tray M8. | §25.29 |
| GATE-KIN-09 | whole | Known answer ¦ On the two-carrier handoff fixture of §25.17 and a 16-robot sparse warehouse: zero contacts, message arrivals or sensor returns from another island inside a certified tile over 10⁶ robot-steps; every robot's digests equal those of the reference that steps the same islands in one world with island-wise solves on `batch-cpu`; the four controls behave as stated; the certified share is reported. ¦ M8, with §25.17 | §16.20 |

**4. Recurring gates.** GATE-GROUND-01, GATE-SAFE-01, GATE-TENFOLD-09, GATE-GAUNT-01, -02, -03, -05, -10;
GATE-GAUNT-08 at each funding decision.

**5. Kill gates decided here.** In brief: KILL-AUGUR-FACILITY → single-robot missions with priced world edits.
KILL-AUGUR-TRANSPORT and KILL-SKILL → family policies and inheritance; the exact skill layer and its witnesses.
KILL-SYNTH-CONT → the exact lane with multi-start recovery, labelled `Partial`. KILL-GATREE → the plain morphology
transformer. KILL-TUBE → reduced-order barriers. KILL-FIRMWARE → controller parameters and test vectors, and no
code. KILL-MEGA → the staged step plan, which is the design of record.

| Kill gate | Trigger | Fallback, named in advance | Decided | Owner (consumer) |
|---|---|---|---|---|
| KILL-MEGA | In the one-week fusion pilot (8,192 worlds, digests unchanged) the paired lower bound on the whole step is below 2.0 | The staged step plan, which is the design of record | M8, before the work package | §21.18 |
| KILL-SKILL | GATE-REFLEX-09 fails | Keep the exact layer and its witnesses; drop the learned residual | M8 | §22.18 |
| KILL-TUBE | GATE-SHIELD-05 fails, or the pilot's falsifier holds | No tables are built; reduced-order barriers cover those subsystems | M8 | §22.18 |
| KILL-GATREE | GATE-CORTEX-17 fails | The plain morphology transformer remains; `fs-ga` features are not used in policies | M8 | §23.23 |
| KILL-AUGUR-FACILITY | The acceptance fixture fails to discriminate two strategies, or misses a seeded failure after one repair cycle | Single-robot missions with priced world edits from §19; multi-robot composition is parked | M8 | §25.27 |
| KILL-AUGUR-TRANSPORT | The interface arm needs as many target trials as the conditioned policy, or reports coverage where the referee shows a fall | Family policies and inheritance of §23; the wrench-coverage test is kept as a static analysis | M8 | §25.27 |
| KILL-SYNTH-CONT | More than 0.1% lost paths, an unflagged lost path, or a missed known count | The exact lane plus multi-start CPU recovery; every result labelled `Partial` | M8 | §25.27 |
| KILL-FIRMWARE | GATE-FOUNDRY-11 fails | Controller parameters, Q formats and the test-vector manifest only; no code | M8 | §31.19 |

| Kill gate | Trigger and fallback (from the defining subsection) | Decided | Gate beside it | Defined in |
|---|---|---|---|---|
| KILL-COUPLING | Trigger: one violated transported bound that survives a repair cycle, or no skill on the tray fixture with a budget below its slack. Fallback: `mission.transport` carries universal claims only, and every chance or expected-value claim is re-measured on the target in the qualification lane of §24.13. | M4 for the finite part (a violated transported bound); M8 for the tray clause | GATE-AUGUR-25 | §25.29 |
| KILL-INTERACT | Trigger: one missed interaction survives a repair cycle; or the certified share is below 0.5 of robot-time on the warehouse fixture. Fallback: a multi-robot world is one coupled world under `strict` with island-wise solves and fixed decimation, and a fleet scene beyond its capacity is refused under §18.11. | M8 | GATE-KIN-09 | §16.20 |

**6. Exit rule.** Recorded by a fresh-context session (seed BD-442, procedure N40-05). The exit line: *per-epic
plans and stop rules.* Every epic has its plan, its pilot result and its stop rule applied; every gate reached has
a recorded value and a caught negative control.

**7. Dependency firewall.** Needs nothing new. Uses if admitted: franken_firmware as a pattern [M]. If missing: the
epic is not funded.

**8. Owner input wanted.** Funding per epic, and a compute envelope of the pretraining class for any pretraining
(tree policies in particular; if not approved, that epic is not funded). The order of activation under the cap of
four (default: none before its pilot passes, then the order of the Scope list).

**9. Stop or re-plan.** Per epic, by its stop rule.

**10. Work packages.**

| Package | Delivers | Crates | Milestones | Key gates |
|---|---|---|---|---|
| WP-78 | Facility co-design and multi-robot handoff [M] | `frob-mission`, `frob-scene` | M8 | GATE-AUGUR-24 |
| WP-79 | Learned transport and tree policies: skill transport with learned residuals, geometric-algebra tree policies, reachable-set tables [M] | `frob-control`, `frob-learn`, `frob-shield`, `frob-mission` | M8 | GATE-REFLEX-09; GATE-AUGUR-23; GATE-CORTEX-17; GATE-SHIELD-05 |
| WP-80 | Continuation-based synthesis, firmware export and the fused step kernel [M] | `frob-synth`, `frob-foundry`, `frob-kernels` | M8 | GATE-AUGUR-22; GATE-FOUNDRY-11 |

## Acceptance criteria

- [ ] Each of the seven M8 epics has a plan, a pilot record with falsifier, ceiling, fallback and result, and its
      stop rule applied, recorded as an ADR.
- [ ] Every gate reached has a recorded value and a caught negative control; a gate of an unfunded epic is
      `not-run` with a `not-reached` decision.
- [ ] The eight kill gates of the compiled table and the two late ones (KILL-INTERACT; the tray clause of
      KILL-COUPLING) each have an outcome.
- [ ] At no commit were more than four [M] items active; an epic past its planned weeks without a passed pilot
      has been returned to the register.
- [ ] With every M8 flag off, the digests of the released product are unchanged.
- [ ] The exit report for M8 is `exit-ready`; the exit record is written by a fresh-context session.

## Test plan

### Unit tests

Carried by the epics. Epic check: due counts for M8 are 0 admission, 7 convergence, 8 kill decisions, plus 3 late
parts and 2 late kill decisions.

### End-to-end script and logging

`scripts/exit/review.sh M8`: each pilot re-run from its registered record; each funded epic's acceptance fixture
through `frob --json` with its flag on, and the flag-off digest comparison against the release. Logs carry the
falsifier, the measured value, the evidence block with the tier label Moonshot, digests and the gates touched.

### Known answers and negative controls

40 certified Stewart–Gough roots on 10 generic parameter sets with lost paths at most 0.1%; 10⁶ fixed-point vectors
bit-equal for firmware export; zero false admissions in 5,000 in-envelope episodes for the reachable-set table; the
fused kernel's paired lower bound at least 2.0 on the whole step at 8,192 worlds with digests unchanged; at least
18 of 20 feasible unseen bodies within 3 percentage points for skill transport. Controls: an unflagged lost path;
a root called real by a tolerance instead of by ball disjointness; a pilot whose outcome is known in advance.

## Considerations

- M8 competes with M6 for the same four slots; OPEN-PROG-5 fixes the order.
- KILL-MEGA's fallback is the design of record: the staged step plan is not a consolation, it is what ships.
- Nothing here may be cited in the tournament report unless it was frozen into the release under test.

## References

Plan §43.11, §43.1, §43.2, §40.3 to §40.6, doctrine D-6. Bet BET-30 (the tree-policy part). Work packages WP-78
to WP-80.
