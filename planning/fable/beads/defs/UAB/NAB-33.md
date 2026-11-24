---
key: "NAB-33"
title: "killed-idea register KI-01 to KI-09: arithmetic, precision, kernels (half precision, fast-math, megakernels)"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "delivery", "spark", "gauntlet", "solid", "registry", "decision-record"]
parent: "EPIC-M0"
deps: ["NAB-32"]
needs: []
related: ["NAB-34"]
gates: []
plan_refs: ["Appendix C.2.1", "§0.8 (H-04, H-04b, H-15)", "§21.20", "§35.6"]
---
## Background

FrankenRobotics keeps every idea that was proposed and then refuted or deflated in a register
(`registries/killed_ideas.toml`, rendered as `docs/decisions/killed-ideas.md`; schema and revive lint in
NAB-32), each with the strong form as proposed, the refutation with its numbers, what survives, and the
condition under which it would be revived. A proposal that matches an entry must cite it.

This bead carries the first group, nine entries about **arithmetic, precision and GPU kernels**. They are the
ideas a performance-minded engineer reaches for first: factor in half precision on tensor cores, specialize a
kernel per design, saturate rewrites, switch on fast-math, relax ordered reductions, fuse the whole step into
one kernel, erase algebra at compile time, tile time per world. Each was examined against one of two facts:
the determinism the system promises (a world replays bit for bit on a CPU twin; the CPU, CUDA and Metal
kernels of one source agree), and where time actually goes in a batched step (an incumbent's trace of a
humanoid at 8,192 worlds spends 262 ns per world-step, of which 124 ns is the contact solve).

Reading conventions for every entry: a number inside a *strong form* is the proposal's own and is quoted, not
endorsed; a figure marked FACT comes from a publication, sibling code or a planning pilot and has a
source-register record that is re-read or re-run before a gate cites it; "DESIGN arithmetic" can be redone by
hand and is not a measurement; a range in a *survives* line is a HYPOTHESIS until the named gate records a
value; every threshold is a TARGET. Nothing here was measured on FrankenRobotics code. Tier of the bead:
Solid (a decision record); the surviving mechanisms carry their own tiers.

## Scope

In: records `KI-01` to `KI-09` with their `calculation` tables, keywords, carriers and revival conditions. Out:
the schema, harness and lint (NAB-32); building any surviving mechanism (the carriers named in each entry).

## Technical approach

**Transcription rule (all killed-idea beads).** One `[[idea]]` record per entry. `title` is the bold heading;
`hypothesis` and `hypothesis_state` come from the parenthesis after it; `strong_form`, `refutation` and
`survives` are the three paragraphs verbatim; `revive_only_if` and `carriers` are the last two lines of each
entry below; `sections`, `gates`, `kill_gates`, `bets` and `refusals` are the identifiers found in the
*survives* paragraph; `state` is `refuted`, `narrowed` or `deflated` (an entry with no hypothesis whose
strong form failed on arithmetic is `refuted`; one that restated an existing position or miscounted is
`deflated`).

**Machine-checked arithmetic of this group** (`[[idea.calculation]]`; DESIGN arithmetic unless marked):

| Record | Expression | Value |
|---|---|---|
| KI-01 | half-precision unit round-off u₁₆ = 2⁻¹¹ | 4.883 × 10⁻⁴ |
| KI-01 | condition number of the rank-one operator with shift ρ = 10⁻²·tr(W)/48: κ = 1 + 48/0.01 | 4,801 |
| KI-01 | κ·u₁₆ (refinement contracts only if below 1) | 2.34 |
| KI-01 | refinement-to-factor cost ratio 9k/n at (n, k) = (27, 7), (35, 6), (18, 6) | 2.33, 1.54, 3.0 |
| KI-02 | ceiling of removing learner and inference at a 13% share: 1/(1 − 0.13) | 1.15 |
| KI-03 | specializable share 53/262 | 20.2% |
| KI-03 | ceiling with an infinitely fast recursion 262/209; with one twice as fast 262/(209 + 26.5) | 1.25; 1.11 |
| KI-03 | ceiling at a 16% share 1/(1 − 0.16); share needed for 1.5 | 1.19; one third |
| KI-04 | solve share 124/262 | 47% |
| KI-06 | probability that 1,000 windows all replay at 99.9% each: 0.999¹⁰⁰⁰ | 0.37 |
| KI-07 | occupancy of 2 and of 12 warps out of 48 | 4.2%; 25% |
| KI-08 | operation-count ratio 18,000/9,600 | 1.875 |
| KI-09 | steps per tile bounded by observation fences: frames at 50 to 100 Hz over steps of 1 to 2 ms | 5 to 20 |

