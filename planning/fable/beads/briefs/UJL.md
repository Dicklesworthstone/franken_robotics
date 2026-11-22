# Brief for drafting unit UJL

Read `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/BEADS_CHARTER.md` first. This brief says what you own.

- Plan file: `/Users/jemanuel/projects/franken_robotics/COMPREHENSIVE_PLAN_FOR_FRANKEN_ROBOTICS.md` (37158 lines). Read by line range with the Read tool (`offset`, `limit`).
- Your output directory: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UJL/` (create it; one `<key>.md` per bead, plus `coverage.md`).
- Key prefix for your new beads: `NJL-` followed by a two-digit number (`NJL-01`, `NJL-02`, …).
- All seed beads, with the unit that writes each: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/seed_index.md`. Work packages: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/wp_index.md`.
- Other keys you may depend on without writing them: `EPIC-PROGRAM`, `EPIC-M0`…`EPIC-M8`, `WP-01`…`WP-80`, `STD-E2E`, `STD-TEST`,
  `SPEC-C-01`…`SPEC-C-40` (the specification bead of construction C-nn), `CARD-T-01`…`CARD-T-35` (theorem card T-nn), and any `BD-nnn`.

## What you own

Appendix J (verb registry) and Appendix L (worked artifacts).

## Specific instructions for this unit

Plan ranges: Appendix J lines 33809–34160; Appendix L lines 34239–35080.

1. Appendix J, the verb registry (about 170 verbs, by cluster in J.2): chapter units implement the verbs their chapters own. You write one registry bead per cluster with key `VERBS-<cluster>`, carrying every verb row completely (name, purpose, inputs, outputs, schema, tier, evidence class, refusals, owner crate) and the checks that go with it: the CLI commands and MCP tools generated from the registry match the list; every verb has an end-to-end script; the `frob --json <verb> --help` schema snapshot is stable. Also beads for J.3 (verbs the chapters define that the inventory lacks), J.4 (one name per verb: the alias lint) and J.5 (task clusters of at most seven tools: the MCP resource that serves them, and its test). In `coverage.md`, a table from each verb to the unit you expect to implement it.
2. Appendix L, worked artifacts L.1 to L.9: one bead per artifact with key `ART-L1` … `ART-L9`: the artifact carried completely as a golden fixture under `fixtures/golden/`, its schema validation, its round-trip test, and the end-to-end script that produces it from the CLI. L.9 (an agent session over MCP) becomes the scripted MCP session run against `frob-mcp` in process and through `frobd --mcp`.

## Seed beads you own (0): write one bead file for each, keeping key, milestone and dependencies


