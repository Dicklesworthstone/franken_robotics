---
key: "N28-CX"
title: "Atelier counterexample registry: CX-302 to CX-318 as negative-control seeds, plus two late entries"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "atelier", "frob-conformance", "solid", "registry", "test"]
parent: "WP-11"
deps: ["BD-025", "N38a-40", "N38b-05", "STD-E2E", "STD-TEST"]
needs: []
related: ["BD-240", "BD-243", "N10a-08", "N23a-31", "N28-REF", "N28a-06", "N28a-11", "N28a-13", "N28a-18", "N28b-02", "N28b-03", "N28b-14", "N28b-22", "N28b-26", "N28b-30", "N28b-31", "N28b-32", "N28b-34", "N28b-35", "N28b-37", "N28c-13", "N28c-21", "N28c-25", "N28c-35", "N28c-39", "N28c-42", "N28c-45", "N38b-04"]
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
that must catch the corresponding control, and the mechanism bead that plants and catches it. Four are stated
first by other subsystems with the same fixture (marked ≡). Each keeps its own entry and its own control, attached
to its own rule's gates, because the registry holds one control per protected rule (catalogue rule 1); the two
controls may share one fixture. Two further counterexamples from late subsections (§28.25 and §28.26) have no
catalogue number yet; they are entered under provisional tags and numbered from a block of the allocation ledger.

Design reason for keeping the numbers in the registry: the cases are small on purpose, so that a reviewer can
recompute them by hand and so that a test that "passes" by accident can be told from one that checks the right
thing. Tier: Solid [S].

## Scope

In: the nineteen entries of the catalogue `crates/frob-conformance/registry/counterexamples.toml` (seventeen
numbered, two late), each with the substance a control needs (kind, plant, target, defect, expected observation,
the gates among `rule_ids`); the block request for the two late numbers; a host test that recomputes every number;
the `controls` report of the example target `registry_report`; the registry script.
Out, with the bead that has it: the catalogue's record schema and loader, and the seeding tool that writes
`crates/frob-conformance/registry/controls.toml`, allocates `NC-` numbers, keeps the seed map and proves
completeness by the join (N38b-05). This bead writes no row of `controls.toml`: its controls appear there when the
seeding is re-run over its entries, which is append-only and idempotent. The control runner (N38b-04). The number
blocks (N38a-40). The protected mechanisms and the planted variants, built by the bead named in each row; the
chapter's known answers (N28-KA); running the controls in the gates' scripts and the chapter battery (N28c-24).

## Technical approach

**Data.** Each row is one entry in the catalogue's record schema (N38b-05): `id` (the CX number, or the tag for a
late entry until its block is issued), `shortcut`, `case`, `numeric` (true, except CX-312), `plan_case`
(`"numbers"`, except CX-312: `"words"`), `case_supplied_by` (empty), `rule`, `rule_ids` (the gates of the gate
column with the refusal codes and analysis identifiers of the rule column), `defined_in`, `same_as` (`"CX-076"` for
CX-306, `"CX-052"` for CX-307, `"CX-262"` for CX-308, `"CX-282"` for CX-314), `label` (DESIGN for arithmetic;
FACT with source for CX-302, CX-314 and CX-318, whose figures come from executed planning pilots or published
work and carry `rerun = false` until re-run here), `control_kind`, `plant` and, for a late entry, `slug`. The
control column gives the control's kind; plant; target; defect and expected observation: its first word is
`control_kind`, the identifier in backquotes is the name of the entry's `plant` (for a `mutation`, the planted
variant built by the named bead behind a test-only Cargo feature reachable only under `gauntlet-mutations`, BD-025;
for the other kinds, the fixture). The plant's `id` is `<planting bead>:<identifier>`, as N38b-05 requires, the
planting bead being the first bead of the "Planted and caught by" column (`N28a-11:a08-polygon-screen` for CX-302,
`BD-243:a09-curvature-only` for CX-304, `N28c-35:curve-fit-threshold` for CX-28L-1); the name after the colon
stays the Cargo feature or the fixture. The target, defect and expected observation are the texts the seeding
tool copies into the control's `target`, `defect` and `expect` (BD-025 requires `defect` and `expect` non-empty). Every gate among an
entry's `rule_ids` must fail with the plant enabled.

