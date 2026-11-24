---
key: "NAB-49"
title: "Owner decision: the sixteen ideas listed as not yet placed (seven since placed; nine to park or decline)"
type: "task"
priority: 1
milestone: "M0"
labels: ["m0", "delivery", "gauntlet", "solid", "registry", "decision-record", "owner-decision"]
parent: "EPIC-M0"
deps: ["NAB-32", "NAB-42", "NAB-48"]
needs: []
related: []
gates: []
plan_refs: ["Appendix B.8", "Appendix B.6.3", "§45.12", "§0.7"]
---
## Background

The idea passes behind FrankenRobotics produced 112 distinct ideas. The rule of the record is that every idea
has one of three dispositions: placed at a tier with a gate (and, for Frontier and Moonshot, a kill gate with
its fallback); refuted or declined, with what survives; or **not yet placed**. The last group was listed
explicitly, sixteen ideas, "so that each can be placed, parked in the killed-idea register with a reason, or
declined. None was refuted." Eleven came from proposal passes; five were named only after the last
cross-examination and were never examined by a second pass.

Two facts make this a work item and not just a table. First, the list is out of date: after it was written,
later-added subsections of the specification **placed seven of the sixteen** (closed-form elimination of
single-loop planar closures; far-field field reuse across forks; controller-budget curves; task audit by foil
bodies; the item bank; placement and sizing along a prescribed motion; visibility under self-occlusion). Each
of those has beads in the units that own it. Second, the remaining **nine have no home**: no bead carries
them, and nothing records whether they are wanted. An idea in that state is either quietly lost or quietly
built by whoever rediscovers it, with no pilot and no kill gate. This bead gives each of the nine a default
disposition, a trigger where one exists, and asks the owner to confirm.

Fail-closed default until the owner decides: **none of the nine is built**, and nothing in the system depends
on any of them. Priority is below the milestone's default because no other work waits on this decision. Tier:
Solid (a decision record).

## Scope

In: records `UP-01` to `UP-16` of the idea-pass register (`registries/ideas.toml`, NAB-42); the standing of
each at drafting time; default dispositions; four parked records (`KP-01` to `KP-04`) and one declined record
(`KD-04`) in the killed-idea register (`registries/killed_ideas.toml`, NAB-32); the two profile triggers; the
owner's decision record. Out: building any of the ideas; the seven placed mechanisms (their own beads).

## Technical approach

**1. Transcription rule.** One `UP` record per row, with `proposals` from the "Passes" column (`0` and
`proposals_note = "review"` for the five named after the last cross-examination), `mechanism`,
`why_not_placed`, `suggested_home` (verbatim), `standing` and `default_disposition` (the last two columns),
`carriers` (for the seven placed), `killed_idea` (for the parked and declined), `trigger` (for the
conditional), `owner_decision` (empty until decided) and `disposition`: `placed`, `parked`, `declined`,
`conditional` or `owner-to-decide`. `counts_toward_tally` is true on UP-01 to UP-11 and false on the five review
ideas (the join of NAB-42 §3: UP-03 and UP-04 count as placed, the other nine as not placed).

**2. The sixteen.**

