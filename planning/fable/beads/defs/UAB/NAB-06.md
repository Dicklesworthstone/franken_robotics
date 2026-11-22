---
key: "NAB-06"
title: "registries/glossary.toml: term records N to R (Negative control to Rung), 65 rows and one added (Reflex)"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "workspace", "delivery", "solid", "registry"]
parent: "WP-01"
deps: ["NAB-01"]
needs: ["§0.5: the subsystem table (23 names with their bounded contexts and crates), from which the missing `Reflex` row is taken (N00-03)"]
related: ["NAB-02", "NAB-03", "NAB-04", "NAB-05", "NAB-07", "NAB-08", "N00-03"]
gates: []
plan_refs: ["Appendix A", "§0.5"]
---
## Background

The glossary of FrankenRobotics is a registry, `registries/glossary.toml`, from which `docs/GLOSSARY.md` is
generated (NAB-01) and against which every `CONTRACT.md` is linted (NAB-02). It is filled in five beads by
letter range. This bead carries every term of art whose sort key begins with N, O, P, Q or R: 65 rows of the
plan-time glossary, and one row the plan-time glossary omitted. Backticks are ignored by the sort key, so
`provisional` belongs here.

A row gives the term, what it means in this project, and in parentheses the chapter subsection, construction
(`C-nn`), doctrine rule (`D-n`) or other identifier that owns the definition. The owning text governs where a
row abbreviates. Numbers quoted in a row are DESIGN values of the owning section unless the row says otherwise.
Tier: Solid.

## Scope

In: 66 `[[term]]` records: the 65 rows of the table and the added `Reflex` record. Out: the schema, loader and
generator (NAB-01); the lint (NAB-02); the other ranges.

## Technical approach

**Transcription rule.** One `[[term]]` record per row, in table order. `term` is the first column without
backticks (`code = true` if it was printed in backticks); a trailing parenthetical in the term column is the
`sense`; `meaning` is the second column up to its final parenthetical, which is split into `defined_in` (items
beginning `§` or `Appendix`) and `constructs` (items such as `C-29`, `GATE-TENFOLD-06`, `BET-13`); `kind` is
`subsystem`, `label`, `binary`, `class`, `record`, `quantity` or `concept` by the rule of NAB-03; `aliases` hold
plurals and further names; `design_values = true` when the meaning contains a number; `id` is the lower-cased
term with hyphens.

**The added row.** The project has 23 subsystem names: Marrow, Skeleton, Sinew, Kinesis, Phylum, Traction,
Habitat, Sensorium, Spark, Reflex, Cortex, Arena, Augur, Nemesis, Autopsy, Atelier, Chronicle, Ground, Foundry,
Tether, Lemma, Gauntlet and Airlock. The plan-time glossary defines 22 of them and omits **Reflex**. The record
is added, between `Referee coverage` and `Refusal`:

> Reflex | The subsystem that owns state estimation, model-based control, gait orbits, skill artifacts and
> runtime shields; its crates are `frob-control`, `frob-gait` and `frob-shield` (§22)

FACT: the subsystem table of the document-control chapter lists Reflex with exactly that bounded context and
those crates.

**Special cases in this range.**

