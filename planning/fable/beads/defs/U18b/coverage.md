# Coverage of drafting unit U18b (§18.9 to §18.20, Traction)

51 bead files: 5 seed beads (BD-156, BD-167, BD-168, BD-365, BD-366), 44 new beads (N18b-01 to N18b-44) and
2 registry beads (N18-CX, N18-REF). The known-answer registry N18-KA is written by unit U18a.

## (a) Subsections and the beads that carry them

| Subsection | Title | Beads |
|---|---|---|
| §18.9 | Wheels, tracks and hybrids | N18b-01 (wheel as body, joint and collider; rolling oracle; the planar-drive admission rule), N18b-02 (mecanum and omni wheels), N18b-03 (pilot: tyre laws), N18b-04 (brush and Fiala tyre laws), N18b-05 (pilot: belt tracks), N18b-06 (belt tracks; skid-steer cannot be a rolling constraint), N18b-07 (pilot: link-chain tracks), N18b-08 (epic: link-chain tracks), BD-365 (all tiers; hybrids as compositions; NS-08 on rigid ground), N18b-09 (e2e, NS-08) |
| §18.10 | Terrain mechanics | N18b-10 (rigid floors with per-texel cards; compliant coverings), N18b-11 (pilot: soft soil), N18b-12 (soil laws and cards; FROB-REF-243), N18b-13 (soil rows in the step; simulated sinkage and drawbar pull), N18b-14 (pilot: granular media), N18b-15 (granular media by resistive force theory), N18b-16 (pilot: persistent ruts), N18b-17 (rut and footprint fields), BD-366 (verb `contact.soil`; gate record), N18b-19 (e2e) |
| §18.11 | Contact capacity and the rule against capacity bias | N18b-20 (static maxima; three admitted responses; verb `contact.capacity`), N18b-21 (patch reduction with qualification fixture), N18b-22 (capacity-bias rule; FROB-REF-822), BD-168 (forced overflow; fixture on batched tiers), N18b-24 (e2e) |
| §18.12 | Precision: diagonal scaling, and why half precision is rejected | BD-156 (solve kernels in scaled variables; GATE-TRAC-10), N18b-25 (K-scale-diag; PO-TRAC-03), N18b-26 (stiffness admission; FROB-REF-244; OPEN-TRAC-4), N18b-27 (half precision refused; FROB-REF-245; the refuting calculation) |
| §18.13 | Verbs | N18b-28 (`contact.laws`, M1), N18b-29 (`contact.probe`, M1), BD-167 (both verbs with cards, M2), N18b-30 (e2e for both), N18b-20 (`contact.capacity`), N18b-23 (`contact.audit`), N18b-24 (e2e for both), BD-366 (`contact.soil`), N18b-19 (e2e), N18b-42 (e2e for `contact.response`). `contact.clearance` is seed BD-233 (unit U19b) and `contact.response` itself is seed BD-367 (unit U18a) |
| §18.14 | Refusals | N18-REF (all ten codes, three uncoded refusals, cited foreign codes); raised by N18b-26 (244), N18b-27 (245), N18b-28 (246), N18b-12, N18b-13, N18b-04, BD-366 (243), N18b-22 (822), BD-167, N18b-02, N18b-29 (240), N18b-29 (242) |
| §18.15 | Invariants and proof obligations | N18b-31 (all seven invariants and four obligations, with derivations); carried in mechanisms by N18b-28 (INV-TRAC-01), BD-167 (INV-TRAC-02), N18b-20, N18b-22, BD-168 (INV-TRAC-04), BD-156 (INV-TRAC-05 through scaled tolerances), N18b-38 (INV-TRAC-06), N18b-25 (PO-TRAC-03) |
| §18.16 | Known-answer battery | N18b-32 (the 29 rows as fixtures with tolerances and the row-by-tier table), BD-168 (batched tiers), BD-167, N18b-01, N18b-02 (rolling, incline, spin and mecanum rows), N18b-12, N18b-13, N18b-17 (soil and rut rows), N18b-04, N18b-06 (tyre and skid-steer rows) |
| §18.17 | Gates and kill gates | Gate table: see (b). Kill decisions: N18b-33 (KILL-CONTACT), N18b-18 (KILL-SOIL). Pilot table: N18b-34 (CL-2), N18b-03 (tyre laws), N18b-05 (belt tracks), N18b-14 (granular media), N18b-16 (persistent ruts), N18b-07 (link-chain tracks), N18b-35 (tactile micro-patches) |
| §18.18 | Open questions | N18b-37 (OPEN-TRAC-1), BD-167 (OPEN-TRAC-2, decided with GATE-TRAC-04), N18b-36 (OPEN-TRAC-3, owner decision), N18b-26 (OPEN-TRAC-4, decided by GATE-TRAC-10), N18b-38 (OPEN-TRAC-5) |
| §18.19 | Response batches: anchor, acceptance, known answers, gate | N18b-39 (anchor; stale-anchor refusal), N18b-40 (per-column acceptance; ρ; guards; worked values), N18b-41 (λ_lo by `rational-ldlt`), N18b-42 (GATE-TRAC-11; second trigger of KILL-RESPONSE; e2e) |
| §18.20 | Swept bounds enclose arcs | N18b-43 (construction; missing-speed refusal; referee rows), N18b-44 (`strict` rows; audit attribution), N18b-23 (the audit's attribution by cause) |

## (b) Gates, kill gates and the beads that measure or decide them

| Gate | Carried in this unit by | Expected from another unit |
|---|---|---|
| GATE-TRAC-01 | BD-168 (batched rows), N18b-32 (fixtures and row-by-tier table), BD-167 (incline, rolling, mecanum rows) | BD-085 (referee rows, U18a), BD-161 (`strict` rows, U18a) |
| GATE-TRAC-02 | none | BD-079 (U33b), BD-158 (U18a) |
| GATE-TRAC-03 | BD-168 (record), N18b-23 (the audit), N18b-24 (e2e) | none |
| GATE-TRAC-04 | BD-167 (spin stop, ring co-stop; OPEN-TRAC-2), N18b-30 (e2e) | BD-154 (cone rows, U18a), BD-155 (joint rows, U15b) |
| GATE-TRAC-05 | none (cited by N18b-29 and N18b-31 as known answers) | BD-230 (U18a) |
| GATE-TRAC-06 | BD-168 (batched rows, capacity-bias fixture), N18b-22, N18b-24, N18b-38 (sill tolerances) | BD-086 (referee rows, U18a), BD-161 (`strict` rows, U18a) |
| GATE-TRAC-07 | none (INV-TRAC-03 test in N18b-31) | BD-232 (U18a), BD-233 (U19b) |
| GATE-TRAC-08 | BD-366 (record), BD-365, N18b-04, N18b-06, N18b-12, N18b-13, N18b-17, N18b-18, N18b-09, N18b-19 | none |
| GATE-TRAC-09 | none | BD-386, BD-424 (U18a) |
| GATE-TRAC-10 | BD-156 (record), N18b-25, N18b-26, N18b-30 | GATE-SPARK-18 on the same kernels (U35) |
| GATE-TRAC-11 | N18b-42 (measurement), N18b-39, N18b-40, N18b-41 | BD-367 supplies the mechanism (U18a) |
| GATE-TRAC-12 | N18b-43 (referee, M1), N18b-44 (`strict` and audit clause, M2) | none |
| KILL-CONTACT | N18b-33 (decision), N18b-34 (CL-2 pilot it reads) | BD-212 measures GATE-FID-01 (U16b) |
| KILL-FILM | none | BD-230 and its decision bead (U18a) |
| KILL-HARDCONTACT | none | BD-386 and its decision bead (U18a) |
| KILL-MESHGPU | none | BD-232 and its decision bead (U18a) |
| KILL-RESPONSE | N18b-42 (second trigger), N18b-40 | BD-367 (first trigger and the pilot, U18a) |
| KILL-SOIL | N18b-18 (decision), N18b-36 (sourcing input), BD-366, N18b-19 | none |

No admission gate (INT-…) is defined in these subsections. INT-FSIM-05 (the planar-drive near-straight case,
BD-033) is cited by N18b-01 as the switch-over condition for the rolling oracle.

## (c) Counts

- **Known answers of §18 (Appendix M.17, KA-387 to KA-425, 39 rows).** 35 carried with their values in this
  unit's beads: the 29 battery rows in N18b-32 (KA-387 to KA-389, KA-391 to KA-397, KA-399 to KA-406, KA-408 to
  KA-410, KA-413 to KA-420), plus KA-390 (N18b-01), KA-398 (BD-167, N18b-31), KA-407 (N18b-29, N18b-31), KA-422
  (BD-156, N18b-25), KA-423 (N18b-27), KA-424 (N18b-31). Left to U18a: KA-411, KA-412, KA-421, KA-425.
