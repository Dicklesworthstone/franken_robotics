---
key: "N41-OPEN4"
title: "registries/open.toml, part 4: open questions of Reflex, gaits, shields, Cortex and Arena (19 rows)"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "delivery", "reflex", "cortex", "arena", "registry", "solid"]
parent: "EPIC-M0"
deps: ["N41-03"]
needs: []
related: ["N41-OPEN5"]
gates: []
plan_refs: ["§42.3", "§42.2", "§22.19", "§23.24", "§24.18"]
---
## Background

Fourth part of the open-question register (rule and schema in N41-03; how to read a row in N41-OPEN1). The
subsystems:

- **Reflex**: baseline controllers that need no training, the sampling controller, gait orbits (`OPEN-GAIT`),
  and runtime shields with energy budgets (`OPEN-SHIELD`);
- **Cortex**: learning (the vectorized environment, on-device policy optimization, family policies and
  inheritance across design edits, the Gymnasium-compatible adapter);
- **Arena**: tasks, requirement language, scorecards, fixtures and the blinded qualification lane.

Three rows need the owner. OPEN-CORTEX-1: the compute envelope for pretraining family policies (proposal: one
class, quadruped, 200 designs, at most 60 GPU-hours of pretraining and 40 for specialists, on the two-GPU machine);
default no pretraining, so no family policy exists and M4 exits in its variant with specialists and inheritance.
OPEN-ARENA-3: who holds the qualification administrator scope and how large the sealed world families are;
default the owner alone opens and closes campaigns, one sealed family per task family sized for 20 requests at 256
clusters, and nothing is called qualified until a family exists. OPEN-SHIELD-2: the calibration level and violation
budget of runtime shields; default 10⁻³ and 10⁻³ per episode.

## Scope

In: the nineteen rows below as records of `registries/open.toml`. Out: the schema and loader (N41-03); the
decisions (the beads in the last column).

## Technical approach

| ID | Question | Fail-closed default | Owner | Blocks, or decided by | Deadline | Decided in |
|---|---|---|---|---|---|---|
| OPEN-REFLEX-1 | Whether the predictive controller runs on device | The single-step program on every tier; the predictive controller on CPU tiers only until measured | Reflex lead | M3 | M3 | N22c-50 |
| OPEN-REFLEX-2 | The rank-agreement thresholds of GATE-REFLEX-04: the pair (0.7, 0.5) or the pair (0.6, 0.3) | The stricter pair, fixed before data; until the gate has run the baseline rung discards only | Reflex lead | GATE-REFLEX-04 | M3 | N22c-51 |
| OPEN-REFLEX-3 | The learned residual's authority ρ per channel | 0.3 of range | Reflex lead | GATE-REFLEX-05 | M3 | N22c-52 |
| OPEN-REFLEX-4 | Whether screening by the sampling controller needs a 4 ms planning step | 20 ms, labelled in the controller identity, with results flagged `model-privileged` | Reflex lead | GATE-REFLEX-06 | M4 | N22c-53 |
| OPEN-GAIT-1 | When the referee route for gait orbits is built | Compliant tiers only | Kinesis lead, with upstream | Nothing on the critical path | M4 | N22c-54 |
| OPEN-GAIT-2 | The norm and tolerance for impact-order sensitivity | The mass norm and 1% of the pre-impact speed | Reflex lead | GATE-GAIT-02 | M4 | N22c-54 |
| OPEN-SHIELD-1 | Where absorber envelopes come from before any bench data exist | `assumed` cards and `Estimated` claims | Sinew lead, with Ground | Any energy claim above `Estimated` | M4 | N22b-17 |
| OPEN-SHIELD-2 | The calibration level α and the per-episode violation budget | α = 10⁻³ and a budget of 10⁻³ per episode | Owner | GATE-SHIELD-03, GATE-SHIELD-04 | M3 | N22b-27 |
| OPEN-CORTEX-1 | The compute envelope for family-policy pretraining | No pretraining | Owner | GATE-CORTEX-11 and every use of a family policy | M3; M4 needs the answer on its first day | N23b-24 |
| OPEN-CORTEX-2 | The default action law | `latent-then-limit` | Cortex lead | GATE-CORTEX-07 | M2 | N23b-25 |
| OPEN-CORTEX-3 | The reset schedule for lineages (depth k and audit rate m) | k = 5 and m = 8 | Cortex lead | GATE-CORTEX-13 | M4 | N23a-29 |
| OPEN-CORTEX-4 | The adapter's transport: shared device views, or one copy per step across a process boundary | One charged copy per step | Tether lead | GATE-CORTEX-16 | M3 | N23b-27 |
| OPEN-CORTEX-5 | Default discount rates β and κ per task family | β = 0.5025 /s and κ = 2.565 /s, which give γ = 0.99 and λ = 0.95 at 20 ms | Arena lead | Nothing | none | N23b-25 |
| OPEN-CORTEX-6 | Whether the incumbent wall-clock of GATE-CORTEX-05 is re-measured on our card before M2 | The gate is recorded against the published figure, labelled `published-context`, until the re-measurement exists | Spark lead | Citing GATE-CORTEX-05 as passed; closed by GATE-SPARK-21 | M2 | N23b-28 |
| OPEN-ARENA-1 | Which public task definitions to mirror for comparability | The reward terms of the public velocity-tracking tasks, with no claim of score parity | Arena lead | M2 | M2 | N24c-16 |
| OPEN-ARENA-2 | A permissively licensed hexapod reference model | None; the template stands alone | Arena lead | M4 | M4 | N24c-16 |
| OPEN-ARENA-3 | The size and composition of the sealed world families, and who holds `qual:admin` (the qualification administrator scope) | The owner alone opens and closes campaigns; one sealed family per task family, sized for 20 requests at 256 clusters; no result is called qualified until a family exists | Project lead; a question for the owner | GATE-QUAL-01 | M2 | N24c-13 |
| OPEN-ARENA-4 | Whether fixture 2's quadruped is the reference template or a dedicated minimal body | The reference template at a pinned `DesignId` | Arena lead | GATE-ARENA-06 | M2 | N24c-14 |
| OPEN-ARENA-5 | The guard band for boundary atoms on each backend | 2⁻²¹, with CPU re-evaluation of every flagged atom | Spark lead | The device half of GATE-ARENA-03 | M2 | N24c-15 |

