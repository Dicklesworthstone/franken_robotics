# Coverage of drafting unit U19a

Chapter §19 Habitat: scenes, living worlds, clearance, retrofits. This unit owns the chapter preamble and §19.1 to
§19.8, five seed beads (BD-104, BD-228, BD-229, BD-234, BD-399) and the known-answer registry N19-KA. It wrote 51
bead files: 5 seed beads, 45 new beads (N19a-01 to N19a-45) and 1 registry bead (N19-KA).

Several seed beads whose core mechanism lies in these subsections are written by other units (BD-162 by U19b;
BD-163 and BD-164 by U19c; BD-165 by U18a; BD-225 and BD-226 by U14b; BD-227 by U14a; BD-230 by U18a; BD-231 by
U41). The new beads of this unit are written as self-contained subtasks that depend on those seeds, so that every
statement of §19.1 to §19.8 is carried in full here whatever the other units write.

## (a) Subsections and the beads that carry them

| Subsection | Title | Beads |
|---|---|---|
| §19 preamble | Owner crate, formats, label conventions | N19a-01 |
| §19.1 | What it owns (ownership boundary, object table, verb table, long-running verbs) | N19a-01 (boundary, objects), N19a-02 (verbs, bounded wait) |
| §19.2 | Decisions (ADR-HAB-01 to -12) | N19a-03 (all twelve, with enforcing tests); restated in N19a-06, N19a-09, N19a-14, N19a-16, N19a-17, N19a-20, N19a-22, N19a-23, BD-229, BD-234, N19a-39 |
| §19.3 | The scene pack (content list, manifest example) | N19a-04 (inventory, semantic labels, spawn, goals, task regions, route graphs, declarations), N19a-05 (provenance block); seed BD-162 (format and bricks, other unit) |
| §19.3.1 | Optical cards and optical volumes [S] | N19a-06 (cards, volumes, lighting, FROB-REF-316), BD-229 (two maps, latents) |
| §19.3.2 | Fixtures and declared behaviours | N19a-07 (fixture library), N19a-08 (behaviours, movables, moving surfaces, FROB-REF-315), BD-229 (tables in the pack) |
| §19.3.3 | What is shared and what is per world | N19a-09 (pools, INV-HAB-03, leak fixture) |
| §19.4 | The scene compiler: stage 1 | N19a-10 |
| §19.4 | stage 2 | N19a-11 |
| §19.4 | stage 3 | N19a-12 |
| §19.4 | stage 4 | N19a-13 |
| §19.4 | stage 5, field layout, truncation, PO-HAB-01 | N19a-14 (bound and its verification), N19a-15 (region labels in bricks); seeds BD-162, BD-163, BD-164 (other units) |
| §19.4 | stage 6, overlays | N19a-16 (primitives, PO-HAB-06), N19a-17 (emission, index, composition, FROB-REF-306) |
| §19.4 | stage 7 | BD-229, N19a-06 |
| §19.4 | stage 8 | N19a-18 |
| §19.4 | stage 9 | N19a-19 (spawn poses), BD-234 (clearance complex and tag) |
| §19.4 | stage 10 | BD-228 |
| §19.4 | stage 11 and receipts | N19a-20 |
| §19.4 | manual trilinear sampling | N19a-03 (ADR-HAB-01), N19a-14; seed BD-162 and kernel BD-166 (other units) |
| §19.4 | end to end | N19a-42 |
| §19.5 | Scene acceptance [S] (verb, report, completeness) | BD-228, N19a-43 (end to end) |
| §19.5.1 | Registration: four points | N19a-21 |
| §19.5.2 | One-sided enclosures | N19a-22 |
| §19.5.3 | Collision-to-visual deviation | N19a-23 |
| §19.5.4 | Optical properties are separate from friction | BD-229, N19a-44 (end to end) |
| §19.5.5 | Infeasible cases are kept, tagged and counted | BD-234 (tags), N19a-26 (quotas, classes, no-resample audit, counts) |
| §19.5.6 | Acceptance fixtures (eight) | N19a-24 (board, normals, ray and visual registration, tile continuity), N19a-25 (gauges, support, ripple), N19a-09 (mutable-state leak), BD-228 (runner) |
| §19.6 | Terrain generators [S] | N19a-27 (heightfield, overhangs, material layers), N19a-28 (ten families, difficulty, curriculum grid), N19a-29 (mixed route), N19a-30 (end to end and known answers); seed BD-165 (framework and verb, other unit) |
| §19.7 | Interior generators: seven archetypes | N19a-31 to N19a-37 (archetype rows with their hazards), N19a-45 (end to end), BD-399 (asset-populated interiors), N19a-40 (pilot), N19a-41 (KILL-DRESSING); seeds BD-225, BD-226 (grammar and proxies, other unit); kinematic actors: epic BD-421 (other unit), declared in N19a-07 and N19a-08 |
| §19.8 | Hazard lists | N19a-31 Home (10 rows), N19a-32 Apartment (5), N19a-33 Office (7), N19a-34 Warehouse (10), N19a-35 Manufacturing plant (8), N19a-36 Retail store (8), N19a-37 Commercial kitchen (11); N19a-38 (incidence export, uncovered-hazard query), N19a-39 (provenance promotion, FROB-REF-312); seed BD-227 (schema, verb, other unit) |
| (seed) | A-13 self-collision and clearance (routed here by keyword; content from §28.4, §14.11, §18.20) | BD-104 |
| Appendix M.18 | Known answers of §19 | N19-KA |

