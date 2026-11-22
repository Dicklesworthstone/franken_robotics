## Appendix K. Review log and plan-time record

This appendix is the provenance of the plan: what was reviewed, by whom, what was found, and what changed. It
also holds the plan-time checks that chapters cite as "FACT, Appendix K". Artifacts are under `planning/` in the
repository.

### K.1 Rounds

| Round | What was examined | By | Findings | What changed |
|---|---|---|---|---|
| 1 | Version 0.2, mechanically | A script over identifiers and the bead graph | Eight refusal codes referenced without a registry; the stated longest bead chain was wrong | Fixed before any review |
| 2 | Where design time goes; what incumbents already do; mechanisms from other fields; hardware envelopes; backcast and pre-mortem | Five independent passes, each given a neutral case file and one question | 94 atomic claims | Every claim sent to audit; none adopted on its author's word |
| 3 | 76 of those claims | Two independent auditors, re-reading primary sources and redoing arithmetic | 31 verified, 39 partially supported with corrected statements, 2 hypothesis-grade, 2 rejected, 2 non-material; 18 not reached | Thirteen claimed differentiators withdrawn; thirteen reworded with their conditions; performance targets re-based on published incumbent figures; the unit error between world-steps and env-steps fixed; the ten-fold metric defined against an incumbent arm |
| 4 | Ideas for a ten-fold plan | Three agents of different model families: thirty ideas each, cross-scoring, reactions, steelman and blind-spot rounds | Seven ideas scored as strong by every scorer; thirteen killed on score; several blind spots named | Survivors placed in chapters at the tier their evidence supports (Appendix B); kills recorded with their refutations (Appendix C) |
| 5 | Twenty-four hypotheses about where a ten-fold could come from | Four investigators working refuter-first; then a challenge round by agents of two other model families; then an adjudicator | First register: 4 refuted, 18 narrowed, 1 deferred, 1 active. Challenge: 27 contests, 11 upheld, 16 partly upheld, 10 sub-claims rejected | The register of §0.8: sixteen rows corrected, three hypotheses split, five admitted (H-25 to H-29); the answer to the question of record restated as open and unmeasured |
| 6 | Thirteen anticipated hot spots against a corpus of neglected primitives | One harvest, every hot spot labelled anticipated | Eighteen matches, three recommendation cards, several rejections by arithmetic | The memo graph past model identity (§28), slot-level dispatch in the daemon (§32), the tier-correction estimator (§16, §29); rejected primitives in Appendix C |
| 7 | Eleven decision problems | One compilation into mathematical families | Eight families selected, each with artifacts, obligations and a fallback; many methods rejected as ornamental or premature | The decision cores DC-01 to DC-11 of §28 |
| 8 | The two other plans of this project | Two digests, each reading its plan in full and comparing it with version 0.2 | 22 and 23 ranked items where the other plan was better; 13 and 14 where version 0.2 was better; 20 and 19 conflicts; 24 claims not to inherit | The integration charter and its sixteen binding decisions; new chapters §17, §25, §26, §27, §30, §31, §39 |
| 9 | The blended chapters §9–§16, §18, §35 | A mathematical reviewer | 2 critical, 7 major, 17 minor, each with an exact replacement | All applied. The two critical: the variational step named the wrong trivialization (the plan's own negative control); padding with +0 broke capacity independence for sums equal to −0.0 |
| 10 | All blended chapters | A coherence reviewer | 44 findings: 9 where two binding texts could not both be implemented, 23 disagreements, 12 minor | All applied by exact substitution (131 substitutions with round 9) |
| 11 | Coverage of the two other plans and of the idea catalog | Two completeness auditors | 315 checklist rows: 260 covered, 33 weaker, 19 missing, 3 deferred; 51 contradictions noticed | 48 subsections inserted; the contradictions fixed in a reconciliation pass (274 fixes logged) |
| 12 | The two other plans' later revisions | A final delta auditor | 54 changes: 8 already covered, 21 weaker, 13 missing, 12 not taken | 34 corrections and 18 further subsections |

### K.2 Revisions of the two other plans that this version integrates

Both documents were revised by their authors while this version was being written. The digests of round 8 read
them as they stood early in the evening of 2026-11-21; rounds 11 and 12 read their later revisions.

| Document | Last revision integrated | Changes after the digest that were taken |
|---|---|---|
| `COMPREHENSIVE_PLAN_FOR_FRANKEN_ROBOTICS__ASTRA.md` | Commit `47e8e89`, 2026-11-21 22:07 | A fifth foundation defect (the joint law of sampled actions in the suite's trainer); two corrections to the ten-fold statistics; the withdrawal of one overhead attribution; controller class as part of a family answer; finite-tolerance docking over a gain interval; channel laws; sampled damping and the work of a held command; free-period orbits; further sibling facts |
| `COMPREHENSIVE_PLAN_FOR_FRANKEN_ROBOTICS__NEON.md` | Commit `1f5703d`, 2026-11-21 21:55 | The absence of 64-bit atomics on Metal and the overflow rule it requires; conflicting exact pins of a JSON crate across siblings; a mixed-frame wrench type in a sibling; corrected known answers; four killed ideas |

A later revision of either document is not reflected here and should be diffed against these commits.

### K.3 Plan-time checks

Chapters cite these as "FACT, Appendix K". They are paper-scale: static, scalar or toy-dynamics programs with
preregistered expectations. They were run by the authors of one of the two other plans and are recorded with
receipts in the reproduction bundle named in Appendix H. This program re-derived several of their values by hand
or by script during rounds 8, 11 and 12 and did not re-execute the bundle. **None of them uses FrankenSim
dynamics, contact or a real sensor pipeline; the integrated system remains a hypothesis.**

| Check | Result as recorded | What it supports |
|---|---|---|
| Stance family | 11,520 of 11,520 classifications agree with the exact classifier; minimum half-width 3/14 m | The family-answer fixture of §25 |
| Material probe | 675 of 675 correct in class; 225 of 225 reversed under a delayed law; direct dwell 900 of 900 | A probe that is perfect in class can reverse on held-out cases (§25) |
| Sensor alias | 27 of 32, 459 of 512, 3,807 of 4,096; the common-mode case stays at 0.75 | Observation aliasing bounds what a better body can do (§20, §25) |
| Docking | Feasible if and only if ε < (U + ½)/4; 561 of 561 agreement with a linear program | The docking fixture (§20, §24) |
| Docking with three gains | Exact docking unsatisfiable; 34,590 of 34,590 labels reverse | A finite tolerance is part of the mission (§25) |
| Solver pilot (pusher) | 20 of 20 correct on both arms; 1.345 times warm and 1.088 times cold; a residual of 575.7 µs that the solver does not touch; ceiling about 1.46 | Compute the ceiling before a speed test (§0.7, §36) |
| Sill, planar and static | 94 of 156 agreement; a change of winding law flips 24 of 156 | Model form changes rankings (§16) |
| Reuse and deadline | A correctly keyed dependency graph ties a semantic receipt on these cases | Memoization is hygiene, not a lever (§28) |
| Integer predicates on devices | Identical labels on an M5 and on two RTX 4090; the host integer path was fastest | Exact-integer kernels for strict predicates (§21, §35) |

Checks made by this program's own investigators during round 5, with scripts that were not retained and must be
re-run before a gate cites them: CPU reference rates for a humanoid in double precision on this machine;
condition numbers of sampled contact Hessians against half-precision round-off (the refutation of half-precision
solves, Appendix C); the Metal working-set limit and largest single buffer (§36).

### K.4 What remains unsettled

The open disagreements are listed at the end of §0.8, each with the test that settles it. Three matter most:
whether a short program over an incumbent engine recovers most of the gain (the recovery ratio of §39); whether
cheap evaluation rungs rank designs as trained specialists do (the rank-agreement experiment of §23 and §28);
and whether design missions are dominated by training or by agent turns and diagnosis (the phase ledger of §39).
Nothing in this plan should be read as having answered them.

### K.5 Limits of this record

Eighteen audited-claim candidates were never audited. The idea duel's synthesis was compiled by an agent of the
same model family as one duellist. The first verdict register was adjudicated from evidence gathered by
investigators of one model family; the challenge round is what corrected it. Web sources were often read through
a summarizing fetch, so quoted figures must be re-read from the primary source before a gate cites them
(Appendix H marks which). The reviews of rounds 9 and 10 covered the chapters as they stood before rounds 11 and
12 inserted 66 subsections; those insertions have had no independent review.
