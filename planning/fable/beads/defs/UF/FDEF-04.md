---
key: "FDEF-04"
title: "limit report, frankensim sustained contact step: fixed witnesses, fixed frame, capacity of 21 contacts"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "suite", "kinesis", "traction", "upstream", "frankensim", "solid", "test"]
parent: "WP-37"
deps: ["BD-035", "BD-037", "STD-E2E"]
needs: []
related: []
gates: ["INT-FSIM-07", "INT-FSIM-04"]
plan_refs: ["Appendix F.2 row 4", "§33.11", "§16.4", "§18.11", "§33.3", "§33.10", "Appendix F.3"]
---
## Background

The referee tier of FrankenRobotics is FrankenSim's articulated dynamics and contact solve, reached through the
airlock crate `frob-fsim`. FrankenSim's sustained contact step, the force–displacement complementarity step
that carries a contact across time, has two stated limits that decide how often the referee can answer at all:

- **Fixed witnesses.** "Body pairs retain their ORIGINAL material witnesses and fixed contact frame";
  tangentially separating witnesses are refused. A wheel rolling over a sharp lip, a sliding sole and a foot
  rocking from heel to toe all move their witnesses.
- **Capacity.** At most 21 contacts in one solve (`pub const MAX_CONTACTS: usize = 21`, also the default budget
  of the frictional multi-contact solve). A single robot is one island, because every foot couples through the
  floating body. A sole with eight samples per foot fits when standing (16) and overflows in a fall or a crawl;
  a tracked robot with patch samples along two belts overflows in normal driving.

Both are documented upstream and enforced by typed refusals, which is the behaviour this project wants from a
referee: it is a **limit report**, not a bug report. It still belongs with the defect reports, because it
decides what M1 can claim and because two upstream work items follow from it (FD-FSIM-12: a moving-surface owner
with witness migration; FD-FSIM-13: a larger or island-wise solve, or a documented patch model). The plan's
rule: a reported limit is a FACT to recheck; the admission tests (INT-FSIM-07 and INT-FSIM-04, BD-035) record
what the pinned commit does, and this report carries that record into FrankenSim's tracker.

Read in source on 2026-11-21 at `483d09c682f2`: confirmed as a documented scope limit with typed refusals
(`frankensim/crates/fs-mbd/src/articulated_contacts/step/sustained.rs`, module header lines 1–10 and lines
333–339; `frankensim/crates/fs-contact/src/sustained/solve.rs:26`).

## Scope

In: the report (`tests/upstream/FD-FSIM-12/REPORT.md`, covering both items and filed as the mirrored beads of
FD-FSIM-12 and FD-FSIM-13); the standalone reproduction of both refusals; the observed refusals copied from the
admission records; the statement of the guard. Out: the admission tests (BD-035); witness migration in the
transactional step (BD-093); the capacity check in the airlock (`RefereeContact`, `ContactCapacities`, N33a-14)
and the refusal conversion (N33a-13); the production referee step on generated contact sets (BD-080, M1); the
upstream changes (FD-FSIM-12, FD-FSIM-13).

## Technical approach

**Smallest reproducing cases.**

1. *Witness transition.* A rigid wheel (one cylinder or sphere primitive, radius r) rolls without slip on a flat
   floor toward a riser of height s < r with a sharp top edge, climbs the edge, and rolls onto the top. Over
   the manoeuvre the contact goes flat → edge → top: the material witness on the ground changes from a point of
   the floor plane to a point on the edge line to a point of the top plane, and the contact normal turns from
   vertical through the edge's fan of normals back to vertical. Expected at the pin: the sustained step returns
   its typed refusal at the first interval in which the witness would have to leave the floor.
2. *Capacity.* A rigid body resting on 22 point contacts (for example two feet with eleven samples each).
   Expected at the pin: a typed capacity refusal; with 21, a solve.

**The reproduction (standalone).** One Rust test file depending only on `fs-mbd` and `fs-contact` at the pinned
commit, suitable for FrankenSim's own test tree. It builds the wheel-and-riser scene and the 22-contact body
from FrankenSim's public constructors, calls the sustained step, and prints the refusal value returned for each
(its variant and message), plus the passing 21-contact control.

**What is asked for upstream (carried by the two work items).**