FACT figures and their sources: the incumbent event trace (262, 53, 124 and 38 ns) and the cold compile time of
about 96 s are public sources of the incumbent-engine topic; the emulation result for KI-01 (κ·u₁₆ ≥ 1 in 37 to
64% of sampled contact states; the refined solve fails outright in 9 to 18%) is the plan-time conditioning
check, whose script was not retained and is re-run before any gate cites it; the published four- to five-fold
gain of three-precision refinement is against double precision at 22,000 unknowns.

**Keywords for the revive lint.** KI-01: half precision, f16, tensor cores for the solve, mixed-precision
refinement of the contact Hessian. KI-02: tensor-core policy inference, reduced-precision learner. KI-03:
supercompiled kernel, per-design kernel, partial evaluation per design. KI-04: equality saturation, e-graph,
superoptimizer. KI-05: fast-math, FMA contraction, approximate math mode, built-in sin and cos in kernels.
KI-06: unordered scatter, floating-point atomics. KI-07: megakernel, launch-cost advantage, limb summary.
KI-08: staged erasure as speed. KI-09: time tiles, parareal, time-parallel integration.

**The entries.**

**KI-01. Half precision on tensor cores for the contact solve** (H-15, REFUTED)
- *Strong form.* Factor the contact Hessian H = M + JᵀDJ in half precision on matrix accelerators, refine in
  single precision, and gain a factor of four or more on the solve.
- *Refutation.* DESIGN arithmetic: half precision has unit round-off u₁₆ = 2⁻¹¹ ≈ 4.9 × 10⁻⁴, and refinement
  contracts only if κ·u₁₆ < 1. A rank-one contact operator with the proposed shift ρ = 10⁻²·tr(W)/48 has
  κ = 1 + 48/0.01 = 4,801, so κ·u₁₆ = 2.34. FACT (software emulation on three reference models): after
  unit-diagonal scaling, κ·u₁₆ ≥ 1 in 37 to 64% of sampled contact states and the refined solve fails outright in
  9 to 18%. Cost: k refinement steps at about 3n² operations against a factor of n³/3 is a ratio of 9k/n, which is
  2.3 at n = 27 and k = 7, 1.5 at n = 35 and k = 6, and 3.0 at n = 18 and k = 6. Refinement alone costs more than
  the factorization it was meant to cheapen. FACT: the published four- to five-fold gain is against double
  precision at 22,000 unknowns, and against a single-precision baseline the theory promises at most two.
- *Survives.* Nothing about contact solves. Single precision in power-of-two scaled variables (§18.12, §35.10;
  BET-08). A request for a half-precision solve or state is refused (`FROB-REF-245`).
- *Revive only if.* On our own contact battery, after unit-diagonal scaling, κ·u₁₆ < 1 on every sampled contact state of a shape class and 9k/n < 1 at the refinement count k that reaches the fidelity envelope. Until both are measured the request is refused (`FROB-REF-245`).
- *Carried by.* BD-156 (scaling, factor and refinement kernels), BD-179 (the conditioning certificate), N00-35 (the conditioning check re-run).

**KI-02. Tensor-core products for a 256-wide policy**
- *Strong form.* Run policy inference and the learner's matrix products on matrix accelerators at reduced
  precision.
