---
key: "SPEC-C-28"
title: "spec C-28: minimal counterexample worlds as 1-minimal failing order ideals: contract, law tests, registry"
type: "task"
priority: 2
milestone: "M3"
labels: ["m3", "lemma", "spine", "spec", "frontier", "nemesis", "frob-nemesis", "test"]
parent: "WP-55"
deps: ["BD-270", "BD-052", "STD-TEST", "STD-E2E"]
needs: []
related: ["BD-271", "BD-272", "BD-274", "BD-275", "BD-279", "BD-293", "BD-369", "N10-CX", "SPEC-C-29", "SPEC-C-33", "N10c-10"]
gates: ["GATE-NEMESIS-03", "GATE-NEMESIS-01"]
plan_refs: ["§10 C-28", "§26.7", "§12.3", "Appendix D.6", "Appendix I.11", "Appendix M.9", "Appendix N.7"]
---
## Background

FrankenRobotics has forty mathematical constructions (C-01 to C-40) that form its spine. A construction fixes
mathematics and its test; the subsystem chapters own the types, kernels, verbs and gates that implement it. This
bead is the specification of construction C-28, "a failure has a minimal counterexample world", tier Frontier [F],
owned by the Nemesis subsystem (adversarial worlds and rare failures), crate `frob-nemesis`.

The problem C-28 solves: when a design fails in simulation, the usual artifact is a failing seed and a video. An
agent cannot act on that. C-28 replaces it with the smallest world that still breaks the design, written as a
passing nominal world plus a short list of typed edits, stored as a regression test (`.fmcw`). It is the dual of
the minimal design repair of construction C-18 (the admitted world nearest to nominal that separates two closed
loops). The payoff is measured in agent repair iterations, not in simulator steps.

Why a specification bead: the mechanism (`nemesis.shrink`, seed bead BD-274) must implement exactly the
mathematics below, and the program's definition of done requires every construction to have a failing-when-wrong
test. This bead writes the contract, builds the law tests that decide whether an implementation satisfies it, and
files the registry entries. Rejected alternatives, with reasons: (1) reuse the sibling bisection tool: FACT
(FrankenSim `483d09c682f2`, observed 2026-11-21) `fs-bisect` bisects a linear history under a monotonicity check
and does not search a subset lattice, and failure sets of worlds are not monotone; (2) report the shrunk world as
"the closest failure": only 1-minimality is computed, so no distance is claimed; (3) keep every failing world:
a world the body cannot solve is a useless trap, hence the solvability witness.

Terms. *World*: a scene, a world address (run seed, world index, episode) and parameters. *Candidate*: a model, a
controller and a policy. *Requirement*: the acceptance formula of the task (construction C-30). *Legality
profile*: the declared bounds inside which Nemesis may edit a world. *Order ideal*: a down-closed subset of a
partially ordered set. *Proof classes*: Proved (a sealed Lean capsule), Certified (an instance certificate the
offline auditor `frob-verify` re-checks in exact arithmetic), Tested (a property or known-answer test with a
negative control), Estimated (depends on a card value, a fit or a sample). Status of every statement today:
HYPOTHESIS; nothing has been run.

## Scope

In: (1) `docs/math/C-28.md`, the construction as a contract; (2) the law tests in `crates/frob-nemesis/tests/`,
each with a negative control; (3) registry entries: known answers KA-142 to KA-144 with derivation files, the
controls seeded from CX-067 and CX-068, the pilot record, the non-claim recorded against the theorem-card table.

Out: the shrinker itself, the `.fmcw` writer and the verb `nemesis.shrink` (BD-274); the solvability oracle
`nemesis.oracle` (BD-271); the legality set and edit vocabulary (BD-270); the artifact guard (BD-272); the
hard-world set (BD-275); recording GATE-NEMESIS-03 and deciding KILL-MCW (BD-274); the retrofit search on a
`.fmcw` (BD-369); attribution on the passing neighbours (construction C-29, beads SPEC-C-29 and BD-279).

## Technical approach

### 1. Definitions (to be written into `docs/math/C-28.md`, section 1)

- A failing world is w = w₀ ⊕ E: a nominal world w₀ in which the candidate passes, plus a finite set E of typed
  atomic edits. An edit is (op, path, from, to, aspects changed): a friction patch, a threshold height, a light, a
  packet delay. If the candidate already fails in w₀ there is nothing to shrink and w₀ is the counterexample.
