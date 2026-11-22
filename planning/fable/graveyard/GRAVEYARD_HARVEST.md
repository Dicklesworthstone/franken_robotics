# Graveyard harvest for FrankenRobotics

Produced with the `alien-graveyard` skill. Entry numbers are the `### N.M` headings of `alien_cs_graveyard.md`; "summary" is the FrankenSuite summary corpus. Goal: ten times less time and compute per accepted robot-design decision.

**Evidence.** No code exists, so no profile exists. Every hotspot is anticipated; to be confirmed by the first profile (M2, plan §28.3). No gain here is measured.

**Scoring.** EV = (Impact × Confidence × Reuse) / (Effort × Friction), each 1 to 5, gate 2.0. Impact counts only the revision goal; Confidence is capped at 4 without a profile; "rel" is the skill's weighted relevance. Tier S = fix in the architecture now, because the target depends on it and retrofit is costly; A = lands with its milestone; B = wait for the profile. Order: tier, then EV, because raw EV favours cheap enablers (a lint rule scores 8.0).

**Ecosystem.** The core admits only Franken runtime dependencies (plan §20.1), so "Exists" names a sibling crate to consume or lift. Of the Fast Start ten, §15.1 and §12.1 ship inside Cards 1 and 3; §6.12 and §0.19 exist as `fs-propcheck` and `franken-evidence`.

Hotspots: H1 contact solve, H2 mixed tree sizes, H3 distance fields, H4 deterministic primitives, H5 fork, H6 telemetry, H7 incremental recompute, H8 kernel specialization, H9 daemon, H10 unified memory, H11 lineage, H12 rare events, H13 simulation testing.

## (a) Candidates

