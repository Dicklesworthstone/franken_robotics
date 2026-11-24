---
key: "NAB-44"
title: "registries/ideas.toml: converged ideas IP-01 to IP-22 (design language, control, batches, worlds, sensing)"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "delivery", "gauntlet", "solid", "registry", "decision-record"]
parent: "EPIC-M0"
deps: ["NAB-42"]
needs: []
related: ["NAB-45", "NAB-46", "NAB-47", "NAB-48"]
gates: []
plan_refs: ["Appendix B.3", "Appendix B.2"]
---
## Background

After the first idea pass for FrankenRobotics, three further rounds were run, in which nine independent
proposal passes produced about 225 raw entries that merge into 112 distinct ideas. An idea is **converged**
when two or more of the nine passes proposed essentially the same mechanism without sight of each other. The
rule for reading convergence is strict: it is evidence about a *problem* (several planners independently felt
the same gap), not about a *mechanism* (several converged ideas were cut by every pass that proposed them, and
several were refuted by calculation).

This bead carries the first 22 converged ideas into the idea-pass register (`registries/ideas.toml`, schema
`frob.ideapass/1`, NAB-42) as records `IP-01` to `IP-22`, in five themes: design language and reuse; control
without per-design training; many bodies in one batch, and the two machines; derivatives and actuator physics;
worlds and sensing. Each row gives the idea, the number of passes that proposed it, the mechanism in one line,
where it lives (subsystem, section, construction or analysis), its tier ([S] Solid, [F] Frontier, [M]
Moonshot), its gates, kill gates and bets, and the beads that carry it. Every threshold quoted is a TARGET
owned by its gate, and the gate register governs. Tier of the bead: Solid (a decision record).

## Scope

In: records `IP-01` to `IP-22` with carriers. Out: the schema and lint (NAB-42); rows 23 to 48 (NAB-45) and 49
to 88 (NAB-46); the contested records these rows point to (NAB-47).

## Technical approach

**Transcription rule.** One record per row. `proposals` is the "Passes" count; `mechanism`, `where`,
`tier_text`, `gates`, `kill_gates`, `bets` from the columns, and `gate_text` the gates column verbatim;
`ungated` for the three Frontier rows with no kill gate, from the reasons under Acceptance criteria (IP-08,
IP-10, IP-20; IP-20 also names no gate); `carriers` from the last column (keys before a
semicolon are the carrying beads; keys after it, with a kill gate in parentheses, are the decision beads);
`disposition = "placed"`, or `"placed-narrowed"` where the row says so or is marked contested;
`contested` = the matching record of NAB-47 for the seven rows marked "Contested": IP-03 (sampling-based
control), IP-04 (family policy), IP-12 (the batch unit), IP-13 (fused step kernel), IP-16 (derivatives through
the contact solve), IP-18 (clearance complexes), IP-21 (observation-alias witness).

**Notes the one-line rows compress.**

- IP-01: the negative control of module contracts is two fingers on one bus, where a contract without time
  correlation prunes a feasible schedule (two identical local summaries; a rule that adds peaks is wrong).
- IP-08: the gate asks for 100 locked cases, with every grazing and fold control labelled or refused.
- IP-10: enumeration of contact modes beyond a small explicit graph is not built.
- IP-12: five designs of the batch unit were proposed (a table-driven interpreter, precompiled limb kernels,
  ragged operation queues, signature buckets, per-world parameter rows); the one adopted is morphology as table
  data inside padded size buckets, decided by a four-arm head-to-head.
- IP-16: derivatives through the contact solve are deferred to M6 (a non-goal before then); paired finite
  differences over short horizons are the Frontier substitute.
- IP-19: friction and compliance are never inferred from appearance.
- IP-20: simulated people are scenario dressing only; no claim rests on them, and the claim-bearing form is a
  killed idea (KI-63).
- IP-21: structural and enumerated witnesses are Solid; searched witnesses are Frontier and labelled
  experiment-specific.

**The rows.**

**Design language and reuse**

