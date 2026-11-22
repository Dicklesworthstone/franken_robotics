# Coverage of drafting unit U12 (§12 Lemma; theorem cards T-01 to T-35; Appendix D)

92 bead files: 5 seed beads (BD-132, BD-364, BD-379, BD-381, BD-382), 35 theorem cards (CARD-T-01 to CARD-T-35),
3 registries (N12-KA, N12-CX, N12-REF), 39 new Lemma beads (N12-01 to N12-39) and 10 compendium beads (N12-41 to
N12-50; N12-40 is unused).

## (a) Subsections and the beads that carry them

| Subsection | Title | Beads |
|---|---|---|
| §12 preamble | What Lemma owns; decisions ADR-LEMMA-01 to -06; the five verbs | N12-01 (ADR-LEMMA-01, -02), N12-03 (ADR-LEMMA-03), N12-02 (ADR-LEMMA-04), BD-382 and CARD-T-35 (ADR-LEMMA-05), N12-13 (ADR-LEMMA-06) |
| §12.1 | What the two proof tools can do today | N12-01 (capability registry, caps, route selection, re-audit); N12-23 (closure and memory measurement); BD-381 (accepted and blocked modules as rules of the Lean lane) |
| §12.2 | Proof classes, family certificates and the claim product | N12-08 (the four classes, Proof wording, the banned word); N12-05 (family certificates, class cap, polarity); N12-11 (claim product and flags: GATE-LEMMA-06); N12-03 (flag rule in bundles); N12-04 (class of a bundle set) |
| §12.3 | Theorem cards | N12-06 (card registry and admission rule); CARD-T-01 to CARD-T-35 (one bead per card); N12-07 (the twelve obligations no checker decides); N12-39 (general statements of dual-route cards); N12-22 (budget pilot for the Frontier cards) |
| §12.4 | Mechanics | Instance certificates: N12-20, BD-379, N12-33. Lean theorems: BD-381, N12-24, N12-23, N12-39. Bundle schema and format rules: N12-03. Proof-kind registry: N12-02. Checkers: BD-132, N12-13 to N12-19 (with BD-027 for `poly-identity` and `rational-ldlt`, owned by U25b). Auditor contract: N12-04, N12-09, N12-10. Stance mutations: BD-132, N12-34, CARD-T-22. Kernel reflection: BD-382, N12-26, N12-27, N12-28, CARD-T-35, N12-38. Multi-claim answers and second-machine re-check: BD-364, N12-35 |
| §12.5 | What Lemma does not claim | N12-08 (the six non-claims as an output scope block); N12-07; N12-12 (the sixth non-claim, measured) |
| §12.6 | Gates | See table (b) |
| §12.7 | Refusals, invariants and known answers | N12-REF (eight codes); N12-KA (KA-207 to KA-227); N12-CX (CX-106 to CX-115); invariants and proof obligations: see the identifier table below |
| §12.8 | Open questions | N12-23 (OPEN-LEMMA-1), N12-26 (OPEN-LEMMA-2), N12-27 (OPEN-LEMMA-3), N12-29 (OPEN-LEMMA-4), N12-12 (OPEN-LEMMA-5), N12-25 (OPEN-LEMMA-6) |
| Appendix D.1 | Groups, frames and inertia (21 rows) | N12-41 |
| Appendix D.2 | Time stepping and hybrid events (12 rows) | N12-42 |
| Appendix D.3 | Contact, wheels and terrain (10 rows) | N12-43 |
| Appendix D.4 | Actuators and power (14 rows) | N12-44 |
| Appendix D.5 | Symmetry, similarity and counting (10 rows) | N12-45 |
| Appendix D.6 | Missions, information and certificates (15 rows) | N12-46 |
| Appendix D.7 | Estimation, control and safety (7 rows) | N12-47 |
| Appendix D.8 | Statistics and decisions (19 rows) | N12-48 |
| Appendix D.9 | Time to an answer, speed-up bounds and capacity (14 rows) | N12-49 |
| Appendix D.10 | Structure and life (4 rows) | N12-50 |

Identifiers of §12 and where they are carried:

