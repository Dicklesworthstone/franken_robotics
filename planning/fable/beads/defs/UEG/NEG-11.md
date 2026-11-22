---
key: "NEG-11"
title: "inconsistency E7-A resolved: default capacities C = 32, R = 160 and C = 64, R = 320, read from one table"
type: "task"
priority: 1
milestone: "M2"
labels: ["m2", "kinesis", "spark", "phylum", "frob-kernels", "solid", "test", "card"]
parent: "WP-47"
deps: ["NEG-03", "GB-41", "N21b-18", "BD-148"]
needs: []
related: ["N16b-26", "N17a-07", "BD-168", "BD-219"]
gates: []
plan_refs: ["Appendix E.7 (default capacities)", "§16.6", "§36.3", "§36.10", "§17.7", "Appendix M KA-814"]
---
## Background

Batched tiers allocate fixed capacities per world: C contacts and R constraint rows. They decide memory (the
dense Jacobian 4·R·n bytes is a third to a half of a world), they decide when a world is flagged `Degraded`
for exceeding capacity, and they are part of a bucket's identity. Three chapters stated defaults. The dynamics
chapter gave the reference quadruped C = 24, R = 128 (27 kB per world) and the humanoid C = 40, R = 224
(72 kB). The performance chapter gave C = 32, R = 160 (35 kB) and C = 64, R = 320 (97 kB), and the families
chapter used the latter for its 18-degree-of-freedom bucket. This was not an arithmetic error; two tables had
drifted. Settled: the performance chapter's table governs and the dynamics chapter now prints its values. The
defaults are DESIGN values frozen at M2 by measurement (owner decision OPEN-SPARK-3: freeze the bucket set and
the capacities of each reference workload). The durable fix is that there is one table and everything reads
it.

## Scope

In: the single capacities table and its readers; the byte-model regression; the scan. Out: the decision that
freezes the values (N21b-18); the packed lowering (BD-148); the capacity audit that measures overflow rates
(BD-168); the card (GB-41).

## Technical approach

1. **One table.** `registries/capacities.toml` holds, per reference workload and bucket: degrees of freedom,
   bodies, C, R, state bytes, status (`design-default` until the M2 freeze, then `frozen` with the measurement
   record). Readers: the packed lowering, the bucket planner, `sim.estimate`, the population memory model, the
   performance harness's disclosure block. No reader has a literal capacity.
2. **Values at plan time.** Q: n = 18, n_b = 13, C = 32, R = 160. H: n = 35, n_b = 30, C = 64, R = 320.
3. **Byte model check (exact).** B = 4·(61·n_b + 17·n) + 12·n² + 4·R·n + 48·R + 96·C + state. Q: 4,396 + 3,888
   + 11,520 + 7,680 + 3,072 = 30,556 before state (35 kB with about 4,000 bytes of state). H: 9,700 + 14,700 +
   44,800 + 15,360 + 6,144 = 90,704 (97 kB with about 6,000). With the superseded capacities: Q: 4,396 + 3,888
   + 9,216 + 6,144 + 2,304 = 25,948 (the 27 kB first printed implies about 1 kB of state); H: 9,700 + 14,700 +
   31,360 + 10,752 + 3,840 = 70,352 (72 kB).
4. **Consistency test across readers.** For each workload the five readers report the same (C, R); the
   buffers the lowering allocates have exactly the byte sizes the estimate predicts.
5. **Register and scan.** Item E7-A: stale patterns `C = 24`, `R = 128`, `C = 40`, `R = 224`, `27 kB`,
   `72 kB` in the scope of capacity, per-world or bucket text; pinned by `gb_41_reference_terms` and the tests
   below.

## Acceptance criteria

- [ ] `registries/capacities.toml` exists with the two rows and validates; five readers load it and none has a
      literal capacity (lint).
- [ ] All readers agree on (32, 160) and (64, 320); estimate bytes equal allocated bytes.
- [ ] Changing a row in a scratch registry changes every reader's value and the bucket identity (the
      `ShapeClassId` or bucket digest differs).
- [ ] The stale-value scan finds no hit for E7-A.

## Test plan

### Unit tests

- `capacities_table_schema`, `capacities_single_source` (lint for literals).
- `capacities_readers_agree`: lowering, planner, estimate, memory model, disclosure block.
- `capacities_byte_model`: 30,556 and 90,704; superseded 25,948 and 70,352 asserted different.
- `capacities_change_changes_identity`: a scratch row with R = 128 yields a different bucket digest.
- `capacity_overflow_is_flagged`: a `synthetic` world that needs 33 contacts at C = 32 is flagged `Degraded`
  with the capacity named, never truncated silently.

### End-to-end script and logging

`scripts/e2e/worked_cards.sh` (NEG-02) through GB-41's surface row (`frob sim estimate --workload Q|H
--json`): logs the table row, its status and each byte term; the stale-value step logs item E7-A.

### Known answers and negative controls

KA-814 (per-world byte model for Q at C = 32, R = 160 and H at C = 64, R = 320). Controls: the superseded
capacities; a reader with a literal. Determinism: integer arithmetic, D0.

## Considerations

- The defaults may change at the M2 freeze; the table carries the measurement that justifies the frozen values
  and every golden derived from them is regenerated in the same change.
- Capacities are part of a bucket's identity, so a change invalidates cached kernels and memoized episodes
  keyed on it; that is intended.

## References

Appendix E.7 (default capacities); §16.6; §36.3; §36.10; §17.7; KA-814; OPEN-SPARK-3.
