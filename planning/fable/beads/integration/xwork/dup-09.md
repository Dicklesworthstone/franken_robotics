# Cross-group overlap packet dup-09

36 pairs in 21 clusters. Each cluster is a set of beads connected by reported overlaps; decide a cluster as a whole.

## Cluster of 6 beads

- `BD-064` (U24c, M1, feature) frob-design: design lints with ranked repairs as typed edits, each with a triggering and a clean fixture  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U24c/BD-064.md`
- `N13-34` (U13, M1, task) Periodic and lifted laws: the typing rule for elements that read a revolute coordinate, and its refusal  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U13/N13-34.md`
- `N14a-08` (U14a, M1, task) frob-model: validate stage and preflight stage 1 (Form): check order, refusals with repairs, seeded defects  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U14a/N14a-08.md`
- `N14a-09` (U14a, M1, task) frob-model: binding functor: actuator and sensor channels, joint row descriptors, contact bindings, turn state  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U14a/N14a-09.md`
- `N15a-03` (U15a, M1, task) frob-model: binding a card to a joint, with actuator.check re-run at bind, stage-1 refusals and --strict-data  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U15a/N15a-03.md`
- `N15b-10` (U15b, M1, feature) frob-actuate: coupling maps C(q) and the feasible joint-torque zonotope returned by actuator.envelope  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U15b/N15b-10.md`

- **pair** `BD-064` / `N14a-08`
  - reported by G24: preflight stage 1 with ranked repairs and seeded-defect fixtures per code appears in both (BD-064 as design lints, N14a-08 as the validate stage)
- **pair** `N13-34` / `N14a-08`
  - reported by G13: both specify the stage-1 refusal of a lifted law on a joint without turn state, and N14a-09 the derivation of which joints need the count, which N13-34 also lists in scope; N13-34 depends on N14a-08 and the stage runner, and the Skeleton beads keep N13-34 related
- **pair** `N13-34` / `N14a-09`
  - reported by G14: the turn-state declaration per revolute coordinate adjoins the periodic/lifted typing rule and its stage-1 refusal
- **pair** `N14a-09` / `N15a-03`
  - reported by G14: N15a-03 also resolves card references, keeps the bound-card record with overrides and records which card fields a product read
  - reported by G15: both bind actuator cards to joints in frob-model (bound-card record against the binding functor)
- **pair** `N14a-09` / `N15b-10`
  - reported by G15: the binding functor's transmission binding and coupling maps against the CouplingMap type

## Cluster of 5 beads

