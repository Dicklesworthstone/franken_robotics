---
key: "EPIC-M2"
title: "Milestone M2, batch, backends, families: the batch under the studio, measured against the incumbent"
type: "epic"
priority: 1
milestone: "M2"
labels: ["m2", "delivery", "solid", "gate"]
parent: "EPIC-PROGRAM"
deps: []
needs: []
related: ["BD-224", "N40-33", "N40-24", "N40-08", "N40-04", "N40-05", "EPIC-M1", "EPIC-M3"]
gates: ["GATE-KIN-03", "GATE-FID-01", "GATE-FID-03", "GATE-SPARK-02", "GATE-SPARK-03", "GATE-SPARK-04", "GATE-PHYLUM-03", "GATE-CORTEX-01", "GATE-CORTEX-05", "GATE-CHRON-01", "GATE-GAUNT-01", "KILL-KIR", "KILL-CONTACT", "KILL-METAL", "KILL-PHYLUM", "KILL-BUCKET", "KILL-D1", "KILL-PARITY", "KILL-SHARD", "KILL-LEARNKERN", "KILL-MARROW-GUARD", "KILL-SINEW-ROWS", "KILL-POOL", "KILL-EXPOSURE", "KILL-SCHED", "KILL-TENFOLD-LEVER"]
plan_refs: ["§43.5", "§43.1", "§43.2", "§40.6", "§40.3", "§40.4", "§43.12"]
---
## Background

**Purpose.** M2 puts the batch under the studio: one model on `batch-cpu`, CUDA and Metal, with many designs in one
batch, measured against the incumbent on the same cards. It decides by measurement the four things the engine rests
on: the kernel toolchain (are kernels derived from symbolic source, or hand-written), the contact law (does convex
compliant contact agree with the referee), the Metal tier (is the Mac a rollout tier), and mixed batches (do
different body plans share one batch at acceptable cost).

**Why here.** GPUs arrive on top of something that already works (the M1 studio and its referee, which becomes the
parity oracle), and after the rival has been measured (M0, M1), so every Performance figure of M2 has a measured
denominator. M2 is also where the engine could turn out not to be the product: each of the four decisions has a
kill gate with a fallback named in advance, and the milestone's exit line is precisely that those decisions are on
record, not that they went one way. Planning estimate: 6 weeks (DESIGN). Critical path: yes. M2 and M3 together
carry half of all convergence gate parts of the program; that concentration is by design and is a schedule risk.

**Demonstrates.** NS-02 *Teleport a world*: `frob sim replay --world 1337 --tier batch-cpu` reproduces one world of
a GPU batch alone on the CPU from its address (seed, world, episode), stops at the first step whose digest differs
or at the first refusal, and reports the contact that started it; where the device reaches class D1 the digests
agree to the end, and the report says which class applied. NS-10 *The same task on a Mac*: the quadruped task
trains through Metal, agrees with the CUDA run inside the declared determinism class, and reports its throughput
beside the one Metal-capable incumbent's on the same machine and beside one RTX 4090's. NS-14 *256 different
robots in one batch*: one dispatch steps 256 distinct designs (three leg counts, mixed link lengths and actuator
cards, in size buckets) with 32 worlds each on one RTX 4090; no kernel is compiled; each world reproduces the
digest of its single-design run; the report says which arm of the head-to-head won.

## Scope

In: the batched step in two profiles on three backends; scene packs and terrain; contact law CL-1 with cone and
joint rows; sensors I; `VecEnv` and PPO on device; variants and cohorts with the four-arm head-to-head; fidelity
and divergence ledgers, replay, teleport, forks, the sampled referee; the job engine; cost estimates; throughput
and training-time figures against the incumbent.

Explicitly not in M2: interiors and task families beyond velocity tracking and terrain traversal (M3); slot
dispatch across jobs and the full MCP surface (M3); promotion with tier correction (M3); family policies (M4).

What M3 needs from M2: a frozen standard battery and fidelity envelopes; replay and forks; the job engine; the
decisions above (the partial ten-fold gate charges whichever engine survived, at its measured rate); OPEN-PROG-2
and OPEN-ARENA-3 answered or defaulted, because sealed world families are built at M3.

## Technical approach

**1. The artifact.**

- The batched step in the `lockstep` and `strict` profiles on three backends; `frob spark check`,
  `frob spark place` with requested-versus-actual receipts, the arena lease and the binary kernel cache.
- Scene packs and terrain generators (`frob scene gen`, `frob scene compile`). Contact law CL-1 with the
  four-dimensional cone and joint stiction and backlash rows, with diagonal scaling.
- Proprioception, height scans, depth, lidar, and camera exposure and rolling-shutter timing; stills and the
  viewer page.
- `VecEnv` with explicit reset, preserved final observations and the termination and truncation distinction; PPO
  on device (`frob train start`), with the two-device update rule.
- Per-world parameter variants and model variants: `frob phylum plan`, `frob phylum sweep`; the head-to-head
  experiment run and published with all four arms.
- The fidelity and divergence ledgers (`frob fidelity ledger`); replay and teleport
  (`frob sim replay --world 1337 --tier batch-cpu`); forks (`frob run fork`); the sampled referee
  (`frob fidelity shadow`).
- The daemon's job engine with device ownership and idempotent submission; `frob sim estimate`.
- Throughput and training-time figures against the incumbent measured on `trj`, each with its disclosure block.

**2. Gates with a part due at this exit (compiled register tables: 2 admission, 80 convergence).** Rows that had a
referee or CPU part at M1 close their `strict`, batched or every-tier part here.

