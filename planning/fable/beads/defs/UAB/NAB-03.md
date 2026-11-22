---
key: "NAB-03"
title: "registries/glossary.toml: term records A to C (Acceptance predicate to Crossover reuse count), 73 rows"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "workspace", "delivery", "solid", "registry"]
parent: "WP-01"
deps: ["NAB-01"]
needs: []
related: ["NAB-02", "NAB-04", "NAB-05", "NAB-06", "NAB-07", "NAB-08"]
gates: []
plan_refs: ["Appendix A"]
---
## Background

The glossary of FrankenRobotics (a robotics simulation and design system built for agents) is a registry,
`registries/glossary.toml`, from which `docs/GLOSSARY.md` is generated (NAB-01) and against which every
`CONTRACT.md` is linted (NAB-02). The registry is filled in five beads by letter range so that each is a
reviewable piece of work. This bead carries the first range: every term of art whose sort key begins with A, B
or C, 73 rows. The table below is the content; nothing else needs to be consulted to write the records.

A row gives the term, what it means in this project, and in parentheses the chapter subsection, construction
(`C-nn`), doctrine rule (`D-n`) or other identifier that owns the definition. The owning text governs where a
row abbreviates. Numbers quoted in a row are DESIGN values of the owning section unless the row says otherwise.
Tier: Solid (data entry with checks).

## Scope

In: the 73 `[[term]]` records of this range, with senses, aliases, kinds and provenance. Out: the schema, loader
and generator (NAB-01); the lint (NAB-02); the other ranges (NAB-04 to NAB-07).

## Technical approach

**Transcription rule (the same for all five ranges).** One `[[term]]` record per row, in the order of the table:

1. `term` is the row's first column without backticks; `code = true` if the row prints it in backticks.
2. A trailing parenthetical in the term column is the `sense` (`Archetype (design)` → term `Archetype`, sense
   `design`). An unqualified row that shares its term with a qualified row takes the sense named below.
3. `meaning` is the second column up to its final parenthetical. The final parenthetical is split into
   `defined_in` (items beginning with `§` or `Appendix`) and `constructs` (items such as `C-17`, `D-10`,
   `BET-13`, `GATE-TENFOLD-06`).
4. `kind`: `subsystem` when the meaning begins "The subsystem that owns" (and for `Airlock`); `label` for a code
   literal that appears in tool output; `binary` for the row of executables; `class` for an enumerated set of
   named classes (determinism classes, evidence classes, kernel classes, tiers); `record` for a file or stored
   object (`.fbuild`, `.frobcert`, a ledger); `quantity` for a defined number or formula; otherwise `concept`.
5. `aliases` hold the plural, and every further name a multi-name row defines.
6. `design_values = true` when the meaning contains a number.
7. `id` is the lower-cased term with spaces as hyphens, followed by `-<sense>` when a sense exists.

**Special cases in this range.**

- Senses: `Archetype` has two records (`design`: the eighteen design templates; `interior`: the seven interior
  families). `Arena` has two (`subsystem` for the unqualified row; `device` for the preallocated slabs).
  `Campaign` has two (`design`, `qualification`). `Cell` has three (`clearance`, `mission`, `reality`).
  `Battery (evaluation)` keeps its sense because the electrical battery of an actuator bus is a different thing
  and is deliberately not a glossary term (it is a card kind).
- Multi-name rows: `Adjoint, coadjoint` (aliases `coadjoint`, `Ad`, `ad*`); `Cold run, warm campaign` (alias
  `warm campaign`).
- Structural row (no owning section; `structural = true`): `ADR`.
- Subsystem rows: `Airlock`, `Arena`, `Atelier`, `Augur`, `Autopsy`, `Chronicle`, `Cortex`.
- `Alias witness` is a cross-reference row: its record carries `see = "indistinguishability-witness"` and its own
  short meaning (the observation-alias form names two distant states that deliver the same readings).
- Formulas and numbers to transcribe exactly: the coadjoint convention ad*_ξ = ad_ξᵀ with no minus sign, so that
  μ̇ = ad*_ξ μ + w; anchored position: cell side 8 m, offset centred in [−4, 4) m; brownout: bus power demand
  above E²/(4·R); Cayley map cay(x) = (I − x/2)⁻¹(I + x/2); compute envelope E0: 16 GiB, 8 cores, one hour, one
  device; conformal margin q̂: the ⌈(n + 1)·(1 − α)⌉-th smallest audit residual; contact laws CL-1, CL-2, CL-M,
  CL-H; the cascade's six stages; constructions C-01 to C-40; the seven claim kinds; the eight `.fcard` kinds.

