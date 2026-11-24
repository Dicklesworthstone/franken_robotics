---
key: "CARD-T-23"
title: "Theorem card T-23: the docking family (a robust causal policy exists iff on time and ε < (U + ½)/4)"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "lemma", "frob-cert", "frob-lemma", "solid", "test"]
parent: "WP-42"
deps: ["BD-041", "BD-132", "N12-06", "SPEC-C-19"]
needs: []
related: ["CARD-T-30", "GB-24", "N10b-08", "N12-05", "N12-35", "N12-39", "N12-46", "N25d-36"]
gates: []
plan_refs: ["§12.3 (T-23)", "§10 C-19", "§10 C-17", "§10 C-18", "§12.7", "Appendix D.6 (docking boundary)"]
---
## Background

Information has a clock. A causal policy is a map from the history of arrived packets to commands: the command
decided at an instant depends only on packets whose arrival is not later than that instant. The docking fixture
gives the trade-off between sensing, actuation and delivery time in closed form. A unit mass starts at rest and
must reach position 1 with velocity 0 after three ticks of unit length. Its acceleration during tick k is g·u_k
with |u_k| ≤ U, and the gain g ∈ {1, 2} is fixed and hidden. The only informative packet is the velocity after
the first tick, read with additive error in [−ε, +ε]. Writing p = u₀, the terminal conditions force
u₁ = 1/g − 2p and u₂ = p − 1/g, and the bounds for both gains hold exactly when 1 − U ≤ p ≤ (U + ½)/2 for
U ∈ [½, 1]. The readings are centred at p and 2p, and they must be disjoint because the required next commands
differ by ½.

Statement (T-23). The docking family: for U ∈ [½, 1] a robust causal policy exists if and only if the packet is
on time (it arrives before the second command) and ε < (U + ½)/4, equality excluded. If the packet arrives after
the third command is committed, no robust policy exists for any U and ε.

The inequality prices sensing in actuator authority at a rate of one quarter, because the first command is also
a probe: a larger probe separates the readings and needs more authority to recover from. Route: exact rational
case analysis on a grid of instances (Solid, `Certified` per cell); the general statement in Lean by linear
arithmetic over ℚ (Frontier, N12-39). It is a scalar model: no device timing follows from it. Rejected
alternative: evidence consisting of one successful policy per world (each world alone has a successful open-loop
command and no single command succeeds in both).

## Scope

In: statement document, registry row, the per-cell certificates on the instance grid (kind `finite-game` for
inclusions, `alias-pair` for exclusions) bound to this card, the boundary cases, the registered mutations.
Out: the solver for the family and its scenario-tree comparator (the docking-family bead); the checkers
(BD-132); the Lean theorem (N12-39); the three-gain amendment (polarity, N12-05).

Order of work. This card is finished before the code that implements or uses its theorem and never depends on it:
the docking-family solver of `frob-mission` (BD-129) is `related`, not a dependency (a card that waited for its
implementer would close a loop, because the implementer's own acceptance runs the card's law tests). What the card
does depend on is in `deps`: the card registry, its specification, its pilot where the route is Frontier, and
Lemma's own proof machinery. It closes on its statement document, its registry row, its specimens and mutants, and
every acceptance line that its own fixtures decide. An acceptance line that can only be observed on live output
(bundles emitted by the solver itself and the grid classification run through them) is run inside the implementing
bead, with this card's law tests, as part of that bead's acceptance; the receipt produced there is what moves the
registry row for that output.

## Technical approach

1. Instance grid (DESIGN, the grid of the plan-time pilot to be re-run): U = ½ + i/32 for i = 0…16 and ε = j/64
   for j = 0…32, 17 × 33 = 561 cells with the packet on time. A cell is admitted exactly when 2j < 32 + i (that is
   ε < (U + ½)/4): for even i that is (32 + i)/2 values of j, for odd i (33 + i)/2, so 180 + 164 = 344 admitted and
   217 excluded, of which the nine cells with 2j = 32 + i (even i) lie exactly on the boundary and are excluded;
   all 561 late-packet cells are excluded.
2. Inclusion certificate for a cell (kind `finite-game`): the policy table is the first command p and, for each
   reading interval, the pair (u₁, u₂). With p = (U + ½)/2 (the largest admissible probe) the readings for gain 1
   lie in [p − ε, p + ε] and for gain 2 in [2p − ε, 2p + ε], disjoint exactly when ε < p/2 = (U + ½)/4. The auditor
   replays both worlds at the error vertices ±ε in exact rationals: velocity g·(p + u₁ + u₂) = 0, position
   g·(5/2·p + 3/2·u₁ + 1/2·u₂) = 1, |u_k| ≤ U, and equal histories receive equal commands.