| Gate | Claim | Part due | Pass criterion in brief (TARGET) | Defined in |
|---|---|---|---|---|
| INT-ASUP-02 | Admission | M2 | Foundation: asupersync lab runtime. The job protocol runs under the lab runtime: 100 of 100 repeats of one seed give identical traces; a planted lost-chunk bug is found within 10,000 explored schedules and replays from its minimized fixture | §33.6 |
| INT-FJAX-01 | Admission | M2 at the earliest, only if the profile asks | Foundation: frankenjax (pattern) [F]. With rewrite rules that preserve floating-point results under the kernel's determinism class, the extracted kernel matches the baseline bit for bit and the operation count falls by at least 10% on at least three kernels | §33.15 |
| GATE-ARENA-01 | Known answer | M2 | Every task is reproducible inside its determinism class from its `TaskId`, `SceneId`, `ModelId` and seed; reference-robot baseline scorecards are recorded. | §24.17 |
| GATE-ARENA-04 | Known answer | M1 on the referee; M2 on the batch tiers | Fixture 1: every analytic row within tolerance on the referee and inside the fidelity envelope on the batch tiers; the creep rigs flip at the stated thresholds; both negative controls are caught. | §24.17 |
| GATE-ARENA-06 | Known answer | M2 | Fixture 2: limiting-model rows within 1% on the referee; slip invariance to 10⁻¹² relative in double precision; reflected cases mirror inside the determinism class. | §24.17 |
| GATE-ARENA-08 | Known answer | M2 | Reset cache: penetration at most 0.5 mm and first impulse within 5% of static support on all 64 states for each reference robot; a design edit invalidates the cache. | §24.17 |
| GATE-ATEL-12 | Performance | M1 (analysis part); M2 (episode part) | PO-ATEL-07 holds; a shadow mismatch stops serving within one hit; `design check` after a one-parameter edit in 0.2 / 0.5 / 1.0 s at the 50th / 95th / 99th percentile. | §28.23 |
| GATE-AUTOPSY-01 | Known answer | M2 | Null forks of 1,000 failing worlds replay with identical digests on the originating backend; a world forked alone and in a batch of 512 children gives identical digests. | §27.17 |
| GATE-AUTOPSY-02 | Known answer | M2 | Classification: 40 catalogued pairs of intervention and state are classified correctly; the warm-start graft and a mid-episode gear change are refused with the right fields. | §27.17 |
| GATE-AUTOPSY-07 | Known answer | M2 | First divergence: planted divergences found at the right tick and stage in 100 of 100 in exact mode; the pointwise mutant is caught; tolerance mode reports gaps. | §27.17 |
| GATE-CHRON-01 | Known answer | M2 | 1,000 random worlds from a GPU run replay with identical digests on the same backend, and inside the class on `batch-cpu`; the same worlds replayed alone, in a batch of another size and among other co-tenants give the same digests. | §29.20 |
| GATE-CHRON-02 | Known answer | M2 | Teleport localizes a planted single-kernel divergence to the right step and stage in 100 of 100 trials. | §29.20 |
| GATE-CHRON-03 | Known answer | M1 (referee); M2 (single precision) | The energy ledger closes: residual below 10⁻⁹ relative on the referee and 10⁻³ on single-precision tiers; a damped pendulum's ledger matches the hand calculation. | §29.20 |
| GATE-CHRON-04 | Known answer | M1 (CPU); M2 (device) | The chunk protocol under 10⁵ seeded fault schedules: INV-CHRON-01 to -04 hold in every one; each planted violation (late lease, overlap, conflict, tick-zero reuse) is refused, 100 of 100. | §29.20 |
| GATE-CHRON-05 | Known answer | M2 | Fork classification: the five classes of §27.4 are assigned correctly on 100 planted change sets; the warm-start graft is refused 100 of 100; the empty intervention equals replay on 1,000 worlds. | §29.20 |
| GATE-CHRON-06 | Known answer | M2 | First divergence: the two-tick case reports the earlier tick 100 of 100; a tolerance case with a loose bound returns `unresolved` and then the right interval after refinement; a trace with missing samples returns an interval and a gap. | §29.20 |
| GATE-CHRON-07 | Known answer | M2 | Sampling and histograms: INV-CHRON-05 and -06 hold over 100 permutations, shard splits and co-tenancies on `batch-cpu`, CUDA and Metal; weighted estimates from the traced stratum recover a planted population mean inside the exact interval. | §29.20 |
| GATE-CHRON-08 | Known answer | M2 | The created-energy audit flags every planted trampolining and cone-excess exploit and no honest fixture. Each failure class's monitor fires on its planted fixture, and a numerical fault is never counted as a physical failure or dropped from a denominator. | §29.20 |
| GATE-CHRON-11 | Known answer | M1 (CPU); M2 (device) | Simulation testing: full await-point coverage of the publish and submit paths; every failure minimized to a regression that replays in under one second; dual-run agreement on 100 small jobs. | §29.20 |
| GATE-CHRON-12 | Known answer | M2 | Retention: 10⁴ random collection cycles remove no pinned object; every removal has a tombstone; a citation of a collected object is refused with its reason. | §29.20 |
| GATE-CHRON-13 | Performance | M2 | T0 and T1 on every world cost at most 3% of step time, and T2 on a 1% sample at most 2% more, on the reference quadruped workload with the disclosure block of §36. | §29.20 |
| GATE-CORTEX-01 | Known answer | M2 | PPO on the reference quadruped's flat velocity task reaches the return threshold on `batch-cpu` and on each GPU backend; a zero-weight and a random policy both fail it; the exported checkpoint, reloaded on `batch-cpu`, reproduces the return inside the D3 envelope with a non-zero policy head. | §23.23 |
| GATE-CORTEX-02 | Parity | M2 | On-device backward kernels match FrankenTorch autograd within 10⁻⁵ relative on 10³ random batches. | §23.23 |
| GATE-CORTEX-05 | Performance | M2 | Wall-clock to the return threshold of GATE-CORTEX-01 on one RTX 4090 is at most 1.25 times an incumbent's wall-clock for the comparable task, re-measured on the same card under the disclosure block of §36. Published context: about 410 s. This is parity, not advantage. | §23.23 |
| GATE-CORTEX-06 | Known answer | M2 | Rules E-1 to E-9 each pass their fixture; the final observation is retrievable in both reset modes; the 0 and 9.9 bootstrap fixture; the 500 against 1,000 reward fixture is invariant under rate terms. | §23.23 |
| GATE-CORTEX-07 | Known answer | M2 | The PPO battery P-1 to P-11 passes for our implementation, and three planted defects that reproduce the sibling's errors each fail it. | §23.23 |
| GATE-CORTEX-08 | Parity | M2 | Two-shard and one-device updates agree inside the D3 envelope on 10³ random batches, an empty shard included; the 0.25 and 32/9 fixtures; strong scaling of at least 1.25 times at constant global workload and equal task quality (§36.3 row 5; 1.6 is the ambition and the funding threshold of KILL-SHARD). | §23.23 |
| GATE-CORTEX-09 | Known answer | M2 | The stride fixture reads element 3; the negative-stride view is materialized or refused; a stale-epoch read is refused. | §23.23 |
| GATE-FID-01 | Fidelity | M2 | M2 battery, `batch-cpu` against `referee`: median and 95th-percentile disagreement at most 2% and 5% for base displacement over 2 s, 3% and 8% for energy per metre, 5% and 12% for stance impulse. Re-baselined once at M2 with a written rationale, then frozen. | §16.17 |
| GATE-FID-03 | Fidelity | M2 | The referee's coverage reported per robot class at M2 exit, with the breakdown by refusal cause, the `strict` share and promotion wall-clock at the 50th and 95th percentiles; an adjudicator named wherever coverage is below 90%; coverage below 90% in a reference class falsifies BET-34 for that class, and its fallback applies. | §16.17 |
| GATE-FID-06 | Known answer | M2 | Energy audit: both planted exploits flagged within one episode; an honest policy under CL-1 unflagged; created energy on the standard battery below the frozen bound. | §16.17 |
| GATE-FID-07 | Fidelity | M2 | Background referee: coverage at least 0.1% of GPU world-seconds, reported; GPU throughput loss at most 5%; a planted single-kernel defect localized to its stage from one-step defects; a planted tier-sensitive policy detected within twice the time §16.11 predicts. | §16.17 |
| GATE-HAB-01 | Known answer | M2 | Compiled field against source geometry: error at most half a fine voxel at 10⁶ surface samples and never above the brick bound; analytic shapes exact to the interpolation bound and overlays to round-off; quantization ripple on a resting 10 kg block at most 1% of weight on every tier. | §19.19 |
| GATE-HAB-13 | Known answer | M2 | Brick pool: the pooled field equals the dense field bit for bit at 10⁶ points per scene; measured occupancy and deduplication ratios are recorded for every archetype. | §19.19 |
| GATE-KIN-03 | Parity | M2 | `batch-cpu` in `f64` against `fs-mbd`: kinematics, inverse dynamics, forward dynamics and mass matrix within 10⁻¹⁰ relative on 10⁴ states per reference robot. | §16.17 |
| GATE-KIN-04 | Known answer | M1 (referee); M2 (`strict`) | The transactional rows of §16.16 on `referee` and `strict`: impact time within 10⁻⁹ s; tied sliders exact to 10⁻¹² N·s; zero uncommitted reads in 10⁵ injected faults; forced-retry digests equal; refinement statistics inside 3σ at 10⁶ draws; the redraw mutation caught. | §16.17 |
| GATE-KIN-05 | Known answer | M2 | INV-KIN-04: 1,000 episodes with identical digests solo and in 100 random co-tenancies, capacities and slot assignments, per back end; the path-table Jacobian gives the same digests as the dense one. | §16.17 |
| GATE-KIN-06 | Parity | M2 | A batch of 256 parameter variants of each reference robot equals 256 homogeneous runs bit for bit on `batch-cpu` and on each GPU back end. | §16.17 |
| GATE-KIN-07 | Known answer | M2 | Under the capacities of §16.6, `Degraded` world-steps ≤ 10⁻⁴ of world-steps per class on the standard battery; every forced overflow flagged; capacities printed in every disclosure block. | §16.17 |
| GATE-MARROW-05 | Known answer | M2 | Zero sign errors of `Guarded` predicates against exact rational evaluation over 10⁹ adversarial inputs within 4 ulp of the zero set, on `batch-cpu`, CUDA and Metal; the fallback rate on the standard contact battery recorded (target below 10⁻³ of evaluations). | §13.10 |
| GATE-PHYLUM-02 | Known answer | M2 | Each of 512 designs in a mixed cohort has the same digests as in a homogeneous batch, on `batch-cpu` and per GPU back end, at three bucket sizes and under slot dispatch with 100 random co-tenancies. | §17.19 |
| GATE-PHYLUM-03 | Performance | M2 | The head-to-head of §17.14 run and published with all four arms. For the cohort to remain the engine: at least 0.70 of the harmonic mean of the homogeneous rates (§36.3 row 10) at padding waste of at most 25% on cohorts of at least 256 designs; a lower bound above 1 against arm B on edit to first accepted result at K ≤ 128; at most eight buckets. | §17.19 |
| GATE-PHYLUM-04 | Performance | M2 | Edit-to-first-step latency on the warm and cool paths inside the targets of §17.8 over 1,000 edits from the mutation corpus, cold reported separately, with the same quantity measured on the pinned incumbents. | §17.19 |
| GATE-REFLEX-01 | Known answer | M1 (referee); M2 (`batch-cpu`); M3 (all but tracked and hybrid); M4 | Each archetype stands for 10 s and traverses 5 m of flat ground under its baseline controller: at M1 the first archetypes, on `referee` only, restricted to those whose contact law exists at M1 and whose contact count fits the referee's 21-contact cap (§16.6); at M2 the same set on `batch-cpu`; at M3 all eighteen on `batch-cpu`, except tracked and hybrid archetypes, which follow at M4 with their contact laws. | §22.18 |
| GATE-SAFE-06 | Known answer | M2 | Supply chain: one `asupersync` package per workspace; all lockfiles committed; a kernel cache entry with one flipped byte is refused; `frob-verify` builds with no dependency outside the standard library. | §37.14 |
| GATE-SAFE-07 | Known answer | M2 | Importers: at least 10⁷ fuzz executions per importer with no panic; each limit is reached by a crafted file and refused. | §37.14 |
| GATE-SENS-01 | Known answer | M2 | Depth against analytic scenes within 1 mm plus the brick bound; the three IMU cases of §20.5 and the turntable case; lidar beam geometry and instants match the pattern definition. | §20.21 |
| GATE-SENS-02 | Fidelity | M2 | Sensor outputs, masks and delivered-observation digests agree across backends inside their determinism class. | §20.21 |
| GATE-SENS-03 | Known answer | M2 | Noninterference: all five properties of §20.17 hold on the standard battery; the draw-order control fails. | §20.21 |
| GATE-SENS-04 | Known answer | M1 (CPU); M2 (every tier) | Clock contract: the 400/30 schedule exactly; zero drift against rational arithmetic over 10⁹ ticks; the impulse and arrival cases; identical schedules on every tier. | §20.21 |
| GATE-SENS-05 | Known answer | M1 (CPU); M2 (every tier) | Checkpoints: restore at the three adversarial instants gives byte-equal delivered observations and physics digests over the next 10⁴ ticks on every tier. | §20.21 |
| GATE-SENS-06 | Known answer | M2 | Rolling shutter and beam instants: skew within 0.5 pixel of 19.2; seam displacement within 1 cm of 0.5 m; no fabricated blur across an impact; at most 5% added ray-stage cost. | §20.21 |
| GATE-SENS-12 | Known answer | M1 (CPU); M2 (every tier) | Causal-policy test: the reference actor and every baseline controller pass; each of the five planted leaks fails. | §20.21 |
| GATE-SENS-13 | Known answer | M2 | Docking fixture: 344, 217 and 9 in exact arithmetic on `batch-cpu`, CUDA and Metal with identical bytes; guarded single precision with fallback gives zero final errors; the third-gain amendment reverses every admitted cell. | §20.21 |
| GATE-SINEW-02 | Known answer | M1 (referee); M2 (single precision) | Port energy balance closes to 10⁻⁹ relative on the referee and 10⁻⁴ relative per episode on single-precision tiers. | §15.14 |
| GATE-SINEW-03 | Fidelity | M2 | Batched L1 and L2 kernels agree with the referee components within 10⁻⁴ relative on a 10⁴-case battery. | §15.14 |
| GATE-SINEW-04 | Known answer | M1 (referee); M2 (batched tiers) | The six cases of §15.6: breakaway within one step of τ_s/r; amplitude loss per cycle within 2% of 4·τ_c/k; drift at most ε·t; zero output motion inside the gap; loop width within 1% of β; closure speed and energy loss within 1%. Referee exact; batched tiers within their stated regularization. | §15.14 |
| GATE-SINEW-09 | Known answer | M2 | PO-SINEW-02 holds; with twelve backlash joints the dense factorization has the size it has with none. | §15.14 |
| GATE-SPARK-02 | Fidelity | M2 | Every M2 kernel and the full step agree between `batch-cpu` (`f32`) and each device inside the declared class: bit-equal for class D1; for class D3 at most 4 ulp per kernel output, statistical equivalence of 1,000-step rollouts, and the decision-level test of §35.4. | §21.24 |
| GATE-SPARK-03 | Performance | M2 | CUDA rows of §36.3 inside their tolerances, with the disclosure block and the paired lower bound of §36.6. | §21.24 |
| GATE-SPARK-04 | Performance | M2 | Metal throughput at least 25% of one RTX 4090 at matched settings on this M5 Ultra (BET-09), reported beside an incumbent with a Metal path measured on the same machine. | §21.24 |
| GATE-SPARK-05 | Known answer | M2 | Precision battery: over 10⁶ steps, group-membership error within 4 ulp, closure and contact violations inside their envelopes, energy-ledger residual under 10⁻⁴ relative, at distances up to 1 km from the origin. | §21.24 |
| GATE-SPARK-06 | Known answer | M2 | Cancellation and fault injection: every injected fault yields its typed refusal, no leaked device memory after 10³ cancel cycles, and the drain report is executor-minted. | §21.24 |
| GATE-SPARK-09 | Known answer | M2 | For every stage, withholding its device kernel under a strict request yields the refusal with zero worlds stepped; receipt counters for transfers and synchronizations match an instrumented shim within one count. | §21.24 |
| GATE-SPARK-10 | Performance | M2 | Per-world parameter variants cost at most 5% at 8,192 worlds; 256 topology variants of one bucket run on one captured graph with zero compiles after warm-up; a mixed batch of three body plans reaches at least 0.70 of the harmonic mean of its homogeneous rates; admitting a variant changes no other world's digest. | §21.24 |
| GATE-SPARK-11 | Known answer | M2 | Allocated device bytes are flat over 10⁶ steps; a warm start loads every kernel with zero compiler invocations; each cache control of §21.23 is caught. | §21.24 |
| GATE-SPARK-12 | Known answer | M2 | Zero graph captures after warm-up and zero synchronizations inside a chunk; no pool memory leaked after 10³ cancel cycles; 10⁵ snapshot reads each match the digest emitted for their epoch; the commit-fence known answer holds under 10³ injected faults. | §21.24 |
| GATE-SPARK-15 | Known answer | M2 | 1,000 episodes per reference workload: per-world digests identical alone, in 100 random co-tenancies, under slot permutation, at two bucket capacities and on one and two shards, on `batch-cpu` and each device family; draws depend on address alone; both negative controls of §35.14 caught. | §35.15 |
| GATE-SPARK-16 | Fidelity | M2 | Every class D3 placement pair used as evidence, on at least 4,096 frozen worlds per reference workload: upper bound on discordant decisions at most 1%; net one-sided discordance at most 0.5%; continuous outcomes equivalent inside ledger margins; first divergences recorded. A planted bias of one decision in fifty must fail. | §35.15 |
| GATE-SPARK-18 | Known answer | M2 | PO-NUM-02 holds on each backend; on 10⁴ contact states per reference robot the relative error of the `f32` Newton step against `f64` is at most 10·κ₂·u, and at most 10⁻³ on near-rigid states under the `stiff` profile; κ̂ never exceeds the κ₂ computed in `f64`. | §35.15 |
| GATE-SPARK-19 | Known answer | M2 | The harness rejects a report that violates the unit ordering or omits a decimation; the paired-bound known answer holds; of 100 A/A campaigns at most 9 clear a threshold of 1.02; a figure without a complete disclosure block is refused. | §36.16 |
| GATE-SPARK-20 | Performance | M2 | Weak scaling at least 1.8 and strong scaling at least 1.25 at equal task quality, both as paired lower bounds; the gradient known answer of §36.13 holds and its control is caught. | §36.16 |
| GATE-SPARK-22 | Known answer | M2 | `sim.estimate` memory within 5% of measured high-water marks at 1, 8, 64 and 256 worlds and at capacity for Q and H on each device; a job that does not fit refused before allocation; time estimates inside their intervals on at least 90% of held-out runs; bytes touched per world-step measured per launch. | §36.16 |
| GATE-TETHER-04 | Known answer | M2 | Each of the three failure classes is exercised on both surfaces; every refusal code maps to exactly one exit code; a killed connection during a wait leaves the job recoverable by key in 100 of 100 trials. | §32.21 |
| GATE-TETHER-05 | Known answer | M2 | Idempotency: 1,000 races of two identical submissions and 1,000 with a changed payload each leave exactly one job and one charge; a crash injected at every commit point leaves one job, one charge and a replayed receipt. Negative control: the two-statement variant fails. | §32.21 |
| GATE-TETHER-06 | Known answer | M2 | Pagination: over 10⁴ seeded schedules of concurrent appends, concatenated pages equal the frontier snapshot, including rows with equal timestamps; a changed filter with an old cursor is refused. | §32.21 |
| GATE-TETHER-08 | Known answer | M2 | Job protocol under seeded faults (late lease, overlapping ranges, a reset reusing tick zero, a kill between chunk write and publish, a daemon restart): contiguous frontier; a duplicate chunk reuses its receipt; a conflicting chunk is flagged; terminal counts sum to planned; a cancel names the last committed tick; no device leak. | §32.21 |
| GATE-TRAC-01 | Known answer | M1 and M2 | Every row of §18.16 that tests a Solid mechanism passes on `strict` and on `batch-cpu`, and on `referee` for the rows FrankenSim's solve can express (the list is recorded with the gate); rows for the fast law pass on GPU tiers within the stated creep. | §18.17 |
| GATE-TRAC-03 | Fidelity | M2 | Tunneling audit: on the M2 battery the fast tiers miss fewer than 1 contact per 10⁶ world-steps that the certified detector finds. | §18.17 |
| GATE-TRAC-04 | Known answer | M2 | Cone and joint rows: spin stop, ring co-stop, stiction breakaway and backlash values within 1%; coupled and diagonal forces within 0.5% of μ·N/√2 and μ·N; the pyramid and independent-row mutations fail. | §18.17 |
| GATE-TRAC-06 | Known answer | M1 (referee); M2 (`strict`) | Zero missed hits in 10⁶ randomized thin-barrier crossings under transactional profiles, and the endpoint-only mutation caught; on the sill, work and impulse within 1% of their refined values at h/4 and no couple above 10⁻⁹ N·m in `f64`; the capacity-bias comparison labeled. | §18.17 |
| GATE-TRAC-10 | Known answer | M2 | Scaling: bit-identical solutions with and without S on 10⁵ sampled systems; `f32` against `f64` Newton-step error at most 0.5% at default cards and 5% at near-rigid cards; cards over the bound refused. | §18.17 |

