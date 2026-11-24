---
key: "FD-FSIM-28"
title: "upstream frankensim: a stable, ungated surface for `fs-topopt::sdf3`"
type: "task"
priority: 3
milestone: "M5"
labels: ["m5", "suite", "foundry", "upstream", "frankensim", "frontier"]
parent: "WP-70"
deps: ["BD-037", "BD-401", "BD-403", "N31b-13", "N33b-04", "STD-E2E"]
needs: []
related: ["FD-FSIM-30", "N31a-04", "N31a-15", "N31a-29", "N33b-14", "NFx-02"]
gates: ["INT-FSIM-15", "GATE-FOUNDRY-05"]
plan_refs: ["Appendix F.1", "Appendix F.3", "§31.4", "§31.7", "§33.9", "§33.3", "§33.16"]
---
## Background

For finalists, Foundry optimizes the topology of links in three dimensions: minimize projected volume subject
to a stress cap S_y/SF on a relaxed measure, a stiffness cap per envelope case from the control budget, a
first-mode floor above actuator bandwidth, a minimum length scale and process constraints. The solver is
FrankenSim's `fs-topopt` (about 40 thousand lines, layer L4; depends on `fs-solid`, `fs-material`,
`fs-spectral`). Its two-dimensional and density-based drivers are ordinary crate surface. FACT (read
2026-11-21): the three-dimensional lane is **behind the feature `cutfem-marquee`, tagged Frontier upstream**:
optimization on cut cells over Cartesian and octree backgrounds (`sdf3`), material body loads (`sdf3::body`), a
relaxed stress measure and a sampled peak bound (`sdf3::stress`), minimum volume under a stress cap
(`sdf3::design`), modal design (`sdf3::modal`) and goal-driven refinement (`sdf3_goal`). Its documentation says
that cut volumes, discrete sensitivities and the stress aggregate are not continuum certificates, and that the
aggregate is not a bound on the maximum. Upstream register priority: **P3**.

A feature-gated lane may change without notice, so this project reaches it behind the same feature, through
the airlock, pinned by commit: every pin move re-admits it. The item asks for the usual next step: a surface
that FrankenSim states is stable. Rejected: copying the `sdf3` modules here (a lift of the least settled part
of a 40-thousand-line crate); using the density driver in three dimensions without cut cells (a different
method with different artefacts, kept as a fallback, not a substitute).

## Scope

In: in `fs-topopt`, the `sdf3` entry points that Foundry uses made available without the Frontier feature,
with a contract; here, the feature gate removed from the airlock entry. Out: the topology rung itself and the
inertia feedback loop (BD-403, BD-404). BD-403, which owns `foundry.optimize`, and N31b-13, which measures
GATE-FOUNDRY-05 and writes its row, are dependencies, because the switch-over compares that row's saving and
iteration count across the pin move; the INT-FSIM-15 test it extends is N33b-14's, reached through BD-403.
Also out: plane stress in two dimensions (FD-FSIM-27); surface extraction for
export, which FrankenSim does not provide and Foundry owns.

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

**The change (DESIGN).** A stable subset of `sdf3`, chosen by FrankenSim, compiled without `cutfem-marquee`:
at least `sdf3::design` (minimum volume under a stress cap), `sdf3::modal` (the first-mode constraint) and the
types they take and return. With it: a contract section that states the method's limits in the words its
documentation already uses (cut volumes, discrete sensitivities and the stress aggregate are not continuum
certificates; the aggregate is not a bound on the maximum), the inputs it admits, its determinism class, and a
stability statement (signatures in the stable subset change only with a version of the contract). The rest of
the lane (`sdf3_goal`, experimental backgrounds) may stay behind the feature. Additive: with the feature on,
nothing changes.

**Interim posture here.** `fs-topopt::sdf3` is reached behind its feature through the airlock group
`structure`, pinned by commit; a binary built without that group refuses the verb with `FROB-REF-790`
(`foundation-group-absent`). Its results carry estimate authority and are checked by the independent solve.

