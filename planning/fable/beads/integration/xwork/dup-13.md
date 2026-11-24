# Cross-group overlap packet dup-13

35 pairs in 20 clusters. Each cluster is a set of beads connected by reported overlaps; decide a cluster as a whole.

## Cluster of 6 beads

- `N14b-04` (U14b, M3, feature) frob-design: M3 archetypes: tracked-flipper, wheel-leg, tri-wheel-stair, mobile-manipulator, snake-crawler  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U14b/N14b-04.md`
- `ZOO-mobile-manipulator` (UEG, M3, task) zoo/mobile-manipulator.fbot: archetype 16, seven-joint arm on a wheeled base: trivial symmetry, workspace  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/ZOO-mobile-manipulator.md`
- `ZOO-snake-crawler` (UEG, M3, task) zoo/snake-crawler.fbot: archetype 17, planar chain with passive wheels: C₂ by reversal, locomotion as holonomy  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/ZOO-snake-crawler.md`
- `ZOO-tracked-flipper` (UEG, M3, task) zoo/tracked-flipper.fbot: archetype 12, two tracks with four flippers: C₂ × C₂, belt law, scrub work in a pivot  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/ZOO-tracked-flipper.md`
- `ZOO-tri-wheel-stair` (UEG, M3, task) zoo/tri-wheel-stair.fbot: archetype 14, two tri-wheel clusters: C₂ outside, C₃ or D₃ inside, riser and tread  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/ZOO-tri-wheel-stair.md`
- `ZOO-wheel-leg` (UEG, M3, task) zoo/wheel-leg.fbot: archetype 13, quadruped with driven wheels as feet: C₂ × C₂, two contact laws per foot  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/ZOO-wheel-leg.md`

- **pair** `N14b-04` / `ZOO-mobile-manipulator`
  - reported by GEG: same off-centre arm, trivial group and iiwa7 presets in both
- **pair** `N14b-04` / `ZOO-snake-crawler`
  - reported by GEG: same holonomy rows (KA-254) on the curvature analysis BD-243 in both
- **pair** `N14b-04` / `ZOO-tracked-flipper`
  - reported by GEG: same belt-law and scrub-work rows (KA-282, KA-418) in both
- **pair** `N14b-04` / `ZOO-tri-wheel-stair`
  - reported by GEG: same cluster relation (riser 0.175 m, tread 0.260 m, ρ = 0.181 m) and the same unmet admission of fs-mbd::tri_wheel_cluster in both
- **pair** `N14b-04` / `ZOO-wheel-leg`
  - reported by GEG: same two-contact-laws-per-foot row (KA-283, FROB-REF-121) in both

## Cluster of 5 beads

- `N14b-02` (U14b, M3, feature) frob-design: legged archetypes of M3: quad-sprawl, hexapod-radial, octopod, biped-point, with known answers  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U14b/N14b-02.md`
- `ZOO-biped-point` (UEG, M3, task) zoo/biped-point.fbot: archetype 5, point-foot biped: C₂, no static stability reported and not refused  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/ZOO-biped-point.md`
- `ZOO-hexapod-radial` (UEG, M3, task) zoo/hexapod-radial.fbot: archetype 3, radial hexapod: D₆ (C₆ handed), 86,185 orbits, tripod gait as (H, K)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/ZOO-hexapod-radial.md`
- `ZOO-octopod` (UEG, M3, task) zoo/octopod.fbot: archetype 4, eight-legged body: D₈ radial or C₂ spider, contact capacity, the 36 bucket  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/ZOO-octopod.md`
- `ZOO-quad-sprawl` (UEG, M3, task) zoo/quad-sprawl.fbot: archetype 2, sprawled quadruped: stable on a wide stance, refused thermally at `@hot`  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/ZOO-quad-sprawl.md`

- **pair** `N14b-02` / `ZOO-biped-point`
  - reported by GEG: same `no static stability: reported, not refused` row and C₂ group in both
- **pair** `N14b-02` / `ZOO-hexapod-radial`
  - reported by GEG: same orbit counts 86,185, 166,870 and 500,500, D₆ and C₆ handed, in both
- **pair** `N14b-02` / `ZOO-octopod`
  - reported by GEG: same D₈ and C₂ spider groups, over-capacity refusal at 24 contacts and bucket 36 in both
- **pair** `N14b-02` / `ZOO-quad-sprawl`
  - reported by GEG: same `@hot` thermal refusal FROB-REF-132 and thermal formula row (0.156; 72.4 °C) in both

## Cluster of 4 beads

- `BD-269` (U35, M3, feature) Determinism licences and the labelled D4 profile at their gates (GATE-SPARK-17, GATE-SAFE-09)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U35/BD-269.md`
- `N23a-18` (U23a, M2, task) frob-learn: where learning runs: placement, single precision, the labeled D4 profile, the learner's share  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U23a/N23a-18.md`
- `N35-06` (U35, M3, feature) The `fast-training` profile: labelled class D4, recorded knobs, the `trained_under` tag and FROB-REF-861  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U35/N35-06.md`
- `N37-28` (U37, M3, task) frob-verbs/frob-learn: a D4 result is never evidence: labels, FROB-REF-825 on five paths, weights as data  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U37/N37-28.md`

