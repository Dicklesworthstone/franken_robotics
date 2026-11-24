# Polishing round 3, slice H-ka-3

6 beads.

| Key | Milestone | Type | Parent | Changed last round | Title | File |
|---|---|---|---|---|---|---|
| N36-KA | M0 | task | WP-34 | yes | registry: performance known answers KA-806 to KA-826 with truth, tolerance, tier, control and carrier | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U36/N36-KA.md` |
| N37-KA | M1 | task | WP-58 | no (here for a request) | frob-conformance: known-answer registry of the safety chapter: KA-827 to KA-835, with values, tiers and controls | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U37/N37-KA.md` |
| N38-KA | M0 | task | WP-14 | yes | frob-conformance: known-answer rows adopted by the Gauntlet: five corrected fixtures, nine late fixtures | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U38b/N38-KA.md` |
| N39-KA | M3 | task | WP-60 | yes | frob-conformance: the known-answer registry of the ten-fold harness (cards, guard, estimands, recovery) | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U39b/N39-KA.md` |
| N40-KA | M0 | task | WP-01 | yes | registry: known answers of the gate register and milestone layer (counts, tables and arithmetic to reproduce) | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U40/N40-KA.md` |
| N41-KA | M0 | task | EPIC-M0 | yes | registry: known answers of the delivery registers (risks, open questions, tracker, first session, done) | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U41/N41-KA.md` |

## Addressed to your beads

- `N36-KA`: request from round 2, slice S12: same as N16-KA: registry/known_answers/perf.toml -> the one file
- `N37-KA`: request from round 2, slice S12: same as N16-KA: registry/known_answers/safe.toml -> the one file
- `N38-KA`: request from round 2, slice S14: KA-837's n = 0.7 control prints 9.17 cm; computed from the stated constants (unrounded p = 5.01648 kPa) it is 0.091646 m = 9.16 cm; 9.17 follows from p rounded to 5.017 kPa before the power 1/n. Align to 9.16 (0.091646, 0.190127, 0.117076 m), as N38b-43 and N38b-46 now state. Its acceptance asks the test to reproduce the control's printed values, which fails at 10^-5 m. Optionally record the slip under the reconciliation register's standing rule (status open, then corrected).
- `N38-KA`: request from round 2, slice S78: KA-837's n = 0.7 control still prints 9.17, 19.01 and 11.71 cm (and p = 5.017 kPa); from the unrounded p = 5.01648 kPa it is 0.091646, 0.190127 and 0.117076 m (9.16 cm), as BD-366, N18b-11, N18b-12, BD-417, N38b-43 and N38b-46 now state (S14's request).
- `N41-KA`: request from round 2, slice S05: keeps its own file crates/frob-conformance/registry/known_answers/delivery.toml with K41- ids and pinned_by/bead fields, which N38a-12 forbids (one file, its row schema): append the rows to known_answers.toml in N38a-12's schema, take a block from N38a-40 and map the K41- tags in table order, and give each row the due milestone of its pinning bead, as N40-KA now does
- `N41-KA`: request from round 2, slice S12: same as N16-KA: registry/known_answers/delivery.toml -> the one file
