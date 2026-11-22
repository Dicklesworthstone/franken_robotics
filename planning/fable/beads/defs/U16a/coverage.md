# Coverage of drafting unit U16a (§16 Kinesis, preamble and §16.1 to §16.7)

35 bead files: 10 seed beads (BD-081, BD-082, BD-088, BD-089, BD-150, BD-159, BD-169, BD-213, BD-384, BD-385),
24 new beads (N16a-01 to N16a-24) and the registry N16-KA.

## (a) Subsections and the beads that carry them

| Subsection | Title | Beads of this unit | Beads of other units that carry part of it |
|---|---|---|---|
| §16 preamble | Crates; what Kinesis calls through the airlock | N16a-01 | — |
| §16.1 | What Kinesis owns | N16a-01 (boundary, layering, the world-step and env-step units) | The trust-ladder items are §16.8 to §16.11 (U16b) |
| §16.2 | Decisions | N16a-01 (all seven ADR files); ADR-KIN-01 → N16a-02; ADR-KIN-02 → BD-081, BD-159, N16a-02; ADR-KIN-03 → N16a-03, N16a-12, BD-169; ADR-KIN-05 → N16a-04; ADR-KIN-06 → N16a-05; ADR-KIN-07 → BD-159, BD-150, BD-385, N16a-24 | ADR-KIN-04 governs the promotion rule (BD-263, BD-264, U16b) |
| §16.3 | One model, three tiers | N16a-02 (tiers, profiles, admission matrix, FROB-REF-222, "not ground truth"), N16a-03 (profile in the step-plan identity) | — |
| §16.4 | The referee as it can exist at M1, and where FrankenSim's limits bind | N16a-05 (the six limits, routing, adjudicator), N16a-08 (the audit pass with certified collision detection), BD-213 and N16a-20 (the coverage figure with its gate) | BD-080 (the referee step itself, U18a), BD-035 and BD-093 (admission tests), BD-061 (joint-class lowering) |
| §16.5 | The transactional step [S] | BD-081 (restore set, fence, driver T1 to T7, numerical fault), N16a-06 (time: base intervals, dyadic grid, L_max, E_max), BD-082 (T4.1 to T4.7, event location), N16a-07 (coupled impulses per island, units, FROB-REF-224), N16a-08 (checks T5), N16a-04 (retries change numerical parameters only), N16a-09 (OPEN-KIN-5), N16a-10 (end-to-end) | BD-083 (the retry ladder and the refinement law for random inputs with FROB-REF-223 and the redraw counterexample; U16c writes it from this subsection), BD-093 (T4.7 witness migration), BD-161 (the same transaction on `strict`), BD-098 (packet queues in the checkpoint), BD-094 (numerical-fault in the taxonomy) |
| §16.6 | The batched step [S], with per-world variants | BD-159 (ten stages, design rules, flags, fence at the batch boundary), BD-150 (stage 6 kernels), N16a-17 (parity harness), N16a-03 (identities), BD-169 (parameter variants), N16a-12 (why padding can be exact, INV-KIN-04), N16a-13 (path table), N16a-14 (capacities, byte model, `Degraded` rate), N16a-15 (OPEN-KIN-3), BD-385, N16a-23, N16a-24 (tree-operator route), N16a-18 and N16a-19 (end-to-end) | BD-170 (model variants, level P-B), BD-181 and BD-183 (device rows), BD-212 (battery record of GATE-KIN-07), stage kernels BD-149, BD-151 to BD-158 |
| §16.7 | Integrators | N16a-11 (default integrator, decimation), N16a-16 (OPEN-KIN-1), BD-384 (articulated variational integrator), N16a-21 (its pilot), N16a-22 (KILL-VARIATIONAL), BD-088 (dynamics known answers) | BD-087 (the free-body variational step as an oracle, U41), BD-157 (integration kernels) |

## (b) Gates, kill gates and the beads that measure or decide them

