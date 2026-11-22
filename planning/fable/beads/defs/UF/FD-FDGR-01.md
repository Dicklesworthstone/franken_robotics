---
key: "FD-FDGR-01"
title: "upstream franken_drone_geometry_reconstruction: published metric geometry with coverage and uncertainty"
type: "task"
priority: 3
milestone: "M5"
labels: ["m5", "suite", "habitat", "upstream", "franken_drone_geometry_reconstruction", "frontier"]
parent: "WP-32"
deps: ["BD-037", "NF-01", "STD-E2E"]
needs: ["§33.15: the schema adapter in `frob-scene` and its fixtures (the M3 half of INT-FDGR-01), against which published geometry is first validated"]
related: ["BD-398", "BD-228", "BD-305", "FD-FSS-01"]
gates: ["INT-FDGR-01", "GATE-HAB-03"]
plan_refs: ["Appendix F.1", "Appendix F.3", "§33.15", "§19.13", "§33.4", "§37", "§42"]
---
## Background

A robot is designed for places. FrankenRobotics can ingest a real place (a home, a warehouse aisle) as a scene,
provided the reconstruction says how much of the place it saw, how sure it is, and what its scale rests on.
One of the owner's projects is built for exactly that evidence: `franken_drone_geometry_reconstruction`, a
reference workspace for drone video.

FACT (its status file, snapshot 2026-09-04, `IMPLEMENTATION_STATUS.md`, at `db45703afc5e`; about 27 thousand
lines): its executable chain runs from media custody through clocks, calibration, scale witnesses, keyframes,
correspondences and relative pose to an audited relative bundle problem; **it does not yet optimize cameras,
emit metres or publish geometry.** The schemas for what it will publish exist:
`schemas/geometry_generation.schema.json` requires `coordinate_frame`, `scale_status`, `representations`,
`uncertainty_root`, `coverage_root` and `publication_stage`; `schemas/scale_model.schema.json` records a scale
authority with residuals in parts per million; its scale vocabulary is `relative_only`, `estimated`,
`witnessed`, `surveyed`. Upstream register priority: **P3**. Tier: Frontier.

So this project consumes it as a pattern today: a schema adapter in `frob-scene` with no crate dependency, and
`scene.import` consumes the other pipeline (the walkthrough twin package) first, with scale declared by the
operator (OPEN-HAB-5). Rejected: importing a mesh and trusting it (a corridor whose far half was never
observed appears as empty space in a naive import, and a traversal certificate over it would be false);
accepting a relative-scale reconstruction into a physics scene (a relative scale cannot enter one).

## Scope

In: in that repository, the remaining steps to a published, metric geometry generation that satisfies its own
schemas; here, the subprocess import lane switched on and its gates run on one capture. Out: the schema
adapter and `scene.import` (BD-398); scene acceptance (BD-228); privacy handling of real-place data (BD-305);
the twin-package route (FD-FSS-01).

## Technical approach

**Where the work is done, and under whose rules (binding).** The change is made in the `franken_drone_geometry_reconstruction` repository
(`../franken_drone_geometry_reconstruction` beside this one; head `db45703afc5e` and toolchain channel `nightly-2026-08-31` when this was written
on 2026-11-21, both FACT to re-read), by an agent of this program working there under that repository's own
`AGENTS.md`, gates and conventions.

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

