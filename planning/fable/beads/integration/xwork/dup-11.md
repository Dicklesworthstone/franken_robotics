# Cross-group overlap packet dup-11

36 pairs in 21 clusters. Each cluster is a set of beads connected by reported overlaps; decide a cluster as a whole.

## Cluster of 6 beads

- `BD-171` (U21b, M2, feature) frob-phylum: size buckets, the exact bucket planner, verb phylum.plan (padding, bytes, placement; no physics)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U21b/BD-171.md`
- `BD-219` (U36, M2, feature) verb sim.estimate: the cost and memory model, device budgets, refusal before allocation (GATE-SPARK-22)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U36/BD-219.md`
- `N16b-26` (U16b, M2, feature) verb sim.estimate: capacities, bytes per world, expected `Degraded` rate and expected referee coverage  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U16b/N16b-26.md`
- `N17a-04` (U17a, M2, task) frob-phylum: padding and bucket arithmetic: Ω_p, its bounds, the pay-off inequality, the mixing ratio  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U17a/N17a-04.md`
- `N17a-05` (U17a, M2, task) frob-phylum: bucket planner: exact dynamic program, cap of eight buckets, default set, FROB-REF-741  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U17a/N17a-05.md`
- `N17a-07` (U17a, M2, task) frob-phylum: population memory model and placement verdicts per machine, with FROB-REF-740  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U17a/N17a-07.md`

- **pair** `BD-171` / `N17a-04`
  - reported by G17: padding-overhead arithmetic and the break-even rule appear in both
- **pair** `BD-171` / `N17a-05`
  - reported by G17: the exact bucket planner (dynamic program, cap of eight, FROB-REF-741) is specified in both; BD-171 is the seed and should be the capstone over N17a-04, N17a-05, N17a-07
  - reported by G21: the exact bucket planner (dynamic program, cap of eight, default set, FROB-REF-741): a seed and a component written by another unit; BD-171 should be the capstone
- **pair** `BD-171` / `N17a-07`
  - reported by G17: the memory and placement table and FROB-REF-740 appear in both
- **pair** `BD-219` / `N16b-26`
  - reported by G16: one verb, sim.estimate: BD-219 is the cost and memory model, N16b-26 adds capacities, bytes per world, the expected Degraded rate and referee coverage
  - reported by G36: one verb, sim.estimate: BD-219 is the model, record and gate; N16b-26 adds the four Kinesis groups and depends on it. Distinct scope; both Scope sections say so
- **pair** `BD-219` / `N17a-07`
  - reported by G17: one byte model per padded world printed by phylum.plan and by sim.estimate (also N16b-26); one owner needed
  - reported by G36: one byte model per padded world printed by phylum.plan and by sim.estimate; the per-world terms should live once, in the estimator

## Cluster of 4 beads

- `FD-FSIM-18` (UF, M4, task) upstream frankensim: the 10⁶ sample caps of `fs-eproc` and of `fs-uq` importance execution parameterized  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FSIM-18.md`
- `FD-FSIM-19` (UF, M4, task) upstream frankensim: an adaptive multilevel splitting estimator beside the importance sampler of `fs-uq`  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UF/FD-FSIM-19.md`
- `N26b-22` (U26b, M3, task) OPEN-NEMESIS-4: the sibling 10⁶ sample caps and missing splitting: admission tests, interim posture, upstream rows  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U26b/N26b-22.md`
- `N26b-24` (U26b, M4, task) frob-nemesis: switch-over when FD-FSIM-18 or FD-FSIM-19 lands: lifted sample caps, upstream splitting estimator  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U26b/N26b-24.md`

- **pair** `FD-FSIM-18` / `N26b-22`
  - reported by G26: the upstream registry row and failing fixture of the 10⁶ sample caps are specified in both (and opened by BD-037)
  - reported by GF: the upstream registry row and failing fixture of the 10⁶ sample caps are specified in both
- **pair** `FD-FSIM-18` / `N26b-24`
  - reported by G26: the switch-over after the caps are lifted (re-run of the gates at the larger count) is in both
  - reported by GF: the switch-over after the caps are lifted (re-run of the gates at the larger count) is in both; N26b-24 depends on FD-FSIM-18
- **pair** `FD-FSIM-19` / `N26b-22`
  - reported by G26: the upstream registry row and failing fixture for the missing splitting estimator are specified in both
  - reported by GF: the upstream registry row and failing fixture of the missing splitting estimator are specified in both