## (b) Gates, kill gates and the beads that measure or decide them

| Gate | Carried by this unit | Also expected from |
|---|---|---|
| GATE-HAB-01 (field against source; overlays; ripple; M2) | N19a-14, N19a-16, N19a-17, N19a-25 (ripple), N19a-42 (end to end) | BD-163 (U19c) |
| GATE-HAB-02 (1,000 seeds per archetype; M3) | BD-234, N19a-26, N19a-31 to N19a-37, N19a-45 (end to end) | BD-225, BD-226 (U14b) for the constraint clause |
| GATE-HAB-04 (scene acceptance; M3) | BD-228, N19a-09, N19a-21, N19a-22, N19a-23, N19a-24, N19a-25, N19a-43 (end to end) | |
| GATE-HAB-05 (optics and friction separate; M3) | BD-229, N19a-44 (end to end) | |
| KILL-DRESSING (M5) | N19a-41 (decision), BD-399 (mechanism measured), N19a-40 (pilot) | BD-421 (U38a) for actors |
| GATE-ATEL-02 (analyses pass their known answers; stage budget) | BD-104 (A-13 rows only) | Atelier units |
| GATE-HAB-03 (real place; M5) | not this unit | BD-398 (U33a), U19c |
| GATE-HAB-06 (film law as scenes consume it; M3) | not this unit; values cited in N19a-35, N19a-37 | BD-230 (U18a), U19b |
| GATE-HAB-07, -08, -09 (advected film; slosh and proxies; proxy adequacy) | not this unit | U19b; BD-413, BD-414, BD-415, BD-416 |
| GATE-HAB-10 (clearance complexes; M3) | not this unit; consumed by BD-234 | BD-233 (U19b) |
| GATE-HAB-11 (retrofit; M4) | not this unit | BD-369 (U19b) |
| GATE-HAB-12 (world bank; M3) | not this unit | BD-236 (U19c) |
| GATE-HAB-13 (brick pool; M2) | not this unit | BD-164 (U19c) |
| GATE-HAB-14 (hygiene; M3) | counts supplied by N19a-26 | BD-235 (U19c) |
| GATE-HAB-15 (far-field reuse; M6), GATE-HAB-16 (scaled instances; M2) | not this unit; the sphere case of GATE-HAB-16 appears as a known answer in N19a-11 | U19c |
| KILL-ADVECT, KILL-LIVING, KILL-PROXY, KILL-SLOSH | not this unit | U19b |
| KILL-FCLEAR, KILL-RETROFIT | not this unit; fallback behaviour of the tag stated in BD-234 | U19b (BD-233, BD-369) |
| KILL-INGEST, KILL-POOL, KILL-BANK-BREADTH, KILL-FARFIELD | not this unit | U19c, BD-398, BD-164, BD-236 |

## (c) Counts

- Known answers: 39 in the registry N19-KA (KA-426 to KA-464). Of these, 22 are carried with values in this unit's
  mechanism and test beads (KA-426 to KA-436, KA-438, KA-440, KA-446, KA-448, KA-449, KA-451, KA-453, KA-457,
  KA-458, KA-461, KA-464). BD-104 additionally carries KA-281 (from the design chapter).
- Counterexamples: 13 of the chapter's 16 cited with their cases in this unit's beads (CX-184 to CX-190, CX-192 to
  CX-195, CX-197, CX-199). CX-191, CX-196 and CX-198 belong to later subsections. The registry N19-CX is written
  by U19c.
- Refusal codes: 10 of the chapter's 17 specified with meaning, trigger, ranked repairs and exit code in this
  unit's beads (`FROB-REF-301`, `-302`, `-303`, `-306`, `-309`, `-310`, `-312`, `-315`, `-316`, `-317`); `-305`
  and `-307` are referenced with their trigger. BD-104 carries `FROB-REF-123`. The registry N19-REF is written by
  U19c.
