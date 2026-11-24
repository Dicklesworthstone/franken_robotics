---
key: "NAB-26"
title: "M3 exit: sweep of the decision records after the partial ten-fold gate (nineteen families due)"
type: "task"
priority: 2
milestone: "M3"
labels: ["m3", "delivery", "gauntlet", "tenfold", "solid", "exit-review", "decision-record"]
parent: "EPIC-M3"
deps: ["BD-289", "BD-310", "N00-20", "N12-30", "N14b-16", "N14b-29", "N16b-10", "N16b-19", "N17a-29", "N18a-34", "N19b-36", "N20a-43", "N20b-32", "N22c-14", "N22c-34", "N23b-13", "N23b-14", "N24a-15", "N24c-26", "N24c-34", "N25a-18", "N25a-19", "N25b-29", "N25c-16", "N26a-12", "N26a-32", "N26a-35", "N27b-11", "N28a-22", "N28c-39", "N32a-23", "N40-04", "N40-15", "NAB-22", "NAB-25", "NAB-50", "STD-E2E"]
needs: []
related: ["BD-319", "NAB-32", "NAB-37", "NAB-38", "NAB-42", "NAB-47", "NAB-49"]
gates: []
plan_refs: ["Appendix C.1", "Appendix C.2", "Appendix B.5", "§43 (M3)", "§39.12", "§0.7", "§0.8"]
---
## Background

FrankenRobotics keeps its decision records as data (the expected-value matrix of 143 scored families, the
Moonshot queue, the killed-idea register of 67 entries, the idea-pass register, the source register) and
re-reads them at every milestone exit against the evidence that exit produced, under the rule that a verdict
changes only on new evidence of the kind that produced it. NAB-22 defines the procedure; this bead is the sweep
at the exit of **M3**: places, tasks, adversaries and qualification, and the **partial ten-fold gate**, the
first measured comparison with the strongest rival a competent engineer can compose from existing systems, on
legged and wheeled briefs, recorded whatever its value, with a written rule for each outcome.

M3 and M4 carry more than half of all kill decisions of the program. At M3 nineteen families of the matrix
have a kill gate decided, the first evidence about *Impact* arrives from our own arm of a design mission, and
several of the most consequential killed-idea conditions are evaluated: whether a family certificate decides
anything a generic comparator does not, whether static verdicts predict simulated failure, whether a baseline
controller can be synthesized per morphology class, whether a requirement term helps training.

## Scope

In: the M3 sweep record and report; history entries; proposals; the reader check of the registries. Out:
deciding gates; the exit review (BD-319); the procedure (NAB-22).

Order of work (DESIGN, as at M2 in NAB-25). The sweep is made at the exit candidate commit, before the exit record
is written, and the exit review depends on it: the review's runner executes `decision-sweep check --exit M3` as
the attached check of NAB-22 and copies the sweep's section into the exit record, so the record must exist first.
What the sweep reads is produced before the review, not by it: the gate report at that commit (N40-04); the
kill-decision ADRs written by their decision beads (step 1's table names the gate; the beads are N28a-22, N24a-15,
N26a-32, N27b-11, N18a-34, N25c-16, N23b-14 with N23b-13, N16b-19, N22c-14, N25a-19, N17a-29, N16b-10, N26a-35,
N19b-36, N20a-43, N25b-29, N14b-16, N12-30, N25a-18, N32a-23 and BD-289); the record of the partial ten-fold gate
with its phase ledgers (N40-15); the rank-agreement pilot record (BD-310); the decision of the delivered-controller
lane (N22c-34) and of the Nemesis search (N26a-12) that KI-26 and KI-46 read; and the late mechanisms of step 2's
last paragraph (N24c-26, N24c-34, N28c-39, N20b-32, N14b-29). The dependency list names those beads for that
reason. This is the order of NAB-22's design decision 1, which every sweep follows; what the sweep shares with
the review is its independence rule (a session that implemented none of the cited beads).

## Technical approach

`decision-sweep skeleton --exit M3`, fill, `check --exit M3`, by a session that implemented none of the cited
beads.

**1. Families due by a kill gate decided at or standing from M3.**