| Id | Idea | Passes | Mechanism in one line | Where it lives | Tier | Gates, kill gates, bets | Carried by |
|---|---|---|---|---|---|---|---|
| IP-01 | Module substitution contracts | 4 | A robot is a wiring of typed open modules; a compiled module carries its port equations, an assume-and-guarantee summary and a dependency digest, so replacing a limb or an actuator invalidates only what depends on it | Skeleton §14.6 (`.fmodule`); C-12; Augur §25.21.1 | [S] modules and substitution; [F] reuse of certificates and analyses | GATE-SKEL-06; KILL-SKEL-SUBST. Negative control: two fingers on one bus, where a contract without time correlation prunes a feasible schedule | BD-060, BD-077, SPEC-C-12, N14a-30 |
| IP-02 | Canonical design forms and symmetry deduplication | 5 | Put a design in canonical form modulo the group that preserves the whole closed loop, and evaluate one representative per orbit | Marrow §13.6; Phylum §17.11; C-21, C-22; Atelier §28.15 | [S] orbit representatives; [F] digest classes | GATE-PHYLUM-01; KILL-QUOTIENT; BET-31 | BD-049, BD-054, BD-285, SPEC-C-21, SPEC-C-22, N17a-28 |

**Control without per-design training**

| Id | Idea | Passes | Mechanism in one line | Where it lives | Tier | Gates, kill gates, bets | Carried by |
|---|---|---|---|---|---|---|---|
| IP-03 | Sampling-based predictive control with no training | 2 | At each control tick, sample perturbed command sequences, roll each out on the design's own model inside the batch, and apply the weighted first command | Reflex §22.8 (`control.mpc`) | [F] | GATE-REFLEX-06; KILL-MPC; BET-27. Contested, the contested-ideas record (NAB-47) | BD-252, BD-331, N22a-23, N22a-25; N22c-16 (KILL-MPC) |
| IP-04 | Family policy conditioned on morphology | 4 | One set of weights per morphology class, with tokens built from the typed design term, so a new design in the class needs little or no training | Cortex §23.11; C-23 | [F] | GATE-CORTEX-11, -12; KILL-FAMILY; BET-15. Contested, the contested-ideas record (NAB-47) | BD-320, BD-321, SPEC-C-23; N23a-26 (KILL-FAMILY) |
| IP-05 | Skill transport through a task-space interface | 2 | A skill commands a task-space wrench or acceleration; per body and contact mode an exact allocation program returns a feasible actuation or a separating infeasibility witness before any training | Reflex §22.10 (`.fskill`); Augur §25.11; A-24 | [S] exact layer; [M] learned residual | GATE-REFLEX-08, -09; KILL-SKILL, KILL-AUGUR-TRANSPORT | BD-336, BD-348 (A-24), BD-436; N22b-08, N25c-05 |
| IP-06 | Controller inheritance along typed edits | 3 | Each typed edit declares how frames, actuator roles, sensors and contact roles of the child correspond to the parent's, and the child's policy starts from the parent's through a checked mapping table | Cortex §23.7, §23.12; C-23 | [F] | GATE-CORTEX-13; KILL-INHERIT; BET-16 | BD-058, BD-322; N23a-30 (KILL-INHERIT) |
| IP-07 | Gait atlas and periodic orbits | 2 | Enumerate the admissible symmetric gait patterns from the group, seed a shooting problem for each, follow the orbit under design changes, and read stability from the full monodromy | Reflex §22.9 (`frob-gait`); C-10, C-16; A-04 | [F] | GATE-GAIT-01, -03; KILL-GAIT; BET-26 | BD-332, BD-333, BD-335, SPEC-C-10, SPEC-C-16; N22c-17 (KILL-GAIT) |
| IP-08 | Return-map sensitivities through impacts | 5 | Differentiate a settled periodic gait as a fixed point of its return map, with a saltation matrix at each isolated transverse event, instead of unrolling a rollout | C-16; Reflex §22.9 (`gait.gradient`) | [F] | GATE-GAIT-02: 100 locked cases, with every grazing and fold control labelled or refused | BD-334, SPEC-C-16, N22a-33, N22a-34 |
| IP-09 | Symmetry-aware control under broken symmetry | 2 | Use only the generators that the current payload, terrain and sensors leave intact, found by intersecting stabilizers, and report each breaker | Marrow §13.4; Reflex §22.13; Cortex §23.13; D-14 | [S] | GATE-MARROW-03; GATE-REFLEX-07; KILL-EQUIV | BD-102, BD-241, BD-327, N13-15, N13-16; N23a-33 (KILL-EQUIV) |
| IP-10 | Contact-mode graphs | 2 | A small explicit graph of drive, stand and step modes with readiness guards, searched instead of a flat action space | Reflex §22.4 (hybrid class); attainable-wrench cone A-24 | [F] | GATE-REFLEX-02. Enumeration of modes beyond a small explicit graph is not built | BD-247, BD-249, BD-348 (A-24), N14b-09, N22c-06 |
| IP-11 | Lie-bracket and geometric-phase motion primitives | 3 | Read reachable directions and displacement per shape cycle from brackets and connection curvature before simulating | C-07, C-08; A-07, A-09 (§28.4); Augur §25.19 | [S] maneuverability; [F] curvature on kinematic fixtures; [M] as an atlas | GATE-AUGUR-20; KILL-AUGUR-PHASE | BD-106 (A-07), BD-243 (A-09), BD-425, SPEC-C-07, SPEC-C-08 |