- Invariants carried with a test: INV-HAB-01 (N19a-04, N19a-20), INV-HAB-02 (N19a-23), INV-HAB-03 (N19a-09),
  INV-HAB-05 (N19a-26, BD-234), INV-HAB-10 (N19a-20, N19a-30, N19a-45); referenced: INV-HAB-04 (N19a-13),
  INV-HAB-06 (N19a-22), INV-HAB-09 (N19a-05).
- Proof obligations: PO-HAB-01 (N19a-14), PO-HAB-06 (N19a-16), PO-HAB-07 (N19a-21).
- Architecture decisions: ADR-HAB-01 to ADR-HAB-12, all in N19a-03 and restated where they govern.
- Hazard rows: 59 (10, 5, 7, 10, 8, 8, 11), all carried verbatim in N19a-31 to N19a-37.
- Subsections: the preamble and §19.1 to §19.8 (8 subsections, with 9 sub-subsections).
- Gates measured or decided here: GATE-HAB-01, -02, -04, -05 and KILL-DRESSING; a contribution to GATE-ATEL-02.

## (d) Needs from other slices

| Need | Asked by | Note |
|---|---|---|
| An admission test for the FrankenSim representation crates `fs-rep-sdf`, `fs-rep-mesh`, `fs-rep-frep` and `fs-topo` through the airlock's geometry feature group (INT-FSIM-08 admits only `fs-scene` and `fs-query` boxes) | N19a-13 | §33.10; no INT row or seed key found |
| An admission test for `fs-motion` (swept charts) through the airlock | BD-104 | §33; no INT row names it |
| The registry name of the `analyze.<name>` verb for A-13 | BD-104 | §28.4, Appendix J |
| The optical frame convention of camera sensors, for the glTF camera check | N19a-21 | §20.6 |
| A refusal code for running on a pack whose acceptance report is incomplete or failed | BD-228 | §19.16 has none in 301 to 317; N19-REF owner (U19c) |
| `scene.import` for single assets at M3 (decode, compile, loss report, licence entry) | BD-399 | The seed for `scene.import`, BD-398, is the M5 real-place importer; no M3 asset-import seed was found |
| The owner decision on OPEN-HAB-1 (dimension norms), with its fail-closed default | N19a-39 | §19.20 is U19c's; expected as an `owner-decision` bead there |
| Decision or Considerations entries for OPEN-HAB-2 (brick apron) and OPEN-HAB-3 (default spacings) | N19a-14, N19a-15, N19a-25 carry them in Considerations | §19.20, U19c |
| Registries N19-CX and N19-REF | every bead that cites CX or refusal numbers | U19c |
| Seed beads written elsewhere that this unit's beads depend on | see deps | BD-162 (U19b); BD-163, BD-164 (U19c); BD-165 (U18a); BD-225, BD-226 (U14b); BD-227 (U14a); BD-230 (U18a); BD-231 (U41); BD-233 (U19b) |

## (e) Contradictions, gaps and what was done

1. **KA-438 tolerance.** Appendix M says the squeeze-film times hold within 1%; §19.18, GATE-HAB-06 and the film
   law's own gate say within 2%. N19-KA records both and takes the gate's 2%.
2. **KA-445 tile count.** Appendix M says a 6 m × 6 m wet floor is 352 tiles (about 360 kB); §19.10.3 counts whole
   tiles, 19 × 19 = 361 (about 370 kB). N19-KA records both and takes 361.
3. **KA-460 resident counts.** Appendix M's M5 Ultra figures (about 2,500 homes at 2 cm, and so on) correspond to
   dividing 189 by the scene size in GB; §19.15's table (about 2,700) corresponds to about 203 GB, which is
   189 GiB. N19-KA records both and notes that the chapter's arithmetic is the consistent one.
4. **Palette bytes.** §19.4 says a palette of four region identifiers with a two-bit selector per sample is
   128 bytes; 512 samples × 2 bits is exactly 128 bytes, so that figure counts selectors only. N19a-15 stores the
   four identifiers in the brick's header record and says so.
5. **Acceptance report inside the identity.** The manifest carries the hash of the acceptance report and `SceneId`
   hashes the manifest, while the report holds values measured per tier and device and the identity must be equal
   on Linux and macOS. N19a-20 and BD-228 split the report into an identity part (reproducible verdicts and
   bounds, hashed) and a measurement part (per tier, stored beside the pack). This is a DESIGN resolution that
   the owner should confirm.
6. **Settled poses and cross-platform identity.** Stage 8 stores poses produced by a dynamics run; bit equality
   across platforms depends on deterministic elementary functions in the referee (an upstream item). N19a-18
   rounds stored poses to a fixed grid and leaves the cross-platform test as the judge.
