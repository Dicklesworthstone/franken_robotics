---
key: "N00-KA"
title: "frob-conformance: known-answer registry rows KA-001 to KA-032 (document control, the audit, the thesis, the bets, the North-Star fixtures, the claim product)"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "gauntlet", "frob-conformance", "solid", "registry", "test"]
parent: "WP-34"
deps: ["BD-025"]
needs: ["§38.6: the known-answer registry format and its loader in `frob-conformance` (rows are append-only, INV-GAUNT-07; the printed appendix is generated from the registry, ADR-GAUNT-02)"]
related: ["N00-17", "N00-27", "N00-33", "NS-13", "NS-22", "BD-033", "BD-034", "BD-036", "BD-093", "BD-130", "BD-129", "BD-030"]
gates: ["GATE-GAUNT-02"]
plan_refs: ["Appendix M.1", "Appendix M.2", "Appendix M.3", "Appendix M.4", "Appendix M.5", "Appendix M.6", "Appendix M.7", "Appendix M.37", "Appendix M.38"]
---
## Background

The known-answer suite is the registry of cases whose truth a test can compare against: 835 rows at plan time,
numbered KA-001 onward and never renumbered or reused. This bead carries the first 32 rows, the ones stated by
document control (2 rows), the audit of existing systems (6), the ten-fold thesis (8), the bets (2), the
North-Star scenarios (8) and the claim product (6). The principles, the reader table and the goals state no
known answers. Mechanism beads cite these numbers; this bead makes sure every row exists in the registry with
its truth, tolerance, tier, site and negative control, so that nothing enumerable is lost.

How to read a row. *Tolerance*: `exact` for integers, rationals, Booleans, verdicts and labels; a stated value;
`to set` where none is stated (such a row is an obligation, not a test, and cannot support a gate until the
tolerance is registered). *Tier · site*: the verification tier (G0 laws and self-tests, G1 known answers) and
where it runs: `host` (exact or double-precision arithmetic in a test binary), `auditor` (`frob-verify`),
`referee`, `all tiers`. *Negative control*: the mutation or planted failure the row must catch; `none stated`
means the control is owed (bead N00-33 supplies the fourteen owed here). Labels: fixture constants are DESIGN;
truths derived from them by hand are DESIGN arithmetic with a derivation file under `docs/derivations/`; truths
quoting a planning pilot or a sibling's source are FACT to be re-run or re-read before a gate cites them. No row
is a measurement of this project.

## Scope

In: the 32 registry rows below, one derivation file per row, and the unit test that loads them and checks the
arithmetic rows. Out: the mechanisms the rows test (named in the last column); the controls and tolerances still
owed (N00-33); the registry loader itself (BD-025).

## Technical approach

Rows are written to `crates/frob-conformance/registry/known_answers.toml` (one `[[ka]]` record each: `id`,
`section`, `case`, `truth`, `tolerance`, `tier`, `site`, `control`, `derivation`, `owner_bead`). DESIGN: the
field names follow the columns of the printed suite.

**Document control (2 rows).**

| KA | Case | Truth | Tolerance | Tier · site | Negative control | Tested by |
|---|---|---|---|---|---|---|
| KA-001 | Ceiling pre-check on the recorded solver comparison: a composed solver took 1.345× the time of a specialized one warm and 1.088× cold; the specialized batch held a residual of 575.7 µs the solver does not touch (parsing, assembly, checking, serialization, harness; not separately timed) | No solver could have shown more than about 1.46× (842.8 ÷ 575.7 = 1.4639…); a speed test whose claimed ratio exceeds its computed ceiling is refused before it is locked | `to set` (the text prints 1.46) | G1 · host | A claimed ratio above the ceiling submitted for locking, which must be refused | N00-17, BD-354 |
| KA-002 | Reachable verdicts of a frozen fixed-n rule with 64 pairs | Three of its four verdicts are reachable at n = 64; the fourth (a tie) is first reachable at 86 pairs | exact | G0 · host | A study that needs the fourth verdict at n = 64, which must be refused at admission | BD-124 |

**The audit of existing systems and foundations (6 rows; each is an admission test for a reported sibling defect).**