**3. Gates of later subsections due at this exit.**

| Gate | Part due at this exit | Statement (from the defining subsection) | Defined in |
|---|---|---|---|
| GATE-WS-06 | the sampler row | Known answer. The mixed-wrench probe passes and its unconverted mutant fails; the planted sampler is rejected by rule P-12; `xtask check-deps` reports exactly one `serde_json` version per binary and fails a graph that adds `ftx-drape` to the core workspace; the Metal language version and the 64-bit atomic probe are recorded in the fingerprint. M0; the sampler row at M2. | §33.17 |
| GATE-WS-07 | the sampler row | Known answer. Each of the four planted instances raises its named signal in 100 of 100 runs; `xtask check-deps` resolves the graph with every feature group enabled and fails on the planted second pin; no row of the register lacks a planted instance. M0 for the two build rows, M2 for the sampler row, M3 for the suite row. | §41.5 |
| GATE-WS-09 | rows 12 and 14 | Known answer. Each of the six admission tests is recorded at the pin with its result: the roll-up of 31.96314202 kg; the refusal at 33 m and the agreement at 1 m to 10⁻¹², with the closed form returning ∓16.5 m; the signed motion witness for both transformer entry points; the arc fixture; the two `fs-sos` probes compared with exact evaluation; the two-plane ray fixture. A probe that now passes upstream is recorded as fixed and its interim posture is reviewed. M0, with rows 12 and 14 at M2. | §33.18 |
| GATE-TRAC-12 | on the transactional batch profile | Known answer. The arc fixture is detected on 1,000 of 1,000 seeded variants (radius 0.2 to 2 m, angle up to π, sphere radius 1 to 10 cm, an obstacle on the arc and outside the padded chord); the chord-only mutant fails the gate. On 10⁴ random two- and three-joint sweeps the bound of the second item contains 1,000 sampled path points per sweep with no exception. In the tunnelling audit (`contact.audit`) no missed contact is attributed to a broad-phase rejection. M1 on the referee, M2 on the transactional batch profile. | §18.20 |
| GATE-CHRON-14 | on the batch tiers | Known answer. The three fixtures at 32, 64 and 2,000 Hz on `referee` and `strict`: constitutive dissipation of the resting block is exactly 0; sliding heat is within 0.1% of 4 W; impact loss is within 10⁻⁹ J of 2 J; a mutant that feeds algorithmic exchange into the film temperature is caught, because its temperature rise differs between 32 and 64 Hz by a factor of 2. M1 on the referee; M2 on the batch tiers. | §29.22 |
| GATE-KIN-08 | whole | Performance; known answer (determinism) ¦ Digests identical to the masked path on 1,000 episodes per backend under 100 random co-tenancies and queue capacities; overflow to the general lane at most 1% of world-steps on the standard battery; a mixed cohort with queues reaches at least 0.70 of its weighted homogeneous throughput; the 95% lower bound of the paired whole-step ratio against the masked path is at least 1.15 at the batch size named in the receipt. ¦ M2 on CUDA; Metal when its dispatch cost is measured | §16.19 |
| GATE-KIN-11 | whole | Known answer; Parity ¦ The four known answers to 10⁻¹⁰ rad in double precision; reduced-coordinate forward dynamics within 10⁻⁹ relative of the bilateral-row solution on `referee` over 2 s; on the lockstep tiers in single precision the loop residual stays below 10⁻⁶ m for 10⁶ steps; the crossed-branch mutant and a loop driven through its toggle are both refused ¦ M2 | §16.22 |
| GATE-KIN-12 | whole | Known answer; Performance ¦ The ledger reproduces 375, 969, 1,742 and 7,140 and the row counts exactly, and reports a ratio of 1 for a chain. The three-coordinate fill case takes the dense path and is flagged. The two paths pass the decision-level equivalence test of §35 on the standard battery. On the M2 device profile the whole-step gain is at least 1.15 for the 35-coordinate humanoid at 8,192 worlds ¦ M2 | §16.23 |
| GATE-KIN-13 | whole | Known answer. The four rows reproduce exactly in rational arithmetic on the host. In single precision on `batch-cpu`, CUDA and Metal the pass or fail decision of each of the 64 cases equals the rational one for the exact map and for d = 1, 2 and 4. A-22 reports a tier radius of 1.366 at d = 1 and withholds the stability label. The mutant study that halves the control period with the step is refused. M2. | §16.24 |
| GATE-PHYLUM-09 | whole | Performance; known answer (determinism) ¦ Every registered archetype reproduces table-driven digests bit for bit on the pinned corpus; at most 16 archetypes cover at least 90% of the limbs of the design corpus; zero kernel compiles while 1,000 corpus edits are admitted; the 95% lower bound of the paired whole-step ratio against the table-driven kernel is at least 1.15 at 8,192 worlds; both hand controls reproduce. ¦ M2 | §17.21 |
| GATE-SENS-15 | whole | Known answer. The rectangular rows are exact; the swapped-order mutant is rejected; on the referee an impulse with grade `none` yields `overload: unknown` and an `undetermined` peak-load atom in 100 of 100 cases; on a compliant tier whose step resolves the pulse with at least 10 samples the clipped reading is within 2% of the closed form. M2. | §20.23 |
| GATE-SENS-18 | whole | Known answer. The rebound window reads 200 m/s² on the referee to 10⁻⁹ relative, and the adjacent windows read zero; the mutant without the jump term and the closed-closed window mutant both fail. The encoder fixture: +80 and −20 turns per second deliver bit-equal samples, `sense.alias` returns the pair, and the largest unambiguous speed is reported as 50 turns per second at ε = 0; after a planted packet loss the estimate is flagged `ambiguous` in 100 of 100 cases. The load-cell rows read +80.38 N and −23.62 N to 10⁻⁹, and each of the four card mutants (side, sign, origin, tare) changes the reading and is caught. M2. | §20.25 |
| GATE-SPARK-23 | whole | Known answer. On CUDA and on Metal a histogram run of 10¹⁰ increments, concentrated in one bin, matches a CPU `u64` reference in every bin; the boundary plan is refused and its neighbour admitted; the dialect lint finds no 64-bit atomic in any kernel source; the two-limb reduce matches `u64` addition on 10⁶ random pairs. M2. | §21.26 |
| GATE-CORTEX-18 | whole | Known answer. For action dimensions 2, 12 and 29, N = 10⁵ decisions and 64 worlds: every channel-pair, consecutive-decision and adjacent-world sign agreement lies in ½ ± 0.005; every off-diagonal entry of the standardized covariance is below 4/√N in magnitude; the planted linear-phase sampler is rejected with an agreement of 0.6655 ± 0.005. M2. | §23.25 |
| GATE-CORTEX-19 | whole; adapter rows at the adapter's release | Known answer. Each of the four planted leaks fails `sense.causal-test` in 100 of 100 seeds; the docking exclusion holds with reward hidden and flips with reward delivered, on `VecEnv` and on the adapter; a contract change alters the `TaskId` and invalidates the cached exclusion. M2; adapter rows at its release. | §23.26 |
| GATE-CORTEX-20 | whole | Known answer. In 2,000 simulated trainings of an unchanged policy at true success 0.89, with 50 looks of 30 episodes, the first-crossing rule reports attainment in 78.5% ± 3 points and the spending rule in at most 5% (exact one-sided test at the 1% level); the four episode counts above are reproduced exactly. M2. | §23.27 |
| GATE-ARENA-10 | whole | Known answer. The cone program reproduces 52.5 N, 0.55 N·m, 25 N and 40 N to 10⁻⁹ in double precision; the two-point rig slips within 2% of each value on the referee and inside the fidelity envelope on the batch tiers; a scorecard that ranks the two soles by moment-free capacity when the task commands zero yaw is rejected by `task.check`. M2. | §24.19 |
| GATE-ARENA-14 | first part | Known answer; Design comparison ¦ Fixture: a slick strip over x ∈ [2.0, 2.5] m (`field`, per 0.25 m tile), a push at t = 3 s (`process`), a push at the first crossing of a gate at x = 5 m (`event`), a baseline controller and one retimed from a 0.5 s to a 0.4 s gait period. With 1,000 unused draws inserted, the step halved and a limb added: tile values bit-identical; the timed push at 3 s in every arm; the gate push at each arm's crossing; per-arm outcomes indistinguishable from independent sampling in a two-sided test at level 0.01 on 4,096 worlds. The tick-keyed and footfall-keyed controls are refused. On a held-out slip-and-dock battery the variance of the contrast is at most half that of named streams keyed per stream and tick, at equal count ¦ M2 for the fixture; M3 for the battery | §24.22 |
| GATE-HAB-16 | whole | Known answer. The two sphere points return clearances of 0.10 m and 0.20 m after compilation, and the pre-compilation query returns the interval of rule 1 and never the local value. The two-plane fixture returns the foreground at 2 m through the compiled pack, through the viewer's pick and through the adapter; the mutant that sorts local parameters returns the background and fails. On 10³ random instances with anisotropic scales in [0.25, 4], depth against an analytic plane is within 1 mm plus the brick bound. M2. | §19.22 |
| GATE-GAUNT-03 | first run; then every exit | Known answer ¦ Each relation of §38.7 holds on 10⁴ random scenarios per tier it names per release, and each registered mutation breaks its relation (PO-GAUNT-02) ¦ M2, then every exit | §38.15 |
| GATE-GAUNT-06 | kernels | Known answer ¦ Chaos: every row of §38.9 ends in its typed refusal or an accepted state with no leaked resource; 10⁵ seeded schedules per release; every await point on the publish and submit paths hit ¦ M2 for kernels; M3 for the daemon | §38.15 |
| GATE-GAUNT-07 | whole | Known answer ¦ Determinism: every kernel and every verb meets its declared class twice on one machine, across worker counts, in debug and release, across platforms and across backends; MR-01 holds on 10⁴ worlds per device family ¦ M2 | §38.15 |

