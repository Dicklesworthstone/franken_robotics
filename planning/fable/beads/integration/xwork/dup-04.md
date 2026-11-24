# Cross-group overlap packet dup-04

36 pairs in 21 clusters. Each cluster is a set of beads connected by reported overlaps; decide a cluster as a whole.

## Cluster of 8 beads

- `BD-037` (U18a, M0, task) upstream: `registries/upstream.toml` and an `upstream` bead per sibling work item, mirrored in sibling trackers  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U18a/BD-037.md`
- `N33c-47` (U33c, M0, task) upstream register: rows for the late foundation facts (FD-FSIM-31 to -33, FD-FTX-01, FD-F3D-01, NF-07 to -12)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U33c/N33c-47.md`
- `NF-07` (UF, M1, task) upstream frankensim: G1 catalog builder folds or declares the fixed attachments it omits (1.377 of 33.34 kg)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/NF-07.md`
- `NF-08` (UF, M1, task) upstream frankensim: `fs_ga::Se3` Jacobians in closed form for large translation (no refusal at 33 m)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/NF-08.md`
- `NF-09` (UF, M1, task) upstream frankensim: the legacy `fs_phs::transformer` documented or removed (opposite sign convention)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/NF-09.md`
- `NF-10` (UF, M2, task) upstream frankensim: an arc-aware swept bound beside `fs_contact::broad_phase::swept_sphere_aabb`  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/NF-10.md`
- `NF-11` (UF, M1, task) upstream frankensim: `fs-sos` keeps authored coefficients and rounds its certified bound outward  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/NF-11.md`
- `NF-12` (UF, M2, task) upstream franken_threed: a ray transform that returns the scale of its parameter (`Ray::apply_matrix4`)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/NF-12.md`

- **pair** `BD-037` / `N33c-47`
  - reported by G33: register rows for the eleven late upstream items against the registry bead and the upstream beads NF-07 to NF-12, FD-FSIM-31 to -33, FD-FTX-01, FD-F3D-01; N33c-47 is re-scoped to the rows and their failing fixtures
- **pair** `N33c-47` / `NF-07`
  - reported by GF: both write the item's row in registries/upstream.toml and open its mirrored bead; N33c-47 assigns the identifier (FD-FSIM-34 is proposed in NF-07), and NF-07 now depends on it
- **pair** `N33c-47` / `NF-08`
  - reported by GF: both write the item's row in registries/upstream.toml and open its mirrored bead; N33c-47 assigns the identifier (FD-FSIM-35 is proposed in NF-08), and NF-08 now depends on it
- **pair** `N33c-47` / `NF-09`
  - reported by GF: both write the item's row in registries/upstream.toml and open its mirrored bead; N33c-47 assigns the identifier (FD-FSIM-36 is proposed in NF-09), and NF-09 now depends on it
- **pair** `N33c-47` / `NF-10`
  - reported by GF: both write the item's row in registries/upstream.toml and open its mirrored bead; N33c-47 assigns the identifier (FD-FSIM-37 is proposed in NF-10), and NF-10 now depends on it
- **pair** `N33c-47` / `NF-11`
  - reported by GF: both write the item's row in registries/upstream.toml and open its mirrored bead; N33c-47 assigns the identifier (FD-FSIM-38 is proposed in NF-11), and NF-11 now depends on it
- **pair** `N33c-47` / `NF-12`
  - reported by GF: both write the item's row in registries/upstream.toml and open its mirrored bead; N33c-47 assigns the identifier (FD-F3D-02 is proposed in NF-12), and NF-12 now depends on it

## Cluster of 5 beads

