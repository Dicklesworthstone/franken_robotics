---
key: "ART-L7"
title: "golden fixture L.7: the `.frobcert` bundle listing of six entries, entry e2 in full, its mutations and CLI script"
type: "task"
priority: 2
milestone: "M3"
labels: ["m3", "lemma", "frob-cert", "frob-lemma", "frob-conformance", "solid", "test", "e2e"]
parent: "WP-54"
deps: ["BD-026", "BD-132", "BD-133", "BD-259", "ART-L4", "NJL-07", "STD-E2E"]
needs: []
related: ["N12-03", "N12-06", "BD-027", "BD-136", "BD-303", "BD-364", "BD-383", "BD-141", "ART-L9", "NJL-09"]
gates: []
plan_refs: ["Appendix L.7", "§12.4", "§25.5.1", "§25.5.2", "§25.5.4", "§24.13", "Appendix I.4", "§8.2"]
---
## Background

In FrankenRobotics (a robotics simulation and design system reached through the command line `frob`) a
**certificate bundle** (`.frobcert`, schema `frob.cert/1`, canonical JSON) is everything needed to re-check one
claim on a machine that has neither the agent, a GPU nor the simulator. `frob-lemma` writes bundles from the
plain proof objects that the mission engine emits; `frob-cert` reads them; the offline auditor `frob-verify`,
a single binary built from the Rust standard library and nothing else, re-checks each claim and returns
`accept`, `reject` or `unsupported` with the first failing rule named. An answer that makes several claims is
a set of bundles, one per claim, under one Merkle root, and the hash such an answer cites is that root.

This bead turns the seventh worked artifact of the specification into a golden fixture: the bundle of the
nine-artifact story (a quadruped carries a tray through a commercial-kitchen galley). It holds the proofs of
the refined mission answer of artifact L.4 (five entries, class Certified) and the qualification receipt of
the blinded lane (one entry, class Tested). The fixture is the listing that `prove.list` prints for the
bundle, one entry in full, and the mutations the auditor must reject. The numbers of the receipt are DESIGN
specimens; the arithmetic of every entry is exact.

Why this fixture matters (DESIGN). A certificate is only as good as the auditor's refusal to accept a wrong
one. The fixture gives the auditor one complete positive specimen per proof kind used by the first mission
slice (`lp-primal`, `lp-farkas`, `partition`) plus the receipt arithmetic, and six mutations that must each be
rejected; and it pins the rule that the weakest entry sets the class of the whole. Rejected: testing the
auditor only on bundles its own producer wrote in the same run (a shared misunderstanding of a field would
pass on both sides; a hand-entered specimen is independent of both).

## Scope

In: the listing; entry e2 in full; what the auditor recomputes for each entry; the six mutations; the
receipt's arithmetic; the class of the bundle; the script that assembles the bundle from the command line and
verifies it in tree and offline. Out: the bundle schema and reader (BD-026), the checkers per proof kind
(BD-132), the writer and the `prove` verbs (BD-133), the qualification lane and its receipts (BD-259), replay
of a whole campaign by the auditor (BD-383); the layers and the identity map (NJL-07).

## Technical approach

**Files** under `fixtures/golden/appendix-l/L7/`: `listing.specimen.json` (the table below as rows);
`e1.resolved.frobcert.json` … `e6.resolved.frobcert.json`; `e2.specimen.frobcert.json` (the text below);
`mutations/m1.json` … `m6.json`; `fields.toml`.

**The listing** (what `prove.list` prints; each row is one claim that the auditor re-checks).