- **Known answers of other chapters cited with values:** KA-027, KA-177, KA-268, KA-282, KA-349, KA-436,
  KA-509, KA-797, and the two late fixture rows (plate sinkage with n = 0.5; residual bound in single precision).
- **Known answers derived in these beads (hand-checkable, each with its derivation):** rim polygon deficit
  (π/m)²/6 (N18b-01); rolling-resistance coast-down c_rr·g/(1 + I/(m·R²)) (BD-167); belt scrub moment
  μ_lat·W·L/4 (N18b-05, N18b-06); fallback tow force (c_rr + c_d)·N (N18b-18); covering series stiffness
  (N18b-10); the 2 × 2 LDLᵀ certificate at s = 63/64 (N18b-41).
- **Counterexamples of §18 (Appendix N.15):** 17 of 17 in N18-CX (CX-167 to CX-183), plus four provisional rows
  from §18.19 and §18.20 that have no catalogue number, plus CX-014, CX-088, CX-131, CX-148, CX-151 cited.
- **Refusal codes of §18:** 10 of 10 in N18-REF (240 to 246, 320, 321, 822), plus three refusals named without
  a code, plus foreign codes 121, 222, 224, 248, 721, 722 cited with meanings.
- **Other identifiers.** Invariants INV-TRAC-01 to -07 and obligations PO-TRAC-01 to -04: N18b-31. Open
  questions OPEN-TRAC-1 to -5: see (a). Decisions cited with reasons: ADR-TRAC-01 (N18b-37), -03 (BD-167,
  N18b-02), -04 (N18b-28, N18b-33), -06 (BD-167, N18b-04, N18b-13, N18b-15), -07 (BD-156, N18b-25, N18b-27),
  -08 (N18b-20, N18b-21, N18b-22, BD-168); ADR-TRAC-02 and -05 belong to U18a's subsections. Bets BET-03,
  BET-08; risks RISK-05, RISK-24, RISK-31, RISK-40; constructions C-06, C-07, C-12, C-14, C-36, C-37; theorem
  cards T-07, T-08; scenario NS-08; upstream items FD-FSIM-04, -11, -12, -13.

