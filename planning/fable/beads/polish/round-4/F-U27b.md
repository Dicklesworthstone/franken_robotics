# Polishing round 4, slice F-U27b

17 beads.

| Key | Milestone | Type | Parent | Changed last round | Title | File |
|---|---|---|---|---|---|---|
| BD-279 | M3 | feature | WP-56 | yes | verb autopsy.intervene: interventions, placebo forks, replicates and coalitions from a fork handle, with a ledger | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U27b/BD-279.md` |
| BD-280 | M3 | feature | WP-56 | yes | verb autopsy.attribute: interventional contrasts with flip rates and labels, in double precision | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U27b/BD-280.md` |
| BD-284 | M3 | feature | WP-56 | yes | verb autopsy.show and the transcript frob.autopsy.transcript/1: time-travel debugging as five replayable moves | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U27b/BD-284.md` |
| N27b-04 | M3 | task | WP-56 | yes | e2e autopsy.intervene: null and placebo forks on 1,000 failing worlds, fork counts, the gear-change refusal | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U27b/N27b-04.md` |
| N27b-07 | M3 | task | WP-56 | yes | frob-autopsy: precision routing for attribution: double-precision default, decision equivalence, FROB-REF-584 | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U27b/N27b-07.md` |
| N27b-08 | M3 | task | WP-56 | yes | frob-autopsy: the planted-single-cause suite: 200 failures in five classes, ten candidates each, with its harness | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U27b/N27b-08.md` |
| N27b-10 | M2 | task | WP-56 | yes | Pilot: planted-cause attribution at paper scale: one-at-a-time contrasts against exact Shapley on 1,024 coalitions | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U27b/N27b-10.md` |
| N27b-14 | M3 | feature | WP-56 | yes | verb autopsy.repair: 1-minimal repair sets by shrinking the intervention set; diagnostic factors kept apart | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U27b/N27b-14.md` |
| N27b-16 | M3 | task | WP-56 | yes | frob-verbs: the autopsy cluster's rows checked against the handlers: examples run, refusals covered, no second spelling | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U27b/N27b-16.md` |
| N27b-18 | M3 | task | WP-56 | no (here for a request) | e2e time-travel debugging session: explain, fork, intervene, attribute, repair, re-run; transcript replay | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U27b/N27b-18.md` |
| N27b-20 | M3 | task | WP-56 | yes | frob-autopsy: invariant and proof-obligation suite: an artifact auditor, mutants and independent cross-checks | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U27b/N27b-20.md` |
| N27b-21 | M3 | task | WP-56 | yes | frob-conformance: the Autopsy known-answer suite: KA-670 to KA-679 and two lane answers, fixtures and controls | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U27b/N27b-21.md` |
| N27b-22 | M2 | task | WP-56 | yes | Pilot: growth of placebo separation G(W) and single-precision admission on reference closed loops | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U27b/N27b-22.md` |
| N27b-24 | M3 | feature | WP-56 | yes | frob-autopsy: the single-precision lane: --lane f64/tier-f32/auto, 16 replicates at 10⁻⁷ on the originating tier | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U27b/N27b-24.md` |
| N27b-26 | M3 | task | WP-56 | yes | frob-autopsy: double-precision cross-check of the single-precision lane, one in eight, with an agreement ledger | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U27b/N27b-26.md` |
| N27b-27 | M3 | feature | WP-56 | yes | frob-autopsy: bulk attribution of a failure cluster on a device: batch planner, cost model, speedup ceiling | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U27b/N27b-27.md` |
| N27b-28 | M3 | task | WP-56 | yes | e2e GATE-AUTOPSY-09: the single-precision lane on a device: known answers, G(W), agreement, bulk speedup | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U27b/N27b-28.md` |

## Addressed to your beads

- `N27b-10`: mechanical check of the conventions: [C11-script] has no end-to-end script of its own (`scripts/e2e/<name>.sh`): convention 11 gives each mechanism bead one, unless it delivers nothing a `frob` command can reach (then say so in the test plan)
- `N27b-18`: request from round 4, slice H-verbs-6: ANSWER: there is no role flag. To cite as established: `frob report run <run> --cite <claim> --key <k> --json` (role Established; an Autopsy claim gives FROB-REF-880, exit 3). Under the Reality claim: `frob report run <run> --claim reality --cite <claim> --key <k> --json` (role Reality, also 880). Without --cite the claim is printed in its described wording (role Described) and is not refused.
- `N27b-21`: mechanical check of the conventions: [C11-script] has no end-to-end script of its own (`scripts/e2e/<name>.sh`): convention 11 gives each mechanism bead one, unless it delivers nothing a `frob` command can reach (then say so in the test plan)
- `N27b-22`: mechanical check of the conventions: [C11-script] has no end-to-end script of its own (`scripts/e2e/<name>.sh`): convention 11 gives each mechanism bead one, unless it delivers nothing a `frob` command can reach (then say so in the test plan)
