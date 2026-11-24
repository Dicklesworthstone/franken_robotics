# Cross-group overlap packet dup-03

36 pairs in 20 clusters. Each cluster is a set of beads connected by reported overlaps; decide a cluster as a whole.

## Cluster of 10 beads

- `N13-REF` (U13, M0, task) Refusal registry for Marrow: FROB-REF-212 to -215 with triggers, payloads, ranked repairs and exit codes  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U13/N13-REF.md`
- `N14-REF` (U14b, M1, task) Refusal registry of Skeleton (§14): FROB-REF-101 to -126 with triggers, payloads, ranked repairs, exit codes  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U14b/N14-REF.md`
- `N21-REF` (U21c, M2, task) Spark refusal registry: FROB-REF-601 to -618 with triggers, ranked repairs, exit code 6 and raising sites  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U21c/N21-REF.md`
- `N23-REF` (U23b, M2, task) Cortex refusal registry: twelve codes (521 to 554, 830, 867) and three late refusals  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U23b/N23-REF.md`
- `N27-REF` (U27b, M2, task) Autopsy refusal-code registry: FROB-REF-580 to -584, -586, -587 and -880 with triggers, repairs and exit codes  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U27b/N27-REF.md`
- `N29-REF` (U29b, M1, task) registries/refusals.toml: eleven Chronicle refusal codes, the fork-graft cross-reference, one uncoded refusal  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U29b/N29-REF.md`
- `N32-REF` (U32b, M0, task) Tether refusal-code registry: FROB-REF-701 to -704, -720, -723 to -725 and -801 to -810  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U32b/N32-REF.md`
- `N35-REF` (U35, M0, task) Numerics refusal registry: FROB-REF-247 to -252, -561, -562, -761, -861 to -866 with repairs and exits  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U35/N35-REF.md`
- `N36-REF` (U36, M2, task) registry: performance refusals FROB-REF-862, -863, -866, -761, -602: triggers, repairs, exit codes, sites  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U36/N36-REF.md`
- `N37-REF` (U37, M1, task) registries/refusals.toml: safety refusal codes 731, 733, 734, 816, 819, 825, 828, 832 with triggers and repairs  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U37/N37-REF.md`

- **pair** `N13-REF` / `N14-REF`
  - reported by G13: both proposed FROB-REF-127 for different uncoded refusals (N14b-34 too); N13-REF and N13-34 now propose FROB-REF-128
- **pair** `N14-REF` / `N32-REF`
  - reported by G32: FROB-REF-804 (surface) and -112 (design layer) both name a stale parent in a patch
- **pair** `N21-REF` / `N36-REF`
  - reported by G36: the row of FROB-REF-602 is owned by N21-REF; N36-REF adds the scene and camera repair variants (never fewer worlds for scenes; never a lower resolution or cadence for frames), which the owner's row should carry
- **pair** `N23-REF` / `N35-REF`
  - reported by G35: neighbouring codes 861 and 867
- **pair** `N27-REF` / `N35-REF`
  - reported by G35: neighbouring codes 562 and 584
- **pair** `N29-REF` / `N32-REF`
  - reported by G29: one event, two codes, twice (878 and 810; 896 and 806); both registries state the same layer split; the registry owner's ruling is still owed
  - reported by G32: one event, two codes, twice: FROB-REF-810 and -878 (different bytes for a committed chunk), -806 and -896 (cursor used with another pin); both registries state the same layer split and a ruling is owed