**Table 1: catalogue entries.**

| CX | The tempting shortcut | The smallest case that breaks it | The rule it protects | Gate(s) | Control (kind; plant; target; defect; expected observation) | Planted and caught by |
|---|---|---|---|---|---|---|
| CX-302 | Support polygon plus total actuator capacity is a stance screen (analysis A-08) | In an exhaustive stance enumeration of 11,520 body and load cases the screen accepts 1,596 bodies that the exact classifier rejects. Under independent foot forces a stance needs 155 N at the worst foot in the reported witness allocation (155, 135, 105, 5) N (A-08's midpoint rule, BD-100; the least worst-foot load is 150 N, at (150, 140, 110, 0) N); under equal passive springs the forces are (175, 115, 85, 25) N against a 160 N cap | Per-contact force limits enter the cone program; the result is typed (`exact`, `inner`, `outer`) and names its load-sharing model | GATE-ATEL-02 | mutation; `a08-polygon-screen`; the A-08 stance classifier of `frob-analyze`; A-08 replaced by the polygon-and-total-capacity screen; the stance battery reports 1,596 bodies accepted that the exact classifier rejects | N28a-11 (row 1) on BD-100, with the battery of N28a-06; M1 |
| CX-303 | Add peak currents on a shared bus (A-23) | Two fingers each need one powered tick and the bus powers one finger per tick: no joint schedule at a one-tick deadline, exactly two at two ticks. A rule that adds both peaks rejects the feasible two-tick design | The joint time-indexed contract; a prune is rejected if any discarded design is feasible under the same mission | GATE-ATEL-05, GATE-ATEL-09 (prune soundness) | mutation; `a23-additive-peak`; the A-23 shared-bus schedule; A-23 replaced by the additive-peak rule; the feasible two-tick design is rejected and, in a campaign, pruned | N28a-11 (row 2) on N28a-10, M1; the prune half by N28b-30, M4 |
| CX-304 | Connection curvature tests for a misaligned mecanum roller (A-09) | A base with misaligned rollers still has spanning curvature, as a differential drive does; the defect is a rank drop in the connection, reported by the growth vector of A-07 | A-07 and A-09 are separate analyses | GATE-ATEL-02 | mutation; `a09-curvature-only`; the A-09 roller-alignment check; a check that uses curvature only; the misaligned base passes, while A-07 reports the growth vector (2, 3) where (3) is required | BD-243 builds the curvature mutant at M3, binding row 3 of N28a-11's table; the growth-vector half by N28a-11 (row 3) on BD-106 at M1 |
| CX-305 | The mechanical symmetry group is the policy's symmetry (A-03) | A quadruped with one forward lidar has a four-element group mechanically and a two-element group in closed loop | Only the observation-preserving subgroup is used for tying, augmentation and deduplication (`FROB-REF-161`) | GATE-ATEL-02 | planted-input; `a03-tie-under-mechanical-group`; the observation-preserving group of `analyze.symmetry` and the tie consumers that read it; a policy tie requested under the four-element mechanical group, with the observation-preserving check removed; the order-4 group is offered for tying, where the tie must be refused with FROB-REF-161 naming the lidar mount as the breaker (the usable group has order 2): by the symmetry interface's tie consumer at M1 (N10a-08, `d14_front_lidar`) and by `train.start --tying equivariant` from M4 (N23a-31) | N28a-11 (row 4) on BD-102, with N10a-08's tie consumer; M1; N23a-31 at M4 |
| CX-306 (≡ CX-076) | A quasi-static energy exclusion needs no approach energy (A-20) | 8 kg on a 12 mm rise needs 8 × 9.81 × 0.012 = 0.94176 J; a 0.90 J budget excludes the passive crossing; 0.05 J of approach kinetic energy removes the exclusion | The budget includes kinetic energy at the start of the segment; the result is `outer` only for the enclosed model (`FROB-REF-551`) | GATE-ATEL-05 | mutation; `a20-no-approach-energy`; the A-20 energy-budget analysis; an exclusion issued with approach energy unbound; the crossing is excluded although 0.90 + 0.05 ≥ 0.94176, and FROB-REF-551 does not fire | N28a-11 (row 5) on N28a-07; M1 |
| CX-307 (≡ CX-052) | Per-reading accuracy is the value of sensor redundancy (A-17) | Seven independent readings each right with probability 0.75 give a majority success of 3807/4096 ≈ 0.929; seven with a perfectly shared error stay at 0.75 | The alias analysis reads the declared error dependence of the cards | GATE-ATEL-05 | mutation; `a17-per-reading-accuracy`; the A-17 redundancy score; a redundancy score from per-reading accuracy; seven shared-error cards are scored 0.929 where 0.75 is true | BD-240 builds the mutant against its alias search at M3, binding row 6 of N28a-11's table; the arithmetic by N28a-11 (row 6) at M1 |
| CX-308 (≡ CX-262) | An instantaneous-current model of the drive loop is adequate (A-22) | 75 mm wheel, total reflected inertia 0.00107890625 kg·m², current target held for 10 ms: one-tick spectral radius 1.4820 with target-current feedforward and 0.9207 with measured-current feedforward; the instantaneous model gives 2.6917 for both (2.7075 without the fixture's shaft damping) and misses the stable case | The hold is modelled exactly over the control period | GATE-ATEL-05 | mutation; `a22-instantaneous-current`; the A-22 sampled-loop analysis; A-22 with the instantaneous-current model; 2.6917 for both feedforward laws, the 0.9207 stable case reported unstable | N28a-11 (row 7) on N28a-09; M1 |
| CX-309 | Identify friction from a convenient run (A-25) | Drawbar force F = min(k_t·I/r, μ·N) with k_t = 0.2 N·m/A, r = 0.06 m: runs at 1 A and 2 A under 30 N and 40 N give 3.33 N and 6.67 N for μ = 0.4 and for μ = 0.8; at 3 A under 10 N they give 4 N and 8 N | Zero sensitivity is reported and an excitation in the traction-limited regime is requested | GATE-ATEL-05 | mutation; `a25-convenient-run`; the A-25 identifiability analysis; a fit that reports μ from the first two runs; μ is reported with no zero-sensitivity finding and no excitation request | N28a-13 builds the mutant at M4, binding row 8 of N28a-11's table |
| CX-310 | A late observation with the right content serves the decision (A-21) | A packet arriving at tick 1 serves decisions committing at ticks 2 and 4; delayed to tick 3 it serves the second and no longer the first | Arrival instants are compared with decision instants per decision, in rational time | GATE-ATEL-05 | mutation; `a21-content-only`; the A-21 deadline closure; a check that compares content and ignores arrival; the packet delayed to tick 3 is reported serving both decisions | N28a-11 (row 9) on N28a-08; M1 |
| CX-311 | Key paired seeds by (world, epoch) | On a 64-wide batch such seeds pair only morphologies whose indices agree modulo 64 | Pair by (world, replicate); a counter that depends on training progress is not part of an address (`FROB-REF-556`) | GATE-ATEL-06 | planted-input; `pair-world-epoch`; the study planner; a study planned with epoch-keyed seeds; refused with FROB-REF-556 (with the check removed, the plan is accepted and pairs only indices equal modulo 64) | N28a-18; M1 (and the refusal battery N28c-21 at M4) |
| CX-312 | Pair exogenous draws by event order | Two tires on different paths need not receive equal friction samples at corresponding ticks; pairing "the k-th friction draw" pairs nothing (the chapter gives no numeric case) | Exogenous fields are indexed by physical coordinate (`FROB-REF-556`) | GATE-ATEL-06 | planted-input; `pair-by-draw-index`; the exogenous-field sampler; a material field sampled by draw index on two paths of different length; refused with FROB-REF-556. Obligation (`numeric = false`) until N28a-18 supplies the small numeric fixture and fills `case_supplied_by` | N28a-18; M1 |
| CX-313 | Binary search finds the first edit that changed the outcome | Outcomes along six versions are pass, fail, pass, pass, pass, fail: the midpoint passes, the search goes right and reports the last edit, while the first edit changed the outcome and the second changed it back | `design.bisect` returns an `a-change`; `first-change` only after the prefix is scanned or monotonicity is declared (`FROB-REF-550`) | GATE-ATEL-04 | mutation; `bisect-last-edit-first-change`; `frob-opt::lineage::bisect`; labels the bracket found by plain binary search as `first-change` without a prefix scan; on (pass, fail, pass, pass, pass, fail) the mutant reports `first-change` at the last edit and the gate's bisect row fails | N28b-02, with the planted-lineage battery N28b-03; M4 |
| CX-314 (≡ CX-282) | Enlarging the hypothesis support slightly leaves inclusions valid | Docking with gains {1, 2}: adding gain 33/32, indistinguishable from gain 1 at an error radius of 1/64, makes every one of the 34,590 positive decisions on each test grid wrong | Revalidation polarity: "hypothesis support enlarged: recheck inclusions" | GATE-ATEL-11 | mutation; `support-enlarged-hash-match`; the revalidation polarity table; a hash match taken as verdict transfer; the 34,590 positive decisions are kept after gain 33/32 is added | N28b-34 with BD-347; M4 |
| CX-315 | A memo key may omit delivery time | Four-state camera fixture (decisions commit at ticks 2 and 4, packet at tick 1; states: initial, a display label renamed, delivery delayed to tick 2, delivery delayed to tick 3): with the complete key the evaluations are 2, 0, 0, 1 and the recomputations 1, 0, 0, 1; the key without delivery time keeps a stale promotion at the fourth state | Complete memo keys: delivery time is a dependency of a decision | GATE-ATEL-11, GATE-ATEL-12 | mutation; `memo-key-no-delivery` (the stale-key mutant); the decision memo key; delivery time dropped from the key; at the fourth state the promotion is kept and the recomputation count reads 0 where 1 is true | N28b-37 (the `Memo` trait) with N28b-32; M1 |
| CX-316 | An optimum survives any change that keeps it feasible | Widening the doorway keeps the cost-9 cart feasible and admits a cost-5 relay that the doorway had excluded: feasibility is kept, optimality is revoked | Revalidation treats feasibility and optimality as separate obligations | GATE-ATEL-11 | mutation; `optimum-kept-on-feasibility`; `result.revalidate`; one obligation for feasibility and optimality; the cost-9 cart is still reported optimal after the cost-5 relay became feasible | N28b-32, M1; grades 3 and 4 by BD-347, M4 |
| CX-317 | An in-class-perfect probe is safe to act on | A probe synthesized for three admitted material laws is right in 675 of 675 in-class cases; a fourth, delayed law follows the same early trace and reverses all 225 held-out decisions; direct measurement at mission dwell is right in 900 of 900 | Measuring the target directly is always on the list; every suggestion states its model class | GATE-ATEL-15 (through the suggestion script), GATE-AUGUR-11 | planted-input; `suggest-without-direct`; `experiment.suggest`; a suggestion list without the direct measurement; the list must be refused, and with the rule removed the probe is offered alone and reverses the 225 held-out decisions | N28c-13 with BD-351; M4 |
| CX-318 | Rank agreement near the top can be assumed | Published neighbours report rank correlations of 0.40 overall and 0.50, 0.60, 0.83 on three robots over 17 designs, and sparse Kendall τ of 0.236, 0.740 and 0.159 on three benchmarks: agreement of 0.7 or better is not supported | The rank-agreement ledger; an unmeasured stratum is `uninformative` (`FROB-REF-537`) | GATE-ATEL-07, GATE-ATEL-09 | planted-input; `rung-selects-unmeasured`; the rank-agreement ledger; a rung allowed to select in an unmeasured stratum; refused with FROB-REF-537 and the stratum labelled `uninformative` | N28b-22, M4; the M3 pilot BD-310 |

**Table 2: late counterexamples awaiting catalogue numbers.** They are appended to the catalogue, which is
append-only. A bead never invents a number above the plan-time maximum (CX-384): the numbers come from the
allocation ledger `crates/frob-conformance/registry/allocations.toml` (schema `frob.gauntlet.alloc/1`, N38a-40).
This bead files one `[[block]]` request with `registry = "cx"`, `count = 2`, `owner = "N28-CX"`,
`section = "§28.25, §28.26"`; the allocator fills `first` and `last`, and the two tags are mapped to the block's
numbers in table order in one commit. Until then the tag is the entry's name; afterwards it stays in `slug`. A tag
still used as an identifier in a registry, a test name or a derivation file at this bead's milestone (M1) fails the
ledger check `provisional-tag-past-due`.

| Tag | The tempting shortcut | The smallest case that breaks it | The rule it protects | Gate | Control (kind; plant; target; defect; expected observation) | Planted and caught by |
|---|---|---|---|---|---|---|
| CX-28L-1 | A saturating law extrapolates any controller-budget curve | A body that needs a threshold budget to find any gait scores 0.20, 0.20, 0.80 at three budgets. The two lower budgets extrapolate to a ceiling of 0.20, wrong by at least 0.60, and the three-point form divides by zero (Δ₁ = 0) | The fit is refused (`curve-unfit`); such a design is escalated and never discarded on its curve | GATE-ATEL-17 | mutation; `curve-fit-threshold`; the controller-budget curve fit of `frob-opt`; a fit without the `curve-unfit` refusal; returns J∞ = 0.20 for the threshold case and the design is discarded | N28c-35, with the gate script N28c-39; M3 |
| CX-28L-2 | Report "reuse" as one number, or from a cold run | A kernel compile of 95.91 s beside 240 s of training is 28.6% of a cold run, and the cached 0.89 s is 0.37% of a warm one, so a cold figure flatters a cache; a warm start that avoids 0.8 of training gives 0.9 × 0.8 = 72% in the reacquisition column while exact reuse after a dynamics-affecting edit is at most about a tenth | Three columns, never summed, cold and warm apart | GATE-ATEL-19 | mutation; `reuse-summed-or-cold`; the reuse-ledger report; a report that sums the columns, or prints one cold figure; one summed or cold-only figure is printed where three named columns, cold and warm, are required, and the report is not refused | N28c-42, with the gate script N28c-45; M3 |

**What the seeding tool writes from an entry** (the record of BD-025, for orientation): `id` (`NC-` plus four
digits, assigned by the tool), `seed` (the CX number; for a late entry its issued number), `gate` (the gates of
the row), `kind`, `target`, `defect`, `expect`, `last_seen` (written by the control runner, never by hand), and
`feature` for a `mutation` or `fixture` for the other kinds. A control whose planted variant is not yet built is
reported `stale` by the runner and keeps its gate from passing; that is the intended pressure, not a defect of
this registry.

**The controls report.** ADR-GAUNT-01 (BD-025) gives `frob-conformance` no binary, verb or subcommand; what it
allows beside the test targets are example targets (`record_gate`, `validate_register`). This bead adds
`crates/frob-conformance/examples/registry_report.rs` with the subcommand `controls --chapter <n> --json`
(DESIGN): it loads `registry/counterexamples.toml` and `registry/controls.toml`, joins them with
`registries/gates.toml`, and prints per entry its controls, their gates and each control's state (`pending` with
the planting bead's key, `stale`, or `caught` with session and date). The chapter's other registry reports
(`gates`, `ka`, `obligations`) are subcommands of the same example, added by their beads; whichever lands first
creates the file.

**Steps.** 1. Enter the seventeen numbered entries and the two tagged ones. 2. File the block request and, once
issued, map the tags in one commit. 3. Re-run the seeding tool (`cargo test -p frob-conformance --test
seed_controls -- --ignored seed`) and the join (`cargo test -p frob-conformance --test seed_join`). 4. Add the
registry test and the report. 5. Publish the table into the crate's reference documentation by the
generated-artifact check.

## Acceptance criteria

- [ ] `counterexamples.toml` holds CX-302 to CX-318 and the two late entries with shortcut, case, numbers and
      protected rule exactly as tabulated, each with `control_kind`, a `plant` (CX-312 may leave it empty while it
      is an obligation) and its gates among `rule_ids`; CX-306, CX-307, CX-308 and CX-314 carry the `same_as` above.
- [ ] After the seeding tool of N38b-05 is re-run, each of the nineteen entries has exactly one control in
      `controls.toml` whose `seed` names it, attached to the gates of its row; the four ≡ entries have their own
      control beside the earlier entry's, not a shared one; the tool's second run changes no byte; the join passes;
      no entry ends in the seeding tool's findings file. No record of `controls.toml` is written by this bead.
- [ ] The two late entries carry numbers from a block issued to N28-CX in `allocations.toml`, in table order, with
      their tags in `slug`; `registry::alloc::validate` passes and no tag remains as an identifier at M1.
- [ ] The arithmetic of the hand-checkable cases is verified by tests: 0.94176 J; 3807/4096; 3.33 N, 6.67 N, 4 N,
      8 N; 150 N and (175, 115, 85, 25) N from the witness allocation; 28.6% and 0.37%; 72%.
- [ ] CX-302, CX-314 and CX-318 carry `label = FACT` with `rerun = false` and are seeded as obligations until
      their re-run receipt sets `rerun = true`.
- [ ] `scripts/e2e/registry_n28_cx.sh` passes and rejects each of its five planted catalogue defects.

## Test plan

### Unit tests

`crates/frob-conformance/tests/cx_atelier.rs` (run as `cargo test -p frob-conformance --test cx_atelier`; real
registry files and scratch copies, no mocks): `seventeen_plus_two_entries` (by tag before the block is issued, by
number and `slug` after); `every_entry_has_kind_plant_and_gate`; `equivalents_have_own_controls` (after a seeding
run on a scratch copy, CX-306 and CX-076 have two distinct controls, each with its own gates; likewise the other
three pairs); `block_request_of_two` (the ledger holds the request; after issue the numbers are contiguous and in
table order); `energy_case_arithmetic` (8 × 9.81 × 0.012 = 0.94176; 0.90 + 0.05 ≥ 0.94176); `majority_of_seven`
(Σ over k ≥ 4 of C(7, k)·0.75ᵏ·0.25⁷⁻ᵏ = 3807/4096 exactly, in rationals); `stance_allocations` (the null direction
(1, −1, −1, 1) of the three balance equations: from (155, 135, 105, 5) the least maximum with all forces ≥ 0 is 150 N
at t = −5, and the equal-spring compatibility f₊₊ + f₋₋ = f₊₋ + f₋₊ gives t = 20 and (175, 115, 85, 25) N);
`drawbar_values` (min(0.2·1/0.06, 0.4·30) = 3.33; at 3 A under 10 N: min(10, 4) = 4 and min(10, 8) = 8);
`bisect_sequence_midpoint_passes`; `cold_and_warm_shares` (95.91/335.91 = 0.2855; 0.89/240.89 = 0.00369;
0.9 × 0.8 = 0.72); `threshold_curve_divides_by_zero` (Δ₁ = 0); `fact_entries_obligation_until_rerun`.

### End-to-end script and logging

`scripts/e2e/registry_n28_cx.sh` (idempotent; no seed needed; header `bead=N28-CX milestone=M1
labels=atelier,registry plants=no-plant,unknown-gate,digit-changed,shared-equivalent,stale-tag`): the registry
seen from the user's side, needing no Atelier mechanism, so it passes at this bead's close (it calls no `frob`
verb: every verb belongs to a bead this registry does not depend on). It runs
`cargo test -p frob-conformance --test cx_atelier` (the registries load through the one parser); the seeding tool and
the join on a scratch copy of `registry/` under `target/e2e-scratch/registry_n28_cx/` (never the committed files),
asserting by digest that a second seeding run is a no-op; `cargo run -p frob-conformance --example registry_report
-- controls --chapter 28 --json`, asserting nineteen entries, one control per entry, every named gate present in
`registries/gates.toml`, and each control's state (`pending` or `stale` with the planting bead's key while that bead
is open; `caught` with session and date once its gate's script has run). It then plants five defects one at a time
on scratch copies and asserts each is rejected by the named check: an entry without a `plant`; an entry naming
`GATE-ATEL-99`; 0.94176 changed to 0.94167; the CX-306 entry with `same_as` removed and its control merged into
CX-076's; a tag left as an identifier at M1. A control whose planting bead is closed and that is still `stale`
fails the script. It sources the STD-E2E helper and writes `target/e2e-logs/registry_n28_cx/<utc-timestamp>.ndjson`
with a transcript; every line has `ts`, `script`, `bead` (`N28-CX`), `step`, `event` (`start`, `cmd`, `assert`,
`metric`, `end`), the exact command line, exit code, wall time and the evidence block (claim Known answer, class
Tested, determinism D0); `metric` lines carry the counts of entries, controls, obligations, `caught`, `stale` and
`pending` against 19; the digests of `counterexamples.toml` and of the scratch `controls.toml` before and after the
second seeding; per planted defect an `assert` with the expected and observed failing check; on failure the first
missing or differing entry with the scratch path; a final `summary` line naming GATE-GAUNT-01. Each control is
exercised by the end-to-end script of its gate (for example `scripts/e2e/atelier_check.sh` of N28a-29 for
GATE-ATEL-02 and -05), and the chapter battery (N28c-24) lists, per control, the script and the `assert` line that
shows it was caught.

### Known answers and negative controls

This bead is the registry of negative-control seeds. Its own negative control: a catalogue entry with a number
changed by one digit (0.94176 → 0.94167) must fail the arithmetic test. Known answers behind the analysis rows:
KA-680 to KA-688 (N28-KA). Determinism: every check is exact or rational arithmetic, class D0, identical twice on
one machine and on both platforms.

## Considerations

- CX-312 has no numeric case in the plan; its control is a planted input (a material field sampled by draw index
  on two paths of different length), and the pairing bead N28a-18 must supply a small numeric fixture when it
  implements the rule. Until then the entry is an obligation: it has an identifier and gates but makes no gate
  eligible.
- CX-318 is a statement about published evidence, re-read before any gate cites it; its control tests the
  fail-closed default, not the literature.
- The exhaustive-enumeration facts (11,520; 1,596; 34,590) come from executed planning pilots and are re-run in
  the repository before a gate cites them (`rerun = false` until then).
- The planting beads name their planted variants by the `plant` identifiers of this table, so the seeding tool can
  join a variant to its entry without a second registry.

## References

Appendix N (N.25), §28.4, §28.6, §28.7, §28.10, §28.13, §28.16, §28.25, §28.26, §38.11; GATE-GAUNT-01;
KA-680 to KA-688.
