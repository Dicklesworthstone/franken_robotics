# Cross-group overlap packet dup-06

36 pairs in 21 clusters. Each cluster is a set of beads connected by reported overlaps; decide a cluster as a whole.

## Cluster of 7 beads

- `N14b-03` (U14b, M3, feature) frob-design: wheeled archetypes of M3: ackermann, omni-3, swerve-4, rocker-bogie, balancer-2w  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U14b/N14b-03.md`
- `N38b-47` (U38b, M1, task) frob-conformance: derivation files for six known answers whose truths are named and not written  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U38b/N38b-47.md`
- `ZOO-ackermann` (UEG, M3, task) zoo/ackermann.fbot: archetype 8, car-like base with a steering linkage: C₂, growth vector (2, 3, 4), L/tan δ  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/ZOO-ackermann.md`
- `ZOO-balancer-2w` (UEG, M3, task) zoo/balancer-2w.fbot: archetype 18, two-wheeled inverted pendulum: C₂, the open-loop pole in closed form  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/ZOO-balancer-2w.md`
- `ZOO-omni-3` (UEG, M3, task) zoo/omni-3.fbot: archetype 10, three omni wheels at 120°: D₃, growth vector (3), the 3 × 3 wheel map  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/ZOO-omni-3.md`
- `ZOO-rocker-bogie` (UEG, M3, task) zoo/rocker-bogie.fbot: archetype 15, six wheels on a passive rocker-bogie: C₂, differential, mobility 3  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/ZOO-rocker-bogie.md`
- `ZOO-swerve-4` (UEG, M3, task) zoo/swerve-4.fbot: archetype 11, four steer-and-drive modules: D₄, nested symmetry, scrub torque, singular pose  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/ZOO-swerve-4.md`

- **pair** `N14b-03` / `ZOO-ackermann`
  - reported by GEG: same inner and outer wheel angles (23.07°, 17.63°) and FROB-REF-110 control in both
- **pair** `N14b-03` / `ZOO-balancer-2w`
  - reported by GEG: same candidate pole formula and example 8.496 rad/s (control 9.17) in both, each naming docs/derivations/balancer-2w-pole.md
- **pair** `N14b-03` / `ZOO-omni-3`
  - reported by GEG: same 3 × 3 wheel map at 90°, 210°, 330° in both
- **pair** `N14b-03` / `ZOO-rocker-bogie`
  - reported by GEG: same differential closure between the rockers and C₂ group rows in both
- **pair** `N14b-03` / `ZOO-swerve-4`
  - reported by GEG: same D₄ group, scrub torque and singular pose rows in both
- **pair** `N38b-47` / `ZOO-balancer-2w`
  - reported by GEG: same pole formula for KA-269, different fixture: mine and N14b-03 use I_b + m_b·l² = 1.1 kg·m² (8.49628 rad/s, control 9.1698); N38b-47's illustrative fixture uses 1.2 kg·m² (7.614 s⁻¹, control 8.087). One fixture should govern

## Cluster of 5 beads

- `N29b-12` (U29b, M2, task) decision OPEN-CHRON-4: retention windows for failure captures and the default store budget on each machine  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U29b/N29b-12.md`
- `N37-09` (U37, M4, task) decision OPEN-SAFE-3: whether sandboxed custom reward terms are admitted, in what sandbox, and the pilot owed first  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U37/N37-09.md`
- `N37-20` (U37, M3, task) decision OPEN-SAFE-2: do policy weights trained in a private or research-only scene inherit its mark and licences  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U37/N37-20.md`
- `N37-30` (U37, M3, task) decision OPEN-SAFE-4: retention periods for job logs, idempotency keys and retired confirmation clusters  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U37/N37-30.md`
- `N41-11` (U41, M2, task) Owner decision OPEN-SAFE-2, -3, -4, OPEN-CHRON-4: weight privacy, custom reward terms, retention, store budget  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U41/N41-11.md`

- **pair** `N29b-12` / `N41-11`
  - reported by G41: both decide OPEN-CHRON-4 (retention of failure captures and store budgets)
- **pair** `N37-09` / `N41-11`
  - reported by G37: both decide OPEN-SAFE-3; N37-09 (M4) also defines the paper pilot and its falsifier
  - reported by G41: both decide OPEN-SAFE-3 (custom reward terms)
