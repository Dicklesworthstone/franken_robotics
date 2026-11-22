# Brief for drafting unit UEG

Read `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/BEADS_CHARTER.md` first. This brief says what you own.

- Plan file: `/Users/jemanuel/projects/franken_robotics/COMPREHENSIVE_PLAN_FOR_FRANKEN_ROBOTICS.md` (37158 lines). Read by line range with the Read tool (`offset`, `limit`).
- Your output directory: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/` (create it; one `<key>.md` per bead, plus `coverage.md`).
- Key prefix for your new beads: `NEG-` followed by a two-digit number (`NEG-01`, `NEG-02`, …).
- All seed beads, with the unit that writes each: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/seed_index.md`. Work packages: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/wp_index.md`.
- Other keys you may depend on without writing them: `EPIC-PROGRAM`, `EPIC-M0`…`EPIC-M8`, `WP-01`…`WP-80`, `STD-E2E`, `STD-TEST`,
  `SPEC-C-01`…`SPEC-C-40` (the specification bead of construction C-nn), `CARD-T-01`…`CARD-T-35` (theorem card T-nn), and any `BD-nnn`.

## What you own

Appendix E (worked cards) and Appendix G (reference robots, scene assets, licences, hazards).

## Specific instructions for this unit

Plan ranges: Appendix E lines 31669–32451; Appendix G lines 32572–32736.

1. Appendix E, worked cards GB-01 to GB-41: one bead per card with key `GB-nn` (type `task`, labels `test`, `card` and the subsystem). A worked card is a decision or a computation with substituted numbers. The bead makes it a reproducible golden: `cards/GB-nn.toml` holding the inputs, the computation in the owning crate, the expected outputs with tolerances, and a test that recomputes the card. The body carries the card completely: the equation, the substituted values, the intuition, the assumptions, and what would change the decision. Each discrepancy recorded in E.7 gets a bead that resolves it.
2. Appendix G.1, reference robots: one bead per robot with key `ROBOT-<short name>`: source, licence and redistribution terms, degrees of freedom, mass, actuators, what it is used for (which gates and fixtures), the import path, validation (physically consistent mass properties, joint limits), the `assets/robots/<name>/` layout and the licence check.
3. Appendix G.2, scene assets: one bead per asset family, in the same form.
4. Appendix G.3, hazard lists: registry bead(s), and the check that every hazard is covered by a requirement, a shield or a refusal.
5. Appendix G.4, the eighteen archetypes: one bead per archetype design with key `ZOO-<name>` (`zoo/<name>.fbot`): morphology, symmetry group, degrees of freedom, what it exercises, what preflight must report, its known answers. They depend on the design-language seed beads of §14 (see `seed_index.md`).

## Seed beads you own (0): write one bead file for each, keeping key, milestone and dependencies


