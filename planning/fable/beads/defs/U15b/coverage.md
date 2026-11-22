# Coverage of drafting unit U15b

Chapter §15 Sinew (actuators, transmissions, power), subsections §15.10 to §15.15, and the ten seed beads
assigned to this unit. Files: 10 seed beads, 2 registry beads (`N15-CX`, `N15-REF`), 38 new beads
(`N15b-01` to `N15b-38`); 50 bead files in all.

## (a) Subsections and the beads that carry them

| Subsection | Title | Carried by |
|---|---|---|
| §15.10 | The power bus and the battery | BD-075 (bus node, ceiling E²/(4·R), brownout event, undervoltage policy, shared current limit, stage-6 static check, verb `actuator.bus`); N15b-03 (two-RC battery, bounded state of charge, exhaustion event); N15b-04 (regeneration: to-battery, shunt, clip); N15b-14 (ageing states f_Q, f_R as constants); N15b-15 (ageing pilot); N15b-16 (KILL-SINEW-AGE); N15b-17 (ageing evolution law); N15b-28 (OPEN-SINEW-4, second polarization pair, FD-FSIM-21); BD-151 (`K-act-bus` and the battery kernel update); N15b-30 (end-to-end) |
| §15.11 | Energy at the ports | N15b-05 (the per-element account, the identity, the four rules, first offending element); N15b-04 (braking-mode lines); N15b-30 (end-to-end). Read by seed beads of other units: BD-076 (referee balance gate), BD-092 (system ledger), BD-208 (batched ledger) |
| §15.12 | What agreement between tiers cannot show | N15b-06 (uncertainty bands, per-field claim products, the assumed-and-generic share, `--strict-data`); N15b-07 (the `card-conditional` label); N15-CX (CX-143) |
| §15.13 | Refusals, invariants and known answers | N15-REF (the seven codes); N15b-38 (the refusals end to end); invariants and proof obligations as listed in the table below; known answers "beyond those tabulated": BD-070, BD-071, N15b-29 (KA-329 to KA-333), N15b-03 (KA-327, KA-328), N15b-22, N15b-36, BD-419 (KA-334) |
| §15.14 | Foundations, gates, kill gates, open questions | Foundation facts: BD-070, BD-071, BD-084, N15b-03, N15b-12. Gates: section (b). Kill gates: N15b-24 (ROWS), N15b-16 (AGE), N15b-23 (ROTOR). Frontier and Moonshot table: N15b-14, N15b-15, N15b-17 (ageing); N15b-18, N15b-19 (series elasticity, belt and cable compliance); N15b-20, N15b-21 (hydraulic L4); BD-419, N15b-22, N15b-36 (rotor port). Open questions: N15b-25 (OPEN-SINEW-1), N15b-26 (OPEN-SINEW-2), N15b-27 (OPEN-SINEW-3), N15b-28 (OPEN-SINEW-4) |
| §15.15 | Coupled transmissions: the torque set, the rotor energy and the direction witness (late; no seed bead) | N15b-10 (one map, rule 1: the torque zonotope in `actuator.envelope`); N15b-11 (rules 2 and 3: rotor inertia pulled back once; inertial bias of varying couplings); N15b-12 (the direction witness; `frob-fsim` reaches only `interconnect_mapped`); N15b-13 (GATE-SINEW-10 measured end to end); N15-CX (four provisional counterexamples) |

Subsections owned by other units that this unit's seed beads carry in full, because the seed is assigned
here:

| Subsection | Carried here by |
|---|---|
| §15.4 The model ladder | BD-070 (L0, L1), BD-071 (L2), N15b-18 (L3 Frontier part), N15b-20 (L4), N15b-36 (rotor port option), N15b-14/-17 (ageing option) |
| §15.5 Port-Hamiltonian form (element table; batched arithmetic) | BD-071 (worked motor instance, thermal nodes, entropy production), N15b-09 (structure of every element), BD-151 (the batched updates) |
| §15.6 Stiction and backlash as rows (bound law, rows, rotor elimination, six known answers) | BD-084 (referee), BD-155 (batched), N15b-08 (rotor elimination), N15b-31 and N15b-37 (end to end) |
| §15.9 Transmissions | BD-074 (four-quadrant loss, self-locking, square couplings, linkages), N15b-18 (compliance), N15b-32 (end to end) |
| §14.6 Reusable modules (format, port contract, the peak-only counterexample) | BD-060, N15b-01, N15b-02, N15b-33 |
| §28.4 rows A-10 and A-11; §14.11 stage 6 | BD-103, N15b-34 |

