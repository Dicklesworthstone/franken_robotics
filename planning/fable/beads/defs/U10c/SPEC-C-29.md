---
key: "SPEC-C-29"
title: "spec C-29 core: a deterministic forkable simulator as a structural causal model: abduction, action, prediction"
type: "task"
priority: 1
milestone: "M2"
labels: ["m2", "lemma", "spine", "spec", "solid", "autopsy", "frob-chronicle", "frob-autopsy", "test"]
parent: "WP-51"
deps: ["N10a-01", "STD-TEST", "STD-E2E"]
needs: []
related: ["BD-203", "BD-205", "BD-279", "BD-283", "N10c-01", "SPEC-C-28", "N10-CX", "N10c-10", "BD-090", "N10a-02"]
gates: ["GATE-AUTOPSY-01", "GATE-AUTOPSY-02"]
plan_refs: ["§10 C-29", "§27.3", "§27.4", "§27.5", "§29.8", "§38.7 MR-08", "Appendix I.12", "Appendix M.9", "Appendix N.7"]
---
## Background

Construction C-29 of the FrankenRobotics spine says: with bitwise replay, the simulator is a structural causal
model, and the three steps of a counterfactual are exact operations on it. It is tier Solid [S], owned by the
Autopsy subsystem (counterfactual debugging), crate `frob-autopsy`; the fork mechanism it stands on lives in
`frob-chronicle`. It replaces earliest-anomaly heuristics and attribution by reading logs. The payoff is that
"which repair edit ranks first?" is answered by experiment, with a stability figure beside each attribution.

This bead is the core of the specification of C-29: the causal-model reading and the laws of forks. The
attribution statistics built on it (the placebo flip rate, the probability of necessity, the labels and the Shapley
summary) are the second part, bead N10c-01, which depends on this one. The split follows the milestones: the null
fork is measured at M2 (GATE-AUTOPSY-01), attribution at M3.

Terms. A *structural causal model* has exogenous variables, endogenous variables and one mechanism per endogenous
variable. Here the exogenous variables are the keyed random streams (counter-based streams addressed by logical
identity: run seed, world, episode, tick, stream name), the endogenous variables are the simulation state, and the
mechanisms are the model's declared components. A *snapshot* covers world, sensor, estimator, policy, solver and
stream state. A *fork* is a parent episode, a fork tick and a change set; it never mutates its parent. *Bitwise
replay* is determinism class D2 (bitwise per backend and independent of the batch). Proof classes: Proved (sealed
Lean capsule), Certified (instance certificate re-checked by the offline auditor in exact arithmetic), Tested
(property or known-answer test with a negative control), Estimated (depends on a card, a fit or a sample). Status
of every statement today: HYPOTHESIS.

Design reasoning. FACT (FrankenSim `483d09c682f2`, observed 2026-11-21): `fs-rand` provides keyed streams, and no
sibling has an atomic full-world snapshot, so snapshots are this project's. Because streams are addressed and not
consumed, a continuation after an intervention reads exactly the draws the parent read: that is what makes
"prediction" exact. Rejected alternatives: forking by re-seeding (a fork without the parent's streams answers a
question about a different world); forking with shared streams for replicates (children that share the parent's
address are identical copies and add nothing).

## Scope

In: `docs/math/C-29.md` sections on the causal model, the three steps, fork semantics, intervention typing and the
snapshot-versus-episode-start rule; the law tests for those in `crates/frob-chronicle/tests/laws_c29.rs`; registry
entries for KA-145 and the control seeded from CX-071; the metamorphic relation MR-08.

Out: the fork mechanism and its five-way classification (BD-205); in-kernel digests and snapshots (BD-203);
interventions and the placebo fork as a verb (`autopsy.intervene`, BD-279); design-parameter re-runs (`autopsy.rerun`,
BD-283); every attribution statistic (N10c-01); recording GATE-AUTOPSY-01 and -02 (BD-205).

