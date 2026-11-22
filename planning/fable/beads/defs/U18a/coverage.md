# Coverage of drafting unit U18a

Unit: chapter 18 (Traction), preamble and §18.1 to §18.8, plus 14 seed beads. 56 bead files: 14 seed beads,
1 registry bead (`N18-KA`) and 41 new beads (`N18a-01` to `N18a-41`).

## (a) Subsections and the beads that carry them

| Subsection | Title | Beads |
|---|---|---|
| §18 preamble | Crate `frob-contact`, layer 3; what it reads, who calls it, what it reaches through the airlock | N18a-01 |
| §18.1 | What Traction owns | N18a-01 (ownership and boundary); BD-165 (seed matched here: terrain generators, a Habitat mechanism) |
| §18.2 | Decisions (ADR-TRAC-01 to -08) | N18a-01 (all eight, with reasons and rejected alternatives); each also in the bead it governs: ADR-TRAC-01 BD-158, BD-232; -02 BD-232, N18a-03; -03 BD-154; -04 N18a-15, N18a-16, N18a-21, BD-386; -05 N18a-32; -06 N18a-29, BD-230; -07 named only (mechanism in the other unit's §18.12); -08 BD-080 (mechanism in the other unit's §18.11) |
| §18.3 | Geometry: fields, primitives, and a mesh path with enclosures | N18a-02 (gap error bound ε_g and the quantization arithmetic); BD-158 (primitive kernels); BD-232 (mesh hierarchy, enclosure sets, thin-feature detection and routing, FROB-REF-320); N18a-03 (three-answer enclosure predicate, certificates, FROB-REF-321); N18a-04 (pilot), N18a-05 (KILL-MESHGPU), N18a-06 (GPU hierarchy epic); N18a-07, N18a-08 (convex hulls, Frontier); N18a-09 (dense bricks against a sparse brick tree, OPEN-TRAC-1); BD-037 (FD-FSIM-03 and the other upstream items); BD-422 (seed matched here) |
| §18.4 | Detection, swept bounds and witnesses | N18a-10 (sample spheres, moving against static); N18a-11 (moving against moving, self-collision filter); BD-158 (batched broad and narrow phase); N18a-12 (speculative margin); N18a-13 (narrow-phase record, swept interface, path admission); N18a-14 (witness pairs and migration, INV-TRAC-06, OPEN-TRAC-5); BD-086 (thin-barrier and sill fixtures); BD-080 (referee step on generated contact sets) |
| §18.5 | Contact laws | N18a-15 (trait, registry, parameter classes, admission, FROB-REF-246, barrier contact not built); N18a-16 (CL-1); N18a-17, N18a-18 (CL-2 pilot and law); N18a-19 (KILL-CONTACT); N18a-20 (CL-M); N18a-21 (CL-H specification, residual, conformance); N18a-22 (pilot), BD-386 (CL-H batch law), N18a-23 (KILL-HARDCONTACT), N18a-24 and BD-424 (GPU tiers); N18a-25 (pilot), BD-367 (response batches, with the anchor, acceptance, known answers and gate of §18.19), N18a-26 (KILL-RESPONSE); BD-085 (battery on the referee); BD-161 (`strict` profile) |
| §18.6 | Friction, and the cone in four dimensions | BD-154 (cone, anisotropic image, spin mask, PO-TRAC-01, FROB-REF-240, cone kernels); N18a-27 (r_eff, PO-TRAC-04, OPEN-TRAC-2); N18a-28 (limit-surface error, patch rules, centre of pressure, load-weighted slip); N18a-29 (lagged state-dependent coefficients); N18a-30, N18a-31 (Stribeck, Frontier); N18a-16 (stiction speed and creep in the fidelity ledger). Rolling resistance as an extra row: seed BD-167 (other unit) |
| §18.7 | Joint-level rows: stiction, backlash and hard stops | N18a-32 (row laws, FROB-REF-241, fixtures, ADR-TRAC-05); integration in seeds BD-084 and BD-155 (other units) |
| §18.8 | Film friction for wet and greasy floors | BD-230 (law, state, `K-film`, INV-TRAC-07, PO-TRAC-02, FROB-REF-242); N18a-33 (wet-floor battery); N18a-34 (KILL-FILM); N18a-29 (lag mechanism) |

End-to-end beads, one per user-visible capability: N18a-35 (referee and `strict` contact stepping), N18a-36 (batched
cone and joint rows, CL-1, collision kernels), N18a-37 (film law), N18a-38 (thin features and enclosures), N18a-39
(`contact.response`), N18a-40 (CL-H batch law), N18a-41 (`scene.gen` terrain).

Seed beads written (14): BD-037, BD-080, BD-085, BD-086, BD-154, BD-158, BD-161, BD-165, BD-230, BD-232, BD-367,
BD-386, BD-422, BD-424.

## (b) Gates and kill gates of chapter 18

