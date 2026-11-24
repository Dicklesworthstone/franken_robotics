---
key: "NAB-39"
title: "killed-idea register KI-57 to KI-63: reality, structure, the thesis; multipliers, withdrawn claims, process"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "delivery", "tenfold", "ground", "gauntlet", "solid", "registry", "decision-record"]
parent: "EPIC-M0"
deps: ["N00-15", "NAB-32"]
needs: []
related: []
gates: []
plan_refs: ["Appendix C.2.7", "Appendix C.2.8", "§0.1 (rule 3)", "§0.8 (H-12, H-14, H-18, H-19)", "§1.3", "§2", "§36.3"]
---
## Background

FrankenRobotics keeps every idea that was proposed and then refuted or deflated in a register
(`registries/killed_ideas.toml`, rendered as `docs/decisions/killed-ideas.md`; schema and revive lint in
NAB-32), each with the strong form as proposed, the refutation with its numbers, what survives, and the
condition under which it would be revived. A proposal that matches an entry must cite it.

This bead carries seven entries in two groups. **Reality, structure and the thesis** (KI-57 to KI-59): what a
simulator may say about a robot that was never built; whether generating structure shortens a design decision;
and the two opposite simplifications of the program's central claim (that a faster engine alone delivers a
ten-fold gain, or that the engine does not matter at all). **Claims, wording and process** (KI-60 to KI-63):
headline multipliers that were printed as if measured; sixteen differentiators withdrawn because incumbent
documentation contradicts them; targets that turned out to be slower than the incumbent; and eleven ways of
working that were proposed and not adopted (copying the foundations wholesale, patching a sibling locally,
gates that need hardware a simulator lacks, numbers called verified by scripts that are not in the
repository).

These entries are where the program's rule about numbers is enforced on itself: no number is written as a
result. Nothing was measured on FrankenRobotics; a ratio, a rate or a coverage figure that describes this
system is a TARGET, or a HYPOTHESIS with the range the evidence supports; headline multipliers do not appear.

Reading conventions: a number inside a *strong form* is the proposal's own; FACT figures have source-register
records and are re-read or re-run before a gate cites them; "DESIGN arithmetic" can be redone by hand; ranges
in *survives* lines are HYPOTHESES until the named gate records a value; thresholds are TARGETs. Tier of the
bead: Solid (a decision record).

## Scope

In: records `KI-57` to `KI-63` with calculations, keywords, carriers and revival conditions; the row
sub-records of KI-61 (nine rows) and KI-63 (eleven rows). Out: the schema, harness and lint (NAB-32); the
claims-wording registry (N00-15); the thesis arithmetic calculators (N00-17).

## Technical approach

**Transcription rule.** As in NAB-33. KI-61 and KI-63 are tables: each becomes one record with `[[idea.row]]`
sub-tables (KI-61: `withdrawn_claim`, `contradicted_by`, `claimed_instead`; KI-63: `item`, `evidence_against`,
`done_instead`), each row with its own keywords so the revive lint can match a single row. KI-62's record
state is `withdrawn-claim`; KI-63's is `process`.

**Machine-checked arithmetic of this group.**

| Record | Expression | Value |
|---|---|---|
| KI-57 | realized coverage of a split-conformal band at n = 20 and 90% nominal: Beta(19, 2), standard deviation | 6.3 points |
| KI-57 | the same at n = 200: Beta(181, 20) | 2.1 points |
| KI-59 | ceiling of an infinitely fast step at a step share s: 1/(1 − s), at 87% and at 29% | 7.7; 1.4 |
| KI-59 | loop ratio with one design per agent turn and 300 s of training: (300 + t)/t at t = 120 s and 60 s | 3.5; 6 |
| KI-60 | the quotient factor of one ballast fixture: 735,471 / 24,546 | 29.96 (a property of that fixture; the factor is 1 in general) |
| KI-60 | optimism of the orbit-search figure: about 2.4 s against 37 ms for 256 orbits | about 65 times |
| KI-62 | 400 k env-steps/s at decimation 10, in world-steps | 4.0 M world-steps/s, twice the stated 2.0 M |
| KI-63 | a thermal limit time printed as 33.5 s and re-derived as 21.1 s | a difference of 12.4 s, 59% above the re-derived value |

