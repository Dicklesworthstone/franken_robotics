---
key: "NAB-07"
title: "registries/glossary.toml: term records S to Z (Saltation matrix to Wreath product), 78 rows"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "workspace", "delivery", "solid", "registry"]
parent: "WP-01"
deps: ["NAB-01"]
needs: []
related: []
gates: []
plan_refs: ["Appendix A"]
---
## Background

The glossary of FrankenRobotics is a registry, `registries/glossary.toml`, from which `docs/GLOSSARY.md` is
generated (NAB-01) and against which every `CONTRACT.md` is linted (NAB-02). It is filled in five beads by
letter range. This bead carries the last range: every term of art whose sort key begins with S to Z, 78 rows.
Backticks are ignored by the sort key, so `screening`, `strict`, `sync_wait_ms`, `tier-sensitive` and `unknown`
belong here.

A row gives the term, what it means in this project, and in parentheses the chapter subsection, construction
(`C-nn`), doctrine rule (`D-n`) or other identifier that owns the definition. The owning text governs where a
row abbreviates. Numbers quoted in a row are DESIGN values of the owning section unless the row says otherwise.
Tier: Solid.

## Scope

In: the 78 `[[term]]` records of this range. Out: the schema, loader and generator (NAB-01); the lint (NAB-02);
the other ranges.

## Technical approach

**Transcription rule.** One `[[term]]` record per row, in table order. `term` is the first column without
backticks (`code = true` if it was printed in backticks); a trailing parenthetical in the term column is the
`sense`; `meaning` is the second column up to its final parenthetical, which is split into `defined_in` (items
beginning `§` or `Appendix`) and `constructs` (items such as `C-21`, `D-13`, `BET-14`); `kind` is `subsystem`,
`label`, `binary`, `class`, `record`, `quantity` or `concept` by the rule of NAB-03; `aliases` hold plurals and
further names; `design_values = true` when the meaning contains a number; `id` is the lower-cased term with
hyphens, plus `-<sense>`.

**Special cases in this range.**

- Senses: `Tier` has two records. `maturity`: [S] Solid, [F] Frontier or [M] Moonshot, carried by every
  mechanism. `physics`: `referee`, `batch-cpu`, `gpu-cuda` or `gpu-metal`. The word "tier" is the most
  overloaded in the project (it also occurs in "Gauntlet tier G0 to G5" and "telemetry level T0 to T3", which
  have their own terms), so the lint's sense rule matters most here.
- Multi-name row: `Toolchain T; toolchain train` (one record; alias `toolchain train`). T is the one Rust
  toolchain of the whole build graph, chosen at the admission gate INT-TOOLCHAIN-01 and recorded in
  `franken-stack.lock` and `rust-toolchain.toml`; a toolchain train is the one coordinated change that moves T
  here and in every consumed sibling that needs it, followed by the admission and parity gates.
- Code literals (kind `label`, except `sync_wait_ms`, which is a `quantity`): `screening`, `strict`,
  `sync_wait_ms`, `tier-sensitive`, `unknown`.
- Structural rows (defined by the glossary itself; `structural = true`): `Stratum`, `Witness`, `World`.
- Subsystem rows: `Sensorium`, `Sinew`, `Skeleton`, `Spark`, `Tether`, `Traction`.
- Enumerations to keep verbatim: the three step profiles `lockstep`, `strict`, `referee`; the four telemetry
  levels T0 per-episode metrics, T1 events, T2 control-rate traces on sampled worlds, T3 restart snapshots; the
  two solvability-witness kinds `solvable-causal` and `plant-feasible`; the theorem cards T-01 to T-35; the two
  study types (anytime-valid, fixed-n).
- Formulas and numbers: the similarity group 𝕊 = (ℝ₊)³ (changes of the units of mass, length and time); the
  survivor set (every design whose lower-rung score is within 2·q̂ of the k-th best lower-rung score); the
  transversality number D_t g + D_x g·f⁻ (near zero is grazing); the world address (run_seed, world, episode); a
  world-second is 1/h world-steps; the wreath product K ≀ H; a verb is a `cluster.verb` entry; the `unknown`
  verdict (a solver failure, a timeout or an unenclosed branch never yields success or failure).