- *Refutation.* FACT: the learner and inference together are about 13% of training wall-clock on one incumbent
  stack, so removing them entirely gives at most 1/(1 − 0.13) = 1.15; and reduced precision leaves determinism
  class D2.
- *Survives.* Plain `f32` products in the kernel dialect. Device matrix paths only inside the labeled D4 training
  profile, whose results are re-evaluated on a stricter profile (§35.2, §37.11).
- *Revive only if.* A measured phase ledger of our own training in which learner plus inference is a share s with 1/(1 − s) at or above the claimed gain, and then only inside the labelled D4 training profile whose results are re-evaluated on a stricter profile.
- *Carried by.* BD-196 (on-device networks in plain `f32`), BD-269 (the D4 profile and its licences).

**KI-03. Supercompiled per-design kernels as a throughput lever** (H-04, REFUTED for recursion-only specialization)
- *Strong form.* Partial evaluation of the recursions on one fixed design, with equality saturation, gives at
  least 1.5 times the throughput of the universal kernel at no more than 60 s of compilation per design.
- *Refutation.* FACT (an incumbent's event trace, humanoid, 8,192 worlds): of 262 ns per world-step, the stages a
  design-specialized recursion could touch sum to 53 ns, 20%; the solve is 124 ns and constraint construction
  38 ns. DESIGN arithmetic: an infinitely fast recursion gives 262/209 = 1.25 and one twice as fast gives 1.11; a
  manipulation scene has a share of 16% and a ceiling of 1.19; a ceiling of 1.5 needs a share of one third. FACT:
  the incumbent's cold kernel compilation takes about 96 s.
- *Survives.* Specialization for tournament survivors only, and only if our own M2 profile shows a specializable
  share of at least one third (§21.18; GATE-SPARK-14; KILL-SPECIALIZE). Derivation stays for correctness,
  certificates and the derivative kernels of CPU solvers (C-14). Topology is data, so a new design recompiles
  nothing (§21.11). Structure-exploiting constraint assembly and solve is a separate and open question (H-04b,
  ACTIVE): its sparsity is carried as data, it is measured at M2, and it is not counted in the thesis until then
  (§16.23).
- *Revive only if.* Our own M2 profile shows a specializable share of at least one third of the step (the profile clause of KILL-SPECIALIZE), and the pilot reaches 1.2 times on the whole step within 60 s of compilation.
- *Carried by.* BD-371, N21c-16 (the measured share), BD-221 (the derivation toolchain against KILL-KIR).

**KI-04. Equality saturation and superoptimization of kernel arithmetic**
- *Strong form.* An e-graph with a solver over real-arithmetic rewrites, or a stochastic superoptimizer, extracts
  faster kernel bodies than common-subexpression elimination.
- *Refutation.* Every accepted rewrite changes bits, so each needs an error bound, a battery run and a new kernel
  identity. No sound equivalence check exists for floating-point code. On the trace of KI-03 the solve alone is
  124 of 262 ns, 47% of the step, which rewrites of the recursions do not touch.
- *Survives.* Common-subexpression elimination, and rule-trace rewrite certificates in the exact class E and the
  bounded class R (§21.7; DC-11). The method is reopened only on the profile condition of DC-11.
- *Revive only if.* The profile condition of decision core DC-11: our own M2 profile shows more than about a fifth of step time in derived arithmetic. Each accepted class-R rewrite still needs its polynomial-identity certificate, an error bound, the fidelity battery and a new kernel identity.
- *Carried by.* BD-018, BD-221, BD-349 (the decision-core compiler), N28a-28 (the DC-11 record), N28c-11 (its GATE-ATEL-13 rows).

**KI-05. Fast-math switches in the kernel pipeline**
- *Strong form.* Contraction of multiply-adds, an approximate arithmetic mode and built-in sine and cosine in
  batched kernels, beside a three-way parity gate.
- *Refutation.* Each switch lets a compiler or a device choose the rounding, so the CPU, CUDA and Metal kernels of
  one source no longer agree bit for bit and the parity gates (GATE-SPARK-01, GATE-SPARK-02) cannot pass. The two
  requirements exclude each other.
- *Survives.* The bans of §35.6 (ADR-SPARK-02): no fast-math, no fused multiply-add, no built-in transcendentals
  in batched kernels, no floating-point atomics. An explicit, declared fused multiply-add is OPEN-NUM-1.
- *Revive only if.* An ADR that replaces ADR-SPARK-02 after the parity gates are restated. A declared, explicit fused multiply-add is the open question OPEN-NUM-1 and is the only door.
- *Carried by.* BD-013 (the dialect lint), BD-017 (the IEEE probe), BD-020 (GATE-SPARK-00).

**KI-06. Unordered floating-point scatter, accepted if 99.9% of windows replay bitwise**
- *Strong form.* Drop the ordered scatter for unordered atomics if the two agree on 99.9% of windows.
- *Refutation.* DESIGN arithmetic: 0.999¹⁰⁰⁰ = 0.37. A campaign of a thousand windows would replay exactly about
  one time in three, and a replay that is bitwise most of the time is not bitwise.
- *Survives.* A stable-key sort and a segmented reduction (`K-seg-reduce`; §35.7). Integer atomics only for
  maximum, minimum, or and wrapping add.
- *Revive only if.* Never on a replayable profile. An unordered floating-point reduction is admissible only inside the labelled D4 profile, where no replay is claimed.
- *Carried by.* BD-152 (`K-sort`, `K-seg-reduce`), BD-181 (batch-composition invariance).

**KI-07. A derived launch-cost advantage, and a megakernel at high occupancy**
- *Strong form.* An 8- to 20-fold launch-cost advantage from fewer graph nodes; one fused megakernel at 75%
  occupancy or more, with a lossless 27-float summary of each limb.
- *Refutation.* FACT: a warm launch-graph replay has near-constant host cost, so cost per node does not
  accumulate. DESIGN arithmetic on the proposals' own figures: 2 warps of 48 per multiprocessor is 4.2% occupancy,
  and 12 of 48 is 25%, not 75%. The limb summary is exact only without base-coupled contact, and its Schur update
  is only positive semidefinite.
- *Survives.* The latency from an edit to the first GPU step, as a measured number (§17.8). Limb port packets as
  an intermediate tier (KILL-SKEL-PORT). The fused per-world step kernel as an [M] pilot whose benefit is unproven
  in sign (§21.18; KILL-MEGA).
- *Revive only if.* The one-week fusion pilot shows a paired lower bound of at least 2.0 on the whole step at 8,192 worlds with digests unchanged (KILL-MEGA). The launch-cost claim is revived only by the measured edit-to-first-step latency.
- *Carried by.* BD-441, N21c-21, N21c-22, BD-184, BD-385 with N14a-22 (limb port packets).

**KI-08. Staged erasure of algebra as a source of runtime speed**
- *Strong form.* Erasing algebraic structure at compile time makes the runtime strictly faster than a
  conventional engine.
- *Refutation.* It restates the plan's own compile-time position and names no new mechanism. Its figure, 1.875 in
  floating-point operations, is counted on a step that is bound by memory traffic (§36.11) and not by arithmetic.
- *Survives.* A compiler rule: proofs, tags and types are erased before runtime (§21.7).
- *Revive only if.* A measured profile in which the step is bound by arithmetic and not by memory traffic. Until then erasure is a compiler rule and is not counted as a lever.
- *Carried by.* BD-018 (no tags reach a kernel), BD-217 (the profile).

**KI-09. Per-world adaptive time tiles and time-parallel integration**
- *Strong form.* Advance each world through tiles of up to 64 steps between interactions, or integrate in
  parallel in time with coarse and fine propagators.
- *Refutation.* Separate worlds are already independent. DESIGN arithmetic: policy and sensor frames at 50 to
  100 Hz are observation fences, and at physics steps of 1 to 2 ms they bound a tile to 5 to 20 steps. Contact
  makes coarse and fine propagators disagree, and tiles make shapes dynamic.
- *Survives.* Fixed decimation in a static launch graph (§21.11).
- *Revive only if.* A static-shape formulation, and measured interaction-free windows longer than the observation fences. Neither exists.
- *Carried by.* BD-172 (the static step plan with fixed decimation).

## Acceptance criteria

- [ ] Records `KI-01` … `KI-09` exist with the three paragraphs verbatim, the revival condition and carriers of
      each entry, and the keywords above.
- [ ] The harness recomputes the thirteen calculation rows within tolerance.
- [ ] `FROB-REF-245` exists in the refusal registry with the meaning "a half-precision solve or state is
      refused", and a request for one from the command line returns it (checked by the carrier's own test; this
      bead checks the registry row).