7. **Layering of acceptance and settle.** `frob-scene` is layer 2, but stage 8 (settle on the referee) and several
   acceptance fixtures (ray kernels, the referee, batched tiers) need crates above it. The plan states the
   inversion explicitly only for the retrofit search. N19a-18, N19a-24, N19a-25 and BD-228 use service traits
   defined in `frob-scene` and supplied by the verb executor.
8. **No refusal code for an unaccepted pack.** §19.5 says a compiled scene is not usable until its acceptance
   report is complete, and §19.16 defines no code for that. BD-228 states the behaviour and asks for a code.
9. **`scene.retrofit` milestone.** The verb table of §19.1 gives M3; its gate GATE-HAB-11 and seed BD-369 are M4.
   N19a-02 registers the row at M3 and notes that the gate's milestone governs acceptance.
10. **`scene.fit` and `scene.import` owners.** §19.1 gives `scene.fit` at M3 and `scene.import` for assets at M3,
    while the seed that names both verbs (BD-398) is M5. N19a-02 attributes `scene.fit` to the clearance seed
    BD-233 (M3) and flags asset import as a need.
11. **BD-104 routing.** The seed "A-13 self-collision and clearance" was mapped to this chapter by the word
    "clearance". Its content is an Atelier analysis in `frob-analyze` (preflight stage 3). It was written in full
    from §28.4, §14.11, §14.13 and §18.20 with parent WP-11 and label `atelier`.
12. **Hazard rows with a Frontier mechanism and no Solid alternative.** Appendix G says rows marked [F] are
    represented by the Solid alternative beside them until the mechanism lands, but some rows have no alternative
    (pets, shoppers, floor vibration) and the sloshing-payload row is not marked [F] although slosh is Frontier.
    N19a-31 to N19a-38 treat such rows as `not-represented` until the mechanism lands, with rigid ballast for the
    hot-liquid row (the fallback of KILL-SLOSH), and report them through the uncovered-hazard query.
13. **Quota tolerance.** GATE-HAB-02 asks for the intended-impossible share within one percentage point of quota
    over 1,000 seeds; independent draws at a 5% quota have a standard deviation of 0.69 points and would miss
    the band about one run in seven. N19a-26 assigns the quota by a stratified rule that is a pure function of
    the seed.
14. **Half a fine voxel against the Lipschitz bound.** The a-priori trilinear error of a 1-Lipschitz field is
    (√3/2)·s, above the gate's half voxel. N19a-14 treats the half-voxel figure as the empirical TARGET measured on
    surfaces and the per-brick bound as the obligation, and records the box-corner case (0.498·s).
15. **Ripple slack.** At 10⁵ N/m and eight contact samples the worst-case ripple is 0.977 N against 1% of a 10 kg
    block's weight, 0.981 N. N19a-14 and N19a-25 record sample count and stiffness with every measurement.
16. **Verb count.** The §19.1 table has eleven rows and twelve verb names (`scene.list` and `scene.show` share a
    row). N19a-02 registers twelve names.
17. **Tolerances "to set".** Normal direction, tile-continuity step, silhouette threshold and the deviation
    refusal threshold have no number in the plan. The beads state DESIGN proposals, mark them as to be frozen
    before the gate run, and N19-KA lists the nine known answers whose tolerance is unset.
18. **Possible overlap with seeds written elsewhere.** N19a-10 to N19a-20 detail compiler stages whose umbrella
    seed is BD-163; N19a-27 and N19a-28 detail terrain whose umbrella is BD-165; N19a-31 to N19a-39 detail hazard
    data whose schema seed is BD-227. Each states its boundary in Scope; the lead may wish to make the umbrella
    seeds depend on these subtasks rather than the reverse where the other unit wrote them thin.

## (f) Added by the integration pass

- N19a-46 (new, M3): the asset lane of the verb `scene.import` (§19.1 verb table, §19.9), which BD-399 and BD-398
  both asked for and no unit had written. It defines the loss report schema that the real-place lane (N19c-04)
  extends. BD-399 now depends on N19b-02 (licence manifest and `FROB-REF-310`) and on N19a-46.
- Scopes re-cut against the sibling units' beads: N19a-14 and N19a-15 against the brick codec N19b-01; N19a-20
  against the manifest BD-162 (one hashing rule for `SceneId`); BD-399 against N19b-02; N19a-02's "implemented by"
  column (N19a-46, N19b-32, N19b-40, N19c-05) and the milestone of `scene.retrofit` (M4).
- Verb names follow the verb registry: A-13 is `analyze.self-collision` (BD-104).
