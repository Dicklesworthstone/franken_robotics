---
key: "NAB-35"
title: "killed-idea register KI-17 to KI-25: worlds and physics (full-room fluid, lattice fluid, lifted models)"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "delivery", "habitat", "kinesis", "solid", "registry", "decision-record"]
parent: "EPIC-M0"
deps: ["N00-12", "NAB-11", "NAB-32"]
needs: []
related: ["NAB-36"]
gates: []
plan_refs: ["Appendix C.2.3", "§0.8 (H-16)", "§16.5", "§16.7", "§19.10"]
---
## Background

FrankenRobotics keeps every idea that was proposed and then refuted or deflated in a register
(`registries/killed_ideas.toml`, rendered as `docs/decisions/killed-ideas.md`; schema and revive lint in
NAB-32), each with the strong form as proposed, the refutation with its numbers, what survives, and the
condition under which it would be revived. A proposal that matches an entry must cite it.

This bead carries the third group, nine entries about **worlds and physics**. Interiors are where a mobile
robot fails: a wet floor, a cord, a rug, a sloshing container. The tempting answer is to simulate the matter:
fluid, cloth and granular solvers coupled to the rigid robot in every training world. The arithmetic says no
by orders of magnitude (a lattice fluid needs 152 bytes per cell, 45.6 GB for one room at 1 cm), and the
system instead uses a contact-level film-friction law, rigid proxies, reduced models, and resolved solvers
only beside a single world on the CPU referee, for finalists, behind a flag. The other entries of the group
are claims about dynamics that do not survive a hand-checkable case: symplecticity of an articulated
integrator, refusing simultaneous impacts, curvature as an assembly test, one lifted linear model for contact,
gradients through everything, composition at run time, entropy as a state.

Reading conventions: a number inside a *strong form* is the proposal's own; FACT figures have source-register
records and are re-read or re-run before a gate cites them; "DESIGN arithmetic" can be redone by hand; ranges
in *survives* lines are HYPOTHESES until the named gate records a value; thresholds are TARGETs. Tier of the
bead: Solid (a decision record).

## Scope

In: records `KI-17` to `KI-25` with calculations, keywords, carriers and revival conditions. Out: the schema,
harness and lint (NAB-32); the surviving mechanisms (their carriers).

## Technical approach

**Transcription rule.** As in NAB-33 (KI-17: `hypothesis_state = "REFUTED"`, row H-16 of N00-12). Revival
fields by NAB-33's rule: KI-17 `pilot` (GATE-HAB-09); KI-18 `none`; KI-19 `gate` (KILL-VARIATIONAL); KI-20
`none`; KI-21 `none`; KI-22 `pilot` (GATE-REFLEX-10, KILL-SOILLIFT); KI-23 `pilot` (KILL-STEPADJOINT,
KILL-FDGRAD); KI-24 `none`; KI-25 `none`.

**Machine-checked arithmetic of this group.**

| Record | Expression | Value |
|---|---|---|
| KI-17 | bytes per cell of a D3Q19 lattice in single precision with two population buffers: 19 × 4 × 2 | 152 |
| KI-17 | cells of a 10 × 10 × 3 m room at 1 cm; memory for one world | 3.0 × 10⁸; 45.6 GB |
| KI-17 | the same at 8,192 worlds | about 374 TB |
| KI-17 | one world at 5 mm (eight times the cells) | 365 GB |
| KI-17 | a spill film at 0.25 mm cells: 4,000 × 4,000 cells per layer, 16 layers per square metre | 2.56 × 10⁸ cells, 39 GB/m² |
| KI-17 | coupled-world cost c that keeps 10% of rigid-only throughput at 5% activity: 0.95 + 0.05·c ≤ 10 | c ≤ 181 |
| KI-18 | lattice Mach number Ma = √3·u·Δt/Δx at u = 1 m/s, Δt = 20 ms, Δx = 10 cm, 1 cm, 5 mm | 0.35, 3.5, 6.9 |
| KI-18 | step that holds Ma = 0.1 at 5 mm: Δt = 0.1·Δx/(√3·u); fluid steps per 20 ms tick | 0.29 ms; 70 |
| KI-18 | relaxation offset τ − ½ = 3·ν·Δt/Δx² for water (ν = 10⁻⁶ m²/s) at 10 cm cells and 20 ms steps | 6 × 10⁻⁶ |
| KI-21 | rank of the body-twist-to-wheel-rate map of a mecanum base with four parallel roller axes | 2 (an omnidirectional base has 3) |
| KI-22 | monomials of a quadratic lift of 60 states: C(62, 2); coefficients of a linear operator on them | 1,891; 3.58 × 10⁶ |

