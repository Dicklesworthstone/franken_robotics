# EV-pack H-09 — Incremental evaluation and reuse after an edit

Session `RS-20261121-frob-fable-10x`. Marks: FETCHED = read this session; SUMMARY = search-engine summary only.

## Claim under test

Dependency-tracked results let a typical single-parameter edit reuse at least 70% of prior evaluation work, by content-addressed memoization of analyses, kernels and scene compiles, and by warm starts for controllers and orbits. Falsifier: measured reuse below 30% on a corpus of edit histories.

## Evidence against (excerpts)

1. Where the compute is. Per-design training on the incumbent takes "under four minutes, and … twenty minutes for uneven terrain" of GPU time (FETCHED, Rudin et al., https://arxiv.org/pdf/2109.11978). The memoizable parts are small beside it: a cold kernel compile is "Total JIT time: 95.91 s" and is paid once per machine, "Total JIT time: 0.89 s" when cached (FETCHED, https://github.com/google-deepmind/mujoco_warp/pull/989 and /pull/1014); static analyses cost milliseconds to seconds (my estimate, not measured). So at least 90% of per-design compute is rollouts and training.
2. Those results depend on every mass and geometry parameter (argument, not a fetched source). The mass matrix, bias forces and contact Jacobians enter every step, and legged contact dynamics amplifies small differences, so no rollout or gradient is bit-reusable after such an edit. Exact reuse weighted by compute is therefore at most about 10% for a dynamics-affecting edit, below the 30% falsifier.
3. Warm starts are a different mechanism with a smaller published gain. Warm-starting PPO from pretrained weights gave "an average 36.9% gain in sample efficiency" across nine tasks (SUMMARY, arXiv 2510.12363); co-design authors state that robots generally need retraining per morphology (SUMMARY, arXiv 2505.21665). Counting warm starts here double-counts H-05 and H-21.
4. Incremental frameworks carry overhead; traditional incremental computation slowed by 4× to 500× on some change patterns (SUMMARY, Adapton, PLDI 2014).
5. "Reuse of work" is ambiguous. Counted by artifacts it is trivially high; counted by GPU-seconds it is low.

## Evidence for (excerpts)

- Edits that do not touch the robot model (reward, controller, sensor placement without mass, scene) leave all model analyses valid.
- Kernel reuse is total when the model is data: "Certain fields are safe to modify directly without compilation, enabling on-device updates"; "many `mjw.Model` fields have a leading batch dimension" (FETCHED, https://mujoco.readthedocs.io/en/latest/_sources/mjwarp/index.rst.txt).
- Tolerance-level reuse has a home: `fs-recompute` is "A content-addressed Merkle DAG" whose `Store::can_skip` answers "is the cached artifact still good enough?" with slack attached (FETCHED, `/Users/jemanuel/projects/frankensim/crates/fs-recompute/src/lib.rs`).

## Kill tests

1. Artifact: present in part (`fs-recompute`; invalidation is behind the `tolerance-invalidation` feature). 2. Lever: FAILS; no unit for "work" and no baseline. 3. Scale: passes. 4. Foundation: passes for memoization; a certified bound on how a parameter edit moves a rollout statistic does not exist for contact-rich dynamics. 5. Conflation: none. 6. Rival: FAILS. The incumbent already caches kernels and updates model fields in place (excerpt above); memoizing static analyses is an ordinary content-addressed cache over any engine.

## Cheapest discriminating test

Step one, two hours, no compute: a dependency table of results against parameters (link length, link mass, gear ratio, friction, controller gain, reward weight, terrain), each cell marked reusable, warm-startable or invalid, with a typical GPU-seconds column. Step two, one day and about 10 GPU-hours: instrument 30 edits on MuJoCo Warp with PPO, log time per stage, and compute reuse weighted by GPU-seconds, with and without warm starts. Kill if weighted exact reuse is below 30%.

## Recommended state and the claim that survives

REFUTED as stated, on arithmetic from fetched numbers and on the premise that rollouts dominate per-design compute (that premise is H-18's to test). Expected exact reuse for a mass or geometry edit is at most about 10% of compute. What survives is hygiene, not a ten-fold source: full reuse of kernels, scene compiles and static analyses, and full reuse of model analyses for controller-only edits. Warm-start gains belong to H-05 and H-21.

## Open points

The mix of edit types in real agent sessions; whether adjoint sensitivities (`fs-adjoint`) can justify skipping a re-evaluation when a metric moves less than the decision tolerance; no measured edit corpus exists.
