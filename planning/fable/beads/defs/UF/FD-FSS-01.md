---
key: "FD-FSS-01"
title: "upstream franken_surveillance_system (conditional): twin package landmarks, scale witness, coverage"
type: "task"
priority: 3
milestone: "M5"
labels: ["m5", "suite", "habitat", "upstream", "franken_surveillance_system", "frontier"]
parent: "WP-32"
deps: ["BD-037", "NF-01", "STD-E2E"]
needs: ["§19.13: the paper-scale inspection pilot of one existing twin package; this bead is opened for work only if the pilot finds any of the three things missing"]
related: ["BD-398", "BD-228", "BD-305", "FD-FDGR-01"]
gates: ["GATE-HAB-03"]
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
the gate re-run with a better scale label. Out: `scene.import` and `scene.fit` (BD-398); scene acceptance and
four-point registration (BD-228); the drone route (FD-FDGR-01); privacy handling (BD-305).

## Technical approach

**Step 0, here: the pilot's record.** Copy the inspection pilot's finding into
`tests/upstream/FD-FSS-01/PILOT.md`: which twin package and version was inspected, and for each of the three
things, present or missing, with the field or file where it was found. If all three are present, set the
registry row's `status` to a closed state with the note "not needed (pilot)", and stop.

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
   and the exporter refuses to claim so if the tetrahedron they span has volume below a stated threshold.
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
naming the missing items.

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

**What the switch-over re-runs.** GATE-HAB-03 **with a scale label of `witnessed` or better** (Reality: one real
place ingested end to end, privately, with its scale label and unknown regions respected, and at least one
probe request emitted for it; a place for which no capture exists is recorded as `no-claim`, not as a pass).
The twin package's version becomes part of the import contract (OPEN-HAB-5).

**Fallback and kill.** KILL-INGEST: trigger: GATE-HAB-03 cannot be passed with a scale label of `witnessed` or
better after one repair cycle. Fallback: the place is authored in the interior grammar from a tape-measure
survey, with every surveyed dimension recorded as `measured`, and the pack says it is a proxy.

## Acceptance criteria

- [ ] The pilot's finding is recorded in `tests/upstream/FD-FSS-01/PILOT.md` with the package version and the three verdicts.
- [ ] If nothing was missing: the registry row is closed as not needed, and no change was made in the sibling.
- [ ] If something was missing, in the sibling: the twin package carries the missing items as new fields; its exporter refuses four coplanar landmarks; its own tests pass.
- [ ] Here: the validator passes on a package from the landed commit; four-point registration succeeds from the package's landmarks; the scene's scale label is `witnessed` or better; regions the package marks unobserved are unknown space in the scene.
- [ ] Here: GATE-HAB-03 is run with that label on one place, or recorded `no-claim` if no capture is admitted.
- [ ] The mirrored bead exists (if the change was needed), files were reserved and released, and nothing in this repository patches the sibling.

## Test plan

### Unit tests
In the sibling (added, if the change is made):
- `coplanar_landmarks_are_refused`: landmarks (0, 0, 0), (1, 0, 0), (0, 1, 0), (1, 1, 0); the spanned volume is 0; expected the exporter's refusal.
- `tetrahedral_landmarks_are_accepted`: (0, 0, 0), (1, 0, 0), (0, 1, 0), (0, 0, 1); spanned volume 1/6 m³; accepted.
- `scale_witness_has_method_and_uncertainty`: a package without either is invalid.
- `coverage_partitions_the_twin`: every face of the exported geometry belongs to exactly one region.
Here:
- `validator_three_things`: the fixture.
- `registration_from_four_landmarks`: a twin rotated 30° about z, translated by (2, −1, 0.5) m and scaled by 1.02 relative to the scene; the recovered transform reproduces the four landmarks within 1 mm, and its mirror image is rejected (the determinant of the rotation is +1).
- `scale_without_witness_is_not_witnessed`: a package without a witness yields at best the label `estimated` (clearance verdicts are then `unresolved` within the stated scale interval), and with no scale prior `relative-only`, on which every metric task is refused (`FROB-REF-304`).
- `unobserved_region_is_unknown`: as for the drone route.

### End-to-end script and logging
`scripts/e2e/upstream_fd_fss_01.sh --phase pre|post`. Phase `pre`: prints the pilot record and runs the
validator on the inspected package, asserting the recorded verdicts. Phase `post`, after the pin move (or after
the package version is fixed, since this sibling is a pattern and not a Cargo dependency): `frob scene import
--json` on a package from the landed commit, `frob scene accept --json`, and the checks above; the log records
identities and labels only, never the place's content.

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
- Negative controls: a registration from three landmarks must report the mirror ambiguity; an importer that ignores the coverage partition must answer `free` in an unobserved region and be caught (CX-355).
- Determinism lanes: validation and registration are deterministic; registration twice gives identical transforms.

## Considerations

- The item is unusual in the register: it may never be opened. The pilot is cheap by design (inspection of one package) and comes first so that the importer is not built on an assumption.
- Real-place data is private: packs are stored outside the repository and everything derived inherits the mark; no log line carries scene content.
- This sibling is a pattern, not a build dependency: "pin" here means the package version named in the import contract, and the switch-over's receipt records that version.
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

Appendix F.1 (row FD-FSS-01), F.3; §19.13 (sources; what import adds; the pilot; KILL-INGEST), §19.5 (four-point registration), §33.4 (FD-FSS), §37, §42 (OPEN-HAB-5); GATE-HAB-03; CX-355.
