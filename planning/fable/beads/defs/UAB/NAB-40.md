---
key: "NAB-40"
title: "killed-idea register: KI-64 to KI-67, index by hypothesis, refuted-or-declined index, three declined records"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "delivery", "gauntlet", "solid", "registry", "decision-record", "owner-decision"]
parent: "EPIC-M0"
deps: ["NAB-32", "NAB-33", "NAB-34", "NAB-35", "NAB-36", "NAB-37", "NAB-38", "NAB-39"]
needs: ["§0.8: the hypothesis verdict register with its states and rows H-01 to H-29, against which the index is checked (N00-10, N00-11, N00-12)", "§21.26: the counts-beyond-2³² rule and its gate GATE-SPARK-23, the surviving form of KI-67 (BD-207)"]
related: ["NAB-42", "NAB-46", "NAB-47", "NAB-49", "NAB-50", "N00-10", "BD-029", "BD-060", "BD-069", "BD-207", "BD-258"]
gates: []
plan_refs: ["Appendix C.2.9", "Appendix C.2.10", "Appendix B.7", "§0.8", "§21.26"]
---
## Background

FrankenRobotics keeps every idea that was proposed and then refuted or deflated in a register
(`registries/killed_ideas.toml`, rendered as `docs/decisions/killed-ideas.md`; schema and revive lint in
NAB-32). NAB-33 to NAB-39 carry entries KI-01 to KI-63. This bead closes the register. It carries:

1. the **four further entries** KI-64 to KI-67, proposed late and recorded so that they are not proposed
   again;
2. the **index by hypothesis**, which joins the register to the hypothesis verdict register (the list of
   planning hypotheses H-01 to H-29 with their states ACTIVE, NARROWED, REFUTED, DEFERRED): the rule is that
   the strong form of a narrowed or refuted hypothesis appears once in the project's documents, here;
3. the **refuted-or-declined index**: a list of 31 ideas or strong forms by the names under which they were
   proposed, each with what survives, joined to its register record so that someone who knows an idea by its
   proposal name can find the refutation;
4. three records for ideas that the refuted-or-declined list names but for which **no refuting calculation is
   on file**. The list says each of its ideas is treated in the register with the calculation or evidence that
   refuted it; for three that is not so. They are entered as *declined* records with the reason marked as not
   recorded, a default wording proposed here, and an owner decision to confirm it.

Tier: Solid (a decision record). The owner-decision label applies to item 4 only.

## Scope

In: records `KI-64` to `KI-67`; the `hypothesis` fields of all records as the index gives them; the alias
table from proposal names to records; records `KD-01` to `KD-03`. Out: the schema and lint (NAB-32); the
entries KI-01 to KI-63; the parked records `KP-01` to `KP-04` and the declined record `KD-04`, which come from
the not-yet-placed list (NAB-49).

## Technical approach

**1. The four further entries.** Entries 64 to 66 restate, from a different starting point, mechanisms that
earlier entries already deflate (KI-24; KI-22 and KI-54; KI-09), and their records carry `restates = [...]`.
Entry 67 is new. Transcription as in NAB-33.

**KI-64. A module algebra interpreted at run time around the step**
- *Strong form.* Wrap the running step in a composition layer that is evaluated every step.
- *Refutation.* Composition is a compile-time fact about ports. At run time it adds an indirection to every read
  of a write-dense state, and it cannot express contact coupled through the base (KI-24).
- *Survives.* Modules compose at compile time into flat tables (§14.6, C-12).
- *Revive only if.* None (as KI-24).
- *Carried by.* BD-060, BD-061.

**KI-65. A safety certificate per contact stratum of the full robot, or one lifted-linear predictive model for the
whole hybrid system**
- *Strong form.* Certify safety stratum by stratum, or plan with one lifted linear model across every contact mode.
- *Refutation.* DESIGN arithmetic: strata grow as 3ᵏ in the number of contacts k, 81 at four and 531,441 at
  twelve. A lifted linear model has no representation of an impact jump, and its parameter count grows
  quadratically in the lifted dimension (KI-22: 1,891² ≈ 3.58 × 10⁶).
- *Survives.* The energy budget, reduced-order filters with a measured margin, and small reachable-set tables
  (§22.11, §22.12).
- *Revive only if.* None (as KI-22 and KI-54).
- *Carried by.* BD-337, BD-339, BD-440.

**KI-66. Time-parallel rollouts, and an asynchronous event calendar as the core stepping model**
- *Strong form.* Integrate in parallel in time with coarse and fine propagators, or advance each world on its own
  event calendar.
- *Refutation.* Contact breaks the agreement between coarse and fine propagators that time-parallel schemes need.
  Policy and sensor ticks at 50 to 100 Hz bound the lookahead of any calendar in a cluttered interior to 5 to 20
  physics steps (KI-09).
