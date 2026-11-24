---
key: "FD-FSIM-20"
title: "upstream frankensim: articulated-dynamics derivatives and hybrid multiple shooting (`fs-orbit`, `fs-mbd`)"
type: "task"
priority: 3
milestone: "M4"
labels: ["m4", "suite", "reflex", "upstream", "frankensim", "frontier", "review-notes-open"]
parent: "WP-64"
deps: ["BD-037", "BD-333", "BD-334", "N22a-33", "N33b-04", "STD-E2E"]
needs: []
related: ["N22a-38", "N22a-39", "N33b-11", "SPEC-C-14"]
gates: ["GATE-GAIT-01", "GATE-GAIT-02"]
plan_refs: ["Appendix F.1", "Appendix F.3", "§22.9", "§31.4", "§33.9", "§33.8"]
---
## Background

`frob-gait` treats a gait as a hybrid periodic orbit: a fixed point of the stride map F, whose stability is
read from the Floquet multipliers of its Jacobian. The Jacobian needs derivatives of the flow through contact
events. On the compliant tiers F is a composition of kernels, and its Jacobian is built column by column from
tangent worlds (central differences in `f64` at two step sizes with a Richardson check, then derivative
programs). On the referee's event-located flow a perturbation is carried through each transverse event by the
saltation matrix

  Ξ = D R + (f⁺ − D R·f⁻)·D g / (D g·f⁻),

for a guard g(x) = 0, a reset R and the vector fields f⁻ and f⁺ before and after the event.

FACT (`frankensim/crates/fs-orbit`, `fs-time/src/hybrid_sensitivity.rs`, `fs-mbd/src`; read 2026-11-21):
FrankenSim has shooting, pseudo-arclength continuation and monodromy, with a shooting module that disclaims
hybrid saltations, and a separate saltation correction. It has **no hybrid multiple-shooting solver on
articulated models and no derivatives of the articulated dynamics with respect to state or design**. The
problem shape of `fs-orbit` is a nonlinear island on a linear port, not an articulated robot in contact, so
today it is an oracle for toy known answers only. Upstream register priority: **P3**. Tier: Frontier.

The compliant-tier orbit route is therefore ours and comes first; the referee route waits on this item.
Rejected: finite-differencing the referee across events as the permanent referee route (each column is a
re-located event sequence; near grazing the difference is meaningless).

## Scope

In: in FrankenSim, (i) derivatives of articulated forward dynamics in state, input and design parameters;
(ii) a hybrid multiple-shooting solver in `fs-orbit` over articulated models with guards and resets, using the
existing saltation correction. Here: the referee orbit route and the re-run of the two gait gates on it. Out:
the compliant-tier route (BD-333, BD-334, BD-335); the referee adapter of `frob-gait`, which answers
`route-not-available: referee` naming this item until the switch-over enables it (N22a-33); the gate records of
GATE-GAIT-01 and GATE-GAIT-02 (N22a-38, N22a-39, their one recorder each; this bead's referee-route re-run is a
check that records nothing); reverse-mode adjoints (FD-FSIM-05). BD-334 (`gait.gradient`, GATE-GAIT-02)
and N22a-33 are dependencies, because the switch-over enables that adapter and re-runs both gates through them.

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

**The change (DESIGN).**

1. *Dynamics derivatives.* For q̈ = FD(q, q̇, τ; θ), entries returning ∂q̈/∂q, ∂q̈/∂q̇, ∂q̈/∂τ = M⁻¹ and ∂q̈/∂θ for
   design parameters θ (masses, centres of mass, inertias, link lengths), by the recursive derivative
   algorithms of the articulated-body method or by forward-mode duals, each checked against central differences.
2. *Hybrid multiple shooting.* The stride is cut at its events into arcs; the unknowns are the arc start
   states and the stride period; the residual stacks arc continuity through the reset maps and the periodicity
   condition modulo the declared symmetry; the Jacobian of an arc is the integrated variational equation
   composed with the saltation matrix at its end. Each event records its guard margin and the denominator
   D g·f⁻; below a declared threshold on |D g·f⁻| the solver refuses (grazing) instead of returning a
   derivative. The monodromy of the converged orbit and its multipliers are returned with their residuals.

Additive: new entries; the existing shooting module and its disclaimer are unchanged.

**Interim posture here.** The compliant-tier orbit route is ours; the referee orbit route is not offered.
`fs-spectral::floquet::analyze_monodromy` analyses a monodromy matrix this project supplies.

