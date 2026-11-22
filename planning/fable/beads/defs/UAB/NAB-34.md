---
key: "NAB-34"
title: "killed-idea register KI-10 to KI-16: memory, forks, telemetry, scheduling (copy-on-write, sketches)"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "delivery", "chronicle", "spark", "solid", "registry", "decision-record"]
parent: "EPIC-M0"
deps: ["NAB-32"]
needs: ["§35.5: the batch-composition invariance test, which is the standing test any alternative telemetry summary must pass (BD-181)", "§16.11: the sampled background referee and its coverage gate GATE-FID-07 (BD-214, BD-268)"]
related: ["NAB-33", "NAB-35", "NAB-41", "NAB-20", "NAB-25", "N19c-33", "BD-164", "BD-203", "BD-205", "BD-207", "BD-214", "BD-349"]
gates: []
plan_refs: ["Appendix C.2.2", "§0.8 (H-11)", "§21.20", "§29.7", "§35.5", "§35.7"]
---
## Background

FrankenRobotics keeps every idea that was proposed and then refuted or deflated in a register
(`registries/killed_ideas.toml`, rendered as `docs/decisions/killed-ideas.md`; schema and revive lint in
NAB-32), each with the strong form as proposed, the refutation with its numbers, what survives, and the
condition under which it would be revived. A proposal that matches an entry must cite it.

This bead carries the second group, seven entries about **memory, forks, telemetry and scheduling**. They share
a pattern: a technique that is a real win in systems with sparse writes, large state or wall-clock deadlines
(copy-on-write, mergeable sketches, perfect hashing, zero-copy sharing, an always-on shadow, feedback control of
effort, incremental replay) is transplanted into a batched simulator whose state is small, write-dense and
required to replay bit for bit. In each case the arithmetic shows either that the bottleneck is elsewhere or
that the technique breaks a determinism property the system promises: a world's trajectory may not depend on
batch size, slot, co-tenant worlds, shard count or machine load.

Reading conventions: a number inside a *strong form* is the proposal's own; FACT figures have source-register
records and are re-read or re-run before a gate cites them; "DESIGN arithmetic" can be redone by hand; ranges
in *survives* lines are HYPOTHESES until the named gate records a value; thresholds are TARGETs. Tier of the
bead: Solid (a decision record).

## Scope

In: records `KI-10` to `KI-16` with calculations, keywords, carriers and revival conditions. Out: the schema,
harness and lint (NAB-32); the surviving mechanisms (their carriers).

## Technical approach

**Transcription rule.** As in NAB-33: one `[[idea]]` per entry; the three paragraphs verbatim; the last two
lines of each entry are `revive_only_if` and `carriers`; identifiers in the *survives* paragraph fill
`sections`, `gates`, `kill_gates`, `bets`.

**Machine-checked arithmetic of this group.**

| Record | Expression | Value |
|---|---|---|
| KI-10 | size of a full fork copy: 8,192 worlds × 30 kB | 0.246 GB |
| KI-10 | one batch step at 2.0 M world-steps/s: 8,192 ÷ 2.0 × 10⁶ | 4.1 ms |
| KI-10 | read plus write traffic of that copy at 1,008 GB/s: 0.49 GB ÷ 1,008 GB/s | 0.49 ms, one eighth of the step |
| KI-11 | two-centroid sketch fed 0, 3, 5, 9 | (8/3 with weight 3; 9 with weight 1) |
| KI-11 | the same sketch fed 0, 9, 5, 3 | (1.5 with weight 2; 7 with weight 2) |
| KI-12 | dense directory of 8³-voxel bricks, 4-byte slots, 10 × 10 × 3 m at 2 cm: 63 × 63 × 19 slots | 75,411 slots, 0.30 MB |
| KI-12 | 60 × 40 × 8 m at 4 cm: 188 × 125 × 25 slots | 587,500 slots, 2.35 MB |
| KI-12 | the same warehouse at 2 cm: 375 × 250 × 50 slots | 4,687,500 slots, 18.8 MB |
| KI-12 | directory as a share of the dense 16-bit field it indexes: 4 ÷ (512 × 2) | 0.39% |
| KI-13 | shadowing 256 humanoid worlds of 540 bytes at 50 Hz | 6.9 MB/s |
| KI-14 | episodes per tier to detect a five-point gap in success rate: 16·p·(1 − p)/δ² at p = 0.9 and p = 0.5 | 576; 1,600 |
| KI-16 | the proposed tile-reuse factor 1/(0.05 + 0.019 + 0.01) | 12.7 |