- **pair** `BD-269` / `N37-28`
  - reported by G35: five-path D4 refusal: measured here, built there (capstone and component, edge recorded)
  - reported by G37: both run the five-path FROB-REF-825 test with the unlabelled plant and record GATE-SAFE-09; BD-269 is the seed that names the gate, N37-28 builds the refusal and the label rules (edge BD-269 -> N37-28 recorded)
- **pair** `N23a-18` / `N35-06`
  - reported by G35: overlap: the profile product-wide here (861), the trainer's handling there (867)
- **pair** `N35-06` / `N37-28`
  - reported by G35: both put class D4 and the profile name on every result, checkpoint and policy of a fast-training job and state the re-evaluation rule
  - reported by G37: both put class D4 and the profile name on every result, checkpoint and exported policy of a fast-training job and state the re-evaluation rule

## Cluster of 4 beads

- `N33a-10` (U33a, M0, task) KILL-FSIM trigger ledger: the INT-FSIM-01 effort clock, the cold-check envelope, the upstream breakage share  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U33a/N33a-10.md`
- `N33b-04` (U33b, M0, task) compatibility train: the pin-advance procedure, its train log with the breakage share, and a rehearsal  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U33b/N33b-04.md`
- `NF-01` (UF, M0, task) suite: the compatibility train: pin-move procedure, script, train log and guards  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/NF-01.md`
- `NF-06` (UF, M0, task) e2e: train rehearsal on a scratch stack: pin move, upstream switch-over, toolchain move, every guard  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/NF-06.md`

- **pair** `N33a-10` / `NF-01`
  - reported by GF: the upstream breakage share over four weeks is computed in both: from the train log here, in the KILL-FSIM trigger ledger there
- **pair** `N33b-04` / `NF-01`
  - reported by G33: true duplicate: both write the compatibility-train procedure, its script, the train log and the breakage share (NF-06 also holds the rehearsal). The log is registries/trains.toml here and receipts/train-log.ndjson there; one must be chosen. N33b-04 now takes its breakage definition from N33a-10 and states this repository's side
  - reported by GF: both specify the compatibility train: the procedure as a script with its refusals and abort path, the list of gates a train re-runs, the train log and its schema, and the breakage share that KILL-FSIM reads; one should keep it (N26b-24 already depends on both)
- **pair** `N33b-04` / `NF-06`
  - reported by GF: the rehearsal of a pin move on scratch stacks is a bead here and the last part of N33b-04 there

## Cluster of 3 beads

- `BD-312` (U32a, M3, feature) frob-verbs: the M3 verbs with schemas, examples and refusal lists; the registry freeze; GATE-TETHER-02  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U32a/BD-312.md`
- `N27b-16` (U27b, M3, task) frob-verbs: registry entries for the autopsy cluster: five verbs, schemas, refusal lists, task clusters  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U27b/N27b-16.md`
- `VERBS-autopsy` (UJL, M3, task) frob-verbs registry: cluster `autopsy`, five rows, with parity, e2e and schema-snapshot checks  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UJL/VERBS-autopsy.md`

- **pair** `BD-312` / `N27b-16`
  - reported by G27: BD-312 enters registry entries, schemas, examples, refusal lists and task-cluster assignments for the whole M3 set and freezes the registry; for the autopsy cluster that is N27b-16's content. BD-312 should take the cluster from N27b-16 (edge recorded). N27b-16 proposes a task cluster `repair` for autopsy.repair and autopsy.rerun, which sit in no cluster of the verb appendix; the assignment belongs to NJL-05
- **pair** `N27b-16` / `VERBS-autopsy`
  - reported by G27: both write the five registry entries of the cluster autopsy (kinds, schema names, refusal lists, task clusters) with parity and schema checks. N27b-16 also holds the tests that need the verbs (examples execute, refusal lists cover raised codes, the unknown-verb repair for `autopsy fork`). Suggested: VERBS-autopsy keeps the rows as data, N27b-16 keeps the tests and depends on it (edge recorded)
  - reported by GJL: both enter the five registry entries of the autopsy cluster with schemas, refusal lists and task clusters

## Cluster of 3 beads

- `N00-09` (U00, M1, task) frob-conformance: funding record per work package: measured rival first, ceiling before speed, no closed hopes  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U00/N00-09.md`
- `N39a-38` (U39a, M1, task) frob-conformance tenfold: GATE-TENFOLD-09 measured: registered speed comparisons, the funding and order rules  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U39a/N39a-38.md`
- `N40-18` (U40, M1, task) program gate GATE-TENFOLD-09: ceiling before speed, program-wide, at M1 and re-run at every exit  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U40/N40-18.md`

