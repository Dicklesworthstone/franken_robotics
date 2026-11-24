---
key: "EPIC-M6"
title: "Milestone M6, frontier physics: epics behind feature flags, each with its plan, its pilot and its kill gate"
type: "epic"
priority: 3
milestone: "M6"
labels: ["m6", "delivery", "frontier", "moonshot", "gate"]
parent: "EPIC-PROGRAM"
deps: []
needs: []
related: ["EPIC-M8", "EPIC-PROGRAM", "N40-02", "N40-04", "N40-05", "N40-06", "N40-24", "N40-29", "N40-35"]
gates: ["GATE-HAB-07", "GATE-HAB-08", "GATE-HAB-09", "GATE-AUGUR-20", "GATE-AUGUR-21", "GATE-TRAC-09", "GATE-FOUNDRY-12", "INT-FSIM-17", "INT-FTX-01", "KILL-LIVING", "KILL-ADVECT", "KILL-SLOSH", "KILL-PROXY", "KILL-SINEW-ROTOR", "KILL-SINEW-AGE", "KILL-LIFECYCLE", "KILL-AUGUR-PHASE", "KILL-AUGUR-SYMEVENT"]
plan_refs: ["§43.9", "§43.1", "§43.2", "§40.6", "§40.3", "§40.4", "§9 (D-6)"]
---
## Background

**Purpose.** M6 admits the physics that makes more failures visible (liquids that move, payloads that slosh, cables
and rugs, soft soil, harder contact), behind feature flags and off the critical path. Each epic needs its own short
plan, its paper-scale pilot and its kill gate before work starts.

**Why it is a milestone of epics and not of deliverables.** Every item here is Frontier [F] or Moonshot [M]: a
mechanism whose value is a hypothesis. The program's rule for such items is fixed: none is funded before a
paper-scale pilot with a falsifier has run (small enough for a CPU in a day, with an outcome not known in advance),
at most four Moonshot items are active at once, and nothing tagged [M] is on the critical path. So M6 has no
artifact that must exist. Its exit line is *per-epic plans and kill gates*: for every epic, a plan, a pilot result
and a kill-gate decision are on record, whichever way each went. The whole milestone pauses whenever M4 or M7 is
slipping, since nothing here is on their path. Planning estimate: per epic, 2 to 6 weeks each (DESIGN).

**Demonstrates.** No North-Star scenario is tagged M6. Each epic's plan names its own acceptance story, and the
Frontier half of bet BET-36 (living worlds) is tested here.

## Scope

In, in the default order of funding: robot-advected films, payload slosh, and cable and rug proxies [F]; then
Cosserat rods, cloth and coupled islands on the CPU referee for finalists [M]; soft soil at scale; the hard-contact
law on devices; the tree-operator route and the mesh path on devices, where their pilots at M5 and M3 kept them
[F]; tactile sensing [M]; the rotor port [M]; RGB and splat scenes; moving people; gradients through contact with
their validity labels; the holonomy and symmetry-at-contact mission queries; lifecycle compression [M].

Explicitly not in M6: anything the tournament of M7 depends on (a sealed stratum that needs an M6 mechanism which
has not landed is declared `not run` in the tournament report, with the reason); the Moonshot delivery epics of M8.

What M8 needs from M6: nothing by dependency. The cap of four active [M] items spans both milestones.

## Technical approach

**1. The artifact: one record per epic.** Each epic bead carries (a) a short plan: scope, the fixture that will
accept it, the compute it asks for; (b) the pilot: registered before it runs, with a falsifier, a computed ceiling,
a fallback and, after it runs, a result (negative results included); (c) the kill-gate decision as an ADR. An epic
whose pilot's falsifier holds is not built; it is described by its fallback's name and returns to the killed-idea
register.

**2. Gates with a part due at this exit (compiled register tables: 2 admission, 7 convergence).** GATE-FOUNDRY-12
is tagged M8 by its chapter; the register places it here, with lifecycle compression, and it closes with its epic.
GATE-TRAC-09's device rows are due here. GATE-HAB-09 reports the proxy-adequacy pilot's number before the island
work package; KILL-LIVING decides. INT-FTX-01 is run with the cloth oracle as a worker process.