**4. Program-level items.** KILL-TENFOLD-LEVER is re-read on our own profile: a mechanism whose removable cost is
under 10% of the measured loop is not funded as a speed lever. GATE-TENFOLD-09 recurs. GATE-GROUND-01, GATE-SAFE-01
and GATE-GAUNT-01 recur; GATE-GAUNT-03 (metamorphic relations) first runs here.

**5. Kill gates decided here, each recorded as an ADR.** In brief: KILL-KIR → hand-written arithmetic in Rust and
the dialect, bound by differential tests. KILL-CONTACT → CL-2 as the default at a published lower throughput; then
locomotion claims on the referee only. KILL-METAL → Metal becomes the development and replay tier, and the Mac
trains on `batch-cpu`. KILL-PHYLUM, with KILL-BUCKET → per-family templates become the engine; parameter variants
are unaffected. KILL-D1 → the family is class D2 for itself and D3 against others, and a teleported world is a
tolerance replay. KILL-PARITY → feature work on the device tier stops until the row is inside tolerance.

| Kill gate | Trigger | Fallback, named in advance | Decided | Owner (consumer) |
|---|---|---|---|---|
| KILL-MARROW-GUARD | Fallback rate above 10⁻² on the contact battery on either GPU backend, or the exact path cannot be made bit-identical across backends | Strict predicates are evaluated exactly only on `strict` and the referee; GPU tiers mark worlds that came within E of a boundary and the sampled referee re-runs them | M2 | §13.10 |
| KILL-SINEW-ROWS | With stiction rows on, the fast-tier solve needs more than 1.5 times the Newton iterations of the smooth-friction model on the contact battery, or throughput falls by more than 20%, after one repair cycle | Throughput tiers apply regularized friction as a force; the rows stay on the referee and `strict`; smooth-model results carry a flag and the fidelity ledger measures the gap | M2 | §15.14 |
| KILL-PHYLUM | GATE-PHYLUM-03 fails: below 0.5 of homogeneous throughput on 512 designs, more than eight buckets, or arm B covers at least 95% of the mutation corpus and is faster | Per-family templates become the engine; cohorts are formed by exact tree with per-design parameter tables; the table-driven path remains for admission and preview on `batch-cpu` | M2 | §17.19 |
| KILL-CONTACT | GATE-FID-01 fails on slip or creep after parameter mapping and one repair cycle | CL-2 becomes the default at a published lower throughput; if that also fails, locomotion claims are restricted to the referee tier and the fast tiers are labeled exploratory | M2 | §18.17 |
| KILL-POOL | Measured deduplication on the archetype battery below 1.5 | Dense per-variant fields behind the same directory interface | M2 | §19.15 |
| KILL-EXPOSURE | Exposure integration costs more than 25% of the ray stage, or changes no mask statistic beyond noise | Midpoint sampling | M2 | §20.21 |
| KILL-SCHED | The race check misses a planted race | The fixed total order, with no enumeration | M2 | §20.21 |
| KILL-BUCKET | More than eight buckets are needed to hold Ω₃ ≤ 2 on the reference family corpus, or the mixed batch of GATE-SPARK-10 runs below 0.5 of the harmonic mean of its homogeneous rates after one optimization pass | At most nine padded per-family templates, each a homogeneous environment, scheduled in turn; parameter variants are unaffected | M2 | §21.11 |
| KILL-D1 | GATE-SPARK-01 or GATE-SPARK-02 shows that a device family cannot hold bit equality with `batch-cpu` for the floating kernels of the step | That family is class D2 for itself and D3 against others, with the decision-level test of §35.4; a teleported world is a tolerance replay; cross-family comparisons are unpaired; the replay claim of §6 and §29 is reworded | M2 | §21.15 |
| KILL-KIR | At M2 the derivation toolchain cannot express or emit more than 20% of the kernel arithmetic, or emitted kernels miss BET-01's speed bound after one optimization pass | Arithmetic is hand-written in Rust and the dialect and bound by differential tests; certificates fall back per KILL-LEMMA | M2 | §21.6 |
| KILL-METAL | GATE-SPARK-04 fails after one optimization pass, or the Metal backend cannot hold run-to-run determinism | Metal becomes a development and replay tier; CUDA is the training tier; the Mac trains on `batch-cpu` | M2 | §21.24 |
| KILL-PARITY | At M2 exit, after one profile-guided optimization pass, the paired lower bound of a physics row (1, 7 or 8 of §36.3) is below 0.33 of the incumbent on the same card | Feature work on the device tier stops until the row is inside tolerance; the arithmetic of §2 and §39 is redone with measured rates before the partial gate, where shipping the layer is decided. Between 0.33 and 0.67: a recorded defect, one further pass, a disclosure on every Performance claim | M2 | §36.16 |
| KILL-SHARD | After one optimization pass the paired lower bound of strong scaling at equal task quality is below 1.6 | No further sharding work is funded; the two cards run as independent job lanes; the tolerance of 1.25 still applies where synchronous sharding is used | M2 | §36.6 |
| KILL-LEARNKERN | GATE-CORTEX-02 fails after one repair cycle, or GATE-CORTEX-05 is missed by more than two times after one optimization pass | The forward pass stays on the device; rollout storage is read back once per rollout and the learner runs on FrankenTorch on the CPU, with the lost throughput disclosed | M2 | §23.23 |