- **pair** `N37-20` / `N41-11`
  - reported by G37: both decide OPEN-SAFE-2; N41-11 bundles OPEN-SAFE-2, -3, -4 and OPEN-CHRON-4 at M2 and keeps the values in registries/policy/data-policy.toml, N37-20 keeps them in the owner-held safety.toml of N37-02
  - reported by G41: both decide OPEN-SAFE-2 (whether weights trained in a private scene inherit its mark and licences)
- **pair** `N37-30` / `N41-11`
  - reported by G37: both decide OPEN-SAFE-4; the two beads name different files for the three retention values (safety.toml against data-policy.toml)
  - reported by G41: both decide OPEN-SAFE-4 (retention periods for logs, keys and retired clusters)

## Cluster of 4 beads

- `BD-401` (U33b, M5, feature) frob-foundry: the beam-rung sizing check and analysis A-15 structural margin (`foundry.size`); INT-FSIM-15  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U33b/BD-401.md`
- `N31a-60` (U31a, M5, task) frob-foundry: beam-rung load model: section forces along a member from one time-coherent link load  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U31a/N31a-60.md`
- `N31a-61` (U31a, M5, task) frob-foundry: the beam rung as the loop's rung: stock sizing, notch fatigue in beam form, the baseline  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U31a/N31a-61.md`
- `N31b-38` (U31b, M3, task) frob-foundry: closed-form member-sizing kernel at M3: mass floor, lightest stock section, exact decisions  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U31b/N31b-38.md`

- **pair** `BD-401` / `N31a-60`
  - reported by G31: overlap with distinct scope: N31a-60 extends BD-401's load model from a tip wrench to a full time-coherent link load and adds `foundry.size --diagram`; it depends on BD-401
- **pair** `BD-401` / `N31a-61`
  - reported by G31: overlap with distinct scope: stock sizing under campaign loads with notch fatigue and the engineered baseline; it depends on BD-401
- **pair** `BD-401` / `N31b-38`
  - reported by G31: overlap with distinct scope: the M3 closed-form sizing kernel in beam/{section,criteria,stock,size}.rs and the M5 beam rung in the same modules; BD-401 should depend on N31b-38 and its Scope should say it extends the kernel
  - reported by G33: seed and earlier component: N31b-38 delivers the closed-form member-sizing kernel at M3 in the modules BD-401 names; BD-401 now depends on it and extends it

## Cluster of 3 beads

- `BD-095` (U29a, M1, feature) frob-chronicle: the artifact graph (claims, evidence, decision rules, traces) and the verb evidence.trace  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U29a/BD-095.md`
- `N29b-09` (U29b, M1, task) decision OPEN-CHRON-3: artifact-graph identifier types, imported from franken_kernel or local in frob-core  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U29b/N29b-09.md`
- `N34-11` (U34, M0, task) frob-core: artifact-graph identifiers ClaimId, EvidenceId, DecisionPolicyId, TraceId in the suite's text form  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U34/N34-11.md`

- **pair** `BD-095` / `N34-11`
  - reported by G29: both define ClaimId, EvidenceId, DecisionPolicyId and TraceId in frob-core (ids/graph.rs against graph_id); N34-11 (M0) should own the types and BD-095 consume them; dependency BD-095 -> N34-11 recorded
  - reported by G34: both define ClaimId, EvidenceId, DecisionPolicyId and TraceId in frob-core; N34-11 (M0) owns the types and BD-095 consumes them (also reported by G29, which aligned its beads)
- **pair** `N29b-09` / `N34-11`
  - reported by G29: text forms: N34-11 fixes clm:, ev: and name@vN; N29b-09 proposed clm: and evd:; N29b-09 and N29a-09 were aligned to N34-11 in this pass

## Cluster of 3 beads