The low-Mach limit used in KI-18 is 0.3 (FACT: the sibling lattice solver). At one flow speed of 1 m/s the Mach
number at 2 mm cells is 17.3; a planning figure of 5.2 at 2 mm corresponds to 0.3 m/s and is pinned as a
discrepancy in NAB-41.

**Hand-checkable counterexamples carried by this group** (each is a permanent negative control of the analysis
named):

- KI-20: trot, bound and pronk put two or four feet down at one instant by symmetry; a rule that refuses
  simultaneous impacts removes the gaits the gait analysis exists to find. The system solves one coupled
  impulse per island and never imposes an order; for orbit derivatives a simultaneous group is split by ±δ and
  the one-sided Jacobians are compared.
- KI-21: a mecanum base assembled with all four roller axes parallel. Its growth vector is (2, 3); the missing
  direction is still reached by a Lie bracket, exactly as on a differential drive, so a test based on the
  curvature of the mechanical connection passes the defective base. The growth vector catches it.
- KI-24: contacts on different limbs couple through the base in one contact operator, so a simultaneous
  multi-foot solve does not factor along module boundaries.
- KI-25: adding a separate irreversible entropy-production term to a thermal node whose existing terms already
  are the irreversible production counts it twice.

**Keywords for the revive lint.** KI-17: fluid in every world, cloth in the batch, granular in training worlds,
full-room multiphysics. KI-18: lattice Boltzmann at control rate, LBM tiles in the batch. KI-19: symplectic
articulated integrator, energy-conserving tree step. KI-20: refuse simultaneous impacts, ordered impact
resolution. KI-21: curvature test for roller alignment, curvature preflight for mecanum. KI-22: Koopman model
predictive control, global lifted linear model. KI-23: differentiate whole rollouts, end-to-end gradients
through contact, gradient search over scenes. KI-24: run-time composition wrapper, higher-categorical module
composition. KI-25: entropy state of a thermal node, separate entropy production term.

**The entries.**

**KI-17. Full-room fluid, cloth or granular matter in every training world** (H-16, REFUTED as stated)
- *Strong form.* Fluid films, cables and cloth coupled to rigid robots inside training worlds, at 10% or more of
  rigid-only throughput when active in 5% of worlds.
- *Refutation.* DESIGN arithmetic: a D3Q19 lattice in single precision with two population buffers needs
  19 × 4 × 2 = 152 bytes per cell. A 10 × 10 × 3 m room at 1 cm is 3.0 × 10⁸ cells, 45.6 GB for one world and
  about 374 TB at 8,192 worlds; at 5 mm one world needs 365 GB, above the working-set limit of the large-memory
  machine. A spill film of 0.1 to 1 mm needs cells of 0.25 mm: 2.56 × 10⁸ cells, 39 GB per square metre of floor.
  Throughput: 0.95 + 0.05·c ≤ 10 gives c ≤ 181 rigid worlds per coupled world, and the sibling cloth solver takes
  tens of CPU seconds per drape (FACT).
- *Survives.* A contact-level film-friction law and rigid proxies [S] (§18.8, §19.10.2); whether they reproduce
  the failures of coupled models is untested and is decided by the pilot of §19.10.7. Advected films, slosh and
  articulated proxies [F]. Coupled islands [M] on the CPU referee for finalists only (§19.10.6; KILL-LIVING;
  BET-36).
- *Revive only if.* The proxy-adequacy pilot (GATE-HAB-09) shows that proxies miss failures of the coupled models (they reproduce fewer than 90% of the full-model failures, the bar of KILL-LIVING), and an island costs at most 181 rigid worlds at 5% activity: then a coupled island is funded for finalists on the CPU referee. Coupled matter in every training world is not revived while a cell costs 152 bytes.
- *Carried by.* BD-230, BD-231, BD-413, BD-414, BD-415, BD-416, N19b-20 (KILL-LIVING).

**KI-18. Lattice fluid at control rate**
- *Strong form.* Advance fluid tiles once per 50 Hz control tick inside the training batch.
- *Refutation.* DESIGN arithmetic: the lattice Mach number is Ma = √3·u·Δt/Δx. At u = 1 m/s and Δt = 20 ms it is
  0.35 at 10 cm cells, 3.5 at 1 cm and 6.9 at 5 mm, against a low-Mach limit of 0.3 (FACT: `fs-lbm`). Holding
  Ma = 0.1 at 5 mm needs Δt ≤ 0.29 ms, 70 fluid steps per control tick. The relaxation time is
  τ = 3·ν·Δt/Δx² + ½; for water at 10 cm cells and 20 ms steps τ − ½ = 6 × 10⁻⁶, where a single-relaxation scheme
  is not stable.
- *Survives.* Payload slosh as one to three sprung masses per axis [F] (§19.10.4; KILL-SLOSH). A 2.5-dimensional
  advected film field [F] (§19.10.3; KILL-ADVECT). A lattice fluid only inside an [M] island, as an offline oracle.
