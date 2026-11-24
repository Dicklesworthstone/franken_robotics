---
key: "EPIC-M0"
title: "Milestone M0, spine and airlock: foundations admitted, one kernel on three backends, the rival's loop measured"
type: "epic"
priority: 0
milestone: "M0"
labels: ["m0", "delivery", "solid", "gate"]
parent: "EPIC-PROGRAM"
deps: []
needs: []
related: ["EPIC-M1", "EPIC-PROGRAM", "N40-02", "N40-04", "N40-05", "N40-06", "N40-07", "N40-12", "N40-23", "N40-27"]
gates: ["INT-TOOLCHAIN-01", "INT-FSIM-01", "INT-FSIM-18", "INT-ASUP-01", "INT-FSQL-01", "INT-FMCP-01", "GATE-MARROW-01", "GATE-SPARK-00", "GATE-SPARK-01", "GATE-LEMMA-00", "GATE-TETHER-03", "GATE-TENFOLD-01", "GATE-GAUNT-01", "KILL-FSIM", "KILL-MCP"]
plan_refs: ["§43.3", "§43.1", "§43.2", "§40.6", "§40.2", "§40.3", "§40.4", "§40.5", "§43.12"]
---
## Background

**Purpose.** M0 retires the risks that could invalidate the architecture or the thesis, with the smallest possible
amount of code. Two questions are answered by measurement: *can the foundations be consumed from outside their
trees*, and *is the rival's loop what the thesis composes from published figures*. No subsystem is built out.

**Why this comes first.** FrankenRobotics is a robotics design and simulation system built on sibling libraries
(physics, async runtime, database, tensor and Metal gateway, symbolic engine, MCP server library, array library) and
aimed at a ten-fold shorter design loop than the strongest incumbent workflow. Both halves of that sentence are
assumptions until measured. If the physics sibling cannot be consumed by path from another workspace (bet BET-04),
the architecture changes. If the incumbent's loop is not what the published numbers suggest, the denominator of the
ten-fold bet (BET-13) changes, and with it which work is worth funding as a speed lever. M0 delivers exactly these
two measurements, plus one arithmetic kernel compiled from one source for the CPU, CUDA and Metal (the smallest
proof that "one source, three backends" is real), the claim-product type with its no-authorization flags, and the
gate register with its exit tooling. Planning estimate: 2 elapsed weeks for a swarm (DESIGN; used only by the
schedule gate). Critical path: yes. Tier: Solid.

**No North-Star scenario is tagged M0.** The milestone delivers measurements, not a product story.

## Scope

In: admission of the foundations; the choice of the one toolchain T; the two P0 upstream work items in their own
repositories; the referee smoke run; one kernel on three backends with the IEEE probe and hardware fingerprints;
the first certificates; the claim product and flags; the M0 verbs on the CLI and over MCP; the incumbent phase
ledger on the Linux workstation `trj`; the plan-time pilots re-run; the gate register and exit tooling.

Explicitly not in M0: any subsystem built out (no design language, no contact battery, no batched step, no
training); any agent-driven rival measurement (M1); any Performance claim of ours.

What M1 needs from M0: the admission records of FrankenSim, asupersync and FrankenSQLite (the only three M1 relies
on); T fixed; `frob-verbs`, the envelope and exit codes; `frob-core` identities, evidence classes and flags;
`frob-cert` with its empty dependency table; the phase ledger's no-agent rows; KILL-FSIM and KILL-MCP decided.

## Technical approach

**1. The artifact (what exists at the exit).**

- `frob doctor` lists the admitted foundations with their records and the stack receipt. `DEPENDENCY_ADMISSION.md`
  holds every record, passing or failing, the toolchain matrix (which of the four nightlies in play compile which
  build sets) and the choice of T.
- fastmcp_rust builds on T in its own repository, with its test suite and its MCP conformance checks passing
  (upstream work item FD-FMCP-01). FrankenSim's constellation, FrankenTorch and fastmcp_rust pin one published
  asupersync version (FD-ASUP-01).