- *Survives.* The lockstep batched step and the transactional referee (§16.5, §16.6).
- *Revive only if.* None (as KI-09).
- *Carried by.* BD-081, BD-159.

**KI-67. 64-bit sums accumulated with device atomics on every backend**
- *Strong form.* Accumulate counts and integer sums on the device with 64-bit atomic adds.
- *Refutation.* FACT to recheck (§21.26, re-run by the fingerprint): 64-bit integer atomic add and
  compare-and-swap are reported absent on Metal, and one backend without them forbids them in a mixed run (§21.6,
  rule 7). DESIGN arithmetic: a 32-bit count wraps after 2³² increments, which a run at 4 M world-steps/s reaches
  in 18 minutes if one bin receives every sample.
- *Survives.* 32-bit counts with a no-wrap bound checked at plan time and host drains into 64-bit totals; a
  two-limb reduction in a fixed order for wider sums (§21.26; GATE-SPARK-23).
- *Revive only if.* The hardware fingerprint reports 64-bit integer atomics on every backend of the run (GATE-SPARK-23).
- *Carried by.* BD-029, BD-207.

Machine-checked arithmetic: 3ᵏ strata at k = 4 and k = 12 contacts are 81 and 531,441; the lifted operator of
KI-22 has 1,891² ≈ 3.58 × 10⁶ coefficients; a 32-bit count wraps after 2³² = 4,294,967,296 increments, which a
run at 4 M world-steps/s reaches in 1,074 s, about 18 minutes, if one bin receives every sample. The FACT
behind KI-67 (64-bit integer atomic add and compare-and-swap reported absent on Metal) is "FACT to recheck": it
is re-run by the hardware fingerprint, and one backend without them forbids them in a mixed run. Keywords:
KI-64: module algebra at run time, composition layer around the step. KI-65: certificate per contact stratum,
lifted-linear model for the hybrid system. KI-66: time-parallel rollouts, event calendar per world,
asynchronous stepping. KI-67: 64-bit atomics on device, atomic 64-bit accumulation.

**2. The index by hypothesis.** The `hypothesis` field of each record is set from this table (it is a
superset of the tags printed in the entry headings: KI-13, KI-18, KI-32, KI-34, KI-37 and KI-47 have no tag in
their heading and are joined to a hypothesis only here).

| Hypothesis | State | Records | Hypothesis | State | Records |
|---|---|---|---|---|---|
| H-01 | NARROWED | KI-26 | H-13 | NARROWED | KI-43 |
| H-02 | NARROWED | KI-27 | H-14 | NARROWED | KI-57 |
| H-03 | NARROWED | KI-42 | H-15 | REFUTED | KI-01 |
| H-04 | REFUTED for recursion-only specialization | KI-03 | H-16 | REFUTED as stated | KI-17, KI-18 |
| H-05 | NARROWED | KI-30 | H-17 | NARROWED | KI-47, KI-52 |
| H-06 | NARROWED | KI-35 | H-18 | ACTIVE | KI-59 |
| H-07 | NARROWED | KI-36, KI-37 | H-19 | ACTIVE | none |
| H-08 | NARROWED | KI-38 | H-20 | NARROWED | KI-31, KI-32 |
| H-09 | REFUTED for exact reuse | KI-33, KI-34 | H-21 | NARROWED | KI-28 |
| H-10 | NARROWED | KI-39 | H-22a, H-22b | ACTIVE | KI-40 |
| H-11 | NARROWED | KI-13, KI-14 | H-23 | NARROWED | KI-41 |
| H-12 | DEFERRED | KI-58 | H-24 | NARROWED | KI-29 |

Three notes belong to the index. (a) H-19, that a thin layer over an incumbent engine would do, is ACTIVE and
has no record: it is the composed-incumbent arm of the ten-fold test, and the thin-layer bet with its kill gate
(KILL-AUGUR-TIE) state what ships if that arm ties. (b) H-04b, H-12b and H-25 to H-29 are ACTIVE and have no
record either. (c) KI-40 and KI-59 stay as records of what was assumed, although their hypotheses are ACTIVE.
So 29 of the 67 records are joined to a hypothesis and 38 are not.

**3. The refuted-or-declined index** (alias table: `[[alias]] name, survives, records`). Each row is a name
under which an idea was proposed, what survives of it, and the record that holds the refutation.