| Entry | Claim kind | Class | Domain | Bundle proof kind | Proof object | Payload | Expected |
|---|---|---|---|---|---|---|---|
| e1 | `family-inclusion` | Certified | c:03: v ∈ [4, 6], r ∈ [2, 5/2] | `lp-primal` | p0, `lp-primal` | Forces affine in v at four load vertices, checked at v = 4 and v = 6 | accept |
| e2 | `family-inclusion` | Certified | c:06a: v ∈ (6, 8], r ∈ [11/5, 5/2] | `lp-primal` | p4, `lp-primal` | The same family, checked at v = 6 and v = 8 | accept |
| e3 | `family-exclusion` | Certified | c:01, c:04: v ∈ [4, 8], r ∈ [1, 3/2] | `lp-farkas` | p1, `lp-farkas` | y = (0, ½, ½); 40 + 6v > 40·r on the cells, least gap 4 N at (4, 3/2) | accept |
| e4 | `family-exclusion` | Certified | c:07 to c:12: v ∈ (8, 12], every r | `lp-farkas` | p2, `lp-farkas` | y = (0, ½, ½); 40 + 6v > 88 for v > 8 | accept |
| e5 | `partition` | Certified | The whole box [4, 12] × [1, 5/2] | `partition` | p3, `partition` | Thirteen cells; union equals the box; interiors disjoint; open and closed ends as listed | accept |
| e6 | `qualification-receipt` | Tested | Campaign `q:kitchen-tray/1`, index 1 | `sample-coverage` | — | 2 failures of 200 sealed clusters; bound 0.0357 at α₁ = 0.025; levels spent 0.025 of 0.05 | accept |

**Entry e2 in full**, in the container form of the bundle schema. A cell with two coordinates lists one
rational per coordinate in `lo` and `hi`, each rational a pair of integer strings (numerator, denominator):
here lo = (6, 11/5) and hi = (8, 5/2) in the chart (v, r).

```json
{
  "schema": "frob.cert/1",
  "claim": { "kind": "family-inclusion", "class": "Certified",
             "quantifiers": ["exists_causal_policy_per_design", "forall_admitted_worlds",
                             "forall_admitted_executions"], "horizon": "static" },
  "query": { "blake3": "b3:3c9d…", "bytes": "…the canonical bytes of L.4…" },
  "definitions": { "units": "SI", "frame": "footprint centre, +x forward, +y left, +z up", "clocks": [],
                   "ports_ordered": ["++", "+-", "-+", "--"] },
  "assumptions": [ { "id": "authority", "value": "independent normal forces", "survives": "grow" },
                   { "id": "world-set", "blake3": "b3:e17a…", "survives": "shrink" } ],
  "plant": { "model": "template:quad-mammal@1 with u = 5, cap = 88 N, lam = 1/10 m, tau_c = 4 N·m",
             "policy_class": "causal, indexed by nominal design, no randomization" },
  "domain": { "chart": "v = 1/half_width in 1/m; r = knee belt ratio",
              "cell": { "lo": [["6", "1"], ["11", "5"]], "hi": [["8", "1"], ["5", "2"]] } },
  "proof": { "kind": "lp-primal",
             "primitive_equations": ["sum f = W", "sum x·f = P_x", "sum y·f = P_y", "0 <= f <= c",
                                     "lam·f <= r·tau_c"],
             "payload": { "vertices": 4,
                          "argument": "forces affine in v; feasibility convex in the load; monotone in r",
                          "forces_newton_at_v8": [ [["88","1"], ["32","1"], ["40","1"], ["0","1"]],
                                                   [["32","1"], ["88","1"], ["0","1"], ["40","1"]],
                                                   [["40","1"], ["0","1"], ["88","1"], ["32","1"]],
                                                   [["0","1"], ["40","1"], ["32","1"], ["88","1"]] ],
                          "forces_newton_at_v6": [ [["76","1"], ["44","1"], ["40","1"], ["0","1"]],
                                                   [["44","1"], ["76","1"], ["0","1"], ["40","1"]],
                                                   [["40","1"], ["0","1"], ["76","1"], ["44","1"]],
                                                   [["0","1"], ["40","1"], ["44","1"], ["76","1"]] ] } },
  "checker": { "name": "frob-verify", "build": "…" },
  "expected_verdict": "accept",
  "lineage": { "merkle_root": "b3:d4a9…", "alpha_spend": [] },
  "authorization": { "execution_authorized": false, "hardware_safety_signoff": false, "sim_to_real_certified": false }
}
```

