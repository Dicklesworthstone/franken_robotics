---
key: "EPIC-M1"
title: "Milestone M1, referee studio and first mission slice on the CPU: a useful, honest product with exact answers"
type: "epic"
priority: 1
milestone: "M1"
labels: ["m1", "delivery", "solid", "gate"]
parent: "EPIC-PROGRAM"
deps: []
needs: []
related: ["BD-147", "N40-11", "N40-13", "N40-18", "N40-04", "N40-05", "EPIC-M0", "EPIC-M2"]
gates: ["GATE-AUGUR-01", "GATE-AUGUR-07", "GATE-LEMMA-03", "GATE-LEMMA-04", "GATE-SKEL-01", "GATE-KIN-01", "GATE-ARENA-03", "GATE-CHRON-10", "GATE-GROUND-01", "GATE-SAFE-01", "GATE-SPARK-21", "GATE-TENFOLD-02", "GATE-TENFOLD-09", "GATE-GAUNT-01", "KILL-SKEL-SEXPR", "KILL-SKEL-SUBST", "KILL-AUGUR-FALSECERT", "KILL-MCP", "KILL-DEVORACLE"]
plan_refs: ["§43.4", "§43.1", "§43.2", "§40.6", "§40.3", "§40.4", "§40.5", "§43.12"]
---
## Background

**Purpose.** M1 delivers the smallest product that is already useful and already honest: a design studio on the
referee tier (the slow, careful CPU physics tier) and exact answers to the first mission queries, with every result
carrying its claim product and every family certificate re-checked by an independent binary.

**Why this is the first product, and why it is CPU only.** The first wedge is chosen to start where value is
deliverable with the fewest unproven dependencies. M1 needs only three foundations: the physics sibling, the async
runtime and the embedded database. It already demonstrates the thesis in small: a typed design, static analyses
from Lie theory, a synthesized controller, a rollout on the transactional referee, structured telemetry, and a
mission query answered over a *family* of designs with a certificate that an independent auditor re-checks, all
through one verb registry. Three further reasons. (1) *The result is not in the step*: an infinitely fast
environment step cannot deliver a ten-fold shorter design loop; what an answer is, how it is checked and how an
amendment withdraws it are exercised here on fixtures whose answers are known exactly. (2) *The rival is measured
while M1 is built*: M0 and M1 time the incumbent loop on `trj`, so M2's engine work is funded against a measured
denominator. (3) *A CPU studio survives every admission failure*: if the GPU, symbolic and proof foundations all
failed admission, M1 would still ship. Planning estimate: 5 weeks (DESIGN). Critical path: yes.

**Demonstrates.** NS-01 *Check before you simulate*: `frob design check` returns in under two seconds with the mass
roll-up, the actuator rating exceeded, the symmetry groups and three ranked repairs as typed edits. NS-07 *An import
that tells the truth*: importing a public humanoid description yields a loss report (dropped fields, inertias that
fail physical consistency, actuator limits filled from a card marked `assumed`). NS-22 *The docking fixture*: a cart
with unknown gain g ∈ {1, 2} must dock in three one-second ticks; the mission query returns four certified facts
(with the packet on time and command bound U = ¾ a first command of ¼ identifies the gain and both worlds dock;
docking tolerates sensor error ε exactly when ε < (U + ½)/4; with the packet late no causal controller docks in
both worlds; admitting a third gain 33/32 aliases with gain 1 at radius 1/64 and reverses every feasible label).
NS-13 on the CPU: the stance family, boundary exactly b = 3/14 m, corner allocation (160, 130, 110, 0) N as
witness, a dual vector as impossibility witness, re-checked by `frob-verify` in exact rationals, and withdrawn
correctly under the passive-spring amendment.

## Scope

In: the design language and compiler; modules; the first four archetypes and the cart; preflight stages 1 to 6;
URDF and MJCF import and export; actuators L1 and L2; the transactional referee step; static analyses; the
invariant filter and first baseline controllers; the clock contract; telemetry T0 to T2, the energy ledger,
`explain`; the chunk store, the job protocol, the artifact graph; the requirement-language core, fixture 1, both
study types; the first mission queries and bundles; lineage basics; every M1 verb on the CLI and over MCP;
`robot-docs`; the completed parity baseline; the incumbent design loop with the common agent.

Explicitly not in M1: any batched step, GPU backend, `strict` profile (M2); interiors, Nemesis, Autopsy,
qualification (M3); family policies (M4); certificates for derived kernels (M5).

What M2 needs from M1: `RobotModel` and the referee as parity oracle; the clock contract and packet queues;
actuator kernels' referee components; the chunk protocol and energy ledger; fixture 1; the measured incumbent
ledger and the removable share of every lever (a lever under 10% is struck from the speed case before M2 begins).

## Technical approach

**1. The artifact.**