| Family | Now | Kill gate | Decided | Trigger (thresholds are TARGETs) | Fallback named in advance |
|---|---|---|---|---|---|
| EV-S14 Static analyses in `design check` | 10.0 | KILL-STATIC | M3 | GATE-ATEL-01 fails (BET-05) | `design check` verdicts are downgraded to advisory lints and the pruning claim is withdrawn |
| EV-S21 Requirement language: bounded-time temporal logic over unit-normalized predicates | 8.0 | KILL-SPEC-REWARD | M3 | In the training pilot the arm with the added requirement term does not improve success over the shaped reward across five seeds | Requirements stay acceptance metrics and search scores only; the added-term option leaves the task schema; margin-only training is offered only if it reaches 80% of the shaped arm's success |
| EV-S22 Rare-failure bounds from bounded-weight importance sampling and crude sampling | 8.0 | KILL-RARE | M3 | In the exploration pilot the gain is below 10 times at 10⁻⁵, or the coverage of any interval offered as a bound is below 85%, or a planted mode is missed in more than 20% of repetitions | Crude nominal sampling with the binomial bound alone; splitting is removed; tilting is kept only inside `nemesis.search` |
| EV-S23 Interventional autopsy on forks, with a placebo fork and a flip rate | 8.0 | KILL-AUTOPSY | M3 | GATE-AUTOPSY-03 fails after one repair cycle: planted-cause accuracy below 90%, or the top factor changes in more than 30% of cases at 10⁻⁷ | Single-factor paired re-runs from the episode start, with first-anomaly ordering labelled `Estimated`; coalitions and Shapley summaries are removed |
| EV-S36 Film-friction law and rigid proxies | 6.0 | KILL-FILM | M3 | On the wet-floor battery the film law changes fewer than 10% of episode outcomes against a constant-coefficient patch, or adds more than 5% to step time | Static condition cards on the lockstep tiers; the law is kept on the transactional profiles for audits |
| EV-S41 Value of an experiment in closed form, used as a stop rule | 4.5 | KILL-AUGUR-PROBE | M3 | The generator never beats the programmed expert plus direct measurement at matched correctness on the coupled-layout experiment, or issues one false lower bound | `experiment.suggest` ships the expert grid and the direct measurement; the experiment grammar is parked |
| EV-F01 Gymnasium-compatible adapter | 12.0 | KILL-GYMADAPT | M3 | GATE-CORTEX-16 fails | Rival stacks drive the environment through the job interface of §32 in whole rollouts, and the ten-fold test records the added latency in its phase ledger |
| EV-F06 Sampled background referee with a reported coverage figure | 6.0 | KILL-SHADOW | M3 | GATE-FID-07 fails on cost or on coverage after one repair cycle, or no planted defect is detected | The referee runs at promotion time only, plus an offline audit of sampled snapshots after each run |
| EV-F07 Baseline controller synthesis per morphology class | 5.0 | KILL-BASELINE | M3 | GATE-REFLEX-02 fails (BET-06) | Baseline synthesis is limited to the morphology classes that pass; others report `not-synthesizable` and route to training |
| EV-F10 Assumption lattice and verdict polarity | 4.5 | KILL-AUGUR-REUSE | Standing, from M3 | One incorrectly inherited verdict from an implication rule | That rule is removed; reuse falls back to the dependency-keyed memo graph of §28 |
| EV-F11 Campaign quotienting by symmetry orbits and mission observables | 4.5 | KILL-QUOTIENT | M3 | One false quotient survives a repair cycle, or digest classes save less than 5% of dispatches on the campaign corpus | Orbit representatives only; the digest is kept as a cache key with no claim |
| EV-F14 Coupled fidelity estimators: control variate, multilevel, doubly robust correction | 4.0 | KILL-DR | M3 | ρ_ep below 0.7 on 200 paired addresses in every stratum tested, or GATE-CHRON-09 coverage fails after one repair cycle | Promotion reports the label only, from referee episodes alone; the ledger keeps recording ρ_ep |
| EV-F15 Minimal counterexample worlds | 4.0 | KILL-MCW | M3 | On unplanted failures fewer than half of shrunk worlds have at most three edits, or fewer than 90% replay | Nemesis ships full failing worlds with replay and single-edit contrasts ranked by margin; the format stays |
| EV-F17 Clearance complexes certified from both sides | 4.0 | KILL-FCLEAR | M3 | One false witness or false cut survives a repair cycle, or more than 30% of doorway queries on the archetype battery stay `unresolved` after refinement | The route-graph traversability check for a nominal footprint, labeled heuristic |
| EV-F18 Observation-alias impossibility witnesses, label counts and schedule classes | 4.0 | KILL-ALIAS | M3 | One false witness from the searched lane | Structural, enumerated and declared witnesses only |
| EV-F18 Observation-alias impossibility witnesses, label counts and schedule classes | 4.0 | KILL-AUGUR-ALIAS | M3 | Generated alias witnesses are contradicted by a permitted probe in more than 5% of audited cases | Witnesses are labelled experiment-specific only; universal observability claims are refused |
| EV-F18 Observation-alias impossibility witnesses, label counts and schedule classes | 4.0 | KILL-SKEL-ALIAS | M3 | Stage 8 returns a witness that a hand-built policy defeats, or decides fewer than half of the planted cases within budget | Stage 8 becomes an advisory report built on A-14 coverage and refuses nothing |
| EV-F19 Family certificates in the `.frobcert` bundle | 4.0 | KILL-FAMILYCERT | M3 | At the partial ten-fold gate no family certificate has decided a cell that the generic comparator of D-17 leaves undecided at equal cost | Mission answers come from the generic solvers alone and still ship in the bundle format; the specialized projections are removed |
| EV-F21 Mission queries with inner and outer design sets | 3.8 | KILL-AUGUR-TIE | M3 | On the strata of the M3 partial gate that use §25, a short robust program over a conventional solver reaches the same checked scope at comparable turnaround | Ship that program behind the same verbs, schema and checker; retire any claim of exclusive expressivity on those families; spend the research budget on the first mission the program cannot resolve |
| EV-F21 Mission queries with inner and outer design sets | 3.8 | KILL-FAMILYCERT | M3 | At the partial ten-fold gate no family certificate has decided a cell that the generic comparator of D-17 leaves undecided at equal cost | Mission answers come from the generic solvers alone and still ship in the bundle format; the specialized projections are removed |
| EV-F24 Slot-level episode dispatch in the daemon | 3.8 | KILL-SLOT | M3 | Any digest difference under co-tenancy, or GATE-TETHER-09's throughput or step-time limits fail after one optimization pass | `frobd --dispatch=per-job`: one environment per job, queued |
| EV-F29 Region certificates and dual witnesses of infeasibility | 3.0 | KILL-AUGUR-REGION | M3 | The pilot of §25.6 meets its falsifier (more than 90% undecided in the best arm, or more than 95% of decided volume from one monotone bound), or GATE-AUGUR-09 fails after one repair cycle | Region certificates are removed; family answers use dual witnesses, monotone bounds and pointwise static analyses; untested regions are reported as sample coverage, `Estimated` |

