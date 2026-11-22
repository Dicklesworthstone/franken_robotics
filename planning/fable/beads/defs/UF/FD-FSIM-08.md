---
key: "FD-FSIM-08"
title: "upstream frankensim: the `Component` trait of `fs-ports` frozen as contract v1, with a conformance kit"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "suite", "skeleton", "upstream", "frankensim", "frob-fsim", "solid"]
parent: "WP-07"
deps: ["BD-037", "BD-007", "SPEC-C-12", "NF-01", "STD-E2E"]
needs: []
related: ["BD-061", "BD-077", "BD-060", "BD-068", "BD-076"]
gates: ["INT-FSIM-09"]
plan_refs: ["Appendix F.1", "Appendix F.3", "§33.2", "§33.3", "§10 C-12", "§41", "§33.8"]
---
## Background

In FrankenRobotics a robot is a composition over typed ports (construction C-12). Components expose ports: a
mechanical port carries a twist and a wrench at a frame; a shaft port carries (ω, τ); an electrical port (V, I);
a thermal port (T, Q̇); a signal port a typed message with a rate and a latency bound. Interconnection is
power-conserving: the port powers of a composition sum to zero, and an algebraic loop is refused. Five functors
give a design its meanings (kinematic, dynamic, geometric, budget, symmetry), and the dynamic functor lands on
FrankenSim's port-Hamiltonian components through `fs_ports::component::Component`.

FACT (FrankenSim at `483d09c682f2`, read 2026-11-21): that trait is contract **v0, not frozen**:
`COMPONENT_CONTRACT_VERSION` is 0 (`frankensim/crates/fs-ports/src/component.rs:52`); it offers `ports()`,
`class()` and `stored_energy_j()`; `fs-system::SystemBuilder` composes components; `fs-mbd` already wraps a
rigid body as a `Component`. An unfrozen composition contract is hazard 2 of the decision to consume FrankenSim
by path, and the first named cause of RISK-01 (FrankenSim API churn in `fs-mbd` and the `Component` trait;
severity high). Upstream register priority: **P1**.

Rejected: defining a parallel component trait here and never touching FrankenSim's (two composition contracts,
with the referee on the other one); pinning forever to the commit whose v0 we adapted (a pin that cannot move).

## Scope

In: in FrankenSim, the trait frozen as v1 with a written contract, a versioning rule and a conformance kit;
here, the airlock adapter retargeted and the functor laws re-run. Out: the airlock trait `PortSystem` itself
(BD-007 and its successors); module files and their port contracts (BD-060); the invalidation contract of
substitution (BD-077); constrained interconnections, which FrankenSim defers.

## Technical approach

**Where the work is done, and under whose rules (binding).** The change is made in the `frankensim` repository
(`../frankensim` beside this one; head `483d09c682f2` and toolchain channel `nightly-2026-07-06` when this was written
on 2026-11-21, both FACT to re-read), by an agent of this program working there under that repository's own
`AGENTS.md`, gates and conventions. For FrankenSim those rules include its layer checks, its contracts, its unsafe-capsule discipline, its ban on
in-process foreign kernels in production paths and its ban on branches and worktrees (FACT:
`frankensim/AGENTS.md`, `frankensim/docs/ACCELERATOR_DOCTRINE.md`), so the work is committed on `main` in small
steps that each pass its gates. FrankenSim moves fast (FACT: 409 commits touched `crates/fs-mbd` in the thirty
days to 2026-11-21, and its checkout had 32 modified paths), so every file named in this bead is re-read at the
current head before it is edited.

1. Read that repository's `AGENTS.md` and its coordination rules first. They govern; where they differ from this
   bead, they win, and the difference is written into this bead's closing note.
2. Other agents work in the same repository. Before editing, reserve every file to be touched through Agent
   Mail and follow the repository's own coordination rules; release the reservations once the change has landed.
3. The change is additive (DESIGN): a new mode, entry point, type or feature. Existing behaviour is not changed,
   and any lossiness is self-described in the new item's documentation and return type.
4. It lands on that repository's `main` through its own tests and gates. Nothing in this repository patches,
   forks, vendors or edits the sibling, and no tool of this repository writes into its checkout (INV-SUITE-04).