- **pair** `N32-REF` / `N36-REF`
  - reported by G32: FROB-REF-704 (the daemon's campaign class) and -761 (the compute envelope) can both refuse the same oversized job
- **pair** `N35-REF` / `N36-REF`
  - reported by G35: rows 761, 862, 863 and 866 appear in both: rows owned here, raising sites and repairs there
  - reported by G36: N35-REF owns the registry rows of 761, 862, 863 and 866; N36-REF writes triggers, pointers and ranked repairs for the same rows, and the two repair lists differ in wording and order. One row, one list: N36-REF's fuller triggers and repairs should be entered in N35-REF's rows, and N36-REF keeps the raising-site matrix and the script
- **pair** `N35-REF` / `N37-REF`
  - reported by G35: neighbouring codes 561 and 825: one code per raising site, the two readings must agree

## Cluster of 4 beads

- `BD-002` (U25a, M0, task) First-commit documents: AGENTS.md, README honest-status table, DEPENDENCY_ADMISSION.md, registries, two schemas  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U25a/BD-002.md`
- `N33a-07` (U33a, M0, task) admission records: schema, writer, validator, `DEPENDENCY_ADMISSION.md`; an admission is never inherited  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U33a/N33a-07.md`
- `N33b-01` (U33b, M0, task) franken-stack.lock: the dependency firewall as data, with its consistency test against admissions  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U33b/N33b-01.md`
- `N40-07` (U40, M0, task) dependency firewall by milestone: what each exit may rely on, and the check that an admission is never inherited  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U40/N40-07.md`

- **pair** `BD-002` / `N33a-07`
  - reported by G25ab: both define the format of DEPENDENCY_ADMISSION.md: BD-002 as a companion document, N33a-07 as the admission-record schema, writer and validator
  - reported by G33: both define the format of DEPENDENCY_ADMISSION.md: BD-002 as a companion document, N33a-07 as the record schema, writer and validator from which the document is rendered (as G25ab recorded)
- **pair** `N33a-07` / `N40-07`
  - reported by G40: the check that an admission is never inherited is implemented in both
- **pair** `N33b-01` / `N40-07`
  - reported by G33: the dependency firewall as data is drafted twice with different homes (two fields per row of franken-stack.lock here; three fields per row of registries/milestones.toml there); N40-07 also restates the never-inherited check of N33a-07 and the M7 freeze check of N33b-03
  - reported by G40: the dependency firewall by milestone is data in both: three fields per row of registries/milestones.toml (N40-07) and two fields per foundation row of franken-stack.lock with a per-milestone report for exit reviews (N33b-01)

## Cluster of 4 beads

- `GB-30` (UEG, M1, task) card GB-30 (frob-arena): which verdicts 64 pairs can return: 521, 521, 1,103 inconclusive, and no tie  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/GB-30.md`
- `N24b-06` (U24b, M1, feature) verb eval.preanalyze: the rule `paired-clopper-pearson/1` and the exact enumerator of reachable verdicts  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U24b/N24b-06.md`
- `N24b-21` (U24b, M1, task) e2e: `scripts/e2e/arena_study_preanalysis.sh`: `eval.preanalyze` tables and the study-type refusals  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U24b/N24b-21.md`
- `N33b-19` (U33b, M1, feature) verbs study.plan, study.submit, study.status, eval.preanalyze, eval.compare on the CPU; end-to-end script  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U33b/N33b-19.md`

- **pair** `GB-30` / `N24b-06`
  - reported by GEG: same enumeration for the fixed-n rule at n = 64 (521, 521, 1,103, first tie at 86): other is the enumerator inside eval.preanalyze, mine the card; also ART-L8 (plan fixture) and N38a-11 row 13 (plan-time re-run)
- **pair** `N24b-06` / `N33b-19`
  - reported by G24: the verb eval.preanalyze with its schemas appears in both; N33b-19 scopes the rule enumerator to N24b-06; both name GATE-ARENA-05, as does BD-124
  - reported by G33: the verb eval.preanalyze appeared in both; N33b-19 now assigns it to N24b-06 and depends on it
- **pair** `N24b-21` / `N33b-19`
  - reported by G24: both own an end-to-end script for the study types, eval.preanalyze and their refusals
  - reported by G33: both own an end-to-end script for the study types and their refusals (as G24 recorded)

## Cluster of 3 beads

- `BD-140` (U25a, M1, task) GATE-AUGUR-07: fresh agent drives the first mission slice by CLI and MCP; lower-verbs arm; first tie-rule reading  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U25a/BD-140.md`
- `N25d-27` (U25d, M1, task) e2e: the first Augur slice: docking on the CPU as the M1 exit demonstration, a rival's answer checked too  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U25d/N25d-27.md`
- `NS-22` (U00, M1, feature) North-Star NS-22: the docking fixture (the first mission slice on the CPU: four certified facts)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U00/NS-22.md`

- **pair** `BD-140` / `N25d-27`
  - reported by G25ab: overlap with distinct scope: both exercise the first mission slice from the user's side at M1; BD-140 is the fresh-agent trial of GATE-AUGUR-07, N25d-27 the scripted docking demonstration
- **pair** `N25d-27` / `NS-22`
  - reported by G25cde: overlap: two demo files for the docking slice at M1 (demos/augur-docking and demos/ns-22), both replaying the worked example through the public verbs

## Cluster of 3 beads

- `BET-18` (U00, M2, task) Decide BET-18: mixed-topology batches beat per-family templates and compete with the incumbent mixed batch  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U00/BET-18.md`
- `N17b-10` (U17b, M2, task) Kill-gate decision KILL-PHYLUM: read the head-to-head, record the decision, switch the engine if it fires  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U17b/N17b-10.md`
- `N21b-19` (U21b, M2, task) Kill-gate decision KILL-BUCKET: mixed-topology buckets on devices, or at most nine padded per-family templates  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U21b/N21b-19.md`

- **pair** `BET-18` / `N17b-10`
  - reported by G00: overlap with distinct scope, no merge: both record the decision KILL-PHYLUM. The kill-gate bead applies the trigger and writes the kill ADR; the bet bead reads that record, evaluates the bet's falsifier clauses and writes the bet verdict. The bet bead's acceptance line "the KILL-PHYLUM decision recorded" means on record by the other bead. The dependency of the bet on this bead is in place.
  - reported by G17: the kill-gate record and the bet decision read the same head-to-head; one must cite the other
- **pair** `BET-18` / `N21b-19`
  - reported by G00: overlap with distinct scope, no merge: both record the decision KILL-BUCKET. The kill-gate bead applies the trigger and writes the kill ADR; the bet bead reads that record, evaluates the bet's falsifier clauses and writes the bet verdict. The bet bead's acceptance line "the KILL-BUCKET decision recorded" means on record by the other bead. The dependency of the bet on this bead is in place.

## Cluster of 3 beads

- `N10a-13` (U10a, M5, task) pilot C-03: Riemannian identification of planted inertial parameters against a projected Euclidean fit  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U10a/N10a-13.md`
- `N13-29` (U13, M1, task) Pilot: affine-invariant geodesic between consistent pseudo-inertias stays positive definite (ten points)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U13/N13-29.md`
- `N30b-15` (U30b, M5, task) Pilot for identification: one planted joint on the referee through three passes, with aliased baits  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U30b/N30b-15.md`

- **pair** `N10a-13` / `N13-29`
  - reported by G10: overlap: both pilot steps on the pseudo-inertia cone (N13-29 the ten-point geodesic check; N10a-13 the planted-parameter identification of the C-03 pilot line)
- **pair** `N10a-13` / `N30b-15`
  - reported by G10: overlap: identification of planted parameters before the calibration lane is funded
  - reported by G30: overlap, as G10 recorded: both are identification pilots on planted parameters before the lane is funded (N10a-13 for inertial parameters on the manifold, N30b-15 for one actuator through the three passes)

## Cluster of 3 beads

- `N20a-03` (U20a, M1, task) frob-sense: four namespaces with declared access, and the frozen actor schema with privileged channels absent  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U20a/N20a-03.md`
- `N22a-03` (U22a, M1, feature) frob-control: estimated and privileged state sources, namespaces, out-of-sequence packets, the paired report  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U22a/N22a-03.md`
- `N23a-03` (U23a, M2, task) frob-learn: access classes, the frozen actor schema and the causality harness (rules E-8, E-9; PO-CORTEX-04)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U23a/N23a-03.md`

- **pair** `N20a-03` / `N22a-03`
  - reported by G20: both define the four namespace types (truth, measurements, belief, policy observations): N20a-03 as channel references in frob-sense, N22a-03 as wrappers in frob-dyn
- **pair** `N20a-03` / `N23a-03`
  - reported by G20: both specify the frozen actor schema with privileged channels absent; N20b-10 holds the leak registry and runner on N20a-03's type, N23a-03 the gym's access classes and its own causality harness
  - reported by G23: both specify the frozen actor schema with privileged channels absent; N23a-03 holds the gym's access classes and harness

## Cluster of 2 beads

- `BD-040` (U36, M0, task) Parity baseline on `trj`: incumbent phase ledger (cold start, Q and H rates, time to threshold, edit latency)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U36/BD-040.md`
- `N41-23` (U41, M0, task) First session, step 6: measure one incumbent's phases on one RTX 4090: cold start, two edit latencies, time to the return threshold  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U41/N41-23.md`

- **pair** `BD-040` / `N41-23`
  - reported by G41: the incumbent's cold start, edit latencies and time to threshold on one RTX 4090 are rows of the parity baseline BD-040, on which the step depends; the step records its first row as a session observation

## Cluster of 2 beads

- `BD-144` (U14b, M1, task) Phylum pilot, no GPU code: padding overhead and bucket count of the first archetypes and 1,000 random designs  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U14b/BD-144.md`
- `N17a-17` (U17a, M1, task) pilot: morphology generator: 1,000 designs per family: rejection rate, bucket histogram, padding  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U17a/N17a-17.md`

- **pair** `BD-144` / `N17a-17`
  - reported by G14: both report padding and a bucket histogram on 1,000 generated designs at M1
  - reported by G17: both sample 1,000 generated designs at M1 and report bucket histogram and padding

## Cluster of 2 beads

- `BD-275` (U15a, M3, feature) frob-nemesis: the hard-world set (frob.hardset/1), verb nemesis.hardset, and its three refused uses  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U15a/BD-275.md`
- `N26b-23` (U26b, M4, task) OPEN-NEMESIS-5: carrying a hard-world set across a robot class or task version: decision and re-promotion path  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U26b/N26b-23.md`

- **pair** `BD-275` / `N26b-23`
  - reported by G26: N26b-23 adds the sub-command nemesis.hardset carry to the verb BD-275 owns with four sub-commands

## Cluster of 2 beads

- `BET-12` (U00, M1, task) Decide BET-12: narrow purity is affordable (the schedule gate at the end of M1)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U00/BET-12.md`
- `N40-11` (U40, M1, task) program gate: the schedule gate of BET-12, M0 plus M1 against 10.5 weeks with the cause of any slip attributed  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U40/N40-11.md`

- **pair** `BET-12` / `N40-11`
  - reported by G00: both evaluate the schedule gate at the M1 exit. N40-11: the elapsed-time computation, the cause attribution of blocked time, the gate record and the pause. BET-12: the registered falsifier, the bet verdict and the trigger of the re-plan. BET-12 depends on N40-11; N40-11 must not depend on BET-12 (its need 3 is met by the bet register N00-28)
  - reported by G40: both evaluate the schedule gate at the M1 exit: N40-11 the elapsed time, cause attribution and gate record; BET-12 the bet verdict and the re-plan (BET-12 depends on N40-11)

## Cluster of 2 beads

- `CARD-T-18` (U12, M5, task) Theorem card T-18: the symmetry–event identity ρ(h)·Ξ_e = Ξ_{h·e}·ρ(h) for each generator and each event class  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U12/CARD-T-18.md`
- `N10b-02` (U10b, M4, task) spec C-16 part (frob-gait): the symmetry–event commuting identity (T-18) and contact-stratum counts  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U10b/N10b-02.md`

- **pair** `CARD-T-18` / `N10b-02`
  - reported by G12: both state the symmetry-event identity: N10b-02 as a specification with law tests in frob-gait, the card as certificates; distinct scope

## Cluster of 2 beads

- `N10a-04` (U10a, M2, task) frob-conformance: doctrine D-8, D-15 checks — adjoint or excuse per kernel, validity label per derivative  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U10a/N10a-04.md`
- `N21a-20` (U21a, M2, task) frob-kir: derivative programs J·v and Jᵀ·w by differentiating the program, validity labels, the duality test  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U21a/N21a-20.md`

- **pair** `N10a-04` / `N21a-20`
  - reported by G10: overlap: the adjoint-duality test of derivative pairs and validity labels on kernel derivatives

## Cluster of 2 beads

- `N10c-11` (U10c, M3, task) pilot PILOT-C-39: thirty registered experiments on the mission slice; interval coverage, probe comparator  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U10c/N10c-11.md`
- `N25c-15` (U25c, M3, task) pilot: the coupled-layout probe experiment against a programmed expert and direct tests  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U25c/N25c-15.md`

- **pair** `N10c-11` / `N25c-15`
  - reported by G10: shared experiment, not a duplicate: the comparator clause of the C-39 pilot is read from the Augur probe pilot

## Cluster of 2 beads

- `N15b-17` (U15b, M6, feature) frob-actuate: the battery ageing evolution law between episodes, from card coefficients [F]  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U15b/N15b-17.md`
- `N31a-48` (U31a, M6, task) frob-lifecycle [M]: four slow-state laws (wear, fatigue, insulation, cell fade), validity domains, FROB-REF-831  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U31a/N31a-48.md`

- **pair** `N15b-17` / `N31a-48`
  - reported by G15: both implement a battery capacity-fade law (ageing between episodes against the cell-fade slow state)

## Cluster of 2 beads

- `N18b-17` (U18b, M4, feature) frob-contact: rut and footprint fields as per-world mutable state over shared static tiles; zero leak  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U18b/N18b-17.md`
- `N38a-29` (U38a, M6, feature) frob-contact: per-world rut state as copy-on-write tiles on the batch tiers, zero leak over 10³ resets  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U38a/N38a-29.md`

- **pair** `N18b-17` / `N38a-29`
  - reported by G18: per-world rut state with zero leak: mine on strict and batch-cpu at M4, the other on the batch tiers at M6; overlap, scopes to be divided
  - reported by G38: per-world rut state with zero leak: N18b-17 on strict and batch-cpu at M4, N38a-29 at batch scale on devices at M6 as copy-on-write tiles; scopes should be divided at the device boundary

## Cluster of 2 beads

- `N20b-17` (U20b, M2, task) frob-verbs: the sense cluster in the verb registry: eight verbs, schemas, refusal lists, MCP names  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U20b/N20b-17.md`
- `VERBS-sense` (UJL, M2, task) frob-verbs registry: cluster `sense`, eight rows, with parity, e2e and schema-snapshot checks  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UJL/VERBS-sense.md`

- **pair** `N20b-17` / `VERBS-sense`
  - reported by G20: both write the eight registry rows of the sense cluster with schemas, refusal lists and parity checks
  - reported by GJL: both write the eight registry rows of the sense cluster (also reported by G20)

## Cluster of 2 beads

- `N25c-18` (U25c, M4, feature) verb experiment.suggest (Augur side): proposals with a prediction per mechanism, cost and decision relevance  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U25c/N25c-18.md`
- `N28c-13` (U28c, M4, feature) frob-opt: measurement proposals for experiment.suggest: partition by decision, direct-measurement comparator  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U28c/N28c-13.md`

- **pair** `N25c-18` / `N28c-13`
  - reported by G25cde: overlap: both specify measurement proposals of experiment.suggest by decision partition with the direct measurement as comparator; BD-351 ranks them
  - reported by G28: both specify measurement proposals of experiment.suggest by decision partition with the direct measurement as comparator; BD-351 ranks them

## Cluster of 2 beads

- `N30a-02` (U30a, M1, task) frob-core, frob-ground: the six Reality statuses, their transition rules and the grammar of the reality line  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U30a/N30a-02.md`
- `N38b-01` (U38b, M1, task) frob-conformance: the Gauntlet's four duties to the Reality lane as checks with planted violations  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U38b/N38b-01.md`

- **pair** `N30a-02` / `N38b-01`
  - reported by G38: overlap with distinct scope (coverage (e) 16 of U38b): the calibration unit owns the Reality statuses, the guessed-field share (N30a-03) and the anchor comparison (BD-311, N30a-04); N38b-01 and N38b-02 keep the conformance checks and depend on N30a-02

## Cluster of 2 beads

- `N38b-20` (U38b, M0, task) frob-conformance: status views generated from gates.toml and nothing else: README table, rubric, dashboard feed  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U38b/N38b-20.md`
- `N41-28` (U41, M1, task) registries/rubric.toml: the release rubric by milestone (what a user can do, what must be true, what must not be said) and its checks  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U41/N41-28.md`

- **pair** `N38b-20` / `N41-28`
  - reported by G41: the three sentences of a rubric row (what a user can do, what must be true, what must not be said): N41-28 keeps them in registries/rubric.toml, N38b-20 prints them into docs/release/STATUS.md from registries/milestones.toml; one source file must remain

