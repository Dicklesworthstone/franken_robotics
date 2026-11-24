# Cross-group overlap packet dup-14

35 pairs in 21 clusters. Each cluster is a set of beads connected by reported overlaps; decide a cluster as a whole.

## Cluster of 6 beads

- `N26b-02` (U26b, M4, task) frob-nemesis: false-discard rate per stratum with the exact binomial bound 1 − δ^(1/m), and the stratum verdict  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U26b/N26b-02.md`
- `N26b-03` (U26b, M4, task) frob-nemesis: the planted winner: a known top design under a blinded identity that must survive every screen  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U26b/N26b-03.md`
- `N26b-04` (U26b, M4, task) frob-nemesis: the audit record frob.nemesis.audit/1, the label screen-biased, stage bypass and re-qualification  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U26b/N26b-04.md`
- `N26b-05` (U26b, M4, feature) verb nemesis.audit: audit one screening stage with stratified discards and a planted winner  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U26b/N26b-05.md`
- `N26b-06` (U26b, M4, task) frob-conformance: planted fixtures for the screening audit: biased and unbiased screens, 1,000 campaigns  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U26b/N26b-06.md`
- `N28b-29` (U28b, M4, feature) verb nemesis.audit: stratified audit of discards with an exact bound, and the sealed planted winner  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U28b/N28b-29.md`

- **pair** `N26b-02` / `N28b-29`
  - reported by G26: the exact false-discard bound 1 − δ^(1/m) per stratum and the stratum verdict are in both
  - reported by G28: confirmed: the false-discard bound 1 − δ^(1/m) per stratum and the stratum verdict are in both; part of the duplicate above
- **pair** `N26b-03` / `N28b-29`
  - reported by G26: the planted winner's seal, blinded entry, reveal and FROB-REF-566 are in both
  - reported by G28: confirmed: the planted winner's seal, blinded entry and reveal are in both; part of the duplicate above
- **pair** `N26b-04` / `N28b-29`
  - reported by G26: the audit record, the label screen-biased and the re-qualification rule are in both; the re-opening and trust rule are also in BD-345
  - reported by G28: confirmed: the audit record, the label screen-biased and re-qualification are in both; the re-opening and the trust rule stay with BD-345
- **pair** `N26b-05` / `N28b-29`
  - reported by G26: both specify the verb nemesis.audit with stratified discards, the exact bound, the sealed planted winner, FROB-REF-566 and the measurement of GATE-NEMESIS-07; §26.10 owns the mechanism (N40-08 says so), U28b should keep the cascade's application (BD-345)
  - reported by G28: CONFIRMED duplicate: both specify the verb nemesis.audit (stratified discards, exact bound, sealed planted winner, FROB-REF-566, GATE-NEMESIS-07). The plan gives the audit to §26.10 and says §28.12 applies it without restating it, so the Nemesis beads should be kept and N28b-29 superseded by N26b-05 after folding; BD-345 and N28b-31 then depend on N26b-05 (edge already recorded by the Nemesis integrator)
- **pair** `N26b-06` / `N28b-29`
  - reported by G26: the two synthetic batteries of GATE-NEMESIS-07 (100 biased-screen runs; 1,000 campaigns) are built in both; N28b-31 takes its population from BD-377
  - reported by G28: confirmed: the two synthetic batteries of GATE-NEMESIS-07 are built in both

## Cluster of 5 beads