The standard deviation of Beta(a, b) is √(a·b / ((a + b)²·(a + b + 1))): for (19, 2) it is √(38/(441 × 22)) =
0.0626; for (181, 20) it is √(3,620/(201² × 202)) = 0.0211.

**The corrected targets that KI-62 leaves in force** (TARGETs owned by their gates): a quadruped joystick
policy trains in no more than 1.25 times the incumbent's time re-measured on the same card (the incumbent's
published figure is about 410 s on one RTX 4090, so the earlier 15-minute gate was slower than the incumbent);
throughput parity at 0.67 of the incumbent or better, with the parity kill gate below 0.33; world-steps and
env-steps defined with decimation (an env-step is d world-steps); published incumbent loops take about seven
minutes for a flat-ground quadruped and under 30 minutes for humanoids, not "six hours".

**The recomputed headline ratios that KI-60 records** (FACT: recomputation in the planning record): four
campaign ratios printed as 10, 16, 12 and 10 are 9.79, 6.41, 11.6 and 5 on their own fixtures, and the 11.6
falls to 2.0 with a mirror symmetry only; "90 to 99% of futile simulations deleted" is not a measurement; the
lifecycle factor of 1,800 counts substeps, not wall time or field life.

**Keywords for the revive lint.** KI-57: reality band for an unbuilt design, conformal band without
calibration. KI-58: topology optimization as a speed lever, generative structure counts toward ten-fold.
KI-59: faster step delivers ten-fold, engine does not matter. KI-60: any printed multiplier of the form "N×
faster", "N-fold" or "N% of simulations deleted" outside a gate record. KI-61: per row (deterministic GPU
replay as a first; different CPU code base; mixed topologies as a capability; actuator thermal modelling as a
novelty; no training needed; anytime-valid comparison as a novelty; first on Apple GPUs; morphology fixed
before the simulator starts; any ten-fold on per-step throughput). KI-62: 15-minute training gate, six hours
later. KI-63: per row (vendor the foundations; verified by script; real-board timing gate; zero-millisecond
stall; unadmitted third-party tool; local patch of a sibling; absolute statement about incumbents; hardware
figure asserted; ideation score as evidence; simulated humans as validated agents; generated board support).

**The entries.**

**KI-57. Conformal reality bands for designs that were never built** (H-14, NARROWED)
- *Strong form.* With 20 paired real episodes, split-conformal bands reach nominal coverage on held-out real
  episodes for a robot class.
- *Refutation.* Coverage is marginal and holds only inside the exchangeable set that was calibrated. DESIGN
  arithmetic: at n = 20 and 90% nominal, realized coverage is distributed as Beta(19, 2), with a standard
  deviation of 6.3 points; at n = 200 it is Beta(181, 20), with 2.1 points. Nothing follows for a design with no
  calibration data.
- *Survives.* Bands per calibrated cell, one robot and one task family, and a refusal elsewhere (§30.12;
  GATE-GROUND-06, GATE-GROUND-07; KILL-BAND; BET-28).
- *Revive only if.* None for an unbuilt design. Per calibrated cell, GATE-GROUND-06 and GATE-GROUND-07.
- *Carried by.* BD-393.

**KI-58. Generative structure as a decision-time lever** (H-12, DEFERRED)
- *Strong form.* Topology optimization on telemetry load spectra yields links at least 15% lighter and counts
  toward the ten-fold.
- *Refutation.* FACT (published results): links 7 to 30% lighter than engineered baselines, which is 2 to 8% of
  robot mass on the one quadruped with a published mass breakdown, at CPU-hours for 12 to 20 links. It improves the
  built object and shortens no decision.
- *Survives.* Foundry as an output-quality subsystem, outside the thesis of §2; its beam rung also runs inside
  screening as a validity check on declared link walls (§31.2, §31.7, §31.23; KILL-TOPOPT; BET-29).