| Kill gate | Trigger and fallback (from the defining subsection) | Decided | Gate beside it | Defined in |
|---|---|---|---|---|
| KILL-ROWQUEUE | Trigger: the pilot's falsifier holds; or the measured lower bound is below 1.15; or h ≥ f·(1 − ℓ) on the measured profile; or a digest mismatch survives one repair cycle. Fallback: the masked lockstep solve of §16.6 alone, with the ℓ counters kept in the disclosure block of §36.7. | M2 | GATE-KIN-08 | §16.19 |
| KILL-LOOPELIM | Trigger: The parity part of GATE-KIN-11 fails on any archetype loop after one repair cycle; or the recognizer matches fewer than half of the closures in the archetype library. Fallback: Compliant equality rows on the lockstep tiers and bilateral rows under the transactional profiles, as in §16.12. | M2 | GATE-KIN-11 | §16.22 |
| KILL-ANCESTOR | Trigger: The pilot's falsifier holds; or the device gain is below 1.15 on the humanoid and below 1.10 on the quadruped; or more than 20% of battery world-steps take the dense path. Fallback: The dense path for 18 to about 60 coordinates as in §16.6; the tree route only where KILL-TREEOP leaves it. | M2 | GATE-KIN-12 | §16.23 |
| KILL-LIMBKERNEL | Trigger: the pilot's falsifier holds; or coverage is below 80% with 16 archetypes; or the gate's ratio fails after one repair cycle. Fallback: the table-driven kernel for every design, per-design specialization for survivors under §17.9's own gate, and the limb port interface kept for the tree-operator route and the per-limb caches of §14.6. | M2 | GATE-PHYLUM-09 | §17.21 |