The sketch example is the hand-checkable counterexample of KI-11, step by step (the sketch keeps at most two
centroids and merges the closest pair). Order 0, 3, 5, 9: after 0, 3, 5 the closest pair is (3, 5), merged to 4
with weight 2; after 9 the closest pair is (0, 4), merged to (0 + 2·4)/3 = 8/3 with weight 3; result (8/3; 9).
Order 0, 9, 5, 3: after 0, 9, 5 the closest pair is (5, 9), merged to 7 with weight 2; after 3 the closest
pair is (0, 3), merged to 1.5 with weight 2; result (1.5; 7). The same data give different sketches.

FACT figures and their sources: the vendor documentation behind KI-13 (shared storage must be quiescent before
the other processor reads it; device writes are visible only when a command buffer completes) is a public
source read from its primary page; the memory bandwidth of 1,008 GB/s used for KI-10 is the performance
chapter's planning figure for the RTX 4090 and is on the not-verified list until the hardware fingerprint
measures it.

**Keywords for the revive lint.** KI-10: copy-on-write, COW fork, lazy fork of device state. KI-11: t-digest,
quantile sketch, mergeable sketch, KLL. KI-12: minimal perfect hash, succinct directory, compressed brick
directory. KI-13: zero-copy read, unified-memory live read, read device state without snapshot. KI-14:
always-on referee, shadow every world. KI-15: PID on solver effort, adaptive iterations from step time,
wall-clock feedback. KI-16: causal tile reuse, incremental replay, recompute only dirty tiles.

**The entries.**

**KI-10. Copy-on-write forks on device**
- *Strong form.* Fork device world state by copy-on-write, so that a fork costs nothing until it diverges.
- *Refutation.* State is write-dense: every step rewrites every world's configuration, velocity, contacts and
  warm starts, so a fork has copied everything after one step and pays an indirection on every read. DESIGN
  arithmetic: the whole copy is small. 8,192 worlds × 30 kB = 0.25 GB, at most about a millisecond at the memory
  bandwidth of an RTX 4090, and so at most a quarter of one 4.1 ms batch step at 2.0 M world-steps/s.
- *Survives.* Fork by indexed copy of the chosen rows into free slots (`K-snapshot`), with warm template
  environments kept resident (§27.3, §29.8).
- *Revive only if.* A measured fork copy that costs more than a stated share of one batch step on our own profile. The recomputed figure is 0.49 GB of traffic, about 0.5 ms at 1,008 GB/s, an eighth of a 4.1 ms step.
- *Carried by.* BD-203 (`K-snapshot`), BD-205 (forks by indexed copy).

**KI-11. Order-dependent quantile sketches**
- *Strong form.* Mergeable quantile sketches for telemetry quantiles across shards and devices.
- *Refutation.* Hand-checkable: a two-centroid sketch that merges its closest pair, fed {0, 3, 5, 9}. In the
  order 0, 3, 5, 9 it ends as (8/3 with weight 3; 9 with weight 1). In the order 0, 9, 5, 3 it ends as (1.5 with
  weight 2; 7 with weight 2). The same data give different sketches, so the statistic depends on shard count and
  arrival order, which the batch-composition invariance of §35.5 forbids.
- *Survives.* Integer log-bin histograms (`K-x-hist`): bins from the bit pattern, integer counts, merge by
  addition (§29.7, §35.7).
- *Revive only if.* Only a summary whose merge is bitwise independent of arrival order and shard count, shown by the batch-composition invariance test, may replace the integer histogram.
- *Carried by.* BD-207 (`K-x-hist`), BD-181.