- `frob sim run g1 --tier referee --seconds 2` produces a state digest equal to the one FrankenSim computes for
  itself. The same job through `frob-fsim-worker` returns equal chunk digests.
- `frob spark probe` and `frob spark fingerprint` run on both device families. `K-cay-step` runs from one source
  on `batch-cpu`, CUDA and Metal. The exact and guarded predicate kernels give identical bytes on all three.
- Theorem cards T-01, T-03 and T-04 have certificates that replay offline. The claim-product type and the
  no-authorization flags exist in `frob-core`, with compile-fail tests.
- The M0 verbs answer through `frob --json` and through `frob-mcp`, which is built in the core graph on T, links
  `frob-verbs` and runs the verbs in process.
- The incumbent phase ledger on `trj`: cold start, a parameter edit and a topology edit to the first step, and
  time to the return threshold on one and two cards, at pinned revisions, each with a disclosure block.
- The plan-time pilots re-run in this repository and recorded: the stance enumeration, the docking grid, the
  pusher battery, the reachable-verdict enumeration of the fixed-n rule, the boundary screen in guarded single
  precision, the ceiling of the solver comparison, and the power of the draft decision rule.

**2. What it demonstrates.** The two measurements the program stands on: BET-04 (consumption by path) and the
denominator of BET-13.

**3. Gates with a part due at this exit (compiled register tables: 23 admission, 11 convergence).**