Family and kill-gate pairs due at this exit: 22.

**2. Killed-idea records whose reversal condition names an M3 gate.**

| Record | Condition read at M3 | If met |
|---|---|---|
| KI-56 a family query as a capability; a faster solver as the lever | At the partial gate, a family certificate decides a cell that the generic comparator leaves undecided at equal cost; and on the strata that use mission queries a short robust program over a conventional solver does not reach the same checked scope at comparable turns | The capability wording may be considered; otherwise the tie rule ships the generic method behind the same verbs |
| KI-14 an always-on referee | Coverage of at least 0.1% of fast-tier world-seconds at a throughput loss of at most 5%, and a planted defect detected | The sampler stays; the strong form stays refuted |
| KI-36, KI-37 rare-event bounds | In the exploration pilot a gain of at least 10 times at 10⁻⁵, with coverage of at least 85% for any interval offered as a bound, and a planted mode missed in at most 20% of repetitions | Exploration by splitting is kept; a bound still comes only from crude or bounded-weight sampling |
| KI-38 a single top cause from forks | Planted-cause accuracy of at least 90%, with the top factor changing in at most 30% of cases at a perturbation of 10⁻⁷ | Attribution keeps its flip rate and placebo fork |
| KI-43 requirements replace rewards | In the three-arm training pilot the arm with the added requirement term improves success over the shaped reward across five seeds | The added-term option stays in the task schema |
| KI-35 joint paving of design boxes | In the region pilot on the 12-parameter box, at most 90% undecided in the best arm and not more than 95% of decided volume from one monotone bound | Per-constraint region certificates are funded for M4 |
| KI-34 verdict reuse by sensitivity margin | No incorrectly inherited verdict from an implication rule (a standing gate from M3) | Implication reuse may be piloted at M4 |
| KI-40 rank fidelity assumed between rungs | The rank-agreement pilot: Kendall τ near the top, with containment and regret reported beside it (a cheap rung selects at τ ≥ 0.7 with the winner check passed, discards only between 0.5 and 0.7, and does neither below 0.5) | The mode per morphology class is recorded; the decisive experiment on our engine is at M4 |
| KI-26 sampling control as a stand-in evaluator | The delivered-controller lane's gate; the measured cost per 20 s episode (planning estimate about 0.33 and 1 GPU-minute for the quadruped and humanoid at a 20 ms planning step, 1.7 and 5.2 at 4 ms) beside the acceptance break-even of the revival condition (2.0 GPU-seconds for a quadruped, 12 for a humanoid: training once, under 10 and 60 GPU-minutes, divided by the 299 clean episodes acceptance needs) | Only the delivered-controller lane admits its own episodes; a screening comparison stays within the break-even count of its class (about 30 and 60 episodes at a 20 ms planning step; 6 and 12 at 4 ms) |
| KI-46 unconstrained adversaries | Every reported failing world has a solvability witness | Stands |
| KI-59, KI-60 engine alone; headline multipliers | The partial gate's phase ledger on both arms; the measured ratio with its interval | No multiplier is printed except the gate's measured value, with its ceiling |