**6. Exit rule.** Recorded by a fresh-context session (seed BD-224, procedure N40-05). The exit line: *the kill
decisions for the kernel toolchain, contact, Metal and mixed batches recorded as ADRs* (KILL-KIR, KILL-CONTACT,
KILL-METAL, KILL-PHYLUM, KILL-BUCKET).

**7. Dependency firewall.** Needs FrankenSim (parity oracle), asupersync (device jobs), `cudarc`, FrankenTorch
(Metal backend, gradient oracle), FrankenSymPy (derived kernels), franken_numpy (small-matrix oracle). If a
foundation is missing: the no-GPU variant; KILL-KIR; KILL-METAL.

**8. Owner input wanted.** OPEN-SPARK-1 falls due at this exit (default: GPU results are labelled exploratory; no
Performance or Parity claim rests on a device result; M2 exits in its no-GPU variant). OPEN-PHYLUM-3 for the
head-to-head (default: the one-device, one-hour rule, which admits the pilot and not the full experiment;
KILL-PHYLUM is then decided on the pilot alone). OPEN-PROG-2 (the adjudicating simulator and neutral party;
default: a pinned incumbent CPU engine that neither arm trains in, run by a fresh-context session; every ratio
labelled `self-adjudicated`) and OPEN-ARENA-3 (who holds `qual:admin`, and the sealed families; default: the owner
alone opens and closes campaigns; one sealed family per task family, sized for 20 requests at 256 clusters).
OPEN-CHRON-4 (retention; default: logs and keys 30 days, failure captures 30 days in 64 GB, retired confirmation
clusters sealed indefinitely).