- `N21a-27` (U21a, M2, task) frob-conformance: the M2 step profile (phase ledger, kernel shares, bytes touched) and the funding screen  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U21a/N21a-27.md`
- `N36-05` (U36, M0, task) frob-conformance perf: the speedup bound 1/(1 − f + f/r + h) and the ceiling-first refusal FROB-REF-866  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U36/N36-05.md`
- `N36-06` (U36, M2, task) perf harness: ratios admitted by a one-sided 95% paired lower bound; A/A calibration; FROB-REF-863  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U36/N36-06.md`
- `N36-18` (U36, M2, task) Bytes touched per world-step: per-launch figure, the bandwidth-bound test, Jacobian traffic at 18, 30, 60  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U36/N36-18.md`
- `N38a-08` (U38a, M1, task) frob-conformance: experiment statistics: ceiling from phases, paired lower bound, potency and A/A arms  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U38a/N38a-08.md`

- **pair** `N21a-27` / `N36-05`
  - reported by G36: Keep both; one part is duplicated. N36-05 (M0) is the one home of the bound 1/(1 − f + f/r + h), min_fraction_for, the family-batch bound, the three usage rules, the record frob.perf.ceiling/1 with before-experiment order checked from the ledger, and the raising function of FROB-REF-866. N21a-27 (M2) keeps profiling mode, the phase ledger, shares, derived figures, frob.stepprofile/1 and the two doctrine thresholds (50% and 15%). Step 3 of its screen recomputes the same ceiling, raises 866 itself and repeats N36-05's known answers (13.5, 6.7, 0.92, 842.8 ÷ 575.7 = 1.46) and the pruning control (20, not 50); it should call ceiling() and admit_experiment() and file a ceiling record (edge recorded). N21a-27 has what N36-05 lacks: refusal of a request with no profile or with a profile whose fingerprint is not the machine's, the candidate's transfer and synchronization cost charged before the 15% test, and KA-549 (the specialization ceiling). N36-05 has what N21a-27 lacks: rules 1 and 3 (warm strong baseline, queue time counted), the record schema with late-record rejection, four ranked repairs, the pilot-register hook
- **pair** `N21a-27` / `N36-18`
  - reported by G36: bytes touched per world-step from manifest footprints at active counts is computed in both (N36-18 in every run receipt with validation, budget and bandwidth test; N21a-27 as a derived figure of the profile); one function should serve both, and the profile should read the receipt's table
- **pair** `N36-05` / `N38a-08`
  - reported by G36: the ceiling-from-phases formula appears in both; N36-05 (M0) is the earlier and holds the record and the refusal
- **pair** `N36-06` / `N38a-08`
  - reported by G36: the one-sided paired lower bound and the A/A procedure are implemented in both (N38a-08 at M1 in ratio and difference form; N36-06 at M2 with the campaign protocol, the PerfRatio type and FROB-REF-863); N36-06 should call N38a-08's estimator and quantile table
  - reported by G38: the one-sided paired lower bound and the A/A procedure are implemented in frob-conformance by N38a-08 (strict A/A form) and again by the performance harness N36-06 (frequency form); N36-05 repeats the ceiling function. One implementation should serve both

## Cluster of 4 beads

- `BD-318` (U38b, M3, task) frob-conformance for M3: hard and covering batteries, planted-defect registries, hostile-input corpora  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U38b/BD-318.md`
- `N27a-12` (U27a, M3, task) Planted joint and alternative causes: 100 two-factor failures with four-corner truth tables, and the host tables  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U27a/N27a-12.md`
- `N27a-14` (U27a, M3, task) Planted failures that must name no factor: 100 chaotic and 100 no-cause cases, with their controls  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U27a/N27a-14.md`
- `N27b-08` (U27b, M3, task) frob-autopsy: the planted-single-cause suite: 200 failures in five classes, ten candidates each, with its harness  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U27b/N27b-08.md`

- **pair** `BD-318` / `N27a-12`
  - reported by G27: the joint and alternative pairs row of BD-318's Autopsy registry is the registry autopsy-plants-coalitions.toml of N27a-12 (100 cases and three host tables)
- **pair** `BD-318` / `N27a-14`
  - reported by G27: the chaotic-placebo row of BD-318's Autopsy registry is the registry autopsy-plants-abstention.toml of N27a-14 (100 chaotic, 100 no-cause, 30 controls)
- **pair** `BD-318` / `N27b-08`
  - reported by G27: BD-318 writes a planted-defect registry of Autopsy, registry/planted/autopsy.toml, with the 200 single causes; N27b-08 writes the manifest frob-conformance/registry/autopsy_planted.toml for the same 200 failures. One file is wanted; BD-318 depends on the Autopsy beads and should index N27b-08's manifest

## Cluster of 4 beads

- `N39a-10` (U39a, M3, task) frob-conformance tenfold: GATE-TENFOLD-05 measured, adjudicator acceptance with twenty planted answers per stratum  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U39a/N39a-10.md`
- `N39a-40` (U39a, M3, task) frob-conformance tenfold: GATE-TENFOLD-04 measured: six ordered freeze-and-seal steps, pre-analysis on record  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U39a/N39a-40.md`
- `N39b-22` (U39b, M3, task) frob-conformance tenfold: GATE-TENFOLD-03: every harness known answer exact, every negative control caught  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U39b/N39b-22.md`
- `N40-14` (U40, M3, task) program gates GATE-TENFOLD-03, -04, -05: harness known answers, freeze-and-seal order, adjudicator acceptance  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U40/N40-14.md`

- **pair** `N39a-10` / `N40-14`
  - reported by G40: both record GATE-TENFOLD-05; N40-14 adds the pre-opening conjunction and reads N39a-10's record
- **pair** `N39a-40` / `N40-14`
  - reported by G39: N40-14 states the checks of GATE-TENFOLD-03, -04 and -05 as one pre-opening verdict; the measurements are N39b-22, N39a-40 and N39a-10
  - reported by G40: both record GATE-TENFOLD-04; N40-14 adds the pre-opening conjunction and reads N39a-40's record
- **pair** `N39b-22` / `N40-14`
  - reported by G40: both record GATE-TENFOLD-03; N40-14 adds the pre-opening conjunction and reads N39b-22's record

## Cluster of 3 beads

- `BD-328` (U23b, M4, feature) verb eval.rank-agreement between rungs: the 40-design calibration per class; GATE-CORTEX-12, KILL-RANK  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U23b/BD-328.md`
- `BET-17` (U00, M4, task) Decide BET-17: rank agreement between rungs (cheap rungs order designs as the trained, promoted rung does)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U00/BET-17.md`
- `N16b-12` (U16b, M3, task) Decision KILL-RANK, tier-pair reading: may a physics tier pair order a population of designs  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U16b/N16b-12.md`