- *Revive only if.* A phase ledger in which structure shortens a decision. Until then structure is output quality, outside the thesis.
- *Carried by.* BD-401, BD-403.

**KI-59. Engine speed alone, and information alone** (H-18, ACTIVE)
- *Strong form.* Either a faster step delivers the ten-fold, or the engine does not matter and information is the
  whole lever.
- *Refutation.* DESIGN arithmetic on published shares: an infinitely fast step speeds training by at most
  1/(1 − s), which is 7.7 at a step share of 87% and 1.4 at 29%. With one design per agent turn and 300 s of
  training the ratio is at most (300 + t)/t, 3.5 to 6 for t between 120 and 60 s. No instrumented design session
  exists. Where every candidate is trained, training is about 96% of an illustrative campaign, so the engine must
  hold parity; once controller acquisition is amortized, agent turns are about two thirds of elapsed time. Which
  regime a mission is in is a measurement.
- *Survives.* Throughput as a parity requirement (§36.3; KILL-PARITY). The levers of §2.4. A ten-fold test that
  measures decisions, with a phase ledger (§39).
- *Revive only if.* The phase ledger and the three single-change ablations (GATE-TENFOLD-13), with throughput parity held (KILL-PARITY).
- *Carried by.* BD-143, BD-217, N40-22.

**KI-60. Headline multipliers**
- *Strong form.* Campaign ratios of 10, 16, 12 and 10; "90 to 99% of futile simulations deleted"; a quotient
  factor of 29.96; a lifecycle factor of 1,800; an orbit search 3,000 times faster than training.
- *Refutation.* FACT (recomputation in the planning record): the four ratios are 9.79, 6.41, 11.6 and 5 on their
  own fixtures, and the 11.6 falls to 2.0 with a mirror symmetry only. DESIGN arithmetic: 29.96 = 735,471/24,546
  is a property of one ballast fixture, and the factor is 1 in general; the 1,800 counts substeps, not wall time
  or field life; the orbit figure omitted the per-step Jacobians and was about 65 times optimistic, 37 ms against
  about 2.4 s for 256 orbits. None is a measurement.
- *Survives.* The mechanisms, each with its ceiling stated before any speed test (D-10). No multiplier is printed
  as a result.
- *Revive only if.* None. A multiplier is printed only as the measured value of a gate, with its ceiling stated first.
- *Carried by.* BD-354, N00-01, N00-15.

**KI-61. Withdrawn differentiators.** §1.3 lists sixteen statements of version 0.2 that are withdrawn. The reason in
each case is FACT: incumbent documentation at pinned versions (§1.1, §1.2).

| Withdrawn claim | What contradicts it | What is claimed instead |
|---|---|---|
| Deterministic GPU replay as such; mixed topologies, determinism and articulated contact together as an incumbent gap | Two incumbents document opt-in bitwise run-to-run determinism; one experimental solver documents all three | Replay of one world alone on the CPU from its address, bit for bit where class D1 is reached (N6); batch-composition invariance (§35.5) |
| "The CPU version is a different code base"; one kernel source for several backends, and several tiers, as novelties | One kernel compiler emits CPU and CUDA code from one source | A CPU twin per kernel with a recorded divergence ledger (§21.13) |
| Different robot topologies in one GPU batch, as a capability | One environment suite and two solvers of one engine document it | Two measured numbers from the head-to-head (N11; §17.14) |
| Electrical and thermal actuator modelling as a novelty | Incumbents ship both | Cards with provenance, and the thermal-runaway check (§15.7) |
| "No training needed" for baseline control | Published sampling controllers run on incumbent engines, with cost weights chosen per task and robot | A baseline synthesized per morphology class, conditional on BET-06 (N7) |
| Anytime-valid comparison as a novelty of the field | Published sequential tests compare policies | Those tests applied to paired design comparison on common world addresses (N4) |
| Apple-GPU support as a first; interior scenes as a novelty; an agent interface to a simulator as such | One incumbent lowers to Metal; interior generators and simulator-facing agent servers exist | A Metal tier measured against that incumbent (N10); verbs with budgets, provenance and typed refusals (N9) |
| "Morphology is fixed before the simulator starts"; an impossible inertia "loads without complaint" everywhere | State-preserving recompilation exists on the CPU; one incumbent's compiler rejects impossible inertias | The latency from an edit to the first GPU step, measured (§17.8) |
| Any ten-fold that rests on per-step throughput, on sampling control as the evaluator, or on a published ten-fold | None is supported (KI-26 and KI-59) | The thesis of §2 as a conjunction, tested in §39 |

