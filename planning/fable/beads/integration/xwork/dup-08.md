# Cross-group overlap packet dup-08

36 pairs in 21 clusters. Each cluster is a set of beads connected by reported overlaps; decide a cluster as a whole.

## Cluster of 5 beads

- `BD-039` (U39a, M0, task) frob-conformance: incumbent harness as an out-of-process oracle: pins, warm configurations, the disclosure block  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U39a/BD-039.md`
- `N36-13` (U36, M0, task) frob-core perf: the disclosure block `frob.perf.disclosure/1`, its completeness check and refusal FROB-REF-862  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U36/N36-13.md`
- `N36-25` (U36, M0, task) Decision OPEN-PERF-2: the incumbent revisions and configurations used as comparators, frozen before pairing  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U36/N36-25.md`
- `N38a-04` (U38a, M0, task) frob-conformance: the oracle register, pinned fixtures and capture scripts under tools/oracle  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U38a/N38a-04.md`
- `N38b-35` (U38b, M0, task) decision OPEN-GAUNT-4: the development-oracle release and the comparator set of releases and container digests  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U38b/N38b-35.md`

- **pair** `BD-039` / `N36-13`
  - reported by G36: BD-039 carries its own field list, minimal writer and FROB-REF-862 check for the disclosure block because it expected the block only at M2; N36-13 delivers the schema, completeness check and refusal at M0 and BD-039 should use them (edge recorded)
  - reported by G39: BD-039 carries a field list and a minimal writer of the disclosure block that N36-13 defines at M0
- **pair** `BD-039` / `N36-25`
  - reported by G36: the comparator registry is defined twice with different fields and paths (registries/comparators.toml here; crates/frob-conformance/registry/comparators.toml with commit, container digest, licence, recipe and warm configuration in the seed). Keep the seed's file and schema; N36-25 keeps the decision (row-to-comparator table, freeze and change rules, ADR) and adds rows[], matched and native configurations, frozen_at and state to it. N36-25 depends on BD-039 (recorded)
- **pair** `BD-039` / `N38a-04`
  - reported by G39: both claim the comparator registry frob-conformance/registry/comparators.toml
- **pair** `N36-25` / `N38a-04`
  - reported by G36: a third definition of registry/comparators.toml, in the oracle register bead
- **pair** `N36-25` / `N38b-35`
  - reported by G36: OPEN-PERF-2 and OPEN-GAUNT-4 decide the same comparator set of releases and container digests; one decision record is needed
  - reported by G38: OPEN-GAUNT-4 and OPEN-PERF-2 are one decision (the frozen incumbent releases and container digests) written as two decision beads with two comparator registries; keep one record, in comparators.toml of N38a-04

## Cluster of 5 beads

- `BD-008` (U33b, M0, task) INT-ASUP-01: one asupersync package per workspace and binary, in check-stack and check-deps; proves FD-ASUP-01  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U33b/BD-008.md`
- `FD-ASUP-01` (UF, M0, task) upstream asupersync, frankensim, frankentorch, fastmcp_rust: one published asupersync version for all  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-ASUP-01.md`
- `N33c-09` (U33c, M0, task) xtask check-deps: suite deny rules (asupersync 0.5.0, frankentui executor, never-dependency siblings), posture register  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U33c/N33c-09.md`
- `N33c-29` (U33c, M0, task) xtask check-deps: tests of INV-SUITE-01 (airlock confinement) and INV-SUITE-02 (one asupersync per binary)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U33c/N33c-29.md`
- `N34-03` (U34, M0, task) xtask check-deps: one asupersync per binary, forbidden sibling forms, airlock groups, tiers off by default  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U34/N34-03.md`

- **pair** `BD-008` / `FD-ASUP-01`
  - reported by G33: overlap resolved: the seed had drafted the execution of the sibling changes; it is re-scoped to the gate INT-ASUP-01, its wiring and the interim posture, and depends on FD-ASUP-01
  - reported by GF: BD-008's Scope lists the execution of FD-ASUP-01 in frankensim, frankentorch and fastmcp_rust; FD-ASUP-01 owns those changes and their mirrored beads and leaves BD-008 the INT-ASUP-01 measurement and its wiring into check-stack and check-deps; BD-008 should depend on it (edge recorded) and drop the execution from its Scope
