---
key: "BET-18"
title: "Decide BET-18: mixed-topology batches beat per-family templates and compete with the incumbent mixed batch"
type: "task"
priority: 0
milestone: "M2"
labels: ["m2", "phylum", "spark", "frob-phylum", "frontier", "bet", "gate"]
parent: "WP-44"
deps: ["BD-183", "BD-184", "BD-185", "BD-186", "N00-28", "N17b-03", "N17b-07", "N17b-10", "N21b-19", "N38a-09", "STD-E2E"]
needs: []
related: ["N00-11", "N00-14", "N00-15", "N39b-44", "N40-31", "N40-32", "NS-06", "NS-14"]
gates: ["GATE-PHYLUM-02", "GATE-PHYLUM-03", "GATE-SPARK-10", "KILL-PHYLUM", "KILL-BUCKET"]
plan_refs: ["§4 (BET-18)", "§40.5", "§0.8 (H-03)", "§1.6 (N11)", "§2.4 (lever 1)", "§17.19", "§21.11"]
---
## Background

**The bet.** A design lowers to tables, not to code, so robots with different topologies (three leg counts,
mixed link lengths and actuator cards) can run as different worlds of one batch, inside padded capacities and a
small number of size buckets, with no kernel compiled per design. The bet is that this "topology as data" batch
beats the obvious alternative, one homogeneous template per robot family, and is competitive with the one
published incumbent mixed batch, on the two numbers that matter. It is the mechanism of the lever "decide many
designs per turn", which removes the one-design-per-agent-turn cap. Tier: Frontier.

**What is and is not claimed.** Mixed topologies in one GPU batch is not a first: one environment suite and two
solvers of one engine document it, and one paper reports a mixed batch of three-, four- and five-legged bodies
at 86 k samples/s at 2,048 environments, 0.86 of the harmonic mean of its homogeneous rates (on a solver that is
maximal-coordinate, documented as not deterministic and reported as slower than its framework's
reduced-coordinate solvers). The only permitted claim is two measured numbers against that incumbent on the
Linux host: distinct designs times steps per second, and time from an edit to the first GPU step. A third
candidate is a property: run-to-run replay and forks inside a mixed batch.

## Scope

In: pre-registration, the M2 reading, verdict and decision record. Out, with the beads that have them: variants on
devices, the mixed batch and GATE-SPARK-10 (BD-183); edit-to-first-step latency (BD-184); the per-family template
control arm (BD-185); the head-to-head study with its four arms (BD-186) on the 512-topology workload and frozen
mutation corpus (N17b-03), under the owner's compute envelope (N17b-07); the kill decisions KILL-PHYLUM (N17b-10)
and KILL-BUCKET (N21b-19), which this bead reads and cites and never re-decides. KILL-PHYLUM is the one decision
on the mixed batch; KILL-BUCKET is its device-side statement and defers to it where the two differ.

## Technical approach

1. **Falsifier clauses** (any falsifies):
   a. Against the incumbent's heterogeneous solver on the Linux host we lose on both numbers (distinct designs
      times steps per second; time from an edit to the first GPU step).
   b. Nine per-family templates cover at least 95% of the mutation corpus at higher throughput.
   c. The mixed batch runs below 0.5 of the harmonic mean of its homogeneous rates (the tolerance for keeping
      the cohort as the engine is 0.70).
   d. Any of the 512 designs of a mixed cohort fails to reproduce the digests of its single-design run.
2. **Competing arms.** Per-family templates with lockable joints and per-design parameter tables; padding every
   world to the largest robot; the incumbent heterogeneous solver. **Potency control.** Admitting a variant into
   a free slot must change no other world's digest; a planted cross-world read must be caught.
3. **Deciding gates (TARGETs).**
   - GATE-PHYLUM-02 (Known answer, M2): each of 512 designs in a mixed cohort has the same digests as in a
     homogeneous batch, on `batch-cpu` and per GPU back end, at three bucket sizes and under slot dispatch with
     100 random co-tenancies.
   - GATE-PHYLUM-03 (Performance, M2): the head-to-head run and published with all four arms. For the cohort to
     remain the engine: at least 0.70 of the harmonic mean of the homogeneous rates at padding waste of at most
     25% on cohorts of at least 256 designs; a lower bound above 1 against the template arm on edit to first
     accepted result at K ≤ 128; at most eight buckets.
   - GATE-SPARK-10 (Performance, M2): per-world parameter variants cost at most 5% at 8,192 worlds; 256
     topology variants of one bucket run on one captured graph with zero compiles after warm-up; a mixed batch
     of three body plans reaches at least 0.70 of the harmonic mean of its homogeneous rates; admitting a
     variant changes no other world's digest.
