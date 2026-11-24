# Polishing round 4, slice H-cx-1

14 beads.

| Key | Milestone | Type | Parent | Changed last round | Title | File |
|---|---|---|---|---|---|---|
| N00-CX | M0 | task | WP-34 | yes | frob-conformance: counterexample catalogue entries CX-001 to CX-015 seeded as negative controls | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U00/N00-CX.md` |
| N09-CX | M0 | task | WP-14 | yes | registry: the three doctrine counterexamples (CX-016 to CX-018) seeded into the negative-control registry | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U10a/N09-CX.md` |
| N10-CX | M1 | task | WP-14 | yes | registry: the counterexamples of the constructions (CX-019 to CX-097) seeded into the negative-control registry | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U10c/N10-CX.md` |
| N11-CX | M1 | task | WP-14 | yes | Counterexample registry for the conventions: CX-098 to CX-105 as negative controls with their gates | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U13/N11-CX.md` |
| N12-CX | M1 | task | WP-36 | no (here for a request) | Lemma counterexample registry: CX-106 to CX-115, the shortcuts that certificates must not take | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U12/N12-CX.md` |
| N13-CX | M1 | task | WP-14 | no (here for a request) | Counterexample registry for Marrow: CX-116 to CX-124 and the turn-count case as negative controls | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U13/N13-CX.md` |
| N14-CX | M1 | task | WP-14 | yes | Counterexample registry of Skeleton (§14): CX-125 to CX-134 and three late cases, seeding negative controls | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U14b/N14-CX.md` |
| N16-CX | M1 | task | WP-10 | yes | frob-conformance: Kinesis counterexample registry (CX-144 to CX-152 and the late-subsection cases) | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U16c/N16-CX.md` |
| N18-CX | M1 | task | WP-14 | yes | frob-conformance: Traction counterexample registry CX-167 to CX-183, seeding the negative-control registry | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U18b/N18-CX.md` |
| N19-CX | M2 | task | WP-17 | yes | registry: Habitat counterexamples CX-184 to CX-199 as negative controls, plus four unnumbered late cases | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U19c/N19-CX.md` |
| N20-CX | M1 | task | WP-14 | yes | registry: Sensorium counterexamples CX-200 to CX-215 and the late-subsection shortcuts, as negative controls | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U20b/N20-CX.md` |
| N21-CX | M2 | task | WP-46 | yes | Spark counterexample registry: CX-216 to CX-233 with breaking cases, protected rules and executable tests | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U21c/N21-CX.md` |
| N22-CX | M1 | task | WP-14 | yes | registry: Reflex counterexamples CX-234 to CX-245 and nine appended entries, as catalogue entries for controls | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U22c/N22-CX.md` |
| N23-CX | M2 | task | WP-48 | yes | Cortex counterexample registry: CX-246 to CX-258 and five late cases, seeding the negative controls | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U23b/N23-CX.md` |

## Addressed to your beads

- `N09-CX`: request from round 3, slice F-U25d-1: CX-018's label and rerun field name 'BD-043 (wheel loop)'. The wheel-loop pilot PILOT-PT-17 is re-run by N38a-11, and BD-043 excludes it. Name N38a-11.
- `N12-CX`: request from round 3, slice F-U12-1: CX-106's control is realized by N12-02 and N12-03; CX-111 is realized by N12p2-02, not N12-18.
- `N13-CX`: request from round 3, slice F-U13: CX-117 is recorded as `mutation`, plant `M3`, observation (0, 0) instead of (-3, -3); that observation is CX-117's own shortcut, while M3 gives (+3, +3). Give CX-117 its own plant (the lower-rows shortcut that BD-047's planar_2r_lower_rows_shortcut_is_wrong builds) and drop 'CX-117 names M3' from the acceptance line and from cx_entries_have_kind_and_plant; N13-18 now seeds M3 from KA-243
- `N18-CX`: request from round 3, slice F-U18b-2: The late entry trac-arc names its plant `N18b-45:trac-arc`, but N18b-43 records GATE-TRAC-12's containment row first and its receipt must carry the chord-only control caught in the same run; N18b-45 depends on N18b-43, so a plant built by N18b-45 is a wait on a later bead (convention 7). N18b-43 now builds the branch (`chord-only-swept-bound`, D = 0 in swept::bound): rename the plant `N18b-43:trac-arc`; N18b-45 and N18b-44 run it, as the row's note says of N18b-44.
- `N18-CX`: request from round 3, slice F-U38a: CX-176: enter the `rerun` pair from PILOT-PT-19's status and receipt when the row is written (N38a-11, M0, has run by then), and name the record in the label so that the catalogue's rerun-disagrees-with-pilot check holds it.
- `N19-CX`: request from round 3, slice F-U19b-2: CX-194's plant carrier is N19b-25, a standalone pilot outside the workspace (tools/pilots/clearance/) with no real component for a gauntlet-mutations feature; change the plant to `N19b-29:cx-194` (the refinement loop returning after candidate selection), which N19b-29 now registers and catches with doorway_60mm.
- `N19-CX`: request from round 3, slice F-U25d-1: CX-195 is written after BD-043 (M0) closes. Write it with rerun = true and rerun_receipt = the receipt of BD-043's PILOT-PT-16 sill result (logged in BD-043's review log) when pilots::rerun_status(PILOT-PT-16) is reproduced. Keep false only while it is not-run or differs.
- `N19-CX`: mechanical check of the conventions: [C2-file] keeps registry rows in a private or per-chapter file (convention 2), e.g. `(`registry/counterexamples/habitat.toml`) with fields of its own (`mutant`, `caught_by`, `gate`, `bead`): a`