**The rows.**

| Term | Meaning, with the section or construction that defines it |
|---|---|
| Saltation matrix | Ξ_e, the first-order map of perturbations across an isolated transverse event (C-16) |
| Sampling controller | A sampling-based predictive controller that needs no training; a screening scorer and fallback that never stands in for a trained policy as acceptance evidence. Its own episodes are accepted only where it is the delivered controller (§22.8) |
| Scene pack | A compiled scene `.fscene`: manifest, distance bricks, materials, fixtures, optical volumes and hazards (§19.3) |
| Scene split | The mechanical separation of training, development and held-out scenes, with building-level holdouts (§19.14) |
| Schedule class | An ordering of timing events modulo commutation of independent events; a race is explored once per class (C-19) |
| Scorecard | A design's metrics beside those of the reference robot of its class (§24.8) |
| `screening` | The label of a score from a rung that may discard and may not establish, such as a sampling controller or a gait orbit (§22.8, §22.9) |
| Sealed suite | The missions of the ten-fold test, fixed and sealed before either arm sees them (§39) |
| Segment mode | Re-simulation of one-second segments from fast-tier snapshots on a higher tier; it estimates a different quantity and says so (§16.8) |
| Sensorium | The subsystem that owns sensors, clocks and observability (§20) |
| Separating world | The admitted world nearest to nominal on which two closed loops receive different verdicts (C-18) |
| Settled-pose reset cache | Precomputed quasi-static start states per model, spawn, contact law and kernel version (§24.7) |
| Shadow recompute | Recomputing a sampled fraction of memo hits to detect an incomplete key (§28.14) |
| Shape class | The set of capacities one static step plan is built for, identified by a `ShapeClassId` (§17.4) |
| Shared dialect | The common subset of CUDA C++ and Metal Shading Language, with per-backend preludes (§21.6) |
| Shield | A runtime filter that projects a policy action onto barrier rows inside the actuator box, with a KKT check and a passive braking fallback (C-31, §22.12) |
| Similarity group | 𝕊 = (ℝ₊)³, changes of the units of mass, length and time, under which executions map to executions (C-21) |
| Sinew | The subsystem that owns actuators, transmissions and power (§15) |
| Size bucket | A shape class chosen so that robots of similar size share padded capacities (§17.5) |
| Skeleton | The subsystem that owns the design language, modules, archetypes and preflight (§14) |
| Skill | A behaviour stated at the task boundary, independent of the body, stored as `.fskill` (§22.10) |
| Skill transport | Carrying a skill to another body through its task interface, exactly where the interface permits and by a learned residual otherwise [M] (§22.10, §25.11) |
| Slot-level dispatch | The daemon's assignment of episodes to free world slots of a persistent environment; the unit of work is a world, not a job (§21.11, §32.11) |
| Snapshot | A full state of selected worlds sufficient for exact restart, including queues and warm starts (§29.8) |
| Solvability witness | Evidence that the candidate's own body can succeed in a failing world: `solvable-causal` or `plant-feasible` (§26.5) |
| Spark | The subsystem that owns GPU backends and the kernel toolchain (§21) |
| Specialist | A policy trained or inherited for one design alone; rung R3 (§28.9) |
| Specialization of survivors | Per-design kernel specialization, applied only to finalists and only if the profile justifies it (§21.18) |
| Splitting | Multilevel splitting on a distress score with fixed levels; an exploration method that issues `Estimated` rates and no bound (C-27, §26.9) |
| Stack lock | The file that pins every sibling's commit and records the one toolchain T, with a receipt of what was built (§33.5) |
| Stage signature | A kernel's declared read and write footprints per buffer and its aliasing rules, held in its manifest (§21.5) |
| Step plan | The static list of kernels, buffers and launches built at environment creation, identified by a `StepPlanId` (§21.11) |
| Step profile | `lockstep`, `strict` or `referee`; part of the step-plan identity every result carries (§16.3) |
| Straight-line program | Branch-free arithmetic with shared subexpressions; the form of derived kernel bodies (C-14) |
| Stratum | A block of design space, missions or contact modes for which a statistic or a verdict is reported separately |
| `strict` | The `batch-cpu` profile that runs the transactional, event-located step (§16.3) |
| Strongest composed rival | Leading simulators, an optimizer, analytic bounds, a verifier and the same agent, assembled by a competent engineer and frozen (§2.1, §39) |
| Structural proxy | The [S] form of an interior archetype: boxes and fixtures with the hazards' dimensions (§19.7) |
| Study | A registered plan to answer one comparative or absolute question, of type anytime-valid or fixed-n (§24.12) |
| Survivor set | Every design whose lower-rung score is within 2·q̂ of the k-th best lower-rung score (§28.9) |
| `sync_wait_ms` | The bounded synchronous wait a long tool takes before returning a job handle (§32.10) |
| Synthetic rehearsal | The run of the whole Ground procedure against data from a simulator standing in for reality; never promoted (§30.21) |
| Task | A `.ftask` file: terms, requirements, randomization, clocks and symmetry declarations (§24.4) |
| Telemetry level | T0 per-episode metrics, T1 events, T2 control-rate traces on sampled worlds, T3 restart snapshots (§29.5) |
| Teleport | Replaying a GPU world on the CPU from its address to find where tiers part (§29.8) |
| Ten-fold test | The comparison of restricted mean times to an independently accepted answer against the strongest composed rival on the sealed suite (§39) |
| Tested | The evidence class of a property held on a stated set of cases (§12.2) |
| Tether | The subsystem that owns the CLI, the daemon, the MCP server, the dashboard and the viewer (§32) |
| Theorem card | A statement the project wants, with its route and tier: T-01 to T-35 (§12.3) |
| Tie rule | If a short robust program over an incumbent engine ties, the plan says so and ships the layer (BET-14, §39) |
| Tier (maturity) | [S] Solid, [F] Frontier or [M] Moonshot, on every mechanism (§0.1) |
| Tier (physics) | `referee`, `batch-cpu`, `gpu-cuda` or `gpu-metal` (§11.1, §16.3) |
| Tier-correction ledger | The record of audited paired differences between tiers that feeds the doubly robust estimate (§29.11) |
| `tier-sensitive` | The label of a result that changes when it is re-run one tier up (§16.8, §23.16) |
| Toolchain T; toolchain train | T is the one Rust toolchain of the whole build graph, chosen at INT-TOOLCHAIN-01 and recorded in `franken-stack.lock` and `rust-toolchain.toml`. A toolchain train is the one coordinated change that moves T here and in every consumed sibling that needs it, followed by the admission and parity gates (§33.13) |
| Top-weighted τ | A Kendall rank correlation in which discordances near the top of the ranking count more (§16.9) |
| Traction | The subsystem that owns collision, contact, wheels, tracks and terrain (§18) |
| Transactional step | The referee and `strict` step: one transaction per base interval that commits a complete checked interval or leaves no trace (§16.5) |
| Transport | T_e, the initialization of a child's policy defined by the correspondence an edit declares (C-23) |
| Transversality number | D_t g + D_x g·f⁻, the rate at which a trajectory crosses a guard; near zero is grazing (C-16) |
| Trust ladder | The rule that fast-tier results are provisional until promoted, with detectors for exploitation (§16.8, §23.16) |
| Tube table | A table of mode-indexed cells with one safe action per cell, closed under resets, sensing error and disturbance (C-31) |
| `unknown` | A verdict: a solver failure, a timeout or an unenclosed branch never yields success or failure (D-13) |
| Unresolved cell | A cell of O ∖ I, reported with its competing explanations and the next test that would split it (C-17, §25.3) |
| Upstream work item | A change this program makes in a sibling repository, under that repository's own rules: a bead here with the label `upstream`, mirrored in the sibling's tracker, with an interim posture and a switch-over rule (§33.8, Appendix F) |
| Value of information | The expected reduction in decision loss from an experiment, by which experiments are ranked per unit cost (C-39, §28.16) |
| Verb | A `cluster.verb` entry of the one registry from which the CLI, the MCP tools and the documentation are generated (§32.6, Appendix J) |
| Verdict | The recorded result of applying a declared rule to evidence, with its class, scope and receipts (§28, §29.13) |
| Witness | An object that makes a claim checkable: a controller with its certificate for an inner cell, a path, a dual vector, a pair of worlds, a replayable trace |
| Witness migration | Re-detection of a contact's witness pair, with transfer of its tangential memory, when sustained contact moves across features (§16.5, §18.4) |
| Work package | A `WP-*` unit of funded scope with entry pilots and exit gates (§44) |
| World | One simulated instance of a scene variant, a robot and its random draws, named by a world address |
| World address | `(run_seed, world, episode)`; it determines every random draw a world sees (§11.1, §35.9) |
| World bank | The set of compiled scene packs resident on a device (§19.15) |
| World set | W, the worlds a mission query quantifies over, declared before any candidate is tested (C-17) |
| World-second | One second of simulated time of one world: 1/h world-steps (§36.2) |
| World-step | One physics step of one world (§16.1, §36.2) |
| Wreath product | K ≀ H, the group that acts when modules with their own symmetries sit in slots a hub permutes (C-22, §13.6) |