| Gate | Carried in this unit by | Expected from another unit |
|---|---|---|
| GATE-TRAC-01 | BD-085 (referee rows, with the expressibility list), BD-161 (`strict` rows), N18a-16 (law rows of CL-1), N18a-35 (script) | BD-168 (batched rows; the other Traction unit) |
| GATE-TRAC-02 | BD-158 (batched kernels against the CPU primitives), N18a-08 (extension to hulls), N18a-36 (script) | BD-079 (CPU primitives against `fs-query`; U33b) |
| GATE-TRAC-03 | N18a-12 (the speculative-margin mechanism and its row) | BD-168 (the tunnelling audit that measures it) |
| GATE-TRAC-04 | BD-154 (cone rows), N18a-27 (r_eff rows), N18a-32 (joint-row laws and referee fixtures), N18a-36 (script) | BD-155 (joint rows on batched tiers; U15b) |
| GATE-TRAC-05 | BD-230, N18a-37 | — |
| GATE-TRAC-06 | BD-086 (referee rows), BD-161 (`strict` rows), N18a-35 (script) | BD-168 (batched rows and the capacity-bias label) |
| GATE-TRAC-07 | N18a-03 (pairwise predicate and certificates), N18a-38 (script) | BD-233 (verb and clearance complexes; U19b) |
| GATE-TRAC-08 | — | The other Traction unit (soil, tracks, tyres: §18.9, §18.10) |
| GATE-TRAC-09 | BD-386 (`batch-cpu` rows), BD-424 (device rows), N18a-40 (script) | — |
| GATE-TRAC-10 | — | BD-156 and the other Traction unit (§18.12) |
| GATE-TRAC-11 | BD-367, N18a-39 | — |
| GATE-TRAC-12 | — (cited as a `needs` by BD-086, BD-161, N18a-03, N18a-13) | The other Traction unit (§18.20) |
| KILL-CONTACT | N18a-19 (decision); N18a-17 (CL-2 pilot that its fallback needs) | The other Traction unit owns §18.17, where the row is printed; if it also writes a decision bead, merge with N18a-19 |
| KILL-FILM | N18a-34 (decision), N18a-33 (measurement) | — |
| KILL-HARDCONTACT | N18a-23 (decision), N18a-22 (pilot), BD-386 (gate measurement) | — |
| KILL-MESHGPU | N18a-05 (decision), N18a-04 (pilot) | — |
| KILL-RESPONSE | N18a-26 (decision), N18a-25 (pilot), BD-367 (gate measurement) | — |
| KILL-SOIL | — | The other Traction unit (§18.10, §18.17) |
| GATE-HAB-06 (chapter 19, named by seed BD-230) | BD-230, N18a-37 | — |
| GATE-KIN-04 (chapter 16, `strict` rows named by seed BD-161) | BD-161 | BD-083 and the Kinesis units (referee rows) |

## (c) Counts

- Known answers: 39 rows in `N18-KA` (KA-387 to KA-425, all of Appendix M.17). Of these, 30 are cited with their
  values in test plans of this unit's mechanism beads; the other 9 (KA-390, KA-415 to KA-420, KA-422, KA-423: the
  rolling-oracle admission row and the rows of soil, tyres, tracks, capacity bias and precision) are registered
  here and carried by other units' beads.
- Counterexamples: the registry `N18-CX` is the other unit's. This unit's beads cite and plant 12 of the 17 rows of
  N.15 as negative controls: CX-167, CX-168, CX-169, CX-170, CX-171, CX-172, CX-177, CX-178, CX-179, CX-181,
  CX-182, CX-183; plus CX-148 (the pyramid cone) from another block. The other 5 (CX-173 to CX-176, CX-180)
  belong to §18.9 to §18.12.
- Refusal codes: the registry `N18-REF` is the other unit's. This unit's beads specify trigger, repairs and exit
  code for 6 of the chapter's 10 codes: FROB-REF-240 (BD-154), -241 (N18a-32), -242 (BD-230), -246 (N18a-15,
  BD-386), -320 (BD-232), -321 (N18a-03). They also use -220, -221, -222, -223, -224, -721 (BD-161, BD-080,
  BD-085, N18a-21). Codes -243, -244, -245 and -822 belong to §18.9 to §18.12.
- Invariants and proof obligations: INV-TRAC-01 (N18a-15, BD-386), -02 (N18a-29), -03 (N18a-03), -05 (N18a-15),
  -06 (N18a-14, BD-086), -07 (BD-230); PO-TRAC-01 (BD-154), -02 (BD-230), -04 (N18a-27). INV-TRAC-04 and
  PO-TRAC-03 belong to §18.11 and §18.12.
- Open questions: OPEN-TRAC-1 (N18a-09), -2 (N18a-27, BD-154), -3 (BD-230, N18a-31, N18a-33: considerations),
  -5 (N18a-14, BD-086). OPEN-TRAC-4 belongs to §18.12.
- Pilots (8): N18a-04, N18a-07, N18a-09, N18a-17, N18a-22, N18a-24, N18a-25, N18a-30. Kill decisions (5): N18a-05,
  N18a-19, N18a-23, N18a-26, N18a-34.