**KI-12. Minimal perfect hashing or succinct structures for brick directories**
- *Strong form.* Compress the directory of scene bricks with a minimal perfect hash or a succinct structure.
- *Refutation.* DESIGN arithmetic: a dense directory of 8³-voxel bricks with 4-byte slots is 63 × 63 × 19 = 75,411
  slots, 0.3 MB, for a 10 × 10 × 3 m interior at 2 cm; 587,500 slots, 2.3 MB, for a 60 × 40 × 8 m warehouse at
  4 cm; and 18.8 MB for that warehouse at 2 cm. That is about 0.4% of the dense 16-bit field it indexes, and a
  hash adds a second, verifying read to every lookup.
- *Survives.* A dense directory per scene variant over a shared brick pool (§19.15; KILL-POOL).
- *Revive only if.* A measured dense directory that threatens the device memory budget at the admitted scene sizes, recorded with the pool gate (GATE-HAB-13, KILL-POOL).
- *Carried by.* BD-164.

**KI-13. Live zero-copy reads of device state by the CPU**
- *Strong form.* On unified memory the CPU reads live GPU state in under a microsecond with no copy.
- *Refutation.* FACT (vendor documentation): shared storage must be quiescent before the other processor reads
  it, and device writes are visible only when a command buffer completes. DESIGN arithmetic: the data are tiny. A
  humanoid's integration state is about 540 bytes, so shadowing 256 worlds at 50 Hz moves 6.9 MB/s.
- *Survives.* Fenced snapshots on any backend (rule R1 of §21.9; `K-snapshot`), which also serve the two-card
  machine by explicit read-back.
- *Revive only if.* Vendor documentation that permits a read of non-quiescent shared storage, and a measured need that fenced snapshots do not meet.
- *Carried by.* BD-028, BD-174, BD-203.

**KI-14. An always-on referee at zero copy cost** (H-11, NARROWED)
- *Strong form.* Idle CPU cores shadow GPU worlds with the referee at zero copy cost and catch tier-sensitive
  behaviour during training at no more than 5% of throughput.
- *Refutation.* Coverage is the ratio of referee rate to device rate: about 0.1 to 2.4% of GPU world-seconds on
  any backend if the referee runs at 0.2 to 1.0 times real time per core on 24 cores, and more if it is faster
  (HYPOTHESIS, §0.8; measured at M2); unified memory is not the lever (KI-13). DESIGN arithmetic:
  detecting a five-point gap in success rate needs n ≈ 16·p·(1 − p)/δ² episodes per tier, 576 at p = 0.9 and
  1,600 at p = 0.5. The mechanism adds cost and removes no training or search.
- *Survives.* A sampled background referee with a reported coverage figure, as a correctness guard (§16.11;
  GATE-FID-07 at coverage of at least 0.1% and a throughput loss of at most 5%; KILL-SHADOW; BET-34).
- *Revive only if.* The strong form (catching tier-sensitive behaviour during training at no more than 5% of throughput) is revived only if the measured referee rate gives coverage that reaches n ≈ 16·p·(1 − p)/δ² episodes per tier inside one training run. The surviving form stands or falls at GATE-FID-07.
- *Carried by.* BD-214, BD-268, N16b-19 (KILL-SHADOW).

**KI-15. Feedback control of solver effort on wall-clock**
- *Strong form.* A proportional-integral-derivative or model-predictive controller reads measured step time and
  adjusts solver iterations and effort profiles.
- *Refutation.* Wall-clock feedback into iteration counts makes a result depend on machine load. The same world
  address run on a loaded and on an idle machine executes different iteration counts and yields different
  digests, so replay under classes D1 and D2 breaks and paired statistics are refused. No calculation is needed:
  one such pair is the counterexample.
- *Survives.* Effort profiles are model identities, chosen by a guard table with hysteresis that reads solver
  residuals and never time (DC-08; §28.19). The lockstep iteration count is a constant of the shape class
  (§16.6).
- *Revive only if.* Never on a replayable profile. One pair of runs at one world address on a loaded and an idle machine with different digests is the standing counterexample.
- *Carried by.* BD-349 (the DC-08 guard table), BD-296.

