---
key: "NAB-19"
title: "registries/ev_matrix.toml: the 59 Frontier and Moonshot families, each with pilot and kill gate"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "delivery", "gauntlet", "frontier", "moonshot", "registry", "decision-record"]
parent: "EPIC-M0"
deps: ["N40-23", "N40-27", "NAB-17"]
needs: []
related: ["NAB-20", "NAB-22"]
gates: []
plan_refs: ["Appendix C.1.2", "Appendix C.1", "§0.1", "§9 (D-6, D-10)"]
---
## Background

FrankenRobotics calibrates ambition with three tiers. **Solid [S]** is established mathematics and
engineering. **Frontier [F]** is published research that mainstream robot simulators have not productized, or a
narrowed form of a mechanism whose strong form failed review. **Moonshot [M]** is a synthesis proposed by this
project; it lives behind a feature flag and never sits on the critical path. Two obligations attach to the
upper tiers: every [F] and [M] item names a paper-scale pilot with a falsifier that must run before its work
package is funded and states the ceiling of its benefit before any speed test; and every [F] and [M] item has
a kill gate, a trigger written in advance with a fallback of a lower tier that the system already contains, so
that losing the item costs a capability and never the critical path.

This bead carries the second table of the expected-value matrix (`registries/ev_matrix.toml`, schema
`frob.evmatrix/1`, NAB-17): the 59 families adopted or admitted at the upper tiers. Score = (Impact ×
Confidence) ÷ Effort, each from 1 to 5, printed to one decimal; the scores are DESIGN judgments. **"Adopted"**
means the work package is funded once the pilot has run. **"Admitted"** means the item is in the catalogue and
waits its turn under the Moonshot cap (at most four [M] items active at once; the queue is NAB-21). Rows are
sorted by score, highest first.

Sixteen rows score below 2.0. The rule of the matrix is that a family below 2.0 is rejected, parked or
deferred unless something else needs it, with one exception: a Moonshot may be admitted below 2.0, behind a
flag and off the critical path. Each of the sixteen is one or the other, and the lint checks it.

## Scope

In: 59 `[[family]]` records, ids `EV-F01` to `EV-F59`, table `frontier-moonshot`. Out: the schema and lints
(NAB-17); the other tables (NAB-18, NAB-20); the queue order (NAB-21); the pilots and kill decisions themselves
(their own beads, in the units that own the mechanisms).

## Technical approach

**Transcription rule.** One record per row, in table order, as in NAB-18. `decision` is `adopted` where the
decision text begins "Adopted" or names a tier without "Admitted" (rows that read "[S] as …; [F] as …" are
adopted at both tiers, larger part first); `admitted` where it begins "Admitted". `tiers` lists every tier the
decision names, the larger part (the part of this table) first, so the five rows that name [S] and [F]
(EV-F03, EV-F11, EV-F14, EV-F37, EV-F39) have `["F", "S"]` whatever the order of their text, and EV-F24, whose
text names no tier, has `["F"]`. `carried = true` for EV-F04, EV-F37 and EV-F40 only (NAB-17 rule 2). `queue`
lists the Moonshot-queue items whose `family` names the row, entered from the mapping of NAB-21 step 8 (which
assigns `MQ-01` … `MQ-21`): each of the twelve admitted rows has one, except skill transport (EV-F45), which
has two (MQ-16 and MQ-17); three adopted rows whose [M] part waits in the queue have one though their text does
not say "queued" (EV-F46 → MQ-10, EV-F55 → MQ-09, EV-F56 → MQ-11). Until NAB-21's records land these lists are
a recorded skip of NAB-17's queue check (NAB-17 step 6). `needed_by` is filled for the two adopted rows below
2.0 that are not Moonshots:

- the tree-operator route for mechanisms above 64 degrees of freedom (1.5): needed because hybrid robots above
  64 degrees of freedom cannot use the dense factor; KILL-TREEOP keeps the dense factor and the cap;
