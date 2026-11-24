# Cross-group overlap packet dup-05

36 pairs in 21 clusters. Each cluster is a set of beads connected by reported overlaps; decide a cluster as a whole.

## Cluster of 6 beads

- `N00-01` (U00, M0, task) frob-conformance: evidence-label, wording and notation lints over contracts, docs, templates and tool output  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U00/N00-01.md`
- `N00-06` (U00, M0, task) frob-conformance: the plan-time check ledger: nine recorded checks re-run here before any gate cites one  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U00/N00-06.md`
- `N12-08` (U12, M1, task) frob-lemma: evidence-class wording, the ban on the word verified, and the non-claims scope block  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U12/N12-08.md`
- `N38a-10` (U38a, M0, task) frob-conformance: the plan-time pilot ledger: twenty recorded pilots with expected values and limits  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U38a/N38a-10.md`
- `NAB-10` (UAB, M0, task) frob-conformance: citation lint: no gate, contract or close reason cites a source that still needs a reading  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UAB/NAB-10.md`
- `NAB-15` (UAB, M0, task) registries/sources.toml: the 25 plan-time checks and the reproduction archive pinned by commit and digest  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UAB/NAB-15.md`

- **pair** `N00-01` / `N12-08`
  - reported by G00: both test the ban on the word `verified`: N00-01 lints contracts, documents, templates and tool output in general; N12-08 tests frob-lemma's output. Distinct scope; related
  - reported by G12: both lint the banned word 'verified': N12-08 on frob-lemma's output and exports, N00-01 on documents, templates, refusal messages and help text
- **pair** `N00-01` / `NAB-10`
  - reported by GAB: lint L-SOURCE of N00-01 and the citation lint both refuse a gate row that cites a source whose reread_before_gate flag is set; NAB-10 depends on N00-01 and should be the one place that grades citations
- **pair** `N00-06` / `N38a-10`
  - reported by G00: the same plan-time checks held as ledger rows twice: N00-06 has nine check records with re-run status, consistency assertions and the citation rule (Appendix K.3); N38a-10 has twenty pilot rows with expected values and limits (§38.5). The re-run status should live in one ledger
  - reported by G38: the same plan-time checks held as two ledgers: N38a-10 has the twenty pilot rows of §38.5 with expected values, owners and limits inside pilots.toml; N00-06 has nine check records with re-run status and a citation rule. Keep N38a-10 as the one ledger (it sits in the register N38a-09 that survives) and fold N00-06's consistency assertions and citation rule into it
- **pair** `N00-06` / `NAB-10`
  - reported by GAB: the citation rule of N00-06 (a gate may cite only reproduced checks) is the finding cite.plan-check-not-reproduced of NAB-10
- **pair** `N00-06` / `NAB-15`
  - reported by G00: overlap with distinct scope: NAB-15 enters the same checks as 25 source records with the reproduction archive's commit and digest; N00-06 holds their re-run status and depends on NAB-15
  - reported by GAB: overlap with distinct scope: NAB-15 enters the plan-time checks as 25 source records with the archive's revisions; N00-06 holds their re-run status in 14 grouped records and depends on NAB-15; the two are joined by id
- **pair** `N38a-10` / `NAB-15`
  - reported by GAB: the same plan-time checks are held a third time as twenty pilot rows with expected values; one list of recorded values should be the source

## Cluster of 5 beads