- `N39b-28` (U39b, M3, task) Owner decision OPEN-TENFOLD-3: who the rival advocates are and their tuning budget  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U39b/N39b-28.md`
- `N39b-29` (U39b, M3, task) Owner decision OPEN-TENFOLD-4: the compute envelope of the partial gate and tournament, with cost per cell  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U39b/N39b-29.md`
- `N39b-31` (U39b, M1, task) Owner decision OPEN-TENFOLD-6: the common agent of the ten-fold test: model, revision, token and tool-call caps  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U39b/N39b-31.md`
- `N39b-33` (U39b, M3, task) Owner decision OPEN-TENFOLD-8: ratify thresholds 3 and 10, guard floor 5, tie 1.5, margin 0.05, four cells  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U39b/N39b-33.md`
- `N41-15` (U41, M2, task) Owner decision OPEN-TENFOLD-3, -4, -6, -8: rival advocates, compute envelope, agent model, ratified thresholds  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U41/N41-15.md`

- **pair** `N39b-28` / `N41-15`
  - reported by G39: both decide OPEN-TENFOLD-3, the rival advocates and their budget
  - reported by G41: both decide OPEN-TENFOLD-3 (rival advocates and their budget)
- **pair** `N39b-29` / `N41-15`
  - reported by G39: both decide OPEN-TENFOLD-4, the compute envelope of the gate and the tournament
  - reported by G41: both decide OPEN-TENFOLD-4 (the compute envelope of the partial gate and the tournament)
- **pair** `N39b-31` / `N41-15`
  - reported by G39: both decide OPEN-TENFOLD-6, the common agent and its caps
  - reported by G41: both decide OPEN-TENFOLD-6 (the common agent: model, revision, caps)
- **pair** `N39b-33` / `N41-15`
  - reported by G39: both ratify the thresholds of OPEN-TENFOLD-8
  - reported by G41: both decide OPEN-TENFOLD-8 (ratified thresholds)

## Cluster of 4 beads

- `BET-29` (U00, M5, task) Decide BET-29: generative structure (optimized links at least 7% lighter than engineered parts at equal margin)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U00/BET-29.md`
- `N31b-15` (U31b, M5, task) Kill-gate decision KILL-TOPOPT: keep three-dimensional link optimization, or beam-rung sizing only  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U31b/N31b-15.md`
- `N31b-18` (U31b, M5, task) Kill-gate decision KILL-TOLERANCE: keep sensitivity-driven allocation, or worst-case boxes per interface  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U31b/N31b-18.md`
- `N31b-19` (U31b, M5, task) Kill-gate decision KILL-PACKET: keep the full build packet, or reduce it to parts list, mass properties and STL  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U31b/N31b-19.md`

- **pair** `BET-29` / `N31b-15`
  - reported by G00: overlap with distinct scope, no merge: both record the decision KILL-TOPOPT. The kill-gate bead applies the trigger and writes the kill ADR; the bet bead reads that record, evaluates the bet's falsifier clauses and writes the bet verdict. The bet bead's acceptance line "the KILL-TOPOPT decision recorded" means on record by the other bead. The dependency of the bet on this bead is in place.
- **pair** `BET-29` / `N31b-18`
  - reported by G00: overlap with distinct scope, no merge: both record the decision KILL-TOLERANCE. The kill-gate bead applies the trigger and writes the kill ADR; the bet bead reads that record, evaluates the bet's falsifier clauses and writes the bet verdict. The bet bead's acceptance line "the KILL-TOLERANCE decision recorded" means on record by the other bead. The dependency of the bet on this bead is in place.
- **pair** `BET-29` / `N31b-19`
  - reported by G00: overlap with distinct scope, no merge: both record the decision KILL-PACKET. The kill-gate bead applies the trigger and writes the kill ADR; the bet bead reads that record, evaluates the bet's falsifier clauses and writes the bet verdict. The bet bead's acceptance line "the KILL-PACKET decision recorded" means on record by the other bead. The dependency of the bet on this bead is in place.

## Cluster of 3 beads

- `BD-124` (U33b, M1, feature) frob-arena: two study types on the CPU: the anytime e-process and the fixed-n exact engine; INT-FSIM-14  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U33b/BD-124.md`
- `N28a-18` (U28a, M1, feature) frob-opt: the pairing discipline: six rules, refusals 542, 543, 553, 555, 556, the accounting identity  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U28a/N28a-18.md`
- `N28a-19` (U28a, M1, feature) frob-opt: the fixed-n rule: paired exact intervals, reachability table, exact curtailment, PO-ATEL-02  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U28a/N28a-19.md`

- **pair** `BD-124` / `N28a-18`
  - reported by G28: BD-124 carries eval.compare on the CPU, eval.preanalyze and names GATE-ATEL-06, whose refusal rows N28a-18 measures; N28a-18 depends on BD-124
- **pair** `BD-124` / `N28a-19`
  - reported by G28: the fixed-n rule and reachability pre-analysis are in both; N28a-19 depends on BD-124

## Cluster of 3 beads

