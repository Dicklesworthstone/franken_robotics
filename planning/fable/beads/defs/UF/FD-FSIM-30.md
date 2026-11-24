---
key: "FD-FSIM-30"
title: "upstream frankensim: a statement of boundary conformity in `fs-mesh`"
type: "task"
priority: 3
milestone: "M5"
labels: ["m5", "suite", "foundry", "upstream", "frankensim", "frontier"]
parent: "WP-70"
deps: ["BD-037", "BD-403", "N33b-04", "STD-E2E"]
needs: []
related: ["N31a-04", "N31a-17", "N31b-24", "N33b-14", "N33bx-04"]
gates: ["GATE-FOUNDRY-04"]
plan_refs: ["Appendix F.1", "Appendix F.3", "§31.4", "§31.7", "§31.8", "§31.20", "§33.8"]
---
## Background

A structural result is a statement about a body. Foundry computes stress, buckling factors and modes of a link
on a tetrahedral mesh of it, and the result is evidence about the link only as far as the mesh's boundary is
the link's surface. The mesh comes from FrankenSim's `fs-mesh` (layer L2): incremental Delaunay
tetrahedralization on exact predicates with quality refinement and audits, and constrained recovery and
labelled volumetric meshing of piecewise-linear complexes (`recovery`, `volumetric`). FACT (read 2026-11-21):
**its crate documentation still lists full boundary conformity as successor scope.** Whether an optimized link
surface meshes cleanly is an admission test inside GATE-FOUNDRY-04. Upstream register priority: **P3**.