- *Revive only if.* None inside the training batch. A lattice fluid appears only inside a Moonshot island as an offline oracle.
- *Carried by.* BD-413, BD-414, BD-416.

**KI-19. Symplectic claims beyond a free rigid body**
- *Strong form.* The variational integrator is symplectic for the articulated robot, and its energy figures
  carry over.
- *Refutation.* The four steps of the trapezoidal scheme are derived for one rigid body. On an articulated tree
  the mass matrix depends on configuration and a kick–drift splitting is no longer the variational step.
  Dissipative contact impulses end energy conservation by design. The published energy figures are for one body.
- *Survives.* C-36 [F] in the referee and the `strict` profile, with the free body and the heavy top as oracles
  and no symplectic claim for a tree (§16.7; KILL-VARIATIONAL). Energy is audited by the ledger, not assumed
  (§16.10, §29.9).
- *Revive only if.* A derivation of the variational step for a tree with a configuration-dependent mass matrix that passes its momentum row (KILL-VARIATIONAL). Even then no symplectic claim is made where contact dissipates.
- *Carried by.* BD-087, BD-384, N16a-21, N16a-22.

**KI-20. Refusing simultaneous multi-foot impacts**
- *Strong form.* Treat simultaneous impacts as ill-posed and refuse them.
- *Refutation.* Trot, bound and pronk put two or four feet down at one instant by symmetry. A refusal removes the
  gaits that the gait analysis exists to find.
- *Survives.* One coupled impulse solve per island, never an imposed order (§16.5). For orbit derivatives a
  simultaneous group is split by ±δ and the one-sided Jacobians are compared (§22.9.4; ADR-REFLEX-05).
- *Revive only if.* None. The refusal is not reinstated; a simultaneous group is solved as one coupled impulse.
- *Carried by.* BD-082, BD-334.

**KI-21. Curvature as a roller-misalignment test**
- *Strong form.* Spanning curvature of the mechanical connection certifies, at preflight, that an
  omnidirectional base is correctly assembled.
- *Refutation.* Hand-checkable: a mecanum base assembled with all four roller axes parallel. The map from body
  twist to wheel rates drops from rank 3 to rank 2, and the growth vector is (2, 3) where an omnidirectional base
  has (3). The missing direction is still reached by a bracket, exactly as on a differential drive, so the
  curvature test passes the defective base.
- *Survives.* The growth vector of A-07 at preflight stage 4 for wheeled and tracked bases. Curvature (A-09,
  C-08) only for shape-changing kinematic locomotors (§14.11, §28.4).
- *Revive only if.* None. The parallel-roller mecanum base is a permanent negative control of the curvature analysis.
- *Carried by.* BD-106, BD-243.

**KI-22. A global lifted linear model for contact-rich predictive control**
- *Strong form.* One Koopman-style linear model in a lifted space replaces the simulator inside predictive
  control.
- *Refutation.* No finite invariant subspace survives contact switching. DESIGN arithmetic: a quadratic lift of
  60 states has C(62, 2) = 1,891 monomials, and a linear operator on them has 1,891² ≈ 3.58 × 10⁶ coefficients to
  identify.
- *Survives.* Parked (the parked, deferred or rejected table). At most one bounded experiment on a single soil regime.
- *Revive only if.* One bounded experiment on a single soil regime (GATE-REFLEX-10, KILL-SOILLIFT). Nothing wider is revived by it.
- *Carried by.* N22c-21 to N22c-26.

**KI-23. Gradients through everything**
- *Strong form.* Differentiate scenes and whole rollouts end to end and search by gradient, for a factor of 150.
- *Refutation.* The gradient of a binary failure outcome is zero almost everywhere. The proposal's own step
  counts give a factor of 2.54, not 150. Incumbents already differentiate, with bias through contact.
- *Survives.* Derivatives with validity labels on regular segments and return maps (C-16). Paired finite
  differences over short horizons (§17.13; KILL-FDGRAD). Gradients through contact are deferred to M6 (D-8).
- *Revive only if.* The M6 review of the non-goal on gradients through contact, with the step-adjoint pilot (KILL-STEPADJOINT) and paired finite differences (KILL-FDGRAD) on record. A gradient of a binary outcome is not revived.
- *Carried by.* BD-334, BD-353, BD-422, N16c-23.

**KI-24. Runtime composition wrappers around the step**
- *Strong form.* Compose robot modules at run time through higher-categorical wrappers.
- *Refutation.* Contacts on different limbs couple through the base in one contact operator, so a simultaneous
  multi-foot solve does not factor along module boundaries. A run-time wrapper also breaks the fixed data layout
  that batched kernels need.
- *Survives.* Composition over typed ports, lowered at compile time and kept to one data structure and five
  functors (C-12; §14.6, §14.9).
