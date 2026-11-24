---
key: "FD-FSIM-22"
title: "upstream frankensim: dynamic rods (`fs-solid` has geometrically exact rods for statics) and a batched cloth"
type: "task"
priority: 4
milestone: "M6"
labels: ["m6", "suite", "habitat", "upstream", "frankensim", "moonshot"]
parent: "WP-73"
deps: ["BD-037", "BD-231", "N19b-20", "N33b-04", "STD-E2E"]
needs: []
related: ["FD-FTX-01", "N14b-05", "N33b-16"]
gates: ["GATE-HAB-09"]
plan_refs: ["Appendix F.1", "Appendix F.3", "§19.10.6", "§19.10.7", "§19.10.5", "§33.9", "§33.15"]
---
## Background

Homes and workplaces contain deformable things a robot trips on: cords, rugs, curtains. FrankenRobotics
represents them in tiers. Solid: rigid proxies (a rug as a free slab, a cord as a fixed ridge or a free rod).
Frontier: articulated proxies (a cord as a chain of links, a rug as hinged plates). Moonshot: **coupled
islands**, a small region of resolved matter beside one world on the CPU referee (an implicit cloth for one rug
or curtain, a Cosserat rod for one cable), used for exactly two purposes: as an oracle against which the
proxies are checked on finalists, and to verify a counterexample that the adversary attributes to living
matter. An island never runs in a training batch, and nothing on the critical path waits for it.

FACT (read 2026-11-21): `fs-solid` has geometrically exact rods **for statics**; dynamic rods and a batched
cloth are not in the siblings. Upstream register priority: **P3**; tier of everything that would consume it:
Moonshot, so this bead's priority is backlog and it is off every critical path.

