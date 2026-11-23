---
key: "SPEC-C-40"
title: "spec C-40: dominance as unreachability in a product system with shared inputs compiled at physical coordinates"
type: "task"
priority: 2
milestone: "M4"
labels: ["m4", "lemma", "spine", "spec", "frontier", "augur", "frob-mission", "frob-cert", "test"]
parent: "WP-66"
deps: ["N10a-01", "STD-TEST", "STD-E2E"]
needs: []
related: ["BD-132", "BD-161", "BD-262", "BD-359", "SPEC-C-32", "N10-CX", "N10c-10", "CARD-T-32", "BD-128", "BD-026"]
gates: ["GATE-AUGUR-16"]
plan_refs: ["§10 C-40", "§25.16", "§12.3 T-32", "§12.4", "Appendix D.6", "Appendix M.9", "Appendix N.7"]
---
## Background

Construction C-40 of the FrankenRobotics spine answers an engineer's question directly: can replacing design A by
design B make a formerly successful mission fail? The two success sets are expensive to compute and may be mostly
unknown. C-40 asks for the relation instead. For two closed loops with fixed controllers, a regression is a
shared world input in which A succeeds and B fails. Build the product of the two systems driven by the same
exogenous scene, payload and clock law, with a joint terminal monitor on the pair of outcomes, and ask whether
the pair (A succeeds, B fails) is reachable. Tier Frontier [F]. Owner: the Augur subsystem (mission queries),
crate `frob-mission`, with the checker in `frob-cert`.

What it replaces: computing two success sets and comparing them, and pairing episodes by seed while assuming the
worlds match. Payoff: a replacement is cleared without either design having a guarantee, by a relation that can
be far cheaper to establish than either success set.

Design reasoning. The lowering is the contribution. Two tyres following different paths over the same floor do
not receive equal friction samples at corresponding ticks, and a shared latent lighting state produces different
sensor residuals once the cameras diverge. So shared inputs are compiled at physical coordinates: one
space-indexed material field read at each robot's actual contact point, one latent lighting state seen by each
camera from where it is. Pairing by world address shares seeds, and a seed indexed by tick is not a field indexed
by position. Honest boundaries: a relative verdict of no loss does not establish that either design meets the
mission; one mislabelled branch invalidates the certificate; a conventional hybrid reachability tool applied to
exactly the same product model is the comparator, and the only advantage claimed is the lowering that keeps
spatially coupled disturbances coupled; a controller retuned per body needs its own information and budget
contract and is a different query. FACT (FrankenSim `483d09c682f2`, observed 2026-11-21): the sibling interval
library supplies enclosures; nothing builds the product.

Terms. *Closed loop*: a design with a fixed controller. *Outcome*: 1 for success of the mission requirement, 0
for failure. *Product system*: both plants run on shared exogenous inputs with a monitor on the outcome pair.
*Shared input*: an exogenous quantity declared common to both runs. *Branch choice*: an internal nondeterministic
alternative of one plant (for example a stick or slip fork). *Cell*: a box of parameters and inputs on which a
verdict is issued. *Enclosure*: an outer approximation of every admitted joint execution. Proof classes: Proved,
Certified (re-checked by the offline auditor), Tested, Estimated; status HYPOTHESIS.

## Scope

In: `docs/math/C-40.md`; law tests in `crates/frob-mission/tests/laws_c40.rs`, with the auditor-side rows bound in
`crates/frob-cert/tests/`; registry entries for KA-189, KA-190, the statement of theorem card T-32, and the
controls seeded from CX-094 to CX-097.

Out: product-system dominance as a mechanism, its pilot and KILL-AUGUR-DOMINANCE (BD-359); the auditor's
`product-unreachable` checker (BD-132 and BD-359); the strict CPU profile used for per-case replay (BD-161); the
paired statistical comparison that is the fallback (BD-262); recording GATE-AUGUR-16.

