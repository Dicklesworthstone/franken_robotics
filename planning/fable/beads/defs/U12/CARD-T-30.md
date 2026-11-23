---
key: "CARD-T-30"
title: "Theorem card T-30: an indistinguishability witness is valid (identical histories, disjoint continuations)"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "lemma", "frob-cert", "frob-sense", "frontier", "test"]
parent: "WP-42"
deps: ["BD-132", "N12-06", "SPEC-C-19"]
needs: ["§0.7: the paper-scale pilot of the information construction (the docking family against an independently written scenario-tree linear program, and one planted stale-read race), owned by the mission and sensing work (BD-041, BD-129)"]
related: ["CARD-T-23", "BD-240", "BD-290", "BD-098", "BD-134"]
gates: []
plan_refs: ["§12.3 (T-30)", "§10 C-19", "§20.12", "§25.10", "§25.5.2 (alias-pair)"]
---
## Background

Every datum that crosses from physics to a controller passes seven distinct instants: sample, exposure,
timestamp, arrival, decision, command and actuation. A causal policy is a map from the history of arrived
packets to commands. An indistinguishability witness is a pair of admitted worlds w₁, w₂ and a commitment
deadline such that, under every admitted command prefix up to the deadline, the two worlds deliver identical
packet histories, while the sets of commands that succeed from the deadline on are disjoint. A witness proves
that no policy in the causal class succeeds in both worlds, whatever its training. It is how the studio says
that training cannot help and what to change instead: extract the information (a better sensor), move to
acquire it (a probing action or a mount change), or change the mechanics so that the information is unnecessary.

Statement (T-30). An indistinguishability witness is valid: two admitted worlds produce identical packet
histories under the stated command prefix and have disjoint successful continuations.

Route: exact replay of both worlds in an integer or rational kernel class; `Tested` if the worlds run in
floating point. Tier: Frontier for witness search; checking a supplied witness is Solid. The exactness matters:
"identical packet histories" is an equality of data, and equality in floating point across two worlds is a test
outcome, not a certified fact. Rejected alternatives: sensor models that are a noise level and a delay; the
belief that more training can overcome any sensing limit.

## Scope

In: statement document, registry row, the `alias-pair` certificates bound to this card, the class rule (exact
kernel class versus floating point), fixtures and mutants. Out: the checker (BD-132); searching for witnesses
(the alias-witness search); the packet model and clock contract; label counts and schedule classes (other
objects of the same construction).

Order of work. This card is finished before the code that implements or uses its theorem and never depends on it:
the late-packet alias witness of `frob-sense` (BD-134) is `related`, not a dependency (a card that waited for its
implementer would close a loop, because the implementer's own acceptance runs the card's law tests). What the card
does depend on is in `deps`: the card registry, its specification, its pilot where the route is Frontier, and
Lemma's own proof machinery. It closes on its statement document, its registry row, its specimens and mutants, and
every acceptance line that its own fixtures decide. An acceptance line that can only be observed on live output
(witnesses found by the alias mechanism and their export by the mission verbs) is run inside the implementing
bead, with this card's law tests, as part of that bead's acceptance; the receipt produced there is what moves the
registry row for that output.

## Technical approach

1. A witness in a bundle (kind `alias-pair`) carries: the two worlds as differences from the query's world set
   (for example the hidden gain 1 or 2); the commitment deadline as an instant of the clock contract; the scope
   (all admitted command prefixes, or one named experiment); the packet histories up to the deadline as
   functions of the command prefix; the success sets of continuations in each world.
2. The auditor replays both worlds exactly. Identity of histories: for a scope of all admitted prefixes the
   histories must be equal as functions of the prefix (for the linear, rational fixtures: equal coefficients; for
   finite games: equal at every vertex of the prefix set together with the stated linearity argument); for a
   named experiment, equal for that prefix. A packet is visible in a history only if its arrival is not later
   than the deadline; the timestamp is a label and is not used.