Kill gates of later-added mechanisms decided or piloted at M3 are listed by the tool and recorded here even
though their families have no matrix row yet: the task audit with foil bodies (detection below 60%, or findings
on more than 30% of clean tasks, kills it), the item bank for close comparisons, the controller-budget curve
(its crossing test), visibility under self-occlusion (its failure-classification pilot) and the composed
quasi-static screen. A mechanism among them that passes gets a row by ADR, scored with this exit's evidence.

**3. Contested ideas with a measurement at M3.** Four disagreements of the planning record were never settled
by argument, and each has a measurement. One arrives here in partial form: *where the ten-fold comes from*
(kernel speed is a parity requirement; the gain is sought in avoided trainings, family answers and fewer agent
rounds). The sweep records what the partial gate's ledgers say and nothing more. The others (closed-loop
design gradients, randomized audits against coupled estimation, orbit sharing) are recorded as measured or
carried forward.

**4. Impact, for the first time with evidence.** For every adopted family whose claimed effect is on the time
to an accepted design decision, the partial gate's phase ledger gives a measured share of the phase it acts on.
The sweep records, per family, the share and the ceiling 1/(1 − share) of removing that phase entirely. An
Impact is changed only where the measured ceiling contradicts the score's premise (for example a family scored
Impact 5 whose phase is under 5% of our arm's time).

**5. Moonshot queue.** MQ-02's proxy-adequacy pilot (three scenes of 200 episodes each on an incumbent engine,
proxies against full models) may run from M3: the sweep records whether it was registered. No slot is taken
before the item's own milestone.

**6. Idea register and the lever register.** The lever register is scored at M3; idea records whose levers were
measured take the measured state. Published-anchor figures for reference robots (search-summary level at plan
time) must be re-read before the Parity comparison cites them: `[[unverified]]` item NV-06.

**7. The reader check.** The hygiene rule says a registry exists because a named verb reads it or a named gate
checks it, and one that nothing reads is deleted. At M3 the sweep lists, for each decision registry, the
readers that actually ran in M0 to M3 (the funding check, the pilot ledger, the revive lint, the citation
lint). A registry with no reader is proposed for reduction to its generated printout.

## Acceptance criteria

- [ ] `docs/delivery/exits/M3/decision-records.json` exists and `decision-sweep check --exit M3` passes.
- [ ] Each of the 22 family-and-kill-gate pairs of step 1 has an outcome citing its decision ADR.
- [ ] Each row of step 2 has measured values and `condition_met`; KI-56's entry cites the partial-gate record.
- [ ] Step 4's table of shares and ceilings is the record's `shares` (NAB-22 step 3), and every changed Impact
      cites a row of it.