- The `.fbot` design language and the model compiler; `.fmodule` modules with port contracts; the first archetypes
  (`quad-mammal`, `humanoid`, `diff-drive`, `mecanum`) and the cart of fixture 1; preflight stages 1 to 6 as
  `frob design check`; URDF and MJCF import and export with loss reports.
- Actuator levels L1 and L2. The transactional referee step with event location, retries that restore all state
  and witness migration, on planes, primitive obstacles and the door sill.
- Static analyses inside `design check` and as separate verbs; the invariant filter and baseline controllers for
  the first archetypes; the clock contract with its seven instants.
- Telemetry levels T0 to T2, the energy ledger and `frob explain run`. The chunk store, the job protocol under
  seeded faults, and the artifact graph behind `frob evidence trace`.
- The core of the requirement language with its three evaluators; `fixture:cart-sill-dock`; both study types on
  the CPU, with `frob eval preanalyze` listing the reachable verdicts of a fixed-n rule.
- The first mission queries on static conditions: `frob mission synthesize` on the stance family, the docking
  family and the pusher battery; `frob mission refute`; bundles written as `.frobcert` and accepted by
  `frob-verify` built from an empty environment.
- Lineage basics: content-addressed design versions, `frob design diff`, typed patches.
- Every M1 verb on the CLI and over MCP; `robot-docs` generated from the registry.

**2. Gates with a part due at this exit (compiled register tables: 4 admission, 59 convergence).** Where a row
names `strict` or `batch-cpu`, only its referee or CPU part is due here.

