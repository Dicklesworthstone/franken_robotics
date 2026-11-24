---
key: "NAB-45"
title: "registries/ideas.toml: converged ideas IP-23 to IP-48 (safety and proof, search, outputs, studio)"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "delivery", "gauntlet", "solid", "registry", "decision-record"]
parent: "EPIC-M0"
deps: ["NAB-42"]
needs: []
related: ["NAB-46", "NAB-47", "NAB-48"]
gates: []
plan_refs: ["Appendix B.3", "Appendix B.2"]
---
## Background

The ideas behind FrankenRobotics were generated in several independent passes (nine proposal passes, about 225
raw entries, 112 distinct ideas) and winnowed by cross-examination. An idea is **converged** when two or more
passes proposed essentially the same mechanism without sight of each other; convergence is evidence about a
problem, not about a mechanism. This bead carries the second half of the converged ideas into the idea-pass
register (`registries/ideas.toml`, schema `frob.ideapass/1`, NAB-42) as records `IP-23` to `IP-48`, in four
themes: safety and proof; search, statistics and failure analysis; outputs and reality; studio and
measurement.

These rows include the two ideas with the widest convergence of the whole record: counterfactual branching
from a failure snapshot (seven of nine passes) and rare-failure search over worlds with a solvability witness
(six). Both were narrowed under cross-examination before they were placed, which is the rule of reading at
work. Each row gives the idea, the number of passes that proposed it, the mechanism in one line, where it
lives, its tier ([S] Solid, [F] Frontier, [M] Moonshot, or a program rule), its gates, kill gates and bets,
and the beads that carry it. Thresholds are TARGETs owned by their gates. Tier of the bead: Solid.

## Scope

In: records `IP-23` to `IP-48` with carriers. Out: the schema and lint (NAB-42); rows 1 to 22 (NAB-44) and 49
to 88 (NAB-46); the contested records (NAB-47).

## Technical approach

**Transcription rule.** As in NAB-44. `proposals` is the integer in the "Passes" column; two rows have a
qualified count and store it as `proposals_note`: IP-35 ("2, on the problem, with two estimators": two passes
agreed on the problem and proposed different estimators) with `proposals = 2`, and IP-46 ("5 or more") with
`proposals = 5`. `contested` links: IP-25 (runtime shields), IP-26 (the probability bound of the typed
product), IP-33 (what a fork may claim), IP-35 (the estimator for cheap referee promotion), IP-37 (structure
and manufacture inside the loop), IP-45 (verdict reuse beyond a keyed cache). Two rows are *program rules*
with no tier: IP-46 (ten-fold defined as the cost of an accepted design decision) and IP-47 (rival audit
before any ratio); their `tiers` is empty and `tier_text = "Program rule"`. IP-46's `gates` expands "the gates
of §39" to GATE-TENFOLD-01 to GATE-TENFOLD-13, as the program-rule clause of `idea.placed-without-gate`
requires.

**Notes the one-line rows compress.**

- IP-24: reachable-set tables are for subsystems of at most four dimensions, with `uncovered` outside the
  table.
- IP-26: the typed product of evidence classes is in every output envelope; its probability bound is reported
  only with breach labels drawn independently of the monitor.
- IP-27 and IP-28: family certificates are issued on small sub-boxes and per constraint; joint paving of a
  high-dimensional box is refused by a refusal code (`FROB-REF-856`) and is a non-goal.
- IP-29: a ranking that flips under admissible physics variants is `model-conditional` and cannot spend build
  budget.
- IP-32: a shrunk world is labelled 1-minimal under the tested edit relation, and replays bit for bit.
- IP-34: value of information schedules work; its use as a *decision procedure* is a killed idea (KI-45).
- IP-36: a build packet carries every hash and the no-authorization flags; `fabrication_released` is always
  false.
- IP-46: the measure is the restricted mean time to an independently accepted answer against the strongest
  composed rival, with every cost in the denominator.

**The rows.**

**Safety and proof**