| Identifier | Bead |
|---|---|
| INV-LEMMA-01 | N12-08 (export check), N12-06, N12-11 (sweep) |
| INV-LEMMA-02 | N12-11 (compile-fail and property tests), N12-05 |
| INV-LEMMA-03 | N12-11 (export sweep), N12-03 (auditor flag rule) |
| INV-LEMMA-04 | N12-02 |
| INV-LEMMA-05 | N12-09 |
| INV-LEMMA-06 | BD-381, N12-24, N12-37 |
| PO-LEMMA-01 | N12-04 (round trip), N12-34 |
| PO-LEMMA-02 | N12-10 |
| PO-LEMMA-03 | N12-05 |
| PO-LEMMA-04 | N12-10 |
| Verbs `prove.kernel`, `prove.verify`, `prove.list` | Implemented by BD-027 (U25b); exercised by N12-33; `prove.list` rows and options defined in N12-06, N12-12, BD-379 |
| Verb `prove.bundle` | Implemented by BD-133 (U25b); extended to M4 kinds by BD-364; exercised by N12-35 |
| Verb `prove.reflect` | BD-382 (pilot form), N12-28 (budgeted admission), N12-38 |
| Proof kinds of `frob.cert.kinds/1` (27) | Registry N12-02; checkers: BD-027 (2, U25b), BD-132 (7), N12-13 (1), N12-14 (4), N12-15 (1), N12-16 (4), N12-17 (2), N12-18 (3), N12-19 (2), BD-381 (1) |

## (b) Gates and kill gates

| Gate | Milestone | Measured or decided by |
|---|---|---|
| GATE-LEMMA-00 | M0 | N12-33 (end-to-end record); CARD-T-01, CARD-T-03, CARD-T-04 (the three cards); mechanism in BD-021 and BD-027 (U25b) |
| GATE-LEMMA-01 | M5 | BD-379 (measurement); N12-36 (end-to-end record) |
| GATE-LEMMA-02 | M5 | BD-381 (mechanism and first three cards); N12-37 (end-to-end record) |
| GATE-LEMMA-03 | M1 | BD-132 (checkers and per-kind condition); N12-34 (end-to-end record, re-recorded as kinds are added) |
| GATE-LEMMA-04 | M1, repeated at M4 | N12-35 (end-to-end record at M1; mechanism in BD-133, U25b); BD-364 (M4) |
| GATE-LEMMA-05 | M5 | BD-382 (pilot); N12-38 (end-to-end record) |
| GATE-LEMMA-06 | M0 | N12-11 (compile-fail suite, export sweep, record); mechanism in BD-005 (U34), BD-021 and BD-027 (U25b) |
| KILL-FAMILYCERT | M3 | N12-30 (decision) |
| KILL-LEMMA | M5 | N12-31 (decision); measurement in BD-379; fallback mechanism N12-21 |
| KILL-REFLECT | M5 | N12-32 (decision); measurement in BD-382 and N12-26 |

No admission gate is defined in §12 (INT-FSYM-01 and INT-FLN-01, -02 belong to §33 and are cited as prerequisites).

## (c) Counts

- Known answers carried: 21 (KA-207 to KA-227), all in N12-KA and cited in the mechanism beads.
- Counterexamples carried: 10 (CX-106 to CX-115) in N12-CX; six further rows of other chapters that Lemma's
  checkers enforce are cross-referenced there (CX-037, CX-045, CX-075, CX-077, CX-096, CX-097).
- Refusal codes carried: 8 (FROB-REF-168, -263, -823, -826, -829, -833, -834, -835) in N12-REF.
- Theorem cards: 35. Compendium rows: 126 across ten sections.
- Subsections: 8 of §12 (plus the chapter preamble) and 10 of Appendix D.

## (d) Needs from other slices

Prerequisites owned by other units, as written in the `needs` fields (a candidate key is given where one was
found in the seed index; the lead resolves the rest):

- §25.5.1, §25.5.2: mission proof objects and the family payload writer (BD-128, BD-133); §25.7 cell refinement
  (BD-357); §25.3 `mission.synthesize` on the slice fixtures (BD-135); §25.22 generic comparators with recorded
  costs (BD-127, BD-129, BD-316); §25.4.6 monotone transport (BD-288); §25.31 the monomial screen (a U25e bead
  with no seed key); §25.6 and §25.34 region certificates (BD-289, BD-358); §25.16 product dominance (BD-359);
  §25.21.4 the continuation lane and its pilot (BD-437); §25.8 the revalidation engine (BD-292).
