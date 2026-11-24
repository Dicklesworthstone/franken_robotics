---
key: "FD-FSIM-03"
title: "upstream frankensim: capsule and hull support maps, field pair gaps, contact generation over an interval"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "suite", "traction", "upstream", "frankensim", "frob-contact", "solid"]
parent: "WP-10"
deps: ["BD-037", "BD-079", "BD-080", "N24b-03", "N33b-04", "STD-E2E"]
needs: []
related: ["FD-FSIM-12", "N16a-05", "N16b-21", "N33a-16", "N33b-10", "NF-10"]
gates: ["GATE-TRAC-02"]
plan_refs: ["Appendix F.1", "Appendix F.3", "§16.4", "§18.3", "§33.8", "§33.10"]
---
## Background

In FrankenRobotics everything that moves is a union of convex primitives (spheres, capsules, boxes, cylinders,
rounded boxes; convex hulls as a Frontier addition), and everything static is a distance field, optionally a
heightfield. Contact needs, for each candidate pair, a gap, a normal and witness points.

FACT (FrankenSim at `483d09c682`, read 2026-11-21): FrankenSim's narrow phase is convex-against-convex for
primitives through a certified Frank–Wolfe separation in `fs-query`; signed-distance pairs, heightfields and
mesh manifolds are refused as `MissingCapability`. Its contact step takes the contact set from the caller and
states that "independent endpoint gap checks are not continuous collision detection". So contact generation and
tunnelling detection are this project's: `frob-contact` generates the contact sets, and
`fs-contact::certified_ccd` audits the path. Upstream register priority: **P1**.

Why the upstream item exists. The referee's contact sets come from code that FrankenSim never sees, so parity
between this project's distances and FrankenSim's can only be checked on the pair kinds FrankenSim has. Every
pair kind added upstream widens the set on which the referee's geometry has an independent check.

Rejected: trusting `frob-contact` unchecked on the pair kinds FrankenSim lacks (no second opinion on the
geometry the referee stands on); a local fork of `fs-query`.

## Scope

In: in FrankenSim, support maps for capsules and convex hulls; pair-gap queries between a convex primitive and
a heightfield and between a convex primitive and a sampled distance field, each with an error bound; a contact
generation entry that works over an interval and not only at endpoints. Here: wider parity. Out: `frob-contact`
itself (BD-079), the batched narrow phase (BD-158, BD-166), mesh paths (BD-232).

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

**The change (DESIGN).** Three additive pieces.

1. *Support maps* in `fs-query`. The support function of a convex set C in direction d is h_C(d) = max over
   x ∈ C of x·d. For a capsule with segment endpoints a and b and radius r: h(d) = max(a·d, b·d) + r·‖d‖. For
   the convex hull of vertices v₁ … vₙ: h(d) = max over i of vᵢ·d. With a support map, the existing certified
   separation applies to the new shapes unchanged.
2. *Field pair gaps.* A query between a convex primitive and (i) a heightfield z = H(x, y) on a regular grid,
   (ii) a sampled signed-distance field with a stated interpolation bound. Each returns gap, normal, witness
   points and an error bound ε_g, so that a caller can keep the bound with the gap. A pair the query cannot
   bound is refused, as today, and never answered approximately without saying so.
3. *Contact generation over an interval.* An entry that, given two poses per body bounding an interval, returns
   the contact set with pairs that come into range inside the interval, built on the spacetime broad phase and
   certified continuous collision detection FrankenSim already has, in place of independent endpoint gap checks.

**Interim posture here.** `frob-contact` generates contact sets from `fs-query` convex separation and
signed-distance queries; `fs-contact::certified_ccd` audits the path on sampled windows; parity with `fs-query`
is claimed only where both apply. Every gap carries its error bound.

