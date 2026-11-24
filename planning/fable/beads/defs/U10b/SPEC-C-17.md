---
key: "SPEC-C-17"
title: "spec C-17 (frob-mission): a mission as a quantified set of designs: F, inner and outer sets, semantic rules"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "lemma", "spine", "spec", "frontier", "frob-mission", "frob-conformance", "test"]
parent: "WP-42"
deps: ["BD-022", "BD-025", "N10-CX", "N10-KA2", "N10a-01", "N10b-08", "STD-E2E", "STD-TEST"]
needs: []
related: ["SPEC-C-19", "SPEC-C-30", "SPEC-C-32"]
gates: ["GATE-GAUNT-02", "GATE-GAUNT-01"]
plan_refs: ["§10 C-17", "§9 D-12", "§9 D-13", "§9 D-17", "§9 D-18", "§12.2", "§25.3", "§25.4", "§25.5", "Appendix D.6", "Appendix I.3", "Appendix M.9 KA-098 to KA-100", "Appendix N.7 CX-045 to CX-047"]
---
## Background

The usual way to evaluate robot designs is to score one design at a time and read a reward, and to report a
search that found nothing as evidence that nothing exists. Construction C-17 replaces this with a **mission
query**: a question with explicit quantifiers whose answer is a *set* of designs, bracketed from inside and
outside. One query returns a family, so an agent chooses among wheels, legs, transmissions and sensors in one
turn (`mission.synthesize`), learns that a policy search cannot succeed before paying for it (`mission.refute`),
and receives the smallest change of body, sensor or world that relaxes the binding constraint. Unresolved
cells direct the next experiment instead of hiding inside an average.

Tier: Frontier [F]. Owning chapter: Augur; crate `frob-mission`; certificates are checked by the independent
auditor (`frob-cert`, binary `frob-verify`). It serves BET-19 (mission queries) and doctrine rules D-12
(quantifiers are part of the claim), D-13 (answers are two-sided and unknown is a verdict), D-17 (a specialized
method names the generic method it must beat) and D-18 (a strict decision is computed exactly).

This bead specifies the definition and its semantic rules. The four proof rules that build an answer are
specified in N10b-04; the pilot is N10b-05.

Terms. *Design domain* D: a disjoint union of parameter spaces of typed assembly graphs. *World*: geometry,
physics, hidden parameters and initial-state uncertainty. *Policy*: a controller that maps observation history
to commands. *Execution*: one admitted run of a design with a policy in a world. *Cell*: a box of designs in
one topology and chart. *Witness*: a controller with a certificate that it works on a whole cell.
*Refutation*: a proof that a cell contains no member of the answer. *Chance constraint*: a requirement on the
probability of an event over a named population.

Rejected alternatives: scoring single designs (no family, no impossibility result); intersecting per-world
answers (wrong quantifier order, shown below); treating a failed training run as an exclusion (it refutes one
controller, not all); a convex hull through feasible samples (the feasible set is not convex; N10b-04).

## Scope

In: `docs/math/C-17.md` (statements S1 to S10; the proof-rule section comes from N10b-04); the law suite
`crates/frob-conformance/tests/laws_c17.rs` with reference model and mutants; registry rows KA-098, KA-099,
KA-100; controls from CX-045, CX-046, CX-047; row C-17 of the construction register.

Out: the four proof rules and the non-convexity fixture (N10b-04); polarity and revalidation (SPEC-C-18); the
observation channel and causal policies (SPEC-C-19); the requirement language of the acceptance predicate
(SPEC-C-30); the `.fmission` schema, the engine and the verbs (BD-126, BD-128, BD-135); bundles and checkers
(BD-132, BD-133).

Home of the law suite (DESIGN, with reason): `frob-conformance`, because `frob-mission` is created by the bead
that implements this construction and that bead depends on this specification.

## Technical approach

### 1. The object (contract section 1)

A mission query fixes:

- a design domain D, quotiented only by admitted interface-preserving isomorphisms;
- a nonempty world set W, **declared before any candidate is tested**;
- an observation channel Y with its clock contract (construction C-19);
- the class Π_Y of causal policies on the history of Y;
- the set T(d, π, w) of maximal admitted executions of design d under policy π in world w;
- a Boolean acceptance predicate A on executions (construction C-30).

The robust design family is

```text
F = { d ∈ D : ∃π ∈ Π_Y  ∀w ∈ W,  T(d, π, w) ≠ ∅  ∧  ∀τ ∈ T(d, π, w), A(τ) }
```

**The public answer** is a triple I ⊆ F ⊆ O with topology identifiers, coordinate charts, units, parameter
boxes and assumptions. Each cell of I carries a witness (a controller and a certificate that it works on the
whole cell). Each cell removed from O carries a refutation. Each cell of O ∖ I is unresolved and carries its
competing explanations and the next test that would split it. Empty I means no certified witness so far; empty
O excludes the family under the declared model.