| Gate | Claim | Part due | Pass criterion in brief (TARGET) | Defined in |
|---|---|---|---|---|
| INT-FSIM-17 | Admission | M6 | Foundation: frankensim group `multiphysics`. Poiseuille flow between plates matches the parabola to the crate's stated tolerance; film mass conserved to 10⁻¹² relative over 10⁴ steps; the bridge refuses when the feature is off and refuses a throughput-tier request | §33.9 |
| INT-FTX-01 | Admission | M6 | Foundation: franken_textiles (oracle) [M]. A square cloth draped over a sphere matches `ftx-drape`'s settled shape to its own stated tolerance | §33.15 |
| GATE-AUGUR-20 | Known answer | M6 | Holonomy: differential-drive loop within 10⁻⁹ of 4w·(1 − cos θ); loop-reversal and chart-change residuals below 10⁻⁸ m; the skid-steer and slip mutations refused or withdrawn; one newly proposed gait admitted by a full referee rollout. | §25.27 |
| GATE-AUGUR-21 | Known answer | M6 | Symmetry at contact: the identity holds to finite-difference error on the mirrored landing and fails by a margin of at least 10 times that error on each of the four asymmetric mutations. | §25.27 |
| GATE-FOUNDRY-12 | Fidelity | M6 | [M] A 100-hour reference (1.8 × 10⁸ steps) compressed at least 50-fold in steps with at most 5% error on every slow state; the planted backlash-induced gait collapse caught. | §31.19 |
| GATE-HAB-07 | Known answer | M6 | Advected film [F]: relative volume residual at most 10⁻⁶ over 10⁵ steps with pickup and redeposition; spreading exponent within 2% of 1/8; the non-conservative control fails; no cross-world leak in 10⁶ world-episodes. | §19.19 |
| GATE-HAB-08 | Known answer | M6 | Slosh and proxies [F]: first-mode frequency within 3% at three fill depths; catenary sag within 2% at 20 links; the free-slab rug slides in the castor case. | §19.19 |
| GATE-HAB-09 | Fidelity | Before the island work package | Proxy adequacy: the pilot of §19.10.7 is run and its number recorded; this gate reports, and KILL-LIVING decides. | §19.19 |
| GATE-TRAC-09 | Fidelity | M5; GPU at M6 | CL-H on `batch-cpu`: residual at most 10⁻⁴ within eight passes on at least 95% of battery contact states and 10⁻⁶ under `strict`; zero creep on the incline below threshold; no separation velocity on a sliding block; agreement with the referee on the Solid rows of §18.16. | §18.17 |

Plus the known answers each epic's chapter lists.

**3. Gates of later subsections due at this exit.**

| Gate | Part due at this exit | Statement (from the defining subsection) | Defined in |
|---|---|---|---|
| GATE-KIN-10 | whole | GATE-KIN-10 (Known answer and Design comparison, M6). The scalar and block fixtures are exact, with their labels. On a 100-case suite of short horizons, 80 with no change of active set and 20 controls with a planted change: cosine against central differences at least 0.95 in at least 72 of the 80, and all 20 controls labeled `one-sided` or `fixed-mode`, or refused. The reverse sweep takes at most twice the forward sweep's wall time on the reference quadruped at 2,048 worlds. | §16.21 |
| GATE-HAB-15 | whole | GATE-HAB-15 (Known answer and Design comparison, M6). On the two-room spill fixture, 1,000 forks with reuse equal 1,000 forks without, tile digest for tile digest at every substep; the three known answers hold; the dirty set never exceeds the stencil cone; and where measured a is at least 0.9, fork cost is at most 0.15 of full recomputation at the paired 95% bound. | §19.21 |
| GATE-REFLEX-10 | whole | Design comparison ¦ Linear recovery to 10⁻⁹; the absolute-value control returns 0.25 against 0.5 and that fit is denied admission; empirical coverage of the simultaneous 95% band at least 0.93 on 1,000 held-out trajectories; closed-loop tracking error with the lifted planner within 10% of the compiled-model planner's on 100 episodes paired by world address; planner cost per tick at least ten times lower at that accuracy; every out-of-regime query refused. ¦ M6 | §22.20 |
| GATE-CORTEX-22 | whole | Design comparison ¦ The pilot's comparison on both tasks: matched referee-promoted return in at most 0.35 of the derivative-free arm's GPU-hours; cosine at least 0.5 on at least 60% of monitored windows; every reported design re-scored by the ladder, none promoted on a gradient ¦ M6 | §23.29 |

**4. Recurring gates.** GATE-GROUND-01, GATE-SAFE-01, GATE-TENFOLD-09, GATE-GAUNT-01, -02, -03, -05, -10;
GATE-GAUNT-08 at each funding decision (no Frontier or Moonshot work package is funded without a pilot record that
has a falsifier, a computed ceiling, a fallback and a result).

**5. Kill gates decided here.** In brief: KILL-LIVING → Solid and Frontier proxies only, and no islands.
KILL-ADVECT → static film-friction patches, and advection is not built. KILL-SLOSH → rigid ballast with a spill
metric. KILL-PROXY → Solid proxies in training. KILL-SINEW-ROTOR and KILL-SINEW-AGE → the rotor port is withdrawn;
ageing factors are constants. KILL-LIFECYCLE → aged model variants at hand-specified wear states. KILL-AUGUR-PHASE
and KILL-AUGUR-SYMEVENT → the forward diagnostic and per-event finite differences.

