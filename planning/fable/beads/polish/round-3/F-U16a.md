# Polishing round 3, slice F-U16a

31 beads.

| Key | Milestone | Type | Parent | Changed last round | Title | File |
|---|---|---|---|---|---|---|
| BD-081 | M1 | task | WP-39 | yes | frob-dyn: the transactional referee step, with the whole restore set and the commit fence on `commit_epoch` | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U16a/BD-081.md` |
| BD-082 | M1 | task | WP-39 | yes | frob-dyn: event location on rational base intervals, calling coupled impulses per island (steps T4.1 to T4.6) | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U16a/BD-082.md` |
| BD-088 | M1 | task | WP-10 | yes | frob-conformance: the dynamics known-answer battery on the referee (GATE-KIN-01) | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U16a/BD-088.md` |
| BD-089 | M1 | task | WP-10 | yes | frob-conformance: contact-free dynamics parity of each reference robot against the pinned oracle | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U16a/BD-089.md` |
| BD-150 | M2 | task | WP-15 | yes | frob-kernels: derive and emit `K-crba-level` and `K-rnea-level` (mass matrix and bias forces by tree level) | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U16a/BD-150.md` |
| BD-159 | M2 | feature | WP-15 | yes | frob-dyn: the ten-stage batched step on `batch-cpu` in f64 and f32, with lockstep masks and capacity flags | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U16a/BD-159.md` |
| BD-213 | M2 | feature | WP-50 | yes | verb fidelity.coverage: the referee's own coverage per robot class, with causes and the adjudicator | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U16a/BD-213.md` |
| BD-384 | M5 | feature | WP-72 | yes | frob-dyn: the articulated variational integrator on the referee and the `strict` profile | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U16a/BD-384.md` |
| BD-385 | M5 | task | WP-72 | yes | Pilot: the tree-operator route for larger mechanisms and limb port packets, with PO-SKEL-04 | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U16a/BD-385.md` |
| N16a-01 | M1 | task | WP-10 | yes | frob-dyn: crate skeleton, CONTRACT.md, the seven Kinesis decisions on file, and the world-step / env-step units | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U16a/N16a-01.md` |
| N16a-02 | M1 | feature | WP-10 | yes | frob-dyn: tiers and step profiles, the profile-by-tier admission matrix and refusal FROB-REF-222 | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U16a/N16a-02.md` |
| N16a-03 | M1 | task | WP-10 | yes | frob-dyn: `StepPlanId`, the content hash of everything numerical that shaped a step, and the two-identity rule | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U16a/N16a-03.md` |
| N16a-04 | M1 | task | WP-39 | yes | frob-dyn: physical and numerical parameters as two typed sets with digests (ADR-KIN-05, INV-KIN-07) | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U16a/N16a-04.md` |
| N16a-05 | M1 | task | WP-39 | yes | frob-dyn: the referee's limit register (FrankenSim's limits as data) and routing of adjudication to `strict` | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U16a/N16a-05.md` |
| N16a-07 | M1 | task | WP-39 | yes | frob-dyn: one coupled impulse solve per island at an event (W = J·P·Jᵀ), with typed impulse units and FROB-REF-224 | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U16a/N16a-07.md` |
| N16a-08 | M1 | task | WP-39 | yes | frob-dyn: the commit checks of a transaction (step T5), with path admission by certified collision detection | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U16a/N16a-08.md` |
| N16a-09 | M1 | task | WP-39 | yes | Decision OPEN-KIN-5: L_max, E_max and the retry budget of the transactional step, from the M1 refusal rate | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U16a/N16a-09.md` |
| N16a-10 | M1 | task | WP-39 | yes | e2e: the transactional referee run through `frob sim run --profile referee` (referee rows of GATE-KIN-04) | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U16a/N16a-10.md` |
| N16a-11 | M2 | task | WP-15 | yes | frob-dyn: the default integrator, semi-implicit Euler on the group with implicit joint damping | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U16a/N16a-11.md` |
| N16a-13 | M2 | task | WP-15 | yes | frob-dyn: the path-table Jacobian, with digests identical to the dense Jacobian | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U16a/N16a-13.md` |
| N16a-14 | M2 | task | WP-15 | no (here for a request) | frob-dyn: default capacities per workload, the bytes-per-world model and the `Degraded`-rate gate | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U16a/N16a-14.md` |
| N16a-15 | M2 | task | WP-15 | no (here for a request) | Decision OPEN-KIN-3: final contact and row capacities for the workloads other than Q and H | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U16a/N16a-15.md` |
| N16a-16 | M2 | task | WP-22 | yes | Decision OPEN-KIN-1: default physics step and Newton iteration budget from the M2 frontier | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U16a/N16a-16.md` |
| N16a-18 | M2 | task | WP-15 | yes | e2e: the lockstep batched step through `frob sim run --tier batch-cpu` (drop, stand, walk, overflow) | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U16a/N16a-18.md` |
| N16a-19 | M2 | task | WP-44 | yes | e2e: 256 parameter variants in one batch equal 256 homogeneous runs bit for bit (`batch-cpu` rows of GATE-KIN-06) | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U16a/N16a-19.md` |
| N16a-20 | M2 | task | WP-50 | yes | e2e: `frob fidelity coverage` reports both coverage figures, the refusal causes and the adjudicator (GATE-FID-03) | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U16a/N16a-20.md` |
| N16a-21 | M5 | task | WP-72 | yes | Pilot: the variational step on a free body and a heavy top against the Cayley semi-implicit step | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U16a/N16a-21.md` |
| N16a-22 | M5 | task | WP-72 | yes | Kill-gate decision KILL-VARIATIONAL: keep the articulated variational integrator or fall back to the default | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U16a/N16a-22.md` |
| N16a-23 | M5 | task | WP-72 | yes | Kill-gate decision KILL-TREEOP: fund the tree-operator route or keep dense with a cap of 64 | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U16a/N16a-23.md` |
| N16a-24 | M6 | feature | WP-74 | yes | frob-dyn: the tree-operator solve path above the dense range (LᵀL on the kinematic tree) | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U16a/N16a-24.md` |
| N16ax-01 | M2 | task | WP-15 | yes | frob-dyn: ContactSetSig, one per-step signature of a world's active contact set, in the step and telemetry | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U16a/N16ax-01.md` |