**Fixture (one reproducible failing fixture, DESIGN).** `tests/upstream/FD-FSIM-20/`, opened by BD-037 and kept current
by this bead: one test tagged `FD-FSIM-20`, compiled as the `frob-conformance` integration test `upstream_fd_fsim_20` and
marked `#[ignore]` so that the default suite stays green, with `expected.toml` beside it holding the signature
recorded at the pinned commit (`observed_at`, the failing assertion and the observed value). It fails at the pin
and passes once the change is adopted; a fixture that passes at the pin no longer demonstrates the limit and is
itself a failure of the registry check. It requests a hybrid periodic orbit of the rimless wheel on the referee
through the airlock (refused at the pin as a missing capability) and, when available, checks the fixed point
and multiplier below.

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

**What the switch-over re-runs.** GATE-GAIT-01 and GATE-GAIT-02 **on the referee route**. GATE-GAIT-01:
rimless-wheel fixed point and multiplier within 10⁻⁶; compass-gait multipliers within 10⁻⁶ of an independent
high-precision computation; the planted transverse instability is found and the restricted computation is
marked incomplete. GATE-GAIT-02: on the 100-case suite, in at least 72 of 80 regular cases the gradient's
cosine against central differences is at least 0.95; all 20 grazing and fold controls are labelled or refused;
no case is refused for simultaneity alone. Derivatives from the referee route carry the label `saltation`.

**Fallback if the change cannot land.** The compliant-tier route alone; orbits are confirmed on the referee by
rollout (a fixed point is re-simulated and its return error reported), not by a referee Jacobian.

## Acceptance criteria

- [ ] In FrankenSim: the four derivative entries agree with central differences to 10⁻⁶ relative on 10³ states of a two-link arm and of the G1 model; ∂q̈/∂τ equals M⁻¹ to 10⁻¹⁰.
- [ ] In FrankenSim: the multiple-shooting solver converges on the rimless wheel and the compass gait to the values below and refuses the grazing control.
- [ ] Here: the airlock exposes the referee orbit route behind the `spectral` and `referee` groups at the new pin; `gait.orbit --tier referee` returns multipliers with the label `saltation` and the event margins.
- [ ] Here: GATE-GAIT-01 and GATE-GAIT-02 pass on the referee route, with every measured value inside its threshold in the re-run's receipt, whose hash is logged; no `record_gate` call is made (the rows stay those N22a-38 and N22a-39 recorded); compliant-tier and referee multipliers for the same gait are tabulated side by side.
- [ ] The item's row in `registries/upstream.toml` names its fixture, this bead and the mirrored bead in the sibling's tracker; its `status` is `adopted`, or `landed` with the reason the switch-over has not run yet.
- [ ] Every file edited in the sibling was reserved through Agent Mail before the edit and released after landing; the change is on the sibling's `main` and passed that repository's own gates under its `AGENTS.md`.
- [ ] Nothing in this repository patches, forks or vendors the sibling; `cargo xtask check-stack` reports it on-pin and clean under the new receipt, and the record in `DEPENDENCY_ADMISSION.md` cites that receipt.

## Test plan

### Unit tests
In FrankenSim (added):
- `rimless_wheel_return_map`: point-mass rimless wheel with half-angle α = π/8 between spokes (eight spokes), leg length l = 1 m, slope γ = 0.08 rad. The post-impact angular velocity obeys ω²ₙ₊₁ = cos²(2α)·(ω²ₙ + (4·g/l)·sin α·sin γ); the multiplier of the map in ω² is cos²(2α) = 0.5; the fixed point is ω*² = (4·g/l)·sin α·sin γ = 1.2000388 rad²/s² with g = 9.81 m/s² (the factor cos²(2α)/(1 − cos²(2α)) equals 1 here). Expected within 10⁻⁶.
- `saltation_of_a_bouncing_ball`: state (h, v), guard h = 0, reset v → −e·v with e = 0.8, impact velocity v⁻ = −2 m/s, g = 9.81 m/s²; expected the closed form Ξ = [[−e, 0], [−(1 + e)·g/v⁻, −e]] = [[−0.8, 0], [8.829 s⁻¹, −0.8]], to 10⁻¹².
- `dynamics_derivatives_match_central_differences`: as in the acceptance line.
- `grazing_is_refused`: an orbit whose swing foot touches with D g·f⁻ within the threshold of zero; expected the typed refusal with the event identity.
Here:
- `referee_orbit_labels_are_saltation`: every derivative from the referee route carries that label.
- `tiers_agree_on_rimless_wheel`: compliant-tier and referee multipliers within 10⁻³ of each other at stated contact stiffness (a model-form check, recorded, not a gate).

