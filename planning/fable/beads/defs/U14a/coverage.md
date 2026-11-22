# Coverage of drafting unit U14a

Chapter §14 Skeleton, subsections §14.1 to §14.9 and the chapter preamble; ten seed beads; the known-answer
registry of §14. 44 bead files: 10 seed beads, 33 new beads `N14a-01` to `N14a-33`, and the registry `N14-KA`.

## (a) Subsections and the beads that carry them

| Subsection | Title | Beads of this unit | Beads of other units that carry part of it |
|---|---|---|---|
| §14 preamble | Skeleton: design language, modules, archetypes, preflight | BD-054 (crate `frob-design`), N14a-07 and BD-061 (crate `frob-model`), BD-066 (crate `frob-io`) | BD-065 (U14b) |
| §14.1 | What it owns | BD-054, BD-061, BD-066, N14a-08 (staging of stage 1) | BD-109, BD-110, BD-244 |
| §14.2 | Decisions | ADR-SKEL-01: BD-054. ADR-SKEL-02: BD-057, N14a-19, N14a-20. ADR-SKEL-03: BD-055, N14a-04, N14a-05, BD-062. ADR-SKEL-04: N14a-11, BD-055. ADR-SKEL-05: BD-077. ADR-SKEL-06: BD-055, N14a-02. ADR-SKEL-07: N14a-06, BD-055. ADR-SKEL-08: N14a-08. ADR-SKEL-09: N14a-07 (source paths), BD-061 and the functor beads (tracked reads) | ADR-SKEL-05: BD-060 (U15b). ADR-SKEL-08: BD-110. ADR-SKEL-09: BD-063 (U14b) |
| §14.3 | The design term | BD-054 (term, top-level shape), N14a-01 (parameters: unit, bounds, manifold, provenance, uncertainty, prior), N14a-02 (components and typed ports), N14a-07 (constructor expansion driver), N14a-11 (C₂ × C₂ as a property of the term; the `symmetry` block) | BD-056 (constructors, U41), N15b-01 (junction checks, U15b) |
| §14.4 | The `.fbot` format | BD-054 (seven canonical rules, `DesignId`), N14a-01 (rules 2 to 5), N14a-03 (Top, Body), N14a-04 (Joint, Actuation), N14a-05 (End effector, `contact`), BD-055 (fill, Power, Sensing, Payload, Budgets, Levers, Locks, Symmetry), N14a-06 (levers and locks in use), BD-057 (S-expression view, `design.view`), N14a-19, N14a-20, N14a-21 (`design.new`, `design.show`), N14a-26 (reference canonicalizer and golden corpus), N14a-27, N14a-28 | BD-056 (Constructors row), BD-060 (Import row `use`), BD-141 (format round trips) |
| §14.5 | Mass from geometry: the fill rule | BD-055 (fill field, no default, shell-thickness refusal), BD-062 (moments by fill, worked card, shell as difference of solids, PO-SKEL-02), N14a-13 (declared, massless, mass support), N14a-14 (mesh shells by inward offset, non-primitive Regions), N14a-10 (mass spiral applied to the fill error) | BD-051 (mass-support predicate), BD-100 (stage 2) |
| §14.6 | Reusable modules: `.fmodule` | BD-077 (`module.substitute`, four conditions, product table), N14a-15 (pilot), N14a-16 (KILL-SKEL-SUBST), N14a-30 (e2e), N14a-22 (limb port packet), N14a-23 (pilot), N14a-24 (KILL-SKEL-PORT), N14a-02 (port signatures), N14a-12 (limb views that substitution reuses) | BD-060 (format, port table, contract digest, `use` elaboration, peak-only rule), N15b-01 (joins), N15b-02 (`module.list`, `module.show`, `module.check`), BD-385 (packets at run time) |
| §14.7 | Typed edits | N14a-17 (all 22 rows as a tested catalogue), N14a-06 (`declare-lever`, `set-lever`, `lock`, `unlock`), N14a-11 (symmetry effect of an edit), N14a-18 (`migrate`), BD-077 (`swap-module`) | BD-058 (the edit engine and `design.edit`, U41), BD-113 (lineage) |
| §14.8 | Design patches and versioning | BD-059 (`design.patch`: result, stale parent, locks and requirements, idempotency, inertia follows geometry, settled pose, verdict survival), N14a-18 (schema versions, migration, artifact envelope), N14a-29 (e2e) | BD-113 (lineage, `design.diff`), BD-195 (settled pose), BD-292 (revalidation), BD-211 (idempotency binding) |
| §14.9 | The compile pipeline and its memoization | N14a-07 (elaborate), N14a-08 (validate; stage 1), BD-061 (kinematic functor, `RobotModel`, `ModelId`, referee lowering), BD-062 (geometric and inertial functor), N14a-09 (binding functor), N14a-10 (budget functor), N14a-11 (symmetry functor), N14a-12 (lowerings 3 and 4), N14a-21 (`design.show` sections), N14a-27 (e2e) | BD-063 (memo keys, influence records, invalidation set, functor laws, U14b), BD-148 (lowering 2, packed buffers), BD-170 (variant record) |

