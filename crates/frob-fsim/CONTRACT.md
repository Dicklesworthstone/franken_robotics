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

- Adapter bodies for every group land only with their owning admission beads (`BD-007`, `BD-009`, `N33a-14..19`, `N33a-30..31`, `N33b-11..16`).
- Upstream refusal conversion (`FROB-REF-792`) lands in `N33a-13`; `mode-remote` client and mutually-exclusive mode compile guards land in `N33c-11`.
- Verb-level `needs_groups` checking, `admitted` status, and ranked-repair envelope attachment land in `N33c-32`; `check-deps` airlock consumer allow-lists land in `N34-03` / `N33b-09` / `N33c-29`.
- Upstream `FD-ASUP-01`: `../frankensqlite/Cargo.toml` currently pins `asupersync = "0.6.2"` while `../asupersync` is `0.7.0`, so enabling `ledger` (in `default`) currently pulls both `0.6.2` and `0.7.0` until `FD-ASUP-01` aligns `frankensqlite`.
