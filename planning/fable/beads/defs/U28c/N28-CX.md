---
key: "N28-CX"
title: "Atelier counterexample registry: CX-302 to CX-318 as negative-control seeds, plus two late entries"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "atelier", "frob-conformance", "solid", "registry", "test"]
parent: "WP-11"
deps: ["BD-025", "STD-TEST"]
needs: ["§38.11: the negative-control registry `frob-conformance/registry/controls.toml` and the counterexample catalogue seeds loaded at M1 (BD-146)", "Appendix N: catalogue numbers for the two late counterexamples of the second table (the unit that owns the catalogue)"]
related: ["N28-REF", "N28c-24", "N28c-21", "BD-146", "BD-318", "BD-377"]
gates: []
plan_refs: ["Appendix N.25", "§28.4", "§28.6", "§28.7", "§28.10", "§28.13", "§28.16", "§28.25", "§28.26", "§38.11"]
---
## Background

A counterexample, in this program, is a hand-checkable case in which a plausible shortcut gives the wrong answer.
The catalogue of counterexamples seeds the negative-control registry: each entry becomes at least one planted
defect that a named gate must catch, on the rule that a gate nothing can fail validates nothing. A control is a
defect, not a boundary: the control is the build or input in which the protective rule was removed, and the gate
must fail on it. A control that has never failed its gate in any recorded run is itself suspect and is reviewed at
each milestone exit.

This bead is the registry of the design studio's counterexamples: seventeen catalogue entries, CX-302 to CX-318,
each with the tempting shortcut, the smallest case that breaks it with its numbers, the rule it protects, the gate
that must catch the corresponding control, and the mechanism bead that plants and catches it. Several are
equivalent to entries first catalogued by other subsystems (noted with ≡); the control is registered once and
attached to every gate that must catch it. Two further counterexamples from late subsections have no catalogue
number yet and are carried in a second table so they are not lost.

Design reason for keeping the numbers in the registry: the cases are small on purpose, so that a reviewer can
recompute them by hand and so that a test that "passes" by accident can be told from one that checks the right
thing. Tier: Solid [S].

## Scope