- `BD-180` (U41, M2, task) frob-kernels: randomness by address: K-x-philox, key derivation from a world address, exact uniforms, streams  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U41/BD-180.md`
- `N23a-04` (U23a, M2, task) frob-learn: world identity, seed plans, caller order across devices, snapshot and restore (rule E-7)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U23a/N23a-04.md`
- `N35-16` (U35, M2, task) frob-conformance: draw tables: a draw depends on (address, step, stream) alone (INV-NUM-04)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U35/N35-16.md`
- `N35-22` (U35, M1, feature) frob-core + frob-fsim: addressed random streams on the host: key layout, exact uniforms, normals  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U35/N35-22.md`
- `N35-23` (U35, M1, task) The stream registry `registries/streams.toml`: streams by purpose, exposure domain and key schema  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U35/N35-23.md`

- **pair** `BD-180` / `N35-16`
  - reported by G35: both carry draw tables and their invariance tests
  - reported by G41: draw tables and their invariance tests (KA-795, INV-NUM-04) are in both; N35-16 depends on BD-180 and re-runs the comparison as a battery on every backend
- **pair** `BD-180` / `N35-22`
  - reported by G41: key layout, exact uniform conversion and the normal construction are specified in both: N35-22 on the host (M1), BD-180 as kernels that must equal it (M2)
- **pair** `BD-180` / `N35-23`
  - reported by G35: both carry the stream registry
  - reported by G41: both list the stream registry in scope; registries/streams.toml is N35-23's, and BD-180 (capstone) adds the device kernel identity and clock per row
- **pair** `N23a-04` / `N35-22`
  - reported by G35: two WorldAddr structs: (run_seed, namespace, world, episode, fork) here, (run_seed, world, episode) there; G21 and G24 resolve the type to N35-22

## Cluster of 4 beads

- `BD-130` (U25a, M1, feature) frob-mission: stance family: body–load feasibility, vertex witnesses, dual exclusions, lifted and projected form  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U25a/BD-130.md`
- `N25a-13` (U25a, M1, feature) frob-mission: M1 revalidation of cells by identity and dependency support (grades 1, 2); passive-spring amendment  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U25a/N25a-13.md`
- `N25c-11` (U25c, M1, feature) frob-mission: authority models and force floors in stance certificates; the passive-spring amendment  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U25c/N25c-11.md`
- `N25c-12` (U25c, M1, task) frob-conformance: the stance validation grid (2,304 bodies, 11,520 states) and the weak-screen control  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U25c/N25c-12.md`

- **pair** `BD-130` / `N25c-11`
  - reported by G25ab: BD-130's scope includes the authority models independent-normal-force and passive-equal-spring, which N25c-11 specifies as a typed field with its lowering; seed and component, scopes to be reconciled
- **pair** `BD-130` / `N25c-12`
  - reported by G25ab: BD-130's scope includes the validation grid (2,304 bodies, 11,520 states) that N25c-12 builds in frob-conformance; seed and component, scopes to be reconciled
- **pair** `N25a-13` / `N25c-11`
  - reported by G25ab: both specify the passive-spring amendment that withdraws the stance cell at half-width 1/4 m: N25a-13 as the M1 revalidation case, N25c-11 as the authority amendment
  - reported by G25cde: overlap: both carry the passive-spring amendment of the stance family (withdrawal of the cell at b = 1/4 m); N25c-11 as an authority model in certificates, N25a-13 as M1 revalidation

## Cluster of 4 beads

- `N15a-08` (U15a, M1, task) frob-actuate: the stiction row descriptor: convex potential and a Stribeck bound frozen per solve  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U15a/N15a-08.md`
- `N15a-09` (U15a, M1, task) frob-actuate: backlash as two unilateral rows on the rotor–joint angle, with the rotor as a coordinate (L3)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U15a/N15a-09.md`
- `N15a-12` (U15a, M1, task) e2e sinew_joint_rows: the six stiction and backlash known answers on a running system (GATE-SINEW-04)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U15a/N15a-12.md`
- `N18a-32` (U18a, M1, feature) frob-contact: joint-level row laws (stiction cone, backlash rows, hard stops) and their known-answer fixtures  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U18a/N18a-32.md`

- **pair** `N15a-08` / `N18a-32`
  - reported by G15: both state the stiction row law (convex potential and its exact form); N18a-32 also needs the descriptor this bead emits
  - reported by G18: the stiction row law (convex potential, Stribeck bound) is specified in both; N15a-09 likewise for backlash rows
- **pair** `N15a-09` / `N18a-32`
  - reported by G15: both state the compliant and hard backlash rows and the hard stops as laws
- **pair** `N15a-12` / `N18a-32`
  - reported by G15: both own hand-checkable joint-row fixtures and the dead-zone mutant

## Cluster of 3 beads

- `BD-249` (U22a, M3, task) Baseline synthesis on the design corpus: GATE-REFLEX-02 measured, the KILL-BASELINE trigger computed (BET-06)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U22a/BD-249.md`
- `BET-06` (U00, M3, task) Decide BET-06: a baseline controller synthesized from the model makes most well-formed designs move  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U00/BET-06.md`
- `N22c-14` (U22c, M3, task) decision KILL-BASELINE: baseline synthesis for every class, or only for the classes that pass on the corpus  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U22c/N22c-14.md`

- **pair** `BD-249` / `BET-06`
  - reported by G00: overlap with distinct scope, no merge: both record the decision KILL-BASELINE. The kill-gate bead applies the trigger and writes the kill ADR; the bet bead reads that record, evaluates the bet's falsifier clauses and writes the bet verdict. The bet bead's acceptance line "the KILL-BASELINE decision recorded" means on record by the other bead. The dependency of the bet on this bead is in place.
- **pair** `BET-06` / `N22c-14`
  - reported by G00: overlap with distinct scope, no merge: both record the decision KILL-BASELINE. The kill-gate bead applies the trigger and writes the kill ADR; the bet bead reads that record, evaluates the bet's falsifier clauses and writes the bet verdict. The bet bead's acceptance line "the KILL-BASELINE decision recorded" means on record by the other bead. The dependency of the bet on this bead is in place.

## Cluster of 3 beads

