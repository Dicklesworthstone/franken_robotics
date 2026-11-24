# Cross-group overlap packet dup-02

36 pairs in 19 clusters. Each cluster is a set of beads connected by reported overlaps; decide a cluster as a whole.

## Cluster of 14 beads

- `N00-08` (U00, M0, task) frob-conformance: the sixteen hygiene rules, with checks for the ones no other bead enforces  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U00/N00-08.md`
- `N17b-07` (U17b, M2, task) Owner decision OPEN-PHYLUM-3: the compute envelope for the head-to-head and for pretraining populations  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U17b/N17b-07.md`
- `N21c-35` (U21c, M0, task) Owner decision OPEN-SPARK-2: the compute envelope E0 to E3; until then one device and one hour per GPU job  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U21c/N21c-35.md`
- `N23b-24` (U23b, M3, task) Owner decision OPEN-CORTEX-1: the compute envelope for family-policy pretraining; default no pretraining  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U23b/N23b-24.md`
- `N26b-21` (U26b, M3, task) OPEN-NEMESIS-3: owner decision on the compute envelope for a certified failure rate of 10⁻⁶  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U26b/N26b-21.md`
- `N32a-15` (U32a, M2, feature) frob-verbs: budgets and admission: `--estimate`, reservations, upper-end admission, attributable device time  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U32a/N32a-15.md`
- `N32a-27` (U32a, M3, feature) frob-mcp: transports and session binding: stdio with an env token, loopback HTTP with a bearer token  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U32a/N32a-27.md`
- `N34-15` (U34, M0, task) Working rules from the machine and the suite, with enforcement points, and the machine-envelope guard  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U34/N34-15.md`
- `N36-20` (U36, M0, task) frob-verbs: compute-envelope enforcement: the device-hour meter and admission refusal FROB-REF-761  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U36/N36-20.md`
- `N37-13` (U37, M3, task) frobd/frob-verbs: resource governance: sizes checked before device work, ceilings, budgets in every unit  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U37/N37-13.md`
- `N38b-03` (U38b, M0, task) frob-conformance: the close audit: typed close reasons that cite a bead's own test, shared reasons flagged  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U38b/N38b-03.md`
- `N41-08` (U41, M1, task) Owner decision OPEN-PROG-4: compute for the rank-agreement experiment, or every cheap rung stays uninformative  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U41/N41-08.md`
- `N41-17` (U41, M1, task) frob-conformance: bead lifecycle rules as checks: typed closes citing a test log, fallback, readiness, splitting  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U41/N41-17.md`
- `N44-01` (U44, M0, task) Owner decision: the envelope requests of the work packages (classes E1 to E3) and their not-approved fallbacks  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U44/N44-01.md`

- **pair** `N00-08` / `N41-17`
  - reported by G00: check H-2 of N00-08 (typed close reasons that cite a test, with its failure signals) and the lifecycle checks of N41-17 (typed close reasons citing a test log) are the same check over the tracker export, at M0 and M1
  - reported by G41: check H-2 of N00-08 is the same typed-close check over the tracker export (also reported by the U00 group)
- **pair** `N17b-07` / `N44-01`
  - reported by G17: the envelope request of the head-to-head (OPEN-PHYLUM-3) is also a row of the work-package envelope decision
  - reported by G44: row R3 (the head-to-head on 512 topologies) is the envelope request that the OPEN-PHYLUM-3 decision also files (also reported by G17)
- **pair** `N21c-35` / `N36-20`
  - reported by G36: Keep both: a decision and its enforcement. N21c-35 is the one home of the record registries/compute_envelope.toml (states, per-run E3 approvals, lint), the decision packet with per-package requests and fallbacks, and the default E0 as a fact. N36-20 is the one home of enforcement: demand vector, device-hour meter, the admission function with its timeout, the Admitted token, the out-of-process hook, KA-823. Duplicated in both: the needed-class function (two frob-core modules, compute_envelope and envelope, each with its own case table), the tests of the 761 refusal, the point-against-upper-end control, the not-run reason, and one e2e script each (open_spark_2.sh, perf_envelope_meter.sh). N36-20 has what N21c-35 lacks: consumption so far per scope inside the class computation, the reading that E2 and E3 inherit E1's per-run limits, the 24-thread and 20 GiB cases, the refusal payload with the split repair, the timeout rule, the whole KA-823 arithmetic. N21c-35 has what N36-20 lacks: the registry schema and lint, the FROB-REF-704 campaign case, the train.start path in its script, outcome handling. Suggested: one function needed_class(demand, consumed, registry) in one module, built in N36-20 (it needs the meter) on the record N21c-35 defines; N21c-35's acceptance lines 2, 3 and 5 cite N36-20's tests. N36-20 depends on N21c-35 (recorded)
- **pair** `N21c-35` / `N44-01`
  - reported by G21: two owner decisions on the compute-envelope classes E1 to E3
  - reported by G44: both put the per-package envelope requests and their not-approved fallbacks to the owner. N21c-35 ratifies the classes and records them in registries/compute_envelope.toml; N44-01 keeps the ten request rows and the rulings in registries/envelopes.toml. One list and one carrier should remain (also reported by G21)
- **pair** `N23b-24` / `N44-01`
  - reported by G44: row R7 and N23b-24's row family-pretrain/<class> record the same pretraining approval in registries/envelopes.toml with different fields
- **pair** `N26b-21` / `N44-01`
  - reported by G44: row R5 is the envelope request that the OPEN-NEMESIS-3 decision rules on
- **pair** `N32a-15` / `N36-20`
  - reported by G32: admission of a job against the compute envelope is refused in both (FROB-REF-704 here, the device-hour meter with FROB-REF-761 there)
- **pair** `N32a-15` / `N37-13`
  - reported by G32: both define the session budget vector in every unit and the reservation of trace bytes; N32a-15 owns reservation and settlement at M2, N37-13 the fail-closed defaults and ceilings at M3
  - reported by G37: both define the session budget vector in every unit and the reservation of trace bytes; N32a-15 owns reservation and settlement at M2, N37-13 the fail-closed defaults, the ceilings and the exhaustion battery at M3
- **pair** `N32a-27` / `N37-13`
  - reported by G32: both set the MCP rate limiter, the listener bound and the call ceiling, and the DESIGN defaults disagree (20 calls per second with a burst of 40 here; 600 requests per minute per session with a burst of 60 there); 64 listeners and the 40 s timeout agree
  - reported by G37: both set the MCP rate limiter, the listener bound and the call ceiling; the DESIGN defaults disagree (600 requests per minute per session with a burst of 60 in N37-13 and N37-02; 20 calls per second with a burst of 40 in N32a-27); 64 listeners and the 40 s timeout agree
- **pair** `N34-15` / `N36-20`
  - reported by G34: two enforcement points of the same machine limits: the script guard scripts/lib/envelope.sh and the verb admission with its meter; one record of limits and approvals should feed both (also reported by G36)
  - reported by G36: two enforcement points of the same machine limits (the script guard scripts/lib/envelope.sh and the verb admission with its meter); with N44-01's registries/envelopes.toml and N21c-35's registries/compute_envelope.toml there are also two approval records. One record of limits and approvals should feed both guards
- **pair** `N38b-03` / `N41-17`
  - reported by G41: rules 2 and 3 of N41-17 (typed close reasons citing the bead's own test and log line, another session, shared reasons, bursts of more than five closes in 60 s) are the six checks of the close audit N38b-03; N41-17 now depends on it
- **pair** `N41-08` / `N44-01`
  - reported by G41: the owner's ruling on compute for the rank-agreement experiment (OPEN-PROG-4) is also the WP-60 row of the envelope-request register in N44-01
  - reported by G44: row R6 is the compute request for the rank-agreement experiment that the OPEN-PROG-4 decision rules on (also reported by G41)
- **pair** `N41-17` / `N44-01`
  - reported by G41: the readiness rule (an over-envelope bead is not ready until its approval is recorded; a Frontier or Moonshot bead not until its pilot has a result) is stated as a rule in both

## Cluster of 3 beads

- `ART-L5` (UJL, M4, task) golden fixture L.5: the `.fenergy` budget for the stop at the pass counter, integer arithmetic, CLI script  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UJL/ART-L5.md`
- `N22b-12` (U22b, M4, task) frob-shield: the four premises of an energy budget and the port-graph walk for unlisted sources (FROB-REF-451)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U22b/N22b-12.md`
- `NEG-13` (UEG, M4, task) inconsistency E7-C resolved: energy-budget labels: 0.25 J external work, 0.10 J stored, reserve a further 0.10 J  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/NEG-13.md`