Status at entry: nineteen `open`. One default is confirmed by the gate register and is open only to new evidence:
the stricter threshold pair of OPEN-REFLEX-2 (0.7 to select, 0.5 to discard). The check on the default of
OPEN-CORTEX-5: with a control interval Δt = 0.02 s, γ = exp(−β·Δt) = exp(−0.01005) = 0.99 and λ = exp(−κ·Δt) =
exp(−0.0513) = 0.95.

## Acceptance criteria

- [ ] `registries/open.toml` contains the nineteen records with no empty required field.
- [ ] OPEN-REFLEX-2 has `confirmed_by_gate_register = true` and its pair equals the gate register's.
- [ ] The three owner rows (OPEN-SHIELD-2, OPEN-CORTEX-1, OPEN-ARENA-3) carry the cost of their defaults and the
      milestone at which each is asked (one before its deadline).
- [ ] Every row names a decision or experiment bead that exists; where two units had drafted one (OPEN-SHIELD-1,
      OPEN-SHIELD-2, OPEN-CORTEX-1, -2, -3) the row names the bead the chapter group kept (N22b-17, N22b-27, N23b-24,
      N23b-25, N23a-29).
- [ ] The discount check of OPEN-CORTEX-5 reproduces 0.99 and 0.95 to three significant figures.

## Test plan

### Unit tests
`crates/frob-conformance/tests/open_part4.rs`: `nineteen_rows_load`; `ids_match_expected_set` (REFLEX-1 to -4,
GAIT-1, -2, SHIELD-1, -2, CORTEX-1 to -6, ARENA-1 to -5); `reflex_2_confirmed_pair_matches_gate_register`;
`cortex_5_discounts` (exp(−0.5025 × 0.02) = exp(−0.01005) = 0.9900003… and exp(−2.565 × 0.02) = exp(−0.0513)
= 0.9499936…, so 0.99 and 0.95 to three significant figures); `owner_rows`;
`decision_beads_resolve`.

### End-to-end script and logging
Covered by `scripts/e2e/registers_exit.sh` (N41-04); the M3 walk asserts that OPEN-CORTEX-1 at its default produces
the default ADR and the expectation of the M4 variant with no family policy.

### Known answers and negative controls
Known answers: 19 rows; γ = 0.99 and λ = 0.95; the pairs (0.7, 0.5) and (0.6, 0.3). Negative controls: a result
labelled as from a family policy while OPEN-CORTEX-1 is at its default; a result called `qualified` with no sealed
family on record; OPEN-REFLEX-2 edited to the looser pair without an ADR.

## Considerations

- OPEN-CORTEX-1 is one of the six questions that matter most; its default removes a whole lever of the thesis and
  the program still exits M4, which is what a default is for.
- OPEN-GAIT-1 has the same decision bead as OPEN-GAIT-2 and blocks nothing on the critical path; its deadline is
  the gait work's own milestone.

## References

§42.3 (rows for Reflex, gaits and shields, Cortex, Arena), §42.2 (owner rows), §42.4 (confirmed defaults).