| Gate | Claim | Part due | Pass criterion in brief (TARGET) | Defined in |
|---|---|---|---|---|
| INT-TOOLCHAIN-01 | Admission | M0 | Foundation: The one Rust toolchain T of the build graph. T is chosen and recorded in `franken-stack.lock` and `rust-toolchain.toml`, in the order of preference of §33.13: (a) the suite's common nightly, (b) FrankenSim's present pin. The whole build graph (the core, the GPU annexes, `derive/frob-derive` and `surface/frob-mcp`) builds and tests on T; the matrix of §33.12 is recorded: which of the four nightlies in play compile `fs-qty`, the bedrock-direct list, the M1 closure and each of the other build sets. After this gate T moves only by a toolchain train | §33.6 |
| INT-FSIM-01 | Admission | M0 | Foundation: frankensim: bedrock crates by path. A crate depending by path on `fs-ga`, `fs-mbd`, `fs-contact`, `fs-time` and `fs-query` passes `cargo check --all-targets` and reproduces the digest of a G1 forward-dynamics state at the pinned commit; the cold check fits one hour, 16 GiB and 8 cores; check times, peak memory, package count and sibling drift recorded | §33.6 |
| INT-FSIM-02 | Admission | M0 | Foundation: frankensim `fs-ga`: group façades, twist and wrench conventions. Power ⟨wrench, twist⟩ is frame-invariant to 10⁻¹² relative; the analytic Jacobian agrees with central differences to 10⁻⁷ relative at 1,000 seeded configurations; a deliberately reversed adjoint fails at least three fixtures | §33.10 |
| INT-FSIM-03 | Admission | M0 | Foundation: frankensim `fs-mbd`: tree kinematics, articulated-body solve, rigid-point motion. A two-joint arm's accelerations equal those from independently differentiated energy to 10⁻¹⁰; an offset inertial sensor's acceleration equals the rigid-point formula; scalar genericity, adjoint support and contact inputs recorded | §33.10 |
| INT-FSIM-04 | Admission | M0 | Foundation: frankensim `fs-mbd` contact steps and `fs-contact` broad phase. A thin step lying between two clear endpoint poses is detected on 1,000 of 1,000 seeded sweeps; a set of 22 contacts returns a typed refusal | §33.10 |
| INT-FSIM-05 | Admission | M0 | Foundation: frankensim `fs-mbd::planar_drive` and the robot catalog builders. Exact straight travel, the near-zero-curvature case of §33.11 row 2, wrapped against unwrapped headings, and incremental against one-shot integration to 10⁻¹². Until it passes, `planar_drive` is not an oracle | §33.10 |
| INT-FSIM-06 | Admission | M0 | Foundation: frankensim `fs-g1-train` (nothing linked: an update rule to compare against). The three probes of §33.11 row 1: the four combinations of advantage sign and clip side against finite differences to 10⁻³; the critic gradient against the current forward output; a packed sequence against separate replays, ratio 1 to 10⁻⁶. Until it passes upstream, neither file is a baseline | §33.10 |
| INT-FSIM-07 | Admission | M0 (the refusal); M1 (under witness migration) | Foundation: frankensim sustained contact step. The flat-to-edge-to-top transition returns the typed refusal through the airlock at the pin, and completes under witness migration with the energy ledger closed to its stated residual (§33.11 row 4) | §33.10 |
| INT-FSIM-08 | Admission | M0 | Foundation: frankensim `fs-scene`, `fs-query`: support and keep-out boxes, convex queries. A box-on-plane support query equals the closed form. A mesh-backed interior is not admitted by this test | §33.10 |
| INT-FSIM-09 | Admission | M0 | Foundation: frankensim `fs-phs`, `fs-tribo`, `fs-matdb`: ports, friction laws, material data. Power across an interconnection sums to zero to 10⁻¹² relative; every pair parameter returns with its provenance record; a missing material pair is a refusal | §33.10 |
| INT-FSIM-10 | Admission | M0 | Foundation: frankensim `fs-symmetry`, `fs-ad`: cyclic cases, implicit sensitivities, gradient checks. The implicit sensitivity of a 2×2 fixed point equals its closed form. Recorded scope: cyclic groups only, no contact-event derivatives | §33.10 |
| INT-FSIM-11 | Admission | M0 | Foundation: frankensim `fs-tilelang` and the instrument-lab kernels (pattern only). No dependency is added. One dialect source compiles through both preludes and matches its CPU twin (the test of GATE-SPARK-00) | §33.10 |
| INT-FSIM-18 | Admission | M0 | Foundation: frankensim worker-process fallback. A 2,000-step G1 referee rollout submitted through the worker returns chunk digests equal, bit for bit, to the same job run in process; a worker killed mid-job and resubmitted returns the same result (§33.14) | §33.6 |
| INT-ASUP-01 | Admission | M0 | Foundation: asupersync. `cargo tree -d` shows one asupersync package in each workspace, the same published version in all of them, and no binary links two. This is the gate that proves FD-ASUP-01 | §33.6 |
| INT-FSQL-01 | Admission | M0 | Foundation: frankensqlite. The ledger opens, writes, and recovers after a forced kill mid-write | §33.6 |
| INT-FT-01 | Admission | M0 | Foundation: frankentorch `ft-kernel-metal`. The gateway builds in the Metal annex, with FrankenTorch's `block` patch repeated at the annex root, and runs a kernel on this Mac | §33.6 |
| INT-FT-02 | Admission | M0 | Foundation: frankentorch `ft-kernel-metal` gateway. A buffer stays resident across two command buffers; the gateway returns a completion receipt from the device; a device tag alone earns zero operator coverage | §33.10 |
| INT-CUDA-01 | Admission | M0 | Foundation: `cudarc`. The pinned `cudarc` builds without a CUDA toolkit, loads the driver on `trj`, and runs a kernel on both GPUs | §33.6 |
| INT-FSYM-01 | Admission | M0 | Foundation: frankensympy. `derive/` builds on T against the `fsym-*` crates by path, derives `K-cay-step`, and its certificate replays in the offline auditor | §33.6 |
| INT-FSYM-02 | Admission | M0 | Foundation: frankensympy sparse Jacobians and formal checker. The planar two-link Jacobian compiles and agrees with closed forms at a regular and a singular configuration | §33.10 |
| INT-FMCP-01 | Admission | M0 (admission); M1 (M1 verbs); M3 (full surface) | Foundation: fastmcp_rust. `frob-mcp` builds in the core graph on T with exactly one asupersync package (`cargo tree -d` shows no duplicate), lists the tools generated from the verb registry, and passes the MCP protocol smoke test over stdio and HTTP | §33.6 |
| INT-FNP-01 | Admission | M0 | Foundation: franken_numpy. An array file written through `fnp-io` reads back identically | §33.6 |
| INT-FNP-02 | Admission | M0 | Foundation: franken_numpy and frankentorch host tensor layouts. The stride conversion at the interchange boundary reads element 3, not element 12, on the 2×2 case (§33.11 row 3) | §33.10 |
| GATE-LEMMA-00 | Proof | M0 | T-01, T-03 and T-04 certified by FrankenSymPy for the M0 kernels; certificates replay offline. | §12.6 |
| GATE-LEMMA-06 | Known answer | M0 | Compile-fail tests show no conversion from a claim product to a factor and no constructor of `Proved` evidence outside `frob-lemma`; every artifact exported by the conformance suite carries the three flags, all false. | §12.6 |
| GATE-MARROW-01 | Known answer | M0 (SO(3) and SE(3)); M1 (complete: every group and the whole law table) | All laws pass on 10⁶ cases each: at most 64 ulp in `f64`; at most 16 ulp in `f32` for single operations. T-01, T-03, T-04 certified (GATE-LEMMA-00). | §13.10 |
| GATE-SPARK-00 | Known answer | M0 | One dialect source for `K-cay-step` compiles and runs on CUDA (`trj`) and on Metal (this Mac) and matches the Rust reference inside its class. | §21.24 |
| GATE-SPARK-01 | Known answer | M0 | IEEE probe per device family: exhaustive bit comparison of `f32` square root and reciprocal over all 2³² inputs; 10⁹ sampled operand pairs for +, −, ×, ÷ with subnormal and overflow edges; the contraction detector; subnormal and NaN behaviour recorded. The outcome sets each device's best attainable determinism class. | §21.24 |
| GATE-SPARK-07 | Known answer | M0 | Fingerprints of both machines are complete (every field measured, or `unavailable` with a reason), repeatable (two runs agree on every discrete field and within 5% on every rate), and reproduce the starting values of §21.16. | §21.24 |
| GATE-SPARK-08 | Known answer | M0 | Exact and guarded kernels pass the boundary corpus and the strict-predicate case on `batch-cpu`, both RTX 4090 and Metal, with identical bytes; the guarded kernel's fallback fraction is reported. | §21.24 |
| GATE-SPARK-21 | Parity | M0 (the `trj` rows that need no agent); M1 (complete) | Incumbents measured on `trj` and on this Mac under the disclosure block: physics and full step for Q and H at matched and native settings; time to threshold on one and two cards; cold start; edit to first step (parameter and topology); CPU MuJoCo at 1, 24 and 36 threads; a Metal-path incumbent. Each published figure of §36.3 reproduced within 20% or the difference explained in writing. | §36.16 |
| GATE-TETHER-03 | Known answer | M0 | Envelope and exit-code conformance on malformed, unknown and over-budget inputs. | §32.21 |
| GATE-WS-01 | Known answer | M0 | Seven plants in a scratch copy (an upward edge, an intra-layer edge against rank, a second asupersync package, an unregistered capsule, an edited generated file, a contract missing a section, a stale stack lock): each caught by the check that owns it and by no other; no failure on the clean tree. | §34.13 |
| GATE-WS-04 | Performance | M0 | The six checks together finish in at most 60 seconds on a warm tree, so that they are run and not skipped. | §34.13 |