| KA | Case | Truth | Tolerance | Tier · site | Negative control | Tested by |
|---|---|---|---|---|---|---|
| KA-003 | `fs-mbd` `planar_drive`: heading 32 rad, wheel speeds differing by 6 × 10⁻¹³ m/s, one second of travel | Displacement about (0.834, 0.551) m, checked against an independent SE(2) exponential | `to set` | G1 · host | The reported defect: the routine returns zero translation; the test fails on a commit that has it and passes after the upstream fix | BD-033 |
| KA-004 | `fs_g1_train::ppo_update`: two independently reset one-step episodes concatenated under a frozen policy | The second probability ratio stays 1 | `to set` | G1 · host | The reported defect: the ratio moves from 1 to 1.582459 | BD-034 |
| KA-005 | `ppo_update` with a negative advantage | The clipped surrogate is applied on the correct side of the ratio | exact (verdict) | G1 · host | The reported defect: the clip on the wrong side | BD-034 |
| KA-006 | `ppo_update`: critic gradient into the shared bias | Non-zero, equal to the gradient of the squared-error objective | `to set` | G1 · host | The reported defect: a zero gradient | BD-034 |
| KA-007 | Stride convention between `franken_numpy` and `frankentorch` | The converted stride reads 3 | exact | G1 · host | An unconverted stride, which reads 12 | BD-036 |
| KA-008 | Sustained articulated contact while rolling around a sharp lip | The material witness and the contact frame both change, and the referee's transactional step migrates the witness | exact (verdict) | G1 · referee | A step that assumes fixed material witnesses and a fixed frame | BD-035, BD-093 |

**The ten-fold thesis (8 rows; pure arithmetic, implemented by N00-17).**

| KA | Case | Truth | Tolerance | Tier · site | Negative control |
|---|---|---|---|---|---|
| KA-009 | Completion cap: the candidate misses a fraction q of deadlines | R ≤ 1/q; at q = 12% the ratio cannot exceed 25/3 ≈ 8.33; at q = 1/8 it cannot show ten-fold (cap 8) | exact | G1 · host | none stated (N00-33) |
| KA-010 | Engine-speed ceiling at fixed sample count, step share s | At most 1/(1 − s): about 7.7× at s = 0.87 (100/13), 6.7× at 0.85 (20/3), 1.9× at 0.46 (50/27), about 1.4× at 0.29 (100/71) | exact as a rational; printed values rounded | G1 · host | none stated (N00-33) |
| KA-011 | Design-cycle ceiling: a 120 s cycle with no training | 3.0–3.5× against an incumbent quadruped cycle of 360–420 s; about 15× against a 30-minute humanoid run (1800/120) | exact | G1 · host | none stated (N00-33) |
| KA-012 | One design per agent turn, agent latency t in both arms | At most (300 + t)/t: 6× at t = 60 s, 3.5× at t = 120 s; against 410 s of training (410 + t)/t: 7.83× and 4.42× | exact as a rational | G1 · host | none stated (N00-33) |
| KA-013 | Compute per decision C ÷ (c + c_T·f), C = 410, c = 38, c_T = 410 GPU-s | 410/38 ≈ 10.79 at f = 0; ten needs f ≤ 3/410 ≈ 0.73%; no f reaches ten if c > 41 GPU-s | exact as a rational | G1 · host | none stated (N00-33) |
| KA-014 | General bound, fraction f improved by r with overhead h | 1/(1 − f + f/r + h); 1/0.07425 ≈ 13.47 at f = 0.97, r = 40, h = 0.02; f = 0.85 with r infinite gives 20/3 < 10 | exact as a rational | G1 · host | A pruning ratio multiplied by a speedup measured on work the pruning already removed |
| KA-015 | Pruning trivial failures: 16% of trained candidates degenerate | Trainings fall by at most 1/(1 − 0.16) = 25/21 ≈ 1.19× | exact as a rational | G1 · host | none stated (N00-33) |
| KA-016 | Paired tier correction, tier correlation ρ, free fast-tier samples | Gain 1/(1 − ρ²): 1.96× at ρ = 0.7 (printed 2×), 3.05× at 0.82 (printed 3×), 10.26× at 0.95 (printed 10×) | exact as a rational | G1 · host | none stated (N00-33) |

**The bets (2 rows).**

