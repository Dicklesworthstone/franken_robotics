# Coverage of drafting unit U21c

Chapter §21 Spark (GPU backends and the kernel toolchain), subsections §21.16 to §21.26, the seed beads BD-370 and
BD-441, and the chapter registries N21-CX and N21-REF.

Files: 47 beads. Two seed beads (BD-370, BD-441), two registry beads (N21-CX, N21-REF), 43 new beads (N21c-01 to
N21c-43). By milestone: M0 12, M2 21, M4 10, M8 4.

## (a) Subsections and the beads that carry them

| Subsection | Title | Beads |
|---|---|---|
| §21.16 | The hardware fingerprint | BD-029 (U21a's seed: the record `frob.fingerprint/1`, its digest and the verb `spark.fingerprint`); N21c-01 (rate probes: bandwidth, launch and commit, compile, thermal soak); N21c-02 (arithmetic-feature probes: 64-bit integers and atomics, language version, canaries, subnormal class); N21c-03 (peer access; OPEN-SPARK-6); N21c-04 (verb `spark.devices`); N21c-05 (currency: identity digest, the four bound consumers, `FROB-REF-606`); N21c-06 (end-to-end lifecycle and the GATE-SPARK-07 record on both machines) |
| §21.17 | Division of labour between the machines, and the fleet scheduler | N21c-07 (role table as data [S]); N21c-08 (transfer classes, cross-machine bundle, meter, `FROB-REF-618`, INV-SPARK-10); N21c-09 (scene regeneration to the same `SceneId`; replay of a failed world from address, design, weights and recipe; unpaired across families); N21c-10 (pilot: phase ledger on `trj` alone, g); N21c-11 (KILL-FLEET, first reading and the full rule); N21c-12 (resource model `frob.fleet.job/1`); N21c-13 (assignment core: queues, role affinity, shortest estimated work with aging); N21c-14 (idempotent records, drop-out and re-queue, balance); N21c-15 (verb `fleet.status`); BD-370 (verb `fleet.plan`, GATE-SPARK-13, KILL-FLEET final reading) |
| §21.18 | Specialization for survivors only; the fused step kernel | Specialization: BD-371 (U21a's seed: the specializer, GATE-SPARK-14, final reading); N21c-16 (specializable share from the M2 profile); N21c-17 (three-arm pilot); N21c-18 (KILL-SPECIALIZE, first reading and the full rule); N21c-19 (bit-identity fence, `FROB-REF-613`, INV-SPARK-09); N21c-20 (survivor cohort switch-over, versioned state map, promotion rule). Fused kernel: BD-441 (epic and short plan); N21c-21 (one-week pilot); N21c-22 (KILL-MEGA); N21c-23 (flag-gated mechanism, conditional) |
| §21.19 | Routes considered and not taken | N21c-24 (the eight routes with revisit triggers; dependency deny-rules; INV-SPARK-01); OPEN-SPARK-5's route also in N21c-38 |
| §21.20 | Rejected mechanisms, with the calculation that refutes each | N21c-25 (half-precision factorization, order-dependent sketches, unordered scatter, tensor-core policy products); N21c-26 (copy-on-write forks, hashed directories, live zero-copy reads, per-design kernels by default, adaptive time tiles, sampling control as evaluator); rows also in N21-CX (CX-224 to CX-233) |
| §21.21 | Refusals | N21-REF (all eighteen codes, exit code, repairs, raising sites, end-to-end provocation); raising sites in this unit: N21c-05 (606), N21c-19 (613, and the specialized case of 603), N21c-42 (616), N21c-08 (618); proposed code for the wide-count refusal: N21c-39 |
| §21.22 | Invariants and proof obligations | N21c-27 (register of all fourteen with test names and planted violations); N21c-24 (INV-SPARK-01); N21c-28 and N21c-05 (INV-SPARK-02); N21c-19 (INV-SPARK-09); N21c-08 (INV-SPARK-10); N21c-29 (PO-SPARK-04, and the world-locality half of INV-SPARK-04) |
| §21.23 | Known answers and negative controls | N21c-30 (controls of rows 1 to 6, due M0); N21c-31 (controls of rows 7 to 12 and the wide-count overflow); values cited in N21c-06, N21c-09, N21c-19, N21c-20, N21c-25, N21c-26, N21c-36, N21-CX. The known-answer registry bead N21-KA is U21a's |
| §21.24 | Gates and kill gates | N21c-32 (all sixteen gates, eight kill gates and the mechanism register as registry rows); N21c-33 (KILL-METAL decision); measuring and deciding beads in (b) |
| §21.25 | Open questions | N21c-34 (OPEN-SPARK-1, owner); N21c-35 (OPEN-SPARK-2, owner); N21c-36 (OPEN-SPARK-3, experiment and freeze); N21c-37 (OPEN-SPARK-4, decision at the CUDA admission test); N21c-38 (OPEN-SPARK-5, owner); N21c-03 (OPEN-SPARK-6, experiment) |
| §21.26 | Counts beyond 2³² without 64-bit atomics (added late) | N21c-39 (no-wrap bound at plan time: admit, shorten, refuse); N21c-40 (drain into `u64` totals, zeroing, exactly once); N21c-41 (two-limb reduce); N21c-42 (no 64-bit atomic lint, feature gating, `FROB-REF-616`); N21c-43 (GATE-SPARK-23 end-to-end); N21c-02 (the 64-bit probes in the fingerprint); N21c-31 (the planted-overflow control) |
| Appendix N.18 | Counterexamples of §21 | N21-CX |
| Appendix O.6 | Refusal codes of §21 | N21-REF |

## (b) Gates, kill gates and the beads that measure or decide them

Gates whose mechanism is in this unit's subsections:

| Gate | Milestone | Measured or decided by | Mechanism and supporting beads |
|---|---|---|---|
| GATE-SPARK-07 | M0 | N21c-06 (script and record); BD-029 (U21a's seed, whose done-condition is this gate) | N21c-01, N21c-02, N21c-03, N21c-04, N21c-05 |
| GATE-SPARK-13 | M4 | BD-370 | N21c-08 (meter), N21c-09, N21c-12, N21c-13, N21c-14, N21c-15 |
| GATE-SPARK-23 | M2 | N21c-43 | N21c-39, N21c-40, N21c-41, N21c-42, N21c-31 |
| ✖ KILL-FLEET | M4 | N21c-11 (first reading, from the pilot N21c-10); BD-370 (final reading at GATE-SPARK-13) | |
| ✖ KILL-SPECIALIZE | M4 (profile clause at M2) | N21c-18 (first reading, from N21c-16 and N21c-17); BD-371 (U21a: final reading at GATE-SPARK-14) | N21c-19, N21c-20 |
| ✖ KILL-MEGA | M8 | N21c-22 (from the pilot N21c-21); epic BD-441 | N21c-23 exists only if it does not fire |
| ✖ KILL-METAL | M2 | N21c-33 (decision; reads GATE-SPARK-04 from BD-217 and the Metal determinism lanes) | N21c-38 (OPEN-SPARK-5) |
| GATE-WS-06 (a gate of §33.17, not of this chapter; only its clause "the Metal language version and the 64-bit atomic probe are recorded in the fingerprint") | M0 | The gate's own unit | N21c-02 produces the record that clause reads |

Gates of the chapter that this unit expects another unit to carry (each has a seed bead, so no extra measurement
bead was written; N21c-32 records every threshold, and N21c-30 and N21c-31 register their negative controls):

| Gate | Expected carrier (seed; unit) | This unit's supporting bead |
|---|---|---|
| GATE-SPARK-00 | BD-020 (U21b) | N21c-30 (contraction-prelude control) |
| GATE-SPARK-01 | BD-017 (U21b) | N21c-30 (fused-form control); N21c-02 records its outcome in the fingerprint |
| GATE-SPARK-02 | BD-176 (U21b) | N21c-28 (AE-08 consistency) |
| GATE-SPARK-03 | BD-217 (U21a) | N21c-35 (envelope), N21c-37 (graph route disclosed) |
| GATE-SPARK-04 | BD-217 (U21a) | N21c-33 (kill decision), N21c-35 |
| GATE-SPARK-05 | BD-179 (U21b) | |
| GATE-SPARK-06 | BD-175 (U21a) | N21-REF (typed refusals provoked end to end) |
| GATE-SPARK-08 | BD-030 (U21b) | N21c-30 (bare `f32` controls) |
| GATE-SPARK-09 | BD-178 (U21a) | N21c-31 (mixed-allowed counterpart) |
| GATE-SPARK-10 | BD-183 (U21b) | N21c-36 (bucket set frozen first) |
| GATE-SPARK-11 | BD-173 (U21a) | N21c-31 (cache corruption control) |
| GATE-SPARK-12 | BD-172 (U21a) | N21c-31 (fence-disabled control), N21c-37 |
| GATE-SPARK-14 | BD-371 (U21a) | N21c-19 (fence), N21c-20, N21c-31 (reassociated control) |
| ✖ KILL-KIR | BD-221 (U21a) | N21c-32 records trigger and fallback |
| ✖ KILL-BUCKET | BD-171 (U21b), BD-186 (U17b); the population chapter owns the decision | N21c-36 reports the "more than eight buckets" finding |
| ✖ KILL-D1 | BD-176 (U21b) | N21c-09 (replay labels on both branches) |
| ✖ KILL-SLOT | BD-295 (U32a); the daemon chapter governs | N21c-32 records both statements |

If U21a or U21b write no separate decision bead for KILL-KIR, KILL-D1 or KILL-BUCKET, the seeds above still carry
"decided" in their done-conditions and N21c-32 holds the full trigger and fallback text of each.

## (c) Counts

- Known answers: Appendix M lists 28 for §21 (KA-525 to KA-552); the registry bead N21-KA is U21a's. This unit's
  beads cite 21 of them with their values: KA-525 to KA-530, KA-532 to KA-539, KA-546 to KA-552. The three known
  answers of §21.26 (two-limb add; the 2³² boundary; the planted overflow reading 5) have no KA number and are
  carried with their values by N21c-39, N21c-41 and N21c-43.
- Counterexamples: 18 of 18 (CX-216 to CX-233) in N21-CX; ten of them are also executable calculations in
  N21c-25 and N21c-26.
- Refusal codes: 18 of 18 (`FROB-REF-601` to `-618`) in N21-REF, plus one proposed code for the refusal that §21.26
  states without a number.
- Invariants and proof obligations: 14 of 14 in N21c-27. Open questions: 6 of 6. Architecture decisions cited:
  ADR-SPARK-01, -02, -03, -07, -08, -09, -10 (ADR-SPARK-04 to -06 belong to U21b's subsections).

## (d) Needs from other slices

| Bead(s) | Need |
|---|---|
| N21c-01, N21c-02, N21c-03 | §21.16: the record type, canonical encoding and verb of BD-029 (U21a). BD-029 should depend on N21c-01 and N21c-02 (they are its probe subtasks) or absorb them; this unit did not add the reverse dependency, to avoid a cycle |
| N21c-05 | §21.10, §21.13, §36.8, §36.7: the cache, the check ledger, the estimator and the disclosure block must call the currency functions and key on the identity digest (BD-173, BD-176, BD-219, BD-216) |
| N21c-04, N21c-15, BD-370 | §32.12, §32.14: MCP tool generation and the dashboard's Devices panel |
| N21c-08, N21c-13, N21c-14, N21c-15, BD-370 | §32.11: the peer link between the two daemons [F], its paper pilot and OPEN-TETHER-5; no seed bead names it |
| N21c-09 | §19.19: GATE-HAB-02's cross-platform `SceneId` clause for interior archetypes (M3); §35.4: the decision-level test |
| N21c-10, BD-370 | §39.6: the phase ledger of `frob-chronicle`; §36.14: an approved envelope (E1 or E2) for the campaigns |
| N21c-16, N21c-17, N21c-19, N21c-20 | §21.18: BD-371 (U21a) should depend on N21c-17 (pilot) and N21c-18 (first reading), and use N21c-19 and N21c-20 as its fence and switch-over subtasks; §17.21: the limb-archetype pilot adds a fourth arm to N21c-17's harness |
| N21c-23 | §32.6: the option `--step-plan` on `sim.run` in the M8 verb set |
| N21c-27, N21-CX, N21-REF | §21.5 to §21.15: the tests and raising sites owned by U21a and U21b that these registers name |
| N21c-28 | §21.2: the doctrine's field names at the pinned FrankenSim revision; §37: the generic claim product at the boundary |
| N21c-33, N21c-38 | §43.5: the M2 exit review; BD-217's measurement of GATE-SPARK-04 after one optimization pass |
| N21c-35 | §36.14 and §36.16: the performance chapter restates OPEN-SPARK-2 and owns the envelope enforcement; one record should serve both |
| N21c-36, N21c-37 | BD-183 (GATE-SPARK-10) should depend on N21c-36; BD-172 should depend on N21c-37 |
| N21c-39, N21-REF | Appendix O: a code for "a device count could wrap within a chunk" (proposed `FROB-REF-619`) |
| N21c-40, N21c-41 | §29: the telemetry histograms and quantized channels (BD-207) as the first consumers |

## (e) Contradictions, gaps and what was done about them

1. **One digest, two requirements.** §21.16 defines one digest over the whole fingerprint record, which contains
   rates that are repeatable only within 5%; the same digest is said to key the kernel cache and check currency.
   A re-measurement would then discard caches. N21c-05 defines two values: `digest` (the whole record, cited by
   receipts) and `identity` (the discrete subset, used for "matches the machine", cache keys and currency). BD-029's
   author should adopt the identity subset or move rates to a side record.
2. **Bootstrap.** The fingerprint's `arithmetic` block cites the probe report, while every device result is supposed
   to cite a fingerprint. N21c-05 exempts the two measuring verbs (`spark.fingerprint`, `spark.probe`) from
   `FROB-REF-606`; their reports cite the live identity.
3. **"The standard campaign"** (GATE-SPARK-13) is not defined anywhere. N21c-10 defines it (the six-stage cascade on
   the seeded M4 conformance population for the reference quadruped class, frozen before the pilot) and BD-370
   reuses it.
4. **g.** The text says "non-training work under a third"; the ceiling formula is about work that can be *removed
   from the critical machine*. N21c-10 defines g as the movable share (by the role table), records the literal
   non-training share beside it, and N21c-11 decides on g in exact rational arithmetic.
5. **Comparator wording.** GATE-SPARK-13 compares with "the better single machine"; BET-35 with "the two RTX 4090
   alone". BD-370 measures the gate's wording and records which machine was better.
6. **A refusal without a code.** §21.26's "plan is refused" has no number. Proposed `FROB-REF-619` (N21c-39,
   N21-REF), marked as a proposal for the registry. Likewise the plan names no code for a regenerated scene whose
   identity differs (N21c-09 uses `FROB-REF-303`), for a device result that fails its own schema (N21c-28: an
   internal-family code to be assigned), for citing an exploratory device result (N21c-34 uses `FROB-REF-819`), or
   for a shape class that does not fit the fused plan (N21c-23 uses `FROB-REF-604`). Each choice is labelled DESIGN.
7. **Bucket set.** §21.11 and OPEN-SPARK-3 give {12, 18, 24, 36, 48, 64} (worst member 25, Ω₁ ≤ 1.44, Ω₃ ≤ 3.0);
   KA-547 and CX-222 quote a set with 32 (worst member 33 in the 48 bucket, 1.45 and 3.1); §42 calls the numbers
   link counts where §21 says degrees of freedom. N21c-36 treats the §21 default as governing, tests both hand
   values and freezes one set.
8. **Kill gates with clauses decidable before and after the mechanism exists** (KILL-FLEET, KILL-SPECIALIZE) are
   split into a first reading (a decision bead the work depends on) and a final reading inside the gate-measuring
   seed, to keep dependencies acyclic.
9. **Two seeds of U21a draw on this unit's subsections.** BD-029 (§21.16) and BD-371 (§21.18) are written by U21a;
   this unit wrote the surrounding subtasks (probes, `spark.devices`, currency, the gate script; share
   measurement, pilot, first reading, fence, switch-over) and two decision beads whose seeds already say
   "decided" (N21c-18 beside BD-371; N21c-33 beside BD-217 for KILL-METAL, which is defined in §21.24). Overlap
   with those seeds' bodies is possible and should be reconciled by making the seeds depend on these beads.
10. **OPEN-SPARK-3 is "decided by the M2 profile" and "blocks GATE-SPARK-10"**, but the profile seed (BD-217)
    depends on the variants seed (BD-183) that measures GATE-SPARK-10. N21c-36 decides from the fingerprint's
    launch cost and the first full-step timings, and re-reads once after the profile.
11. **`fleet.status` is registered at M4** but carries a Solid policy needed from M2. The role table and the meter
    land at M2 (N21c-07, N21c-08) and are readable through the estimate and export envelopes until the verb exists.
12. **Compute envelope against gate cost.** GATE-SPARK-13 needs ten paired campaigns on three devices; under the
    default E0 it cannot run. BD-370 sizes the campaign for E1, and records `not run` otherwise; the fleet pilot
    runs on one card for one hour under E0.
13. **Capacity overflow has two codes** (`FROB-REF-610` here; a `7xx` code in the dynamics chapter). N21-REF
    cross-references them and assigns 610 to the batched device path.
14. **"Exact constant operations"** in specialization is read as folds whose result needs no rounding (N21c-19), so
    that no backend's rounding behaviour can affect bit identity.
