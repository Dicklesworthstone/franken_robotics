---
key: "N41-OPEN1"
title: "registries/open.toml, part 1: open questions of Lemma, Marrow, Skeleton and Sinew (17 rows, one closed)"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "delivery", "lemma", "marrow", "skeleton", "sinew", "registry", "solid"]
parent: "EPIC-M0"
deps: ["N41-03"]
needs: []
related: ["N41-OPEN2", "N41-OPEN8"]
gates: []
plan_refs: ["§42.3", "§42.1", "§12.9", "§13.10", "§14.15", "§15.14"]
---
## Background

The open-question register lists every decision the design has not taken, each with an owner, a fail-closed
default that holds until it is decided, and the gate, measurement or person that decides it (rule and schema in
N41-03). Nothing is decided by silence: at its deadline an undecided question takes its default, recorded as an
ADR. This bead enters the questions of four foundation subsystems:

- **Lemma**: certificates and proofs (the offline auditor `frob-verify`, Lean re-checks through the sibling
  FrankenLean, reflection of small exact checks into the proof kernel);
- **Marrow**: the algebra layer (groups, symmetry, cones, exact predicates);
- **Skeleton**: the design term, its compiler and preflight (`design check`);
- **Sinew**: actuators, batteries and transmissions as cards with provenance.