- `GB-07` (UEG, M2, task) card GB-07 (frob-scene): how much memory a scene is: volume/spacing³ samples, 2 bytes each, and the narrow band  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/GB-07.md`
- `N19c-16` (U19c, M2, task) frob-scene: the scene byte model (dense, band, directory) and the memory arithmetic of scenes as checked tests  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U19c/N19c-16.md`
- `N36-16` (U36, M2, task) frob-kernels estimate: the per-scene memory model (dense, band, labels) and shared-scene capacity tables  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U36/N36-16.md`

- **pair** `GB-07` / `N19c-16`
  - reported by G19: the worked card recomputes the scene memory arithmetic (dense and band figures) that N19c-16 holds as checked tests
  - reported by GEG: same scene memory arithmetic (10 × 10 × 3 m at 2 cm: 37,500,000 samples, 75 MB; the 600 MB figure belongs to 4 cm, CX-198). Other holds it as checked tests of the byte model; mine is the card file with two lanes and controls (edge GB-07 → N19c-16)
- **pair** `N19c-16` / `N36-16`
  - reported by G36: the scene byte model (dense 2·V/s³, band fraction, labels 12.5%, directory, capacity counts) is written twice, in frob_kernels::estimate::scene and in frob-scene bytes.rs; N19c-16 says the job estimator calls its model. Their directory formulas differ (4-byte slots per 8³-voxel brick, 75,411 slots and 0.3 MB for a 10 × 10 × 3 m interior, against 6 bytes per coarse cell, 73,242 cells and 0.44 MB). Keep the functions and recount in N19c-16; N36-16 keeps the slabs, the group memory.scenes, the two modes and the repairs (dependency recorded)

## Cluster of 3 beads

- `N14b-34` (U14b, M1, feature) frob-io: reduced models: the `reduced` block, refusal of mass dropped at import, the `g1-reduced-30` profile  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U14b/N14b-34.md`
- `N33c-41` (U33c, M0, task) admission probe (row 9): the G1 catalog builder's mass roll-up of 31.96314202 kg and its declared omissions  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U33c/N33c-41.md`
- `ROBOT-fsim-g1` (UEG, M1, task) assets/robots/fsim-g1: the sibling library's G1 builder (30 links, 31.96314202 kg) as profile `g1-reduced-30`  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/ROBOT-fsim-g1.md`

- **pair** `N14b-34` / `ROBOT-fsim-g1`
  - reported by G14: both define the profile `g1-reduced-30`; ROBOT-fsim-g1 depends on N14b-34
  - reported by GEG: both define profile g1-reduced-30 (30 links, 31.96314202 kg): other the reduced-model mechanism and assets/profiles/g1-reduced-30.toml, mine the asset directory, manifest and validation golden (dependency exists)
- **pair** `N33c-41` / `ROBOT-fsim-g1`
  - reported by GEG: both record the admission roll-up of the G1 catalog builder (31.96314202 kg, GATE-WS-09); NF-07 repeats the test upstream

## Cluster of 3 beads