## (d) Needs from other slices

Prerequisites owned by other units (also recorded per bead in `needs`):

- §18.4, §18.5, §18.6, §18.8 (U18a): the detection kernels, speculative margin and gap error bound; the
  `ContactLaw` trait and the laws CL-1, CL-2, CL-M, CL-H; the anisotropic cone and masked spin row (BD-154); the
  film law (BD-230); witness migration and the sill fixture (BD-086, BD-093); the production CL-2 law bead,
  which must depend on the pilot N18b-34; the response-batch mechanism and its pilot (BD-367).
- §15.3 (U15a): the `.fcard` container with per-field provenance; joint speed limits and actuator limits.
- §16.5, §16.6 (U16a): contact memory and terrain edits in the restore set; rows with a column on a joint
  coordinate; run-time capacity flags; codes FROB-REF-721 and -722.
- §16.8, §16.17 (U16b): GATE-FID-01 and the frozen standard battery (BD-212).
- §19.3 (U19a, U19b, U19c): the interface map (BD-229); per-world mutable pools and INV-HAB-03; compiled
  interior packs at 1 cm.
- §20 (U20a/b): a tactile or force sensor that reads per-contact impulses (for the pilot N18b-35).
- §12.4 (U12, U25b): the `rational-ldlt` and `poly-identity` proof kinds and the offline auditor (BD-027).
- §28.6 (U28a): the M3 comparison verbs (BD-262) must call the capacity-bias rule of N18b-22.
- §32 (U32a): the verb registrations of M1, M2 and M4 (BD-137, BD-222, BD-376) list the six verbs here.
- §35.10 (U35): GATE-SPARK-18, FROB-REF-248 and the run-time κ̂·u limit.
- §38 (U38a/b): the pilot register, the negative-control registry, the known-answer registry loader.

Dependencies that beads of other units should take on beads of this unit (for the lead to wire):

- BD-367 (U18a) → N18b-39, N18b-40, N18b-41. BD-417 and BD-418 (U38a) → N18b-16/N18b-17 and N18b-35.
- BD-117 and BD-109 (U14b; mecanum and differential archetypes on the referee at M1) → N18b-01, N18b-02.
- BD-085, BD-161 (U18a) → N18b-32 (the battery fixtures). BD-081/BD-082 (U16a) → N18b-43 (swept bounds).
- BD-212 (U16b) names "KILL-CONTACT decided"; the decision bead N18b-33 depends on BD-212's measurement.
- BD-245 (U14a) and N18b-05/N18b-06: see (e) item 9.

## (e) Contradictions, gaps and what was done

1. **Verb milestones.** §18.13 gives `contact.laws` and `contact.probe` first milestone M1; the seed that names
   them (BD-167) is M2. Wrote M1 verb beads (N18b-28, N18b-29); BD-167 keeps M2 and adds cards.
