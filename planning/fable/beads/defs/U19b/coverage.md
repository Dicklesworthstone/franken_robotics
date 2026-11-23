# Coverage of drafting unit U19b

Chapter §19 Habitat, subsections §19.9 to §19.12, and the seed beads BD-162, BD-233, BD-369, BD-414, BD-415, BD-420.
49 bead files: 6 seed beads and 43 new beads (`N19b-01` to `N19b-43`). No registry bead is owned by this unit
(`N19-KA` is U19a's; `N19-CX` and `N19-REF` are U19c's).

## (a) Subsections and the beads that carry them

| Subsection | Title | Beads |
|---|---|---|
| §19.9 | Assets and licences | N19b-02 |
| §19.10 (intro, tier table) | Living worlds in three tiers | N19b-03 (declarations, flags, tier admission, recorded fallbacks); BD-414, BD-415 (epics); N19b-04 (ADR-HAB-07 record) |
| §19.10.1 | The calculations that remove the naive versions | N19b-04 (all five calculations and the island cost as tests; CX-192); cited by N19b-20 |
| §19.10.2 | Solid: film-friction patches and rigid proxies | N19b-05 (patch, thickness state, exact update, PO-HAB-04); N19b-06 (GATE-HAB-06 through a scene); N19b-03 (rigid-proxy declarations and the castor arithmetic); N19b-22 (validity hypothesis, OPEN-HAB-8). The law's cone row is BD-230 (U18a); rigid-proxy bodies are BD-231 (U41) |
| §19.10.3 | Frontier: robot-advected films | N19b-07 (pilot); N19b-08 (field, tiles, pool, reservoirs, admission, FROB-REF-307); N19b-09 (face-flux update, pickup and redeposition, PO-HAB-05); N19b-10 (GATE-HAB-07); N19b-11 (KILL-ADVECT). The epic BD-413 is U33b's |
| §19.10.4 | Frontier: payload slosh | BD-414 (epic); N19b-12 (pilot); N19b-13 (card, admission, module, validity); N19b-14 (slosh rows of GATE-HAB-08); N19b-15 (KILL-SLOSH and the ballast-and-spill fallback) |
| §19.10.5 | Frontier: articulated cord and rug proxies | BD-415 (epic); N19b-17 (chain cord, hinged-plate rug); N19b-18 (proxy rows of GATE-HAB-08, cost); N19b-19 (KILL-PROXY); N19b-16 (pilot) |
| §19.10.6 | Moonshot: coupled islands | N19b-21 (island declaration, exchange contract, oracle and counterexample harness); N19b-04 (cost arithmetic). The epic BD-416 is U33a's |
| §19.10.7 | The proxy-adequacy pilot and the kill rule | N19b-16 (pilot, GATE-HAB-09); N19b-20 (KILL-LIVING); N19b-19 (KILL-PROXY); N19b-11 (KILL-ADVECT) |
| §19.11 | Clearance complexes (`.fclear`) [F] | BD-233 (capstone); N19b-25 (pilot); N19b-26 (cells, bounds, PO-HAB-02, INV-HAB-06); N19b-27 (format); N19b-28 (three answers, sweep, bottleneck, PO-HAB-03); N19b-29 (local refinement); N19b-30 (slabs, transitions, modes, labels, unknown, FROB-REF-308); N19b-31 (certificates, checkers, T-31); N19b-32 (`scene.fit`, FROB-REF-309); N19b-33 (`contact.clearance`, GATE-TRAC-07); N19b-34 (A-18); N19b-35 (GATE-HAB-10); N19b-36 (KILL-FCLEAR) |
| §19.12 | The retrofit catalog [F] | BD-369 (capstone, NS-18); N19b-37 (pilot); N19b-38 (catalog, typed edits, constraints, FROB-REF-313); N19b-39 (localize, kits); N19b-40 (`scene.retrofit`, INV-HAB-08); N19b-41 (GATE-HAB-11); N19b-42 (KILL-RETROFIT); N19b-43 (OPEN-HAB-4) |
| §19.3, §19.4 (owned by U19a; read for BD-162) | The scene pack; field layout | BD-162 (manifest, `SceneId`, ranged fields); N19b-01 (band-brick codec, bound, reference sampler) |
| §43.9, §20.7, §20.9 (read for BD-420) | RGB and splat scenes | BD-420 (epic); N19b-23 (short plan, kill gate, owner decision); N19b-24 (pilot) |