## Addressed to your beads

- `BD-081`: request from round 2, slice S24: add row 12 to the restore-set table: the interval-extremum folds (component dyn.extremum-folds, N24a-44), owned and implemented by frob-dyn, registered in the world manifest; include it in the inventory test that mutates each component after a checkpoint.
- `BD-081`: request from round 3, slice H-verbs-6: Define the chaos-build-only sim.run inputs (test feature `chaos`): `--chaos faults=<n>,seed=<s>,point=<p>` (N16a-10), `--chaos-plan <file>` and `--numerical-plan-floor <k>` (N16c-02, N18a-37); VERBS-sim lists them as absent from release and conformance builds.
- `BD-385`: request from round 3, slice H-owners-1: Declare `[[bin]] name = "frob-pilot", path = "src/bin/frob_pilot.rs"` with a CONTRACT.md row (BD-025: autobins off, so the command name is the declared one).
- `N16a-05`: request from round 2, slice S59: Its e2e text and N16a-10's use '--adjudicate strict' while the policy enum is referee-only, patch-reduction, strict-fallback; N16a-20 now uses '--adjudicate strict-fallback'. Use the policy names as the flag's values (and request the same of N16a-10).
- `N16a-05`: request from round 3, slice H-verbs-6: Use the value `strict-fallback` (this bead's own set), not `--adjudicate strict`, in scripts and examples.
- `N16a-09`: request from round 3, slice H-verbs-6: Confirm the DESIGN rule that `--txn l-max=,e-max=,retries=` is admitted only under the transactional profiles (referee, strict) and is 802 under lockstep.
- `N16a-10`: request from round 3, slice H-verbs-6: Use `--adjudicate strict-fallback`, not `strict`; the `--chaos` input is chaos-build-only (BD-081).
- `N16a-14`: request from round 3, slice H-verbs-6: Adopt the DESIGN spelling `--capacity contacts=<C>[,rows=<R>]` for per-world capacities (new StepPlanId), used by N16a-12, N16a-18 and N16b-03.
- `N16a-15`: request from round 3, slice H-verbs-2: frob eval run: spell the seed --run-seed (not --seed). The design-less --battery standard --class form is now on the row (VERBS-eval DESIGN).
- `N16a-15`: request from round 3, slice H-verbs-6: Replace `--battery` on sim.run with `eval.run --battery` (no sim.run owner).
