# Polishing round 1, slice UF-1

37 beads (UF-1).

| Key | Milestone | Type | Parent | Title | File |
|---|---|---|---|---|---|
| FD-ASUP-01 | M0 | task | WP-02 | upstream asupersync, frankensim, frankentorch, fastmcp_rust: one published asupersync version for all | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-ASUP-01.md` |
| FD-F3D-01 | M3 | task | WP-52 | upstream franken_threed: exact pin `serde_json = "=1.0.151"` relaxed (the same change as in franken_textiles) | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-F3D-01.md` |
| FD-FDGR-01 | M5 | task | WP-32 | upstream franken_drone_geometry_reconstruction: published metric geometry with coverage and uncertainty | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FDGR-01.md` |
| FD-FKER-01 | M1 | task | WP-36 | upstream asupersync (franken-kernel): claim and evidence identifier types beside the existing three | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FKER-01.md` |
| FD-FLN-01 | M5 | task | WP-31 | upstream franken_lean: Mathlib roots unblocked (matrix exponential, Lie groups, quaternions, representations) | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FLN-01.md` |
| FD-FLN-02 | M5 | task | WP-31 | upstream franken_lean: its suite lock certified on macOS aarch64 (today only x86-64 Linux) | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FLN-02.md` |
| FD-FMCP-01 | M0 | task | WP-06 | upstream fastmcp_rust: brought onto the one toolchain T, its test suite and MCP conformance checks passing | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FMCP-01.md` |
| FD-FNP-01 | M1 | task | WP-37 | upstream franken_numpy, frankentorch: a named, checked conversion between byte strides and element strides | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FNP-01.md` |
| FD-FSIM-01 | M2 | task | WP-10 | upstream frankensim: spherical, universal, planar and cylindrical joints as native joint kinds in `fs-mbd` | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FSIM-01.md` |
| FD-FSIM-02 | M2 | task | WP-22 | upstream frankensim: a persistent contact time stepper (its open Moreau–Jean lane; barrier lane stays its own) | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FSIM-02.md` |
| FD-FSIM-03 | M1 | task | WP-10 | upstream frankensim: capsule and hull support maps, field pair gaps, contact generation over an interval | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FSIM-03.md` |
| FD-FSIM-04 | M2 | task | WP-39 | upstream frankensim: holonomic loop constraints and nonholonomic rolling in `fs-mbd` | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FSIM-04.md` |
| FD-FSIM-05 | M6 | task | WP-74 | upstream frankensim: adjoints through the articulated step and through the contact step at a fixed mode | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FSIM-05.md` |
| FD-FSIM-06 | M1 | task | WP-02 | upstream frankensim: one vector vocabulary across `fs-mbd` and `fs-ga` (conversions there, then none here) | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FSIM-06.md` |
| FD-FSIM-07 | M2 | task | WP-14 | upstream frankensim: deterministic elementary functions and cancellation checkpoints inside `fs-mbd` | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FSIM-07.md` |
| FD-FSIM-08 | M1 | task | WP-07 | upstream frankensim: the `Component` trait of `fs-ports` frozen as contract v1, with a conformance kit | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FSIM-08.md` |
| FD-FSIM-09 | M2 | task | WP-46 | upstream frankensim: a home for device receipts (evidence fields AE-01 to AE-12) in its evidence types | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FSIM-09.md` |
| FD-FSIM-10 | M1 | task | WP-37 | upstream frankensim: correct `fs_g1_train::ppo_update` (clip branch, critic gradient, episode boundaries) | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FSIM-10.md` |
| FD-FSIM-11 | M1 | task | WP-37 | upstream frankensim: `planar_drive` without the near-straight defect (the half-angle chord form) | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FSIM-11.md` |
| FD-FSIM-12 | M2 | task | WP-39 | upstream frankensim: a moving-surface owner for the sustained contact step (witness migration) | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FSIM-12.md` |
| FD-FSIM-13 | M2 | task | WP-39 | upstream frankensim: sustained contact beyond 21 contacts (larger solve, islands, or a qualified patch model) | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FSIM-13.md` |
| FD-FSIM-14 | M1 | task | WP-01 | upstream frankensim: documented build-identity setting for path consumers outside the tree (`fs-la/build.rs`) | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FSIM-14.md` |
| FD-FSIM-15 | M1 | task | WP-01 | upstream frankensim: `fs-qty` macro-generated dimension products behind a feature (no `generic_const_exprs`) | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FSIM-15.md` |
| FD-FSIM-16 | M1 | task | WP-02 | upstream frankensim: `constellation.lock` refreshed to the heads its `Cargo.lock` builds with | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FSIM-16.md` |
| FD-FSIM-17 | M3 | task | WP-50 | upstream frankensim: `fs-uq` batched multilevel entry point and importance sampling beyond Gaussian germs | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FSIM-17.md` |
| FD-FSIM-18 | M4 | task | WP-55 | upstream frankensim: the 10⁶ sample caps of `fs-eproc` and of `fs-uq` importance execution parameterized | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FSIM-18.md` |
| FD-FSIM-19 | M4 | task | WP-55 | upstream frankensim: an adaptive multilevel splitting estimator beside the importance sampler of `fs-uq` | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FSIM-19.md` |
| FD-FSIM-20 | M4 | task | WP-64 | upstream frankensim: articulated-dynamics derivatives and hybrid multiple shooting (`fs-orbit`, `fs-mbd`) | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FSIM-20.md` |
| FD-FSIM-21 | M2 | task | WP-09 | upstream frankensim: a second polarization pair for the battery element of `fs-phs` | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FSIM-21.md` |
| FD-FSIM-22 | M6 | task | WP-73 | upstream frankensim: dynamic rods (`fs-solid` has geometrically exact rods for statics) and a batched cloth | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FSIM-22.md` |
| FD-FSIM-23 | M5 | task | WP-68 | upstream frankensim: `fs-assimilate` windowed or streaming entry to the iterated update beyond 256 rows | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FSIM-23.md` |
| FD-FSIM-24 | M5 | task | WP-68 | upstream frankensim: a cross-ISA statement for the ensemble lane of `fs-assimilate` | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FSIM-24.md` |
| FD-FSIM-25 | M5 | task | WP-68 | upstream frankensim: a correspondence-search lane in `fs-asbuilt` | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FSIM-25.md` |
| FD-FSIM-26 | M5 | task | WP-71 | upstream frankensim: a mesh entry point to the faceted STEP exporter of `fs-io` | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FSIM-26.md` |
| FD-FSIM-27 | M5 | task | WP-70 | upstream frankensim: a plane-stress option in `fs-topols` (its sampled stress is plane strain) | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FSIM-27.md` |
| FD-FSIM-28 | M5 | task | WP-70 | upstream frankensim: a stable, ungated surface for `fs-topopt::sdf3` | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FSIM-28.md` |
| FD-FSIM-29 | M5 | task | WP-71 | upstream frankensim: geometric evaluators for overhang and tool access in `fs-fab` | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FSIM-29.md` |

## Addressed to your beads

- `FD-FSIM-19`: request from round 1, slice U26b: N26b-24 was split: the switch-over for this item (Procedure B) is now N26bp1-01, and its script is scripts/e2e/nemesis_switchover_splitting.sh; replace the references to N26b-24 (Procedure B) and nemesis_switchover.sh (lines 39, 116, 141, 143, 153, 156, 166). FD-FSIM-18's references to N26b-24 remain correct.

## Graph findings about your beads

None.
