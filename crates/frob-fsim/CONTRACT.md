# Contract: `frob-fsim`

## Purpose

`frob-fsim` is the Layer 0 (rank 2) airlock crate confining FrankenSim path
dependencies behind FrankenRobotics-owned types and entry points (`ADR-SUITE-01`,
`INV-SUITE-01`, bead `BD-006`, gate `INT-FSIM-01`).

- **Pinned FrankenSim revision (`franken-stack.lock`):** `08e9195c6faea3363377b7fdfb6d6b07f5b552f3`
- **Admitted FrankenSim crates (`INT-FSIM-01`, `default-features = false`):**
  1. `fs-contact` (`../frankensim/crates/fs-contact`)
  2. `fs-ga` (`../frankensim/crates/fs-ga`)
  3. `fs-mbd` (`../frankensim/crates/fs-mbd`)
  4. `fs-query` (`../frankensim/crates/fs-query`)
  5. `fs-time` (`../frankensim/crates/fs-time`)

None of these five crates requires any non-default Cargo feature to compile.
Volatile FrankenSim types are never re-exported publicly; callers above Layer 0
import only FrankenRobotics types owned by `frob-fsim`.

## Inputs

- Compile-time build-identity environment (`FRANKENSIM_DEPGRAPH_RECEIPT` for
  evidence-bearing builds, or `FRANKENSIM_DEPGRAPH_SALT` supplied by the
  workspace `.cargo/config.toml` for development builds).
- Trivial analytic inputs supplied internally by the `INT-FSIM-01` smoke entry
  points (`smoke_fs_contact`, `smoke_fs_ga`, `smoke_fs_mbd`, `smoke_fs_query`,
  `smoke_fs_time`) and workspace metadata in integration tests.

## Outputs

- `BuildIdentity`: compile-time evidence of which build-identity environment
  variable was present (or `None`, fail-closed, when absent).
- `CrateSmokeResult`: structured pass/fail result identifying each admitted
  FrankenSim crate and exercised public item, containing no bare floating-point
  fields and no FrankenSim types.

## Invariants

- **Airlock confinement (`INV-SUITE-01`, rule `L-2`):** `frob-fsim` never
  publicly re-exports (`pub use` / `pub extern crate`) any FrankenSim crate or
  type.
- **Fail-closed build identity (`BD-006`, `N33a-08`):** `frob-fsim` has no
  `build.rs` and never synthesizes a fallback `FRANKENSIM_DEPGRAPH_SALT` or
  `FRANKENSIM_DEPGRAPH_RECEIPT`. Unsetting both variables causes FrankenSim's
  `fs-la/build.rs` to panic (`fz2.6`) and stop compilation.
- **Single `asupersync` package (`INT-ASUP-01`):** workspace resolution links
  exactly one `asupersync` package.
- **No `unsafe`, no library stdout/stderr (`L-7`):** `#![forbid(unsafe_code)]`
  is enforced at crate root and library entry points never print.

## Refusals

No runtime refusal codes are registered by `frob-fsim` at `BD-006`. Feature-group
refusal `FROB-REF-790` (disabled airlock feature group) is added by `N33a-11`,
and referee dynamics / digest refusals are added by `BD-007`.

## Determinism class

`D0` (bit-identical across runs on the same platform and toolchain). All five
admission smoke functions evaluate pure deterministic arithmetic with no wall
clock, no random seed, and no ambient I/O.

## Evidence class of outputs

`Tested` for the out-of-tree closure type-check and link claim (`INT-FSIM-01`
at the pinned FrankenSim commit and recorded toolchain). Makes no physics
fidelity or referee parity claim (covered separately by `BD-007`).

## Validity domain

- Applies strictly to the pinned FrankenSim checkout located at sibling path
  `../frankensim` under the single workspace toolchain declared in
  `rust-toolchain.toml`, with the three repeated root settings in `Cargo.toml`
  and `.cargo/config.toml`.
- Covers only the five crates listed in Purpose; additional FrankenSim crates
  and feature groups remain un-admitted until their owning beads (`N33a-11`,
  `N33a-12`, `BD-007`, etc.) land.

## Oracles

- Testing conventions and evidence discipline: `docs/testing/CONVENTIONS.md`.
- Analytic closed-form expected values for the five `INT-FSIM-01` smoke checks
  (`e_x × e_y = e_z`, `(1,2,3)·(4,-5,6) = 12`, constant-velocity closing contact
  `t = 2.0 s`, 1-step Stormer-Verlet harmonic oscillator `q_1 = 0.995, p_1 = -0.09975`,
  orthogonal point-to-triangle projection `d = 2.0`).
- Live workspace `cargo metadata --format-version 1 --locked --offline` cross-checked
  against the recorded `INT-FSIM-01` closure count and `asupersync` uniqueness.
- Negative control `NC-INT-FSIM-01-unset-build-identity`: build fails closed
  when `FRANKENSIM_DEPGRAPH_SALT` and `FRANKENSIM_DEPGRAPH_RECEIPT` are unset.

## Known gaps

- Referee multibody dynamics (`RefereeDynamics` over `fs_mbd::articulated`) and
  the G1 bit-parity digest test against FrankenSim's own tree land in `BD-007`.
- The ten airlock feature groups, group registry, and `FROB-REF-790` land in
  `N33a-11`.
- The `INT-FSIM-01` gate row in `registries/gates.toml` is recorded by
  `N33a-28` once both halves (`BD-006` and `BD-007`) are complete.
- Upstream sibling hazards (dirty `../frankensim` tree, `constellation.lock`
  staleness, and `fs-la/build.rs` build-identity requirement) are tracked by
  `FD-FSIM-14`, `FD-FSIM-16`, and `KILL-FSIM` (`docs/delivery/kill-fsim/ledger.toml`).