In: the registry rows for the seventeen entries and the two late ones; a control record for each in the format of
the negative-control registry; the check that every entry has a gate and an owner.
Out: implementing the protected rules (the mechanism beads named per row); the known-answer registry of the
chapter; running the controls (each gate's own bead, and the chapter battery N28c-24).

## Technical approach

**Table 1: catalogue entries.**

| CX | The tempting shortcut | The smallest case that breaks it | The rule it protects | Gate(s) that must catch the control | Planted as |
|---|---|---|---|---|---|
| CX-302 | Support polygon plus total actuator capacity is a stance screen (analysis A-08) | In an exhaustive stance enumeration of 11,520 body and load cases the screen accepts 1,596 bodies that the exact classifier rejects. Under independent foot forces a stance needs 155 N at the worst foot; under equal passive springs the forces are (175, 115, 85, 25) N against a 160 N cap | Per-contact force limits enter the cone program; the result is typed (`exact`, `inner`, `outer`) and names its load-sharing model | GATE-ATEL-02 | A-08 replaced by the polygon-and-total-capacity screen |
| CX-303 | Add peak currents on a shared bus (A-23) | Two fingers each need one powered tick and the bus powers one finger per tick: no joint schedule at a one-tick deadline, exactly two at two ticks. A rule that adds both peaks rejects the feasible two-tick design | The joint time-indexed contract; a prune is rejected if any discarded design is feasible under the same mission | GATE-ATEL-05, GATE-ATEL-09 (prune soundness) | A-23 replaced by the additive-peak rule |
| CX-304 | Connection curvature tests for a misaligned mecanum roller (A-09) | A base with misaligned rollers still has spanning curvature, as a differential drive does; the defect is a rank drop in the connection, reported by the growth vector of A-07 | A-07 and A-09 are separate analyses | GATE-ATEL-02 | A check that uses curvature only |
| CX-305 | The mechanical symmetry group is the policy's symmetry (A-03) | A quadruped with one forward lidar has a four-element group mechanically and a two-element group in closed loop | Only the observation-preserving subgroup is used for tying, augmentation and deduplication | GATE-ATEL-02 | A policy tied under the mechanical group (must be refused) |
| CX-306 (≡ CX-076) | A quasi-static energy exclusion needs no approach energy (A-20) | 8 kg on a 12 mm rise needs 8 × 9.81 × 0.012 = 0.94176 J; a 0.90 J budget excludes the passive crossing; 0.05 J of approach kinetic energy removes the exclusion | The budget includes kinetic energy at the start of the segment; the result is `outer` only for the enclosed model (`FROB-REF-551`) | GATE-ATEL-05 | An exclusion issued with approach energy unbound |
| CX-307 (≡ CX-052) | Per-reading accuracy is the value of sensor redundancy (A-17) | Seven independent readings each right with probability 0.75 give a majority success of 3807/4096 ≈ 0.929; seven with a perfectly shared error stay at 0.75 | The alias analysis reads the declared error dependence of the cards | GATE-ATEL-05 | A redundancy score from per-reading accuracy |
| CX-308 (≡ CX-262) | An instantaneous-current model of the drive loop is adequate (A-22) | 75 mm wheel, total reflected inertia 0.00107890625 kg·m², current target held for 10 ms: one-tick spectral radius 1.4820 with target-current feedforward and 0.9207 with measured-current feedforward; the instantaneous model gives 2.6917 for both (2.7075 without the fixture's shaft damping) and misses the stable case | The hold is modelled exactly over the control period | GATE-ATEL-05 | A-22 with the instantaneous-current model |
| CX-309 | Identify friction from a convenient run (A-25) | Drawbar force F = min(k_t·I/r, μ·N) with k_t = 0.2 N·m/A, r = 0.06 m: runs at 1 A and 2 A under 30 N and 40 N give 3.33 N and 6.67 N for μ = 0.4 and for μ = 0.8; at 3 A under 10 N they give 4 N and 8 N | Zero sensitivity is reported and an excitation in the traction-limited regime is requested | GATE-ATEL-05 | A fit that reports μ from the first two runs |
| CX-310 | A late observation with the right content serves the decision (A-21) | A packet arriving at tick 1 serves decisions committing at ticks 2 and 4; delayed to tick 3 it serves the second and no longer the first | Arrival instants are compared with decision instants per decision, in rational time | GATE-ATEL-05 | A check that compares content and ignores arrival |
| CX-311 | Key paired seeds by (world, epoch) | On a 64-wide batch such seeds pair only morphologies whose indices agree modulo 64 | Pair by (world, replicate); a counter that depends on training progress is not part of an address (`FROB-REF-556`) | GATE-ATEL-06 | A study planned with epoch-keyed seeds |
| CX-312 | Pair exogenous draws by event order | Two tires on different paths need not receive equal friction samples at corresponding ticks; pairing "the k-th friction draw" pairs nothing (the chapter gives no numeric case) | Exogenous fields are indexed by physical coordinate (`FROB-REF-556`) | GATE-ATEL-06 | A material field sampled by draw index |
| CX-313 | Binary search finds the first edit that changed the outcome | Outcomes along six versions are pass, fail, pass, pass, pass, fail: the midpoint passes, the search goes right and reports the last edit, while the first edit changed the outcome and the second changed it back | `design.bisect` returns an `a-change`; `first-change` only after the prefix is scanned or monotonicity is declared (`FROB-REF-550`) | GATE-ATEL-04 | A bisect that labels the last edit `first-change` |
| CX-314 (≡ CX-282) | Enlarging the hypothesis support slightly leaves inclusions valid | Docking with gains {1, 2}: adding gain 33/32, indistinguishable from gain 1 at an error radius of 1/64, makes every one of the 34,590 positive decisions on each test grid wrong | Revalidation polarity: "hypothesis support enlarged: recheck inclusions" | GATE-ATEL-11 | A hash match taken as verdict transfer |
| CX-315 | A memo key may omit delivery time | Four-state camera fixture (decisions commit at ticks 2 and 4, packet at tick 1; states: initial, a display label renamed, delivery delayed to tick 2, delivery delayed to tick 3): with the complete key the evaluations are 2, 0, 0, 1 and the recomputations 1, 0, 0, 1; the key without delivery time keeps a stale promotion at the fourth state | Complete memo keys: delivery time is a dependency of a decision | GATE-ATEL-11, GATE-ATEL-12 | The stale-key mutant |
| CX-316 | An optimum survives any change that keeps it feasible | Widening the doorway keeps the cost-9 cart feasible and admits a cost-5 relay that the doorway had excluded: feasibility is kept, optimality is revoked | Revalidation treats feasibility and optimality as separate obligations | GATE-ATEL-11 | An optimum kept after a cheaper rival became feasible |
| CX-317 | An in-class-perfect probe is safe to act on | A probe synthesized for three admitted material laws is right in 675 of 675 in-class cases; a fourth, delayed law follows the same early trace and reverses all 225 held-out decisions; direct measurement at mission dwell is right in 900 of 900 | Measuring the target directly is always on the list; every suggestion states its model class | GATE-ATEL-15 (through the suggestion script), GATE-AUGUR-11 | A suggestion list without the direct measurement |
| CX-318 | Rank agreement near the top can be assumed | Published neighbours report rank correlations of 0.40 overall and 0.50, 0.60, 0.83 on three robots over 17 designs, and sparse Kendall τ of 0.236, 0.740 and 0.159 on three benchmarks: agreement of 0.7 or better is not supported | The rank-agreement ledger; an unmeasured stratum is `uninformative` (`FROB-REF-537`) | GATE-ATEL-07, GATE-ATEL-09 | A rung allowed to select in an unmeasured stratum |

**Table 2: late counterexamples awaiting catalogue numbers** (temporary identifiers; the catalogue owner assigns
`CX` numbers).

| Temporary id | The tempting shortcut | The smallest case that breaks it | The rule it protects | Gate | Planted as |
|---|---|---|---|---|---|
| CX-28L-1 | A saturating law extrapolates any controller-budget curve | A body that needs a threshold budget to find any gait scores 0.20, 0.20, 0.80 at three budgets. The two lower budgets extrapolate to a ceiling of 0.20, wrong by at least 0.60, and the three-point form divides by zero | The fit is refused (`curve-unfit`); such a design is escalated and never discarded on its curve | GATE-ATEL-17 | A fit that returns 0.20 for the threshold case |
| CX-28L-2 | Report "reuse" as one number, or from a cold run | A kernel compile of 95.91 s beside 240 s of training is 28.6% of a cold run, and the cached 0.89 s is 0.37% of a warm one, so a cold figure flatters a cache; a warm start that avoids 0.8 of training gives 72% in the reacquisition column while exact reuse after a dynamics-affecting edit is at most about a tenth | Three columns, never summed, cold and warm apart | GATE-ATEL-19 | A report that sums the columns, or prints one cold figure |

**Control records (format of the negative-control registry).** For each row one record, for example:

```toml
[[control]]
id        = "NC-28-313"                     # assigned by the registry; stable, never reused
seed      = "CX-313"
gate      = ["GATE-ATEL-04"]
kind      = "mutation"                      # mutation | planted-input | planted-design | planted-world | a-a
target    = "frob-opt::lineage::bisect"
defect    = "labels the bracket found by plain binary search as first-change without a prefix scan"
expect    = "on outcomes (pass, fail, pass, pass, pass, fail) the mutant reports first-change at the last edit; the gate's bisect row fails"
```

Kinds: CX-302, -303, -304, -306 to -310, -313 to -316 and both late rows are `mutation`; CX-305, -311, -312, -317
and -318 are `planted-input`.

**Steps.** 1. Enter the nineteen rows in the catalogue file of the conformance crate with the numbers above. 2.
Write a control record per row. 3. Attach each control to its gates. 4. Add the registry test below. 5. Publish
the table into the crate's reference documentation by the generated-artifact check.

## Acceptance criteria

- [ ] All seventeen catalogue entries and both late entries exist in the catalogue with shortcut, case, numbers
      and protected rule exactly as tabulated.
- [ ] Each entry has at least one control record with a gate, a target and an `expect`; equivalent entries
      (CX-306 ≡ CX-076, CX-307 ≡ CX-052, CX-308 ≡ CX-262, CX-314 ≡ CX-282) share one control attached to both
      subsystems' gates.
- [ ] The registry test fails if an entry has no control, if a control names a gate that does not exist, or if a
      number in the table differs from the fixture the control uses.
- [ ] The arithmetic of the hand-checkable cases is verified by tests: 0.94176 J; 3807/4096; 3.33 N, 6.67 N, 4 N,
      8 N; 28.6% and 0.37%; 72%.
- [ ] Each control's `last_seen.caught` is recorded true by its gate's bead before that gate can record PASS.

## Test plan

### Unit tests

`crates/frob-conformance/tests/cx_atelier.rs`: `seventeen_plus_two_entries`; `every_entry_has_control_and_gate`;
`equivalents_share_control`; `energy_case_arithmetic` (8 × 9.81 × 0.012 = 0.94176; 0.90 + 0.05 ≥ 0.94176);
`majority_of_seven` (sum over k ≥ 4 of C(7, k)·0.75ᵏ·0.25⁷⁻ᵏ = 3807/4096 exactly);
`drawbar_values` (min(0.2·1/0.06, 0.4·30) = 3.33; at 3 A under 10 N: min(10, 4) = 4 and min(10, 8) = 8);
`bisect_sequence_midpoint_passes`; `cold_and_warm_shares` (95.91/335.91 = 0.2855; 0.89/240.89 = 0.00369);
`threshold_curve_divides_by_zero` (Δ₁ = 0).

### End-to-end script and logging

No script of its own. Each control is exercised by the end-to-end script of its gate; the chapter battery
(N28c-24) lists, per control, the script and the `assert` line that shows it was caught, and the conformance
crate's registry report (`frob-conformance report controls --chapter 28 --json`) prints `caught` per control with
the session and date.

### Known answers and negative controls

This bead is the registry of negative-control seeds. Its own negative control: a catalogue entry with a number
changed by one digit (0.94176 → 0.94167) must fail the arithmetic test.

## Considerations

- CX-312 has no numeric case in the plan; its control is a planted input (a material field sampled by draw index
  on two paths of different length), and the mechanism bead for pairing must supply a small numeric fixture when
  it implements the rule.
- CX-318 is a statement about published evidence, re-read before any gate cites it; its control tests the
  fail-closed default, not the literature.
- The exhaustive-enumeration facts (11,520; 1,596; 34,590) come from executed planning pilots and are re-run in
  the repository before a gate cites them; until then the fixtures carry the label `to-be-rerun`.

## References

Appendix N (N.25), §28.4, §28.6, §28.7, §28.10, §28.13, §28.16, §28.25, §28.26, §38.11; GATE-GAUNT-01.
