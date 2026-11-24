# Polishing round 3, slice F-U17a

21 beads.

| Key | Milestone | Type | Parent | Changed last round | Title | File |
|---|---|---|---|---|---|---|
| BD-285 | M3 | feature | WP-63 | yes | frob-phylum: campaign quotienting: similarity form, orbit representatives, mission digest (phylum.quotient) | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U17a/BD-285.md` |
| BD-287 | M3 | feature | WP-63 | yes | frob-phylum: morphology generator: GeneratorId, addressed sampling, exact probabilities (phylum.generate) | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U17a/BD-287.md` |
| BD-356 | M4 | feature | WP-63 | yes | frob-phylum: design-family lanes (level P-C) with the speedup bound 1/(1 − f + f/r + h) | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U17a/BD-356.md` |
| N17a-01 | M2 | task | WP-44 | yes | frob-phylum: crate skeleton, CONTRACT.md, ownership boundary, the narrowed claim, ADR-PHYLUM-01 to -06 | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U17a/N17a-01.md` |
| N17a-02 | M2 | task | WP-44 | yes | frob-phylum: shape-class fitting: capacity requirement, smallest admitted class, P-A/P-B, FROB-REF-160 | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U17a/N17a-02.md` |
| N17a-03 | M2 | task | WP-44 | yes | frob-phylum: cohort formation: model slots, WorldSlots, runs of 16 or 32 worlds, results keyed by address | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U17a/N17a-03.md` |
| N17a-04 | M2 | task | WP-44 | yes | frob-phylum: padding and bucket arithmetic: Ω_p, its bounds, the pay-off inequality, the mixing ratio | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U17a/N17a-04.md` |
| N17a-05 | M2 | task | WP-44 | yes | frob-phylum: bucket planner: exact dynamic program, cap of eight buckets, default set, FROB-REF-741 | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U17a/N17a-05.md` |
| N17a-06 | M2 | task | WP-44 | yes | experiment: cohort cost-model calibration on the M2 profile: c_L, stage shares, run length, contact padding | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U17a/N17a-06.md` |
| N17a-07 | M2 | task | WP-44 | yes | frob-phylum: population memory model and placement verdicts per machine, with FROB-REF-740 | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U17a/N17a-07.md` |
| N17a-08 | M2 | task | WP-44 | yes | e2e phylum.plan: script for buckets, padding, bytes, placement and refusals 160, 740, 741 | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U17a/N17a-08.md` |
| N17a-10 | M2 | task | WP-44 | yes | frob-phylum: batch-independence test at the crate boundary and the GATE-PHYLUM-02 measurement | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U17a/N17a-10.md` |
| N17a-11 | M3 | task | WP-63 | yes | pilot: design-family lanes: the ceiling 1/(1 − f + f/r + h) from the M2 profile, before funding | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U17a/N17a-11.md` |
| N17a-13 | M4 | task | WP-63 | no (here for a request) | e2e family lanes: matched-work f, r, h and the paired whole-step ratio, with digest equality | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U17a/N17a-13.md` |
| N17a-16 | M4 | feature | WP-46 | yes | frob-phylum: survivor promotion to specialized kernels: promotion rule, 60 s cap, boundary swap | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U17a/N17a-16.md` |
| N17a-19 | M3 | task | WP-63 | yes | frob-phylum: DesignSplit: held-out designs by seed and stratum, whole Γ_obs orbits on one side | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U17a/N17a-19.md` |
| N17a-21 | M3 | task | WP-63 | yes | e2e phylum.generate: reproducibility across platforms, exact probabilities, splits; GATE-PHYLUM-07 | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U17a/N17a-21.md` |
| N17a-25 | M2 | task | WP-63 | no (here for a request) | pilot PILOT-C-22: mission-observable digest on the ballast fixture, end to end with replay, before funding | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U17a/N17a-25.md` |
| N17a-26 | M3 | task | WP-63 | yes | frob-phylum: mission-observable digest: declared read set, exact lowering, memoization, split reports | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U17a/N17a-26.md` |
| N17a-27 | M3 | task | WP-63 | yes | e2e phylum.quotient: orbit counts, ballast classes, lidar and equal-Σx controls, a quotiented sweep | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U17a/N17a-27.md` |
| N17a-28 | M3 | task | WP-63 | no (here for a request) | experiment: quotient audit: replay of 10⁴ in-class pairs, dispatches saved, the unquotiented arm | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U17a/N17a-28.md` |

## Addressed to your beads

- `BD-356`: request from round 3, slice H-ref-2: raise FROB-REF-866 through N36-05's `admit_experiment` (the code's one home) and file a ceiling record
- `N17a-06`: request from round 3, slice H-verbs-6: Pass the design positionally, not as `--design <d>`.
- `N17a-10`: request from round 3, slice H-verbs-6: Pass the design positionally, not as `--design <d>`.
- `N17a-11`: request from round 3, slice H-verbs-6: Pass the design positionally; `--rows exact` is in no row: use `--capacity rows=<R>` or define a pilot-only input here (conformance build) and request it of VERBS-sim.
- `N17a-13`: request from round 3, slice H-verbs-6: Pass the design positionally, not as `--design <d>`.
- `N17a-25`: request from round 3, slice H-verbs-6: Pass the design positionally, not as `--design <d>`.
- `N17a-28`: request from round 3, slice H-verbs-6: Pass the design positionally, not as `--design <d>`.