**Fixture (one reproducible failing fixture, DESIGN).** `tests/upstream/FD-FSIM-28/`, opened by BD-037 and kept current
by this bead: one test tagged `FD-FSIM-28`, compiled as the `frob-conformance` integration test `upstream_fd_fsim_28` and
marked `#[ignore]` so that the default suite stays green, with `expected.toml` beside it holding the signature
recorded at the pinned commit (`observed_at`, the failing assertion and the observed value). It fails at the pin
and passes once the change is adopted; a fixture that passes at the pin no longer demonstrates the limit and is
itself a failure of the registry check. It builds a probe that calls `sdf3::design` with `cutfem-marquee` off; at
the pin the symbol does not exist without the feature and the build fails.

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

**What the switch-over re-runs.** The rule is "remove the feature gate from the airlock entry": `frob-fsim`'s
`structure` group no longer enables `cutfem-marquee`, and the group gate INT-FSIM-15 is re-run (a tolerance
allocation on a two-term stack reproduces the hand-computed variance split; a mesh written through `fs-io`
reads back with the same hash; a STEP file above the stated bound is quarantined with a receipt, not parsed),
with one known answer through `sdf3::design` added to it (DESIGN): the two-bar slab of KA-739 (N31a-18's
fixture `twobar`: load P at distance L from a wall; minimum volume under a stress cap is 2·P·L/σ_allow, reached
by two bars at ±45°) run through the airlock with `cutfem-marquee` off, asserting two bars at 45° ± 5° and
compliance within 10% of 4·P²·L²/(E·V) at the achieved volume V. Then the topology rung's gate,
GATE-FOUNDRY-05 (the stage-2 pilot: saving at least 7%, the kill threshold, with 15% the target; the
convergence test passes within five iterations; coarse solve at most 120 s per link class), is re-measured at
the new pin by its own script (N31b-13) to show that results did not move with the gate's removal.

**Fallback if the change cannot land.** The feature-gated route, re-admitted at every pin move. If the lane
breaks at a pin, KILL-TOPOPT's fallback applies: the sizing rung (beams and the engineered baseline) is the
product for that pin.

## Acceptance criteria

- [ ] In FrankenSim: the stable subset builds and is tested with `cutfem-marquee` off; the contract states the limits and the stability rule; with the feature on, existing tests are unchanged.
- [ ] Here: `frob-fsim` does not enable `cutfem-marquee` at the new pin (`cargo tree -e features` on the `structure` group shows it absent), and `foundry optimize` still runs.
- [ ] Here: INT-FSIM-15 passes at the new pin, as N33b-14 states it (the two-term stack, the mesh round trip, the quarantined STEP file) and with the added two-bar `sdf3::design` known answer; GATE-FOUNDRY-05's saving and iteration count at the new pin equal those in its row recorded at the old pin to the solver's tolerance.
- [ ] Here: results still carry the sibling's own disclaimer (not a continuum certificate) in their evidence.
- [ ] The item's row in `registries/upstream.toml` names its fixture, this bead and the mirrored bead in the sibling's tracker; its `status` is `adopted`, or `landed` with the reason the switch-over has not run yet.
- [ ] Every file edited in the sibling was reserved through Agent Mail before the edit and released after landing; the change is on the sibling's `main` and passed that repository's own gates under its `AGENTS.md`.
- [ ] Nothing in this repository patches, forks or vendors the sibling; `cargo xtask check-stack` reports it on-pin and clean under the new receipt, and the record in `DEPENDENCY_ADMISSION.md` cites that receipt.

## Test plan

