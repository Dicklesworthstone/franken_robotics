# Polishing round 4, slice F-U10a

14 beads.

| Key | Milestone | Type | Parent | Changed last round | Title | File |
|---|---|---|---|---|---|---|
| N10a-01 | M0 | task | WP-14 | yes | frob-conformance: the construction register C-01 to C-40, the contract template, the D-2 obligations check | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U10a/N10a-01.md` |
| N10a-02 | M0 | task | WP-14 | no (here for a request) | frob-conformance: doctrine D-6, D-10, D-17 checks — moonshot cap, pilot and ceiling, generic comparator | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U10a/N10a-02.md` |
| N10a-05 | M1 | task | WP-14 | yes | frob-conformance: doctrine D-4, D-9 checks — statements become relations, counterexamples become controls | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U10a/N10a-05.md` |
| N10a-06 | M1 | task | WP-14 | yes | frob-conformance: doctrine D-5, D-12, D-19 checks — weakest-link class, quantifiers, named assumptions | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U10a/N10a-06.md` |
| N10a-09 | M3 | task | WP-55 | yes | frob-conformance: doctrine D-16 check — exploration issues no bound; bounds come from certification lanes | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U10a/N10a-09.md` |
| N10a-14 | M4 | task | WP-72 | yes | pilot C-06: the hard-contact batch law and factor reuse on the contact known answers and one 21-contact stack | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U10a/N10a-14.md` |
| N10a-20 | M1 | task | WP-12 | yes | pilot C-13: baseline controller synthesis across the reference robots under the three-outcome report | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U10a/N10a-20.md` |
| N10a-22 | M1 | task | WP-14 | yes | e2e: spine audit for constructions C-01 to C-15: contracts, law suites, controls, registry rows, cards, verbs | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U10a/N10a-22.md` |
| SPEC-C-01 | M1 | task | WP-03 | yes | frob-lie: C-01 specified — configuration group, joint subgroups, Hervé classes; contract and law tests | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U10a/SPEC-C-01.md` |
| SPEC-C-04 | M1 | task | WP-10 | no (here for a request) | frob-dyn: C-04 specified — Euler–Poincaré dynamics on a tree, momentum map, Jacobians; contract, law tests | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U10a/SPEC-C-04.md` |
| SPEC-C-11 | M1 | task | WP-11 | yes | frob-analyze: C-11 specified — kinematics as algebraic geometry, certified inverse kinematics, singular loci | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U10a/SPEC-C-11.md` |
| SPEC-C-12 | M1 | task | WP-07 | yes | frob-design: C-12 specified — typed ports, power-conserving wiring, five functors; contract and law tests | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U10a/SPEC-C-12.md` |
| SPEC-C-13 | M1 | task | WP-12 | yes | frob-control: C-13 specified — estimation and control on groups, invariant filter, scoring gauge; law tests | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U10a/SPEC-C-13.md` |
| SPEC-C-15 | M1 | task | WP-41 | yes | frob-arena: C-15 specified — evidence as a test martingale, two study types, level spending; law tests | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U10a/SPEC-C-15.md` |

## Addressed to your beads

- `N10a-02`: request from round 3, slice F-U10a: The register's `pilot` field is now a list (N10a-01), because C-06 has two records (PILOT-C-06-law, PILOT-C-06-reuse; N10a-14). The d10 check that reads it should accept a list, require it non-empty when the tier contains F or M, and require every entry to exist in pilots.toml.
- `N10a-05`: request from round 3, slice F-U38a: `generated_from` is now in the relation schema (form C-nn.Sk; the construction must be in the record's source), so check C can run; MR-38's source names C-14.
- `N10a-05`: mechanical check of the conventions: [C13-pending] 1 place(s) narrate a pending request instead of stating the owner's form (convention 13), e.g. "…reads "Kernel rewrites,    INV-SPARK-09"; adding C-14 to it is requested of N38a-18); for any other statement the link is a record whose    `so…"
- `SPEC-C-01`: request from round 3, slice F-U13: spec_c01.sh step (3) runs `cargo test -p frob-conformance --test g1 -- KA-037 KA-038 KA-039`; `g1` is not a target (KNOWN_DRIFTS 1) and KA-039 is a G0 row: use `--test g1_known_answers -- KA-037 KA-038` and `--test g0_laws -- KA-039`
- `SPEC-C-01`: request from round 3, slice H-ka-1: Replace `cargo test -p frob-conformance --test g1|g0|g3 -- KA-…` with the tier targets g1_known_answers, g0_laws, g3_metamorphic (KNOWN_DRIFTS 1).
- `SPEC-C-04`: request from round 3, slice F-U24a-2: Replace telemetry.query's `--ticks` with `--from`/`--to` (N24a-36's one spelling).