- **pair** `FD-FSIM-19` / `N26b-24`
  - reported by G26: the differential test of the two splitting estimators and the choice between them is in both
  - reported by GF: the differential test of the two splitting estimators and the choice between them is in both; N26b-24 depends on FD-FSIM-19

## Cluster of 4 beads

- `BD-207` (U29a, M2, feature) frob-chronicle: telemetry on devices: T0 to T2 in the step, traced-world mask, histograms, drain at the fence  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U29a/BD-207.md`
- `N21c-40` (U21c, M2, task) frob-chronicle: drain of 32-bit device bins into 64-bit ledger totals at every chunk commit, exactly once  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U21c/N21c-40.md`
- `N29a-13` (U29a, M2, task) frob-chronicle: deterministic integer histograms: quantize, octave sub-buckets, merge by add, interval quantiles  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U29a/N29a-13.md`
- `N35-21` (U35, M2, task) frob-core: order-independent integer histograms over `f32` bit patterns; quantiles as bin intervals  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U35/N35-21.md`

- **pair** `BD-207` / `N21c-40`
  - reported by G21: the drain of device histogram bins at the commit fence appears in both
  - reported by G29: the drain of device histogram bins at the commit fence appears in both
- **pair** `BD-207` / `N35-21`
  - reported by G35: two histogram parametrizations: f32 pattern bins here, channel quantum there
- **pair** `N29a-13` / `N35-21`
  - reported by G29: deterministic integer histograms with interval quantiles in frob-chronicle (quantized integer, 3.1%) and in frob-core (f32 bit pattern, 3.2%); one binning must govern the evidence path
  - reported by G35: integer histograms with interval quantiles in frob-core (f32 pattern, 3.125%) and in frob-chronicle (quantized integer, 3.1%); one binning must govern

## Cluster of 4 beads

- `N29a-23` (U29a, M2, feature) verbs job.list, job.status, job.wait, job.find, job.cancel: the job handle contract from the caller's side  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U29a/N29a-23.md`
- `N32a-09` (U32a, M1, task) frob-verbs: the admission transaction: atomic binding of principal, verb, key, digest, reservation, job  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U32a/N32a-09.md`
- `N32a-16` (U32a, M2, feature) frob-verbs, frob-mcp, frob-cli: bounded synchronous wait: `sync_wait_ms`, exit 8 with a job handle  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U32a/N32a-16.md`
- `N32a-17` (U32a, M2, feature) frob-mcp, frob-cli: progress with denominators; a cancelled wait is not a cancelled job  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U32a/N32a-17.md`

- **pair** `N29a-23` / `N32a-09`
  - reported by G32: job.find by idempotency key is specified in both, and again in BD-211
- **pair** `N29a-23` / `N32a-16`
  - reported by G29: job.wait semantics and the bounded wait are in both; job.find is in N29a-23, N32a-09 and BD-211; job.cancel semantics also in N32a-17. N32a-08 names BD-210 as the owner of the job verbs' registry entries, which are N29a-23
  - reported by G32: job.wait semantics and the bounded wait are in both; N29a-23 owns the registry entries of the five job verbs and depends on N32a-16
- **pair** `N29a-23` / `N32a-17`
  - reported by G32: job.cancel semantics (drain, cancelled wait is not a cancelled job) are in both

## Cluster of 3 beads

- `BD-300` (U33a, M3, feature) frob-tui dashboard (INT-FTUI-01), viewer overlays with the FrankenThreeD route (INT-F3D-01); KILL-TUI recorded  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U33a/BD-300.md`
- `N32b-06` (U32b, M3, feature) frob-tui: the dashboard core: a pure model over verb envelopes and the daemon's event stream, read-only  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U32b/N32b-06.md`
- `N32b-10` (U32b, M2, feature) viewer: the Three.js r186 page: immutable snapshot sources, timeline, anchored poses, picking, test entry  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U32b/N32b-10.md`

- **pair** `BD-300` / `N32b-06`
  - reported by G32: a seed and its components, not a duplicate: BD-300 (admission INT-FTUI-01, GATE-TETHER-14 record, KILL-TUI outcome) is the capstone of the dashboard beads N32b-06 to N32b-09 and N32b-31; its Scope should say so (dependencies recorded under edges)
  - reported by G33: seed and components: the dashboard and viewer parts are N32b-06 to N32b-14; BD-300's scope now says it is their capstone and reads the decision N32b-31
- **pair** `BD-300` / `N32b-10`
  - reported by G32: a seed and its components: BD-300 lists the viewer overlays and the FrankenThreeD route, which are built by N32b-10 to N32b-14; its Scope should say so (dependencies recorded under edges)

## Cluster of 3 beads

- `GB-22` (UEG, M2, task) card GB-22 (frob-contact): why half precision is rejected for contact solves: κ·u₁₆ = 2.34, cost 9k/n  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/GB-22.md`
- `N18b-27` (U18b, M2, task) frob-contact: half precision is not built: refusal FROB-REF-245 and the refuting calculation as known answers  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U18b/N18b-27.md`
- `N21c-25` (U21c, M2, task) Rejected mechanisms I: four precision and statistics ideas with their refuting calculations as tests  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U21c/N21c-25.md`