5. The work is tracked twice: this bead, and a mirrored bead in the sibling's tracker whose identifier is stored
   in the `mirror` field of this item's row in `registries/upstream.toml` (schema `frob.upstream/1`). The row's
   `status` moves `open` → `landed` (merged on the sibling's `main`) → `adopted` (switch-over run here).

**The change (DESIGN; the contract's wording is FrankenSim's).** `COMPONENT_CONTRACT_VERSION` becomes 1, and
the contract document of `fs-ports` states, for v1:

1. the method set and the meaning of each method (`ports()`: the typed port list in a stable order; `class()`;
   `stored_energy_j()`: non-negative, in joules);
2. the port types, their effort and flow variables with units, and the sign convention of port power (one
   convention for every port type, stated once);
3. the laws a component must satisfy: the sum of port powers plus the rate of stored energy plus dissipation is
   zero; dissipation is non-negative for a component declared passive; `ports()` is pure;
4. the versioning rule: v1 is never changed; a later capability is a new trait or an extension trait, and
   `COMPONENT_CONTRACT_VERSION` moves only with a new contract document;
5. a conformance kit: a function any implementation can call in its tests, which drives the component with
   seeded port inputs and checks the laws.

Additive: v0 implementations compile unchanged if v1 is the v0 method set made precise; if a v0 ambiguity must
be resolved in a way that changes a method, v0 stays available under its own name during a stated window.

**Interim posture here.** The airlock adapts v0: `frob-fsim` exposes `PortSystem` (feature group `referee`, over
`fs-ports` and `fs-system`) and converts. A change to v0 costs that one crate and is counted in the train log as
adapter lines changed per pin move.

**Fixture (one reproducible failing fixture, DESIGN).** `tests/upstream/FD-FSIM-08/`, opened by BD-037 and kept current
by this bead: one test tagged `FD-FSIM-08`, compiled as the `frob-conformance` integration test `upstream_fd_fsim_08` and
marked `#[ignore]` so that the default suite stays green, with `expected.toml` beside it holding the signature
recorded at the pinned commit (`observed_at`, the failing assertion and the observed value). It fails at the pin
and passes once the change is adopted; a fixture that passes at the pin no longer demonstrates the limit and is
itself a failure of the registry check. It reads `COMPONENT_CONTRACT_VERSION` through the airlock (0 at the pin)
and asserts it is at least 1, then runs the sibling's conformance kit on the airlock's two reference components.

**Switch-over (the compatibility train).** When the change is on the sibling's `main`: (1) move the sibling's pin
in `franken-stack.lock` to a clean commit that contains it (a pin only moves forward; a sibling's newer clean head
is never rolled back locally); (2) run `cargo xtask check-stack`, which writes a new resolution receipt
(`frob.stack-receipt/1`) under `receipts/`; (3) re-run that sibling's admission gates, the parity gates and the
gates this item names; (4) relock (`Cargo.lock` regenerated and committed, builds `--locked` on the one toolchain
T) and append the record, with the receipt hash, to `DEPENDENCY_ADMISSION.md`. Only then does the row's `status`
become `adopted` and the interim posture end. No PASS is recorded on a dirty sibling tree, and neither a pin nor
T moves while a ten-fold tournament is running. If a gate named here does not exist yet when the change lands,
the pin still moves with the gates that do exist, the row stays `landed`, and the gate's first run is made
against the new pin.

**What the switch-over re-runs.** The rule is "re-run functor laws". Each of the five design functors commutes
with composition (KA-075, exact, tier G0): for composable designs A and B and functor F,
F(A ∘ B) = F(A) ∘ F(B). The law tests of the specification bead of C-12 are re-run with the dynamic functor
landing on v1 components. With them: INT-FSIM-09 (power across an interconnection sums to zero to 10⁻¹²
relative; every pair parameter returns with its provenance record; a missing material pair is a refusal) and
the port energy-balance check on the referee.

**Fallback if the change cannot land.** The adapter stays on v0 at the pin, and each pin move pays the adapter
cost; if churn in this trait is what pushes upstream breakage above 20% of working days in a four-week window,
KILL-FSIM's worker-process fallback applies, with the worker held at the last pin whose v0 was adapted.

## Acceptance criteria

- [ ] In FrankenSim: `COMPONENT_CONTRACT_VERSION` is 1; the contract document states the five points above; the conformance kit exists and FrankenSim's own components pass it (the rigid-body wrapper of `fs-mbd` included).
- [ ] In FrankenSim: every existing implementation of the trait compiles and passes without behavioural change.
- [ ] Here: `PortSystem` adapts v1 at the new pin, and the adapter refuses at build time a pin whose contract version it does not know (a compile-time assertion on the constant).
- [ ] Here: the functor laws pass for all five functors on the specification's fixtures; INT-FSIM-09 passes at the new pin.
- [ ] Here: adapter lines changed for this pin move are recorded in the train log.
- [ ] The item's row in `registries/upstream.toml` names its fixture, this bead and the mirrored bead in the sibling's tracker; its `status` is `adopted`, or `landed` with the reason the switch-over has not run yet.
- [ ] Every file edited in the sibling was reserved through Agent Mail before the edit and released after landing; the change is on the sibling's `main` and passed that repository's own gates under its `AGENTS.md`.
- [ ] Nothing in this repository patches, forks or vendors the sibling; `cargo xtask check-stack` reports it on-pin and clean under the new receipt, and the record in `DEPENDENCY_ADMISSION.md` cites that receipt.

## Test plan

### Unit tests
In FrankenSim (added):
- `conformance_ideal_transformer`: a gear of ratio n = 5 as a two-port: τ₂ = 5·τ₁ and ω₂ = ω₁/5; expected port powers sum to zero exactly, stored energy 0.
- `conformance_gyrator`: a motor constant k = 0.1 N·m/A: τ = k·I and V = k·ω; at I = 2 A and ω = 30 rad/s the electrical power 6 W equals the mechanical power 6 W.
- `conformance_spring_stores_energy`: stiffness 200 N/m at deflection 0.05 m; expected `stored_energy_j()` = 0.25 J and power balance closed against its rate.
- `ports_is_pure_and_ordered`: two calls return equal lists in equal order.
Here:
- `contract_version_is_known`: the adapter's compile-time assertion; a scratch build against a constant of 2 must fail to compile.
- `dynamic_functor_commutes_with_composition`: (motor ∘ gear) ∘ link against motor ∘ (gear ∘ link): equal port signatures and equal stored energy at 10³ seeded states, to 10⁻¹².
- `algebraic_loop_is_refused`: two effort sources wired port to port; expected the named refusal.

### End-to-end script and logging
`scripts/e2e/upstream_fd_fsim_08.sh --phase pre|post` (plain shell, seed-explicit, idempotent, non-zero on failure).
Phase `pre`, valid at any pin before adoption: `cargo xtask check-stack`; `frob doctor --json` (the registry
section shows this item with its status); `cargo test -p frob-conformance --test upstream_fd_fsim_08 -- --ignored`,
asserting that the fixture fails with the signature in `expected.toml`; then the interim posture's own tests,
asserting that they pass. Phase `post`, after the pin move: the same fixture must pass, the gates named in this
bead are re-run, and the registry row must read `adopted` with a receipt newer than the pin move. Phase `post` also runs `frob design check` and `frob module check` on a template quadruped,
asserting that the port energy balance in the envelope closes and that the contract version it reports is 1.

Logging per STD-E2E: the script sources the shared helper and writes NDJSON to
`target/e2e-logs/upstream_fd_fsim_08/<utc-timestamp>.ndjson` with a human-readable transcript beside it. Every line has `ts`,
`script`, `bead` (`FD-FSIM-08`), `step`, `event` (`start`, `cmd`, `assert`, `metric`, `refusal`, `end`), the exact
command line, exit code, wall time and the envelope's `evidence` block (claim, class, tier, determinism,
identities, seed). It also records the sibling's pinned commit, the stack-receipt hash, the phase, each measured
value with its unit and threshold, digests where state is compared and, on failure, the first differing value
and the artifact paths needed to reproduce. A final `summary` line gives assertions passed and failed and the
gates touched. No private-scene data is logged.