**4. Gates of later subsections due at this exit.**

| Gate | Part due at this exit | Statement (from the defining subsection) | Defined in |
|---|---|---|---|
| GATE-WS-05 | whole | Known answer. A golden corpus of 10³ artifacts, including duplicates that differ only by escapes at depths 1 to 8, integers at 2⁵³ ± 1, non-printable identifiers, and decimals whose nearest double differs between parsers: every duplicate and every bad identifier is refused on both surfaces before a job identifier exists; canonical digests are identical across the four feature combinations; the mutant writer that serializes a library map fails under `preserve_order`. M0. | §34.14 |
| GATE-WS-06 | all rows but the sampler row | Known answer. The mixed-wrench probe passes and its unconverted mutant fails; the planted sampler is rejected by rule P-12; `xtask check-deps` reports exactly one `serde_json` version per binary and fails a graph that adds `ftx-drape` to the core workspace; the Metal language version and the 64-bit atomic probe are recorded in the fingerprint. M0; the sampler row at M2. | §33.17 |
| GATE-WS-07 | the two build rows | Known answer. Each of the four planted instances raises its named signal in 100 of 100 runs; `xtask check-deps` resolves the graph with every feature group enabled and fails on the planted second pin; no row of the register lacks a planted instance. M0 for the two build rows, M2 for the sampler row, M3 for the suite row. | §41.5 |
| GATE-WS-09 | all rows but rows 12 and 14 | Known answer. Each of the six admission tests is recorded at the pin with its result: the roll-up of 31.96314202 kg; the refusal at 33 m and the agreement at 1 m to 10⁻¹², with the closed form returning ∓16.5 m; the signed motion witness for both transformer entry points; the arc fixture; the two `fs-sos` probes compared with exact evaluation; the two-plane ray fixture. A probe that now passes upstream is recorded as fixed and its interim posture is reviewed. M0, with rows 12 and 14 at M2. | §33.18 |
| GATE-GAUNT-02 | first run; then every exit | Known answer ¦ Every row of Appendix M that is due at the milestone passes on its tier inside its tolerance, with a derivation file; no row with tolerance `to set` or control `none stated` is counted; the count of such rows is printed and is zero by M3 for rows due by M3 ¦ Every milestone exit | §38.15 |
| GATE-GAUNT-05 | first run; then every exit | Known answer ¦ Oracle hygiene: every fixture has a pin, a date and a capture script; a planted fixture regenerated from the candidate fails CI; no foreign tool appears in any production dependency graph; a planted G2 result offered as Reality is refused ¦ M0, then every exit | §38.15 |
| GATE-GAUNT-08 | first run; then at each funding decision | Known answer ¦ Pilots: no Frontier or Moonshot work package is funded without a pilot record that has a falsifier, a computed ceiling, a fallback and a result; the plan-time pilots of §38.5 re-run at M0 and reproduce every count exactly and every timing ratio inside its recorded band ¦ M0, then at each funding decision | §38.15 |
| GATE-GAUNT-10 | first run; then every exit | Known answer ¦ Status accounting: the README table equals a fresh generation from `gates.toml`; a planted self-certification and a planted PASS with an uncaught control are each refused ¦ M0, then every exit | §38.15 |