- **pair** `N00-09` / `N39a-38`
  - reported by G00: both check work-package funding against ceilings and the 10% lever rule: N00-09 defines the funding record and its admission refusals; N39a-38 measures GATE-TENFOLD-09 with a funding-rule check and an order-rule check. N39a-38 should read the record of N00-09 (edge recorded)
  - reported by G39: the funding-rule check over work packages is specified in both (N00-09 also names GATE-TENFOLD-09)
- **pair** `N39a-38` / `N40-18`
  - reported by G39: both record GATE-TENFOLD-09 and check registered speed comparisons and funding justifications
  - reported by G40: both implement the two clauses of GATE-TENFOLD-09 (the ceiling a speed comparison must carry, the funding rule over work packages), the refusal and the gate record; N40-18 adds the re-run at every exit. Close to a true duplicate

## Cluster of 3 beads

- `N17b-11` (U17b, M2, task) Decide OPEN-PHYLUM-1, -2 and -5 by measurement: bucket set, waste limit, run length, contact axis  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U17b/N17b-11.md`
- `N21b-18` (U21b, M2, task) Decision OPEN-SPARK-3: freeze the bucket set and the capacities of each reference workload at M2  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U21b/N21b-18.md`
- `NEG-11` (UEG, M2, task) inconsistency E7-A resolved: default capacities C = 32, R = 160 and C = 64, R = 320, read from one table  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/NEG-11.md`

- **pair** `N17b-11` / `N21b-18`
  - reported by G17: OPEN-PHYLUM-1 and OPEN-SPARK-3 freeze the same bucket set; one record must close both
  - reported by G21: OPEN-SPARK-3 and OPEN-PHYLUM-1 freeze the same bucket set; one registry file should serve both
- **pair** `N21b-18` / `NEG-11`
  - reported by GEG: both fix the default capacities C = 32, R = 160 and C = 64, R = 320; N21b-18 is the decision OPEN-SPARK-3 that freezes the table, mine pins the single table and the stale 24/128 and 40/224

## Cluster of 3 beads

- `N39b-26` (U39b, M2, task) Decision OPEN-TENFOLD-1: which simulator adjudicates each stratum (terrain, films, tracks)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U39b/N39b-26.md`
- `N39b-27` (U39b, M2, task) Owner decision OPEN-TENFOLD-2: who the adjudicator of the ten-fold test is, or the self-adjudicated default  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U39b/N39b-27.md`
- `N41-06` (U41, M2, task) Owner decision OPEN-PROG-2, OPEN-TENFOLD-1, -2: adjudicating simulator and neutral party of the ten-fold test  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U41/N41-06.md`

- **pair** `N39b-26` / `N41-06`
  - reported by G39: both decide OPEN-TENFOLD-1, the adjudicating simulator per stratum (N41-06 decides it together with OPEN-PROG-2)
  - reported by G41: both decide OPEN-TENFOLD-1 (which simulator adjudicates each stratum); N41-06 is the packet put to the owner, N39b-26 the ten-fold unit's decision bead
- **pair** `N39b-27` / `N41-06`
  - reported by G39: both decide OPEN-TENFOLD-2, who the adjudicator is, with the self-adjudicated default
  - reported by G41: both decide OPEN-TENFOLD-2 (who the adjudicator is, or the self-adjudicated default)

## Cluster of 2 beads

