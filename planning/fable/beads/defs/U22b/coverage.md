# Coverage of drafting unit U22b

Chapter §22 Reflex, subsections §22.10 to §22.13. Seven seed beads and 41 new beads (`N22b-01` to `N22b-41`).

## (a) Subsections and the beads that carry them

| Subsection | Title | Beads |
|---|---|---|
| §22.10 | Skill artifacts (`.fskill`) | BD-336 (verbs `skill.compile`, `skill.check`, `skill.show`; GATE-REFLEX-08), N22b-01 (format FMT-FSKILL, `frob.skill/1`, limits block), N22b-02 (per-mode map w = w₀ + B_m·u), N22b-03 (allocation program, both witnesses, FROB-REF-448), N22b-04 (refinement, FROB-REF-444, enforcement of the five limits), N22b-05 (end-to-end script), N22b-06 (pilot [M], learned residual), N22b-07 (`skill.transport`, layer 2 [M], GATE-REFLEX-09), N22b-08 (KILL-SKILL) |
| §22.11 | The energy-budget safety tier (`.fenergy`) [F] | BD-337 (verb `shield.budget`, budget half of `shield.check`; GATE-SHIELD-01), N22b-09 (pilot [F]), N22b-10 (format FMT-FENERGY, `frob.energy/1`, exact admission), N22b-11 (the six terms, class, closed-form cutoff delay, "does not support" list), N22b-12 (four premises, port-graph walk, FROB-REF-451), N22b-13 (kernel `K-energy-monitor`), N22b-14 (proof kind `energy-budget`, T-28, mutation corpus, PO-SHIELD-01), N22b-15 (GATE-SHIELD-01 campaign), N22b-16 (KILL-ENERGY), N22b-17 (OPEN-SHIELD-1), N22b-18 (end-to-end script) |
| §22.12 | Runtime shields (`frob-shield`) [F] | BD-338 (verbs `shield.fit`, `shield.check`; the `Shield` record; GATE-SHIELD-02), BD-339 (GATE-SHIELD-03 campaign), BD-340 (`shield.report`; GATE-SHIELD-04; shipping rule for adaptive elements), BD-440 (epic, reachable-set tables), N22b-19 (pilot [F]), N22b-27 (OPEN-SHIELD-2), N22b-28 (KILL-SHIELD), N22b-32 (end-to-end script) and the rows below |
| §22.12.1 | The filter | N22b-20 (rows, three-part margin δ, quantifier order, split cones, common admissible command) |
| §22.12.2 | The projection | N22b-22 (one row, breakpoints, PO-SHIELD-02), N22b-23 (two to four rows, optimality check, `Checked<Command>`, PO-SHIELD-03, more than four coupled rows refused) |
| §22.12.3 | Backup controllers and monitors | N22b-24 (three backups with domains, sampled gain bound, PO-SHIELD-04), N22b-25 (five assumption monitors, alarms), N22b-26 (the violation bound, independent labels, shifted worlds separate) |
| §22.12.4 | Reduced-order barrier filters with a measured margin | N22b-21 (templates and tracking realization), N22b-29 (calibration, margin, scope key FROB-REF-457, tier term), N22b-30 (drift alarm), BD-340 (coverage gate) |
| §22.12.5 | Reachable-set tables for small subsystems [M] | BD-440 (epic), N22b-33 (pilot [M]), N22b-34 (builder), N22b-35 (lookup, proof kind `tube-inclusion`, GATE-SHIELD-05), N22b-36 (KILL-TUBE) |
| §22.12.6 | Shields during training; the rule for every adaptive element | N22b-31 (filter as environment stage, likelihood accounting, priced interventions, `shield-dominated`), BD-340 (the shipping rule, enforced at fit), N22b-30 and N22b-25 (rule cited per element) |
| §22.13 | Symmetric controllers under stabilizer intersection [S] | BD-327 (verb `control.symmetry`; GATE-REFLEX-07), N22b-37 (Γ_use, four stabilizers, FROB-REF-461, INV-REFLEX-04), N22b-38 (tied gains, isotypic blocks, linear only, reduced-against-full check at mode switches), N22b-39 (parity, equivariance in law, asymmetric worlds), N22b-40 (symmetry at an event), N22b-41 (end-to-end script) |

## (b) Gates, kill gates and decisions