Register row (the shared rule of N10a-01). The construction register
`frob-conformance/registry/constructions.toml` is created by N10a-01 with forty typed rows whose obligation fields
are empty; this bead fills row C-40: `rust_type` (the subject trait of the law suite below, replaced by the
production type when the implementing bead binds it), `verb` (the paired-dominance verb of the mission surface),
`failing_test` (a test of `frob-mission::laws_c40` with its registered control), `payoff.gate` (GATE-AUGUR-16),
`statements`, `known_answers` (KA-189, KA-190), `cards` (T-32), `gates`, `pilot` (the record `PILOT-C-40` of the
pilot register, required because the tier is Frontier) and `generic_comparator` (the off-the-shelf method named in
the pilot paragraph of this bead). Doctrine D-2 (no ornamental mathematics) is checked on these fields: a
construction with no verb or no failing test is moved to the parked register by a decision record. The contract
`docs/math/C-40.md` has the ten sections of the template `docs/math/TEMPLATE.md` of the same bead, in the
template's order (object and definitions; what it replaces and the payoff; conventions; statements; formulas;
fixtures and known answers; failure cases, refusals and retirement; non-claims; foundations; implementation map);
content that this bead lists under other headings goes into the template section that matches. Statement
identifiers are entered in the contract and in the register with the construction prefix (`C-40.S1` and so on, as
the template requires); the tables of this bead and of its part beads abbreviate them.

Order of work (DESIGN; the specification-first rule stated in section 6 of the construction register bead
N10a-01). This bead is written before the code it specifies and does not depend on BD-128 (the `frob-mission`
core) or BD-026 (the auditor crate `frob-cert`): those beads are listed in `related` and are not prerequisites.
The law suite is generic over its subject trait and closes on the exact reference model and the mutants in its own
test support. If `frob-mission` or `frob-cert` do not exist when this bead starts, the suite and its support files
are written under `crates/frob-conformance/tests/` with the same file names (that crate exists from BD-025) and
the commands below read `-p frob-conformance`; the bead that implements the construction moves them into the
owning crate, binds its production types to the subject trait and runs the unchanged suite as part of its own
acceptance. A criterion or script step below that names production code or a verb of an implementing bead is owed
at that bead's acceptance and is logged `pending` here until then.

## Technical approach

### 1. Statements

| Id | Statement | Class |
|---|---|---|
| S1 | If the outcome pair (success, failure) is unreachable over an enclosure of every admitted joint execution of the product system, with progress and nonempty executions, then B dominates A on that cell. | Certified per cell (theorem card T-32, proof kind `product-unreachable`: exact case analysis for rational piecewise-affine outcome maps, interval enclosure for cells that are not affine) |
| S2 | A concrete admissible paired trace that reaches (success, failure) is a counterexample. | Tested class at most (proof kind `trace-witness`, a replayed execution) |
| S3 | An outer enclosure that intersects (success, failure) without a concrete trace triggers a witness search or the verdict `unknown`. | Tested |
| S4 | Only declared shared inputs are coupled; every other branch choice varies independently inside the enclosure. | Tested (the coverage control) |
| S5 | Joint coverage is checked explicitly: the diagonal pairs (0, 0) and (1, 1) project onto both full marginal outcome sets and hide (1, 0) when the two internal branch choices can vary independently; a product that omits an admitted combination of branch choices is rejected. | Tested (auditor rule) |
| S6 | Shared inputs are compiled at physical coordinates: a space-indexed field is read at each robot's actual contact point; a latent state is observed by each sensor from its own pose. | Tested (the coupling counterexample) |
| S7 | The answer carries the paired verdict and each design's absolute verdict, separately. | Tested |
| S8 | The checker confirms that each projection of the product solves its original plant and that the common variables express the requested physical comparison. The initial domain is split where a mode transition could break relational invariance, and a numerical branch that cannot be enclosed goes to per-case replay. | Contract clauses for the mechanism |

### 2. Hand-checkable fixture

A surface parameter z ∈ [0, 1]; A succeeds when z ≥ 3/5 and B when z ≥ 2/5. Neither is guaranteed: A fails for
z < 3/5 and B for z < 2/5. Under the shared z the cells are [0, 2/5) with outcomes (0, 0), [2/5, 3/5) with
(0, 1) and [3/5, 1] with (1, 1); the pair (success, failure) is unreachable, and B adds the interval [2/5, 3/5),
a gain of 1/5 under a uniform law. Without a law the interval of newly covered surfaces is still an engineering
answer.

- Counterexample to separate enclosures: enclosing the two outcomes separately gives {0, 1} for each and admits
  all four pairs, including the impossible one.