Seed beads of this unit whose subject lies in other chapters or in U14b's subsections, written in full here:
BD-066 (§14.12, with §14.17 and §18.5), BD-227 (§19.8, Appendix G.3), BD-245 (§18.6, §18.9, §18.16).

Identifiers of §14.14 and §14.15 that concern this unit's mechanisms:

| Identifier | Carried by |
|---|---|
| INV-SKEL-01 | BD-061 |
| INV-SKEL-02 | N14a-07 (elaborate stage); gate measurement in BD-063 (U14b) |
| INV-SKEL-03 | BD-062 |
| INV-SKEL-04 | BD-057, N14a-28 |
| INV-SKEL-05 | BD-062, BD-055 |
| INV-SKEL-06 | BD-077, N14a-17, N14a-30; mechanism in BD-063 (U14b) |
| INV-SKEL-07 | N14a-11 |
| INV-SKEL-08 | N14a-06 |
| INV-SKEL-09 | BD-077, N14a-30 |
| PO-SKEL-01 | BD-057 (BD-054 for idempotence) |
| PO-SKEL-02 | BD-062 |
| PO-SKEL-03 | BD-063 (U14b); this unit's functors read through the tracked view |
| PO-SKEL-04 | N14a-22, N14a-23 |
| OPEN-SKEL-1 (closed, tombstoned) | BD-057, N14a-20 |
| OPEN-SKEL-2 | N14a-10 (Considerations and the `--strict-data` rule), BD-055 |
| OPEN-SKEL-3 | Expected from U14b (stage 8) |
| OPEN-SKEL-4 | BD-062, BD-055 |
| Constructions | C-01: N14a-04, BD-061. C-03: BD-062, N14a-13. C-09: N14a-11. C-12: BD-061, BD-077, N14a-02, N14a-10. C-06, C-07: BD-245 |
| Verbs | `design.new`, `design.show`: N14a-21. `design.view`: BD-057. `design.patch`: BD-059. `design.import --format mjcf`: BD-066. `design.export`: N14a-25. `module.substitute`: BD-077. `scene.hazards`: BD-227 |

## (b) Gates and kill gates

