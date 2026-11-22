# Coverage of drafting unit U15a (§15 Sinew, subsections §15.1 to §15.9, and six seed beads)

35 bead files: 6 seed beads (BD-069, BD-072, BD-073, BD-076, BD-112, BD-275), 28 new beads (N15a-01 to N15a-28) and
the chapter's known-answer registry (N15-KA).

## (a) Subsections and the beads that carry them

| Subsection | Title | Beads |
|---|---|---|
| §15 preamble and §15.1 | What it owns, and why actuators are first-class | N15a-01 (crate, ownership, the parity-not-novelty claim, foundations); the four additions land in BD-069, BD-076, N15a-08, N15a-09, N15a-14, N15a-16 |
| §15.2 | Decisions | ADR-SINEW-01 → BD-069; ADR-SINEW-02, -03 → N15a-08, N15a-09; ADR-SINEW-04 → N15a-14, BD-072; ADR-SINEW-05 → N15a-02; ADR-SINEW-06 → N15a-17; ADR-SINEW-07 → N15a-07; ADR-SINEW-08 → N15a-15; the decision table itself in N15a-01 |
| §15.3 | The actuator card | BD-069 (schema, provenance, identity, field groups, three checks, catalog, verbs), N15a-02 (conventions), N15a-03 (checks at binding, `--strict-data`), N15a-25 (end-to-end) |
| §15.4 | The model ladder | N15a-04 (levels and options as identity, capability table, FROB-REF-136, L1 default, L1 screen), N15a-24 (GATE-SINEW-01 on a running referee); L0/L1 and L2 implementations are seeds BD-070, BD-071 (unit U15b); L3 rotor coordinate N15a-09; L3 compliance N15a-19 with pilot N15a-20; L4 N15a-21 with pilot N15a-22; rotor port seed BD-419 (U15b); ageing option (U15b) |
| §15.5 | The port-Hamiltonian form of each element | N15a-05 (element table, worked instance, dq machine, referee stepping, PO-SINEW-01), N15a-06 (thermal node, first law, per-step second law), BD-076 (port balance and residual), N15a-07 (batched-tier arithmetic and its residuals), N15a-23 (passivity and second-law battery) |
| §15.6 | Stiction and backlash as rows in the solve | N15a-08 (stiction row), N15a-09 (backlash rows, rotor coordinate), N15a-10 (rotor elimination, PO-SINEW-02), N15a-11 (creep, step dependence, gyroscopic hypothesis in the fidelity ledger), N15a-12 (the six known answers, GATE-SINEW-04), N15a-13 (KILL-SINEW-ROWS); solving the rows is seeds BD-084 and BD-155 (U15b) |
| §15.7 | Winding temperature, resistance, and thermal runaway | N15a-14 (closed forms and solves), BD-072 (L2 laws, derating and trip, `actuator.thermal`, FROB-REF-132, GATE-SINEW-05), N15a-15 (duty-history start), N15a-26 (end-to-end) |
| §15.8 | The voltage limit and field weakening | N15a-16 (static envelope library), BD-073 (L2 enforcement, `actuator.envelope`, FROB-REF-131, GATE-SINEW-06), N15a-02 (V_lim by modulation), N15a-26 (end-to-end) |
| §15.9 | Transmissions | N15a-17 (four-quadrant loss in the stiction bound, the counterexample, self-locking, screw), N15a-18 (battery: four-quadrant, screw, coupling differential, linkage modulus, PO-SINEW-03), N15a-19 and N15a-20 (belts, cables, series elasticity); the transmission mechanisms as a whole are seed BD-074 (U15b) |

Seed beads mapped to this unit by keyword whose content lies in other chapters, written completely here:

| Seed | Source sections read | Beads |
|---|---|---|
| BD-112 `explain.run`, `explain.design` | §28.5, §28.3, §29.10, §29.9, §29.12, §29.5, §27.1, §32.4, Appendix J | BD-112, N15a-27 (end-to-end) |
| BD-275 the hard-world set | §26.8, §26.1 to §26.3, §26.5 to §26.7, §26.12 to §26.16, §24.8, §24.9, §24.13, §38.8 | BD-275, N15a-28 (end-to-end) |

## (b) Gates, kill gates and the beads that measure or decide them