**5. Program gate.** GATE-TENFOLD-01, the parity-baseline gate (bead N40-12): the rival audit and the incumbent
phases that need no agent, measured on `trj` (the M0 rows of GATE-SPARK-21). If it is not met, M0 does not exit; the
thesis arithmetic stays a HYPOTHESIS and no work package is funded as a speed lever. The whole of GATE-SPARK-21
closes at M1, and its rows on the Mac close no later than M1. GATE-GAUNT-01 holds at this and every exit.

**6. Kill gates decided here (each recorded as an ADR).**

| Kill gate | Trigger | Fallback, named in advance | Decided | Owner (consumer) |
|---|---|---|---|---|
| KILL-FSIM | INT-FSIM-01 cannot pass within two weeks of M0 effort, or its cold check exceeds the machine envelope, or upstream breakage exceeds 20% of working days in any four-week window (BET-04) | (a) One pinned FrankenSim worker process per device exchanging whole jobs through the airlock traits. (b) Only if (a) fails INT-FSIM-18 or the pin can no longer be built: lift the minimal referee set with provenance headers and golden tests, and state lift-only | M0, then rolling | §33.8 |
| KILL-MCP | INT-FMCP-01 cannot pass within the M0 budget after FD-FMCP-01 and FD-ASUP-01 have been attempted | Fallback 1: `frob-mcp` as its own workspace, talking to `frobd` over a local socket in newline-delimited JSON. Fallback 2: agents use the CLI, which is complete on its own (§33.13, §40.7) | M0; re-read at M1 with the M1 verbs | §32.21 |