| Identifier | Milestone | Measured or decided by |
|---|---|---|
| GATE-REFLEX-07 | M4 | BD-327 (gate), N22b-41 (script); mechanisms N22b-37 to N22b-40 |
| GATE-REFLEX-08 | M4 | BD-336 (gate), N22b-05 (script) |
| GATE-REFLEX-09 | M8 | N22b-07 (gate), decided with N22b-08; the Augur-owned epic BD-436 lists it in its done-condition |
| GATE-SHIELD-01 | M4 (the gate register moved it from M3) | N22b-15 (campaign), BD-337 (hand rows and record), N22b-18 (script) |
| GATE-SHIELD-02 | M4 | BD-338 (gate), N22b-32 (script) |
| GATE-SHIELD-03 | M4 | BD-339 (campaign); levels from N22b-27 |
| GATE-SHIELD-04 | M4 | BD-340 (gate), N22b-32 (smoke); levels from N22b-27 |
| GATE-SHIELD-05 | M8 | N22b-35 (gate), BD-440 (epic) |
| KILL-ENERGY | M4 | N22b-16 (decision); BD-337 closes on it |
| KILL-SHIELD | M4 | N22b-28 (decision); BD-339 supplies the measurement; BD-340 and N22b-31 depend on it |
| KILL-SKILL | M8 | N22b-08 (decision) |
| KILL-TUBE | M8 | N22b-36 (decision); BD-440 closes on it |
| OPEN-SHIELD-1 | M4 | N22b-17 (decision; default `assumed` cards and `Estimated` claims) |
| OPEN-SHIELD-2 | M3 | N22b-27 (owner decision; default α = 10⁻³ and 10⁻³ per episode) |

Gates of this chapter that this unit expects other units to carry: GATE-SHIELD-06 (§22.21) and GATE-SHIELD-07
(§22.25), both U22c, are referenced through `needs` by BD-338, N22b-20, N22b-24 and N22b-25;
GATE-REFLEX-01 to -06, GATE-GAIT-01 to -03, KILL-BASELINE, KILL-RESIDUAL, KILL-MPC, KILL-GAIT (U22a);
GATE-REFLEX-10, -11, GATE-GAIT-04, -05, KILL-SOILLIFT, KILL-MPCRECORD, KILL-FREEPERIOD, KILL-STRIDESEQ (U22c).

Pilots of §22.17 that belong to these subsections, each its own bead: energy budget (N22b-09), reduced-order
barrier shield (N22b-19), skill transport by learned residual (N22b-06), reachable-set tables (N22b-33).

## (c) Counts and identifiers carried