- Code literal (kind `label`): `provisional` (a fast-tier result that has not been promoted).
- Subsystem rows: `Nemesis`, `Phylum`, `Reflex`.
- Classes and enumerations to keep verbatim: the four no-authorization flags `execution_authorized`,
  `hardware_safety_signoff`, `sim_to_real_certified` and, on build packets, `fabrication_released` (always
  false, with no setter); the five noninterference properties (presentation, hidden truth, unused sensors and
  batch composition cannot change physics or delivered observations; optics and contact cards cannot change
  each other's outputs); the five postures Consume, Subprocess, Lift, Oracle, Pattern; the two precision
  profiles `default` and `stiff`; the eight preflight stages of `design check`; the six reality statuses
  `no-claim`, `synthetic`, `published-anchor`, `candidate`, `calibrated`, `retired`; the four reuse grades
  (1 identity, 2 dependency, 3 implication, 4 none); the five port kinds (mechanical, shaft, electrical,
  thermal, signal); the two rewrite classes E (no bit changes under round-to-nearest with contraction off) and R
  (equal over the reals only; needs an error bound and a new kernel identity).
- Formulas and numbers: the observability Gramian W = Σ Φᵀ·Hᵀ·R⁻¹·H·Φ over a window, reported by its smallest
  singular value off the gauge; the paired lower bound is the one-sided 95% lower confidence bound of a paired
  ratio; the probability of necessity PN(c) = P(no failure under do(¬c) given failure); the pseudo-inertia is the
  4×4 matrix [[Σ, h], [hᵀ, m]], consistent exactly when positive definite; the qualification lane's error budget
  α_i = α/[i·(i + 1)]; the reality band [ŷ − q̂, ŷ + q̂]; the restricted mean time RMT = E[min(T, H)] at a common
  deadline H, where an unresolved mission costs H; robustness ρ = ψ/s; the North-Star scenarios NS-01 to NS-24;
  the partial ten-fold gate is GATE-TENFOLD-06 at M3 and the tournament is GATE-TENFOLD-07 at M7; a region
  certificate encloses one constraint on a box of at most about six design parameters.

**The rows** (65 as printed at plan time; the `Reflex` record above is the 66th).

| Term | Meaning, with the section or construction that defines it |
|---|---|
| Negative control | A planted defect or deliberately invalid input that a gate must catch (§38) |
| Nemesis | The subsystem that owns adversarial worlds and rare-failure estimation (§26) |
| Neutral adjudicator | The party outside both arms of the ten-fold test that releases receipts only after both arms close (§2.1, §39) |
| No-authorization flags | `execution_authorized`, `hardware_safety_signoff`, `sim_to_real_certified` and, on build packets, `fabrication_released`: always false, with no setter (§12.2, §37.9) |
| Noninterference | Five properties: presentation, hidden truth, unused sensors and batch composition cannot change physics or delivered observations, and optics and contact cards cannot change each other's outputs (§20.17) |
| North-Star scenario | One of NS-01 to NS-24, an end-to-end session the plan must make possible (§7) |
| Null fork | The empty intervention; it must reproduce the parent bit for bit (§27.5) |
| Observability Gramian | W = Σ Φᵀ·Hᵀ·R⁻¹·H·Φ over a window, reported by its smallest singular value off the gauge (C-20, §20.11) |
| Observation-preserving subgroup | Γ_obs ≤ Γ_mech: the elements that also preserve sensors, payload, task goal, gravity and declared terrain structure (C-09, §13.4, §20.13) |
| One-step defect | The difference after one control interval between a device world and the CPU advanced from the same committed state (§16.11) |
| Optical volume | A box or ellipsoid with an extinction coefficient and a backscatter albedo, for steam, dust and glare (§19.3) |
| Packet queue | The pending sensor and command packets of a world; part of every checkpoint and restore set (§11.7, §16.5, §20.4) |
| Padding | Capacity of a shape class above a world's own size, masked so that it changes no result (§17.5) |
| Paired lower bound | The one-sided 95% lower confidence bound of a paired ratio; the only form in which a performance ratio is admitted (§36.6) |
| Parameter variant | A per-world set of continuous parameters within one tree (§17.3) |
| Partial ten-fold gate | The M3 measurement of the ratio on legged and wheeled briefs, recorded whatever its value, with a written rule for each outcome (GATE-TENFOLD-06; §39.12, BET-13). The tournament at M7 is GATE-TENFOLD-07 |
| Patch reduction | The documented replacement of the samples of a small contact patch by one contact, with its qualification fixture (§18.11) |
| Per-world mutable state | Scene state that a world may change, such as film thickness or a moved proxy, allocated from fixed-capacity pools and held in every snapshot (§19.3) |
| Phase accumulator | The integer counter by which a process with a rational period fires (§11.7) |
| Phase ledger | The measured breakdown of the wall time of a design loop by phase, device and agent turn, taken on both arms (§36.15, §39) |
| Phylum | The subsystem that owns families of bodies in one batch (§17) |
| Pilot | A paper-scale experiment with a written falsifier that must run before a Frontier or Moonshot work package is funded (D-10) |
| Placebo fork | A fork whose change no physical cause could depend on, used to measure how often chaos alone flips a verdict (§27.5) |
| Placement receipt | The record of which device each stage was requested on and where it actually ran (§21.12) |
| Planted winner | A known-good design inserted into a screened population; losing it voids the screen (§28.12) |
| Polarity | The direction in which a certificate survives an amendment of the world set, the policy class or the predicate (C-18) |
| Port | A typed interface of a component: mechanical, shaft, electrical, thermal or signal (C-12) |
| Posture | How a foundation is used: Consume, Subprocess, Lift, Oracle or Pattern (§33.1) |
| Potency control | A planted effect of known size that a bet's test must detect before a null result counts (§4) |
| Power-of-two similarity test | Scaling every dimensional input by powers of two according to its dimension and requiring the run to reproduce in every bit (C-21, §17.11) |
| Pre-analysis | For a fixed-n study, the list of verdicts reachable at that n and the power at declared alternatives (§24.12) |
| Precision profile | `default` or `stiff`, the declared arithmetic of the contact solve (§21.14) |
| Preflight | The eight stages of `design check` (§14.11) |
| Probability of necessity | PN(c) = P(no failure under do(¬c) given failure), estimated over failing worlds (C-29) |
| Probe | A physical experiment chosen so that mechanisms implying different designs give disjoint readings (C-39, §25.14) |
| Promotion | Re-evaluating a fast-tier result on a higher tier before it is called established (§16.8) |
| Proof kind | An entry of the closed registry of certificate forms a bundle may contain (§12.4) |
| Proved | The evidence class of a general statement accepted by a Lean kernel (§12.2) |
| `provisional` | The label of a fast-tier result that has not been promoted (§16.8) |
| Pseudo-inertia | The 4×4 matrix [[Σ, h], [hᵀ, m]] equivalent to the ten inertial parameters; consistent exactly when positive definite (C-03, §11.5) |
| Qualification lane | The blinded confirmation service: pre-committed requests, fresh sealed clusters and the error budget α_i = α/[i·(i + 1)] (§24.13) |
| Quality-diversity archive | A map of different good designs by behaviour cell, in place of one optimum (§28.8, §17.12) |
| Rank agreement | Agreement of the order of designs between two rungs or tiers: Kendall τ, a top-weighted τ and top-k recall with intervals (C-26, §16.9, §28.10) |
| Rational clock | A process whose period is an exact rational number of seconds, advanced by a phase accumulator (§11.7) |
| Reality band | The conformal interval [ŷ − q̂, ŷ + q̂] reported for a simulated metric inside a calibrated cell (§30.12) |
| Reality status | `no-claim`, `synthetic`, `published-anchor`, `candidate`, `calibrated` or `retired` (§30.2) |
| Referee | The most trusted tier, built on FrankenSim; not ground truth (§16.3) |
| Referee coverage | The share of fast-tier world-seconds, and of standard-battery scenarios, that the referee has re-run (§16.11) |
| Refusal | A structured error `FROB-REF-nnn` that names the rule a request violates (Appendix O) |
| Region certificate | An outward-rounded enclosure of one constraint on a box of at most about six design parameters (C-32, §25.6) |
| Registered prediction | A forecast an agent files before a run, scored afterwards with a proper scoring rule (§28.17) |
| Replication kit | The public bundle of data, commands and expected outputs with which a third party repeats a Ground result (§30.18) |
| Requirement | A signal-temporal-logic formula over unit-normalized predicates (C-30, §24.5) |
| Rescue rate | The fraction of replicates of an intervention in which the requirement is not violated (§27.6) |
| Restricted mean time | RMT = E[min(T, H)] at a common deadline H; an unresolved mission costs H (§2.1) |
| Retraction | A map from the Lie algebra to the group used to advance state (C-05) |
| Retrofit | A priced, typed amendment to the building, drawn from a catalog and compared with design changes in one budget (§19.12) |
| Return map | The map P of a Poincaré section to itself whose fixed point is a periodic gait (C-16) |
| Reuse grade | The licence under which a stored verdict skips a recheck: 1 identity, 2 dependency, 3 implication, 4 none (§28.13) |
| Revalidation | Re-deciding stored verdicts after an amendment by one engine behind `result.revalidate` and `mission.revalidate` (§28.13, §25.8) |
| Rewrite class | Class E rewrites change no bit under round-to-nearest with contraction off; class R rewrites are equal over the reals only and need an error bound and a new kernel identity (C-14, §28.19) |
| Rigid proxy | A body without internal joints that stands for a soft object: a rug as a free slab, a cord as a ridge or rod (§19.10) |
| Robustness | ρ = ψ/s, the margin of a requirement atom divided by its declared scale (C-30) |
| Rotor port | A thrust-producing actuator port for flight, admitted as [M] behind a flag (§15.4) |
| Rung | One level of the ladder of evaluation, allowed to decide only what its evidence supports (§28.9) |

Rows in this range: 65.

## Acceptance criteria

- [ ] The registry contains 66 records whose sort key begins with N to R and loads; `Reflex` sits between
      `Referee coverage` and `Refusal`.
- [ ] Across the whole registry, the set of records of kind `subsystem` equals the 23 subsystem names.
- [ ] The four no-authorization flags, six reality statuses, five postures and four reuse grades are present
      verbatim in their rows.
- [ ] The generated document prints the 65 plan-time meanings character for character as in the table.

## Test plan

### Unit tests

`crates/frob-conformance/tests/glossary_content.rs`, module `n_to_r`:

- `count_is_66`; `order_matches_frozen_list` (with `Reflex` in place).
- `all_23_subsystems_defined` (whole registry): the 23 names above each resolve to a record of kind
  `subsystem`, and no other record has that kind.
- `pinned_values`: `Qualification lane` contains "α_i = α/[i·(i + 1)]"; `Restricted mean time` contains
  "E[min(T, H)]"; `Robustness` contains "ρ = ψ/s"; `Pseudo-inertia` contains "[[Σ, h], [hᵀ, m]]"; `Paired lower
  bound` contains "95%"; `Reality status` contains the six names; `Reuse grade` contains "1 identity, 2
  dependency, 3 implication, 4 none"; `Partial ten-fold gate` contains "GATE-TENFOLD-06" and "GATE-TENFOLD-07".
- `no_authorization_flags_are_four`.
- `meanings_match_frozen_table` against `tests/fixtures/glossary/n_to_r.tsv` (65 rows).

### End-to-end script and logging

`scripts/e2e/glossary.sh` (NAB-08) asserts the count of this range (66) and the subsystem completeness check,
and logs both as `metric` and `assert` lines per STD-E2E.

### Known answers and negative controls

Known answers: 65 plan-time rows (7 N, 4 O, 29 P, 2 Q, 23 R) plus one added, 66 records. With the budget
α_i = α/[i·(i + 1)] the levels sum to α over all i (Σ 1/[i·(i + 1)] = 1), and the first three are α/2, α/6 and
α/12: a test evaluates the formula from the row's text at α = 0.05 and expects 0.025, 0.008333…, 0.004166….
Negative controls: deleting the `Reflex` record must fail `all_23_subsystems_defined`; writing the budget as
α/i must fail the evaluation test (the sum would diverge); dropping `retired` from the reality statuses must
fail `pinned_values`.

## Considerations

- The omission of `Reflex` is a finding of the conversion, recorded in this unit's coverage file. The
  completeness test turns it into a standing check: a subsystem added later needs its row.
- `Refusal` codes are owned by `registries/refusals.toml`; the glossary row only says what a refusal is.

## References

Appendix A, rows N to R; §0.5 (the subsystem table).