**Fixture (one reproducible failing fixture, DESIGN).** `tests/upstream/FD-FSIM-03/`, opened by BD-037 and kept current
by this bead: one test tagged `FD-FSIM-03`, compiled as the `frob-conformance` integration test `upstream_fd_fsim_03` and
marked `#[ignore]` so that the default suite stays green, with `expected.toml` beside it holding the signature
recorded at the pinned commit (`observed_at`, the failing assertion and the observed value). It fails at the pin
and passes once the change is adopted; a fixture that passes at the pin no longer demonstrates the limit and is
itself a failure of the registry check. It requests, through the airlock trait `GeometryKernel`, a capsule–capsule
gap, a hull–box gap, a sphere–heightfield gap and a sphere–distance-field gap, and records which are answered
and which return the upstream `MissingCapability` refusal at the pin.

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

**What the switch-over re-runs.** GATE-TRAC-02 (parity, M1): primitive pair distances and normals agree with
`fs-query` within 10⁻⁹ m in `f64` on 10⁶ random pairs. After adoption the 10⁶ pairs are drawn over the enlarged
set of pair kinds, with the count per kind recorded; field pairs are compared within the sum of the two stated
error bounds. The tunnelling audit keeps `certified_ccd` as its detector.

**Fallback if the change cannot land.** `frob-contact` stays the only generator for those pair kinds; their
correctness rests on its own known answers and on the certified audit, and the gate's record lists the pair
kinds with no sibling check.

## Acceptance criteria

- [ ] In FrankenSim: the two support maps exist and the certified separation passes its own tests on capsule and hull pairs; existing primitive pairs are unchanged.
- [ ] In FrankenSim: the two field queries return gap, normal, witnesses and an error bound, and refuse what they cannot bound.
- [ ] In FrankenSim: the interval contact generator finds the thin-step case (a body whose endpoint poses are both clear of a thin obstacle it passes through) on 1,000 of 1,000 seeded sweeps.
- [ ] Here: the airlock answers the four fixture queries at the new pin.
- [ ] Here: GATE-TRAC-02 passes on 10⁶ pairs over the enlarged set, with measured maxima recorded against 10⁻⁹ m (and against the summed bounds for field pairs).
- [ ] Here: no pair kind is reported as checked that was not in the sample; the per-kind counts are in the record.
- [ ] The item's row in `registries/upstream.toml` names its fixture, this bead and the mirrored bead in the sibling's tracker; its `status` is `adopted`, or `landed` with the reason the switch-over has not run yet.
- [ ] Every file edited in the sibling was reserved through Agent Mail before the edit and released after landing; the change is on the sibling's `main` and passed that repository's own gates under its `AGENTS.md`.
- [ ] Nothing in this repository patches, forks or vendors the sibling; `cargo xtask check-stack` reports it on-pin and clean under the new receipt, and the record in `DEPENDENCY_ADMISSION.md` cites that receipt.

## Test plan

### Unit tests
In FrankenSim (added):
- `capsule_support_closed_form`: a = (0, 0, 0), b = (1, 0, 0), r = 0.1; d = (1, 0, 0) gives 1.1; d = (0, 1, 0) gives 0.1; d = (−1, 0, 0) gives 0.1.
- `parallel_capsules_gap`: axes parallel at distance 0.5, radii 0.1 and 0.15; expected gap 0.25 m and a normal perpendicular to both axes, to 10⁻¹².
- `crossed_capsules_gap`: perpendicular axes whose common perpendicular has length 0.4, radii 0.1 each; expected 0.2 m.
- `hull_support_is_vertex_max`: the unit cube's hull in direction (1, 1, 1)/√3; expected √3/2 from the centre.
- `sphere_on_flat_heightfield`: sphere of radius 0.05 with centre at height 0.20 over H ≡ 0; expected gap 0.15 m, normal (0, 0, 1), error bound reported as 0.
- `sphere_against_sdf_of_plane`: the same scene through a sampled distance field with cell 2 cm; expected gap within the stated ε_g of 0.15 m.
Here:
- `airlock_field_pairs_answer_or_refuse_by_name`: the four queries; expected four answers after adoption and the typed referee refusal carrying `MissingCapability` at the pin.
- `parity_sample_reports_per_kind_counts`: the sample of GATE-TRAC-02; expected every pair kind present with a non-zero count.