**The rows.**

| Term | Meaning, with the section or construction that defines it |
|---|---|
| Acceptance predicate | The Boolean predicate A on executions that a mission query must satisfy; written in the requirement language (C-17, C-30, §24.5) |
| Adjoint, coadjoint | Ad_g moves twists and Ad_g⁻ᵀ moves wrenches; the algebra coadjoint is ad*_ξ = ad_ξᵀ with no minus sign, so that μ̇ = ad*_ξ μ + w (§11.2, §13.3) |
| Admission gate | An `INT-*` test that a sibling foundation must pass at its pinned commit before any crate may depend on it (§33.6) |
| ADR | An architecture decision record `ADR-<SUBSYS>-nn`: a decision with its reason, stated in the chapter that owns it |
| Airlock | `frob-fsim`, the only crate that names FrankenSim's volatile types (§33.3) |
| Alias witness | See indistinguishability witness; the observation-alias form of §20.12 names two distant states that deliver the same readings |
| Amdahl ledger | The table of phase shares of an incumbent design loop from which every ceiling of the thesis is computed (§2.2) |
| Anchored position | An integer cell index plus an `f32` offset in a cell of side 8 m, the offset centred in [−4, 4) m so that a cell shift is exact (§11.1, §35.8) |
| Annex | A separate workspace or project outside the core's lockfile and rules, kept apart by the unsafe and foreign-interface policy and, where it is Rust, built on the same toolchain T: `spark/`, `derive/`, `bindings/`, `viewer/`, `lean/` (§34.1). `surface/frob-mcp` is a member of the core workspace, not an annex |
| Answer | What a mission asks for: a design and controller that pass a frozen battery, a family answer, a correct withdrawal or confirmation after an amendment, or a truthful `unknown` with its witness (§2.1) |
| Anytime-valid study | A study whose inference is an e-process or a confidence sequence, valid at every stopping time (§24.12) |
| Archetype (design) | One of the eighteen design templates that `design.templates` lists (§14.10) |
| Archetype (interior) | One of the seven interior families with a generator, a structural proxy and a hazard list (§19.7) |
| Arena | The subsystem that owns tasks, requirements, fixtures, metrics and the qualification lane (§24) |
| Arena (device) | The preallocated structure-of-arrays slabs that hold every per-world quantity on a device; no allocation after environment creation (§21.10) |
| Artifact graph | The record that links every claim to the evidence, runs and decision rules that produced it (§29.13) |
| Artifact guard | The three checks a failure or a witness must survive before Nemesis counts it: replay one tier up, replay under numerical variants, and the energy audit (§26.6) |
| Assumption lattice | Assumption sets ordered by strength, on which every certificate records a polarity (C-18, §28.13) |
| Atelier | The subsystem that owns the studio loop: analyses, comparison, lineage, optimization, the ladder, the cascade and the decision cores (§28) |
| Audit lane | The share of a rung's spend given to designs drawn by a keyed stream before any score exists and evaluated at the higher rung at fixed n; the only source of calibration (§28.9) |
| Augur | The subsystem that owns mission queries and inverse design (§25) |
| Authority model | The declared law by which forces or actuators can be commanded, for example independent normal forces or equal passive springs; a field of every stance result (C-32, §25) |
| Autopsy | The subsystem that owns counterfactual debugging by forks and interventions (§27) |
| Background referee | The CPU referee re-running a keyed sample of fast-tier world-seconds and one-step defects, with a reported coverage figure (§16.11) |
| Base interval | A piece [t0, t1) of rational time cut by the physics grid and the clock boundaries; the unit a transaction commits (§16.5) |
| Baseline controller | The estimator and controller synthesized from the model for every design, with no training (§22.3, §22.4) |
| Batch-composition invariance | The property that a world's trajectory does not depend on batch size, slot, co-tenant worlds, bucket capacity or shard count (§35.5) |
| Battery (evaluation) | A frozen, versioned set of scenarios against which a design is accepted (§24.9) |
| Bead | A unit of delivery work with a `BD-nnn` identifier and a structured log (§45, §29.14) |
| Bet | A hypothesis the plan depends on, stated with its falsifier and a potency control (§4) |
| Brick pool | The shared pool of distance-field bricks from which the scene variants resident on a device are assembled (§19.15) |
| Brownout | A bus power demand above E²/(4·R), for which no operating point exists; recorded as an event (§15.10) |
| Budget stop | A stop on budget that publishes nothing partial as a result, except that a mission query returns its certified cells and the unresolved remainder (§25, §32.9) |
| Build packet | The bounded manufacturing export `.fbuild`: geometry, tolerances, datums and fiducials, load envelope and evidence, with `fabrication_released` always false (§31.12) |
| Campaign (design) | A declared sequence of evaluations toward one design decision, with a budget and a report (§28) |
| Campaign (qualification) | A set of confirmation requests that share a claim family, a sealed world family and a total error budget α (§24.13) |
| Campaign quotienting | Evaluating one canonical representative per Γ_obs orbit and per mission-observable class (C-22, §17.11) |
| Canonical form | A design written as its dimensionless groups after division by its own scales m₀, ℓ₀ and t₀ (C-21) |
| Capability scope | A named permission attached to a session, such as `qual:admin` or `qual:run` (§32.13, §37.5) |
| Capacity bias | The bias introduced when contact-capacity overflow removes worlds from one arm of a comparison; such a verdict is labelled `capacity-biased` (§18.11) |
| Card | Immutable, content-addressed data about a real or generic part, with provenance per field: `.fcard` kinds actuator, battery, transmission, interface, material, optical, sensor, payload (§15.3) |
| Cascade | The six-stage co-design sequence: deduplicate and check, reuse and synthesize, screen, race, rank robustness, qualify (§28.11) |
| Causal policy | A map from the history of arrived packets to commands; the policy class Π_Y of a mission query (C-19, §20.15) |
| Cayley map | The rational retraction cay(x) = (I − x/2)⁻¹(I + x/2) (C-05) |
| Ceiling | The most a mechanism could gain, stated before any speed test (D-10) |
| Cell (clearance) | A cube of the (x, y, ψ) grid of a clearance complex, with two one-sided clearances (§19.11) |
| Cell (mission) | A box of a design family's parameter space that carries one verdict and one certificate (§25.3) |
| Cell (reality) | The scope of a Reality result: physical units, controller and firmware, surface, task family and envelope of conditions (§30.5) |
| Certificate bundle | The portable `.frobcert` file that carries statements, proof objects and receipts for `frob-verify` (§12.4) |
| Certified | The evidence class of an instance-level statement accepted by a checker with an offline-verifiable certificate (§12.2) |
| Chronicle | The subsystem that owns telemetry, replay, forks, ledgers and the artifact graph (§29) |
| Chunk | The unit of published telemetry, written once and addressed by key (§29.4) |
| Claim | One of seven kinds that are never conflated: Proof, Known answer, Parity, Fidelity, Design comparison, Reality, Performance (§8.1) |
| Claim product | The type of a result that rests on two claims; its class is the weakest input's and nothing converts it into a proof (§8.2, §12.2) |
| Clearance complex | The pair of cubical complexes 𝒞_in ⊆ 𝒞_free ⊆ 𝒞_out of a planar base in one fixture mode, stored as `.fclear` (C-33, §19.11) |
| Clock contract | A task's declaration of rational periods, phases, the seven instants and the rule for intra-step events (§20.4) |
| Cohort | A set of model slots and worlds running under one step plan (§17.4) |
| Cold run, warm campaign | A cold run includes authoring the environment and the design domain; a warm campaign runs a declared sequence of amendments with reusable inputs (§2.1) |
| Commit fence | The rule that readers see only state at a committed `commit_epoch`; a candidate interval becomes visible in one step (§16.5) |
| Committed frontier | The contiguous prefix of published chunks to which a paginated query is pinned (§29.4, §32.8) |
| Compute envelope | An owner-approved resource class E0 to E3 for a run; E0 is 16 GiB, 8 cores, one hour, one device (§36.14) |
| Confirmation world | A world from a sealed cluster drawn fresh for one qualification request and unreadable by discovery until the campaign closes (§24.13) |
| Conformal margin | q̂, the ⌈(n + 1)·(1 − α)⌉-th smallest audit residual between two rungs or tiers (C-26, §28.9) |
| Construction | A mathematical object of the plan with its test, honest limits, tier and, for [F] and [M], pilot and ceiling: C-01 to C-40 (§10) |
| Contact capacity | The fixed maximum number of contacts per world in a shape class (§16.6, §18.11) |
| Contact law | A named law of the contact solve: CL-1 compliant default, CL-2 Hunt–Crossley, CL-M import mapping, CL-H hard contact (§18.5) |
| Controller arm | The declaration, in a comparison of designs, of whether the controller is shared, retuned with equal resources, or inherited (§24.12) |
| Convergence gate | A `GATE-*` row: one claim, one test, one numeric threshold and one milestone (§40) |
| Cortex | The subsystem that owns the gym, learning, family policies and inheritance (§23) |
| Cosserat rod | A one-dimensional elastic body with a frame per cross-section; the [M] model of cables and cords inside a coupled island (§19.10) |
| Counterexample | The smallest hand-checkable case that breaks a plausible shortcut; a permanent negative control (D-9, Appendix N) |
| Coupled island | A free-surface fluid, cloth or rod solver that runs beside one world on the CPU referee, for finalists only and behind a flag (§19.10) |
| Crossover reuse count | The number of amendments after which one-time authoring has paid for itself in a warm campaign (§2.1) |

