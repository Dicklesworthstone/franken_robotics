# Polishing round 4, slice F-UF-1

28 beads.

| Key | Milestone | Type | Parent | Changed last round | Title | File |
|---|---|---|---|---|---|---|
| FD-ASUP-01 | M0 | task | WP-02 | yes | upstream asupersync, frankensim, frankentorch, fastmcp_rust: one published asupersync version for all | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-ASUP-01.md` |
| FD-F3D-01 | M3 | task | WP-52 | no (here for a request) | upstream franken_threed: exact pin `serde_json = "=1.0.151"` relaxed (the same change as in franken_textiles) | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-F3D-01.md` |
| FD-FDGR-01 | M5 | task | WP-32 | yes | upstream franken_drone_geometry_reconstruction: published metric geometry with coverage and uncertainty | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FDGR-01.md` |
| FD-FKER-01 | M1 | task | WP-36 | yes | upstream asupersync (franken-kernel): claim and evidence identifier types beside the existing three | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FKER-01.md` |
| FD-FLN-01 | M5 | task | WP-31 | yes | upstream franken_lean: Mathlib roots unblocked (matrix exponential, Lie groups, quaternions, representations) | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FLN-01.md` |
| FD-FLN-02 | M5 | task | WP-31 | yes | upstream franken_lean: its suite lock certified on macOS aarch64 (today only x86-64 Linux) | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FLN-02.md` |
| FD-FMCP-01 | M0 | task | WP-06 | yes | upstream fastmcp_rust: brought onto the one toolchain T, its test suite and MCP conformance checks passing | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FMCP-01.md` |
| FD-FSIM-02 | M2 | task | WP-22 | yes | upstream frankensim: a persistent contact time stepper (its open Moreau–Jean lane; barrier lane stays its own) | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FSIM-02.md` |
| FD-FSIM-03 | M1 | task | WP-10 | yes | upstream frankensim: capsule and hull support maps, field pair gaps, contact generation over an interval | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FSIM-03.md` |
| FD-FSIM-04 | M2 | task | WP-39 | yes | upstream frankensim: holonomic loop constraints and nonholonomic rolling in `fs-mbd` | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FSIM-04.md` |
| FD-FSIM-06 | M1 | task | WP-02 | yes | upstream frankensim: one vector vocabulary across `fs-mbd` and `fs-ga` (conversions there, then none here) | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FSIM-06.md` |
| FD-FSIM-08 | M1 | task | WP-07 | yes | upstream frankensim: the `Component` trait of `fs-ports` frozen as contract v1, with a conformance kit | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FSIM-08.md` |
| FD-FSIM-10 | M1 | task | WP-37 | yes | upstream frankensim: correct `fs_g1_train::ppo_update` (clip branch, critic gradient, episode boundaries) | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FSIM-10.md` |
| FD-FSIM-11 | M1 | task | WP-37 | yes | upstream frankensim: `planar_drive` without the near-straight defect (the half-angle chord form) | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FSIM-11.md` |
| FD-FSIM-12 | M2 | task | WP-39 | yes | upstream frankensim: a moving-surface owner for the sustained contact step (witness migration) | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FSIM-12.md` |
| FD-FSIM-13 | M2 | task | WP-39 | yes | upstream frankensim: sustained contact beyond 21 contacts (larger solve, islands, or a qualified patch model) | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FSIM-13.md` |
| FD-FSIM-14 | M1 | task | WP-01 | yes | upstream frankensim: documented build-identity setting for path consumers outside the tree (`fs-la/build.rs`) | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FSIM-14.md` |
| FD-FSIM-19 | M4 | task | WP-55 | yes | upstream frankensim: an adaptive multilevel splitting estimator beside the importance sampler of `fs-uq` | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FSIM-19.md` |
| FD-FSIM-20 | M4 | task | WP-64 | yes | upstream frankensim: articulated-dynamics derivatives and hybrid multiple shooting (`fs-orbit`, `fs-mbd`) | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FSIM-20.md` |
| FD-FSIM-21 | M2 | task | WP-09 | yes | upstream frankensim: a second polarization pair for the battery element of `fs-phs` | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FSIM-21.md` |
| FD-FSIM-22 | M6 | task | WP-73 | yes | upstream frankensim: dynamic rods (`fs-solid` has geometrically exact rods for statics) and a batched cloth | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FSIM-22.md` |
| FD-FSIM-23 | M5 | task | WP-68 | yes | upstream frankensim: `fs-assimilate` windowed or streaming entry to the iterated update beyond 256 rows | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FSIM-23.md` |
| FD-FSIM-24 | M5 | task | WP-68 | yes | upstream frankensim: a cross-ISA statement for the ensemble lane of `fs-assimilate` | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FSIM-24.md` |
| FD-FSIM-25 | M5 | task | WP-68 | yes | upstream frankensim: a correspondence-search lane in `fs-asbuilt` | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FSIM-25.md` |
| FD-FSIM-31 | M1 | task | WP-01 | yes | upstream frankensim: the `fs-qty` dependency of `fs-evidence` removed or put behind a feature | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FSIM-31.md` |
| FD-FSIM-33 | M1 | task | WP-02 | yes | upstream frankensim: `fs-mbd::Wrench` mixed frames documented, or named body and world constructors | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FSIM-33.md` |
| FD-FSS-01 | M5 | task | WP-32 | yes | upstream franken_surveillance_system (conditional): twin package landmarks, scale witness, coverage | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FSS-01.md` |
| FD-FSYM-03 | M2 | task | WP-03 | yes | upstream frankensympy: permutation groups and character tables in Rust | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FSYM-03.md` |

## Addressed to your beads

- `FD-F3D-01`: request from round 3, slice F-UF-1: As for FD-FTUI-01: state BD-037's `pin_state = unpinned` rule and that this bead re-runs the fixture at franken_threed's first pin and clears the field.
- `FD-FSIM-23`: request from round 4, slice H-verbs-4: VERBS-ground's ground.identify now lists --windows <def | raw> and 'Extended by: FD-FSIM-23 (--windows raw, M5)', with your 724 answer at a pin without the streaming entry. Please state it as the row's form and drop 'requests of the ground.identify row (VERBS-ground, BD-391)' at lines 149-150.
- `FD-FSS-01`: request from round 3, slice F-UF-1: As for FD-FTUI-01: state BD-037's `pin_state = unpinned` rule (and its `observed_at = pending: inspection pilot` exemption) in the bead.
- `FD-FSS-01`: mechanical check of the conventions: [C13-pending] 1 place(s) narrate a pending request instead of stating the owner's form (convention 13), e.g. "…e registry row's `status` to `not-needed` (DESIGN: the value requested of BD-037's schema `frob.upstream/1`, admitted only on a conditional …"
