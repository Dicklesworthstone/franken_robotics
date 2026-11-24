---
key: "FD-FSIM-13"
title: "upstream frankensim: sustained contact beyond 21 contacts (larger solve, islands, or a qualified patch model)"
type: "task"
priority: 1
milestone: "M2"
labels: ["m2", "suite", "kinesis", "traction", "upstream", "frankensim", "solid", "review-notes-open"]
parent: "WP-39"
deps: ["BD-080", "FDEF-04", "N24b-03", "N33b-04", "STD-E2E"]
needs: []
related: ["N16a-05", "N33a-14", "N33b-07", "N33b-08", "NFp3-12"]
gates: ["INT-FSIM-04"]
plan_refs: ["Appendix F.1", "Appendix F.2 row 4", "Appendix F.3", "§16.4", "§18.11", "§33.3", "§33.8"]
---
## Background

FrankenSim's sustained contact solver admits **at most 21 contacts** in one solve (FACT,
`frankensim/crates/fs-contact/src/sustained/solve.rs:26`: `pub const MAX_CONTACTS: usize = 21`, also the default
budget of the frictional multi-contact solve). The referee tier of this project is that solver. A single robot
is one island, because every foot couples through the floating body. A sole with eight samples per foot fits
when standing (16 contacts) and overflows in a fall or a crawl; a tracked robot with patch samples along two
belts overflows in normal driving. The limit report is FDEF-04. Upstream register priority: **P1**.

This project never truncates a contact set: an over-capacity set is a refusal by name (`FROB-REF-721`). Three
responses to an overflow are allowed, and removing contacts is not one of them: a documented patch reduction
with its qualification fixture; a larger admitted solve; a named refusal. The reason is capacity bias: a
comparison between a tracked design and a point-foot design is honest only if both arms were simulated, and
dropping contacts would bias exactly the comparison the product exists to make.

The upstream item asks FrankenSim for any one of three things. Rejected here: a local higher-capacity copy of
the solve (a fork of the referee); silently clamping at 21.

## Scope

In: in FrankenSim, one of: (i) a higher or parameterized capacity; (ii) an island-wise solve; (iii) a
documented patch model with a qualification fixture. Here: the capacity read as data, the pin move and the
referee-side re-runs of the switch-over (INT-FSIM-04's refusal row at the new limit; BD-080's 22-contact fixture
`traction/referee_capacity_22` on the referee, and BD-080's test `referee_capacity_22_refuses`, which asserts the
refusal at 22 with `required = 21` and is re-pointed in the pin-move change at a set of capacity + 1 spheres with
`required` equal to the capacity the airlock reads, once a form that raises the capacity of one island has landed). Out: the M2 side of the switch-over, that is the capacity audit through `contact.capacity` and
`contact.audit`, the capacity-bias fixture of GATE-TRAC-06 and the re-issue of the referee's coverage table
GATE-FID-03 (NFp3-12, which depends on this bead and on the M2 beads that own those checks, N18b-24 and BD-213, so
that this upstream work can start before they exist; the split made for FD-FSIM-02 in NFp2-01); the capacity
record and its two checks in the airlock (`RefereeContact`, N33a-14) and the referee step's
capacity refusal (BD-080, a dependency: the 22-contact fixture runs through it); the capacity audit and the
capacity-bias label on this side (BD-168, N18b-22);
witness migration (FD-FSIM-12).

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

**The change, three admissible forms (DESIGN; FrankenSim chooses, and may land more than one).**

1. *A larger solve.* The capacity becomes a parameter of the solve with the present value as its default. Cost,
   stated: the solve is dense, so with three rows per contact its factorization grows with the cube of the
   contact count; 64 contacts are 192 rows against 63, about 28 times the work per factorization. The crate's
   contract states the tested maximum.
2. *An island-wise solve.* Contacts are partitioned into islands that share no dynamic body, and each island is
   solved alone under the present cap. Honest limit: one robot is one island, so this helps scenes with several
   robots or loose objects and does not help one many-contact robot.
3. *A documented patch model.* The samples of a small patch are replaced by one four-dimensional contact
   (normal, two tangential, spin) with the patch's torsional capacity, provided the remaining contacts still
   carry the tilting moments the scenario needs. For a uniformly loaded disc of radius a under normal force N
   with friction coefficient μ, the torsional capacity is M_max = (2/3)·μ·N·a. The model ships with a
   **qualification fixture**: a scene solved both with the full sample set (at or under the cap) and with the
   patch, and the stated bound on their difference.

Additive in every form: the default capacity and the default solve are unchanged.

**Interim posture here.** Over-capacity sets refuse by name; patch reductions are declared. Capacity is exposed
per solver and revision as a field of the airlock trait `RefereeContact`, checked at model compile (from the
static maximum of simultaneously active samples) and again at run time. When the arms of a comparison differ by
more than one percentage point of worlds in capacity-refused episodes, the verdict is labelled
`capacity-biased`, the envelope prints the overflow of each arm, and the result cannot be established
(`FROB-REF-822` if cited).

