---
key: "NEG-03"
title: "frob-conformance: the discrepancy register `registries/cards/discrepancies.toml` and the stale-value scan"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "gauntlet", "frob-conformance", "solid", "test", "registry", "card"]
parent: "WP-34"
deps: ["BD-025", "N38b-50", "NEG-01"]
needs: []
related: []
gates: []
plan_refs: ["Appendix E.7", "Appendix M.38", "§38", "Appendix C.3"]
---
## Background

When every substituted number of the 41 worked cards was recomputed from its equation, together with the
enumerations behind them (the Clopper–Pearson tables at n = 64, 86, 128 and 256 with their power rows, the
first reachable tie at each qualification level, the peeking probabilities, the 24,546 ballast classes, the
stance grid's 2,254, 114 and 1,596, the docking grid, the wheel-loop spectral radii, the thermal tangency and
the coverage laws), everything reproduced except twelve items: seven arithmetic disagreements between a chapter
and its own formula, three inconsistencies between chapters that are not arithmetic errors, and two
corrections to carried cards. Each has been settled in the specification. A settled discrepancy can still
return: an implementer copies the first-printed value from memory, a fixture is authored from an old note, a
schema example carries the stale figure. This bead makes each settlement permanent by recording it as data and
scanning the workspace for the withdrawn values. One bead per item (NEG-04 to NEG-14) carries the derivation
and the regression test; this bead carries the register and the scan they share.

Two other lists of the same kind exist, each made by a different recomputation: the reconciliation register
of the known-answer suite (`crates/frob-conformance/registry/reconciliations.toml`, eleven records REC-01 to
REC-11, bead N38b-50) and the discrepancy pins of the killed-idea register
(`crates/frob-conformance/registry/discrepancy_pins.toml`, seventeen records DX-01 to DX-17, bead NAB-41).
Four items here are also records there (E7-1 is REC-07; E7-2 is REC-02 and DX-02; E7-3 is REC-01 and DX-03;
E7-6 is REC-06). The three registers stay separate files, because each records where a value was found and
who settled it, but the search for a withdrawn value in its context is one engine, built in N38b-50 and run
by all three (DESIGN: three searches with three matching rules would each flag the other registers' records,
tests and printouts, which quote the withdrawn values on purpose).

## Scope

In: the register file `registries/cards/discrepancies.toml` and its schema, with the twelve items and their
stale entries; the links from the four shared items to the reconciliation register; the test
`stale_values_absent`, which runs the shared scan over this register's items; the check that every pinning
test exists and is not ignored; the report.

Out, with the bead that has it:
- the scan engine itself (entry form, the 200-character window, the walk set, the exemption list with the
  marker `stale-as-control: <id>`, the hit record, link resolution): N38b-50. This bead registers its register
  as a source of that engine and specifies none of the engine's rules a second time;
- the per-item derivations and tests: NEG-04 to NEG-14;
- the known-answer suite's own list of unreconciled values and the rows that hold the settled values of the
  four shared items: N38b-50 and its value beads (N38b-51 for REC-02, N38b-55 for REC-06, N38b-56 for REC-07).
  They are cross-referenced by `same_as`, not duplicated;
- the discrepancy pins DX-01 to DX-17: NAB-41.

## Technical approach

**Register `registries/cards/discrepancies.toml` (DESIGN).** One table per item:

```toml
[[item]]
id = "E7-1"                      # E7-1 … E7-7 arithmetic rows; E7-A, E7-B, E7-C inconsistencies; E7-K1, E7-K2 carried cards
where = "rare-failure certification: clean episodes at the tenth qualification request for 1e-4"
first_printed = "76,963"
settled = "76,959"
rule = "exact: ceil(ln δ / ln(1 − p*)); the approximation ceil(ln(1/δ)/p*) is not used"
owner_bead = "NEG-04"
first_milestone = "M3"           # the owner bead's milestone: from here the converse and link checks apply
pinned_by = ["gb_32_reference_counts"]          # tests that assert the settled value
stale = [{ pattern = "76[, ]?963", scope = "nemesis|rare|qualif|episode" }]   # the scan's common entry form
governs = "the certification chapter's exact rule"
same_as = ["REC-07"]             # the record of the reconciliation register for the same correction; absent on eight items
value = "76959"                  # on a linked item: the settled number, exact integer or rational, compared with REC's
```