Identifiers carried. ADR-HAB-01, -04, -12 (BD-162); ADR-HAB-02 (N19b-01); ADR-HAB-07 (N19b-03, N19b-04); ADR-HAB-08
(BD-233, N19b-28); ADR-HAB-09 (BD-369, N19b-38). INV-HAB-01, -10 (BD-162); INV-HAB-06 (N19b-26, N19b-28, N19b-35);
INV-HAB-07 (N19b-09, N19b-10); INV-HAB-08 (N19b-40, N19b-41); INV-HAB-02 (BD-420, N19b-23, as a constraint); INV-HAB-03
and INV-HAB-09 are cited where living state and retrofit variants touch them and are owned by the other §19 units.
PO-HAB-01 (statement and bound formula in N19b-01; verification in BD-163); PO-HAB-02 (N19b-26); PO-HAB-03 (N19b-28,
N19b-35); PO-HAB-04 (N19b-05); PO-HAB-05 (N19b-09). OPEN-HAB-2, -3 (BD-162, N19b-01 Considerations); OPEN-HAB-4
(N19b-43); OPEN-HAB-8 (N19b-22). BET-36, BET-37; GOAL-32, GOAL-33; NS-18 (BD-369); C-33, C-07; T-31; A-18, A-07;
RISK-11 (N19b-02), RISK-31 (N19b-17), RISK-41 (BD-233); FD-FSIM-22 (BD-415, N19b-21); NONGOAL-01 (BD-420).

## (b) Gates and kill gates of §19, and the beads that measure or decide them

| Gate | Measured or decided by (this unit) | Expected from another unit |
|---|---|---|
| GATE-HAB-01 | — | BD-163 (U19c); N19b-01 supplies the codec known answers |
| GATE-HAB-02 | — | BD-234, BD-225, BD-226 (U19a, U14b) |
| GATE-HAB-03 | — | BD-398 (U33a) and the §19.13 beads (U19c) |
| GATE-HAB-04 | — | BD-228 (U19a) |
| GATE-HAB-05 | — | BD-229 (U19a) |
| GATE-HAB-06 | N19b-06 | BD-230 (U18a) also names it |
| GATE-HAB-07 | N19b-10 | BD-413 (U33b) names it as the epic's done-condition |
| GATE-HAB-08 | N19b-14 (slosh rows), N19b-18 (proxy rows); BD-414, BD-415 | — |
| GATE-HAB-09 | N19b-16; BD-415 | — |
| GATE-HAB-10 | N19b-35; BD-233 | — |
| GATE-HAB-11 | N19b-41; BD-369 | — |
| GATE-HAB-12 | — | BD-236 (U19c) |
| GATE-HAB-13 | — | BD-164 (U19c) |
| GATE-HAB-14 | — | BD-235 (U19c) |
| GATE-HAB-15 | — | the §19.21 beads (U19c) |
| GATE-HAB-16 | — | the §19.22 beads (U19c) |
| KILL-ADVECT | N19b-11 | — |
| KILL-SLOSH | N19b-15; BD-414 | — |
| KILL-PROXY | N19b-19; BD-415 | — |
| KILL-LIVING | N19b-20 | BD-416 (U33a) names it as the epic's done-condition |
| KILL-FCLEAR | N19b-36; BD-233 | — |
| KILL-RETROFIT | N19b-42; BD-369 | — |
| KILL-DRESSING | — | the §19.7 beads (U19a) |
| KILL-INGEST | — | the §19.13 beads (U19c) |
| KILL-POOL | — | BD-164 (U19c) |
| KILL-BANK-BREADTH | — | the §19.15 beads (U19c) |
| KILL-FARFIELD | — | the §19.21 beads (U19c) |
| GATE-TRAC-07 (a §18 gate named by seed BD-233) | N19b-33; BD-233 | — |

Six gates and six kill gates of §19 are carried here, plus GATE-TRAC-07. BD-420 registers a new kill gate for RGB and
splat scenes (working name KILL-RGBSPLAT) through N19b-23; it does not exist in the register yet.

## (c) Counts carried

- Known answers of §19 cited with their values: 24 of 39 (KA-426, KA-427, KA-428, KA-430, KA-437 to KA-455, KA-458). The
  other 15 belong to the subsections of U19a and U19c. Also cited from other chapters: KA-412 (§18), KA-689 (§28),
  KA-772 (§33), KA-827 and KA-832 (§37).
- Counterexamples of §19 cited with their cases: 10 of 16 (CX-186, CX-187, CX-188 to CX-195). Also CX-168 (§18).
- Refusal codes of §19 carried with meaning, trigger and repairs: 8 of 17 (FROB-REF-303, -304, -305, -307, -308, -309,
  -310, -313). Also FROB-REF-321 (§18), FROB-REF-891 to -893 (§34), FROB-REF-733 (§37, by reference).
- Verbs implemented: `scene.fit` (N19b-32), `scene.retrofit` (N19b-40), `contact.clearance` (N19b-33), the A-18 analysis
  verb (N19b-34).