- `BD-126` (U24b, M1, feature) verbs mission.new, mission.check: the `.fmission` schema and admission, with the quantifier order echoed  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U24b/BD-126.md`
- `N25a-02` (U25a, M1, feature) frob-mission: semantic admission of a .fmission query: per-field rules, MissionId sensitivity, admission refusals  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U25a/N25a-02.md`

- **pair** `BD-126` / `N25a-02`
  - reported by G24: both specify admission of a .fmission query and its refusals 868, 874, 884, 887, 847; N25a-02 depends on BD-126 and leaves it parsing, canonical bytes and the verbs

## Cluster of 2 beads

- `BD-248` (U28b, M3, task) The design corpus authored and frozen: about 200 designs with planted defects, winners and single causes  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U28b/BD-248.md`
- `N24b-02` (U24b, M3, task) frob-arena: the design-corpus registry (`frob.corpus/1`): frozen labels, the held-out third, corpus identity  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U24b/N24b-02.md`

- **pair** `BD-248` / `N24b-02`
  - reported by G24: the freeze of the design corpus is claimed by both; N24b-02 holds the registry, the split and the freeze mechanics, BD-248 the content
  - reported by G28: the freeze of the design corpus is claimed by both: N24b-02 holds the registry, the split and the freeze mechanics, BD-248 the content

## Cluster of 2 beads

- `BET-04` (U00, M0, task) Decide BET-04: FrankenSim can be consumed by path from outside its tree, stably, on one toolchain  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U00/BET-04.md`
- `N33b-05` (U33b, M0, task) decision KILL-FSIM: keep FrankenSim by path, move the referee behind a worker process, or lift a referee set  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U33b/N33b-05.md`

- **pair** `BET-04` / `N33b-05`
  - reported by G00: overlap with distinct scope, no merge: both record the decision KILL-FSIM. The kill-gate bead applies the trigger and writes the kill ADR; the bet bead reads that record, evaluates the bet's falsifier clauses and writes the bet verdict. The bet bead's acceptance line "the KILL-FSIM decision recorded" means on record by the other bead. The dependency of the bet on this bead is in place.
  - reported by G33: overlap with distinct scope: N33b-05 applies the KILL-FSIM trigger and writes the decision; BET-04 reads it for the bet's verdict (as G00 recorded)

## Cluster of 2 beads

- `BET-34` (U00, M3, task) Decide BET-34: referee coverage (the referee completes decision scenes; the sampled referee covers enough)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U00/BET-34.md`
- `N16b-19` (U16b, M3, task) Decision KILL-SHADOW: keep the background referee, or run the referee at promotion time only  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U16b/N16b-19.md`

- **pair** `BET-34` / `N16b-19`
  - reported by G00: overlap with distinct scope, no merge: both record the decision KILL-SHADOW. The kill-gate bead applies the trigger and writes the kill ADR; the bet bead reads that record, evaluates the bet's falsifier clauses and writes the bet verdict. The bet bead's acceptance line "the KILL-SHADOW decision recorded" means on record by the other bead. The dependency of the bet on this bead is in place.

## Cluster of 2 beads

- `N00-20` (U00, M1, task) frob-conformance: the lever register and expected ranges by mission class, pre-registered, scored at M3 and M7  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U00/N00-20.md`
- `N39a-26` (U39a, M1, task) frob-conformance tenfold: reading the ledger: lever attribution, end-to-end rows 20 to 22, lost-factor records  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U39a/N39a-26.md`

- **pair** `N00-20` / `N39a-26`
  - reported by G00: both attribute measured phase savings to the eight levers: N00-20 pre-registers the lever register and expected ranges and scores them at M3 and M7; N39a-26 computes the removable share and the saving table per lever from a ledger. Distinct scope; N00-20's scoring should read N39a-26's tables
  - reported by G39: lever attribution with a single-counting check is specified in both

## Cluster of 2 beads

- `N10b-15` (U10b, M4, task) pilot PILOT-C-23: policy transport on 12 parent–child edit pairs (4 blind) against training from scratch  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U10b/N10b-15.md`
- `N23b-20` (U23b, M3, task) Pilot [F]: inheritance along design edits: scratch, from parent, from prior, and a ten-edit chain  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U23b/N23b-20.md`

- **pair** `N10b-15` / `N23b-20`
  - reported by G10: overlap: policy inheritance along design edits against training from scratch
  - reported by G23: two pilots of policy transport before BD-322; N10b-15's 12 pairs with 4 blind are also the experiment of GATE-CORTEX-13 in BD-322