- FD-FSIM-12 (register priority P1): a moving-surface owner for the sustained step: witness migration across an
  edge, with tangential memory transported or reset under the declared friction law.
- FD-FSIM-13 (register priority P1): a sustained solve above 21 contacts, an island-wise solve, or a documented
  patch model with a qualification fixture.

**Admission tests and pass conditions.** INT-FSIM-07 (the refusal half at M0, BD-035; in full at M1, BD-093):
the flat-to-edge-to-top transition returns the typed refusal through the airlock at the pin, and the same
transition completes under witness migration with the energy ledger closed to its stated residual. INT-FSIM-04
(BD-035): a set of 22 contacts returns a typed refusal; and a thin step lying between two clear endpoint poses
is detected on 1,000 of 1,000 seeded sweeps by the spacetime broad phase.

**The guard this repository keeps meanwhile.**

- *Witness migration above the airlock.* The referee's transactional step migrates witnesses between calls, so
  each call to FrankenSim sees an interval short enough that its witnesses are fixed inside FrankenSim's own
  gate; new witness pairs are localized at a consistent world contact point and tangential memory is
  transported or reset. If FrankenSim's gate still refuses, the refusal is surfaced with the upstream reason
  and never retried with weakened inputs; the `strict` profile of `batch-cpu` adjudicates.
- *Capacity as data.* The limit of 21 is a field of the airlock trait `RefereeContact`, checked when a model is
  compiled (from the static maximum of simultaneously active samples) and again at run time. An over-capacity
  set is a refusal by name (`FROB-REF-721`), never a truncated contact set: dropping contacts would bias exactly
  the comparison between a tracked robot and a point-foot robot. Three responses are allowed: a documented patch
  reduction with its qualification fixture; a larger admitted solve; the named refusal.
- *What M1 may claim.* Referee rollouts over planes and primitive obstacles are admitted. A wheel climbing a
  riser is admitted only through the event-located transactional step.

**Procedure.** (1) Copy the refusal values and the commit from the records of INT-FSIM-07 and INT-FSIM-04 into
the report (`observed_at`). (2) Run the reproduction in a scratch clone at the pin. (3) If the wheel transition
completes at the pin without a refusal, the report says so, the energy ledger of that run is attached, and
FD-FSIM-12 is re-scoped or closed. (4) File the report as the mirrored bead of both items in FrankenSim's
tracker under its `AGENTS.md`; store the identifiers in the two registry rows.

## Acceptance criteria

- [ ] `tests/upstream/FD-FSIM-12/REPORT.md` has the sections Limit, Evidence (files, lines, commit), the two cases, Expected and observed (the refusal variants), What is asked for, Interim posture, and `observed_at`; `tests/upstream/FD-FSIM-13/` links to it.
- [ ] The reproduction compiles against `fs-mbd` and `fs-contact` alone at the pin and prints the refusal for the wheel transition, the refusal for 22 contacts, and a successful solve for 21.
- [ ] The report is filed in FrankenSim's tracker and its identifier is the `mirror` of both registry rows.
- [ ] Through the airlock, the 22-contact set returns `FROB-REF-721` with `observed` 22 and `required` 21 before any sibling call, the wheel transition returns `FROB-REF-721` carrying the upstream reason verbatim, and the capacity read from `RefereeContact` equals the constant read from FrankenSim at the pin (21).
- [ ] No code path above the airlock truncates a contact set: a planted truncation is caught by the capacity fixture.

## Test plan

### Unit tests
- `repro_wheel_edge_transition_refuses`: wheel radius 0.10 m, riser 0.03 m, approach 0.5 m/s; expected a typed refusal at the pin (variant recorded in the report).
- `repro_22_contacts_refuse_21_solve`: expected refusal and success respectively.
- `airlock_capacity_field_matches_upstream`: `capacities().max_contacts_per_solve` of `RefereeContact` (N33a-14) equals `fs_contact`'s `MAX_CONTACTS`; expected 21 at the pin, and the test reads the constant instead of hard-coding it, so a larger upstream capacity changes the field and not the test.
- `airlock_refusal_carries_upstream_reason`: the typed referee refusal for case 1 (the wheel transition) contains the upstream variant and message verbatim and is not retried (one sibling call); the refusal for case 2 is raised by the airlock before any sibling call and carries `observed` 22 and `required` 21.
- `no_truncation_mutant`: an adapter mutant that keeps the first 21 of 22 contacts must fail the capacity fixture (the two arms of a tracked-against-point-foot comparison then differ in refused worlds, and the verdict must be labelled `capacity-biased`).