| KA | Case | Truth | Tolerance | Tier · site | Negative control | Tested by |
|---|---|---|---|---|---|---|
| KA-017 | Omitted-law fixture for calibration bands: a fixture with an omitted law passes every in-family residual check and reverses the decision (recorded pilot: 675 of 675 correct in class, 0 of 225 on the omitted law) | The exchangeability alarm fires or a refusal is returned; no band is issued on the omitted-law cases | exact (verdict) | G1 · host | A band issued on the omitted-law cases | BET-28, BD-393 |
| KA-018 | Boundary screen for strict predicates in single precision: 65,536 boundary cases | Unguarded comparison gives 452 false admissions and 23,438 false exclusions; under the guard with the exact fallback the screen ends with zero errors | exact | G1 · all tiers | The unguarded single-precision comparison, which must show the errors | BET-08, BD-030 |

**The North-Star fixtures (8 rows).** Stance family: 400 N on four point feet at (±a, ±b), a = 1/3 m; centre of
mass anywhere in a rectangle of half-dimensions 150 mm (along a) by 75 mm (along b); each foot's commanded normal
force in [0, 160] N; half-width b free. Docking fixture: a cart starting at rest must reach position 1 at rest in
three one-second ticks; acceleration g·u with unknown gain g ∈ {1, 2}; commands bounded |u| ≤ U; the only
informative packet is the first velocity reading.

| KA | Case | Truth | Tolerance | Tier · site | Negative control | Tested by |
|---|---|---|---|---|---|---|
| KA-019 | Stance, inner set | Every b ≥ 3/14 m is in the inner set, corner allocation (160, 130, 110, 0) N as witness; zero reserve on two constraints at the boundary; `frob-verify` re-checks the bundle offline in exact rational arithmetic | exact | G1 · auditor | none stated (bundle mutations exist elsewhere; N00-33) | NS-13, BD-130 |
| KA-020 | Stance, exclusion | Every b < 3/14 m excluded with a dual vector as impossibility witness; at b = 0.20 m one foot would have to carry 165 N | exact | G1 · auditor | none stated (a perturbed multiplier; N00-33) | NS-13, BD-130 |
| KA-021 | Stance at b = 1/4 m under commanded forces | Admitted with allocation (155, 135, 105, 5) N | exact | G1 · auditor | none stated (N00-33) | NS-13 |
| KA-022 | Stance amended to a passive equal-spring suspension, b = 1/4 m | Springs deliver (175, 115, 85, 25) N against the 160 N rating; `result.revalidate` withdraws the earlier inner set and says what must be re-solved | exact | G1 · host | An inner set carried across the amendment | NS-13 |
| KA-023 | Docking, packet on time, U = 3/4 | A first command of 1/4 identifies the gain and both worlds dock exactly (g = 1: commands 1/4, 1/2, −3/4; g = 2: 1/4, 0, −1/4) | exact | G1 · auditor | none stated (N00-33) | NS-22, BD-129 |
| KA-024 | Docking, tolerable sensor error ε | Feasible exactly when ε < (U + 1/2)/4; choosing probe and feedback together raises the tolerable error from 1/8 to 5/16 on the same actuator | exact | G1 · auditor | none stated (equality at the boundary; N00-33) | NS-22, BD-129 |
| KA-025 | Docking, packet late | No causal controller docks in both worlds | exact (verdict) | G1 · auditor | none stated (N00-33) | NS-22, BD-134 |
| KA-026 | Docking with a third gain 33/32 admitted | It aliases with gain 1 at an error radius of 1/64; exact docking becomes impossible; every label that was feasible reverses | exact | G1 · auditor | none stated (N00-33) | NS-22, BD-129 |

**The claim product (6 rows; implemented by BD-005, exercised by N00-27).**