- **pair** `BD-328` / `BET-17`
  - reported by G00: overlap with distinct scope, no merge: both record the decision KILL-RANK. The kill-gate bead applies the trigger and writes the kill ADR; the bet bead reads that record, evaluates the bet's falsifier clauses and writes the bet verdict. The bet bead's acceptance line "the KILL-RANK decision recorded" means on record by the other bead. The dependency of the bet on this bead is in place.
- **pair** `BET-17` / `N16b-12`
  - reported by G00: overlap with distinct scope, no merge: both record the decision KILL-RANK. The kill-gate bead applies the trigger and writes the kill ADR; the bet bead reads that record, evaluates the bet's falsifier clauses and writes the bet verdict. The bet bead's acceptance line "the KILL-RANK decision recorded" means on record by the other bead. The dependency of the bet on this bead is in place.

## Cluster of 3 beads

- `N00-14` (U00, M0, task) frob-conformance: documented incumbent absences, the not-found list, and the M0 re-audit with execution on trj  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U00/N00-14.md`
- `N39b-08` (U39b, M0, task) frob-conformance: GATE-TENFOLD-01, the rival audit and the no-agent incumbent baseline in the first ledger  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U39b/N39b-08.md`
- `N40-12` (U40, M0, task) program gate GATE-TENFOLD-01: the parity baseline, the rival audit and no-agent incumbent phases on trj  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U40/N40-12.md`

- **pair** `N00-14` / `N39b-08`
  - reported by G00: both audit the installed incumbents at M0: N00-14 turns each documented absence and not-found item into an executed check on trj; N39b-08 writes the audit record per incumbent with its capability table and records GATE-TENFOLD-01. Distinct scope; the capability table of N39b-08 should be the register of N00-13
- **pair** `N39b-08` / `N40-12`
  - reported by G39: both write the record of GATE-TENFOLD-01: measured here, a program-gate row with its own completeness check there
  - reported by G40: both write the record of GATE-TENFOLD-01 and apply the within-20% reproduction rule to the M0 incumbent rows; N40-12 is made to depend on N39b-08

## Cluster of 3 beads

- `N18a-28` (U18a, M2, task) frob-contact: patch rules and patch quadrature: cone error against the true limit surface, load-weighted slip  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U18a/N18a-28.md`
- `N24b-11` (U24b, M2, task) frob-arena: the slip-distance metric, load-weighted over the contact quadrature and invariant to its refinement  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U24b/N24b-11.md`
- `N24b-12` (U24b, M2, task) fixture:quad-split-grip: the limiting model on the host and the plate-drag rig on the referee  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U24b/N24b-12.md`