**The change (DESIGN; the sibling's own plan and status file govern the order).** Three stages, each landing
separately:

1. *Optimized cameras.* The audited relative bundle problem is solved (bundle adjustment), with its residuals
   and covariance retained as evidence.
2. *Metres with a scale witness.* The relative reconstruction is scaled by a witness (an object or baseline of
   known length visible in the capture), producing a scale model with its authority and its residual in parts
   per million; `scale_status` becomes `witnessed`, or `surveyed` when the baseline was surveyed. With no
   witness the status stays `estimated` or `relative_only`, truthfully.
3. *Publication.* A geometry generation conforming to `geometry_generation.schema.json`: the coordinate frame;
   the representations (mesh, point cloud or distance field); a coverage root that says, per region, whether
   it was observed; an uncertainty root with per-region uncertainty; the publication stage. A command-line
   entry writes the generation to a directory.

Additive in the sense that applies to a growing project: the custody and audit chain already built is not
weakened to reach publication.

**Interim posture here.** `scene.import` consumes the twin package first; scale is declared by the operator
and the result carries `reality: no-claim`; the drone route is a schema adapter only. Fallback of the lane:
mesh and point-cloud import through `fs-io` with scale declared by the operator.

**Fixture (one reproducible failing fixture, DESIGN).** `tests/upstream/FD-FDGR-01/`, opened by BD-037 and kept current
by this bead: one test tagged `FD-FDGR-01`, compiled as the `frob-conformance` integration test `upstream_fd_fdgr_01` and
marked `#[ignore]` so that the default suite stays green, with `expected.toml` beside it holding the signature
recorded at the pinned commit (`observed_at`, the failing assertion and the observed value). It fails at the pin
and passes once the change is adopted; a fixture that passes at the pin no longer demonstrates the limit and is
itself a failure of the registry check. It runs the sibling's command-line tool at the pinned commit on its own
smallest sample capture and expects a geometry generation that validates against the sibling's schema with a
metric scale status; at the pin the tool publishes no geometry.

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

**What the switch-over re-runs.** "Run INT-FDGR-01, then GATE-HAB-03 on one capture." INT-FDGR-01: the schema
fixtures validate; a geometry generation whose scale status is not metric is refused; coverage masks become
the scene's unknown-space label, so that a clearance certificate treats an unscanned region as unknown and
never as free; the privacy inheritance of real-place data applies to the result. GATE-HAB-03 (Reality): one
real place ingested end to end, privately, with its scale label and unknown regions respected, and at least
one probe request emitted for it; a place for which no capture exists is recorded as `no-claim`, not as a
pass. The posture of the foundation moves from Pattern to Subprocess: the tool is invoked as a binary and no
crate of it enters this project's build graph.

**Fallback and kill.** Kill of the lane: no published geometry by the M5 entry review, after which the lane
stays a schema adapter. KILL-INGEST for ingestion as a whole: GATE-HAB-03 cannot be passed with a scale label
of `witnessed` or better after one repair cycle; fallback: the place is authored in the interior grammar from
a tape-measure survey, every surveyed dimension recorded as `measured`, and the pack says it is a proxy.

## Acceptance criteria

- [ ] In the sibling: its tool publishes a geometry generation for its sample capture that validates against its own schemas, with `scale_status` `witnessed` or `surveyed`, a scale residual in parts per million, and coverage and uncertainty roots.
- [ ] In the sibling: with the scale witness removed from the sample, the status is `estimated` or `relative_only`, never metric.
- [ ] Here: INT-FDGR-01 passes on the published generation (validates; metric scale accepted; non-metric refused; coverage becomes unknown space; privacy mark inherited).
- [ ] Here: GATE-HAB-03 is run on one capture, or recorded `no-claim` if no capture is admitted; the scene's scale label equals the sibling's status.
- [ ] Here: no crate of the sibling is in `Cargo.lock`; the import is a subprocess.
- [ ] The item's row in `registries/upstream.toml` names its fixture, this bead and the mirrored bead in the sibling's tracker; its `status` is `adopted`, or `landed` with the reason the switch-over has not run yet.
- [ ] Every file edited in the sibling was reserved through Agent Mail before the edit and released after landing; the change is on the sibling's `main` and passed that repository's own gates under its `AGENTS.md`.
- [ ] Nothing in this repository patches, forks or vendors the sibling; `cargo xtask check-stack` reports it on-pin and clean under the new receipt, and the record in `DEPENDENCY_ADMISSION.md` cites that receipt.

## Test plan

### Unit tests
In the sibling (added, under its own conventions):
- `scale_from_surveyed_baseline`: a baseline surveyed at 2.000 m measures 1.999 m in the scaled reconstruction; expected a recorded residual of 500 parts per million and status `surveyed`.
- `no_witness_is_not_metric`: expected `relative_only` or `estimated`.
- `coverage_root_marks_unobserved_half`: a synthetic corridor captured from one end only; the far half's regions are marked unobserved.
- `published_generation_validates`: against `geometry_generation.schema.json`.
Here:
- `non_metric_scale_is_refused`: a generation with `scale_status = "relative_only"`; expected the import refusal.
- `unobserved_region_is_unknown_not_free`: the half-observed corridor; a traversal query through the far half returns `unknown` (the counterexample CX-355 as a test).
- `privacy_mark_is_inherited`: a scene imported from a private capture is private, and so is every artifact derived from it.
- `no_sibling_crate_in_lockfile`: a search of `Cargo.lock`.

### End-to-end script and logging
`scripts/e2e/upstream_fd_fdgr_01.sh --phase pre|post` (plain shell, seed-explicit, idempotent, non-zero on failure).
Phase `pre`, valid at any pin before adoption: `cargo xtask check-stack`; `frob doctor --json` (the registry
section shows this item with its status); `cargo test -p frob-conformance --test upstream_fd_fdgr_01 -- --ignored`,
asserting that the fixture fails with the signature in `expected.toml`; then the interim posture's own tests,
asserting that they pass. Phase `post`, after the pin move: the same fixture must pass, the gates named in this
bead are re-run, and the registry row must read `adopted` with a receipt newer than the pin move. Phase `post` also runs `frob scene import --json` on the published sample generation and
`frob scene accept --json`, asserting the scale label, the unknown-space mask and the private mark; the log
records identities and labels only, never capture content.

Logging per STD-E2E: the script sources the shared helper and writes NDJSON to
`target/e2e-logs/upstream_fd_fdgr_01/<utc-timestamp>.ndjson` with a human-readable transcript beside it. Every line has `ts`,
`script`, `bead` (`FD-FDGR-01`), `step`, `event` (`start`, `cmd`, `assert`, `metric`, `refusal`, `end`), the exact
command line, exit code, wall time and the envelope's `evidence` block (claim, class, tier, determinism,
identities, seed). It also records the sibling's pinned commit, the stack-receipt hash, the phase, each measured
value with its unit and threshold, digests where state is compared and, on failure, the first differing value
and the artifact paths needed to reproduce. A final `summary` line gives assertions passed and failed and the
gates touched. No private-scene data is logged.

