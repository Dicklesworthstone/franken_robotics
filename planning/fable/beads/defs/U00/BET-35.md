---
key: "BET-35"
title: "Decide BET-35: two machines (the division of labour between the two-card Linux host and the Mac Studio pays)"
type: "task"
priority: 1
milestone: "M4"
labels: ["m4", "spark", "frob-kernels", "frontier", "bet", "gate"]
parent: "WP-46"
deps: ["N00-28", "N00-07", "BD-370", "STD-E2E"]
needs: ["§21.17: the fleet pilot (its parameter g and the falsifier g < 1/3) and the list of transfer classes allowed to cross between the machines", "§19.15: the world-bank breadth experiment that measures the held-out gap against the number of resident scenes"]
related: ["BD-029", "BD-199", "BD-236", "BET-09", "N00-17", "N00-22"]
gates: ["GATE-SPARK-13", "KILL-FLEET"]
plan_refs: ["§4 (BET-35)", "§40.5", "§1.8", "§21.24", "§36"]
---
## Background

**The bet.** The program has two machines and uses each for what it is good at.

| | Linux host `trj` | The Mac Studio |
|---|---|---|
| Hardware (FACT) | Two RTX 4090, 24 GB each, no link between the cards; often heavily loaded | M5 Ultra: 36 CPU cores, an 80-core GPU, 256 GB of unified memory; a measured Metal working-set limit of about 222 GiB and a largest single buffer of about 167 GiB; memory bandwidth 1.2 TB/s by the vendor's figure |
| Role | Training and high-throughput rollouts on CUDA; the two cards run independent shards | The world bank and large scenes; proof and symbolic work; the sampled referee; replay; the viewer; large-memory jobs; a Metal rollout tier whose throughput is a measured number |
| Crosses the boundary | Seeds, policy weights and receipts only | Seeds, policy weights and receipts only |

The bet is that orchestrating a campaign across both is faster than the two cards alone, that the Mac's large
memory buys something measurable (a larger world bank that narrows the held-out gap), and that the rule about
what crosses the boundary holds. Tier: Frontier (the orchestration; each machine alone is Solid).

**Scale facts behind the division (derivations).** Per-world state is about 20–53 kB for a quadruped and 56–180
kB for a humanoid, so 8,192 quadruped or 4,096 humanoid worlds use about 1 to 3% of a 24 GB card: memory does
not bind at the planned batch sizes. A 2 cm home scene is tens of megabytes and a 2 cm warehouse several
gigabytes, which is why the Mac holds the world bank. A CPU referee can shadow about 0.1–2.4% of GPU
world-seconds, which is why referee work is sampled. Whether the step on an RTX 4090 is bound by memory
bandwidth at these state sizes is a HYPOTHESIS for the M2 profile.

## Scope

In: pre-registration, the M4 reading, verdict and decision record. Out: the fleet scheduler, the world bank,
sharding across the two cards.

## Technical approach

1. **Falsifier clauses** (any falsifies):
   a. A campaign on both machines is less than 1.3× faster than on the two RTX 4090 alone.
   b. A larger world bank does not reduce the held-out gap.
   c. More than 1 GB per campaign-hour crosses between the machines.
2. **Competing arm.** The two cards alone. **Potency controls.** The paired harness returns a planted ratio; an
   A/A campaign does not clear 1.02; a planted forbidden transfer (a scene pack or a trajectory batch sent
   across) is detected and refused.
3. **Deciding gate (TARGET).** GATE-SPARK-13 (Performance, M4): on the standard campaign, the paired lower bound
   of the speedup of two machines over the better single machine is at least 1.3; at most 1 GB crosses per
   campaign-hour; no forbidden transfer class crosses.
4. **Kill gate and fallback.** KILL-FLEET (M4): the pilot shows g < 1/3, or after one repair cycle the paired
   lower bound of campaign speedup is below 1.3 against the better single machine. Fallback: the machines run
   standalone as independent shards; jobs are assigned by the role table; the seeds-only rule stays; the Mac
   keeps proof, replay and viewer work.
5. **Downstream if falsified.** No fleet scheduler ships; campaign plans are per machine; clause (b) alone
   limits the world bank to evaluation, qualification, replay, clearance and retrofit search (its own kill gate
   decides that); nothing on the critical path depends on this bet.
6. **Procedure.** Register the standard campaign, the role table, the transfer-class list and the byte
   accounting before the run; read at M4 exit; `docs/adr/bet-35-m4.md`.

## Acceptance criteria

- [ ] Pre-registered with the campaign's digest and the allowed transfer classes (seeds, policy weights,
      receipts).
- [ ] Campaign wall time on both machines and on the two cards alone, paired, with the lower bound of the ratio
      against 1.3 and disclosure blocks for both.
- [ ] Bytes crossed per campaign-hour against 1 GB, by transfer class; zero forbidden classes.
- [ ] The held-out gap at two world-bank sizes with its confidence sequence.
- [ ] Verdict and the KILL-FLEET decision recorded as an ADR.

## Test plan

### Unit tests
`bets.rs`: `bet35_lb_1_42_and_0_3_gb_per_hour_holds`; `bet35_lb_1_18_falsifies_a`;
`bet35_gap_unchanged_falsifies_b`; `bet35_1_6_gb_per_hour_falsifies_c`;
`bet35_forbidden_transfer_class_fails_gate`; `bet35_pilot_g_0_28_fires_kill_fleet`.

### End-to-end script and logging
`scripts/e2e/bet_35_decide.sh` runs `frob fleet status --json` and `frob fleet plan --json` for the standard
campaign, reads the two campaigns' receipts and the transfer log, recomputes the paired bound, the byte rate
and the verdict. Logged per STD-E2E: machine fingerprints, wall times, ratio and bound, bytes by transfer
class per campaign-hour, world-bank sizes and held-out gaps; `summary` naming GATE-SPARK-13 and KILL-FLEET.

### Known answers and negative controls
Scale arithmetic: 8,192 × 20–53 kB = 0.16–0.43 GB (0.7–1.8% of 24 GB); 4,096 × 56–180 kB = 0.23–0.74 GB
(1.0–3.1%). Negative controls: the planted forbidden transfer; an A/A campaign; a campaign in which the Mac ran
the same shard twice (no added work: must not count as speedup).

## Considerations

- The Linux host is often heavily loaded by other work; both arms are run under the same load conditions and
  the load is in the disclosure block.
- The seeds-only rule is also a privacy and reproducibility rule: a world is reconstructed from its address on
  whichever machine needs it.

## References

§4 (BET-35), §40.5, §1.8, §21.24; GATE-SPARK-13; KILL-FLEET.