Register row (the shared rule of N10a-01). The construction register
`frob-conformance/registry/constructions.toml` is created by N10a-01 with forty typed rows whose obligation fields
are empty; this bead fills row C-29: `rust_type` (the subject trait of the law suite below, replaced by the
production type when the implementing bead binds it), `verb` (`run.fork`; also `autopsy.intervene`,
`autopsy.attribute`, `autopsy.rerun`), `failing_test` (a test of `frob-chronicle::laws_c29` with its registered
control), `payoff.gate` (GATE-AUTOPSY-01), `statements`, `known_answers` (KA-145 to KA-148), `cards` (none),
`gates`, `pilot` left empty (the construction is Solid) and `generic_comparator` only if this bead claims an
advantage over a named generic method. Doctrine D-2 (no ornamental mathematics) is checked on these fields: a
construction with no verb or no failing test is moved to the parked register by a decision record. The part bead
N10c-01 adds their statements and tests to the same row. The contract `docs/math/C-29.md` has the ten sections of
the template `docs/math/TEMPLATE.md` of the same bead, in the template's order (object and definitions; what it
replaces and the payoff; conventions; statements; formulas; fixtures and known answers; failure cases, refusals
and retirement; non-claims; foundations; implementation map); content that this bead lists under other headings
goes into the template section that matches. Statement identifiers are entered in the contract and in the register
with the construction prefix (`C-29.S1` and so on, as the template requires); the tables of this bead and of its
part beads abbreviate them.

Order of work (DESIGN; the specification-first rule stated in section 6 of the construction register bead
N10a-01). This bead is written before the code it specifies and does not depend on BD-090 (the ledger schema of
`frob-chronicle`): that bead is listed in `related` and is not a prerequisite. The law suite is generic over its
subject trait and closes on the exact reference model and the mutants in its own test support. If `frob-chronicle`
does not exist when this bead starts, the suite and its support files are written under
`crates/frob-conformance/tests/` with the same file names (that crate exists from BD-025) and the commands below
read `-p frob-conformance`; the bead that implements the construction moves them into the owning crate, binds its
production types to the subject trait and runs the unchanged suite as part of its own acceptance. A criterion or
script step below that names production code or a verb of an implementing bead is owed at that bead's acceptance
and is logged `pending` here until then.

## Technical approach

### 1. The model and the three steps (contract sections 1 and 2)

