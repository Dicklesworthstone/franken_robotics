# Coverage of drafting unit U14b

Chapter §14 Skeleton, subsections §14.10 to §14.17, plus eleven seed beads (six of which draw on §17, §19 and
§22). 47 bead files: 11 seed beads, 34 new beads `N14b-01` to `N14b-34`, and the registries `N14-CX` and `N14-REF`.

## (a) Subsections and the beads that carry them

| Subsection | Title | Beads |
|---|---|---|
| §14.10 | Archetypes: the eighteen | BD-109 (catalog, `design.templates`, the first four); N14b-02 (quad-sprawl, hexapod-radial, octopod, biped-point); N14b-03 (ackermann, omni-3, swerve-4, rocker-bogie, balancer-2w); N14b-04 (tracked-flipper, wheel-leg, tri-wheel-stair, mobile-manipulator, snake-crawler); BD-246 (catalog closed over all eighteen, groups on file, mass rows, 24 planted asymmetries); N14b-01 (the open modular class and its generator); N14b-05 (the five other variants behind feature flags); N14b-06 (end-to-end script); BD-117, BD-247, N14b-07, N14b-08, N14b-09 (baseline controllers that make each archetype stand or roll); BD-144 and BD-185 (Phylum pilot and control arm built on the archetypes) |
| §14.11 | Preflight: eight stages that are `design check` | N14b-10 (stage table, budgets, `undecided`, options, analysis-to-stage map); N14b-11 (stage 4 and the both-tests counterexample); N14b-12 (stage 7); N14b-13 (stage 8); BD-244 (stages 7 and 8 integrated, the 40 planted cases); N14b-14, N14b-15 (pilots); N14b-16 (kill decision); N14b-17 (OPEN-SKEL-3); N14b-18 (100 seeded defects); N14b-19 (end-to-end script). Stages 1 to 6 as a runner are BD-110 (unit U28a); stage 1's checks are N14a-08 (unit U14a) |
| §14.12 | Import and export (`frob-io`) | BD-065 (XML reader, URDF import, macros expanded outside, `design.import`); N14b-20 (loss report, field-coverage diff, `--strict`, meaning comparator); N14b-21 (GLB export); N14b-22 (SDF feature matrix; OpenUSD); N14b-23 (OpenUSD pilot); N14b-24 (end-to-end script and GATE-SKEL-01 rows). MJCF import and URDF and MJCF export are BD-066 (unit U14a) |
| §14.13 | Refusals instead of repairs in silence | N14-REF (25 codes with payloads, repairs, exit codes); N14b-18 (the first rejection fixtures and the defect corpus); N14b-19 (repairs that repair) |
| §14.14 | Invariants, proof obligations, known answers | N14b-25 (INV-SKEL-01 to -09, PO-SKEL-01 to -04 as one battery; owns INV-SKEL-07 and -08 cross-cutting tests); BD-063 (INV-SKEL-01, -02, -06; PO-SKEL-03); the eight known answers of the subsection are cited with their values in BD-109, BD-244, N14b-03, N14b-11, N14b-13, N14b-25, N14-CX. The registry `N14-KA` is unit U14a's |
| §14.15 | Gates, kill gates, open questions | N14b-26 (every gate, kill gate, pilot and open question of the chapter pre-registered verbatim); N14b-16 (KILL-SKEL-ALIAS); N14b-17 (OPEN-SKEL-3); N14b-10 Considerations (OPEN-SKEL-2, OPEN-SKEL-4 with owners and defaults); N14b-14, N14b-15, N14b-23 (pilots of the Frontier table); measurement beads per table (b) |
| §14.16 | Module capability tables and the composed quasi-static screen (late) | N14b-27 (pilot); N14b-28 (`CapabilityTable`, `module.characterize`); N14b-29 (composed screen); N14b-32 (end-to-end script, GATE-SKEL-09); N14b-30 (GATE-SKEL-10 experiment, cascade wiring); N14b-31 (KILL-SKEL-CELL) |
| §14.17 | Folding fixed joints, and reduced models (late) | N14b-33 (the fold and the GATE-SKEL-11 battery); N14b-34 (reduced models, the mass-dropped refusal, `g1-reduced-30`); N14b-24 (outside-visible rows) |

Seed beads owned by this unit and the subsections they actually draw on: BD-063 (§14.9); BD-065 (§14.12);
BD-109, BD-246 (§14.10); BD-244 (§14.11); BD-117, BD-247 (§22.4, §22.18); BD-144, BD-185 (§17.5, §17.14);
BD-225, BD-226 (§19.7, §19.8, §19.15).

## (b) Gates, kill gates and admission gates