| Idea | Passes | Mechanism in one line | Why it is not placed | Suggested home | Standing at drafting time | Default disposition (DESIGN, for the owner to confirm) |
|---|---|---|---|---|---|---|
| Inverted index of contact-compatible modules | 1 | Postings from required screw, wrench, geometry and actuator envelopes to module records, so a catalog is not scanned per edit | Catalogs hold hundreds of entries and a linear scan takes microseconds | Decline, or a note under `module.list` (§14.6) | Residual: no bead carries it. Nearest work: module capability tables (N14b-28) and `module.list` (BD-060) | Decline. Record KD-04. Revive only if a measured `module.list` or per-edit catalog scan exceeds its latency share in a profile |
| Proof-driven time stepping in quiet windows | 1 | Per-world error budgets admit a coarser step class where separation and truncation error are certified | Dynamic shapes conflict with the static step plan; §21.20 rejects per-world time tiles for a related reason | Park in the killed-idea register; if revived, an effort profile under DC-08 on the `strict` profile only | Residual. Mentioned only among the rejected step shapes of the device plan | Park. Record KP-01. Revive only as an effort profile under DC-08 on the `strict` profile, with a certified separation and truncation bound per window and a static shape |
| Symbolic elimination of kinematic loop closures | 1 | Eliminate four-bar and parallel-knee closures per limb into exact reduced-coordinate kernels | §16.12 keeps closures as compliant rows on lockstep tiers and bilateral rows on transactional profiles; exact closure waits on an upstream work item (FD-FSIM-04) | §16.12 as an [F] option for templates with closed-form closures; §25.21.3 holds the algebra | Placed by a later subsection for single-loop planar closures: N16c-29 (pilot), N16c-30 to N16c-34 (GATE-KIN-11, KILL-LOOPELIM). Exact spatial closure still waits on FD-FSIM-04 | Placed. No decision owed here; the idea record points at its carriers |
| Far-field recomputation of only what an intervention touches | 2 | Content-addressed spacetime tiles for slow far-field state, so a fork recomputes only descendants of its dirty frontier | Contested (the contested-ideas record (NAB-47)); the robot's neighbourhood is always dirty | Park in the killed-idea register; a candidate only for multi-room film or steam fields in §19.10 | Placed by a later subsection as far-field field reuse across forks: N19c-33 (pilot), N19c-34 to N19c-37 (GATE-HAB-15, KILL-FARFIELD). The robot-neighbourhood form stays not built (KI-16) | Placed, at M6, for multi-room film and steam fields only |
| Correlated contact-material grids | 1 | Condition-indexed friction cards with correlated uncertainty factors and sparse regional overrides | Interface cards and static condition cards exist; correlation across regions is not modelled | §18.6 or §19.5, with the randomization of §23.15 as consumer | Residual. Interface cards by material pair and condition exist (BD-167); randomization from cards exists (BD-200); correlation across regions is not modelled | Owner decision. Default: not built. If placed: correlated uncertainty factors and sparse regional overrides on interface cards, with card randomization as the consumer, and a pilot that shows a ranking change against independent marginals |
| Coherent sensor ray bundles | 3, each cut by its proposer | Share conservative candidate lists across nearby rays, with a per-ray fallback near thin geometry | It speeds one stage, and coherence vanishes in clutter | §20.6, only if the ray stage exceeds its share in the M2 profile | Residual. Ray kernels exist (BD-187); each of the three proposals was cut by its own proposer | Conditional. Default: not built. Trigger: the ray stage exceeds its share in the M2 profile (read at the M2 sweep, NAB-25); then a pilot with a per-ray fallback near thin geometry |
| Continuous interpolation between topologies | 1 | Blend discrete topologies through continuous coupling weights so a gradient crosses a topology change | Ill-conditioned as a weight tends to zero, where a limb becomes massless | Park in the killed-idea register | Residual. Discrete families are searched as terms with typed holes (N25b-13) | Park. Record KP-02. Ill-conditioned as a coupling weight tends to zero, where a limb becomes massless |
| Saddle-point robust co-design | 1 | Differentiate design parameters at the worst-case friction and payload | Subsumed by adversarial search with design gradients; depends on IP-16 | Park in the killed-idea register until NONGOAL-15 is revisited | Residual. Depends on derivatives through the contact solve (IP-16) | Park. Record KP-03, until the non-goal on gradients through contact is revisited at M6 (BD-422) |
| Missing-contact diagnosis | 1 | Infer feasible reaction envelopes from momentum residuals over a dictionary of candidate contact surfaces | Ambiguity grows quickly with the number of plausible surfaces | An `explain` view in §28.5, or Ground §30 for real logs | Residual. `explain.run` (BD-112) and Ground admission (BD-387) are the candidate homes | Owner decision. Default: not built. If placed: an `explain` view that reports feasible reaction envelopes from momentum residuals, labelled `Estimated`, with the ambiguity count printed |
| Scene-side incremental geometry rebuilds | 1 | A dependency map from typed scene edits to bricks, so an edit rebuilds only the geometry it touches | Scenes are immutable per `SceneId`; a retrofit is a new scene variant over a shared brick pool | §19.4, measured against the pooled variant path of §19.15 | Residual. Scenes are immutable per `SceneId`; variants share the brick pool (BD-163, BD-164) | Conditional. Default: not built. Trigger: the pooled-variant path misses its rebuild target at GATE-HAB-13; then measure a dependency map from typed scene edits to bricks against it |
| Micro-slip precursors for tactile sensing | 1 | Couple vibration modes with tangential micro-slip so a policy senses slip before it happens | Cannot be validated without tactile-array data | With tactile micro-patches (IP-69), behind the import lane of §30 | Residual. Tactile micro-patches are a Moonshot pilot (BD-418, N18b-35) | Park behind the tactile pilot and the import lane. Record KP-04. Cannot be validated without tactile-array data |
| Ranking designs by achievable performance | review | Controller-budget sweeps with a fitted saturation curve | Not examined by a second pass | §28.9 as a rung rule, setting `controller-sensitive`; pilot: 20 variants at three budgets on a CPU | Placed by a later subsection: controller-budget curves. N28c-32 is the pilot the row names (20 variants at three budgets on a CPU); N28c-35, N28c-37, N28c-41 | Placed. The `controller-limited` flag now has a procedure that sets it |
| Task-specification audit by ghost bodies | review | Run degenerate or deliberately capable bodies against the task predicate | Not examined by a second pass | §24.5 beside `task.check`; compare with the detectors of §23.16 | Placed by a later subsection: `task.audit`. N24c-20 to N24c-28 | Placed |
| Calibrated item bank for batteries | review | A response model over battery items, sampled by importance | Not examined by a second pass | §26.11 and DC-10 | Placed by a later subsection: the item bank. N24c-29 to N24c-35 | Placed |
| Convex synthesis from inverse dynamics | review | One cone program over inertial parameters, gearing and actuator sizing along nominal trajectories | Not examined by a second pass | §25.21, beside mass placement | Placed by a later subsection: placement and sizing along a prescribed motion. N25e-18 to N25e-22 | Placed |
| Self-occlusion in sensor coverage | review | Include occlusion by the robot's own links in coverage and observability analyses | Not stated in A-14 or §20.11 | A-14 (§28.4) and §20.11 | Placed by a later subsection: visibility under self-occlusion, and sensor mounts as design variables. N20b-32 to N20b-34, N20a-35 (the pilot), N20b-36 to N20b-38 | Placed |