**Many bodies in one batch, and the two machines**

| Id | Idea | Passes | Mechanism in one line | Where it lives | Tier | Gates, kill gates, bets | Carried by |
|---|---|---|---|---|---|---|---|
| IP-12 | Many different robots in one launch | 6 | Morphology becomes table data read per world, so a population of different trees steps in one batch with no compile per design. Five designs of the batch unit were proposed: a table-driven interpreter, precompiled limb kernels, ragged operation queues, signature buckets and per-world parameter rows | Phylum §17.3 to §17.8; Spark §21.11; Kinesis §16.6 | [S] parameter variants; [F] model variants and design families | GATE-KIN-06; GATE-PHYLUM-02, -03, -04; GATE-SPARK-10; KILL-PHYLUM, KILL-BUCKET, KILL-FAMILYLANE; BET-18. Contested on the unit, the contested-ideas record (NAB-47) | BD-169, BD-170, BD-171, BD-183, BD-184, BD-185, BD-186, BD-356; N17b-10 (KILL-PHYLUM) |
| IP-13 | Fused per-world step kernel | 3 | One persistent kernel keeps a world's working set in thread-group memory for a whole control interval | Spark §21.18 | [M] | KILL-MEGA. Contested, the contested-ideas record (NAB-47) | BD-441, N21c-21, N21c-22 |
| IP-14 | The large-memory machine as scene bank, the two cards as trainers | 5 | Hold many resident scenes on the 256 GB machine, train on the two RTX 4090, and move only seeds, weights and receipts | §1.8; Habitat §19.15; Spark §21.17 | [S] division of labour; [F] fleet scheduler | GATE-HAB-12; GATE-SPARK-13; KILL-BANK-BREADTH, KILL-FLEET; BET-35 | BD-164, BD-236, BD-370; N19c-26 (KILL-BANK-BREADTH), N21c-11 (KILL-FLEET) |
| IP-15 | The two cards as one pipeline | 2 | Split collection and learning, or search and training, across two cards with no link | Spark §21.17; §36.6; Cortex §23.9 | [S] normalization rule; [F] sharding | GATE-SPARK-20; GATE-CORTEX-08; KILL-SHARD | BD-199, BD-216, N23a-19 |

**Derivatives and actuator physics**

| Id | Idea | Passes | Mechanism in one line | Where it lives | Tier | Gates, kill gates, bets | Carried by |
|---|---|---|---|---|---|---|---|
| IP-16 | Derivatives through the contact solve | 3 | Differentiate the converged convex solve by one back-substitution instead of through its iterations | D-8; Kinesis §16.12; Phylum §17.13 (paired differences); NONGOAL-15 | Deferred to M6; paired finite differences [F] | GATE-PHYLUM-06; KILL-FDGRAD. Contested, the contested-ideas record (NAB-47) | BD-353, BD-422; N16c-23 to N16c-28 (step adjoints), N17b-20 (KILL-FDGRAD) |
| IP-17 | Port-Hamiltonian actuator and compliance lane | 2 | Model each actuator element by its energy ports, so passivity and the energy ledger hold by construction | Sinew §15.5, §15.11; C-38 | [S] | GATE-SINEW-02, -08 | BD-068, BD-070, BD-071, BD-076, SPEC-C-38 |

**Worlds and sensing**