| Gate | Beads of this unit that measure or decide it | Carried elsewhere |
|---|---|---|
| GATE-SKEL-01 | BD-065 (URDF rows), N14b-20 (coverage clause), N14b-24 (the recorded run) | BD-066 (U14a) for MJCF and export |
| GATE-SKEL-02 | BD-063; N14b-25 (battery) | — |
| GATE-SKEL-03 | BD-109 (M1 rows), BD-246, N14b-02, N14b-03, N14b-04 (compile, preflight, groups), BD-247 (standing rows), N14b-06 (script) | BD-118 (standing rows at M1) |
| GATE-SKEL-04 | N14b-10, N14b-11, N14b-18, N14b-19 (stages 1 to 6); BD-244, N14b-12, N14b-13 (stages 7 and 8); N14-REF | BD-110 (U28a) records stages 1 to 6; N14a-08 (U14a) stage 1 |
| GATE-SKEL-05 | N14b-25 (battery rows only) | Expected from U14a: BD-057 measures it |
| GATE-SKEL-06 | N14b-25 (battery rows only) | Expected from U14a: BD-077 measures it |
| GATE-SKEL-07 | BD-109, BD-246, N14b-04, N14b-06 (mass clause); N14b-25 (shell clause in the battery) | BD-062 (U14a) measures the shell clause |
| GATE-SKEL-08 | none (registered in N14b-26) | Expected from U14a: BD-059 and its schema-version bead |
| GATE-SKEL-09 | N14b-28, N14b-29, N14b-32 | — |
| GATE-SKEL-10 | N14b-30 | — |
| GATE-SKEL-11 | N14b-33, N14b-34, N14b-24 | — |
| KILL-SKEL-ALIAS | BD-244 (measurement), N14b-13, N14b-15, N14b-16 (decision) | — |
| KILL-SKEL-CELL | N14b-30 (measurement), N14b-31 (decision) | — |
| KILL-SKEL-PORT | none (registered in N14b-26) | Expected from U14a (limb port packet, its pilot and its decision bead) and BD-385 |
| KILL-SKEL-SEXPR | none (registered in N14b-26) | Expected from U14a: BD-057 and its decision bead |
| KILL-SKEL-SUBST | none (registered in N14b-26) | Expected from U14a: BD-077, its pilot and its decision bead |
| GATE-REFLEX-01 | BD-117, BD-247, N14b-07, N14b-08, N14b-09 | BD-118 (M1), the M2 `batch-cpu` rows |
| GATE-MARROW-03 | BD-109, BD-246, N14b-02, N14b-03, N14b-04 (archetype side and the 24 planted asymmetries) | The symmetry beads of §13 and U14a |
| GATE-PHYLUM-03 | BD-185 (arm B's measures) | BD-186 publishes |
| GATE-HAB-02, GATE-HAB-13 | BD-225, BD-226 (constraint, identity and brick-count clauses) | BD-234 (tags), the pool bead |

All sixteen identifiers the brief lists are registered verbatim in N14b-26. I checked unit U14a's directory while
drafting: it contains decision beads for KILL-SKEL-SUBST, KILL-SKEL-SEXPR and KILL-SKEL-PORT and pilots for
substitution reuse and the limb port interface, so this unit wrote none of those.

## (c) Counts

- Known answers of §14 cited by number with their values: 31 of 32 (KA-261, KA-263 to KA-292); KA-262 is carried
  by content in N14-CX (the mass-spiral consequence of the fill error). From other chapters: KA-096, KA-254,
  KA-259, KA-418. Five known answers of the late subsections have no KA number and are stated in full: the two
  stance cases and the trot case (§14.16), the fold hand case and the 320 N reduced-model case (§14.17).
- Counterexamples: 10 numbered (CX-125 to CX-134) and 3 late, unnumbered (N14-CX); CX-118 and CX-121 cited in
  BD-246.
- Refusal codes: 25 (101 to 116, 118 to 126) and 1 unnumbered late refusal (N14-REF, N14b-34).
- Invariants and proof obligations: INV-SKEL-01 to -09 and PO-SKEL-01 to -04 (N14b-25).
- Open questions: OPEN-SKEL-1 (closed), -2, -3, -4. Pilots: 4 beads here (stage 7, stage 8, OpenUSD, capability
  tables); 2 expected from U14a.

## (d) Needs from other slices (no key known)

- §14 (U14a): the verb `design.new` creating a design from a template; the lever interface for optimizers; an
  optional top-level `reduced` block in the `frob.design/1` field reference (§14.17 adds it after the field
  reference was written).
- §14.9 (U14a and the Sinew and Marrow units): the binding, budget and symmetry functors as separately keyed
  compile products.
- §15.13: Sinew's codes 130 to 134 as raised by card checks and stage 6.
- §17.14: the frozen mutation corpus of structural edits for arms A and B.
- §18.9: the `roller-wheel` friction ellipse on the referee at M1 (the mecanum row of BD-117).
- §22.17: pilot beads for the Frontier rows of the controller class table (biped balance, hybrid, arm on a base).
- §28.14: the `Memo` trait and its store.
- §32.4: the envelope's `refusal` object and the exit-code table; §32: the viewer as first consumer of GLB.
- §33: admission of the airlock's `geometry` group for GLB export; admission of `fs-mbd::tri_wheel_cluster`;
  the admission test of the G1 catalog builder (§33.18).
- §35: host-side counter-based random streams addressed by (seed, index, stream).
- §38.6: a derivation file for the balancer's open-loop pole; §38.11: the negative-control registry's schema;
  §40: the row schema of `registries/gates.toml`.
- §16: dual-arm loops through a payload and a rod as a generalized joint (variants only, backlog).
- §21: the owner's approval of compute for the 200-assembly experiment if it exceeds the default envelope.

## (e) Contradictions, gaps and what was done

1. **A refusal with no code.** §14.17 refuses "mass dropped at import without a declared reduction" and Appendix
   O gives it no number. N14-REF and N14b-34 propose FROB-REF-127 (the next free code of the chapter's block)
   and leave the assignment to the registry owner. Code 117 is unassigned and is treated as reserved.
2. **Exit code of FROB-REF-112.** The exit table gives 3 for the `1xx` family and 7 for "stale parent". N14-REF
   uses 7 and flags it for the surface owner, because the parity gate needs exactly one exit per code.
3. **FROB-REF-125** is in the field reference and not in the stage table; N14b-10 places it at stage 1.
4. **`quad-sprawl`** is described as "stable and still fails A-11 thermally", while GATE-SKEL-03 requires every
   archetype to pass stages 1 to 6. N14b-02 makes the default pass and puts the failure in a named preset.
5. **GATE-SKEL-07 with several reference robots per archetype** (`quad-mammal` has six): read as one row per
   preset (BD-109, BD-246).
6. **No controller class for three archetypes.** The class table of §22.4 has no row for `balancer-2w`,
   `snake-crawler` or a six-wheel passive rover. BD-247 adds DESIGN rows (LQR balance; a travelling-wave shape
   gait; skid-steer on six wheels), built in N14b-07 and N14b-08.
7. **Milestone tensions.** (a) BD-117 asks the mecanum to roll at M1, but its roller law is scheduled later; the
   row is recorded `blocked: contact law` if the law is absent, as GATE-REFLEX-01's own restriction allows. (b)
   The humanoid is among the M1 archetypes, but biped balance is Frontier and lands with BD-247 at M3; its M1
   row depends on BD-116 and BD-118. (c) BD-185 (M2) needs trees and contact laws of archetypes that land at M3
   and M4; templates are authored from the layouts and families without laws are `not run`. (d) Stage 7 (M3)
   should name a retrofit, but the catalog lands at M4: `retrofit: catalog-absent` until then. (e) Capability
   characterization (M3) uses "the program of A-24", which lands at M4; N14b-28 specifies the program itself.
8. **BD-144's population.** §17.14 names "the design corpus", which exists only at M3; the seed names the first
   archetypes. The archetypes stand in and the pilot is re-run on the corpus.
9. **Default bucket set.** §17.5 and OPEN-PHYLUM-1 say {12, 18, 24, 36, 48, 64}; Appendix I's comment says 32
   where they say 36. BD-144 uses 36 and reports the 32 variant beside it. "Padding waste" is not defined in the
   plan; BD-144 reads it as Ω₁ − 1 per bucket.
10. **Compositions the plan leaves open, proposed here as DESIGN and frozen by hash before data:** the grouping
    of eighteen archetypes into nine template families (BD-185); the 40 planted cases (BD-244); the 24 planted
    asymmetries (BD-246); the 100 seeded defects by code (N14b-18); the 200 assemblies (N14b-30).
11. **§14.16 names no pilot or falsifier** although the mechanism is Frontier. N14b-27 defines one from the
    chapter's known answers and adds a random part whose outcome is not known in advance. The 26 directions are
    read as the nonzero vectors of {−1, 0, 1}³; the mapping of an `outer` exclusion to FROB-REF-124 at stage 6
    is DESIGN.
12. **Stage 4's "class the archetype declares"** has no field in `.fbot`. N14b-11 takes it from the bound task,
    else infers it from the declared contact laws.
13. **Truths named and not written.** KA-269 (the balancer's pole) has no formula; N14b-03 gives a candidate and
    requires the derivation file. KA-282 points at a belt-law row the Traction battery does not list; KA-418 is
    used.
14. **Late known answers and counterexamples have no numbers** (§14.16, §14.17). They are stated in full here;
    `N14-KA` (U14a) and the catalogue need rows for them.
15. **Fill default.** §11.5 says a body without a wall thickness is solid (with a plausibility refusal); §14
    says there is no default fill. §14 governs designs; N14-CX records the disagreement.
16. **Unexpanded xacro** is refused by BD-065 even without `--strict` (DESIGN), because dropping macro elements
    silently changes topology.
17. **Overlaps with unit U14a to reconcile at merge:** N14a-25 also covers GLB export (N14b-21 here);
    N14a-08 has stage-1 seeded defects (N14b-18 is the 100-defect corpus across stages 1 to 6); N14a-21 owns
    `design.new`, which BD-109 needs.
18. **Seed beads from other chapters.** BD-117 and BD-247 (Reflex), BD-144 and BD-185 (Phylum), BD-225 and
    BD-226 (Habitat) were assigned here by keyword. Each is written completely from its own chapter; the units
    that own §17, §19 and §22 should depend on them, not rewrite them.
19. **No new registry file for the stage table.** The plan forbids registries that nothing reads; N14b-10 keeps
    the stage table as a compiled constant with generated documentation. The random-design generator is a
    development tool (`frob-gen-designs`), not a verb, for the same reason.