4. **Kill gates and fallback.** KILL-PHYLUM (M2): GATE-PHYLUM-03 fails (below 0.5 of homogeneous throughput on
   512 designs, more than eight buckets, or the template arm covers at least 95% of the mutation corpus and is
   faster). KILL-BUCKET (M2): more than eight buckets are needed to hold the padding overhead Ω₃ ≤ 2 on the
   reference family corpus, or the mixed batch of GATE-SPARK-10 runs below 0.5 of the harmonic mean of its
   homogeneous rates after one optimization pass. Fallback: per-family templates become the
   engine (at most nine padded templates, each a homogeneous environment, scheduled in turn), with per-world
   parameter variants, which are Solid and unaffected; then sequential environments. The table-driven path
   remains for admission and preview on `batch-cpu`.
5. **Downstream if falsified.** The "many designs per turn" lever is delivered by parameter variants and
   scheduled templates; designs per agent turn falls for topology-diverse populations and the turn cap of the
   thesis is recomputed; the differentiator N11 is rendered with whatever the two numbers are, or withheld.
6. **Procedure.** Register the four clauses, the corpus and the incumbent's pinned configuration before the
   head-to-head; read at M2 exit; `docs/adr/bet-18-m2.md`.

## Acceptance criteria

- [ ] Pre-registered with the mutation corpus digest and the incumbent pin.
- [ ] The head-to-head is published with all four arms, each with its disclosure block; both numbers are
      recorded for our arm and for the incumbent.
- [ ] Digest equality holds for 512 of 512 designs (or the failing design is named).
- [ ] Verdict recorded in `docs/adr/bet-18-m2.md`, citing the KILL-PHYLUM (N17b-10) and KILL-BUCKET (N21b-19)
      decisions on record by identifier and hash; with either missing the verdict is not filed.

## Test plan

### Unit tests
`bets.rs`: `bet18_mixed_at_0_78_holds`; `bet18_mixed_at_0_62_gate_fails_short_of_falsifier` (between 0.5 and
0.70: GATE-PHYLUM-03 fails, so the bet is not `Held`; clause (c) does not fire and the 0.5 kill threshold is not
crossed, so it is not `Falsified`; the verdict is recorded `open: gate failed short of the falsifier`, which
blocks the M2 exit until the decision record states the engine in force and why);
`bet18_mixed_at_0_44_falsifies_c_with_kill_record` (clause (c) true; N17b-10's record: KILL-PHYLUM fired);
`bet18_templates_cover_96_percent_faster_falsifies_b`; `bet18_one_digest_mismatch_falsifies_d`;
`bet18_reads_kill_bucket_nine_buckets` (N21b-19's record: nine buckets needed for Ω₃ ≤ 2 → fired; read, not
computed here).

### End-to-end script and logging
`scripts/e2e/bet_18_decide.sh` recomputes the harmonic-mean ratio from the per-family homogeneous rates and the
mixed rate, checks digest equality counts, reads the verdict of BET-18 from the bet register's report (`cargo
run -p frob-conformance --example bets -- status --json`, example `bets` of N00-28, the one verdict rule, which
prints a `GateFailedShort` reading as `open: gate failed short of the falsifier`; no second verdict rule in the
script) and checks the ADR and N11's rendering against it. Logged
per STD-E2E: per arm the rate with its disclosure block, the harmonic mean, the ratio against 0.70 and 0.5,
padding waste, bucket count, edit-to-first-step times, digest matches; `summary` naming the three gates and two
kill gates.

### Known answers and negative controls
Harmonic mean of rates r₁…r_k: k ÷ Σ(1/r_i); for homogeneous rates 100, 80 and 60 k samples/s it is 76.6 k, so
0.70 of it is 53.6 k and 0.5 is 38.3 k. The published incumbent ratio 0.86 is context. Negative controls: the
planted cross-world read; a mixed rate compared with the arithmetic mean (overstates the baseline differently;
the test pins the harmonic mean); a template arm denied lockable joints (an incompetent arm; rejected by the
arm audit).

## Considerations

- Between 0.5 and 0.70 the gate fails without the kill threshold being crossed; the decision record states
  which engine is in force and why.
- Edit-to-first-step latency is reported on warm, cool and cold paths; only like paths are compared.
- Without the approved envelope the 512-design head-to-head is replaced by the pilot, and the verdict says so.

## References

§4 (BET-18), §40.5, §0.8 (H-03), §1.6 (N11), §2.4, §17.19, §21.11; GATE-PHYLUM-02, -03; GATE-SPARK-10;
KILL-PHYLUM; KILL-BUCKET.