Invariants, proof obligations, decisions and other identifiers:

| Identifier | Carried by |
|---|---|
| INV-SINEW-01 (every element passive in four quadrants) | BD-074, N15b-09, N15b-04, N15b-32 |
| INV-SINEW-02 (port balance closes each step to a recorded residual) | N15b-05 (and BD-076 of the other unit for the gate) |
| INV-SINEW-03 (entropy production nonnegative) | BD-071, BD-074, N15b-04, N15b-32 |
| INV-SINEW-04 (state of charge bounded; exhaustion an event, never clipped) | N15b-03, BD-075, BD-151 |
| INV-SINEW-05 (row bounds constant within a solve; every row potential convex) | BD-084, BD-155 |
| INV-SINEW-06 (card hash covers every field) | Expected in BD-069 (other unit); touched by N15b-26 (`card_override_changes_hash`) |
| PO-SINEW-01 | N15b-09 (exact structural check; theorem-card proposal) |
| PO-SINEW-02 | N15b-08, N15b-37 |
| PO-SINEW-03 | BD-074, N15b-09 (theorem card T-15) |
| ADR-SINEW-02, -03 | BD-084, BD-155 |
| ADR-SINEW-04, -05, -08 | BD-071 (and BD-103 for -04) |
| ADR-SINEW-06 | BD-074 |
| ADR-SINEW-07 | BD-151 |
| ADR-SINEW-01 | N15b-06 (the unit owning §15.2 states it for the card bead) |
| ADR-SKEL-05, ADR-SKEL-06 | BD-060, BD-075 |
| RISK-32 | N15b-06, N15b-07, BD-070 |
| C-37, C-38, C-12 | BD-084/BD-155 (C-37); BD-070, BD-071, N15b-04, N15b-05 (C-38); BD-060, N15b-01, BD-103 (C-12) |
| T-08, T-12, T-15, T-29 | BD-155 (T-08); BD-103 (T-12); BD-074, N15b-09 (T-15); N15b-01 (T-29) |
| GB-03, GB-13 | BD-103, N15b-34 (GB-03); BD-075, N15b-03, N15b-14 (GB-13) |
| FD-FSIM-21 | N15b-28, N15b-03 |

## (b) Gates, kill gates and the beads that measure or decide them

| Gate | Milestone (register) | Measured or decided by |
|---|---|---|
| GATE-SINEW-01 | M1 | BD-070 (L1 rows), BD-071 (remaining rows), N15b-29 (receipt through the CLI) |
| GATE-SINEW-02 | M1 referee; M2 single precision | Referee row: BD-076 (other unit, expected). Single-precision row: BD-208 (other unit, expected). This unit supplies the account both read: N15b-05 |
| GATE-SINEW-03 | M2 | BD-151, N15b-35 |
| GATE-SINEW-04 | M1 referee; M2 batched | Referee: BD-084, N15b-31. Batched: BD-155, N15b-37. Default ε decided at it: N15b-26 |
| GATE-SINEW-05 | M1 | BD-072 (other unit, expected). OPEN-SINEW-3 decided at it: N15b-27 |
| GATE-SINEW-06 | M1 | BD-073 (other unit, expected) for the five closed forms; the circle-against-hexagon row where the circle is used: BD-151, N15b-35 |
| GATE-SINEW-07 | M1 | BD-075, N15b-03, N15b-30; switch-over re-run: N15b-28 |
| GATE-SINEW-08 | M1 | BD-074, N15b-04, N15b-09, N15b-32 |
| GATE-SINEW-09 | M2 | N15b-08, BD-155, N15b-37; referee reference from BD-084 |
| GATE-SINEW-10 | M1 | N15b-10, N15b-11, N15b-12, N15b-13 |
| KILL-SINEW-ROWS | M2 | N15b-24 (decision); BD-155 (comparator and counters) |
| KILL-SINEW-AGE | M6, before the lifecycle work package | N15b-16 (decision); N15b-15 (pilot); N15b-17 (the branch that exists only if it does not fire); N15b-14 (the fallback form) |
| KILL-SINEW-ROTOR | M6 | N15b-23 (decision); N15b-22 (pilot); BD-419 (epic); N15b-36 (the branch) |