### End-to-end script and logging
`scripts/e2e/upstream_fd_fsim_20.sh --phase pre|post` (plain shell, seed-explicit, idempotent, non-zero on failure).
Phase `pre`, valid at any pin before adoption: `cargo xtask check-stack`; `frob doctor --json` (the registry
section shows this item with its status); `cargo test -p frob-conformance --test upstream_fd_fsim_20 -- --ignored`,
asserting that the fixture fails with the signature in `expected.toml`; then the interim posture's own tests,
asserting that they pass. Phase `post`, after the pin move: the same fixture must pass, the gates named in this
bead are re-run on the referee route, and the registry row must read `adopted` with a receipt newer than the pin
move. The referee calls use the fixture spellings of BD-333 and N22a-38, each with a fresh run-scoped key (two
phases compared, or a re-run after a later pin move, must not replay a stored receipt): `frob gait orbit
fixture:rimless-wheel --set spokes=8,l=1,slope=0.08 --section post-impact --tier referee --key
fdfsim20-rw-<run-id> --json` (the values below), `frob gait orbit fixture:rimless-wheel --set
spokes=8,l=0.5,slope_deg=4 --section post-impact --tier referee --key fdfsim20-rw4-<run-id> --json` (GATE-GAIT-01's
parameter set), `frob gait orbit fixture:compass-gait --section auto --tier referee --key fdfsim20-cg-<run-id>
--json`, `frob gait orbit fixture:transverse-instability --set T=1 --tier referee` with and without
`--spectrum-scope restricted` (keys
`fdfsim20-ti-<run-id>`, `fdfsim20-tir-<run-id>`), and per case of the frozen 100-case suite `frob gait gradient
<case orbit> --params <case parameters> --metrics <case metrics> --tier referee --check fd --key
fdfsim20-gg-<case>-<run-id> --json`. The gate values are recomputed with
`FROB_TEST_SEED=<seed> cargo test -p frob-conformance --test g1_known_answers -- GATE-GAIT-01 GATE-GAIT-02` (the
Gauntlet has no `frob` command), and the script stops at that receipt: the re-run on the referee route is a check
whose receipt hash the script logs, and it is not entered with `record_gate`. Each of the two gates has one
recording bead, N22a-38 (GATE-GAIT-01) and N22a-39 (GATE-GAIT-02), as N22c-49's register declares, and N38b-19
admits a second recording only as the lift of a row recorded against a fallback named in advance, which these
rows are not (their rows record the tier that adjudicated each clause and say the clauses are re-run on the
referee route when it exists); their rows are unchanged by this bead.

Logging per STD-E2E: the script sources the shared helper and writes NDJSON to
`target/e2e-logs/upstream_fd_fsim_20/<utc-timestamp>.ndjson` with a human-readable transcript beside it. Every line has `ts`,
`script`, `bead` (`FD-FSIM-20`), `step`, `event` (`start`, `cmd`, `assert`, `metric`, `refusal`, `end`), the exact
command line, exit code, wall time and the envelope's `evidence` block (claim, class, tier, determinism,
identities, seed). It also records the sibling's pinned commit, the stack-receipt hash, the phase, each measured
value with its unit and threshold, digests where state is compared and, on failure, the first differing value
and the artifact paths needed to reproduce. A final `summary` line gives assertions passed and failed and the
gates touched. No private-scene data is logged.

### Known answers and negative controls
- Known answers: the rimless wheel's multiplier cos²(2α) = 0.5 and fixed point ω*² = 1.2000388 rad²/s² (4·9.81·sin(π/8)·sin(0.08) = 4·9.81·0.3826834·0.0799147); the monodromy of ẍ + x = 0 over one period has both multipliers equal to 1 to 10⁻¹², and a monodromy with a multiplier of modulus 1.01 is reported unstable (INT-FSIM-13, KA-768).
- Negative controls: a Jacobian assembled without the saltation term must give a wrong multiplier on the bouncing ball and be caught; a solver restricted to the sagittal plane must miss the planted transverse instability and be marked incomplete.
- Determinism lanes: twice on one machine; debug and release; same-platform class for the referee.

## Considerations

- PO-GAIT-01 and the label set of derivatives belong to the gait chapter; this item only makes the referee able to supply the `saltation` case.
- A referee orbit is limited by the referee's contact limits (fixed witnesses, 21 contacts): a rolling foot contact inside a stride needs FD-FSIM-12 or the migration above the airlock.
- The reduced state has dimension about 40 to 130 for the robots in scope (DESIGN estimate), so the referee Jacobian costs that many variational solves per stride.
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

Appendix F.1 (row FD-FSIM-20), F.3; §22.9 (gaits as orbits; derivatives by tier), §31.4 (`fs-orbit`), §33.9 (INT-FSIM-13), §33.8; GATE-GAIT-01, GATE-GAIT-02; KA-768.