- **pair** `ART-L5` / `NEG-13`
  - reported by GEG: the worked energy budget as a fixture: mine with governing labels, other as golden fixture L.5
  - reported by GJL: the worked energy budget as a fixture: golden fixture L.5 here, a card with governing labels there (also reported by GEG)
- **pair** `N22b-12` / `NEG-13`
  - reported by GEG: both state that every energy port must be covered by a budget term; N22b-12 has the four premises and the port-graph walk (FROB-REF-451), mine the six term kinds, the governing labels and the superseded-label fixture

## Cluster of 3 beads

- `BD-136` (U25b, M1, task) The mission mutation corpus (60+ cases) and the checker-independence campaign (200 mutants per proof kind)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U25b/BD-136.md`
- `N12-34` (U12, M1, task) e2e lemma_auditor_mutations.sh: frob-verify alone on every specimen and mutation (GATE-LEMMA-03)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U12/N12-34.md`
- `N38b-11` (U38b, M1, task) frob-conformance: certificate mutation corpus per proof kind, and the checker run before a mission answer  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U38b/N38b-11.md`

- **pair** `BD-136` / `N38b-11`
  - reported by G25ab: both build a certificate mutation corpus per proof kind with a generator: BD-136 for the gates of the mission slice, N38b-11 for the Gauntlet's own gate
  - reported by G38: both generate a certificate mutation corpus per proof kind; BD-136 (200 mutants per kind, mission gates) is the seed and N38b-11 extends it to 1,000 and adds the checker-before-answer test: one generator should remain, in BD-136, and N12-34 (GATE-LEMMA-03) runs the same specimens
- **pair** `N12-34` / `N38b-11`
  - reported by G12: N12-34 runs every specimen and mutant through frob-verify for GATE-LEMMA-03; N38b-11 generates a mutation corpus per proof kind (as BD-136 does for the mission slice); the corpora should be one

## Cluster of 3 beads

- `BD-358` (U25e, M4, feature) frob-mission: per-constraint region certificates on small sub-boxes with contractors (GATE-AUGUR-09)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U25e/BD-358.md`
- `N25b-09` (U25b, M4, feature) frob-mission: per-constraint paving driver with interval, affine and Taylor enclosures and contractors  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U25b/N25b-09.md`
- `N25b-10` (U25b, M4, feature) frob-mission: region answers from per-constraint sets: implicit membership, monotone bounds first, dual witnesses  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U25b/N25b-10.md`