- Coverage control: a product whose internal branches are wrongly tied reaches only (0, 0) and (1, 1), whose
  projections cover both marginal outcome sets, and so hides a reachable (1, 0).
- Coupling counterexample: two tyres on different paths over a patterned floor meet different friction at the same
  tick, so a product coupled by tick and not by position reports a false regression or hides a true one.

### 3. Conventions, failure cases and non-claims

- Outcome pair order is (A, B); "regression" is (1, 0).
- A vacuous product (no execution, or one that does not progress to the horizon or a declared terminal outcome)
  establishes nothing.
- Not claimed: that either design meets the mission; any advantage over a conventional reachability tool beyond
  the position-indexed lowering; anything for controllers retuned per body.

### 4. Law-test layout, registry, pilot and gate

`crates/frob-mission/tests/laws_c40.rs` with `tests/support/c40_ref.rs`: outcome maps as rational piecewise-affine
functions of shared inputs and of independent branch choices; exact case analysis over the induced cells; a
*synthetic* striped floor μ(x) with two robots on different paths for the coupling laws; a coverage checker.
Generic over a trait `C40Subject` (build the product; return per-cell joint outcomes, a witness or `unknown`).
Mutants in `tests/support/c40_mutants.rs`. Auditor rows are bound in `frob-cert` once the checker exists.
Registry: KA-189, KA-190 with derivation files; controls from CX-094, CX-095, CX-096, CX-097.

Pilot as the construction states it: one contact transition, a sole exchange over a split-grip strip, against a
reachability tool on the same product model. Falsifier: one mislabelled draw, or no cell closed that per-case
replay would not close as cheaply. The owning chapter words it as a sole replacement on a single step edge with a
shared space-indexed friction field, against a conventional hybrid reachability tool, with every assumption
translation, uncertainty split and checker call charged to both arms. Ceiling: paired decisions on cells where
the relation closes (the fraction of paired draws closed without replay, measured on the pilot); elsewhere,
per-case replay. Fallback: the paired statistical comparison of construction C-15 with position-indexed fields.
Gate GATE-AUGUR-16 at M4: the z fixture exact; on the contact-transition pilot zero mislabelled joint outcomes
against per-case referee replay over 10⁴ draws. Kill gate KILL-AUGUR-DOMINANCE (M4): one mislabelled joint
outcome, or a conventional reachability tool closes the same cells at similar cost; fallback: paired comparison
by replay, with the product lowering kept only as the rule for space-indexed shared inputs.

## Acceptance criteria

- [ ] `docs/math/C-40.md` exists with S1 to S8, the fixture with its three cells, the three counterexamples, the
      conventions, the non-claims and both wordings of the pilot.
- [ ] `cargo test -p frob-mission --test laws_c40` passes on the reference model, and on the production product
      if BD-359 has landed.
- [ ] Every mutant below fails its named law.
- [ ] KA-189 and KA-190 are registered with derivation files; controls from CX-094 to CX-097 are attached to
      GATE-AUGUR-16.
- [ ] `scripts/e2e/spec_c40.sh` exits 0.
- [ ] Row C-40 of `frob-conformance/registry/constructions.toml` has its obligation fields filled as the
      Scope states, and `cargo test -p frob-conformance --test g0_doctrine -- d2` reports no failed
      obligation for the row (fields not yet due are reported `pending` under the grace rule of N10a-01).

## Test plan

### Unit tests

- `ka_189_shared_z`: the three cells and their outcome pairs as above; (1, 0) unreachable; gain interval
  [2/5, 3/5) of length 1/5; absolute verdicts "not guaranteed" for both designs.
- `threshold_family_property`: 10⁴ seeded rational thresholds a (for A) and b (for B): dominance holds exactly
  when b ≤ a, and the gain is max(0, a − b).
- `separate_enclosures_admit_impossible_pair`: the product of the marginal outcome sets has four pairs.
- `witness_when_reachable`: with B succeeding only for z ≥ 4/5, the pair (1, 0) is reached on [3/5, 4/5) and the
  trace z = 7/10 is returned as a counterexample.
- `unknown_when_enclosure_is_loose`: a deliberately loose enclosure that intersects (1, 0) with no trace found
  returns `unknown`, never dominance.