| H | Graveyard entry: use | Replaces | I·C·R/E·F = EV (rel) | Tier | Exists | Risk → countermeasure |
|---|---|---|---|---|---|---|
| 7 | §6.1 Self-Adjusting / Incremental Computation (+§8.15): memo graph past `ModelId` | Memo that stops at `RobotModel` (§12.3) | 4·4·5/3·2 = 13.3 (4.8) | S | `fs-recompute` | Stale hit → full identity key, shadow recompute |
| 9, 2 | §8.2 Vectorized Execution + Morsel-Driven Parallelism (+§14.2 Flat Combining): slot-level episode dispatch | One environment per job; no sharing described | 5·3·5/4·3 = 6.3 (4.0) | S | `fs-exec` (CPU pattern) | Co-tenant dependence → slot-invariance gate |
| 12 | §12.12 Off-Policy Evaluation (IPS/DR): paired tier correction, importance sampling | Pass/fail promotion (§14.5), plain Monte Carlo | 4·3·4/3·3 = 5.3 (4.1) | S | `fs-eproc`, `fs-uq` | Weight degeneracy → clipping, sample-size floor |
| 9 | §12.1 Conformal Prediction: interval on `sim estimate` | Unspecified interval | 2·4·4/1·2 = 16.0 (4.0) | A | asupersync `lab/conformal.rs` | Driver change → recalibrate per device family |
| 7, 10 | §15.1 S3-FIFO Cache Eviction: memo store, scene cache | LRU or unbounded | 2·4·4/1·2 = 16.0 (3.8) | A | `fsqlite-pager/s3_fifo.rs` | Unequal miss costs → admit by recompute cost |
| 13 | §6.20 Deterministic Simulation Testing (+§13.13 LDFI, §6.15): daemon and ledger under seeded faults | Chaos at kernel-batch boundaries (§30.6) | 2·4·3/2·2 = 6.0 (3.9) | A | asupersync `lab` (chaos, ldfi, replay_minimization) | Fake device diverges → dual-run scenarios |
| 9 | §10.3 Learning-Augmented Online Algorithms: shortest estimated work first | "Fair sharing", unspecified (§24.5) | 3·3·3/2·3 = 4.5 (3.8) | A | none found | Starvation → aging, round-robin fallback |
| 3, 10 | §7.10 Persistent / Immutable Data Structures: shared brick pool, one directory per scene variant | K scene variants, sharing unspecified (§16.8) | 3·3·3/3·2 = 4.5 (3.9) | A | `fs-rep-sdf` (CPU) | Low sharing on scans → dense fallback |
| 5 | §15.7 Zygote / Copy-on-Write Process Forking: warm template environments, fork by indexed copy | Environment per job, no fork | 3·3·4/3·3 = 4.0 (3.7) | A | none | Correlated branches → nested estimator |
| 4 | §4.4 Lattice-Based Deterministic Parallelism (LVars): atomics only for integer max, min, or, add | Float-atomic ban, no positive rule | 1·4·4/1·2 = 8.0 (3.8) | B | lint | Misuse → `check-generated` |
| 6 | §15.5 NitroSketch / Probabilistic Telemetry: sample worlds by address hash, weight 1/p | Reservoir sample (§23.2) | 1·4·3/1·2 = 6.0 (3.8) | B | none needed | Seed correlation → own Philox stream |
| 11 | §8.5 CRDTs (grow-only set, max-lattice): lineage and archive merge | Merge unspecified (§24.7) | 2·4·3/2·2 = 6.0 (4.0) | B | `fs-archive` | Ties → total order |
| 4, 2 | §8.14 Radix Hash Join, partition phase: one stable integer-key sort | Ad hoc sorts and compactions | 1·4·5/2·2 = 5.0 (4.0) | B | none found | Small n → sorting networks |
| 2 | §4.6 Nested Data Parallelism / Flattening Transform: segmented arrays, size classes | Padding to the largest robot (assumed), (60/18)³ ≈ 37× in the factor | 3·3·3/3·3 = 3.0 (3.5) | B | `fs-soa` | Memory blow-up → fixed segments |
| 1 | §9.6 Communication-Avoiding Algorithms: fused assemble, factor, solve | `K-hess`, `K-chol`, `K-linesearch` chain | 3·2·3/3·2 = 3.0 (3.5) | B | `fs-la::batched_f32` | Constants → gate at n = 18, 30, 60 |
| 6 | §9.4 Sketching & Streaming Algorithms: integer log-bin histograms | T0 scalars or T2 traces | 1·4·3/2·2 = 3.0 (3.4) | B | none | KLL, t-digest order-dependent → integer bins |
| 1, 8 | §6.17 Adaptive Compilation & Runtime Specialization (+§6.7): factorization staged per topology | Generic dense Cholesky | 3·3·3/4·3 = 2.3 (3.3) | B | `fs-tilelang` | Compile storm → generic default |
| 8 | §6.6 Equality Saturation / E-Graphs: extraction beyond CSE | CSE (BD-018) | 2·3·3/3·3 = 2.0 (3.2) | B | `fj-egraph`, `fsym-simplify` | Float bits change → real-exact rules, node budget |

## (b) Recommendation cards

### Card 1. Memoize everything downstream of `ModelId`

- **Change:** One content-addressed memo graph for analyses, controller synthesis, step plans (keyed by shape class, so a parameter edit recompiles nothing) and episodes (keyed by full identity, kernel versions, tier and device family).
- **Hotspot evidence:** Anticipated; to be confirmed by the first profile. In §24.8 a comparison follows a 2,048-world run of the previous version on the same scene; the plan does not say those episodes are reused.
- **Mapped graveyard sections:** §6.1, §8.15, §15.1.
- **EV score:** (4 × 4 × 5) / (3 × 2) = 13.3; relevance 4.8.
- **Priority tier:** S.
- **Adoption wedge:** A `Memo` trait before each verb's executor; shadow, then D0 analyses, then D2 episodes.
- **Budgeted mode:** 64 GB store; on exhaustion evict, never block.
- **Expected-loss model:** States {valid hit, stale hit, miss}; actions {serve, recompute, shadow}; a stale result served costs 100, a needless recompute its GPU-seconds.
- **Calibration + fallback trigger:** Recompute 1 in 64 hits chosen by key hash; one digest mismatch stops serving that node kind.
- **Isomorphism proof plan:** Ordering and ties unchanged (results placed by index); floats identical (hits only within D0 or same-backend D2); RNG by address; incremental equals full recomputation on 10⁴ random edit sequences.
- **p50/p95/p99 before/after target:** Before: not measured. Target, unmeasured: `design check` after a one-parameter edit 0.2 / 0.5 / 1.0 s.
- **Primary failure risk + countermeasure:** Operability gap (an input missing from a key); keys derived from declared inputs, each property-tested.
- **Repro artifact pack:** `env.json`, `manifest.json` with key-derivation version, `repro.lock`; no `LEGAL.md` needed.
- **Primary paper status:** Hypothesis; Acar 2002 and Adapton 2014 unread.
- **Interference test status:** No second controller; a partly memoized comparison must reach the same e-value.
- **Demo linkage:** `demo_id: memo-edit-loop`, `claim_id: CLM-MEMO-01`.
- **Rollback:** `--no-memo`; the store is deletable.
- **Baseline comparator:** Full recomputation per verb with compile-only memoization.

