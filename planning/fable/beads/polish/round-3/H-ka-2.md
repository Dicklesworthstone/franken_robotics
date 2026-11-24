# Polishing round 3, slice H-ka-2

14 beads.

| Key | Milestone | Type | Parent | Changed last round | Title | File |
|---|---|---|---|---|---|---|
| N22-KA | M1 | task | WP-14 | yes | Reflex known answers (KA-553 to KA-575): estimator, certificates, sampling cost, gaits, skills, shields | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U22a/N22-KA.md` |
| N23-KA | M2 | task | WP-48 | yes | Known-answer registry for Cortex (KA-576 to KA-593): gym semantics, PPO rules, shards, strides, requirements | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U23a/N23-KA.md` |
| N24-KA | M1 | task | WP-41 | yes | Known-answer registry for Arena (KA-594 to KA-623): requirements, metrics, fixtures, studies, qualification | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U24a/N24-KA.md` |
| N25-KA | M1 | task | WP-42 | yes | Augur known-answer registry: KA-624 to KA-658, truth, tolerance, tier, control; eighteen late answers unnumbered | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U25a/N25-KA.md` |
| N26-KA | M3 | task | WP-55 | yes | Nemesis known-answer registry: KA-659 to KA-669 with truths, tolerances, tiers, controls and owner beads | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U26a/N26-KA.md` |
| N27-KA | M2 | task | WP-56 | yes | Known-answer registry for Autopsy: KA-670 to KA-679, the related answers, derived values and their controls | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U27a/N27-KA.md` |
| N28-KA | M1 | task | WP-14 | yes | registry: the known answers of the design studio (KA-680 to KA-706), with tolerances, tiers and negative controls | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U28a/N28-KA.md` |
| N29-KA | M1 | task | WP-13 | yes | Chronicle known-answer registry: KA-707 to KA-720 with truths, tolerances, tiers and negative controls | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U29a/N29-KA.md` |
| N30-KA | M5 | task | WP-68 | no (here for a request) | Ground known-answer registry: KA-721 to KA-736, the synthetic rehearsal's truths with tolerances, tiers and controls | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U30a/N30-KA.md` |
| N31-KA | M5 | task | WP-70 | yes | Known-answer registry for Foundry: KA-737 to KA-754, related construction answers, derived values and controls | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U31a/N31-KA.md` |
| N32-KA | M0 | task | WP-06 | yes | frob-conformance: known-answer registry of the surface chapter (KA-755 to KA-766) with derivations | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U32a/N32-KA.md` |
| N33-KA | M0 | task | WP-37 | no (here for a request) | frob-conformance: known-answer registry of suite consumption (KA-767 to KA-780) with derivation files and controls | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U33a/N33-KA.md` |
| N34-KA | M0 | task | WP-01 | yes | Workspace known-answer registry: KA-781 to KA-784 | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U34/N34-KA.md` |
| N35-KA | M1 | task | WP-14 | yes | frob-conformance: known-answer registry rows KA-785 to KA-805 (numerics and determinism) | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U35/N35-KA.md` |

## Addressed to your beads

- `N23-KA`: request from round 2, slice S12: same as N16-KA: registry/known_answers/cortex.toml -> the one file known_answers.toml in N38a-12's schema
- `N24-KA`: request from round 2, slice S12: same as N16-KA: registry/known_answers/arena.toml -> the one file
- `N24-KA`: request from round 2, slice S37: e2e `frob gauntlet run --tier G1 --filter arena` uses a `frob gauntlet …` or `frob gates …` command, which does not exist (ADR-GAUNT-01): use `FROB_TEST_SEED=<s> cargo test -p frob-conformance --test <target> -- <filter>` and read registry rows (pilots.toml, gates.toml) read-only, as N20a-25/N20a-27/N20a-43 now do
- `N26-KA`: request from round 2, slice S12: same as N16-KA: registry/known_answers/nemesis.toml -> the one file
- `N26-KA`: request from round 2, slice S63: KA-664's control column still reads 'an episode count from ln(1/delta)/p* rounded down (29,957; 299,573; 2,995,732)', which errs on the safe side and is withdrawn by NEG-04 and CX-400: replace it with the undercounts one below the exact values (29,955; 299,571; 2,995,730; 76,958 for the tenth request), for which (1 - p*)^n > delta.
- `N27-KA`: request from round 2, slice S12: same as N16-KA: registry/known_answers/autopsy.toml -> the one file
- `N30-KA`: request from round 2, slice S12: same as N16-KA: registry/known_answers/ground.toml -> the one file
- `N31-KA`: request from round 2, slice S12: same as N16-KA: registry/known_answers/foundry.toml -> the one file
- `N31-KA`: request from round 2, slice S84: Part 3's heading says 'tests drafted by unit U31b': name N31b-04 (a dependency) as the author of the foundry/S-1 to S-6 derivation sections in docs/derivations/foundry/derived.md and of their tests (s_1_... to s_6_member_sizing); N31b-04 no longer requests KA numbers.
- `N32-KA`: request from round 2, slice S12: derivation files are placed under crates/frob-conformance/known_answers/tether/, one per row; N38a-12 puts them at docs/derivations/<fixture>.md, one per fixture, with the rows listed in front matter
- `N33-KA`: request from round 2, slice S12: same as N16-KA: registry/known_answers/suite.toml -> the one file
- `N35-KA`: request from round 2, slice S52: KA-790 and KA-791 name 'the anchored-position bead of the Spark unit' as carrier of the device form: it is N21b-37.