- Edits are partially ordered by dependency: e ≤ e′ when e′ cannot be applied unless e is applied.
- An admissible sub-world is an order ideal I ⊆ E (e′ ∈ I and e ≤ e′ imply e ∈ I) whose world w₀ ⊕ I is legal.
- The test relation T(I) takes one of four values. `fail`: the world is legal, replays the candidate
  deterministically, and violates the same requirement atom with the same failure class on at least 48 of 64
  paired seeds (TARGET). `pass`: it does not. `inadmissible`: I is not an ideal or the world is illegal; no replay
  is spent. `other-failure`: it fails by a different mechanism.
- A 1-minimal failing ideal is an ideal I with T(I) = fail such that for every maximal element e of I,
  T(I ∖ {e}) is `pass` or `inadmissible`. Removing a non-maximal element does not give an ideal, so only maximal
  elements are tested.
- A solvability witness is evidence that the candidate's own body, actuators and mission constraints admit a
  success in the world. Classes: `solvable-causal` (a witness controller that uses only delivered information
  succeeds), `plant-feasible` (the witness needs privileged state), or rejection with the failed necessary
  condition named.

### 2. Statements, each with its proof class (section 2 of the contract)

| Id | Statement | Class |
|---|---|---|
| S1 | The order ideals of (E, ≤), ordered by inclusion, form a distributive lattice: unions and intersections of ideals are ideals, and ∩ distributes over ∪. | Tested (exhaustive on posets of at most 8 elements) |
| S2 | If T(E) = fail, a 1-minimal failing ideal exists and single-removal descent finds one: while some maximal e has T(I ∖ {e}) = fail, replace I by I ∖ {e}. The descent ends after at most \|E\| removals and at most \|E\|·(\|E\| + 1)/2 tests. | Tested |
| S3 | Delta debugging (the search of record): order E by a linear extension of ≤; partition into k parts, starting at k = 2; if some part fails, recurse on it; otherwise if some complement fails, recurse on it with k − 1 parts; otherwise double k; stop when k exceeds the number of edits. A candidate that is not an ideal is `inadmissible` and costs no replay. The result is then passed through the descent of S2, so 1-minimality does not depend on what the partition skipped. | Tested |
| S4 | 1-minimal is not minimum. When the original list has at most 12 edits, every subset of one or two edits is also tested (at most 12 + 66 = 78 tests) and a smaller failing ideal replaces the result, which is then labelled "minimum among pairs" (`pairs_exhausted: true`). | Tested |
| S5 | Failure sets are not monotone in the edit set, so no distance to a boundary is claimed and no world is called the closest failure. The reported label is "1-minimal under the tested edit relation". | Tested (control) |
| S6 | A sub-world that fails by a different mechanism is neither a pass nor the same failure: it is recorded as a separate finding with its own event classification, and the shrink continues on the original mechanism. | Tested |
| S7 | A counterexample is kept only with a solvability witness; a witness that needs privileged state is tagged `plant-feasible` only; a world with no witness is rejected. | Tested |
| S8 | After the discrete search, each remaining continuous edit is moved toward its nominal value by bisection until the failure is lost, and the last failing value is kept. | Tested |
| S9 | The stored world reproduces its state hash: a `.fmcw` replays with identical digests on its originating backend, or it is invalid. | Tested (digest equality, determinism tier) |
| S10 | Worst-case cost is quadratic in \|E\|. | Tested (counted) |
| S11 | Each passing neighbour I ∖ {e} of the result is an interventional contrast in the sense of C-29: the failing world and the edit that removes the failure travel together to the retrofit search and to the autopsy. | Structural (asserted on the record) |

Size TARGET: at most three atomic edits; a larger result is kept and labelled `non-minimal(k)`.

### 3. Conventions fixed, failure cases and non-claims

- "Minimal" always means 1-minimal under removal of atomic edits. The obligation "a minimal counterexample world is
  the minimum" was considered as a theorem card and rejected because no checker decides it; no bundle may present
  minimality as Proved or Certified.
- A shrinking run whose failure mechanism changes reports a different failure.
- The record carries what replays it: identities, edits, addresses, state digests, the passing neighbours, the
  oracle result, the legality bounds touched with their provenance, and the three no-authorization flags, all
  false. Type sketch (DESIGN): `CounterexampleWorld { id, task, requirement, candidate, base, edits, minimality:
  OneMinimal { pairs_exhausted } | NonMinimal(k), failure, replay, promotion, oracle, legality, prior,
  neighbours_passing, evidence, no_authorization }`, schema `frob.mcw/1`.
- Evidence class of a shrunk world: Tested at the tier it replays on.

### 4. Law-test layout (DESIGN)