**Fixture (one reproducible failing fixture, DESIGN).** `tests/upstream/FD-FSIM-13/`, opened by BD-037 and kept current
by this bead: one test tagged `FD-FSIM-13`, compiled as the `frob-conformance` integration test `upstream_fd_fsim_13` and
marked `#[ignore]` so that the default suite stays green, with `expected.toml` beside it holding the signature
recorded at the pinned commit (`observed_at`, the failing assertion and the observed value). It fails at the pin
and passes once the change is adopted; a fixture that passes at the pin no longer demonstrates the limit and is
itself a failure of the registry check. It sends 22 point contacts under one rigid body to the sustained solve
through the airlock and expects a solution; at the pin it receives the typed capacity refusal.

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

**What the switch-over re-runs.** The rule is "re-run the capacity fixtures of Traction". In this bead, the ones
that need only the referee: INT-FSIM-04's refusal row at the new limit (a set one above the new capacity
refuses; a set at it solves), and the 22-contact fixture run on the referee through `frob sim run`. If form
(iii) is what landed, the patch model's qualification fixture is re-run through the airlock and its bound is
recorded. Their record, with the receipt hash and the capacity read at the new pin, is what NFp3-12 starts from,
and this bead leaves the row `landed`. In NFp3-12, the ones that need the M2 capability beads: the capacity audit;
the capacity-bias fixture (a tracked design against a point-foot design under an artificially low cap must return
`capacity-biased` and no winner), which is part of GATE-TRAC-06; and the referee's coverage per robot class
(GATE-FID-03), re-issued with the refusal cause `contact-cap` expected to fall for tracked and many-sample robots;
under form (iii), the bound entered with the patch reduction's row so that it is printed with every result that
used the patch. Only then does this item's row become `adopted`.

**Fallback if the change cannot land.** Named refusals and declared patch reductions on this side; the `strict`
profile adjudicates where the referee refuses; referee coverage for tracked robots stays low and is reported.

## Acceptance criteria

- [ ] In FrankenSim: at least one of the three forms exists with its tests; the default behaviour at 21 contacts and the default refusal at 22 are unchanged unless a caller opts in.
- [ ] In FrankenSim: for form (i), a solve at the new tested maximum agrees with the same scene solved island by island or by a reference method to its stated tolerance; for form (iii), the qualification fixture passes with its bound.
- [ ] Here: the airlock's capacity field equals the value read from FrankenSim at the new pin, with no constant 21 anywhere above the airlock.
- [ ] Here: the 22-contact fixture solves (or, under form (iii), solves through a declared patch), and a set one above the new capacity still returns `FROB-REF-721` carrying the upstream reason.
- [ ] Here: the switch-over's referee-side record (INT-FSIM-04 at the new limit, the 22-contact run, the capacity value read at the new pin, the form that landed and, under form (iii), the qualification bound, with the receipt hash) is on file for NFp3-12, which re-runs the capacity audit and the capacity-bias fixture of GATE-TRAC-06 and re-issues GATE-FID-03's coverage table; nothing in this bead waits on those M2 checks.
- [ ] The item's row in `registries/upstream.toml` names its fixture, this bead, the switch-over bead NFp3-12 and the mirrored bead in the sibling's tracker; its `status` is `landed` once the change is on the sibling's `main` and the pin has moved (NFp3-12 moves it to `adopted`).
- [ ] Every file edited in the sibling was reserved through Agent Mail before the edit and released after landing; the change is on the sibling's `main` and passed that repository's own gates under its `AGENTS.md`.
- [ ] Nothing in this repository patches, forks or vendors the sibling; `cargo xtask check-stack` reports it on-pin and clean under the new receipt, and the record in `DEPENDENCY_ADMISSION.md` cites that receipt.

## Test plan