## (d) Needs from other slices

- U18a (BD-230): the film law's cone row and `K-film` read the patch table and thickness lifecycle of N19b-05; BD-230
  should list N19b-05 as a prerequisite. Whether the viscous term is capped at μ_b must be settled there (see (e) 5).
- U18a (BD-232): outer and inner enclosures with registered error bounds for N19b-33; swept bounds that enclose arcs.
- U19a: semantic labels that name doors and openings; route graphs with typed transitions and their dimensions; the
  record of hazards drawn per feature; per-world mutable state pools and the leak fixture (INV-HAB-03); the canonical,
  timing-free acceptance report whose hash `.fscene` carries; BD-234 consumes `nominal_feasibility` from BD-233.
- U19c: the per-variant directory behind the trait N19b-01 defines (BD-164); split groups for retrofit variants (BD-235);
  scale labels, the unknown mask and `scene.probe-request` data for `scene.fit` on ingested scenes; the registries
  `N19-CX` and `N19-REF` should include the rows this unit cites.
- U41 (BD-231): the free-slab rug and rod bodies that N19b-17 and N19b-18 use as Solid alternatives and controls.
- U33b, U33a: INT-FSIM-17 (the fluid bridge) and INT-FTX-01 for N19b-21; BD-413 and BD-416 are the parents of the
  advected-film and island beads written here.
- U12 (Lemma): the `complex-connectivity` row of the proof-kind registry, its payload schema, and how a bundle carries
  megabyte blobs; the kind under which a separating hyperplane travels for `contact.clearance`.
- U32a (Tether): the task cluster for `scene_fit` and `scene_retrofit`; the run-admission hook for FROB-REF-309; the
  refusal code for a capability not admitted (exit 5); the identity-mismatch integrity code.
- U26a, U26b (Nemesis): reconciliation of the catalog's `changes` vocabulary with the edit classes; typed access to a
  counterexample world's edits; the paired-seed rule for "the same failure".
- U28a (Atelier): the verb name of analysis A-18; a design-search entry point returning the cheapest verdict-flipping
  design edit priced from part cards; the frozen design set and ranking metric used by KILL-ADVECT and KILL-SLOSH.
- U24 (Arena): fixture tasks (`film-pad`, `spill-track`, `film-release`, `tray-shake`, `hanging-chain`, `castor-rug`);
  the multi-pass low-friction and open-container task variants; the home battery.
- U39a: the pinned incumbent engine with cable, cloth and fluid for the proxy-adequacy pilot.
- U30 (Ground): frozen predictions and sealed trials for the spill-pad import lane.
- Owner decisions: OPEN-PROG-5 (which M6 epics are funded), OPEN-HAB-4, OPEN-HAB-8.

## (e) Contradictions, gaps and what was done

1. **Squeeze-film tolerance.** Appendix M row KA-438 prints 1% for GATE-HAB-06; §19.18, §19.19 and §40 print 2%, and the
   worked-card appendix records that both gates use 2%. Used 2%.
2. **Film tile count.** §19.10.3 counts whole tiles (361 tiles, about 370 kB for a 6 m floor); Appendix M row KA-445
   prints 352 and about 360 kB (the unrounded area ratio). Used 361, with the difference stated in N19b-08.
3. **Milestone of the retrofit verb.** The verb tables print M3 for `scene.retrofit`; §40 moves GATE-HAB-11 to M4 with
   the catalog and the seed BD-369 is M4. Used M4 (the register governs).
4. **Schema names.** Chapter prose has `frob.fscene/1` and `frob.fclear/1`; §34.9 has `frob.scene/1` and
   `frob.clearance/1` and owns schema names. Used the registry's.
5. **Film law form.** §19.10.2 and KA-439 cap the viscous term at μ_b; §18.8 and INV-TRAC-07 state uncapped Couette shear
   above 3σ. §18 owns the law. N19b-05 and N19b-06 assert only the limits both forms share and flag the cap for BD-230.
6. **Domain of the corridor closed form.** L ≤ 2√2·a − 2b is the rotating manoeuvre; a rectangle with both sides at most
   a also passes by pure translation, so for b above about 0.914·a the true limit is L ≤ a. The gate's 100 pairs are kept
   at b ≤ 0.85·a (N19b-35), ten wide pairs are run against max(a, 2√2·a − 2b) and reported, and the pilot notes it.