- *Revive only if.* None. A contact operator coupled through the base does not factor along module boundaries.
- *Carried by.* BD-060, BD-061.

**KI-25. The entropy balance as printed, with entropy as the state of a lumped thermal node**
- *Strong form.* Carry entropy as the state of each thermal node and add a separate irreversible production
  term.
- *Refutation.* The terms already present are the irreversible production, so the added term counts it twice.
- *Survives.* One thermal node with temperature as its state. Stored heat is a store and not a loss on the
  ledger (C-38; §15.5, §15.11).
- *Revive only if.* None. The correction is arithmetic: the added term counts the irreversible production twice.
- *Carried by.* BD-071, N15a-06, SPEC-C-38.

## Acceptance criteria

- [ ] Records `KI-17` … `KI-25` exist with the three paragraphs verbatim, revival conditions, carriers and
      keywords.
- [ ] The harness recomputes the eleven calculation rows.
- [ ] Each of the four counterexamples names among its record's carriers the bead that owns it as a negative
      control of its analysis: KI-20 the impulse solve (BD-082), KI-21 the growth-vector analysis (BD-106),
      KI-24 the module compiler (BD-060, BD-061), KI-25 the thermal node (N15a-06). Registering each control
      and its fixture is that bead's (they land at M1, after this bead).
- [ ] The revive lint fails a planted bead "Put fluid in every world with LBM tiles in the batch" with
      `ki.match-without-citation` naming KI-17 and KI-18 (it holds a keyword of each), and passes the epic for
      coupled islands on the CPU referee (BD-416), which cites both and their kill gate.
- [ ] KI-23's record states that gradients through contact are deferred to M6 and names the two surviving
      forms (derivatives with validity labels on regular segments and return maps; paired finite differences
      over short horizons).

## Test plan

### Unit tests

`crates/frob-conformance/tests/killed_ideas.rs`, module `g3_worlds`:

- `ki17_lattice_memory`: 152 B; 1,000 × 1,000 × 300 = 3.0 × 10⁸; 3.0 × 10⁸ × 152 = 45.6 GB; × 8,192 = 373.6 TB;
  at 5 mm 2.4 × 10⁹ cells = 364.8 GB; film 2.56 × 10⁸ × 152 = 38.9 GB.
- `ki17_coupled_cost`: (10 − 0.95)/0.05 = 181.
- `ki18_mach`: √3 × 0.02/0.1 = 0.3464; /0.01 = 3.464; /0.005 = 6.928; 0.1 × 0.005/√3 = 2.887 × 10⁻⁴ s;
  0.02/2.887 × 10⁻⁴ = 69.3, rounded up to 70.
- `ki18_relaxation`: 3 × 10⁻⁶ × 0.02/0.01 = 6 × 10⁻⁶.
- `ki21_rank_and_growth_vector`: the wheel-rate map of four mecanum wheels with parallel rollers has rank 2
  (computed from the 4 × 3 matrix with identical roller directions); with the standard ±45° pattern it has
  rank 3.
- `ki22_lift_size`: C(62, 2) = 1,891; 1,891² = 3,575,881.
- `records_complete_and_verbatim` against `tests/fixtures/killed/g3.md`.
- `ki17_revive_cases`: the planted bead of the criteria fails naming both records; BD-416's title and
  description pass.
- `revival_fields`: `revive_evidence` and `revive_gates` per record as listed.

### End-to-end script and logging

`scripts/e2e/decision_records.sh` (NAB-50) runs this module with the rest of `killed_ideas` and recomputes the
calculations. Logged per STD-E2E.

### Known answers and negative controls

Known answers: the table. Negative controls: a KI-17 record that gives 15.2 bytes per cell; a KI-18 record
that reports Ma at 5 mm as 0.69 (a decimal slip); the curvature test wired in as the roller-alignment
preflight (the parallel-roller base must then pass it, which is the failure the record documents); a proposal
"Build Koopman model predictive control on a lifted-linear model for the hybrid system" (must match KI-22 and
KI-65: it holds a keyword of each). Metamorphic check for
`ki18_mach`: halving Δx doubles Ma.

## Considerations

- KI-17's surviving proxies are untested against coupled models; whether they reproduce the failures of
  coupled matter is decided by the proxy-adequacy pilot. If they do, the coupled island is never funded; that
  kill gate fires on *success* of the cheap path.
- KI-22 has a narrow, later-specified experiment: a lifted linear planning model for one tracked base on one
  soil card, with a kill gate. It does not reopen the global form.
- KI-19: energy is audited by the energy ledger, never assumed from an integrator's name.

## References

Appendix C.2.3; §0.8 row H-16; §16.5; §16.7; §19.10; constructions C-08, C-12, C-36, C-38.
