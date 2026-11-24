---
key: "N41-RISK2"
title: "risks.toml, part 2: thesis, ten-fold and program risks (RISK-09, -10, -25 to -30, -33, -43, -46, -51 to -53, -59)"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "delivery", "tenfold", "gauntlet", "registry", "solid"]
parent: "EPIC-M0"
deps: ["N40-10", "N41-01"]
needs: []
related: ["N41-RISK3", "N41-RISK4"]
gates: ["GATE-WS-07"]
plan_refs: ["§41.3", "§41.4", "§41.5", "§2", "§39", "§36.14", "§43"]
---
## Background

This is the second part of the risk register (conventions, severity definitions and the "no risk is mitigated by a
hope" rule are stated in part 1, N41-RISK1, and enforced by N41-01). Its fifteen rows are the ways the **thesis**
can fail and the ways the **program** can fail to find out: the ten-fold claim rests on a measured rival, on cheap
rungs that rank designs as the expensive ones do, on family policies, on mixed batches and on a tournament with
enough power; and the program rests on compute the owner must approve and on decisions that must arrive in time.

The thesis in one paragraph, for a reader of this bead alone: the product claims that an agent can go from a brief
to an accepted, independently checked robot design ten times faster than with the strongest composed incumbent
tools, on at least one named class of missions. No single lever delivers that. The levers are: decide many designs
per turn; train few; spend where it matters; find failures instead of waiting for them; answer over families
instead of points; do not recompute or re-decide; explain by intervention; short setup. Each has its own early
experiment and a fallback that ships under its own name. A ratio between 3 and 10 is described by its value, and
under 3 the product is described by what it lets a reader audit. The rows below are where each lever, and the measurement of
the whole, can go wrong.

## Scope

In: the fifteen rows below as records of `registries/risks.toml`. Out: the schema, loader and exit review (N41-01);
the rare-tail planted instance and its GATE-WS-07 row (N41-31); the mitigations (the beads named per row).

## Technical approach

Record fields as in N41-01. Terms used in the table: *paired lower bound* is the lower confidence limit of a
throughput ratio measured on interleaved paired runs; *τ* is the top-weighted rank correlation between a cheap rung
and the trained, promoted rung; *E0 to E3* are the compute envelope classes (E0: 16 GiB, 8 cores, one hour, one
device; E1 a named gate measurement; E2 a design campaign of 48 device-hours; E3 pretraining of one family policy,
500 device-hours approved per run).

| ID, severity | Risk | Earliest signal (detector) | Mitigation | Owner | Detected or decided by | Beads that carry the mitigation | Planted instance; first readable |
|---|---|---|---|---|---|---|---|
| RISK-09, Medium | `trj` (the two-GPU machine) is heavily loaded, so measurements are noisy and compiles slow | The spread of A/A campaigns; the recorded host load in disclosure blocks | The binary kernel cache; interleaved runs; admission by paired lower bound; the A/A known answer of GATE-SPARK-19 | Performance lead | GATE-SPARK-19 | BD-216, N38a-08, BD-039 | An A/A campaign with a synthetic background load in one arm: the spread exceeds the band and the measurement is not admitted. M2 |
| RISK-10, High | No compute envelope is approved, so the performance gates, the rank-agreement experiment and pretraining cannot run as specified | OPEN-SPARK-2 at class E0 when M1 opens | The classes E0 to E3 are proposed at M0; every package above E0 names its budget; variants of M2 and M4 that fit E0 | Owner | The owner decision; job admission against the envelope in force, refusal `FROB-REF-761` (or `FROB-REF-704` for a campaign-class job; both exit 4) | N21c-35, N44-01, N44-02, N36-20, N40-33, N40-34, N40-35 | A run record of class E2 with no approval in a scratch approvals registry is listed as over the envelope, and the M0 listing shows OPEN-SPARK-2 at class E0. Further observation, bound at M2: a job of class E2 submitted with no approval on record is refused before it starts (`FROB-REF-761`, exit 4). M0 |
| RISK-25, High | The denominator of the ten-fold ratio is wrong: incumbents are faster, or their loop is shorter, than the composite assumed | The M0 phase ledger (the published context is about 410 s to threshold on one RTX 4090, of which about 45 s is compilation) | GATE-SPARK-21 before any mechanism is funded against the rival; the ceiling-first rule; pinned comparator revisions (OPEN-PERF-2) | Gauntlet lead | GATE-SPARK-21; GATE-TENFOLD-01; GATE-TENFOLD-09 | BD-039, BD-040, BD-142, N40-12, N40-18, N00-17, N00-09, N41-23 | A funding record for a speed lever whose ceiling, computed from the measured ledger, is below its claimed ratio is refused. M0 |
| RISK-26, High | A short robust program over an incumbent engine reaches the same accepted answers in comparable time | The composed arm within a factor of 1.5 on a stratum at the M3 partial gate | BET-14; KILL-AUGUR-TIE; the layer ships over that engine for the stratum; research budget moves to the first mission the rival cannot resolve | Project lead | GATE-TENFOLD-06; GATE-TENFOLD-13; KILL-AUGUR-TIE; KILL-TENFOLD-TIE | BET-14, N25a-18, N40-15, N40-22, BD-316, BD-317 | A synthetic partial-gate ledger with the composed arm at 1.3 times: the tie rule fires and the wording changes to the tie statement. M3 |
| RISK-27, High | Rank agreement between rungs is too low for cheap rungs to eliminate designs | τ under 0.5 at the lower bound in the pilot; verdict flips after finalist training | GATE-REFLEX-04, GATE-CORTEX-12, GATE-ATEL-07; the rung modes (`discard-only` from 0.5, `uninformative` below it or with no record); the audit lane; KILL-RANK, with the lost factor recorded in the ten-fold report | Atelier lead | GATE-CORTEX-12; GATE-ATEL-07; GATE-REFLEX-04; KILL-RANK | BD-145, BD-310, BD-328, N28b-23, N22c-51, BET-17, N41-08 | A rung with no rank-agreement record asked to select is refused; a synthetic ledger with τ = 0.4 puts the rung in `uninformative`. M3 |
| RISK-28, High | Family policies do not transfer, lose plasticity along lineages, or their pretraining is never approved | OPEN-CORTEX-1 at its default; the pre-funding pilot (return with no adaptation under 70% of specialists); the ten-edit chain of GATE-CORTEX-13 | KILL-FAMILY; KILL-INHERIT; scheduled resets; the variant of M4 with no family policy | Cortex lead | GATE-CORTEX-11; GATE-CORTEX-13; KILL-FAMILY; KILL-INHERIT | N23b-24, N23a-26, N23a-30, BD-320, BD-321, BD-322, N40-34, BET-15, BET-16 | A family policy applied outside its class is refused; a ten-edit lineage with resets disabled fails the chain's control. M4 |
| RISK-29, Medium | Mixed batches lose to per-family templates or to an incumbent's mixed batch | The M1 pilot of padding overhead and bucket count; the share of the mutation corpus that the template arm expresses (at least 95% is the adverse signal) | The head-to-head with outcomes declared in advance; KILL-PHYLUM; KILL-BUCKET; parameter variants within one topology are Solid and unaffected | Phylum lead | GATE-PHYLUM-03; KILL-PHYLUM; KILL-BUCKET | BD-144, BD-185, BD-186, N17b-10, N21b-19, BET-18 | A synthetic design population that needs more than eight buckets to hold padding waste at or below 25%: the pilot reports the waste and the fragmentation refusal `FROB-REF-741` (exit 4), entered as adverse for KILL-PHYLUM. Further observation, bound at M2: a head-to-head record in which the template arm expresses 96% of the corpus and wins the first measure makes the pre-declared outcome rule fire KILL-PHYLUM and withdraw the mixed-batch differentiator. M1 |
| RISK-30, High | Engine throughput or training time lands below the parity band | A paired lower bound under 0.67 of the incumbent; GATE-CORTEX-05 above 1.25 times the incumbent | KILL-PARITY (fires below 0.33); KILL-LEARNKERN; the profile decides where effort goes | Performance lead | GATE-SPARK-03; GATE-CORTEX-05; KILL-PARITY; KILL-LEARNKERN | BD-217, BD-218, N23a-16, N23b-33 | A synthetic paired measurement at 0.30 of the incumbent makes the KILL-PARITY trigger evaluate to "fires". M2 |
| RISK-33, High | Breadth starves the spine: forty-three crates, nine milestones and more than four hundred gates spread a swarm thin | Lines added to Frontier-only and Moonshot-only crates while a critical-path exit slips; more than four Moonshot items active; a work package funded without its pilot | Doctrine rule D-6 and the cap of four active Moonshot items; rule L-12 (features that select a tier are off by default); pilots before funding; the critical path; the schedule gate (BET-12) | Project lead | GATE-PROG-SCHEDULE; GATE-GAUNT-08 | N40-11, N40-06, N38a-09 (which superseded N00-07), N40-05, BET-12, N34-03, N41-09 | Activating a fifth Moonshot item is refused by the cap check; a work package marked funded with no pilot record is refused. M1 |
| RISK-43, Medium | Calibration data never arrives | No admitted dataset when M5 opens | Reality is an import lane with a recorded `no-claim`; the synthetic rehearsal; bench packages that may be sourced from M0 | Owner | GATE-GROUND-10 (an import lane); GATE-GROUND-01 | BD-397, BD-139, N41-10, WP-69 | A report that prints a Reality band with no admitted dataset is refused; the reality line prints `no-claim`. M1 |
| RISK-46, Medium | Scaling across two cards or two machines vanishes under transfer cost, or under vision workloads | Strong scaling under 1.25 at GATE-SPARK-20; the fleet pilot | KILL-SHARD; KILL-FLEET; the two cards run as independent lanes; only seeds, weights and receipts cross | Spark lead | GATE-SPARK-20; KILL-SHARD; KILL-FLEET | BD-199, BD-370, N21c-10, N21c-11 | A synthetic scaling record of 1.1 makes the KILL-SHARD trigger evaluate to "fires". M2 |
| RISK-51, High | The tournament cannot confirm a true ratio of ten, for lack of power or because unfinished missions cap the ratio at 1/q (q the unresolved share) | The power analysis before the suite is opened; the share of missions unresolved at the deadline in the M3 partial gate | A pooled ratio whose pre-analysis is recorded before the suite is opened; a finite-suite result at 24 families and a population claim only at 32 or more; completion treated as a first-order goal | Gauntlet lead | GATE-TENFOLD-10; GATE-TENFOLD-04; KILL-TENFOLD-REACH | N40-19, N40-14, N40-15 | A pre-analysis for a cell whose confirmatory verdict is unreachable at its family count opens the cell as `descriptive` before any sealed run. M3 |
| RISK-52, Medium | No neutral party or independent evaluator is named, or the repository stays private, so nobody outside can confirm the result | OPEN-PROG-1 and OPEN-PROG-2 at their defaults when the pins are frozen for M7 | The replication kit; `frob-verify` as a standalone binary; the label `self-adjudicated` on any ratio measured without a neutral party | Owner | GATE-TENFOLD-05; GATE-TENFOLD-08; KILL-TENFOLD-ADJ | N41-05, N41-06, N40-17, BD-432, BD-026 | A report text containing "independently accepted" with no neutral party on record is refused by the wording lint. M3 |
| RISK-53, High | Owner decisions arrive after the gates they block | An open-question row at its default past its deadline | Fail-closed defaults; each question is asked one milestone early; variants that need no decision | Owner | The exit review's open-question check | N40-35, N41-03, N40-33, N40-34, N40-32 | A register row past its deadline with no decision record and no default record blocks the exit. M0 |
| RISK-59, High | A rare slow mission is absent from the sealed sample, and a resampling interval certifies a ratio the population does not have | The pre-analysis of the ten-fold test reports a family count below the count its registered bound needs; every sealed mission of an arm finishes near its fastest time | The bounded paired contrast; the two result types (finite-suite and population); a stratum below the necessary count reports a finite-suite result | Gauntlet lead | GATE-WS-07 (M3 row); GATE-TENFOLD-10 | N40-19, N41-31 | The rare-tail arm of the pre-analysis: with a deadline of 1,000 s the rival always takes 20 s and the candidate takes 1 s on 99% of families and 999 s on 1% (population ratio 1.8215); a suite of 24 families misses the tail with probability 0.78568, and a population claim at 24 families is refused while a percentile bootstrap on that sample would report 20. M3 |

## Acceptance criteria

- [ ] `registries/risks.toml` contains the fifteen records above with no empty field; every `beads` and `detects`
      entry resolves.
- [ ] Every row names a planted instance and its first-readable milestone; RISK-59 carries GATE-WS-07.
- [ ] Rows owned by "Owner" (RISK-10, -43, -52, -53) each cite the open question whose default is their fail-closed
      state. That each cited question's record exists in the open-question register is the cross-register check
      of N41-01 (`risks check --cross`), run on the full registers by N41-04 and at every exit, because the
      question rows are entered by other beads (N41-OPEN3, -6, -8); this part's test checks the citations.
- [ ] The loader's rule test passes on these rows: no mitigation is a bare hope or a bare upstream work item.

## Test plan

### Unit tests
`crates/frob-conformance/tests/risks_part2.rs`: `fifteen_rows_load`; `ids_match_expected_set`;
`owner_rows_cite_open_questions` (RISK-10 → OPEN-SPARK-2; RISK-43 → OPEN-GAUNT-1 and OPEN-GROUND-2; RISK-52 →
OPEN-PROG-1 and OPEN-PROG-2; RISK-53 → the register itself); `severity_counts` (10 High: RISK-10, -25, -26, -27,
-28, -30, -33, -51, -53, -59; 5 Medium: RISK-09, -29, -43, -46, -52); `every_bead_key_resolves`.

### End-to-end script and logging
Covered by `scripts/e2e/registers_exit.sh` (N41-04): for each row whose first-readable milestone has been reached
the planted instance is fired and an `assert` line records the observed signal and the detector's verdict.

### Known answers and negative controls
Known answers: the row count, the identifier set, the severity tally, the thresholds repeated in signals (0.67 and
0.33 of parity; 1.25 for training time; 1.5 for the tie; τ of 0.5 and 0.7; 24 and 32 families; scaling 1.25), each
of which the unit test compares with the gate register so that a drift is caught (the gate register governs).
Negative controls: a row whose threshold differs from the gate register; an owner row with no open question.

## Considerations

- Several of these risks are not avoidable, only measurable early: the quadruped ceiling cannot be engineered away,
  and the class may leave the claim. The register's job is that the program says so first.
- Three rows (RISK-26, -27, -28) have fallbacks that shrink the engine's share of the product rather than stop it;
  the exit review records which fallback is in force by its own name.
- The first-readable milestone of a row is the earliest exit at which its signal can be evaluated; before it, the
  review records "not yet readable", which is different from "not fired".

## References

§41.3 (rows RISK-09, -10, -25 to -30, -33, -43, -46, -51 to -53), §41.5 (the suite row), §41.2, §2.3, §2.6, §2.7,
§36.14, §39.7, §39.20, §42.2.