- `N23b-30` (U23b, M2, task) frob-conformance: the joint-law battery for action samplers with the planted sampler; GATE-CORTEX-18  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U23b/N23b-30.md`
- `N33c-37` (U33c, M2, task) frob-conformance: planted linear-phase action sampler (the sibling's joint-law defect), the control of rule P-12  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U33c/N33c-37.md`
- `N39b-36` (U39b, M3, task) e2e tenfold_preanalysis.sh: GATE-TENFOLD-10: pre-analysis known answers and 2,000 rare-tail campaigns  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U39b/N39b-36.md`
- `N40-19` (U40, M3, task) program gate GATE-TENFOLD-10: the pre-analysis known answers and the smallest suite that can certify  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U40/N40-19.md`
- `N41-31` (U41, M3, task) GATE-WS-07 at M3: the rare-tail suite arm raises its signal 100 of 100; the gate closed from its four rows  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U41/N41-31.md`

- **pair** `N23b-30` / `N33c-37`
  - reported by G23: both build the planted linear-phase sampler; one fixture should serve the battery and GATE-WS-06
  - reported by G33: both build the planted linear-phase sampler; one fixture should serve the joint-law battery and GATE-WS-06 (as G23 recorded)
- **pair** `N33c-37` / `N41-31`
  - reported by G41: the planted linear-phase sampler: N33c-37 builds it as a control, N41-31 fires it 100 times for the M2 row of GATE-WS-07 (the Cortex group reports the same fixture against N23b-30)
- **pair** `N39b-36` / `N40-19`
  - reported by G39: both record GATE-TENFOLD-10
  - reported by G40: both measure GATE-TENFOLD-10 (the pre-analysis known answers and the 2,000 rare-tail campaigns); N40-19 is made to depend on N39b-36
- **pair** `N40-19` / `N41-31`
  - reported by G41: the rare-tail candidate simulation and the refusal of an undersized population claim are run by both (and by the end-to-end script N39b-36); N41-31 measures them as the M3 row of GATE-WS-07, 100 of 100

## Cluster of 4 beads

- `BD-398` (U33a, M5, feature) frob-scene: real-place ingestion (`scene.import`, `scene.fit`), scale labels, unknown regions; INT-FDGR-01  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U33a/BD-398.md`
- `N19c-04` (U19c, M5, task) frob-scene: real-place ingest: twin-package adapter, what import adds, assumed cards, loss report, privacy  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U19c/N19c-04.md`
- `N19c-08` (U19c, M5, task) Decide KILL-INGEST: can the real-place gate pass at scale label witnessed or better after one repair cycle  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U19c/N19c-08.md`
- `N33c-25` (U33c, M3, task) pilot INT-FDGR-01 (schema half): scan-import schema adapter, non-metric refusal, coverage as unknown space  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U33c/N33c-25.md`

- **pair** `BD-398` / `N19c-04`
  - reported by G19: seed and components: BD-398's Scope restates registration, scale labels, the unknown mask, assumed cards and privacy, which N19c-02 to N19c-05 build; BD-398 should become the capstone over them (edges recorded)
  - reported by G33: seed and components: registration, scale labels, the unknown mask, assumed cards and privacy are built by N19c-02 to N19c-05; BD-398's scope now says it is their capstone, and KILL-INGEST is decided by N19c-08
- **pair** `BD-398` / `N19c-08`
  - reported by G19: BD-398 names `the record of KILL-INGEST` in its Scope; the decision bead is N19c-08, which depends on the gate script N19c-07
- **pair** `N19c-04` / `N33c-25`
  - reported by G33: the drone schema's non-metric refusal is tested in both; N33c-25 holds the M3 schema adapter and fixtures, N19c-04 the M5 ingest that re-runs them

## Cluster of 3 beads

- `BD-063` (U14b, M1, task) frob-model: memoized incremental compilation with influence records and the invalidation set of an edit  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U14b/BD-063.md`
- `N10a-19` (U10a, M1, task) pilot C-12: a 64-edit sequence over one modular quadruped with four invalidation controls  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U10a/N10a-19.md`
- `N14a-15` (U14a, M1, task) Pilot: module-substitution reuse: hand-listed reusable products against a full-recompile diff, per archetype  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U14a/N14a-15.md`

- **pair** `BD-063` / `N10a-19`
  - reported by G14: BD-063's test `sixty_four_edit_sequence` reproduces the C-12 pilot that N10a-19 runs and records
- **pair** `N10a-19` / `N14a-15`
  - reported by G10: overlap: reuse under module substitution counted against a full recompile

## Cluster of 3 beads