- [ ] The reader list of step 7 is the record's `readers`; a registry with no reader has a proposal (an entry
      without one fails `sweep.row-not-swept`).
- [ ] No reading task is open for a gate recorded at M3.
- [ ] The sweep record exists at the exit candidate commit before the M3 exit record is written, and the exit
      runner's attached check reads it; the sweep's session implemented none of the beads it cites.
- [ ] A family whose kill gate fired at M3 names its fallback in every registry of the decision records the same
      day (no family reads "adopted" under a fired gate).

## Test plan

### Unit tests

`crates/frob-conformance/tests/decision_sweep.rs`: `due_m3_families` (the nineteen ids `EV-S14`, `EV-S21`,
`EV-S22`, `EV-S23`, `EV-S36`, `EV-S41`, `EV-F01`, `EV-F06`, `EV-F07`, `EV-F10`, `EV-F11`, `EV-F14`, `EV-F15`,
`EV-F17`, `EV-F18`, `EV-F19`, `EV-F21`, `EV-F24`, `EV-F29`; 22 pairs, because EV-F18 has three kill gates and
EV-F21 two); `impact_change_requires_phase_share` (a fixture family scored Impact 5 whose phase share on our arm
is 0.04 → the Impact change is accepted only with that share cited, ceiling 1/(1 − 0.04) = 1.042; an Impact change
with no share → `check` fails); `reader_check_lists_registries` (a scratch registry with no reader in M0 to M3 →
listed with a reduction proposal); `fired_gate_family_renamed` (a family whose kill gate fired, still reading
`adopted` → `check` fails); `ki56_needs_partial_gate_record` (KI-56 `condition_met` with no partial-gate record
cited → `check` fails).

### End-to-end script and logging

`scripts/e2e/decision_records.sh --exit M3` (NAB-50): prints the due list, runs `decision-sweep check --exit M3`,
and asserts the 22 pairs, the conditions of step 2, the share table of step 4 and the reader list of step 7; then
replays the three negative controls below (fixtures under
`tests/fixtures/sweep/M3/controls/` written by this bead, the hook of NAB-50) on scratch copies of the sweep record and asserts that `check` fails on
each with its reason. Logged through the shared helper of STD-E2E to `target/e2e-logs/decision_records/<utc>.ndjson`
with a transcript, every line with `ts`, `script`, `bead` (`NAB-26`), `step`, `event`: `cmd` lines with the exact
command line, exit code and wall time; one `assert` per pair with the kill gate, the ADR identifier and the
outcome; one `metric` per condition of step 2 with the measured number, its unit, the threshold and
`condition_met`; one `metric` per family of step 4 with the phase share and the ceiling; the receipt identifier of
the partial gate and the digests of the gate report and of the sweep record; `assert` lines of kind
`negative-control` with `caught`; on failure the first due item without an outcome, or the first condition whose
number does not resolve to a receipt, with the path of the sweep record; a `summary` with assertions passed and
failed and the kill gates read.

### Known answers and negative controls

Known answers: nineteen families, 22 pairs; thresholds quoted above (0.1% and 5%; 10 times and 85%; 90% and
30%; 90% and 95%; τ 0.7 and 0.5; 60% and 30%). A zero-failure 95% bound at 10⁻⁵ needs ln(20)/10⁻⁵ ≈ 3.0 × 10⁵
nominal episodes. Negative controls: an Impact raised with no phase share (`sweep.change-without-evidence`, by
the evidence-kind rule); KI-56 marked `condition_met` when the comparator tied, its measured count of cells
decided beyond the comparator being 0 against the threshold `>= 1` (`sweep.change-without-evidence`, NAB-22
check (c)); a kill gate that fired at the exit with its family still reading "adopted"
(`sweep.kill-not-reflected`).

## Considerations

- The partial gate is recorded whatever its value. If it shows no gain, the sweep does not lower every score;
  it records the phase ledger, and the rule written in advance for that outcome governs what changes.
- A family whose kill gate fired here is renamed to its fallback in every registry the same day, so that M4
  work is planned against what exists.

## References

Appendix C.1, C.2; Appendix B.5 (contested ideas); §39.12; §0.7; the M3 milestone and its exit review.