**9. Stop or re-plan.** Stop feature work on the device tier, and redo the thesis arithmetic with measured rates,
if a physics row is below 0.33 of the incumbent after one optimization pass (KILL-PARITY). Re-plan the legged
strata of the partial gate if both fallbacks of KILL-CONTACT are exhausted. Name an adjudicator, or restrict claims
to the classes the referee covers, if GATE-FID-03 reports coverage under 90% for a reference class. Fall back to
the no-GPU variant if no device family holds batch-composition invariance.

**10. The no-GPU variant (DESIGN; bead N40-33).** M2 can exit without a device. It applies if OPEN-SPARK-1 is not
ratified, if INT-CUDA-01 and INT-FT-01 both fail, or if no device family holds a controlled class. Everything above
lands on `batch-cpu`, on the 36 cores of the M5 Ultra and on the CPU of `trj`; device gates are recorded `not run`
with the reason; NS-10 and the device halves of NS-02 and NS-14 are not demonstrated.

**11. No single failed bet blocks the exit.** OPEN-SPARK-1 unratified or both device families fail → the no-GPU
variant. BET-18 (mixed batches lose) → per-family templates with per-world parameter variants. BET-09 (Metal is not
a rollout tier) → CUDA trains; the Mac keeps the world bank, proof, replay and `batch-cpu`. BET-34 (the referee
covers too little) → the `strict` profile as named adjudicator, with results that say so.

