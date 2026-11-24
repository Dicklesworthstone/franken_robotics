---
key: "CARD-T-32"
title: "Theorem card T-32: dominance on a product model (success of A with failure of B is unreachable)"
type: "task"
priority: 2
milestone: "M4"
labels: ["m4", "lemma", "frob-cert", "frob-mission", "frontier", "test"]
parent: "WP-66"
deps: ["N12-06", "N12-17", "N25c-24", "SPEC-C-40"]
needs: []
related: ["BD-359", "BD-364", "N12-16", "N12-46", "N25d-36"]
gates: []
plan_refs: ["§12.3 (T-32)", "§10 C-40", "§25.16", "Appendix D.6 (dominance by unreachability)"]
---
## Background

The question "can replacing design A by design B make a formerly successful mission fail?" does not need either
design to have a guarantee. For two closed loops with fixed controllers, a regression is a shared world input in
which A succeeds and B fails. The product system runs both plants on the same exogenous scene, payload and clock
law, with a joint terminal monitor on the pair of outcomes. If the pair (success, failure) is unreachable over an
enclosure of every admitted joint execution, with progress and nonempty executions, B dominates A on that cell.
A concrete paired trace that reaches it is a counterexample; an outer enclosure that intersects it triggers a
witness search or `unknown`. Only declared shared inputs are coupled, and every other branch choice varies
independently inside the enclosure. Shared inputs are compiled at physical coordinates: one space-indexed
material field read at each robot's actual contact point, one latent lighting state seen by each camera from
where it is. The answer carries the paired verdict and each design's absolute verdict.

Statement (T-32). Dominance on a product model with rational piecewise-affine outcome maps, the reference
fixture included: the pair (success of A, failure of B) is unreachable.

Route: exact case analysis; interval enclosure for cells that are not affine (`product-unreachable`). Tier:
Frontier. Rejected alternatives: computing two success sets and comparing them (outcomes enclosed separately
admit the impossible pair); pairing by seed and assuming the worlds match.

## Scope

In: statement document, registry row, the per-cell `product-unreachable` certificates bound to this card, the
reference fixture, mutants. Out: building product systems and searching for witnesses (the product-dominance
bead); the auditor's checker (N12-17); the statistical paired comparison used as the fallback.

Order of work. This card is finished before the code that implements or uses its theorem and never depends on it:
product-system dominance in `frob-mission` (BD-359) is `related`, not a dependency (a card that waited for its
implementer would close a loop, because the implementer's own acceptance runs the card's law tests). What the card
does depend on is in `deps`: the card registry, its specification, its pilot where the route is Frontier, and
Lemma's own proof machinery. It closes on its statement document, its registry row, its specimens and mutants, and
every acceptance line that its own fixtures decide. An acceptance line that can only be observed on live output
(per-cell certificates of a product system built by that mechanism) is run inside the implementing bead, with this
card's law tests, as part of that bead's acceptance; the receipt produced there is what moves the registry row for
that output.

## Technical approach

1. Model class: shared inputs z in a box with rational ends; finitely many independent branch variables per
   plant; on each piece (a cell of z together with a combination of branch values) each plant's success is the
   sign of a rational affine function of z, or, for a piece that is not affine, an outward-rounded interval
   enclosure of the success margin.