- **pair** `BD-008` / `N34-03`
  - reported by G34: BD-008 also wires per-binary asupersync uniqueness, the 0.5.0 ban and the frankentui feature into check-deps; N34-03 names BD-008 only for the measured admission record
- **pair** `N33c-09` / `N34-03`
  - reported by G34: both enter the suite deny rules in check-deps: asupersync 0.5.0, the frankentui executor feature, never-dependency siblings
- **pair** `N33c-29` / `N34-03`
  - reported by G33: check-deps rule groups for one asupersync per binary, forbidden sibling forms and airlock groups are in N34-03 and also in N33c-29 (INV-SUITE-01, -02), N33c-09 (deny rows) and N33b-09 (group allow-lists, tier features off by default)
  - reported by G34: both define and test one asupersync package per binary (INV-SUITE-02, INV-WS-04) and airlock confinement (INV-SUITE-01) as check-deps rules with plants

## Cluster of 4 beads

- `BD-017` (U21b, M0, feature) verb spark.probe: the IEEE probe kernels and report that set each device family's attainable determinism class  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U21b/BD-017.md`
- `BD-020` (U21b, M0, task) GATE-SPARK-00, INT-FSIM-11: one `K-cay-step` source on `batch-cpu`, CUDA and Metal, in the divergence ledger  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U21b/BD-020.md`
- `N41-21` (U41, M0, task) First session, step 4: one kernel on Metal: a Cayley step on 10⁶ quaternions bit for bit against a Rust loop, and the exhaustive square-root probe  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U41/N41-21.md`
- `N41-22` (U41, M0, task) First session, step 5: the same kernel on CUDA: compiled with NVRTC on both RTX 4090 of trj, bit for bit against the same Rust loop  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U41/N41-22.md`

- **pair** `BD-017` / `N41-21`
  - reported by G41: the exhaustive square-root probe run in steps 4 and 5 of the first session (N41-21, N41-22) is half of the IEEE probe report of BD-017
- **pair** `BD-020` / `N41-21`
  - reported by G21: one kernel (a Cayley step) on Metal bit for bit against the CPU: the first-session step repeats GATE-SPARK-00's Metal half
  - reported by G41: one Cayley-step kernel on Metal bit for bit against a Rust loop is the Metal half of GATE-SPARK-00 (also reported by the Spark group)
- **pair** `BD-020` / `N41-22`
  - reported by G21: the same kernel on CUDA: the first-session step repeats GATE-SPARK-00's CUDA half
  - reported by G41: the same kernel on CUDA is the CUDA half of GATE-SPARK-00 (also reported by the Spark group)

## Cluster of 4 beads

- `N00-35` (U00, M0, task) tools/oracle: re-run the three plan-time checks whose scripts were lost (CPU rates, conditioning, Metal limits)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U00/N00-35.md`
- `N38a-11` (U38a, M0, task) plan-time pilots re-run: fixed-n enumeration, draft rule power, wheel loop, rotor and cadence, conditioning  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U38a/N38a-11.md`
- `N39b-16` (U39b, M7, task) pilot: the tournament decision rule simulated before sampling: draft rule, power table, binding pre-analysis  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U39b/N39b-16.md`
- `NEG-10` (UEG, M1, task) discrepancy E7-7 resolved: the wheel-loop row uses one plant: 2.6917 with shaft damping, 2.7075 without  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/NEG-10.md`

- **pair** `N00-35` / `N38a-11`
  - reported by G00: both re-run the contact-Hessian conditioning check (row 20 of the plan-time ledger; one of the three checks of N00-35 whose scripts were not retained). N00-35 alone covers the CPU reference rates and the Metal limits
  - reported by G38: both re-run the humanoid contact-Hessian conditioning survey (row 20 of the plan-time ledger); N38a-11 owns rows 13, 14, 17 to 20; N00-35 alone has the CPU reference rates and Metal limits. Row 20 should be run once, by N38a-11
- **pair** `N38a-11` / `N39b-16`
  - reported by G39: both re-run the planning simulation of the draft decision rule's power
- **pair** `N38a-11` / `NEG-10`
  - reported by GEG: same wheel-loop row. Mine requires one plant: 2.6917 with the fixture's shaft damping (as BD-108 and N16c-42 state), 2.7075 only without. Other reproduces row 17 as 2.7075, 1.4820, 0.9207 (two plant models): its 2.7075 would trip the stale-value scan unless labelled

## Cluster of 3 beads

- `BD-217` (U21a, M2, task) Throughput parity rows: profile, one lever at a time, measure, disclose; GATE-SPARK-03, -04, the kill inputs  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U21a/BD-217.md`
- `N36-07` (U36, M2, task) perf harness: the optimization ledger: ceiling beforehand, a behaviour proof, paired bound at least 1.03  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U36/N36-07.md`
- `N36-27` (U36, M2, task) Kill-gate decision KILL-PARITY: a physics row below 0.33 of the incumbent pauses device-tier feature work  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U36/N36-27.md`