7. **Heading bins.** C-33 mentions 48 bins as an example; §19.11's default lattice has 32. Used 32.
8. **Edit-aspect vocabularies.** The catalog's `changes` are geometry, material, optics, clocks, actors; the adversarial
   search's classes are Geometry, Material, Optics, Fixture, Clock and link, Disturbance, Parameter, and the `.fmcw`
   example prints `fixture` and `payload`. N19b-38 states a mapping (actors ↔ Fixture, clocks ↔ Clock and link) and asks
   for reconciliation.
9. **PO-HAB-02 "as a theorem card".** No card T-01 to T-35 states the cell bound (T-31 is about verdicts following from
   stored complexes). N19b-26 carries a written proof on file and a randomized check; a card number is the Lemma unit's
   to assign.
10. **Clearance checker and the offline auditor.** §19.11 says the checker recomputes c(q₀) and needs only the scene
    field; §12 says `frob-verify` is standard-library only and reads only bundles; T-31 speaks of stored complexes.
    N19b-31 splits the check: connectivity on the stored complex in `frob-cert` (`complex-connectivity`, T-31) and
    recomputation against the scene in `frob-scene`.
11. **Proof kind for `contact.clearance` certificates** is not named. N19b-33 proposes `lp-farkas` for separating
    hyperplanes, pending the registry owner; until settled the answer is labelled `Tested` (fail-closed).
12. **BD-420.** The plan defines neither "splat scene" nor a kill gate for the epic, and INV-HAB-02 forbids visual-mesh
    data in results. The epic is written as preparation only (short plan, pilot, kill-gate registration with a working
    name), with the constraints the plan must honour; no mechanism bead exists until the owner funds it.
13. **Unstated numbers and names fixed here as DESIGN, each labelled in its bead:** the three slosh fill depths (0.05,
    0.10, 0.15 m at L = 0.30 m); the rank statistic of KILL-ADVECT (Kendall τ-b); the number for KILL-PROXY's falsifier
    (0.90, mirroring KILL-LIVING); the six mutated certificates of GATE-HAB-10; the A-18 verb name (`analyze.clearance`);
    the film-update kernel name (`K-film-advect`); the range of 16-bit coarse samples (`coarse_truncation_m`); the eight
    palette-identifier bytes beside the 128 selector bytes; the spill metric; modal masses and heights for slosh (a
    candidate expression marked HYPOTHESIS with sanity tests; heights default to the liquid centre, marked `assumed`);
    the refusal used for an out-of-range stored value (`FROB-REF-303`); admitted parameter ranges of the two paper pilots
    (registered as `assumed`).
14. **Film thickness representation.** The plan implies floating-point cells with a compensated-sum audit; N19b-08
    stores integer quanta so that conservation is exact and bit-identical, keeps the float form as a fallback, and
    leaves the gate's criterion unchanged.
15. **Seed done-conditions that name gates and kill decisions** (BD-233, BD-369) cannot be met by a first-in-chain
    mechanism bead without a cycle. Both seeds are written as capstones that depend on their parts, the gate battery
    and the kill decision; other units' beads that depend on them therefore wait for the gated capability.
16. **BD-162** was listed with best-matching subsection §19.10 but draws on §19.3 and §19.4 (U19a's subsections); it is
    written completely from those, with the brick codec split out as N19b-01.
17. **GATE-HAB-08's rug row** tests a Solid proxy delivered at M3 (BD-231) inside an M6 gate; N19b-18 measures it at M6
    on BD-231's body.
18. **KILL-LIVING's cost trigger** ("more than 181 rigid worlds at 5% activity") is stated for batch activity although
    islands never run in a batch; N19b-20 records the reading it uses (the island cannot serve its two offline purposes
    within the finalist budget) and the arithmetic either way.
19. **Hazard dimensions for containers.** The kitchen hazard row gives fill 30 to 80% and no container size; the slosh
    pilot (N19b-12) must register an `assumed` container box, which is its main weakness and is stated there.

## (f) Added by the integration pass

- N19b-44 (new, M3, Solid): typed scene edits (§19.1, §19.12): the closed set of edit kinds, `apply` on an immutable
  pack, the lineage record and the declared-aspect check. The adversarial search, scene groups (N19c-11) and the hygiene
  gate script (N19c-15) need edits at M3, a milestone before the catalog; N19b-38 (M4) now instantiates these kinds and
  depends on N19b-44.
- The epics BD-414, BD-415 and BD-420 carry no dependencies; the seed dependency of BD-414 (BD-231) is held by N19b-13.
- A-18's verb is `analyze.route`, the verb registry's name (N19b-34); N19b-32 depends on N19b-34 for the footprint
  family of a design.
- ADR-HAB-07's file is the one N19a-03 creates (`docs/adr/ADR-HAB-07.md`); kill-gate decision records use the gate
  register's path `docs/delivery/adr/ADR-<gate>-<milestone>.md` in all three units.