**Fields of the container.** `claim`: kind, class, the quantifier prefix and the horizon. `query`: the BLAKE3
hash and the canonical bytes of the statement it certifies (the mission query of artifact L.4). `definitions`:
units, frame, clocks and the ordered ports. `assumptions`: each with `survives` (`grow` or `shrink`: whether
the claim survives when the assumption set grows or shrinks). `plant`: the model and policy class. `domain`:
the chart and the cell. `proof`: the kind from the one closed registry of proof kinds, the primitive
equations (they travel in the bundle, because a proof about the wrong translation of the physics is still an
error), and the payload. `checker`, `expected_verdict`, `lineage` (the Merkle root; `alpha_spend` entries for
qualification receipts), and `authorization` (the three no-authorization flags, all false). The payload keys
`argument`, `forces_newton_at_v8` and `forces_newton_at_v6` extend the one-vertex key `forces_newton` to a
cell.

**What the auditor recomputes.**

| Entry | Recomputation |
|---|---|
| e1 | At v = 4 and v = 6 and each of the four load vertices: the forces of the affine family (40 + 6v, 80 − 6v, 40, 0) and its three mirror images sum to 160 N, give first moments (±16, ±12) N·m, lie in [0, 88] N and satisfy (1/10)·f ≤ 2·4 = 8 N·m at the cell's worst ratio r = 2 (76·(1/10) = 7.6 ≤ 8) |
| e2 | At each of the eight listed force vectors: the vertical sum (160 N); both first moments against the vertex (±16, ±12) N·m at the stated v (for (88, 32, 40, 0) at v = 8: (1/5)·(88 + 32 − 40 − 0) = 16 and (1/8)·(88 − 32 + 40 − 0) = 12); every capacity (at most 88 N); every knee bound at the cell's worst ratio (88·(1/10) = 8.8 ≤ (11/5)·4 = 8.8 N·m). That the forces are affine in v between the two ends, so the checks at v = 6 and v = 8 cover the cell; that the four vertices span the declared load hull; that the lower end in v is open as the partition entry says; that the information the allocator is assumed to have is in the observation contract |
| e3 | With y = (0, 1/2, 1/2) on the three balance rows, yᵀA = (1, 0, 0, −1), so f₊₊ − f₋₋ = 40 + 6v on every balanced allocation, while f₊₊ − f₋₋ ≤ 40·r on every admissible one; 40 + 6v > 40·r on v ∈ [4, 8], r ∈ [1, 3/2], with least gap 4 N at (4, 3/2) |
| e4 | The same dual; 40 + 6v > 88 for every v > 8 (the cells are open at v = 8, where equality holds) |
| e5 | Exact box arithmetic: thirteen cells; their union equals [4, 12] × [1, 5/2]; interiors are disjoint; every open and closed end matches the cells' own entries |
| e6 | The arithmetic of the receipt: the level α₁ = α/[1·2] = 0.05/2 = 0.025; the one-sided exact binomial upper bound for 2 failures of 200 at that level, 0.0357, which is at most the claimed 0.05; levels spent 0.025 of 0.05. Never a proof: class at most Tested |

**The class of the bundle.** Five Certified entries and one Tested receipt: the bundle is Tested, the class of
its weakest entry (e6). The offline verdict line is: `accept 6 of 6 entries; class of the bundle: Tested
(weakest entry e6); all three authorization flags false`.

**Mutations that must each cause rejection** (applied to e2, except the last, which applies to any entry):
(m1) a swapped foot order with unchanged forces; (m2) (16, 12) passed as the torque, where the support torque
in right-handed axes with +z upward is (P_y, −P_x, 0) = (12, −16, 0) N·m; (m3) an unreported positive force
floor; (m4) a passive-authority tag on a certificate that assumes independent normal forces; (m5) a cell end
certified closed where the partition lists it open (v = 6 for c:06a); (m6) a flag missing or true. The
refusal codes: m1 and m2 `FROB-REF-850` (a frame, unit, sign or port order that does not match the
definitions); m3 and m4 `FROB-REF-851` (an authority the design does not declare, or an omitted force floor);
m5 `FROB-REF-829` (a missing side condition or a wrong field); m6 `FROB-REF-833` (a no-authorization flag
missing or true).

**Two notes on naming.** The mission answer and the bundle name proof kinds from one closed registry
(`frob.cert.kinds/1`), under the same names, so the writer maps nothing. The schema writes one claim per
bundle file while a mission answer cites one bundle hash: the per-claim entries share one
`lineage.merkle_root`, and that root (`b3:d4a9…` in the story) is the hash the answer cites.

