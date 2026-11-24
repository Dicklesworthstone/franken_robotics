---
key: "FD-FSS-01"
title: "upstream franken_surveillance_system (conditional): twin package landmarks, scale witness, coverage"
type: "task"
priority: 3
milestone: "M5"
labels: ["m5", "suite", "habitat", "upstream", "franken_surveillance_system", "frontier", "review-notes-open"]
parent: "WP-32"
deps: ["BD-037", "N19c-01", "N33b-04", "STD-E2E"]
needs: []
related: ["BD-398", "N19c-04", "N19c-06", "N19c-08", "N19c-09", "N33c-25", "NFp2-03"]
gates: []
plan_refs: ["Appendix F.1", "Appendix F.3", "§19.13", "§19.5", "§33.4", "§37", "§42"]
---
## Background

Real places enter FrankenRobotics through `scene.import`. Of the two pipelines in the owner's projects, the one
it consumes first is the route from walkthrough video to an agent-built Blender twin with stable feature
identifiers and a GLB export, for which an accepted plan describes a **twin package** linking visual, geometric
and traversability representations (FACT: the reconstruction skills and
`franken_surveillance_system/BLENDER_TWIN_INTEGRATION_PLAN.md`). The posture of that sibling here is Pattern:
the twin package and the camera-model contract.

Import adds what a physics scene needs: the collision compile and acceptance, including **four-point
registration**; interface and optical cards, marked `assumed` unless measured; the unknown mask; and the scale
label. For that, a twin package must carry three things: four non-coplanar landmarks (to fix the registration
of the twin to the scene frame without a mirror ambiguity); a scale witness (so that the scene's scale label,
one of `relative-only`, `estimated`, `witnessed` and `surveyed` in order of authority, can be `witnessed`); and coverage or confidence per region (so that unobserved space is
unknown and never free).

This item is **conditional**. The paper-scale pilot of real-place ingestion, run before the importer's work
package is funded, takes one existing twin package and checks by inspection that it exports those three
things. If it does, no upstream change is needed and this bead is closed as not needed. If it does not, "the
first deliverable is an upstream work item and not an importer". Upstream register priority: **P3**. Tier:
Frontier.

Rejected: an importer that guesses landmarks or assumes full coverage (a traversal certificate over
unobserved space would be false).

## Scope

In: recording the pilot's finding; if needed, the three additions to the twin package in its repository; here,
the validator fixture and the record of the package version exported from the landed commit. Out: the pilot itself
(N19c-01, M3, which this bead reads); the switch-over (the package version into the import contract, the checks
through the importer and GATE-HAB-03 re-run at `witnessed` or better: NFp2-03, which depends on this bead and on
BD-398, so that this upstream work starts when the pilot reports instead of waiting for the M5 capstone);
`scene.import` and `scene.fit` (BD-398); scene acceptance and four-point registration (BD-228); the drone route
(FD-FDGR-01); privacy handling (BD-305).

## Technical approach

**Step 0, here: the pilot's record.** Copy the inspection pilot's finding (N19c-01's appended result in
`crates/frob-conformance/registry/pilots.toml`, cited by its id and digest) into
`tests/upstream/FD-FSS-01/PILOT.md`: which twin package and version was inspected, and for each of the three
things, present or missing, with the field or file where it was found. If all three are present (verdict
`falsifier-rejected`), set the registry row's `status` to `not-needed` (DESIGN: the value requested of BD-037's
schema `frob.upstream/1`, admitted only on a conditional row whose pilot found nothing missing, with `PILOT.md`
citing the pilot record; until the schema admits it the row stays `open` and `PILOT.md` carries the verdict
"not needed (pilot)"), and stop.

**Where the work is done, and under whose rules (binding), if step 0 finds something missing.** The change is
made in the `franken_surveillance_system` repository (beside this one; its head and toolchain were not read for
the plan and are recorded when the work starts), by an agent of this program working there under that
repository's own `AGENTS.md`, gates and conventions: read them first, and they govern; reserve every file to be
touched through Agent Mail, because other agents work there; the change is additive and lands on that
repository's `main` through its own gates; nothing in this repository patches or vendors the sibling
(INV-SUITE-04); the work is mirrored as a bead in the sibling's tracker, whose identifier is stored in the
`mirror` field of this item's row in `registries/upstream.toml`, and the row's `status` moves `open` →
`landed` → `adopted`.

**The change (DESIGN), only the parts the pilot found missing.**

1. *Landmarks.* At least four named landmarks with stable identifiers, each with its coordinates in the twin's
   frame and a description sufficient to find it in the place; the package states that they are not coplanar,
   and the exporter refuses to claim so if the tetrahedron they span has volume V below 1% (TARGET) of d³, where d
   is the largest of the six pairwise distances between the four landmarks. That is the importer's own rule for
   four-point registration (failure there is `FROB-REF-302`; CX-185), so a package the exporter accepts is never
   refused by the importer for near-coplanarity.