- kernel reflection (1.0): needed because it is the one route from an instance certificate to evidence class
  Proved; KILL-REFLECT parks it.

**Notes an implementer needs that the one-line decisions compress.**

- *Ceilings stated first.* Derived kernels with dimension typing are adopted for correctness, derivatives and
  certificates, and as a throughput lever are capped near 1.2 (KILL-KIR). Design-family lanes are adopted under
  the bound 1/(1 − f + f/r + h), where f is the fraction of step time in the stages a family lane shares, r the
  reduction of that work, and h the added overhead as a fraction of the step; the ceiling is 1.10 to 1.32, stated
  before any measurement (KILL-FAMILYLANE). The geometric-algebra tree policy has a ceiling of 1.3.
- *Conditions on funding.* Family policies need an owner-approved compute envelope. Coupled multiphysics
  islands are funded only if the proxy-adequacy pilot shows that proxies miss failures. Mission queries have
  their first slice on the CPU at M1.
- *Narrowings.* The connection-curvature oracle is narrowed to kinematic locomotors and shape-changers, is not a
  wheel-layout test (KI-21), is among the first items to be cut, and was rescored from 3.0 to 2.0
  (`rescored_from = "3.0"`). The sampling controller is a fallback and a source of demonstrations and never a
  stand-in evaluator for a trained policy. Calibration against reality is an import lane that is mostly
  `no-claim`. Topology optimization, fatigue, tolerance allocation and the build packet are adopted for output
  quality and are not counted in the ten-fold thesis.
- *Split tiers.* Coupled fidelity estimators are [S] as mathematics and [F] across contact-rich tiers. Campaign
  quotienting is [S] for orbit enumeration and counting and [F] for wreath products and the projection
  quotient. The hard-contact law CL-H is [S] as the referee's written specification and [F] as a batch law. The
  trapezoidal variational step is [F] at M5 in the referee and the `strict` profile, while its nonholonomic
  rows are [S]. Symmetry at the instant of contact is [F] as a test and [M] as an amortization; the shape-space
  atlas is [F] on kinematic fixtures and [M] as an atlas; label minimization is [F] on supplied sets and [M]
  from contact.