Rejected: resolved cloth or cable in every training world (a refuted idea: memory and time are out of reach by
orders of magnitude); writing a cloth and rod solver in this repository (it is structural mechanics, which is
FrankenSim's subject).

## Scope

In: in FrankenSim, time-dependent rods on the existing geometrically exact rod, and a cloth model with a
batched evaluation path; here, their use as the "full model" of the proxy-adequacy pilot and as island oracles.
Out: the proxies (BD-231, BD-415); the island contract and the cloth oracle of franken_textiles (BD-416,
FD-FTX-01); fluids (BD-413).

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

**The change (DESIGN; FrankenSim's own structural-mechanics plans govern).**

1. *Dynamic rods.* The geometrically exact (Cosserat) rod of `fs-solid` gains inertia per unit length (ρ·A for
   translation, ρ·I for rotation of the cross-section), a structure-preserving time integrator from FrankenSim's
   own set, contact with rigid primitives through declared instants, and an energy ledger per step.
2. *Cloth.* A thin-shell or equivalent discrete model with stretching, shearing and bending stiffness and area
   density, with an implicit integrator and an evaluation entry that advances many independent cloth instances
   in one call (the batched path).

Both are evaluation-tier owners: CPU, `f64`, with their own evidence. Additive: the static rod is unchanged.

**Interim posture here.** Rigid proxies [S]; chain cords and hinged plates [F]; rods and cloth stay [M] and off
the critical path. Reports say that deformable matter is represented by proxy.

**Fixture (one reproducible failing fixture, DESIGN).** `tests/upstream/FD-FSIM-22/`, opened by BD-037 and kept current
by this bead: one test tagged `FD-FSIM-22`, compiled as the `frob-conformance` integration test `upstream_fd_fsim_22` and
marked `#[ignore]` so that the default suite stays green, with `expected.toml` beside it holding the signature
recorded at the pinned commit (`observed_at`, the failing assertion and the observed value). It fails at the pin
and passes once the change is adopted; a fixture that passes at the pin no longer demonstrates the limit and is
itself a failure of the registry check. It asks the airlock's `structure` group for a dynamic rod and for a cloth
entry (both absent at the pin) and, when present, runs the cantilever-frequency known answer below.

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

**What the switch-over re-runs.** The **proxy-adequacy pilot against the new owner**. As first run (on an
incumbent engine that ships cable and cloth, used as an instrument outside the product): three scenes (a cord
across the path, a rug with a free edge, one square metre of oil), 200 randomized episodes each, one wheeled
and one legged robot; then again with the proxies (a 20-link chain, a hinged thin plate, a friction patch, the
patch with the dwell-time law). The measure is the share of full-model failures that the proxy reproduces in
the same failure class within one second. At switch-over the cord and rug scenes are re-run with FrankenSim's
rod and cloth as the full model, and the two shares are reported side by side (GATE-HAB-09 reports; KILL-LIVING
decides).

**Fallback if the change cannot land, and the kill gate.** KILL-LIVING: trigger: the proxies reproduce 90% or
more of the full-model failures, or an island costs more than 181 rigid worlds at 5% activity with no route to
less. Fallback, named in advance: the Solid tier and the Frontier proxies are the product; coupled islands are
not built beyond the oracle fixtures that already exist in the siblings. KILL-LIVING is decided per island kind
(N19b-20: one `decision` entry per kind, outcomes in N40-03's four names, ADR `docs/adr/ADR-KILL-LIVING-M6.md`).
The rod half of this item needs `not-fired` for `kind:rod`, the cloth half `not-fired` for `kind:cloth`; a half
whose kind is `fired` or `not-reached` is not started, and if both are, this item is closed `superseded` citing
that ADR (the row in `registries/upstream.toml` keeps its number, as every withdrawn upstream item does).

## Acceptance criteria

- [ ] KILL-LIVING has been decided with the outcome `not-fired` for the island kind of each half started (`kind:rod`, `kind:cloth`), and `docs/adr/ADR-KILL-LIVING-M6.md` is cited, before any work starts in FrankenSim.
- [ ] In FrankenSim: the dynamic rod passes the known answers below and closes its energy ledger to its stated residual over 10⁵ steps; the static rod's tests are unchanged.
- [ ] In FrankenSim: the cloth's batched entry gives, for each instance, results bit-identical to the same instance advanced alone.
- [ ] Here: the airlock exposes both as evaluation-tier owners; a result that used them is labelled evaluation tier and is refused in a throughput claim (INV-SUITE-05).
- [ ] Here: the pilot is re-run with the new owner and both shares are recorded for the cord and rug scenes, per robot.
- [ ] The item's row in `registries/upstream.toml` names its fixture, this bead and the mirrored bead in the sibling's tracker; its `status` is `adopted`, or `landed` with the reason the switch-over has not run yet.
- [ ] Every file edited in the sibling was reserved through Agent Mail before the edit and released after landing; the change is on the sibling's `main` and passed that repository's own gates under its `AGENTS.md`.
- [ ] Nothing in this repository patches, forks or vendors the sibling; `cargo xtask check-stack` reports it on-pin and clean under the new receipt, and the record in `DEPENDENCY_ADMISSION.md` cites that receipt.

## Test plan

### Unit tests
In FrankenSim (added):
- `cantilever_first_bending_frequency`: a steel rod of diameter 2 mm and length 0.5 m (E = 200 GPa, ρ = 7,850 kg/m³: E·I = 0.15708 N·m², ρ·A = 0.02466 kg/m); expected f₁ = (1.8751²/(2π))·√(E·I/(ρ·A·L⁴)) = 5.65 Hz within 1% at small amplitude.
- `cantilever_static_tip_deflection`: tip load 0.01 N; expected δ = F·L³/(3·E·I) = 2.65 mm within 0.5% (the static rod's own answer, unchanged by the dynamic one at rest).
- `string_wave_speed`: a rod with negligible bending stiffness under tension 10 N and 0.02466 kg/m; expected wave speed √(T/μ) = 20.14 m/s within 1%.
- `cloth_batch_equals_solo`: 16 instances; bit equality per instance.
- `cloth_drape_on_sphere_is_symmetric`: a square cloth centred on a sphere settles with its four corners at equal height to 10⁻⁶ m.
Here:
- `island_result_is_evaluation_tier`: the label is present, and merging such a result into a throughput-tier claim is refused.

### End-to-end script and logging
`scripts/e2e/upstream_fd_fsim_22.sh --phase pre|post` (plain shell, seed-explicit, idempotent, non-zero on failure).
Phase `pre`, valid at any pin before adoption: `cargo xtask check-stack`; `frob doctor --json` (the registry
section shows this item with its status); `cargo test -p frob-conformance --test upstream_fd_fsim_22 -- --ignored`,
asserting that the fixture fails with the signature in `expected.toml`; then the interim posture's own tests,
asserting that they pass. Phase `post`, after the pin move: the same fixture must pass, the gates named in this
bead are re-run, and the registry row must read `adopted` with a receipt newer than the pin move. Phase `post` also drives the pilot's cord and rug scenes with the island attached
(`frob sim run <robot design> --scene <cord or rug scene> --tier referee --worlds 1 --seconds <episode length>
--seed <s> --key fdfsim22-<scene>-<robot>-<s> --json`: the pilot's wheeled or legged robot as the positional
design, which is the one spelling that admits `--scene`, a registered fixture being `--fixture <name>` with its
own scene; one distinct key per scene, robot and seed) and logs the
reproduced-failure share against 0.90, beside the incumbent-instrument share.

Logging per STD-E2E: the script sources the shared helper and writes NDJSON to
`target/e2e-logs/upstream_fd_fsim_22/<utc-timestamp>.ndjson` with a human-readable transcript beside it. Every line has `ts`,
`script`, `bead` (`FD-FSIM-22`), `step`, `event` (`start`, `cmd`, `assert`, `metric`, `refusal`, `end`), the exact
command line, exit code, wall time and the envelope's `evidence` block (claim, class, tier, determinism,
identities, seed). It also records the sibling's pinned commit, the stack-receipt hash, the phase, each measured
value with its unit and threshold, digests where state is compared and, on failure, the first differing value
and the artifact paths needed to reproduce. A final `summary` line gives assertions passed and failed and the
gates touched. No private-scene data is logged.

### Known answers and negative controls
- Known answers: the cantilever frequency 5.65 Hz and deflection 2.65 mm; the wave speed 20.14 m/s; the cloth oracle of franken_textiles (a square cloth draped over a sphere matches `ftx-drape`'s settled shape to its own stated tolerance, INT-FTX-01).
- Counterexample kept in view (CX-192): living matter does not mean a resolved solver in every training world; an island is one world, offline.
- Negative controls: a rod integrator that gains energy on the undamped cantilever must be caught by its ledger; a batched cloth whose instances share a solver state must fail `cloth_batch_equals_solo`.
- Determinism lanes: twice on one machine; debug and release; same-platform class.

## Considerations

- Estimated cost of the pilot's first run: one engineer-week and under ten GPU-hours (TARGET). The pilot needs no new solver, which is why it comes before this item and can cancel it.
- Results that cross the multiphysics bridge or come from an island are never merged into a throughput-tier or same-stack claim.
- Priority: the register says P3; the consuming tier is Moonshot, and no more than a small number of Moonshot items are active at once by program rule.
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

Appendix F.1 (row FD-FSIM-22), F.3; §19.10.5, §19.10.6 (coupled islands), §19.10.7 (the pilot; KILL-LIVING), §33.9, §33.15; GATE-HAB-09; INV-SUITE-05; CX-192.
