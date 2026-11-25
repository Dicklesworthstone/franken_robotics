# Contract: `frob-fsim`

## Purpose

`frob-fsim` is the Layer 0 (rank 2, Solid tier, 20 kLOC cap) airlock crate
confining FrankenSim's volatile path dependencies behind FrankenRobotics-owned
traits, a static feature-group registry, and structured `FROB-REF-790` refusals
(`ADR-SUITE-01`, `INV-SUITE-01`, `BD-006`, `N33a-11`, gate `INT-FSIM-01`).

- **Pinned FrankenSim revision (`franken-stack.lock`):** `08e9195c6faea3363377b7fdfb6d6b07f5b552f3`
- **Admitted `INT-FSIM-01` core smoke crates (`BD-006`, `default-features = false`):**
  `fs-contact`, `fs-ga`, `fs-mbd`, `fs-query`, `fs-time` (none requires extra Cargo features).
- **Ten airlock feature groups** (plus the M1 `optimize-race` sub-feature per
  `N33a-11` Review Note 1, and test-only `fs-ir` inside `harness` per Review Note 2):

| Feature | Default | Traits exposed | FrankenSim crates behind it | Allowed consumers | Gate |
|---|---|---|---|---|---|
| `referee` | on | `RefereeDynamics`, `RefereeContact`, `Integrator`, `PortSystem` | `fs-mbd`, `fs-mbd-material`, `fs-contact`, `fs-tribo`, `fs-time`, `fs-motion`, `fs-ports`, `fs-system` | `frob-model`, `frob-dyn`, `frob-contact`, `frob-analyze`, `frob-verbs`, `frob-conformance` | `INT-FSIM-01` |
| `geometry` | on | `GeometryKernel`, `SceneQuery`, `MeshExchange` | `fs-query`, `fs-geom`, `fs-rep-sdf`, `fs-rep-mesh`, `fs-rep-frep`, `fs-scene`, `fs-topo`, `fs-io` | `frob-contact`, `frob-scene`, `frob-foundry`, `frob-conformance` | `INT-FSIM-01` |
| `ledger` | on | `Ledger`, `Session`, `Recompute` | `fs-ledger`, `fs-session`, `fs-package`, `fs-recompute` | `frob-chronicle`, `frob-verbs` | `INT-FSQL-01` |
| `optimize` | off | `Optimizer`, `Archive`, `Racing` | `fs-dfo`, `fs-bo`, `fs-ascent`, `fs-archive`, `fs-opt`, `fs-sos`, `fs-race`, `fs-eproc`, `fs-tropical`, `fs-symmetry` | `frob-opt`, `frob-arena`, `frob-dyn`, `frob-mission`, `frob-analyze`, `frob-nemesis` | `INT-FSIM-14` |
| `optimize-race` | off (sub-feature of `optimize`) | `Racing` | `fs-race`, `fs-eproc` | `frob-opt`, `frob-arena`, `frob-dyn`, `frob-mission`, `frob-analyze`, `frob-nemesis` | `INT-FSIM-14` |
| `stats` | off | `Uncertainty`, `ValueOfInformation`, `Surrogate`, `Assimilation` | `fs-uq`, `fs-voi`, `fs-surrogate`, `fs-assimilate`, `fs-robust`, `fs-regime` | `frob-nemesis`, `frob-arena`, `frob-ground`, `frob-opt`, `frob-mission`, `frob-dyn` | `INT-FSIM-14` |
| `spectral` | off | `Spectral` | `fs-spectral` | `frob-gait`, `frob-lifecycle` | `INT-FSIM-13` |
| `actuation` | off | `Magnetics`, `ThermalNetwork`, `CurrentLoop`, `PortHamiltonian` | `fs-em`, `fs-thermal`, `fs-control`, `fs-phs`, `fs-matdb` | `frob-actuate` | `INT-FSIM-09` |
| `structure` | off | `StructuralOpt`, `ToleranceAllocation`, `AsBuilt`, `FabricationRules` | `fs-solid`, `fs-topopt`, `fs-topols`, `fs-toleralloc`, `fs-asbuilt`, `fs-geocon`, `fs-fab`, `fs-adjoint` | `frob-foundry`, `frob-ground` | `INT-FSIM-15` |
| `multiphysics` | off (gated M6 bridge) | `FluidIsland`, `CoupledIsland` | `fs-lbm`, `fs-couple` | `frob-scene` (evaluation tier only) | `INT-FSIM-17` |
| `harness` | off (dev-only) | none (test/oracle support) | `fs-roofline`, `fs-detaudit`, `fs-propcheck`, `fs-render`, `fs-la` (oracle only), `fs-ir` (test only) | dev-dependencies of `frob-fsim` and `frob-conformance` | `INT-FSIM-01` |

