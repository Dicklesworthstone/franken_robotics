# Coverage of drafting unit U13 (§11 Conventions, §13 Marrow: the algebraic core)

Bead files in this directory: 9 seed beads (BD-010, BD-011, BD-012, BD-045, BD-046, BD-047, BD-048, BD-182, BD-325), 37 new beads (N13-01 to N13-37) and 5 registry beads (N11-KA, N11-CX, N13-KA, N13-CX, N13-REF). Total 51.

## (a) Subsections and the beads that carry them

| Subsection | Title | Beads |
|---|---|---|
| §11 (preamble) | One convention per concept, one boundary each; checked against `fs-ga` at M0 | N13-01, N13-07, BD-010 |
| §11.1 | The table | N13-01 (the 21 rows verbatim, with boundary and owner; Marrow rows at M0), N13-07 (all rows, every tier, M2); per row: BD-010, BD-047 (rows 2 to 5), N13-02 (6), BD-010 and N13-09 (7), BD-012 and N13-35 (8), N13-04 (10), N13-05 (11), N13-06 (13 to 15), N13-10 (16), N13-12 (units in buffer schemas) |
| §11.2 | Adjoint, coadjoint and the Euler–Poincaré sign | BD-010 (`coad`, formulas, bit parity), BD-011 (sign fixtures, laws), N13-01 (formulas in the conventions document), N13-18 (mutation M1), N11-KA, N11-CX |
| §11.3 | Three typed maps between frames | BD-047, N13-18 (mutations M2, M3), N11-KA, N11-CX |
| §11.4 | Reflection parity | N13-02, N13-15 (exact form), N13-16 (induced signed permutations), N11-KA, N11-CX |
| §11.5 | Inertia: pseudo-inertia, shells and mass support | N13-04 (fixtures and FROB-REF-168), N13-03 (decision on the plausibility rule), N13-29 and N13-30 (affine-invariant operations); mechanisms owned elsewhere: BD-051, BD-062 |
| §11.6 | Curvature of the connection | N13-05, N13-18 (mutation M7), N11-KA, N11-CX |
| §11.7 | Rational clocks and phase accumulators | N13-06 (rules in full, known answers, controls), N13-07 (every tier); mechanism owned elsewhere: BD-078, BD-098, BD-189 |
| §11.8 | The seven instants | N13-06, N13-07; mechanism owned elsewhere: BD-078, BD-098 |
| §13.1 | What it owns | BD-010 (`frob-lie` crate and scope), N13-13 (`frob-algebra` crate and scope), N13-10 (predicate classes), N13-02 (isometry type), N13-12 (layouts) |
| §13.2 | Decisions | ADR-MARROW-01: BD-010; -02: BD-011; -03: BD-012; -04: N13-12; -05: BD-011, BD-047; -06: N13-15, N13-16; -07: BD-325, N13-14; -08: N13-10, BD-182; -09: N13-02 |
| §13.3 | Conventions fixed here: the coadjoint, the wrench maps, reflections | BD-010 (component table, PO-MARROW-01 extended to four rows), BD-011 (the fix; why energy does not catch the error; small angles; chart cuts), BD-047 (typed maps), N13-02 (reflections, mirrored joints) |
| §13.4 | The mechanical group and the observation-preserving subgroup | N13-15 (Γ_mech: steps 1 to 5, data layout, three routes), N13-16 (steps 6 to 8, Γ_obs, breakers, stabilizers, table of uses, counterexamples), N13-21 and N13-22 (end to end); verb owned elsewhere: BD-102 |
| §13.5 | Equivariant linear maps by irreducible blocks | BD-048 (characters, projectors, Reynolds), N13-14 (verb, typing, counts), BD-325 (block bases, layer, PO-MARROW-05), N13-23 (pilot), N13-24 (kill decision), N13-20 (end to end) |
| §13.6 | Orbits, subgroup lattices and wreath products | N13-17 (specification as conformance battery and e2e of `algebra.orbits`), N13-25 (wreath pilot), N13-26 (nested tying); mechanism owned elsewhere: BD-049 |
| §13.7 | Exact predicates and guarded single precision | N13-10 (classes, exact kernel, guarded reference, bit budgets), N13-11 (Marrow-owned predicates), BD-182 (devices), N13-31 (limb width), N13-32 and N13-33 (kill gate and fallback); kernel side owned elsewhere: BD-053, BD-030 |
| §13.8 | Laws | N13-08 (harness, boundary set, ulp metric), N13-19 (the whole table, one record), N13-18 (mutation rule); per law: BD-010, BD-011, BD-012, BD-045 (group axioms, retraction, round trip, adjoint, coadjoint pair, free body, Jacobians), BD-047 (pairing), BD-046 (subgroups), N13-02 (reflections), BD-048 (representations), N13-16 (nested groups), BD-325 (blocks), N13-17 (orbits), N13-10 and N13-11 and BD-182 (predicates), N13-05 (curvature sign); cones owned elsewhere (BD-050, BD-154) and measured in N13-19 |
| §13.9 | Refusals | N13-REF; raised by BD-011 (212, 213), BD-048 and N13-14 and N13-15 (214), N13-10 and N13-11 and N13-13 (215) |
| §13.10 | Proof obligations, known answers, gates, open questions | PO-MARROW-01: BD-010; -02: N13-12; -03: BD-012; -04: N13-16; -05: BD-325; -06: BD-182 (tested half; certified half owned by BD-053). Known answers: N13-KA. Gates: table (b). Frontier items and pilots: N13-23/BD-325, N13-25/N13-26, N13-27/N13-28, N13-29/N13-30. OPEN-MARROW-1: N13-09; -2: Considerations of N13-13 and N13-15 (blocks nothing); -3: N13-31 |
| §13.11 | A circle-valued joint needs periodic laws, or its turn count (late subsection) | N13-34 (typing rule, declarations, refusal), N13-35 (turn count as referee state), N13-36 (GATE-MARROW-06 referee rows), N13-37 (device rows, replay, teleport, reset cache), BD-012 (the counter), N13-11 (the branch predicate) |

