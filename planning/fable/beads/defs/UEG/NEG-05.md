---
key: "NEG-05"
title: "discrepancy E7-2 resolved: the engine-speed ceiling is 7.7 from the share 0.87, and 7.8 only from 82/94"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "tenfold", "frob-conformance", "solid", "test", "card"]
parent: "WP-34"
deps: ["GB-09", "N00-17", "N38b-51", "NEG-03"]
needs: []
related: []
gates: []
plan_refs: ["Appendix E.7 row 2", "§36.4 item 1", "§2.3 item 1", "Appendix M.38 row 2", "Appendix M KA-010, KA-818"]
---
## Background

The first ceiling of the ten-fold thesis is Amdahl's: if the environment step takes a share s of training
wall-clock, an infinitely fast step speeds training by at most 1/(1 − s). The source figure is a published
set of nested rates on one incumbent stack: 94 k env-steps/s for the step, 88 k with inference and 82 k for
training, so s = 82/94 = 0.8723. Two sections printed "1/(1 − 0.87) = 7.8". That equation is false as written:
1/(1 − 0.87) = 7.69. The figure 7.8 is right for the unrounded share (94/12 = 7.83). A rounded input had been
printed beside an output computed from the unrounded one. The specification now prints "about 7.7
(1/(1 − 0.87))", and the card prints both with their inputs. It matters because the ceiling is quoted
wherever a speed mechanism is funded, and a printed equation that does not hold invites every reader to
distrust the other ceilings.

## Scope

In: the print-consistency rule for derived figures and its check; the formatter contract `format_ceiling`
and the report lint; the regression tests of the formatter on this ceiling; the content of register item
E7-2 (its stale patterns, its pinning tests, its link to REC-02).

Out, with the bead that has it:
- the settled truth of the known-answer rows KA-010 and KA-818 (the exact rational 100/13, tolerance 0.05),
  the derivation file `docs/derivations/amdahl-step-share.md` and the record REC-02 of the reconciliation
  register: N38b-51 (a dependency, at M0, because this bead's acceptance resolves item E7-2 to REC-02). This
  bead does not set a row's truth or tolerance; it cites those rows;
- the pin DX-02 of the killed-idea register, which records the same correction: NAB-41;
- the scan engine: N38b-50; the register file that holds item E7-2: NEG-03;
- the ceiling calculators (N00-17); the card (GB-09); the measured incumbent ledger that will replace the
  published share (BD-040, BD-143).

## Technical approach

1. **Values.** s = 82/94 = 41/47 = 0.872340…; 1/(1 − 41/47) = 47/6 = 7.8333…; 1/(1 − 0.87) = 100/13 =
   7.6923…. Both are legitimate; each must be printed with the input it came from.
2. **Print-consistency rule (general, implemented once).** Any report, card or registry row that prints an
   equation "f(a) = b" with rounded operands must satisfy: evaluating f on the printed operands and rounding
   to the printed precision of b reproduces b. The card loader of NEG-01 enforces it for cards (`printed`
   against `value`); this bead adds the same check to the ceiling calculators' report formatter: it prints
   either "7.7 (1/(1 − 0.87))" or "7.8 (from the unrounded share 82/94 = 0.872)", never "1/(1 − 0.87) = 7.8".
3. **Formatter contract.** `format_ceiling(share: Rational, digits: u8)` returns the share rounded to
   `digits`, the ceiling computed from the rounded share, and separately the ceiling from the exact share,
   each labelled. A caller cannot obtain a string that mixes them.
4. **Register and scan.** Item E7-2: stale pattern `1/(1 − 0.87) = 7.8` and "about 7.8" in the scope of
   step-share or Amdahl text; pinned by `gb_09_reference` and the test below; `same_as = ["REC-02"]` and
   `value = "100/13"`, the figure that record settles. The formatter's legitimate second form, "7.8 (from the
   unrounded share 82/94 = 0.872)", is printed on a line the report marks `stale-as-control: E7-2`, so the
   scan (the engine of N38b-50, which honours the marker for every linked record) does not flag it.

## Acceptance criteria

- [ ] The formatter returns ("0.87", "7.7", "7.8 from 82/94") for the share 41/47 at two digits.
- [ ] A report line that pairs 0.87 with 7.8 without the "unrounded" label is rejected by the report lint.
- [ ] The stale-value scan finds no hit for E7-2, and item E7-2 resolves to REC-02 with the value 100/13.
- [ ] When the measured incumbent share exists, the same formatter prints its ceiling and the published one
      side by side, each with its source tag.

## Test plan

### Unit tests

- `ceiling_from_rounded_share`: the formatter's output for the rounded share: 100/13 rounds to 7.7 at two
  significant digits.
- `ceiling_from_exact_share`: the formatter's output for the exact share: 47/6 rounds to 7.8. (The register's
  own recomputation of 100/13 and 94/12 = 47/6 is `rec_02_from_rounded_share` and `rec_02_from_raw_rates` of
  N38b-51; these two tests are of the formatter's rounding and labelling, not of the record.)
- `formatter_never_mixes`: property test over 10³ random shares in (0.5, 0.99): the printed ceiling always
  equals the ceiling recomputed from the printed share at the printed precision.
- `report_lint_rejects_mixed_line`: the string "1/(1 − 0.87) = 7.8" fails; "about 7.7 (1/(1 − 0.87))" passes.
- `second_stack`: s = 347/1,200 prints 0.29 and 1.4 (1/(1 − 0.29) = 1.408; from the exact share 1.407): both
  round to 1.4, so no label is needed, and the formatter says so.

### End-to-end script and logging

Covered by `scripts/e2e/worked_cards.sh` (NEG-02): GB-09's outputs include both ceilings; the stale-value step
logs item E7-2 with zero hits; the log shows the share as a rational and both rounded forms.

### Known answers and negative controls

KA-010 (engine-speed ceiling: about 7.7 at s = 87%); KA-818 (Amdahl limits on incumbent stacks: at most
1/(1 − 0.87), about 7.7). Control: the mixed line. Determinism: rational arithmetic, D0.

## Considerations

- The published rates are FACT about another stack at the mark their reading earned; the binding share is the
  one this programme measures on its own machine before any mechanism is funded against it.
- The print rule applies to every derived figure in reports, not only this one; this bead is where it is
  first enforced.

## References

Appendix E.7 row 2; §36.4 item 1; §2.3 item 1; Appendix M.38 row 2; KA-010, KA-818.