| Gate | Claim | Part due | Pass criterion in brief (TARGET) | Defined in |
|---|---|---|---|---|
| INT-FSIM-07 | Admission | M0 (the refusal); M1 (under witness migration) | Foundation: frankensim sustained contact step. The flat-to-edge-to-top transition returns the typed refusal through the airlock at the pin, and completes under witness migration with the energy ledger closed to its stated residual (§33.11 row 4) | §33.10 |
| INT-FSIM-16 | Admission | M1 | Foundation: frankensim group `ledger` (`fs-recompute`). Incremental recomputation equals full recomputation on 10⁴ random edit sequences; an input missing from a key is caught by the shadow recompute within 64 hits | §33.9 |
| INT-FSQL-02 | Admission | M1 | Foundation: frankensqlite pager policy (pattern and oracle). The memo store's eviction sequence equals the sibling policy's on 10 recorded traces of at least 10⁵ accesses | §33.6 |
| INT-FMCP-01 | Admission | M0 (admission); M1 (M1 verbs); M3 (full surface) | Foundation: fastmcp_rust. `frob-mcp` builds in the core graph on T with exactly one asupersync package (`cargo tree -d` shows no duplicate), lists the tools generated from the verb registry, and passes the MCP protocol smoke test over stdio and HTTP | §33.6 |
| GATE-ARENA-03 | Known answer | M1 | Monitors: the three evaluators agree bit for bit on 10⁶ random traces per operator, including multi-rate signals and windows with rounding; every requirement-language refusal of §24.14 (FROB-REF-520, -535, -536, -523, -538, -525, -526, -539 and -528) is triggered by its planted task file. | §24.17 |
| GATE-ARENA-04 | Known answer | M1 on the referee; M2 on the batch tiers | Fixture 1: every analytic row within tolerance on the referee and inside the fidelity envelope on the batch tiers; the creep rigs flip at the stated thresholds; both negative controls are caught. | §24.17 |
| GATE-ARENA-05 | Known answer | M1 | `eval.preanalyze` reproduces the tables of §24.10 exactly and refuses the unreachable verdict. | §24.17 |
| GATE-ATEL-02 | Known answer | M1 (the analyses that exist); M4 (the rest) | Each analysis A-01 to A-14 passes its known-answer tests; `design check` finishes in under 2 s at the 95th percentile on templates. | §28.23 |
| GATE-ATEL-05 | Known answer | M1 (A-20 to A-23); M4 (the rest) | Each analysis A-15 to A-26 passes its known answers (§28.22 and the owning chapters) and its negative control fails; the analyses marked ✓ keep `design check` under 2 s. | §28.23 |
| GATE-ATEL-06 | Known answer | M1 | The fixed-n planner reproduces the enumeration at n = 64 and n = 86 exactly; FROB-REF-553, -555, -542, -543 and -556 each fire on their planted violation, 100 of 100. | §28.23 |
| GATE-ATEL-11 | Known answer | M1 (dependency grades on mission cells); M3 (dependency grades); M4 | Revalidation on the frozen 30-amendment campaign: every decision equals full recomputation; zero unsafe reuse; the stale-key mutant is caught; unnecessary invalidations are at most 20% of unaffected verdicts, counted and reported. | §28.23 |
| GATE-ATEL-12 | Performance | M1 (analysis part); M2 (episode part) | PO-ATEL-07 holds; a shadow mismatch stops serving within one hit; `design check` after a one-parameter edit in 0.2 / 0.5 / 1.0 s at the 50th / 95th / 99th percentile. | §28.23 |
| GATE-AUGUR-01 | Known answer | M1 | Admission and checker reject 100% of a mutation corpus of at least 60 mutated queries and certificates (every mutation of §25.5.3, every row of §25.4.7, a swapped prefix, a clairvoyant witness) with the expected refusal code; zero false accepts. | §25.27 |
| GATE-AUGUR-02 | Known answer | M1 | Stance family through the public verb: boundary exactly 3/14 m; four vertex witnesses; dual (0, 1/2, 1/2) for the excluded cell; 11,520 of 11,520 grid states agree with an independent zonotope test; lifted and projected forms agree; the passive-spring amendment withdraws the cell at b = 1/4 m. | §25.27 |
| GATE-AUGUR-03 | Known answer | M1 | The docking family through the public verbs: 561 of 561 cells, nine boundary cells excluded, all late-packet cells excluded, the equality reading 15/16 exported, the three-gain amendment withdraws every positive cell and reports 1/65 m. | §25.27 |
| GATE-AUGUR-04 | Known answer | M1 | The pusher battery: 20 of 20 intervals with exact ends, and the per-build and shared queries return [1/2, 2] and ∅ for the same plant. | §25.27 |
| GATE-AUGUR-05 | Proof | M1 | Every `Certified` cell of GATE-AUGUR-02 to 04 is accepted by `frob-verify` built from an empty environment on a host with no GPU, in under 10 s in total; bundle digests identical across two hosts. | §25.27 |
| GATE-AUGUR-06 | Proof | M1, repeated at M3 | Checker independence: dependency closure is `std`; zero shared source files with proposers; a seeded-fault campaign of at least 200 mutated witnesses per proof kind yields zero false accepts. | §25.27 |
| GATE-AUGUR-07 | Design comparison | M1 | A fresh agent with only the generated documentation completes 10 scripted docking and stance amendments, names the binding hardware restriction in at least 9, and never reports an unresolved cell as a verdict; serial turns are recorded against an arm restricted to the lower verbs. | §25.27 |
| GATE-CHRON-03 | Known answer | M1 (referee); M2 (single precision) | The energy ledger closes: residual below 10⁻⁹ relative on the referee and 10⁻³ on single-precision tiers; a damped pendulum's ledger matches the hand calculation. | §29.20 |
| GATE-CHRON-04 | Known answer | M1 (CPU); M2 (device) | The chunk protocol under 10⁵ seeded fault schedules: INV-CHRON-01 to -04 hold in every one; each planted violation (late lease, overlap, conflict, tick-zero reuse) is refused, 100 of 100. | §29.20 |
| GATE-CHRON-10 | Known answer | M1 | The artifact graph: every row of `registries/gates.toml` with a PASS resolves; a dangling identifier fails the build; `evidence.trace` on 100 random claims returns closures whose hashes recompute. | §29.20 |
| GATE-CHRON-11 | Known answer | M1 (CPU); M2 (device) | Simulation testing: full await-point coverage of the publish and submit paths; every failure minimized to a regression that replays in under one second; dual-run agreement on 100 small jobs. | §29.20 |
| GATE-GROUND-01 | Known answer | M1, then every exit | Every report prints the `reality:` line; with no dataset it reads `no-claim`; a negative battery of 50 planted misuses is refused in full. | §30.22 |
| GATE-KIN-01 | Known answer | M1 | Pendulum period within 10⁻⁶ relative at h = 1 ms; symmetric-top precession within 10⁻⁴; free-flight momentum conserved to 10⁻¹² relative over 10⁶ steps on the variational lane; ballistic invariants of C-04. | §16.17 |
| GATE-KIN-02 | Parity | M1 | Contact-free forward dynamics of each reference robot against the pinned oracle: joint accelerations within 10⁻⁹ relative in `f64`. | §16.17 |
| GATE-KIN-04 | Known answer | M1 (referee); M2 (`strict`) | The transactional rows of §16.16 on `referee` and `strict`: impact time within 10⁻⁹ s; tied sliders exact to 10⁻¹² N·s; zero uncommitted reads in 10⁵ injected faults; forced-retry digests equal; refinement statistics inside 3σ at 10⁶ draws; the redraw mutation caught. | §16.17 |
| GATE-LEMMA-03 | Known answer | M1 | Every proof kind in use has one specimen that `frob-verify` accepts and at least three mutations it rejects; the four stance mutations, an unknown kind with consistent hashes and a changed quantifier are all rejected; the binary has no dependency outside the standard library and runs in an empty environment. | §12.6 |
| GATE-LEMMA-04 | Proof | M1, repeated at M4 | The first mission slice of §25 ships its inner cells and its exclusions as bundles; a re-check on a second machine accepts every one; the seeded-defect suite yields zero accepted false certificates. One accepted false certificate fails the gate. | §12.6 |
| GATE-MARROW-02 | Known answer | M1 | Character tables and isotypic projectors correct for Cₙ and Dₙ with n ≤ 12, their products with C₂, and S₄, against tables on file. | §13.10 |
| GATE-REFLEX-01 | Known answer | M1 (referee); M2 (`batch-cpu`); M3 (all but tracked and hybrid); M4 | Each archetype stands for 10 s and traverses 5 m of flat ground under its baseline controller: at M1 the first archetypes, on `referee` only, restricted to those whose contact law exists at M1 and whose contact count fits the referee's 21-contact cap (§16.6); at M2 the same set on `batch-cpu`; at M3 all eighteen on `batch-cpu`, except tracked and hybrid archetypes, which follow at M4 with their contact laws. | §22.18 |
| GATE-REFLEX-03 | Known answer | M1 | Filter consistency inside χ² bounds on the simulated IMU; position drift under 2% of distance on a flat trot, scored by the gauge rule; PO-REFLEX-01; the comparator's `overconfident` flag fires on a planted bias. | §22.18 |
| GATE-SAFE-01 | Known answer | M1, then every exit | Flags: every artifact produced by every exporting verb carries the three fields, all `false`. Negative control: an artifact edited to `true`, and one with the fields removed, are each refused by the importer and by `frob-verify`. | §37.14 |
| GATE-SAFE-08 | Known answer | M1 | Claim product: over random sets of input evidence the envelope's class equals the meet; no public function returns a stronger class than its inputs. | §37.14 |
| GATE-SAFE-10 | Known answer | M1 | Path jail: a corpus of traversal, absolute-path and swapped-link attempts is refused on every path-taking argument of every verb. | §37.14 |
| GATE-SENS-04 | Known answer | M1 (CPU); M2 (every tier) | Clock contract: the 400/30 schedule exactly; zero drift against rational arithmetic over 10⁹ ticks; the impulse and arrival cases; identical schedules on every tier. | §20.21 |
| GATE-SENS-05 | Known answer | M1 (CPU); M2 (every tier) | Checkpoints: restore at the three adversarial instants gives byte-equal delivered observations and physics digests over the next 10⁴ ticks on every tier. | §20.21 |
| GATE-SENS-08 | Known answer | M1 (docking rows); M3 | Observability: the corridor case and the four-dimensional gauge, by the linearized and the empirical form, agreeing to within 5% in the non-zero eigenvalues. | §20.21 |
| GATE-SENS-09 | Known answer | M1 (docking rows); M3 | Alias witness: the T-junction and the late-packet docking witnesses are found and pass the checker; on 64 planted layouts every planted alias is found and no false witness is reported. | §20.21 |
| GATE-SENS-12 | Known answer | M1 (CPU); M2 (every tier) | Causal-policy test: the reference actor and every baseline controller pass; each of the five planted leaks fails. | §20.21 |
| GATE-SINEW-01 | Known answer | M1 | Known answers within 1%: stall torque, no-load speed, electrical and mechanical time constants of a DC motor step, thermal steady-state rise ΔT = P·R_th. | §15.14 |
| GATE-SINEW-02 | Known answer | M1 (referee); M2 (single precision) | Port energy balance closes to 10⁻⁹ relative on the referee and 10⁻⁴ relative per episode on single-precision tiers. | §15.14 |
| GATE-SINEW-04 | Known answer | M1 (referee); M2 (batched tiers) | The six cases of §15.6: breakaway within one step of τ_s/r; amplitude loss per cycle within 2% of 4·τ_c/k; drift at most ε·t; zero output motion inside the gap; loop width within 1% of β; closure speed and energy loss within 1%. Referee exact; batched tiers within their stated regularization. | §15.14 |
| GATE-SINEW-05 | Known answer | M1 | Thermal: equilibrium, time constant, critical current and time to limit within 0.5% of the closed forms of §15.7; the two-node condition; tangency under torque holding within 1%; the worked card's 20 A point admitted by the constant-resistance check and refused by the corrected one. | §15.14 |
| GATE-SINEW-06 | Known answer | M1 | The five closed forms of §15.8 within 0.5%; over 10⁴ random operating points the inscribed-circle envelope never exceeds the hexagon envelope. | §15.14 |
| GATE-SINEW-07 | Known answer | M1 | Battery and bus: the step response's drop and both time constants within 1%; the ceiling E²/(4·R) and the voltage at a given power within 0.1%; a demand above the ceiling produces a brownout event in 100 of 100 trials and never a completed step; exhaustion refuses. | §15.14 |
| GATE-SINEW-08 | Known answer | M1 | Passivity and second law: over 10⁶ random operating points in all four quadrants every loss and every entropy production is nonnegative; the single-multiplier gear fixture refuses; every card in the catalog passes `actuator.check`. | §15.14 |
| GATE-SKEL-01 | Parity | M1 | Each reference robot imports with forward-kinematics parity against the oracle within 10⁻⁹ m and 10⁻⁹ rad over 10⁴ configurations, and its loss report lists every dropped field found by a field-coverage diff. | §14.15 |
| GATE-SKEL-02 | Known answer | M1 | Functor laws and INV-SKEL-02 hold on 10⁴ random designs and edits. | §14.15 |
| GATE-SKEL-03 | Known answer | M1 (archetypes landed); M3 (all eighteen) | Each of the eighteen archetypes compiles, passes stages 1 to 6 of `design check` with no refusal, has the symmetry groups on file, and stands or rolls under its baseline controller at the milestone GATE-REFLEX-01 gives for it. | §14.15 |
| GATE-SKEL-04 | Known answer | M1 (stages 1 to 6); M3 (stages 7 and 8) | Stages 1 to 6: of 100 seeded defects (at least two per refusal code) all refused, at least 95 by the stage and code that own them; zero refusals on the eighteen archetypes and on 10³ random valid designs. Stages 7 and 8: of 40 planted cases at least 36 decided correctly within budget and none wrongly. | §14.15 |
| GATE-SKEL-05 | Known answer | M1 | INV-SKEL-04 and PO-SKEL-01 hold on 10⁴ random designs; a JavaScript client's round trip of every wide integer and hash matches the native decoder. | §14.15 |
| GATE-SKEL-06 | Known answer | M1 | Substitution controls: the mirror swap reuses every sibling product bit-identically; the shared-bus edit invalidates every electrical and thermal product; a reversed port sign refuses; INV-SKEL-06 holds on 10⁴ random edits. | §14.15 |
| GATE-SKEL-07 | Known answer | M1; M3 (all eighteen) | PO-SKEL-02 holds exactly; the mass of each archetype lies within 15% of the mass on file for its reference robot where one exists. | §14.15 |
| GATE-SKEL-08 | Known answer | M1 | Patches: a stale parent refuses in 100 of 100 trials; a repeated idempotency key returns the same `DesignId`; a schema migration leaves forward kinematics and mass properties bit-identical and old runs replayable. | §14.15 |
| GATE-SPARK-21 | Parity | M0 (the `trj` rows that need no agent); M1 (complete) | Incumbents measured on `trj` and on this Mac under the disclosure block: physics and full step for Q and H at matched and native settings; time to threshold on one and two cards; cold start; edit to first step (parameter and topology); CPU MuJoCo at 1, 24 and 36 threads; a Metal-path incumbent. Each published figure of §36.3 reproduced within 20% or the difference explained in writing. | §36.16 |
| GATE-TETHER-02 | Known answer | M1 | CLI and MCP surfaces are generated from the registry and their parity diff is empty; every verb has a schema, an example and a refusal list. The served `tools/list` equals the registry under the name map. | §32.21 |
| GATE-TETHER-07 | Known answer | M1 | Integers: the golden envelope set passes through a double-decoding client with no changed value; the schema lint finds no bare 64-bit integer. | §32.21 |
| GATE-TRAC-01 | Known answer | M1 and M2 | Every row of §18.16 that tests a Solid mechanism passes on `strict` and on `batch-cpu`, and on `referee` for the rows FrankenSim's solve can express (the list is recorded with the gate); rows for the fast law pass on GPU tiers within the stated creep. | §18.17 |
| GATE-TRAC-02 | Parity | M1 | Primitive pair distances and normals agree with `fs-query` within 10⁻⁹ m (`f64`) on 10⁶ random pairs. | §18.17 |
| GATE-TRAC-06 | Known answer | M1 (referee); M2 (`strict`) | Zero missed hits in 10⁶ randomized thin-barrier crossings under transactional profiles, and the endpoint-only mutation caught; on the sill, work and impulse within 1% of their refined values at h/4 and no couple above 10⁻⁹ N·m in `f64`; the capacity-bias comparison labeled. | §18.17 |
| GATE-WS-02 | Known answer | M1, then at each new format; complete at M5 | The format registry round trips of §34.9. | §34.13 |
| GATE-WS-03 | Known answer | M1 | The auditor's independence, §34.11. | §34.13 |