KILL-MCP is decided between M0 and M1 and re-read at M1 with the M1 verbs. The choice of T and the decision on a
local units crate (`frob-units`, built only if forced) follow from the recorded toolchain matrix.

**7. Exit rule.** The exit is recorded by a fresh-context session that did not implement the milestone (seed bead
BD-044, procedure N40-05). The exit line is: *the admission records, one kernel on three backends, and the incumbent
phase ledger on `trj`.* Every gate above has a recorded value (a failing value is a record; a failed admission is
the result, and its fallback is decided here); every demonstration runs from a clean checkout through `frob --json`
and `frob-mcp`; GATE-GAUNT-01 holds; the risk register has been read, each risk's earliest signal recorded as fired
or not; and the open questions due at M0 (§9) have been answered or have taken their fail-closed defaults, each
default recorded as an ADR (N40-05, steps 9 and 10). The `reality:` line of GATE-GROUND-01 is required from M1 on.

**8. Dependency firewall.** Nothing is relied on. T is chosen. FrankenSim, asupersync, FrankenSQLite, FrankenTorch's
Metal gateway, `cudarc`, FrankenSymPy, fastmcp_rust and franken_numpy are admitted. The two P0 upstream items are
done in their repositories, or KILL-MCP is decided.

**9. Owner input wanted.** OPEN-SPARK-1 (ratify GPU backends in annex workspaces; default: GPU results are labelled
exploratory, and M2 exits in its no-GPU variant). OPEN-SPARK-2 (the compute envelope classes E0 to E3; default E0:
16 GiB, 8 cores, one hour, one device). OPEN-PROG-1 before the first push (repository visibility and licence;
default: private). OPEN-SUITE-3 and OPEN-SUITE-5 (dirty sibling checkouts acceptable for development with the tree
hash in the receipt and no PASS recorded; the cold-build budget is the machine envelope). OPEN-PERF-2 (comparator
revisions; default: the revisions in the source register, frozen before any paired run).

**10. Stop or re-plan.** Re-plan before M1 if INT-FSIM-01 cannot pass within two weeks and the worker process also
fails INT-FSIM-18, because the only route left is a lift from a closure of about 1.5 million lines (FACT, measured
in the sibling). Rewrite the thesis chapter before any work is funded against it if the measured ledger puts the
ceilings under 3 for every mission class. Drop a device family from M2 if the probe shows that it can hold no
controlled determinism class. None of these stops the CPU studio except the first.

**11. No single failed bet blocks the exit.** BET-04 fails → the worker process; the lift only if that fails.
INT-FMCP-01 cannot pass → KILL-MCP: `frob-mcp` as its own workspace over a local socket to `frobd`; then the CLI,
which is complete on its own.

**12. Work packages.**

| Package | Delivers | Crates | Milestones | Key gates |
|---|---|---|---|---|
| WP-01 | Bootstrap and stack: the one toolchain T and the stack lock | workspace, `xtask`, `frob-core` | M0 | INT-TOOLCHAIN-01; GATE-WS-01, -04 |
| WP-02 | Airlock and referee smoke; one asupersync version across the consumed siblings (upstream work item FD-ASUP-01) | `frob-fsim`; upstream in frankensim, frankentorch and fastmcp_rust | M0 | INT-FSIM-01, INT-ASUP-01, INT-FSQL-01 |
| WP-03 | Marrow core | `frob-lie`, `frob-algebra` | M0, M1 | GATE-MARROW-01, -02 |
| WP-04 | Spark spikes | the kernel dialect, `spark/frob-spark-cuda`, `spark/frob-spark-metal`, the backend trait | M0 | INT-CUDA-01, INT-FT-01, -02; GATE-SPARK-00, -01 |
| WP-05 | Derivation spike | `frob-kir`, `derive/frob-derive`, `frob-lemma` | M0 | INT-FSYM-01, -02; GATE-LEMMA-00 |
| WP-06 | Verb registry and surfaces; fastmcp_rust brought onto T (upstream work item FD-FMCP-01), with `frob-mcp` in the core graph running verbs in process | `frob-verbs`, `frob-cli`, `surface/frob-mcp`; upstream in fastmcp_rust | M0, M1 | INT-FMCP-01; GATE-TETHER-02, -03, -07 |
| WP-34 | Incumbent phase ledger and parity baseline | `frob-conformance`, `tools/oracle` | M0, M1 | GATE-SPARK-21 |
| WP-35 | Hardware fingerprint; exact and guarded kernels | `frob-kir`, `frob-kernels`, the Spark annexes | M0 | GATE-SPARK-07, -08 |
| WP-36 | Claim product, flags and the offline auditor | `frob-core`, `frob-cert` (binary `frob-verify`) | M0, M1 | GATE-LEMMA-06, -03; GATE-WS-03; GATE-SAFE-01, -08 |
| WP-37 | Worker fallback, file-by-file admission tests, the plan-time pilots | `frob-fsim` (binary `frob-fsim-worker`), `worker/`; `frob-units` only if forced | M0 | INT-FSIM-02 to -11, INT-FSIM-18; INT-FNP-01, -02 |