- `BD-194` (U24b, M2, task) frob-spec/frob-learn: task terms: requirement terms, rate rewards, physical discount, applied-action likelihood  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U24b/BD-194.md`
- `N23a-05` (U23a, M2, task) frob-learn: the term contract — what each kind of task term must declare before VecEnv accepts a TaskProgram  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U23a/N23a-05.md`
- `N23a-08` (U23a, M2, task) frob-learn: registered action laws latent-then-limit and clipped-with-endpoint-mass, with entropy (P-3, P-11)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U23a/N23a-08.md`

- **pair** `BD-194` / `N23a-05`
  - reported by G23: the table of what each kind of task term must declare is in both; N23a-05 is the learner-side admission pass and depends on BD-194
  - reported by G24: the table of what each kind of task term must declare is in both; N23a-05 depends on BD-194
- **pair** `BD-194` / `N23a-08`
  - reported by G23: the registered action laws, their log-likelihoods and FROB-REF-527 are specified in both (also reported by G24)
  - reported by G24: the registered action laws, their log-likelihoods and FROB-REF-527 are specified in both

## Cluster of 3 beads

- `BET-25` (U00, M4, task) Decide BET-25: energy-budget safety tier (no contact model in the claim; measured shield margin; class Tested)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U00/BET-25.md`
- `N22b-16` (U22b, M4, task) Kill-gate decision KILL-ENERGY: keep the energy-budget claim, or keep only an `Estimated` alarm  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U22b/N22b-16.md`
- `N22b-28` (U22b, M4, task) Kill-gate decision KILL-SHIELD: keep runtime shields, or turn barrier rows into reward penalties and alarms  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U22b/N22b-28.md`

- **pair** `BET-25` / `N22b-16`
  - reported by G00: overlap with distinct scope, no merge: both record the decision KILL-ENERGY. The kill-gate bead applies the trigger and writes the kill ADR; the bet bead reads that record, evaluates the bet's falsifier clauses and writes the bet verdict. The bet bead's acceptance line "the KILL-ENERGY decision recorded" means on record by the other bead. The dependency of the bet on this bead is in place.
- **pair** `BET-25` / `N22b-28`
  - reported by G00: overlap with distinct scope, no merge: both record the decision KILL-SHIELD. The kill-gate bead applies the trigger and writes the kill ADR; the bet bead reads that record, evaluates the bet's falsifier clauses and writes the bet verdict. The bet bead's acceptance line "the KILL-SHIELD decision recorded" means on record by the other bead. The dependency of the bet on this bead is in place.

## Cluster of 3 beads

- `N10b-10` (U10b, M2, task) pilot PILOT-C-19: docking family against a scenario-tree linear program, and one planted stale-read race  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U10b/N10b-10.md`
- `N20a-09` (U20a, M2, task) pilot: the schedule-race check finds a planted stale read and a planted watchdog-against-latch race  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U20a/N20a-09.md`
- `N25c-20` (U25c, M1, task) frob-mission: the scenario-tree linear-program comparator for the docking family, checked as a competing answer  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U25c/N25c-20.md`

- **pair** `N10b-10` / `N20a-09`
  - reported by G10: overlap: both plant a stale-read race for the schedule-race check
- **pair** `N10b-10` / `N25c-20`
  - reported by G25cde: overlap: both run the docking family against a scenario-tree linear program (51 solves); N10b-10 is the construction's pilot at M2, N25c-20 the M1 comparator that also discharges PO-AUGUR-01

## Cluster of 3 beads

- `N21a-10` (U21a, M2, task) frob-kir: dialect lint completed (loops, integers, masks, padding), Rust kernel lint, compile paths wired  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U21a/N21a-10.md`
- `N21b-36` (U21b, M0, task) frob-kernels: double-float accumulators `f32dd` with the split product, and the start-up canaries `K-canary`  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U21b/N21b-36.md`
- `N35-18` (U35, M2, task) The bans in batched kernels: ban table, banned-construct corpus, compile-option audit, canaries  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U35/N35-18.md`

- **pair** `N21a-10` / `N35-18`
  - reported by G21: the ban table and banned-construct corpus of batched kernels against the dialect lint and its planted-defect corpus
  - reported by G35: overlap: ban table and banned-construct corpus here, the lint and its planted-defect corpus there
- **pair** `N21b-36` / `N35-18`
  - reported by G21: the start-up canaries appear in both
  - reported by G35: overlap: start-up canaries in both (seven ban canaries here, the double-float canaries there)

## Cluster of 2 beads

- `BD-058` (U41, M1, feature) verb design.edit: the typed-edit engine with preconditions, inverses and declared symmetry effects  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U41/BD-058.md`
- `N14a-17` (U14a, M1, task) Typed-edit catalogue conformance: 22 rows, declared symmetry effect and predicted invalidation scope tested  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U14a/N14a-17.md`

- **pair** `BD-058` / `N14a-17`
  - reported by G41: registries/edits.toml: N14a-17 lists the 22-row file in its scope and depends on BD-058, whose acceptance compares the file's kinds with its EditOp kinds; BD-058's Scope now says it enters the rows it loads and N14a-17 completes and tests them