### End-to-end script and logging
`scripts/e2e/upstream_fd_fsim_03.sh --phase pre|post` (plain shell, seed-explicit, idempotent, non-zero on failure).
Phase `pre`, valid at any pin before adoption: `cargo xtask check-stack`; `frob doctor --json` (the registry
section shows this item with its status); `cargo test -p frob-conformance --test upstream_fd_fsim_03 -- --ignored`,
asserting that the fixture fails with the signature in `expected.toml`; then the interim posture's own tests,
asserting that they pass. Phase `post`, after the pin move: the same fixture must pass, the gates named in this
bead are re-run, and the registry row must read `adopted` with a receipt newer than the pin move. Phase `post` also re-runs the parity gate in the Gauntlet's cargo form (ADR-GAUNT-01, BD-025; no `frob
conformance` command exists) `FROB_TEST_SEED=<seed> cargo test -p frob-conformance --test g2_parity --
GATE-TRAC-02` over the enlarged pair set, asserting from its receipt the maximum distance and normal
disagreements against their thresholds and a non-zero count for every pair kind; and, from the user's side, runs
`frob sim run --tier referee --fixture traction/sphere_on_heightfield --seed <s> --key fsim03-hf-<s> --json` (a sphere dropped on a
flat heightfield, a field pair the referee could not check at the pin; the fixture is registered by this bead as
`assets/fixtures/traction/sphere_on_heightfield/fixture.toml` in the fixture registry of N24b-03, and the run is
the referee step of BD-080, both dependencies), asserting a committed run whose contacts
carry their ε_g. (No verb takes a raw primitive pair: `contact.probe`, N18b-29, evaluates laws at a contact state,
and the four fixture queries are asserted by the `upstream_fd_fsim_03` test, not through `frob`.)

Logging per STD-E2E: the script sources the shared helper and writes NDJSON to
`target/e2e-logs/upstream_fd_fsim_03/<utc-timestamp>.ndjson` with a human-readable transcript beside it. Every line has `ts`,
`script`, `bead` (`FD-FSIM-03`), `step`, `event` (`start`, `cmd`, `assert`, `metric`, `refusal`, `end`), the exact
command line, exit code, wall time and the envelope's `evidence` block (claim, class, tier, determinism,
identities, seed). It also records the sibling's pinned commit, the stack-receipt hash, the phase, each measured
value with its unit and threshold, digests where state is compared and, on failure, the first differing value
and the artifact paths needed to reproduce. A final `summary` line gives assertions passed and failed and the
gates touched. No private-scene data is logged.

### Known answers and negative controls
- Known answers: the support values and gaps above; a box on a plane has a support query equal to its closed form (the admission test INT-FSIM-08).
- Negative controls: a capsule support that omits the r·‖d‖ term must fail `parallel_capsules_gap` by 0.25 m; an endpoint-only contact generator must miss the thin-step case; a field query that returns a gap without an error bound must be rejected by the airlock.
- Metamorphic relations: gaps are invariant under a common rigid motion of both shapes; swapping the two shapes negates the normal.
- Determinism lanes: twice on one machine; debug and release; same-platform class.

## Considerations

- A mesh-backed interior is not admitted by any of this: it needs scene cooking and swept cases of its own.
- Parity "where both apply" must not turn into a silent shrink of the sample; the per-kind counts in the gate's record are the guard.
- FrankenSim's certified separation gives an enclosure, not a point value; comparisons use its stated width.
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

Appendix F.1 (row FD-FSIM-03), F.3; §16.4 (limits table), §18.3, §33.8, §33.10 (INT-FSIM-04, INT-FSIM-08); GATE-TRAC-02, GATE-TRAC-03.
