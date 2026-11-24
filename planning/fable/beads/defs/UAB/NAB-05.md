---
key: "NAB-05"
title: "registries/glossary.toml: term records G to M (Gait orbit to Morphology class), 61 rows, one corrected"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "workspace", "delivery", "solid", "registry", "review-notes-open"]
parent: "WP-01"
deps: ["N00-10", "NAB-01"]
needs: []
related: ["NAB-06", "NAB-07"]
gates: []
plan_refs: ["Appendix A", "§0.8"]
---
## Background

The glossary of FrankenRobotics is a registry, `registries/glossary.toml`, from which `docs/GLOSSARY.md` is
generated (NAB-01) and against which every `CONTRACT.md` is linted (NAB-02). It is filled in five beads by
letter range. This bead carries every term of art whose sort key begins with G, H, I, J, K, L or M: 61 rows.
Backticks are ignored by the sort key, so `lockstep` and `model-conditional` belong here.

A row gives the term, what it means in this project, and in parentheses the chapter subsection, construction
(`C-nn`), doctrine rule (`D-n`) or other identifier that owns the definition. The owning text governs where a
row abbreviates. Numbers quoted in a row are DESIGN values of the owning section unless the row says otherwise.
Tier: Solid.

## Scope

In: the 61 `[[term]]` records of this range, including one correction made on transcription. Out: the schema,
loader and generator (NAB-01); the lint (NAB-02); the other ranges.

## Technical approach

**Transcription rule.** One `[[term]]` record per row, in table order. `term` is the first column without
backticks (`code = true` if it was printed in backticks); a trailing parenthetical in the term column is the
`sense`; `meaning` is the second column up to its final parenthetical, which is split into `defined_in` (items
beginning `§` or `Appendix`) and `constructs` (items such as `C-16`, `D-6`); `kind` is `subsystem`, `label`,
`binary`, `class`, `record`, `quantity` or `concept` by the rule of NAB-03; `aliases` hold plurals and further
names; `design_values = true` when the meaning contains a number; `id` is the lower-cased term with hyphens.

**Special cases in this range.**

- `Instants (seven)`: the parenthetical is a count, not a sense. The record is term `Instants`, empty sense,
  aliases `seven instants`, `instant`. The seven, in their fixed boundary order: sample, exposure, timestamp,
  arrival, decision, command, actuation.
- Paired rows defined together (one record each, aliases for the second name): `Inner and outer complex`
  (aliases `inner complex`, `outer complex`), `Inner and outer design sets`, `Inner and outer enclosure`.
- Code literals (kind `label`): `lockstep` (also a step profile), `model-conditional`.
- Subsystem rows: `Gauntlet` (kind `subsystem` although its meaning begins "The verification program"), `Ground`,
  `Habitat`, `Kinesis`, `Lemma`, `Marrow`.
- Classes and enumerations to keep verbatim: the Gauntlet tiers G0 laws, G1 known answers, G2 parity, G3
  metamorphic, G4 chaos, G5 determinism; the four kernel classes `float`, `exact`, `guarded`, `stat`; the six
  rungs of the ladder R0 static, R1 shared policy, R2 fine-tuned, R3 specialist, R4 referee, R5 rare failure;
  the layer rules L-1 to L-12; the twelve Hervé classes (conjugacy classes of connected subgroups of SE(3)).
- Formulas and numbers: the mass-support bound ‖c − o‖² + tr(I_C)/(2m) ≤ ρ² for a body whose mass lies in a ball
  of centre o and radius ρ; the Moonshot cap of four active items; the kernel IR schema name `frob.kir/1`; a
  minimal counterexample world is 1-minimal in its typed edits and is stored as `.fmcw`; a mission query is
  stored as `.fmission`; the identity examples `DesignId`, `ModelId`, `SceneId`, `KernelId`, `PolicyId`,
  `EpisodeId`.