## Cluster of 2 beads

- `BD-165` (U18a, M2, feature) frob-scene: terrain generators with a scalar difficulty, deterministic across platforms (`scene.gen`)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U18a/BD-165.md`
- `N19a-28` (U19a, M2, task) frob-scene terrain: the ten family definitions with a scalar difficulty, and the curriculum grid of tiles  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U19a/N19a-28.md`

- **pair** `BD-165` / `N19a-28`
  - reported by G18: both define the terrain families with a scalar difficulty and the curriculum grid; N19a-28 (and N19a-27) are components that depend on the seed
  - reported by G19: seed and components: BD-165's Scope includes the terrain families, the difficulty map and the curriculum grid that N19a-27 and N19a-28 specify in full, and the outdoor half of the mixed route of N19a-29

## Cluster of 2 beads

- `BD-302` (U25e, M3, feature) verbs report.design, report.run, report.campaign, report.mission: evidence packages from the artifact graph  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U25e/BD-302.md`
- `N28c-44` (U28c, M3, feature) verbs design.diff --reuse and report.campaign: three named reuse columns; an unnamed reuse figure is refused  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U28c/N28c-44.md`

- **pair** `BD-302` / `N28c-44`
  - reported by G25cde: overlap: both specify the three named reuse columns of report.campaign and the refusal of an unnamed reuse figure
  - reported by G28: both specify the three named reuse columns of report.campaign and the refusal of an unnamed figure

## Cluster of 2 beads

- `BET-16` (U00, M4, task) Decide BET-16: inheritance (a child policy tuned from its parent needs a small share of from-scratch steps)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U00/BET-16.md`
- `N23a-30` (U23a, M4, task) Kill-gate decision KILL-INHERIT: keep inheritance along design edits or train children from prior or scratch  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U23a/N23a-30.md`

- **pair** `BET-16` / `N23a-30`
  - reported by G00: overlap with distinct scope, no merge: both record the decision KILL-INHERIT. The kill-gate bead applies the trigger and writes the kill ADR; the bet bead reads that record, evaluates the bet's falsifier clauses and writes the bet verdict. The bet bead's acceptance line "the KILL-INHERIT decision recorded" means on record by the other bead. The dependency of the bet on this bead is in place.

## Cluster of 2 beads

- `GB-39` (UEG, M3, task) card GB-39 (frob-verbs): how long a 10 s job waits behind a training run: 2,000 s, 621 s, or 0.17 s  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/GB-39.md`
- `N32a-20` (U32a, M3, task) frob-conformance: dispatcher queueing model: M/G/1 closed forms, event simulator, Kingman capacity table  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U32a/N32a-20.md`

- **pair** `GB-39` / `N32a-20`
  - reported by G32: the same queueing figures (2,001 s, 621 s, 0.17 s; KA-758, CX-351): the dispatcher model with its event simulator here, the worked card there
  - reported by GEG: same queueing figures (2,000 s, 621 s, 0.17 s; KA-758): other is the dispatcher queueing model with event simulator in frob-conformance (it also registers CX-351), mine the card whose lane B reaches it through BD-296

## Cluster of 2 beads

- `N10a-16` (U10a, M3, task) pilot C-08: holonomy on a three-link body and a wheeled snake against direct collocation  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U10a/N10a-16.md`
- `N25c-42` (U25c, M6, task) pilot: holonomy of the differential-drive loop and of a three-link kinematic crawler on the CPU referee  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U25c/N25c-42.md`

- **pair** `N10a-16` / `N25c-42`
  - reported by G10: overlap: holonomy pilot on a three-link body (M3 here against direct collocation; M6 there on the CPU referee)
  - reported by G25cde: overlap: both are paper-scale holonomy pilots on a three-link body and a wheeled base; N10a-16 (construction C-08, M3) and N25c-42 (KILL-AUGUR-PHASE, M6)

## Cluster of 2 beads

- `N12-19` (U12, M8, feature) frob-cert: Moonshot-tier checkers for tube-inclusion and alpha-root (parked until funded)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U12/N12-19.md`
- `N22b-35` (U22b, M8, feature) frob-shield, frob-cert: belief-set table lookup, proof kind `tube-inclusion`, GATE-SHIELD-05 on braking [M]  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U22b/N22b-35.md`