`crates/frob-nemesis/tests/laws_c28.rs` holds one test per statement and per known answer.
`tests/support/c28_ref.rs` holds the reference model: ideals enumerated explicitly, the descent of S2 and the
delta debugging of S3 written directly, driven by a *synthetic* failure oracle (a Boolean function of the edit
set with a counter of calls). The suite is generic over a trait `C28Subject { fn shrink(&self, poset, oracle) ->
ShrinkReport }` so the same laws run on the reference model and on the production shrinker of BD-274 when it
exists. Negative controls are mutant subjects in `tests/support/c28_mutants.rs`, each with one planted defect, and
the test `controls_are_caught` asserts each mutant fails its named law. Every test emits one structured line
through the STD-TEST helper: row id, statement id, measured value, expected value, tolerance.

### 5. Registry entries

- `frob-conformance/registry/known_answers.toml`: rows KA-142, KA-143, KA-144 bound to the test ids of this suite,
  each with a derivation file under `docs/derivations/` (created here if the chapter registry bead has not created
  the rows; numbers are append-only).
- `frob-conformance/registry/controls.toml`: one control per mutant, seeded from CX-067 and CX-068, attached to
  GATE-NEMESIS-03 and GATE-NEMESIS-01.
- `frob-conformance/registry/pilots.toml`: the pilot of this construction (below).

### 6. Pilot and ceiling (carried for the mechanism; run by BD-274 with BD-273)

Pilot: planted traps in three scenes. Falsifier (TARGET): fewer than 70% of returned worlds carry a solvability
witness, or one returned world from which a single edit can still be removed. Ceiling: a better artifact per
failure, measured in agent repair iterations and not in steps. Fallback: the failing world unshrunk, with its edit
list. Kill gate KILL-MCW (decided at M3): on unplanted failures fewer than half of shrunk worlds have at most three
edits, or fewer than 90% replay; its fallback is full failing worlds with replay and single-edit contrasts ranked
by margin, with the format kept.

### 7. Implementers and gates

Implementers: `frob-nemesis` (shrinker and `.fmcw`, BD-274; oracle, BD-271). Consumers: `frob-arena` regression
batteries, `frob-autopsy`, `frob-mission` (`mission.separate`, BD-293), `scene.retrofit` (BD-369). Gates that
measure it: GATE-NEMESIS-03 (planted causes recovered exactly in at least 95 of 100 trials; every `.fmcw` replays
with identical digests in 100 of 100; the share of unplanted failures with at most three edits recorded) and the
oracle rows of GATE-NEMESIS-01 (0 of 100 planted impossible traps counted; at least 95 of 100 planted solvable
traps receive a witness). This bead records neither gate; it supplies their known-answer rows and controls.

## Acceptance criteria

- [ ] `docs/math/C-28.md` exists with ten sections (definitions; statements S1 to S11 with proof classes;
      conventions; failure cases; non-claims; foundations with the FACT about `fs-bisect`, its pin and date; law
      tests; registry entries; pilot and ceiling; implementers and gates) and contains no formula with an
      undefined symbol.
- [ ] `cargo test -p frob-nemesis --test laws_c28` passes on the reference model, and on the production shrinker
      if BD-274 has landed.
- [ ] `controls_are_caught` shows every mutant of the list below failing its named law.
- [ ] KA-142, KA-143 and KA-144 are in the known-answer registry with derivation files, each with a stated
      tolerance and a registered control (KA-143 and KA-144 were printed with no control; the controls below fill
      them).
- [ ] Controls seeded from CX-067 and CX-068 are in the control registry, attached to their gates.
- [ ] The pilot record is in the pilot register with its falsifier, ceiling and fallback.
- [ ] `scripts/e2e/spec_c28.sh` exits 0 and its log has no failed assertion.

## Test plan

### Unit tests

- `ideals_form_distributive_lattice`: seeded random posets of at most 8 elements; asserts closure under ∪ and ∩ and
  distributivity; counts: a chain of n has n + 1 ideals, an antichain of n has 2ⁿ, the two-element chain has 3.
- `ka_142_two_edit_cause`: eight edits, no dependencies, failure exactly when 3 and 6 are both present; expected
  result {3, 6}; label 1-minimal, and "minimum among pairs" since 8 ≤ 12.
- `nonmonotone_grease_rug`: edits 1 (grease patch) and 2 (rug over the patch), 1 ≤ 2; {1} fails, {1, 2} passes;
  input {1} returns {1}; input {1, 2} is refused as not a failing world; the report has no distance field.
- `ddmin_is_not_minimum`: failure when {a, b} or {c, d, e} is present, list order (a, c, d, e, b); plain delta
  debugging ends at {c, d, e}; with the pair check the result is {a, b}.