- **pair** `GB-22` / `N18b-27`
  - reported by GEG: same refuting calculation for half precision (κ·u₁₆ = 2.34, refinement cost 9k/n): other carries it as known answers with refusal FROB-REF-245 (a dependency of mine), mine as the card; DX-01 of NAB-41 pins 9k/n = 3.0
- **pair** `N18b-27` / `N21c-25`
  - reported by G21: half precision is not built: the refuting calculation as a test appears in both

## Cluster of 3 beads

- `N16b-27` (U16b, M2, task) frob-verbs: registry entries of the `fidelity` cluster and the Kinesis verb extensions, with the naming question  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U16b/N16b-27.md`
- `VERBS-fidelity` (UJL, M2, task) frob-verbs registry: cluster `fidelity`, five rows, with parity, e2e and schema-snapshot checks  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UJL/VERBS-fidelity.md`
- `VERBS-sim` (UJL, M0, task) frob-verbs registry: cluster `sim`, three rows, with parity, e2e and schema-snapshot checks  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UJL/VERBS-sim.md`

- **pair** `N16b-27` / `VERBS-fidelity`
  - reported by G16: both hold the registry rows of the fidelity cluster; N16b-27 also carries the sim.run and sim.estimate extensions (overlapping VERBS-sim) and the fidelity.rank naming question
  - reported by GJL: both hold the registry rows of the fidelity cluster (also reported by G16)
- **pair** `N16b-27` / `VERBS-sim`
  - reported by GJL: N16b-27 also carries the registry extensions of sim.run and sim.estimate

## Cluster of 3 beads

- `N32a-25` (U32a, M3, feature) frob-mcp: resources under `frob://`: docs, registry, schemas, catalogs, stills, reports, jobs; 4 MiB cap  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U32a/N32a-25.md`
- `N32b-18` (U32b, M1, task) docs single source: the skill file, frob://docs resource texts and MCP prompt texts, with a drift check  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U32b/N32b-18.md`
- `NJL-06` (UJL, M1, feature) frob-mcp: the resource frob://docs/clusters, the task-cluster directory generated from the registry, its test  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UJL/NJL-06.md`

- **pair** `N32a-25` / `NJL-06`
  - reported by G32: the resource frob://docs/clusters is built in both
  - reported by GJL: both register and serve the frob://docs resources on the MCP server; NJL-06 has the cluster directory at M1, N32a-25 all resources at M3
- **pair** `N32b-18` / `NJL-06`
  - reported by GJL: both generate the text of the resource frob://docs/clusters from the registry

## Cluster of 2 beads

- `BD-100` (U28a, M1, feature) frob-analyze: A-02 mass roll-up, and A-08 static stability typed by bound side and load-sharing model  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U28a/BD-100.md`
- `N14a-10` (U14a, M1, task) frob-model: the budget functor: per-subtree summaries, the budget graph, and budget findings (FROB-REF-115)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U14a/N14a-10.md`

- **pair** `BD-100` / `N14a-10`
  - reported by G14: per-subtree mass summaries adjoin the A-02 mass roll-up (coverage (e) 12 of U14a)

## Cluster of 2 beads

- `BD-245` (U14a, M3, feature) frob-contact: laws `belt`, `skid`, `pad`, `roller-wheel`, `wheel-rolling`, all tiers; scrub-work known answer  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U14a/BD-245.md`
- `N18b-06` (U18b, M4, feature) frob-contact: belt tracks: drive-tied belt surface, anisotropic ellipse, distributed patches and yaw scrub  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U18b/N18b-06.md`

