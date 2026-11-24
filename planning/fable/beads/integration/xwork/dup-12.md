# Cross-group overlap packet dup-12

36 pairs in 21 clusters. Each cluster is a set of beads connected by reported overlaps; decide a cluster as a whole.

## Cluster of 6 beads

- `N10a-02` (U10a, M0, task) frob-conformance: doctrine D-6, D-10, D-17 checks — moonshot cap, pilot and ceiling, generic comparator  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U10a/N10a-02.md`
- `N40-05` (U40, M0, task) exit protocol: the fresh-context review procedure, exit record and runner shared by all nine milestone exits  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U40/N40-05.md`
- `N41-01` (U41, M0, task) frob-conformance: the risk register as data, its four usage rules, the risk review recorded at every exit  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U41/N41-01.md`
- `N41-03` (U41, M0, task) frob-conformance: the open-question register (161 questions): decision rule as loader checks, defaults as ADRs  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U41/N41-03.md`
- `N41-09` (U41, M5, task) Owner decision OPEN-PROG-5: the order in which Moonshot items become active under the cap of four; cap check  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U41/N41-09.md`
- `NAB-21` (UAB, M0, task) registries/ev_matrix.toml: the Moonshot queue: 21 waiting items, order of entry and the cap of four  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UAB/NAB-21.md`