**Correction applied on transcription (one row).** The plan-time row for `Hypothesis verdict` reads "the
recorded state of one of the planning hypotheses H-01 to H-24: supported, narrowed or refuted". The register it
describes has four states, **ACTIVE** (unrefuted and untested), **NARROWED** (the strong form failed; a stated
weaker claim survives), **REFUTED** (a calculation or an executed check shows the mechanism does not deliver
what was claimed) and **DEFERRED** (real, but not an answer to the question asked of it), and its rows run to
H-29 with split rows: 32 identifiers, H-01 to H-29 with H-22 replaced by H-22a and H-22b, plus H-04b and H-12b
(N00-10's count). The record is therefore written:

> Hypothesis verdict | The recorded state of one planning hypothesis (H-01 to H-29, with the split rows H-04b,
> H-12b, H-22a and H-22b): ACTIVE, NARROWED, REFUTED or DEFERRED. The state binds what may be said and built;
> it changes only on new evidence of the kind that produced it (§0.8)

The table below prints the plan-time wording so that the change is visible; the registry holds the corrected
wording. FACT: the register's four states and its rows through H-29 are printed in the document-control
chapter; the hypothesis register bead (N00-10) carries them.

**The rows.**

| Term | Meaning, with the section or construction that defines it |
|---|---|
| Gait orbit | A periodic gait computed as a fixed point of a return map, with its multipliers (C-16, §22.9) |
| Gauge | Directions that are unobservable by construction, such as global position and yaw under proprioception and gravity (C-20) |
| Gauntlet | The verification program, with tiers G0 laws, G1 known answers, G2 parity, G3 metamorphic, G4 chaos, G5 determinism (§38) |
| Generator | A keyed sampler of designs within a morphology class, identified by a `GeneratorId` (§17.10) |
| Ground | The subsystem that owns calibration against reality (§30) |
| Growth vector | The dimensions of the flag of a distribution under brackets; it decides controllability and the cost of small motions (C-07) |
| Guarded single precision | An `f32` predicate with a proved error bound that falls back to exact arithmetic inside the bound (§13.7, §21.8) |
| Habitat | The subsystem that owns scenes, living worlds, clearance and retrofits (§19) |
| Hard-contact law | CL-H: Signorini complementarity with Coulomb friction on the cone; the referee's specification and an optional batch law (§18.5) |
| Hard-world set | The versioned set of minimal counterexample worlds per task family and robot class; discovery data, never a confirmation sample (§26.8) |
| Hardware fingerprint | The digest of host, device, driver, runtime, compiler and probe results to which caches and receipts are bound (§21.16) |
| Hazard list | The registry, per interior archetype, of hazards with dimensions and provenance (§19.8) |
| Hervé class | One of the twelve conjugacy classes of connected subgroups of SE(3) (C-01) |
| Hypothesis verdict | The recorded state of one of the planning hypotheses H-01 to H-24: supported, narrowed or refuted (§0.8) |
| Idempotency key | The caller's key that binds a mutating request to exactly one effect (§32.7) |
| Identity | A content hash such as `DesignId`, `ModelId`, `SceneId`, `KernelId`, `PolicyId`, `EpisodeId`; it names bytes and carries no verdict (§11.1, D-11) |
| Impossibility cut | Disconnection of start and goal in the outer complex at a margin, with the separating cells named (§19.11) |
| Independently accepted | Accepted by a checker that is not the proposing system: a battery in a neutral evaluator, `frob-verify`, a neutral adjudicator (§2.1) |
| Indistinguishability witness | Two admitted worlds and a deadline such that every admitted command prefix yields identical packet histories while the successful continuations are disjoint (C-19) |
| Inheritance | Initializing the policy of an edited design from its parent by the transport of the edit (C-23, §23.12) |
| Inner and outer complex | 𝒞_in contains a cell only if every pose in it is free; 𝒞_out omits a cell only if every pose in it collides (C-33) |
| Inner and outer design sets | I and O with I ⊆ F ⊆ O: every cell of I has a witness, every cell outside O a refutation (C-17) |
| Inner and outer enclosure | Sets contained in, and containing, the occupied set of a body under the stated field and registration errors (§18.3, §19.5) |
| Instants (seven) | Sample, exposure, timestamp, arrival, decision, command, actuation, with a fixed boundary ordering (§11.8) |
| Intervention | A typed change do(X := x) to one declared mechanism, recorded as a model difference (C-29, §27.5) |
| Job handle | What a long verb returns when it does not finish inside `sync_wait_ms` (§32.10) |
| Joint-level rows | Stiction, backlash and hard stops written as rows of the same solve as contact (C-37, §18.7) |
| Kernel | A manifest, its arithmetic and its sources in Rust and the shared dialect (§21.5) |
| Kernel class | `float`, `exact`, `guarded` or `stat` (§21.8) |
| Kernel IR | The typed intermediate form `frob.kir/1` from which kernel sources are emitted and on which dimension checks and rewrite classes are decided (§21.7) |
| Kernel reflection | The route by which an instance certificate is promoted by evaluating a reflected checker inside a proof kernel (§12.4) |
| Kill gate | A `KILL-*` row: a trigger written in advance and the fallback that replaces the mechanism (§40) |
| Kinesis | The subsystem that owns dynamics on three tiers (§16) |
| Known answer | An analytic or enumerated truth a mechanism must reproduce; numbered `KA-nn` (§38, Appendix M) |
| Label count | The least number of calibration labels with which one controller serves all admitted variants by a deadline (C-19) |
| Ladder | The six evaluation rungs R0 static, R1 shared policy, R2 fine-tuned, R3 specialist, R4 referee, R5 rare failure (§28.9) |
| Layer rule | One of L-1 to L-12, the dependency rules between crate layers (§34.3) |
| Lease | An epoch-fenced right of the host to read or write a device slab (§21.10) |
| Legality set | The typed world edits, with bounds from cited norms, that an adversarial search may apply (§26.3) |
| Lemma | The subsystem that owns proofs, certificates and the offline auditor (§12) |
| Lifecycle compression | Advancing slow wear and ageing states over many hours of duty without simulating every step [M] (§31.14) |
| Lineage | The graph of designs linked by typed edits, with diff, blame, bisect and merge (§28.7) |
| Link token | The per-link input of a family policy: the link with its parent joint, in dimensionless features (C-23) |
| Living world | A scene whose state the robot changes: films, proxies, slosh and cords, admitted in three tiers (§19.10) |
| Load case | One instant of one episode from which structural loads are taken; peaks from different instants are never assembled (§31.6) |
| `lockstep` | The step profile of the throughput tiers: fixed step, fixed iteration count, convex compliant solve (§16.3, §16.6) |
| Marrow | The subsystem that owns the algebraic core: Lie types, groups, exact predicates (§13) |
| Mass-support bound | ‖c − o‖² + tr(I_C)/(2m) ≤ ρ² for a body whose mass lies in a ball of centre o and radius ρ (§11.5) |
| Mechanical group | Γ_mech, the automorphisms of the labelled kinematic tree realized by an isometry that preserves every mechanical parameter (C-09, §13.4) |
| Memo graph | The store of results under complete dependency keys, checked by shadow recomputation (§28.14) |
| Metamorphic test | A relation that must hold between two runs without knowing either answer, such as equivariance under a symmetry or exact similarity scaling (§38) |
| Minimal counterexample world | A failing world reduced to a 1-minimal set of typed edits on a passing world, stored as `.fmcw` with a solvability witness (C-28, §26.7) |
| Mission query | A design domain, a world set, an observation channel, a causal policy class and an acceptance predicate with their quantifiers, stored as `.fmission` (C-17, §25) |
| Mission-observable projection | A digest of the compiled model restricted to what the mission's equations read; equal digests are evaluated once (C-22, §17.11) |
| Mixed-topology batch | One batch whose worlds run robots with different kinematic trees (§17) |
| Model variant | A per-world model with its own tree, joint classes and actuator count inside the padded capacities of a shape class (§17.3) |
| `model-conditional` | The label of a ranking that reverses between contact laws or other model forms (§16.9) |
| Model-form rank check | The comparison of design rankings under two contact laws or stiffness variants (§16.9) |
| Module | A reusable sub-assembly `.fmodule` with a port contract and an invalidation contract (§14.6) |
| Moonshot cap | At most four [M] items are active at once; the rest wait in the queue of Appendix C (D-6) |
| Morphology class | A set of designs with one kind of body plan, for example quadrupeds, within which a family policy is trained and rank agreement is measured (§23.11, §28.10) |

Rows in this range: 61.

## Acceptance criteria

- [ ] The registry contains 61 records whose sort key begins with G to M, in the order above, and loads.
- [ ] The `Hypothesis verdict` record holds the corrected wording (four states, rows through H-29), and a test
      compares its state list with the states the hypothesis register's schema admits.
- [ ] `Instants` has no sense and lists seven names in the boundary order.
- [ ] `lookup("inner complex")`, `lookup("outer complex")` and `lookup("seven instants")` each resolve.
- [ ] The generated document prints the other 60 meanings character for character as in the table.

## Test plan

### Unit tests

`crates/frob-conformance/tests/glossary_content.rs`, module `g_to_m`:

- `count_is_61`; `order_matches_frozen_list`.
- `hypothesis_verdict_is_corrected`: the meaning contains "ACTIVE, NARROWED, REFUTED or DEFERRED" and "H-29",
  and does not contain "supported, narrowed or refuted".
- `hypothesis_states_equal_register_states`: the four state names equal the enumeration of schema
  `frob.hypothesis/1`, loaded from `crates/frob-conformance/registry/hypotheses.toml` (N00-10, a dependency of
  this bead, so the register exists when this test is written), and the 32 identifiers named by the corrected
  row (H-01 to H-29 with H-22 replaced by H-22a and H-22b, plus H-04b and H-12b) equal the register's identifiers;
  a register with 33 rows (a bare H-22 kept beside its split rows) fails it.
- `seven_instants_in_order`: sample, exposure, timestamp, arrival, decision, command, actuation.
- `no_sense_is_a_number_word`: no record of the registry has a `sense` that is a numeral or a number word
  (`Instants` has an empty sense); the negative control below plants `sense = "seven"`.
- `pinned_values`: `Mass-support bound` contains "‖c − o‖² + tr(I_C)/(2m) ≤ ρ²"; `Moonshot cap` contains "four";
  `Ladder` contains "R0 static" and "R5 rare failure"; `Gauntlet` contains "G0 laws" and "G5 determinism";
  `Kernel class` contains the four names; `Layer rule` contains "L-1 to L-12"; `Hervé class` contains "twelve".
- `meanings_match_frozen_table` against `tests/fixtures/glossary/g_to_m.tsv` (60 rows; the corrected row is
  checked by its own test).

### End-to-end script and logging

`scripts/e2e/glossary.sh` (NAB-08) asserts the count of this range (61) and logs it as a `metric` per STD-E2E.

### Known answers and negative controls

Known answers: 61 rows (7 G, 7 H, 11 I, 2 J, 7 K, 12 L, 15 M). Negative controls: restoring the plan-time
wording of `Hypothesis verdict` must fail `hypothesis_verdict_is_corrected`; reordering two instants must fail
`seven_instants_in_order`; giving `Instants` the sense `seven` must fail a test that no sense is a number word.

## Considerations

- The correction is a finding of the conversion, recorded in this unit's coverage file. If a later revision of
  the hypothesis register adds a state or a row, the glossary row follows in the same commit.
- `lockstep` is both a label and a step profile; the lint treats a use inside a code span as the label and a
  use in prose ("the lockstep solve") as the same record.

## References

Appendix A, rows G to M; §0.8 (states and rows of the hypothesis register).