### Unit tests
In FrankenSim (added, default features):
- `sdf3_design_is_reachable_without_frontier_feature`: a compile test.
- `sdf3_cantilever_block_volume_decreases`: a clamped block under a tip load; the optimized volume is below the initial volume, the relaxed stress is at or under the cap, and the result is identical to the same run with the feature on.
- `sdf3_reports_limits_in_result`: the returned evidence names the aggregate as not a bound.
Here:
- `structure_group_has_no_frontier_feature`: parses the resolved feature set.
- `verb_refuses_without_group`: a binary built without `structure`; expected `FROB-REF-790`.
- INT-FSIM-15 re-run at the new pin (N33b-14's `crates/frob-fsim/tests/admission/int_fsim_15.rs`, lane `--features structure`), unchanged: KA-770's two-term stack with sensitivities (1, 4), equal costs under the 1/t model, k = 3 and √B = 10 µm returns t = (15.99, 6.35) µm to 10⁻⁹ relative and the variance split 28.41 and 71.59 µm² summing to 100 µm² to 10⁻¹² (N33b-14 sets these values and tolerances; this bead does not restate KA-770 with other numbers); the mesh hash round trip; the over-bound STEP file quarantined.
- `sdf3_twobar_known_answer` (added to that file at the switch-over): the two-bar slab through `sdf3::design` with the feature off: two bars at 45° ± 5°; compliance within 10% of 4·P²·L²/(E·V) at the achieved volume; hand values from N31a-18 (L = 0.10 m, P = 1,000 N; least volume 1.449 × 10⁻⁶ m³ at 138 MPa).

### End-to-end script and logging
`scripts/e2e/upstream_fd_fsim_28.sh --phase pre|post` (plain shell, seed-explicit, idempotent, non-zero on failure).
Phase `pre`, valid at any pin before adoption: `cargo xtask check-stack`; `frob doctor --json` (the registry
section shows this item with its status); `cargo test -p frob-conformance --test upstream_fd_fsim_28 -- --ignored`,
asserting that the fixture fails with the signature in `expected.toml`; then the interim posture's own tests,
asserting that they pass. Phase `post`, after the pin move: the same fixture must pass, the gates named in this
bead are re-run, and the registry row must read `adopted` with a receipt newer than the pin move. Phase `post`
also re-runs pair 1 of the GATE-FOUNDRY-05 measurement at the new pin in N31b-13's command forms, under keys of
its own (the old pin's keys would return the stored envelopes): `frob foundry loads quad-ref --campaign
<set-1> --key fd28-post-loads --json`, `frob foundry optimize quad-ref --links lower-leg --mode coarse --loop
--key fd28-post-opt --json` and `frob foundry check quad-ref --link lower-leg --key fd28-post-check --json`,
and compares saving, iteration count and wall time with pair 1 of the row recorded at the old pin.

Logging per STD-E2E: the script sources the shared helper and writes NDJSON to
`target/e2e-logs/upstream_fd_fsim_28/<utc-timestamp>.ndjson` with a human-readable transcript beside it. Every line has `ts`,
`script`, `bead` (`FD-FSIM-28`), `step`, `event` (`start`, `cmd`, `assert`, `metric`, `refusal`, `end`), the exact
command line, exit code, wall time and the envelope's `evidence` block (claim, class, tier, determinism,
identities, seed). It also records the sibling's pinned commit, the stack-receipt hash, the phase, each measured
value with its unit and threshold, digests where state is compared and, on failure, the first differing value
and the artifact paths needed to reproduce. A final `summary` line gives assertions passed and failed and the
gates touched. No private-scene data is logged.

### Known answers and negative controls
- Known answers: KA-770 (the two-term variance split; the mesh hash round trip, as N33b-14 states them); KA-739 (the two-bar optimum through `sdf3::design`); the cantilever block's monotone volume decrease under an active stress cap.
- Negative controls: a probe that calls an entry still behind the feature must fail to build (so that "ungated" is tested, not assumed); a STEP file above the stated bound must be quarantined, not parsed.
- Determinism lanes: the optimization twice on one machine gives identical iterates; debug and release; same-platform class.

## Considerations

- BET-29 and its kill threshold (a saving below 7% in the stage-2 pilot) decide whether topology optimization is worth running at all; an ungated surface does not change that calculation.
- Foundry is an output-quality subsystem and not a speed lever; nothing in the ten-fold thesis counts it.
- The `structure` group is compiled only by the M5 lanes; removing one feature from it shrinks nothing measurable in build time.
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

Appendix F.1 (row FD-FSIM-28), F.3; §31.4 (`fs-topopt`), §31.7 (the topology rung), §33.9 (INT-FSIM-15), §33.3 (airlock groups), §33.16 (FROB-REF-790); GATE-FOUNDRY-05 (N31b-13); KILL-TOPOPT; KA-770 (N33b-14); KA-739 (N31a-18).
