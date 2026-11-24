# Polishing round 3, slice H-cx-2

11 beads.

| Key | Milestone | Type | Parent | Changed last round | Title | File |
|---|---|---|---|---|---|---|
| N28-CX | M1 | task | WP-11 | yes | Atelier counterexample registry: CX-302 to CX-318 as negative-control seeds, plus two late entries | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U28c/N28-CX.md` |
| N31-CX | M5 | task | WP-70 | yes | Counterexample registry of Foundry (§31): CX-336 to CX-344 and six late cases, with controls and gates | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U31b/N31-CX.md` |
| N32-CX | M1 | task | WP-06 | yes | Tether counterexample registry: CX-345 to CX-351 with their planted mutants, and the unnumbered cases | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U32b/N32-CX.md` |
| N33-CX | M0 | task | WP-14 | yes | registry: counterexamples of Suite consumption (CX-352 to CX-357) and the late foundation-fact shortcuts | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U33c/N33-CX.md` |
| N34-CX | M0 | task | WP-01 | yes | Workspace counterexample registry: CX-358 to CX-363 (identity, auditor and layer-order shortcuts) | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U34/N34-CX.md` |
| N35-CX | M1 | task | WP-14 | yes | frob-conformance: counterexample registry rows CX-364 to CX-373 (numerics and determinism) | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U35/N35-CX.md` |
| N36-CX | M0 | task | WP-34 | yes | registry: performance counterexamples CX-374 to CX-383 with breaking cases, rules and executable tests | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U36/N36-CX.md` |
| N38-CX | M0 | task | WP-14 | yes | frob-conformance: counterexample entries stated by the Gauntlet chapter: sixteen shortcuts with cases | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U38b/N38-CX.md` |
| N39-CX | M3 | task | WP-60 | yes | frob-conformance: the counterexample and negative-control registry of the ten-fold harness | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U39b/N39-CX.md` |
| N40-CX | M0 | task | WP-01 | yes | registry: planted defects of the gate register and milestone layer, each a control the tooling must catch | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U40/N40-CX.md` |
| N41-CX | M0 | task | EPIC-M0 | yes | registry: counterexamples and planted defects of the delivery registers, each a control the tooling must catch | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U41/N41-CX.md` |

## Addressed to your beads

- `N38-CX`: request from round 2, slice S14: CX-386: KA-837's n = 0.7 control prints 9.17 cm; computed from the stated constants (unrounded p = 5.01648 kPa) it is 0.091646 m = 9.16 cm; 9.17 follows from p rounded to 5.017 kPa before the power 1/n. Align to 9.16 (0.091646, 0.190127, 0.117076 m), as N38b-43 and N38b-46 now state.
- `N40-CX`: request from round 2, slice S01: three states are now tested in N40-05 and N40-35 with no registered control: an attached check that exits non-zero treated as a pass; an Owner question with no calendar row at or before its deadline; a not-run gate behind a defaulted question reported with no question named. Register them (next RC numbers).
- `N40-CX`: request from round 2, slice S46: Register a control for N40-33's new rule: a kill gate recorded `not-fired` at the M2 exit under the no-GPU variant while a part its record reads is `not run (variant: no-GPU)` (caught by `kill_decisions_under_variant`).