- *Rows without a kill gate* (four): max-plus gait timing (adopted [F]; its test is a hand-computed stride
  period); the connection-curvature oracle (decided with the geometric-phase epic's kill gate); Study-quadric
  mechanism synthesis (queued behind the continuation lane and piloted with it); tactile hands (fallback: one
  patch per pad).

**The rows.**

| Id | Family | Where | I | C | E | Score | Decision |
|---|---|---|---|---|---|---|---|
| EV-F01 | Gymnasium-compatible adapter | §23.18; GATE-CORTEX-16 | 3 | 4 | 1 | 12.0 | Adopted [F]; KILL-GYMADAPT |
| EV-F02 | Max-plus gait timing | C-10; §22.9 | 3 | 4 | 1 | 12.0 | Adopted [F] |
| EV-F03 | Monotone co-design: closure analysis and catalog lane | C-12; A-10, §28.4; §25.21.1 | 4 | 4 | 2 | 8.0 | Adopted: the closure analysis A-10 runs in `design check` [S]; Pareto antichains over catalogs are [F]; KILL-SYNTH-CODESIGN |
| EV-F04 | Composition over typed ports (the wiring diagram of version 0.2) as a thin data structure | C-12; §14.6 | 3 | 4 | 2 | 6.0 | Adopted [F], kept thin: one data structure and five functors; KILL-SKEL-SUBST |
| EV-F05 | Inheritance of a policy along design edits | C-23; §23.12; GATE-CORTEX-13 | 4 | 3 | 2 | 6.0 | Adopted [F] for parameter edits by default, with scheduled resets; KILL-INHERIT |
| EV-F06 | Sampled background referee with a reported coverage figure | §16.11; GATE-FID-07 | 3 | 4 | 2 | 6.0 | Adopted [F]; KILL-SHADOW |
| EV-F07 | Baseline controller synthesis per morphology class | C-13; §22.4; BET-06 | 5 | 3 | 3 | 5.0 | Adopted [F] across morphologies; KILL-BASELINE routes failing classes to training |
| EV-F08 | Rank agreement between rungs, the ladder allocation and conformal survivor sets | C-26; §16.9, §28.9, §28.10 | 5 | 3 | 3 | 5.0 | Adopted [F]; the decisive early experiment; KILL-RANK, KILL-LADDER |
| EV-F09 | Gait atlas from symmetry | C-10; §22.9; A-04 | 3 | 3 | 2 | 4.5 | Adopted [F]; KILL-GAIT |
| EV-F10 | Assumption lattice and verdict polarity | C-18; §25.8, §28.13 | 3 | 3 | 2 | 4.5 | Adopted [F]; KILL-AUGUR-REUSE; fallback, recompute from the dependency closure |
| EV-F11 | Campaign quotienting by symmetry orbits and mission observables | C-22; §17.11, §28.15 | 3 | 3 | 2 | 4.5 | [S] for orbit enumeration and counting; [F] for wreath products and the projection quotient; KILL-QUOTIENT, KILL-AUGUR-CUTS |
| EV-F12 | Retrofit catalog: priced edits of the world as repairs | §19.12; GATE-HAB-11 | 3 | 3 | 2 | 4.5 | Adopted [F]; KILL-RETROFIT |
| EV-F13 | Division of labour between the two machines, and the fleet scheduler | §21.17; GATE-SPARK-13 | 3 | 3 | 2 | 4.5 | Adopted [F]; KILL-FLEET |
| EV-F14 | Coupled fidelity estimators: control variate, multilevel, doubly robust correction | C-25; §16.8, §29.11 | 4 | 3 | 3 | 4.0 | [S] as mathematics; [F] across contact-rich tiers; KILL-DR |
| EV-F15 | Minimal counterexample worlds | C-28; §26.7; `.fmcw` | 4 | 3 | 3 | 4.0 | Adopted [F]; KILL-MCW |
| EV-F16 | Energy-budget safety tier and runtime shields | C-31; §22.11, §22.12 | 4 | 3 | 3 | 4.0 | Adopted [F], class Tested; KILL-ENERGY, KILL-SHIELD |
| EV-F17 | Clearance complexes certified from both sides | C-33; §19.11; A-18 | 4 | 3 | 3 | 4.0 | Adopted [F]; KILL-FCLEAR |
| EV-F18 | Observation-alias impossibility witnesses, label counts and schedule classes | C-19; §20.12, §25.10, §25.18 | 4 | 3 | 3 | 4.0 | Adopted [F]; KILL-ALIAS, KILL-AUGUR-ALIAS, KILL-SKEL-ALIAS |
| EV-F19 | Family certificates in the `.frobcert` bundle | §12.2, §12.4, §25.5 | 4 | 3 | 3 | 4.0 | Adopted [F]; KILL-FAMILYCERT, KILL-AUGUR-FALSECERT |
| EV-F20 | Sampling controller as a fallback and a source of demonstrations | §22.8; BET-27 | 3 | 4 | 3 | 4.0 | Adopted [F]; never a stand-in evaluator for a trained policy; KILL-MPC |
| EV-F21 | Mission queries with inner and outer design sets | C-17; §25.3 to §25.5 | 5 | 3 | 4 | 3.8 | Adopted [F]; first slice on the CPU at M1; KILL-AUGUR-TIE, KILL-FAMILYCERT |
| EV-F22 | Family policies per morphology class | C-23; §23.11; GATE-CORTEX-11 | 5 | 3 | 4 | 3.8 | Adopted [F]; KILL-FAMILY; needs an owner-approved compute envelope |
| EV-F23 | Mixed-topology model variants in padded size buckets | §17.4, §17.5, §21.11; BET-18 | 5 | 3 | 4 | 3.8 | Adopted [F]; KILL-PHYLUM, KILL-BUCKET |
| EV-F24 | Slot-level episode dispatch in the daemon | §21.11, §32.11; DC-09 | 5 | 3 | 4 | 3.8 | Adopted; KILL-SLOT falls back to one environment per job |
| EV-F25 | Equivariant networks under the observation-preserving subgroup | C-09; §13.5, §23.13 | 3 | 3 | 3 | 3.0 | Adopted [F]; KILL-EQUIV, KILL-MARROW-BLOCKS |
| EV-F26 | Derived kernels with dimension typing | C-14; §21.6, §21.7 | 4 | 3 | 4 | 3.0 | Adopted [F] for correctness, derivatives and certificates; as throughput capped near 1.2; KILL-KIR |
| EV-F27 | Instance certificates | C-14; §12.4 | 3 | 3 | 3 | 3.0 | Adopted [F]; KILL-LEMMA |
| EV-F28 | Hybrid gait orbits: saltation, return maps, labeled derivatives, continuation in design parameters | C-16; §16, §22.9; A-04 | 3 | 3 | 3 | 3.0 | Adopted [F], labeled screening; KILL-GAIT |
| EV-F29 | Region certificates and dual witnesses of infeasibility | C-32; §25.6 | 3 | 3 | 3 | 3.0 | Adopted [F] on sub-boxes of about six parameters or fewer; KILL-AUGUR-REGION |
| EV-F30 | Bounded exact linkage synthesis | C-34, lane 1; §25.21.3 | 3 | 3 | 3 | 3.0 | Adopted [F]; KILL-SYNTH-LINKAGE |
| EV-F31 | Trajectory optimization with design variables, as local refinement | §25.21.2; GATE-AUGUR-18 | 3 | 3 | 3 | 3.0 | Adopted [F]; KILL-SYNTH-TRAJOPT |
| EV-F32 | Calibration against reality, and conformal bands per calibrated cell | §30.6 to §30.13; DC-07 | 4 | 3 | 4 | 3.0 | Adopted [F] as an import lane that is mostly `no-claim`; KILL-GROUND, KILL-BAND |
| EV-F33 | Paired finite-difference design gradients over short horizons | §17.13, §28.8 | 3 | 2 | 2 | 3.0 | Adopted [F]; KILL-FDGRAD |
| EV-F34 | Design-family lanes under the bound 1/(1 − f + f/r + h) | §17.6, §36.5 | 2 | 3 | 2 | 3.0 | Adopted [F], with a ceiling of 1.10 to 1.32 stated first; KILL-FAMILYLANE |
| EV-F35 | Payload slosh by an equivalent mechanical model | §19.10.4 | 2 | 3 | 2 | 3.0 | Adopted [F]; KILL-SLOSH |
| EV-F36 | Articulated cord and rug proxies | §19.10.5 | 2 | 3 | 2 | 3.0 | Adopted [F]; KILL-PROXY |
| EV-F37 | Trapezoidal variational step (the variational articulated integrator of version 0.2) | C-36, C-05; §16.7 | 3 | 3 | 4 | 2.3 | Adopted [F] at M5 in the referee and the `strict` profile; KILL-VARIATIONAL; the nonholonomic rows of C-36 are [S] |
| EV-F38 | Topology optimization, fatigue, tolerance allocation and the build packet | C-35; §31.5 to §31.12 | 3 | 3 | 4 | 2.3 | Adopted [F] for output quality, not counted in the thesis of §2; KILL-TOPOPT, KILL-LOADS, KILL-TOLERANCE, KILL-PACKET |
| EV-F39 | Hard-contact batch law CL-H | §18.5; GATE-TRAC-09 | 3 | 3 | 4 | 2.3 | [S] as the referee's written specification; [F] as a batch law; KILL-HARDCONTACT |
| EV-F40 | Connection-curvature oracle | C-08; A-09, §28.4; §25.19 | 2 | 3 | 3 | 2.0 | Adopted [F], narrowed to kinematic locomotors and shape-changers; not a wheel-layout test (KI-21); among the first items to be cut; rescored from 3.0 |
| EV-F41 | Dominance as unreachability in a product system | C-40; §25.16 | 3 | 2 | 3 | 2.0 | Adopted [F]; KILL-AUGUR-DOMINANCE |
| EV-F42 | Robot-advected film fields | §19.10.3 | 3 | 2 | 3 | 2.0 | Adopted [F] at M6; KILL-ADVECT |
| EV-F43 | Rotor port for flight | §14.10, §15.4 | 2 | 3 | 3 | 2.0 | Admitted [M]; KILL-SINEW-ROTOR; queued (the Moonshot queue) |
| EV-F44 | Tree-operator route for mechanisms above 64 degrees of freedom | §16.6 | 3 | 2 | 4 | 1.5 | Adopted [F] because hybrids above 64 degrees of freedom need it; KILL-TREEOP keeps the dense factor and the cap |
| EV-F45 | Skill transport: through a physical interface, and by a learned residual | §22.10, §25.11; GATE-REFLEX-09, GATE-AUGUR-23 | 3 | 2 | 4 | 1.5 | Admitted [M]; KILL-SKILL, KILL-AUGUR-TRANSPORT; queued |
| EV-F46 | Symmetry at the instant of contact, amortized per family | §25.20; GATE-AUGUR-21 | 2 | 2 | 3 | 1.3 | [F] as a test; [M] as an amortization; KILL-AUGUR-SYMEVENT |
| EV-F47 | Reachable-set tables for small subsystems | §22.12.5; GATE-SHIELD-05 | 2 | 2 | 3 | 1.3 | Admitted [M]; KILL-TUBE; queued |
| EV-F48 | Kernel reflection | §12.4; T-35; GATE-LEMMA-05 | 2 | 2 | 4 | 1.0 | [F] pilot at M5, kept because it is the one route from an instance certificate to class Proved; KILL-REFLECT parks it in the parked, deferred or rejected table |
| EV-F49 | Geometric-algebra policy on the kinematic tree | C-24; §23.14; GATE-CORTEX-17 | 2 | 2 | 4 | 1.0 | Admitted [M], with a ceiling of 1.3 stated first; KILL-GATREE; queued |
| EV-F50 | Numerical continuation for linkage synthesis, on the CPU | C-34, lane 2; §25.21.4 | 2 | 2 | 4 | 1.0 | Admitted [M]; KILL-SYNTH-CONT; queued |
| EV-F51 | Study-quadric mechanism synthesis | C-11; piloted with C-34 | 2 | 2 | 4 | 1.0 | Admitted [M]; queued behind the continuation lane |
| EV-F52 | Tactile hands: micro-patch contact pads | §14.10, §18.17 | 2 | 2 | 4 | 1.0 | Admitted [M]; fallback, one patch per pad; queued |
| EV-F53 | Firmware export of fixed-point controllers | §31.13; GATE-FOUNDRY-11 | 2 | 2 | 4 | 1.0 | Admitted [M]; KILL-FIRMWARE; queued |
| EV-F54 | Lifecycle compression | §31.14; GATE-FOUNDRY-12 | 2 | 2 | 4 | 1.0 | Admitted [M]; KILL-LIFECYCLE; queued |
| EV-F55 | Shape-space atlas for locomotion by geometric phase | §25.19; GATE-AUGUR-20 | 2 | 2 | 4 | 1.0 | [F] on kinematic fixtures; the atlas is [M]; KILL-AUGUR-PHASE |
| EV-F56 | Label minimization from contact for a shared controller | §25.18; GATE-AUGUR-19 | 2 | 2 | 4 | 1.0 | [F] on supplied sets; [M] from contact; KILL-AUGUR-LABELS |
| EV-F57 | Fused per-world step kernel | §21.18 | 3 | 1 | 3 | 1.0 | Admitted [M]; the sign of the benefit is unproven; KILL-MEGA; queued |
| EV-F58 | Coupled multiphysics islands: free-surface fluid, implicit cloth, Cosserat rod, on the CPU referee for finalists | §19.10.6, §19.10.7; GATE-HAB-09 | 2 | 2 | 5 | 0.8 | Admitted [M]; funded only if the proxy-adequacy pilot shows that proxies miss failures; KILL-LIVING |
| EV-F59 | Facility co-design and multi-robot handoff | §25.17; GATE-AUGUR-24 | 3 | 1 | 5 | 0.6 | Admitted [M]; KILL-AUGUR-FACILITY; queued |

Rows in this table: 59.

## Acceptance criteria

- [ ] 59 records `EV-F01` … `EV-F59` exist in table `frontier-moonshot`, in the order above, and the loader and
      lints of NAB-17 pass on them.
- [ ] Every printed score equals the derived score; the first two rows score 12.0 and the last scores 0.6.
- [ ] Exactly 16 rows are below 2.0; 14 of them have `"M"` in `tiers` and 2 have a non-empty `needed_by`.
- [ ] 55 rows name at least one kill gate, each resolving in the gate register or, while the exit record of its
      ledger group's milestone does not exist, on NAB-17's frozen list `tests/fixtures/ev/kill_gates.tsv` with a
      recorded `skip` (NAB-17 step 6). The 4 rows without one are EV-F02, EV-F40, EV-F51 and EV-F52, with what
      stands in for a kill gate as listed above.
- [ ] The twelve admitted rows and the three adopted rows with a queued [M] part list the items of NAB-21's
      step 8 mapping: fifteen rows list sixteen items, and every other row has `queue = []`. (That those ids
      exist and name the row in their `family`, both ways, is NAB-21's `ev.queue-mismatch` criterion; NAB-21
      comes after this bead.)
- (Not this bead's close condition: that each upper-tier mechanism has a pilot row is the completeness check
  of the pilot register, N38a-09, whose inputs are the capability records, kernel manifests and verb rows,
  filled by the pilot beads of the owning units.)

## Test plan

### Unit tests

`crates/frob-conformance/tests/ev_matrix.rs`, module `frontier_moonshot`:

- `count_is_59`; `order_matches_frozen_list`.
- `scores`: spot values: baseline controller synthesis (5, 3, 3) → "5.0"; mission queries (5, 3, 4) → "3.8";
  the trapezoidal variational step (3, 3, 4) → "2.3"; coupled islands (2, 2, 5) → "0.8"; facility co-design
  (3, 1, 5) → "0.6"; the fused per-world step kernel (3, 1, 3) → "1.0".
- `sixteen_below_two`: the rows below 2.0 are EV-F44 to EV-F59; 14 have `"M"` in `tiers` and 2 (EV-F44,
  EV-F48) a non-empty `needed_by`.
- `kill_gate_rows_are_55`; `rows_without_kill_gate_are_four` (EV-F02, EV-F40, EV-F51, EV-F52).
- `tiers`: `["F", "S"]` for EV-F03, EV-F11, EV-F14, EV-F37, EV-F39; `["F", "M"]` for EV-F46, EV-F55, EV-F56;
  `["F"]` for EV-F24.
- `admitted_rows_are_queued`: the twelve `decision = "admitted"` rows (rotor port, skill transport, reachable-set
  tables, the geometric-algebra tree policy, numerical continuation, Study-quadric synthesis, tactile hands,
  firmware export, lifecycle compression, the fused step kernel, coupled islands, facility co-design) each have
  a non-empty `queue`.
- `queue_lists`: EV-F43 → [MQ-03]; EV-F45 → [MQ-16, MQ-17]; EV-F46 → [MQ-10]; EV-F47 → [MQ-18]; EV-F49 →
  [MQ-19]; EV-F50 → [MQ-14]; EV-F51 → [MQ-15]; EV-F52 → [MQ-04]; EV-F53 → [MQ-13]; EV-F54 → [MQ-12]; EV-F55 →
  [MQ-09]; EV-F56 → [MQ-11]; EV-F57 → [MQ-01]; EV-F58 → [MQ-02]; EV-F59 → [MQ-20] (NAB-21 step 8); every
  other row has `queue = []`.
- `rescored_row`: the connection-curvature oracle has `carried = true`, `rescored_from = "3.0"`, score "2.0".
- `split_tier_rows_sit_in_table_of_larger_part`.
- `decision_text_frozen` against `tests/fixtures/ev/frontier_moonshot.tsv` (the columns of NAB-17 rule 2), for
  every row whose `history` is empty.

### End-to-end script and logging

`scripts/e2e/decision_records.sh` (NAB-50) asserts the count (59), zero mismatches, 16 rows below 2.0 with
their justification, and the queue cross-check, logged as `metric` and `assert` lines per STD-E2E.

### Known answers and negative controls

Known answers: 59 rows; 16 below 2.0; 55 with kill gates. Deciding milestones of this table's kill gates (from
the gate register): M1: KILL-SKEL-SUBST, and KILL-AUGUR-FALSECERT standing from M1; M2: KILL-BUCKET,
KILL-PHYLUM, KILL-KIR; M3: KILL-GYMADAPT, KILL-SHADOW, KILL-BASELINE, KILL-QUOTIENT, KILL-DR, KILL-MCW,
KILL-FCLEAR, KILL-ALIAS, KILL-AUGUR-ALIAS, KILL-SKEL-ALIAS, KILL-FAMILYCERT, KILL-AUGUR-TIE, KILL-SLOT,
KILL-AUGUR-REGION, and KILL-AUGUR-REUSE standing from M3; M4: KILL-SYNTH-CODESIGN, KILL-INHERIT, KILL-LADDER,
KILL-RANK, KILL-GAIT, KILL-AUGUR-CUTS, KILL-RETROFIT, KILL-FLEET, KILL-ENERGY, KILL-SHIELD, KILL-MPC,
KILL-FAMILY, KILL-EQUIV, KILL-MARROW-BLOCKS, KILL-SYNTH-LINKAGE, KILL-SYNTH-TRAJOPT, KILL-FDGRAD,
KILL-FAMILYLANE, KILL-AUGUR-DOMINANCE, KILL-AUGUR-LABELS; M5: KILL-LEMMA, KILL-BAND, KILL-GROUND,
KILL-VARIATIONAL, KILL-LOADS, KILL-PACKET, KILL-TOLERANCE, KILL-TOPOPT, KILL-HARDCONTACT, KILL-TREEOP,
KILL-REFLECT; M6: KILL-SLOSH, KILL-PROXY, KILL-ADVECT, KILL-SINEW-ROTOR, KILL-AUGUR-SYMEVENT, KILL-LIFECYCLE,
KILL-AUGUR-PHASE, KILL-LIVING; M8: KILL-AUGUR-TRANSPORT, KILL-SKILL, KILL-TUBE, KILL-GATREE, KILL-SYNTH-CONT,
KILL-FIRMWARE, KILL-MEGA, KILL-AUGUR-FACILITY. Negative controls: an [F]-only row at 1.5 with no `needed_by`
(`ev.below-two-unjustified`); an "admitted" row with no queue entry (`ev.queue-mismatch`); a row whose kill gate is renamed to a name in neither the gate register nor the
frozen list (`ev.kill-gate-unknown` at once; a name of the frozen list missing from the register is only a
skip until its group's exit).

## Considerations

- A score below 2.0 with an [M] tier is permission to wait, not permission to build. Nothing in this table is
  funded before its pilot has a recorded result.
- When a kill gate fires, the family is described by its fallback's name everywhere, the row gains a history
  entry, and (for kernel reflection, by the rule of the matrix) the row moves to the parked table.
- The table is where the program is most likely to overreach. The sweeps (NAB-23 to NAB-31) re-read it at every
  milestone exit against the kill decisions that exit recorded.

## References

Appendix C.1.2; Appendix C.1 (rules); §0.1 (tiers and the two obligations); doctrine rules D-6 and D-10.