Non-group features declared in `Cargo.toml` and excluded from `Group`, `BUILT_WITH`, and `contract.toml`:
`mode-inproc`, `mode-remote` (enables no group), and `conformance = ["mode-inproc"]` (off by default,
enabling `frob_fsim::conformance` for `frob-conformance` only).

## Inputs

- Slice of required runtime groups `&[Group]` passed to `require(...)`.
- Compile-time Cargo feature flags (`default`, per-group features, `optimize-race`, `mode-inproc`, `mode-remote`, `conformance`).
- Compile-time build-identity environment (`FRANKENSIM_DEPGRAPH_RECEIPT` or `FRANKENSIM_DEPGRAPH_SALT`).
- Trivial analytic inputs supplied internally by the `INT-FSIM-01` smoke entry points.

## Outputs

- `BUILT_WITH: &[Group]`: compile-time compiled-in runtime group slice (identical in debug and release for the same feature set).
- `GROUP_REGISTRY: &[GroupDescriptor]`: static metadata mirroring `crates/frob-fsim/contract.toml`.
- `Result<(), Refusal>` from `require(...)` and `Result<(), GroupAbsent>` from disabled group stub constructors.
- `BuildIdentity` and `CrateSmokeResult` from the `BD-006` admission entry points.

## Invariants

- **Airlock confinement (`INV-SUITE-01`, rule `L-2`):** No volatile FrankenSim crate or type is publicly re-exported or named in any public signature of `frob-fsim`.
- **Single data authority:** `crates/frob-fsim/contract.toml` `[features.*]` and `GROUP_REGISTRY` match `Cargo.toml` `[features]` (excluding `default`, `mode-inproc`, `mode-remote`, and `conformance`).
- **In-process implication:** Every group feature (and `optimize-race`) implies `mode-inproc`; `mode-remote` enables no group; `conformance` implies only `mode-inproc` and is enabled by no manifest outside `frob-conformance`.
- **Zero-cost refusal before work:** `require` checks `BUILT_WITH` in declared order and returns `FROB-REF-790` on the first absent group without charging any budget.
- **Disabled stubs are pure:** Every disabled group's stub constructors return `Err(GroupAbsent { group })` and reference zero `fs-*` crates.
- **Fail-closed build identity (`BD-006`, `N33a-08`):** `frob-fsim` has no `build.rs` and never synthesizes a fallback `FRANKENSIM_DEPGRAPH_SALT` or `FRANKENSIM_DEPGRAPH_RECEIPT`.
- **No `unsafe`, no library stdout/stderr (`L-7`):** `#![forbid(unsafe_code)]` is enforced at crate root and library code prints nothing.

## Refusals

- `FROB-REF-790` (`foundation-group-absent`, exit class 5, `retry: after-change`): raised by `require` (and via `GroupAbsent::into_refusal`) when a requested airlock group is not compiled into this binary. Carries `pointer = "foundation/<group>/compiled"`, `observed` = `BUILT_WITH`, `required` = missing group name. Ranked repairs live in `registries/refusals.toml` (`N33-REF`) and are attached by the verb envelope builder (`N33c-32`).

## Determinism class

- Metadata, group registry, `BUILT_WITH`, `require`, stub constructors, and `INT-FSIM-01` smokes: `D0` (bit-identical across runs and identical between debug and release for the same feature set).
- Per-group adapter determinism claims (populated by downstream adapter beads):
  - `referee`, `geometry`, `actuation`, `structure`, `multiphysics`, `spectral`: same-platform determinism (`D1`/`D2` per upstream FrankenSim platform `sqrt`/trig contracts).
  - `ledger`: `D0` for content-addressed artifact/session digests and recompute graph invalidation.
  - `optimize`, `optimize-race`, `stats`: deterministic given explicit addressed random streams (`N35-22`).

## Evidence class of outputs

- `Tested` for the `INT-FSIM-01` out-of-tree closure type-check and link claim (`BD-006`) and for the `N33a-11` group-registry / stub / `FROB-REF-790` invariants.
- Downstream adapter outputs carry the evidence class conferred by FrankenSim's own contracts and their respective admission records (never inherited; an adapter body lands only with its covering gate record).

