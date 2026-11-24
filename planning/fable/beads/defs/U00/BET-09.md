---
key: "BET-09"
title: "Decide BET-09: Metal on Apple silicon is a real rollout tier (at least 25% of one RTX 4090 at matched settings)"
type: "task"
priority: 0
milestone: "M2"
labels: ["m2", "spark", "frob-spark-metal", "frontier", "bet", "gate"]
parent: "WP-19"
deps: ["BD-142", "BD-174", "BD-217", "N00-28", "N21b-08", "STD-E2E"]
needs: []
related: ["BET-35", "N00-14", "N00-15", "N40-31", "N40-32", "NS-10"]
gates: ["GATE-SPARK-04", "KILL-METAL"]
plan_refs: ["§4 (BET-09)", "§40.5", "§1.8", "§1.6 (N10)", "§21.24"]
---
## Background

**The bet.** The project has two machines: a Linux host with two RTX 4090 cards and a Mac Studio (M5 Ultra, 36
CPU cores, an 80-core GPU, 256 GB of unified memory). The bet is that the Mac's GPU, driven through Metal from
the same kernel source as CUDA, is a real rollout tier: fast enough that training and evaluation on it are
practical, not a demonstration.

**What is and is not claimed.** One incumbent lowers its kernels to Metal; other stacks document CPU-only
support on macOS; no published rigid-body throughput on Apple silicon was found. So the Metal tier is a number,
not a first: the permitted wording is "measured quadruped throughput on an M5 Ultra, reported against the
Metal-capable incumbent on the same machine and against one RTX 4090", printed only with the measured values.

**Why it is plausible and what must be measured.** At the planned state sizes the 25% threshold needs about a
tenth of the Mac's memory bandwidth (vendor figure 1.2 TB/s), so the open questions are command-buffer overhead
and the GPU's arithmetic rate. The generic Metal compute gateway the backend builds on accepts at most 31
buffer arguments, which forces packed arguments. Tier: Frontier (the Metal rollout tier is a Frontier mechanism
under KILL-METAL; the Mac's other roles are Solid and do not depend on this bet).

## Scope

In: pre-registration, the M2 reading, verdict and decision record. Out, with the beads that have them: the Metal
backend's full batched step (BD-174); the throughput parity rows, the performance harness and the GATE-SPARK-04
record (BD-217); the Metal-path incumbent measured on the Mac (BD-142); the run-to-run determinism lanes per
device family (N21b-39); the KILL-METAL decision itself (N21b-08), which this bead reads and cites and never
re-decides.

## Technical approach

1. **Falsifier.** On the M5 Ultra, quadruped throughput is below 25% of one RTX 4090 at matched settings (same
   workload, world count, time step, substeps, solver iterations and contact capacity).
2. **Competing arm.** The Metal-capable incumbent on the same machine, measured under the same disclosure block
   and reported beside the project's figure. The arm does not enter the threshold; it enters the wording.
3. **Potency control.** The paired harness must return the planted ratio on ten pairs of constant ratio, and an
   A/A run (the same backend against itself) must not clear 1.02; a throttled build (a planted sleep per step)
   must be detected as below threshold.
4. **Deciding gate (TARGET).** GATE-SPARK-04 (Performance, M2): Metal throughput at least 25% of one RTX 4090 at
   matched settings on the M5 Ultra, reported beside an incumbent with a Metal path measured on the same
   machine. The quantity compared is the paired lower bound of the throughput ratio.
5. **Kill gate and fallback.** KILL-METAL (M2): GATE-SPARK-04 fails after one optimization pass, or the Metal
   backend cannot hold run-to-run determinism. Fallback: Metal becomes the development and replay tier; CUDA is
   the training tier; the Mac trains on `batch-cpu`.
6. **Downstream if falsified.** The Mac's other roles do not depend on this bet: the world bank and large
   scenes, proof and symbolic work, the sampled referee, replay, the viewer and large-memory jobs stay. The
   scenario "the same task on a Mac" is demonstrated on `batch-cpu` and described by that name. The two-machine
   bet (BET-35) is re-read with the Mac as a CPU shard.
7. **Evidence so far.** Sibling code shows one kernel source compiled for Metal and checked against a CPU
   emulation (FACT with path, in the foundation fact sheet); no throughput figure of any rigid-body engine on
   this machine exists.
8. **Procedure.** Register the matched settings and the threshold before the first timed run; one optimization
   pass is allowed and recorded; read at M2 exit; `docs/adr/bet-09-m2.md`.

## Acceptance criteria

- [ ] Pre-registered with the matched settings as a disclosure-block template.
- [ ] Three figures recorded with disclosure blocks: ours on Metal, the incumbent on Metal on the same machine,
      ours on one RTX 4090; the paired lower bound of the Metal-to-4090 ratio against 0.25.
- [ ] Run-to-run determinism of the Metal backend recorded (class D2 on that family, or not).
- [ ] Verdict recorded in `docs/adr/bet-09-m2.md`, citing the KILL-METAL decision on record in N21b-08's ADR
      (by identifier and hash; with no decision on record the verdict is not filed); N10 rendered with the
      numbers or withheld.

## Test plan

### Unit tests
`bets.rs`: `bet09_ratio_lower_bound_0_27_holds`; `bet09_point_0_26_lower_bound_0_22_falsifies` (the threshold
is on the lower bound); `bet09_reads_kill_metal_fired_on_determinism` (throughput bound 0.31, but N21b-08's
record says KILL-METAL fired because Metal failed run-to-run determinism → `Falsified`, the record's hash
carried; the trigger itself is applied in N21b-08); `bet09_unmatched_settings_rejected` (substeps differ between
the two disclosure blocks → the comparison is refused, not scored); `bet09_kill_metal_names_fallback_tier`.

### End-to-end script and logging
`scripts/e2e/bet_09_decide.sh` reads the three performance records (the receipts of BD-217's `kill_inputs` step),
checks that their disclosure blocks match field by field, recomputes the ratio's paired lower bound, reads the
verdict of BET-09 from the bet register's report (`cargo run -p frob-conformance --example bets -- status
--json`, example `bets` of N00-28; no second verdict rule in the script), and checks the ADR and the rendered
wording against both. Logged per STD-E2E: each figure with units and its block, the ratio and bound against 0.25, the
incumbent's figure, determinism class; `summary` naming GATE-SPARK-04 and KILL-METAL.

### Known answers and negative controls
Harness known answers: ten pairs of constant ratio r return the bound r; an A/A campaign does not clear 1.02.
Negative controls: the throttled build; mismatched disclosure blocks; a figure with no block (refused).

## Considerations

- Thermal state and other load on the Mac change the number; the fingerprint and machine state are part of the
  record and the run is repeated on a quiet machine.
- A pass at 25% says the tier is usable, not that it is competitive with the incumbent on Metal; that
  comparison is reported and may go either way.
- No bit-equality claim follows from this bet (see BET-02).

## References

§4 (BET-09), §40.5, §1.8, §1.6 (N10), §21.24; GATE-SPARK-04; KILL-METAL.