- `BD-341` (U41, M4, feature) verbs opt.start, opt.status: lever-only continuous optimization and e-process racing with mid-run cancellation  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U41/BD-341.md`
- `N28b-07` (U28b, M4, task) frob-opt: continuous optimizers through the airlock behind one ask/tell interface, moving levers by typed edits  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U28b/N28b-07.md`
- `N28b-08` (U28b, M4, task) frob-opt: race ledger of evaluations saved against a no-stopping baseline, with the optimization campaign e2e  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U28b/N28b-08.md`

- **pair** `BD-341` / `N28b-07`
  - reported by G41: the optimizer adapters (CMA-ES, trust-region Bayesian optimization, quasi-Newton) are in the scope of both; N28b-07 builds them behind ask/tell and asks that BD-341 depend on it (done)
- **pair** `BD-341` / `N28b-08`
  - reported by G41: the race ledger is in the scope of both: BD-341 writes the per-generation record, N28b-08 the saving against a no-stopping baseline and the campaign script; N28b-08 depends on BD-341

## Cluster of 3 beads

- `N00-27` (U00, M1, task) frob-conformance: the fourteen claim rules as one battery, with KA-027 to KA-032 and a CLI-plus-auditor script  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U00/N00-27.md`
- `N10a-06` (U10a, M1, task) frob-conformance: doctrine D-5, D-12, D-19 checks — weakest-link class, quantifiers, named assumptions  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U10a/N10a-06.md`
- `N37-25` (U37, M1, task) frob-conformance: GATE-SAFE-08 at M1: random evidence sets through the real serializer; class-raising API scan  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U37/N37-25.md`

- **pair** `N00-27` / `N10a-06`
  - reported by G10: overlap: both run the claim-product known answers KA-027 to KA-032 with their controls
- **pair** `N00-27` / `N37-25`
  - reported by G00: both test the claim product at M1 under GATE-SAFE-08: N00-27 as a battery of the fourteen claim rules with KA-027 to KA-032 and a per-surface sweep; N37-25 with random evidence sets through the real serializer and the class-raising API scan (rule 5). Rule 5 is tested twice
  - reported by G37: both test the claim product at M1 under GATE-SAFE-08 (the fourteen-rule battery against random evidence sets and the API scan); reported by G00 from its side

## Cluster of 3 beads

- `N19c-12` (U19c, M3, task) frob-arena scoring check: training scene manifests, leak check by scene and by group, FROB-REF-502  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U19c/N19c-12.md`
- `N19c-15` (U19c, M3, task) e2e GATE-HAB-14, GATE-ARENA-02: planted mirror and retrofit variant refused, counts recounted, reuse rejected  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U19c/N19c-15.md`
- `N24a-31` (U24a, M3, feature) frob-arena: scene-split enforcement at scoring time: reuse of a training scene group is refused (GATE-ARENA-02)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U24a/N24a-31.md`

- **pair** `N19c-12` / `N24a-31`
  - reported by G19: same work: the scoring-time check that refuses an evaluation on a scene or scene group of the policy's training manifest (FROB-REF-502, GATE-ARENA-02); N19c-12 adds training scene manifests and lineage, N24a-31 the audit record and planted cases
  - reported by G24: same mechanism: training scene manifests, leak check by split group at scoring time, FROB-REF-502, GATE-ARENA-02; N24a-31 now uses the same label in-sample
- **pair** `N19c-15` / `N24a-31`
  - reported by G19: both own an end-to-end script that records GATE-ARENA-02 on a training scene, its mirror and a retrofit variant
  - reported by G24: both own an end-to-end script that records GATE-ARENA-02 on planted mirror and retrofit variants

## Cluster of 2 beads

- `BD-006` (U33a, M0, task) INT-FSIM-01, core row of INT-TOOLCHAIN-01: FrankenSim consumed by path from outside its tree, costs measured  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U33a/BD-006.md`
- `N41-19` (U41, M0, task) First session, step 2: consume FrankenSim once: G1 forward-dynamics digest against FrankenSim's own, build cost, a two-second referee rollout  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U41/N41-19.md`

- **pair** `BD-006` / `N41-19`
  - reported by G41: INT-FSIM-01: FrankenSim consumed by path from outside its tree with costs measured is BD-006 (with the digest test of BD-007); step 2 of the first session records the same run as an observation and fills the same admission record

## Cluster of 2 beads

- `BD-041` (U38a, M0, task) plan-time pilot re-run: the cart docking closed form with two gains and with three, in exact rationals  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U38a/BD-041.md`
- `N41-24` (U41, M0, task) First session, step 7: re-run the cart-docking closed-form checks on the CPU in exact rationals, with the strict-comparator control caught  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U41/N41-24.md`

- **pair** `BD-041` / `N41-24`
  - reported by G41: the cart-docking closed form re-run in exact rationals with the strict-comparator control is BD-041; step 7 of the first session runs that program and records the result

## Cluster of 2 beads