**3. Gates of later subsections due at this exit.**

| Gate | Part due at this exit | Statement (from the defining subsection) | Defined in |
|---|---|---|---|
| GATE-MARROW-06 | whole | Known answer. The spring fixture is exact to 10⁻¹² in double precision on the referee and within 10⁻⁵ relative on device tiers; snapshot, restore and teleport preserve k on 10⁴ random states; the mutant that omits k from the snapshot is caught by an energy jump of π² J; `sense.alias` returns the witness for a single-turn encoder and none for a multi-turn encoder. M1. | §13.11 |
| GATE-SKEL-11 | whole | Known answer. On 10³ random trees with fixed joints the folded model's mass, first moment and pseudo-inertia equal the sums over the unfolded links exactly in rationals and to 10⁻¹² relative in double precision; forward dynamics of the folded model and of the unfolded model with the joints held as constraints on the referee agree to 10⁻⁹; every named attachment resolves to the same world pose at 100 random configurations. The hand case is exact, and the mutant without the offset term fails it. The `g1-reduced-30` profile reports its omissions, sums to 31.96314202 kg, and the 320 N fixture passes on it and fails on the full import. M1. | §14.17 |
| GATE-SINEW-10 | whole | Known answer. The one-rotor table is exact: the envelope has one generator (1, 1) N·m, the command (1, −1) N·m is refused as outside the image, the reflected inertia is [[1, 1], [1, 1]] and the two energies are 0 J and 2 J; the diagonal mutant reports 1 J twice and fails. For 10³ random couplings with up to four motors and six joints the power τ_jointᵀ·q̇ equals τ_motorᵀ·θ̇ to 10⁻¹², and the mass matrix with pulled-back rotors equals the mass matrix of the model with explicit rotor coordinates to 10⁻¹⁰. The signed motion witness passes on every card of the catalog and fails on each card with its ratio negated. M1. | §15.15 |
| GATE-TRAC-12 | on the referee | Known answer. The arc fixture is detected on 1,000 of 1,000 seeded variants (radius 0.2 to 2 m, angle up to π, sphere radius 1 to 10 cm, an obstacle on the arc and outside the padded chord); the chord-only mutant fails the gate. On 10⁴ random two- and three-joint sweeps the bound of the second item contains 1,000 sampled path points per sweep with no exception. In the tunnelling audit (`contact.audit`) no missed contact is attributed to a broad-phase rejection. M1 on the referee, M2 on the transactional batch profile. | §18.20 |
| GATE-AUGUR-31 | whole | Known answer. Through the public verb the stance family returns 3/14 m under `adjustable` with the bilinear law, 1/2 m under `affine` with the symmetric law and a matching dual, and an exclusion for every b under `constant`. The bilinear law evaluated on a 121 × 61 grid of loads meets every balance equation and rating in exact rationals. The four vertex witnesses relabelled `affine` are rejected by `frob-verify`, and so is a certificate with no law object. M1. | §25.35 |
| GATE-AUGUR-32 | whole | Known answer. Through the public verbs: δ = 1/50 is excluded with the alias pair and the partition of the probe axis; δ = 1/40 is included with p = 5/8; the minimum is returned as the root of 16·δ² + 41·δ − 1 with an enclosure no wider than 10⁻⁹ m and `attained: true`. The witness policy passes the 133,185 exact cases on the raw dynamics. A screen compiled for the gain set {1, 2} is refused on the interval [1, 2]. M1. | §25.36 |
| GATE-CHRON-14 | on the referee | Known answer. The three fixtures at 32, 64 and 2,000 Hz on `referee` and `strict`: constitutive dissipation of the resting block is exactly 0; sliding heat is within 0.1% of 4 W; impact loss is within 10⁻⁹ J of 2 J; a mutant that feeds algorithmic exchange into the film temperature is caught, because its temperature rise differs between 32 and 64 Hz by a factor of 2. M1 on the referee; M2 on the batch tiers. | §29.22 |
| GATE-WS-08 | first run; then at every release | Known answer. A release manifest with four planted cells is rejected on each: a device cell backed only by a CPU receipt; a device cell whose receipt records a fallback on one stage; a comparison cell whose incumbent figure is published and not re-measured; a cell whose study ended `inconclusive` and is listed as a capability. M1, and at every release. | §47.4 |
| GATE-FID-09 | rows 1, 3 and 5 | Known answer. Every row is reproduced at its tolerance (times to 0.1 s; sinkage to 10⁻⁵ m in double precision; counts exact; admissions exact) and every negative control fails. Rows 1, 3 and 5 at M1; row 2 at M4; row 4 with the continuation lane. | §38.17 |
| GATE-GAUNT-04 | the checker | Known answer ¦ Certifiers: every row of §38.12 meets its threshold; the certificate mutation corpus holds at least 1,000 mutants per proof kind with zero accepted ¦ M1 for the checker; M3 for the lanes | §38.15 |
| GATE-GAUNT-09 | whole | Known answer ¦ Experiment form: every registered method experiment has the six parts of §38.4; the harness passes PO-GAUNT-01; a planted experiment with an unreachable verdict is refused ¦ M1 | §38.15 |
| GATE-GAUNT-11 | first rows; each row lands at the milestone of its owner section | Known answer. Every row is reproduced at its tolerance (single-precision rows bit-exact on every backend; rational rows exact; the root row to four significant figures in exact rational evaluation of α and γ) and every negative control fails. Each row lands at the milestone of its owner section. | §38.18 |