*Revive only if.* A withdrawn claim returns only if the M0 re-audit with execution contradicts the documentation that withdrew it; the claims registry is append-only.

*Carried by.* N00-13, N00-14, N00-15.

**KI-62. Corrected targets and narrative figures**
- *Strong form (version 0.2).* A quadruped training gate of 15 minutes; a humanoid throughput target presented
  as parity; 400 k env-steps/s beside a target of 2.0 M world-steps/s; an incumbent loop that teaches an agent
  its mistake "six hours later".
- *Refutation.* FACT: a quadruped joystick policy trains in about 410 s on one RTX 4090, so the 15-minute gate
  was slower than the incumbent, and incumbents publish about 2.55 M humanoid world-steps/s, about five times the
  old target. DESIGN arithmetic: 400 k env-steps/s at decimation 10 is 4.0 M world-steps/s, twice the stated
  2.0 M. Published loops take about seven minutes for a flat-ground quadruped and under 30 minutes for humanoids.
- *Survives.* GATE-CORTEX-05 at no more than 1.25 times the incumbent re-measured on the same card. Throughput
  parity at 0.67 of the incumbent or better, with KILL-PARITY below 0.33. World-steps and env-steps defined with
  decimation in §36.2.
- *Revive only if.* The incumbent re-measured on the same card; the targets move with that measurement and with nothing else.
- *Carried by.* BD-040, BD-217, BD-218.

**KI-63. Posture and process.** Items that were proposed as ways of working and are not adopted.

| Item | Evidence against | What the plan does instead |
|---|---|---|
| Copying the foundations wholesale | FACT: the closure of the multibody crate is 42 crates and about 1.57 million lines of library source; the M1 set is 65 crates and about 2.06 million | Consume by path behind the airlock; a worker process as the first fallback; a minimal lifted referee set last (§33.2, §33.14; KILL-FSIM) |
| Numbers called verified by scripts that are not in the repository | FACT: one re-derivation gave 21.1 s where 33.5 s was printed for a thermal limit time | Known answers are derived on file in this repository before a gate pins them (§38; the known-answer suite) |
| Gates that need hardware a simulator lacks: a real-board timing receipt, bench validation, review panels, real building scans | A simulator cannot produce the evidence | Import lanes with a recorded `no-claim` (§30.2, §30.17) |
| Latency slogans: a zero-millisecond topology stall, sub-millisecond compiles, a preflight budget summed from invented stage budgets | None was measured | Percentiles at gates: `design check` under 2 s at the 95th percentile (GATE-ATEL-02); the edit-to-first-step latency measured (§17.8) |
| Third-party tools used without admission | They bypass the admission gates of §33.6 | Admission by an INT gate, or not used; the portable GPU layer is recorded as not admitted (§21.19) |
| A local patch or fork of a sibling in place of a change in its own repository | Every sibling is the owner's; a local patch diverges from the sibling's head and does not survive a pin move (§33.8) | Upstream work items with fixtures, done in the sibling's repository under its own rules (the upstream work-item register) |
| Absolute statements about incumbents, and a legality bound with no cited source | Documentation at pinned versions contradicts several (§1.2) | Quoted documentation only; dimensional norms from cited sources (OPEN-HAB-1) |
| Hardware and memory figures asserted as fact: compressed buildings, resident-world counts, bandwidths, matrix rates | Unmeasured | The hardware fingerprint and world-bank gates (GATE-SPARK-07, GATE-HAB-12) |
| A score given to an idea during ideation, used as evidence | A score is not a calculation, a pilot or a measurement | Verdicts change only on evidence of those three kinds (§0.8) |
| Simulated humans as validated agents; markets over designs | Unvalidatable; no baseline exists | Scripted humans as scene dressing (§19.7; KILL-DRESSING) |
| Automated generation of bare-metal board support | Outside the safety boundary of §37.9 | Firmware export [M] compiles only against interfaces the caller supplies (§31.13) |