- §21.5, §21.7: the manifest field for declared identities, the `proof_digest` slot, square-root and select
  nodes, canonical program text, reserved fixed-point types (BD-014, BD-018); the kernel library of M2; DC-11 and
  the class-E rule table (BD-349, BD-371).
- §32: the run-verb envelope with idempotency key, budget and exit code 8 (BD-022); a refusal code for a parked
  verb and for an over-budget reflection job (none is defined in the Lemma table; see (e) item 8).
- §33: pins of frankensympy and franken_lean in the stack lock (BD-031); `fs-blake3` through the airlock (BD-006);
  frankensympy, `fs-checker` and the capsule consumer reachable from the conformance crate for re-check only.
- §34: registry loader and schema check (BD-004, BD-005); the FMT-FROBCERT format row (BD-141).
- §38: the known-answer, counterexample and negative-control registries (BD-025, BD-146).
- §13.4, §13.6, §17.11, §20: signed-permutation representations, orbits and Burnside counts, Γ_obs and its action
  (BD-048, BD-049, BD-102, BD-241, BD-285).
- §14, §14.5, §11.5: compiled wiring of a design, geometry-derived inertia with the shell rule, mass
  plausibility bounds (BD-054, BD-055, BD-061, BD-062).
- §15.6, §18.5, §18.6: contact laws on lockstep tiers, joint-level rows, the cone kernels (BD-050, BD-080, BD-084,
  BD-153, BD-155, BD-167).
- §19.11: clearance complexes and their proof objects (BD-233). §22.5, §22.9, §22.11, §22.12.5: conic
  certificates, the gait atlas, the `.fenergy` artifact, reachable-set tables (BD-115, BD-251, BD-332, BD-337,
  BD-440). §24.12: study receipts (BD-124). §28: certified inverse kinematics and the exact linkage lane (BD-361).
  §31.13: the firmware exporter (BD-438). §35: addressed random draws (BD-180).
- §39, §28.18: the phase ledger of the ten-fold test and the campaign report that print the decision-effect
  count (BD-302, BD-314, BD-317).
- §0.7 pilots owned elsewhere that Frontier cards rely on: the gait pilot (CARD-T-25), the derivation pilot
  (CARD-T-27), the information pilot (CARD-T-30), the dominance pilot (CARD-T-32).
- Beads that extend specialized mission projections at M4 should depend on N12-30 (KILL-FAMILYCERT); bundle
  writers of other units (BD-133, BD-337, BD-359, BD-383) should depend on the checker bead for the kind they emit
  (N12-13 to N12-19) and on N12-05 for the family-certificate type.
- For `EPIC-PROGRAM`: the three plan-level rules restated by Lemma beads and not checkable by one bead alone:
  the word "verified" is not used in any output (N12-08 scans Lemma's surface; other surfaces need the same
  scan); every exported artifact carries the three flags (N12-11 sweeps what the conformance suite exports;
  BD-303 extends it); a kill gate is never quietly re-scoped (stated in N12-30, N12-31, N12-32).

## (e) Contradictions, gaps and what was done

1. Boundary with U25b. The seed beads that build the Lemma spine at M0 and M1 (BD-021, BD-026, BD-027, BD-133,
   BD-136) are written by U25b, and BD-005 by U34. To lose nothing from §12, this unit wrote specification-level
   beads for the registry of kinds (N12-02), the schema and generic rules (N12-03), the report contract (N12-04)
   and the family-certificate type (N12-05), each scoped to what those seeds do not state. Some overlap with the
   U25b beads is likely and should be reconciled by `related` links or by merging scope, not by dropping either.
2. Gate records. Each gate is named in the `gates` field of the bead that builds or measures the mechanism and
   of the end-to-end bead that writes the record. To avoid a deadlock between "the seed bead is done when the
   gate passes" and "the test bead depends on what it tests", the four affected seed beads carry a closing rule:
   they close on their own tests, and the gate's record is written by the dependent end-to-end bead.
3. OPEN-LEMMA-1 is said to block INT-FLN-01, yet measuring a closure needs `fln` built, which is INT-FLN-01's
   own content (BD-380, U33a). Resolution: the measurement on the trivial theorem is taken inside BD-380; N12-23
   depends on BD-380 and extends the measurement to the closures real cards need, and BD-381 depends on N12-23.