- `BD-148` (U29a, M2, task) frob-model: packed lowering of `RobotModel`: depth levels, path tables, capacities, `ShapeClassId`  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U29a/BD-148.md`
- `BD-170` (U17b, M2, feature) frob-phylum: model variants in padded capacities (level P-B): a model table per slot, masks, CPU digest rows  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U17b/BD-170.md`

- **pair** `BD-148` / `BD-170`
  - reported by G17: lowering to a model table with padding to a shape class, and the ShapeClass type, are in both (frob-model and frob-phylum)
  - reported by G29: lowering to a padded model table and the ShapeClass type appear in both; BD-148 owns the plain-data type and ShapeClassId

## Cluster of 2 beads

- `BD-289` (U25b, M3, task) Pilot: region certificates on the 12-parameter box, four paving arms at 10⁴ boxes; KILL-AUGUR-REGION decided  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U25b/BD-289.md`
- `BET-20` (U00, M3, task) Decide BET-20: region certificates (per-constraint, about six split parameters) resolve a useful family share  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U00/BET-20.md`

- **pair** `BD-289` / `BET-20`
  - reported by G00: overlap with distinct scope, no merge: both record the decision KILL-AUGUR-REGION. The kill-gate bead applies the trigger and writes the kill ADR; the bet bead reads that record, evaluates the bet's falsifier clauses and writes the bet verdict. The bet bead's acceptance line "the KILL-AUGUR-REGION decision recorded" means on record by the other bead. The dependency of the bet on this bead is in place.

## Cluster of 2 beads

- `BET-13` (U00, M7, task) Decide BET-13: ten-fold on the sealed suite against the strongest composed rival, on at least one named stratum  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U00/BET-13.md`
- `N39b-06` (U39b, M3, task) Kill-gate decision KILL-TENFOLD-SPEED at M3 and M7: a ratio under 3 withdraws the speed claim for the class  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U39b/N39b-06.md`

- **pair** `BET-13` / `N39b-06`
  - reported by G00: overlap with distinct scope, no merge: both record the decision KILL-TENFOLD-SPEED. The kill-gate bead applies the trigger and writes the kill ADR; the bet bead reads that record, evaluates the bet's falsifier clauses and writes the bet verdict. The bet bead's acceptance line "the KILL-TENFOLD-SPEED decision recorded" means on record by the other bead. The dependency of the bet on this bead is in place.

## Cluster of 2 beads

- `GB-19` (UEG, M1, task) card GB-19 (frob-algebra): how many distinct designs a symmetric sweep holds: Burnside counts, ballast classes  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/GB-19.md`
- `N10b-12` (U10b, M3, task) spec C-22 part (frob-phylum): the mission-observable projection quotient and the ballast fixture  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U10b/N10b-12.md`

- **pair** `GB-19` / `N10b-12`
  - reported by GEG: same ballast fixture (735,471 placements, 24,546 classes): other specifies it for C-22 at M3 with pilot N10b-13, mine holds the count as a card golden at M1

## Cluster of 2 beads

- `N10a-09` (U10a, M3, task) frob-conformance: doctrine D-16 check — exploration issues no bound; bounds come from certification lanes  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U10a/N10a-09.md`
- `N26b-15` (U26b, M3, task) frob-nemesis: INV-NEMESIS-01 to -06 and PO-NEMESIS-01 to -04: binding ledger, crate-wide tests, mutants  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U26b/N26b-15.md`

- **pair** `N10a-09` / `N26b-15`
  - reported by G26: INV-NEMESIS-03 (exploration data never enter a bound) is tested crate-wide with mutants in N26b-15 and as the doctrine D-16 check in N10a-09

## Cluster of 2 beads

- `N12-15` (U12, M3, feature) frob-cert: checker for proof kind complex-connectivity (path replay; union–find on the outer complex)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U12/N12-15.md`
- `N19b-31` (U19b, M3, task) Clearance certificates: witness and cut objects, scene recomputation check, complex-connectivity in frob-cert  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U19b/N19b-31.md`

- **pair** `N12-15` / `N19b-31`
  - reported by G12: both specify the complex-connectivity checker in frob-cert (check A of N19b-31); keep it in N12-15; N19b-31 keeps the certificate objects, the scene-side check and the writer and should depend on N12-15
  - reported by G19: both specify the `complex-connectivity` checker in frob-cert (path replay, union-find on the outer complex, mutations); N19b-31 also carries the certificate objects, the scene recomputation check and the bundle entry

## Cluster of 2 beads