Rows in this range: 78.

## Acceptance criteria

- [ ] The registry contains 78 records whose sort key begins with S to Z, in the order above, and loads.
- [ ] `Tier` has two records with senses `maturity` and `physics`.
- [ ] `Stratum`, `Witness` and `World` are the only records of this range with `structural = true`.
- [ ] With NAB-03 to NAB-06 landed, the whole registry holds 337 records (73 + 59 + 61 + 66 + 78) and the
      generated `docs/GLOSSARY.md` has 337 table rows.
- [ ] The generated document prints the 78 meanings character for character as in the table.

## Test plan

### Unit tests

`crates/frob-conformance/tests/glossary_content.rs`, module `s_to_z`:

- `count_is_78`; `order_matches_frozen_list`.
- `tier_has_two_senses`; `structural_rows_are_three`; `toolchain_train_is_an_alias`.
- `pinned_values`: `Survivor set` contains "2·q̂"; `Similarity group` contains "(ℝ₊)³"; `World address` contains
  "(run_seed, world, episode)"; `World-second` contains "1/h"; `Telemetry level` contains "T0" and "T3";
  `Theorem card` contains "T-01 to T-35"; `Step profile` contains the three names; `Tier (physics)` contains the
  four tier names; `Transversality number` contains "D_t g + D_x g·f⁻".
- `whole_registry_count_is_337` (runs once all five ranges are present).
- `meanings_match_frozen_table` against `tests/fixtures/glossary/s_to_z.tsv`.

### End-to-end script and logging

`scripts/e2e/glossary.sh` (NAB-08) asserts the count of this range (78) and the total (337), logged as `metric`
lines with expected values per STD-E2E.

### Known answers and negative controls

Known answers: 78 rows (42 S, 20 T, 3 U, 3 V, 10 W); total 337. Negative controls: merging the two `Tier`
records must fail `tier_has_two_senses` and, in NAB-02's lint, make an unqualified "tier" in a third crate
pass when it should not; removing `structural = true` from `World` must fail the loader with
`gloss.no-definition`; changing "2·q̂" to "q̂" in `Survivor set` must fail `pinned_values`.

## Considerations

- `unknown` is a verdict that appears in almost every envelope; its row must stay identical in meaning to the
  doctrine rule it abbreviates (a solver failure, a timeout or an unenclosed branch never yields success or
  failure).
- The three structural rows are the only definitions the glossary owns outright. If a chapter later defines
  `World` more precisely, the row gains a `defined_in` and loses the flag.

## References

Appendix A, rows S to Z.