| Id | Idea | Passes | Mechanism in one line | Where it lives | Tier | Gates, kill gates, bets | Carried by |
|---|---|---|---|---|---|---|---|
| IP-23 | Energy budget at a physical boundary | 2 | Bound the kinetic, drive, external and stored energy that can reach an absorber and compare it with the absorber's measured lower work envelope; the inequality holds for every contact sequence and friction coefficient | Reflex §22.11 (`.fenergy`); C-31; A-20 | [F] | GATE-SHIELD-01; KILL-ENERGY; BET-25 | BD-108 (A-20), BD-337, SPEC-C-31; N22b-16 (KILL-ENERGY) |
| IP-24 | Reachable-set tables for small hybrid subsystems | 3 | Mode-indexed invariant cells with checked guard and reset inclusions, looked up at run time, with `uncovered` outside the table | Reflex §22.12; proof kind `tube-inclusion` (§12.4) | [M], subsystems of at most four dimensions | GATE-SHIELD-05; KILL-TUBE | BD-440, N22b-35, N22b-36 |
| IP-25 | Runtime shields | 2 | A minimal-deviation filter on the commanded action under reduced-order barrier rows and box limits, with a verified backup | Reflex §22.12 (`frob-shield`) | [F] | GATE-SHIELD-02, -03, -04; KILL-SHIELD. Contested, the contested-ideas record (NAB-47) | BD-338, BD-339, BD-340; N22b-28 (KILL-SHIELD) |
| IP-26 | Typed product of evidence classes | 2 | A result that rests on a proof and on a test has a product type with no conversion to either factor, so the weakest input governs | §8.2; Lemma §12.2; §37.10 | [S] | GATE-LEMMA-06; GATE-SAFE-08. Contested on its probability bound, the contested-ideas record (NAB-47) | BD-005, BD-308, N00-25, N00-26, N00-27 |
| IP-27 | Certificates over families of designs | 3 | Decide a box of designs at once, by a witness that every member satisfies a constraint or a dual witness that none can, in a bundle an independent checker accepts | Augur §25.5, §25.6; C-17, C-32; Lemma §12.4 | [F], on small sub-boxes | GATE-AUGUR-05, -09; GATE-LEMMA-04; KILL-AUGUR-REGION, KILL-FAMILYCERT, KILL-AUGUR-FALSECERT; BET-19, BET-20 | BD-128, BD-132, BD-133, BD-358, BD-364, SPEC-C-17, SPEC-C-32; N12-30 (KILL-FAMILYCERT) |
| IP-28 | Interval and monotone carving of design space | 3 | Evaluate preflight constraints over parameter boxes; monotone maps make corner evaluation exact; bisect into inner, excluded and unresolved boxes | Augur §25.4.6, §25.6, §25.7 | [F], per constraint | GATE-AUGUR-08, -09; KILL-AUGUR-REGION. Joint paving of a high-dimensional box is refused (FROB-REF-856, NONGOAL-11) | BD-288, BD-289, BD-358, N25b-10, N25b-11 |
| IP-29 | Model-form robustness and created-energy audit | 3 | Re-evaluate finalists under admissible physics variants and audit per-step work against energy; a ranking that flips is `model-conditional` and cannot spend build budget | Kinesis §16.9, §16.10; Cortex §23.16; A-19 (§28.4); Chronicle §29.9 | [S] | GATE-FID-05, -06; GATE-ATEL-10; GATE-CORTEX-15; GATE-CHRON-08 | BD-208, BD-266, BD-346, N16b-32, N16b-33 |
| IP-30 | Exact linkage synthesis inside the solver's caps | 2 | Solve precision-pose synthesis in exact arithmetic for small zero-dimensional systems and certify circuit, branch and clearance by enclosure | Augur §25.21.3 (`frob-synth`); C-34 | [F] | GATE-AUGUR-17; KILL-SYNTH-LINKAGE | BD-361, SPEC-C-34; N25d-20 (KILL-SYNTH-LINKAGE) |

**Search, statistics and failure analysis**

