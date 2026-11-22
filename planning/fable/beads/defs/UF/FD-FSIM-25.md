---
key: "FD-FSIM-25"
title: "upstream frankensim: a correspondence-search lane in `fs-asbuilt`"
type: "task"
priority: 3
milestone: "M5"
labels: ["m5", "suite", "ground", "foundry", "upstream", "frankensim", "frontier"]
parent: "WP-68"
deps: ["BD-037", "BD-395", "NF-01", "STD-E2E"]
needs: []
related: ["BD-407", "BD-408", "BD-401", "FD-FSIM-23"]
gates: ["GATE-GROUND-08"]
plan_refs: ["Appendix F.1", "Appendix F.3", "§30.4", "§30.14", "§30.21", "§31.12", "§33.9"]
---
## Background

As-built ingestion compares a manufactured part with its design: a scan of the part is registered to the
design geometry, and the deviations become an as-built variant of the model with a new `ModelId`. The
registration comes from FrankenSim's `fs-asbuilt` (about 10.8 thousand lines, layer L2, depends on `fs-qty`):
closed-form two-dimensional rigid registration from at least three non-collinear fiducials (`register`);
three-dimensional weighted rigid and similarity registration **with known correspondences**, a scale estimate
with its standard error and a unit-suspicion diagnostic, and a calibrated six-parameter pose covariance
(`rigid3`); datum-priority 3-2-1 registration (`datum`); deviation, thickness, form and roughness fields from
supplied correspondences (`field`).

FACT (read 2026-11-21): correspondences are supplied, never discovered: **there is no closest-point matching**.
Upstream register priority: **P3**.