*Revive only if.* Per row, new evidence of the kind that is named against it. A local patch or fork of a sibling is never revived.

*Carried by.* BD-006, BD-037, BD-038, N00-08.

## Acceptance criteria

- [ ] Records `KI-57` … `KI-63` exist with the paragraphs and tables verbatim, revival conditions, carriers and
      keywords; KI-61 has nine row sub-records and KI-63 has eleven.
- [ ] The harness recomputes the eight calculation rows.
- [ ] The revive lint (with the wording lint) fails a planted README sentence "FrankenRobotics is 16× faster
      than incumbents" as `ki.strong-form-as-goal` naming KI-60, and passes a gate record that prints a
      measured ratio with its interval, ceiling and disclosure block.
- [ ] Each withdrawn claim of KI-61 resolves to a record of the claims-wording registry, and the "claimed
      instead" wording equals the permitted wording there.
- [ ] A planted bead titled "Vendor the multibody crate into this workspace" fails
      `ki.match-without-citation` naming KI-63 (the row on copying the foundations wholesale), and a planted
      bead "Patch the sibling locally until upstream lands" fails naming the row on a local patch or fork.
- [ ] KI-59's record carries hypothesis state ACTIVE and record state `assumed`: it is kept as a record of
      what was assumed, not as a refutation.

## Test plan

### Unit tests

`crates/frob-conformance/tests/killed_ideas.rs`, module `g7_thesis`:

- `ki57_beta_sd`: √(19·2/(21²·22)) = 0.0626; √(181·20/(201²·202)) = 0.0211.
- `ki59_ceilings`: 1/(1 − 0.87) = 7.692; 1/(1 − 0.29) = 1.408; 420/120 = 3.5; 360/60 = 6.
- `ki60_ratios`: 735,471/24,546 = 29.963; 2,400/37 = 64.9.
- `ki62_units`: 400,000 × 10 = 4,000,000.
- `ki63_rederivation`: 33.5 − 21.1 = 12.4; 12.4/21.1 = 0.588.
- `ki61_has_nine_rows`; `ki63_has_eleven_rows`.
- `ki61_wordings_match_claims_registry` (the registry is N00-15's, a dependency of this bead, so the test runs
  from the start and never records a `skip`).
- `records_complete_and_verbatim` against `tests/fixtures/killed/g7.md`.

### End-to-end script and logging

`scripts/e2e/decision_records.sh` (NAB-50) recomputes the calculations and runs the planted wording and
manifest cases. Logged per STD-E2E.

### Known answers and negative controls

Known answers: the table; nine and eleven rows. Negative controls: a multiplier in a README; a target written
"15 minutes" for quadruped training; a contract that says a reality band applies to "any design of the
class"; a gate whose threshold needs a bench measurement and is not marked as an import lane; a known answer
whose derivation script is not in the repository (the row on numbers called verified). Metamorphic check for
`ki59_ceilings`: the ceiling is increasing in the share and equals 1 at a share of 0.

## Considerations

- KI-59's question (which regime a mission is in: training-dominated, or agent-turn-dominated once controller
  acquisition is amortized) is a measurement that the ten-fold test's phase ledger makes at M3 and M7. The
  entry cannot be resolved earlier and says so.
- KI-61 exists because incumbents move: a withdrawn claim could become true again if an incumbent removed a
  feature, and a kept claim could fall. The M0 re-audit with execution is the first re-read; the claims
  registry is append-only.
- KI-63's rows are the program's memory of its own process mistakes. They are enforced by other beads' checks
  (the dependency firewall, the admission gates, the hygiene rules); this record is where the reason is kept.

## References

Appendix C.2.7 and C.2.8; §0.1 rule 3; §0.8 rows H-12, H-14, H-18, H-19; §1.3; §2; §36.3; §39.