| Id | Idea | Passes | Mechanism in one line | Where it lives | Tier | Gates, kill gates, bets | Carried by |
|---|---|---|---|---|---|---|---|
| IP-31 | Rare-failure search over worlds with a solvability witness | 6 | Search a legal scene space for failures instead of sampling it, and count a failure only if the candidate's own body could have completed the task | Nemesis §26.3 to §26.9; C-27 | [S] legality, oracle, guard and bounds; [F] search | GATE-NEMESIS-01, -02, -04, -05, -06; KILL-NEMESIS, KILL-RARE; BET-32 | BD-270, BD-271, BD-272, BD-273, BD-276, BD-277, SPEC-C-27; N26a-12 (KILL-NEMESIS), N26a-32 (KILL-RARE) |
| IP-32 | Minimal counterexample world | 2 | Shrink a failing world's edit list by delta debugging to a world of few edits that replays bit for bit, labelled 1-minimal under the tested edit relation | Nemesis §26.7 (`.fmcw`); C-28; Augur §25.9 | [F] | GATE-NEMESIS-03; KILL-MCW; BET-22 | BD-274, SPEC-C-28; N26a-35 (KILL-MCW) |
| IP-33 | Counterfactual branching from a failure snapshot | 7 | Rewind failing worlds to before the first cause, fork a batch of typed interventions on identical random streams, and rank what would have prevented the failure | Autopsy §27; C-29; forks of §29.8 | [S] | GATE-AUTOPSY-01 to -08; KILL-AUTOPSY; BET-23. Narrowed under cross-examination, the contested-ideas record (NAB-47) | BD-205, BD-279, BD-280, BD-283, SPEC-C-29; N27b-11 (KILL-AUTOPSY) |
| IP-34 | Value-of-information fidelity ladder | 4 | Spend the next episode, or the next rung, where it could change a ranking | Atelier §28.9, §28.10, §28.16; C-26, C-39 | [S] fixed table; [F] conformal survivors and value of information | GATE-ATEL-07, -08, -15; KILL-RANK, KILL-LADDER, KILL-VOI; BET-17. Its use as a decision procedure is in the killed-idea register | BD-310, BD-328, BD-329, BD-351, SPEC-C-26, SPEC-C-39; N28b-20 (KILL-LADDER), N28c-17 (KILL-VOI) |
| IP-35 | Cheap referee promotion | 2, on the problem, with two estimators | Report a referee-level estimate from many paired fast-tier episodes and a few referee episodes at the same world addresses | C-25; Kinesis §16.8; Chronicle §29.11 | [S] | GATE-FID-08; GATE-CHRON-09; KILL-DR. Contested on the estimator, the contested-ideas record (NAB-47) | BD-263, BD-264, SPEC-C-25; N16b-10 (KILL-DR) |

**Outputs and reality**

| Id | Idea | Passes | Mechanism in one line | Where it lives | Tier | Gates, kill gates, bets | Carried by |
|---|---|---|---|---|---|---|---|
| IP-36 | Build packet | 3 | From a qualified design, export a parts list, geometry, nets, budgets and test vectors, with every hash and the no-authorization flags | Foundry §31.12 (`.fbuild`) | [F] | GATE-FOUNDRY-09, -10; KILL-PACKET | BD-407, BD-408; N31b-19 (KILL-PACKET), recorded at the M5 exit review BD-412 |
| IP-37 | Link topology optimization closed into the loop, with tolerance allocation | 4 | Extract load cases from telemetry, size or optimize each link, write its mass and inertia back into the model, and allocate tolerances by sensitivity | Foundry §31.5 to §31.11; C-35; A-15 | [F] | GATE-FOUNDRY-01 to -08; KILL-LOADS, KILL-TOPOPT, KILL-FAB, KILL-TOLERANCE; BET-29. Contested, the contested-ideas record (NAB-47) | BD-400 to BD-406, SPEC-C-35; N31b-16 (KILL-LOADS), N31b-15 (KILL-TOPOPT), N31b-17 (KILL-FAB), N31b-18 (KILL-TOLERANCE), recorded at BD-412 |
| IP-38 | Firmware export for a controller | 3 | Emit a fixed-point controller behind a caller-owned hardware interface, with range proofs and test vectors | Foundry §31.13 | [M] | GATE-FOUNDRY-11; KILL-FIRMWARE | BD-438 |
| IP-39 | Lifecycle compression | 3 | Advance slow damage states by averaged macro-steps estimated from short windows on the current gait, keeping rainflow residues between windows | Foundry §31.14 (`frob-lifecycle`) | [M] | GATE-FOUNDRY-12; KILL-LIFECYCLE, KILL-SINEW-AGE | BD-423; N15b-15 to N15b-17 (KILL-SINEW-AGE) |
| IP-40 | Self-contained replay | 3 | One offline HTML file that replays an episode with its overlays | Sensorium §20.10; Tether §32.15 (`telemetry.export-html`) | [S] | GATE-TETHER-14 | BD-191, BD-300, N24a-41 |
| IP-41 | Actuator identification from logs | 4 | Fit actuator parameters and delay from logged commands and encoder traces, actuators first, and store the result as card fields with their joint covariance | Ground §30.9, §30.10 | [F] | GATE-GROUND-04; KILL-GROUND; BET-28 | BD-390, BD-391 |
| IP-42 | Experiments chosen by the decision they could change | 2 | Rank a probe, an excitation or a measurement by the expected change in a decision per unit cost | Ground §30.8; Atelier §28.16; Augur §25.14; C-39 | [F] | GATE-GROUND-03; GATE-ATEL-15; GATE-AUGUR-11; KILL-VOI, KILL-AUGUR-PROBE | BD-291, BD-351, BD-390, SPEC-C-39 |

**Studio and measurement**