### End-to-end script and logging
`scripts/e2e/defect_fdef_04.sh`: runs the reproduction in `target/upstream-scratch/frankensim@<rev>`; then runs
the admission lane of BD-035 and reads the two refusal records it writes in the envelope's refusal shape
(`target/admission/int_fsim_04.json`, `target/admission/int_fsim_07_refusal.json`), asserting `FROB-REF-721`
and the exit code 5 the registry maps the code to, with the upstream reason verbatim for the wheel transition
and `observed` 22, `required` 21 for the capacity case (the airlock refuses that one before calling the sibling;
the sibling's own refusal of 22 contacts is what the reproduction shows); compares the refusal
variants with those printed by the reproduction and written in the report; reads `frob doctor --json` (section
`referee`: `max_contacts_per_solve` 21, `fixed_witnesses` true) and the two registry rows. At M0 no `frob` verb
reaches a contact step (`sim.run` at M0 is the contact-free G1 form of BD-024); the verb-level form, `frob sim
run` on the lip fixture exiting 5, belongs to BD-093's script at M1.

Logging per STD-E2E: the script sources the shared helper and writes NDJSON to
`target/e2e-logs/defect_fdef_04/<utc-timestamp>.ndjson` with a human-readable transcript beside it. Every line has `ts`,
`script`, `bead` (`FDEF-04`), `step`, `event` (`start`, `cmd`, `assert`, `metric`, `refusal`, `end`), the exact
command line, exit code, wall time and the envelope's `evidence` block (claim, class, tier, determinism,
identities, seed). It also records the sibling's pinned commit, the stack-receipt hash, the phase, each measured
value with its unit and threshold, digests where state is compared and, on failure, the first differing value
and the artifact paths needed to reproduce. A final `summary` line gives assertions passed and failed and the
gates touched. No private-scene data is logged. Refusals are logged as `refusal` events with the code, the upstream reason
and the interval at which they occurred.

### Known answers and negative controls
- KA-778: the flat-to-edge-to-top transition returns the typed refusal through the airlock at the pin and completes under witness migration with the energy ledger closed to its stated residual; negative control: a set over the capacity of 21 contacts (the case of 22) refuses, while 21 solves (the plan's row reads "over capacity (21 contacts) refuses", meaning over a capacity of 21).
- Negative control: the truncation mutant above; and a scene of a wheel on a flat floor only (no edge) must not refuse, so that the refusal is shown to come from the transition.
- Determinism lanes: the refusal occurs at the same interval twice on one machine and across worker counts (same-platform class; FrankenSim's `fs-mbd` makes no cross-ISA claim).

## Considerations

- RISK-31 (the referee cannot complete the scenes where decisions are made) and BET-34 (referee coverage: at M2 exit the referee completes at least 90% of standard-battery scenarios, or the bet is falsified for that class and the `strict` profile stands as the highest tier there). The share of scenarios the referee completes is a reported figure with a gate (GATE-FID-03), with the breakdown by refusal cause; this report's two causes are two of its rows.
- Capacity bias: when the arms of a comparison differ by more than one percentage point of worlds in capacity-refused episodes, the verdict is labelled `capacity-biased` and cannot be established (`FROB-REF-822` if cited).
- No milestone waits for either upstream change. If neither lands, the referee keeps migrating witnesses above the airlock and refusing over-capacity sets; the cost is referee coverage on tracked and many-contact robots.
- When either lands it is adopted only through its switch-over rule (re-baseline the fidelity ledger once; re-run the capacity fixtures); neither a pin nor the toolchain moves during the ten-fold tournament.

## References

Appendix F.2 (row 4), F.3; §33.11 (row 4, the consequence), §16.4, §18.11, §33.3 (capacities as data), §33.10; INT-FSIM-07, INT-FSIM-04; FD-FSIM-12, FD-FSIM-13; FROB-REF-721, -822; KA-778; RISK-31; BET-34; GATE-FID-03.