4. Cards whose route has no proof kind in the closed registry: T-09 (decided by the kernel-IR type checker),
   T-11 (exact rational linear algebra), T-14 (bit-vector decision), T-26 part (a) (enumeration of reachable
   verdicts) and T-29's graph check. Each card bead states how the result is carried (a checker verdict bound
   to the kernel hash for T-09; linear identities under `poly-identity` with a typed witness for T-11 and T-29;
   a capsule-style reference for T-14; the `sample-coverage` receipt or a host test for T-26), marked DESIGN,
   with the note that a dedicated kind would be a registry version bump.
5. Class of `sample-coverage`: §12.4 says "class at most Tested" and Appendix I.4 says "Estimated; never a
   proof" for mission answers. Carried as: cap `Tested`; an empirical bound inside a mission answer is
   `Estimated` (N12-02, N12-13).
6. No per-card milestone exists in the plan beyond GATE-LEMMA-00 (T-01, T-03, T-04 at M0) and the M5 gates.
   Milestones were assigned from the earliest consumer and the route's tier: M0 (T-01, T-03, T-04, T-09), M1
   (T-02, T-05, T-11, T-15, T-16, T-19, T-21, T-22, T-23, T-26, T-29, T-30), M3 (T-20, T-31), M4 (T-24, T-25,
   T-27, T-28, T-32), M5 (T-06, T-07, T-08, T-10, T-12, T-17, T-18, T-33, T-35), M8 parked (T-13, T-14, T-34).
   The general Lean halves of dual-route cards are split out (BD-381 for T-12, T-15, T-26; BD-382 for T-21;
   N12-39 for T-01, T-03, T-23 and conditionally T-24) so that the instance halves can close early.
7. No work package owns `frob-cert` or `frob-lemma` at M3. Parents used: WP-54 (N12-13), WP-59 (N12-14, N12-30),
   WP-53 (N12-15, CARD-T-31), EPIC-M3 (CARD-T-20). Moonshot items use EPIC-M8 or WP-80.
8. Not specified in the plan and fixed here as DESIGN, to be confirmed by the lead: exit codes of the standalone
   `frob-verify` (aligned with the `frob` exit table: 0, 2, 3; never 1); the Merkle-root construction; the
   rule-identifier catalogue; the axiom whitelist's initial content; the evaluation budgets of the two pilots
   (600 s, 16 GiB) and the caps of the auditor's small group engine; the outcome when exactly one reflector meets
   GATE-LEMMA-05 (the kill fires as written); the definition of "equal cost" for KILL-FAMILYCERT; the
   counterfactual used by the decision-effect ledger. No refusal code exists in §12 for a parked verb or an
   over-budget `prove.reflect` job; the beads say "refused" without a code and need one from the Tether registry.
9. CARD-T-20 (M3) needs the budget pilot N12-22, which is scheduled at M4 before WP-31 is funded. To avoid a
   dependency on a later milestone, the card runs the pilot's arm E itself on its one fixture first.
10. The auditor's size cap (12,000 lines, TARGET) has to hold 27 checkers, Taylor-model arithmetic and a small
    permutation-group engine. The risk is recorded in N12-04, N12-16 and N12-18; a kind that would break the cap
    stays `not-implemented`.
11. Appendix D has no milestone or owner in the plan. All ten compendium beads are placed at M1 under WP-14 as
    oracle libraries built ahead of their consumers; every row is carried (126 rows), and the 34 rows with no
    construction are marked `compendium-only` (3 in D.1, 4 in D.2, 2 in D.3, 7 in D.4, 4 in D.8, 14 in D.9 and none
    in D.5, D.6, D.7, D.10), so no row depends on a specification bead existing. Some rows were given small
    worked numbers that are not in the plan (labelled as synthetic fixtures or computed from the row's formula).
12. KA rows with "none stated" as negative control (KA-210, KA-222, KA-223, KA-224, KA-225, KA-226): N12-KA lists
    them and adds a control for KA-210; the differential and round-trip beads add planted-defect builds for the
    others.
13. FROB-REF-168 and FROB-REF-263 are listed in §12.7 but are raised by the model compiler and the
    event-derivative code. N12-REF registers them with their triggers and repairs and leaves the raising
    mechanisms to their owners (see (d)).