| Id | Statement | Class |
|---|---|---|
| S1 | Under bitwise replay the simulator is a structural causal model whose exogenous variables are the keyed random streams and whose mechanisms are the model's components. | Structural reading; its consequences S2 to S8 are Tested |
| S2 | *Abduction* is replay of the recorded world to a snapshot, bit for bit: the digest of the replayed state at the snapshot tick equals the stored digest. | Tested (digest equality, determinism tier G5) |
| S3 | *Action* is a typed intervention do(X := x) that changes one declared mechanism and is recorded as a model difference: torque limit off, friction set to nominal, estimator replaced by privileged state. A change that would alter two mechanisms is split into two interventions or refused. | Tested |
| S4 | *Prediction* is continuation on the same streams: every draw a fork reads at tick t under stream name s equals the draw the parent read at (t, s). | Tested (exact) |
| S5 | The null fork (the empty intervention) reproduces the parent bit for bit on the originating profile. If it does not, the record is unsound and nothing is attributed. | Tested (digest equality) |
| S6 | A fork never mutates its parent, and a world forked alone and in a batch with other children gives identical digests. | Tested |
| S7 | A child given its own address (the parent's address extended by a fork path) has fresh streams; children that share the parent's streams are identical copies. | Tested |
| S8 | An intervention at a snapshot answers a question about the continuation only. For a design parameter (a gear ratio, a mass) the thermal state, the estimator state and the policy's history at the snapshot would all differ under the changed design, so the intervention is re-run from the episode start on the same streams. | Tested (counter-fixture) |
| S9 | The policy's reaction is part of the effect. A fork is refused when the controller cannot adapt without retraining (for example when the change alters its observation or action space). | Tested (refusal) |
| S10 | Attribution runs in double precision on a CPU tier. A successful counterfactual is not a minimal repair, and a local rescue can disagree with the global outcome. | Non-claims, recorded |

### 2. Types the contract fixes (DESIGN sketch, carried from the type appendix)

```rust
pub struct Intervention { factor: FactorPath, value: Value, mechanism: MechanismId, model_diff: Blake3 }
pub enum ForkClass { Continuation, Exogenous, ContactAffecting, Information, Structural }
pub struct Fork { parent: EpisodeId, tick: Tick, changes: Vec<Intervention>, class: ForkClass,
                  earliest_valid_tick: Tick, forced_by: Vec<FieldPath>, addr: WorldAddr, null_replay: NullReplay }
```

`Continuation` covers the null fork, a placebo, replay and teleport. `Exogenous` restarts at the fork tick with
nothing regenerated. `ContactAffecting` restarts at the fork tick with contact memory discarded and the first solve
cold, and is labelled. `Information` restarts at the fork tick with packet queues and filter state re-initialized.
`Structural` (design parameters included) restarts from the authored initial condition, settled again:
`earliest_valid_tick` is the episode start. `NullReplay` is `Bitwise`, or the record is unsound.

### 3. Conventions, failure cases and non-claims

- A factor is a path into the world or the model whose mechanism is declared: a world edit class, a typed design
  edit, or a diagnostic factor (`torque_limit := off`, `estimator := privileged`, `latency := 0`).
- Refusals this construction relies on (codes owned by the Autopsy chapter): FROB-REF-583 when the null fork does
  not replay; FROB-REF-586 when one requested change alters two mechanisms.
- Not claimed: that an intervention identifies a cause (that needs the statistics of N10c-01); anything on a
  single-precision tier beyond what the Autopsy chapter admits for short windows; anything about a physical robot.

### 4. Law-test layout (DESIGN)

`crates/frob-chronicle/tests/laws_c29.rs` with `tests/support/c29_ref.rs`. The reference model is a *synthetic*
structural causal model with integer state so every comparison is exact: three mechanisms (`drive` with gain θ₁,
`friction` with coefficient θ₂, `limiter` clipping at L), one slow state h with h₊ = h + θ₁²·|u| (a stand-in for a
thermal accumulator), a counter-based stream u(run, world, episode, tick, name) from a fixed integer hash, a digest
chain over committed states, and failure defined as x_T above a threshold. The suite is generic over a trait
`C29Subject` (make a snapshot, fork with a change set, continue, report digests and draws) so the production fork
mechanism of BD-205 binds to the same laws. Mutant subjects in `tests/support/c29_mutants.rs` are the negative
controls; `controls_are_caught` asserts each fails its named law.

### 5. Registry entries

KA-145 in the known-answer registry, with `docs/derivations/` file and the control below (the row was printed with
no control). Relation MR-08 in `frob-conformance/registry/relations.toml`: the empty intervention replays bit for
bit; a setpoint change at tick t reuses the prefix to the snapshot before t; a child with its own address has fresh
streams. Controls seeded from CX-071 and from MR-08's control (a checkpoint with its packet queue stripped; a fork
that shares streams).

### 6. Implementers and gates

Implementers: `frob-chronicle` (snapshots, forks, classification: BD-203, BD-205) and `frob-autopsy` (interventions,
BD-279; re-runs from the episode start, BD-283). Gates that measure this part: GATE-AUTOPSY-01 at M2 (null forks of
1,000 failing worlds replay with identical digests on the originating backend; a world forked alone and in a batch
of 512 children gives identical digests) and GATE-AUTOPSY-02 at M2 (40 catalogued pairs of intervention and state
classified correctly; the warm-start graft and a mid-episode gear change refused with the right fields). The
thermal counterexample of GATE-AUTOPSY-06 (M3) measures S8 on a real model. This bead records none of them.

## Acceptance criteria

- [ ] `docs/math/C-29.md` exists; its part A carries S1 to S10, the fork classes, the refusals and the non-claims,
      and names the FACT about `fs-rand` and the missing sibling snapshot with pin and date.
- [ ] `cargo test -p frob-chronicle --test laws_c29` passes on the reference model, and on the production fork
      mechanism if BD-205 has landed.
- [ ] Each mutant listed below fails its named law in `controls_are_caught`.
- [ ] KA-145 is registered with a derivation file, tolerance `exact (digest)` and a control.
- [ ] MR-08 and the control seeded from CX-071 are registered and attached to GATE-AUTOPSY-01 and GATE-AUTOPSY-02.
- [ ] `scripts/e2e/spec_c29.sh` exits 0.
- [ ] Row C-29 of `frob-conformance/registry/constructions.toml` has its obligation fields filled as the
      Scope states, and `cargo test -p frob-conformance --test g0_doctrine -- d2` reports no failed
      obligation for the row (fields not yet due are reported `pending` under the grace rule of N10a-01).

