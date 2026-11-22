# Brief for drafting unit UF

Read `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/BEADS_CHARTER.md` first. This brief says what you own.

- Plan file: `/Users/jemanuel/projects/franken_robotics/COMPREHENSIVE_PLAN_FOR_FRANKEN_ROBOTICS.md` (37158 lines). Read by line range with the Read tool (`offset`, `limit`).
- Your output directory: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/` (create it; one `<key>.md` per bead, plus `coverage.md`).
- Key prefix for your new beads: `NF-` followed by a two-digit number (`NF-01`, `NF-02`, …).
- All seed beads, with the unit that writes each: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/seed_index.md`. Work packages: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/wp_index.md`.
- Other keys you may depend on without writing them: `EPIC-PROGRAM`, `EPIC-M0`…`EPIC-M8`, `WP-01`…`WP-80`, `STD-E2E`, `STD-TEST`,
  `SPEC-C-01`…`SPEC-C-40` (the specification bead of construction C-nn), `CARD-T-01`…`CARD-T-35` (theorem card T-nn), and any `BD-nnn`.

## What you own

Appendix F (upstream work items).

## Specific instructions for this unit

Plan ranges: Appendix F lines 32452–32571; §33 lines 22571–23533 (read §33.8 on upstream work and drift, and the subsections on toolchain facts, the toolchain train and the admission gates: find them with `grep -n '^### 33\.'`). Read `/Users/jemanuel/projects/franken_robotics/planning/fable/blend/REVISION_2.md` first: every sibling is the owner's own project, so upstream work is this program's work, done in the sibling's repository under that repository's own rules.

1. F.1, the register: one bead per upstream work item, with the item's identifier as its key (for example `FD-FMCP-01`), type `task`, labels `upstream`, `suite`, the sibling's name and the milestone. Body: which sibling repository; what changes there, and that the change is additive; why this program needs it; the interim posture in this repository; the switch-over rule and the gate it re-runs; acceptance in the sibling (its own tests and gates, under its own `AGENTS.md`); acceptance here; coordination (a mirrored bead in the sibling's tracker, file reservations, other agents working in the same repository); the fallback if the change cannot land. FD-FMCP-01 (bring fastmcp_rust onto toolchain T) and FD-ASUP-01 (one asupersync version across the graph) are priority 0 at M0.
2. F.2, the reported foundation defects: one bead per defect report with key `FDEF-nn`: the defect, the evidence with file and line, the minimal reproduction, the proposed fix, and the guard this repository keeps meanwhile.
3. F.3 (what upstream work does not change) goes into the Considerations of each bead.
4. If no seed bead carries them (check `seed_index.md`, in particular BD-008, BD-023 and BD-037): the toolchain train (moving T in this repository and in every consumed sibling in one coordinated change) and the compatibility-train procedure.

## Seed beads you own (0): write one bead file for each, keeping key, milestone and dependencies