- **pair** `BD-358` / `N25b-09`
  - reported by G25ab: BD-358's scope restates the paving driver, the combination, the graph and the gate measurement that N25b-08 to N25b-11 specify; BD-358 is the seed capstone (edges recorded) and its Scope should say so
  - reported by G25cde: seed and component: BD-358's Scope lists the per-constraint paving driver with contractors that N25b-09 specifies; BD-358 should be reduced to the capstone of GATE-AUGUR-09
- **pair** `BD-358` / `N25b-10`
  - reported by G25cde: seed and component: BD-358's Scope lists combination by cylinders, the monotone-first rule and dual witnesses that N25b-10 specifies; N25b-11 (the gate script) already depends on BD-358

## Cluster of 3 beads

- `N10-KA1` (U10a, M1, task) frob-conformance: known-answer registry of the constructions, part 1 — C-01 to C-15 (KA-037 to KA-091)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U10a/N10-KA1.md`
- `N38b-27` (U38b, M1, task) decision OPEN-GAUNT-2: set tolerances for the 133 known-answer rows printed `to set`, by rule and by chapter  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U38b/N38b-27.md`
- `N38b-29` (U38b, M1, task) frob-conformance: negative controls owed by the 72 known-answer rows of the constructions (KA-039 to KA-187)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U38b/N38b-29.md`

- **pair** `N10-KA1` / `N38b-27`
  - reported by G10: overlap: tolerances for the construction rows printed to set are set in N10-KA1 to N10-KA3 and the specification beads, and decided again by rule in N38b-27
  - reported by G38: tolerances of the 47 construction rows printed `to set`: decided by rule in N38b-27, set by owners in N10-KA1 to N10-KA3; the owners' values stand, N38b-27 keeps the rule and the meta-test
- **pair** `N10-KA1` / `N38b-29`
  - reported by G10: overlap: negative controls for the construction rows printed none stated are set in N10-KA1 to N10-KA3 and the specification beads, and proposed again (72 controls) in N38b-29
  - reported by G38: negative controls for the 72 construction rows printed `none stated` are proposed in N38b-29 and set again in N10-KA1 to N10-KA3 and the specification beads; N38b-29 now depends on the three registries and should register only what they leave open

## Cluster of 3 beads

- `N19c-39` (U19c, M2, task) frob-sense and viewer: ray hit parameters in world metres under scaled instances; the sibling ray adapter  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U19c/N19c-39.md`
- `N20b-43` (U20b, M2, task) frob-kernels: the ray kernel family K-ray: sphere tracing, exact overlays, moving primitives, hit records  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U20b/N20b-43.md`
- `N33c-46` (U33c, M2, task) admission probe (row 14): f3d-math renormalizes a ray direction; two-plane fixture, world-parameter conversion  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U33c/N33c-46.md`