| Id | Idea | Passes | Mechanism in one line | Where it lives | Tier | Gates, kill gates, bets | Carried by |
|---|---|---|---|---|---|---|---|
| IP-18 | Clearance complexes and route-feasibility evidence | 4 | Cells over position and heading with one-sided clearances: a path in the surely-free complex is a witness of fit, a cut of the possibly-free complex is a witness of no fit, and the band between is `unresolved` | Habitat §19.11 (`.fclear`); C-33; A-18; enclosures of §18.3 | [F] | GATE-HAB-10; GATE-TRAC-07; KILL-FCLEAR. Contested, the contested-ideas record (NAB-47) | BD-233, BD-234, SPEC-C-33; N19b-36 (KILL-FCLEAR) |
| IP-19 | Scan or video to a simulation twin | 4 | Ingest a captured place as geometry with a scale label and unknown regions; friction and compliance are never inferred from appearance | Habitat §19.13; Ground §30.14 | [F] | GATE-HAB-03; GATE-GROUND-08; KILL-INGEST, KILL-ASBUILT | BD-395, BD-398, N19c-06; N19c-08 (KILL-INGEST) |
| IP-20 | Simulated human co-inhabitants | 2 | Moving people as scripted kinematic actors on route graphs | Habitat §19.3, §19.7; NONGOAL-16 | [F], scenario dressing only | No claim rests on them. The claim-bearing form is in the killed-idea register | BD-399, BD-421; N19a-40 (pilot) |
| IP-21 | Observation-alias impossibility witness | 2 | Two admitted worlds give the same observation transcript up to the decision and require disjoint actions, so every policy on that interface fails; the cheapest separating sensor or rate is returned | Sensorium §20.12; preflight stage 8 (§14.11); Augur §25.10, §25.18; C-19 | [S] structural and enumerated; [F] searched | GATE-SENS-09; GATE-AUGUR-10, -19; KILL-ALIAS, KILL-SKEL-ALIAS, KILL-AUGUR-ALIAS, KILL-AUGUR-LABELS. Contested, the contested-ideas record (NAB-47) | BD-134, BD-240, BD-244, BD-290, BD-363, SPEC-C-19; N20a-43 (KILL-ALIAS) |
| IP-22 | Sensor placement by task observability | 2 | Score a sensor suite by the observability Gramian of the states the task needs, modulo the gauge, at the declared rates and delays | Sensorium §20.11; C-20; A-17 | [S] | GATE-SENS-08 | BD-120, BD-239, SPEC-C-20 |

## Acceptance criteria

- [ ] Records `IP-01` … `IP-22` exist in five themes with the columns verbatim and carriers as listed.
- [ ] Every carrier key exists as a bead; every gate and kill gate named exists in the gate register (25
      distinct kill gates in these rows).
- [ ] Seven records carry a `contested` link that resolves.
- [ ] The five rows with no kill gate (IP-08, IP-10, IP-17, IP-20, IP-22) are either Solid or state why no kill
      gate applies (IP-08: its gate labels or refuses every hard case; IP-10: nothing beyond a small explicit
      graph is built; IP-20: no claim rests on it).
- [ ] The traceability lint of NAB-42 passes on these records.

## Test plan

### Unit tests

`crates/frob-conformance/tests/idea_register.rs`, module `converged_a`:

- `twenty_two_records_in_five_themes` (2, 9, 4, 2, 5).
- `proposal_counts`: IP-12 has 6; IP-02, IP-08 and IP-14 have 5; none has fewer than 2.
- `contested_links_are_seven`; `kill_gates_are_25`.
- `carriers_resolve`; `gates_resolve`.
- `tiers`: 11 rows name [S], 17 name [F], 3 name [M] (IP-05, IP-11, IP-13).
- `ungated_rows`: exactly IP-08, IP-10 and IP-20 carry `ungated`, and removing it from any of them fails
  `idea.upper-tier-without-kill`.
- `rows_frozen` against `tests/fixtures/ideas/converged_a.tsv`.

### End-to-end script and logging

`scripts/e2e/decision_records.sh` (NAB-50) asserts 22 records, zero unresolved carriers and gates, logged per
STD-E2E.

### Known answers and negative controls

Known answers: 22 rows; themes of 2, 9, 4, 2 and 5 rows; 7 contested; 25 distinct kill gates. Numbers quoted in
rows that tests pin as text: 100 locked cases (IP-08); the 256 GB machine and the two RTX 4090 (IP-14).
Negative controls: a carrier removed from IP-12; a record that cites its proposal count as support in a gate
record (`idea.cited-as-evidence`); a kill gate renamed in the gate register.

## Considerations

- A high proposal count (six for many robots in one launch) did not exempt the idea from a head-to-head with a
  control arm that may win; the kill gate makes per-family templates the engine if they do.
- Carriers name beads of other units by the keys they had when this bead was drafted; the lint is the check.

## References

Appendix B.3 (first five themes); Appendix B.2 (rules of reading).