| Gate | Milestone (§40) | Beads of this unit | Expected from another unit |
|---|---|---|---|
| GATE-SINEW-01 | M1 | N15a-24 (measurement on the running referee and gate record), N15a-04, N15-KA | BD-070, BD-071 (U15b) implement the levels and hold the rows in their unit tests |
| GATE-SINEW-02 | M1 referee; M2 single precision | BD-076 (referee row), N15a-24 (script), N15a-07 (single-precision statistic on the host reference) | BD-208 (U29a) records the single-precision row on devices |
| GATE-SINEW-03 | M2 | N15a-07 (battery definition and host-reference measurement) | BD-151 (U15b) records it on kernels |
| GATE-SINEW-04 | M1 referee; M2 batched | N15a-12 (fixtures, script, referee record), N15a-08, N15a-09, N15a-11 | BD-084 (referee rows), BD-155 (batched row) (U15b) |
| GATE-SINEW-05 | M1 | BD-072, N15a-14, N15a-15, N15a-26 | — |
| GATE-SINEW-06 | M1 | BD-073, N15a-16, N15a-26 | — |
| GATE-SINEW-07 | M1 | N15-KA only (rows KA-324 to KA-328) | BD-075 (U15b) measures it |
| GATE-SINEW-08 | M1 | N15a-23 (battery and record), BD-069, N15a-03, N15a-05, N15a-06, N15a-17, N15a-18, N15a-25 | BD-074 (U15b) names it in its done-condition |
| GATE-SINEW-09 | M2 | N15a-10 | BD-084 (U15b) names it in its done-condition |
| GATE-SINEW-10 | M1 | none (call site reserved in N15a-03) | U15b (§15.15) |
| KILL-SINEW-ROWS | M2 | N15a-13 (decision bead) | BD-155 (U15b) supplies the system under test and names the decision |
| KILL-SINEW-AGE | M6 | none (named in N15a-04) | U15b (§15.10, §15.14); BD-423 names the decision |
| KILL-SINEW-ROTOR | M6 | none (named in N15a-04) | U15b; BD-419 names the decision |
| GATE-NEMESIS-06 (hard-set row) | M3 | BD-275, N15a-28 | BD-272 (U26a) owns the gate |

## (c) Counts

- Known answers: 45 registered (KA-293 to KA-337, N15-KA). 38 of them are carried by tests of this unit's beads;
  KA-324 to KA-328, KA-334 and KA-337 are registered here and owned by U15b's beads. Also cited from other chapters'
  blocks: KA-181 to KA-186 (constructions C-37, C-38), KA-279, KA-280.
- Counterexamples: the registry N15-CX is U15b's. Cited and turned into negative controls here: CX-135, CX-136,
  CX-137, CX-138, CX-139, CX-140, CX-142 (seven of the nine of §15; CX-141 and CX-143 belong to §15.10 and
  §15.12), plus CX-090, CX-091 (C-38), CX-172 (§18.7) and CX-294 (§26.8).
- Refusal codes: the registry N15-REF is U15b's. Behaviour specified here for five of §15's seven: FROB-REF-130
  (BD-069, N15a-03, N15a-05, N15a-17), -131 (BD-073, BD-069), -132 (BD-072, N15a-15, BD-069), -134 (N15a-02,
  BD-069, N15a-03), -136 (N15a-04, N15a-02, N15a-09). FROB-REF-133 and -135 are §15.10 (U15b). Codes of other
  chapters used: FROB-REF-102, -103, -113, -241, -560, -567, -840, -888.
- Invariants: INV-SINEW-01 (N15a-05, N15a-17, N15a-23), -02 (BD-076), -03 (N15a-06, N15a-23), -05 (N15a-08),
  -06 (BD-069). INV-SINEW-04 is §15.10 (U15b). INV-NEMESIS-01, -04, -05, -06 in BD-275.
- Proof obligations: PO-SINEW-01 (N15a-05, N15a-23), PO-SINEW-02 (N15a-10), PO-SINEW-03 (N15a-18).
- Pilots: N15a-20 (series elasticity, Frontier), N15a-22 (hydraulic, Moonshot).
- Open questions placed in Considerations (none blocks work; each has its default): OPEN-SINEW-1 (BD-069),
  OPEN-SINEW-2 (N15a-08, N15a-12, N15a-13), OPEN-SINEW-3 (BD-072, N15a-05, N15a-14), OPEN-SINEW-4 (N15a-01),
  OPEN-NEMESIS-5 (BD-275). Risk: RISK-32 (N15a-01, BD-069), RISK-39 (BD-112).

## (d) Needs from other slices, and what this unit offers them