| Kill gate | Trigger | Fallback, named in advance | Decided | Owner (consumer) |
|---|---|---|---|---|
| KILL-SINEW-AGE | The ageing law fails its pilot, or no card with ageing coefficients better than `assumed` exists when the lifecycle work of §31 is funded | f_Q and f_R are user-set constants per scenario, with no evolution law | M6, before the lifecycle work package | §15.14 |
| KILL-SINEW-ROTOR | The hover known answer or the port balance fails, or the hybrid variant has no consumer when frontier physics is funded | The rotor port and the variant are withdrawn | M6 | §15.14 |
| KILL-LIVING | The proxies reproduce at least 90% of the full-model failures (GATE-HAB-09), or an island costs more than 181 rigid worlds at 5% activity with no route to less | The Solid tier and the Frontier proxies are the product; coupled islands are not built; reports say deformable matter is represented by proxy | M6, before the island work package | §19.10 |
| KILL-PROXY | The proxy pilot's falsifier fires (Solid proxies already reproduce the failures of the articulated ones), or the cost exceeds its target with no route to less | Solid proxies in training batches; articulated proxies in evaluation batches only, or not at all | M6 | §19.10 |
| KILL-SLOSH | The frequency gate fails after one repair cycle, or sprung masses change no ranking on the carried-container battery | Rigid ballast, with a spill metric computed from payload acceleration | M6 | §19.10 |
| KILL-AUGUR-PHASE | No proposed gait is admitted by a full rollout, or trajectory optimization finds every proposed gait at lower total cost | A-09 remains a forward diagnostic; the inverse query and the atlas are parked | M6 | §25.27 |
| KILL-AUGUR-SYMEVENT | The identity cannot separate the symmetric fixture from its mutations, or amortization shows no reduction in decision time on either GPU | Per-event finite differences; symmetry is used only on smooth kernels | M6 | §25.27 |
| KILL-LIFECYCLE | GATE-FOUNDRY-12 fails | Aged model variants at hand-specified wear states, with no time axis and no compression claim | M6 | §31.19 |
| KILL-ADVECT | The advected film fails its mass gate after one repair cycle, or costs more than 15% of the step, or design rankings on the spill battery agree with static patches at a rank correlation of 0.95 or above | Static film-friction patches under the law of §18; advection is not built | M6, with GATE-HAB-07 | §19.10.7 |

| Kill gate | Trigger and fallback (from the defining subsection) | Decided | Gate beside it | Defined in |
|---|---|---|---|---|
| KILL-STEPADJOINT | Trigger: the pilot's falsifier fires; or GATE-KIN-10 fails after one repair cycle; or the median cosine on the design corpus is below 0.5 at K = 8. Fallback: the paired stencil of §17.13 where its own gate holds, otherwise rank-based search; the response batches of §18.5 keep supplying adjoint columns at a single state. | M6 | GATE-KIN-10 | §16.21 |
| KILL-FARFIELD | Trigger: one digest mismatch that survives a repair cycle; or the pilot's falsifier; or no admitted living-world fixture reaches a ≥ 0.8. Fallback: the full-replay forks of §27.3, which are exact and need no dependency tracking. | M6 | GATE-HAB-15 | §19.21 |
| KILL-SOILLIFT | Trigger: the pilot's falsifier holds; or more than 1% of calibration trajectories contain a contact-mode change inside the declared regime; or the accuracy or cost clause of the gate fails after one repair cycle. Fallback: `mpc-sample` on the compiled model, as §22.8 specifies; no lifted model is offered for any other regime. | M6 | GATE-REFLEX-10 | §22.20 |
| KILL-SHORTGRAD | Trigger: The pilot's falsifier fires, or the gate fails after one repair cycle. Fallback: PPO with the paired stencil of §17.13 where that passes its own gate; otherwise derivative-free search over φ. | M6 | GATE-CORTEX-22 | §23.29 |

**6. Exit rule.** Recorded by a fresh-context session (seed BD-427, procedure N40-05). The exit line: *per-epic
plans and kill gates.* Every epic has its plan, its pilot result and its kill-gate decision on record; every gate
reached has a recorded value and a caught negative control. A gate of an epic that was not funded is `not-run`
with the kill decision `not-reached`, which does not block (the capability does not exist and is not described).

**7. Dependency firewall.** Needs FrankenSim's multiphysics bridge. Uses if admitted: franken_textiles as a cloth
oracle [M]. If a foundation is missing: the epic is not funded.