- **pair** `BD-245` / `N18b-06`
  - reported by G14: the scrub-moment derivation μ·W·L/4 is written in both (coverage (e) 2 of U14a)
  - reported by G18: the belt law: BD-245 delivers law name bindings and a fallback form at M3, N18b-06 the funded belt model at M4; N18b-02 and the roller-wheel law likewise

## Cluster of 2 beads

- `BET-02` (U00, M2, task) Decide BET-02: one kernel source, three backends, in declared determinism classes; worlds independent of batch  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U00/BET-02.md`
- `N21b-34` (U21b, M2, task) Kill-gate decision KILL-D1 per device family: bit equality with `batch-cpu`, or D2 for itself and D3 to others  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U21b/N21b-34.md`

- **pair** `BET-02` / `N21b-34`
  - reported by G00: overlap with distinct scope, no merge: both record the decision KILL-D1. The kill-gate bead applies the trigger and writes the kill ADR; the bet bead reads that record, evaluates the bet's falsifier clauses and writes the bet verdict. The bet bead's acceptance line "the KILL-D1 decision recorded" means on record by the other bead. The dependency of the bet on this bead is in place.

## Cluster of 2 beads

- `BET-31` (U00, M3, task) Decide BET-31: campaign quotienting (exact deduplication under symmetry and observables, losing no winner)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U00/BET-31.md`
- `N17a-29` (U17a, M3, task) decision KILL-QUOTIENT: keep the mission digest as a deduplication claim or fall back to orbits (M3)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U17a/N17a-29.md`

- **pair** `BET-31` / `N17a-29`
  - reported by G00: overlap with distinct scope, no merge: both record the decision KILL-QUOTIENT. The kill-gate bead applies the trigger and writes the kill ADR; the bet bead reads that record, evaluates the bet's falsifier clauses and writes the bet verdict. The bet bead's acceptance line "the KILL-QUOTIENT decision recorded" means on record by the other bead. The dependency of the bet on this bead is in place.
  - reported by G17: the kill-gate record and the bet decision read the same quotient audit; one must cite the other

## Cluster of 2 beads

- `N00-16` (U00, M0, task) tools/oracle: the foundation fact sheet: sibling-code facts with paths, re-read at pinned commits by a script  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U00/N00-16.md`
- `N33a-09` (U33a, M0, task) frob-conformance: sibling fact probes: FrankenSim's hazards and limits and the ledger notes, re-read at the pin  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U33a/N33a-09.md`

- **pair** `N00-16` / `N33a-09`
  - reported by G00: both build a registry of sibling-code facts with paths and a runner that re-reads them at the pinned commits and marks stale facts (N00-16: tools/oracle/recheck_foundation_facts.sh over the facts of the thesis chapter; N33a-09: fact probes in frob-conformance over the facts of the Suite chapter). One registry and one runner, with rows from both chapters
  - reported by G33: both build a registry of sibling-code facts with a runner that re-reads them at the pin and marks stale facts; one registry and one runner should remain (reported by G00 as well)

## Cluster of 2 beads

- `N10b-13` (U10b, M3, task) pilot PILOT-C-22: the ballast fixture end to end, with bit-identical replay of merged designs  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U10b/N10b-13.md`
- `N17a-25` (U17a, M2, task) pilot: mission-observable digest on the ballast fixture, end to end with replay, before funding  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U17a/N17a-25.md`

- **pair** `N10b-13` / `N17a-25`
  - reported by G10: same pilot: the ballast fixture end to end with bit-identical replay of merged designs
  - reported by G17: same pilot: the ballast fixture end to end with bit-identical replay of merged designs (N17a-25 at M2, PILOT-C-22 at M3)

## Cluster of 2 beads

- `N14b-06` (U14b, M1, task) e2e: the archetype catalog end to end: templates, instantiation, preflight, symmetry and standing rows  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U14b/N14b-06.md`
- `NEG-31` (UEG, M1, task) e2e: `scripts/e2e/zoo_goldens.sh`: every zoo archetype instantiated, preflighted and compared with its golden  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/NEG-31.md`

- **pair** `N14b-06` / `NEG-31`
  - reported by G14: two end-to-end scripts that instantiate and preflight every archetype (skel_archetypes.sh, zoo_goldens.sh)
  - reported by GEG: two end-to-end scripts instantiate and preflight every archetype (zoo_goldens.sh from expectation files, skel_archetypes.sh from the catalog); one gate record, BD-246's