**3. The nine residual ideas, by default disposition.**

*Parked* (entered in the killed-idea register with `state = "parked"`; a parked idea may be reopened by the
evidence named):

| Record | Idea | Reason it is parked | Reopened by |
|---|---|---|---|
| KP-01 | Proof-driven time stepping in quiet windows (per-world error budgets admit a coarser step class where separation and truncation error are certified) | Dynamic shapes conflict with the static step plan; per-world time tiles are rejected for a related reason (KI-09) | A static-shape formulation as an effort profile under the solver-effort decision core, on the `strict` profile only, with a certified bound per window |
| KP-02 | Continuous interpolation between topologies (blend discrete topologies through continuous coupling weights so a gradient crosses a topology change) | Ill-conditioned as a weight tends to zero, where a limb becomes massless | A formulation whose mass matrix stays positive definite as a weight vanishes, with a known answer |
| KP-03 | Saddle-point robust co-design (differentiate design parameters at the worst-case friction and payload) | Subsumed by adversarial search with design gradients; depends on derivatives through the contact solve, a non-goal before M6 | The M6 review of gradients through contact (the M6 sweep records the outcome) |
| KP-04 | Micro-slip precursors for tactile sensing (couple vibration modes with tangential micro-slip so a policy senses slip before it happens) | Cannot be validated without tactile-array data | The tactile micro-patch pilot passing, and tactile data admitted through the import lane |

*Declined:*

| Record | Idea | Reason | Reopened by |
|---|---|---|---|
| KD-04 | Inverted index of contact-compatible modules (postings from required screw, wrench, geometry and actuator envelopes to module records) | Catalogs hold hundreds of entries and a linear scan takes microseconds | A measured `module.list` or per-edit catalog scan above its latency share in a profile |

*Conditional on a measurement* (`disposition = "conditional"`; not built unless the trigger is met, and then
only as a pilot with a falsifier):

| Record | Idea | Trigger, and where it is read | If the trigger is met |
|---|---|---|---|
| UP-06 | Coherent sensor ray bundles (share conservative candidate lists across nearby rays, with a per-ray fallback near thin geometry) | The ray stage exceeds its share of the step in the M2 profile (read at the M2 sweep, NAB-25). Three passes proposed it and each cut it itself: it speeds one stage, and coherence vanishes in clutter | A pilot on cluttered and open scenes; falsifier: no gain in clutter after the per-ray fallback |
| UP-10 | Scene-side incremental geometry rebuilds (a dependency map from typed scene edits to bricks, so an edit rebuilds only the geometry it touches) | The pooled-variant path misses its rebuild target (read with the pool gate). Scenes are immutable per `SceneId`, and a retrofit is a new scene variant over a shared brick pool | A measurement against the pooled-variant path on the retrofit catalogue |

*For the owner to decide* (`disposition = "owner-to-decide"`; default: not built):

| Record | Idea | What placing it would mean | What is missing today |
|---|---|---|---|
| UP-05 | Correlated contact-material grids (condition-indexed friction cards with correlated uncertainty factors and sparse regional overrides) | Interface cards gain a correlation structure across regions and conditions; randomization drawn from cards samples it jointly | Correlation across regions is not modelled; independent marginals may over- or under-state risk. A pilot would compare design rankings under joint and independent draws |
| UP-09 | Missing-contact diagnosis (infer feasible reaction envelopes from momentum residuals over a dictionary of candidate contact surfaces) | An `explain` view for runs, or an analysis of real logs in the calibration lane, labelled `Estimated` | Ambiguity grows quickly with the number of plausible surfaces; the view must print how many surfaces are consistent with the residual |