So this project makes the correspondences known by construction: datums and fiducials are placed at design time
and travel in the build packet, which is what makes registration well-posed. That is good practice and it
excludes any part that was not designed here or was scanned without its fiducials visible. Rejected: an
iterative-closest-point routine in this repository feeding `fs-asbuilt` (it would be the step that decides the
pose, sitting outside the crate that states the pose's uncertainty).

## Scope

In: in `fs-asbuilt`, a lane that discovers correspondences between a scan and a design surface and then calls
the existing closed-form fit; here, the as-built known answers re-run without fiducials. Out: datum and
fiducial placement and the build packet (BD-407, BD-408); as-built ingestion on this side (BD-395); density,
porosity and mass properties, which the crate does not read.

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

**The change (DESIGN).** A lane with this contract:

1. *Inputs.* A scan (points, optionally with normals and per-point noise), a design surface, an initial pose
   with its uncertainty, and limits (iterations, the trimming fraction, the search radius).
2. *Iteration.* For each scan point inside the search radius, the closest point on the design surface; trim the
   worst residuals by the declared fraction; call the existing weighted rigid (or similarity) fit on the
   surviving pairs; repeat until the pose change is below a threshold or the iteration limit is reached.
3. *Outputs.* The correspondences, the pose with its covariance from the existing calibrated estimator, the
   residual field, the trimmed share, and the convergence history. The result has **estimate authority** only,
   as everything in this crate does.
4. *Refusals.* It refuses, by name: an initial pose outside its stated basin; a geometry whose pose is not
   determined by its shape (a surface of revolution leaves a rotation free; a plane leaves three motions free),
   reporting the undetermined directions from the rank of the fit's normal matrix; a scale that suggests a unit
   error (the existing unit-suspicion diagnostic).

Additive: the supplied-correspondence entries are unchanged.

**Interim posture here.** Datums and fiducials are placed at design time so correspondences are known; a scan
without them is not admitted for registration.

**Fixture (one reproducible failing fixture, DESIGN).** `tests/upstream/FD-FSIM-25/`, opened by BD-037 and kept current
by this bead: one test tagged `FD-FSIM-25`, compiled as the `frob-conformance` integration test `upstream_fd_fsim_25` and
marked `#[ignore]` so that the default suite stays green, with `expected.toml` beside it holding the signature
recorded at the pinned commit (`observed_at`, the failing assertion and the observed value). It fails at the pin
and passes once the change is adopted; a fixture that passes at the pin no longer demonstrates the limit and is
itself a failure of the registry check. It submits a synthetic point cloud of one link with a planted pose and no
correspondences through the airlock trait `AsBuilt`; at the pin the request is refused as a missing capability.

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

**What the switch-over re-runs.** The as-built known answers **without fiducials** (GATE-GROUND-08: the as-built
known answers of the synthetic rehearsal; the as-built variant has a new `ModelId`): a planted rotation and
translation and a planted 0.3 mm wall deviation are recovered inside the stated uncertainty from the cloud
alone; the two refusals of the fiducial route keep their analogues (too little geometry to fix the pose is
refused; a scale of 25.4 raises unit suspicion). Fiducial registration stays the preferred route where
fiducials exist, and the two routes are compared on the fixtures that have both.

**Fallback if the change cannot land.** Fiducials and datums by design; scans without them are refused.

## Acceptance criteria

- [ ] In FrankenSim: the lane exists with the four parts of the contract; the supplied-correspondence entries and their tests are unchanged.
- [ ] In FrankenSim: on the planted-pose fixture it recovers the pose inside the stated covariance in at least 95 of 100 seeded noise draws, and refuses the surface of revolution and the out-of-basin start.
- [ ] Here: the airlock exposes the lane at the new pin; an as-built variant made through it records the route (`correspondence-search`), the trimmed share and the initial pose.
- [ ] Here: GATE-GROUND-08 passes without fiducials, and on fixtures with fiducials the two routes' poses agree within their combined uncertainty.
- [ ] The item's row in `registries/upstream.toml` names its fixture, this bead and the mirrored bead in the sibling's tracker; its `status` is `adopted`, or `landed` with the reason the switch-over has not run yet.
- [ ] Every file edited in the sibling was reserved through Agent Mail before the edit and released after landing; the change is on the sibling's `main` and passed that repository's own gates under its `AGENTS.md`.
- [ ] Nothing in this repository patches, forks or vendors the sibling; `cargo xtask check-stack` reports it on-pin and clean under the new receipt, and the record in `DEPENDENCY_ADMISSION.md` cites that receipt.

## Test plan

### Unit tests
In FrankenSim (added, in `fs-asbuilt`):
- `recovers_planted_pose`: an L-shaped bracket sampled at 5,000 points with 0.05 mm noise, planted rotation 3° about (1, 1, 0)/√2 and translation (1.0, −0.5, 0.2) mm, initial pose identity; expected the planted pose inside the reported 95% region.
- `recovers_wall_deviation`: one wall offset by 0.3 mm; expected the deviation field's mean on that wall within 0.02 mm of 0.3 mm, and near zero elsewhere.
- `cylinder_is_refused_with_free_rotation`: a plain cylinder; expected the named refusal listing rotation about the axis as undetermined.
- `out_of_basin_start_is_refused`: initial rotation of 90°; expected the named refusal, not a wrong pose.
- `unit_suspicion_at_25_4`: a scan in inches against a design in millimetres under the similarity fit; expected the diagnostic.
Here:
- `asbuilt_variant_has_new_model_id`: the variant's identity differs from the design's and names the route.
- `fiducial_and_search_routes_agree`: on a fixture with both, the pose difference is inside the combined uncertainty.

### End-to-end script and logging
`scripts/e2e/upstream_fd_fsim_25.sh --phase pre|post` (plain shell, seed-explicit, idempotent, non-zero on failure).
Phase `pre`, valid at any pin before adoption: `cargo xtask check-stack`; `frob doctor --json` (the registry
section shows this item with its status); `cargo test -p frob-conformance --test upstream_fd_fsim_25 -- --ignored`,
asserting that the fixture fails with the signature in `expected.toml`; then the interim posture's own tests,
asserting that they pass. Phase `post`, after the pin move: the same fixture must pass, the gates named in this
bead are re-run, and the registry row must read `adopted` with a receipt newer than the pin move. Phase `post` also runs the as-built ingestion verb on the planted-pose cloud with and
without fiducials (the verb BD-395 defines, on a dataset whose manifest declares `origin = "synthetic"`), asserting the recovered values.

Logging per STD-E2E: the script sources the shared helper and writes NDJSON to
`target/e2e-logs/upstream_fd_fsim_25/<utc-timestamp>.ndjson` with a human-readable transcript beside it. Every line has `ts`,
`script`, `bead` (`FD-FSIM-25`), `step`, `event` (`start`, `cmd`, `assert`, `metric`, `refusal`, `end`), the exact
command line, exit code, wall time and the envelope's `evidence` block (claim, class, tier, determinism,
identities, seed). It also records the sibling's pinned commit, the stack-receipt hash, the phase, each measured
value with its unit and threshold, digests where state is compared and, on failure, the first differing value
and the artifact paths needed to reproduce. A final `summary` line gives assertions passed and failed and the
gates touched. No private-scene data is logged.

### Known answers and negative controls
- Known answers: the planted rotation, translation and the 0.3 mm wall deviation, recovered inside stated uncertainty; with the fiducial route, two fiducials are refused and a scale of 25.4 raises unit suspicion.
- Negative controls: a matcher without trimming must be pulled off by the planted wall deviation and report a biased pose (caught by the planted-pose test); a lane that returns a pose for the cylinder must be caught by the refusal test.
- Determinism lanes: closest-point ties are broken by index, so results are identical twice on one machine and across worker counts.

## Considerations

- A pose found by search carries a risk the closed form does not: a wrong local minimum that looks converged. The basin statement and the refusals are the content of this item, more than the iteration.
- `fs-asbuilt` emits estimated candidates and has no promotion beyond that (`NoEvidenceVerifier`); an as-built variant therefore never raises a Reality claim by itself.
- Real scans are private data: they are stored outside the repository and everything derived from them inherits the mark; the fixtures here are synthetic.
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

Appendix F.1 (row FD-FSIM-25), F.3; §30.4 (what `fs-asbuilt` provides), §30.14 (as-built ingestion), §30.21 (known answers), §31.12 (datums and fiducials), §33.9 (INT-FSIM-15); GATE-GROUND-08.