| Gate | Milestone (register) | Beads of this unit | Expected from another unit |
|---|---|---|---|
| GATE-KIN-01 | M1 | BD-088 (all M1 rows); BD-384 (the same rows on a tree, M5) | BD-087 (free-body oracle) |
| GATE-KIN-02 | M1 | BD-089 | BD-067 (oracle capture) |
| GATE-KIN-03 | M2 | N16a-17 (harness and gate assembly), BD-150 (inverse dynamics, mass matrix), BD-159 (forward dynamics; closes the record), N16a-18 | BD-149 (kinematics row) |
| GATE-KIN-04 | M1 (referee); M2 (`strict`) | BD-081 (fence row), BD-082 and N16a-07 (impact time, tied sliders), N16a-06, N16a-08, N16a-10 (script and referee record) | BD-083 (forced-retry digests, refinement statistics, redraw mutation; U16c), BD-161 (`strict` rows; U18a) |
| GATE-KIN-05 | M2 | N16a-12 (`batch-cpu` rows), N16a-13 (path-table clause), N16a-18 | BD-181 (per-backend rows; U35) |
| GATE-KIN-06 | M2 | BD-169, N16a-19 (`batch-cpu` rows) | BD-183 (device rows; U21b) |
| GATE-KIN-07 | M2 | BD-159 (every forced overflow flagged), N16a-14 (capacities, rate harness, disclosure), N16a-15 (capacities of the other workloads), N16a-18 | BD-212 (the rate on the frozen battery; U16b) |
| GATE-KIN-08 | M2 (§16.19) | — | U16c |
| GATE-KIN-09 | M8 (§16.20) | — | U16c |
| GATE-KIN-10 | M6 (§16.21) | — | U16c |
| GATE-KIN-11 | M2 (§16.22) | — | U16c |
| GATE-KIN-12 | M2 (§16.23) | — (BD-385 and N16a-24 reuse its operation counts) | U16c |
| GATE-KIN-13 | M2 (§16.24) | — (N16a-11 supplies the integrator it tests) | U16c |
| GATE-FID-01 | M2 | — (N16a-16 reads its thresholds) | BD-212 (U16b) |
| GATE-FID-02 | M3 | — | BD-263 (U16b) |
| GATE-FID-03 | M2 | BD-213, N16a-20 | — |
| GATE-FID-04 | M3 | — | BD-265 (U16b) |
| GATE-FID-05 | M3 | — | BD-266 (U16b) |
| GATE-FID-06 | M2 | — (N16a-08 holds a check-level fixture) | BD-208 (U29a), U16b |
| GATE-FID-07 | M2 | — | BD-214, BD-268 (U16b) |
| GATE-FID-08 | M3 | — | BD-264 (U16b) |
| KILL-TREEOP | M5 | BD-385 (measurement), N16a-23 (decision); N16a-24 depends on it | — |
| KILL-VARIATIONAL | M5 | N16a-21 and BD-384 (measurements), N16a-22 (decision) | — |
| KILL-SKEL-PORT (defined in §14.15; decided with the seed pilot) | M5 | BD-385 (PO-SKEL-04 and the pilot), N16a-23 (decision) | The §14 unit may also write a decision bead: merge |
| KILL-DR, KILL-RANK, KILL-SHADOW | M3, M4, M3 | — | U16b |
| KILL-ANCESTOR, KILL-LOOPELIM, KILL-STEPADJOINT (and KILL-ROWQUEUE, KILL-INTERACT) | with their gates | — | U16c |

Identifiers of the chapter carried by this unit: ADR-KIN-01 to -07 (N16a-01 and the governing beads);
INV-KIN-01 and -02 (BD-081), -04 (N16a-12), -05 (BD-159, N16a-14), -07 (N16a-04); PO-KIN-01 (BD-169, BD-159,
N16a-11), PO-KIN-02 and -03 (N16a-12); OPEN-KIN-1 (N16a-16), -2 (BD-159), -3 (N16a-15), -4 (N16a-02), -5
(N16a-09); RISK-31 (N16a-05, BD-213), RISK-40 (BD-159, N16a-14); BET-34 (BD-213); C-04, C-05, C-06, C-14, C-36
through the SPEC-C dependencies of BD-088, BD-150, BD-159, N16a-07, N16a-11, BD-384, N16a-21, BD-385, N16a-24.
Left to the units that own their mechanisms: INV-KIN-03 and PO-KIN-04 (BD-083, U16c), INV-KIN-06, INV-KIN-08,
PO-KIN-05 and OPEN-KIN-6 (U16b).

## (c) Counts

