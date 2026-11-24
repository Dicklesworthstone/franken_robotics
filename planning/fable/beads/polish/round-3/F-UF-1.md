# Polishing round 3, slice F-UF-1

33 beads.

| Key | Milestone | Type | Parent | Changed last round | Title | File |
|---|---|---|---|---|---|---|
| FD-ASUP-01 | M0 | task | WP-02 | yes | upstream asupersync, frankensim, frankentorch, fastmcp_rust: one published asupersync version for all | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-ASUP-01.md` |
| FD-FDGR-01 | M5 | task | WP-32 | yes | upstream franken_drone_geometry_reconstruction: published metric geometry with coverage and uncertainty | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FDGR-01.md` |
| FD-FKER-01 | M1 | task | WP-36 | yes | upstream asupersync (franken-kernel): claim and evidence identifier types beside the existing three | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FKER-01.md` |
| FD-FLN-01 | M5 | task | WP-31 | yes | upstream franken_lean: Mathlib roots unblocked (matrix exponential, Lie groups, quaternions, representations) | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FLN-01.md` |
| FD-FLN-02 | M5 | task | WP-31 | yes | upstream franken_lean: its suite lock certified on macOS aarch64 (today only x86-64 Linux) | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FLN-02.md` |
| FD-FMCP-01 | M0 | task | WP-06 | yes | upstream fastmcp_rust: brought onto the one toolchain T, its test suite and MCP conformance checks passing | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FMCP-01.md` |
| FD-FNP-01 | M1 | task | WP-37 | yes | upstream franken_numpy, frankentorch: a named, checked conversion between byte strides and element strides | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FNP-01.md` |
| FD-FSIM-02 | M2 | task | WP-22 | yes | upstream frankensim: a persistent contact time stepper (its open Moreau–Jean lane; barrier lane stays its own) | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FSIM-02.md` |
| FD-FSIM-03 | M1 | task | WP-10 | yes | upstream frankensim: capsule and hull support maps, field pair gaps, contact generation over an interval | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FSIM-03.md` |
| FD-FSIM-04 | M2 | task | WP-39 | yes | upstream frankensim: holonomic loop constraints and nonholonomic rolling in `fs-mbd` | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FSIM-04.md` |
| FD-FSIM-05 | M6 | task | WP-74 | yes | upstream frankensim: adjoints through the articulated step and through the contact step at a fixed mode | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FSIM-05.md` |
| FD-FSIM-06 | M1 | task | WP-02 | yes | upstream frankensim: one vector vocabulary across `fs-mbd` and `fs-ga` (conversions there, then none here) | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FSIM-06.md` |
| FD-FSIM-08 | M1 | task | WP-07 | yes | upstream frankensim: the `Component` trait of `fs-ports` frozen as contract v1, with a conformance kit | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FSIM-08.md` |
| FD-FSIM-09 | M2 | task | WP-46 | yes | upstream frankensim: a home for device receipts (evidence fields AE-01 to AE-12) in its evidence types | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FSIM-09.md` |
| FD-FSIM-10 | M1 | task | WP-37 | yes | upstream frankensim: correct `fs_g1_train::ppo_update` (clip branch, critic gradient, episode boundaries) | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FSIM-10.md` |
| FD-FSIM-11 | M1 | task | WP-37 | yes | upstream frankensim: `planar_drive` without the near-straight defect (the half-angle chord form) | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FSIM-11.md` |
| FD-FSIM-12 | M2 | task | WP-39 | yes | upstream frankensim: a moving-surface owner for the sustained contact step (witness migration) | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FSIM-12.md` |
| FD-FSIM-13 | M2 | task | WP-39 | yes | upstream frankensim: sustained contact beyond 21 contacts (larger solve, islands, or a qualified patch model) | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FSIM-13.md` |
| FD-FSIM-14 | M1 | task | WP-01 | yes | upstream frankensim: documented build-identity setting for path consumers outside the tree (`fs-la/build.rs`) | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FSIM-14.md` |
| FD-FSIM-17 | M3 | task | WP-50 | yes | upstream frankensim: `fs-uq` batched multilevel entry point and importance sampling beyond Gaussian germs | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FSIM-17.md` |
| FD-FSIM-18 | M4 | task | WP-55 | yes | upstream frankensim: the 10⁶ sample caps of `fs-eproc` and of `fs-uq` importance execution parameterized | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FSIM-18.md` |
| FD-FSIM-19 | M4 | task | WP-55 | yes | upstream frankensim: an adaptive multilevel splitting estimator beside the importance sampler of `fs-uq` | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FSIM-19.md` |
| FD-FSIM-20 | M4 | task | WP-64 | yes | upstream frankensim: articulated-dynamics derivatives and hybrid multiple shooting (`fs-orbit`, `fs-mbd`) | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FSIM-20.md` |
| FD-FSIM-21 | M2 | task | WP-09 | yes | upstream frankensim: a second polarization pair for the battery element of `fs-phs` | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FSIM-21.md` |
| FD-FSIM-22 | M6 | task | WP-73 | yes | upstream frankensim: dynamic rods (`fs-solid` has geometrically exact rods for statics) and a batched cloth | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FSIM-22.md` |
| FD-FSIM-23 | M5 | task | WP-68 | yes | upstream frankensim: `fs-assimilate` windowed or streaming entry to the iterated update beyond 256 rows | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FSIM-23.md` |
| FD-FSIM-24 | M5 | task | WP-68 | yes | upstream frankensim: a cross-ISA statement for the ensemble lane of `fs-assimilate` | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FSIM-24.md` |
| FD-FSIM-25 | M5 | task | WP-68 | yes | upstream frankensim: a correspondence-search lane in `fs-asbuilt` | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FSIM-25.md` |
| FD-FSIM-26 | M5 | task | WP-71 | yes | upstream frankensim: a mesh entry point to the faceted STEP exporter of `fs-io` | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FSIM-26.md` |
| FD-FSIM-27 | M5 | task | WP-70 | yes | upstream frankensim: a plane-stress option in `fs-topols` (its sampled stress is plane strain) | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FSIM-27.md` |
| FD-FSIM-28 | M5 | task | WP-70 | yes | upstream frankensim: a stable, ungated surface for `fs-topopt::sdf3` | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FSIM-28.md` |
| FD-FSIM-29 | M5 | task | WP-71 | yes | upstream frankensim: geometric evaluators for overhang and tool access in `fs-fab` | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FSIM-29.md` |
| FD-FSIM-30 | M5 | task | WP-70 | yes | upstream frankensim: a statement of boundary conformity in `fs-mesh` | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FSIM-30.md` |

## Addressed to your beads

- `FD-ASUP-01`: request from round 2, slice S09: Same P0 ordering fix as FD-FMCP-01: it precedes BD-037, so it writes its own [[item]] row in BD-037's field names and opens its own failing fixture; its pre-phase uses frob doctor --section upstream (logged not-run before BD-022); add dep STD-E2E.
- `FD-FKER-01`: request from round 2, slice S04: N33c-09 no longer has suite.dev-only: a frob-core -> franken-kernel edge is BD-003's L-4.foundation-confined on the FD-FKER row; case 1's narrowing is an ADR changing that row's uses, and the negative control expects L-4.foundation-confined
- `FD-FSIM-12`: request from round 2, slice S52: Its one ledger re-baseline is the cause `upstream:FD-FSIM-12` of N16bp1-01's re-baseline record (new strata, no edit of frozen envelopes). Its post phase needs BD-212 and N16bp1-01, which it does not depend on; NFp2-01 shows the split made for FD-FSIM-02 (a bead after both), which would suit it too.
- `FD-FSIM-19`: request from round 3, slice F-U26b: Phase post can now name the selector: N26bp1-01 defines nemesis.splitting.estimator = ours | upstream (set per call in a scratch configuration, one binary); please name it and assert each call's result estimator (frob-nemesis | fs-uq).
- `FD-FSIM-21`: request from round 3, slice H-verbs-6: Replace `--seeds` with a loop over `--seed <s>`.