### Card 2. Slot-level episode dispatch in `frobd`

- **Change:** One persistent environment per device and shape class. A slot that ends an episode pulls the next from any admitted job at the masked reset; model, scene, task and job are per-world buffers.
- **Hotspot evidence:** Anticipated; to be confirmed by the first profile. The 96-pair comparison of §24.8 fills 192 of 8,192 slots, 2.3% of the §28.1 batch; the plan describes no batch sharing between jobs.
- **Mapped graveyard sections:** §8.2, §14.2, §4.6, §10.3.
- **EV score:** (5 × 3 × 5) / (4 × 3) = 6.3; relevance 4.0.
- **Priority tier:** S; it changes the unit of work.
- **Adoption wedge:** A dispatcher inside `frobd`; addresses and digests unchanged; shadow on `batch-cpu` first.
- **Budgeted mode:** 64 jobs per environment, 5% of slots reserved for short jobs; on exhaustion queue. No shape change enters a running environment.
- **Expected-loss model:** States {under-filled, saturated, mixed shapes, one agent dominant}; actions {co-batch, second environment, queue, preempt}; loss = idle slot-seconds + waiting + fairness deficit.
- **Calibration + fallback trigger:** Conformal interval on step time; revert to one batch per job on any slot-invariance failure, or p99 step time 10% over solo.
- **Isomorphism proof plan:** Results keyed by world address, never slot or arrival; dispatch a pure function of queue state; no kernel reads another world's rows, so digests match under any co-tenancy (D2); golden: 1,000 episodes solo and in 100 random co-tenancies.
- **p50/p95/p99 before/after target:** Before: not measured. Target, unmeasured: admission to first step for jobs up to 256 worlds, 0.1 / 0.5 / 2 s.
- **Primary failure risk + countermeasure:** Performance unpredictability (per-world parameter reads); gate at 8,192 identical worlds, at most 5% loss.
- **Repro artifact pack:** `env.json` with device load, `manifest.json` with the job mix, `repro.lock`.
- **Primary paper status:** Hypothesis; Leis 2014 and Hendler 2010 unread.
- **Interference test status:** Required, not run: dispatch (per reset), racing (per round), admission (per job); oscillation microbench and lab-runtime replay.
- **Demo linkage:** `demo_id: shared-batch`, `claim_id: CLM-SLOT-01`.
- **Rollback:** `frobd --dispatch=per-job`.
- **Baseline comparator:** One environment per job, queued, as the plan is written.

### Card 3. Paired multi-fidelity estimates with exact importance weights