- Known answers: 27 in the registry N16-KA (KA-338 to KA-364), every row with its owning bead. 18 of them are
  restated with values in this unit's mechanism and test beads (KA-338, -339, -341, -342, -343, -344, -345,
  -346, -347, -348, -349, -354, -355, -356, -357, -358, -359, -360); the other 9 belong to other units'
  mechanisms (KA-340 to BD-083 of U16c; KA-350 to -353 and -361 to -364 to U16b). Also cited from other chapters' registries: KA-047 to -054,
  KA-172 to -174, KA-264, KA-396. N16-KA additionally lists 10 unnumbered known answers of §16.19 to §16.24.
- Counterexamples: 5 of the chapter's 9 are cited with their numbers in this unit's test plans (CX-144, CX-145,
  CX-146, CX-150, CX-151), plus CX-129 from §14. The registry N16-CX is U16c's.
- Refusal codes: of the chapter's 9, six have their trigger, repairs and exit code in this unit's beads
  (FROB-REF-220 in N16a-06 and BD-082; -221 in BD-081; -222 in N16a-02; -224 in N16a-07; -721 in N16a-05; -722
  in BD-159), -223 is referenced in BD-082 and owned by BD-083, and -820 and -821 are U16b's. FROB-REF-212
  (defined in §13) is carried in N16a-11. The registry N16-REF is U16c's.
- Sizes: the largest bead is BD-385 (221 lines) and the smallest N16a-19 (111 lines); N16-KA is the largest in
  bytes because of its tables.

## (d) Needs from other slices (no seed key could be named)

- §16.13 (U16b): the registry rows of `sim.run` (`--profile`, `--law`, `--designs`, `--integrator`,
  `--solve-path`), `sim.estimate` and `fidelity.coverage`. This unit's beads implement the mechanisms and name
  the flags as DESIGN spellings (`--on-overflow`, `--capacity`, `--jacobian`, `--adjudicate`, `--txn`,
  `--step`, `--newton-iterations`, `--select`, `--init`, `--workers`, `--chaos`, `--force-retry`).
- §16.14 (U16c, N16-REF): registry entries for FROB-REF-220, -221, -222, -223, -224, -721, -722.
- §16.8 and §16.11 (U16b): the label rule for `established` and its adjudicator; the `referee_coverage` receipt
  record.
- §16.23 and §16.24 (U16c): the solve-stage ledger whose measured shares BD-385 reads; the refusal for a
  refinement study that changes a clock.
- §18.4, §18.5, §18.11, §18.16, §18.20 (Traction): swept bounds with ε_g; the hard-contact law as the referee's
  specification; patch reduction with its fixture; the Painlevé rod; arcs in swept bounds.
- §20.4 (Sensorium): intra-step policies on throughput tiers; the boundary ordering rule on every tier.
- §29.5, §29.8, §29.9, §29.10 (Chronicle): telemetry query of epochs, events and checks; in-kernel digests;
  the created-energy rule; the numerical outcome kind.
- §36.2, §36.3, §36.7, §36.10 (Performance): unit ordering of rates; capacities of Q and H (OPEN-SPARK-3); the
  disclosure block; the memory model of `sim.estimate`.
- §38.5, §38.6, §38.8, §38.11 (Gauntlet): the pilot register; the known-answer and negative-control registries;
  the frozen standard battery.
- §14.6, §14.9, §14.15 (Skeleton): limb port packets; the path table; ownership of KILL-SKEL-PORT.
- §17.4, §17.5, §17.15, §17.16 (Phylum): `ShapeClass`; the 64 bucket; the sweep file; FROB-REF-160.
- §33.9 (Suite): admission of FrankenSim's discrete Euler–Poincaré step at the M5 pin.

## (e) Contradictions and things that could not be turned into work as written, and what was done

1. **Capacities in the known-answer suite.** KA-348 prints the quadruped at C 24, R 128 (27 kB) and the humanoid
   at C 40, R 224 (72 kB); §16.6, §36.3 and §36.10 print C 32, R 160 (31 kB) and C 64, R 320 (92 kB), and the
   plan's own note says §36.3 governs. KA-348 also prints 37 kB for the tracked row where the formula gives
   36,464 bytes and §16.6 prints 36 kB. Done: the formula is the truth; N16a-14 and N16-KA test both capacity
   sets and record the discrepancy.