## Acceptance criteria

- [ ] Every admission gate in the table has a record in `DEPENDENCY_ADMISSION.md` and a row in the register,
      passing or failing, at the pin of the day.
- [ ] GATE-MARROW-01 for SO(3) and SE(3); GATE-SPARK-00, -01, -07, -08; GATE-LEMMA-00, -06; GATE-TETHER-03;
      GATE-WS-01, -04; and the `trj` no-agent rows of GATE-SPARK-21 have measured values and caught controls.
- [ ] GATE-TENFOLD-01 is recorded; FD-FMCP-01 and FD-ASUP-01 have landed in their repositories, or KILL-MCP is
      decided.
- [ ] KILL-FSIM and KILL-MCP each have an ADR with an outcome.
- [ ] The late gates GATE-WS-05, the M0 rows of GATE-WS-06, -07, -09, and GATE-GAUNT-02, -05, -08, -10 are recorded.
- [ ] The exit report (N40-04) for M0 is `exit-ready` and the exit record is written by a fresh-context session.

## Test plan

### Unit tests

Carried by the member beads. The epic's own check is `exit::due_counts_match_load_table` for M0: 23 admission
parts, 11 convergence parts, 2 kill decisions, plus 8 late parts.

### End-to-end script and logging

`scripts/exit/review.sh M0` (N40-05) from a clean clone: `frob doctor --json`; `frob sim run g1 --tier referee
--seconds 2 --json` with the digest compared; the same job through the worker; `frob spark probe --json` and
`frob spark fingerprint --json` on both machines; the three-backend `K-cay-step` comparison; `frob-verify` on the
T-01, T-03, T-04 bundles; each M0 verb through `frob-mcp`. The transcript logs every command, exit code, wall
time, the envelope's evidence block, digests compared, and thresholds beside measured values, and ends with a
summary naming the gates touched.

### Known answers and negative controls

The G1 referee digest (equal to FrankenSim's own); bit-equal predicate kernels on three backends; the plan-time
pilot counts reproduced exactly. Negative controls at this exit: the seven workspace plants of GATE-WS-01; a
deliberately reversed adjoint (INT-FSIM-02); a planted self-certification and a PASS with an uncaught control
(GATE-GAUNT-10).

## Considerations

- M0 is two weeks on paper and carries 23 admissions; the risk is breadth, not depth. Admissions fail closed: a
  foundation without a record is not used.
- The rival measurement needs both RTX 4090 cards of `trj` and exceeds the default compute envelope; without the
  owner's approval the single-card and eight-core rows are recorded and the rest are `not run`.
- A dirty sibling tree may be built against, but no PASS is recorded from it.

## References

Plan §43.3, §43.1, §43.2, §40.2, §40.3, §40.4, §40.5, §40.6, §43.12. Bets BET-04, BET-13. Upstream items
FD-FMCP-01, FD-ASUP-01. Work packages WP-01 to WP-06, WP-34 to WP-37.