- **pair** `N19c-39` / `N20b-43`
  - reported by G19: the ray kernel family states the hit-parameter rule under scaled instances that N19c-39 writes for frob-sense
  - reported by G20: both write the conversion of ray hit parameters to world metres under scaled instances in the frob-sense ray kernels
- **pair** `N19c-39` / `N33c-46`
  - reported by G19: both carry the conversion of a renormalized ray parameter to world metres for the sibling ray primitive and the viewer's pick, on the same two-plane fixture
  - reported by G33: both carry the conversion of a renormalized ray parameter to world metres and the viewer's pick on the two-plane fixture (as G19 recorded)

## Cluster of 2 beads

- `BD-031` (U33c, M0, task) INT-TOOLCHAIN-01: run the toolchain matrix, choose and record T, test the fs-qty coupling, decide frob-units  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U33c/BD-031.md`
- `N41-20` (U41, M0, task) First session, step 3: count asupersync packages and compilers, fix T, and build fastmcp_rust on T with its own suite and conformance checks  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U41/N41-20.md`

- **pair** `BD-031` / `N41-20`
  - reported by G41: both record the choice of the toolchain T: BD-031 from the full matrix, step 3 of the first session from its first cells

## Cluster of 2 beads

- `BD-143` (U39b, M1, task) frob-conformance: the incumbent design-loop phase ledger on trj, cold and warm, with the ceilings recomputed  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U39b/BD-143.md`
- `N40-13` (U40, M1, task) program gate GATE-TENFOLD-02: the incumbent design loop with the common agent, closed ledgers, a share per lever  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U40/N40-13.md`

- **pair** `BD-143` / `N40-13`
  - reported by G39: both record GATE-TENFOLD-02 and the removable share per lever; N40-13 also restates the strike rule decided in N39a-39
  - reported by G40: both write the record of GATE-TENFOLD-02 and the removable share per lever (N39a-26 computes the shares); N40-13 depends on BD-143

## Cluster of 2 beads

- `BD-266` (U16b, M3, feature) verb fidelity.modelform: the model-form rank check, `model-robust` or `model-conditional` for a ranking  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U16b/BD-266.md`
- `N23b-08` (U23b, M3, feature) Rank robustness of trained finalists: model-robust or model-conditional, FROB-REF-830; GATE-CORTEX-15  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U23b/N23b-08.md`

- **pair** `BD-266` / `N23b-08`
  - reported by G16: both name GATE-CORTEX-15 and the model-robust or model-conditional verdict; N23b-08 applies the check to trained finalists and depends on BD-266
  - reported by G23: both name GATE-CORTEX-15; BD-266 is the verb and variant step plans, N23b-08 the check on trained finalists (depends on BD-266)