2. The certificate lists the pieces with their outcome pairs. The auditor checks (a) joint coverage: the pieces
   partition the product of the shared box and all branch combinations of both plants, and no branch of A is
   tied to a branch of B unless that input is declared shared; (b) the outcome pair on each piece, exactly for
   affine pieces (signs at the piece's vertices, with no sign change inside) and by interval ends otherwise;
   (c) that no piece has the pair (success, failure); (d) the declarations of progress and nonempty executions
   per piece; (e) that the bundle carries each design's absolute verdict next to the paired one.
3. Class and limits: `Certified` for the declared product model. A relative verdict of no loss does not
   establish that either design meets the mission. One mislabeled branch invalidates the certificate. The
   lowering that keeps spatially coupled disturbances coupled is the producer's and is reviewed through the
   primitive equations in the bundle; a conventional reachability tool applied to the same product model is the
   comparator, and a tie ships the generic method.
4. Relies on it: construction C-40; clearing a replacement without a guarantee for either design; design
   separation by a minimized world pair (the dual question).
5. Artifacts: `docs/math/cards/T-32.md`; registry row; specimens `product-unreachable/accept-T-32-*.frobcert`
   and mutants.

## Acceptance criteria

- [ ] `frob-verify` accepts the reference fixture: a surface parameter z ∈ [0, 1]; A succeeds when z ≥ 3/5 and B
      when z ≥ 2/5; under the shared z the outcome pairs are (0, 0) on [0, 2/5), (0, 1) on [2/5, 3/5) and (1, 1) on
      [3/5, 1]; the pair (1, 0) is unreachable; B adds the interval [2/5, 3/5), a gain of 1/5 under a uniform law;
      neither design is guaranteed (A fails for z < 3/5 and B for z < 2/5).
- [ ] Rejected mutations: the two outcomes enclosed separately (all four pairs admitted, including the
      impossible one); a product whose internal branches are wrongly tied so that only (0, 0) and (1, 1) appear,
      whose projections cover both marginal outcome sets and so hide a reachable (1, 0): the checker must demand
      joint coverage; a missing piece; a piece on which a predicate changes sign.
- [ ] The bundle prints both absolute verdicts; a bundle that prints only "B never loses to A" is rejected as
      incomplete.
- [ ] Registry row `checked`, class `Certified`, with hashes and checker identity.

## Test plan

### Unit tests

`crates/frob-lemma/tests/cards/t32.rs`: `surface_parameter_fixture_accepts`; `gain_interval_is_one_fifth`;
`separate_enclosures_admit_all_four_pairs`; `tied_branches_hide_one_zero`; `missing_piece_rejects`;
`sign_change_inside_piece_rejects`; `absolute_verdicts_required`; `non_affine_piece_by_interval`
(a margin z² − 1/4 on [3/5, 1] enclosed as [11/100, 3/4] > 0); `progress_declaration_required` (a piece without
its progress and nonempty-execution declarations is rejected, naming the piece); `undeclared_shared_input_rejects`
(a branch of A tied to a branch of B by an input absent from the declared shared set).

### End-to-end script and logging

`scripts/e2e/lemma_auditor_mutations.sh` (bead N12-34) runs the specimens and mutants;
`scripts/e2e/lemma_second_machine.sh` with profile `m4` (bead N12-35) re-checks dominance bundles of the M4 family
answers. For this card their NDJSON lines (STD-E2E helper) carry `ts`, `script`, `bead` ("CARD-T-32"), `step`
(`T-32/<fixture-or-mutant>`), `event`, the exact command line, exit code, wall time, the number of pieces and
whether they cover the joint product, the outcome pairs found with their measure under the declared law (the
fixture's gain 1/5), both absolute verdicts, the bundle's BLAKE3 on each machine, the expected and observed
verdict, and on a mismatch the first failing piece (its cell, branch combination and pair) with the path to
reproduce.

### Known answers and negative controls

The fixture above. CX-096 (pairing two designs by seed and tick does not make the worlds match: two tyres on
different paths over a patterned floor meet different friction at the same tick, so a product coupled by tick
and not by position reports a false regression or hides a true one). CX-097 ("B never loses to A" does not mean
B meets the mission). Negative controls as listed. Determinism: exact.

## Considerations

- The kill gate of the owning mechanism falls back to the paired statistical comparison with position-indexed
  fields, for which this card issues nothing.
- A controller retuned per body needs its own information and budget contract; the card assumes fixed
  controllers.

## References

§12.3; C-40; §25.16; CX-096, CX-097; D-17.