- **pair** `N10a-02` / `N41-09`
  - reported by G10: overlap: the cap-of-four check on active Moonshot items is implemented in both
  - reported by G41: the cap of four active Moonshot items is checked in both (N10a-02 as doctrine D-6; N41-09 as the Moonshot register's activation rule)
- **pair** `N10a-02` / `NAB-21`
  - reported by G10: both carry the Moonshot queue of 21 items and the cap of four as data (moonshots.toml here, ev_matrix.toml there); one file should hold it
  - reported by GAB: both hold the Moonshot queue of 21 items and the cap of four as data (registries/ev_matrix.toml here, a queue table there); one file should hold it and the other read it
- **pair** `N40-05` / `N41-01`
  - reported by G40: the risk-review step of an exit (read every row, a fired signal needs an outcome, otherwise the exit is invalid) is specified in both: as step 9 of the exit protocol and as the review step of the risk register
  - reported by G41: the fired-signal block (a fired risk signal with no recorded outcome invalidates the exit) is specified and tested in both: N41-01 as risk-review.sh, N40-05 as step 9 and an acceptance line of the runner
- **pair** `N40-05` / `N41-03`
  - reported by G41: the open-question block (a question at its deadline with neither an answer nor a default ADR blocks the exit) is specified and tested in both
- **pair** `N40-05` / `N41-09`
  - reported by G41: step 11 of the exit procedure counts active Moonshot items and blocks at five; N41-09 implements the same cap on its register

## Cluster of 4 beads

- `GB-09` (UEG, M0, task) card GB-09 (frob-conformance): which single strategy can reach ten-fold: three ceilings, none of which does  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/GB-09.md`
- `N00-17` (U00, M0, task) frob-conformance: the thesis arithmetic: exact ceiling calculators with known answers KA-001, KA-009 to KA-016  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U00/N00-17.md`
- `N36-04` (U36, M0, task) frob-conformance perf: the Amdahl limits calculator, recomputed from the measured incumbent ledger  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U36/N36-04.md`
- `N39a-37` (U39a, M1, task) frob-conformance tenfold scorer: the six ceilings computed from a ledger-closed run of the rival, before any speed test  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U39a/N39a-37.md`

- **pair** `GB-09` / `N36-04`
  - reported by GEG: same single-phase ceilings (7.7 from the 0.87 step share and the others, KA-818). Other is the Amdahl limits calculator of the performance chapter (seven limits); mine is the three-ceiling card on N00-17's calculators
- **pair** `N00-17` / `N36-04`
  - reported by G00: both implement the Amdahl ceiling calculators in frob-conformance with exact known answers (N00-17: nine calculators and two scale helpers, KA-009 to KA-016; N36-04: seven functions, KA-818 to KA-820). One module should hold the arithmetic
  - reported by G36: the Amdahl ceiling calculators with exact known answers exist in both (seven limits, KA-818 to KA-820 here; nine calculators, KA-009 to KA-016 there), and again over ledger records in N39a-37 and on card GB-09; one module should hold the arithmetic
- **pair** `N00-17` / `N39a-37`
  - reported by G00: both compute the thesis ceilings from a measured phase ledger (N39a-37: six calculators over ledger records, M1). N39a-37 should call the functions of N00-17 (M0) and keep only the ledger binding and the ceiling record
  - reported by G39: the exact thesis calculators overlap the ceilings; N39a-37 reuses them where they coincide
- **pair** `N36-04` / `N39a-37`
  - reported by G39: ceiling calculators from the measured incumbent ledger in two modules (six ceilings here, seven Amdahl limit functions there)

## Cluster of 4 beads

- `BD-227` (U14a, M3, feature) frob-scene: hazard registries `registries/hazards/<archetype>.toml` with provenance; the verb scene.hazards  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U14a/BD-227.md`
- `N19a-31` (U19a, M3, task) registries/hazards/home.toml: the ten Home hazards, their mechanisms and the draw-and-represent test  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U19a/N19a-31.md`
- `N19a-39` (U19a, M3, task) frob-scene: provenance promotion of hazard dimensions against the source register; FROB-REF-312 path  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U19a/N19a-39.md`
- `NEG-28` (UEG, M3, task) registry: the consolidated hazard list: 59 hazards in seven archetypes, every dimension `assumed`  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/NEG-28.md`

- **pair** `BD-227` / `N19a-31`
  - reported by G14: BD-227 lists the 59 hazard rows of all seven registry files in its scope; N19a-31 to N19a-37 each write one file's rows (they depend on BD-227 for the loader and the verb)
  - reported by G19: seed and components: BD-227's Scope includes the seven registry files with the 59 rows; N19a-31 to N19a-37 enter those rows per archetype with their draw-and-represent tests and depend on BD-227 for the schema and loader
- **pair** `BD-227` / `N19a-39`
  - reported by G14: the realism-claim check behind FROB-REF-312 is in both; N19a-39 depends on BD-227 and owns promotion against the source register
  - reported by G19: the realism-claim check behind FROB-REF-312 is in both; N19a-39 owns promotion against the source register and the refusal path
- **pair** `BD-227` / `NEG-28`
  - reported by G14: the consolidated list of the same 59 hazards
  - reported by GEG: the same 59 hazards: other has the seven registry files, loader and scene.hazards (N19a-31 to -37 write the rows), mine the consolidated INDEX.toml, the agreement check and the `Covered by` column

## Cluster of 4 beads

- `N33a-03` (U33a, M0, task) xtask: `franken-stack.lock` schema, strict parser and validation rules  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U33a/N33a-03.md`
- `N33a-04` (U33a, M0, task) xtask: the resolution receipt `frob.stack-receipt/1`: canonical writer, tree hashes, drift classes  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U33a/N33a-04.md`
- `N33a-05` (U33a, M0, task) xtask check-stack: comparison with the newest receipt, the dirty-tree PASS guard, no sibling written by tools  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U33a/N33a-05.md`
- `N34-13` (U34, M0, task) xtask check-stack: stack lock against lockfiles, toolchain files and sibling heads; receipt; drift classes  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U34/N34-13.md`

- **pair** `N33a-03` / `N34-13`
  - reported by G34: both specify the franken-stack.lock fields and its strict reader; N34-13 adds the [json_library] and [auditor] tables
- **pair** `N33a-04` / `N34-13`
  - reported by G33: true duplicate across units: N34-13 holds the check-stack subcommand, the lock and receipt readers, the drift classifier and the receipt writer, which N33a-03 (parser), N33a-04 (receipt, drift classes) and N33a-05 (comparison, PASS guard) specify in more detail
  - reported by G34: both specify the resolution receipt frob.stack-receipt/1: fields, tree hash of a dirty sibling, drift classes, duplicate-package scan. They disagree on the writer: frob-core's canonical writer here, a small local writer in xtask there
- **pair** `N33a-05` / `N34-13`
  - reported by G34: both specify xtask check-stack: the verify and write modes, the comparison of lock, newest receipt and trees, the dirty-tree rule and the read-only sibling rule

## Cluster of 3 beads

- `BD-305` (U37, M3, feature) frob-chronicle/frob-verbs: private places: private root, absent from views, read and export scopes; GATE-SAFE-04, -05  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U37/BD-305.md`
- `N29b-14` (U29b, M3, task) frob-chronicle: telemetry of a private scene keeps the scene's privacy class through retention, export and purge  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U29b/N29b-14.md`
- `N37-19` (U37, M3, feature) verb scene.purge: remove a private scene pack and the lineage closure of everything derived from it, with a receipt  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U37/N37-19.md`

- **pair** `BD-305` / `N29b-14`
  - reported by G29: the private mark applied to Chronicle's stored objects (listing filter, read check, sweep) is in both; N29b-14 depends on BD-305
  - reported by G37: N29b-14 applies the private mark, the read and export checks and the listing filters to Chronicle's stored objects, which BD-305's listing filter, dispatcher read check and kind sweep also cover; N29b-14 depends on BD-305
- **pair** `N29b-14` / `N37-19`
  - reported by G29: purge against pinned evidence: N29b-14 (with BD-209) refuses with FROB-REF-813 unless --withdraw-claims is passed; N37-19 withdraws inside the purge and never raises 813; one rule must govern
  - reported by G37: the two disagree on purge against pinned evidence: N37-19 withdraws the claims and releases the pins inside the purge transaction and never raises FROB-REF-813; N29b-14 refuses with 813 unless --withdraw-claims is passed, and the registry row of scene.purge in VERBS-scene lists 813 among its refusals

## Cluster of 3 beads

- `GB-24` (UEG, M1, task) card GB-24 (frob-mission, frob-sense): how bad the sensor may be when the first move is a probe: ε < (U + ½)/4  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/GB-24.md`
- `N10b-08` (U10b, M1, task) spec C-19 part (frob-sense): the docking fixture and its closed-form boundary ε < (U + ½)/4  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U10b/N10b-08.md`
- `N20b-13` (U20b, M1, task) frob-conformance: the docking fixture in exact rationals: boundary, 561-cell grid, late-packet exclusion  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U20b/N20b-13.md`

- **pair** `GB-24` / `N20b-13`
  - reported by GEG: same docking fixture: 561 cells, 344 admitted, 217 excluded, 9 on the boundary, worked commands, the non-strict mutant (353). Other has the exact evaluator, label digest and derivation file; mine the card file, lane B through the frob-mission family solver. N20b-14 has the third gain and single-precision controls; BD-041 the plan-time re-run
- **pair** `N10b-08` / `N20b-13`
  - reported by G20: the conformance fixture and the spec C-19 part both write the docking model, its closed-form boundary and its law tests; N20b-13 is recorded as depending on the spec

## Cluster of 3 beads

- `N17a-15` (U17a, M2, task) pilot: survivor specialization: gate to fund (share s ≥ 1/3) and the three-arm whole-step pilot  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U17a/N17a-15.md`
- `N21c-16` (U21c, M2, task) Measurement: the specializable share of the step from the M2 profile (the profile clause of KILL-SPECIALIZE)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U21c/N21c-16.md`
- `N21c-17` (U21c, M2, task) Pilot: three kernel arms on the 12-joint quadruped at 8,192 worlds: padded, unpadded, unrolled and folded  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U21c/N21c-17.md`

- **pair** `N17a-15` / `N21c-16`
  - reported by G17: both read the specializable share s from the M2 profile for KILL-SPECIALIZE
  - reported by G21: the gate to fund specialization from the specializable share (s at least one third) is drafted twice
- **pair** `N17a-15` / `N21c-17`
  - reported by G17: same three-arm specialization pilot on the 12-joint quadruped at 8,192 worlds
  - reported by G21: the three-arm survivor-specialization pilot is drafted twice (M2 here, M3 there)

## Cluster of 3 beads

- `N33b-03` (U33b, M7, task) franken-stack.lock: the tournament freeze of every pin and of T, enforced by check-stack and the harness  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U33b/N33b-03.md`
- `N39b-13` (U39b, M7, task) The tournament at M7: the order of work, the pin freeze, the breach rules and the record of GATE-TENFOLD-07  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U39b/N39b-13.md`
- `N40-16` (U40, M7, task) program gate GATE-TENFOLD-07: the tournament at M7, published with its ledgers and kit whatever it says  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U40/N40-16.md`

- **pair** `N33b-03` / `N39b-13`
  - reported by G33: overlap with distinct scope: N33b-03 is the freeze mechanism in the stack lock, check-stack and the harness rule; N39b-13 the tournament's order of work, which applies and verifies the freeze
- **pair** `N39b-13` / `N40-16`
  - reported by G39: both record GATE-TENFOLD-07
  - reported by G40: both record GATE-TENFOLD-07 with the freeze check and the outcome rules at M7; N40-16 is made to depend on N39b-13

## Cluster of 2 beads

- `BD-110` (U28a, M1, feature) verb design.check, stages 1 to 6: the stage runner, aggregation, ranked repairs as typed edits, per-stage timing  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U28a/BD-110.md`
- `N14b-10` (U14b, M1, task) frob-analyze: the preflight staging contract: eight stages, their refusals, budgets, undecided, check options  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U14b/N14b-10.md`

- **pair** `BD-110` / `N14b-10`
  - reported by G14: stage table, statuses, `--stages`/`--all`/`--strict-data` and timing budgets are specified in both; edge BD-110 → N14b-10 recorded
  - reported by G28: stage table, statuses, check options and timing budgets are specified in both; BD-110 -> N14b-10 recorded by the Skeleton integrator, and BD-108 now depends on N14b-10 too

## Cluster of 2 beads

- `BD-246` (U14b, M3, feature) frob-design: the remaining fourteen archetypes: groups on file, mass rows and the planted asymmetries  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U14b/BD-246.md`
- `NEG-30` (UEG, M1, task) zoo: the `zoo/` directory of pinned archetype instances, `frob.zoo.expect/1` and its conformance test  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/NEG-30.md`

- **pair** `BD-246` / `NEG-30`
  - reported by GEG: both hold per-archetype expectations for all eighteen (group on file, mass rows) behind GATE-SKEL-03: other in the catalog and its gate record, mine as zoo/ with frob.zoo.expect/1 and the conformance test

## Cluster of 2 beads

- `BET-03` (U00, M3, task) Decide BET-03: convex compliant contact, audited by a referee, is good enough for locomotion decisions  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U00/BET-03.md`
- `N18b-33` (U18b, M2, task) Kill-gate decision KILL-CONTACT: keep CL-1 as default, make CL-2 the default, or claim locomotion on the referee only  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U18b/N18b-33.md`

- **pair** `BET-03` / `N18b-33`
  - reported by G00: overlap with distinct scope, no merge: both record the decision KILL-CONTACT. The kill-gate bead applies the trigger and writes the kill ADR; the bet bead reads that record, evaluates the bet's falsifier clauses and writes the bet verdict. The bet bead's acceptance line "the KILL-CONTACT decision recorded" means on record by the other bead. The dependency of the bet on this bead is in place.

## Cluster of 2 beads

- `BET-32` (U00, M3, task) Decide BET-32: rare-failure bounds (bounded-weight importance sampling refutes, tightens; never cheap absence)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U00/BET-32.md`
- `N26a-32` (U26a, M3, task) Decision KILL-RARE: keep splitting, or fall back to crude nominal sampling with the binomial bound alone  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U26a/N26a-32.md`

- **pair** `BET-32` / `N26a-32`
  - reported by G00: overlap with distinct scope, no merge: both record the decision KILL-RARE. The kill-gate bead applies the trigger and writes the kill ADR; the bet bead reads that record, evaluates the bet's falsifier clauses and writes the bet verdict. The bet bead's acceptance line "the KILL-RARE decision recorded" means on record by the other bead. The dependency of the bet on this bead is in place.

## Cluster of 2 beads

- `N00-19` (U00, M1, task) frob-conformance: the ten-fold metric contract and a conformance test for any report that states a ratio  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U00/N00-19.md`
- `N39a-36` (U39a, M3, task) frob-conformance tenfold: the report record frob.tenfold.report/1: descriptive tables always, no bare ratio  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U39a/N39a-36.md`

- **pair** `N00-19` / `N39a-36`
  - reported by G39: N00-19 specifies a typed stratum-report record and a conformance test that every stated ratio carries its fields, which is INV-TENFOLD-08 and the ReportedRatio type of N39a-36

## Cluster of 2 beads

- `N10b-14` (U10b, M4, task) spec C-23 part (frob-learn): transport of a policy along a typed edit: correspondences and composition  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U10b/N10b-14.md`
- `N23a-27` (U23a, M4, task) frob-learn: InheritanceMap: transport of a parent's policy to a child design through the typed edit [F]  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U23a/N23a-27.md`

- **pair** `N10b-14` / `N23a-27`
  - reported by G23: N10b-14 specifies the transport along a typed edit that N23a-27 implements, with the same types and refusal

## Cluster of 2 beads

- `N14b-12` (U14b, M3, feature) frob-analyze: preflight stage 7 (place): reach, coverage, swept-body clearance; refusal 119 with bottleneck  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U14b/N14b-12.md`
- `N19b-30` (U19b, M3, task) frob-scene: route verdicts: height slabs, transition predicates, fixture modes, witness labels, FROB-REF-308  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U19b/N19b-30.md`

- **pair** `N14b-12` / `N19b-30`
  - reported by G14: stage 7 restates the height slab, the transition predicates and the door-push predicate that N19b-30 defines
  - reported by G19: preflight stage 7 restates the height slab, the transition capability predicates and the door push predicate that N19b-30 defines; N14b-12 should call N19b-30 through A-18 (N19b-34)

## Cluster of 2 beads

- `N18-CX` (U18b, M1, task) frob-conformance: Traction counterexample registry CX-167 to CX-183, seeding the negative-control registry  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U18b/N18-CX.md`
- `N38b-05` (U38b, M0, task) frob-conformance: seed the negative-control registry from 384 counterexamples and 510 stated control cells  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U38b/N38b-05.md`

- **pair** `N18-CX` / `N38b-05`
  - reported by G18: both seed controls.toml with CX-167 to CX-183; N38b-05 should take the rows from N18-CX
  - reported by G38: both write controls.toml records for CX-167 to CX-183: N38b-05 is the seeding tool and map, the entries are the chapter bead's; the same holds for every N<chapter>-CX

## Cluster of 2 beads

- `N19c-05` (U19c, M5, feature) verb scene.probe-request: the measurement that would settle an unresolved answer (box, quantity, accuracy)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U19c/N19c-05.md`
- `N25c-19` (U25c, M5, feature) verb scene.probe-request: the measurements that would settle an unresolved answer, with their accuracy  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U25c/N25c-19.md`

- **pair** `N19c-05` / `N25c-19`
  - reported by G19: both implement the verb scene.probe-request at M5 (record, three triggers, requested accuracy, privacy); N19c-05 is the Habitat owner per §19.13
  - reported by G25cde: true duplicate: both specify the verb scene.probe-request at M5

## Cluster of 2 beads

- `N25a-28` (U25a, M4, task) e2e mission_family_screens.sh: 65,536-row screen, 1,024-design batch, identical on CPU, CUDA, Metal (GATE-AUGUR-15)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U25a/N25a-28.md`
- `N25c-09` (U25c, M4, feature) frob-mission: placing family screens on CUDA, Metal and the host; the CPU checker queue in decision time  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U25c/N25c-09.md`

- **pair** `N25a-28` / `N25c-09`
  - reported by G25ab: overlap with distinct scope, connected by a dependency: N25c-09 places family screens on devices and host, N25a-28 is the script that asserts its placement receipts for GATE-AUGUR-15

## Cluster of 2 beads

- `N27b-09` (U27b, M3, task) e2e GATE-AUTOPSY-03: planted single causes through run.fork, autopsy.intervene and autopsy.attribute  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U27b/N27b-09.md`
- `N38b-15` (U38b, M3, task) frob-conformance: self-tests of failure attribution on planted causes and of conformal band coverage  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U38b/N38b-15.md`

- **pair** `N27b-09` / `N38b-15`
  - reported by G27: N38b-15 (self-test CERT-14) runs attribution on 200 planted single causes at the thresholds of GATE-AUTOPSY-03 (90% first with but-for; top factor changes in at most 30%), which N27b-09 measures on the suite of N27b-08. Suggested: N38b-15 reads the suite and the gate row and adds only its broken-certifier controls

## Cluster of 2 beads

- `N32b-05` (U32b, M3, task) e2e: many agents on one daemon: scope refusals, attenuation, namespace isolation, shared views  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U32b/N32b-05.md`
- `N37-15` (U37, M3, task) e2e: `scripts/e2e/safe_capabilities.sh`: scopes with no partial data, attenuation, namespaces, budgets, workers  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U37/N37-15.md`

- **pair** `N32b-05` / `N37-15`
  - reported by G32: two end-to-end scripts for the same mechanisms (scope refusals with no partial data, attenuation, namespaces): tether_capabilities.sh and safe_capabilities.sh
  - reported by G37: two end-to-end scripts for the same mechanisms (scope refusals with no partial data, attenuation, namespaces): safe_capabilities.sh and tether_capabilities.sh; N37-15 adds budgets and the worker manifest, N32b-05 the sealed-read and shared-view steps

## Cluster of 2 beads

- `N40-08` (U40, M2, task) gate register: shared fixtures run once and related thresholds under different identifiers, with a check  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U40/N40-08.md`
- `NEG-12` (UEG, M3, task) inconsistency E7-B resolved: both film gates hold the three squeeze times to 2%, the bound of the law's owner  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/NEG-12.md`

- **pair** `N40-08` / `NEG-12`
  - reported by GEG: both make GATE-TRAC-05 and GATE-HAB-06 hold the squeeze times to one 2% bound; N40-08 has the gate register's shared-fixture table and check, mine the single registration in the known-answer rows, the lint and the E7-B scan