## Cluster of 2 beads

- `N17b-12` (U17b, M2, task) verb cluster phylum: seven registry entries with schemas, refusal lists, options and the seven-verb limit test  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U17b/N17b-12.md`
- `VERBS-phylum` (UJL, M2, task) frob-verbs registry: cluster `phylum`, seven rows, with parity, e2e and schema-snapshot checks  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UJL/VERBS-phylum.md`

- **pair** `N17b-12` / `VERBS-phylum`
  - reported by G17: both hold the seven registry entries of cluster phylum with schemas, refusal lists and MCP names; N17b-12 also holds the option table
  - reported by GJL: both hold the seven registry entries of cluster phylum (also reported by G17)

## Cluster of 2 beads

- `N19a-40` (U19a, M5, task) pilot: paper-scale test of scene dressing (populated interiors, kinematic actors) against the proxies  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U19a/N19a-40.md`
- `N38a-32` (U38a, M6, task) pilot: hazard classes a moving actor can provoke that structural proxies cannot, for one archetype  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U38a/N38a-32.md`

- **pair** `N19a-40` / `N38a-32`
  - reported by G38: N19a-40 (M5) pilots scene dressing, kinematic actors included, against the proxies; N38a-32 (M6) pilots the actors alone on the warehouse hazard list. The actor half is decided twice

## Cluster of 2 beads

- `N25a-27` (U25a, M4, task) e2e ceiling_before_speed.sh: comparisons admitted or refused by ceiling; unregistered-ratio audit (GATE-AUGUR-14)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U25a/N25a-27.md`
- `N25c-06` (U25c, M4, task) pilot: time an amended-mission workflow and measure f, r, h and the unremovable floor before any family kernel  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U25c/N25c-06.md`

- **pair** `N25a-27` / `N25c-06`
  - reported by G25ab: overlap with distinct scope, connected by a dependency: N25c-06 is the pilot that measures f, r and h of one amended-mission workflow, N25a-27 the script that audits registered speed comparisons and reads that record

## Cluster of 2 beads

- `N27a-09` (U27a, M2, task) GATE-AUTOPSY-07: the first-divergence battery: planted ticks and stages, the pointwise mutant, tolerance gaps  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U27a/N27a-09.md`
- `N29a-21` (U29a, M2, task) GATE-CHRON-06: first divergence: two-tick case 100 of 100, loose bound refined, missing samples as gap; e2e  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U29a/N29a-21.md`

- **pair** `N27a-09` / `N29a-21`
  - reported by G27: two gate batteries on one mechanism with the same fixtures: planted tick and stage through the conformance fault hook, the two-tick case, the pointwise mutant, loose bound and missing samples (GATE-AUTOPSY-07 in N27a-09, GATE-CHRON-06 in N29a-21). The plan wants the fixtures shared and each gate compared with its own row; the fault hook and the trace fixtures should be built once

## Cluster of 2 beads

- `N32a-34` (U32a, M4, task) pilot and owner decision OPEN-TETHER-5: does a peer link between the two machines earn its upkeep  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U32a/N32a-34.md`
- `N41-12` (U41, M4, task) Owner decision OPEN-TETHER-5: the peer link between the two machines earns its upkeep, or they stay standalone  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U41/N41-12.md`

- **pair** `N32a-34` / `N41-12`
  - reported by G41: both decide OPEN-TETHER-5 (the peer link or standalone machines); N32a-34 already absorbed N32b-36

## Cluster of 2 beads

- `N39b-47` (U39b, M3, task) e2e tenfold_recovery_ablation.sh: GATE-TENFOLD-13: recovery ratios, ablations in bounds, one-receipt record  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U39b/N39b-47.md`
- `N40-22` (U40, M3, task) program gate GATE-TENFOLD-13: three arms, recovery ratios with intervals, phase ablations inside their bounds  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U40/N40-22.md`

- **pair** `N39b-47` / `N40-22`
  - reported by G39: both record GATE-TENFOLD-13
  - reported by G40: both measure GATE-TENFOLD-13 (recovery ratios, ablations inside their bounds, the one-receipt record); BD-317 records its M3 part and N39b-13 its M7 row; N40-22 is made to depend on N39b-47

