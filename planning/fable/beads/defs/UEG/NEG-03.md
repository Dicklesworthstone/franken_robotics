---
key: "NEG-03"
title: "frob-conformance: the discrepancy register `registries/cards/discrepancies.toml` and the stale-value scan"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "gauntlet", "frob-conformance", "solid", "test", "registry", "card"]
parent: "WP-34"
deps: ["NEG-01", "BD-025"]
needs: ["Appendix M.38: the eleven rows of values that did not reconcile with their own formulas, as registry rows owned by the known-answer suite"]
related: ["NEG-04", "NEG-05", "NEG-06", "NEG-07", "NEG-08", "NEG-09", "NEG-10", "NEG-11", "NEG-12", "NEG-13", "NEG-14", "NEG-02"]
gates: []
plan_refs: ["Appendix E.7", "Appendix M.38", "§38"]
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

## Scope

In: the register file and its schema; the scan and its allow-list rule; the report. Out: the per-item
derivations and tests (NEG-04 to NEG-14); the known-answer suite's own list of unreconciled values, which
overlaps on four items and is cross-referenced, not duplicated.

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
pinned_by = ["gb_32_reference_counts"]          # tests that assert the settled value
stale = [{ pattern = "76[, ]?963", scope = "nemesis|rare|qualif|episode" }]
governs = "the certification chapter's exact rule"
```

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

**The scan (`stale_values_absent`).** A test in `frob-conformance` walks `registries/`, `schemas/`,
`crates/*/CONTRACT.md`, `crates/*/tests/`, `scripts/e2e/`, `demos/`, `assets/` and `docs/` (never `target/`,
never the specification document itself). For each `stale` entry it reports every line that matches `pattern`
and whose surrounding 200 characters match `scope` (the scope keeps a bare "7.8" in an unrelated table from
firing). A hit fails the test with file, line, item id and the settled value. Two allow-list forms: the
register itself, and a line carrying the marker `stale-as-control: <id>` (a test that uses the stale value as
its planted wrong golden). The scan also checks the converse: every `pinned_by` test exists and is not
ignored.

**Report.** `cards_discrepancy_report` prints one JSON line per item: id, settled value, pinning tests with
their last status, scan hits (zero expected), and `first_milestone` of the owning mechanism.

## Acceptance criteria

- [ ] The register validates against its schema with exactly twelve items and the fields above.
- [ ] The scan passes on a clean tree and fails on each of twelve planted reintroductions (one per item),
      naming the item.
- [ ] A stale value used as a labelled control does not fire; the same value unlabelled does.
- [ ] Removing or `#[ignore]`-ing a pinning test fails the scan.

## Test plan

### Unit tests

- `discrepancy_register_schema`, `discrepancy_register_has_twelve`.
- `stale_scan_detects_planted`: twelve scratch files, each with one stale value in scope.
- `stale_scan_scope_filters`: "7.8 mm" in a dimension table and "615" as a part count do not fire.
- `stale_scan_allows_labelled_control`, `stale_scan_requires_pinning_tests`.

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
  registry bead owns those rows and this register points at them.
- New discrepancies found during implementation are added as items with a new id; an item is never deleted.

## References

Appendix E.7; Appendix M.38; §38.