- Known answers of §22 carried with values (11 of the chapter's rows): KA-565, KA-566, KA-567 (skills);
  KA-568, KA-569, KA-570 (energy); KA-571, KA-572, KA-573 (shields); KA-574, KA-575 (symmetry). Also cited
  with values: KA-556 (friction certificate, §22.5), KA-066 to KA-071 (C-09), KA-153 to KA-156 (C-31).
- Counterexamples of §22 carried (7): CX-236, CX-239, CX-240, CX-241, CX-242, CX-243, CX-244. Also cited:
  CX-235, CX-012, CX-031, CX-032, CX-033, CX-058, CX-074, CX-075, CX-076, CX-109, CX-249.
- Refusal codes carried with meaning, trigger, repairs and exit code (6 of the chapter's 11): FROB-REF-444,
  -448 (BD-336, N22b-03, N22b-04); -451 (BD-337, N22b-11, N22b-12); -454, -457 (BD-338, N22b-20, N22b-23,
  N22b-29); -461 (BD-327, N22b-37, N22b-38). Cited from other chapters: FROB-REF-215, -443, -527, -826, -834.
- Invariants: INV-SHIELD-01 (N22b-22, N22b-23, BD-338), INV-SHIELD-02 (N22b-20, N22b-26, BD-338),
  INV-REFLEX-04 (N22b-37, BD-327); INV-REFLEX-01 and -02 are cited where a layer record or the
  `ControllerId` is touched.
- Proof obligations: PO-SHIELD-01 (N22b-10, N22b-13, N22b-14, BD-337), PO-SHIELD-02 (N22b-22, BD-338),
  PO-SHIELD-03 (N22b-23, BD-338), PO-SHIELD-04 (N22b-24).
- Decisions: ADR-REFLEX-02 (N22b-03, N22b-23), ADR-REFLEX-06 (BD-337, BD-338, N22b-21), ADR-REFLEX-07
  (N22b-37, BD-327), ADR-REFLEX-08 (N22b-23, N22b-24, BD-340).
- Verbs (Appendix J): `skill.compile`, `skill.check`, `skill.show` (BD-336); `skill.transport` (N22b-07);
  `shield.budget` (BD-337); `shield.check` (BD-337 for budgets, BD-338 for shields); `shield.fit` (BD-338);
  `shield.report` (BD-340); `control.symmetry` (BD-327).
- Formats and types: FMT-FSKILL (N22b-01); FMT-FENERGY and the `EnergyBudget` sketch of Appendix I.15
  (N22b-10); the `Shield` and `ShieldEvent` sketches (BD-338); the worked artifact of Appendix L.5 (N22b-09,
  N22b-10, N22b-18). Kernel `K-energy-monitor` (N22b-13). Proof kinds `energy-budget` (N22b-14) and
  `tube-inclusion` (N22b-35).
- Other identifiers named where served: BET-25, NS-23, C-31, C-09, C-12, C-13, C-32, T-28, T-18, A-20, A-22,
  A-24, MR-22, GB-26, GB-27.

## (d) Needs from other slices

| Need | Asked by |
|---|---|
| The chapter registries: N22-KA (U22a), N22-CX and N22-REF (U22c) | All beads cite numbers and state values inline |
| §22.21 timeliness rule, pre-slip aliasing fixture, GATE-SHIELD-06 (U22c) | BD-338, BD-339, N22b-20, N22b-25 |
| §22.25 sampled damping: four known-answer rows, held-work rule, `shield.fit` gain refusal, GATE-SHIELD-07 (U22c) | BD-338, N22b-13, N22b-24 |
| §22.4 `ControllerId` with the group used; §22.9.3 saltation matrices, labels and grazing threshold (U22a) | N22b-37, N22b-40 |
| `conic-farkas` checker in `frob-cert`; the proof-kind registry file; a theorem card for the continuous-time damping identity (Lemma, U12) | N22b-03, N22b-14, N22b-24, N22b-35 |
| The pilot register and the pilot rule (§38.5); registration of fixtures and planted defects (U38a, U38b) | N22b-06, N22b-09, N22b-19, N22b-33, N22b-15, BD-340, BD-327 |
| Format-registry rows for FMT-FSKILL and FMT-FENERGY (§34.9) | N22b-01, N22b-10 |
| Verb registration for the M4 set (BD-376) and refusal registry entries | BD-327, BD-336, BD-337, BD-338, BD-340 |
| `ActionFilter` trait and the driver-facing checked-command type in `frob-dyn` (§16, §22.1) | N22b-20, N22b-23 |
| Kernel-library registration and step-plan slot for `K-energy-monitor` and the projection kernel (§21) | N22b-13, N22b-22 |
| Stop-path delay distribution in the clock contract (§20); scene symmetries and regions (§19); hard-limit atoms and the two-latch and one-sided-patch tasks (§24) | N22b-12, N22b-21, N22b-25, N22b-37, N22b-39 |
| Evaluator-only access to hidden simulator state for labels (§29) | N22b-26, BD-339 |
| Absorber and drive bench import through Ground (§30); card fields for absorber envelopes (§15) | N22b-11, N22b-17 |
| PPO rule P-3 and rollout storage (§23.6); the penalty-trained comparison policy; family-policy treatment for skill transport (§23) | N22b-31, BD-339, N22b-07 |
| Compute envelopes for Moonshot pilots and pretraining (§36.14); the owner's ordering of Moonshot items (§43.9) | N22b-06, N22b-07, BD-440 |
| Exit reviews recording kill decisions (BD-378 at M4, BD-442 at M8, BD-319 at M3) | N22b-08, N22b-16, N22b-27, N22b-28, N22b-36 |

## (e) Contradictions, gaps and what was done

1. **Milestone of the energy tier.** Appendix J lists `shield.budget` and `shield.check` with first
   milestone M3; the gate register places GATE-SHIELD-01 and KILL-ENERGY at M4 "with the tier". The register
   governs: every energy bead is M4.
2. **The worked card's gravity term.** 8·9.81·0.040·0.08 is 0.251136 J (0.2503 J with the sine of the
   slope), printed as 0.25 J, which is below the product it bounds. The card is carried as declared decimal
   bounds (total 1.49 J, as the known answer requires); the builder (N22b-11) rounds its own derivations
   outward and reports a declared bound below a derived one as a finding, using the larger value.
3. **The card's speed.** The sketch has a speed error of 0.01 m/s and T_ub = 0.64 J "at 0.4 m/s". 0.4 m/s
   is read as the inflated speed (reading 0.39 m/s); both readings are tested (N22b-10).
4. **Where the flags sit.** The `.fskill` and `.fenergy` sketches hold a placeholder string for the flags,
   and Appendix L.5 uses the key `flags` where §22.11 uses `authorization`. Both formats carry a top-level
   `authorization` object with the three constants, as the envelope and the certificate bundle do.
5. **"One accumulator and one comparison per tick."** E_drive is defined as an integral over the cutoff
   delay bounded by P_max·Δt_cut. N22b-13 uses the bound in the admission comparison and integrates actual
   positive drive power after the cut as an auditable check (`cutoff-overrun`).
6. **≤ against "at zero margin it cuts".** Static admission uses ≤ (Appendix I.15); the runtime monitor
   continues only while the margin is positive.
7. **GATE-REFLEX-08's fixtures.** "Both hand fixtures" and "4 of 4 infeasible bodies" are not enumerated.
   BD-336 fixes them: the two-link arm and the friction push as the hand fixtures; four bodies binding on an
   actuator, a contact cone, a unilateral support and a speed-dependent actuator box. The floating-base
   counterexample CX-243 has no numbers in the plan; N22b-02 supplies a cart-tipping case.
8. **GATE-SHIELD-03's "stated bound".** A bound computed from the episodes it is compared with holds by
   construction. BD-339 freezes the bound on a disjoint estimation set of 15,000 episodes (three zero-event
   upper bounds sum to about 8 × 10⁻⁴ there; 5,000 episodes cannot state 10⁻³ that way) and reads "within the
   stated bound" literally (V/5,000 ≤ B), with the stricter reading noted for the pre-registration.
9. **"More than four coupled rows."** Coupling is defined in N22b-23 as sharing an actuator channel,
   transitively.
10. **The margin's tightening.** "The barriers are tightened by ε̂" is realized as h − ‖∇h‖·ε̂, which adds
    κ·‖∇h‖·ε̂ to a row with linear α (N22b-20); the tier term defaults to the largest positive referee-minus-fast
    difference on the replayed sample (N22b-29).
11. **Pilot scales.** The skill-transport pilot costs about 40 GPU-hours, beyond the "CPU in a day" pilot
    rule; it is carried with the plan's cost and an envelope `needs`. "24 target bodies" is read as 20
    feasible plus 4 infeasible. The energy pilot's "one-degree-of-freedom cart" gains a loose-payload
    configuration so that the contact sequence is not trivial.
12. **Expected generators.** The plan states the removed sets for the side slope and the front lidar only;
    the sets for the off-centre payload, the one-sided patch, the twist and the trot stance are derived in
    N22b-37 from the parity rule and repeated in BD-327.
13. **Fields added.** `limits` in `.fskill` (N22b-01), `cutoff` and `caps` in the `.fenergy` premises
    (N22b-12), an optional matrix form of the kinetic term (N22b-10), a registry entry for the shield levels
    (N22b-27), and a default `shield-dominated` fraction of 0.2 (N22b-31) are this unit's DESIGN.
14. **Reachable-set fixture.** The plan names "braking across two floor materials"; its numbers (tile
    boundary at 0.5 m, grid 128 × 128, error box, closed-form viability boundary and area 4.7181 m²/s) are
    fixed in N22b-33 from the chapter's braking fixture.
15. **Ownership overlaps to check at load time.** The proof kinds `energy-budget` and `tube-inclusion` are
    rows of §12.4 and may also appear in U12's beads; here they are N22b-14 and N22b-35 because BD-337's
    title assigns the first to this unit and §22.12.5 defines the second. BD-436 (skill transport epic) is
    owned by U25a; its Reflex half is N22b-06, N22b-07 and N22b-08 with `parent: "BD-436"`. The children of
    BD-440 use `parent: "BD-440"`.
16. **KILL-SHIELD per class.** N22b-28 decides the kill gate per morphology class (wheeled, legged), because
    the campaign has one subject of each and the plan expects shields to be strongest on wheeled bases.
17. **PO-SHIELD-04's theorem card** is named in §22.15 without a card number in §12.3; N22b-24 asks for it
    through `needs`.