`stale` uses the entry form of the scan engine (N38b-50): `pattern` is a regular expression for the withdrawn
value, `scope` a regular expression that must match in the window around it, and an optional `all_of` lists
phrases that must all occur in the window. The links: E7-1 → REC-07 (`value = "76959"`), E7-2 → REC-02
(`"100/13"`), E7-3 → REC-01 (`"8428/5757"`), E7-6 → REC-06 (`"616"`). Links run from this register to the
reconciliation register only; the engine reports a hit under every linked identifier, the DX records of
NAB-41 that name the same REC record included.

The twelve items:

| Id | Subject | First printed | Settled | Owner |
|---|---|---|---|---|
| E7-1 | Clean episodes, tenth request, 10⁻⁴ | 76,963 | 76,959 | NEG-04 |
| E7-2 | Engine-speed ceiling at share 0.87 | 7.8 | 7.7 from 0.87; 7.8 only from 82/94 | NEG-05 |
| E7-3 | Zero-cost solver ceiling | 1.345/0.94 "94% shared" | 842.8 ÷ 575.7 = 1.46; share attribution withdrawn | NEG-06 |
| E7-4 | Referee share for the humanoid | 0.5 M → 1,000 world-s/s, 0.5 to 2.4% | 1.7 M → 3,400, 0.14 to 0.71% | NEG-07 |
| E7-5 | Ladder cost with six survivors | 2.6 h with the audit lane | 2.0 h before it; 2.2 h with a tenth; 3.0 h with 19 audits | NEG-08 |
| E7-6 | Every-thirteenth-step decisions in 20 s | 615 | 616 (against 600) | NEG-09 |
| E7-7 | Wheel-loop spectral radius, instantaneous-current model | 2.7075 beside 1.4820 and 0.9207 | 2.6917 with the fixture's damping; 2.7075 without | NEG-10 |
| E7-A | Default capacities | C = 24, R = 128; C = 40, R = 224 | C = 32, R = 160; C = 64, R = 320 | NEG-11 |
| E7-B | Squeeze-film tolerance | 1% at the scene gate | 2% at both gates | NEG-12 |
| E7-C | Energy-budget labels | 0.25 J stored, 0.10 J reserve | 0.25 J external, 0.10 J stored, reserve a further 0.10 J | NEG-13 |
| E7-K1 | Worst spacing in an 8 m cell | 0.48 µm | 0.24 µm (centred offset) | NEG-14 |
| E7-K2 | Sideways cost of a 0.10 m shift | 0.57 m only | 0.57 m small-angle and 0.55 m exact | NEG-14 |

**The stale entries of the twelve items (DESIGN, written here at M0; each owner bead states why its entry has
its form).** In TOML basic strings, so a regular-expression backslash is doubled:

```toml
# E7-1 (NEG-04)
stale = [{ pattern = "76[, ]?963", scope = "nemesis|rare|qualif|episode" }]
# E7-2 (NEG-05)
stale = [{ pattern = "1 ?/ ?\\(1 ?[−-] ?0\\.87\\) ?= ?7\\.8\\b", scope = "step share|Amdahl|87 ?%" },
         { pattern = "about 7\\.8\\b", scope = "step share|Amdahl|87 ?%" }]
# E7-3 (NEG-06)
stale = [{ pattern = "1\\.345 ?[÷/] ?0\\.94", scope = "solver|ceiling|share" },
         { pattern = "94 ?% of the batch was shared", scope = "solver|ceiling" }]
# E7-4 (NEG-07)
stale = [{ pattern = "\\b0\\.5 ?M world-steps", scope = "referee|shadow|coverage" },
         { pattern = "\\b0\\.5 to 2\\.4 ?%", scope = "referee|shadow|coverage" }]
# E7-5 (NEG-08)
stale = [{ pattern = "\\b2\\.6 ?h\\b", scope = "ladder|survivor|audit" }]
# E7-6 (NEG-09)
stale = [{ pattern = "\\b615\\b", scope = "decision|thirteenth|30 Hz|20 s" }]
# E7-7 (NEG-10): the withdrawn print, 2.7075 after 0.9207 with no damped value between
stale = [{ pattern = "0\\.9207[^0-9]{1,160}2\\.7075", scope = "wheel|spectral|radius|instantaneous" }]
# E7-A (NEG-11): the pairs, never C = 24 or C = 40 alone, and not 27 kB or 72 kB (KA-348 keeps them)
stale = [{ pattern = "\\bC = 24\\b", all_of = ["R = 128"] }, { pattern = "\\bC = 40\\b", all_of = ["R = 224"] }]
# E7-B (NEG-12)
stale = [{ pattern = "(^|[^0-9.])1 ?%", all_of = ["squeeze", "GATE-HAB-06"] }]
# E7-C (NEG-13)
stale = [{ pattern = "\\b0\\.25 ?J[^0-9;,.]{0,12}stored", scope = "budget|energy|absorber|stop" }]
# E7-K1 (NEG-14)
stale = [{ pattern = "\\b0\\.48 ?[µμu]m", scope = "cell|offset|anchor|spacing" }]
# E7-K2 (NEG-14)
stale = [{ pattern = "\\b0\\.57 ?m\\b", scope = "sideways|lateral|shift" }]
```