| Idea or strong form | What survives, and where | Register record |
|---|---|---|
| Half-precision factorization of the contact Hessian on tensor cores | Single precision in scaled variables (§35.10); response batches (IP-55) | KI-01 |
| Lattice fluid tiles at control rate inside training | Slosh by sprung masses, the film law, and resolved fluid as a referee-side island (IP-61, IP-62) | KI-18 |
| Full-room fluid, granular or cloth physics in every world | Reduced laws and offline challenges (NONGOAL-02) | KI-17 |
| Global lifted-linear control of contact dynamics | Nothing on the critical path | KI-22, KI-65 |
| Topological stratification of contact modes, and exact algebraic reachability | Symmetry-orbit mode strata as a counted known answer (C-16); max-plus timing (IP-73) | KI-54 |
| Per-world time tiles between interactions, and time-parallel rollouts | Fixed decimation in a static graph (§21.20) | KI-09, KI-66 |
| End-to-end differentiation through every contact change | Labelled fixed-mode and return-map derivatives (IP-08, IP-16) | KI-23 |
| Live zero-copy CPU reads of GPU state | Fenced snapshots; the shadow referee (IP-48) | KI-13 |
| Copy-on-write forks of device state | Forks by indexed copy (§21.20, §29.8) | KI-10 |
| Learned step surrogates and learned corrections between tiers | None; an anti-goal (§6.3) | KI-45 (method 7); EV-P row for a learned correction |
| Search over kernel rewrites with proofs of equivalence | Rewrite classes with an error bound and a new kernel identity per changed bit (§21.7, DC-11) | KI-04 |
| Multi-agent design markets and debates as a product lever | Namespaces and capabilities for many agents (§32.13) | KI-63 (the row on markets over designs). No refuting calculation is on file for debates as a lever: record KD-01 |
| Custom stator and gearbox co-design | Actuator cards from existing parts (§15.3) | No entry with a refuting calculation is on file: record KD-02 |
| Live editing in the viewer | Typed edits and patches (§14.7, §14.8) | No entry with a refuting calculation is on file: record KD-03 |
| Simulated humans as validated agents | Scripted actors as dressing (IP-20) | KI-63 (the row on simulated humans) |
| One universal policy across all morphologies | Family policies per class (IP-04; NONGOAL-10) | KI-27 |
| Sampling control as a stand-in evaluator for a trained policy | A screening rung, and the delivered-controller lane of §22.8 (IP-03; NONGOAL-14) | KI-26 |
| Per-design kernels as the default path, and the launch-cost advantage of per-morphology dispatch | Model variants as data; specialization for survivors (IP-12, IP-57) | KI-03, KI-07 |
| One megakernel of precompiled limbs at near-homogeneous speed, with a lossless limb summary | The limb port interface, semidefinite and exact only without base-coupled contact (IP-53) | KI-07 |
| Staged erasure as a throughput lever | A compile-time rule (IP-60) | KI-08 |
| The clairvoyant barrier condition, the smallest-cone rule and the closed-form projection | The rewritten shield (IP-25) | KI-32 |
| A stratified probability bound that validates itself and holds under shift | The typed product and a conditional bound with independent labels (IP-26) | KI-55 |
| Sampling only inside a certified inner ellipsoid; feasibility by a logarithmic change of variables | Per-constraint boxes behind proved exclusions (IP-28) | KI-53 |
| Unbounded whole-robot synthesis by elimination | Bounded exact synthesis, then continuation (IP-30, IP-76) | KI-52 |
| Family queries and minimum-label queries as capabilities no existing method has | The queries, their checked answers, and the tie rule of KILL-AUGUR-TIE and KILL-FAMILYCERT | KI-56 |
| Adaptive-fidelity stopping as a decision procedure | A scheduling heuristic that orders work without touching validity (§28.19.4) | KI-45 (method 8) |
| Verdict reuse by sensitivity margin | Dependency-keyed reuse (IP-45) | KI-34 |
| Requirement rewards that remove reward hacking; a lifecycle step-count ratio read as a speedup | Unit-normalized margins (IP-43); lifecycle results classed `Estimated` (IP-39) | KI-43 (rewards); KI-60 (the lifecycle step-count ratio) |
| The headline ratios of campaign quotienting, rare-failure search, symmetry deduplication and the scene bank | The mechanisms with recomputed ranges (IP-02, IP-14, IP-31, IP-50) | KI-60 |
| Unconstrained zero-sum adversarial worlds; automated board-support generation | Search with a solvability witness (IP-31); controller code behind a caller-owned interface (IP-38) | KI-46 (adversarial worlds); KI-63 (board support) |
| Faster numerics alone as the source of the ten-fold | Engine throughput as parity (§2.3, §2.5) | KI-59 |

**4. Three declined records with no refuting calculation on file.**

