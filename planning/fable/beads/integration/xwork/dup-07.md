# Cross-group overlap packet dup-07

36 pairs in 21 clusters. Each cluster is a set of beads connected by reported overlaps; decide a cluster as a whole.

## Cluster of 6 beads

- `N38b-50` (U38b, M0, task) frob-conformance: the reconciliation register, and value 1: the solver-comparison ceiling 1.46 against 1.43  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U38b/N38b-50.md`
- `N38b-51` (U38b, M0, task) frob-conformance: reconciliation value 2: the engine-speed ceiling at an 87% step share, 7.7 against 7.8  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U38b/N38b-51.md`
- `NAB-41` (UAB, M0, task) frob-conformance: discrepancy pins: eight figures that did not follow from their formulas, nine more found  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UAB/NAB-41.md`
- `NEG-03` (UEG, M0, task) frob-conformance: the discrepancy register `registries/cards/discrepancies.toml` and the stale-value scan  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/NEG-03.md`
- `NEG-05` (UEG, M0, task) discrepancy E7-2 resolved: the engine-speed ceiling is 7.7 from the share 0.87, and 7.8 only from 82/94  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/NEG-05.md`
- `NEG-06` (UEG, M0, task) discrepancy E7-3 resolved: the zero-cost-solver ceiling is 842.8 ÷ 575.7 = 1.46; the 94% share is withdrawn  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/NEG-06.md`

- **pair** `N38b-50` / `NEG-03`
  - reported by G38: two registers of the same disagreements: the reconciliation register of N38b-50 (Appendix M.38, eleven values) and registries/cards/discrepancies.toml of NEG-03 (Appendix E.7). NEG-06 settles value 1 (842.8 ÷ 575.7 = 1.46) as N38b-50 does
  - reported by GEG: two registers of corrected values with a withdrawn-value scan. Mine: registries/cards/discrepancies.toml, twelve E.7 items, scan with scope filter and labelled-control allow-list. Other: crates/frob-conformance/registry/reconciliations.toml, eleven M.38 values REC-01…, document scan. Four items are in both (E7-1, E7-2, E7-3, E7-6)
- **pair** `N38b-50` / `NEG-06`
  - reported by GEG: both pin 842.8 ÷ 575.7 = 1.46 and withdraw the 94% share
- **pair** `N38b-51` / `NEG-05`
  - reported by G38: both settle the engine-speed ceiling: 7.7 from the share 0.87, 7.8 only from 82/94
  - reported by GEG: both pin the engine-speed ceiling 7.7 from 0.87 (7.8 only from 82/94)
- **pair** `NAB-41` / `NEG-03`
  - reported by GAB: two discrepancy registers with two stale-value scans (crates/frob-conformance/registry/discrepancies.toml and dx.withdrawn-figure here; registries/cards/discrepancies.toml there)
  - reported by GEG: a third register of the same kind: crates/frob-conformance/registry/discrepancies.toml (frob.discrepancy/1), seventeen records DX-01…; DX-02 is E7-2 and DX-03 is E7-3; the file name collides with mine
- **pair** `NAB-41` / `NEG-05`
  - reported by GAB: pin DX-02 and discrepancy E7-2 are the same correction: ceiling 7.7 from the share 0.87
  - reported by GEG: pin DX-02 is the same correction as E7-2
- **pair** `NAB-41` / `NEG-06`
  - reported by GAB: pin DX-03 and discrepancy E7-3 are the same correction: 842.8 ÷ 575.7 = 1.46, the 94% share withdrawn
  - reported by GEG: pin DX-03 is the same correction as E7-3

## Cluster of 4 beads

- `N30b-25` (U30b, M5, task) Decision OPEN-GROUND-1: which paired simulation-and-reality dataset, if any, is admitted first  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U30b/N30b-25.md`
- `N30b-26` (U30b, M5, task) Decision OPEN-GROUND-2: whether any bench hardware for the optional packages A to F is acquired  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U30b/N30b-26.md`
- `N38b-26` (U38b, M5, task) decision OPEN-GAUNT-1: which real-robot dataset, if any, is admitted to the Reality lane first  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U38b/N38b-26.md`
- `N41-10` (U41, M5, task) Owner decision OPEN-GAUNT-1, OPEN-GROUND-2: the first real dataset of the Reality lane; bench hardware or none  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U41/N41-10.md`