### Known answers and negative controls
- Known answers: KA-075 (each functor commutes with composition); the transformer and gyrator power identities; ½·k·x² = 0.25 J.
- Negative controls: a component whose power sign convention is flipped on one port must fail the conformance kit; a functor implementation that depends on the order of association must fail the composition law; an edit to the frozen trait's method set in a scratch copy of FrankenSim must be caught by its own contract check.
- Determinism lanes: exact where the laws are exact (class D0 for port signatures); 10⁻¹² relative for powers, twice on one machine and in debug and release.

## Considerations

- A port contract must carry time correlation to be sound for budgets (two loads on one bus with identical local summaries can be schedulable or not); v1 freezes the instantaneous laws only, and the budget functor's limits stay this project's concern.
- `fs-phs::interconnect_mapped` exists while constrained Dirac structures are deferred upstream, so closed-loop mechanisms do not rely on FrankenSim's interconnection; freezing `Component` does not change that.
- This item is the cheapest lever on RISK-01: it does not reduce commits to `fs-mbd`, but it removes the one trait every composed model touches from the set of things that move.
- No milestone waits on this bead. Its milestone label says when its switch-over gate first exists, not that the
  milestone's exit depends on it. Until the switch-over has been run, the interim posture above is the design of
  record. M1 depends only on FrankenSim, asupersync and FrankenSQLite, and no upstream work item is on its
  critical path.
- If this change never lands, the program still ships on the interim posture, at the cost stated there. A change
  that does land is adopted only through its switch-over rule: never by a local patch, a fork or a vendored copy,
  and never by rolling a sibling's newer clean head back. Neither a pin nor T moves during the ten-fold
  tournament of M7.
- The priority in the upstream register states the item's value to this project and its place in this program's
  queue. The risks carried at program level are the cost and ordering of upstream work and collisions with other
  agents in the same sibling; the mitigations are file reservation, small commits and the sibling's own gates.

## References

Appendix F.1 (row FD-FSIM-08), F.3; §33.2 (hazard 2), §33.3 (airlock groups), §10 C-12 (foundations; functors), §33.8; INT-FSIM-09; KA-075; RISK-01; BET-04; KILL-FSIM.
