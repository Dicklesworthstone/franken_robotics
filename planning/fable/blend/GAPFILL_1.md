# GAPFILL 1: completeness audit of the hybrid parts against the two source plans

Scope: Phase 1 parts `parts/P00.md`–`P12.md` (§1–§37). Chapters §38–§47 and the appendices are not written yet, so
anything the charter assigns to them is absent from the parts by construction; where a checklist item lands there,
the status says so and an insertion block supplies the text. Method: grep of the parts for every mechanism, fixture
and number in the checklists, targeted reads of the hits, and a re-read of both source plans against their digests.
Arithmetic in the insertion blocks was recomputed for this file; sibling facts were re-read in source on 2026-11-21.

**Headline finding.** Both source plans changed after their digests were written, not only the third.

- Second plan (ASTRA): commit `5afef55` at 21:27:18, after the digest (19:59:32). 123 lines added, 28 removed. It adds
  a fifth foundation defect, eight new counterexamples with rules, two corrections to the ten-fold statistics, and it
  withdraws the attribution of the pusher overhead. None of this is in the digest, and almost none is in the parts.
- Third plan (NEON): the supplied diff equals commit `d18b5a1` (21:19:57). The working tree was edited again at
  21:38:02 (56 lines replaced, uncommitted, not in the supplied diff): dependency-pin facts, layout arithmetic, a
  46-tool list.