- **pair** `N30b-25` / `N38b-26`
  - reported by G30: OPEN-GROUND-1 and OPEN-GAUNT-1 are two halves of one question (which real dataset is admitted first); two decision records for one decision
- **pair** `N30b-25` / `N41-10`
  - reported by G30: N41-10 also decides OPEN-GAUNT-1; same decision
- **pair** `N30b-26` / `N41-10`
  - reported by G30: both decide OPEN-GROUND-2 (whether bench hardware for packages A to F is acquired)
  - reported by G41: both decide OPEN-GROUND-2 (whether bench hardware is acquired); the bench packages N30b-38 to N30b-40 depend on N30b-26
- **pair** `N38b-26` / `N41-10`
  - reported by G38: both decide OPEN-GAUNT-1 (which real dataset enters the Reality lane first); N41-10 pairs it with OPEN-GROUND-2 as an owner decision, N30b-25 has the engineering half
  - reported by G41: both decide OPEN-GAUNT-1 (which real dataset enters the Reality lane first)

## Cluster of 4 beads

- `BET-07` (U00, M4, task) Decide BET-07: symmetry buys sample efficiency on symmetric robots and symmetry-preserving tasks  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U00/BET-07.md`
- `BET-30` (U00, M4, task) Decide BET-30: equivariance under the observation-preserving subgroup (Γ_obs ≤ Γ_mech is the right group)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U00/BET-30.md`
- `N13-24` (U13, M4, task) Kill-gate decision KILL-MARROW-BLOCKS: irreducible blocks, or Reynolds projection in f64 classed Tested  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U13/N13-24.md`
- `N23a-33` (U23a, M4, task) Kill-gate decision KILL-EQUIV: keep equivariant networks or use symmetry for augmentation and testing only  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U23a/N23a-33.md`

- **pair** `BET-07` / `N23a-33`
  - reported by G00: overlap with distinct scope, no merge: both record the decision KILL-EQUIV. The kill-gate bead applies the trigger and writes the kill ADR; the bet bead reads that record, evaluates the bet's falsifier clauses and writes the bet verdict. The bet bead's acceptance line "the KILL-EQUIV decision recorded" means on record by the other bead. The dependency of the bet on this bead is in place.
- **pair** `BET-30` / `N13-24`
  - reported by G00: overlap with distinct scope, no merge: both record the decision KILL-MARROW-BLOCKS. The kill-gate bead applies the trigger and writes the kill ADR; the bet bead reads that record, evaluates the bet's falsifier clauses and writes the bet verdict. The bet bead's acceptance line "the KILL-MARROW-BLOCKS decision recorded" means on record by the other bead. The dependency of the bet on this bead is in place.
- **pair** `BET-30` / `N23a-33`
  - reported by G00: overlap with distinct scope, no merge: both record the decision KILL-EQUIV. The kill-gate bead applies the trigger and writes the kill ADR; the bet bead reads that record, evaluates the bet's falsifier clauses and writes the bet verdict. The bet bead's acceptance line "the KILL-EQUIV decision recorded" means on record by the other bead. The dependency of the bet on this bead is in place.

## Cluster of 3 beads