| Gate | Earliest | Measured or decided by (this unit) | Expected from another unit |
|---|---|---|---|
| GATE-SKEL-01 | M1 | BD-066 (MJCF rows), N14a-31 (e2e) | BD-065 and N14b-24 (URDF rows and the combined record, U14b) |
| GATE-SKEL-02 | M1 | — (this unit's functors feed it) | BD-063 (U14b) |
| GATE-SKEL-03 | M1 and M3 | — | BD-109, BD-118, BD-246, BD-247 (U14b and others) |
| GATE-SKEL-04 | M1 (stages 1 to 6), M3 (stages 7, 8) | N14a-08 and N14a-27 (stage 1's share of the seeded defects) | BD-110 (stages 1 to 6), BD-244 (stages 7 and 8) |
| GATE-SKEL-05 | M1 | BD-057, N14a-28 | — |
| GATE-SKEL-06 | M1 | BD-077, N14a-30 | BD-060 and N15b-01 name it for the reversed-sign row |
| GATE-SKEL-07 | M1 and M3 | BD-062 and N14a-27 (PO-SKEL-02 clause) | BD-109, BD-246 (archetype masses within 15%) |
| GATE-SKEL-08 | M1 | BD-059, N14a-18 (migration clause), N14a-29 | — |
| GATE-SKEL-09 | M3 | — | U14b (§14.16) |
| GATE-SKEL-10 | M4 | — | U14b (§14.16) |
| GATE-SKEL-11 | M1 | — | U14b (§14.17); BD-066 relies on the fold |
| KILL-SKEL-SEXPR | M1 | N14a-20 (decision); measured by BD-057, N14a-28 | — |
| KILL-SKEL-SUBST | M1 | N14a-16 (decision); pilot N14a-15; measured by BD-077, N14a-30 | — |
| KILL-SKEL-PORT | M5 | N14a-24 (decision); pilot N14a-23; PO-SKEL-04 in N14a-22 | BD-385 (tree-operator pilot and KILL-TREEOP, U16a) |
| KILL-SKEL-ALIAS | M3 | — | U14b (BD-244) |
| KILL-SKEL-CELL | M4 | — | U14b (§14.16) |
| GATE-MARROW-03 (not of §14) | M3 | N14a-11 supplies four planted-asymmetry rows | BD-102 |

## (c) Counts

- Known answers: `N14-KA` carries all 32 rows of Appendix M.13 (KA-261 to KA-292). 22 of them are also cited
  with their values in this unit's mechanism and test beads: KA-261, 262, 263, 264, 265, 268, 270, 271, 272,
  273, 276, 277, 278, 282, 283, 284, 286, 288, 289, 290, 291, 292. The other ten belong to mechanisms of
  other units and are carried by the registry with their owner named.
- Counterexamples: the registry `N14-CX` is U14b's. This unit's beads cite, with the case and its numbers,
  CX-124, 125, 126, 127, 128, 129, 130, 131, 133, 134 and 173: 9 of the 10 counterexamples of §14 (all but
  CX-132, the stage-8 alias).
- Refusal codes: the registry `N14-REF` is U14b's. This unit's beads state the meaning, trigger, ranked
  repairs and exit code of 15 of the 25 codes of §14: FROB-REF-101, 102, 103, 104, 105, 106, 107, 109, 111,
  112, 113, 115, 121, 125, 126; they use 108, 114, 116 and 122 with their meaning. They also carry, from
  other chapters, FROB-REF-312 (BD-227), 246 and 241 (as used), 803 and 805 (idempotency), 892 (view is not
  an identity). Codes 110, 118, 119, 120, 123 and 124 belong to stages 3 to 8 and are expected from U14b and
  the analysis units.

## (d) Needs from other slices

| Need | Why | Named owner, where known |
|---|---|---|
| The canonical raw-byte reader and explicit-sort writer, and an exact rational type | BD-054, N14a-01 | `frob-core` beads (BD-005; §34.14, GATE-WS-05) |
| Constructor expansion rules, identifier suffixes, reflection parity of `mirror` | N14a-07, BD-061 | BD-056 (U41) |
| The edit engine and `design.edit` | BD-059, N14a-17, N14a-18 | BD-058 (U41) |
| Tracked term view, memo graph, invalidation set, `--no-memo`, random design and edit generator | every functor bead, BD-077, N14a-15, N14a-30 | BD-063 and N14b-01 (U14b) |
| Module format, digest, `use` elaboration, contract checks, joins, module verbs | BD-077, N14a-07, N14a-08 | BD-060, N15b-01, N15b-02 (U15b) |
| Loss-report schema, field-coverage diff, `--strict`, round-trip comparator; fold of fixed joints; reduced models; the combined GATE-SKEL-01 record | BD-066, N14a-25, N14a-31 | N14b-20, N14b-33, N14b-34, N14b-24 (U14b) |
| Oracle pose fixtures of the MJCF reference robots, with pins | BD-066, N14a-31 | BD-067, BD-039 |
| Symmetry algorithm and analysis A-03 | N14a-11 | BD-102, BD-048 |
| Mass-support predicate and exact LDLᵀ; stage 2 staging | N14a-13, BD-062 | BD-051, BD-100 |
| `design.check` runner and stages 2 to 8 | N14a-08, N14a-27, N14a-31 | BD-110, BD-244 |
| Cards, transmissions, joint rows, bus model | N14a-04, N14a-09, N14a-10 | BD-069, BD-074, BD-084, BD-075 |
| Sensor cards and the clock contract | BD-055, N14a-09 | BD-099, BD-078 |
| Turn-state rule and its refusal code (§13.11) | N14a-08, N14a-09 | the Marrow bead of GATE-MARROW-06 (U13) |
| Geometry airlock: mesh checks, distance fields, GLB export | N14a-03, N14a-14, N14a-25 | BD-079 and the airlock beads |
| Size buckets and the variant record | N14a-12 | BD-170, BD-171, BD-148 |
| Lineage graph, head pointers, `design.diff` | BD-059, N14a-18, N14a-21 | BD-113 |
| Idempotency binding, M1 verb registration, MCP tools | BD-059, BD-077, N14a-21, N14a-25 | BD-211, BD-137, BD-138 |
| Templates and mission cells for `design.new` | N14a-21 | BD-109, BD-135 |
| Replay of an old run after a migration (the gate is M1, replay is listed at M2) | N14a-18, N14a-29 | BD-203 |
| Wheel, omni-wheel and belt laws, cone kernels, patch rules, law registry, `contact.laws`, capacity | BD-245, N14a-33 | N18b-01, N18b-02, N18b-05, N18b-06, N18a-15, N18a-27, N18a-28, N18b-20, N18b-21, N18b-28, BD-154 |
| Scene manifest count, interior generators, report verbs with a realism claim | BD-227, N14a-32 | BD-162, BD-163, BD-225, BD-226, BD-302 |
| Tree-operator route and KILL-TREEOP | N14a-22, N14a-24 | BD-385 (U16a); it should depend on the paper pilot N14a-23 |

## (e) Contradictions, gaps and what was done

1. **Default fill.** §11.5 says a body without a wall thickness is solid and refuses an implausibly heavy
   solid when "no fill is declared" (FROB-REF-168); §14.5 says there is no default fill (a missing fill is
   FROB-REF-105) and that a heavy solid is an advisory only. §14 was taken as governing for the design
   format. BD-062 records that FROB-REF-168 cannot arise from a `.fbot` and stays relevant to paths that
   infer solids (importers).
2. **Belt-law known answer.** §14.13 sends the track rejection fixture to a belt-law known answer that §18.16
   does not list (Appendix M notes the gap). BD-245 derives it (scrub moment μ_lat·W·L/4 for stations at
   segment midpoints; μ_lat·W·a for a four-wheel skid-steer base); the Traction unit derived the same form
   independently in N18b-06.
3. **`skid`, `pad` and the compatibility matrix.** §14.4 names the six contact selectors and says an
   incompatible law refuses, without defining `skid`, `pad` or "incompatible". N14a-05 defines the matrix
   and BD-245 the two laws, both labelled DESIGN, kept as data, with the Traction lead as owner of changes.
   The roller angle is signed (a mirror flips it), following the Traction and archetype units.
4. **S-expression grammar.** The chapter gives examples and the bijection requirement only. BD-057 gives a
   schema-directed mapping and a canonical layout, labelled as a proposal confined to the examples.
5. **Turn angles.** Rule 3 asks for the shortest decimal; rule 4's example writes `1/8 turn`. N14a-01 always
   writes a turn as a reduced ratio and records the choice.
6. **Stage ownership of FROB-REF-107.** The stage table gives the empty range to stage 3, while limits are a
   physics field with no default and so must be present when the file is read. N14a-04: a missing `limits`
   is refused when the document is typed (stage 1) with the same code; the lo ≥ hi comparison is staged at
   stage 3.
7. **The artifact envelope.** §14.8 cites "the common artifact envelope of §34" with six contents; §34 has no
   passage under that name. N14a-18 specifies an envelope with exactly those contents and asks for
   reconciliation with the format registry.
8. **Verbs without a seed bead.** `design.new` and `design.show` (Appendix J.3) had none; N14a-21 writes
   them, and the archetype bead of U14b already lists `design.new` as a need.
9. **Fixed base.** The format has no field for a robot bolted to the world; BD-061 gives the root a floating
   base and records that a fixed base needs a schema decision.
10. **Seed beads outside the chapter.** BD-227 (hazard lists, §19.8) and BD-245 (contact laws, §18) were
    assigned here by title matching. Both are written in full. Parents: WP-23 for BD-227; WP-38 for BD-245
    (the M3 archetypes are its consumer; no work package holds `frob-contact` at M3 except WP-53).
11. **BD-245 against the Traction unit's beads.** Wheels, omni wheels, the belt pilot and the Frontier belt
    are already written by U18b and depend on BD-245. BD-245 is therefore scoped to the selector bindings on
    all tiers, the `skid` and `pad` laws, the M3 fallback form of `belt` (road wheels with anisotropic
    friction) and the scrub-work answer. No belt pilot was written here. The first archetypes need the wheel
    and roller laws on the referee at M1, before BD-245's milestone; those are U18b's.
12. **Overlaps to merge at integration.** N14a-02 (port signatures and port instances) adjoins N15b-01
    (junction checks); N14a-10 (per-subtree summaries) adjoins A-02 mass roll-up (BD-100); BD-066 and
    N14a-25 use N14b-20's report machinery. No bead was written here for `use` elaboration, module contract
    checks, functor laws, PO-SKEL-03 or the 64-edit sequence of C-12, because BD-060 and BD-063 carry them.
13. **KILL-SKEL-SUBST, second trigger.** "Saves less than 20% of compile time on the archetypes" does not say
    whether every archetype or their median. N14a-16 decides on the median and records every value.
14. **GATE-SKEL-08 at M1 and replay.** "Old runs replayable" needs run replay, which the seed table places at
    M2. N14a-29 logs that step as partial until replay exists and does not record the gate as passed on a
    partial run.
15. **Other DESIGN choices made where the text is silent**, each labelled in its bead: the spelling of
    written-out defaults (`"collision":"region"`, `"members":"*"`); `pitch` as the advance per turn; storing
    a design that has a canonical form but fails validation (`design.new`, exit 2); MJCF `armature` as a card
    override; enclosing sets for the mass-support test beyond the sphere and ellipsoid; the class vocabulary
    of invalidation scopes; `ModelId` as a hash of section digests that excludes names.