- **pair** `N12-19` / `N22b-35`
  - reported by G12: both specify the tube-inclusion checker; N12-19 holds the auditor's rule, N22b-35 the lookup stage, the writer and the gate campaign
  - reported by G22: both write the checker of proof kind `tube-inclusion` (N12-19 parks it at M8 with alpha-root); N22b-35 also has the belief-set lookup and GATE-SHIELD-05

## Cluster of 2 beads

- `N16b-21` (U16b, M2, task) frob-dyn: the missed-contact audit, event location and certified CCD on sampled windows of lockstep runs  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U16b/N16b-21.md`
- `N18b-23` (U18b, M2, feature) verb contact.audit: the tunnelling audit on sampled windows with certified collision detection  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U18b/N18b-23.md`

- **pair** `N16b-21` / `N18b-23`
  - reported by G16: both run certified collision detection on sampled windows of lockstep runs to count missed contacts; N18b-23 is the verb contact.audit, N16b-21 the fidelity-ledger quantity
  - reported by G18: certified collision detection on sampled windows of lockstep runs: mine is the verb contact.audit, the other the fidelity-ledger quantity built on it

## Cluster of 2 beads

- `N18b-37` (U18b, M3, task) Experiment OPEN-TRAC-1: dense bricks or a sparse brick tree for interiors at 1 cm, decided on measured memory  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U18b/N18b-37.md`
- `N19c-32` (U19c, M3, task) OPEN-HAB-3: decide default field spacings and truncation (2 cm, 16 cm, T = 8 cm) against 1 cm interiors at M3  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U19c/N19c-32.md`

- **pair** `N18b-37` / `N19c-32`
  - reported by G18: OPEN-TRAC-1 and OPEN-HAB-3 are decided from the same measured memory of compiled packs
  - reported by G19: OPEN-HAB-3 and OPEN-TRAC-1 are decided from the same measured memory (N18a-09 states the same experiment again)

## Cluster of 2 beads

- `N23b-13` (U23b, M3, task) e2e: an external PPO reaches the threshold through the adapter; GATE-CORTEX-16 completed, KILL-GYMADAPT read  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U23b/N23b-13.md`
- `N39a-22` (U39a, M3, task) frob-conformance tenfold: the learner-swap diagnostic that splits a train-phase difference into engine and learner parts  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U39a/N39a-22.md`

- **pair** `N23b-13` / `N39a-22`
  - reported by G23: both report learner-swap figures from an external PPO driven through the adapter

## Cluster of 2 beads

- `N26a-24` (U26a, M3, task) frob-nemesis: normal germs for card distributions, and the defensive mixture for discrete edits, exact weights  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U26a/N26a-24.md`
- `N35-25` (U35, M3, feature) frob-core + frob-fsim: addressed proposal draws with exactly computed importance weights  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U35/N35-25.md`

- **pair** `N26a-24` / `N35-25`
  - reported by G35: both build proposals on normal germs with a defensive mixture and exact weights

## Cluster of 2 beads

- `N31a-51` (U31a, M5, feature) verbs foundry.*: the seven-verb cluster: schemas, bounded wait, corrected refusal lists, CLI and MCP parity  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U31a/N31a-51.md`
- `VERBS-foundry` (UJL, M4, task) frob-verbs registry: cluster `foundry`, seven rows, with parity, e2e and schema-snapshot checks  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UJL/VERBS-foundry.md`

- **pair** `N31a-51` / `VERBS-foundry`
  - reported by G31: both enter the seven registry rows of cluster foundry; here N31a-51 depends on VERBS-foundry and adds schemas, the corrected refusal lists, the bounded wait and parity
  - reported by GJL: both specify the seven-verb foundry cluster: schemas, corrected refusal lists, CLI and MCP parity

## Cluster of 2 beads

- `N38b-56` (U38b, M1, task) frob-conformance: reconciliation value 7: failure-free episodes for the tenth request, 76,959 against 76,963  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U38b/N38b-56.md`
- `NEG-04` (UEG, M3, task) discrepancy E7-1 resolved: the tenth qualification request needs 76,959 clean episodes for 10⁻⁴, not 76,963  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/NEG-04.md`

- **pair** `N38b-56` / `NEG-04`
  - reported by G38: both settle the tenth qualification request at 76,959 failure-free episodes
  - reported by GEG: both pin 76,959 against 76,963 for the tenth request; N38b-57 holds the companion control row

