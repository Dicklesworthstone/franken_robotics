---
key: "FD-FSIM-29"
title: "upstream frankensim: geometric evaluators for overhang and tool access in `fs-fab`"
type: "task"
priority: 3
milestone: "M5"
labels: ["m5", "suite", "foundry", "upstream", "frankensim", "frontier"]
parent: "WP-71"
deps: ["BD-037", "BD-405", "N33b-04", "STD-E2E"]
needs: []
related: ["N31a-04", "N31a-26", "N31a-28", "N31a-29", "N31a-30", "N31b-17", "N33b-14", "NFx-02"]
gates: ["GATE-FOUNDRY-07"]
plan_refs: ["Appendix F.1", "Appendix F.3", "§31.4", "§31.10", "§33.9", "§33.8"]
---
## Background

Before a part goes into a build packet, Foundry checks that it can be made: walls thick enough, internal
corners no sharper than the tool, every machined surface reachable, printed overhangs within the process
limit, no undercut where the process cannot produce one. FrankenSim's `fs-fab` holds the vocabulary of those
constraints (FACT, read 2026-11-21: 355 lines, layer L4): scalar limit constraints by process (overhang angle,
minimum feature size, tool radius, draft angle, and three structural-code and transport limits), each declaring
its differentiability and certificate availability; `check_all` with a repair target per violation; parametric
cost and embodied-carbon estimates. **It compares a supplied number with a limit. It does not measure
geometry.** `fs-geocon` measures minimum thickness, draft and envelopes, and has no overhang, tool-access or
corner-radius measure. Upstream register priority: **P3**.

So the measurements that feed `fs-fab` are written here, in `frob-foundry`, and reported as
`review: no-claim`: a screening aid for a human reviewer, not a statement that the part is manufacturable.
The upstream item would give the suite one owner for these measurements, shared with FrankenSim's own
fabrication work. Rejected: promoting the local evaluators to a claim (no process-capability data, no machine
models); leaving the limits unchecked.

## Scope