- **pair** `N18a-28` / `N24b-11`
  - reported by G18: the load-weighted, quadrature-invariant slip metric is specified in both (and again in N24a-20)
  - reported by G24: the load-weighted slip metric and its refinement-invariance test are defined in both (N18a-28 as a weighted mean slip speed, N24b-11 as its time integral)
- **pair** `N18a-28` / `N24b-12`
  - reported by G24: the sole straddling a low-grip strip (KA-616, the same case as KA-413) is computed in both

## Cluster of 2 beads

- `BD-001` (U33c, M0, task) workspace: core skeleton, one toolchain file, build-identity setting, stack lock and first resolution receipt  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U33c/BD-001.md`
- `N41-18` (U41, M0, task) First session, step 1: cargo check on the core workspace with its three repeated settings; the first receipt  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U41/N41-18.md`

- **pair** `BD-001` / `N41-18`
  - reported by G41: BD-001 creates the workspace and writes the first resolution receipt; step 1 of the first session runs cargo check on it and records the receipt as an observation (N41-18's title no longer says it creates the workspace)

## Cluster of 2 beads

- `BD-021` (U25b, M0, task) frob-lemma v0: claims and instance certificates for T-01, T-03, T-04 on the M0 kernels, checked by FrankenSymPy  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U25b/BD-021.md`
- `CARD-T-01` (U12, M0, task) Theorem card T-01: the Cayley map of a skew-symmetric matrix is a rotation  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U12/CARD-T-01.md`

- **pair** `BD-021` / `CARD-T-01`
  - reported by G12: BD-021 issues the M0 instance certificates of T-01, T-03 and T-04; the three cards also list the instance certificate in scope. Distinct in practice: BD-021 is the machinery and first run, the cards hold statement documents, rows, specimens and mutants

## Cluster of 2 beads

- `BD-137` (U32a, M1, feature) frob-verbs: the M1 verbs registered with schemas, examples and refusal lists; GATE-TETHER-02 for the M1 set  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U32a/BD-137.md`
- `NJL-03` (UJL, M1, task) frob-verbs registry: the 80 verbs the mechanical inventory missed; the 243-row reconciliation, count checks  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UJL/NJL-03.md`

- **pair** `BD-137` / `NJL-03`
  - reported by G32: the verb rows are entered twice: by milestone in BD-137, BD-222, BD-312, BD-376 and BD-409, and by cluster in NJL-03 and the VERBS-<cluster> beads, each with parity and schema-snapshot checks

## Cluster of 2 beads

- `BD-258` (U37, M3, feature) frobd/frob-verbs: capability tokens, scopes, attenuation, namespaces, session roots; GATE-SAFE-10 full surface  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U37/BD-258.md`
- `N32b-01` (U32b, M2, task) frobd, frob-verbs: capability tokens: scope catalogue, minting, daemon-key tag, the check before every verb  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U32b/N32b-01.md`

- **pair** `BD-258` / `N32b-01`
  - reported by G32: a seed and its components: BD-258 is the capstone over the token, attenuation, namespace and session-verb beads N32b-01 to N32b-04 (dependencies recorded by the safety group); the end-to-end script N32b-05 stays related because it runs over N32a-27

## Cluster of 2 beads

- `BET-05` (U00, M3, task) Decide BET-05: static analysis prunes bad designs before simulation and does not throw away a winner  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U00/BET-05.md`
- `N28a-22` (U28a, M3, task) decision KILL-STATIC: keep design.check refusals, or downgrade them to advisory lints  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U28a/N28a-22.md`

- **pair** `BET-05` / `N28a-22`
  - reported by G00: overlap with distinct scope, no merge: both record the decision KILL-STATIC. The kill-gate bead applies the trigger and writes the kill ADR; the bet bead reads that record, evaluates the bet's falsifier clauses and writes the bet verdict. The bet bead's acceptance line "the KILL-STATIC decision recorded" means on record by the other bead. The dependency of the bet on this bead is in place.

## Cluster of 2 beads

- `BET-35` (U00, M4, task) Decide BET-35: two machines (the division of labour between the two-card Linux host and the Mac Studio pays)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U00/BET-35.md`
- `N21c-11` (U21c, M4, task) Kill-gate decision KILL-FLEET, first reading: fund the fleet scheduler or run the machines standalone  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U21c/N21c-11.md`

- **pair** `BET-35` / `N21c-11`
  - reported by G00: overlap with distinct scope, no merge: both record the decision KILL-FLEET. The kill-gate bead applies the trigger and writes the kill ADR; the bet bead reads that record, evaluates the bet's falsifier clauses and writes the bet verdict. The bet bead's acceptance line "the KILL-FLEET decision recorded" means on record by the other bead. The dependency of the bet on this bead is in place.

## Cluster of 2 beads

- `N00-28` (U00, M0, task) frob-conformance: the bet register: thirty-seven bets and the rule that turns gate records into a bet verdict  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U00/N00-28.md`
- `N40-31` (U40, M1, task) registry: the 37 bets with their deciding gates, kill gates and milestones; the check that each has a decider  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U40/N40-31.md`

- **pair** `N00-28` / `N40-31`
  - reported by G00: both register the 37 bets with deciding gates, kill gates and milestones and compute a bet status from gate records: N00-28 as registry/bets.toml with the pre-registration check and the verdict function (M0); N40-31 as rows of the gate register with the check that each bet has a decider (M1). One register should own the rows
  - reported by G40: both register the 37 bets with their deciding gates and kill gates: N00-28 as the bet register with the verdict function (M0), N40-31 as rows of the gate register with the check that each bet has a decider (M1); N40-31 is made to depend on N00-28

## Cluster of 2 beads

- `N10b-16` (U10b, M8, task) pilot PILOT-C-24: three arms at equal wall-clock: plain transformer against geometric-algebra variants  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U10b/N10b-16.md`
- `N23b-01` (U23b, M8, task) Pilot [M]: geometric-algebra tree policy, three arms at equal wall-clock on existing tools, before any funding  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U23b/N23b-01.md`

- **pair** `N10b-16` / `N23b-01`
  - reported by G10: same pilot: geometric-algebra tree policy, three arms at equal wall-clock
  - reported by G23: the same pilot: three arms (plain transformer against two geometric-algebra variants) before BD-439 is funded

## Cluster of 2 beads

- `N14b-24` (U14b, M1, task) e2e: import and export: reference robots, loss reports, kinematic parity against the oracle, round trips  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U14b/N14b-24.md`
- `NEG-16` (UEG, M1, task) e2e: `scripts/e2e/assets_licences.sh`: reference robots and scene-asset sources validated and licence-checked  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/NEG-16.md`

- **pair** `N14b-24` / `NEG-16`
  - reported by GEG: two end-to-end scripts walk every reference robot through import: other measures GATE-SKEL-01 parity (skel_import_export.sh, with N14a-31 for MJCF), mine checks licences, manifests and validation (assets_licences.sh)

## Cluster of 2 beads

- `N18a-41` (U18a, M2, task) e2e: `scripts/e2e/scene_gen_terrain.sh`: `frob scene gen` terrain families, identity across platforms  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U18a/N18a-41.md`
- `N19a-30` (U19a, M2, task) e2e scene.gen terrain: geometric known answers per family, determinism lanes, seeds not voxels  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U19a/N19a-30.md`

- **pair** `N18a-41` / `N19a-30`
  - reported by G18: same end-to-end script for frob scene gen terrain families
  - reported by G19: the same end-to-end script for `frob scene gen` on the terrain families with identity across platforms

## Cluster of 2 beads

- `N20a-19` (U20a, M6, task) frob-sense: tactile arrays [F]: taxel footprints with saturation, resultant equal to the sum over taxels  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U20a/N20a-19.md`
- `N38a-31` (U38a, M6, feature) frob-sense, frob-contact: tactile arrays with taxel footprints and saturation, fed by micro-patch pads  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U38a/N38a-31.md`

- **pair** `N20a-19` / `N38a-31`
  - reported by G20: both specify tactile arrays in frob-sense with taxel footprints and saturation; N20a-19 from the sensor side (resultant equals the sum over taxels), N38a-31 under the tactile epic fed by micro-patch pads
  - reported by G38: both specify tactile arrays in frob-sense with taxel footprints and saturation; N20a-19 from the sensor side, N38a-31 with the micro-patch pads and cards under the seed epic BD-418

## Cluster of 2 beads

- `N25b-08` (U25b, M3, task) Experiment OPEN-AUGUR-3: lowered static constraints, the constraint-to-parameter graph, measured sparsity  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U25b/N25b-08.md`
- `N25e-27` (U25e, M3, task) Experiment for OPEN-AUGUR-3: the constraint-to-parameter graph of the robot classes, with |aᵢ| per parameter  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U25e/N25e-27.md`

- **pair** `N25b-08` / `N25e-27`
  - reported by G25ab: both are the OPEN-AUGUR-3 experiment: the constraint-to-parameter graph of the robot classes and its measured sparsity
  - reported by G25cde: both are the experiment that settles OPEN-AUGUR-3 (the constraint-to-parameter graph of the robot classes); U25d's coverage asks for one bead

## Cluster of 2 beads

- `N28a-05` (U28a, M1, feature) verbs analyze.<name>: one read verb per analysis A-01 to A-26, generated from the analysis registry  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U28a/N28a-05.md`
- `VERBS-analyze` (UJL, M1, task) frob-verbs registry: cluster `analyze`, twenty-six rows, with parity, e2e and schema-snapshot checks  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UJL/VERBS-analyze.md`

- **pair** `N28a-05` / `VERBS-analyze`
  - reported by G28: both enter the twenty-six analyze.* rows: VERBS-analyze as registry data, N28a-05 as verbs generated from the analysis registry with proposed names for 25 of them

## Cluster of 2 beads

- `N33c-38` (U33c, M0, task) xtask check-deps: one serde_json version per binary, and the exact-pin conflict probe that keeps ftx-drape out  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U33c/N33c-38.md`
- `N34-04` (U34, M0, task) xtask check-deps: JSON-library guard (version and unified feature set per binary against the stack lock)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U34/N34-04.md`

- **pair** `N33c-38` / `N34-04`
  - reported by G33: N34-04 guards the serde_json version and unified feature set per binary; N33c-38 holds the one-version rule, the conflict probe and the ftx-drape plant on top of it (dependency recorded)
  - reported by G34: both state the one-version rule for serde_json per binary against the stack lock; N34-04 adds the unified feature set, N33c-38 the exact-pin conflict probe (G33 made N33c-38 depend on N34-04)