## (b) Gates, kill gates and admission gates

| Gate | Milestone (register) | Measured or decided by |
|---|---|---|
| GATE-MARROW-01 | M0 (SO(3), SE(3)); complete at M1 | BD-011 (M0 rows and record), N13-19 (complete record at M1); rows supplied by N13-08, BD-012, BD-045, BD-046, BD-047, N13-02, N13-05, N13-10, N13-17; decision cited: N13-09 |
| GATE-MARROW-02 | M1 | BD-048 (criterion at library level), N13-14 (verb), N13-20 (end-to-end record) |
| GATE-MARROW-03 | M3; rows for the first four archetypes at M1 | N13-21 (M1 rows), N13-22 (complete record); computation N13-15, N13-16 |
| GATE-MARROW-04 | M4 | BD-325 (both clauses recorded), N13-18 (mutation clause measured at M1) |
| GATE-MARROW-05 | M2 | BD-182; decision cited: N13-31 |
| GATE-MARROW-06 (not in the register's tables; indexed among late gates) | M1, device and teleport clauses at M2 | N13-34 (declaration rows), N13-35 (state rows), N13-36 (referee record), N13-37 (complete record) |
| KILL-MARROW-BLOCKS | M4 | N13-24 (decision), BD-325 (inputs) |
| KILL-MARROW-GUARD | M2 | N13-32 (decision), BD-182 (inputs), N13-33 (fallback, only if fired) |

No admission gate (`INT-…`) is defined in these chapters. PO-MARROW-01 relies on INT-FSIM-01 (BD-006) and is the consumer-side counterpart of INT-FSIM-02 (BD-032).

## (c) Counts carried

- Known answers: 16 for §11 (KA-191 to KA-206, in N11-KA) and 33 for §13 (KA-228 to KA-260, in N13-KA), 49 numbered rows; plus 4 unnumbered answers of §13.11 recorded in N13-KA as late slots.
- Counterexamples: 8 for §11 (CX-098 to CX-105, in N11-CX) and 9 for §13 (CX-116 to CX-124, in N13-CX), 17 numbered entries; plus the unnumbered §13.11 counterexample recorded in N13-CX.
- Refusal codes: 4 (FROB-REF-212, 213, 214, 215, in N13-REF); plus one refusal stated without a code in §13.11 (candidate FROB-REF-127) and the cross-referenced FROB-REF-168.
- Other identifiers: ADR-MARROW-01 to -09 (9), PO-MARROW-01 to -06 (6), OPEN-MARROW-1 to -3 (3), 4 Frontier items with 4 pilots, 17 laws, 8 planted mutations, 3 verbs (`analyze.symmetry`, `algebra.irreps`, `algebra.orbits`).

## (d) Needs from other slices

Dependencies the lead should add to beads owned by other units (I could not edit them):
- BD-102 (`analyze.symmetry`, U20b) should depend on N13-15 and N13-16, which hold the §13.4 computation it exposes; BD-241 (`sense.symmetry`) inherits that.
- BD-050 and BD-052 (U22a) depend only on BD-005 in the seed; they need the `frob-algebra` crate foundation, N13-13. BD-049 gets it through BD-048.
- BD-053 (U21a) should depend on N13-10 (predicate classes, exact reference evaluator, bit-budget rule) in addition to BD-030.
- BD-243 (A-09, U33b) should depend on N13-05 and call its `curvature_form`.
- BD-056 (`mirror`, `radial`, U41) should depend on N13-02 (the parity rule and `mirror_joint`).
- BD-061 (`frob-model`, U14a) consumes BD-046 (joint subgroups) and BD-047 (frames); BD-114 (invariant filter, U22a) consumes BD-045 (SE₂₊ₖ(3)); BD-107 (A-12) consumes BD-048's projectors.
- BD-032 (INT-FSIM-02, U33b) shares the reversed-adjoint fixtures with N13-18.

Prerequisites for which I could name no key (also in the `needs` fields):
- §15.5: a passive linear torsion spring element on a revolute joint, on the referee (N13-34, N13-36).
- §20.5: the exposure accumulator of a force sensor on the referee (N13-06).
- §20.12: `sense.alias` on two enumerated worlds that differ only in a joint's turn count, with single-turn and multi-turn encoder cards (N13-36).
- §14.6: the module port contract as comparable data (N13-26). §14.11: `design.check` stage 1 as the place where declaration refusals are raised (N13-34).
- §34.14: the canonical reader and writer (rationals as reduced pairs, integers above 2⁵³ as strings, declared interval implementation) (N13-07). §21: the kernel buffer schema that records the dimension of every raw `f32` buffer (N13-07).
- §35.11 and Appendix O: a code for the refusal "lifted law on a joint without turn state" from the owner of `registries/refusals.toml` (N13-REF, N13-34).
- An oracle-generator location under `tools/oracle/` for character tables and symmetry groups on file (BD-048, N13-21, N13-22) and a `tools/pilots/` location for development-time pilot scripts (N13-23, N13-25, N13-27, N13-29).
- Reserved keys used in `deps`: STD-E2E, STD-TEST, SPEC-C-01, SPEC-C-02, SPEC-C-03, SPEC-C-05, SPEC-C-08, SPEC-C-09, SPEC-C-22, WP-03, WP-07, WP-10, WP-14, WP-29, WP-38, WP-40, WP-46, WP-51, WP-68, EPIC-M2, EPIC-M3. CARD-T keys appear only in `related`.

## (e) Contradictions, gaps and what was done

1. **Solid fill.** §11.5 says a body without a wall thickness is solid and that an over-heavy solid is refused (FROB-REF-168) against "the plausibility bound of its part class"; §14.5 says there is no default fill (ADR-SKEL-03, FROB-REF-105) and that an over-heavy solid gets an advisory at one quarter of `mass_max`. No part classes or bounds exist anywhere. Done: decision bead N13-03 with a fail-closed default (written `solid` above one quarter of `mass_max` is refused unless marked intended), implemented provisionally by N13-04.
2. **FROB-REF-168** is listed as defined in §12.7 while its rule is in §11.5; carried in N13-04 and cross-referenced in N13-REF.
3. **Type name.** §13.3 calls the adjoint's type `AdjointMap`; §11.3 and the type sketches call it `TwistMap`. Done: one type `TwistMap`, with `AdjointMap` as an alias (BD-047).
4. **§13.11 has no numbers.** Its known answers and counterexample are not in the appendices, and its refusal has no code. Done: late slots in N13-KA and N13-CX, a candidate code in N13-REF, all flagged for the registry owners.
5. **GATE-MARROW-06 is dated M1** and needs device tiers and teleport, which exist at M2. Done: referee clauses at M1 (N13-36), device and teleport clauses and the full record at M2 (N13-37).
6. **GATE-MARROW-01 says "all laws"** at M0 and M1, while the mechanisms of three laws arrive later (blocks at M4, predicates on devices at M2, the four-dimensional cone at M2). Done: N13-19 records the laws whose mechanisms exist and lists the deferred rows with their gates. The seed done-condition of BD-045 ("GATE-MARROW-01 complete") is split between BD-045 (Lie-group rows) and N13-19 (the record).
7. **A stated control that the stated law cannot catch.** KA-249 names "a conjugate missing in the projector" as its negative control, but that mutant is the projector of the conjugate character: still idempotent, still summing to the identity. Done: BD-048 adds a restriction-character clause and an eigenvector fixture that do catch it, and N13-18 documents the blind fixtures.
8. **A second such case.** KA-247 names "the bracket's arguments exchanged" as the control of the Subgroups law, but bracket closure is sign-independent. Done: BD-046 adds a second-order composition fixture whose sign depends on the bracket.
9. **Two of the eight mutations coincide mathematically** ("Ad* and its inverse exchanged" and "a pull-back in place of a push-forward": the inverse of the push-forward is the pull-back). Done: N13-18 distinguishes them as a definition-level defect and a use-site defect.
10. **Colour key of §13.4, step 1.** The characteristic polynomial of the origin-based pseudo-inertia is not invariant under a translation of the body frame. Done: N13-15 uses the central form (mass and second moment about the centre of mass) and says so.
11. **Single-precision rows of GATE-MARROW-01** against ADR-MARROW-04 (no typed single-precision group). Done: the laws' `f32` rows are measured on a generic reference implementation instantiated at `f32` inside tests and on generated kernels (BD-010, N13-08, N13-12).
12. **Where exact arithmetic lives.** §13.1 says both crates depend only on `frob-core` and the airlock; §13.7 makes `frob-lie` a co-owner of the predicate classes; no crate is named for producer-side exact integers and rationals (the auditor's are deliberately separate). Done: `frob-lie::exact` (N13-10), with `frob-algebra` depending on `frob-lie` as an earlier row of the same layer; the alternative (`frob-core`) is stated in both beads.
13. **Decisions "at a gate" that also "block" it** (OPEN-MARROW-1, OPEN-MARROW-3). Done: each decision bead precedes the gate's measurement with a fail-closed default and is annotated afterwards, so there is no dependency cycle (N13-09, N13-31).
14. **Wreath-product ceiling.** The stated ceiling is a factor equal to the limb group's order (1 or 2). A hand count for the swerve base gives a reduction of 3 in free parameters on odd channels (3 to 1, and 12 to 4 on mixed channels). Done: N13-25 registers the counts as predictions and requires the pilot to report them against the ceiling and revise the line if they stand.
15. **A registered pilot that cannot fail as worded.** The geodesic between two positive-definite matrices stays positive definite, and so does the straight segment (the cone is convex). Done: N13-29 runs the pilot as registered and adds an extrapolation contrast in which the Euclidean alternative fails.
16. **Numbers the text leaves open**, all labelled DESIGN in the beads and to be fixed at their gates: chart-cut margins and small-angle thresholds (BD-011); the host rational size cap (N13-10); magnitude floors of guarded-predicate inputs and the treatment of out-of-range inputs (N13-10, N13-11); the distribution of the 24 planted asymmetries over archetypes (N13-21, N13-22); the reading of the GATE-MARROW-02 set as 49 presentations (BD-048); the tolerance of "Ad(exp x) = exp(ad x) to tolerance" (BD-011, N13-19).
17. **Algorithms the text names without giving.** Character tables (BD-048 specifies the class-matrix method with exact lifting and self-certification); symmetry-adapted bases with aligned copies (BD-325 specifies a splitting-subgroup construction and the complex structure J = i·(P_χ − P_χ̄)); SE(3) exponential differential in closed form (BD-011 gives a candidate and makes the dual-number law its arbiter). These are DESIGN in the beads.
18. **Content of my subsections whose seed beads belong to other units** (BD-049, BD-050, BD-051, BD-052, BD-053, BD-062, BD-078, BD-098, BD-102, BD-154). To make sure nothing in §11 and §13 depends on those units reading these chapters, the full text of each such subsection is carried in a U13 bead as a specification with its conformance tests (N13-04, N13-06, N13-10, N13-15, N13-16, N13-17, N13-19). The lead may wish to de-duplicate once the other units' files exist.

## (f) Integration addendum

Changes made when this unit was connected to the others:

- BD-011 no longer depends on BD-021: a seed never depends on a higher-numbered seed. The certified clause of GATE-MARROW-01 is verified in N13-19.
- N13-34 and the Skeleton beads N14a-08 and N14a-09 state the same stage-1 check. One direction is kept: N13-34 follows stage 1 (N14a-08) and the stage runner (BD-110), and the Skeleton beads keep N13-34 as related. N13-36 depends on BD-110, since its script drives `design.check`.
- N13-06 depends on the window-boundary decision N20a-49 written by the sensing group: §11.8 says (a, b] and §20.4.3 says [open, close) for the shared instant of two neighbouring exposure windows. The boundary-instant clause of the window row is frozen only after that ruling.
- The candidate code of the uncoded turn-state refusal is FROB-REF-128, not FROB-REF-127 as in (c) and (e) item 4: the Skeleton registry already proposes 127 for its own uncoded refusal. Assignment still belongs to the registry's owner, for whom no bead exists (reported as a gap).
- The row of FROB-REF-168 is entered by the Lemma registry N12-REF with the wording of N13-03's default; N13-03 amends or tombstones it.