## (d) Needs from other slices

- §18.9 to §18.20 (the other Traction unit): the swept-bound construction that encloses arcs and GATE-TRAC-12
  (needed by BD-086, BD-161, N18a-03, N18a-13); the patch reduction with its qualification fixture (N18a-28); the
  refusal registry `N18-REF`, including a **new code for the stale-anchor refusal of the response batch** (BD-367:
  §18.19 names the refusal and Appendix O assigns no number); the read verbs `contact.laws` and `contact.probe`
  at M1 (N18a-15, N18a-35); standing decision records for OPEN-TRAC-1, -2, -3 if that unit writes them (§18.18).
- Kinesis units (§16): the measurement of GATE-FID-01 on the M2 battery (N18a-19); the step-adjoint mechanism,
  its pilot, GATE-KIN-10 and KILL-STEPADJOINT (BD-422).
- Sinew unit (§15.6): the row descriptors emitted from the actuator card (N18a-32).
- Habitat units (§19): the thin-feature corpus from the hazard lists (N18a-04).
- Arena unit (§24): the low-friction crossing task family and outcome classes (N18a-33).
- Dependency edges that other units' seed beads should add: BD-153 should depend on N18a-16 (the CL-1 law it
  evaluates); BD-093 on N18a-14 (the migration algorithm it wires in); BD-166 on N18a-10 and N18a-12 (sample
  criterion and margin); BD-233 on N18a-03 (the pairwise predicate); BD-084 and BD-155 on N18a-32 (row laws);
  BD-168 on N18a-12; BD-266 may use N18a-18 as its CL-2 arm.

## (e) Contradictions, gaps and what was done

1. **CL-1 has no closed form in the plan.** §18.5 states its contents ("linear compliant normal with a
   dissipation time; regularized Coulomb friction with a stated stiction speed") and its properties, not its
   potential. N18a-16 states the properties as binding and gives one construction consistent with all of them
   (a cone-projected compliant potential), labelled DESIGN, with the final form deferred to SPEC-C-06.
2. **CL-2's convex form is not specified.** N18a-17 and N18a-18 give a DESIGN form (monotone Hunt–Crossley normal
   impulse; friction bounded by the committed normal impulse) to be confirmed by the pilot.
3. **Film law, two statements.** §18.8 gives μ_eff = ψ·μ_b + (1 − ψ)·η·v·π·a²/(h·N) with an unspecified monotone
   blend and "pure Couette shear for h ≥ 3σ" (INV-TRAC-07); §19.10.2 gives the blend clamp((3 − Λ)/2, 0, 1) and
   caps the viscous term at μ_b. BD-230 uses §19's blend and cap, flags a contact where the cap binds
   (`viscous-capped`), and tests the pure-Couette limit where it does not bind.
4. **Squeeze-time tolerance.** Appendix M row KA-438 says "within 1% (GATE-HAB-06)"; §40 and §19.19 say 2%.
   BD-230 uses 2% (§40 governs).
5. **`contact.laws` and `contact.probe` milestone.** §18.13 and Appendix J give first milestone M1; the only seed
   naming them (BD-167) is at M2. Recorded as a `needs` on N18a-15 and N18a-35.
6. **Pilots not named by the plan.** Convex hulls, Stribeck friction and the GPU port of CL-H are Frontier with no
   named pilot. N18a-07, N18a-30 and N18a-24 register pilots with proposed falsifiers, labelled DESIGN.
7. **KILL-CONTACT ownership.** §40 says "defined in §18.17" (other unit), but the mechanism it decides (the
   default law, with CL-2 as fallback) is §18.5. N18a-19 carries it; flagged for merging if duplicated.
8. **GATE-TRAC-01 tolerances "to set".** KA-388 and KA-395 have no tolerance in the register; BD-085 sets 0.5%
   and an exact verdict at μ ± 5% as TARGETs. KA-413 is set to 0.5% in N18a-28.
9. **Referee-expressible rows.** The plan requires the list to be recorded with the gate and does not give it.
   BD-085 derives an initial list from the tier column of Appendix M and makes the measured list an output.
10. **Two refusals at sub-voxel geometry.** FROB-REF-320 (Traction: no thin route) and FROB-REF-306 (Habitat: no
    overlay or an over-full overlay index cell) can both apply to one feature; BD-232 requires the compile stage
    to raise exactly one.
11. **Seed beads outside the subsections.** BD-037 (upstream register, Appendix F) and BD-165 (terrain
    generators, §19.6) were matched to this unit and are written in full from their own sections; BD-422 draws
    on §16.21 and is written as an epic with a `needs` for the mechanism bead.
12. **`registries/upstream.toml` has no named reader.** The hygiene rule deletes registries that no verb or gate
    reads. BD-037 proposes `frob doctor` and the loader test as readers and flags the point for the lead.
13. **Upstream register counts.** Appendix F states 53 items (33/3/5/2 plus ten singletons; 2/8/16/27 by
    priority); the table in BD-037 was recounted and matches.