| KA | Case | Truth | Tolerance | Tier · site | Negative control |
|---|---|---|---|---|---|
| KA-027 | A design comparison consuming a kernel with a Certified group-membership identity, a friction coefficient from a card field marked `assumed` (Estimated), and a paired anytime-valid study on the GPU tier under class D2 (Tested) | Design comparison of class Estimated; weakest input the friction field; tier GPU, so provisional until promoted; reality scope `no-claim` | exact | G0 · host | A product reported at the class of its strongest input |
| KA-028 | Compile-fail tests on the claim-product type | Three things do not exist: a conversion from `ClaimProduct` into a Proved value; a setter for `class`; a public constructor taking a class | exact (verdict) | G0 · host | A crate that adds any of the three, which must fail to compile |
| KA-029 | `ClaimProduct::combine` | Class, tier and determinism take the weakest; scope the intersection; conditions the union; a quantifier-order mismatch is a refusal | exact | G0 · host | Parts with mismatched quantifier order, which must be refused |
| KA-030 | `NoAuthorization` on every exported artifact | `execution_authorized`, `hardware_safety_signoff`, `sim_to_real_certified` each serialize as the JSON literal false; no constructor produces true | exact | G0 · auditor | A bundle with a flag missing or true, which the auditor must reject |
| KA-031 | A bundle whose assumptions are missing | `frob-verify` refuses it | exact (verdict) | G0 · auditor | The bundle with its assumption list removed |
| KA-032 | Quantifier-order fixture: every build has a command that works and no single command works for all builds | "For every world, some controller works" and "one controller works for every world" receive different answers; evidence for the easier order is rejected; the auditor rejects a changed quantifier even when every hash is consistent | exact (verdict) | G0 · auditor | Evidence for the easier order; a bundle with the quantifier changed and consistent hashes |

**Corrections that these rows already carry (regressions to keep).** Three printed values were found not to
reconcile with their formulas and were corrected; the rows above carry the corrected values and the test pins
them: (1) the solver ceiling is 842.8 ÷ 575.7 ≈ 1.46, not 1.345 ÷ 0.94 = 1.43 (the "94% shared share"
attribution is withdrawn); (2) the step-share ceiling at 87% is 1/(1 − 0.87) = 7.69, printed "about 7.7", not
7.8 (which came from the unrounded share 82/94); (3) 8,192 quadruped worlds at 20–53 kB each use 0.7–1.8% of a
24 GB card and 4,096 humanoid worlds at 56–180 kB use 1.0–3.1%, printed "about 1 to 3%", not "1 to 1.5%".

## Acceptance criteria

- [ ] All 32 records load from the registry; each has `id`, `truth`, `tolerance`, `tier`, `site`, `control`,
      `derivation` and `owner_bead`; loading an unknown field or a duplicated or reused number is a load error.
- [ ] A derivation file exists under `docs/derivations/` for each of KA-001, KA-002 and KA-009 to KA-032.
- [ ] The registry prints the count of rows with tolerance `to set` (4: KA-001, -003, -004, -006) and control
      `none stated` (14: KA-009 to -013, -015, -016, -019 to -021, -023 to -026); neither kind is counted as
      passing by GATE-GAUNT-02 until N00-33 closes them.
- [ ] The printed table generated from the registry equals the rows above (`xtask check-generated`).

## Test plan

### Unit tests
`crates/frob-conformance/tests/ka_part0_8.rs`: `registry_has_ka_001_to_032` (32 ids, contiguous, no gaps);
`uncontrolled_rows_are_not_counted` (a synthetic gate run that cites KA-009 before N00-33 lands is reported as
not supported); `corrected_values_pinned` (1.46 not 1.43; 100/13 not 7.8; 0.7–3.1% not 1–1.5%);
`renumbering_is_a_load_error` (a registry copy with KA-010 and KA-011 swapped is rejected);
`printed_table_matches_registry`.

### End-to-end script and logging
`scripts/e2e/ka_part0_8.sh` runs `cargo test -p frob-conformance --test ka_part0_8` and the registry printer,
then asserts the two obligation counts. Logged per STD-E2E: one `assert` line per row with its id, tier, site,
truth, tolerance and whether a control is registered; a `summary` line with rows loaded, rows counted, rows owed,
and gate GATE-GAUNT-02.

### Known answers and negative controls
The rows are the known answers. Controls of this bead itself: the swapped-number registry; a row whose printed
truth was edited without its derivation file (must fail `check-generated`); a row cited by a gate while its
tolerance is `to set` (must be reported uncounted).

## Considerations

- Values are carried as stated and are not improved in transit; a row whose printed value disagrees with its
  formula keeps the printed value and is listed as unreconciled until the derivation file settles it.
- The sibling-defect rows (KA-003 to KA-008) are FACT to recheck: they were reported by review and not yet
  reproduced by this program at a pinned commit.
- KA-019 to KA-026 share fixtures with later rows of the mission and sensing chapters; the derivation files are
  shared and each rule keeps its own row.

## References

Appendix M.1 to M.7, M.37, M.38; GATE-GAUNT-02; OPEN-GAUNT-2, OPEN-GAUNT-3, OPEN-GAUNT-6; INV-GAUNT-07;
ADR-GAUNT-02.