### Known answers and negative controls
- Known answers: 0.001 m in 2.000 m is 500 parts per million; CX-355 (a corridor whose far half was never observed appears as empty space in a naive import; with the coverage mask the same query returns unknown).
- Negative controls: an import that ignores the coverage root must answer `free` in the unobserved half and be caught; a generation claiming `witnessed` without a scale model must be refused by schema validation.
- Determinism lanes: schema validation and label mapping are exact; the sibling's reconstruction carries its own determinism statement, recorded with the import.

## Considerations

- This is the sibling's own roadmap, and it is the largest item in the register after the Metal gateway; it is worked in its three stages, and this project benefits only from the third.
- Real-place data is governed by the privacy rules: the pack is private, is stored outside the repository, and everything derived from it inherits the mark; logs carry no private-scene data.
- OPEN-HAB-5 (which reconstruction package and version is the import contract; default: the walkthrough twin package, and the drone route when it publishes geometry; owner: Habitat lead) is decided at GATE-HAB-03.
- Ceiling, stated: one import lane among several; ingestion has no effect on decision time, and it is the precondition for any Reality claim about a particular place.
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

Appendix F.1 (row FD-FDGR-01), F.3; §33.15 (the scan-import lane), §19.13 (ingesting real places; KILL-INGEST), §33.4 (FD-FDGR), §37, §42 (OPEN-HAB-5); INT-FDGR-01; GATE-HAB-03; CX-355.