## Test plan

### Unit tests

- `abduction_is_replay`: replay from tick 0 to snapshot ticks 1, 17 and T − 1 reproduces each stored digest.
- `ka_145_null_fork_replays_bitwise`: the empty intervention at each of 64 seeded fork ticks gives a digest chain
  equal to the parent's from that tick to the end.
- `prediction_reads_parent_draws`: after do(limiter := off) at tick k, every draw read at t ≥ k equals the
  parent's draw at the same (t, name).
- `intervention_changes_one_mechanism`: the model difference of do(limiter := off) touches the limiter only; a
  requested change that touches friction and drive together is refused.
- `fork_never_mutates_parent`: the parent's digest chain is byte-identical before and after 100 forks.
- `fork_batch_invariance`: one child run alone and among 511 siblings has identical digests.
- `child_address_gives_fresh_streams`: two replicates with distinct fork paths read different draws; two children
  forced to the parent's address are detected as duplicates.
- `design_parameter_reruns_from_start`: doubling θ₁ at snapshot k leaves the slow state h at its parent value,
  while the re-run from the episode start has a different h at k and a different end state; the mid-episode request
  is classified `Structural` with `earliest_valid_tick` equal to the episode start, and is refused at tick k.
- `refuse_when_controller_cannot_adapt`: an intervention that changes the observation width is refused.

### End-to-end script and logging

`scripts/e2e/spec_c29.sh`: `frob --json doctor`; `cargo test -p frob-chronicle --test laws_c29`; `cargo test -p
frob-conformance --test g5 -- KA-145`; `cargo test -p frob-conformance --test g3 -- MR-08`; file and registry
checks. It sources the STD-E2E helper and writes `target/e2e-logs/spec_c29/<utc>.ndjson` and a transcript; every
line has `ts`, `script`, `bead`, `step`, `event`, the command line, exit code, wall time, the evidence block
(claim Known answer, class Tested, tier, determinism D0 for the integer model, seed), the digests compared, and on
failure the first differing tick and digest with the fixture path. The `summary` line counts assertions and names
GATE-AUTOPSY-01 and GATE-AUTOPSY-02.

### Known answers and negative controls

| Row | Truth | Tolerance | Tier | Control |
|---|---|---|---|---|
| KA-145 | The empty intervention replays the recorded failure bit for bit | exact (digest) | G5 referee (bound by BD-205) | Mutant `reseed_on_fork` (new): the fork restores state and re-seeds the streams |

Mutants: `reseed_on_fork`; `snapshot_without_queue` (the snapshot omits one state component, standing for a
checkpoint with its packet queue stripped); `shared_streams_for_replicates`; `snapshot_fork_for_design_parameter`
(accepts the mid-episode design change of CX-071); `two_mechanism_change_accepted`; `parent_mutated`.
Counterexample carried: CX-071 (a design-parameter change cannot be tested by forking at a snapshot just before the
failure). Determinism lanes: twice on one machine; debug and release; the production binding repeats the digest
laws on each backend against its class.

## Considerations

- If the null fork does not replay on some backend, nothing downstream may attribute: the law S5 is the gate on
  every later use, and the fallback named by KILL-AUTOPSY (single-factor paired re-runs from the episode start)
  still needs S2 and S4.
- Snapshot size and the reservoir on each device are an open question owned by the Chronicle lead
  (OPEN-AUTOPSY-4); its fail-closed default is that every fork point not in the reservoir is reconstructed by
  replay. The usable fork horizon depends on Lyapunov exponents of our own controllers (OPEN-AUTOPSY-3, default
  leads of at most 2 s, blocking nothing).
- The type appendix nests the three intervention kinds of the Autopsy chapter inside the middle class of the
  Chronicle chapter's three-way list; the five names above are the contract, and the mapping is recorded in the
  contract file so the two chapters cannot drift.
- The reference model is integer-valued by design; floating-point behaviour is the production binding's burden.

## References

Plan §10 C-29; §27.3 to §27.5; §29.8; Appendix I.12; MR-08; KA-145; CX-071; GATE-AUTOPSY-01, -02, -06; INV-AUTOPSY-01,
INV-AUTOPSY-02; KILL-AUTOPSY; BET-23; RISK-50; C-15, C-27, C-28.