- `BD-102` (U20b, M1, feature) verb analyze.symmetry (A-03): Γ_mech and Γ_obs with breakers and near-symmetry repairs; stage-5 assertion  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U20b/BD-102.md`
- `N13-15` (U13, M1, task) frob-algebra::symmetry I: the mechanical group Γ_mech by exact colouring, candidate isometries and descent  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U13/N13-15.md`
- `N14a-11` (U14a, M1, task) frob-model: the symmetry functor and the assertion check: computed Γ_mech and Γ_obs, FROB-REF-109, INV-SKEL-07  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U14a/N14a-11.md`

- **pair** `BD-102` / `N13-15`
  - reported by G13: distinct scope, same section: N13-15 and N13-16 compute Γ_mech and Γ_obs in frob-algebra, BD-102 is the verb; BD-102 should depend on both
- **pair** `BD-102` / `N14a-11`
  - reported by G14: both name the stage-5 assertion check behind FROB-REF-109; N14a-11 depends on BD-102

## Cluster of 3 beads

- `BD-210` (U29a, M2, feature) Crash-consistent job protocol on devices: states, worker chunk publishing, leases, cancel, restart  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U29a/BD-210.md`
- `N29a-05` (U29a, M1, task) frob-chronicle: stopping at safe boundaries and the terminal receipt: four counts, three outcomes never merged  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U29a/N29a-05.md`
- `N32a-08` (U32a, M1, task) frob-verbs: the job ledger and state machine: transitions, leases, terminal receipts, restart recovery  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U32a/N32a-08.md`

- **pair** `BD-210` / `N32a-08`
  - reported by G29: job row, state machine, lease generations and restart recovery are in both (N32a-08 at M1 in frob-verbs, BD-210 on devices at M2); dependency BD-210 -> N32a-08 recorded
  - reported by G32: job row, state machine, lease generations and restart recovery are in both: N32a-08 at M1 in frob-verbs, BD-210 on devices at M2; BD-210 depends on N32a-08 (recorded by the Chronicle group) and its Scope should say that it extends that ledger
- **pair** `N29a-05` / `N32a-08`
  - reported by G29: terminal receipts and trial accounting are in both (store-level types here, the job ledger there); one must state that it uses the other's types
  - reported by G32: terminal receipts and trial accounting are in both (the job ledger here, store-level stop and receipt types there); one must state that it uses the other's types

## Cluster of 3 beads

- `BET-27` (U00, M4, task) Decide BET-27: a sampling controller as fallback and demonstration source, never as a stand-in evaluator  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U00/BET-27.md`
- `N22c-16` (U22c, M4, task) decision KILL-MPC: keep the sampling controller as screen and fallback, or as a single-world CPU debugging aid  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U22c/N22c-16.md`
- `N23a-21` (U23a, M4, task) Kill-gate decision KILL-DISTILL: keep general distillation or restrict it to privileged teachers  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U23a/N23a-21.md`

- **pair** `BET-27` / `N22c-16`
  - reported by G00: overlap with distinct scope, no merge: both record the decision KILL-MPC. The kill-gate bead applies the trigger and writes the kill ADR; the bet bead reads that record, evaluates the bet's falsifier clauses and writes the bet verdict. The bet bead's acceptance line "the KILL-MPC decision recorded" means on record by the other bead. The dependency of the bet on this bead is in place.
- **pair** `BET-27` / `N23a-21`
  - reported by G00: overlap with distinct scope, no merge: both record the decision KILL-DISTILL. The kill-gate bead applies the trigger and writes the kill ADR; the bet bead reads that record, evaluates the bet's falsifier clauses and writes the bet verdict. The bet bead's acceptance line "the KILL-DISTILL decision recorded" means on record by the other bead. The dependency of the bet on this bead is in place.

## Cluster of 3 beads

- `N12-KA` (U12, M1, task) Lemma known-answer registry: KA-207 to KA-227 (proofs, certificates, the offline auditor)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U12/N12-KA.md`
- `N13-KA` (U13, M1, task) Known-answer registry for Marrow: KA-228 to KA-260 and the turn-count answers, with tolerance, tier, control  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U13/N13-KA.md`
- `N38b-30` (U38b, M1, task) frob-conformance: negative controls owed by 25 known-answer rows of conventions, Lemma, Marrow and Skeleton  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U38b/N38b-30.md`