### Unit tests
In FrankenSim (added):
- `capacity_parameter_default_is_21`: default-constructed solves refuse at 22 as before.
- `solve_64_contacts_matches_reference`: a rigid plate on 64 coplanar points under its weight; expected the sum of normal forces equal to m·g to 10⁻¹⁰ relative and zero net moment about the centre of mass.
- `islands_are_independent`: two bodies with 15 contacts each and no shared dynamic body; expected each island's solution equal to its solution alone, bit for bit.
- `disc_patch_torsional_capacity`: μ = 0.5, N = 100 N, a = 0.05 m; expected M_max = (2/3)·0.5·100·0.05 = 1.6667 N·m; a spin torque of 1.6 N·m holds and 1.7 N·m slips.
- `patch_qualification_fixture`: a foot with eight samples against the same foot as one patch contact plus the contacts that carry tilt; expected normal force and tilting moments within the stated bound.
Here:
- `airlock_capacity_is_read_not_assumed`: a scan finds no literal 21 in capacity logic above the airlock; the field equals the sibling's constant.
- `over_capacity_refuses_at_new_limit`: capacity + 1 contacts; expected `FROB-REF-721`.
- `doctor_reports_the_capacity_read`: the `referee` section of `frob doctor --section referee --json` (N33a-14) prints the capacity the airlock read at the new pin, equal to the value the 721 refusal carries.
(That the capacity-bias fixture still returns `capacity-biased` and no winner under a cap of 8 is re-run by NFp3-12, with N18b-22's script.)

### End-to-end script and logging
`scripts/e2e/upstream_fd_fsim_13.sh --phase pre|post` (plain shell, seed-explicit, idempotent, non-zero on failure).
Phase `pre`, valid at any pin before adoption: `cargo xtask check-stack`; `frob doctor --json` (the registry
section shows this item with its status); `cargo test -p frob-conformance --test upstream_fd_fsim_13 -- --ignored`,
asserting that the fixture fails with the signature in `expected.toml`; then the interim posture's own tests,
asserting that they pass. Phase `post`, after the pin move: the same fixture must pass, INT-FSIM-04's refusal row
is re-run at the new limit, and the registry row must read `landed` with a receipt newer than the pin move
(`adopted` once NFp3-12 has run: GATE-TRAC-06 and GATE-FID-03 are read there, not here). Phase `post` also runs
`frob doctor --section referee --json` (the capacity the airlock read, N33a-14), `frob sim run --fixture
traction/referee_capacity_22 --tier referee --seed 5 --key fd13-22-post --json` (the 22-contact fixture, a
`synthetic` body resting on a plane through 22 sample spheres, registered by BD-080 in the fixture registry of
N24b-03 and stepped by BD-080's referee step: a committed run, where
phase `pre` under the key `fd13-22-pre` asserts `FROB-REF-721`, exit 5, with the contact count, the capacity and
the upstream reason; where only form (ii) or form (iii) landed the capacity of one island is still 21, the `post`
run still refuses and the script logs that as the expected state, the run through a registered patch reduction
being NFp3-12's `patch-model` step). The refusal one above the new capacity is asserted by INT-FSIM-04's row and by
`over_capacity_refuses_at_new_limit`, in the conformance crate: a fixture's contact count is not an input of
`sim.run`. The capacity audit (`frob contact capacity`, `frob contact audit`), the
capacity-bias fixture and the coverage table (`frob fidelity coverage`) are in NFp3-12's script.

Logging per STD-E2E: the script sources the shared helper and writes NDJSON to
`target/e2e-logs/upstream_fd_fsim_13/<utc-timestamp>.ndjson` with a human-readable transcript beside it. Every line has `ts`,
`script`, `bead` (`FD-FSIM-13`), `step`, `event` (`start`, `cmd`, `assert`, `metric`, `refusal`, `end`), the exact
command line, exit code, wall time and the envelope's `evidence` block (claim, class, tier, determinism,
identities, seed). It also records the sibling's pinned commit, the stack-receipt hash, the phase, each measured
value with its unit and threshold, digests where state is compared and, on failure, the first differing value
and the artifact paths needed to reproduce. A final `summary` line gives assertions passed and failed and the
gates touched. No private-scene data is logged. Refusals are logged with the code, the contact count and the capacity.

### Known answers and negative controls
- Known answers: the sum of normal forces equals the weight; M_max = (2/3)·μ·N·a = 1.6667 N·m; 16 standing contacts fit under 21, 22 do not.
- Negative controls: an adapter that keeps the first `capacity` contacts and drops the rest must be caught by the capacity fixture; a patch reduction applied to a foot whose remaining contacts cannot carry the tilting moment must be refused by the patch rule; a comparison whose arms differ by 2 percentage points of capacity-refused worlds must not return a winner.
- Determinism lanes: twice on one machine; across worker counts; debug and release; same-platform class.

## Considerations

- RISK-31 and BET-34: coverage below 90% in a reference class at M2 exit falsifies the bet for that class; this item and FD-FSIM-12 are the bet's named fallbacks, with the `strict` profile as the highest tier until they land.
- A larger dense solve makes each referee step slower by the cube law above; the referee is called in rollouts and for adjudication, never inside a training loop, so the cost is paid where an answer is otherwise a refusal.
- The lockstep tiers have their own capacities, raised by a new step-plan identity on this side; nothing here changes them.
- If form (ii) is the only one that lands, the fixture still refuses, truthfully: 22 contacts under one body are one island. The row then stays `landed` with the form named in its note, the interim posture stays for a single many-contact robot, and what the pin move gains (scenes with several robots or loose objects) is shown by the two-island case through the airlock. The row becomes `adopted` only when form (i) or form (iii) makes the fixture pass.
- No milestone waits on this bead. Its milestone label says when its switch-over gate first exists (NFp3-12), not that the
  milestone's exit depends on it; the work itself may start as soon as the referee step BD-080 is done. Until the switch-over has been run, the interim posture above is the design of
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

Appendix F.1 (row FD-FSIM-13), F.2 (row 4), F.3; §16.4 (limits table), §18.11 (capacity and capacity bias), §33.3 (capacities as data), §33.8; INT-FSIM-04; GATE-TRAC-06, GATE-FID-03; FROB-REF-721, -722, -822; RISK-31; BET-34.