Status tokens: COVERED, WEAKER, MISSING, DEFERRED (owned by the lead or by an appendix; correctly absent from
chapters), DROP (correctly absent under the do-not-inherit list or superseded by the parts' own source reading).

---

## (a) Coverage table

### A.1 Second plan, digest section A (22 ranked items)

| # | Item | Source | Status | Where covered, or what is missing |
|---|---|---|---|---|
| 1 | Mission queries: quantifiers, I ⊆ F ⊆ O, six verbs, schema, four proof rules | ASTRA A1 | COVERED | P07 §25.3 (verbs, `.fmission`), §25.4 (semantics, progress, four rules), §25.5; P01 C-17. Naming differs between chapters, see (d)1 |
| 2 | Success defined against the strongest rival's whole workflow, with kill rules | ASTRA A2 | WEAKER | Estimand, 1/q ceiling, one-tenth rule, cold and warm, crossover count, disqualification, tie rule: P00 §2.1, §2.3, BET-13, BET-14; P07 KILL-AUGUR-TIE. Missing because §39 is unwritten: rival classes and what must be earned, protocol controls, the sealed-suite table, the second claim, statistical detail. Blocks 4, 5 |
| 3 | Plan-time pilots with negative results; a pilot before funding; ceiling before a speed test | ASTRA A3 | COVERED | P00 §0.7, §0.8; P07 §25.22; P05 §36.5. One figure's attribution is now withdrawn at source, see (d)7 |
| 4 | Information, time and causality: seven instants, ordering rule, queues in checkpoints, causal test, indistinguishability, docking trade-off | ASTRA A4 | COVERED | P01 §11.7, §11.8, C-19; P04 §20.4, §20.15, §20.16 |
| 5 | Edits invalidate evidence: revalidation, assumption lattice with polarity, feasibility and optimality separate | ASTRA A5 | COVERED | P01 C-18; P09 §28.13, §28.14; P07 §25.8 |
| 6 | Fully specified, pre-analysed first experiments (cart, split-grip quadruped) | ASTRA A6 | COVERED | P08 §24.10 (n = 64 enumeration, tie first reachable at n = 86), §24.11, §24.12 |
| 7 | Correct gym semantics | ASTRA A7 | COVERED | P06 §23.3 (E-1 to E-9, bootstrap, physical discount, time-aware rewards), §23.6 (P-1 to P-11) |
| 8 | Defects found in the foundations | ASTRA A8 | COVERED | P12 §33.11 (four rows with hand cases), asks FD-FSIM-10 to -13, FD-FNP-01; P00 §1.7. A fifth defect was added at source after the digest: Δ1 below |
| 9 | The step is a transaction | ASTRA A9 | COVERED | P03 §16.5 (commit fence, restore set, Brownian-bridge refinement of noise, witness migration), §16.3 |
| 10 | Crash-consistent job and storage protocol | ASTRA A10 | COVERED | P11 §32.5, §32.7, §32.8, §32.10; P09 §29.4 |
| 11 | Relations verified directly: product dominance, separating worlds, pairing by physical field | ASTRA A11 | COVERED | P01 C-40; P07 §25.9, §25.16 |
| 12 | Counterexamples as first-class output | ASTRA A12 | COVERED | Every chapter's known-answer table; P01 ledger lists the Appendix N seeds (two-latch, tied sliders, 841/800 midpoint, pairwise labels, bus schedule, four-point registration, scale-fitted scoring) |
| 13 | Hardware realism in small things | ASTRA A13 | COVERED | P05 §21.8 (representation before placement), §21.9 (readback rules R1–R6), §21.12, §36.6, §36.12, §36.13; P06 §23.9 |
| 14 | Family batching with the speedup bound | ASTRA A14 | COVERED | P03 §17.5, §17.6; P05 §36.5 |
| 15 | Hybrid-systems mathematics | ASTRA A15 | COVERED | P01 C-16, T-18; P06 §22.9; P07 §25.20 |
| 16 | A sim-to-real procedure | ASTRA A16 | COVERED | P10 §30.6–§30.13, §30.17 |
| 17 | Probe and instrument design | ASTRA A17 | COVERED | P07 §25.14 (grammar, 675 of 675 and 0 of 225); P09 §28.16; P01 C-39 |
| 18 | Scene acceptance | ASTRA A18 | COVERED | P04 §19.5.1–§19.5.5; P03 §18.3 |
| 19 | Symmetry done carefully | ASTRA A19 | COVERED | P01 C-09; P02 §13.4; P06 §22.13, §23.13 |
| 20 | Facility co-design, handoff, factory labels, skill transport | ASTRA A20 | COVERED | P07 §25.11 [M], §25.17 [M], §25.18 [F]/[M] |
| 21 | Multi-model adversarial ideation with scores and withdrawals | ASTRA A21 | DEFERRED | Appendices B, C, K, written by the lead; scores may not appear in chapters (style rule 1) |
| 22 | Release rubric, stop decisions per milestone, risks keyed by earliest signal | ASTRA A22 | WEAKER | Stop decisions exist for Augur only (P07 §25.23). Rubric and risk rows belong to §47 and §41, unwritten. Blocks 15, 16 |

### A.2 Second plan, digest section E (18 landing sites)

| # | Material | Source | Status | Where |
|---|---|---|---|---|
| 1 | Mission queries, semantics, proof rules, certificates | ASTRA E1 | COVERED | P07 §25.3–§25.5; P00 §8; P01 §12.2 |
| 2 | Ten-fold test and competitor table | ASTRA E2 | WEAKER | As A2 |
| 3 | Clock contract, sensors, causality | ASTRA E3 | COVERED | P04 §20.4–§20.8, §20.15; P01 §11.7–§11.8 |
| 4 | Transactional step, refinement law, coupled constraints, new fixtures | ASTRA E4 | COVERED | P03 §16.5, §18.4, §18.16 (thin barrier, tied sliders) |
| 5 | Gym and PPO rules | ASTRA E5 | COVERED | P06 §23.3, §23.6, GATE-CORTEX-06, -07 |
| 6 | Job, chunk and idempotency protocol | ASTRA E6 | COVERED | P11 §32.7–§32.10; P09 §29.4 |
| 7 | Revalidation and reuse | ASTRA E7 | COVERED | P09 §28.13–§28.15; P07 §25.8 |
| 8 | Fork classification, first-divergence algorithm | ASTRA E8 | COVERED | P08 §27.4, §27.10; P09 §29.8 |
| 9 | Cart and quadruped fixtures | ASTRA E9 | COVERED | P08 §24.10, §24.11 |
| 10 | Saltation, return maps, symmetry–event identity | ASTRA E10 | COVERED | P01 C-16, T-18; P07 §25.20 |
| 11 | Stabilizer intersection, equivariance in law | ASTRA E11 | COVERED | P01 C-09; P06 §22.13 |
| 12 | Scene acceptance and enclosures | ASTRA E12 | COVERED | P04 §19.5; P03 §18.3 |
| 13 | Placement receipts, exact kernels, family buckets, PPO normalization | ASTRA E13 | COVERED | P05 §21.8, §21.12; P03 §17.5; P06 §23.9 |
| 14 | Sim-to-real procedure | ASTRA E14 | COVERED | P10 §30 |
| 15 | Foundation defects as facts and asks | ASTRA E15 | COVERED | P12 §33.8, §33.11 |
| 16 | Rejection fixtures and counterexamples | ASTRA E16 | COVERED | Chapter known-answer tables; ledgers |
| 17 | Idea ranks 16–30, R1–R3 with activation triggers | ASTRA E17 | DEFERRED | Appendices B and C |
| 18 | Release rubric, new risk rows | ASTRA E18 | MISSING | §41, §47 unwritten; no draft text in the parts. Blocks 15, 16 |

### A.3 Third plan, digest section (A) (23 ranked items)

| # | Item | Source | Status | Where covered, or what is missing |
|---|---|---|---|---|
| 1 | Error control across a campaign: sealed lane, α_i = α/[i(i+1)], 500-retry gate | NEON A1 | COVERED | P08 §24.13, GATE-QUAL-01 to -03; P11 §37.5; P00 principle 9 |
| 2 | Many different robots in one batch, with the per-template control arm | NEON A2 | COVERED | P03 §17.3–§17.5, §17.14; P05 §21.11 |
| 3 | Sensors in feasibility and symmetry: Γ_obs, alias witness, observability Gramian | NEON A3 | COVERED | P02 §13.4; P04 §20.11–§20.13; P01 C-19, C-20 |
| 4 | Minimal counterexample worlds | NEON A4 | COVERED | P08 §26.3–§26.7 |
| 5 | Attribution by intervention, design parameters re-run from the start | NEON A5 | COVERED | P08 §27.3–§27.9 |
| 6 | Energy-budget safety tier, claim product, no-authorization flags, shield with optimality check | NEON A6 | COVERED | P06 §22.11, §22.12; P00 §8.2; P11 §37.9 |
| 7 | Rank robustness with consequences; created-energy audit; audit of discards | NEON A7 | COVERED | P03 §16.9, §16.10; P06 §23.16; P08 §26.10; P09 §28.11, §28.12. Two labels and thresholds for one audit, see (d)4 |
| 8 | A mission language above reward terms | NEON A8 | COVERED | P08 §24.4–§24.7; P01 C-30 |
| 9 | Two-sided navigation certificates; the building as a design variable | NEON A9 | COVERED | P04 §19.11, §19.12 |
| 10 | Worlds the robot changes; sensors that fail realistically; hazard lists | NEON A10 | COVERED | P04 §19.8, §19.10, §20.6–§20.8; P03 §18.8 |
| 11 | Campaign deduplication with stated limits | NEON A11 | COVERED | P03 §17.11; P02 §14.6; P01 C-12, C-22; P09 §28.15 |
| 12 | Gaits as orbits; lifecycle | NEON A12 | COVERED | P06 §22.9 (including simultaneous impacts, which the source refused); P10 §31.14 |
| 13 | Constructive linkage synthesis | NEON A13 | COVERED | P07 §25.21.3, §25.21.4; P01 C-34 |
| 14 | Joint-level and actuator physics | NEON A14 | COVERED | P02 §15.5–§15.10; P03 §18.6, §18.7 |
| 15 | The form of experiments; a measurement protocol; a gate that freezes incumbents first | NEON A15 | WEAKER | Form adopted (P00 §4 preamble, §0.7; P05 §36.6, §36.8). The freeze gate is only referred to (P03 OPEN-PHYLUM-4 cites "the rival audit (§39)"). Block 5 |
| 16 | Conventions as formulas | NEON A16 | COVERED | P01 §11.2–§11.6; P02 §13.3, §14.5 |
| 17 | Named artifacts, offline auditor, log per bead | NEON A17 | COVERED | P12 §34.9, §34.11; P09 §29.14 |
| 18 | Division of labour between the machines; fingerprint; lease; canary | NEON A18 | COVERED | P05 §21.10, §21.14, §21.16, §21.17; P00 §1.8 |
| 19 | Agent surface details | NEON A19 | COVERED | P11 §32.12, §32.14, §32.15 |
| 20 | Manufacturing and firmware as outputs | NEON A20 | COVERED | P10 §31.12, §31.13; P00 NONGOAL-04 narrowed |
| 21 | The `fs-qty` nightly coupling | NEON A21 | COVERED | P12 §33.12 (matrix, decision rule, escape hatch); ask FD-FSIM-15 |
| 22 | Killed-idea register with the refuting calculation; grounding table with consequences | NEON A22 | DEFERRED | Register is Appendix C. Chapter-level equivalents exist: P05 §21.20, P09 §28.19 (not-taken table), P00 §1.1 (consequence column). New killed rows offered in (b) |
| 23 | Skill artifacts with infeasibility witnesses | NEON A23 | COVERED | P06 §22.10 |

### A.4 Additions at source after the digests (not in either digest)

| # | Item | Source | Status | Where covered, or what is missing |
|---|---|---|---|---|
| Δ1 | Action sampler with correct marginals and a wrong joint law (sign agreement 0.6655) | ASTRA §9.3 new | MISSING | P06 §23.6 has P-1 to P-11; none tests the joint law. Blocks 1, 13 |
| Δ2 | Duplicate member names after escape decoding, refused before planning or charging | ASTRA §3 new | WEAKER | P02 §14.4 rule 1 says "a duplicate key refuses"; no mechanism, no escape rule, no admission-order rule. Block 12 |
| Δ3 | Contact-cone check at lift-off for one wheel | ASTRA §4.4 new | COVERED | P08 §24.10 row "Front lift-off" has the same thresholds (0.7500, 0.6459). The single-wheel reversal is offered as a known answer in block 17 |
| Δ4 | A circle-valued joint only where every law is periodic | ASTRA §5.1 new | WEAKER | P01 §11.1 and P02 ADR store a turn count; no rule for lifted laws, no encoder alias, no fixture. Block 11 |
| Δ5 | The solver's kinetic-energy correction is not physical heat | ASTRA §6.1 new | MISSING | P09 §29.9 has one line "contact dissipation" and a residual; nothing separates the predictor exchange. Block 9 |
| Δ6 | Sensor overload acts on force over time; clip before or after averaging | ASTRA §8.1 new | MISSING | P04 §20.5 averages the wrench over the exposure and adds each impulse once. Block 10 |
| Δ7 | The actor's information contract includes rewards, `info`, flags, masks | ASTRA §9.3 new | MISSING | P06 E-8, E-9 and P04 §20.15 cover observation channels only. Block 2 |
| Δ8 | Compare soles on the commanded wrench slice, not scalar capacity | ASTRA §13.2 new | WEAKER | P08 §24.11 reports the yaw moment at the slip limit; it has no capacity at a commanded moment. Block 8 |
| Δ9 | A non-monotone friction sweep refutes monotone fill | ASTRA §13.2 new | MISSING | P07 rule 3 requires conditions for inheritance; no rule against filling a sweep, no detector. Block 7 |
| Δ10 | A recorded score crossing is not population attainment | ASTRA §13.4 new | MISSING | GATE-CORTEX-05 and §36.3 rows 4 and 9 use "time to the return threshold" with no rule for repeated looks. Block 3 |
| Δ11 | Probabilistic claims do not transport through successor matching | ASTRA §18.5 new | MISSING | P07 §25.11 has the alternating simulation only. Block 6 |
| Δ12 | Restricted-time ratio against accepted-output rate; timestamp before the joint reveal | ASTRA §19.2 new | MISSING | Block 4 |
| Δ13 | Percentile bootstrap fails on a rare slow tail; a bounded paired contrast | ASTRA §19.2 new | MISSING | P00 §2.1 defers the rule to §39. Block 4 |
| Δ14 | Pusher overhead: residual by subtraction, no longer attributed to checking and serialization | ASTRA §19.4 corrected | WEAKER | P00 §0.7, §2.3; P05 §36.5; P07 §25.22 carry the withdrawn attribution. (d)7 |
| Δ15 | Mixed-frame wrench in `fs-mbd` | NEON F-03 new | MISSING | No mention in the parts; confirmed in source. Block 13 |
| Δ16 | Metal has no 64-bit integer atomic add or compare-and-swap | NEON §17.3 new | WEAKER | Design already avoids them (P05 §21.6 rule 7, §35.7); P05 §21.16 says "unmeasured"; device counts have no overflow rule. Block 14 |
| Δ17 | Exact `serde_json` pins conflict across siblings; feature unification | NEON F-07, F-08 new (uncommitted) | MISSING | P12 L-5 allows "exact pins" without naming the conflict; the cloth oracle is planned as a Cargo dependency. Blocks 12, 13 |
| Δ18 | Corrected known answers (thermal card, plate sinkage, strata under the observation subgroup, all-real root fixtures) | NEON KA-08, -09, -13, GB-11 | WEAKER | The parts have their own re-derived cards (P02 §15.7, P03 §18.10), which stand. The corrected rows are now reproducible and are offered as additional rows. Block 17 |
| Δ19 | Agent acceptance scored on diagnosis, unnecessary simulations and honouring inconclusive verdicts | ASTRA digest §14 (W27 card) | WEAKER | GATE-TETHER-01, -13 and GATE-AUTOPSY-08 score completion and repair; none scores restraint. Block 18 |

Counts over the 63 digest checklist rows (A.1–A.3): COVERED 55, WEAKER 4, MISSING 1, DEFERRED 3.
Counts over the 19 post-digest rows (A.4): COVERED 1, WEAKER 7, MISSING 11.

---

## (b) Changes in the third plan since its digest

Two layers: the supplied diff (commit `d18b5a1`) and a later uncommitted edit of the working tree. "Take" means the
hybrid should adopt it; the reason follows.

| # | Change | Layer | Take? | Reason |
|---|---|---|---|---|
| 1 | Metal Shading Language 3.2 and 4.0 have no 64-bit integer atomic add or compare-and-swap (live probe). Metal scatter is sort plus segmented reduction or a per-body fixed-order gather; CUDA may use sort or fixed-point integer atomics. Risk row moved to "confirmed" | diff | Take the fact | The parts already design around it; the fact turns "unmeasured" into an expected fingerprint value and exposes a missing overflow rule for 32-bit counts. Block 14 |
| 2 | Withdrawn: 64-bit atomic accumulation on Metal (killed hypothesis K-13) | diff | Nothing to withdraw | The parts never assumed it (P05 §21.6 rule 7) |
| 3 | Device facts: working-set limit 222.72 GiB; thread-group memory 32 KiB; the `metal` 0.33.0 binding stops at language version 3.1; an 85% working-set cap with eviction when a large job starts | diff | Take the binding fact | Confirmed in the registry source (`library.rs:363`). The parts already carry 222.7 GiB and an admission-and-eviction policy (P04 §19.15). Block 13 row 8 |
| 4 | Known answer for plate sinkage corrected: n = 0.5, loads 45 and 75 kg, widths 8 and 12 cm, giving 3.52, 9.79, 4.96 cm | diff | Optional row | The digest objected that the pair did not follow from the law. It now does (recomputed with g = 9.81). The parts' own substitution stays. Block 17 |
| 5 | Known answer for the thermal cliff corrected: C = 140 J/K, G = 0.4545 W/K, giving 33.5 s | diff | Optional row | The digest computed 21.1 s for the old parameters. The new card gives 33.49 s if R₀ is referenced to 20 °C and the ambient is 25 °C; referencing R₀ to 25 °C gives 34.08 s. The parts' own card (P02 §15.7) checks out and stays. Block 17 |
| 6 | Clearance lattice corrected to 240 × 160 × 32 with a 74.6 mm band | diff | Already there | P04 §19.11 has exactly these figures |
| 7 | Linkage counts restated as generic complex counts, with all-real fixtures (40 real Stewart–Gough, 16 real 6R) | diff | Take the fixtures | The parts already say "generic complex instance". Saturated-real fixtures test the reality classification. Block 17 |
| 8 | Energy budget relabeled: 0.25 J external work, 0.10 J stored, a further 0.10 J reserve for 45.5 ms | diff | Already in §22.11 | P06 matches; P01 C-31 still carries the old labels, see (d)3 |
| 9 | Created-energy audit subtracts work by moving fixtures | diff | Already there | P03 §16.10, P09 §29.9 |
| 10 | Qualification split into a ledger state machine with sealed cluster tokens and a separate runner | diff | Already there | P08 §24.13; P11 §37.5; GATE-QUAL-02, -03 |
| 11 | Schedule-race check as a generic engine plus a robotics suite; "three disjoint conflicting pairs" | diff | Already there | P04 §20.4.7; P01 C-19 |
| 12 | `.ftask` grammar with payload, poses, observation and action layouts; unscaled predicates refused | diff | Already there | P08 §24.4, §24.5 are richer (labelled examples, mutation check) |
| 13 | Gait gradients: implicit-function adjoint at any smooth fixed point; envelope theorem only at converged optimal policies | diff | No | P06 §22.9.6 uses the implicit-function form with a fixed baseline controller, so the envelope argument is never needed |
| 14 | Fatigue from `fs-material::fatigue` (`RainflowMiner`), not `fs-solid::fatigue` | diff | Already in §31 | P10 §31.4 cites it correctly; P01 C-35 cites the other crate, see (d)11 |
| 15 | Layering repaired (shield owns the backup trait; scene representation device-agnostic; surface crates independent); the verifier is no longer claimed standard-library only | diff, uncommitted | No | Not the hybrid's crate map. R2 keeps `frob-cert` with an empty dependency table, which is stronger than the source now claims |
| 16 | A refusal-code registry of 20 codes | diff | No | Appendix O has 296 |
| 17 | Dependency ledger: nine `fs-qty`-free crates, 22 coupled; `fs-mbd::Wrench` mixes world force with body torque; `fs-ga::Motor` stores 16 blades | diff, uncommitted | Take the wrench fact | Confirmed at `fs-mbd/src/lib.rs:1695–1701`. The rest is superseded by the parts' own reading (P12 §33.3, §33.9, §33.12). Block 13 row 5 |
| 18 | Killed-hypothesis register unified as K-01 to K-13. New rows: runtime operad wrappers around the step; full contact-mode stratification and a global lifted-linear predictive controller; unbounded whole-robot elimination; time-parallel rollouts and an asynchronous interaction calendar; Metal 64-bit atomics | diff | Take four rows into Appendix C | Three are absent from the parts and are plausible things an agent would re-propose. Ready rows follow this table |
| 19 | The verification script is now in the repository (`planning/verify_plan_math.py`, 641 lines) | diff | Note only | Answers the digest's objection that scripts were absent. The rule "no number as a result" is unchanged; the rows taken here were recomputed independently |
| 20 | Strata counts under the observation subgroup: 45 strata and 324 transitions, against 27 and 162 | diff | Take as a known answer | A hand-checkable instance of "the mechanical group used where the observation subgroup is required". Block 17 |
| 21 | Tool list enumerated (42, then 46); three flags on running responses; a budget refusal | diff, uncommitted | No | The verb registry (169 verbs) and R7 already cover it |
| 22 | asupersync 0.5.0 behind frankentui's executor feature and in `f3d-runtime` | diff | Already there | P12 §33.13 item 3; P11 §32.14, §32.15 |
| 23 | `serde_json` exact pins: `=1.0.151` in franken_textiles and franken_threed, `=1.0.154` in fastmcp_rust, `1.0.154` or later in frankenscipy | uncommitted | Take | Confirmed in each manifest. It breaks the planned cloth oracle as a Cargo dependency, and it points at a larger hazard the source did not draw out: feature unification changes map order and number parsing in every crate of a build. Blocks 12, 13 |
| 24 | Where `rayon` enters (fnp-linalg, fnx-algorithms, fsci-linalg and -sparse) | uncommitted | No | P12 L-5 bans a direct pool and notes transitive presence |
| 25 | The `metal` 0.33.0 binding already exposes blit encoders, binary archives, thread-group memory and indirect dispatch | uncommitted | Take as a refinement of the ask | Two of the four confirmed in source (`encoder.rs:1715`, `:1730`). FD-FT-01 becomes a request to wrap existing bindings. The source still schedules the sibling change as its own bead, which stays on the do-not-inherit list |
| 26 | Layout arithmetic: 112-byte port record, 1,872-byte tier stride, 128 bytes per tube cell, 1.22 GB per fluid environment, world-bank headroom per card | uncommitted | No | Unmeasured assertions about a design the hybrid does not use (do-not-inherit item 16) |
| 27 | Preflight gate split by milestone; several sibling admissions moved later | diff | No | Milestones are fixed by the Phase 2 brief |

**Ready rows for the killed-idea register (Appendix C), from change 18.** Plain names, with the refuting argument.

| Killed idea | Refutation | What replaces it |
|---|---|---|
| Wrapping the running step in a composition layer evaluated at run time (module algebra interpreted per step) | Composition is a compile-time fact about ports; at run time it adds indirection to every read of a write-dense state and cannot express contact coupled through the base | Modules compose at compile time into flat tables (§14.6, C-12) |
| A safety certificate per contact stratum of the full robot, or one lifted-linear predictive model for the whole hybrid system | Strata grow as 3ᵏ in the number of contacts k; a lifted linear model has no representation of an impact jump, and its parameter count grows quadratically in the lifted dimension | The energy budget, reduced-order filters with a measured margin, and small reachable-set tables (§22.11, §22.12) |
| Time-parallel rollouts (coarse and fine propagators), and an asynchronous event calendar as the core stepping model | Contact breaks the agreement between coarse and fine propagators that time-parallel schemes need; policy and sensor ticks at 50–100 Hz bound the lookahead of any calendar in a cluttered interior | The lockstep batched step and the transactional referee (§16.5, §16.6) |
| Accumulating 64-bit sums with device atomics on every backend | Reported absent on Metal; one backend without them forbids them in a mixed run | 32-bit counts with a proven no-wrap bound and host drains; two-limb fixed-order reduction (block 14) |

### (b2) Changes in the second plan since its digest (commit `5afef55`)

The brief expected none. There are fourteen material ones; each is a row Δ1–Δ14 of table A.4. The three that matter
most: a fifth foundation defect (the action sampler's joint law, confirmed here from the constants in
`fs-g1-train/src/ppo.rs:181–189`); two statistical corrections to the ten-fold claim (the rate estimand differs from
the restricted-time ratio, and a percentile bootstrap certifies a 1.82-fold arm as 20-fold in 79% of 24-family
suites); and the withdrawal of the "94% is shared checking and serialization" attribution, which the parts repeat in
four places. Smaller changes: the bundle manifest is now 160 files with 25 deliberate regressions (was 154 and 20);
the worked shell session rejects identifiers that a shell cannot round-trip.

---

## (c) Insertion blocks

Eighteen blocks. Gates are numbered 901–919 in one sequence across this file. Refusals are named by description.

<<<INSERT chapter=23 title="The joint law of sampled actions">>>
**What it is.** [S] A rule for the action sampler, added to the PPO battery of §23.6 as P-12: the joint law of the
sampled action equals the law that the likelihood computation assumes, across the channels of one decision, across
consecutive decisions of one world, and across neighbouring worlds. Marginal checks do not establish it.

**Why it is a rule.** FACT (`frankensim/crates/fs-g1-train/src/ppo.rs:172–189` at `483d09c682f2`; to be re-run at
admission, §33.11): `gaussian_action` draws each channel from `gaussian_sample`, a Box–Muller cosine over a counter
advanced by the constant 0x9E3779B97F4A7C15, with the phase uniform taken from the top 53 bits of the counter times
0x94D049BB133111EB. The phase is therefore a linear function of the counter modulo 1, and consecutive draws differ in
phase by the fixed fraction 0.167245 of a turn, an angle of 0.334490·π. For a uniform angle θ, cos θ and cos(θ + φ)
have the same sign with probability 1 − φ/π, so two adjacent channels agree in sign with probability 0.665510 where
independence gives 0.5. Reported counts in 100,000 two-channel calls per seed are 66,552, 66,552 and 66,551, while
each channel alone is positive half the time. The trainer scores these samples with a product of marginal densities,
so its likelihood ratio is computed for a random variable that was not the one sampled, which is the failure rule
P-3 exists to prevent.

**Mechanism.** DESIGN:
- Every noise draw is a Philox output keyed by `(WorldAddr, decision index, stream id, channel)` (§35.9). A
  Box–Muller pair consumes two uniforms from distinct counters, and its second output is used or discarded by a
  declared rule. No draw is a function of a neighbouring draw's counter.
- The sampler has a manifest: the declared joint law (a product Gaussian, or a full covariance when the policy has
  one), the transform and bounds of P-3, and the stream layout.
- `frob-conformance` qualifies a sampler against its manifest with four statistics over N decisions: the
  sign-agreement frequency of every pair of channels; the same for one channel at consecutive decisions; the same
  for one channel in adjacent worlds; and the empirical covariance of the standardized noise. Under a product law
  every agreement frequency has mean ½ and standard deviation ½/√N.
- `train.start` refuses a sampler that has no qualification receipt for the pinned kernel (refusal: action sampler
  not qualified against its likelihood).

Owner: `frob-learn`; kernels in `frob-kernels`; fixtures in `frob-conformance`.

**Known answer and negative control.** The recurrence above, re-implemented as a planted sampler, must be rejected:
its adjacent-channel agreement is 0.6655, more than 100 standard deviations from ½ at N = 10⁵. The qualified sampler
must show every agreement inside ½ ± 0.005 at N = 10⁵ (TARGET: 3.2 standard deviations; the number of pairs tested
is recorded, so the family-wise rate is stated).

| | |
|---|---|
| GATE-CORTEX-901 | Known answer. For action dimensions 2, 12 and 29, N = 10⁵ decisions and 64 worlds: every channel-pair, consecutive-decision and adjacent-world sign agreement lies in ½ ± 0.005; every off-diagonal entry of the standardized covariance is below 4/√N in magnitude; the planted linear-phase sampler is rejected with an agreement of 0.6655 ± 0.005. M2. |
<<<END>>>

<<<INSERT chapter=23 title="The actor's information contract covers every field it can read">>>
**What it is.** [S] Rule E-10, extending E-8 and E-9 of §23.3. The actor's information contract lists every value an
online actor can read at a decision instant, with its delivery instant under the clock contract of §20: observation
channels, earlier rewards, termination and truncation flags, action masks, and each field of `info`. Outputs of the
trainer and of the scorer are outside the actor's view unless the mission declares them delivered.

**Why.** A reward is a function of state, and delivered to the actor it is a sensor. Smallest case, on the docking
model of §20.16 (three one-second ticks, hidden gain g ∈ {1, 2}, acceleration g·u): with first command u₀ = ¼ the
position after one tick is x₁ = g/8, so a shaped reward −|1 − x₁| is −7/8 under g = 1 and −3/4 under g = 2. When the
velocity packet arrives late, no causal policy docks in both worlds (§25.15). If the wrapper hands that reward to
the actor before the second command, the two worlds are distinguished and the commands (¼, ½, −¾) and (¼, 0, −¼)
dock both. An exclusion computed without reward visibility in its contract is then false for the environment that
was run.

**Mechanism.** DESIGN:
- `ActorView` is a typed projection of `StepView` that contains only fields of access class `actor`. `reward`,
  `terminated`, `truncated`, masks and every `info` field carry an access class and a delivery instant. The default
  class of `reward` and of every `info` field is `trainer-only`. A recurrent policy that takes the previous reward
  as an input requires the task to declare reward as an `actor` channel.
- The digest of the contract (channels, classes, instants) is part of the `TaskId`, of every memo key and of every
  certificate's assumption set (C-18), so a cached answer is not reused under a different contract.
- `sense.causal-test` (§20.15) gains four planted leaks that must each fail: the previous reward wired into the
  actor; an `info` field that carries a hidden parameter; an action mask computed from hidden state; a termination
  flag delivered before its instant.
- The Gymnasium-compatible adapter (§23.18) returns rewards to the learner, as that interface requires, and records
  whether the driven policy consumed them. A run whose policy read an undeclared field is labeled and cannot be
  cited for a mission answer (refusal: actor read a field outside its information contract).

Owner: `frob-learn` (types), `frob-sense` (test), `frob-spec` (declaration).

**Known answer.** The late-packet docking exclusion, run through `VecEnv` and through the adapter. With the reward
hidden the verdict is excluded. With the reward declared and delivered before the second command the verdict
changes to feasible, with the witness above. Both runs are required: a harness that reports the same verdict in
both is defective.

| | |
|---|---|
| GATE-CORTEX-902 | Known answer. Each of the four planted leaks fails `sense.causal-test` in 100 of 100 seeds; the docking exclusion holds with reward hidden and flips with reward delivered, on `VecEnv` and on the adapter; a contract change alters the `TaskId` and invalidates the cached exclusion. M2; adapter rows at its release. |
<<<END>>>

<<<INSERT chapter=23 title="A threshold crossing is not attainment">>>
**What it is.** [S] A rule for every figure of the form "time to a target", which includes the training-time parity
rows of §36.3 and GATE-CORTEX-05. A recorded score crossing is a descriptive event. Attainment is a claim about the
policy's population performance, and it needs error control over every evaluation look that could have produced the
crossing.

**Counterexample.** A policy with true success rate 0.89 is evaluated against a target of 0.90 with 30 episodes per
look. A look with 30 successes of 30 has an exact one-sided 95% lower bound of 0.05^(1/30) = 0.90497, above the
target. One look does this with probability 0.89³⁰ = 0.0303. Across 50 independent looks at the unchanged policy the
probability of at least one such crossing is 1 − (1 − 0.0303)⁵⁰ = 0.78548. A rule that stops the clock at the first
crossing reports attainment, about four times in five, for a policy that never attains.

**Mechanism.** DESIGN:
- Evaluation during training runs on a fixed development stream, disjoint from training worlds, at a cadence
  registered before the run. The run record holds every look, not only the best.
- `train.status` reports two fields. `crossing` is the first look whose point score met the target, and is
  descriptive. `attained` is the first look at which a procedure valid for the registered cadence rejects "at or
  below target". The default procedure allocates error across looks as αₖ = α/[k·(k + 1)], the series of §24.13,
  and applies an exact one-sided binomial bound at level αₖ for success targets, or a confidence sequence on bounded
  returns (C-15) for return targets. With all-success looks against a 0.90 target at α = 0.05, look k needs at
  least ln(1/αₖ)/ln(1/0.9) episodes: 36 at k = 1, 46 at k = 2, 74 at k = 10, 103 at k = 50.
- Alternatively the checkpoint chosen on the development stream is evaluated once on a final holdout. That
  qualifies the chosen checkpoint and leaves its first attainment time unknown; the reported time is then an upper
  bound and is labeled as one.
- Non-attainment within the training budget is right-censored at the budget and summarized by a restricted mean.
- In a parity comparison both arms are scored by the same rule on the same stream sizes. An incumbent's published
  time to threshold is context; the figure re-measured under this rule is the comparator.

`eval.preanalyze` prints, for a registered cadence, episode count and target, the probability that an unchanged
policy at stated true rates produces a crossing. Owner: `frob-learn`, with the study machinery of `frob-arena`.

| | |
|---|---|
| GATE-CORTEX-903 | Known answer. In 2,000 simulated trainings of an unchanged policy at true success 0.89, with 50 looks of 30 episodes, the first-crossing rule reports attainment in 78.5% ± 3 points and the spending rule in at most 5% (exact one-sided test at the 1% level); the four episode counts above are reproduced exactly. M2. |
<<<END>>>

<<<INSERT chapter=39 title="Estimating the ratio, and the smallest suite that can certify">>>
**What it is.** [S] The inference rules for the ratio of §2.1, fixed before the sealed suite is opened: what is
estimated, which procedure is invalid, and how many independent mission families a population claim needs.

**Two estimands, never interchanged.** The restricted-time ratio is
R_time = Σᵢ min(T_rival,i, H) ÷ Σᵢ min(T_frob,i, H). The accepted-output rate ratio is
R_rate = (accepted_frob ÷ makespan_frob) ÷ (accepted_rival ÷ makespan_rival). Known answer: 25 serial missions with
H = 100 s; the rival takes 50 s on each (1,250 s, 25 accepted); the candidate finishes 24 in 1 s each and times out
once (124 s, 24 accepted). Then R_time = 1,250/124 = 10.081 and R_rate = (24/124)/(25/1,250) = 9.677. The word
ten-fold attaches to R_time only; a statement about decisions per hour cites R_rate. With parallel jobs the makespan
is the measured campaign makespan under matched resource caps. Each arm's independent acceptance is timestamped
before the joint reveal.

**The unit of inference is the independent mission family.** Missions that share a family, an originating site or a
material batch form one cluster; 24 rows from one site are one site replicate. Connected groups stay together
across the development and sealed splits. The resampling unit, H and the cluster structure are registered with the
suite.

**A procedure that is invalid here.** Let H = 1,000 s, the rival always take 20 s, and the candidate take 1 s on 99%
of families and 999 s on 1%. Both always complete. The population ratio is 20/10.98 = 1.8215. A suite of 24 families
contains only fast missions with probability 0.99²⁴ = 0.78568, and then every percentile-bootstrap resample reports
20. Resampling cannot show a tail that the sample does not contain.

**The test.** For the claim R_time ≥ 10, form the bounded paired contrast
Xᵢ = min(T_rival,i, H) − 10·min(T_frob,i, H), which lies in [−10·H, H]; the claim is E[X] ≥ 0. For n independent
families a distribution-free one-sided lower bound is X̄ − 11·H·√(ln(1/α)/(2n)). The claim is made only when a
registered valid bound is nonnegative.

**How many families a population claim needs.** DESIGN (derived; hand-checkable). If a fraction p of families has
X = −10·H and the rest have X = H, then E[X] = H·(1 − 11p), which is negative for p > 1/11, and a suite of n
families shows no slow family with probability (1 − p)ⁿ. A procedure with one-sided error α that reports a
nonnegative bound on the best possible sample must therefore have (10/11)ⁿ ≤ α, so n ≥ ln(1/α)/ln(1.1): 32 families
at α = 0.05, 46 at α = 0.0125 (four strata under a Bonferroni split), 49 at α = 0.01. The bound of the previous
paragraph needs n ≥ 121·ln(1/α)/2, which is 182 at α = 0.05, even on that best sample. With 24 families, zero slow
missions is compatible at the 5% level with a slow-family rate of 1 − 0.05^(1/24) = 11.7%, and 11 × 0.117 exceeds 1.

**Consequence.** Each stratum declares one of two result types before sealing. A *population result* has at least
the family count that its registered bound requires, and never fewer than ln(1/α)/ln(1.1). A *finite-suite result*
states the ratio on the sealed missions as an exact fact about that suite, with its completion table and its phase
ledger, and carries no population wording. Completion and useful coverage are tested as paired differences against
non-inferiority margins frozen with the suite. A rare binary failure is reported with an interval valid at zero
counts (0 of n gives the upper bound 1 − α^(1/n)); a bootstrap of n zeros has zero width and is not used.

`eval.preanalyze` accepts a ten-fold study plan (n, cluster structure, H, α, result type) and prints the reachable
verdicts. It refuses a population result whose n is below the necessary count (refusal: suite too small for a
population claim). Owner: `frob-arena`; the harness is in `frob-conformance`.

| | |
|---|---|
| GATE-TENFOLD-904 | Known answer. The pre-analysis reproduces 10.081, 9.677, 1.8215, 0.78568 and the counts 32, 46, 49 and 182. In 2,000 simulated campaigns of 24 families with the rare-tail candidate, the contrast test certifies ten-fold in none, and the percentile bootstrap, run as the negative control, certifies it in 78.6% ± 3 points. M3, before the partial gate. |
<<<END>>>

<<<INSERT chapter=39 title="Rival arms, protocol controls and the sealed suite">>>
**What it is.** [S] The parts of the protocol that keep the comparison fair to the rival and hard on the candidate.

**Rival classes, and what must be earned against each.** FACT (Appendix H; each named system and its revision is
re-read and pinned at the rival audit below).

| Problem | Strongest rival to compose | What the candidate must earn |
|---|---|---|
| GPU learning with sensing, control and transfer | The leading batched engines of §1.1 with their training stacks and system-identification workflows | More accepted designs per unit time, counting setup, training and independent evidence |
| Industrial and rich-world simulation | The industrial stacks and the batched manipulation engines of §1.1 | A checked family answer across several contact or sensing regimes at competitive total cost, at fixed scene diversity and sensor resolution |
| Joint synthesis of body and control | Grammar-based morphology search, differentiable co-design and learned design generators (RoboGrammar, DiffHand, N-LIMB, DiffuseBot) | A cheaper design under the same sealed constraints, or stricter checkable coverage at matched cost; the rival receives the same grammar, masses, symmetry knowledge and controller budget |
| Certified geometry, adversarial scenes, robust design | Certified configuration-space regions; a scenario language (Scenic); linear and mixed-integer programs; interval, reachability and satisfiability tools | More of the mission decided with correct quantifiers and interpretable counterexamples |
| Digital twins and systems engineering | A multibody systems-engineering tool with a skilled author; an agent-built twin | Less engineering time from a changed mission to a reconciled portable answer |

DESIGN: the most serious rival may be a short analytic program wrapped around a mature simulator. It is always an arm.

**The rival audit.** Before any sealed mission is opened, each rival arm is frozen. Its configuration, exact
revisions, licence, wrappers, tolerances and optimizer scripts are committed to the experiment pack, after a
competent baseline engineer has improved the fastest rival on the development fixtures. Each arm receives the same
calibration data, hardware envelope, reusable prior answers, cache budget and access to the checker.

**Controls.** Missions are paired across arms with isolated transcripts. Order, operator, site and resource
assignment are randomized and counterbalanced. Agent arms start from fresh contexts on the same model revision, with
the same caps on model calls and parallel workers; human operators are matched for proficiency. An adjudicator who
does not know which system proposed an answer releases paired receipts after both arms close. Where a platform
lacks a native path, a best-supported trial and a matched-device trial both run, and a missing cell is published as
`unavailable`. The fastest qualifying rival, all arms and the raw pairings are published; a per-case best-of-rivals
ensemble is reported only as a labeled extra lower bound on the ratio.

**A second claim with its own acceptance: a new engineering operation.** A fresh team receives a typed continuous
design family, realistic uncertainty, a causal controller contract and sealed changes. Acceptance requires (i) a
machine-checkable inner or outer answer with explicit scope, (ii) a physically interpretable witness or a
scope-preserving ambiguity report, and (iii) correct reuse or withdrawal after an amendment. Each mission
predeclares its mandatory interior, exterior and boundary obligations, a largest unresolved band or optimality gap,
and any mandatory distinguishing experiment. Strict and closed endpoints are part of acceptance, and a vacuous
enclosure fails. The rival's answer goes through the same checker. A tie retires the claim of exclusive expressivity
on that fixture, and can still justify shipping the simpler implementation behind the same interface.

**The sealed suite.** Six mission types, each with what the verifier changes privately and the strongest control.

| Sealed mission | Private changes | Strongest control |
|---|---|---|
| Adjustable stance under static load (§25.13) | Foot authority, a force floor, package dimensions, asymmetric payload, a missing foot, passive-spring compatibility | A lifted robust linear program; conventional projection; a force-law switch |
| Split-grip landing and the last usable observation (§24.11, §25.15) | Mirror the strip; hide or delay the surface cue; shift the payload; change the contact order | Belief-state reachability; equal-budget reinforcement learning; geometric control |
| Gripping unfamiliar compliant stock (§25.14) | Geometry; an early-law alias; contact coupling | An expert probe; direct measurement at mission dwell; active system identification |
| Changing an accepted carrier or walker (§28.13) | Replace a gearbox, a wheel or a lower limb; perturb latency, friction or energy reserve | Retraining; a shared conditioned actor; warm-started control; ordinary graph caching |
| Recommissioning a facility (§25.17) | Move a door or a counter; perturb scan uncertainty; reduce a shared resource; alter a shift | Recomputation; incremental mixed-integer programming; standard motion planning; selective replay with cached bounds |
| A heterogeneous high-throughput family (§17) | Mix fixed and changed topology; grazing contacts; dense depth sensors; pressure to spill to the host | An optimized batched incumbent; this engine with family sharing switched off |

Sealed cases include routine easy wins for conventional methods and answers that are genuinely unresolved. Some
worlds come from an evaluator independent of the proposing engine, because a rollout and a solver that share one
mistranslated model agree on the wrong answer. Independently derived mutations of units, contact, causal clocks and
passive load are planted. Warm campaigns use predeclared amendment sequences; one-time authoring is amortized over
1, 10 and 100 amendments; failed amendments stay in the sequence; the consequential edit is concealed; missed
withdrawals and unnecessary invalidations are both counted.

| | |
|---|---|
| GATE-TENFOLD-905 | Known answer. The rival audit is complete before sealing: every arm's frozen configuration reproduces its own development-fixture figure within 10% (TARGET) on the tournament hardware; the baseline engineer's sign-off and the digest of the committed pack are recorded; no arm's configuration changes afterwards, and a changed digest voids the arm. M3 for the partial gate; M7. |
| GATE-TENFOLD-906 | Known answer. Suite hygiene: every stratum holds at least one routine easy win and one unresolved answer; the checker rejects 100% of the planted unit, contact, clock and passive-load mutations; in the amendment sequences an arm keeps its scoped claim only with zero missed withdrawals, and unnecessary invalidations are reported per arm. M7. |
<<<END>>>

<<<INSERT chapter=25 title="Chance claims transport only through a coupling">>>
**What it is.** [F] A restriction on `mission.transport` (§25.11), and the proof object that lifts it. The
refinement relation of §25.11 matches every possible successor of the target to an allowed successor of the source.
That preserves universal claims. It preserves only the support of a probability law, so it carries no chance claim
and no expected value.

**Counterexample.** Source and target each have one action and two outcomes, success and failure. The source
succeeds with probability 0.99 and the target with probability 0.01. Every target successor has a matching source
successor, so the relation holds, and the claim "succeeds with probability at least 0.95" is true of the source and
false of the target. Every coupling of the two laws disagrees on at least 0.98 of its mass, which is their
total-variation distance.

**Mechanism.** DESIGN:
- A transport certificate declares the kind of each transported claim: `universal`, `chance` or `expectation`.
- For `chance` and `expectation` it carries a coupling budget: for each step t of the horizon, a bound εₜ on the
  total-variation distance between the source and target transition-and-observation kernels at related states,
  under the command that the interface selects. The transported bounds are
  P_target(accept) ≥ P_source(accept) − Σₜ εₜ and, for a quantity in [0, B], |E_target − E_source| ≤ B·Σₜ εₜ. The
  coupling is causal: it uses no disturbance that the interface cannot have observed.
- Sources of εₜ, in order of strength. Exact computation for finite kernels (class Certified). A common-noise
  construction in which both systems are driven by one disturbance stream compiled at physical coordinates (C-40)
  and the relation is shown to persist, with its failure rate bounded from sealed paired runs (class Tested: a
  one-sided upper confidence bound). Otherwise none, and the claim is measured on the target by `eval.qualify`.
- A certificate that marks a chance claim `universal`, or omits the budget, is rejected by `frob-verify`
  (refusal: chance claim transported without a coupling budget).

Owner: `frob-mission` (proposer); `frob-lemma` and `frob-cert` (bundle and check).

**Known answers.** The two-outcome case: Σ ε = 0.98, the transported bound is 0.01, and the claim is refused. A
finite chain of 10 steps with ε = 0.003 per step: a source probability of 0.99 transports to at least 0.96, and the
exact target value computed by enumeration lies above it.

**Pilot, ceiling, kill.** Pilot: 200 random finite Markov fixtures with known kernels, then the tray carried by a
wheeled base and by a quadruped. Falsifier: one transported bound that the exact or sealed target value violates.
Ceiling: the budget helps only when Σ ε is small against the slack of the claim; it saves one qualification study
per transported skill and no simulation time.

| | |
|---|---|
| GATE-AUGUR-907 | Known answer. Zero violated bounds over the 200 finite fixtures; the two-outcome reversal is refused; on the tray fixture the 95% upper bound of the measured budget is below the claim's slack for at least one skill, or the kill gate fires. Finite part M4; tray M8. |
| ✖ KILL-COUPLING | Trigger: one violated transported bound that survives a repair cycle, or no skill on the tray fixture with a budget below its slack. Fallback: `mission.transport` carries universal claims only, and every chance or expected-value claim is re-measured on the target in the qualification lane of §24.13. |
<<<END>>>

<<<INSERT chapter=25 title="No monotone fill without a proof of monotonicity">>>
**What it is.** [S] A restriction on proof rule 3 (§25.4) and on every sweep that feeds a mission answer. A verdict
at an unevaluated parameter value is never inferred from its neighbours unless a checked monotonicity argument for
that parameter, model and verdict is attached. An observed pattern is not such an argument.

**Counterexample.** FACT (planning archive, the 300-trajectory planar friction sweep of the sill fixture; Appendix H;
to be re-run before a gate cites it): for the 75 mm wheel, a sharp lip and the payload shifted 40 mm backward, the
planar scorer returns success, timeout, success at μ = 0.500, 0.525 and 0.550. Filling every friction level above
the first success adds one success that did not occur. With the scoring period held at 1 ms, halving the physics
step from 50 µs to 25 µs changes the triplet to timeout, timeout, success; at 25 µs with a stiffer contact and
adjusted damping all three succeed. The physical reading of the middle timeout is unresolved, and the fill is
refuted without resolving it.

**Mechanism.** DESIGN:
- `mission.synthesize`, `nemesis.search` and `phylum.sweep` return `unresolved` for every parameter value that was
  neither evaluated nor covered by a proof. No verb interpolates verdicts.
- A fill is a proof step of the monotone-inheritance kind. It cites the theorem that makes the verdict monotone in
  the parameter under the declared model. The creep-limit rows of §24.10 qualify, because the friction demand
  μ ≥ tan θ is monotone in μ by construction. A dynamic rollout verdict has no such theorem by default.
- A detector runs on every swept scalar: along the sorted grid it counts the sign changes of the verdict. More than
  one change is recorded as a non-monotone outcome, with the grid, the step and the contact parameters, and is
  routed to the fidelity ledger (§16.8) for step-halving and stiffness variants. Results that depend on the affected
  interval are typed `model-conditional` until the variants agree.
- When the physics step is refined the requirement clock stays fixed (§24.5), so two refinements are compared on
  one definition of the verdict.

Owner: `frob-mission` (answers); `frob-nemesis` and `frob-phylum` (sweeps); `frob-analyze` (detector).

**Known answer.** A planted fixture with a provable band: a cart on a compliant sill mount passes below and above a
resonance band of mount stiffness and fails inside it, so the grid shows pass, fail, pass. The licensed fill on the
creep rig reproduces its exhaustive grid exactly. The unlicensed fill on the banded fixture is refused.

| | |
|---|---|
| GATE-AUGUR-908 | Known answer. On the banded fixture the detector flags the non-monotone outcome in 100 of 100 seeds, no verdict is filled, and the mission answer leaves the band `unresolved`; on the creep rig the licensed fill equals the exhaustive grid at every point; a mutant that fills above the first success is caught on both. M3. |
<<<END>>>

<<<INSERT chapter=24 title="Traction capacity is a slice of the wrench set at the commanded wrench">>>
**What it is.** [S] A metric, and two analytic rows for Fixture 2 (§24.11), used in every comparison of soles, tyres
and pads. The scalar friction capacity Σ μᵢ·Nᵢ is the largest longitudinal force with the yaw moment left free. A
robot commands a wrench. The capacity that ranks two designs is the largest force along the commanded direction
among contact forces that also deliver the commanded moments.

**Counterexample.** A two-point sole carries 50 N on each point, at lateral offsets ±20 mm, one point on μ = 0.25
and the other on μ = 0.8. The largest forward force is 12.5 + 40 = 52.5 N, and it comes with a yaw moment of
(40 − 12.5) N × 0.020 m = 0.55 N·m. If the command is a pure forward force, the two longitudinal forces must be
equal, and the largest is 2 × 12.5 = 25 N. A sole on uniform μ = 0.4 supplies 40 N at zero yaw. By scalar capacity
the split sole wins, 52.5 N against 40 N. At the commanded wrench it loses, 25 N against 40 N.

**Mechanism.** DESIGN:
- `traction_at_wrench(direction, moment command)` is a cone program over the contact wrench set of C-06: maximize
  the force along the direction, subject to the per-point friction cones, the measured normal loads and the moment
  equalities. It is the program of §22.5 with another objective, so it needs no new solver. Owner: `frob-control`
  for the program, `frob-arena` for the metric, `frob-analyze` for the static margin of A-08.
- Scorecards (§24.8) report the scalar capacity under the name `capacity, moment free`, and the slice under the name
  of the commanded wrench. A comparison of contact geometries cites the slice.
- Fixture 2 gains two rows. The plate-drag rig runs once guided against yaw, as now, and once free to yaw, which
  enforces zero moment physically. For the uniform-pressure strip model with the boundary through the centre and
  longitudinal traction only, the zero-yaw capacity per unit normal load is (μ_L + √(μ_L·μ_H))/2, which is 0.3486
  for μ_L = 0.25 and μ_H = 0.8, against 0.525 with the moment free, and it does not depend on the sole width. The
  closed form is a lower bound when lateral traction may also carry moment; the cone program gives the exact value.

**Known answers.** Two-point rig: 52.5 N and 0.55 N·m guided; 25 N free; 40 N both ways on uniform μ = 0.4. Strip
model at zero offset: 0.525·N guided for both widths; at least 0.3486·N free for both widths.

| | |
|---|---|
| GATE-ARENA-909 | Known answer. The cone program reproduces 52.5 N, 0.55 N·m, 25 N and 40 N to 10⁻⁹ in double precision; the two-point rig slips within 2% of each value on the referee and inside the fidelity envelope on the batch tiers; a scorecard that ranks the two soles by moment-free capacity when the task commands zero yaw is rejected by `task.check`. M2. |
<<<END>>>

<<<INSERT chapter=29 title="The solver's kinetic-energy correction is not heat">>>
**What it is.** [S] A separate ledger line for the energy that an impulse-level contact step exchanges with the
velocity predictor, and a rule that only constitutive dissipation feeds a thermal or a wear state.

**Counterexample.** A 1 kg block rests on a frictionless stationary support under g = 8 m/s². A step that applies
forces, then contact, then drift gives the block a free velocity g·h and removes it with the support impulse. The
kinetic energy removed per step is ½·m·(g·h)², which is ½·m·g²·h per second of simulated time: 1 J at 32 Hz and
0.5 J at 64 Hz. The block has zero displacement and the support produces no heat. The quantity is first order in h
and vanishes under refinement. Booked as contact dissipation, it warms the tyres of a parked robot at a rate set by
the step size.

**Mechanism.** DESIGN:
- Stage 9 (`K-ledger-step`) accumulates two quantities separately for each contact row and each joint-limit row.
  The *algorithmic exchange* is the energy that the constraint impulse removes from, or returns to, the
  unconstrained predictor. The *constitutive dissipation* is computed from a law with a dissipation function:
  sliding friction as the integral of f_t·v_slip at the solved slip velocity; normal damping of the compliant law as
  the integral of c·ḋ²; restitution loss ½·m_eff·(1 − e²)·v_n² at a located impact; joint damping; stiction and
  backlash rows by their own laws (§15.6).
- The ledger identity of §29.9 gains the line `algorithmic exchange` beside `numerical residual`. On a resting body
  the algorithmic work of gravity and the algorithmic removal by the support cancel exactly in every step, and the
  identity is checked with both shown.
- Invariant: a thermal state (winding, housing, tyre, film) and a wear state (§31.14) take input only from
  constitutive dissipation lines and from electrical loss. `frob-chronicle` rejects a channel binding that routes
  algorithmic exchange or numerical residual into a state (refusal: non-physical energy bound to a physical state).
- `telemetry.energy` prints the two lines separately on every tier. On the compliant tiers the algorithmic exchange
  of a resting contact is zero, because the contact force is a function of state. The line matters on the referee's
  impulse step and on the hard-contact batch law.

Owner: `frob-contact` and `frob-dyn` (the two quantities); `frob-chronicle` (ledger and invariant).

**Known answers.** Resting block: algorithmic exchange 1 J/s at 32 Hz and 0.5 J/s at 64 Hz; constitutive
dissipation 0; heat 0. The same block sliding at 1 m/s on μ = 0.5: 4 W of friction heat at both rates. A plastic
drop arriving at 2 m/s: 2 J of impact dissipation, booked once at the located event.

| | |
|---|---|
| GATE-CHRON-910 | Known answer. The three fixtures at 32, 64 and 2,000 Hz on `referee` and `strict`: constitutive dissipation of the resting block is exactly 0; sliding heat is within 0.1% of 4 W; impact loss is within 10⁻⁹ J of 2 J; a mutant that feeds algorithmic exchange into the film temperature is caught, because its temperature rise differs between 32 and 64 Hz by a factor of 2. M1 on the referee; M2 on the batch tiers. |
<<<END>>>

<<<INSERT chapter=20 title="Force sensors act on a force history: filter, clip and integrate in a declared order">>>
**What it is.** [S] A sensor-chain rule for force, torque, current and pressure readouts that saturate, and a
refusal for peak-load claims on tiers whose contacts are impulses. §20.5 averages the wrench over the exposure and
adds each impact impulse once. That is correct for a linear sensor. A transducer that overloads acts on force as a
function of time, and an impulse has no duration.

**Counterexample.** A rectangular pulse of 1 N·s lasts 1 ms or 10 ms, with peaks of 1,000 N and 100 N. Both average
50 N over a 20 ms exposure. A transducer that clips at 200 N before integration reports 200 N × 1 ms ÷ 20 ms = 10 N
for the short pulse and 50 N for the long one. Clipping after averaging reports 50 N for both.

**Mechanism.** DESIGN:
- A sensor card declares its chain as an ordered list of stages: a transfer function (bandwidth, with its state in
  every checkpoint), an overload law (a clip level, or a recovery model), exposure integration and quantization.
  The kernel `K-sense-ft` applies the stages in the declared order, at the substep at which force is resolved.
- A contact force history is available in three grades. `resolved`: the compliant laws compute force as a function
  of state at every physics step, and the chain is applied to that history. `modeled`: on an impulse-level step (the
  referee's event step, the hard-contact batch law) the interface card supplies a pulse model with provenance, a
  duration τ_c and a shape, for example a half-sine with τ_c = π·√(m_eff/k) and peak (π/2)·J/τ_c for an impulse J.
  `none`: there is no pulse model; the linear reading is still produced, its `overload` field is `unknown`, and any
  requirement atom on peak load or on sensor overload evaluates to `undetermined` (§24.5).
- `sense.profile` lists, per sensor, the chain, the grade available on each tier, and the atoms that the grade
  cannot decide (refusal: peak-load claim without a resolved or modeled force history).

Owner: `frob-sense`; pulse models live on interface cards (§18).

**Known answers.** The rectangular rows: 10 N and 50 N in the declared order; 50 N twice in the swapped order, which
is the mutation that must fail. A half-sine with J = 1 N·s: τ_c = 10 ms gives a peak of 157 N, no clipping and a
reading of 50 N; τ_c = 1 ms gives a peak of 1,571 N and a clipped reading of 9.59 N.

| | |
|---|---|
| GATE-SENS-911 | Known answer. The rectangular rows are exact; the swapped-order mutant is rejected; on the referee an impulse with grade `none` yields `overload: unknown` and an `undetermined` peak-load atom in 100 of 100 cases; on a compliant tier whose step resolves the pulse with at least 10 samples the clipped reading is within 2% of the closed form. M2. |
<<<END>>>

<<<INSERT chapter=13 title="A circle-valued joint needs periodic laws, or its turn count">>>
**What it is.** [S] A typing rule for laws that read a revolute coordinate. The stored coordinate is a unit complex
number and an integer turn count (§11.1). A law is `periodic` if it reads only the unit complex number, and `lifted`
if it reads the angle θ = atan2(s, c) + 2π·k. These are different state spaces, and using the circle where a law is
lifted loses state.

**Counterexample.** A joint has a linear torsion spring of 0.5 N·m/rad with rest angle 0. Held stationary at θ = 0
and at θ = 2π it has identical link poses and zero velocity. At 2π the spring stores ½ × 0.5 × (2π)² = π² J, which
is 9.8696 J, and exerts −π N·m. A state without the turn count cannot tell the two apart, and a replay from it
creates or loses π² J.

**Mechanism.** DESIGN:
- Every element that reads a revolute coordinate declares `periodic` or `lifted` in its card or constructor. Lifted
  elements include linear torsion springs, cable and hose wrap, multi-turn travel limits, lead screws and tether
  winding. `design.check` stage 1 refuses a lifted law on a joint whose state omits the turn count (refusal: lifted
  law on a joint without turn state).
- The turn count k is state. It is in the state layout, in every snapshot and digest, in replay and teleport, and
  in the settled-pose cache (§24.7). It changes by ±1 when the unit complex number crosses the cut c < 0, s = 0,
  which is an exact integer update; the Cayley bound ‖ω‖·h ≤ 0.2 rad (C-05) allows at most one crossing per
  substep.
- An encoder that reports the angle modulo one turn does not observe k. For a joint with a lifted law, the worlds
  k = 0 and k = 1 at rest deliver identical observations and need holding torques that differ by π N·m in the
  example, so `sense.alias` (§20.12) returns an alias witness. The remedies it lists are a multi-turn encoder, a
  homing move and a torque reading.

Owner: `frob-lie` (coordinate and counter); `frob-model` (declarations); `frob-actuate` (springs).

**Known answers.** The spring example: energy 0 and π² J; torque 0 and −π N·m. A wheel with only periodic laws
replays bit-identically from a state with k dropped, which is the control showing that the rule binds only lifted
laws.

| | |
|---|---|
| GATE-MARROW-912 | Known answer. The spring fixture is exact to 10⁻¹² in double precision on the referee and within 10⁻⁵ relative on device tiers; snapshot, restore and teleport preserve k on 10⁴ random states; the mutant that omits k from the snapshot is caught by an energy jump of π² J; `sense.alias` returns the witness for a single-turn encoder and none for a multi-turn encoder. M1. |
<<<END>>>

<<<INSERT chapter=34 title="The canonical reader and writer, and JSON features that unify across the build">>>
**What it is.** [S] The mechanism behind the rule "a duplicate key refuses" (§14.4), for every artifact format of
§34.9, and a guard against a dependency feature changing canonical bytes.

**The reader.** DESIGN: admission validates the raw bytes before any map is built or any digest is computed.

1. Member names are compared after escape decoding, at every nesting level, and a repeated name is refused.
   Smallest case: `{"wheel_radius_mm": 60, "wheel_radius_mm": 75}` names one member twice. A reader that keeps
   the first value simulates a 60 mm wheel, one that keeps the last simulates 75 mm, and both hash the same bytes.
2. An identifier in an envelope is a nonempty string of printable characters: no code point below 32, and not 127.
   An identifier therefore survives a round trip through a shell variable.
3. Integers above 2⁵³ and exact decimals are strings (§32.8, §14.4), so no value depends on a float parser.
4. The refusal is issued by the CLI and by the MCP server before a job is planned or a budget is charged
   (refusals: duplicate member name after escape decoding; identifier not printable).

**The writer.** Canonical bytes are produced by this project's writer from the typed term, with keys sorted by code
point explicitly. No canonical byte is produced by serializing a library map or a library number.

**Why the writer may not lean on the library.** FACT, read 2026-11-21:

- The `preserve_order` feature of `serde_json` switches its maps from sorted to insertion order, and Cargo unifies
  features across a build graph. `frankensqlite/Cargo.toml` records that the feature "leaked into every downstream
  consumer via feature unification, silently switching serde_json maps to insertion order across their whole
  build", and removed it from its workspace for that reason.
- `frankenscipy/Cargo.toml:59` enables `preserve_order` and `float_roundtrip`. FrankenSim's bedrock depends on
  FrankenSciPy (`fs-la` on `fsci-linalg`, `fs-math` on `fsci-special`), and `frankensim/Cargo.lock` resolves
  `serde_json` 1.0.154 with `indexmap` among its dependencies, which only `preserve_order` adds.
- `fastmcp_rust/Cargo.toml:54` enables `arbitrary_precision`, which changes how numbers are represented.
- `franken_textiles/Cargo.toml` records that without `float_roundtrip` a decimal may be read one unit in the last
  place off, which "flipped point-in-polygon verdicts" in its fixtures.

The feature set of the JSON library in a `frob` binary is therefore decided by whichever sibling is linked, and not
by this workspace. Smallest case: `{"b": 1, "a": 2}` re-serialized through a library map is `{"a":2,"b":1}` in one
build and `{"b":1,"a":2}` in another, and a digest over those bytes changes when an unrelated crate enters the graph.

**The guard.** `xtask check-deps` prints, for every binary, the resolved `serde_json` version and its unified
feature set, and fails when either differs from the stack lock. A conformance binary is built four ways, with and
without a dummy dependency that enables `preserve_order` and one that enables `arbitrary_precision`, and must
produce identical canonical bytes and digests on the golden corpus. Owner: `frob-core` (reader and writer),
`frob-verbs` (admission order), `xtask`.

| | |
|---|---|
| GATE-WS-913 | Known answer. A golden corpus of 10³ artifacts, including duplicates that differ only by escapes at depths 1 to 8, integers at 2⁵³ ± 1, non-printable identifiers, and decimals whose nearest double differs between parsers: every duplicate and every bad identifier is refused on both surfaces before a job identifier exists; canonical digests are identical across the four feature combinations; the mutant writer that serializes a library map fails under `preserve_order`. M0. |
<<<END>>>

<<<INSERT chapter=33 title="Further foundation facts and their admission tests">>>
**What it is.** [S] Four additions to the table of §33.11 and to the sibling gates of §33.15, each with what was
read, the admission test and the interim posture. None was executed for this chapter; the fraction in row 6 is
arithmetic on the two constants in the source.

| # | Component | Fact | Read in source on 2026-11-21 | Admission test | Interim posture |
|---|---|---|---|---|---|
| 5 | `fs-mbd::Wrench` (`frankensim/crates/fs-mbd/src/lib.rs:1695–1701`) | The step's external wrench is `force_world` with `torque_body`: two frames in one value, unlike `fs_ga::Wrench` and unlike §11.1 | Confirmed by reading. The rigid-body type requires the body origin at the centre of mass (`lib.rs:582–595`) and applies the angular impulse as Δt·R̄·`torque_body` (`:1913`) | A free body at R = R_z(90°) receives a pure world force and a pure body torque through the airlock; linear and angular momentum changes match to 10⁻¹² | A fourth frame type in `frob-fsim` only, with exactly two constructors |
| 6 | `fs_g1_train::gaussian_action` (`fs-g1-train/src/ppo.rs:172–189`) | Adjacent channels agree in sign with probability 0.6655 where independence gives ½; the marginals are correct | Mechanism confirmed by reading | The joint-law battery of §23 (rule P-12) | Not linked; a planted copy is the negative control |
| 7 | `serde_json` pins across siblings | `franken_textiles` and `franken_threed` pin `=1.0.151`; `fastmcp_rust` pins `=1.0.154`; `frankenscipy` requires 1.0.154 or later | Confirmed in each `Cargo.toml`; `ftx-drape` takes its workspace pin | `cargo metadata` resolves the graph that INT-FTX-01 needs, or reports the conflict | `ftx-drape` is reached as a separate process in its own workspace; `f3d-*` crates are used with their `serde` feature off |
| 8 | The `metal` 0.33.0 binding under `ft-kernel-metal` 0.1.3 | Its language-version enumeration ends at Metal Shading Language 3.1. It already exposes indirect thread-group dispatch and thread-group memory length, which the gateway does not wrap | Confirmed in the registry source (`metal-0.33.0/src/library.rs:363`, `src/encoder.rs:1715`, `:1730`) | INT-FT-01 records the language version the gateway compiles under | Dialect sources use no construct newer than 3.1; the upstream ask is to wrap existing bindings |

**Row 5, the hand case and the type.** Let the body frame be rotated by 90° about z, so that body x is world y. A
caller holding a body-frame wrench with force (1, 0, 0) N means a push along world y. Copied unconverted into
`force_world` it pushes along world x: the same magnitude, the same energy, a direction wrong by 90°, and nothing
faults. DESIGN: `frob-fsim` defines `MixedWrench { force_world, torque_body }` beside the three maps of §11.3. Its
only constructors are from a body wrench at the centre of mass (f_world = R·f_body, torque unchanged) and from a
world wrench at the centre of mass (τ_body = Rᵀ·τ_world, force unchanged). A wrench at any other reference point is
first moved by the pushforward of §11.3. No other crate names the type.

**Row 7, the consequence.** A Cargo graph holds one version of `serde_json` 1.x. The cloth oracle of §33.15 cannot
be a dependency of a crate that also links FrankenSim, whose graph resolves 1.0.154. INT-FTX-01 therefore runs
`ftx-drape` as a worker process that exchanges whole jobs, the posture §33.14 already defines for FrankenSim, until
an upstream pin moves. That move is an upstream ask and is not scheduled here.

**Row 8, a reported fact to recheck.** A compile probe on this M5 Ultra is reported to show that Metal Shading
Language 3.2 and 4.0 provide no 64-bit integer atomic add and no 64-bit compare-and-swap (FACT to recheck; the
fingerprint of §21.16 re-runs it). The consequence for kernels is in §21.

| | |
|---|---|
| GATE-WS-914 | Known answer. The mixed-wrench probe passes and its unconverted mutant fails; the planted sampler is rejected by rule P-12; `xtask check-deps` reports exactly one `serde_json` version per binary and fails a graph that adds `ftx-drape` to the core workspace; the Metal language version and the 64-bit atomic probe are recorded in the fingerprint. M0; the sampler row at M2. |
<<<END>>>

<<<INSERT chapter=21 title="Counts beyond 2³² without 64-bit atomics">>>
**What it is.** [S] One design for device-side counting that is correct on every backend, given that exact kernels
are 32 bits wide (§21.6, rule 7) and that 64-bit atomics cannot be assumed.

**Facts.** FACT to recheck (a reported compile probe on this M5 Ultra; re-run by `spark.fingerprint`): Metal Shading
Language 3.2 and 4.0 offer no 64-bit integer atomic add and no 64-bit compare-and-swap. By rule 7 a kernel may use
64-bit atomics only if every device family in use records them, so no kernel of a mixed run uses them. A 32-bit
count wraps at 2³² = 4,294,967,296 increments. A run at 4 M world-steps/s reaches that in 18 minutes if one
histogram bin receives every sample.

**Mechanism.** DESIGN:
- *Bins.* `K-x-hist` increments `u32` bins with an integer atomic add, which is order-independent (§35.7).
- *Drain.* At each chunk commit the host reads the bins of the committed slab and adds them into `u64` totals in the
  ledger. The first kernel of the next chunk zeroes the device bins. Totals never live on the device.
- *A no-wrap bound, checked at plan time.* The step plan records, per histogram, the largest number of increments
  one bin can receive in a chunk: worlds × control intervals per chunk × samples per interval. The plan is admitted
  only if that product is at most 2³² − 1. Otherwise the chunk is shortened, or the plan is refused (refusal: a
  device count could wrap within a chunk).
- *Sums wider than 32 bits*, such as integer sums of quantized values, are never accumulated by atomics. Each world
  writes its own `u32` partial sum, and `K-seg-reduce` adds two-limb values in its fixed tree:
  (lo, hi) + (lo′, hi′) = (lo + lo′ mod 2³², hi + hi′ + carry), with the carry computed by an unsigned comparison.
- *Digests* are wrapping sums by definition (§21.8) and need neither.

Owner: `frob-kernels` (`K-x-hist`, the two-limb reduce); `frob-chronicle` (drain and totals); the Spark annexes.

**Known answers.** Two-limb add: (0xFFFFFFFF, 0) + (1, 0) = (0, 1). Boundary of the bound: 65,536 worlds × 4,096
intervals × 16 samples is exactly 2³² and is refused; one sample fewer per chunk is admitted. Planted overflow: a
mutant plan that lets one bin receive 2³² + 5 increments in a chunk reads 5 on the device.

| | |
|---|---|
| GATE-SPARK-915 | Known answer. On CUDA and on Metal a histogram run of 10¹⁰ increments, concentrated in one bin, matches a CPU `u64` reference in every bin; the boundary plan is refused and its neighbour admitted; the dialect lint finds no 64-bit atomic in any kernel source; the two-limb reduce matches `u64` addition on 10⁶ random pairs. M2. |
<<<END>>>

<<<INSERT chapter=41 title="Risk rows keyed by their earliest signal">>>
**What it is.** [S] Thirteen rows for the risk register, each written as risk, earliest signal, bounded response and
the section that owns the detector. The earliest signal is the first observable that distinguishes the risk from
normal operation, and each one is a quantity that some gate already measures. A risk without such a signal is not
admitted to the register.

| Risk | Earliest signal | Bounded response | Detector |
|---|---|---|---|
| A scene looks plausible and cannot support a robot | Scene acceptance fails the settle test, or collision-to-visual deviation exceeds the pack threshold | The scene is not admitted; the part is tagged `optical-only`, or the collision model is corrected | §19.5 |
| A model fits nominal motion for the wrong physical reason | Current draw and mechanical work disagree in the energy ledger while speed matches | The identification is rejected; actuators are identified first, then contact | §30.9, §29.9 |
| Contact capacity favours the wrong morphology | Capacity-overflow counts differ between the arms of a comparison | The comparison is void and is rerun at a capacity that both arms fit | §18.11 |
| A gradient rewards a contact-order artifact | A derivative label other than `smooth`, or an order sensitivity above tolerance | The derivative is refused and the search continues derivative-free | §22.9, D-15 |
| Two-device scaling vanishes under vision | The weak-scaling row with cameras enabled falls below its floor | The figure is published; camera work moves to the machine that holds the scenes | §36.6, §36.12 |
| An agent optimizes failures out of the denominator | A battery's identity changes, or scenarios migrate to the `refused`, `infeasible` or `undetermined` columns | The battery is frozen; a removal is a recorded event; both denominators are printed | §24.8 |
| Adaptation leaks into the final evaluation | A sealed holdout is read more than once, or the causal test fails | The holdout is retired and redrawn | §23.17, §20.15 |
| An optimization changes unrelated semantics | Contact count, solver iterations or a sensor rate differs between the before and after disclosure blocks | The speed claim is void until the work is equal | §36.7 |
| Trace retention deletes the interval that explains a decision | An evidence edge of the artifact graph points at a chunk scheduled to expire | The retention plan is refused, or the chunk is pinned | §29.13, §29.16 |
| A dependency feature changes canonical bytes | The unified feature set of the JSON library differs from the stack lock | The build fails; digests come only from this project's writer | §34 |
| An exact pin in a sibling blocks an oracle | Dependency resolution reports two exact requirements on one package | The oracle runs as a worker process; the pin is an upstream ask | §33 |
| A sampler has correct marginals and a wrong joint law | A pairwise sign-agreement frequency leaves ½ ± 0.005 | The sampler is refused; training results from it are withdrawn | §23 |
| A rare slow mission is absent from the sealed sample | The suite's family count is below the count that its registered bound needs | The stratum reports a finite-suite result and no population claim | §39 |

DESIGN: each row's detector is exercised by a planted instance in the negative-control registry (§38), so the signal
is known to fire. A row whose planted instance is not detected is escalated to an open question with a fail-closed
default.

| | |
|---|---|
| GATE-WS-916 | Known answer. For each of the thirteen rows a planted instance raises the named signal and triggers the named response in 100 of 100 runs; the register lists no risk without a detector. M3 for the first nine rows; the other four at the milestone of their owner. |
<<<END>>>

<<<INSERT chapter=47 title="The release rubric, and the order in which evidence is procured">>>
**What it is.** [S] Four named release levels. A release is a capability matrix in which every cell names what it
can do and the limit of the claim, for example "exact static family with independent active vertical reactions" or
"CUDA two-device throughput in the pinned camera and contact profile". A cell enters a release only with gate
receipts of the class that its level requires.

| Level | What a user can do | Evidence every cell needs | Earliest |
|---|---|---|---|
| Studio preview | Describe a design, check it, simulate it on the CPU referee, read its energy ledger, ask the first mission queries, reproduce any verdict from its address | The known-answer gates of the cell's subsystem; certificates accepted by `frob-verify` on a second machine; no Performance claim and no Reality claim | M1 |
| Gym release | Train and evaluate on device with the corrected semantics; drive the environment from a rival learner through the adapter; reproduce a losing learning experiment as readily as a winning one | The parity gates re-measured on the release hardware with the disclosure block; the determinism class stated per backend; the environment and PPO batteries | M2–M3 |
| Design-science release | Run family queries, adversarial search, autopsy, qualification and the cascade; receive two-sided answers with witnesses | The method gates with competing arms and potency controls; the blinded qualification gate; the partial ten-fold result, published whatever its value | M4 |
| Physically validated profile | Cite a reality band for a named robot, task family and cell | A Ground record: frozen predictions, sealed trials and a conformal band at its stated coverage; otherwise the cell reads `reality: no-claim` | M5 or later |

**Rules.** DESIGN:

1. A level is claimed per cell, not per release. A release may hold cells at different levels, and prints the matrix.
2. Tied, infeasible, incomparable and inconclusive are complete dispositions of a cell's study and are published.
3. A cell that loses its evidence, through a pin move, a retired holdout or a failed re-run, drops a level at the
   next release, and the matrix records the drop.
4. No level uses the word "verified" (§12).

**Order of evidence.** Evidence is procured in dependency order, and a claim is never scheduled ahead of the kind of
evidence it needs: (a) what can be authored at once, namely analytic fixtures, exact checkers and CPU runs; (b) what
needs a pinned external comparison, namely an incumbent re-measured on the same hardware; (c) what needs an operator
or a dataset, namely bench logs, scans and proficiency-matched trials; (d) what needs the native devices. A
successful CPU run on the M5 Ultra, or in a shell session on `trj`, establishes the CPU path and nothing about a
device path.

`xtask check-contracts` reads the release manifest and verifies that every cell cites receipts of the required class
at the pinned commit. A cell without them cannot be listed at that level (refusal: release cell lacks evidence for
its level).

| | |
|---|---|
| GATE-WS-917 | Known answer. A release manifest with one planted cell whose receipt is missing, one whose receipt is of a lower class, and one whose receipt predates a pin move is rejected on all three; a cell demoted by a failed re-run appears one level lower in the next matrix. M1, and at every release. |
<<<END>>>

<<<INSERT chapter=38 title="Known answers adopted from corrected fixtures">>>
**What it is.** [S] Five rows for the known-answer suite, each with a negative control. Every value is derived on
file from the stated inputs; none is a measurement. The rows are owned by `frob-conformance` and run under the verbs
of their subsystems (`design.check`, `gait.atlas`, `task.fixture`).

| Row | Inputs | Value | Negative control | Owner section |
|---|---|---|---|---|
| Thermal runaway, second card | Constant I = 25.4 A; R₀ = 0.54 Ω referenced to T₀ = 20 °C; α = 0.00393 K⁻¹; C = 140 J/K; G = 1/2.2 W/K; ambient and initial winding temperature 25 °C; limit 120 °C | Loss 348.4 W at 20 °C and 485.3 W at 120 °C. s = I²·R₀·α = 1.369 W/K exceeds G, so there is no equilibrium; growth rate (s − G)/C = 0.006533 s⁻¹; the limit is reached at 33.49 s. Regearing from 12 to 16 at equal output torque cuts copper loss by 1 − (12/16)² = 43.75% | Reading R₀ as referenced to 25 °C gives 34.08 s; the constant-resistance model gives 39.92 s; both fall outside a 0.1 s tolerance | §15.7 |
| Plate sinkage, power law with n = 0.5 | k_c = 0.41 kN/m^1.5; k_φ = 21.6 kN/m^2.5; two tracks of length 0.55 m; g = 9.81 m/s² | 45 kg on 8 cm tracks sinks 3.52 cm; 75 kg on 8 cm sinks 9.79 cm, more than an 8 cm belly clearance; 75 kg on 12 cm sinks 4.96 cm | The same constants with n = 0.7 give 9.17, 19.01 and 11.71 cm, so a mistranscribed exponent is caught | §18.10 |
| Contact strata under the observation subgroup | Four feet, each open, stick or slip | Under the four-element leg-permutation group: 27 strata and 162 one-foot transitions. Under the two-element subgroup that a forward-facing sensor leaves: (3⁴ + 3²)/2 = 45 strata and 648/2 = 324 transitions | Using the mechanical group where the observation subgroup is required collapses 45 strata to 27 | C-16, §13.4 |
| Reality of roots at saturation | A Stewart–Gough platform instance with 40 real assembly modes; a 6R instance with 16 real inverse-kinematics solutions (FACT, literature: Dietmaier 1998; Manseur and Doty 1989; Appendix H; to be re-read) | The continuation lane certifies 40 of 40 and 16 of 16 roots as real by the conjugate-ball test. A generic instance of each family has the same complex count, and its real count is whatever the test certifies | A checker that calls a root real when its imaginary part is below a tolerance, instead of by ball disjointness, is caught on a planted near-real conjugate pair | §25.21.4 |
| A single wheel at a sharp lip | Axle load W = 25 N; lip height 12 mm; pure drive couple; a = √(2rh − h²), b = r − h; lift-off needs M = W·a and μ ≥ a/b | r = 60 mm: M = 0.900 N·m and μ ≥ 0.7500. r = 75 mm: M = 1.017 N·m and μ ≥ 0.6459. With μ = 0.70 and a 1.2 N·m cap only the 75 mm wheel is admitted; with μ = 0.80 and a 0.95 N·m cap only the 60 mm wheel is | A check of torque alone admits both wheels in the first condition | §24.10 |

The last row is the pure-couple case of a general rule for the corner. With a forward axle push P and a drive
couple M, lift-off requires b·P + M = W·a, the corner reaction requires a·P + b·W ≥ 0, and friction requires
|M| ≤ μ·(a·P + b·W).

| | |
|---|---|
| GATE-FID-918 | Known answer. Every row is reproduced at its tolerance (times to 0.1 s; sinkage to 10⁻⁵ m in double precision; counts exact; admissions exact) and every negative control fails. Rows 1, 3 and 5 at M1; row 2 at M4; row 4 with the continuation lane. |
<<<END>>>

<<<INSERT chapter=32 title="The agent diagnosis battery">>>
**What it is.** [S] An acceptance test of the agent surface on setups that are wrong in the ways an engineer meets,
scored on diagnosis and on restraint. It complements GATE-TETHER-01 and GATE-TETHER-13, which score task completion.

**The battery.** DESIGN: 30 tasks on the fixtures of §24.10 and §24.11, each handed to a fresh agent with only the
generated documentation and a budget.

| Class | Seeded cause | The minimal evidence that identifies it |
|---|---|---|
| Wrong units | A length entered in millimetres where metres are declared, inside the admissible range | The mass roll-up of `design.check` against the declared budget |
| Insufficient friction | Floor friction below the creep threshold of the lip | The friction-demand row of the analytic layer; no rollout is needed |
| Low current limit | A driver limit below the stall requirement | The tread-force cap row, and one `sim.run` to confirm the stall reason |
| Hidden visual and collision mismatch | A collision shape that differs from the visual by more than the pack threshold | The collision-to-visual deviation of `scene.accept` |
| Delayed observations | A packet latency beyond the decision deadline | `sense.causal-test` and the sample-age field |
| No defect (5 tasks) | None | A passing verdict; the correct answer is that nothing is wrong |
| Inconclusive (5 tasks) | The evidence supplied cannot decide between two causes | The correct answer is `unknown`, with the experiment from `experiment.suggest` that would decide |

There are four tasks per seeded class, in randomized order, with the class hidden.

**Scoring.** Five figures per agent arm: the fraction of seeded tasks whose first named cause is the seeded one; the
number of simulations run beyond the minimal evidence above; wall time to the answer; whether a clean client
reproduces the diagnosis from the receipts alone; and, on the inconclusive and no-defect tasks, whether the agent
reported `unknown` or "no defect" without converting it into a cause. One invented cause on an inconclusive task
fails the arm.

Owner: `frob-conformance` (battery and scorer); `frob-verbs` (transcript capture). The battery runs through
`session.open`, with a namespace per arm.

| | |
|---|---|
| GATE-TETHER-919 | Performance of the method. On the 30 tasks a fresh agent names the seeded cause first in at least 16 of 20 seeded tasks; converts no inconclusive or no-defect task into a cause; runs at most twice the minimal number of simulations in the median task; and every diagnosis is reproduced by a clean client. A second arm with `sim.run` and logs only is run as the comparison, and the difference is reported with its interval. M3. |
<<<END>>>

---

## (d) Contradictions and errors noticed in the parts

1. **Two names for one verb.** P01 C-18 (line 850) and the P01 ledger (line 2376) call the separating-world query
   `world.separate`. P07 §25.3.1 and the verb inventory call it `mission.separate`. `world.separate` is not a
   registered verb.
2. **Two verbs for skill transport.** `skill.transport` (P06 §22.10, line 462) and `mission.transport` (P07 §25.3.1)
   are both registered. One should own the query and the other be an alias, or their scopes should be stated.
3. **The energy-budget card is labeled two ways.** P01 C-31 test (lines 1386–1388) says "0.25 J stored and 0.10 J
   reserve" and divides (2.00 − 0.64 − 0.25 − 0.10) by 20. P06 §22.11 (lines 527–531) says 0.25 J is external work
   (gravity on an 8% slope over the stroke), 0.10 J is stored in a series spring, and 45.5 ms follows from a further
   0.10 J of reserve. The totals agree (1.49 J, 50.5 ms). P06 is the corrected attribution; P01 should match it.
4. **One audit, two flags and two thresholds.** P03 §16.10 labels a result `energy-suspect` when created energy
   exceeds 1% of actuator work per episode. P09 §29.9 raises `energy-created` above 1% of the absolute work exchanged
   in any 0.5 s window on a fast tier. Different names, windows and denominators for the same monitor.
5. **Two sets of derivative labels.** P01 D-15 and the C-16 ledger list `smooth`, `saltation`, `discrete-step`,
   `fixed-mode`, `one-sided`, `generalized`, `smoothed`, `finite-difference`, `refused`. P06 §22.9.3 says every
   derivative is `smooth`, `one-sided`, `set-valued` or `surrogate`.
6. **Two names for an uncovered window.** P01 C-30 (lines 1316, 1330) returns `inconclusive`. P08 §24.5 defines the
   verdict values as `satisfied`, `violated`, `undetermined`, `invalid`.
7. **A withdrawn attribution is repeated four times.** P00 §0.7 (lines 266–267) and §2.3 item 6 (line 748), P05
   §36.5 (line 1396) and P07 §25.22 (line 1858) say that about 576 µs, 94% of the specialized batch, was "shared
   checking and serialization". The source now states that the 575.7 µs is a residual obtained by subtracting the
   34.4 µs solve-only median on pre-parsed input from the 610.1 µs full specialized median, that it includes
   parsing, result assembly, checking, serialization and harness costs, and that separate timers would be needed to
   attribute it. The ceiling of about 1.46 (842.8 ÷ 575.7) stands. Suggested wording: "a residual of 575.7 µs that
   the solver does not touch (parsing, assembly, checking, serialization and harness, not separately timed)".
8. **The ten-fold decision rule as sketched cannot support a population claim.** P00 §2.1 (lines 689–693) specifies
   "a pooled ratio with at least ten paired campaigns per arm". If the campaign is the independent unit, ten is
   below the 32 that any distribution-free one-sided 95% procedure needs before it can certify ten-fold (block 4).
   Either the unit is the mission family and the count rises, or the result is a finite-suite result. BET-13's
   wording, "ten-fold on the sealed suite", already fits the second reading.
9. **A device counter with no width.** P09 §29.7 lets a run-level histogram accumulate "on the device with integer
   atomic add", and P05 §21.6 rule 7 makes exact kernels 32 bits wide. Nothing bounds the count; a 32-bit bin wraps
   after 2³² increments, which a 4 M world-steps/s run reaches in 18 minutes (block 14).
10. **Counts of foundation defects are stale.** P12 §33.11 opens with "Four defects or limits"; P06 ADR-CORTEX-03 and
    P00 §1.7 say the suite's PPO "has three errors". The sampler is a further defect in the same file, and the
    mixed-frame wrench is a further convention hazard (block 13).
11. **The rainflow owner is cited two ways.** P01 C-35 foundations (line 1544) credits `fs-solid::fatigue`. P10 §31.4
    (line 727) credits `fs-material` (`RainflowMiner`), which is where the counter lives; `fs-solid::fatigue` imports
    it.
12. **The cloth oracle cannot be linked as planned.** P12 §33.4 row FD-FTX and §33.15 place `ftx-drape` in
    `frob-conformance` behind a flag. `ftx-drape` takes its workspace's `serde_json = "=1.0.151"`, and FrankenSim's
    graph resolves 1.0.154 through FrankenSciPy, so the two cannot share a Cargo graph (block 13, row 7).
13. **"Unmeasured" is now reported measured.** P05 §21.16 (lines 596–598) lists 64-bit atomics among the things that
    are unmeasured. A probe is reported to show them absent on Metal. The sentence should say "reported absent on
    Metal, to be re-run by the fingerprint".
14. **The `fs-evidence` coupling is described without the structure the source now gives.** P12 §33.12 says that
    through `fs-evidence` "nearly every physics crate" depends on `fs-qty`. The third plan now splits the 22 coupled
    crates into 12 direct and 10 transitive, and notes that removing `fs-qty` from `fs-evidence` alone would free 8
    of the 22. That is a smaller upstream ask than FD-FSIM-15 and could be filed beside it. Not re-verified here.
