# Question of record — FrankenRobotics ten-fold session (`RS-20261121-frob-fable-10x`)

Mode: fresh-question, compressed loop (see `../deliverables/HANDBACK.md` for which phases were compressed and why).
Target: `/Users/jemanuel/projects/franken_robotics/COMPREHENSIVE_PLAN_FOR_FRANKEN_ROBOTICS.md` (v0.2).
Workspace: `/Users/jemanuel/projects/franken_robotics/planning/fable/brenner/`.

## Q-001

Which mechanisms, if any, let FrankenRobotics reach an **accepted robot-design decision** with at least ten times
less elapsed time and compute than the strongest incumbent pipeline at equal or better decision quality, and which
proposed mechanisms should be killed?

An *accepted design decision* is: a design, a controller and an evidence package that an independent referee
protocol accepts as meeting a stated mission on held-out scenes, with the failure probability bounded at a stated
level.

## Falsifier for the whole thesis

On a pre-registered benchmark of design missions, run by a party that did not build the system, the FrankenRobotics
pipeline needs more than one tenth of the GPU-hours **or** more than one tenth of the wall-clock of the strongest
incumbent pipeline (MuJoCo Warp or Newton with per-design PPO and a standard outer-loop optimizer, tuned by its own
advocates) for the median mission at matched accepted quality. If so the ten-fold thesis is dead and the plan must
say so.

## Kill tests every hypothesis must survive

A hypothesis is killed if any of these holds.

1. **No artifact.** It cannot name the crate, the data layout or algorithm, and the test that would show it working.
2. **No lever.** It cannot state a number, a baseline and a gate.
3. **Scale.** It violates the memory, compute or precision envelope of an M5 Ultra (256 GB unified, 80 GPU cores)
   or two RTX 4090 (24 GB each, no link between them).
4. **Foundation.** It needs a capability a sibling project does not have, and names no fallback.
5. **Conflation.** It presents a proof about a model as a fact about a real robot, or parity with a simulator as
   agreement with reality.
6. **Rival.** An incumbent engine with a thin agent layer already does it, so it is not a source of the ten-fold.

## Scope

In: mechanisms for evaluation speed, search pruning, synthesis, verification, world realism, agent ergonomics, GPU
and memory use. Out: business questions, physical hardware, staffing.

## Third-alternative guard

The slate must contain hypotheses under which the leading ideas are *all* wrong about where the ten-fold lives.
See H-18 and H-19 in `../hypothesis_slate.md`.