- [ ] The revive lint, run on a planted bead titled "Factor the contact Hessian in f16 on tensor cores", fails
      `ki.match-without-citation` naming KI-01; the same title on a bead that cites KI-01 and the scaling
      kernels passes as the surviving form.
- [ ] The structure-exploiting constraint assembly and solve (an ACTIVE hypothesis, measured at M2) is **not**
      a record here: a test asserts that no keyword of KI-03 matches "ancestor-sparse assembly" or
      "tree-ordered factor".

## Test plan

### Unit tests

`crates/frob-conformance/tests/killed_ideas.rs`, module `g1_arithmetic`:

- `ki01_condition_and_roundoff`: 1 + 48/0.01 = 4801; 4801 × 2⁻¹¹ = 2.344 > 1.
- `ki01_refinement_ratio`: 9·7/27 = 2.333; 9·6/35 = 1.543; 9·6/18 = 3.0 (and not 2.75).
- `ki02_ceiling`: 1/0.87 = 1.149.
- `ki03_ceilings`: 262/209 = 1.2536; 262/235.5 = 1.1125; 1/0.84 = 1.1905; 1/(1 − 1/3) = 1.5.
- `ki06_window_survival`: 0.999¹⁰⁰⁰ = 0.36770.
- `ki07_occupancy`: 2/48 = 0.0417; 12/48 = 0.25.
- `ki08_ratio`: 1.875.
- `ki09_tile_bound`: 10 ms ÷ 2 ms = 5; 20 ms ÷ 1 ms = 20.
- `records_complete_and_verbatim` against `tests/fixtures/killed/g1.md` (this bead's entries).

### End-to-end script and logging

`scripts/e2e/decision_records.sh` (NAB-50) recomputes the group's calculations and runs the two planted revive
cases of the criteria. Logged per STD-E2E with each calculation's name, value, expected value and tolerance.

### Known answers and negative controls

Known answers: the table above. Negative controls: a stored ratio of 2.75 for (n, k) = (18, 6) (the planning
record's own slip; the harness must fail it); a record of KI-06 whose survival probability reads 0.90; a
proposal to enable a fast-math switch "only on Metal" (must match KI-05: one backend choosing its rounding is
exactly the failure). Determinism: the harness uses exact rationals for the ratios and `f64` with a stated
tolerance only for the power and the logarithms.

## Considerations

- Three of these entries have a live reversal condition on our own M2 profile (KI-03: a specializable share of
  at least one third; KI-04: more than about a fifth of step time in derived arithmetic; KI-08: a step bound
  by arithmetic). The M2 sweep (NAB-25) evaluates them; until then nobody builds the strong forms.
- KI-05 and KI-06 are not performance judgments; they are incompatibilities with the replay guarantee. They
  can be relaxed only inside the labelled training profile whose results are re-evaluated on a stricter one.
- An explicit, declared fused multiply-add is an open question (OPEN-NUM-1), not a killed idea.

## References

Appendix C.2.1; §0.8 rows H-04, H-04b, H-15; §21.20; §35.6; ADR-SPARK-02.