- `BD-160` (U41, M2, task) frob-kernels: the batch-cpu tile program on fs-exec, fixed-slot reductions, bitwise equal across worker counts  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U41/BD-160.md`
- `N21a-07` (U21a, M2, task) frob-kernels: `K-reduce` and `K-seg-reduce`: fixed pairwise trees, neutral padding, two-limb integer sums  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U21a/N21a-07.md`
- `N35-13` (U35, M2, task) frob-kernels: padding-neutral reductions: neutral elements and the pairwise tree anchored at index 0  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U35/N35-13.md`

- **pair** `BD-160` / `N35-13`
  - reported by G41: both define the pairwise reduction tree anchored at index 0 and assert KA-786; BD-160 now depends on N35-13 and only shows that tile slots compose to that tree
- **pair** `N21a-07` / `N35-13`
  - reported by G21: padding-neutral reductions: neutral elements and the pairwise tree anchored at index 0
  - reported by G35: overlap: the reduction contract (neutral elements, anchored tree, host references) here, the kernels K-reduce and K-seg-reduce there

## Cluster of 3 beads

- `BET-22` (U00, M3, task) Decide BET-22: minimal counterexample worlds (adversarial search beats randomization; shrink to ≤ 3 edits)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U00/BET-22.md`
- `N26a-12` (U26a, M3, task) Decision KILL-NEMESIS: keep the adversarial search methods, or fall back to card randomization  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U26a/N26a-12.md`
- `N26a-35` (U26a, M3, task) Decision KILL-MCW: keep minimal counterexample worlds, or ship full failing worlds with ranked contrasts  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U26a/N26a-35.md`

- **pair** `BET-22` / `N26a-12`
  - reported by G00: overlap with distinct scope, no merge: both record the decision KILL-NEMESIS. The kill-gate bead applies the trigger and writes the kill ADR; the bet bead reads that record, evaluates the bet's falsifier clauses and writes the bet verdict. The bet bead's acceptance line "the KILL-NEMESIS decision recorded" means on record by the other bead. The dependency of the bet on this bead is in place.
- **pair** `BET-22` / `N26a-35`
  - reported by G00: overlap with distinct scope, no merge: both record the decision KILL-MCW. The kill-gate bead applies the trigger and writes the kill ADR; the bet bead reads that record, evaluates the bet's falsifier clauses and writes the bet verdict. The bet bead's acceptance line "the KILL-MCW decision recorded" means on record by the other bead. The dependency of the bet on this bead is in place.

## Cluster of 3 beads

- `N10a-14` (U10a, M4, task) pilot C-06: the hard-contact batch law and factor reuse on the contact known answers and one 21-contact stack  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U10a/N10a-14.md`
- `N18a-22` (U18a, M5, task) pilot: hard-contact fixed-point passes to a residual of 10⁻⁴ on 10⁴ battery contact states (`batch-cpu`)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U18a/N18a-22.md`
- `N18a-25` (U18a, M4, task) pilot: the share of response-batch columns that the mode guard reroutes on battery contact states  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U18a/N18a-25.md`

- **pair** `N10a-14` / `N18a-22`
  - reported by G10: same pilot subject: the hard-contact law as a batch law, fixed passes to a residual of 1e-4 on battery contact states
  - reported by G18: pilot of the hard-contact batch law (and of factor reuse, see N18a-25) on the contact known answers
- **pair** `N10a-14` / `N18a-25`
  - reported by G10: overlap: factor reuse on a fixed active set and the share of response columns the mode guard reroutes

## Cluster of 3 beads

- `N21a-01` (U21a, M0, task) workspace: Spark annex policy: annex workspaces, capsule registry, `apps/` workspace, the no-GPU-in-core plant  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U21a/N21a-01.md`
- `N34-01` (U34, M0, task) Workspace boundaries: annex, apps and worker workspaces, one lockfile each, backends linked only in apps  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U34/N34-01.md`
- `N34-07` (U34, M0, task) xtask check-unsafe: core deny lint, annex capsule registry with SAFETY.md, and the lint-table doorway scan  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U34/N34-07.md`

- **pair** `N21a-01` / `N34-01`
  - reported by G21: annex and apps workspace layout, lockfile and toolchain rules
  - reported by G34: annex, apps and worker workspace layout, one lockfile each, toolchain-file rules and the no-GPU-in-core plant (also reported by G21)
- **pair** `N21a-01` / `N34-07`
  - reported by G21: the capsule registry format and the SAFETY.md rules
  - reported by G34: the capsule registry format and the SAFETY.md section list (also reported by G21)

## Cluster of 2 beads

- `BD-042` (U23a, M0, task) Plan-time pilot, static stance family: re-run from the reproduction archive and record every figure  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U23a/BD-042.md`
- `GB-31` (UEG, M1, task) card GB-31 (frob-mission): how narrow the stance can be: b ≥ 3/14 m, with witnesses and a dual exclusion  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/GB-31.md`