Needs (also in each bead's `needs` field):

- §15.10 (U15b): the battery and bus elements in port-Hamiltonian form; regeneration paths on the bus; the bus
  voltage per step (BD-071, BD-075).
- §15.15 (U15b): the signed motion witness run at card binding (GATE-SINEW-10); the generators that
  `actuator.envelope` returns for coupled transmissions; the coupling map C(q) for the block exception of rotor
  elimination.
- §18.5 (Traction): the compliant unilateral row with a dissipation time that a joint limit takes under the
  lockstep law, which the backlash rows reuse.
- §16.6 (Kinesis): the Newton system and dense Cholesky into which eliminated rotor terms are folded.
- §20 (Sensorium): proprioception reading the encoder on the side the card names.
- §21 (Spark): the straight-line program rules the host reference arithmetic must satisfy.
- §24.4 and §23.3 (Arena, Cortex): a task-file field for the thermal start and the reset path that applies it.
- §14.3 (Skeleton): the closure constructor for position-dependent linkage ratios.
- §28.4, §29.8, §27 (Atelier, Chronicle, Autopsy): sensitivity ranking, first divergence and interventions, which
  `explain.run` consumes when they exist.
- §30: a calibrated card with a measured stiffness (lifts the kill condition of N15a-19).
- §36: the performance harness and disclosure block for the KILL-SINEW-ROWS throughput measurement.

Offers (the other Sinew unit could not know these keys; the lead may add them as dependencies): BD-070 and BD-071
build on N15a-04 (level identity), N15a-05 and N15a-06 (elements, thermal node); BD-074 on N15a-17 (loss inside the
stiction bound); BD-084 on N15a-08 and N15a-09 (row descriptors); BD-151 on N15a-07 (reference arithmetic); BD-155
on N15a-10 (rotor elimination); BD-103 on N15a-14 and N15a-16 (static libraries).

## (e) Contradictions, gaps and what was done about them

1. **No refusal code for a card field without provenance or uncertainty.** The plan says such a card is refused and
   names no code. BD-069 uses the form-error code FROB-REF-103 as the fail-closed default; codes 137 to 139 are
   free in Sinew's range if the owner prefers a dedicated one.
2. **No code for check 3 of `actuator.check`** (stated ratings unreachable inside the card's own limits), and
   stage 1 of the preflight lists only 130 and 134 for card checks. BD-069 refuses with 131 or 132 when a card is
   checked alone; N15a-03 leaves the bound case to stage 6, where those codes already apply.
3. **Back-EMF constant convention.** The card requires `current_convention` only. N15a-02 and BD-069 define `ke`
   in the voltage quantity dual to the current convention, with line-to-line figures converted by the author.
4. **`six-step` modulation** is a legal card value with no stated voltage-limit radius. N15a-02: the referee's
   exact hexagon serves it; circle-based envelopes refuse with FROB-REF-136 until a derivation is on file.
5. **GATE-SINEW-03 (10⁻⁴ relative) against the implicit current update.** The update is strongly damped and cannot
   track sub-step electrical transients of the referee. N15a-07 fixes the comparison battery on settled values and
   integrals at matched refinement, before any measurement, and records that reading for the owner.
6. **GATE-SINEW-01's two time constants.** L/R and J·R/(k_t·k_e) are exact only when measured separately; on one
   coupled L2 step they differ from the formulas by more than 1% unless L/R ≪ J·R/k². N15a-24 measures the
   electrical one with a locked rotor and the mechanical one at L1.
7. **`steady(task)` with no admissible equilibrium.** The plan defines the start as the equilibrium for the task's
   mean loss and is silent when none exists or it exceeds a limit. N15a-15 refuses with FROB-REF-132 and never
   substitutes `cold`. Consequence, handled in BD-112 and N15a-27: the planted knee-thermal episode runs from a
   declared `soaked(T)` start, and the default start's refusal is asserted as the static form of the same finding.
8. **KILL-SINEW-ROWS statistics.** The trigger says "Newton iterations" and "regularized friction as a force".
   N15a-13 fixes the iteration statistic (the smallest fixed count at which 99% of world-steps converge), the
   smooth arm (rational saturation x/(1 + |x|)) and the throughput rule, before measurement.
9. **Hard-world set refusals without codes.** FROB-REF-567 and -560 are named; a failed admission check and an
   only-hard battery have no code. BD-275 keeps both as exit-3 refusals and asks the registry owner for two codes
   in Nemesis's block in the change that adds the verb.
10. **KA-336's closed form** is named and not written. N15a-22 derives it (v = (C_d·A_o/A_p)·√(2·(p_s − F/A_p)/ρ))
    as the pilot's derivation file; N15-KA marks the row `to set` until then.
11. **The series-elastic pilot is expected to bind.** The referee's discrete-gradient step is the implicit midpoint
    rule for a spring, whose frequency error is about (ω·h)²/12: about 3% at 47 Hz and a 480 Hz step. N15a-20
    states this as a HYPOTHESIS and fixes the consequences; N15a-19 names two repairs.
12. **Compliant backlash row.** The plan says "exactly as a joint limit" with tooth stiffness and a dissipation
    time and does not write the potential. N15a-09 gives one (a squared positive part) as DESIGN, to be replaced by
    Traction's own form of the lockstep law if it differs.
13. **Possible overlap with unit U15b.** This unit owns §15.4, §15.6 and §15.9 as text while the seed beads that
    implement them (BD-070, BD-071, BD-074, BD-084, BD-155) are U15b's. To avoid both gaps and duplicate
    mechanisms, this unit wrote descriptor, library, identity, battery and decision beads around those seeds
    (N15a-04, -08, -09, -10, -12, -13, -17, -18, -23, -24). If U15b also wrote a KILL-SINEW-ROWS decision bead or
    a GATE-SINEW-01, -04 or -08 measurement bead, the lead should merge them.
14. **Seeds mapped by keyword.** BD-112 (`explain`) and BD-275 (hard-world set) have nothing to do with actuators;
    they are parented to WP-13 and WP-55 and labelled with their real subsystems. `explain.run` at M1 lacks
    sensitivity ranking (M3), first divergence (M2), stills (M2) and interventions (M3); BD-112 states what it does
    in their absence and lists them in `needs`.
15. **Catalog location.** The plan gives no path for the starter catalog; BD-069 uses `assets/cards/` by analogy
    with `assets/archetypes/`.