- **pair** `BD-217` / `N36-07`
  - reported by G36: the landing rule of an optimization (ceiling beforehand, screen verdict, behaviour proof, paired bound at least 1.03, one lever) is specified in the ledger bead and restated as acceptance of the optimization pass; BD-217 should land levers through the ledger (edge recorded)
- **pair** `BD-217` / `N36-27`
  - reported by G21: the seed's done-condition says KILL-PARITY decided; N36-27 is the decision bead; BD-217 now only supplies the measurement
  - reported by G36: KILL-PARITY: the seed's done-condition says decided; N36-27 is the decision bead and BD-217 supplies the measurement (already re-scoped by its group)

## Cluster of 3 beads

- `BET-33` (U00, M4, task) Decide BET-33: revalidation ("which verdicts survive this edit" is answered, never with a stale verdict)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U00/BET-33.md`
- `N25a-19` (U25a, M3, task) Decision KILL-AUGUR-REUSE (standing from M3): one wrong inherited verdict removes its rule; fall back to keyed reuse  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U25a/N25a-19.md`
- `N28b-36` (U28b, M4, task) Kill-gate decision KILL-REUSE (M4): enable implication-licensed reuse (grade 3), or keep grades 1 and 2 only  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U28b/N28b-36.md`

- **pair** `BET-33` / `N25a-19`
  - reported by G00: overlap with distinct scope, no merge: both record the decision KILL-AUGUR-REUSE. The kill-gate bead applies the trigger and writes the kill ADR; the bet bead reads that record, evaluates the bet's falsifier clauses and writes the bet verdict. The bet bead's acceptance line "the KILL-AUGUR-REUSE decision recorded" means on record by the other bead. The dependency of the bet on this bead is in place.
- **pair** `BET-33` / `N28b-36`
  - reported by G00: overlap with distinct scope, no merge: both record the decision KILL-REUSE. The kill-gate bead applies the trigger and writes the kill ADR; the bet bead reads that record, evaluates the bet's falsifier clauses and writes the bet verdict. The bet bead's acceptance line "the KILL-REUSE decision recorded" means on record by the other bead. The dependency of the bet on this bead is in place.

## Cluster of 3 beads

- `N14b-01` (U14b, M1, task) frob-design: the open modular class; a seeded generator of random designs and typed edits for property tests  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U14b/N14b-01.md`
- `N14b-18` (U14b, M1, task) frob-conformance: the preflight seeded-defect corpus for stages 1 to 6 and the first rejection fixtures  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U14b/N14b-18.md`
- `N28a-12` (U28a, M1, task) test fixtures: the seeded-defect corpus for preflight stages 1 to 6 and the valid-design generator  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U28a/N28a-12.md`