### 2. Statements (status HYPOTHESIS; class once the route has run)

| Id | Statement | Class and route |
|---|---|---|
| C-17.S1 | The definition of F with the quantifier order ∃π ∀w ∀τ; the quantifier prefix is part of the query's identity | `Tested` (type and admission); a changed prefix is rejected by the auditor |
| C-17.S2 | I ⊆ F ⊆ O for every answer; every inner cell has a witness, every excluded cell a refutation, every other cell is unresolved with its reason | `Certified` cell by cell (each certificate checked by the auditor); `Tested` as an invariant of answers |
| C-17.S3 | Progress: an execution must progress to the horizon or to a declared terminal outcome. Deadlock, Zeno behaviour and undefined dynamics cannot earn success | `Tested` |
| C-17.S4 | Solver failure: a numerical solver failure leaves the verdict `unknown` unless independent evidence establishes physical failure or impossibility | `Tested` |
| C-17.S5 | Comparators: strict, non-strict and equality atoms are evaluated with their declared comparators. A uniform positive margin is a separate, stronger request | `Certified` where evaluated exactly; `Tested` |
| C-17.S6 | Set-valued contact: a stick or slip fork with any failing admissible branch invalidates a universal success certificate | `Tested` |
| C-17.S7 | Policy indexing: a design-dependent policy is indexed by the known nominal build. Hidden as-built tolerances stay in the world until a calibration observation reveals them; otherwise the answer certifies a choice nobody can make | `Tested` (admission) |
| C-17.S8 | Refuting a controller is not excluding a design: a rollout can refute a particular controller; excluding a design under ∃π needs an argument covering every admitted controller, and a training run that failed does not supply one | `Tested` (type: an exclusion needs a proof kind that quantifies over controllers) |
| C-17.S9 | Chance constraints: a chance constraint names its population, its event and its confidence level, and preserves the joint law across stages or allocates failure budgets that sum to the allowance | `Certified` on finite populations by enumeration; `Estimated` otherwise |
| C-17.S10 | Evidence for an easier quantifier order (one policy per world, ∀w ∃π) is rejected by type for a query that asks ∃π ∀w | `Tested`; refusal FROB-REF-826 (the quantifier prefix, domain or assumption set of a certificate does not match the query it is offered for; exit code 3) |

### 3. Type sketch (DESIGN: fixes intent, not signatures)

```rust
pub struct MissionQuery {
    design_domain: DesignDomain, world: WorldFamily /* nonempty */, observation: Channel,
    policy_class: PolicyClass, horizon: Horizon, predicates: Predicates /* atoms carry comparator and units */,
    quantifiers: Vec<Binder> /* ordered; part of the identity */, objective: Option<Objective>,
    evidence: EvidenceProfile, resolution: Resolution, response: Vec<ResponsePart>, budget: Budget,
}
pub enum Binder { Exists { var: Var, scope: PolicyScope }, ForAll { var: Var },
                  Prob { var: Var, population: PopulationId, at_least: Ratio, confidence: Ratio } }
pub struct Cell { topology: TopologyId, chart: ChartId, bounds: Vec<Bound> /* closed or open per end */,
                  units: Vec<Unit>, assumptions: AssumptionSet, policy_scope: PolicyScope }
pub enum Verdict { Inner(Vec<ProofRef>), Excluded(Vec<ProofRef>), Unresolved(Vec<Explanation>) }
pub enum Explanation { StraddlesThreshold, ModeSequenceUnenclosed, SolverFailure, BudgetExhausted,
                       CompetingMechanisms(Vec<MechanismRef>), NeedsMeasurement(MeasurementRef) }
```

The outer set is the domain minus the excluded cells: inner ∪ unresolved. An answer is immutable; a later query
with more budget produces a new answer that refines it.

### 4. Worked fixtures (all exact)

- **Quantifier order (KA-098).** The docking fixture (unit mass, hidden gain g ∈ {1, 2}, three ticks; reference
  model of N10b-08) with the packet arriving late. With first command p, world g needs the remaining commands
  (1/g − 2p, p − 1/g), which differ by ½ between the gains. Each world alone has a successful open-loop
  command, for example (¼, ½, −¾) for g = 1 and (¼, 0, −¼) for g = 2, and no single command succeeds in both.
  So ∀w ∃π holds and ∃π ∀w fails; evidence consisting of one policy per world must be rejected.
- **Strictness (KA-099).** At U = ¾ and ε = 5/16 the docking residual (U + ½)/4 − ε is exactly zero and the
  strict predicate fails. On 0 < w ≤ 1 every realization has w > 0 although the infimum is 0: a strict
  requirement on each realization holds, a uniform positive margin does not.
