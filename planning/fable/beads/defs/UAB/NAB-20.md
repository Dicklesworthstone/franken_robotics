---
key: "NAB-20"
title: "registries/ev_matrix.toml: the 43 families parked, deferred or rejected, with reasons and reopening evidence"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "delivery", "gauntlet", "solid", "registry", "decision-record"]
parent: "EPIC-M0"
deps: ["NAB-17"]
needs: []
related: ["NAB-22", "NAB-32", "NAB-33", "NAB-34", "NAB-35", "NAB-36", "NAB-37", "NAB-38", "NAB-39"]
gates: []
plan_refs: ["Appendix C.1.3", "Appendix C.1", "§28.19.4"]
---
## Background

A plan is defined as much by what it declines as by what it builds. FrankenRobotics scored every family of
mechanism it considered (Score = (Impact × Confidence) ÷ Effort, each from 1 to 5, printed to one decimal; a
DESIGN judgment), and this bead carries the third table of that matrix (`registries/ev_matrix.toml`, schema
`frob.evmatrix/1`, NAB-17): the 43 families that are **parked, deferred or rejected**.

The vocabulary is exact. A **parked** row may be reopened by the evidence named in its decision. A **deferred**
row is real work that is not on the present path (it waits for a milestone or for an upstream change). A
**rejected** row is not built; where the rejection rests on a calculation, the decision points to its record
in the killed-idea register (`KI-nn`, NAB-33 to NAB-40), which holds the strong form, the refuting numbers and
the narrower version that survives. Two rows are neither: coalition-value attribution is *kept as a budgeted
summary only*, and barrier contact for articulated bodies is *left to the sibling's own lane and not built
here*. Methods that were weighed for one decision core (the eleven recurring decisions DC-01 to DC-11, each
with a compiled table and a deterministic fallback) and rejected are scored here and listed with reasons in
`KI-45`.

Eleven of the 43 rows score 2.0 or more and are still rejected, parked or deferred: a good score does not
rescue a family whose objective is wrong (bandit allocation scores 3.0 and is rejected because regret is the
wrong objective for identifying the best design). Rows are sorted by score, highest first. Tier: Solid
(a decision record).

## Scope

In: 43 `[[family]]` records, ids `EV-P01` to `EV-P43`, table `parked`, each with a machine-readable decision,
its reason, and `reopen_on` where the decision allows reopening. Out: the schema and lints (NAB-17); the
killed-idea records the rows point to; the revive lint (NAB-32).

## Technical approach

**Transcription rule.** One record per row, in table order, as in NAB-18. `decision` is the first word of the
decision text mapped to `rejected`, `parked`, `deferred`, `kept-as-summary` ("Kept as a budgeted summary only")
or `left-to-sibling` ("Left to FrankenSim's own lane; not built here"). `reason` is the text after the colon.
Three rejected rows have no colon: for EV-P14 and EV-P40 `reason` is the qualifier after the decision word
("under D-2"; "at current tool limits; inside the caps, certified inverse kinematics is [F]"), and for EV-P39,
whose decision is the single word "Rejected", it is "below 2.0 (table rule 1); the plan states no other
reason" (DESIGN). `killed_idea` holds the `KI-nn` named in the row. `queue` is `["MQ-21"]` for EV-P35 (general
Lean theorems, the deferred item of the Moonshot queue, NAB-21) and `[]` for every other row. `reopen_on` is filled as follows (the text of the row where
it gives one; otherwise the standing rule that a verdict changes only on new evidence of the kind that
produced it):

| Row | `reopen_on` |
|---|---|
| Pressure-field (hydroelastic) contact (deferred [F]) | A contact law of that form is specified and passes the contact known-answer battery; none of the present laws implements it |
| Contact-mode enumeration by hyperplane arrangements (parked) | A consumer that needs strata beyond the counts up to symmetry orbits |
| Gradients through contact (deferred to M6) | The M6 review; regime-local and return-map derivatives with validity labels arrive earlier |
| E-graph saturation over real-arithmetic rewrites (rejected) | Only the profile condition of decision core DC-11: our own M2 profile shows more than about a fifth of step time in derived arithmetic |
| Specialized kernels for tournament survivors (parked [F]) | The M2 profile shows a specializable share of at least one third (KILL-SPECIALIZE; GATE-SPARK-14) |
| Operator-theoretic (Koopman) locomotion models (parked) | One bounded experiment on a single soil regime (KI-22) |
| Path signatures as telemetry features (parked) | A decision that a signature feature would change, shown on recorded telemetry |
| Sheaf-theoretic sensor fusion (parked) | A fusion need the invariant filter and the least-squares pose graph do not cover |
| General Lean theorems on Lie groups (deferred [M]) | The proof library's blocked modules are unblocked |
| General 6R inverse kinematics by Gröbner bases (rejected at current tool limits) | Raised caps in the exact solver; inside the caps certified inverse kinematics is [F] |
| Half precision on matrix accelerators (rejected; refused by `FROB-REF-245`) | The condition of KI-01 |
| Quaternionic-type irreducible blocks (rejected; `FROB-REF-214`) | A group in scope with such a block (none exists; KI-48) |
| Every other rejected row | New evidence of the kind that produced the rejection: a pilot, a measurement at a gate, or a primary source re-read |