**4. Program gates.** GATE-TENFOLD-02 (N40-13): the incumbent design loop with the common agent, with closed phase
ledgers; if unmet, M1 does not exit and the composed table of published figures may not be cited as a denominator.
GATE-TENFOLD-09 (N40-18), ceiling before speed, first run here and at every exit after. The schedule gate
(N40-11): M0 plus M1 against 7 weeks; not met above 10.5 weeks for the cause BET-12 names; then the program pauses
for a written re-plan before M2. GATE-SPARK-21 complete. Recurring from here: GATE-GROUND-01 (the `reality:` line)
and GATE-SAFE-01 (the flags). GATE-GAUNT-01.

**5. Kill gates decided here.**

| Kill gate | Trigger | Fallback, named in advance | Decided | Owner (consumer) |
|---|---|---|---|---|
| KILL-SKEL-SEXPR | INV-SKEL-04 cannot be held after one repair cycle | The S-expression view is removed; JSON is the only syntax | M1 | §14.15 |
| KILL-SKEL-SUBST | A reused product differs from its recompiled value after one repair cycle, or leaf-local reuse saves less than 20% of compile time on the archetypes | Every substitution is global; the memo graph keeps subterm reuse by content hash only | M1 | §14.15 |
| KILL-AUGUR-FALSECERT | One certificate accepted by `frob-verify` is shown false within its scope | The procedure of §25.5.5; if the cause is the rule and not an implementation defect, the proof kind is removed and its cells become `Tested` | Standing, from M1 | §25.27 |
| KILL-MCP | INT-FMCP-01 cannot pass within the M0 budget after FD-FMCP-01 and FD-ASUP-01 have been attempted | Fallback 1: `frob-mcp` as its own workspace, talking to `frobd` over a local socket in newline-delimited JSON. Fallback 2: agents use the CLI, which is complete on its own (§33.13, §40.7) | M0; re-read at M1 with the M1 verbs | §32.21 |