## Acceptance criteria

- [ ] `e2.specimen.frobcert.json` equals the text above byte for byte (manifest SHA-256); the six resolved
      entries parse strictly with the auditor's own reader, validate against `frob.cert/1`, and re-serialize
      to their input bytes.
- [ ] M1 lane: `frob prove bundle` on the refined mission answer of artifact L.4 writes five entries (e1 to
      e5) under one Merkle root; `frob prove list` prints five rows equal to the first five of the listing;
      `frob prove verify` and the offline `frob-verify` both accept all five; the bundle class is Certified.
- [ ] M3 lane: with the receipt of a committed qualification request added, the bundle has six entries, the
      listing has six rows, both checks accept, and the verdict line reports class Tested with e6 as the
      weakest entry and all three flags false.
- [ ] The auditor's recomputations of the table hold exactly: the eight force vectors of e2, the dual of e3
      and e4, the partition of thirteen cells, and the receipt bound 0.0357.
- [ ] Each of the six mutations is rejected by `frob prove verify` and by `frob-verify`, with the first
      failing rule named and the stated code; none is accepted.
- [ ] An unknown proof kind with consistent hashes, and a changed quantifier prefix with consistent hashes,
      are each rejected (`FROB-REF-823`, `FROB-REF-826`).
- [ ] `frob-verify` runs in an empty environment with no dependency outside the standard library, on a second
      machine, and returns the same six verdicts.
- [ ] Bundle bytes, the Merkle root and the listing are identical on two runs, in debug and release, on Linux
      and macOS (class D0); once blessed, the root equals the map's entry for `b3:d4a9…`.

## Test plan

### Unit tests

`cargo test -p frob-cert --test golden_l7` (the auditor's real reader and checkers; exact rational arithmetic)
and `cargo test -p frob-lemma --test golden_l7` (the real writer):
- `l7_e2_specimen_parses`; `l7_manifest_hash`; `l7_entries_reserialize_identically`.
- `l7_e2_eight_vectors`: sums 160; moments (±16, ±12); capacities ≤ 88; knee 8.8 ≤ 8.8 at v = 8 and
  7.6 ≤ 8.8 at v = 6.
- `l7_e2_affine_between_ends`: the vector at v = 7 interpolated from the ends is (82, 38, 40, 0).
- `l7_e2_cell_rationals`: lo = (6/1, 11/5), hi = (8/1, 5/2); a non-reduced rational `["12", "2"]` is refused
  (`FROB-REF-849`).
- `l7_e1_ends`: (64, 56, 40, 0) at v = 4 and (76, 44, 40, 0) at v = 6; knee 7.6 ≤ 8.
- `l7_e3_dual`: yᵀA = (1, 0, 0, −1); gap 4 N at (4, 3/2); `l7_e4_dual`: 40 + 6v − 88 > 0 for v = 8 + 1/1000.
- `l7_e5_partition_thirteen`: union equals the box; interiors disjoint; a planted overlap and a planted gap
  are each rejected.
- `l7_e6_receipt_bound`: bounds 0.0183, 0.0275, 0.0357, 0.0432, 0.0504 for 0 to 4 failures of 200 at 0.025;
  2 failures → promoted at claim 0.05; 4 failures → not promoted.
- `l7_bundle_class_is_weakest`: {Certified × 5, Tested} → Tested; no function returns a stronger class.
- `l7_mutation_m1` … `l7_mutation_m6`; `l7_unknown_kind_rejected`; `l7_changed_quantifier_rejected`.
- `l7_writer_reader_share_no_code`: a source scan (the writer crate does not depend on the reader crate).

### End-to-end script and logging

`scripts/e2e/art_l7_frobcert.sh` (header: `bead=ART-L7 milestone=M3 labels=golden,appendix-l,lemma,
verb:prove.bundle,verb:prove.list,verb:prove.verify seed=1 needs=cpu,frobd,second-host
plants=torque-as-moment,flag-true`). Steps: (1) run the mission steps of artifact L.4's script to obtain the
refined answer; (2) `frob prove bundle … --mission <id> --out quad-tray.m1.frobcert --key l7-b1 --json`,
assert five entries and `in_tree_check: "accept"`; (3) `frob prove list quad-tray.m1.frobcert --json`, compare
the rows with the first five of the listing (`e2e_assert_json` per cell of the table); (4) `frob prove
verify`, then the offline `frob-verify quad-tray.m1.frobcert`, both `accept`, class Certified; (5) M3 lane:
commit and run a qualification request for `quad-tray` on the task of artifact L.3 (`frob eval qualify …
--key l7-q1`), then `frob prove bundle … --receipt <hash> --out quad-tray.frobcert --key l7-b2`, assert six
entries; (6) `frob-verify quad-tray.frobcert`, assert the verdict line's three facts (six of six, class Tested
with the receipt as weakest entry, flags false) and recompute the receipt bound from its counts with
`e2e_assert_close` against the bundle's value; (7) copy the bundle to the second host and repeat step (6);
(8) the six mutations and the two extra rejections through `e2e_expect_caught`; (9) `frob-golden check
--artifact L7`. Plants: `torque-as-moment` and `flag-true` must each be rejected. The log
(`target/e2e-logs/art_l7_frobcert/<utc>.ndjson`) records every command with exit code and wall time, the
evidence block (claims [Proof], class, tier `static`, determinism `exact`), the Merkle root and entry digests,
each recomputed value against its expectation as an exact rational, each rejection with the rule named, and
the `summary`. In the M1 lane the M3 steps are logged as skips.