- `ka_190_joint_coverage`: each plant has one independent branch choice that decides its outcome; all four pairs
  are reachable; the product with the two choices tied reaches only (0, 0) and (1, 1), covers both marginals, and
  is rejected by the coverage check because the combination (1, 0) of branch choices is admitted and absent.
- `position_coupling_is_truth`: on a striped floor with 10³ seeded pairs of paths the position-indexed product
  equals direct evaluation of each robot on the floor.
- `tick_coupling_hides_regression`: A reads the floor at x = 0.75 (high friction, success) and B at x = 1.5 (low,
  failure) at one tick; the true pair is (1, 0); a tick-indexed product gives both robots one sample and returns
  (1, 1) or (0, 0).
- `tick_coupling_false_regression`: both robots are on high-friction stripes at their decision ticks; a field
  generated per tick gives B a low sample and reports (1, 0).
- `paired_and_absolute_reported`: the answer object has the paired verdict and two absolute verdicts.
- `vacuous_product_rejected`: a product with no admitted execution does not establish dominance.
- `mislabelled_cell_rejected`: flipping the label of one cell makes the exact re-evaluation reject the
  certificate.

### End-to-end script and logging

`scripts/e2e/spec_c40.sh`: `frob --json doctor`; `cargo test -p frob-mission --test laws_c40`; `cargo test -p
frob-conformance --test g1 -- KA-189`; `cargo test -p frob-conformance --test g0 -- KA-190`; `frob --json prove
list` filtered to card T-32; when bundles exist, `frob-verify` on the specimen bundle and on the wrongly tied
product, expecting accept and reject. Log to `target/e2e-logs/spec_c40/<utc>.ndjson` with a transcript; each line
has `ts`, `script`, `bead`, `step`, `event`, command line, exit code, wall time, the evidence block (claim Known
answer, class, tier G1 auditor or host, determinism D0, seed), cell boundaries as rationals with their joint
outcomes against expected, and on failure the first cell or path pair that differs. The `summary` line counts
assertions and names GATE-AUGUR-16.

### Known answers and negative controls

| Row | Truth | Tolerance | Tier | Control |
|---|---|---|---|---|
| KA-189 | z ∈ [0, 1], A succeeds when z ≥ 3/5 and B when z ≥ 2/5: neither is guaranteed; (success, failure) is unreachable under the shared z; B adds [2/5, 3/5), a gain of 1/5 under a uniform law | exact (rationals) | G1 auditor | Mutant `separate_enclosures`: admits all four pairs |
| KA-190 | A product whose internal branches are wrongly tied reaches only (0, 0) and (1, 1) and hides a reachable (1, 0); the checker demands joint coverage and rejects it | exact (verdict) | G0 auditor | The wrongly tied product itself; mutant `marginal_coverage_only` accepts it |

Other mutants: `tick_indexed_field`; `paired_verdict_only` (drops the absolute verdicts); `vacuous_dominance`.
Counterexamples carried: CX-094 (dominance cannot be checked by enclosing each design's outcomes separately),
CX-095 (covering each marginal outcome set does not cover the joint outcomes), CX-096 (pairing by seed and tick
does not make the worlds match), CX-097 ("B never loses to A" does not mean B meets the mission). Determinism:
exact rationals, twice on one machine.

## Considerations

- The two wordings of the pilot (a split-grip strip; a single step edge) describe one contact transition with a
  shared space-indexed friction field; the Augur lead owns the fixture, and this bead records both.
- If KILL-AUGUR-DOMINANCE fires, S6 survives as a rule of every paired comparison: shared inputs are indexed by
  position and not by tick.
- The stronger combined test named by the owning chapter (replace a sole, perturb the clock, and ask which earlier
  conclusions can still be proved) is scheduled after the pilots and is out of this contract.
- Risk: one mislabelled branch voids a certificate; the coverage check and the per-case replay of the gate are
  the two defences.

## References

Plan §10 C-40; §25.16; theorem card T-32; proof kinds `product-unreachable`, `trace-witness`; Appendix D.6
"Dominance by unreachability"; KA-189, KA-190; CX-094 to CX-097; GATE-AUGUR-16; KILL-AUGUR-DOMINANCE; D-17; C-15.
