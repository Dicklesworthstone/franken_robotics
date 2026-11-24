---
key: "N38-CX"
title: "frob-conformance: counterexample entries stated by the Gauntlet chapter: sixteen shortcuts with cases"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "gauntlet", "frob-conformance", "solid", "registry", "test"]
parent: "WP-14"
deps: ["BD-025", "N38-KA", "N38a-40", "N38b-05", "N38b-28", "STD-E2E", "STD-TEST"]
needs: []
related: ["N38b-04", "N38b-23", "N38b-39", "N38b-40", "N38b-41", "N38b-42", "N38b-43", "N38b-44", "N38b-45", "N38b-46", "N38b-57"]
gates: ["GATE-GAUNT-01", "GATE-FID-09", "GATE-GAUNT-11"]
plan_refs: ["§38.17", "§38.18", "§38.11", "§38.6", "Appendix M.37", "Appendix M.38", "Appendix N.1", "Appendix N.35"]
---
## Background

FrankenRobotics keeps a counterexample catalogue: for every plausible shortcut that is wrong, the smallest
hand-checkable case that breaks it and the rule that exists because of the case. Each entry seeds at least one
negative control (a planted defect that a gate's test must catch). The printed catalogue ends at CX-384 and has
no section for the verification subsystem's own chapter (the Gauntlet). That chapter nevertheless states
sixteen shortcuts with cases: the fourteen negative controls of the known answers it adopts, and two about the
negative-control registry itself. This bead enters them so that none is lost, in the catalogue's form: the
tempting shortcut, the smallest case with its numbers, the rule it protects, where it is defined. Numbers
CX-385 to CX-400 are reserved in table order.

Two entries already in the catalogue protect rules of this chapter and are cited, not duplicated: CX-002
("treat a closed bead as an accepted capability": 87 beads closed within 12 seconds with one shared close
reason, serving figures from a stand-in; rule: typed close reasons and live-dependency proofs) and CX-257 ("a
learning gate passes when the pipeline runs": an exported all-zero policy head passes a plumbing test; rule:
behaviour, not plumbing).

## Scope

In: sixteen `[[cx]]` records with kind of control and gates; the test that each case's numbers reproduce. Out:
the known-answer rows (N38-KA); running the controls (N38b-04, N38b-46); the catalogue loader and seeding
(N38b-05).

## Technical approach

Records go to `crates/frob-conformance/registry/counterexamples.toml`; each seeds one NC record of the kind
shown, attached to the gate shown.

| CX | The tempting shortcut | The smallest case that breaks it | The rule it protects | Control kind · gate |
|---|---|---|---|---|
| CX-385 | Read a winding resistance as referenced to the ambient temperature, or hold it constant | I = 25.4 A, R₀ = 0.54 Ω at 20 °C, α = 0.00393 K⁻¹, C = 140 J/K, G = 1/2.2 W/K, start 25 °C, limit 120 °C: the limit is reached at 33.49 s; with R₀ read at 25 °C the answer is 34.08 s, and with constant resistance 39.92 s | The reference temperature is part of a card; thermal runaway is computed with temperature-dependent resistance | `mutation` · GATE-FID-09 |
| CX-386 | Copy a soil exponent from a neighbouring table row | With k_c = 0.41 kN/m^1.5, k_φ = 21.6 kN/m^2.5, n = 0.5, two tracks 0.55 m long and g = 9.81 m/s², 45 kg on 8 cm wide tracks, 75 kg on 8 cm and 75 kg on 12 cm sink 3.52, 9.79 and 4.96 cm; with n = 0.7 the same constants give 9.16, 19.01 and 11.71 cm (0.091646, 0.190127 and 0.117076 m, computed from the unrounded pressure) | A terrain card's exponent and moduli are one unit-consistent set | `mutation` · GATE-FID-09 |
| CX-387 | Count contact strata under the mechanical symmetry group when a sensor breaks the symmetry | Four feet with three contact states each: 27 strata under the four-element leg-permutation group, 45 under the two-element subgroup a forward-facing sensor leaves; 162 against 324 one-foot transitions | Quotients use the observation-preserving subgroup, not the mechanical group | `mutation` · GATE-FID-09 |
| CX-388 | Call a root real when its imaginary part is below a tolerance | A planted near-real conjugate pair passes a tolerance test and is not one real root; on instances with 40 and 16 real roots the conjugate-ball test certifies exactly 40 and 16 | Reality of a root is certified by ball disjointness, never by a tolerance | `mutation` · GATE-FID-09 |
| CX-389 | Admit a wheel at a lip on drive torque alone | W = 25 N, lip h = 12 mm, a = √(2rh − h²), b = r − h: a wheel of radius r = 60 mm needs M = W·a = 0.900 N·m and μ ≥ a/b = 0.7500; one of radius 75 mm needs 1.017 N·m and μ ≥ 0.6459. With μ = 0.70 and a 1.2 N·m cap a torque-only check admits both; only the 75 mm wheel is feasible | Lift-off needs both the couple and the friction demand | `planted-design` · GATE-FID-09 |
| CX-390 | Compute a variance as the mean of squares minus the squared mean when merging shards in single precision | Shards of constants 4,095 and 4,097: the true variance is 1; the raw form returns 0 because 4,096² + 1 is not representable; likewise 8,191 and 8,193 | Statistics are merged from (count, mean, sum of squared deviations) by the exact pairwise formula | `mutation` · GATE-GAUNT-11 |
| CX-391 | Rank two arms on their complete cases | Baseline 60 of 100 valid; candidate 2 of 2 valid with 98 unresolved: the complete-case rate is 100% while the success rate lies anywhere in [2%, 100%] | Unresolved outcomes are reported per arm and bound the rate; a preference needs complete pairs | `planted-input` · GATE-GAUNT-11 |
| CX-392 | Admit a writable view when every address is in bounds | A 2×2 view with element strides (1, 1): indices [0, 1] and [1, 0] address one element, so stores of 7 and 11 leave a result that depends on their order | A write needs an injective index map, a declared reduction or separate storage | `planted-input` · GATE-GAUNT-11 |
| CX-393 | Infer that a certified root is real because the test also passes at its real part | f(z) = z³ + z at z = 0.001 + 1.001·i: the test at z certifies +i; the test at Re z certifies 0, a different root; ‖z − Re z‖ = 1.001 exceeds 1/(20·γ) = 0.033 | Reality needs the robust test with the distance condition | `mutation` · GATE-GAUNT-11 |
| CX-394 | Pair the Cayley rotation with the translation unchanged | ω = (0, 0, 2), v = (1, 0, 0): the true translation is (I − ω̂/2)⁻¹·v = (1/2, 1/2, 0); the shortcut returns (1, 0, 0) and fails the differential test | The rigid-motion Cayley map transforms the translation | `mutation` · GATE-GAUNT-11 |
| CX-395 | Erode a safe cell by the error set E when cells hold measured states | Cell [0, 1], true = measured + e, e in [0, 0.1]: the right answer is erosion by −E, [0.1, 1.0]; erosion by E gives [0, 0.9], admitting 0.05 (measurable as −0.05) and rejecting 0.95 | The erosion is by −E, which equals E only for a centrally symmetric error set | `mutation` · GATE-GAUNT-11 |
| CX-396 | Bound a solve error by the computed residual alone | Single precision, H = 3, b = 1: x̂ = 0.33333334, computed residual exactly 0, true error 9.93 × 10⁻⁹ | The residual bound carries the rounding margin ρ (here 2.4 × 10⁻⁷, bound 7.9 × 10⁻⁸) | `mutation` · GATE-GAUNT-11 |
| CX-397 | Average face mobilities of a film harmonically or geometrically | Cells with thickness (1 mm, 0): the arithmetic mean of cubes is 0.5 mm³; a harmonic or geometric mean is 0, so the front never reaches a dry cell | Face mobility is the arithmetic mean of the cubes | `mutation` · GATE-GAUNT-11 |
| CX-398 | Use the adjoint of the predicted pose itself on the tail of a calibration edge | Predicted pose of sensor 2 in sensor 1 is a translation (1, 0, 0) m; sensor 1 is rotated 0.01 rad about z: the linear residual is (0, −0.01, 0) m with the inverse pose's adjoint and (0, +0.01, 0) m with the wrong one | The tail block of an edge uses the adjoint of the inverse predicted pose | `mutation` · GATE-GAUNT-11 |
| CX-399 | Register a boundary behaviour as a negative control | "A 4 mm film of water is refused" (the stable limit is 2.5 mm at a 2 cm cell and a 2 ms step) tests the limit and passes on a correct build; so do "with a 29 m budget the witness is rejected" and "a tree refuses localization". None can make its row fail | A control is a defect, not a boundary: the build or input in which the limit was removed | `planted-input` · GATE-GAUNT-01 |
| CX-400 | Any miscount will do as the control of a counting rule | For zero observed failures at δ = 0.05 and p⋆ = 10⁻⁴ the exact count is ⌈ln δ/ln(1 − p⋆)⌉ = 29,956; the stated control "ln(1/δ)/p⋆ rounded down" gives 29,957, which errs on the safe side and cannot fail the row. A control that undercounts is needed: ⌊ln δ/ln(1 − p⋆)⌋ = 29,955 leaves (1 − p⋆)^29,955 = 0.050004 > δ | A control must err in the unsafe direction of the rule it tests | `mutation` · GATE-GAUNT-01 |

**Record fields.** `id`, `shortcut`, `case`, `numeric = true`, `plan_case = "numbers"` (each case is printed
with its numbers in the chapter's text), `case_supplied_by` (empty), `rule`, `rule_ids` (the gate shown; for
CX-387 also the construction on stratified contact and the symmetry section; for CX-399 and CX-400 rule 3 of
the control registry), `defined_in`, `same_as` (empty; none shares a fixture with an earlier entry), `label`
(DESIGN arithmetic; CX-388 and CX-390 quote FACTs to be re-read or re-run, so they carry `rerun = false` and are
seeded as obligations until the re-run's receipt sets `rerun = true`), `control_kind`, and `plant`. CX-385 to
CX-398 come in order from the adopted rows KA-836 to KA-849 (N38-KA); each entry's `plant` names the planted
defect with the identifier that row's control cell uses, so that the seeding tool (N38b-05, step 3) appends
the CX-seeded control to the row's `control.nc` and allocates no second control for the same plant. The plant
`id` is `<row bead>:cx-<nnn>` (N38b-05), the row bead being the adopted-known-answer bead that builds the plant,
so the Cargo feature or fixture is `cx-<nnn>`: CX-385 `N38b-40:cx-385`, CX-386 `N38b-43:cx-386`, CX-387
`N38b-40:cx-387`, CX-388 `N38b-45:cx-388`, CX-389 `N38b-40:cx-389`, CX-390 `N38b-41:cx-390`, CX-391
`N38b-40:cx-391`, CX-392 `N38b-41:cx-392`, CX-393 `N38b-45:cx-393`, CX-394 `N38b-39:cx-394` (the unpaired Cayley
translation), CX-395 `N38b-45:cx-395`, CX-396 `N38b-43:cx-396`, CX-397 `N38b-44:cx-397`, CX-398
`N38b-42:cx-398`. CX-399's
plant is a control record whose defect is the product's own refusal (the 4 mm film); CX-400's is the
safe-side count 29,957 offered as the control of the counting row. The entries were numbered from a reserved
block, so no `slug` is kept.

## Acceptance criteria

- [ ] Sixteen records exist with numbers CX-385 to CX-400 (the block the allocation ledger of N38a-40 issues to this bead).
- [ ] Each seeds exactly one NC record of the stated kind, attached to the stated gate, and the seed-completeness
      join passes; for KA-836 to KA-849 the seeding tool reuses these records (one control per plant), so no
      row of N38-KA gains a second control for the same defect.
- [ ] The numbers of each case reproduce in a host test, CX-386's from the unrounded pressure.
- [ ] `scripts/e2e/registry_n38_cx.sh` passes and rejects each of its four planted registry defects.
- [ ] CX-399 and CX-400 are enforced by the control loader through the rules their owners define (this bead
      defines no loader rule): a boundary-only control is rejected `boundary-not-control` (BD-025's loader check),
      and a control whose planted value lies on the safe side of its row is rejected `cannot-fail` (N38b-28's
      rule); each entry's seeded control is caught by its rule in a host test.

## Test plan

### Unit tests

`crates/frob-conformance/tests/catalogue_gauntlet.rs`: `cx_385` to `cx_398` each recompute the shortcut's wrong
value and the true value and assert they differ by more than the row's tolerance (values as in the table);
`cx_399_boundary_rejected`: a control whose defect is the product's own refusal is rejected by the loader;
`cx_400_safe_side_flagged`: 29,957 ≥ 29,956 is flagged, 29,955 is accepted as a control and (1 − 10⁻⁴)^29,955
is computed as 0.0500041; `sixteen_entries_seeded`; `adopted_rows_reuse_cx_controls`: after a seeding run on
a scratch copy, each of KA-836 to KA-849 lists the NC seeded by its CX entry and no control has `seed` KA-836 to
KA-849 with the same plant; `cx_386_widths`: 45 kg on 8 cm gives p = 5.01648 kPa and z = 3.52 cm, and n = 0.7
gives 9.16 cm (0.091646, 0.190127 and 0.117076 m for the three loads, compared to 10⁻⁵ m and computed from the
unrounded pressure; rounding p to 5.017 kPa before the power 1/n would print 9.17 cm, which the test rejects);
`cx_389_radii`: a = 36.000 mm and 40.694 mm at r = 60 mm and 75 mm, giving 0.900 and 1.017 N·m and 0.7500 and
0.6459; `cx_400_counts`: ⌈ln 0.05/ln(1 − 10⁻⁴)⌉ = 29,956, ⌊ln(1/0.05)/10⁻⁴⌋ = 29,957 and the floor 29,955.

### End-to-end script and logging

`scripts/e2e/registry_n38_cx.sh` (this bead; header `bead=N38-CX milestone=M0 labels=gauntlet,registry
plants=drop-plant,unknown-gate,digit-changed,second-control`): runs the test file; re-runs the seeding tool
(`cargo test -p frob-conformance --test seed_controls -- --ignored seed`) and the join (`cargo test -p
frob-conformance --test seed_join`) on a scratch copy of `registry/` and asserts by digest that a second seeding
run is a no-op and that each of KA-836 to KA-849 lists the NC of its CX entry; plants, one at a time on scratch
copies, an entry with its plant removed, an entry naming a gate absent from the register (GATE-GAUNT-99), the
CX-386 literal 9.16 changed to 9.17, and a second control with seed KA-840 and CX-389's plant, and asserts each is
rejected. It calls no `frob` verb. It sources the STD-E2E helper and writes
`target/e2e-logs/registry_n38_cx/<utc-timestamp>.ndjson` with a transcript: every line has `ts`, `script`,
`bead`, `step`, `event`; `cmd` lines with the exact command line, exit code and wall time; the evidence block
(claim Known answer, class Tested, determinism D0); `metric` lines with entries, controls seeded and reused, and
obligations; the catalogue and scratch-controls digests before and after the second seeding; per planted defect
an `assert` with the expected and observed failing check; on failure the first differing entry and the scratch
path; a `summary` line naming GATE-GAUNT-01, GATE-FID-09 and GATE-GAUNT-11. The controls themselves are run by
`scripts/e2e/adopted_known_answers.sh` (N38b-46) for CX-385 to CX-398 and by `scripts/e2e/control_runner.sh`
(N38b-04) for CX-399 and CX-400; each logs the control's outcome with the observed and expected values.

### Known answers and negative controls

Each entry is the seed of a negative control; the entries' own check is that the shortcut's value is wrong by
more than the tolerance. Determinism: D0.

## Considerations

- Milestone M0 (moved from M1 in polishing round 1): CX-394, the unpaired Cayley translation, seeds the control
  of the adopted row KA-845, which is due at M0 through N38b-39; the entries are registered at M0 and each
  control is runnable when its row's test lands.
- The same case may protect another rule in its owner chapter's part of the catalogue; each statement keeps
  its entry, because the registry holds one control per protected rule.
- A counterexample found in use (a false certificate, a false promotion, a withdrawn result) is appended after
  these and becomes a permanent control.

## References

§38.17, §38.18, §38.11 (rule 3), §38.6 (rule 3), Appendix M.37, M.38 (row 8), Appendix N.1, N.35; CX-002,
CX-257.