| Id | Idea | Passes | Mechanism in one line | Where it lives | Tier | Gates, kill gates, bets | Carried by |
|---|---|---|---|---|---|---|---|
| IP-43 | Requirements as a typed temporal specification | 2 | A mission is temporal logic over unit-normalized signals; its robustness margin is the acceptance metric and may be a reward term | Arena §24.5, §24.6; C-30; Cortex §23.5 | [S] | GATE-ARENA-03, -09; GATE-CORTEX-10; KILL-SPEC-REWARD; BET-24 | BD-121, BD-122, BD-194, BD-255, BD-257, SPEC-C-30 |
| IP-44 | One verb from a specification to a ranked result set | 2 | Compile task, constraints and budget into check, sweep, race and promote, and return typed verdicts | Tether §32.6.2 (`solve.run`); Atelier §28.11 (`opt.cascade`) | [F] | GATE-ATEL-16; GATE-TETHER-13; KILL-SOLVE | BD-352, BD-372 |
| IP-45 | Content-addressed evaluation cache with dependency invalidation | 4 | Cache every result under the identities it read, and recheck only what an edit touches | Atelier §28.13, §28.14; Phylum §17.11; Augur §25.8 | [S] memo graph and reuse grades 1 and 2; [F] grade 3 | GATE-ATEL-11, -12; GATE-PHYLUM-05; GATE-AUGUR-12; KILL-REUSE, KILL-AUGUR-REUSE; BET-33. Contested, the contested-ideas record (NAB-47) | BD-111, BD-220, BD-286, BD-292, BD-347; N28a-23 (pilot), N28b-36 (KILL-REUSE) |
| IP-46 | Ten-fold defined as the cost of an accepted design decision | 5 or more | Measure restricted mean time to an independently accepted answer against the strongest composed rival, with every cost in the denominator | §2.1; §39; the design benchmark of §24.9 | Program rule | BET-13, BET-14; the gates of §39 | BD-314 to BD-317, BD-428 to BD-433, N00-19 |
| IP-47 | Rival audit before any ratio | 2 | Pin incumbent versions and record their strongest warm configuration on this hardware before a denominator is used | §0.7; §36.3; §39 | Program rule | GATE-SPARK-21 | BD-039, BD-040, BD-142, N40-12 |
| IP-48 | CPU shadow referee | 2 | Idle CPU cores re-run sampled GPU worlds on the referee from snapshots and feed the fidelity ledger | Kinesis §16.11 | [F] | GATE-FID-07; KILL-SHADOW; BET-34 | BD-214, BD-268; N16b-19 (KILL-SHADOW) |

## Acceptance criteria

- [ ] Records `IP-23` … `IP-48` exist in four themes with the columns verbatim and carriers as listed.
- [ ] Every carrier key exists as a bead; every gate and kill gate named exists in the gate register (30
      distinct kill gates in these rows).
- [ ] Six records carry a `contested` link that resolves; IP-34 carries `killed_idea = ["KI-45"]` for its
      strong form.
- [ ] The five rows with no kill gate (IP-26, IP-29, IP-40, IP-46, IP-47) are Solid or program rules.
- [ ] `FROB-REF-856` exists in the refusal registry with the meaning "joint paving of a high-dimensional box
      is refused".
- [ ] The traceability lint of NAB-42 passes on these records.

## Test plan

### Unit tests

`crates/frob-conformance/tests/idea_register.rs`, module `converged_b`:

- `twenty_six_records_in_four_themes` (8, 5, 7, 6).
- `proposal_counts`: IP-33 has 7 (the widest); IP-31 has 6; IP-46 has 5 with a note; IP-35 has 2 with a note.
- `contested_links_are_six`; `kill_gates_are_30`; `program_rules_are_two`.
- `carriers_resolve`; `gates_resolve`.
- `tiers`: [M] is named by IP-24, IP-38 and IP-39.
- `rows_frozen` against `tests/fixtures/ideas/converged_b.tsv`.

### End-to-end script and logging

`scripts/e2e/decision_records.sh` (NAB-50) asserts 26 records and zero unresolved carriers and gates, logged
per STD-E2E.

### Known answers and negative controls

Known answers: 26 rows; themes of 8, 5, 7 and 6 rows; 6 contested; 30 distinct kill gates; with NAB-44 the
converged table has 48 rows. Negative controls: IP-33's proposal count cited as support in a contract
(`idea.cited-as-evidence`); IP-46 given a tier; a carrier removed from IP-37.

## Considerations

- IP-37 (structure and manufacture inside the loop) is adopted as an output-quality subsystem and is not
  counted as a speed lever; each of its stages has its own kill gate with a simpler fallback.
- The two program rules are enforced by gates of the ten-fold test and of the parity baseline, not by a
  mechanism; their carriers are those gates' beads.

## References

Appendix B.3 (last four themes); Appendix B.2.