**Reasons that the one-line decisions compress, stated in full** (an implementer who is tempted by one of these
should find the reason here):

- *Bandit allocation* for the ladder or the queue: regret is the wrong objective for identifying the best
  design, and on dispatch a bandit is a second controller on shared telemetry. Used instead: the fixed halving
  table, and the conformal ladder where rank agreement licenses it.
- *Expected information gain* as the selection rule: it buys knowledge that cannot change the choice. Used
  instead: value of information per cost, as a stop rule.
- *A second in-house geometric-algebra layer*: duplication; the sibling crate `fs-ga` already provides it.
- *Defensive-mixture importance sampling as the certified bound*: its floor is ln(1/δ)/(β·n), never below crude
  sampling on the nominal share β; kept as a lower bound for refutation.
- *Extreme-value extrapolation of failure tails*: checking a fitted tail at 10⁻⁶ takes the same
  ln(20)/10⁻⁶ ≈ 3 × 10⁶ nominal episodes that certify directly.
- *Feedback control of solver effort on wall-clock*: it breaks replay; the same world address on a loaded and
  an idle machine would execute different iteration counts.
- *Kernel two-sample and optimal-transport drift tests*: no power at twenty scalar residuals. Used instead: a
  conformal band with an exchangeability alarm.
- *A category-theoretic framework beyond five functors*: rejected under the doctrine rule that binds
  vocabulary to the theorems actually used.
- *Gaussian-process surrogates over designs*: no validated kernel exists on typed designs of varying topology.
- *A learned correction of the fast tier, or a learned step surrogate as a screen*: it hides the artifacts the
  trust ladder exists to expose.
- *Copy-on-write device state for forks* and *minimal perfect hashing for brick directories*: the wrong
  bottleneck (KI-10, KI-12).
- *Topological complexity of configuration spaces*: it changes no decision an agent makes.
- *Polyhedral loop optimization of kernels*: loops of at most 60 iterations unroll under staging.
- *Mechanism design for the job queue*: estimates come from the system, not from agents.
- *Replicated consensus, gossip membership and escrow counters for the daemon*: one daemon per host; nothing is
  replicated.
- *Stochastic superoptimization and counterexample-guided synthesis of kernels*: no sound equivalence check
  exists for floating-point code.

**The rows.**