3. Disjointness of successful continuations: the two success sets, as sets of command continuations (boxes or
   polytopes in exact rationals, or finite sets), have empty intersection, checked exactly.
4. Class rule: if both worlds were evaluated in an exact-integer or rational kernel class the claim is
   `Certified`; if either ran in floating point the bundle may only claim `Tested`, and a `Certified` claim over a
   floating-point replay is rejected (FROB-REF-834).
5. With read error: an alias exists when the reading sets of the two worlds intersect; the witness then carries
   the common reading explicitly (in the docking family at equality both worlds can report 15/16 while needing
   −¼ and −¾).
6. Limits: a witness ends a futile policy search for the declared channel and world pair; it does not speed a
   search that can succeed, and it is about the declared model. With stochastic packets the object is different:
   a fixed experiment whose reading has law P in one world and Q in the other caps success at (1 + TV(P, Q))/2
   with equal priors; that bound is not a witness of this card.
7. Artifacts: `docs/math/cards/T-30.md`; registry row; specimens `alias-pair/accept-T-30-*.frobcert` and mutants.

## Acceptance criteria

- [ ] `frob-verify` accepts the late-packet docking witness: gains g ∈ {1, 2}; no informative packet arrives
      before the third command is committed; for a first command p the remaining commands must be
      (1/g − 2p, p − 1/g), which differ by ½ between the gains, so no command sequence succeeds in both worlds.
- [ ] Rejected mutations: histories that differ in one packet before the deadline; success sets that intersect
      (the on-time packet with ε below the bound); a packet counted as visible by its timestamp although its
      arrival is after the deadline; a `Certified` claim over floating-point replays; a scope of "all admitted
      prefixes" supported by one prefix only.
- [ ] Registry row `checked`, class `Certified` for exact replays and `Tested` otherwise, with hashes and checker
      identity.
- [ ] The witness is exported with the three engineering answers named as next actions by the mission verbs (a
      check on the verb's output, owned there; the card supplies the certificate).

## Test plan

### Unit tests

`crates/frob-cert/tests/cards_t30.rs`: `late_packet_alias_accepts`; `continuations_differ_by_half`;
`on_time_small_error_is_not_an_alias` (ε = ¼ at U = ¾: reading sets disjoint, so the alias claim is rejected);
`equality_reading_shared` (ε = 5/16, p = 5/8: 15/16 in both worlds); `history_mismatch_rejects`;
`timestamp_not_arrival_rejects`; `float_replay_cannot_be_certified`; `scope_all_prefixes_needs_argument`;
`range_anchor_ambiguity` (a related fixture: range anchors at (−1, 0) and (1, 0) both read √2 from (0, 1) and
from (0, −1); a third anchor at (0, 2) reads 1 or 3, and closed reading intervals of half-width ε separate
exactly when ε < 1).

### End-to-end script and logging

`scripts/e2e/lemma_second_machine.sh` (bead N12-35) ships the late-packet exclusions as bundles and re-checks
them; `lemma_auditor_mutations.sh` (bead N12-34) runs the mutants.

### Known answers and negative controls

KA-210 (docking, packet late: `accept` for the exclusion at every U and ε). CX-105 (a decision may not use any
packet whose timestamp is earlier than the decision: the timestamp is a label written on the sensor's own clock;
arrival governs). Repeated readings as a related known answer: a binary probe correct with probability ¾ gives
27/32 with three independent reads, 459/512 with five and 3807/4096 with seven, and stays at ¾ after any number
of reads that share one fault bit. Determinism: exact for certified witnesses.

## Considerations

- Kill gates of the owning mechanisms (the alias-witness kill gates of the sensing, mission and design work)
  govern search; checking supplied witnesses stays as the fallback.
- A witness is conditional on the admitted world set; enlarging the set can only add witnesses.

## References

§12.3; C-19; KA-210; CX-105; T-23.