- `one_minimal_property`: 10⁴ seeded random Boolean failure functions on at most 10 edits with random dependency
  posets; asserts T(R) = fail, every single-removal neighbour is not `fail`, and the call count is at most
  \|E\|² + 2·\|E\| + 78.
- `dependency_respected`: the oracle is never called on a set that is not an ideal.
- `mechanism_change_is_new_finding`: a sub-world of class `timeout` under a parent of class `slip` yields a
  separate finding and the final class is `slip`.
- `ka_143_impossible_world_rejected`: a doorway of 0.50 m for a body 0.60 m wide; the reference oracle's necessary
  condition fails; the world is rejected with the condition named.
- `witness_tagging`: a witness that reads privileged state is tagged `plant-feasible`.
- `continuous_edit_bisection`: failure when x > 0.3 on [0, 1]; ten bisections keep a value in (0.3, 0.3 + 2⁻¹⁰].
- `ka_144_replay_digest`: a stored record replays on the synthetic deterministic oracle to the same digest.

### End-to-end script and logging

`scripts/e2e/spec_c28.sh` (seed-explicit, idempotent): runs `frob --json doctor`; `cargo test -p frob-nemesis
--test laws_c28`; `cargo test -p frob-conformance --test g1 -- KA-142 KA-143 KA-144`; checks that
`docs/math/C-28.md` and the three derivation files exist; checks the registry rows. It sources the STD-E2E helper
and writes `target/e2e-logs/spec_c28/<utc>.ndjson` plus a transcript. Every line has `ts`, `script`, `bead`
(`SPEC-C-28`), `step`, `event` (`start`, `cmd`, `assert`, `metric`, `refusal`, `end`), the command line, exit code,
wall time, the evidence block (claim Known answer, class Tested, tier G1 host, determinism D0, seed), measured and
expected values (result sets, call counts against their bounds), and on failure the first differing value and the
path of the fixture. The final `summary` line counts assertions and names GATE-NEMESIS-03 and GATE-NEMESIS-01.

### Known answers and negative controls

| Row | Truth | Tolerance | Tier | Control |
|---|---|---|---|---|
| KA-142 | Eight edits, failure exactly when 3 and 6 are present: the search returns {3, 6} | exact | G1 host | Non-monotone control: {1} fails, {1, 2} passes; the tool returns {1} and does not call it a distance |
| KA-143 | A doorway narrower than the body is rejected as a counterexample for lack of a witness | exact (verdict) | G1 host | Mutant `keep_any_failure` (new): keeps the trap; must be caught |
| KA-144 | A stored `.fmcw` reproduces its state hash | exact (digest) | G5 referee (bound by BD-274) | Mutant `edit_changed_after_hash` (new): one edit value altered; replay digest differs |

Mutants: `monotone_pruning` (assumes a subset of a passing set passes; on edits {1, 2, 3} with failure when
(1 and not 2) or (2 and 3), removal order (3, 2, 1), it returns {1, 3} although {1} fails, and is caught by
`one_minimal_property`); `stop_after_partition` (skips the final descent); `keep_any_failure`;
`claims_distance` (adds a distance-to-boundary field); `ignores_dependency` (tests non-ideals);
`edit_changed_after_hash`. Counterexamples carried: CX-067 (the shrunk world is not the smallest and its size is
not a distance), CX-068 (not every failing world is a counterexample). Metamorphic relation: permuting the labels
of independent edits permutes the result. Determinism: twice on one machine with equal logs.

## Considerations

- Risk: on unplanted failures shrinking may not reach three edits; that is the trigger of KILL-MCW, named above.
- The 48-of-64 threshold and the three-edit size are TARGETs owned by the Nemesis chapter; this contract carries
  them and does not set them.
- Cost envelope: quadratic in the number of edits in the worst case; each test is one deterministic replay on
  paired seeds, so the law tests themselves use a synthetic oracle and run in milliseconds.
- Interactions: the passing neighbours feed C-29; the bottleneck named by a clearance complex (C-33) explains a
  doorway trap; the solvability witness is what separates a trap from a counterexample.
- Privacy: logs carry edit paths of synthetic fixtures only, never private-scene data.

## References

Plan §10 C-28; §26.7; §12.3 (obligations no checker decides); Appendix D.6 "Minimal failing ideal"; Appendix I.11;
KA-142 to KA-144; CX-067, CX-068; GATE-NEMESIS-01, -02, -03; KILL-MCW; BET-22; C-18, C-29, C-33.