## Validity domain

- Applies strictly to the pinned FrankenSim checkout at `../frankensim` under the workspace toolchain declared in `rust-toolchain.toml` and the three repeated root settings in `Cargo.toml` and `.cargo/config.toml`.
- Re-read FrankenSim group closure sizes (2026-11-24): default (`referee` + `geometry` + `ledger`) = 67 FrankenSim crates (2.12M lib lines); `mode-inproc` alone = 0 volatile `fs-*` crates; `spectral` = 14 crates (no `fs-lbm`); `optimize-race` = 8 crates; `optimize` = 42 crates; `stats` = 42 crates; `actuation` = 43 crates; `structure` = 61 crates; `multiphysics` = 68 crates (4.19M lib lines).
- Deliberately excluded FrankenSim crates: `fs-conform`, `fs-truss`, `fs-g1-train` (not admitted), `fs-checker`, `fs-tilelang`, `fs-soa` (pattern only).

## Oracles

- Testing conventions and evidence discipline: `docs/testing/CONVENTIONS.md`.
- `crates/frob-fsim/tests/groups.rs`: verifies `BUILT_WITH` across default, no-group (`mode-inproc`), and optional-group builds, `FROB-REF-790` fields and exit class 5 against `registries/refusals.toml`, disabled stub `GroupAbsent` returns, manifest-vs-`contract.toml` parity, `conformance` isolation (including planted `frob-verbs` fixture), public-API volatile-type scan, and `multiphysics` non-default check.
- `crates/frob-fsim/tests/fixtures/frob_dyn_enables_structure/Cargo.toml`: planted negative-control fixture shipped here for `N33c-29`'s `airlock_fixture_frob_dyn_structure_refused` check-deps test (Review Note 3).
- `crates/frob-fsim/tests/admission_int_fsim_01.rs`: five-crate analytic smokes, build-identity check, `asupersync` count, and closure-count guard.

## Known gaps

- Remaining adapter bodies for other groups land only with their owning admission beads (`BD-009`, `N33a-14..19`, `N33a-30..31`, `N33b-11..16`).
- Upstream refusal conversion (`FROB-REF-792`) lands in `N33a-13`; `mode-remote` client and mutually-exclusive mode compile guards land in `N33c-11`.
- Verb-level `needs_groups` checking, `admitted` status, and ranked-repair envelope attachment land in `N33c-32`; `check-deps` airlock consumer allow-lists land in `N34-03` / `N33b-09` / `N33c-29`.
- Upstream `FD-ASUP-01`: `../frankensqlite/Cargo.toml` currently pins `asupersync = "0.6.2"` while `../asupersync` is `0.7.0`, so enabling `ledger` (in `default`) currently pulls both `0.6.2` and `0.7.0` until `FD-ASUP-01` aligns `frankensqlite`.

## `RefereeDynamics` (`BD-007`, feature `referee`)

In-process adapter `frob_fsim::referee::InProcRefereeDynamics` over FrankenSim's `fs_mbd::articulated` (compiled strictly under `#[cfg(feature = "referee")]`; zero volatile FrankenSim types in any public signature).

- **Inputs:**
  - `catalog_model(CatalogRobot)`: v0 supports `CatalogRobot::G1` (Unitree G1 mode-11 29-DoF reduced catalog humanoid, 30 retained links, `nq = 36`, `nv = 35`, `actuators = 29`, total mass `31.96314202` kg).
  - `forward_dynamics(model, state, tau)` and `step_free(model, state, tau, h)`: `RefereeState { q: Vec<f64>, v: Vec<f64> }` (base world position `[x, y, z]` in `m`, base unit quaternion `[w, x, y, z]`, tree-order joint positions in `rad`/`m`; body-frame base angular velocity in `rad/s`, body-frame base linear velocity in `m/s`, tree-order joint velocities in `rad/s`/`m/s`), actuator torques `tau: &[f64]` in `N·m`/`N` (`len == actuators`), and strictly positive exact rational timestep `h: Rational` (converted to `f64` as `(h.num() as f64) / (h.den() as f64)`; standard M0 rollout step `1/480` s = `1.0 / 480.0`), under standard world gravity `[0, 0, -9.80665]` `m/s^2` and zero external body wrenches.