| Kill gate | Trigger and fallback (from the defining subsection) | Decided | Gate beside it | Defined in |
|---|---|---|---|---|
| KILL-DEVORACLE | Trigger: of the first five pilots re-run on the CPU referee, two or more change verdict relative to their development-oracle run. Fallback: development-oracle results lose standing as funding evidence; pilots for work packages not yet funded wait for the CPU referee, and packages funded on a reversed pilot are re-decided | M1 | — | §38.15 |

KILL-AUGUR-FALSECERT stands from this milestone on: one certificate accepted by `frob-verify` and shown false
removes its proof kind if the rule is at fault. The first reading of KILL-AUGUR-TIE (decided at M3) is taken on the
docking slice: if a conventional scenario-tree program gives the better answer per engineering hour, that program
ships behind the same verbs. KILL-TENFOLD-LEVER is applied to the rival's ledger.

**6. Exit rule.** Recorded by a fresh-context session (seed BD-147, procedure N40-05). The exit line: *NS-01, NS-07
and the docking scenario demonstrated; the schedule gate met.* Every report prints its `reality:` line from here on.

**7. Dependency firewall.** Needs FrankenSim (referee, geometry, ledger, memo store), asupersync (jobs, lab
runtime), FrankenSQLite (ledger). Uses if admitted: FrankenSymPy, fastmcp_rust, franken_numpy, frankenscipy. If a
foundation is missing: CLI only; no kernel certificates. The mission slice is unaffected, because `frob-verify`
uses the standard library alone.