- **pair** `N14b-01` / `N28a-12`
  - reported by G14: N28a-12 also contains a generator of random valid designs
  - reported by G28: N28a-12 also contains a generator of random valid designs, which N14b-01 owns
- **pair** `N14b-18` / `N28a-12`
  - reported by G14: both build the seeded-defect corpus for preflight stages 1 to 6
  - reported by G28: both build the seeded-defect corpus for preflight stages 1 to 6

## Cluster of 3 beads

- `N24a-41` (U24a, M2, feature) verb telemetry.export-html: the replay packet writer, one offline HTML file sized before it is written  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U24a/N24a-41.md`
- `N32b-40` (U32b, M2, feature) viewer: playing a replay packet: packet source, block verification, WebGL 2 policy, the in-page probe  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U32b/N32b-40.md`
- `N32b-41` (U32b, M3, feature) frob-chronicle: replay packets with forks: 16 fork timelines, the tables block, the licence switch  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U32b/N32b-41.md`

- **pair** `N24a-41` / `N32b-40`
  - reported by G32: the packet's block layout is restated by the reader; the writer owns the format, and the reader depends on it
- **pair** `N24a-41` / `N32b-41`
  - reported by G24: fork timelines and the tables block of the replay packet are specified in both; N32b-41 states it extends the parent-only writer of N24a-41
  - reported by G32: fork timelines and the tables block of the replay packet are specified in both; N32b-41 extends the parent-only writer and depends on it

## Cluster of 2 beads

- `BD-067` (U24b, M1, task) tools/oracle: capture scripts and pinned, dated oracle fixtures for reference robots; no regeneration path  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U24b/BD-067.md`
- `NEG-15` (UEG, M1, task) assets/robots: layout, manifest `frob.robot.asset/1`, licence check and validation for reference robots  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/NEG-15.md`

- **pair** `BD-067` / `NEG-15`
  - reported by GEG: two records of source, commit and licence for the same reference robots of table G.1: other has tools/oracle/models.toml beside the oracle fixtures, mine assets/robots/<name>/ manifests frob.robot.asset/1 with the licence check and validation

## Cluster of 2 beads

- `BD-221` (U21a, M2, task) decision KILL-KIR: derivation toolchain measured: share expressed and emitted, speed against BET-01's bound  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U21a/BD-221.md`
- `BET-01` (U00, M2, task) Decide BET-01: the algebra pays for itself (derived kernels within 1.3× of hand-tuned; types remove bug classes)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U00/BET-01.md`

- **pair** `BD-221` / `BET-01`
  - reported by G00: overlap with distinct scope, no merge: both record the decision KILL-KIR. The kill-gate bead applies the trigger and writes the kill ADR; the bet bead reads that record, evaluates the bet's falsifier clauses and writes the bet verdict. The bet bead's acceptance line "the KILL-KIR decision recorded" means on record by the other bead. The dependency of the bet on this bead is in place.

## Cluster of 2 beads

- `BD-317` (U39b, M3, task) frob-conformance: the partial ten-fold gate at M3, adjudication, scoring, outcome rules and the gate record  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U39b/BD-317.md`
- `N40-15` (U40, M3, task) program gate GATE-TENFOLD-06: the partial ten-fold gate at M3, recorded with its outcome rules applied  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U40/N40-15.md`

- **pair** `BD-317` / `N40-15`
  - reported by G39: both apply the outcome rules per cell and record GATE-TENFOLD-06 with its ADRs
  - reported by G40: both record GATE-TENFOLD-06 and apply the outcome rules per cell and write the ADRs; N40-15 depends on BD-317

## Cluster of 2 beads

- `BET-24` (U00, M3, task) Decide BET-24: a requirement language as acceptance metric and falsification score, beside shaped rewards  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U00/BET-24.md`
- `N24a-15` (U24a, M3, task) Decision KILL-SPEC-REWARD: keep or remove the requirement-derived reward term; is margin-only training offered  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U24a/N24a-15.md`

- **pair** `BET-24` / `N24a-15`
  - reported by G00: overlap with distinct scope, no merge: both record the decision KILL-SPEC-REWARD. The kill-gate bead applies the trigger and writes the kill ADR; the bet bead reads that record, evaluates the bet's falsifier clauses and writes the bet verdict. The bet bead's acceptance line "the KILL-SPEC-REWARD decision recorded" means on record by the other bead. The dependency of the bet on this bead is in place.

## Cluster of 2 beads

- `N00-07` (U00, M0, task) frob-conformance: the pilot ledger (`pilots.toml`) for Frontier and Moonshot mechanisms, and the funding refusal  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U00/N00-07.md`
- `N38a-09` (U38a, M0, task) frob-conformance: the pilot register pilots.toml, protocols frozen before execution, and the funding rule  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U38a/N38a-09.md`

- **pair** `N00-07` / `N38a-09`
  - reported by G00: true duplicate: both create frob-conformance/registry/pilots.toml with its schema, registration before the run, and the funding refusal for a Frontier or Moonshot mechanism without a pilot (GATE-GAUNT-08). N00-07 also holds the completeness check, the twenty plan-time rows as history and the seed list of pilots still to run; N38a-09 also holds the printout. One file and one rule must remain (coverage (e) item 1 of U00)
  - reported by G38: true duplicate (both create frob-conformance/registry/pilots.toml, registration before the run and the funding refusal for GATE-GAUNT-08). Keep N38a-09: it is the owning chapter's bead (§38.5), 25 pilot beads of ten units already depend on it by key, and its record is the fuller one (append-only results, protocol hash and ledger sequence, sites, verdicts, printout, e2e script). Fold from N00-07: the completeness check (every Frontier or Moonshot capability record, kernel-manifest entry and verb row is named by a pilot), the Moonshot cap of four as a refusal, the rule that a fallback names a lower-tier mechanism, kill_gate must exist in the gate register, and the seed list of 23 pilots still to run with their kill gates and thresholds. The 18 beads that depend on N00-07 (BET-15 to BET-37, N00-09) then depend on N38a-09

## Cluster of 2 beads

- `N10a-21` (U10a, M0, task) pilot C-14: the M0 kernels derived, emitted on three back ends and certified  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U10a/N10a-21.md`
- `N21a-13` (U21a, M0, task) pilot: derived kernel arithmetic: three M0 bodies derived, emitted on three backends, certified, timed  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U21a/N21a-13.md`

- **pair** `N10a-21` / `N21a-13`
  - reported by G10: same pilot: three M0 kernel bodies derived, emitted on three back ends and certified
  - reported by G21: the M0 pilot of construction C-14: kernels derived, emitted on three back ends and certified

## Cluster of 2 beads

- `N14a-02` (U14a, M1, task) frob-design: component kinds and their port signatures on the design term; port instances for junction checks  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U14a/N14a-02.md`
- `N15b-01` (U15b, M1, task) frob-design: port junctions: kind, frame and sign checks, zero power sum, causality and loop refusal  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U15b/N15b-01.md`

- **pair** `N14a-02` / `N15b-01`
  - reported by G14: port signatures and port instances adjoin the junction checks (coverage (e) 12 of U14a)

## Cluster of 2 beads

- `N17a-08` (U17a, M2, task) e2e phylum.plan: script for buckets, padding, bytes, placement and refusals 160, 740, 741  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U17a/N17a-08.md`
- `N21b-27` (U21b, M2, task) e2e: `phylum_plan.sh`: bucket plans, padding known answers, placement table and refusals of `frob phylum plan`  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U21b/N21b-27.md`

- **pair** `N17a-08` / `N21b-27`
  - reported by G17: same end-to-end script scripts/e2e/phylum_plan.sh
  - reported by G21: two end-to-end beads for verb phylum.plan

## Cluster of 2 beads

- `N19a-02` (U19a, M2, task) verb family scene.*: registry rows for the Habitat verbs and the bounded wait of the four long ones  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U19a/N19a-02.md`
- `VERBS-scene` (UJL, M2, task) frob-verbs registry: cluster `scene`, thirteen rows, with parity, e2e and schema-snapshot checks  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UJL/VERBS-scene.md`

- **pair** `N19a-02` / `VERBS-scene`
  - reported by G19: both enter the registry rows of the scene verb cluster (N19a-02 twelve names with the bounded wait, VERBS-scene thirteen rows with parity and snapshot checks); VERBS-scene prints scene.retrofit at M3, the gate register has its catalog and gate at M4
  - reported by GJL: both enter the registry rows of the scene cluster: twelve names there, thirteen here (also reported by G19)

## Cluster of 2 beads

- `N25a-07` (U25a, M1, task) frob-mission: four proof rules as a battery: one accepted fixture and the invalid fixtures each rule must reject  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U25a/N25a-07.md`
- `N25d-36` (U25d, M1, task) PO-AUGUR-06: the four proof rules as candidate theorem cards on finite and polyhedral fragments  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U25d/N25d-36.md`

- **pair** `N25a-07` / `N25d-36`
  - reported by G25ab: both state the four proof rules with an invalid fixture each: N25a-07 as the battery in frob-mission, N25d-36 as the candidate theorem cards of PO-AUGUR-06
  - reported by G25cde: overlap: both pair each of the four proof rules with a deliberately invalid fixture; N25d-36 adds the candidate theorem cards

## Cluster of 2 beads

- `N26b-07` (U26b, M4, task) e2e: screening audit on planted screens and 1,000 campaigns (`scripts/e2e/nemesis_audit.sh`), GATE-NEMESIS-07  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U26b/N26b-07.md`
- `N28b-31` (U28b, M4, task) e2e screening audits: clean and biased screens, labels and re-opening, the prune audit, two gate batteries  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U28b/N28b-31.md`

- **pair** `N26b-07` / `N28b-31`
  - reported by G26: two end-to-end scripts record GATE-NEMESIS-07: scripts/e2e/nemesis_audit.sh and scripts/e2e/screening_audit.sh
  - reported by G28: confirmed overlap: two scripts record GATE-NEMESIS-07 (screening_audit.sh and nemesis_audit.sh). N26b-07 should record that gate; N28b-31 keeps the cascade's application (GATE-ATEL-09: trust rule, labels and re-opening, prune soundness) and should invoke nemesis_audit.sh for the audit steps

## Cluster of 2 beads

- `N31b-25` (U31b, M5, task) Owner decision OPEN-FOUNDRY-2: parts catalogue and price data, under what licence; default no prices  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U31b/N31b-25.md`
- `N41-13` (U41, M5, task) Owner decision OPEN-SKEL-2, OPEN-FOUNDRY-2: sources and licences for card prices and the parts catalogue  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U41/N41-13.md`

- **pair** `N31b-25` / `N41-13`
  - reported by G31: both are the owner decision OPEN-FOUNDRY-2 (parts catalogue and price data, licence; default no prices); N41-13 also decides OPEN-SKEL-2
  - reported by G41: both decide OPEN-FOUNDRY-2 (sources and licences of the parts catalogue and its prices); N41-13 also carries OPEN-SKEL-2

## Cluster of 2 beads

- `N39b-21` (U39b, M7, task) e2e tenfold_kit.sh: build the kit, recompute level 1, re-run level 2, refuse a private input (GATE-TENFOLD-08)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U39b/N39b-21.md`
- `N40-17` (U40, M7, task) program gate GATE-TENFOLD-08: the replication kit recomputes every number and re-runs a pair per stratum  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U40/N40-17.md`

- **pair** `N39b-21` / `N40-17`
  - reported by G39: both record GATE-TENFOLD-08
  - reported by G40: both measure GATE-TENFOLD-08 (level 1 recomputation, level 2 re-run, refusal of a private input); N40-17 is made to depend on N39b-21