Gates of other chapters that these beads contribute rows to: GATE-SKEL-06 (reversed port sign refuses:
BD-060, N15b-01, N15b-33; decided by BD-077), GATE-TRAC-04 joint rows (BD-155, N15b-37), GATE-ATEL-02
(BD-103, N15b-34), GATE-SKEL-04 (seeded defects for codes 130 to 134: N15b-38), GATE-GAUNT-08 (pilot
records: N15b-15, N15b-19, N15b-21, N15b-22).

Gates this unit expects another unit to carry: GATE-SINEW-05 and GATE-SINEW-06 (the unit owning §15.7 and
§15.8, through seeds BD-072 and BD-073); the referee row of GATE-SINEW-02 (BD-076); the single-precision row
of GATE-SINEW-02 (BD-208).

## (c) Counts

- Known answers of §15 (KA-293 to KA-337, 45 rows): 42 are cited by number with their values in this unit's
  beads (39 individually, plus KA-308, KA-316, KA-317 inside ranges in `N15-CX`). The three not cited here,
  KA-304, KA-311 and KA-314 (single-node equilibrium, the two-node condition, the voltage limit), belong to
  the thermal and envelope beads of the other unit. Known answers of other chapters cited: KA-077, KA-079,
  KA-080, KA-262, KA-271, KA-279, KA-280, KA-285.
- Counterexamples: 9 of Appendix N.12 (CX-135 to CX-143) in `N15-CX`, each with its planted mutant and
  owning test; 4 provisional rows for §15.15 with no catalogue number yet (CX-SINEW-a to -d); 3 related rows
  of other chapters cited (CX-128, CX-172, CX-303).
- Refusal codes: 7 (FROB-REF-130 to 136) in `N15-REF`, exercised by `N15b-38`; codes of other chapters
  raised or cited: 101, 103, 108, 111, 113, 114, 115, 116, 121, 125, 241, 831.

## (d) Needs from other slices

Prerequisites or adoptions owned elsewhere, for which no key of this unit exists:

- The unit owning §15.1 to §15.9: the known-answer registry `N15-KA`; the card schema (BD-069) must admit
  the fields these beads name (DESIGN names): `regen_policy` required for actuators on a bus that can
  receive regeneration, the `ageing` group of a battery card, `series_stiffness` and `series_damping`, the
  `rotor` group, `iron_loss` coefficients, the bus `brownout` and `regen` objects; BD-072 wires the
  `Steady(task)` thermal initial condition defined in BD-071; BD-073's `actuator.envelope` output gains the
  `joint_torque_set` object of N15b-10.
- §22.5 (control): the cone solver's programs carry τ_motor as variables with τ_joint = Cᵀ·τ_motor; no
  controller model adds rotor inertia a second time (N15b-10, N15b-11).
- §28.4 (analyses): A-08 and A-24 read the torque generators; BD-108 (A-23) keeps the joint time-indexed
  schedule; BD-110 runs the junction checks at stage 5 and the stage-6 functions.
- §25 (missions): queries read the same torque generators.
- §16 (dynamics): BD-082 exposes the guard class "actuator protection" for brownout, exhaustion and thermal
  trip; BD-159's shape class counts two backlash rows per level-L3 joint in the row capacity.
- §29 (telemetry): BD-092 and BD-208 read `frob.portaccount/1`; the failure taxonomy (BD-094) carries
  `brownout` and a battery-exhausted event.
- §33 (suite consumption): admission tests for the sibling files behind the `actuation` feature group
  (`fs-control` thermal bands and hexagon limiting, `fs-em`, `fs-thermal`); no seed key was found for them.
  The admission record of the two transformer entry points (§33.18 row 11, GATE-WS-09).
- §14 (design): the aerial-terrestrial hybrid variant as the rotor port's consumer (§14.10); the limb port
  packet and capability tables of modules (§14.6 [F], §14.16), which BD-060 leaves to their owners.
- §24 (arena): the verb `task.fixture` (BD-123) must accept the options the scripts assume (`--tier`,
  `--actuator-level`, `--measure`, `--precision`); these are DESIGN.
- §32 (surfaces): registry entries and schemas for `actuator.bus`, `module.list`, `module.show`,
  `module.check` and the per-analysis verbs (BD-137, BD-138).