- **Chance constraints (KA-100).** Among 20 equally likely worlds, two stages each pass in 19. The mission
  passes in 18 worlds if the failing worlds differ and in 19 if they coincide. Multiplying stage rates gives
  361/400, which is neither. Failure budgets that sum (1/20 + 1/20) guarantee 18/20; only conditional success
  at every reached history justifies a product bound.

### 5. Honest limits, non-claims, foundations

- Constructive inclusion over a contact-rich cell needs a sound reachable tube; the planning pilots that exist
  are scalar or static.
- On small exactly solvable plants a generic solver answers the same query: FACT (planning pilots, to be re-run
  in this repository) a lifted linear program tied the specialized projection, and a satisfiability solver
  derived the family answer of a pusher plant from the raw equations. The contribution claimed is the mapping
  from a causal mission and a morphology to sound small sub-problems, and the cross-check back; no speed is
  claimed for it.
- A family certificate is about the declared model, world set and policy class; its class is capped by its
  inputs (an exact argument over an `Estimated` material card is `Estimated`). It says nothing about a physical
  robot; every exported answer carries the three no-authorization flags, all false.
- Foundations (FACT, observed 2026-11-21 at FrankenSim `483d09c682f2`): `fs-sos` (polynomial models,
  sum-of-squares programs, interval verification), `fs-regime` (dimensionless groups, scaling maps, model
  admission), `fs-voi`, `fs-eproc`, `fs-ivl` and `fs-checker` exist; none knows what a mission is. Not admitted
  for this use until their admission gates pass.

### 6. Law suite and registry (DESIGN)

- `crates/frob-conformance/tests/laws_c17.rs`, generic over `C17Subject { fn answer(&self, q: &FiniteQuery) ->
  FiniteAnswer; fn accept_evidence(&self, q, ev: &Evidence) -> Verdict }`. `tests/support/c17_ref.rs`: finite
  queries decided by exhaustive enumeration in exact rationals (designs, worlds, policies as explicit finite
  sets; the docking worlds come from `docking_ref.rs`). `tests/support/c17_mutants.rs`: mutants under the umbrella feature
  `gauntlet-mutations` (BD-025; no feature is named `controls`).
- Register row C-17: `verb = "mission.synthesize"` (with `mission.refute`), `failing_test` =
  `frob-conformance::laws_c17::s10_per_world_evidence_rejected`, `payoff.gate = "GATE-AUGUR-03"`,
  `known_answers = KA-098 … KA-102`, `pilot = "PILOT-C-17"`, `generic_comparator = "a lifted robust linear
  program; a scenario tree; a satisfiability solver"`.
- Gates that measure the construction (recorded by mechanism beads): GATE-AUGUR-01 (admission and checker
  reject 100% of a mutation corpus of at least 60 mutated queries and certificates), GATE-AUGUR-02 to -04 (the
  stance, docking and pusher families through the public verbs), GATE-AUGUR-05 and -06 (every `Certified` cell
  accepted by the auditor on a second host; checker independence), GATE-AUGUR-07 (a fresh agent names the
  binding hardware restriction and never reports an unresolved cell as a verdict), GATE-LEMMA-04 (zero accepted
  false certificates). Kill gates: KILL-FAMILYCERT, KILL-AUGUR-TIE, KILL-AUGUR-FALSECERT (N10b-05 states them).

## Acceptance criteria

- [ ] `docs/math/C-17.md` has the ten sections, the definition of F verbatim, the answer triple, statements
      C-17.S1 to S10 with classes and the three worked fixtures.
- [ ] `cargo test -p frob-conformance --test laws_c17` passes; with `--features gauntlet-mutations` every mutant fails.
- [ ] KA-098: on the late-packet docking query the reference returns F = ∅ for the joint query and a nonempty
      answer for each single-world query; evidence made of one policy per world is rejected with FROB-REF-826.
- [ ] KA-099: the cell (U, ε) = (¾, 5/16) is excluded under the strict predicate and admitted by the
      non-strict mutant; the infimum fixture returns "strict holds, margin fails".
- [ ] KA-100: 18 and 19 passing worlds for disjoint and coincident failures; the product mutant returns 361/400
      and is rejected.
- [ ] Every answer of the reference satisfies I ⊆ F ⊆ O on 500 seeded finite queries.
- [ ] `scripts/e2e/spec_c17.sh` exits 0.

## Test plan

### Unit tests

- `s1_definition_by_enumeration`: 500 seeded finite queries (at most 4 designs, 4 worlds, 8 policies, Boolean
  outcome tables): F computed by the formula equals the brute-force set.