- **pair** `BD-042` / `GB-31`
  - reported by GEG: same static stance family (boundary 3/14 m; 2,254, 114 and 1,596 grid counts): other re-runs the plan-time pilot from the archive, mine is the card; ART-L4 is the `.fmission` golden of the same query

## Cluster of 2 beads

- `BD-154` (U18a, M2, feature) frob-contact: the four-dimensional friction cone (normal, tangential, spin) in the `Cone` trait and cone kernels  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U18a/BD-154.md`
- `CARD-T-07` (U12, M5, task) Theorem card T-07: the friction-cone projection satisfies the Moreau identities (3- and 4-dimensional cones)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U12/CARD-T-07.md`

- **pair** `BD-154` / `CARD-T-07`
  - reported by G12: both issue the poly-identity certificates of the Moreau identities for the four-dimensional cone: BD-154 as PO-TRAC-01 at M2, the card at M5 through the coverage pipeline

## Cluster of 2 beads

- `BD-299` (U32a, M3, feature) frob, frob-mcp: the three failure classes on both surfaces over the full M3 registry; GATE-TETHER-04  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U32a/BD-299.md`
- `N35-26` (U35, M1, task) frob-conformance: the three standing rules of refusal as a sweep over the verb registry  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U35/N35-26.md`

- **pair** `BD-299` / `N35-26`
  - reported by G32: the three standing rules of refusal as a sweep over the registry there, the three failure classes on both surfaces here
  - reported by G35: overlap: the three standing rules as a sweep here, the three failure classes on both surfaces there

## Cluster of 2 beads

- `BET-15` (U00, M4, task) Decide BET-15: family policies (one per morphology class decides most designs without per-design training)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U00/BET-15.md`
- `N23a-26` (U23a, M4, task) Kill-gate decision KILL-FAMILY: keep the family policy or fall back to specialists with inheritance  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U23a/N23a-26.md`

- **pair** `BET-15` / `N23a-26`
  - reported by G00: overlap with distinct scope, no merge: both record the decision KILL-FAMILY. The kill-gate bead applies the trigger and writes the kill ADR; the bet bead reads that record, evaluates the bet's falsifier clauses and writes the bet verdict. The bet bead's acceptance line "the KILL-FAMILY decision recorded" means on record by the other bead. The dependency of the bet on this bead is in place.

## Cluster of 2 beads

- `GB-26` (UEG, M4, task) card GB-26 (frob-shield): can the stop absorb everything that can reach it: 1.49 J against 2.00 J  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/GB-26.md`
- `N22b-09` (U22b, M4, task) Pilot [F]: energy budget: worked card in exact arithmetic and a cart stop with μ, restitution and law swept  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U22b/N22b-09.md`

- **pair** `GB-26` / `N22b-09`
  - reported by GEG: the pilot recomputes this worked card in exact arithmetic (1.49 J against 2.00 J, 50.5 ms) before the mechanism is funded; mine is the standing golden with lane B on shield.budget

## Cluster of 2 beads

- `N10a-12` (U10a, M1, task) pilot C-01: lattice mobility on Sarrus, Bennett and an over-constrained leg linkage against the exact rank  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U10a/N10a-12.md`
- `N13-27` (U13, M1, task) Pilot: subgroup-lattice mobility by hand on the Sarrus and Bennett linkages against the exact rank  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U13/N13-27.md`

- **pair** `N10a-12` / `N13-27`
  - reported by G10: same pilot: subgroup-lattice mobility on the Sarrus and Bennett linkages against the exact rank
  - reported by G13: the same paper-scale pilot: lattice mobility on the Sarrus and Bennett linkages (and one leg linkage) against the exact rank; N13-28 depends on N13-27

## Cluster of 2 beads

- `N12-17` (U12, M4, feature) frob-cert: checkers for proof kinds energy-budget and product-unreachable  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U12/N12-17.md`
- `N22b-14` (U22b, M4, task) frob-cert, frob-lemma: proof kind `energy-budget` (T-28): bundle writer, offline checker, mutation corpus  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U22b/N22b-14.md`

- **pair** `N12-17` / `N22b-14`
  - reported by G12: both specify the energy-budget checker in frob-cert; keep the checker in N12-17 and the writer, run-replay recomputation and corpus in N22b-14, which should depend on N12-17; N22b-14's dependency on CARD-T-28 must not be folded into N12-17, because the card depends on the checker
  - reported by G22: both write the offline checker of proof kind `energy-budget` in frob-cert; N22b-14 also has the bundle writer and the mutation corpus

