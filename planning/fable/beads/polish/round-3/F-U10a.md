# Polishing round 3, slice F-U10a

18 beads.

| Key | Milestone | Type | Parent | Changed last round | Title | File |
|---|---|---|---|---|---|---|
| N10a-01 | M0 | task | WP-14 | yes | frob-conformance: the construction register C-01 to C-40, the contract template, the D-2 obligations check | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U10a/N10a-01.md` |
| N10a-05 | M1 | task | WP-14 | yes | frob-conformance: doctrine D-4, D-9 checks — statements become relations, counterexamples become controls | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U10a/N10a-05.md` |
| N10a-06 | M1 | task | WP-14 | yes | frob-conformance: doctrine D-5, D-12, D-19 checks — weakest-link class, quantifiers, named assumptions | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U10a/N10a-06.md` |
| N10a-09 | M3 | task | WP-55 | yes | frob-conformance: doctrine D-16 check — exploration issues no bound; bounds come from certification lanes | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U10a/N10a-09.md` |
| N10a-14 | M4 | task | WP-72 | yes | pilot C-06: the hard-contact batch law and factor reuse on the contact known answers and one 21-contact stack | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U10a/N10a-14.md` |
| N10a-15 | M1 | task | WP-11 | yes | pilot C-07: maneuverability index and bracket primitives on four reference bases in three doorway scenes | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U10a/N10a-15.md` |
| N10a-16 | M3 | task | WP-11 | yes | pilot C-08: holonomy on a three-link body and a wheeled snake against direct collocation | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U10a/N10a-16.md` |
| N10a-17 | M4 | task | WP-64 | yes | pilot C-10: trot and bound orbits of one quadruped continued in a design parameter, ranked against rollouts | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U10a/N10a-17.md` |
| N10a-20 | M1 | task | WP-12 | yes | pilot C-13: baseline controller synthesis across the reference robots under the three-outcome report | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U10a/N10a-20.md` |
| N10a-22 | M1 | task | WP-14 | yes | e2e: spine audit for constructions C-01 to C-15: contracts, law suites, controls, registry rows, cards, verbs | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U10a/N10a-22.md` |
| SPEC-C-01 | M1 | task | WP-03 | no (here for a request) | frob-lie: C-01 specified — configuration group, joint subgroups, Hervé classes; contract and law tests | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U10a/SPEC-C-01.md` |
| SPEC-C-04 | M1 | task | WP-10 | yes | frob-dyn: C-04 specified — Euler–Poincaré dynamics on a tree, momentum map, Jacobians; contract, law tests | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U10a/SPEC-C-04.md` |
| SPEC-C-10 | M4 | task | WP-64 | yes | frob-gait: C-10 specified — gaits as (H, K) subgroup pairs, max-plus footfall timing; contract, law tests | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U10a/SPEC-C-10.md` |
| SPEC-C-11 | M1 | task | WP-11 | yes | frob-analyze: C-11 specified — kinematics as algebraic geometry, certified inverse kinematics, singular loci | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U10a/SPEC-C-11.md` |
| SPEC-C-12 | M1 | task | WP-07 | yes | frob-design: C-12 specified — typed ports, power-conserving wiring, five functors; contract and law tests | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U10a/SPEC-C-12.md` |
| SPEC-C-13 | M1 | task | WP-12 | yes | frob-control: C-13 specified — estimation and control on groups, invariant filter, scoring gauge; law tests | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U10a/SPEC-C-13.md` |
| SPEC-C-14 | M0 | task | WP-05 | yes | frob-kir: C-14 specified — derived straight-line kernels, dimension typing, rewrite classes E and R; law tests | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U10a/SPEC-C-14.md` |
| SPEC-C-15 | M1 | task | WP-41 | yes | frob-arena: C-15 specified — evidence as a test martingale, two study types, level spending; law tests | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U10a/SPEC-C-15.md` |

## Addressed to your beads

- `N10a-01`: request from round 2, slice S75: `--test g0_doctrine` (the d2_* obligation check) is not one of the six Gauntlet tier targets of KNOWN_DRIFTS 1; 18 beads call `cargo test -p frob-conformance --test g0_doctrine -- d2` (SPEC-C-31 among them). Move d2_* into g0_laws (or state the exception in BD-025) so the callers can be aligned.
- `N10a-01`: request from round 2, slice S81: Optional: the G0 test target g0_doctrine is not one of the six Gauntlet targets of drift 1 (g0_laws, ...); SPEC-C-40 still cites it in an acceptance line (left as is). Either fold the d2_* family into g0_laws or record g0_doctrine as an allowed extra target in KNOWN_DRIFTS.
- `N10a-22`: request from round 2, slice S76: Its check 3d runs a known-answer row as `cargo test … --test <tier target> -- ka:<id>`; N38a-02 defines the row filter as the bare identifier (`KA-501`). Same in N10c-10's step d. N10b-19 now uses the bare form.
- `SPEC-C-01`: request from round 2, slice S23: its e2e runs 'frob --json analyze mobility --design fixtures/linkages/sarrus.fbot': the analyze verbs take the design as their positional argument (N28a-05: 'frob analyze <name> <design> --json') and have no --design flag.
- `SPEC-C-01`: request from round 3, slice H-owners-1: BD-025/ADR-GAUNT-01: tier targets are g0_laws, g1_known_answers, g2_parity, g3_metamorphic, g4_chaos, g5_determinism; replace `--test g0`..`--test g5` with the long name (the lint refuses the short form).