- **Outputs:**
  - `catalog_model`: opaque airlock-owned `RefereeModel` handle.
  - `reference_state`: stationary identity-base neutral-joint `RefereeState`.
  - `forward_dynamics`: generalized accelerations `Vec<f64>` (`len == nv`: body-frame base angular/linear acceleration followed by joint accelerations).
  - `step_free`: contact-free next `RefereeState` advanced by one Lie-midpoint step of duration `h`.
  - `describe`: `ModelFacts` reporting `links`, `nq`, `nv`, `actuators`, `mass_kg`, and self-describing `CoordLayout` read dynamically from FrankenSim at the pin.
  - Canonical digest (`frob_fsim::referee::digest`, depending only on `std` + bedrock `fs-blake3`): 32-byte BLAKE3 rendered as 64 lower-case hex digits over `b"frob.referee-state/1\0"`, `(links, nq, nv)` as `u32` LE, `q` then `v` as IEEE-754 `f64::to_bits().to_le_bytes()` (`-0.0` distinct from `+0.0`), and payload tag `0x00` (plain state / rollout) or `0x01` + `qdd` (forward dynamics).
- **Refusals:**
  - Typed `RefereeRefusal` with machine-readable `RefereeRefusalKind`:
    - `NonFinite { field, index }`: refuses any `NaN`, `+inf`, or `-inf` in `q`, `v`, or `tau` **before** calling FrankenSim.
    - `LengthMismatch { field, expected, actual }`: refuses wrong `q` (`!= nq`), `v` (`!= nv`), or `tau` (`!= actuators`) lengths **before** calling FrankenSim, naming both expected and supplied lengths in `reason`.
    - `InvalidStep`: refuses zero or non-finite rational step durations **before** calling FrankenSim.
    - `Upstream`: converts every upstream FrankenSim error into a typed refusal preserving the upstream reason text; never retries with weakened inputs.
  - When compiled without feature `referee`, `new_referee_dynamics()` returns `Err(GroupAbsent { group: Group::Referee })` (`FROB-REF-790`).
- **Determinism class:**
  - Same-platform, same-binary bit equality only (`D1`/`D2` per upstream FrankenSim platform square-root/trig contracts; cross-platform and debug-vs-release equality are not claimed for the referee). The canonical digest encoding itself is `D0`.
- **Evidence class of outputs:**
  - `Tested` (v0 outputs verified against the pinned in-tree FrankenSim oracle capture for `INT-FSIM-01`).
- **Validity domain:**
  - Contact-free articulated tree kinematics and free-floating dynamics over FrankenSim's supported joint classes: **fixed, revolute, prismatic, and helical** joints, evaluated strictly at the pinned FrankenSim revision (`franken-stack.lock`, `FD-FSIM.git_rev`). The G1 catalog model is a 30-link reduced profile (`31.96314202` kg; four welded links totaling `1.377` kg omitted relative to the 38-link `33.34014202` kg import-parity reference) and is never used as an import-parity reference.
- **Oracle:**
  - In-tree FrankenSim oracle probe (`crates/frob-fsim/intree-probe/`, driven by `tools/oracle/fsim_g1_digest.sh`), executed inside a read-only `git archive` export of FrankenSim at the pinned commit under FrankenSim's own toolchain and `.cargo/config.toml`, hashing with the byte-identical `crates/frob-fsim/src/referee/digest.rs` source.
  - Checked-in authenticated fixture `tests/fixtures/oracle/fsim/g1_fd_digest.toml` plus append-only capture log `tests/fixtures/oracle/fsim/g1_fd_digest.capture.log` (bound via `capture_log_blake3`, `digest_source_blake3`, `rustc -vV`, `fd_digest`, and `rollout_2s_digest`), verified by `crates/frob-fsim/tests/referee.rs` (including ULP, torque, and four planted capture-record negative controls).
- **Known gaps:**
  - **No contact** (`RefereeContact` lands in `N33a-14`).
  - **No adjoints / automatic differentiation** (plain `f64` only, not generic over a differentiation scalar; no articulated adjoints).
  - **`f64` only** and single-world unbatched execution; the underlying Lie-midpoint integrator is neither energy-exact nor variational/symplectic.
  - Full `FROB-REF-792` upstream refusal mapping lands in `N33a-13`; compiled design lowering lands in `BD-061`; worker-process remote serving lands in `BD-038`/`N33c-11`.