**8. Owner input wanted.** OPEN-PROG-4, the compute for the rank-agreement experiment (about forty per-design
trainings, about 5 device-hours with reused specialists, about 45 otherwise; default: not run above the default
envelope, every cheap rung is `uninformative` and neither selects nor discards). The owner's reading of the M0
ledger against the expected ranges per mission class.

**9. Stop or re-plan.** The schedule gate. Beyond it: re-plan the mission chapter before adding physics if a fresh
agent cannot complete the docking and stance amendments of GATE-AUGUR-07 after one repair cycle, because the
mapping from sensors and actuators to the query is then the defect. Re-plan the referee if its refusal rate on the
M1 fixtures (retry budget defaults L_max = 16, E_max = 64, 3 retries; exhaustion refuses) leaves the sill fixture
without a completed run.

**10. No single failed bet blocks the exit.** A symbolic, proof or GPU foundation fails admission → the same M1.
BET-19 on the first slice (a conventional program answers the docking query as well) → that program behind the same
verbs, schema and checker.

**11. Work packages.**

| Package | Delivers | Crates | Milestones | Key gates |
|---|---|---|---|---|
| WP-07 | Design language and compiler | `frob-design`, `frob-model` | M1 | GATE-SKEL-02, -03, -05, -08 |
| WP-08 | Import and export | `frob-io` | M1 | GATE-SKEL-01 |
| WP-09 | Actuators | `frob-actuate` | M1 | GATE-SINEW-01, -02, -05 to -08 |
| WP-10 | Referee tier | `frob-dyn`, `frob-contact` | M1 | GATE-KIN-01, -02; GATE-TRAC-01, -02 |
| WP-11 | Static analyses I | `frob-analyze` | M1 | GATE-ATEL-02 |
| WP-12 | Reflex I: invariant filter and baseline controllers | `frob-control` | M1 | GATE-REFLEX-01, -03 |
| WP-13 | Chronicle I: telemetry, energy ledger, explain | `frob-chronicle` | M1 | GATE-CHRON-03 |
| WP-14 | Gauntlet harness and oracles | `frob-conformance`, `tools/oracle` | M1 | GATE-GAUNT-01 |
| WP-38 | Modules, archetypes and preflight | `frob-design`, `frob-model`, `frob-analyze`, `frob-algebra` | M1, M3 | GATE-SKEL-03, -04, -06, -07; GATE-MARROW-03 |
| WP-39 | Transactional referee step | `frob-dyn`, `frob-contact`, `frob-fsim` | M1 | GATE-KIN-04; GATE-TRAC-06 |
| WP-40 | Clock contract | `frob-sense` | M1, M2 | GATE-SENS-04, -05 |
| WP-41 | Requirement language, fixtures and the two study types | `frob-spec`, `frob-arena` | M1 | GATE-ARENA-03, -04, -05; GATE-ATEL-06 |
| WP-42 | First mission slice [F] | `frob-mission`, `frob-spec`, `frob-lemma`, `frob-cert` | M1 | GATE-AUGUR-01 to -07; GATE-LEMMA-04 |
| WP-43 | Chunk store, job protocol, job engine and artifact graph | `frob-chronicle`, `frob-verbs` | M1, M2 | GATE-CHRON-04, -10, -11; INT-ASUP-02; GATE-TETHER-04, -05, -06, -08 |