- **Change:** Promotion returns a doubly robust estimate: the fast-tier mean over all N episodes plus the mean referee-minus-fast difference on n episodes at the same addresses. Failure rates draw disturbances from a tilted proposal; weights are exact because every draw is an addressed Philox stream (§27.4).
- **Hotspot evidence:** Anticipated; to be confirmed by the first profile. Arithmetic: at the 0.2× real-time referee target a 20 s episode costs up to 100 core-seconds; a 1% failure rate needs about 4,200 plain episodes for ±30% at 95%.
- **Mapped graveyard sections:** §12.12, §12.1, §12.15, §0.18.
- **EV score:** (4 × 3 × 4) / (3 × 3) = 5.3; relevance 4.1.
- **Priority tier:** S; pairing and proposal fields belong in the episode schema now.
- **Adoption wedge:** The estimator behind `eval promote`; verdicts unchanged; shadow (report both rules, act on the old) first.
- **Budgeted mode:** 256 referee episodes per promotion, weights clipped at 20 (clipped mass reported); on exhaustion return `provisional`.
- **Expected-loss model:** States {tiers agree, constant bias, tier-sensitive, rare failure}; actions {establish, correct and establish, add episodes, refuse}; a wrong establishment costs 100, a referee episode 1, a wrong refusal 10.
- **Calibration + fallback trigger:** Interval coverage against full-referee values on the standard battery; revert to the paired e-process when effective sample size falls under 30% of n.
- **Isomorphism proof plan:** Episode digests untouched (ordering, ties, floats, RNG addressing); with zero tilt and n = N the estimator equals the plain mean; type-I error at most α over 10⁴ null comparisons under optional stopping.
- **p50/p95/p99 before/after target:** Before: not measured. Target, unmeasured: 16 / 64 / 256 referee episodes per established verdict. Variance ratio is about 1 − ρ²: tenfold at tier correlation 0.95, little below 0.5.
- **Primary failure risk + countermeasure:** Performance unpredictability (heavy-tailed weights); self-normalized weights, median-of-means.
- **Repro artifact pack:** `env.json`, `manifest.json` (addresses, proposal, estimator version), `repro.lock`.
- **Primary paper status:** Hypothesis; the entry names no paper, and the corpus has no splitting entry.
- **Interference test status:** Required, not run: proposal frozen within a round; null-coverage replay corpus (§0.25.6).
- **Demo linkage:** `demo_id: promote-dr`, `claim_id: CLM-DR-01`.
- **Rollback:** `--estimator=paired-eprocess`.
- **Baseline comparator:** The promotion rule of §14.5 and plain Monte Carlo.

## (c) Rejected

- **§6.4 Stochastic Superoptimization, §6.3 CEGIS.** EV 0.5; no sound equivalence check for floating-point code.
- **§6.5 Polyhedral.** EV 1.0; loops of at most 60 unroll under staging.
- **§7.15 Minimal Perfect Hashing, §7.1 Succinct.** Wrong bottleneck; a dense brick directory is 0.3 to 2.3 MB for the §16.8 scenes (8³ bricks, 4-byte slots assumed).
- **Copy-on-write for device world state (§15.7, §7.10).** Wrong bottleneck; copying 8,192 worlds at 30 kB is 0.25 GB.
- **§3.10 Hindsight Logging.** Already in the plan (rings flushed on failure, replay by address).
- **§8.12 Escrow counters, §13.1 EPaxos, §13.7 SWIM.** Operability; one daemon per host, nothing replicated.
- **§7.12 CXL-Aware, §9.2 AMG, §9.3 FMM.** Wrong bottleneck; one memory tier, dense systems of 18 to 60 unknowns.
- **§12.14 Contextual Bandits.** Operability; a second controller on shared telemetry (§0.25.5).

## (d) FrankenSuite conventions to adopt

A search of the plan finds none of these terms.

- **Artifact-graph identifiers** (summary, "Artifact Graph"): each gate claim gets a `claim_id` resolving to `evidence_id`, `policy_id`, `trace_id`; import the types from `franken-kernel`.
- **Contracts-as-code** (summary §0.4): a machine-validated record per change with budgets and safe mode, inside the existing `check-contracts` (plan §26.6 restricts new checks).
- **Shadow-run as the adoption wedge** (summary §0.20): the incumbent is the plan's own reference path.
- **Demo gate and `slo.yaml`** (summary, "Definition of Done"): `demo.yaml` with `demo_id`, `claim_id`s, commands under 60 s, checksums; budgets with safe-mode triggers.
- **Two planes** (summary, "Suite Reference Architecture"): kernels never depend on dispatcher, memo policy or estimator.
- **Controller gate** (summary, "Critical Path DAG"): no adaptive controller without replay scenario, evidence records, calibration report and demonstrated fallback.