**12. Work packages.**

| Package | Delivers | Crates | Milestones | Key gates |
|---|---|---|---|---|
| WP-15 | Kernel library and batched step | `frob-kernels`, `frob-dyn` | M2 | GATE-KIN-03, -05, -07 |
| WP-16 | Traction, batched | `frob-contact` | M2 | GATE-TRAC-01, -03 |
| WP-17 | Scene packs and terrain | `frob-scene` | M2 | GATE-HAB-01, -13 |
| WP-18 | CUDA backend | `spark/frob-spark-cuda` | M2 | GATE-SPARK-02, -03, -06 |
| WP-19 | Metal backend | `spark/frob-spark-metal` | M2 | GATE-SPARK-02, -04, -06 |
| WP-20 | Sensors I | `frob-sense` | M2 | GATE-SENS-01, -02, -03 |
| WP-21 | Cortex I: `VecEnv` and PPO on device | `frob-learn` | M2 | GATE-CORTEX-01, -02, -05 |
| WP-22 | Trust ladder and replay | `frob-dyn`, `frob-chronicle` | M2 | GATE-FID-01; GATE-CHRON-01, -02; GATE-SPARK-05 |
| WP-44 | Phylum: variants, buckets, cohorts and the head-to-head [S], [F] | `frob-phylum`, `frob-kernels` | M2 | GATE-PHYLUM-02, -03, -04; GATE-KIN-06; GATE-SPARK-10 |
| WP-45 | Contact cone, joint rows and scaling | `frob-contact`, `frob-actuate`, `frob-kernels` | M2 | GATE-TRAC-04, -10; GATE-SINEW-03, -04, -09; GATE-SPARK-18 |
| WP-46 | Determinism, placement and device services; the fleet scheduler and survivor specialization [F] | `frob-kernels`, `frob-kir`, the Spark annexes, `frob-cli` | M2, M4 | GATE-SPARK-09, -11, -12, -15, -16; GATE-MARROW-05; GATE-SPARK-13, -14 |
| WP-47 | Performance harness and parity gates | `frob-conformance` | M2 | GATE-SPARK-19, -20, -22 |
| WP-48 | Gym semantics, the PPO battery and the two-device update | `frob-learn` | M2 | GATE-CORTEX-06, -07, -08, -09 |
| WP-49 | Sensors II and the sensing analyses | `frob-sense`, `frob-analyze` | M2, M3 | GATE-SENS-06, -12, -13; GATE-SENS-07 to -11, -14 |
| WP-50 | Fidelity ledger, sampled referee, energy audit, tier correction and model-form checks [S], [F] | `frob-dyn`, `frob-chronicle`, `frob-opt` | M2, M3 | GATE-FID-03 to -08; GATE-CHRON-08, -09; GATE-CORTEX-15 |
| WP-51 | Replay, teleport and forks | `frob-chronicle` | M2 | GATE-CHRON-05, -06, -07, -12; GATE-AUTOPSY-01, -02, -07 |
| WP-52 | Viewer, stills and dashboard | `viewer/`, `frob-tui`, `frob-chronicle` | M2, M3 | GATE-TETHER-14; GATE-CHRON-13; INT-FTUI-01 |

## Acceptance criteria

- [ ] Every part in the gate tables has a recorded value and a caught negative control, or is recorded `not run`
      under the no-GPU variant with its reason.
- [ ] The fourteen kill gates of the compiled table and the four late ones each have an ADR with an outcome; the
      five named in the exit line are among them.
- [ ] NS-02, NS-10 and NS-14 run from a clean checkout through `frob --json` and `frob-mcp`, or are recorded as not
      demonstrated under the variant.
- [ ] Every throughput and training-time figure carries its disclosure block and the incumbent figure measured on
      the same card.
- [ ] GATE-FID-01 is re-baselined at most once at M2, with a written rationale, then frozen.
- [ ] The exit report for M2 is `exit-ready`; the exit record is written by a fresh-context session.

## Test plan

### Unit tests

Carried by member beads. Epic check: due counts for M2 are 2 admission, 80 convergence, 14 kill decisions, plus 22
late parts and 4 late kill decisions.

### End-to-end script and logging

`scripts/exit/review.sh M2`: the three demonstrations on `trj` and on the Mac (`frob train start` to the return
threshold; `frob sim replay --world 1337 --tier batch-cpu`; `frob phylum sweep` over 256 designs with the digest
comparison), each through CLI and MCP, logging the evidence block (determinism class, tier, identities, seed),
digests compared, throughput with its disclosure block and thresholds, and the gates touched.

### Known answers and negative controls

Bit-equal digests solo and in 100 random co-tenancies; 256 homogeneous runs equal to one batch of 256 variants;
the 400/30 clock schedule; 344, 217 and 9 on the docking fixture in exact arithmetic. Controls: a zero-weight and a
random policy fail the return threshold; a planted single-kernel divergence is localized; a planted bias of one
decision in fifty fails the D3 equivalence test; the endpoint-only tunnelling mutant is caught.

## Considerations

- M2 has the largest single-exit load of convergence parts (80). The exit runner's control sweep is sharded.
- Most Performance gates exceed the default compute envelope; without approval the single-card rows are recorded
  and GATE-SPARK-20 is `not run`.
- A device result recorded before OPEN-SPARK-1 is ratified is labelled exploratory and supports no claim.

## References

Plan §43.5, §43.1, §43.2, §40.3 to §40.6, §43.12. Bets BET-01, BET-02, BET-03, BET-08, BET-09, BET-18, BET-34.
Scenarios NS-02, NS-10, NS-14. Work packages WP-15 to WP-22, WP-44 to WP-52.
