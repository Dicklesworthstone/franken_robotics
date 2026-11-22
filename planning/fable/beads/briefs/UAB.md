# Brief for drafting unit UAB

Read `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/BEADS_CHARTER.md` first. This brief says what you own.

- Plan file: `/Users/jemanuel/projects/franken_robotics/COMPREHENSIVE_PLAN_FOR_FRANKEN_ROBOTICS.md` (37158 lines). Read by line range with the Read tool (`offset`, `limit`).
- Your output directory: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UAB/` (create it; one `<key>.md` per bead, plus `coverage.md`).
- Key prefix for your new beads: `NAB-` followed by a two-digit number (`NAB-01`, `NAB-02`, …).
- All seed beads, with the unit that writes each: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/seed_index.md`. Work packages: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/wp_index.md`.
- Other keys you may depend on without writing them: `EPIC-PROGRAM`, `EPIC-M0`…`EPIC-M8`, `WP-01`…`WP-80`, `STD-E2E`, `STD-TEST`,
  `SPEC-C-01`…`SPEC-C-40` (the specification bead of construction C-nn), `CARD-T-01`…`CARD-T-35` (theorem card T-nn), and any `BD-nnn`.

## What you own

Appendix A (glossary), B (idea passes), C (EV matrix and killed-idea register), H (source register).

## Specific instructions for this unit

Plan ranges: Appendix A lines 29683–30029; Appendix B lines 30030–30466; Appendix C lines 30467–31478; Appendix H lines 32737–33092.

1. Appendix A, the glossary: bead(s) for `docs/GLOSSARY.md` generated from `registries/glossary.toml`, with a lint that every term of art used in a `CONTRACT.md` is defined. The bead bodies carry the complete glossary, every term with its definition; split by letter range into several beads if needed.
2. Appendix H, the source register: bead(s) for `registries/sources.toml` (source, version or date, licence, what it supports) and the citation lint; the bodies carry every row.
3. Appendix C, the EV matrix and the killed-idea register: decision-record beads (labels `registry`, `decision-record`) carrying every killed idea with the reason it was killed and the condition under which it would be revived (deliverable `docs/decisions/killed-ideas.md` and a lint that a revived idea cites its record); the EV matrix as `registries/ev_matrix.toml` with its scores, and a bead for re-scoring it at each milestone exit.
4. Appendix B, the idea passes: for every idea marked adopted, find the seed bead or the unit that should carry it (`seed_index.md`, and the chapter the idea names). List in `coverage.md` every adopted idea for which you could not find a carrier, with its plan section, and write the bead yourself where the idea is a self-contained piece of work with no home.

## Seed beads you own (0): write one bead file for each, keeping key, milestone and dependencies