| Id | Family | Where | I | C | E | Score | Decision |
|---|---|---|---|---|---|---|---|
| EV-P01 | Bandit allocation for the ladder or the queue | DC-02, DC-09; §28.19.4 | 3 | 3 | 3 | 3.0 | Rejected: regret is the wrong objective for identifying the best design (KI-45) |
| EV-P02 | Expected information gain as the selection rule | DC-05; §28.19.4 | 2 | 3 | 2 | 3.0 | Rejected: it buys knowledge that cannot change the choice |
| EV-P03 | A second in-house geometric-algebra layer | — | 2 | 4 | 3 | 2.7 | Rejected as duplication: `fs-ga` already provides it |
| EV-P04 | Coalition-value attribution as the primary output of an autopsy | DC-06; §27.7 | 2 | 4 | 3 | 2.7 | Kept as a budgeted summary only |
| EV-P05 | Defensive-mixture importance sampling as the certified bound | DC-04; §26.9 | 2 | 4 | 3 | 2.7 | Rejected as a certificate; kept as a lower bound for refutation (KI-37) |
| EV-P06 | Pressure-field (hydroelastic) contact | C-06 | 3 | 3 | 4 | 2.3 | Deferred [F]; none of the laws of §18.5 implements it |
| EV-P07 | Contact-mode enumeration by hyperplane arrangements | — | 2 | 3 | 3 | 2.0 | Parked; mode strata are counted only up to symmetry orbits (§13.6, C-16) |
| EV-P08 | Gradients through contact | D-8; §17.13, §28.8 | 4 | 2 | 4 | 2.0 | Deferred to M6; regime-local and return-map derivatives with validity labels arrive earlier (C-16) |
| EV-P09 | Extreme-value extrapolation of failure tails | DC-04 | 3 | 2 | 3 | 2.0 | Rejected (KI-37) |
| EV-P10 | Feedback control of solver effort on wall-clock | DC-08 | 2 | 3 | 3 | 2.0 | Rejected: it breaks replay (KI-15) |
| EV-P11 | Kernel two-sample and optimal-transport drift tests | DC-07 | 2 | 3 | 3 | 2.0 | Rejected: no power at twenty scalar residuals |
| EV-P12 | Barrier contact for articulated bodies | C-06; §18.5 | 3 | 3 | 5 | 1.8 | Left to FrankenSim's own lane; not built here |
| EV-P13 | E-graph saturation over real-arithmetic rewrites | DC-11 | 4 | 2 | 5 | 1.6 | Rejected; reopened only on the profile condition of DC-11 (KI-04) |
| EV-P14 | A category-theoretic framework beyond five functors | C-12 | 2 | 3 | 4 | 1.5 | Rejected under D-2 |
| EV-P15 | Gaussian-process surrogates over designs | DC-02, DC-05 | 3 | 2 | 4 | 1.5 | Rejected: no validated kernel on typed designs of varying topology |
| EV-P16 | Exact reuse of rollouts after a mass or geometry edit | §28.13 | 3 | 1 | 2 | 1.5 | Rejected (H-09; KI-33) |
| EV-P17 | Tensor-core products for a 256-wide policy | §21.20 | 1 | 3 | 2 | 1.5 | Rejected on the deterministic profiles (KI-02) |
| EV-P18 | Specialized kernels for tournament survivors | §17.9, §21.18; GATE-SPARK-14 | 2 | 2 | 3 | 1.3 | Parked [F] behind its profile gate: built only if the M2 profile shows a specializable share of at least one third; KILL-SPECIALIZE |
| EV-P19 | Sampling control as a stand-in evaluator for a trained policy | §21.20, §22.8 | 4 | 1 | 3 | 1.3 | Rejected (H-01; KI-26) |
| EV-P20 | Operator-theoretic (Koopman) locomotion models | — | 2 | 2 | 3 | 1.3 | Parked (KI-22) |
| EV-P21 | Path signatures as telemetry features | — | 2 | 2 | 3 | 1.3 | Parked |
| EV-P22 | Copy-on-write device state for forks | §21.20 | 1 | 4 | 3 | 1.3 | Rejected: wrong bottleneck (KI-10) |
| EV-P23 | Minimal perfect hashing or succinct structures for brick directories | §21.20 | 1 | 4 | 3 | 1.3 | Rejected: wrong bottleneck (KI-12) |
| EV-P24 | Half precision on matrix accelerators for the contact solve | §18.12, §21.20 | 3 | 1 | 3 | 1.0 | Rejected (H-15; KI-01); refused by `FROB-REF-245` |
| EV-P25 | One universal policy across topologies | §23.11 | 5 | 1 | 5 | 1.0 | Rejected at an affordable pretraining budget (H-02; KI-27) |
| EV-P26 | A learned correction of the fast tier, or a learned step surrogate as a screen | DC-01, DC-02 | 3 | 1 | 3 | 1.0 | Rejected: it hides the artifacts the trust ladder exists to expose |
| EV-P27 | Order-dependent mergeable quantile sketches | §21.20, §29.7 | 2 | 1 | 2 | 1.0 | Rejected (KI-11) |
| EV-P28 | Connection curvature as a wheel-layout test | A-09 | 2 | 1 | 2 | 1.0 | Rejected: the growth vector of A-07 is the test (KI-21) |
| EV-P29 | Live zero-copy reads of device state by the CPU | §21.20 | 1 | 2 | 2 | 1.0 | Rejected (KI-13) |
| EV-P30 | Sheaf-theoretic sensor fusion | — | 2 | 2 | 4 | 1.0 | Parked; the invariant filter and the least-squares pose graph of §20.14 cover the need |
| EV-P31 | Topological complexity of configuration spaces | — | 1 | 3 | 3 | 1.0 | Rejected: it changes no decision an agent makes |
| EV-P32 | Per-world adaptive time tiles and time-parallel integration | §21.20 | 2 | 2 | 4 | 1.0 | Rejected (KI-09) |
| EV-P33 | Polyhedral loop optimization of kernels | — | 2 | 2 | 4 | 1.0 | Rejected: loops of at most 60 iterations unroll under staging |
| EV-P34 | Mechanism design for the job queue | DC-09 | 2 | 2 | 4 | 1.0 | Rejected: estimates come from the system, not from agents |
| EV-P35 | General Lean theorems on Lie groups | T-13 | 2 | 2 | 5 | 0.8 | Deferred [M]; the modules are blocked today (§12.1) |
| EV-P36 | Joint paving of a 10- to 20-dimensional design box | §25.6 | 3 | 1 | 4 | 0.8 | Rejected (H-06; KI-35) |
| EV-P37 | Replicated consensus, gossip membership and escrow counters for the daemon | DC-09 | 1 | 3 | 4 | 0.8 | Rejected: one daemon per host; nothing is replicated |
| EV-P38 | Full-room fluid, cloth or granular matter in every training world | §19.10.1 | 3 | 1 | 5 | 0.6 | Rejected (H-16; KI-17) |
| EV-P39 | Persistent homology of configuration spaces | — | 1 | 2 | 4 | 0.5 | Rejected |
| EV-P40 | General 6R inverse kinematics by Gröbner bases | C-11 | 2 | 1 | 4 | 0.5 | Rejected at current tool limits; inside the caps, certified inverse kinematics is [F] |
| EV-P41 | Quaternionic-type irreducible blocks | §13.5 | 1 | 1 | 2 | 0.5 | Rejected: no group in scope has one; `FROB-REF-214` (KI-48) |
| EV-P42 | Stochastic superoptimization and counterexample-guided synthesis of kernels | DC-11 | 2 | 1 | 4 | 0.5 | Rejected: no sound equivalence check exists for floating-point code |
| EV-P43 | Continuation on the GPU with root certificates checked inside a proof kernel | §25.21.4 | 2 | 1 | 5 | 0.4 | Rejected (KI-47); the CPU lane is [M] |