| Id | Idea as proposed | What survives | Reason on file | Default wording proposed here (DESIGN, for the owner to confirm) |
|---|---|---|---|---|
| KD-01 | Multi-agent design debates as a product lever (markets over designs are covered by KI-63) | Namespaces and capabilities for many agents | None for debates. For markets: unvalidatable; no baseline exists | A debate among agents has no acceptance evidence of its own: what is accepted is a design that passes a frozen battery or a qualification request, whoever argued for it. Declined as a lever; agents coordinate through sessions, namespaces and capability scopes |
| KD-02 | Custom stator and gearbox co-design | Actuator cards from existing parts | None | Every actuator constant must come from a card with provenance per field; a designed stator has no data sheet, so its constants would be invented. Declined until an electromagnetic design lane with its own known answers exists; the generic catalogue and the co-design closure check cover selection among existing parts |
| KD-03 | Live editing in the viewer | Typed edits and patches | None | The viewer is presentation, and presentation may not change physics or delivered observations (the first noninterference property). A change to a design is a typed edit with preconditions, an inverse and an invalidation set, submitted as a patch with an expected parent; a gesture in a viewer carries none of these. Declined; the viewer may *compose* a patch for the user to submit |

Each `KD` record has `state = "declined"`, `refutation = ""`, `reason_on_file = "none"`, the default wording
as `reason_proposed`, `revive_only_if` = "an accepted ADR that states a consumer, an acceptance test and the
evidence", and `owner_confirmed = false`. Fail-closed default until the owner decides: the idea is not built,
and the revive lint treats the record like any other.

## Acceptance criteria

- [ ] Records `KI-64` … `KI-67` exist with the three paragraphs verbatim, `restates` for the first three,
      revival conditions and carriers; the register now holds 67 `KI` records.
- [ ] The `hypothesis` field of every record equals the index: 29 records joined to 23 hypothesis rows
      (H-22a and H-22b share one row), 38 records with none; H-19, H-04b, H-12b and H-25 to H-29 have no
      record, and a test fails if one is added without an ADR.
- [ ] Every REFUTED or NARROWED hypothesis of the hypothesis register has at least one record, and each
      record's `hypothesis_state` equals that register's current state.
- [ ] The alias table has 31 rows and every row resolves to at least one existing record (`KI`, or one of
      `KD-01` to `KD-03`).
- [ ] `KD-01` to `KD-03` exist with `owner_confirmed = false`; the generated document prints "reason not
      recorded; default wording pending the owner's confirmation" beside each.
- [ ] `GATE-SPARK-23` is named by KI-67 and exists in the gate register.

## Test plan

### Unit tests

`crates/frob-conformance/tests/killed_ideas.rs`, module `g8_index`:

- `ki64_to_ki67_complete`; `restates_resolve` (KI-64 → KI-24; KI-65 → KI-22, KI-54; KI-66 → KI-09).
- `strata_counts`: 3⁴ = 81; 3¹² = 531,441.
- `wrap_time`: 2³² / 4,000,000 = 1,073.74 s = 17.9 min.
- `index_matches_records`: the 24 table cells above against the records' fields; 29 joined, 38 not.
- `index_matches_hypothesis_register` (skipped with a recorded `skip` until that registry exists): states
  equal; every REFUTED or NARROWED row has a record; ACTIVE rows H-19, H-04b, H-12b, H-25 … H-29 have none.
- `alias_table_has_31_rows_and_resolves`.
- `declined_records_pending_owner`: three records, `owner_confirmed = false`, non-empty `reason_proposed`.
- `total_is_67`.

### End-to-end script and logging

`scripts/e2e/decision_records.sh` (NAB-50) asserts 67 records, the index join, the 31 aliases and the three
pending declined records, and looks an idea up by its proposal name ("copy-on-write forks of device state" →
KI-10). Logged per STD-E2E.

### Known answers and negative controls

Known answers: 67 records in groups of 9, 7, 9, 7, 14, 10, 3, 4 and 4; 29 joined to hypotheses; 31 aliases;
3 declined records here. Negative controls: a record tagged H-19 (must fail: an ACTIVE hypothesis with no
record by rule); an alias that points at a record id that does not exist; a `KD` record with
`owner_confirmed = true` and no ADR; the hypothesis register showing H-15 as NARROWED while KI-01 says REFUTED
(state mismatch).

## Considerations

- The three declined ideas are small, and the missing reasons are a finding of the conversion, not a defect of
  the ideas' rejection. Recording a default and asking the owner costs one decision; leaving them without a
  record would let them be proposed again with nothing to cite.
- The index is the join that the hypothesis register's once-only rule depends on; when a hypothesis changes
  state at a milestone exit, the sweep updates `hypothesis_state` here in the same change.
- OPEN (owner): confirm or replace the default wordings of KD-01 to KD-03. Fail-closed default: declined, not
  built.

## References

Appendix C.2.9, C.2.10; Appendix B.7; §0.8; §21.26.