2. **The canonical zero.** §16.6, PO-KIN-03 and §35.5 say padding for sums is −0.0 and that +0.0 is not neutral.
   CX-146 words the rule as "canonical zero +0", and the seed done-condition of BD-152 reads "padding a
   reduction with −0.0 as the neutral element is caught". Done: N16a-12 follows the chapter and the proof
   obligation (−0.0 neutral; +0.0 is the planted defect). The lead should correct the two wordings.
3. **Referee coverage threshold.** GATE-FID-03, BET-34 and §16.11 say 90%; CX-151 and RISK-31 say 80%. Done:
   90% (the register governs), noted in BD-213.
4. **Label under a `strict` adjudicator.** §16.8 and §16.11 let `established` name `batch-cpu:strict`; BET-34's
   fallback column says results there "stay provisional and say so". Done: BD-213 reports the adjudicator and
   leaves the label to the promotion bead; the owner should rule.
5. **Impact time within 10⁻⁹ s against a 31 ns grid.** With L_max = 16 and a 2 ms base interval the event grid
   resolves 31 ns, coarser than the gate's 1 ns. Done: BD-082 records two times per event (the exact grid node,
   used by dynamics, and a refined root inside the final cell, used by the gate), tests both on a fixture whose
   impact falls on a node, and N16a-09 puts L_max = 21 before the owner as the alternative.
6. **`continue-flagged` against "no contact is removed to fit a capacity".** Fixed-shape buffers cannot hold
   more than C contacts, so continuing means solving with at most C. Done: BD-159 flags the world before the
   solve, makes the kept set a deterministic function of the world, keeps the flag for the episode, excludes
   `Degraded` world-steps from verdicts, and defaults evaluation verbs to `refuse`.
7. **OPEN-KIN-5 names "the standard battery at M1"**, but the battery is frozen at M2. Done: N16a-09 decides on
   the M1 fixtures and re-reads once on the battery at M2.
8. **GATE-FID-03 asks for promotion wall-clock at M2**, but the promotion verb lands at M3. Done: BD-213
   measures a 16-episode proxy on the adjudicating tier and labels it as such.
9. **Closed forms named and not written.** The pendulum period and the precession rate are cited "from the
   earlier gate". Done: BD-088 and N16-KA state the standard forms (T = (4/ω₀)·K(sin(θ₀/2));
   I₁·Ω²·cos θ − I₃·ω₃·Ω + m·g·ℓ = 0) for the derivation files to confirm.
10. **Known answers without numbers.** §16.19 to §16.24 state known answers that Appendix M does not number.
    Done: listed in N16-KA as pending rows for U16c's beads to register.
11. **FROB-REF-222 names only the GPU case.** Three other profile and tier pairs are also outside the table
    (`referee` on `batch-cpu`; `lockstep` or `strict` on `referee`). Done: N16a-02 refuses them under the same
    code, so that no pair is silently remapped.
12. **Stale bead number.** §38.8 says the standard battery is frozen "at M2 (BD-104)"; BD-104 is the
    self-collision analysis and the battery bead is BD-212. Done: beads cite BD-212.
13. **Bucket sets differ.** Appendix I prints degree-of-freedom capacities {12, 18, 24, 32, 48, 64}; §17.5 and
    §21.11 print {12, 18, 24, 36, 48, 64}. Not this unit's to settle; N16a-12's tests pad to 24, 36, 48 and 64.
14. **Possible duplicate decision beads.** OPEN-KIN-1, -3 and -5 are listed in §16.18 (U16c's subsection) but
    concern mechanisms of §16.5 to §16.7, so this unit wrote their decision beads (N16a-16, N16a-15, N16a-09);
    KILL-SKEL-PORT is owned by §14.15 but decided by seed BD-385, so N16a-23 records it. If U16c or the §14
    unit wrote the same decisions, merge them.
15. **Seed dependencies extended.** BD-159's seed list omits BD-150 and BD-153 (kernels it runs) and BD-116 (the
    baseline controller its done-condition needs); they were added. BD-385 gained BD-217 (the M2 profile its
    trigger reads) and BD-144 (the bucket population). Every seed dependency was kept.
