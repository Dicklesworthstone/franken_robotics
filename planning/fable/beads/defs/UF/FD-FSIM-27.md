---
key: "FD-FSIM-27"
title: "upstream frankensim: a plane-stress option in `fs-topols` (its sampled stress is plane strain)"
type: "task"
priority: 3
milestone: "M5"
labels: ["m5", "suite", "foundry", "upstream", "frankensim", "frontier"]
parent: "WP-70"
deps: ["BD-037", "BD-403", "N33b-04", "STD-E2E"]
needs: []
related: ["FD-FSIM-28", "FD-FSIM-30", "N31a-04", "N31a-16", "N33b-14"]
gates: ["GATE-FOUNDRY-04"]
plan_refs: ["Appendix F.1", "Appendix F.3", "§31.4", "§31.7", "§33.9", "§33.8"]
---
## Background

Foundry sizes and shapes robot links. Plate-like parts (brackets, side plates) are shaped in two dimensions
with FrankenSim's `fs-topols` (about 15.7 thousand lines, layer L4): level-set optimization on a bilinear grid
level set with exact enclosures, high-order advection, redistancing, velocity extension, topological
derivatives for hole nucleation, compliance descent with volume control, multi-load robust selection and
sampled stress admission. FACT (read 2026-11-21): it is two-dimensional only, and **its sampled stress is plane
strain, while a thin plate is a plane-stress problem**. Upstream register priority: **P3**.

The two differ by a known factor. With Young's modulus E and Poisson's ratio ν, in-plane stress σ and strain ε:

  plane stress: σ = E/(1 − ν²)·[[1, ν, 0], [ν, 1, 0], [0, 0, (1 − ν)/2]]·ε;
  plane strain: σ = E/((1 + ν)·(1 − 2ν))·[[1 − ν, ν, 0], [ν, 1 − ν, 0], [0, 0, (1 − 2ν)/2]]·ε.

Plane strain is plane stress with E′ = E/(1 − ν²) and ν′ = ν/(1 − ν): for ν = 0.33 the body is 12% stiffer in
uniaxial loading, and stress concentrations shift. A plate optimized under the wrong assumption is checked
against a stress cap it does not actually meet. Rejected: correcting plane-strain results by a scalar factor
(the factor is exact for uniaxial loading only).

## Scope

In: in `fs-topols`, a constitutive option for plane stress with a thickness, alongside plane strain; here, a
plane-stress known answer in the Foundry fixtures. Out: three-dimensional topology optimization
(`fs-topopt::sdf3`, FD-FSIM-28); the independent solver that checks plates in three dimensions (BD-402).

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

**The change (DESIGN).** A constitutive selector on the two-dimensional problem: `PlaneStrain` (the default,
unchanged) and `PlaneStress { thickness }`. Under plane stress the stiffness uses the first matrix above scaled
by the thickness, the sampled stress is the in-plane stress with σ_zz = 0, the out-of-plane strain
ε_zz = −ν/(1 − ν)·(ε_xx + ε_yy) is reported, and compliance, volume and the topological derivative use the
matching coefficients. The stress admission (`stress`, `robust_stress`) states which assumption its samples
were computed under. Additive: every existing result is unchanged under the default.

