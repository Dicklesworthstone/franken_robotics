# Seed beads of §45 (all 442), with the drafting unit that writes each

You may name any of these keys in `deps`. Only the owning unit writes the bead file.

| Key | Milestone | Unit | Title |
|---|---|---|---|
| BD-001 | M0 | U33c | Core workspace skeleton: `Cargo.toml`, one `rust-toolchain.toml` for the whole build graph naming candidate (a) of §33.13 as the provisional T, `.cargo/config.toml` wi… |
| BD-002 | M0 | U25a | Companion documents: `AGENTS.md`, `README.md` with a generated honest-status table, `DEPENDENCY_ADMISSION.md`, schema-valid registries (empty except `formats.toml` wit… |
| BD-003 | M0 | U34 | `xtask check-layers` and `check-deps` |
| BD-004 | M0 | U34 | `xtask check-stack`, `check-contracts`, `check-unsafe`, `check-generated`; the test that `xtask` has exactly six subcommands |
| BD-005 | M0 | U34 | `frob-core`: identities and hashing, evidence classes, the claim-product type, the no-authorization flags, the refusal type and registry loader, the `qty` re-export mo… |
| BD-006 | M0 | U33a | INT-FSIM-01 and the core row of INT-TOOLCHAIN-01: path dependencies on `fs-ga`, `fs-mbd`, `fs-contact`, `fs-time`, `fs-query` with the root patch, profile and build-id… |
| BD-007 | M0 | U33a | `frob-fsim` v0: `RefereeDynamics` over `fs_mbd::articulated`; the G1 digest test |
| BD-008 | M0 | U33b | INT-ASUP-01 wired into `check-stack`, and upstream work item FD-ASUP-01: FrankenSim's constellation, FrankenTorch and fastmcp_rust aligned on one published asupersync … |
| BD-009 | M0 | U33a | INT-FSQL-01: ledger open, write, forced kill, recover |
| BD-010 | M0 | U13 | `frob-lie`: traits; SO(3) and SE(3) over `fs-ga`; the PO-MARROW-01 fixture |
| BD-011 | M0 | U13 | `frob-lie`: Cayley and exponential retractions with derivatives; laws for SO(3) and SE(3); the coadjoint convention of §11.2 |
| BD-012 | M0 | U13 | `frob-lie`: SO(2) as a unit complex number with a turn counter |
| BD-013 | M0 | U21a | Shared dialect prelude v0 for CUDA and Metal; the dialect lint |
| BD-014 | M0 | U21b | `KernelBackend` trait, the kernel manifest with its stage signature, and the `batch-cpu` harness |
| BD-015 | M0 | U33a | INT-CUDA-01 and `spark/frob-spark-cuda` v0: driver load, NVRTC compile, cache, launch |
| BD-016 | M0 | U33a | INT-FT-01 and `spark/frob-spark-metal` v0 over the `ft-kernel-metal` gateway, with FrankenTorch's `block` patch repeated at the annex root |
| BD-017 | M0 | U21b | `spark.probe`: the IEEE probe kernels and report |
| BD-018 | M0 | U21a | `frob-kir` v0: straight-line programs, common-subexpression elimination, Rust and dialect emitters, the dimension checker |
| BD-019 | M0 | U33a | INT-FSYM-01 and `derive/frob-derive` v0: derive `K-cay-step` and `K-normalize` |
| BD-020 | M0 | U21b | GATE-SPARK-00 and INT-FSIM-11: one dialect source for `K-cay-step` on `batch-cpu`, CUDA and Metal |
| BD-021 | M0 | U25b | `frob-lemma` v0: claims and instance certificates for T-01, T-03 and T-04 on the M0 kernels, checked by FrankenSymPy |
| BD-022 | M0 | U32a | `frob-verbs` v0 and the `frob` skeleton: registry loader, the envelope with claim product and flags, exit codes including 8 (accepted and still running), `doctor`, `ca… |
| BD-023 | M0 | U32a | INT-FMCP-01 and upstream work item FD-FMCP-01: fastmcp_rust brought onto T in its own repository (its toolchain file and, where T requires it, its declared `rust-versi… |
| BD-024 | M0 | U29a | `sim.run --tier referee` for the built-in G1; digest and T0 telemetry to the ledger |
| BD-025 | M0 | U38b | `frob-conformance` v0: tiers G0 to G5 and the negative-control registry |
| BD-026 | M0 | U25b | `frob-cert` with an empty dependency table and its own stable toolchain file: canonical JSON reader, BLAKE3, exact integer and rational arithmetic, outward-rounded int… |
| BD-027 | M0 | U25b | `.frobcert` bundles for instance certificates: the writer in `frob-lemma` against the schema, the `poly-identity` and `rational-ldlt` checkers in `frob-cert`, and `pro… |
| BD-028 | M0 | U33b | INT-FT-02: resident buffers and device completion receipts through the Metal gateway |
| BD-029 | M0 | U21a | `spark.fingerprint`: the hardware fingerprint of both machines (`frob.fingerprint/1`) |
| BD-030 | M0 | U21b | Exact-integer kernel class probe: `K-x-pred` and `K-g-pred` with exact fallback on the docking-boundary screen of 65,536 rows |
| BD-031 | M0 | U33c | INT-TOOLCHAIN-01: the matrix of §33.12 over the four nightlies in play, the choice of T and its record in `franken-stack.lock` and `rust-toolchain.toml`, the `fs-qty` … |
| BD-032 | M0 | U33b | INT-FSIM-02 and INT-FSIM-03: first admission tests of the `fs-ga` Lie façades and of `fs-mbd` articulated dynamics |
| BD-033 | M0 | U33b | Foundation-defect admission INT-FSIM-05: the `planar_drive` near-straight case, and the independent chord formula in `frob-conformance` |
| BD-034 | M0 | U33b | Foundation-defect admission INT-FSIM-06: three PPO probes (the clip branch for both advantage signs, the critic gradient from the current output, packed sequences acro… |
| BD-035 | M0 | U33b | Foundation-defect admission INT-FSIM-04 and INT-FSIM-07 (refusal half): the fixed-witness sustained step and the contact capacity, through the airlock |
| BD-036 | M0 | U33a | Foundation-defect admission INT-FNP-01 and INT-FNP-02: the typed stride converter at the interchange boundary |
| BD-037 | M0 | U18a | The upstream work items of Appendix F opened as beads labelled `upstream` in this graph and mirrored in the sibling trackers (FD-FSIM-10 to -16, FD-FNP-01 and the othe… |
| BD-038 | M0 | U33c | INT-FSIM-18: the worker-process fallback exercised before it is needed (`frob-fsim-worker`, the `remote` mode of the airlock) |
| BD-039 | M0 | U39a | The incumbent harness as an out-of-process oracle: pinned revisions, the strongest warm configurations, the disclosure block |
| BD-040 | M0 | U36 | Parity baseline on `trj`: the incumbent phase ledger for cold start, physics and full step on workloads Q and H, time to threshold on one and two cards, and edit to fi… |
| BD-041 | M0 | U38a | The plan-time pilot for the cart docking closed form, re-run from the reproduction bundle with two gains and with three |
| BD-042 | M0 | U23a | The plan-time pilot for the static stance family, re-run from the reproduction bundle |
| BD-043 | M0 | U25d | The remaining plan-time pilots of §25.22, re-run and recorded (probe, pad family, sensor aliasing, pusher and its timing ceiling, labels, decision reuse, shared bus, a… |
| BD-044 | M0 | U33a | M0 exit review by a fresh-context session that did not implement the work; KILL-FSIM and KILL-MCP decisions recorded as ADRs; the state of the two P0 upstream work ite… |
| BD-045 | M1 | U13 | `frob-lie`: SE(2), SE₂(3), SE₂₊ₖ(3), product groups |
| BD-046 | M1 | U13 | `frob-lie`: joint subgroups, Hervé classification (T-11), generated joint maps |
| BD-047 | M1 | U13 | Frame-typed transforms, twists and wrenches; the three typed maps between frames (§11.3) |
| BD-048 | M1 | U13 | `frob-algebra`: finite groups from generators, classes, characters, projectors (`algebra.irreps`) |
| BD-049 | M1 | U22a | `frob-algebra`: orbits, Burnside counts, subgroup lattices, wreath products (`algebra.orbits`) |
| BD-050 | M1 | U22a | `frob-algebra`: the `Cone` trait; orthant, second-order and semidefinite cones |
| BD-051 | M1 | U22a | `frob-algebra`: the inertia cone and the mass-support predicate |
| BD-052 | M1 | U22a | `frob-algebra`: posets, antichains, Kleene iteration |
| BD-053 | M1 | U21a | Exact predicates and guarded single precision as kernel classes: predicate typing, the select lint and error-bound derivation in `frob-kir` |
| BD-054 | M1 | U14a | `frob-design`: the term model, schema `frob.design/1`, the seven canonical-form rules, `DesignId` |
| BD-055 | M1 | U14a | `.fbot` fields: fills with shell thickness and no default fill, budgets, levers, locks, the power bus, sensors, payloads, symmetry assertions |
| BD-056 | M1 | U41 | Constructors: `attach`, `chain`, `serial`, `mirror`, `radial`, `scale`, `replace`, `closure` |
| BD-057 | M1 | U14a | The S-expression view and `design.view` |
| BD-058 | M1 | U41 | Typed edits with preconditions, inverses and declared symmetry effects (`design.edit`) |
| BD-059 | M1 | U14a | Design patches and versioning: `design.patch`, schema `frob.design.patch/1`, expected parent, idempotency key, schema migration |
| BD-060 | M1 | U15b | `.fmodule`: the module format and its port contract (mechanical, electrical, thermal, signal and contact ports; parameters); `module.list`, `module.show`, `module.check` |
| BD-061 | M1 | U14a | `frob-model`: the kinematic functor, `RobotModel`, the referee lowering |
| BD-062 | M1 | U14a | Geometry and inertia functor: Regions, materials, the fill rule with shell moments as a difference of solids, primitive fitting with recorded error |
| BD-063 | M1 | U14b | Memoized incremental compilation, with the invalidation set returned by an edit |
| BD-064 | M1 | U24c | Design lints with ranked repairs as typed edits |
| BD-065 | M1 | U14b | `frob-io`: the in-house XML reader; URDF import with a loss report |
| BD-066 | M1 | U14a | MJCF import with a loss report; URDF and MJCF export with the reverse loss report |
| BD-067 | M1 | U24b | Oracle capture scripts; fixtures for reference robots with pin and date |
| BD-068 | M1 | U33b | INT-FSIM-09: admission of `fs-phs` interconnection, `fs-tribo` friction-law selection and `fs-matdb` material data |
| BD-069 | M1 | U15a | Actuator, battery and transmission cards: the `.fcard` schema, per-field provenance, the generic catalog; `actuator.list`, `actuator.show`, `actuator.check` |
| BD-070 | M1 | U15b | Actuator levels L0 and L1 on the referee |
| BD-071 | M1 | U15b | Actuator level L2: winding current, the thermal network, the battery |
| BD-072 | M1 | U15a | Winding temperature with temperature-dependent resistance, and thermal runaway (`actuator.thermal`) |
| BD-073 | M1 | U15a | The voltage limit and field weakening (`actuator.envelope`) |
| BD-074 | M1 | U15b | Transmissions: gear, belt, screw, linkage, cable and coupling matrices with loss laws |
| BD-075 | M1 | U15b | The power bus and the battery, with brownout events (`actuator.bus`) |
| BD-076 | M1 | U15a | Port energy-balance check on the referee |
| BD-077 | M1 | U14a | `module.substitute` and the invalidation contract of C-12 |
| BD-078 | M1 | U20a | The clock contract: rational periods, phase accumulators, the seven instants and the boundary ordering rule (`sense.schedule`) |
| BD-079 | M1 | U33b | `frob-contact`: collision primitives; contact sets from `fs-query`; INT-FSIM-08 |
| BD-080 | M1 | U18a | The referee step: FrankenSim's contact step on generated contact sets; compliant feet |
| BD-081 | M1 | U16a | The transactional referee step: the restore set and the commit fence with `commit_epoch` |
| BD-082 | M1 | U16a | Event location on rational base intervals, with coupled impulses per island |
| BD-083 | M1 | U16c | The retry ladder that changes numerical parameters only, and the refinement law for random inputs |
| BD-084 | M1 | U15b | Joint-level stiction, backlash and hard stops as rows in the referee solve |
| BD-085 | M1 | U18a | The contact known-answer battery on the referee |
| BD-086 | M1 | U18a | Thin-barrier and sill fixtures under the transactional step |
| BD-087 | M1 | U41 | The free-body variational step as an oracle [S] |
| BD-088 | M1 | U16a | The dynamics known-answer battery |
| BD-089 | M1 | U16a | Contact-free dynamics parity against the pinned oracle |
| BD-090 | M1 | U28b | `frob-chronicle`: the ledger schema, `.frun` episodes, telemetry levels T0 to T2 with streams and channel kinds |
| BD-091 | M1 | U29a | The chunk protocol on the CPU: leases, ranges, the committed frontier |
| BD-092 | M1 | U29a | The energy ledger on the referee, with stored heat, separate braking modes and the first offending event |
| BD-093 | M1 | U33b | Witness migration in the transactional step |
| BD-094 | M1 | U24a | Failure taxonomy with outcome kinds, first-cause ordering and budgeted summaries (`telemetry.summary`, `telemetry.failures`, `telemetry.energy`, `telemetry.query`) |
| BD-095 | M1 | U29a | The artifact graph (`evidence.trace`) and the per-bead log `frob.beadlog/1` |
| BD-096 | M1 | U33a | Simulation testing of the ledger and the submit path under asupersync's lab runtime |
| BD-097 | M1 | U29a | `artifact.read`, `artifact.list`, `artifact.export`, `run.show`, `run.list` |
| BD-098 | M1 | U20a | Packet queues and delivered observations, with the queues inside every checkpoint |
| BD-099 | M1 | U20b | Proprioception on the referee: IMU with lever arm, encoders, sensor cards (`sense.profile`) |
| BD-100 | M1 | U28a | A-02 mass roll-up and A-08 static stability |
| BD-101 | M1 | U28a | A-01 mobility |
| BD-102 | M1 | U20b | A-03 symmetry: Γ_mech and Γ_obs with breakers and near-symmetry repairs (`analyze.symmetry`) |
| BD-103 | M1 | U15b | A-10 co-design closure and A-11 actuator envelope with the temperature-dependent steady state |
| BD-104 | M1 | U19a | A-13 self-collision and clearance |
| BD-105 | M1 | U33a | A-05 workspace, A-06 singularities, and the derived leg IK kernel; INT-FSYM-02 |
| BD-106 | M1 | U28a | A-07 growth vector and maneuverability |
| BD-107 | M1 | U28a | A-12 modes with symmetry block-diagonalization |
| BD-108 | M1 | U28a | A-20 energy-budget feasibility, A-21 deadline and latency closure, A-22 sampled-loop stability, A-23 shared-bus schedule |
| BD-109 | M1 | U14b | The first archetypes: `quad-mammal`, `humanoid`, `diff-drive`, `mecanum` (`design.templates`) |
| BD-110 | M1 | U28a | `design.check` stages 1 to 6: aggregation, ranked repairs as typed edits, per-stage timing |
| BD-111 | M1 | U33b | The memo graph in front of analyses, with shadow recompute; INT-FSIM-16 |
| BD-112 | M1 | U15a | `explain.run` and `explain.design` |
| BD-113 | M1 | U28a | Lineage basics: the lineage graph over patches, `design.diff`, `design.blame` |
| BD-114 | M1 | U22a | The invariant filter (`control.estimate`) |
| BD-115 | M1 | U41 | The small dense cone-program solver, with end states and dual certificates |
| BD-116 | M1 | U22a | Legged baseline controller |
| BD-117 | M1 | U14b | Wheeled baseline controller |
| BD-118 | M1 | U22c | `control.synth` with three-outcome reporting, and `control.show` |
| BD-119 | M1 | U20b | The causal-policy test (`sense.causal-test`) |
| BD-120 | M1 | U20a | A-17 observability: the linearized Gramian modulo gauge on the CPU (`sense.observability`) |
| BD-121 | M1 | U24a | `frob-spec`: the requirement language core, with unit-normalized predicates, temporal operators and three evaluators |
| BD-122 | M1 | U24c | The task file `.ftask`: terms, requirements, randomization; `task.new`, `task.show`, `task.list`, `task.check`, `task.verdict` |
| BD-123 | M1 | U24b | Fixture 1, `fixture:cart-sill-dock`: bodies, motor, sill, the analytic layer and the creep rigs (`task.fixture`) |
| BD-124 | M1 | U33b | The two study types on the CPU: the anytime-valid e-process and the fixed-n exact study with its reachability pre-analysis (`study.plan`, `study.submit`, `study.status… |
| BD-125 | M1 | U24c | Task families of M1: stand and recover, threshold delivery, timed docking, each with a requirement, labelled traces and a reference scorecard (`eval.run`, `eval.scorec… |
| BD-126 | M1 | U24b | `.fmission`: the mission schema and its admission (`mission.new`, `mission.check`), with the quantifier order echoed |
| BD-127 | M1 | U25a | Admission of the second linear-program solver (`fsci-opt` `linprog`) and of the airlock entries `fs-ivl` and `fs-sos` for mission work |
| BD-128 | M1 | U25b | `frob-mission` core: inner and outer design sets, cells, polarity, the proof rules on finite and polyhedral fragments, plain proof objects |
| BD-129 | M1 | U25c | The docking family: the finite game over causal histories with one velocity packet, and its scenario-tree comparator |
| BD-130 | M1 | U25a | The stance family: body–load feasibility with vertex witnesses and dual exclusions, in lifted and projected form |
| BD-131 | M1 | U25a | The pusher battery: per-build and shared queries on one plant |
| BD-132 | M1 | U12 | `frob-cert` checkers for the mission proof kinds (`lp-primal`, `lp-farkas`, `finite-game`, `alias-pair`, `partition`, `interval-box`, `trace-witness`) |
| BD-133 | M1 | U25b | Mission bundles: the writer in `frob-lemma` for inner cells and exclusions; `prove.bundle`, `prove.verify`, `prove.list`; `frob-verbs` runs the checker before an answe… |
| BD-134 | M1 | U20a | The late-packet alias witness for the docking mission (`sense.alias` on enumerated worlds) |
| BD-135 | M1 | U25a | `mission.synthesize`, `mission.refute`, `mission.distinguish`, `mission.show`, with budget stops |
| BD-136 | M1 | U25b | The mission mutation corpus and the checker-independence campaign |
| BD-137 | M1 | U32a | The M1 verbs with schemas, examples and refusal lists |
| BD-138 | M1 | U32a | MCP tools generated for the M1 verbs in task clusters of at most seven, with `session.open`, `session.show`, `session.focus`, `session.close` |
| BD-139 | M1 | U30b | The `reality:` line on every report, and its negative battery |
| BD-140 | M1 | U25a | The first mission slice driven by a fresh agent through the CLI and MCP |
| BD-141 | M1 | U34 | The format registry rows and round trips for the formats that exist at M1 (`.fbot`, `.fmodule`, `.fcard`, `.ftask`, `.fmission`, `.frun`, `.frobcert`) |
| BD-142 | M1 | U36 | Parity baseline completed: incumbents on this Mac (CPU MuJoCo at 1, 24 and 36 threads; an incumbent with a Metal path) and native settings on `trj` |
| BD-143 | M1 | U39b | The incumbent design-loop phase ledger on `trj`: one agent-driven round timed by phase, cold and warm separated, with the ceilings of §2 recomputed from the measured s… |
| BD-144 | M1 | U14b | Phylum pilot with no GPU code: padding overhead and bucket count of the first archetypes and of 1,000 generated designs under the bucket set |
| BD-145 | M1 | U34 | Pre-registration of the rank-agreement experiment: design set, strata, the τ thresholds 0.7 and 0.5 fixed before data, and the compute request under OPEN-SPARK-2 |
| BD-146 | M1 | U38b | `frob-conformance` for M1: known answers against pinned oracles, the counterexample catalogue seeds, the `smoke` and `fixtures` batteries |
| BD-147 | M1 | U39a | M1 exit review by a fresh-context session that did not implement the work; the schedule gate (BET-12, §40.5) |
| BD-148 | M2 | U29a | Packed lowering of `RobotModel`: depth levels, path tables, capacities, `ShapeClassId` |
| BD-149 | M2 | U36 | Derive and emit `K-fk-level` and `K-twist-level` |
| BD-150 | M2 | U16a | Derive and emit `K-crba-level` and `K-rnea-level` |
| BD-151 | M2 | U15b | `K-act-l1`, `K-act-l2`, `K-passive`, and the shared-bus reduction `K-act-bus` |
| BD-152 | M2 | U21a | Utility kernels: `K-scan`, `K-sort`, `K-reduce`, `K-seg-reduce`, `K-compact` |
| BD-153 | M2 | U41 | `K-jac-rows`, `K-cone-setup`, `K-cone-eval` for CL-1 |
| BD-154 | M2 | U18a | The four-dimensional contact cone (normal, two tangential, spin) in the `Cone` trait and the cone kernels |
| BD-155 | M2 | U15b | `K-joint-rows`: stiction, backlash and hard stops on the batched tiers |
| BD-156 | M2 | U18b | `K-scale-diag`, `K-hess`, batched dense `K-chol`, exact `K-linesearch`, `K-refine` |
| BD-157 | M2 | U41 | Articulated `K-cay-step`, `K-normalize`, `K-anchor` |
| BD-158 | M2 | U18a | `K-geom-pose`, `K-np-prim`, `K-bp-sort`, `K-bp-pairs`: poses of primitives, primitive pairs, the deterministic broad phase |
| BD-159 | M2 | U16a | The batched step on `batch-cpu` in `f64` and `f32`, with lockstep masks and capacity flags |
| BD-160 | M2 | U41 | The tile program on `fs-exec` with fixed-slot reductions |
| BD-161 | M2 | U18a | The `strict` profile of `batch-cpu`: the transactional, event-located step in `f64` on our own kernels |
| BD-162 | M2 | U19b | `frob-scene`: the `.fscene` scene-pack format, bricks, interpolation bounds, truncated fields with a stated range |
| BD-163 | M2 | U19c | The scene compiler through the airlock, with analytic overlays (`scene.compile`, `scene.show`, `scene.list`) |
| BD-164 | M2 | U19c | The brick pool with one dense directory per scene variant |
| BD-165 | M2 | U18a | Terrain generators with a difficulty parameter (`scene.gen`) |
| BD-166 | M2 | U41 | `K-np-sdf` with overlays and the pooled directory; `K-np-thin` |
| BD-167 | M2 | U18b | Rolling and torsional rows; interface cards by material pair and condition (`contact.laws`, `contact.probe`) |
| BD-168 | M2 | U18b | The contact battery on batched tiers; the tunneling audit; the capacity audit (`contact.audit`, `contact.capacity`) |
| BD-169 | M2 | U16a | Per-world parameter variants (level P-A): model, scene and task slots with a parameter row |
| BD-170 | M2 | U17b | Model variants inside padded capacities (level P-B): one model table per slot, masks, `K-variant-gather` |
| BD-171 | M2 | U21b | Size buckets and `phylum.plan` |
| BD-172 | M2 | U21a | CUDA backend: buffers, the step plan (`frob.stepplan/1`), launch-graph capture, stream-ordered pools, receipts |
| BD-173 | M2 | U21a | The device arena, leases and the binary kernel cache (`spark.cache`, `spark.precompile`) |
| BD-174 | M2 | U21b | Metal backend: packed arguments, batch chains, snapshot blits, receipts |
| BD-175 | M2 | U21a | Device faults and cancellation |
| BD-176 | M2 | U21b | `spark.check` and the divergence ledger |
| BD-177 | M2 | U35 | The decision-level equivalence test under class D3 |
| BD-178 | M2 | U21a | The placement planner with requested-versus-actual receipts per stage, and strict backends (`spark.place`, `frob.placement/1`) |
| BD-179 | M2 | U21b | Anchored positions (cell side 8 m, offset in [−4, 4) m), the precision battery and the conditioning certificate |
| BD-180 | M2 | U41 | Randomness by address: `K-x-philox` and addressed draws |
| BD-181 | M2 | U35 | The batch-composition invariance test: co-tenancy, slot permutation, bucket capacity, shards |
| BD-182 | M2 | U13 | Guarded predicates on devices over the contact battery |
| BD-183 | M2 | U21b | Variants on devices: parameter variants, topology variants on one captured graph, a mixed batch of three body plans, admission of a variant into a free slot |
| BD-184 | M2 | U17b | Admission and edit-to-first-step latency on the warm, cool and cold paths (`phylum.sweep`) |
| BD-185 | M2 | U14b | The control arm of the head-to-head: nine padded per-family templates with lockable joints and per-design parameter tables |
| BD-186 | M2 | U17b | The mixed-batch head-to-head experiment with arms A, B, C and D, as a fixed-n study |
| BD-187 | M2 | U20b | Ray kernels; height scan, depth, lidar; proprioception with noise on batched tiers |
| BD-188 | M2 | U20a | Camera timing: exposure integration, rolling shutter and lidar beam instants |
| BD-189 | M2 | U20a | The clock contract and checkpointed packet queues on every tier |
| BD-190 | M2 | U20a | The docking fixture in exact arithmetic on every backend |
| BD-191 | M2 | U24a | Stills and pixel inspection (`telemetry.stills`, `telemetry.inspect-pixel`, `telemetry.export-html`) |
| BD-192 | M2 | U23a | `VecEnv` with the corrected semantics E-1 to E-9: explicit reset, the preserved final observation, termination against truncation, typed spaces, access classes, snapsh… |
| BD-193 | M2 | U23a | Device-resident tensor views with the stride converter and ownership epochs |
| BD-194 | M2 | U24b | Declarative task specifications with requirement terms, time-aware rewards, physical discounting and the likelihood of the applied action |
| BD-195 | M2 | U24a | The settled-pose reset cache |
| BD-196 | M2 | U23a | On-device networks and optimizer (`K-gemm`, `K-act-fn`, `K-adam`); gradient parity with FrankenTorch |
| BD-197 | M2 | U23a | PPO under rules P-1 to P-11 |
| BD-198 | M2 | U23a | PPO training of the reference quadruped's flat velocity task, with negative controls; `train.start`, `train.status`, `train.stop`, `train.export` |
| BD-199 | M2 | U36 | Sharding across both RTX 4090, and multi-device PPO under the global normalization rule |
| BD-200 | M2 | U41 | Randomization from cards, drawn by address |
| BD-201 | M2 | U24a | Task families of M2: velocity tracking and terrain traversal, with reference-robot scorecards |
| BD-202 | M2 | U24b | Fixture 2, `fixture:quad-split-grip` |
| BD-203 | M2 | U29a | In-kernel digests (`K-x-digest`), per-stage digests, snapshots at the fence (`K-commit`, `K-snapshot`), replay (`sim.replay`) |
| BD-204 | M2 | U29a | Teleport with step and stage localization (`sim.replay --tier`) |
| BD-205 | M2 | U27a | Forks from snapshots by indexed copy, and their classification into the five classes of §27.4 (`run.fork`) |
| BD-206 | M2 | U29a | The first-divergence search over the digest chain, in exact and tolerance modes (`telemetry.divergence`) |
| BD-207 | M2 | U29a | Telemetry on devices: the reservoir by address, deterministic integer histograms (`K-x-hist`, `telemetry.histogram`), the cost of T0 to T2 |
| BD-208 | M2 | U29a | The energy ledger on batched tiers (`K-ledger-step`) and the audit for energy created by the simulator (`fidelity.energy`) |
| BD-209 | M2 | U29a | Retention and collection with tombstones (`store.status`, `store.collect`) |
| BD-210 | M2 | U29a | The crash-consistent job protocol on devices: chunk publish, leases, cancel, restart; `job.list`, `job.status`, `job.wait`, `job.find`, `job.cancel` |
| BD-211 | M2 | U32a | Atomic idempotency binding, pagination pinned to a committed frontier, integers above 2⁵³ as strings |
| BD-212 | M2 | U16b | The standard battery, frozen; the fidelity ledger with its frozen envelopes (`fidelity.ledger`) |
| BD-213 | M2 | U16a | The referee's own coverage report per robot class (`fidelity.coverage`) |
| BD-214 | M2 | U16b | The sampled background referee (`fidelity.shadow`): address-hashed sampling, snapshots, one-step defects, the coverage figure on every receipt |
| BD-215 | M2 | U20b | The noninterference test, and the causal-policy test on every tier |
| BD-216 | M2 | U36 | The performance harness: unit ordering, the paired lower bound, the disclosure block |
| BD-217 | M2 | U21a | Profile, optimize one lever at a time, measure, disclose: the throughput parity rows |
| BD-218 | M2 | U23b | Training-time parity on one RTX 4090 |
| BD-219 | M2 | U36 | The `sim.estimate` cost and memory model |
| BD-220 | M2 | U33a | Memoized episodes under class D2 on one fingerprint; INT-FSQL-02 |
| BD-221 | M2 | U21a | The derivation toolchain measured against KILL-KIR; INT-FJAX-01 only if the profile asks for rewrite rules |
| BD-222 | M2 | U32a | The M2 verbs with schemas, examples and refusal lists |
| BD-223 | M2 | U38b | `frob-conformance` for M2: the `standard` battery wiring, metamorphic relations, chaos tests on devices |
| BD-224 | M2 | U40 | M2 exit review by a fresh-context session that did not implement the work; kill decisions as ADRs |
| BD-225 | M3 | U14b | The interior grammar mechanism and the structural proxies of the warehouse and the commercial kitchen, with doors, ramps, thresholds, stairs, glass and mirror regions |
| BD-226 | M3 | U14b | Structural proxies of the home, apartment, office, manufacturing plant and retail store |
| BD-227 | M3 | U14a | Hazard lists: `registries/hazards/<archetype>.toml` with dimension ranges, mechanisms, provoked failures and provenance (`scene.hazards`) |
| BD-228 | M3 | U19a | Scene acceptance (`scene.accept`) |
| BD-229 | M3 | U19a | Optical and friction properties as separate maps; fixtures and movable objects |
| BD-230 | M3 | U18a | The film-friction law: Habitat's thickness state and Traction's cone row (`K-film`) |
| BD-231 | M3 | U41 | Rigid proxies: a rug as a free slab, a cord as a fixed ridge or a free rod, clutter as rigid bodies |
| BD-232 | M3 | U18a | The mesh path with enclosures and a bounding-volume hierarchy; thin-feature detection |
| BD-233 | M3 | U19b | Clearance complexes `.fclear`: two-sided cells, path witnesses and component cuts; enclosure tests (`contact.clearance`); A-18 |
| BD-234 | M3 | U19a | Feasibility tags for generated scenes, from the clearance complex |
| BD-235 | M3 | U19c | Train and test hygiene: `scene.split`, building-level holdouts, mirrored layouts and edited variants |
| BD-236 | M3 | U19c | The world bank (`scene.bank`): resident scenes under the admission cap, a change of scene at reset |
| BD-237 | M3 | U20a | Optical failure modes: mirror, glass, steam, pattern dropout, with a reason on every masked sample |
| BD-238 | M3 | U20a | Sensor cards with failure modes and per-field provenance |
| BD-239 | M3 | U20a | The empirical observability Gramian on batched tiers |
| BD-240 | M3 | U20a | The alias-witness search over admitted world families (`sense.alias`) |
| BD-241 | M3 | U20b | Γ_obs per robot and task, with the equivariance test (`sense.symmetry`) |
| BD-242 | M3 | U20b | Sensor calibration as a least-squares pose graph (`sense.calibrate`) |
| BD-243 | M3 | U33b | A-09 connection curvature, A-14 sensor coverage, A-16 sensitivity; INT-FSIM-10 |
| BD-244 | M3 | U14b | Preflight stages 7 (place) and 8 (information) |
| BD-245 | M3 | U14a | Contact laws for the remaining archetypes on all tiers: `belt`, `skid`, `pad`, `roller-wheel`, `wheel-rolling` |
| BD-246 | M3 | U14b | The remaining fourteen archetypes |
| BD-247 | M3 | U14b | Baseline controllers for every archetype: multi-legged, biped balance, steered, omnidirectional, tracked, hybrid, arm on a base |
| BD-248 | M3 | U28b | The design corpus: about 200 designs with frozen labels, planted defects, planted winners and planted single causes |
| BD-249 | M3 | U22a | Validation of baseline synthesis on the corpus |
| BD-250 | M3 | U28a | Validation of static analysis on the corpus |
| BD-251 | M3 | U22a | Control programs with layer records, infeasibility certificates and fallback events |
| BD-252 | M3 | U22a | The sampling-based predictive controller as a fallback (`control.mpc`; `K-fork`, `K-cost`, `K-weight`) |
| BD-253 | M3 | U24a | Task families of M3: interior navigation, payload transport, low-friction crossing, tight-space maneuvering |
| BD-254 | M3 | U24b | Metrics, scorecard accounting and the batteries `standard`, `full`, `hard`, `covering` |
| BD-255 | M3 | U23a | Requirement verdicts on batched tiers: streaming past-time monitors, `task.verdict`, `task.settle`, `frob.verdict/1` |
| BD-256 | M3 | U24a | The requirement-writing test with a fresh agent |
| BD-257 | M3 | U24a | The requirement reward-term pilot under three arms |
| BD-258 | M3 | U37 | Capabilities, namespaces and many agents: scopes, session budgets, the path jail |
| BD-259 | M3 | U24b | The blinded qualification lane: sealed world clusters, pre-committed requests, receipts, the error budget α_i = α / [i·(i+1)] (`eval.qualify`, `eval.ledger`) |
| BD-260 | M3 | U24a | The 500-retry gate on no-signal worlds |
| BD-261 | M3 | U24b | The confirmation firewall: capability scopes `qual:admin` and `qual:run`, the sealed-field audit |
| BD-262 | M3 | U28a | Anytime-valid paired comparison on devices (`eval.compare`, `study.compare`, `explain.compare`) |
| BD-263 | M3 | U16b | The trust ladder and the promotion rule with the DC-01 table (`eval.promote`) |
| BD-264 | M3 | U16b | The tier-correction ledger: control variate and doubly robust correction |
| BD-265 | M3 | U16b | Rank agreement between tiers on the 40-design set (`eval.rank-agreement` on a tier pair) |
| BD-266 | M3 | U16b | The model-form rank check (`fidelity.modelform`) |
| BD-267 | M3 | U23b | Exploitation detectors (`eval.detect`) |
| BD-268 | M3 | U16b | The background referee at its gate |
| BD-269 | M3 | U35 | Determinism-class licences and the labeled D4 training profile |
| BD-270 | M3 | U26a | The Nemesis search space and the legality set (`frob.legality/1`), with the hazard registries as the scene-edit vocabulary |
| BD-271 | M3 | U26b | The solvability oracle (`nemesis.oracle`) |
| BD-272 | M3 | U26a | The artifact guard |
| BD-273 | M3 | U26a | The falsification score and the search (`nemesis.search`) |
| BD-274 | M3 | U26b | Shrinking to a minimal counterexample world `.fmcw` (`nemesis.shrink`) |
| BD-275 | M3 | U15a | The hard-world set (`nemesis.hardset`, `frob.hardset/1`) |
| BD-276 | M3 | U26a | Rare failures, certification lane: bounded-weight estimation with the defensive mixture (`nemesis.bound`) |
| BD-277 | M3 | U26a | Rare failures, exploration lane: splitting |
| BD-278 | M3 | U26b | Scene-set coverage by submodular selection (`nemesis.cover`) |
| BD-279 | M3 | U27b | Interventions and the placebo fork (`autopsy.intervene`) |
| BD-280 | M3 | U27b | Interventional contrasts with flip rates, in double precision (`autopsy.attribute`) |
| BD-281 | M3 | U27a | Coalitions and budgeted Shapley summaries |
| BD-282 | M3 | U27a | The decision model for attribution, with abstention (DC-06) |
| BD-283 | M3 | U27a | Design parameters re-run from the episode start (`autopsy.rerun`) |
| BD-284 | M3 | U27b | Time-travel debugging as an agent workflow (`autopsy.repair`, `autopsy.show`) |
| BD-285 | M3 | U17a | Campaign quotienting: the similarity canonical form, orbit representatives, the mission-observable digest (`phylum.quotient`) |
| BD-286 | M3 | U17b | The invalidation contract for reuse across designs (`phylum.reuse`) |
| BD-287 | M3 | U17a | The morphology generator with `GeneratorId` and `DesignSplit` (`phylum.generate`) |
| BD-288 | M3 | U25a | Monotone transport of verdicts, with its falsification harness |
| BD-289 | M3 | U25b | The region-certificate pilot of §25.6 on the 12-parameter box |
| BD-290 | M3 | U25b | Alias and jaw fixtures; `mission.refute` on the blind payload task |
| BD-291 | M3 | U25c | Probe and instrument design (`mission.distinguish`, `scene.probe-request`) |
| BD-292 | M3 | U25a | The revalidation engine behind `result.revalidate` and `mission.revalidate`: dependency grades 1 and 2, polarity |
| BD-293 | M3 | U25b | Separation of two designs by a minimized world pair (`mission.separate`) |
| BD-294 | M3 | U25e | Checker independence repeated over every proof kind in use at M3 |
| BD-295 | M3 | U32a | The daemon: persistent environments with slot-level episode dispatch |
| BD-296 | M3 | U32a | Size classes, shortest estimated work and preempt-resume, with the DC-09 table |
| BD-297 | M3 | U32a | Estimates and budgets: calibrated intervals per stratum |
| BD-298 | M3 | U32a | The full MCP surface: seventeen task clusters, the bounded synchronous wait, progress and cancellation, resources, prompts |
| BD-299 | M3 | U32a | The three failure classes on both surfaces |
| BD-300 | M3 | U33a | The dashboard `frob-tui` (INT-FTUI-01) and the viewer overlays (INT-F3D-01) |
| BD-301 | M3 | U23b | The Gymnasium-compatible adapter `bindings/frob-gymnasium` |
| BD-302 | M3 | U25e | `report.design`, `report.run`, `report.campaign`, `report.mission` |
| BD-303 | M3 | U37 | The no-authorization flags on every exporting verb |
| BD-304 | M3 | U37 | Imported text as data: the `x-frob-untrusted` mark through every verb that echoes imported text |
| BD-305 | M3 | U37 | Private places and licences: the private mark, licence joins, `scene.purge` |
| BD-306 | M3 | U37 | Supply-chain checks: one asupersync package per workspace, committed lockfiles, kernel-cache integrity |
| BD-307 | M3 | U37 | Importer fuzz targets and limits |
| BD-308 | M3 | U37 | The claim product at the boundary |
| BD-309 | M3 | U28c | Decision-core tables DC-01, DC-09 and DC-10 in `registries/cores/` (`cores.show`) |
| BD-310 | M3 | U28a | The rank-agreement and ladder pilots on an incumbent engine or on `batch-cpu` |
| BD-311 | M3 | U30a | The published-anchor comparison under the Parity label |
| BD-312 | M3 | U32a | The M3 verbs with schemas, examples and refusal lists |
| BD-313 | M3 | U32a | `robot-docs`, the skill file and the two worked sessions; the fresh-agent test |
| BD-314 | M3 | U39a | The partial ten-fold gate, protocol: sealed legged and wheeled briefs, strata, the decision rule with its reachable verdicts, the ceilings, the adjudicating evaluator |
| BD-315 | M3 | U39a | The partial ten-fold gate, incumbent arm: the strongest composed rival at pinned versions, warm, driven through its own tools and the adapter |
| BD-316 | M3 | U25a | The partial ten-fold gate, our arm: fresh agents on the full MCP surface |
| BD-317 | M3 | U39b | The partial ten-fold gate, adjudication and record |
| BD-318 | M3 | U38b | `frob-conformance` for M3: the `hard` and `covering` batteries, the planted-defect registries of Nemesis and Autopsy, the hostile-input corpora of §37 |
| BD-319 | M3 | U40 | M3 exit review by a fresh-context session that did not implement the work |
| BD-320 | M4 | U23b | Family policies per morphology class: link tokens, the mapping table (`frob.policy/1`), pretraining on a `GeneratorId` with a `DesignSplit` (`train.family`) |
| BD-321 | M4 | U23a | The family-policy experiment on held-out parametric designs, with the fine-tuning rungs |
| BD-322 | M4 | U23a | Inheritance along design edits (`train.inherit`, `policy.apply`) |
| BD-323 | M4 | U23a | Distillation from baseline, specialist and sampling-controller teachers (`train.distill`) |
| BD-324 | M4 | U23b | Pilots of the other learning algorithms of §23.22 against PPO at equal wall-clock |
| BD-325 | M4 | U13 | Equivariant linear maps by irreducible blocks under Γ_obs |
| BD-326 | M4 | U23a | The three-arm equivariance experiment |
| BD-327 | M4 | U22b | Symmetric controllers under stabilizer intersection (`control.symmetry`) |
| BD-328 | M4 | U23b | The rank-agreement experiment on our engine, per morphology class and task family (`eval.rank-agreement`; the model-form check is `fidelity.modelform`) |
| BD-329 | M4 | U28a | The ladder of rungs R0 to R5: the fixed halving table and the calibrated layer with DC-02 and DC-03 |
| BD-330 | M4 | U22a | An inspectable baseline plus a learned residual (`control.ladder`) |
| BD-331 | M4 | U22a | The sampling controller at its gate |
| BD-332 | M4 | U28b | The gait atlas and max-plus timing; A-04 (`gait.atlas`) |
| BD-333 | M4 | U33b | Gaits as hybrid orbits: return maps, saltation, Floquet multipliers (`gait.orbit`); INT-FSIM-13 |
| BD-334 | M4 | U22a | Labeled derivatives through events (`gait.gradient`) |
| BD-335 | M4 | U22a | Continuation of orbits in morphology, and orbit screening (`gait.continue`) |
| BD-336 | M4 | U22b | Skill artifacts `.fskill`, the exact layer (`skill.compile`, `skill.check`, `skill.show`) |
| BD-337 | M4 | U22b | The energy-budget safety tier `.fenergy`, with the exact-integer monitor `K-energy-monitor` and its proof kind (`shield.budget`) |
| BD-338 | M4 | U22b | Runtime shields with a backup controller (`shield.fit`, `shield.check`) |
| BD-339 | M4 | U22b | Shields on in-envelope and shifted worlds |
| BD-340 | M4 | U22b | The measured shield margin with its drift alarm (`shield.report`) |
| BD-341 | M4 | U41 | Optimization and racing (`opt.start`, `opt.status`) |
| BD-342 | M4 | U17b | The quality-diversity archive and maps per dispatch (`opt.archive`, `phylum.map`) |
| BD-343 | M4 | U28a | Lineage bisect and merge (`design.bisect`) |
| BD-344 | M4 | U41 | Registered predictions (`predict.register`, `predict.report`) |
| BD-345 | M4 | U28b | Screening audits: sampled discarded strata and the planted winner (`nemesis.audit`, applied by the cascade) |
| BD-346 | M4 | U28b | Rank robustness of finalists inside the cascade (A-19) |
| BD-347 | M4 | U28a | Revalidation grades 3 and 4 |
| BD-348 | M4 | U28a | The remaining analyses: A-24 task-wrench attainability, A-25 identifiability, A-26 regime atlas |
| BD-349 | M4 | U28a | Decision-core tables DC-02 to DC-08 and DC-11 |
| BD-350 | M4 | U28c | Composition of decision cores: the nine interference tests |
| BD-351 | M4 | U28a | Experiment suggestion by value of information (`experiment.suggest`) |
| BD-352 | M4 | U28a | The six-stage co-design cascade (`opt.cascade`, `report.campaign`) |
| BD-353 | M4 | U17b | Paired finite-difference design gradients (`phylum.gradient`) |
| BD-354 | M4 | U25a | The ceiling-before-speed check on every registered speed comparison |
| BD-355 | M4 | U25a | Mission queries over families on the GPU: the 65,536-row screen and a 1,024-design family batch under the exact-integer or guarded class |
| BD-356 | M4 | U17a | Design-family lanes (level P-C) with the speedup bound 1/(1 − f + f/r + h) |
| BD-357 | M4 | U25b | Quotients, cuts and cell refinement in family search |
| BD-358 | M4 | U25e | Region certificates on small sub-boxes |
| BD-359 | M4 | U25c | Product-system dominance |
| BD-360 | M4 | U25d | The co-design fixed-point lane of `frob-synth` (`mission.synthesize` with strategy `codesign`) |
| BD-361 | M4 | U25d | Bounded exact linkage synthesis through the derivation subprocess, emitting a `.fmodule` (strategy `linkage`) |
| BD-362 | M4 | U25d | Trajectory optimization with design variables (strategy `refine`) |
| BD-363 | M4 | U25c | Label queries on supplied command sets |
| BD-364 | M4 | U12 | Mission bundles for family answers, re-checked on a second machine |
| BD-365 | M4 | U18b | Tyre laws, tracks and hybrids on all tiers |
| BD-366 | M4 | U18b | Soil mechanics: plate sinkage, drawbar pull, ruts as per-world mutable state (`contact.soil`) |
| BD-367 | M4 | U18a | The contact response batch with per-column bounds and reroutes (`contact.response`) |
| BD-368 | M4 | U24a | Moving surfaces and the task families of M4: conveyors, stair and curb climbing for wheels and tracks, reach and carry |
| BD-369 | M4 | U19b | The retrofit catalog `registries/retrofits.toml` with priced kits, and `scene.retrofit` on a `.fmcw` |
| BD-370 | M4 | U21c | The fleet scheduler across the two machines (`fleet.status`, `fleet.plan`) |
| BD-371 | M4 | U21a | Specialized kernels for survivors |
| BD-372 | M4 | U32a | The one-call pipeline `solve.run`, which calls `opt.cascade` |
| BD-373 | M4 | U32a | The mission session with a fresh agent |
| BD-374 | M4 | U28b | The agent-surface A/B on the design benchmark |
| BD-375 | M4 | U27a | Autopsy against logs and `explain` alone, with agents on the planted-defect designs |
| BD-376 | M4 | U32a | The M4 verbs with schemas, examples and refusal lists |
| BD-377 | M4 | U38b | `frob-conformance` for M4: the calibration sets, the frozen 30-amendment campaign, the seeded cascade population |
| BD-378 | M4 | U40 | M4 exit review by a fresh-context session that did not implement the work |
| BD-379 | M5 | U12 | Certificate coverage for derived kernels |
| BD-380 | M5 | U33a | INT-FLN-01 and INT-FLN-02: the Lean project under `lean/` and the `fln` checker on a Linux host |
| BD-381 | M5 | U12 | Lean theorem cards as sealed capsules (proof kind `lean-capsule`) |
| BD-382 | M5 | U12 | The reflection pilot: Farkas and rational LDLᵀ reflectors (`prove.reflect`) |
| BD-383 | M5 | U24b | Qualification campaigns replayed by `frob-verify` |
| BD-384 | M5 | U16a | The articulated variational integrator on the referee and the `strict` profile |
| BD-385 | M5 | U16a | The tree-operator route for larger mechanisms and limb port packets, as a pilot |
| BD-386 | M5 | U18a | The hard-contact law CL-H as a batch law on `batch-cpu` |
| BD-387 | M5 | U25a | Ground data admission: the dataset manifest (`frob.ground.dataset/1`), cells, sealed validation and holdout splits (`ground.ingest`) |
| BD-388 | M5 | U30b | The synthetic rehearsal: a `synthetic` dataset generated with planted parameters |
| BD-389 | M5 | U30b | Clock fitting |
| BD-390 | M5 | U30a | Identifiability analysis and excitation design (`ground.excite`) |
| BD-391 | M5 | U30b | Identification, actuators first, and card updates from logs (`ground.identify`) |
| BD-392 | M5 | U30b | Frozen predictions, sealed trials and holdout retirement (`ground.predict`, `ground.score`) |
| BD-393 | M5 | U30b | Conformal reality bands per calibrated cell, with the DC-07 table (`ground.band`) |
| BD-394 | M5 | U30b | Drift detection that retires a calibration |
| BD-395 | M5 | U30b | As-built ingestion |
| BD-396 | M5 | U30b | The replication kit (`ground.kit`) |
| BD-397 | M5 | U30b | The Reality lane's first record |
| BD-398 | M5 | U33a | Real-place ingestion (`scene.import`, `scene.fit`); INT-FDGR-01 |
| BD-399 | M5 | U19a | Asset-populated interiors with their licences |
| BD-400 | M5 | U31b | Load cases and spectra extracted from T2 telemetry (`foundry.loads`) |
| BD-401 | M5 | U33b | The beam-rung sizing check and A-15 structural margin (`foundry.size`); INT-FSIM-15 |
| BD-402 | M5 | U31b | Stress, buckling, modes and cumulative fatigue damage |
| BD-403 | M5 | U31b | Topology optimization of links (`foundry.optimize`) |
| BD-404 | M5 | U31b | Inertia fed back into the model as a typed edit, with the convergence test |
| BD-405 | M5 | U31b | Manufacturability checks (`foundry.check`) |
| BD-406 | M5 | U31b | Tolerance allocation driven by sensitivities (`foundry.tolerance`) |
| BD-407 | M5 | U31b | Bounded geometry export |
| BD-408 | M5 | U31b | The build packet `.fbuild` with its four flags (`foundry.export`) |
| BD-409 | M5 | U32a | The M5 verbs with schemas, examples and refusal lists |
| BD-410 | M5 | U34 | The format registry complete for all thirteen formats |
| BD-411 | M5 | U38b | `frob-conformance` for M5: the certificate mutation corpora, the synthetic rehearsal, the structure fixtures |
| BD-412 | M5 | U40 | M5 exit review by a fresh-context session that did not implement the work |
| BD-413 | M6 | U33b | Epic: robot-advected films; INT-FSIM-17 |
| BD-414 | M6 | U19b | Epic: payload slosh by an equivalent mechanical model |
| BD-415 | M6 | U19b | Epic: articulated proxies, a cord as a chain of links and a rug as hinged plates |
| BD-416 | M6 | U33a | Epic: Cosserat rods, cloth and coupled multiphysics islands on the CPU referee, for finalists only; INT-FTX-01 |
| BD-417 | M6 | U38a | Epic: soft soil at scale |
| BD-418 | M6 | U38a | Epic: tactile sensing and compliant pads |
| BD-419 | M6 | U15b | Epic: the rotor port for flight |
| BD-420 | M6 | U19b | Epic: RGB and splat scenes |
| BD-421 | M6 | U38a | Epic: moving people as kinematic actors on route graphs |
| BD-422 | M6 | U18a | Epic: gradients through contact |
| BD-423 | M6 | U31b | Epic: lifecycle compression (`foundry.lifecycle`) |
| BD-424 | M6 | U18a | Epic: the hard-contact law on GPU tiers |
| BD-425 | M6 | U25c | Epic: geometric phase for locomotion |
| BD-426 | M6 | U25d | Epic: symmetry at the instant of contact |
| BD-427 | M6 | U40 | M6 exit review by a fresh-context session that did not implement the work |
| BD-428 | M7 | U39a | Epic: the sealed suite: missions in named strata, sealed worlds including externally authored ones, the crossover reuse count |
| BD-429 | M7 | U39b | Epic: the tournament decision rule: the pooled ratio of restricted mean times, 24 mission families per cell for a finite-suite result and at least 32 for a population … |
| BD-430 | M7 | U39a | Epic: the incumbent arms: the strongest composed rival at pinned versions, cold and warm, including a short robust program over an incumbent engine |
| BD-431 | M7 | U39a | Epic: our arm: fresh agents on the released surface, cold and warm runs separated |
| BD-432 | M7 | U38a | Epic: neutral adjudication of accepted answers |
| BD-433 | M7 | U39a | Epic: publication of the result |
| BD-434 | M7 | U39a | M7 exit review by a fresh-context session that did not implement the work |
| BD-435 | M8 | U25c | Epic: facility co-design and multi-robot handoff |
| BD-436 | M8 | U25a | Epic: skill transport. `skill.transport` moves a skill artifact with a learned residual [M]; `mission.transport` moves a certified claim through an interface map with … |
| BD-437 | M8 | U25d | Epic: continuation-based synthesis (strategy `continuation`) |
| BD-438 | M8 | U31b | Epic: firmware export |
| BD-439 | M8 | U23b | Epic: geometric-algebra tree policies at scale |
| BD-440 | M8 | U22b | Epic: reachable-set tables for shields |
| BD-441 | M8 | U21c | Epic: the fused per-world step kernel |
| BD-442 | M8 | U40 | M8 exit review by a fresh-context session that did not implement the work |