Every entry has a `scope` or an `all_of`, as the engine requires. E7-C has no entry for "0.10 J … reserve":
after the settlement the reserve is a further 0.10 J, so "0.10 J reserve" is correct text (GB-26's row "Same
with 0.10 J reserve"); the superseded labelling of that term is rejected by the checker on NEG-13's fixture,
not by the scan.

**Lines that state a withdrawn value on purpose.** The engine reads no label and has no negative condition, so
in the walk set every line that states a withdrawn value on purpose, or a correct figure its pattern cannot
tell from the stale one, carries `stale-as-control: <id>` (exemption (d) of N38b-50): a card's planted wrong
golden or control row, a test's planted string, a fixture in the superseded form. The card files that hold
such lines, each marked by its own bead: GB-04 (0.48 µm, E7-K1), GB-05 (0.57 m, E7-K2), GB-09 (7.8, E7-2),
GB-16 (0.5 M world-steps/s, 0.5 to 2.4%, E7-4), GB-18 (1.345 ÷ 0.94, E7-3), GB-23 (615 and 615.38, E7-6),
GB-26 (0.25 J labelled stored, E7-C), GB-32 (76,963, E7-1), GB-35 (2.6 h, E7-5), GB-41 (the superseded
capacity pairs, E7-A). Outside the cards, the owner beads name theirs (KA-348's row and N38b-58's
`byte_model_example` for E7-A, among others). An unmarked line fails `stale_values_absent` from the day its
file lands.

**The scan (`stale_values_absent`).** A test in `frob-conformance` registers this register as a source of the
scan engine of N38b-50 and runs the engine over its `stale` entries. The engine's rules are stated once, in
N38b-50, and are these in outline: it walks `README.md`, `registries/`, `schemas/`, `crates/*/CONTRACT.md`,
`crates/*/tests/`, `crates/*/registry/`, `scripts/e2e/`, `demos/`, `assets/`, `docs/` and report templates
(never `target/`, never the specification document itself); for each entry it reports every match of
`pattern` whose window (200 characters either side, on text with white space collapsed) matches `scope` (the
scope keeps a bare "7.8" in an unrelated table from firing); it exempts the three register files, their
declared printouts (this bead declares the report below), the derivation file a record names, and a line
carrying the marker `stale-as-control: <id>` (a test that uses the stale value as its planted wrong golden).
A hit fails the test with file, line, the item id with every linked id, and the settled value.

Two checks are this bead's own. The converse: every `pinned_by` test exists and is not ignored. The links:
`resolve_links` of the engine is called, so that every `same_as` names a record of the reconciliation
register and every linked `value` equals the figure that record settles.

**Milestones of the items (DESIGN).** This bead closes at M0, but the items' owners land from M0 to M4 (E7-2
and E7-3 at M0; E7-6, E7-7 at M1; E7-4, E7-A, E7-K1, E7-K2 at M2; E7-1, E7-B at M3; E7-5, E7-C at M4), and two
linked records arrive at M1 (REC-06 of N38b-55, REC-07 of N38b-56). So this bead writes all twelve items at M0
with their stale entries, and the scan guards every withdrawn value from M0 on: a stale value does not need its
mechanism to be wrong. The converse check and the link check apply to an item only from its `first_milestone`
(and a link only once its record's bead has landed); before that the item is reported `pending (Mn)`, which is
not a pass and does not fail the build. Each owner bead (NEG-04 to NEG-14) confirms its item's `pinned_by` and
`same_as` when it lands, and its tests are what make the converse check pass.

**Report.** `cards_discrepancy_report` prints one JSON line per item: id, settled value, pinning tests with
their last status, scan hits (zero expected), `first_milestone` of the owning bead, and the state of the
converse and link checks (`pass`, `fail` or `pending (Mn)`).

## Acceptance criteria

- [ ] The register validates against its schema with exactly twelve items and the fields above.
- [ ] The scan passes on a clean tree and fails on each of twelve planted reintroductions (one per item),
      naming the item.
- [ ] A stale value used as a labelled control does not fire; the same value unlabelled does.
- [ ] Removing or `#[ignore]`-ing a pinning test of an item whose `first_milestone` has arrived fails the
      scan; at M0 the due items are E7-2 and E7-3, and the other ten are reported `pending` with their
      milestones while their stale patterns are already scanned.
- [ ] The scan is the engine of N38b-50: this bead adds a source and no second matching rule, walk set or
      exemption list.
- [ ] The four linked items resolve to REC-07, REC-02, REC-01 and REC-06 and their values agree, each from
      the later of its item's milestone and its record's (E7-2 and E7-3 at M0, E7-6 at M1, E7-1 at M3); a link
      edited to a record that does not exist, or a `value` edited to the withdrawn figure, fails the test once
      due.

## Test plan

### Unit tests

- `discrepancy_register_schema`, `discrepancy_register_has_twelve`.
- `stale_scan_detects_planted`: twelve scratch files, each with one stale value in scope.
- `stale_scan_scope_filters`: "7.8 mm" in a dimension table and "615" as a part count do not fire.
- `stale_scan_allows_labelled_control`, `stale_scan_requires_pinning_tests` (a due item without its test
  fails; a pending item without it is reported `pending`).
- `discrepancy_items_milestones`: every item's `first_milestone` equals its owner bead's milestone in the
  bead index, and at a build of M0 exactly E7-2 and E7-3 are due.
- `links_resolve_and_agree`: E7-1, E7-2, E7-3 and E7-6 name REC-07, REC-02, REC-01 and REC-06; 76959, 100/13,
  8428/5757 and 616 equal the figures those records settle (each asserted once due, as above).
- `linked_hit_names_both`: a planted "the Amdahl ceiling 1/(1 − 0.87) = 7.8" (the scope word is part of the
  plant, since both entries require one in the window) is reported once, under E7-2 and REC-02 (and DX-02 when
  the pins of NAB-41 are present).
- `stale_entries_engine_form`: each of the twelve items loads with at least one entry and every entry has a
  `scope` or an `all_of`; on scratch files, "wheel-loop spectral radii 1.4820 and 0.9207; instantaneous model
  2.7075" fires E7-7 and "1.4820, 0.9207, 2.6917; undamped control 2.7075" does not; "the quadruped default
  C = 24, R = 128" fires E7-A and "contact capacity C = 24" does not; "the energy budget files the 0.25 J
  term as stored" fires E7-C and "energy budget: 0.25 J is external work, 0.10 J is stored energy" does not.

The engine's own tests (the window, reflow invariance, exemptions, de-duplication of linked hits) are in
N38b-50; the tests above exercise this register's twelve items through it.

### End-to-end script and logging

Invoked as step 7 of `scripts/e2e/worked_cards.sh` (NEG-02): the scan's result is logged as one `assert` event
per item with `hits`, the files scanned and the pinning tests' status; the summary counts items clean.

### Known answers and negative controls

The settled values are the known answers; each is asserted by the owner bead's test. The planted
reintroductions are the negative controls. Determinism: the scan is a pure function of the tree; files are
visited in sorted order so that the report is byte-identical between runs.

## Considerations

- The scan is a guard against regression, not a proof of absence in prose: a paraphrased stale claim ("just
  under 77 thousand") escapes it, which is why each item also has a test on the computed value.
- Four items (E7-1, E7-2, E7-3, E7-6) are also rows of the known-answer suite's unreconciled list; the suite's
  registry bead (N38b-50) owns those rows and this register points at them by `same_as`.
- The discrepancy pins of NAB-41 are a third register with a similar name. Its file is
  `crates/frob-conformance/registry/discrepancy_pins.toml`; `discrepancies.toml` names this register only.
- New discrepancies found during implementation are added as items with a new id; an item is never deleted.

## References

Appendix E.7; Appendix M.38; Appendix C.3; §38.