2. **"Batched rows" of GATE-TRAC-06.** The gate's zero-miss and 1% clauses are stated for transactional
   profiles; the seed BD-168 asks for the gate "on batched tiers". Read as: the capacity-bias fixture on
   lockstep, with thin-barrier and sill reported there (DESIGN reading, stated in BD-168).
3. **GATE-TRAC-03 and the battery freeze.** The gate names "the M2 battery"; the bead that freezes the standard
   battery (BD-212) depends on BD-168. Recorded against the pre-freeze identity and re-read on freeze. The text
   of §38.8 names BD-104 as the freezing bead; the seed table makes it BD-212.
4. **Refusals without codes.** "Stale anchor" (§18.19), "swept bound without a speed bound" (§18.20) and the
   seed's "missing material pair" have no code in Appendix O. Listed in N18-REF; each raising bead appends a
   code in the `2xx` family.
5. **GATE-TRAC-11 is absent from the §40 tables** (present only in the late-gates table). §18.19 governs; M4.
6. **Kill gates "decided" inside seed done-conditions** against the rule of one decision bead per kill gate.
   KILL-SOIL: decision bead N18b-18 precedes BD-366, which closes on capability or fallback. KILL-CONTACT:
   N18b-33 depends on BD-212's measurement. KILL-RESPONSE: first trigger with BD-367, second in N18b-42.
7. **Persistent ruts** are Moonshot with M6 in the Moonshot queue, yet GATE-TRAC-08 (M4) has a rut-leak row and
   seed BD-366 names ruts. Pilot N18b-16 and mechanism N18b-17 are at M4 with priority 4 by the Moonshot rule;
   BD-366 depends on N18b-17; if unfunded, the row is recorded against the fallback "off". The lead may wish to
   raise their priority.
8. **Soft soil has a kill gate and no named pilot.** N18b-11 is a DESIGN reading of the pilot rule (host closed
   forms plus a one-plate sinkage check). Appendix H holds no soil table, so KILL-SOIL's sourcing trigger is
   live (N18b-36).
9. **Belt law at M3 and at M4.** BD-245 (M3, U14a) delivers the `belt` law "on all tiers" with the scrub-work
   answer; §18.9 makes belt tracks Frontier with a pilot, and BD-365 (M4) is "tracks on all tiers". Read as:
   BD-245 owns the law-name bindings; N18b-05/N18b-06 own the funded belt model. §14.13 cites a "belt-law known
   answer" that §18.16 does not list; derived it (scrub moment μ_lat·W·L/4) in N18b-05 and N18b-06.
10. **Underspecified laws.** Only the lateral brush law is printed; combined slip, the Fiala longitudinal form,
    and all resistive-force formulas are recalled from the literature, labelled DESIGN and "to be confirmed at a
    primary source" (N18b-04, N18b-14, N18b-15). How a slip-to-force or shear law enters the convex solve is not
    stated; used the lagged-coefficient rule of ADR-TRAC-06 (N18b-04, N18b-13, N18b-15). "Prescribed belt
    velocity" is tied to the sprocket coordinate (N18b-06). The "vehicle corpus" is undefined; defined in
    N18b-03. `contact.soil` for a wheel has no closed form; it uses station quadrature (BD-366).
11. **Tolerances printed "to set"** (KA-388, KA-395, KA-413, KA-414): set as DESIGN in N18b-32.
12. **Rim samples and the 0.1% rolling answer.** Eight samples per wheel (the capacity table) cannot meet
    v = ω·r within 0.1% (polygon deficit 2.6%); specified a 64-sample rim with a tested window of 8 (N18b-01).
13. **Response-batch bound units.** A 2-norm over mixed metre and radian unknowns; read as stated in the scaled
    variables of stage 8, and a guard for slipping rows was added to match "stick, slip or open boundary"
    (N18b-40).
14. **KA-394 tier.** Printed for `strict` and lockstep, while the mecanum archetype must traverse on the referee
    at M1; N18b-02 is M1 with a referee check and the batched rows stay with BD-167 and BD-168.
15. **NS-08 and bead order.** BD-365 must demonstrate NS-08, whose soft-soil leg needs BD-366, which depends on
    BD-365. Rigid-ground legs in BD-365; soil leg in the e2e bead N18b-09.
16. **Two conditioning thresholds.** Admission at scaled κ·u > 0.05 (FROB-REF-244, §18.12) and run-time
    κ̂·u > 0.1 (FROB-REF-248, §35.10) are different checks; both kept. `K-scale-diag` and `K-scale` are one
    kernel, and PO-TRAC-03 and PO-NUM-02 one fixture identity.