- §38 (verification): the conformance harness builds test-only mutant features and records caught or
  missed (BD-025, BD-146); the catalogue owner assigns numbers to the four provisional counterexamples.
- §12 (proofs): whether PO-SINEW-01 becomes a numbered theorem card (N15b-09 files the proposal).
- §31 (fabrication): the lifecycle epic (BD-423) consumes the ageing states and needs KILL-SINEW-AGE decided.

## (e) Contradictions, gaps and what was done

1. **GATE-SINEW-09 milestone.** The seed BD-084 (M1) lists it; the gate register says M2. The register
   governs: the gate is recorded at M2 by N15b-08 and N15b-37; BD-084 supplies the referee reference at M1.
2. **BD-155's done-condition against the dependency order.** The seed asks for GATE-SINEW-04 on batched
   tiers and for KILL-SINEW-ROWS decided, yet the batched step (BD-159) depends on BD-155. BD-155 therefore
   measures the six cases at kernel level with a reference Newton driver and supplies the comparator and
   counters; tier-level receipts are N15b-37 and the decision is N15b-24, both after BD-159.
3. **GATE-SINEW-03 has a size and a tolerance and no case definition**, and the batched arithmetic differs
   from the referee by design (implicit rational updates, slower thermal clock, inscribed circle against the
   hexagon), so step-for-step agreement at 10⁻⁴ cannot hold in fast transients or near a hexagon vertex.
   BD-151 defines the cases (settled, quasi-steady, band-limited transients) and separates expected
   conservative differences into their own ledger rows. The lead should confirm this reading.
4. **§15.10 leaves three things unspecified**: the content of the undervoltage policy, the allocation rule
   under the bus `current_max`, and the polarization state used by the static check. BD-075 states DESIGN
   defaults for each and names the owner.
5. **No refusal code exists for an unmet module contract assumption.** Fail-closed default in N15b-01:
   FROB-REF-114 with a pointer at the assumption.
6. **How a module parameter names its target in the body is not fixed** (noted in the plan's worked
   artifact). BD-060 adopts a `path` in the lever grammar as the fail-closed default.
7. **FROB-REF-135 at run time**: inside a batch it is a world event and the run completes with findings;
   it is exit 3 only when the verb's own question cannot be answered. Stated in N15-REF.
8. **`--strict-data` and `generic` fields**: the refusal's wording names `assumed` only; N15b-06 reports
   `generic` fields in the share and does not refuse them.
9. **Tolerances marked "to set"** (KA-303, KA-312, KA-321, KA-328, KA-334, KA-336): each bead proposes a
   value and marks it as a proposal.
10. **KA-336's closed form is not stated in the chapter.** N15b-21 fixes it before the pilot runs.
11. **The ageing law's functional form** is described only as semi-empirical in throughput and calendar
    time; N15b-15 and N15b-17 use the form of the lifecycle table (√t plus throughput), labelled DESIGN.
12. **§15.15's four shortcuts have no counterexample numbers**; recorded as provisional rows in N15-CX.
13. **The rotor port's reaction-torque law and its energy export are not specified**; N15b-36 and the pilot
    N15b-22 give DESIGN forms (an air port, axial momentum theory).
14. **Work-package placement.** WP-09's gate list omits GATE-SINEW-04 although M1 acceptance includes its
    referee rows; BD-084 is placed under WP-09. BD-151 and BD-155 are placed under WP-45 (which names their
    gates) although the batched step of another package depends on them; `parent` is hierarchy only.
15. **BD-060 is assigned here while §14.6 belongs to another unit.** BD-060 covers the format and contract;
    N15b-01 the junction checks; N15b-02 the three verbs; N15b-33 the end-to-end test. Substitution
    (BD-077), the limb port packet and capability tables stay with their owners.
16. **The epic BD-419 lists its children in `deps`** (pilot, decision, element) because its done-condition
    is their completion; the children do not depend on the epic.
17. **`task.fixture` (BD-123) depends on BD-075 and BD-084**, so those two cannot depend on it; only the
    end-to-end beads do.
18. **Registry beads and their tests.** N15-REF and N15-CX close on the registry rows and loader rules; the
    behaviour through the CLI is N15b-38 and the per-row mutant runs, which need the mechanisms that in turn
    depend on the registries.
