# Polishing round 3, slice H-ka-1

14 beads.

| Key | Milestone | Type | Parent | Changed last round | Title | File |
|---|---|---|---|---|---|---|
| N00-KA | M0 | task | WP-34 | yes | frob-conformance: known-answer registry rows KA-001 to KA-032 (document control through the claim product) | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U00/N00-KA.md` |
| N09-KA | M0 | task | WP-14 | yes | frob-conformance: known-answer registry of the doctrine chapter, KA-033 to KA-036 | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U10a/N09-KA.md` |
| N10-KA1 | M1 | task | WP-14 | yes | frob-conformance: known-answer registry of the constructions, part 1 — C-01 to C-15 (KA-037 to KA-091) | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U10a/N10-KA1.md` |
| N10-KA2 | M1 | task | WP-14 | yes | frob-conformance: known-answer registry of the constructions, part 2 — C-16 to C-27 (KA-092 to KA-141) | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U10a/N10-KA2.md` |
| N10-KA3 | M1 | task | WP-14 | yes | frob-conformance: known-answer registry of the constructions, part 3 — C-28 to C-40 (KA-142 to KA-190) | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U10a/N10-KA3.md` |
| N11-KA | M1 | task | WP-14 | yes | Known-answer registry for the conventions: KA-191 to KA-206 with truth, tolerance, tier, control and test | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U13/N11-KA.md` |
| N13-KA | M1 | task | WP-14 | yes | Known-answer registry for Marrow: KA-228 to KA-260 and the turn-count answers, with tolerance, tier, control | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U13/N13-KA.md` |
| N14-KA | M1 | task | WP-07 | yes | Known-answer registry of Skeleton (§14): KA-261 to KA-292 with truths, tolerances, tiers and negative controls | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U14a/N14-KA.md` |
| N16-KA | M1 | task | WP-14 | yes | Known-answer registry of Kinesis: KA-338 to KA-364, with truths, tolerances, tiers, controls and owning beads | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U16a/N16-KA.md` |
| N17-KA | M1 | task | WP-14 | yes | frob-conformance: known-answer registry for Phylum (KA-365 to KA-386) with derivations and controls | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U17a/N17-KA.md` |
| N18-KA | M1 | task | WP-14 | yes | registry: Traction known answers KA-387 to KA-425 with truth, tolerance, tier and negative control | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U18a/N18-KA.md` |
| N19-KA | M2 | task | WP-17 | yes | registry: Habitat known answers KA-426 to KA-464 with truths, tolerances, tiers and negative controls | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U19a/N19-KA.md` |
| N20-KA | M1 | task | WP-14 | yes | registry: Sensorium known answers KA-465 to KA-524 with truth, tolerance, tier, negative control and owner | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U20a/N20-KA.md` |
| N21-KA | M0 | task | WP-14 | yes | registry: Spark known answers KA-525 to KA-552 with truth, tolerance, tier, negative control and carrier | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U21a/N21-KA.md` |

## Addressed to your beads

- `N14-KA`: request from round 2, slice S26: KA-270, row `snake-crawler` ('C₂ under head-to-tail reversal'): N14bp1-01's hand derivation finds the sagittal mirror in every left-right symmetric chain (as for the waist yaw of `humanoid`), so the computed group is C₂ × C₂ if the model is palindromic and C₂ by the sagittal mirror if actuator mass sits on one side of each joint; the row is under an OPEN there for the Skeleton lead (amend the cell, or stagger the wheel pairs); mark the row accordingly
- `N16-KA`: request from round 2, slice S12: rows go to crates/frob-conformance/registry/known_answers/kinesis.toml, a chapter file that N38a-12's one-file rule (now also matching the known_answers/ directory) refuses with ka-second-file: append the rows to crates/frob-conformance/registry/known_answers.toml in N38a-12's schema, as N18-KA, N21-KA, N22-KA and N28-KA now do
- `N17-KA`: request from round 2, slice S35: KA-371 states only the rounded reading (Ω = 3.5: 14.8 ms, 1.35 ms, crossover 360). With exact arithmetic Ω = 3.4908 gives 14.75 ms, 1.344 ms, 0.673 ms and a crossover of 361.3; state both readings, as N17a-04, BD-171 and GB-17 now do.
- `N19-KA`: request from round 2, slice S70: Its accept on the reflected room should use `frob scene accept --source <room> --preview --key hka-2`: the room does not compile (302), so there is no pack to accept.
- `N21-KA`: request from round 2, slice S52: KA-541's carrier: BD-179 measures all clauses but the energy residual; N21bp2-01 measures that clause and records GATE-SPARK-05. The same correction applies wherever BD-179 is named as the recorder of GATE-SPARK-05 (N21c-32's gate table, EPIC-M2, WP-22).