Rows in this range: 73.

## Acceptance criteria

- [ ] `registries/glossary.toml` contains 73 records whose sort key begins with A, B or C, in the order above,
      and the loader of NAB-01 accepts the file.
- [ ] `Archetype`, `Arena`, `Campaign` and `Cell` have 2, 2, 2 and 3 records with distinct senses.
- [ ] Every record except `ADR` has a non-empty `defined_in` or `constructs`.
- [ ] `lookup("coadjoint")` and `lookup("warm campaign")` each resolve to one record.
- [ ] The generated `docs/GLOSSARY.md` prints the 73 meanings character for character as in the table (the
      test compares against a frozen copy of this table).

## Test plan

### Unit tests

`crates/frob-conformance/tests/glossary_content.rs`, module `a_to_c` (reads the real registry):

- `count_is_73`; `order_matches_frozen_list` (the 73 terms in sequence).
- `senses`: `Archetype` {design, interior}; `Arena` {subsystem, device}; `Campaign` {design, qualification};
  `Cell` {clearance, mission, reality}.
- `subsystem_rows`: the seven names above have `kind = "subsystem"`.
- `pinned_values`: the meaning of `Anchored position` contains "8 m" and "[−4, 4) m"; `Brownout` contains
  "E²/(4·R)"; `Compute envelope` contains "16 GiB, 8 cores, one hour"; `Conformal margin` contains
  "⌈(n + 1)·(1 − α)⌉"; `Cayley map` contains "(I − x/2)⁻¹(I + x/2)"; `Construction` contains "C-01 to C-40".
- `alias_witness_points_at_indistinguishability_witness` (the target record exists, in NAB-05's range).
- `meanings_match_frozen_table`: byte comparison of each `meaning` with the frozen fixture
  `tests/fixtures/glossary/a_to_c.tsv` (the fixture is this table, frozen).

### End-to-end script and logging

`scripts/e2e/glossary.sh` (NAB-08) asserts the per-range counts (73 here) and regenerates the document; it logs
the count per range as a `metric` with its expected value, per STD-E2E.

### Known answers and negative controls

Known answers: 73 rows; 23 begin with A, 11 with B, 39 with C. Negative controls: deleting the `sense` of one
`Cell` record must fail the loader (`gloss.missing-sense`); swapping `Cascade` and `Causal policy` must fail
`gloss.unsorted`; changing "8 m" to "4 m" in `Anchored position` must fail `pinned_values`.

## Considerations

- A row is an abbreviation. When the owning crate's contract lands, compare, correct the row if they differ,
  and set `owner_crate`.
- The count of archetypes (eighteen design, seven interior), of claim kinds (seven) and of constructions (forty)
  are stated in rows; if an owning chapter changes a count, the row changes in the same commit.

## References

Appendix A, rows A to C.