## Cluster of 2 beads

- `BET-11` (U00, M5, task) Decide BET-11: proof-carrying kernels are affordable (certificates cover at least 80% of derived kernels)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U00/BET-11.md`
- `N12-31` (U12, M5, task) Kill-gate decision KILL-LEMMA at M5: coverage below 50% turns certificates into randomized tests  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U12/N12-31.md`

- **pair** `BET-11` / `N12-31`
  - reported by G00: overlap with distinct scope, no merge: both record the decision KILL-LEMMA. The kill-gate bead applies the trigger and writes the kill ADR; the bet bead reads that record, evaluates the bet's falsifier clauses and writes the bet verdict. The bet bead's acceptance line "the KILL-LEMMA decision recorded" means on record by the other bead. The dependency of the bet on this bead is in place.

## Cluster of 2 beads

- `BET-37` (U00, M4, task) Decide BET-37: the building is a design variable (a priced retrofit catalogue against robot-only repair)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U00/BET-37.md`
- `N19b-42` (U19b, M4, task) Decide KILL-RETROFIT: share of building failures that kits flip on the home battery; reversals on promotion  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U19b/N19b-42.md`

- **pair** `BET-37` / `N19b-42`
  - reported by G00: overlap with distinct scope, no merge: both record the decision KILL-RETROFIT. The kill-gate bead applies the trigger and writes the kill ADR; the bet bead reads that record, evaluates the bet's falsifier clauses and writes the bet verdict. The bet bead's acceptance line "the KILL-RETROFIT decision recorded" means on record by the other bead. The dependency of the bet on this bead is in place.
  - reported by G19: both record the outcome of KILL-RETROFIT; N19b-42 applies the trigger and writes the decision, BET-37 reads it for the bet

## Cluster of 2 beads

- `N00-33` (U00, M0, task) frob-conformance: negative controls and tolerances for the known answers of the thesis and North-Star fixtures  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U00/N00-33.md`
- `N38b-28` (U38b, M0, task) decision OPEN-GAUNT-3: negative controls for the 325 rows that state none: the rule, boundary rows, first 15 rows  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U38b/N38b-28.md`

- **pair** `N00-33` / `N38b-28`
  - reported by G00: both register negative controls for KA-009 to KA-013, KA-015, KA-016, KA-019 to KA-021 and KA-023 to KA-026. N38b-28's own need says the thesis unit's bead takes precedence where they differ; N38b-28 should keep the rule, the boundary rows and KA-033
  - reported by G38: both register negative controls for KA-009 to KA-013, KA-015, KA-016, KA-019 to KA-021 and KA-023 to KA-026; N38b-28 defers to the thesis unit where they differ and should keep only the rule, the three boundary rows and KA-033

## Cluster of 2 beads

- `N10b-18` (U10b, M3, task) pilot PILOT-C-27: splitting on the random-walk fixture and one planted trap of probability ≤ 3·10⁻⁵  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U10b/N10b-18.md`
- `N26a-28` (U26a, M2, task) Pilot: rare-failure exploration on one quadruped: crude, splitting, subset simulation, defensive mixture  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U26a/N26a-28.md`

- **pair** `N10b-18` / `N26a-28`
  - reported by G10: overlap: rare-failure exploration by splitting against crude sampling (random-walk fixture and one trap here; one quadruped, four methods there)
  - reported by G26: two pilots of splitting share the random-walk fixture: PILOT-C-27 (M3, falsifier 3 times and 1% of forks) and the chapter's exploration pilot (M2, falsifier of KILL-RARE); N26a-28 records both falsifiers and lets the gate register govern

## Cluster of 2 beads

- `N15b-02` (U15b, M1, feature) verbs module.list, module.show, module.check: registry entries, schemas, refusals and CLI surface  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U15b/N15b-02.md`
- `VERBS-module` (UJL, M1, task) frob-verbs registry: cluster `module`, five rows, with parity, e2e and schema-snapshot checks  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UJL/VERBS-module.md`