- `s2_sandwich`: on the same queries I ⊆ F ⊆ O; with budget cut to zero, I = ∅ and O = D (unknown is a verdict).
- `s3_deadlock_is_not_success`: an execution table with an empty T(d, π, w) excludes d from F; an execution
  marked as stopping short of the horizon without a declared terminal outcome (deadlock), as Zeno, or as having
  undefined dynamics fails A and cannot make d robust.
- `s4_solver_failure_is_unknown`: a cell whose evaluation returns `SolverFailure` is `Unresolved`, never
  `Excluded` and never `Inner`.
- `s5_strict_boundary`, `s5_infimum_not_attained`: the KA-099 values.
- `s6_fork_branch`: a world with a stick/slip fork whose slip branch fails invalidates the inner verdict.
- `s7_policy_indexed_by_nominal_build`: a finite query whose worlds carry two as-built values of a tolerance:
  a policy table indexed by the as-built value with no calibration observation in the channel is rejected at
  admission; the same table is admitted once a calibration observation that reveals the value arrives before
  the decision; a table indexed by the nominal build alone is admitted.
- `s8_failed_training_is_not_exclusion`: evidence "policy π₁ failed" leaves d in O.
- `s9_chance_joint_law`: the KA-100 values; budgets 1/20 + 1/20 certify 18/20.
- `s10_per_world_evidence_rejected`: the KA-098 values.
- `controls_are_caught`.

### End-to-end script and logging

`scripts/e2e/spec_c17.sh` (idempotent; seed 1700 printed): `frob --json doctor`; the law suite with and without
`--features gauntlet-mutations`; `FROB_TEST_SEED=1700 cargo test -p frob-conformance --test g1_known_answers -- KA-098
KA-099 KA-100` (the tier target of the rows' G1 tier; `g1` is not a target name); existence of
`docs/math/C-17.md` and the derivation files. Sources the STD-E2E helper; writes
`target/e2e-logs/spec_c17/<utc>.ndjson` and a transcript; each line has `ts`, `script`, `bead` (`SPEC-C-17`),
`step`, `event` (`start`, `cmd`, `assert`, `metric`, `refusal`, `end`), command line, exit code, wall time,
evidence block (claim Known answer, class, tier G1 host, determinism D0, seed), measured and expected sets or
counts, a `refusal` line for each expected FROB-REF-826, and on failure the first query whose answer differs
from enumeration with both sets. Final `summary`: assertion counts and GATE-GAUNT-02.

### Known answers and negative controls

| Row | Truth | Tolerance | Tier | Control |
|---|---|---|---|---|
| KA-098 | Late-packet docking: each world alone has a successful open-loop command and no single command succeeds in both; one policy per world is rejected | exact (verdict) | G1 · host here; G1 · auditor in BD-133 | CX-045: one policy per world offered as evidence for ∃π ∀w |
| KA-099 | Residual exactly zero at U = ¾, ε = 5/16 and the strict predicate fails; infimum 0 with every w > 0 | exact | G1 · host | CX-046: a non-strict comparator, or evaluation by an infimum |
| KA-100 | 18 worlds if the failing worlds differ, 19 if they coincide | exact | G1 · host | CX-047: multiplying the stage rates |

Further controls (mutants): a subject that returns `Excluded` on solver failure; one that intersects per-world
answers (its F is nonempty on KA-098); one that admits an empty world set (every design becomes robust
vacuously). Metamorphic relations: adding a world never enlarges F; adding a policy never shrinks it; refining a
cell only resolves cells and never changes a resolved verdict. Determinism: exact arithmetic, class D0; twice
on one machine, debug and release, identical output.

## Considerations

- Kill gates named in advance: KILL-FAMILYCERT (decided at M3: at the partial ten-fold gate no family
  certificate has decided a cell that the generic comparator leaves undecided at equal cost; fallback: answers
  come from the generic solvers alone and still ship in the bundle format); KILL-AUGUR-FALSECERT (standing from
  M1: one certificate accepted by the auditor is shown false within its scope).
- A false certificate is worse than an ordinary wrong answer; the law suite's first duty is the rejection
  cases (S4, S8, S10), not the happy path.
- Open question carried: the exit code of a query that stops on budget with certified cells; the fail-closed
  default is exit 2 with `resolution-unmet`, returning the certified cells and the unresolved remainder, each
  cell complete in itself.
- The reference model is exhaustive and finite; it is an oracle for semantics, not a solver.

## References

Plan §10 C-17; §9 D-12, D-13, D-17, D-18; §12.2 (family certificates); §25.3 to §25.5; Appendix D.6 (feasible
set, inner and outer sandwich); Appendix I.3; KA-098 to KA-100; CX-045 to CX-047, CX-010, CX-015; BET-19;
GATE-AUGUR-01 to -07; GATE-LEMMA-04; KILL-FAMILYCERT.