In: in `fs-fab` (or `fs-geocon`, as FrankenSim's layering prefers), evaluators that measure overhang and tool
access on a triangle surface and feed the existing limit checks; here, a differential test and deletion of the
local evaluators. Out: wall thickness and draft (already in `fs-geocon`); process capability, machine models
and prices; the manufacturability report and its repairs as typed edits (BD-405).

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

1. *Overhang.* Given a build direction b (unit vector) and a closed oriented triangle surface, a facet with
   outward normal n faces downward when n·b < 0, and its inclination from the build plane is β with
   cos β = −n·b. The facet violates a minimum self-supporting inclination β_min (a process limit from the
   existing vocabulary) when it faces downward and β < β_min, unless it lies on the build plate. The evaluator
   returns the violating area, its connected regions with centroids, and the worst inclination, and feeds the
   worst value to the existing overhang-angle limit.
   *Two angle conventions meet here.* The local evaluator of N31a-28 (and BD-405) reports the overhang angle
   from the vertical, ψ = asin(−n·b) (0° for a vertical wall, 90° for a ceiling), against a maximum
   `overhang_max_deg`; the inclination above is measured from the build plane, so ψ = 90° − β and a card's
   ψ_max corresponds to β_min = 90° − ψ_max (45° on both sides for the usual limit; a 30° chamfer in β is a
   60° overhang in ψ). Which of the two the existing `fs-fab::overhang_angle` limit reads is a FACT to re-read
   at the pin (the sibling's source is what counts); the adapter converts once, the differential test of the
   switch-over compares converted values, and a unit test pins the conversion on the 30° and 60° ramps.
2. *Tool access.* Given a tool of radius r and a set of approach directions, a surface point is reachable if a
   cylinder of radius r along some approach direction touches it without cutting the part. The evaluator
   returns the unreachable area with its regions, and the smallest internal corner radius found, which it feeds
   to the existing tool-radius limit. A concave edge of radius below r is reported with the repair target
   "fillet radius at least r".
3. *Authority.* Each measurement declares its resolution (it is exact for the triangle surface it was given and
   says nothing beyond it) and whether it is differentiable; neither claims a certificate.

Additive: `check_all` and the scalar limits are unchanged; they gain callers.

**Interim posture here.** `frob-foundry` owns the evaluators and reports `review: no-claim` on every
manufacturability result.

**Fixture (one reproducible failing fixture, DESIGN).** `tests/upstream/FD-FSIM-29/`, opened by BD-037 and kept current
by this bead: one test tagged `FD-FSIM-29`, compiled as the `frob-conformance` integration test `upstream_fd_fsim_29` and
marked `#[ignore]` so that the default suite stays green, with `expected.toml` beside it holding the signature
recorded at the pinned commit (`observed_at`, the failing assertion and the observed value). It fails at the pin
and passes once the change is adopted; a fixture that passes at the pin no longer demonstrates the limit and is
itself a failure of the registry check. It asks the airlock trait `FabricationRules` to measure overhang on a
triangle surface (absent at the pin: only a supplied number can be checked).

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

**What the switch-over re-runs.** The rule is "differential test, then delete the local evaluators": on the
twenty parts of GATE-FOUNDRY-07 (planted violations of wall, corner radius, tool access, overhang and undercut,
each detected with location and repair; clean parts pass; 20 of 20) the sibling's and the local evaluators must
agree on every verdict, on violating area within 1% and on locations within one facet. Only then are the local
evaluators deleted and the gate re-run on the sibling's alone. The label `review: no-claim` stays: the sibling
evaluators declare estimate authority too.

**Fallback if the change cannot land.** The local evaluators stay, with their label.

## Acceptance criteria

- [ ] In FrankenSim: the two evaluators exist, state their resolution and authority, and pass the known answers below; `check_all` and existing limits are unchanged.
- [ ] Here: the differential test passes on all twenty gate parts (verdicts equal; areas within 1%; locations within one facet), and its table is recorded.
- [ ] Here: after deletion, `frob-foundry` contains no overhang or tool-access measurement of its own, and GATE-FOUNDRY-07 passes 20 of 20 at the new pin.
- [ ] Here: every manufacturability result still carries `review: no-claim`.
- [ ] The item's row in `registries/upstream.toml` names its fixture, this bead and the mirrored bead in the sibling's tracker; its `status` is `adopted`, or `landed` with the reason the switch-over has not run yet.
- [ ] Every file edited in the sibling was reserved through Agent Mail before the edit and released after landing; the change is on the sibling's `main` and passed that repository's own gates under its `AGENTS.md`.
- [ ] Nothing in this repository patches, forks or vendors the sibling; `cargo xtask check-stack` reports it on-pin and clean under the new receipt, and the record in `DEPENDENCY_ADMISSION.md` cites that receipt.

## Test plan

### Unit tests
In FrankenSim (added):
- `horizontal_ceiling_is_an_overhang`: a box with a rectangular pocket open downward, build direction +z; the pocket's ceiling has β = 0 and is reported in full (its area exactly), with β_min = 45°.
- `chamfer_angles`: a downward chamfer at 60° from the build plane passes at β_min = 45°; one at 30° is reported, with worst inclination 30°.
- `face_on_build_plate_is_exempt`: the part's bottom face is not reported.
- `slot_width_against_tool`: a slot 6 mm wide through a 10 mm plate, with semicircular ends of radius 3 mm, approach direction along the slot's depth; expected every slot wall reachable by a tool of radius 2.5 mm (diameter 5 mm), and unreachable by one of radius 4 mm (diameter 8 mm), with the unreachable area equal to the slot's wall area (a through slot has no floor).
- `sharp_internal_corner`: an internal corner of radius 1 mm with a tool of radius 2.5 mm is reported with repair target "fillet radius at least 2.5 mm".
- `undercut_needs_another_direction`: a T-slot is unreachable from +z alone and reachable when the side direction is added.
Here:
- `overhang_convention_bridge`: the 30° and 60° ramps of BD-405 read ψ = 30° and 60° locally and β = 60° and 30° from the sibling; the adapter's converted values agree, and a 45° card limit gives the same verdicts on both sides.
- `five_axis_cone_through_sibling`: N31a-28's `five_axis_cone_reaches_tilted_face` fixture (a 30° cone reaching a face tilted 20° past vertical) passes through the sibling's evaluator fed the 32 cone directions.
- `differential_on_gate_parts`: as in the acceptance line.
- `no_local_evaluator_remains`: a source scan after deletion.

### End-to-end script and logging
`scripts/e2e/upstream_fd_fsim_29.sh --phase pre|post` (plain shell, seed-explicit, idempotent, non-zero on failure).
Phase `pre`, valid at any pin before adoption: `cargo xtask check-stack`; `frob doctor --json` (the registry
section shows this item with its status); `cargo test -p frob-conformance --test upstream_fd_fsim_29 -- --ignored`,
asserting that the fixture fails with the signature in `expected.toml`; then the interim posture's own tests,
asserting that they pass. Phase `post`, after the pin move: the same fixture must pass, the gates named in this
bead are re-run, and the registry row must read `adopted` with a receipt newer than the pin move. Phase `post` also runs `frob foundry check --fab --fixture <part> --process <p> --key "$(foundry_key post-<part>)" --json` on the twenty gate parts (a `run` verb, so each call carries a key from the cluster preamble, N31a-51) and asserts
verdict, location and repair for each, and the `review: no-claim` line.

Logging per STD-E2E: the script sources the shared helper and writes NDJSON to
`target/e2e-logs/upstream_fd_fsim_29/<utc-timestamp>.ndjson` with a human-readable transcript beside it. Every line has `ts`,
`script`, `bead` (`FD-FSIM-29`), `step`, `event` (`start`, `cmd`, `assert`, `metric`, `refusal`, `end`), the exact
command line, exit code, wall time and the envelope's `evidence` block (claim, class, tier, determinism,
identities, seed). It also records the sibling's pinned commit, the stack-receipt hash, the phase, each measured
value with its unit and threshold, digests where state is compared and, on failure, the first differing value
and the artifact paths needed to reproduce. A final `summary` line gives assertions passed and failed and the
gates touched. No private-scene data is logged.

### Known answers and negative controls
- Known answers: the ceiling at β = 0; the chamfers at 60° and 30° against β_min = 45°; the 6 mm slot against tools of radius 2.5 mm and 4 mm.
- Negative controls: an overhang evaluator that ignores the sign of n·b must report upward faces and be caught by a clean part; a tool-access evaluator that tests the tool's axis only (radius zero) must pass the 4 mm tool through the 6 mm slot and be caught.
- Metamorphic relation: rotating the part and the build direction together leaves every measurement unchanged.
- Determinism lanes: exact on a triangle surface up to rounding in dot products; identical twice on one machine; regions are ordered by centroid so that output order is stable.

## Considerations

- "Manufacturable" is not claimed by this project at any tier of evidence: the report is a screen, and a build packet's `fabrication_released` flag is always false.
- `fs-fab` is thin today; this item roughly doubles it. Whether the evaluators belong there or beside the other geometric measures in `fs-geocon` is FrankenSim's call under its layer checks.
- Tool access by swept cylinders along given directions is a fixed-axis model. Foundry's `cnc-5axis` check is kept through it: Foundry passes, for each setup, the 32 sampled directions of the setup's tilt cone (N31a-28) as approach directions, so the local five-axis check is reproduced by the sibling's evaluator before the local one is deleted (the differential test includes the `cnc-5axis` fixtures). Continuous simultaneous five-axis tool paths are out of scope and are said to be so in the measurement's declared resolution.
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

Appendix F.1 (row FD-FSIM-29), F.3; §31.4 (`fs-fab`, `fs-geocon`), §31.10 (manufacturability), §33.9 (INT-FSIM-15); GATE-FOUNDRY-07.
