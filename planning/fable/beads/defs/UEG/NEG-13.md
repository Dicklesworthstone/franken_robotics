---
key: "NEG-13"
title: "inconsistency E7-C resolved: energy-budget labels: 0.25 J external work, 0.10 J stored, reserve a further 0.10 J"
type: "task"
priority: 2
milestone: "M4"
labels: ["m4", "reflex", "safety", "frob-shield", "solid", "test", "card"]
parent: "WP-65"
deps: ["NEG-03", "GB-26", "N22b-10", "BD-337"]
needs: []
related: ["N22b-14", "N22b-18", "SPEC-C-31"]
gates: []
plan_refs: ["Appendix E.7 (energy-budget labels)", "§22.11", "C-31", "Appendix L (worked budget)", "Appendix M KA-153, KA-568"]
---
## Background

The energy-budget safety claim adds up every energy that can reach an absorber: kinetic energy at the cap
speed, drive energy until cutoff, stored energy, external work done by gravity over the stroke, any other
term, and a reserve; the sum must not exceed the least work the absorber takes. In the worked example the
terms are 0.64 J kinetic, 0.50 J drive, 0.25 J and 0.10 J, against a 2.00 J absorber. The construction's test
called the 0.25 J term stored energy and the 0.10 J term reserve. The control chapter calls 0.25 J external
work (gravity on an 8% slope over the 40 mm stroke: 8 × 9.81 × 0.040 × 0.08) and 0.10 J stored energy (a
series spring), with reserve as a further 0.10 J. Settled: the control chapter's labels govern; the total is
1.49 J without the reserve, and the reserve shortens the longest admissible cutoff delay from 50.5 ms to
45.5 ms. Labels are not cosmetic here. The checker rejects a budget whose port graph has a gravity port or a
spring port with no matching term, so a term filed under the wrong label either hides a missing source or
fails a correct budget; and the reserve is a design margin, not a physical source, so it must not be able to
stand in for one.

## Scope

In: the term taxonomy of the `.fenergy` budget and its checker rule; the worked budget as a fixture with the
governing labels; the regression test. Out: the format (N22b-10); `shield.budget` and the monitor (BD-337);
the proof kind (N22b-14); the card (GB-26).

## Technical approach

1. **Term kinds (binding).** `kinetic`, `drive` (∫ max(P_elec, 0) dt until cutoff), `stored` (elastic or
   other internal storage that can discharge into the stop), `external` (work by gravity or another external
   field over the stroke), `other`, `reserve` (a margin with no source). Each non-reserve term names the port
   of the port graph it accounts for.
2. **Checker rule.** Every port of the graph that can deliver energy toward the absorber has exactly one
   non-reserve term; `reserve` names no port and cannot satisfy a port's requirement; a term whose kind does
   not match its port's kind (a gravity port accounted as `stored`) is rejected.
3. **The worked budget with governing labels (exact).** kinetic 0.64 J; drive 0.50 J; external 0.251136 J;
   stored 0.10 J; total 1.491136 J; absorber 2.00 J; longest cutoff delay (2.00 − 0.991136)/20 W = 50.4432 ms.
   With reserve 0.10 J: 1.591136 J and 45.4432 ms.
4. **The superseded labelling as a fixture.** The same numbers with 0.25 J as `stored` and 0.10 J as
   `reserve`: the gravity port has no term and the spring port has none, so the checker rejects it for two
   uncovered ports, although its arithmetic total (1.49 J) is the same.
5. **Register and scan.** Item E7-C: stale phrasing "0.25 J … stored" and "0.10 J … reserve" in the scope of
   energy-budget fixtures; pinned by `gb_26_reference` and the tests below.

## Acceptance criteria

- [ ] The `.fenergy` schema has the six term kinds; non-reserve terms require a port reference.
- [ ] The governing fixture is admitted with total 1.491136 J and delay 50.4432 ms; with reserve, 45.4432 ms.
- [ ] The superseded labelling is rejected naming the two uncovered ports.
- [ ] A budget in which `reserve` is the only term covering the spring is rejected.
- [ ] The worked budget of the artifacts appendix and the card use the governing labels; the stale-value scan
      finds no hit for E7-C.

## Test plan

### Unit tests

- `energy_terms_taxonomy`: the six kinds parse; an unknown kind is refused.
- `energy_governing_fixture_admitted`: totals and both delays in exact decimals.
- `energy_superseded_labels_rejected`: two uncovered ports reported.
- `energy_reserve_cannot_cover_port`.
- `energy_kind_must_match_port`: gravity port with a `stored` term is rejected; with `external` accepted.
- `energy_delay_arithmetic`: the reserve shortens the delay by exactly 0.10 J/20 W = 5 ms.

### End-to-end script and logging

`scripts/e2e/shield_energy_budget.sh` (N22b-18) drives `shield.budget` on worked cards and refusals; this bead
adds the superseded-label fixture to its refusal table. `scripts/e2e/worked_cards.sh` (NEG-02) covers GB-26
and the stale-value step for item E7-C. Log lines list each term with kind, port, value and the verdict.

### Known answers and negative controls

KA-153 and KA-568 (the budget arithmetic: 0.64 + 0.50 + 0.25 + 0.10 against 2.00 J). Controls: the superseded
labelling; a reserve used as a source; a budget omitting the gravity or spring term (rejected, per the card's
flip). Determinism: exact decimal arithmetic, D0.

## Considerations

- The arithmetic total is identical under both labellings, which is why this could not be caught by a sum
  check and needs the port rule.
- Whether an 8% slope is the right worst case is a property of the site declaration, not of the labels; the
  budget records the slope's provenance.

## References

Appendix E.7 (energy-budget labels); §22.11; C-31; Appendix L; KA-153, KA-568.