- `N16b-01` (U16b, M2, task) frob-arena: the `standard` battery defined and frozen per robot class, with a battery identity  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U16b/N16b-01.md`
- `N38a-20` (U38a, M2, task) frob-conformance: the standard battery: scenario grid per robot class, fixed seeds, frozen identity  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U38a/N38a-20.md`

- **pair** `N16b-01` / `N38a-20`
  - reported by G16: both define and freeze the standard battery (scenario grid per robot class, seeds, identity); N16b-01 writes it in frob-arena, N38a-20 in frob-conformance
  - reported by G38: both define and freeze the standard battery (scenario grid per robot class, seeds, identity, change events): N38a-20 in frob-conformance/battery, which is where §38.1 places it, N16b-01 in frob-arena with the manifest format and address scheme. Keep N38a-20 for the grid, identity and events and fold N16b-01's address scheme into it, or keep N16b-01 as the frob.battery/1 instance only; eight beads of U16 depend on N16b-01

## Cluster of 2 beads

- `N18b-25` (U18b, M2, task) frob-kernels: K-scale-diag, exact power-of-two diagonal scaling, and the bitwise scale-invariance proof PO-TRAC-03  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U18b/N18b-25.md`
- `N21b-38` (U21b, M2, task) Conditioning certificate of the `f32` contact solve: κ̂ from the factor, scaling invariance (PO-NUM-02)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U21b/N21b-38.md`

- **pair** `N18b-25` / `N21b-38`
  - reported by G18: K-scale-diag and K-scale are one kernel; PO-TRAC-03 and PO-NUM-02 are one identity on one fixture

## Cluster of 2 beads

- `N21a-15` (U21a, M2, task) pilot INT-FJAX-01: e-graph extraction over exact rewrite rules in the derive annex, only if the profile asks  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U21a/N21a-15.md`
- `N33c-21` (U33c, M2, task) pilot INT-FJAX-01: ceiling check and e-graph extraction pilot on derived kernel arithmetic (frankenjax pattern)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U33c/N33c-21.md`

- **pair** `N21a-15` / `N33c-21`
  - reported by G21: the INT-FJAX-01 e-graph extraction pilot is drafted twice
  - reported by G33: the INT-FJAX-01 e-graph extraction pilot is drafted twice; BD-221 also names the gate (as G21 recorded)

## Cluster of 2 beads

- `N25d-48` (U25d, M1, task) frob-conformance: the Augur open-question register (OPEN-AUGUR-1 to -11) and the defaults-in-force battery  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U25d/N25d-48.md`
- `N41-OPEN5` (U41, M0, task) registries/open.toml, part 5: open questions of Augur, Nemesis and Autopsy (20 rows, one settled)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U41/N41-OPEN5.md`

- **pair** `N25d-48` / `N41-OPEN5`
  - reported by G25cde: overlap: both enter the eleven Augur rows of registries/open.toml; N25d-48 also carries the defaults-in-force battery
  - reported by G41: the eleven Augur questions are entered in both, and into different files: N25d-48 writes registries/open_questions.toml, the register of N41-03 is registries/open.toml (also reported by the Augur group)

## Cluster of 2 beads

- `N30b-13` (U30b, M5, task) verb cluster ground: seven registry entries with schemas, capabilities and corrected refusal lists  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U30b/N30b-13.md`
- `VERBS-ground` (UJL, M1, task) frob-verbs registry: cluster `ground`, seven rows, with parity, e2e and schema-snapshot checks  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UJL/VERBS-ground.md`

- **pair** `N30b-13` / `VERBS-ground`
  - reported by G30: both enter the seven registry rows of cluster ground (kinds, schemas, refusal lists, MCP names); N30b-13 adds the corrected refusal lists, capabilities and the --retire parameter set
  - reported by GJL: both enter the seven registry rows of cluster ground; N30b-13 has the corrected refusal lists (also reported by G30)

## Cluster of 2 beads

- `N38b-21` (U38b, M0, task) e2e status_accounting: GATE-GAUNT-10 measured from a real run to the README table, with both planted refusals  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U38b/N38b-21.md`
- `N40-36` (U40, M1, task) e2e: the gate register from a recorded measurement to the status table, evidence trace and exit report  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U40/N40-36.md`

- **pair** `N38b-21` / `N40-36`
  - reported by G38: two e2e scripts over the gate register: N38b-21 measures the three clauses of GATE-GAUNT-10 at M0, N40-36 the wider path to the exit report at M1; distinct scope, related
  - reported by G40: both scripts walk a recorded measurement to the generated README status table; N38b-21 stops at the three clauses of GATE-GAUNT-10, N40-36 continues to the evidence trace and the exit report