**KI-16. Causal tile reuse across replays**
- *Strong form.* Reuse the unchanged tiles of a replay, for a factor near 12.7 if 95% of the replay is reusable.
- *Refutation.* The robot's neighbourhood dominates the compute and is always dirty. The 95% premise holds for
  far-field state that costs almost nothing to recompute.
- *Survives.* Sharing of static far-field fields through the brick pool (§19.15).
- *Revive only if.* The far-field pilot measures a reusable share whose recomputation cost is material (GATE-HAB-15, KILL-FARFIELD). The robot-neighbourhood form is not revived.
- *Carried by.* BD-164, N19c-33 to N19c-37.

## Acceptance criteria

- [ ] Records `KI-10` … `KI-16` exist with the three paragraphs verbatim, revival conditions, carriers and
      keywords.
- [ ] The harness recomputes the twelve calculation rows; the sketch counterexample is executed by a test
      that implements the two-centroid sketch and feeds it both orders.
- [ ] The revive lint fails a planted bead "Use a t-digest for telemetry quantiles across shards" with
      `ki.match-without-citation` naming KI-11, and passes "Integer log-bin histograms `K-x-hist` (replacing
      order-dependent sketches, KI-11)".
- [ ] KI-14's record states both forms: the refuted strong form and the surviving sampler with its gate
      (coverage of at least 0.1% and a throughput loss of at most 5%).
- [ ] KI-10's record carries the recomputed copy cost (about 0.5 ms, an eighth of the step) beside the
      original upper bounds (about a millisecond, a quarter of the step).

## Test plan

### Unit tests

`crates/frob-conformance/tests/killed_ideas.rs`, module `g2_memory`:

- `ki10_copy_arithmetic`: 8,192 × 30,000 B = 245.76 MB; 8,192/2.0e6 = 4.096 ms; 2 × 0.2458/1,008 = 0.488 ms;
  0.488/4.096 = 0.119.
- `ki11_sketch_order_dependence`: a 30-line two-centroid sketch returns `[(8/3, 3), (9, 1)]` and
  `[(3/2, 2), (7, 2)]` for the two orders (exact rationals), and the two results differ.
- `ki12_directory_sizes`: ⌈500/8⌉ = 63, ⌈150/8⌉ = 19; ⌈1500/8⌉ = 188, 1000/8 = 125, 200/8 = 25; 3000/8 = 375,
  2000/8 = 250, 400/8 = 50; products and sizes as in the table.
- `ki13_bandwidth`: 540 × 256 × 50 = 6,912,000 B/s.
- `ki14_sample_sizes`: 576 and 1,600.
- `ki16_factor`: 1/0.079 = 12.66.
- `records_complete_and_verbatim` against `tests/fixtures/killed/g2.md`.

### End-to-end script and logging

`scripts/e2e/decision_records.sh` (NAB-50) recomputes the calculations, runs the sketch counterexample and the
two planted revive cases. Logged per STD-E2E.

### Known answers and negative controls

Known answers: the table and the two sketch results. Negative controls: a "sketch" that sorts its input first
(order-independent; the test must then report equality, proving the test distinguishes); a record of KI-12
that states the directory is 4% of the field; a proposal to adjust solver iterations from measured step time
"only in training" (must match KI-15 and be routed to the labelled training profile). Metamorphic relation
for the surviving histogram, stated here because it is the property the killed sketch lacked: merging
histograms in any order and any sharding gives identical integer counts.

## Considerations

- KI-16's far-field half was later given a pilot of its own (field tiles keyed by event digests for
  multi-room film and steam fields, at M6). The robot-neighbourhood form stays killed: the neighbourhood
  dominates compute and is always dirty.
- KI-15 has no calculation because one counterexample suffices: the same world address on a loaded and an
  idle machine would execute different iteration counts. The surviving guard table reads solver residuals and
  never time.
- The bandwidth figure in KI-10 is unverified; the conclusion does not depend on it within a factor of several.

## References

Appendix C.2.2; §0.8 row H-11; §21.20; §29.7; §35.5; §35.7.