Rows in this table: 43.

## Acceptance criteria

- [ ] 43 records `EV-P01` … `EV-P43` exist in table `parked`, in the order above, and the lints of NAB-17 pass.
- [ ] Every printed score equals the derived score; the first row scores 3.0 and the last 0.4.
- [ ] 32 rows are below 2.0; 11 are at 2.0 or above, and each of those 11 has a non-empty reason.
- [ ] Every `rejected` row has a reason or a `killed_idea`; every `parked` and `deferred` row has `reopen_on`.
- [ ] The 20 distinct `KI-nn` references of the table (KI-01, -02, -04, -09, -10, -11, -12, -13, -15, -17, -21,
      -22, -26, -27, -33, -35, -37, -45, -47, -48) resolve in the killed-idea register.
- [ ] The two refusal codes named (`FROB-REF-245` for a half-precision solve or state; `FROB-REF-214` for a
      quaternionic constituent) exist in the refusal registry with those meanings.

## Test plan

### Unit tests

`crates/frob-conformance/tests/ev_matrix.rs`, module `parked`:

- `count_is_43`; `order_matches_frozen_list`.
- `scores`: spot values: bandit allocation (3, 3, 3) → "3.0"; a second geometric-algebra layer (2, 4, 3) →
  "2.7"; pressure-field contact (3, 3, 4) → "2.3"; barrier contact (3, 3, 5) → "1.8"; e-graph saturation
  (4, 2, 5) → "1.6"; one universal policy (5, 1, 5) → "1.0"; continuation on the GPU (2, 1, 5) → "0.4".
- `eleven_rows_at_or_above_two_all_have_reasons`.
- `decision_vocabulary`: 33 rejected, 5 parked, 3 deferred, 1 `kept-as-summary`, 1 `left-to-sibling`; total 43.
- `killed_idea_references_resolve`.
- `rejected_rows_have_reason`: each of the 33 rejected rows has a non-empty `reason` or a `killed_idea`; the
  three rows without a colon (EV-P14, EV-P39, EV-P40) carry the `reason` of the transcription rule.
- `deferred_lean_row_lists_mq21`: EV-P35 has `queue = ["MQ-21"]`, and no other row of this table has a queue.
- `parked_and_deferred_have_reopen_on`.
- `decision_text_frozen` against `tests/fixtures/ev/parked.tsv`.

### End-to-end script and logging

`scripts/e2e/decision_records.sh` (NAB-50) asserts the count (43), zero mismatches, the 32/11 split and the
resolution of every `KI-nn`, and runs the revive lint's planted case against one row of this table (a work
package that names "bandit allocation for the ladder" without citing its record must be refused). Logged per
STD-E2E.

### Known answers and negative controls

Known answers: 43 rows (33 rejected, 5 parked, 3 deferred, 2 other); 32 below 2.0; scores from 3.0 to 0.4; the arithmetic quoted in reasons: ln(20)/10⁻⁶ =
2.996 × 10⁶; 1/(1 − 0.13) = 1.15 for the tensor-core row. The only kill gate in this table is KILL-SPECIALIZE
(decided at M4, on a share measured from the M2 profile). Negative controls: a rejected row with an empty
reason (`ev.rejected-without-reason`); a parked row with no `reopen_on` (`ev.parked-without-reopen`); a row
copied into table `frontier-moonshot` without an ADR (the frozen-table comparison fails).

## Considerations

- This table is the first place a new contributor's good idea has probably already been. The revive lint
  (NAB-32) makes citing the record a condition of proposing it again; that is a courtesy to the idea as much as
  a guard.
- A row leaves this table only by an ADR that names the new evidence, and the move is a history entry, never a
  deletion.

## References

Appendix C.1.3; §28.19.4 (methods rejected as ornamental or premature); doctrine rule D-2.