Without a statement from the mesher, this project measures conformity itself, on every mesh, and stores the
measurement with the structural evidence. That is sound and it is repeated work with no general guarantee: a
surface that happened to mesh cleanly says nothing about the next one. Rejected: assuming conformity because
the mesher returned without error (the crate itself says it does not yet promise it); remeshing with a
third-party tool (outside the suite's dependency rules, and with no exact-predicate audit).

## Scope

In: in `fs-mesh`, a contract statement of when the output's boundary equals the input's, with a typed outcome
when it does not, and tests; here, use of the statement in place of the per-mesh measurement where it applies.
Out: the mesh audit of the structure fixtures (BD-403, BD-402); surface extraction from an optimized level set
(Foundry's own); mesh quality, which the crate already audits.

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

**The change (DESIGN).** A section of the crate's contract and a field of its result:

1. *The class.* The inputs for which conformity is guaranteed: a closed, manifold, oriented piecewise-linear
   surface whose features satisfy stated bounds (for example a minimum dihedral angle and a minimum local
   feature size relative to the requested element size).
2. *The guarantee.* For inputs in the class, every input facet is exactly the union of boundary faces of the
   output mesh (Steiner points may be added on input facets and edges; none is moved off them), so the mesh's
   boundary and the input surface are the same point set, and the meshed volume equals the enclosed volume.
3. *The outcome type.* The mesher returns `Conforming`, or `ConformingWithSteiner { count }`, or
   `NotConforming { facets }` listing the input facets it could not recover; it never returns a mesh whose
   boundary differs from the input without saying so. An input outside the class is reported as such, and its
   result is then measured, not guaranteed.

Additive: meshes produced today are unchanged; they gain a statement about themselves.

**Interim posture here.** Conformity is measured per mesh and recorded with the structural evidence: the
maximum distance from mesh boundary vertices to the input surface and from input vertices to the mesh boundary
(both must be zero to rounding for a piecewise-linear input), and the relative difference between meshed and
enclosed volume. A mesh that fails the measurement is refused as evidence. The structural check of that surface
then follows the default of OPEN-FOUNDRY-1 (decided by N31b-24): a second cut-cell solve at a different
resolution and background offset, whose result carries the weaker label `check: same-family`, never the label of
an independent body-fitted check. Until N31b-24 records its decision, GATE-FOUNDRY-04 reads
`blocked: OPEN-FOUNDRY-1` for any surface that did not mesh conformingly.

**Fixture (one reproducible failing fixture, DESIGN).** `tests/upstream/FD-FSIM-30/`, opened by BD-037 and kept current
by this bead: one test tagged `FD-FSIM-30`, compiled as the `frob-conformance` integration test `upstream_fd_fsim_30` and
marked `#[ignore]` so that the default suite stays green, with `expected.toml` beside it holding the signature
recorded at the pinned commit (`observed_at`, the failing assertion and the observed value). It fails at the pin
and passes once the change is adopted; a fixture that passes at the pin no longer demonstrates the limit and is
itself a failure of the registry check. It meshes a cube through the airlock and reads the conformity outcome
from the result; at the pin the result has no such field.

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

**What the switch-over re-runs.** The rule is "replace the measurement with the stated bound where it
applies": for an input inside the stated class whose outcome is `Conforming` or `ConformingWithSteiner`, the
structural evidence cites the sibling's contract and outcome instead of this project's measurement; for an
input outside the class, or a `NotConforming` outcome, the measurement is still made and still decides, and a
`NotConforming` mesh is admitted only when the measurement passes and the listed facets lie outside every gauge
region of the check (BD-402); otherwise it is refused as evidence, naming the facets.
GATE-FOUNDRY-04 is re-run (two-bar compliance within 10% and bar angle within 5°; the extracted surface meshes
and passes the mesh audit on 20 of 20 fixtures), and on those twenty the measurement is run once more beside
the outcome to confirm that the statement and the measurement agree.

**Fallback if the change cannot land.** Per-mesh measurement, as today; surfaces that fail it take the
`check: same-family` route of OPEN-FOUNDRY-1. A landed statement shrinks that route to the inputs outside the
stated class or with a `NotConforming` outcome, which is the user-visible gain: fewer structural results carry
the weaker label.

## Acceptance criteria

- [ ] In FrankenSim: the contract states the class and the guarantee; the result carries the outcome type; its tests cover a conforming case, a Steiner case and a case it must report as not recovered.
- [ ] Here: the airlock exposes the outcome; structural evidence records either "conformity: stated (contract version, outcome)" or "conformity: measured (three numbers)", never neither.
- [ ] Here: on the twenty fixtures of GATE-FOUNDRY-04 the stated outcome and the measurement agree on all twenty, and the gate passes at the new pin.
- [ ] Here: a `NotConforming` outcome makes the mesh inadmissible as evidence unless the measurement passes and the non-recovered facets lie outside every gauge region; the refusal names the facets.
- [ ] The item's row in `registries/upstream.toml` names its fixture, this bead and the mirrored bead in the sibling's tracker; its `status` is `adopted`, or `landed` with the reason the switch-over has not run yet.
- [ ] Every file edited in the sibling was reserved through Agent Mail before the edit and released after landing; the change is on the sibling's `main` and passed that repository's own gates under its `AGENTS.md`.
- [ ] Nothing in this repository patches, forks or vendors the sibling; `cargo xtask check-stack` reports it on-pin and clean under the new receipt, and the record in `DEPENDENCY_ADMISSION.md` cites that receipt.

## Test plan

### Unit tests
In FrankenSim (added, in `fs-mesh`):
- `cube_is_conforming`: the unit cube as twelve triangles; expected outcome `Conforming` or `ConformingWithSteiner`; the sum of tetrahedron volumes equals 1 to 10⁻¹²; the boundary area equals 6 to 10⁻¹²; every boundary face lies in one of the six planes exactly.
- `l_bracket_with_reentrant_edge`: a concave edge at 90°; expected conforming, with the re-entrant edge present as mesh edges.
- `thin_sliver_outside_class_is_reported`: a wedge with a dihedral angle of 2°, below the stated bound; expected the input reported as outside the class.
- `unrecoverable_facet_is_listed`: a constructed input for which recovery is declined at the requested element size; expected `NotConforming` naming the facet.
Here:
- `evidence_records_conformity_source`: stated or measured, for every structural result.
- `measurement_agrees_with_statement`: on the twenty gate fixtures.
- `measured_cube_numbers`: the three numbers for the cube are 0, 0 and 0 to rounding.

### End-to-end script and logging
`scripts/e2e/upstream_fd_fsim_30.sh --phase pre|post` (plain shell, seed-explicit, idempotent, non-zero on failure).
Phase `pre`, valid at any pin before adoption: `cargo xtask check-stack`; `frob doctor --json` (its `upstream`
section shows this item with its status); `cargo test -p frob-conformance --test upstream_fd_fsim_30 -- --ignored`,
asserting that the fixture fails with the signature in `expected.toml`; then the interim posture's own tests,
asserting that they pass. Phase `post`, after the pin move: the same fixture must pass, the gates named in this
bead are re-run, and the registry row must read `adopted` with a receipt newer than the pin move. Phase `post`
also runs the twenty-fixture battery lane of GATE-FOUNDRY-04 (N31a-18's `topology_battery`, reached through
BD-403) and parses its report, asserting the conformity line (stated or measured) of each fixture, and runs the
verb route on the two-bar fixture: `frob foundry optimize --fixture twobar --rung topology --exploratory --key
fd30-post-opt --json`, then `frob foundry check <result> --key fd30-post-chk --json` (the independent check,
BD-402, is where the mesh and its conformity record are made; `foundry.size` meshes nothing), asserting the
conformity line of the check record.

Logging per STD-E2E: the script sources the shared helper and writes NDJSON to
`target/e2e-logs/upstream_fd_fsim_30/<utc-timestamp>.ndjson` with a human-readable transcript beside it. Every line has `ts`,
`script`, `bead` (`FD-FSIM-30`), `step`, `event` (`start`, `cmd`, `assert`, `metric`, `refusal`, `end`), the exact
command line, exit code, wall time and the envelope's `evidence` block (claim, class, tier, determinism,
identities, seed). It also records the sibling's pinned commit, the stack-receipt hash, the phase, each measured
value with its unit and threshold, digests where state is compared and, on failure, the first differing value
and the artifact paths needed to reproduce. A final `summary` line gives assertions passed and failed and the
gates touched. No private-scene data is logged.

### Known answers and negative controls
- Known answers: the cube's volume 1 and area 6; cantilever deflection within 2% and stress within 5% after refinement on a conforming mesh (GATE-FOUNDRY-01), which is the downstream reason conformity matters.
- Negative controls: a mesh whose boundary vertex is displaced by 10⁻⁶ m off an input face must fail the measurement and must not be reported `Conforming`; a mesher result with no outcome field must be treated as "measured" and never as "stated"; a structural result computed on a mesh that failed the measurement must carry `check: same-family` (or be refused), never the independent-check label.
- Determinism lanes: exact predicates make the outcome identical twice on one machine and across platforms; recorded with the mesh's hash.

## Considerations

- Full boundary conformity is on `fs-mesh`'s own roadmap; this item asks that the roadmap item land as a statement a consumer can cite, with the class written down, not only as an implementation.
- A smoothed or curved design surface is piecewise-linear only after extraction; the extraction error is Foundry's to bound and is separate from mesh conformity.
- P3 and off every critical path: the measurement is cheap and already decides.
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

Appendix F.1 (row FD-FSIM-30), F.3; §31.4 (`fs-mesh`), §31.7, §31.8, §31.20 (OPEN-FOUNDRY-1), §33.8; GATE-FOUNDRY-04, GATE-FOUNDRY-01.