Reading a row. *Default* is the state in force until the question is decided; it removes a claim or a capability
and never adds one. *Blocks, or decided by* names the gate the question blocks or the measurement that settles it.
*Deadline* is the milestone exit at which the default is taken if the question is still open (derived from the
gate's milestone in the gate register, which governs). *Decided in* names the bead that takes the decision or runs
the experiment; "considerations" means the question blocks nothing and is carried in the named bead's
Considerations.

## Scope

One record per open question, written once, and for these seventeen it is written here.

In: the seventeen rows below as records of `registries/open.toml`, including the closed record and tombstone of
OPEN-SKEL-1.

Out, and which bead has it:

- The schema and loader: N41-03.
- The decisions: the beads in the last column. The owner's two questions here have their own decision beads
  (OPEN-SKEL-2 in N41-13, OPEN-SINEW-1 in N15b-25).
- The Skeleton chapter's side of OPEN-SKEL-1 to -4: N14b-26, the Skeleton chapter's register bead. It prints the
  four questions as the chapter states them and checks that the Skeleton verbs apply the defaults registered
  here (prices `assumed` and `--strict-data` refusing a cost verdict; the task's randomization grid in the
  stage-8 witness; isotropic `lattice` fill). It enters no record in `registries/open.toml` and does not
  tombstone OPEN-SKEL-1; it depends on this bead.

## Technical approach

Each row becomes one `[[open]]` record with the fields of N41-03 (`default_effect` classified once by the reviewer
who enters the row, from the closed set).

| ID | Question | Fail-closed default | Owner | Blocks, or decided by | Deadline | Decided in |
|---|---|---|---|---|---|---|
| OPEN-LEMMA-1 | Memory and time to re-check one theorem's closure with FrankenLean | Measure before any claim | Lemma lead | The admission gate INT-FLN-01 | M5 | N12-23 |
| OPEN-LEMMA-2 | Whether the reflectors' soundness theorems have closures that avoid the blocked Mathlib modules | Specifications are stated in elementary form, without analysis modules | Lemma lead | GATE-LEMMA-05 | M5 | N12-26 |
| OPEN-LEMMA-3 | The cost of kernel evaluation on exact rationals at specimen size | Reflection is limited to specimens that fit the budget | Lemma lead | GATE-LEMMA-05 | M5 | N12-27 |
| OPEN-LEMMA-4 | Whether one auditor is enough, or a family certificate needs a second independent checker (FrankenSim's `fs-checker`, or FrankenSymPy's capsule consumer for polynomial kinds) | One auditor with the mutation suite of GATE-LEMMA-03 | Lemma lead | GATE-LEMMA-04 | M4 | N12-29 |
| OPEN-LEMMA-5 | Whether certificates change any design decision | Counted in the phase ledger of the ten-fold test; if the count is zero, Lemma is evidence quality only and stays off the critical path | Lemma lead | The M7 tournament | M7 | N12-12 |
| OPEN-LEMMA-6 | Whether the Mathlib module that holds the orbit-counting lemma of theorem card T-24 re-checks in FrankenLean at the pinned commit | Cited only after a recorded re-check; until then the count is `Tested` | Lemma lead | GATE-LEMMA-02 | M5 | N12-25 |
| OPEN-MARROW-1 | Whether to canonicalize quaternion sign in stored state | No; state stays continuous and observations use sign-invariant features | Marrow lead | GATE-MARROW-01 | M1 | N13-09 |
| OPEN-MARROW-2 | Whether exact field arithmetic extends to radial designs with n above 12 | No; such designs are symmetric by construction, and imported ones use the tolerance route with a `Tested` label | Marrow lead | Nothing | none | considerations of N13-13 and BD-056 |
| OPEN-MARROW-3 | Whether the exact fallback of guarded predicates may use 64-bit integers where a backend offers them | 32-bit limbs everywhere, so the three backends run the same integer program | Marrow lead | GATE-MARROW-05 | M2 | N13-31 |
| OPEN-SKEL-1 | A text syntax for designs | Closed by architecture decision ADR-SKEL-02: JSON is canonical and the S-expression view is lossless and optional | none | Closed; tombstoned; the identifier is not reused | closed | BD-057 carries the view |
| OPEN-SKEL-2 | Where card prices come from, and under what licence | Prices are `assumed`, the cost budget is advisory, and `--strict-data` refuses a cost verdict | Project lead; asked of the owner with the other data-source questions | No gate before the build packet | M5 | N41-13 |
| OPEN-SKEL-3 | How a task with continuous world parameters declares the finite family that preflight stage 8 searches | The task's own randomization grid, recorded in the witness | Arena lead | Before stage 8 is funded | M3 | N14b-17 |
| OPEN-SKEL-4 | Whether the `lattice` fill needs anisotropy | Isotropic fraction only, with the assumption printed in the check report | Foundry lead | Nothing before the fabrication work | none before M5 | considerations of BD-055 |
| OPEN-SINEW-1 | Which real actuator parts ship in the catalogue, and under what data licence | Generic classes only | Project lead; asked of the owner at M3, one milestone before its M4 deadline, with the sensor-card and film-table questions | Nothing before M4 | M4 | N15b-25 |
| OPEN-SINEW-2 | The default Stribeck shape and stiction velocity ε on batched tiers | The rational shape and ε = 10⁻³ rad/s, both overridable by a card | Sinew lead | GATE-SINEW-04 | M2 | N15b-26 |
| OPEN-SINEW-3 | Whether iron loss is on by default at actuator level L2 | On when the card gives coefficients; otherwise zero, with the flag `iron loss not modelled` on every result | Sinew lead | GATE-SINEW-05 | M1 | N15b-27 |
| OPEN-SINEW-4 | Whether the second polarization pair of the battery model on the referee is composed from existing FrankenSim circuit elements or needs an upstream change | Open the upstream work item (FD-FSIM-21); an in-house `f64` reference until it lands | Airlock lead | The admission test for `fs-phs` | M1 | N15b-28 |

Status at entry: sixteen `open`, one `closed` (OPEN-SKEL-1, by ADR-SKEL-02). Two rows are among the questions put to
the owner as groups: OPEN-SKEL-2 (one cited source each for dimensional norms, retrofit prices and legality, card
prices and the parts catalogue; default generic ranges and prices labelled `assumed`, constraints marked
unchecked; cost of the default: no claim about realism or cost in a published report, and `--strict-data` refuses a
cost verdict) and OPEN-SINEW-1 (which real actuator parts, sensor cards and film or soil tables enter the
catalogues, under what data licences; default generic classes and ranges labelled `assumed`; cost: every result
that depends on them stays `Estimated`, and realism and Reality claims are refused).

## Acceptance criteria

- [ ] `registries/open.toml` contains the seventeen records with no empty required field; OPEN-SKEL-1 has status
      `closed` with its ADR and cannot be reused.
- [ ] Every row that blocks something names a decision bead that exists in the tracker; the two rows that block
      nothing name the bead whose considerations carry them.
- [ ] Each default is classified with an effect from the closed set, and none adds a claim.
- [ ] Deadlines agree with the milestone of the gate each row names, as read from the gate register.

## Test plan

### Unit tests
`crates/frob-conformance/tests/open_part1.rs`: `seventeen_rows_load`; `ids_match_expected_set` (LEMMA-1 to -6,
MARROW-1 to -3, SKEL-1 to -4, SINEW-1 to -4); `skel_1_is_tombstoned`; `decision_beads_resolve`;
`deadline_matches_gate_milestone` (GATE-LEMMA-05 at M5, GATE-LEMMA-04 at M4, GATE-MARROW-05 at M2, GATE-SINEW-04
at M2, GATE-SINEW-05 at M1); `no_default_adds_claim`.

### End-to-end script and logging
Covered by `scripts/e2e/registers_exit.sh` (N41-04), which lists these rows at the exits where they fall due and
logs one line per row with status and deciding bead.

### Known answers and negative controls
Known answers: 17 rows, 1 closed, 6 + 3 + 4 + 4 by subsystem. Negative controls: a second question entered under
OPEN-SKEL-1; OPEN-LEMMA-1 marked decided with no admission record; a blocking row with its decision bead removed.

## Considerations

- OPEN-LEMMA-5 is the question that could move a whole subsystem off the critical path; its evidence is a count in
  the tournament's phase ledger, collected from M1 by the decision-effect ledger.
- The data-source questions (OPEN-SKEL-2, OPEN-SINEW-1) do not block engineering; they bound what may be said. Until
  they are answered every affected result prints `assumed` and stays `Estimated`.
- If the proof siblings do not mature, the defaults of OPEN-LEMMA-1 to -3 and -6 keep the affected results at
  `Tested`, which is a weaker word and still an honest one.
- N14b-26 prints the four Skeleton questions with the same defaults and owners as the rows above (compared when
  the two beads were reconciled). It words the scope of OPEN-SKEL-4 as "nothing before the structural work" and
  this bead as "nothing before the fabrication work"; both mean the Foundry chapter's work, which lands at M5.
  The record is this bead's.

## References

§42.3 (rows for Lemma, Marrow, Skeleton, Sinew), §42.2 (the grouped owner rows), §42.4 (the closed questions),
§14.15 (the Skeleton questions in full).