### Known answers and negative controls

- KA-762 (exact binomial bound, 200 worlds, α = 0.025: upper bounds 0.0183, 0.0275, 0.0357, 0.0432, 0.0504
  for 0 to 4 failures; to the printed digits; tier G0 on the host) is the receipt's arithmetic. KA-760 (the
  error budget of the blinded lane: Σ over i = 1…n of α/[i·(i + 1)] = α·n/(n + 1) < α; exact) gives the level
  spent.
- Known answers of this bead (exact; recomputed for it): the eight force vectors and their sums and moments;
  the dual values; thirteen cells; the least gap 4 N. They have no number in the known-answer suite.
- Counterexample kept alive: passing a first moment as a torque. For P = (16, 12) N·m the support torque is
  (12, −16, 0) N·m; a packet that passes (16, 12) as the torque must be rejected.
- Negative controls: the six mutations; the unknown kind; the changed quantifier; the planted overlap and
  gap; a non-reduced rational.
- Determinism lanes: twice on one machine; debug and release; Linux and macOS; a second machine; class D0.

## Considerations

- The class of a `sample-coverage` entry is stated in two ways in the specification: "never a proof: class at
  most Tested" in the certificate chapter, and Estimated in the mission chapter's table of proof kinds. The
  listing shows the receipt as Tested. Default used here: a qualification receipt from a referee-tier study
  with sealed clusters is Tested; a `sample-coverage` entry of a mission answer under a named population
  keeps the mission chapter's Estimated. The discrepancy is recorded in the unit's coverage file.
- The receipt's counts (2 failures of 200) are a story value; the real qualification of step (5) will give
  its own counts, and may not promote. The script asserts the arithmetic from whatever counts the receipt
  holds, and logs the decision as a metric, not as an assertion.
- Replay of the whole campaign by the auditor (levels, commitments and, after close, the cluster derivation)
  is a later bead (BD-383, M5); this fixture checks one receipt's arithmetic only.
- Kill gate KILL-AUGUR-FALSECERT (one certificate accepted by the auditor is shown false within its scope):
  the false-certificate procedure applies, and if the cause is the rule the proof kind is removed and its
  cells become Tested; the listing's classes then change by a blessed edit.
- The auditor's size is capped (TARGET: at most 12,000 lines) so that a person can read it; nothing in this
  fixture may be solved by adding a second reader.

## References

Appendix L.7; §12.4 (the bundle, the registry of proof kinds, the auditor), §25.5.1 to §25.5.4, §24.13 (the
receipt), Appendix I.4, §8.2 (the weakest input sets the class); GATE-LEMMA-03 and GATE-LEMMA-04 are measured
by the certificate beads, GATE-QUAL-03 by the campaign-replay bead, GATE-SAFE-01 by the flags beads; KA-760,
KA-762.