3. Exclusion certificate at or above the bound (kind `alias-pair`): a reading common to both worlds exists
   (p + e₁ = 2p + e₂ with |eᵢ| ≤ ε requires ε ≥ p/2) for every admissible first command p, and the continuations
   that succeed in the two worlds are disjoint (they differ by ½ in u₁). At equality the common reading is exact:
   at U = ¾ with p = 5/8 both worlds can report 15/16 while needing −¼ and −¾.
4. Exclusion certificate for a late packet (kind `alias-pair`): with no informative packet before commitment the
   two worlds have identical packet histories under every command prefix; required commands (1/g − 2p, p − 1/g)
   differ by ½ between the gains, so no single sequence succeeds in both.
5. The bound is an open threshold: the supremum of tolerable ε is not attained, and a certificate that claims
   ε = (U + ½)/4 as admitted is rejected.
6. Class and limits: `Certified` per cell for the declared scalar model; `Proved` for the general statement only
   when its capsule is sealed.
7. Artifacts: `docs/math/cards/T-23.md`; registry row; specimens `finite-game/accept-T-23-*.frobcert`,
   `alias-pair/accept-T-23-*.frobcert` and mutants.

## Acceptance criteria

- [ ] `frob-verify` accepts the inclusion at (U, ε) = (¾, ¼) with the packet on time, and the exclusions at
      (¾, 5/16), (¾, 3/8) and for the late packet at several (U, ε).
- [ ] Rejected mutations: the inclusion bundle with its quantifiers reordered; an inclusion claimed at
      ε = 5/16; a policy table that gives different commands for one reading; an alias pair whose histories
      differ in one packet.
- [ ] The grid classification through the certificates reproduces 344 admitted, 217 excluded, nine boundary
      cells excluded and every late-packet cell excluded.
- [ ] Registry row: instance route `checked`, class `Certified`, with bundle hashes; Lean route `hypothesis`
      until sealed.

## Test plan

### Unit tests

`crates/frob-cert/tests/cards_t23.rs`: `terminal_conditions_force_commands` (u₁ = 1/g − 2p, u₂ = p − 1/g from the
two terminal equations); `bounds_give_probe_interval` (1 − U ≤ p ≤ (U + ½)/2 on U ∈ [½, 1]);
`witness_quarter_probe` (p = ¼ with an exact on-time reading: (¼, ½, −¾) for g = 1 and (¼, 0, −¼) for g = 2 both
reach (1, 0) exactly); `three_quarters_probe_tolerances` (at U = ¾ the probe p = ¼ tolerates ε < 1/8 and the probe
p = 5/8 tolerates ε < 5/16); `inclusion_three_quarters_quarter` (p = 5/8: commands (−¼, −3/8) for g = 1 and
(−¾, 1/8) for g = 2); `equality_shared_reading_15_16`; `late_packet_commands_differ_by_half`;
`grid_counts_344_217_9`; `quantifier_reorder_rejects`; `open_threshold_not_attained`;
`table_not_function_rejects` (the (¾, ¼) table with two different command pairs for the reading 1);
`alias_histories_differ_rejects` (the late-packet alias pair with the packet delivered before the second command
in one world: the histories differ and the pair is rejected).

### End-to-end script and logging

`scripts/e2e/lemma_second_machine.sh` (bead N12-35) ships the docking cells as bundles and re-checks them on the
second machine; `scripts/e2e/lemma_auditor_mutations.sh` (bead N12-34) runs the mutants. For this card their
NDJSON lines (STD-E2E helper) carry `ts`, `script`, `bead` ("CARD-T-23"), `step` (`T-23/U=<i>/eps=<j>` or
`T-23/<mutant>`), `event`, the exact command line, exit code, wall time, the cell's (U, ε) as rationals, the
expected class (admitted, excluded, boundary, late) and the observed verdict, the bundle's BLAKE3 on each machine,
and on a mismatch the first failing rule and the path to reproduce; a `metric` line gives the counts against
344, 217, 9 and 561 late-packet exclusions.

### Known answers and negative controls

KA-209 (docking cell (¾, ¼), packet on time: `accept` for the inclusion; quantifiers reordered: `reject`),
KA-210 (packet late: `accept` for the exclusion at every U and ε). CX-045, CX-107, CX-114. Amendment known
answer: enlarging the world set by one hidden gain 33/32 reverses the inclusion at (¾, 1/64), the least
worst-case terminal error becoming 1/65 m. Determinism: exact (D0).

## Considerations

- With bounded probes and positive read error, a nondegenerate interval of hidden gains always contains an
  indistinguishable pair, so an exact guarantee fails there and a finite tolerance must be declared; the card is
  about the two-gain model.
- A scenario-tree linear program answers the same family; agreement cell by cell is the mission-side pilot, and
  a tie ships the generic method.

## References

§12.3; C-17, C-18, C-19; KA-209, KA-210; CX-045, CX-107, CX-114; GATE-AUGUR-03 (mission side).