- `N26a-26` (U26a, M3, task) frob-nemesis: the certify-refute-stop decision policy of nemesis.bound (DC-04): a fixed table, finalists only  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U26a/N26a-26.md`
- `N27b-12` (U27b, M3, task) frob-autopsy: the attribution decision policy (DC-06): stage table, fallback triggers, rollback, baselines  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U27b/N27b-12.md`
- `N28a-27` (U28a, M4, task) registries/cores: failure-plane cores DC-04 (rare-failure bound) and DC-06 (cause and repair ranking)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U28a/N28a-27.md`

- **pair** `N26a-26` / `N28a-27`
  - reported by G26: both write the DC-04 record; N28a-27 prints the certification count as ⌈ln(1/δ)/p*⌉ and cites FROB-REF-549, where this chapter uses the exact ⌈ln δ/ln(1 − p⋆)⌉ (29,956 not 29,958) and FROB-REF-560 at the bound
  - reported by G28: both write the DC-04 record. N28a-27 now states the exact binomial counts (29,956; 299,572; 2,995,731) beside the table formula, as N28c-10 does; the plan has two codes for one refusal (549 in §28.20, 560 in §26.12), now noted as a layered pair in N28-REF
- **pair** `N27b-12` / `N28a-27`
  - reported by G27: both write the DC-06 record registries/cores/dc-06.toml with its compiled table, forced-trigger demonstrations, shadow records and demonstration manifest: N27b-12 at M3 with the policy (BD-282 adds the abstention replay cases), N28a-27 at M4. Suggested: N28a-27 validates and completes the M3 record for GATE-ATEL-13 and writes no second one; it already depends on BD-282

## Cluster of 2 beads

- `BD-087` (U41, M1, task) frob-dyn: the free-body variational step (kick, drift, kick on SE(3), Cayley map) as a known-answer oracle  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U41/BD-087.md`
- `N16c-03` (U16c, M1, task) frob-conformance: Kinesis known answers with failing mutants, transactional step and free-body rows (M1)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U16c/N16c-03.md`

- **pair** `BD-087` / `N16c-03`
  - reported by G41: the free-body step's known answer KA-355 and its failing mutants (right-trivialized differential, additive update) are registered in both; N16c-03 depends on BD-087

## Cluster of 2 beads

- `BD-230` (U18a, M3, feature) frob-contact: the film-friction law for wet and greasy floors: thickness state and cone row (`K-film`)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U18a/BD-230.md`
- `N19b-05` (U19b, M3, task) frob-scene: Solid film patches: fluid class, h₀, wetness latent; per-contact thickness state and exact update  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U19b/N19b-05.md`

- **pair** `BD-230` / `N19b-05`
  - reported by G18: the per-contact film thickness state and its update appear in both; BD-230 keeps the cone row and K-film and uses the blend and the cap at μ_b of the scene-side statement
  - reported by G19: the per-contact film thickness state and its exact update are specified in both; N19b-05 owns the patch table, lifecycle and host reference with PO-HAB-04, BD-230 the cone row and K-film (edge BD-230 depends on N19b-05 recorded)

## Cluster of 2 beads

- `BD-319` (U40, M3, task) M3 exit review by a fresh-context session that did not implement the work; the partial ten-fold gate on record  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U40/BD-319.md`
- `N27b-30` (U27b, M3, task) Autopsy exit review at M3 by a fresh-context session: nine gates and two kill gates on record  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U27b/N27b-30.md`

- **pair** `BD-319` / `N27b-30`
  - reported by G40: N27b-30 is a chapter-level exit review of the nine Autopsy gates and two kill gates due at M3, by the same fresh-context procedure; BD-319 reviews the same rows with every other M3 gate and is made to depend on it

## Cluster of 2 beads

- `BET-26` (U00, M4, task) Decide BET-26: gait orbits as screening (periodic orbits continued in design parameters, on regular ground)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U00/BET-26.md`
- `N22c-17` (U22c, M4, task) decision KILL-GAIT: gait orbits as a screening rung, or as stability diagnostics and warm starts only  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U22c/N22c-17.md`

- **pair** `BET-26` / `N22c-17`
  - reported by G00: overlap with distinct scope, no merge: both record the decision KILL-GAIT. The kill-gate bead applies the trigger and writes the kill ADR; the bet bead reads that record, evaluates the bet's falsifier clauses and writes the bet verdict. The bet bead's acceptance line "the KILL-GAIT decision recorded" means on record by the other bead. The dependency of the bet on this bead is in place.

## Cluster of 2 beads

- `N00-10` (U00, M0, task) The evidence ledger: the hypothesis verdict register, its append-only change rule and the wording it binds  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U00/N00-10.md`
- `NAB-32` (UAB, M0, task) killed-idea register: registries/killed_ideas.toml, docs/decisions/killed-ideas.md and the revive lint  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UAB/NAB-32.md`

- **pair** `N00-10` / `NAB-32`
  - reported by GAB: the hypothesis register's banned-pattern lint (a refuted strong form stated as a goal) and the revive lint's findings ki.strong-form-as-goal and ki.strong-form-duplicated check the same rule from two pattern lists

## Cluster of 2 beads

- `N10b-01` (U10b, M4, task) spec C-16 part (frob-dyn): the nine derivative-validity labels and refusal FROB-REF-263  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U10b/N10b-01.md`
- `N16b-24` (U16b, M3, task) frob-dyn: limits of the claim, labelled fixed-mode step derivatives with refusal, and no real-time claim  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U16b/N16b-24.md`

- **pair** `N10b-01` / `N16b-24`
  - reported by G16: both specify the derivative-validity labels on a step and the refusal FROB-REF-263; N10b-01 is the M4 specification part of C-16, N16b-24 the M3 mechanism

## Cluster of 2 beads

- `N14a-12` (U14a, M1, task) frob-model: lowerings 3 and 4: analysis views (limb kinematics, limb permutation, budget graph), shape summary  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U14a/N14a-12.md`
- `N17a-02` (U17a, M2, task) frob-phylum: shape-class fitting: capacity requirement, smallest admitted class, P-A/P-B, FROB-REF-160  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U17a/N17a-02.md`

- **pair** `N14a-12` / `N17a-02`
  - reported by G14: both state whether a model fits a shape class and with what padding
  - reported by G17: N14a-12 also carries the function that states whether a model fits a shape class and with what padding

## Cluster of 2 beads

- `N17a-16` (U17a, M4, feature) frob-phylum: survivor promotion to specialized kernels: promotion rule, 60 s cap, boundary swap  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U17a/N17a-16.md`
- `N21c-20` (U21c, M4, task) frob-kernels: survivor cohort switch-over to a specialized kernel at a chunk boundary, versioned state map  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U21c/N21c-20.md`

- **pair** `N17a-16` / `N21c-20`
  - reported by G17: both specify survivor promotion: promotion rule, 60 s compile cap, swap at a boundary with a versioned state map

## Cluster of 2 beads

- `N19a-08` (U19a, M3, task) frob-scene: declared behaviours, movable objects, moving surfaces, and FROB-REF-315  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U19a/N19a-08.md`
- `N24a-43` (U24a, M4, feature) frob-contact, frob-scene: moving surfaces as prescribed-velocity contacts; boundary work on its ledger line  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U24a/N24a-43.md`

- **pair** `N19a-08` / `N24a-43`
  - reported by G19: the moving-surface record and the ledger line for the boundary work of prescribed surfaces are specified in both
  - reported by G24: the moving-surface record and the ledger line for boundary work of prescribed surfaces are specified in both

## Cluster of 2 beads

- `N25a-20` (U25a, M8, task) Decision KILL-AUGUR-TRANSPORT at M8: interface skill transport, or family policies with static wrench-coverage test  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U25a/N25a-20.md`
- `N25c-05` (U25c, M8, task) decision KILL-AUGUR-TRANSPORT: keep interface-based skill transport, or fall back to family policies  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U25c/N25c-05.md`

- **pair** `N25a-20` / `N25c-05`
  - reported by G25ab: both decide KILL-AUGUR-TRANSPORT at M8 on the record of the three-treatment tray experiment; one decision bead should remain
  - reported by G25cde: true duplicate: both are the kill-gate decision KILL-AUGUR-TRANSPORT at M8 on the tray experiment's record

## Cluster of 2 beads

- `N26b-13` (U26b, M3, task) verb family nemesis.*: registry rows, schemas, refusal lists and task clusters for the seven Nemesis verbs  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U26b/N26b-13.md`
- `VERBS-nemesis` (UJL, M3, task) frob-verbs registry: cluster `nemesis`, seven rows, with parity, e2e and schema-snapshot checks  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UJL/VERBS-nemesis.md`

- **pair** `N26b-13` / `VERBS-nemesis`
  - reported by G26: both enter the seven nemesis.* rows in registries/verbs.toml with schemas, tool names and checks; N26b-13 also carries scopes, cost models, wait defaults and the cluster proposal for nemesis_cover
  - reported by GJL: both enter the seven nemesis rows with schemas, tool names and checks (also reported by G26)

## Cluster of 2 beads

- `N32a-02` (U32a, M0, task) frob-verbs: `registries/verbs.toml`, build script and static table, registry lints, `registry-only`  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U32a/N32a-02.md`
- `NJL-01` (UJL, M0, task) registries/verbs.toml: the entry schema of one verb, the four kinds, the tool-name map and the registry lints  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UJL/NJL-01.md`

- **pair** `N32a-02` / `NJL-01`
  - reported by G32: registries/verbs.toml, its entry schema, the tool-name map and the registry lints are specified in both
  - reported by GJL: both specify the format and schema of registries/verbs.toml, the MCP tool-name map and the registry lints; N32a-02 adds the build script, the static table and registry-only

## Cluster of 2 beads

- `N39b-37` (U39b, M3, task) frob-conformance tenfold: the pre-sealing rival audit and experiment pack (GATE-TENFOLD-11)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U39b/N39b-37.md`
- `N40-20` (U40, M3, task) program gate GATE-TENFOLD-11: the rival audit complete before sealing, each arm within 10% of its figure  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U40/N40-20.md`

- **pair** `N39b-37` / `N40-20`
  - reported by G39: both record GATE-TENFOLD-11
  - reported by G40: both check the experiment pack, the 10% reproduction, the sign-off and the digest lock of GATE-TENFOLD-11; N40-20 is made to depend on N39b-37