- **pair** `N15b-02` / `VERBS-module`
  - reported by G15: both write the registry entries of module.list, module.show and module.check
  - reported by GJL: both write the registry entries of module.list, module.show and module.check (also reported by G15)

## Cluster of 2 beads

- `N18b-16` (U18b, M4, task) Pilot [M]: persistent ruts and footprints: the leak fixture on batch-cpu  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U18b/N18b-16.md`
- `N38a-28` (U38a, M6, task) pilot: the rut leak fixture on batch-cpu, deciding whether persistent ruts are funded  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U38a/N38a-28.md`

- **pair** `N18b-16` / `N38a-28`
  - reported by G18: same pilot: the rut leak fixture on batch-cpu (mine at M4, the other at M6 under BD-417)
  - reported by G38: same pilot (the rut leak fixture on batch-cpu); N18b-16 is M4 with the soil work, N38a-28 is M6 under the seed epic BD-417. One pilot should remain; N38a-29 depends on whichever is kept

## Cluster of 2 beads

- `N20b-04` (U20b, M3, task) frob-sense::calib: the extrinsic pose graph and its weighted Gauss-Newton solve on SE(3)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U20b/N20b-04.md`
- `N38b-42` (U38b, M3, task) frob-conformance: adopted known answer at M3: one edge of the extrinsic calibration graph, with the wrong-adjoint mutant  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U38b/N38b-42.md`

- **pair** `N20b-04` / `N38b-42`
  - reported by G38: overlap with distinct scope: N20b-04 and N20b-07 test the calibration solver and cite GATE-GAUNT-11; N38b-42 supplies only the registry-bound row and its wrong-adjoint control and depends on N20b-04

## Cluster of 2 beads

- `N25b-24` (U25b, M4, feature) frob-mission: the dual query: the smallest priced world edit that restores a mission, in one repair budget  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U25b/N25b-24.md`
- `N25c-31` (U25c, M8, feature) frob-mission: a priced world-edit grammar in one budget with robot edits; lower bound, alternatives and the gap  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U25c/N25c-31.md`

- **pair** `N25b-24` / `N25c-31`
  - reported by G25cde: overlap: both specify the smallest priced world edit that restores a mission in one repair budget; N25b-24 at M4 for one robot, N25c-31 at M8 with robot edits, the site and the comparator

## Cluster of 2 beads

- `N29a-04` (U29a, M1, task) frob-chronicle: queries pinned to the committed frontier: the pin, the total order, the cursor, FROB-REF-896  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U29a/N29a-04.md`
- `N32a-10` (U32a, M1, task) frob-verbs: pagination pinned to a committed frontier: authenticated cursor `frob.cursor/1`, `follow`  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U32a/N32a-10.md`

- **pair** `N29a-04` / `N32a-10`
  - reported by G29: frontier-pinned keyset pagination and a cursor at M1 in both (store level in frob-chronicle, authenticated frob.cursor/1 in frob-verbs); 896 against 806 follows the same split
  - reported by G32: frontier-pinned pagination with a cursor at M1 in both: the store-level pin, total order and cursor in frob-chronicle, the authenticated frob.cursor/1 that wraps it in frob-verbs; FROB-REF-896 against -806 follows the same split

## Cluster of 2 beads

- `N38b-18` (U38b, M0, task) registries/gates.toml: the base gate record of status accounting, its claim kinds and its six statuses  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U38b/N38b-18.md`
- `N40-01` (U40, M0, task) registries/gates.toml: the gate record, its seven statuses and the nine rules every gate obeys  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U40/N40-01.md`

- **pair** `N38b-18` / `N40-01`
  - reported by G38: both define the record of registries/gates.toml: N38b-18 the base record with six statuses, N40-01 the register layer with seven and nine rules. Split kept as coverage (e) 16 of U38b states it; N40-01 depends on N38b-18 (edge recorded) and one schema file must result
  - reported by G40: both define the row of registries/gates.toml: N38b-18 the base record with six statuses, N40-01 the same record with parts per milestone, gate kinds, a seventh status, kill fields and the validator. N40-01 is made to depend on N38b-18; one schema frob.gates/1 must come out of the two

