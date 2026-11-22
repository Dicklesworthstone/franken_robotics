---
key: "BET-10"
title: "Decide BET-10: agents design better with structure than with reward alone (the surface A/B, factor 1.5)"
type: "task"
priority: 1
milestone: "M4"
labels: ["m4", "atelier", "tether", "frob-opt", "solid", "bet", "gate"]
parent: "WP-28"
deps: ["N00-28", "BD-317", "BD-374", "STD-E2E"]
needs: ["§24.9: the design benchmark and the planted-defect corpus on which turns to a fix are counted", "§39.5: arm B of the partial ten-fold gate (the same agent on the composed incumbent tool layer)"]
related: ["BD-248", "BD-372", "BD-373", "BET-13", "BET-14", "BET-23", "N00-19", "N00-15"]
gates: ["GATE-ATEL-03", "GATE-TENFOLD-06"]
plan_refs: ["§4 (BET-10)", "§40.5", "§28.23", "§1.6 (N9)"]
---
## Background

**The bet.** The studio gives an agent structure: typed designs and typed edits, static analyses with ranked
repairs, structured telemetry with first causes, paired comparison, lineage and bisect, counterfactual autopsy,
budgets and typed refusals. The alternative is what most simulators give: run it and get a number. The bet is
that an agent with the full surface reaches an accepted design in materially less time than the same agent with
only `sim run` and a scalar score, at equal compute and tokens.

**Why it could fail.** A capable agent may extract most of the value from a score and its own reasoning; a rich
surface costs tokens to read and turns to learn. The surface is only worth its upkeep if it changes outcomes.
Tier: Solid.

## Scope

In: pre-registration, the M3 reading (arm B of the partial ten-fold gate) and the M4 reading (the surface A/B),
verdict, the per-verb effect table and the decision record. Out: the benchmark, the agent harness, the arms.

## Technical approach

1. **Falsifier.** On the design benchmark, agents given the full studio surface do not reach an accepted design
   in at most half the restricted mean time of agents given only `sim run` and a scalar score, at equal compute
   and tokens. Statistical form: the one-sided 95% lower bound on the ratio of restricted mean times must be
   above 1.5.
2. **Competing arms.** (i) The same agent with `sim run` and a score. (ii) The same agent on the composed
   incumbent tool layer (this arm is run inside the ten-fold test). **Potency control.** A planted-defect corpus
   on which turns to a fix are counted: the harness must show that a surface which hides the planted cause
   takes more turns.
3. **Deciding gates (TARGETs).**
   - GATE-ATEL-03 (Design comparison, M4): the pre-registered A/B of agent surfaces on the design benchmark,
     with an anytime-valid verdict and an effect size: restricted mean time to an accepted answer lower by a
     factor of at least 1.5 on at least ten briefs.
   - Arm B of GATE-TENFOLD-06 (M3): the same comparison against the composed incumbent tool layer, recorded with
     its phase ledger.
4. **No kill gate; the consequence is written.** The surface is reduced to the verbs that showed an effect. The
   rest are kept as audit tools and carry no speed claim.
5. **Per-verb effect table.** To make the consequence executable, the experiment records for each verb cluster
   whether removing it changes the time to acceptance (ablation arms pre-registered with the main arms); the
   decision record lists clusters as `effect shown`, `no effect shown`, `not tested`.
6. **Downstream if falsified.** The MCP surface's default visible clusters shrink to those with an effect;
   documentation stops describing the others as accelerating design; the "explain by intervention" and "decide
   many designs per turn" levers are re-read against the phase ledger, since they rely on the surface being
   used.
7. **Evidence so far.** None: no instrumented design session exists on any stack, and no published figure
   measures agent repair turns.
8. **Procedure.** Register briefs, arms, budgets (compute and tokens), the agent model and the analysis rule
   before the first session; fresh contexts for every session; read at M3 (arm B) and M4 exit;
   `docs/adr/bet-10-m4.md`.

## Acceptance criteria

- [ ] Pre-registered: at least ten briefs, the arms, equal compute and token budgets, the anytime-valid rule and
      its reachable budget under the declared alternative.
- [ ] Restricted mean times per arm, the ratio with its one-sided 95% lower bound against 1.5, and the
      completion counts (unfinished sessions cost the deadline).
- [ ] The per-verb effect table exists.
- [ ] Verdict recorded; if falsified, the reduced default surface is specified by cluster.

## Test plan

### Unit tests
`bets.rs`: `bet10_lower_bound_1_62_holds`; `bet10_point_1_7_lower_bound_1_3_falsifies`;
`bet10_unequal_token_budgets_rejected`; `bet10_fewer_than_ten_briefs_is_untested`;
`bet10_reduction_lists_clusters_without_effect`.

### End-to-end script and logging
`scripts/e2e/bet_10_decide.sh` recomputes the ratio and its bound from the study's ledger records (charged time
per session with the deadline rule of the metric contract), checks budgets are equal across arms, and checks the
ADR. Logged per STD-E2E: per brief and arm the charged time, tokens, compute, acceptance; the ratio, bound and
threshold; the effect table; `summary` naming GATE-ATEL-03 and GATE-TENFOLD-06.

### Known answers and negative controls
The completion cap: an arm that leaves a fraction q of briefs unresolved cannot show a ratio above 1/q (KA-009).
Negative controls: the planted-defect corpus; an A/A run of the same surface against itself (must not show an
effect); sessions that share context (rejected: each session starts fresh).

## Considerations

- Agent models change; the result is tied to the model and version in its record and is re-run when the common
  agent changes.
- "Accepted" is defined by the benchmark's frozen battery, never by the arm's own report.
- Human-readable richness is not the goal; an effect on time to acceptance is.

## References

§4 (BET-10), §40.5, §28.23, §1.6 (N9); GATE-ATEL-03; GATE-TENFOLD-06 (arm B).