## Cluster of 2 beads

- `N16b-15` (U16b, M2, task) frob-conformance: the two planted exploits (penetration recovery, pyramid cone) and the planted rankings  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U16b/N16b-15.md`
- `N26a-15` (U26a, M3, task) frob-conformance: planted simulator artifacts and planted genuine failures for the artifact guard  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U26a/N26a-15.md`

- **pair** `N16b-15` / `N26a-15`
  - reported by G26: the same two solver mutations (pyramid4 cone, position-level penetration recovery) with a hopper and a diagonal-push task; N26a-15 should build its 100 planted failures on N16b-15's options (dep recorded)

## Cluster of 2 beads

- `N18b-35` (U18b, M6, task) Pilot [M]: tactile micro-patches: a fingertip pad as 2 × 2 and 4 × 4 contacts on strict against one patch  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U18b/N18b-35.md`
- `N38a-30` (U38a, M6, task) pilot: a fingertip pad as 2×2 and 4×4 four-dimensional contacts against one patch, on strict  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U38a/N38a-30.md`

- **pair** `N18b-35` / `N38a-30`
  - reported by G18: same pilot: a fingertip pad as 2x2 and 4x4 contacts against one patch on strict
  - reported by G38: same pilot: a fingertip pad as 2×2 and 4×4 contacts on strict against one patch; N20a-18 is a third pilot of the tactile array on an indentation fixture

## Cluster of 2 beads

- `N23a-15` (U23a, M2, task) frob-learn: the FrankenTorch bridge: CPU learner, gradient oracle, safetensors interchange, FD-FT-02 rule  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U23a/N23a-15.md`
- `N33a-22` (U33a, M2, task) frob-learn: FrankenTorch CPU crates admitted at M2 behind a feature: trainer, gradient oracle, interchange  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U33a/N33a-22.md`

- **pair** `N23a-15` / `N33a-22`
  - reported by G23: both build the frankentorch feature of frob-learn and its bridge module; N33a-22 should keep the admission record and controls, N23a-15 the learner, oracle and interchange
  - reported by G33: both build the frankentorch feature of frob-learn and its bridge module; N33a-22 should keep the admission record and controls (as G23 recorded)

## Cluster of 2 beads

- `N26a-06` (U26a, M2, task) Pilot: adversarial search and shrinking on ten planted traps in two interiors, against card randomization  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U26a/N26a-06.md`
- `SPEC-C-28` (U10c, M3, task) spec C-28: minimal counterexample worlds as 1-minimal failing order ideals: contract, law tests, registry  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U10c/SPEC-C-28.md`

- **pair** `N26a-06` / `SPEC-C-28`
  - reported by G26: SPEC-C-28 carries a pilot record for C-28 (planted traps in three scenes, run by BD-274 with BD-273) beside the chapter pilot N26a-06 (ten traps in two interiors, M2): two pilot rows for one mechanism

## Cluster of 2 beads

- `N30b-27` (U30b, M5, task) Decision OPEN-GROUND-3: calibration expiry by age and operating hours  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U30b/N30b-27.md`
- `N41-OPEN6` (U41, M0, task) registries/open.toml, part 6: Atelier, Chronicle, Ground, Foundry and Tether questions (27 rows, one decided)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U41/N41-OPEN6.md`

- **pair** `N30b-27` / `N41-OPEN6`
  - reported by G30: overlap with distinct scope: N41-OPEN6 is the register row of OPEN-GROUND-3 and the other Ground questions; N30b-25 to N30b-28 are the decisions

## Cluster of 2 beads

- `N38b-55` (U38b, M1, task) frob-conformance: reconciliation value 6: every-thirteenth-step scheduling, 2.6% in rate and 616 decisions  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U38b/N38b-55.md`
- `NEG-09` (UEG, M1, task) discrepancy E7-6 resolved: every-thirteenth-step scheduling puts 616 decision instants in 20 s, against 600  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/NEG-09.md`

- **pair** `N38b-55` / `NEG-09`
  - reported by G38: both settle every-thirteenth-step scheduling at 616 decision instants in 20 s
  - reported by GEG: both pin 616 decision instants in 20 s against 600 for every-thirteenth-step scheduling

