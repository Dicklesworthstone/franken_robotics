---
key: "NEG-13"
title: "inconsistency E7-C resolved: energy-budget labels: 0.25 J external work, 0.10 J stored, reserve a further 0.10 J"
type: "task"
priority: 2
milestone: "M4"
labels: ["m4", "reflex", "safety", "frob-shield", "solid", "test", "card", "review-notes-open"]
parent: "WP-65"
deps: ["BD-337", "GB-26", "N22b-10", "N22b-12", "N22b-18", "NEG-03"]
needs: []
related: ["SPEC-C-31"]
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
1.491136 J without the reserve (1.49 J printed, from the gravity term entered as its printed 0.25 J), and the
reserve shortens the longest admissible cutoff delay from 50.4432 ms to 45.4432 ms (50.5 ms and 45.5 ms
printed); GB-26 states the three readings of the gravity term and asserts both. Labels are not cosmetic here. The checker rejects a budget whose port graph has a gravity port or a
spring port with no matching term, so a term filed under the wrong label either hides a missing source or
fails a correct budget; and the reserve is a design margin, not a physical source, so it must not be able to
stand in for one.

## Scope

In: the resolution of item E7-C as a regression. That is: the worked drop-stop budget (the cart of the worked
card) as a `.fenergy` fixture with the governing labels and its exact totals and delays; the same numbers
under the superseded labels as a fixture that must be rejected; the register entry and the stale-value scan
over energy-budget fixtures, the card and the worked budget of the artifacts appendix.

Out, with the bead that has it: the term keys of the format (N22b-10: `kinetic_ub`, `drive`, `external`,
`stored`, `reserve_j`); the checker rule that every source is covered by exactly one term of the right kind
and that the reserve covers none, with the port-graph walk and refusal FROB-REF-451 (N22b-12: the one
definition; this bead depends on it and runs its two fixtures through that walk, adding no kind and no rule);
`shield.budget` and the monitor (BD-337); the proof kind (N22b-14); the card (GB-26); the golden fixture of
the artifacts appendix, the pass-counter budget of 18 kg and 4.50 J, which is a different budget and already
uses the governing labels (ART-L5; this bead's scan reads its specimen and writes nothing to it).

## Technical approach

1. **Term kinds, as the format and the walk have them.** The construction's inequality names kinetic, drive
   (∫ max(P_elec, 0) dt until cutoff), stored (elastic or other internal storage that can discharge into the
   stop), gravity, other (every other actuator on the load path) and reserve (a margin with no source). In the
   `.fenergy` format these are the five keys `kinetic_ub`, `drive`, `stored`, `external` (gravity or another
   external field over the stroke, and the "other" actuators) and `reserve_j`; there is no sixth key `other`
   and no separate schema of kinds. Each `stored` and `external` entry names its source. This mapping is fixed
   in N22b-12 and only restated here so that the fixtures below can be read.
2. **Checker rule (N22b-12's, exercised here).** Every source that can deliver energy toward the absorber has
   exactly one term of the kind its classification requires; `reserve_j` names no source and cannot satisfy
   one; a term whose kind does not match its source (gravity accounted under `stored`) does not cover it.
3. **The worked budget with governing labels (exact).** kinetic 0.64 J; drive 0.50 J; external 0.251136 J;
   stored 0.10 J; total 1.491136 J; absorber 2.00 J; longest cutoff delay (2.00 − 0.991136)/20 W = 50.4432 ms.
   With reserve 0.10 J: 1.591136 J and 45.4432 ms.
4. **The superseded labelling as a fixture.** The same numbers with 0.25 J as `stored` and 0.10 J as
   `reserve`: the gravity port has no term and the spring port has none, so the checker rejects it for two
   uncovered ports, although its arithmetic total (1.491136 J; 1.49 J printed) is the same.
5. **Register and scan.** Item E7-C: stale phrasing "0.25 J … stored" and "0.10 J … reserve" in the scope of
   energy-budget fixtures; pinned by `gb_26_reference` and the tests below.

## Acceptance criteria

- [ ] The two fixtures are written in the format's five term keys (no `other` key, no kind field), and each
      `stored` and `external` entry names its source.
- [ ] The governing fixture is admitted with total 1.491136 J and delay 50.4432 ms; with reserve, 45.4432 ms.
- [ ] The superseded labelling is rejected by the walk of N22b-12 with FROB-REF-451, naming the two uncovered
      sources (gravity and the series spring).
- [ ] A budget in which `reserve_j` is the only allowance for the spring is rejected (N22b-12's
      `reserve_cannot_cover_source`, run here on the cart fixture).
- [ ] The worked budget of the artifacts appendix and the card use the governing labels; the stale-value scan
      finds no hit for E7-C.

## Test plan

### Unit tests

- `energy_fixtures_use_format_keys`: both fixtures parse under `frob.energy/1` with the five term keys; a
  fixture with a key `other` under `terms` is refused by the format's reader as an unknown key.
- `energy_governing_fixture_admitted`: totals and both delays in exact decimals.
- `energy_superseded_labels_rejected`: two uncovered sources reported, code FROB-REF-451.
- `energy_reserve_cannot_cover_port` and `energy_kind_must_match_port` (gravity under `stored` rejected; under
  `external` accepted): the rule's own tests are N22b-12's `reserve_cannot_cover_source` and
  `kind_must_match_source`; these two run the same walk on the cart fixture as the regression for E7-C.
- `energy_delay_arithmetic`: the reserve shortens the delay by exactly 0.10 J/20 W = 5 ms.

### End-to-end script and logging

`scripts/e2e/shield_energy_budget.sh` (N22b-18, on which this bead depends) drives `shield.budget` on worked
cards and refusals; this bead adds the stage `labels_e7c` to it, with that script's spellings: `frob shield
budget drop-stop-cart-slope --boundary dock_stop_A --reading-max 0.39 --key e7c-gov --json` on N22b-18's
product-reading cart, whose declared terms use the governing labels (admitted with the finding
`declared-below-derived`, exit code 2, as N22b-18's own step asserts; total 1,491,136 at 10⁻⁶ J, delay
50,443 µs; with `--reserve 0.10 --key e7c-res`, 45,443 µs) and the same cart with the superseded labelling, a
variant fixture this bead adds beside it (same options, key `e7c-old`: exit code 3, FROB-REF-451, the gravity
and spring ports named as uncovered). `scripts/e2e/worked_cards.sh` (NEG-02) covers GB-26 and the stale-value
step for item E7-C. Log
lines list each term with kind, port, value and the verdict, the command with its key, and the refusal's
pointer.

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
