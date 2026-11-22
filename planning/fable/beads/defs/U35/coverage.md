# Coverage of drafting unit U35 (§35 Numerics and determinism)

38 bead files: 3 seed beads (BD-177, BD-181, BD-269), 32 new beads (N35-01 to N35-32) and 3 registries
(N35-KA, N35-CX, N35-REF).

Keys of other units are named below where their beads already carry part of a subsection; they appear in no
`deps` field of this unit (only seed keys, reserved keys and this unit's own keys do).

## (a) Subsections and the beads that carry them

| Subsection | Title | Beads of this unit | Also carried by |
|---|---|---|---|
| §35 (opening) | What a number may claim; tiers of the chapter; ADR-NUM-01 to -06 | ADR-NUM-01: N35-01, N35-07. ADR-NUM-02: N35-14, BD-181. ADR-NUM-03: N35-09, BD-177. ADR-NUM-04: N35-29. ADR-NUM-05: N35-04, N35-06. ADR-NUM-06: N35-30. The two non-Solid items: class D1 on devices (N35-02, under KILL-D1) and the asynchronous variant behind `fast-training` (N35-05 pilot, N35-06, under KILL-SHARD) | KILL-D1's decision bead and the device probe are the Spark unit's (N21b-34, BD-017); KILL-SHARD's measurement is BD-199 |
| §35.1 | Determinism classes | N35-01 (the type, weakest-input-wins, no upward conversion, kernel-class mapping, the parity statement), N35-02 (the classifier: attained class from current records; never raised by relabelling; D1 status per family), N35-03 (G5 lanes on `batch-cpu` and the referee; the referee's cross-architecture row gated on FD-FSIM-07) | BD-005 (the `determinism` field of the claim product); FD-FSIM-07 (the upstream change); N21b-39 (device lanes) |
| §35.2 | Profiles | N35-04 (the four profiles as pin sets; options on `sim.run`; defaults; admission; snapshot binding), N35-05 (pilot: ceilings 1.15 and the asynchronous variant), N35-06 (`fast-training`: knobs, tag, three rules, `FROB-REF-861`) | N23a-18 (profile handling inside the trainer, `FROB-REF-867`) |
| §35.3 | What each class licenses | N35-07 (the table as data; per-verb minimum classes; the check; labels; 247, 561, 562, 864), N35-08 (realized pair correlation; divergence rate λ̂ and the 16.6/λ horizon; KA-803), BD-269 (the gate: planted cases and twins), N35-32 (its script), N35-31 (the M2 subset: replay, forks, memo) | BD-220 (memo reuse), BD-262 (`eval.compare`), BD-263 (promotion) |
| §35.4 | Decision-level equivalence under class D3 | N35-09 (per-world record: the five items, margins, `boundary-sensitive`, first divergence of discordant worlds), N35-10 (the four criteria and the verdict), N35-11 (thresholds frozen: OPEN-NUM-2), BD-177 (frozen ensembles, `spark.check --decisions`, the gate), N35-12 (its script) | N21b-33 (the check calls the test for a D3 pair) |
| §35.5 | A world does not depend on the batch it ran in | N35-13 (conditions 2 and 3: the anchored tree, neutral padding, compensated sums), N35-14 (all eight conditions as an audit with one fixture each; the normalizer rule), N35-15 (ten mutants, one per arrangement axis), N35-16 (condition 6: draw tables), BD-181 (the test at gate size), N35-17 (its script) | N16a-12 (padding in the batched step), N21b-15 (bucket layout), N21b-33 (fast sweeps), N17a-10 (family batches) |
| §35.6 | The bans | N35-18 (ban table, corpus, compile-option audit, seven canaries, OPEN-NUM-1 recorded) | BD-013 and N21a-10 (the lints), BD-053 (`FROB-REF-252`), BD-017 (probe of fused rounding) |
| §35.7 | Hazards and rules | N35-19 (HZ-01, -02, -06, -07, -12, -15, -16, -17, -18, -24, -25, -26, -27, -29, -30, -31), N35-20 (HZ-03, -04, -05, -08, -09, -10, -11, -13, -14, -19, -20, -21, -22, -23, -28), N35-21 (the histogram law of the quantile-sketch row), N35-03 (thread scheduling; worker count and tile size; hash-map iteration) | Each row's owner is named in the register rows |
| §35.8 | Anchored positions | N35-30 (row 5: representation, difference rule, ADR-NUM-06, PO-NUM-01 exhaustive host form, KA-790 to -792), N35-19 (HZ-31; OPEN-NUM-3), N35-31 (`FROB-REF-249` end to end), N35-REF (row 249) | N21b-37 (the mechanism and its per-backend exhaustive test), BD-157 (`K-anchor`), BD-179 (GATE-SPARK-05 out to 1 km) |
| §35.9 | Randomness by address | N35-22 (host service: address, three-level key layout, exact uniforms, normals; PO-NUM-03), N35-23 (the stream registry with exposure domains; 22 first rows), N35-24 (keying by physical entity; KA-793), N35-25 (proposal draws with exact weights), N35-16 (draw tables on every backend) | BD-180 (the device kernel `K-x-philox`), BD-200 (randomization from cards) |
| §35.10 | Diagonal scaling for single-precision contact solves | N35-29 (the five parts and the facts; decision OPEN-NUM-4 from GATE-SPARK-18), N35-30 (rows 7 and 8: PO-NUM-02 host form, the scaling rule, the tolerance case), N35-04 (precision profiles and the degraded policy), N35-27 (κ̂ in every record) | N18b-25 and BD-156 (the kernels), N21b-38 (the conditioning record and harness), BD-179 (GATE-SPARK-18's measurement) |
| §35.11 | Refusals | N35-REF (fifteen rows with triggers, repairs, exits and neighbouring codes), N35-26 (families; the three standing rules and the mission-query clarification as a sweep) | BD-005 (refusal type and loader) |
| §35.12 | A number never claims more than its certificate | N35-27 (the `numerics` block on every step and result), N35-28 (summaries state exclusions; the three wording rules by schema) | N21b-38 (κ̂ fields at the solve stage) |
| §35.13 | Invariants and proof obligations | INV-NUM-01: BD-181 (slot axis), N35-03, N35-20 (HZ-10). INV-NUM-02: N35-13. INV-NUM-03: N35-14, BD-181. INV-NUM-04: N35-16, N35-22. INV-NUM-05: N35-28. INV-NUM-06: N35-01. PO-NUM-01: N35-30. PO-NUM-02: N35-30, N35-29. PO-NUM-03: N35-22, N35-16 | PO-NUM-01 and -02 per backend: N21b-37, N21b-38, BD-179 |
| §35.14 | Known answers and negative controls | N35-30 (the ten rows and ten controls as one battery), N35-KA, N35-CX, N35-15 | — |
| §35.15 | Gates | See table (b) | — |
| §35.16 | Open questions | OPEN-NUM-1: N35-18 (recorded, default no, blocks nothing). OPEN-NUM-2: N35-11 (decision; blocks GATE-SPARK-16). OPEN-NUM-3: N35-19 and N35-30 (default 8 m centred; blocks nothing). OPEN-NUM-4: N35-29 (decision from GATE-SPARK-18) | — |

## (b) Gates, kill gates and admission gates

| Gate | Milestone (§40) | Measured or decided by | Supporting beads |
|---|---|---|---|
| GATE-SPARK-15 (batch-composition invariance) | M2 | BD-181, N35-17 | N35-13, N35-14, N35-15, N35-16 |
| GATE-SPARK-16 (decision-level equivalence under D3) | M2 | BD-177, N35-12 | N35-09, N35-10, N35-11 (thresholds) |
| GATE-SPARK-17 (licence refusals; D4 re-evaluation) | M3 | BD-269, N35-32 | N35-06, N35-07, N35-08 |
| GATE-SPARK-18 (scaling invariance; `f32` Newton step against `f64`; κ̂ ≤ κ₂) | M2 | N35-29 reads it to decide OPEN-NUM-4; the measurement is BD-179 (the Spark unit's seed bead) | N35-30 (host form of PO-NUM-02) |
| GATE-KIN-05 (the dynamics chapter's statement of the same invariance; named by seed BD-181) | M2 | BD-181, N35-17 (device rows and the path-table clause) | `batch-cpu` row shared with the dynamics unit |
| GATE-SAFE-09 (D4 refused on five paths; named by seed BD-269) | M3 | BD-269, N35-32 | N35-06, N35-02 |

No kill gate is defined in this chapter. Two defined elsewhere bear on it and are stated, with trigger and
fallback, in the Considerations of the beads they affect: KILL-D1 (N35-01, N35-02, N35-07, BD-177, BD-269)
and KILL-SHARD (N35-05, N35-06, BD-269). The other determinism and precision gates the chapter cites
(GATE-SPARK-01, -02, -05, -08) are the Spark unit's.

## (c) Counts

- Known answers carried: 21 (KA-785 to KA-805), all in N35-KA, each bound to an owner test.
- Counterexamples carried: 10 (CX-364 to CX-373), all in N35-CX.
- Refusal codes carried: 15 (FROB-REF-247, 248, 249, 250, 251, 252, 561, 562, 761, 861, 862, 863, 864, 865,
  866), all in N35-REF.
- Hazard rows: 31 (HZ-01 to HZ-31), 16 in N35-19 and 15 in N35-20.
- Invariants 6, proof obligations 3, architecture decisions 6, open questions 4: all placed (table (a)).

## (d) Needs from other slices

- **The device kernel for randomness by address (seed BD-180).** It must port the three-level key layout and
  the pinned vectors of N35-22 unchanged; the draw-table test (N35-16) compares the two. If a device reason
  forces another layout, N35-22 changes first.
- **The world address type in `frob-core`.** N35-22 uses the fields `(run_seed, namespace, world, episode,
  fork)`; the glossary gives `(run_seed, world, episode)`, a fork's child "has its own address", and the
  safety chapter needs a namespace field. One unit's need mentions "four `u32` words". The lead should fix
  one struct.
- **`spark.check` surface (seed BD-176 and the Spark unit's check beads).** Options used here:
  `--decisions --pair`, `--invariance --workload`, `--conditions`, `--kernel`, `--sweeps`, and the
  conformance `--mutant` loader.
- **Verb registry (the Tether unit).** A `uses` list with minimum determinism class per verb (N35-07); the
  options `--determinism`, `--precision`, `--on-degraded` on `sim.run` and the other world-running verbs
  (N35-04); `--pairing {exact, by-address, unpaired}` and the `pair_correlation` field on `eval.compare`
  (N35-07, N35-08); `x-frob-kind` and `x-frob-summary` annotations in generated schemas (N35-28).
- **Callers of the licence check.** Teleport (BD-204), forks (BD-205), memoized episodes (BD-220), comparison
  (BD-262), promotion (BD-263), the attribution verbs (BD-279, BD-280).
- **The evidence boundary (the safety unit).** `FROB-REF-825` on promotion, qualification, certificates and
  build packets (BD-308, BD-259, BD-133, BD-408).
- **Decision records with quantitative margins at every tier** (the requirement language and task units), and
  the fidelity ledger's envelopes and equivalence margins (BD-212): inputs of N35-09 and N35-10.
- **The rank-agreement measure** (Kendall τ on the top band with a bootstrap interval) as a shared function
  (the comparison unit).
- **Telemetry (seed BD-207).** The device histogram kernel and `telemetry.histogram` must reproduce the types
  of N35-21; a raw-draw telemetry tap is needed by N35-16.
- **The refusal registry's owner.** A `76x` entry in the family-to-exit rule (exit 4 for FROB-REF-761), and a
  code for a stream-entity hash collision inside one design or task (N35-23).
- **Compute envelope.** GATE-SPARK-15 at gate size (1,000 batch-of-one runs and 100 co-tenancies of up to
  8,192 worlds per workload and backend) and the referee pair of GATE-SPARK-16 exceed the default envelope;
  the work-package epic registers no request above it. A gate-measurement request is needed for BD-181 and
  BD-177.
- **Stream names.** Other units asked for streams under slightly different names (`autopsy.perturb`,
  `autopsy.shapley`, "certification draws", "command switches by physical time"); N35-23 registers them as
  `autopsy-perturb`, `autopsy-shapley`, `nemesis-certify`, `command-schedule`.

## (e) Contradictions, gaps and what was done

1. **"Both negative controls of §35.14."** GATE-SPARK-15 requires "both" controls; the table has ten rows, of
   which three concern this gate, and the metamorphic register and BET-02 name others. Used: every control
   named for this property must be caught (N35-15, BD-181). BET-02's planted contraction does not break batch
   independence and is discharged by N35-18 and the cross-backend check.
2. **Canonical zero.** One catalogue entry (CX-146) prints the canonical zero of a padded sum as +0; §35.5,
   §16.6 and KA-786 require −0.0. Used −0.0 (N35-13).
3. **"At least 300 worlds."** The exact smallest ensemble with a zero-discordance bound at or below 1% is 299
   (1.0002% at 298, 0.9969% at 299). Both figures are carried (N35-KA, N35-10).
4. **The tolerance illustration.** §35.10 and KA-798 use D = diag(1, 10⁴); the exact power-of-two rule gives
   D₂₂ = 2¹³ = 8,192 and a scaled residual of 8.192·10⁻⁴. Tests assert the exact figure.
5. **Pairing across device families.** The licence table allows a D3 pair as `paired-by-address`, calls arms
   on different families without D1 "unpaired by declaration", and `FROB-REF-561` refuses a request "as
   paired across device families without class D1". Used: never granted silently; refused when demanded as
   exact pairing; `paired-by-address` only on explicit request with a decision-equivalence record; otherwise
   unpaired (N35-07, BD-269).
6. **Overlapping refusal codes.** 561 and 825; 861 and 867; 562 and 584; 864 and 605; 248 and 244. Used: one
   code per condition and raising site, tabulated in N35-REF, consistent with the readings the attribution
   and learning units recorded.
7. **Exit code of FROB-REF-761.** The exit table assigns 4 to `70x` and 5 to `72x` and `73x`; `76x` is not
   named. Used exit 4 (a budget refusal at admission), flagged.
8. **Repairs.** The chapter's refusal table gives conditions only. The ranked repairs in N35-REF are derived
   from the chapter's rules and labelled DESIGN.
9. **Two histogram parametrizations.** §35.7 bins by the `f32` bit pattern (sign, exponent, top five mantissa
   bits; "at most 3.2%", exactly 3.125%); §29.7 bins a channel value quantized by a declared quantum (3.1%).
   N35-21 implements the pattern form and the integer octave-splitting form; the quantum and the device kernel
   stay with the telemetry bead.
10. **Unspecified estimators and thresholds in §35.4.** The net one-sided bound's estimator and the rank
    criterion's threshold are not given, and the gate row omits the rank criterion. Used: opposite endpoints
    of two one-sided 97.5% exact bounds; lower bootstrap bound of top-band τ at least 0.7 with top-design
    agreement; both DESIGN and frozen with OPEN-NUM-2 (N35-10, N35-11).
11. **The decimal fixture.** 1.0000001 and 0.9999999 are not `f32` values; the fixture is stated in `f64` and
    again with the `f32` neighbours 1 + 2⁻²³ and 1 − 2⁻²⁴ (N35-09).
12. **Class D2 without a batch-independence record.** The class table has no name for "repeats as the same
    batch but is not shown independent of it". Used: such a set is not D2; it is D3 against its own isolated
    run if an envelope exists and D4 otherwise, with reason `batch-dependent` (N35-02).
13. **GATE-SAFE-09's fifth path.** The build-packet writer arrives at M5, the gate is M3. Used: the evidence
    boundary's function with a conformance request at M3, recorded as such, re-measured with the real verb
    (BD-269, N35-32).
14. **A D4 result when no relaxation is funded.** If KILL-SHARD fires and the pilot funds no knob, the gates
    still need a D4 result. Used: a conformance D4 job, stated in the gate rows (BD-269, N35-32).
15. **Files not named by the plan.** `registries/streams.toml`, `registries/numeric_hazards.toml`,
    `registries/kernel_bans.toml`, `registries/placement_pairs.toml` and the schemas `frob.numcert/1`,
    `frob.equiv.world/1`, `frob.equiv.verdict/1`, `frob.detrecord/1`, `frob.batchcond/1`, `frob.hist14/1`,
    `frob.streams/1` are DESIGN names introduced here.
16. **Mechanisms of §35.8 and §35.10 already drafted elsewhere.** The Spark and contact units wrote the
    anchored-position, conditioning and scaling mechanisms. This unit wrote no duplicate; it carries their
    host-level known answers, the decisions and the registry rows.
17. **Overlap of N35-03 with FD-FSIM-07.** The upstream item's bead already lists cross-architecture lanes in
    G5. N35-03 owns the `batch-cpu` lanes and records the referee's row as `not run` until that item is
    adopted.
18. **Overlap of N35-26 with the Tether unit.** The three standing rules are stated in §35.11, so the sweep
    is here; the surface chapter's failure-class and parity gates are not duplicated.
19. **Milestones of the registries.** N35-REF is M0 because two build-time codes are raised by M0 beads;
    N35-KA and N35-CX are M1 because the first owner tests are M0 and M1.