2. *Scale witness.* At least one measured length between two identified points of the twin, with the measuring
   method and its uncertainty.
3. *Coverage or confidence per region.* A partition of the twin into regions, each marked observed or not, or
   carrying a confidence value, exported beside the geometry.

Each is a new field or file of the package; existing fields are unchanged.

**Interim posture here.** A tape-measure survey authored in the interior grammar, every dimension recorded as
`measured`, and the pack says it is a proxy; or, with a twin package that lacks the three things, import with
scale declared by the operator and `reality: no-claim`.

**Fixture (DESIGN).** `tests/upstream/FD-FSS-01/`: a validator that reads a twin package and asserts the three
things; it fails on the package the pilot inspected if the pilot found something missing, with `expected.toml`
naming the missing items. BD-037 wrote the row with `pin_state = "unpinned"` and `observed_at = "pending:
inspection pilot"`, the one row exempt from "fails at its recorded commit" until the pilot has run. Step 0 ends
that exemption: `observed_at` becomes the commit that exported the inspected package (with the package version
in `PILOT.md`), and the fixture must fail there with the recorded signature (in the not-needed case it passes
there, and the `not-needed` status is the row's exemption). `pin_state` stays `unpinned`, since
this sibling is never a Cargo dependency, until NFp2-03 enters a package version into the import contract, which
is this sibling's first admission; NFp2-03 clears it.

**Switch-over (the compatibility train).** When the change is on the sibling's `main`: (1) move the sibling's pin
in `franken-stack.lock` to a clean commit that contains it (a pin only moves forward; a sibling's newer clean head
is never rolled back locally); (2) run `cargo xtask check-stack`, which writes a new resolution receipt
(`frob.stack-receipt/1`) under `receipts/`; (3) re-run that sibling's admission gates, the parity gates and the
gates this item names; (4) relock (`Cargo.lock` regenerated and committed, builds `--locked` on the one toolchain
T) and append the record, with the receipt hash, to `DEPENDENCY_ADMISSION.md`. For this sibling, a pattern and
not a Cargo dependency, the "pin" is the package version: this bead records the version exported from the landed
commit and the commit itself, runs the validator on it, and leaves the row `landed`. No PASS is recorded on a dirty
sibling tree, and neither a pin nor T moves while a ten-fold tournament is running. The gate this item names needs
the `place` form of `scene.import`, so its run against the new version, the contract row and the move to `adopted`
are the switch-over bead NFp2-03 (the plan's rule: if a gate named here does not exist yet when the change lands,
the pin still moves with the gates that do exist, the row stays `landed`, and the gate's first run is made against
the new pin).

**What the switch-over re-runs (NFp2-03).** GATE-HAB-03 **with a scale label of `witnessed` or better** (Reality: one real
place ingested end to end, privately, with its scale label and unknown regions respected, and at least one
probe request emitted for it; a place for which no capture exists is recorded as `no-claim`, not as a pass).
The twin package's version becomes part of the import contract (OPEN-HAB-5).

**Fallback and kill.** KILL-INGEST: trigger: GATE-HAB-03 cannot be passed with a scale label of `witnessed` or
better after one repair cycle. Fallback: the place is authored in the interior grammar from a tape-measure
survey, with every surveyed dimension recorded as `measured`, and the pack says it is a proxy.

## Acceptance criteria

- [ ] The pilot's finding is recorded in `tests/upstream/FD-FSS-01/PILOT.md` with the package version and the three verdicts.
- [ ] If nothing was missing: the registry row reads `not-needed` (or `open` with the verdict in `PILOT.md` while BD-037's schema lacks that value), and no change was made in the sibling.
- [ ] If something was missing, in the sibling: the twin package carries the missing items as new fields; its exporter refuses four coplanar landmarks; its own tests pass.
- [ ] Here: the validator passes on a package from the landed commit (four named landmarks spanning a tetrahedron above the threshold, a witness with method and uncertainty, a coverage partition), and fails with the recorded signature on the package the pilot inspected; the package version and the exporting commit are recorded and the row reads `landed` (NFp2-03 runs the importer checks and GATE-HAB-03 and moves it to `adopted`).
- [ ] The mirrored bead exists (if the change was needed), files were reserved and released, and nothing in this repository patches the sibling.

## Test plan

### Unit tests
In the sibling (added, if the change is made):
- `coplanar_landmarks_are_refused`: landmarks (0, 0, 0), (1, 0, 0), (0, 1, 0), (1, 1, 0); the spanned volume is 0; expected the exporter's refusal.
- `tetrahedral_landmarks_are_accepted`: (0, 0, 0), (1, 0, 0), (0, 1, 0), (0, 0, 1); spanned volume 1/6 m³; accepted.
- `scale_witness_has_method_and_uncertainty`: a package without either is invalid.
- `coverage_partitions_the_twin`: every face of the exported geometry belongs to exactly one region.
Here (`tests/upstream/FD-FSS-01/`, compiled as the `frob-conformance` integration test `upstream_fd_fss_01`,
`#[ignore]` so that the default suite stays green; packages labelled `synthetic`):
- `validator_three_things`: a complete skeleton passes; one without landmarks, one without a witness, one without coverage each fail naming the missing item.
- `validator_coplanar_fails`: landmarks (0, 0, 0), (1, 0, 0), (0, 1, 0), (1, 1, 0) fail the tetrahedron check (volume 0); (0, 0, 0), (1, 0, 0), (0, 1, 0), (0, 0, 1) pass (V = 1/6 m³, d = √2 m, V/d³ = 0.0589 against the 1% threshold); (0, 0, 0), (1, 0, 0), (0, 1, 0), (0, 0, 0.05) fail (V = 1/120 m³, d = √2 m, V/d³ = 0.0029).
- `validator_signature_matches_pilot`: on a skeleton reproducing the pilot's findings, the failure equals `expected.toml`.
- `pilot_record_read`: `PILOT.md` cites a pilot record that loads under `frob.gauntlet.pilot/1`; a citation to a missing record is an error.
(Registration from the package's landmarks, the scale label and unobserved regions as unknown space run through
`scene.import` and are NFp2-03's tests.)

### End-to-end script and logging
`scripts/e2e/upstream_fd_fss_01.sh --phase pre|post` (sources the STD-E2E helper; header `bead=FD-FSS-01
milestone=M5 plants=three-landmarks,witness-without-method`). Phase `pre`: prints the pilot record and runs the
validator on the inspected package (`cargo test -p frob-conformance --test upstream_fd_fss_01 -- --ignored`),
asserting the recorded verdicts; or, in the not-needed case, asserts the row's `not-needed` state and the verdict in `PILOT.md`. Phase `post`, after the
package version is fixed (this sibling is a pattern and not a Cargo dependency): the validator on a package exported
from the landed commit, expecting a pass; the version and exporting commit read back from the row, which must read
`landed`; nothing of the sibling in `Cargo.lock` or the tree. The imports through `scene.import`, the probe request
on an unresolved door and GATE-HAB-03 are NFp2-03's script. The log records identities and labels only, never the
place's content.

Logging per STD-E2E: the script sources the shared helper and writes NDJSON to
`target/e2e-logs/upstream_fd_fss_01/<utc-timestamp>.ndjson` with a human-readable transcript beside it. Every line has `ts`,
`script`, `bead` (`FD-FSS-01`), `step`, `event` (`start`, `cmd`, `assert`, `metric`, `refusal`, `end`), the exact
command line, exit code, wall time and the envelope's `evidence` block (claim, class, tier, determinism,
identities, seed). It also records the sibling's pinned commit, the stack-receipt hash, the phase, each measured
value with its unit and threshold, digests where state is compared and, on failure, the first differing value
and the artifact paths needed to reproduce. A final `summary` line gives assertions passed and failed and the
gates touched. No private-scene data is logged.

### Known answers and negative controls
- Known answers: four points are coplanar exactly when the tetrahedron they span has zero volume (1/6 m³ for the unit corner; 0 for the square); a similarity transform has seven parameters, and four non-coplanar correspondences determine it with redundancy and exclude a reflection.
- Negative controls: a validator that accepts four coplanar landmarks, or a witness without method and uncertainty, must be caught by the planted skeletons. (Registration from three landmarks and an importer that ignores the coverage partition, CX-355, are NFp2-03's controls.)
- Determinism lanes: validation is deterministic; the same package validated twice gives identical verdicts and signature.

## Considerations

- The item is unusual in the register: it may never be opened. The pilot is cheap by design (inspection of one package) and comes first so that the importer is not built on an assumption.
- Real-place data is private: packs are stored outside the repository and everything derived inherits the mark; no log line carries scene content.
- This sibling is a pattern, not a build dependency: "pin" here means the package version named in the import contract, and the switch-over's receipt records that version.
- No milestone waits on this bead. Its milestone label says when its switch-over gate first exists (NFp2-03), not
  that the milestone's exit depends on it; the work itself may start as soon as the pilot N19c-01 (M3) records
  `falsifier-holds`. Until the switch-over has been run, the interim posture above is the design of
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

Appendix F.1 (row FD-FSS-01), F.3; §19.13 (sources; what import adds; the pilot; KILL-INGEST), §19.5 (four-point registration), §33.4 (FD-FSS), §37, §42 (OPEN-HAB-5); GATE-HAB-03; CX-355.