## Cluster of 2 beads

- `N14b-13` (U14b, M3, feature) frob-analyze: preflight stage 8 (information): the observation-alias search and refusal 120 with its repair  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U14b/N14b-13.md`
- `N20a-41` (U20a, M3, feature) verb sense.alias: a witness with its class and separating changes; standing witnesses and refusal 445  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U20a/N20a-41.md`

- **pair** `N14b-13` / `N20a-41`
  - reported by G14: N20a-41 also specifies the design-check labels `aliased-passive`/`aliased-all-probes` and refusal 120

## Cluster of 2 beads

- `N18a-03` (U18a, M3, feature) frob-contact: pairwise enclosure predicate per time slab: `clear`, `penetrates` or `unknown`, with certificates  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U18a/N18a-03.md`
- `N19b-33` (U19b, M3, feature) verb contact.clearance: clear, penetrates or unknown on one-sided enclosures per time slab (GATE-TRAC-07)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U19b/N19b-33.md`

- **pair** `N18a-03` / `N19b-33`
  - reported by G18: the three-answer enclosure decision per time slab: mechanism in mine, the verb in the other, which also claims the deciding step
  - reported by G19: both specify the three-answer enclosure predicate per time slab with certificates, FROB-REF-321 and GATE-TRAC-07; N18a-03 leaves the verb out, N19b-33 is the verb contact.clearance with the battery and the gate record (edge N19b-33 depends on N18a-03 recorded)

## Cluster of 2 beads

- `N19c-14` (U19c, M3, task) frob-scene: sealed scene families for the qualification lane: a handle that returns scores, never scenes  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U19c/N19c-14.md`
- `N24b-17` (U24b, M3, task) frob-arena: sealed world clusters: derivation, freshness, FROB-REF-844, close-time reveal, family retirement  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U24b/N24b-17.md`

- **pair** `N19c-14` / `N24b-17`
  - reported by G19: sealed families: key, commitment, derivation, reveal and retirement appear in both; N19c-14 owns the handle and the sealed pack wrapper in frob-scene, N24b-17 the freshness ledger and FROB-REF-844
  - reported by G24: seed derivation, reveal and retirement of sealed families appear in both; N19c-14 holds the scene side and the handle

## Cluster of 2 beads

- `N25b-05` (U25b, M0, task) e2e auditor_independence.sh: copied crate, offline build, no socket and no child process (GATE-WS-03)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U25b/N25b-05.md`
- `N34-25` (U34, M0, task) xtask check-deps: independent-auditor rules (L-9, INV-WS-02), shared-source scan, GATE-WS-03 record  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U34/N34-25.md`

- **pair** `N25b-05` / `N34-25`
  - reported by G25ab: both claim the record of GATE-WS-03: N25b-05 measures three clauses end to end, N34-25 assembles the row from four clauses
  - reported by G34: both write the record of GATE-WS-03: N34-25 assembles the row from four clauses, N25b-05 measures three of them end to end (also reported by G25ab)

## Cluster of 2 beads

- `N27b-22` (U27b, M2, task) Pilot: growth of placebo separation G(W) and single-precision admission on reference closed loops  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U27b/N27b-22.md`
- `N35-08` (U35, M3, feature) frob-opt: the pairing report: realized pair correlation and the per-task divergence rate λ̂  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U35/N35-08.md`

- **pair** `N27b-22` / `N35-08`
  - reported by G35: both measure the growth rate by twin runs; one estimator should serve both

## Cluster of 2 beads

- `N33c-08` (U33c, M0, task) workspace: the toolchain train procedure and the one-toolchain rule enforced by check-stack  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U33c/N33c-08.md`
- `NF-02` (UF, M0, task) suite: the toolchain train: moving T here and in every consumed sibling in one coordinated change  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/NF-02.md`

- **pair** `N33c-08` / `NF-02`
  - reported by G33: the toolchain-train procedure is drafted in both; N33c-08 also holds the one-toolchain conditions of check-stack and the dry-run script, which stay whichever procedure bead is kept
  - reported by GF: both specify the toolchain train (docs/runbooks/toolchain-train.md, a dry run for a proposed T, the record a train leaves); N33c-08 adds the check-stack conditions of the one-toolchain rule, NF-02 the legs in the siblings and the refusals