**4. The decision record.** `docs/adr/ADR-IDEAS-NOT-PLACED.md`: one line per residual idea with the
owner's disposition (`confirm default`, `place` with the home and tier, `park`, `decline`), dated. A `place`
decision creates the idea's pilot bead (with its falsifier and kill gate) in the unit that owns the home; this
bead does not create mechanism beads on a default.

**5. Consistency checks** (in `crates/frob-conformance/tests/idea_register.rs`): the seven placed ideas have
carriers that exist; the nine residual have a default; no idea is both `placed` here and `not-placed` in the
blind-spot records; `KP-01` … `KP-04` and `KD-04` exist; the revive lint matches a bead that builds a residual
idea and requires it to cite its record.

## Acceptance criteria

- [ ] Records `UP-01` … `UP-16` exist: seven `placed` with carriers (UP-03, UP-04, UP-12 to UP-16), four
      `parked` (UP-02, UP-07, UP-08, UP-11), one `declined` (UP-01), two `conditional` (UP-06, UP-10), two
      `owner-to-decide` (UP-05, UP-09).
- [ ] `KP-01` to `KP-04` and `KD-04` exist in the killed-idea register with reason and reopening evidence.
- [ ] The decision record exists with the owner's line for each of the nine, or the bead is closed as
      "defaults stand" with the owner's explicit confirmation recorded (silence is not a decision).
- [ ] UP-06 and UP-10 carry their trigger and `trigger_exit = "M2"` (the ray-stage share of the M2 profile;
      the pool gate GATE-HAB-13, recorded at M2), the field from which NAB-22's due list names them at the M2
      sweep; NAB-50's script asserts that `decision-sweep due --exit M2` lists both.
- [ ] A planted bead "Bundle coherent rays in the ray kernel" with no citation fails the revive lint naming
      UP-06, and passes when it cites the record and the met trigger.
- [ ] The tally check of NAB-42 holds: nine residual ideas, equal to the recorded count of ideas not yet
      placed.

## Test plan

### Unit tests

`crates/frob-conformance/tests/idea_register.rs`, module `not_placed`:

- `sixteen_records`; `dispositions_are_7_4_1_2_2`.
- `placed_have_carriers`: UP-03 → N16c-29 to N16c-34; UP-04 → N19c-33 to N19c-37; UP-12 → N28c-32, N28c-35,
  N28c-37, N28c-41; UP-13 → N24c-20 to N24c-28; UP-14 → N24c-29 to N24c-35; UP-15 → N25e-18 to N25e-22; UP-16 →
  N20b-32 to N20b-34, N20a-35, N20b-36 to N20b-38.
- `residual_count_equals_tally`: 9.
- `parked_and_declined_records_exist`.
- `conditional_has_trigger_and_reader`: UP-06 and UP-10 name a measurement and have `trigger_exit = "M2"`; a
  `conditional` record with an empty `trigger_exit` fails.
- `owner_decision_required_to_close`: with `owner_decision` empty for any of the nine, the close check fails.
- `proposals`: UP-06 has 3; UP-04 has 2; the five review ideas have 0 with the note.

### End-to-end script and logging

`scripts/e2e/decision_records.sh` (NAB-50) prints the sixteen with standing and disposition, asserts the
counts 7, 4, 1, 2, 2, and runs the planted revive case. Logged per STD-E2E.

### Known answers and negative controls

Known answers: 16 = 7 placed + 9 residual; 9 = 4 parked + 1 declined + 2 conditional + 2 for the owner; 11 of
the 16 came from proposal passes and 5 from review; of the 11, two (loop closures; far-field reuse) are now
placed, leaving 9, the tally's count. Negative controls: a residual idea built without a record; a `place`
decision with no pilot bead; the bead closed with an empty owner line.

## Considerations

- The defaults follow the suggested homes of the planning record; they are proposals, labelled DESIGN, and the
  owner may replace any of them.
- Two placed ideas keep an unplaced remainder: exact spatial loop closure still waits on an upstream work item
  in the sibling multibody crate, and recomputing only what an intervention touches in the robot's
  neighbourhood stays not built (KI-16).
- Nothing blocks on this bead. Its cost is one short decision by the owner; its value is that nine ideas stop
  being rediscoverable without a record.
- OPEN (owner): the nine dispositions. Fail-closed default: not built.

## References

Appendix B.8; Appendix B.6.3; §45.12 (the later-added subsections); §0.7 (every Frontier and Moonshot item
needs a pilot with a falsifier before it is funded).