**Interim posture here.** Three-dimensional numbers only for anything whose stress is used as evidence; plate
parts use `fs-topols` under plane strain for shape in two dimensions only, and are extruded and checked in three
by the independent solver (N31a-16; the same posture in N31a-04's gap table).

**Fixture (one reproducible failing fixture, DESIGN).** `tests/upstream/FD-FSIM-27/`, opened by BD-037 and kept current
by this bead: one test tagged `FD-FSIM-27`, compiled as the `frob-conformance` integration test `upstream_fd_fsim_27` and
marked `#[ignore]` so that the default suite stays green, with `expected.toml` beside it holding the signature
recorded at the pinned commit (`observed_at`, the failing assertion and the observed value). It fails at the pin
and passes once the change is adopted; a fixture that passes at the pin no longer demonstrates the limit and is
itself a failure of the registry check. It requests a plane-stress run through the airlock trait `StructuralOpt`
(refused at the pin as a missing capability) and, when available, runs the strip known answer below.

**Switch-over (the compatibility train).** When the change is on the sibling's `main`: (1) move the sibling's pin
in `franken-stack.lock` to a clean commit that contains it (a pin only moves forward; a sibling's newer clean head
is never rolled back locally); (2) write a new resolution receipt (`frob.stack-receipt/1`) under `receipts/`
with `cargo xtask check-stack --write-receipt --reason pin-move --gate INT-FSIM-01` (plain `check-stack` only
verifies and writes nothing); (3) re-run that sibling's admission gates, the parity gates and the
gates this item names; (4) relock (`Cargo.lock` regenerated and committed, builds `--locked` on the one toolchain
T) and append the record, with the receipt hash, to `DEPENDENCY_ADMISSION.md`. Only then does the row's `status`
become `adopted` and the interim posture end. No PASS is recorded on a dirty sibling tree, and neither a pin nor
T moves while a ten-fold tournament is running. If a gate named here does not exist yet when the change lands,
the pin still moves with the gates that do exist, the row stays `landed`, and the gate's first run is made
against the new pin.

**What the switch-over re-runs.** The rule is "add the plane-stress known answer": the strip and plate cases
below join the Foundry structure fixtures, and GATE-FOUNDRY-04 is re-run (two-bar compliance within 10% and bar
angle within 5°; the extracted surface meshes and passes the mesh audit on 20 of 20 fixtures). Plates may then
be shaped under plane stress; the three-dimensional check stays, as the independent solver of the structure
loop.

**Fallback if the change cannot land.** Shape in two dimensions under plane strain, check in three; no
two-dimensional stress is used as evidence.

## Acceptance criteria

- [ ] In FrankenSim: the option exists; under the default every existing test and value is unchanged.
- [ ] In FrankenSim: the strip and the plate-with-hole cases below pass under plane stress.
- [ ] Here: the airlock passes the selector; a result records the assumption it was computed under, and citing a plane-strain stress for a plate is refused by the Foundry check.
- [ ] Here: the plane-stress known answers are rows of the structure fixtures and pass; GATE-FOUNDRY-04 passes at the new pin.
- [ ] The item's row in `registries/upstream.toml` names its fixture, this bead and the mirrored bead in the sibling's tracker; its `status` is `adopted`, or `landed` with the reason the switch-over has not run yet.
- [ ] Every file edited in the sibling was reserved through Agent Mail before the edit and released after landing; the change is on the sibling's `main` and passed that repository's own gates under its `AGENTS.md`.
- [ ] Nothing in this repository patches, forks or vendors the sibling; `cargo xtask check-stack` reports it on-pin and clean under the new receipt, and the record in `DEPENDENCY_ADMISSION.md` cites that receipt.

## Test plan

### Unit tests
In FrankenSim (added, in `fs-topols`):
- `uniaxial_strip_both_assumptions`: E = 70 GPa, ν = 0.33, σ_xx = 100 MPa with free lateral edges. Plane stress: ε_xx = σ/E = 1.4286·10⁻³ and ε_zz = −ν·σ/E = −4.714·10⁻⁴. Plane strain: ε_xx = σ·(1 − ν²)/E = 1.2730·10⁻³ and ε_zz = 0. Expected to 10⁻⁹ relative.
- `equivalence_by_substitution`: plane strain with (E, ν) equals plane stress with (E/(1 − ν²), ν/(1 − ν)) on 10³ random strain states, to 10⁻¹².
- `plate_with_hole_concentration`: a wide plate with a small circular hole under remote uniaxial stress; expected peak stress at the hole's edge within 5% of 3 times the remote stress at the stated grid refinement (the plane-stress Kirsch solution).
- `thickness_scales_compliance`: doubling the thickness halves the compliance under a fixed in-plane load.
Here:
- `result_records_constitutive_assumption`: present and correct for both selectors.
- `plane_strain_stress_is_not_cited_for_a_plate`: the Foundry check refuses it by name.

### End-to-end script and logging
`scripts/e2e/upstream_fd_fsim_27.sh --phase pre|post` (plain shell, seed-explicit, idempotent, non-zero on failure).
Phase `pre`, valid at any pin before adoption: `cargo xtask check-stack`; `frob doctor --json` (its `upstream`
section shows this item with its status); `cargo test -p frob-conformance --test upstream_fd_fsim_27 -- --ignored`,
asserting that the fixture fails with the signature in `expected.toml`; then the interim posture's own tests,
asserting that they pass. Phase `post`, after the pin move: the same fixture must pass, the gates named in this
bead are re-run, and the registry row must read `adopted` with a receipt newer than the pin move. Phase `post` also runs `frob foundry optimize --fixture bracket-plate --plate --rung topology
--exploratory --key fd27-opt --json` on the plate fixture under plane stress and `frob foundry check <result>
--key fd27-chk --json` for the independent three-dimensional check (`foundry.check`, BD-402; `foundry.size` is
the beam rung and checks nothing in three dimensions), asserting agreement of peak stress within the fixture's
stated band.

Logging per STD-E2E: the script sources the shared helper and writes NDJSON to
`target/e2e-logs/upstream_fd_fsim_27/<utc-timestamp>.ndjson` with a human-readable transcript beside it. Every line has `ts`,
`script`, `bead` (`FD-FSIM-27`), `step`, `event` (`start`, `cmd`, `assert`, `metric`, `refusal`, `end`), the exact
command line, exit code, wall time and the envelope's `evidence` block (claim, class, tier, determinism,
identities, seed). It also records the sibling's pinned commit, the stack-receipt hash, the phase, each measured
value with its unit and threshold, digests where state is compared and, on failure, the first differing value
and the artifact paths needed to reproduce. A final `summary` line gives assertions passed and failed and the
gates touched. No private-scene data is logged.

### Known answers and negative controls
- Known answers: the strip strains above; the factor 1/(1 − ν²) = 1.1222 at ν = 0.33; the stress concentration factor 3 at a circular hole.
- Negative controls: a plane-stress option implemented by scaling plane-strain results by a constant must fail `equivalence_by_substitution` on shear-dominated states; a result with no recorded assumption must be rejected at the airlock.
- Determinism lanes: twice on one machine; debug and release; same-platform class.

## Considerations

- `fs-topols` states that its sampled stress is an admission sample and not a continuum bound; plane stress does not change that. Structural evidence in a build packet still comes from the three-dimensional solve.
- The structure group adds 13 crates and about 412 thousand lines to the M1 closure when enabled (FACT); it is compiled only by the Foundry lanes at M5.
- Thin plates buckle; the buckling check is a separate analysis and is not affected by this option.
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

Appendix F.1 (row FD-FSIM-27), F.3; §31.4 (`fs-topols`), §31.7 (sizing and topology optimization of links), §33.9 (INT-FSIM-15); GATE-FOUNDRY-04.