- **pair** `N12-KA` / `N38b-30`
  - reported by G12: N38b-30 supplies negative controls for known-answer rows of Lemma that state none (KA-210, KA-222 to KA-226); N12-KA lists the same rows and names its own
  - reported by G38: N38b-30 proposes controls for Lemma rows KA-210 and KA-222 to KA-226 that N12-KA names itself; likewise N13-KA and N11-KA for Marrow and the conventions
- **pair** `N13-KA` / `N38b-30`
  - reported by G13: N38b-30 supplies negative controls owed by known-answer rows of the conventions and Marrow that state none; N13-KA and N11-KA track the same rows

## Cluster of 3 beads

- `N21b-39` (U21b, M2, task) e2e: `spark_determinism_lanes.sh`: run-to-run digest equality per device family (D2; KILL-METAL's clause)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U21b/N21b-39.md`
- `N35-03` (U35, M2, task) frob-conformance: G5 determinism lanes on `batch-cpu` and the referee, including the two CPU architectures  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U35/N35-03.md`
- `N38a-35` (U38a, M2, feature) frob-conformance: determinism tier G5: five lanes over every kernel and verb against its declared class  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U38a/N38a-35.md`

- **pair** `N21b-39` / `N38a-35`
  - reported by G21: the device rows of the determinism lanes of GATE-GAUNT-07 (twice on one machine, worker counts, debug and release)
  - reported by G38: the device rows of the determinism lanes of GATE-GAUNT-07 are scripted in both
- **pair** `N35-03` / `N38a-35`
  - reported by G35: overlap: G5 lanes on batch-cpu and the referee here, the five lanes over every kernel and verb there

## Cluster of 2 beads

- `BD-062` (U14a, M1, task) frob-model: geometry and inertia functor: Region moments by fill, exact shell moments, collision primitives  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U14a/BD-062.md`
- `N13-04` (U13, M1, task) frob-model: solid-fill plausibility refusal FROB-REF-168 and the inertia-convention fixtures  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U13/N13-04.md`

- **pair** `BD-062` / `N13-04`
  - reported by G13: opposite settlements of §11.5 against §14.5: BD-062 gives an advisory for a solid above a quarter of mass_max and says FROB-REF-168 cannot arise from a .fbot; N13-04 refuses it under the default of decision N13-03

## Cluster of 2 beads

- `BD-203` (U29a, M2, feature) In-kernel digests and the digest chain, per-stage digests, fence snapshots (`K-commit`, `K-snapshot`), verb sim.replay  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U29a/BD-203.md`
- `N21a-23` (U21a, M2, task) frob-kernels: device digest contract: `K-x-digest`, an integer mix into four `u32` words; slot-free run digest  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U21a/N21a-23.md`

- **pair** `BD-203` / `N21a-23`
  - reported by G21: K-x-digest: the mixing contract here is a component of the digest-chain seed
  - reported by G29: K-x-digest: the mixing contract is N21a-23's, a component of this seed; dependency recorded

## Cluster of 2 beads

- `BD-309` (U28c, M3, task) registries/cores: records DC-01, DC-09, DC-10 with regenerating tables and forced fallbacks (cores.show)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U28c/BD-309.md`
- `N26b-11` (U26b, M3, task) frob-nemesis: covering battery assembly: one held-out slot in five, items, fallback to standard, DC-10 record  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U26b/N26b-11.md`

- **pair** `BD-309` / `N26b-11`
  - reported by G26: the DC-10 record: N26b-11 writes the decision record in frob-nemesis, BD-309 writes registries/cores/dc-10.toml with owner frob-arena; one should cite the other
  - reported by G28: the DC-10 record: N26b-11 writes the decision record in frob-nemesis, BD-309 the registry record; the Nemesis integrator recorded BD-309 -> N26b-11

## Cluster of 2 beads

- `BET-23` (U00, M4, task) Decide BET-23: interventional autopsy is stable enough to act on (planted causes, flip rates, repair turns)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U00/BET-23.md`
- `N27b-11` (U27b, M3, task) Decision KILL-AUTOPSY: keep contrasts with coalitions, or fall back to single-factor re-runs from the start  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U27b/N27b-11.md`

- **pair** `BET-23` / `N27b-11`
  - reported by G00: overlap with distinct scope, no merge: both record the decision KILL-AUTOPSY. The kill-gate bead applies the trigger and writes the kill ADR; the bet bead reads that record, evaluates the bet's falsifier clauses and writes the bet verdict. The bet bead's acceptance line "the KILL-AUTOPSY decision recorded" means on record by the other bead. The dependency of the bet on this bead is in place.

## Cluster of 2 beads

- `N00-02` (U00, M0, feature) The per-capability checklist: what "implemented" means (eight conditions, two riders) behind `frob capabilities`  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U00/N00-02.md`
- `N32b-19` (U32b, M1, feature) verb capabilities: what is implemented, from the gate register, with modes and ceilings per device  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U32b/N32b-19.md`

- **pair** `N00-02` / `N32b-19`
  - reported by G00: both define registries/capabilities.toml, the status computation from the gate register and the dispatch that raises FROB-REF-724. N00-02 (M0): the registry schema, the eight conditions and two riders, the status function and the per-bead checklist text. N32b-19 (M1): the verb's output schema, the per-device section and the MCP resource. N32b-19 should depend on N00-02 and drop the registry and status function
  - reported by G32: both define registries/capabilities.toml, the status computed from the gate register and the dispatch that raises FROB-REF-724; N00-02 (M0) has the registry schema and the eight conditions, N32b-19 (M1) the verb's output, the per-device section and the MCP resource; N32b-19 depends on N00-02 (recorded by the charter group)

## Cluster of 2 beads

- `N10a-17` (U10a, M4, task) pilot C-10: trot and bound orbits of one quadruped continued in a design parameter, ranked against rollouts  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U10a/N10a-17.md`
- `N22a-28` (U22a, M3, task) pilot: orbit continuation of a 12-joint quadruped trot over twenty 10% design perturbations (BET-26)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U22a/N22a-28.md`

- **pair** `N10a-17` / `N22a-28`
  - reported by G10: same pilot subject: continuation of a quadruped trot orbit in a design parameter before funding (N22c-09 is a third copy)

## Cluster of 2 beads

- `N12-36` (U12, M5, task) e2e lemma_coverage.sh: kernel certificate coverage, mutated-kernel control and NS-11 (GATE-LEMMA-01)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U12/N12-36.md`
- `NS-11` (U00, M5, feature) North-Star NS-11: a kernel with a certificate (`frob prove kernel K-cay-step`, re-checked offline)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U00/NS-11.md`

- **pair** `N12-36` / `NS-11`
  - reported by G00: scripts/e2e/lemma_coverage.sh of N12-36 also runs NS-11, and the seed BD-379 carries the NS-11 demonstration on K-cay-step; NS-11 holds the demo.yaml and the offline re-check

## Cluster of 2 beads

- `N17-KA` (U17a, M1, task) frob-conformance: known-answer registry for Phylum (KA-365 to KA-386) with derivations and controls  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U17a/N17-KA.md`
- `N38b-31` (U38b, M1, task) frob-conformance: negative controls owed by 69 known-answer rows of Sinew, Kinesis, Phylum and Traction  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U38b/N38b-31.md`

- **pair** `N17-KA` / `N38b-31`
  - reported by G17: N17-KA registers DESIGN negative controls for nine Phylum rows that N38b-31 also assigns
  - reported by G38: nine Phylum rows receive a DESIGN control in both

## Cluster of 2 beads

- `N18b-45` (U18b, M1, task) frob-conformance: GATE-TRAC-12 on the referee: the arc fixture under the transactional step, chord mutant caught  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U18b/N18b-45.md`
- `N33c-44` (U33c, M2, task) admission probe (row 12): fs-contact's swept_sphere_aabb misses an arc; fixture against the arc-aware bound  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U33c/N33c-44.md`

- **pair** `N18b-45` / `N33c-44`
  - reported by G18: same arc fixture (the 1 m arm, chord against arc), mine under the referee step, the other as the admission probe of the sibling routine; NF-10 is the upstream item for the bound of N18b-43
  - reported by G33: the same arc fixture: N33c-44 as the admission probe of the sibling helper, N18b-45 under the referee step (as G18 recorded)

## Cluster of 2 beads

- `N24a-34` (U24a, M3, task) frob-conformance: the full-scale qualification adversary: protocol attacks and the daemon campaign driver  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U24a/N24a-34.md`
- `N38b-12` (U38b, M3, task) frob-conformance: self-tests of the qualification lane under adaptive retries and of the solvability oracle  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U38b/N38b-12.md`

- **pair** `N24a-34` / `N38b-12`
  - reported by G24: a scripted adversary and a self-test of the qualification lane under adaptive retries with a broken-certifier control are specified in both (and in N24b-18)
  - reported by G38: the scripted adversary and the adaptive-retry self-test of the qualification lane are specified in N24a-34 and N24b-18 and registered again in N38b-12, which now depends on both and should only register the certifier row and its broken-certifier control

## Cluster of 2 beads

- `N26b-01` (U26b, M4, task) frob-nemesis: screening-audit lane: strata of discards, a keyed draw before any score, fixed-n uncancellable runs  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U26b/N26b-01.md`
- `N28b-13` (U28b, M3, task) frob-opt: the audit lane: a keyed draw before any score exists, fixed n, never cancelled (INV-ATEL-03)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U28b/N28b-13.md`

- **pair** `N26b-01` / `N28b-13`
  - reported by G26: same audit lane: keyed draw before any score, merged starved strata, pre-score commitment, fixed n, never cancelled; N26b-01 puts it in frob-nemesis at M4, N28b-13 in frob-opt at M3
  - reported by G28: PARTLY confirmed, not a clean duplicate: the plan has one audit lane, defined in §28.9 and used by §26.10. N28b-13 (frob-opt, M3) is that lane and is needed at M3 by the pilots, the calibration and N28c-36; N26b-01 (frob-nemesis, M4) restates the keyed pre-score draw and adds strata of discards, quotas and spend accounting. Keep both with distinct scope: N26b-01 should take its draw from N28b-13 (edge recorded) and drop its own draw and commitment

## Cluster of 2 beads

- `N31b-14` (U31b, M5, task) e2e: North-Star NS-20: qualified design to a verified `.fbuild` through the six Foundry verbs  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U31b/N31b-14.md`
- `NS-20` (U00, M5, feature) North-Star NS-20: a build packet (`.fbuild`: parts, geometry, tolerance stack, fatigue margins; flags false)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U00/NS-20.md`

- **pair** `N31b-14` / `NS-20`
  - reported by G00: true duplicate of the scenario script: N31b-14 is titled as the end-to-end script of North-Star NS-20 (qualified design to a verified .fbuild through the six Foundry verbs, with the demonstration manifest); NS-20 specifies the same script and demos/ns-20/demo.yaml
  - reported by G31: both script the North-Star NS-20 chain from qualified design to verified packet with a demonstration file (scripts/e2e/foundry_pipeline_ns20.sh and docs/demos/ns-20.toml against scripts/e2e/ns_20.sh and demos/ns-20/demo.yaml)

## Cluster of 2 beads

- `N39b-17` (U39b, M7, task) e2e tenfold_tournament_rehearsal.sh: the freeze and dress rehearsal of the tournament on the development pool  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U39b/N39b-17.md`
- `NS-24` (U00, M7, feature) North-Star NS-24: the sealed ten-fold tournament (sealed strata, frozen rival, adjudication, published ratio)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U00/NS-24.md`

- **pair** `N39b-17` / `NS-24`
  - reported by G00: overlap with distinct scope: N39b-17 is the tournament's freeze and dress rehearsal on the development pool; the rehearsal lane of NS-24 is a two-mission demonstration of clock, receipts and report with toy arms, labelled `rehearsal: unsealed`