## Acceptance criteria

- [ ] Every part in the two gate tables has a recorded value and a caught negative control; the referee-only and
      CPU-only parts are recorded as parts, with their later parts still `not-run`.
- [ ] NS-01, NS-07, NS-22 and NS-13 (CPU) run from a clean checkout through `frob --json` and through `frob-mcp`.
- [ ] GATE-TENFOLD-02 and -09 are recorded; GATE-SPARK-21 is complete; the schedule gate is evaluated with its
      cause attribution.
- [ ] KILL-SKEL-SEXPR, KILL-SKEL-SUBST, KILL-DEVORACLE and the M1 re-read of KILL-MCP have ADRs;
      KILL-AUGUR-FALSECERT is registered as standing.
- [ ] Every report prints `reality: no-claim` (no dataset exists) and every exported artifact carries the three
      flags, all false.
- [ ] The exit report for M1 is `exit-ready`; the exit record is written by a fresh-context session.

## Test plan

### Unit tests

Carried by member beads. Epic check: due counts for M1 are 4 admission, 59 convergence, 4 kill decisions, plus 12
late parts and 1 late kill decision.

### End-to-end script and logging

`scripts/exit/review.sh M1`: the four demonstrations as scripted sessions (`frob design check`, the URDF import of a
public humanoid description with its loss report, `frob mission synthesize` on stance and docking, `frob result revalidate` under the amendment,
`frob-verify` on every bundle on a host with no GPU), each through CLI and MCP; structured logs with the evidence
block of every envelope, bundle digests, the measured boundary values beside the exact ones, and a summary of
gates touched.

### Known answers and negative controls

3/14 m; (160, 130, 110, 0) N; dual (0, 1/2, 1/2); 11,520 of 11,520 grid states; 561 of 561 docking cells; 15/16;
1/65 m; ε < (U + ½)/4; 20 of 20 pusher intervals. Controls: at least 60 mutated queries and certificates rejected
with zero false accepts; 200 mutated witnesses per proof kind; the negative battery of 50 planted misuses of the
`reality:` line; the 100 seeded design defects.

## Considerations

- M1 has 59 convergence parts in five weeks; it is the first real test of bet BET-12 (narrow purity is affordable).
- The split of gates into referee parts (M1) and `strict` or batch parts (M2) must not let a part print as a whole
  pass: the register records parts (N40-01).
- The incumbent design-loop measurement needs an agent and device time on `trj`; it is an owner-approved run.

## References

Plan §43.4, §43.1, §43.2, §40.3 to §40.6, §43.12. Bets BET-12, BET-19, BET-24. Scenarios NS-01, NS-07, NS-13,
NS-22. Work packages WP-07 to WP-14, WP-38 to WP-43.