**8. Owner input wanted.** OPEN-PROG-5 and OPEN-AUGUR-10: which epics are funded, in which order, under the cap of
four active [M] items. Defaults: none is active before its pilot passes, then the order listed in Scope; no
Moonshot mission slice is funded before its pilot passes.

**9. Stop or re-plan.** An epic stops when its pilot's falsifier holds or its kill gate fires. The whole milestone
pauses whenever M4 or M7 is slipping.

**10. Work packages.**

| Package | Delivers | Crates | Milestones | Key gates |
|---|---|---|---|---|
| WP-33 | Frontier epics not named below | — | M6 | Per-epic plans |
| WP-73 | Living worlds II: advected films, slosh, articulated proxies; coupled islands [F], [M] | `frob-scene`, `frob-contact`, `frob-fsim` | M6 | GATE-HAB-07, -08, -09; INT-FSIM-17 |
| WP-74 | Frontier physics and sensing: soil at scale, the tree-operator route, meshes on devices, tactile sensing, the rotor port, RGB and splat scenes, moving people, gradients through contact [F], [M] | `frob-contact`, `frob-dyn`, `frob-sense`, `frob-actuate`, `frob-scene`, `frob-mission`, `viewer/` | M6 | GATE-AUGUR-20, -21; per-epic known answers |
| WP-75 | Lifecycle compression [M] | `frob-lifecycle` | M6 | GATE-FOUNDRY-12 |

## Acceptance criteria

- [ ] Each of the sixteen M6 epics (advected films; slosh; articulated proxies; rods, cloth and islands; soft soil
      at scale; the hard-contact law on GPU tiers; the tree-operator route on devices; the mesh path on devices;
      tactile sensing; the rotor port; RGB and splat scenes; moving people; gradients through contact; geometric
      phase; symmetry at contact; lifecycle compression) has a plan, a pilot record with falsifier, ceiling, fallback
      and result, and a kill-gate decision ADR. The tree-operator route and the mesh path on devices are epics only
      where their earlier decisions kept them (KILL-TREEOP at M5, KILL-MESHGPU at M3); where those fired, the
      earlier ADR is their record here and nothing further is due.
- [ ] Every gate reached has a recorded value and a caught negative control; every gate not reached is `not-run`
      with a `not-reached` decision.
- [ ] The nine kill gates of the compiled table and the four late ones each have an outcome.
- [ ] At no commit of the milestone were more than four [M] items active.
- [ ] No M6 feature flag is on by default; with every M6 flag off, the standard battery's digests equal the
      reference recorded at the latest earlier exit (M4, or M5 or M7 where one closed later, since M6 runs beside
      them), as re-recorded by the compatibility train's parity run if a pin or T has moved since, so no M6 change
      leaks into the default path.
- [ ] The exit report for M6 is `exit-ready`; the exit record is written by a fresh-context session.

## Test plan

### Unit tests

Carried by the epics. Epic check: due counts for M6 are 2 admission, 7 convergence, 9 kill decisions, plus 4 late
parts and 4 late kill decisions.

### End-to-end script and logging

`scripts/exit/review.sh M6`: for every epic, the pilot is re-run from its registered record and its result
compared with the recorded one; for every funded epic, its acceptance fixture is run through `frob --json` with
its feature flag on and again with the flag off (the flag-off digests must equal the reference of the latest
earlier exit, re-recorded by the compatibility train if a pin or T has moved; the log names that exit, the
reference's commit and its pins). Logs carry
the pilot's falsifier and measured value, the evidence block with the tier label Frontier or Moonshot, digests,
and the gates touched.

### Known answers and negative controls

Advected film: volume residual at most 10⁻⁶ over 10⁵ steps and a spreading exponent within 2% of 1/8; slosh
first-mode frequency within 3% at three fill depths; catenary sag within 2% at 20 links; differential-drive
holonomy within 10⁻⁹ of 4w·(1 − cos θ); lifecycle compression at least 50-fold with at most 5% error. Controls: the
non-conservative film update; a pilot that cannot fail (refused as not a pilot); a fifth active Moonshot item.

## Considerations

- The risk of M6 is that it starves the spine: lines added to Frontier-only crates while a critical-path exit
  slips. The pause rule and the cap of four are the mitigations, and the exit review counts both.
- An island of coupled multiphysics may cost more than 181 rigid worlds at 5% activity; KILL-LIVING reads that
  number before the island work package is funded.
- Epics here may be funded in any milestone after their pilots pass; "M6" is where their decisions are recorded.

## References

Plan §43.9, §43.1, §43.2, §40.3 to §40.6, doctrine D-6. Bet BET-36. Risk RISK-33. Work packages WP-33, WP-73 to
WP-75.
